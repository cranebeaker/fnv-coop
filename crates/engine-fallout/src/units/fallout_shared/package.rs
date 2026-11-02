//! `fallout shared/package.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds `TESPackage` (91 functions to translate). The first
//! session translated the first 40, `006707c0` to `00674d70`: the
//! constructor, `CreatePackage`, `SetPackType`, the flag accessors of
//! `PACKAGE_DATA`, the location, second-location and target setters, the
//! dialogue-data accessors, `InitItem` and `SetIsCreated`. The second
//! session translated the next 40, `00674dd0` to `00678ec0`: the script and
//! never-run flags, the getters of the own and second location (world,
//! cell, coordinates, reference, radius), the search location, the
//! per-type data getters, `ComputeInitialTarget`, the two "is the subject at
//! the location" tests (`00676390`, `006768d0`), `IsActorAtRefTarget`,
//! `IsTargetAtLocation`, `IsTargetAtSecondLocation`,
//! `CalculateProcedureType`, the follow-target search (`006780e0` and its
//! callback), the two radius functions and the schedule day test. The
//! third session translated the last 13, `00678fd0` to `0067acf0`: the save
//! size, the old and new save/load/init-load functions, the form-flag
//! hooks, `GetObjectTypeFromForm`, `FormMatchesPackageObjectType` and the
//! package-data constructor. The unit is complete.
//!
//! ## Layout (PC build)
//!
//! `TESPackage` is 0x80 bytes on PC (the constructor's allocation); the
//! Xbox PDB says 0x90 and its fields after `TESForm` sit 0x10 lower here,
//! because the PC `TESForm` has no editor ID or version-control fields. The
//! `PACKAGE_DATA` member of the PDB (`iPackFlags` at PDB +0x2c) is
//! flattened into the `TESPackage` layout at its PC offsets (+0x1c).
//!
//! The parts of other classes are read at their offsets with the PDB name
//! in a comment, as their own units are not ours: `PackageLocation` (0xC),
//! `PackageTarget` (0x10), the per-procedure data object at `+0x28`
//! (`TESPackageData`, a vtable pointer and the fields of the subclass for
//! the package's procedure type).
//!
//! ## Helpers the translations call by address
//!
//! Nearly every callee is a one-line accessor of the original source (the
//! build did not optimise): `0041ca90` returns the package type
//! (`cPackType`, a sign-extended byte at +0x20), `0055b980` the location
//! (`pPackLoc`, +0x2c), `0087eaa0` the address of the conditions (+0x40),
//! `005f36f0` the idle collection (+0x34), `006733e0` and `00673400` get and
//! set the combat style (+0x48), `0084e3a0` the form ID (+0x0c), `004013e0`
//! tests form flag 0x8, `00484ab0` sets or clears it. The names the engine
//! map gives to some of these belong to other methods that the linker
//! folded onto the same body.
//!
//! `00401000` is `operator new`; objects are built the way the compiler
//! wrote it: allocate, and call the constructor only when the allocation
//! returned something. The scalar deleting destructors are called with
//! the flag 1.
//!
//! Not translated: the C++ exception-unwinding frames (`FS:[0]` chains) of
//! every function that has one.

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new(size)`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The package type: a sign-extended byte at +0x20 (`PACKAGE_DATA::cPackType`).
const GET_PACK_TYPE: u32 = 0x0041_ca90;
/// `pPackLoc` getter (+0x2c).
const GET_LOCATION: u32 = 0x0055_b980;
/// Address of `packConditions` (+0x40).
const GET_CONDITIONS: u32 = 0x0087_eaa0;
/// `pIdleCollection` getter (+0x34).
const GET_IDLE_COLLECTION: u32 = 0x005f_36f0;
/// `pCombatStyle` getter (+0x48).
const GET_COMBAT_STYLE: u32 = 0x0067_33e0;
/// `pCombatStyle` setter (+0x48).
const SET_COMBAT_STYLE: u32 = 0x0067_3400;
/// `TESForm::iFormID` getter (+0x0c).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// Tests form flag 0x8 (`iFormFlags`, +0x08).
const GET_FORM_FLAG_8: u32 = 0x0040_13e0;
/// Sets (argument 1) or clears form flag 0x8.
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `TESForm::TESForm`.
const TES_FORM_CONSTRUCTOR: u32 = 0x0048_3370;
/// Sets `cFormType` (+0x04); the constructor passes 0x49.
const SET_FORM_TYPE: u32 = 0x004f_15a0;
/// `TESForm::GetFile(this, -1)`.
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex(&formId, file)` (cdecl).
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// Looks a form up by ID (cdecl, one argument): the pointer that goes
/// to `__RTDynamicCast`.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast(object, 0, from, to, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `InitItem` passes to `__RTDynamicCast`: from
/// `TESForm` to the class of the combat style.
const TYPE_DESCRIPTOR_TES_FORM: u32 = 0x0118_3028;
const TYPE_DESCRIPTOR_COMBAT_STYLE: u32 = 0x0118_627c;
/// Printf-style logger of the data loader (cdecl, variable arguments).
const LOG_FORM_WARNING: u32 = 0x005b_5e40;
/// `fr2Assert(file, line)`.
const ASSERT: u32 = 0x00aa_fb90;
/// The source file path string `fr2Assert` receives.
const PACKAGE_SOURCE_FILE: u32 = 0x0106_85dc;
/// `"FORMS: Could not find combat style (%08X) on package '%s' (%08X)."`.
const MESSAGE_COMBAT_STYLE_MISSING: u32 = 0x0106_8688;
/// `"MASTERFILE: Warnings were generated for package '%s' (%08). ..."`.
const MESSAGE_WARNINGS_GENERATED: u32 = 0x0106_8620;
/// The loader's warning counter (a thread-local field, read by a function).
const GET_WARNING_COUNT: u32 = 0x0046_e8a0;
/// The data handler singleton and its "is this form ID dynamic" test
/// (`formID >= 0xFF000000`).
const DATA_HANDLER: u32 = 0x011c_3f2c;
const IS_DYNAMIC_FORM_ID: u32 = 0x0046_9860;
/// `0.0` as a `double`: the compare constant of `GetAcquireRadius`.
const ZERO: u32 = 0x0101_2060;

/// `TESPackage` vtable.
const TES_PACKAGE_VTABLE: u32 = 0x0106_847c;
/// Slot 0 of every package-data object: the scalar deleting destructor.
const SLOT_DELETE: u32 = 0x00;
/// Slot of `InitItem` in the package-data vtable (`TESPackageData`).
const SLOT_DATA_INIT_ITEM: u32 = 0x10;
/// `TESForm` slot 0x130: returns the name the loader prints for a form
/// (it takes no argument; the code still pushes the form ID right before
/// the call, which stays on the stack as the first variable argument of
/// the logger call that follows).
const SLOT_FORM_NAME: u32 = 0x130;

/// `PackageSchedule`, `TESCondition` and the three `PackageEventAction`
/// members: constructors, the setter of the event kind and `InitItem`.
const PACKAGE_SCHEDULE_CONSTRUCTOR: u32 = 0x0067_fcb0;
const TES_CONDITION_CONSTRUCTOR: u32 = 0x0068_0890;
const PACKAGE_EVENT_ACTION_CONSTRUCTOR: u32 = 0x0067_da40;
const PACKAGE_EVENT_ACTION_SET_KIND: u32 = 0x0041_fd00;
const PACKAGE_EVENT_ACTION_INIT_ITEM: u32 = 0x0067_e190;
const TES_CONDITION_INIT_ITEM: u32 = 0x0068_0a20;
const BGS_IDLE_COLLECTION_INIT_ITEM: u32 = 0x0047_9b00;

/// `PackageLocation` (0xC bytes): constructor, scalar deleting destructor,
/// `Copy(source, 0)`, `InitItem(package)`, `SetLocType`, `GetLocType`.
const LOCATION_SIZE: u32 = 0xc;
const LOCATION_CONSTRUCTOR: u32 = 0x0067_f030;
const LOCATION_DELETE: u32 = 0x0067_0b30;
const LOCATION_COPY: u32 = 0x0067_f4d0;
const LOCATION_INIT_ITEM: u32 = 0x0067_f760;
const LOCATION_SET_TYPE: u32 = 0x0067_f140;
const LOCATION_GET_TYPE: u32 = 0x0067_8ca0;
/// The location type `InitItem` sets for the types that decide their own
/// location at run time (a number; the exe does not name it here).
const LOCATION_TYPE_SIX: u32 = 6;
/// The `GetLocType` value for which `GetPackageSearchLocation` answers the
/// package's own location.
const LOCATION_TYPE_SEVEN: u32 = 7;

/// `PackageTarget` (0x10 bytes): constructor, scalar deleting destructor,
/// `Copy(source)`, `InitItem(package)`, `GetTargType`, `SetTargType`,
/// `iValue` getter (+0x08).
const TARGET_SIZE: u32 = 0x10;
const TARGET_CONSTRUCTOR: u32 = 0x0067_ff70;
const TARGET_DELETE: u32 = 0x007b_3fa0;
const TARGET_COPY: u32 = 0x0068_01a0;
const TARGET_INIT_ITEM: u32 = 0x0068_03b0;
const TARGET_GET_TYPE: u32 = 0x0051_9b00;
const TARGET_SET_TYPE: u32 = 0x0068_00b0;
const TARGET_GET_VALUE: u32 = 0x0044_ddc0;

/// `PACKAGE_PROCEDURE_TYPE` getter (+0x18).
const GET_PROCEDURE_TYPE: u32 = 0x0096_11e0;
/// A `float` getter at +0x0c of the target (`PackageTarget::fAcquireRadius`
/// is at +0x0c in the Xbox PDB), returned in `ST0`.
const GET_TARGET_ACQUIRE_RADIUS: u32 = 0x0084_d030;
/// Other functions of this unit, called by address because a later
/// session translates them.
const FN_00673180: u32 = 0x0067_3180;
const FN_00673890: u32 = 0x0067_3890;

/// Package types (`cPackType` values) the code compares with. The exe
/// does not name them in the code translated here, so they are numbers
/// except the two that select the dialogue data and the one that selects
/// the use-weapon data.
const TYPE_DIALOGUE: i32 = 0xf;
/// The second type that shares the dialogue-data object.
const TYPE_DIALOGUE_SECOND: i32 = 0x1c;
const TYPE_USE_WEAPON: i32 = 0x10;
/// "No type yet" (the reset stores 0xff).
const TYPE_NONE: u8 = 0xff;

/// Size and constructor of the per-procedure data objects.
const DIALOGUE_DATA_SIZE: u32 = 0x20;
const DIALOGUE_DATA_CONSTRUCTOR: u32 = 0x0067_b170;
const USE_WEAPON_DATA_SIZE: u32 = 0x24;
const USE_WEAPON_DATA_CONSTRUCTOR: u32 = 0x0067_caa0;

/// `TESDialoguePackageData` (Xbox PDB), 0x20 bytes: the offsets read here.
const DIALOGUE_FOV: u32 = 0x04;
const DIALOGUE_TOPIC: u32 = 0x08;
const DIALOGUE_TARGET_LOCATION: u32 = 0x0c;
const DIALOGUE_NO_HEADTRACKING: u32 = 0x10;
const DIALOGUE_DO_NOT_CONTROL_TARGET: u32 = 0x11;
const DIALOGUE_SAY_TO: u32 = 0x18;
/// `TESUseWeaponPackageData::pTargetLocation` (Xbox PDB).
const USE_WEAPON_TARGET_LOCATION: u32 = 0x08;
/// The other procedure data objects keep their second location at +4.
const SECOND_LOCATION_AT_4: u32 = 0x04;

/// `PACKAGE_DATA::iPackFlags` bits (roles unconfirmed except the two named).
const FLAG_BIT_40: u32 = 0x40;
const FLAG_BIT_10: u32 = 0x10;
const FLAG_BIT_80: u32 = 0x80;
const FLAG_BIT_20: u32 = 0x20;
const FLAG_BIT_100: u32 = 0x100;
const FLAG_ONCE_PER_DAY: u32 = 0x400;
const FLAG_CREATED: u32 = 0x800;
const FLAG_BIT_800000: u32 = 0x80_0000;
/// Set by `InitItem` when loading the package produced warnings (the
/// message says the package is disabled in game).
const FLAG_DISABLED: u32 = 0x8000;

// Second batch (`00674dd0` to `00678ec0`): flags, callees, vtable slots and
// data-section addresses. Slots and callees are named by what the code
// does with them; where the role is not confirmed the name says so.

/// `iPackFlags` bit 0x4000 (`IsScriptPackage`) and bit 0x10000 (set by
/// `fn_00674f30`; role unconfirmed).
const FLAG_SCRIPT_PACKAGE: u32 = 0x4000;
const FLAG_BIT_10000: u32 = 0x1_0000;

/// `TESForm` slots 0x48 and 0x4c, called with a mask after a package flag
/// changed (looks like the "changed flags" marker; unconfirmed).
const SLOT_FORM_SET_CHANGED: u32 = 0x48;
const SLOT_FORM_CLEAR_CHANGED: u32 = 0x4c;

/// `PackageTarget::GetTargReference` (`00680020`), `GetTargObject`
/// (`00680050`), `GetTargObjectType` (`00680080`).
const TARGET_GET_REFERENCE: u32 = 0x0068_0020;
const TARGET_GET_OBJECT: u32 = 0x0068_0050;
const TARGET_GET_OBJECT_TYPE: u32 = 0x0068_0080;
/// `PackageLocation::GetLocReference` (`0067f390`), `GetLocCell`
/// (`0067f3e0`), `GetLocObject` (`0067f430`), `GetLocObjectType`
/// (`0067f480`).
const LOCATION_GET_REFERENCE: u32 = 0x0067_f390;
const LOCATION_GET_CELL: u32 = 0x0067_f3e0;
const LOCATION_GET_OBJECT: u32 = 0x0067_f430;
const LOCATION_GET_OBJECT_TYPE: u32 = 0x0067_f480;
/// The location's own radius (`0067f1c0`: the word at `+0x04`, 0 when the
/// location type is 0xff or 1).
const LOCATION_GET_RADIUS: u32 = 0x0067_f1c0;
/// Resolves the reference a location stands for, for an actor (`0067f2a0`).
const LOCATION_RESOLVE_REFERENCE: u32 = 0x0067_f2a0;
/// The location's own "is the subject there" test
/// (`0067f220(location, actor, subject, &answer)`): true when it decided,
/// the answer going to the byte.
const LOCATION_TEST_SUBJECT: u32 = 0x0067_f220;
/// Tests that the word at `+0x04` of a location is set (`00454b70`).
const LOCATION_HAS_VALUE: u32 = 0x0045_4b70;

/// `TESObjectREFR::GetWorldSpace` (`00575d70`), `TESObjectCELL::GetWorldSpace`
/// (`0054ddd0`).
const REFERENCE_GET_WORLD_SPACE: u32 = 0x0057_5d70;
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
/// A reference's parent cell (`+0x40`, `008d6f30`).
const GET_PARENT_CELL: u32 = 0x008d_6f30;
/// A reference's base form (`+0x20`, `007af430`) and a form's type
/// (`00401170`: the byte at `+0x04`).
const GET_BASE_FORM: u32 = 0x007a_f430;
const GET_FORM_TYPE: u32 = 0x0040_1170;
/// The actor's process object (`+0x68`, `008d8520`).
const GET_PROCESS: u32 = 0x008d_8520;
/// The reference held by extra data type 0x51 (`00569b80`: the extra data
/// list at `+0x44`, then `0041e410`).
const GET_LINKED_REFERENCE: u32 = 0x0056_9b80;
/// A reference's extra data list (`+0x44`, `005d43c0`).
const GET_EXTRA_DATA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList::GetPackageStartLocation` (`00418b70`),
/// `GetReferencePointer` (`0041c8d0`), `GetPackageExtra` (`0041cb10`).
const EXTRA_GET_START_LOCATION: u32 = 0x0041_8b70;
const EXTRA_GET_REFERENCE_POINTER: u32 = 0x0041_c8d0;
const EXTRA_GET_PACKAGE_EXTRA: u32 = 0x0041_cb10;
/// `TESObjectREFR::GetPackageStartLocationWorld` (`0056ad30`),
/// `...InteriorCell` (`0056ad80`), `...Coord` (`0056add0`) and
/// `MobileObject::SetPackageStartLocationFromCurrentLocation` (`009316c0`).
const START_LOCATION_WORLD: u32 = 0x0056_ad30;
const START_LOCATION_INTERIOR_CELL: u32 = 0x0056_ad80;
const START_LOCATION_COORD: u32 = 0x0056_add0;
const SET_START_LOCATION_FROM_CURRENT: u32 = 0x0093_16c0;
/// `TESObjectCELL::GetFirstRefr` (`0054cee0`), `GetDataX` (`00544c30`),
/// `GetDataY` (`00544c60`) and the test of bit 0 of the cell's flag byte at
/// `+0x24` (`00425fd0`; a cell without it counts as no cell where the
/// tests use it).
const CELL_GET_FIRST_REFERENCE: u32 = 0x0054_cee0;
const CELL_GET_DATA_X: u32 = 0x0054_4c30;
const CELL_GET_DATA_Y: u32 = 0x0054_4c60;
const CELL_FLAG_1: u32 = 0x0042_5fd0;
/// `TESObjectREFR::IsFurniture` (`00568680`: base form set and of form
/// type 0x27), the model bound size as a float (`00571600`), and the form
/// flag 0x20 test (`00440d80`).
const IS_FURNITURE: u32 = 0x0056_8680;
const MODEL_BOUND_SIZE: u32 = 0x0057_1600;
const FORM_FLAG_20: u32 = 0x0044_0d80;
/// Whether a reference has a linked door (`00568e50`) and its teleport
/// position (`00568fa0`).
const HAS_LINKED_DOOR: u32 = 0x0056_8e50;
const LINKED_DOOR_TELEPORT_POSITION: u32 = 0x0056_8fa0;
/// The process field at `+0x28` (`0045cd60`).
const PROCESS_FIELD_28: u32 = 0x0045_cd60;

/// Math and CRT helpers by address: `_ftol2` (`00ec62c0`, a leading `f64`),
/// the minimum of two floats (`0040ebd0`), the absolute value (`00408840`),
/// `NiPoint3` minus (`004578c0`, in place) and difference (`00439ef0`,
/// `this - other` into an out parameter), the flat length (`00589850`),
/// `_finite` and `sprintf`.
const FTOL: u32 = 0x00ec_62c0;
const FLOAT_MIN: u32 = 0x0040_ebd0;
const ABSOLUTE_VALUE: u32 = 0x0040_8840;
const POINT_SUBTRACT: u32 = 0x0045_78c0;
const POINT_DIFFERENCE: u32 = 0x0043_9ef0;
const FLAT_LENGTH: u32 = 0x0058_9850;
const CRT_FINITE: u32 = 0x00ec_7595;
const CRT_SPRINTF: u32 = 0x00ec_623a;
/// The loader's error report (`0040fbe0(text)`) and a function that returns
/// its first stack argument (`00464f30`).
const REPORT_ERROR: u32 = 0x0040_fbe0;
const PASS_THROUGH: u32 = 0x0046_4f30;
/// The bounds of a base form of type 0x15: the high point (`0050eb90`) and
/// the low point (`0050ead0`), each written to an out parameter.
const BOUNDS_HIGH: u32 = 0x0050_eb90;
const BOUNDS_LOW: u32 = 0x0050_ead0;

/// Settings: a setting object keeps its value at `+4`; `0043d4d0` gives
/// the value's address (and `00403e20` the float's, `00403df0` the word).
const SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
const SETTING_VALUE_OR_ZERO: u32 = 0x0040_3df0;
/// The default radius setting (an integer), the flat height tolerance and
/// the follow search radius (floats), the follower limit (an integer) and
/// the message setting.
const DEFAULT_RADIUS_SETTING: u32 = 0x011c_de98;
const HEIGHT_TOLERANCE_SETTING: u32 = 0x011d_09d0;
const FOLLOW_RADIUS_SETTING: u32 = 0x011d_65ac;
const FOLLOWER_LIMIT_SETTING: u32 = 0x011c_dad0;
const MESSAGE_SETTING: u32 = 0x011d_3d54;

/// Data-section constants: three floats of the default point, `-1.0` and
/// `100.0` and `2.0` and `0.5` (doubles), `20.0` (a float), the message
/// delay (a float).
const DEFAULT_POINT: u32 = 0x011f_426c;
const NO_EXTRA_RADIUS: u32 = 0x0101_a6b0;
const DISTANCE_LIMIT: u32 = 0x0101_7a40;
const TWO: u32 = 0x0101_1590;
const HALF: u32 = 0x0101_1588;
const DEFAULT_MARGIN: u32 = 0x0101_7868;
const MESSAGE_DELAY: u32 = 0x0101_62c0;
/// Globals: the player (`011dea3c`), the two marker base forms
/// (`011ca248`, `011ca244`), the reference found by the follow search
/// (`011d6580`) and the game clock object (`011de7b8`).
const PLAYER: u32 = 0x011d_ea3c;
const MARKER_FORM_A: u32 = 0x011c_a248;
const MARKER_FORM_B: u32 = 0x011c_a244;
const FOUND_REFERENCE: u32 = 0x011d_6580;
const GAME_CLOCK: u32 = 0x011d_e7b8;

/// Messages (format strings in the exe): the two location warnings of
/// `GetLocationCoord`, the radius and distance error of
/// `IsActorAtRefTarget` and the invalid package text of
/// `CalculateProcedureType`.
const MESSAGE_NO_REFERENCE_LOCATION: u32 = 0x0106_8718;
const MESSAGE_NO_LINKED_REFERENCE: u32 = 0x0106_86d0;
const MESSAGE_RADIUS_DISTANCE: u32 = 0x0106_8764;
const MESSAGE_INVALID_PACKAGE: u32 = 0x0106_8780;

/// Package types the code compares with (see also `TYPE_USE_WEAPON`): 1
/// follow, 2 escort, 0xd patrol, 0xe an unnamed type.
const TYPE_FOLLOW: i32 = 1;
const TYPE_ESCORT: i32 = 2;
const TYPE_PATROL: i32 = 0xd;
const TYPE_0E: i32 = 0xe;
/// The form type `fn_00676280` looks for in a base form.
const FORM_TYPE_0X15: u32 = 0x15;
/// `+0x9c` of the type 0x1c (dialogue) package object: a reference the
/// second-location getters fall back to for the player.
const DIALOGUE_STORED_REFERENCE: u32 = 0x9c;
/// The escort data object's follow distance.
const ESCORT_FOLLOW_DISTANCE: u32 = 0x08;

/// Slots of other classes the code calls (offsets as the code indexes the
/// vtable; roles are named by use, not confirmed): the reference's
/// position (`0x1f4`, a pointer to three floats), the actor's point
/// (`0x170`, written to a buffer), the actor's world space (`0x294`) and
/// cell (`0x298`), "is an actor" (`0x100`), the actor's kind (`0x214`),
/// the actor's `0x290` test, its target reference (`0x2c8`), the "can reach
/// this point" test (`0x2cc`), reach height (`0x380`), a form's `0xe4` test,
/// the actor's `0x22c` test, the extent writer (`0x1dc`).
const SLOT_POSITION: u32 = 0x1f4;
const SLOT_ACTOR_POINT: u32 = 0x170;
const SLOT_ACTOR_WORLD_SPACE: u32 = 0x294;
const SLOT_ACTOR_CELL: u32 = 0x298;
const SLOT_IS_ACTOR: u32 = 0x100;
const SLOT_KIND: u32 = 0x214;
const SLOT_ACTOR_290: u32 = 0x290;
const SLOT_TARGET_REFERENCE: u32 = 0x2c8;
const SLOT_REACH: u32 = 0x2cc;
const SLOT_REACH_HEIGHT: u32 = 0x380;
const SLOT_FORM_E4: u32 = 0xe4;
const SLOT_REJECT_22C: u32 = 0x22c;
const SLOT_BOUNDS_1DC: u32 = 0x1dc;
/// Slots of the actor's process: the package it runs (`0x27c`), the
/// reference chosen for the first and the second location (`0x514`,
/// `0x51c`), the point it holds (`0x4d4`), `0x4c8`, `0x11c`, `0x52c` and
/// `0x288`.
const SLOT_PROCESS_PACKAGE: u32 = 0x27c;
const SLOT_PROCESS_REFERENCE_514: u32 = 0x514;
const SLOT_PROCESS_REFERENCE_51C: u32 = 0x51c;
const SLOT_PROCESS_POINT: u32 = 0x4d4;
const SLOT_PROCESS_4C8: u32 = 0x4c8;
const SLOT_PROCESS_11C: u32 = 0x11c;
const SLOT_PROCESS_52C: u32 = 0x52c;
const SLOT_PROCESS_288: u32 = 0x288;

/// Calls of other units: `Interface::GetTargetREFR` (`00703180`),
/// `TESDataHandler::EnumReferencesCloseToPoint` (`0046f280`), the
/// callback `fn_006784f0`, `Actor::AddFollower` (`008bc790`), the
/// candidate rejection (`008bc7d0`), `MobileObject::GetCurrentPackage`
/// (`009344a0`), the actor's eye level (`008be940`), the on-screen
/// message (`007052f0`), `0055d3b0` (whether a reference is of a form), the
/// player's follower tests (`00962720`, `00962620`), the schedule getter
/// (`0041d8a0`), its day selector (`00678cc0`) and the game clock's day of
/// the week (`00867ef0`).
const INTERFACE_TARGET_REFERENCE: u32 = 0x0070_3180;
const ENUM_REFERENCES_CLOSE_TO_POINT: u32 = 0x0046_f280;
const CALLBACK_FOLLOW_CANDIDATE: u32 = 0x0067_84f0;
const ADD_FOLLOWER: u32 = 0x008b_c790;
const ACTOR_REJECTS_CANDIDATE: u32 = 0x008b_c7d0;
const GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
const ACTOR_EYE_LEVEL: u32 = 0x008b_e940;
const SHOW_MESSAGE: u32 = 0x0070_52f0;
const REFERENCE_MATCHES_FORM: u32 = 0x0055_d3b0;
const PLAYER_CHECK_00962720: u32 = 0x0096_2720;
const PLAYER_COUNT_00962620: u32 = 0x0096_2620;
const GET_SCHEDULE: u32 = 0x0041_d8a0;
const SCHEDULE_DAY_SELECTOR: u32 = 0x0067_8cc0;
const GAME_DAY_OF_WEEK: u32 = 0x0086_7ef0;

layout! {
    /// `TESPackage` (Xbox PDB), 0x80 bytes on PC (0x90 on the Xbox). The
    /// `PACKAGE_DATA` member is flattened in at its PC offsets.
    pub struct TESPackage: 0x80 {
        /// `ePROCEDURE_TYPE` (Xbox PDB, `PACKAGE_PROCEDURE_TYPE`); -1 until
        /// the package is set up.
        0x18 ePROCEDURE_TYPE: i32,
        /// `PackData.iPackFlags` (Xbox PDB).
        0x1C iPackFlags: u32,
        /// `PackData.cPackType` (Xbox PDB): the package type.
        0x20 cPackType: i8,
        /// `PackData.iFOBehaviorFlags` (Xbox PDB).
        0x22 iFOBehaviorFlags: u16,
        /// `PackData.iPackageSpecificFlags` (Xbox PDB).
        0x24 iPackageSpecificFlags: u16,
        /// `pPackData` (Xbox PDB): the `TESPackageData` subclass object of
        /// the package's type.
        0x28 pPackData: Ptr,
        /// `pPackLoc` (Xbox PDB): `PackageLocation*`.
        0x2C pPackLoc: Ptr,
        /// `pPackTarg` (Xbox PDB): `PackageTarget*`.
        0x30 pPackTarg: Ptr,
        /// `pIdleCollection` (Xbox PDB): `BGSIdleCollection*`.
        0x34 pIdleCollection: Ptr,
        /// `packSched` (Xbox PDB): a `PackageSchedule`, 8 bytes.
        0x38 packSched: u32,
        /// `packConditions` (Xbox PDB): a `TESCondition`.
        0x40 packConditions: u32,
        /// `pCombatStyle` (Xbox PDB): `TESCombatStyle*`.
        0x48 pCombatStyle: Ptr,
        /// `OnBegin` (Xbox PDB): a `PackageEventAction`, 0x10 bytes.
        0x4C OnBegin: u32,
        /// `OnEnd` (Xbox PDB).
        0x5C OnEnd: u32,
        /// `OnChange` (Xbox PDB).
        0x6C OnChange: u32,
        /// `uiRefCount` (Xbox PDB).
        0x7C uiRefCount: u32,
    }
}

/// Save and load: the save/load game object (a global pointer) and its
/// methods. `UseSaveGameBlocks`, the stream position (`00825c00`), the raw
/// save (`008579b0(ptr, size)`) and load (`008579e0(ptr, size)`), the
/// current-form words (`004fd3c0` loading, `004fd3e0` saving) and the
/// version byte (`008df040`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
const SAVE_POSITION: u32 = 0x0082_5c00;
const SAVE_BYTES: u32 = 0x0085_79b0;
const LOAD_BYTES: u32 = 0x0085_79e0;
const CURRENT_LOAD_FORM: u32 = 0x004f_d3c0;
const CURRENT_SAVE_FORM: u32 = 0x004f_d3e0;
const LOAD_VERSION: u32 = 0x008d_f040;
/// `00408d60(object)`: the address of a flag byte; the "log the save sizes"
/// flag lives behind `SAVE_SIZE_LOG_OBJECT`.
const FLAG_BYTE_ADDRESS: u32 = 0x0040_8d60;
const SAVE_SIZE_LOG_OBJECT: u32 = 0x011d_e4e8;
/// `Error(format, ...)` (cdecl) and the save/load logger (the same body as
/// `LOG_FORM_WARNING`, cdecl).
const ERROR_LOG: u32 = 0x0040_fbe0;
const SAVE_LOAD_LOG: u32 = LOG_FORM_WARNING;
/// `TESForm::SaveGameDataOLD(ptr, size)`, `TESForm::LoadGameDataOLD(ptr,
/// size)` (Xbox PDB), `TESForm::LoadGame(flags, extra)` (Xbox PDB) and the
/// matching clear of `fn_00679760` (`004534f0(flags)`).
const FORM_SAVE_DATA_OLD: u32 = 0x0048_4ce0;
const FORM_LOAD_DATA_OLD: u32 = 0x0048_4d00;
const FORM_LOAD_GAME: u32 = 0x0048_4c50;
const FORM_CLEAR_FLAGS: u32 = 0x0045_34f0;
/// Save buffer methods: `00865e50(ptr, size, 0)` writes, `00864980(ptr,
/// size)` reads. The two flag reads `00428110(tmp)` / `0042ce30(tmp)` (RET 4) return
/// an object whose `004280f0(mask)` (RET 4) is the answer.
const BUFFER_SAVE_DATA: u32 = 0x0086_5e50;
const BUFFER_LOAD_DATA: u32 = 0x0086_4980;
const BUFFER_FLAG_OBJECT_A: u32 = 0x0042_8110;
const BUFFER_FLAG_OBJECT_B: u32 = 0x0042_ce30;
const BUFFER_FLAG_TEST: u32 = 0x0042_80f0;
/// `PackageLocation` / `PackageTarget` save/load methods: save size,
/// `SaveGame()` (old), `LoadGame()` (old), the unnamed one `fn_006796c0`
/// calls, `SaveGame(buffer)`, `LoadGame(buffer)`, `InitLoadGame(buffer)`.
const LOCATION_GET_SAVE_SIZE: u32 = 0x0067_f990;
const LOCATION_SAVE_OLD: u32 = 0x0067_fa00;
const LOCATION_LOAD_OLD: u32 = 0x0067_faa0;
const LOCATION_AFTER_LOAD: u32 = 0x0067_fb20;
const LOCATION_SAVE_BUFFER: u32 = 0x0067_fb60;
const LOCATION_LOAD_BUFFER: u32 = 0x0067_fbe0;
const LOCATION_INIT_LOAD_BUFFER: u32 = 0x0067_fc50;
const TARGET_GET_SAVE_SIZE: u32 = 0x0068_0520;
const TARGET_SAVE_OLD: u32 = 0x0068_0590;
const TARGET_LOAD_OLD: u32 = 0x0068_0640;
const TARGET_AFTER_LOAD: u32 = 0x0068_06e0;
const TARGET_SAVE_BUFFER: u32 = 0x0068_0720;
const TARGET_LOAD_BUFFER: u32 = 0x0068_07b0;
const TARGET_INIT_LOAD_BUFFER: u32 = 0x0068_0830;
/// Package-data object slots: `SaveGame(buffer)`, `LoadGame(buffer)`,
/// `InitLoadGame(buffer)`.
const SLOT_DATA_SAVE: u32 = 0x14;
const SLOT_DATA_LOAD: u32 = 0x18;
const SLOT_DATA_INIT_LOAD: u32 = 0x1c;
/// The `BLOK` tag that opens a save game block.
const BLOCK_TAG: u32 = 0x424c_4f4b;
/// Form flags and the bits of `iPackFlags` the package keeps for them
/// (`fn_00679700`, `fn_00679760`, `fn_00679b10`).
const FORM_FLAG_HIGH: u32 = 0x8000_0000;
const FORM_FLAG_NEXT: u32 = 0x4000_0000;
const PACK_FLAG_FOR_HIGH: u32 = 0x8000;
const PACK_FLAG_FOR_NEXT: u32 = 0x1_0000;
/// The `+0x3c` member of some forms: slot 4 of its vtable answers a flag
/// (`GetObjectTypeFromForm` and `FormMatchesPackageObjectType`).
const FORM_MEMBER_3C: u32 = 0x3c;
const SLOT_MEMBER_FLAG: u32 = 0x04;
/// Tests on a form of type 0x28: `006450c0` and `004c0c30`; and on the
/// `+0x24` member of a form of type 0x14 (the effect item list):
/// `00405f30`, `00405fe0` and `00406090`.
const FORM_TEST_SIXTEEN: u32 = 0x0064_50c0;
const FORM_TEST_SEVENTEEN: u32 = 0x004c_0c30;
const EFFECT_LIST_TEST_TARGET: u32 = 0x0040_5f30;
const EFFECT_LIST_TEST_TOUCH: u32 = 0x0040_5fe0;
const EFFECT_LIST_TEST_THIRD: u32 = 0x0040_6090;
const FORM_MEMBER_24: u32 = 0x24;
/// Base constructor called by `fn_0067acf0`, and the vtable it stores.
const DATA_BASE_CONSTRUCTOR: u32 = 0x0067_ad20;
const DATA_BASE_VTABLE: u32 = 0x0106_8860;

/// A `PackageLocation*`, `PackageTarget*` or package-data pointer: the
/// classes belong to other units, so the pointer is untyped here.
type Object = Ptr;

/// The package type, as the sign-extended byte the getter returns.
fn pack_type(e: &mut Engine, this: Ptr<TESPackage>) -> i32 {
    e.call(GET_PACK_TYPE, &args![this]).i32()
}

/// The package's `pPackLoc` through its getter.
fn location_of(e: &mut Engine, this: Ptr<TESPackage>) -> Object {
    e.call(GET_LOCATION, &args![this]).ptr()
}

/// Allocates `size` bytes and runs `constructor` on them, as the compiler
/// wrote `new T(...)`: no constructor call when the allocation is null.
/// `extra` are the constructor's stack arguments.
fn construct(e: &mut Engine, size: u32, constructor: u32, extra: &[u32]) -> Object {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    let mut words = vec![block];
    words.extend_from_slice(extra);
    e.call(constructor, &words).ptr()
}

fn new_location(e: &mut Engine) -> Object {
    construct(e, LOCATION_SIZE, LOCATION_CONSTRUCTOR, &[])
}

fn new_target(e: &mut Engine) -> Object {
    construct(e, TARGET_SIZE, TARGET_CONSTRUCTOR, &[])
}

/// Gives the package a location object if it has none (the block the type
/// setup repeats).
fn ensure_location(e: &mut Engine, this: Ptr<TESPackage>) {
    if location_of(e, this).is_null() {
        let made = new_location(e);
        e.set(this, TESPackage::pPackLoc, made);
    }
}

/// The same for the target.
fn ensure_target(e: &mut Engine, this: Ptr<TESPackage>) {
    if fn_00671d10(e, this).is_null() {
        let made = new_target(e);
        e.set(this, TESPackage::pPackTarg, made);
    }
}

/// Builds a per-procedure data object and stores it at +0x28.
fn build_data(e: &mut Engine, this: Ptr<TESPackage>, size: u32, constructor: u32) {
    let made = construct(e, size, constructor, &[]);
    e.set(this, TESPackage::pPackData, made);
}

/// Copies `source` into the location held in the slot at `slot` (a field
/// of the package or of its data object), creating the object when the slot
/// is empty; a null `source` deletes the object and clears the slot.
fn replace_location(e: &mut Engine, slot: u32, source: Object) {
    if source.is_null() {
        let old = e.mem.u32(slot);
        if old != 0 {
            e.call(LOCATION_DELETE, &args![old, 1u32]);
        }
        e.mem.set_u32(slot, 0);
    } else {
        if e.mem.u32(slot) == 0 {
            let made = new_location(e);
            e.mem.set_u32(slot, made.addr());
        }
        let held = e.mem.u32(slot);
        e.call(LOCATION_COPY, &args![held, source, 0u32]);
    }
}

/// Whether the package type is one of the two that carry the dialogue
/// data object. The second test runs only when the first failed, as in
/// the code.
fn is_dialogue_type(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    pack_type(e, this) == TYPE_DIALOGUE || pack_type(e, this) == TYPE_DIALOGUE_SECOND
}

/// The dialogue data object (null when the package has none yet), or
/// `None` when the type has no dialogue data.
fn dialogue_data(e: &mut Engine, this: Ptr<TESPackage>) -> Option<u32> {
    if !is_dialogue_type(e, this) {
        return None;
    }
    Some(e.get(this, TESPackage::pPackData).addr())
}

/// The dialogue data object of a dialogue-type package, built when missing;
/// `None` for another type (the setters then do nothing).
fn dialogue_data_or_create(e: &mut Engine, this: Ptr<TESPackage>) -> Option<u32> {
    if !is_dialogue_type(e, this) {
        return None;
    }
    if e.get(this, TESPackage::pPackData).is_null() {
        build_data(e, this, DIALOGUE_DATA_SIZE, DIALOGUE_DATA_CONSTRUCTOR);
    }
    Some(e.get(this, TESPackage::pPackData).addr())
}

// Translated from 006707c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::TESPackage` (Xbox PDB): constructs the form base, installs
/// the vtable, constructs the schedule, conditions and the three event
/// actions, sets the form type 0x49 and resets the package data.
pub fn tes_package_tes_package(e: &mut Engine, this: Ptr<TESPackage>) -> Ptr<TESPackage> {
    e.call(TES_FORM_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), TES_PACKAGE_VTABLE);
    e.call(
        PACKAGE_SCHEDULE_CONSTRUCTOR,
        &args![this.addr() + TESPackage::packSched.off],
    );
    e.call(
        TES_CONDITION_CONSTRUCTOR,
        &args![this.addr() + TESPackage::packConditions.off],
    );
    for action in [TESPackage::OnBegin, TESPackage::OnEnd, TESPackage::OnChange] {
        e.call(
            PACKAGE_EVENT_ACTION_CONSTRUCTOR,
            &args![this.addr() + action.off],
        );
    }
    e.call(SET_FORM_TYPE, &args![this, 0x49u32]);
    fn_006709d0(e, this);
    e.set(this, TESPackage::uiRefCount, 0);
    this
}

// Translated from 006709d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the package data: flags, behaviour flags, the object pointers,
/// the package type (0xff) and the procedure type (-1), and clears the
/// "created" flag.
pub fn fn_006709d0(e: &mut Engine, this: Ptr<TESPackage>) {
    e.set(this, TESPackage::iPackFlags, 0);
    e.set(this, TESPackage::iPackageSpecificFlags, 0);
    e.set(this, TESPackage::iFOBehaviorFlags, 0);
    e.set(this, TESPackage::pPackLoc, Ptr::NULL);
    e.set(this, TESPackage::pPackTarg, Ptr::NULL);
    e.set(this, TESPackage::pIdleCollection, Ptr::NULL);
    e.set(this, TESPackage::pCombatStyle, Ptr::NULL);
    e.set(this, TESPackage::pPackData, Ptr::NULL);
    e.call(FN_00673180, &args![this, 0u32]);
    e.set(this, TESPackage::ePROCEDURE_TYPE, -1);
    e.set(this, TESPackage::cPackType, TYPE_NONE as i8);
    tes_package_set_is_created(e, this, 0);
}

// Translated from 00670b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::CreatePackage` (Xbox PDB), a cdecl static: builds the
/// package object for a package type and sets its type. The size and
/// constructor depend on the type (the switch is a jump table over
/// `type + 1`): 0xf and 0x1c, 0x12, 0x15, 0x16, 0x17, 0x18 and 0x27 build
/// one of the subclasses; -1, 9, 0x11 and 0x28 build nothing (0x11 is
/// mapped to 0 first, but there is still no object); every other value
/// builds a plain `TESPackage`. Returns null when nothing was built.
pub fn tes_package_create_package(e: &mut Engine, package_type: i32) -> Ptr<TESPackage> {
    let mut package_type = package_type;
    let package: Object = match package_type {
        -1 | 9 | 0x28 => Ptr::NULL,
        0x11 => {
            package_type = 0;
            Ptr::NULL
        }
        0xf | 0x1c => construct(e, 0xd0, 0x009e_dbc0, &[]),
        0x12 => construct(e, 0x188, 0x0097_d3a0, &[0, 0, 0]),
        0x15 => construct(e, 0x88, 0x009e_c5d0, &[]),
        0x16 => construct(e, 0xac, 0x009f_0e00, &[0, 0, 0]),
        0x17 => construct(e, 0x9c, 0x009f_91b0, &[]),
        0x18 => construct(e, 0xb4, 0x009f_72f0, &[]),
        0x27 => construct(e, 0x8c, 0x009e_d030, &[]),
        _ => construct(e, 0x80, 0x0067_07c0, &[]),
    };
    let package: Ptr<TESPackage> = package.cast();
    if !package.is_null() {
        tes_package_set_pack_type(e, package, package_type);
    }
    package
}

// Translated from 00670ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 0x40 of `iPackFlags`.
pub fn fn_00670ed0(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    e.get(this, TESPackage::iPackFlags) & FLAG_BIT_40 != 0
}

// Translated from 00670ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 0x10 of `iPackFlags`.
pub fn fn_00670ef0(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    e.get(this, TESPackage::iPackFlags) & FLAG_BIT_10 != 0
}

// Translated from 00670f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 0x80 of `iPackFlags`.
pub fn fn_00670f10(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    e.get(this, TESPackage::iPackFlags) & FLAG_BIT_80 != 0
}

// Translated from 00670f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 0x20 of `iPackFlags`.
pub fn fn_00670f40(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    e.get(this, TESPackage::iPackFlags) & FLAG_BIT_20 != 0
}

// Translated from 00670f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 0x100 of `iPackFlags`.
pub fn fn_00670f60(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    e.get(this, TESPackage::iPackFlags) & FLAG_BIT_100 != 0
}

// Translated from 00670f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetOncePerDay` (Xbox PDB): bit 0x400 of `iPackFlags`.
pub fn tes_package_get_once_per_day(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    e.get(this, TESPackage::iPackFlags) & FLAG_ONCE_PER_DAY != 0
}

// Translated from 00670fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::SetPackType` (Xbox PDB): changes the package type. A
/// change deletes the data object of the old type, clears the objects the
/// new type does not use, and builds the location, target and data objects
/// the new type needs (two jump tables over the type). Type 0x11 is mapped
/// to 0. For type 0xC the specific-flags bits 0x01 to 0x20 are set to the
/// "off" state and bit 0x40 cleared; for type 9 a missing second location
/// is created.
pub fn tes_package_set_pack_type(e: &mut Engine, this: Ptr<TESPackage>, new_type: i32) {
    let mut new_type = new_type;
    if new_type == 0x11 {
        new_type = 0;
    }
    if new_type == pack_type(e, this) {
        return;
    }
    // Delete the old data object (scalar deleting destructor, flag 1).
    let old_data = e.get(this, TESPackage::pPackData);
    if !old_data.is_null() {
        e.vcall(old_data.addr(), SLOT_DELETE, &args![1u32]);
    }
    e.set(this, TESPackage::pPackData, Ptr::NULL);

    // First table: clear what the new type does not keep.
    match new_type {
        0 | 1 | 2 | 8 | 9 | 10 | 11 | 14 | 16 => {
            tes_package_set_package_second_location(e, this, Ptr::NULL)
        }
        4..=6 | 13 => tes_package_set_package_target(e, this, Ptr::NULL),
        7 => tes_package_set_package_location(e, this, Ptr::NULL),
        12 => {
            tes_package_set_package_target(e, this, Ptr::NULL);
            tes_package_set_package_second_location(e, this, Ptr::NULL);
        }
        _ => {}
    }

    // Second table: build what the new type needs.
    match new_type {
        0 | 7 | 14 => ensure_target(e, this),
        1 => {
            build_data(e, this, 0xc, 0x0067_bfd0);
            ensure_target(e, this);
        }
        2 => {
            ensure_target(e, this);
            ensure_location(e, this);
            build_data(e, this, 0xc, 0x0067_bbf0);
        }
        3 => {
            ensure_target(e, this);
            let target = fn_00671d10(e, this);
            if e.call(TARGET_GET_TYPE, &args![target]).i32() == 0 {
                e.call(TARGET_SET_TYPE, &args![target, 2u32]);
            }
            build_data(e, this, 8, 0x0067_b950);
            ensure_location(e, this);
        }
        4..=6 | 12 => ensure_location(e, this),
        8 => {
            build_data(e, this, 8, 0x0067_c770);
            ensure_location(e, this);
            ensure_target(e, this);
        }
        9 => {
            ensure_location(e, this);
            build_data(e, this, 8, 0x0067_acf0);
        }
        13 => build_data(e, this, 8, 0x0067_c3f0),
        0xf | 0x1c => {
            ensure_target(e, this);
            build_data(e, this, DIALOGUE_DATA_SIZE, DIALOGUE_DATA_CONSTRUCTOR);
        }
        0x10 => {
            fn_00671a20(e, this, 1);
            ensure_target(e, this);
            build_data(e, this, USE_WEAPON_DATA_SIZE, USE_WEAPON_DATA_CONSTRUCTOR);
        }
        _ => {}
    }

    e.set(this, TESPackage::iPackageSpecificFlags, 0);
    if new_type == 0xc {
        fn_00671ad0(e, this, 1);
        fn_00671b30(e, this, 1);
        fn_00671b90(e, this, 1);
        fn_00671bf0(e, this, 1);
        fn_00671c50(e, this, 1);
        fn_00671cb0(e, this, 1);
        fn_00671a70(e, this, 0);
    }
    e.set(this, TESPackage::cPackType, new_type as i8);
    if new_type == 9 && fn_00672dd0(e, this).is_null() {
        let location = new_location(e);
        tes_package_set_package_second_location(e, this, location);
        if !location.is_null() {
            e.call(LOCATION_DELETE, &args![location, 1u32]);
        }
    }
}

// Translated from 00671a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag != 0`) or clears bit 0x800000 of `iPackFlags`.
pub fn fn_00671a20(e: &mut Engine, this: Ptr<TESPackage>, flag: u8) {
    let flags = e.get(this, TESPackage::iPackFlags);
    let flags = if flag == 0 {
        flags & !FLAG_BIT_800000
    } else {
        flags | FLAG_BIT_800000
    };
    e.set(this, TESPackage::iPackFlags, flags);
}

/// Sets or clears `bit` of `iPackageSpecificFlags`; `set` says which.
fn specific_flag(e: &mut Engine, this: Ptr<TESPackage>, bit: u16, set: bool) {
    let flags = e.get(this, TESPackage::iPackageSpecificFlags);
    let flags = if set { flags | bit } else { flags & !bit };
    e.set(this, TESPackage::iPackageSpecificFlags, flags);
}

// Translated from 00671a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag != 0`) or clears bit 0x40 of `iPackageSpecificFlags`.
pub fn fn_00671a70(e: &mut Engine, this: Ptr<TESPackage>, flag: u8) {
    specific_flag(e, this, 0x40, flag != 0);
}

// Translated from 00671ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears (`flag != 0`) or sets (`flag == 0`) bit 0x01 of
/// `iPackageSpecificFlags`: the flag is stored inverted.
pub fn fn_00671ad0(e: &mut Engine, this: Ptr<TESPackage>, flag: u8) {
    specific_flag(e, this, 0x01, flag == 0);
}

// Translated from 00671b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Same for bit 0x02 (stored inverted).
pub fn fn_00671b30(e: &mut Engine, this: Ptr<TESPackage>, flag: u8) {
    specific_flag(e, this, 0x02, flag == 0);
}

// Translated from 00671b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Same for bit 0x04 (stored inverted).
pub fn fn_00671b90(e: &mut Engine, this: Ptr<TESPackage>, flag: u8) {
    specific_flag(e, this, 0x04, flag == 0);
}

// Translated from 00671bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Same for bit 0x08 (stored inverted).
pub fn fn_00671bf0(e: &mut Engine, this: Ptr<TESPackage>, flag: u8) {
    specific_flag(e, this, 0x08, flag == 0);
}

// Translated from 00671c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Same for bit 0x10 (stored inverted).
pub fn fn_00671c50(e: &mut Engine, this: Ptr<TESPackage>, flag: u8) {
    specific_flag(e, this, 0x10, flag == 0);
}

// Translated from 00671cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Same for bit 0x20 (stored inverted).
pub fn fn_00671cb0(e: &mut Engine, this: Ptr<TESPackage>, flag: u8) {
    specific_flag(e, this, 0x20, flag == 0);
}

// Translated from 00671d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The package's `pPackTarg` (`PackageTarget*`, +0x30).
pub fn fn_00671d10(e: &mut Engine, this: Ptr<TESPackage>) -> Object {
    e.get(this, TESPackage::pPackTarg)
}

// Translated from 00671d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::SetPackageLocation` (Xbox PDB): copies `location` into the
/// package's own `PackageLocation` (creating it when missing); a null
/// `location` deletes it.
pub fn tes_package_set_package_location(e: &mut Engine, this: Ptr<TESPackage>, location: Object) {
    replace_location(e, this.addr() + TESPackage::pPackLoc.off, location);
}

// Translated from 00671e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::SetPackageSecondLocation` (Xbox PDB): the second location
/// lives in the data object of the package's type, whose layout depends on
/// the type: types 0xF and 0x1C keep it at +0xC of the 0x20-byte dialogue
/// data, type 0x10 at +8 of the use-weapon data (built on demand), types
/// 3, 2, 9, 1 and 8 at +4 of their small data objects (built here when
/// missing, with their sizes and constructors). Other types do nothing.
/// `location` is copied into it; null deletes it.
pub fn tes_package_set_package_second_location(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    location: Object,
) {
    if is_dialogue_type(e, this) {
        if e.get(this, TESPackage::pPackData).is_null() {
            build_data(e, this, DIALOGUE_DATA_SIZE, DIALOGUE_DATA_CONSTRUCTOR);
        }
        let data = e.get(this, TESPackage::pPackData).addr();
        replace_location(e, data + DIALOGUE_TARGET_LOCATION, location);
        return;
    }
    // (type, data size, data constructor), each keeping the location at +4.
    const SMALL: [(i32, u32, u32); 5] = [
        (3, 8, 0x0067_b950),
        (2, 0xc, 0x0067_bbf0),
        (9, 8, 0x0067_acf0),
        (1, 0xc, 0x0067_bfd0),
        (8, 8, 0x0067_c770),
    ];
    for (kind, size, constructor) in SMALL {
        if pack_type(e, this) == kind {
            if e.get(this, TESPackage::pPackData).is_null() {
                build_data(e, this, size, constructor);
            }
            let data = e.get(this, TESPackage::pPackData).addr();
            replace_location(e, data + SECOND_LOCATION_AT_4, location);
            return;
        }
    }
    if pack_type(e, this) == TYPE_USE_WEAPON {
        let data = tes_package_get_or_create_use_weapon_package_data(e, this);
        replace_location(e, data.addr() + USE_WEAPON_TARGET_LOCATION, location);
    }
}

// Translated from 00672710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetDialogueSayToFlag` (Xbox PDB): `bSayTo` of the dialogue
/// data, or 0.
pub fn tes_package_get_dialogue_say_to_flag(e: &mut Engine, this: Ptr<TESPackage>) -> u8 {
    match dialogue_data(e, this) {
        Some(data) if data != 0 => e.mem.u8(data + DIALOGUE_SAY_TO),
        _ => 0,
    }
}

// Translated from 00672760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetDialogueTopic` (Xbox PDB): `ptopic` of the dialogue
/// data (`TESTopic*`), or null.
pub fn tes_package_get_dialogue_topic(e: &mut Engine, this: Ptr<TESPackage>) -> Object {
    match dialogue_data(e, this) {
        Some(data) if data != 0 => Ptr::new(e.mem.u32(data + DIALOGUE_TOPIC)),
        _ => Ptr::NULL,
    }
}

// Translated from 006727b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetDialogueDoNotControlTarget` (Xbox PDB):
/// `bDoNotControlTarget`, or 0.
pub fn tes_package_get_dialogue_do_not_control_target(e: &mut Engine, this: Ptr<TESPackage>) -> u8 {
    match dialogue_data(e, this) {
        Some(data) if data != 0 => e.mem.u8(data + DIALOGUE_DO_NOT_CONTROL_TARGET),
        _ => 0,
    }
}

// Translated from 00672800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetDialogueNoHeadtrack` (Xbox PDB): `bNOHeadtracking`, or 0.
pub fn tes_package_get_dialogue_no_headtrack(e: &mut Engine, this: Ptr<TESPackage>) -> u8 {
    match dialogue_data(e, this) {
        Some(data) if data != 0 => e.mem.u8(data + DIALOGUE_NO_HEADTRACKING),
        _ => 0,
    }
}

// Translated from 00672850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fFov` of the dialogue data (1.0 when the package has none).
pub fn fn_00672850(e: &mut Engine, this: Ptr<TESPackage>) -> f32 {
    match dialogue_data(e, this) {
        Some(data) if data != 0 => e.mem.f32(data + DIALOGUE_FOV),
        _ => 1.0,
    }
}

// Translated from 006728a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetAcquireRadius` (Xbox PDB): writes the radius at `radius`
/// (0 first) and says whether the package has one. Type 0xC takes it from
/// `00676280(argument)`, converted from an unsigned integer; otherwise it
/// is the target's float (`0084d030`) when the package has a target and
/// the value is not below or equal to zero.
pub fn tes_package_get_acquire_radius(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    argument: u32,
    radius: Ptr,
) -> bool {
    e.mem.set_f32(radius.addr(), 0.0);
    if pack_type(e, this) == 0xc {
        let value = fn_00676280(e, this, Ptr::new(argument)) as u32;
        e.mem.set_f32(radius.addr(), value as f64 as f32);
        return true;
    }
    let target = fn_00671d10(e, this);
    if target.is_null() {
        return false;
    }
    let value = e.call(GET_TARGET_ACQUIRE_RADIUS, &args![target]).f32();
    let zero: f64 = e.global(ZERO);
    // `FCOMP` then `TEST AH,0x41; JP`: the radius is written when the
    // value is greater than zero or unordered.
    if (value as f64) <= zero {
        return false;
    }
    e.mem.set_f32(radius.addr(), value);
    true
}

// Translated from 00672930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetInitialTargetCount` (Xbox PDB): 0 without a target; 1
/// when the target's type is 0 or 3; otherwise the target's `iValue` when it
/// is positive; otherwise from the procedure type: 1 for 0x1c, 0x7fffffff
/// for 2, 3 and 0x1a, else 0.
pub fn tes_package_get_initial_target_count(e: &mut Engine, this: Ptr<TESPackage>) -> i32 {
    let target = fn_00671d10(e, this);
    if target.is_null() {
        return 0;
    }
    if e.call(TARGET_GET_TYPE, &args![target]).i32() == 0
        || e.call(TARGET_GET_TYPE, &args![target]).i32() == 3
    {
        return 1;
    }
    let value = e.call(TARGET_GET_VALUE, &args![target]).i32();
    if value > 0 {
        return value;
    }
    if e.call(GET_PROCEDURE_TYPE, &args![this]).i32() == 0x1c {
        return 1;
    }
    if e.call(GET_PROCEDURE_TYPE, &args![this]).i32() == 2
        || e.call(GET_PROCEDURE_TYPE, &args![this]).i32() == 3
        || e.call(GET_PROCEDURE_TYPE, &args![this]).i32() == 0x1a
    {
        return 0x7fff_ffff;
    }
    0
}

// Translated from 006729d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `ptopic` of the dialogue data (built when missing); does nothing
/// for another package type.
pub fn fn_006729d0(e: &mut Engine, this: Ptr<TESPackage>, topic: u32) {
    if let Some(data) = dialogue_data_or_create(e, this) {
        e.mem.set_u32(data + DIALOGUE_TOPIC, topic);
    }
}

// Translated from 00672aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bDoNotControlTarget` of the dialogue data (built when missing).
pub fn fn_00672aa0(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    if let Some(data) = dialogue_data_or_create(e, this) {
        e.mem.set_u8(data + DIALOGUE_DO_NOT_CONTROL_TARGET, value);
    }
}

// Translated from 00672b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bNOHeadtracking` of the dialogue data (built when missing).
pub fn fn_00672b70(e: &mut Engine, this: Ptr<TESPackage>, value: u8) {
    if let Some(data) = dialogue_data_or_create(e, this) {
        e.mem.set_u8(data + DIALOGUE_NO_HEADTRACKING, value);
    }
}

// Translated from 00672c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `fFov` (+4) of the dialogue data (built when missing).
pub fn fn_00672c40(e: &mut Engine, this: Ptr<TESPackage>, value: f32) {
    if let Some(data) = dialogue_data_or_create(e, this) {
        e.mem.set_f32(data + DIALOGUE_FOV, value);
    }
}

// Translated from 00672d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetOrCreateUseWeaponPackageData` (Xbox PDB): asserts the
/// type is 0x10 (the assert does not stop the function), builds the
/// `TESUseWeaponPackageData` (0x24 bytes) when missing, and returns the
/// data object.
pub fn tes_package_get_or_create_use_weapon_package_data(
    e: &mut Engine,
    this: Ptr<TESPackage>,
) -> Object {
    if pack_type(e, this) != TYPE_USE_WEAPON {
        e.call(ASSERT, &args![PACKAGE_SOURCE_FILE, 0x607u32]);
    }
    if e.get(this, TESPackage::pPackData).is_null() {
        build_data(e, this, USE_WEAPON_DATA_SIZE, USE_WEAPON_DATA_CONSTRUCTOR);
    }
    e.get(this, TESPackage::pPackData)
}

// Translated from 00672dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The package's second location (`PackageLocation*`), read from the data
/// object of its type: +0xC for types 0xF and 0x1C, +4 for types 3, 2, 9,
/// 8 and 1, +8 for type 0x10; null when there is no data object or another
/// type.
pub fn fn_00672dd0(e: &mut Engine, this: Ptr<TESPackage>) -> Object {
    let has_data = |e: &Engine| !e.get(this, TESPackage::pPackData).is_null();
    if has_data(e) && is_dialogue_type(e, this) {
        let data = e.get(this, TESPackage::pPackData).addr();
        return Ptr::new(e.mem.u32(data + DIALOGUE_TARGET_LOCATION));
    }
    for kind in [3, 2, 9, 8, 1] {
        if has_data(e) && pack_type(e, this) == kind {
            let data = e.get(this, TESPackage::pPackData).addr();
            return Ptr::new(e.mem.u32(data + SECOND_LOCATION_AT_4));
        }
    }
    if has_data(e) && pack_type(e, this) == TYPE_USE_WEAPON {
        let data = e.get(this, TESPackage::pPackData).addr();
        return Ptr::new(e.mem.u32(data + USE_WEAPON_TARGET_LOCATION));
    }
    Ptr::NULL
}

// Translated from 00672f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetPackageSearchLocation` (Xbox PDB): the package's own
/// location for types 0, 0xC, 0x11 and 0x1C; for types 2, 3, 8 and 0x10 the
/// second location, replaced by the package's own location when the second
/// one exists and has location type 7 (`GetLocType`); null otherwise.
pub fn tes_package_get_package_search_location(e: &mut Engine, this: Ptr<TESPackage>) -> Object {
    match pack_type(e, this) {
        0 | 0xc | 0x11 | 0x1c => location_of(e, this),
        2 | 3 | 8 | 0x10 => {
            let second = fn_00672dd0(e, this);
            if !second.is_null()
                && e.call(LOCATION_GET_TYPE, &args![second]).u32() == LOCATION_TYPE_SEVEN
            {
                location_of(e, this)
            } else {
                fn_00672dd0(e, this)
            }
        }
        _ => Ptr::NULL,
    }
}

// Translated from 00672fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::SetPackageTarget` (Xbox PDB): copies `target` into the
/// package's own `PackageTarget` (creating it when missing); a null
/// `target` deletes it.
pub fn tes_package_set_package_target(e: &mut Engine, this: Ptr<TESPackage>, target: Object) {
    if target.is_null() {
        let old = e.get(this, TESPackage::pPackTarg);
        if !old.is_null() {
            e.call(TARGET_DELETE, &args![old, 1u32]);
        }
        e.set(this, TESPackage::pPackTarg, Ptr::NULL);
    } else {
        ensure_target(e, this);
        let held = e.get(this, TESPackage::pPackTarg);
        e.call(TARGET_COPY, &args![held, target]);
    }
}

// Translated from 00674930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::InitItem` (Xbox PDB): resolves the package's references
/// after loading, once (it returns at once when form flag 0x8 is set).
/// Initialises the location, second location, target, conditions, idle
/// collection and the data object; resolves the combat style form ID
/// through `AddCompileIndex` and a dynamic cast (logging when it cannot be
/// found); initialises the three event actions; marks the package disabled
/// when the loader's warning count changed; then applies the per-type fix
/// ups (types 9, 0xC, 0xD, 0xE) and sets form flag 0x8.
pub fn tes_package_init_item(e: &mut Engine, this: Ptr<TESPackage>) {
    if e.call(GET_FORM_FLAG_8, &args![this]).bool() {
        return;
    }
    let warnings_before = e.call(GET_WARNING_COUNT, &[]).u32();

    if !location_of(e, this).is_null() {
        let location = location_of(e, this);
        e.call(LOCATION_INIT_ITEM, &args![location, this]);
    }
    if !fn_00672dd0(e, this).is_null() {
        let second = fn_00672dd0(e, this);
        e.call(LOCATION_INIT_ITEM, &args![second, this]);
    }
    if !fn_00671d10(e, this).is_null() {
        let target = fn_00671d10(e, this);
        e.call(TARGET_INIT_ITEM, &args![target, this]);
    }
    if e.call(GET_CONDITIONS, &args![this]).u32() != 0 {
        let conditions = e.call(GET_CONDITIONS, &args![this]).u32();
        e.call(TES_CONDITION_INIT_ITEM, &args![conditions, this]);
    }
    if e.call(GET_IDLE_COLLECTION, &args![this]).u32() != 0 {
        let idle = e.call(GET_IDLE_COLLECTION, &args![this]).u32();
        e.call(BGS_IDLE_COLLECTION_INIT_ITEM, &args![idle, this]);
    }
    let data = e.get(this, TESPackage::pPackData);
    if !data.is_null() {
        e.vcall(data.addr(), SLOT_DATA_INIT_ITEM, &args![this]);
    }

    // The combat style is held as a form ID until now.
    let combat_style_id = e.call(GET_COMBAT_STYLE, &args![this]).u32();
    if combat_style_id != 0 {
        let (resolved_id, style) = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), combat_style_id);
            let file = e.call(TES_FORM_GET_FILE, &args![this, -1i32]).u32();
            e.call(TES_FORM_ADD_COMPILE_INDEX, &args![slot, file]);
            let resolved_id = e.mem.u32(slot.addr());
            let form = e.call(LOOKUP_FORM, &args![resolved_id]).u32();
            let style = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![
                        form,
                        0u32,
                        TYPE_DESCRIPTOR_TES_FORM,
                        TYPE_DESCRIPTOR_COMBAT_STYLE,
                        0u32
                    ],
                )
                .u32();
            (resolved_id, style)
        });
        e.call(SET_COMBAT_STYLE, &args![this, style]);
        if e.call(GET_COMBAT_STYLE, &args![this]).u32() == 0 {
            let id = e.call(GET_FORM_ID, &args![this]).u32();
            let name = e.vcall(this.addr(), SLOT_FORM_NAME, &[]).u32();
            e.call(
                LOG_FORM_WARNING,
                &args![MESSAGE_COMBAT_STYLE_MISSING, resolved_id, name, id],
            );
        }
    }

    for (action, kind) in [
        (TESPackage::OnBegin, 0u32),
        (TESPackage::OnEnd, 1),
        (TESPackage::OnChange, 2),
    ] {
        e.call(
            PACKAGE_EVENT_ACTION_SET_KIND,
            &args![this.addr() + action.off, kind],
        );
    }
    for action in [TESPackage::OnBegin, TESPackage::OnEnd, TESPackage::OnChange] {
        e.call(
            PACKAGE_EVENT_ACTION_INIT_ITEM,
            &args![this.addr() + action.off, this],
        );
    }

    if e.call(GET_WARNING_COUNT, &[]).u32() != warnings_before {
        let id = e.call(GET_FORM_ID, &args![this]).u32();
        let name = e.vcall(this.addr(), SLOT_FORM_NAME, &[]).u32();
        e.call(
            LOG_FORM_WARNING,
            &args![MESSAGE_WARNINGS_GENERATED, name, id],
        );
        let flags = e.get(this, TESPackage::iPackFlags);
        e.set(this, TESPackage::iPackFlags, flags | FLAG_DISABLED);
    }

    // Per-type fix-ups of the location and target.
    match pack_type(e, this) {
        9 => {
            if fn_00674d20(e, this) {
                let location = location_of(e, this);
                e.call(LOCATION_SET_TYPE, &args![location, LOCATION_TYPE_SIX]);
            }
        }
        0xc => {
            if e.call(FN_00673890, &args![this]).bool() {
                let location = location_of(e, this);
                e.call(LOCATION_SET_TYPE, &args![location, LOCATION_TYPE_SIX]);
            }
        }
        0xd => {
            // Byte at +5 of the data object (`pPackData`).
            let data = e.get(this, TESPackage::pPackData).addr();
            if e.mem.u8(data + 5) != 0 {
                let location = location_of(e, this);
                e.call(LOCATION_SET_TYPE, &args![location, LOCATION_TYPE_SIX]);
            }
            if location_of(e, this).is_null() {
                let made = new_location(e);
                e.call(LOCATION_SET_TYPE, &args![made, LOCATION_TYPE_SIX]);
                tes_package_set_package_location(e, this, made);
                if !made.is_null() {
                    e.call(LOCATION_DELETE, &args![made, 1u32]);
                }
            }
        }
        0xe if fn_00674d20(e, this) => {
            if fn_00671d10(e, this).is_null() {
                let made = new_target(e);
                tes_package_set_package_target(e, this, made);
            }
            let target = fn_00671d10(e, this);
            e.call(TARGET_SET_TYPE, &args![target, 3u32]);
        }
        _ => {}
    }
    e.call(SET_FORM_FLAG_8, &args![this, 1u32]);
}

// Translated from 00674d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 0x02 of `iPackageSpecificFlags`.
pub fn fn_00674d20(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    e.get(this, TESPackage::iPackageSpecificFlags) & 2 != 0
}

// Translated from 00674d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetIsCreated` (Xbox PDB): bit 0x800 of `iPackFlags`.
pub fn tes_package_get_is_created(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    e.get(this, TESPackage::iPackFlags) & FLAG_CREATED != 0
}

// Translated from 00674d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::SetIsCreated` (Xbox PDB): sets or clears bit 0x800 of
/// `iPackFlags`, but only for a package whose form ID is a dynamic one
/// (the data handler's test, `formID >= 0xFF000000`).
pub fn tes_package_set_is_created(e: &mut Engine, this: Ptr<TESPackage>, created: u8) {
    let form_id = e.call(GET_FORM_ID, &args![this]).u32();
    let handler: u32 = e.global(DATA_HANDLER);
    if !e.call(IS_DYNAMIC_FORM_ID, &args![handler, form_id]).bool() {
        return;
    }
    let flags = e.get(this, TESPackage::iPackFlags);
    let flags = if created == 0 {
        flags & !FLAG_CREATED
    } else {
        flags | FLAG_CREATED
    };
    e.set(this, TESPackage::iPackFlags, flags);
}

// Translated from 00674dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::IsScriptPackage` (Xbox PDB): bit 0x4000 of `iPackFlags`.
pub fn tes_package_is_script_package(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    e.get(this, TESPackage::iPackFlags) & FLAG_SCRIPT_PACKAGE != 0
}

// Translated from 00674e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit 0x4000 of `iPackFlags`.
pub fn fn_00674e00(e: &mut Engine, this: Ptr<TESPackage>, flag: u8) {
    let flags = e.get(this, TESPackage::iPackFlags);
    let flags = if flag == 0 {
        flags & !FLAG_SCRIPT_PACKAGE
    } else {
        flags | FLAG_SCRIPT_PACKAGE
    };
    e.set(this, TESPackage::iPackFlags, flags);
}

// Translated from 00674e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::IsNeverToRun` (Xbox PDB): bit 0x8000 of `iPackFlags`.
pub fn tes_package_is_never_to_run(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    e.get(this, TESPackage::iPackFlags) & FLAG_DISABLED != 0
}

/// Whether the package's form ID is a dynamic one (the data handler's test).
fn has_dynamic_form_id(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let form_id = e.call(GET_FORM_ID, &args![this]).u32();
    let handler: u32 = e.global(DATA_HANDLER);
    e.call(IS_DYNAMIC_FORM_ID, &args![handler, form_id]).bool()
}

/// Sets or clears one `iPackFlags` bit, and, when the form ID is not a
/// dynamic one, tells the form through slot 0x48 (set) or 0x4c (clear)
/// with `mask` (the slots look like the form's "changed flags" marker;
/// they are not named here).
fn change_flag_and_mark(e: &mut Engine, this: Ptr<TESPackage>, bit: u32, set: bool, mask: u32) {
    let flags = e.get(this, TESPackage::iPackFlags);
    if set {
        e.set(this, TESPackage::iPackFlags, flags | bit);
    } else {
        e.set(this, TESPackage::iPackFlags, flags & !bit);
    }
    if !has_dynamic_form_id(e, this) {
        let slot = if set {
            SLOT_FORM_SET_CHANGED
        } else {
            SLOT_FORM_CLEAR_CHANGED
        };
        e.vcall(this.addr(), slot, &args![mask]);
    }
}

// Translated from 00674e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::SetNeverRun` (Xbox PDB): when the package's target is the
/// reference `reference`, sets (`flag` non-zero) or clears bit 0x8000 of
/// `iPackFlags` and marks the form with the mask 0x80000000.
pub fn tes_package_set_never_run(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    reference: Object,
    flag: u8,
) {
    if fn_00671d10(e, this).is_null() {
        return;
    }
    let target = fn_00671d10(e, this);
    let held = e.call(TARGET_GET_REFERENCE, &args![target]).ptr::<()>();
    if held != reference {
        return;
    }
    change_flag_and_mark(e, this, FLAG_DISABLED, flag != 0, 0x8000_0000);
}

// Translated from 00674f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit 0x10000 of `iPackFlags` and marks
/// the form with the mask 0x40000000.
pub fn fn_00674f30(e: &mut Engine, this: Ptr<TESPackage>, flag: u8) {
    change_flag_and_mark(e, this, FLAG_BIT_10000, flag != 0, 0x4000_0000);
}

/// The location type of a `PackageLocation`.
fn location_type(e: &mut Engine, location: Object) -> u32 {
    e.call(LOCATION_GET_TYPE, &args![location]).u32()
}

/// `PackageLocation::GetLocReference` (`+0x08` when the type is 0).
fn location_reference(e: &mut Engine, location: Object) -> Object {
    e.call(LOCATION_GET_REFERENCE, &args![location]).ptr()
}

/// `PackageLocation::GetLocCell` (`+0x08` when the type is 1).
fn location_cell(e: &mut Engine, location: Object) -> Object {
    e.call(LOCATION_GET_CELL, &args![location]).ptr()
}

/// `TESObjectREFR::GetWorldSpace` of a reference.
fn world_of_reference(e: &mut Engine, reference: Object) -> Object {
    e.call(REFERENCE_GET_WORLD_SPACE, &args![reference]).ptr()
}

/// The actor's process object (the engine map names this getter
/// `MiddleHighProcess::GetSavedAcquireObject`; the body reads `+0x68`).
fn process_of(e: &mut Engine, actor: Object) -> Object {
    e.call(GET_PROCESS, &args![actor]).ptr()
}

/// The reference held by the extra data of type 0x51, which the loader's
/// messages call the "linked reference" (`00569b80`).
fn linked_reference(e: &mut Engine, reference: Object) -> Object {
    e.call(GET_LINKED_REFERENCE, &args![reference]).ptr()
}

/// A reference's parent cell (`+0x40`, `008d6f30`).
fn parent_cell_of(e: &mut Engine, reference: Object) -> Object {
    e.call(GET_PARENT_CELL, &args![reference]).ptr()
}

/// The reference the actor's process chose for this package, or `None`
/// when the actor has no process or the process's slot 0x27c does not
/// answer this package. `slot` is the process slot that gives the
/// reference: 0x514 for the first location, 0x51c for the second.
fn process_reference(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
    slot: u32,
) -> Option<Object> {
    if actor.is_null() || process_of(e, actor).is_null() {
        return None;
    }
    let process = process_of(e, actor);
    if e.vcall(process.addr(), SLOT_PROCESS_PACKAGE, &args![])
        .u32()
        != this.addr()
    {
        return None;
    }
    let process = process_of(e, actor);
    Some(e.vcall(process.addr(), slot, &args![]).ptr())
}

/// The fallback of the second-location getters for types 4 and 5: the
/// player running a type 0x1c package whose field at `+0x9c` is set.
/// Returns that field.
fn player_dialogue_reference(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
) -> Option<Object> {
    let player: u32 = e.global(PLAYER);
    if actor.addr() != player || pack_type(e, this) != TYPE_DIALOGUE_SECOND {
        return None;
    }
    let held = e.mem.u32(this.addr() + DIALOGUE_STORED_REFERENCE);
    (held != 0).then(|| Ptr::new(held))
}

// Translated from 00674fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetSecondLocationWorld` (Xbox PDB): the world space of the
/// second location for the actor `actor`. By location type: 0 the world
/// space of the reference, 1 of the cell, 3 the actor's own (slot 0x294),
/// 4 and 5 that of the reference the actor's process chose (or the
/// actor), 6 that of the linked reference; no second location, or type 2,
/// gives the actor's start location's world space.
pub fn tes_package_get_second_location_world(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
) -> Object {
    let location = fn_00672dd0(e, this);
    if location.is_null() || location_type(e, location) == 2 {
        if !actor.is_null() {
            return e.call(START_LOCATION_WORLD, &args![actor]).ptr();
        }
        return Ptr::NULL;
    }
    let mut result = Ptr::NULL;
    match location_type(e, location) {
        0 => {
            if !location_reference(e, location).is_null() {
                let reference = location_reference(e, location);
                result = world_of_reference(e, reference);
            }
        }
        1 => {
            if !location_cell(e, location).is_null() {
                let cell = location_cell(e, location);
                result = e.call(CELL_GET_WORLD_SPACE, &args![cell]).ptr();
            }
        }
        3 => {
            if !actor.is_null() {
                result = e
                    .vcall(actor.addr(), SLOT_ACTOR_WORLD_SPACE, &args![])
                    .ptr();
            }
        }
        4 | 5 => {
            if let Some(chosen) = process_reference(e, this, actor, SLOT_PROCESS_REFERENCE_51C) {
                return if chosen.is_null() {
                    world_of_reference(e, actor)
                } else {
                    world_of_reference(e, chosen)
                };
            }
            if let Some(held) = player_dialogue_reference(e, this, actor) {
                result = world_of_reference(e, held);
            }
        }
        6 if !actor.is_null() && !linked_reference(e, actor).is_null() => {
            let linked = linked_reference(e, actor);
            result = world_of_reference(e, linked);
        }
        _ => {}
    }
    result
}

// Translated from 006751a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetSecondLocationCell` (Xbox PDB): as
/// `GetSecondLocationWorld`, for the cell: type 0 the reference's parent
/// cell, 1 the cell itself, 3 the actor's (slot 0x298), 4 and 5 that of
/// the chosen reference or the actor (process slot 0x514), 6 that of the
/// linked reference; none or type 2: the start location's interior cell.
pub fn tes_package_get_second_location_cell(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
) -> Object {
    let location = fn_00672dd0(e, this);
    if location.is_null() || location_type(e, location) == 2 {
        if !actor.is_null() {
            return e.call(START_LOCATION_INTERIOR_CELL, &args![actor]).ptr();
        }
        return Ptr::NULL;
    }
    let mut result = Ptr::NULL;
    match location_type(e, location) {
        0 => {
            if !location_reference(e, location).is_null() {
                let reference = location_reference(e, location);
                result = parent_cell_of(e, reference);
            }
        }
        1 => result = location_cell(e, location),
        3 => {
            if !actor.is_null() {
                result = e.vcall(actor.addr(), SLOT_ACTOR_CELL, &args![]).ptr();
            }
        }
        4 | 5 => {
            if let Some(chosen) = process_reference(e, this, actor, SLOT_PROCESS_REFERENCE_514) {
                return if chosen.is_null() {
                    parent_cell_of(e, actor)
                } else {
                    parent_cell_of(e, chosen)
                };
            }
            if let Some(held) = player_dialogue_reference(e, this, actor) {
                result = parent_cell_of(e, held);
            }
        }
        6 if !actor.is_null() && !linked_reference(e, actor).is_null() => {
            let linked = linked_reference(e, actor);
            result = parent_cell_of(e, linked);
        }
        _ => {}
    }
    result
}

/// The position of a reference through its slot 0x1f4 (a pointer to the
/// three floats), copied as words.
fn reference_position(e: &mut Engine, reference: Object) -> [u32; 3] {
    let at = e.vcall(reference.addr(), SLOT_POSITION, &args![]).u32();
    [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)]
}

/// Slot 0x170 of an actor, which writes a point into a caller buffer and
/// returns a pointer to it; the three words, copied.
fn actor_point(e: &mut Engine, actor: Object) -> [u32; 3] {
    e.with_stack(12, |e, buffer| {
        let at = e
            .vcall(actor.addr(), SLOT_ACTOR_POINT, &args![buffer])
            .u32();
        [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)]
    })
}

/// The point of the first reference of a cell, or, for a cell without one
/// whose flag byte (`+0x24`, bit 0) is clear, the cell's corner
/// (`x << 12`, `y << 12`, 0) as floats; `None` for a cell without
/// references whose bit 0 is set (the caller keeps its default).
fn cell_point(e: &mut Engine, cell: Object, location: Object) -> Option<[u32; 3]> {
    let first = e.call(CELL_GET_FIRST_REFERENCE, &args![cell]).ptr::<()>();
    if !first.is_null() {
        return Some(reference_position(e, first));
    }
    if e.call(CELL_FLAG_1, &args![cell]).bool() {
        return None;
    }
    let corner_cell = location_cell(e, location);
    let x = e.call(CELL_GET_DATA_X, &args![corner_cell]).i32();
    let corner_cell = location_cell(e, location);
    let y = e.call(CELL_GET_DATA_Y, &args![corner_cell]).i32();
    Some([
        ((x << 12) as f64 as f32).to_bits(),
        ((y << 12) as f64 as f32).to_bits(),
        0.0f32.to_bits(),
    ])
}

/// The default point the coordinate getters start from (three floats in
/// the data section).
fn default_point(e: &mut Engine) -> [u32; 3] {
    [
        e.global(DEFAULT_POINT),
        e.global(DEFAULT_POINT + 4),
        e.global(DEFAULT_POINT + 8),
    ]
}

/// The three words at `at`.
fn point_at(e: &mut Engine, at: u32) -> [u32; 3] {
    [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)]
}

fn store_point(e: &mut Engine, out: Object, point: [u32; 3]) {
    for (i, word) in point.iter().enumerate() {
        e.mem.set_u32(out.addr() + 4 * i as u32, *word);
    }
}

// Translated from 00675360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetSecondLocationCoord` (Xbox PDB): writes the position of
/// the second location into `out` (a `NiPoint3`) and returns `out`. It
/// starts from a default point in the data section. By location type: 0
/// the reference's position, 1 the first reference of the cell (else the
/// corner of an exterior cell), 3 the actor's (slot 0x170), 4 and 5 the
/// position of the chosen reference or of the actor, 6 that of the linked
/// reference; no second location or type 2: the start location of the
/// actor.
pub fn tes_package_get_second_location_coord(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    out: Object,
    actor: Object,
) -> Object {
    let location = fn_00672dd0(e, this);
    let mut point = default_point(e);
    if location.is_null() || location_type(e, location) == 2 {
        if !actor.is_null() {
            let at = e.call(START_LOCATION_COORD, &args![actor]).u32();
            point = point_at(e, at);
        }
        store_point(e, out, point);
        return out;
    }
    match location_type(e, location) {
        0 => {
            if !location_reference(e, location).is_null() {
                let reference = location_reference(e, location);
                point = reference_position(e, reference);
            }
        }
        1 => {
            let cell = location_cell(e, location);
            if !cell.is_null() {
                if let Some(found) = cell_point(e, cell, location) {
                    point = found;
                }
            }
        }
        3 => {
            if !actor.is_null() {
                point = actor_point(e, actor);
            }
        }
        4 | 5 => {
            if let Some(chosen) = process_reference(e, this, actor, SLOT_PROCESS_REFERENCE_51C) {
                let source = if chosen.is_null() { actor } else { chosen };
                point = reference_position(e, source);
                store_point(e, out, point);
                return out;
            }
            if let Some(held) = player_dialogue_reference(e, this, actor) {
                point = reference_position(e, held);
            }
        }
        6 if !actor.is_null() && !linked_reference(e, actor).is_null() => {
            let linked = linked_reference(e, actor);
            point = reference_position(e, linked);
        }
        _ => {}
    }
    store_point(e, out, point);
    out
}

// Translated from 00675670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The radius of the second location (`0067f1c0`: its `+0x04` word unless
/// the location type is 0xff or 1), or 0 without one.
pub fn fn_00675670(e: &mut Engine, this: Ptr<TESPackage>) -> u32 {
    let location = fn_00672dd0(e, this);
    if location.is_null() {
        return 0;
    }
    e.call(LOCATION_GET_RADIUS, &args![location]).u32()
}

// Translated from 006756a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reference the second location stands for, for the actor `actor`:
/// the actor itself when there is no second location, or for type 2 or 3;
/// type 0 the location's reference; 4 and 5 the reference the actor's
/// process chose (else the actor); 6 the linked reference. Null where the
/// actor is needed and null.
pub fn fn_006756a0(e: &mut Engine, this: Ptr<TESPackage>, actor: Object) -> Object {
    let location = fn_00672dd0(e, this);
    let mut result = Ptr::NULL;
    if location.is_null() || location_type(e, location) == 2 {
        if !actor.is_null() {
            result = actor;
        }
        return result;
    }
    match location_type(e, location) {
        0 => {
            if !location_reference(e, location).is_null() {
                result = location_reference(e, location);
            }
        }
        3 => {
            if !actor.is_null() {
                result = actor;
            }
        }
        4 | 5 => {
            if let Some(chosen) = process_reference(e, this, actor, SLOT_PROCESS_REFERENCE_51C) {
                result = if chosen.is_null() { actor } else { chosen };
            }
        }
        6 if !actor.is_null() && !linked_reference(e, actor).is_null() => {
            result = linked_reference(e, actor);
        }
        _ => {}
    }
    result
}

// Translated from 006757e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetSearchLocationCell` (Xbox PDB): the cell of the
/// package's own location when the search location is that location,
/// otherwise the cell of the second location.
pub fn tes_package_get_search_location_cell(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
) -> Object {
    let search = tes_package_get_package_search_location(e, this);
    let own = location_of(e, this);
    if search == own {
        tes_package_get_location_cell(e, this, actor)
    } else {
        tes_package_get_second_location_cell(e, this, actor)
    }
}

// Translated from 00675830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetSearchLocationCoord` (Xbox PDB): writes into `out` the
/// position of the own location or of the second location, as
/// `GetSearchLocationCell` chooses, and returns `out`.
pub fn tes_package_get_search_location_coord(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    out: Object,
    actor: Object,
) -> Object {
    let search = tes_package_get_package_search_location(e, this);
    let own = location_of(e, this);
    let point = e.with_stack(12, |e, buffer| {
        let made = if search == own {
            tes_package_get_location_coord(e, this, buffer, actor)
        } else {
            tes_package_get_second_location_coord(e, this, buffer, actor)
        };
        point_at(e, made.addr())
    });
    store_point(e, out, point);
    out
}

// Translated from 006758a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetSearchLocationRadius` (Xbox PDB): the radius of the own
/// location (`00676280`, which wants the actor) or of the second location.
pub fn tes_package_get_search_location_radius(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
) -> i32 {
    let search = tes_package_get_package_search_location(e, this);
    let own = location_of(e, this);
    if search == own {
        fn_00676280(e, this, actor)
    } else {
        fn_00675670(e, this) as i32
    }
}

/// The per-procedure data object when the package type is `kind`, else null.
fn data_of_type(e: &mut Engine, this: Ptr<TESPackage>, kind: i32) -> Object {
    if pack_type(e, this) == kind {
        e.get(this, TESPackage::pPackData)
    } else {
        Ptr::NULL
    }
}

// Translated from 006758f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetUseWeaponPackageData` (Xbox PDB): the data object of a
/// type 0x10 package, else null.
pub fn tes_package_get_use_weapon_package_data(e: &mut Engine, this: Ptr<TESPackage>) -> Object {
    data_of_type(e, this, TYPE_USE_WEAPON)
}

// Translated from 00675920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetPatrolPackageData` (Xbox PDB): the data object of a
/// type 0xd package, else null.
pub fn tes_package_get_patrol_package_data(e: &mut Engine, this: Ptr<TESPackage>) -> Object {
    data_of_type(e, this, TYPE_PATROL)
}

// Translated from 00675950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetFollowPackageData` (Xbox PDB): the data object of a
/// type 1 package, else null.
pub fn tes_package_get_follow_package_data(e: &mut Engine, this: Ptr<TESPackage>) -> Object {
    data_of_type(e, this, TYPE_FOLLOW)
}

// Translated from 00675980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The data object of a type 0xe package, else null.
pub fn fn_00675980(e: &mut Engine, this: Ptr<TESPackage>) -> Object {
    data_of_type(e, this, TYPE_0E)
}

// Translated from 006759b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetEscortPackageData` (Xbox PDB): the data object of a
/// type 2 package, else null.
pub fn tes_package_get_escort_package_data(e: &mut Engine, this: Ptr<TESPackage>) -> Object {
    data_of_type(e, this, TYPE_ESCORT)
}

// Translated from 006759e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::ComputeInitialTarget` (Xbox PDB): the reference the target
/// stands for: for target type 0 its reference, for type 3 the actor's
/// linked reference; null otherwise.
pub fn tes_package_compute_initial_target(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
) -> Object {
    if fn_00671d10(e, this).is_null() {
        return Ptr::NULL;
    }
    let target = fn_00671d10(e, this);
    match e.call(TARGET_GET_TYPE, &args![target]).u32() {
        0 => {
            let target = fn_00671d10(e, this);
            e.call(TARGET_GET_REFERENCE, &args![target]).ptr()
        }
        3 => linked_reference(e, actor),
        _ => Ptr::NULL,
    }
}

/// The start-location fallback of the first-location getters: the
/// actor's package start location, which, when the actor has none yet, is
/// first set from its current location. Returns what `read` answers after
/// that. `first` is the answer of the start-location getter already read.
fn start_location_or_current(
    e: &mut Engine,
    actor: Object,
    first: Object,
    read: impl FnOnce(&mut Engine) -> Object,
) -> Object {
    if first.is_null() {
        e.call(SET_START_LOCATION_FROM_CURRENT, &args![actor]);
        return read(e);
    }
    first
}

// Translated from 00675a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetLocationWorld` (Xbox PDB): the world space of the
/// package's own location for the actor `actor`. By location type: 0 the
/// reference's (the actor's when the location has none), 1 the cell's, 3
/// the actor's own (slot 0x294), 4 and 5 that of the reference the
/// actor's process chose (or the actor), 6 the linked reference's (else
/// the actor's). No location, or type 2: the actor's start location's
/// world space, setting the start location from the current one first
/// when it has none.
pub fn tes_package_get_location_world(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
) -> Object {
    let location = location_of(e, this);
    if location.is_null() || location_type(e, location) == 2 {
        if actor.is_null() {
            return Ptr::NULL;
        }
        let first = e.call(START_LOCATION_WORLD, &args![actor]).ptr();
        return start_location_or_current(e, actor, first, |e| world_of_reference(e, actor));
    }
    let mut result = Ptr::NULL;
    match location_type(e, location) {
        0 => {
            if location_reference(e, location).is_null() {
                if !actor.is_null() {
                    result = world_of_reference(e, actor);
                }
            } else {
                let reference = location_reference(e, location);
                result = world_of_reference(e, reference);
            }
        }
        1 => {
            if !location_cell(e, location).is_null() {
                let cell = location_cell(e, location);
                result = e.call(CELL_GET_WORLD_SPACE, &args![cell]).ptr();
            }
        }
        3 => {
            if !actor.is_null() {
                result = e
                    .vcall(actor.addr(), SLOT_ACTOR_WORLD_SPACE, &args![])
                    .ptr();
            }
        }
        4 | 5 => {
            if let Some(chosen) = process_reference(e, this, actor, SLOT_PROCESS_REFERENCE_514) {
                result = if chosen.is_null() {
                    world_of_reference(e, actor)
                } else {
                    world_of_reference(e, chosen)
                };
            }
        }
        6 => {
            if actor.is_null() || linked_reference(e, actor).is_null() {
                if !actor.is_null() {
                    result = world_of_reference(e, actor);
                }
            } else {
                let linked = linked_reference(e, actor);
                result = world_of_reference(e, linked);
            }
        }
        _ => {}
    }
    result
}

// Translated from 00675c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetLocationCell` (Xbox PDB): as `GetLocationWorld`, for
/// the cell: type 0 the reference's parent cell (the actor's when the
/// location has no reference), 1 the cell itself, 3 the actor's (slot
/// 0x298), 4 and 5 that of the chosen reference or the actor, 6 that of
/// the linked reference (else the actor's); no location or type 2: the
/// actor's start location's interior cell, set from the current location
/// first when missing.
pub fn tes_package_get_location_cell(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
) -> Object {
    let location = location_of(e, this);
    if location.is_null() || location_type(e, location) == 2 {
        if actor.is_null() {
            return Ptr::NULL;
        }
        let first = e.call(START_LOCATION_INTERIOR_CELL, &args![actor]).ptr();
        return start_location_or_current(e, actor, first, |e| parent_cell_of(e, actor));
    }
    let mut result = Ptr::NULL;
    match location_type(e, location) {
        0 => {
            if location_reference(e, location).is_null() {
                if !actor.is_null() {
                    result = parent_cell_of(e, actor);
                }
            } else {
                let reference = location_reference(e, location);
                result = parent_cell_of(e, reference);
            }
        }
        1 => result = location_cell(e, location),
        3 => {
            if !actor.is_null() {
                result = e.vcall(actor.addr(), SLOT_ACTOR_CELL, &args![]).ptr();
            }
        }
        4 | 5 => {
            if let Some(chosen) = process_reference(e, this, actor, SLOT_PROCESS_REFERENCE_514) {
                result = if chosen.is_null() {
                    parent_cell_of(e, actor)
                } else {
                    parent_cell_of(e, chosen)
                };
            }
        }
        6 => {
            if actor.is_null() || linked_reference(e, actor).is_null() {
                if !actor.is_null() {
                    result = parent_cell_of(e, actor);
                }
            } else {
                let linked = linked_reference(e, actor);
                result = parent_cell_of(e, linked);
            }
        }
        _ => {}
    }
    result
}

/// Logs the loader warning `message` (a format with one `%s`) with the
/// actor's name (actor slot 0x130), then returns the actor's point.
fn warn_and_actor_point(e: &mut Engine, actor: Object, message: u32) -> [u32; 3] {
    let name = e.vcall(actor.addr(), SLOT_FORM_NAME, &args![]).u32();
    e.call(LOG_FORM_WARNING, &args![message, name]);
    actor_point(e, actor)
}

// Translated from 00675de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetLocationCoord` (Xbox PDB): writes the position of the
/// package's own location for the actor `actor` into `out` and returns
/// `out`. It starts from the default point in the data section. By
/// location type: 0 the reference's position (the actor's own point,
/// after a warning, when the location has no reference), 1 the first
/// reference of the cell (else the corner of an exterior cell), 3 the
/// actor's point (slot 0x170), 4 and 5 the position of the chosen
/// reference or of the actor, 6 that of the linked reference (the
/// actor's point after a warning when it has none). No location, or type
/// 2: the actor's start location (set from the current location first
/// when the actor's extra data has none).
pub fn tes_package_get_location_coord(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    out: Object,
    actor: Object,
) -> Object {
    let location = location_of(e, this);
    let mut point = default_point(e);
    if location.is_null() || location_type(e, location) == 2 {
        if !actor.is_null() {
            let extra = e.call(GET_EXTRA_DATA_LIST, &args![actor]).u32();
            if e.call(EXTRA_GET_START_LOCATION, &args![extra]).u32() == 0 {
                e.call(SET_START_LOCATION_FROM_CURRENT, &args![actor]);
            }
        }
        if !actor.is_null() {
            let at = e.call(START_LOCATION_COORD, &args![actor]).u32();
            point = point_at(e, at);
        }
        store_point(e, out, point);
        return out;
    }
    match location_type(e, location) {
        0 => {
            if !location_reference(e, location).is_null() {
                let reference = location_reference(e, location);
                point = reference_position(e, reference);
            } else if !actor.is_null() {
                point = warn_and_actor_point(e, actor, MESSAGE_NO_REFERENCE_LOCATION);
            }
        }
        1 => {
            let cell = location_cell(e, location);
            if !cell.is_null() {
                if let Some(found) = cell_point(e, cell, location) {
                    point = found;
                }
            }
        }
        3 => {
            if !actor.is_null() {
                point = actor_point(e, actor);
            }
        }
        4 | 5 => {
            if let Some(chosen) = process_reference(e, this, actor, SLOT_PROCESS_REFERENCE_514) {
                let source = if chosen.is_null() { actor } else { chosen };
                point = reference_position(e, source);
            }
        }
        6 => {
            if actor.is_null() || linked_reference(e, actor).is_null() {
                if !actor.is_null() {
                    point = warn_and_actor_point(e, actor, MESSAGE_NO_LINKED_REFERENCE);
                }
            } else {
                let linked = linked_reference(e, actor);
                point = reference_position(e, linked);
            }
        }
        _ => {}
    }
    store_point(e, out, point);
    out
}

// Translated from 00676140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetLocationReference` (Xbox PDB): the reference the own
/// location stands for, for the actor `actor`: the actor itself when
/// there is no location, or for type 2 or 3; type 0 the location's
/// reference; 4 and 5 the reference the actor's process chose (else the
/// actor); 6 the linked reference. Null where the actor is needed and
/// null.
pub fn tes_package_get_location_reference(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
) -> Object {
    let location = location_of(e, this);
    let mut result = Ptr::NULL;
    if location.is_null() || location_type(e, location) == 2 {
        if !actor.is_null() {
            result = actor;
        }
        return result;
    }
    match location_type(e, location) {
        0 => {
            if !location_reference(e, location).is_null() {
                result = location_reference(e, location);
            }
        }
        3 => {
            if !actor.is_null() {
                result = actor;
            }
        }
        4 | 5 => {
            if let Some(chosen) = process_reference(e, this, actor, SLOT_PROCESS_REFERENCE_514) {
                result = if chosen.is_null() { actor } else { chosen };
            }
        }
        6 if !actor.is_null() && !linked_reference(e, actor).is_null() => {
            result = linked_reference(e, actor);
        }
        _ => {}
    }
    result
}

// Translated from 00676280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The radius of the package's own location for the actor `actor`. Without
/// a location it is 0. When the location resolves (`0067f2a0`) to a
/// reference whose base form has the form type 0x15 and the location's
/// own radius is zero, the radius is half of the smallest component of
/// the base form's bounds (`0050eb90` minus `0050ead0`), truncated;
/// otherwise it is the location's own radius (`0067f1c0`).
pub fn fn_00676280(e: &mut Engine, this: Ptr<TESPackage>, actor: Object) -> i32 {
    let location = location_of(e, this);
    if location.is_null() {
        return 0;
    }
    let reference = e
        .call(LOCATION_RESOLVE_REFERENCE, &args![location, actor])
        .ptr::<()>();
    if !reference.is_null() {
        let base = e.call(GET_BASE_FORM, &args![reference]).ptr::<()>();
        if e.call(GET_FORM_TYPE, &args![base]).u32() == FORM_TYPE_0X15 {
            let own = e.call(LOCATION_GET_RADIUS, &args![location]).u32();
            let zero: f64 = e.global(ZERO);
            if own as f64 == zero {
                return e.with_stack(12, |e, high| {
                    e.with_stack(12, |e, low| {
                        e.call(BOUNDS_HIGH, &args![base, high]);
                        let low_point = e.call(BOUNDS_LOW, &args![base, low]).u32();
                        e.call(POINT_SUBTRACT, &args![high, low_point]);
                        let x = e.mem.f32(high.addr());
                        let y = e.mem.f32(high.addr() + 4);
                        let z = e.mem.f32(high.addr() + 8);
                        let smaller = e.call(FLOAT_MIN, &args![x, y]).f32();
                        let smallest = e.call(FLOAT_MIN, &args![smaller, z]).f32();
                        let half: f64 = e.global(HALF);
                        let radius = (smallest as f64 * half) as f32;
                        radius as i64 as i32
                    })
                });
            }
        }
    }
    e.call(LOCATION_GET_RADIUS, &args![location]).i32()
}

/// Whether the reference is a furniture object (`00568680`: the base form
/// is set and has form type 0x27).
fn is_furniture(e: &mut Engine, reference: Object) -> bool {
    e.call(IS_FURNITURE, &args![reference]).bool()
}

/// The base form of a reference (`+0x20`).
fn base_form_of(e: &mut Engine, reference: Object) -> Object {
    e.call(GET_BASE_FORM, &args![reference]).ptr()
}

/// Whether the base form of `reference` is one of the two forms the data
/// section keeps at `0x011ca248` and `0x011ca244`.
fn is_one_of_two_marker_forms(e: &mut Engine, reference: Object) -> bool {
    let first: u32 = e.global(MARKER_FORM_A);
    if base_form_of(e, reference).addr() == first {
        return true;
    }
    let second: u32 = e.global(MARKER_FORM_B);
    base_form_of(e, reference).addr() == second
}

/// `_ftol2` of a float (the game passes it in `ST0`).
fn float_to_int(e: &mut Engine, value: f32) -> i32 {
    e.call(FTOL, &args![value as f64]).i32()
}

/// The size of the reference's model bound as an integer (`_ftol2` of
/// `00571600`).
fn model_bound_size(e: &mut Engine, reference: Object) -> i32 {
    let size = e.call(MODEL_BOUND_SIZE, &args![reference]).f32();
    float_to_int(e, size)
}

/// The default radius setting: `0043d4d0` on the setting object at
/// `0x011cde98` gives the address of its value.
fn default_radius_setting(e: &mut Engine) -> i32 {
    let at = e
        .call(SETTING_VALUE_ADDRESS, &args![DEFAULT_RADIUS_SETTING])
        .u32();
    e.mem.i32(at)
}

/// The radius the movement tests use for a reference that stands for the
/// location, and whether it sets the "exact" flag: for a marker base form
/// 20 and exact; for an actor of kind 9 (slot 0x214) 90 and exact, any
/// other actor its model bound plus 20; for anything else its model bound
/// plus a margin that is 0 for the base form types 0x15, 0x1b, 0x20 and
/// 0x21 and 20.0 otherwise, never negative.
fn radius_for_reference(e: &mut Engine, reference: Object) -> (i32, bool) {
    if is_one_of_two_marker_forms(e, reference) {
        return (0x14, true);
    }
    if e.vcall(reference.addr(), SLOT_IS_ACTOR, &args![]).bool() {
        if e.vcall(reference.addr(), SLOT_KIND, &args![]).i32() == 9 {
            return (0x5a, true);
        }
        let bound = model_bound_size(e, reference);
        return (bound.wrapping_add(0x14), false);
    }
    let mut margin: f32 = e.global(DEFAULT_MARGIN);
    let base = base_form_of(e, reference);
    let kind = e.call(GET_FORM_TYPE, &args![base]).u32();
    if matches!(kind, 0x15 | 0x1b | 0x20 | 0x21) {
        margin = 0.0;
    }
    let bound = model_bound_size(e, reference);
    let margin = float_to_int(e, margin);
    let radius = bound.wrapping_add(margin);
    (if radius < 0 { 0 } else { radius }, false)
}

/// Whether the actor's process holds a point (its slot 0x4d4 is non-zero).
fn process_has_point(e: &mut Engine, actor: Object) -> bool {
    if process_of(e, actor).is_null() {
        return false;
    }
    let process = process_of(e, actor);
    e.vcall(process.addr(), SLOT_PROCESS_POINT, &args![]).u32() != 0
}

/// The process's point (slot 0x4d4), copied over `point`.
fn copy_process_point(e: &mut Engine, actor: Object, point: u32) {
    let process = process_of(e, actor);
    let at = e.vcall(process.addr(), SLOT_PROCESS_POINT, &args![]).u32();
    let copy = point_at(e, at);
    store_point(e, Ptr::new(point), copy);
}

/// The flat distance between `point` and the position of `subject`
/// (`00439ef0` subtracts, `00589850` measures); the furniture cases of the
/// "is at the location" tests use it.
fn flat_distance_from(e: &mut Engine, point: u32, subject: Object) -> f32 {
    let subject_at = e.vcall(subject.addr(), SLOT_POSITION, &args![]).u32();
    e.with_stack(12, |e, difference| {
        e.call(POINT_DIFFERENCE, &args![point, difference, subject_at]);
        e.call(FLAT_LENGTH, &args![difference]).f32()
    })
}

/// The subject's slot 0x2cc: whether it is within `radius` of `point`.
fn subject_reaches(e: &mut Engine, subject: Object, point: u32, radius: i32, exact: bool) -> bool {
    e.vcall(
        subject.addr(),
        SLOT_REACH,
        &args![point, radius as f32, !exact, 1u32],
    )
    .bool()
}

/// The outcome of the place test the two "is at the location" functions
/// start with.
enum Place {
    /// A type 1 location in the same cell: the answer is yes at once.
    Immediate,
    /// The subject is in the same place; carries the subject's cell (null
    /// when it is not an interior cell).
    Same(Object),
    /// Not in the same place: the answer is no.
    Different,
}

/// The place test: the subject's world space and cell against the
/// location's (a cell without the flag `00425fd0` tests counts as none).
fn same_place(
    e: &mut Engine,
    location: Object,
    subject: Object,
    location_world: Object,
    location_cell_found: Object,
) -> Place {
    let mut location_cell_found = location_cell_found;
    if !location_cell_found.is_null() && !e.call(CELL_FLAG_1, &args![location_cell_found]).bool() {
        location_cell_found = Ptr::NULL;
    }
    let subject_world = world_of_reference(e, subject);
    let mut subject_cell = parent_cell_of(e, subject);
    if !subject_cell.is_null() && !e.call(CELL_FLAG_1, &args![subject_cell]).bool() {
        subject_cell = Ptr::NULL;
    }
    let mut same = false;
    if !subject_cell.is_null() || !location_cell_found.is_null() {
        if subject_cell == location_cell_found {
            same = true;
            if !location.is_null() && location_type(e, location) == 1 {
                return Place::Immediate;
            }
        }
    } else if subject_world == location_world {
        same = true;
    }
    if same {
        Place::Same(subject_cell)
    } else {
        Place::Different
    }
}

// Translated from 00676390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the subject `subject` stands at the package's second location
/// for the actor `actor`. No second location is always true; a location
/// of type 3 is true when the actor's slot 0x290 says no. The subject must
/// be in the same world space or cell as the location (a type 1 location
/// answers true right away on a cell match); then the location's own test
/// (`0067f220`) may decide; then the radius is worked out (the location's
/// radius, or 0 when `ignore_extra` is set, or `extra_radius` unless it is
/// -1.0; with a positive location radius `furniture_check` is dropped),
/// from the reference the location stands for when it is still 0
/// (`radius_for_reference`), else from the default radius setting, and
/// the subject's slot 0x2cc decides whether it is within it. For a type 5
/// package the height difference to the second location may be at most
/// 100. With `furniture_check` set and a furniture reference whose subject
/// process holds a point, the answer is whether the subject is an actor of
/// kind 4 or 9 whose process slot 0x4c8 gives that reference.
pub fn fn_00676390(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    subject: Object,
    actor: Object,
    ignore_extra: u8,
    extra_radius: f32,
    furniture_check: u8,
) -> bool {
    e.with_stack(12, |e, point| {
        second_location_test(
            e,
            this,
            subject,
            actor,
            ignore_extra,
            extra_radius,
            furniture_check,
            point.addr(),
        )
    })
}

#[allow(clippy::too_many_arguments)]
fn second_location_test(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    subject: Object,
    actor: Object,
    ignore_extra: u8,
    extra_radius: f32,
    furniture_check: u8,
    point: u32,
) -> bool {
    if subject.is_null() {
        return false;
    }
    let mut furniture_check = furniture_check;
    let mut subject_actor = Ptr::NULL;
    if e.vcall(subject.addr(), SLOT_IS_ACTOR, &args![]).bool() {
        subject_actor = subject;
    }
    let location = fn_00672dd0(e, this);
    if location.is_null() {
        return true;
    }
    if location_type(e, location) == 3 && !e.vcall(actor.addr(), SLOT_ACTOR_290, &args![]).bool() {
        return true;
    }
    let location_world = tes_package_get_second_location_world(e, this, actor);
    let location_cell_found = tes_package_get_second_location_cell(e, this, actor);
    let subject_cell = match same_place(e, location, subject, location_world, location_cell_found) {
        Place::Immediate => return true,
        Place::Same(cell) => cell,
        Place::Different => return false,
    };
    let (decided, flag) = e.with_stack(4, |e, flag| {
        let decided = e
            .call(
                LOCATION_TEST_SUBJECT,
                &args![location, actor, subject, flag],
            )
            .bool();
        (decided, e.mem.u8(flag.addr()))
    });
    if decided {
        return flag != 0;
    }

    let mut radius = fn_00675670(e, this) as i32;
    if radius > 0 {
        furniture_check = 0;
    }
    if ignore_extra != 0 {
        radius = 0;
    } else {
        let sentinel: f64 = e.global(NO_EXTRA_RADIUS);
        if extra_radius as f64 != sentinel {
            radius = float_to_int(e, extra_radius);
        }
    }
    let mut exact = false;
    tes_package_get_second_location_coord(e, this, Ptr::new(point), actor);
    let reference = fn_006756a0(e, this, actor);
    if radius == 0 {
        if !reference.is_null() {
            if is_furniture(e, reference) && process_has_point(e, subject) {
                let holder = process_of(e, actor);
                if e.call(PROCESS_FIELD_28, &args![holder]).u32() == 0 {
                    copy_process_point(e, actor, point);
                }
                radius = 0x14;
                exact = true;
            } else {
                let (found, flag) = radius_for_reference(e, reference);
                radius = found;
                exact = flag;
            }
        } else if location_type(e, location) == 3 {
            exact = true;
            radius = 10;
        }
    }
    if radius == 0 {
        radius = default_radius_setting(e);
    }
    if pack_type(e, this) == 5 && !subject_cell.is_null() {
        let subject_at = e.vcall(subject.addr(), SLOT_POSITION, &args![]).u32();
        let second_height = e.with_stack(12, |e, buffer| {
            let made = tes_package_get_second_location_coord(e, this, buffer, actor);
            e.mem.f32(made.addr() + 8)
        });
        let difference = (e.mem.f32(subject_at + 8) as f64 - second_height as f64) as f32;
        let length = e.call(ABSOLUTE_VALUE, &args![difference]).f32();
        let limit: f64 = e.global(DISTANCE_LIMIT);
        if length as f64 > limit {
            return false;
        }
    }
    if furniture_check != 0
        && !reference.is_null()
        && is_furniture(e, reference)
        && process_has_point(e, subject)
    {
        // The difference is computed and not used.
        flat_distance_from(e, point, subject);
        if subject_actor.is_null() {
            return false;
        }
        if e.vcall(subject_actor.addr(), SLOT_KIND, &args![]).i32() != 4
            && e.vcall(subject_actor.addr(), SLOT_KIND, &args![]).i32() != 9
        {
            return false;
        }
        let process = process_of(e, subject_actor);
        return e
            .vcall(process.addr(), SLOT_PROCESS_4C8, &args![])
            .ptr::<()>()
            == reference;
    }
    subject_reaches(e, subject, point, radius, exact)
}

// Translated from 006768d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the subject `subject` (an actor running this package) stands
/// at the package's own location. As `fn_00676390`, for the own location
/// (`GetLocationWorld`, `GetLocationCell`, `GetLocationCoord`,
/// `GetLocationReference`, the radius `00676280`) and with the subject as
/// its own actor, with these differences: no location is not an automatic
/// yes (the world space and cell tests run against the start location);
/// `furniture_check`, when clear, is set for an actor of kind 4 or 9; the
/// location's own test runs only when its radius is zero; `skip_radius`
/// non-zero zeroes the radius, otherwise `extra_radius` (unless -1.0) is
/// added to it; for a location radius up to 20 and a reference whose base
/// form is one of the two marker forms the "exact" flag is set; and the
/// furniture case answers whether the subject, within 100 flat metres of
/// the point, is an actor of kind 4, or an actor of kind 9.
pub fn fn_006768d0(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    subject: Object,
    skip_radius: u8,
    extra_radius: f32,
    furniture_check: u8,
) -> bool {
    e.with_stack(12, |e, point| {
        own_location_test(
            e,
            this,
            subject,
            skip_radius,
            extra_radius,
            furniture_check,
            point.addr(),
        )
    })
}

fn own_location_test(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    subject: Object,
    skip_radius: u8,
    extra_radius: f32,
    furniture_check: u8,
    point: u32,
) -> bool {
    if subject.is_null() {
        return false;
    }
    let mut furniture_check = furniture_check;
    let mut subject_actor = Ptr::NULL;
    if e.vcall(subject.addr(), SLOT_IS_ACTOR, &args![]).bool() {
        subject_actor = subject;
    }
    if furniture_check == 0
        && !subject_actor.is_null()
        && (e.vcall(subject_actor.addr(), SLOT_KIND, &args![]).i32() == 4
            || e.vcall(subject_actor.addr(), SLOT_KIND, &args![]).i32() == 9)
    {
        furniture_check = 1;
    }
    let location = location_of(e, this);
    if !location.is_null()
        && location_type(e, location) == 3
        && !e.vcall(subject.addr(), SLOT_ACTOR_290, &args![]).bool()
    {
        return true;
    }
    let location_world = tes_package_get_location_world(e, this, subject);
    let location_cell_found = tes_package_get_location_cell(e, this, subject);
    match same_place(e, location, subject, location_world, location_cell_found) {
        Place::Immediate => return true,
        Place::Same(_) => {}
        Place::Different => return false,
    }
    if !location.is_null() && e.call(LOCATION_GET_RADIUS, &args![location]).u32() == 0 {
        let (decided, flag) = e.with_stack(4, |e, flag| {
            let decided = e
                .call(
                    LOCATION_TEST_SUBJECT,
                    &args![location, subject, subject, flag],
                )
                .bool();
            (decided, e.mem.u8(flag.addr()))
        });
        if decided {
            return flag != 0;
        }
    }

    let mut radius = fn_00676280(e, this, subject);
    if skip_radius != 0 {
        radius = 0;
    } else {
        let sentinel: f64 = e.global(NO_EXTRA_RADIUS);
        if extra_radius as f64 != sentinel {
            let extra = float_to_int(e, extra_radius);
            radius = extra.wrapping_add(radius);
        }
    }
    if radius > 0 {
        furniture_check = 0;
    }
    let mut exact = false;
    tes_package_get_location_coord(e, this, Ptr::new(point), subject_actor);
    let reference = if location.is_null() {
        Ptr::NULL
    } else {
        tes_package_get_location_reference(e, this, subject)
    };
    if radius == 0 && !location.is_null() {
        if !reference.is_null() {
            let mut base_if_furniture = Ptr::NULL;
            if is_furniture(e, reference) {
                base_if_furniture = base_form_of(e, reference);
            }
            if !base_if_furniture.is_null() && process_has_point(e, subject) {
                let holder = process_of(e, subject);
                if e.call(PROCESS_FIELD_28, &args![holder]).u32() == 0 {
                    copy_process_point(e, subject, point);
                }
                radius = 0x14;
                exact = true;
            } else {
                let (found, flag) = radius_for_reference(e, reference);
                radius = found;
                exact = flag;
            }
        } else if location_type(e, location) == 3 {
            exact = true;
            radius = 10;
        }
    }
    if (fn_00676280(e, this, subject) as u32) < 0x15
        && !reference.is_null()
        && is_one_of_two_marker_forms(e, reference)
    {
        exact = true;
    }
    if radius == 0 {
        radius = default_radius_setting(e);
    }
    if furniture_check != 0
        && !reference.is_null()
        && is_furniture(e, reference)
        && process_has_point(e, subject)
    {
        let length = flat_distance_from(e, point, subject_actor);
        let limit: f64 = e.global(DISTANCE_LIMIT);
        if (length as f64) < limit && e.vcall(subject.addr(), SLOT_KIND, &args![]).i32() == 4 {
            return true;
        }
        return e.vcall(subject.addr(), SLOT_KIND, &args![]).i32() == 9;
    }
    subject_reaches(e, subject, point, radius, exact)
}

/// The reference the actor is heading for (actor slot 0x2c8), or null.
fn actor_target_reference(e: &mut Engine, actor: Object) -> Object {
    e.vcall(actor.addr(), SLOT_TARGET_REFERENCE, &args![]).ptr()
}

/// The reference's cell, or null for a cell without the flag `00425fd0`
/// tests.
fn flagged_parent_cell(e: &mut Engine, reference: Object) -> Object {
    let cell = parent_cell_of(e, reference);
    if !cell.is_null() && !e.call(CELL_FLAG_1, &args![cell]).bool() {
        return Ptr::NULL;
    }
    cell
}

/// Whether two references share a world space and cell the way the
/// "is at" tests compare them: when either has an interior cell the cells
/// decide, otherwise the world spaces.
fn same_cell_or_world(
    first_world: Object,
    first_cell: Object,
    second_world: Object,
    second_cell: Object,
) -> bool {
    if !first_cell.is_null() || !second_cell.is_null() {
        first_cell == second_cell
    } else {
        first_world == second_world
    }
}

// Translated from 00676e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::IsActorAtRefTarget` (Xbox PDB): whether the actor `actor`
/// has reached the reference it is heading for (its slot 0x2c8), with the
/// tolerance `extra` added to the radius. The actor and the reference must
/// share a world space or cell; the radius comes from
/// `GetPackageRadiusActorToRefTarget` and is replaced by 10 for a
/// furniture reference whose process holds a point (the actor's goal
/// point then becomes that point) or for a form type 0x30 reference, by 90
/// or 200 for some actors of kind 9 or 4, by the model bound plus 20 for
/// other actors, and by 20 for the two marker forms. Slot 0x2cc of the
/// actor decides. The loader's error with the radius and distance is
/// raised when the actor is the interface's target and the distance is
/// finite. Not translated: the stack cookie check.
pub fn tes_package_is_actor_at_ref_target(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
    extra: i32,
) -> bool {
    if actor.is_null() || actor_target_reference(e, actor).is_null() {
        return false;
    }
    e.with_stack(12, |e, point| {
        e.with_stack(12, |e, difference| {
            actor_at_ref_target(e, this, actor, extra, point.addr(), difference.addr())
        })
    })
}

fn actor_at_ref_target(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
    extra: i32,
    point: u32,
    difference: u32,
) -> bool {
    let mut actor_as_actor = Ptr::NULL;
    if e.vcall(actor.addr(), SLOT_IS_ACTOR, &args![]).bool() {
        actor_as_actor = actor;
    }
    let mut exact = false;
    let target = actor_target_reference(e, actor);
    let target_position = reference_position(e, target);
    store_point(e, Ptr::new(point), target_position);
    if e.call(HAS_LINKED_DOOR, &args![target]).u32() != 0 {
        let at = e.call(LINKED_DOOR_TELEPORT_POSITION, &args![target]).u32();
        let teleport = point_at(e, at);
        store_point(e, Ptr::new(point), teleport);
    }
    let target_world = world_of_reference(e, target);
    let target_cell = flagged_parent_cell(e, target);
    let actor_world = world_of_reference(e, actor);
    let actor_cell = flagged_parent_cell(e, actor);
    if !same_cell_or_world(actor_world, actor_cell, target_world, target_cell) {
        return false;
    }
    let mut radius = {
        let radius = tes_package_get_package_radius_actor_to_ref_target(e, this, actor, 0);
        float_to_int(e, radius)
    };
    let mut result = false;
    let actor_at = e.vcall(actor.addr(), SLOT_POSITION, &args![]).u32();
    e.call(POINT_DIFFERENCE, &args![point, difference, actor_at]);
    if is_furniture(e, target) && process_has_point(e, actor) {
        let holder = process_of(e, actor);
        if e.call(PROCESS_FIELD_28, &args![holder]).u32() == 0 {
            copy_process_point(e, actor, point);
        }
        radius = 10;
        exact = true;
    } else {
        let base = base_form_of(e, target);
        if e.call(GET_FORM_TYPE, &args![base]).u32() == 0x30 {
            radius = 10;
            exact = true;
        } else {
            let player: u32 = e.global(PLAYER);
            if radius <= 0x14
                && e.vcall(target.addr(), SLOT_IS_ACTOR, &args![]).bool()
                && target.addr() != player
            {
                if e.vcall(target.addr(), SLOT_KIND, &args![]).i32() == 9 {
                    radius = 0x5a;
                    exact = true;
                } else if e.vcall(actor.addr(), SLOT_KIND, &args![]).i32() == 4 {
                    radius = 200;
                    exact = true;
                } else if radius <= 0x14 {
                    radius = model_bound_size(e, target).wrapping_add(0x14);
                }
            } else if is_one_of_two_marker_forms(e, target) {
                radius = 0x14;
                exact = true;
            }
        }
    }
    radius = radius.wrapping_add(extra);
    let height = e.mem.f32(difference + 8);
    let flat = e.call(ABSOLUTE_VALUE, &args![height]).f32() as f64;
    let setting_at = e
        .call(SETTING_FLOAT_ADDRESS, &args![HEIGHT_TOLERANCE_SETTING])
        .u32();
    if (e.mem.f32(setting_at) as f64) > flat {
        e.mem.set_f32(difference + 8, 0.0);
    }
    if subject_reaches_with(e, actor, point, radius, exact, 0) {
        result = true;
    }
    if !target.is_null() && is_furniture(e, target) && process_has_point(e, actor) {
        let actor_at = e.vcall(actor.addr(), SLOT_POSITION, &args![]).u32();
        let length = e.with_stack(12, |e, scratch| {
            e.call(POINT_DIFFERENCE, &args![point, scratch, actor_at]);
            e.call(FLAT_LENGTH, &args![scratch]).f32()
        });
        let limit: f64 = e.global(DISTANCE_LIMIT);
        let close_kind_4 = (length as f64) < limit
            && e.vcall(actor_as_actor.addr(), SLOT_KIND, &args![]).i32() == 4;
        if close_kind_4 || e.vcall(actor_as_actor.addr(), SLOT_KIND, &args![]).i32() == 9 {
            result = true;
        }
    }
    let interface_target = e.call(INTERFACE_TARGET_REFERENCE, &args![]).ptr::<()>();
    if interface_target == actor_as_actor {
        let length = e.call(FLAT_LENGTH, &args![difference]).f32();
        if e.call(CRT_FINITE, &args![length as f64]).i32() != 0 {
            e.with_stack(308, |e, buffer| {
                e.call(
                    CRT_SPRINTF,
                    &args![buffer, MESSAGE_RADIUS_DISTANCE, radius, length as f64],
                );
                e.call(REPORT_ERROR, &args![buffer]);
            });
        }
    }
    result
}

/// The subject's slot 0x2cc with a chosen last argument: whether it is
/// within `radius` of `point`.
fn subject_reaches_with(
    e: &mut Engine,
    subject: Object,
    point: u32,
    radius: i32,
    exact: bool,
    last: u32,
) -> bool {
    e.vcall(
        subject.addr(),
        SLOT_REACH,
        &args![point, radius as f32, !exact, last],
    )
    .bool()
}

// Translated from 006773a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::IsTargetAtLocation` (Xbox PDB): whether the reference the
/// actor `actor` is heading for stands at the package's own location. It
/// must share the location's world space or cell (`GetLocationWorld`,
/// `GetLocationCell`); the location's own test (`0067f220`) may decide;
/// the radius is the location's (`00676280`), or the model bound of the
/// reference it stands for, or the default radius setting when that is
/// zero or negative, plus `extra`; the answer is whether the flat
/// distance, truncated, is within it.
pub fn tes_package_is_target_at_location(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
    extra: i32,
) -> bool {
    if actor.is_null() || actor_target_reference(e, actor).is_null() {
        return false;
    }
    let target = actor_target_reference(e, actor);
    let target_world = world_of_reference(e, target);
    let target_cell = flagged_parent_cell(e, target);
    let location_world = tes_package_get_location_world(e, this, actor);
    let location_cell_found = tes_package_get_location_cell(e, this, actor);
    let location_cell_found = if !location_cell_found.is_null()
        && !e.call(CELL_FLAG_1, &args![location_cell_found]).bool()
    {
        Ptr::NULL
    } else {
        location_cell_found
    };
    if !same_cell_or_world(
        location_world,
        location_cell_found,
        target_world,
        target_cell,
    ) {
        return false;
    }
    let location = location_of(e, this);
    if !location.is_null() {
        let (decided, flag) = e.with_stack(4, |e, flag| {
            let decided = e
                .call(LOCATION_TEST_SUBJECT, &args![location, actor, target, flag])
                .bool();
            (decided, e.mem.u8(flag.addr()))
        });
        if decided {
            return flag != 0;
        }
    }
    let mut radius = fn_00676280(e, this, actor);
    if radius == 0 && !location.is_null() {
        let reference = tes_package_get_location_reference(e, this, actor);
        if !reference.is_null() {
            radius = model_bound_size(e, reference);
        }
    }
    if radius <= 0 {
        radius = default_radius_setting(e);
    }
    radius = radius.wrapping_add(extra);
    let target_at = e.vcall(target.addr(), SLOT_POSITION, &args![]).u32();
    let length = e.with_stack(12, |e, buffer| {
        e.with_stack(12, |e, difference| {
            let coord = tes_package_get_location_coord(e, this, buffer, actor);
            e.call(POINT_DIFFERENCE, &args![coord, difference, target_at]);
            e.call(FLAT_LENGTH, &args![difference]).f32()
        })
    });
    float_to_int(e, length) <= radius
}

// Translated from 00677570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::IsTargetAtSecondLocation` (Xbox PDB): as
/// `IsTargetAtLocation`, for the second location, with the radius a
/// float: when `radius` is negative it is worked out from the second
/// location's radius, or the model bound of the reference it stands for,
/// or the default radius setting. True when the flat distance, truncated,
/// is at most the radius.
pub fn tes_package_is_target_at_second_location(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
    radius: f32,
) -> bool {
    let target = actor_target_reference(e, actor);
    if actor.is_null() || target.is_null() {
        return false;
    }
    let target_world = world_of_reference(e, target);
    let target_cell = flagged_parent_cell(e, target);
    let location_world = tes_package_get_location_world(e, this, actor);
    let location_cell_found = tes_package_get_location_cell(e, this, actor);
    let location_cell_found = if !location_cell_found.is_null()
        && !e.call(CELL_FLAG_1, &args![location_cell_found]).bool()
    {
        Ptr::NULL
    } else {
        location_cell_found
    };
    if !same_cell_or_world(
        location_world,
        location_cell_found,
        target_world,
        target_cell,
    ) {
        return false;
    }
    let location = fn_00672dd0(e, this);
    if !location.is_null() {
        let (decided, flag) = e.with_stack(4, |e, flag| {
            let decided = e
                .call(LOCATION_TEST_SUBJECT, &args![location, actor, target, flag])
                .bool();
            (decided, e.mem.u8(flag.addr()))
        });
        if decided {
            return flag != 0;
        }
    }
    let mut radius = radius;
    if radius < 0.0 {
        let mut worked_out = fn_00675670(e, this) as i32;
        if worked_out == 0 {
            fn_00675670(e, this);
        }
        if worked_out == 0 && !location.is_null() {
            let reference = fn_006756a0(e, this, actor);
            if !reference.is_null() {
                worked_out = model_bound_size(e, reference);
            }
        }
        if worked_out <= 0 {
            worked_out = default_radius_setting(e);
        }
        radius = worked_out as f32;
    }
    let target_at = e.vcall(target.addr(), SLOT_POSITION, &args![]).u32();
    let length = e.with_stack(12, |e, buffer| {
        e.with_stack(12, |e, difference| {
            let coord = tes_package_get_second_location_coord(e, this, buffer, actor);
            e.call(POINT_DIFFERENCE, &args![coord, difference, target_at]);
            e.call(FLAT_LENGTH, &args![difference]).f32()
        })
    });
    let length = float_to_int(e, length);
    (radius as f64) >= (length as f64)
}

// Translated from 00677760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot 0x11c of the actor's process, or false for an actor without one.
pub fn fn_00677760(e: &mut Engine, _this: Ptr<TESPackage>, actor: Object) -> bool {
    if actor.is_null() || process_of(e, actor).is_null() {
        return false;
    }
    let process = process_of(e, actor);
    e.vcall(process.addr(), SLOT_PROCESS_11C, &args![]).bool()
}

/// What the two form-type tables of `CalculateProcedureType` make of a
/// form type or object type: sets the "ground" flag, answers procedure type
/// 0x16 at once, answers 0x1a at once, or does nothing.
enum Verdict {
    Flag,
    Procedure16,
    Procedure1a,
    Nothing,
}

/// The table for the base form of a target reference (and for the
/// reference passed in): `Nothing` for every other form type.
fn verdict_for_base_form(kind: u32) -> Verdict {
    match kind {
        0x15 | 0x1b | 0x1c | 0x20 | 0x21 | 0x25 | 0x26 | 0x27 | 0x2b => Verdict::Flag,
        0x2a => Verdict::Procedure16,
        _ => Verdict::Nothing,
    }
}

/// The table for the form type of a target object: every other form type
/// answers procedure type 0x1a.
fn verdict_for_object_form(kind: u32) -> Verdict {
    match kind {
        0x15 | 0x1c | 0x20 | 0x21 | 0x25 | 0x26 | 0x27 | 0x2b => Verdict::Flag,
        0x2a => Verdict::Procedure16,
        _ => Verdict::Procedure1a,
    }
}

/// The table for the object type of a target: every other value answers
/// procedure type 0x1a.
fn verdict_for_object_type(kind: u32) -> Verdict {
    match kind {
        1 | 6 | 10 | 0xb | 0xf => Verdict::Flag,
        0xe => Verdict::Procedure16,
        _ => Verdict::Procedure1a,
    }
}

/// The form type of the base form of `reference` (`007af430`, `00401170`).
fn base_form_type(e: &mut Engine, reference: Object) -> u32 {
    let base = base_form_of(e, reference);
    e.call(GET_FORM_TYPE, &args![base]).u32()
}

/// The target of the package (`00671d10`), fresh as the code reads it each
/// time.
fn target_of(e: &mut Engine, this: Ptr<TESPackage>) -> Object {
    fn_00671d10(e, this)
}

// Translated from 006777b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::CalculateProcedureType` (Xbox PDB): sets
/// `ePROCEDURE_TYPE` from the package type. Most package types map to a
/// fixed procedure type (a jump table over the type; types the table does
/// not know give -1). Types 0, 1 and 2 look at the package's target and
/// location: type 0 gives 0 or 1 without a target (by whether the
/// location has a value), otherwise 2 or 3 by whether the target is of a
/// "ground" kind of form (the verdict tables above), or 0x16 or 0x1a for
/// the others; type 1 gives 7, 0x2d or -1; type 2 gives 8, 9 or -1 by
/// whether the target is a form of type 0x2a or 0x2b or an object type 0xe
/// or 0xf. `reference` is the reference the target of kind 3 stands for.
/// A result of -1 logs nothing (the message is formatted into a buffer
/// that is dropped), resets the procedure type to 0 and changes the
/// package type to 6. Not translated: the stack cookie check.
pub fn tes_package_calculate_procedure_type(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    reference: Object,
) {
    let kind = pack_type(e, this);
    let procedure: Option<i32> = match kind {
        0 => procedure_type_0(e, this, reference),
        1 => Some(procedure_type_1(e, this)),
        2 => Some(procedure_type_2(e, this, reference)),
        3 => Some(5),
        4 => Some(4),
        5 => Some(1),
        6 => Some(0),
        7 => Some(0x1b),
        8 => Some(0x1c),
        9 => Some(0x1e),
        10 => Some(0x20),
        0xc => Some(0x25),
        0xd => Some(0x26),
        0xe => Some(0x29),
        0xf => Some(10),
        0x10 => Some(0x2c),
        0x12 => Some(0xc),
        0x14 => Some(0xd),
        0x15 => Some(0xb),
        0x16 => Some(0x13),
        0x17 => Some(0x14),
        0x18 => Some(0xf),
        0x1a => Some(0x15),
        0x1b => Some(0x19),
        0x1c => Some(10),
        0x1d => Some(0x1f),
        0x1e => Some(0x21),
        0x1f => Some(0x24),
        0x20 => Some(0x27),
        0x21 => Some(0x28),
        0x22 => Some(0x2a),
        0x23 => Some(0x2b),
        0x24 => Some(0x2e),
        _ => Some(-1),
    };
    if let Some(value) = procedure {
        e.set(this, TESPackage::ePROCEDURE_TYPE, value);
    }
    if e.get(this, TESPackage::ePROCEDURE_TYPE) == -1 {
        let name = e.vcall(this.addr(), SLOT_FORM_NAME, &args![]).u32();
        let text = e.call(PASS_THROUGH, &args![name, 0u32]).u32();
        e.with_stack(264, |e, buffer| {
            e.call(CRT_SPRINTF, &args![buffer, MESSAGE_INVALID_PACKAGE, text]);
        });
        e.set(this, TESPackage::ePROCEDURE_TYPE, 0);
        tes_package_set_pack_type(e, this, 6);
    }
}

/// Package type 0. `None` leaves the procedure type as it is (a target
/// that holds none of reference, object or object type).
fn procedure_type_0(e: &mut Engine, this: Ptr<TESPackage>, reference: Object) -> Option<i32> {
    if target_of(e, this).is_null() {
        let location = e.get(this, TESPackage::pPackLoc);
        if !location.is_null() && e.call(LOCATION_HAS_VALUE, &args![location]).bool() {
            return Some(1);
        }
        return Some(0);
    }
    let mut ground = false;
    if target_of(e, this).is_null() {
        return None;
    }
    let target = target_of(e, this);
    let has_reference = !e
        .call(TARGET_GET_REFERENCE, &args![target])
        .ptr::<()>()
        .is_null();
    if !has_reference {
        let target = target_of(e, this);
        if e.call(TARGET_GET_OBJECT, &args![target]).u32() == 0 {
            let target = target_of(e, this);
            if e.call(TARGET_GET_OBJECT_TYPE, &args![target]).u32() == 0 {
                return None;
            }
        }
    }
    let target = target_of(e, this);
    let target_kind = e.call(TARGET_GET_TYPE, &args![target]).u32();
    let verdict = match target_kind {
        0 => {
            let target = target_of(e, this);
            let held = e.call(TARGET_GET_REFERENCE, &args![target]).ptr();
            let kind = base_form_type(e, held);
            verdict_for_base_form(kind)
        }
        1 => {
            let target = target_of(e, this);
            let object = e.call(TARGET_GET_OBJECT, &args![target]).ptr::<()>();
            let kind = e.call(GET_FORM_TYPE, &args![object]).u32();
            verdict_for_object_form(kind)
        }
        2 => {
            let target = target_of(e, this);
            let object_type = e.call(TARGET_GET_OBJECT_TYPE, &args![target]).u32();
            verdict_for_object_type(object_type)
        }
        3 => {
            if !reference.is_null() {
                let kind = base_form_type(e, reference);
                verdict_for_base_form(kind)
            } else {
                Verdict::Nothing
            }
        }
        _ => Verdict::Nothing,
    };
    match verdict {
        Verdict::Flag => ground = true,
        Verdict::Procedure16 => return Some(0x16),
        Verdict::Procedure1a => return Some(0x1a),
        Verdict::Nothing => {}
    }
    Some(if ground { 2 } else { 3 })
}

/// Package type 1: -1 without a target; 7 when the second location is of
/// location type 6 or holds a reference, object or object type; else 0x2d.
fn procedure_type_1(e: &mut Engine, this: Ptr<TESPackage>) -> i32 {
    if target_of(e, this).is_null() {
        return -1;
    }
    let location = fn_00672dd0(e, this);
    if !location.is_null()
        && (location_type(e, location) == 6
            || !location_reference(e, location).is_null()
            || e.call(LOCATION_GET_OBJECT, &args![location]).u32() != 0
            || e.call(LOCATION_GET_OBJECT_TYPE, &args![location]).u32() != 0)
    {
        return 7;
    }
    0x2d
}

/// Package type 2: -1 without a target; 8 when the target stands for a
/// form of type 0x2a or 0x2b (an object of those types, or object type 0xe
/// or 0xf); else 9.
fn procedure_type_2(e: &mut Engine, this: Ptr<TESPackage>, reference: Object) -> i32 {
    let mut found = false;
    if target_of(e, this).is_null() {
        return -1;
    }
    let target = target_of(e, this);
    let target_kind = e.call(TARGET_GET_TYPE, &args![target]).u32();
    match target_kind {
        0 => {
            let target = target_of(e, this);
            if !e
                .call(TARGET_GET_REFERENCE, &args![target])
                .ptr::<()>()
                .is_null()
            {
                let target = target_of(e, this);
                let held = e.call(TARGET_GET_REFERENCE, &args![target]).ptr::<()>();
                if !base_form_of(e, held).is_null() {
                    let target = target_of(e, this);
                    let held = e.call(TARGET_GET_REFERENCE, &args![target]).ptr::<()>();
                    let mut kind = base_form_type(e, held);
                    if kind != 0x2a {
                        let target = target_of(e, this);
                        let held = e.call(TARGET_GET_REFERENCE, &args![target]).ptr::<()>();
                        kind = base_form_type(e, held);
                        found = kind == 0x2b;
                    } else {
                        found = true;
                    }
                }
            }
        }
        3 => {
            if !reference.is_null() && !base_form_of(e, reference).is_null() {
                let mut kind = base_form_type(e, reference);
                if kind != 0x2a {
                    kind = base_form_type(e, reference);
                    found = kind == 0x2b;
                } else {
                    found = true;
                }
            }
        }
        1 => {
            let target = target_of(e, this);
            let object = e.call(TARGET_GET_OBJECT, &args![target]).ptr::<()>();
            let mut matches = false;
            if !object.is_null() {
                let target = target_of(e, this);
                let object = e.call(TARGET_GET_OBJECT, &args![target]).ptr::<()>();
                matches = e.call(GET_FORM_TYPE, &args![object]).u32() == 0x2a;
            }
            if !matches {
                let target = target_of(e, this);
                let object = e.call(TARGET_GET_OBJECT, &args![target]).ptr::<()>();
                matches = e.call(GET_FORM_TYPE, &args![object]).u32() == 0x2b;
            }
            found = matches;
        }
        2 => {
            let target = target_of(e, this);
            let mut object_type = e.call(TARGET_GET_OBJECT_TYPE, &args![target]).u32();
            if object_type != 0xe {
                let target = target_of(e, this);
                object_type = e.call(TARGET_GET_OBJECT_TYPE, &args![target]).u32();
                found = object_type == 0xf;
            } else {
                found = true;
            }
        }
        _ => {}
    }
    if found {
        8
    } else {
        9
    }
}

/// The persistent-reference refinement both reference-finding cases of
/// `fn_006780e0` apply: a reference that has the form flag 0x20 (tested by
/// `00440d80`) and an extra data list is replaced by the reference the
/// list's pointer extra (`0041c8d0`) names.
fn refine_reference(e: &mut Engine, reference: Object) -> Object {
    if !reference.is_null()
        && e.call(FORM_FLAG_20, &args![reference]).bool()
        && e.call(GET_EXTRA_DATA_LIST, &args![reference]).u32() != 0
    {
        let list = e.call(GET_EXTRA_DATA_LIST, &args![reference]).u32();
        return e.call(EXTRA_GET_REFERENCE_POINTER, &args![list]).ptr();
    }
    reference
}

/// `TESDataHandler::EnumReferencesCloseToPoint` as `fn_006780e0` calls it:
/// the handler (`011c3f2c`), the actor's parent cell, its position twice
/// each followed by the radius setting at `011d65ac`, the callback
/// `fn_006784f0` and the search context. The first reference the callback
/// accepts is left in the global at `011d6580`, which is read and cleared.
fn find_close_reference(e: &mut Engine, actor: Object, context: u32) -> Object {
    let setting_at = e
        .call(SETTING_FLOAT_ADDRESS, &args![FOLLOW_RADIUS_SETTING])
        .u32();
    let radius_first = e.mem.u32(setting_at);
    let position_first = e.vcall(actor.addr(), SLOT_POSITION, &args![]).u32();
    let setting_at = e
        .call(SETTING_FLOAT_ADDRESS, &args![FOLLOW_RADIUS_SETTING])
        .u32();
    let radius_second = e.mem.u32(setting_at);
    let position_second = e.vcall(actor.addr(), SLOT_POSITION, &args![]).u32();
    let cell = parent_cell_of(e, actor);
    let handler: u32 = e.global(DATA_HANDLER);
    e.call(
        ENUM_REFERENCES_CLOSE_TO_POINT,
        &args![
            handler,
            cell,
            position_second,
            radius_second,
            position_first,
            radius_first,
            CALLBACK_FOLLOW_CANDIDATE,
            context
        ],
    );
    let found: u32 = e.global(FOUND_REFERENCE);
    e.set_global(FOUND_REFERENCE, 0u32);
    Ptr::new(found)
}

// Translated from 006780e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reference the package's target stands for, for the actor `actor`,
/// and, when it is a new actor to follow, registers the follower. By
/// target type: 0 its reference, 3 the actor's linked reference (both
/// refined to the persistent reference when the form has flag 0x20), 1 the
/// player when the target object is the player's base form, otherwise the
/// first reference of that object near the actor (`fn_006784f0` chooses),
/// 2 the first nearby reference the callback accepts. If the answer is an
/// actor and the package is not "created": for a package of type 1 or 7 a
/// reference that is not the player is added as a follower of the actor
/// (`008bc790`), the player is added unless its process points at the
/// player or `00962720` holds or the player's count is over the setting
/// at `011cdad0`, in which case the actor's process slot 0x288 is told and
/// the message at `011d3d54` is shown; a type 2 package adds the actor as
/// a follower of a reference that is not the player.
///
/// open: for a target of type 2 the game passes the callback a context
/// whose first word is never written (an uninitialised stack word); it is
/// 0 here, and likewise for type 1 when the target object is null or does
/// not answer slot 0xe4.
pub fn fn_006780e0(e: &mut Engine, this: Ptr<TESPackage>, actor: Object, flag: u8) -> Object {
    if target_of(e, this).is_null() {
        return Ptr::NULL;
    }
    let target = target_of(e, this);
    let target_kind = e.call(TARGET_GET_TYPE, &args![target]).u32();
    let mut result: Object = Ptr::NULL;
    match target_kind {
        0 => {
            let target = target_of(e, this);
            result = e.call(TARGET_GET_REFERENCE, &args![target]).ptr();
            result = refine_reference(e, result);
        }
        3 => {
            result = linked_reference(e, actor);
            result = refine_reference(e, result);
        }
        1 => {
            let mut object: Object = Ptr::NULL;
            let target = target_of(e, this);
            if !e
                .call(TARGET_GET_OBJECT, &args![target])
                .ptr::<()>()
                .is_null()
            {
                let target = target_of(e, this);
                let held = e.call(TARGET_GET_OBJECT, &args![target]).ptr::<()>();
                if e.vcall(held.addr(), SLOT_FORM_E4, &args![]).bool() {
                    let target = target_of(e, this);
                    object = e.call(TARGET_GET_OBJECT, &args![target]).ptr();
                }
            }
            let player: u32 = e.global(PLAYER);
            if player != 0 && object.addr() == base_form_of(e, Ptr::new(player)).addr() {
                result = Ptr::new(player);
            } else {
                result = e.with_stack(12, |e, context| {
                    e.mem.set_u32(context.addr(), object.addr());
                    e.mem.set_u32(context.addr() + 4, actor.addr());
                    e.mem.set_u8(context.addr() + 8, flag);
                    find_close_reference(e, actor, context.addr())
                });
            }
        }
        2 => {
            let target = target_of(e, this);
            e.call(TARGET_GET_OBJECT_TYPE, &args![target]);
            result = e.with_stack(12, |e, context| {
                e.mem.set_u32(context.addr() + 4, actor.addr());
                e.mem.set_u8(context.addr() + 8, flag);
                find_close_reference(e, actor, context.addr())
            });
        }
        _ => {}
    }
    if !result.is_null()
        && e.vcall(result.addr(), SLOT_IS_ACTOR, &args![]).bool()
        && !tes_package_get_is_created(e, this)
    {
        let held = result;
        let player: u32 = e.global(PLAYER);
        if pack_type(e, this) == 1 || pack_type(e, this) == 7 {
            if held.addr() == player {
                let process = process_of(e, actor);
                let chosen = e.vcall(process.addr(), SLOT_PROCESS_52C, &args![]).u32();
                if chosen != player && !e.call(PLAYER_CHECK_00962720, &args![player, actor]).bool()
                {
                    let count = e.call(PLAYER_COUNT_00962620, &args![player]).i32();
                    let at = e
                        .call(SETTING_VALUE_ADDRESS, &args![FOLLOWER_LIMIT_SETTING])
                        .u32();
                    if count > e.mem.i32(at) {
                        let process = process_of(e, actor);
                        e.vcall(process.addr(), SLOT_PROCESS_288, &args![actor, -1i32]);
                        let message = e.call(SETTING_VALUE_OR_ZERO, &args![MESSAGE_SETTING]).u32();
                        let delay: f32 = e.global(MESSAGE_DELAY);
                        e.call(SHOW_MESSAGE, &args![message, 0u32, 0u32, 0u32, delay, 0u32]);
                        return result;
                    }
                }
            }
            e.call(ADD_FOLLOWER, &args![held, actor]);
        } else if pack_type(e, this) == 2 && held.addr() != player {
            e.call(ADD_FOLLOWER, &args![actor, held]);
        }
    }
    result
}

// Translated from 006784c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetEscortFollowDistance` (Xbox PDB): the follow distance of
/// an escort package (type 2: the word at +8 of its data object), else
/// 0x100.
pub fn tes_package_get_escort_follow_distance(e: &mut Engine, this: Ptr<TESPackage>) -> u32 {
    if pack_type(e, this) == TYPE_ESCORT {
        let data = e.get(this, TESPackage::pPackData);
        e.mem.u32(data.addr() + ESCORT_FOLLOW_DISTANCE)
    } else {
        0x100
    }
}

// Translated from 006784f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback of `fn_006780e0`'s reference search, `__cdecl(reference,
/// context)` with the context {form, actor, flag}: accepts `reference`
/// (stores it in the global at `011d6580`) when the form answers slot
/// 0xe4, `0055d3b0` accepts the reference for the form, the reference does
/// not have form flag 0x20, and, for an actor candidate, it is not the
/// searching actor, is not rejected by its slot 0x22c, and, with the flag
/// set, is not in a package of type 1 and not rejected by `008bc7d0`.
pub fn fn_006784f0(e: &mut Engine, reference: Object, context: Ptr) -> bool {
    if context.is_null() {
        return false;
    }
    let form = Ptr::<()>::new(e.mem.u32(context.addr()));
    let actor = Ptr::<()>::new(e.mem.u32(context.addr() + 4));
    let flag = e.mem.u8(context.addr() + 8);
    if !e.vcall(form.addr(), SLOT_FORM_E4, &args![]).bool() {
        return false;
    }
    if !e
        .call(REFERENCE_MATCHES_FORM, &args![reference, form])
        .bool()
    {
        return false;
    }
    if e.call(FORM_FLAG_20, &args![reference]).bool() {
        return false;
    }
    if e.vcall(reference.addr(), SLOT_IS_ACTOR, &args![]).bool() {
        if reference == actor {
            return false;
        }
        if e.vcall(reference.addr(), SLOT_REJECT_22C, &args![1u32])
            .bool()
        {
            return false;
        }
        if flag != 0 {
            let package = e
                .call(GET_CURRENT_PACKAGE, &args![reference])
                .ptr::<TESPackage>();
            if !package.is_null() && pack_type(e, package) == 1 {
                return false;
            }
            if e.call(ACTOR_REJECTS_CANDIDATE, &args![actor, reference])
                .bool()
            {
                return false;
            }
        }
    }
    e.set_global(FOUND_REFERENCE, reference.addr());
    true
}

// Translated from 00678610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::IsInterruptPackage` (Xbox PDB): whether the package type is
/// one of 0x12 to 0x19, 0x1b to 0x1d or 0x1f to 0x24.
pub fn tes_package_is_interrupt_package(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    matches!(pack_type(e, this), 0x12..=0x19 | 0x1b..=0x1d | 0x1f..=0x24)
}

// Translated from 00678670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetPackageRadiusActorToLocation` (Xbox PDB): the radius
/// within which the actor `actor` counts as at the location (the second
/// location when `second` is non-zero, else the own one), as a float
/// holding an integer. A location's own radius (`0067f1c0`) is used when
/// non-zero; otherwise the reference the location resolves to (`0067f2a0`)
/// gives 10 (furniture), 90 (an actor of kind 9), 20 (a marker form) or the
/// model bound plus 20, never negative; a location of type 3 without a
/// reference gives 10; zero becomes the default radius setting. A package
/// without that location gives 0.
pub fn tes_package_get_package_radius_actor_to_location(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
    second: u8,
) -> f32 {
    let location = if second != 0 {
        fn_00672dd0(e, this)
    } else {
        location_of(e, this)
    };
    let mut radius: i32 = 0;
    if !location.is_null() {
        e.call(LOCATION_GET_RADIUS, &args![location]);
        radius = e.call(LOCATION_GET_RADIUS, &args![location]).i32();
        if radius == 0 {
            let reference = e
                .call(LOCATION_RESOLVE_REFERENCE, &args![location, actor])
                .ptr::<()>();
            if !reference.is_null() {
                if is_furniture(e, reference) {
                    radius = 10;
                } else if e.vcall(reference.addr(), SLOT_IS_ACTOR, &args![]).bool() {
                    if e.vcall(reference.addr(), SLOT_KIND, &args![]).i32() == 9 {
                        radius = 0x5a;
                    } else {
                        radius = model_bound_size(e, reference).wrapping_add(0x14);
                    }
                } else if is_one_of_two_marker_forms(e, reference) {
                    radius = 0x14;
                } else {
                    radius = model_bound_size(e, reference).wrapping_add(0x14);
                }
                if radius < 0 {
                    radius = 0;
                }
            } else if location_type(e, location) == 3 {
                radius = 10;
            }
        }
        if radius == 0 {
            radius = default_radius_setting(e);
        }
    }
    radius as f32
}

// Translated from 006787e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetPackageRadiusActorToRefTarget` (Xbox PDB): the radius
/// within which the actor `actor` counts as at the reference it is heading
/// for (actor slot 0x2c8), as a float holding an integer. `second`
/// selects the second location, which is looked up and not used. 10 for a
/// furniture reference, 20 for a marker form; otherwise the target's
/// value (`0044ddc0`) for a type 0x1c package with a package extra that
/// has a target, or for a package whose procedure type is not 0x1a and
/// that has a target; when still 0 and the reference is not an actor, the
/// model bound, plus, for other form types than 0x15, 0x1b, 0x20 and 0x21
/// and an actor that is an actor, its reach height and half of the
/// reference's slot 0x1dc height when the eye line is above the
/// reference; zero becomes the default radius setting.
pub fn tes_package_get_package_radius_actor_to_ref_target(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
    second: u8,
) -> f32 {
    let mut radius: i32 = 0;
    if second != 0 {
        fn_00672dd0(e, this);
    } else {
        location_of(e, this);
    }
    let mut actor_as_actor = Ptr::NULL;
    if e.vcall(actor.addr(), SLOT_IS_ACTOR, &args![]).bool() {
        actor_as_actor = actor;
    }
    let target = actor_target_reference(e, actor);
    if !target.is_null() {
        if is_furniture(e, target) {
            radius = 10;
        } else if is_one_of_two_marker_forms(e, target) {
            radius = 0x14;
        } else {
            let list = e.call(GET_EXTRA_DATA_LIST, &args![actor_as_actor]).u32();
            let package_extra = e.call(EXTRA_GET_PACKAGE_EXTRA, &args![list]).u32();
            if pack_type(e, this) == TYPE_DIALOGUE_SECOND
                && package_extra != 0
                && !fn_00671d10(e, Ptr::new(package_extra)).is_null()
            {
                let held = fn_00671d10(e, Ptr::new(package_extra));
                radius = e.call(TARGET_GET_VALUE, &args![held]).i32();
            } else if !target_of(e, this).is_null()
                && e.get(this, TESPackage::ePROCEDURE_TYPE) != 0x1a
            {
                let held = target_of(e, this);
                radius = e.call(TARGET_GET_VALUE, &args![held]).i32();
            }
            if radius == 0 && !e.vcall(target.addr(), SLOT_IS_ACTOR, &args![]).bool() {
                radius = model_bound_size(e, target);
                let kind = base_form_type(e, target);
                if !matches!(kind, 0x15 | 0x1b | 0x20 | 0x21) && !actor_as_actor.is_null() {
                    radius = radius.wrapping_add(actor_reach_adjustment(
                        e,
                        actor,
                        actor_as_actor,
                        target,
                    ));
                }
            }
        }
    }
    if radius == 0 {
        radius = default_radius_setting(e);
    }
    radius as f32
}

/// The part of `GetPackageRadiusActorToRefTarget` that uses the actor's
/// reach height (slot 0x380) and eye level (`008be940`): the height as an
/// integer, and, when the eye line above the actor's position (position
/// z plus eye level minus the height) is over the target's z by less than
/// half the actor's slot 0x1dc height, half of that height as well.
fn actor_reach_adjustment(
    e: &mut Engine,
    actor: Object,
    actor_as_actor: Object,
    target: Object,
) -> i32 {
    let reach = e
        .vcall(actor_as_actor.addr(), SLOT_REACH_HEIGHT, &args![])
        .f32();
    let mut added = float_to_int(e, reach);
    let eye = e.call(ACTOR_EYE_LEVEL, &args![actor_as_actor]).f32() as f64;
    let at = e
        .vcall(actor_as_actor.addr(), SLOT_POSITION, &args![])
        .u32();
    let eye_height = ((e.mem.f32(at + 8) as f64 + eye) - reach as f64) as f32;
    let target_at = e.vcall(target.addr(), SLOT_POSITION, &args![]).u32();
    let target_height = e.mem.f32(target_at + 8);
    if target_height < eye_height {
        let difference = eye_height as f64 - target_height as f64;
        let half = e.global::<f64>(TWO);
        let extent = e.with_stack(12, |e, buffer| {
            let at = e.vcall(actor.addr(), SLOT_BOUNDS_1DC, &args![buffer]).u32();
            e.mem.f32(at + 8)
        });
        if extent as f64 / half > difference {
            let extent = e.with_stack(12, |e, buffer| {
                let at = e.vcall(actor.addr(), SLOT_BOUNDS_1DC, &args![buffer]).u32();
                e.mem.f32(at + 8)
            });
            let value = extent as f64 / half;
            added = added.wrapping_add(e.call(FTOL, &args![value]).i32());
        }
    }
    added
}

// Translated from 00678d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetShouldReserveTarget` (Xbox PDB): whether the procedure
/// type is 2 to 5 or 0x1a.
pub fn tes_package_get_should_reserve_target(e: &mut Engine, this: Ptr<TESPackage>) -> bool {
    let procedure = e.call(GET_PROCEDURE_TYPE, &args![this]).i32();
    (2..=5).contains(&procedure) || procedure == 0x1a
}

// Translated from 00678d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the day of the week (the game clock's, `00867ef0`: its value
/// modulo 7; `previous` non-zero takes the day before, wrapping 0 to 6)
/// fits the package schedule's day selector (the signed byte at +1 of the
/// schedule): 0 to 6 one day, 7 a weekday (not 0 or 6), 8 day 0 or 6,
/// 9 days 1, 3, 5, 10 days 2 and 4, anything else never.
pub fn fn_00678d40(e: &mut Engine, this: Ptr<TESPackage>, previous: u8) -> bool {
    let schedule = e.call(GET_SCHEDULE, &args![this]).u32();
    let mut day = e.call(GAME_DAY_OF_WEEK, &args![GAME_CLOCK]).u8() as i8;
    if previous != 0 {
        day = day.wrapping_sub(1);
        if day < 0 {
            day = 6;
        }
    }
    let selector = e.call(SCHEDULE_DAY_SELECTOR, &args![schedule]).u32();
    match selector {
        0..=6 => day as u32 == selector,
        7 => day != 0 && day != 6,
        8 => day == 0 || day == 6,
        9 => day == 1 || day == 3 || day == 5,
        10 => day == 2 || day == 4,
        _ => false,
    }
}

// Translated from 00678ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::IsTargetAnActor` (Xbox PDB): whether the package's target
/// stands for an actor: type 0 a reference that answers slot 0x100, type 1
/// an object whose form type is 0x2a or 0x2b, type 2 an object type of 0xe
/// or 0xf, type 3 the actor's linked reference when it answers slot 0x100
/// (of the actor `actor`).
pub fn tes_package_is_target_an_actor(
    e: &mut Engine,
    this: Ptr<TESPackage>,
    actor: Object,
) -> bool {
    if target_of(e, this).is_null() {
        return false;
    }
    let target = target_of(e, this);
    let mut result = false;
    match e.call(TARGET_GET_TYPE, &args![target]).u32() {
        0 => {
            if !e
                .call(TARGET_GET_REFERENCE, &args![target])
                .ptr::<()>()
                .is_null()
            {
                let held = e.call(TARGET_GET_REFERENCE, &args![target]).ptr::<()>();
                result = e.vcall(held.addr(), SLOT_IS_ACTOR, &args![]).bool();
            }
        }
        1 => {
            let object = e.call(TARGET_GET_OBJECT, &args![target]).ptr::<()>();
            let kind = e.call(GET_FORM_TYPE, &args![object]).u8();
            result = kind == 0x2a || kind == 0x2b;
        }
        2 => {
            let object_type = e.call(TARGET_GET_OBJECT_TYPE, &args![target]).i32();
            result = (0xe..=0xf).contains(&object_type);
        }
        3 if !linked_reference(e, actor).is_null() => {
            let held = linked_reference(e, actor);
            result = e.vcall(held.addr(), SLOT_IS_ACTOR, &args![]).bool();
        }
        _ => {}
    }
    result
}

/// The save/load game object.
fn save_load_game(e: &mut Engine) -> u32 {
    e.global::<u32>(SAVE_LOAD_GAME)
}

/// Whether the "log the save sizes" flag byte is set.
fn save_size_logging(e: &mut Engine) -> bool {
    let flag = e
        .call(FLAG_BYTE_ADDRESS, &args![SAVE_SIZE_LOG_OBJECT])
        .u32();
    e.mem.u8(flag) != 0
}

/// The form of the record the save/load object keeps for the form being
/// saved or loaded (`current`, whose first word is the form ID).
fn lookup_current(e: &mut Engine, current: u32) -> u32 {
    let form_id = e.mem.u32(current);
    e.call(LOOKUP_FORM, &args![form_id]).u32()
}

/// The `Error` message of the save size and save functions: with the form
/// being saved (`current`) its id, name and flags, otherwise only the size
/// and the source position.
fn log_save_size(e: &mut Engine, saved: u32, current: u32, line: u32, formats: (u32, u32)) {
    if current != 0 {
        let form = lookup_current(e, current);
        let form_id = e.mem.u32(current);
        // The 0x130 virtual returns the `%s` of the message.
        let name = e.vcall(form, SLOT_FORM_NAME, &args![]).u32();
        let flags = e.mem.u32(current + 5);
        e.call(
            ERROR_LOG,
            &args![
                formats.0,
                saved,
                form_id,
                name,
                flags,
                line,
                PACKAGE_SOURCE_FILE
            ],
        );
    } else {
        e.call(
            ERROR_LOG,
            &args![formats.1, saved, line, PACKAGE_SOURCE_FILE],
        );
    }
}

// Translated from 00678fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The package's save size (a 16-bit size; its trace message says
/// `GetSaveSize`): 6 more bytes when save game blocks are used, 13 for the
/// form data, the save size of the location and of the target when there
/// are some, and 4 for the trailing word. With the size logging flag set
/// the size is reported through `Error`.
pub fn fn_00678fd0(e: &mut Engine, this: Ptr<TESPackage>) -> u16 {
    let save = save_load_game(e);
    let mut size: u16 = 0;
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    size = size.wrapping_add(0xc);
    size = size.wrapping_add(1);
    let location = e.get(this, TESPackage::pPackLoc);
    if !location.is_null() {
        size = size.wrapping_add(e.call(LOCATION_GET_SAVE_SIZE, &args![location]).u16());
    }
    let target = e.get(this, TESPackage::pPackTarg);
    if !target.is_null() {
        size = size.wrapping_add(e.call(TARGET_GET_SAVE_SIZE, &args![target]).u16());
    }
    size = size.wrapping_add(4);
    if save_size_logging(e) {
        let current = e.call(CURRENT_SAVE_FORM, &args![save]).u32();
        // "GetSaveSize(): %-5i for form %08X %s with flags %08X ending at
        // line %i in file %s", or without the form part.
        log_save_size(e, size as u32, current, 0x1d51, (0x0101_2cb0, 0x0101_2c78));
    }
    size
}

// Translated from 00679120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The package's old save (no buffer argument; its trace message says
/// `SaveGame`): with save game blocks the `BLOK` tag and a 16-bit length
/// (patched at the end), the 12 bytes at +0x1c, a byte of which parts
/// exist (1 location, 2 target) through `TESForm::SaveGameDataOLD`, the
/// location and target saves and the word at +0x18. With the logging flag
/// set the bytes written are reported through `Error`; a block over 0xFFFF
/// bytes is reported through `005b5e40`.
pub fn fn_00679120(e: &mut Engine, this: Ptr<TESPackage>) {
    let save = save_load_game(e);
    let mut start = e.call(SAVE_POSITION, &args![save]).u32();
    if save_size_logging(e) {
        start = e.call(SAVE_POSITION, &args![save]).u32();
    }
    // Locals of the game's frame passed by address: the tag (+0), the
    // 16-bit block length (+4) and the part flags (+8).
    e.with_stack(0x10, |e, locals| {
        let (tag, length_slot, parts) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        let mut block = 0u32;
        e.mem.set_u16(length_slot, 0);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
            e.mem.set_u32(tag, BLOCK_TAG);
            e.call(SAVE_BYTES, &args![save, tag, 4u32]);
            block = e.call(SAVE_POSITION, &args![save]).u32();
            e.call(SAVE_BYTES, &args![save, length_slot, 2u32]);
        }
        e.call(SAVE_BYTES, &args![save, this.addr() + 0x1c, 0xcu32]);
        let mut flags = 0u8;
        if !e.get(this, TESPackage::pPackLoc).is_null() {
            flags |= 1;
        }
        if !e.get(this, TESPackage::pPackTarg).is_null() {
            flags |= 2;
        }
        e.mem.set_u8(parts, flags);
        e.call(FORM_SAVE_DATA_OLD, &args![this, parts, 1u32]);
        let location = e.get(this, TESPackage::pPackLoc);
        if !location.is_null() {
            e.call(LOCATION_SAVE_OLD, &args![location]);
        }
        let target = e.get(this, TESPackage::pPackTarg);
        if !target.is_null() {
            e.call(TARGET_SAVE_OLD, &args![target]);
        }
        e.call(SAVE_BYTES, &args![save, this.addr() + 0x18, 4u32]);
        if save_size_logging(e) {
            let end = e.call(SAVE_POSITION, &args![save]).u32();
            let current = e.call(CURRENT_SAVE_FORM, &args![save]).u32();
            // "SaveGame(): %-5i for form %08X %s with flags %08X ending at
            // line %i in file %s", or without the form part.
            log_save_size(
                e,
                end.wrapping_sub(start),
                current,
                0x1d6e,
                (0x0101_53a0, 0x0101_536c),
            );
        }
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
            let end = e.call(SAVE_POSITION, &args![save]).u32();
            if end > block.wrapping_add(0xffff) {
                e.call(
                    SAVE_LOAD_LOG,
                    &args![0x0101_5318u32, PACKAGE_SOURCE_FILE, 0x1d6eu32],
                );
            }
            e.mem.set_u16(block, end.wrapping_sub(block) as u16);
        }
    });
}

/// A `LoadGame` message of `005b5e40`: with the form being loaded (`form`,
/// looked up from the record `current`) the arguments are the format, the
/// byte count if the message has one, the source file and line, the form
/// ID, its name, the version byte (+9) and the flags (+5) of the record;
/// without one, the format, the count, the file, the line and the version
/// of the save/load object.
fn log_load_message(
    e: &mut Engine,
    (save, current, form): (u32, u32, u32),
    (format_with_form, format_without): (u32, u32),
    amount: Option<u32>,
    line: u32,
) {
    let mut words: Vec<u32> = vec![];
    if current != 0 {
        let form_id = e.mem.u32(current);
        let name = e.vcall(form, SLOT_FORM_NAME, &args![]).u32();
        let version = e.mem.u8(current + 9) as u32;
        let flags = e.mem.u32(current + 5);
        words.push(format_with_form);
        words.extend(amount);
        words.extend([PACKAGE_SOURCE_FILE, line, form_id, name, version, flags]);
    } else {
        let version = e.call(LOAD_VERSION, &args![save]).u8() as u32;
        words.push(format_without);
        words.extend(amount);
        words.extend([PACKAGE_SOURCE_FILE, line, version]);
    }
    e.call(SAVE_LOAD_LOG, &words);
}

// Translated from 00679340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::LoadGame` (Xbox PDB), the old form: reads what
/// `fn_00679120` wrote. With save game blocks the `BLOK` tag is checked (a
/// wrong tag is logged through `005b5e40`) and the 16-bit length read; then
/// the 12 bytes at +0x1c, the part flags (`TESForm::LoadGameDataOLD`), a
/// new location and a new target when flagged (each loaded with its old
/// `LoadGame`) and the word at +0x18. The block length is checked against
/// the bytes read (overrun and underrun are logged). The C++ exception
/// frame is not translated.
pub fn tes_package_load_game(e: &mut Engine, this: Ptr<TESPackage>) {
    let save = save_load_game(e);
    // Locals: the tag (+0), the 16-bit length (+4), the part flags (+8).
    e.with_stack(0x10, |e, locals| {
        let (tag, length_slot, parts) = (locals.addr(), locals.addr() + 4, locals.addr() + 8);
        let mut block = 0u32;
        e.mem.set_u16(length_slot, 0);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
            e.call(LOAD_BYTES, &args![save, tag, 4u32]);
            if e.mem.u32(tag) != BLOCK_TAG {
                let current = e.call(CURRENT_LOAD_FORM, &args![save]).u32();
                let form = if current != 0 {
                    lookup_current(e, current)
                } else {
                    0
                };
                // "SAVELOAD: (LoadGame Buffer error) Block Header is
                // incorrect in file %s on line %i. ..."
                log_load_message(
                    e,
                    (save, current, form),
                    (0x0101_5718, 0x0101_56a8),
                    None,
                    0x1d74,
                );
            }
            block = e.call(SAVE_POSITION, &args![save]).u32();
            e.call(LOAD_BYTES, &args![save, length_slot, 2u32]);
        }
        e.call(LOAD_BYTES, &args![save, this.addr() + 0x1c, 0xcu32]);
        e.call(FORM_LOAD_DATA_OLD, &args![this, parts, 1u32]);
        let flags = e.mem.u8(parts);
        if flags & 1 != 0 {
            let made = new_location(e);
            e.set(this, TESPackage::pPackLoc, made);
            e.call(LOCATION_LOAD_OLD, &args![made]);
        }
        if flags & 2 != 0 {
            let made = new_target(e);
            e.set(this, TESPackage::pPackTarg, made);
            e.call(TARGET_LOAD_OLD, &args![made]);
        }
        e.call(LOAD_BYTES, &args![save, this.addr() + 0x18, 4u32]);
        if e.call(USE_SAVE_GAME_BLOCKS, &args![save]).bool() {
            let position = e.call(SAVE_POSITION, &args![save]).u32();
            let current = e.call(CURRENT_LOAD_FORM, &args![save]).u32();
            let form = if current != 0 {
                lookup_current(e, current)
            } else {
                0
            };
            let expected = (e.mem.u16(length_slot) as u32).wrapping_add(block);
            let context = (save, current, form);
            // "SAVELOAD: LoadGame Buffer overrun of %i bytes in file %s on
            // line %i. ..." and the underrun message.
            if position > expected {
                let amount = Some(position.wrapping_sub(expected));
                log_load_message(e, context, (0x0101_5588, 0x0101_54a0), amount, 0x1d8b);
            } else if position < expected {
                let amount = Some(expected.wrapping_sub(position));
                log_load_message(e, context, (0x0101_5500, 0x0101_5440), amount, 0x1d8b);
            }
        }
    });
}

// Translated from 006796c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the unnamed post-load step of the location and of the target
/// (`0067fb20`, `006806e0`) when the package has them.
pub fn fn_006796c0(e: &mut Engine, this: Ptr<TESPackage>) {
    let location = e.get(this, TESPackage::pPackLoc);
    if !location.is_null() {
        e.call(LOCATION_AFTER_LOAD, &args![location]);
    }
    let target = e.get(this, TESPackage::pPackTarg);
    if !target.is_null() {
        e.call(TARGET_AFTER_LOAD, &args![target]);
    }
}

// Translated from 00679700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::LoadGame(flags, extra)`, then the form flag 0x80000000 sets
/// bit 0x8000 of `iPackFlags` and the form flag 0x40000000 sets bit
/// 0x10000.
pub fn fn_00679700(e: &mut Engine, this: Ptr<TESPackage>, flags: u32, extra: u32) {
    e.call(FORM_LOAD_GAME, &args![this, flags, extra]);
    if flags & FORM_FLAG_HIGH != 0 {
        let pack = e.get(this, TESPackage::iPackFlags);
        e.set(this, TESPackage::iPackFlags, pack | PACK_FLAG_FOR_HIGH);
    }
    if flags & FORM_FLAG_NEXT != 0 {
        let pack = e.get(this, TESPackage::iPackFlags);
        e.set(this, TESPackage::iPackFlags, pack | PACK_FLAG_FOR_NEXT);
    }
}

// Translated from 00679760 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reverse of `fn_00679700`: `004534f0(flags)` on the form, then the
/// form flag 0x80000000 clears bit 0x8000 of `iPackFlags` and the form flag
/// 0x40000000 clears bit 0x10000.
pub fn fn_00679760(e: &mut Engine, this: Ptr<TESPackage>, flags: u32) {
    e.call(FORM_CLEAR_FLAGS, &args![this, flags]);
    if flags & FORM_FLAG_HIGH != 0 {
        let pack = e.get(this, TESPackage::iPackFlags);
        e.set(this, TESPackage::iPackFlags, pack & !PACK_FLAG_FOR_HIGH);
    }
    if flags & FORM_FLAG_NEXT != 0 {
        let pack = e.get(this, TESPackage::iPackFlags);
        e.set(this, TESPackage::iPackFlags, pack & !PACK_FLAG_FOR_NEXT);
    }
}

// Translated from 006797c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::SaveGame` (Xbox PDB): only for a package with a dynamic
/// form ID. Writes the 12 bytes at +0x1c, a byte of which parts exist (1
/// location, 2 target, 4 data object), each part's `SaveGame(buffer)` (the
/// data object's through slot 0x14) and the word at +0x18, all into the
/// save buffer `buffer`.
pub fn tes_package_save_game(e: &mut Engine, this: Ptr<TESPackage>, buffer: u32) {
    if !has_dynamic_form_id(e, this) {
        return;
    }
    e.call(
        BUFFER_SAVE_DATA,
        &args![buffer, this.addr() + 0x1c, 0xcu32, 0u32],
    );
    let mut flags = 0u8;
    if !e.get(this, TESPackage::pPackLoc).is_null() {
        flags |= 1;
    }
    if !e.get(this, TESPackage::pPackTarg).is_null() {
        flags |= 2;
    }
    if !e.get(this, TESPackage::pPackData).is_null() {
        flags |= 4;
    }
    e.with_stack(4, |e, parts| {
        e.mem.set_u8(parts.addr(), flags);
        e.call(BUFFER_SAVE_DATA, &args![buffer, parts, 1u32, 0u32]);
    });
    let location = e.get(this, TESPackage::pPackLoc);
    if !location.is_null() {
        e.call(LOCATION_SAVE_BUFFER, &args![location, buffer]);
    }
    let target = e.get(this, TESPackage::pPackTarg);
    if !target.is_null() {
        e.call(TARGET_SAVE_BUFFER, &args![target, buffer]);
    }
    let data = e.get(this, TESPackage::pPackData);
    if !data.is_null() {
        e.vcall(data.addr(), SLOT_DATA_SAVE, &args![buffer]);
    }
    e.call(
        BUFFER_SAVE_DATA,
        &args![buffer, this.addr() + 0x18, 4u32, 0u32],
    );
}

// Translated from 006798c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::LoadGame` (Xbox PDB, second overload), reading from the
/// load buffer `buffer`. For a package with a dynamic form ID: the 12
/// bytes at +0x1c, the part flags, a new location and a new target when
/// flagged (each loaded with `LoadGame(buffer)`), the data object's
/// `LoadGame` through slot 0x18 (the object must exist already) and the
/// word at +0x18. For another form ID only the two form flag tests on the
/// buffer (`00428110` / `004280f0`) run, which set bit 0x8000 and 0x10000
/// of `iPackFlags`. The C++ exception frame is not translated.
pub fn tes_package_load_game_ov2(e: &mut Engine, this: Ptr<TESPackage>, buffer: u32) {
    if !has_dynamic_form_id(e, this) {
        for (mask, bit) in [
            (FORM_FLAG_HIGH, PACK_FLAG_FOR_HIGH),
            (FORM_FLAG_NEXT, PACK_FLAG_FOR_NEXT),
        ] {
            let set = e.with_stack(4, |e, slot| {
                // 0067990f: `00428110` takes one word (RET 4); the pushed mask is the
                // word of `004280f0` (RET 4).
                let object = e.call(BUFFER_FLAG_OBJECT_A, &args![buffer, slot]).u32();
                e.call(BUFFER_FLAG_TEST, &args![object, mask]).bool()
            });
            if set {
                let pack = e.get(this, TESPackage::iPackFlags);
                e.set(this, TESPackage::iPackFlags, pack | bit);
            }
        }
        return;
    }
    e.call(BUFFER_LOAD_DATA, &args![buffer, this.addr() + 0x1c, 0xcu32]);
    let flags = e.with_stack(4, |e, parts| {
        e.mem.set_u8(parts.addr(), 0);
        e.call(BUFFER_LOAD_DATA, &args![buffer, parts, 1u32]);
        e.mem.u8(parts.addr())
    });
    if flags & 1 != 0 {
        let made = new_location(e);
        e.set(this, TESPackage::pPackLoc, made);
        e.call(LOCATION_LOAD_BUFFER, &args![made, buffer]);
    }
    if flags & 2 != 0 {
        let made = new_target(e);
        e.set(this, TESPackage::pPackTarg, made);
        e.call(TARGET_LOAD_BUFFER, &args![made, buffer]);
    }
    if flags & 4 != 0 {
        let data = e.get(this, TESPackage::pPackData);
        e.vcall(data.addr(), SLOT_DATA_LOAD, &args![buffer]);
    }
    e.call(BUFFER_LOAD_DATA, &args![buffer, this.addr() + 0x18, 4u32]);
}

// Translated from 00679a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::InitLoadGame` (Xbox PDB): for a package with a dynamic form
/// ID, `InitLoadGame(buffer)` of the location, the target and the data
/// object (slot 0x1c) when the package has them.
pub fn tes_package_init_load_game(e: &mut Engine, this: Ptr<TESPackage>, buffer: u32) {
    if !has_dynamic_form_id(e, this) {
        return;
    }
    let location = e.get(this, TESPackage::pPackLoc);
    if !location.is_null() {
        e.call(LOCATION_INIT_LOAD_BUFFER, &args![location, buffer]);
    }
    let target = e.get(this, TESPackage::pPackTarg);
    if !target.is_null() {
        e.call(TARGET_INIT_LOAD_BUFFER, &args![target, buffer]);
    }
    let data = e.get(this, TESPackage::pPackData);
    if !data.is_null() {
        e.vcall(data.addr(), SLOT_DATA_INIT_LOAD, &args![buffer]);
    }
}

// Translated from 00679b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// For a package whose form ID is not a dynamic one: clears bit 0x8000 of
/// `iPackFlags` when the buffer's flag test `0042ce30(tmp, 0x80000000)` /
/// `004280f0` holds, and bit 0x10000 for the test with `0x40000000`. The
/// counterpart of the second branch of `tes_package_load_game_ov2`.
pub fn fn_00679b10(e: &mut Engine, this: Ptr<TESPackage>, buffer: u32) {
    if has_dynamic_form_id(e, this) {
        return;
    }
    for (mask, bit) in [
        (FORM_FLAG_HIGH, PACK_FLAG_FOR_HIGH),
        (FORM_FLAG_NEXT, PACK_FLAG_FOR_NEXT),
    ] {
        let set = e.with_stack(4, |e, slot| {
            // 00679b40: `0042ce30` takes one word (RET 4); the pushed mask is the word
            // of `004280f0` (RET 4).
            let object = e.call(BUFFER_FLAG_OBJECT_B, &args![buffer, slot]).u32();
            e.call(BUFFER_FLAG_TEST, &args![object, mask]).bool()
        });
        if set {
            let pack = e.get(this, TESPackage::iPackFlags);
            e.set(this, TESPackage::iPackFlags, pack & !bit);
        }
    }
}

// Translated from 00679ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::GetObjectTypeFromForm` (Xbox PDB): the package object type
/// (0 for none) of a form, from its form type: 0x14 gives 0x18, 0x15 gives
/// 1, 0x18 to 0x1c give 2 to 6, 0x1d gives 7 (0x12 when the `+0x3c`
/// member's slot 4 answers true), 0x1e gives 8, 0x1f, 0x32, 0x67, 0x6c,
/// 0x73 and 0x74 give 9, 0x26 and 0x27 give 10 and 11, 0x28 gives 0xc (0x16
/// when `006450c0` holds, else 0x17 when `004c0c30` holds), 0x29 to 0x2b
/// give 0xd to 0xf, 0x2e gives 0x10 and 0x2f gives 0x11 (0x12 when the
/// `+0x3c` member answers true).
pub fn tes_package_get_object_type_from_form(e: &mut Engine, form: Object) -> i32 {
    if form.is_null() {
        return 0;
    }
    let member_flag = |e: &mut Engine| {
        e.vcall(form.addr() + FORM_MEMBER_3C, SLOT_MEMBER_FLAG, &args![])
            .bool()
    };
    match e.call(GET_FORM_TYPE, &args![form]).u32() {
        0x14 => 0x18,
        0x15 => 1,
        0x18 => 2,
        0x19 => 3,
        0x1a => 4,
        0x1b => 5,
        0x1c => 6,
        0x1d => {
            if member_flag(e) {
                0x12
            } else {
                7
            }
        }
        0x1e => 8,
        0x1f | 0x32 | 0x67 | 0x6c | 0x73 | 0x74 => 9,
        0x26 => 10,
        0x27 => 0xb,
        0x28 => {
            if e.call(FORM_TEST_SIXTEEN, &args![form]).bool() {
                0x16
            } else if e.call(FORM_TEST_SEVENTEEN, &args![form]).bool() {
                0x17
            } else {
                0xc
            }
        }
        0x29 => 0xd,
        0x2a => 0xe,
        0x2b => 0xf,
        0x2e => 0x10,
        0x2f => {
            if member_flag(e) {
                0x12
            } else {
                0x11
            }
        }
        _ => 0,
    }
}

// Translated from 00679e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESPackage::FormMatchesPackageObjectType` (Xbox PDB): whether a form
/// satisfies a package object type. Both must be non-null/non-zero. By the
/// form type: 0x14 matches 0x18, and 0x19, 0x1a, 0x1b when the effect item
/// list at `+0x24` has the target effect, the touch effect or answers the
/// third test (`00406090`); 0x15, 0x19, 0x1b, 0x1c, 0x1e, 0x26, 0x27 and
/// 0x2e match 1, 3, 5, 6, 8, 10, 11 and 0x10; 0x18 and 0x29 match 2 and
/// 0xd and 0x13, 0x14; 0x1a matches 4 and 0x14; 0x1d matches 7, and 0x12
/// when the `+0x3c` member answers true; 0x1f, 0x32, 0x67, 0x6c, 0x73 and
/// 0x74 match 9; 0x28 matches 0xc, 0x13, 0x14, and 0x16 / 0x17 when
/// `006450c0` / `004c0c30` hold; 0x2a and 0x2b match 0xe / 0xf and 0x1c;
/// 0x2f matches 0x11, and 0x12 when the `+0x3c` member answers true.
pub fn tes_package_form_matches_package_object_type(
    e: &mut Engine,
    form: Object,
    object_type: i32,
) -> bool {
    if form.is_null() || object_type == 0 {
        return false;
    }
    let member_flag = |e: &mut Engine| {
        e.vcall(form.addr() + FORM_MEMBER_3C, SLOT_MEMBER_FLAG, &args![])
            .bool()
    };
    let wide_range = (0x13..=0x14).contains(&object_type);
    match e.call(GET_FORM_TYPE, &args![form]).u32() {
        0x14 => {
            if object_type == 0x18 {
                true
            } else if (0x19..=0x1b).contains(&object_type) {
                let list = form.addr() + FORM_MEMBER_24;
                match object_type {
                    0x19 => e.call(EFFECT_LIST_TEST_TARGET, &args![list]).bool(),
                    0x1a => e.call(EFFECT_LIST_TEST_TOUCH, &args![list]).bool(),
                    _ => e.call(EFFECT_LIST_TEST_THIRD, &args![list]).bool(),
                }
            } else {
                false
            }
        }
        0x15 => object_type == 1,
        0x18 => object_type == 2 || wide_range,
        0x19 => object_type == 3,
        0x1a => object_type == 4 || object_type == 0x14,
        0x1b => object_type == 5,
        0x1c => object_type == 6,
        0x1d => object_type == 7 || (object_type == 0x12 && member_flag(e)),
        0x1e => object_type == 8,
        0x1f | 0x32 | 0x67 | 0x6c | 0x73 | 0x74 => object_type == 9,
        0x26 => object_type == 10,
        0x27 => object_type == 0xb,
        0x28 => match object_type {
            0xc | 0x13 | 0x14 => true,
            0x16 | 0x17 => {
                (e.call(FORM_TEST_SIXTEEN, &args![form]).bool() && object_type == 0x16)
                    || (e.call(FORM_TEST_SEVENTEEN, &args![form]).bool() && object_type == 0x17)
            }
            _ => false,
        },
        0x29 => object_type == 0xd || wide_range,
        0x2a => object_type == 0xe || object_type == 0x1c,
        0x2b => object_type == 0xf || object_type == 0x1c,
        0x2e => object_type == 0x10,
        0x2f => object_type == 0x11 || (object_type == 0x12 && member_flag(e)),
        _ => false,
    }
}

// Translated from 0067acf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a package-data object: the base constructor
/// (`0067ad20`), then the vtable of the object (`01068860`) and a zero in
/// the word at +4. Returns `this`.
pub fn fn_0067acf0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(DATA_BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), DATA_BASE_VTABLE);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006707c0,
            tes_package_tes_package(Ptr<TESPackage>) -> Ptr<TESPackage>
        ),
        entry!(0x006709d0, fn_006709d0(Ptr<TESPackage>)),
        entry!(0x00670b90, tes_package_create_package(i32) -> Ptr<TESPackage>),
        entry!(0x00670ed0, fn_00670ed0(Ptr<TESPackage>) -> bool),
        entry!(0x00670ef0, fn_00670ef0(Ptr<TESPackage>) -> bool),
        entry!(0x00670f10, fn_00670f10(Ptr<TESPackage>) -> bool),
        entry!(0x00670f40, fn_00670f40(Ptr<TESPackage>) -> bool),
        entry!(0x00670f60, fn_00670f60(Ptr<TESPackage>) -> bool),
        entry!(
            0x00670f90,
            tes_package_get_once_per_day(Ptr<TESPackage>) -> bool
        ),
        entry!(0x00670fc0, tes_package_set_pack_type(Ptr<TESPackage>, i32)),
        entry!(0x00671a20, fn_00671a20(Ptr<TESPackage>, u8)),
        entry!(0x00671a70, fn_00671a70(Ptr<TESPackage>, u8)),
        entry!(0x00671ad0, fn_00671ad0(Ptr<TESPackage>, u8)),
        entry!(0x00671b30, fn_00671b30(Ptr<TESPackage>, u8)),
        entry!(0x00671b90, fn_00671b90(Ptr<TESPackage>, u8)),
        entry!(0x00671bf0, fn_00671bf0(Ptr<TESPackage>, u8)),
        entry!(0x00671c50, fn_00671c50(Ptr<TESPackage>, u8)),
        entry!(0x00671cb0, fn_00671cb0(Ptr<TESPackage>, u8)),
        entry!(0x00671d10, fn_00671d10(Ptr<TESPackage>) -> Ptr),
        entry!(
            0x00671d30,
            tes_package_set_package_location(Ptr<TESPackage>, Ptr)
        ),
        entry!(
            0x00671e10,
            tes_package_set_package_second_location(Ptr<TESPackage>, Ptr)
        ),
        entry!(
            0x00672710,
            tes_package_get_dialogue_say_to_flag(Ptr<TESPackage>) -> u8
        ),
        entry!(
            0x00672760,
            tes_package_get_dialogue_topic(Ptr<TESPackage>) -> Ptr
        ),
        entry!(
            0x006727b0,
            tes_package_get_dialogue_do_not_control_target(Ptr<TESPackage>) -> u8
        ),
        entry!(
            0x00672800,
            tes_package_get_dialogue_no_headtrack(Ptr<TESPackage>) -> u8
        ),
        entry!(0x00672850, fn_00672850(Ptr<TESPackage>) -> f32),
        entry!(
            0x006728a0,
            tes_package_get_acquire_radius(Ptr<TESPackage>, u32, Ptr) -> bool
        ),
        entry!(
            0x00672930,
            tes_package_get_initial_target_count(Ptr<TESPackage>) -> i32
        ),
        entry!(0x006729d0, fn_006729d0(Ptr<TESPackage>, u32)),
        entry!(0x00672aa0, fn_00672aa0(Ptr<TESPackage>, u8)),
        entry!(0x00672b70, fn_00672b70(Ptr<TESPackage>, u8)),
        entry!(0x00672c40, fn_00672c40(Ptr<TESPackage>, f32)),
        entry!(
            0x00672d10,
            tes_package_get_or_create_use_weapon_package_data(Ptr<TESPackage>) -> Ptr
        ),
        entry!(0x00672dd0, fn_00672dd0(Ptr<TESPackage>) -> Ptr),
        entry!(
            0x00672f20,
            tes_package_get_package_search_location(Ptr<TESPackage>) -> Ptr
        ),
        entry!(
            0x00672fc0,
            tes_package_set_package_target(Ptr<TESPackage>, Ptr)
        ),
        entry!(0x00674930, tes_package_init_item(Ptr<TESPackage>)),
        entry!(0x00674d20, fn_00674d20(Ptr<TESPackage>) -> bool),
        entry!(
            0x00674d40,
            tes_package_get_is_created(Ptr<TESPackage>) -> bool
        ),
        entry!(0x00674d70, tes_package_set_is_created(Ptr<TESPackage>, u8)),
        entry!(
            0x00674dd0,
            tes_package_is_script_package(Ptr<TESPackage>) -> bool
        ),
        entry!(0x00674e00, fn_00674e00(Ptr<TESPackage>, u8)),
        entry!(
            0x00674e40,
            tes_package_is_never_to_run(Ptr<TESPackage>) -> bool
        ),
        entry!(
            0x00674e70,
            tes_package_set_never_run(Ptr<TESPackage>, Ptr, u8)
        ),
        entry!(0x00674f30, fn_00674f30(Ptr<TESPackage>, u8)),
        entry!(
            0x00674fd0,
            tes_package_get_second_location_world(Ptr<TESPackage>, Ptr) -> Ptr
        ),
        entry!(
            0x006751a0,
            tes_package_get_second_location_cell(Ptr<TESPackage>, Ptr) -> Ptr
        ),
        entry!(
            0x00675360,
            tes_package_get_second_location_coord(Ptr<TESPackage>, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00675670, fn_00675670(Ptr<TESPackage>) -> u32),
        entry!(0x006756a0, fn_006756a0(Ptr<TESPackage>, Ptr) -> Ptr),
        entry!(
            0x006757e0,
            tes_package_get_search_location_cell(Ptr<TESPackage>, Ptr) -> Ptr
        ),
        entry!(
            0x00675830,
            tes_package_get_search_location_coord(Ptr<TESPackage>, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x006758a0,
            tes_package_get_search_location_radius(Ptr<TESPackage>, Ptr) -> i32
        ),
        entry!(
            0x006758f0,
            tes_package_get_use_weapon_package_data(Ptr<TESPackage>) -> Ptr
        ),
        entry!(
            0x00675920,
            tes_package_get_patrol_package_data(Ptr<TESPackage>) -> Ptr
        ),
        entry!(
            0x00675950,
            tes_package_get_follow_package_data(Ptr<TESPackage>) -> Ptr
        ),
        entry!(0x00675980, fn_00675980(Ptr<TESPackage>) -> Ptr),
        entry!(
            0x006759b0,
            tes_package_get_escort_package_data(Ptr<TESPackage>) -> Ptr
        ),
        entry!(
            0x006759e0,
            tes_package_compute_initial_target(Ptr<TESPackage>, Ptr) -> Ptr
        ),
        entry!(
            0x00675a50,
            tes_package_get_location_world(Ptr<TESPackage>, Ptr) -> Ptr
        ),
        entry!(
            0x00675c20,
            tes_package_get_location_cell(Ptr<TESPackage>, Ptr) -> Ptr
        ),
        entry!(
            0x00675de0,
            tes_package_get_location_coord(Ptr<TESPackage>, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00676140,
            tes_package_get_location_reference(Ptr<TESPackage>, Ptr) -> Ptr
        ),
        entry!(0x00676280, fn_00676280(Ptr<TESPackage>, Ptr) -> i32),
        entry!(
            0x00676390,
            fn_00676390(Ptr<TESPackage>, Ptr, Ptr, u8, f32, u8) -> bool
        ),
        entry!(
            0x006768d0,
            fn_006768d0(Ptr<TESPackage>, Ptr, u8, f32, u8) -> bool
        ),
        entry!(
            0x00676e40,
            tes_package_is_actor_at_ref_target(Ptr<TESPackage>, Ptr, i32) -> bool
        ),
        entry!(
            0x006773a0,
            tes_package_is_target_at_location(Ptr<TESPackage>, Ptr, i32) -> bool
        ),
        entry!(
            0x00677570,
            tes_package_is_target_at_second_location(Ptr<TESPackage>, Ptr, f32) -> bool
        ),
        entry!(0x00677760, fn_00677760(Ptr<TESPackage>, Ptr) -> bool),
        entry!(
            0x006777b0,
            tes_package_calculate_procedure_type(Ptr<TESPackage>, Ptr)
        ),
        entry!(0x006780e0, fn_006780e0(Ptr<TESPackage>, Ptr, u8) -> Ptr),
        entry!(
            0x006784c0,
            tes_package_get_escort_follow_distance(Ptr<TESPackage>) -> u32
        ),
        entry!(0x006784f0, fn_006784f0(Ptr, Ptr) -> bool),
        entry!(
            0x00678610,
            tes_package_is_interrupt_package(Ptr<TESPackage>) -> bool
        ),
        entry!(
            0x00678670,
            tes_package_get_package_radius_actor_to_location(Ptr<TESPackage>, Ptr, u8) -> f32
        ),
        entry!(
            0x006787e0,
            tes_package_get_package_radius_actor_to_ref_target(Ptr<TESPackage>, Ptr, u8) -> f32
        ),
        entry!(
            0x00678d00,
            tes_package_get_should_reserve_target(Ptr<TESPackage>) -> bool
        ),
        entry!(0x00678d40, fn_00678d40(Ptr<TESPackage>, u8) -> bool),
        entry!(
            0x00678ec0,
            tes_package_is_target_an_actor(Ptr<TESPackage>, Ptr) -> bool
        ),
        entry!(0x00678fd0, fn_00678fd0(Ptr<TESPackage>) -> u16),
        entry!(0x00679120, fn_00679120(Ptr<TESPackage>)),
        entry!(0x00679340, tes_package_load_game(Ptr<TESPackage>)),
        entry!(0x006796c0, fn_006796c0(Ptr<TESPackage>)),
        entry!(0x00679700, fn_00679700(Ptr<TESPackage>, u32, u32)),
        entry!(0x00679760, fn_00679760(Ptr<TESPackage>, u32)),
        entry!(0x006797c0, tes_package_save_game(Ptr<TESPackage>, u32)),
        entry!(0x006798c0, tes_package_load_game_ov2(Ptr<TESPackage>, u32)),
        entry!(0x00679a90, tes_package_init_load_game(Ptr<TESPackage>, u32)),
        entry!(0x00679b10, fn_00679b10(Ptr<TESPackage>, u32)),
        entry!(
            0x00679ba0,
            tes_package_get_object_type_from_form(Ptr) -> i32
        ),
        entry!(
            0x00679e00,
            tes_package_form_matches_package_object_type(Ptr, i32) -> bool
        ),
        entry!(0x0067acf0, fn_0067acf0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    /// Vtable of the fake package-data objects: slot 0 (`FAKE_DELETE`) and
    /// slot 0x10 (`FAKE_INIT_ITEM`).
    pub(super) const DATA_VTABLE: u32 = 0x0130_0000;
    const FAKE_DELETE: u32 = 0x7000_0000;
    const FAKE_INIT_ITEM: u32 = 0x7000_0010;
    /// Vtable of the test package: slot 0x130 gives the form name.
    const PACKAGE_VTABLE: u32 = 0x0130_1000;
    const FAKE_NAME: u32 = 0x7000_0130;
    pub(super) const NAME_POINTER: u32 = 0x0aa0_0001;

    /// Constructors and the small data objects' constructors: they return
    /// their `this`.
    const CONSTRUCTORS: [u32; 19] = [
        LOCATION_CONSTRUCTOR,
        TARGET_CONSTRUCTOR,
        DIALOGUE_DATA_CONSTRUCTOR,
        USE_WEAPON_DATA_CONSTRUCTOR,
        0x0067_bfd0,
        0x0067_bbf0,
        0x0067_b950,
        DATA_BASE_CONSTRUCTOR,
        0x0067_c770,
        0x0067_c3f0,
        0x009e_dbc0,
        0x0097_d3a0,
        0x009e_c5d0,
        0x009f_0e00,
        0x009f_91b0,
        0x009f_72f0,
        0x009e_d030,
        TES_FORM_CONSTRUCTOR,
        PACKAGE_SCHEDULE_CONSTRUCTOR,
    ];

    /// Callees that do nothing visible to the tests (the log shows the call).
    const QUIET: [u32; 24] = [
        TES_CONDITION_CONSTRUCTOR,
        PACKAGE_EVENT_ACTION_CONSTRUCTOR,
        SET_FORM_TYPE,
        FN_00673180,
        LOCATION_DELETE,
        LOCATION_COPY,
        LOCATION_INIT_ITEM,
        LOCATION_SET_TYPE,
        TARGET_DELETE,
        TARGET_COPY,
        TARGET_INIT_ITEM,
        TARGET_SET_TYPE,
        TES_CONDITION_INIT_ITEM,
        BGS_IDLE_COLLECTION_INIT_ITEM,
        PACKAGE_EVENT_ACTION_SET_KIND,
        PACKAGE_EVENT_ACTION_INIT_ITEM,
        SET_FORM_FLAG_8,
        LOG_FORM_WARNING,
        ASSERT,
        IS_DYNAMIC_FORM_ID,
        GET_FORM_FLAG_8,
        GET_WARNING_COUNT,
        FAKE_DELETE,
        FAKE_INIT_ITEM,
    ];

    /// An engine whose accessor callees behave as the exe's one-line
    /// accessors do (they read the field the code says), whose constructors
    /// return `this`, and whose other callees do nothing.
    pub(super) fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        for address in CONSTRUCTORS {
            e.register(address, |_, a| a[0].into_ret());
        }
        for address in QUIET {
            e.register(address, |_, _| Ret::default());
        }
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(GET_PACK_TYPE, |e, a| {
            (e.mem.i8(a[0] + 0x20) as i32).into_ret()
        });
        e.register(GET_LOCATION, |e, a| e.mem.u32(a[0] + 0x2c).into_ret());
        e.register(GET_FORM_ID, |e, a| e.mem.u32(a[0] + 0x0c).into_ret());
        e.register(GET_COMBAT_STYLE, |e, a| e.mem.u32(a[0] + 0x48).into_ret());
        e.register(SET_COMBAT_STYLE, |e, a| {
            e.mem.set_u32(a[0] + 0x48, a[1]);
            Ret::default()
        });
        e.register(GET_IDLE_COLLECTION, |e, a| {
            e.mem.u32(a[0] + 0x34).into_ret()
        });
        e.register(GET_CONDITIONS, |_, a| (a[0] + 0x40).into_ret());
        e.register(GET_PROCEDURE_TYPE, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(TARGET_GET_TYPE, |e, a| e.mem.u8(a[0]).into_ret());
        e.register(TARGET_GET_VALUE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(LOCATION_GET_TYPE, |e, a| (e.mem.i8(a[0]) as i32).into_ret());
        e.register(FN_00673890, |e, a| {
            (e.mem.u16(a[0] + 0x24) & 0x40 != 0).into_ret()
        });
        e.register(GET_TARGET_ACQUIRE_RADIUS, |e, a| {
            e.mem.f32(a[0] + 0x0c).into_ret()
        });
        e.register(FAKE_NAME, |_, _| NAME_POINTER.into_ret());
        e.put_vtable(DATA_VTABLE, &[FAKE_DELETE, 0, 0, 0, FAKE_INIT_ITEM]);
        let mut slots = vec![0; 0x130 / 4 + 1];
        slots[0x130 / 4] = FAKE_NAME;
        e.put_vtable(PACKAGE_VTABLE, &slots);
        e
    }

    /// A zeroed package of the given type (as the getter sees it).
    pub(super) fn package(e: &mut Engine, kind: i8) -> Ptr<TESPackage> {
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::cPackType, kind);
        e.mem.set_u32(p.addr(), PACKAGE_VTABLE);
        p
    }

    /// A data object with the fake vtable.
    pub(super) fn data_object(e: &mut Engine, size: u32) -> u32 {
        let d = e.mem.alloc(size);
        e.mem.set_u32(d, DATA_VTABLE);
        d
    }

    pub(super) fn log_on(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The argument lists of every call to `address` since `log_on`.
    pub(super) fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The addresses called, in order, without the call under test and the
    /// type getter.
    fn sequence(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .map(|(a, _)| *a)
            .filter(|a| *a != GET_PACK_TYPE)
            .collect()
    }

    #[test]
    fn constructor_builds_base_and_members_then_resets() {
        let mut e = engine();
        let p = e.new_object::<TESPackage>();
        e.set(p, TESPackage::uiRefCount, 7);
        log_on(&mut e);
        assert_eq!(e.call(0x006707c0, &args![p]).ptr::<TESPackage>(), p);
        assert_eq!(e.mem.u32(p.addr()), TES_PACKAGE_VTABLE);
        assert_eq!(
            sequence(&e),
            [
                TES_FORM_CONSTRUCTOR,
                PACKAGE_SCHEDULE_CONSTRUCTOR,
                TES_CONDITION_CONSTRUCTOR,
                PACKAGE_EVENT_ACTION_CONSTRUCTOR,
                PACKAGE_EVENT_ACTION_CONSTRUCTOR,
                PACKAGE_EVENT_ACTION_CONSTRUCTOR,
                SET_FORM_TYPE,
                FN_00673180,
                GET_FORM_ID,
                IS_DYNAMIC_FORM_ID,
            ]
        );
        assert_eq!(
            calls_to(&e, PACKAGE_SCHEDULE_CONSTRUCTOR),
            [[p.addr() + 0x38]]
        );
        assert_eq!(calls_to(&e, TES_CONDITION_CONSTRUCTOR), [[p.addr() + 0x40]]);
        assert_eq!(
            calls_to(&e, PACKAGE_EVENT_ACTION_CONSTRUCTOR),
            [[p.addr() + 0x4c], [p.addr() + 0x5c], [p.addr() + 0x6c]]
        );
        assert_eq!(calls_to(&e, SET_FORM_TYPE), [[p.addr(), 0x49]]);
        assert_eq!(e.get(p, TESPackage::uiRefCount), 0);
        assert_eq!(e.get(p, TESPackage::cPackType), -1);
    }

    #[test]
    fn reset_clears_fields_and_sets_none_types() {
        let mut e = engine();
        let p = package(&mut e, 5);
        e.set(p, TESPackage::iPackFlags, 0xffff_ffff);
        e.set(p, TESPackage::iPackageSpecificFlags, 0xff);
        e.set(p, TESPackage::iFOBehaviorFlags, 0xff);
        for field in [
            TESPackage::pPackData,
            TESPackage::pPackLoc,
            TESPackage::pPackTarg,
            TESPackage::pIdleCollection,
            TESPackage::pCombatStyle,
        ] {
            e.set(p, field, Ptr::new(0x1234));
        }
        log_on(&mut e);
        e.call(0x006709d0, &args![p]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        assert_eq!(e.get(p, TESPackage::iPackageSpecificFlags), 0);
        assert_eq!(e.get(p, TESPackage::iFOBehaviorFlags), 0);
        assert!(e.get(p, TESPackage::pPackData).is_null());
        assert!(e.get(p, TESPackage::pPackLoc).is_null());
        assert!(e.get(p, TESPackage::pPackTarg).is_null());
        assert!(e.get(p, TESPackage::pIdleCollection).is_null());
        assert!(e.get(p, TESPackage::pCombatStyle).is_null());
        assert_eq!(e.get(p, TESPackage::ePROCEDURE_TYPE), -1);
        assert_eq!(e.get(p, TESPackage::cPackType), -1);
        assert_eq!(calls_to(&e, FN_00673180), [[p.addr(), 0]]);
    }

    #[test]
    fn create_package_builds_by_type() {
        let mut e = engine();
        // Types that build nothing.
        for kind in [-1, 9, 0x11, 0x28] {
            log_on(&mut e);
            assert!(e
                .call(0x00670b90, &args![kind])
                .ptr::<TESPackage>()
                .is_null());
            assert!(calls_to(&e, OPERATOR_NEW).is_empty(), "type {kind}");
        }
        // A subclass: size 0xd0, its constructor, then the type is set.
        log_on(&mut e);
        let p = e.call(0x00670b90, &args![0xfi32]).ptr::<TESPackage>();
        assert_eq!(calls_to(&e, OPERATOR_NEW)[0], [0xd0]);
        assert_eq!(calls_to(&e, 0x009e_dbc0), [[p.addr()]]);
        assert_eq!(e.get(p, TESPackage::cPackType), 0xf);
        // The constructors with three stack arguments.
        log_on(&mut e);
        let p = e.call(0x00670b90, &args![0x12i32]).ptr::<TESPackage>();
        assert_eq!(calls_to(&e, OPERATOR_NEW)[0], [0x188]);
        assert_eq!(calls_to(&e, 0x0097_d3a0), [[p.addr(), 0, 0, 0]]);
        log_on(&mut e);
        e.call(0x00670b90, &args![0x16i32]);
        assert_eq!(calls_to(&e, OPERATOR_NEW)[0], [0xac]);
        // The other subclass sizes.
        for (kind, size, ctor) in [
            (0x15, 0x88, 0x009e_c5d0),
            (0x17, 0x9c, 0x009f_91b0),
            (0x18, 0xb4, 0x009f_72f0),
            (0x27, 0x8c, 0x009e_d030),
            (0x1c, 0xd0, 0x009e_dbc0),
        ] {
            log_on(&mut e);
            e.call(0x00670b90, &args![kind]);
            assert_eq!(calls_to(&e, OPERATOR_NEW)[0], [size], "type {kind}");
            assert_eq!(calls_to(&e, ctor).len(), 1, "type {kind}");
        }
        // Any other type: a plain package (0x80 bytes, its own constructor).
        log_on(&mut e);
        let p = e.call(0x00670b90, &args![5i32]).ptr::<TESPackage>();
        assert_eq!(calls_to(&e, OPERATOR_NEW)[0], [0x80]);
        assert_eq!(e.mem.u32(p.addr()), TES_PACKAGE_VTABLE);
        assert_eq!(e.get(p, TESPackage::cPackType), 5);
        // Out of range types are plain packages too.
        log_on(&mut e);
        e.call(0x00670b90, &args![0x30i32]);
        assert_eq!(calls_to(&e, OPERATOR_NEW)[0], [0x80]);
        // A failed allocation builds nothing and calls no constructor.
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        log_on(&mut e);
        assert!(e
            .call(0x00670b90, &args![5i32])
            .ptr::<TESPackage>()
            .is_null());
        assert!(calls_to(&e, 0x0067_07c0).is_empty());
    }

    /// A flag getter test: false without the bit, true with it, other bits
    /// ignored.
    fn check_flag_getter(address: u32, bit: u32) {
        let mut e = engine();
        let p = package(&mut e, 0);
        e.set(p, TESPackage::iPackFlags, !bit);
        assert!(!e.call(address, &args![p]).bool());
        e.set(p, TESPackage::iPackFlags, bit);
        assert!(e.call(address, &args![p]).bool());
    }

    #[test]
    fn flag_getter_00670ed0() {
        check_flag_getter(0x00670ed0, 0x40);
    }

    #[test]
    fn flag_getter_00670ef0() {
        check_flag_getter(0x00670ef0, 0x10);
    }

    #[test]
    fn flag_getter_00670f10() {
        check_flag_getter(0x00670f10, 0x80);
    }

    #[test]
    fn flag_getter_00670f40() {
        check_flag_getter(0x00670f40, 0x20);
    }

    #[test]
    fn flag_getter_00670f60() {
        check_flag_getter(0x00670f60, 0x100);
    }

    #[test]
    fn get_once_per_day_tests_bit_0x400() {
        check_flag_getter(0x00670f90, 0x400);
    }

    #[test]
    fn set_pack_type_same_type_does_nothing() {
        let mut e = engine();
        let p = package(&mut e, 3);
        let data = data_object(&mut e, 8);
        e.set(p, TESPackage::pPackData, Ptr::new(data));
        log_on(&mut e);
        e.call(0x00670fc0, &args![p, 3i32]);
        assert!(sequence(&e).is_empty());
        assert_eq!(e.get(p, TESPackage::pPackData).addr(), data);
        // Type 0x11 counts as 0.
        let q = package(&mut e, 0);
        log_on(&mut e);
        e.call(0x00670fc0, &args![q, 0x11i32]);
        assert!(sequence(&e).is_empty());
    }

    #[test]
    fn set_pack_type_builds_for_type_3_and_deletes_old_data() {
        let mut e = engine();
        let p = package(&mut e, -1);
        let old = data_object(&mut e, 8);
        e.set(p, TESPackage::pPackData, Ptr::new(old));
        e.set(p, TESPackage::iPackageSpecificFlags, 0x55);
        log_on(&mut e);
        e.call(0x00670fc0, &args![p, 3i32]);
        // The old data object is deleted through its vtable, flag 1.
        assert_eq!(calls_to(&e, FAKE_DELETE), [[old, 1]]);
        // A target (type 0 so far -> set to 2), a location and the 8 byte data.
        let target = e.get(p, TESPackage::pPackTarg);
        assert!(!target.is_null());
        assert_eq!(calls_to(&e, TARGET_SET_TYPE), [[target.addr(), 2]]);
        assert!(!e.get(p, TESPackage::pPackLoc).is_null());
        let data = e.get(p, TESPackage::pPackData);
        assert!(!data.is_null() && data.addr() != old);
        assert_eq!(e.mem.block_size(data.addr()), Some(8));
        assert_eq!(calls_to(&e, 0x0067_b950), [[data.addr()]]);
        assert_eq!(e.get(p, TESPackage::cPackType), 3);
        assert_eq!(e.get(p, TESPackage::iPackageSpecificFlags), 0);
    }

    #[test]
    fn set_pack_type_clears_by_old_table_and_builds_other_types() {
        let mut e = engine();
        // Type 12: target removed; the specific flags are reset; location
        // created.
        let p = package(&mut e, -1);
        let target = e.mem.alloc(0x10);
        e.set(p, TESPackage::pPackTarg, Ptr::new(target));
        e.set(p, TESPackage::iPackageSpecificFlags, 0x7f);
        log_on(&mut e);
        e.call(0x00670fc0, &args![p, 12i32]);
        assert_eq!(calls_to(&e, TARGET_DELETE), [[target, 1]]);
        assert!(e.get(p, TESPackage::pPackTarg).is_null());
        assert!(!e.get(p, TESPackage::pPackLoc).is_null());
        assert_eq!(e.get(p, TESPackage::iPackageSpecificFlags), 0);
        assert!(e.get(p, TESPackage::pPackData).is_null());
        // Type 7: its location removed, target created.
        let q = package(&mut e, -1);
        let location = e.mem.alloc(0xc);
        e.set(q, TESPackage::pPackLoc, Ptr::new(location));
        log_on(&mut e);
        e.call(0x00670fc0, &args![q, 7i32]);
        assert_eq!(calls_to(&e, LOCATION_DELETE), [[location, 1]]);
        assert!(e.get(q, TESPackage::pPackLoc).is_null());
        assert!(!e.get(q, TESPackage::pPackTarg).is_null());
        // Type 0x10: bit 0x800000 set, 0x24 byte data, target.
        let r = package(&mut e, -1);
        e.call(0x00670fc0, &args![r, 0x10i32]);
        assert_eq!(e.get(r, TESPackage::iPackFlags) & 0x80_0000, 0x80_0000);
        assert_eq!(
            e.mem.block_size(e.get(r, TESPackage::pPackData).addr()),
            Some(0x28)
        );
        assert!(!e.get(r, TESPackage::pPackTarg).is_null());
        // Type 8: 8 byte data, location, target; type 13: only the data.
        let s = package(&mut e, -1);
        e.call(0x00670fc0, &args![s, 8i32]);
        assert!(!e.get(s, TESPackage::pPackLoc).is_null());
        assert!(!e.get(s, TESPackage::pPackTarg).is_null());
        assert!(!e.get(s, TESPackage::pPackData).is_null());
        let t = package(&mut e, -1);
        e.call(0x00670fc0, &args![t, 13i32]);
        assert!(e.get(t, TESPackage::pPackLoc).is_null());
        assert!(!e.get(t, TESPackage::pPackData).is_null());
        // Dialogue types: target and 0x20 byte data.
        let u = package(&mut e, -1);
        log_on(&mut e);
        e.call(0x00670fc0, &args![u, 0x1ci32]);
        assert_eq!(calls_to(&e, OPERATOR_NEW), [[0x10], [0x20]]);
        assert_eq!(e.get(u, TESPackage::cPackType), 0x1c);
    }

    #[test]
    fn set_pack_type_9_creates_the_second_location() {
        let mut e = engine();
        let p = package(&mut e, -1);
        log_on(&mut e);
        e.call(0x00670fc0, &args![p, 9i32]);
        let data = e.get(p, TESPackage::pPackData).addr();
        assert_ne!(data, 0);
        // The temporary location is copied into a location of its own and
        // then deleted.
        let held = e.mem.u32(data + 4);
        assert_ne!(held, 0);
        let copies = calls_to(&e, LOCATION_COPY);
        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0][0], held);
        let deletes = calls_to(&e, LOCATION_DELETE);
        assert_eq!(deletes, [[copies[0][1], 1]]);
        assert_ne!(copies[0][1], held);
        assert!(!e.get(p, TESPackage::pPackLoc).is_null());
    }

    #[test]
    fn specific_flag_setters() {
        // (address, bit, stored inverted)
        for (address, bit, inverted) in [
            (0x00671a70u32, 0x40u16, false),
            (0x00671ad0, 0x01, true),
            (0x00671b30, 0x02, true),
            (0x00671b90, 0x04, true),
            (0x00671bf0, 0x08, true),
            (0x00671c50, 0x10, true),
            (0x00671cb0, 0x20, true),
        ] {
            let mut e = engine();
            let p = package(&mut e, 0);
            e.set(p, TESPackage::iPackageSpecificFlags, 0x0100);
            e.call(address, &args![p, 1u32]);
            let on = e.get(p, TESPackage::iPackageSpecificFlags);
            e.call(address, &args![p, 0u32]);
            let off = e.get(p, TESPackage::iPackageSpecificFlags);
            let (set, clear) = if inverted { (off, on) } else { (on, off) };
            assert_eq!(set, 0x0100 | bit, "{address:08x}");
            assert_eq!(clear, 0x0100, "{address:08x}");
        }
    }

    #[test]
    fn flag_setter_00671a20_sets_bit_0x800000() {
        let mut e = engine();
        let p = package(&mut e, 0);
        e.set(p, TESPackage::iPackFlags, 1);
        e.call(0x00671a20, &args![p, 1u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x80_0001);
        e.call(0x00671a20, &args![p, 0u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 1);
    }

    #[test]
    fn target_getter_00671d10() {
        let mut e = engine();
        let p = package(&mut e, 0);
        assert!(e.call(0x00671d10, &args![p]).ptr::<()>().is_null());
        e.set(p, TESPackage::pPackTarg, Ptr::new(0x4321));
        assert_eq!(e.call(0x00671d10, &args![p]).u32(), 0x4321);
    }

    #[test]
    fn set_package_location_creates_copies_and_deletes() {
        let mut e = engine();
        let p = package(&mut e, 0);
        let source = e.mem.alloc(0xc);
        log_on(&mut e);
        e.call(0x00671d30, &args![p, source]);
        let made = e.get(p, TESPackage::pPackLoc).addr();
        assert_ne!(made, 0);
        assert_eq!(calls_to(&e, LOCATION_COPY), [[made, source, 0]]);
        // A second call reuses the object.
        log_on(&mut e);
        e.call(0x00671d30, &args![p, source]);
        assert_eq!(e.get(p, TESPackage::pPackLoc).addr(), made);
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        // Null deletes it and clears the field.
        log_on(&mut e);
        e.call(0x00671d30, &args![p, 0u32]);
        assert_eq!(calls_to(&e, LOCATION_DELETE), [[made, 1]]);
        assert!(e.get(p, TESPackage::pPackLoc).is_null());
        // Null on an empty package deletes nothing.
        log_on(&mut e);
        e.call(0x00671d30, &args![p, 0u32]);
        assert!(calls_to(&e, LOCATION_DELETE).is_empty());
    }

    #[test]
    fn set_second_location_by_type() {
        let mut e = engine();
        let source = e.mem.alloc(0xc);
        // Dialogue types: the 0x20 byte data object is built; slot +0xc.
        let p = package(&mut e, 0xf);
        log_on(&mut e);
        e.call(0x00671e10, &args![p, source]);
        let data = e.get(p, TESPackage::pPackData).addr();
        assert_eq!(e.mem.block_size(data), Some(0x20));
        let made = e.mem.u32(data + 0xc);
        assert_ne!(made, 0);
        assert_eq!(calls_to(&e, LOCATION_COPY), [[made, source, 0]]);
        // The other dialogue type reuses the object.
        let p = package(&mut e, 0x1c);
        let data = e.mem.alloc(0x20);
        e.set(p, TESPackage::pPackData, Ptr::new(data));
        let held = e.mem.alloc(0xc);
        e.mem.set_u32(data + 0xc, held);
        log_on(&mut e);
        e.call(0x00671e10, &args![p, source]);
        assert_eq!(calls_to(&e, LOCATION_COPY), [[held, source, 0]]);
        // Small data objects keep it at +4: size and constructor by type.
        for (kind, size, ctor) in [
            (3, 8, 0x0067_b950),
            (2, 0xc, 0x0067_bbf0),
            (9, 8, 0x0067_acf0),
            (1, 0xc, 0x0067_bfd0),
            (8, 8, 0x0067_c770),
        ] {
            let p = package(&mut e, kind);
            log_on(&mut e);
            e.call(0x00671e10, &args![p, source]);
            let data = e.get(p, TESPackage::pPackData).addr();
            assert_eq!(
                e.mem.block_size(data),
                Some(u32::div_ceil(size, 8) * 8),
                "type {kind}"
            );
            assert_eq!(calls_to(&e, ctor), [[data]], "type {kind}");
            assert_ne!(e.mem.u32(data + 4), 0, "type {kind}");
        }
        // Use weapon: slot +8 of the 0x24 byte data object.
        let p = package(&mut e, 0x10);
        e.call(0x00671e10, &args![p, source]);
        let data = e.get(p, TESPackage::pPackData).addr();
        assert_eq!(e.mem.block_size(data), Some(0x28));
        assert_ne!(e.mem.u32(data + 8), 0);
        // A null source deletes the held location.
        let p = package(&mut e, 3);
        let data = e.mem.alloc(8);
        e.set(p, TESPackage::pPackData, Ptr::new(data));
        let held = e.mem.alloc(0xc);
        e.mem.set_u32(data + 4, held);
        log_on(&mut e);
        e.call(0x00671e10, &args![p, 0u32]);
        assert_eq!(calls_to(&e, LOCATION_DELETE), [[held, 1]]);
        assert_eq!(e.mem.u32(data + 4), 0);
        // Another type does nothing.
        let p = package(&mut e, 5);
        log_on(&mut e);
        e.call(0x00671e10, &args![p, source]);
        assert!(sequence(&e).is_empty());
        assert!(e.get(p, TESPackage::pPackData).is_null());
    }

    /// A package of `kind` with a dialogue data object holding known values.
    fn dialogue_package(e: &mut Engine, kind: i8) -> (Ptr<TESPackage>, u32) {
        let p = package(e, kind);
        let data = e.mem.alloc(0x20);
        e.set(p, TESPackage::pPackData, Ptr::new(data));
        e.mem.set_f32(data + 4, 2.5);
        e.mem.set_u32(data + 8, 0x7007);
        e.mem.set_u8(data + 0x10, 3);
        e.mem.set_u8(data + 0x11, 4);
        e.mem.set_u8(data + 0x18, 5);
        (p, data)
    }

    #[test]
    fn dialogue_say_to_flag() {
        let mut e = engine();
        for kind in [0xf, 0x1c] {
            let (p, _) = dialogue_package(&mut e, kind);
            assert_eq!(e.call(0x00672710, &args![p]).u8(), 5);
        }
        let (p, _) = dialogue_package(&mut e, 5);
        assert_eq!(e.call(0x00672710, &args![p]).u8(), 0);
        let p = package(&mut e, 0xf);
        assert_eq!(e.call(0x00672710, &args![p]).u8(), 0);
    }

    #[test]
    fn dialogue_topic() {
        let mut e = engine();
        let (p, _) = dialogue_package(&mut e, 0xf);
        assert_eq!(e.call(0x00672760, &args![p]).u32(), 0x7007);
        let (p, _) = dialogue_package(&mut e, 0x1c);
        assert_eq!(e.call(0x00672760, &args![p]).u32(), 0x7007);
        let (p, _) = dialogue_package(&mut e, 4);
        assert_eq!(e.call(0x00672760, &args![p]).u32(), 0);
        let p = package(&mut e, 0xf);
        assert_eq!(e.call(0x00672760, &args![p]).u32(), 0);
    }

    #[test]
    fn dialogue_do_not_control_target() {
        let mut e = engine();
        let (p, _) = dialogue_package(&mut e, 0xf);
        assert_eq!(e.call(0x006727b0, &args![p]).u8(), 4);
        let (p, _) = dialogue_package(&mut e, 0x1c);
        assert_eq!(e.call(0x006727b0, &args![p]).u8(), 4);
        let (p, _) = dialogue_package(&mut e, 4);
        assert_eq!(e.call(0x006727b0, &args![p]).u8(), 0);
    }

    #[test]
    fn dialogue_no_headtrack() {
        let mut e = engine();
        let (p, _) = dialogue_package(&mut e, 0xf);
        assert_eq!(e.call(0x00672800, &args![p]).u8(), 3);
        let (p, _) = dialogue_package(&mut e, 4);
        assert_eq!(e.call(0x00672800, &args![p]).u8(), 0);
        let p = package(&mut e, 0xf);
        assert_eq!(e.call(0x00672800, &args![p]).u8(), 0);
    }

    #[test]
    fn dialogue_fov_defaults_to_one() {
        let mut e = engine();
        let (p, _) = dialogue_package(&mut e, 0x1c);
        assert_eq!(e.call(0x00672850, &args![p]).f32(), 2.5);
        let (p, _) = dialogue_package(&mut e, 4);
        assert_eq!(e.call(0x00672850, &args![p]).f32(), 1.0);
        let p = package(&mut e, 0xf);
        assert_eq!(e.call(0x00672850, &args![p]).f32(), 1.0);
    }

    #[test]
    fn acquire_radius() {
        let mut e = engine();
        let out = e.mem.alloc(4);
        e.mem.set_f32(out, 9.0);
        // Type 0xC: from 00676280(argument), unsigned: here the location's
        // own radius (0067f1c0) with no reference resolved.
        e.register(LOCATION_GET_RADIUS, |_, _| 0xffff_ffffu32.into_ret());
        e.register(LOCATION_RESOLVE_REFERENCE, |_, _| Ret::default());
        let p = package(&mut e, 0xc);
        let location = e.mem.alloc(0xc);
        e.set(p, TESPackage::pPackLoc, Ptr::new(location));
        assert!(e.call(0x006728a0, &args![p, 0xffff_fffeu32, out]).bool());
        assert_eq!(e.mem.f32(out), 4_294_967_295.0f64 as f32);
        // No target: false, the radius left at 0.
        let p = package(&mut e, 4);
        e.mem.set_f32(out, 9.0);
        assert!(!e.call(0x006728a0, &args![p, 0u32, out]).bool());
        assert_eq!(e.mem.f32(out), 0.0);
        // A target with a positive radius.
        let target = e.mem.alloc(0x10);
        e.mem.set_f32(target + 0xc, 5.5);
        e.set(p, TESPackage::pPackTarg, Ptr::new(target));
        assert!(e.call(0x006728a0, &args![p, 0u32, out]).bool());
        assert_eq!(e.mem.f32(out), 5.5);
        // Zero and negative radii are rejected; NaN is accepted.
        for (value, accepted) in [(0.0f32, false), (-1.0, false), (f32::NAN, true)] {
            e.mem.set_f32(target + 0xc, value);
            e.mem.set_f32(out, 9.0);
            assert_eq!(e.call(0x006728a0, &args![p, 0u32, out]).bool(), accepted);
            if !accepted {
                assert_eq!(e.mem.f32(out), 0.0);
            }
        }
    }

    #[test]
    fn initial_target_count() {
        let mut e = engine();
        let p = package(&mut e, 4);
        assert_eq!(e.call(0x00672930, &args![p]).i32(), 0);
        let target = e.mem.alloc(0x10);
        e.set(p, TESPackage::pPackTarg, Ptr::new(target));
        // Target types 0 and 3: one.
        assert_eq!(e.call(0x00672930, &args![p]).i32(), 1);
        e.mem.set_u8(target, 3);
        assert_eq!(e.call(0x00672930, &args![p]).i32(), 1);
        // Another type: its value when positive.
        e.mem.set_u8(target, 1);
        e.mem.set_u32(target + 8, 6);
        assert_eq!(e.call(0x00672930, &args![p]).i32(), 6);
        // Otherwise by the procedure type.
        e.mem.set_u32(target + 8, 0);
        for (procedure, expected) in [
            (0x1c, 1),
            (2, 0x7fff_ffff),
            (3, 0x7fff_ffff),
            (0x1a, 0x7fff_ffff),
            (5, 0),
        ] {
            e.set(p, TESPackage::ePROCEDURE_TYPE, procedure);
            assert_eq!(e.call(0x00672930, &args![p]).i32(), expected);
        }
        // A negative value does not count.
        e.mem.set_u32(target + 8, 0xffff_ffff);
        e.set(p, TESPackage::ePROCEDURE_TYPE, 2);
        assert_eq!(e.call(0x00672930, &args![p]).i32(), 0x7fff_ffff);
    }

    #[test]
    fn dialogue_topic_setter() {
        let mut e = engine();
        // Built when missing.
        let p = package(&mut e, 0xf);
        e.call(0x006729d0, &args![p, 0x7007u32]);
        let data = e.get(p, TESPackage::pPackData).addr();
        assert_eq!(e.mem.block_size(data), Some(0x20));
        assert_eq!(e.mem.u32(data + 8), 0x7007);
        // Reused, and not for another type.
        e.call(0x006729d0, &args![p, 1u32]);
        assert_eq!(e.get(p, TESPackage::pPackData).addr(), data);
        assert_eq!(e.mem.u32(data + 8), 1);
        let q = package(&mut e, 4);
        e.call(0x006729d0, &args![q, 1u32]);
        assert!(e.get(q, TESPackage::pPackData).is_null());
    }

    #[test]
    fn dialogue_do_not_control_target_setter() {
        let mut e = engine();
        let p = package(&mut e, 0x1c);
        e.call(0x00672aa0, &args![p, 1u32]);
        let data = e.get(p, TESPackage::pPackData).addr();
        assert_eq!(e.mem.u8(data + 0x11), 1);
        let q = package(&mut e, 4);
        e.call(0x00672aa0, &args![q, 1u32]);
        assert!(e.get(q, TESPackage::pPackData).is_null());
    }

    #[test]
    fn dialogue_no_headtrack_setter() {
        let mut e = engine();
        let p = package(&mut e, 0xf);
        e.call(0x00672b70, &args![p, 1u32]);
        let data = e.get(p, TESPackage::pPackData).addr();
        assert_eq!(e.mem.u8(data + 0x10), 1);
        assert_eq!(e.mem.u8(data + 0x11), 0);
        let q = package(&mut e, 4);
        e.call(0x00672b70, &args![q, 1u32]);
        assert!(e.get(q, TESPackage::pPackData).is_null());
    }

    #[test]
    fn dialogue_fov_setter() {
        let mut e = engine();
        let p = package(&mut e, 0xf);
        e.call(0x00672c40, &args![p, 1.75f32]);
        let data = e.get(p, TESPackage::pPackData).addr();
        assert_eq!(e.mem.f32(data + 4), 1.75);
        let q = package(&mut e, 4);
        e.call(0x00672c40, &args![q, 1.75f32]);
        assert!(e.get(q, TESPackage::pPackData).is_null());
    }

    #[test]
    fn get_or_create_use_weapon_data() {
        let mut e = engine();
        let p = package(&mut e, 0x10);
        log_on(&mut e);
        let data = e.call(0x00672d10, &args![p]).u32();
        assert_eq!(e.mem.block_size(data), Some(0x28));
        assert_eq!(calls_to(&e, USE_WEAPON_DATA_CONSTRUCTOR), [[data]]);
        assert!(calls_to(&e, ASSERT).is_empty());
        // The second call returns the same object.
        assert_eq!(e.call(0x00672d10, &args![p]).u32(), data);
        // Another type asserts (line 0x607) but still builds.
        let q = package(&mut e, 4);
        log_on(&mut e);
        let data = e.call(0x00672d10, &args![q]).u32();
        assert_ne!(data, 0);
        assert_eq!(calls_to(&e, ASSERT), [[PACKAGE_SOURCE_FILE, 0x607]]);
    }

    #[test]
    fn second_location_getter() {
        let mut e = engine();
        // No data object: null.
        let p = package(&mut e, 3);
        assert_eq!(e.call(0x00672dd0, &args![p]).u32(), 0);
        for (kind, offset) in [
            (0xf, 0xc),
            (0x1c, 0xc),
            (3, 4),
            (2, 4),
            (9, 4),
            (8, 4),
            (1, 4),
            (0x10, 8),
        ] {
            let p = package(&mut e, kind);
            let data = e.mem.alloc(0x24);
            e.set(p, TESPackage::pPackData, Ptr::new(data));
            e.mem.set_u32(data + offset, 0x8800 + kind as u32);
            assert_eq!(e.call(0x00672dd0, &args![p]).u32(), 0x8800 + kind as u32);
        }
        // Another type with a data object: null.
        let p = package(&mut e, 5);
        let data = e.mem.alloc(0x24);
        e.set(p, TESPackage::pPackData, Ptr::new(data));
        assert_eq!(e.call(0x00672dd0, &args![p]).u32(), 0);
    }

    #[test]
    fn package_search_location() {
        let mut e = engine();
        // Own location for types 0, 0xC, 0x11 and 0x1C.
        for kind in [0, 0xc, 0x11, 0x1c] {
            let p = package(&mut e, kind);
            e.set(p, TESPackage::pPackLoc, Ptr::new(0x1110));
            assert_eq!(e.call(0x00672f20, &args![p]).u32(), 0x1110, "type {kind}");
        }
        // Types 2, 3, 8 and 0x10: the second location, unless it is of
        // location type 7, when the own location replaces it.
        let second = e.mem.alloc(0xc);
        for kind in [2, 3, 8] {
            let p = package(&mut e, kind);
            let data = e.mem.alloc(0x24);
            e.set(p, TESPackage::pPackData, Ptr::new(data));
            e.mem.set_u32(data + 4, second);
            e.set(p, TESPackage::pPackLoc, Ptr::new(0x1110));
            e.mem.set_u8(second, 1);
            assert_eq!(e.call(0x00672f20, &args![p]).u32(), second, "type {kind}");
            e.mem.set_u8(second, 7);
            assert_eq!(e.call(0x00672f20, &args![p]).u32(), 0x1110, "type {kind}");
        }
        let p = package(&mut e, 0x10);
        let data = e.mem.alloc(0x24);
        e.set(p, TESPackage::pPackData, Ptr::new(data));
        e.mem.set_u8(second, 2);
        e.mem.set_u32(data + 8, second);
        assert_eq!(e.call(0x00672f20, &args![p]).u32(), second);
        // No second location: null.
        let p = package(&mut e, 2);
        assert_eq!(e.call(0x00672f20, &args![p]).u32(), 0);
        // Other types: null.
        let p = package(&mut e, 1);
        e.set(p, TESPackage::pPackLoc, Ptr::new(0x1110));
        assert_eq!(e.call(0x00672f20, &args![p]).u32(), 0);
    }

    #[test]
    fn set_package_target_creates_copies_and_deletes() {
        let mut e = engine();
        let p = package(&mut e, 0);
        let source = e.mem.alloc(0x10);
        log_on(&mut e);
        e.call(0x00672fc0, &args![p, source]);
        let made = e.get(p, TESPackage::pPackTarg).addr();
        assert_ne!(made, 0);
        assert_eq!(calls_to(&e, TARGET_COPY), [[made, source]]);
        log_on(&mut e);
        e.call(0x00672fc0, &args![p, source]);
        assert!(calls_to(&e, OPERATOR_NEW).is_empty());
        log_on(&mut e);
        e.call(0x00672fc0, &args![p, 0u32]);
        assert_eq!(calls_to(&e, TARGET_DELETE), [[made, 1]]);
        assert!(e.get(p, TESPackage::pPackTarg).is_null());
        log_on(&mut e);
        e.call(0x00672fc0, &args![p, 0u32]);
        assert!(calls_to(&e, TARGET_DELETE).is_empty());
    }

    /// Registers the combat-style resolution doubles: the compile index
    /// turns the stored ID into `0x0100_0000 | id`, the lookup gives the
    /// form `form`, the cast `style`.
    fn resolve_combat_style(e: &mut Engine, form: u32, style: u32) {
        e.register(TES_FORM_GET_FILE, |_, _| 0xf11e.into_ret());
        e.register(TES_FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], 0x0100_0000 | id);
            Ret::default()
        });
        e.register_double(LOOKUP_FORM, move |_, _| form.into_ret());
        e.register_double(RT_DYNAMIC_CAST, move |_, _| style.into_ret());
    }

    #[test]
    fn init_item_does_nothing_when_flag_8_is_set() {
        let mut e = engine();
        e.register(GET_FORM_FLAG_8, |_, _| true.into_ret());
        let p = package(&mut e, 9);
        log_on(&mut e);
        e.call(0x00674930, &args![p]);
        assert_eq!(sequence(&e), [GET_FORM_FLAG_8]);
    }

    #[test]
    fn init_item_initialises_members_and_resolves_the_combat_style() {
        let mut e = engine();
        let p = package(&mut e, 4);
        let location = e.mem.alloc(0xc);
        let target = e.mem.alloc(0x10);
        let data = data_object(&mut e, 8);
        e.set(p, TESPackage::pPackLoc, Ptr::new(location));
        e.set(p, TESPackage::pPackTarg, Ptr::new(target));
        e.set(p, TESPackage::pPackData, Ptr::new(data));
        e.set(p, TESPackage::pIdleCollection, Ptr::new(0x5550));
        e.set(p, TESPackage::pCombatStyle, Ptr::new(0x1234));
        e.mem.set_u32(p.addr() + 0x0c, 0xabcd);
        resolve_combat_style(&mut e, 0xf0, 0x5ea1);
        log_on(&mut e);
        e.call(0x00674930, &args![p]);
        assert_eq!(calls_to(&e, LOCATION_INIT_ITEM), [[location, p.addr()]]);
        assert_eq!(calls_to(&e, TARGET_INIT_ITEM), [[target, p.addr()]]);
        assert_eq!(
            calls_to(&e, TES_CONDITION_INIT_ITEM),
            [[p.addr() + 0x40, p.addr()]]
        );
        assert_eq!(
            calls_to(&e, BGS_IDLE_COLLECTION_INIT_ITEM),
            [[0x5550, p.addr()]]
        );
        assert_eq!(calls_to(&e, FAKE_INIT_ITEM), [[data, p.addr()]]);
        // The combat style: file of the package (-1), compile index of the
        // slot, lookup of the resolved ID, dynamic cast to the style class.
        assert_eq!(calls_to(&e, TES_FORM_GET_FILE), [[p.addr(), 0xffff_ffff]]);
        assert_eq!(calls_to(&e, LOOKUP_FORM), [[0x0100_1234]]);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            [[
                0xf0,
                0,
                TYPE_DESCRIPTOR_TES_FORM,
                TYPE_DESCRIPTOR_COMBAT_STYLE,
                0
            ]]
        );
        assert_eq!(e.get(p, TESPackage::pCombatStyle).addr(), 0x5ea1);
        assert!(calls_to(&e, LOG_FORM_WARNING).is_empty());
        // Event actions: kinds 0, 1, 2, then their InitItem with the package.
        assert_eq!(
            calls_to(&e, PACKAGE_EVENT_ACTION_SET_KIND),
            [
                [p.addr() + 0x4c, 0],
                [p.addr() + 0x5c, 1],
                [p.addr() + 0x6c, 2]
            ]
        );
        assert_eq!(
            calls_to(&e, PACKAGE_EVENT_ACTION_INIT_ITEM),
            [
                [p.addr() + 0x4c, p.addr()],
                [p.addr() + 0x5c, p.addr()],
                [p.addr() + 0x6c, p.addr()]
            ]
        );
        // Finally the form flag.
        assert_eq!(calls_to(&e, SET_FORM_FLAG_8), [[p.addr(), 1]]);
        assert_eq!(e.get(p, TESPackage::iPackFlags) & FLAG_DISABLED, 0);
    }

    #[test]
    fn init_item_logs_a_missing_combat_style_and_new_warnings() {
        let mut e = engine();
        let p = package(&mut e, 4);
        e.set(p, TESPackage::pCombatStyle, Ptr::new(0x1234));
        e.mem.set_u32(p.addr() + 0x0c, 0xabcd);
        resolve_combat_style(&mut e, 0xf0, 0);
        // The warning counter reads 0 first and 1 afterwards.
        let reads = Rc::new(Cell::new(0u32));
        let counter = reads.clone();
        e.register_double(GET_WARNING_COUNT, move |_, _| {
            let n = counter.get();
            counter.set(n + 1);
            u32::from(n >= 1).into_ret()
        });
        log_on(&mut e);
        e.call(0x00674930, &args![p]);
        assert!(e.get(p, TESPackage::pCombatStyle).is_null());
        assert_eq!(
            calls_to(&e, LOG_FORM_WARNING),
            vec![
                vec![
                    MESSAGE_COMBAT_STYLE_MISSING,
                    0x0100_1234,
                    NAME_POINTER,
                    0xabcd
                ],
                vec![MESSAGE_WARNINGS_GENERATED, NAME_POINTER, 0xabcd]
            ]
        );
        assert_eq!(
            e.get(p, TESPackage::iPackFlags) & FLAG_DISABLED,
            FLAG_DISABLED
        );
    }

    #[test]
    fn init_item_type_fixups() {
        // Type 9 and 0xC: the location type is set to 6 under the flag tests.
        let mut e = engine();
        let location = e.mem.alloc(0xc);
        let p = package(&mut e, 9);
        e.set(p, TESPackage::pPackLoc, Ptr::new(location));
        e.set(p, TESPackage::iPackageSpecificFlags, 2);
        log_on(&mut e);
        e.call(0x00674930, &args![p]);
        assert_eq!(calls_to(&e, LOCATION_SET_TYPE), [[location, 6]]);
        let p = package(&mut e, 9);
        e.set(p, TESPackage::pPackLoc, Ptr::new(location));
        log_on(&mut e);
        e.call(0x00674930, &args![p]);
        assert!(calls_to(&e, LOCATION_SET_TYPE).is_empty());
        let p = package(&mut e, 0xc);
        e.set(p, TESPackage::pPackLoc, Ptr::new(location));
        e.set(p, TESPackage::iPackageSpecificFlags, 0x40);
        log_on(&mut e);
        e.call(0x00674930, &args![p]);
        assert_eq!(calls_to(&e, LOCATION_SET_TYPE), [[location, 6]]);
        // Type 0xD without a location: one is made, typed 6, stored by copy
        // and the temporary deleted.
        let p = package(&mut e, 0xd);
        let data = data_object(&mut e, 8);
        e.set(p, TESPackage::pPackData, Ptr::new(data));
        log_on(&mut e);
        e.call(0x00674930, &args![p]);
        let made = calls_to(&e, LOCATION_SET_TYPE);
        assert_eq!(made.len(), 1);
        assert_eq!(made[0][1], 6);
        assert_eq!(calls_to(&e, LOCATION_COPY).len(), 1);
        assert_eq!(calls_to(&e, LOCATION_DELETE), [[made[0][0], 1]]);
        assert!(!e.get(p, TESPackage::pPackLoc).is_null());
        // Type 0xD with the data flag at +5 and a location: that one is typed.
        let p = package(&mut e, 0xd);
        let data = data_object(&mut e, 8);
        e.mem.set_u8(data + 5, 1);
        e.set(p, TESPackage::pPackData, Ptr::new(data));
        e.set(p, TESPackage::pPackLoc, Ptr::new(location));
        log_on(&mut e);
        e.call(0x00674930, &args![p]);
        assert_eq!(calls_to(&e, LOCATION_SET_TYPE), [[location, 6]]);
        // Type 0xE under flag 2 without a target: one is made and typed 3.
        let p = package(&mut e, 0xe);
        e.set(p, TESPackage::iPackageSpecificFlags, 2);
        log_on(&mut e);
        e.call(0x00674930, &args![p]);
        let target = e.get(p, TESPackage::pPackTarg).addr();
        assert_ne!(target, 0);
        assert_eq!(calls_to(&e, TARGET_SET_TYPE), [[target, 3]]);
        // Without the flag nothing happens.
        let p = package(&mut e, 0xe);
        log_on(&mut e);
        e.call(0x00674930, &args![p]);
        assert!(calls_to(&e, TARGET_SET_TYPE).is_empty());
        assert!(e.get(p, TESPackage::pPackTarg).is_null());
    }

    #[test]
    fn specific_flag_bit_2() {
        let mut e = engine();
        let p = package(&mut e, 0);
        assert!(!e.call(0x00674d20, &args![p]).bool());
        e.set(p, TESPackage::iPackageSpecificFlags, 2);
        assert!(e.call(0x00674d20, &args![p]).bool());
        e.set(p, TESPackage::iPackageSpecificFlags, 0xfffd);
        assert!(!e.call(0x00674d20, &args![p]).bool());
    }

    #[test]
    fn get_is_created_tests_bit_0x800() {
        check_flag_getter(0x00674d40, 0x800);
    }

    #[test]
    fn set_is_created_only_for_dynamic_form_ids() {
        let mut e = engine();
        e.set_global(DATA_HANDLER, 0x7777u32);
        let p = package(&mut e, 0);
        e.mem.set_u32(p.addr() + 0x0c, 0xff00_0001);
        // Not dynamic: nothing changes.
        log_on(&mut e);
        e.call(0x00674d70, &args![p, 1u32]);
        assert_eq!(calls_to(&e, IS_DYNAMIC_FORM_ID), [[0x7777, 0xff00_0001]]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        // Dynamic: set and clear.
        e.register(IS_DYNAMIC_FORM_ID, |_, _| true.into_ret());
        e.set(p, TESPackage::iPackFlags, 1);
        e.call(0x00674d70, &args![p, 1u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x801);
        e.call(0x00674d70, &args![p, 0u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 1);
    }
}

#[cfg(test)]
mod second_batch_tests {
    use super::tests::{calls_to, engine, log_on, package};
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    // A fake world for the second batch: references, cells, forms and
    // processes are zeroed blocks whose fields the doubles of the callees
    // read. Offsets inside a block:
    //   +0x04 form type byte (forms)        +0x20 base form (references)
    //   +0x24 cell flag byte                +0x40 parent cell
    //   +0x44 linked reference              +0x68 process
    //   +0x100 world space                  +0x180 position (3 floats)
    //   +0x190 actor point (3 floats)       +0x1a0 bounds height (float)
    //   +0x1f0 is-actor byte                +0x1f4 kind
    //   +0x1f8 reject byte                  +0x1fc form-test byte
    //   +0x200 target reference             +0x204 slot 0x290 byte
    //   +0x208 slot 0x294 answer            +0x20c slot 0x298 answer
    //   +0x210 reach answer byte            +0x214 reach height (float)
    //   +0x218 form flag 0x20 byte          +0x21c model bound (float)
    //   +0x220 start-location world         +0x224 start-location cell
    //   +0x228 first reference (cells)      +0x230 data x   +0x234 data y
    //   +0x240 start-location point (3 floats)
    //   +0x250 eye level (float)       +0x300 the point of the last reach call
    // Processes use +0x100 package, +0x104 / +0x108 chosen references for the
    // first and second location, +0x10c point pointer, +0x110 slot 0x4c8
    // answer, +0x114 slot 0x11c byte, +0x118 slot 0x52c answer, +0x28 field.
    const REFERENCE_VTABLE: u32 = 0x0130_2000;
    const PROCESS_VTABLE: u32 = 0x0130_3000;
    const PACKAGE_VTABLE_B: u32 = 0x0130_4000;
    const SWITCH_PAGE: u32 = 0x0130_5000;
    const REFERENCE_SLOT: u32 = 0x7100_0000;
    const PROCESS_SLOT: u32 = 0x7200_0000;
    const PACKAGE_SLOT: u32 = 0x7300_0000;
    const PLAYER_BLOCK_SIZE: u32 = 0x400;

    /// A reference (or cell or form) block with the reference vtable.
    fn block(e: &mut Engine) -> u32 {
        let b = e.mem.alloc(PLAYER_BLOCK_SIZE);
        e.mem.set_u32(b, REFERENCE_VTABLE);
        b
    }

    fn process_block(e: &mut Engine) -> u32 {
        let b = e.mem.alloc(PLAYER_BLOCK_SIZE);
        e.mem.set_u32(b, PROCESS_VTABLE);
        b
    }

    fn set_point(e: &mut Engine, at: u32, point: [f32; 3]) {
        for (i, v) in point.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn point(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    /// The engine of the first batch plus the second batch's doubles.
    fn world() -> Engine {
        let mut e = engine();
        for page in [
            0x0101_1000u32,
            0x0101_6000,
            0x0101_7000,
            0x0101_a000,
            0x011c_a000,
            0x011c_d000,
            0x011d_0000,
            0x011d_6000,
            0x011d_e000,
            0x011f_4000,
            SWITCH_PAGE,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(0x0101_1588, 0.5f64);
        e.set_global(0x0101_1590, 2.0f64);
        e.set_global(0x0101_7a40, 100.0f64);
        e.set_global(0x0101_a6b0, -1.0f64);
        e.set_global(0x0101_7868, 20.0f32);
        e.set_global(0x0101_62c0, 3.0f32);
        // Settings (value at +4 of the setting object; here the double
        // answers with the address inside the same page).
        e.set_global(0x011c_de9c, 77i32);
        e.set_global(0x011c_dad4, 2i32);
        e.set_global(0x011d_09d4, 5.0f32);
        e.set_global(0x011d_65b0, 500.0f32);
        e.register(SETTING_VALUE_ADDRESS, |_, a| (a[0] + 4).into_ret());
        e.register(SETTING_FLOAT_ADDRESS, |_, a| (a[0] + 4).into_ret());
        e.register(SETTING_VALUE_OR_ZERO, |_, a| (a[0] + 4).into_ret());

        let vtable_slots = |base: u32, slots: &[u32]| {
            let mut v = vec![0u32; 0x540 / 4];
            for s in slots {
                v[(*s / 4) as usize] = base + s;
            }
            v
        };
        let reference_slots = [
            SLOT_IS_ACTOR,
            SLOT_FORM_NAME,
            SLOT_ACTOR_POINT,
            SLOT_POSITION,
            SLOT_BOUNDS_1DC,
            SLOT_KIND,
            SLOT_REJECT_22C,
            SLOT_FORM_E4,
            SLOT_TARGET_REFERENCE,
            SLOT_ACTOR_290,
            SLOT_ACTOR_WORLD_SPACE,
            SLOT_ACTOR_CELL,
            SLOT_REACH,
            SLOT_REACH_HEIGHT,
        ];
        e.put_vtable(
            REFERENCE_VTABLE,
            &vtable_slots(REFERENCE_SLOT, &reference_slots),
        );
        e.register(REFERENCE_SLOT + SLOT_IS_ACTOR, |e, a| {
            e.mem.u8(a[0] + 0x1f0).into_ret()
        });
        e.register(REFERENCE_SLOT + SLOT_FORM_NAME, |_, _| {
            0x0aa0_0002u32.into_ret()
        });
        e.register(REFERENCE_SLOT + SLOT_ACTOR_POINT, |e, a| {
            for i in 0..3 {
                let w = e.mem.u32(a[0] + 0x190 + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, w);
            }
            a[1].into_ret()
        });
        e.register(REFERENCE_SLOT + SLOT_POSITION, |_, a| {
            (a[0] + 0x180).into_ret()
        });
        e.register(REFERENCE_SLOT + SLOT_BOUNDS_1DC, |e, a| {
            let z = e.mem.u32(a[0] + 0x1a0);
            e.mem.set_u32(a[1] + 8, z);
            a[1].into_ret()
        });
        e.register(REFERENCE_SLOT + SLOT_KIND, |e, a| {
            e.mem.u32(a[0] + 0x1f4).into_ret()
        });
        e.register(REFERENCE_SLOT + SLOT_REJECT_22C, |e, a| {
            e.mem.u8(a[0] + 0x1f8).into_ret()
        });
        e.register(REFERENCE_SLOT + SLOT_FORM_E4, |e, a| {
            e.mem.u8(a[0] + 0x1fc).into_ret()
        });
        e.register(REFERENCE_SLOT + SLOT_TARGET_REFERENCE, |e, a| {
            e.mem.u32(a[0] + 0x200).into_ret()
        });
        e.register(REFERENCE_SLOT + SLOT_ACTOR_290, |e, a| {
            e.mem.u8(a[0] + 0x204).into_ret()
        });
        e.register(REFERENCE_SLOT + SLOT_ACTOR_WORLD_SPACE, |e, a| {
            e.mem.u32(a[0] + 0x208).into_ret()
        });
        e.register(REFERENCE_SLOT + SLOT_ACTOR_CELL, |e, a| {
            e.mem.u32(a[0] + 0x20c).into_ret()
        });
        e.register(REFERENCE_SLOT + SLOT_REACH, |e, a| {
            // Keeps the point it was asked about at +0x300.
            for i in 0..3 {
                let w = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 0x300 + 4 * i, w);
            }
            e.mem.u8(a[0] + 0x210).into_ret()
        });
        e.register(REFERENCE_SLOT + SLOT_REACH_HEIGHT, |e, a| {
            e.mem.f32(a[0] + 0x214).into_ret()
        });

        let process_slots = [
            SLOT_PROCESS_PACKAGE,
            SLOT_PROCESS_REFERENCE_514,
            SLOT_PROCESS_REFERENCE_51C,
            SLOT_PROCESS_POINT,
            SLOT_PROCESS_4C8,
            SLOT_PROCESS_11C,
            SLOT_PROCESS_52C,
            SLOT_PROCESS_288,
        ];
        e.put_vtable(PROCESS_VTABLE, &vtable_slots(PROCESS_SLOT, &process_slots));
        e.register(PROCESS_SLOT + SLOT_PROCESS_PACKAGE, |e, a| {
            e.mem.u32(a[0] + 0x100).into_ret()
        });
        e.register(PROCESS_SLOT + SLOT_PROCESS_REFERENCE_514, |e, a| {
            e.mem.u32(a[0] + 0x104).into_ret()
        });
        e.register(PROCESS_SLOT + SLOT_PROCESS_REFERENCE_51C, |e, a| {
            e.mem.u32(a[0] + 0x108).into_ret()
        });
        e.register(PROCESS_SLOT + SLOT_PROCESS_POINT, |e, a| {
            e.mem.u32(a[0] + 0x10c).into_ret()
        });
        e.register(PROCESS_SLOT + SLOT_PROCESS_4C8, |e, a| {
            e.mem.u32(a[0] + 0x110).into_ret()
        });
        e.register(PROCESS_SLOT + SLOT_PROCESS_11C, |e, a| {
            e.mem.u8(a[0] + 0x114).into_ret()
        });
        e.register(PROCESS_SLOT + SLOT_PROCESS_52C, |e, a| {
            e.mem.u32(a[0] + 0x118).into_ret()
        });
        e.register(PROCESS_SLOT + SLOT_PROCESS_288, |_, _| Ret::default());

        // The package vtable of this module: the changed-flags slots and the
        // name.
        let mut package_slots = vec![0u32; 0x140 / 4];
        package_slots[(SLOT_FORM_SET_CHANGED / 4) as usize] = PACKAGE_SLOT + SLOT_FORM_SET_CHANGED;
        package_slots[(SLOT_FORM_CLEAR_CHANGED / 4) as usize] =
            PACKAGE_SLOT + SLOT_FORM_CLEAR_CHANGED;
        package_slots[(SLOT_FORM_NAME / 4) as usize] = PACKAGE_SLOT + SLOT_FORM_NAME;
        e.put_vtable(PACKAGE_VTABLE_B, &package_slots);
        for slot in [SLOT_FORM_SET_CHANGED, SLOT_FORM_CLEAR_CHANGED] {
            e.register(PACKAGE_SLOT + slot, |_, _| Ret::default());
        }
        e.register(PACKAGE_SLOT + SLOT_FORM_NAME, |_, _| {
            0x0aa0_0003u32.into_ret()
        });
        e.register(IS_DYNAMIC_FORM_ID, |e, _| e.mem.u8(SWITCH_PAGE).into_ret());

        // Callees that read the blocks.
        e.register(LOCATION_GET_REFERENCE, |e, a| {
            e.mem.u32(a[0] + 8).into_ret()
        });
        e.register(LOCATION_GET_CELL, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(LOCATION_GET_OBJECT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(LOCATION_GET_OBJECT_TYPE, |e, a| {
            e.mem.u32(a[0] + 12).into_ret()
        });
        e.register(LOCATION_GET_RADIUS, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LOCATION_RESOLVE_REFERENCE, |e, a| {
            e.mem.u32(a[0] + 8).into_ret()
        });
        e.register(LOCATION_TEST_SUBJECT, |_, _| Ret::default());
        e.register(LOCATION_HAS_VALUE, |e, a| {
            (e.mem.u32(a[0] + 4) != 0).into_ret()
        });
        e.register(REFERENCE_GET_WORLD_SPACE, |e, a| {
            e.mem.u32(a[0] + 0x100).into_ret()
        });
        e.register(CELL_GET_WORLD_SPACE, |e, a| {
            e.mem.u32(a[0] + 0x100).into_ret()
        });
        e.register(GET_PARENT_CELL, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
        e.register(GET_FORM_TYPE, |e, a| e.mem.u8(a[0] + 4).into_ret());
        e.register(GET_PROCESS, |e, a| e.mem.u32(a[0] + 0x68).into_ret());
        e.register(GET_LINKED_REFERENCE, |e, a| {
            e.mem.u32(a[0] + 0x44).into_ret()
        });
        e.register(GET_EXTRA_DATA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(EXTRA_GET_START_LOCATION, |_, _| 1u32.into_ret());
        e.register(EXTRA_GET_REFERENCE_POINTER, |e, a| {
            e.mem.u32(a[0]).into_ret()
        });
        e.register(EXTRA_GET_PACKAGE_EXTRA, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(START_LOCATION_WORLD, |e, a| {
            e.mem.u32(a[0] + 0x220).into_ret()
        });
        e.register(START_LOCATION_INTERIOR_CELL, |e, a| {
            e.mem.u32(a[0] + 0x224).into_ret()
        });
        e.register(START_LOCATION_COORD, |_, a| (a[0] + 0x240).into_ret());
        e.register(SET_START_LOCATION_FROM_CURRENT, |_, _| Ret::default());
        e.register(CELL_GET_FIRST_REFERENCE, |e, a| {
            e.mem.u32(a[0] + 0x228).into_ret()
        });
        e.register(CELL_GET_DATA_X, |e, a| e.mem.u32(a[0] + 0x230).into_ret());
        e.register(CELL_GET_DATA_Y, |e, a| e.mem.u32(a[0] + 0x234).into_ret());
        e.register(CELL_FLAG_1, |e, a| {
            (e.mem.u8(a[0] + 0x24) & 1 != 0).into_ret()
        });
        e.register(IS_FURNITURE, |e, a| {
            let base = e.mem.u32(a[0] + 0x20);
            (base != 0 && e.mem.u8(base + 4) == 0x27).into_ret()
        });
        e.register(MODEL_BOUND_SIZE, |e, a| e.mem.f32(a[0] + 0x21c).into_ret());
        e.register(FORM_FLAG_20, |e, a| e.mem.u8(a[0] + 0x218).into_ret());
        e.register(HAS_LINKED_DOOR, |_, _| 0u32.into_ret());
        e.register(PROCESS_FIELD_28, |e, a| e.mem.u32(a[0] + 0x28).into_ret());
        e.register(TARGET_GET_REFERENCE, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(TARGET_GET_OBJECT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(TARGET_GET_OBJECT_TYPE, |e, a| {
            e.mem.u32(a[0] + 12).into_ret()
        });
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            (value as i64 as i32).into_ret()
        });
        e.register(FLOAT_MIN, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            (if y <= x { y } else { x }).into_ret()
        });
        e.register(ABSOLUTE_VALUE, |_, a| f32::from_bits(a[0]).abs().into_ret());
        e.register(POINT_SUBTRACT, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        e.register(POINT_DIFFERENCE, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e.register(FLAT_LENGTH, |e, a| {
            let (x, y) = (e.mem.f32(a[0]), e.mem.f32(a[0] + 4));
            (x * x + y * y).sqrt().into_ret()
        });
        e.register(BOUNDS_HIGH, |e, a| {
            // The high point is the base form's bounds height triple.
            for i in 0..3 {
                let w = e.mem.u32(a[0] + 0x60 + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, w);
            }
            a[1].into_ret()
        });
        e.register(BOUNDS_LOW, |e, a| {
            for i in 0..3 {
                let w = e.mem.u32(a[0] + 0x70 + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, w);
            }
            a[1].into_ret()
        });
        e.register(CRT_FINITE, |_, a| {
            f64::from_bits(a[0] as u64 | (a[1] as u64) << 32)
                .is_finite()
                .into_ret()
        });
        e.register(CRT_SPRINTF, |_, _| 0u32.into_ret());
        e.register(REPORT_ERROR, |_, _| Ret::default());
        e.register(PASS_THROUGH, |_, a| a[0].into_ret());
        e.register(ACTOR_EYE_LEVEL, |e, a| e.mem.f32(a[0] + 0x250).into_ret());
        e.register(REFERENCE_MATCHES_FORM, |e, a| {
            (e.mem.u32(a[0] + 0x20) == a[1]).into_ret()
        });
        e.register(GET_CURRENT_PACKAGE, |e, a| {
            e.mem.u32(a[0] + 0x234).into_ret()
        });
        e.register(ACTOR_REJECTS_CANDIDATE, |e, a| {
            e.mem.u8(a[0] + 0x238).into_ret()
        });
        e.register(ADD_FOLLOWER, |_, _| Ret::default());
        e.register(ENUM_REFERENCES_CLOSE_TO_POINT, |_, _| Ret::default());
        e.register(PLAYER_CHECK_00962720, |e, a| {
            e.mem.u8(a[0] + 0x23c).into_ret()
        });
        e.register(PLAYER_COUNT_00962620, |e, a| {
            e.mem.u32(a[0] + 0x240).into_ret()
        });
        e.register(SHOW_MESSAGE, |_, _| Ret::default());
        e.register(GAME_DAY_OF_WEEK, |e, a| e.mem.u8(a[0]).into_ret());
        e.register(GET_SCHEDULE, |_, a| (a[0] + 0x38).into_ret());
        e.register(SCHEDULE_DAY_SELECTOR, |e, a| {
            (e.mem.i8(a[0] + 1) as i32).into_ret()
        });
        e.register(INTERFACE_TARGET_REFERENCE, |_, _| 0u32.into_ret());
        e
    }

    /// The player global.
    fn set_player(e: &mut Engine, player: u32) {
        e.set_global(PLAYER, player);
    }

    /// A package of `kind` with the fake vtable, a second-location data
    /// object (types 2, 3, 8, 9, 1 keep it at +4) and the locations given.
    fn package_with(
        e: &mut Engine,
        kind: i8,
        own: Option<u32>,
        second: Option<u32>,
    ) -> Ptr<TESPackage> {
        // Room for the 0xd0 bytes of the dialogue package classes.
        let p = Ptr::<TESPackage>::new(e.mem.alloc(0xd0));
        e.set(p, TESPackage::cPackType, kind);
        e.mem.set_u32(p.addr(), PACKAGE_VTABLE_B);
        if let Some(own) = own {
            e.set(p, TESPackage::pPackLoc, Ptr::new(own));
        }
        if let Some(second) = second {
            let data = e.mem.alloc(0x24);
            let offset = match kind {
                0xf | 0x1c => 0xc,
                0x10 => 8,
                _ => 4,
            };
            e.mem.set_u32(data + offset, second);
            e.set(p, TESPackage::pPackData, Ptr::new(data));
        }
        p
    }

    /// A location block: type byte at +0, radius at +4, the reference, cell
    /// or object at +8.
    fn location(e: &mut Engine, kind: u8, radius: u32, held: u32) -> u32 {
        let l = e.mem.alloc(0x10);
        e.mem.set_u8(l, kind);
        e.mem.set_u32(l + 4, radius);
        e.mem.set_u32(l + 8, held);
        l
    }

    /// A target block: type byte at +0, reference or object at +8, object
    /// type at +12, value at +8 as well (the `TARGET_GET_VALUE` double of
    /// the first batch reads +8).
    fn target(e: &mut Engine, kind: u8, held: u32) -> u32 {
        let t = e.mem.alloc(0x10);
        e.mem.set_u8(t, kind);
        e.mem.set_u32(t + 8, held);
        t
    }

    fn attach_target(e: &mut Engine, p: Ptr<TESPackage>, t: u32) {
        e.set(p, TESPackage::pPackTarg, Ptr::new(t));
    }

    /// A form block of the given type.
    fn form(e: &mut Engine, kind: u8) -> u32 {
        let f = block(e);
        e.mem.set_u8(f + 4, kind);
        f
    }

    /// A reference with a base form of the given type.
    fn reference_of(e: &mut Engine, kind: u8) -> u32 {
        let r = block(e);
        let f = form(e, kind);
        e.mem.set_u32(r + 0x20, f);
        r
    }

    // ------------------------------------------------------------------
    // Flags.

    #[test]
    fn script_package_flag() {
        let mut e = world();
        let p = package(&mut e, 0);
        assert!(!e.call(0x00674dd0, &args![p]).bool());
        e.set(p, TESPackage::iPackFlags, 0x4000);
        assert!(e.call(0x00674dd0, &args![p]).bool());
    }

    #[test]
    fn script_package_flag_setter() {
        let mut e = world();
        let p = package(&mut e, 0);
        e.set(p, TESPackage::iPackFlags, 0x1);
        e.call(0x00674e00, &args![p, 1u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x4001);
        e.call(0x00674e00, &args![p, 0u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x1);
    }

    #[test]
    fn never_to_run_flag() {
        let mut e = world();
        let p = package(&mut e, 0);
        assert!(!e.call(0x00674e40, &args![p]).bool());
        e.set(p, TESPackage::iPackFlags, 0x8000);
        assert!(e.call(0x00674e40, &args![p]).bool());
    }

    #[test]
    fn set_never_run_needs_the_matching_target_reference() {
        let mut e = world();
        let reference = block(&mut e);
        let p = package_with(&mut e, 0, None, None);
        // No target: nothing.
        e.call(0x00674e70, &args![p, reference, 1u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        // Another reference: nothing.
        let t = target(&mut e, 0, reference + 8);
        attach_target(&mut e, p, t);
        e.call(0x00674e70, &args![p, reference, 1u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        // The matching reference sets the flag and marks the form (not a
        // dynamic form ID), then clears it again.
        e.mem.set_u32(t + 8, reference);
        log_on(&mut e);
        e.call(0x00674e70, &args![p, reference, 1u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x8000);
        assert_eq!(
            calls_to(&e, PACKAGE_SLOT + SLOT_FORM_SET_CHANGED),
            [[p.addr(), 0x8000_0000]]
        );
        e.call(0x00674e70, &args![p, reference, 0u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        assert_eq!(
            calls_to(&e, PACKAGE_SLOT + SLOT_FORM_CLEAR_CHANGED),
            [[p.addr(), 0x8000_0000]]
        );
        // A dynamic form ID is not marked.
        e.mem.set_u8(SWITCH_PAGE, 1);
        log_on(&mut e);
        e.call(0x00674e70, &args![p, reference, 1u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x8000);
        assert!(calls_to(&e, PACKAGE_SLOT + SLOT_FORM_SET_CHANGED).is_empty());
    }

    #[test]
    fn flag_bit_10000_setter_marks_the_form() {
        let mut e = world();
        let p = package_with(&mut e, 0, None, None);
        log_on(&mut e);
        e.call(0x00674f30, &args![p, 1u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x1_0000);
        assert_eq!(
            calls_to(&e, PACKAGE_SLOT + SLOT_FORM_SET_CHANGED),
            [[p.addr(), 0x4000_0000]]
        );
        e.call(0x00674f30, &args![p, 0u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        assert_eq!(
            calls_to(&e, PACKAGE_SLOT + SLOT_FORM_CLEAR_CHANGED),
            [[p.addr(), 0x4000_0000]]
        );
        e.mem.set_u8(SWITCH_PAGE, 1);
        log_on(&mut e);
        e.call(0x00674f30, &args![p, 1u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x1_0000);
        assert!(calls_to(&e, PACKAGE_SLOT + SLOT_FORM_SET_CHANGED).is_empty());
    }

    // ------------------------------------------------------------------
    // The second location.

    /// A type 3 package whose second location has the type `kind` and holds
    /// `held`.
    fn second(e: &mut Engine, kind: u8, held: u32) -> Ptr<TESPackage> {
        let l = location(e, kind, 0, held);
        package_with(e, 3, None, Some(l))
    }

    /// An actor with a process that runs `p` and chose `first` and `second`
    /// for its two locations.
    fn give_process(e: &mut Engine, actor: u32, p: Ptr<TESPackage>, first: u32, second: u32) {
        let process = process_block(e);
        e.mem.set_u32(actor + 0x68, process);
        e.mem.set_u32(process + 0x100, p.addr());
        e.mem.set_u32(process + 0x104, first);
        e.mem.set_u32(process + 0x108, second);
    }

    #[test]
    fn second_location_world_by_location_type() {
        let mut e = world();
        let actor = block(&mut e);
        e.mem.set_u32(actor + 0x100, 0xa0);
        e.mem.set_u32(actor + 0x220, 0xa1);
        e.mem.set_u32(actor + 0x208, 0xa2);
        let call = |e: &mut Engine, p: Ptr<TESPackage>, actor: u32| {
            e.call(0x00674fd0, &args![p, actor]).u32()
        };
        // No second location, or type 2: the actor's start location.
        let p = package_with(&mut e, 3, None, None);
        assert_eq!(call(&mut e, p, actor), 0xa1);
        assert_eq!(call(&mut e, p, 0), 0);
        let p = second(&mut e, 2, 0);
        assert_eq!(call(&mut e, p, actor), 0xa1);
        // Type 0: the reference's world space.
        let reference = block(&mut e);
        e.mem.set_u32(reference + 0x100, 0xb0);
        let p = second(&mut e, 0, reference);
        assert_eq!(call(&mut e, p, actor), 0xb0);
        let p = second(&mut e, 0, 0);
        assert_eq!(call(&mut e, p, actor), 0);
        // Type 1: the cell's.
        let cell = block(&mut e);
        e.mem.set_u32(cell + 0x100, 0xb1);
        let p = second(&mut e, 1, cell);
        assert_eq!(call(&mut e, p, actor), 0xb1);
        let p = second(&mut e, 1, 0);
        assert_eq!(call(&mut e, p, actor), 0);
        // Type 3: the actor's own, slot 0x294.
        let p = second(&mut e, 3, 0);
        assert_eq!(call(&mut e, p, actor), 0xa2);
        assert_eq!(call(&mut e, p, 0), 0);
        // Type 6: the linked reference's.
        let linked = block(&mut e);
        e.mem.set_u32(linked + 0x100, 0xb2);
        e.mem.set_u32(actor + 0x44, linked);
        let p = second(&mut e, 6, 0);
        assert_eq!(call(&mut e, p, actor), 0xb2);
        e.mem.set_u32(actor + 0x44, 0);
        assert_eq!(call(&mut e, p, actor), 0);
        assert_eq!(call(&mut e, p, 0), 0);
    }

    #[test]
    fn second_location_world_for_the_actors_process() {
        let mut e = world();
        let actor = block(&mut e);
        e.mem.set_u32(actor + 0x100, 0xa0);
        let chosen = block(&mut e);
        e.mem.set_u32(chosen + 0x100, 0xb4);
        for kind in [4, 5] {
            let p = second(&mut e, kind, 0);
            // No process: nothing.
            assert_eq!(e.call(0x00674fd0, &args![p, actor]).u32(), 0);
            give_process(&mut e, actor, p, 0, chosen);
            assert_eq!(e.call(0x00674fd0, &args![p, actor]).u32(), 0xb4);
            // The process chose none: the actor's own world space.
            let process = e.mem.u32(actor + 0x68);
            e.mem.set_u32(process + 0x108, 0);
            assert_eq!(e.call(0x00674fd0, &args![p, actor]).u32(), 0xa0);
            // The process runs another package: nothing.
            e.mem.set_u32(process + 0x100, 0x1234);
            assert_eq!(e.call(0x00674fd0, &args![p, actor]).u32(), 0);
            e.mem.set_u32(actor + 0x68, 0);
        }
        // The player running a type 0x1c package with a stored reference.
        set_player(&mut e, actor);
        let l = location(&mut e, 4, 0, 0);
        let q = package_with(&mut e, 0x1c, None, Some(l));
        let stored = block(&mut e);
        e.mem.set_u32(stored + 0x100, 0xb5);
        e.mem.set_u32(q.addr() + 0x9c, stored);
        assert_eq!(e.call(0x00674fd0, &args![q, actor]).u32(), 0xb5);
        e.mem.set_u32(q.addr() + 0x9c, 0);
        assert_eq!(e.call(0x00674fd0, &args![q, actor]).u32(), 0);
    }

    #[test]
    fn second_location_cell_by_location_type() {
        let mut e = world();
        let actor = block(&mut e);
        e.mem.set_u32(actor + 0x40, 0xc0);
        e.mem.set_u32(actor + 0x224, 0xc1);
        e.mem.set_u32(actor + 0x20c, 0xc2);
        let call = |e: &mut Engine, p: Ptr<TESPackage>, actor: u32| {
            e.call(0x006751a0, &args![p, actor]).u32()
        };
        let p = package_with(&mut e, 3, None, None);
        assert_eq!(call(&mut e, p, actor), 0xc1);
        assert_eq!(call(&mut e, p, 0), 0);
        let p = second(&mut e, 2, 0);
        assert_eq!(call(&mut e, p, actor), 0xc1);
        // Type 0: the reference's parent cell.
        let reference = block(&mut e);
        e.mem.set_u32(reference + 0x40, 0xc3);
        let p = second(&mut e, 0, reference);
        assert_eq!(call(&mut e, p, actor), 0xc3);
        let p = second(&mut e, 0, 0);
        assert_eq!(call(&mut e, p, actor), 0);
        // Type 1: the cell itself.
        let p = second(&mut e, 1, 0xc4);
        assert_eq!(call(&mut e, p, actor), 0xc4);
        // Type 3: slot 0x298 of the actor.
        let p = second(&mut e, 3, 0);
        assert_eq!(call(&mut e, p, actor), 0xc2);
        assert_eq!(call(&mut e, p, 0), 0);
        // Types 4 and 5: the chosen reference (slot 0x514), else the actor.
        let chosen = block(&mut e);
        e.mem.set_u32(chosen + 0x40, 0xc5);
        for kind in [4, 5] {
            let p = second(&mut e, kind, 0);
            give_process(&mut e, actor, p, chosen, 0);
            assert_eq!(call(&mut e, p, actor), 0xc5);
            let process = e.mem.u32(actor + 0x68);
            e.mem.set_u32(process + 0x104, 0);
            assert_eq!(call(&mut e, p, actor), 0xc0);
            e.mem.set_u32(actor + 0x68, 0);
            assert_eq!(call(&mut e, p, actor), 0);
        }
        // The player's stored reference of a type 0x1c package.
        set_player(&mut e, actor);
        let l = location(&mut e, 5, 0, 0);
        let q = package_with(&mut e, 0x1c, None, Some(l));
        let stored = block(&mut e);
        e.mem.set_u32(stored + 0x40, 0xc6);
        e.mem.set_u32(q.addr() + 0x9c, stored);
        assert_eq!(call(&mut e, q, actor), 0xc6);
        // Type 6: the linked reference's parent cell.
        let linked = block(&mut e);
        e.mem.set_u32(linked + 0x40, 0xc7);
        e.mem.set_u32(actor + 0x44, linked);
        let p = second(&mut e, 6, 0);
        assert_eq!(call(&mut e, p, actor), 0xc7);
        e.mem.set_u32(actor + 0x44, 0);
        assert_eq!(call(&mut e, p, actor), 0);
    }

    #[test]
    fn second_location_coord_by_location_type() {
        let mut e = world();
        for (i, v) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            e.set_global(DEFAULT_POINT + 4 * i as u32, *v);
        }
        let actor = block(&mut e);
        set_point(&mut e, actor + 0x180, [1.0, 1.0, 1.0]);
        set_point(&mut e, actor + 0x190, [2.0, 2.0, 2.0]);
        set_point(&mut e, actor + 0x240, [3.0, 3.0, 3.0]);
        let out = e.mem.alloc(12);
        let call = |e: &mut Engine, p: Ptr<TESPackage>, actor: u32| -> [f32; 3] {
            let r = e.call(0x00675360, &args![p, out, actor]).u32();
            assert_eq!(r, out);
            point(e, out)
        };
        // No second location, or type 2: the actor's start point; with no
        // actor the default point.
        let p = package_with(&mut e, 3, None, None);
        assert_eq!(call(&mut e, p, actor), [3.0; 3]);
        assert_eq!(call(&mut e, p, 0), [7.0, 8.0, 9.0]);
        let p = second(&mut e, 2, 0);
        assert_eq!(call(&mut e, p, actor), [3.0; 3]);
        // Type 0: the reference's position.
        let reference = block(&mut e);
        set_point(&mut e, reference + 0x180, [4.0, 5.0, 6.0]);
        let p = second(&mut e, 0, reference);
        assert_eq!(call(&mut e, p, actor), [4.0, 5.0, 6.0]);
        let p = second(&mut e, 0, 0);
        assert_eq!(call(&mut e, p, actor), [7.0, 8.0, 9.0]);
        // Type 1: the first reference of the cell; without one, an interior
        // cell (flag bit 0) keeps the default and an exterior cell gives its
        // corner.
        let cell = block(&mut e);
        let p = second(&mut e, 1, cell);
        e.mem.set_u32(cell + 0x228, reference);
        assert_eq!(call(&mut e, p, actor), [4.0, 5.0, 6.0]);
        e.mem.set_u32(cell + 0x228, 0);
        e.mem.set_u8(cell + 0x24, 1);
        assert_eq!(call(&mut e, p, actor), [7.0, 8.0, 9.0]);
        e.mem.set_u8(cell + 0x24, 0);
        e.mem.set_u32(cell + 0x230, 2);
        e.mem.set_u32(cell + 0x234, (-3i32) as u32);
        assert_eq!(call(&mut e, p, actor), [8192.0, -12288.0, 0.0]);
        let p = second(&mut e, 1, 0);
        assert_eq!(call(&mut e, p, actor), [7.0, 8.0, 9.0]);
        // Type 3: the actor's point (slot 0x170).
        let p = second(&mut e, 3, 0);
        assert_eq!(call(&mut e, p, actor), [2.0; 3]);
        assert_eq!(call(&mut e, p, 0), [7.0, 8.0, 9.0]);
        // Types 4 and 5: the chosen reference's position, else the actor's.
        for kind in [4, 5] {
            let p = second(&mut e, kind, 0);
            give_process(&mut e, actor, p, 0, reference);
            assert_eq!(call(&mut e, p, actor), [4.0, 5.0, 6.0]);
            let process = e.mem.u32(actor + 0x68);
            e.mem.set_u32(process + 0x108, 0);
            assert_eq!(call(&mut e, p, actor), [1.0; 3]);
            e.mem.set_u32(actor + 0x68, 0);
            assert_eq!(call(&mut e, p, actor), [7.0, 8.0, 9.0]);
        }
        set_player(&mut e, actor);
        let l = location(&mut e, 4, 0, 0);
        let q = package_with(&mut e, 0x1c, None, Some(l));
        e.mem.set_u32(q.addr() + 0x9c, reference);
        assert_eq!(call(&mut e, q, actor), [4.0, 5.0, 6.0]);
        // Type 6: the linked reference's position.
        e.mem.set_u32(actor + 0x44, reference);
        let p = second(&mut e, 6, 0);
        assert_eq!(call(&mut e, p, actor), [4.0, 5.0, 6.0]);
        e.mem.set_u32(actor + 0x44, 0);
        assert_eq!(call(&mut e, p, actor), [7.0, 8.0, 9.0]);
    }

    #[test]
    fn second_location_radius() {
        let mut e = world();
        let p = package_with(&mut e, 3, None, None);
        assert_eq!(e.call(0x00675670, &args![p]).u32(), 0);
        let l = location(&mut e, 0, 15, 0);
        let p = package_with(&mut e, 3, None, Some(l));
        assert_eq!(e.call(0x00675670, &args![p]).u32(), 15);
    }

    #[test]
    fn second_location_reference() {
        let mut e = world();
        let actor = block(&mut e);
        let call = |e: &mut Engine, p: Ptr<TESPackage>, actor: u32| {
            e.call(0x006756a0, &args![p, actor]).u32()
        };
        // No second location, type 2 and type 3: the actor.
        let p = package_with(&mut e, 3, None, None);
        assert_eq!(call(&mut e, p, actor), actor);
        assert_eq!(call(&mut e, p, 0), 0);
        let p = second(&mut e, 2, 0);
        assert_eq!(call(&mut e, p, actor), actor);
        let p = second(&mut e, 3, 0);
        assert_eq!(call(&mut e, p, actor), actor);
        assert_eq!(call(&mut e, p, 0), 0);
        // Type 0: the location's reference.
        let reference = block(&mut e);
        let p = second(&mut e, 0, reference);
        assert_eq!(call(&mut e, p, actor), reference);
        let p = second(&mut e, 0, 0);
        assert_eq!(call(&mut e, p, actor), 0);
        // Types 4 and 5: the process's choice, else the actor.
        for kind in [4, 5] {
            let p = second(&mut e, kind, 0);
            assert_eq!(call(&mut e, p, actor), 0);
            give_process(&mut e, actor, p, 0, reference);
            assert_eq!(call(&mut e, p, actor), reference);
            let process = e.mem.u32(actor + 0x68);
            e.mem.set_u32(process + 0x108, 0);
            assert_eq!(call(&mut e, p, actor), actor);
            e.mem.set_u32(actor + 0x68, 0);
        }
        // Type 6: the linked reference.
        let p = second(&mut e, 6, 0);
        assert_eq!(call(&mut e, p, actor), 0);
        e.mem.set_u32(actor + 0x44, reference);
        assert_eq!(call(&mut e, p, actor), reference);
        assert_eq!(call(&mut e, p, 0), 0);
    }

    // ------------------------------------------------------------------
    // The search location and the data accessors.

    /// A type 0 package with only its own location, and a type 2 package
    /// with an own and a second one, whose locations are `own` and `other`.
    fn search_packages(e: &mut Engine, own: u32, other: u32) -> (Ptr<TESPackage>, Ptr<TESPackage>) {
        let own_only = package_with(e, 0, Some(own), None);
        let both = package_with(e, 2, Some(own), Some(other));
        (own_only, both)
    }

    #[test]
    fn search_location_cell() {
        let mut e = world();
        let own = location(&mut e, 1, 0, 0xd1);
        let other = location(&mut e, 1, 0, 0xd2);
        let (own_only, both) = search_packages(&mut e, own, other);
        let actor = block(&mut e);
        assert_eq!(e.call(0x006757e0, &args![own_only, actor]).u32(), 0xd1);
        assert_eq!(e.call(0x006757e0, &args![both, actor]).u32(), 0xd2);
    }

    #[test]
    fn search_location_coord() {
        let mut e = world();
        let first = block(&mut e);
        set_point(&mut e, first + 0x180, [1.0, 2.0, 3.0]);
        let second_reference = block(&mut e);
        set_point(&mut e, second_reference + 0x180, [4.0, 5.0, 6.0]);
        let own = location(&mut e, 0, 0, first);
        let other = location(&mut e, 0, 0, second_reference);
        let (own_only, both) = search_packages(&mut e, own, other);
        let actor = block(&mut e);
        let out = e.mem.alloc(12);
        assert_eq!(e.call(0x00675830, &args![own_only, out, actor]).u32(), out);
        assert_eq!(point(&e, out), [1.0, 2.0, 3.0]);
        assert_eq!(e.call(0x00675830, &args![both, out, actor]).u32(), out);
        assert_eq!(point(&e, out), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn search_location_radius() {
        let mut e = world();
        let own = location(&mut e, 0, 11, 0);
        let other = location(&mut e, 0, 22, 0);
        let (own_only, both) = search_packages(&mut e, own, other);
        let actor = block(&mut e);
        assert_eq!(e.call(0x006758a0, &args![own_only, actor]).i32(), 11);
        assert_eq!(e.call(0x006758a0, &args![both, actor]).i32(), 22);
    }

    fn check_data_accessor(address: u32, kind: i8) {
        let mut e = world();
        let p = package_with(&mut e, kind, None, None);
        assert_eq!(e.call(address, &args![p]).u32(), 0);
        let data = e.mem.alloc(0x24);
        e.set(p, TESPackage::pPackData, Ptr::new(data));
        assert_eq!(e.call(address, &args![p]).u32(), data);
        let other = package_with(&mut e, 4, None, None);
        e.set(other, TESPackage::pPackData, Ptr::new(data));
        assert_eq!(e.call(address, &args![other]).u32(), 0);
    }

    #[test]
    fn use_weapon_package_data() {
        check_data_accessor(0x006758f0, 0x10);
    }

    #[test]
    fn patrol_package_data() {
        check_data_accessor(0x00675920, 0xd);
    }

    #[test]
    fn follow_package_data() {
        check_data_accessor(0x00675950, 1);
    }

    #[test]
    fn package_data_of_type_0e() {
        check_data_accessor(0x00675980, 0xe);
    }

    #[test]
    fn escort_package_data() {
        check_data_accessor(0x006759b0, 2);
    }

    #[test]
    fn compute_initial_target() {
        let mut e = world();
        let actor = block(&mut e);
        let linked = block(&mut e);
        e.mem.set_u32(actor + 0x44, linked);
        let p = package_with(&mut e, 0, None, None);
        assert_eq!(e.call(0x006759e0, &args![p, actor]).u32(), 0);
        let reference = block(&mut e);
        let t = target(&mut e, 0, reference);
        attach_target(&mut e, p, t);
        assert_eq!(e.call(0x006759e0, &args![p, actor]).u32(), reference);
        e.mem.set_u8(t, 3);
        assert_eq!(e.call(0x006759e0, &args![p, actor]).u32(), linked);
        e.mem.set_u8(t, 1);
        assert_eq!(e.call(0x006759e0, &args![p, actor]).u32(), 0);
    }

    // ------------------------------------------------------------------
    // The package's own location.

    /// A type 0 package whose own location has the type `kind`, the radius
    /// `radius` and holds `held`.
    fn own(e: &mut Engine, kind: u8, held: u32) -> Ptr<TESPackage> {
        let l = location(e, kind, 0, held);
        package_with(e, 0, Some(l), None)
    }

    #[test]
    fn location_world_by_location_type() {
        let mut e = world();
        let actor = block(&mut e);
        e.mem.set_u32(actor + 0x100, 0xa0);
        e.mem.set_u32(actor + 0x220, 0xa1);
        e.mem.set_u32(actor + 0x208, 0xa2);
        let call = |e: &mut Engine, p: Ptr<TESPackage>, actor: u32| {
            e.call(0x00675a50, &args![p, actor]).u32()
        };
        // No location: the start location, set from the current location
        // first when it has no world space.
        let p = package_with(&mut e, 0, None, None);
        log_on(&mut e);
        assert_eq!(call(&mut e, p, actor), 0xa1);
        assert!(calls_to(&e, SET_START_LOCATION_FROM_CURRENT).is_empty());
        e.mem.set_u32(actor + 0x220, 0);
        assert_eq!(call(&mut e, p, actor), 0xa0);
        assert_eq!(calls_to(&e, SET_START_LOCATION_FROM_CURRENT), [[actor]]);
        assert_eq!(call(&mut e, p, 0), 0);
        let p = own(&mut e, 2, 0);
        assert_eq!(call(&mut e, p, actor), 0xa0);
        // Type 0: the reference's world space, the actor's without one.
        let reference = block(&mut e);
        e.mem.set_u32(reference + 0x100, 0xb0);
        let p = own(&mut e, 0, reference);
        assert_eq!(call(&mut e, p, actor), 0xb0);
        let p = own(&mut e, 0, 0);
        assert_eq!(call(&mut e, p, actor), 0xa0);
        assert_eq!(call(&mut e, p, 0), 0);
        // Type 1: the cell's.
        let cell = block(&mut e);
        e.mem.set_u32(cell + 0x100, 0xb1);
        let p = own(&mut e, 1, cell);
        assert_eq!(call(&mut e, p, actor), 0xb1);
        let p = own(&mut e, 1, 0);
        assert_eq!(call(&mut e, p, actor), 0);
        // Type 3: the actor's slot 0x294.
        let p = own(&mut e, 3, 0);
        assert_eq!(call(&mut e, p, actor), 0xa2);
        assert_eq!(call(&mut e, p, 0), 0);
        // Types 4 and 5: the process's choice, else the actor; no process,
        // nothing.
        let chosen = block(&mut e);
        e.mem.set_u32(chosen + 0x100, 0xb4);
        for kind in [4, 5] {
            let p = own(&mut e, kind, 0);
            assert_eq!(call(&mut e, p, actor), 0);
            give_process(&mut e, actor, p, chosen, 0);
            assert_eq!(call(&mut e, p, actor), 0xb4);
            let process = e.mem.u32(actor + 0x68);
            e.mem.set_u32(process + 0x104, 0);
            assert_eq!(call(&mut e, p, actor), 0xa0);
            e.mem.set_u32(actor + 0x68, 0);
        }
        // Type 6: the linked reference's, else the actor's.
        let linked = block(&mut e);
        e.mem.set_u32(linked + 0x100, 0xb5);
        let p = own(&mut e, 6, 0);
        assert_eq!(call(&mut e, p, actor), 0xa0);
        e.mem.set_u32(actor + 0x44, linked);
        assert_eq!(call(&mut e, p, actor), 0xb5);
        assert_eq!(call(&mut e, p, 0), 0);
    }

    #[test]
    fn location_cell_by_location_type() {
        let mut e = world();
        let actor = block(&mut e);
        e.mem.set_u32(actor + 0x40, 0xc0);
        e.mem.set_u32(actor + 0x224, 0xc1);
        e.mem.set_u32(actor + 0x20c, 0xc2);
        let call = |e: &mut Engine, p: Ptr<TESPackage>, actor: u32| {
            e.call(0x00675c20, &args![p, actor]).u32()
        };
        let p = package_with(&mut e, 0, None, None);
        log_on(&mut e);
        assert_eq!(call(&mut e, p, actor), 0xc1);
        assert!(calls_to(&e, SET_START_LOCATION_FROM_CURRENT).is_empty());
        e.mem.set_u32(actor + 0x224, 0);
        assert_eq!(call(&mut e, p, actor), 0xc0);
        assert_eq!(calls_to(&e, SET_START_LOCATION_FROM_CURRENT), [[actor]]);
        assert_eq!(call(&mut e, p, 0), 0);
        let reference = block(&mut e);
        e.mem.set_u32(reference + 0x40, 0xc3);
        let p = own(&mut e, 0, reference);
        assert_eq!(call(&mut e, p, actor), 0xc3);
        let p = own(&mut e, 0, 0);
        assert_eq!(call(&mut e, p, actor), 0xc0);
        assert_eq!(call(&mut e, p, 0), 0);
        let p = own(&mut e, 1, 0xc4);
        assert_eq!(call(&mut e, p, actor), 0xc4);
        let p = own(&mut e, 3, 0);
        assert_eq!(call(&mut e, p, actor), 0xc2);
        let chosen = block(&mut e);
        e.mem.set_u32(chosen + 0x40, 0xc5);
        for kind in [4, 5] {
            let p = own(&mut e, kind, 0);
            give_process(&mut e, actor, p, chosen, 0);
            assert_eq!(call(&mut e, p, actor), 0xc5);
            let process = e.mem.u32(actor + 0x68);
            e.mem.set_u32(process + 0x104, 0);
            assert_eq!(call(&mut e, p, actor), 0xc0);
            e.mem.set_u32(actor + 0x68, 0);
            assert_eq!(call(&mut e, p, actor), 0);
        }
        let linked = block(&mut e);
        e.mem.set_u32(linked + 0x40, 0xc7);
        let p = own(&mut e, 6, 0);
        assert_eq!(call(&mut e, p, actor), 0xc0);
        e.mem.set_u32(actor + 0x44, linked);
        assert_eq!(call(&mut e, p, actor), 0xc7);
    }

    #[test]
    fn location_coord_by_location_type() {
        let mut e = world();
        for (i, v) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            e.set_global(DEFAULT_POINT + 4 * i as u32, *v);
        }
        let actor = block(&mut e);
        set_point(&mut e, actor + 0x180, [1.0; 3]);
        set_point(&mut e, actor + 0x190, [2.0; 3]);
        set_point(&mut e, actor + 0x240, [3.0; 3]);
        let out = e.mem.alloc(12);
        let call = |e: &mut Engine, p: Ptr<TESPackage>, actor: u32| -> [f32; 3] {
            assert_eq!(e.call(0x00675de0, &args![p, out, actor]).u32(), out);
            point(e, out)
        };
        // No location: the start point; the start location is set first when
        // the extra data has none.
        let p = package_with(&mut e, 0, None, None);
        log_on(&mut e);
        assert_eq!(call(&mut e, p, actor), [3.0; 3]);
        assert!(calls_to(&e, SET_START_LOCATION_FROM_CURRENT).is_empty());
        e.register(EXTRA_GET_START_LOCATION, |_, _| 0u32.into_ret());
        assert_eq!(call(&mut e, p, actor), [3.0; 3]);
        assert_eq!(calls_to(&e, SET_START_LOCATION_FROM_CURRENT), [[actor]]);
        assert_eq!(call(&mut e, p, 0), [7.0, 8.0, 9.0]);
        // Type 0: the reference's position; without one a warning naming the
        // actor, and the actor's point.
        let reference = block(&mut e);
        set_point(&mut e, reference + 0x180, [4.0, 5.0, 6.0]);
        let p = own(&mut e, 0, reference);
        assert_eq!(call(&mut e, p, actor), [4.0, 5.0, 6.0]);
        let p = own(&mut e, 0, 0);
        log_on(&mut e);
        assert_eq!(call(&mut e, p, actor), [2.0; 3]);
        assert_eq!(
            calls_to(&e, LOG_FORM_WARNING),
            [[MESSAGE_NO_REFERENCE_LOCATION, 0x0aa0_0002]]
        );
        assert_eq!(call(&mut e, p, 0), [7.0, 8.0, 9.0]);
        // Type 1: the first reference of the cell, the corner of an exterior
        // cell, the default for an interior one.
        let cell = block(&mut e);
        let p = own(&mut e, 1, cell);
        e.mem.set_u32(cell + 0x228, reference);
        assert_eq!(call(&mut e, p, actor), [4.0, 5.0, 6.0]);
        e.mem.set_u32(cell + 0x228, 0);
        e.mem.set_u8(cell + 0x24, 1);
        assert_eq!(call(&mut e, p, actor), [7.0, 8.0, 9.0]);
        e.mem.set_u8(cell + 0x24, 0);
        e.mem.set_u32(cell + 0x230, 1);
        e.mem.set_u32(cell + 0x234, 1);
        assert_eq!(call(&mut e, p, actor), [4096.0, 4096.0, 0.0]);
        // Type 3: the actor's point.
        let p = own(&mut e, 3, 0);
        assert_eq!(call(&mut e, p, actor), [2.0; 3]);
        // Types 4 and 5.
        for kind in [4, 5] {
            let p = own(&mut e, kind, 0);
            give_process(&mut e, actor, p, reference, 0);
            assert_eq!(call(&mut e, p, actor), [4.0, 5.0, 6.0]);
            let process = e.mem.u32(actor + 0x68);
            e.mem.set_u32(process + 0x104, 0);
            assert_eq!(call(&mut e, p, actor), [1.0; 3]);
            e.mem.set_u32(actor + 0x68, 0);
            assert_eq!(call(&mut e, p, actor), [7.0, 8.0, 9.0]);
        }
        // Type 6: the linked reference's position; none gives a warning and
        // the actor's point.
        let p = own(&mut e, 6, 0);
        log_on(&mut e);
        assert_eq!(call(&mut e, p, actor), [2.0; 3]);
        assert_eq!(
            calls_to(&e, LOG_FORM_WARNING),
            [[MESSAGE_NO_LINKED_REFERENCE, 0x0aa0_0002]]
        );
        e.mem.set_u32(actor + 0x44, reference);
        assert_eq!(call(&mut e, p, actor), [4.0, 5.0, 6.0]);
        assert_eq!(call(&mut e, p, 0), [7.0, 8.0, 9.0]);
    }

    #[test]
    fn location_reference() {
        let mut e = world();
        let actor = block(&mut e);
        let call = |e: &mut Engine, p: Ptr<TESPackage>, actor: u32| {
            e.call(0x00676140, &args![p, actor]).u32()
        };
        let p = package_with(&mut e, 0, None, None);
        assert_eq!(call(&mut e, p, actor), actor);
        assert_eq!(call(&mut e, p, 0), 0);
        let p = own(&mut e, 2, 0);
        assert_eq!(call(&mut e, p, actor), actor);
        let p = own(&mut e, 3, 0);
        assert_eq!(call(&mut e, p, actor), actor);
        assert_eq!(call(&mut e, p, 0), 0);
        let reference = block(&mut e);
        let p = own(&mut e, 0, reference);
        assert_eq!(call(&mut e, p, actor), reference);
        let p = own(&mut e, 0, 0);
        assert_eq!(call(&mut e, p, actor), 0);
        for kind in [4, 5] {
            let p = own(&mut e, kind, 0);
            assert_eq!(call(&mut e, p, actor), 0);
            give_process(&mut e, actor, p, reference, 0);
            assert_eq!(call(&mut e, p, actor), reference);
            let process = e.mem.u32(actor + 0x68);
            e.mem.set_u32(process + 0x104, 0);
            assert_eq!(call(&mut e, p, actor), actor);
            e.mem.set_u32(actor + 0x68, 0);
        }
        let p = own(&mut e, 6, 0);
        assert_eq!(call(&mut e, p, actor), 0);
        e.mem.set_u32(actor + 0x44, reference);
        assert_eq!(call(&mut e, p, actor), reference);
        assert_eq!(call(&mut e, p, 0), 0);
    }

    #[test]
    fn location_radius_of_the_own_location() {
        let mut e = world();
        let actor = block(&mut e);
        let call = |e: &mut Engine, p: Ptr<TESPackage>| e.call(0x00676280, &args![p, actor]).i32();
        // No location: 0.
        let p = package_with(&mut e, 0, None, None);
        assert_eq!(call(&mut e, p), 0);
        // The location's own radius, with no reference or another base form.
        let l = location(&mut e, 0, 12, 0);
        let p = package_with(&mut e, 0, Some(l), None);
        assert_eq!(call(&mut e, p), 12);
        let other = reference_of(&mut e, 0x1);
        let l = location(&mut e, 0, 13, other);
        let p = package_with(&mut e, 0, Some(l), None);
        assert_eq!(call(&mut e, p), 13);
        // A reference with a base form of type 0x15 and no radius of its own:
        // half the smallest extent of the base form, truncated.
        let big = reference_of(&mut e, 0x15);
        let base = e.mem.u32(big + 0x20);
        set_point(&mut e, base + 0x60, [20.0, 16.0, 18.0]);
        set_point(&mut e, base + 0x70, [10.0, 10.0, 10.0]);
        let l = location(&mut e, 0, 0, big);
        let p = package_with(&mut e, 0, Some(l), None);
        assert_eq!(call(&mut e, p), 3);
        set_point(&mut e, base + 0x60, [20.0, 17.0, 18.0]);
        assert_eq!(call(&mut e, p), 3);
        // With a radius of its own the location keeps it.
        let l = location(&mut e, 0, 9, big);
        let p = package_with(&mut e, 0, Some(l), None);
        assert_eq!(call(&mut e, p), 9);
    }

    // ------------------------------------------------------------------
    // "Is the subject at the location".

    /// A subject, an actor, a reference at (10, 0, 0) and the world space
    /// they all share.
    fn arrival(e: &mut Engine) -> (u32, u32, u32) {
        let subject = reference_of(e, 0x1);
        e.mem.set_u32(subject + 0x100, 0x10);
        let actor = reference_of(e, 0x1);
        e.mem.set_u32(actor + 0x100, 0x10);
        let reference = reference_of(e, 0x1);
        e.mem.set_u32(reference + 0x100, 0x10);
        set_point(e, reference + 0x180, [10.0, 0.0, 0.0]);
        (subject, actor, reference)
    }

    fn reach_calls(e: &Engine, subject: u32) -> Vec<Vec<u32>> {
        calls_to(e, REFERENCE_SLOT + SLOT_REACH)
            .into_iter()
            .filter(|c| c[0] == subject)
            .collect()
    }

    /// The reach call's radius and its two flags.
    fn reach_arguments(e: &Engine, subject: u32) -> (f32, u32, u32) {
        let calls = reach_calls(e, subject);
        let last = calls.last().expect("the reach slot was called");
        (f32::from_bits(last[2]), last[3], last[4])
    }

    #[test]
    fn at_second_location_simple_answers() {
        let mut e = world();
        let (subject, actor, reference) = arrival(&mut e);
        let call = |e: &mut Engine, p: Ptr<TESPackage>, subject: u32| {
            e.call(0x00676390, &args![p, subject, actor, 0u32, -1.0f32, 0u32])
                .bool()
        };
        // No subject: false.
        let p = second(&mut e, 0, reference);
        assert!(!call(&mut e, p, 0));
        // No second location: true.
        let p = package_with(&mut e, 3, None, None);
        assert!(call(&mut e, p, subject));
        // A location of type 3 when the actor's slot 0x290 says no: true.
        let p = second(&mut e, 3, 0);
        assert!(call(&mut e, p, subject));
        // Another world space: false; a type 1 location in the cell: true.
        e.mem.set_u8(actor + 0x204, 1);
        e.mem.set_u32(actor + 0x208, 0x99);
        assert!(!call(&mut e, p, subject));
        let cell = block(&mut e);
        e.mem.set_u8(cell + 0x24, 1);
        e.mem.set_u32(subject + 0x40, cell);
        let p = second(&mut e, 1, cell);
        assert!(call(&mut e, p, subject));
        // The same cell, but another type of location: the test goes on.
        let p = second(&mut e, 0, reference);
        e.mem.set_u32(reference + 0x40, cell);
        e.mem.set_u8(subject + 0x210, 1);
        assert!(call(&mut e, p, subject));
        // The location's own test decides: the byte is the answer.
        e.register(LOCATION_TEST_SUBJECT, |e, a| {
            e.mem.set_u8(a[3], 1);
            true.into_ret()
        });
        log_on(&mut e);
        assert!(call(&mut e, p, subject));
        assert!(reach_calls(&e, subject).is_empty());
        e.register(LOCATION_TEST_SUBJECT, |e, a| {
            e.mem.set_u8(a[3], 0);
            true.into_ret()
        });
        assert!(!call(&mut e, p, subject));
    }

    #[test]
    fn at_second_location_radius_and_reach() {
        let mut e = world();
        let (subject, actor, reference) = arrival(&mut e);
        e.mem.set_u8(subject + 0x210, 1);
        let run = |e: &mut Engine, p: Ptr<TESPackage>, ignore: u32, extra: f32| {
            log_on(e);
            e.call(0x00676390, &args![p, subject, actor, ignore, extra, 0u32])
                .bool()
        };
        // A reference that is not an actor: the model bound plus 20.
        e.mem.set_f32(reference + 0x21c, 5.0);
        let p = second(&mut e, 0, reference);
        assert!(run(&mut e, p, 0, -1.0));
        let (radius, inverted, last) = reach_arguments(&e, subject);
        assert_eq!((radius, inverted, last), (25.0, 1, 1));
        // The point passed is the second location's.
        assert_eq!(point(&e, subject + 0x300), [10.0, 0.0, 0.0]);
        // An extra radius replaces it; -1.0 does not.
        assert!(run(&mut e, p, 0, 3.5));
        assert_eq!(reach_arguments(&e, subject).0, 3.0);
        // Ignoring the extra radius starts from 0.
        let l = location(&mut e, 0, 7, reference);
        let p7 = package_with(&mut e, 3, None, Some(l));
        assert!(run(&mut e, p7, 0, -1.0));
        assert_eq!(reach_arguments(&e, subject).0, 7.0);
        assert!(run(&mut e, p7, 1, -1.0));
        assert_eq!(reach_arguments(&e, subject).0, 25.0);
        // An actor of kind 9: 90, exact.
        let hero = reference_of(&mut e, 0x2a);
        e.mem.set_u32(hero + 0x100, 0x10);
        e.mem.set_u8(hero + 0x1f0, 1);
        e.mem.set_u32(hero + 0x1f4, 9);
        let p = second(&mut e, 0, hero);
        assert!(run(&mut e, p, 0, -1.0));
        assert_eq!(reach_arguments(&e, subject), (90.0, 0, 1));
        // Another actor: its bound plus 20.
        e.mem.set_u32(hero + 0x1f4, 3);
        e.mem.set_f32(hero + 0x21c, 2.0);
        assert!(run(&mut e, p, 0, -1.0));
        assert_eq!(reach_arguments(&e, subject), (22.0, 1, 1));
        // A marker base form: 20, exact.
        let marker = reference_of(&mut e, 0x1);
        e.mem.set_u32(marker + 0x100, 0x10);
        let base = e.mem.u32(marker + 0x20);
        e.set_global(MARKER_FORM_A, base);
        let p = second(&mut e, 0, marker);
        assert!(run(&mut e, p, 0, -1.0));
        assert_eq!(reach_arguments(&e, subject), (20.0, 0, 1));
        e.set_global(MARKER_FORM_A, 0u32);
        // A location without anything (no world space either): the default
        // radius setting.
        e.mem.set_u32(subject + 0x100, 0);
        let p = second(&mut e, 0, 0);
        assert!(run(&mut e, p, 0, -1.0));
        assert_eq!(reach_arguments(&e, subject).0, 77.0);
        // The slot's answer is the result.
        e.mem.set_u8(subject + 0x210, 0);
        assert!(!run(&mut e, p, 0, -1.0));
    }

    /// Gives a reference a process whose point (slot 0x4d4) is `point`.
    fn give_point_process(e: &mut Engine, reference: u32, point: [f32; 3]) -> u32 {
        let process = process_block(e);
        let held = e.mem.alloc(12);
        set_point(e, held, point);
        e.mem.set_u32(process + 0x10c, held);
        e.mem.set_u32(reference + 0x68, process);
        process
    }

    #[test]
    fn at_second_location_height_and_furniture() {
        let mut e = world();
        let (subject, actor, reference) = arrival(&mut e);
        e.mem.set_u8(subject + 0x210, 1);
        // A type 5 package in an interior cell: more than 100 apart in
        // height is false.
        let cell = block(&mut e);
        e.mem.set_u8(cell + 0x24, 1);
        e.mem.set_u32(subject + 0x40, cell);
        e.mem.set_u32(reference + 0x40, cell);
        // A type 5 package has no second location (only types 1, 2, 3, 8, 9,
        // 0x10, 0xf and 0x1c do), so the check can only run if the type read
        // changes under the function: the double reports 5 at the 19th read
        // (the 18 before it are the six reads of the second location by their
        // three each) and 3 at every other.
        let l = location(&mut e, 0, 5, reference);
        let p = package_with(&mut e, 3, None, Some(l));
        let reads = Rc::new(Cell::new(0u32));
        let seen = reads.clone();
        e.register_double(GET_PACK_TYPE, move |_, _| {
            seen.set(seen.get() + 1);
            (if seen.get() == 19 { 5i32 } else { 3 }).into_ret()
        });
        set_point(&mut e, subject + 0x180, [0.0, 0.0, 50.0]);
        assert!(e
            .call(0x00676390, &args![p, subject, actor, 0u32, -1.0f32, 0u32])
            .bool());
        reads.set(0);
        set_point(&mut e, subject + 0x180, [0.0, 0.0, 150.0]);
        assert!(!e
            .call(0x00676390, &args![p, subject, actor, 0u32, -1.0f32, 0u32])
            .bool());
        // The package type back to what the field says.
        e.register(GET_PACK_TYPE, |e, a| {
            (e.mem.i8(a[0] + 0x20) as i32).into_ret()
        });
        // Furniture whose process holds a point.
        e.mem.set_u32(subject + 0x40, 0);
        set_point(&mut e, subject + 0x180, [0.0; 3]);
        let furniture = reference_of(&mut e, 0x27);
        e.mem.set_u32(furniture + 0x100, 0x10);
        set_point(&mut e, furniture + 0x180, [10.0, 0.0, 0.0]);
        let l = location(&mut e, 0, 0, furniture);
        let q = package_with(&mut e, 3, None, Some(l));
        let process = give_point_process(&mut e, subject, [1.0, 2.0, 3.0]);
        give_point_process(&mut e, actor, [5.0, 6.0, 7.0]);
        let run = |e: &mut Engine, flag: u32| {
            log_on(e);
            e.call(0x00676390, &args![q, subject, actor, 0u32, -1.0f32, flag])
                .bool()
        };
        // Furniture check off: the reach slot decides, with the radius 20 and
        // the actor's process point (its field at +0x28 is zero).
        assert!(run(&mut e, 0));
        assert_eq!(reach_arguments(&e, subject), (20.0, 0, 1));
        assert_eq!(point(&e, subject + 0x300), [5.0, 6.0, 7.0]);
        // Furniture check on, a point held but the subject not an actor:
        // false; an actor of kind 4 or 9 whose process names the furniture:
        // true.
        assert!(!run(&mut e, 1));
        e.mem.set_u8(subject + 0x1f0, 1);
        assert!(!run(&mut e, 1));
        e.mem.set_u32(subject + 0x1f4, 4);
        assert!(!run(&mut e, 1));
        e.mem.set_u32(process + 0x110, furniture);
        assert!(run(&mut e, 1));
        e.mem.set_u32(subject + 0x1f4, 9);
        assert!(run(&mut e, 1));
        // The process holds no point: the reach slot again.
        e.mem.set_u32(process + 0x10c, 0);
        assert!(run(&mut e, 1));
    }

    #[test]
    fn at_own_location_simple_answers() {
        let mut e = world();
        let (subject, _actor, reference) = arrival(&mut e);
        let call = |e: &mut Engine, p: Ptr<TESPackage>, subject: u32| {
            e.call(0x006768d0, &args![p, subject, 0u32, -1.0f32, 0u32])
                .bool()
        };
        let p = own(&mut e, 0, reference);
        assert!(!call(&mut e, p, 0));
        // A location of type 3 when the subject's slot 0x290 says no: true.
        let p3 = own(&mut e, 3, 0);
        assert!(call(&mut e, p3, subject));
        // Another world space: false.
        e.mem.set_u8(subject + 0x204, 1);
        e.mem.set_u32(subject + 0x208, 0x99);
        assert!(!call(&mut e, p3, subject));
        // A type 1 location in the subject's cell: true.
        let cell = block(&mut e);
        e.mem.set_u8(cell + 0x24, 1);
        e.mem.set_u32(subject + 0x40, cell);
        let p1 = own(&mut e, 1, cell);
        assert!(call(&mut e, p1, subject));
        // The own test decides only for a location with radius 0.
        e.mem.set_u32(reference + 0x40, cell);
        e.mem.set_u8(subject + 0x210, 1);
        e.register(LOCATION_TEST_SUBJECT, |e, a| {
            e.mem.set_u8(a[3], 0);
            true.into_ret()
        });
        assert!(!call(&mut e, p, subject));
        let l = location(&mut e, 0, 5, reference);
        let p5 = package_with(&mut e, 0, Some(l), None);
        assert!(call(&mut e, p5, subject));
    }

    #[test]
    fn at_own_location_radius_and_reach() {
        let mut e = world();
        let (subject, _actor, reference) = arrival(&mut e);
        e.mem.set_u8(subject + 0x210, 1);
        let run = |e: &mut Engine, p: Ptr<TESPackage>, skip: u32, extra: f32, furniture: u32| {
            log_on(e);
            e.call(0x006768d0, &args![p, subject, skip, extra, furniture])
                .bool()
        };
        // A plain reference: its bound plus 20.
        e.mem.set_f32(reference + 0x21c, 5.0);
        let p = own(&mut e, 0, reference);
        assert!(run(&mut e, p, 0, -1.0, 0));
        assert_eq!(reach_arguments(&e, subject), (25.0, 1, 1));
        // The extra radius is added to the location's own radius.
        let l = location(&mut e, 0, 7, reference);
        let p7 = package_with(&mut e, 0, Some(l), None);
        assert!(run(&mut e, p7, 0, 3.5, 0));
        assert_eq!(reach_arguments(&e, subject).0, 10.0);
        assert!(run(&mut e, p7, 0, -1.0, 0));
        assert_eq!(reach_arguments(&e, subject).0, 7.0);
        // Skipping starts from 0, which asks the reference.
        assert!(run(&mut e, p7, 1, -1.0, 0));
        assert_eq!(reach_arguments(&e, subject).0, 25.0);
        // A marker base form with a small radius sets the exact flag.
        let marker = reference_of(&mut e, 0x1);
        e.mem.set_u32(marker + 0x100, 0x10);
        let base = e.mem.u32(marker + 0x20);
        e.set_global(MARKER_FORM_A, base);
        let l = location(&mut e, 0, 3, marker);
        let pm = package_with(&mut e, 0, Some(l), None);
        assert!(run(&mut e, pm, 0, -1.0, 0));
        assert_eq!(reach_arguments(&e, subject), (3.0, 0, 1));
        e.set_global(MARKER_FORM_A, 0u32);
        // An actor of kind 9: 90, exact.
        let hero = reference_of(&mut e, 0x2a);
        e.mem.set_u32(hero + 0x100, 0x10);
        e.mem.set_u8(hero + 0x1f0, 1);
        e.mem.set_u32(hero + 0x1f4, 9);
        let ph = own(&mut e, 0, hero);
        assert!(run(&mut e, ph, 0, -1.0, 0));
        assert_eq!(reach_arguments(&e, subject), (90.0, 0, 1));
        // Nothing at all (no world space either): the default radius setting.
        e.mem.set_u32(subject + 0x100, 0);
        let pn = own(&mut e, 0, 0);
        assert!(run(&mut e, pn, 0, -1.0, 0));
        assert_eq!(reach_arguments(&e, subject).0, 77.0);
    }

    #[test]
    fn at_own_location_furniture() {
        let mut e = world();
        let (subject, _actor, _reference) = arrival(&mut e);
        e.mem.set_u8(subject + 0x1f0, 1);
        let furniture = reference_of(&mut e, 0x27);
        e.mem.set_u32(furniture + 0x100, 0x10);
        set_point(&mut e, furniture + 0x180, [10.0, 0.0, 0.0]);
        let p = own(&mut e, 0, furniture);
        let process = give_point_process(&mut e, subject, [10.0, 0.0, 0.0]);
        let run = |e: &mut Engine, furniture_check: u32| {
            e.call(
                0x006768d0,
                &args![p, subject, 0u32, -1.0f32, furniture_check],
            )
            .bool()
        };
        // A kind 4 actor within 100 flat metres of the point: true, even with
        // the check off (kinds 4 and 9 turn it on).
        e.mem.set_u32(subject + 0x1f4, 4);
        assert!(run(&mut e, 0));
        // Kind 9: true; another kind: false.
        e.mem.set_u32(subject + 0x1f4, 9);
        assert!(run(&mut e, 1));
        e.mem.set_u32(subject + 0x1f4, 3);
        assert!(!run(&mut e, 1));
        // A kind 4 actor further than 100 metres from the point: false.
        e.mem.set_u32(subject + 0x1f4, 4);
        set_point(&mut e, subject + 0x180, [500.0, 0.0, 0.0]);
        assert!(!run(&mut e, 1));
        // The reach slot decides when the process holds no point.
        e.mem.set_u32(process + 0x10c, 0);
        e.mem.set_u8(subject + 0x210, 1);
        assert!(run(&mut e, 1));
    }

    // ------------------------------------------------------------------
    // "Has the actor reached its target".

    /// An actor heading for a target reference: both in the world space
    /// 0x10, the actor at the origin, the target at (10, 0, 0) with the
    /// model bound 30 and a base form of type 0x15.
    fn heading(e: &mut Engine) -> (u32, u32) {
        let actor = reference_of(e, 0x1);
        e.mem.set_u32(actor + 0x100, 0x10);
        e.mem.set_u8(actor + 0x1f0, 1);
        e.mem.set_u32(actor + 0x1f4, 1);
        let target = reference_of(e, 0x15);
        e.mem.set_u32(target + 0x100, 0x10);
        set_point(e, target + 0x180, [10.0, 0.0, 0.0]);
        e.mem.set_f32(target + 0x21c, 30.0);
        e.mem.set_u32(actor + 0x200, target);
        (actor, target)
    }

    fn reach_point(e: &Engine, actor: u32) -> [f32; 3] {
        point(e, actor + 0x300)
    }

    #[test]
    fn actor_at_ref_target_basic_answers() {
        let mut e = world();
        let (actor, target) = heading(&mut e);
        let p = package_with(&mut e, 0, None, None);
        let call = |e: &mut Engine, actor: u32, extra: i32| {
            log_on(e);
            e.call(0x00676e40, &args![p, actor, extra]).bool()
        };
        // No actor, or no target reference: false.
        assert!(!call(&mut e, 0, 0));
        e.mem.set_u32(actor + 0x200, 0);
        assert!(!call(&mut e, actor, 0));
        e.mem.set_u32(actor + 0x200, target);
        // Another world space: false.
        e.mem.set_u32(target + 0x100, 0x11);
        assert!(!call(&mut e, actor, 0));
        e.mem.set_u32(target + 0x100, 0x10);
        // The radius is the target's model bound (30) plus the extra; the
        // reach slot is asked about the target's position with the "exact"
        // flag clear and the last argument 0.
        e.mem.set_u8(actor + 0x210, 1);
        assert!(call(&mut e, actor, 5));
        let reaches = reach_calls(&e, actor);
        assert_eq!(reaches.len(), 1);
        assert_eq!(f32::from_bits(reaches[0][2]), 35.0);
        assert_eq!((reaches[0][3], reaches[0][4]), (1, 0));
        assert_eq!(reach_point(&e, actor), [10.0, 0.0, 0.0]);
        e.mem.set_u8(actor + 0x210, 0);
        assert!(!call(&mut e, actor, 0));
    }

    #[test]
    fn actor_at_ref_target_radius_cases() {
        let mut e = world();
        let (actor, target) = heading(&mut e);
        e.mem.set_u8(actor + 0x210, 1);
        // A package target whose value is 5 gives the small radius that lets
        // the actor kinds replace it.
        let p = package_with(&mut e, 0, None, None);
        let t = e.mem.alloc(0x10);
        e.mem.set_u32(t + 8, 5);
        attach_target(&mut e, p, t);
        let run = |e: &mut Engine, actor: u32| {
            log_on(e);
            e.call(0x00676e40, &args![p, actor, 0i32]).bool()
        };
        let last = |e: &Engine| {
            let c = reach_calls(e, actor);
            let last = c.last().unwrap();
            (f32::from_bits(last[2]), last[3])
        };
        // The target is an actor of kind 9: 90, exact.
        e.mem.set_u8(target + 0x1f0, 1);
        e.mem.set_u32(target + 0x1f4, 9);
        assert!(run(&mut e, actor));
        assert_eq!(last(&e), (90.0, 0));
        // Another kind, the actor of kind 4: 200, exact.
        e.mem.set_u32(target + 0x1f4, 2);
        e.mem.set_u32(actor + 0x1f4, 4);
        assert!(run(&mut e, actor));
        assert_eq!(last(&e), (200.0, 0));
        // Neither: the target's bound plus 20.
        e.mem.set_u32(actor + 0x1f4, 1);
        assert!(run(&mut e, actor));
        assert_eq!(last(&e), (50.0, 1));
        // The player as the target is treated like a non-actor.
        set_player(&mut e, target);
        assert!(run(&mut e, actor));
        assert_eq!(last(&e), (5.0, 1));
        set_player(&mut e, 0);
        // A target of form type 0x30: 10, exact.
        e.mem.set_u8(target + 0x1f0, 0);
        let base = e.mem.u32(target + 0x20);
        e.mem.set_u8(base + 4, 0x30);
        assert!(run(&mut e, actor));
        assert_eq!(last(&e), (10.0, 0));
        // A marker base form: 20, exact.
        e.mem.set_u8(base + 4, 0x15);
        e.set_global(MARKER_FORM_A, base);
        assert!(run(&mut e, actor));
        assert_eq!(last(&e), (20.0, 0));
        e.set_global(MARKER_FORM_A, 0u32);
    }

    #[test]
    fn actor_at_ref_target_furniture_and_error_report() {
        let mut e = world();
        let (actor, target) = heading(&mut e);
        let p = package_with(&mut e, 0, None, None);
        // Furniture whose process holds a point: radius 10, exact, the point
        // of the process (the field at +0x28 being zero); the second block
        // answers true for an actor of kind 4 within 100 metres.
        let base = e.mem.u32(target + 0x20);
        e.mem.set_u8(base + 4, 0x27);
        give_point_process(&mut e, actor, [3.0, 4.0, 0.0]);
        log_on(&mut e);
        assert!(!e.call(0x00676e40, &args![p, actor, 0i32]).bool());
        assert_eq!(reach_point(&e, actor), [3.0, 4.0, 0.0]);
        let reaches = reach_calls(&e, actor);
        assert_eq!(f32::from_bits(reaches[0][2]), 10.0);
        assert_eq!(reaches[0][3], 0);
        e.mem.set_u32(actor + 0x1f4, 4);
        assert!(e.call(0x00676e40, &args![p, actor, 0i32]).bool());
        e.mem.set_u32(actor + 0x1f4, 9);
        assert!(e.call(0x00676e40, &args![p, actor, 0i32]).bool());
        e.mem.set_u32(actor + 0x1f4, 1);
        // The error text is raised when the actor is the interface's target
        // and the distance is finite.
        e.register(INTERFACE_TARGET_REFERENCE, |e, _| {
            // The test keeps the interface target in a byte-sized slot.
            e.mem.u32(SWITCH_PAGE + 8).into_ret()
        });
        e.mem.set_u32(SWITCH_PAGE + 8, actor);
        log_on(&mut e);
        e.call(0x00676e40, &args![p, actor, 0i32]);
        assert_eq!(calls_to(&e, REPORT_ERROR).len(), 1);
        let printed = calls_to(&e, CRT_SPRINTF);
        assert_eq!(printed.len(), 1);
        assert_eq!(printed[0][1], MESSAGE_RADIUS_DISTANCE);
        assert_eq!(printed[0][2], 10);
        e.mem.set_u32(SWITCH_PAGE + 8, 0);
        log_on(&mut e);
        e.call(0x00676e40, &args![p, actor, 0i32]);
        assert!(calls_to(&e, REPORT_ERROR).is_empty());
    }

    #[test]
    fn target_at_location() {
        let mut e = world();
        let (actor, target) = heading(&mut e);
        let reference = reference_of(&mut e, 0x1);
        e.mem.set_u32(reference + 0x100, 0x10);
        set_point(&mut e, reference + 0x180, [10.0, 0.0, 0.0]);
        let l = location(&mut e, 0, 0, reference);
        let p = package_with(&mut e, 0, Some(l), None);
        let call = |e: &mut Engine, actor: u32, extra: i32| {
            e.call(0x006773a0, &args![p, actor, extra]).bool()
        };
        assert!(!call(&mut e, 0, 0));
        e.mem.set_u32(actor + 0x200, 0);
        assert!(!call(&mut e, actor, 0));
        e.mem.set_u32(actor + 0x200, target);
        // Another world space: false.
        e.mem.set_u32(target + 0x100, 0x11);
        assert!(!call(&mut e, actor, 0));
        e.mem.set_u32(target + 0x100, 0x10);
        // The default radius setting (77) when nothing gives one: a target
        // 70 away is within it, 90 away is not; the extra widens it.
        set_point(&mut e, target + 0x180, [80.0, 0.0, 0.0]);
        assert!(call(&mut e, actor, 0));
        set_point(&mut e, target + 0x180, [100.0, 0.0, 0.0]);
        assert!(!call(&mut e, actor, 0));
        assert!(call(&mut e, actor, 20));
        // The model bound of the location's reference (30) when it has no
        // radius: 30 away is within it, 31 is not.
        e.mem.set_f32(reference + 0x21c, 30.0);
        set_point(&mut e, target + 0x180, [40.0, 0.0, 0.0]);
        assert!(call(&mut e, actor, 0));
        set_point(&mut e, target + 0x180, [41.0, 0.0, 0.0]);
        assert!(!call(&mut e, actor, 0));
        // A radius of its own on the location.
        let l = location(&mut e, 0, 50, reference);
        let q = package_with(&mut e, 0, Some(l), None);
        set_point(&mut e, target + 0x180, [60.0, 0.0, 0.0]);
        assert!(e.call(0x006773a0, &args![q, actor, 0i32]).bool());
        set_point(&mut e, target + 0x180, [61.0, 0.0, 0.0]);
        assert!(!e.call(0x006773a0, &args![q, actor, 0i32]).bool());
        // The location's own test decides when it can.
        e.register(LOCATION_TEST_SUBJECT, |e, a| {
            e.mem.set_u8(a[3], 1);
            true.into_ret()
        });
        assert!(e.call(0x006773a0, &args![q, actor, 0i32]).bool());
    }

    #[test]
    fn target_at_second_location() {
        let mut e = world();
        let (actor, target) = heading(&mut e);
        let reference = reference_of(&mut e, 0x1);
        e.mem.set_u32(reference + 0x100, 0x10);
        set_point(&mut e, reference + 0x180, [10.0, 0.0, 0.0]);
        // The same-place test uses the own location: give the package one in
        // the same world space (type 2 has both).
        let own_location = location(&mut e, 0, 0, reference);
        let second_location = location(&mut e, 0, 0, reference);
        let p = package_with(&mut e, 2, Some(own_location), Some(second_location));
        let call = |e: &mut Engine, actor: u32, radius: f32| {
            e.call(0x00677570, &args![p, actor, radius]).bool()
        };
        e.mem.set_u32(actor + 0x200, target);
        // The target stands on the location: any radius reaches.
        assert!(call(&mut e, actor, 5.0));
        // A radius at least the distance: true; below it: false.
        set_point(&mut e, target + 0x180, [20.0, 0.0, 0.0]);
        assert!(call(&mut e, actor, 10.0));
        assert!(call(&mut e, actor, 10.5));
        assert!(!call(&mut e, actor, 9.5));
        // A negative radius is worked out: the default setting 77 without a
        // radius or reference bound; the reference's bound when it has one.
        set_point(&mut e, target + 0x180, [100.0, 0.0, 0.0]);
        assert!(!call(&mut e, actor, -1.0));
        set_point(&mut e, target + 0x180, [80.0, 0.0, 0.0]);
        assert!(call(&mut e, actor, -1.0));
        e.mem.set_f32(reference + 0x21c, 100.0);
        set_point(&mut e, target + 0x180, [100.0, 0.0, 0.0]);
        assert!(call(&mut e, actor, -1.0));
        e.mem.set_f32(reference + 0x21c, 80.0);
        assert!(!call(&mut e, actor, -1.0));
        // No target: false.
        e.mem.set_u32(actor + 0x200, 0);
        assert!(!call(&mut e, actor, 5.0));
        // The second location's own test decides when it can.
        e.mem.set_u32(actor + 0x200, target);
        e.register(LOCATION_TEST_SUBJECT, |e, a| {
            e.mem.set_u8(a[3], 0);
            true.into_ret()
        });
        assert!(!call(&mut e, actor, 1000.0));
    }

    #[test]
    fn process_slot_11c() {
        let mut e = world();
        let p = package_with(&mut e, 0, None, None);
        let actor = block(&mut e);
        assert!(!e.call(0x00677760, &args![p, 0u32]).bool());
        assert!(!e.call(0x00677760, &args![p, actor]).bool());
        let process = process_block(&mut e);
        e.mem.set_u32(actor + 0x68, process);
        assert!(!e.call(0x00677760, &args![p, actor]).bool());
        e.mem.set_u8(process + 0x114, 1);
        assert!(e.call(0x00677760, &args![p, actor]).bool());
    }

    // ------------------------------------------------------------------
    // CalculateProcedureType.

    fn procedure_of(e: &mut Engine, p: Ptr<TESPackage>, reference: u32) -> i32 {
        e.call(0x006777b0, &args![p, reference]);
        e.get(p, TESPackage::ePROCEDURE_TYPE)
    }

    #[test]
    fn procedure_type_follows_the_package_type_table() {
        let mut e = world();
        for (kind, expected) in [
            (3i8, 5),
            (4, 4),
            (5, 1),
            (6, 0),
            (7, 0x1b),
            (8, 0x1c),
            (9, 0x1e),
            (10, 0x20),
            (0xc, 0x25),
            (0xd, 0x26),
            (0xe, 0x29),
            (0xf, 10),
            (0x10, 0x2c),
            (0x12, 0xc),
            (0x14, 0xd),
            (0x15, 0xb),
            (0x16, 0x13),
            (0x17, 0x14),
            (0x18, 0xf),
            (0x1a, 0x15),
            (0x1b, 0x19),
            (0x1c, 10),
            (0x1d, 0x1f),
            (0x1e, 0x21),
            (0x1f, 0x24),
            (0x20, 0x27),
            (0x21, 0x28),
            (0x22, 0x2a),
            (0x23, 0x2b),
            (0x24, 0x2e),
        ] {
            let p = package_with(&mut e, kind, None, None);
            assert_eq!(procedure_of(&mut e, p, 0), expected, "type {kind:#x}");
        }
    }

    #[test]
    fn invalid_package_type_resets_to_package_type_6() {
        let mut e = world();
        // Types the table does not know give -1: the package is turned into
        // a type 6 package with procedure type 0 (after a message that goes
        // nowhere).
        for kind in [11i8, 0x11, 0x13, 0x19, 0x25, 0x7f, -1] {
            let p = package_with(&mut e, kind, None, None);
            log_on(&mut e);
            assert_eq!(procedure_of(&mut e, p, 0), 0, "type {kind:#x}");
            assert_eq!(e.get(p, TESPackage::cPackType), 6);
            let printed = calls_to(&e, CRT_SPRINTF);
            assert_eq!(printed.len(), 1);
            assert_eq!(printed[0][1], MESSAGE_INVALID_PACKAGE);
            assert_eq!(printed[0][2], 0x0aa0_0003);
        }
    }

    #[test]
    fn procedure_type_0() {
        let mut e = world();
        // No target: 1 with a location that has a value, else 0.
        let p = package_with(&mut e, 0, None, None);
        assert_eq!(procedure_of(&mut e, p, 0), 0);
        let l = location(&mut e, 0, 1, 0);
        let p = package_with(&mut e, 0, Some(l), None);
        assert_eq!(procedure_of(&mut e, p, 0), 1);
        let l = location(&mut e, 0, 0, 0);
        let p = package_with(&mut e, 0, Some(l), None);
        assert_eq!(procedure_of(&mut e, p, 0), 0);
        // A target that holds none of reference, object and object type
        // leaves the procedure type alone.
        let p = package_with(&mut e, 0, None, None);
        e.set(p, TESPackage::ePROCEDURE_TYPE, 0x77);
        let t = target(&mut e, 0, 0);
        attach_target(&mut e, p, t);
        assert_eq!(procedure_of(&mut e, p, 0), 0x77);
        // A reference target: the "ground" form types give 2, the 0x2a form
        // type 0x16, any other 3.
        for (form_type, expected) in [
            (0x15u8, 2),
            (0x1b, 2),
            (0x1c, 2),
            (0x20, 2),
            (0x21, 2),
            (0x25, 2),
            (0x26, 2),
            (0x27, 2),
            (0x2b, 2),
            (0x2a, 0x16),
            (0x30, 3),
            (0x16, 3),
            (0x01, 3),
        ] {
            let reference = reference_of(&mut e, form_type);
            let p = package_with(&mut e, 0, None, None);
            let t = target(&mut e, 0, reference);
            attach_target(&mut e, p, t);
            assert_eq!(
                procedure_of(&mut e, p, 0),
                expected,
                "reference {form_type:#x}"
            );
        }
        // An object target: its own form type, with 0x1a for the others.
        for (form_type, expected) in [
            (0x15u8, 2),
            (0x1c, 2),
            (0x2b, 2),
            (0x2a, 0x16),
            (0x1b, 0x1a),
            (0x30, 0x1a),
        ] {
            let object = form(&mut e, form_type);
            let p = package_with(&mut e, 0, None, None);
            let t = target(&mut e, 1, object);
            attach_target(&mut e, p, t);
            assert_eq!(
                procedure_of(&mut e, p, 0),
                expected,
                "object {form_type:#x}"
            );
        }
        // An object type target.
        for (object_type, expected) in [
            (1u32, 2),
            (6, 2),
            (10, 2),
            (0xb, 2),
            (0xf, 2),
            (0xe, 0x16),
            (5, 0x1a),
            (0x10, 0x1a),
            (0, 0x77),
        ] {
            let p = package_with(&mut e, 0, None, None);
            e.set(p, TESPackage::ePROCEDURE_TYPE, 0x77);
            let t = target(&mut e, 2, 0);
            e.mem.set_u32(t + 12, object_type);
            attach_target(&mut e, p, t);
            assert_eq!(
                procedure_of(&mut e, p, 0),
                expected,
                "object type {object_type}"
            );
        }
        // A target of kind 3 looks at the reference passed in.
        let mut table = vec![];
        for (form_type, expected) in [(0x25u8, 2), (0x2a, 0x16), (0x30, 3)] {
            table.push((reference_of(&mut e, form_type), expected));
        }
        for (reference, expected) in table {
            let p = package_with(&mut e, 0, None, None);
            let t = target(&mut e, 3, 1);
            attach_target(&mut e, p, t);
            assert_eq!(procedure_of(&mut e, p, reference), expected);
        }
        let p = package_with(&mut e, 0, None, None);
        let t = target(&mut e, 3, 1);
        attach_target(&mut e, p, t);
        assert_eq!(procedure_of(&mut e, p, 0), 3);
        // A target kind past 3 gives 3.
        let p = package_with(&mut e, 0, None, None);
        let t = target(&mut e, 7, 1);
        attach_target(&mut e, p, t);
        assert_eq!(procedure_of(&mut e, p, 0), 3);
    }

    #[test]
    fn procedure_type_1() {
        let mut e = world();
        // No target: -1, so the package becomes a type 6 package.
        let p = package_with(&mut e, 1, None, None);
        assert_eq!(procedure_of(&mut e, p, 0), 0);
        assert_eq!(e.get(p, TESPackage::cPackType), 6);
        // With a target: 7 for a second location of type 6 or one that holds
        // a reference, object or object type; 0x2d otherwise.
        let target_block = target(&mut e, 0, 0);
        for (kind, held, object_type, expected) in [
            (6u8, 0u32, 0u32, 7),
            (0, 0x100, 0, 7),
            (4, 0x100, 0, 7),
            (2, 0, 0, 0x2d),
        ] {
            let l = location(&mut e, kind, 0, held);
            e.mem.set_u32(l + 12, object_type);
            let p = package_with(&mut e, 1, None, Some(l));
            attach_target(&mut e, p, target_block);
            assert_eq!(procedure_of(&mut e, p, 0), expected, "location type {kind}");
        }
        // No second location: 0x2d.
        let p = package_with(&mut e, 1, None, None);
        attach_target(&mut e, p, target_block);
        assert_eq!(procedure_of(&mut e, p, 0), 0x2d);
        // Type 6 location, an object type held at +12 only.
        let l = location(&mut e, 1, 0, 0);
        e.mem.set_u32(l + 12, 3);
        let p = package_with(&mut e, 1, None, Some(l));
        attach_target(&mut e, p, target_block);
        assert_eq!(procedure_of(&mut e, p, 0), 7);
    }

    #[test]
    fn procedure_type_2() {
        let mut e = world();
        // No target: -1 and a type 6 package.
        let p = package_with(&mut e, 2, None, None);
        assert_eq!(procedure_of(&mut e, p, 0), 0);
        assert_eq!(e.get(p, TESPackage::cPackType), 6);
        let check = |e: &mut Engine, t: u32, reference: u32| {
            let p = package_with(e, 2, None, None);
            attach_target(e, p, t);
            procedure_of(e, p, reference)
        };
        // A reference target: 8 for a form of type 0x2a or 0x2b, else 9.
        for (form_type, expected) in [(0x2au8, 8), (0x2b, 8), (0x15, 9)] {
            let reference = reference_of(&mut e, form_type);
            let t = target(&mut e, 0, reference);
            assert_eq!(check(&mut e, t, 0), expected, "reference {form_type:#x}");
        }
        // A target reference without a reference, or without a base form: 9.
        let t = target(&mut e, 0, 0);
        assert_eq!(check(&mut e, t, 0), 9);
        let bare = block(&mut e);
        let t = target(&mut e, 0, bare);
        assert_eq!(check(&mut e, t, 0), 9);
        // An object target.
        for (form_type, expected) in [(0x2au8, 8), (0x2b, 8), (0x15, 9)] {
            let object = form(&mut e, form_type);
            let t = target(&mut e, 1, object);
            assert_eq!(check(&mut e, t, 0), expected, "object {form_type:#x}");
        }
        // An object type target: 0xe and 0xf give 8.
        for (object_type, expected) in [(0xeu32, 8), (0xf, 8), (0xd, 9), (0x10, 9)] {
            let t = target(&mut e, 2, 0);
            e.mem.set_u32(t + 12, object_type);
            assert_eq!(check(&mut e, t, 0), expected, "object type {object_type}");
        }
        // Kind 3 looks at the reference passed in.
        let t = target(&mut e, 3, 1);
        let reference = reference_of(&mut e, 0x2b);
        assert_eq!(check(&mut e, t, reference), 8);
        let reference = reference_of(&mut e, 0x2a);
        assert_eq!(check(&mut e, t, reference), 8);
        let reference = reference_of(&mut e, 0x15);
        assert_eq!(check(&mut e, t, reference), 9);
        assert_eq!(check(&mut e, t, 0), 9);
        let bare = block(&mut e);
        assert_eq!(check(&mut e, t, bare), 9);
        // A kind past 3: 9.
        let t = target(&mut e, 5, 1);
        assert_eq!(check(&mut e, t, 0), 9);
    }

    // ------------------------------------------------------------------
    // The follow search and its callback.

    /// A player block that is an actor.
    fn make_player(e: &mut Engine) -> u32 {
        let player = reference_of(e, 0x2a);
        e.mem.set_u8(player + 0x1f0, 1);
        set_player(e, player);
        player
    }

    /// Records the search request of `fn_006780e0` at `SWITCH_PAGE + 0x40`
    /// (all eight words) and the context at `+0x80`, then offers the
    /// candidate kept at `SWITCH_PAGE + 0x10` to the callback.
    fn install_search(e: &mut Engine) {
        e.register(ENUM_REFERENCES_CLOSE_TO_POINT, |e, a| {
            for (i, word) in a.iter().enumerate() {
                e.mem.set_u32(SWITCH_PAGE + 0x40 + 4 * i as u32, *word);
            }
            for i in 0..3 {
                let w = e.mem.u32(a[7] + 4 * i);
                e.mem.set_u32(SWITCH_PAGE + 0x80 + 4 * i, w);
            }
            let candidate = e.mem.u32(SWITCH_PAGE + 0x10);
            if candidate != 0 {
                e.call(a[6], &[candidate, a[7]]);
            }
            Ret::default()
        });
    }

    fn follow_package(e: &mut Engine, kind: i8, target_kind: u8, held: u32) -> Ptr<TESPackage> {
        let p = package_with(e, kind, None, None);
        let t = target(e, target_kind, held);
        attach_target(e, p, t);
        p
    }

    #[test]
    fn follow_target_reference_and_followers() {
        let mut e = world();
        let actor = reference_of(&mut e, 0x2a);
        e.mem.set_u8(actor + 0x1f0, 1);
        let other = reference_of(&mut e, 0x2a);
        e.mem.set_u8(other + 0x1f0, 1);
        let call = |e: &mut Engine, p: Ptr<TESPackage>| {
            log_on(e);
            e.call(0x006780e0, &args![p, actor, 0u32]).u32()
        };
        // No target: null.
        let p = package_with(&mut e, 1, None, None);
        assert_eq!(call(&mut e, p), 0);
        // A reference target that is an actor: it is added as a follower of
        // the actor by packages of type 1 and 7, and the actor is added to it
        // by a type 2 package.
        for kind in [1, 7] {
            let p = follow_package(&mut e, kind, 0, other);
            assert_eq!(call(&mut e, p), other);
            assert_eq!(calls_to(&e, ADD_FOLLOWER), [[other, actor]]);
        }
        let p = follow_package(&mut e, 2, 0, other);
        assert_eq!(call(&mut e, p), other);
        assert_eq!(calls_to(&e, ADD_FOLLOWER), [[actor, other]]);
        // Another type: none. A package that is "created": none.
        let p = follow_package(&mut e, 3, 0, other);
        assert_eq!(call(&mut e, p), other);
        assert!(calls_to(&e, ADD_FOLLOWER).is_empty());
        let p = follow_package(&mut e, 1, 0, other);
        e.set(p, TESPackage::iPackFlags, 0x800);
        assert_eq!(call(&mut e, p), other);
        assert!(calls_to(&e, ADD_FOLLOWER).is_empty());
        // A target that is not an actor: nothing added.
        let thing = reference_of(&mut e, 0x15);
        let p = follow_package(&mut e, 1, 0, thing);
        assert_eq!(call(&mut e, p), thing);
        assert!(calls_to(&e, ADD_FOLLOWER).is_empty());
        // The persistent refinement: flag 0x20 and a pointer extra.
        let replaced = reference_of(&mut e, 0x2a);
        e.mem.set_u32(thing + 0x44, replaced);
        e.mem.set_u8(thing + 0x218, 1);
        assert_eq!(call(&mut e, p), replaced);
        // The linked reference of the actor for a target of kind 3.
        e.mem.set_u32(actor + 0x44, other);
        let p = follow_package(&mut e, 1, 3, 1);
        assert_eq!(call(&mut e, p), other);
    }

    #[test]
    fn follow_target_the_player() {
        let mut e = world();
        let player = make_player(&mut e);
        let actor = reference_of(&mut e, 0x2a);
        e.mem.set_u8(actor + 0x1f0, 1);
        let process = process_block(&mut e);
        e.mem.set_u32(actor + 0x68, process);
        let p = follow_package(&mut e, 1, 0, player);
        let call = |e: &mut Engine| {
            log_on(e);
            e.call(0x006780e0, &args![p, actor, 0u32]).u32()
        };
        // The actor's process points at the player: the player is added.
        e.mem.set_u32(process + 0x118, player);
        assert_eq!(call(&mut e), player);
        assert_eq!(calls_to(&e, ADD_FOLLOWER), [[player, actor]]);
        // Otherwise, when 00962720 holds, the player is added as well.
        e.mem.set_u32(process + 0x118, 0);
        e.mem.set_u8(player + 0x23c, 1);
        assert_eq!(call(&mut e), player);
        assert_eq!(calls_to(&e, ADD_FOLLOWER), [[player, actor]]);
        // Or while the player's count (00962620) is within the setting (2).
        e.mem.set_u8(player + 0x23c, 0);
        e.mem.set_u32(player + 0x240, 2);
        assert_eq!(call(&mut e), player);
        assert_eq!(calls_to(&e, ADD_FOLLOWER), [[player, actor]]);
        // Above it the actor's process is told and the message shown, and
        // nobody is added.
        e.mem.set_u32(player + 0x240, 3);
        assert_eq!(call(&mut e), player);
        assert!(calls_to(&e, ADD_FOLLOWER).is_empty());
        assert_eq!(
            calls_to(&e, PROCESS_SLOT + SLOT_PROCESS_288),
            [[process, actor, 0xffff_ffff]]
        );
        let shown = calls_to(&e, SHOW_MESSAGE);
        assert_eq!(shown.len(), 1);
        assert_eq!(shown[0][0], MESSAGE_SETTING + 4);
        assert_eq!(f32::from_bits(shown[0][4]), 3.0);
        // A type 2 package never adds the player.
        let q = follow_package(&mut e, 2, 0, player);
        log_on(&mut e);
        assert_eq!(e.call(0x006780e0, &args![q, actor, 0u32]).u32(), player);
        assert!(calls_to(&e, ADD_FOLLOWER).is_empty());
    }

    #[test]
    fn follow_target_search_by_object() {
        let mut e = world();
        install_search(&mut e);
        let actor = reference_of(&mut e, 0x2a);
        e.mem.set_u8(actor + 0x1f0, 1);
        set_point(&mut e, actor + 0x180, [1.0, 2.0, 3.0]);
        let cell = block(&mut e);
        e.mem.set_u32(actor + 0x40, cell);
        // An object that answers slot 0xe4.
        let object = block(&mut e);
        e.mem.set_u8(object + 0x1fc, 1);
        let p = follow_package(&mut e, 3, 1, object);
        // The player's base form is the object: the player.
        let player = make_player(&mut e);
        e.mem.set_u32(player + 0x20, object);
        e.set_global(FOUND_REFERENCE, 0x55u32);
        assert_eq!(e.call(0x006780e0, &args![p, actor, 0u32]).u32(), player);
        assert_eq!(e.global::<u32>(FOUND_REFERENCE), 0x55);
        // Another base form: the search runs, with the request described in
        // the module notes; the callback's pick is returned and the global
        // cleared.
        let other_form = block(&mut e);
        e.mem.set_u32(player + 0x20, other_form);
        let candidate = reference_of(&mut e, 0x2a);
        e.mem.set_u32(candidate + 0x20, object);
        e.mem.set_u32(SWITCH_PAGE + 0x10, candidate);
        assert_eq!(e.call(0x006780e0, &args![p, actor, 1u32]).u32(), candidate);
        assert_eq!(e.global::<u32>(FOUND_REFERENCE), 0);
        let request: Vec<u32> = (0..8)
            .map(|i| e.mem.u32(SWITCH_PAGE + 0x40 + 4 * i))
            .collect();
        assert_eq!(request[0], e.global::<u32>(DATA_HANDLER));
        assert_eq!(request[1], cell);
        assert_eq!(request[2], actor + 0x180);
        assert_eq!(f32::from_bits(request[3]), 500.0);
        assert_eq!(request[4], actor + 0x180);
        assert_eq!(f32::from_bits(request[5]), 500.0);
        assert_eq!(request[6], CALLBACK_FOLLOW_CANDIDATE);
        let context: Vec<u32> = (0..3)
            .map(|i| e.mem.u32(SWITCH_PAGE + 0x80 + 4 * i))
            .collect();
        assert_eq!(context[0], object);
        assert_eq!(context[1], actor);
        assert_eq!(context[2] & 0xff, 1);
        // A candidate the callback rejects: nobody.
        e.mem.set_u32(candidate + 0x20, 0x1234);
        assert_eq!(e.call(0x006780e0, &args![p, actor, 1u32]).u32(), 0);
        // An object that does not answer slot 0xe4 passes a null form.
        e.mem.set_u32(SWITCH_PAGE + 0x10, 0);
        e.mem.set_u8(object + 0x1fc, 0);
        assert_eq!(e.call(0x006780e0, &args![p, actor, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(SWITCH_PAGE + 0x80), 0);
        // A null object too.
        let p = follow_package(&mut e, 3, 1, 0);
        assert_eq!(e.call(0x006780e0, &args![p, actor, 0u32]).u32(), 0);
    }

    #[test]
    fn follow_target_search_by_object_type() {
        let mut e = world();
        install_search(&mut e);
        let actor = reference_of(&mut e, 0x2a);
        e.mem.set_u8(actor + 0x1f0, 1);
        let p = follow_package(&mut e, 3, 2, 0);
        assert_eq!(e.call(0x006780e0, &args![p, actor, 1u32]).u32(), 0);
        // The context's first word is never written by the game.
        assert_eq!(e.mem.u32(SWITCH_PAGE + 0x80), 0);
        assert_eq!(e.mem.u32(SWITCH_PAGE + 0x84), actor);
        assert_eq!(e.mem.u32(SWITCH_PAGE + 0x88) & 0xff, 1);
        assert_eq!(e.mem.u32(SWITCH_PAGE + 0x58), CALLBACK_FOLLOW_CANDIDATE);
    }

    #[test]
    fn follow_candidate_callback() {
        let mut e = world();
        let form_block = block(&mut e);
        e.mem.set_u8(form_block + 0x1fc, 1);
        let searcher = reference_of(&mut e, 0x2a);
        e.mem.set_u8(searcher + 0x1f0, 1);
        let context = e.mem.alloc(12);
        e.mem.set_u32(context, form_block);
        e.mem.set_u32(context + 4, searcher);
        let candidate = reference_of(&mut e, 0x2a);
        e.mem.set_u32(candidate + 0x20, form_block);
        let call = |e: &mut Engine, candidate: u32, context: u32| {
            e.set_global(FOUND_REFERENCE, 0u32);
            e.call(0x006784f0, &args![candidate, context]).bool()
        };
        // No context: false.
        assert!(!call(&mut e, candidate, 0));
        // A form that does not answer slot 0xe4, a reference that is not of
        // the form, a reference with form flag 0x20: false.
        e.mem.set_u8(form_block + 0x1fc, 0);
        assert!(!call(&mut e, candidate, context));
        e.mem.set_u8(form_block + 0x1fc, 1);
        e.mem.set_u32(candidate + 0x20, 0x1234);
        assert!(!call(&mut e, candidate, context));
        e.mem.set_u32(candidate + 0x20, form_block);
        e.mem.set_u8(candidate + 0x218, 1);
        assert!(!call(&mut e, candidate, context));
        e.mem.set_u8(candidate + 0x218, 0);
        // An actor candidate: not the searcher, not rejected by slot 0x22c.
        e.mem.set_u8(candidate + 0x1f0, 1);
        assert!(call(&mut e, candidate, context));
        assert_eq!(e.global::<u32>(FOUND_REFERENCE), candidate);
        e.mem.set_u32(context + 4, candidate);
        assert!(!call(&mut e, candidate, context));
        e.mem.set_u32(context + 4, searcher);
        e.mem.set_u8(candidate + 0x1f8, 1);
        assert!(!call(&mut e, candidate, context));
        e.mem.set_u8(candidate + 0x1f8, 0);
        // With the flag set: not in a package of type 1, not rejected by
        // 008bc7d0.
        e.mem.set_u8(context + 8, 1);
        assert!(call(&mut e, candidate, context));
        let running = package_with(&mut e, 1, None, None);
        e.mem.set_u32(candidate + 0x234, running.addr());
        assert!(!call(&mut e, candidate, context));
        let other = package_with(&mut e, 2, None, None);
        e.mem.set_u32(candidate + 0x234, other.addr());
        assert!(call(&mut e, candidate, context));
        e.mem.set_u8(searcher + 0x238, 1);
        assert!(!call(&mut e, candidate, context));
        e.mem.set_u8(searcher + 0x238, 0);
        // A candidate that is not an actor is accepted without those tests.
        e.mem.set_u8(candidate + 0x1f0, 0);
        e.mem.set_u8(searcher + 0x238, 1);
        assert!(call(&mut e, candidate, context));
    }

    // ------------------------------------------------------------------
    // Small getters and radii.

    #[test]
    fn escort_follow_distance() {
        let mut e = world();
        let p = package_with(&mut e, 3, None, None);
        assert_eq!(e.call(0x006784c0, &args![p]).u32(), 0x100);
        let q = package_with(&mut e, 2, None, None);
        let data = e.mem.alloc(0x24);
        e.mem.set_u32(data + 8, 321);
        e.set(q, TESPackage::pPackData, Ptr::new(data));
        assert_eq!(e.call(0x006784c0, &args![q]).u32(), 321);
    }

    #[test]
    fn interrupt_packages() {
        let mut e = world();
        let expected: Vec<i8> = (0x12..=0x19)
            .chain(0x1b..=0x1d)
            .chain(0x1f..=0x24)
            .collect();
        for kind in -1i8..=0x40 {
            let p = package_with(&mut e, kind, None, None);
            assert_eq!(
                e.call(0x00678610, &args![p]).bool(),
                expected.contains(&kind),
                "type {kind:#x}"
            );
        }
    }

    #[test]
    fn radius_actor_to_location() {
        let mut e = world();
        let actor = block(&mut e);
        let call = |e: &mut Engine, p: Ptr<TESPackage>, second: u32| {
            e.call(0x00678670, &args![p, actor, second]).f32()
        };
        // No location: 0.
        let p = package_with(&mut e, 2, None, None);
        assert_eq!(call(&mut e, p, 0), 0.0);
        assert_eq!(call(&mut e, p, 1), 0.0);
        // The location's own radius; the second location when asked.
        let own_location = location(&mut e, 0, 12, 0);
        let second_location = location(&mut e, 0, 34, 0);
        let p = package_with(&mut e, 2, Some(own_location), Some(second_location));
        assert_eq!(call(&mut e, p, 0), 12.0);
        assert_eq!(call(&mut e, p, 1), 34.0);
        // Without a radius: the reference the location resolves to.
        let make = |e: &mut Engine, form_type: u8, bound: f32| {
            let reference = reference_of(e, form_type);
            e.mem.set_f32(reference + 0x21c, bound);
            let l = location(e, 0, 0, reference);
            (reference, package_with(e, 0, Some(l), None))
        };
        // Furniture: 10.
        let (_, p) = make(&mut e, 0x27, 5.0);
        assert_eq!(call(&mut e, p, 0), 10.0);
        // Anything else that is no actor: its bound plus 20.
        let (_, p) = make(&mut e, 0x15, 5.0);
        assert_eq!(call(&mut e, p, 0), 25.0);
        // An actor of kind 9: 90; of another kind: its bound plus 20.
        let (reference, p) = make(&mut e, 0x2a, 5.0);
        e.mem.set_u8(reference + 0x1f0, 1);
        e.mem.set_u32(reference + 0x1f4, 9);
        assert_eq!(call(&mut e, p, 0), 90.0);
        e.mem.set_u32(reference + 0x1f4, 3);
        assert_eq!(call(&mut e, p, 0), 25.0);
        // A marker base form: 20.
        let (reference, p) = make(&mut e, 0x1, 5.0);
        let base = e.mem.u32(reference + 0x20);
        e.set_global(MARKER_FORM_A, base);
        assert_eq!(call(&mut e, p, 0), 20.0);
        e.set_global(MARKER_FORM_A, 0u32);
        // A negative result is clamped to 0, which becomes the default 77.
        let (_, p) = make(&mut e, 0x15, -100.0);
        assert_eq!(call(&mut e, p, 0), 77.0);
        // No reference: a type 3 location gives 10, another the default 77.
        let l = location(&mut e, 3, 0, 0);
        let p = package_with(&mut e, 0, Some(l), None);
        assert_eq!(call(&mut e, p, 0), 10.0);
        let l = location(&mut e, 0, 0, 0);
        let p = package_with(&mut e, 0, Some(l), None);
        assert_eq!(call(&mut e, p, 0), 77.0);
    }

    #[test]
    fn radius_actor_to_ref_target() {
        let mut e = world();
        let (actor, target_reference) = heading(&mut e);
        let p = package_with(&mut e, 0, None, None);
        let call = |e: &mut Engine, p: Ptr<TESPackage>, actor: u32| {
            e.call(0x006787e0, &args![p, actor, 0u32]).f32()
        };
        // No target reference: the default 77.
        e.mem.set_u32(actor + 0x200, 0);
        assert_eq!(call(&mut e, p, actor), 77.0);
        e.mem.set_u32(actor + 0x200, target_reference);
        // A base form of type 0x15 and a model bound of 30: 30.
        assert_eq!(call(&mut e, p, actor), 30.0);
        // The second location is looked up and not used.
        assert_eq!(e.call(0x006787e0, &args![p, actor, 1u32]).f32(), 30.0);
        // Furniture 10, a marker form 20.
        let base = e.mem.u32(target_reference + 0x20);
        e.mem.set_u8(base + 4, 0x27);
        assert_eq!(call(&mut e, p, actor), 10.0);
        e.mem.set_u8(base + 4, 0x15);
        e.set_global(MARKER_FORM_A, base);
        assert_eq!(call(&mut e, p, actor), 20.0);
        e.set_global(MARKER_FORM_A, 0u32);
        // The package target's value when the procedure type is not 0x1a.
        let t = e.mem.alloc(0x10);
        e.mem.set_u32(t + 8, 7);
        attach_target(&mut e, p, t);
        assert_eq!(call(&mut e, p, actor), 7.0);
        e.set(p, TESPackage::ePROCEDURE_TYPE, 0x1a);
        assert_eq!(call(&mut e, p, actor), 30.0);
        e.set(p, TESPackage::ePROCEDURE_TYPE, 0);
        // A type 0x1c package with a package extra on the actor that has a
        // target of its own: that target's value.
        let q = package_with(&mut e, 0x1c, None, None);
        let extra_target = e.mem.alloc(0x10);
        e.mem.set_u32(extra_target + 8, 9);
        let extra_package = package_with(&mut e, 0, None, None);
        attach_target(&mut e, extra_package, extra_target);
        e.mem.set_u32(actor + 0x44, extra_package.addr());
        assert_eq!(call(&mut e, q, actor), 9.0);
        e.mem.set_u32(actor + 0x44, 0);
        // The reach adjustment: the form type is not one of the four, the
        // actor an actor: bound 10 + reach 4; the eye line 100 + 6 - 4 = 102
        // is above the target's height 90, the difference 12 below half of
        // the actor's extent 30: plus 15.
        let plain = package_with(&mut e, 0, None, None);
        e.mem.set_u8(base + 4, 0x16);
        e.mem.set_f32(target_reference + 0x21c, 10.0);
        e.mem.set_f32(actor + 0x214, 4.0);
        e.mem.set_f32(actor + 0x250, 6.0);
        set_point(&mut e, actor + 0x180, [0.0, 0.0, 100.0]);
        set_point(&mut e, target_reference + 0x180, [10.0, 0.0, 90.0]);
        e.mem.set_f32(actor + 0x1a0, 30.0);
        assert_eq!(call(&mut e, plain, actor), 29.0);
        // An extent of 20 is not enough; a target above the eye line neither.
        e.mem.set_f32(actor + 0x1a0, 20.0);
        assert_eq!(call(&mut e, plain, actor), 14.0);
        e.mem.set_f32(actor + 0x1a0, 30.0);
        set_point(&mut e, target_reference + 0x180, [10.0, 0.0, 110.0]);
        assert_eq!(call(&mut e, plain, actor), 14.0);
        // An actor that is no actor gets the bound only (its extra data list
        // is then read at address 0x44).
        e.map(0, 0x1000);
        e.mem.set_u8(actor + 0x1f0, 0);
        assert_eq!(call(&mut e, plain, actor), 10.0);
        // A target reference that is an actor keeps the default.
        e.mem.set_u8(actor + 0x1f0, 1);
        e.mem.set_u8(target_reference + 0x1f0, 1);
        assert_eq!(call(&mut e, plain, actor), 77.0);
    }

    #[test]
    fn should_reserve_target() {
        let mut e = world();
        for procedure in -1i32..0x30 {
            let p = package_with(&mut e, 0, None, None);
            e.set(p, TESPackage::ePROCEDURE_TYPE, procedure);
            assert_eq!(
                e.call(0x00678d00, &args![p]).bool(),
                (2..=5).contains(&procedure) || procedure == 0x1a,
                "procedure {procedure}"
            );
        }
    }

    #[test]
    fn schedule_day_test() {
        let mut e = world();
        let p = package_with(&mut e, 0, None, None);
        let call = |e: &mut Engine, selector: i8, day: u8, previous: u32| {
            e.mem.set_u8(p.addr() + 0x38 + 1, selector as u8);
            e.mem.set_u8(GAME_CLOCK, day);
            e.call(0x00678d40, &args![p, previous]).bool()
        };
        for day in 0..7u8 {
            for selector in 0..=6i8 {
                assert_eq!(call(&mut e, selector, day, 0), day as i8 == selector);
            }
            assert_eq!(call(&mut e, 7, day, 0), day != 0 && day != 6);
            assert_eq!(call(&mut e, 8, day, 0), day == 0 || day == 6);
            assert_eq!(call(&mut e, 9, day, 0), [1, 3, 5].contains(&day));
            assert_eq!(call(&mut e, 10, day, 0), [2, 4].contains(&day));
            assert!(!call(&mut e, 11, day, 0));
            assert!(!call(&mut e, -1, day, 0));
        }
        // "Previous" takes the day before, 0 wrapping to 6.
        assert!(call(&mut e, 2, 3, 1));
        assert!(!call(&mut e, 3, 3, 1));
        assert!(call(&mut e, 6, 0, 1));
        assert!(call(&mut e, 8, 0, 1));
        assert!(call(&mut e, 8, 1, 1));
    }

    #[test]
    fn target_is_an_actor() {
        let mut e = world();
        let actor = block(&mut e);
        let call = |e: &mut Engine, p: Ptr<TESPackage>, actor: u32| {
            e.call(0x00678ec0, &args![p, actor]).bool()
        };
        // No target: false.
        let p = package_with(&mut e, 0, None, None);
        assert!(!call(&mut e, p, actor));
        // A reference target that is an actor, one that is not, none.
        let person = block(&mut e);
        e.mem.set_u8(person + 0x1f0, 1);
        let p = follow_package(&mut e, 0, 0, person);
        assert!(call(&mut e, p, actor));
        let thing = block(&mut e);
        let p = follow_package(&mut e, 0, 0, thing);
        assert!(!call(&mut e, p, actor));
        let p = follow_package(&mut e, 0, 0, 0);
        assert!(!call(&mut e, p, actor));
        // An object target: form type 0x2a or 0x2b.
        for (form_type, expected) in [(0x2au8, true), (0x2b, true), (0x2c, false), (0x15, false)] {
            let object = form(&mut e, form_type);
            let p = follow_package(&mut e, 0, 1, object);
            assert_eq!(call(&mut e, p, actor), expected);
        }
        // An object type target: 0xe and 0xf.
        for (object_type, expected) in [(0xdu32, false), (0xe, true), (0xf, true), (0x10, false)] {
            let p = follow_package(&mut e, 0, 2, 0);
            let t = e.get(p, TESPackage::pPackTarg).addr();
            e.mem.set_u32(t + 12, object_type);
            assert_eq!(call(&mut e, p, actor), expected);
        }
        // A target of kind 3: the actor's linked reference.
        let p = follow_package(&mut e, 0, 3, 0);
        assert!(!call(&mut e, p, actor));
        e.mem.set_u32(actor + 0x44, thing);
        assert!(!call(&mut e, p, actor));
        e.mem.set_u32(actor + 0x44, person);
        assert!(call(&mut e, p, actor));
        // A target of another kind: false.
        let p = follow_package(&mut e, 0, 6, person);
        assert!(!call(&mut e, p, actor));
    }
}

#[cfg(test)]
mod third_batch_tests {
    use super::tests::{calls_to, data_object, engine, log_on, package, DATA_VTABLE, NAME_POINTER};
    use super::*;

    // ------------------------------------------------------------------
    // Save and load (`00678fd0` to `0067acf0`)
    // ------------------------------------------------------------------

    /// The save/load game object, its byte stream and the cells the doubles
    /// share (the test engine's memory is sparse; these pages are ours).
    const SAVE_OBJECT: u32 = 0x0131_0000;
    const STREAM: u32 = 0x0131_2000;
    const LOG_FLAG: u32 = 0x0131_0200;
    const SCRATCH: u32 = 0x0131_0300;
    /// The record of the form being saved or loaded: ID, flags, version.
    const RECORD: u32 = 0x0131_0400;
    const FORM_OBJECT_VTABLE: u32 = 0x0131_0800;
    const FAKE_MEMBER_FLAG: u32 = 0x7000_0204;
    const FAKE_BUFFER: u32 = 0x0131_0900;

    /// The first batch's engine plus the save/load doubles: the stream is
    /// `STREAM`, its position is the word at `SAVE_OBJECT + 0x14`, save game
    /// blocks are on when the byte at `SAVE_OBJECT + 0x20` is set.
    fn save_engine() -> Engine {
        let mut e = engine();
        e.map(0x011d_e000, 0x1000);
        e.map(0x0131_0000, 0x1000);
        e.map(STREAM, 0x2_0000);
        e.set_global(SAVE_LOAD_GAME, SAVE_OBJECT);
        e.mem.set_u32(SAVE_OBJECT + 0x14, STREAM);
        e.mem.write(STREAM, &[0xee; 0x100]);
        e.register(USE_SAVE_GAME_BLOCKS, |e, _| {
            (e.mem.u8(SAVE_OBJECT + 0x20) != 0).into_ret()
        });
        e.register(SAVE_POSITION, |e, _| {
            e.mem.u32(SAVE_OBJECT + 0x14).into_ret()
        });
        e.register(FLAG_BYTE_ADDRESS, |_, _| LOG_FLAG.into_ret());
        e.register(CURRENT_SAVE_FORM, |e, _| {
            e.mem.u32(SAVE_OBJECT + 0x88).into_ret()
        });
        e.register(CURRENT_LOAD_FORM, |e, _| {
            e.mem.u32(SAVE_OBJECT + 0x84).into_ret()
        });
        e.register(LOAD_VERSION, |e, _| e.mem.u8(SAVE_OBJECT + 0x30).into_ret());
        for address in [ERROR_LOG, SAVE_LOAD_LOG] {
            e.register(address, |_, _| Ret::default());
        }
        // The stream writers and readers: (save or buffer, ptr, size).
        for address in [SAVE_BYTES, BUFFER_SAVE_DATA] {
            e.register(address, |e, a| {
                let position = e.mem.u32(SAVE_OBJECT + 0x14);
                let data = e.mem.bytes(a[1], a[2]);
                e.mem.write(position, &data);
                e.mem.set_u32(SAVE_OBJECT + 0x14, position + a[2]);
                Ret::default()
            });
        }
        for address in [LOAD_BYTES, BUFFER_LOAD_DATA] {
            e.register(address, |e, a| {
                let position = e.mem.u32(SAVE_OBJECT + 0x14);
                let data = e.mem.bytes(position, a[2]);
                e.mem.write(a[1], &data);
                e.mem.set_u32(SAVE_OBJECT + 0x14, position + a[2]);
                Ret::default()
            });
        }
        e.register(FORM_SAVE_DATA_OLD, |e, a| {
            // The part flags the package computed.
            e.mem.set_u8(SCRATCH, e.mem.u8(a[1]));
            Ret::default()
        });
        e.register(FORM_LOAD_DATA_OLD, |e, a| {
            // The part flags the saved data holds.
            e.mem.set_u8(a[1], e.mem.u8(SCRATCH + 1));
            Ret::default()
        });
        e.register(LOOKUP_FORM, |e, _| {
            let named = e.mem.u32(SCRATCH + 0x10);
            named.into_ret()
        });
        for address in [
            LOCATION_SAVE_OLD,
            LOCATION_LOAD_OLD,
            LOCATION_AFTER_LOAD,
            LOCATION_SAVE_BUFFER,
            LOCATION_LOAD_BUFFER,
            LOCATION_INIT_LOAD_BUFFER,
            TARGET_SAVE_OLD,
            TARGET_LOAD_OLD,
            TARGET_AFTER_LOAD,
            TARGET_SAVE_BUFFER,
            TARGET_LOAD_BUFFER,
            TARGET_INIT_LOAD_BUFFER,
            FORM_LOAD_GAME,
            FORM_CLEAR_FLAGS,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(LOCATION_GET_SAVE_SIZE, |_, _| 0x30u32.into_ret());
        e.register(TARGET_GET_SAVE_SIZE, |_, _| 0x500u32.into_ret());
        // The form named in the log messages.
        let named = package(&mut e, 0);
        e.mem.set_u32(SCRATCH + 0x10, named.addr());
        e
    }

    /// A record `RECORD` of the form being saved/loaded.
    fn set_record(e: &mut Engine, slot: u32) {
        e.mem.set_u32(RECORD, 0x0100_0014);
        e.mem.set_u32(RECORD + 5, 0x20);
        e.mem.set_u8(RECORD + 9, 3);
        e.mem.set_u32(SAVE_OBJECT + slot, RECORD);
    }

    fn make_dynamic(e: &mut Engine) {
        e.register(IS_DYNAMIC_FORM_ID, |_, _| true.into_ret());
    }

    #[test]
    fn save_size_adds_the_parts() {
        let mut e = save_engine();
        let p = package(&mut e, 0);
        // 13 for the form data and 4 for the trailing word.
        assert_eq!(e.call(0x00678fd0, &args![p]).u16(), 17);
        // Save game blocks add the 4 byte tag and the 2 byte length.
        e.mem.set_u8(SAVE_OBJECT + 0x20, 1);
        assert_eq!(e.call(0x00678fd0, &args![p]).u16(), 23);
        // The location and the target add their sizes (0x30 and 0x500).
        let location = e.mem.alloc(16);
        e.set(p, TESPackage::pPackLoc, Ptr::new(location));
        assert_eq!(e.call(0x00678fd0, &args![p]).u16(), 23 + 0x30);
        let target = e.mem.alloc(16);
        e.set(p, TESPackage::pPackTarg, Ptr::new(target));
        log_on(&mut e);
        assert_eq!(e.call(0x00678fd0, &args![p]).u16(), 23 + 0x30 + 0x500);
        assert_eq!(calls_to(&e, LOCATION_GET_SAVE_SIZE), [[location]]);
        assert_eq!(calls_to(&e, TARGET_GET_SAVE_SIZE), [[target]]);
        assert!(calls_to(&e, ERROR_LOG).is_empty());
    }

    #[test]
    fn save_size_logging_reports_with_or_without_the_form() {
        let mut e = save_engine();
        let p = package(&mut e, 0);
        e.mem.set_u8(LOG_FLAG, 1);
        log_on(&mut e);
        e.call(0x00678fd0, &args![p]);
        assert_eq!(
            calls_to(&e, ERROR_LOG),
            [[0x0101_2c78, 17, 0x1d51, PACKAGE_SOURCE_FILE]]
        );
        set_record(&mut e, 0x88);
        log_on(&mut e);
        e.call(0x00678fd0, &args![p]);
        assert_eq!(
            calls_to(&e, ERROR_LOG),
            [[
                0x0101_2cb0,
                17,
                0x0100_0014,
                NAME_POINTER,
                0x20,
                0x1d51,
                PACKAGE_SOURCE_FILE
            ]]
        );
        assert_eq!(calls_to(&e, LOOKUP_FORM), [[0x0100_0014]]);
    }

    #[test]
    fn old_save_writes_the_block_and_patches_the_length() {
        let mut e = save_engine();
        let p = package(&mut e, 0);
        e.mem.set_u8(SAVE_OBJECT + 0x20, 1);
        e.mem.set_u32(p.addr() + 0x18, 0xaabb_ccdd);
        e.mem.write(p.addr() + 0x1c, &[7; 12]);
        let location = e.mem.alloc(16);
        let target = e.mem.alloc(16);
        e.set(p, TESPackage::pPackLoc, Ptr::new(location));
        e.set(p, TESPackage::pPackTarg, Ptr::new(target));
        log_on(&mut e);
        e.call(0x00679120, &args![p]);
        // Tag, 16-bit length, the 12 bytes, the word at +0x18.
        assert_eq!(e.mem.u32(STREAM), 0x424c_4f4b);
        assert_eq!(e.mem.bytes(STREAM + 6, 12), vec![7; 12]);
        assert_eq!(e.mem.u32(STREAM + 18), 0xaabb_ccdd);
        assert_eq!(e.mem.u32(SAVE_OBJECT + 0x14), STREAM + 22);
        // The length counts from the length field itself to the end.
        assert_eq!(e.mem.u16(STREAM + 4), 18);
        // Both parts present: flags 3.
        assert_eq!(e.mem.u8(SCRATCH), 3);
        assert_eq!(calls_to(&e, FORM_SAVE_DATA_OLD).len(), 1);
        assert_eq!(calls_to(&e, LOCATION_SAVE_OLD), [[location]]);
        assert_eq!(calls_to(&e, TARGET_SAVE_OLD), [[target]]);
        assert!(calls_to(&e, ERROR_LOG).is_empty());
        // Without parts and without blocks: flags 0 and the bare bytes.
        let mut e = save_engine();
        let p = package(&mut e, 0);
        e.mem.set_u8(SCRATCH, 9);
        log_on(&mut e);
        e.call(0x00679120, &args![p]);
        assert_eq!(e.mem.u8(SCRATCH), 0);
        assert_eq!(e.mem.u32(SAVE_OBJECT + 0x14), STREAM + 16);
        assert!(calls_to(&e, LOCATION_SAVE_OLD).is_empty());
        assert!(calls_to(&e, TARGET_SAVE_OLD).is_empty());
    }

    #[test]
    fn old_save_logs_the_size_and_the_oversized_block() {
        let mut e = save_engine();
        let p = package(&mut e, 0);
        e.mem.set_u8(LOG_FLAG, 1);
        log_on(&mut e);
        e.call(0x00679120, &args![p]);
        assert_eq!(
            calls_to(&e, ERROR_LOG),
            [[0x0101_536c, 16, 0x1d6e, PACKAGE_SOURCE_FILE]]
        );
        set_record(&mut e, 0x88);
        e.mem.set_u32(SAVE_OBJECT + 0x14, STREAM);
        log_on(&mut e);
        e.call(0x00679120, &args![p]);
        assert_eq!(
            calls_to(&e, ERROR_LOG),
            [[
                0x0101_53a0,
                16,
                0x0100_0014,
                NAME_POINTER,
                0x20,
                0x1d6e,
                PACKAGE_SOURCE_FILE
            ]]
        );
        // A block longer than 0xFFFF bytes: the logger is told.
        e.mem.set_u8(LOG_FLAG, 0);
        e.mem.set_u8(SAVE_OBJECT + 0x20, 1);
        e.mem.set_u32(SAVE_OBJECT + 0x14, STREAM);
        let location = e.mem.alloc(16);
        e.set(p, TESPackage::pPackLoc, Ptr::new(location));
        e.register(LOCATION_SAVE_OLD, |e, _| {
            let position = e.mem.u32(SAVE_OBJECT + 0x14);
            e.mem.set_u32(SAVE_OBJECT + 0x14, position + 0x1_0000);
            Ret::default()
        });
        log_on(&mut e);
        e.call(0x00679120, &args![p]);
        assert_eq!(
            calls_to(&e, SAVE_LOAD_LOG),
            [[0x0101_5318, PACKAGE_SOURCE_FILE, 0x1d6e]]
        );
    }

    /// Writes a saved package into the stream: the tag and length when the
    /// blocks are on, the 12 bytes and the word at +0x18.
    fn put_saved_package(e: &mut Engine, blocks: bool, length: u16) {
        e.mem.set_u8(SAVE_OBJECT + 0x20, blocks as u8);
        let mut at = STREAM;
        if blocks {
            e.mem.set_u32(at, 0x424c_4f4b);
            e.mem.set_u16(at + 4, length);
            at += 6;
        }
        e.mem.write(at, &[9; 12]);
        e.mem.set_u32(at + 12, 0x1122_3344);
    }

    #[test]
    fn old_load_reads_the_parts() {
        let mut e = save_engine();
        let p = package(&mut e, 0);
        put_saved_package(&mut e, true, 18);
        e.mem.set_u8(SCRATCH + 1, 3);
        log_on(&mut e);
        e.call(0x00679340, &args![p]);
        assert_eq!(e.mem.bytes(p.addr() + 0x1c, 12), vec![9; 12]);
        assert_eq!(e.mem.u32(p.addr() + 0x18), 0x1122_3344);
        let location = e.get(p, TESPackage::pPackLoc);
        let target = e.get(p, TESPackage::pPackTarg);
        assert!(!location.is_null() && !target.is_null());
        assert_eq!(calls_to(&e, LOCATION_LOAD_OLD), [[location.addr()]]);
        assert_eq!(calls_to(&e, TARGET_LOAD_OLD), [[target.addr()]]);
        assert_eq!(calls_to(&e, OPERATOR_NEW)[0], [LOCATION_SIZE]);
        assert_eq!(calls_to(&e, OPERATOR_NEW)[1], [TARGET_SIZE]);
        // The block length matches the bytes read: nothing is logged.
        assert!(calls_to(&e, SAVE_LOAD_LOG).is_empty());
        // Flags 1 and 2 pick one part each; flags 0 none.
        for (flags, location_made, target_made) in
            [(1u8, true, false), (2, false, true), (0, false, false)]
        {
            let mut e = save_engine();
            let p = package(&mut e, 0);
            put_saved_package(&mut e, false, 0);
            e.mem.set_u8(SCRATCH + 1, flags);
            e.call(0x00679340, &args![p]);
            assert_eq!(!e.get(p, TESPackage::pPackLoc).is_null(), location_made);
            assert_eq!(!e.get(p, TESPackage::pPackTarg).is_null(), target_made);
        }
    }

    #[test]
    fn old_load_reports_a_wrong_tag_and_a_wrong_length() {
        // A wrong tag, no form loading: the version of the save object.
        let mut e = save_engine();
        let p = package(&mut e, 0);
        put_saved_package(&mut e, true, 18);
        e.mem.set_u32(STREAM, 0x1234_5678);
        e.mem.set_u8(SAVE_OBJECT + 0x30, 5);
        log_on(&mut e);
        e.call(0x00679340, &args![p]);
        assert_eq!(
            calls_to(&e, SAVE_LOAD_LOG),
            [[0x0101_56a8, PACKAGE_SOURCE_FILE, 0x1d74, 5]]
        );
        // With a form being loaded: its id, name, version and flags.
        set_record(&mut e, 0x84);
        put_saved_package(&mut e, true, 18);
        e.mem.set_u32(STREAM, 0x1234_5678);
        e.mem.set_u32(SAVE_OBJECT + 0x14, STREAM);
        log_on(&mut e);
        e.call(0x00679340, &args![p]);
        assert_eq!(
            calls_to(&e, SAVE_LOAD_LOG),
            [[
                0x0101_5718,
                PACKAGE_SOURCE_FILE,
                0x1d74,
                0x0100_0014,
                NAME_POINTER,
                3,
                0x20
            ]]
        );
        // The block says 16 bytes but 18 were read: an overrun of 2 bytes.
        let mut e = save_engine();
        let p = package(&mut e, 0);
        put_saved_package(&mut e, true, 16);
        e.mem.set_u8(SAVE_OBJECT + 0x30, 5);
        log_on(&mut e);
        e.call(0x00679340, &args![p]);
        assert_eq!(
            calls_to(&e, SAVE_LOAD_LOG),
            [[0x0101_54a0, 2, PACKAGE_SOURCE_FILE, 0x1d8b, 5]]
        );
        // The block says 20: an underrun of 2 bytes, with the form named.
        set_record(&mut e, 0x84);
        put_saved_package(&mut e, true, 20);
        e.mem.set_u32(SAVE_OBJECT + 0x14, STREAM);
        log_on(&mut e);
        e.call(0x00679340, &args![p]);
        assert_eq!(
            calls_to(&e, SAVE_LOAD_LOG),
            [[
                0x0101_5500,
                2,
                PACKAGE_SOURCE_FILE,
                0x1d8b,
                0x0100_0014,
                NAME_POINTER,
                3,
                0x20
            ]]
        );
        // And the same two with a form for the overrun, without for the
        // underrun.
        put_saved_package(&mut e, true, 16);
        e.mem.set_u32(SAVE_OBJECT + 0x14, STREAM);
        log_on(&mut e);
        e.call(0x00679340, &args![p]);
        assert_eq!(calls_to(&e, SAVE_LOAD_LOG)[0][0], 0x0101_5588);
        e.mem.set_u32(SAVE_OBJECT + 0x84, 0);
        put_saved_package(&mut e, true, 20);
        e.mem.set_u32(SAVE_OBJECT + 0x14, STREAM);
        log_on(&mut e);
        e.call(0x00679340, &args![p]);
        assert_eq!(
            calls_to(&e, SAVE_LOAD_LOG),
            [[0x0101_5440, 2, PACKAGE_SOURCE_FILE, 0x1d8b, 5]]
        );
    }

    #[test]
    fn post_load_step_runs_for_the_parts_that_exist() {
        let mut e = save_engine();
        let p = package(&mut e, 0);
        log_on(&mut e);
        e.call(0x006796c0, &args![p]);
        assert!(calls_to(&e, LOCATION_AFTER_LOAD).is_empty());
        assert!(calls_to(&e, TARGET_AFTER_LOAD).is_empty());
        let location = e.mem.alloc(16);
        e.set(p, TESPackage::pPackLoc, Ptr::new(location));
        log_on(&mut e);
        e.call(0x006796c0, &args![p]);
        assert_eq!(calls_to(&e, LOCATION_AFTER_LOAD), [[location]]);
        assert!(calls_to(&e, TARGET_AFTER_LOAD).is_empty());
        let target = e.mem.alloc(16);
        e.set(p, TESPackage::pPackTarg, Ptr::new(target));
        log_on(&mut e);
        e.call(0x006796c0, &args![p]);
        assert_eq!(calls_to(&e, TARGET_AFTER_LOAD), [[target]]);
    }

    #[test]
    fn form_flags_map_to_package_flag_bits() {
        let mut e = save_engine();
        let p = package(&mut e, 0);
        log_on(&mut e);
        e.call(0x00679700, &args![p, 0x8000_0000u32, 0x55u32]);
        assert_eq!(
            calls_to(&e, FORM_LOAD_GAME),
            [[p.addr(), 0x8000_0000, 0x55]]
        );
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x8000);
        e.call(0x00679700, &args![p, 0x4000_0000u32, 0u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x1_8000);
        // Other flags change nothing.
        let q = package(&mut e, 0);
        e.call(0x00679700, &args![q, 0x3fff_ffffu32, 0u32]);
        assert_eq!(e.get(q, TESPackage::iPackFlags), 0);
        // The clearing counterpart.
        e.set(p, TESPackage::iPackFlags, 0x1_8005);
        log_on(&mut e);
        e.call(0x00679760, &args![p, 0x8000_0000u32]);
        assert_eq!(calls_to(&e, FORM_CLEAR_FLAGS), [[p.addr(), 0x8000_0000]]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x1_0005);
        e.call(0x00679760, &args![p, 0xc000_0000u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 5);
        e.call(0x00679760, &args![p, 0x1u32]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 5);
    }

    #[test]
    fn save_writes_only_for_a_dynamic_form() {
        let mut e = save_engine();
        let p = package(&mut e, 0);
        log_on(&mut e);
        e.call(0x006797c0, &args![p, FAKE_BUFFER]);
        assert!(calls_to(&e, BUFFER_SAVE_DATA).is_empty());
        make_dynamic(&mut e);
        e.mem.set_u32(p.addr() + 0x18, 0xa1b2_c3d4);
        e.mem.write(p.addr() + 0x1c, &[5; 12]);
        let location = e.mem.alloc(16);
        let target = e.mem.alloc(16);
        let data = data_object(&mut e, 16);
        e.set(p, TESPackage::pPackLoc, Ptr::new(location));
        e.set(p, TESPackage::pPackTarg, Ptr::new(target));
        e.set(p, TESPackage::pPackData, Ptr::new(data));
        e.register(FAKE_DATA_SAVE, |_, _| Ret::default());
        e.mem.set_u32(DATA_VTABLE + 0x14, FAKE_DATA_SAVE);
        log_on(&mut e);
        e.call(0x006797c0, &args![p, FAKE_BUFFER]);
        // 12 bytes, the part flags (7), the word.
        assert_eq!(e.mem.bytes(STREAM, 12), vec![5; 12]);
        assert_eq!(e.mem.u8(STREAM + 12), 7);
        assert_eq!(e.mem.u32(STREAM + 13), 0xa1b2_c3d4);
        assert_eq!(
            calls_to(&e, LOCATION_SAVE_BUFFER),
            [[location, FAKE_BUFFER]]
        );
        assert_eq!(calls_to(&e, TARGET_SAVE_BUFFER), [[target, FAKE_BUFFER]]);
        assert_eq!(calls_to(&e, FAKE_DATA_SAVE), [[data, FAKE_BUFFER]]);
        assert_eq!(calls_to(&e, BUFFER_SAVE_DATA).len(), 3);
        assert_eq!(calls_to(&e, BUFFER_SAVE_DATA)[1][2..], [1, 0]);
        // Without parts the flag byte is 0.
        let mut e = save_engine();
        make_dynamic(&mut e);
        let q = package(&mut e, 0);
        e.call(0x006797c0, &args![q, FAKE_BUFFER]);
        assert_eq!(e.mem.u8(STREAM + 12), 0);
    }

    const FAKE_DATA_SAVE: u32 = 0x7000_0014;
    const FAKE_DATA_LOAD: u32 = 0x7000_0018;
    const FAKE_DATA_INIT_LOAD: u32 = 0x7000_001c;

    #[test]
    fn load_from_the_buffer_reads_the_parts() {
        let mut e = save_engine();
        make_dynamic(&mut e);
        let p = package(&mut e, 0);
        let data = data_object(&mut e, 16);
        e.set(p, TESPackage::pPackData, Ptr::new(data));
        e.mem.set_u32(DATA_VTABLE + 0x18, FAKE_DATA_LOAD);
        e.register(FAKE_DATA_LOAD, |_, _| Ret::default());
        e.mem.write(STREAM, &[4; 12]);
        e.mem.set_u8(STREAM + 12, 7);
        e.mem.set_u32(STREAM + 13, 0x0102_0304);
        log_on(&mut e);
        e.call(0x006798c0, &args![p, FAKE_BUFFER]);
        assert_eq!(e.mem.bytes(p.addr() + 0x1c, 12), vec![4; 12]);
        assert_eq!(e.mem.u32(p.addr() + 0x18), 0x0102_0304);
        let location = e.get(p, TESPackage::pPackLoc);
        let target = e.get(p, TESPackage::pPackTarg);
        assert_eq!(
            calls_to(&e, LOCATION_LOAD_BUFFER),
            [[location.addr(), FAKE_BUFFER]]
        );
        assert_eq!(
            calls_to(&e, TARGET_LOAD_BUFFER),
            [[target.addr(), FAKE_BUFFER]]
        );
        assert_eq!(calls_to(&e, FAKE_DATA_LOAD), [[data, FAKE_BUFFER]]);
        // No part flags: nothing created.
        let mut e = save_engine();
        make_dynamic(&mut e);
        let q = package(&mut e, 0);
        e.mem.write(STREAM, &[4; 12]);
        e.mem.set_u8(STREAM + 12, 0);
        e.call(0x006798c0, &args![q, FAKE_BUFFER]);
        assert!(e.get(q, TESPackage::pPackLoc).is_null());
        assert!(e.get(q, TESPackage::pPackTarg).is_null());
    }

    #[test]
    fn load_from_the_buffer_of_a_static_form_tests_the_flags() {
        let mut e = save_engine();
        // The flag tests answer true for the masks in the scratch word.
        e.register(BUFFER_FLAG_OBJECT_A, |_, a| a[1].into_ret());
        e.register(BUFFER_FLAG_OBJECT_B, |_, a| a[1].into_ret());
        e.register(BUFFER_FLAG_TEST, |e, a| {
            (e.mem.u32(SCRATCH + 8) & a[1] != 0).into_ret()
        });
        let p = package(&mut e, 0);
        log_on(&mut e);
        e.call(0x006798c0, &args![p, FAKE_BUFFER]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0);
        let objects = calls_to(&e, BUFFER_FLAG_OBJECT_A);
        assert_eq!(
            objects,
            [[FAKE_BUFFER, objects[0][1]], [FAKE_BUFFER, objects[1][1]]]
        );
        assert_eq!(
            calls_to(&e, BUFFER_FLAG_TEST),
            [[objects[0][1], 0x8000_0000], [objects[1][1], 0x4000_0000]]
        );
        assert!(calls_to(&e, BUFFER_LOAD_DATA).is_empty());
        e.mem.set_u32(SCRATCH + 8, 0x8000_0000);
        e.call(0x006798c0, &args![p, FAKE_BUFFER]);
        assert_eq!(e.get(p, TESPackage::iPackFlags), 0x8000);
        e.mem.set_u32(SCRATCH + 8, 0x4000_0000);
        let q = package(&mut e, 0);
        e.call(0x006798c0, &args![q, FAKE_BUFFER]);
        assert_eq!(e.get(q, TESPackage::iPackFlags), 0x1_0000);
        // The clearing twin, `00679b10`.
        e.mem.set_u32(SCRATCH + 8, 0xc000_0000);
        e.set(q, TESPackage::iPackFlags, 0x1_8007);
        e.call(0x00679b10, &args![q, FAKE_BUFFER]);
        assert_eq!(e.get(q, TESPackage::iPackFlags), 7);
        e.mem.set_u32(SCRATCH + 8, 0x8000_0000);
        e.set(q, TESPackage::iPackFlags, 0x1_8007);
        e.call(0x00679b10, &args![q, FAKE_BUFFER]);
        assert_eq!(e.get(q, TESPackage::iPackFlags), 0x1_0007);
        // A dynamic form is left alone.
        make_dynamic(&mut e);
        e.mem.set_u32(SCRATCH + 8, 0xc000_0000);
        e.set(q, TESPackage::iPackFlags, 0x1_8007);
        log_on(&mut e);
        e.call(0x00679b10, &args![q, FAKE_BUFFER]);
        assert_eq!(e.get(q, TESPackage::iPackFlags), 0x1_8007);
        assert!(calls_to(&e, BUFFER_FLAG_OBJECT_B).is_empty());
    }

    #[test]
    fn init_load_calls_the_parts_of_a_dynamic_form() {
        let mut e = save_engine();
        let p = package(&mut e, 0);
        let location = e.mem.alloc(16);
        let target = e.mem.alloc(16);
        let data = data_object(&mut e, 16);
        e.set(p, TESPackage::pPackLoc, Ptr::new(location));
        e.set(p, TESPackage::pPackTarg, Ptr::new(target));
        e.set(p, TESPackage::pPackData, Ptr::new(data));
        e.mem.set_u32(DATA_VTABLE + 0x1c, FAKE_DATA_INIT_LOAD);
        e.register(FAKE_DATA_INIT_LOAD, |_, _| Ret::default());
        log_on(&mut e);
        e.call(0x00679a90, &args![p, FAKE_BUFFER]);
        assert!(calls_to(&e, LOCATION_INIT_LOAD_BUFFER).is_empty());
        make_dynamic(&mut e);
        log_on(&mut e);
        e.call(0x00679a90, &args![p, FAKE_BUFFER]);
        assert_eq!(
            calls_to(&e, LOCATION_INIT_LOAD_BUFFER),
            [[location, FAKE_BUFFER]]
        );
        assert_eq!(
            calls_to(&e, TARGET_INIT_LOAD_BUFFER),
            [[target, FAKE_BUFFER]]
        );
        assert_eq!(calls_to(&e, FAKE_DATA_INIT_LOAD), [[data, FAKE_BUFFER]]);
        // A package without parts calls nothing.
        let q = package(&mut e, 0);
        log_on(&mut e);
        e.call(0x00679a90, &args![q, FAKE_BUFFER]);
        assert!(calls_to(&e, TARGET_INIT_LOAD_BUFFER).is_empty());
    }

    /// A form block of the given type, with the `+0x3c` member answering
    /// `member` through slot 4 of its vtable.
    fn typed_form(e: &mut Engine, kind: u8, member: bool) -> u32 {
        let f = e.mem.alloc(0x80);
        e.mem.set_u8(f + 4, kind);
        e.put_vtable(FORM_OBJECT_VTABLE, &[0, FAKE_MEMBER_FLAG]);
        e.mem.set_u32(f + 0x3c, FORM_OBJECT_VTABLE);
        e.mem.set_u8(f + 0x40, member as u8);
        f
    }

    fn type_engine() -> Engine {
        let mut e = save_engine();
        e.register(GET_FORM_TYPE, |e, a| (e.mem.u8(a[0] + 4) as u32).into_ret());
        // The member's flag lives in the byte after its vtable pointer.
        e.register(FAKE_MEMBER_FLAG, |e, a| {
            (e.mem.u8(a[0] + 4) != 0).into_ret()
        });
        // 0x28 tests: the byte at +0x50 (sixteen) and +0x51 (seventeen).
        e.register(FORM_TEST_SIXTEEN, |e, a| {
            (e.mem.u8(a[0] + 0x50) != 0).into_ret()
        });
        e.register(FORM_TEST_SEVENTEEN, |e, a| {
            (e.mem.u8(a[0] + 0x51) != 0).into_ret()
        });
        // The effect list tests answer with the byte at +0, +1, +2 of the
        // list (the form + 0x24).
        e.register(EFFECT_LIST_TEST_TARGET, |e, a| {
            (e.mem.u8(a[0]) != 0).into_ret()
        });
        e.register(EFFECT_LIST_TEST_TOUCH, |e, a| {
            (e.mem.u8(a[0] + 1) != 0).into_ret()
        });
        e.register(EFFECT_LIST_TEST_THIRD, |e, a| {
            (e.mem.u8(a[0] + 2) != 0).into_ret()
        });
        e
    }

    #[test]
    fn object_type_from_the_form_type() {
        let mut e = type_engine();
        let expected: [(u8, i32); 24] = [
            (0x14, 0x18),
            (0x15, 1),
            (0x18, 2),
            (0x19, 3),
            (0x1a, 4),
            (0x1b, 5),
            (0x1c, 6),
            (0x1e, 8),
            (0x1f, 9),
            (0x32, 9),
            (0x67, 9),
            (0x6c, 9),
            (0x73, 9),
            (0x74, 9),
            (0x26, 10),
            (0x27, 11),
            (0x29, 0xd),
            (0x2a, 0xe),
            (0x2b, 0xf),
            (0x2e, 0x10),
            (0x13, 0),
            (0x16, 0),
            (0x2c, 0),
            (0x70, 0),
        ];
        for (kind, object_type) in expected {
            let f = typed_form(&mut e, kind, false);
            assert_eq!(
                e.call(0x00679ba0, &args![f]).i32(),
                object_type,
                "form type {kind:#x}"
            );
        }
        // No form: 0 and nothing is read.
        log_on(&mut e);
        assert_eq!(e.call(0x00679ba0, &args![0u32]).i32(), 0);
        assert!(calls_to(&e, GET_FORM_TYPE).is_empty());
        // Types 0x1d and 0x2f ask the `+0x3c` member.
        for (kind, plain) in [(0x1du8, 7), (0x2f, 0x11)] {
            let a = typed_form(&mut e, kind, false);
            assert_eq!(e.call(0x00679ba0, &args![a]).i32(), plain);
            let b = typed_form(&mut e, kind, true);
            assert_eq!(e.call(0x00679ba0, &args![b]).i32(), 0x12);
        }
        // Type 0x28: 0xc, or 0x16 / 0x17 by the two tests (0x16 first).
        let f = typed_form(&mut e, 0x28, false);
        assert_eq!(e.call(0x00679ba0, &args![f]).i32(), 0xc);
        e.mem.set_u8(f + 0x51, 1);
        assert_eq!(e.call(0x00679ba0, &args![f]).i32(), 0x17);
        e.mem.set_u8(f + 0x50, 1);
        assert_eq!(e.call(0x00679ba0, &args![f]).i32(), 0x16);
    }

    #[test]
    fn form_matches_object_types() {
        let mut e = type_engine();
        let matches = |e: &mut Engine, form: u32, object_type: i32| {
            e.call(0x00679e00, &args![form, object_type]).bool()
        };
        // The fixed pairs: form type and the object types it matches.
        let table: [(u8, &[i32]); 21] = [
            (0x15, &[1]),
            (0x18, &[2, 0x13, 0x14]),
            (0x19, &[3]),
            (0x1a, &[4, 0x14]),
            (0x1b, &[5]),
            (0x1c, &[6]),
            (0x1e, &[8]),
            (0x1f, &[9]),
            (0x32, &[9]),
            (0x67, &[9]),
            (0x6c, &[9]),
            (0x73, &[9]),
            (0x74, &[9]),
            (0x26, &[10]),
            (0x27, &[11]),
            (0x29, &[0xd, 0x13, 0x14]),
            (0x2a, &[0xe, 0x1c]),
            (0x2b, &[0xf, 0x1c]),
            (0x2e, &[0x10]),
            (0x14, &[0x18]),
            (0x2f, &[0x11]),
        ];
        for (kind, accepted) in table {
            let f = typed_form(&mut e, kind, false);
            for object_type in 1..0x20 {
                assert_eq!(
                    matches(&mut e, f, object_type),
                    accepted.contains(&object_type),
                    "form type {kind:#x}, object type {object_type:#x}"
                );
            }
        }
        // Nothing for a null form or a zero object type.
        let f = typed_form(&mut e, 0x15, false);
        assert!(!matches(&mut e, 0, 1));
        assert!(!matches(&mut e, f, 0));
        // 0x1d: 7, and 0x12 when the member answers.
        let no = typed_form(&mut e, 0x1d, false);
        let yes = typed_form(&mut e, 0x1d, true);
        assert!(matches(&mut e, no, 7) && matches(&mut e, yes, 7));
        assert!(!matches(&mut e, no, 0x12) && matches(&mut e, yes, 0x12));
        let no = typed_form(&mut e, 0x2f, false);
        let yes = typed_form(&mut e, 0x2f, true);
        assert!(!matches(&mut e, no, 0x12) && matches(&mut e, yes, 0x12));
        // 0x28: 0xc, 0x13, 0x14 always; 0x16 / 0x17 by their own test.
        let f = typed_form(&mut e, 0x28, false);
        for object_type in [0xc, 0x13, 0x14] {
            assert!(matches(&mut e, f, object_type));
        }
        assert!(!matches(&mut e, f, 0x16) && !matches(&mut e, f, 0x17));
        e.mem.set_u8(f + 0x50, 1);
        assert!(matches(&mut e, f, 0x16) && !matches(&mut e, f, 0x17));
        e.mem.set_u8(f + 0x50, 0);
        e.mem.set_u8(f + 0x51, 1);
        assert!(!matches(&mut e, f, 0x16) && matches(&mut e, f, 0x17));
        assert!(!matches(&mut e, f, 0x15));
        // 0x14: 0x19, 0x1a and 0x1b by the effect list tests at +0x24.
        let f = typed_form(&mut e, 0x14, false);
        for (object_type, byte) in [(0x19, 0u32), (0x1a, 1), (0x1b, 2)] {
            assert!(!matches(&mut e, f, object_type));
            e.mem.set_u8(f + 0x24 + byte, 1);
            assert!(matches(&mut e, f, object_type));
        }
        assert!(!matches(&mut e, f, 0x1c));
    }

    #[test]
    fn data_object_constructor_sets_vtable_and_clears_the_word() {
        let mut e = save_engine();
        let object = e.mem.alloc(16);
        e.mem.set_u32(object + 4, 0x55);
        log_on(&mut e);
        assert_eq!(e.call(0x0067acf0, &args![object]).u32(), object);
        assert_eq!(calls_to(&e, DATA_BASE_CONSTRUCTOR), [[object]]);
        assert_eq!(e.mem.u32(object), DATA_BASE_VTABLE);
        assert_eq!(e.mem.u32(object + 4), 0);
    }
}

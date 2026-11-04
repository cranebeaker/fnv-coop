//! `fallout/ai/combat/vats.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `VATS` singleton lives at `0x011f2250` (its `commandList` is the object's first word pair, so
//! the object itself is the head node of a `BSSimpleList<VATS_COMMAND *>`). The unit also owns a
//! block of static data between `0x011f21b8` and `0x011f2320` that the functions below share; the
//! constants name it by role. The role names are read from how the code uses the data; where the
//! Xbox PDB names the flag that selects a global (`VATS_COMMAND::bStranger`, `bMissFortune`), the
//! doc comment says so, but what the global itself is called is not confirmed.
//!
//! x87 notes: the game computes `float` expressions in extended precision and stores `float`s. The
//! translations compute in `f64` and round to `f32` at each `float` store. Every float comparison
//! is the exe's own flag test: `x87_compare` gives the status-word high byte `FNSTSW AX` leaves in
//! `AH` (NaN sets all of C0, C2 and C3), and the branch is the same `TEST AH,mask` / `JZ` / `JP`
//! the code uses, so NaN operands take the same branch as in the game. A `FLD dword` followed by
//! `FSTP dword` turns a signalling NaN into a quiet one (`x87_quiet_bits`).
//!
//! Call conventions: arguments of every call are the words the exe pushes (read from the
//! disassembly; the decompiler often hangs a pushed word on the wrong callee), `this` first.

// Match arms mirror the exe's switch; folding an inner `if` into a guard
// would let a failed guard fall through to later arms, which the exe does
// not do.
#![allow(clippy::collapsible_match)]

#[allow(unused_imports)]
use crate::prelude::*;

// ---- callees outside this unit, by address ----------------------------------------------------
//
// The doc comments say what the call sites show (words in, result) and what the body does where
// it was read. A name from the Xbox PDB is marked; the engine map's name is used only when the
// body fits it.

/// Returns its `this` (the address of a list node, where the node's item word lives).
const LIST_ITEM_SLOT: u32 = 0x0068_15c0;
/// Returns `[this + 4]` (the node after a list node).
const LIST_NEXT: u32 = 0x0072_6070;
/// Byte result: the node has no next node and no item (`[this + 4] == 0 && [this] == 0`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// Removes the first node of a list: the head node stays and takes the second node's item and link
/// (or its item becomes 0 when it is the only node).
const LIST_POP_FRONT: u32 = 0x0063_f7b0;
/// `list.remove(&item)`: unlinks the node whose item equals `[argument]`.
const LIST_REMOVE_ITEM: u32 = 0x0090_5330;
/// `list.append(&item)`: one pointer to the item.
const LIST_APPEND: u32 = 0x0090_5820;
/// Allocation (cdecl, one size word).
const ALLOCATE: u32 = 0x0040_1000;
/// Release (cdecl, one pointer word).
const FREE_BLOCK: u32 = 0x0040_1030;
/// Constructor of a `VATS_COMMAND` in a 0x28-byte block (returns `this`).
const COMMAND_CONSTRUCTOR: u32 = 0x009c_a4a0;
/// Frees the command's hit-data block (`[this + 0x14]`) and clears the field.
const COMMAND_RELEASE_HIT_DATA: u32 = 0x009c_a4c0;
/// Setting object to the address of its float value (`this + 4`, or a static zero when null).
const SETTING_FLOAT_POINTER: u32 = 0x0040_3e20;
/// The same for an integer setting.
const SETTING_INT_POINTER: u32 = 0x0043_d4d0;
/// Float value of a setting (through `SETTING_FLOAT_POINTER`), returned in `ST0`.
const SETTING_FLOAT_VALUE: u32 = 0x0045_0410;
/// `BGSCameraShot::FreeCamera` (Xbox PDB).
const FREE_CAMERA: u32 = 0x0058_ce50;
/// `BGSCameraShot::IsInitialized` (Xbox PDB), byte result.
const CAMERA_IS_INITIALIZED: u32 = 0x0058_d630;
/// `BGSCameraShot::InitCamera` (Xbox PDB): `(actor, kind)`.
const INIT_CAMERA: u32 = 0x0058_c5d0;
/// Camera shot: sets the first view target (one word; the name is by use).
const CAMERA_SET_VIEW_TARGET: u32 = 0x0058_cbc0;
/// Camera shot: sets the second view target (one word; the name is by use).
const CAMERA_SET_SECOND_TARGET: u32 = 0x0058_ccf0;
/// Camera shot: advances it to a playback time (one float).
const CAMERA_ADVANCE: u32 = 0x0058_cf60;
/// Camera shot: writes its position to an output block (one pointer), returns that pointer.
const CAMERA_POSITION: u32 = 0x0058_d510;
/// Getters of a word at a fixed offset of `this` (their bodies are `MOV EAX,[ECX + n]`).
const WORD_AT_0X08: u32 = 0x0044_ddc0;
const WORD_AT_0X0C: u32 = 0x0084_e3a0;
const WORD_AT_0X2C: u32 = 0x0055_b980;
const WORD_AT_0X38: u32 = 0x009e_32d0;
const WORD_AT_0X3C: u32 = 0x0063_9b40;
const WORD_AT_0X40: u32 = 0x008d_6f30;
/// Getters of a float at a fixed offset of `this` (`FLD [ECX + n]`, result in `ST0`).
const FLOAT_AT_0X48: u32 = 0x0063_9aa0;
const FLOAT_AT_0X4C: u32 = 0x0050_7b00;
const FLOAT_AT_0X50: u32 = 0x009d_1b30;
const FLOAT_AT_0X54: u32 = 0x009d_1b50;
const FLOAT_AT_0X58: u32 = 0x0050_81b0;
/// Camera shot: byte result (body not read); when true the update asks the actor's process for a
/// preferred target.
const CAMERA_TEST_0058D4F0: u32 = 0x0058_d4f0;
/// Shot object: the three kind tests the update tries in order (byte results; the first tests
/// bits 0 and 1 of the byte at `+0x28`).
const SHOT_FLAG_FIRST: u32 = 0x0058_b4a0;
const SHOT_FLAG_SECOND: u32 = 0x0058_b4d0;
const SHOT_FLAG_THIRD: u32 = 0x0058_b4f0;
/// Picks a shot for `(actor, target)`; `this` is the object at `0x011ca700`.
const SELECT_SHOT: u32 = 0x0058_bb50;
/// Returns `this + 0x20` (the camera list embedded in a shot).
const SHOT_CAMERA_LIST: u32 = 0x0089_1170;
/// `ImageSpaceModifierInstance::Stop` (Xbox PDB), cdecl, one word.
const IMAGESPACE_STOP: u32 = 0x0052_93f0;
/// `ImageSpaceModifierInstanceForm::Trigger` (Xbox PDB), cdecl, three words.
const IMAGESPACE_FORM_TRIGGER: u32 = 0x0052_99a0;
/// `ImageSpaceModifierInstanceRB::Trigger` (Xbox PDB), cdecl, seven words.
const IMAGESPACE_RB_TRIGGER: u32 = 0x0052_a440;
/// Updates a running image-space modifier: four words after `this`.
const IMAGESPACE_UPDATE: u32 = 0x0052_9df0;
/// Image-space modifier: one float (called with the strength).
const IMAGESPACE_SET_STRENGTH: u32 = 0x0063_f790;
/// Image-space modifier: one flag word, called on a freshly triggered modifier.
const IMAGESPACE_ACTIVATE: u32 = 0x005b_bd60;
/// `BSTimer::SetGlobalTimeMultiplier` (Xbox PDB): `(multiplier, flag)`.
const SET_GLOBAL_TIME_MULTIPLIER: u32 = 0x00aa_4db0;
/// Frame time on the timer object, returned in `ST0` (callers store it as a double).
const TIMER_DELTA: u32 = 0x0084_d030;
/// The frame time's divisor, returned in `ST0`.
const TIMER_DIVISOR: u32 = 0x0071_6440;
/// The player-side time factor, returned in `ST0`.
const PLAYER_TIME_FACTOR: u32 = 0x0094_43f0;
/// `HUDMainMenu::SetMenuMode` (Xbox PDB), cdecl, one word.
const HUD_SET_MENU_MODE: u32 = 0x0077_1700;
/// `MiscStatManager::Increment` (Xbox PDB), cdecl, one stat number.
const STAT_INCREMENT: u32 = 0x004d_5c60;
/// `Actor::GetAnimation` (Xbox PDB), no stack words.
const ACTOR_GET_ANIMATION: u32 = 0x008b_70d0;
/// `Actor::GetAnimAction` (Xbox PDB).
const ACTOR_GET_ANIM_ACTION: u32 = 0x008a_7570;
/// `Animation::SpecialIdleFree` (Xbox PDB): two words after `this`.
const SPECIAL_IDLE_FREE: u32 = 0x0049_8910;
/// `Animation::SpecialIdleQueue` (Xbox PDB): `(idle, section)`.
const SPECIAL_IDLE_QUEUE: u32 = 0x0049_7ca0;
/// Returns `[this + 0x68]`, the actor's process (the engine map calls it
/// `MiddleHighProcess::GetSavedAcquireObject`).
const ACTOR_GET_PROCESS: u32 = 0x008d_8520;
/// `Actor::GetCurrentWeapon` (Xbox PDB).
const ACTOR_GET_CURRENT_WEAPON: u32 = 0x008a_1710;
/// `Actor::GetAlphaMult` (Xbox PDB), returns `ST0`.
const ACTOR_GET_ALPHA_MULT: u32 = 0x008c_4850;
/// `Actor::SetAlphaMult` (Xbox PDB), one float.
const ACTOR_SET_ALPHA_MULT: u32 = 0x008c_4890;
/// `Actor::UpdateAlpha` (Xbox PDB).
const ACTOR_UPDATE_ALPHA: u32 = 0x008c_4640;
/// `FalloutAudio::MysteriousStrangerAppears` (Xbox PDB), no words.
const STRANGER_APPEARS_SOUND: u32 = 0x0082_ed80;
/// `Actor::SetIronSights` (Xbox PDB): three words.
const ACTOR_SET_IRON_SIGHTS: u32 = 0x008b_b650;
/// `Actor::SetBlock` (Xbox PDB): one word.
const ACTOR_SET_BLOCK: u32 = 0x0089_4cc0;
/// `Actor::UnEquipObject` (Xbox PDB): six words.
const ACTOR_UNEQUIP_OBJECT: u32 = 0x0088_d7d0;
/// `Actor::RestoreActorValue` (Xbox PDB): `(actor value, amount)`.
const ACTOR_RESTORE_ACTOR_VALUE: u32 = 0x0088_b740;
/// `Actor::LineOfSight` (Xbox PDB): five words after `this`.
const ACTOR_LINE_OF_SIGHT: u32 = 0x0088_b880;
/// `ActionPoints::IsAttackAction` (Xbox PDB), cdecl, one action number.
const IS_ATTACK_ACTION: u32 = 0x0066_dde0;
/// Cdecl test on an action number: true for 7 and 8 (body read).
const IS_ACTION_SEVEN_OR_EIGHT: u32 = 0x0066_de80;
/// `HitData::ClearData` (Xbox PDB).
const HIT_DATA_CLEAR: u32 = 0x009b_4dc0;
/// `HitData::InitializeHitData` (Xbox PDB), cdecl `(hitData, command)`.
const HIT_DATA_INITIALIZE: u32 = 0x009b_5050;
/// `BGSEntryPoint::HandleEntryPoint` (Xbox PDB), cdecl, five words.
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// `PlayerCharacter::SetFirstPerson` (Xbox PDB): one flag.
const PLAYER_SET_FIRST_PERSON: u32 = 0x0095_0110;
/// `TESIdleManager::GetIdleToPlay` (Xbox PDB): `(actor, target)` after `this`.
const GET_IDLE_TO_PLAY: u32 = 0x0060_0950;
/// `TESIdleForm::GetIdleAnimGroupSection` (Xbox PDB).
const IDLE_GET_ANIM_GROUP_SECTION: u32 = 0x005f_f160;
/// Formatted message output (cdecl, variable words).
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// `Pathing::FindSafeStraightLinePathPoint` (Xbox PDB), cdecl, `(actor, target, reach, out)`.
const FIND_SAFE_PATH_POINT: u32 = 0x006d_3780;
/// `TESObjectREFR::SetLocationOnReference` (Xbox PDB): one position pointer.
const SET_LOCATION_ON_REFERENCE: u32 = 0x0057_5830;
/// `Animation::UpdateSceneGraphNoController` (Xbox PDB).
const UPDATE_SCENE_GRAPH: u32 = 0x0049_3bd0;
/// `CombatUtilities::GetWeaponReach` (Xbox PDB), cdecl `(actor, weapon)`, returns `ST0`.
const GET_WEAPON_REACH: u32 = 0x009a_69c0;
/// Byte result on a weapon (body not read); when true the reach is scaled by a second setting.
const WEAPON_TEST_006450C0: u32 = 0x0064_50c0;
/// `bhkCollisionFilter::SetMultiLayerCollision` (Xbox PDB), cdecl, three words.
const SET_MULTI_LAYER_COLLISION: u32 = 0x00c8_2870;
/// `GetAVObjectForCollidable` (Xbox PDB), cdecl, one word.
const GET_AV_OBJECT_FOR_COLLIDABLE: u32 = 0x004b_5820;
/// `TESObjectREFR::FindReferenceFor3D` (Xbox PDB), cdecl, one word.
const FIND_REFERENCE_FOR_3D: u32 = 0x0056_f930;
/// `MobileObject::GetCharController` (Xbox PDB).
const GET_CHAR_CONTROLLER: u32 = 0x0093_06d0;
/// Collision group of a character controller (body not read).
const CONTROLLER_COLLISION_GROUP: u32 = 0x0081_9250;
/// Collision group used without a controller (no words; body not read).
const WORLD_COLLISION_GROUP: u32 = 0x005d_be10;
/// Ray-cast filter-info word: constructor (one word), set layer (one word), set group (one word).
const FILTER_INFO_CONSTRUCTOR: u32 = 0x008c_71b0;
const FILTER_INFO_SET_LAYER: u32 = 0x004a_39f0;
const FILTER_INFO_SET_GROUP: u32 = 0x0059_ce80;
/// Ray query (0xb0 bytes): constructor and the setters the line-of-sight code calls (one word
/// each: the filter-info value, the from and to positions, the collector).
const RAY_QUERY_CONSTRUCTOR: u32 = 0x004a_3c20;
const RAY_QUERY_SET_FILTER: u32 = 0x004a_3f70;
const RAY_QUERY_SET_FROM: u32 = 0x004a_3da0;
const RAY_QUERY_SET_TO: u32 = 0x004a_3eb0;
const RAY_QUERY_SET_COLLECTOR: u32 = 0x004a_3fb0;
/// `hkpAllRayHitCollector::hkpAllRayHitCollector` and `~hkpAllRayHitCollector` (Xbox PDB).
const RAY_COLLECTOR_CONSTRUCTOR: u32 = 0x004a_3a70;
const RAY_COLLECTOR_DESTRUCTOR: u32 = 0x004a_3bc0;
/// The hit list of the query is `RAY_HITS_SECOND(RAY_HITS_FIRST(query))`; its count is
/// `LIST_NEXT` of it (`[list + 4]`) and `RAY_HIT_AT(list, index)` is one hit.
const RAY_HITS_FIRST: u32 = 0x008c_dd90;
const RAY_HITS_SECOND: u32 = 0x0046_0140;
const RAY_HIT_AT: u32 = 0x004a_46b0;
/// Builds a hit record in an output block from a hit (one word); the record holds the collidable
/// at `+0x50`.
const HIT_RECORD_CONSTRUCTOR: u32 = 0x0069_6c30;
/// Process to the object the ray cast runs on (body not read).
const ACTOR_CONTROLLER_BASE: u32 = 0x0045_43c0;

/// `NiPoint3::NiPoint3(x, y, z)` (returns `this`).
const POINT3_CONSTRUCTOR: u32 = 0x0041_6870;
/// `NiPoint2::NiPoint2(x, y)` (returns `this`).
const POINT2_CONSTRUCTOR: u32 = 0x0045_2dc0;
/// Vector arithmetic (bodies read): `out = this + rhs`, `out = this - rhs`, `out = this * s`,
/// `out = this * (1 / s)` (all `(out, operand)` after `this`, returning `out`), `this *= s` and
/// `this += rhs` (one operand after `this`, returning `this`), and `this == rhs` (byte result).
const POINT3_ADD: u32 = 0x0043_9e90;
const POINT3_SUBTRACT: u32 = 0x0043_9ef0;
const POINT3_TIMES: u32 = 0x0045_bb20;
const POINT3_TIMES_RECIPROCAL: u32 = 0x0053_d280;
const POINT3_SCALE: u32 = 0x0043_9180;
const POINT3_ADD_ASSIGN: u32 = 0x0063_c8a0;
const POINT3_EQUALS: u32 = 0x0043_90c0;
/// Vector length (`ST0`).
const POINT3_LENGTH: u32 = 0x0045_7990;
/// `NiPoint3::UnitizeGetLength` (Xbox PDB): normalizes in place, returns the length in `ST0`.
const POINT3_UNITIZE: u32 = 0x0045_7910;
/// Builds a matrix from an angle (one float after `this`; body not read).
const MATRIX_FROM_ANGLE: u32 = 0x004a_0c90;
/// Normalizes the vector `this` in place (calls the vector length; body not read further).
const VECTOR_NORMALIZE: u32 = 0x004a_0c10;
/// Cdecl `(out, vector, matrix)` returning `out` (body not read; the shape is from the call).
const MATRIX_TIMES_VECTOR: u32 = 0x004b_3ae0;
/// `NiAVObject::GetWorldBound` (Xbox PDB); the centre vector sits at the returned object's item
/// slot (`LIST_ITEM_SLOT`).
const GET_WORLD_BOUND: u32 = 0x0043_d450;
/// Actor to its model object (no stack words; body not read).
const ACTOR_MODEL_OBJECT: u32 = 0x0043_fcd0;
/// Object to the part the position math uses (no stack words; body not read).
const BOUND_NEXT_PART: u32 = 0x0045_bb80;
/// `Actor::GetHeight` (Xbox PDB), returns `ST0`.
const ACTOR_GET_HEIGHT: u32 = 0x0088_53a0;
/// `FLD [this + 0x698]` on the player (body read).
const PLAYER_FLOAT: u32 = 0x0070_c3b0;
/// Cdecl, five floats `(a, b, c, d, v)`: `((v - c) / (d - c)) * (b - a) + a` (body read).
const REMAP_FLOAT: u32 = 0x004b_3ab0;
/// Cdecl, two floats: `b < a ? a : b` (body read).
const FLOAT_MAX: u32 = 0x0040_4010;
/// Cdecl, two floats: `b <= a ? b : a` (body read).
const FLOAT_MIN: u32 = 0x0040_ebd0;
/// Cdecl, one index: `[0x011f91c8 + index * 4]` (body read); index 0 is the scene node used here.
const SCENE_NODE_BY_INDEX: u32 = 0x0045_0b80;
/// `ShadowSceneNode::AddLight` (Xbox PDB): `(light, 1)`.
const ADD_LIGHT: u32 = 0x00b5_c940;
/// `ShadowSceneNode::RemoveLight` (Xbox PDB): one word.
const REMOVE_LIGHT: u32 = 0x00b5_eed0;
/// Light placement trace: ten words after `this`, returns `ST0` (body not read).
const LIGHT_TRACE: u32 = 0x00b5_b610;
/// Calls the smart pointer getter on a field of its object (body not read); the light setters
/// take its result.
const LIGHT_HANDLE: u32 = 0x004e_6540;
/// Light object setters (one pointer, or one float, each; bodies not read).
const LIGHT_SET_POSITION: u32 = 0x004f_00e0;
const LIGHT_SET_DIRECTION: u32 = 0x0050_dd50;
const LIGHT_SET_COLOUR: u32 = 0x004b_c2e0;
const LIGHT_SET_DIMMER: u32 = 0x0045_0f90;
/// Returns `[this]` (the pointer a smart pointer holds).
const SMART_POINTER_GET: u32 = 0x0055_9450;
/// Smart pointer assignment (one word; releases the old pointer, keeps the new one).
const SMART_POINTER_ASSIGN: u32 = 0x0066_b0d0;
/// Returns `[this] != argument` (body read).
const SMART_POINTER_DIFFERS: u32 = 0x0052_aa80;
/// `GetVATSLight` of the `VATS` object: `SMART_POINTER_GET(this + 0x34)` (body read).
const GET_VATS_LIGHT: u32 = 0x005d_8710;
/// Allocator for engine objects (cdecl, one size word).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// Constructor of the 0xfc-byte light object (returns `this`).
const LIGHT_CONSTRUCTOR: u32 = 0x00a7_d670;
/// No stack words: returns the object the mode-2 change marks (body not read).
const INTERFACE_OBJECT_GETTER: u32 = 0x0070_5910;
/// Sets the byte at `this + 0x2b5` when the byte at `+0x2b4` is set (body read).
const INTERFACE_MARK_DIRTY: u32 = 0x0080_09d0;
/// Getter on the object at `0x011dea0c` (tail-calls `007fdf30`).
const HOLDER_GET_OBJECT: u32 = 0x0087_7720;
/// Two calls on that getter's result with the number 6 (one word each; bodies not read).
const HOLDER_OBJECT_SELECT: u32 = 0x00a2_5410;
const HOLDER_OBJECT_APPLY: u32 = 0x00a2_4280;
/// Actor to its base object (no stack words; body not read).
const REFERENCE_BASE: u32 = 0x0041_81e0;
/// Body-part data getter (no stack words; body not read).
const BODY_PART_DATA: u32 = 0x006a_9540;
/// `BGSBodyPartData::GetBodyPart_ov2` and `GetBodyPart` (Xbox PDB): one word after `this`.
const GET_BODY_PART_BY_LIMB: u32 = 0x005e_5130;
const GET_BODY_PART: u32 = 0x005e_50f0;
/// `TESObjectREFR::GetDismembered` (Xbox PDB): one limb word.
const GET_DISMEMBERED: u32 = 0x0057_3090;
/// Body part to its node name (no stack words; body not read).
const BODY_PART_NODE_NAME: u32 = 0x0043_b230;
/// Cdecl `(root object, node name)`: finds the node (body not read).
const FIND_NODE_BY_NAME: u32 = 0x004a_ae30;
/// Actor to the argument the process virtual call `+0x190` takes (returns 0; body read).
const ACTOR_FACE_VALUE: u32 = 0x00ac_bb70;
/// Bounds-checked element getter at `this + 0x9c` (one index word).
const ARRAY_ELEMENT_BY_INDEX: u32 = 0x0045_bc00;
/// Animation to an action object (one word, 4); the action object to its record; two tests on the
/// record (bodies read: each forwards a 16-bit field to a cdecl test).
const ANIMATION_ACTION_OBJECT: u32 = 0x0049_1040;
const ACTION_RECORD: u32 = 0x0048_f7f0;
const ACTION_RECORD_TEST_FIRST: u32 = 0x0049_37c0;
const ACTION_RECORD_TEST_SECOND: u32 = 0x0049_37e0;
/// Animation: count for a kind (one word, 4; body read: an indexed table lookup).
const ANIMATION_KIND_COUNT: u32 = 0x0070_f490;
/// Animation: a 16-bit value for a kind (one word, 4); and the cdecl test on it.
const ANIMATION_SLOT_VALUE: u32 = 0x0043_01b0;
const SLOT_VALUE_TEST: u32 = 0x005f_25d0;
/// Queued idle: finished test (reads `this + 0x18` through the smart pointer getter).
const QUEUED_IDLE_FINISHED: u32 = 0x0049_0e40;
/// Actor state tests (no stack words; bodies not read).
const ACTOR_STATE_TEST_FIRST: u32 = 0x0043_7bf0;
const ACTOR_STATE_TEST_SECOND: u32 = 0x0043_7bd0;
/// Starts the action for the player: `(0x400, flag)`; for another actor: `(flag)`.
const PLAYER_START_ACTION: u32 = 0x0096_6ea0;
const ACTOR_START_ACTION: u32 = 0x005c_e9d0;
/// Actor state get and set (one word for the setter; bodies not read).
const ACTOR_GET_FLAG_STATE: u32 = 0x008a_6970;
const ACTOR_SET_FLAG_STATE: u32 = 0x008a_6840;
/// `ItemChange::HasModEffectActive_ov2` (Xbox PDB): one word after `this`.
const ITEM_HAS_MOD_EFFECT: u32 = 0x004b_da70;
/// Reference flag test (one word) and change (two words) after `this`.
const REFERENCE_FLAG_TEST: u32 = 0x0044_4d00;
const REFERENCE_FLAG_CHANGE: u32 = 0x005d_e0a0;
/// Hit data: sets a counter (two words after `this`; body not read).
const HIT_DATA_SET_COUNTER: u32 = 0x0040_7e00;
/// Value read off a target (returns `ST0`; body not read).
const TARGET_LAST_VALUE: u32 = 0x0056_8ad0;
/// Reads the byte at `0x012677a3` (body read), no words.
const INTERFACE_ENABLED: u32 = 0x0088_8960;

// ---- the VATS object and its statics ----------------------------------------------------------

/// The `VATS` singleton.
pub(crate) const VATS_SINGLETON: u32 = 0x011f_2250;

/// `PlayerCharacter` pointer (`PlayerCharacter::pSingleton`).
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// Object whose getter `00877720` the update and the mode change call.
const INTERFACE_HOLDER: u32 = 0x011d_ea0c;
/// Object at `0x011ca700`, `this` of the shot selection.
const SHOT_SELECTOR: u32 = 0x011c_a700;
/// Object at `0x011cb6a0`, `this` of `TESIdleManager::GetIdleToPlay`.
const IDLE_MANAGER_POINTER: u32 = 0x011c_b6a0;
/// The timer object, `this` of `00aa4db0` and `0084d030`.
const TIMER_OBJECT: u32 = 0x011f_6394;
/// Weapon used when the player has none.
const DEFAULT_WEAPON: u32 = 0x011c_a278;

// The statics of vats.cpp, by role (read from their use; the names are not from the PDB).
/// Camera stage of the running command (0, 1 or 2), compared with the camera shot's count.
const CAMERA_STAGE: u32 = 0x011f_21b8;
/// Shot forced for the next command; and a second forced shot that is cleared when used.
const FORCED_SHOT_FIRST: u32 = 0x011f_21c0;
const FORCED_SHOT_SECOND: u32 = 0x011f_21c4;
/// The camera kind chosen from the shot's kind tests (0, 1 or 2), passed to `InitCamera`.
const CAMERA_KIND: u32 = 0x011f_21c8;
/// Target reference of the command being played.
const TARGET_REFERENCE: u32 = 0x011f_21cc;
/// Byte set when the mode returns to 0.
const MENU_MODE_CHANGED: u32 = 0x011f_21d0;
/// Byte: the head action has started.
const ACTION_STARTED: u32 = 0x011f_21d2;
/// Byte: the first-person view must be switched off again.
const FIRST_PERSON_PENDING: u32 = 0x011f_21d3;
/// Float: playback time.
const PLAYBACK_TIME: u32 = 0x011f_21d4;
/// Float: time since the current camera shot began.
const PLAYBACK_SHOT_TIME: u32 = 0x011f_21d8;
/// The shot picked for the command (0 or a shot object).
const SELECTED_SHOT: u32 = 0x011f_21dc;
/// Float: countdown that ends the playback.
const END_COUNTDOWN: u32 = 0x011f_21e0;
/// Byte: the player was put into the action state of `HOLDER_OBJECT_APPLY` during an attack.
const SLOW_MOTION_ACTION: u32 = 0x011f_21e4;
/// Actor the commands with `VATS_COMMAND::bStranger` (Xbox PDB) set run on (the global's own name
/// is unconfirmed).
const STRANGER_ACTOR: u32 = 0x011f_21e8;
/// Actor the commands with `VATS_COMMAND::bMissFortune` (Xbox PDB) set run on (unconfirmed).
const MISS_FORTUNE_ACTOR: u32 = 0x011f_21ec;
/// Actor of the command being played (the player, or one of the two above).
const CURRENT_ACTOR: u32 = 0x011f_21f0;
/// Counter the update resets once it is above 1.
const LAST_COMMAND_COUNTER: u32 = 0x011f_231c;
/// The command the camera was last set up for.
const LAST_COMMAND: u32 = 0x011f_2320;
/// Byte written by `fn_009c86a0` (`mode == 4`); selects between two floats.
const PLAYING_BACK_FLAG: u32 = 0x0126_77cc;
/// The float `fn_009c8ea0` compares with.
const PICK_TIME_LIMIT: u32 = 0x0126_77c4;
/// The two floats `fn_009c8ef0` returns, for the flag clear and set.
const PICK_TIME_FLOAT_OFF: u32 = 0x011a_fe74;
const PICK_TIME_FLOAT_ON: u32 = 0x011a_fe78;
/// Byte written by `fn_009c7190`.
const MODE_TWO_FLAG: u32 = 0x011f_9424;
/// Three floats: the default look-at position and the vector the shot position is compared with.
const DEFAULT_POSITION: u32 = 0x011f_426c;
/// Slow-motion state: a byte, the saved time and the blend factor.
const SLOW_MOTION_ACTIVE: u32 = 0x011c_a728;
const SLOW_MOTION_SAVED_TIME: u32 = 0x011c_a72c;
const SLOW_MOTION_BLEND: u32 = 0x011c_a730;

// Settings: the address of the setting object, passed as `this` to the value getters.
const QUIT_DELAY_SETTING: u32 = 0x011c_e338;
const MIN_SHOT_TIME_SETTING: u32 = 0x011c_e368;
const SHOT_IDLE_LIMIT_SETTING: u32 = 0x011c_e8c4;
const STRANGER_REACH_SETTING: u32 = 0x011c_e888;
const REACH_FACTOR_NO_WEAPON_SETTING: u32 = 0x011c_f138;
const REACH_FACTOR_WEAPON_TEST_SETTING: u32 = 0x011c_f748;
const PLAYER_MULT_SETTING: u32 = 0x011c_f374;
const TARGET_MULT_SETTING: u32 = 0x011c_fcec;
const MODIFIER_STRENGTH_SETTING: u32 = 0x011c_f700;
const MODIFIER_LIMIT_SETTING: u32 = 0x011c_fb04;
/// Integer setting: the distance added to a target's bound in the camera stage decision.
const CAMERA_DISTANCE_SETTING: u32 = 0x011c_f81c;
/// Integer setting `FreeHeadAction` compares a target's value with.
const HEAD_ACTION_LIMIT_SETTING: u32 = 0x011c_e7e4;
/// The four float settings of the mode-4 transition modifier.
const TRANSITION_SETTING_A: u32 = 0x011c_e6d8;
const TRANSITION_SETTING_B: u32 = 0x011c_f1f8;
const TRANSITION_SETTING_C: u32 = 0x011c_e424;
const TRANSITION_SETTING_D: u32 = 0x011c_e564;
/// The eight settings the light placement reads (they sit in the class's static `SettingT` block,
/// Xbox PDB `VATS::fVATSLight*` / `fVatsLightColor*`; which is which is not confirmed).
const LIGHT_SETTING_011F2304: u32 = 0x011f_2304;
const LIGHT_SETTING_011F222C: u32 = 0x011f_222c;
const LIGHT_SETTING_011F220C: u32 = 0x011f_220c;
const LIGHT_SETTING_011F22CC: u32 = 0x011f_22cc;
const LIGHT_SETTING_011F22A8: u32 = 0x011f_22a8;
const LIGHT_SETTING_011F21F4: u32 = 0x011f_21f4;
const LIGHT_SETTING_011F2200: u32 = 0x011f_2200;
const LIGHT_SETTING_011F2298: u32 = 0x011f_2298;

// Constants of the data section.
const ZERO_DOUBLE: u32 = 0x0101_2060;
const ONE_DOUBLE: u32 = 0x0101_2070;
const SIXTY_DOUBLE: u32 = 0x0101_2638;
const HALF_DOUBLE: u32 = 0x0101_1588;
const SHOT_TIME_THRESHOLD: u32 = 0x0101_6978;
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
const SLOW_FLOAT_A: u32 = 0x0101_41cc;
const SLOW_FLOAT_B: u32 = 0x0101_7870;
const HALF_FLOAT: u32 = 0x0101_6248;
const OFFSET_FLOAT: u32 = 0x0101_6410;
/// `"COMBAT: Bad VATS Melee Attack '%s' in the idle manager. ..."`.
const BAD_MELEE_ATTACK_FORMAT: u32 = 0x0109_06b0;

layout! {
    /// `VATS` (Xbox PDB), 0x48 bytes. The `commandList` (a `BSSimpleList<VATS_COMMAND *>`) is the
    /// first two words, so the object doubles as the list's head node in the list calls.
    pub struct Vats: 0x48 {
        /// `eMode` (Xbox PDB): 0 none, 1 target select, 2 menu, 3 scan, 4 playback.
        0x08 eMode: i32,
        /// `pCameraList` (Xbox PDB): `BSSimpleList<BGSCameraShot *>*`.
        0x0C pCameraList: Ptr,
        /// `pCamera` (Xbox PDB): `BGSCameraShot*`.
        0x10 pCamera: Ptr,
        /// `fCameraTimer` (Xbox PDB).
        0x14 fCameraTimer: f32,
        /// `fCameraMinTimer` (Xbox PDB).
        0x18 fCameraMinTimer: f32,
        /// `pFirstProjectile` (Xbox PDB).
        0x1C pFirstProjectile: Ptr,
        /// `pLastProjectile` (Xbox PDB).
        0x20 pLastProjectile: Ptr,
        /// `pMeleeAttack` (Xbox PDB): `TESIdleForm*`.
        0x24 pMeleeAttack: Ptr,
        /// `pImagespaceMod` (Xbox PDB).
        0x28 pImagespaceMod: Ptr,
        /// `pTransitionImagespaceMod` (Xbox PDB).
        0x2C pTransitionImagespaceMod: Ptr,
        /// `bAiming` (Xbox PDB).
        0x30 bAiming: bool,
        /// `spVATSLight` (Xbox PDB): `NiPointer<ShadowSceneLight>`.
        0x34 spVATSLight: Ptr,
        /// `bFirstVatsAttack` (Xbox PDB).
        0x38 bFirstVatsAttack: bool,
        /// `iKillCount` (Xbox PDB).
        0x3C iKillCount: i32,
        /// `pUnEquipWeapon` (Xbox PDB): `TESBoundObject*`.
        0x40 pUnEquipWeapon: Ptr,
        /// `pUnEquipExtra` (Xbox PDB): `ExtraDataList*`.
        0x44 pUnEquipExtra: Ptr,
    }

    /// `VATS_COMMAND` (Xbox PDB), 0x28 bytes.
    pub struct VatsCommand: 0x28 {
        /// `eAction` (Xbox PDB): `ActionPoints::Action`.
        0x00 eAction: u32,
        /// `bSuccess` (Xbox PDB).
        0x04 bSuccess: bool,
        /// `bIronSights` (Xbox PDB).
        0x05 bIronSights: bool,
        /// `bStranger` (Xbox PDB).
        0x06 bStranger: bool,
        /// `bParalyzingPalm` (Xbox PDB).
        0x07 bParalyzingPalm: bool,
        /// `cFireShots` (Xbox PDB).
        0x08 cFireShots: u8,
        /// `cFakeFireShots` (Xbox PDB).
        0x09 cFakeFireShots: u8,
        /// `pTarget` (Xbox PDB): `TESObjectREFR*`.
        0x0C pTarget: Ptr,
        /// `eLimb` (Xbox PDB): `ActorValue::Index`.
        0x10 eLimb: i32,
        /// `pHitData` (Xbox PDB): `HitData*`.
        0x14 pHitData: Ptr,
        /// `fTotalDamage` (Xbox PDB).
        0x18 fTotalDamage: f32,
        /// `fCumulativeDamage` (Xbox PDB).
        0x1C fCumulativeDamage: f32,
        /// `fActionPoints` (Xbox PDB).
        0x20 fActionPoints: f32,
        /// `bMissFortune` (Xbox PDB).
        0x24 bMissFortune: bool,
    }
}

// ---- helpers ----------------------------------------------------------------------------------

/// The high byte of the x87 status word (`FNSTSW AX`, then `AH`) after `FCOM st0, operand`: C0
/// (`0x01`) when `st0 < operand`, C3 (`0x40`) when equal, nothing when `st0 > operand`, and
/// C0 | C2 | C3 (`0x45`) when either is a NaN.
fn x87_compare(st0: f64, operand: f64) -> u8 {
    if st0.is_nan() || operand.is_nan() {
        0x45
    } else if st0 > operand {
        0x00
    } else if st0 < operand {
        0x01
    } else {
        0x40
    }
}

/// The parity flag `TEST AH, mask` leaves (set when the masked byte has an even number of 1 bits),
/// which `JP` / `JNP` test.
fn parity_even(value: u8) -> bool {
    value.count_ones() % 2 == 0
}

fn list_is_empty(e: &mut Engine, node: u32) -> bool {
    e.call(LIST_IS_EMPTY, &args![node]).bool()
}

/// The item of a list node (`[ListItemSlot(node)]`).
fn list_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
    e.mem.u32(slot)
}

fn list_next(e: &mut Engine, node: u32) -> u32 {
    e.call(LIST_NEXT, &args![node]).u32()
}

fn read_vec3(e: &Engine, address: u32) -> [u32; 3] {
    [
        e.mem.u32(address),
        e.mem.u32(address.wrapping_add(4)),
        e.mem.u32(address.wrapping_add(8)),
    ]
}

fn write_vec3(e: &mut Engine, address: u32, value: [u32; 3]) {
    e.mem.set_u32(address, value[0]);
    e.mem.set_u32(address.wrapping_add(4), value[1]);
    e.mem.set_u32(address.wrapping_add(8), value[2]);
}

/// Copies a float with `FLD` / `FSTP` (a signalling NaN is quieted).
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

/// A float setting the way the game pushes it (`FLD [value]`, `FSTP [ESP]`).
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_FLOAT_POINTER, &args![setting]).u32();
    f32::from_bits(x87_quiet_bits(e.mem.u32(value)))
}

/// `frame time / divisor` as the update code computes it (`FSTP double`, `FDIVR double`).
fn frame_time(e: &mut Engine) -> f64 {
    let delta = e.call(TIMER_DELTA, &args![TIMER_OBJECT]).f64();
    let divisor = e.call(TIMER_DIVISOR, &args![]).f64();
    delta / divisor
}

fn set_time_multiplier(e: &mut Engine, multiplier: f32, flag: u32) {
    e.call(
        SET_GLOBAL_TIME_MULTIPLIER,
        &args![TIMER_OBJECT, multiplier, flag],
    );
}

fn stop_modifier(e: &mut Engine, this: Ptr<Vats>, field: Field<Vats, Ptr>) {
    let modifier = e.get(this, field);
    if !modifier.is_null() {
        e.call(IMAGESPACE_STOP, &args![modifier]);
        e.set(this, field, Ptr::NULL);
    }
}

/// The light held by the `VATS` object, as `00559450(this + 0x34)` reads it.
fn held_light(e: &mut Engine, this: Ptr<Vats>) -> u32 {
    e.call(SMART_POINTER_GET, &args![this.addr().wrapping_add(0x34)])
        .u32()
}

fn held_light_handle(e: &mut Engine, this: Ptr<Vats>) -> u32 {
    let light = held_light(e, this);
    e.call(LIGHT_HANDLE, &args![light]).u32()
}

// ---- the translated functions -----------------------------------------------------------------

// Translated from 005ae380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::GetCount` (Xbox PDB; the body is a plain list count, so the linker may have folded it
/// with other list code): the number of nodes of the list whose item is not null.
pub fn vats_get_count(e: &mut Engine, this: Ptr<Vats>) -> i32 {
    let mut count = 0;
    let mut node = this.addr();
    while node != 0 {
        if list_item(e, node) != 0 {
            count += 1;
        }
        node = list_next(e, node);
    }
    count
}

// Translated from 009c6ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::ClearData` (Xbox PDB): clears the command list, frees the camera shot and resets the
/// camera fields, and stops the two image-space modifiers.
pub fn vats_clear_data(e: &mut Engine, this: Ptr<Vats>) {
    vats_clear_command_list(e, this);
    let camera = e.get(this, Vats::pCamera);
    if !camera.is_null() {
        e.call(FREE_CAMERA, &args![camera]);
    }
    e.set(this, Vats::pCameraList, Ptr::NULL);
    e.set(this, Vats::pCamera, Ptr::NULL);
    e.set(this, Vats::fCameraTimer, 0.0f32);
    e.set(this, Vats::fCameraMinTimer, 0.0f32);
    stop_modifier(e, this, Vats::pImagespaceMod);
    stop_modifier(e, this, Vats::pTransitionImagespaceMod);
}

// Translated from 009c6c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::SetMode` (Xbox PDB): changes the mode (`force` runs the change even when the mode is the
/// same). The player becomes the current actor, then per mode: 0 (none) destroys the two special
/// actors, resets the timers, the camera and the image-space modifiers and un-equips the stored
/// weapon; 2 (menu) and 3 (scan) clear the stored weapon and create the VATS light when there is
/// none; 4 (playback) clears the counters, stops the modifiers and starts the transition modifier.
/// After the switch the mode is stored (a menu flag is set when it is 2) and, for mode 0, the HUD
/// menu mode is restored. Modes 2 and 3 reposition the light for the target reference; mode 0
/// removes the light from the scene.
// Translation incomplete: 009c6c30 - C++ exception-unwinding frame (FS:[0], handler 00f1b4c4, states 0 and 1 around the light allocation) not translated
pub fn vats_set_mode(e: &mut Engine, this: Ptr<Vats>, mode: i32, force: u8) {
    let current_mode = e.get(this, Vats::eMode);
    if mode != current_mode || force != 0 {
        let player = e.global::<u32>(PLAYER_POINTER);
        e.set_global(CURRENT_ACTOR, player);
        match mode as u32 {
            0 => {
                for special in [STRANGER_ACTOR, MISS_FORTUNE_ACTOR] {
                    let actor = e.global::<u32>(special);
                    if actor != 0 {
                        e.vcall(actor, 0x10, &args![1u32]);
                        e.set_global(special, 0u32);
                    }
                }
                set_time_multiplier(e, 1.0, 1);
                e.set_global(FORCED_SHOT_FIRST, 0u32);
                e.set_global(FORCED_SHOT_SECOND, 0u32);
                e.set_global(CAMERA_KIND, 0u32);
                let camera = e.get(this, Vats::pCamera);
                if !camera.is_null() {
                    e.call(FREE_CAMERA, &args![camera]);
                }
                e.set_global(CAMERA_STAGE, 0u32);
                e.set(this, Vats::pCameraList, Ptr::NULL);
                e.set(this, Vats::pCamera, Ptr::NULL);
                e.set(this, Vats::fCameraTimer, 0.0f32);
                e.set(this, Vats::fCameraMinTimer, 0.0f32);
                e.set(this, Vats::pFirstProjectile, Ptr::NULL);
                e.set(this, Vats::pLastProjectile, Ptr::NULL);
                e.set_global(SELECTED_SHOT, 0u32);
                e.set_global(FIRST_PERSON_PENDING, 0u8);
                stop_modifier(e, this, Vats::pImagespaceMod);
                stop_modifier(e, this, Vats::pTransitionImagespaceMod);
                e.set(this, Vats::bFirstVatsAttack, true);
                e.call(ACTOR_SET_IRON_SIGHTS, &args![player, 0u32, 0u32, 0u32]);
                e.call(ACTOR_SET_BLOCK, &args![player, 0u32]);
                let holder = e.global::<u32>(INTERFACE_HOLDER);
                let holder_object = e.call(HOLDER_GET_OBJECT, &args![holder]).u32();
                e.call(HOLDER_OBJECT_SELECT, &args![holder_object, 6u32]);
                let weapon = e.get(this, Vats::pUnEquipWeapon);
                if !weapon.is_null() {
                    let extra = e.get(this, Vats::pUnEquipExtra);
                    e.call(
                        ACTOR_UNEQUIP_OBJECT,
                        &args![player, weapon, 1u32, extra, 0u32, 1u32, 1u32],
                    );
                    e.set(this, Vats::pUnEquipWeapon, Ptr::NULL);
                    e.set(this, Vats::pUnEquipExtra, Ptr::NULL);
                }
            }
            2 => {
                e.set(this, Vats::pUnEquipWeapon, Ptr::NULL);
                e.set(this, Vats::pUnEquipExtra, Ptr::NULL);
                let interface = e.call(INTERFACE_OBJECT_GETTER, &args![]).u32();
                e.call(INTERFACE_MARK_DIRTY, &args![interface]);
                create_light_when_missing(e);
            }
            3 => {
                e.set(this, Vats::pUnEquipWeapon, Ptr::NULL);
                e.set(this, Vats::pUnEquipExtra, Ptr::NULL);
                create_light_when_missing(e);
            }
            4 => {
                e.set(this, Vats::pUnEquipWeapon, Ptr::NULL);
                e.set(this, Vats::pUnEquipExtra, Ptr::NULL);
                e.set(this, Vats::iKillCount, 0);
                e.set_global(PLAYBACK_TIME, 0.0f32);
                e.set_global(PLAYBACK_SHOT_TIME, 0.0f32);
                e.set_global(ACTION_STARTED, 0u8);
                e.set_global(SLOW_MOTION_ACTION, 0u8);
                stop_modifier(e, this, Vats::pImagespaceMod);
                stop_modifier(e, this, Vats::pTransitionImagespaceMod);
                // The point (0.5, 0.5) is a local of the exe; its constructor takes two floats and
                // the third word pushed with them (0.0) stays on the stack for the trigger call.
                let point = e.mem.alloc(8);
                let half = f32::from_bits(x87_quiet_bits(e.mem.u32(HALF_FLOAT)));
                e.call(POINT2_CONSTRUCTOR, &args![point, half, half]);
                let setting_a = setting_float(e, TRANSITION_SETTING_A);
                let setting_b = setting_float(e, TRANSITION_SETTING_B);
                let setting_c = setting_float(e, TRANSITION_SETTING_C);
                let setting_d = setting_float(e, TRANSITION_SETTING_D);
                // Seven cdecl words, the last pushed first: 0.0, the four settings in reverse, the
                // point, and the 0.0 pushed before the constructor call.
                let transition = e
                    .call(
                        IMAGESPACE_RB_TRIGGER,
                        &args![0.0f32, setting_d, setting_c, setting_b, setting_a, point, 0.0f32],
                    )
                    .u32();
                e.mem.free(point);
                e.set(this, Vats::pTransitionImagespaceMod, Ptr::new(transition));
                if transition != 0 {
                    e.call(IMAGESPACE_ACTIVATE, &args![transition, 1u32]);
                }
            }
            _ => {}
        }
        fn_009c7190(e, (mode == 2) as u8);
        e.set(this, Vats::eMode, mode);
        if mode == 0 {
            e.call(HUD_SET_MENU_MODE, &args![1u32]);
            e.set_global(MENU_MODE_CHANGED, 1u8);
        }
    }
    if mode == 0 {
        if e.call(GET_VATS_LIGHT, &args![VATS_SINGLETON]).u32() != 0 {
            let light = e.call(GET_VATS_LIGHT, &args![VATS_SINGLETON]).u32();
            let handle = e.call(LIGHT_HANDLE, &args![light]).u32();
            let node = e.call(SCENE_NODE_BY_INDEX, &args![0u32]).u32();
            e.call(REMOVE_LIGHT, &args![node, handle]);
            let node = e.call(SCENE_NODE_BY_INDEX, &args![0u32]).u32();
            let light = e.call(GET_VATS_LIGHT, &args![VATS_SINGLETON]).u32();
            let handle = e.call(LIGHT_HANDLE, &args![light]).u32();
            e.vcall(node, 0xe8, &args![handle]);
            vats_set_vats_light(e, Ptr::new(VATS_SINGLETON), 0, 1);
        }
    } else if (2..4).contains(&mode) {
        let target = e.global::<u32>(TARGET_REFERENCE);
        if target != 0 {
            fn_009c9ae0(e, Ptr::new(VATS_SINGLETON), target);
        }
    }
}

/// The light creation `VATS::SetMode` does for the modes 2 and 3: when the `VATS` object holds no
/// light, a 0xfc-byte light is allocated and constructed and handed to `SetVATSLight`.
fn create_light_when_missing(e: &mut Engine) {
    if e.call(GET_VATS_LIGHT, &args![VATS_SINGLETON]).u32() == 0 {
        let block = e.call(NI_ALLOCATE, &args![0xfcu32]).u32();
        let light = if block != 0 {
            e.call(LIGHT_CONSTRUCTOR, &args![block]).u32()
        } else {
            0
        };
        vats_set_vats_light(e, Ptr::new(VATS_SINGLETON), light, 1);
    }
}

// Translated from 009c7190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a flag byte in the global at `0x011f9424` (set to `mode == 2` by `VATS::SetMode`).
pub fn fn_009c7190(e: &mut Engine, flag: u8) {
    e.set_global(MODE_TWO_FLAG, flag);
}

// Translated from 009c71a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a command to the command list: the list method takes the address of a stack slot that
/// holds the item.
pub fn fn_009c71a0(e: &mut Engine, this: Ptr<Vats>, command: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), command);
        e.call(LIST_APPEND, &args![this, slot]);
    });
}

// Translated from 009c71c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::GetCurrentAction` (Xbox PDB): the first command of the list, or null when it is empty.
pub fn vats_get_current_action(e: &mut Engine, this: Ptr<Vats>) -> u32 {
    if list_is_empty(e, this.addr()) {
        0
    } else {
        list_item(e, this.addr())
    }
}

// Translated from 009c71f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The last command of the list: walks the nodes while they are not empty and returns the item of
/// the node that has no next node; 0 when the list is empty.
pub fn fn_009c71f0(e: &mut Engine, this: Ptr<Vats>) -> u32 {
    let mut node = this.addr();
    while node != 0 && !list_is_empty(e, node) {
        if list_next(e, node) == 0 {
            return list_item(e, node);
        }
        node = list_next(e, node);
    }
    0
}

// Translated from 009c8600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Camera shot (`this` = shot): bit `0x08` of the flags word at `+0x44`.
pub fn fn_009c8600(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr().wrapping_add(0x44)) & 8 != 0
}

// Translated from 009c8620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Camera shot (`this` = shot): bit `0x10` of the flags word at `+0x44`.
pub fn fn_009c8620(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr().wrapping_add(0x44)) & 0x10 != 0
}

// Translated from 009c8640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Camera shot: the smart pointer at `+0x60` (read through its getter `00559450`).
pub fn fn_009c8640(e: &mut Engine, this: Ptr) -> u32 {
    e.call(SMART_POINTER_GET, &args![this.addr().wrapping_add(0x60)])
        .u32()
}

// Translated from 009c8660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Camera shot: the smart pointer at `+0x64` (read through its getter `00559450`).
pub fn fn_009c8660(e: &mut Engine, this: Ptr) -> u32 {
    e.call(SMART_POINTER_GET, &args![this.addr().wrapping_add(0x64)])
        .u32()
}

// Translated from 009c8680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Camera shot: the smart pointer at `+0x70` (read through its getter `00559450`).
pub fn fn_009c8680(e: &mut Engine, this: Ptr) -> u32 {
    e.call(SMART_POINTER_GET, &args![this.addr().wrapping_add(0x70)])
        .u32()
}

// Translated from 009c86a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a flag byte in the global at `0x012677cc` (the update stores `mode == 4`).
pub fn fn_009c86a0(e: &mut Engine, flag: u8) {
    e.set_global(PLAYING_BACK_FLAG, flag);
}

// Translated from 009c86b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::FreeHeadAction` (Xbox PDB): deletes the first command and removes it from the list,
/// frees the melee-attack idle, then keeps deleting the following commands while they have a
/// target that reports `true` to the virtual call at `+0x22c` (argument 0), or are stranger
/// commands whose target's value is above the integer setting at `0x011ce7e4`. When the command
/// left at the head is a stranger (or miss-fortune) command, the current actor becomes that actor
/// and its stat (`0x19` / `0x1c`) is incremented.
pub fn vats_free_head_action(e: &mut Engine, this: Ptr<Vats>) {
    let list = this.addr();
    if list_is_empty(e, list) {
        return;
    }
    let first = list_item(e, list);
    if first == 0 {
        return;
    }
    fn_009c8870(e, Ptr::new(first), 1);
    e.call(LIST_POP_FRONT, &args![list]);
    clear_melee_attack(e, this);
    let mut current;
    loop {
        if list_is_empty(e, list) {
            current = 0;
            break;
        }
        current = list_item(e, list);
        let mut remove = false;
        if current != 0 && e.mem.u32(current.wrapping_add(0xc)) != 0 {
            let target = e.mem.u32(current.wrapping_add(0xc));
            if e.vcall(target, 0x22c, &args![0u32]).bool() {
                remove = true;
            } else if e.mem.u8(current.wrapping_add(6)) != 0 {
                let value = e.call(TARGET_LAST_VALUE, &args![target]).f64();
                let limit_pointer = e
                    .call(SETTING_INT_POINTER, &args![HEAD_ACTION_LIMIT_SETTING])
                    .u32();
                let limit = e.mem.i32(limit_pointer) as f64;
                if !parity_even(x87_compare(limit, value) & 0x05) {
                    remove = true;
                }
            }
        }
        if !remove {
            break;
        }
        fn_009c8870(e, Ptr::new(current), 1);
        e.call(LIST_POP_FRONT, &args![list]);
    }
    if current != 0 && e.mem.u8(current.wrapping_add(6)) != 0 {
        let stranger = e.global::<u32>(STRANGER_ACTOR);
        e.set_global(CURRENT_ACTOR, stranger);
        e.call(STAT_INCREMENT, &args![0x19u32]);
    }
    if current != 0 && e.mem.u8(current.wrapping_add(0x24)) != 0 {
        let miss_fortune = e.global::<u32>(MISS_FORTUNE_ACTOR);
        e.set_global(CURRENT_ACTOR, miss_fortune);
        e.call(STAT_INCREMENT, &args![0x1cu32]);
    }
}

/// Frees the melee-attack idle when there is one: the current actor's animation releases its
/// special idle (`1`, `0`) and the field is cleared. `FreeHeadAction` and `ClearCommandList`
/// carry this same inline code.
fn clear_melee_attack(e: &mut Engine, this: Ptr<Vats>) {
    if !e.get(this, Vats::pMeleeAttack).is_null() {
        let actor = e.global::<u32>(CURRENT_ACTOR);
        let animation = e.call(ACTOR_GET_ANIMATION, &args![actor]).u32();
        e.call(SPECIAL_IDLE_FREE, &args![animation, 1u32, 0u32]);
        e.set(this, Vats::pMeleeAttack, Ptr::NULL);
    }
}

// Translated from 009c8870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of a `VATS_COMMAND`: releases the hit data, and frees the block when
/// bit 0 of `flags` is set. Returns `this`.
pub fn fn_009c8870(e: &mut Engine, this: Ptr<VatsCommand>, flags: u32) -> Ptr<VatsCommand> {
    e.call(COMMAND_RELEASE_HIT_DATA, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_BLOCK, &args![this]);
    }
    this
}

// Translated from 009c88a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pops the last command of the list: stores its action number at `out_action` and its iron-sights
/// flag at `out_iron_sights`, deletes it and removes it from the list.
pub fn fn_009c88a0(e: &mut Engine, this: Ptr<Vats>, out_action: Ptr, out_iron_sights: Ptr) {
    let mut node = this.addr();
    while node != 0 {
        if list_is_empty(e, node) {
            return;
        }
        if list_next(e, node) == 0 {
            let command = list_item(e, node);
            let action = e.mem.u32(command);
            let iron_sights = e.mem.u8(command.wrapping_add(5));
            e.mem.set_u32(out_action.addr(), action);
            e.mem.set_u8(out_iron_sights.addr(), iron_sights);
            if command != 0 {
                fn_009c8870(e, Ptr::new(command), 1);
            }
            remove_list_item(e, this.addr(), command);
            return;
        }
        node = list_next(e, node);
    }
}

/// `list.remove(&item)`: the list method takes the address of a slot that holds the item.
fn remove_list_item(e: &mut Engine, list: u32, item: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_REMOVE_ITEM, &args![list, slot]);
    });
}

// Translated from 009c8950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::QuitVATSPlayback` (Xbox PDB): does nothing in mode 0, nor while the end countdown is
/// running with an empty list, nor (unless `force` is set) when the current command is action 15.
/// With `keep_current` set it only normalizes the current command (more than one fire shot is cut
/// to one and the fake shots are cleared) and deletes every command after the first; otherwise it
/// clears the command list, starts the end countdown from the setting at `0x011ce338` and resets
/// the global time multiplier.
pub fn vats_quit_vats_playback(e: &mut Engine, this: Ptr<Vats>, force: u8, keep_current: u8) {
    let list = this.addr();
    if e.get(this, Vats::eMode) == 0 {
        return;
    }
    let countdown: f32 = e.global(END_COUNTDOWN);
    let ah = x87_compare(countdown as f64, e.global::<f64>(ZERO_DOUBLE));
    if parity_even(ah & 0x44) && list_is_empty(e, list) {
        return;
    }
    if force == 0 && !list_is_empty(e, list) {
        let first = list_item(e, list);
        if e.mem.u32(first) == 0xf {
            return;
        }
    }
    if keep_current != 0 {
        let current = vats_get_current_action(e, this);
        if current != 0 && e.mem.u8(current.wrapping_add(8)) > 1 {
            e.mem.set_u8(current.wrapping_add(8), 1);
            e.mem.set_u8(current.wrapping_add(9), 0);
        }
        if !list_is_empty(e, list) {
            let mut node = list_next(e, list);
            while node != 0 {
                let command = list_item(e, node);
                if command != 0 {
                    fn_009c8870(e, Ptr::new(command), 1);
                }
                remove_list_item(e, list, command);
                node = list_next(e, list);
            }
        }
    } else {
        vats_clear_command_list(e, this);
        let setting = e
            .call(SETTING_FLOAT_POINTER, &args![QUIT_DELAY_SETTING])
            .u32();
        copy_float(e, setting, END_COUNTDOWN);
        set_time_multiplier(e, 1.0, 0);
    }
}

// Translated from 009c8a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::QuitVATSPlayback_ov2` (Xbox PDB): stores the weapon and extra data to un-equip, then
/// quits the playback with both flags set.
pub fn vats_quit_vats_playback_ov2(e: &mut Engine, this: Ptr<Vats>, weapon: u32, extra: u32) {
    e.set(this, Vats::pUnEquipWeapon, Ptr::new(weapon));
    e.set(this, Vats::pUnEquipExtra, Ptr::new(extra));
    vats_quit_vats_playback(e, this, 1, 1);
}

// Translated from 009c8ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a look-at position into `out` and returns `out`. It starts as the vector at
/// `0x011f426c`; with a camera shot, the shot's own position replaces it. When that vector equals
/// the global one (`004390c0` is the vector `==`), the player's float at `+0x698` builds a vertical
/// offset that is added to a position taken from the current actor's model object, and the sum is
/// the result.
pub fn fn_009c8ac0(e: &mut Engine, this: Ptr<Vats>, out: Ptr) -> Ptr {
    let mut position = read_vec3(e, DEFAULT_POSITION);
    let camera = e.get(this, Vats::pCamera);
    if !camera.is_null() {
        let block = e.mem.alloc(12);
        let shot_position = e.call(CAMERA_POSITION, &args![camera, block]).u32();
        position = read_vec3(e, shot_position);
        e.mem.free(block);
    }
    let local = e.mem.alloc(12);
    write_vec3(e, local, position);
    let equal = e
        .call(POINT3_EQUALS, &args![local, DEFAULT_POSITION])
        .bool();
    e.mem.free(local);
    if equal {
        let player = e.global::<u32>(PLAYER_POINTER);
        let height = e.call(PLAYER_FLOAT, &args![player]).f32();
        let offset_block = e.mem.alloc(12);
        let offset = e
            .call(
                POINT3_CONSTRUCTOR,
                &args![offset_block, 0.0f32, 0.0f32, height],
            )
            .u32();
        let sum = e.mem.alloc(12);
        let actor = e.global::<u32>(CURRENT_ACTOR);
        let model = e.call(ACTOR_MODEL_OBJECT, &args![actor]).u32();
        let part = e.call(BOUND_NEXT_PART, &args![model]).u32();
        let result = e.call(POINT3_ADD, &args![part, sum, offset]).u32();
        position = read_vec3(e, result);
        e.mem.free(sum);
        e.mem.free(offset_block);
    }
    write_vec3(e, out.addr(), position);
    out
}

// Translated from 009c8b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the reference at `+0x1c`. When there is a camera shot, its first and second targets
/// (when the selectors `00639b40` and `008d6f30` of the shot return 1) are set to the reference's
/// 3D object (virtual call at `+0x1d0`), or to null when the reference is null.
pub fn fn_009c8b90(e: &mut Engine, this: Ptr<Vats>, reference: u32) {
    let camera = e.get(this, Vats::pCamera);
    if !camera.is_null() {
        for (selector, setter) in [
            (WORD_AT_0X3C, CAMERA_SET_VIEW_TARGET),
            (WORD_AT_0X40, CAMERA_SET_SECOND_TARGET),
        ] {
            if e.call(selector, &args![camera]).i32() == 1 {
                let object = if reference == 0 {
                    0
                } else {
                    e.vcall(reference, 0x1d0, &args![]).u32()
                };
                e.call(setter, &args![camera, object]);
            }
        }
    }
    e.set(this, Vats::pFirstProjectile, Ptr::new(reference));
}

// Translated from 009c8c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the reference at `+0x20`. With a camera shot it updates the camera stage for the current
/// command and, while the minimum timer is exactly 0.0, advances the camera shots until the stage
/// is not above the shot's count or no further shot starts.
pub fn fn_009c8c40(e: &mut Engine, this: Ptr<Vats>, reference: u32) {
    e.set(this, Vats::pLastProjectile, Ptr::new(reference));
    if e.get(this, Vats::pCamera).is_null() {
        return;
    }
    let current = vats_get_current_action(e, this);
    fn_009c95e0(e, this, Ptr::new(current));
    let timer = e.get(this, Vats::fCameraMinTimer);
    if !parity_even(x87_compare(timer as f64, e.global::<f64>(ZERO_DOUBLE)) & 0x44) {
        let mut more = true;
        while !e.get(this, Vats::pCamera).is_null() && more {
            let camera = e.get(this, Vats::pCamera);
            let count = e.call(WORD_AT_0X38, &args![camera]).i32();
            if e.global::<i32>(CAMERA_STAGE) <= count {
                return;
            }
            more = vats_use_next_camera(e, this);
        }
    }
}

// Translated from 009c8cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::GetPlayerUpdateMult` (Xbox PDB): in mode 4 the factor is the camera shot's float
/// (`00639aa0`) when it is above 0.0, else the setting at `0x011cf374` (an uninitialized shot
/// gives 1.0); other modes give 1.0. The result is the factor times the player's time factor
/// divided by the frame divisor.
pub fn vats_get_player_update_mult(e: &mut Engine, this: Ptr<Vats>) -> f32 {
    update_mult(e, this, FLOAT_AT_0X48, PLAYER_MULT_SETTING)
}

// Translated from 009c8d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::GetTargetUpdateMult` (Xbox PDB): the same computation as `GetPlayerUpdateMult` with the
/// camera shot's float `00507b00` and the setting at `0x011cfcec`.
pub fn vats_get_target_update_mult(e: &mut Engine, this: Ptr<Vats>) -> f32 {
    update_mult(e, this, FLOAT_AT_0X4C, TARGET_MULT_SETTING)
}

fn update_mult(e: &mut Engine, this: Ptr<Vats>, camera_float: u32, setting: u32) -> f32 {
    let mut factor = 1.0f32;
    if e.get(this, Vats::eMode) == 4 {
        let camera = e.get(this, Vats::pCamera);
        if !camera.is_null() && !e.call(CAMERA_IS_INITIALIZED, &args![camera]).bool() {
            factor = 1.0;
        } else {
            let mut from_camera = false;
            if !camera.is_null() {
                let value = e.call(camera_float, &args![camera]).f64();
                if x87_compare(value, e.global::<f64>(ZERO_DOUBLE)) & 0x41 == 0 {
                    factor = e.call(camera_float, &args![camera]).f32();
                    from_camera = true;
                }
            }
            if !from_camera {
                factor = setting_float(e, setting);
            }
        }
    }
    let dividend = e.call(PLAYER_TIME_FACTOR, &args![]).f64();
    let divisor = e.call(TIMER_DIVISOR, &args![]).f64();
    ((dividend / divisor) * factor as f64) as f32
}

// Translated from 009c8e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::EvaluateCommand` (Xbox PDB): for an attack command, a command that is still successful
/// and has a target loses its success when the pick-time check fails and the current actor has no
/// line of sight to the target; its hit data is cleared (when there is any) and initialized from
/// the command.
pub fn vats_evaluate_command(e: &mut Engine, _unused_0: Ptr<Vats>, command: Ptr<VatsCommand>) {
    if command.is_null() {
        return;
    }
    let action = e.get(command, VatsCommand::eAction);
    if !e.call(IS_ATTACK_ACTION, &args![action]).bool() {
        return;
    }
    if e.get(command, VatsCommand::bSuccess)
        && !e.get(command, VatsCommand::pTarget).is_null()
        && !fn_009c8ea0(e)
    {
        let actor = e.global::<u32>(CURRENT_ACTOR);
        let target = e.get(command, VatsCommand::pTarget);
        let visible = e
            .call(
                ACTOR_LINE_OF_SIGHT,
                &args![actor, 1u32, target, 1u32, 0u32, 0u32],
            )
            .bool();
        if !visible {
            e.set(command, VatsCommand::bSuccess, false);
        }
    }
    let hit_data = e.get(command, VatsCommand::pHitData);
    if !hit_data.is_null() {
        e.call(HIT_DATA_CLEAR, &args![hit_data]);
    }
    e.call(HIT_DATA_INITIALIZE, &args![hit_data, command]);
}

// Translated from 009c8ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names this `bhkWorld::QPickTimeOverMax`; the body reads the VATS statics
/// (`fn_009c8ef0` and the float at `0x012677c4`), so the map name is not trusted here. False when
/// the call at `0x00888960` returns true; otherwise true when the float at `0x012677c4` is above
/// the value `fn_009c8ef0` returns (an ordered comparison: NaN gives false).
pub fn fn_009c8ea0(e: &mut Engine) -> bool {
    if e.call(INTERFACE_ENABLED, &args![]).bool() {
        return false;
    }
    let value = fn_009c8ef0(e);
    let limit: f32 = e.global(PICK_TIME_LIMIT);
    // FLD limit; FCOMPP against the returned value (st1): C0 is set when limit < value.
    x87_compare(limit as f64, value as f64) & 0x41 == 0
}

// Translated from 009c8ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `0x011afe78` when the byte at `0x012677cc` is set, else the float at
/// `0x011afe74` (copied through `FLD` / `FSTP`).
pub fn fn_009c8ef0(e: &mut Engine) -> f32 {
    let source = if e.global::<u8>(PLAYING_BACK_FLAG) != 0 {
        PICK_TIME_FLOAT_ON
    } else {
        PICK_TIME_FLOAT_OFF
    };
    f32::from_bits(x87_quiet_bits(e.mem.u32(source)))
}

/// The blocks the line-of-sight ray cast keeps on the exe's stack: the ray query (0xb0 bytes), the
/// filter-info word, the all-hits collector (0x330 bytes), the hit record (0x70 bytes) and the two
/// positions.
struct RayBlocks {
    query: u32,
    filter: u32,
    collector: u32,
    record: u32,
    from: u32,
    to: u32,
}

// Translated from 009c8f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::GetLineOfSight` (Xbox PDB): true when no other reference is in the way between the
/// current actor and the command's target. The target must be an actor (virtual call `+0x100`)
/// with an object at `+0x40` of its process; both positions are raised by 60.0 and a ray is cast
/// on the target's object with the layer pair `0x27` / `0x26206`; the first hit whose 3D object
/// resolves to a reference other than the target and the current actor blocks the sight.
// Translation incomplete: 009c8f20 - C++ exception-unwinding frame (FS:[0], handler 00f1b4eb, state 0 while the all-hits collector lives) and /GS cookie check (00ec408c) not translated
pub fn vats_get_line_of_sight(
    e: &mut Engine,
    _unused_0: Ptr<Vats>,
    command: Ptr<VatsCommand>,
) -> bool {
    let target = e.get(command, VatsCommand::pTarget).addr();
    let actor = if e.vcall(target, 0x100, &args![]).bool() {
        target
    } else {
        0
    };
    if actor == 0 {
        return false;
    }
    let mut base = 0;
    if e.call(WORD_AT_0X40, &args![actor]).u32() != 0 {
        let process = e.call(WORD_AT_0X40, &args![actor]).u32();
        base = e.call(ACTOR_CONTROLLER_BASE, &args![process]).u32();
    }
    if base == 0 {
        return false;
    }
    e.call(SET_MULTI_LAYER_COLLISION, &args![0x27u32, 0x26206u32, 0u32]);
    let current = e.global::<u32>(CURRENT_ACTOR);
    let from_position = e.vcall(current, 0x1f4, &args![]).u32();
    let mut from = read_vec3(e, from_position);
    let to_position = e.vcall(actor, 0x1f4, &args![]).u32();
    let mut to = read_vec3(e, to_position);
    let raise: f64 = e.global(SIXTY_DOUBLE);
    from[2] = ((f32::from_bits(from[2]) as f64 + raise) as f32).to_bits();
    to[2] = ((f32::from_bits(to[2]) as f64 + raise) as f32).to_bits();
    let blocks = RayBlocks {
        query: e.mem.alloc(0xb0),
        filter: e.mem.alloc(4),
        collector: e.mem.alloc(0x330),
        record: e.mem.alloc(0x70),
        from: e.mem.alloc(12),
        to: e.mem.alloc(12),
    };
    write_vec3(e, blocks.from, from);
    write_vec3(e, blocks.to, to);
    let clear = cast_line_of_sight_ray(e, &blocks, actor, base);
    for block in [
        blocks.query,
        blocks.filter,
        blocks.collector,
        blocks.record,
        blocks.from,
        blocks.to,
    ] {
        e.mem.free(block);
    }
    clear
}

/// The ray cast of `GetLineOfSight`: true when no hit belongs to another reference. The exe also
/// keeps a byte next to the collector that would end the loop early; nothing in the function sets
/// it, so it is always clear.
fn cast_line_of_sight_ray(e: &mut Engine, blocks: &RayBlocks, actor: u32, base: u32) -> bool {
    e.call(RAY_QUERY_CONSTRUCTOR, &args![blocks.query]);
    e.call(FILTER_INFO_CONSTRUCTOR, &args![blocks.filter, 0u32]);
    e.call(FILTER_INFO_SET_LAYER, &args![blocks.filter, 0x27u32]);
    let actor_or_null = if e.vcall(actor, 0x100, &args![]).bool() {
        actor
    } else {
        0
    };
    let has_controller =
        actor_or_null != 0 && e.call(GET_CHAR_CONTROLLER, &args![actor_or_null]).u32() != 0;
    let group = if has_controller {
        let controller = e.call(GET_CHAR_CONTROLLER, &args![actor_or_null]).u32();
        e.call(CONTROLLER_COLLISION_GROUP, &args![controller]).u32()
    } else {
        e.call(WORLD_COLLISION_GROUP, &args![]).u32()
    };
    e.call(FILTER_INFO_SET_GROUP, &args![blocks.filter, group]);
    let filter_word = e.mem.u32(blocks.filter);
    e.call(RAY_QUERY_SET_FILTER, &args![blocks.query, filter_word]);
    e.call(RAY_QUERY_SET_FROM, &args![blocks.query, blocks.from]);
    e.call(RAY_QUERY_SET_TO, &args![blocks.query, blocks.to]);
    e.call(RAY_COLLECTOR_CONSTRUCTOR, &args![blocks.collector]);
    e.call(
        RAY_QUERY_SET_COLLECTOR,
        &args![blocks.query, blocks.collector],
    );
    e.vcall(base, 0xc8, &args![blocks.query]);
    let current = e.global::<u32>(CURRENT_ACTOR);
    let mut index = 0u32;
    let clear = loop {
        let hits = e.call(RAY_HITS_FIRST, &args![blocks.query]).u32();
        let hit_list = e.call(RAY_HITS_SECOND, &args![hits]).u32();
        let count = list_next(e, hit_list) as i32;
        if index as i32 >= count {
            break true;
        }
        let hits = e.call(RAY_HITS_FIRST, &args![blocks.query]).u32();
        let hit_list = e.call(RAY_HITS_SECOND, &args![hits]).u32();
        let hit = e.call(RAY_HIT_AT, &args![hit_list, index]).u32();
        e.call(HIT_RECORD_CONSTRUCTOR, &args![blocks.record, hit]);
        let collidable = e.mem.u32(blocks.record.wrapping_add(0x50));
        let object = e
            .call(GET_AV_OBJECT_FOR_COLLIDABLE, &args![collidable])
            .u32();
        let reference = e.call(FIND_REFERENCE_FOR_3D, &args![object]).u32();
        if object != 0 && reference != actor && reference != current {
            break false;
        }
        index += 1;
    };
    e.call(RAY_COLLECTOR_DESTRUCTOR, &args![blocks.collector]);
    clear
}

// Translated from 009c9280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::UpdateMeleeAttack` (Xbox PDB): for a command with fire shots left, when the current
/// actor is one of the two special actors or the command is a melee command and it has a target:
/// the reach is the stranger setting (stranger and miss-fortune commands, which also restore the
/// actor's alpha to 1.0 and play the stranger sound when it was not 1.0) or the weapon's reach
/// (scaled by one of two settings for an unarmed or a ranged-kind weapon); the actor is moved to
/// the safe path point at that reach from the target (the player's location first for the special
/// actors) and faces the target, falling back to the target's own position (also for the player
/// when `GetLineOfSight` fails). Then the idle for the pair is chosen from the idle manager; one
/// that is not in animation group section 4 is reported and dropped.
pub fn vats_update_melee_attack(e: &mut Engine, this: Ptr<Vats>, command: Ptr<VatsCommand>) {
    if e.get(command, VatsCommand::cFireShots) == 0 {
        return;
    }
    let actor = e.global::<u32>(CURRENT_ACTOR);
    let stranger = e.global::<u32>(STRANGER_ACTOR);
    let miss_fortune = e.global::<u32>(MISS_FORTUNE_ACTOR);
    let target = e.get(command, VatsCommand::pTarget).addr();
    let applies =
        actor == stranger || actor == miss_fortune || vats_is_melee_command(e, this, command);
    if applies && target != 0 {
        let reach;
        if e.get(command, VatsCommand::bStranger) {
            reach = setting_float(e, STRANGER_REACH_SETTING);
            restore_alpha(e, stranger);
        } else if e.get(command, VatsCommand::bMissFortune) {
            reach = setting_float(e, STRANGER_REACH_SETTING);
            restore_alpha(e, miss_fortune);
        } else {
            let weapon = e.call(ACTOR_GET_CURRENT_WEAPON, &args![actor]).u32();
            let mut value = e.call(GET_WEAPON_REACH, &args![actor, weapon]).f32();
            if e.call(ACTOR_GET_CURRENT_WEAPON, &args![actor]).u32() == 0 {
                let factor = e
                    .call(
                        SETTING_FLOAT_POINTER,
                        &args![REACH_FACTOR_NO_WEAPON_SETTING],
                    )
                    .u32();
                value = (value as f64 * e.mem.f32(factor) as f64) as f32;
            } else {
                let weapon = e.call(ACTOR_GET_CURRENT_WEAPON, &args![actor]).u32();
                if e.call(WEAPON_TEST_006450C0, &args![weapon]).bool() {
                    let factor = e
                        .call(
                            SETTING_FLOAT_POINTER,
                            &args![REACH_FACTOR_WEAPON_TEST_SETTING],
                        )
                        .u32();
                    value = (value as f64 * e.mem.f32(factor) as f64) as f32;
                }
            }
            reach = value;
        }
        let point = e.mem.alloc(12);
        e.call(LIST_ITEM_SLOT, &args![point]);
        let player = e.global::<u32>(PLAYER_POINTER);
        if actor == stranger || actor == miss_fortune {
            let player_position = e.vcall(player, 0x1f4, &args![]).u32();
            e.call(SET_LOCATION_ON_REFERENCE, &args![actor, player_position]);
        }
        let found = e
            .call(FIND_SAFE_PATH_POINT, &args![actor, target, reach, point])
            .bool();
        if found {
            e.vcall(actor, 0x2a8, &args![point]);
            e.vcall(actor, 0x2c0, &args![target]);
            if player == actor && !vats_get_line_of_sight(e, this, command) {
                face_target_position(e, actor, target);
            }
        } else {
            face_target_position(e, actor, target);
        }
        let scene_node = e.vcall(actor, 0x1e4, &args![]).u32();
        e.call(UPDATE_SCENE_GRAPH, &args![scene_node]);
        e.mem.free(point);
    }
    let manager = e.global::<u32>(IDLE_MANAGER_POINTER);
    let target = e.get(command, VatsCommand::pTarget).addr();
    let idle = e
        .call(GET_IDLE_TO_PLAY, &args![manager, actor, target])
        .u32();
    e.set(this, Vats::pMeleeAttack, Ptr::new(idle));
    if idle != 0 && e.call(IDLE_GET_ANIM_GROUP_SECTION, &args![idle]).i32() != 4 {
        let name = e.vcall(idle, 0x130, &args![]).u32();
        e.call(PRINT_MESSAGE, &args![BAD_MELEE_ATTACK_FORMAT, name]);
        e.set(this, Vats::pMeleeAttack, Ptr::NULL);
    }
}

/// Sets the actor's position to the target's position and makes it face the target (virtual calls
/// `+0x2a8` and `+0x2c0`).
fn face_target_position(e: &mut Engine, actor: u32, target: u32) {
    let position = e.vcall(target, 0x1f4, &args![]).u32();
    e.vcall(actor, 0x2a8, &args![position]);
    e.vcall(actor, 0x2c0, &args![target]);
}

/// The special actors' alpha is put back to 1.0 when it was something else, with the stranger
/// sound.
fn restore_alpha(e: &mut Engine, special_actor: u32) {
    let alpha = e.call(ACTOR_GET_ALPHA_MULT, &args![special_actor]).f64();
    let one: f64 = e.global(ONE_DOUBLE);
    if parity_even(x87_compare(alpha, one) & 0x44) {
        e.call(ACTOR_SET_ALPHA_MULT, &args![special_actor, 1.0f32]);
        e.call(ACTOR_UPDATE_ALPHA, &args![special_actor]);
        e.call(STRANGER_APPEARS_SOUND, &args![]);
    }
}

// Translated from 009c95e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Decides the camera stage (`0x011f21b8`, 0 = undecided) for a command (unnamed in the engine
/// map). A melee command only acts while the stage is 0: it goes to stage 2 when the animation's
/// action object is missing, or its record tests say so, or the target answers `true` to the
/// virtual call `+0x22c` (argument 0); at stage 2 with the minimum timer exactly 0.0 the camera
/// shots then advance. Action 18 goes to stage 2 and sets the flag byte `+0x76` of the camera
/// shot. For other commands the stage becomes 1 when the first reference (`+0x1c`) exists and the
/// stage is 0; without both references it moves from 1 to 2; without the first reference alone it
/// is 2; with both it is 2 when there is no camera or the shot's word `+0x38` is 2; the stage is
/// also 2 when the distance from the first (and then the second) reference to the target exceeds
/// the target's bound value plus the distance setting.
pub fn fn_009c95e0(e: &mut Engine, this: Ptr<Vats>, command: Ptr<VatsCommand>) {
    if vats_is_melee_command(e, this, command) {
        let actor = e.global::<u32>(CURRENT_ACTOR);
        let animation = e.call(ACTOR_GET_ANIMATION, &args![actor]).u32();
        if e.global::<i32>(CAMERA_STAGE) != 0 {
            return;
        }
        let action_object = e
            .call(ANIMATION_ACTION_OBJECT, &args![animation, 4u32])
            .u32();
        if action_object != 0 {
            let record = e.call(ACTION_RECORD, &args![action_object]).u32();
            let first = e.call(ACTION_RECORD_TEST_FIRST, &args![record]).bool();
            let mut to_stage_two = false;
            if first && e.get(command, VatsCommand::cFireShots) == 0 {
                to_stage_two = true;
            } else {
                let record = e.call(ACTION_RECORD, &args![action_object]).u32();
                let second = e.call(ACTION_RECORD_TEST_SECOND, &args![record]).bool();
                if second && e.call(ANIMATION_KIND_COUNT, &args![animation, 4u32]).i32() >= 1 {
                    to_stage_two = true;
                }
            }
            if to_stage_two {
                e.set_global(CAMERA_STAGE, 2i32);
            } else {
                let target = e.get(command, VatsCommand::pTarget).addr();
                if target != 0 && e.vcall(target, 0x22c, &args![0u32]).bool() {
                    e.set_global(CAMERA_STAGE, 2i32);
                }
            }
        } else {
            e.set_global(CAMERA_STAGE, 2i32);
        }
        advance_cameras_at_stage_two(e, this);
        return;
    }
    if !command.is_null() && e.get(command, VatsCommand::eAction) == 0x12 {
        e.set_global(CAMERA_STAGE, 2i32);
        let camera = e.get(this, Vats::pCamera).addr();
        fn_009c9900(e, Ptr::new(camera), 1);
        return;
    }
    let first_reference = e.get(this, Vats::pFirstProjectile).addr();
    let second_reference = e.get(this, Vats::pLastProjectile).addr();
    if first_reference != 0 && e.global::<i32>(CAMERA_STAGE) == 0 {
        e.set_global(CAMERA_STAGE, 1i32);
    }
    if first_reference == 0 && second_reference == 0 {
        if e.global::<i32>(CAMERA_STAGE) == 1 {
            e.set_global(CAMERA_STAGE, 2i32);
        }
        return;
    }
    if first_reference == 0 {
        e.set_global(CAMERA_STAGE, 2i32);
        return;
    }
    if second_reference != 0 {
        let camera = e.get(this, Vats::pCamera).addr();
        if camera == 0 || e.call(WORD_AT_0X38, &args![camera]).i32() == 2 {
            e.set_global(CAMERA_STAGE, 2i32);
            return;
        }
    }
    let target = e.get(command, VatsCommand::pTarget).addr();
    let first_position = e.vcall(first_reference, 0x1f4, &args![]).u32();
    let out = e.mem.alloc(12);
    let target_position = e.vcall(target, 0x1f4, &args![]).u32();
    e.call(
        POINT3_SUBTRACT,
        &args![target_position, out, first_position],
    );
    let length = e.call(POINT3_LENGTH, &args![out]).f64();
    let distance = e
        .call(SETTING_INT_POINTER, &args![CAMERA_DISTANCE_SETTING])
        .u32();
    let distance = e.mem.i32(distance) as f64;
    let bounds = e.mem.alloc(12);
    let bound_object = e.vcall(target, 0x1dc, &args![bounds]).u32();
    let radius = e.mem.f32(bound_object.wrapping_add(4)) as f64;
    // FCOMP: (radius + distance) against the length.
    if x87_compare(radius + distance, length) & 0x41 == 0 {
        e.set_global(CAMERA_STAGE, 2i32);
        e.mem.free(out);
        e.mem.free(bounds);
        return;
    }
    e.mem.free(bounds);
    if second_reference != 0 {
        let second_position = e.vcall(second_reference, 0x1f4, &args![]).u32();
        let target_position = e.vcall(target, 0x1f4, &args![]).u32();
        let result = e
            .call(
                POINT3_SUBTRACT,
                &args![target_position, out, second_position],
            )
            .u32();
        let value = read_vec3(e, result);
        write_vec3(e, out, value);
        let length = e.call(POINT3_LENGTH, &args![out]).f64();
        let distance = e
            .call(SETTING_INT_POINTER, &args![CAMERA_DISTANCE_SETTING])
            .u32();
        let distance = e.mem.i32(distance) as f64;
        let bounds = e.mem.alloc(12);
        let bound_object = e.vcall(target, 0x1dc, &args![bounds]).u32();
        let radius = e.mem.f32(bound_object.wrapping_add(4)) as f64;
        if x87_compare(radius + distance, length) & 0x41 == 0 {
            e.set_global(CAMERA_STAGE, 2i32);
        }
        e.mem.free(bounds);
    }
    e.mem.free(out);
}

/// The melee branch of the stage decision ends here: at stage 2 with the minimum timer exactly 0.0
/// the camera shots are advanced until the stage is not above the shot count or no shot starts.
fn advance_cameras_at_stage_two(e: &mut Engine, this: Ptr<Vats>) {
    if e.global::<i32>(CAMERA_STAGE) == 2 {
        let timer = e.get(this, Vats::fCameraMinTimer);
        if !parity_even(x87_compare(timer as f64, e.global::<f64>(ZERO_DOUBLE)) & 0x44) {
            let mut more = true;
            while more {
                let camera = e.get(this, Vats::pCamera).addr();
                let count = e.call(WORD_AT_0X38, &args![camera]).i32();
                if e.global::<i32>(CAMERA_STAGE) <= count {
                    break;
                }
                more = vats_use_next_camera(e, this);
            }
        }
    }
}

// Translated from 009c9900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Camera shot: stores a flag byte at `+0x76`.
pub fn fn_009c9900(e: &mut Engine, this: Ptr, flag: u8) {
    e.mem.set_u8(this.addr().wrapping_add(0x76), flag);
}

// Translated from 009c9920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::UseNextCamera` (Xbox PDB): moves to the camera shot after the current one in the camera
/// list. The old shot is freed, the new one's minimum time raises the minimum timer when larger,
/// and the first-person view is switched on when the shot has flag `0x08` (else the "switch first
/// person off again" flag is set); the result is true. At the end of the list an uninitialized
/// shot is freed and the result is false.
pub fn vats_use_next_camera(e: &mut Engine, this: Ptr<Vats>) -> bool {
    let mut node = e.get(this, Vats::pCameraList).addr();
    let mut started = false;
    while node != 0 && list_item(e, node) != e.get(this, Vats::pCamera).addr() {
        node = list_next(e, node);
    }
    if node != 0 {
        node = list_next(e, node);
    }
    if node != 0 {
        let old = e.get(this, Vats::pCamera);
        if !old.is_null() {
            e.call(FREE_CAMERA, &args![old]);
            e.set(this, Vats::pCamera, Ptr::NULL);
        }
        let next = list_item(e, node);
        e.set(this, Vats::pCamera, Ptr::new(next));
        if next != 0 {
            let minimum = e.call(FLOAT_AT_0X58, &args![next]).f64();
            let timer = e.get(this, Vats::fCameraMinTimer);
            if !parity_even(x87_compare(timer as f64, minimum) & 0x05) {
                let minimum = e.call(FLOAT_AT_0X58, &args![next]).f32();
                e.set(this, Vats::fCameraMinTimer, minimum);
            }
        }
        if next != 0 && fn_009c8600(e, Ptr::new(next)) {
            let player = e.global::<u32>(PLAYER_POINTER);
            e.call(PLAYER_SET_FIRST_PERSON, &args![player, 1u32]);
        } else {
            e.set_global(FIRST_PERSON_PENDING, 1u8);
        }
        started = true;
    } else {
        let camera = e.get(this, Vats::pCamera);
        if !camera.is_null() && !e.call(CAMERA_IS_INITIALIZED, &args![camera]).bool() {
            e.call(FREE_CAMERA, &args![camera]);
            e.set(this, Vats::pCamera, Ptr::NULL);
        }
    }
    started
}

// Translated from 009c9a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::SetVATSLight` (Xbox PDB): with `clear` set, a held light is released (the smart pointer
/// at `+0x34` is set to null when it is not already). A new light is added to the scene's shadow
/// node, kept in the smart pointer, and told (virtual call `+0xdc`, arguments the light and 1)
/// to the scene node.
pub fn vats_set_vats_light(e: &mut Engine, this: Ptr<Vats>, light: u32, clear: u8) {
    let slot = this.addr().wrapping_add(0x34);
    if clear != 0 && e.call(SMART_POINTER_DIFFERS, &args![slot, 0u32]).bool() {
        e.call(SMART_POINTER_ASSIGN, &args![slot, 0u32]);
    }
    if light != 0 {
        let node = e.call(SCENE_NODE_BY_INDEX, &args![0u32]).u32();
        let held = e.call(ADD_LIGHT, &args![node, light, 1u32]).u32();
        e.call(SMART_POINTER_ASSIGN, &args![slot, held]);
        let node = e.call(SCENE_NODE_BY_INDEX, &args![0u32]).u32();
        e.vcall(node, 0xdc, &args![light, 1u32]);
    }
}

// Translated from 009c9ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Places the VATS light for an actor reference: for an actor with a 3D object and a process, the
/// light's position is the actor's bound centre offset along the direction from the reference to
/// the current actor (the direction is rotated by an angle setting and scaled by a setting), plus
/// a height setting; its range, direction and colour are set from a trace result and the
/// settings. Nothing happens when the VATS object holds no light.
pub fn fn_009c9ae0(e: &mut Engine, this: Ptr<Vats>, actor: u32) {
    if !e.vcall(actor, 0x100, &args![]).bool() {
        return;
    }
    if e.vcall(actor, 0x1d0, &args![]).u32() == 0 {
        return;
    }
    if held_light(e, this) == 0 {
        return;
    }
    if e.call(ACTOR_GET_PROCESS, &args![actor]).u32() == 0 {
        return;
    }
    // Locals of the exe's frame, each 12 bytes unless noted.
    let scratch = e.mem.alloc(0x100);
    let bound_centre = scratch;
    let direction = scratch + 0x10;
    let matrix = scratch + 0x20; // 36 bytes
    let rotated = scratch + 0x50;
    let sum = scratch + 0x60;
    let light_direction = scratch + 0x70;
    let ground = scratch + 0x90;
    let trace_a = scratch + 0xa0; // a dword
    let trace_b = scratch + 0xa4; // a float
    let trace_c = scratch + 0xa8; // a float
    let radius_vector = scratch + 0xb0;
    let extent_vector = scratch + 0xc0;

    let object = e.vcall(actor, 0x1d0, &args![]).u32();
    let bound = e.call(GET_WORLD_BOUND, &args![object]).u32();
    let bound_slot = e.call(LIST_ITEM_SLOT, &args![bound]).u32();
    let centre = read_vec3(e, bound_slot);
    write_vec3(e, bound_centre, centre);

    let actor_position = e.vcall(actor, 0x1f4, &args![]).u32();
    let current = e.global::<u32>(CURRENT_ACTOR);
    let current_position = e.vcall(current, 0x1f4, &args![]).u32();
    e.call(
        POINT3_SUBTRACT,
        &args![current_position, direction, actor_position],
    );
    e.call(LIST_ITEM_SLOT, &args![matrix]);
    let angle = e
        .call(SETTING_FLOAT_VALUE, &args![LIGHT_SETTING_011F2304])
        .f32();
    e.call(MATRIX_FROM_ANGLE, &args![matrix, angle]);
    e.call(VECTOR_NORMALIZE, &args![direction]);
    let rotated_result = e
        .call(MATRIX_TIMES_VECTOR, &args![rotated, direction, matrix])
        .u32();
    let rotated_value = read_vec3(e, rotated_result);
    write_vec3(e, direction, rotated_value);
    let scale = e
        .call(SETTING_FLOAT_VALUE, &args![LIGHT_SETTING_011F222C])
        .f32();
    e.call(POINT3_SCALE, &args![direction, scale]);
    let moved = e
        .call(POINT3_ADD, &args![bound_centre, sum, direction])
        .u32();
    let moved_value = read_vec3(e, moved);
    write_vec3(e, bound_centre, moved_value);
    let lift = e
        .call(SETTING_FLOAT_VALUE, &args![LIGHT_SETTING_011F220C])
        .f32();
    let z = e.mem.f32(bound_centre + 8);
    e.mem
        .set_f32(bound_centre + 8, (lift as f64 + z as f64) as f32);

    let setting_z = setting_float(e, LIGHT_SETTING_011F22CC);
    let setting_y = setting_float(e, LIGHT_SETTING_011F22A8);
    let setting_x = setting_float(e, LIGHT_SETTING_011F21F4);
    e.call(
        POINT3_CONSTRUCTOR,
        &args![light_direction, setting_x, setting_y, setting_z],
    );
    e.mem.set_u32(trace_a, 0);
    e.mem.set_f32(trace_b, 0.0);
    e.mem.set_f32(trace_c, 0.0);
    let height = e.call(ACTOR_GET_HEIGHT, &args![actor]).f32();
    let height_vector = e
        .call(
            POINT3_CONSTRUCTOR,
            &args![scratch + 0xd0, 0.0f32, 0.0f32, height],
        )
        .u32();
    let actor_position = e.vcall(actor, 0x1f4, &args![]).u32();
    e.call(POINT3_ADD, &args![actor_position, ground, height_vector]);
    let handle = held_light_handle(e, this);
    let current = e.global::<u32>(CURRENT_ACTOR);
    let current_position = e.vcall(current, 0x1f4, &args![]).u32();
    let current_vector = read_vec3(e, current_position);
    let ground_vector = read_vec3(e, ground);
    let node = e.call(SCENE_NODE_BY_INDEX, &args![0u32]).u32();
    let traced = e
        .call(
            LIGHT_TRACE,
            &args![
                node,
                ground_vector[0],
                ground_vector[1],
                ground_vector[2],
                trace_a,
                trace_b,
                trace_c,
                current_vector[0],
                current_vector[1],
                current_vector[2],
                handle
            ],
        )
        .f64();
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    let traced = (traced * hundred) as f32;
    let setting_b = e
        .call(SETTING_FLOAT_VALUE, &args![LIGHT_SETTING_011F2200])
        .f32();
    let setting_c = e
        .call(SETTING_FLOAT_VALUE, &args![LIGHT_SETTING_011F2298])
        .f32();
    let remapped = e
        .call(
            REMAP_FLOAT,
            &args![1.0f32, 0.0f32, setting_c, setting_b, traced],
        )
        .f32();
    let at_least = e.call(FLOAT_MAX, &args![0.0f32, remapped]).f32();
    let dimmer = e.call(FLOAT_MIN, &args![1.0f32, at_least]).f32();
    e.call(POINT3_SCALE, &args![light_direction, dimmer]);

    let object = e.vcall(actor, 0x1d0, &args![]).u32();
    let bound = e.call(GET_WORLD_BOUND, &args![object]).u32();
    let bound_slot = e.call(LIST_ITEM_SLOT, &args![bound]).u32();
    let extent = e
        .call(
            POINT3_SUBTRACT,
            &args![bound_centre, extent_vector, bound_slot],
        )
        .u32();
    let length = e.call(POINT3_LENGTH, &args![extent]).f64();
    let doubled = (length + length) as f32;
    let handle = held_light_handle(e, this);
    e.call(LIGHT_SET_POSITION, &args![handle, bound_centre]);
    let radius = e
        .call(
            POINT3_CONSTRUCTOR,
            &args![radius_vector, doubled, 0.0f32, 0.0f32],
        )
        .u32();
    let handle = held_light_handle(e, this);
    e.call(LIGHT_SET_DIRECTION, &args![handle, radius]);
    let handle = held_light_handle(e, this);
    e.call(LIGHT_SET_COLOUR, &args![handle, light_direction]);
    let handle = held_light_handle(e, this);
    e.call(LIGHT_SET_DIMMER, &args![handle, 0u32]);
    e.mem.free(scratch);
}

// Translated from 009c9e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes every command whose target is `reference` from the command list (the head node is
/// popped in place, a later node is unlinked); the target reference global is cleared when it was
/// this reference.
pub fn fn_009c9e90(e: &mut Engine, this: Ptr<Vats>, reference: u32) {
    if reference == e.global::<u32>(TARGET_REFERENCE) {
        e.set_global(TARGET_REFERENCE, 0u32);
    }
    let mut previous = 0u32;
    let mut node = this.addr();
    while node != 0 && !list_is_empty(e, node) {
        let command = list_item(e, node);
        if e.mem.u32(command.wrapping_add(0xc)) == reference {
            if command != 0 {
                fn_009c8870(e, Ptr::new(command), 1);
            }
            if previous == 0 {
                e.call(LIST_POP_FRONT, &args![node]);
            } else {
                remove_list_item(e, previous, command);
                node = list_next(e, previous);
            }
        } else {
            previous = node;
            node = list_next(e, node);
        }
    }
}

// Translated from 009c9f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::IsMeleeCommand` (Xbox PDB): true when the command's action is one of 0, 1, 2, 0x10,
/// 0x11, 0x14, 0x15. With a null `command` the first item of the list is used instead.
pub fn vats_is_melee_command(e: &mut Engine, this: Ptr<Vats>, command: Ptr<VatsCommand>) -> bool {
    let mut command = command.addr();
    if command == 0 {
        command = list_item(e, this.addr());
    }
    if command == 0 {
        return false;
    }
    matches!(e.mem.u32(command), 0 | 1 | 2 | 0x10 | 0x11 | 0x14 | 0x15)
}

// Translated from 009c9fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a position into `out` and returns `out`. It starts as the centre of the current actor's
/// model bound. When the first command has a target, or the target reference global is set, that
/// reference's 3D bound centre (or its position without a 3D object) is the aim point; the vector
/// from the start to the aim point is normalized and rebuilt (times its own length), multiplied by
/// `distance`, divided by the float at `0x01016410` and added to the start.
pub fn fn_009c9fe0(e: &mut Engine, this: Ptr<Vats>, out: Ptr, distance: f32) -> Ptr {
    let actor = e.global::<u32>(CURRENT_ACTOR);
    let model = e.call(ACTOR_MODEL_OBJECT, &args![actor]).u32();
    let bound = e.call(GET_WORLD_BOUND, &args![model]).u32();
    let bound_slot = e.call(LIST_ITEM_SLOT, &args![bound]).u32();
    let mut start = read_vec3(e, bound_slot);
    let first = list_item(e, this.addr());
    let command_target = if first != 0 {
        e.mem.u32(first.wrapping_add(0xc))
    } else {
        0
    };
    let reference = e.global::<u32>(TARGET_REFERENCE);
    if command_target != 0 || reference != 0 {
        // The exe starts the aim point as the vector at 0x011f426c; both branches overwrite it.
        let aimed = if command_target != 0 {
            command_target
        } else {
            reference
        };
        let object = e.vcall(aimed, 0x1d0, &args![]).u32();
        let aim = if object != 0 {
            let object = e.vcall(aimed, 0x1d0, &args![]).u32();
            let bound = e.call(GET_WORLD_BOUND, &args![object]).u32();
            let slot = e.call(LIST_ITEM_SLOT, &args![bound]).u32();
            read_vec3(e, slot)
        } else {
            let position = e.vcall(aimed, 0x1f4, &args![]).u32();
            read_vec3(e, position)
        };
        let start_block = e.mem.alloc(12);
        write_vec3(e, start_block, start);
        let aim_block = e.mem.alloc(12);
        write_vec3(e, aim_block, aim);
        let direction = e.mem.alloc(12);
        e.call(POINT3_SUBTRACT, &args![aim_block, direction, start_block]);
        let length = e.call(POINT3_UNITIZE, &args![direction]).f32();
        let offset: f32 = e.global(OFFSET_FLOAT);
        let scaled_a = e.mem.alloc(12);
        let scaled_b = e.mem.alloc(12);
        let scaled_c = e.mem.alloc(12);
        let first_scale = e
            .call(POINT3_TIMES, &args![direction, scaled_a, length])
            .u32();
        let second_scale = e
            .call(POINT3_TIMES, &args![first_scale, scaled_b, distance])
            .u32();
        let moved = e
            .call(
                POINT3_TIMES_RECIPROCAL,
                &args![second_scale, scaled_c, offset],
            )
            .u32();
        e.call(POINT3_ADD_ASSIGN, &args![start_block, moved]);
        start = read_vec3(e, start_block);
        for block in [
            start_block,
            aim_block,
            direction,
            scaled_a,
            scaled_b,
            scaled_c,
        ] {
            e.mem.free(block);
        }
    }
    write_vec3(e, out.addr(), start);
    out
}

// Translated from 009ca1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts a VATS playback with action 15: quits any playback, appends a new command (action 15,
/// success set, the player as target), switches to mode 4 and sets the HUD menu mode to 4.
// Translation incomplete: 009ca1f0 - C++ exception-unwinding frame (FS:[0], handler 00eeb0fb, state 0 around the command allocation) not translated
pub fn fn_009ca1f0(e: &mut Engine, this: Ptr<Vats>) {
    vats_quit_vats_playback(e, this, 0, 0);
    let command = new_command(e);
    e.set(command, VatsCommand::eAction, 0xfu32);
    e.set(command, VatsCommand::bSuccess, true);
    let player = e.global::<u32>(PLAYER_POINTER);
    e.set(command, VatsCommand::pTarget, Ptr::new(player));
    fn_009c71a0(e, this, command.addr());
    vats_set_mode(e, this, 4, 0);
    e.call(HUD_SET_MENU_MODE, &args![4u32]);
}

/// Allocates and constructs a `VATS_COMMAND` (0x28 bytes). When the allocation fails the exe goes
/// on with a null command, so this does too.
fn new_command(e: &mut Engine) -> Ptr<VatsCommand> {
    let block = e.call(ALLOCATE, &args![0x28u32]).u32();
    if block == 0 {
        Ptr::NULL
    } else {
        Ptr::new(e.call(COMMAND_CONSTRUCTOR, &args![block]).u32())
    }
}

// Translated from 009ca2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts a VATS playback with action 18 against `target`: quits any playback, appends a new
/// command (action 18, success set) whose hit data gets the limb (`-1` becomes 0) at `+0x10` and,
/// when the player's process has a value at its virtual call `+0x148`, that value's number at
/// `+0x30`; the hit data's counter (`0x10`, 1) is set, the player becomes the current actor and
/// the mode becomes 4.
// Translation incomplete: 009ca2c0 - C++ exception-unwinding frame (FS:[0], handler 00ef2e1b, state 0 around the command allocation) not translated
pub fn fn_009ca2c0(e: &mut Engine, this: Ptr<Vats>, target: u32, limb: i32) {
    vats_quit_vats_playback(e, this, 0, 0);
    let command = new_command(e);
    e.set(command, VatsCommand::eAction, 0x12u32);
    e.set(command, VatsCommand::bSuccess, true);
    e.set(command, VatsCommand::pTarget, Ptr::new(target));
    let limb = if limb == -1 { 0 } else { limb };
    let hit_data = e.get(command, VatsCommand::pHitData).addr();
    e.mem.set_i32(hit_data.wrapping_add(0x10), limb);
    let player = e.global::<u32>(PLAYER_POINTER);
    if e.call(ACTOR_GET_PROCESS, &args![player]).u32() != 0 {
        let process = e.call(ACTOR_GET_PROCESS, &args![player]).u32();
        if e.vcall(process, 0x148, &args![]).u32() != 0 {
            let process = e.call(ACTOR_GET_PROCESS, &args![player]).u32();
            let value = e.vcall(process, 0x148, &args![]).u32();
            let number = e.call(WORD_AT_0X08, &args![value]).u32();
            let hit_data = e.get(command, VatsCommand::pHitData).addr();
            e.mem.set_u32(hit_data.wrapping_add(0x30), number);
        }
    }
    let hit_data = e.get(command, VatsCommand::pHitData).addr();
    e.call(HIT_DATA_SET_COUNTER, &args![hit_data, 0x10u32, 1u32]);
    e.set_global(CURRENT_ACTOR, player);
    fn_009c71a0(e, this, command.addr());
    vats_set_mode(e, this, 4, 0);
}

// Translated from 009ca410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VATS::ClearCommandList` (Xbox PDB): frees the melee-attack idle and deletes every command,
/// popping the list until it is empty.
pub fn vats_clear_command_list(e: &mut Engine, this: Ptr<Vats>) {
    clear_melee_attack(e, this);
    let list = this.addr();
    while !list_is_empty(e, list) {
        let head = list_item(e, list);
        if head != 0 {
            fn_009c8870(e, Ptr::new(head), 1);
        }
        e.call(LIST_POP_FRONT, &args![list]);
    }
}

fn current_actor(e: &Engine) -> u32 {
    e.global(CURRENT_ACTOR)
}

// Translated from 009c7240 (decompiled, FalloutNV.exe 1.4.0.525)
/// The per-frame update of the VATS playback (unnamed in the engine map). It stores `mode == 4` in
/// the flag at `0x012677cc` and does nothing else outside mode 4. With commands in the list it
/// advances the camera timers and picks the camera shots for the head command, evaluates the
/// command, plays its camera, starts and frees its action (`FreeHeadAction`) and passes the
/// time that action leaves over to the player's virtual call `+0x33c`; with an empty list the end
/// countdown runs down, and when it ends the VATS mode is left (the kill count turns into an
/// entry-point bonus on an actor value). Finally the target reference's light is repositioned.
pub fn fn_009c7240(e: &mut Engine, this: Ptr<Vats>) {
    let list = this.addr();
    let playing = e.get(this, Vats::eMode) == 4;
    fn_009c86a0(e, playing as u8);
    if !playing {
        return;
    }
    if !list_is_empty(e, list) {
        update_head_command(e, this);
    }
    update_end_countdown(e, this);
    let target = e.global::<u32>(TARGET_REFERENCE);
    if target != 0 && e.call(WORD_AT_0X08, &args![this]).u32() != 0 {
        fn_009c9ae0(e, Ptr::new(VATS_SINGLETON), target);
    }
}

/// The part of `fn_009c7240` that runs while the list has commands (`009c7283` to `009c8307`).
fn update_head_command(e: &mut Engine, this: Ptr<Vats>) {
    let list = this.addr();
    let zero: f64 = e.global(ZERO_DOUBLE);
    let head = list_item(e, list);
    let holder = e.global::<u32>(INTERFACE_HOLDER);
    let sound_object = e.call(HOLDER_GET_OBJECT, &args![holder]).u32();
    let actor = current_actor(e);
    let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
    let mut stage_changed = false;
    let target = e.mem.u32(head.wrapping_add(0xc));
    e.set_global(TARGET_REFERENCE, target);

    // The minimum timer runs down; at 0 the camera shots advance.
    let camera = e.get(this, Vats::pCamera);
    let min_timer = e.get(this, Vats::fCameraMinTimer) as f64;
    if !camera.is_null()
        && e.call(CAMERA_IS_INITIALIZED, &args![camera]).bool()
        && x87_compare(min_timer, zero) & 0x41 == 0
    {
        let elapsed = frame_time(e);
        let timer = (min_timer - elapsed) as f32;
        e.set(this, Vats::fCameraMinTimer, timer);
        if !parity_even(x87_compare(timer as f64, zero) & 0x05) {
            e.set(this, Vats::fCameraMinTimer, 0.0f32);
            let mut more = true;
            while more {
                let camera = e.get(this, Vats::pCamera).addr();
                let count = e.call(WORD_AT_0X38, &args![camera]).i32();
                if e.global::<i32>(CAMERA_STAGE) <= count {
                    break;
                }
                more = vats_use_next_camera(e, this);
            }
        }
    } else if !camera.is_null() && head != e.global::<u32>(LAST_COMMAND) {
        e.call(FREE_CAMERA, &args![camera]);
        e.set_global(CAMERA_STAGE, 0i32);
        e.set(this, Vats::pCamera, Ptr::NULL);
        e.set(this, Vats::pCameraList, Ptr::NULL);
        e.set(this, Vats::fCameraTimer, 0.0f32);
        e.set(this, Vats::fCameraMinTimer, 0.0f32);
    }

    // The shot timer runs down; at 0 the next camera is wanted.
    let camera = e.get(this, Vats::pCamera);
    let shot_timer = e.get(this, Vats::fCameraTimer) as f64;
    if !camera.is_null() && x87_compare(shot_timer, zero) & 0x41 == 0 {
        let elapsed = frame_time(e);
        let timer = (shot_timer - elapsed) as f32;
        e.set(this, Vats::fCameraTimer, timer);
        if !parity_even(x87_compare(timer as f64, zero) & 0x41) {
            stage_changed = true;
            e.set(this, Vats::fCameraTimer, 0.0f32);
            e.set(this, Vats::fCameraMinTimer, 0.0f32);
        }
    }

    // Pick the shot and the camera list for the command.
    let mut first_time = false;
    if e.get(this, Vats::pCameraList).is_null() || stage_changed {
        if e.get(this, Vats::pCameraList).is_null() {
            first_time = true;
            vats_evaluate_command(e, this, Ptr::new(head));
            pick_shot(e, this, head);
        }
        if !e.get(this, Vats::pCameraList).is_null() {
            if stage_changed {
                if !vats_use_next_camera(e, this) {
                    let actor = current_actor(e);
                    let stranger_command = e.mem.u8(head.wrapping_add(6)) != 0;
                    let miss_fortune_command = e.mem.u8(head.wrapping_add(0x24)) != 0;
                    if (actor == e.global::<u32>(STRANGER_ACTOR) && stranger_command)
                        || (actor == e.global::<u32>(MISS_FORTUNE_ACTOR) && miss_fortune_command)
                    {
                        e.mem.set_u8(head.wrapping_add(8), 0);
                    } else {
                        fn_009c8c40(e, this, 0);
                    }
                }
            } else {
                let camera_list = e.get(this, Vats::pCameraList).addr();
                let first_camera = list_item(e, camera_list);
                e.set(this, Vats::pCamera, Ptr::new(first_camera));
                if first_camera != 0 {
                    let minimum = e.call(FLOAT_AT_0X58, &args![first_camera]).f64();
                    let timer = e.get(this, Vats::fCameraMinTimer) as f64;
                    if !parity_even(x87_compare(timer, minimum) & 0x05) {
                        let minimum = e.call(FLOAT_AT_0X58, &args![first_camera]).f32();
                        e.set(this, Vats::fCameraMinTimer, minimum);
                    }
                }
                let camera = e.get(this, Vats::pCamera);
                if !camera.is_null() && fn_009c8600(e, camera) {
                    let player = e.global::<u32>(PLAYER_POINTER);
                    e.call(PLAYER_SET_FIRST_PERSON, &args![player, 1u32]);
                } else {
                    e.set_global(FIRST_PERSON_PENDING, 1u8);
                }
            }
        }
        if first_time {
            vats_update_melee_attack(e, this, Ptr::new(head));
        }
        e.set_global(LAST_COMMAND, head);
        if e.global::<i32>(LAST_COMMAND_COUNTER) > 1 {
            e.set_global(LAST_COMMAND_COUNTER, 0i32);
        }
    }

    // Keep the melee idle queued on the actor's animation.
    let actor = current_actor(e);
    let animation = e.call(ACTOR_GET_ANIMATION, &args![actor]).u32();
    let queued = e
        .call(SMART_POINTER_GET, &args![animation.wrapping_add(0x124)])
        .u32();
    let melee_idle = e.get(this, Vats::pMeleeAttack).addr();
    if melee_idle != 0 {
        let requeue = queued == 0
            || e.call(WORD_AT_0X2C, &args![queued]).u32() != melee_idle
            || (e.call(WORD_AT_0X08, &args![queued]).i32() >= 2
                && e.call(QUEUED_IDLE_FINISHED, &args![queued]).u32() == 0);
        if requeue {
            let section = e
                .call(IDLE_GET_ANIM_GROUP_SECTION, &args![melee_idle])
                .u32();
            let actor = current_actor(e);
            let animation = e.call(ACTOR_GET_ANIMATION, &args![actor]).u32();
            e.call(SPECIAL_IDLE_QUEUE, &args![animation, melee_idle, section]);
        }
    }

    // Camera setup and timing.
    let camera = e.get(this, Vats::pCamera);
    if !camera.is_null() {
        if !e.call(CAMERA_IS_INITIALIZED, &args![camera]).bool() {
            fn_009c95e0(e, this, Ptr::new(head));
            let camera = e.get(this, Vats::pCamera);
            if !camera.is_null() {
                let count = e.call(WORD_AT_0X38, &args![camera]).i32();
                let stage = e.global::<i32>(CAMERA_STAGE);
                if stage >= count || e.call(WORD_AT_0X38, &args![camera]).i32() == 3 {
                    start_camera_shot(e, this, head);
                }
            }
        }
        update_camera_timing(e, this);
    }

    // The head command itself.
    let mut pending = 0.0f32;
    let action = e.mem.u32(head);
    if e.call(IS_ATTACK_ACTION, &args![action]).bool() {
        let player = e.global::<u32>(PLAYER_POINTER);
        if !vats_is_melee_command(e, this, Ptr::new(head))
            && !e.call(IS_ACTION_SEVEN_OR_EIGHT, &args![action]).bool()
            && action != 6
            && e.call(ACTOR_GET_ANIM_ACTION, &args![player]).i32() != 9
            && e.call(ACTOR_GET_ANIM_ACTION, &args![player]).i32() != 0x11
        {
            e.call(HOLDER_OBJECT_APPLY, &args![sound_object, 6u32]);
            e.set_global(SLOW_MOTION_ACTION, 1u8);
        } else {
            e.set_global(SLOW_MOTION_ACTION, 0u8);
        }
        if e.mem.u8(head.wrapping_add(8)) == 0 {
            let mut finished = false;
            let camera = e.get(this, Vats::pCamera);
            let shot_timer = e.get(this, Vats::fCameraMinTimer) as f64;
            if camera.is_null() || !parity_even(x87_compare(shot_timer, zero) & 0x44) {
                if vats_is_melee_command(e, this, Ptr::new(head)) {
                    fn_009c95e0(e, this, Ptr::new(head));
                    let actor = current_actor(e);
                    if e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32() == -1 {
                        finished = true;
                    } else {
                        let animation = e.call(ACTOR_GET_ANIMATION, &args![actor]).u32();
                        let slot = e.call(ANIMATION_SLOT_VALUE, &args![animation, 4u32]).u16();
                        if e.call(SLOT_VALUE_TEST, &args![slot as u32]).bool() {
                            finished = true;
                        }
                    }
                } else if e.get(this, Vats::pLastProjectile).is_null() {
                    finished = true;
                }
            }
            if finished {
                pending = f32::from_bits(x87_quiet_bits(e.mem.u32(head.wrapping_add(0x20))));
                vats_free_head_action(e, this);
            }
        }
        if e.vcall(process, 0x148, &args![]).u32() != 0 {
            let value = e.vcall(process, 0x148, &args![]).u32();
            e.call(WORD_AT_0X08, &args![value]);
        }
    } else if e.global::<u8>(ACTION_STARTED) != 0 {
        let actor = current_actor(e);
        if e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32() == -1 {
            pending = f32::from_bits(x87_quiet_bits(e.mem.u32(head.wrapping_add(0x20))));
            vats_free_head_action(e, this);
        }
    } else {
        let action = e.mem.u32(head);
        match action.wrapping_sub(9) {
            0 => {
                let actor = current_actor(e);
                if e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32() == -1 {
                    if e.vcall(process, 0x148, &args![]).u32() != 0 {
                        let item = e.vcall(process, 0x148, &args![]).u32();
                        let has_effect = e.call(ITEM_HAS_MOD_EFFECT, &args![item, 2u32]).u8();
                        let item = e.vcall(process, 0x148, &args![]).u32();
                        let number = e.call(WORD_AT_0X08, &args![item]).u32();
                        let actor = current_actor(e);
                        e.vcall(actor, 0x3ec, &args![number, 2u32, has_effect as u32, 0u32]);
                        e.vcall(process, 0x6e0, &args![0u32]);
                    }
                    e.set_global(ACTION_STARTED, 1u8);
                }
            }
            1 | 2 => {
                let actor = current_actor(e);
                if e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32() == -1 {
                    if !e.vcall(actor, 0x234, &args![]).bool()
                        && !e.vcall(actor, 0x22c, &args![0u32]).bool()
                        && !e.vcall(actor, 0x230, &args![]).bool()
                        && !e.call(ACTOR_STATE_TEST_FIRST, &args![actor]).bool()
                        && !e.call(ACTOR_STATE_TEST_SECOND, &args![actor]).bool()
                    {
                        let ten = (e.mem.u32(head) == 10) as u32;
                        if actor == e.global::<u32>(PLAYER_POINTER) {
                            let player = e.global::<u32>(PLAYER_POINTER);
                            e.call(PLAYER_START_ACTION, &args![player, 0x400u32, ten]);
                        } else {
                            e.call(ACTOR_START_ACTION, &args![actor, ten]);
                        }
                    }
                    pending = f32::from_bits(x87_quiet_bits(e.mem.u32(head.wrapping_add(0x20))));
                    vats_free_head_action(e, this);
                }
            }
            4 => {
                let actor = current_actor(e);
                if e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32() == -1 {
                    let state = e.call(ACTOR_GET_FLAG_STATE, &args![actor]).bool();
                    e.call(ACTOR_SET_FLAG_STATE, &args![actor, (!state) as u32]);
                    e.set_global(ACTION_STARTED, 1u8);
                }
            }
            _ => {}
        }
    }
    if x87_compare(pending as f64, zero) & 0x41 == 0 {
        let player = e.global::<u32>(PLAYER_POINTER);
        e.vcall(player, 0x33c, &args![pending]);
    }
}

/// Picks the shot for the head command (`009c7433` to `009c751c`): a forced shot, else the second
/// forced shot (cleared when used), else the selector's pick for the current actor and the
/// command's target; then the camera kind (0, 1 or 2) from the shot's tests, and the shot's camera
/// list becomes `pCameraList`.
fn pick_shot(e: &mut Engine, this: Ptr<Vats>, head: u32) {
    let first_forced = e.global::<u32>(FORCED_SHOT_FIRST);
    if first_forced != 0 {
        e.set_global(SELECTED_SHOT, first_forced);
    } else {
        let second_forced = e.global::<u32>(FORCED_SHOT_SECOND);
        if second_forced != 0 {
            e.set_global(SELECTED_SHOT, second_forced);
            e.set_global(FORCED_SHOT_SECOND, 0u32);
        } else {
            let selector = e.global::<u32>(SHOT_SELECTOR);
            let actor = current_actor(e);
            let target = e.mem.u32(head.wrapping_add(0xc));
            let shot = e.call(SELECT_SHOT, &args![selector, actor, target]).u32();
            e.set_global(SELECTED_SHOT, shot);
        }
    }
    let shot = e.global::<u32>(SELECTED_SHOT);
    if shot != 0 {
        if e.call(SHOT_FLAG_FIRST, &args![shot]).bool() {
            e.set_global(CAMERA_KIND, 0u32);
        } else if e.call(SHOT_FLAG_SECOND, &args![shot]).bool() {
            e.set_global(CAMERA_KIND, 1u32);
        } else if e.call(SHOT_FLAG_THIRD, &args![shot]).bool() {
            e.set_global(CAMERA_KIND, 2u32);
        }
    } else {
        e.set_global(CAMERA_KIND, 0u32);
    }
    let shot = e.global::<u32>(SELECTED_SHOT);
    let list = if shot != 0 {
        e.call(SHOT_CAMERA_LIST, &args![shot]).u32()
    } else {
        0
    };
    e.set(this, Vats::pCameraList, Ptr::new(list));
}

/// Starts the head command's camera shot (`009c7717` to `009c7d48`): the shot's time becomes the
/// camera timer (at most the threshold constant, else the setting), the minimum timer is raised to
/// the setting when below it, the camera is initialized for the target (or the current actor), the
/// first and second view targets are chosen by the shot's two mode words (0: the preferred or
/// default target, 1: a tracked reference, 2: the hit body part's node, then the target's
/// object, then the preferred target), and the image-space modifier is created or updated.
fn start_camera_shot(e: &mut Engine, this: Ptr<Vats>, head: u32) {
    let camera = e.get(this, Vats::pCamera).addr();
    let shot_time = e.call(FLOAT_AT_0X54, &args![camera]).f32();
    e.set(this, Vats::fCameraTimer, shot_time);
    let threshold: f64 = e.global(SHOT_TIME_THRESHOLD);
    if !parity_even(x87_compare(shot_time as f64, threshold) & 0x41) {
        let setting = e
            .call(SETTING_FLOAT_POINTER, &args![SHOT_IDLE_LIMIT_SETTING])
            .u32();
        copy_float(e, setting, this.addr().wrapping_add(0x14));
        e.set_global(PLAYBACK_SHOT_TIME, 0.0f32);
    }
    let setting = e
        .call(SETTING_FLOAT_POINTER, &args![MIN_SHOT_TIME_SETTING])
        .u32();
    let minimum = e.mem.f32(setting) as f64;
    let timer = e.get(this, Vats::fCameraMinTimer) as f64;
    if !parity_even(x87_compare(timer, minimum) & 0x05) {
        let setting = e
            .call(SETTING_FLOAT_POINTER, &args![MIN_SHOT_TIME_SETTING])
            .u32();
        copy_float(e, setting, this.addr().wrapping_add(0x18));
    }
    let target = e.mem.u32(head.wrapping_add(0xc));
    let kind = e.global::<u32>(CAMERA_KIND);
    let camera_actor = if target == 0
        || (e.call(WORD_AT_0X3C, &args![camera]).u32() == 0 && e.mem.u32(head) != 0x12)
    {
        current_actor(e)
    } else {
        target
    };
    e.call(INIT_CAMERA, &args![camera, camera_actor, kind]);

    let actor = current_actor(e);
    let animation = e.call(ACTOR_GET_ANIMATION, &args![actor]).u32();
    let animation_part = e.call(WORD_AT_0X0C, &args![animation]).u32();
    let default_node = e
        .call(ARRAY_ELEMENT_BY_INDEX, &args![animation_part, 0u32])
        .u32();

    let mut preferred = 0u32;
    let mut target_object = 0u32;
    let mut body_node = 0u32;
    if e.call(CAMERA_TEST_0058D4F0, &args![camera]).bool() {
        let actor = current_actor(e);
        let process = e.call(ACTOR_GET_PROCESS, &args![actor]).u32();
        let argument = e.call(ACTOR_FACE_VALUE, &args![actor]).u32();
        preferred = e.vcall(process, 0x190, &args![argument]).u32();
    }
    if target != 0 {
        target_object = e.vcall(target, 0x1d0, &args![]).u32();
        if e.vcall(target, 0x100, &args![]).bool() {
            let base = e.call(REFERENCE_BASE, &args![target]).u32();
            let body_data = e.vcall(base, 0x180, &args![]).u32();
            if body_data != 0 {
                e.call(BODY_PART_DATA, &args![body_data]);
                let hit_data = e.mem.u32(head.wrapping_add(0x14));
                let hit_limb = e.mem.u32(hit_data.wrapping_add(0x10));
                if hit_limb != u32::MAX {
                    let limb = e.mem.u32(head.wrapping_add(0x10));
                    let part = e.call(GET_BODY_PART_BY_LIMB, &args![body_data, limb]).u32();
                    if part != 0 {
                        let hit_data = e.mem.u32(head.wrapping_add(0x14));
                        let hit_limb = e.mem.u32(hit_data.wrapping_add(0x10));
                        if e.call(GET_DISMEMBERED, &args![target, hit_limb]).bool() {
                            let target_process = e.call(ACTOR_GET_PROCESS, &args![target]).u32();
                            let hit_data = e.mem.u32(head.wrapping_add(0x14));
                            let hit_limb = e.mem.u32(hit_data.wrapping_add(0x10));
                            let severed = e.vcall(target_process, 0x6d0, &args![hit_limb]).u32();
                            if severed != 0 {
                                let name = e.call(BODY_PART_NODE_NAME, &args![part]).u32();
                                body_node = e.call(FIND_NODE_BY_NAME, &args![severed, name]).u32();
                            }
                        } else {
                            let name = e.call(BODY_PART_NODE_NAME, &args![part]).u32();
                            body_node =
                                e.call(FIND_NODE_BY_NAME, &args![target_object, name]).u32();
                        }
                    }
                }
                if body_node == 0 {
                    let part = e.call(GET_BODY_PART, &args![body_data, 0u32]).u32();
                    if part != 0 {
                        let name = e.call(BODY_PART_NODE_NAME, &args![part]).u32();
                        body_node = e.call(FIND_NODE_BY_NAME, &args![target_object, name]).u32();
                    }
                }
            }
        }
    }

    match e.call(WORD_AT_0X3C, &args![camera]).i32() {
        0 => {
            let view = if preferred != 0 {
                preferred
            } else {
                default_node
            };
            e.call(CAMERA_SET_VIEW_TARGET, &args![camera, view]);
        }
        2 => {
            let view = [body_node, target_object, preferred, default_node]
                .into_iter()
                .find(|node| *node != 0)
                .unwrap_or(default_node);
            e.call(CAMERA_SET_VIEW_TARGET, &args![camera, view]);
        }
        1 => {
            if fn_009c8640(e, Ptr::new(camera)) == 0 {
                let first = e.get(this, Vats::pFirstProjectile).addr();
                let second = e.get(this, Vats::pLastProjectile).addr();
                if first != 0 {
                    let object = e.vcall(first, 0x1d0, &args![]).u32();
                    e.call(CAMERA_SET_VIEW_TARGET, &args![camera, object]);
                } else if second != 0 {
                    let object = e.vcall(second, 0x1d0, &args![]).u32();
                    e.call(CAMERA_SET_VIEW_TARGET, &args![camera, object]);
                } else if preferred != 0 {
                    e.call(CAMERA_SET_SECOND_TARGET, &args![camera, preferred]);
                } else {
                    e.call(CAMERA_SET_SECOND_TARGET, &args![camera, default_node]);
                }
            }
        }
        _ => {}
    }
    match e.call(WORD_AT_0X40, &args![camera]).i32() {
        0 => {
            let view = if preferred != 0 {
                preferred
            } else {
                default_node
            };
            e.call(CAMERA_SET_SECOND_TARGET, &args![camera, view]);
        }
        2 => {
            let view = [body_node, target_object, preferred, default_node]
                .into_iter()
                .find(|node| *node != 0)
                .unwrap_or(default_node);
            e.call(CAMERA_SET_SECOND_TARGET, &args![camera, view]);
        }
        1 => {
            if fn_009c8660(e, Ptr::new(camera)) == 0 {
                let first = e.get(this, Vats::pFirstProjectile).addr();
                let second = e.get(this, Vats::pLastProjectile).addr();
                if first != 0 {
                    start_reference_action(e, camera, first, first);
                } else if second != 0 {
                    start_reference_action(e, camera, first, second);
                } else if preferred != 0 {
                    e.call(CAMERA_SET_SECOND_TARGET, &args![camera, preferred]);
                } else {
                    e.call(CAMERA_SET_SECOND_TARGET, &args![camera, default_node]);
                }
            }
        }
        _ => {}
    }

    let modifier_value = fn_009c8680(e, Ptr::new(camera));
    let modifier = e.get(this, Vats::pImagespaceMod);
    let next = e.call(LIST_NEXT, &args![camera.wrapping_add(0x30)]).u32();
    if !modifier.is_null() {
        let strength = setting_float(e, MODIFIER_STRENGTH_SETTING);
        e.call(
            IMAGESPACE_UPDATE,
            &args![modifier, next, strength, 1.0f32, modifier_value],
        );
    } else {
        let created = e
            .call(
                IMAGESPACE_FORM_TRIGGER,
                &args![next, 1.0f32, modifier_value],
            )
            .u32();
        e.set(this, Vats::pImagespaceMod, Ptr::new(created));
        if created != 0 {
            e.call(IMAGESPACE_ACTIVATE, &args![created, 1u32]);
        }
    }
    let camera = e.get(this, Vats::pCamera);
    if !camera.is_null() && !e.call(CAMERA_IS_INITIALIZED, &args![camera]).bool() {
        vats_use_next_camera(e, this);
    }
}

/// The second-target branch of the camera setup for mode word 1: when the camera shot has flag
/// `0x10` and `tested` has the reference flag `0x10`, that flag is cleared on `reference` and its
/// virtual call `+0x30c` runs; the reference's 3D object becomes the second target. The exe tests
/// the first projectile reference (`+0x1c`) in both uses, even when `reference` is the second one
/// (which it only is when the first is null).
fn start_reference_action(e: &mut Engine, camera: u32, tested: u32, reference: u32) {
    if fn_009c8620(e, Ptr::new(camera))
        && e.call(REFERENCE_FLAG_TEST, &args![tested, 0x10u32]).bool()
    {
        e.call(REFERENCE_FLAG_CHANGE, &args![reference, 0x10u32, 0u32]);
        e.vcall(reference, 0x30c, &args![]);
    }
    let object = e.vcall(reference, 0x1d0, &args![]).u32();
    e.call(CAMERA_SET_SECOND_TARGET, &args![camera, object]);
}

/// The timing block of the update (`009c7d6b` to `009c7f2c`), run while there is a camera shot.
fn update_camera_timing(e: &mut Engine, this: Ptr<Vats>) {
    let zero: f64 = e.global(ZERO_DOUBLE);
    if e.global::<u8>(SLOW_MOTION_ACTIVE) != 0 {
        let blend = e.global::<f32>(SLOW_MOTION_BLEND) as f64;
        if !parity_even(x87_compare(blend, zero) & 0x44) {
            copy_float(e, PLAYBACK_TIME, SLOW_MOTION_SAVED_TIME);
        }
    }
    let elapsed = frame_time(e);
    let time: f32 = e.global(PLAYBACK_TIME);
    e.set_global(PLAYBACK_TIME, (elapsed + time as f64) as f32);
    let elapsed = frame_time(e);
    let shot_time: f32 = e.global(PLAYBACK_SHOT_TIME);
    e.set_global(PLAYBACK_SHOT_TIME, (elapsed + shot_time as f64) as f32);
    let limit_pointer = e
        .call(SETTING_FLOAT_POINTER, &args![SHOT_IDLE_LIMIT_SETTING])
        .u32();
    let limit = e.mem.f32(limit_pointer) as f64;
    let shot_time = e.global::<f32>(PLAYBACK_SHOT_TIME) as f64;
    if !parity_even(x87_compare(limit, shot_time) & 0x05) {
        vats_quit_vats_playback(e, this, 0, 0);
    }
    let camera = e.get(this, Vats::pCamera);
    if !camera.is_null() && e.call(CAMERA_IS_INITIALIZED, &args![camera]).bool() {
        if e.global::<u8>(FIRST_PERSON_PENDING) != 0 {
            let player = e.global::<u32>(PLAYER_POINTER);
            e.call(PLAYER_SET_FIRST_PERSON, &args![player, 0u32]);
            e.set_global(FIRST_PERSON_PENDING, 0u8);
        }
        let time: f32 = e.global(PLAYBACK_TIME);
        e.call(CAMERA_ADVANCE, &args![camera, time]);
        if e.global::<u8>(SLOW_MOTION_ACTIVE) != 0 {
            let multiplier = f32::from_bits(x87_quiet_bits(e.mem.u32(SLOW_FLOAT_A)));
            set_time_multiplier(e, multiplier, 0);
        } else {
            let last = e.global::<u32>(LAST_COMMAND);
            if last != 0 && e.mem.u32(last) == 0x12 {
                let multiplier = f32::from_bits(x87_quiet_bits(e.mem.u32(SLOW_FLOAT_B)));
                set_time_multiplier(e, multiplier, 0);
            } else {
                let multiplier = e.call(FLOAT_AT_0X50, &args![camera]).f32();
                set_time_multiplier(e, multiplier, 0);
            }
        }
        let mut blend = f32::from_bits(x87_quiet_bits(e.mem.u32(SLOW_MOTION_BLEND)));
        if x87_compare(blend as f64, e.global::<f64>(HALF_DOUBLE)) & 0x41 == 0 {
            blend = (1.0 - blend as f64) as f32;
        }
        let factor_pointer = e
            .call(SETTING_FLOAT_POINTER, &args![MODIFIER_LIMIT_SETTING])
            .u32();
        let factor = e.mem.f32(factor_pointer) as f64;
        let strength = (blend as f64 * factor) as f32;
        let transition = e.get(this, Vats::pTransitionImagespaceMod);
        e.call(IMAGESPACE_SET_STRENGTH, &args![transition, strength]);
    } else {
        set_time_multiplier(e, 1.0, 0);
    }
}

/// The part of `fn_009c7240` that runs while the list is empty (`009c8307` to `009c85bd`).
fn update_end_countdown(e: &mut Engine, this: Ptr<Vats>) {
    if !list_is_empty(e, this.addr()) {
        return;
    }
    let zero: f64 = e.global(ZERO_DOUBLE);
    let countdown = e.global::<f32>(END_COUNTDOWN);
    if !parity_even(x87_compare(countdown as f64, zero) & 0x44) {
        let setting = e
            .call(SETTING_FLOAT_POINTER, &args![QUIT_DELAY_SETTING])
            .u32();
        copy_float(e, setting, END_COUNTDOWN);
        return;
    }
    let delta = e.call(TIMER_DELTA, &args![TIMER_OBJECT]).f64();
    let factor = e.call(PLAYER_TIME_FACTOR, &args![]).f64();
    let product = factor * delta;
    let divisor = e.call(TIMER_DIVISOR, &args![]).f64();
    let countdown = (countdown as f64 - product / divisor) as f32;
    e.set_global(END_COUNTDOWN, countdown);
    let player = e.global::<u32>(PLAYER_POINTER);
    if !parity_even(x87_compare(countdown as f64, zero) & 0x41) {
        set_time_multiplier(e, 1.0, 0);
        let actor = current_actor(e);
        let mut finish = actor == e.global::<u32>(STRANGER_ACTOR)
            || actor == e.global::<u32>(MISS_FORTUNE_ACTOR);
        for expected in [-1i32, 7, 9, 0xf, 0x11] {
            if finish {
                break;
            }
            let actor = current_actor(e);
            if e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32() == expected {
                finish = true;
            }
        }
        if finish {
            vats_set_mode(e, this, 0, 0);
            if e.get(this, Vats::iKillCount) > 0 {
                let weapon = e.call(ACTOR_GET_CURRENT_WEAPON, &args![player]).u32();
                let weapon = if weapon != 0 {
                    weapon
                } else {
                    e.global::<u32>(DEFAULT_WEAPON)
                };
                let result = e.mem.alloc(4);
                e.mem.set_f32(result, 0.0);
                e.call(
                    HANDLE_ENTRY_POINT,
                    &args![0x23u32, player, weapon, player, result],
                );
                let restore = e.mem.f32(result);
                e.mem.free(result);
                if x87_compare(restore as f64, zero) & 0x41 == 0 {
                    e.call(ACTOR_RESTORE_ACTOR_VALUE, &args![player, 0xcu32, restore]);
                }
                e.set(this, Vats::iKillCount, 0);
            }
            e.set_global(END_COUNTDOWN, 0.0f32);
        }
    } else {
        let elapsed = frame_time(e);
        let time: f32 = e.global(PLAYBACK_TIME);
        e.set_global(PLAYBACK_TIME, (elapsed + time as f64) as f32);
        let camera = e.get(this, Vats::pCamera);
        if !camera.is_null() && e.call(CAMERA_IS_INITIALIZED, &args![camera]).bool() {
            let time: f32 = e.global(PLAYBACK_TIME);
            e.call(CAMERA_ADVANCE, &args![camera, time]);
        }
        if e.global::<u8>(SLOW_MOTION_ACTIVE) != 0 {
            let multiplier = f32::from_bits(x87_quiet_bits(e.mem.u32(SLOW_FLOAT_A)));
            set_time_multiplier(e, multiplier, 0);
        } else if !camera.is_null() && e.call(CAMERA_IS_INITIALIZED, &args![camera]).bool() {
            let multiplier = e.call(FLOAT_AT_0X50, &args![camera]).f32();
            set_time_multiplier(e, multiplier, 0);
        } else {
            set_time_multiplier(e, 1.0, 0);
        }
        if e.global::<u8>(SLOW_MOTION_ACTION) != 0
            && e.call(ACTOR_GET_ANIM_ACTION, &args![player]).i32() != 9
            && e.call(ACTOR_GET_ANIM_ACTION, &args![player]).i32() != 0x11
        {
            let holder = e.global::<u32>(INTERFACE_HOLDER);
            let object = e.call(HOLDER_GET_OBJECT, &args![holder]).u32();
            e.call(HOLDER_OBJECT_APPLY, &args![object, 6u32]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005ae380, vats_get_count(Ptr<Vats>) -> i32),
        entry!(0x009c6ba0, vats_clear_data(Ptr<Vats>)),
        entry!(0x009c6c30, vats_set_mode(Ptr<Vats>, i32, u8)),
        entry!(0x009c7190, fn_009c7190(u8)),
        entry!(0x009c71a0, fn_009c71a0(Ptr<Vats>, u32)),
        entry!(0x009c71c0, vats_get_current_action(Ptr<Vats>) -> u32),
        entry!(0x009c71f0, fn_009c71f0(Ptr<Vats>) -> u32),
        entry!(0x009c7240, fn_009c7240(Ptr<Vats>)),
        entry!(0x009c8600, fn_009c8600(Ptr) -> bool),
        entry!(0x009c8620, fn_009c8620(Ptr) -> bool),
        entry!(0x009c8640, fn_009c8640(Ptr) -> u32),
        entry!(0x009c8660, fn_009c8660(Ptr) -> u32),
        entry!(0x009c8680, fn_009c8680(Ptr) -> u32),
        entry!(0x009c86a0, fn_009c86a0(u8)),
        entry!(0x009c86b0, vats_free_head_action(Ptr<Vats>)),
        entry!(
            0x009c8870,
            fn_009c8870(Ptr<VatsCommand>, u32) -> Ptr<VatsCommand>
        ),
        entry!(0x009c88a0, fn_009c88a0(Ptr<Vats>, Ptr, Ptr)),
        entry!(0x009c8950, vats_quit_vats_playback(Ptr<Vats>, u8, u8)),
        entry!(0x009c8a90, vats_quit_vats_playback_ov2(Ptr<Vats>, u32, u32)),
        entry!(0x009c8ac0, fn_009c8ac0(Ptr<Vats>, Ptr) -> Ptr),
        entry!(0x009c8b90, fn_009c8b90(Ptr<Vats>, u32)),
        entry!(0x009c8c40, fn_009c8c40(Ptr<Vats>, u32)),
        entry!(0x009c8cc0, vats_get_player_update_mult(Ptr<Vats>) -> f32),
        entry!(0x009c8d60, vats_get_target_update_mult(Ptr<Vats>) -> f32),
        entry!(
            0x009c8e00,
            vats_evaluate_command(Ptr<Vats>, Ptr<VatsCommand>)
        ),
        entry!(0x009c8ea0, fn_009c8ea0() -> bool),
        entry!(0x009c8ef0, fn_009c8ef0() -> f32),
        entry!(
            0x009c8f20,
            vats_get_line_of_sight(Ptr<Vats>, Ptr<VatsCommand>) -> bool
        ),
        entry!(
            0x009c9280,
            vats_update_melee_attack(Ptr<Vats>, Ptr<VatsCommand>)
        ),
        entry!(0x009c95e0, fn_009c95e0(Ptr<Vats>, Ptr<VatsCommand>)),
        entry!(0x009c9900, fn_009c9900(Ptr, u8)),
        entry!(0x009c9920, vats_use_next_camera(Ptr<Vats>) -> bool),
        entry!(0x009c9a50, vats_set_vats_light(Ptr<Vats>, u32, u8)),
        entry!(0x009c9ae0, fn_009c9ae0(Ptr<Vats>, u32)),
        entry!(0x009c9e90, fn_009c9e90(Ptr<Vats>, u32)),
        entry!(
            0x009c9f60,
            vats_is_melee_command(Ptr<Vats>, Ptr<VatsCommand>) -> bool
        ),
        entry!(0x009c9fe0, fn_009c9fe0(Ptr<Vats>, Ptr, f32) -> Ptr),
        entry!(0x009ca1f0, fn_009ca1f0(Ptr<Vats>)),
        entry!(0x009ca2c0, fn_009ca2c0(Ptr<Vats>, u32, i32)),
        entry!(0x009ca410, vats_clear_command_list(Ptr<Vats>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- helpers ------------------------------------------------------------------------------

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| eax(value));
    }

    fn stub_float(e: &mut Engine, address: u32, value: f64) {
        e.register_double(address, move |_, _| st0(value));
    }

    fn stub_all(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            stub(e, address, 0);
        }
    }

    /// The argument words of every logged call of `address`.
    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .expect("the call log is on")
            .iter()
            .filter(|(logged, _)| *logged == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Pages holding the globals, settings and data constants the code reads.
    const PAGES: [u32; 17] = [
        0x011a_f000,
        0x011c_a000,
        0x011c_b000,
        0x011c_e000,
        0x011c_f000,
        0x011d_e000,
        0x011f_2000,
        0x011f_4000,
        0x011f_6000,
        0x011f_9000,
        0x0126_7000,
        0x0101_1000,
        0x0101_2000,
        0x0101_4000,
        0x0101_6000,
        0x0101_7000,
        0x0109_0000,
    ];

    /// An engine with the list helpers and the setting getters standing in for the exe's.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in PAGES {
            e.map(page, 0x1000);
        }
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.mem.set_f64(SIXTY_DOUBLE, 60.0);
        e.mem.set_f64(HUNDRED_DOUBLE, 100.0);
        e.mem.set_f64(SHOT_TIME_THRESHOLD, 0.001);
        e.mem.set_f32(HALF_FLOAT, 0.5);
        e.register(LIST_ITEM_SLOT, |_, a| eax(a[0]));
        e.register(LIST_NEXT, |e, a| eax(e.mem.u32(a[0] + 4)));
        e.register(LIST_IS_EMPTY, |e, a| {
            eax(u32::from(e.mem.u32(a[0] + 4) == 0 && e.mem.u32(a[0]) == 0))
        });
        e.register(LIST_POP_FRONT, |e, a| {
            let head = a[0];
            let next = e.mem.u32(head + 4);
            if next != 0 {
                let after = e.mem.u32(next + 4);
                let item = e.mem.u32(next);
                e.mem.set_u32(head + 4, after);
                e.mem.set_u32(head, item);
            } else {
                e.mem.set_u32(head, 0);
            }
            Ret::default()
        });
        e.register(LIST_REMOVE_ITEM, |e, a| {
            let item = e.mem.u32(a[1]);
            let mut previous = 0;
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == item {
                    let next = e.mem.u32(node + 4);
                    if previous == 0 {
                        if next != 0 {
                            let after = e.mem.u32(next + 4);
                            let value = e.mem.u32(next);
                            e.mem.set_u32(node, value);
                            e.mem.set_u32(node + 4, after);
                        } else {
                            e.mem.set_u32(node, 0);
                        }
                    } else {
                        e.mem.set_u32(previous + 4, next);
                    }
                    break;
                }
                previous = node;
                node = e.mem.u32(node + 4);
            }
            Ret::default()
        });
        e.register(COMMAND_RELEASE_HIT_DATA, |_, _| Ret::default());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(SETTING_FLOAT_POINTER, |_, a| eax(a[0] + 4));
        e.register(SETTING_INT_POINTER, |_, a| eax(a[0] + 4));
        e.register(SMART_POINTER_GET, |e, a| eax(e.mem.u32(a[0])));
        e
    }

    fn new_vats(e: &mut Engine) -> Ptr<Vats> {
        e.new_object::<Vats>()
    }

    /// Writes the items into a list whose first node is at `head`.
    fn fill_list(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if index + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            } else {
                e.mem.set_u32(node + 4, 0);
            }
        }
    }

    fn list_items(e: &Engine, head: u32) -> Vec<u32> {
        let mut items = Vec::new();
        let mut node = head;
        while node != 0 {
            items.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        items
    }

    fn make_command(e: &mut Engine, action: u32) -> u32 {
        let command = e.mem.alloc(0x28);
        e.mem.set_u32(command, action);
        command
    }

    /// An object whose vtable slots (byte offsets) lead to doubles returning the given values.
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x800);
        e.mem.set_u32(object, vtable);
        for &(slot, value) in slots {
            let target = vtable + slot;
            e.mem.set_u32(target, target);
            e.register_double(target, move |_, _| eax(value));
        }
        object
    }

    fn slot_target(e: &Engine, object: u32, slot: u32) -> u32 {
        e.mem.u32(e.mem.u32(object) + slot)
    }

    fn set_setting(e: &mut Engine, setting: u32, value: f32) {
        e.mem.set_f32(setting + 4, value);
    }

    fn float_bits(value: f32) -> u32 {
        value.to_bits()
    }

    /// The doubles `SetMode(0)` needs for the calls it makes.
    fn mode_zero_doubles(e: &mut Engine) {
        stub_all(
            e,
            &[
                SET_GLOBAL_TIME_MULTIPLIER,
                FREE_CAMERA,
                IMAGESPACE_STOP,
                ACTOR_SET_IRON_SIGHTS,
                ACTOR_SET_BLOCK,
                HOLDER_GET_OBJECT,
                HOLDER_OBJECT_SELECT,
                ACTOR_UNEQUIP_OBJECT,
                HUD_SET_MENU_MODE,
                GET_VATS_LIGHT,
            ],
        );
    }

    /// The doubles `SetMode(4)` needs.
    fn mode_four_doubles(e: &mut Engine) {
        stub_all(e, &[IMAGESPACE_STOP, IMAGESPACE_ACTIVATE, GET_VATS_LIGHT]);
        e.register(POINT2_CONSTRUCTOR, |_, a| eax(a[0]));
        stub(e, IMAGESPACE_RB_TRIGGER, 0x777);
    }

    // ---- x87 helpers --------------------------------------------------------------------------

    #[test]
    fn x87_flags_follow_the_status_word() {
        assert_eq!(x87_compare(2.0, 1.0), 0x00);
        assert_eq!(x87_compare(1.0, 2.0), 0x01);
        assert_eq!(x87_compare(1.0, 1.0), 0x40);
        assert_eq!(x87_compare(f64::NAN, 1.0), 0x45);
        assert_eq!(x87_compare(1.0, f64::NAN), 0x45);
        assert!(parity_even(0x00));
        assert!(parity_even(0x44));
        assert!(!parity_even(0x40));
        assert!(!parity_even(0x04));
        assert_eq!(x87_quiet_bits(0x7f80_0001), 0x7fc0_0001);
        assert_eq!(x87_quiet_bits(0x3f80_0000), 0x3f80_0000);
        assert_eq!(x87_quiet_bits(0x0000_0001), 0x0000_0001);
    }

    // ---- the small functions ------------------------------------------------------------------

    #[test]
    fn get_count_counts_the_non_null_items() {
        let mut e = engine();
        let this = new_vats(&mut e);
        fill_list(&mut e, this.addr(), &[3, 0, 9]);
        assert_eq!(vats_get_count(&mut e, this), 2);
    }

    #[test]
    fn clear_data_frees_the_camera_and_stops_the_modifiers() {
        let mut e = engine();
        stub_all(&mut e, &[FREE_CAMERA, IMAGESPACE_STOP]);
        let this = new_vats(&mut e);
        e.set(this, Vats::pCamera, Ptr::new(0x1111));
        e.set(this, Vats::pCameraList, Ptr::new(0x2222));
        e.set(this, Vats::fCameraTimer, 3.0f32);
        e.set(this, Vats::fCameraMinTimer, 4.0f32);
        e.set(this, Vats::pImagespaceMod, Ptr::new(0x3333));
        e.set(this, Vats::pTransitionImagespaceMod, Ptr::new(0x4444));
        e.call_log = Some(vec![]);
        vats_clear_data(&mut e, this);
        assert_eq!(calls(&e, FREE_CAMERA), vec![vec![0x1111]]);
        assert_eq!(calls(&e, IMAGESPACE_STOP), vec![vec![0x3333], vec![0x4444]]);
        assert!(e.get(this, Vats::pCamera).is_null());
        assert!(e.get(this, Vats::pCameraList).is_null());
        assert_eq!(e.get(this, Vats::fCameraTimer), 0.0);
        assert_eq!(e.get(this, Vats::fCameraMinTimer), 0.0);
        assert!(e.get(this, Vats::pImagespaceMod).is_null());
        assert!(e.get(this, Vats::pTransitionImagespaceMod).is_null());
    }

    #[test]
    fn mode_two_flag_and_playing_back_flag_store_their_byte() {
        let mut e = engine();
        fn_009c7190(&mut e, 1);
        fn_009c86a0(&mut e, 1);
        assert_eq!(e.global::<u8>(MODE_TWO_FLAG), 1);
        assert_eq!(e.global::<u8>(PLAYING_BACK_FLAG), 1);
        fn_009c7190(&mut e, 0);
        fn_009c86a0(&mut e, 0);
        assert_eq!(e.global::<u8>(MODE_TWO_FLAG), 0);
        assert_eq!(e.global::<u8>(PLAYING_BACK_FLAG), 0);
    }

    #[test]
    fn append_passes_the_address_of_a_slot_holding_the_command() {
        let mut e = engine();
        e.register(LIST_APPEND, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0] + 0x40, item);
            Ret::default()
        });
        let this = e.mem.alloc(0x48);
        fn_009c71a0(&mut e, Ptr::new(this), 0x1234);
        assert_eq!(e.mem.u32(this + 0x40), 0x1234);
    }

    #[test]
    fn current_action_is_the_first_item_or_null() {
        let mut e = engine();
        let this = new_vats(&mut e);
        assert_eq!(vats_get_current_action(&mut e, this), 0);
        fill_list(&mut e, this.addr(), &[0xa1, 0xa2]);
        assert_eq!(vats_get_current_action(&mut e, this), 0xa1);
    }

    #[test]
    fn last_item_is_the_item_of_the_node_without_a_next_node() {
        let mut e = engine();
        let this = new_vats(&mut e);
        assert_eq!(fn_009c71f0(&mut e, this), 0);
        fill_list(&mut e, this.addr(), &[0xa1, 0xa2, 0xa3]);
        assert_eq!(fn_009c71f0(&mut e, this), 0xa3);
    }

    #[test]
    fn shot_flag_tests_read_the_flags_word() {
        let mut e = engine();
        let shot = e.mem.alloc(0x80);
        assert!(!fn_009c8600(&mut e, Ptr::new(shot)));
        assert!(!fn_009c8620(&mut e, Ptr::new(shot)));
        e.mem.set_u32(shot + 0x44, 0x08);
        assert!(fn_009c8600(&mut e, Ptr::new(shot)));
        assert!(!fn_009c8620(&mut e, Ptr::new(shot)));
        e.mem.set_u32(shot + 0x44, 0x10);
        assert!(!fn_009c8600(&mut e, Ptr::new(shot)));
        assert!(fn_009c8620(&mut e, Ptr::new(shot)));
        fn_009c9900(&mut e, Ptr::new(shot), 1);
        assert_eq!(e.mem.u8(shot + 0x76), 1);
    }

    #[test]
    fn shot_pointer_getters_read_their_fields() {
        let mut e = engine();
        let shot = e.mem.alloc(0x80);
        e.mem.set_u32(shot + 0x60, 7);
        e.mem.set_u32(shot + 0x64, 8);
        e.mem.set_u32(shot + 0x70, 9);
        assert_eq!(fn_009c8640(&mut e, Ptr::new(shot)), 7);
        assert_eq!(fn_009c8660(&mut e, Ptr::new(shot)), 8);
        assert_eq!(fn_009c8680(&mut e, Ptr::new(shot)), 9);
    }

    #[test]
    fn command_destructor_frees_the_block_only_with_the_flag() {
        let mut e = engine();
        let command = make_command(&mut e, 1);
        e.call_log = Some(vec![]);
        let result = fn_009c8870(&mut e, Ptr::new(command), 0);
        assert_eq!(result.addr(), command);
        assert_eq!(calls(&e, COMMAND_RELEASE_HIT_DATA), vec![vec![command]]);
        assert!(calls(&e, FREE_BLOCK).is_empty());
        fn_009c8870(&mut e, Ptr::new(command), 1);
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![command]]);
    }

    #[test]
    fn pop_last_command_reports_its_action_and_iron_sights() {
        let mut e = engine();
        let this = new_vats(&mut e);
        let first = make_command(&mut e, 0x10);
        let last = make_command(&mut e, 0x14);
        e.mem.set_u8(last + 5, 1);
        fill_list(&mut e, this.addr(), &[first, last]);
        let outputs = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        fn_009c88a0(&mut e, this, Ptr::new(outputs), Ptr::new(outputs + 4));
        assert_eq!(e.mem.u32(outputs), 0x14);
        assert_eq!(e.mem.u8(outputs + 4), 1);
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![last]]);
        assert_eq!(list_items(&e, this.addr()), vec![first]);
    }

    #[test]
    fn pop_last_command_does_nothing_for_an_empty_list() {
        let mut e = engine();
        let this = new_vats(&mut e);
        let outputs = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        fn_009c88a0(&mut e, this, Ptr::new(outputs), Ptr::new(outputs + 4));
        assert!(calls(&e, FREE_BLOCK).is_empty());
        assert_eq!(e.mem.u32(outputs), 0);
    }

    // ---- ClearCommandList, FreeHeadAction -----------------------------------------------------

    #[test]
    fn clear_command_list_frees_the_melee_idle_and_every_command() {
        let mut e = engine();
        stub(&mut e, ACTOR_GET_ANIMATION, 0x900);
        stub(&mut e, SPECIAL_IDLE_FREE, 0);
        let this = new_vats(&mut e);
        e.set_global(CURRENT_ACTOR, 0x5000u32);
        e.set(this, Vats::pMeleeAttack, Ptr::new(0x8000));
        let first = make_command(&mut e, 1);
        let second = make_command(&mut e, 2);
        fill_list(&mut e, this.addr(), &[first, second]);
        e.call_log = Some(vec![]);
        vats_clear_command_list(&mut e, this);
        assert_eq!(calls(&e, SPECIAL_IDLE_FREE), vec![vec![0x900, 1, 0]]);
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![first], vec![second]]);
        assert!(e.get(this, Vats::pMeleeAttack).is_null());
        assert_eq!(list_items(&e, this.addr()), vec![0]);
    }

    #[test]
    fn free_head_action_removes_commands_until_one_stays() {
        let mut e = engine();
        stub(&mut e, STAT_INCREMENT, 0);
        stub_float(&mut e, TARGET_LAST_VALUE, 1.0);
        e.mem.set_u32(HEAD_ACTION_LIMIT_SETTING + 4, 5);
        let this = new_vats(&mut e);
        let released = object_with_slots(&mut e, &[(0x22c, 1)]);
        let kept_target = object_with_slots(&mut e, &[(0x22c, 0)]);
        let first = make_command(&mut e, 3);
        let second = make_command(&mut e, 3);
        e.mem.set_u32(second + 0xc, released);
        let third = make_command(&mut e, 3);
        e.mem.set_u32(third + 0xc, kept_target);
        e.mem.set_u8(third + 6, 1);
        fill_list(&mut e, this.addr(), &[first, second, third]);
        e.set_global(STRANGER_ACTOR, 0x5100u32);
        e.call_log = Some(vec![]);
        vats_free_head_action(&mut e, this);
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![first], vec![second]]);
        assert_eq!(list_items(&e, this.addr()), vec![third]);
        assert_eq!(e.global::<u32>(CURRENT_ACTOR), 0x5100);
        assert_eq!(calls(&e, STAT_INCREMENT), vec![vec![0x19]]);
    }

    #[test]
    fn free_head_action_removes_a_stranger_command_whose_target_value_is_high() {
        let mut e = engine();
        stub(&mut e, STAT_INCREMENT, 0);
        stub_float(&mut e, TARGET_LAST_VALUE, 10.0);
        e.mem.set_u32(HEAD_ACTION_LIMIT_SETTING + 4, 5);
        let this = new_vats(&mut e);
        let target = object_with_slots(&mut e, &[(0x22c, 0)]);
        let first = make_command(&mut e, 3);
        let second = make_command(&mut e, 3);
        e.mem.set_u32(second + 0xc, target);
        e.mem.set_u8(second + 6, 1);
        fill_list(&mut e, this.addr(), &[first, second]);
        e.call_log = Some(vec![]);
        vats_free_head_action(&mut e, this);
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![first], vec![second]]);
        assert_eq!(list_items(&e, this.addr()), vec![0]);
        assert!(calls(&e, STAT_INCREMENT).is_empty());
    }

    #[test]
    fn free_head_action_marks_a_miss_fortune_head() {
        let mut e = engine();
        stub(&mut e, STAT_INCREMENT, 0);
        let this = new_vats(&mut e);
        let first = make_command(&mut e, 3);
        let second = make_command(&mut e, 3);
        e.mem.set_u8(second + 0x24, 1);
        fill_list(&mut e, this.addr(), &[first, second]);
        e.set_global(MISS_FORTUNE_ACTOR, 0x5200u32);
        e.call_log = Some(vec![]);
        vats_free_head_action(&mut e, this);
        assert_eq!(e.global::<u32>(CURRENT_ACTOR), 0x5200);
        assert_eq!(calls(&e, STAT_INCREMENT), vec![vec![0x1c]]);
        let empty = new_vats(&mut e);
        e.call_log = Some(vec![]);
        vats_free_head_action(&mut e, empty);
        assert!(calls(&e, FREE_BLOCK).is_empty());
    }

    // ---- QuitVATSPlayback ---------------------------------------------------------------------

    #[test]
    fn quit_playback_does_nothing_in_mode_zero() {
        let mut e = engine();
        let this = new_vats(&mut e);
        e.call_log = Some(vec![]);
        vats_quit_vats_playback(&mut e, this, 1, 0);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn quit_playback_clears_the_list_and_starts_the_countdown() {
        let mut e = engine();
        stub(&mut e, SET_GLOBAL_TIME_MULTIPLIER, 0);
        set_setting(&mut e, QUIT_DELAY_SETTING, 7.0);
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        let command = make_command(&mut e, 3);
        fill_list(&mut e, this.addr(), &[command]);
        e.call_log = Some(vec![]);
        vats_quit_vats_playback(&mut e, this, 1, 0);
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![command]]);
        assert_eq!(e.global::<f32>(END_COUNTDOWN), 7.0);
        assert_eq!(
            calls(&e, SET_GLOBAL_TIME_MULTIPLIER),
            vec![vec![TIMER_OBJECT, float_bits(1.0), 0]]
        );
    }

    #[test]
    fn quit_playback_keeps_the_current_command_and_drops_the_rest() {
        let mut e = engine();
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        let first = make_command(&mut e, 3);
        e.mem.set_u8(first + 8, 3);
        e.mem.set_u8(first + 9, 2);
        let second = make_command(&mut e, 3);
        let third = make_command(&mut e, 3);
        fill_list(&mut e, this.addr(), &[first, second, third]);
        e.call_log = Some(vec![]);
        vats_quit_vats_playback(&mut e, this, 1, 1);
        assert_eq!(e.mem.u8(first + 8), 1);
        assert_eq!(e.mem.u8(first + 9), 0);
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![second], vec![third]]);
        assert_eq!(list_items(&e, this.addr()), vec![first]);
    }

    #[test]
    fn quit_playback_ignores_action_fifteen_unless_forced() {
        let mut e = engine();
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        let command = make_command(&mut e, 0xf);
        fill_list(&mut e, this.addr(), &[command]);
        e.call_log = Some(vec![]);
        vats_quit_vats_playback(&mut e, this, 0, 1);
        assert!(calls(&e, FREE_BLOCK).is_empty());
        assert_eq!(list_items(&e, this.addr()), vec![command]);
    }

    #[test]
    fn quit_playback_waits_while_the_countdown_runs_with_an_empty_list() {
        let mut e = engine();
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        e.set_global(END_COUNTDOWN, 2.0f32);
        e.call_log = Some(vec![]);
        vats_quit_vats_playback(&mut e, this, 1, 0);
        assert!(e.call_log.as_ref().unwrap().len() == 1);
        assert_eq!(e.global::<f32>(END_COUNTDOWN), 2.0);
    }

    #[test]
    fn quit_playback_ov2_stores_the_unequip_data() {
        let mut e = engine();
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        vats_quit_vats_playback_ov2(&mut e, this, 0x1010, 0x2020);
        assert_eq!(e.get(this, Vats::pUnEquipWeapon).addr(), 0x1010);
        assert_eq!(e.get(this, Vats::pUnEquipExtra).addr(), 0x2020);
    }

    // ---- positions ----------------------------------------------------------------------------

    fn vec_block(e: &mut Engine, x: f32, y: f32, z: f32) -> u32 {
        let block = e.mem.alloc(12);
        e.mem.set_f32(block, x);
        e.mem.set_f32(block + 4, y);
        e.mem.set_f32(block + 8, z);
        block
    }

    #[test]
    fn look_at_position_defaults_to_the_global_vector() {
        let mut e = engine();
        e.mem.set_f32(DEFAULT_POSITION, 1.0);
        e.mem.set_f32(DEFAULT_POSITION + 4, 2.0);
        e.mem.set_f32(DEFAULT_POSITION + 8, 3.0);
        stub(&mut e, POINT3_EQUALS, 0);
        let this = new_vats(&mut e);
        let out = e.mem.alloc(12);
        let result = fn_009c8ac0(&mut e, this, Ptr::new(out));
        assert_eq!(result.addr(), out);
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(e.mem.f32(out + 4), 2.0);
        assert_eq!(e.mem.f32(out + 8), 3.0);
    }

    #[test]
    fn look_at_position_uses_the_shot_position() {
        let mut e = engine();
        let shot_position = vec_block(&mut e, 9.0, 8.0, 7.0);
        stub(&mut e, CAMERA_POSITION, shot_position);
        stub(&mut e, POINT3_EQUALS, 0);
        let this = new_vats(&mut e);
        e.set(this, Vats::pCamera, Ptr::new(0x1234));
        let out = e.mem.alloc(12);
        fn_009c8ac0(&mut e, this, Ptr::new(out));
        assert_eq!(e.mem.f32(out), 9.0);
        assert_eq!(e.mem.f32(out + 8), 7.0);
    }

    #[test]
    fn look_at_position_offsets_the_model_position_when_it_equals_the_default() {
        let mut e = engine();
        stub(&mut e, POINT3_EQUALS, 1);
        stub_float(&mut e, PLAYER_FLOAT, 70.0);
        e.register(POINT3_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            e.mem.set_u32(a[0] + 8, a[3]);
            eax(a[0])
        });
        stub(&mut e, ACTOR_MODEL_OBJECT, 0x10);
        stub(&mut e, BOUND_NEXT_PART, 0x20);
        let sum = vec_block(&mut e, 5.0, 6.0, 7.0);
        e.register_double(POINT3_ADD, move |e, a| {
            // The offset vector built from the player float is (0, 0, 70).
            assert_eq!(e.mem.f32(a[2] + 8), 70.0);
            assert_eq!(a[0], 0x20);
            eax(sum)
        });
        let this = new_vats(&mut e);
        let out = e.mem.alloc(12);
        fn_009c8ac0(&mut e, this, Ptr::new(out));
        assert_eq!(e.mem.f32(out), 5.0);
        assert_eq!(e.mem.f32(out + 4), 6.0);
        assert_eq!(e.mem.f32(out + 8), 7.0);
    }

    // ---- reference setters --------------------------------------------------------------------

    #[test]
    fn first_reference_setter_retargets_the_camera() {
        let mut e = engine();
        stub(&mut e, WORD_AT_0X3C, 1);
        stub(&mut e, WORD_AT_0X40, 1);
        stub_all(&mut e, &[CAMERA_SET_VIEW_TARGET, CAMERA_SET_SECOND_TARGET]);
        let reference = object_with_slots(&mut e, &[(0x1d0, 0x77)]);
        let this = new_vats(&mut e);
        e.set(this, Vats::pCamera, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        fn_009c8b90(&mut e, this, reference);
        assert_eq!(calls(&e, CAMERA_SET_VIEW_TARGET), vec![vec![0x1234, 0x77]]);
        assert_eq!(
            calls(&e, CAMERA_SET_SECOND_TARGET),
            vec![vec![0x1234, 0x77]]
        );
        assert_eq!(e.get(this, Vats::pFirstProjectile).addr(), reference);
        fn_009c8b90(&mut e, this, 0);
        assert_eq!(calls(&e, CAMERA_SET_VIEW_TARGET)[1], vec![0x1234, 0]);
        assert!(e.get(this, Vats::pFirstProjectile).is_null());
    }

    #[test]
    fn first_reference_setter_leaves_the_camera_alone_for_other_selectors() {
        let mut e = engine();
        stub(&mut e, WORD_AT_0X3C, 2);
        stub(&mut e, WORD_AT_0X40, 0);
        let this = new_vats(&mut e);
        e.set(this, Vats::pCamera, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        fn_009c8b90(&mut e, this, 0x55);
        assert!(calls(&e, CAMERA_SET_VIEW_TARGET).is_empty());
        assert!(calls(&e, CAMERA_SET_SECOND_TARGET).is_empty());
        assert_eq!(e.get(this, Vats::pFirstProjectile).addr(), 0x55);
        let no_camera = new_vats(&mut e);
        fn_009c8b90(&mut e, no_camera, 0x66);
        assert_eq!(e.get(no_camera, Vats::pFirstProjectile).addr(), 0x66);
    }

    #[test]
    fn second_reference_setter_advances_cameras_while_the_stage_is_above_the_count() {
        let mut e = engine();
        stub(&mut e, WORD_AT_0X38, 1);
        stub(&mut e, CAMERA_IS_INITIALIZED, 0);
        stub(&mut e, FREE_CAMERA, 0);
        let this = new_vats(&mut e);
        e.set(this, Vats::pCamera, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        fn_009c8c40(&mut e, this, 0x88);
        assert_eq!(e.get(this, Vats::pLastProjectile).addr(), 0x88);
        // The empty list gives no current command; with a second reference and no first one the
        // stage decision sets stage 2, which is above the shot count 1. The minimum timer is
        // exactly 0.0, so the shots advance: no next shot exists and the uninitialized one is freed.
        assert_eq!(e.global::<i32>(CAMERA_STAGE), 2);
        assert_eq!(calls(&e, FREE_CAMERA), vec![vec![0x1234]]);
        assert!(e.get(this, Vats::pCamera).is_null());
    }

    #[test]
    fn second_reference_setter_without_a_camera_only_stores() {
        let mut e = engine();
        let this = new_vats(&mut e);
        e.call_log = Some(vec![]);
        fn_009c8c40(&mut e, this, 0x88);
        assert_eq!(e.get(this, Vats::pLastProjectile).addr(), 0x88);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    // ---- update factors -----------------------------------------------------------------------

    #[test]
    fn player_update_mult_uses_the_setting_without_a_camera() {
        let mut e = engine();
        set_setting(&mut e, PLAYER_MULT_SETTING, 2.0);
        stub_float(&mut e, PLAYER_TIME_FACTOR, 6.0);
        stub_float(&mut e, TIMER_DIVISOR, 3.0);
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        assert_eq!(vats_get_player_update_mult(&mut e, this), 4.0);
        e.set(this, Vats::eMode, 2);
        assert_eq!(vats_get_player_update_mult(&mut e, this), 2.0);
    }

    #[test]
    fn player_update_mult_uses_the_camera_value_when_positive() {
        let mut e = engine();
        set_setting(&mut e, PLAYER_MULT_SETTING, 2.0);
        stub_float(&mut e, PLAYER_TIME_FACTOR, 6.0);
        stub_float(&mut e, TIMER_DIVISOR, 3.0);
        stub(&mut e, CAMERA_IS_INITIALIZED, 1);
        stub_float(&mut e, FLOAT_AT_0X48, 0.5);
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        e.set(this, Vats::pCamera, Ptr::new(0x1234));
        assert_eq!(vats_get_player_update_mult(&mut e, this), 1.0);
        stub_float(&mut e, FLOAT_AT_0X48, 0.0);
        assert_eq!(vats_get_player_update_mult(&mut e, this), 4.0);
        stub_float(&mut e, FLOAT_AT_0X48, f64::NAN);
        assert_eq!(vats_get_player_update_mult(&mut e, this), 4.0);
        stub(&mut e, CAMERA_IS_INITIALIZED, 0);
        assert_eq!(vats_get_player_update_mult(&mut e, this), 2.0);
    }

    #[test]
    fn target_update_mult_uses_its_own_camera_float_and_setting() {
        let mut e = engine();
        set_setting(&mut e, TARGET_MULT_SETTING, 3.0);
        stub_float(&mut e, PLAYER_TIME_FACTOR, 4.0);
        stub_float(&mut e, TIMER_DIVISOR, 2.0);
        stub(&mut e, CAMERA_IS_INITIALIZED, 1);
        stub_float(&mut e, FLOAT_AT_0X4C, 0.25);
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        e.set(this, Vats::pCamera, Ptr::new(0x1234));
        assert_eq!(vats_get_target_update_mult(&mut e, this), 0.5);
        e.set(this, Vats::pCamera, Ptr::NULL);
        assert_eq!(vats_get_target_update_mult(&mut e, this), 6.0);
    }

    // ---- EvaluateCommand and the pick-time check ----------------------------------------------

    #[test]
    fn pick_time_float_follows_the_flag_and_quiets_signalling_nans() {
        let mut e = engine();
        e.mem.set_f32(PICK_TIME_FLOAT_OFF, 1.0);
        e.mem.set_f32(PICK_TIME_FLOAT_ON, 2.0);
        assert_eq!(fn_009c8ef0(&mut e), 1.0);
        e.set_global(PLAYING_BACK_FLAG, 1u8);
        assert_eq!(fn_009c8ef0(&mut e), 2.0);
        e.mem.set_u32(PICK_TIME_FLOAT_ON, 0x7f80_0001);
        assert_eq!(fn_009c8ef0(&mut e).to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn pick_time_check_compares_the_limit_with_the_returned_float() {
        let mut e = engine();
        stub(&mut e, INTERFACE_ENABLED, 0);
        e.mem.set_f32(PICK_TIME_FLOAT_OFF, 1.0);
        e.mem.set_f32(PICK_TIME_LIMIT, 2.0);
        assert!(fn_009c8ea0(&mut e));
        e.mem.set_f32(PICK_TIME_LIMIT, 1.0);
        assert!(!fn_009c8ea0(&mut e));
        e.mem.set_f32(PICK_TIME_LIMIT, f32::NAN);
        assert!(!fn_009c8ea0(&mut e));
        e.mem.set_f32(PICK_TIME_LIMIT, 2.0);
        stub(&mut e, INTERFACE_ENABLED, 1);
        assert!(!fn_009c8ea0(&mut e));
    }

    fn evaluate_rig(e: &mut Engine) -> (Ptr<Vats>, Ptr<VatsCommand>, u32) {
        stub(e, IS_ATTACK_ACTION, 1);
        stub(e, INTERFACE_ENABLED, 0);
        stub_all(e, &[HIT_DATA_CLEAR, HIT_DATA_INITIALIZE]);
        e.mem.set_f32(PICK_TIME_FLOAT_OFF, 1.0);
        let this = new_vats(e);
        let command = e.new_object::<VatsCommand>();
        let hit_data = e.mem.alloc(0x40);
        e.set(command, VatsCommand::eAction, 3);
        e.set(command, VatsCommand::bSuccess, true);
        e.set(command, VatsCommand::pTarget, Ptr::new(0x6000));
        e.set(command, VatsCommand::pHitData, Ptr::new(hit_data));
        e.set_global(CURRENT_ACTOR, 0x5000u32);
        (this, command, hit_data)
    }

    #[test]
    fn evaluate_command_keeps_success_when_the_pick_time_check_passes() {
        let mut e = engine();
        let (this, command, hit_data) = evaluate_rig(&mut e);
        e.mem.set_f32(PICK_TIME_LIMIT, 2.0);
        e.call_log = Some(vec![]);
        vats_evaluate_command(&mut e, this, command);
        assert!(e.get(command, VatsCommand::bSuccess));
        assert_eq!(calls(&e, HIT_DATA_CLEAR), vec![vec![hit_data]]);
        assert_eq!(
            calls(&e, HIT_DATA_INITIALIZE),
            vec![vec![hit_data, command.addr()]]
        );
        assert!(calls(&e, ACTOR_LINE_OF_SIGHT).is_empty());
    }

    #[test]
    fn evaluate_command_drops_success_without_line_of_sight() {
        let mut e = engine();
        let (this, command, _) = evaluate_rig(&mut e);
        e.mem.set_f32(PICK_TIME_LIMIT, 0.5);
        stub(&mut e, ACTOR_LINE_OF_SIGHT, 0);
        e.call_log = Some(vec![]);
        vats_evaluate_command(&mut e, this, command);
        assert!(!e.get(command, VatsCommand::bSuccess));
        assert_eq!(
            calls(&e, ACTOR_LINE_OF_SIGHT),
            vec![vec![0x5000, 1, 0x6000, 1, 0, 0]]
        );
    }

    #[test]
    fn evaluate_command_with_line_of_sight_keeps_success_and_skips_missing_hit_data() {
        let mut e = engine();
        let (this, command, _) = evaluate_rig(&mut e);
        e.mem.set_f32(PICK_TIME_LIMIT, 0.5);
        stub(&mut e, ACTOR_LINE_OF_SIGHT, 1);
        e.set(command, VatsCommand::pHitData, Ptr::NULL);
        e.call_log = Some(vec![]);
        vats_evaluate_command(&mut e, this, command);
        assert!(e.get(command, VatsCommand::bSuccess));
        assert!(calls(&e, HIT_DATA_CLEAR).is_empty());
        assert_eq!(
            calls(&e, HIT_DATA_INITIALIZE),
            vec![vec![0, command.addr()]]
        );
    }

    #[test]
    fn evaluate_command_ignores_commands_that_are_not_attacks() {
        let mut e = engine();
        let (this, command, _) = evaluate_rig(&mut e);
        stub(&mut e, IS_ATTACK_ACTION, 0);
        e.call_log = Some(vec![]);
        vats_evaluate_command(&mut e, this, command);
        vats_evaluate_command(&mut e, this, Ptr::NULL);
        assert!(calls(&e, HIT_DATA_INITIALIZE).is_empty());
    }

    // ---- GetLineOfSight -----------------------------------------------------------------------

    /// Everything the ray cast calls; returns (this, command, target actor, hit list holder).
    fn line_of_sight_rig(
        e: &mut Engine,
        hit_reference: Option<u32>,
    ) -> (Ptr<Vats>, Ptr<VatsCommand>, u32) {
        let from = vec_block(e, 1.0, 2.0, 3.0);
        let to = vec_block(e, 4.0, 5.0, 6.0);
        let current = object_with_slots(e, &[(0x1f4, from)]);
        e.set_global(CURRENT_ACTOR, current);
        let ray_base = object_with_slots(e, &[(0xc8, 0)]);
        let target = object_with_slots(e, &[(0x100, 1), (0x1f4, to)]);
        stub(e, WORD_AT_0X40, 0x50);
        stub(e, ACTOR_CONTROLLER_BASE, ray_base);
        stub_all(
            e,
            &[
                SET_MULTI_LAYER_COLLISION,
                RAY_QUERY_CONSTRUCTOR,
                FILTER_INFO_CONSTRUCTOR,
                FILTER_INFO_SET_LAYER,
                FILTER_INFO_SET_GROUP,
                RAY_QUERY_SET_FILTER,
                RAY_QUERY_SET_FROM,
                RAY_QUERY_SET_TO,
                RAY_COLLECTOR_CONSTRUCTOR,
                RAY_QUERY_SET_COLLECTOR,
                RAY_COLLECTOR_DESTRUCTOR,
                GET_CHAR_CONTROLLER,
                WORLD_COLLISION_GROUP,
            ],
        );
        let hits = e.mem.alloc(8);
        e.mem.set_u32(hits + 4, u32::from(hit_reference.is_some()));
        stub(e, RAY_HITS_FIRST, 1);
        stub(e, RAY_HITS_SECOND, hits);
        stub(e, RAY_HIT_AT, 0x70);
        e.register(HIT_RECORD_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x50, 0x71);
            eax(a[0])
        });
        stub(e, GET_AV_OBJECT_FOR_COLLIDABLE, 0x72);
        stub(e, FIND_REFERENCE_FOR_3D, hit_reference.unwrap_or(0));
        let this = new_vats(e);
        let command = e.new_object::<VatsCommand>();
        e.set(command, VatsCommand::pTarget, Ptr::new(target));
        (this, command, target)
    }

    #[test]
    fn line_of_sight_is_clear_without_hits() {
        let mut e = engine();
        let (this, command, _) = line_of_sight_rig(&mut e, None);
        e.call_log = Some(vec![]);
        assert!(vats_get_line_of_sight(&mut e, this, command));
        assert_eq!(
            calls(&e, SET_MULTI_LAYER_COLLISION),
            vec![vec![0x27, 0x26206, 0]]
        );
        let from = calls(&e, RAY_QUERY_SET_FROM)[0][1];
        assert_eq!(e.mem.f32(from + 8), 63.0);
        let to = calls(&e, RAY_QUERY_SET_TO)[0][1];
        assert_eq!(e.mem.f32(to + 8), 66.0);
        assert_eq!(calls(&e, RAY_COLLECTOR_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn line_of_sight_is_blocked_by_another_reference() {
        let mut e = engine();
        let (this, command, _) = line_of_sight_rig(&mut e, Some(0x999));
        assert!(!vats_get_line_of_sight(&mut e, this, command));
    }

    #[test]
    fn line_of_sight_ignores_hits_on_the_target_itself() {
        let mut e = engine();
        let (this, command, target) = line_of_sight_rig(&mut e, Some(0));
        stub(&mut e, FIND_REFERENCE_FOR_3D, target);
        assert!(vats_get_line_of_sight(&mut e, this, command));
        let current = e.global::<u32>(CURRENT_ACTOR);
        stub(&mut e, FIND_REFERENCE_FOR_3D, current);
        assert!(vats_get_line_of_sight(&mut e, this, command));
    }

    #[test]
    fn line_of_sight_needs_an_actor_with_a_process() {
        let mut e = engine();
        let (this, command, target) = line_of_sight_rig(&mut e, None);
        stub(&mut e, WORD_AT_0X40, 0);
        assert!(!vats_get_line_of_sight(&mut e, this, command));
        let not_an_actor = object_with_slots(&mut e, &[(0x100, 0)]);
        e.set(command, VatsCommand::pTarget, Ptr::new(not_an_actor));
        assert!(!vats_get_line_of_sight(&mut e, this, command));
        let _ = target;
    }

    #[test]
    fn line_of_sight_uses_the_controller_group_for_characters() {
        let mut e = engine();
        let (this, command, _) = line_of_sight_rig(&mut e, None);
        stub(&mut e, GET_CHAR_CONTROLLER, 0x30);
        stub(&mut e, CONTROLLER_COLLISION_GROUP, 0x44);
        e.call_log = Some(vec![]);
        assert!(vats_get_line_of_sight(&mut e, this, command));
        assert_eq!(calls(&e, CONTROLLER_COLLISION_GROUP), vec![vec![0x30]]);
        assert_eq!(calls(&e, FILTER_INFO_SET_GROUP)[0][1], 0x44);
    }

    // ---- UpdateMeleeAttack --------------------------------------------------------------------

    fn melee_rig(e: &mut Engine) -> (Ptr<Vats>, Ptr<VatsCommand>, u32, u32) {
        let position = vec_block(e, 1.0, 2.0, 3.0);
        let actor = object_with_slots(
            e,
            &[(0x1f4, position), (0x2a8, 0), (0x2c0, 0), (0x1e4, 0x42)],
        );
        let target = object_with_slots(e, &[(0x1f4, position)]);
        let player = object_with_slots(e, &[(0x1f4, position)]);
        e.set_global(CURRENT_ACTOR, actor);
        e.set_global(PLAYER_POINTER, player);
        stub_all(
            e,
            &[
                SET_LOCATION_ON_REFERENCE,
                UPDATE_SCENE_GRAPH,
                ACTOR_SET_ALPHA_MULT,
                ACTOR_UPDATE_ALPHA,
                STRANGER_APPEARS_SOUND,
                GET_IDLE_TO_PLAY,
            ],
        );
        stub(e, FIND_SAFE_PATH_POINT, 1);
        let this = new_vats(e);
        let command = e.new_object::<VatsCommand>();
        e.set(command, VatsCommand::eAction, 5);
        e.set(command, VatsCommand::cFireShots, 1);
        e.set(command, VatsCommand::pTarget, Ptr::new(target));
        (this, command, actor, target)
    }

    #[test]
    fn melee_attack_does_nothing_without_fire_shots() {
        let mut e = engine();
        let (this, command, _, _) = melee_rig(&mut e);
        e.set(command, VatsCommand::cFireShots, 0);
        e.call_log = Some(vec![]);
        vats_update_melee_attack(&mut e, this, command);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn melee_attack_moves_a_stranger_command_to_its_reach() {
        let mut e = engine();
        let (this, command, actor, target) = melee_rig(&mut e);
        e.set_global(STRANGER_ACTOR, actor);
        e.set(command, VatsCommand::bStranger, true);
        set_setting(&mut e, STRANGER_REACH_SETTING, 5.0);
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        stub_float(&mut e, ACTOR_GET_ALPHA_MULT, 0.5);
        e.call_log = Some(vec![]);
        vats_update_melee_attack(&mut e, this, command);
        let path = calls(&e, FIND_SAFE_PATH_POINT);
        assert_eq!(path.len(), 1);
        assert_eq!(&path[0][..3], &[actor, target, float_bits(5.0)]);
        assert_eq!(
            calls(&e, ACTOR_SET_ALPHA_MULT),
            vec![vec![actor, float_bits(1.0)]]
        );
        assert_eq!(calls(&e, STRANGER_APPEARS_SOUND).len(), 1);
        assert_eq!(calls(&e, UPDATE_SCENE_GRAPH), vec![vec![0x42]]);
        assert_eq!(calls(&e, SET_LOCATION_ON_REFERENCE).len(), 1);
        assert_eq!(calls(&e, GET_IDLE_TO_PLAY).len(), 1);
    }

    #[test]
    fn melee_attack_scales_the_weapon_reach_by_the_settings() {
        let mut e = engine();
        let (this, command, actor, _) = melee_rig(&mut e);
        e.set(command, VatsCommand::eAction, 1);
        set_setting(&mut e, REACH_FACTOR_NO_WEAPON_SETTING, 0.5);
        set_setting(&mut e, REACH_FACTOR_WEAPON_TEST_SETTING, 3.0);
        stub(&mut e, ACTOR_GET_CURRENT_WEAPON, 0);
        stub_float(&mut e, GET_WEAPON_REACH, 100.0);
        e.call_log = Some(vec![]);
        vats_update_melee_attack(&mut e, this, command);
        assert_eq!(calls(&e, FIND_SAFE_PATH_POINT)[0][2], float_bits(50.0));
        stub(&mut e, ACTOR_GET_CURRENT_WEAPON, 0x9000);
        stub(&mut e, WEAPON_TEST_006450C0, 1);
        e.call_log = Some(vec![]);
        vats_update_melee_attack(&mut e, this, command);
        assert_eq!(calls(&e, FIND_SAFE_PATH_POINT)[0][2], float_bits(300.0));
        assert_eq!(calls(&e, GET_WEAPON_REACH), vec![vec![actor, 0x9000]]);
        stub(&mut e, WEAPON_TEST_006450C0, 0);
        e.call_log = Some(vec![]);
        vats_update_melee_attack(&mut e, this, command);
        assert_eq!(calls(&e, FIND_SAFE_PATH_POINT)[0][2], float_bits(100.0));
    }

    #[test]
    fn melee_attack_falls_back_to_the_target_position_without_a_path() {
        let mut e = engine();
        let (this, command, actor, target) = melee_rig(&mut e);
        e.set(command, VatsCommand::eAction, 1);
        stub(&mut e, ACTOR_GET_CURRENT_WEAPON, 0);
        stub_float(&mut e, GET_WEAPON_REACH, 10.0);
        set_setting(&mut e, REACH_FACTOR_NO_WEAPON_SETTING, 1.0);
        stub(&mut e, FIND_SAFE_PATH_POINT, 0);
        e.call_log = Some(vec![]);
        vats_update_melee_attack(&mut e, this, command);
        let move_to = slot_target(&e, actor, 0x2a8);
        let face = slot_target(&e, actor, 0x2c0);
        assert_eq!(calls(&e, move_to).len(), 1);
        assert_eq!(calls(&e, face), vec![vec![actor, target]]);
    }

    #[test]
    fn melee_attack_checks_the_line_of_sight_of_the_player() {
        let mut e = engine();
        // The ray cast doubles come first; the melee rig then sets the actors.
        let (_, _, ray_target) = line_of_sight_rig(&mut e, Some(0x999));
        let (this, command, actor, _) = melee_rig(&mut e);
        e.set_global(PLAYER_POINTER, actor);
        e.set(command, VatsCommand::eAction, 1);
        e.set(command, VatsCommand::pTarget, Ptr::new(ray_target));
        stub(&mut e, ACTOR_GET_CURRENT_WEAPON, 0);
        stub_float(&mut e, GET_WEAPON_REACH, 10.0);
        set_setting(&mut e, REACH_FACTOR_NO_WEAPON_SETTING, 1.0);
        e.call_log = Some(vec![]);
        vats_update_melee_attack(&mut e, this, command);
        let move_to = slot_target(&e, actor, 0x2a8);
        // First the path point, then (the other reference blocks the sight) the target position.
        assert_eq!(calls(&e, move_to).len(), 2);
        assert_eq!(calls(&e, RAY_COLLECTOR_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn melee_attack_keeps_a_valid_idle_and_drops_a_bad_one() {
        let mut e = engine();
        let (this, command, _, _) = melee_rig(&mut e);
        // Not applicable (action 5, player): only the idle part runs.
        stub(&mut e, GET_IDLE_TO_PLAY, 0x8000);
        stub(&mut e, IDLE_GET_ANIM_GROUP_SECTION, 4);
        vats_update_melee_attack(&mut e, this, command);
        assert_eq!(e.get(this, Vats::pMeleeAttack).addr(), 0x8000);
        let idle = object_with_slots(&mut e, &[(0x130, 0x1234)]);
        stub(&mut e, GET_IDLE_TO_PLAY, idle);
        stub(&mut e, IDLE_GET_ANIM_GROUP_SECTION, 3);
        stub(&mut e, PRINT_MESSAGE, 0);
        e.call_log = Some(vec![]);
        vats_update_melee_attack(&mut e, this, command);
        assert!(e.get(this, Vats::pMeleeAttack).is_null());
        assert_eq!(
            calls(&e, PRINT_MESSAGE),
            vec![vec![BAD_MELEE_ATTACK_FORMAT, 0x1234]]
        );
        stub(&mut e, GET_IDLE_TO_PLAY, 0);
        vats_update_melee_attack(&mut e, this, command);
        assert!(e.get(this, Vats::pMeleeAttack).is_null());
    }

    // ---- IsMeleeCommand -----------------------------------------------------------------------

    #[test]
    fn melee_commands_are_the_listed_actions() {
        let mut e = engine();
        let this = new_vats(&mut e);
        let command = e.new_object::<VatsCommand>();
        for action in 0..0x20u32 {
            e.set(command, VatsCommand::eAction, action);
            let expected = matches!(action, 0 | 1 | 2 | 0x10 | 0x11 | 0x14 | 0x15);
            assert_eq!(vats_is_melee_command(&mut e, this, command), expected);
        }
        e.set(command, VatsCommand::eAction, u32::MAX);
        assert!(!vats_is_melee_command(&mut e, this, command));
    }

    #[test]
    fn melee_command_test_falls_back_to_the_first_command() {
        let mut e = engine();
        let this = new_vats(&mut e);
        assert!(!vats_is_melee_command(&mut e, this, Ptr::NULL));
        let command = make_command(&mut e, 0x14);
        fill_list(&mut e, this.addr(), &[command]);
        assert!(vats_is_melee_command(&mut e, this, Ptr::NULL));
    }

    // ---- UseNextCamera ------------------------------------------------------------------------

    #[test]
    fn use_next_camera_moves_to_the_following_shot() {
        let mut e = engine();
        stub(&mut e, FREE_CAMERA, 0);
        stub(&mut e, PLAYER_SET_FIRST_PERSON, 0);
        stub_float(&mut e, FLOAT_AT_0X58, 2.0);
        let this = new_vats(&mut e);
        let first = e.mem.alloc(0x100);
        let second = e.mem.alloc(0x100);
        e.mem.set_u32(second + 0x44, 8);
        let list = e.mem.alloc(8);
        fill_list(&mut e, list, &[first, second]);
        e.set(this, Vats::pCameraList, Ptr::new(list));
        e.set(this, Vats::pCamera, Ptr::new(first));
        e.set(this, Vats::fCameraMinTimer, 1.0f32);
        e.set_global(PLAYER_POINTER, 0x5000u32);
        e.call_log = Some(vec![]);
        assert!(vats_use_next_camera(&mut e, this));
        assert_eq!(calls(&e, FREE_CAMERA), vec![vec![first]]);
        assert_eq!(e.get(this, Vats::pCamera).addr(), second);
        assert_eq!(e.get(this, Vats::fCameraMinTimer), 2.0);
        assert_eq!(calls(&e, PLAYER_SET_FIRST_PERSON), vec![vec![0x5000, 1]]);
    }

    #[test]
    fn use_next_camera_without_the_first_person_flag_sets_the_pending_byte() {
        let mut e = engine();
        stub(&mut e, FREE_CAMERA, 0);
        stub_float(&mut e, FLOAT_AT_0X58, 0.5);
        let this = new_vats(&mut e);
        let first = e.mem.alloc(0x100);
        let second = e.mem.alloc(0x100);
        let list = e.mem.alloc(8);
        fill_list(&mut e, list, &[first, second]);
        e.set(this, Vats::pCameraList, Ptr::new(list));
        e.set(this, Vats::pCamera, Ptr::new(first));
        e.set(this, Vats::fCameraMinTimer, 1.0f32);
        assert!(vats_use_next_camera(&mut e, this));
        assert_eq!(e.get(this, Vats::fCameraMinTimer), 1.0);
        assert_eq!(e.global::<u8>(FIRST_PERSON_PENDING), 1);
    }

    #[test]
    fn use_next_camera_at_the_end_frees_an_uninitialized_shot() {
        let mut e = engine();
        stub(&mut e, FREE_CAMERA, 0);
        stub(&mut e, CAMERA_IS_INITIALIZED, 0);
        let this = new_vats(&mut e);
        let only = e.mem.alloc(0x100);
        let list = e.mem.alloc(8);
        fill_list(&mut e, list, &[only]);
        e.set(this, Vats::pCameraList, Ptr::new(list));
        e.set(this, Vats::pCamera, Ptr::new(only));
        e.call_log = Some(vec![]);
        assert!(!vats_use_next_camera(&mut e, this));
        assert_eq!(calls(&e, FREE_CAMERA), vec![vec![only]]);
        assert!(e.get(this, Vats::pCamera).is_null());
        // An initialized shot stays.
        e.set(this, Vats::pCamera, Ptr::new(only));
        stub(&mut e, CAMERA_IS_INITIALIZED, 1);
        assert!(!vats_use_next_camera(&mut e, this));
        assert_eq!(e.get(this, Vats::pCamera).addr(), only);
    }

    // ---- SetVATSLight, the light placement ----------------------------------------------------

    #[test]
    fn set_light_releases_the_old_light_and_adds_the_new_one() {
        let mut e = engine();
        let scene_node = object_with_slots(&mut e, &[(0xdc, 0)]);
        stub(&mut e, SCENE_NODE_BY_INDEX, scene_node);
        stub(&mut e, SMART_POINTER_DIFFERS, 1);
        stub(&mut e, SMART_POINTER_ASSIGN, 0);
        stub(&mut e, ADD_LIGHT, 0x8000);
        let this = new_vats(&mut e);
        e.call_log = Some(vec![]);
        vats_set_vats_light(&mut e, this, 0x7000, 1);
        let slot = this.addr() + 0x34;
        assert_eq!(calls(&e, SMART_POINTER_DIFFERS), vec![vec![slot, 0]]);
        assert_eq!(
            calls(&e, SMART_POINTER_ASSIGN),
            vec![vec![slot, 0], vec![slot, 0x8000]]
        );
        assert_eq!(calls(&e, ADD_LIGHT), vec![vec![scene_node, 0x7000, 1]]);
        let notify = slot_target(&e, scene_node, 0xdc);
        assert_eq!(calls(&e, notify), vec![vec![scene_node, 0x7000, 1]]);
    }

    #[test]
    fn set_light_without_a_light_only_releases() {
        let mut e = engine();
        stub(&mut e, SMART_POINTER_DIFFERS, 0);
        stub(&mut e, SMART_POINTER_ASSIGN, 0);
        let this = new_vats(&mut e);
        e.call_log = Some(vec![]);
        vats_set_vats_light(&mut e, this, 0, 1);
        assert!(calls(&e, SMART_POINTER_ASSIGN).is_empty());
        vats_set_vats_light(&mut e, this, 0, 0);
        assert_eq!(calls(&e, SMART_POINTER_DIFFERS).len(), 1);
    }

    fn light_rig(e: &mut Engine) -> (Ptr<Vats>, u32) {
        let position = vec_block(e, 1.0, 2.0, 3.0);
        let actor = object_with_slots(e, &[(0x100, 1), (0x1d0, 0x6000), (0x1f4, position)]);
        let current = object_with_slots(e, &[(0x1f4, position)]);
        e.set_global(CURRENT_ACTOR, current);
        let bound = vec_block(e, 0.0, 0.0, 0.0);
        let result = vec_block(e, 0.0, 0.0, 0.0);
        stub(e, ACTOR_GET_PROCESS, 1);
        stub(e, GET_WORLD_BOUND, bound);
        e.register_double(POINT3_SUBTRACT, move |_, _| eax(result));
        e.register(POINT3_ADD, |_, a| eax(a[1]));
        e.register(MATRIX_TIMES_VECTOR, |_, a| eax(a[0]));
        e.register(POINT3_CONSTRUCTOR, |_, a| eax(a[0]));
        stub_float(e, SETTING_FLOAT_VALUE, 1.0);
        stub_float(e, ACTOR_GET_HEIGHT, 1.7);
        stub_float(e, LIGHT_TRACE, 0.0);
        stub_float(e, REMAP_FLOAT, 0.5);
        stub_float(e, FLOAT_MAX, 0.5);
        stub_float(e, FLOAT_MIN, 0.5);
        stub_float(e, POINT3_LENGTH, 2.0);
        stub(e, LIGHT_HANDLE, 0x4000);
        stub(e, SCENE_NODE_BY_INDEX, 0x5000);
        stub_all(
            e,
            &[
                MATRIX_FROM_ANGLE,
                VECTOR_NORMALIZE,
                POINT3_SCALE,
                LIGHT_SET_POSITION,
                LIGHT_SET_DIRECTION,
                LIGHT_SET_COLOUR,
                LIGHT_SET_DIMMER,
            ],
        );
        let this = new_vats(e);
        e.set(this, Vats::spVATSLight, Ptr::new(0x3000));
        (this, actor)
    }

    #[test]
    fn light_placement_sets_the_light_from_the_actor_bound() {
        let mut e = engine();
        let (this, actor) = light_rig(&mut e);
        e.call_log = Some(vec![]);
        fn_009c9ae0(&mut e, this, actor);
        let position = calls(&e, LIGHT_SET_POSITION);
        assert_eq!(position.len(), 1);
        assert_eq!(position[0][0], 0x4000);
        // The radius vector is built with twice the bound length (2 * 2.0) in x.
        let radius: Vec<_> = calls(&e, POINT3_CONSTRUCTOR)
            .into_iter()
            .filter(|words| words[1] == float_bits(4.0))
            .collect();
        assert_eq!(radius.len(), 1);
        assert_eq!(calls(&e, LIGHT_SET_DIRECTION)[0][0], 0x4000);
        assert_eq!(calls(&e, LIGHT_SET_COLOUR)[0][0], 0x4000);
        assert_eq!(calls(&e, LIGHT_SET_DIMMER), vec![vec![0x4000, 0]]);
        // The trace is called on the scene node with the ten words after it.
        let trace = calls(&e, LIGHT_TRACE);
        assert_eq!(trace.len(), 1);
        assert_eq!(trace[0].len(), 11);
        assert_eq!(trace[0][0], 0x5000);
        assert_eq!(trace[0][10], 0x4000);
    }

    #[test]
    fn light_placement_needs_an_actor_a_3d_object_a_light_and_a_process() {
        let mut e = engine();
        let (this, actor) = light_rig(&mut e);
        let not_an_actor = object_with_slots(&mut e, &[(0x100, 0)]);
        e.call_log = Some(vec![]);
        fn_009c9ae0(&mut e, this, not_an_actor);
        let no_3d = object_with_slots(&mut e, &[(0x100, 1), (0x1d0, 0)]);
        fn_009c9ae0(&mut e, this, no_3d);
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        fn_009c9ae0(&mut e, this, actor);
        stub(&mut e, ACTOR_GET_PROCESS, 1);
        e.set(this, Vats::spVATSLight, Ptr::NULL);
        fn_009c9ae0(&mut e, this, actor);
        assert!(calls(&e, LIGHT_SET_POSITION).is_empty());
    }

    // ---- removing the commands of a reference -------------------------------------------------

    #[test]
    fn commands_of_a_reference_are_removed_from_the_list() {
        let mut e = engine();
        let this = new_vats(&mut e);
        let first = make_command(&mut e, 3);
        e.mem.set_u32(first + 0xc, 0xa);
        let second = make_command(&mut e, 3);
        e.mem.set_u32(second + 0xc, 0xb);
        let third = make_command(&mut e, 3);
        e.mem.set_u32(third + 0xc, 0xa);
        fill_list(&mut e, this.addr(), &[first, second, third]);
        e.set_global(TARGET_REFERENCE, 0xau32);
        e.call_log = Some(vec![]);
        fn_009c9e90(&mut e, this, 0xa);
        assert_eq!(list_items(&e, this.addr()), vec![second]);
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![first], vec![third]]);
        assert_eq!(e.global::<u32>(TARGET_REFERENCE), 0);
    }

    #[test]
    fn commands_of_other_references_stay() {
        let mut e = engine();
        let this = new_vats(&mut e);
        let command = make_command(&mut e, 3);
        e.mem.set_u32(command + 0xc, 0xb);
        fill_list(&mut e, this.addr(), &[command]);
        e.set_global(TARGET_REFERENCE, 0xbu32);
        fn_009c9e90(&mut e, this, 0xa);
        assert_eq!(list_items(&e, this.addr()), vec![command]);
        assert_eq!(e.global::<u32>(TARGET_REFERENCE), 0xb);
    }

    // ---- the aim position ---------------------------------------------------------------------

    #[test]
    fn aim_position_without_a_target_is_the_model_bound_centre() {
        let mut e = engine();
        let centre = vec_block(&mut e, 1.0, 2.0, 3.0);
        stub(&mut e, ACTOR_MODEL_OBJECT, 0x10);
        stub(&mut e, GET_WORLD_BOUND, centre);
        let this = new_vats(&mut e);
        let out = e.mem.alloc(12);
        let result = fn_009c9fe0(&mut e, this, Ptr::new(out), 2.0);
        assert_eq!(result.addr(), out);
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(e.mem.f32(out + 4), 2.0);
        assert_eq!(e.mem.f32(out + 8), 3.0);
    }

    #[test]
    fn aim_position_moves_towards_the_target_by_the_distance() {
        let mut e = engine();
        let centre = vec_block(&mut e, 1.0, 2.0, 3.0);
        stub(&mut e, ACTOR_MODEL_OBJECT, 0x10);
        stub(&mut e, GET_WORLD_BOUND, centre);
        e.mem.set_f32(OFFSET_FLOAT, 4.0);
        let target_position = vec_block(&mut e, 5.0, 2.0, 3.0);
        let target = object_with_slots(&mut e, &[(0x1d0, 0), (0x1f4, target_position)]);
        let this = new_vats(&mut e);
        let command = make_command(&mut e, 3);
        e.mem.set_u32(command + 0xc, target);
        fill_list(&mut e, this.addr(), &[command]);
        // The vector helpers: subtract writes the difference, unitize returns the length,
        // times/divided write the scaled vector, add-assign adds into this.
        e.register(POINT3_SUBTRACT, |e, a| {
            for axis in 0..3 {
                let value = e.mem.f32(a[0] + 4 * axis) - e.mem.f32(a[2] + 4 * axis);
                e.mem.set_f32(a[1] + 4 * axis, value);
            }
            eax(a[1])
        });
        stub_float(&mut e, POINT3_UNITIZE, 4.0);
        e.register(POINT3_TIMES, |e, a| {
            let scale = f32::from_bits(a[2]);
            for axis in 0..3 {
                let value = e.mem.f32(a[0] + 4 * axis) * scale;
                e.mem.set_f32(a[1] + 4 * axis, value);
            }
            eax(a[1])
        });
        e.register(POINT3_TIMES_RECIPROCAL, |e, a| {
            let scale = f32::from_bits(a[2]);
            for axis in 0..3 {
                let value = e.mem.f32(a[0] + 4 * axis) / scale;
                e.mem.set_f32(a[1] + 4 * axis, value);
            }
            eax(a[1])
        });
        e.register(POINT3_ADD_ASSIGN, |e, a| {
            for axis in 0..3 {
                let value = e.mem.f32(a[0] + 4 * axis) + e.mem.f32(a[1] + 4 * axis);
                e.mem.set_f32(a[0] + 4 * axis, value);
            }
            eax(a[0])
        });
        let out = e.mem.alloc(12);
        fn_009c9fe0(&mut e, this, Ptr::new(out), 2.0);
        // direction (4, 0, 0) is unitized (stub), rebuilt times the length 4, times 2, divided by 4.
        assert_eq!(e.mem.f32(out), 1.0 + 4.0 * 4.0 * 2.0 / 4.0);
        assert_eq!(e.mem.f32(out + 4), 2.0);
    }

    #[test]
    fn aim_position_uses_the_target_reference_global_and_its_position() {
        let mut e = engine();
        let centre = vec_block(&mut e, 0.0, 0.0, 0.0);
        stub(&mut e, ACTOR_MODEL_OBJECT, 0x10);
        stub(&mut e, GET_WORLD_BOUND, centre);
        let target_position = vec_block(&mut e, 0.0, 0.0, 0.0);
        let target = object_with_slots(&mut e, &[(0x1d0, 0), (0x1f4, target_position)]);
        e.set_global(TARGET_REFERENCE, target);
        let this = new_vats(&mut e);
        stub_all(
            &mut e,
            &[
                POINT3_SUBTRACT,
                POINT3_TIMES,
                POINT3_TIMES_RECIPROCAL,
                POINT3_ADD_ASSIGN,
            ],
        );
        stub_float(&mut e, POINT3_UNITIZE, 1.0);
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        fn_009c9fe0(&mut e, this, Ptr::new(out), 1.0);
        let move_target = slot_target(&e, target, 0x1f4);
        assert_eq!(calls(&e, move_target).len(), 1);
        assert_eq!(calls(&e, POINT3_ADD_ASSIGN).len(), 1);
    }

    // ---- starting playbacks -------------------------------------------------------------------

    #[test]
    fn start_playback_appends_action_fifteen_and_enters_mode_four() {
        let mut e = engine();
        mode_four_doubles(&mut e);
        stub(&mut e, HUD_SET_MENU_MODE, 0);
        e.register(ALLOCATE, |e, _| eax(e.mem.alloc(0x28)));
        e.register(COMMAND_CONSTRUCTOR, |_, a| eax(a[0]));
        e.register(LIST_APPEND, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e.set_global(PLAYER_POINTER, 0x5000u32);
        let this = new_vats(&mut e);
        e.call_log = Some(vec![]);
        fn_009ca1f0(&mut e, this);
        let command = e.mem.u32(this.addr());
        assert_eq!(e.mem.u32(command), 0xf);
        assert_eq!(e.mem.u8(command + 4), 1);
        assert_eq!(e.mem.u32(command + 0xc), 0x5000);
        assert_eq!(e.get(this, Vats::eMode), 4);
        assert_eq!(calls(&e, HUD_SET_MENU_MODE), vec![vec![4]]);
    }

    #[test]
    fn start_playback_with_a_target_prepares_the_hit_data() {
        let mut e = engine();
        mode_four_doubles(&mut e);
        e.register(ALLOCATE, |e, _| eax(e.mem.alloc(0x28)));
        e.register(COMMAND_CONSTRUCTOR, |e, a| {
            let hit_data = e.mem.alloc(0x40);
            e.mem.set_u32(a[0] + 0x14, hit_data);
            eax(a[0])
        });
        e.register(LIST_APPEND, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        let value_holder = object_with_slots(&mut e, &[(0x148, 0x4242)]);
        let player_process = object_with_slots(&mut e, &[(0x148, value_holder)]);
        stub(&mut e, ACTOR_GET_PROCESS, player_process);
        stub(&mut e, WORD_AT_0X08, 0x33);
        stub(&mut e, HIT_DATA_SET_COUNTER, 0);
        e.set_global(PLAYER_POINTER, 0x5000u32);
        let this = new_vats(&mut e);
        e.call_log = Some(vec![]);
        fn_009ca2c0(&mut e, this, 0x6000, -1);
        let command = e.mem.u32(this.addr());
        let hit_data = e.mem.u32(command + 0x14);
        assert_eq!(e.mem.u32(command), 0x12);
        assert_eq!(e.mem.u32(command + 0xc), 0x6000);
        assert_eq!(e.mem.i32(hit_data + 0x10), 0);
        assert_eq!(e.mem.u32(hit_data + 0x30), 0x33);
        assert_eq!(
            calls(&e, HIT_DATA_SET_COUNTER),
            vec![vec![hit_data, 0x10, 1]]
        );
        assert_eq!(e.global::<u32>(CURRENT_ACTOR), 0x5000);
        assert_eq!(e.get(this, Vats::eMode), 4);
    }

    #[test]
    fn start_playback_without_a_player_process_skips_the_hit_data_number() {
        let mut e = engine();
        mode_four_doubles(&mut e);
        e.register(ALLOCATE, |e, _| eax(e.mem.alloc(0x28)));
        e.register(COMMAND_CONSTRUCTOR, |e, a| {
            let hit_data = e.mem.alloc(0x40);
            e.mem.set_u32(a[0] + 0x14, hit_data);
            eax(a[0])
        });
        e.register(LIST_APPEND, |_, _| Ret::default());
        stub(&mut e, ACTOR_GET_PROCESS, 0);
        stub(&mut e, HIT_DATA_SET_COUNTER, 0);
        e.set_global(PLAYER_POINTER, 0x5000u32);
        let this = new_vats(&mut e);
        e.call_log = Some(vec![]);
        fn_009ca2c0(&mut e, this, 0x6000, 7);
        assert!(calls(&e, WORD_AT_0X08).is_empty());
        assert_eq!(calls(&e, HIT_DATA_SET_COUNTER).len(), 1);
    }

    // ---- SetMode ------------------------------------------------------------------------------

    #[test]
    fn set_mode_none_resets_the_playback_state() {
        let mut e = engine();
        mode_zero_doubles(&mut e);
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        let stranger = object_with_slots(&mut e, &[(0x10, 0)]);
        e.set_global(STRANGER_ACTOR, stranger);
        e.set_global(PLAYER_POINTER, 0x5000u32);
        e.set(this, Vats::pCamera, Ptr::new(0x6000));
        e.set(this, Vats::pUnEquipWeapon, Ptr::new(0x7000));
        e.set(this, Vats::pUnEquipExtra, Ptr::new(0x7100));
        e.set(this, Vats::pImagespaceMod, Ptr::new(0x7200));
        e.call_log = Some(vec![]);
        vats_set_mode(&mut e, this, 0, 0);
        assert_eq!(e.get(this, Vats::eMode), 0);
        assert_eq!(e.global::<u32>(STRANGER_ACTOR), 0);
        assert_eq!(e.global::<u32>(CURRENT_ACTOR), 0x5000);
        let destructor = slot_target(&e, stranger, 0x10);
        assert_eq!(calls(&e, destructor), vec![vec![stranger, 1]]);
        assert_eq!(calls(&e, FREE_CAMERA), vec![vec![0x6000]]);
        assert_eq!(calls(&e, IMAGESPACE_STOP), vec![vec![0x7200]]);
        assert_eq!(
            calls(&e, ACTOR_UNEQUIP_OBJECT),
            vec![vec![0x5000, 0x7000, 1, 0x7100, 0, 1, 1]]
        );
        assert!(e.get(this, Vats::pUnEquipWeapon).is_null());
        assert!(e.get(this, Vats::bFirstVatsAttack));
        assert_eq!(calls(&e, HUD_SET_MENU_MODE), vec![vec![1]]);
        assert_eq!(e.global::<u8>(MENU_MODE_CHANGED), 1);
        assert_eq!(e.global::<u8>(MODE_TWO_FLAG), 0);
        assert_eq!(
            calls(&e, SET_GLOBAL_TIME_MULTIPLIER),
            vec![vec![TIMER_OBJECT, float_bits(1.0), 1]]
        );
        assert_eq!(
            calls(&e, ACTOR_SET_IRON_SIGHTS),
            vec![vec![0x5000, 0, 0, 0]]
        );
    }

    #[test]
    fn set_mode_none_removes_the_light_from_the_scene() {
        let mut e = engine();
        let scene_node = object_with_slots(&mut e, &[(0xe8, 0)]);
        stub(&mut e, GET_VATS_LIGHT, 0x900);
        stub(&mut e, LIGHT_HANDLE, 0x901);
        stub(&mut e, SCENE_NODE_BY_INDEX, scene_node);
        stub(&mut e, REMOVE_LIGHT, 0);
        stub(&mut e, SMART_POINTER_DIFFERS, 1);
        stub(&mut e, SMART_POINTER_ASSIGN, 0);
        let this = new_vats(&mut e);
        e.call_log = Some(vec![]);
        // The mode is already 0 and not forced: only the light removal runs.
        vats_set_mode(&mut e, this, 0, 0);
        assert_eq!(calls(&e, REMOVE_LIGHT), vec![vec![scene_node, 0x901]]);
        let notify = slot_target(&e, scene_node, 0xe8);
        assert_eq!(calls(&e, notify), vec![vec![scene_node, 0x901]]);
        assert_eq!(
            calls(&e, SMART_POINTER_ASSIGN),
            vec![vec![VATS_SINGLETON + 0x34, 0]]
        );
    }

    #[test]
    fn set_mode_menu_creates_the_light_and_marks_the_interface() {
        let mut e = engine();
        let scene_node = object_with_slots(&mut e, &[(0xdc, 0)]);
        stub(&mut e, INTERFACE_OBJECT_GETTER, 0x55);
        stub(&mut e, INTERFACE_MARK_DIRTY, 0);
        stub(&mut e, GET_VATS_LIGHT, 0);
        stub(&mut e, NI_ALLOCATE, 0x7000);
        stub(&mut e, LIGHT_CONSTRUCTOR, 0x7001);
        stub(&mut e, SCENE_NODE_BY_INDEX, scene_node);
        stub(&mut e, SMART_POINTER_DIFFERS, 0);
        stub(&mut e, SMART_POINTER_ASSIGN, 0);
        stub(&mut e, ADD_LIGHT, 0x8000);
        let this = new_vats(&mut e);
        e.call_log = Some(vec![]);
        vats_set_mode(&mut e, this, 2, 0);
        assert_eq!(e.get(this, Vats::eMode), 2);
        assert_eq!(e.global::<u8>(MODE_TWO_FLAG), 1);
        assert_eq!(calls(&e, INTERFACE_MARK_DIRTY), vec![vec![0x55]]);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0xfc]]);
        assert_eq!(calls(&e, LIGHT_CONSTRUCTOR), vec![vec![0x7000]]);
        assert_eq!(calls(&e, ADD_LIGHT), vec![vec![scene_node, 0x7001, 1]]);
    }

    #[test]
    fn set_mode_scan_keeps_an_existing_light_and_places_it_for_the_target() {
        let mut e = engine();
        stub(&mut e, GET_VATS_LIGHT, 0x900);
        let not_an_actor = object_with_slots(&mut e, &[(0x100, 0)]);
        e.set_global(TARGET_REFERENCE, not_an_actor);
        let this = new_vats(&mut e);
        e.set(this, Vats::pUnEquipWeapon, Ptr::new(0x7000));
        e.call_log = Some(vec![]);
        vats_set_mode(&mut e, this, 3, 0);
        assert_eq!(e.get(this, Vats::eMode), 3);
        assert!(e.get(this, Vats::pUnEquipWeapon).is_null());
        assert!(calls(&e, NI_ALLOCATE).is_empty());
        let is_actor = slot_target(&e, not_an_actor, 0x100);
        assert_eq!(calls(&e, is_actor).len(), 1);
    }

    #[test]
    fn set_mode_playback_starts_the_transition_modifier() {
        let mut e = engine();
        mode_four_doubles(&mut e);
        set_setting(&mut e, TRANSITION_SETTING_A, 1.0);
        set_setting(&mut e, TRANSITION_SETTING_B, 2.0);
        set_setting(&mut e, TRANSITION_SETTING_C, 3.0);
        set_setting(&mut e, TRANSITION_SETTING_D, 4.0);
        e.set_global(PLAYBACK_TIME, 5.0f32);
        e.set_global(PLAYBACK_SHOT_TIME, 6.0f32);
        e.set_global(ACTION_STARTED, 1u8);
        let this = new_vats(&mut e);
        e.set(this, Vats::iKillCount, 3);
        e.set(this, Vats::pImagespaceMod, Ptr::new(0x7200));
        e.set(this, Vats::pUnEquipWeapon, Ptr::new(0x7000));
        e.call_log = Some(vec![]);
        vats_set_mode(&mut e, this, 4, 0);
        assert_eq!(e.get(this, Vats::eMode), 4);
        assert_eq!(e.get(this, Vats::iKillCount), 0);
        assert!(e.get(this, Vats::pUnEquipWeapon).is_null());
        assert_eq!(e.global::<f32>(PLAYBACK_TIME), 0.0);
        assert_eq!(e.global::<u8>(ACTION_STARTED), 0);
        assert_eq!(calls(&e, IMAGESPACE_STOP), vec![vec![0x7200]]);
        let trigger = calls(&e, IMAGESPACE_RB_TRIGGER);
        assert_eq!(trigger.len(), 1);
        assert_eq!(trigger[0][0], 0);
        assert_eq!(
            &trigger[0][1..5],
            &[
                float_bits(4.0),
                float_bits(3.0),
                float_bits(2.0),
                float_bits(1.0)
            ]
        );
        assert_eq!(trigger[0][6], 0);
        let point = trigger[0][5];
        assert_eq!(
            calls(&e, POINT2_CONSTRUCTOR),
            vec![vec![point, float_bits(0.5), float_bits(0.5)]]
        );
        assert_eq!(e.get(this, Vats::pTransitionImagespaceMod).addr(), 0x777);
        assert_eq!(calls(&e, IMAGESPACE_ACTIVATE), vec![vec![0x777, 1]]);
        assert!(calls(&e, HUD_SET_MENU_MODE).is_empty());
    }

    #[test]
    fn set_mode_to_the_same_mode_does_nothing() {
        let mut e = engine();
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        e.call_log = Some(vec![]);
        vats_set_mode(&mut e, this, 4, 0);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        assert_eq!(e.get(this, Vats::eMode), 4);
    }

    #[test]
    fn set_mode_forced_runs_even_for_the_same_mode() {
        let mut e = engine();
        mode_four_doubles(&mut e);
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        e.set(this, Vats::iKillCount, 3);
        vats_set_mode(&mut e, this, 4, 1);
        assert_eq!(e.get(this, Vats::iKillCount), 0);
    }

    // ---- the update ---------------------------------------------------------------------------

    #[test]
    fn update_outside_playback_only_clears_the_flag() {
        let mut e = engine();
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 2);
        e.set_global(PLAYING_BACK_FLAG, 1u8);
        e.call_log = Some(vec![]);
        fn_009c7240(&mut e, this);
        assert_eq!(e.global::<u8>(PLAYING_BACK_FLAG), 0);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn update_with_an_empty_list_loads_the_countdown_from_the_setting() {
        let mut e = engine();
        set_setting(&mut e, QUIT_DELAY_SETTING, 2.5);
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        // The target reference is placed with the light when the process getter is true.
        let target = object_with_slots(&mut e, &[(0x100, 0)]);
        e.set_global(TARGET_REFERENCE, target);
        stub(&mut e, WORD_AT_0X08, 1);
        e.call_log = Some(vec![]);
        fn_009c7240(&mut e, this);
        assert_eq!(e.global::<u8>(PLAYING_BACK_FLAG), 1);
        assert_eq!(e.global::<f32>(END_COUNTDOWN), 2.5);
        let is_actor = slot_target(&e, target, 0x100);
        assert_eq!(calls(&e, is_actor).len(), 1);
    }

    #[test]
    fn update_runs_the_countdown_down_and_leaves_the_mode() {
        let mut e = engine();
        mode_zero_doubles(&mut e);
        stub_float(&mut e, TIMER_DELTA, 1.0);
        stub_float(&mut e, PLAYER_TIME_FACTOR, 1.0);
        stub_float(&mut e, TIMER_DIVISOR, 1.0);
        stub(&mut e, ACTOR_GET_CURRENT_WEAPON, 0);
        stub(&mut e, ACTOR_RESTORE_ACTOR_VALUE, 0);
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            e.mem.set_f32(a[4], 1.5);
            Ret::default()
        });
        let stranger = object_with_slots(&mut e, &[(0x10, 0)]);
        e.set_global(STRANGER_ACTOR, stranger);
        e.set_global(PLAYER_POINTER, 0x5000u32);
        e.set_global(DEFAULT_WEAPON, 0x7777u32);
        e.set_global(END_COUNTDOWN, 0.5f32);
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        e.set(this, Vats::iKillCount, 2);
        // The current actor is the stranger actor, so the playback is over.
        e.set_global(CURRENT_ACTOR, stranger);
        e.call_log = Some(vec![]);
        fn_009c7240(&mut e, this);
        assert_eq!(e.get(this, Vats::eMode), 0);
        assert_eq!(e.get(this, Vats::iKillCount), 0);
        assert_eq!(e.global::<f32>(END_COUNTDOWN), 0.0);
        assert_eq!(
            calls(&e, HANDLE_ENTRY_POINT)[0][..4],
            [0x23, 0x5000, 0x7777, 0x5000]
        );
        assert_eq!(
            calls(&e, ACTOR_RESTORE_ACTOR_VALUE),
            vec![vec![0x5000, 0xc, float_bits(1.5)]]
        );
    }

    #[test]
    fn update_countdown_asks_the_animation_action_when_the_actor_is_not_special() {
        let mut e = engine();
        stub_float(&mut e, TIMER_DELTA, 1.0);
        stub_float(&mut e, PLAYER_TIME_FACTOR, 1.0);
        stub_float(&mut e, TIMER_DIVISOR, 1.0);
        stub(&mut e, SET_GLOBAL_TIME_MULTIPLIER, 0);
        stub(&mut e, ACTOR_GET_ANIM_ACTION, 3);
        e.set_global(END_COUNTDOWN, 0.5f32);
        e.set_global(CURRENT_ACTOR, 0x5300u32);
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        e.call_log = Some(vec![]);
        fn_009c7240(&mut e, this);
        // Five questions (-1, 7, 9, 15, 17), none answered: the mode stays.
        assert_eq!(calls(&e, ACTOR_GET_ANIM_ACTION).len(), 5);
        assert_eq!(e.get(this, Vats::eMode), 4);
        assert_eq!(e.global::<f32>(END_COUNTDOWN), -0.5);
    }

    #[test]
    fn update_countdown_above_zero_advances_the_playback_time() {
        let mut e = engine();
        stub_float(&mut e, TIMER_DELTA, 1.0);
        stub_float(&mut e, PLAYER_TIME_FACTOR, 1.0);
        stub_float(&mut e, TIMER_DIVISOR, 1.0);
        stub(&mut e, SET_GLOBAL_TIME_MULTIPLIER, 0);
        e.set_global(END_COUNTDOWN, 5.0f32);
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        e.call_log = Some(vec![]);
        fn_009c7240(&mut e, this);
        assert_eq!(e.global::<f32>(END_COUNTDOWN), 4.0);
        assert_eq!(e.global::<f32>(PLAYBACK_TIME), 1.0);
        assert_eq!(
            calls(&e, SET_GLOBAL_TIME_MULTIPLIER),
            vec![vec![TIMER_OBJECT, float_bits(1.0), 0]]
        );
    }

    #[test]
    fn update_countdown_above_zero_drives_the_camera_and_the_slow_motion_state() {
        let mut e = engine();
        stub_float(&mut e, TIMER_DELTA, 1.0);
        stub_float(&mut e, PLAYER_TIME_FACTOR, 1.0);
        stub_float(&mut e, TIMER_DIVISOR, 1.0);
        stub(&mut e, SET_GLOBAL_TIME_MULTIPLIER, 0);
        stub(&mut e, CAMERA_IS_INITIALIZED, 1);
        stub(&mut e, CAMERA_ADVANCE, 0);
        stub_float(&mut e, FLOAT_AT_0X50, 0.25);
        stub(&mut e, ACTOR_GET_ANIM_ACTION, 3);
        stub(&mut e, HOLDER_GET_OBJECT, 0x66);
        stub(&mut e, HOLDER_OBJECT_APPLY, 0);
        e.set_global(PLAYER_POINTER, 0x5000u32);
        e.set_global(END_COUNTDOWN, 5.0f32);
        e.set_global(SLOW_MOTION_ACTION, 1u8);
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        e.set(this, Vats::pCamera, Ptr::new(0x1234));
        e.call_log = Some(vec![]);
        fn_009c7240(&mut e, this);
        assert_eq!(
            calls(&e, CAMERA_ADVANCE),
            vec![vec![0x1234, float_bits(1.0)]]
        );
        assert_eq!(
            calls(&e, SET_GLOBAL_TIME_MULTIPLIER),
            vec![vec![TIMER_OBJECT, float_bits(0.25), 0]]
        );
        assert_eq!(calls(&e, HOLDER_OBJECT_APPLY), vec![vec![0x66, 6]]);
        // With the slow-motion state active the fixed multiplier is used instead.
        e.mem.set_f32(SLOW_FLOAT_A, 0.125);
        e.set_global(SLOW_MOTION_ACTIVE, 1u8);
        e.set_global(END_COUNTDOWN, 5.0f32);
        e.call_log = Some(vec![]);
        fn_009c7240(&mut e, this);
        assert_eq!(
            calls(&e, SET_GLOBAL_TIME_MULTIPLIER),
            vec![vec![TIMER_OBJECT, float_bits(0.125), 0]]
        );
    }

    /// The doubles the head-command update calls on its way through an idle command.
    fn head_command_rig(e: &mut Engine, action: u32) -> (Ptr<Vats>, u32) {
        stub(e, HOLDER_GET_OBJECT, 0x11);
        stub(e, ACTOR_GET_PROCESS, 0);
        stub(e, IS_ATTACK_ACTION, 0);
        stub(e, SELECT_SHOT, 0);
        let animation = e.mem.alloc(0x200);
        stub(e, ACTOR_GET_ANIMATION, animation);
        stub(e, WORD_AT_0X08, 0);
        e.set_global(CURRENT_ACTOR, 0x5000u32);
        let this = new_vats(e);
        e.set(this, Vats::eMode, 4);
        let command = make_command(e, action);
        e.mem.set_u32(command + 0xc, 0xabc);
        fill_list(e, this.addr(), &[command]);
        (this, command)
    }

    #[test]
    fn update_picks_the_shot_for_an_idle_command() {
        let mut e = engine();
        let (this, command) = head_command_rig(&mut e, 12);
        e.call_log = Some(vec![]);
        fn_009c7240(&mut e, this);
        assert_eq!(e.global::<u32>(TARGET_REFERENCE), 0xabc);
        assert_eq!(e.global::<u32>(LAST_COMMAND), command);
        assert_eq!(e.global::<u32>(SELECTED_SHOT), 0);
        assert_eq!(e.global::<u32>(CAMERA_KIND), 0);
        assert!(e.get(this, Vats::pCameraList).is_null());
        assert_eq!(calls(&e, SELECT_SHOT), vec![vec![0, 0x5000, 0xabc]]);
        assert_eq!(list_items(&e, this.addr()), vec![command]);
    }

    #[test]
    fn update_uses_forced_shots_and_the_shot_kind_tests() {
        let mut e = engine();
        let (this, _) = head_command_rig(&mut e, 12);
        stub(&mut e, SHOT_FLAG_FIRST, 0);
        stub(&mut e, SHOT_FLAG_SECOND, 0);
        stub(&mut e, SHOT_FLAG_THIRD, 1);
        let camera_list = e.mem.alloc(8);
        stub(&mut e, SHOT_CAMERA_LIST, camera_list);
        e.set_global(FORCED_SHOT_FIRST, 0x300u32);
        fn_009c7240(&mut e, this);
        assert_eq!(e.global::<u32>(SELECTED_SHOT), 0x300);
        assert_eq!(e.global::<u32>(CAMERA_KIND), 2);
        assert_eq!(e.get(this, Vats::pCameraList).addr(), camera_list);
    }

    #[test]
    fn update_consumes_the_second_forced_shot() {
        let mut e = engine();
        let (this, _) = head_command_rig(&mut e, 12);
        stub(&mut e, SHOT_FLAG_FIRST, 0);
        stub(&mut e, SHOT_FLAG_SECOND, 1);
        stub(&mut e, SHOT_CAMERA_LIST, 0);
        e.set_global(FORCED_SHOT_SECOND, 0x400u32);
        fn_009c7240(&mut e, this);
        assert_eq!(e.global::<u32>(SELECTED_SHOT), 0x400);
        assert_eq!(e.global::<u32>(FORCED_SHOT_SECOND), 0);
        assert_eq!(e.global::<u32>(CAMERA_KIND), 1);
    }

    #[test]
    fn update_starts_the_flag_state_toggle_for_action_thirteen() {
        let mut e = engine();
        let (this, _) = head_command_rig(&mut e, 13);
        stub(&mut e, ACTOR_GET_ANIM_ACTION, u32::MAX);
        stub(&mut e, ACTOR_GET_FLAG_STATE, 0);
        stub(&mut e, ACTOR_SET_FLAG_STATE, 0);
        e.call_log = Some(vec![]);
        fn_009c7240(&mut e, this);
        assert_eq!(calls(&e, ACTOR_SET_FLAG_STATE), vec![vec![0x5000, 1]]);
        assert_eq!(e.global::<u8>(ACTION_STARTED), 1);
        // Once started the action ends with the animation: the head is freed.
        fn_009c7240(&mut e, this);
        assert_eq!(list_items(&e, this.addr()), vec![0]);
    }

    #[test]
    fn update_starts_a_player_action_for_actions_ten_and_eleven() {
        let mut e = engine();
        let (this, command) = head_command_rig(&mut e, 10);
        let player = object_with_slots(&mut e, &[(0x234, 0), (0x22c, 0), (0x230, 0), (0x33c, 0)]);
        e.set_global(PLAYER_POINTER, player);
        e.set_global(CURRENT_ACTOR, player);
        stub(&mut e, ACTOR_GET_ANIM_ACTION, u32::MAX);
        stub(&mut e, ACTOR_STATE_TEST_FIRST, 0);
        stub(&mut e, ACTOR_STATE_TEST_SECOND, 0);
        stub(&mut e, PLAYER_START_ACTION, 0);
        e.mem.set_f32(command + 0x20, 2.5);
        e.call_log = Some(vec![]);
        fn_009c7240(&mut e, this);
        assert_eq!(calls(&e, PLAYER_START_ACTION), vec![vec![player, 0x400, 1]]);
        assert_eq!(list_items(&e, this.addr()), vec![0]);
        // The time the action left over goes to the player's virtual call.
        let leftover = slot_target(&e, player, 0x33c);
        assert_eq!(calls(&e, leftover), vec![vec![player, float_bits(2.5)]]);
    }

    #[test]
    fn update_starts_an_actor_action_for_other_actors() {
        let mut e = engine();
        let (this, _) = head_command_rig(&mut e, 11);
        let actor = object_with_slots(&mut e, &[(0x234, 0), (0x22c, 0), (0x230, 0)]);
        e.set_global(CURRENT_ACTOR, actor);
        e.set_global(PLAYER_POINTER, 0x5000u32);
        stub(&mut e, ACTOR_GET_ANIM_ACTION, u32::MAX);
        stub(&mut e, ACTOR_STATE_TEST_FIRST, 0);
        stub(&mut e, ACTOR_STATE_TEST_SECOND, 0);
        stub(&mut e, ACTOR_START_ACTION, 0);
        e.call_log = Some(vec![]);
        fn_009c7240(&mut e, this);
        assert_eq!(calls(&e, ACTOR_START_ACTION), vec![vec![actor, 0]]);
    }

    #[test]
    fn update_plays_an_attack_command_to_its_end() {
        let mut e = engine();
        let (this, command) = head_command_rig(&mut e, 3);
        let player = object_with_slots(&mut e, &[(0x33c, 0)]);
        let process = object_with_slots(&mut e, &[(0x148, 0)]);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        stub(&mut e, IS_ATTACK_ACTION, 1);
        stub(&mut e, IS_ACTION_SEVEN_OR_EIGHT, 0);
        stub(&mut e, ACTOR_GET_ANIM_ACTION, 0);
        stub(&mut e, HOLDER_OBJECT_APPLY, 0);
        stub_all(&mut e, &[HIT_DATA_CLEAR, HIT_DATA_INITIALIZE]);
        e.mem.set_f32(PICK_TIME_FLOAT_OFF, 1.0);
        e.mem.set_f32(command + 0x20, 1.5);
        e.set_global(PLAYER_POINTER, player);
        e.call_log = Some(vec![]);
        fn_009c7240(&mut e, this);
        assert_eq!(e.global::<u8>(SLOW_MOTION_ACTION), 1);
        assert_eq!(calls(&e, HOLDER_OBJECT_APPLY), vec![vec![0x11, 6]]);
        // No camera and no projectile: the command is finished and its leftover time passed on.
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![command]]);
        assert_eq!(list_items(&e, this.addr()), vec![0]);
        let leftover = slot_target(&e, player, 0x33c);
        assert_eq!(calls(&e, leftover), vec![vec![player, float_bits(1.5)]]);
    }

    #[test]
    fn update_keeps_an_attack_command_that_still_has_fire_shots() {
        let mut e = engine();
        let (this, command) = head_command_rig(&mut e, 3);
        let process = object_with_slots(&mut e, &[(0x148, 0)]);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        stub(&mut e, IS_ATTACK_ACTION, 1);
        stub(&mut e, IS_ACTION_SEVEN_OR_EIGHT, 1);
        stub_all(
            &mut e,
            &[HIT_DATA_CLEAR, HIT_DATA_INITIALIZE, GET_IDLE_TO_PLAY],
        );
        e.mem.set_u8(command + 8, 2);
        e.set_global(PLAYER_POINTER, 0x5000u32);
        e.call_log = Some(vec![]);
        fn_009c7240(&mut e, this);
        assert_eq!(e.global::<u8>(SLOW_MOTION_ACTION), 0);
        assert_eq!(list_items(&e, this.addr()), vec![command]);
        assert!(calls(&e, FREE_BLOCK).is_empty());
    }

    // ---- the camera pieces of the update ------------------------------------------------------

    #[test]
    fn camera_timing_without_a_camera_restores_the_time_multiplier() {
        let mut e = engine();
        stub_float(&mut e, TIMER_DELTA, 2.0);
        stub_float(&mut e, TIMER_DIVISOR, 1.0);
        stub(&mut e, SET_GLOBAL_TIME_MULTIPLIER, 0);
        set_setting(&mut e, SHOT_IDLE_LIMIT_SETTING, 100.0);
        let this = new_vats(&mut e);
        e.call_log = Some(vec![]);
        update_camera_timing(&mut e, this);
        assert_eq!(e.global::<f32>(PLAYBACK_TIME), 2.0);
        assert_eq!(e.global::<f32>(PLAYBACK_SHOT_TIME), 2.0);
        assert_eq!(
            calls(&e, SET_GLOBAL_TIME_MULTIPLIER),
            vec![vec![TIMER_OBJECT, float_bits(1.0), 0]]
        );
    }

    #[test]
    fn camera_timing_quits_the_playback_after_the_idle_limit() {
        let mut e = engine();
        stub_float(&mut e, TIMER_DELTA, 2.0);
        stub_float(&mut e, TIMER_DIVISOR, 1.0);
        stub(&mut e, SET_GLOBAL_TIME_MULTIPLIER, 0);
        set_setting(&mut e, SHOT_IDLE_LIMIT_SETTING, 1.0);
        set_setting(&mut e, QUIT_DELAY_SETTING, 9.0);
        let this = new_vats(&mut e);
        e.set(this, Vats::eMode, 4);
        update_camera_timing(&mut e, this);
        assert_eq!(e.global::<f32>(END_COUNTDOWN), 9.0);
    }

    #[test]
    fn camera_timing_drives_an_initialized_shot() {
        let mut e = engine();
        stub_float(&mut e, TIMER_DELTA, 1.0);
        stub_float(&mut e, TIMER_DIVISOR, 1.0);
        stub(&mut e, SET_GLOBAL_TIME_MULTIPLIER, 0);
        stub(&mut e, CAMERA_IS_INITIALIZED, 1);
        stub(&mut e, CAMERA_ADVANCE, 0);
        stub(&mut e, PLAYER_SET_FIRST_PERSON, 0);
        stub(&mut e, IMAGESPACE_SET_STRENGTH, 0);
        stub_float(&mut e, FLOAT_AT_0X50, 0.5);
        set_setting(&mut e, SHOT_IDLE_LIMIT_SETTING, 100.0);
        set_setting(&mut e, MODIFIER_LIMIT_SETTING, 2.0);
        e.set_global(FIRST_PERSON_PENDING, 1u8);
        e.set_global(SLOW_MOTION_BLEND, 0.75f32);
        e.set_global(PLAYER_POINTER, 0x5000u32);
        let this = new_vats(&mut e);
        e.set(this, Vats::pCamera, Ptr::new(0x1234));
        e.set(this, Vats::pTransitionImagespaceMod, Ptr::new(0x4321));
        e.call_log = Some(vec![]);
        update_camera_timing(&mut e, this);
        assert_eq!(calls(&e, PLAYER_SET_FIRST_PERSON), vec![vec![0x5000, 0]]);
        assert_eq!(e.global::<u8>(FIRST_PERSON_PENDING), 0);
        assert_eq!(
            calls(&e, CAMERA_ADVANCE),
            vec![vec![0x1234, float_bits(1.0)]]
        );
        assert_eq!(
            calls(&e, SET_GLOBAL_TIME_MULTIPLIER),
            vec![vec![TIMER_OBJECT, float_bits(0.5), 0]]
        );
        // Blend 0.75 is above 0.5, so it is mirrored to 0.25 and scaled by the setting.
        assert_eq!(
            calls(&e, IMAGESPACE_SET_STRENGTH),
            vec![vec![0x4321, float_bits(0.5)]]
        );
    }

    #[test]
    fn camera_timing_copies_the_playback_time_when_slow_motion_starts() {
        let mut e = engine();
        stub_float(&mut e, TIMER_DELTA, 0.0);
        stub_float(&mut e, TIMER_DIVISOR, 1.0);
        stub(&mut e, SET_GLOBAL_TIME_MULTIPLIER, 0);
        set_setting(&mut e, SHOT_IDLE_LIMIT_SETTING, 100.0);
        e.set_global(SLOW_MOTION_ACTIVE, 1u8);
        e.set_global(SLOW_MOTION_BLEND, 0.0f32);
        e.set_global(PLAYBACK_TIME, 7.0f32);
        let this = new_vats(&mut e);
        update_camera_timing(&mut e, this);
        assert_eq!(e.global::<f32>(SLOW_MOTION_SAVED_TIME), 7.0);
    }

    fn shot_rig(e: &mut Engine) -> (Ptr<Vats>, u32, u32) {
        let camera = e.mem.alloc(0x100);
        stub_float(e, FLOAT_AT_0X54, 0.0005);
        set_setting(e, SHOT_IDLE_LIMIT_SETTING, 7.0);
        set_setting(e, MIN_SHOT_TIME_SETTING, 1.0);
        stub_all(
            e,
            &[
                INIT_CAMERA,
                CAMERA_SET_VIEW_TARGET,
                CAMERA_SET_SECOND_TARGET,
                IMAGESPACE_ACTIVATE,
            ],
        );
        stub(e, WORD_AT_0X3C, 0);
        stub(e, WORD_AT_0X40, 0);
        stub(e, ACTOR_GET_ANIMATION, 0x100);
        stub(e, WORD_AT_0X0C, 0x200);
        stub(e, ARRAY_ELEMENT_BY_INDEX, 0x300);
        stub(e, CAMERA_TEST_0058D4F0, 0);
        stub(e, CAMERA_IS_INITIALIZED, 1);
        stub(e, IMAGESPACE_FORM_TRIGGER, 0x888);
        e.set_global(CURRENT_ACTOR, 0x5000u32);
        let this = new_vats(e);
        e.set(this, Vats::pCamera, Ptr::new(camera));
        e.set(this, Vats::fCameraMinTimer, 0.5f32);
        let head = make_command(e, 3);
        (this, head, camera)
    }

    #[test]
    fn camera_shot_starts_with_the_default_targets() {
        let mut e = engine();
        let (this, head, camera) = shot_rig(&mut e);
        e.mem.set_u32(camera + 0x70, 0x77);
        e.set_global(PLAYBACK_SHOT_TIME, 5.0f32);
        e.call_log = Some(vec![]);
        start_camera_shot(&mut e, this, head);
        // A shot time at or below the threshold constant is replaced by the setting.
        assert_eq!(e.get(this, Vats::fCameraTimer), 7.0);
        assert_eq!(e.global::<f32>(PLAYBACK_SHOT_TIME), 0.0);
        assert_eq!(e.get(this, Vats::fCameraMinTimer), 1.0);
        assert_eq!(calls(&e, INIT_CAMERA), vec![vec![camera, 0x5000, 0]]);
        assert_eq!(calls(&e, CAMERA_SET_VIEW_TARGET), vec![vec![camera, 0x300]]);
        assert_eq!(
            calls(&e, CAMERA_SET_SECOND_TARGET),
            vec![vec![camera, 0x300]]
        );
        assert_eq!(e.get(this, Vats::pImagespaceMod).addr(), 0x888);
        assert_eq!(
            calls(&e, IMAGESPACE_FORM_TRIGGER),
            vec![vec![0, float_bits(1.0), 0x77]]
        );
        assert_eq!(calls(&e, IMAGESPACE_ACTIVATE), vec![vec![0x888, 1]]);
    }

    #[test]
    fn camera_shot_keeps_a_long_shot_time_and_a_larger_minimum_timer() {
        let mut e = engine();
        let (this, head, _) = shot_rig(&mut e);
        stub_float(&mut e, FLOAT_AT_0X54, 3.0);
        e.set(this, Vats::fCameraMinTimer, 2.0f32);
        e.set_global(PLAYBACK_SHOT_TIME, 5.0f32);
        start_camera_shot(&mut e, this, head);
        assert_eq!(e.get(this, Vats::fCameraTimer), 3.0);
        assert_eq!(e.global::<f32>(PLAYBACK_SHOT_TIME), 5.0);
        assert_eq!(e.get(this, Vats::fCameraMinTimer), 2.0);
    }

    #[test]
    fn camera_shot_updates_a_running_modifier_and_a_preferred_target() {
        let mut e = engine();
        let (this, head, camera) = shot_rig(&mut e);
        stub(&mut e, CAMERA_TEST_0058D4F0, 1);
        let process = object_with_slots(&mut e, &[(0x190, 0x555)]);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        stub(&mut e, ACTOR_FACE_VALUE, 0);
        stub_all(&mut e, &[IMAGESPACE_UPDATE]);
        set_setting(&mut e, MODIFIER_STRENGTH_SETTING, 0.75);
        e.set(this, Vats::pImagespaceMod, Ptr::new(0x999));
        e.mem.set_u32(camera + 0x34, 0x66);
        e.mem.set_u32(camera + 0x70, 0x77);
        e.call_log = Some(vec![]);
        start_camera_shot(&mut e, this, head);
        assert_eq!(calls(&e, CAMERA_SET_VIEW_TARGET), vec![vec![camera, 0x555]]);
        assert_eq!(
            calls(&e, IMAGESPACE_UPDATE),
            vec![vec![0x999, 0x66, float_bits(0.75), float_bits(1.0), 0x77]]
        );
        assert!(calls(&e, IMAGESPACE_FORM_TRIGGER).is_empty());
    }

    #[test]
    fn camera_shot_with_selector_two_aims_at_the_hit_body_part() {
        let mut e = engine();
        let (this, head, camera) = shot_rig(&mut e);
        stub(&mut e, WORD_AT_0X3C, 2);
        stub(&mut e, WORD_AT_0X40, 2);
        let body_data = 0x7000;
        let base = object_with_slots(&mut e, &[(0x180, body_data)]);
        let target = object_with_slots(&mut e, &[(0x100, 1), (0x1d0, 0x6100)]);
        e.mem.set_u32(head + 0xc, target);
        let hit_data = e.mem.alloc(0x40);
        e.mem.set_u32(hit_data + 0x10, 3);
        e.mem.set_u32(head + 0x14, hit_data);
        e.mem.set_u32(head + 0x10, 3);
        stub(&mut e, REFERENCE_BASE, base);
        stub(&mut e, BODY_PART_DATA, 0);
        stub(&mut e, GET_BODY_PART_BY_LIMB, 0x7100);
        stub(&mut e, GET_DISMEMBERED, 0);
        stub(&mut e, BODY_PART_NODE_NAME, 0x7200);
        stub(&mut e, FIND_NODE_BY_NAME, 0x7300);
        e.call_log = Some(vec![]);
        start_camera_shot(&mut e, this, head);
        assert_eq!(calls(&e, GET_BODY_PART_BY_LIMB), vec![vec![body_data, 3]]);
        assert_eq!(calls(&e, FIND_NODE_BY_NAME), vec![vec![0x6100, 0x7200]]);
        assert_eq!(
            calls(&e, CAMERA_SET_VIEW_TARGET),
            vec![vec![camera, 0x7300]]
        );
        assert_eq!(
            calls(&e, CAMERA_SET_SECOND_TARGET),
            vec![vec![camera, 0x7300]]
        );
        // The camera is initialized for the target, not the current actor.
        assert_eq!(calls(&e, INIT_CAMERA), vec![vec![camera, target, 0]]);
    }

    #[test]
    fn camera_shot_uses_the_severed_part_of_a_dismembered_limb() {
        let mut e = engine();
        let (this, head, camera) = shot_rig(&mut e);
        stub(&mut e, WORD_AT_0X3C, 2);
        let base = object_with_slots(&mut e, &[(0x180, 0x7000)]);
        let target = object_with_slots(&mut e, &[(0x100, 1), (0x1d0, 0x6100)]);
        e.mem.set_u32(head + 0xc, target);
        let hit_data = e.mem.alloc(0x40);
        e.mem.set_u32(hit_data + 0x10, 3);
        e.mem.set_u32(head + 0x14, hit_data);
        let process = object_with_slots(&mut e, &[(0x6d0, 0x6200)]);
        stub(&mut e, ACTOR_GET_PROCESS, process);
        stub(&mut e, REFERENCE_BASE, base);
        stub(&mut e, BODY_PART_DATA, 0);
        stub(&mut e, GET_BODY_PART_BY_LIMB, 0x7100);
        stub(&mut e, GET_DISMEMBERED, 1);
        stub(&mut e, BODY_PART_NODE_NAME, 0x7200);
        stub(&mut e, FIND_NODE_BY_NAME, 0x7300);
        e.call_log = Some(vec![]);
        start_camera_shot(&mut e, this, head);
        assert_eq!(calls(&e, FIND_NODE_BY_NAME), vec![vec![0x6200, 0x7200]]);
        assert_eq!(
            calls(&e, CAMERA_SET_VIEW_TARGET),
            vec![vec![camera, 0x7300]]
        );
    }

    #[test]
    fn camera_shot_falls_back_to_the_first_body_part_without_a_hit_limb() {
        let mut e = engine();
        let (this, head, camera) = shot_rig(&mut e);
        stub(&mut e, WORD_AT_0X3C, 2);
        let base = object_with_slots(&mut e, &[(0x180, 0x7000)]);
        let target = object_with_slots(&mut e, &[(0x100, 1), (0x1d0, 0x6100)]);
        e.mem.set_u32(head + 0xc, target);
        let hit_data = e.mem.alloc(0x40);
        e.mem.set_i32(hit_data + 0x10, -1);
        e.mem.set_u32(head + 0x14, hit_data);
        stub(&mut e, REFERENCE_BASE, base);
        stub(&mut e, BODY_PART_DATA, 0);
        stub(&mut e, GET_BODY_PART, 0x7150);
        stub(&mut e, BODY_PART_NODE_NAME, 0x7250);
        stub(&mut e, FIND_NODE_BY_NAME, 0x7350);
        e.call_log = Some(vec![]);
        start_camera_shot(&mut e, this, head);
        assert_eq!(calls(&e, GET_BODY_PART), vec![vec![0x7000, 0]]);
        assert_eq!(
            calls(&e, CAMERA_SET_VIEW_TARGET),
            vec![vec![camera, 0x7350]]
        );
    }

    #[test]
    fn camera_shot_with_selector_one_tracks_the_projectile_references() {
        let mut e = engine();
        let (this, head, camera) = shot_rig(&mut e);
        stub(&mut e, WORD_AT_0X3C, 1);
        stub(&mut e, WORD_AT_0X40, 1);
        let first = object_with_slots(&mut e, &[(0x1d0, 0x91), (0x30c, 0)]);
        e.set(this, Vats::pFirstProjectile, Ptr::new(first));
        e.mem.set_u32(camera + 0x44, 0x10);
        stub(&mut e, REFERENCE_FLAG_TEST, 1);
        stub(&mut e, REFERENCE_FLAG_CHANGE, 0);
        e.call_log = Some(vec![]);
        start_camera_shot(&mut e, this, head);
        assert_eq!(calls(&e, CAMERA_SET_VIEW_TARGET), vec![vec![camera, 0x91]]);
        assert_eq!(
            calls(&e, CAMERA_SET_SECOND_TARGET),
            vec![vec![camera, 0x91]]
        );
        assert_eq!(calls(&e, REFERENCE_FLAG_CHANGE), vec![vec![first, 0x10, 0]]);
        let notify = slot_target(&e, first, 0x30c);
        assert_eq!(calls(&e, notify), vec![vec![first]]);
    }

    #[test]
    fn camera_shot_with_selector_one_tests_the_first_reference_for_the_second() {
        let mut e = engine();
        let (this, head, camera) = shot_rig(&mut e);
        stub(&mut e, WORD_AT_0X3C, 1);
        stub(&mut e, WORD_AT_0X40, 1);
        let second = object_with_slots(&mut e, &[(0x1d0, 0x92), (0x30c, 0)]);
        e.set(this, Vats::pLastProjectile, Ptr::new(second));
        e.mem.set_u32(camera + 0x44, 0x10);
        stub(&mut e, REFERENCE_FLAG_TEST, 1);
        stub(&mut e, REFERENCE_FLAG_CHANGE, 0);
        e.call_log = Some(vec![]);
        start_camera_shot(&mut e, this, head);
        assert_eq!(calls(&e, CAMERA_SET_VIEW_TARGET), vec![vec![camera, 0x92]]);
        // The exe tests the (null) first reference, not the second one.
        assert_eq!(calls(&e, REFERENCE_FLAG_TEST), vec![vec![0, 0x10]]);
        assert_eq!(
            calls(&e, REFERENCE_FLAG_CHANGE),
            vec![vec![second, 0x10, 0]]
        );
    }

    #[test]
    fn camera_shot_with_selector_one_and_no_references_uses_the_preferred_target() {
        let mut e = engine();
        let (this, head, camera) = shot_rig(&mut e);
        stub(&mut e, WORD_AT_0X3C, 1);
        stub(&mut e, WORD_AT_0X40, 1);
        e.call_log = Some(vec![]);
        start_camera_shot(&mut e, this, head);
        assert!(calls(&e, CAMERA_SET_VIEW_TARGET).is_empty());
        assert_eq!(
            calls(&e, CAMERA_SET_SECOND_TARGET),
            vec![vec![camera, 0x300], vec![camera, 0x300]]
        );
    }

    #[test]
    fn camera_shot_selector_one_is_skipped_when_the_shot_has_a_pointer() {
        let mut e = engine();
        let (this, head, camera) = shot_rig(&mut e);
        stub(&mut e, WORD_AT_0X3C, 1);
        stub(&mut e, WORD_AT_0X40, 1);
        e.mem.set_u32(camera + 0x60, 1);
        e.mem.set_u32(camera + 0x64, 1);
        e.call_log = Some(vec![]);
        start_camera_shot(&mut e, this, head);
        assert!(calls(&e, CAMERA_SET_VIEW_TARGET).is_empty());
        assert!(calls(&e, CAMERA_SET_SECOND_TARGET).is_empty());
    }

    #[test]
    fn camera_shot_advances_when_the_new_shot_is_not_started() {
        let mut e = engine();
        let (this, head, camera) = shot_rig(&mut e);
        stub(&mut e, CAMERA_IS_INITIALIZED, 0);
        stub(&mut e, FREE_CAMERA, 0);
        e.call_log = Some(vec![]);
        start_camera_shot(&mut e, this, head);
        assert_eq!(calls(&e, FREE_CAMERA), vec![vec![camera]]);
        assert!(e.get(this, Vats::pCamera).is_null());
    }

    // ---- the camera stage decision ------------------------------------------------------------

    fn stage_rig(e: &mut Engine) -> (Ptr<Vats>, Ptr<VatsCommand>) {
        let this = new_vats(e);
        let command = e.new_object::<VatsCommand>();
        e.set(command, VatsCommand::eAction, 3);
        e.set(command, VatsCommand::pTarget, Ptr::new(0x6000));
        e.set_global(CAMERA_STAGE, 0i32);
        (this, command)
    }

    #[test]
    fn stage_for_action_eighteen_is_two_and_flags_the_camera() {
        let mut e = engine();
        let (this, command) = stage_rig(&mut e);
        let camera = e.mem.alloc(0x100);
        e.set(this, Vats::pCamera, Ptr::new(camera));
        e.set(command, VatsCommand::eAction, 0x12);
        fn_009c95e0(&mut e, this, command);
        assert_eq!(e.global::<i32>(CAMERA_STAGE), 2);
        assert_eq!(e.mem.u8(camera + 0x76), 1);
    }

    #[test]
    fn stage_without_references_moves_from_one_to_two() {
        let mut e = engine();
        let (this, command) = stage_rig(&mut e);
        e.set_global(CAMERA_STAGE, 1i32);
        fn_009c95e0(&mut e, this, command);
        assert_eq!(e.global::<i32>(CAMERA_STAGE), 2);
        e.set_global(CAMERA_STAGE, 0i32);
        fn_009c95e0(&mut e, this, command);
        assert_eq!(e.global::<i32>(CAMERA_STAGE), 0);
    }

    #[test]
    fn stage_with_only_a_second_reference_is_two() {
        let mut e = engine();
        let (this, command) = stage_rig(&mut e);
        e.set(this, Vats::pLastProjectile, Ptr::new(0x77));
        fn_009c95e0(&mut e, this, command);
        assert_eq!(e.global::<i32>(CAMERA_STAGE), 2);
    }

    #[test]
    fn stage_with_both_references_and_no_camera_is_two() {
        let mut e = engine();
        let (this, command) = stage_rig(&mut e);
        e.set(this, Vats::pFirstProjectile, Ptr::new(0x66));
        e.set(this, Vats::pLastProjectile, Ptr::new(0x77));
        fn_009c95e0(&mut e, this, command);
        assert_eq!(e.global::<i32>(CAMERA_STAGE), 2);
    }

    #[test]
    fn stage_with_both_references_and_a_two_stage_shot_is_two() {
        let mut e = engine();
        let (this, command) = stage_rig(&mut e);
        e.set(this, Vats::pFirstProjectile, Ptr::new(0x66));
        e.set(this, Vats::pLastProjectile, Ptr::new(0x77));
        e.set(this, Vats::pCamera, Ptr::new(0x1234));
        stub(&mut e, WORD_AT_0X38, 2);
        fn_009c95e0(&mut e, this, command);
        assert_eq!(e.global::<i32>(CAMERA_STAGE), 2);
    }

    /// A first reference only, with the distance check: returns the stage after the decision.
    fn stage_by_distance(length: f64) -> i32 {
        let mut e = engine();
        let (this, command) = stage_rig(&mut e);
        let position = vec_block(&mut e, 0.0, 0.0, 0.0);
        let bounds = vec_block(&mut e, 0.0, 2.0, 0.0);
        let first = object_with_slots(&mut e, &[(0x1f4, position)]);
        let target = object_with_slots(&mut e, &[(0x1f4, position), (0x1dc, bounds)]);
        e.set(command, VatsCommand::pTarget, Ptr::new(target));
        e.set(this, Vats::pFirstProjectile, Ptr::new(first));
        e.mem.set_u32(CAMERA_DISTANCE_SETTING + 4, 3);
        stub(&mut e, POINT3_SUBTRACT, position);
        stub_float(&mut e, POINT3_LENGTH, length);
        fn_009c95e0(&mut e, this, command);
        e.global::<i32>(CAMERA_STAGE)
    }

    #[test]
    fn stage_by_distance_compares_the_bound_plus_distance_with_the_length() {
        // Stage 1 is set by the first reference; the bound (2.0) plus the setting (3) is 5.0.
        assert_eq!(stage_by_distance(10.0), 1);
        assert_eq!(stage_by_distance(4.0), 2);
        assert_eq!(stage_by_distance(5.0), 1);
        assert_eq!(stage_by_distance(f64::NAN), 1);
    }

    #[test]
    fn stage_checks_the_second_reference_distance_too() {
        let mut e = engine();
        let (this, command) = stage_rig(&mut e);
        let position = vec_block(&mut e, 0.0, 0.0, 0.0);
        let bounds = vec_block(&mut e, 0.0, 2.0, 0.0);
        let first = object_with_slots(&mut e, &[(0x1f4, position)]);
        let second = object_with_slots(&mut e, &[(0x1f4, position)]);
        let target = object_with_slots(&mut e, &[(0x1f4, position), (0x1dc, bounds)]);
        e.set(command, VatsCommand::pTarget, Ptr::new(target));
        e.set(this, Vats::pFirstProjectile, Ptr::new(first));
        e.set(this, Vats::pLastProjectile, Ptr::new(second));
        let camera = e.mem.alloc(0x100);
        e.set(this, Vats::pCamera, Ptr::new(camera));
        stub(&mut e, WORD_AT_0X38, 1);
        e.mem.set_u32(CAMERA_DISTANCE_SETTING + 4, 3);
        stub(&mut e, POINT3_SUBTRACT, position);
        let lengths = std::rc::Rc::new(std::cell::RefCell::new(vec![10.0, 4.0]));
        let queue = lengths.clone();
        e.register_double(POINT3_LENGTH, move |_, _| st0(queue.borrow_mut().remove(0)));
        fn_009c95e0(&mut e, this, command);
        assert_eq!(e.global::<i32>(CAMERA_STAGE), 2);
        assert!(lengths.borrow().is_empty());
    }

    #[test]
    fn melee_stage_is_decided_once_and_advances_the_cameras() {
        let mut e = engine();
        let (this, command) = stage_rig(&mut e);
        e.set(command, VatsCommand::eAction, 1);
        stub(&mut e, ACTOR_GET_ANIMATION, 0x100);
        stub(&mut e, ANIMATION_ACTION_OBJECT, 0);
        stub(&mut e, WORD_AT_0X38, 5);
        e.set_global(CURRENT_ACTOR, 0x5000u32);
        fn_009c95e0(&mut e, this, command);
        assert_eq!(e.global::<i32>(CAMERA_STAGE), 2);
        // An already decided stage is left alone.
        e.set_global(CAMERA_STAGE, 1i32);
        e.call_log = Some(vec![]);
        fn_009c95e0(&mut e, this, command);
        assert_eq!(e.global::<i32>(CAMERA_STAGE), 1);
        assert!(calls(&e, ANIMATION_ACTION_OBJECT).is_empty());
    }

    #[test]
    fn melee_stage_follows_the_action_record_tests() {
        let mut e = engine();
        let (this, command) = stage_rig(&mut e);
        e.set(command, VatsCommand::eAction, 1);
        e.set(command, VatsCommand::cFireShots, 0);
        stub(&mut e, ACTOR_GET_ANIMATION, 0x100);
        stub(&mut e, ANIMATION_ACTION_OBJECT, 0x200);
        stub(&mut e, ACTION_RECORD, 0x300);
        stub(&mut e, ACTION_RECORD_TEST_FIRST, 1);
        stub(&mut e, WORD_AT_0X38, 5);
        e.set_global(CURRENT_ACTOR, 0x5000u32);
        // First test true and no fire shots left: stage two.
        fn_009c95e0(&mut e, this, command);
        assert_eq!(e.global::<i32>(CAMERA_STAGE), 2);
        // Fire shots left: the second test and the kind count decide.
        e.set_global(CAMERA_STAGE, 0i32);
        e.set(command, VatsCommand::cFireShots, 1);
        stub(&mut e, ACTION_RECORD_TEST_SECOND, 1);
        stub(&mut e, ANIMATION_KIND_COUNT, 1);
        fn_009c95e0(&mut e, this, command);
        assert_eq!(e.global::<i32>(CAMERA_STAGE), 2);
        // Kind count zero: the target's virtual call decides.
        e.set_global(CAMERA_STAGE, 0i32);
        stub(&mut e, ANIMATION_KIND_COUNT, 0);
        let target = object_with_slots(&mut e, &[(0x22c, 0)]);
        e.set(command, VatsCommand::pTarget, Ptr::new(target));
        fn_009c95e0(&mut e, this, command);
        assert_eq!(e.global::<i32>(CAMERA_STAGE), 0);
        let target = object_with_slots(&mut e, &[(0x22c, 1)]);
        e.set(command, VatsCommand::pTarget, Ptr::new(target));
        fn_009c95e0(&mut e, this, command);
        assert_eq!(e.global::<i32>(CAMERA_STAGE), 2);
    }
}

//! `fallout shared/animation.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds the animation sequence containers (`AnimSequenceBase`
//! and its two subclasses) and the `Animation` object every animated
//! reference owns. Session 1 (b0010) covers `0048ee70` to `00491090`, the
//! first 40 functions: the sequence containers, `Animation`'s constructor
//! and destructor, and `AddAnimation` with the functions that feed it.
//! Session 2 (b0010) covers `004910d0` to `00495da0`, the next 40:
//! `FindSkinnedNode`, `Update` (the frame step, split into private helpers
//! for its parts) with the flag helpers, the movement and scene graph
//! updates, `InitGroupSpeed`, the group functions (`AddGroup`,
//! `GroupLoaded`, `PlayGroup`, `StartGroup`, `StartGroup_ov2`,
//! `ForceSection`, `PickBestAnimation`, `SyncSequences`) and the small
//! getters between them.
//! Session 3 (b0010) covers `00495e40` to `00497f20`, the next 40: the text
//! key searches, `ClearGroup`, the transform and accumulation root resets,
//! `GetTESAnimGroup`, and the `AnimIdle` class (constructor, destructors,
//! `Loaded`, the state steps and the save and load functions, laid out by
//! [`AnimIdle`] below) with `SpecialIdleQueue`.
//! Session 4 (b0010) covers `00498030` to `0049c050`, the next 40: the special
//! idle functions (`SpecialIdleReplace`, `SpecialIdleFree`, `AnimIdleFree` and
//! the `SpecialIdleWorking` family), `ClearBlendInterps`,
//! `ClearControllersInterpolators`, `ReloadTargets`, `BlendOut`, the animation's
//! save and load functions (`SaveGame`, `SaveAnimation`, the load game side and
//! the buffer variants), `UpdateBipOnly`, and the small sequence helpers up to
//! the `NiTPointerMap` constructor.
//! Session 5 (b0010) finishes the unit: the sequence map's destructors,
//! `SetAt`, `GetAt`, `GetNext`, `SetValue` and base constructor (`0049c080` to
//! `0049c5a0`), the two folded `NiQuatTransform` getters (`0058cb00`,
//! `00a3f9e0`) with `0058cb60`, and `Animation::SkipUpdate` (`008eeaa0`).
//! Session 6 translates the last one, `00f39c20` (`Animation::SpecialIdleAuto`,
//! a static initializer); the unit is complete.
//!
//! The 8 slots of `Animation` (`group`, `action`, `loopCount`, `nextGroup`,
//! `nextLoops`, `pCurrentSequence` are arrays of 8) are indexed by the slot
//! a group plays in; `0x14` and `0x15` stand for slots 1 and 4 in the
//! functions that take a slot. The type table (`SEQUENCE_TYPE_TABLE`, stride
//! 0x24) maps a group type to its slot and its category; the category picks
//! how `Update` steps the stages (`action`) of the slot's sequence.
//!
//! Conventions this file uses, so the next session finds them:
//!
//! - Tiny functions of other units that the game reaches by address are
//!   called by address through the constants below (`READ_WORD`,
//!   `NI_POINTER_*`, the list accessors). `00559450` is the one the linker
//!   folded into dozens of getters (`NiPointer::operator T*`,
//!   `NiTList::GetHeadPos`, `KFModel` and `NiTMapBase` field readers): it
//!   returns the word at the address in `ECX`.
//! - Functions of this unit that are translated here are called directly;
//!   the ones further on (`00498910`, `00496080`, `004994f0`, `00498670`,
//!   `0049bca0`, ...) by address until a later session translates them.
//! - The compiler's exception-unwinding frames and security cookies are
//!   not translated. A local the game keeps on its stack and passes by
//!   address is [`Engine::with_stack`].
//! - Stack arguments a callee declares but never reads (the second word of
//!   the `AnimSequenceBase` virtual functions) are parameters named
//!   `_unused_*`, so the uniform form accepts what the callers push.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleList, NiTPointerMap};
use crate::units::fallout_shared::extradataobjects::NiPoint3;

// ---- Callees outside this unit ---------------------------------------------

/// `operator new` (`00401000`), `operator delete` (`00401030`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `NiMemObject::operator new(size)`, the block comes back in EAX.
const NI_OPERATOR_NEW: u32 = 0x00aa_13e0;
/// Returns the word at the address in `ECX` (see the module comment).
const READ_WORD: u32 = 0x0055_9450;
/// `NiPointer::NiPointer(T*)`: stores the pointer and adds a reference.
const NI_POINTER_INIT: u32 = 0x0063_3c90;
/// `NiPointer::operator=(T*)`: releases the old, stores and adds the new.
const NI_POINTER_SET: u32 = 0x0066_b0d0;
/// `NiPointer::~NiPointer`: releases the pointed-to object.
const NI_POINTER_RELEASE: u32 = 0x0045_cec0;
/// `NiRefObject::IncRefCount` and `DecRefCount` (deletes at zero).
const REF_ADD: u32 = 0x0040_f6e0;
const REF_RELEASE: u32 = 0x0040_1970;
/// `NiObjectNET::GetName` (address of the `NiFixedString` at +8), and
/// `NiFixedString::operator const char*`.
const OBJECT_NAME: u32 = 0x0041_3f40;
const FIXED_STRING_CSTR: u32 = 0x0043_b1b0;
/// `NiFixedString::NiFixedString(const char*)` and its destructor.
const FIXED_STRING_INIT: u32 = 0x0043_8170;
const FIXED_STRING_FREE: u32 = 0x0043_81b0;

/// `strcpy_s(dst, size, src)` wrapper, `sprintf_s(dst, size, fmt, ...)`
/// wrapper, `strcpy`, `strcat`, `strchr`, `strrchr`, `_stricmp`.
const STRCPY_S: u32 = 0x0040_6d30;
const SPRINTF_S: u32 = 0x0040_6d00;
const STRCPY: u32 = 0x0040_46f0;
const STRCAT: u32 = 0x00ec_6380;
const STRCHR: u32 = 0x0043_b9d0;
const STRRCHR: u32 = 0x00ec_6e30;
const STRICMP: u32 = 0x0040_4dc0;
/// `memset(dst, value, size)` wrapper.
const MEMSET: u32 = 0x0040_3d30;
/// `_eh_vector_constructor_iterator_(ptr, size, count, ctor, dtor)` and
/// `_eh_vector_destructor_iterator_(ptr, size, count, dtor)`.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
/// The per-element constructor `Animation` hands to it (`NiPointer()` with
/// a null pointer).
const NI_POINTER_DEFAULT_CONSTRUCT: u32 = 0x0066_94e0;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;

/// `NiTList<T>` (the sequence list of `AnimSequenceMultiple`): first
/// position (`READ_WORD`), next position, address of a position's item,
/// element count, add at head, remove by item address, is empty, and the
/// scalar deleting destructor `NiTList::~NiTList(flags)`'s inner body.
const LIST_HEAD: u32 = READ_WORD;
const LIST_NEXT: u32 = 0x007b_52d0;
const LIST_ITEM_ADDRESS: u32 = 0x0063_17a0;
const LIST_COUNT: u32 = 0x0044_ddc0;
const LIST_ADD_HEAD: u32 = 0x0055_97d0;
const LIST_REMOVE: u32 = 0x0049_c0b0;
const LIST_IS_EMPTY: u32 = 0x0076_b610;
const LIST_DESTRUCT: u32 = 0x0054_da00;
/// The empty `NiTList` constructor (the linker folded it with an interface
/// manager constructor).
const LIST_CONSTRUCT: u32 = 0x0071_8d90;

/// `NiPoint3::NiPoint3()` (`006815c0`, shared with the `BSSimpleList`
/// item accessor: both return `ECX`).
const NI_POINT3_CONSTRUCT: u32 = 0x0068_15c0;
/// `KFModel` release `~Animation` runs on every model of `kfModelList`.
const KF_MODEL_RELEASE: u32 = 0x0043_bac0;
/// `TESObjectREFR::GetBaseForm`-style getter and the form-type getter
/// `fn_0048ffd0` runs on its reference (the maps name `007af430`
/// `BGSSaveFormBuffer::GetForm`, a folded name).
const REFERENCE_FORM: u32 = 0x007a_f430;
const FORM_TYPE: u32 = 0x0040_1170;
/// The form type `fn_0048ffd0` compares the reference's form with.
const FORM_TYPE_COMPARED: u32 = 0x2a;
/// `NiNode` helper `fn_0048ffd0` calls on the accumulation root with the
/// `NiRTTI` at `011a9448`.
const ACCUM_ROOT_SETUP: u32 = 0x0043_fa80;
const ACCUM_ROOT_SETUP_ARGUMENT: u32 = 0x011a_9448;
/// `ShouldQueueSequenceCloning`'s inputs: the player singleton pointer,
/// `PlayerCharacter::GetNode(first)` (`00950bb0`), the setting-value
/// address getter (`0043d4d0`) and the setting `011c5778` that turns
/// cloning on, `00822510` (the node accepts an object), and the other
/// flag objects.
const PLAYER_SINGLETON: u32 = 0x011d_ea3c;
const PLAYER_NODE: u32 = 0x0095_0bb0;
const SETTING_VALUE_ADDRESS: u32 = 0x0043_d4d0;
const SETTING_CLONING: u32 = 0x011c_5778;
const NODE_ACCEPTS_OBJECT: u32 = 0x0082_2510;
const FLAG_OBJECT_GLOBAL: u32 = 0x011d_df38;
const FLAG_OBJECT_CHECK: u32 = 0x0042_ce10;
const PLAYER_REFERENCE: u32 = 0x0047_b200;
const TES_GLOBAL: u32 = 0x011d_ea10;
const TES_CHECK: u32 = 0x0045_1530;
/// `fn_00491090`: begin and end time getters of the sequence.
const SEQUENCE_BEGIN_TIME: u32 = 0x0075_9450;
const SEQUENCE_END_TIME: u32 = 0x0050_8100;
/// `BSSimpleList<T>`: constructor, address of a node's item (the node
/// itself), next node, is empty, remove the head, remove all, destructor,
/// push back (pointer to the item), and the list's scalar deleting
/// destructor.
const SIMPLE_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const SIMPLE_LIST_ITEM: u32 = 0x0068_15c0;
const SIMPLE_LIST_NEXT: u32 = 0x0072_6070;
const SIMPLE_LIST_IS_EMPTY: u32 = 0x0082_56d0;
const SIMPLE_LIST_POP_FRONT: u32 = 0x0063_f7b0;
const SIMPLE_LIST_CLEAR: u32 = 0x0047_0470;
const SIMPLE_LIST_DESTRUCT: u32 = 0x0046_ffb0;
const SIMPLE_LIST_PUSH_BACK: u32 = 0x0090_5820;
const SIMPLE_LIST_DELETE: u32 = 0x0047_02f0;

/// `NiTMapBase<unsigned short, AnimSequenceBase*>` (Xbox PDB name of
/// `GetAt`): constructor with the hash size, `GetAt(key, &value)`,
/// `SetAt(key, value)`, `RemoveAt(key)`, first position, next
/// (`GetNext(&pos, &key, &value)`), and the clear-all.
const MAP_CONSTRUCT: u32 = 0x0049_c050;
const MAP_GET_AT: u32 = 0x0049_c390;
const MAP_SET_AT: u32 = 0x0049_c170;
const MAP_REMOVE_AT: u32 = 0x0049_c250;
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
const MAP_GET_NEXT: u32 = 0x0049_c410;
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;

/// The 4-byte scope guard the game keeps on its stack (sets the
/// memory-allocation tag to its first argument for its lifetime).
const SCOPE_GUARD_CTOR: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DTOR: u32 = 0x0040_4ee0;
const SCOPE_GUARD_SIZE: u32 = 4;
const SCOPE_GUARD_TAG: u32 = 0x33;
/// `"D:\\_Fallout3\\Platforms\\Common\\Code\\Fallout Shared\\Animation.cpp"`.
const SOURCE_FILE: u32 = 0x0101_d930;

/// The `ModelLoader` singleton pointer (`ECX` of the loader's methods).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// `ModelLoader::RemoveModel(name, 1)` (body of [`fn_0048eff0`]).
const MODEL_LOADER_REMOVE: u32 = 0x0044_5370;
/// `ModelLoader::LoadKF(name)`, `CancelReplacementKFList(animation)`.
const MODEL_LOADER_LOAD_KF: u32 = 0x0044_71c0;
const MODEL_LOADER_CANCEL_REPLACEMENT_KF_LIST: u32 = 0x0044_5430;
/// `KFModel` field getters: its sequence (+4) and its animation group (+8),
/// the second one adds a reference when the group is kept (`0043be10`).
const KF_MODEL_SEQUENCE: u32 = 0x007f_a950;
const KF_MODEL_ANIM_GROUP: u32 = 0x0055_85e0;
const KF_MODEL_ADD_REF: u32 = 0x0043_be10;

/// `ANIM_GROUP` helpers: the group's sequence type (`TESAnimGroup` +0x10
/// run through the type table), and the group id (`TESAnimGroup` +0x10).
const ANIM_GROUP_SEQUENCE_TYPE: u32 = 0x005f_2420;
const ANIM_GROUP_ID: u32 = 0x0047_d3b0;
/// The per-sequence-type table: 0x24 bytes per type, its first byte says
/// the type keeps several sequences (an `AnimSequenceMultiple`).
const SEQUENCE_TYPE_TABLE: u32 = 0x0119_77dc;
/// The 8 pointers to the names of the sound priority bones.
const SOUND_PRIORITY_BONE_NAMES: u32 = 0x0119_9bcc;

/// `NiControllerSequence` helpers: cycle type (+0x24), number of objects,
/// the name of object `i` (`GetObjectNameAt(i, buffer)`), and
/// `BSAnimGroupSequence::BSAnimGroupSequence(group, sequence)`.
const SEQUENCE_CYCLE_TYPE: u32 = 0x0059_bb30;
const SEQUENCE_OBJECT_COUNT: u32 = 0x0084_e3a0;
const SEQUENCE_OBJECT_NAME_AT: u32 = 0x00a3_07c0;
const ANIM_GROUP_SEQUENCE_CONSTRUCT: u32 = 0x004e_e9f0;
/// `NiRTTI` of `BSAnimGroupSequence` and `NiObject::IsKindOf` on it
/// (`__cdecl(rtti, object)`).
const RTTI_ANIM_GROUP_SEQUENCE: u32 = 0x011c_7d74;
const IS_KIND_OF: u32 = 0x0043_b300;
/// The root name passed to `NiControllerManager::AddSequence`.
const ACCUM_ROOT_NAME: u32 = 0x011f_49e0;

/// `NiControllerManager` methods: `NiControllerManager(root, cumulative)`,
/// `AddSequence(sequence, rootName, flag)`, `RemoveSequence(sequence)`,
/// `Deactivate(sequence, easeOut)`, a "reset" (`00a2e9a0`), the target
/// getter (`0055b980`), and the element accessors of its sequence
/// array/active set.
const MANAGER_CONSTRUCT: u32 = 0x00a2_efa0;
const MANAGER_ADD_SEQUENCE: u32 = 0x00a2_f0c0;
const MANAGER_REMOVE_SEQUENCE: u32 = 0x00a2_ec50;
const MANAGER_DEACTIVATE: u32 = 0x0047_b220;
const MANAGER_RESET: u32 = 0x00a2_e9a0;
const MANAGER_TARGET: u32 = 0x0055_b980;
const ACTIVE_SET_ELEMENT: u32 = 0x0062_9ab0;
const SEQUENCE_ARRAY_COUNT: u32 = 0x0065_8930;
const SEQUENCE_ARRAY_ELEMENT: u32 = 0x0087_7a30;
/// Root-node helper of `GetAccumRoot` (`005f5f80`, on the first sequence's
/// object).
const ACCUM_ROOT_OF_SEQUENCE: u32 = 0x005f_5f80;
/// `NiObjectNET::GetController(rtti)`, `RemoveController(controller)`, and
/// the two controller `NiRTTI`s `~Animation` removes.
const GET_CONTROLLER: u32 = 0x00a5_c570;
const REMOVE_CONTROLLER: u32 = 0x00a5_c480;
const RTTI_CONTROLLER_FIRST: u32 = 0x011f_36bc;
const RTTI_CONTROLLER_SECOND: u32 = 0x011f_36e4;
/// `NiNode` helpers used by `AddAnimation` and the initializer:
/// `SetFlag(flag, 2)` (`0043b370`), `FindObject(root, name, 1)`
/// (`__cdecl`), the node update `00a59c60(node, &updateData)` (it runs the
/// node's virtual function at +0xa4 with the record and 0, then a virtual
/// function at +0xfc of the object at +0x18), the update-data constructor
/// `NiUpdateData(time, updateControllers, parallelUpdate)` (`0043d410`, Xbox PDB layout), and
/// `NiControllerSequence::Activate(priority, flag, weight, easeIn, ...)`.
const NODE_SET_FLAG: u32 = 0x0043_b370;
const NODE_FIND_OBJECT: u32 = 0x00c4_b310;
const NODE_UPDATE: u32 = 0x00a5_9c60;
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
const SEQUENCE_ACTIVATE: u32 = 0x00a3_4f20;

/// Functions of this unit past the first 40, called by address:
/// `Animation::SpecialIdleFree(bool, bool)` and `AnimIdleFree(&idle)`.
const SPECIAL_IDLE_FREE: u32 = 0x0049_8910;
const ANIM_IDLE_FREE: u32 = 0x0049_8670;

/// `Interface::IsMenuIDVisible(id, 0)` and `Interface::IsInMenuMode`.
const IS_MENU_ID_VISIBLE: u32 = 0x0070_2680;
const IS_IN_MENU_MODE: u32 = 0x0070_2360;
const MENU_ID_PIPBOY_WAIT: u32 = 0x40c;
/// `printf`-style logger (`__cdecl(format, ...)`).
const LOG: u32 = 0x005b_5e40;
/// `MessageHandler::IncDisableWarningCount(flag)`.
const DISABLE_WARNINGS: u32 = 0x0043_b2b0;
/// Random number source (`MersenneTwister` wrapper) used for random picks.
const RANDOM: u32 = 0x0048_7f50;

// ---- Callees and data of the second session (`004910d0` onward) ---------------------
//
// Several of these small getters are bodies the linker folded, so the engine
// map's name for them is often another class's: each constant says what the
// body does and on which object it is called (`ECX`, the first argument in
// the uniform form).

/// The global time-scale object (an object at a fixed address, not a pointer
/// to one) and its getter: `ECX` = the object, the `float` at +0xc in ST0.
const TIME_SCALE_OBJECT: u32 = 0x011f_6394;
const TIME_SCALE_GET: u32 = 0x0084_d030;
/// Flag helpers of `Animation`: `00493830(animation)` is true while
/// `kfModelList` (+0x104) holds models; `00493880(animation, value, mask)`
/// sets (`value` != 0) or clears the bits of `mask` in the flag byte at +0.
const MODEL_QUEUE_NOT_EMPTY: u32 = 0x0049_3830;
const FLAG_SET: u32 = 0x0049_3880;
/// Flag bit 1: the movement was updated (`UpdateMovementNoWorldUpdate` set it
/// and `Update` clears it); bit 2: the scene graph update was postponed.
const FLAG_MOVEMENT_UPDATED: u32 = 1;
const FLAG_SCENE_GRAPH_PENDING: u32 = 2;
/// Number of queued models `Update` has added so far (the setting at
/// `SETTING_CLONING` bounds it).
const QUEUED_MODEL_COUNTER: u32 = 0x011c_56e8;
/// `BSSimpleList::remove`: removes the item whose address is in the cell
/// (`list.remove(&item)`), `RET 4`.
const SIMPLE_LIST_REMOVE_ITEM: u32 = 0x0090_5330;
/// `_ftol2_sse`: truncates the float in ST0 to an integer in EAX. The
/// uniform form has no ST0 argument, so the value is a leading `f64`.
const FTOL: u32 = 0x00ec_62c0;

/// Settings read through `SETTING_VALUE_ADDRESS` (`ECX` = the setting object,
/// the result is the address of its value): the limit
/// `UpdateSceneGraphNoController` compares with. `00403e20(setting)` is the
/// same for float settings: the address of the value (the object + 4), or of
/// a static zero for a null object.
const SETTING_SCENE_GRAPH_LIMIT: u32 = 0x011c_3ea4;
const SETTING_FLOAT_ADDRESS: u32 = 0x0040_3e20;
/// Float settings: the movement scale (`Update` multiplies the group's
/// movement by it, `StartGroup_ov2` divides the blend time by it), the blend
/// time `StartGroup_ov2` starts from and the one it uses in menus.
const SETTING_MOVEMENT_SCALE: u32 = 0x011c_5724;
const SETTING_BLEND_TIME: u32 = 0x011c_56fc;
const SETTING_BLEND_TIME_MENU: u32 = 0x011c_5740;
/// A global byte that makes `UpdateSceneGraphNoController` always update, a
/// global byte `Update` tests before it steps a sequence of category 9, and
/// the `VATS` object (its `0044ddc0` word and `009c71c0` current action).
const SCENE_GRAPH_ALWAYS_UPDATE: u32 = 0x011e_07a8;
const SEQUENCE_STEP_FLAG: u32 = 0x011e_0783;
const VATS_OBJECT: u32 = 0x011f_2250;
const VATS_CURRENT_ACTION: u32 = 0x009c_71c0;
/// The word at +8 of an object (`0044ddc0`), the word at +0xc (`0084e3a0`:
/// the object count of a sequence, the form ID of a reference).
const WORD_AT_8: u32 = 0x0044_ddc0;
const WORD_AT_C: u32 = 0x0084_e3a0;
/// `StartGroup_ov2` asks `0047c850(object)` (a constant `false` in this
/// build) of the object whose pointer is at `011de45c`; `00705a00` is
/// `Interface::IsInPipboyMenu`.
const PIPBOY_QUERY_OBJECT: u32 = 0x011d_e45c;
const PIPBOY_QUERY: u32 = 0x0047_c850;
const IS_IN_PIPBOY_MENU: u32 = 0x0070_5a00;
/// Two flag checks `Update` makes before it plays the notes of a step:
/// `00456610(node)` and `00709cb0()`.
const NODE_FLAG_CHECK: u32 = 0x0045_6610;
const SOUND_FLAG_CHECK: u32 = 0x0070_9cb0;

/// `Actor` accessors (`ECX` = the actor): the process object (`+0x68`),
/// `IsWeaponDrawn`, `GetAnimAction`, `SetAnimAction(action, sequence)`,
/// `SetHavokWeapon`, `GetAnimGroup(wanted, 0, 0, 0)`, the 16 flag bits
/// `008846e0` reads through the process, its "any of the low 4 bits"
/// test (`004938e0`), the process byte at +0x384 (`004938c0`), the word at
/// +0x108 (`004f8960`), `TESObjectREFR::GetScale` (ST0) and the flag-word
/// setter `008b0140(actor, value, mask)` (+0x11c).
const ACTOR_PROCESS: u32 = 0x008d_8520;
const ACTOR_IS_WEAPON_DRAWN: u32 = 0x008a_16d0;
const ACTOR_GET_ANIM_ACTION: u32 = 0x008a_7570;
const ACTOR_SET_ANIM_ACTION: u32 = 0x008a_73e0;
const ACTOR_SET_HAVOK_WEAPON: u32 = 0x008a_5eb0;
const ACTOR_GET_ANIM_GROUP: u32 = 0x0089_7910;
const ACTOR_FLAGS_WORD: u32 = 0x0088_46e0;
const ACTOR_FLAGS_ANY: u32 = 0x0049_38e0;
const PROCESS_FLAG_BYTE: u32 = 0x0049_38c0;
const ACTOR_WORD_108: u32 = 0x004f_8960;
const REFERENCE_SCALE: u32 = 0x0056_7400;
const ACTOR_FLAG_SET: u32 = 0x008b_0140;
/// `PlayerCharacter::GetAnimation(first)`.
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;

/// `BSAnimGroupSequence` accessors (`ECX` = the sequence): the elapsed time
/// `00493770` (ST0, a stored `float`), `SetPhase(phase, flag)`,
/// `GetScaledTime(time)`, the state word at +0x44 (`008041a0`; the map calls
/// it `LowProcess::GetGenericLocation`), `GetPriority(boneName, 0)`.
const SEQUENCE_ELAPSED: u32 = 0x0049_3770;
const SEQUENCE_SET_PHASE: u32 = 0x00a3_28b0;
const SEQUENCE_SCALED_TIME: u32 = 0x004e_ec60;
const SEQUENCE_STATE: u32 = 0x0080_41a0;
const SEQUENCE_PRIORITY: u32 = 0x004e_f9a0;
/// `00493800(animation, sequence)`: the sequence's float at +0x48 plus the
/// animation's `time`, a `float` in ST0 (the engine map names the body
/// `TESAnimGroup::IsJumpingLoopAnim`, a folded name).
const SEQUENCE_TIME_ON_ANIMATION: u32 = 0x0049_3800;
/// `004937a0(sequence, index)`: the pointer in the 16-byte entry `index` of
/// the array at +0x14 of the sequence.
const SEQUENCE_INTERPOLATOR_AT: u32 = 0x0049_37a0;
/// `00a30c80(from, to)`: whether the two sequences can be morphed.
const SEQUENCE_MORPH_COMPATIBLE: u32 = 0x00a3_0c80;
/// The sequence manager's entry points (`ECX` = the manager, which the
/// first one ignores): `0047aab0(sequence, priority, flag, weight, ease,
/// 0)` is `NiControllerSequence::Activate` (`SEQUENCE_ACTIVATE`) with a
/// trailing 0; `00a2e1b0`, `00a2e280` and `00a2f800` are the morph,
/// cross-fade and blend-in transitions `StartGroup_ov2` chooses from.
const MANAGER_ACTIVATE: u32 = 0x0047_aab0;
const MANAGER_MORPH_FADE: u32 = 0x00a2_e1b0;
const MANAGER_CROSS_FADE: u32 = 0x00a2_e280;
const MANAGER_BLEND_IN: u32 = 0x00a2_f800;

/// `TESAnimGroup` functions that take the 16-bit group id on the stack
/// (`__cdecl`): `GetType` (the low byte), `GetWeapon` (bits 8..11),
/// `IsAimAction`, `IsAttackAction`, `IsIronSightsAction`.
const GROUP_ID_TYPE: u32 = 0x005f_2440;
const GROUP_ID_WEAPON: u32 = 0x005f_2400;
const GROUP_ID_IS_AIM: u32 = 0x005f_2630;
const GROUP_ID_IS_ATTACK: u32 = 0x005f_2540;
const GROUP_ID_IS_IRON_SIGHTS: u32 = 0x005f_2720;
/// `TESAnimGroup` methods (`ECX` = the group): `GetTime(index)` (ST0),
/// the move type and the weapon type of its own id, the movement vector
/// getter `005f4c90(out)` and setter `005f4c40(in)`, the speed `005f4c70`
/// (ST0), the "type is 0xe4 or 0xec..0xef" test `005f4d60`, the notes step
/// `005f2b60(actor, from, to, sequence)` and the sound-priority byte
/// `00508d90(bone index)`.
const GROUP_TIME: u32 = 0x005f_3780;
const GROUP_MOVE_TYPE: u32 = 0x005f_23a0;
const GROUP_WEAPON_TYPE: u32 = 0x005f_23e0;
const GROUP_MOVEMENT_VECTOR: u32 = 0x005f_4c90;
const GROUP_SET_MOVEMENT_VECTOR: u32 = 0x005f_4c40;
const GROUP_SPEED: u32 = 0x005f_4c70;
const GROUP_IS_SPECIAL_TYPE: u32 = 0x005f_4d60;
const GROUP_PLAY_NOTES: u32 = 0x005f_2b60;
const GROUP_BONE_PRIORITY: u32 = 0x0050_8d90;
/// Columns of the type table the group type indexes (stride 0x24, base
/// `SEQUENCE_TYPE_TABLE`): +4 the slot the group plays in, +8 its
/// category; the entry's name pointer is at -4. The weapon and move name
/// tables.
const SEQUENCE_TYPE_SLOT: u32 = 0x0119_77e0;
const SEQUENCE_TYPE_CATEGORY: u32 = 0x0119_77e4;
const SEQUENCE_TYPE_NAME: u32 = 0x0119_77d8;
const WEAPON_NAME_TABLE: u32 = 0x0119_77a4;
const MOVE_NAME_TABLE: u32 = 0x0119_7794;

/// Vector helpers (`ECX` = the vector, both return `out`): `out = this *
/// scale` (`0045bb20(out, scale)`) and `out = this - other`
/// (`00439ef0(out, other)`); `006815c0` returns its `ECX` (the address of the
/// first three floats of a transform record).
const VECTOR_SCALE: u32 = 0x0045_bb20;
const VECTOR_SUBTRACT: u32 = 0x0043_9ef0;
const RECORD_ADDRESS: u32 = 0x0068_15c0;
/// `NiMatrix3::TransformVertices(matrix, translate, count, in, out)`
/// (`__cdecl`).
const MATRIX_TRANSFORM_VERTICES: u32 = 0x00a5_82f0;
/// The world rotation matrix of a node (`006a9540` returns node + 0x34).
const NODE_ROTATION: u32 = 0x006a_9540;
/// Dynamic cast `00653270(rtti, object)` (`__cdecl`, 0 for a null object) and
/// the `NiRTTI` `Update` casts to; the controller `NiRTTI` it asks the
/// accumulation root for; the controller-list setter `00a5c000(object, list)`
/// and getter `0043b230` (the word at +0xc).
const DYNAMIC_CAST: u32 = 0x0065_3270;
const RTTI_CAST_TARGET: u32 = 0x011f_36fc;
const RTTI_ACCUM_CONTROLLER: u32 = 0x011f_36ec;
const OBJECT_SET_CONTROLLERS: u32 = 0x00a5_c000;
const OBJECT_CONTROLLERS: u32 = 0x0043_b230;
/// The skin getter `0043fad0(node)` (the word at +0xbc), the child count
/// `0043b480` and the child accessor `0043b4a0(index)` of a node.
const NODE_SKIN: u32 = 0x0043_fad0;
const NODE_CHILD_COUNT: u32 = 0x0043_b480;
const NODE_CHILD_AT: u32 = 0x0043_b4a0;
/// The world translation of a node (`0043c490` returns node + 0x58), the
/// biped update `004f0040(root, time)` and `UpdateBipOnly(time, &out,
/// flag)` of `Animation` (`0049bca0`).
const NODE_WORLD_TRANSLATION: u32 = 0x0043_c490;
const BIP_UPDATE_ALL_BUT_BIP: u32 = 0x004f_0040;
const UPDATE_BIP_ONLY: u32 = 0x0049_bca0;
/// The shader accumulator (`00b4f5c0`) and its test
/// `00b63680(accumulator, id, 0)`.
const SHADER_ACCUMULATOR: u32 = 0x00b4_f5c0;
const SHADER_ACCUMULATOR_TEST: u32 = 0x00b6_3680;
/// Functions of this unit after the second session's range, called by
/// address: `ClearGroup(slot, blend)`, `BlendOut(slot, flag)`,
/// `SpecialIdleWorking(sequence)`, the idle state step `00497040(idle,
/// animation)` and the controller step `00496280`.
const CLEAR_GROUP: u32 = 0x0049_6080;
const BLEND_OUT: u32 = 0x0049_94f0;
const SPECIAL_IDLE_WORKING: u32 = 0x0049_8ea0;
const IDLE_STATE_STEP: u32 = 0x0049_7040;
const RESET_CONTROLLERS: u32 = 0x0049_6280;
/// `00403550(idle, value)` stores a word at +8; `00c75b40(object)` and
/// `00974d90(group)` (the signed byte at +0x28) belong to the checks
/// `StartGroup_ov2` makes, as does `0045cd60(process)` (the word at +0x28).
const IDLE_SET_WORD_8: u32 = 0x0040_3550;
const RAGDOLL_REFRESH: u32 = 0x00c7_5b40;
const GROUP_BYTE_28: u32 = 0x0097_4d90;
const PROCESS_WORD_28: u32 = 0x0045_cd60;
/// Format strings of the log messages (`.rdata`).
const LOG_IDLE_FREE_ANIMATING: u32 = 0x0101_da98;
const LOG_NO_ACCUM_ROOT: u32 = 0x0101_db48;
const LOG_ANIMATE_IN_PLACE: u32 = 0x0101_daf0;
const LOG_MORPH_TAGS: u32 = 0x0101_dc58;
const LOG_MORPH_CONTROLLERS: u32 = 0x0101_dbe8;
const LOG_MORPH_SELF: u32 = 0x0101_db90;
/// Float constants read from memory: the doubles `0.0`, `-1.0`, `30.0`,
/// `4.0`, `0.5`, `0.01` and `1e-5`; the floats `0.5` and `FLT_MAX`.
const DOUBLE_ZERO: u32 = 0x0101_2060;
const DOUBLE_MINUS_ONE: u32 = 0x0101_a6b0;
const DOUBLE_THIRTY: u32 = 0x0101_db88;
const DOUBLE_FOUR: u32 = 0x0101_db80;
const DOUBLE_HALF: u32 = 0x0101_1588;
const DOUBLE_HUNDREDTH: u32 = 0x0101_6408;
const DOUBLE_MICRO: u32 = 0x0101_dae8;
const FLOAT_HALF: u32 = 0x0101_6248;
const FLOAT_MAX: u32 = 0x0101_d890;
/// The words `fn_00494260` copies into a transform record: three from
/// `011a8400`, four from `011f3704` and the `float` at `01096b8c`.
const RECORD_DEFAULT_TRANSLATE: u32 = 0x011a_8400;
const RECORD_DEFAULT_ROTATE: u32 = 0x011f_3704;
const RECORD_DEFAULT_SCALE: u32 = 0x0109_6b8c;
/// `00408d60(object)` on the setting object at `01267c30`: the address of
/// its byte value.
const SETTING_BYTE_OBJECT: u32 = 0x0126_7c30;
const SETTING_BYTE_ADDRESS: u32 = 0x0040_8d60;
/// `004937c0(group)` and `004937e0(group)`: `IsAimAction` and
/// `IsAttackAction` of the group's own id (the word at +0x10).
const GROUP_IS_AIM_OF: u32 = 0x0049_37c0;
const GROUP_IS_ATTACK_OF: u32 = 0x0049_37e0;
/// `006d2c40(movement)`: the `float` at +0xb4 of an actor's movement object
/// (ST0).
const MOVEMENT_SPEED: u32 = 0x006d_2c40;
/// The virtual slot of an actor that says whether it has a process.
const SLOT_HAS_PROCESS: u32 = 0x100;

// ---- Callees and data of the third session (`00495e40` onward) -----------------------
//
// As before, many of these getters are bodies the linker folded together, so
// each constant says what the body does and on which object it is called.

/// `"a:"`, the text key prefix `fn_00495e40` looks for in the sequence of
/// slot 4.
const TEXT_KEY_PREFIX_A: u32 = 0x0101_dcc0;
/// The text keys of a sequence: `00700300(sequence)` returns the word at
/// +0x20. `fn_00495f30` reads them as `{count at +0xc, entries at +0x10}`
/// with 8-byte entries `{time: f32, text: NiFixedString}`: `00717e50(entry)`
/// is the address of the text (`entry + 4`), `006a7f50(entry)` the `float` at
/// `entry` (ST0), and `00598040(sequence)` the `float` at +0x3c of a
/// sequence (ST0). The maps name `00717e50` after another class.
const SEQUENCE_TEXT_KEYS: u32 = 0x0070_0300;
const TEXT_KEY_TEXT: u32 = 0x0071_7e50;
const TEXT_KEY_TIME: u32 = 0x006a_7f50;
const SEQUENCE_FLOAT_3C: u32 = 0x0059_8040;
/// `strlen`'s wrapper (`__cdecl(string)`), `_strnicmp(a, b, count)` and
/// `tolower(char)` (all `__cdecl`).
const STRING_LENGTH: u32 = 0x0044_a670;
const STRNICMP: u32 = 0x00ec_7ec0;
const TOLOWER: u32 = 0x00ec_67aa;

/// `Interface::GetMenuModeType` (`__cdecl()`): 1 while a menu that pauses the
/// game is open (the maps give it another name).
const MENU_MODE_TYPE: u32 = 0x0070_2640;
/// The word at +0x58 of a sequence (`006286d0`; `ClearGroup` deactivates it
/// together with the sequence).
const SEQUENCE_WORD_58: u32 = 0x0062_86d0;
/// `fn_00496280`'s node helper `00440460(node, &vector)`, which copies three
/// words into +0x58; the next controller of a controller list
/// (`004a8a90(controller)`, the word at +0x30); the two `NiRTTI`s it casts a
/// controller and the controller's target to; the virtual slot that returns
/// the controller's target.
const NODE_SET_WORLD_TRANSLATION: u32 = 0x0044_0460;
const CONTROLLER_NEXT: u32 = 0x004a_8a90;
const RTTI_CONTROLLER_KIND: u32 = 0x011f_3714;
const RTTI_CONTROLLER_TARGET_KIND: u32 = 0x011f_371c;
const SLOT_CONTROLLER_TARGET: u32 = 0xc4;
/// The quaternion `(1, 0, 0, 0)` that `fn_00496340` resets a transform to.
const QUATERNION_IDENTITY: u32 = 0x011a_9ea4;
/// `00c81b90(object, 0)` (`__cdecl`): a `float` in ST0, 1.0 when the object
/// has none; `009611e0(object)` the word at +0x18 (its parent).
const OBJECT_VALUE: u32 = 0x00c8_1b90;
const OBJECT_PARENT: u32 = 0x0096_11e0;
/// `NiMemObject::operator delete(block, size)` (`__cdecl`).
const NI_OPERATOR_DELETE: u32 = 0x00aa_1460;
/// `NiRefObject`'s vtable and object counter (`ms_uiObjects`), the vtable of
/// `AnimIdle`, and the `InterlockedIncrement` and `InterlockedDecrement`
/// wrappers (`__cdecl(address)`) that count the objects.
const VTABLE_NI_REF_OBJECT: u32 = 0x0101_dce4;
const VTABLE_ANIM_IDLE: u32 = 0x0101_dcd8;
const NI_REF_OBJECT_COUNT: u32 = 0x011f_4400;
const INTERLOCKED_INCREMENT: u32 = 0x0040_b460;
const INTERLOCKED_DECREMENT: u32 = 0x0040_19a0;
/// `NiPointer<KFModel>`: `(T*)` constructor, `operator=(T*)` and destructor
/// (they count the model's own reference at +0x10).
const KF_MODEL_POINTER_INIT: u32 = 0x0044_afe0;
const KF_MODEL_POINTER_SET: u32 = 0x0044_b070;
const KF_MODEL_POINTER_RELEASE: u32 = 0x0044_b030;
/// `NiPointer<T>::operator=(const NiPointer&)` (`ECX` = destination, argument
/// = the address of the source `NiPointer`).
const NI_POINTER_COPY: u32 = 0x006e_5cc0;
/// The 8-byte string local `AnimIdle`'s constructor builds the model path in:
/// constructor, destructor and `format(string, format, ...)` (`__cdecl`); the
/// format `"%s\\%s"` and the folder `"Meshes"`.
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
const STRING_FORMAT: u32 = 0x0040_6f60;
const STRING_LOCAL_SIZE: u32 = 8;
const PATH_FORMAT: u32 = 0x0101_dcc4;
const MESHES_FOLDER: u32 = 0x0101_dccc;
/// The data handler singleton pointer and its `0046fd50(idleForm, index)`
/// (the animation object of an idle form); `BipedAnim::LoadAndAttachAddOn`
/// (`__cdecl(object, node, -1, actor, 0)`).
const DATA_HANDLER: u32 = 0x011c_3f2c;
const IDLE_FORM_ANIM_OBJECT: u32 = 0x0046_fd50;
const LOAD_AND_ATTACH_ADD_ON: u32 = 0x004a_eed0;
/// `ModelLoader` methods (`ECX` = the loader): `00445200(path, idle, 0,
/// actor)` loads the KF of an idle; `0045a5e0(name)` and `004454d0(idle)` drop
/// a model and an idle.
const MODEL_LOADER_LOAD_IDLE_KF: u32 = 0x0044_5200;
const MODEL_LOADER_DROP_MODEL: u32 = 0x0045_a5e0;
const MODEL_LOADER_DROP_IDLE: u32 = 0x0044_54d0;
/// `RecurseAndAddObjectsToPalette(object, palette)` and
/// `RecurseAndRemoveObjectsFromPalette(object, palette)` (`__cdecl`), and the
/// palette of a controller manager (`00537bd0(manager)`).
const PALETTE_ADD: u32 = 0x00a6_e870;
const PALETTE_REMOVE: u32 = 0x00a6_e8e0;
const MANAGER_PALETTE: u32 = 0x0053_7bd0;
/// The virtual slot of an actor that returns its `Animation`, the one of a
/// form's name sub-object (`form + 0x18`) that returns the model name, and
/// the one of a form-type description that returns its name.
const SLOT_ACTOR_ANIMATION: u32 = 0x1e4;
const SLOT_MODEL_NAME: u32 = 0x14;
const SLOT_FORM_TYPE_NAME: u32 = 0x130;
/// Task queue and shadow scene: `008c7aa0()` (true while the task queue takes
/// the work), the queue singleton getter `004537b0()`, its `0087ad00(object,
/// 0)` and `0087ac60(object)`; the `ShadowSceneNode` getter `00450b80(index)`
/// and its `RemoveObject(object)` (`00b5b1c0`).
const TASK_QUEUE_ACTIVE: u32 = 0x008c_7aa0;
const TASK_QUEUE: u32 = 0x0045_37b0;
const TASK_QUEUE_ATTACH: u32 = 0x0087_ad00;
const TASK_QUEUE_DETACH: u32 = 0x0087_ac60;
const SHADOW_SCENE_NODE: u32 = 0x0045_0b80;
const SHADOW_SCENE_NODE_REMOVE: u32 = 0x00b5_b1c0;
/// `fn_00496a50`'s helpers: the sequence step `004eecb0(sequence)`, the
/// animation's step `0049bd90(animation, object)`, and the process slot that
/// returns the object an actor holds.
const SEQUENCE_RELEASE_HELPER: u32 = 0x004e_ecb0;
const ANIMATION_ADD_ON_REMOVED: u32 = 0x0049_bd90;
const SLOT_PROCESS_HELD_OBJECT: u32 = 0x3e8;
/// Functions of this unit after the third session's range: the idle replace
/// `00498170(animation)` and the idle check `00498290(animation)`.
const SPECIAL_IDLE_REPLACE_OV2: u32 = 0x0049_8170;
const SPECIAL_IDLE_CHECK: u32 = 0x0049_8290;
/// The virtual slot of a process that takes the idle form `fn_00497f20` hands
/// it.
const SLOT_PROCESS_SET_IDLE: u32 = 0x390;

/// The save/load game object (a pointer at `011de45c`) and its methods
/// (`ECX` = the object): `008579b0(ptr, size)` writes bytes, `008579e0(ptr,
/// size)` reads them, `00857bd0(count)` skips `count` bytes, `00825c00()`
/// returns the buffer position (the word at +0x14), `00862110()` is true while
/// the game saves in blocks, `004fd3e0()` and `004fd3c0()` return the record
/// being written and read, `008df040()` the byte at +0x80, `00857a10(ptr,
/// size)` saves and `00857aa0(ptr, size)` loads a form ID (the load returns a
/// byte).
const SAVE_GAME_OBJECT: u32 = 0x011d_e45c;
const SAVE_WRITE: u32 = 0x0085_79b0;
const SAVE_READ: u32 = 0x0085_79e0;
const SAVE_SKIP: u32 = 0x0085_7bd0;
const SAVE_POSITION: u32 = WORD_AT_14;
const SAVE_USES_BLOCKS: u32 = 0x0086_2110;
const SAVE_RECORD_WRITTEN: u32 = 0x004f_d3e0;
const SAVE_RECORD_READ: u32 = 0x004f_d3c0;
const SAVE_BYTE_80: u32 = 0x008d_f040;
const SAVE_FORM_ID: u32 = 0x0085_7a10;
const LOAD_FORM_ID: u32 = 0x0085_7aa0;
/// The setting object at `011de4e8` (`00408d60` gives the address of its
/// byte value) that turns the save size diagnostics on.
const SAVE_DIAGNOSTICS_OBJECT: u32 = 0x011d_e4e8;
/// The tag that opens a save block (`"KOLB"` as a little-endian word).
const SAVE_BLOCK_TAG: u32 = 0x424c_4f4b;
/// `TESForm` lookup by form ID (`__cdecl(id)`), `__RTDynamicCast(object, 0,
/// source, target, 0)` with the type descriptors of `TESForm` and
/// `TESIdleForm` that `fn_004977e0` casts between, and `Error(format, ...)`
/// (`__cdecl`).
const FORM_BY_ID: u32 = 0x0048_39c0;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_DESCRIPTOR_FORM: u32 = 0x0118_3028;
const TYPE_DESCRIPTOR_IDLE_FORM: u32 = 0x0118_6a18;
const ERROR_LOG: u32 = 0x0040_fbe0;
/// `AnimIdle` helpers: the idle form of an idle (`0055b980`, the word at
/// +0x2c), the save size (`004efb10(sequence)`), save (`004efb20(sequence,
/// time)`) and load (`004efbc0(sequence, time)`) of a sequence, and the
/// buffer methods of `BGSSaveGameBuffer` (`00865e50(ptr, 4, 0)`) and
/// `BGSLoadGameBuffer` (`00864980(ptr, 4)`).
const IDLE_FORM_OF: u32 = 0x0055_b980;
const SEQUENCE_SAVE_SIZE: u32 = 0x004e_fb10;
const SEQUENCE_SAVE: u32 = 0x004e_fb20;
const SEQUENCE_LOAD: u32 = 0x004e_fbc0;
const SAVE_BUFFER_FIELD: u32 = 0x0086_5e50;
const LOAD_BUFFER_FIELD: u32 = 0x0086_4980;
/// Format strings of the save and load messages (`.rdata`).
const LOG_SAVE_SIZE_FORM: u32 = 0x0101_2cb0;
const LOG_SAVE_SIZE: u32 = 0x0101_2c78;
const LOG_SAVE_GAME_FORM: u32 = 0x0101_53a0;
const LOG_SAVE_GAME: u32 = 0x0101_536c;
const LOG_SAVE_BLOCK_TOO_BIG: u32 = 0x0101_5318;
const LOG_LOAD_NO_BLOCK_FORM: u32 = 0x0101_5718;
const LOG_LOAD_NO_BLOCK: u32 = 0x0101_56a8;
const LOG_LOAD_UNKNOWN_IDLE: u32 = 0x0101_dcec;
const LOG_LOAD_LONG_FORM: u32 = 0x0101_5588;
const LOG_LOAD_SHORT_FORM: u32 = 0x0101_5500;
const LOG_LOAD_LONG: u32 = 0x0101_54a0;
const LOG_LOAD_SHORT: u32 = 0x0101_5440;
const LOG_QUEUE_JUST_STARTED: u32 = 0x0101_dd28;
/// `004f5d90(quaternion, value)`: stores `value` at +4 of the quaternion. The
/// virtual slot of a node that removes a child. `00825c00(object)` is the
/// word at +0x14 (the section of an idle, the position of the save buffer),
/// and `004efaa0()` the constant size, a 16-bit value, a sequence's save
/// starts from.
const QUATERNION_STORE_AT_4: u32 = 0x004f_5d90;
const SLOT_DETACH_CHILD: u32 = 0xe8;
const WORD_AT_14: u32 = 0x0082_5c00;
const SEQUENCE_SAVE_BASE_SIZE: u32 = 0x004e_faa0;

// ---- Callees and data of the fourth session (`00498030` onward) ----------------------
//
// Several of these bodies are folded getters and setters, so each constant says
// what the body does and on which object it is called (`ECX`, the first argument
// in the uniform form).

/// `006fa820(idle)`: the word at +0x10 of an idle (`spKFModel`, the pointer a
/// `NiPointer<KFModel>` holds). `008d7dc0(idle, actor)` stores `actor` at +0x34
/// of the idle (`pActor`); the maps name it `LowProcess::SetItemBeingUsed`, a
/// folded name.
const IDLE_KF_MODEL: u32 = 0x006f_a820;
const IDLE_SET_ACTOR: u32 = 0x008d_7dc0;
/// Two getters of an idle form: `005ff770(form)` is a byte computed from the
/// bytes at +0x39 and +0x3a, `004d8a00(form)` the 16-bit word at +0x3c.
const IDLE_FORM_BYTE: u32 = 0x005f_f770;
const IDLE_FORM_WORD: u32 = 0x004d_8a00;
/// The byte at +0x61d of the data handler (`004226e0`, `ECX` = the object in
/// `DATA_HANDLER`).
const DATA_HANDLER_FLAG: u32 = 0x0042_26e0;
/// `Actor` method `008b28c0(actor, type, animation)`.
const ACTOR_START_ANIMATION: u32 = 0x008b_28c0;
/// `TESAnimGroup::IsIronSightsAction_ov2` (Xbox PDB), `__cdecl(group type)`.
const GROUP_TYPE_IS_IRON_SIGHTS: u32 = 0x005f_2750;
/// The virtual slots of an actor's process that `LoadGame`, `SaveGame` and
/// `BlendOut` use (the slot numbers are the byte offsets into the vtable; the
/// bodies are not named by the maps), and of the actor itself.
const SLOT_PROCESS_RELOAD: u32 = 0x4dc;
const SLOT_PROCESS_BYTE: u32 = 0x454;
const SLOT_PROCESS_SET_BIPED: u32 = 0x1cc;
const SLOT_PROCESS_NOTIFY: u32 = 0x1d8;
const SLOT_PROCESS_REFRESH: u32 = 0x61c;
const SLOT_PROCESS_SET_HELD: u32 = 0x3ec;
const SLOT_PROCESS_HELD_PHASE: u32 = 0x3e4;
const SLOT_ACTOR_FLAG_22C: u32 = 0x22c;
const SLOT_ACTOR_FLAG_2E8: u32 = 0x2e8;
const SLOT_ACTOR_BIPED: u32 = 0x1e8;
/// The sub-object at +0x88 of an actor and its virtual slot 0x34, which the
/// save and load functions ask before they look for the controller sequence.
const ACTOR_SUB_OBJECT: u32 = 0x88;
const SLOT_SUB_OBJECT_CHECK: u32 = 0x34;
/// Virtual slots of the save game buffers: slot 8 (`BGSSaveGameBuffer`) and
/// slot 0xc (`BGSLoadGameBuffer`) give the actor the buffer belongs to.
const SLOT_SAVE_BUFFER_OWNER: u32 = 0x8;
const SLOT_LOAD_BUFFER_OWNER: u32 = 0xc;

/// `PlayerCharacter::GetBiped` (Xbox PDB): `00950b00(player, first, animation,
/// player)`.
const PLAYER_BIPED: u32 = 0x0095_0b00;

/// Controller and node helpers: `00a6e960(object)` (the object the manager's
/// palette is cast to with `RTTI_PALETTE_KIND`), `00a30160(controller)` (a
/// `NiMultiTargetTransformController` method, per the map), the virtual slot
/// 0xc8 of the controller kind `ClearControllersInterpolators` casts to, and
/// `0045bc00(object, index)` (the pointer at `index` of the array at +0x9c).
const RTTI_PALETTE_KIND: u32 = 0x011f_4a04;
const PALETTE_OBJECT_RESET: u32 = 0x00a6_e960;
const CONTROLLER_RESET: u32 = 0x00a3_0160;
const SLOT_CONTROLLER_CLEAR: u32 = 0xc8;
const NODE_ARRAY_ELEMENT: u32 = 0x0045_bc00;
const SLOT_NODE_AS_NODE: u32 = 0xc;
/// Sequence helpers (`ECX` = the `BSAnimGroupSequence`): `004efa50` clears the
/// interpolator pointers (+8 of every 16-byte entry of the array at +0x14),
/// `004eedd0(sequence, root, count)` and `004eee00(sequence)` re-aim a
/// sequence, `00a32c70(sequence, root)` attaches it (`NiControllerSequence`
/// per the map), `004efc90(sequence, buffer, time)` saves it into a
/// `BGSSaveGameBuffer`, `004efd10(sequence, buffer, time)` loads it from a
/// `BGSLoadGameBuffer`, `004eeb00` is `BSAnimGroupSequence::~BSAnimGroupSequence`
/// (Xbox PDB) and `00a34ba0(sequence, time, flag)` is
/// `NiControllerSequence::Update` (Xbox PDB).
const SEQUENCE_CLEAR_INTERPOLATORS: u32 = 0x004e_fa50;
const SEQUENCE_RETARGET: u32 = 0x004e_edd0;
const SEQUENCE_PREPARE: u32 = 0x004e_ee00;
const SEQUENCE_ATTACH: u32 = 0x00a3_2c70;
const SEQUENCE_SAVE_TO_BUFFER: u32 = 0x004e_fc90;
const SEQUENCE_LOAD_FROM_BUFFER: u32 = 0x004e_fd10;
const SEQUENCE_DESTRUCT: u32 = 0x004e_eb00;
const SEQUENCE_UPDATE: u32 = 0x00a3_4ba0;
/// The `NiRTTI` the save and load functions cast an object's controller to
/// (`0043b230` gives the controller), `0047a520(object, key)` looks up the
/// sequence in it, and the two keys come from the globals read by
/// `fn_00499b70` and `fn_00499b80`.
const RTTI_SAVED_CONTROLLER: u32 = 0x011f_36ac;
const SEQUENCE_LOOKUP: u32 = 0x0047_a520;
const SAVED_SEQUENCE_KEY_GLOBAL: u32 = 0x011c_61bc;
const SAVED_SEQUENCE_LOOKUP_GLOBAL: u32 = 0x011c_61c0;
/// Float accessors: `00621b00(object)` reads and `00621b20(object, value)`
/// writes the `float` at +0x10, `00639aa0(sequence)` and `0098adb0(sequence,
/// value)` the one at +0x48, `0081c470(light, 1, value)` is
/// `MagicLight::Fade` (Xbox PDB).
const FLOAT_AT_10: u32 = 0x0062_1b00;
const SET_FLOAT_AT_10: u32 = 0x0062_1b20;
const FLOAT_AT_48: u32 = 0x0063_9aa0;
const SET_FLOAT_AT_48: u32 = 0x0098_adb0;
const LIGHT_FADE: u32 = 0x0081_c470;
/// `0047aa40(manager, flag)`: a method of the controller manager that takes a
/// byte (it calls `0047aa60(manager, flag, 8)`).
const MANAGER_SET_FLAG: u32 = 0x0047_aa40;
/// `00726070(object)`: the word at +4 (the next node of a `BSSimpleList`, or a
/// field of the actor's sub-object).
const WORD_AT_4: u32 = 0x0072_6070;
/// `BSBipNode::UpdateBipOnly` (Xbox PDB): `004efe80(root, time, accumRoot)`, and
/// `004f05c0(root, object)` (a method of the animation root).
const BIP_NODE_UPDATE_BIP_ONLY: u32 = 0x004e_fe80;
const ROOT_NODE_NOTIFY: u32 = 0x004f_05c0;
/// `bhkUtilFunctions` (Xbox PDB): `FindNextCollisionObject(node)` and
/// `SetCOUseVel(flag, node)` (`__cdecl`), and `0047b470(object)`, whether a
/// collision object is flagged.
const FIND_NEXT_COLLISION_OBJECT: u32 = 0x00c8_02d0;
const COLLISION_OBJECT_FLAGGED: u32 = 0x0047_b470;
const SET_CO_USE_VEL: u32 = 0x00c8_0330;
/// Save buffer methods (`ECX` = the `BGSSaveGameBuffer` or `BGSLoadGameBuffer`):
/// `SaveFormID_ov2(formID, 0)`, `StartVariableSizedValue()`,
/// `SaveVariableSizedValue_ov2(count, start)`, `LoadVariableSizedValue()`,
/// `LoadFormID()` (reads an ID), `LoadFormID_ov2(&id)` (a byte, set when the ID
/// is missing from the save) and the replay delay list push `005ae3d0(list,
/// &item)`.
const SAVE_BUFFER_FORM_ID: u32 = 0x0086_5df0;
const SAVE_BUFFER_START_VARIABLE: u32 = 0x0086_5f20;
const SAVE_BUFFER_END_VARIABLE: u32 = 0x0086_5ff0;
const LOAD_BUFFER_VARIABLE: u32 = 0x0086_4a60;
const LOAD_BUFFER_FORM_ID: u32 = 0x0086_48a0;
const LOAD_BUFFER_FORM_ID_FOUND: u32 = 0x0086_48e0;
const REPLAY_LIST_PUSH: u32 = 0x005a_e3d0;
/// `0085f010(save game object, pointer, count)`, which `fn_0049ab00` runs for
/// the bytes it reads.
const SAVE_GAME_CONSUME: u32 = 0x0085_f010;
/// The base constructor of `NiTPointerMap` (`0049c100(map, hashSize)`) and the
/// map's vtable; the base constructor of `NiControllerSequence`
/// (`00a316a0`) and the vtable of `BSAnimGroupSequence`.
const MAP_BASE_CONSTRUCT: u32 = 0x0049_c100;
const VTABLE_ANIM_SEQUENCE_MAP: u32 = 0x0101_ded0;
const SEQUENCE_BASE_CONSTRUCT: u32 = 0x00a3_16a0;
const VTABLE_ANIM_GROUP_SEQUENCE: u32 = 0x0101_de3c;
const ANIM_GROUP_SEQUENCE_SIZE: u32 = 0x78;
/// Format strings of the fourth session's messages (`.rdata`).
const LOG_IDLE_FREE_JUST_STARTED: u32 = 0x0101_dd78;
const LOG_SAVE_NULL_SEQUENCE: u32 = 0x0101_ddc8;
/// Doubles: `0.75` and `1000.0`.
const DOUBLE_THREE_QUARTERS: u32 = 0x0101_de30;
const DOUBLE_THOUSAND: u32 = 0x0101_7b70;

// ---- Data used by the functions ----------------------------------------------

/// `AnimSequenceBase`, `AnimSequenceSingle` and `AnimSequenceMultiple`
/// vtables (`.rdata`).
const VTABLE_ANIM_SEQUENCE_SINGLE: u32 = 0x0101_d8c8;
const VTABLE_ANIM_SEQUENCE_BASE: u32 = 0x0101_d8e8;
const VTABLE_ANIM_SEQUENCE_MULTIPLE: u32 = 0x0101_d908;
/// The base destructor body (`0048ef40`, sets the base vtable).
const ANIM_SEQUENCE_BASE_DESTRUCT: u32 = 0x0048_ef40;

/// `"%s\\%s\\%s"` and `"SpecialAnims"`.
const SPECIAL_ANIMS_FORMAT: u32 = 0x0101_7f74;
const SPECIAL_ANIMS_FOLDER: u32 = 0x0101_3448;
/// The two suffix strings `GetCorrespondingSequence` appends for kinds 5
/// and 6.
const SUFFIX_KIND_5: u32 = 0x0101_d92c;
const SUFFIX_KIND_6: u32 = 0x0101_d924;
/// Log formats of `AddAnimation`.
const LOG_SEQUENCE_TYPE_NOT_FOUND: u32 = 0x0101_da38;
const LOG_UNABLE_TO_ADD: u32 = 0x0101_d9a8;
const LOG_OBJECT_NOT_IN_SKELETON: u32 = 0x0101_d970;
/// The three floats a fresh `Animation` copies into `movementDelta` and
/// `AccumRootTranslate`.
const ZERO_VECTOR: u32 = 0x011f_426c;

layout! {
    /// `AnimSequenceSingle` (Xbox PDB): vtable at +0, then the one sequence.
    pub struct AnimSequenceSingle: 0x08 {
        /// `pSeq` (Xbox PDB): `BSAnimGroupSequence*`.
        0x04 pSeq: Ptr,
    }

    /// `AnimSequenceMultiple` (Xbox PDB): vtable at +0, then the list.
    pub struct AnimSequenceMultiple: 0x08 {
        /// `pSeqList` (Xbox PDB): `NiTList<BSAnimGroupSequence*>*`.
        0x04 pSeqList: Ptr,
    }

    /// `BSAnimGroupSequence` (Xbox PDB), 0x78 bytes: a
    /// `NiControllerSequence` (0x74 bytes) and the group.
    pub struct BSAnimGroupSequence: 0x78 {
        /// `spAnimGroup` (Xbox PDB): `NiPointer<TESAnimGroup>`.
        0x74 spAnimGroup: Ptr,
    }

    /// `NiControllerManager` (Xbox PDB), 0x7c bytes; only the fields this
    /// unit reads. `m_kSequenceArray` and `m_kActiveSequences` are embedded
    /// containers: only their addresses are used, so they are declared as
    /// their first word.
    pub struct NiControllerManager: 0x7c {
        /// `m_kSequenceArray` (Xbox PDB): `NiTObjectArray<NiPointer<..>>`.
        0x34 m_kSequenceArray: u32,
        /// `m_kActiveSequences` (Xbox PDB): `NiTPrimitiveSet<..>`.
        0x44 m_kActiveSequences: u32,
        /// `m_bCumulative` (Xbox PDB).
        0x68 m_bCumulative: bool,
    }

    /// `Animation` (Xbox PDB), 0x13c bytes on PC as on the Xbox. The
    /// `[[n]]` arrays of the PDB are `n` bytes: the arrays below are 8
    /// entries of 2 or 4 bytes, declared by their first entry.
    pub struct Animation: 0x13c {
        /// `m_uFlags` (Xbox PDB).
        0x00 m_uFlags: u8,
        /// `pActorRef` (Xbox PDB): `TESObjectREFR*`.
        0x04 pActorRef: Ptr,
        /// `pAnimRoot` (Xbox PDB): `NiPointer<NiNode>`.
        0x08 pAnimRoot: Ptr,
        /// `pAccumRoot` (Xbox PDB): `NiNode*`.
        0x0C pAccumRoot: Ptr,
        /// `movementDelta` (Xbox PDB).
        0x10 movementDelta: Inline<NiPoint3>,
        /// `AccumRootTranslate` (Xbox PDB).
        0x1C AccumRootTranslate: Inline<NiPoint3>,
        /// `pSoundPriorityBone` (Xbox PDB): `NiAVObject*[8]`, first entry.
        0x28 pSoundPriorityBone: Ptr,
        /// `m_fLooking` (Xbox PDB).
        0x48 m_fLooking: f32,
        /// `group` (Xbox PDB): `u16[8]`, first entry.
        0x4C group: u16,
        /// `action` (Xbox PDB): `ANIM_GROUP_ACTION[8]`, first entry.
        0x5C action: i32,
        /// `loopCount` (Xbox PDB): `i32[8]`, first entry.
        0x7C loopCount: i32,
        /// `nextGroup` (Xbox PDB): `u16[8]`, first entry.
        0x9C nextGroup: u16,
        /// `nextLoops` (Xbox PDB): `i32[8]`, first entry.
        0xAC nextLoops: i32,
        /// `cSkipUpdate` (Xbox PDB).
        0xCC cSkipUpdate: u8,
        /// `bShutDown` (Xbox PDB).
        0xCD bShutDown: bool,
        /// `time` (Xbox PDB).
        0xD0 time: f32,
        /// `fLipTime` (Xbox PDB).
        0xD4 fLipTime: f32,
        /// `spManager` (Xbox PDB): `NiPointer<NiControllerManager>`.
        0xD8 spManager: Ptr,
        /// `pAnimSequenceMap` (Xbox PDB):
        /// `NiTPointerMap<unsigned short, AnimSequenceBase*>*`.
        0xDC pAnimSequenceMap: Ptr,
        /// `pCurrentSequence` (Xbox PDB): `BSAnimGroupSequence*[8]`, first
        /// entry.
        0xE0 pCurrentSequence: Ptr,
        /// `pLastMovementSequence` (Xbox PDB).
        0x100 pLastMovementSequence: Ptr,
        /// `kfModelList` (Xbox PDB): `BSSimpleList<KFModel*>`.
        0x104 kfModelList: Inline<BSSimpleList>,
        /// `m_fMoveSpeed` (Xbox PDB).
        0x10C m_fMoveSpeed: f32,
        /// `m_fAttackSpeed` (Xbox PDB).
        0x110 m_fAttackSpeed: f32,
        /// `m_fGlobalTimeMultiplier` (Xbox PDB).
        0x114 m_fGlobalTimeMultiplier: f32,
        /// `m_fReloadModifier` (Xbox PDB).
        0x118 m_fReloadModifier: f32,
        /// `m_fEquipModifier` (Xbox PDB).
        0x11C m_fEquipModifier: f32,
        /// `cSkipNextBlend` (Xbox PDB).
        0x120 cSkipNextBlend: u8,
        /// `sQueuedReloadGroup` (Xbox PDB).
        0x122 sQueuedReloadGroup: u16,
        /// `spAnimIdle` (Xbox PDB): `NiPointer<AnimIdle>`.
        0x124 spAnimIdle: Ptr,
        /// `spAnimIdleQueued` (Xbox PDB): `NiPointer<AnimIdle>`.
        0x128 spAnimIdleQueued: Ptr,
        /// `spAnimIdleFreeWhenInactiveA` (Xbox PDB):
        /// `NiPointer<AnimIdle>[2]`, first entry.
        0x12C spAnimIdleFreeWhenInactiveA: Ptr,
        /// `replayDelayList` (Xbox PDB): `BSSimpleList<IDLE_REPLAY_DELAY*>`.
        0x134 replayDelayList: Inline<BSSimpleList>,
    }
}

layout! {
    /// `AnimIdle` (Xbox PDB), 0x38 bytes on PC as on the Xbox: a
    /// `NiRefObject` (vtable and reference count) and the idle's members.
    /// The state (`eFlags`) takes the values 0 (no model yet), 1 (loaded),
    /// 2 (playing) and 3 (done) in the functions below.
    pub struct AnimIdle: 0x38 {
        /// `eFlags` (Xbox PDB): `AnimIdle::ANIM_IDLE_ENUM`, the state.
        0x08 eFlags: u32,
        /// `eType` (Xbox PDB): `AnimIdle::PLAY_TYPE_ENUM`.
        0x0C eType: u32,
        /// `spKFModel` (Xbox PDB): `NiPointer<KFModel>`.
        0x10 spKFModel: Ptr,
        /// `eSection` (Xbox PDB): `ANIM_GROUP_SECTION`, the slot (or the
        /// combined slot 0x14 or 0x15) the idle plays in.
        0x14 eSection: u32,
        /// `pSeq` (Xbox PDB): `NiPointer<BSAnimGroupSequence>`.
        0x18 pSeq: Ptr,
        /// `pAnimObj` (Xbox PDB): `TESObjectANIO*[2]`, first entry.
        0x1C pAnimObj: Ptr,
        /// `spAddOnObj` (Xbox PDB): `NiPointer<NiAVObject>[2]`, first entry.
        0x24 spAddOnObj: Ptr,
        /// `pIdleForm` (Xbox PDB): `TESIdleForm*`.
        0x2C pIdleForm: Ptr,
        /// `pAnimation` (Xbox PDB): `Animation*`.
        0x30 pAnimation: Ptr,
        /// `pActor` (Xbox PDB): `Actor*`.
        0x34 pActor: Ptr,
    }
}

// ---- Small helpers --------------------------------------------------------------

/// `*(NiPointer*)pointer`: the pointer held by the `NiPointer` at `pointer`.
fn ni_pointer_get(e: &mut Engine, pointer: u32) -> u32 {
    e.call(READ_WORD, &args![pointer]).u32()
}

/// The `ModelLoader` singleton.
fn model_loader(e: &Engine) -> u32 {
    e.global(MODEL_LOADER)
}

/// The file name of an `NiObjectNET`: `GetName()` then its `const char*`.
fn object_name(e: &mut Engine, object: u32) -> u32 {
    let name = e.call(OBJECT_NAME, &args![object]).u32();
    e.call(FIXED_STRING_CSTR, &args![name]).u32()
}

/// Runs `body` between the scope guard's constructor (tag `0x33`, source
/// line `line` of `Animation.cpp`) and its destructor.
fn with_scope_guard<R>(e: &mut Engine, line: u32, body: impl FnOnce(&mut Engine) -> R) -> R {
    e.with_stack(SCOPE_GUARD_SIZE, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, SCOPE_GUARD_TAG, 1u32, SOURCE_FILE, line],
        );
        let result = body(e);
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
        result
    })
}

/// Address of entry `i` of an array field of `Animation`.
fn animation_entry<T>(this: Ptr<Animation>, field: Field<Animation, T>, i: u32, size: u32) -> u32 {
    this.addr().wrapping_add(field.off).wrapping_add(i * size)
}

/// Address of `action[slot]`, `group[slot]`, `loopCount[slot]`,
/// `nextGroup[slot]`, `nextLoops[slot]` and `pCurrentSequence[slot]` (the
/// slot is not range checked, as in the game).
fn action_at(this: Ptr<Animation>, slot: u32) -> u32 {
    animation_entry(this, Animation::action, slot, 4)
}
fn group_at(this: Ptr<Animation>, slot: u32) -> u32 {
    animation_entry(this, Animation::group, slot, 2)
}
fn loop_count_at(this: Ptr<Animation>, slot: u32) -> u32 {
    animation_entry(this, Animation::loopCount, slot, 4)
}
fn next_group_at(this: Ptr<Animation>, slot: u32) -> u32 {
    animation_entry(this, Animation::nextGroup, slot, 2)
}
fn next_loops_at(this: Ptr<Animation>, slot: u32) -> u32 {
    animation_entry(this, Animation::nextLoops, slot, 4)
}
fn current_sequence_at(this: Ptr<Animation>, slot: u32) -> u32 {
    animation_entry(this, Animation::pCurrentSequence, slot, 4)
}

/// `pAnimSequenceMap->GetAt(key, &entry)`: the `AnimSequenceBase*` stored
/// for animation group `key`, or `None` when the map has no such key.
fn sequence_map_get(e: &mut Engine, this: Ptr<Animation>, key: u16) -> Option<u32> {
    let map = e.get(this, Animation::pAnimSequenceMap).addr();
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), 0);
        if e.call(MAP_GET_AT, &args![map, key, cell]).bool() {
            Some(e.mem.u32(cell.addr()))
        } else {
            None
        }
    })
}

/// The `float` at the address a setting-value getter returns for the
/// setting object `setting` (`SETTING_FLOAT_ADDRESS`).
fn float_setting(e: &mut Engine, setting: u32) -> f32 {
    let address = e.call(SETTING_FLOAT_ADDRESS, &args![setting]).u32();
    e.mem.f32(address)
}

/// `TESAnimGroup::GetType(group)`: the low byte of the group id.
fn group_type(e: &mut Engine, group: u16) -> u32 {
    e.call(GROUP_ID_TYPE, &args![group]).u32()
}

/// A `double` constant the code loads from `.rdata`.
fn double_constant(e: &Engine, address: u32) -> f64 {
    e.global::<f64>(address)
}

/// The type-table column `column` (`SEQUENCE_TYPE_SLOT`,
/// `SEQUENCE_TYPE_CATEGORY`) of the animation group type `group_type`.
fn type_column(e: &Engine, column: u32, group_type: u32) -> i32 {
    e.mem
        .i32(column.wrapping_add(group_type.wrapping_mul(0x24)))
}

// ---- AnimSequenceBase, AnimSequenceSingle, AnimSequenceMultiple ------------------------------

// Translated from 0048ee70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceSingle` destructor body (the map has no name for it): sets
/// the vtable, and when a sequence is held, unloads its model by file name
/// (`ModelLoader` `00445370`) and releases it; then the base destructor.
/// The compiler's exception-unwinding frame is not translated.
pub fn fn_0048ee70(e: &mut Engine, this: Ptr<AnimSequenceSingle>) {
    e.mem.set_u32(this.addr(), VTABLE_ANIM_SEQUENCE_SINGLE);
    let sequence = e.get(this, AnimSequenceSingle::pSeq);
    if !sequence.is_null() {
        e.with_stack(0x104, |e, buffer| {
            let name = object_name(e, sequence.addr());
            e.call(STRCPY_S, &args![buffer, 0x104u32, name]);
            let sequence = e.get(this, AnimSequenceSingle::pSeq);
            e.call(REF_RELEASE, &args![sequence]);
            let loader = model_loader(e);
            fn_0048eff0(e, Ptr::new(loader), buffer.addr());
        });
    }
    e.call(ANIM_SEQUENCE_BASE_DESTRUCT, &args![this]);
}

// Translated from 0048ef60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceBase::_scalar_deleting_destructor_` (Xbox PDB): the base
/// destructor body, then `operator delete` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn anim_sequence_base_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(ANIM_SEQUENCE_BASE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0048ef90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceSingle` implementation of two virtual functions the linker
/// folded (`GetSequenceToPlay` at +0x10 and `GetSequence` at +0x14): the
/// held sequence, whatever the argument.
pub fn fn_0048ef90(e: &mut Engine, this: Ptr<AnimSequenceSingle>, _unused_argument: u32) -> Ptr {
    e.get(this, AnimSequenceSingle::pSeq)
}

// Translated from 0048efb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceSingle::GetSequenceIndex` (slot +0x18): always `0xff`
/// ("no index"), whatever the argument.
pub fn fn_0048efb0(_e: &mut Engine, _this: Ptr<AnimSequenceSingle>, _unused_argument: u32) -> u8 {
    0xff
}

// Translated from 0048efc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceSingle::_scalar_deleting_destructor_` (Xbox PDB):
/// destroys, and frees when bit 0 of `flags` is set. Returns `this`.
pub fn anim_sequence_single_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<AnimSequenceSingle>,
    flags: u32,
) -> Ptr<AnimSequenceSingle> {
    fn_0048ee70(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0048eff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unloads the model named `name` from the `ModelLoader` `loader`
/// (`00445370(name, 1)`); the map has no name for it.
pub fn fn_0048eff0(e: &mut Engine, loader: Ptr, name: u32) {
    e.call(MODEL_LOADER_REMOVE, &args![loader, name, 1u32]);
}

// Translated from 0048f010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceSingle::SetSequence` (Xbox PDB): replaces the held
/// sequence with `sequence`. The old one is released, and when
/// `free_model` is set its model is first unloaded from the `ModelLoader`
/// by file name; the new one gets a reference.
pub fn anim_sequence_single_set_sequence(
    e: &mut Engine,
    this: Ptr<AnimSequenceSingle>,
    sequence: Ptr,
    free_model: u8,
) {
    let old = e.get(this, AnimSequenceSingle::pSeq);
    if !old.is_null() {
        if free_model != 0 {
            let name = object_name(e, old.addr());
            let loader = model_loader(e);
            fn_0048eff0(e, Ptr::new(loader), name);
        }
        let old = e.get(this, AnimSequenceSingle::pSeq);
        e.call(REF_RELEASE, &args![old]);
    }
    e.set(this, AnimSequenceSingle::pSeq, sequence);
    let held = e.get(this, AnimSequenceSingle::pSeq);
    if !held.is_null() {
        e.call(REF_ADD, &args![held]);
    }
}

// Translated from 0048f080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceSingle::RemoveSequence` (Xbox PDB): when `sequence` is the
/// held one, clears it through `SetSequence(0, free_model)` (virtual call,
/// slot +4) and returns true.
pub fn anim_sequence_single_remove_sequence(
    e: &mut Engine,
    this: Ptr<AnimSequenceSingle>,
    sequence: Ptr,
    free_model: u8,
) -> bool {
    if e.get(this, AnimSequenceSingle::pSeq) == sequence {
        e.vcall(this.addr(), 4, &args![0u32, free_model]);
        true
    } else {
        false
    }
}

// Translated from 0048f0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceMultiple` constructor from another sequence container
/// (the map has no name for it): builds an empty list, moves `other`'s
/// current sequence (`GetSequenceToPlay(-1)`) into it, clears `other`
/// (`SetSequence(0, 0)`) and deletes it. Returns `this`.
///
/// The exe pushes `1, -1` before `GetSequenceToPlay` (+0x10), which pops one
/// word (the -1; `RET 4`); the 1 left on the stack is the second word of
/// `SetSequence` (0048f3a0, `RET 8`, which reads only the first), after the
/// sequence pushed at 0048f14f.
pub fn fn_0048f0c0(
    e: &mut Engine,
    this: Ptr<AnimSequenceMultiple>,
    other: Ptr,
) -> Ptr<AnimSequenceMultiple> {
    fn_0048f1b0(e, this.cast());
    e.mem.set_u32(this.addr(), VTABLE_ANIM_SEQUENCE_MULTIPLE);
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let list = if block == 0 {
        0
    } else {
        fn_0048f200(e, Ptr::new(block)).addr()
    };
    e.set(this, AnimSequenceMultiple::pSeqList, Ptr::new(list));
    let sequence = e
        .vcall(other.addr(), 0x10, &args![0xffff_ffffu32])
        .ptr::<()>();
    anim_sequence_multiple_set_sequence(e, this, sequence, 1);
    e.vcall(other.addr(), 4, &args![0u32, 0u32]);
    if !other.is_null() {
        e.vcall(other.addr(), 0, &args![1u32]);
    }
    this
}

// Translated from 0048f1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceBase` constructor (the map has no name for it): sets the
/// base vtable. Returns `this`.
pub fn fn_0048f1b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_ANIM_SEQUENCE_BASE);
    this
}

// Translated from 0048f1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceMultiple::_scalar_deleting_destructor_` (Xbox PDB):
/// destroys, and frees when bit 0 of `flags` is set. Returns `this`.
pub fn anim_sequence_multiple_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<AnimSequenceMultiple>,
    flags: u32,
) -> Ptr<AnimSequenceMultiple> {
    fn_0048f220(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0048f200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the empty `NiTList` an `AnimSequenceMultiple` holds
/// (`00718d90`, shared with other lists); returns `this`.
pub fn fn_0048f200(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(LIST_CONSTRUCT, &args![this]);
    this
}

// Translated from 0048f220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceMultiple` destructor body (the map has no name for it):
/// for every sequence of the list, unloads its model by file name and
/// releases it; then deletes the list and runs the base destructor.
pub fn fn_0048f220(e: &mut Engine, this: Ptr<AnimSequenceMultiple>) {
    e.mem.set_u32(this.addr(), VTABLE_ANIM_SEQUENCE_MULTIPLE);
    let list = e.get(this, AnimSequenceMultiple::pSeqList);
    if !list.is_null() {
        e.with_stack(0x104, |e, buffer| {
            let mut position = e.call(LIST_HEAD, &args![list]).u32();
            while position != 0 {
                let item = e.call(LIST_ITEM_ADDRESS, &args![list, position]).u32();
                let sequence = e.mem.u32(item);
                let name = object_name(e, sequence);
                e.call(STRCPY_S, &args![buffer, 0x104u32, name]);
                e.call(REF_RELEASE, &args![sequence]);
                let loader = model_loader(e);
                fn_0048eff0(e, Ptr::new(loader), buffer.addr());
                position = e.call(LIST_NEXT, &args![list, position]).u32();
            }
        });
        let list = e.get(this, AnimSequenceMultiple::pSeqList);
        if !list.is_null() {
            fn_0048f370(e, list, 1);
        }
    }
    e.call(ANIM_SEQUENCE_BASE_DESTRUCT, &args![this]);
}

// Translated from 0048f370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the `NiTList` (the map has no name for
/// it): destroys the list, frees it when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_0048f370(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(LIST_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0048f3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceMultiple::SetSequence` (Xbox PDB): adds a reference to
/// `sequence` and adds it at the head of the list. The second argument the
/// virtual interface passes is not read.
pub fn anim_sequence_multiple_set_sequence(
    e: &mut Engine,
    this: Ptr<AnimSequenceMultiple>,
    sequence: Ptr,
    _unused_free_model: u32,
) {
    e.call(REF_ADD, &args![sequence]);
    let list = e.get(this, AnimSequenceMultiple::pSeqList);
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), sequence.addr());
        e.call(LIST_ADD_HEAD, &args![list, cell]);
    });
}

// Translated from 0048f3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceMultiple::RemoveSequence` (Xbox PDB): releases `sequence`
/// and removes it from the list; when that leaves the list empty, deletes
/// the list and returns true, otherwise false. The second argument the
/// virtual interface passes is not read.
pub fn anim_sequence_multiple_remove_sequence(
    e: &mut Engine,
    this: Ptr<AnimSequenceMultiple>,
    sequence: Ptr,
    _unused_free_model: u32,
) -> bool {
    e.call(REF_RELEASE, &args![sequence]);
    let list = e.get(this, AnimSequenceMultiple::pSeqList);
    e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), sequence.addr());
        e.call(LIST_REMOVE, &args![list, cell]);
    });
    let list = e.get(this, AnimSequenceMultiple::pSeqList);
    if e.call(LIST_IS_EMPTY, &args![list]).bool() {
        let list = e.get(this, AnimSequenceMultiple::pSeqList);
        if !list.is_null() {
            fn_0048f370(e, list, 1);
        }
        e.set(this, AnimSequenceMultiple::pSeqList, Ptr::NULL);
        true
    } else {
        false
    }
}

// Translated from 0048f450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceMultiple::GetSequenceToPlay` (Xbox PDB): the sequence at
/// `index` (a signed byte), or a random one (`random % count`) when
/// `index` is -1 or past the end; 0 when the list runs out first. A random
/// pick from an empty list divides by zero, as in the game.
pub fn anim_sequence_multiple_get_sequence_to_play(
    e: &mut Engine,
    this: Ptr<AnimSequenceMultiple>,
    index: i8,
) -> Ptr {
    let list = e.get(this, AnimSequenceMultiple::pSeqList);
    let use_random = if index == -1 {
        true
    } else {
        let count = e.call(LIST_COUNT, &args![list]).u32();
        (index as i32 as u32) >= count
    };
    let mut remaining = if use_random {
        let random = e.call(RANDOM, &args![]).u32();
        let list = e.get(this, AnimSequenceMultiple::pSeqList);
        let count = e.call(LIST_COUNT, &args![list]).u32();
        random % count
    } else {
        index as i32 as u32
    };
    let list = e.get(this, AnimSequenceMultiple::pSeqList);
    let mut position = e.call(LIST_HEAD, &args![list]).u32();
    while position != 0 {
        let item = e.call(LIST_ITEM_ADDRESS, &args![list, position]).u32();
        let sequence = e.mem.u32(item);
        let before = remaining;
        remaining = remaining.wrapping_sub(1);
        if before == 0 {
            return Ptr::new(sequence);
        }
        position = e.call(LIST_NEXT, &args![list, position]).u32();
    }
    Ptr::NULL
}

// Translated from 0048f500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceMultiple::GetCorrespondingSequence` (Xbox PDB): the
/// sequence in the list whose file name (from the last `\`) equals that of
/// `sequence`, case-insensitively. For `kind` 5 and 6 the name is first
/// rewritten: the text from the first `_` of the file name is replaced by
/// a fixed suffix and the rest of the original name after its first `_`
/// is appended. 0 when there is no match (or no `\` in the name).
pub fn anim_sequence_multiple_get_corresponding_sequence(
    e: &mut Engine,
    this: Ptr<AnimSequenceMultiple>,
    sequence: Ptr,
    kind: i32,
) -> Ptr {
    let list = e.get(this, AnimSequenceMultiple::pSeqList);
    let mut position = e.call(LIST_HEAD, &args![list]).u32();
    let full_name = object_name(e, sequence.addr());
    let file_name = e.call(STRRCHR, &args![full_name, 0x5cu32]).u32();
    if file_name == 0 {
        return Ptr::NULL;
    }
    e.with_stack(0x104, |e, buffer| {
        let mut wanted = file_name;
        let suffix = match kind {
            5 => Some(SUFFIX_KIND_5),
            6 => Some(SUFFIX_KIND_6),
            _ => None,
        };
        if let Some(suffix) = suffix {
            e.call(STRCPY, &args![buffer, file_name]);
            let underscore = e.call(STRCHR, &args![buffer, 0x5fu32]).u32();
            if underscore != 0 {
                e.call(STRCPY, &args![underscore, suffix]);
                let tail = e.call(STRCHR, &args![file_name, 0x5fu32]).u32();
                e.call(STRCAT, &args![buffer, tail]);
            } else {
                e.call(STRCAT, &args![buffer, suffix]);
            }
            wanted = buffer.addr();
        }
        while position != 0 {
            let item = e.call(LIST_ITEM_ADDRESS, &args![list, position]).u32();
            let candidate = e.mem.u32(item);
            let candidate_path = object_name(e, candidate);
            let candidate_name = e.call(STRRCHR, &args![candidate_path, 0x5cu32]).u32();
            if candidate_name != 0 && e.call(STRICMP, &args![candidate_name, wanted]).i32() == 0 {
                return Ptr::new(candidate);
            }
            position = e.call(LIST_NEXT, &args![list, position]).u32();
        }
        Ptr::NULL
    })
}

// Translated from 0048f720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceMultiple::GetSequenceIndex` (Xbox PDB): the position of
/// `sequence` in the list as a byte, `0xff` when it is not there.
pub fn anim_sequence_multiple_get_sequence_index(
    e: &mut Engine,
    this: Ptr<AnimSequenceMultiple>,
    sequence: Ptr,
) -> u8 {
    let list = e.get(this, AnimSequenceMultiple::pSeqList);
    let mut position = e.call(LIST_HEAD, &args![list]).u32();
    let mut index = 0u8;
    while position != 0 {
        let item = e.call(LIST_ITEM_ADDRESS, &args![list, position]).u32();
        if e.mem.u32(item) == sequence.addr() {
            return index;
        }
        index = index.wrapping_add(1);
        position = e.call(LIST_NEXT, &args![list, position]).u32();
    }
    0xff
}

// Translated from 0048f790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceMultiple::GetSequence` (Xbox PDB): the sequence of the
/// list whose animation group is `group`, or 0.
pub fn anim_sequence_multiple_get_sequence(
    e: &mut Engine,
    this: Ptr<AnimSequenceMultiple>,
    group: Ptr,
) -> Ptr {
    let list = e.get(this, AnimSequenceMultiple::pSeqList);
    let mut position = e.call(LIST_HEAD, &args![list]).u32();
    while position != 0 {
        let item = e.call(LIST_ITEM_ADDRESS, &args![list, position]).u32();
        let sequence = e.mem.u32(item);
        if fn_0048f7f0(e, Ptr::new(sequence)) == group {
            return Ptr::new(sequence);
        }
        position = e.call(LIST_NEXT, &args![list, position]).u32();
    }
    Ptr::NULL
}

// Translated from 0048f7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads `BSAnimGroupSequence::spAnimGroup` (+0x74). The map names this
/// body `Animation::ZeroGlobalTransform`, a name the linker's folding put
/// on it; the body is the group getter.
pub fn fn_0048f7f0(e: &mut Engine, this: Ptr<BSAnimGroupSequence>) -> Ptr {
    let group = this.byte_add(BSAnimGroupSequence::spAnimGroup.off);
    e.call(READ_WORD, &args![group]).ptr()
}

// ---- Animation ---------------------------------------------------------------------

// Translated from 0048f810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::Animation` (Xbox PDB): constructs the members (the
/// `NiPointer`s, the lists, the `NiTMap` with 0x65 buckets), copies the
/// zero vector into `movementDelta` and `AccumRootTranslate`, fills the
/// group/action arrays with their "none" values (0xff / -1), sets the
/// speed multipliers to 1.0 and `sQueuedReloadGroup` to 0xff. Returns
/// `this`.
pub fn animation_animation(e: &mut Engine, this: Ptr<Animation>) -> Ptr<Animation> {
    let a = this.addr();
    e.call(NI_POINTER_INIT, &args![a + 0x8, 0u32]);
    e.call(NI_POINT3_CONSTRUCT, &args![a + 0x10]);
    e.call(NI_POINT3_CONSTRUCT, &args![a + 0x1c]);
    e.call(NI_POINTER_INIT, &args![a + 0xd8, 0u32]);
    e.call(SIMPLE_LIST_CONSTRUCT, &args![a + 0x104]);
    e.call(NI_POINTER_INIT, &args![a + 0x124, 0u32]);
    e.call(NI_POINTER_INIT, &args![a + 0x128, 0u32]);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            a + 0x12c,
            4u32,
            2u32,
            NI_POINTER_DEFAULT_CONSTRUCT,
            NI_POINTER_RELEASE
        ],
    );
    e.call(SIMPLE_LIST_CONSTRUCT, &args![a + 0x134]);
    e.call(NI_POINTER_SET, &args![a + 0x8, 0u32]);
    e.set(this, Animation::pAccumRoot, Ptr::NULL);
    for field in [
        Animation::movementDelta.off,
        Animation::AccumRootTranslate.off,
    ] {
        for word in 0..3 {
            let value = e.mem.u32(ZERO_VECTOR + 4 * word);
            e.mem.set_u32(a + field + 4 * word, value);
        }
    }
    e.set(this, Animation::m_fLooking, 0.0);
    e.set(this, Animation::cSkipUpdate, 0xff);
    e.set(this, Animation::cSkipNextBlend, 0);
    fn_0048fb20(e, this, 0, 0xff, 0);
    e.set(this, Animation::pActorRef, Ptr::NULL);
    e.set(this, Animation::pLastMovementSequence, Ptr::NULL);
    e.set(this, Animation::time, 0.0);
    e.call(NI_POINTER_SET, &args![a + 0xd8, 0u32]);
    e.call(NI_POINTER_SET, &args![a + 0x124, 0u32]);
    e.call(NI_POINTER_SET, &args![a + 0x128, 0u32]);
    e.call(NI_POINTER_SET, &args![a + 0x12c, 0u32]);
    e.call(NI_POINTER_SET, &args![a + 0x130, 0u32]);
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let map = if block == 0 {
        0
    } else {
        e.call(MAP_CONSTRUCT, &args![block, 0x65u32]).u32()
    };
    e.set(this, Animation::pAnimSequenceMap, Ptr::new(map));
    e.call(MEMSET, &args![a + 0x28, 0u32, 0x20u32]);
    e.call(MEMSET, &args![a + 0xe0, 0u32, 0x20u32]);
    e.call(MEMSET, &args![a + 0x4c, 0xffu32, 0x10u32]);
    e.call(MEMSET, &args![a + 0x9c, 0xffu32, 0x10u32]);
    e.call(MEMSET, &args![a + 0x5c, 0xffff_ffffu32, 0x20u32]);
    e.call(MEMSET, &args![a + 0x7c, 0xffff_ffffu32, 0x20u32]);
    e.call(MEMSET, &args![a + 0xac, 0xffff_ffffu32, 0x20u32]);
    e.set(this, Animation::m_fMoveSpeed, 1.0);
    e.set(this, Animation::m_fAttackSpeed, 1.0);
    e.set(this, Animation::m_fGlobalTimeMultiplier, 1.0);
    e.set(this, Animation::m_fReloadModifier, 1.0);
    e.set(this, Animation::m_fEquipModifier, 1.0);
    e.set(this, Animation::sQueuedReloadGroup, 0xff);
    e.set(this, Animation::bShutDown, false);
    this
}

// Translated from 0048fb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the bits of `m_uFlags` selected by `mask` with `value` shifted
/// left by `shift` (the shift count is taken modulo 32, as `SHL` does):
/// `flags = (flags & ~mask) | (value << shift)`, kept to a byte.
pub fn fn_0048fb20(e: &mut Engine, this: Ptr<Animation>, value: u8, mask: u8, shift: u8) {
    let flags = e.get(this, Animation::m_uFlags) as u32;
    let flags = (flags & !(mask as u32)) | ((value as u32) << (shift & 0x1f));
    e.set(this, Animation::m_uFlags, flags as u8);
}

// Translated from 0048fb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::~Animation` body (the map has no name for it): shuts down
/// the idles, cancels the replacement KF list, and when there is a
/// controller manager removes the two controllers it put on the animation
/// root, deactivates all its sequences and detaches it from its target.
/// Then deletes every `AnimSequenceBase` of the map, releases the KF
/// models of the list, frees the map and the replay delay entries, and
/// destroys the members in reverse order of construction. The compiler's
/// exception-unwinding frame is not translated.
pub fn fn_0048fb50(e: &mut Engine, this: Ptr<Animation>) {
    let a = this.addr();
    animation_shutdown_all_anim_idles(e, this);
    let loader = model_loader(e);
    e.call(
        MODEL_LOADER_CANCEL_REPLACEMENT_KF_LIST,
        &args![loader, this],
    );
    if ni_pointer_get(e, a + 0xd8) != 0 {
        let first = if ni_pointer_get(e, a + 8) != 0 {
            let root = ni_pointer_get(e, a + 8);
            e.call(GET_CONTROLLER, &args![root, RTTI_CONTROLLER_FIRST])
                .u32()
        } else {
            0
        };
        let second = if ni_pointer_get(e, a + 8) != 0 {
            let root = ni_pointer_get(e, a + 8);
            e.call(GET_CONTROLLER, &args![root, RTTI_CONTROLLER_SECOND])
                .u32()
        } else {
            0
        };
        if first != 0 {
            let root = ni_pointer_get(e, a + 8);
            e.call(REMOVE_CONTROLLER, &args![root, first]);
        }
        if second != 0 {
            let root = ni_pointer_get(e, a + 8);
            e.call(REMOVE_CONTROLLER, &args![root, second]);
        }
        let manager = ni_pointer_get(e, a + 0xd8);
        ni_controller_manager_deactivate_all(e, Ptr::new(manager), 0.0);
        let manager = ni_pointer_get(e, a + 0xd8);
        e.call(MANAGER_RESET, &args![manager]);
        let manager = ni_pointer_get(e, a + 0xd8);
        let target = e.call(MANAGER_TARGET, &args![manager]).u32();
        if target != 0 && e.vcall(target, 0xc, &args![]).u32() != 0 {
            let manager = ni_pointer_get(e, a + 0xd8);
            e.call(REMOVE_CONTROLLER, &args![target, manager]);
        }
        e.call(NI_POINTER_SET, &args![a + 0xd8, 0u32]);
    }

    // Delete the sequence containers of the map.
    let map = e.get(this, Animation::pAnimSequenceMap).addr();
    let mut position = e.call(MAP_FIRST_POSITION, &args![map]).u32();
    while position != 0 {
        e.with_stack(12, |e, cells| {
            let (position_cell, key_cell, value_cell) =
                (cells.addr(), cells.addr() + 4, cells.addr() + 8);
            e.mem.set_u32(position_cell, position);
            e.mem.set_u32(value_cell, 0);
            e.call(
                MAP_GET_NEXT,
                &args![map, position_cell, key_cell, value_cell],
            );
            position = e.mem.u32(position_cell);
            let value = e.mem.u32(value_cell);
            if value != 0 {
                e.vcall(value, 0, &args![1u32]);
            }
        });
    }

    // Release the KF models.
    let mut node = a + Animation::kfModelList.off;
    while node != 0 && !e.call(SIMPLE_LIST_IS_EMPTY, &args![node]).bool() {
        let item = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
        let model = e.mem.u32(item);
        e.call(KF_MODEL_RELEASE, &args![model]);
        node = e.call(SIMPLE_LIST_NEXT, &args![node]).u32();
    }
    e.call(SIMPLE_LIST_CLEAR, &args![a + 0x104]);
    let map = e.get(this, Animation::pAnimSequenceMap).addr();
    e.call(MAP_REMOVE_ALL, &args![map]);
    let map = e.get(this, Animation::pAnimSequenceMap).addr();
    if map != 0 {
        e.vcall(map, 0, &args![1u32]);
    }
    e.set(this, Animation::pAnimSequenceMap, Ptr::NULL);
    e.set(this, Animation::pAccumRoot, Ptr::NULL);

    // Free the replay delay entries.
    if !e.call(SIMPLE_LIST_IS_EMPTY, &args![a + 0x134]).bool() {
        let mut node = a + Animation::replayDelayList.off;
        while node != 0 {
            let item = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
            let entry = e.mem.u32(item);
            e.call(OPERATOR_DELETE, &args![entry]);
            node = e.call(SIMPLE_LIST_NEXT, &args![node]).u32();
        }
        e.call(SIMPLE_LIST_CLEAR, &args![a + 0x134]);
    }

    // Destroy the members in reverse order.
    e.call(SIMPLE_LIST_DESTRUCT, &args![a + 0x134]);
    e.call(
        VECTOR_DESTRUCT,
        &args![a + 0x12c, 4u32, 2u32, NI_POINTER_RELEASE],
    );
    e.call(NI_POINTER_RELEASE, &args![a + 0x128]);
    e.call(NI_POINTER_RELEASE, &args![a + 0x124]);
    e.call(SIMPLE_LIST_DESTRUCT, &args![a + 0x104]);
    e.call(NI_POINTER_RELEASE, &args![a + 0xd8]);
    e.call(NI_POINTER_RELEASE, &args![a + 0x8]);
}

// Translated from 0048fef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::DeactivateAll` (Xbox PDB): deactivates every
/// active sequence with the ease-out time `ease_out`.
pub fn ni_controller_manager_deactivate_all(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    ease_out: f32,
) {
    let active = this.addr() + NiControllerManager::m_kActiveSequences.off;
    let mut i = 0u32;
    while i < e.call(LIST_COUNT, &args![active]).u32() {
        let slot = e.call(ACTIVE_SET_ELEMENT, &args![active, i]).u32();
        let sequence = e.mem.u32(slot);
        e.call(MANAGER_DEACTIVATE, &args![this, sequence, ease_out]);
        i += 1;
    }
}

// Translated from 0048ff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::ShutdownAllAnimIdles` (Xbox PDB): flags the animation as
/// shut down, frees the special idles (`SpecialIdleFree(1, 1)`), and frees
/// and clears each of the two "free when inactive" idles that are set.
pub fn animation_shutdown_all_anim_idles(e: &mut Engine, this: Ptr<Animation>) {
    e.set(this, Animation::bShutDown, true);
    e.call(SPECIAL_IDLE_FREE, &args![this, 1u32, 1u32]);
    for i in 0..2 {
        let slot = animation_entry(this, Animation::spAnimIdleFreeWhenInactiveA, i, 4);
        if ni_pointer_get(e, slot) != 0 {
            e.call(ANIM_IDLE_FREE, &args![this, slot]);
            e.call(NI_POINTER_SET, &args![slot, 0u32]);
        }
    }
}

// Translated from 0048ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets up an animation from its keyframe list (the map has no name for
/// it; the arguments are the KF file `list`, the animation root node
/// `root`, the owning reference `actor` and `play_idle`). Returns false
/// without doing anything when an argument is null or the sequence map
/// already has the entry for group 0; otherwise stores the reference and
/// root, finds the sound priority bones, creates the `NiControllerManager`
/// for the root, loads and adds every KF file (deleting the list),
/// and when the map then has an entry for group 0 and `play_idle` is set,
/// activates its sequence and applies a zero translation to the root.
/// Returns whether the map has that entry. The scope guard of source line
/// 568 is kept; the compiler's exception frame is not translated.
pub fn fn_0048ffd0(
    e: &mut Engine,
    this: Ptr<Animation>,
    list: Ptr,
    root: Ptr,
    actor: Ptr,
    play_idle: bool,
) -> bool {
    with_scope_guard(e, 0x238, |e| {
        if list.is_null() || root.is_null() || actor.is_null() {
            return false;
        }
        let a = this.addr();
        e.set(this, Animation::pActorRef, actor);
        e.with_stack(4, |e, out| {
            if e.get(this, Animation::pAnimSequenceMap).addr() != 0 {
                let map = e.get(this, Animation::pAnimSequenceMap);
                if e.call(MAP_GET_AT, &args![map, 0u32, out]).bool() {
                    return false;
                }
            }
            // The actor's base form type, compared with 0x2a (the value is
            // not used afterwards).
            let form = e.call(REFERENCE_FORM, &args![actor]).u32();
            let _is_form_type_2a = e.call(FORM_TYPE, &args![form]).u32() == FORM_TYPE_COMPARED;
            e.call(NI_POINTER_SET, &args![a + 8, root]);
            for i in 0..8u32 {
                let bone_name = e.mem.u32(SOUND_PRIORITY_BONE_NAMES + 4 * i);
                if bone_name != 0 {
                    let bone = fn_00490310(e, actor, root, bone_name);
                    e.mem
                        .set_u32(a + Animation::pSoundPriorityBone.off + 4 * i, bone);
                }
            }
            let anim_root = ni_pointer_get(e, a + 8);
            fn_004902f0(e, Ptr::new(anim_root), 0);
            let block = e.call(NI_OPERATOR_NEW, &args![0x7cu32]).u32();
            let manager = if block == 0 {
                0
            } else {
                let anim_root = ni_pointer_get(e, a + 8);
                e.call(MANAGER_CONSTRUCT, &args![block, anim_root, 1u32])
                    .u32()
            };
            e.call(NI_POINTER_SET, &args![a + 0xd8, manager]);
            let queue_cloning = animation_should_queue_sequence_cloning(e, this);
            while !e.call(SIMPLE_LIST_IS_EMPTY, &args![list]).bool() {
                let item = e.call(SIMPLE_LIST_ITEM, &args![list]).u32();
                let name = e.mem.u32(item);
                let loader = model_loader(e);
                let kf = e.call(MODEL_LOADER_LOAD_KF, &args![loader, name]).u32();
                if kf != 0 && e.call(KF_MODEL_ANIM_GROUP, &args![kf]).u32() != 0 {
                    animation_add_animation(e, this, kf, queue_cloning);
                }
                e.call(OPERATOR_DELETE, &args![name]);
                e.call(SIMPLE_LIST_POP_FRONT, &args![list]);
            }
            if e.get(this, Animation::pAccumRoot).addr() != 0 {
                let accum_root = e.get(this, Animation::pAccumRoot);
                e.call(
                    ACCUM_ROOT_SETUP,
                    &args![accum_root, ACCUM_ROOT_SETUP_ARGUMENT],
                );
            }
            if !list.is_null() {
                e.call(SIMPLE_LIST_DELETE, &args![list, 1u32]);
            }
            let map = e.get(this, Animation::pAnimSequenceMap);
            let found = e.call(MAP_GET_AT, &args![map, 0u32, out]).bool();
            if found && play_idle {
                let entry = e.mem.u32(out.addr());
                let sequence = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32();
                if sequence != 0 {
                    e.call(
                        SEQUENCE_ACTIVATE,
                        &args![sequence, 0x64u32, 1u32, 1.0f32, 0.0f32, 0u32, 0u32],
                    );
                    e.with_stack(12, |e, translation| {
                        e.call(
                            UPDATE_DATA_CONSTRUCT,
                            &args![translation, 0.0f32, 1u32, 0u32],
                        );
                        let anim_root = ni_pointer_get(e, a + 8);
                        e.call(NODE_UPDATE, &args![anim_root, translation]);
                    });
                    e.vcall(sequence, 0x8c, &args![0.0f32, 0u32]);
                }
            }
            found
        })
    })
}

// Translated from 004902f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0043b370(flag, 2)` on `this` (the animation root node); the map
/// has no name for it.
pub fn fn_004902f0(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(NODE_SET_FLAG, &args![this, flag, 2u32]);
}

// Translated from 00490310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the object called `name` under `root` (`__cdecl(root, name, 1)`
/// at `00c4b310`). `this` (the reference) is not used.
pub fn fn_00490310(e: &mut Engine, _this: Ptr, root: Ptr, name: u32) -> u32 {
    e.call(NODE_FIND_OBJECT, &args![root, name, 1u32]).u32()
}

// Translated from 00490330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::AddSpecialAnimations` (Xbox PDB): for every file name in
/// the `BSSimpleList` `list`, loads `"<directory>\\SpecialAnims\\<name>"`
/// as a KF and adds it. Does nothing for a null list. The list and its
/// strings are not freed.
pub fn animation_add_special_animations(
    e: &mut Engine,
    this: Ptr<Animation>,
    list: Ptr,
    directory: u32,
) {
    if list.is_null() {
        return;
    }
    let mut node = list.addr();
    while node != 0 {
        let item = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
        let name = e.mem.u32(item);
        e.with_stack(0x104, |e, path| {
            e.call(
                SPRINTF_S,
                &args![
                    path,
                    0x104u32,
                    SPECIAL_ANIMS_FORMAT,
                    directory,
                    SPECIAL_ANIMS_FOLDER,
                    name
                ],
            );
            let loader = model_loader(e);
            let kf = e.call(MODEL_LOADER_LOAD_KF, &args![loader, path]).u32();
            let queue_cloning = animation_should_queue_sequence_cloning(e, this);
            if kf != 0 {
                animation_add_animation(e, this, kf, queue_cloning);
            }
        });
        node = e.call(SIMPLE_LIST_NEXT, &args![node]).u32();
    }
}

// Translated from 00490400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::AddAnimationsFromList` (Xbox PDB): loads and adds every KF
/// file named in the `BSSimpleList` `list` (freeing each name and
/// removing it from the list), then deletes the list. Does nothing for a
/// null list. The scope guard of source line 750 is kept.
pub fn animation_add_animations_from_list(e: &mut Engine, this: Ptr<Animation>, list: Ptr) {
    if list.is_null() {
        return;
    }
    with_scope_guard(e, 0x2ee, |e| {
        let queue_cloning = animation_should_queue_sequence_cloning(e, this);
        while !e.call(SIMPLE_LIST_IS_EMPTY, &args![list]).bool() {
            let item = e.call(SIMPLE_LIST_ITEM, &args![list]).u32();
            let name = e.mem.u32(item);
            let loader = model_loader(e);
            let kf = e.call(MODEL_LOADER_LOAD_KF, &args![loader, name]).u32();
            animation_add_animation(e, this, kf, queue_cloning);
            e.call(OPERATOR_DELETE, &args![name]);
            e.call(SIMPLE_LIST_POP_FRONT, &args![list]);
        }
        if !list.is_null() {
            e.call(SIMPLE_LIST_DELETE, &args![list, 1u32]);
        }
    });
}

// Translated from 00490500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::AddAnimation` (Xbox PDB): adds the keyframe model `kf` to
/// the animation. Returns true when it was added (or was already there),
/// false when its group has no sequence type, the sequence cannot be added
/// to the controller manager, or a clamped looping sequence is refused.
///
/// - The group's sequence type indexes the per-type table; for types that
///   keep several sequences the existing entry of the group's id in
///   `pAnimSequenceMap` becomes an `AnimSequenceMultiple`, otherwise an
///   existing single sequence is replaced (unless it is the same group, or
///   one of the idles in use), after dropping it from the manager, from
///   `pCurrentSequence`/`group` and `pLastMovementSequence`.
/// - With `queue_cloning` (see `ShouldQueueSequenceCloning`), for types
///   other than 0 and 0x17 and while the Pip-Boy wait menu is not showing,
///   the model is only queued on `kfModelList` and true is returned.
/// - Otherwise the sequence is cloned (unless it is a
///   `BSAnimGroupSequence` with a single reference), stored in the map
///   entry, added to the controller manager, and the accumulation root and
///   group are registered (`AddGroup`).
///
/// The failure report loops over the sequence's objects with a null name
/// buffer, exactly as the game does, so it logs `(null)` for each object
/// the skeleton lacks. The scope guard of source line 794 is kept; the
/// compiler's exception frame is not translated.
pub fn animation_add_animation(
    e: &mut Engine,
    this: Ptr<Animation>,
    kf: u32,
    queue_cloning: bool,
) -> bool {
    e.with_stack(4, |e, group_ref| {
        let group = e.call(KF_MODEL_ANIM_GROUP, &args![kf]).u32();
        e.call(NI_POINTER_INIT, &args![group_ref, group]);
        let result = add_animation_with_group(e, this, kf, queue_cloning, group_ref.addr());
        e.call(NI_POINTER_RELEASE, &args![group_ref]);
        result
    })
}

/// `AddAnimation` after the animation group is held in `group_ref`.
fn add_animation_with_group(
    e: &mut Engine,
    this: Ptr<Animation>,
    kf: u32,
    queue_cloning: bool,
    group_ref: u32,
) -> bool {
    if ni_pointer_get(e, group_ref) == 0 {
        return false;
    }
    let group = ni_pointer_get(e, group_ref);
    let sequence_type = e.call(ANIM_GROUP_SEQUENCE_TYPE, &args![group]).i32();
    let group = ni_pointer_get(e, group_ref);
    let key = e.call(ANIM_GROUP_ID, &args![group]).u16();
    if sequence_type == 0xff {
        if e.call(KF_MODEL_SEQUENCE, &args![kf]).u32() != 0 {
            let file = ni_pointer_get(e, kf);
            let sequence = e.call(KF_MODEL_SEQUENCE, &args![kf]).u32();
            let sequence_name = object_name(e, sequence);
            e.call(
                LOG,
                &args![LOG_SEQUENCE_TYPE_NOT_FOUND, sequence_name, file],
            );
        }
        return false;
    }
    with_scope_guard(e, 0x31a, |e| {
        add_animation_body(e, this, kf, queue_cloning, sequence_type, key)
    })
}

/// The part of `AddAnimation` inside its scope guard.
fn add_animation_body(
    e: &mut Engine,
    this: Ptr<Animation>,
    kf: u32,
    queue_cloning: bool,
    sequence_type: i32,
    key: u16,
) -> bool {
    let a = this.addr();
    let map = e.get(this, Animation::pAnimSequenceMap).addr();
    let (found, mut entry) = e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), 0);
        let found = e.call(MAP_GET_AT, &args![map, key, cell]).bool();
        (found, e.mem.u32(cell.addr()))
    });
    if !found {
        let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
        entry = if block == 0 {
            0
        } else {
            fn_00490e10(e, Ptr::new(block)).addr()
        };
        e.call(MAP_SET_AT, &args![map, key, entry]);
    }

    if e.vcall(entry, 0xc, &args![]).bool()
        && e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32() != 0
    {
        let keeps_several = e
            .mem
            .u8(SEQUENCE_TYPE_TABLE.wrapping_add((sequence_type as u32).wrapping_mul(0x24)))
            != 0;
        if keeps_several {
            e.call(MAP_REMOVE_AT, &args![map, key]);
            let old = entry;
            let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
            entry = if block == 0 {
                0
            } else {
                fn_0048f0c0(e, Ptr::new(block), Ptr::new(old)).addr()
            };
            e.call(MAP_SET_AT, &args![map, key, entry]);
        } else {
            let current = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32();
            let current_group = fn_0048f7f0(e, Ptr::new(current));
            let kf_group = e.call(KF_MODEL_ANIM_GROUP, &args![kf]).ptr::<()>();
            if current_group == kf_group {
                return true;
            }
            let mut replace = true;
            for slot in [a + 0x12c, a + 0x130] {
                if ni_pointer_get(e, slot) != 0 {
                    let idle = ni_pointer_get(e, slot);
                    let idle_sequence = fn_00490e40(e, Ptr::new(idle)).addr();
                    let current = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32();
                    if idle_sequence == current {
                        replace = false;
                        break;
                    }
                }
            }
            if replace {
                let current = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32();
                if current == e.get(this, Animation::pLastMovementSequence).addr() {
                    e.set(this, Animation::pLastMovementSequence, Ptr::NULL);
                }
                let current = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32();
                let manager = ni_pointer_get(e, a + 0xd8);
                e.call(MANAGER_REMOVE_SEQUENCE, &args![manager, current]);
            }
            let current = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32();
            if current != 0 {
                for i in 0..8u32 {
                    let slot = animation_entry(this, Animation::pCurrentSequence, i, 4);
                    if e.mem.u32(slot) == current {
                        e.mem.set_u32(slot, 0);
                        let group_slot = animation_entry(this, Animation::group, i, 2);
                        e.mem.set_u16(group_slot, 0);
                    }
                }
            }
            if e.get(this, Animation::pLastMovementSequence).addr() != 0 {
                let current = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32();
                if current == e.get(this, Animation::pLastMovementSequence).addr() {
                    e.set(this, Animation::pLastMovementSequence, Ptr::NULL);
                }
            }
            e.vcall(entry, 4, &args![0u32, 1u32]);
        }
    }

    // Queue the model instead of adding it now.
    if queue_cloning
        && !e
            .call(IS_MENU_ID_VISIBLE, &args![MENU_ID_PIPBOY_WAIT, 0u32])
            .bool()
        && sequence_type != 0x17
        && sequence_type != 0
        && sequence_type != 0xe0
    {
        e.call(KF_MODEL_ADD_REF, &args![kf]);
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), kf);
            e.call(SIMPLE_LIST_PUSH_BACK, &args![a + 0x104, cell]);
        });
        return true;
    }

    let new_sequence = e.call(KF_MODEL_SEQUENCE, &args![kf]).u32();
    if (3..=0x10).contains(&sequence_type)
        && e.call(SEQUENCE_CYCLE_TYPE, &args![new_sequence]).u32() == 2
    {
        e.call(MAP_REMOVE_AT, &args![map, key]);
        if entry != 0 {
            e.vcall(entry, 0, &args![1u32]);
        }
        return false;
    }

    let use_sequence = if e
        .call(IS_KIND_OF, &args![RTTI_ANIM_GROUP_SEQUENCE, new_sequence])
        .bool()
        && e.call(SIMPLE_LIST_NEXT, &args![new_sequence]).u32() == 1
    {
        // The sequence has a single reference (`+4` is its count): reuse it.
        new_sequence
    } else {
        let block = e.call(NI_OPERATOR_NEW, &args![0x78u32]).u32();
        if block == 0 {
            0
        } else {
            let group = e.call(KF_MODEL_ANIM_GROUP, &args![kf]).u32();
            e.call(
                ANIM_GROUP_SEQUENCE_CONSTRUCT,
                &args![block, group, new_sequence],
            )
            .u32()
        }
    };
    e.vcall(entry, 4, &args![use_sequence, 1u32]);
    let manager = ni_pointer_get(e, a + 0xd8);
    let added = e
        .call(
            MANAGER_ADD_SEQUENCE,
            &args![manager, use_sequence, ACCUM_ROOT_NAME, 1u32],
        )
        .bool();
    if added {
        if e.get(this, Animation::pAccumRoot).addr() == 0 {
            let manager = ni_pointer_get(e, a + 0xd8);
            let accum_root = ni_controller_manager_get_accum_root(e, Ptr::new(manager));
            e.set(this, Animation::pAccumRoot, accum_root);
        }
        let group = e.call(KF_MODEL_ANIM_GROUP, &args![kf]).u32();
        animation_add_group(e, this, Ptr::new(group));
        return true;
    }

    // The manager refused the sequence: report which objects the skeleton
    // lacks, undo, and give up.
    let mut removed = false;
    let root = ni_pointer_get(e, a + 8);
    let root_name = object_name(e, root);
    let sequence_name = object_name(e, use_sequence);
    e.call(LOG, &args![LOG_UNABLE_TO_ADD, sequence_name, root_name]);
    e.call(DISABLE_WARNINGS, &args![1u32]);
    let mut i = 0u32;
    while i < e.call(SEQUENCE_OBJECT_COUNT, &args![use_sequence]).u32() {
        // The name buffer the game passes here is null, so `GetObjectNameAt`
        // writes nothing and the name stays null.
        let name = 0u32;
        e.call(SEQUENCE_OBJECT_NAME_AT, &args![use_sequence, i, name]);
        let root = ni_pointer_get(e, a + 8);
        let missing = e.with_stack(4, |e, fixed| {
            e.call(FIXED_STRING_INIT, &args![fixed, name]);
            let found = e.vcall(root, 0x9c, &args![fixed]).u32();
            e.call(FIXED_STRING_FREE, &args![fixed]);
            found == 0
        });
        if missing {
            e.call(LOG, &args![LOG_OBJECT_NOT_IN_SKELETON, name]);
        }
        e.call(OPERATOR_DELETE, &args![name]);
        i += 1;
    }
    e.call(DISABLE_WARNINGS, &args![0u32]);
    if use_sequence != 0 {
        if use_sequence == e.get(this, Animation::pLastMovementSequence).addr() {
            e.set(this, Animation::pLastMovementSequence, Ptr::NULL);
        }
        let manager = ni_pointer_get(e, a + 0xd8);
        e.call(MANAGER_REMOVE_SEQUENCE, &args![manager, use_sequence]);
        removed = e.vcall(entry, 8, &args![use_sequence, 1u32]).bool();
    }
    if entry != 0 && removed {
        e.call(MAP_REMOVE_AT, &args![map, key]);
        e.vcall(entry, 0, &args![1u32]);
    }
    false
}

// Translated from 00490d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::GetAccumRoot` (Xbox PDB): for a cumulative
/// manager, the accumulation root of the first sequence that has an
/// object; 0 for a non-cumulative manager or none.
pub fn ni_controller_manager_get_accum_root(e: &mut Engine, this: Ptr<NiControllerManager>) -> Ptr {
    if e.get(this, NiControllerManager::m_bCumulative) {
        let array = this.addr() + NiControllerManager::m_kSequenceArray.off;
        let mut i = 0u32;
        while i < e.call(SEQUENCE_ARRAY_COUNT, &args![array]).u32() {
            let slot = e.call(SEQUENCE_ARRAY_ELEMENT, &args![array, i]).u32();
            if e.call(READ_WORD, &args![slot]).u32() != 0 {
                let slot = e.call(SEQUENCE_ARRAY_ELEMENT, &args![array, i]).u32();
                let sequence = e.call(READ_WORD, &args![slot]).u32();
                return e.call(ACCUM_ROOT_OF_SEQUENCE, &args![sequence]).ptr();
            }
            i += 1;
        }
    }
    Ptr::NULL
}

// Translated from 00490e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimSequenceSingle` constructor (the map has no name for it): base
/// constructor, the single vtable, no sequence. Returns `this`.
pub fn fn_00490e10(e: &mut Engine, this: Ptr<AnimSequenceSingle>) -> Ptr<AnimSequenceSingle> {
    fn_0048f1b0(e, this.cast());
    e.mem.set_u32(this.addr(), VTABLE_ANIM_SEQUENCE_SINGLE);
    e.set(this, AnimSequenceSingle::pSeq, Ptr::NULL);
    this
}

// Translated from 00490e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimIdle::pSeq` (+0x18): the sequence an idle plays (the map has no
/// name for it).
pub fn fn_00490e40(e: &mut Engine, this: Ptr) -> Ptr {
    let field = this.byte_add(0x18);
    e.call(READ_WORD, &args![field]).ptr()
}

// Translated from 00490e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::ShouldQueueSequenceCloning` (Xbox PDB): true when a KF
/// should be queued instead of cloned right away. False without the
/// player object, its (loaded) 3D, or when the cloning setting at
/// `011c5778` is 0. When the animation root is the player's first or
/// second 3D root, or the player's `+0x69c` object accepts it
/// (`00822510`), the answer is whether the Pip-Boy wait menu is visible.
/// Otherwise false when any of the other checks hold (`0042ce10`, the
/// reference is the one at `0047b200`, menu mode, `00451530`).
pub fn animation_should_queue_sequence_cloning(e: &mut Engine, this: Ptr<Animation>) -> bool {
    let a = this.addr();
    let player: u32 = e.global(PLAYER_SINGLETON);
    if player == 0 {
        return false;
    }
    if e.call(PLAYER_NODE, &args![player, 0u32]).u32() == 0 {
        return false;
    }
    let setting = e.call(SETTING_VALUE_ADDRESS, &args![SETTING_CLONING]).u32();
    if e.mem.u32(setting) == 0 {
        return false;
    }
    let root = ni_pointer_get(e, a + 8);
    let first = e.call(PLAYER_NODE, &args![player, 0u32]).u32();
    let mut menu_check = root == first;
    if !menu_check {
        let root = ni_pointer_get(e, a + 8);
        let second = e.call(PLAYER_NODE, &args![player, 1u32]).u32();
        menu_check = root == second;
    }
    if !menu_check {
        let object = fn_00490f80(e, Ptr::new(player));
        menu_check = e.call(NODE_ACCEPTS_OBJECT, &args![a + 8, object]).bool();
    }
    if menu_check {
        return e
            .call(IS_MENU_ID_VISIBLE, &args![MENU_ID_PIPBOY_WAIT, 0u32])
            .bool();
    }
    let flag_object: u32 = e.global(FLAG_OBJECT_GLOBAL);
    if e.call(FLAG_OBJECT_CHECK, &args![flag_object]).bool() {
        return false;
    }
    let player_reference = e.call(PLAYER_REFERENCE, &args![]).u32();
    if e.get(this, Animation::pActorRef).addr() == player_reference {
        return false;
    }
    if e.call(IS_IN_MENU_MODE, &args![]).bool() {
        return false;
    }
    let tes: u32 = e.global(TES_GLOBAL);
    !e.call(TES_CHECK, &args![tes]).bool()
}

// Translated from 00490f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer at `+0x69c` of the player object (`READ_WORD` on its
/// address); the map has no name for it.
pub fn fn_00490f80(e: &mut Engine, this: Ptr) -> Ptr {
    let field = this.byte_add(0x69c);
    e.call(READ_WORD, &args![field]).ptr()
}

// Translated from 00490fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues the keyframe model `kf` on `kfModelList` (two references added)
/// when it has an animation group; the map has no name for it.
pub fn fn_00490fa0(e: &mut Engine, this: Ptr<Animation>, kf: u32) {
    e.with_stack(4, |e, group_ref| {
        let group = e.call(KF_MODEL_ANIM_GROUP, &args![kf]).u32();
        e.call(NI_POINTER_INIT, &args![group_ref, group]);
        if ni_pointer_get(e, group_ref.addr()) != 0 {
            e.call(KF_MODEL_ADD_REF, &args![kf]);
            e.call(KF_MODEL_ADD_REF, &args![kf]);
            e.with_stack(4, |e, cell| {
                e.mem.set_u32(cell.addr(), kf);
                e.call(
                    SIMPLE_LIST_PUSH_BACK,
                    &args![this.addr() + Animation::kfModelList.off, cell],
                );
            });
        }
        e.call(NI_POINTER_RELEASE, &args![group_ref]);
    });
}

// Translated from 00491040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The current sequence of animation group `group`: `pCurrentSequence`
/// slot 1 for group 0x14, slot 4 for group 0x15, otherwise slot `group`
/// (not range checked); the map has no name for it.
pub fn fn_00491040(e: &mut Engine, this: Ptr<Animation>, group: u32) -> Ptr {
    let slot = match group {
        0x14 => 1,
        0x15 => 4,
        other => other,
    };
    let address = animation_entry(this, Animation::pCurrentSequence, slot, 4);
    Ptr::new(e.mem.u32(address))
}

// Translated from 00491090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The length of a sequence in seconds: 0.0 for a null sequence, otherwise
/// `end (00508100) - begin (00759450)`, both read as `float` from the
/// sequence (`__cdecl`, the sequence in the argument, result in ST0); the
/// map has no name for it.
pub fn fn_00491090(e: &mut Engine, sequence: Ptr) -> f32 {
    if sequence.is_null() {
        return 0.0;
    }
    let begin = e.call(SEQUENCE_BEGIN_TIME, &args![sequence]).f32();
    let end = e.call(SEQUENCE_END_TIME, &args![sequence]).f32();
    (end as f64 - begin as f64) as f32
}

// ---- Animation::FindSkinnedNode, Animation::Update --------------------------------

// Translated from 004910d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::FindSkinnedNode` (Xbox PDB), `__cdecl(node)`: true when
/// `node` or one of its descendants has a skin. The virtual function at +0x18
/// gives the object whose word at +0xbc `NODE_SKIN` reads; the one at +0xc
/// gives the node whose children are searched.
pub fn animation_find_skinned_node(e: &mut Engine, node: Ptr) -> bool {
    if node.is_null() {
        return false;
    }
    let skin_owner = e.vcall(node.addr(), 0x18, &args![]).u32();
    if skin_owner != 0 && e.call(NODE_SKIN, &args![skin_owner]).u32() != 0 {
        return true;
    }
    let children = e.vcall(node.addr(), 0xc, &args![]).u32();
    if children != 0 {
        let mut index = 0u32;
        while index < e.call(NODE_CHILD_COUNT, &args![children]).u32() {
            let child = e.call(NODE_CHILD_AT, &args![children, index]).u32();
            if child != 0 && animation_find_skinned_node(e, Ptr::new(child)) {
                return true;
            }
            index += 1;
        }
    }
    false
}

/// The stage values `Update` writes into `action[slot]` and the phase it
/// sets are `float`s computed in extended precision and stored as `float`;
/// this sets the phase of `sequence` (`SetPhase(phase, 0)`).
fn set_phase(e: &mut Engine, sequence: u32, phase: f32) {
    e.call(SEQUENCE_SET_PHASE, &args![sequence, phase, 0u32]);
}

/// The elapsed time of a sequence (`00493770`, a stored `float`).
fn sequence_elapsed(e: &mut Engine, sequence: u32) -> f64 {
    e.call(SEQUENCE_ELAPSED, &args![sequence]).f64()
}

/// `section < elapsed` where an unordered pair (a NaN) counts as "not
/// before", as the `FCOMP` tests in `Update` do.
fn section_is_before(section: f64, elapsed: f64) -> bool {
    section < elapsed
}

/// `GetTime(group of sequence, index)` and the time `00493800` gives for
/// the sequence: the pair `Update` compares (the section time against the
/// time the animation is at).
fn section_and_elapsed(
    e: &mut Engine,
    this: Ptr<Animation>,
    sequence: u32,
    index: i32,
) -> (f64, f64) {
    let elapsed = e
        .call(SEQUENCE_TIME_ON_ANIMATION, &args![this, sequence])
        .f64();
    let group = fn_0048f7f0(e, Ptr::new(sequence));
    let section = e.call(GROUP_TIME, &args![group, index]).f64();
    (section, elapsed)
}

/// `loopCount[slot] = nextLoops[slot]; StartGroup(nextGroup[slot], -1);
/// nextGroup[slot] = 0xff`: the next queued group takes over the slot.
fn start_next_group(e: &mut Engine, this: Ptr<Animation>, slot: u32) {
    let loops = e.mem.u32(next_loops_at(this, slot));
    e.mem.set_u32(loop_count_at(this, slot), loops);
    let group = e.mem.u16(next_group_at(this, slot));
    animation_start_group(e, this, group, -1);
    e.mem.set_u16(next_group_at(this, slot), 0xff);
}

/// `BlendOut(slot, 0)`.
fn blend_out(e: &mut Engine, this: Ptr<Animation>, slot: u32) {
    e.call(BLEND_OUT, &args![this, slot, 0u32]);
}

/// `((group movement * scaledTime) * movement scale setting) * m_fMoveSpeed`,
/// computed through the `NiPoint3` helpers into temporaries; the three
/// result words.
fn scaled_group_movement(
    e: &mut Engine,
    this: Ptr<Animation>,
    group: Ptr,
    scaled_time: f32,
) -> [u32; 3] {
    e.with_stack(48, |e, buffers| {
        let first = buffers.addr();
        let (second, third, fourth) = (first + 12, first + 24, first + 36);
        let move_speed = e.get(this, Animation::m_fMoveSpeed);
        let setting = float_setting(e, SETTING_MOVEMENT_SCALE);
        e.call(GROUP_MOVEMENT_VECTOR, &args![group, first]);
        e.call(VECTOR_SCALE, &args![first, second, scaled_time]);
        e.call(VECTOR_SCALE, &args![second, third, setting]);
        e.call(VECTOR_SCALE, &args![third, fourth, move_speed]);
        [
            e.mem.u32(fourth),
            e.mem.u32(fourth + 4),
            e.mem.u32(fourth + 8),
        ]
    })
}

/// `Update`'s handling of `sQueuedReloadGroup` while no models are queued:
/// plays the queued group when the actor's weapon is drawn and tells the
/// player object and the actor. Returns early (leaving the queued group
/// set) when the weapon of slot 4 is in an attack.
fn update_queued_reload(e: &mut Engine, this: Ptr<Animation>, actor: Ptr) {
    let a = this.addr();
    let saved = e.mem.u32(current_sequence_at(this, 4));
    let action = e.mem.i32(action_at(this, 4));
    let mut play = false;
    if action > 0 && action <= 3 {
        if action <= 2 && saved != 0 {
            let group = fn_0048f7f0(e, Ptr::new(saved));
            // The aim test comes first; a held attack ends the block.
            if !e.call(GROUP_IS_AIM_OF, &args![group]).bool()
                && e.call(GROUP_IS_ATTACK_OF, &args![group]).bool()
            {
                return;
            }
        }
        play = true;
    }
    if play {
        let reference = e.get(this, Animation::pActorRef).addr();
        if e.call(ACTOR_IS_WEAPON_DRAWN, &args![reference]).bool() {
            let queued = e.mem.u16(a + Animation::sQueuedReloadGroup.off);
            animation_play_group(e, this, queued, 1, -1, -1);
            let player: u32 = e.global(PLAYER_SINGLETON);
            if actor.addr() == player {
                let queued = e.mem.u16(a + Animation::sQueuedReloadGroup.off);
                e.vcall(player, 0x4b0, &args![queued, 1u32]);
            }
            let mut has_item = false;
            if e.call(ACTOR_PROCESS, &args![actor]).u32() != 0 {
                let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
                let equipped = e.vcall(process, 0x148, &args![]).u32();
                let owner = if equipped != 0 {
                    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
                    let equipped = e.vcall(process, 0x148, &args![]).u32();
                    e.call(WORD_AT_8, &args![equipped]).u32()
                } else {
                    // The game calls `004938c0` with null here too.
                    0
                };
                if e.call(PROCESS_FLAG_BYTE, &args![owner]).bool() {
                    has_item = true;
                }
            }
            let sequence = e.mem.u32(current_sequence_at(this, 4));
            let action = if has_item { 0x11u32 } else { 9u32 };
            e.call(ACTOR_SET_ANIM_ACTION, &args![actor, action, sequence]);
        }
    }
    e.mem.set_u16(a + Animation::sQueuedReloadGroup.off, 0xff);
}

/// The countdown of `replayDelayList`: every delay loses `delta_time`, and
/// the ones that reach zero are removed and freed.
fn update_replay_delays(e: &mut Engine, this: Ptr<Animation>, delta_time: f32) {
    let list = this.addr() + Animation::replayDelayList.off;
    if e.call(SIMPLE_LIST_IS_EMPTY, &args![list]).bool() {
        return;
    }
    let mut node = list;
    while node != 0 {
        let cell = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
        let delay = e.mem.u32(cell);
        let remaining = (e.mem.f32(delay + 4) as f64 - delta_time as f64) as f32;
        e.mem.set_f32(delay + 4, remaining);
        if remaining <= 0.0 {
            node = if node == list {
                0
            } else {
                e.call(SIMPLE_LIST_NEXT, &args![node]).u32()
            };
            let item = e.with_stack(4, |e, item| {
                e.mem.set_u32(item.addr(), delay);
                e.call(SIMPLE_LIST_REMOVE_ITEM, &args![list, item]);
                e.mem.u32(item.addr())
            });
            e.call(OPERATOR_DELETE, &args![item]);
            if node == 0 && !e.call(SIMPLE_LIST_IS_EMPTY, &args![list]).bool() {
                node = list;
            }
        } else {
            node = e.call(SIMPLE_LIST_NEXT, &args![node]).u32();
        }
    }
}

/// The two `spAnimIdleFreeWhenInactiveA` idles: frees one whose sequence is
/// gone, and complains about (and drops) one that is still animating.
fn update_free_idles(e: &mut Engine, this: Ptr<Animation>) {
    let a = this.addr();
    for slot in 0..2u32 {
        let field = a + Animation::spAnimIdleFreeWhenInactiveA.off + slot * 4;
        if ni_pointer_get(e, field) == 0 {
            continue;
        }
        let idle = ni_pointer_get(e, field);
        let sequence = fn_00490e40(e, Ptr::new(idle));
        if sequence.is_null() {
            let idle = ni_pointer_get(e, field);
            if e.call(WORD_AT_8, &args![idle]).u32() != 0 {
                e.call(ANIM_IDLE_FREE, &args![this, field]);
            }
            continue;
        }
        let idle = ni_pointer_get(e, field);
        let sequence = fn_00490e40(e, Ptr::new(idle));
        if e.call(SEQUENCE_STATE, &args![sequence]).i32() == 1 {
            let idle = ni_pointer_get(e, field);
            let owner = e.call(MANAGER_TARGET, &args![idle]).u32();
            let idle = ni_pointer_get(e, field);
            let sequence = fn_00490e40(e, Ptr::new(idle));
            let name = object_name(e, sequence.addr());
            // Both virtual functions at +0x130 take no argument: `name` and
            // the first result stay on the stack as the log's arguments.
            let owner_text = e.vcall(owner, 0x130, &args![]).u32();
            let reference = e.get(this, Animation::pActorRef).addr();
            let reference_text = e.vcall(reference, 0x130, &args![]).u32();
            e.call(
                LOG,
                &args![LOG_IDLE_FREE_ANIMATING, reference_text, owner_text, name],
            );
            e.call(NI_POINTER_SET, &args![field, 0u32]);
        } else {
            let idle = ni_pointer_get(e, field);
            let sequence = fn_00490e40(e, Ptr::new(idle));
            if e.call(SEQUENCE_STATE, &args![sequence]).i32() == 0 {
                e.call(ANIM_IDLE_FREE, &args![this, field]);
            }
        }
    }
}

/// An `NiUpdateData` (Xbox PDB: `fTime`, `bUpdateControllers`,
/// `bParallelUpdate`, ...; `0043d410` fills `fTime` and the first two flags)
/// for `time`, applied to the animation root with `NODE_UPDATE`.
fn update_root_with_time(e: &mut Engine, this: Ptr<Animation>, time: f32, first_flag: u32) {
    e.with_stack(12, |e, data| {
        e.call(UPDATE_DATA_CONSTRUCT, &args![data, time, first_flag, 0u32]);
        let root = ni_pointer_get(e, this.addr() + Animation::pAnimRoot.off);
        e.call(NODE_UPDATE, &args![root, data]);
    });
}

// Translated from 00491180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::Update` (Xbox PDB): one frame of the animation. In order:
/// adds the queued KF models (up to the setting `SETTING_CLONING` allows);
/// otherwise plays a queued reload group; counts down the replay delays;
/// frees the idles that are done; then, with an animation root, either sets
/// the time and updates the root (when `time_override` is not -1, or when
/// `cSkipUpdate` is 0x14) or advances `time` by `delta_time` scaled by the
/// global time multiplier. For every slot of the eight it applies the
/// per-type speed modifiers to the sequence's phase and steps the slot's
/// action (the stage of the animation: intro, loop, outro) according to the
/// category of the group's type, starting the next group or blending out
/// when a stage ends. Then the accumulation root's translation gives the
/// movement of the frame (`movementDelta`, scaled by the reference's scale;
/// a special override when the actor's flags ask for one), the notes of the
/// groups are played for the slots whose scaled time moved, and the idle
/// state step runs. `cSkipUpdate` returns to 0xff.
///
/// `actor` is the reference being animated (its virtual function at +0x100
/// says whether it has a process), `delta_time` the frame time.
/// The compiler's exception-unwinding frame is not translated.
pub fn animation_update(
    e: &mut Engine,
    this: Ptr<Animation>,
    actor: Ptr,
    delta_time: f32,
    time_override: f32,
) {
    let a = this.addr();
    let time_scale = e.call(TIME_SCALE_GET, &args![TIME_SCALE_OBJECT]).f64();
    let multiplier = e.get(this, Animation::m_fGlobalTimeMultiplier);
    let scaled_time = (time_scale * multiplier as f64) as f32;

    if e.call(MODEL_QUEUE_NOT_EMPTY, &args![this]).bool() {
        loop {
            let limit_address = e.call(SETTING_VALUE_ADDRESS, &args![SETTING_CLONING]).u32();
            let limit = e.mem.u32(limit_address);
            let counter: u32 = e.global(QUEUED_MODEL_COUNTER);
            if counter >= limit {
                break;
            }
            let queue = a + Animation::kfModelList.off;
            let cell = e.call(SIMPLE_LIST_ITEM, &args![queue]).u32();
            let model = e.mem.u32(cell);
            animation_add_animation(e, this, model, false);
            e.call(KF_MODEL_RELEASE, &args![model]);
            e.call(SIMPLE_LIST_POP_FRONT, &args![queue]);
            if !e.call(MODEL_QUEUE_NOT_EMPTY, &args![this]).bool() {
                break;
            }
            let counter: u32 = e.global(QUEUED_MODEL_COUNTER);
            e.set_global(QUEUED_MODEL_COUNTER, counter.wrapping_add(1));
        }
    } else if e.mem.u16(a + Animation::sQueuedReloadGroup.off) != 0xff {
        update_queued_reload(e, this, actor);
    }

    update_replay_delays(e, this, delta_time);

    // The actor counts as "with a process" when its virtual function at
    // +0x100 says so.
    let mut process_actor = 0u32;
    if !actor.is_null() && e.vcall(actor.addr(), SLOT_HAS_PROCESS, &args![]).bool() {
        process_actor = actor.addr();
    }
    update_free_idles(e, this);

    if ni_pointer_get(e, a + Animation::pAnimRoot.off) == 0 {
        return;
    }
    fn_00493860(e, this, 0);
    e.mem.set_f32(a + 0x10, 0.0);
    e.mem.set_f32(a + 0x14, 0.0);

    let skip_update = e.mem.i8(a + Animation::cSkipUpdate.off);
    let minus_one = double_constant(e, DOUBLE_MINUS_ONE);
    if skip_update == 0x14 {
        let time = if time_override as f64 == minus_one {
            e.get(this, Animation::time)
        } else {
            time_override
        };
        update_root_with_time(e, this, time, 0);
        return;
    }

    // The sequences and scaled times the slots had before this frame.
    let phase_time = delta_time;
    let mut sequences_before = [0u32; 8];
    let mut scaled_before = [0f32; 8];
    for slot in 0..8u32 {
        let group = e.mem.u16(group_at(this, slot));
        let mut current_type = group_type(e, group);
        let next_group = e.mem.u16(next_group_at(this, slot));
        let next_type = group_type(e, next_group);
        if current_type == 0xff && next_type != 0xff {
            start_next_group(e, this, slot);
            let group = e.mem.u16(group_at(this, slot));
            current_type = group_type(e, group);
        }
        let sequence = fn_00491040(e, this, slot).addr();
        sequences_before[slot as usize] = sequence;
        scaled_before[slot as usize] = 0.0;
        if current_type != 0xff && sequence != 0 {
            let time = e.get(this, Animation::time);
            scaled_before[slot as usize] =
                e.call(SEQUENCE_SCALED_TIME, &args![sequence, time]).f32();
        }
    }

    if time_override as f64 != minus_one {
        e.set(this, Animation::time, time_override);
        let time = e.get(this, Animation::time);
        update_root_with_time(e, this, time, 1);
        return;
    }

    let multiplier = e.get(this, Animation::m_fGlobalTimeMultiplier);
    let phase_time = (phase_time as f64 * multiplier as f64) as f32;
    let time = e.get(this, Animation::time);
    e.set(
        this,
        Animation::time,
        (time as f64 + phase_time as f64) as f32,
    );

    let player: u32 = e.global(PLAYER_SINGLETON);
    for slot in 0..8u32 {
        let sequence = fn_00491040(e, this, slot).addr();
        let mut apply_modifiers = true;
        let group = e.mem.u16(group_at(this, slot));
        let current_type = group_type(e, group);
        let next_group = e.mem.u16(next_group_at(this, slot));
        let next_type = group_type(e, next_group);
        if current_type == 0xff || sequence == 0 {
            continue;
        }
        if slot == 3 {
            apply_modifiers = false;
        }
        let hold = match e.mem.i8(a + Animation::cSkipUpdate.off) as i32 {
            4 => (4..=6).contains(&slot),
            0x14 => true,
            0x15 => (2..=6).contains(&slot),
            0x17 => slot != 7 && slot != 0,
            other => slot as i32 == other,
        };
        if hold {
            apply_modifiers = false;
        }

        if !apply_modifiers {
            let phase = (sequence_elapsed(e, sequence) - phase_time as f64) as f32;
            set_phase(e, sequence, phase);
        } else if e.call(SEQUENCE_STATE, &args![sequence]).i32() == 1 {
            // The speed modifier of the group's type, as an offset in `Animation`.
            let modifier = if (3..=0x10).contains(&current_type) {
                Some(Animation::m_fMoveSpeed.off)
            } else if current_type == 0x18 || current_type == 0x19 {
                Some(Animation::m_fEquipModifier.off)
            } else if (0x18..=0xa8).contains(&current_type) {
                Some(Animation::m_fAttackSpeed.off)
            } else if (0xb1..=0xc7).contains(&current_type) {
                Some(Animation::m_fReloadModifier.off)
            } else {
                None
            };
            if let Some(offset) = modifier {
                let elapsed = sequence_elapsed(e, sequence);
                let factor = e.mem.f32(a + offset);
                let phase =
                    (elapsed + (phase_time as f64 * factor as f64 - phase_time as f64)) as f32;
                set_phase(e, sequence, phase);
            }
        }

        if !(apply_modifiers && e.call(SEQUENCE_STATE, &args![sequence]).i32() == 1) {
            continue;
        }
        let category = type_column(e, SEQUENCE_TYPE_CATEGORY, current_type) as u32;
        if category > 10 {
            continue;
        }
        update_slot_stage(
            e,
            this,
            slot,
            sequence,
            current_type,
            next_type,
            process_actor,
            player,
            &mut scaled_before[slot as usize],
            category,
        );
    }

    update_movement_and_notes(
        e,
        this,
        actor,
        process_actor,
        scaled_time,
        &sequences_before,
        &scaled_before,
    );
}

/// The per-category stage stepping of `Update` for one slot (the jump
/// table on the category in the type table). `scaled_before` is the slot's
/// scaled time of the start of the frame, which some categories adjust.
#[allow(clippy::too_many_arguments)]
fn update_slot_stage(
    e: &mut Engine,
    this: Ptr<Animation>,
    slot: u32,
    sequence: u32,
    current_type: u32,
    next_type: u32,
    process_actor: u32,
    player: u32,
    scaled_before: &mut f32,
    category: u32,
) {
    let a = this.addr();
    match category {
        0 | 1 => {
            if e.call(SEQUENCE_CYCLE_TYPE, &args![sequence]).u32() == 0 {
                loop {
                    let action = e.mem.i32(action_at(this, slot));
                    if action >= 1 {
                        break;
                    }
                    let (section, elapsed) = section_and_elapsed(e, this, sequence, action);
                    if !section_is_before(section, elapsed) {
                        break;
                    }
                    let action = e.mem.i32(action_at(this, slot));
                    if action == 0 || action == 1 {
                        e.mem.set_i32(action_at(this, slot), 1);
                    }
                }
                if next_type != 0xff {
                    let (section, elapsed) = section_and_elapsed(e, this, sequence, 1);
                    if section <= elapsed {
                        start_next_group(e, this, slot);
                    }
                } else {
                    let (section, elapsed) = section_and_elapsed(e, this, sequence, 1);
                    if section <= elapsed && e.mem.i32(loop_count_at(this, slot)) != 0 {
                        e.mem.set_i32(action_at(this, slot), 0);
                        let end = e.call(SEQUENCE_END_TIME, &args![sequence]).f64();
                        let begin = e.call(SEQUENCE_BEGIN_TIME, &args![sequence]).f64();
                        let length = (end - begin) as f32;
                        let phase = (sequence_elapsed(e, sequence) - length as f64) as f32;
                        set_phase(e, sequence, phase);
                        let loops = e.mem.i32(loop_count_at(this, slot));
                        if loops != -1 {
                            e.mem.set_i32(loop_count_at(this, slot), loops - 1);
                        }
                    } else {
                        let (section, elapsed) = section_and_elapsed(e, this, sequence, 1);
                        if section <= elapsed {
                            blend_out(e, this, slot);
                        }
                    }
                }
            } else {
                if e.mem.i32(action_at(this, slot)) == 0 {
                    let action = e.mem.i32(action_at(this, slot));
                    let (section, elapsed) = section_and_elapsed(e, this, sequence, action);
                    if section < elapsed {
                        e.mem.set_i32(action_at(this, slot), 1);
                    }
                }
                if next_type != 0xff {
                    let (section, elapsed) = section_and_elapsed(e, this, sequence, 1);
                    if section <= elapsed {
                        let hands_free = slot == 1
                            && e.call(ACTOR_FLAGS_WORD, &args![process_actor]).u32() & 0xf == 0;
                        if hands_free {
                            blend_out(e, this, slot);
                        } else {
                            let loops = e.mem.u32(next_loops_at(this, slot));
                            e.mem.set_u32(loop_count_at(this, slot), loops);
                            let group = e.mem.u16(next_group_at(this, slot));
                            animation_start_group(e, this, group, -1);
                        }
                        e.mem.set_u16(next_group_at(this, slot), 0xff);
                    }
                } else {
                    let (section, elapsed) = section_and_elapsed(e, this, sequence, 1);
                    if section <= elapsed {
                        blend_out(e, this, slot);
                    }
                }
            }
        }
        2 => {
            loop {
                let action = e.mem.i32(action_at(this, slot));
                if action >= 3 {
                    break;
                }
                let (section, elapsed) = section_and_elapsed(e, this, sequence, action);
                if !section_is_before(section, elapsed) {
                    break;
                }
                let next = match e.mem.i32(action_at(this, slot)) {
                    0 => Some(1),
                    1 => Some(2),
                    2 => Some(3),
                    _ => None,
                };
                if let Some(next) = next {
                    e.mem.set_i32(action_at(this, slot), next);
                }
            }
            if next_type != 0xff {
                let (section, elapsed) = section_and_elapsed(e, this, sequence, 3);
                if section <= elapsed {
                    start_next_group(e, this, slot);
                }
            } else {
                let (section, elapsed) = section_and_elapsed(e, this, sequence, 2);
                if section <= elapsed && e.mem.i32(loop_count_at(this, slot)) != 0 {
                    e.call(RESET_CONTROLLERS, &args![this]);
                    e.mem.set_i32(action_at(this, slot), 1);
                    let group = fn_0048f7f0(e, Ptr::new(sequence));
                    let second = e.call(GROUP_TIME, &args![group, 2i32]).f64();
                    let group = fn_0048f7f0(e, Ptr::new(sequence));
                    let first = e.call(GROUP_TIME, &args![group, 1i32]).f64();
                    let length = (second - first) as f32;
                    let phase = (sequence_elapsed(e, sequence) - length as f64) as f32;
                    set_phase(e, sequence, phase);
                    *scaled_before = (*scaled_before as f64 - length as f64) as f32;
                    let loops = e.mem.i32(loop_count_at(this, slot));
                    if loops != -1 && loops < 0xff {
                        e.mem.set_i32(loop_count_at(this, slot), loops - 1);
                    }
                } else {
                    let (section, elapsed) = section_and_elapsed(e, this, sequence, 3);
                    if section <= elapsed {
                        let idle = ni_pointer_get(e, a + Animation::spAnimIdle.off);
                        if e.call(WORD_AT_C, &args![idle]).u32() == 1 {
                            let idle = ni_pointer_get(e, a + Animation::spAnimIdle.off);
                            e.call(IDLE_SET_WORD_8, &args![idle, 3u32]);
                        } else {
                            blend_out(e, this, slot);
                        }
                    }
                }
            }
        }
        3 | 4 | 6 | 10 => {
            advance_action(e, this, slot, sequence, 2);
            if e.mem.i32(action_at(this, slot)) == 2 {
                let (section, elapsed) = section_and_elapsed(e, this, sequence, 2);
                if section <= elapsed {
                    if next_type == 0xff {
                        blend_out(e, this, slot);
                        if process_actor != 0 {
                            let kind = type_column(e, SEQUENCE_TYPE_CATEGORY, current_type);
                            if kind == 3 || kind == 4 {
                                let process = e.call(ACTOR_PROCESS, &args![process_actor]).u32();
                                if process != 0 {
                                    if process_actor == player {
                                        e.call(ACTOR_SET_HAVOK_WEAPON, &args![process_actor]);
                                    } else {
                                        e.vcall(process, 0x614, &args![0x4000u32]);
                                    }
                                }
                            }
                        }
                    } else {
                        start_next_group(e, this, slot);
                    }
                }
            }
        }
        5 | 7 => finish_last_stage(e, this, slot, sequence, next_type, 4),
        8 => finish_last_stage(e, this, slot, sequence, next_type, 3),
        _ => {
            // Category 9.
            advance_action(e, this, slot, sequence, 3);
            if e.mem.i32(action_at(this, slot)) == 2 && process_actor != 0 {
                let process = e.call(ACTOR_PROCESS, &args![process_actor]).u32();
                if process != 0 {
                    let process = e.call(ACTOR_PROCESS, &args![process_actor]).u32();
                    let aiming = e.vcall(process, 0x6dc, &args![]).bool()
                        && e.call(ACTOR_GET_ANIM_ACTION, &args![process_actor]).i32() == 2;
                    let reference = e.get(this, Animation::pActorRef).addr();
                    let aiming = aiming
                        || (reference == player
                            && e.call(WORD_AT_8, &args![VATS_OBJECT]).u32() == 4
                            && e.call(VATS_CURRENT_ACTION, &args![VATS_OBJECT]).u32() != 0);
                    if aiming {
                        let group = fn_0048f7f0(e, Ptr::new(sequence));
                        let second = e.call(GROUP_TIME, &args![group, 2i32]).f64();
                        if *scaled_before as f64 <= second {
                            let group = fn_0048f7f0(e, Ptr::new(sequence));
                            let second = e.call(GROUP_TIME, &args![group, 2i32]).f64();
                            let group = fn_0048f7f0(e, Ptr::new(sequence));
                            let first = e.call(GROUP_TIME, &args![group, 1i32]).f64();
                            let length = (second - first) as f32;
                            loop {
                                let action = e.mem.i32(action_at(this, slot));
                                let (section, elapsed) =
                                    section_and_elapsed(e, this, sequence, action);
                                if !section_is_before(section, elapsed) {
                                    break;
                                }
                                let phase = (sequence_elapsed(e, sequence) - length as f64) as f32;
                                set_phase(e, sequence, phase);
                                *scaled_before = (*scaled_before as f64 - length as f64) as f32;
                            }
                            let action = e.mem.i32(action_at(this, slot));
                            e.mem.set_i32(action_at(this, slot), action - 1);
                        }
                    }
                }
            }
            let flag: u8 = e.global(SEQUENCE_STEP_FLAG);
            if flag != 0 && e.mem.i32(action_at(this, slot)) == 1 {
                let group = fn_0048f7f0(e, Ptr::new(sequence));
                let second = e.call(GROUP_TIME, &args![group, 2i32]).f64();
                if *scaled_before as f64 <= second {
                    let group = fn_0048f7f0(e, Ptr::new(sequence));
                    let second = e.call(GROUP_TIME, &args![group, 2i32]).f64();
                    let group = fn_0048f7f0(e, Ptr::new(sequence));
                    let first = e.call(GROUP_TIME, &args![group, 1i32]).f64();
                    let length = (second - first) as f32;
                    let phase = (sequence_elapsed(e, sequence) + length as f64) as f32;
                    set_phase(e, sequence, phase);
                    *scaled_before = (*scaled_before as f64 + length as f64) as f32;
                    let action = e.mem.i32(action_at(this, slot));
                    e.mem.set_i32(action_at(this, slot), action + 1);
                }
            }
            if e.mem.i32(action_at(this, slot)) == 3 {
                let (section, elapsed) = section_and_elapsed(e, this, sequence, 3);
                if section <= elapsed {
                    if next_type == 0xff {
                        blend_out(e, this, slot);
                    } else {
                        start_next_group(e, this, slot);
                    }
                }
            }
        }
    }
}

/// While `action[slot]` is below `last`, moves it up one when the section
/// after it has passed (`GetTime(action + 1) < elapsed`).
fn advance_action(e: &mut Engine, this: Ptr<Animation>, slot: u32, sequence: u32, last: i32) {
    if e.mem.i32(action_at(this, slot)) < last {
        let action = e.mem.i32(action_at(this, slot));
        let (section, elapsed) = section_and_elapsed(e, this, sequence, action + 1);
        if section < elapsed {
            let action = e.mem.i32(action_at(this, slot));
            e.mem.set_i32(action_at(this, slot), action + 1);
        }
    }
}

/// The ending shared by categories 5, 7 and 8: after `advance_action` has
/// brought the stage to `last`, the slot blends out or hands over to the
/// next group when that section has passed.
fn finish_last_stage(
    e: &mut Engine,
    this: Ptr<Animation>,
    slot: u32,
    sequence: u32,
    next_type: u32,
    last: i32,
) {
    advance_action(e, this, slot, sequence, last);
    if e.mem.i32(action_at(this, slot)) == last {
        let (section, elapsed) = section_and_elapsed(e, this, sequence, last);
        if section <= elapsed {
            if next_type == 0xff {
                blend_out(e, this, slot);
            } else {
                start_next_group(e, this, slot);
            }
        }
    }
}

/// The end of `Update`: the movement of the frame (`movementDelta`) from the
/// accumulation root's translation, the notes of the slots whose scaled time
/// moved, the idle state step and the reset of the one-frame flags.
fn update_movement_and_notes(
    e: &mut Engine,
    this: Ptr<Animation>,
    actor: Ptr,
    process_actor: u32,
    scaled_time: f32,
    sequences_before: &[u32; 8],
    scaled_before: &[f32; 8],
) {
    let a = this.addr();
    let player: u32 = e.global(PLAYER_SINGLETON);
    let accum_root = e.get(this, Animation::pAccumRoot).addr();
    if accum_root != 0 {
        let translate = a + Animation::AccumRootTranslate.off;
        let old_translate = [
            e.mem.f32(translate),
            e.mem.f32(translate + 4),
            e.mem.f32(translate + 8),
        ];
        let mut use_override = false;
        let mut override_vector = [
            e.mem.f32(ZERO_VECTOR),
            e.mem.f32(ZERO_VECTOR + 4),
            e.mem.f32(ZERO_VECTOR + 8),
        ];
        if e.call(ACTOR_FLAGS_ANY, &args![process_actor]).bool() {
            let mask = e.call(ACTOR_FLAGS_WORD, &args![process_actor]).u16();
            let first = e.mem.u32(current_sequence_at(this, 1));
            if first != 0 && mask & 0xf != 0 {
                let group = fn_0048f7f0(e, Ptr::new(first));
                if e.call(GROUP_IS_SPECIAL_TYPE, &args![group]).bool() {
                    use_override = true;
                }
                let first = e.mem.u32(current_sequence_at(this, 1));
                let group = fn_0048f7f0(e, Ptr::new(first));
                let kind = e.call(ANIM_GROUP_SEQUENCE_TYPE, &args![group]).i32();
                if kind == 0xe3 {
                    use_override = true;
                } else {
                    let first = e.mem.u32(current_sequence_at(this, 1));
                    let state = e.call(SEQUENCE_STATE, &args![first]).i32();
                    if state == 2 || state == 5 {
                        use_override = true;
                    }
                }
                let controller = e
                    .call(GET_CONTROLLER, &args![accum_root, RTTI_ACCUM_CONTROLLER])
                    .u32();
                // The interpolator that the accumulation root's controller
                // blends for this frame; its priority (the signed byte at
                // +0x10) beats the sequence's for the root bone.
                let blend = e.vcall(controller, 0xc4, &args![0u32]).u32();
                if blend != 0 {
                    let priority = fn_00493750(e, Ptr::new(blend));
                    let first = e.mem.u32(current_sequence_at(this, 1));
                    let target = e.call(SEQUENCE_INTERPOLATOR_AT, &args![first, 0u32]).u32();
                    // The cast's result is stored in a local that is never read.
                    e.call(DYNAMIC_CAST, &args![RTTI_CAST_TARGET, target]);
                    let root_name = object_name(e, accum_root);
                    let first = e.mem.u32(current_sequence_at(this, 1));
                    let sequence_priority = e
                        .call(SEQUENCE_PRIORITY, &args![first, root_name, 0u32])
                        .u8() as i32;
                    if sequence_priority < priority {
                        use_override = false;
                    }
                }
                if use_override {
                    let mut wanted = 0u32;
                    if mask & 0x200 != 0 {
                        wanted = if mask & 1 != 0 {
                            7
                        } else if mask & 2 != 0 {
                            8
                        } else if mask & 4 != 0 {
                            9
                        } else if mask & 8 != 0 {
                            10
                        } else {
                            0
                        };
                    } else if mask & 0xff00 != 0 {
                        wanted = if mask & 1 != 0 {
                            3
                        } else if mask & 2 != 0 {
                            4
                        } else if mask & 4 != 0 {
                            5
                        } else if mask & 8 != 0 {
                            6
                        } else {
                            0
                        };
                    }
                    let first = e.mem.u32(current_sequence_at(this, 1));
                    let group = fn_0048f7f0(e, Ptr::new(first));
                    let kind = e.call(ANIM_GROUP_SEQUENCE_TYPE, &args![group]).u32();
                    if wanted == kind {
                        let words = scaled_group_movement(e, this, group, scaled_time);
                        override_vector = words.map(f32::from_bits);
                    } else {
                        let found_group = e
                            .call(
                                ACTOR_GET_ANIM_GROUP,
                                &args![process_actor, wanted, 0u32, 0u32, 0u32],
                            )
                            .u16();
                        let found_type = group_type(e, found_group);
                        if found_type != 0 {
                            if let Some(entry) = sequence_map_get(e, this, found_group) {
                                let sequence = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32();
                                if sequence != 0 {
                                    let group = fn_0048f7f0(e, Ptr::new(sequence));
                                    let words = scaled_group_movement(e, this, group, scaled_time);
                                    override_vector = words.map(f32::from_bits);
                                }
                            }
                        }
                    }
                }
            }
        }
        let time = e.get(this, Animation::time);
        e.call(UPDATE_BIP_ONLY, &args![this, time, translate, 1u32]);
        if use_override {
            e.mem.set_f32(a + 0x10, override_vector[0]);
            e.mem.set_f32(a + 0x14, override_vector[1]);
        } else {
            e.with_stack(24, |e, buffers| {
                let (out, old) = (buffers.addr(), buffers.addr() + 12);
                for (i, value) in old_translate.iter().enumerate() {
                    e.mem.set_f32(old + 4 * i as u32, *value);
                }
                e.call(VECTOR_SUBTRACT, &args![translate, out, old]);
                let (x, y) = (e.mem.f32(out), e.mem.f32(out + 4));
                e.mem.set_f32(a + 0x10, x);
                e.mem.set_f32(a + 0x14, y);
            });
        }
        let scale = e.call(REFERENCE_SCALE, &args![actor]).f64();
        let x = e.mem.f32(a + 0x10);
        e.mem.set_f32(a + 0x10, (scale * x as f64) as f32);
        let scale = e.call(REFERENCE_SCALE, &args![actor]).f64();
        let y = e.mem.f32(a + 0x14);
        e.mem.set_f32(a + 0x14, (scale * y as f64) as f32);
        let last_address = a + Animation::pLastMovementSequence.off;
        if !use_override
            && e.mem.u32(current_sequence_at(this, 1)) == 0
            && e.mem.u32(last_address) != 0
        {
            let last = e.mem.u32(last_address);
            if e.call(SEQUENCE_STATE, &args![last]).u32() != 0 {
                e.mem.set_f32(a + 0x14, 0.0);
                e.mem.set_f32(a + 0x10, 0.0);
            }
        }
        if e.mem.u32(last_address) != 0 {
            let last = e.mem.u32(last_address);
            if e.call(SEQUENCE_STATE, &args![last]).u32() == 0 {
                e.mem.set_u32(last_address, 0);
            }
        }
    }

    let mut notes_blocked = false;
    if process_actor != 0 && e.call(ACTOR_PROCESS, &args![process_actor]).u32() != 0 {
        let process = e.call(ACTOR_PROCESS, &args![process_actor]).u32();
        notes_blocked = e.vcall(process, 0x2d8, &args![]).bool();
    }
    if !notes_blocked {
        let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
        let mut play_notes = !e.call(NODE_FLAG_CHECK, &args![root]).bool();
        if !play_notes {
            let player_animation = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
            play_notes =
                this.addr() == player_animation && e.call(SOUND_FLAG_CHECK, &args![]).bool();
        }
        if play_notes {
            for slot in 0..8u32 {
                let skip = e.mem.u8(a + Animation::cSkipUpdate.off);
                let step = match skip {
                    0x14 => false,
                    0x15 => !(2..=4).contains(&slot),
                    0x17 => slot == 7 || slot == 0,
                    _ => slot as i32 != e.mem.i8(a + Animation::cSkipUpdate.off) as i32,
                };
                if step {
                    play_slot_notes(
                        e,
                        this,
                        actor,
                        slot,
                        sequences_before[slot as usize],
                        scaled_before[slot as usize],
                    );
                }
            }
        }
    }

    let idle = ni_pointer_get(e, a + Animation::spAnimIdle.off);
    if idle != 0 {
        let idle = ni_pointer_get(e, a + Animation::spAnimIdle.off);
        e.call(IDLE_STATE_STEP, &args![idle, this]);
    }
    if e.mem.i8(a + Animation::cSkipNextBlend.off) != 0 {
        e.mem.set_u8(a + Animation::cSkipNextBlend.off, 0);
    }
    e.mem.set_u8(a + Animation::cSkipUpdate.off, 0xff);
}

/// One slot of the notes pass at the end of `Update`: when the slot still
/// plays the sequence it played before the frame (in a state that animates),
/// no other sequence outranks it for the slot's sound bone, and its scaled
/// time moved, the group's notes between the two times are played
/// (`005f2b60(actor, from, to, sequence)`).
fn play_slot_notes(
    e: &mut Engine,
    this: Ptr<Animation>,
    actor: Ptr,
    slot: u32,
    sequence_before: u32,
    scaled_before: f32,
) {
    let sequence = fn_00491040(e, this, slot).addr();
    if sequence == 0 || sequence_before != sequence {
        return;
    }
    let animating = e.call(SEQUENCE_STATE, &args![sequence]).i32() == 1
        || e.call(SEQUENCE_STATE, &args![sequence]).i32() == 2
        || e.call(SEQUENCE_STATE, &args![sequence]).i32() == 5;
    if !animating {
        return;
    }
    let mut owns_bone = true;
    let bone_name = e.mem.u32(SOUND_PRIORITY_BONE_NAMES + slot * 4);
    let bone_at = animation_entry(this, Animation::pSoundPriorityBone, slot, 4);
    if bone_name == 0 {
        owns_bone = false;
    } else if e.mem.u32(bone_at) != 0 {
        let mut best = 0u32;
        let mut best_priority = 0u8;
        for other in 0..8u32 {
            if e.mem.u32(current_sequence_at(this, other)) != 0 && e.mem.u32(bone_at) != 0 {
                let bone = e.mem.u32(bone_at);
                let name = object_name(e, bone);
                if name != 0 {
                    let candidate = e.mem.u32(current_sequence_at(this, other));
                    let group = fn_0048f7f0(e, Ptr::new(candidate));
                    let priority = e.call(GROUP_BONE_PRIORITY, &args![group, slot]).u8();
                    if priority > best_priority {
                        best = candidate;
                        best_priority = priority;
                    }
                }
            }
        }
        if best == 0 || best != sequence {
            owns_bone = false;
        }
    }
    if !owns_bone {
        return;
    }
    let time = e.get(this, Animation::time);
    let current = e.call(SEQUENCE_SCALED_TIME, &args![sequence, time]).f32();
    let lowest = -(e.global::<f32>(FLOAT_MAX) as f64);
    if current as f64 != lowest && scaled_before as f64 != lowest && scaled_before != current {
        let group = fn_0048f7f0(e, Ptr::new(sequence));
        e.call(
            GROUP_PLAY_NOTES,
            &args![group, actor, scaled_before, current, sequence],
        );
    }
}

// ---- Flags, movement and scene graph updates -------------------------------------

// Translated from 00493750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The signed byte at +0x10 of the object (`Update` calls it on the object
/// the accumulation root's controller blends and compares the result with a
/// sequence's `GetPriority`); the map has no name for it.
pub fn fn_00493750(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.i8(this.addr() + 0x10) as i32
}

// Translated from 00493860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` != 0) or clears the "movement updated" bit (1) of the
/// animation's flag byte; the map has no name for it.
pub fn fn_00493860(e: &mut Engine, this: Ptr<Animation>, value: u8) {
    e.call(FLAG_SET, &args![this, value, FLAG_MOVEMENT_UPDATED]);
}

// Translated from 00493900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::UpdateMovement` (Xbox PDB): `UpdateMovementNoWorldUpdate`
/// for `actor`, then `UpdateSceneGraphNoController`.
pub fn animation_update_movement(e: &mut Engine, this: Ptr<Animation>, actor: Ptr) {
    animation_update_movement_no_world_update(e, this, actor);
    animation_update_scene_graph_no_controller(e, this);
}

// Translated from 00493930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::UpdateQueuedScenegraph` (Xbox PDB): when the scene graph
/// update was postponed (flag bit 2), does it now and clears the bit.
/// Returns whether it did.
pub fn animation_update_queued_scenegraph(e: &mut Engine, this: Ptr<Animation>) -> bool {
    if !fn_00493970(e, this) {
        return false;
    }
    animation_update_scene_graph_no_controller(e, this);
    fn_004939b0(e, this, 0);
    true
}

// Translated from 00493970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag bit 2 (the scene graph update was postponed) is set; the
/// map has no name for it.
pub fn fn_00493970(e: &mut Engine, this: Ptr<Animation>) -> bool {
    fn_00493990(e, this, FLAG_SCENE_GRAPH_PENDING as u8)
}

// Translated from 00493990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether any bit of `mask` is set in the flag byte at +0.
pub fn fn_00493990(e: &mut Engine, this: Ptr<Animation>, mask: u8) -> bool {
    e.mem.u8(this.addr()) & mask != 0
}

// Translated from 004939b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` != 0) or clears flag bit 2 (the scene graph update is
/// postponed); the map has no name for it.
pub fn fn_004939b0(e: &mut Engine, this: Ptr<Animation>, value: u8) {
    e.call(FLAG_SET, &args![this, value, FLAG_SCENE_GRAPH_PENDING]);
}

// Translated from 004939d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::UpdateMovementNoWorldUpdate` (Xbox PDB): with an animation
/// root, an accumulation root and the "movement updated" bit clear, sets the
/// actor's flag bits (1 always, 2 from the shader accumulator test of the
/// actor's form ID, 4 from the speed of its movement object) and updates the
/// biped under the animation root for `time`; `movementDelta.z` becomes the
/// change of the accumulation root's first child's world height. Finally
/// sets the "movement updated" bit.
pub fn animation_update_movement_no_world_update(e: &mut Engine, this: Ptr<Animation>, actor: Ptr) {
    let a = this.addr();
    if ni_pointer_get(e, a + Animation::pAnimRoot.off) == 0 {
        return;
    }
    if e.get(this, Animation::pAccumRoot).is_null() {
        return;
    }
    if fn_00493b90(e, this) {
        return;
    }
    let player: u32 = e.global(PLAYER_SINGLETON);
    // The actor's process gives a movement object (virtual functions at
    // +0x1d0 of the actor and +0x10 of the process).
    let process = e.vcall(actor.addr(), 0x1d0, &args![]).u32();
    let movement = if process != 0 {
        e.vcall(process, 0x10, &args![]).u32()
    } else {
        0
    };
    let check_speed = actor.addr() == player
        || (e.call(ACTOR_WORD_108, &args![actor]).i32() != 1 && fn_00493bb0(e, actor) == 0);
    if check_speed && movement != 0 {
        // The game compares the speed with 0.0 and keeps the answer in a
        // local it never reads again.
        e.call(MOVEMENT_SPEED, &args![movement]);
    }
    e.call(ACTOR_FLAG_SET, &args![actor, 1u32, 1u32]);
    let accumulator = e.call(SHADER_ACCUMULATOR, &args![]).u32();
    let mut accumulator_accepts = false;
    if accumulator != 0 {
        let form_id = e.call(WORD_AT_C, &args![actor]).u32();
        accumulator_accepts = e
            .call(SHADER_ACCUMULATOR_TEST, &args![accumulator, form_id, 0u32])
            .bool();
    }
    e.call(
        ACTOR_FLAG_SET,
        &args![actor, !accumulator_accepts as u32, 2u32],
    );
    let slow = movement != 0 && {
        let speed = e.call(MOVEMENT_SPEED, &args![movement]).f64();
        speed < double_constant(e, DOUBLE_MICRO)
    };
    e.call(ACTOR_FLAG_SET, &args![actor, !slow as u32, 4u32]);

    let accum_root = e.get(this, Animation::pAccumRoot).addr();
    let child = e.call(NODE_CHILD_AT, &args![accum_root, 0u32]).u32();
    let translation = e.call(NODE_WORLD_TRANSLATION, &args![child]).u32();
    let height_before = e.mem.f32(translation + 8);
    let time: f32 = e.get(this, Animation::time);
    let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
    e.call(BIP_UPDATE_ALL_BUT_BIP, &args![root, time]);
    let accum_root = e.get(this, Animation::pAccumRoot).addr();
    let child = e.call(NODE_CHILD_AT, &args![accum_root, 0u32]).u32();
    let translation = e.call(NODE_WORLD_TRANSLATION, &args![child]).u32();
    let height_after = e.mem.f32(translation + 8);
    e.mem.set_f32(
        a + 0x18,
        (height_after as f64 - height_before as f64) as f32,
    );
    fn_00493860(e, this, 1);
}

// Translated from 00493b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether flag bit 1 (the movement was updated) is set; the map has no
/// name for it.
pub fn fn_00493b90(e: &mut Engine, this: Ptr<Animation>) -> bool {
    fn_00493990(e, this, FLAG_MOVEMENT_UPDATED as u8)
}

// Translated from 00493bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x104 of the object (of an actor, in the callers); the map
/// has no name for it.
pub fn fn_00493bb0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x104)
}

// Translated from 00493bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::UpdateSceneGraphNoController` (Xbox PDB): with an animation
/// root and an accumulation root, either postpones the scene graph update
/// (sets flag bit 2) when the limit setting is above 1 and nothing forces
/// the update, or updates the animation root's scene graph with the
/// controllers of the animation root and the accumulation root detached
/// for the duration and restored afterwards.
pub fn animation_update_scene_graph_no_controller(e: &mut Engine, this: Ptr<Animation>) {
    let a = this.addr();
    if ni_pointer_get(e, a + Animation::pAnimRoot.off) == 0 {
        return;
    }
    if e.get(this, Animation::pAccumRoot).is_null() {
        return;
    }
    let limit_address = e
        .call(SETTING_VALUE_ADDRESS, &args![SETTING_SCENE_GRAPH_LIMIT])
        .u32();
    let limit = e.mem.i32(limit_address);
    let forced: i8 = e.global(SCENE_GRAPH_ALWAYS_UPDATE);
    let flag_object: u32 = e.global(FLAG_OBJECT_GLOBAL);
    if limit > 1
        && forced == 0
        && !fn_00493970(e, this)
        && !e.call(FLAG_OBJECT_CHECK, &args![flag_object]).bool()
    {
        fn_004939b0(e, this, 1);
        return;
    }
    let time: f32 = e.get(this, Animation::time);
    e.with_stack(12, |e, data| {
        e.call(UPDATE_DATA_CONSTRUCT, &args![data, time, 1u32, 0u32]);
        let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
        let root_controllers = e.call(OBJECT_CONTROLLERS, &args![root]).u32();
        e.with_stack(4, |e, saved_root| {
            e.call(NI_POINTER_INIT, &args![saved_root, root_controllers]);
            let accum_root = e.get(this, Animation::pAccumRoot).addr();
            let accum_controllers = e.call(OBJECT_CONTROLLERS, &args![accum_root]).u32();
            e.with_stack(4, |e, saved_accum| {
                e.call(NI_POINTER_INIT, &args![saved_accum, accum_controllers]);
                let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
                e.call(OBJECT_SET_CONTROLLERS, &args![root, 0u32]);
                let accum_root = e.get(this, Animation::pAccumRoot).addr();
                e.call(OBJECT_SET_CONTROLLERS, &args![accum_root, 0u32]);
                let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
                e.vcall(root, 0xa4, &args![data, 0u32]);
                let controllers = ni_pointer_get(e, saved_root.addr());
                let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
                e.call(OBJECT_SET_CONTROLLERS, &args![root, controllers]);
                let controllers = ni_pointer_get(e, saved_accum.addr());
                let accum_root = e.get(this, Animation::pAccumRoot).addr();
                e.call(OBJECT_SET_CONTROLLERS, &args![accum_root, controllers]);
                e.call(NI_POINTER_RELEASE, &args![saved_accum]);
            });
            e.call(NI_POINTER_RELEASE, &args![saved_root]);
        });
    });
}

// ---- Group speed and small accessors ---------------------------------------------

// Translated from 00493d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::InitGroupSpeed` (Xbox PDB): for animation group `group`,
/// when the animation has a manager and a sequence for the group (and the
/// group is neither type 1 nor 2), finds the sequence's interpolator that
/// targets the accumulation root; for the sequence categories that move, it
/// samples the interpolator's transform at the sequence's start and end
/// times and stores `(end - start) / duration` as the group's movement vector
/// (`005f4c40`). Logs a message when there is no accumulation root, and when
/// a group of type 3..14 ends up with a zero speed ("exported with Animate
/// in Place").
pub fn animation_init_group_speed(e: &mut Engine, this: Ptr<Animation>, group: Ptr) {
    let a = this.addr();
    let group_id = e.call(ANIM_GROUP_ID, &args![group]).u16();
    let kind = e.call(ANIM_GROUP_SEQUENCE_TYPE, &args![group]).i32();
    if ni_pointer_get(e, a + Animation::spManager.off) == 0 {
        return;
    }
    let Some(entry) = sequence_map_get(e, this, group_id) else {
        return;
    };
    if kind == 1 || kind == 2 {
        return;
    }
    let manager = ni_pointer_get(e, a + Animation::spManager.off);
    let accum_root = ni_controller_manager_get_accum_root(e, Ptr::new(manager));
    if accum_root.is_null() {
        let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
        let name = object_name(e, root);
        e.call(LOG, &args![LOG_NO_ACCUM_ROOT, name]);
        return;
    }
    // Only an entry that holds a sequence, whose type keeps a single
    // sequence and whose category moves (0..2, 5..9) is measured. The game
    // also stores the first and last stage of the category in two locals it
    // never reads.
    let measured = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32() != 0
        && e.mem
            .u8(SEQUENCE_TYPE_TABLE.wrapping_add((kind as u32).wrapping_mul(0x24)))
            == 0
        && matches!(
            type_column(e, SEQUENCE_TYPE_CATEGORY, kind as u32),
            0..=2 | 5..=9
        );
    if measured {
        let sequence = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32();
        e.with_stack(4, |e, sequence_ref| {
            e.call(NI_POINTER_INIT, &args![sequence_ref, sequence]);
            let mut index = 0u32;
            loop {
                let sequence = ni_pointer_get(e, sequence_ref.addr());
                let count = e.call(WORD_AT_C, &args![sequence]).u32();
                if index >= count {
                    break;
                }
                let sequence = ni_pointer_get(e, sequence_ref.addr());
                let target = fn_00494210(e, Ptr::new(sequence), index);
                let sequence = ni_pointer_get(e, sequence_ref.addr());
                let accum = e.call(ACCUM_ROOT_OF_SEQUENCE, &args![sequence]).u32();
                if target != accum {
                    index += 1;
                    continue;
                }
                let sequence = ni_pointer_get(e, sequence_ref.addr());
                let interpolator = e
                    .call(SEQUENCE_INTERPOLATOR_AT, &args![sequence, index])
                    .u32();
                let sequence = ni_pointer_get(e, sequence_ref.addr());
                let begin = e.call(SEQUENCE_BEGIN_TIME, &args![sequence]).f32();
                let sequence = ni_pointer_get(e, sequence_ref.addr());
                let end = e.call(SEQUENCE_END_TIME, &args![sequence]).f32();
                e.with_stack(0x80, |e, records| {
                    let (start, finish, difference) =
                        (records.addr(), records.addr() + 0x20, records.addr() + 0x40);
                    fn_00494260(e, Ptr::new(start));
                    fn_00494260(e, Ptr::new(finish));
                    e.vcall(interpolator, 0x8c, &args![begin, 0u32, start]);
                    e.vcall(interpolator, 0x8c, &args![end, 0u32, finish]);
                    if fn_004942c0(e, Ptr::new(start)) && fn_004942c0(e, Ptr::new(finish)) {
                        let start_translate = e.call(RECORD_ADDRESS, &args![start]).u32();
                        let finish_translate = e.call(RECORD_ADDRESS, &args![finish]).u32();
                        e.call(
                            VECTOR_SUBTRACT,
                            &args![finish_translate, difference, start_translate],
                        );
                        let duration = (end as f64 - begin as f64) as f32;
                        fn_004941c0(e, Ptr::new(difference), duration);
                        e.call(GROUP_SET_MOVEMENT_VECTOR, &args![group, difference]);
                    }
                });
                break;
            }
            e.call(NI_POINTER_RELEASE, &args![sequence_ref]);
        });
    }
    let kind = group_type(e, group_id);
    if (3..=0xe).contains(&kind) {
        let speed = e.call(GROUP_SPEED, &args![group]).f64();
        if speed == double_constant(e, DOUBLE_ZERO) {
            let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
            let name = object_name(e, root);
            let type_of_group = e.call(ANIM_GROUP_SEQUENCE_TYPE, &args![group]).u32();
            let type_name = e
                .mem
                .u32(SEQUENCE_TYPE_NAME.wrapping_add(type_of_group.wrapping_mul(0x24)));
            let weapon = e.call(GROUP_WEAPON_TYPE, &args![group]).u32();
            let weapon_name = e
                .mem
                .u32(WEAPON_NAME_TABLE.wrapping_add(weapon.wrapping_mul(4)));
            let movement = e.call(GROUP_MOVE_TYPE, &args![group]).u32();
            let movement_name = e
                .mem
                .u32(MOVE_NAME_TABLE.wrapping_add(movement.wrapping_mul(4)));
            e.call(
                LOG,
                &args![
                    LOG_ANIMATE_IN_PLACE,
                    movement_name,
                    weapon_name,
                    type_name,
                    name
                ],
            );
        }
    }
}

// Translated from 004941c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Multiplies the three floats at `this` by `1.0 / divisor` and returns
/// `this` (a vector scaled by the inverse of its argument); the map has no
/// name for it.
pub fn fn_004941c0(e: &mut Engine, this: Ptr, divisor: f32) -> Ptr {
    // `FLD1`, `FDIVRP`, `FSTP` the scratch float; then `FLD` each lane, `FMUL` the scratch.
    let inverse = x87_store32(x87_div(1.0, x87_load32(divisor.to_bits())));
    for i in 0..3u32 {
        let address = this.addr() + 4 * i;
        let value = x87_load32(e.mem.u32(address));
        e.mem
            .set_u32(address, x87_store32(x87_mul(value, x87_m32(inverse))));
    }
    this
}

// Translated from 00494210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The target of the object held in entry `index` of the array at +0x14 of
/// `this` (a `NiControllerSequence`'s interpolator array): the word at +0x2c
/// of the pointer in the 16-byte entry's second word, 0 when that is null.
pub fn fn_00494210(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let array = e.mem.u32(this.addr() + 0x14);
    let entry = array.wrapping_add(index << 4).wrapping_add(4);
    if ni_pointer_get(e, entry) == 0 {
        return 0;
    }
    let object = ni_pointer_get(e, entry);
    e.call(MANAGER_TARGET, &args![object]).u32()
}

// Translated from 00494260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the 32-byte record at `this` the way an `NiQuatTransform`
/// (Xbox PDB layout: `m_kTranslate`, `m_kRotate`, `m_fScale`) starts: three
/// words from `011a8400` (the invalid translation), four from `011f3704`
/// (the rotation) and the `float` at `01096b8c` (the scale). The map has no
/// name for it; returns `this`.
pub fn fn_00494260(e: &mut Engine, this: Ptr) -> Ptr {
    for i in 0..3u32 {
        let word = e.mem.u32(RECORD_DEFAULT_TRANSLATE + 4 * i);
        e.mem.set_u32(this.addr() + 4 * i, word);
    }
    for i in 0..4u32 {
        let word = e.mem.u32(RECORD_DEFAULT_ROTATE + 4 * i);
        e.mem.set_u32(this.addr() + 12 + 4 * i, word);
    }
    let scale = e.mem.f32(RECORD_DEFAULT_SCALE);
    e.mem.set_f32(this.addr() + 0x1c, scale);
    this
}

// Translated from 004942c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the first `float` of the `NiQuatTransform` at `this` (its
/// translation's x) is not `-FLT_MAX`, the value `fn_00494260` starts it
/// with (no key was sampled). A NaN counts as filled in; the map has no name
/// for it.
pub fn fn_004942c0(e: &mut Engine, this: Ptr) -> bool {
    let lowest = -(e.global::<f32>(FLOAT_MAX) as f64);
    e.mem.f32(this.addr()) as f64 != lowest
}

// Translated from 00494300 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` that `005f4c70` gives for the animation group of the
/// sequence stored for group `group` (truncated to an integer through
/// `_ftol2`), 0 when the map has no entry, the entry keeps several
/// sequences or holds no sequence; the map has no name for it.
pub fn fn_00494300(e: &mut Engine, this: Ptr<Animation>, group: u16) -> i32 {
    let Some(entry) = sequence_map_get(e, this, group) else {
        return 0;
    };
    if !e.vcall(entry, 0xc, &args![]).bool() {
        return 0;
    }
    if e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32() == 0 {
        return 0;
    }
    let sequence = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32();
    let group_object = fn_0048f7f0(e, Ptr::new(sequence));
    let speed = e.call(GROUP_SPEED, &args![group_object]).f64();
    e.call(FTOL, &args![speed]).i32()
}

// Translated from 00494390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The movement of the frame for `actor`: writes three floats at `out`
/// (the `movementDelta`, with its components clamped to four times the
/// speed of the group playing in slot 1 or 4 when that is positive, divided
/// by the actor's scale for the actor kinds 3, 5, 8 and 10 on x and y, z
/// zeroed when `flatten` is set, and rotated by the animation root's world
/// rotation into `out` when `rotate` is set). Returns false without an
/// animation root; the map has no name for it.
pub fn fn_00494390(
    e: &mut Engine,
    this: Ptr<Animation>,
    out: Ptr,
    actor: Ptr,
    rotate: u8,
    flatten: u8,
) -> bool {
    let a = this.addr();
    if ni_pointer_get(e, a + Animation::pAnimRoot.off) == 0 {
        return false;
    }
    let time_scale = e.call(TIME_SCALE_GET, &args![TIME_SCALE_OBJECT]).f64();
    let multiplier = e.get(this, Animation::m_fGlobalTimeMultiplier);
    let scaled_time = (time_scale * multiplier as f64) as f32;
    let zero = [
        e.mem.u32(ZERO_VECTOR),
        e.mem.u32(ZERO_VECTOR + 4),
        e.mem.u32(ZERO_VECTOR + 8),
    ];
    let mut movement = [
        e.mem.f32(a + 0x10),
        e.mem.f32(a + 0x14),
        e.mem.f32(a + 0x18),
    ];
    let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
    let rotation = e.call(NODE_ROTATION, &args![root]).u32();
    let matrix: Vec<u32> = (0..9u32).map(|i| e.mem.u32(rotation + 4 * i)).collect();

    let actor_flag = e.vcall(actor.addr(), 0x21c, &args![]).bool();
    let walking = e.mem.u32(current_sequence_at(this, 1));
    if !actor_flag && e.call(ACTOR_FLAGS_ANY, &args![actor]).bool() && walking != 0 {
        let group = fn_0048f7f0(e, Ptr::new(walking));
        let kind = e.call(ANIM_GROUP_SEQUENCE_TYPE, &args![group]).i32();
        if kind >= 3 {
            let walking = e.mem.u32(current_sequence_at(this, 1));
            let group = fn_0048f7f0(e, Ptr::new(walking));
            let kind = e.call(ANIM_GROUP_SEQUENCE_TYPE, &args![group]).i32();
            if kind <= 0x10 {
                let walking = e.mem.u32(current_sequence_at(this, 1));
                let group = fn_0048f7f0(e, Ptr::new(walking));
                let speed = e.call(GROUP_SPEED, &args![group]).f64();
                let move_speed = e.get(this, Animation::m_fMoveSpeed);
                let reference = e.get(this, Animation::pActorRef).addr();
                let reference_scale = e.call(REFERENCE_SCALE, &args![reference]).f64();
                let product = speed * move_speed as f64 * scaled_time as f64;
                let mut limit = (reference_scale * product) as f32;
                let attack = e.mem.u32(current_sequence_at(this, 4));
                if attack != 0 && e.call(SEQUENCE_STATE, &args![attack]).i32() == 1 {
                    let group = fn_0048f7f0(e, Ptr::new(attack));
                    let speed = e.call(GROUP_SPEED, &args![group]).f64();
                    let attack_speed = e.get(this, Animation::m_fAttackSpeed);
                    let reference = e.get(this, Animation::pActorRef).addr();
                    let reference_scale = e.call(REFERENCE_SCALE, &args![reference]).f64();
                    let product = speed * attack_speed as f64 * scaled_time as f64;
                    let attack_limit = (reference_scale * product) as f32;
                    if limit < attack_limit {
                        limit = attack_limit;
                    }
                }
                let ceiling = (limit as f64 * double_constant(e, DOUBLE_FOUR)) as f32;
                if limit as f64 > double_constant(e, DOUBLE_ZERO) {
                    for value in movement.iter_mut() {
                        if ceiling < *value {
                            *value = limit;
                        }
                    }
                    for value in movement.iter_mut() {
                        if *value < -ceiling {
                            *value = -limit;
                        }
                    }
                }
            }
        }
    }
    if !actor.is_null() {
        let kind = e.vcall(actor.addr(), 0x214, &args![]).u32();
        if matches!(kind, 3 | 5 | 8 | 10) {
            let scale = e.call(REFERENCE_SCALE, &args![actor]).f32();
            movement[0] = (movement[0] as f64 / scale as f64) as f32;
            movement[1] = (movement[1] as f64 / scale as f64) as f32;
        }
    }
    if flatten != 0 {
        movement[2] = 0.0;
    }
    if rotate == 0 {
        for (i, value) in movement.iter().enumerate() {
            e.mem.set_f32(out.addr() + 4 * i as u32, *value);
        }
    } else {
        e.with_stack(36 + 12 + 12, |e, block| {
            let (matrix_at, zero_at, vector_at) =
                (block.addr(), block.addr() + 36, block.addr() + 48);
            for (i, word) in matrix.iter().enumerate() {
                e.mem.set_u32(matrix_at + 4 * i as u32, *word);
            }
            for (i, word) in zero.iter().enumerate() {
                e.mem.set_u32(zero_at + 4 * i as u32, *word);
            }
            for (i, value) in movement.iter().enumerate() {
                e.mem.set_f32(vector_at + 4 * i as u32, *value);
            }
            e.call(
                MATRIX_TRANSFORM_VERTICES,
                &args![matrix_at, zero_at, 1u32, vector_at, out],
            );
        });
    }
    true
}

// ---- Groups: AddGroup, GroupLoaded, PlayGroup, StartGroup -------------------------

// Translated from 004946a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::AddGroup` (Xbox PDB): registers animation group `group`. When
/// slot 1 has no group and no group is queued for it, and the group's id has
/// a zero low byte (type 0), the id is queued as slot 1's next group. Then
/// the group's speed is initialized. Does nothing for a null group.
pub fn animation_add_group(e: &mut Engine, this: Ptr<Animation>, group: Ptr) {
    if group.is_null() {
        return;
    }
    if e.mem.u16(group_at(this, 1)) == 0xff && e.mem.u16(next_group_at(this, 1)) == 0xff {
        let id = e.call(ANIM_GROUP_ID, &args![group]).u16();
        if id == 0 {
            let id = e.call(ANIM_GROUP_ID, &args![group]).u16();
            e.mem.set_u16(next_group_at(this, 1), id);
        }
    }
    animation_init_group_speed(e, this, group);
}

// Translated from 00494710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::GroupLoaded` (Xbox PDB): whether the animation sequence map
/// has an entry for group `group`.
pub fn animation_group_loaded(e: &mut Engine, this: Ptr<Animation>, group: u16) -> bool {
    sequence_map_get(e, this, group).is_some()
}

// Translated from 00494740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::PlayGroup` (Xbox PDB): plays group `group` in `slot` (-1 =
/// the slot of the group's type; 0x14 and 0x15 stand for 1 and 4). For the
/// categories 0..2, `mode` 0 queues it as the slot's next group with `loops`
/// loops and `mode` 1 starts it now; for the categories 3..10 it always
/// starts now. Starting calls `StartGroup` and `UpdateBipOnly`. Returns the
/// started sequence, 0 when nothing was started.
pub fn animation_play_group(
    e: &mut Engine,
    this: Ptr<Animation>,
    group: u16,
    mode: i32,
    loops: i32,
    slot: i32,
) -> u32 {
    let kind = group_type(e, group);
    let wanted = if slot == -1 {
        type_column(e, SEQUENCE_TYPE_SLOT, kind)
    } else {
        slot
    };
    let mut started = 0u32;
    let wanted = match wanted {
        0x14 => 1,
        0x15 => 4,
        other => other,
    };
    if kind == 0xff {
        return started;
    }
    let category = type_column(e, SEQUENCE_TYPE_CATEGORY, kind) as u32;
    if category > 10 {
        return started;
    }
    let slot_index = wanted as u32;
    if category <= 2 {
        if mode == 0 {
            e.mem.set_u16(next_group_at(this, slot_index), group);
            e.mem.set_i32(next_loops_at(this, slot_index), loops);
        } else if mode == 1 {
            e.mem.set_u16(next_group_at(this, slot_index), 0xff);
            e.mem.set_i32(loop_count_at(this, slot_index), loops);
            started = animation_start_group(e, this, group, slot).addr();
            let time: f32 = e.get(this, Animation::time);
            e.call(UPDATE_BIP_ONLY, &args![this, time, 0u32, 1u32]);
        }
    } else {
        started = animation_start_group(e, this, group, slot).addr();
        let time: f32 = e.get(this, Animation::time);
        e.call(UPDATE_BIP_ONLY, &args![this, time, 0u32, 1u32]);
    }
    started
}

// Translated from 004948c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::StartGroup` (Xbox PDB): starts group `group` in `slot` (-1 =
/// the slot of the group's type). Takes the sequence the map holds for the
/// group; for slots 5 and 6 of a container that keeps several sequences it
/// asks `GetCorrespondingSequence` for the one matching slot 4's current
/// sequence. Returns what `StartGroup_ov2` returns, 0 for group 0xff or one
/// the map does not know.
pub fn animation_start_group(e: &mut Engine, this: Ptr<Animation>, group: u16, slot: i32) -> Ptr {
    if group == 0xff {
        return Ptr::NULL;
    }
    let Some(entry) = sequence_map_get(e, this, group) else {
        return Ptr::NULL;
    };
    let wanted = if slot == -1 {
        let kind = group_type(e, group);
        type_column(e, SEQUENCE_TYPE_SLOT, kind)
    } else {
        slot
    };
    let sequence = if (5..=6).contains(&wanted) && !e.vcall(entry, 0xc, &args![]).bool() {
        let current = e.mem.u32(current_sequence_at(this, 4));
        anim_sequence_multiple_get_corresponding_sequence(
            e,
            Ptr::new(entry),
            Ptr::new(current),
            wanted,
        )
    } else {
        e.vcall(entry, 0x10, &args![0xffff_ffffu32]).ptr()
    };
    animation_start_group_ov2(e, this, sequence, group, slot)
}

// Translated from 004949a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::StartGroup_ov2` (Xbox PDB): puts `sequence` (the sequence of
/// group `group`) into slot `slot_arg` (-1 = the slot of the group's type;
/// 0x14 and 0x15 stand for 1 and 4), replacing the slot's current sequence.
/// Returns `sequence`, or 0 when it is null, the group is 0xff, a special
/// idle is already working on it, or a non-player's first slot is asked to
/// replace an animating sequence.
///
/// In order: sets the sequence weight for the weapon slots (4..6); resolves
/// what happens to the slot's current sequence (cleared, kept for a
/// cross-fade, or cleared together with slots 5 and 6); stores the group and
/// sequence in the slot; checks whether the two sequences can be morphed
/// (same controller count and morph tags, logging when not); picks the blend
/// time (the setting, or the longer of the groups' blend frames over 30, or
/// the menu setting; zero when the previous group was an attack of the
/// player's, when `cSkipNextBlend` is set, or when the slot is replaced
/// right away), divided by the movement scale setting; and activates the
/// sequence through the manager (a morph fade, a cross-fade, an immediate
/// activation or a blend-in). A replaced sequence that is animating is
/// deactivated. `action[slot]` returns to 0.
/// The compiler's exception-unwinding frame is not translated.
pub fn animation_start_group_ov2(
    e: &mut Engine,
    this: Ptr<Animation>,
    sequence: Ptr,
    group: u16,
    slot_arg: i32,
) -> Ptr {
    let a = this.addr();
    let sequence = sequence.addr();
    if e.call(SPECIAL_IDLE_WORKING, &args![this, sequence]).bool() {
        return Ptr::NULL;
    }
    let kind = group_type(e, group);
    let reference = e.get(this, Animation::pActorRef).addr();
    let mut slot = slot_arg;
    if slot == -1 {
        slot = type_column(e, SEQUENCE_TYPE_SLOT, kind);
    }
    let requested = slot;
    if slot == 0x14 {
        slot = 1;
    } else if slot == 0x15 {
        slot = 4;
    }
    let slot_index = slot as u32;
    let mut current = e.mem.u32(current_sequence_at(this, slot_index));
    let previous_group = e.mem.u16(group_at(this, slot_index));
    let previous_type = group_type(e, previous_group);
    let mut morph = false;
    let mut current_state = 0;
    if current != 0 {
        current_state = e.call(SEQUENCE_STATE, &args![current]).i32();
    }
    if sequence == 0 || group == 0xff {
        return Ptr::NULL;
    }

    let player: u32 = e.global(PLAYER_SINGLETON);
    if slot == 4 || slot == 5 || slot == 6 {
        if reference == player
            && this.addr() == e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32()
        {
            let weight = if slot == 4 { 1.0 } else { 0.0 };
            fn_00495480(e, Ptr::new(sequence), weight);
        } else {
            let process = e.call(ACTOR_PROCESS, &args![reference]).u32();
            let mut weight_unset = true;
            let aims = e.call(GROUP_ID_IS_AIM, &args![kind as u16]).bool();
            if (aims || e.call(GROUP_ID_IS_ATTACK, &args![kind as u16]).bool())
                && process != 0
                && e.call(PROCESS_WORD_28, &args![process]).i32() <= 1
            {
                let held = fn_00495560(e, Ptr::new(process), slot - 4);
                if held != 0 {
                    let weight = fn_00495460(e, Ptr::new(held));
                    fn_00495480(e, Ptr::new(sequence), weight);
                    weight_unset = false;
                }
            }
            if weight_unset && (slot == 5 || slot == 6) {
                fn_00495480(e, Ptr::new(sequence), 0.0);
            }
        }
    }
    if e.call(SEQUENCE_STATE, &args![sequence]).i32() == 3 {
        e.vcall(sequence, 0x8c, &args![0.0f32, 0u32]);
    }

    let state = e.call(SEQUENCE_STATE, &args![sequence]).i32();
    let replaced_in_place = (state != 0
        && e.call(SEQUENCE_CYCLE_TYPE, &args![sequence]).u32() == 0)
        || (e.call(SEQUENCE_STATE, &args![sequence]).i32() == 1 && sequence == current);
    if replaced_in_place
        && (!e.call(IS_IN_MENU_MODE, &args![]).bool()
            || e.call(SEQUENCE_STATE, &args![sequence]).i32() == 1)
    {
        e.mem.set_i32(action_at(this, slot_index), 0);
        fn_004954c0(e, Ptr::new(sequence));
        return Ptr::new(sequence);
    }

    let in_menu = e.call(IS_IN_MENU_MODE, &args![]).bool();
    let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
    let first_node = e.call(PLAYER_NODE, &args![player, 0u32]).u32();
    let pipboy_object: u32 = e.global(PIPBOY_QUERY_OBJECT);
    let skip_clearing = in_menu
        && root == first_node
        && !e.call(PIPBOY_QUERY, &args![pipboy_object]).bool()
        && e.call(WORD_AT_8, &args![VATS_OBJECT]).u32() == 0;
    if !skip_clearing {
        let mut handled = false;
        if e.call(IS_IN_PIPBOY_MENU, &args![]).bool() {
            let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
            let second_node = e.call(PLAYER_NODE, &args![player, 1u32]).u32();
            if root == second_node && kind == 1 && slot_arg == 0 && current != 0 {
                e.call(CLEAR_GROUP, &args![this, requested, 0.0f32]);
                current = 0;
                e.mem.set_u8(a + Animation::cSkipNextBlend.off, 1);
                handled = true;
            }
        }
        if !handled {
            if current != 0 && slot == 2 {
                let held_group = fn_0048f7f0(e, Ptr::new(current));
                let held_type = e.call(ANIM_GROUP_SEQUENCE_TYPE, &args![held_group]).i32();
                if kind == 0xe2 || (kind == 0xf0 && (0xe6..=0xeb).contains(&held_type)) {
                    e.call(CLEAR_GROUP, &args![this, requested, 0.0f32]);
                    current = 0;
                }
            } else if current_state != 0 && current != 0 {
                if current_state == 1 {
                    if requested != slot
                        || e.mem.i8(a + Animation::cSkipNextBlend.off) != 0
                        || e.call(SEQUENCE_STATE, &args![sequence]).i32() != 0
                    {
                        e.call(CLEAR_GROUP, &args![this, requested, 0.0f32]);
                    } else if requested == 4
                        && (kind as i32 <= 0x19 || kind as i32 > 0xd4)
                        && !e.call(GROUP_ID_IS_AIM, &args![kind as u16]).bool()
                    {
                        e.call(CLEAR_GROUP, &args![this, 5u32, 0.0f32]);
                        e.call(CLEAR_GROUP, &args![this, 6u32, 0.0f32]);
                    }
                } else {
                    if reference != player && slot == 1 {
                        return Ptr::NULL;
                    }
                    e.call(CLEAR_GROUP, &args![this, requested, 0.0f32]);
                    current = 0;
                }
            }
        }
    }

    e.mem.set_u16(group_at(this, slot_index), group);
    e.mem
        .set_u32(current_sequence_at(this, slot_index), sequence);

    // Whether the new sequence can be morphed from the old one.
    let in_menu = e.call(IS_IN_MENU_MODE, &args![]).bool();
    let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
    let first_node = e.call(PLAYER_NODE, &args![player, 0u32]).u32();
    if (!in_menu || root != first_node) && sequence != 0 && current != 0 {
        let new_group = fn_0048f7f0(e, Ptr::new(sequence));
        let old_group = fn_0048f7f0(e, Ptr::new(current));
        e.with_stack(4, |e, new_ref| {
            e.call(NI_POINTER_INIT, &args![new_ref, new_group]);
            e.with_stack(4, |e, old_ref| {
                e.call(NI_POINTER_INIT, &args![old_ref, old_group]);
                let old_group = ni_pointer_get(e, old_ref.addr());
                if e.call(GROUP_BYTE_28, &args![old_group]).u8() as i8 != 0 {
                    let old_group = ni_pointer_get(e, old_ref.addr());
                    let old_value = e.call(GROUP_BYTE_28, &args![old_group]).u8() as i8;
                    let new_group = ni_pointer_get(e, new_ref.addr());
                    let new_value = e.call(GROUP_BYTE_28, &args![new_group]).u8() as i8;
                    if old_value == new_value {
                        let old_count = e.call(WORD_AT_C, &args![current]).u32();
                        let new_count = e.call(WORD_AT_C, &args![sequence]).u32();
                        if old_count == new_count {
                            if e.call(SEQUENCE_MORPH_COMPATIBLE, &args![sequence, current])
                                .bool()
                            {
                                morph = true;
                            } else {
                                let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
                                let root_name = object_name(e, root);
                                let new_name = object_name(e, sequence);
                                let old_name = object_name(e, current);
                                e.call(LOG, &args![LOG_MORPH_TAGS, old_name, new_name, root_name]);
                            }
                        } else {
                            let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
                            let root_name = object_name(e, root);
                            let new_count = e.call(WORD_AT_C, &args![sequence]).u32();
                            let new_name = object_name(e, sequence);
                            let old_count = e.call(WORD_AT_C, &args![current]).u32();
                            let old_name = object_name(e, current);
                            e.call(
                                LOG,
                                &args![
                                    LOG_MORPH_CONTROLLERS,
                                    old_name,
                                    old_count,
                                    new_name,
                                    new_count,
                                    root_name
                                ],
                            );
                        }
                    }
                }
                if morph && sequence == current {
                    let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
                    let root_name = object_name(e, root);
                    let name = object_name(e, current);
                    e.call(LOG, &args![LOG_MORPH_SELF, name, root_name]);
                    morph = false;
                }
                e.call(NI_POINTER_RELEASE, &args![old_ref]);
            });
            e.call(NI_POINTER_RELEASE, &args![new_ref]);
        });
    }

    // The blend time.
    let mut blend = float_setting(e, SETTING_BLEND_TIME);
    let mut frames = 0u8;
    if current != 0 {
        let group = fn_0048f7f0(e, Ptr::new(current));
        frames = fn_00495520(e, group);
    }
    let new_group = fn_0048f7f0(e, Ptr::new(sequence));
    if fn_004954e0(e, new_group) > frames {
        let new_group = fn_0048f7f0(e, Ptr::new(sequence));
        frames = fn_004954e0(e, new_group);
    }
    if frames != 0 {
        let thirty = double_constant(e, DOUBLE_THIRTY);
        blend = (frames as f64 / thirty) as f32;
    }
    let mut menu_blend = false;
    if e.call(IS_IN_MENU_MODE, &args![]).bool() {
        let object = fn_00490f80(e, Ptr::new(player));
        menu_blend = e.call(NODE_ACCEPTS_OBJECT, &args![a + 8, object]).bool();
    }
    if menu_blend
        || e.call(IS_MENU_ID_VISIBLE, &args![MENU_ID_PIPBOY_WAIT, 0u32])
            .bool()
    {
        blend = float_setting(e, SETTING_BLEND_TIME_MENU);
    }
    if (0xb1..=0xc7).contains(&(kind as i32))
        && current != 0
        && reference != player
        && (blend as f64) < double_constant(e, DOUBLE_HALF)
    {
        let new_group = fn_0048f7f0(e, Ptr::new(sequence));
        let new_move = e.call(GROUP_MOVE_TYPE, &args![new_group]).i32();
        let old_group = fn_0048f7f0(e, Ptr::new(current));
        let old_move = e.call(GROUP_MOVE_TYPE, &args![old_group]).i32();
        if (new_move == 1) != (old_move == 1) {
            blend = e.global::<f32>(FLOAT_HALF);
        }
    }
    if e.call(GROUP_ID_IS_ATTACK, &args![previous_type as u16])
        .bool()
        && (0xad..=0xc7).contains(&(kind as i32))
        && reference == player
    {
        blend = 0.0;
    }
    if e.mem.i8(a + Animation::cSkipNextBlend.off) != 0 {
        blend = 0.0;
    }
    let divisor = float_setting(e, SETTING_MOVEMENT_SCALE);
    blend = (blend as f64 / divisor as f64) as f32;
    set_phase(e, sequence, 0.0);

    if current == 0 && (slot == 5 || slot == 6) {
        let manager = ni_pointer_get(e, a + Animation::spManager.off);
        let weight = fn_00495460(e, Ptr::new(sequence));
        e.call(
            MANAGER_ACTIVATE,
            &args![manager, sequence, 0u32, 1u32, weight, blend, 0u32],
        );
    } else if (blend as f64) < double_constant(e, DOUBLE_HUNDREDTH) {
        let weight = fn_00495460(e, Ptr::new(sequence));
        let manager = ni_pointer_get(e, a + Animation::spManager.off);
        e.call(
            MANAGER_ACTIVATE,
            &args![manager, sequence, 0u32, 1u32, weight, 0.0f32, 0u32],
        );
    } else if morph {
        let new_weight = fn_00495460(e, Ptr::new(sequence));
        let old_weight = fn_00495460(e, Ptr::new(current));
        let manager = ni_pointer_get(e, a + Animation::spManager.off);
        e.call(
            MANAGER_MORPH_FADE,
            &args![manager, current, sequence, blend, 0u32, old_weight, new_weight],
        );
    } else {
        let mut crossed = false;
        if current != 0 && e.call(SEQUENCE_STATE, &args![current]).i32() != 0 {
            let weight = fn_00495460(e, Ptr::new(sequence));
            let manager = ni_pointer_get(e, a + Animation::spManager.off);
            crossed = e
                .call(
                    MANAGER_CROSS_FADE,
                    &args![manager, current, sequence, blend, 0u32, 1u32, weight, 0u32],
                )
                .bool();
        }
        if !crossed {
            let tes: u32 = e.global(TES_GLOBAL);
            if e.call(TES_CHECK, &args![tes]).bool() {
                let manager = ni_pointer_get(e, a + Animation::spManager.off);
                e.call(
                    MANAGER_ACTIVATE,
                    &args![manager, sequence, 0u32, 1u32, 1.0f32, 0.0f32, 0u32],
                );
            } else {
                let reference_object = e.get(this, Animation::pActorRef).addr();
                if reference_object != 0
                    && e.vcall(reference_object, 0x100, &args![]).bool()
                    && fn_00495580(e) != 0
                    && e.mem.u32(reference_object + 0xac) != 0
                {
                    let ragdoll = e.mem.u32(reference_object + 0xac);
                    if fn_004955a0(e, Ptr::new(ragdoll)) != 0 {
                        let ragdoll = e.mem.u32(reference_object + 0xac);
                        e.call(RAGDOLL_REFRESH, &args![ragdoll]);
                    }
                }
                let manager = ni_pointer_get(e, a + Animation::spManager.off);
                e.call(
                    MANAGER_BLEND_IN,
                    &args![manager, sequence, 0.0f32, blend, 0u32, 0u32],
                );
            }
        }
    }

    if current != 0 && current != sequence {
        let state = e.call(SEQUENCE_STATE, &args![current]).i32();
        if state > 0 && (state <= 2 || state == 5) {
            let manager = ni_pointer_get(e, a + Animation::spManager.off);
            e.call(MANAGER_DEACTIVATE, &args![manager, current, 0.0f32]);
        }
    }
    e.mem.set_u32(action_at(this, slot_index), 0);
    Ptr::new(sequence)
}

// ---- Small accessors of the sequence and the group -------------------------------

// Translated from 00495460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0x1c of `this` (the weight of a `NiControllerSequence`);
/// the map has no name for it.
pub fn fn_00495460(e: &mut Engine, this: Ptr) -> f32 {
    e.mem.f32(this.addr() + 0x1c)
}

// Translated from 00495480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the `float` at +0x1c of `this` (the sequence weight),
/// raised to 0 when negative; the map has no name for it.
pub fn fn_00495480(e: &mut Engine, this: Ptr, value: f32) {
    e.mem.set_f32(this.addr() + 0x1c, x87_quiet(value));
    if (e.mem.f32(this.addr() + 0x1c) as f64) < double_constant(e, DOUBLE_ZERO) {
        e.mem.set_f32(this.addr() + 0x1c, 0.0);
    }
}

// Translated from 004954c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `float` at +0x48 of `this` (the last scaled time of a
/// `NiControllerSequence`) to `-FLT_MAX`, the "no time yet" marker
/// `GetScaledTime` users compare with; the map has no name for it.
pub fn fn_004954c0(e: &mut Engine, this: Ptr) {
    let lowest = -e.global::<f32>(FLOAT_MAX);
    e.mem.set_f32(this.addr() + 0x48, lowest);
}

// Translated from 004954e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The larger of the bytes at +0x29 and +0x2a of the animation group (two
/// blend times in frames); the map has no name for it.
pub fn fn_004954e0(e: &mut Engine, this: Ptr) -> u8 {
    let first = e.mem.u8(this.addr() + 0x29);
    let second = e.mem.u8(this.addr() + 0x2a);
    if second > first {
        second
    } else {
        first
    }
}

// Translated from 00495520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The larger of the bytes at +0x29 and +0x2b of the animation group; the
/// map has no name for it.
pub fn fn_00495520(e: &mut Engine, this: Ptr) -> u8 {
    let first = e.mem.u8(this.addr() + 0x29);
    let second = e.mem.u8(this.addr() + 0x2b);
    if second > first {
        second
    } else {
        first
    }
}

// Translated from 00495560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `this + 0x1c4 + index * 4` (an array in the actor's process
/// object); the map has no name for it.
pub fn fn_00495560(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    e.mem.u32(
        this.addr()
            .wrapping_add(0x1c4)
            .wrapping_add((index as u32).wrapping_mul(4)),
    )
}

// Translated from 00495580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte value of the setting object at `01267c30`; the map has no name
/// for it.
pub fn fn_00495580(e: &mut Engine) -> u8 {
    let address = e
        .call(SETTING_BYTE_ADDRESS, &args![SETTING_BYTE_OBJECT])
        .u32();
    e.mem.u8(address)
}

// Translated from 004955a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x222 of `this` (a ragdoll controller, in the caller); the
/// map has no name for it.
pub fn fn_004955a0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x222)
}

// ---- ForceSection, PickBestAnimation, ShouldBeMoving and the rest -----------------

// Translated from 004955c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::ForceSection` (Xbox PDB): forces slot `slot` (0x14 and 0x15
/// stand for 1 and 4) to group `group`. With `action` -1 it clears the slot
/// (`ClearGroup(slot, 0.0)`) and, for a group the map knows, takes the entry's
/// sequence (selected by `selector`) as the slot's current sequence and
/// activates it at once; otherwise, for a loaded group, starts the entry's
/// sequence through `StartGroup_ov2` and stores `action` as the slot's
/// action. Then stores the group in the slot and `time` in `time`.
pub fn animation_force_section(
    e: &mut Engine,
    this: Ptr<Animation>,
    slot: i32,
    group: u16,
    action: i32,
    time: f32,
    selector: u8,
) {
    let a = this.addr();
    let index = match slot {
        0x14 => 1,
        0x15 => 4,
        other => other,
    } as u32;
    if action == -1 {
        e.call(CLEAR_GROUP, &args![this, slot, 0.0f32]);
        if group != 0xff {
            if let Some(entry) = sequence_map_get(e, this, group) {
                if e.vcall(entry, 0x10, &args![selector]).u32() != 0 {
                    let sequence = e.vcall(entry, 0x10, &args![selector]).u32();
                    e.mem.set_u32(current_sequence_at(this, index), sequence);
                    let manager = ni_pointer_get(e, a + Animation::spManager.off);
                    let sequence = e.mem.u32(current_sequence_at(this, index));
                    e.call(
                        MANAGER_ACTIVATE,
                        &args![manager, sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
                    );
                }
            }
        }
    } else if animation_group_loaded(e, this, group) {
        if let Some(entry) = sequence_map_get(e, this, group) {
            let sequence = e.vcall(entry, 0x10, &args![selector]).u32();
            animation_start_group_ov2(e, this, Ptr::new(sequence), group, slot);
        }
        e.mem.set_i32(action_at(this, index), action);
    }
    e.mem.set_u16(group_at(this, index), group);
    e.set(this, Animation::time, time);
}

// Translated from 00495740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::PickBestAnimation` (Xbox PDB): the animation group id the
/// animation can actually play in place of `group`. `group` itself when the
/// map has a sequence for it; otherwise the first of these that has one:
/// the group with the 0x8000 bit cleared (when set), the same for the
/// player's own animation with the 0x7000 bits cleared, the group three
/// lower for an iron-sights action, the weapon variants of the group
/// (weapon 3 maps to 2, the others except 2 and 4 to 4) and the base group
/// (low byte and bits 12..14 only), the groups of the slot-5/6 types, the
/// movement-type substitutes (types 7..10 map to 3..6), and finally the
/// group with its low byte cleared (unless `flag` is set); 0 when there is
/// none.
pub fn animation_pick_best_animation(
    e: &mut Engine,
    this: Ptr<Animation>,
    group: u16,
    flag: u8,
) -> u16 {
    // Whether the map holds a sequence for `candidate`.
    fn playable(e: &mut Engine, this: Ptr<Animation>, candidate: u16) -> bool {
        match sequence_map_get(e, this, candidate) {
            Some(entry) => e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32() != 0,
            None => false,
        }
    }
    // The answer of a nested pick counts when its low byte is the group's.
    fn same_type(answer: u16, group: u16) -> bool {
        answer & 0xff == group & 0xff
    }

    if playable(e, this, group) {
        return group;
    }
    if group & 0x8000 != 0 {
        let answer = animation_pick_best_animation(e, this, group & 0x7fff, 1);
        if same_type(answer, group) {
            return answer;
        }
        if e.call(GROUP_ID_IS_IRON_SIGHTS, &args![group]).bool()
            && same_type(answer, group.wrapping_sub(3))
        {
            return answer;
        }
    }
    let player: u32 = e.global(PLAYER_SINGLETON);
    if e.get(this, Animation::pActorRef).addr() == player
        && this.addr() == e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32()
        && group & 0x7000 != 0
    {
        let slot_type = e
            .mem
            .i32(SEQUENCE_TYPE_SLOT.wrapping_add((group as u32 & 0xff).wrapping_mul(0x24)));
        if (4..=6).contains(&slot_type) {
            let answer = animation_pick_best_animation(e, this, group & 0xfff, 1);
            if same_type(answer, group) {
                return answer;
            }
            if e.call(GROUP_ID_IS_IRON_SIGHTS, &args![group]).bool()
                && same_type(answer, group.wrapping_sub(3))
            {
                return answer;
            }
        }
    }
    if e.call(GROUP_ID_IS_IRON_SIGHTS, &args![group]).bool() {
        return animation_pick_best_animation(e, this, group.wrapping_sub(3), 1);
    }
    let kind = group_type(e, group);
    let slot_type = type_column(e, SEQUENCE_TYPE_SLOT, kind);
    if (5..=6).contains(&slot_type) {
        return 0;
    }
    if group & 0xf00 != 0 {
        let weapon = e.call(GROUP_ID_WEAPON, &args![group]).i32();
        if weapon != 2 {
            if weapon == 3 {
                let candidate = group & 0xf0ff | 0x200;
                if playable(e, this, candidate) {
                    return candidate;
                }
            } else if weapon != 4 {
                let candidate = group & 0xf0ff | 0x400;
                if playable(e, this, candidate) {
                    return candidate;
                }
            }
        }
        let candidate = group & 0xf0ff;
        if playable(e, this, candidate) {
            return candidate;
        }
    }
    if group & 0x8000 != 0 {
        let answer = animation_pick_best_animation(e, this, group & 0x7fff, 1);
        if same_type(answer, group) {
            return answer;
        }
    }
    if group != 0 {
        let kind = group_type(e, group);
        let substitute = match kind {
            7 => Some(group & 0x7f00 | 3),
            8 => Some(group & 0x7f00 | 4),
            9 => Some(group & 0x7f00 | 5),
            10 => Some(group & 0x7f00 | 6),
            _ => None,
        };
        if let Some(substitute) = substitute {
            let answer = animation_pick_best_animation(e, this, substitute, 1);
            if same_type(answer, substitute) {
                return answer;
            }
        }
    }
    if flag != 0 {
        return 0;
    }
    if group & 0x7000 != 0 {
        let answer = animation_pick_best_animation(e, this, group & 0xfff, 1);
        if same_type(answer, group) {
            return answer;
        }
    }
    if group & 0xff != 0 {
        return animation_pick_best_animation(e, this, group & 0x7f00, 0);
    }
    0
}

// Translated from 00495be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::ShouldBeMoving` (Xbox PDB): whether the sequence in slot 1 is
/// the one that wins the sound-priority bone of slot 1 (the highest
/// `GetPriority` among the eight current sequences, ties going to slot 1)
/// while slot 1's group is of a type from 3 to 14. False without a slot 1
/// sequence or a sound-priority bone.
pub fn animation_should_be_moving(e: &mut Engine, this: Ptr<Animation>) -> bool {
    let first = e.mem.u32(current_sequence_at(this, 1));
    if first == 0 {
        return false;
    }
    let bone_at = animation_entry(this, Animation::pSoundPriorityBone, 1, 4);
    let bone = e.mem.u32(bone_at);
    if bone == 0 {
        return false;
    }
    let bone_name = object_name(e, bone);
    let group = e.mem.u16(group_at(this, 1));
    let kind = group_type(e, group) as i32;
    if !(3..=0xe).contains(&kind) {
        return false;
    }
    let mut best = 0u32;
    let mut best_priority = 0u8;
    for slot in 0..8u32 {
        let sequence = e.mem.u32(current_sequence_at(this, slot));
        if sequence == 0 {
            continue;
        }
        let priority = e
            .call(SEQUENCE_PRIORITY, &args![sequence, bone_name, 0u32])
            .u8();
        if priority > best_priority {
            best = e.mem.u32(current_sequence_at(this, slot));
            best_priority = priority;
        } else if priority == best_priority && slot == 1 {
            best = e.mem.u32(current_sequence_at(this, 1));
            best_priority = priority;
        }
    }
    best == e.mem.u32(current_sequence_at(this, 1))
}

// Translated from 00495d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of entries of the sequence array at +0x34 of an
/// `NiControllerManager`; the map has no name for it.
pub fn fn_00495d00(e: &mut Engine, this: Ptr<NiControllerManager>) -> u32 {
    let array = this.addr() + NiControllerManager::m_kSequenceArray.off;
    e.call(SEQUENCE_ARRAY_COUNT, &args![array]).u32()
}

// Translated from 00495d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiControllerManager::GetSequenceAt` (Xbox PDB): the sequence at `index`
/// of the sequence array.
pub fn ni_controller_manager_get_sequence_at(
    e: &mut Engine,
    this: Ptr<NiControllerManager>,
    index: u32,
) -> Ptr {
    let array = this.addr() + NiControllerManager::m_kSequenceArray.off;
    let element = e.call(SEQUENCE_ARRAY_ELEMENT, &args![array, index]).u32();
    e.call(READ_WORD, &args![element]).ptr()
}

// Translated from 00495d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendInterpolator::GetInterpolator` (Xbox PDB): the interpolator of
/// entry `index`: the single interpolator at +0x18 when the blend holds
/// exactly one (byte +0xe is 1) and its index (byte +0xf) is `index`,
/// otherwise the pointer in the 0x18-byte entry `index` of the array at
/// +0x14.
pub fn ni_blend_interpolator_get_interpolator(e: &mut Engine, this: Ptr, index: u8) -> Ptr {
    let a = this.addr();
    if e.mem.u8(a + 0xe) == 1 && index == e.mem.u8(a + 0xf) {
        return Ptr::new(e.mem.u32(a + 0x18));
    }
    let entry = (index as u32 * 0x18).wrapping_add(e.mem.u32(a + 0x14));
    e.call(READ_WORD, &args![entry]).ptr()
}

// Translated from 00495da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SyncSequences` (Xbox PDB): starts the sequence of group
/// `group` that corresponds to `sequence` (`GetCorrespondingSequence`, for
/// an entry that keeps several sequences) in the slot of the group's type.
/// Returns 0 for group 0xff, an unknown group, an entry that holds a single
/// sequence, or no corresponding sequence.
pub fn animation_sync_sequences(
    e: &mut Engine,
    this: Ptr<Animation>,
    sequence: Ptr,
    group: u16,
) -> Ptr {
    if group == 0xff {
        return Ptr::NULL;
    }
    let Some(entry) = sequence_map_get(e, this, group) else {
        return Ptr::NULL;
    };
    let mut multiple = 0u32;
    if !e.vcall(entry, 0xc, &args![]).bool() {
        multiple = entry;
    }
    if multiple == 0 {
        return Ptr::NULL;
    }
    let corresponding =
        anim_sequence_multiple_get_corresponding_sequence(e, Ptr::new(multiple), sequence, -1);
    if corresponding.is_null() {
        return Ptr::NULL;
    }
    animation_start_group_ov2(e, this, corresponding, group, -1)
}

// ---- Text keys, ClearGroup and the transform resets (third session) --------------------

// Translated from 00495f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads a sequence's text key list (see `SEQUENCE_TEXT_KEYS`): stores the
/// entry count (+0xc) in `*count` and returns the entry array (+0x10). The
/// map has no name for it.
pub fn fn_00495f30(e: &mut Engine, this: Ptr, count: Ptr) -> u32 {
    let entries = e.mem.u32(this.addr() + 0x10);
    let total = e.mem.u32(this.addr() + 0xc);
    e.mem.set_u32(count.addr(), total);
    entries
}

/// The text keys of `sequence` as (entry array, count): `00700300` then
/// [`fn_00495f30`].
fn text_keys_of(e: &mut Engine, sequence: u32) -> (u32, u32) {
    let keys = e.call(SEQUENCE_TEXT_KEYS, &args![sequence]).u32();
    e.with_stack(4, |e, count| {
        let entries = fn_00495f30(e, Ptr::new(keys), count);
        (entries, e.mem.u32(count.addr()))
    })
}

/// The text of text key `entry` (`NiFixedString` to `const char*`).
fn text_key_text(e: &mut Engine, entry: u32) -> u32 {
    let text = e.call(TEXT_KEY_TEXT, &args![entry]).u32();
    e.call(FIXED_STRING_CSTR, &args![text]).u32()
}

/// `tolower` of the character that follows a matched prefix (a signed `char`
/// widened to `int`, as the game passes it); only the low byte is the result.
fn lower_next_character(e: &mut Engine, text: u32, length: u32) -> u8 {
    let next = e.mem.u8(text.wrapping_add(length)) as i8 as i32;
    e.call(TOLOWER, &args![next]).u8()
}

// Translated from 00495e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks through the text keys of the sequence in slot 4 for a key that
/// starts with `"a:"` (case-insensitive). On the first match stores that
/// key's time in `*time` (when `time` is not null) and returns the lower
/// case character that follows the prefix; returns 0 for no sequence, no
/// match or an empty prefix. The map has no name for it. The upper bytes of
/// the game's return value are whatever the register held; callers read `AL`.
pub fn fn_00495e40(e: &mut Engine, this: Ptr<Animation>, time: Ptr) -> u8 {
    let sequence = e.mem.u32(current_sequence_at(this, 4));
    if sequence == 0 {
        return 0;
    }
    let (entries, count) = text_keys_of(e, sequence);
    let prefix = TEXT_KEY_PREFIX_A;
    let length = e.call(STRING_LENGTH, &args![prefix]).u32();
    if length == 0 {
        return 0;
    }
    for index in 0..count {
        let entry = entries.wrapping_add(index.wrapping_mul(8));
        let text = text_key_text(e, entry);
        if text != 0 && e.call(STRNICMP, &args![text, prefix, length]).u32() == 0 {
            if !time.is_null() {
                let key_time = e.call(TEXT_KEY_TIME, &args![entry]).f32();
                e.mem.set_f32(time.addr(), key_time);
            }
            return lower_next_character(e, text, length);
        }
    }
    0
}

// Translated from 00495f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_00495e40`] for the sequence in slot `slot` and the prefix
/// `prefix`: clears `*time` first (when not null), and only accepts a key
/// whose time is at or after the sequence's `float` at +0x3c. Returns the
/// lower case character after the prefix, or 0 (also for no sequence or a
/// null prefix). The map has no name for it.
pub fn fn_00495f50(e: &mut Engine, this: Ptr<Animation>, slot: i32, prefix: Ptr, time: Ptr) -> u8 {
    if !time.is_null() {
        e.mem.set_f32(time.addr(), 0.0);
    }
    let sequence = e.mem.u32(current_sequence_at(this, slot as u32));
    if sequence == 0 || prefix.is_null() {
        return 0;
    }
    let limit = e.call(SEQUENCE_FLOAT_3C, &args![sequence]).f32();
    let (entries, count) = text_keys_of(e, sequence);
    let length = e.call(STRING_LENGTH, &args![prefix]).u32();
    if length == 0 {
        return 0;
    }
    for index in 0..count {
        let entry = entries.wrapping_add(index.wrapping_mul(8));
        let text = text_key_text(e, entry);
        if text != 0 && e.call(STRNICMP, &args![text, prefix, length]).u32() == 0 {
            let key_time = e.call(TEXT_KEY_TIME, &args![entry]).f32();
            if limit <= key_time {
                if !time.is_null() {
                    let key_time = e.call(TEXT_KEY_TIME, &args![entry]).f32();
                    e.mem.set_f32(time.addr(), key_time);
                }
                return lower_next_character(e, text, length);
            }
        }
    }
    0
}

// Translated from 00496080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::ClearGroup` (Xbox PDB): clears the group playing in slot
/// `slot`, blending its sequence out over `blend_time`. The two combined
/// slots run the ladder of single slots: 0x14 clears 7 (and 0 unless
/// `cSkipNextBlend` is set; neither while the menu mode type is 1), 1, and
/// then what 0x15 clears (2 unless the menu mode type is 1, 3); 4, and 0x15,
/// clear 5, 6 and then slot 4 itself. For a single slot below 8 whose
/// sequence has a state, deactivates the sequence (and its `+0x58` partner)
/// on the controller manager and remembers slot 1's sequence as the last
/// movement sequence. Then empties the slot: no sequence, group and next
/// group 0xff, action -1. The slot number is not range checked, as in the
/// game.
pub fn animation_clear_group(e: &mut Engine, this: Ptr<Animation>, slot: i32, blend_time: f32) {
    let a = this.addr();
    let mut slot = slot;
    if slot != 4 {
        let mut combined = true;
        if slot == 0x14 {
            if e.call(MENU_MODE_TYPE, &args![]).i32() != 1 {
                animation_clear_group(e, this, 7, blend_time);
                if e.get(this, Animation::cSkipNextBlend) as i8 == 0 {
                    animation_clear_group(e, this, 0, blend_time);
                }
            }
            animation_clear_group(e, this, 1, blend_time);
        } else if slot != 0x15 {
            combined = false;
        }
        if combined {
            if e.call(MENU_MODE_TYPE, &args![]).i32() != 1 {
                animation_clear_group(e, this, 2, blend_time);
            }
            animation_clear_group(e, this, 3, blend_time);
            animation_clear_group(e, this, 5, blend_time);
            animation_clear_group(e, this, 6, blend_time);
            slot = 4;
        }
    } else {
        animation_clear_group(e, this, 5, blend_time);
        animation_clear_group(e, this, 6, blend_time);
        slot = 4;
    }
    if ni_pointer_get(e, a + Animation::spManager.off) != 0 && slot < 8 {
        let current = e.mem.u32(current_sequence_at(this, slot as u32));
        if current != 0 && e.call(SEQUENCE_STATE, &args![current]).u32() != 0 {
            let partner = e.call(SEQUENCE_WORD_58, &args![current]).u32();
            if partner != 0 {
                let manager = ni_pointer_get(e, a + Animation::spManager.off);
                e.call(MANAGER_DEACTIVATE, &args![manager, partner, blend_time]);
            }
            let manager = ni_pointer_get(e, a + Animation::spManager.off);
            let current = e.mem.u32(current_sequence_at(this, slot as u32));
            e.call(MANAGER_DEACTIVATE, &args![manager, current, blend_time]);
            if slot == 1 {
                let sequence = e.mem.u32(current_sequence_at(this, 1));
                e.set(this, Animation::pLastMovementSequence, Ptr::new(sequence));
            }
        }
    }
    e.mem.set_u32(current_sequence_at(this, slot as u32), 0);
    e.mem.set_u16(group_at(this, slot as u32), 0xff);
    e.mem.set_u16(next_group_at(this, slot as u32), 0xff);
    e.mem.set_i32(action_at(this, slot as u32), -1);
}

// Translated from 004964d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `AccumRootTranslate` (+0x1c) to the zero vector (`ZERO_VECTOR`); the
/// map has no name for it.
pub fn fn_004964d0(e: &mut Engine, this: Ptr<Animation>) {
    let target = this.addr() + Animation::AccumRootTranslate.off;
    for i in 0..3u32 {
        let word = e.mem.u32(ZERO_VECTOR + 4 * i);
        e.mem.set_u32(target + 4 * i, word);
    }
}

// Translated from 00496440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks the translation of the `NiQuatTransform` at `this` invalid (the
/// first float becomes `-FLT_MAX`) when `valid` is 0. The map has no name
/// for it.
pub fn fn_00496440(e: &mut Engine, this: Ptr, valid: u8) {
    if valid == 0 {
        let lowest = -e.global::<f32>(FLOAT_MAX);
        e.mem.set_f32(this.addr(), lowest);
    }
}

// Translated from 00496470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks the rotation of the `NiQuatTransform` at `this` invalid when `valid`
/// is 0: calls `004f5d90` on the rotation (+0xc) with `-FLT_MAX`, which
/// stores it at +4 of the quaternion. The map has no name for it.
pub fn fn_00496470(e: &mut Engine, this: Ptr, valid: u8) {
    if valid == 0 {
        let lowest = -e.global::<f32>(FLOAT_MAX);
        e.call(QUATERNION_STORE_AT_4, &args![this.addr() + 0xc, lowest]);
    }
}

// Translated from 004964a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks the scale of the `NiQuatTransform` at `this` (+0x1c) invalid when
/// `valid` is 0 (`-FLT_MAX`); the map has no name for it.
pub fn fn_004964a0(e: &mut Engine, this: Ptr, valid: u8) {
    if valid == 0 {
        let lowest = -e.global::<f32>(FLOAT_MAX);
        e.mem.set_f32(this.addr() + 0x1c, lowest);
    }
}

// Translated from 004963b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the three-float vector at `translate` into the translation of the
/// `NiQuatTransform` at `this` and marks it valid; the map has no name for
/// it.
pub fn fn_004963b0(e: &mut Engine, this: Ptr, translate: Ptr) {
    for i in 0..3u32 {
        let word = e.mem.u32(translate.addr() + 4 * i);
        e.mem.set_u32(this.addr() + 4 * i, word);
    }
    fn_00496440(e, this, 1);
}

// Translated from 004963e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the four-float quaternion at `rotate` into the rotation (+0xc) of
/// the `NiQuatTransform` at `this` and marks it valid; the map has no name
/// for it.
pub fn fn_004963e0(e: &mut Engine, this: Ptr, rotate: Ptr) {
    for i in 0..4u32 {
        let word = e.mem.u32(rotate.addr() + 4 * i);
        e.mem.set_u32(this.addr() + 0xc + 4 * i, word);
    }
    fn_00496470(e, this, 1);
}

// Translated from 00496420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `scale` in the scale (+0x1c) of the `NiQuatTransform` at `this`
/// and marks it valid; the map has no name for it.
pub fn fn_00496420(e: &mut Engine, this: Ptr, scale: f32) {
    e.mem.set_f32(this.addr() + 0x1c, scale);
    fn_004964a0(e, this, 1);
}

// Translated from 00496340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the transform at +0x30 of `this` (an interpolator; the map has no
/// name for it) and sets the byte at +0x54. When the byte at +0xe is 1 the
/// transform becomes the invalid default ([`fn_00494260`]); otherwise it
/// becomes the zero translation (`ZERO_VECTOR`), the identity quaternion
/// (`QUATERNION_IDENTITY`) and scale 1.0.
pub fn fn_00496340(e: &mut Engine, this: Ptr) {
    let transform = Ptr::<()>::new(this.addr() + 0x30);
    if e.mem.u8(this.addr() + 0xe) == 1 {
        e.with_stack(0x20, |e, record| {
            fn_00494260(e, record);
            for i in 0..8u32 {
                let word = e.mem.u32(record.addr() + 4 * i);
                e.mem.set_u32(transform.addr() + 4 * i, word);
            }
        });
    } else {
        fn_004963b0(e, transform, Ptr::new(ZERO_VECTOR));
        fn_004963e0(e, transform, Ptr::new(QUATERNION_IDENTITY));
        fn_00496420(e, transform, 1.0);
    }
    e.mem.set_u8(this.addr() + 0x54, 1);
}

// Translated from 00496280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the accumulation root of the animation: when `pAccumRoot` is set,
/// zeroes `AccumRootTranslate`, gives the root the matrix at
/// `ACCUM_ROOT_SETUP_ARGUMENT` and a zero translation, then goes through the
/// root's controllers and, for each one that casts to the controller kind
/// whose target casts to the target kind, runs [`fn_00496340`] on that
/// target. The map has no name for it.
pub fn fn_00496280(e: &mut Engine, this: Ptr<Animation>) {
    let root = e.get(this, Animation::pAccumRoot).addr();
    if root == 0 {
        return;
    }
    fn_004964d0(e, this);
    e.call(ACCUM_ROOT_SETUP, &args![root, ACCUM_ROOT_SETUP_ARGUMENT]);
    e.call(NODE_SET_WORLD_TRANSLATION, &args![root, ZERO_VECTOR]);
    let mut controller = e.call(OBJECT_CONTROLLERS, &args![root]).u32();
    while controller != 0 {
        let kind = e
            .call(DYNAMIC_CAST, &args![RTTI_CONTROLLER_KIND, controller])
            .u32();
        if kind != 0 {
            let target = e.vcall(kind, SLOT_CONTROLLER_TARGET, &args![0u32]).u32();
            if target != 0 {
                let target_kind = e
                    .call(DYNAMIC_CAST, &args![RTTI_CONTROLLER_TARGET_KIND, target])
                    .u32();
                if target_kind != 0 {
                    fn_00496340(e, Ptr::new(target_kind));
                }
            }
        }
        controller = e.call(CONTROLLER_NEXT, &args![controller]).u32();
    }
}

// Translated from 00496500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::GetTESAnimGroup` (Xbox PDB): the animation group of the
/// sequence the map entry for `key` currently offers (virtual slot 0x10 with
/// -1), or null when the animation has no entry for `key`.
pub fn animation_get_tes_anim_group(e: &mut Engine, this: Ptr<Animation>, key: u16) -> Ptr {
    let Some(entry) = sequence_map_get(e, this, key) else {
        return Ptr::NULL;
    };
    let sequence = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32();
    fn_0048f7f0(e, Ptr::new(sequence))
}

// Translated from 00496550 (decompiled, FalloutNV.exe 1.4.0.525)
/// The smallest of 1.0 and the value `00c81b90(object, 0)` gives for `object`
/// and, going up through the parents (`009611e0`), for every object until the
/// one stored at +0xc of `this`. The map has no name for it. Returns the
/// `float` in ST0.
pub fn fn_00496550(e: &mut Engine, this: Ptr, object: Ptr) -> f32 {
    let mut lowest = 1.0f32;
    if !object.is_null() {
        let value = e.call(OBJECT_VALUE, &args![object, 0u32]).f32();
        if value < lowest {
            lowest = value;
        }
        if object.addr() != e.mem.u32(this.addr() + 0xc) {
            let parent = e.call(OBJECT_PARENT, &args![object]).ptr::<()>();
            let value = fn_00496550(e, this, parent);
            if value < lowest {
                lowest = value;
            }
        }
    }
    lowest
}

// ---- AnimIdle ----------------------------------------------------------------------

// Translated from 004968b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiRefObject` constructor body, inlined into this unit: sets the
/// vtable, clears the reference count (+4) and counts the object
/// (`InterlockedIncrement` of `ms_uiObjects`). Returns `this`.
pub fn fn_004968b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_NI_REF_OBJECT);
    e.mem.set_u32(this.addr() + 4, 0);
    e.call(INTERLOCKED_INCREMENT, &args![NI_REF_OBJECT_COUNT]);
    this
}

// Translated from 00496910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiRefObject` destructor body, inlined into this unit: sets the vtable
/// back to `NiRefObject`'s and uncounts the object (`InterlockedDecrement`
/// of `ms_uiObjects`).
pub fn fn_00496910(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), VTABLE_NI_REF_OBJECT);
    e.call(INTERLOCKED_DECREMENT, &args![NI_REF_OBJECT_COUNT]);
}

// Translated from 004968e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRefObject::_scalar_deleting_destructor_` (Xbox PDB), as this unit
/// emitted it for an 8-byte object: the destructor body, then
/// `NiMemObject::operator delete(this, 8)` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn ni_ref_object_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00496910(e, this);
    if flags & 1 != 0 {
        e.call(NI_OPERATOR_DELETE, &args![this, 8u32]);
    }
    this
}

// Translated from 00496940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The controller manager (`spManager`, +0xd8) of an `Animation`; the map
/// has no name for it.
pub fn fn_00496940(e: &mut Engine, this: Ptr<Animation>) -> u32 {
    ni_pointer_get(e, this.addr() + Animation::spManager.off)
}

// Translated from 00496960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the byte at +0x14e of `this` (an actor); the map has
/// no name for it.
pub fn fn_00496960(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x14e, value);
}

// Translated from 004965d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimIdle::AnimIdle` (Xbox PDB): builds an idle for `idle_form`, playing in
/// section `section` with play type `play_type` for `actor`'s `animation`.
/// Makes the idle's members empty, looks up the idle form's two animation
/// objects, builds the model path `"Meshes\<model name>"` and loads the KF
/// (`load_kf` set: `ModelLoader::LoadKF`; clear: `ModelLoader` `00445200`,
/// which takes the idle and the actor). When a KF came back, attaches each
/// animation object to the actor's 3D and adds it to the controller
/// manager's palette, adds a reference to the model when `load_kf` is clear
/// and sets the state (+8) to 1; otherwise the state is 0. The compiler's
/// exception frame is not translated. Returns `this`.
// The game's constructor takes this and six words; `e` makes eight.
#[allow(clippy::too_many_arguments)]
pub fn anim_idle_constructor(
    e: &mut Engine,
    this: Ptr<AnimIdle>,
    idle_form: Ptr,
    section: u32,
    play_type: u32,
    actor: Ptr,
    load_kf: u8,
    animation: Ptr,
) -> Ptr<AnimIdle> {
    let a = this.addr();
    fn_004968b0(e, Ptr::new(a));
    e.mem.set_u32(a, VTABLE_ANIM_IDLE);
    e.call(
        KF_MODEL_POINTER_INIT,
        &args![a + AnimIdle::spKFModel.off, 0u32],
    );
    e.call(NI_POINTER_INIT, &args![a + AnimIdle::pSeq.off, 0u32]);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            a + AnimIdle::spAddOnObj.off,
            4u32,
            2u32,
            NI_POINTER_DEFAULT_CONSTRUCT,
            NI_POINTER_RELEASE
        ],
    );
    e.call(NI_POINTER_SET, &args![a + AnimIdle::pSeq.off, 0u32]);
    e.set(this, AnimIdle::eSection, section);
    e.set(this, AnimIdle::eType, play_type);
    e.set(this, AnimIdle::pIdleForm, idle_form);
    for i in 0..2u32 {
        let handler = e.global::<u32>(DATA_HANDLER);
        let object = e
            .call(IDLE_FORM_ANIM_OBJECT, &args![handler, idle_form, i])
            .u32();
        e.mem.set_u32(a + AnimIdle::pAnimObj.off + 4 * i, object);
        e.call(
            NI_POINTER_SET,
            &args![a + AnimIdle::spAddOnObj.off + 4 * i, 0u32],
        );
    }
    e.set(this, AnimIdle::pAnimation, animation);
    e.set(this, AnimIdle::pActor, actor);
    e.with_stack(STRING_LOCAL_SIZE, |e, path| {
        e.call(STRING_CONSTRUCT, &args![path]);
        let name = e
            .vcall(idle_form.addr() + 0x18, SLOT_MODEL_NAME, &args![])
            .u32();
        e.call(
            STRING_FORMAT,
            &args![path, PATH_FORMAT, MESHES_FOLDER, name],
        );
        let loader = model_loader(e);
        if load_kf == 0 {
            let text = ni_pointer_get(e, path.addr());
            let kf = e
                .call(
                    MODEL_LOADER_LOAD_IDLE_KF,
                    &args![loader, text, a, 0u32, actor],
                )
                .u32();
            e.call(
                KF_MODEL_POINTER_SET,
                &args![a + AnimIdle::spKFModel.off, kf],
            );
        } else {
            let text = ni_pointer_get(e, path.addr());
            let kf = e.call(MODEL_LOADER_LOAD_KF, &args![loader, text]).u32();
            e.call(
                KF_MODEL_POINTER_SET,
                &args![a + AnimIdle::spKFModel.off, kf],
            );
        }
        if ni_pointer_get(e, a + AnimIdle::spKFModel.off) != 0 {
            anim_idle_attach_add_ons(e, this, actor);
            if load_kf == 0 {
                let kf = ni_pointer_get(e, a + AnimIdle::spKFModel.off);
                e.call(KF_MODEL_ADD_REF, &args![kf]);
            }
            e.set(this, AnimIdle::eFlags, 1u32);
        } else {
            e.set(this, AnimIdle::eFlags, 0u32);
        }
        e.call(STRING_DESTRUCT, &args![path]);
    });
    this
}

/// For each of the idle's two animation objects (`pAnimObj`) that exists, when
/// `actor` is set: attaches it to the actor's 3D (`LoadAndAttachAddOn`),
/// keeps the result in `spAddOnObj`, adds it to the palette of the actor
/// animation's controller manager and sets the byte at +0x14e of the actor.
/// The common loop of the constructor and [`anim_idle_loaded`].
fn anim_idle_attach_add_ons(e: &mut Engine, this: Ptr<AnimIdle>, actor: Ptr) {
    let a = this.addr();
    for i in 0..2u32 {
        let object = e.mem.u32(a + AnimIdle::pAnimObj.off + 4 * i);
        if object != 0 && !actor.is_null() {
            let node = if object == 0 { 0 } else { object + 0x18 };
            let add_on = e
                .call(
                    LOAD_AND_ATTACH_ADD_ON,
                    &args![object, node, 0xffff_ffffu32, actor, 0u32],
                )
                .u32();
            e.call(
                NI_POINTER_SET,
                &args![a + AnimIdle::spAddOnObj.off + 4 * i, add_on],
            );
            let animation = e.vcall(actor.addr(), SLOT_ACTOR_ANIMATION, &args![]).u32();
            let manager = fn_00496940(e, Ptr::new(animation));
            let palette = e.call(MANAGER_PALETTE, &args![manager]).u32();
            let held = ni_pointer_get(e, a + AnimIdle::spAddOnObj.off + 4 * i);
            e.call(PALETTE_ADD, &args![held, palette]);
            fn_00496960(e, actor, 1);
        }
    }
}

// Translated from 004969b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `AnimIdle` destructor body: sets the vtable, runs [`fn_00496a50`]
/// (which detaches everything), destroys the members (`spAddOnObj`, `pSeq`,
/// `spKFModel`) and the `NiRefObject` base. The compiler's exception frame is
/// not translated.
pub fn fn_004969b0(e: &mut Engine, this: Ptr<AnimIdle>) {
    let a = this.addr();
    e.mem.set_u32(a, VTABLE_ANIM_IDLE);
    fn_00496a50(e, this);
    e.call(
        VECTOR_DESTRUCT,
        &args![a + AnimIdle::spAddOnObj.off, 4u32, 2u32, NI_POINTER_RELEASE],
    );
    e.call(NI_POINTER_RELEASE, &args![a + AnimIdle::pSeq.off]);
    e.call(
        KF_MODEL_POINTER_RELEASE,
        &args![a + AnimIdle::spKFModel.off],
    );
    fn_00496910(e, Ptr::new(a));
}

// Translated from 00496980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimIdle::_scalar_deleting_destructor_` (Xbox PDB): [`fn_004969b0`], then
/// `NiMemObject::operator delete(this, 0x38)` when bit 0 of `flags` is set.
/// Returns `this`.
pub fn anim_idle_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<AnimIdle>,
    flags: u32,
) -> Ptr<AnimIdle> {
    fn_004969b0(e, this);
    if flags & 1 != 0 {
        e.call(NI_OPERATOR_DELETE, &args![this, 0x38u32]);
    }
    this
}

// Translated from 00496a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Detaches an idle from its actor (`__cdecl(idle)`; the map has no name for
/// it): for each attached add-on object it hands the object to the task queue
/// or the shadow scene for removal, takes it out of the controller manager's
/// palette, tells the actor's animation, detaches it from its parent and
/// unloads the model of its animation object, then drops the pointer. Then
/// drops the idle from the model loader while its state is 0, and clears the
/// actor's anim action when it is the idle's sequence.
/// Finally forgets the actor. Does nothing for a null idle.
pub fn fn_00496a50(e: &mut Engine, idle: Ptr<AnimIdle>) {
    if idle.is_null() {
        return;
    }
    let a = idle.addr();
    for i in 0..2u32 {
        let slot = a + AnimIdle::spAddOnObj.off + 4 * i;
        if ni_pointer_get(e, slot) == 0 {
            continue;
        }
        let actor = e.get(idle, AnimIdle::pActor);
        if !actor.is_null() {
            if e.call(TASK_QUEUE_ACTIVE, &args![]).bool() {
                let add_on = ni_pointer_get(e, slot);
                let queue = e.call(TASK_QUEUE, &args![]).u32();
                e.call(TASK_QUEUE_ATTACH, &args![queue, add_on, 0u32]);
            } else {
                let add_on = ni_pointer_get(e, slot);
                let scene = e.call(SHADOW_SCENE_NODE, &args![0u32]).u32();
                e.call(SHADOW_SCENE_NODE_REMOVE, &args![scene, add_on]);
            }
            let animation = e.vcall(actor.addr(), SLOT_ACTOR_ANIMATION, &args![]).u32();
            let manager = fn_00496940(e, Ptr::new(animation));
            let palette = e.call(MANAGER_PALETTE, &args![manager]).u32();
            let add_on = ni_pointer_get(e, slot);
            e.call(PALETTE_REMOVE, &args![add_on, palette]);
            let animation = e.vcall(actor.addr(), SLOT_ACTOR_ANIMATION, &args![]).u32();
            if animation != 0 {
                let add_on = ni_pointer_get(e, slot);
                e.call(ANIMATION_ADD_ON_REMOVED, &args![animation, add_on]);
            }
        }
        if e.call(TASK_QUEUE_ACTIVE, &args![]).bool() {
            let add_on = ni_pointer_get(e, slot);
            let queue = e.call(TASK_QUEUE, &args![]).u32();
            e.call(TASK_QUEUE_DETACH, &args![queue, add_on]);
        } else {
            let add_on = ni_pointer_get(e, slot);
            let parent = e.call(OBJECT_PARENT, &args![add_on]).u32();
            let add_on = ni_pointer_get(e, slot);
            // NiNode::DetachChild-style virtual slot of the parent.
            e.vcall(parent, SLOT_DETACH_CHILD, &args![add_on]);
        }
        let object = e.mem.u32(a + AnimIdle::pAnimObj.off + 4 * i);
        let name = e.vcall(object + 0x18, SLOT_MODEL_NAME, &args![]).u32();
        let loader = model_loader(e);
        e.call(MODEL_LOADER_DROP_MODEL, &args![loader, name]);
        e.call(NI_POINTER_SET, &args![slot, 0u32]);
        if ni_pointer_get(e, a + AnimIdle::pSeq.off) != 0 {
            let sequence = ni_pointer_get(e, a + AnimIdle::pSeq.off);
            e.call(SEQUENCE_RELEASE_HELPER, &args![sequence]);
        }
    }
    if e.get(idle, AnimIdle::eFlags) == 0 {
        let loader = model_loader(e);
        e.call(MODEL_LOADER_DROP_IDLE, &args![loader, idle]);
    }
    let object = e.global::<u32>(SAVE_GAME_OBJECT);
    let actor = e.get(idle, AnimIdle::pActor);
    if !e.call(PIPBOY_QUERY, &args![object]).bool()
        && !actor.is_null()
        && e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32() == 0xd
    {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        let held = e.vcall(process, SLOT_PROCESS_HELD_OBJECT, &args![]).u32();
        let sequence = ni_pointer_get(e, a + AnimIdle::pSeq.off);
        if held == sequence {
            e.call(ACTOR_SET_ANIM_ACTION, &args![actor, 0xffff_ffffu32, 0u32]);
        }
    }
    e.set(idle, AnimIdle::pActor, Ptr::NULL);
}

// Translated from 00496cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AnimIdle::Loaded` (Xbox PDB): the model loader hands the idle its KF
/// (`kf_model`, null when loading failed). Stores it in `spKFModel`; without
/// one the state (+8) becomes 0. With one, attaches the add-on objects
/// ([`anim_idle_attach_add_ons`]), adds a reference to the model and sets the
/// state to 1. Then, for an idle of an actor: with play type 2 or 3 it needs
/// the idle's animation (`pAnimation`, else the actor's); when there is none
/// the idle deletes itself, when `00498290` says no idle is active it frees
/// the animation's special idle (`SpecialIdleFree(1, 0)`), else type 3
/// tells the actor to play the sequence (anim action 0xd); with play type 0
/// it has the animation replace the idle (`00498170`) or deletes itself
/// without one. The compiler's exception frame is not translated.
pub fn anim_idle_loaded(e: &mut Engine, this: Ptr<AnimIdle>, kf_model: Ptr) {
    with_scope_guard(e, 0x1101, |e| {
        let a = this.addr();
        e.call(
            KF_MODEL_POINTER_SET,
            &args![a + AnimIdle::spKFModel.off, kf_model],
        );
        if kf_model.is_null() {
            e.set(this, AnimIdle::eFlags, 0u32);
            return;
        }
        let actor = e.get(this, AnimIdle::pActor);
        anim_idle_attach_add_ons(e, this, actor);
        e.call(KF_MODEL_ADD_REF, &args![kf_model]);
        e.set(this, AnimIdle::eFlags, 1u32);
        let play_type = e.get(this, AnimIdle::eType);
        if !actor.is_null() && (play_type == 2 || play_type == 3) {
            let mut animation = e.get(this, AnimIdle::pAnimation).addr();
            if animation == 0 {
                animation = e.vcall(actor.addr(), SLOT_ACTOR_ANIMATION, &args![]).u32();
            }
            if animation == 0 {
                e.vcall(a, 0, &args![1u32]);
                return;
            }
            if e.call(SPECIAL_IDLE_CHECK, &args![animation]).bool() {
                if play_type == 3 {
                    let sequence = ni_pointer_get(e, a + AnimIdle::pSeq.off);
                    e.call(ACTOR_SET_ANIM_ACTION, &args![actor, 0xdu32, sequence]);
                }
            } else {
                e.call(SPECIAL_IDLE_FREE, &args![animation, 1u32, 0u32]);
            }
        } else if !actor.is_null() && play_type == 0 {
            let mut animation = e.get(this, AnimIdle::pAnimation).addr();
            if animation == 0 {
                animation = e.vcall(actor.addr(), SLOT_ACTOR_ANIMATION, &args![]).u32();
            }
            if animation == 0 {
                e.vcall(a, 0, &args![1u32]);
                return;
            }
            e.call(SPECIAL_IDLE_REPLACE_OV2, &args![animation]);
        }
    });
}

// Translated from 00496fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves an idle from state 1 (loaded) to state 2 (playing): returns 0 in
/// any other state; otherwise stores `sequence` in `pSeq`, sets the state
/// and, for play type 3, tells the actor to play the sequence (anim action
/// 0xd), and returns 1. The map has no name for it.
pub fn fn_00496fe0(e: &mut Engine, this: Ptr<AnimIdle>, sequence: Ptr) -> u8 {
    if e.get(this, AnimIdle::eFlags) != 1 {
        return 0;
    }
    e.call(
        NI_POINTER_SET,
        &args![this.addr() + AnimIdle::pSeq.off, sequence],
    );
    e.set(this, AnimIdle::eFlags, 2u32);
    if e.get(this, AnimIdle::eType) == 3 {
        let held = ni_pointer_get(e, this.addr() + AnimIdle::pSeq.off);
        let actor = e.get(this, AnimIdle::pActor);
        e.call(ACTOR_SET_ANIM_ACTION, &args![actor, 0xdu32, held]);
    }
    1
}

// Translated from 00497040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves an idle from state 2 (playing) to state 3 (done) once its sequence
/// has no state any more (the word at +0x44 is 0): clears the actor's anim
/// action when it is 0xd and, for play type 2 or 3 with an `animation`,
/// frees the animation's special idle (`SpecialIdleFree(1, 0)`). The map has
/// no name for it.
pub fn fn_00497040(e: &mut Engine, this: Ptr<AnimIdle>, animation: Ptr) {
    let a = this.addr();
    if e.get(this, AnimIdle::eFlags) != 2 {
        return;
    }
    if ni_pointer_get(e, a + AnimIdle::pSeq.off) != 0 {
        ni_pointer_get(e, a + AnimIdle::pSeq.off);
        let sequence = ni_pointer_get(e, a + AnimIdle::pSeq.off);
        if e.call(SEQUENCE_STATE, &args![sequence]).u32() != 0 {
            return;
        }
    }
    e.set(this, AnimIdle::eFlags, 3u32);
    let actor = e.get(this, AnimIdle::pActor);
    if !actor.is_null() && e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32() == 0xd {
        e.call(ACTOR_SET_ANIM_ACTION, &args![actor, 0xffff_ffffu32, 0u32]);
    }
    let play_type = e.get(this, AnimIdle::eType);
    if (play_type == 2 || play_type == 3) && !animation.is_null() {
        e.call(SPECIAL_IDLE_FREE, &args![animation, 1u32, 0u32]);
    }
}

// Translated from 004970e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `pSeq` (+0x18) of an idle to `sequence`; the map has no name for it.
pub fn fn_004970e0(e: &mut Engine, this: Ptr<AnimIdle>, sequence: Ptr) {
    e.call(
        NI_POINTER_SET,
        &args![this.addr() + AnimIdle::pSeq.off, sequence],
    );
}

// Translated from 00497100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of bytes an idle's state takes in a save: 13 (the state, the
/// play type, the section and a flag byte), plus the sequence's own save size
/// (`004efb10`) and 1 more when the idle holds a sequence. The map has no
/// name for it.
pub fn fn_00497100(e: &mut Engine, this: Ptr<AnimIdle>) -> u16 {
    let a = this.addr();
    let mut size: u16 = 0;
    for step in [4u16, 4, 4, 1] {
        size = size.wrapping_add(step);
    }
    if ni_pointer_get(e, a + AnimIdle::pSeq.off) != 0 {
        let sequence = ni_pointer_get(e, a + AnimIdle::pSeq.off);
        let own = e.call(SEQUENCE_SAVE_SIZE, &args![sequence]).u16();
        size = size.wrapping_add(own).wrapping_add(1);
    }
    size
}

// Translated from 00497280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `pAnimSequenceMap` (+0xdc) of an `Animation`; the map has no name for
/// it.
pub fn fn_00497280(e: &mut Engine, this: Ptr<Animation>) -> Ptr {
    e.get(this, Animation::pAnimSequenceMap)
}

// Translated from 004974a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `cSkipNextBlend` (+0x120) of an `Animation`; the map has no name for
/// it.
pub fn fn_004974a0(e: &mut Engine, this: Ptr<Animation>) {
    e.set(this, Animation::cSkipNextBlend, 1u8);
}

/// The entry of the animation's sequence map for the group type of an idle's
/// KF model (`GetAt(type of the model's group)`), or `None`.
fn idle_map_entry(e: &mut Engine, this: Ptr<AnimIdle>, animation: Ptr<Animation>) -> Option<u32> {
    let kf = ni_pointer_get(e, this.addr() + AnimIdle::spKFModel.off);
    let group = e.call(KF_MODEL_ANIM_GROUP, &args![kf]).u32();
    let key = e.call(ANIM_GROUP_SEQUENCE_TYPE, &args![group]).u16();
    let map = fn_00497280(e, animation).addr();
    e.with_stack(4, |e, cell| {
        if e.call(MAP_GET_AT, &args![map, key, cell]).bool() {
            Some(e.mem.u32(cell.addr()))
        } else {
            None
        }
    })
}

// Translated from 00497180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes an idle's state to the save game: its state, play type and
/// section (4 bytes each), a flag byte for "holds a sequence" and, when it
/// does, a selector byte (the sequence's index within its map entry, asked of
/// the entry with virtual slot 0x18; 0xff without an entry) and the sequence
/// itself (`004efb20(sequence, time)`). The map has no name for it.
pub fn fn_00497180(e: &mut Engine, this: Ptr<AnimIdle>, time: f32, animation: Ptr<Animation>) {
    let a = this.addr();
    let save = e.global::<u32>(SAVE_GAME_OBJECT);
    for (offset, size) in [(0x8u32, 4u32), (0xc, 4), (0x14, 4)] {
        e.call(SAVE_WRITE, &args![save, a + offset, size]);
    }
    e.with_stack(4, |e, cells| {
        let flag = cells.addr();
        let selector = cells.addr() + 1;
        let held = ni_pointer_get(e, a + AnimIdle::pSeq.off) != 0;
        e.mem.set_u8(flag, held as u8);
        e.call(SAVE_WRITE, &args![save, flag, 1u32]);
        if e.mem.u8(flag) != 0 {
            e.mem.set_u8(selector, 0xff);
            if let Some(entry) = idle_map_entry(e, this, animation) {
                let sequence = ni_pointer_get(e, a + AnimIdle::pSeq.off);
                let index = e.vcall(entry, 0x18, &args![sequence]).u8();
                e.mem.set_u8(selector, index);
            }
            e.call(SAVE_WRITE, &args![save, selector, 1u32]);
            let sequence = ni_pointer_get(e, a + AnimIdle::pSeq.off);
            e.call(SEQUENCE_SAVE, &args![sequence, time]);
        }
    });
}

// Translated from 004972a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads an idle's state back from the save game (the mirror of
/// [`fn_00497180`]). Without a sequence in the save a state 0 becomes 1.
/// With one, adds the idle's animation to `animation`
/// ([`animation_add_animation`]) and then: state 2 starts the entry's
/// sequence in the idle's section (`cSkipNextBlend` set first) and keeps the
/// sequence it returns; state 0 becomes 1; state 3 keeps the entry's sequence
/// without starting it. Finally loads the sequence's own data
/// (`004efbc0(sequence, time)`) and skips the bytes the sequence's save size
/// accounts for. The map has no name for it.
pub fn fn_004972a0(e: &mut Engine, this: Ptr<AnimIdle>, time: f32, animation: Ptr<Animation>) {
    let a = this.addr();
    let save = e.global::<u32>(SAVE_GAME_OBJECT);
    for (offset, size) in [(0x8u32, 4u32), (0xc, 4), (0x14, 4)] {
        e.call(SAVE_READ, &args![save, a + offset, size]);
    }
    e.with_stack(4, |e, cells| {
        let flag = cells.addr();
        let selector = cells.addr() + 1;
        e.call(SAVE_READ, &args![save, flag, 1u32]);
        if e.mem.u8(flag) == 0 {
            if e.get(this, AnimIdle::eFlags) == 0 {
                e.set(this, AnimIdle::eFlags, 1u32);
            }
            return;
        }
        e.call(SAVE_READ, &args![save, selector, 1u32]);
        let kf = ni_pointer_get(e, a + AnimIdle::spKFModel.off);
        if animation_add_animation(e, animation, kf, false) {
            let state = e.get(this, AnimIdle::eFlags);
            if state == 2 {
                if let Some(entry) = idle_map_entry(e, this, animation) {
                    fn_004974a0(e, animation);
                    let slot = e.get(this, AnimIdle::eSection);
                    let kf = ni_pointer_get(e, a + AnimIdle::spKFModel.off);
                    let group = e.call(KF_MODEL_ANIM_GROUP, &args![kf]).u32();
                    let group_type = e.call(ANIM_GROUP_SEQUENCE_TYPE, &args![group]).u16();
                    let wanted = e.mem.u8(selector) as u32;
                    let sequence = e.vcall(entry, 0x10, &args![wanted]).u32();
                    let started = animation_start_group_ov2(
                        e,
                        animation,
                        Ptr::new(sequence),
                        group_type,
                        slot as i32,
                    );
                    e.call(NI_POINTER_SET, &args![a + AnimIdle::pSeq.off, started]);
                }
            } else if state == 0 {
                e.set(this, AnimIdle::eFlags, 1u32);
            } else if state == 3 {
                // The entry cell starts empty here.
                if let Some(entry) = idle_map_entry(e, this, animation) {
                    let wanted = e.mem.u8(selector) as u32;
                    let sequence = e.vcall(entry, 0x10, &args![wanted]).u32();
                    e.call(NI_POINTER_SET, &args![a + AnimIdle::pSeq.off, sequence]);
                }
            }
        }
        if ni_pointer_get(e, a + AnimIdle::pSeq.off) != 0 {
            let sequence = ni_pointer_get(e, a + AnimIdle::pSeq.off);
            e.call(SEQUENCE_LOAD, &args![sequence, time]);
        }
        let skip = e.call(SEQUENCE_SAVE_BASE_SIZE, &args![]).u16();
        e.call(SAVE_SKIP, &args![save, skip as u32]);
    });
}

// Translated from 004974c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of bytes the idle `idle` takes in a save (`__cdecl`; the first
/// word is never read): 4, plus 6 while the game saves in blocks, plus 2 and
/// [`fn_00497100`] when the idle has an idle form. While the diagnostics
/// setting (`SAVE_DIAGNOSTICS_OBJECT`) is on, logs the size with
/// `Error`, naming the record being written when there is one. The map has
/// no name for it.
pub fn fn_004974c0(e: &mut Engine, _unused_0: u32, idle: Ptr<AnimIdle>) -> u16 {
    let mut size: u16 = 0;
    let save = e.global::<u32>(SAVE_GAME_OBJECT);
    if e.call(SAVE_USES_BLOCKS, &args![save]).bool() {
        size = size.wrapping_add(4).wrapping_add(2);
    }
    size = size.wrapping_add(4);
    if !idle.is_null() && e.call(IDLE_FORM_OF, &args![idle]).u32() != 0 {
        size = size.wrapping_add(2);
        let own = fn_00497100(e, idle);
        size = size.wrapping_add(own);
    }
    let diagnostics = e
        .call(SETTING_BYTE_ADDRESS, &args![SAVE_DIAGNOSTICS_OBJECT])
        .u32();
    if e.mem.u8(diagnostics) != 0 {
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        let record = e.call(SAVE_RECORD_WRITTEN, &args![save]).u32();
        let written = size as u32;
        if record != 0 {
            let form_id = e.mem.u32(record);
            let kind = e.call(FORM_BY_ID, &args![form_id]).u32();
            let name = e.vcall(kind, SLOT_FORM_TYPE_NAME, &args![]).u32();
            let flags = e.mem.u32(record + 5);
            e.call(
                ERROR_LOG,
                &args![
                    LOG_SAVE_SIZE_FORM,
                    written,
                    form_id,
                    name,
                    flags,
                    0x11f3u32,
                    SOURCE_FILE
                ],
            );
        } else {
            e.call(
                ERROR_LOG,
                &args![LOG_SAVE_SIZE, written, 0x11f3u32, SOURCE_FILE],
            );
        }
    }
    size
}

/// Logs, when the diagnostics setting is on, how many bytes the save
/// function wrote since `start`: the part of [`fn_004975e0`] after the
/// payload.
fn log_saved_size(e: &mut Engine, start: u32) {
    log_written_since(e, start, 0x120e, LOG_SAVE_GAME_FORM, LOG_SAVE_GAME);
}

// Translated from 004975e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes an idle to the save game (`__cdecl`; the first word is never
/// read): inside a save block when blocks are in use (the tag `"KOLB"`, then
/// a 16-bit length that is patched in at the end, with a log message when the
/// block is longer than 0xffff), the form ID of the idle form (0 without
/// one), and, for a non-zero ID, the idle's save size and state
/// ([`fn_00497180`]). Logs the written size when the diagnostics setting is
/// on. The map has no name for it.
pub fn fn_004975e0(
    e: &mut Engine,
    _unused_0: u32,
    idle: Ptr<AnimIdle>,
    animation: Ptr<Animation>,
    time: f32,
) {
    let save = e.global::<u32>(SAVE_GAME_OBJECT);
    let mut block_start = 0u32;
    let start = e.call(SAVE_POSITION, &args![save]).u32();
    let diagnostics = e
        .call(SETTING_BYTE_ADDRESS, &args![SAVE_DIAGNOSTICS_OBJECT])
        .u32();
    let mut start = start;
    if e.mem.u8(diagnostics) != 0 {
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        start = e.call(SAVE_POSITION, &args![save]).u32();
    }
    e.with_stack(0x10, |e, cells| {
        let tag = cells.addr();
        let length = cells.addr() + 4;
        let form_id = cells.addr() + 8;
        let own_size = cells.addr() + 12;
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        let uses_blocks = e.call(SAVE_USES_BLOCKS, &args![save]).bool();
        if uses_blocks {
            e.mem.set_u32(tag, SAVE_BLOCK_TAG);
            e.call(SAVE_WRITE, &args![save, tag, 4u32]);
            let save = e.global::<u32>(SAVE_GAME_OBJECT);
            block_start = e.call(SAVE_POSITION, &args![save]).u32();
            e.mem.set_u16(length, 0);
            e.call(SAVE_WRITE, &args![save, length, 2u32]);
        }
        e.mem.set_u32(form_id, 0);
        if !idle.is_null() && e.call(IDLE_FORM_OF, &args![idle]).u32() != 0 {
            let form = e.call(IDLE_FORM_OF, &args![idle]).u32();
            let id = e.call(WORD_AT_C, &args![form]).u32();
            e.mem.set_u32(form_id, id);
        }
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        e.call(SAVE_FORM_ID, &args![save, form_id, 4u32]);
        if e.mem.u32(form_id) != 0 {
            let size = fn_00497100(e, idle);
            e.mem.set_u16(own_size, size);
            let save = e.global::<u32>(SAVE_GAME_OBJECT);
            e.call(SAVE_WRITE, &args![save, own_size, 2u32]);
            fn_00497180(e, idle, time, animation);
        }
    });
    log_saved_size(e, start);
    let save = e.global::<u32>(SAVE_GAME_OBJECT);
    if e.call(SAVE_USES_BLOCKS, &args![save]).bool() {
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        let end = e.call(SAVE_POSITION, &args![save]).u32();
        if end > block_start.wrapping_add(0xffff) {
            e.call(LOG, &args![LOG_SAVE_BLOCK_TOO_BIG, SOURCE_FILE, 0x120eu32]);
        }
        e.mem
            .set_u16(block_start, end.wrapping_sub(block_start) as u16);
    }
}

// Translated from 004977e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads an idle back from the save game (`__cdecl(actor, animation,
/// time)`): the mirror of [`fn_004975e0`]. Checks the block tag when blocks
/// are in use, reads the form ID; for a non-zero ID or flag byte reads the
/// idle's length, and when the ID casts to a `TESIdleForm` builds the idle
/// (section 7, play type 1, `load_kf` set, no animation) for `actor` and has
/// [`fn_004972a0`] fill it in, otherwise logs the unknown form and skips the
/// bytes. Logs when the amount read differs from the block's length. Returns
/// the new idle, or 0. The map has no name for it.
pub fn fn_004977e0(
    e: &mut Engine,
    actor: Ptr,
    animation: Ptr<Animation>,
    time: f32,
) -> Ptr<AnimIdle> {
    with_scope_guard(e, 0x1218, |e| {
        let mut idle = Ptr::<AnimIdle>::NULL;
        let mut length = 0u16;
        let mut block_start = 0u32;
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        e.with_stack(0x10, |e, cells| {
            let tag = cells.addr();
            let size = cells.addr() + 4;
            let form_id = cells.addr() + 8;
            let skip = cells.addr() + 12;
            if e.call(SAVE_USES_BLOCKS, &args![save]).bool() {
                e.call(SAVE_READ, &args![save, tag, 4u32]);
                if e.mem.u32(tag) != SAVE_BLOCK_TAG {
                    let record = e.call(SAVE_RECORD_READ, &args![save]).u32();
                    if record != 0 {
                        let record_id = e.mem.u32(record);
                        let kind = e.call(FORM_BY_ID, &args![record_id]).u32();
                        let name = e.vcall(kind, SLOT_FORM_TYPE_NAME, &args![]).u32();
                        let flags = e.mem.u32(record + 5);
                        let extra = e.mem.u8(record + 9) as u32;
                        e.call(
                            LOG,
                            &args![
                                LOG_LOAD_NO_BLOCK_FORM,
                                SOURCE_FILE,
                                0x121cu32,
                                record_id,
                                name,
                                extra,
                                flags
                            ],
                        );
                    } else {
                        let version = e.call(SAVE_BYTE_80, &args![save]).u8() as u32;
                        e.call(
                            LOG,
                            &args![LOG_LOAD_NO_BLOCK, SOURCE_FILE, 0x121cu32, version],
                        );
                    }
                }
                block_start = e.call(SAVE_POSITION, &args![save]).u32();
                e.call(SAVE_READ, &args![save, size, 2u32]);
                length = e.mem.u16(size);
            }
            let found = e.call(LOAD_FORM_ID, &args![save, form_id, 4u32]).u8();
            if e.mem.u32(form_id) != 0 || found != 0 {
                e.call(SAVE_READ, &args![save, skip, 2u32]);
                let mut idle_form = 0u32;
                if e.mem.u32(form_id) != 0 {
                    let form = e.call(FORM_BY_ID, &args![e.mem.u32(form_id)]).u32();
                    idle_form = e
                        .call(
                            RT_DYNAMIC_CAST,
                            &args![
                                form,
                                0u32,
                                TYPE_DESCRIPTOR_FORM,
                                TYPE_DESCRIPTOR_IDLE_FORM,
                                0u32
                            ],
                        )
                        .u32();
                }
                if idle_form != 0 {
                    let block = e.call(NI_OPERATOR_NEW, &args![0x38u32]).u32();
                    if block != 0 {
                        idle = anim_idle_constructor(
                            e,
                            Ptr::new(block),
                            Ptr::new(idle_form),
                            7,
                            1,
                            actor,
                            1,
                            Ptr::NULL,
                        );
                    }
                    fn_004972a0(e, idle, time, animation);
                } else {
                    let name = e.vcall(actor.addr(), SLOT_FORM_TYPE_NAME, &args![]).u32();
                    let id = e.mem.u32(form_id);
                    e.call(LOG, &args![LOG_LOAD_UNKNOWN_IDLE, id, name]);
                    let count = e.mem.u16(skip) as u32;
                    e.call(SAVE_SKIP, &args![save, count]);
                }
            }
        });
        if e.call(SAVE_USES_BLOCKS, &args![save]).bool() {
            let end = e.call(SAVE_POSITION, &args![save]).u32();
            let record = e.call(SAVE_RECORD_READ, &args![save]).u32();
            let expected = (length as u32).wrapping_add(block_start);
            if record != 0 {
                let record_id = e.mem.u32(record);
                let kind = e.call(FORM_BY_ID, &args![record_id]).u32();
                let flags = e.mem.u32(record + 5);
                let extra = e.mem.u8(record + 9) as u32;
                if end > expected {
                    let name = e.vcall(kind, SLOT_FORM_TYPE_NAME, &args![]).u32();
                    e.call(
                        LOG,
                        &args![
                            LOG_LOAD_LONG_FORM,
                            end.wrapping_sub(expected),
                            SOURCE_FILE,
                            0x1237u32,
                            record_id,
                            name,
                            extra,
                            flags
                        ],
                    );
                } else if end < expected {
                    let name = e.vcall(kind, SLOT_FORM_TYPE_NAME, &args![]).u32();
                    e.call(
                        LOG,
                        &args![
                            LOG_LOAD_SHORT_FORM,
                            expected.wrapping_sub(end),
                            SOURCE_FILE,
                            0x1237u32,
                            record_id,
                            name,
                            extra,
                            flags
                        ],
                    );
                }
            } else if end > expected {
                let version = e.call(SAVE_BYTE_80, &args![save]).u8() as u32;
                e.call(
                    LOG,
                    &args![
                        LOG_LOAD_LONG,
                        end.wrapping_sub(expected),
                        SOURCE_FILE,
                        0x1237u32,
                        version
                    ],
                );
            } else if end < expected {
                let version = e.call(SAVE_BYTE_80, &args![save]).u8() as u32;
                e.call(
                    LOG,
                    &args![
                        LOG_LOAD_SHORT,
                        expected.wrapping_sub(end),
                        SOURCE_FILE,
                        0x1237u32,
                        version
                    ],
                );
            }
        }
        idle
    })
}

// Translated from 00497bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes an idle's state, play type and section (4 bytes each, at +0x8, +0xc
/// and +0x14) to a `BGSSaveGameBuffer` (`buffer`). The second word is never
/// read. The map has no name for it.
pub fn fn_00497bc0(e: &mut Engine, this: Ptr<AnimIdle>, buffer: Ptr, _unused_1: u32) {
    for offset in [0x8u32, 0xc, 0x14] {
        e.call(
            SAVE_BUFFER_FIELD,
            &args![buffer, this.addr() + offset, 4u32, 0u32],
        );
    }
}

// Translated from 00497c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads an idle's state, play type and section from a `BGSLoadGameBuffer`
/// (`buffer`); a state 0 becomes 1 and, in state 2 with a KF model, adds the
/// idle's animation to `animation`. The map has no name for it.
pub fn fn_00497c10(e: &mut Engine, this: Ptr<AnimIdle>, buffer: Ptr, animation: Ptr<Animation>) {
    for offset in [0x8u32, 0xc, 0x14] {
        e.call(
            LOAD_BUFFER_FIELD,
            &args![buffer, this.addr() + offset, 4u32],
        );
    }
    if e.get(this, AnimIdle::eFlags) == 0 {
        e.set(this, AnimIdle::eFlags, 1u32);
    }
    if e.get(this, AnimIdle::eFlags) == 2 {
        let kf = ni_pointer_get(e, this.addr() + AnimIdle::spKFModel.off);
        if kf != 0 {
            let kf = ni_pointer_get(e, this.addr() + AnimIdle::spKFModel.off);
            animation_add_animation(e, animation, kf, false);
        }
    }
}

// ---- SpecialIdleQueue and its constructor helper --------------------------------------

// Translated from 00497ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SpecialIdleQueue` (Xbox PDB): queues the idle form
/// `idle_form` to play in section `section`. When an idle is already queued
/// (`spAnimIdleQueued`, +0x124): maps section 0x14 to slot 1 and 0x15 to slot
/// 4; when that idle's sequence is the current sequence of the slot, a
/// sequence in state 2 or 5 means it just started (logs and gives up),
/// otherwise blends the slot out; then moves the queued idle into the first
/// free `spAnimIdleFreeWhenInactiveA` entry, or frees it (`AnimIdleFree`) when
/// both are taken. Then builds the new idle (play type 1, no `load_kf`
/// flag) and queues it. The compiler's exception frame is not translated.
pub fn animation_special_idle_queue(
    e: &mut Engine,
    this: Ptr<Animation>,
    idle_form: Ptr,
    section: u32,
) {
    let a = this.addr();
    with_scope_guard(e, 0x125d, |e| {
        let current_field = a + Animation::spAnimIdle.off;
        if ni_pointer_get(e, current_field) != 0 {
            let idle = ni_pointer_get(e, current_field);
            let mut slot = e.call(WORD_AT_14, &args![idle]).i32();
            if slot == 0x14 {
                slot = 1;
            } else if slot == 0x15 {
                slot = 4;
            }
            let idle = ni_pointer_get(e, current_field);
            if fn_00490e40(e, Ptr::new(idle)).addr() != 0 {
                let idle = ni_pointer_get(e, current_field);
                let sequence = fn_00490e40(e, Ptr::new(idle)).addr();
                if e.mem.u32(current_sequence_at(this, slot as u32)) == sequence {
                    let idle = ni_pointer_get(e, current_field);
                    let sequence = fn_00490e40(e, Ptr::new(idle)).addr();
                    let state = e.call(SEQUENCE_STATE, &args![sequence]).i32();
                    if state == 2 || state == 5 {
                        let idle = ni_pointer_get(e, current_field);
                        let sequence = fn_00490e40(e, Ptr::new(idle)).addr();
                        let name = object_name(e, sequence);
                        e.call(LOG, &args![LOG_QUEUE_JUST_STARTED, name]);
                        return false;
                    }
                    let idle = ni_pointer_get(e, current_field);
                    let section_of_idle = e.call(WORD_AT_14, &args![idle]).u32();
                    e.call(BLEND_OUT, &args![this, section_of_idle, 0u32]);
                }
            }
            let first = a + Animation::spAnimIdleFreeWhenInactiveA.off;
            if ni_pointer_get(e, first) == 0 {
                e.call(NI_POINTER_COPY, &args![first, current_field]);
                e.call(NI_POINTER_SET, &args![current_field, 0u32]);
            } else if ni_pointer_get(e, first + 4) == 0 {
                e.call(NI_POINTER_COPY, &args![first + 4, current_field]);
                e.call(NI_POINTER_SET, &args![current_field, 0u32]);
            } else {
                e.call(ANIM_IDLE_FREE, &args![this, current_field]);
            }
        }
        let block = e.call(NI_OPERATOR_NEW, &args![0x38u32]).u32();
        let mut idle = Ptr::<AnimIdle>::NULL;
        if block != 0 {
            let actor = e.get(this, Animation::pActorRef);
            idle = anim_idle_constructor(
                e,
                Ptr::new(block),
                idle_form,
                section,
                1,
                actor,
                0,
                Ptr::NULL,
            );
        }
        e.call(NI_POINTER_SET, &args![current_field, idle]);
        true
    });
}

// Translated from 00497f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds an idle for `idle_form` and `actor` in section `section` as the
/// animation's queued idle (`spAnimIdleQueued`, +0x128), with the play type
/// `play_type`, which is forced to 3 for the sections 0, 1 and 0x14. Hands the
/// idle form to the actor's process (virtual slot 0x390) and runs `00498290`
/// on the animation. The map has no name for it. The compiler's exception
/// frame is not translated.
pub fn fn_00497f20(
    e: &mut Engine,
    this: Ptr<Animation>,
    idle_form: Ptr,
    actor: Ptr,
    section: i32,
    play_type: u32,
) {
    with_scope_guard(e, 0x129d, |e| {
        let mut play_type = play_type;
        if section >= 0 && (section <= 1 || section == 0x14) {
            play_type = 3;
        }
        let block = e.call(NI_OPERATOR_NEW, &args![0x38u32]).u32();
        let mut idle = Ptr::<AnimIdle>::NULL;
        if block != 0 {
            idle = anim_idle_constructor(
                e,
                Ptr::new(block),
                idle_form,
                section as u32,
                play_type,
                actor,
                0,
                Ptr::new(this.addr()),
            );
        }
        e.call(
            NI_POINTER_SET,
            &args![this.addr() + Animation::spAnimIdleQueued.off, idle],
        );
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        e.vcall(process, SLOT_PROCESS_SET_IDLE, &args![idle_form]);
        e.call(SPECIAL_IDLE_CHECK, &args![this]);
    });
}

// ---- Special idles, fourth session (`00498030` onward) ----------------------------------

/// The address of the `NiPointer<AnimIdle>` field `offset` of an animation.
fn idle_field(this: Ptr<Animation>, offset: u32) -> u32 {
    this.addr() + offset
}

/// The idle held in the `NiPointer<AnimIdle>` field `offset` of an animation.
fn idle_in(e: &mut Engine, this: Ptr<Animation>, offset: u32) -> Ptr<AnimIdle> {
    Ptr::new(ni_pointer_get(e, idle_field(this, offset)))
}

/// The state (`eFlags`) of an idle, read through `0044ddc0`.
fn idle_state(e: &mut Engine, idle: Ptr<AnimIdle>) -> u32 {
    e.call(WORD_AT_8, &args![idle]).u32()
}

// Translated from 00498030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SpecialIdleReplace` (Xbox PDB): builds an idle for `idle_form`
/// and `actor` in `section` (play type 0) and stores it as the queued idle
/// (`spAnimIdleQueued`, +0x128). While the flag object `FLAG_OBJECT_GLOBAL`
/// says so (`0042ce10`) the idle is built without an animation and with the
/// `load_kf` flag set, otherwise it is built for this animation without it.
/// Then runs [`animation_special_idle_replace_ov2`]. The scope guard of source
/// line 0x12bd is kept; the compiler's exception frame is not translated.
pub fn animation_special_idle_replace(
    e: &mut Engine,
    this: Ptr<Animation>,
    idle_form: Ptr,
    actor: Ptr,
    section: u32,
) {
    with_scope_guard(e, 0x12bd, |e| {
        let flag_object: u32 = e.global(FLAG_OBJECT_GLOBAL);
        let (load_kf, animation) = if e.call(FLAG_OBJECT_CHECK, &args![flag_object]).bool() {
            (1u8, Ptr::NULL)
        } else {
            (0u8, Ptr::new(this.addr()))
        };
        let block = e.call(NI_OPERATOR_NEW, &args![0x38u32]).u32();
        let mut idle = Ptr::<AnimIdle>::NULL;
        if block != 0 {
            idle = anim_idle_constructor(
                e,
                Ptr::new(block),
                idle_form,
                section,
                0,
                actor,
                load_kf,
                animation,
            );
        }
        e.call(
            NI_POINTER_SET,
            &args![idle_field(this, Animation::spAnimIdleQueued.off), idle],
        );
        animation_special_idle_replace_ov2(e, this);
    });
}

// Translated from 00498170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SpecialIdleReplace_ov2` (Xbox PDB): when the queued idle
/// (`spAnimIdleQueued`) is loaded (state 1), adds its KF model to the
/// animation (`AddAnimation(model, 0)`); if that worked, drops the queued
/// idle and returns true, otherwise false.
pub fn animation_special_idle_replace_ov2(e: &mut Engine, this: Ptr<Animation>) -> bool {
    let queued = Animation::spAnimIdleQueued.off;
    if idle_in(e, this, queued).addr() != 0 {
        let idle = idle_in(e, this, queued);
        if idle_state(e, idle) == 1 {
            let idle = idle_in(e, this, queued);
            let kf = e.call(IDLE_KF_MODEL, &args![idle]).u32();
            if animation_add_animation(e, this, kf, false) {
                e.call(NI_POINTER_SET, &args![idle_field(this, queued), 0u32]);
                return true;
            }
        }
    }
    false
}

// Translated from 004981f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SpecialIdleLoaded` (Xbox PDB): whether the current idle
/// (`spAnimIdle`, +0x124) exists and is loaded (state 1).
pub fn animation_special_idle_loaded(e: &mut Engine, this: Ptr<Animation>) -> bool {
    let current = Animation::spAnimIdle.off;
    if idle_in(e, this, current).addr() == 0 {
        return false;
    }
    let idle = idle_in(e, this, current);
    idle_state(e, idle) == 1
}

// Translated from 00498230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs [`fn_00498290`]; when it worked, hands the current idle `actor` (the
/// idle's `pActor`, `008d7dc0`) and tells the actor to play the idle's
/// sequence (`SetAnimAction(0xd, sequence)`). Returns whether it worked. The
/// map has no name for it.
pub fn fn_00498230(e: &mut Engine, this: Ptr<Animation>, actor: Ptr) -> bool {
    if !fn_00498290(e, this) {
        return false;
    }
    let current = Animation::spAnimIdle.off;
    let idle = idle_in(e, this, current);
    e.call(IDLE_SET_ACTOR, &args![idle, actor]);
    let idle = idle_in(e, this, current);
    let sequence = fn_00490e40(e, idle.cast()).addr();
    e.call(ACTOR_SET_ANIM_ACTION, &args![actor, 0xdu32, sequence]);
    true
}

// Translated from 00498290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts the loaded current idle (the map has no name for it). First clears
/// the queued idle: one that is not loaded (state other than 1) makes this
/// return false, a loaded one is freed with `SpecialIdleFree(0, 1)`. Then the
/// current idle must be loaded. Adds its KF model to the animation; when that
/// fails (or the model has no group), marks the idle done (state 3) and, when
/// its play type is 2 or 3, frees it (`SpecialIdleFree(1, 0)`), returning
/// false. Otherwise plays the model's group (`PlayGroup`: in the idle's
/// section, mode 1, with no loop count for section 0 and a group of type 1,
/// otherwise with the idle form's byte as the loop count); a failed play ends
/// like a failed add. When it started a sequence, queues a replay delay
/// (idle form, `IDLE_FORM_WORD` as seconds) when the idle form's word is not
/// 0, moves the idle to state 2 with the sequence ([`fn_00496fe0`]) and
/// returns true.
pub fn fn_00498290(e: &mut Engine, this: Ptr<Animation>) -> bool {
    let a = this.addr();
    let queued = Animation::spAnimIdleQueued.off;
    let current = Animation::spAnimIdle.off;
    if idle_in(e, this, queued).addr() != 0 {
        let idle = idle_in(e, this, queued);
        if idle_state(e, idle) != 1 {
            return false;
        }
        animation_special_idle_free(e, this, 0, 1);
    }
    if idle_in(e, this, current).addr() == 0 {
        return false;
    }
    let idle = idle_in(e, this, current);
    if idle_state(e, idle) != 1 {
        return false;
    }
    let idle = idle_in(e, this, current);
    let kf = e.call(IDLE_KF_MODEL, &args![idle]).u32();
    let mut started = 0u32;
    let mut loaded = false;
    if kf != 0 && animation_add_animation(e, this, kf, false) {
        loaded = e.call(KF_MODEL_ANIM_GROUP, &args![kf]).u32() != 0;
    }
    if loaded {
        let mut played = false;
        let idle = idle_in(e, this, current);
        if e.call(WORD_AT_14, &args![idle]).u32() == 0 {
            let idle = idle_in(e, this, current);
            let model = e.call(IDLE_KF_MODEL, &args![idle]).u32();
            let group = e.call(KF_MODEL_ANIM_GROUP, &args![model]).u32();
            if e.call(ANIM_GROUP_SEQUENCE_TYPE, &args![group]).i32() == 1 {
                let idle = idle_in(e, this, current);
                let section = e.call(WORD_AT_14, &args![idle]).i32();
                let group = e.call(KF_MODEL_ANIM_GROUP, &args![kf]).u32();
                let group_type = e.call(ANIM_GROUP_SEQUENCE_TYPE, &args![group]).u16();
                started = animation_play_group(e, this, group_type, 1, -1, section);
                played = true;
            }
        }
        if !played {
            let idle = idle_in(e, this, current);
            let section = e.call(WORD_AT_14, &args![idle]).i32();
            let idle = idle_in(e, this, current);
            let form = e.call(IDLE_FORM_OF, &args![idle]).u32();
            let loops = e.call(IDLE_FORM_BYTE, &args![form]).u8();
            let group = e.call(KF_MODEL_ANIM_GROUP, &args![kf]).u32();
            let group_type = e.call(ANIM_GROUP_SEQUENCE_TYPE, &args![group]).u16();
            started = animation_play_group(e, this, group_type, 1, loops as i32, section);
        }
    }
    if started != 0 {
        let idle = idle_in(e, this, current);
        let form = e.call(IDLE_FORM_OF, &args![idle]).u32();
        let delay = e.call(IDLE_FORM_WORD, &args![form]).u16();
        if delay > 0 {
            let item = e.call(OPERATOR_NEW, &args![8u32]).u32();
            e.with_stack(4, |e, cell| {
                e.mem.set_u32(cell.addr(), item);
                let idle = idle_in(e, this, current);
                let form = e.call(IDLE_FORM_OF, &args![idle]).u32();
                e.mem.set_u32(item, form);
                let idle = idle_in(e, this, current);
                let form = e.call(IDLE_FORM_OF, &args![idle]).u32();
                let delay = e.call(IDLE_FORM_WORD, &args![form]).u16();
                e.mem.set_f32(item + 4, delay as f32);
                e.call(
                    SIMPLE_LIST_PUSH_BACK,
                    &args![a + Animation::replayDelayList.off, cell],
                );
            });
        }
        let idle = idle_in(e, this, current);
        fn_00496fe0(e, idle, Ptr::new(started));
        return true;
    }
    // The failed path.
    let idle = idle_in(e, this, current);
    e.call(IDLE_SET_WORD_8, &args![idle, 3u32]);
    let idle = idle_in(e, this, current);
    let play_type = e.call(WORD_AT_C, &args![idle]).u32();
    if play_type == 2 {
        animation_special_idle_free(e, this, 1, 0);
    } else {
        let idle = idle_in(e, this, current);
        if e.call(WORD_AT_C, &args![idle]).u32() == 3 {
            animation_special_idle_free(e, this, 1, 0);
        }
    }
    false
}

// Translated from 004985b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SpecialIdlePlaying` (Xbox PDB): whether the current idle
/// exists and is playing (state 2).
pub fn animation_special_idle_playing(e: &mut Engine, this: Ptr<Animation>) -> bool {
    let current = Animation::spAnimIdle.off;
    if idle_in(e, this, current).addr() == 0 {
        return false;
    }
    let idle = idle_in(e, this, current);
    idle_state(e, idle) == 2
}

// Translated from 004985f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SpecialIdleDonePlaying` (Xbox PDB): true when no idle is
/// queued and either there is no current idle or it holds a sequence and is
/// done (state 3).
pub fn animation_special_idle_done_playing(e: &mut Engine, this: Ptr<Animation>) -> bool {
    let current = Animation::spAnimIdle.off;
    if idle_in(e, this, Animation::spAnimIdleQueued.off).addr() != 0 {
        return false;
    }
    if idle_in(e, this, current).addr() == 0 {
        return true;
    }
    let idle = idle_in(e, this, current);
    if fn_00490e40(e, idle.cast()).addr() == 0 {
        return false;
    }
    let idle = idle_in(e, this, current);
    idle_state(e, idle) == 3
}

// Translated from 00498670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::AnimIdleFree` (Xbox PDB): frees the idle held in the
/// `NiPointer<AnimIdle>` at `idle_ref` (one of the animation's idle fields).
/// Looks up the map entry for the idle's model group; an entry whose
/// current sequence is not the idle's is ignored (as if there were none). With
/// no entry the idle's sequence is kept in a local `NiPointer`. When the idle's
/// sequence is current in the idle's section, clears that section
/// (`ClearGroup(section, 0)`). Unless the animation is shut down, removes the
/// sequence from the controller manager (and forgets it as the last movement
/// sequence). Then releases the idle (`fn_00496a50`), empties `idle_ref`, and
/// removes the map entry (deleting the entry through virtual slot 0), or the
/// map key when there is neither an entry nor a held sequence. The compiler's
/// exception frame is not translated.
pub fn animation_anim_idle_free(e: &mut Engine, this: Ptr<Animation>, idle_ref: Ptr) {
    let a = this.addr();
    let idle = ni_pointer_get(e, idle_ref.addr());
    let kf = e.call(IDLE_KF_MODEL, &args![idle]).u32();
    let mut entry = 0u32;
    let mut key: u16 = 0xff;
    e.with_stack(8, |e, cells| {
        let held = cells.addr();
        let cell = cells.addr() + 4;
        e.mem.set_u32(cell, 0);
        e.call(NI_POINTER_INIT, &args![held, 0u32]);
        if kf != 0 && e.call(KF_MODEL_ANIM_GROUP, &args![kf]).u32() != 0 {
            let group = e.call(KF_MODEL_ANIM_GROUP, &args![kf]).u32();
            key = e.call(ANIM_GROUP_ID, &args![group]).u16();
            let map = e.get(this, Animation::pAnimSequenceMap).addr();
            e.call(MAP_GET_AT, &args![map, key, cell]);
            entry = e.mem.u32(cell);
            let mut mismatch = false;
            if entry != 0 {
                let sequence = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32();
                let idle = ni_pointer_get(e, idle_ref.addr());
                if sequence != fn_00490e40(e, Ptr::new(idle)).addr() {
                    entry = 0;
                    mismatch = true;
                }
            }
            if !mismatch {
                let idle = ni_pointer_get(e, idle_ref.addr());
                let slot = e.call(WORD_AT_14, &args![idle]).u32();
                let idle = ni_pointer_get(e, idle_ref.addr());
                let sequence = fn_00490e40(e, Ptr::new(idle)).addr();
                if e.mem.u32(current_sequence_at(this, slot)) == sequence {
                    let idle = ni_pointer_get(e, idle_ref.addr());
                    let slot = e.call(WORD_AT_14, &args![idle]).i32();
                    animation_clear_group(e, this, slot, 0.0);
                }
            }
        }
        if entry == 0 {
            let idle = ni_pointer_get(e, idle_ref.addr());
            let sequence = fn_00490e40(e, Ptr::new(idle));
            e.call(NI_POINTER_SET, &args![held, sequence]);
        }
        if !e.get(this, Animation::bShutDown) {
            if entry != 0 {
                let sequence = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32();
                if sequence == e.get(this, Animation::pLastMovementSequence).addr() {
                    e.set(this, Animation::pLastMovementSequence, Ptr::NULL);
                }
                let sequence = e.vcall(entry, 0x10, &args![0xffff_ffffu32]).u32();
                let manager = ni_pointer_get(e, a + Animation::spManager.off);
                e.call(MANAGER_REMOVE_SEQUENCE, &args![manager, sequence]);
            } else if ni_pointer_get(e, held) != 0 {
                let last = e.get(this, Animation::pLastMovementSequence).addr();
                if e.call(NODE_ACCEPTS_OBJECT, &args![held, last]).bool() {
                    e.set(this, Animation::pLastMovementSequence, Ptr::NULL);
                }
                let sequence = ni_pointer_get(e, held);
                let manager = ni_pointer_get(e, a + Animation::spManager.off);
                e.call(MANAGER_REMOVE_SEQUENCE, &args![manager, sequence]);
            }
        }
        let idle = ni_pointer_get(e, idle_ref.addr());
        fn_00496a50(e, Ptr::new(idle));
        e.call(NI_POINTER_SET, &args![idle_ref, 0u32]);
        if entry != 0 {
            let map = e.get(this, Animation::pAnimSequenceMap).addr();
            e.call(MAP_REMOVE_AT, &args![map, key]);
            // The entry's scalar deleting destructor (slot 0, flags 1).
            e.vcall(entry, 0, &args![1u32]);
            entry = 0;
        }
        if ni_pointer_get(e, held) == 0 && entry == 0 {
            let map = e.get(this, Animation::pAnimSequenceMap).addr();
            e.call(MAP_REMOVE_AT, &args![map, key]);
        }
        e.call(NI_POINTER_RELEASE, &args![held]);
    });
}

/// The first half of `SpecialIdleFree` for the idle in the field `field`
/// (the `NiPointer<AnimIdle>` at that offset): when the idle's sequence is the
/// current sequence of its slot (section 0x14 is slot 1, 0x15 slot 4), either
/// forgets the sequence (`immediate`) or blends the section out. Returns false
/// when the sequence just started (state 2 or 5, nothing is freed then). The
/// idle that sits in `spAnimIdle` is blended out with the raw section, the
/// queued one with the slot; `raw_section` says which.
fn special_idle_blend_out(
    e: &mut Engine,
    this: Ptr<Animation>,
    field: u32,
    raw_section: bool,
    immediate: u8,
) -> bool {
    let idle = idle_in(e, this, field);
    let mut slot = e.call(WORD_AT_14, &args![idle]).i32();
    if slot == 0x14 {
        slot = 1;
    } else if slot == 0x15 {
        slot = 4;
    }
    let idle = idle_in(e, this, field);
    if fn_00490e40(e, idle.cast()).addr() != 0 {
        let idle = idle_in(e, this, field);
        let sequence = fn_00490e40(e, idle.cast()).addr();
        if e.mem.u32(current_sequence_at(this, slot as u32)) == sequence {
            if immediate != 0 {
                e.mem.set_u32(current_sequence_at(this, slot as u32), 0);
            } else {
                let idle = idle_in(e, this, field);
                let sequence = fn_00490e40(e, idle.cast()).addr();
                let state = e.call(SEQUENCE_STATE, &args![sequence]).i32();
                if state == 2 || state == 5 {
                    let idle = idle_in(e, this, field);
                    let sequence = fn_00490e40(e, idle.cast()).addr();
                    let name = object_name(e, sequence);
                    e.call(LOG, &args![LOG_IDLE_FREE_JUST_STARTED, name]);
                    return false;
                }
                let target = if raw_section {
                    let idle = idle_in(e, this, field);
                    e.call(WORD_AT_14, &args![idle]).i32()
                } else {
                    slot
                };
                animation_blend_out(e, this, target, 0);
            }
        }
    }
    true
}

/// The second half of `SpecialIdleFree` for the idle in the field `field`:
/// unless `immediate`, parks it in the first free "free when inactive" place
/// (+0x12c, then +0x130) to wait for its sequence to stop; otherwise (or when
/// both places are taken) frees it with [`animation_anim_idle_free`].
fn special_idle_park_or_free(e: &mut Engine, this: Ptr<Animation>, field: u32, immediate: u8) {
    let source = idle_field(this, field);
    for place in [
        Animation::spAnimIdleFreeWhenInactiveA.off,
        Animation::spAnimIdleFreeWhenInactiveA.off + 4,
    ] {
        if immediate == 0 && idle_in(e, this, place).addr() == 0 {
            e.call(NI_POINTER_COPY, &args![idle_field(this, place), source]);
            e.call(NI_POINTER_SET, &args![source, 0u32]);
            return;
        }
    }
    animation_anim_idle_free(e, this, Ptr::new(source));
}

// Translated from 00498910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SpecialIdleFree` (Xbox PDB), `(free_queued, immediate)`: ends
/// the current idle (`spAnimIdle`): blends its section out, or with
/// `immediate` just forgets its sequence, and then parks it to wait for its
/// sequence to stop or frees it. Stops (frees nothing more) with a log
/// message when the idle's sequence just started. Then, when `free_queued` is
/// set and an idle is queued, does the same for the queued idle (blending out
/// with the slot instead of the raw section); otherwise the queued idle (or
/// none) becomes the current idle.
pub fn animation_special_idle_free(
    e: &mut Engine,
    this: Ptr<Animation>,
    free_queued: u8,
    immediate: u8,
) {
    let current = Animation::spAnimIdle.off;
    let queued = Animation::spAnimIdleQueued.off;
    if idle_in(e, this, current).addr() != 0 {
        if !special_idle_blend_out(e, this, current, true, immediate) {
            return;
        }
        special_idle_park_or_free(e, this, current, immediate);
    }
    if free_queued != 0 && idle_in(e, this, queued).addr() != 0 {
        if !special_idle_blend_out(e, this, queued, false, immediate) {
            return;
        }
        special_idle_park_or_free(e, this, queued, immediate);
    } else {
        e.call(
            NI_POINTER_COPY,
            &args![idle_field(this, current), idle_field(this, queued)],
        );
        e.call(NI_POINTER_SET, &args![idle_field(this, queued), 0u32]);
    }
}

// Translated from 00498cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sequence of the current idle (`spAnimIdle`), or 0 without an idle; the
/// map has no name for it.
pub fn fn_00498cf0(e: &mut Engine, this: Ptr<Animation>) -> Ptr {
    let current = Animation::spAnimIdle.off;
    if idle_in(e, this, current).addr() == 0 {
        return Ptr::NULL;
    }
    let idle = idle_in(e, this, current);
    fn_00490e40(e, idle.cast())
}

// Translated from 00498d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SpecialIdleWorking` (Xbox PDB): whether an idle of the idle
/// form `form` is at work: the current or the queued idle with that form that
/// is not loaded (state other than 1), or one of the two parking places
/// holding an idle with that form, or (while the `VATS_OBJECT` word at +8 is
/// 4) holding any sequence.
pub fn animation_special_idle_working(e: &mut Engine, this: Ptr<Animation>, form: Ptr) -> bool {
    for offset in [Animation::spAnimIdle.off, Animation::spAnimIdleQueued.off] {
        if idle_in(e, this, offset).addr() != 0 {
            let idle = idle_in(e, this, offset);
            if e.call(IDLE_FORM_OF, &args![idle]).u32() == form.addr() {
                let idle = idle_in(e, this, offset);
                if idle_state(e, idle) != 1 {
                    return true;
                }
            }
        }
    }
    for offset in [
        Animation::spAnimIdleFreeWhenInactiveA.off,
        Animation::spAnimIdleFreeWhenInactiveA.off + 4,
    ] {
        if idle_in(e, this, offset).addr() != 0 {
            let idle = idle_in(e, this, offset);
            if e.call(IDLE_FORM_OF, &args![idle]).u32() == form.addr() {
                return true;
            }
            if e.call(WORD_AT_8, &args![VATS_OBJECT]).u32() == 4 {
                let idle = idle_in(e, this, offset);
                if fn_00490e40(e, idle.cast()).addr() != 0 {
                    return true;
                }
            }
        }
    }
    false
}

// Translated from 00498ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SpecialIdleWorking_ov2` (Xbox PDB): whether one of the four
/// idles (current, queued and the two parking places) holds `sequence`.
pub fn animation_special_idle_working_ov2(
    e: &mut Engine,
    this: Ptr<Animation>,
    sequence: Ptr,
) -> bool {
    for offset in [
        Animation::spAnimIdle.off,
        Animation::spAnimIdleQueued.off,
        Animation::spAnimIdleFreeWhenInactiveA.off,
        Animation::spAnimIdleFreeWhenInactiveA.off + 4,
    ] {
        if idle_in(e, this, offset).addr() != 0 {
            let idle = idle_in(e, this, offset);
            if fn_00490e40(e, idle.cast()) == sequence {
                return true;
            }
        }
    }
    false
}

// Translated from 00498f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SpecialIdleWorking_ov3` (Xbox PDB): whether the special idles
/// are busy: the current idle has no sequence yet and is not done (state 3),
/// or its sequence just started (state 2 or 5); or the queued idle has no
/// sequence, or its sequence just started.
pub fn animation_special_idle_working_ov3(e: &mut Engine, this: Ptr<Animation>) -> bool {
    let current = Animation::spAnimIdle.off;
    let queued = Animation::spAnimIdleQueued.off;
    if idle_in(e, this, current).addr() != 0 {
        let idle = idle_in(e, this, current);
        if fn_00490e40(e, idle.cast()).addr() == 0 {
            let idle = idle_in(e, this, current);
            return idle_state(e, idle) != 3;
        }
        let idle = idle_in(e, this, current);
        let sequence = fn_00490e40(e, idle.cast()).addr();
        let state = e.call(SEQUENCE_STATE, &args![sequence]).i32();
        if state == 2 || state == 5 {
            return true;
        }
    }
    if idle_in(e, this, queued).addr() != 0 {
        let idle = idle_in(e, this, queued);
        if fn_00490e40(e, idle.cast()).addr() == 0 {
            return true;
        }
        let idle = idle_in(e, this, queued);
        let sequence = fn_00490e40(e, idle.cast()).addr();
        let state = e.call(SEQUENCE_STATE, &args![sequence]).i32();
        if state == 2 || state == 5 {
            return true;
        }
    }
    false
}

// ---- Clearing and reloading ------------------------------------------------------------

/// The controller manager of an animation (the `NiPointer` at +0xd8).
fn manager_of(e: &mut Engine, this: Ptr<Animation>) -> Ptr<NiControllerManager> {
    Ptr::new(ni_pointer_get(e, this.addr() + Animation::spManager.off))
}

// Translated from 00499080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::ClearBlendInterps` (Xbox PDB): with a controller manager,
/// clears the interpolators of every `BSAnimGroupSequence` it holds
/// (`004efa50`); then releases the models still queued in `kfModelList`
/// (twice each, as the game does) and empties the list.
pub fn animation_clear_blend_interps(e: &mut Engine, this: Ptr<Animation>) {
    if manager_of(e, this).addr() == 0 {
        return;
    }
    let manager = manager_of(e, this);
    let count = fn_00495d00(e, manager);
    for index in 0..count {
        let manager = manager_of(e, this);
        let sequence = ni_controller_manager_get_sequence_at(e, manager, index);
        let cast = e
            .call(DYNAMIC_CAST, &args![RTTI_ANIM_GROUP_SEQUENCE, sequence])
            .u32();
        if cast != 0 {
            e.call(SEQUENCE_CLEAR_INTERPOLATORS, &args![cast]);
        }
    }
    let list = this.addr() + Animation::kfModelList.off;
    while e.call(MODEL_QUEUE_NOT_EMPTY, &args![this]).bool() {
        let cell = e.call(RECORD_ADDRESS, &args![list]).u32();
        let model = e.mem.u32(cell);
        e.call(KF_MODEL_RELEASE, &args![model]);
        e.call(KF_MODEL_RELEASE, &args![model]);
        e.call(SIMPLE_LIST_POP_FRONT, &args![list]);
    }
}

// Translated from 00499160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::ClearControllersInterpolators` (Xbox PDB), `__cdecl(node)`:
/// for every controller of `node` (and recursively of its children), a
/// controller of the `RTTI_CONTROLLER_KIND` kind gets its virtual slot 0xc8
/// called with (0, 0), and otherwise one of the `RTTI_CONTROLLER_FIRST` kind
/// gets `00a30160`.
pub fn animation_clear_controllers_interpolators(e: &mut Engine, node: Ptr) {
    if node.is_null() {
        return;
    }
    let mut controller = e.call(OBJECT_CONTROLLERS, &args![node]).u32();
    while controller != 0 {
        let kind = e
            .call(DYNAMIC_CAST, &args![RTTI_CONTROLLER_KIND, controller])
            .u32();
        if kind != 0 {
            e.vcall(kind, SLOT_CONTROLLER_CLEAR, &args![0u32, 0u32]);
        } else {
            let first = e
                .call(DYNAMIC_CAST, &args![RTTI_CONTROLLER_FIRST, controller])
                .u32();
            if first != 0 {
                e.call(CONTROLLER_RESET, &args![first]);
            }
        }
        controller = e.call(CONTROLLER_NEXT, &args![controller]).u32();
    }
    let as_node = e.vcall(node.addr(), SLOT_NODE_AS_NODE, &args![]).u32();
    if as_node != 0 {
        let mut index = 0u32;
        while index < e.call(NODE_CHILD_COUNT, &args![as_node]).u32() {
            let child = e.call(NODE_CHILD_AT, &args![as_node, index]).ptr::<()>();
            animation_clear_controllers_interpolators(e, child);
            index += 1;
        }
    }
}

// Translated from 00499240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::ReloadTargets` (Xbox PDB): with a controller manager, resets
/// the manager's palette object (when it casts to `RTTI_PALETTE_KIND`),
/// removes the two controllers (`RTTI_CONTROLLER_FIRST` and `_SECOND`) from
/// the animation root and deactivates every sequence. Then finds the largest
/// object count among the manager's sequences and re-aims each sequence: with
/// `retarget` set, a sequence is given the root and that count (`004eedd0`)
/// when the root has no first controller yet, and unless its name starts with
/// `"__"` it is attached to the root (`00a32c70`); without it the sequence is
/// prepared (`004eee00`) and re-aimed. Finally activates the sequence of
/// every slot that has one.
pub fn animation_reload_targets(e: &mut Engine, this: Ptr<Animation>, retarget: u8) {
    let a = this.addr();
    if manager_of(e, this).addr() == 0 {
        return;
    }
    let manager = manager_of(e, this);
    let palette = e.call(MANAGER_PALETTE, &args![manager]).u32();
    if palette != 0 {
        let cast = e
            .call(DYNAMIC_CAST, &args![RTTI_PALETTE_KIND, palette])
            .u32();
        if cast != 0 {
            e.call(PALETTE_OBJECT_RESET, &args![cast]);
        }
    }
    let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
    let first = e
        .call(GET_CONTROLLER, &args![root, RTTI_CONTROLLER_FIRST])
        .u32();
    let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
    let second = e
        .call(GET_CONTROLLER, &args![root, RTTI_CONTROLLER_SECOND])
        .u32();
    if first != 0 {
        let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
        e.call(REMOVE_CONTROLLER, &args![root, first]);
    }
    if second != 0 {
        let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
        e.call(REMOVE_CONTROLLER, &args![root, second]);
    }
    let manager = manager_of(e, this);
    ni_controller_manager_deactivate_all(e, manager, 0.0);
    let mut largest = 0u32;
    let mut index = 0u32;
    loop {
        let manager = manager_of(e, this);
        if index >= fn_00495d00(e, manager) {
            break;
        }
        let manager = manager_of(e, this);
        let sequence = ni_controller_manager_get_sequence_at(e, manager, index);
        if !sequence.is_null() && e.call(WORD_AT_C, &args![sequence]).u32() > largest {
            largest = e.call(WORD_AT_C, &args![sequence]).u32();
        }
        index += 1;
    }
    index = 0;
    loop {
        let manager = manager_of(e, this);
        if index >= fn_00495d00(e, manager) {
            break;
        }
        let manager = manager_of(e, this);
        let sequence = ni_controller_manager_get_sequence_at(e, manager, index).addr();
        if sequence != 0 {
            if retarget != 0 {
                let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
                let controller = e
                    .call(GET_CONTROLLER, &args![root, RTTI_CONTROLLER_FIRST])
                    .u32();
                if controller == 0 && largest != 0 {
                    let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
                    e.call(SEQUENCE_RETARGET, &args![sequence, root, largest]);
                }
                let name = object_name(e, sequence);
                if e.mem.u16(name) != 0x5f5f {
                    let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
                    e.call(SEQUENCE_ATTACH, &args![sequence, root]);
                }
            } else {
                e.call(SEQUENCE_PREPARE, &args![sequence]);
                let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
                e.call(SEQUENCE_RETARGET, &args![sequence, root, largest]);
            }
        }
        index += 1;
    }
    for slot in 0..8u32 {
        let sequence = e.mem.u32(current_sequence_at(this, slot));
        if sequence != 0 {
            let manager = manager_of(e, this);
            e.call(
                MANAGER_ACTIVATE,
                &args![manager, sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
            );
        }
    }
}

// Translated from 004994f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::BlendOut` (Xbox PDB): blends the group in `slot` out (0x14 is
/// slot 1, 0x15 slot 4). Slot 1 resets `movementDelta` to the zero vector. The
/// blend time is 0 without a sequence in the slot (or while `cSkipNextBlend`
/// is set); otherwise the blend-time setting, or the group's byte
/// (`fn_00495520`, in 1/30 s) when it is not 0, divided by the movement-scale
/// setting. For an actor holding a drawn weapon (slot 4 or section 0x14, and
/// not while the data handler's flag byte is set) the blend may be skipped
/// (`do_clear` false) for slot 4: it clears slots 5 and 6 itself, with the
/// blend time raised to 0.5 when the actor's anim action is 9 or 0x11, it is
/// not the player and the time is below 0.5, depending on the group's move
/// type and `fn_004997b0`. Then clears the slot (`ClearGroup(slot, time)`) and
/// tells the actor to start the anim action of the sequence its process holds
/// (type from the held sequence's group, 3 less for an iron sights action) when
/// `flag` is set, the anim action is 2 and the process holds one, or action
/// 0x11 otherwise.
pub fn animation_blend_out(e: &mut Engine, this: Ptr<Animation>, slot: i32, flag: u8) {
    let a = this.addr();
    let mapped = match slot {
        0x14 => 1,
        0x15 => 4,
        other => other,
    };
    let actor = e.get(this, Animation::pActorRef).addr();
    if mapped == 1 {
        for i in 0..3u32 {
            let word = e.mem.u32(ZERO_VECTOR + 4 * i);
            e.mem
                .set_u32(a + Animation::movementDelta.off + 4 * i, word);
        }
    }
    let mut time = 0.0f32;
    let sequence = e.mem.u32(current_sequence_at(this, mapped as u32));
    if sequence != 0 && e.get(this, Animation::cSkipNextBlend) as i8 == 0 {
        time = float_setting(e, SETTING_BLEND_TIME);
        let group = fn_0048f7f0(e, Ptr::new(sequence));
        let speed = fn_00495520(e, group) as i8;
        if speed != 0 {
            time = (speed as f64 / double_constant(e, DOUBLE_THIRTY)) as f32;
        }
        let scale = float_setting(e, SETTING_MOVEMENT_SCALE);
        time = (time as f64 / scale as f64) as f32;
    }
    let mut clear_slot = true;
    let mut start_held = false;
    let mut start_plain = false;
    let drawn = e.get(this, Animation::pActorRef).addr() != 0
        && (mapped == 4 || slot == 0x14)
        && e.call(ACTOR_IS_WEAPON_DRAWN, &args![actor]).bool()
        && {
            let handler: u32 = e.global(DATA_HANDLER);
            !e.call(DATA_HANDLER_FLAG, &args![handler]).bool()
        };
    if drawn {
        let mut held = false;
        if flag != 0 && e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32() == 2 {
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            if e.vcall(process, SLOT_PROCESS_HELD_OBJECT, &args![]).u32() != 0 {
                held = true;
            }
        }
        if held {
            start_held = true;
        } else {
            start_plain = true;
        }
        if mapped == 4 {
            clear_slot = false;
            let player: u32 = e.global(PLAYER_SINGLETON);
            if (e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32() == 9
                || e.call(ACTOR_GET_ANIM_ACTION, &args![actor]).i32() == 0x11)
                && actor != player
                && (time as f64) < double_constant(e, DOUBLE_HALF)
            {
                let group = fn_0048f7f0(e, Ptr::new(sequence));
                let move_type = e.call(GROUP_MOVE_TYPE, &args![group]).i32();
                let weapon_flag = fn_004997b0(e, Ptr::new(actor));
                if move_type == 1 {
                    if !weapon_flag {
                        time = e.global::<f32>(FLOAT_HALF);
                    }
                } else if weapon_flag {
                    time = e.global::<f32>(FLOAT_HALF);
                }
            }
            animation_clear_group(e, this, 5, time);
            animation_clear_group(e, this, 6, time);
        }
    }
    if clear_slot {
        animation_clear_group(e, this, slot, time);
    }
    if start_held {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        let held = e.vcall(process, SLOT_PROCESS_HELD_OBJECT, &args![]).u32();
        let group = fn_0048f7f0(e, Ptr::new(held));
        let mut group_type = e.call(ANIM_GROUP_SEQUENCE_TYPE, &args![group]).u16() as u32;
        if e.call(GROUP_TYPE_IS_IRON_SIGHTS, &args![group_type]).bool() {
            group_type = group_type.wrapping_sub(3);
        }
        e.call(ACTOR_START_ANIMATION, &args![actor, group_type, this]);
    } else if start_plain {
        e.call(ACTOR_START_ANIMATION, &args![actor, 0x11u32, this]);
    }
}

// Translated from 004997b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when the actor's flag bits (`008846e0`) have 0x400 set and 0x800
/// clear; the map has no name for it.
pub fn fn_004997b0(e: &mut Engine, actor: Ptr) -> bool {
    if e.call(ACTOR_FLAGS_WORD, &args![actor]).u32() & 0x400 != 0 {
        return e.call(ACTOR_FLAGS_WORD, &args![actor]).u32() & 0x800 == 0;
    }
    false
}

// ---- Saving and loading the animation ---------------------------------------------------

/// Whether group slot `index` of an animation holds a group (neither `0xff`
/// nor `0xffff`).
fn slot_in_use(e: &Engine, this: Ptr<Animation>, index: u32) -> bool {
    let group = e.mem.u16(group_at(this, index));
    group != 0xff && group != 0xffff
}

/// Whether `actor` (a `TESObjectREFR*`) answers its virtual slot 0x100 (the
/// game keeps it as the owner of the idles it saves, or null).
fn idle_owner(e: &mut Engine, actor: Ptr) -> Ptr {
    if !actor.is_null() && e.vcall(actor.addr(), SLOT_HAS_PROCESS, &args![]).bool() {
        actor
    } else {
        Ptr::NULL
    }
}

/// The sequence the save and load functions look for on the owner: when the
/// owner's sub-object (+0x88, virtual slot 0x34) answers, takes the
/// controller manager's palette, asks it (virtual slot 0x8c) for the object of
/// `fn_00499b70`, walks to the first child of that node, casts its controller
/// to `RTTI_SAVED_CONTROLLER` and looks the sequence up there with
/// `fn_00499b80`. Returns the controller object cast and the sequence, each 0
/// when not found (the sequence is 0 whenever the controller is).
fn saved_sequence(e: &mut Engine, this: Ptr<Animation>, owner: Ptr) -> (u32, u32) {
    let sub = owner.addr() + ACTOR_SUB_OBJECT;
    if e.vcall(sub, SLOT_SUB_OBJECT_CHECK, &args![]).u32() == 0 {
        return (0, 0);
    }
    let manager = fn_00496940(e, this);
    let palette = e.call(MANAGER_PALETTE, &args![manager]).u32();
    let key = fn_00499b70(e);
    let object = e.vcall(palette, 0x8c, &args![key]).u32();
    if object == 0 {
        return (0, 0);
    }
    let node = e.vcall(object, SLOT_NODE_AS_NODE, &args![]).u32();
    if node == 0 {
        return (0, 0);
    }
    let child = e.call(NODE_ARRAY_ELEMENT, &args![node, 0u32]).u32();
    if child == 0 {
        return (0, 0);
    }
    let controller = e.call(OBJECT_CONTROLLERS, &args![child]).u32();
    let cast = e
        .call(DYNAMIC_CAST, &args![RTTI_SAVED_CONTROLLER, controller])
        .u32();
    if cast == 0 {
        return (0, 0);
    }
    let lookup = fn_00499b80(e);
    let sequence = e.call(SEQUENCE_LOOKUP, &args![cast, lookup]).u32();
    (cast, sequence)
}

// Translated from 004997f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation`'s save size for `actor` (the map has no name for it): 26 bytes
/// (32 while the game saves in blocks), 17 more for each used group slot plus
/// the save size of its sequence (`004efb10`), the save size of the current
/// idle ([`fn_004974c0`], the queued one when there is one), 1 (2 from save
/// version 0x40), and for an actor whose sub-object answers, the save size of
/// the sequence found by `saved_sequence` plus 4. Logs the size with `Error`
/// (source line 0x15b8) while the diagnostics setting is on.
pub fn fn_004997f0(e: &mut Engine, this: Ptr<Animation>, actor: Ptr) -> u16 {
    let mut size: u16 = 0;
    let save = e.global::<u32>(SAVE_GAME_OBJECT);
    if e.call(SAVE_USES_BLOCKS, &args![save]).bool() {
        size = size.wrapping_add(4).wrapping_add(2);
    }
    for step in [4u16, 4, 0xc, 4, 1, 1] {
        size = size.wrapping_add(step);
    }
    for index in 0..8u32 {
        if slot_in_use(e, this, index) {
            for step in [2u16, 4, 4, 2, 4, 1] {
                size = size.wrapping_add(step);
            }
            let sequence = e.mem.u32(current_sequence_at(this, index));
            if sequence != 0 {
                let own = e.call(SEQUENCE_SAVE_SIZE, &args![sequence]).u16();
                size = size.wrapping_add(own);
            }
        }
    }
    let owner = idle_owner(e, actor);
    let idle = if ni_pointer_get(e, idle_field(this, Animation::spAnimIdleQueued.off)) != 0 {
        ni_pointer_get(e, idle_field(this, Animation::spAnimIdleQueued.off))
    } else {
        ni_pointer_get(e, idle_field(this, Animation::spAnimIdle.off))
    };
    let own = fn_004974c0(e, owner.addr(), Ptr::new(idle));
    size = size.wrapping_add(own);
    let save = e.global::<u32>(SAVE_GAME_OBJECT);
    size = size.wrapping_add(1);
    if e.call(SAVE_BYTE_80, &args![save]).u8() >= 0x40 {
        size = size.wrapping_add(1);
    }
    let (_, sequence) = saved_sequence(e, this, owner);
    if sequence != 0 {
        let own = e.call(SEQUENCE_SAVE_SIZE, &args![sequence]).u16();
        size = size.wrapping_add(own).wrapping_add(4);
    }
    let diagnostics = e
        .call(SETTING_BYTE_ADDRESS, &args![SAVE_DIAGNOSTICS_OBJECT])
        .u32();
    if e.mem.u8(diagnostics) != 0 {
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        let record = e.call(SAVE_RECORD_WRITTEN, &args![save]).u32();
        let written = size as u32;
        if record != 0 {
            let form_id = e.mem.u32(record);
            let kind = e.call(FORM_BY_ID, &args![form_id]).u32();
            let name = e.vcall(kind, SLOT_FORM_TYPE_NAME, &args![]).u32();
            let flags = e.mem.u32(record + 5);
            e.call(
                ERROR_LOG,
                &args![
                    LOG_SAVE_SIZE_FORM,
                    written,
                    form_id,
                    name,
                    flags,
                    0x15b8u32,
                    SOURCE_FILE
                ],
            );
        } else {
            e.call(
                ERROR_LOG,
                &args![LOG_SAVE_SIZE, written, 0x15b8u32, SOURCE_FILE],
            );
        }
    }
    size
}

// Translated from 00499b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The global at `011c61bc` (the key `saved_sequence` asks the palette for);
/// the map has no name for it.
pub fn fn_00499b70(e: &mut Engine) -> u32 {
    e.global(SAVED_SEQUENCE_KEY_GLOBAL)
}

// Translated from 00499b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The global at `011c61c0` (the key `saved_sequence` looks the sequence up
/// with); the map has no name for it.
pub fn fn_00499b80(e: &mut Engine) -> u32 {
    e.global(SAVED_SEQUENCE_LOOKUP_GLOBAL)
}

/// Logs how many bytes a save or load function wrote since `start`, when the
/// diagnostics setting is on: with the record being written (its form ID,
/// the form type's name and flags) when there is one. `line` is the source
/// line the game names, `form_format` and `plain_format` the two formats.
fn log_written_since(e: &mut Engine, start: u32, line: u32, form_format: u32, plain_format: u32) {
    let diagnostics = e
        .call(SETTING_BYTE_ADDRESS, &args![SAVE_DIAGNOSTICS_OBJECT])
        .u32();
    if e.mem.u8(diagnostics) == 0 {
        return;
    }
    let save = e.global::<u32>(SAVE_GAME_OBJECT);
    let end = e.call(SAVE_POSITION, &args![save]).u32();
    let save = e.global::<u32>(SAVE_GAME_OBJECT);
    let record = e.call(SAVE_RECORD_WRITTEN, &args![save]).u32();
    let written = end.wrapping_sub(start);
    if record != 0 {
        let form_id = e.mem.u32(record);
        let kind = e.call(FORM_BY_ID, &args![form_id]).u32();
        let name = e.vcall(kind, SLOT_FORM_TYPE_NAME, &args![]).u32();
        let flags = e.mem.u32(record + 5);
        e.call(
            ERROR_LOG,
            &args![
                form_format,
                written,
                form_id,
                name,
                flags,
                line,
                SOURCE_FILE
            ],
        );
    } else {
        e.call(ERROR_LOG, &args![plain_format, written, line, SOURCE_FILE]);
    }
}

/// At the end of a save function: when the game saves in blocks, patches the
/// 16-bit length at `block_start` (a block longer than 0xffff is logged) with
/// the bytes written since.
fn finish_save_block(e: &mut Engine, block_start: u32, line: u32) {
    let save = e.global::<u32>(SAVE_GAME_OBJECT);
    if e.call(SAVE_USES_BLOCKS, &args![save]).bool() {
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        let end = e.call(SAVE_POSITION, &args![save]).u32();
        if end > block_start.wrapping_add(0xffff) {
            e.call(LOG, &args![LOG_SAVE_BLOCK_TOO_BIG, SOURCE_FILE, line]);
        }
        e.mem
            .set_u16(block_start, end.wrapping_sub(block_start) as u16);
    }
}

// Translated from 00499b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SaveGame` (Xbox PDB): writes the animation to the save game
/// for `actor`. Inside a save block when blocks are in use (the tag `"KOLB"`
/// and a 16-bit length patched in at the end), then the speeds (+0x10c,
/// +0x110), `movementDelta` (12 bytes), `m_fLooking`, `cSkipUpdate`, a mask
/// byte of the used group slots and, for each used slot, its group, action,
/// loop count, next group and next loops, a selector byte (the sequence's index
/// in its map entry, 0xff without an entry, 0xfe with a null sequence, which is
/// logged) and the sequence ([`SEQUENCE_SAVE`]). Then the queued idle (or the
/// current one, [`fn_004975e0`]), `cSkipNextBlend`, and for an actor whose
/// sub-object answers a flag byte (from save version 0x40), the controller
/// sequence found by `saved_sequence` and a `float` of the actor's sub-object.
/// Logs the size written (source line 0x1628) while the diagnostics setting is
/// on.
pub fn animation_save_game(e: &mut Engine, this: Ptr<Animation>, actor: Ptr) {
    let a = this.addr();
    let save = e.global::<u32>(SAVE_GAME_OBJECT);
    let mut start = e.call(SAVE_POSITION, &args![save]).u32();
    let diagnostics = e
        .call(SETTING_BYTE_ADDRESS, &args![SAVE_DIAGNOSTICS_OBJECT])
        .u32();
    if e.mem.u8(diagnostics) != 0 {
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        start = e.call(SAVE_POSITION, &args![save]).u32();
    }
    let mut block_start = 0u32;
    e.with_stack(0x10, |e, cells| {
        let tag = cells.addr();
        let length = cells.addr() + 4;
        let mask = cells.addr() + 8;
        let selector = cells.addr() + 9;
        let flag = cells.addr() + 10;
        let value = cells.addr() + 12;
        e.mem.set_u16(length, 0);
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        if e.call(SAVE_USES_BLOCKS, &args![save]).bool() {
            e.mem.set_u32(tag, SAVE_BLOCK_TAG);
            e.call(SAVE_WRITE, &args![save, tag, 4u32]);
            let save = e.global::<u32>(SAVE_GAME_OBJECT);
            block_start = e.call(SAVE_POSITION, &args![save]).u32();
            e.call(SAVE_WRITE, &args![save, length, 2u32]);
        }
        for (offset, size) in [
            (0x10cu32, 4u32),
            (0x110, 4),
            (0x10, 0xc),
            (0x48, 4),
            (0xcc, 1),
        ] {
            let save = e.global::<u32>(SAVE_GAME_OBJECT);
            e.call(SAVE_WRITE, &args![save, a + offset, size]);
        }
        e.mem.set_u8(mask, 0);
        for index in 0..8u32 {
            if slot_in_use(e, this, index) {
                let bits = e.mem.u8(mask) | (1u8 << index);
                e.mem.set_u8(mask, bits);
            }
        }
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        e.call(SAVE_WRITE, &args![save, mask, 1u32]);
        for index in 0..8u32 {
            if !slot_in_use(e, this, index) {
                continue;
            }
            for (address, size) in [
                (group_at(this, index), 2u32),
                (action_at(this, index), 4),
                (loop_count_at(this, index), 4),
                (next_group_at(this, index), 2),
                (next_loops_at(this, index), 4),
            ] {
                let save = e.global::<u32>(SAVE_GAME_OBJECT);
                e.call(SAVE_WRITE, &args![save, address, size]);
            }
            e.mem.set_u8(selector, 0xff);
            let key = e.mem.u16(group_at(this, index));
            let found = sequence_map_get(e, this, key);
            if let Some(entry) = found {
                let sequence = e.mem.u32(current_sequence_at(this, index));
                let chosen = e.vcall(entry, 0x18, &args![sequence]).u8();
                e.mem.set_u8(selector, chosen);
            }
            if e.mem.u32(current_sequence_at(this, index)) == 0 {
                let id = e.call(WORD_AT_C, &args![actor]).u32();
                let name = e.vcall(actor.addr(), SLOT_FORM_TYPE_NAME, &args![]).u32();
                let group = e.mem.u16(group_at(this, index)) as u32;
                let action = e.mem.u32(action_at(this, index));
                e.call(
                    LOG,
                    &args![LOG_SAVE_NULL_SEQUENCE, name, id, index, group, action],
                );
                e.mem.set_u8(selector, 0xfe);
            }
            let save = e.global::<u32>(SAVE_GAME_OBJECT);
            e.call(SAVE_WRITE, &args![save, selector, 1u32]);
            let sequence = e.mem.u32(current_sequence_at(this, index));
            if sequence != 0 {
                let time = e.get(this, Animation::time);
                e.call(SEQUENCE_SAVE, &args![sequence, time]);
            }
        }
        let owner = idle_owner(e, actor);
        let queued = ni_pointer_get(e, idle_field(this, Animation::spAnimIdleQueued.off));
        let idle = if queued != 0 {
            queued
        } else {
            ni_pointer_get(e, idle_field(this, Animation::spAnimIdle.off))
        };
        let time = e.get(this, Animation::time);
        fn_004975e0(e, owner.addr(), Ptr::new(idle), this, time);
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        e.call(
            SAVE_WRITE,
            &args![save, a + Animation::cSkipNextBlend.off, 1u32],
        );
        e.mem.set_u8(flag, 0);
        let (_, sequence) = saved_sequence(e, this, owner);
        if sequence != 0 {
            e.mem.set_u8(flag, 1);
            let save = e.global::<u32>(SAVE_GAME_OBJECT);
            if e.call(SAVE_BYTE_80, &args![save]).u8() >= 0x40 {
                e.call(SAVE_WRITE, &args![save, flag, 1u32]);
            }
            let time = e.get(this, Animation::time);
            e.call(SEQUENCE_SAVE, &args![sequence, time]);
            let sub = owner.addr() + ACTOR_SUB_OBJECT;
            let object = e.call(WORD_AT_4, &args![sub]).u32();
            e.mem.set_f32(value, 0.0);
            if object != 0 {
                let amount = e.call(FLOAT_AT_10, &args![object]).f32();
                e.mem.set_f32(value, amount);
            }
            let save = e.global::<u32>(SAVE_GAME_OBJECT);
            e.call(SAVE_WRITE, &args![save, value, 4u32]);
        }
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        if e.call(SAVE_BYTE_80, &args![save]).u8() >= 0x40 && e.mem.u8(flag) == 0 {
            e.call(SAVE_WRITE, &args![save, flag, 1u32]);
        }
    });
    log_written_since(e, start, 0x1628, LOG_SAVE_GAME_FORM, LOG_SAVE_GAME);
    finish_save_block(e, block_start, 0x1628);
}

// Translated from 0049a1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the animation back from the save game for `actor` (the mirror of
/// [`animation_save_game`]; the map has no name for it). Checks the block tag
/// and reads the block length when blocks are in use, reads the fields and the
/// groups (each used slot is started again with [`animation_force_section`],
/// with `cSkipNextBlend` set; a slot that does not come back is cleared and
/// the bytes of its sequence skipped), then the current idle and, below save
/// version 0x4c, the queued one ([`fn_004977e0`]). Clears the last slot's
/// sequence when the current idle is loaded, reads `cSkipNextBlend` and, from
/// the flag byte on, the controller sequence of `saved_sequence`: it is
/// activated, loaded and a light fade set from it; without it the
/// bytes are skipped. Then runs `UpdateBipOnly` and logs a block that was not
/// read exactly (source lines 0x1630 and 0x16ac).
pub fn fn_0049a1c0(e: &mut Engine, this: Ptr<Animation>, actor: Ptr) {
    let a = this.addr();
    let save = e.global::<u32>(SAVE_GAME_OBJECT);
    let mut block_start = 0u32;
    let mut block_length = 0u16;
    e.with_stack(0x10, |e, cells| {
        let tag = cells.addr();
        let length = cells.addr() + 4;
        let mask = cells.addr() + 6;
        let selector = cells.addr() + 7;
        let flag = cells.addr() + 8;
        let value = cells.addr() + 12;
        e.mem.set_u16(length, 0);
        if e.call(SAVE_USES_BLOCKS, &args![save]).bool() {
            e.call(SAVE_READ, &args![save, tag, 4u32]);
            if e.mem.u32(tag) != SAVE_BLOCK_TAG {
                let record = e.call(SAVE_RECORD_READ, &args![save]).u32();
                if record != 0 {
                    let record_id = e.mem.u32(record);
                    let kind = e.call(FORM_BY_ID, &args![record_id]).u32();
                    let name = e.vcall(kind, SLOT_FORM_TYPE_NAME, &args![]).u32();
                    let flags = e.mem.u32(record + 5);
                    let extra = e.mem.u8(record + 9) as u32;
                    e.call(
                        LOG,
                        &args![
                            LOG_LOAD_NO_BLOCK_FORM,
                            SOURCE_FILE,
                            0x1630u32,
                            record_id,
                            name,
                            extra,
                            flags
                        ],
                    );
                } else {
                    let version = e.call(SAVE_BYTE_80, &args![save]).u8() as u32;
                    e.call(
                        LOG,
                        &args![LOG_LOAD_NO_BLOCK, SOURCE_FILE, 0x1630u32, version],
                    );
                }
            }
            block_start = e.call(SAVE_POSITION, &args![save]).u32();
            e.call(SAVE_READ, &args![save, length, 2u32]);
        }
        let owner = idle_owner(e, actor);
        for (offset, size) in [
            (0x10cu32, 4u32),
            (0x110, 4),
            (0x10, 0xc),
            (0x48, 4),
            (0xcc, 1),
        ] {
            e.call(SAVE_READ, &args![save, a + offset, size]);
        }
        e.call(SAVE_READ, &args![save, mask, 1u32]);
        for index in 0..8u32 {
            if (e.mem.u8(mask) as i8 as i32) & (1 << index) == 0 {
                continue;
            }
            for (address, size) in [
                (group_at(this, index), 2u32),
                (action_at(this, index), 4),
                (loop_count_at(this, index), 4),
                (next_group_at(this, index), 2),
                (next_loops_at(this, index), 4),
            ] {
                e.call(SAVE_READ, &args![save, address, size]);
            }
            e.call(SAVE_READ, &args![save, selector, 1u32]);
            e.mem.set_u32(current_sequence_at(this, index), 0);
            if e.mem.u8(selector) as i8 != -2 {
                fn_004974a0(e, this);
                let group = e.mem.u16(group_at(this, index));
                let action = e.mem.i32(action_at(this, index));
                let time = e.get(this, Animation::time);
                let chosen = e.mem.u8(selector);
                animation_force_section(e, this, index as i32, group, action, time, chosen);
                let sequence = e.mem.u32(current_sequence_at(this, index));
                if sequence == 0 {
                    let skip = e.call(SEQUENCE_SAVE_BASE_SIZE, &args![]).u16() as u32;
                    e.call(SAVE_SKIP, &args![save, skip]);
                    animation_clear_group(e, this, index as i32, 0.0);
                } else {
                    let time = e.get(this, Animation::time);
                    e.call(SEQUENCE_LOAD, &args![sequence, time]);
                }
            }
        }
        let time = e.get(this, Animation::time);
        let idle = fn_004977e0(e, owner, this, time);
        e.call(
            NI_POINTER_SET,
            &args![idle_field(this, Animation::spAnimIdle.off), idle],
        );
        if e.call(SAVE_BYTE_80, &args![save]).u8() < 0x4c {
            let time = e.get(this, Animation::time);
            let idle = fn_004977e0(e, owner, this, time);
            e.call(
                NI_POINTER_SET,
                &args![idle_field(this, Animation::spAnimIdleQueued.off), idle],
            );
        }
        if ni_pointer_get(e, idle_field(this, Animation::spAnimIdle.off)) != 0 {
            let idle = idle_in(e, this, Animation::spAnimIdle.off);
            if idle_state(e, idle) == 1 {
                e.mem.set_u32(current_sequence_at(this, 7), 0);
            }
        }
        e.call(
            SAVE_READ,
            &args![save, a + Animation::cSkipNextBlend.off, 1u32],
        );
        e.mem.set_u8(flag, 1);
        if e.call(SAVE_BYTE_80, &args![save]).u8() >= 0x40 {
            e.call(SAVE_READ, &args![save, flag, 1u32]);
        }
        if e.mem.u8(flag) != 0 {
            let mut found = false;
            let (controller, sequence) = saved_sequence(e, this, owner);
            if sequence != 0 {
                ni_controller_manager_deactivate_all(e, Ptr::new(controller), 0.0);
                e.call(
                    MANAGER_ACTIVATE,
                    &args![controller, sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
                );
                e.call(MANAGER_SET_FLAG, &args![controller, 1u32]);
                let time = e.get(this, Animation::time);
                e.call(SEQUENCE_LOAD, &args![sequence, time]);
                e.call(SAVE_READ, &args![save, value, 4u32]);
                let sub = owner.addr() + ACTOR_SUB_OBJECT;
                let object = e.call(WORD_AT_4, &args![sub]).u32();
                if object != 0 {
                    let end = e.call(SEQUENCE_END_TIME, &args![sequence]).f64();
                    let fade = (end * double_constant(e, DOUBLE_THREE_QUARTERS)) as f32;
                    e.call(LIGHT_FADE, &args![object, 1u32, fade]);
                    let amount = e.mem.f32(value);
                    e.call(SET_FLOAT_AT_10, &args![object, amount]);
                }
                found = true;
            }
            if e.call(SAVE_BYTE_80, &args![save]).u8() >= 0x40 && !found {
                let skip = e.call(SEQUENCE_SAVE_BASE_SIZE, &args![]).u16() as u32;
                e.call(SAVE_SKIP, &args![save, skip + 4]);
            }
        }
        let time = e.get(this, Animation::time);
        animation_update_bip_only(e, this, time, Ptr::new(a + 0x1c), 1);
        block_length = e.mem.u16(length);
    });
    if e.call(SAVE_USES_BLOCKS, &args![save]).bool() {
        let end = e.call(SAVE_POSITION, &args![save]).u32();
        let record = e.call(SAVE_RECORD_READ, &args![save]).u32();
        let expected = (block_length as u32).wrapping_add(block_start);
        if record != 0 {
            let record_id = e.mem.u32(record);
            let kind = e.call(FORM_BY_ID, &args![record_id]).u32();
            let flags = e.mem.u32(record + 5);
            let extra = e.mem.u8(record + 9) as u32;
            if expected < end {
                let name = e.vcall(kind, SLOT_FORM_TYPE_NAME, &args![]).u32();
                e.call(
                    LOG,
                    &args![
                        LOG_LOAD_LONG_FORM,
                        end.wrapping_sub(expected),
                        SOURCE_FILE,
                        0x16acu32,
                        record_id,
                        name,
                        extra,
                        flags
                    ],
                );
            } else if end < expected {
                let name = e.vcall(kind, SLOT_FORM_TYPE_NAME, &args![]).u32();
                e.call(
                    LOG,
                    &args![
                        LOG_LOAD_SHORT_FORM,
                        expected.wrapping_sub(end),
                        SOURCE_FILE,
                        0x16acu32,
                        record_id,
                        name,
                        extra,
                        flags
                    ],
                );
            }
        } else if expected < end {
            let version = e.call(SAVE_BYTE_80, &args![save]).u8() as u32;
            e.call(
                LOG,
                &args![
                    LOG_LOAD_LONG,
                    end.wrapping_sub(expected),
                    SOURCE_FILE,
                    0x16acu32,
                    version
                ],
            );
        } else if end < expected {
            let version = e.call(SAVE_BYTE_80, &args![save]).u8() as u32;
            e.call(
                LOG,
                &args![
                    LOG_LOAD_SHORT,
                    expected.wrapping_sub(end),
                    SOURCE_FILE,
                    0x16acu32,
                    version
                ],
            );
        }
    }
}

/// Frees the special idles (current and queued) and empties the last slot, the
/// way `fn_0049a920` and `fn_0049bb70` begin: `ClearGroup(0x14, 0)`,
/// [`fn_00496280`], frees and empties both idle fields and clears slot 7.
fn clear_idles_and_slot_7(e: &mut Engine, this: Ptr<Animation>) {
    animation_clear_group(e, this, 0x14, 0.0);
    fn_00496280(e, this);
    for offset in [Animation::spAnimIdle.off, Animation::spAnimIdleQueued.off] {
        if idle_in(e, this, offset).addr() != 0 {
            animation_anim_idle_free(e, this, Ptr::new(idle_field(this, offset)));
        }
        e.call(NI_POINTER_SET, &args![idle_field(this, offset), 0u32]);
    }
    e.mem.set_u32(current_sequence_at(this, 7), 0);
    animation_clear_group(e, this, 7, 0.0);
    if manager_of(e, this).addr() != 0 {
        let manager = manager_of(e, this);
        ni_controller_manager_deactivate_all(e, manager, 0.0);
    }
}

// Translated from 0049a920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the animation (the map has no name for it): clears the groups and
/// idles ([`clear_idles_and_slot_7`] steps), then clears the controller
/// interpolators of the animation root (`pAnimRoot`, read through
/// `005585e0`), the blend interpolators and reloads the targets with the flag
/// set. The stack word the callers push is never read (`RET 0x4` at
/// `0049aa16`).
pub fn fn_0049a920(e: &mut Engine, this: Ptr<Animation>, _unused_argument: u32) {
    clear_idles_and_slot_7(e, this);
    let root = e.call(KF_MODEL_ANIM_GROUP, &args![this]).ptr::<()>();
    animation_clear_controllers_interpolators(e, root);
    animation_clear_blend_interps(e, this);
    animation_reload_targets(e, this, 1);
}

// Translated from 0049aa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of bytes `SaveAnimation` writes for `actor` and `animation`
/// (`__cdecl`; the map has no name for it): 2, plus [`fn_004997f0`] when there
/// is an animation and the actor's virtual slot 0x22c (argument 0) says no.
pub fn fn_0049aa20(e: &mut Engine, actor: Ptr, animation: Ptr<Animation>) -> u16 {
    let mut size: u16 = 2;
    if !animation.is_null()
        && !e
            .vcall(actor.addr(), SLOT_ACTOR_FLAG_22C, &args![0u32])
            .bool()
    {
        let own = fn_004997f0(e, animation, actor);
        size = size.wrapping_add(own);
    }
    size
}

// Translated from 0049aa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SaveAnimation` (Xbox PDB), `__cdecl(actor, animation)`: writes
/// the 16-bit save size ([`fn_004997f0`], 0 without an animation or when the
/// actor's virtual slot 0x22c says yes) and then the animation
/// ([`animation_save_game`]).
pub fn animation_save_animation(e: &mut Engine, actor: Ptr, animation: Ptr<Animation>) {
    let mut size: u16 = 0;
    if !animation.is_null()
        && !e
            .vcall(actor.addr(), SLOT_ACTOR_FLAG_22C, &args![0u32])
            .bool()
    {
        size = fn_004997f0(e, animation, actor);
    }
    e.with_stack(4, |e, cell| {
        e.mem.set_u16(cell.addr(), size);
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        e.call(SAVE_WRITE, &args![save, cell, 2u32]);
    });
    if !animation.is_null()
        && !e
            .vcall(actor.addr(), SLOT_ACTOR_FLAG_22C, &args![0u32])
            .bool()
    {
        animation_save_game(e, animation, actor);
    }
}

// Translated from 0049ab00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads a 16-bit length from the save game (`__cdecl`) and, when it is not 0,
/// has the save game object consume that many bytes into `destination`
/// (`0085f010`); the map has no name for it. The exe's callers push two words
/// (the object, then a word from it); the body reads only the first
/// (`[EBP+8]`) and returns with a plain `RET`, so `_unused` is the second.
pub fn fn_0049ab00(e: &mut Engine, destination: Ptr, _unused: u32) {
    e.with_stack(4, |e, cell| {
        let save = e.global::<u32>(SAVE_GAME_OBJECT);
        e.call(SAVE_READ, &args![save, cell, 2u32]);
        let length = e.mem.u16(cell.addr());
        if length != 0 {
            let save = e.global::<u32>(SAVE_GAME_OBJECT);
            e.call(SAVE_GAME_CONSUME, &args![save, destination, length as u32]);
        }
    });
}

// Translated from 0049ab40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the animation into a `BGSSaveGameBuffer` (the map has no name for
/// it): the speeds, `movementDelta`, `m_fLooking`, `cSkipUpdate`, `time`,
/// `sQueuedReloadGroup`, the replay delay list as a variable-sized value (a
/// form ID and a 4-byte delay each), the queued idle's (or the current one's)
/// form ID and, for an idle form, its state as a variable-sized value
/// ([`fn_00497bc0`]), a mask of the group slots to save (all used slots except
/// the one that holds the idle's sequence when an idle is queued), and for
/// each of those slots its group, action, next group, loop count, next loops,
/// the selector byte (as in [`animation_save_game`], 0xfe for a null
/// sequence, without the log) and the sequence (`004efc90`). Ends with
/// `cSkipNextBlend`, the 16-bit value of the actor's process (virtual slot
/// 0x3e4) and the slot numbers of the sequence its process holds and of the
/// idle's sequence (0xff for none).
pub fn fn_0049ab40(e: &mut Engine, this: Ptr<Animation>, buffer: Ptr) {
    let a = this.addr();
    let owner = e
        .vcall(buffer.addr(), SLOT_SAVE_BUFFER_OWNER, &args![])
        .u32();
    for (offset, size) in [
        (0x10cu32, 4u32),
        (0x110, 4),
        (0x10, 0xc),
        (0x48, 4),
        (0xcc, 1),
        (0xd4, 4),
        (0x122, 2),
    ] {
        e.call(SAVE_BUFFER_FIELD, &args![buffer, a + offset, size, 0u32]);
    }
    let mut count = 0u32;
    let start = e.call(SAVE_BUFFER_START_VARIABLE, &args![buffer]).u32();
    let mut node = a + Animation::replayDelayList.off;
    while node != 0 {
        let cell = e.call(RECORD_ADDRESS, &args![node]).u32();
        let item = e.mem.u32(cell);
        if item != 0 {
            let form = e.mem.u32(item);
            e.call(SAVE_BUFFER_FORM_ID, &args![buffer, form, 0u32]);
            e.call(SAVE_BUFFER_FIELD, &args![buffer, item + 4, 4u32, 0u32]);
            count += 1;
        }
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
    e.call(SAVE_BUFFER_END_VARIABLE, &args![buffer, count, start]);
    let queued = idle_field(this, Animation::spAnimIdleQueued.off);
    let idle_ref = if ni_pointer_get(e, queued) != 0 {
        queued
    } else {
        idle_field(this, Animation::spAnimIdle.off)
    };
    let idle = ni_pointer_get(e, idle_ref);
    let form = if idle != 0 {
        e.call(IDLE_FORM_OF, &args![idle]).u32()
    } else {
        0
    };
    e.call(SAVE_BUFFER_FORM_ID, &args![buffer, form, 0u32]);
    if idle != 0 && e.call(IDLE_FORM_OF, &args![idle]).u32() != 0 {
        let start = e.call(SAVE_BUFFER_START_VARIABLE, &args![buffer]).u32();
        let before = e.call(WORD_AT_C, &args![buffer]).u32();
        fn_00497bc0(e, Ptr::new(idle), buffer, a);
        let after = e.call(WORD_AT_C, &args![buffer]).u32();
        e.call(
            SAVE_BUFFER_END_VARIABLE,
            &args![buffer, after.wrapping_sub(before), start],
        );
    }
    let mut idle_sequence = 0u32;
    if ni_pointer_get(e, queued) != 0
        && ni_pointer_get(e, idle_field(this, Animation::spAnimIdle.off)) != 0
    {
        let current = ni_pointer_get(e, idle_field(this, Animation::spAnimIdle.off));
        idle_sequence = fn_00490e40(e, Ptr::new(current)).addr();
    }
    let included = |e: &Engine, index: u32| {
        !(idle_sequence != 0 && e.mem.u32(current_sequence_at(this, index)) == idle_sequence)
            && slot_in_use(e, this, index)
    };
    e.with_stack(0x10, |e, cells| {
        let mask = cells.addr();
        let selector = cells.addr() + 1;
        let held_slot = cells.addr() + 2;
        let idle_slot = cells.addr() + 3;
        let phase = cells.addr() + 4;
        e.mem.set_u8(mask, 0);
        for index in 0..8u32 {
            if included(e, index) {
                let bits = e.mem.u8(mask) | (1u8 << index);
                e.mem.set_u8(mask, bits);
            }
        }
        e.call(SAVE_BUFFER_FIELD, &args![buffer, mask, 1u32, 0u32]);
        e.mem.set_u8(held_slot, 0xff);
        let process = e.call(ACTOR_PROCESS, &args![owner]).u32();
        let word = e.vcall(process, SLOT_PROCESS_HELD_PHASE, &args![]).u16();
        e.mem.set_u16(phase, word);
        let process = e.call(ACTOR_PROCESS, &args![owner]).u32();
        let held = e.vcall(process, SLOT_PROCESS_HELD_OBJECT, &args![]).u32();
        e.mem.set_u8(idle_slot, 0xff);
        let own_sequence = if idle != 0 {
            fn_00490e40(e, Ptr::new(idle)).addr()
        } else {
            0
        };
        for index in 0..8u32 {
            if !included(e, index) {
                continue;
            }
            for (address, size) in [
                (group_at(this, index), 2u32),
                (action_at(this, index), 4),
                (next_group_at(this, index), 2),
                (loop_count_at(this, index), 4),
                (next_loops_at(this, index), 4),
            ] {
                e.call(SAVE_BUFFER_FIELD, &args![buffer, address, size, 0u32]);
            }
            e.mem.set_u8(selector, 0xff);
            let key = e.mem.u16(group_at(this, index));
            if let Some(entry) = sequence_map_get(e, this, key) {
                let sequence = e.mem.u32(current_sequence_at(this, index));
                let chosen = e.vcall(entry, 0x18, &args![sequence]).u8();
                e.mem.set_u8(selector, chosen);
            }
            let sequence = e.mem.u32(current_sequence_at(this, index));
            if sequence == 0 {
                e.mem.set_u8(selector, 0xfe);
            }
            e.call(SAVE_BUFFER_FIELD, &args![buffer, selector, 1u32, 0u32]);
            if sequence != 0 {
                let time = e.get(this, Animation::time);
                e.call(SEQUENCE_SAVE_TO_BUFFER, &args![sequence, buffer, time]);
            }
            if sequence == held {
                e.mem.set_u8(held_slot, index as u8);
            }
            if sequence == own_sequence {
                e.mem.set_u8(idle_slot, index as u8);
            }
        }
        e.call(
            SAVE_BUFFER_FIELD,
            &args![buffer, a + Animation::cSkipNextBlend.off, 1u32, 0u32],
        );
        e.call(SAVE_BUFFER_FIELD, &args![buffer, phase, 2u32, 0u32]);
        e.call(SAVE_BUFFER_FIELD, &args![buffer, held_slot, 1u32, 0u32]);
        e.call(SAVE_BUFFER_FIELD, &args![buffer, idle_slot, 1u32, 0u32]);
    });
}

// Translated from 0049b050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the animation back from a `BGSLoadGameBuffer` (the mirror of
/// [`fn_0049ab40`]; the map has no name for it). Without an animation root it
/// does nothing. Adds the queued models, then has the actor's process
/// (virtual slot 0x4dc) take the animation, plays the idle-start group
/// 0xe0 in section 0x14 for the actors whose virtual slots 0x22c or 0x2e8
/// say so, hands the player's or the actor's biped to the process (slot 0x1cc),
/// notifies it (slots 0x1d8 and 0x61c), and updates the root. Clears the
/// groups, reads the fields, the replay delays (each a form cast to `TESIdleForm`
/// and a 4-byte delay), the idle (a new idle in section 7 whose state is read by
/// [`fn_00497c10`]; unknown forms are skipped), then the used slots (a slot
/// that does not come back is loaded into a throw-away
/// `BSAnimGroupSequence` and cleared), `cSkipNextBlend`, the phase and the
/// slot numbers of the held sequence and of the idle's sequence. Finally updates
/// the bip nodes.
pub fn fn_0049b050(e: &mut Engine, this: Ptr<Animation>, buffer: Ptr) {
    let a = this.addr();
    if ni_pointer_get(e, a + Animation::pAnimRoot.off) == 0 {
        return;
    }
    let actor = e
        .vcall(buffer.addr(), SLOT_LOAD_BUFFER_OWNER, &args![])
        .u32();
    let list = a + Animation::kfModelList.off;
    while !e.call(SIMPLE_LIST_IS_EMPTY, &args![list]).bool() {
        let cell = e.call(RECORD_ADDRESS, &args![list]).u32();
        let model = e.mem.u32(cell);
        animation_add_animation(e, this, model, false);
        e.call(KF_MODEL_RELEASE, &args![model]);
        e.call(SIMPLE_LIST_POP_FRONT, &args![list]);
    }
    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
    e.vcall(process, SLOT_PROCESS_RELOAD, &args![actor, this]);
    if e.vcall(actor, SLOT_ACTOR_FLAG_22C, &args![0u32]).bool()
        || e.vcall(actor, SLOT_ACTOR_FLAG_2E8, &args![]).bool()
    {
        animation_force_section(e, this, 0x14, 0xe0, -1, 0.0, 0xff);
    }
    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
    let byte = e.vcall(process, SLOT_PROCESS_BYTE, &args![]).u8();
    fn_004974a0(e, this);
    let player: u32 = e.global(PLAYER_SINGLETON);
    if actor == player {
        if e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32() == a {
            let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
            let biped = e.call(PLAYER_BIPED, &args![player, 1u32]).u32();
            // 0049b1e4: `this` and the player pushed before `00950b00` (RET 4) are
            // the last two words of slot 0x1cc (RET 0x10).
            e.vcall(
                process,
                SLOT_PROCESS_SET_BIPED,
                &args![byte, biped, this, player],
            );
        } else {
            let process = e.call(ACTOR_PROCESS, &args![player]).u32();
            let biped = e.call(PLAYER_BIPED, &args![player, 0u32]).u32();
            e.vcall(
                process,
                SLOT_PROCESS_SET_BIPED,
                &args![byte, biped, this, player],
            );
        }
    } else {
        let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
        // 0049b271: slot 0x1e8 takes no stack words; `this` and the actor are
        // the last two words of slot 0x1cc.
        let biped = e.vcall(actor, SLOT_ACTOR_BIPED, &args![]).u32();
        e.vcall(
            process,
            SLOT_PROCESS_SET_BIPED,
            &args![byte, biped, this, actor],
        );
    }
    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
    e.vcall(process, SLOT_PROCESS_NOTIFY, &args![actor]);
    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
    e.vcall(process, SLOT_PROCESS_REFRESH, &args![]);
    let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
    e.vcall(process, SLOT_PROCESS_SET_HELD, &args![0xffff_ffffu32, 0u32]);
    for index in 0..8u32 {
        let sequence = e.mem.u32(current_sequence_at(this, index));
        if sequence != 0 {
            let begin = e.call(FLOAT_AT_48, &args![sequence]).f32();
            let end = e.call(SEQUENCE_END_TIME, &args![sequence]).f64();
            let total = (end + begin as f64) as f32;
            e.call(SET_FLOAT_AT_48, &args![sequence, total]);
        }
    }
    let time = e.get(this, Animation::time);
    animation_update_bip_only(e, this, time, Ptr::new(a + 0x1c), 1);
    let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
    let time = e.get(this, Animation::time);
    e.call(BIP_UPDATE_ALL_BUT_BIP, &args![root, time]);
    let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
    let collision = e.call(FIND_NEXT_COLLISION_OBJECT, &args![root]).u32();
    let flagged = if collision != 0 {
        e.call(COLLISION_OBJECT_FLAGGED, &args![collision]).u8()
    } else {
        0
    };
    if flagged != 0 {
        let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
        e.call(SET_CO_USE_VEL, &args![0u32, root]);
    }
    animation_update_scene_graph_no_controller(e, this);
    if flagged != 0 {
        let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
        e.call(SET_CO_USE_VEL, &args![flagged as u32, root]);
    }
    animation_clear_group(e, this, 0x14, 0.0);
    fn_00496280(e, this);
    if manager_of(e, this).addr() != 0 {
        let manager = manager_of(e, this);
        ni_controller_manager_deactivate_all(e, manager, 0.0);
    }
    for (offset, size) in [
        (0x10cu32, 4u32),
        (0x110, 4),
        (0x10, 0xc),
        (0x48, 4),
        (0xcc, 1),
        (0xd4, 4),
        (0x122, 2),
    ] {
        e.call(LOAD_BUFFER_FIELD, &args![buffer, a + offset, size]);
    }
    let count = e.call(LOAD_BUFFER_VARIABLE, &args![buffer]).u32();
    for _ in 0..count {
        let item = e.call(OPERATOR_NEW, &args![8u32]).u32();
        let id = e.call(LOAD_BUFFER_FORM_ID, &args![buffer]).u32();
        let form = e.call(FORM_BY_ID, &args![id]).u32();
        let idle_form = e
            .call(
                RT_DYNAMIC_CAST,
                &args![
                    form,
                    0u32,
                    TYPE_DESCRIPTOR_FORM,
                    TYPE_DESCRIPTOR_IDLE_FORM,
                    0u32
                ],
            )
            .u32();
        e.mem.set_u32(item, idle_form);
        e.call(LOAD_BUFFER_FIELD, &args![buffer, item + 4, 4u32]);
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), item);
            e.call(
                REPLAY_LIST_PUSH,
                &args![a + Animation::replayDelayList.off, cell],
            );
        });
    }
    // The idle: a form ID, then its state.
    let mut id = 0u32;
    let found = e.with_stack(4, |e, cell| {
        e.mem.set_u32(cell.addr(), 0);
        let found = e.call(LOAD_BUFFER_FORM_ID_FOUND, &args![buffer, cell]).u8();
        id = e.mem.u32(cell.addr());
        found
    });
    if found != 0 {
        let length = e.call(LOAD_BUFFER_VARIABLE, &args![buffer]).u32();
        fn_0049bb50(e, buffer, length);
    } else if id != 0 {
        let form = e.call(FORM_BY_ID, &args![id]).u32();
        let idle_form = e
            .call(
                RT_DYNAMIC_CAST,
                &args![
                    form,
                    0u32,
                    TYPE_DESCRIPTOR_FORM,
                    TYPE_DESCRIPTOR_IDLE_FORM,
                    0u32
                ],
            )
            .u32();
        if idle_form == 0 {
            let length = e.call(LOAD_BUFFER_VARIABLE, &args![buffer]).u32();
            fn_0049bb50(e, buffer, length);
        } else {
            e.call(LOAD_BUFFER_VARIABLE, &args![buffer]);
            let block = e.call(NI_OPERATOR_NEW, &args![0x38u32]).u32();
            let mut idle = Ptr::<AnimIdle>::NULL;
            if block != 0 {
                idle = anim_idle_constructor(
                    e,
                    Ptr::new(block),
                    Ptr::new(idle_form),
                    7,
                    1,
                    Ptr::new(actor),
                    1,
                    Ptr::NULL,
                );
            }
            e.call(
                NI_POINTER_SET,
                &args![idle_field(this, Animation::spAnimIdle.off), idle],
            );
            let idle = idle_in(e, this, Animation::spAnimIdle.off);
            fn_00497c10(e, idle, buffer, this);
        }
    }
    e.with_stack(0x10, |e, cells| {
        let mask = cells.addr();
        let selector = cells.addr() + 1;
        let phase = cells.addr() + 2;
        let held_slot = cells.addr() + 4;
        let idle_slot = cells.addr() + 5;
        e.mem.set_u8(mask, 0);
        e.call(LOAD_BUFFER_FIELD, &args![buffer, mask, 1u32]);
        for index in 0..8u32 {
            if (e.mem.u8(mask) as i8 as i32) & (1 << index) == 0 {
                continue;
            }
            for (address, size) in [
                (group_at(this, index), 2u32),
                (action_at(this, index), 4),
                (next_group_at(this, index), 2),
                (loop_count_at(this, index), 4),
                (next_loops_at(this, index), 4),
            ] {
                e.call(LOAD_BUFFER_FIELD, &args![buffer, address, size]);
            }
            e.mem.set_u32(current_sequence_at(this, index), 0);
            e.mem.set_u8(selector, 0xff);
            e.call(LOAD_BUFFER_FIELD, &args![buffer, selector, 1u32]);
            if e.mem.u8(selector) as i8 != -2 {
                fn_004974a0(e, this);
                let group = e.mem.u16(group_at(this, index));
                let action = e.mem.i32(action_at(this, index));
                let time = e.get(this, Animation::time);
                let chosen = e.mem.u8(selector);
                animation_force_section(e, this, index as i32, group, action, time, chosen);
                let sequence = e.mem.u32(current_sequence_at(this, index));
                let time = e.get(this, Animation::time);
                if sequence != 0 {
                    e.call(SEQUENCE_LOAD_FROM_BUFFER, &args![sequence, buffer, time]);
                } else {
                    e.with_stack(ANIM_GROUP_SEQUENCE_SIZE, |e, temporary| {
                        fn_0049baa0(e, temporary);
                        e.call(SEQUENCE_LOAD_FROM_BUFFER, &args![temporary, buffer, time]);
                        animation_clear_group(e, this, index as i32, 0.0);
                        e.call(SEQUENCE_DESTRUCT, &args![temporary]);
                    });
                }
            }
        }
        e.call(
            LOAD_BUFFER_FIELD,
            &args![buffer, a + Animation::cSkipNextBlend.off, 1u32],
        );
        e.mem.set_u16(phase, 0xffff);
        e.mem.set_u8(held_slot, 0xff);
        e.call(LOAD_BUFFER_FIELD, &args![buffer, phase, 2u32]);
        e.call(LOAD_BUFFER_FIELD, &args![buffer, held_slot, 1u32]);
        let held_index = e.mem.u8(held_slot) as i8;
        if held_index != -1 {
            let held = e.mem.u32(current_sequence_at(this, held_index as u32));
            if held != 0 {
                let process = e.call(ACTOR_PROCESS, &args![actor]).u32();
                let phase_value = e.mem.u16(phase) as i16 as i32;
                e.vcall(process, SLOT_PROCESS_SET_HELD, &args![phase_value, held]);
            }
        }
        e.mem.set_u8(idle_slot, 0xff);
        e.call(LOAD_BUFFER_FIELD, &args![buffer, idle_slot, 1u32]);
        if ni_pointer_get(e, idle_field(this, Animation::spAnimIdle.off)) != 0 {
            let idle_index = e.mem.u8(idle_slot) as i8;
            if idle_index != -1 {
                let sequence = e.mem.u32(current_sequence_at(this, idle_index as u32));
                let idle = idle_in(e, this, Animation::spAnimIdle.off);
                fn_004970e0(e, idle, Ptr::new(sequence));
            }
            let idle = idle_in(e, this, Animation::spAnimIdle.off);
            if idle_state(e, idle) == 1 {
                fn_004974a0(e, this);
                fn_00498290(e, this);
            }
        }
    });
    let time = e.get(this, Animation::time);
    animation_update_bip_only(e, this, time, Ptr::new(a + 0x1c), 1);
    let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
    let time = e.get(this, Animation::time);
    e.call(BIP_UPDATE_ALL_BUT_BIP, &args![root, time]);
}

// ---- BSAnimGroupSequence and small helpers ---------------------------------------------

// Translated from 0049baa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimGroupSequence`'s default constructor (the map has no name for it):
/// the `NiControllerSequence` base constructor (`00a316a0`), the
/// `BSAnimGroupSequence` vtable, and an empty `spAnimGroup` (+0x74). Returns
/// `this`. The compiler's exception frame is not translated.
pub fn fn_0049baa0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SEQUENCE_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE_ANIM_GROUP_SEQUENCE);
    e.call(
        NI_POINTER_INIT,
        &args![this.addr() + BSAnimGroupSequence::spAnimGroup.off, 0u32],
    );
    this
}

// Translated from 0049bb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimGroupSequence::GetRTTI` (Xbox PDB): the class's `NiRTTI`
/// (`RTTI_ANIM_GROUP_SEQUENCE`).
pub fn bs_anim_group_sequence_get_rtti(_e: &mut Engine, _this: Ptr) -> u32 {
    RTTI_ANIM_GROUP_SEQUENCE
}

// Translated from 0049bb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAnimGroupSequence::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor (`004eeb00`), then `NiMemObject::operator delete(this, 0x78)`
/// when bit 0 of `flags` is set. Returns `this`.
pub fn bs_anim_group_sequence_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(SEQUENCE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_OPERATOR_DELETE, &args![this, ANIM_GROUP_SEQUENCE_SIZE]);
    }
    this
}

// Translated from 0049bb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `count` to the word at +0xc of a load game buffer (it skips `count`
/// bytes; the linker folded the body with `ExternalStatistics::
/// IncrementDequeuedTaskCounter`, whose name the map gives it).
pub fn fn_0049bb50(e: &mut Engine, this: Ptr, count: u32) {
    let field = this.addr() + 0xc;
    let value = e.mem.u32(field).wrapping_add(count);
    e.mem.set_u32(field, value);
}

// Translated from 0049bb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the animation (the map has no name for it, the declared word after
/// `this` is never read): `cSkipNextBlend` cleared, then the groups and idles
/// ([`clear_idles_and_slot_7`] steps), and the replay delays freed and the list
/// emptied.
pub fn fn_0049bb70(e: &mut Engine, this: Ptr<Animation>, _unused_0: u32) {
    e.set(this, Animation::cSkipNextBlend, 0u8);
    clear_idles_and_slot_7(e, this);
    let list = this.addr() + Animation::replayDelayList.off;
    let mut node = list;
    while node != 0 {
        let cell = e.call(RECORD_ADDRESS, &args![node]).u32();
        let item = e.mem.u32(cell);
        e.call(OPERATOR_DELETE, &args![item]);
        node = e.call(WORD_AT_4, &args![node]).u32();
    }
    e.call(SIMPLE_LIST_CLEAR, &args![list]);
}

// Translated from 0049bca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::UpdateBipOnly` (Xbox PDB), `(time, out, _)`: moves the
/// accumulation root to `AccumRootTranslate` (+0x1c), updates the bip nodes
/// of the animation root (`BSBipNode::UpdateBipOnly(time, accumRoot)`) and,
/// with an accumulation root, copies its world translation (three words) to
/// `out` when `out` is not null and moves the root back to the zero vector.
/// The third word is never read.
pub fn animation_update_bip_only(
    e: &mut Engine,
    this: Ptr<Animation>,
    time: f32,
    out: Ptr,
    _unused_2: u32,
) {
    let a = this.addr();
    let accum_root = e.get(this, Animation::pAccumRoot).addr();
    if accum_root != 0 {
        e.call(
            NODE_SET_WORLD_TRANSLATION,
            &args![accum_root, a + Animation::AccumRootTranslate.off],
        );
    }
    let accum_root = e.get(this, Animation::pAccumRoot).addr();
    let root = ni_pointer_get(e, a + Animation::pAnimRoot.off);
    e.call(BIP_NODE_UPDATE_BIP_ONLY, &args![root, time, accum_root]);
    let accum_root = e.get(this, Animation::pAccumRoot).addr();
    if accum_root != 0 {
        if !out.is_null() {
            let translation = e.call(NODE_WORLD_TRANSLATION, &args![accum_root]).u32();
            for i in 0..3u32 {
                let word = e.mem.u32(translation + 4 * i);
                e.mem.set_u32(out.addr() + 4 * i, word);
            }
        }
        let accum_root = e.get(this, Animation::pAccumRoot).addr();
        e.call(NODE_SET_WORLD_TRANSLATION, &args![accum_root, ZERO_VECTOR]);
    }
}

// Translated from 0049bd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of models queued in `kfModelList` (+0x104); the map has no name
/// for it.
pub fn fn_0049bd30(e: &mut Engine, this: Ptr<Animation>) -> u32 {
    let list = this.addr() + Animation::kfModelList.off;
    let mut count = 0u32;
    if !e.call(SIMPLE_LIST_IS_EMPTY, &args![list]).bool() {
        let mut node = list;
        while node != 0 {
            count += 1;
            node = e.call(WORD_AT_4, &args![node]).u32();
        }
    }
    count
}

// Translated from 0049bd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands `object` to the animation root (`004f05c0(root, object)`) when the
/// animation has a root (`pAnimRoot`, read through `005585e0`); the linker
/// folded the body with `CHtmlView::OnStatusTextChange`, whose name the map
/// gives it.
pub fn fn_0049bd90(e: &mut Engine, this: Ptr<Animation>, object: Ptr) {
    let root = e.call(KF_MODEL_ANIM_GROUP, &args![this]).u32();
    if root != 0 {
        e.call(ROOT_NODE_NOTIFY, &args![root, object]);
    }
}

// Translated from 0049bdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drives the group 0xe1 in slot 3 with `weight` (the map has no name for
/// it). A weight of exactly 0 updates the slot's sequence to time 0 and clears
/// the slot. Otherwise starts the group when the slot is empty (and updates the
/// new sequence to its begin time plus `time`), and while the slot's sequence
/// is in state 1 stores a time between its begin and end times in
/// `fLipTime` (+0xd4, begin + (end - begin) * weight) and sets the sequence's
/// phase to `fLipTime - time`.
pub fn fn_0049bdc0(e: &mut Engine, this: Ptr<Animation>, weight: f32) {
    let slot_3 = current_sequence_at(this, 3);
    if weight == 0.0 {
        let sequence = e.mem.u32(slot_3);
        if sequence != 0 {
            e.call(SEQUENCE_UPDATE, &args![sequence, 0.0f32, 1u32]);
            animation_clear_group(e, this, 3, 0.0);
        }
        return;
    }
    if e.mem.u32(slot_3) == 0 {
        animation_play_group(e, this, 0xe1, 1, -1, -1);
        let sequence = e.mem.u32(slot_3);
        if sequence != 0 {
            let begin = e.call(SEQUENCE_BEGIN_TIME, &args![sequence]).f32();
            let time = e.get(this, Animation::time);
            let moment = (begin as f64 + time as f64) as f32;
            e.call(SEQUENCE_UPDATE, &args![sequence, moment, 1u32]);
        }
    }
    let sequence = e.mem.u32(slot_3);
    if sequence != 0 && e.call(SEQUENCE_STATE, &args![sequence]).i32() == 1 {
        let end = e.call(SEQUENCE_END_TIME, &args![sequence]).f64();
        let begin = e.call(SEQUENCE_BEGIN_TIME, &args![sequence]).f32();
        let span = (end - begin as f64) as f32;
        let begin = e.call(SEQUENCE_BEGIN_TIME, &args![sequence]).f32();
        let lip_time = (begin as f64 + span as f64 * weight as f64) as f32;
        e.set(this, Animation::fLipTime, lip_time);
        let time = e.get(this, Animation::time);
        let phase = (lip_time as f64 - time as f64) as f32;
        e.call(SEQUENCE_SET_PHASE, &args![sequence, phase, 1u32]);
    }
}

// Translated from 0049bf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drives a sequence on a controller manager by `weight` (`__cdecl(weight,
/// manager, sequence)`; the map has no name for it). Does nothing without
/// both. A weight of exactly 0 resets a sequence in state 1 (phase 0, update
/// to time 0). Otherwise a sequence in state 0 is activated and updated to its
/// begin time plus the time in seconds `TIME_SCALE_OBJECT` keeps in
/// milliseconds at +0x14, and a sequence in state 1 gets the phase
/// begin + (end - begin) * weight minus that time.
pub fn fn_0049bf10(e: &mut Engine, weight: f32, manager: Ptr, sequence: Ptr) {
    if manager.is_null() || sequence.is_null() {
        return;
    }
    let sequence = sequence.addr();
    if weight == 0.0 {
        if e.call(SEQUENCE_STATE, &args![sequence]).i32() == 1 {
            e.call(SEQUENCE_SET_PHASE, &args![sequence, 0.0f32, 1u32]);
            e.call(SEQUENCE_UPDATE, &args![sequence, 0.0f32, 1u32]);
        }
        return;
    }
    if e.call(SEQUENCE_STATE, &args![sequence]).i32() == 0 {
        e.call(MANAGER_SET_FLAG, &args![manager, 1u32]);
        e.call(
            MANAGER_ACTIVATE,
            &args![manager, sequence, 0u32, 0u32, 1.0f32, 0.0f32, 0u32],
        );
        let milliseconds = e.call(WORD_AT_14, &args![TIME_SCALE_OBJECT]).u32();
        let seconds = milliseconds as f64 / double_constant(e, DOUBLE_THOUSAND);
        let begin = e.call(SEQUENCE_BEGIN_TIME, &args![sequence]).f32();
        let moment = (begin as f64 + seconds) as f32;
        e.call(SEQUENCE_UPDATE, &args![sequence, moment, 1u32]);
    }
    if e.call(SEQUENCE_STATE, &args![sequence]).i32() == 1 {
        let end = e.call(SEQUENCE_END_TIME, &args![sequence]).f64();
        let begin = e.call(SEQUENCE_BEGIN_TIME, &args![sequence]).f32();
        let span = (end - begin as f64) as f32;
        let begin = e.call(SEQUENCE_BEGIN_TIME, &args![sequence]).f32();
        let target = (begin as f64 + span as f64 * weight as f64) as f32;
        let milliseconds = e.call(WORD_AT_14, &args![TIME_SCALE_OBJECT]).u32();
        let seconds = milliseconds as f64 / double_constant(e, DOUBLE_THOUSAND);
        let phase = (target as f64 - seconds) as f32;
        e.call(SEQUENCE_SET_PHASE, &args![sequence, phase, 1u32]);
    }
}

// Translated from 0049c050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of `NiTPointerMap<unsigned short, AnimSequenceBase*>`
/// (`pAnimSequenceMap`'s class): the base constructor with the hash size
/// (`0049c100`), then the class's vtable. Returns `this`.
pub fn fn_0049c050(e: &mut Engine, this: Ptr, hash_size: u32) -> Ptr {
    e.call(MAP_BASE_CONSTRUCT, &args![this, hash_size]);
    e.mem.set_u32(this.addr(), VTABLE_ANIM_SEQUENCE_MAP);
    this
}

// ---- Fifth session: the sequence map's classes and the tail ------------------

/// `NiTMapBase` base class vtable (the one `0049c100` and `0049c570` set; the
/// derived `NiTPointerMap` instance's is [`VTABLE_ANIM_SEQUENCE_MAP`]), the
/// array allocator and its release, and the list helpers `0049c0b0` uses.
const VTABLE_MAP_BASE: u32 = 0x0101_def0;
const ARRAY_ALLOC: u32 = 0x00aa_1070;
const ARRAY_FREE: u32 = 0x00aa_10f0;
const LIST_FIND_NODE: u32 = 0x0049_c680;
const LIST_REMOVE_NODE: u32 = 0x0049_c5d0;

// Translated from 0049c080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned short, AnimSequenceBase*>::scalar deleting
/// destructor` (Xbox PDB): the destructor `0049c510`, then `operator delete`
/// when bit 0 of `flags` is set. Returns `this`.
pub fn fn_0049c080(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    fn_0049c510(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0049c0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTList::Remove` by item address: looks the list node holding a value
/// equal to the word at `item` up (`0049c680(item, 0)`) and removes it
/// (`0049c5d0`, which returns the removed value); without such a node the
/// result is the word at `item` itself.
pub fn fn_0049c0b0(e: &mut Engine, this: Ptr, item: Ptr) -> u32 {
    let node = e.call(LIST_FIND_NODE, &args![this, item, 0u32]).u32();
    if node != 0 {
        e.with_stack(4, |e, cell| {
            e.mem.set_u32(cell.addr(), node);
            e.call(LIST_REMOVE_NODE, &args![this, cell]).u32()
        })
    } else {
        e.mem.u32(item.addr())
    }
}

// Translated from 0049c100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base constructor of the `NiTPointerMap<unsigned short,
/// AnimSequenceBase*>`: the base vtable, the bucket count, an empty item
/// count, and a bucket array of `hash_size` words, zeroed. Returns `this`.
pub fn fn_0049c100(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), VTABLE_MAP_BASE);
    e.set(this, NiTPointerMap::m_uiHashSize, hash_size);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let bytes = hash_size.wrapping_shl(2);
    let table = e.call(ARRAY_ALLOC, &args![bytes]).u32();
    e.set(this, NiTPointerMap::m_ppkHashTable, table);
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

// Translated from 0049c170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned short,
/// AnimSequenceBase*>::SetAt`: walks the bucket the hash (vtable slot 4)
/// picks; a node whose key the comparison (slot 8) calls equal gets the new
/// value (+8); otherwise a node from the allocator (slot 0x14) is filled
/// (`SetValue`, slot 0xc), linked at the head of the bucket, and counted.
pub fn fn_0049c170(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u16, value: u32) {
    let index = e.vcall(this.addr(), 4, &args![key]).u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let bucket = table.wrapping_add(index.wrapping_mul(4));
    let mut node = e.mem.u32(bucket);
    while node != 0 {
        let node_key = e.mem.u16(node + 4);
        if e.vcall(this.addr(), 8, &args![key, node_key]).bool() {
            e.mem.set_u32(node + 8, value);
            return;
        }
        node = e.mem.u32(node);
    }
    let fresh = e.vcall(this.addr(), 0x14, &args![]).u32();
    e.vcall(this.addr(), 0xc, &args![fresh, key, value]);
    // The bucket array is read again, as the code does after the calls.
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let bucket = table.wrapping_add(index.wrapping_mul(4));
    let head = e.mem.u32(bucket);
    e.mem.set_u32(fresh, head);
    e.mem.set_u32(bucket, fresh);
    let count = e.get(this, NiTPointerMap::m_uiCount);
    e.set(this, NiTPointerMap::m_uiCount, count.wrapping_add(1));
}

// Translated from 0049c390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned short,
/// AnimSequenceBase*>::GetAt` (Xbox PDB): the value (+8) of the node in the
/// key's bucket whose key the comparison (vtable slot 8) accepts is stored at
/// `value_out`; false when there is none.
pub fn fn_0049c390(e: &mut Engine, this: Ptr<NiTPointerMap>, key: u16, value_out: Ptr) -> bool {
    let index = e.vcall(this.addr(), 4, &args![key]).u32();
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    let mut node = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    while node != 0 {
        let node_key = e.mem.u16(node + 4);
        if e.vcall(this.addr(), 8, &args![key, node_key]).bool() {
            let value = e.mem.u32(node + 8);
            e.mem.set_u32(value_out.addr(), value);
            return true;
        }
        node = e.mem.u32(node);
    }
    false
}

// Translated from 0049c410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., unsigned short, AnimSequenceBase*>::GetNext`: gives the
/// key (`u16`) and value of the node at the position `*position`, then moves
/// the position to the next node of its bucket, or to the first node of the
/// next non-empty bucket (the hash of the key plus one onwards), or to null.
pub fn fn_0049c410(
    e: &mut Engine,
    this: Ptr<NiTPointerMap>,
    position: Ptr,
    key_out: Ptr,
    value_out: Ptr,
) {
    let node = e.mem.u32(position.addr());
    let key = e.mem.u16(node + 4);
    e.mem.set_u16(key_out.addr(), key);
    let value = e.mem.u32(node + 8);
    e.mem.set_u32(value_out.addr(), value);
    let next = e.mem.u32(node);
    if next != 0 {
        e.mem.set_u32(position.addr(), next);
        return;
    }
    let mut index = e.vcall(this.addr(), 4, &args![key]).u32().wrapping_add(1);
    loop {
        if index >= e.get(this, NiTPointerMap::m_uiHashSize) {
            e.mem.set_u32(position.addr(), 0);
            return;
        }
        let table = e.get(this, NiTPointerMap::m_ppkHashTable);
        let head = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
        if head != 0 {
            e.mem.set_u32(position.addr(), head);
            return;
        }
        index = index.wrapping_add(1);
    }
}

// Translated from 0049c4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., unsigned short, AnimSequenceBase*>::SetValue` (Xbox PDB):
/// fills a map node (`node`, the first stack word; `ECX` is not read) with
/// its key (+4, `u16`) and value (+8).
pub fn fn_0049c4e0(e: &mut Engine, _this: Ptr, node: Ptr, key: u16, value: u32) {
    e.mem.set_u16(node.addr() + 4, key);
    e.mem.set_u32(node.addr() + 8, value);
}

// Translated from 0049c510 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of `NiTPointerMap<unsigned short, AnimSequenceBase*>`:
/// its vtable, `RemoveAll` (`00438af0`), then the base destructor
/// (`0049c570`). The exception-unwinding frame is not translated.
pub fn fn_0049c510(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), VTABLE_ANIM_SEQUENCE_MAP);
    e.call(MAP_REMOVE_ALL, &args![this]);
    fn_0049c570(e, this);
}

// Translated from 0049c570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The base destructor of the map (`NiTMapBase`): the base vtable,
/// `RemoveAll` (`00438af0`), and the release of the bucket array.
pub fn fn_0049c570(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), VTABLE_MAP_BASE);
    e.call(MAP_REMOVE_ALL, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(ARRAY_FREE, &args![table]);
}

// Translated from 0049c5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned short,
/// AnimSequenceBase*>::scalar deleting destructor` (Xbox PDB): the base
/// destructor `0049c570`, then `operator delete` when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_0049c5a0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    fn_0049c570(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0058cb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the four values of the object the pointer at `this + 0x2c` holds
/// (the map names it `NiQuatTransform::SetTranslate`, a folded name; the body
/// is a getter): `fn_0058cb60` on that object writes them to the three out
/// pointers and returns the fourth. Without the object the outs are zero
/// (the third one a byte) and the result 0.
pub fn fn_0058cb00(e: &mut Engine, this: Ptr, out_a: Ptr, out_b: Ptr, out_c: Ptr) -> u32 {
    let field = this.addr() + 0x2c;
    if e.call(READ_WORD, &args![field]).u32() == 0 {
        e.mem.set_u32(out_a.addr(), 0);
        e.mem.set_u32(out_b.addr(), 0);
        e.mem.set_u8(out_c.addr(), 0);
        return 0;
    }
    let object = e.call(READ_WORD, &args![field]).ptr();
    fn_0058cb60(e, object, out_a, out_b, out_c)
}

// Translated from 0058cb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Getter: the `u16` at +8 (zero-extended) to `out_a`, the word at +0x10 to
/// `out_b`, the byte at +0x1c to `out_c`; returns the word at +0x20.
pub fn fn_0058cb60(e: &mut Engine, this: Ptr, out_a: Ptr, out_b: Ptr, out_c: Ptr) -> u32 {
    let a = this.addr();
    let first = e.mem.u16(a + 8) as u32;
    e.mem.set_u32(out_a.addr(), first);
    let second = e.mem.u32(a + 0x10);
    e.mem.set_u32(out_b.addr(), second);
    let third = e.mem.u8(a + 0x1c);
    e.mem.set_u8(out_c.addr(), third);
    e.mem.u32(a + 0x20)
}

// Translated from 00f39c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SpecialIdleAuto` (Xbox PDB name of the slot; the body is a
/// static initializer): constructs the `bAnimateDoorPhysics:General` setting
/// object at `011c4c64` (name string at `0101abb4`, value 1) and registers its
/// destructor (`00fc8470`) with `atexit`.
pub fn animation_special_idle_auto(e: &mut Engine) {
    e.call(0x0045_cf30, &args![0x011c_4c64u32, 0x0101_abb4u32, 1u32]);
    e.call(0x00ec_658f, &args![0x00fc_8470u32]);
}

// Translated from 008eeaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Animation::SkipUpdate` (Xbox PDB): stores `value` in `cSkipUpdate`.
pub fn animation_skip_update(e: &mut Engine, this: Ptr<Animation>, value: u8) {
    e.set(this, Animation::cSkipUpdate, value);
}

// Translated from 00a3f9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map names it `NiQuatTransform::SetRotate` (a folded name; the body is
/// a getter). Like `fn_0058cb60`, on the object the pointer at `this + 0x2c`
/// holds, with its fields 0x0c (`u16`), 0x18, 0x1e (byte) and 0x28; without
/// the object the outs are zero and the result 0.
pub fn fn_00a3f9e0(e: &mut Engine, this: Ptr, out_a: Ptr, out_b: Ptr, out_c: Ptr) -> u32 {
    let object = e.mem.u32(this.addr() + 0x2c);
    if object == 0 {
        e.mem.set_u32(out_a.addr(), 0);
        e.mem.set_u32(out_b.addr(), 0);
        e.mem.set_u8(out_c.addr(), 0);
        return 0;
    }
    let first = e.mem.u16(object + 0xc) as u32;
    e.mem.set_u32(out_a.addr(), first);
    let second = e.mem.u32(object + 0x18);
    e.mem.set_u32(out_b.addr(), second);
    let third = e.mem.u8(object + 0x1e);
    e.mem.set_u8(out_c.addr(), third);
    e.mem.u32(object + 0x28)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048ee70, fn_0048ee70(Ptr<AnimSequenceSingle>)),
        entry!(
            0x0048ef60,
            anim_sequence_base_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0048ef90, fn_0048ef90(Ptr<AnimSequenceSingle>, u32) -> Ptr),
        entry!(0x0048efb0, fn_0048efb0(Ptr<AnimSequenceSingle>, u32) -> u8),
        entry!(
            0x0048efc0,
            anim_sequence_single_scalar_deleting_destructor(
                Ptr<AnimSequenceSingle>,
                u32,
            ) -> Ptr<AnimSequenceSingle>
        ),
        entry!(0x0048eff0, fn_0048eff0(Ptr, u32)),
        entry!(
            0x0048f010,
            anim_sequence_single_set_sequence(Ptr<AnimSequenceSingle>, Ptr, u8)
        ),
        entry!(
            0x0048f080,
            anim_sequence_single_remove_sequence(Ptr<AnimSequenceSingle>, Ptr, u8) -> bool
        ),
        entry!(
            0x0048f0c0,
            fn_0048f0c0(Ptr<AnimSequenceMultiple>, Ptr) -> Ptr<AnimSequenceMultiple>
        ),
        entry!(0x0048f1b0, fn_0048f1b0(Ptr) -> Ptr),
        entry!(
            0x0048f1d0,
            anim_sequence_multiple_scalar_deleting_destructor(
                Ptr<AnimSequenceMultiple>,
                u32,
            )
                -> Ptr<AnimSequenceMultiple>
        ),
        entry!(0x0048f200, fn_0048f200(Ptr) -> Ptr),
        entry!(0x0048f220, fn_0048f220(Ptr<AnimSequenceMultiple>)),
        entry!(0x0048f370, fn_0048f370(Ptr, u32) -> Ptr),
        entry!(
            0x0048f3a0,
            anim_sequence_multiple_set_sequence(Ptr<AnimSequenceMultiple>, Ptr, u32)
        ),
        entry!(
            0x0048f3d0,
            anim_sequence_multiple_remove_sequence(Ptr<AnimSequenceMultiple>, Ptr, u32) -> bool
        ),
        entry!(
            0x0048f450,
            anim_sequence_multiple_get_sequence_to_play(Ptr<AnimSequenceMultiple>, i8) -> Ptr
        ),
        entry!(
            0x0048f500,
            anim_sequence_multiple_get_corresponding_sequence(
                Ptr<AnimSequenceMultiple>,
                Ptr,
                i32,
            ) -> Ptr
        ),
        entry!(
            0x0048f720,
            anim_sequence_multiple_get_sequence_index(Ptr<AnimSequenceMultiple>, Ptr) -> u8
        ),
        entry!(
            0x0048f790,
            anim_sequence_multiple_get_sequence(Ptr<AnimSequenceMultiple>, Ptr) -> Ptr
        ),
        entry!(0x0048f7f0, fn_0048f7f0(Ptr<BSAnimGroupSequence>) -> Ptr),
        entry!(
            0x0048f810,
            animation_animation(Ptr<Animation>) -> Ptr<Animation>
        ),
        entry!(0x0048fb20, fn_0048fb20(Ptr<Animation>, u8, u8, u8)),
        entry!(0x0048fb50, fn_0048fb50(Ptr<Animation>)),
        entry!(
            0x0048fef0,
            ni_controller_manager_deactivate_all(Ptr<NiControllerManager>, f32)
        ),
        entry!(
            0x0048ff50,
            animation_shutdown_all_anim_idles(Ptr<Animation>)
        ),
        entry!(
            0x0048ffd0,
            fn_0048ffd0(Ptr<Animation>, Ptr, Ptr, Ptr, bool) -> bool
        ),
        entry!(0x004902f0, fn_004902f0(Ptr, u8)),
        entry!(0x00490310, fn_00490310(Ptr, Ptr, u32) -> u32),
        entry!(
            0x00490330,
            animation_add_special_animations(Ptr<Animation>, Ptr, u32)
        ),
        entry!(
            0x00490400,
            animation_add_animations_from_list(Ptr<Animation>, Ptr)
        ),
        entry!(
            0x00490500,
            animation_add_animation(Ptr<Animation>, u32, bool) -> bool
        ),
        entry!(
            0x00490d90,
            ni_controller_manager_get_accum_root(Ptr<NiControllerManager>) -> Ptr
        ),
        entry!(
            0x00490e10,
            fn_00490e10(Ptr<AnimSequenceSingle>) -> Ptr<AnimSequenceSingle>
        ),
        entry!(0x00490e40, fn_00490e40(Ptr) -> Ptr),
        entry!(
            0x00490e60,
            animation_should_queue_sequence_cloning(Ptr<Animation>) -> bool
        ),
        entry!(0x00490f80, fn_00490f80(Ptr) -> Ptr),
        entry!(0x00490fa0, fn_00490fa0(Ptr<Animation>, u32)),
        entry!(0x00491040, fn_00491040(Ptr<Animation>, u32) -> Ptr),
        entry!(0x00491090, fn_00491090(Ptr) -> f32),
        entry!(0x004910d0, animation_find_skinned_node(Ptr) -> bool),
        entry!(0x00491180, animation_update(Ptr<Animation>, Ptr, f32, f32)),
        entry!(0x00493750, fn_00493750(Ptr) -> i32),
        entry!(0x00493860, fn_00493860(Ptr<Animation>, u8)),
        entry!(0x00493900, animation_update_movement(Ptr<Animation>, Ptr)),
        entry!(
            0x00493930,
            animation_update_queued_scenegraph(Ptr<Animation>) -> bool
        ),
        entry!(0x00493970, fn_00493970(Ptr<Animation>) -> bool),
        entry!(0x00493990, fn_00493990(Ptr<Animation>, u8) -> bool),
        entry!(0x004939b0, fn_004939b0(Ptr<Animation>, u8)),
        entry!(
            0x004939d0,
            animation_update_movement_no_world_update(Ptr<Animation>, Ptr)
        ),
        entry!(0x00493b90, fn_00493b90(Ptr<Animation>) -> bool),
        entry!(0x00493bb0, fn_00493bb0(Ptr) -> u8),
        entry!(
            0x00493bd0,
            animation_update_scene_graph_no_controller(Ptr<Animation>)
        ),
        entry!(0x00493d50, animation_init_group_speed(Ptr<Animation>, Ptr)),
        entry!(0x004941c0, fn_004941c0(Ptr, f32) -> Ptr),
        entry!(0x00494210, fn_00494210(Ptr, u32) -> u32),
        entry!(0x00494260, fn_00494260(Ptr) -> Ptr),
        entry!(0x004942c0, fn_004942c0(Ptr) -> bool),
        entry!(0x00494300, fn_00494300(Ptr<Animation>, u16) -> i32),
        entry!(
            0x00494390,
            fn_00494390(Ptr<Animation>, Ptr, Ptr, u8, u8) -> bool
        ),
        entry!(0x004946a0, animation_add_group(Ptr<Animation>, Ptr)),
        entry!(
            0x00494710,
            animation_group_loaded(Ptr<Animation>, u16) -> bool
        ),
        entry!(
            0x00494740,
            animation_play_group(Ptr<Animation>, u16, i32, i32, i32) -> u32
        ),
        entry!(
            0x004948c0,
            animation_start_group(Ptr<Animation>, u16, i32) -> Ptr
        ),
        entry!(
            0x004949a0,
            animation_start_group_ov2(Ptr<Animation>, Ptr, u16, i32) -> Ptr
        ),
        entry!(0x00495460, fn_00495460(Ptr) -> f32),
        entry!(0x00495480, fn_00495480(Ptr, f32)),
        entry!(0x004954c0, fn_004954c0(Ptr)),
        entry!(0x004954e0, fn_004954e0(Ptr) -> u8),
        entry!(0x00495520, fn_00495520(Ptr) -> u8),
        entry!(0x00495560, fn_00495560(Ptr, i32) -> u32),
        entry!(0x00495580, fn_00495580() -> u8),
        entry!(0x004955a0, fn_004955a0(Ptr) -> u8),
        entry!(
            0x004955c0,
            animation_force_section(Ptr<Animation>, i32, u16, i32, f32, u8)
        ),
        entry!(
            0x00495740,
            animation_pick_best_animation(Ptr<Animation>, u16, u8) -> u16
        ),
        entry!(
            0x00495be0,
            animation_should_be_moving(Ptr<Animation>) -> bool
        ),
        entry!(0x00495d00, fn_00495d00(Ptr<NiControllerManager>) -> u32),
        entry!(
            0x00495d20,
            ni_controller_manager_get_sequence_at(Ptr<NiControllerManager>, u32) -> Ptr
        ),
        entry!(
            0x00495d50,
            ni_blend_interpolator_get_interpolator(Ptr, u8) -> Ptr
        ),
        entry!(
            0x00495da0,
            animation_sync_sequences(Ptr<Animation>, Ptr, u16) -> Ptr
        ),
        entry!(0x00495e40, fn_00495e40(Ptr<Animation>, Ptr) -> u8),
        entry!(0x00495f30, fn_00495f30(Ptr, Ptr) -> u32),
        entry!(0x00495f50, fn_00495f50(Ptr<Animation>, i32, Ptr, Ptr) -> u8),
        entry!(0x00496080, animation_clear_group(Ptr<Animation>, i32, f32)),
        entry!(0x00496280, fn_00496280(Ptr<Animation>)),
        entry!(0x00496340, fn_00496340(Ptr)),
        entry!(0x004963b0, fn_004963b0(Ptr, Ptr)),
        entry!(0x004963e0, fn_004963e0(Ptr, Ptr)),
        entry!(0x00496420, fn_00496420(Ptr, f32)),
        entry!(0x00496440, fn_00496440(Ptr, u8)),
        entry!(0x00496470, fn_00496470(Ptr, u8)),
        entry!(0x004964a0, fn_004964a0(Ptr, u8)),
        entry!(0x004964d0, fn_004964d0(Ptr<Animation>)),
        entry!(
            0x00496500,
            animation_get_tes_anim_group(Ptr<Animation>, u16) -> Ptr
        ),
        entry!(0x00496550, fn_00496550(Ptr, Ptr) -> f32),
        entry!(
            0x004965d0,
            anim_idle_constructor(Ptr<AnimIdle>, Ptr, u32, u32, Ptr, u8, Ptr) -> Ptr<AnimIdle>
        ),
        entry!(0x004968b0, fn_004968b0(Ptr) -> Ptr),
        entry!(
            0x004968e0,
            ni_ref_object_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00496910, fn_00496910(Ptr)),
        entry!(0x00496940, fn_00496940(Ptr<Animation>) -> u32),
        entry!(0x00496960, fn_00496960(Ptr, u8)),
        entry!(
            0x00496980,
            anim_idle_scalar_deleting_destructor(Ptr<AnimIdle>, u32) -> Ptr<AnimIdle>
        ),
        entry!(0x004969b0, fn_004969b0(Ptr<AnimIdle>)),
        entry!(0x00496a50, fn_00496a50(Ptr<AnimIdle>)),
        entry!(0x00496cd0, anim_idle_loaded(Ptr<AnimIdle>, Ptr)),
        entry!(0x00496fe0, fn_00496fe0(Ptr<AnimIdle>, Ptr) -> u8),
        entry!(0x00497040, fn_00497040(Ptr<AnimIdle>, Ptr)),
        entry!(0x004970e0, fn_004970e0(Ptr<AnimIdle>, Ptr)),
        entry!(0x00497100, fn_00497100(Ptr<AnimIdle>) -> u16),
        entry!(0x00497180, fn_00497180(Ptr<AnimIdle>, f32, Ptr<Animation>)),
        entry!(0x00497280, fn_00497280(Ptr<Animation>) -> Ptr),
        entry!(0x004972a0, fn_004972a0(Ptr<AnimIdle>, f32, Ptr<Animation>)),
        entry!(0x004974a0, fn_004974a0(Ptr<Animation>)),
        entry!(0x004974c0, fn_004974c0(u32, Ptr<AnimIdle>) -> u16),
        entry!(
            0x004975e0,
            fn_004975e0(u32, Ptr<AnimIdle>, Ptr<Animation>, f32)
        ),
        entry!(
            0x004977e0,
            fn_004977e0(Ptr, Ptr<Animation>, f32) -> Ptr<AnimIdle>
        ),
        entry!(0x00497bc0, fn_00497bc0(Ptr<AnimIdle>, Ptr, u32)),
        entry!(0x00497c10, fn_00497c10(Ptr<AnimIdle>, Ptr, Ptr<Animation>)),
        entry!(
            0x00497ca0,
            animation_special_idle_queue(Ptr<Animation>, Ptr, u32)
        ),
        entry!(0x00497f20, fn_00497f20(Ptr<Animation>, Ptr, Ptr, i32, u32)),
        entry!(
            0x00498030,
            animation_special_idle_replace(Ptr<Animation>, Ptr, Ptr, u32)
        ),
        entry!(
            0x00498170,
            animation_special_idle_replace_ov2(Ptr<Animation>) -> bool
        ),
        entry!(
            0x004981f0,
            animation_special_idle_loaded(Ptr<Animation>) -> bool
        ),
        entry!(0x00498230, fn_00498230(Ptr<Animation>, Ptr) -> bool),
        entry!(0x00498290, fn_00498290(Ptr<Animation>) -> bool),
        entry!(
            0x004985b0,
            animation_special_idle_playing(Ptr<Animation>) -> bool
        ),
        entry!(
            0x004985f0,
            animation_special_idle_done_playing(Ptr<Animation>) -> bool
        ),
        entry!(0x00498670, animation_anim_idle_free(Ptr<Animation>, Ptr)),
        entry!(
            0x00498910,
            animation_special_idle_free(Ptr<Animation>, u8, u8)
        ),
        entry!(0x00498cf0, fn_00498cf0(Ptr<Animation>) -> Ptr),
        entry!(
            0x00498d30,
            animation_special_idle_working(Ptr<Animation>, Ptr) -> bool
        ),
        entry!(
            0x00498ea0,
            animation_special_idle_working_ov2(Ptr<Animation>, Ptr) -> bool
        ),
        entry!(
            0x00498f80,
            animation_special_idle_working_ov3(Ptr<Animation>) -> bool
        ),
        entry!(0x00499080, animation_clear_blend_interps(Ptr<Animation>)),
        entry!(0x00499160, animation_clear_controllers_interpolators(Ptr)),
        entry!(0x00499240, animation_reload_targets(Ptr<Animation>, u8)),
        entry!(0x004994f0, animation_blend_out(Ptr<Animation>, i32, u8)),
        entry!(0x004997b0, fn_004997b0(Ptr) -> bool),
        entry!(0x004997f0, fn_004997f0(Ptr<Animation>, Ptr) -> u16),
        entry!(0x00499b70, fn_00499b70() -> u32),
        entry!(0x00499b80, fn_00499b80() -> u32),
        entry!(0x00499b90, animation_save_game(Ptr<Animation>, Ptr)),
        entry!(0x0049a1c0, fn_0049a1c0(Ptr<Animation>, Ptr)),
        entry!(0x0049a920, fn_0049a920(Ptr<Animation>, u32)),
        entry!(0x0049aa20, fn_0049aa20(Ptr, Ptr<Animation>) -> u16),
        entry!(0x0049aa80, animation_save_animation(Ptr, Ptr<Animation>)),
        entry!(0x0049ab00, fn_0049ab00(Ptr, u32)),
        entry!(0x0049ab40, fn_0049ab40(Ptr<Animation>, Ptr)),
        entry!(0x0049b050, fn_0049b050(Ptr<Animation>, Ptr)),
        entry!(0x0049baa0, fn_0049baa0(Ptr) -> Ptr),
        entry!(
            0x0049bb10,
            bs_anim_group_sequence_get_rtti(Ptr) -> u32
        ),
        entry!(
            0x0049bb20,
            bs_anim_group_sequence_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x0049bb50, fn_0049bb50(Ptr, u32)),
        entry!(0x0049bb70, fn_0049bb70(Ptr<Animation>, u32)),
        entry!(
            0x0049bca0,
            animation_update_bip_only(Ptr<Animation>, f32, Ptr, u32)
        ),
        entry!(0x0049bd30, fn_0049bd30(Ptr<Animation>) -> u32),
        entry!(0x0049bd90, fn_0049bd90(Ptr<Animation>, Ptr)),
        entry!(0x0049bdc0, fn_0049bdc0(Ptr<Animation>, f32)),
        entry!(0x0049bf10, fn_0049bf10(f32, Ptr, Ptr)),
        entry!(0x0049c050, fn_0049c050(Ptr, u32) -> Ptr),
        entry!(
            0x0049c080,
            fn_0049c080(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x0049c0b0, fn_0049c0b0(Ptr, Ptr) -> u32),
        entry!(
            0x0049c100,
            fn_0049c100(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x0049c170, fn_0049c170(Ptr<NiTPointerMap>, u16, u32)),
        entry!(
            0x0049c390,
            fn_0049c390(Ptr<NiTPointerMap>, u16, Ptr) -> bool
        ),
        entry!(0x0049c410, fn_0049c410(Ptr<NiTPointerMap>, Ptr, Ptr, Ptr)),
        entry!(0x0049c4e0, fn_0049c4e0(Ptr, Ptr, u16, u32)),
        entry!(0x0049c510, fn_0049c510(Ptr<NiTPointerMap>)),
        entry!(0x0049c570, fn_0049c570(Ptr<NiTPointerMap>)),
        entry!(
            0x0049c5a0,
            fn_0049c5a0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x0058cb00, fn_0058cb00(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x0058cb60, fn_0058cb60(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x008eeaa0, animation_skip_update(Ptr<Animation>, u8)),
        entry!(0x00a3f9e0, fn_00a3f9e0(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00f39c20, animation_special_idle_auto()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Test support ----------------------------------------------------------

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Doubles for the tiny getters nearly every function goes through, and
    /// the pages that hold the globals the functions read.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_d000,
            0x0101_7000,
            0x0101_3000,
            0x0119_7000,
            0x0119_9000,
            0x011c_3000,
            0x011c_5000,
            0x011d_d000,
            0x011d_e000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(READ_WORD, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINTER_INIT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(NI_POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(NI_POINTER_RELEASE, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(NI_OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        // `GetName` is the address of the name field at +8, whose word is
        // the C string.
        e.register(OBJECT_NAME, |_, a| ret(a[0] + 8));
        e.register(FIXED_STRING_CSTR, |e, a| ret(e.mem.u32(a[0])));
        e.register(REF_ADD, |_, _| Ret::default());
        e.register(REF_RELEASE, |_, _| Ret::default());
        e.register(SCOPE_GUARD_CTOR, |_, _| Ret::default());
        e.register(SCOPE_GUARD_DTOR, |_, _| Ret::default());
        e.set_global(MODEL_LOADER, 0x2222_0000u32);
        e
    }
    fn start_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The calls recorded since `start_log`, without the first one (the
    /// call of the function under test).
    fn peek_log(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.clone().unwrap().into_iter().skip(1).collect()
    }

    /// Takes the recorded calls like [`peek_log`] and stops recording.
    fn end_log(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        let log = peek_log(e);
        e.call_log = None;
        log
    }

    /// The addresses called since `start_log`, in order, without the call of
    /// the function under test.
    fn called(e: &mut Engine) -> Vec<u32> {
        end_log(e).into_iter().map(|(address, _)| address).collect()
    }

    /// The argument words of the calls to `address` in `log`.
    fn arguments_of(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An object with a vtable whose `(offset, target)` slots are given.
    fn object_with_vtable(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x800);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A NUL-terminated string in game memory.
    fn cstring(e: &mut Engine, text: &str) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(address, text.as_bytes());
        address
    }

    fn read_cstring(e: &Engine, address: u32) -> String {
        let mut bytes = vec![];
        while e.mem.u8(address + bytes.len() as u32) != 0 {
            bytes.push(e.mem.u8(address + bytes.len() as u32));
        }
        String::from_utf8(bytes).unwrap()
    }

    /// An object that only has a name (`+8` holds the C string).
    fn named_object(e: &mut Engine, name: &str) -> u32 {
        let object = e.mem.alloc(0x80);
        let text = cstring(e, name);
        e.mem.set_u32(object + 8, text);
        object
    }

    /// An `NiTList`: header `{head, tail, count}` and nodes
    /// `{next, prev, item}`; doubles for its accessors.
    fn make_list(e: &mut Engine, items: &[u32]) -> u32 {
        e.register(LIST_NEXT, |e, a| ret(e.mem.u32(a[1])));
        e.register(LIST_ITEM_ADDRESS, |_, a| ret(a[1] + 8));
        e.register(LIST_COUNT, |e, a| ret(e.mem.u32(a[0] + 8)));
        let list = e.mem.alloc(12);
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(12);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 8, *item);
            next = node;
        }
        e.mem.set_u32(list, next);
        e.mem.set_u32(list + 8, items.len() as u32);
        list
    }

    /// A `BSSimpleList`: nodes `{item, next}`, the first one is the list;
    /// doubles for its accessors.
    fn make_simple_list(e: &mut Engine, items: &[u32]) -> u32 {
        e.register(SIMPLE_LIST_ITEM, |_, a| ret(a[0]));
        e.register(SIMPLE_LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(SIMPLE_LIST_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(SIMPLE_LIST_POP_FRONT, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        let mut head = 0;
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
            head = node;
        }
        if head == 0 {
            head = e.mem.alloc(8);
        }
        head
    }

    /// Plain C string functions as doubles (`strrchr`, `strchr`, `strcpy`,
    /// `strcat`, case-insensitive compare).
    fn register_string_functions(e: &mut Engine) {
        e.register(STRRCHR, |e, a| {
            let text = read_cstring(e, a[0]);
            match text.rfind(a[1] as u8 as char) {
                Some(i) => ret(a[0] + i as u32),
                None => ret(0),
            }
        });
        e.register(STRCHR, |e, a| {
            let text = read_cstring(e, a[0]);
            match text.find(a[1] as u8 as char) {
                Some(i) => ret(a[0] + i as u32),
                None => ret(0),
            }
        });
        e.register(STRCPY, |e, a| {
            let text = read_cstring(e, a[1]);
            e.mem.write(a[0], text.as_bytes());
            e.mem.set_u8(a[0] + text.len() as u32, 0);
            ret(a[0])
        });
        e.register(STRCAT, |e, a| {
            let (head, tail) = (read_cstring(e, a[0]), read_cstring(e, a[1]));
            e.mem.write(a[0] + head.len() as u32, tail.as_bytes());
            e.mem.set_u8(a[0] + (head.len() + tail.len()) as u32, 0);
            ret(a[0])
        });
        e.register(STRICMP, |e, a| {
            let (x, y) = (read_cstring(e, a[0]), read_cstring(e, a[1]));
            ret((x.to_lowercase() != y.to_lowercase()) as u32)
        });
    }

    /// A single-sequence container holding `sequence`.
    fn single(e: &mut Engine, sequence: u32) -> Ptr<AnimSequenceSingle> {
        let this: Ptr<AnimSequenceSingle> = e.new_object();
        e.set(this, AnimSequenceSingle::pSeq, Ptr::new(sequence));
        this
    }

    /// A multiple-sequence container holding `items`.
    fn multiple(e: &mut Engine, items: &[u32]) -> (Ptr<AnimSequenceMultiple>, u32) {
        let list = make_list(e, items);
        let this: Ptr<AnimSequenceMultiple> = e.new_object();
        e.set(this, AnimSequenceMultiple::pSeqList, Ptr::new(list));
        (this, list)
    }

    // ---- AnimSequenceSingle ------------------------------------------------------------

    #[test]
    fn single_destructor_unloads_the_model_and_runs_the_base_destructor() {
        let mut e = engine();
        let sequence = named_object(&mut e, "Meshes\\a.kf");
        let this = single(&mut e, sequence);
        e.register(STRCPY_S, |_, _| Ret::default());
        e.register(MODEL_LOADER_REMOVE, |_, _| Ret::default());
        e.register(ANIM_SEQUENCE_BASE_DESTRUCT, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0048_ee70, &args![this]);
        let log = peek_log(&e);
        assert_eq!(
            called(&mut e),
            [
                OBJECT_NAME,
                FIXED_STRING_CSTR,
                STRCPY_S,
                REF_RELEASE,
                MODEL_LOADER_REMOVE,
                ANIM_SEQUENCE_BASE_DESTRUCT
            ]
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ANIM_SEQUENCE_SINGLE);
        // The copy buffer is 0x104 bytes and the loader's name is that copy.
        let copy = arguments_of(&log, STRCPY_S)[0].clone();
        assert_eq!(copy[1], 0x104);
        assert_eq!(copy[2], e.mem.u32(sequence + 8));
        let remove = arguments_of(&log, MODEL_LOADER_REMOVE)[0].clone();
        assert_eq!(remove, [0x2222_0000, copy[0], 1]);
    }

    #[test]
    fn single_destructor_without_a_sequence_only_runs_the_base_destructor() {
        let mut e = engine();
        let this = single(&mut e, 0);
        e.register(ANIM_SEQUENCE_BASE_DESTRUCT, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0048_ee70, &args![this]);
        assert_eq!(called(&mut e), [ANIM_SEQUENCE_BASE_DESTRUCT]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ANIM_SEQUENCE_SINGLE);
    }

    #[test]
    fn base_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = engine();
        let this = Ptr::<()>::new(e.mem.alloc(8));
        e.register(ANIM_SEQUENCE_BASE_DESTRUCT, |_, _| Ret::default());
        start_log(&mut e);
        assert_eq!(e.call(0x0048_ef60, &args![this, 0u32]).ptr::<()>(), this);
        assert_eq!(called(&mut e), [ANIM_SEQUENCE_BASE_DESTRUCT]);
        start_log(&mut e);
        e.call(0x0048_ef60, &args![this, 1u32]);
        assert_eq!(
            called(&mut e),
            [ANIM_SEQUENCE_BASE_DESTRUCT, OPERATOR_DELETE]
        );
    }

    #[test]
    fn single_get_sequence_returns_the_held_sequence() {
        let mut e = engine();
        let this = single(&mut e, 0x1234);
        assert_eq!(e.call(0x0048_ef90, &args![this, 7u32]).u32(), 0x1234);
    }

    #[test]
    fn single_get_sequence_index_is_always_ff() {
        let mut e = engine();
        let this = single(&mut e, 0x1234);
        assert_eq!(e.call(0x0048_efb0, &args![this, 0x1234u32]).u8(), 0xff);
    }

    #[test]
    fn single_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = engine();
        let this = single(&mut e, 0);
        e.register(ANIM_SEQUENCE_BASE_DESTRUCT, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0048_efc0, &args![this, 0u32]);
        assert_eq!(called(&mut e), [ANIM_SEQUENCE_BASE_DESTRUCT]);
        start_log(&mut e);
        let back = e.call(0x0048_efc0, &args![this, 1u32]).ptr::<()>();
        assert_eq!(back.addr(), this.addr());
        assert_eq!(
            called(&mut e),
            [ANIM_SEQUENCE_BASE_DESTRUCT, OPERATOR_DELETE]
        );
    }

    #[test]
    fn unload_model_calls_the_loader_with_the_name_and_one() {
        let mut e = engine();
        e.register(MODEL_LOADER_REMOVE, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0048_eff0, &args![0x2222_0000u32, 0x5555u32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, MODEL_LOADER_REMOVE),
            [[0x2222_0000, 0x5555, 1]]
        );
    }

    #[test]
    fn single_set_sequence_replaces_unloads_and_references() {
        let mut e = engine();
        e.register(MODEL_LOADER_REMOVE, |_, _| Ret::default());
        let old = named_object(&mut e, "old.kf");
        let new = named_object(&mut e, "new.kf");
        let this = single(&mut e, old);

        // Free the old model: unload by name, release old, reference new.
        start_log(&mut e);
        e.call(0x0048_f010, &args![this, new, 1u8]);
        let log = peek_log(&e);
        assert_eq!(
            called(&mut e),
            [
                OBJECT_NAME,
                FIXED_STRING_CSTR,
                MODEL_LOADER_REMOVE,
                REF_RELEASE,
                REF_ADD
            ]
        );
        assert_eq!(arguments_of(&log, REF_RELEASE), [[old]]);
        assert_eq!(arguments_of(&log, REF_ADD), [[new]]);
        assert_eq!(e.get(this, AnimSequenceSingle::pSeq).addr(), new);

        // Keep the model loaded: no unload.
        start_log(&mut e);
        e.call(0x0048_f010, &args![this, 0u32, 0u8]);
        assert_eq!(called(&mut e), [REF_RELEASE]);
        assert!(e.get(this, AnimSequenceSingle::pSeq).is_null());

        // Nothing held: no release.
        start_log(&mut e);
        e.call(0x0048_f010, &args![this, new, 1u8]);
        assert_eq!(called(&mut e), [REF_ADD]);
    }

    #[test]
    fn single_remove_sequence_clears_only_the_held_sequence() {
        let mut e = engine();
        let this = object_with_vtable(&mut e, 8, &[(4, 0x00f0_0001)]);
        e.mem.set_u32(this + 4, 0x4000);
        e.register(0x00f0_0001, |_, _| Ret::default());
        start_log(&mut e);
        assert!(!e.call(0x0048_f080, &args![this, 0x9999u32, 1u8]).bool());
        assert!(called(&mut e).is_empty());
        start_log(&mut e);
        assert!(e.call(0x0048_f080, &args![this, 0x4000u32, 1u8]).bool());
        let log = end_log(&mut e);
        // `SetSequence(0, free_model)` through slot +4.
        assert_eq!(arguments_of(&log, 0x00f0_0001), [[this, 0, 1]]);
    }

    // ---- AnimSequenceMultiple ------------------------------------------------------------

    #[test]
    fn multiple_constructor_moves_the_sequence_and_deletes_the_other() {
        let mut e = engine();
        let other = object_with_vtable(
            &mut e,
            8,
            &[(0, 0x00f0_0000), (4, 0x00f0_0001), (0x10, 0x00f0_0010)],
        );
        e.register(0x00f0_0000, |_, _| Ret::default());
        e.register(0x00f0_0001, |_, _| Ret::default());
        e.register(0x00f0_0010, |_, _| ret(0x7777));
        e.register(LIST_CONSTRUCT, |_, _| Ret::default());
        e.register(LIST_ADD_HEAD, |_, _| Ret::default());
        let this: Ptr<AnimSequenceMultiple> = e.new_object();
        start_log(&mut e);
        let back = e.call(0x0048_f0c0, &args![this, other]).ptr::<()>();
        let log = end_log(&mut e);
        assert_eq!(back.addr(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ANIM_SEQUENCE_MULTIPLE);
        let list = e.get(this, AnimSequenceMultiple::pSeqList);
        assert!(!list.is_null());
        // GetSequenceToPlay(-1) (+0x10 pops one word; the pushed 1 is SetSequence's),
        // SetSequence(0, 0), delete(1) on `other`.
        assert_eq!(arguments_of(&log, 0x00f0_0010), [[other, 0xffff_ffff]]);
        assert_eq!(arguments_of(&log, 0x00f0_0001), [[other, 0, 0]]);
        assert_eq!(arguments_of(&log, 0x00f0_0000), [[other, 1]]);
        assert_eq!(arguments_of(&log, REF_ADD), [[0x7777]]);
        assert_eq!(arguments_of(&log, LIST_ADD_HEAD)[0][0], list.addr());
    }

    #[test]
    fn base_constructor_sets_the_base_vtable() {
        let mut e = engine();
        let this = Ptr::<()>::new(e.mem.alloc(8));
        assert_eq!(e.call(0x0048_f1b0, &args![this]).ptr::<()>(), this);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ANIM_SEQUENCE_BASE);
    }

    #[test]
    fn multiple_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = engine();
        let (this, _) = multiple(&mut e, &[]);
        e.register(LIST_HEAD, |_, _| ret(0));
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.register(ANIM_SEQUENCE_BASE_DESTRUCT, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0048_f1d0, &args![this, 0u32]);
        // The destructor deletes the (empty) list with flags 1, never the
        // object itself.
        assert!(!arguments_of(&end_log(&mut e), OPERATOR_DELETE)
            .iter()
            .any(|words| words[0] == this.addr()));
        let (this, _) = multiple(&mut e, &[]);
        start_log(&mut e);
        e.call(0x0048_f1d0, &args![this, 1u32]);
        let log = end_log(&mut e);
        assert_eq!(log.last().unwrap(), &(OPERATOR_DELETE, vec![this.addr()]));
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ANIM_SEQUENCE_MULTIPLE);
    }

    #[test]
    fn list_constructor_forwards_and_returns_this() {
        let mut e = engine();
        e.register(LIST_CONSTRUCT, |_, _| Ret::default());
        start_log(&mut e);
        let this = Ptr::<()>::new(0x3000_0000);
        assert_eq!(e.call(0x0048_f200, &args![this]).ptr::<()>(), this);
        assert_eq!(called(&mut e), [LIST_CONSTRUCT]);
    }

    #[test]
    fn multiple_destructor_unloads_every_sequence_then_deletes_the_list() {
        let mut e = engine();
        e.register(STRCPY_S, |_, _| Ret::default());
        e.register(MODEL_LOADER_REMOVE, |_, _| Ret::default());
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.register(ANIM_SEQUENCE_BASE_DESTRUCT, |_, _| Ret::default());
        let first = named_object(&mut e, "a.kf");
        let second = named_object(&mut e, "b.kf");
        let (this, list) = multiple(&mut e, &[first, second]);
        start_log(&mut e);
        e.call(0x0048_f220, &args![this]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, REF_RELEASE), [[first], [second]]);
        assert_eq!(arguments_of(&log, MODEL_LOADER_REMOVE).len(), 2);
        // The list is destroyed and freed (flags 1), then the base runs.
        assert_eq!(arguments_of(&log, LIST_DESTRUCT), [[list]]);
        assert_eq!(arguments_of(&log, OPERATOR_DELETE), [[list]]);
        assert_eq!(log.last().unwrap().0, ANIM_SEQUENCE_BASE_DESTRUCT);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ANIM_SEQUENCE_MULTIPLE);
    }

    #[test]
    fn multiple_destructor_without_a_list_only_runs_the_base_destructor() {
        let mut e = engine();
        e.register(ANIM_SEQUENCE_BASE_DESTRUCT, |_, _| Ret::default());
        let this: Ptr<AnimSequenceMultiple> = e.new_object();
        start_log(&mut e);
        e.call(0x0048_f220, &args![this]);
        assert_eq!(called(&mut e), [ANIM_SEQUENCE_BASE_DESTRUCT]);
    }

    #[test]
    fn list_deleting_destructor_frees_only_when_asked() {
        let mut e = engine();
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        let this = Ptr::<()>::new(0x3000_0000);
        start_log(&mut e);
        assert_eq!(e.call(0x0048_f370, &args![this, 0u32]).ptr::<()>(), this);
        assert_eq!(called(&mut e), [LIST_DESTRUCT]);
        start_log(&mut e);
        e.call(0x0048_f370, &args![this, 1u32]);
        assert_eq!(called(&mut e), [LIST_DESTRUCT, OPERATOR_DELETE]);
    }

    #[test]
    fn multiple_set_sequence_references_and_adds_at_the_head() {
        let mut e = engine();
        let (this, list) = multiple(&mut e, &[]);
        let seen = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let seen_in_double = seen.clone();
        e.register_double(LIST_ADD_HEAD, move |e, a| {
            // The item is passed by address of a cell holding the sequence.
            seen_in_double.set(e.mem.u32(a[1]));
            Ret::default()
        });
        start_log(&mut e);
        e.call(0x0048_f3a0, &args![this, 0x4242u32, 0u32]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, REF_ADD), [[0x4242]]);
        assert_eq!(arguments_of(&log, LIST_ADD_HEAD)[0][0], list);
        assert_eq!(seen.get(), 0x4242);
    }

    #[test]
    fn multiple_remove_sequence_deletes_the_list_when_it_empties() {
        let mut e = engine();
        e.register(LIST_REMOVE, |_, _| Ret::default());
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.register(LIST_IS_EMPTY, |_, _| ret(0));
        let (this, list) = multiple(&mut e, &[1]);
        start_log(&mut e);
        assert!(!e.call(0x0048_f3d0, &args![this, 0x4242u32, 0u32]).bool());
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, REF_RELEASE), [[0x4242]]);
        assert_eq!(e.get(this, AnimSequenceMultiple::pSeqList).addr(), list);
        assert!(arguments_of(&log, LIST_DESTRUCT).is_empty());

        e.register(LIST_IS_EMPTY, |_, _| ret(1));
        start_log(&mut e);
        assert!(e.call(0x0048_f3d0, &args![this, 0x4242u32, 0u32]).bool());
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, LIST_DESTRUCT), [[list]]);
        assert_eq!(arguments_of(&log, OPERATOR_DELETE), [[list]]);
        assert!(e.get(this, AnimSequenceMultiple::pSeqList).is_null());
    }

    #[test]
    fn multiple_get_sequence_to_play_by_index_and_random() {
        let mut e = engine();
        e.register(RANDOM, |_, _| ret(7));
        let (this, _) = multiple(&mut e, &[0xa, 0xb, 0xc]);
        // An index inside the list is used as is.
        assert_eq!(e.call(0x0048_f450, &args![this, 2i8]).u32(), 0xc);
        // -1 picks `random % count` = 7 % 3 = 1.
        assert_eq!(e.call(0x0048_f450, &args![this, -1i8]).u32(), 0xb);
        // An index past the end also picks at random.
        assert_eq!(e.call(0x0048_f450, &args![this, 5i8]).u32(), 0xb);
    }

    #[test]
    fn multiple_get_corresponding_sequence_matches_file_names() {
        let mut e = engine();
        register_string_functions(&mut e);
        let wanted = named_object(&mut e, "Meshes\\Chars\\Idle.kf");
        let other = named_object(&mut e, "Meshes\\Chars\\Walk.kf");
        let matching = named_object(&mut e, "meshes\\other\\IDLE.KF");
        let (this, _) = multiple(&mut e, &[other, matching]);
        // Same file name, case-insensitively.
        assert_eq!(
            e.call(0x0048_f500, &args![this, wanted, 0u32]).u32(),
            matching
        );
        // A name without a backslash matches nothing.
        let bare = named_object(&mut e, "Idle.kf");
        assert_eq!(e.call(0x0048_f500, &args![this, bare, 0u32]).u32(), 0);
        // No sequence with that file name.
        let missing = named_object(&mut e, "x\\Run.kf");
        assert_eq!(e.call(0x0048_f500, &args![this, missing, 0u32]).u32(), 0);
    }

    #[test]
    fn multiple_get_corresponding_sequence_rewrites_the_name_for_kinds_5_and_6() {
        let mut e = engine();
        register_string_functions(&mut e);
        // The suffix strings live at the exe's addresses.
        e.mem.write(SUFFIX_KIND_5, b"XX\0");
        e.mem.write(SUFFIX_KIND_6, b"YY\0");
        let query = named_object(&mut e, "m\\Foo_Bar.kf");
        // Kind 5 rewrites "\\Foo_Bar.kf" to "\\FooXX_Bar.kf".
        let for_five = named_object(&mut e, "m\\fooxx_bar.kf");
        let for_six = named_object(&mut e, "m\\fooyy_bar.kf");
        let (this, _) = multiple(&mut e, &[for_six, for_five]);
        assert_eq!(
            e.call(0x0048_f500, &args![this, query, 5u32]).u32(),
            for_five
        );
        assert_eq!(
            e.call(0x0048_f500, &args![this, query, 6u32]).u32(),
            for_six
        );
        // Without an underscore the suffix is appended.
        let plain = named_object(&mut e, "m\\Foo.kf");
        let appended = named_object(&mut e, "m\\Foo.kfXX");
        let (this, _) = multiple(&mut e, &[appended]);
        assert_eq!(
            e.call(0x0048_f500, &args![this, plain, 5u32]).u32(),
            appended
        );
    }

    #[test]
    fn multiple_get_sequence_index_counts_positions() {
        let mut e = engine();
        let (this, _) = multiple(&mut e, &[0xa, 0xb, 0xc]);
        assert_eq!(e.call(0x0048_f720, &args![this, 0xcu32]).u8(), 2);
        assert_eq!(e.call(0x0048_f720, &args![this, 0xau32]).u8(), 0);
        assert_eq!(e.call(0x0048_f720, &args![this, 0x99u32]).u8(), 0xff);
    }

    #[test]
    fn multiple_get_sequence_finds_the_sequence_by_group() {
        let mut e = engine();
        let first = e.mem.alloc(0x78);
        let second = e.mem.alloc(0x78);
        e.mem.set_u32(first + 0x74, 0x100);
        e.mem.set_u32(second + 0x74, 0x200);
        let (this, _) = multiple(&mut e, &[first, second]);
        assert_eq!(e.call(0x0048_f790, &args![this, 0x200u32]).u32(), second);
        assert_eq!(e.call(0x0048_f790, &args![this, 0x300u32]).u32(), 0);
    }

    #[test]
    fn sequence_group_getter_reads_offset_74() {
        let mut e = engine();
        let sequence = e.mem.alloc(0x78);
        e.mem.set_u32(sequence + 0x74, 0xabcd);
        assert_eq!(e.call(0x0048_f7f0, &args![sequence]).u32(), 0xabcd);
    }

    // ---- Animation ---------------------------------------------------------------------

    /// An `Animation` whose bytes are all `0xaa`, so the constructor's
    /// writes show.
    fn dirty_animation(e: &mut Engine) -> Ptr<Animation> {
        let this: Ptr<Animation> = e.new_object();
        e.mem.write(this.addr(), &[0xaa; 0x13c]);
        this
    }

    #[test]
    fn animation_constructor_sets_every_member() {
        let mut e = engine();
        for address in [NI_POINT3_CONSTRUCT, SIMPLE_LIST_CONSTRUCT, VECTOR_CONSTRUCT] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(MAP_CONSTRUCT, |_, a| ret(a[0]));
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            ret(a[0])
        });
        for (i, word) in [0x1111u32, 0x2222, 0x3333].iter().enumerate() {
            e.mem.set_u32(ZERO_VECTOR + 4 * i as u32, *word);
        }
        let this = dirty_animation(&mut e);
        start_log(&mut e);
        let back = e.call(0x0048_f810, &args![this]).ptr::<Animation>();
        let log = end_log(&mut e);
        assert_eq!(back, this);
        assert_eq!(e.get(this, Animation::m_uFlags), 0);
        assert_eq!(e.get(this, Animation::pAnimRoot), Ptr::NULL);
        assert!(e.get(this, Animation::pAccumRoot).is_null());
        assert!(e.get(this, Animation::pActorRef).is_null());
        assert!(e.get(this, Animation::pLastMovementSequence).is_null());
        for field in [
            Animation::movementDelta.off,
            Animation::AccumRootTranslate.off,
        ] {
            let words: Vec<u32> = (0..3)
                .map(|i| e.mem.u32(this.addr() + field + 4 * i))
                .collect();
            assert_eq!(words, [0x1111, 0x2222, 0x3333]);
        }
        assert_eq!(e.get(this, Animation::m_fLooking), 0.0);
        assert_eq!(e.get(this, Animation::cSkipUpdate), 0xff);
        assert_eq!(e.get(this, Animation::cSkipNextBlend), 0);
        assert_eq!(e.get(this, Animation::time), 0.0);
        assert!(!e.get(this, Animation::bShutDown));
        // 0x10 bytes of 0xff in `group` and `nextGroup`, 0x20 of -1 in the
        // action arrays, zero in the sound bones and current sequences.
        assert_eq!(e.mem.bytes(this.addr() + 0x4c, 0x10), [0xff; 0x10]);
        assert_eq!(e.mem.bytes(this.addr() + 0x9c, 0x10), [0xff; 0x10]);
        for offset in [0x5c, 0x7c, 0xac] {
            assert_eq!(e.mem.bytes(this.addr() + offset, 0x20), [0xff; 0x20]);
        }
        for offset in [0x28, 0xe0] {
            assert_eq!(e.mem.bytes(this.addr() + offset, 0x20), [0; 0x20]);
        }
        for field in [
            Animation::m_fMoveSpeed,
            Animation::m_fAttackSpeed,
            Animation::m_fGlobalTimeMultiplier,
            Animation::m_fReloadModifier,
            Animation::m_fEquipModifier,
        ] {
            assert_eq!(e.get(this, field), 1.0);
        }
        assert_eq!(e.get(this, Animation::sQueuedReloadGroup), 0xff);
        // The sequence map has 0x65 buckets and is built from a 0x10 byte
        // block.
        let map = e.get(this, Animation::pAnimSequenceMap);
        assert!(!map.is_null());
        assert_eq!(arguments_of(&log, MAP_CONSTRUCT), [[map.addr(), 0x65]]);
        assert_eq!(
            arguments_of(&log, VECTOR_CONSTRUCT),
            [[
                this.addr() + 0x12c,
                4,
                2,
                NI_POINTER_DEFAULT_CONSTRUCT,
                NI_POINTER_RELEASE
            ]]
        );
        // The first member constructed is the `pAnimRoot` NiPointer.
        assert_eq!(log[0], (NI_POINTER_INIT, vec![this.addr() + 8, 0]));
    }

    #[test]
    fn flag_bits_are_replaced_under_the_mask() {
        let mut e = engine();
        let this = dirty_animation(&mut e);
        e.set(this, Animation::m_uFlags, 0x0f);
        e.call(0x0048_fb20, &args![this, 3u8, 0xf0u8, 4u8]);
        assert_eq!(e.get(this, Animation::m_uFlags), 0x3f);
        // Value 0 under mask 0xff clears (what the constructor does).
        e.call(0x0048_fb20, &args![this, 0u8, 0xffu8, 0u8]);
        assert_eq!(e.get(this, Animation::m_uFlags), 0);
        // The shift count is taken modulo 32, as `SHL` does.
        e.call(0x0048_fb20, &args![this, 1u8, 0xffu8, 33u8]);
        assert_eq!(e.get(this, Animation::m_uFlags), 2);
    }

    /// Doubles `~Animation` needs besides the engine's own.
    fn register_destructor_callees(e: &mut Engine) {
        for address in [
            SPECIAL_IDLE_FREE,
            MODEL_LOADER_CANCEL_REPLACEMENT_KF_LIST,
            REMOVE_CONTROLLER,
            MANAGER_RESET,
            MAP_REMOVE_ALL,
            SIMPLE_LIST_CLEAR,
            SIMPLE_LIST_DESTRUCT,
            VECTOR_DESTRUCT,
            KF_MODEL_RELEASE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn animation_destructor_tears_everything_down_in_order() {
        let mut e = engine();
        register_destructor_callees(&mut e);
        e.register(LIST_COUNT, |_, _| ret(0));
        e.register(GET_CONTROLLER, |_, a| {
            ret(if a[1] == RTTI_CONTROLLER_FIRST {
                0xc1
            } else {
                0xc2
            })
        });
        let target = object_with_vtable(&mut e, 0x20, &[(0xc, 0x00f0_000c)]);
        e.register(0x00f0_000c, |_, _| ret(1));
        // The manager's target is the object made above.
        e.register_double(MANAGER_TARGET, move |_, _| ret(target));
        // The map has one entry, an object that is deleted through its
        // vtable.
        let entry = object_with_vtable(&mut e, 8, &[(0, 0x00f0_0000)]);
        e.register(0x00f0_0000, |_, _| Ret::default());
        e.register(MAP_FIRST_POSITION, |_, _| ret(0x55));
        e.register_double(MAP_GET_NEXT, move |e, a| {
            // `GetNext(&pos, &key, &value)`: ends the iteration.
            e.mem.set_u32(a[1], 0);
            e.mem.set_u32(a[3], entry);
            Ret::default()
        });
        let map = object_with_vtable(&mut e, 0x10, &[(0, 0x00f0_0001)]);
        e.register(0x00f0_0001, |_, _| Ret::default());

        let this = dirty_animation(&mut e);
        let manager = 0xaaa0u32;
        let root = 0xbbb0u32;
        e.set(this, Animation::spManager, Ptr::new(manager));
        e.set(this, Animation::pAnimRoot, Ptr::new(root));
        e.set(this, Animation::pAnimSequenceMap, Ptr::new(map));
        e.set(this, Animation::pAccumRoot, Ptr::new(0x77));
        // Both "free when inactive" idles are empty.
        e.mem.set_u32(this.addr() + 0x12c, 0);
        e.mem.set_u32(this.addr() + 0x130, 0);
        // One KF model and one replay delay entry.
        let models = make_simple_list(&mut e, &[0x1357]);
        let (item, next) = (e.mem.u32(models), e.mem.u32(models + 4));
        e.mem.set_u32(this.addr() + 0x104, item);
        e.mem.set_u32(this.addr() + 0x108, next);
        let delays = make_simple_list(&mut e, &[0x2468]);
        let (item, next) = (e.mem.u32(delays), e.mem.u32(delays + 4));
        e.mem.set_u32(this.addr() + 0x134, item);
        e.mem.set_u32(this.addr() + 0x138, next);

        start_log(&mut e);
        e.call(0x0048_fb50, &args![this]);
        let log = end_log(&mut e);
        let addresses: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        let position = |address: u32| addresses.iter().position(|a| *a == address).unwrap();

        assert_eq!(addresses[0], SPECIAL_IDLE_FREE);
        assert!(position(SPECIAL_IDLE_FREE) < position(MODEL_LOADER_CANCEL_REPLACEMENT_KF_LIST));
        assert_eq!(
            arguments_of(&log, MODEL_LOADER_CANCEL_REPLACEMENT_KF_LIST),
            [[0x2222_0000, this.addr()]]
        );
        // Both controllers are removed from the root, then the manager
        // detaches from its target.
        assert_eq!(
            arguments_of(&log, GET_CONTROLLER),
            [
                [root, RTTI_CONTROLLER_FIRST],
                [root, RTTI_CONTROLLER_SECOND]
            ]
        );
        assert_eq!(
            arguments_of(&log, REMOVE_CONTROLLER),
            [[root, 0xc1], [root, 0xc2], [target, manager]]
        );
        assert_eq!(arguments_of(&log, MANAGER_RESET), [[manager]]);
        // The map entry, the KF model, the map and the replay entry go.
        assert_eq!(arguments_of(&log, 0x00f0_0000), [[entry, 1]]);
        assert_eq!(arguments_of(&log, KF_MODEL_RELEASE), [[0x1357]]);
        assert_eq!(arguments_of(&log, 0x00f0_0001), [[map, 1]]);
        assert_eq!(arguments_of(&log, OPERATOR_DELETE), [[0x2468]]);
        assert!(e.get(this, Animation::pAnimSequenceMap).is_null());
        assert!(e.get(this, Animation::pAccumRoot).is_null());
        assert!(e.get(this, Animation::spManager).is_null());
        // Members are destroyed last, in reverse order.
        let tail: Vec<(u32, Vec<u32>)> = log[log.len() - 7..].to_vec();
        let a = this.addr();
        assert_eq!(
            tail,
            [
                (SIMPLE_LIST_DESTRUCT, vec![a + 0x134]),
                (VECTOR_DESTRUCT, vec![a + 0x12c, 4, 2, NI_POINTER_RELEASE]),
                (NI_POINTER_RELEASE, vec![a + 0x128]),
                (NI_POINTER_RELEASE, vec![a + 0x124]),
                (SIMPLE_LIST_DESTRUCT, vec![a + 0x104]),
                (NI_POINTER_RELEASE, vec![a + 0xd8]),
                (NI_POINTER_RELEASE, vec![a + 8]),
            ]
        );
    }

    #[test]
    fn animation_destructor_without_a_manager_skips_the_controllers() {
        let mut e = engine();
        register_destructor_callees(&mut e);
        e.register(MAP_FIRST_POSITION, |_, _| ret(0));
        let this = dirty_animation(&mut e);
        e.set(this, Animation::spManager, Ptr::NULL);
        e.set(this, Animation::pAnimSequenceMap, Ptr::NULL);
        e.mem.write(this.addr() + 0x104, &[0; 8]);
        e.mem.write(this.addr() + 0x134, &[0; 8]);
        e.mem.write(this.addr() + 0x12c, &[0; 8]);
        e.register(SIMPLE_LIST_IS_EMPTY, |_, _| ret(1));
        start_log(&mut e);
        e.call(0x0048_fb50, &args![this]);
        let addresses = called(&mut e);
        assert!(!addresses.contains(&GET_CONTROLLER));
        assert!(!addresses.contains(&REMOVE_CONTROLLER));
        assert!(!addresses.contains(&MANAGER_RESET));
        // The empty replay list is neither walked nor cleared a second time.
        assert_eq!(
            addresses
                .iter()
                .filter(|a| **a == SIMPLE_LIST_CLEAR)
                .count(),
            1
        );
    }

    #[test]
    fn deactivate_all_deactivates_each_active_sequence() {
        let mut e = engine();
        let manager = e.mem.alloc(0x7c);
        // The active set at +0x44: elements pointer, allocated, used.
        let elements = e.mem.alloc(8);
        e.mem.set_u32(elements, 0xa1);
        e.mem.set_u32(elements + 4, 0xa2);
        e.mem.set_u32(manager + 0x44, elements);
        e.mem.set_u32(manager + 0x4c, 2);
        e.register(LIST_COUNT, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ACTIVE_SET_ELEMENT, |e, a| ret(e.mem.u32(a[0]) + 4 * a[1]));
        e.register(MANAGER_DEACTIVATE, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0048_fef0, &args![manager, 0.25f32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, MANAGER_DEACTIVATE),
            [
                [manager, 0xa1, 0.25f32.to_bits()],
                [manager, 0xa2, 0.25f32.to_bits()]
            ]
        );
    }

    #[test]
    fn shutdown_all_anim_idles_frees_the_idles_that_are_set() {
        let mut e = engine();
        e.register(SPECIAL_IDLE_FREE, |_, _| Ret::default());
        e.register(ANIM_IDLE_FREE, |_, _| Ret::default());
        let this = dirty_animation(&mut e);
        e.mem.set_u32(this.addr() + 0x12c, 0x1234);
        e.mem.set_u32(this.addr() + 0x130, 0);
        e.set(this, Animation::bShutDown, false);
        start_log(&mut e);
        e.call(0x0048_ff50, &args![this]);
        let log = end_log(&mut e);
        assert!(e.get(this, Animation::bShutDown));
        assert_eq!(arguments_of(&log, SPECIAL_IDLE_FREE), [[this.addr(), 1, 1]]);
        // Only the first idle is freed and cleared.
        assert_eq!(
            arguments_of(&log, ANIM_IDLE_FREE),
            [[this.addr(), this.addr() + 0x12c]]
        );
        assert_eq!(
            arguments_of(&log, NI_POINTER_SET),
            [[this.addr() + 0x12c, 0]]
        );
    }

    /// A `NiTMapBase` double holding one entry for any key at `map + 4`.
    fn register_single_entry_map(e: &mut Engine) {
        e.register(MAP_GET_AT, |e, a| {
            let entry = e.mem.u32(a[0] + 4);
            if entry == 0 {
                return ret(0);
            }
            e.mem.set_u32(a[2], entry);
            ret(1)
        });
        e.register(MAP_SET_AT, |e, a| {
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret::default()
        });
        e.register(MAP_REMOVE_AT, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
    }

    #[test]
    fn setup_refuses_null_arguments_and_an_existing_group_entry() {
        let mut e = engine();
        register_single_entry_map(&mut e);
        let this: Ptr<Animation> = e.new_object();
        let list = make_simple_list(&mut e, &[]);
        let object = e.mem.alloc(0x40);
        start_log(&mut e);
        assert!(!e
            .call(0x0048_ffd0, &args![this, list, 0u32, object, true])
            .bool());
        let log = end_log(&mut e);
        // The scope guard is built and torn down around the refusal.
        assert_eq!(
            arguments_of(&log, SCOPE_GUARD_CTOR)[0][1..],
            [SCOPE_GUARD_TAG, 1, SOURCE_FILE, 0x238]
        );
        assert_eq!(log.last().unwrap().0, SCOPE_GUARD_DTOR);
        assert!(e.get(this, Animation::pActorRef).is_null());

        // An entry for group 0 already in the map refuses too.
        let map = e.mem.alloc(8);
        e.mem.set_u32(map + 4, 0x9999);
        e.set(this, Animation::pAnimSequenceMap, Ptr::new(map));
        assert!(!e
            .call(0x0048_ffd0, &args![this, list, object, object, true])
            .bool());
        assert_eq!(e.get(this, Animation::pActorRef).addr(), object);
        assert!(e.get(this, Animation::spManager).is_null());
    }

    #[test]
    fn setup_creates_the_manager_loads_the_list_and_plays_the_idle() {
        let mut e = engine();
        // The map has no entry until the (stubbed) loading adds one.
        register_single_entry_map(&mut e);
        let map = e.mem.alloc(8);
        let this: Ptr<Animation> = e.new_object();
        e.set(this, Animation::pAnimSequenceMap, Ptr::new(map));
        let root = e.mem.alloc(0x40);
        let actor = e.mem.alloc(0x40);
        let name = cstring(&mut e, "Meshes\\a.kf");
        let list = make_simple_list(&mut e, &[name]);

        // The reference's form type is not used; the bone table has one
        // name.
        e.register(REFERENCE_FORM, |_, _| ret(1));
        e.register(FORM_TYPE, |_, _| ret(FORM_TYPE_COMPARED));
        e.mem.set_u32(SOUND_PRIORITY_BONE_NAMES + 4, 0x4040);
        e.register(NODE_FIND_OBJECT, |_, a| ret(a[1] + 1));
        e.register(NODE_SET_FLAG, |_, _| Ret::default());
        e.register(MANAGER_CONSTRUCT, |_, a| ret(a[0] + 0x100));
        // The loader returns a KF without a group, so nothing is added; the
        // entry for the idle is put in the map by hand once loading is done.
        e.register(MODEL_LOADER_LOAD_KF, |_, _| ret(0x6000));
        e.register(KF_MODEL_ANIM_GROUP, |_, _| ret(0));
        e.register(SIMPLE_LIST_DELETE, |_, _| Ret::default());
        e.register(ACCUM_ROOT_SETUP, |_, _| Ret::default());
        let sequence = e.mem.alloc(0x80);
        let entry = object_with_vtable(&mut e, 8, &[(0x10, 0x00f0_0010)]);
        e.register_double(0x00f0_0010, move |_, _| ret(sequence));
        e.register(SEQUENCE_ACTIVATE, |_, _| Ret::default());
        e.register(UPDATE_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(NODE_UPDATE, |_, _| Ret::default());
        e.register(0x00f0_008c, |_, _| Ret::default());
        // Loading removes nothing from the map; the pop-front double has
        // emptied the list by then and we install the entry now.
        e.register_double(SIMPLE_LIST_POP_FRONT, move |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            e.mem.set_u32(map + 4, entry);
            Ret::default()
        });
        // `sequence` needs a vtable for the final virtual call.
        let sequence_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(sequence, sequence_vtable);
        e.mem.set_u32(sequence_vtable + 0x8c, 0x00f0_008c);
        e.set(this, Animation::pAccumRoot, Ptr::new(0x5151));

        start_log(&mut e);
        let result = e
            .call(0x0048_ffd0, &args![this, list, root, actor, true])
            .bool();
        let log = end_log(&mut e);
        assert!(result);
        assert_eq!(e.get(this, Animation::pActorRef).addr(), actor);
        assert_eq!(e.get(this, Animation::pAnimRoot).addr(), root);
        // The bone found for the one named entry is stored at its index.
        assert_eq!(e.mem.u32(this.addr() + 0x28 + 4), 0x4041);
        assert_eq!(arguments_of(&log, NODE_FIND_OBJECT), [[root, 0x4040, 1]]);
        // The root's flag is set to 0, 2, and the manager is made for the
        // root and kept.
        assert_eq!(arguments_of(&log, NODE_SET_FLAG), [[root, 0, 2]]);
        let manager = e.get(this, Animation::spManager).addr();
        assert_ne!(manager, 0);
        assert_eq!(arguments_of(&log, MANAGER_CONSTRUCT)[0][1..], [root, 1]);
        // The listed file was loaded, passed to `AddAnimation` (its group
        // is null so nothing more happens), freed and removed from the list.
        assert_eq!(
            arguments_of(&log, MODEL_LOADER_LOAD_KF),
            [[0x2222_0000, name]]
        );
        assert_eq!(arguments_of(&log, OPERATOR_DELETE), [[name]]);
        assert_eq!(arguments_of(&log, SIMPLE_LIST_DELETE), [[list, 1]]);
        // The accumulation root is set up, then the idle sequence is
        // activated and the root's translation zeroed.
        assert_eq!(
            arguments_of(&log, ACCUM_ROOT_SETUP),
            [[0x5151, ACCUM_ROOT_SETUP_ARGUMENT]]
        );
        assert_eq!(
            arguments_of(&log, SEQUENCE_ACTIVATE),
            [[sequence, 0x64, 1, 1.0f32.to_bits(), 0.0f32.to_bits(), 0, 0]]
        );
        assert_eq!(
            arguments_of(&log, UPDATE_DATA_CONSTRUCT)[0][1..],
            [0.0f32.to_bits(), 1, 0]
        );
        assert_eq!(arguments_of(&log, NODE_UPDATE)[0][0], root);
        assert_eq!(
            arguments_of(&log, 0x00f0_008c),
            [[sequence, 0.0f32.to_bits(), 0]]
        );
    }

    #[test]
    fn root_flag_setter_passes_the_flag_and_two() {
        let mut e = engine();
        e.register(NODE_SET_FLAG, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0049_02f0, &args![0x3000u32, 1u8]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, NODE_SET_FLAG), [[0x3000, 1, 2]]);
    }

    #[test]
    fn find_object_searches_the_root_by_name_with_a_one() {
        let mut e = engine();
        e.register(NODE_FIND_OBJECT, |_, a| ret(a[0] + a[1] + a[2]));
        start_log(&mut e);
        assert_eq!(
            e.call(0x0049_0310, &args![0x1u32, 0x100u32, 0x20u32]).u32(),
            0x121
        );
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, NODE_FIND_OBJECT), [[0x100, 0x20, 1]]);
    }

    /// Doubles that make `AddAnimation` do nothing but report that the
    /// keyframe has no group.
    fn register_kf_without_group(e: &mut Engine) {
        e.register(MODEL_LOADER_LOAD_KF, |_, a| ret(a[1] + 0x1000));
        e.register(KF_MODEL_ANIM_GROUP, |_, _| ret(0));
    }

    #[test]
    fn special_animations_are_loaded_from_the_special_folder() {
        let mut e = engine();
        register_kf_without_group(&mut e);
        e.register(SPRINTF_S, |_, _| Ret::default());
        let this: Ptr<Animation> = e.new_object();
        let (first, second) = (cstring(&mut e, "a.kf"), cstring(&mut e, "b.kf"));
        let list = make_simple_list(&mut e, &[first, second]);
        let directory = cstring(&mut e, "Meshes");
        start_log(&mut e);
        e.call(0x0049_0330, &args![this, list, directory]);
        let log = end_log(&mut e);
        let formats = arguments_of(&log, SPRINTF_S);
        assert_eq!(formats.len(), 2);
        for (words, name) in formats.iter().zip([first, second]) {
            assert_eq!(
                words[1..],
                [
                    0x104,
                    SPECIAL_ANIMS_FORMAT,
                    directory,
                    SPECIAL_ANIMS_FOLDER,
                    name
                ]
            );
        }
        // The loader gets the formatted path, and every KF is handed on.
        assert_eq!(arguments_of(&log, MODEL_LOADER_LOAD_KF).len(), 2);
        assert_eq!(arguments_of(&log, KF_MODEL_ANIM_GROUP).len(), 2);
        // A null list does nothing.
        start_log(&mut e);
        e.call(0x0049_0330, &args![this, 0u32, directory]);
        assert!(called(&mut e).is_empty());
    }

    #[test]
    fn animations_from_a_list_are_loaded_freed_and_the_list_deleted() {
        let mut e = engine();
        register_kf_without_group(&mut e);
        e.register(SIMPLE_LIST_DELETE, |_, _| Ret::default());
        let this: Ptr<Animation> = e.new_object();
        let (first, second) = (cstring(&mut e, "a.kf"), cstring(&mut e, "b.kf"));
        let list = make_simple_list(&mut e, &[first, second]);
        start_log(&mut e);
        e.call(0x0049_0400, &args![this, list]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, MODEL_LOADER_LOAD_KF),
            [[0x2222_0000, first], [0x2222_0000, second]]
        );
        assert_eq!(arguments_of(&log, OPERATOR_DELETE), [[first], [second]]);
        assert_eq!(arguments_of(&log, SIMPLE_LIST_DELETE), [[list, 1]]);
        assert_eq!(
            arguments_of(&log, SCOPE_GUARD_CTOR)[0][1..],
            [SCOPE_GUARD_TAG, 1, SOURCE_FILE, 0x2ee]
        );
        assert_eq!(log.last().unwrap().0, SCOPE_GUARD_DTOR);
        // A null list does nothing.
        start_log(&mut e);
        e.call(0x0049_0400, &args![this, 0u32]);
        assert!(called(&mut e).is_empty());
    }

    // ---- AddAnimation ------------------------------------------------------------------

    /// The `AnimSequenceSingle` and `AnimSequenceMultiple` vtables of the
    /// exe, written into memory, and doubles for the folded getters in
    /// them and for the destructor/list pieces they use.
    fn install_sequence_vtables(e: &mut Engine) {
        let single = [
            0x0048_efc0,
            0x0048_f010,
            0x0048_f080,
            0x008d_0360,
            0x0048_ef90,
            0x0048_ef90,
            0x0048_efb0,
        ];
        let multiple = [
            0x0048_f1d0,
            0x0048_f3a0,
            0x0048_f3d0,
            0x0047_c850,
            0x0048_f450,
            0x0048_f790,
            0x0048_f720,
        ];
        for (i, target) in single.iter().enumerate() {
            e.mem
                .set_u32(VTABLE_ANIM_SEQUENCE_SINGLE + 4 * i as u32, *target);
        }
        for (i, target) in multiple.iter().enumerate() {
            e.mem
                .set_u32(VTABLE_ANIM_SEQUENCE_MULTIPLE + 4 * i as u32, *target);
        }
        // `IsSingleSeq`: true for the single, false for the multiple.
        e.register(0x008d_0360, |_, _| ret(1));
        e.register(0x0047_c850, |_, _| ret(0));
        e.register(ANIM_SEQUENCE_BASE_DESTRUCT, |_, _| Ret::default());
        e.register(LIST_CONSTRUCT, |_, _| Ret::default());
        e.register(LIST_ADD_HEAD, |_, _| Ret::default());
    }

    /// Everything `AddAnimation` touches, with doubles that behave like the
    /// game for a one-entry sequence map.
    struct Fixture {
        e: Engine,
        this: Ptr<Animation>,
        kf: u32,
        sequence: u32,
        group: u32,
        map: u32,
        manager: u32,
        root: u32,
    }

    /// A keyframe model `kf` (`+4` the sequence, `+8` the group) of the
    /// given sequence type, and an animation with a manager and a root.
    fn add_fixture(sequence_type: u32) -> Fixture {
        let mut e = engine();
        install_sequence_vtables(&mut e);
        register_single_entry_map(&mut e);
        // The group: id at +0x10 (7), sequence type at +0x20 (the doubles
        // read it there).
        let group = e.mem.alloc(0x80);
        e.mem.set_u16(group + 0x10, 7);
        e.mem.set_u32(group + 0x20, sequence_type);
        // The sequence has one reference, a name and the group.
        let sequence = named_object(&mut e, "seq.kf");
        e.mem.set_u32(sequence + 4, 1);
        e.mem.set_u32(sequence + 0x74, group);
        let kf = e.mem.alloc(0x20);
        let file = cstring(&mut e, "Meshes\\file.kf");
        e.mem.set_u32(kf, file);
        e.mem.set_u32(kf + 4, sequence);
        e.mem.set_u32(kf + 8, group);
        e.register(KF_MODEL_ANIM_GROUP, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(KF_MODEL_SEQUENCE, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(ANIM_GROUP_SEQUENCE_TYPE, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(ANIM_GROUP_ID, |e, a| ret(e.mem.u32(a[0] + 0x10) & 0xffff));
        e.register(IS_MENU_ID_VISIBLE, |_, _| ret(0));
        e.register(IS_KIND_OF, |_, _| ret(1));
        e.register(GROUP_ID_TYPE, |_, a| ret(a[0] & 0xff));
        e.register(GROUP_SPEED, |_, _| 1.0f32.into_ret());
        // The constants `AddGroup` compares with (a zero double).
        e.map(0x0101_2000, 0x1000);
        e.register(MODEL_LOADER_REMOVE, |_, _| Ret::default());
        e.register(SEQUENCE_CYCLE_TYPE, |_, _| ret(0));
        e.register(MANAGER_ADD_SEQUENCE, |_, _| ret(1));
        e.register(MANAGER_REMOVE_SEQUENCE, |_, _| Ret::default());
        e.register(KF_MODEL_ADD_REF, |_, _| Ret::default());
        e.register(SIMPLE_LIST_PUSH_BACK, |_, _| Ret::default());
        e.register(LOG, |_, _| Ret::default());
        e.register(DISABLE_WARNINGS, |_, _| Ret::default());
        e.register(SEQUENCE_OBJECT_COUNT, |_, _| ret(0));
        // `NiRefObject` count read (`+4`) through the node accessor.
        e.register(SIMPLE_LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));

        let this: Ptr<Animation> = e.new_object();
        let map = e.mem.alloc(8);
        let manager = e.mem.alloc(0x7c);
        let root = object_with_vtable(&mut e, 0x40, &[(0x9c, 0x00f0_009c)]);
        e.register(0x00f0_009c, |_, _| ret(0));
        e.set(this, Animation::pAnimSequenceMap, Ptr::new(map));
        e.mem.set_u32(this.addr() + 0xd8, manager);
        e.mem.set_u32(this.addr() + 8, root);
        // A cumulative-less manager has no accumulation root, so set one.
        e.set(this, Animation::pAccumRoot, Ptr::new(0x5151));
        Fixture {
            e,
            this,
            kf,
            sequence,
            group,
            map,
            manager,
            root,
        }
    }

    impl Fixture {
        fn add(&mut self, queue_cloning: bool) -> bool {
            let (this, kf) = (self.this, self.kf);
            self.e
                .call(0x0049_0500, &args![this, kf, queue_cloning])
                .bool()
        }

        fn entry(&self) -> u32 {
            self.e.mem.u32(self.map + 4)
        }
    }

    #[test]
    fn add_animation_without_a_group_does_nothing() {
        let mut f = add_fixture(5);
        f.e.mem.set_u32(f.kf + 8, 0);
        start_log(&mut f.e);
        assert!(!f.add(false));
        let addresses = called(&mut f.e);
        // The group is held and released, and nothing else happens (no
        // scope guard either).
        assert_eq!(
            addresses,
            [
                KF_MODEL_ANIM_GROUP,
                NI_POINTER_INIT,
                READ_WORD,
                NI_POINTER_RELEASE
            ]
        );
    }

    #[test]
    fn add_animation_reports_a_group_without_a_sequence_type() {
        let mut f = add_fixture(0xff);
        start_log(&mut f.e);
        assert!(!f.add(false));
        let log = end_log(&mut f.e);
        let file = f.e.mem.u32(f.kf);
        let name = f.e.mem.u32(f.sequence + 8);
        assert_eq!(
            arguments_of(&log, LOG),
            [[LOG_SEQUENCE_TYPE_NOT_FOUND, name, file]]
        );
        assert!(arguments_of(&log, SCOPE_GUARD_CTOR).is_empty());
        assert_eq!(f.entry(), 0);
    }

    #[test]
    fn add_animation_queues_the_model_while_the_menu_allows_it() {
        let mut f = add_fixture(5);
        start_log(&mut f.e);
        assert!(f.add(true));
        let log = end_log(&mut f.e);
        assert_eq!(arguments_of(&log, KF_MODEL_ADD_REF), [[f.kf]]);
        let push = arguments_of(&log, SIMPLE_LIST_PUSH_BACK);
        assert_eq!(push[0][0], f.this.addr() + 0x104);
        // A fresh single-sequence entry was made for the group (key 7).
        assert_ne!(f.entry(), 0);
        assert_eq!(f.e.mem.u32(f.entry()), VTABLE_ANIM_SEQUENCE_SINGLE);
        assert_eq!(arguments_of(&log, MAP_SET_AT)[0][1], 7);
        // Nothing was added to the manager.
        assert!(arguments_of(&log, MANAGER_ADD_SEQUENCE).is_empty());
        assert_eq!(
            arguments_of(&log, SCOPE_GUARD_CTOR)[0][1..],
            [SCOPE_GUARD_TAG, 1, SOURCE_FILE, 0x31a]
        );
        assert_eq!(log.last().unwrap().0, NI_POINTER_RELEASE);

        // Types 0, 0x17 and 0xe0, or a visible menu, are added right away.
        for sequence_type in [0u32, 0x17, 0xe0] {
            let mut f = add_fixture(sequence_type);
            start_log(&mut f.e);
            assert!(f.add(true));
            let log = end_log(&mut f.e);
            assert!(arguments_of(&log, KF_MODEL_ADD_REF).is_empty());
            assert_eq!(arguments_of(&log, MANAGER_ADD_SEQUENCE).len(), 1);
        }
        let mut f = add_fixture(5);
        f.e.register(IS_MENU_ID_VISIBLE, |_, a| {
            ret((a[0] == MENU_ID_PIPBOY_WAIT) as u32)
        });
        start_log(&mut f.e);
        assert!(f.add(true));
        let log = end_log(&mut f.e);
        assert!(arguments_of(&log, KF_MODEL_ADD_REF).is_empty());
        assert_eq!(arguments_of(&log, MANAGER_ADD_SEQUENCE).len(), 1);
    }

    #[test]
    fn add_animation_adds_the_sequence_to_the_manager() {
        let mut f = add_fixture(5);
        start_log(&mut f.e);
        assert!(f.add(false));
        let log = end_log(&mut f.e);
        // The sequence has one reference and is a `BSAnimGroupSequence`, so
        // it is used as is, stored in the entry and added to the manager.
        assert!(arguments_of(&log, ANIM_GROUP_SEQUENCE_CONSTRUCT).is_empty());
        assert_eq!(
            f.e.get(
                Ptr::<AnimSequenceSingle>::new(f.entry()),
                AnimSequenceSingle::pSeq
            )
            .addr(),
            f.sequence
        );
        assert_eq!(
            arguments_of(&log, MANAGER_ADD_SEQUENCE),
            [[f.manager, f.sequence, ACCUM_ROOT_NAME, 1]]
        );
        // `AddGroup` (translated in this file) runs for the sequence's group:
        // it asks for the group id, and the non-cumulative manager has no
        // accumulation root to find.
        assert_eq!(
            arguments_of(&log, ANIM_GROUP_ID).last(),
            Some(&vec![f.group])
        );
        assert_eq!(
            arguments_of(&log, LOG),
            [[LOG_NO_ACCUM_ROOT, f.e.mem.u32(f.root + 8)]]
        );
        assert_eq!(f.e.get(f.this, Animation::pAccumRoot).addr(), 0x5151);

        // A sequence with other references is wrapped in a new
        // `BSAnimGroupSequence` (0x78 bytes).
        let mut f = add_fixture(5);
        f.e.mem.set_u32(f.sequence + 4, 2);
        f.e.register(ANIM_GROUP_SEQUENCE_CONSTRUCT, |_, a| ret(a[0]));
        start_log(&mut f.e);
        assert!(f.add(false));
        let log = end_log(&mut f.e);
        let wrapped = arguments_of(&log, ANIM_GROUP_SEQUENCE_CONSTRUCT);
        assert_eq!(wrapped[0][1..], [f.group, f.sequence]);
        assert_eq!(
            arguments_of(&log, MANAGER_ADD_SEQUENCE)[0][1],
            wrapped[0][0]
        );
    }

    #[test]
    fn add_animation_registers_the_accumulation_root_the_first_time() {
        let mut f = add_fixture(5);
        f.e.set(f.this, Animation::pAccumRoot, Ptr::NULL);
        // A manager that is not cumulative has no accumulation root.
        assert!(f.add(false));
        assert!(f.e.get(f.this, Animation::pAccumRoot).is_null());

        // A cumulative one reports the root of its first sequence.
        let mut f = add_fixture(5);
        f.e.set(f.this, Animation::pAccumRoot, Ptr::NULL);
        let slots = f.e.mem.alloc(4);
        f.e.mem.set_u32(slots, 0xd00d);
        f.e.mem.set_u32(f.manager + 0x34, slots);
        f.e.mem.set_u32(f.manager + 0x38, 1);
        f.e.mem.set_u8(f.manager + 0x68, 1);
        f.e.register(SEQUENCE_ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0] + 4)));
        f.e.register(SEQUENCE_ARRAY_ELEMENT, |e, a| {
            ret(e.mem.u32(a[0]) + 4 * a[1])
        });
        f.e.register(ACCUM_ROOT_OF_SEQUENCE, |_, a| ret(a[0] + 1));
        assert!(f.add(false));
        assert_eq!(f.e.get(f.this, Animation::pAccumRoot).addr(), 0xd00e);
    }

    #[test]
    fn add_animation_refuses_a_clamped_sequence_for_the_cycling_types() {
        let mut f = add_fixture(5);
        f.e.register(SEQUENCE_CYCLE_TYPE, |_, _| ret(2));
        f.e.register(ANIM_SEQUENCE_BASE_DESTRUCT, |_, _| Ret::default());
        start_log(&mut f.e);
        assert!(!f.add(false));
        let log = end_log(&mut f.e);
        // The entry that was made for the group is removed again and
        // deleted; the manager never sees the sequence.
        assert_eq!(f.entry(), 0);
        assert_eq!(arguments_of(&log, MAP_REMOVE_AT), [[f.map, 7]]);
        assert!(arguments_of(&log, MANAGER_ADD_SEQUENCE).is_empty());
        assert_eq!(log.last().unwrap().0, NI_POINTER_RELEASE);

        // Type 2 is outside the range 3..=0x10: the cycle type does not
        // matter.
        let mut f = add_fixture(2);
        f.e.register(SEQUENCE_CYCLE_TYPE, |_, _| ret(2));
        assert!(f.add(false));
    }

    #[test]
    fn add_animation_returns_true_when_the_group_is_already_there() {
        let mut f = add_fixture(5);
        // An entry already holds a sequence of the same group.
        let held = f.e.mem.alloc(0x80);
        f.e.mem.set_u32(held + 0x74, f.group);
        let entry = single(&mut f.e, held);
        f.e.mem.set_u32(entry.addr(), VTABLE_ANIM_SEQUENCE_SINGLE);
        f.e.mem.set_u32(f.map + 4, entry.addr());
        start_log(&mut f.e);
        assert!(f.add(false));
        let addresses = called(&mut f.e);
        assert!(!addresses.contains(&MANAGER_ADD_SEQUENCE));
        assert!(!addresses.contains(&MANAGER_REMOVE_SEQUENCE));
        assert_eq!(addresses[addresses.len() - 2], SCOPE_GUARD_DTOR);
    }

    #[test]
    fn add_animation_replaces_an_existing_sequence_of_another_group() {
        let mut f = add_fixture(5);
        let old = f.e.mem.alloc(0x80);
        f.e.mem.set_u32(old + 0x74, 0xbeef);
        let entry = single(&mut f.e, old);
        f.e.mem.set_u32(entry.addr(), VTABLE_ANIM_SEQUENCE_SINGLE);
        f.e.mem.set_u32(f.map + 4, entry.addr());
        // The old sequence is current in slot 3 and the last movement
        // sequence.
        let this = f.this;
        f.e.mem.set_u32(this.addr() + 0xe0 + 12, old);
        f.e.mem.set_u16(this.addr() + 0x4c + 6, 9);
        f.e.set(this, Animation::pLastMovementSequence, Ptr::new(old));
        start_log(&mut f.e);
        assert!(f.add(false));
        let log = end_log(&mut f.e);
        assert_eq!(
            arguments_of(&log, MANAGER_REMOVE_SEQUENCE),
            [[f.manager, old]]
        );
        assert_eq!(f.e.mem.u32(this.addr() + 0xe0 + 12), 0);
        assert_eq!(f.e.mem.u16(this.addr() + 0x4c + 6), 0);
        assert!(f.e.get(this, Animation::pLastMovementSequence).is_null());
        // The entry now holds the new sequence.
        assert_eq!(f.e.get(entry, AnimSequenceSingle::pSeq).addr(), f.sequence);
        assert_eq!(arguments_of(&log, MANAGER_ADD_SEQUENCE).len(), 1);

        // An idle that plays the old sequence keeps it in the manager.
        let mut f = add_fixture(5);
        let old = f.e.mem.alloc(0x80);
        f.e.mem.set_u32(old + 0x74, 0xbeef);
        let entry = single(&mut f.e, old);
        f.e.mem.set_u32(entry.addr(), VTABLE_ANIM_SEQUENCE_SINGLE);
        f.e.mem.set_u32(f.map + 4, entry.addr());
        let idle = f.e.mem.alloc(0x38);
        f.e.mem.set_u32(idle + 0x18, old);
        f.e.mem.set_u32(f.this.addr() + 0x130, idle);
        start_log(&mut f.e);
        assert!(f.add(false));
        let log = end_log(&mut f.e);
        assert!(arguments_of(&log, MANAGER_REMOVE_SEQUENCE).is_empty());
        assert_eq!(f.e.get(entry, AnimSequenceSingle::pSeq).addr(), f.sequence);
    }

    #[test]
    fn add_animation_turns_the_entry_into_a_multiple_for_listed_types() {
        let mut f = add_fixture(5);
        // Type 5 keeps several sequences.
        f.e.mem.set_u8(SEQUENCE_TYPE_TABLE + 5 * 0x24, 1);
        let old = f.e.mem.alloc(0x80);
        f.e.mem.set_u32(old + 0x74, 0xbeef);
        let entry = single(&mut f.e, old);
        f.e.mem.set_u32(entry.addr(), VTABLE_ANIM_SEQUENCE_SINGLE);
        f.e.mem.set_u32(f.map + 4, entry.addr());
        start_log(&mut f.e);
        assert!(f.add(false));
        let log = end_log(&mut f.e);
        // The old entry was removed from the map, a multiple was built from
        // it and stored, and the multiple received the new sequence.
        assert_eq!(arguments_of(&log, MAP_REMOVE_AT), [[f.map, 7]]);
        assert_eq!(arguments_of(&log, MAP_SET_AT).len(), 1);
        assert_ne!(f.entry(), entry.addr());
        assert_eq!(f.e.mem.u32(f.entry()), VTABLE_ANIM_SEQUENCE_MULTIPLE);
        assert!(arguments_of(&log, REF_ADD).contains(&vec![old]));
        assert!(arguments_of(&log, REF_ADD).contains(&vec![f.sequence]));
        assert_eq!(arguments_of(&log, MANAGER_ADD_SEQUENCE).len(), 1);
    }

    #[test]
    fn add_animation_undoes_everything_when_the_manager_refuses() {
        let mut f = add_fixture(5);
        f.e.register(MANAGER_ADD_SEQUENCE, |_, _| ret(0));
        f.e.register(SEQUENCE_OBJECT_COUNT, |_, _| ret(2));
        f.e.register(SEQUENCE_OBJECT_NAME_AT, |_, _| Ret::default());
        f.e.register(FIXED_STRING_INIT, |_, _| Ret::default());
        f.e.register(FIXED_STRING_FREE, |_, _| Ret::default());
        f.e.register(ANIM_SEQUENCE_BASE_DESTRUCT, |_, _| Ret::default());
        f.e.register(MODEL_LOADER_REMOVE, |_, _| Ret::default());
        start_log(&mut f.e);
        assert!(!f.add(false));
        let log = end_log(&mut f.e);
        let root_name = f.e.mem.u32(f.root + 8);
        let _ = root_name;
        let sequence_name = f.e.mem.u32(f.sequence + 8);
        // The failure is logged with the sequence's name, warnings are
        // switched off around the object loop, and each of the two objects
        // (looked up with a null name) is reported as missing.
        let logged = arguments_of(&log, LOG);
        assert_eq!(logged[0][..2], [LOG_UNABLE_TO_ADD, sequence_name]);
        assert_eq!(arguments_of(&log, DISABLE_WARNINGS), [[1], [0]]);
        assert_eq!(
            arguments_of(&log, SEQUENCE_OBJECT_NAME_AT)
                .iter()
                .map(|w| (w[1], w[2]))
                .collect::<Vec<_>>(),
            [(0, 0), (1, 0)]
        );
        assert_eq!(
            logged[1..],
            [
                [LOG_OBJECT_NOT_IN_SKELETON, 0],
                [LOG_OBJECT_NOT_IN_SKELETON, 0]
            ]
        );
        assert_eq!(arguments_of(&log, OPERATOR_DELETE).len(), 2 + 1);
        // The sequence leaves the manager and the entry is removed from the
        // map and deleted.
        assert_eq!(
            arguments_of(&log, MANAGER_REMOVE_SEQUENCE),
            [[f.manager, f.sequence]]
        );
        assert_eq!(f.entry(), 0);
        assert_eq!(log.last().unwrap().0, NI_POINTER_RELEASE);
    }

    // ---- GetAccumRoot and the small getters ----------------------------------------------

    #[test]
    fn get_accum_root_takes_the_first_sequence_that_has_an_object() {
        let mut e = engine();
        let manager = e.mem.alloc(0x7c);
        let slots = e.mem.alloc(12);
        e.mem.set_u32(slots, 0);
        e.mem.set_u32(slots + 4, 0xaaaa);
        e.mem.set_u32(slots + 8, 0xbbbb);
        e.mem.set_u32(manager + 0x34, slots);
        e.mem.set_u32(manager + 0x38, 3);
        e.register(SEQUENCE_ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(SEQUENCE_ARRAY_ELEMENT, |e, a| {
            ret(e.mem.u32(a[0]) + 4 * a[1])
        });
        e.register(ACCUM_ROOT_OF_SEQUENCE, |_, a| ret(a[0] + 1));
        // Not cumulative: null.
        assert_eq!(e.call(0x0049_0d90, &args![manager]).u32(), 0);
        e.mem.set_u8(manager + 0x68, 1);
        assert_eq!(e.call(0x0049_0d90, &args![manager]).u32(), 0xaaab);
        // Cumulative, every slot empty: null.
        e.mem.set_u32(slots + 4, 0);
        e.mem.set_u32(slots + 8, 0);
        assert_eq!(e.call(0x0049_0d90, &args![manager]).u32(), 0);
    }

    #[test]
    fn single_constructor_sets_the_vtable_and_clears_the_sequence() {
        let mut e = engine();
        let this = Ptr::<AnimSequenceSingle>::new(e.mem.alloc(8));
        e.mem.set_u32(this.addr() + 4, 0xdead);
        assert_eq!(
            e.call(0x0049_0e10, &args![this]).ptr::<()>().addr(),
            this.addr()
        );
        assert_eq!(e.mem.u32(this.addr()), VTABLE_ANIM_SEQUENCE_SINGLE);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
    }

    #[test]
    fn idle_sequence_getter_reads_offset_18() {
        let mut e = engine();
        let idle = e.mem.alloc(0x38);
        e.mem.set_u32(idle + 0x18, 0x7171);
        assert_eq!(e.call(0x0049_0e40, &args![idle]).u32(), 0x7171);
    }

    /// Doubles for `ShouldQueueSequenceCloning`: the player's nodes are
    /// `0xa000 + which`, the cloning setting is 1, and the other checks say
    /// "no".
    fn queue_engine() -> (Engine, Ptr<Animation>) {
        let mut e = engine();
        e.set_global(PLAYER_SINGLETON, 0x6000u32);
        e.register(PLAYER_NODE, |_, a| ret(0xa000 + a[1]));
        e.register(SETTING_VALUE_ADDRESS, |_, a| {
            ret(a[0] - SETTING_CLONING + 0x6100)
        });
        e.mem.map(0x6000, 0x1000);
        e.mem.set_u32(0x6100, 1);
        e.register(NODE_ACCEPTS_OBJECT, |_, _| ret(0));
        e.register(FLAG_OBJECT_CHECK, |_, _| ret(0));
        e.register(PLAYER_REFERENCE, |_, _| ret(0xcafe));
        e.register(IS_IN_MENU_MODE, |_, _| ret(0));
        e.register(TES_CHECK, |_, _| ret(0));
        e.register(IS_MENU_ID_VISIBLE, |_, _| ret(0));
        let this: Ptr<Animation> = e.new_object();
        e.mem.set_u32(this.addr() + 8, 0x9000);
        e.set(this, Animation::pActorRef, Ptr::new(0x1234));
        (e, this)
    }

    #[test]
    fn should_queue_cloning_follows_the_player_and_menu_state() {
        let (mut e, this) = queue_engine();
        // The ordinary case: nothing says no, so queue.
        assert!(e.call(0x0049_0e60, &args![this]).bool());

        // No player, no 3D, or the setting off: false.
        e.set_global(PLAYER_SINGLETON, 0u32);
        assert!(!e.call(0x0049_0e60, &args![this]).bool());
        let (mut e, this) = queue_engine();
        e.register(PLAYER_NODE, |_, _| ret(0));
        assert!(!e.call(0x0049_0e60, &args![this]).bool());
        let (mut e, this) = queue_engine();
        e.mem.set_u32(0x6100, 0);
        assert!(!e.call(0x0049_0e60, &args![this]).bool());

        // The player's own nodes (or an object the player's object
        // accepts) follow the Pip-Boy wait menu instead.
        for root in [0xa000u32, 0xa001] {
            let (mut e, this) = queue_engine();
            e.mem.set_u32(this.addr() + 8, root);
            assert!(!e.call(0x0049_0e60, &args![this]).bool());
            e.register(IS_MENU_ID_VISIBLE, |_, a| {
                ret((a[0] == MENU_ID_PIPBOY_WAIT) as u32)
            });
            assert!(e.call(0x0049_0e60, &args![this]).bool());
        }
        let (mut e, this) = queue_engine();
        e.register(NODE_ACCEPTS_OBJECT, |_, _| ret(1));
        assert!(!e.call(0x0049_0e60, &args![this]).bool());
        e.register(IS_MENU_ID_VISIBLE, |_, _| ret(1));
        assert!(e.call(0x0049_0e60, &args![this]).bool());

        // Each of the other checks says no.
        let (mut e, this) = queue_engine();
        e.register(FLAG_OBJECT_CHECK, |_, _| ret(1));
        assert!(!e.call(0x0049_0e60, &args![this]).bool());
        let (mut e, this) = queue_engine();
        e.register(PLAYER_REFERENCE, |_, _| ret(0x1234));
        assert!(!e.call(0x0049_0e60, &args![this]).bool());
        let (mut e, this) = queue_engine();
        e.register(IS_IN_MENU_MODE, |_, _| ret(1));
        assert!(!e.call(0x0049_0e60, &args![this]).bool());
        let (mut e, this) = queue_engine();
        e.register(TES_CHECK, |_, _| ret(1));
        assert!(!e.call(0x0049_0e60, &args![this]).bool());
    }

    #[test]
    fn player_object_getter_reads_offset_69c() {
        let mut e = engine();
        let player = e.mem.alloc(0x6a0);
        e.mem.set_u32(player + 0x69c, 0x8181);
        assert_eq!(e.call(0x0049_0f80, &args![player]).u32(), 0x8181);
    }

    #[test]
    fn queueing_a_model_needs_an_animation_group() {
        let mut e = engine();
        e.register(KF_MODEL_ADD_REF, |_, _| Ret::default());
        e.register(SIMPLE_LIST_PUSH_BACK, |_, _| Ret::default());
        e.register(KF_MODEL_ANIM_GROUP, |e, a| ret(e.mem.u32(a[0] + 8)));
        let this: Ptr<Animation> = e.new_object();
        let kf = e.mem.alloc(0x20);
        start_log(&mut e);
        e.call(0x0049_0fa0, &args![this, kf]);
        let log = end_log(&mut e);
        // No group: nothing is queued.
        assert!(arguments_of(&log, SIMPLE_LIST_PUSH_BACK).is_empty());
        assert_eq!(log.last().unwrap().0, NI_POINTER_RELEASE);
        e.mem.set_u32(kf + 8, 0x4444);
        start_log(&mut e);
        e.call(0x0049_0fa0, &args![this, kf]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, KF_MODEL_ADD_REF), [[kf], [kf]]);
        assert_eq!(
            arguments_of(&log, SIMPLE_LIST_PUSH_BACK)[0][0],
            this.addr() + 0x104
        );
    }

    #[test]
    fn current_sequence_of_a_group_maps_the_special_groups() {
        let mut e = engine();
        let this: Ptr<Animation> = e.new_object();
        for slot in 0..8u32 {
            e.mem.set_u32(this.addr() + 0xe0 + 4 * slot, 0x100 + slot);
        }
        assert_eq!(e.call(0x0049_1040, &args![this, 0x14u32]).u32(), 0x101);
        assert_eq!(e.call(0x0049_1040, &args![this, 0x15u32]).u32(), 0x104);
        assert_eq!(e.call(0x0049_1040, &args![this, 2u32]).u32(), 0x102);
    }

    #[test]
    fn sequence_length_is_end_minus_begin() {
        let mut e = engine();
        e.register(SEQUENCE_BEGIN_TIME, |_, _| 1.5f32.into_ret());
        e.register(SEQUENCE_END_TIME, |_, _| 4.0f32.into_ret());
        assert_eq!(e.call(0x0049_1090, &args![0x1000u32]).f32(), 2.5);
        assert_eq!(e.call(0x0049_1090, &args![0u32]).f32(), 0.0);
    }

    // ---- Second session: `004910d0` onward ---------------------------------------------

    /// The player object and a reference used as the animated actor.
    const PLAYER: u32 = 0x3000_0000;
    const ACTOR: u32 = 0x3000_1000;

    /// `engine()` plus the pages and values of the constants the functions of
    /// the second session load, and doubles for the small getters most of them
    /// go through (flags, sequence state, group id and type).
    fn engine_two() -> Engine {
        let mut e = engine();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0101_a000,
            0x0109_6000,
            0x011a_8000,
            0x011e_0000,
            0x011f_3000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(DOUBLE_ZERO, 0.0f64);
        e.set_global(DOUBLE_MINUS_ONE, -1.0f64);
        e.set_global(DOUBLE_THIRTY, 30.0f64);
        e.set_global(DOUBLE_FOUR, 4.0f64);
        e.set_global(DOUBLE_HALF, 0.5f64);
        e.set_global(DOUBLE_HUNDREDTH, 0.01f64);
        e.set_global(DOUBLE_MICRO, 9.999_999_747_378_752e-6f64);
        e.set_global(FLOAT_HALF, 0.5f32);
        e.set_global(FLOAT_MAX, f32::MAX);
        for i in 0..3 {
            e.set_global(RECORD_DEFAULT_TRANSLATE + 4 * i, -f32::MAX);
        }
        e.set_global(RECORD_DEFAULT_SCALE, -f32::MAX);
        e.set_global(PLAYER_SINGLETON, PLAYER);
        e.map(PLAYER, 0x700);
        e.map(ACTOR, 0x400);
        // Both objects have a vtable whose slot 0x100 ("has a process") says no.
        e.register(0x00f4_0100, |_, _| ret(0));
        for object in [PLAYER, ACTOR] {
            let vtable = e.mem.alloc(0x800);
            e.mem.set_u32(object, vtable);
            e.mem.set_u32(vtable + 0x100, 0x00f4_0100);
        }
        e.register(GROUP_ID_TYPE, |_, a| ret(a[0] & 0xff));
        e.register(ANIM_GROUP_ID, |e, a| ret(e.mem.u16(a[0] + 0x10) as u32));
        e.register(ANIM_GROUP_SEQUENCE_TYPE, |e, a| {
            ret(e.mem.u16(a[0] + 0x10) as u32 & 0xff)
        });
        e.register(SEQUENCE_STATE, |e, a| ret(e.mem.u32(a[0] + 0x44)));
        e.register(SEQUENCE_CYCLE_TYPE, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        e.register(FLAG_SET, |e, a| {
            let flags = e.mem.u8(a[0]);
            let mask = a[2] as u8;
            e.mem.set_u8(
                a[0],
                if a[1] as u8 != 0 {
                    flags | mask
                } else {
                    flags & !mask
                },
            );
            Ret::default()
        });
        e.register(WORD_AT_8, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(WORD_AT_C, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(ACTOR_PROCESS, |e, a| ret(e.mem.u32(a[0] + 0x68)));
        e.register(MANAGER_TARGET, |e, a| ret(e.mem.u32(a[0] + 0x2c)));
        e.register(SEQUENCE_BEGIN_TIME, |e, a| {
            e.mem.f32(a[0] + 0x2c).into_ret()
        });
        e.register(SEQUENCE_END_TIME, |e, a| e.mem.f32(a[0] + 0x30).into_ret());
        e
    }

    /// Writes `slot` and `category` of animation group type `kind` into the
    /// type table.
    fn set_type(e: &mut Engine, kind: u32, slot: i32, category: i32) {
        e.mem.set_i32(SEQUENCE_TYPE_SLOT + kind * 0x24, slot);
        e.mem
            .set_i32(SEQUENCE_TYPE_CATEGORY + kind * 0x24, category);
    }

    /// An animation group object: the id at +0x10.
    fn group_with_id(e: &mut Engine, id: u16) -> u32 {
        let group = e.mem.alloc(0x80);
        e.mem.set_u16(group + 0x10, id);
        group
    }

    /// A sequence with the given state (+0x44) and group (+0x74).
    fn sequence_with_group(e: &mut Engine, state: u32, group: u32) -> u32 {
        let sequence = e.mem.alloc(0x80);
        e.mem.set_u32(sequence + 0x44, state);
        e.mem.set_u32(sequence + 0x74, group);
        sequence
    }

    /// A sequence-map entry (`AnimSequenceBase`): slot 0xc says whether it
    /// holds a single sequence, slot 0x10 gives the stored sequence (+4).
    fn map_entry(e: &mut Engine, sequence: u32, single: bool) -> u32 {
        e.register(0x00f1_000c, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(0x00f1_0010, |e, a| ret(e.mem.u32(a[0] + 4)));
        let entry = object_with_vtable(e, 0x10, &[(0xc, 0x00f1_000c), (0x10, 0x00f1_0010)]);
        e.mem.set_u32(entry + 4, sequence);
        e.mem.set_u32(entry + 8, single as u32);
        entry
    }

    /// An animation whose sequence map is the block returned: `count` at +0 and
    /// `(key, entry)` pairs behind it, with a `GetAt` double that searches it.
    fn map_for(e: &mut Engine, this: Ptr<Animation>) -> u32 {
        let map = e.mem.alloc(0x100);
        e.set(this, Animation::pAnimSequenceMap, Ptr::new(map));
        e.register(MAP_GET_AT, |e, a| {
            let count = e.mem.u32(a[0]);
            for i in 0..count {
                if e.mem.u32(a[0] + 4 + 8 * i) == a[1] {
                    let entry = e.mem.u32(a[0] + 8 + 8 * i);
                    e.mem.set_u32(a[2], entry);
                    return ret(1);
                }
            }
            ret(0)
        });
        map
    }

    /// Adds `(key, entry)` to a map made by `map_for`.
    fn map_add(e: &mut Engine, map: u32, key: u32, entry: u32) {
        let count = e.mem.u32(map);
        e.mem.set_u32(map + 4 + 8 * count, key);
        e.mem.set_u32(map + 8 + 8 * count, entry);
        e.mem.set_u32(map, count + 1);
    }

    // ---- FindSkinnedNode -------------------------------------------------------------

    /// Nodes for `FindSkinnedNode`: +0x10 the object slot 0x18 returns, +0x14 the
    /// node slot 0xc returns (the children holder), +0xbc the skin of an object,
    /// +0x9c the child count and +0xa0.. the children of a children holder.
    fn skin_nodes(e: &mut Engine) {
        e.register(0x00f1_0018, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(0x00f1_000c, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        e.register(NODE_SKIN, |e, a| ret(e.mem.u32(a[0] + 0xbc)));
        e.register(NODE_CHILD_COUNT, |e, a| ret(e.mem.u32(a[0] + 0x9c)));
        e.register(NODE_CHILD_AT, |e, a| ret(e.mem.u32(a[0] + 0xa0 + 4 * a[1])));
    }

    fn skin_node(e: &mut Engine) -> u32 {
        object_with_vtable(e, 0x100, &[(0x18, 0x00f1_0018), (0xc, 0x00f1_000c)])
    }

    #[test]
    fn find_skinned_node_looks_at_the_node_and_then_its_children() {
        let mut e = engine_two();
        skin_nodes(&mut e);
        assert!(!e.call(0x0049_10d0, &args![0u32]).bool());
        // A node whose slot-0x18 object has a skin.
        let node = skin_node(&mut e);
        let owner = e.mem.alloc(0x100);
        e.mem.set_u32(owner + 0xbc, 0x1234);
        e.mem.set_u32(node + 0x10, owner);
        start_log(&mut e);
        assert!(e.call(0x0049_10d0, &args![node]).bool());
        assert_eq!(called(&mut e), [0x00f1_0018, NODE_SKIN]);
        // A node without an owner, whose second child has a skinned child.
        let root = skin_node(&mut e);
        let holder = e.mem.alloc(0x100);
        let (plain, skinned_parent) = (skin_node(&mut e), skin_node(&mut e));
        let (parent_holder, leaf) = (e.mem.alloc(0x100), skin_node(&mut e));
        e.mem.set_u32(leaf + 0x10, owner);
        e.mem.set_u32(parent_holder + 0x9c, 1);
        e.mem.set_u32(parent_holder + 0xa0, leaf);
        e.mem.set_u32(skinned_parent + 0x14, parent_holder);
        e.mem.set_u32(root + 0x14, holder);
        e.mem.set_u32(holder + 0x9c, 3);
        e.mem.set_u32(holder + 0xa0, plain);
        e.mem.set_u32(holder + 0xa4, 0);
        e.mem.set_u32(holder + 0xa8, skinned_parent);
        assert!(e.call(0x0049_10d0, &args![root]).bool());
        // Nothing skinned anywhere.
        e.mem.set_u32(leaf + 0x10, 0);
        assert!(!e.call(0x0049_10d0, &args![root]).bool());
    }

    // ---- Flags -------------------------------------------------------------------------

    #[test]
    fn flag_functions_test_and_set_the_bits_of_the_flag_byte() {
        let mut e = engine_two();
        let this: Ptr<Animation> = e.new_object();
        assert!(!e.call(0x0049_3970, &args![this]).bool());
        assert!(!e.call(0x0049_3b90, &args![this]).bool());
        e.call(0x0049_39b0, &args![this, 1u32]);
        assert_eq!(e.mem.u8(this.addr()), 2);
        assert!(e.call(0x0049_3970, &args![this]).bool());
        assert!(e.call(0x0049_3990, &args![this, 3u32]).bool());
        assert!(!e.call(0x0049_3990, &args![this, 4u32]).bool());
        e.call(0x0049_3860, &args![this, 1u32]);
        assert_eq!(e.mem.u8(this.addr()), 3);
        assert!(e.call(0x0049_3b90, &args![this]).bool());
        e.call(0x0049_39b0, &args![this, 0u32]);
        e.call(0x0049_3860, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this.addr()), 0);
        // The setters pass the mask 1 and 2 to the flag function.
        start_log(&mut e);
        e.call(0x0049_3860, &args![this, 1u32]);
        e.call(0x0049_39b0, &args![this, 0u32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, FLAG_SET),
            [[this.addr(), 1, 1], [this.addr(), 0, 2]]
        );
    }

    #[test]
    fn small_byte_getters_read_their_offsets() {
        let mut e = engine_two();
        let object = e.mem.alloc(0x300);
        e.mem.set_u8(object + 0x10, 0xf0);
        e.mem.set_u8(object + 0x104, 0x7a);
        e.mem.set_u8(object + 0x222, 0x33);
        // Sign extended into the result.
        assert_eq!(e.call(0x0049_3750, &args![object]).i32(), -16);
        assert_eq!(e.call(0x0049_3bb0, &args![object]).u8(), 0x7a);
        assert_eq!(e.call(0x0049_55a0, &args![object]).u8(), 0x33);
    }

    #[test]
    fn the_group_blend_bytes_give_the_larger_of_two() {
        let mut e = engine_two();
        let group = e.mem.alloc(0x40);
        e.mem.set_u8(group + 0x29, 5);
        e.mem.set_u8(group + 0x2a, 9);
        e.mem.set_u8(group + 0x2b, 2);
        assert_eq!(e.call(0x0049_54e0, &args![group]).u8(), 9);
        assert_eq!(e.call(0x0049_5520, &args![group]).u8(), 5);
        e.mem.set_u8(group + 0x2b, 200);
        assert_eq!(e.call(0x0049_5520, &args![group]).u8(), 200);
    }

    // ---- Sequence weight, last time and records ------------------------------------------

    #[test]
    fn sequence_weight_is_stored_and_never_negative() {
        let mut e = engine_two();
        let sequence = e.mem.alloc(0x80);
        e.call(0x0049_5480, &args![sequence, 0.75f32]);
        assert_eq!(e.call(0x0049_5460, &args![sequence]).f32(), 0.75);
        e.call(0x0049_5480, &args![sequence, -2.0f32]);
        assert_eq!(e.call(0x0049_5460, &args![sequence]).f32(), 0.0);
        // The last scaled time is reset to -FLT_MAX.
        e.call(0x0049_54c0, &args![sequence]);
        assert_eq!(e.mem.f32(sequence + 0x48), -f32::MAX);
    }

    #[test]
    fn transform_records_start_invalid_and_are_checked_by_their_first_float() {
        let mut e = engine_two();
        for i in 0..4 {
            e.set_global(RECORD_DEFAULT_ROTATE + 4 * i, 0.25f32 * i as f32);
        }
        let record = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0049_4260, &args![record]).u32(), record);
        assert_eq!(e.mem.f32(record), -f32::MAX);
        assert_eq!(e.mem.f32(record + 8), -f32::MAX);
        assert_eq!(e.mem.f32(record + 12 + 8), 0.5);
        assert_eq!(e.mem.f32(record + 0x1c), -f32::MAX);
        assert!(!e.call(0x0049_42c0, &args![record]).bool());
        e.mem.set_f32(record, 1.0);
        assert!(e.call(0x0049_42c0, &args![record]).bool());
        e.mem.set_f32(record, f32::NAN);
        assert!(e.call(0x0049_42c0, &args![record]).bool());
    }

    #[test]
    fn vector_is_scaled_by_the_inverse_of_the_divisor() {
        let mut e = engine_two();
        let vector = e.mem.alloc(12);
        for (i, v) in [2.0f32, -4.0, 8.0].iter().enumerate() {
            e.mem.set_f32(vector + 4 * i as u32, *v);
        }
        assert_eq!(e.call(0x0049_41c0, &args![vector, 4.0f32]).u32(), vector);
        assert_eq!(
            [
                e.mem.f32(vector),
                e.mem.f32(vector + 4),
                e.mem.f32(vector + 8)
            ],
            [0.5, -1.0, 2.0]
        );
    }

    #[test]
    fn interpolator_target_is_read_from_the_sixteen_byte_entry() {
        let mut e = engine_two();
        let object = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x14, table);
        let target = e.mem.alloc(0x40);
        e.mem.set_u32(target + 0x2c, 0xabcd);
        e.mem.set_u32(table + 0x10 + 4, target);
        assert_eq!(e.call(0x0049_4210, &args![object, 1u32]).u32(), 0xabcd);
        // An empty entry has no target.
        assert_eq!(e.call(0x0049_4210, &args![object, 0u32]).u32(), 0);
    }

    // ---- Process object, setting byte -----------------------------------------------------

    #[test]
    fn process_array_and_setting_byte_getters() {
        let mut e = engine_two();
        let process = e.mem.alloc(0x200);
        e.mem.set_u32(process + 0x1c4 + 8, 0x777);
        assert_eq!(e.call(0x0049_5560, &args![process, 2i32]).u32(), 0x777);
        e.register(SETTING_BYTE_ADDRESS, |e, a| {
            assert_eq!(a[0], SETTING_BYTE_OBJECT);
            ret(e.mem.alloc(4))
        });
        // The byte at the returned address is read (a fresh block is zero).
        assert_eq!(e.call(0x0049_5580, &args![]).u8(), 0);
    }

    // ---- Controller manager and blend interpolator ------------------------------------------

    #[test]
    fn manager_sequence_count_and_element_use_the_array_at_0x34() {
        let mut e = engine_two();
        e.register(SEQUENCE_ARRAY_COUNT, |e, a| {
            ret(e.mem.u16(a[0] + 0xa) as u32)
        });
        e.register(SEQUENCE_ARRAY_ELEMENT, |e, a| {
            ret(e.mem.u32(a[0] + 4) + 4 * a[1])
        });
        let manager = e.mem.alloc(0x80);
        e.mem.set_u16(manager + 0x34 + 0xa, 3);
        let elements = e.mem.alloc(0x20);
        e.mem.set_u32(manager + 0x34 + 4, elements);
        e.mem.set_u32(elements + 8, 0x4242);
        assert_eq!(e.call(0x0049_5d00, &args![manager]).u32(), 3);
        start_log(&mut e);
        assert_eq!(e.call(0x0049_5d20, &args![manager, 2u32]).u32(), 0x4242);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, SEQUENCE_ARRAY_ELEMENT),
            [[manager + 0x34, 2]]
        );
    }

    #[test]
    fn blend_interpolator_returns_the_single_interpolator_for_its_index() {
        let mut e = engine_two();
        let blend = e.mem.alloc(0x40);
        let entries = e.mem.alloc(0x80);
        e.mem.set_u32(blend + 0x14, entries);
        e.mem.set_u32(blend + 0x18, 0x5050);
        e.mem.set_u32(entries + 0x18 * 2, 0x6060);
        // Not a single blend: the entry of the array.
        assert_eq!(e.call(0x0049_5d50, &args![blend, 2u32]).u32(), 0x6060);
        e.mem.set_u8(blend + 0xe, 1);
        e.mem.set_u8(blend + 0xf, 2);
        assert_eq!(e.call(0x0049_5d50, &args![blend, 2u32]).u32(), 0x5050);
        // A different index still reads the array.
        e.mem.set_u32(entries + 0x18, 0x7070);
        assert_eq!(e.call(0x0049_5d50, &args![blend, 1u32]).u32(), 0x7070);
    }

    // ---- UpdateMovementNoWorldUpdate, UpdateMovement, scene graph -----------------------

    /// An animation with an animation root, an accumulation root whose first
    /// child is `child`, and an actor with a process whose movement object is
    /// `movement`. Doubles for the movement/flag getters it calls.
    struct MovementFixture {
        e: Engine,
        this: Ptr<Animation>,
        actor: u32,
        child: u32,
    }

    fn movement_fixture(speed: f32, accumulator_accepts: bool) -> MovementFixture {
        let mut e = engine_two();
        e.register(0x00f2_01d0, |e, a| ret(e.mem.u32(a[0] + 0x70)));
        e.register(0x00f2_0010, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        let actor = object_with_vtable(&mut e, 0x200, &[(0x1d0, 0x00f2_01d0)]);
        let process = object_with_vtable(&mut e, 0x40, &[(0x10, 0x00f2_0010)]);
        let movement = e.mem.alloc(0xc0);
        e.mem.set_f32(movement + 0xb4, speed);
        e.mem.set_u32(process + 0x20, movement);
        e.mem.set_u32(actor + 0x70, process);
        e.mem.set_u32(actor + 0xc, 0x00ab_cdef);
        e.register(ACTOR_WORD_108, |e, a| ret(e.mem.u32(a[0] + 0x108)));
        e.register(MOVEMENT_SPEED, |e, a| (e.mem.f32(a[0] + 0xb4)).into_ret());
        e.register(ACTOR_FLAG_SET, |e, a| {
            let flags = e.mem.u32(a[0] + 0x11c);
            let flags = if a[1] != 0 {
                flags | a[2]
            } else {
                flags & !a[2]
            };
            e.mem.set_u32(a[0] + 0x11c, flags);
            Ret::default()
        });
        e.register(SHADER_ACCUMULATOR, |_, _| ret(0x6600));
        e.register_double(SHADER_ACCUMULATOR_TEST, move |_, a| {
            assert_eq!(a[1], 0x00ab_cdef);
            ret(accumulator_accepts as u32)
        });
        e.register(NODE_CHILD_AT, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        e.register(NODE_WORLD_TRANSLATION, |_, a| ret(a[0] + 0x58));
        let this: Ptr<Animation> = e.new_object();
        let root = e.mem.alloc(0x80);
        let accum = e.mem.alloc(0x80);
        let child = e.mem.alloc(0x80);
        e.mem.set_u32(accum + 0x40, child);
        e.mem.set_f32(child + 0x58 + 8, 5.0);
        e.mem.set_u32(this.addr() + 8, root);
        e.set(this, Animation::pAccumRoot, Ptr::new(accum));
        e.set(this, Animation::time, 0.5f32);
        e.register_double(BIP_UPDATE_ALL_BUT_BIP, move |e, a| {
            // The biped update lifts the first child by 2.5.
            assert_eq!(a[1], 0.5f32.to_bits());
            let z = e.mem.f32(child + 0x58 + 8);
            e.mem.set_f32(child + 0x58 + 8, z + 2.5);
            Ret::default()
        });
        MovementFixture {
            e,
            this,
            actor,
            child,
        }
    }

    #[test]
    fn movement_update_sets_the_actor_flags_and_the_height_change() {
        let mut f = movement_fixture(0.5, false);
        start_log(&mut f.e);
        f.e.call(0x0049_39d0, &args![f.this, f.actor]);
        let log = end_log(&mut f.e);
        let flags = f.e.mem.u32(f.actor + 0x11c);
        // Bit 1 always; bit 2 (accumulator test false); bit 4 (speed 0.5 is
        // above the threshold).
        assert_eq!(flags, 7);
        assert_eq!(
            arguments_of(&log, ACTOR_FLAG_SET),
            [[f.actor, 1, 1], [f.actor, 1, 2], [f.actor, 1, 4],]
        );
        assert_eq!(f.e.mem.f32(f.this.addr() + 0x18), 2.5);
        assert_eq!(f.e.mem.u8(f.this.addr()), 1);
        // The biped update gets the animation root and the time.
        let root = f.e.mem.u32(f.this.addr() + 8);
        assert_eq!(
            arguments_of(&log, BIP_UPDATE_ALL_BUT_BIP),
            [[root, 0.5f32.to_bits()]]
        );
        let _ = f.child;
    }

    #[test]
    fn movement_update_clears_flags_two_and_four_for_a_slow_actor() {
        let mut f = movement_fixture(0.0, true);
        f.e.call(0x0049_39d0, &args![f.this, f.actor]);
        // Bit 2 cleared by the accepting accumulator, bit 4 by the zero speed.
        assert_eq!(f.e.mem.u32(f.actor + 0x11c), 1);
        // An actor with a 1 at +0x108 skips the speed check but still sets the flags.
        let mut f = movement_fixture(0.5, false);
        f.e.mem.set_u32(f.actor + 0x108, 1);
        start_log(&mut f.e);
        f.e.call(0x0049_39d0, &args![f.this, f.actor]);
        let log = end_log(&mut f.e);
        // The speed is read once (for bit 4), not twice.
        assert_eq!(arguments_of(&log, MOVEMENT_SPEED).len(), 1);
    }

    #[test]
    fn movement_update_stops_early_without_roots_or_with_the_updated_bit() {
        let mut f = movement_fixture(0.5, false);
        f.e.mem.set_u8(f.this.addr(), 1);
        start_log(&mut f.e);
        f.e.call(0x0049_39d0, &args![f.this, f.actor]);
        assert!(!called(&mut f.e).contains(&ACTOR_FLAG_SET));
        f.e.mem.set_u8(f.this.addr(), 0);
        f.e.set(f.this, Animation::pAccumRoot, Ptr::NULL);
        start_log(&mut f.e);
        f.e.call(0x0049_39d0, &args![f.this, f.actor]);
        assert!(!called(&mut f.e).contains(&ACTOR_FLAG_SET));
        f.e.mem.set_u32(f.this.addr() + 8, 0);
        start_log(&mut f.e);
        f.e.call(0x0049_39d0, &args![f.this, f.actor]);
        assert_eq!(called(&mut f.e), [READ_WORD]);
    }

    /// Doubles for the setting and flag object `UpdateSceneGraphNoController`
    /// reads; `limit` is the value of the limit setting.
    fn scene_graph_settings(e: &mut Engine, limit: i32, forced: i8) {
        let value = e.mem.alloc(4);
        e.mem.set_i32(value, limit);
        e.register_double(SETTING_VALUE_ADDRESS, move |_, a| {
            assert_eq!(a[0], SETTING_SCENE_GRAPH_LIMIT);
            ret(value)
        });
        e.set_global(SCENE_GRAPH_ALWAYS_UPDATE, forced);
        e.set_global(FLAG_OBJECT_GLOBAL, 0x7000u32);
        e.register(FLAG_OBJECT_CHECK, |_, _| ret(0));
    }

    #[test]
    fn scene_graph_update_is_postponed_above_the_limit() {
        let mut f = movement_fixture(0.5, false);
        scene_graph_settings(&mut f.e, 2, 0);
        start_log(&mut f.e);
        f.e.call(0x0049_3bd0, &args![f.this]);
        let log = end_log(&mut f.e);
        assert_eq!(arguments_of(&log, FLAG_SET), [[f.this.addr(), 1, 2]]);
        assert_eq!(f.e.mem.u8(f.this.addr()), 2);
        assert!(arguments_of(&log, OBJECT_SET_CONTROLLERS).is_empty());
        // Already postponed once: the next call updates.
        scene_graph_settings(&mut f.e, 2, 0);
        f.e.register(OBJECT_CONTROLLERS, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        f.e.register(OBJECT_SET_CONTROLLERS, |e, a| {
            e.mem.set_u32(a[0] + 0xc, a[1]);
            Ret::default()
        });
        f.e.register(UPDATE_DATA_CONSTRUCT, |_, _| Ret::default());
        f.e.register(0x00f2_00a4, |_, _| Ret::default());
        let root = f.e.mem.u32(f.this.addr() + 8);
        let vtable = f.e.mem.alloc(0x100);
        f.e.mem.set_u32(vtable + 0xa4, 0x00f2_00a4);
        f.e.mem.set_u32(root, vtable);
        start_log(&mut f.e);
        f.e.call(0x0049_3bd0, &args![f.this]);
        assert!(called(&mut f.e).contains(&OBJECT_SET_CONTROLLERS));
        // Also forced by the global byte.
        let mut f = movement_fixture(0.5, false);
        scene_graph_settings(&mut f.e, 2, 1);
        f.e.register(OBJECT_CONTROLLERS, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        f.e.register(OBJECT_SET_CONTROLLERS, |_, _| Ret::default());
        f.e.register(UPDATE_DATA_CONSTRUCT, |_, _| Ret::default());
        f.e.register(0x00f2_00a4, |_, _| Ret::default());
        let root = f.e.mem.u32(f.this.addr() + 8);
        let vtable = f.e.mem.alloc(0x100);
        f.e.mem.set_u32(vtable + 0xa4, 0x00f2_00a4);
        f.e.mem.set_u32(root, vtable);
        start_log(&mut f.e);
        f.e.call(0x0049_3bd0, &args![f.this]);
        assert!(called(&mut f.e).contains(&OBJECT_SET_CONTROLLERS));
    }

    #[test]
    fn scene_graph_update_detaches_the_controllers_while_it_runs() {
        let mut f = movement_fixture(0.5, false);
        scene_graph_settings(&mut f.e, 1, 0);
        let root = f.e.mem.u32(f.this.addr() + 8);
        let accum = f.e.get(f.this, Animation::pAccumRoot).addr();
        f.e.mem.set_u32(root + 0xc, 0x1111);
        f.e.mem.set_u32(accum + 0xc, 0x2222);
        f.e.register(OBJECT_CONTROLLERS, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        f.e.register(OBJECT_SET_CONTROLLERS, |e, a| {
            e.mem.set_u32(a[0] + 0xc, a[1]);
            Ret::default()
        });
        f.e.register(UPDATE_DATA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        // The node's update (slot 0xa4) runs with both controller lists detached.
        f.e.register_double(0x00f2_00a4, move |e, a| {
            assert_eq!(e.mem.u32(root + 0xc), 0);
            assert_eq!(e.mem.u32(accum + 0xc), 0);
            assert_eq!(e.mem.u32(a[1]), 0.5f32.to_bits());
            assert_eq!(a[2], 0);
            Ret::default()
        });
        let vtable = f.e.mem.alloc(0x100);
        f.e.mem.set_u32(vtable + 0xa4, 0x00f2_00a4);
        f.e.mem.set_u32(root, vtable);
        f.e.call(0x0049_3bd0, &args![f.this]);
        assert_eq!(f.e.mem.u32(root + 0xc), 0x1111);
        assert_eq!(f.e.mem.u32(accum + 0xc), 0x2222);
    }

    #[test]
    fn queued_scene_graph_update_runs_once_and_clears_the_bit() {
        let mut f = movement_fixture(0.5, false);
        scene_graph_settings(&mut f.e, 2, 0);
        start_log(&mut f.e);
        assert!(!f.e.call(0x0049_3930, &args![f.this]).bool());
        assert!(arguments_of(&end_log(&mut f.e), FLAG_SET).is_empty());
        // With the bit set (and the limit low) it updates and clears it.
        scene_graph_settings(&mut f.e, 1, 0);
        f.e.mem.set_u8(f.this.addr(), 2);
        f.e.register(OBJECT_CONTROLLERS, |_, _| ret(0));
        f.e.register(OBJECT_SET_CONTROLLERS, |_, _| Ret::default());
        f.e.register(UPDATE_DATA_CONSTRUCT, |_, _| Ret::default());
        f.e.register(0x00f2_00a4, |_, _| Ret::default());
        let root = f.e.mem.u32(f.this.addr() + 8);
        let vtable = f.e.mem.alloc(0x100);
        f.e.mem.set_u32(vtable + 0xa4, 0x00f2_00a4);
        f.e.mem.set_u32(root, vtable);
        assert!(f.e.call(0x0049_3930, &args![f.this]).bool());
        assert_eq!(f.e.mem.u8(f.this.addr()), 0);
    }

    #[test]
    fn update_movement_runs_the_movement_and_then_the_scene_graph() {
        let mut f = movement_fixture(0.5, false);
        scene_graph_settings(&mut f.e, 2, 0);
        start_log(&mut f.e);
        f.e.call(0x0049_3900, &args![f.this, f.actor]);
        let log = end_log(&mut f.e);
        let position = |address: u32| log.iter().position(|(a, _)| *a == address).unwrap();
        // The biped update comes first, the postponement last.
        assert!(position(BIP_UPDATE_ALL_BUT_BIP) < position(SETTING_VALUE_ADDRESS));
        assert_eq!(f.e.mem.u8(f.this.addr()), 3);
        assert_eq!(f.e.mem.f32(f.this.addr() + 0x18), 2.5);
    }

    // ---- InitGroupSpeed ----------------------------------------------------------------------

    /// An animation group with a given id and movement vector words.
    struct SpeedFixture {
        e: Engine,
        this: Ptr<Animation>,
        group: u32,
    }

    fn speed_fixture(category: i32) -> SpeedFixture {
        let mut e = engine_two();
        let this: Ptr<Animation> = e.new_object();
        // Group id 0x0105: type 5.
        let group = group_with_id(&mut e, 0x0105);
        set_type(&mut e, 5, 1, category);
        e.mem.set_u32(SEQUENCE_TYPE_NAME + 5 * 0x24, 0);
        let manager = e.mem.alloc(0x80);
        e.mem.set_u8(manager + 0x68, 1);
        e.mem.set_u32(this.addr() + 0xd8, manager);
        e.register(SEQUENCE_ARRAY_COUNT, |_, _| ret(1));
        e.register(SEQUENCE_ARRAY_ELEMENT, |e, a| {
            ret(e.mem.u32(a[0] + 4) + 4 * a[1])
        });
        let first_sequence = e.mem.alloc(0x80);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, first_sequence);
        e.mem.set_u32(manager + 0x34 + 4, array);
        // The accumulation root of the first sequence is the target of the
        // interpolator entry 0 of the entry's sequence.
        let accum_root = e.mem.alloc(0x40);
        e.register_double(ACCUM_ROOT_OF_SEQUENCE, move |_, _| ret(accum_root));
        let target = e.mem.alloc(0x40);
        e.mem.set_u32(target + 0x2c, accum_root);
        let sequence = e.mem.alloc(0x80);
        let entries = e.mem.alloc(0x20);
        e.mem.set_u32(entries + 4, target);
        e.mem.set_u32(sequence + 0x14, entries);
        e.mem.set_u32(sequence + 0xc, 1);
        e.mem.set_f32(sequence + 0x2c, 0.0);
        e.mem.set_f32(sequence + 0x30, 2.0);
        // The interpolator's transform at time t: translation (3t, 0, t).
        e.register(0x00f2_008c, |e, a| {
            let time = f32::from_bits(a[1]);
            e.mem.set_f32(a[3], 3.0 * time);
            e.mem.set_f32(a[3] + 4, 0.0);
            e.mem.set_f32(a[3] + 8, time);
            Ret::default()
        });
        let interpolator = object_with_vtable(&mut e, 0x20, &[(0x8c, 0x00f2_008c)]);
        e.register_double(SEQUENCE_INTERPOLATOR_AT, move |_, _| ret(interpolator));
        e.register(RECORD_ADDRESS, |_, a| ret(a[0]));
        e.register(VECTOR_SUBTRACT, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            ret(a[1])
        });
        e.register(GROUP_SET_MOVEMENT_VECTOR, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 0x1c + 4 * i, value);
            }
            Ret::default()
        });
        e.register(GROUP_SPEED, |e, a| e.mem.f32(a[0] + 0x1c).into_ret());
        let map = map_for(&mut e, this);
        let entry = map_entry(&mut e, sequence, true);
        map_add(&mut e, map, 0x0105, entry);
        e.register(LOG, |_, _| Ret::default());
        SpeedFixture { e, this, group }
    }

    #[test]
    fn group_speed_is_the_translation_per_second_between_start_and_end() {
        let mut f = speed_fixture(0);
        start_log(&mut f.e);
        f.e.call(0x0049_3d50, &args![f.this, f.group]);
        let log = end_log(&mut f.e);
        // (6, 0, 2) over 2 seconds.
        assert_eq!(f.e.mem.f32(f.group + 0x1c), 3.0);
        assert_eq!(f.e.mem.f32(f.group + 0x20), 0.0);
        assert_eq!(f.e.mem.f32(f.group + 0x24), 1.0);
        assert_eq!(arguments_of(&log, GROUP_SET_MOVEMENT_VECTOR).len(), 1);
        assert!(arguments_of(&log, LOG).is_empty());
    }

    #[test]
    fn group_speed_complains_about_a_group_exported_in_place() {
        // A category that does not move: the vector stays zero.
        let mut f = speed_fixture(3);
        let name = cstring(&mut f.e, "Idle");
        f.e.mem.set_u32(SEQUENCE_TYPE_NAME + 5 * 0x24, name);
        f.e.register(GROUP_WEAPON_TYPE, |_, _| ret(1));
        f.e.register(GROUP_MOVE_TYPE, |_, _| ret(2));
        let weapon = cstring(&mut f.e, "2h");
        let movement = cstring(&mut f.e, "Fast");
        f.e.mem.set_u32(WEAPON_NAME_TABLE + 4, weapon);
        f.e.mem.set_u32(MOVE_NAME_TABLE + 8, movement);
        let root = f.e.mem.alloc(0x80);
        let root_name = cstring(&mut f.e, "Root");
        f.e.mem.set_u32(root + 8, root_name);
        f.e.mem.set_u32(f.this.addr() + 8, root);
        start_log(&mut f.e);
        f.e.call(0x0049_3d50, &args![f.this, f.group]);
        let log = end_log(&mut f.e);
        assert_eq!(
            arguments_of(&log, LOG),
            [[LOG_ANIMATE_IN_PLACE, movement, weapon, name, root_name]]
        );
        assert!(arguments_of(&log, GROUP_SET_MOVEMENT_VECTOR).is_empty());
    }

    #[test]
    fn group_speed_logs_a_missing_accumulation_root_and_skips_other_cases() {
        let mut f = speed_fixture(0);
        let manager = f.e.mem.u32(f.this.addr() + 0xd8);
        f.e.mem.set_u8(manager + 0x68, 0);
        let root = f.e.mem.alloc(0x80);
        let root_name = cstring(&mut f.e, "Root");
        f.e.mem.set_u32(root + 8, root_name);
        f.e.mem.set_u32(f.this.addr() + 8, root);
        start_log(&mut f.e);
        f.e.call(0x0049_3d50, &args![f.this, f.group]);
        let log = end_log(&mut f.e);
        assert_eq!(arguments_of(&log, LOG), [[LOG_NO_ACCUM_ROOT, root_name]]);
        // No manager: nothing at all.
        f.e.mem.set_u32(f.this.addr() + 0xd8, 0);
        start_log(&mut f.e);
        f.e.call(0x0049_3d50, &args![f.this, f.group]);
        assert!(arguments_of(&end_log(&mut f.e), LOG).is_empty());
        // Group types 1 and 2 are skipped.
        let mut f = speed_fixture(0);
        let group = group_with_id(&mut f.e, 0x0101);
        let entry = f.e.mem.u32(f.e.mem.u32(f.this.addr() + 0xdc) + 8);
        let map = f.e.mem.u32(f.this.addr() + 0xdc);
        map_add(&mut f.e, map, 0x0101, entry);
        start_log(&mut f.e);
        f.e.call(0x0049_3d50, &args![f.this, group]);
        assert!(arguments_of(&end_log(&mut f.e), GROUP_SET_MOVEMENT_VECTOR).is_empty());
    }

    #[test]
    fn adding_a_group_queues_it_for_an_empty_first_slot() {
        let mut e = engine_two();
        let this: Ptr<Animation> = e.new_object();
        let group = group_with_id(&mut e, 0);
        // Null group: nothing happens.
        start_log(&mut e);
        e.call(0x0049_46a0, &args![this, 0u32]);
        assert!(called(&mut e).is_empty());
        // Slot 1 and its next group are empty (0xff) and the id is 0.
        e.mem.set_u16(this.addr() + 0x4e, 0xff);
        e.mem.set_u16(this.addr() + 0x9e, 0xff);
        e.call(0x0049_46a0, &args![this, group]);
        assert_eq!(e.mem.u16(this.addr() + 0x9e), 0);
        // A nonzero id leaves it alone, so does a group in the slot.
        e.mem.set_u16(this.addr() + 0x9e, 0xff);
        e.mem.set_u16(group + 0x10, 0x0102);
        e.call(0x0049_46a0, &args![this, group]);
        assert_eq!(e.mem.u16(this.addr() + 0x9e), 0xff);
        e.mem.set_u16(group + 0x10, 0);
        e.mem.set_u16(this.addr() + 0x4e, 3);
        e.call(0x0049_46a0, &args![this, group]);
        assert_eq!(e.mem.u16(this.addr() + 0x9e), 0xff);
    }

    #[test]
    fn group_loaded_asks_the_sequence_map() {
        let mut e = engine_two();
        let this: Ptr<Animation> = e.new_object();
        let map = map_for(&mut e, this);
        let entry = map_entry(&mut e, 0, true);
        map_add(&mut e, map, 0x0204, entry);
        assert!(e.call(0x0049_4710, &args![this, 0x0204u32]).bool());
        assert!(!e.call(0x0049_4710, &args![this, 0x0205u32]).bool());
    }

    #[test]
    fn group_movement_is_the_truncated_speed_of_the_groups_sequence() {
        let mut e = engine_two();
        let this: Ptr<Animation> = e.new_object();
        let map = map_for(&mut e, this);
        let group = group_with_id(&mut e, 0x0105);
        let sequence = sequence_with_group(&mut e, 0, group);
        e.register(GROUP_SPEED, |_, _| 7.9f32.into_ret());
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            ret(value as i32 as u32)
        });
        // Unknown group.
        assert_eq!(e.call(0x0049_4300, &args![this, 0x0105u32]).i32(), 0);
        let entry = map_entry(&mut e, sequence, true);
        map_add(&mut e, map, 0x0105, entry);
        assert_eq!(e.call(0x0049_4300, &args![this, 0x0105u32]).i32(), 7);
        // An entry that keeps several sequences, and an empty one.
        let several = map_entry(&mut e, sequence, false);
        map_add(&mut e, map, 0x0106, several);
        assert_eq!(e.call(0x0049_4300, &args![this, 0x0106u32]).i32(), 0);
        let empty = map_entry(&mut e, 0, true);
        map_add(&mut e, map, 0x0107, empty);
        assert_eq!(e.call(0x0049_4300, &args![this, 0x0107u32]).i32(), 0);
    }

    // ---- GetMovement (00494390) ----------------------------------------------------------------

    /// An animation with a root, slot 1 playing a group of type 5 with the given
    /// speed, and an actor of the given kind.
    fn movement_vector_fixture(kind: u32) -> (Engine, Ptr<Animation>, u32) {
        let mut e = engine_two();
        e.register(0x00f3_021c, |_, _| ret(0));
        e.register(0x00f3_0214, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        let actor = object_with_vtable(&mut e, 0x40, &[(0x21c, 0x00f3_021c), (0x214, 0x00f3_0214)]);
        e.mem.set_u32(actor + 0x10, kind);
        let this: Ptr<Animation> = e.new_object();
        let root = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 8, root);
        e.register(TIME_SCALE_GET, |_, _| 1.0f32.into_ret());
        e.set(this, Animation::m_fGlobalTimeMultiplier, 1.0f32);
        e.set(this, Animation::m_fMoveSpeed, 1.0f32);
        e.set(this, Animation::pActorRef, Ptr::new(actor));
        for (i, v) in [100.0f32, -100.0, 5.0].iter().enumerate() {
            e.mem.set_f32(this.addr() + 0x10 + 4 * i as u32, *v);
        }
        let rotation = e.mem.alloc(0x40);
        for i in 0..9 {
            e.mem.set_u32(rotation + 4 * i, 0x100 + i);
        }
        e.register_double(NODE_ROTATION, move |_, _| ret(rotation));
        let group = group_with_id(&mut e, 0x0105);
        let sequence = sequence_with_group(&mut e, 1, group);
        e.mem.set_u32(this.addr() + 0xe4, sequence);
        e.register(GROUP_SPEED, |_, _| 2.0f32.into_ret());
        e.register(REFERENCE_SCALE, |_, _| 2.0f32.into_ret());
        e.register(ACTOR_FLAGS_ANY, |_, _| ret(1));
        (e, this, actor)
    }

    #[test]
    fn frame_movement_is_clamped_scaled_and_flattened() {
        let (mut e, this, actor) = movement_vector_fixture(3);
        let out = e.mem.alloc(12);
        assert!(e
            .call(0x0049_4390, &args![this, out, actor, 0u32, 0u32])
            .bool());
        // limit = scale 2 * (speed 2 * 1 * 1) = 4: x and y clamp to +-4, and the
        // actor kind 3 divides x and y by the scale 2.
        assert_eq!(
            [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)],
            [2.0, -2.0, 5.0]
        );
        // Flattened.
        assert!(e
            .call(0x0049_4390, &args![this, out, actor, 0u32, 1u32])
            .bool());
        assert_eq!(e.mem.f32(out + 8), 0.0);
        // Another actor kind is not divided.
        let (mut e, this, actor) = movement_vector_fixture(4);
        let out = e.mem.alloc(12);
        e.call(0x0049_4390, &args![this, out, actor, 0u32, 0u32]);
        assert_eq!(e.mem.f32(out), 4.0);
        assert_eq!(e.mem.f32(out + 4), -4.0);
        // Without an animation root nothing is written.
        e.mem.set_u32(this.addr() + 8, 0);
        e.mem.set_f32(out, 9.0);
        assert!(!e
            .call(0x0049_4390, &args![this, out, actor, 0u32, 0u32])
            .bool());
        assert_eq!(e.mem.f32(out), 9.0);
    }

    #[test]
    fn frame_movement_can_be_rotated_into_the_output() {
        let (mut e, this, actor) = movement_vector_fixture(4);
        let out = e.mem.alloc(12);
        e.register(MATRIX_TRANSFORM_VERTICES, |e, a| {
            // matrix, translate, count, in, out
            assert_eq!(e.mem.u32(a[0] + 8), 0x102);
            assert_eq!(e.mem.f32(a[1]), 0.0);
            assert_eq!(a[2], 1);
            for i in 0..3 {
                let value = e.mem.f32(a[3] + 4 * i) + 1.0;
                e.mem.set_f32(a[4] + 4 * i, value);
            }
            Ret::default()
        });
        assert!(e
            .call(0x0049_4390, &args![this, out, actor, 1u32, 0u32])
            .bool());
        assert_eq!(
            [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)],
            [5.0, -3.0, 6.0]
        );
    }

    // ---- PlayGroup, StartGroup ------------------------------------------------------------------

    #[test]
    fn play_group_queues_it_or_starts_it_by_the_category_of_its_type() {
        let mut e = engine_two();
        let this: Ptr<Animation> = e.new_object();
        map_for(&mut e, this);
        e.register(UPDATE_BIP_ONLY, |_, _| Ret::default());
        e.set(this, Animation::time, 1.25f32);
        // Type 5 plays in slot 2 and has category 1: queued by mode 0.
        set_type(&mut e, 5, 2, 1);
        start_log(&mut e);
        let started = e
            .call(0x0049_4740, &args![this, 5u32, 0i32, 7i32, -1i32])
            .u32();
        assert_eq!(started, 0);
        assert_eq!(e.mem.u16(this.addr() + 0x9c + 4), 5);
        assert_eq!(e.mem.i32(this.addr() + 0xac + 8), 7);
        assert!(!called(&mut e).contains(&UPDATE_BIP_ONLY));
        // Mode 1 starts it: no next group, the loop count is set and the biped
        // is refreshed with the animation time.
        start_log(&mut e);
        e.call(0x0049_4740, &args![this, 5u32, 1i32, 3i32, -1i32]);
        let log = end_log(&mut e);
        assert_eq!(e.mem.u16(this.addr() + 0x9c + 4), 0xff);
        assert_eq!(e.mem.i32(this.addr() + 0x7c + 8), 3);
        assert_eq!(
            arguments_of(&log, UPDATE_BIP_ONLY),
            [[this.addr(), 1.25f32.to_bits(), 0, 1]]
        );
        // Any other mode does nothing for these categories.
        start_log(&mut e);
        e.call(0x0049_4740, &args![this, 5u32, 2i32, 3i32, -1i32]);
        assert!(!called(&mut e).contains(&UPDATE_BIP_ONLY));
    }

    #[test]
    fn play_group_always_starts_the_groups_of_the_higher_categories() {
        let mut e = engine_two();
        let this: Ptr<Animation> = e.new_object();
        let map = map_for(&mut e, this);
        e.register(UPDATE_BIP_ONLY, |_, _| Ret::default());
        e.register(SPECIAL_IDLE_WORKING, |_, _| ret(0));
        e.register(IS_IN_MENU_MODE, |_, _| ret(0));
        // Type 6: slot 3, category 4. The slot already plays the sequence, so
        // StartGroup_ov2 returns it right away.
        set_type(&mut e, 6, 3, 4);
        let group = group_with_id(&mut e, 0x0006);
        let sequence = sequence_with_group(&mut e, 1, group);
        let entry = map_entry(&mut e, sequence, true);
        map_add(&mut e, map, 6, entry);
        e.mem.set_u32(this.addr() + 0xe0 + 12, sequence);
        start_log(&mut e);
        let started = e
            .call(0x0049_4740, &args![this, 6u32, 0i32, 3i32, -1i32])
            .u32();
        let log = end_log(&mut e);
        assert_eq!(started, sequence);
        assert_eq!(arguments_of(&log, UPDATE_BIP_ONLY).len(), 1);
        // The group 0xff (type 0xff) does nothing and returns 0.
        start_log(&mut e);
        assert_eq!(
            e.call(0x0049_4740, &args![this, 0xffu32, 1i32, 3i32, -1i32])
                .u32(),
            0
        );
        assert_eq!(called(&mut e), [GROUP_ID_TYPE]);
        // The slot numbers 0x14 and 0x15 index the arrays as slots 1 and 4.
        set_type(&mut e, 5, 2, 1);
        e.call(0x0049_4740, &args![this, 5u32, 0i32, 9i32, 0x14i32]);
        e.call(0x0049_4740, &args![this, 5u32, 0i32, 8i32, 0x15i32]);
        assert_eq!(e.mem.u16(this.addr() + 0x9c + 2), 5);
        assert_eq!(e.mem.i32(this.addr() + 0xac + 4), 9);
        assert_eq!(e.mem.i32(this.addr() + 0xac + 16), 8);
    }

    #[test]
    fn start_group_takes_the_sequence_of_the_entry() {
        let mut e = engine_two();
        let this: Ptr<Animation> = e.new_object();
        let map = map_for(&mut e, this);
        e.register(SPECIAL_IDLE_WORKING, |_, _| ret(1));
        set_type(&mut e, 5, 2, 1);
        let sequence = sequence_with_group(&mut e, 0, 0);
        let entry = map_entry(&mut e, sequence, true);
        map_add(&mut e, map, 5, entry);
        // Unknown and 0xff groups stop before anything is asked of the entry.
        start_log(&mut e);
        assert_eq!(e.call(0x0049_48c0, &args![this, 0xffu32, -1i32]).u32(), 0);
        assert_eq!(e.call(0x0049_48c0, &args![this, 9u32, -1i32]).u32(), 0);
        assert!(!called(&mut e).contains(&SPECIAL_IDLE_WORKING));
        // The known group hands its sequence on with the group and the slot.
        start_log(&mut e);
        e.call(0x0049_48c0, &args![this, 5u32, 4i32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, SPECIAL_IDLE_WORKING),
            [[this.addr(), sequence]]
        );
    }

    #[test]
    fn start_group_asks_the_corresponding_sequence_for_slots_five_and_six() {
        let mut e = engine_two();
        register_string_functions(&mut e);
        let this: Ptr<Animation> = e.new_object();
        let map = map_for(&mut e, this);
        e.register(SPECIAL_IDLE_WORKING, |_, _| ret(1));
        // The multiple entry holds a list with "dir\\same.kf"; slot 4 plays a
        // sequence of that name.
        let wanted = named_object(&mut e, "dir\\same.kf");
        let other = named_object(&mut e, "dir\\other.kf");
        let list = make_list(&mut e, &[other, wanted]);
        e.register(LIST_HEAD, |e, a| ret(e.mem.u32(a[0])));
        let playing = named_object(&mut e, "dir\\same.kf");
        e.mem.set_u32(this.addr() + 0xe0 + 16, playing);
        e.register(0x00f1_000c, |e, a| ret(e.mem.u32(a[0] + 8)));
        let entry = object_with_vtable(&mut e, 0x10, &[(0xc, 0x00f1_000c)]);
        e.mem.set_u32(entry + 4, list);
        map_add(&mut e, map, 5, entry);
        start_log(&mut e);
        e.call(0x0049_48c0, &args![this, 5u32, 5i32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, SPECIAL_IDLE_WORKING),
            [[this.addr(), wanted]]
        );
    }

    // ---- StartGroup_ov2 --------------------------------------------------------------------------

    /// Everything `StartGroup_ov2` touches with plain doubles: a group of type 1
    /// (slot 1, category 0), the sequence to start, an animation with a manager,
    /// and the float settings.
    struct StartFixture {
        e: Engine,
        this: Ptr<Animation>,
        manager: u32,
        sequence: u32,
        group: u32,
    }

    fn start_fixture() -> StartFixture {
        let mut e = engine_two();
        e.register(SPECIAL_IDLE_WORKING, |_, _| ret(0));
        e.register(IS_IN_MENU_MODE, |_, _| ret(0));
        e.register(IS_IN_PIPBOY_MENU, |_, _| ret(0));
        e.register(IS_MENU_ID_VISIBLE, |_, _| ret(0));
        e.register(PLAYER_NODE, |_, _| ret(0x4400));
        e.register(PLAYER_GET_ANIMATION, |_, _| ret(0));
        e.register(PIPBOY_QUERY, |_, _| ret(0));
        e.register(CLEAR_GROUP, |_, _| Ret::default());
        e.register(TES_CHECK, |_, _| ret(0));
        e.register(MANAGER_ACTIVATE, |_, _| ret(1));
        e.register(MANAGER_BLEND_IN, |_, _| ret(1));
        e.register(MANAGER_CROSS_FADE, |_, _| ret(1));
        e.register(MANAGER_MORPH_FADE, |_, _| Ret::default());
        e.register(MANAGER_DEACTIVATE, |_, _| Ret::default());
        e.register(SEQUENCE_SET_PHASE, |_, _| Ret::default());
        e.register(GROUP_BYTE_28, |e, a| ret(e.mem.u8(a[0] + 0x28) as u32));
        e.register(SEQUENCE_MORPH_COMPATIBLE, |_, _| ret(1));
        e.register(LOG, |_, _| Ret::default());
        e.register(GROUP_ID_IS_AIM, |_, _| ret(0));
        e.register(GROUP_ID_IS_ATTACK, |_, _| ret(0));
        e.register(GROUP_MOVE_TYPE, |e, a| ret(e.mem.u32(a[0] + 0x60)));
        let settings = e.mem.alloc(16);
        e.mem.set_f32(settings, 0.25);
        e.mem.set_f32(settings + 4, 0.75);
        e.mem.set_f32(settings + 8, 1.0);
        e.register_double(SETTING_FLOAT_ADDRESS, move |_, a| {
            ret(match a[0] {
                SETTING_BLEND_TIME => settings,
                SETTING_BLEND_TIME_MENU => settings + 4,
                SETTING_MOVEMENT_SCALE => settings + 8,
                other => panic!("setting {other:08x}"),
            })
        });
        set_type(&mut e, 1, 1, 0);
        let this: Ptr<Animation> = e.new_object();
        let manager = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0xd8, manager);
        let root = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 8, root);
        let group = group_with_id(&mut e, 1);
        let sequence = sequence_with_group(&mut e, 0, group);
        e.mem.set_u32(sequence + 0xc, 3);
        e.mem.set_f32(sequence + 0x1c, 1.0);
        StartFixture {
            e,
            this,
            manager,
            sequence,
            group,
        }
    }

    impl StartFixture {
        fn start(&mut self) -> u32 {
            let (this, sequence) = (self.this, self.sequence);
            self.e
                .call(0x0049_49a0, &args![this, sequence, 1u32, -1i32])
                .u32()
        }
    }

    #[test]
    fn starting_a_group_blends_in_the_sequence_when_the_slot_is_empty() {
        let mut f = start_fixture();
        start_log(&mut f.e);
        let result = f.start();
        let log = end_log(&mut f.e);
        assert_eq!(result, f.sequence);
        assert_eq!(f.e.mem.u16(f.this.addr() + 0x4c + 2), 1);
        assert_eq!(f.e.mem.u32(f.this.addr() + 0xe0 + 4), f.sequence);
        assert_eq!(f.e.mem.i32(f.this.addr() + 0x5c + 4), 0);
        assert_eq!(
            arguments_of(&log, SEQUENCE_SET_PHASE),
            [[f.sequence, 0.0f32.to_bits(), 0]]
        );
        // The blend time is the setting, divided by the movement scale.
        assert_eq!(
            arguments_of(&log, MANAGER_BLEND_IN),
            [[
                f.manager,
                f.sequence,
                0.0f32.to_bits(),
                0.25f32.to_bits(),
                0,
                0
            ]]
        );
        assert!(arguments_of(&log, MANAGER_ACTIVATE).is_empty());
    }

    #[test]
    fn starting_a_group_gives_up_for_a_working_special_idle_or_a_null_sequence() {
        let mut f = start_fixture();
        f.e.register(SPECIAL_IDLE_WORKING, |_, _| ret(1));
        start_log(&mut f.e);
        assert_eq!(f.start(), 0);
        assert_eq!(called(&mut f.e), [SPECIAL_IDLE_WORKING]);
        f.e.register(SPECIAL_IDLE_WORKING, |_, _| ret(0));
        let this = f.this;
        assert_eq!(
            f.e.call(0x0049_49a0, &args![this, 0u32, 1u32, -1i32]).u32(),
            0
        );
        assert_eq!(
            f.e.call(0x0049_49a0, &args![this, f.sequence, 0xffu32, -1i32])
                .u32(),
            0
        );
        assert_eq!(f.e.mem.u32(f.this.addr() + 0xe0 + 4), 0);
    }

    #[test]
    fn restarting_the_playing_sequence_only_resets_the_action() {
        let mut f = start_fixture();
        f.e.mem.set_u32(f.sequence + 0x44, 1);
        f.e.mem.set_u32(f.this.addr() + 0xe0 + 4, f.sequence);
        f.e.mem.set_i32(f.this.addr() + 0x5c + 4, 2);
        start_log(&mut f.e);
        assert_eq!(f.start(), f.sequence);
        let log = end_log(&mut f.e);
        assert_eq!(f.e.mem.i32(f.this.addr() + 0x5c + 4), 0);
        assert_eq!(f.e.mem.f32(f.sequence + 0x48), -f32::MAX);
        assert!(arguments_of(&log, MANAGER_BLEND_IN).is_empty());
        assert!(arguments_of(&log, MANAGER_ACTIVATE).is_empty());
    }

    #[test]
    fn starting_a_group_cross_fades_from_the_animating_sequence_and_deactivates_it() {
        let mut f = start_fixture();
        let old_group = group_with_id(&mut f.e, 1);
        let old = sequence_with_group(&mut f.e, 1, old_group);
        f.e.mem.set_u32(f.this.addr() + 0xe0 + 4, old);
        f.e.mem.set_u16(f.this.addr() + 0x4c + 2, 1);
        // The old group has 9 blend frames: 9 / 30 = 0.3 seconds.
        f.e.mem.set_u8(old_group + 0x29, 9);
        start_log(&mut f.e);
        assert_eq!(f.start(), f.sequence);
        let log = end_log(&mut f.e);
        assert_eq!(
            arguments_of(&log, MANAGER_CROSS_FADE),
            [[
                f.manager,
                old,
                f.sequence,
                0.3f32.to_bits(),
                0,
                1,
                1.0f32.to_bits(),
                0
            ]]
        );
        assert_eq!(
            arguments_of(&log, MANAGER_DEACTIVATE),
            [[f.manager, old, 0.0f32.to_bits()]]
        );
        assert!(arguments_of(&log, MANAGER_BLEND_IN).is_empty());
    }

    #[test]
    fn starting_a_group_clears_a_slot_whose_sequence_is_not_the_one_asked_for() {
        // The old sequence is animating but the slot asked for is not the slot
        // used (group 0x14 stands for slot 1): the requested slot is cleared.
        let mut f = start_fixture();
        let old_group = group_with_id(&mut f.e, 1);
        let old = sequence_with_group(&mut f.e, 1, old_group);
        f.e.mem.set_u32(f.this.addr() + 0xe0 + 4, old);
        let this = f.this;
        start_log(&mut f.e);
        f.e.call(0x0049_49a0, &args![this, f.sequence, 1u32, 0x14i32]);
        let log = end_log(&mut f.e);
        assert_eq!(
            arguments_of(&log, CLEAR_GROUP),
            [[this.addr(), 0x14, 0.0f32.to_bits()]]
        );
        // A non-player's first slot is not replaced while the old sequence is
        // in any other state than 0 or 1.
        let mut f = start_fixture();
        let old_group = group_with_id(&mut f.e, 1);
        let old = sequence_with_group(&mut f.e, 2, old_group);
        f.e.mem.set_u32(f.this.addr() + 0xe0 + 4, old);
        f.e.mem.set_u32(f.this.addr() + 4, ACTOR);
        assert_eq!(f.start(), 0);
        assert_eq!(f.e.mem.u32(f.this.addr() + 0xe0 + 4), old);
    }

    #[test]
    fn starting_a_group_morphs_between_sequences_with_equal_tags() {
        let mut f = start_fixture();
        let old_group = group_with_id(&mut f.e, 1);
        let old = sequence_with_group(&mut f.e, 1, old_group);
        f.e.mem.set_u32(f.this.addr() + 0xe0 + 4, old);
        f.e.mem.set_u8(old_group + 0x28, 7);
        f.e.mem.set_u8(f.group + 0x28, 7);
        f.e.mem.set_u32(old + 0xc, 3);
        f.e.mem.set_f32(old + 0x1c, 0.5);
        f.e.mem.set_u8(old_group + 0x29, 9);
        start_log(&mut f.e);
        assert_eq!(f.start(), f.sequence);
        let log = end_log(&mut f.e);
        assert_eq!(
            arguments_of(&log, MANAGER_MORPH_FADE),
            [[
                f.manager,
                old,
                f.sequence,
                0.3f32.to_bits(),
                0,
                0.5f32.to_bits(),
                1.0f32.to_bits()
            ]]
        );
        assert!(arguments_of(&log, MANAGER_CROSS_FADE).is_empty());
    }

    #[test]
    fn starting_a_group_reports_why_a_morph_is_refused() {
        // Different controller counts.
        let mut f = start_fixture();
        let old_group = group_with_id(&mut f.e, 1);
        let old = sequence_with_group(&mut f.e, 1, old_group);
        f.e.mem.set_u32(f.this.addr() + 0xe0 + 4, old);
        f.e.mem.set_u8(old_group + 0x28, 7);
        f.e.mem.set_u8(f.group + 0x28, 7);
        f.e.mem.set_u32(old + 0xc, 2);
        let root = f.e.mem.u32(f.this.addr() + 8);
        for (object, name) in [(root, "Root"), (old, "old.kf"), (f.sequence, "new.kf")] {
            let text = cstring(&mut f.e, name);
            f.e.mem.set_u32(object + 8, text);
        }
        let (root_name, old_name, new_name) = (
            f.e.mem.u32(root + 8),
            f.e.mem.u32(old + 8),
            f.e.mem.u32(f.sequence + 8),
        );
        start_log(&mut f.e);
        f.start();
        let log = end_log(&mut f.e);
        assert_eq!(
            arguments_of(&log, LOG),
            [[LOG_MORPH_CONTROLLERS, old_name, 2, new_name, 3, root_name]]
        );
        // Equal counts but different tags.
        let mut f = start_fixture();
        let old_group = group_with_id(&mut f.e, 1);
        let old = sequence_with_group(&mut f.e, 1, old_group);
        f.e.mem.set_u32(f.this.addr() + 0xe0 + 4, old);
        f.e.mem.set_u8(old_group + 0x28, 7);
        f.e.mem.set_u8(f.group + 0x28, 7);
        f.e.mem.set_u32(old + 0xc, 3);
        f.e.register(SEQUENCE_MORPH_COMPATIBLE, |_, _| ret(0));
        let root = f.e.mem.u32(f.this.addr() + 8);
        for (object, name) in [(root, "Root"), (old, "old.kf"), (f.sequence, "new.kf")] {
            let text = cstring(&mut f.e, name);
            f.e.mem.set_u32(object + 8, text);
        }
        let (root_name, old_name, new_name) = (
            f.e.mem.u32(root + 8),
            f.e.mem.u32(old + 8),
            f.e.mem.u32(f.sequence + 8),
        );
        start_log(&mut f.e);
        f.start();
        let log = end_log(&mut f.e);
        assert_eq!(
            arguments_of(&log, LOG),
            [[LOG_MORPH_TAGS, old_name, new_name, root_name]]
        );
        assert!(arguments_of(&log, MANAGER_MORPH_FADE).is_empty());
    }

    #[test]
    fn starting_a_weapon_slot_group_sets_the_sequence_weight() {
        // Type 2 plays in slot 5: the weight of an unheld sequence is 0, and
        // a new sequence in an empty slot 5 activates through the manager.
        let mut f = start_fixture();
        set_type(&mut f.e, 2, 5, 0);
        f.e.mem.set_u32(f.this.addr() + 4, ACTOR);
        let this = f.this;
        start_log(&mut f.e);
        f.e.call(0x0049_49a0, &args![this, f.sequence, 2u32, -1i32]);
        let log = end_log(&mut f.e);
        assert_eq!(f.e.mem.f32(f.sequence + 0x1c), 0.0);
        assert_eq!(
            arguments_of(&log, MANAGER_ACTIVATE),
            [[
                f.manager,
                f.sequence,
                0,
                1,
                0.0f32.to_bits(),
                0.25f32.to_bits(),
                0
            ]]
        );
        // The player's own animation gives slot 4 weight 1 and slot 5 weight 0.
        let mut f = start_fixture();
        set_type(&mut f.e, 2, 4, 0);
        f.e.mem.set_u32(f.this.addr() + 4, PLAYER);
        let this = f.this;
        f.e.register_double(PLAYER_GET_ANIMATION, move |_, _| ret(this.addr()));
        f.e.mem.set_f32(f.sequence + 0x1c, 0.5);
        f.e.call(0x0049_49a0, &args![this, f.sequence, 2u32, -1i32]);
        assert_eq!(f.e.mem.f32(f.sequence + 0x1c), 1.0);
    }

    // ---- ForceSection, SyncSequences ---------------------------------------------------------------

    #[test]
    fn force_section_with_no_action_clears_the_slot_and_activates_the_sequence() {
        let mut e = engine_two();
        let this: Ptr<Animation> = e.new_object();
        let map = map_for(&mut e, this);
        let manager = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0xd8, manager);
        e.register(CLEAR_GROUP, |_, _| Ret::default());
        e.register(MANAGER_ACTIVATE, |_, _| ret(1));
        // The entry hands the sequence for selector 3 out of its +4.
        let sequence = e.mem.alloc(0x80);
        let entry = map_entry(&mut e, sequence, true);
        map_add(&mut e, map, 0x0105, entry);
        start_log(&mut e);
        e.call(
            0x0049_55c0,
            &args![this, 0x15i32, 0x0105u32, -1i32, 0.75f32, 3u32],
        );
        let log = end_log(&mut e);
        // Slot 0x15 is slot 4 of the arrays, ClearGroup keeps the original.
        assert_eq!(e.mem.u32(this.addr() + 0xe0 + 16), sequence);
        assert_eq!(e.mem.u16(this.addr() + 0x4c + 8), 0x0105);
        assert_eq!(e.mem.f32(this.addr() + 0xd0), 0.75);
        assert_eq!(
            arguments_of(&log, CLEAR_GROUP),
            [[this.addr(), 0x15, 0.0f32.to_bits()]]
        );
        assert_eq!(
            arguments_of(&log, MANAGER_ACTIVATE),
            [[
                manager,
                sequence,
                0,
                0,
                1.0f32.to_bits(),
                0.0f32.to_bits(),
                0
            ]]
        );
        // The group 0xff and unknown groups stop after clearing.
        start_log(&mut e);
        e.call(
            0x0049_55c0,
            &args![this, 2i32, 0xffu32, -1i32, 0.5f32, 0u32],
        );
        let log = end_log(&mut e);
        assert!(arguments_of(&log, MANAGER_ACTIVATE).is_empty());
        assert_eq!(e.mem.u16(this.addr() + 0x4c + 4), 0xff);
    }

    #[test]
    fn force_section_with_an_action_starts_a_loaded_group() {
        let mut e = engine_two();
        let this: Ptr<Animation> = e.new_object();
        let map = map_for(&mut e, this);
        e.register(SPECIAL_IDLE_WORKING, |_, _| ret(1));
        let sequence = e.mem.alloc(0x80);
        let entry = map_entry(&mut e, sequence, true);
        map_add(&mut e, map, 0x0105, entry);
        start_log(&mut e);
        e.call(
            0x0049_55c0,
            &args![this, 0x14i32, 0x0105u32, 2i32, 1.5f32, 0u32],
        );
        let log = end_log(&mut e);
        // StartGroup_ov2 got the entry's sequence, the group and the slot.
        assert_eq!(
            arguments_of(&log, SPECIAL_IDLE_WORKING),
            [[this.addr(), sequence]]
        );
        assert_eq!(e.mem.i32(this.addr() + 0x5c + 4), 2);
        assert_eq!(e.mem.u16(this.addr() + 0x4c + 2), 0x0105);
        // A group that is not loaded only stores the group and the time.
        start_log(&mut e);
        e.call(
            0x0049_55c0,
            &args![this, 0x15i32, 0x0777u32, 5i32, 3.0f32, 0u32],
        );
        assert!(!called(&mut e).contains(&SPECIAL_IDLE_WORKING));
        assert_eq!(e.mem.i32(this.addr() + 0x5c + 16), 0);
        assert_eq!(e.mem.u16(this.addr() + 0x4c + 8), 0x0777);
        assert_eq!(e.mem.f32(this.addr() + 0xd0), 3.0);
    }

    #[test]
    fn sync_sequences_starts_the_corresponding_sequence() {
        let mut e = engine_two();
        register_string_functions(&mut e);
        let this: Ptr<Animation> = e.new_object();
        let map = map_for(&mut e, this);
        e.register(SPECIAL_IDLE_WORKING, |_, _| ret(1));
        let wanted = named_object(&mut e, "dir\\same.kf");
        let list = make_list(&mut e, &[wanted]);
        e.register(LIST_HEAD, |e, a| ret(e.mem.u32(a[0])));
        let given = named_object(&mut e, "dir\\same.kf");
        e.register(0x00f1_000c, |e, a| ret(e.mem.u32(a[0] + 8)));
        let several = object_with_vtable(&mut e, 0x10, &[(0xc, 0x00f1_000c)]);
        e.mem.set_u32(several + 4, list);
        map_add(&mut e, map, 0x0105, several);
        let single = map_entry(&mut e, 0, true);
        map_add(&mut e, map, 0x0106, single);
        start_log(&mut e);
        e.call(0x0049_5da0, &args![this, given, 0x0105u32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, SPECIAL_IDLE_WORKING),
            [[this.addr(), wanted]]
        );
        // 0xff, unknown groups and single-sequence entries give nothing.
        for group in [0xffu32, 0x0999, 0x0106] {
            assert_eq!(e.call(0x0049_5da0, &args![this, given, group]).u32(), 0);
        }
        // A name without a match gives nothing either.
        let stranger = named_object(&mut e, "dir\\elsewhere.kf");
        assert_eq!(
            e.call(0x0049_5da0, &args![this, stranger, 0x0105u32]).u32(),
            0
        );
    }

    // ---- PickBestAnimation -----------------------------------------------------------------------------

    /// An animation whose map has playable entries for `keys`; the weapon and
    /// iron-sights group tests are doubles.
    fn pick_fixture(keys: &[u32]) -> (Engine, Ptr<Animation>) {
        let mut e = engine_two();
        let this: Ptr<Animation> = e.new_object();
        let map = map_for(&mut e, this);
        for key in keys {
            let sequence = e.mem.alloc(0x10);
            let entry = map_entry(&mut e, sequence, true);
            map_add(&mut e, map, *key, entry);
        }
        e.register(GROUP_ID_WEAPON, |_, a| ret((a[0] & 0xf00) >> 8));
        e.register(GROUP_ID_IS_IRON_SIGHTS, |_, a| {
            ret((a[0] & 0xf000 == 0xa000) as u32)
        });
        (e, this)
    }

    fn pick(e: &mut Engine, this: Ptr<Animation>, group: u32, flag: u32) -> u32 {
        e.call(0x0049_5740, &args![this, group, flag]).u32() & 0xffff
    }

    #[test]
    fn a_group_in_the_map_is_its_own_best_animation() {
        let (mut e, this) = pick_fixture(&[0x0105]);
        assert_eq!(pick(&mut e, this, 0x0105, 0), 0x0105);
        // An entry without a sequence does not count.
        let (mut e, this) = pick_fixture(&[]);
        let map = e.mem.u32(this.addr() + 0xdc);
        let empty = map_entry(&mut e, 0, true);
        map_add(&mut e, map, 0x0105, empty);
        assert_eq!(pick(&mut e, this, 0x0105, 0), 0);
    }

    #[test]
    fn the_weapon_variants_of_a_group_are_tried_in_order() {
        // Weapon 3 asks for the variant with weapon 2 first, then the base.
        let (mut e, this) = pick_fixture(&[0x0205]);
        assert_eq!(pick(&mut e, this, 0x0305, 0), 0x0205);
        let (mut e, this) = pick_fixture(&[0x0005]);
        assert_eq!(pick(&mut e, this, 0x0305, 0), 0x0005);
        // Weapon 1 asks for the variant with weapon 4.
        let (mut e, this) = pick_fixture(&[0x0405, 0x0005]);
        assert_eq!(pick(&mut e, this, 0x0105, 0), 0x0405);
        // Weapons 2 and 4 go straight to the base.
        let (mut e, this) = pick_fixture(&[0x0005, 0x0205]);
        assert_eq!(pick(&mut e, this, 0x0405, 0), 0x0005);
        let (mut e, this) = pick_fixture(&[0x0005, 0x0405]);
        assert_eq!(pick(&mut e, this, 0x0205, 0), 0x0005);
    }

    #[test]
    fn the_high_bit_group_falls_back_to_the_group_without_it() {
        let (mut e, this) = pick_fixture(&[0x0105]);
        assert_eq!(pick(&mut e, this, 0x8105, 0), 0x0105);
        // Another low byte answer is not taken.
        let (mut e, this) = pick_fixture(&[0x0106]);
        assert_eq!(pick(&mut e, this, 0x8105, 0), 0);
    }

    #[test]
    fn an_iron_sights_group_falls_back_to_the_group_three_lower() {
        let (mut e, this) = pick_fixture(&[0xa105]);
        // 0xa108 is an iron-sights action (double); 0xa105 is playable.
        assert_eq!(pick(&mut e, this, 0xa108, 1), 0xa105);
    }

    #[test]
    fn substitute_groups_for_the_movement_types() {
        // Type 7 stands for type 3 (with the same high bits).
        let (mut e, this) = pick_fixture(&[0x0003]);
        assert_eq!(pick(&mut e, this, 0x0007, 0), 0x0003);
        let (mut e, this) = pick_fixture(&[0x0204]);
        assert_eq!(pick(&mut e, this, 0x0208, 0), 0x0204);
    }

    #[test]
    fn the_last_resorts_depend_on_the_flag() {
        // With the flag set there is nothing after the substitutes.
        let (mut e, this) = pick_fixture(&[0x0100]);
        assert_eq!(pick(&mut e, this, 0x0105, 1), 0);
        // Without it the group with only its high byte is tried.
        assert_eq!(pick(&mut e, this, 0x0105, 0), 0x0100);
        // A zero low byte gives up.
        let (mut e, this) = pick_fixture(&[]);
        assert_eq!(pick(&mut e, this, 0x0100, 0), 0);
        assert_eq!(pick(&mut e, this, 0, 0), 0);
        // Slot-5/6 types give 0 as soon as the map misses.
        let (mut e, this) = pick_fixture(&[0x0100]);
        set_type(&mut e, 5, 5, 0);
        assert_eq!(pick(&mut e, this, 0x0105, 0), 0);
    }

    #[test]
    fn the_players_animation_tries_the_group_without_the_upper_bits() {
        let (mut e, this) = pick_fixture(&[0x0105]);
        set_type(&mut e, 5, 4, 0);
        e.mem.set_u32(this.addr() + 4, PLAYER);
        e.register_double(PLAYER_GET_ANIMATION, move |_, _| ret(this.addr()));
        assert_eq!(pick(&mut e, this, 0x1105, 1), 0x0105);
    }

    // ---- ShouldBeMoving -----------------------------------------------------------------------------------

    #[test]
    fn the_sequence_in_slot_one_is_moving_when_it_wins_the_priority() {
        let mut e = engine_two();
        e.register(SEQUENCE_PRIORITY, |e, a| ret(e.mem.u8(a[0] + 0x70) as u32));
        let this: Ptr<Animation> = e.new_object();
        let bone = named_object(&mut e, "Bip01 Spine");
        let first = e.mem.alloc(0x80);
        let second = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0xe0 + 4, first);
        e.mem.set_u32(this.addr() + 0xe0, second);
        e.mem.set_u32(this.addr() + 0x28 + 4, bone);
        e.mem.set_u16(this.addr() + 0x4e, 0x0005);
        e.mem.set_u8(first + 0x70, 40);
        e.mem.set_u8(second + 0x70, 40);
        // A tie goes to slot 1.
        assert!(e.call(0x0049_5be0, &args![this]).bool());
        e.mem.set_u8(second + 0x70, 50);
        assert!(!e.call(0x0049_5be0, &args![this]).bool());
        // Types outside 3..14 never move.
        e.mem.set_u8(second + 0x70, 10);
        e.mem.set_u16(this.addr() + 0x4e, 0x0002);
        assert!(!e.call(0x0049_5be0, &args![this]).bool());
        e.mem.set_u16(this.addr() + 0x4e, 0x000f);
        assert!(!e.call(0x0049_5be0, &args![this]).bool());
        // No bone or no sequence in slot 1.
        e.mem.set_u16(this.addr() + 0x4e, 0x0005);
        e.mem.set_u32(this.addr() + 0x28 + 4, 0);
        assert!(!e.call(0x0049_5be0, &args![this]).bool());
        e.mem.set_u32(this.addr() + 0x28 + 4, bone);
        e.mem.set_u32(this.addr() + 0xe0 + 4, 0);
        assert!(!e.call(0x0049_5be0, &args![this]).bool());
    }

    // ---- Update ---------------------------------------------------------------------------------------

    /// An animation ready for `Update`: no queued models, empty slots (group
    /// 0xff, no sequences), no pending reload, and doubles for the list pieces.
    fn update_fixture() -> (Engine, Ptr<Animation>) {
        let mut e = engine_two();
        e.register(TIME_SCALE_GET, |_, _| 1.0f32.into_ret());
        e.register(MODEL_QUEUE_NOT_EMPTY, |e, a| {
            ret(!(e.mem.u32(a[0] + 0x104) == 0 && e.mem.u32(a[0] + 0x108) == 0) as u32)
        });
        e.register(ACTOR_FLAGS_ANY, |_, _| ret(0));
        e.register(NODE_FLAG_CHECK, |_, _| ret(0));
        // The first list pieces; the removal double works on the embedded head.
        make_simple_list(&mut e, &[]);
        e.register(SIMPLE_LIST_REMOVE_ITEM, |e, a| {
            let item = e.mem.u32(a[1]);
            let head = a[0];
            if e.mem.u32(head) == item {
                let next = e.mem.u32(head + 4);
                if next == 0 {
                    e.mem.set_u32(head, 0);
                } else {
                    let (next_item, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                    e.mem.set_u32(head, next_item);
                    e.mem.set_u32(head + 4, after);
                }
            } else {
                let mut previous = head;
                loop {
                    let node = e.mem.u32(previous + 4);
                    if node == 0 {
                        break;
                    }
                    if e.mem.u32(node) == item {
                        let after = e.mem.u32(node + 4);
                        e.mem.set_u32(previous + 4, after);
                        break;
                    }
                    previous = node;
                }
            }
            Ret::default()
        });
        let this: Ptr<Animation> = e.new_object();
        e.set(this, Animation::m_fGlobalTimeMultiplier, 1.0f32);
        e.mem
            .set_u16(this.addr() + Animation::sQueuedReloadGroup.off, 0xff);
        e.mem.set_u8(this.addr() + Animation::cSkipUpdate.off, 0xff);
        for slot in 0..8 {
            e.mem.set_u16(this.addr() + 0x4c + 2 * slot, 0xff);
            e.mem.set_u16(this.addr() + 0x9c + 2 * slot, 0xff);
        }
        (e, this)
    }

    /// Fills the `BSSimpleList` whose head node is at `head` with `items`.
    fn fill_list(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    fn update(e: &mut Engine, this: Ptr<Animation>, actor: u32, delta: f32, time_override: f32) {
        e.call(0x0049_1180, &args![this, actor, delta, time_override]);
    }

    #[test]
    fn update_adds_the_queued_models_up_to_the_setting_limit() {
        let (mut e, this) = update_fixture();
        let models = [e.mem.alloc(0x20), e.mem.alloc(0x20), e.mem.alloc(0x20)];
        fill_list(&mut e, this.addr() + 0x104, &models);
        let limit = e.mem.alloc(4);
        e.mem.set_u32(limit, 2);
        e.register_double(SETTING_VALUE_ADDRESS, move |_, a| {
            assert_eq!(a[0], SETTING_CLONING);
            ret(limit)
        });
        // The models have no animation group, so adding one does nothing.
        e.register(KF_MODEL_ANIM_GROUP, |_, _| ret(0));
        e.register(KF_MODEL_RELEASE, |_, _| Ret::default());
        start_log(&mut e);
        update(&mut e, this, 0, 0.1, -1.0);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, KF_MODEL_RELEASE),
            [[models[0]], [models[1]]]
        );
        assert_eq!(e.global::<u32>(QUEUED_MODEL_COUNTER), 2);
        // The third model is still queued, and nothing else happened.
        assert_eq!(e.mem.u32(this.addr() + 0x104), models[2]);
        assert_eq!(e.mem.u32(this.addr() + 0x108), 0);
    }

    #[test]
    fn update_plays_the_queued_reload_group_when_the_weapon_is_drawn() {
        let (mut e, this) = update_fixture();
        let vtable = e.mem.u32(PLAYER);
        e.register(0x00f4_04b0, |_, _| Ret::default());
        e.mem.set_u32(vtable + 0x4b0, 0x00f4_04b0);
        e.mem.set_u16(this.addr() + 0x122, 0x01ff);
        e.mem.set_i32(this.addr() + 0x6c, 1);
        e.set(this, Animation::pActorRef, Ptr::new(ACTOR));
        e.register(ACTOR_IS_WEAPON_DRAWN, |_, _| ret(1));
        e.register(ACTOR_SET_ANIM_ACTION, |_, _| Ret::default());
        start_log(&mut e);
        update(&mut e, this, PLAYER, 0.1, -1.0);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, ACTOR_IS_WEAPON_DRAWN), [[ACTOR]]);
        // The player is told about the group, and the actor gets action 9 (no
        // item in its process) with the sequence of slot 4 (none).
        assert_eq!(arguments_of(&log, 0x00f4_04b0), [[PLAYER, 0x01ff, 1]]);
        assert_eq!(arguments_of(&log, ACTOR_SET_ANIM_ACTION), [[PLAYER, 9, 0]]);
        assert_eq!(e.mem.u16(this.addr() + 0x122), 0xff);
    }

    #[test]
    fn update_keeps_the_queued_reload_group_during_an_attack_and_drops_it_for_other_actions() {
        let (mut e, this) = update_fixture();
        e.mem.set_u16(this.addr() + 0x122, 0x01ff);
        e.mem.set_i32(this.addr() + 0x6c, 2);
        let group = group_with_id(&mut e, 0x0005);
        let sequence = sequence_with_group(&mut e, 1, group);
        e.mem.set_u32(this.addr() + 0xe0 + 16, sequence);
        e.register(GROUP_IS_AIM_OF, |_, _| ret(0));
        e.register(GROUP_IS_ATTACK_OF, |_, _| ret(1));
        e.register(ACTOR_IS_WEAPON_DRAWN, |_, _| {
            panic!("not asked during an attack")
        });
        update(&mut e, this, 0, 0.1, -1.0);
        assert_eq!(e.mem.u16(this.addr() + 0x122), 0x01ff);
        // An aim action does not hold it back; a later stage of action 4 drops it.
        e.register(GROUP_IS_AIM_OF, |_, _| ret(1));
        e.register(ACTOR_IS_WEAPON_DRAWN, |_, _| ret(0));
        update(&mut e, this, 0, 0.1, -1.0);
        assert_eq!(e.mem.u16(this.addr() + 0x122), 0xff);
        e.mem.set_u16(this.addr() + 0x122, 0x01ff);
        e.mem.set_i32(this.addr() + 0x6c, 5);
        update(&mut e, this, 0, 0.1, -1.0);
        assert_eq!(e.mem.u16(this.addr() + 0x122), 0xff);
    }

    #[test]
    fn update_counts_down_the_replay_delays_and_frees_the_expired_ones() {
        let (mut e, this) = update_fixture();
        let (first, second) = (e.mem.alloc(0x10), e.mem.alloc(0x10));
        e.mem.set_f32(first + 4, 0.5);
        e.mem.set_f32(second + 4, 2.0);
        fill_list(&mut e, this.addr() + 0x134, &[first, second]);
        start_log(&mut e);
        update(&mut e, this, 0, 1.0, -1.0);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, OPERATOR_DELETE), [[first]]);
        assert_eq!(e.mem.u32(this.addr() + 0x134), second);
        assert_eq!(e.mem.u32(this.addr() + 0x138), 0);
        assert_eq!(e.mem.f32(second + 4), 1.0);
        // The last delay going away empties the list.
        update(&mut e, this, 0, 5.0, -1.0);
        assert_eq!(e.mem.u32(this.addr() + 0x134), 0);
    }

    #[test]
    fn update_frees_idles_that_are_done_and_reports_animating_ones() {
        let (mut e, this) = update_fixture();
        e.register(ANIM_IDLE_FREE, |_, _| Ret::default());
        e.register(LOG, |_, _| Ret::default());
        // Slot 0: an idle without sequence that has the word at +8.
        let done = e.mem.alloc(0x40);
        e.mem.set_u32(done + 8, 1);
        e.mem.set_u32(this.addr() + 0x12c, done);
        // Slot 1: an idle whose sequence is animating (state 1).
        let busy = e.mem.alloc(0x40);
        let sequence = named_object(&mut e, "walk.kf");
        e.mem.set_u32(sequence + 0x44, 1);
        e.mem.set_u32(busy + 0x18, sequence);
        let owner = object_with_vtable(&mut e, 0x40, &[(0x130, 0x00f5_0130)]);
        e.register(0x00f5_0130, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.mem.set_u32(owner + 0x10, 0x1111);
        e.mem.set_u32(busy + 0x2c, owner);
        e.mem.set_u32(this.addr() + 0x130, busy);
        let reference_vtable = e.mem.u32(ACTOR);
        e.mem.set_u32(reference_vtable + 0x130, 0x00f5_0130);
        e.mem.set_u32(ACTOR + 0x10, 0x2222);
        e.set(this, Animation::pActorRef, Ptr::new(ACTOR));
        start_log(&mut e);
        update(&mut e, this, 0, 0.1, -1.0);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, ANIM_IDLE_FREE),
            [[this.addr(), this.addr() + 0x12c]]
        );
        let name = e.mem.u32(sequence + 8);
        assert_eq!(
            arguments_of(&log, LOG),
            [[LOG_IDLE_FREE_ANIMATING, 0x2222, 0x1111, name]]
        );
        // The animating idle is released.
        assert_eq!(
            arguments_of(&log, NI_POINTER_SET),
            [[this.addr() + 0x130, 0]]
        );
        // An idle whose sequence is idle (state 0) is freed, state 2 is kept.
        e.mem.set_u32(busy + 0x18, sequence);
        e.mem.set_u32(sequence + 0x44, 0);
        e.mem.set_u32(this.addr() + 0x130, busy);
        start_log(&mut e);
        update(&mut e, this, 0, 0.1, -1.0);
        assert!(arguments_of(&end_log(&mut e), ANIM_IDLE_FREE)
            .contains(&vec![this.addr(), this.addr() + 0x130]));
        e.mem.set_u32(sequence + 0x44, 2);
        start_log(&mut e);
        update(&mut e, this, 0, 0.1, -1.0);
        assert!(!arguments_of(&end_log(&mut e), ANIM_IDLE_FREE)
            .contains(&vec![this.addr(), this.addr() + 0x130]));
    }

    #[test]
    fn update_with_cskipupdate_0x14_only_updates_the_root_for_the_time() {
        let (mut e, this) = update_fixture();
        let root = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 8, root);
        e.set(this, Animation::time, 2.5f32);
        e.mem.set_u8(this.addr() + 0xcc, 0x14);
        e.mem.set_u8(this.addr(), 1);
        e.register(UPDATE_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(NODE_UPDATE, |_, _| Ret::default());
        start_log(&mut e);
        update(&mut e, this, 0, 0.1, -1.0);
        let log = end_log(&mut e);
        // The time is the animation's own; the flag bit is cleared and the
        // skip value stays (the function returns early).
        assert_eq!(
            arguments_of(&log, UPDATE_DATA_CONSTRUCT)[0][1..],
            [2.5f32.to_bits(), 0, 0]
        );
        assert_eq!(arguments_of(&log, NODE_UPDATE)[0][0], root);
        assert_eq!(e.mem.u8(this.addr()), 0);
        assert_eq!(e.mem.u8(this.addr() + 0xcc), 0x14);
        assert_eq!(e.mem.f32(this.addr() + 0x10), 0.0);
        // A given time replaces the animation's.
        start_log(&mut e);
        update(&mut e, this, 0, 0.1, 7.0);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, UPDATE_DATA_CONSTRUCT)[0][1..],
            [7.0f32.to_bits(), 0, 0]
        );
        assert_eq!(e.mem.f32(this.addr() + 0xd0), 2.5);
    }

    #[test]
    fn update_with_a_given_time_sets_it_and_updates_the_root_once() {
        let (mut e, this) = update_fixture();
        let root = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 8, root);
        e.set(this, Animation::time, 2.5f32);
        e.register(UPDATE_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(NODE_UPDATE, |_, _| Ret::default());
        start_log(&mut e);
        update(&mut e, this, 0, 0.1, 7.0);
        let log = end_log(&mut e);
        assert_eq!(e.mem.f32(this.addr() + 0xd0), 7.0);
        assert_eq!(
            arguments_of(&log, UPDATE_DATA_CONSTRUCT)[0][1..],
            [7.0f32.to_bits(), 1, 0]
        );
        assert_eq!(arguments_of(&log, NODE_UPDATE)[0][0], root);
        // Without an animation root nothing happens at all.
        e.mem.set_u32(this.addr() + 8, 0);
        start_log(&mut e);
        update(&mut e, this, 0, 0.1, 9.0);
        assert!(arguments_of(&end_log(&mut e), UPDATE_DATA_CONSTRUCT).is_empty());
        assert_eq!(e.mem.f32(this.addr() + 0xd0), 7.0);
    }

    #[test]
    fn update_a_next_group_takes_over_an_empty_slot_before_the_frame() {
        let (mut e, this) = update_fixture();
        let root = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 8, root);
        map_for(&mut e, this);
        e.register(UPDATE_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(NODE_UPDATE, |_, _| Ret::default());
        // Slot 2 has no group but a queued next group (unknown to the map).
        e.mem.set_u16(this.addr() + 0x9c + 4, 0x0105);
        e.mem.set_i32(this.addr() + 0xac + 8, 6);
        update(&mut e, this, 0, 0.1, 5.0);
        assert_eq!(e.mem.i32(this.addr() + 0x7c + 8), 6);
        assert_eq!(e.mem.u16(this.addr() + 0x9c + 4), 0xff);
    }

    #[test]
    fn update_derives_the_frame_movement_from_the_accumulation_root() {
        let (mut e, this) = update_fixture();
        let (root, accum) = (e.mem.alloc(0x80), e.mem.alloc(0x80));
        e.mem.set_u32(this.addr() + 8, root);
        e.set(this, Animation::pAccumRoot, Ptr::new(accum));
        e.set(this, Animation::time, 1.0f32);
        for i in 0..3 {
            e.mem.set_f32(this.addr() + 0x1c + 4 * i, 1.0);
        }
        e.register(UPDATE_BIP_ONLY, |e, a| {
            // The biped update moves the accumulation root's translation.
            for (i, v) in [3.0f32, 4.0, 5.0].iter().enumerate() {
                e.mem.set_f32(a[2] + 4 * i as u32, *v);
            }
            Ret::default()
        });
        e.register(VECTOR_SUBTRACT, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            ret(a[1])
        });
        e.register(REFERENCE_SCALE, |_, _| 2.0f32.into_ret());
        e.mem.set_u8(this.addr(), 1);
        start_log(&mut e);
        update(&mut e, this, ACTOR, 0.5, -1.0);
        let log = end_log(&mut e);
        assert_eq!(e.mem.f32(this.addr() + 0xd0), 1.5);
        assert_eq!(
            arguments_of(&log, UPDATE_BIP_ONLY),
            [[this.addr(), 1.5f32.to_bits(), this.addr() + 0x1c, 1]]
        );
        // (3 - 1, 4 - 1) scaled by the reference's scale 2.
        assert_eq!(e.mem.f32(this.addr() + 0x10), 4.0);
        assert_eq!(e.mem.f32(this.addr() + 0x14), 6.0);
        // The movement flag is cleared and the one-frame values are reset.
        assert_eq!(e.mem.u8(this.addr()), 0);
        assert_eq!(e.mem.u8(this.addr() + 0xcc), 0xff);
    }

    #[test]
    fn update_uses_the_override_movement_when_the_actors_flags_ask_for_it() {
        let (mut e, this) = update_fixture();
        let (root, accum) = (e.mem.alloc(0x80), e.mem.alloc(0x80));
        e.mem.set_u32(this.addr() + 8, root);
        e.set(this, Animation::pAccumRoot, Ptr::new(accum));
        e.set(this, Animation::time, 1.0f32);
        e.set(this, Animation::m_fMoveSpeed, 1.0f32);
        // The actor has a process (virtual function at +0x100).
        let vtable = e.mem.u32(ACTOR);
        e.register(0x00f6_0100, |_, _| ret(1));
        e.mem.set_u32(vtable + 0x100, 0x00f6_0100);
        e.register(ACTOR_FLAGS_ANY, |_, _| ret(1));
        e.register(ACTOR_FLAGS_WORD, |_, _| ret(0x0001));
        // Slot 1 plays a group of type 0xe3, which asks for the override.
        let group = group_with_id(&mut e, 0x00e3);
        let sequence = sequence_with_group(&mut e, 1, group);
        e.mem.set_u32(this.addr() + 0xe0 + 4, sequence);
        e.register(GROUP_IS_SPECIAL_TYPE, |_, _| ret(0));
        let controller = object_with_vtable(&mut e, 0x100, &[(0xc4, 0x00f6_00c4)]);
        e.register(0x00f6_00c4, |_, _| ret(0));
        e.register_double(GET_CONTROLLER, move |_, a| {
            assert_eq!(a[1], RTTI_ACCUM_CONTROLLER);
            ret(controller)
        });
        // No wanted type is found in the process: type 5 comes back.
        e.register(ACTOR_GET_ANIM_GROUP, |_, a| {
            assert_eq!(a[1..], [0, 0, 0, 0]);
            ret(0x0005)
        });
        let map = map_for(&mut e, this);
        let moving_group = group_with_id(&mut e, 0x0005);
        let moving = sequence_with_group(&mut e, 1, moving_group);
        let entry = map_entry(&mut e, moving, true);
        map_add(&mut e, map, 0x0005, entry);
        // The group's movement is (1, 2, 3); time and settings scale it.
        e.register(GROUP_MOVEMENT_VECTOR, |e, a| {
            for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            ret(a[1])
        });
        e.register(VECTOR_SCALE, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) * f32::from_bits(a[2]);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            ret(a[1])
        });
        let settings = e.mem.alloc(4);
        e.mem.set_f32(settings, 1.0);
        e.register_double(SETTING_FLOAT_ADDRESS, move |_, _| ret(settings));
        e.register(UPDATE_BIP_ONLY, |_, _| Ret::default());
        e.register(REFERENCE_SCALE, |_, _| 2.0f32.into_ret());
        e.register(SEQUENCE_STATE, |_, _| ret(0));
        update(&mut e, this, ACTOR, 0.5, -1.0);
        // (1, 2) at time scale 1, then the reference's scale 2.
        assert_eq!(e.mem.f32(this.addr() + 0x10), 2.0);
        assert_eq!(e.mem.f32(this.addr() + 0x14), 4.0);
    }

    /// An animation with a root and one sequence in slot 1 (group type 5, state
    /// 1, cycle type 0, category `category`), `GetTime(i)` = 0.1, 0.4, ... from the
    /// group, `elapsed` the time `00493800` gives.
    fn slot_fixture(category: i32, elapsed: f32) -> (Engine, Ptr<Animation>, u32) {
        let (mut e, this) = update_fixture();
        let root = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 8, root);
        set_type(&mut e, 5, 1, category);
        let group = group_with_id(&mut e, 0x0005);
        let sequence = sequence_with_group(&mut e, 1, group);
        e.mem.set_u32(this.addr() + 0xe0 + 4, sequence);
        e.mem.set_u16(this.addr() + 0x4c + 2, 5);
        e.set(this, Animation::m_fMoveSpeed, 2.0f32);
        e.mem.set_f32(sequence + 0x2c, 1.0);
        e.mem.set_f32(sequence + 0x30, 3.0);
        e.register(SEQUENCE_SCALED_TIME, |_, _| 0.0f32.into_ret());
        e.register(SEQUENCE_ELAPSED, |_, _| 0.0f32.into_ret());
        e.register(SEQUENCE_SET_PHASE, |_, _| Ret::default());
        e.register_double(SEQUENCE_TIME_ON_ANIMATION, move |_, _| elapsed.into_ret());
        e.register(GROUP_TIME, |e, a| {
            e.mem.f32(a[0] + 0x100 + 4 * a[1]).into_ret()
        });
        e.register(BLEND_OUT, |_, _| Ret::default());
        e.register(SEQUENCE_OBJECT_COUNT, |_, _| ret(0));
        (e, this, sequence)
    }

    #[test]
    fn update_steps_a_looping_sequence_and_blends_it_out_at_its_end() {
        let (mut e, this, sequence) = slot_fixture(0, 0.5);
        let group = e.mem.u32(sequence + 0x74);
        e.mem.set_f32(group + 0x100, 0.1);
        e.mem.set_f32(group + 0x104, 0.4);
        start_log(&mut e);
        update(&mut e, this, 0, 0.1, -1.0);
        let log = end_log(&mut e);
        // The speed modifier of the type (move speed 2) moves the phase by the
        // frame time; the intro (0.1 < 0.5) is over, and the end section (0.4
        // <= 0.5) with no loop left blends the slot out.
        assert_eq!(
            arguments_of(&log, SEQUENCE_SET_PHASE),
            [[sequence, 0.1f32.to_bits(), 0]]
        );
        assert_eq!(e.mem.i32(this.addr() + 0x5c + 4), 1);
        assert_eq!(arguments_of(&log, BLEND_OUT), [[this.addr(), 1, 0]]);
        assert_eq!(e.mem.f32(this.addr() + 0xd0), 0.1);
        assert_eq!(e.mem.u8(this.addr() + 0xcc), 0xff);
    }

    #[test]
    fn update_restarts_a_looping_sequence_that_has_loops_left() {
        let (mut e, this, sequence) = slot_fixture(0, 0.5);
        let group = e.mem.u32(sequence + 0x74);
        e.mem.set_f32(group + 0x100, 0.1);
        e.mem.set_f32(group + 0x104, 0.4);
        e.mem.set_i32(this.addr() + 0x7c + 4, 2);
        e.mem.set_i32(this.addr() + 0x5c + 4, 1);
        start_log(&mut e);
        update(&mut e, this, 0, 0.1, -1.0);
        let log = end_log(&mut e);
        // The stage restarts, one loop is used and the phase goes back by the
        // sequence's length (3 - 1 seconds).
        assert_eq!(e.mem.i32(this.addr() + 0x5c + 4), 0);
        assert_eq!(e.mem.i32(this.addr() + 0x7c + 4), 1);
        assert_eq!(
            arguments_of(&log, SEQUENCE_SET_PHASE),
            [
                [sequence, 0.1f32.to_bits(), 0],
                [sequence, (-2.0f32).to_bits(), 0]
            ]
        );
        assert!(arguments_of(&log, BLEND_OUT).is_empty());
    }

    #[test]
    fn update_hands_the_slot_to_the_next_group_at_the_end_of_a_last_stage() {
        let (mut e, this, sequence) = slot_fixture(5, 0.5);
        let group = e.mem.u32(sequence + 0x74);
        e.mem.set_f32(group + 0x100 + 16, 0.2);
        e.mem.set_i32(this.addr() + 0x5c + 4, 4);
        e.mem.set_u16(this.addr() + 0x9c + 2, 0x0106);
        e.mem.set_i32(this.addr() + 0xac + 4, 7);
        map_for(&mut e, this);
        update(&mut e, this, 0, 0.1, -1.0);
        assert_eq!(e.mem.i32(this.addr() + 0x7c + 4), 7);
        assert_eq!(e.mem.u16(this.addr() + 0x9c + 2), 0xff);
        // Without a next group it blends out.
        let (mut e, this, sequence) = slot_fixture(5, 0.5);
        let group = e.mem.u32(sequence + 0x74);
        e.mem.set_f32(group + 0x100 + 16, 0.2);
        e.mem.set_i32(this.addr() + 0x5c + 4, 4);
        start_log(&mut e);
        update(&mut e, this, 0, 0.1, -1.0);
        assert_eq!(
            arguments_of(&end_log(&mut e), BLEND_OUT),
            [[this.addr(), 1, 0]]
        );
    }

    #[test]
    fn update_keeps_the_slots_named_by_cskipupdate_at_their_phase() {
        let (mut e, this, sequence) = slot_fixture(0, 0.5);
        // The skip value 1 is the slot of the sequence: its phase goes back by
        // the frame time and none of the stage logic runs.
        e.mem.set_u8(this.addr() + 0xcc, 1);
        start_log(&mut e);
        update(&mut e, this, 0, 0.1, -1.0);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, SEQUENCE_SET_PHASE),
            [[sequence, (-0.1f32).to_bits(), 0]]
        );
        assert!(arguments_of(&log, BLEND_OUT).is_empty());
        assert_eq!(e.mem.u8(this.addr() + 0xcc), 0xff);
    }

    // ---- Third session: text keys, ClearGroup, the resets and AnimIdle ------------------

    /// Doubles for the text key readers: the key list sits at +0x20 of the
    /// sequence, keys are `{time, text}` pairs.
    fn register_text_key_doubles(e: &mut Engine) {
        e.register(SEQUENCE_TEXT_KEYS, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(TEXT_KEY_TEXT, |_, a| ret(a[0] + 4));
        e.register(TEXT_KEY_TIME, |e, a| e.mem.f32(a[0]).into_ret());
        e.register(SEQUENCE_FLOAT_3C, |e, a| e.mem.f32(a[0] + 0x3c).into_ret());
        e.register(STRING_LENGTH, |e, a| {
            let mut length = 0;
            while e.mem.u8(a[0] + length) != 0 {
                length += 1;
            }
            ret(length)
        });
        e.register(STRNICMP, |e, a| {
            for i in 0..a[2] {
                let left = e.mem.u8(a[0] + i).to_ascii_lowercase();
                let right = e.mem.u8(a[1] + i).to_ascii_lowercase();
                if left != right {
                    return ret(if left < right { u32::MAX } else { 1 });
                }
            }
            ret(0)
        });
        e.register(
            TOLOWER,
            |_, a| ret((a[0] as u8).to_ascii_lowercase() as u32),
        );
        e.mem.write(TEXT_KEY_PREFIX_A, b"a:\0");
    }

    /// A sequence whose text key list holds `keys` (time, text).
    fn text_key_sequence(e: &mut Engine, keys: &[(f32, &str)]) -> u32 {
        let sequence = e.mem.alloc(0x80);
        let list = e.mem.alloc(0x20);
        let entries = e.mem.alloc(8 * keys.len().max(1) as u32);
        for (i, (time, text)) in keys.iter().enumerate() {
            let text = cstring(e, text);
            e.mem.set_f32(entries + 8 * i as u32, *time);
            e.mem.set_u32(entries + 8 * i as u32 + 4, text);
        }
        e.mem.set_u32(list + 0xc, keys.len() as u32);
        e.mem.set_u32(list + 0x10, entries);
        e.mem.set_u32(sequence + 0x20, list);
        sequence
    }

    #[test]
    fn text_key_list_gives_the_entries_and_the_count() {
        let mut e = engine();
        let list = e.mem.alloc(0x20);
        e.mem.set_u32(list + 0xc, 5);
        e.mem.set_u32(list + 0x10, 0x1234);
        let count = e.mem.alloc(4);
        assert_eq!(e.call(0x0049_5f30, &args![list, count]).u32(), 0x1234);
        assert_eq!(e.mem.u32(count), 5);
    }

    #[test]
    fn a_prefix_search_returns_the_character_after_the_first_match() {
        let mut e = engine();
        register_text_key_doubles(&mut e);
        let this: Ptr<Animation> = e.new_object();
        let time = e.mem.alloc(4);
        // No sequence in slot 4.
        assert_eq!(e.call(0x0049_5e40, &args![this, time]).u8(), 0);
        let sequence =
            text_key_sequence(&mut e, &[(1.0, "b:x"), (2.5, "A:Fire"), (3.0, "a:other")]);
        e.mem.set_u32(this.addr() + 0xe0 + 16, sequence);
        // The first key that starts with "a:" (any case) wins; the character
        // after the prefix comes back in lower case.
        assert_eq!(e.call(0x0049_5e40, &args![this, time]).u8(), b'f');
        assert_eq!(e.mem.f32(time), 2.5);
        // Without a time pointer only the character comes back.
        assert_eq!(e.call(0x0049_5e40, &args![this, 0u32]).u8(), b'f');
        // No match.
        let sequence = text_key_sequence(&mut e, &[(1.0, "b:x")]);
        e.mem.set_u32(this.addr() + 0xe0 + 16, sequence);
        e.mem.set_f32(time, 9.0);
        assert_eq!(e.call(0x0049_5e40, &args![this, time]).u8(), 0);
        assert_eq!(e.mem.f32(time), 9.0);
    }

    #[test]
    fn a_timed_prefix_search_skips_keys_before_the_sequence_limit() {
        let mut e = engine();
        register_text_key_doubles(&mut e);
        let this: Ptr<Animation> = e.new_object();
        let time = e.mem.alloc(4);
        e.mem.set_f32(time, 9.0);
        let prefix = cstring(&mut e, "hit:");
        // No sequence in the slot: the time is cleared and nothing is found.
        assert_eq!(
            e.call(0x0049_5f50, &args![this, 3i32, prefix, time]).u8(),
            0
        );
        assert_eq!(e.mem.f32(time), 0.0);
        let sequence = text_key_sequence(
            &mut e,
            &[
                (1.0, "hit:a"),
                (2.0, "other:q"),
                (3.0, "HIT:B"),
                (4.0, "hit:c"),
            ],
        );
        e.mem.set_f32(sequence + 0x3c, 2.0);
        e.mem.set_u32(this.addr() + 0xe0 + 12, sequence);
        // "hit:a" is before the limit 2.0; "HIT:B" at 3.0 is the first accepted.
        assert_eq!(
            e.call(0x0049_5f50, &args![this, 3i32, prefix, time]).u8(),
            b'b'
        );
        assert_eq!(e.mem.f32(time), 3.0);
        // A key exactly at the limit counts.
        e.mem.set_f32(sequence + 0x3c, 3.0);
        assert_eq!(
            e.call(0x0049_5f50, &args![this, 3i32, prefix, 0u32]).u8(),
            b'b'
        );
        // A limit past every key finds nothing; a null prefix never does.
        e.mem.set_f32(sequence + 0x3c, 5.0);
        e.mem.set_f32(time, 9.0);
        assert_eq!(
            e.call(0x0049_5f50, &args![this, 3i32, prefix, time]).u8(),
            0
        );
        assert_eq!(e.mem.f32(time), 0.0);
        assert_eq!(e.call(0x0049_5f50, &args![this, 3i32, 0u32, time]).u8(), 0);
    }

    // ---- ClearGroup

    /// An animation with a manager and every slot holding group 0x10, next
    /// group 0x20, action 7 and 3 loops left.
    fn clear_fixture() -> (Engine, Ptr<Animation>, u32) {
        let mut e = engine();
        e.register(MENU_MODE_TYPE, |_, _| ret(0));
        e.register(SEQUENCE_STATE, |e, a| ret(e.mem.u32(a[0] + 0x44)));
        e.register(SEQUENCE_WORD_58, |e, a| ret(e.mem.u32(a[0] + 0x58)));
        e.register(MANAGER_DEACTIVATE, |_, _| Ret::default());
        let this: Ptr<Animation> = e.new_object();
        let manager = e.mem.alloc(0x7c);
        e.mem.set_u32(this.addr() + 0xd8, manager);
        for slot in 0..8u32 {
            e.mem.set_u16(this.addr() + 0x4c + 2 * slot, 0x10);
            e.mem.set_u16(this.addr() + 0x9c + 2 * slot, 0x20);
            e.mem.set_i32(this.addr() + 0x5c + 4 * slot, 7);
            e.mem.set_i32(this.addr() + 0x7c + 4 * slot, 3);
        }
        (e, this, manager)
    }

    /// The slots whose group is 0xff.
    fn cleared_slots(e: &Engine, this: Ptr<Animation>) -> Vec<u32> {
        (0..8)
            .filter(|slot| e.mem.u16(this.addr() + 0x4c + 2 * slot) == 0xff)
            .collect()
    }

    #[test]
    fn clear_group_deactivates_a_running_sequence_and_empties_the_slot() {
        let (mut e, this, manager) = clear_fixture();
        let sequence = e.mem.alloc(0x80);
        let partner = e.mem.alloc(0x80);
        e.mem.set_u32(sequence + 0x44, 2);
        e.mem.set_u32(sequence + 0x58, partner);
        e.mem.set_u32(this.addr() + 0xe0 + 4, sequence);
        start_log(&mut e);
        e.call(0x0049_6080, &args![this, 1i32, 0.25f32]);
        let log = end_log(&mut e);
        // The partner and then the sequence are deactivated with the blend time.
        assert_eq!(
            arguments_of(&log, MANAGER_DEACTIVATE),
            [
                [manager, partner, 0.25f32.to_bits()],
                [manager, sequence, 0.25f32.to_bits()]
            ]
        );
        // Slot 1 is the movement slot: its sequence is remembered.
        assert_eq!(
            e.get(this, Animation::pLastMovementSequence).addr(),
            sequence
        );
        assert_eq!(e.mem.u32(this.addr() + 0xe0 + 4), 0);
        assert_eq!(cleared_slots(&e, this), [1]);
        assert_eq!(e.mem.u16(this.addr() + 0x9c + 2), 0xff);
        assert_eq!(e.mem.i32(this.addr() + 0x5c + 4), -1);
        // The loop count stays.
        assert_eq!(e.mem.i32(this.addr() + 0x7c + 4), 3);
    }

    #[test]
    fn clear_group_leaves_an_idle_sequence_alone() {
        let (mut e, this, _) = clear_fixture();
        // No state: nothing to deactivate (and no partner is asked for).
        let sequence = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0xe0 + 12, sequence);
        start_log(&mut e);
        e.call(0x0049_6080, &args![this, 3i32, 0.0f32]);
        let log = end_log(&mut e);
        assert!(arguments_of(&log, MANAGER_DEACTIVATE).is_empty());
        assert!(e.get(this, Animation::pLastMovementSequence).is_null());
        assert_eq!(cleared_slots(&e, this), [3]);
        // Without a controller manager nothing is deactivated either.
        let (mut e, this, _) = clear_fixture();
        e.mem.set_u32(this.addr() + 0xd8, 0);
        let sequence = e.mem.alloc(0x80);
        e.mem.set_u32(sequence + 0x44, 2);
        e.mem.set_u32(this.addr() + 0xe0, sequence);
        start_log(&mut e);
        e.call(0x0049_6080, &args![this, 0i32, 0.0f32]);
        assert!(arguments_of(&end_log(&mut e), MANAGER_DEACTIVATE).is_empty());
        assert_eq!(cleared_slots(&e, this), [0]);
    }

    #[test]
    fn clear_group_clears_the_aim_slots_with_slot_4() {
        let (mut e, this, _) = clear_fixture();
        e.call(0x0049_6080, &args![this, 4i32, 0.0f32]);
        assert_eq!(cleared_slots(&e, this), [4, 5, 6]);
        // 0x15 adds slots 2 and 3, and only 3 while a pausing menu is open.
        let (mut e, this, _) = clear_fixture();
        e.call(0x0049_6080, &args![this, 0x15i32, 0.0f32]);
        assert_eq!(cleared_slots(&e, this), [2, 3, 4, 5, 6]);
        let (mut e, this, _) = clear_fixture();
        e.register(MENU_MODE_TYPE, |_, _| ret(1));
        e.call(0x0049_6080, &args![this, 0x15i32, 0.0f32]);
        assert_eq!(cleared_slots(&e, this), [3, 4, 5, 6]);
        // A single slot clears only itself, whatever its number.
        let (mut e, this, _) = clear_fixture();
        e.call(0x0049_6080, &args![this, 7i32, 0.0f32]);
        assert_eq!(cleared_slots(&e, this), [7]);
    }

    #[test]
    fn clear_group_0x14_runs_the_whole_ladder() {
        let (mut e, this, _) = clear_fixture();
        e.call(0x0049_6080, &args![this, 0x14i32, 0.0f32]);
        assert_eq!(cleared_slots(&e, this), [0, 1, 2, 3, 4, 5, 6, 7]);
        // With `cSkipNextBlend` set slot 0 stays.
        let (mut e, this, _) = clear_fixture();
        e.set(this, Animation::cSkipNextBlend, 1u8);
        e.call(0x0049_6080, &args![this, 0x14i32, 0.0f32]);
        assert_eq!(cleared_slots(&e, this), [1, 2, 3, 4, 5, 6, 7]);
        // In a pausing menu slots 7, 0 and 2 stay.
        let (mut e, this, _) = clear_fixture();
        e.register(MENU_MODE_TYPE, |_, _| ret(1));
        e.call(0x0049_6080, &args![this, 0x14i32, 0.0f32]);
        assert_eq!(cleared_slots(&e, this), [1, 3, 4, 5, 6]);
    }

    // ---- The transform resets

    /// Words at `address`, as floats.
    fn set_floats(e: &mut Engine, address: u32, values: &[f32]) {
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(address + 4 * i as u32, *value);
        }
    }

    /// A transform record in an engine whose `FLT_MAX` is 1000, and a
    /// four-float source vector.
    fn transform_fixture() -> (Engine, u32, u32) {
        let mut e = engine();
        e.set_global(FLOAT_MAX, 1000.0f32);
        e.register(QUATERNION_STORE_AT_4, |_, _| Ret::default());
        let record = e.mem.alloc(0x20);
        let vector = e.mem.alloc(0x10);
        set_floats(&mut e, vector, &[1.0, 2.0, 3.0, 4.0]);
        (e, record, vector)
    }

    #[test]
    fn transform_translation_setter_copies_three_floats() {
        let (mut e, record, vector) = transform_fixture();
        start_log(&mut e);
        e.call(0x0049_63b0, &args![record, vector]);
        // The valid flag leaves the first float alone: no quaternion call.
        assert!(arguments_of(&end_log(&mut e), QUATERNION_STORE_AT_4).is_empty());
        assert_eq!(e.mem.f32(record), 1.0);
        assert_eq!(e.mem.f32(record + 8), 3.0);
        assert_eq!(e.mem.f32(record + 12), 0.0);
    }

    #[test]
    fn transform_rotation_setter_copies_four_floats_at_0xc() {
        let (mut e, record, vector) = transform_fixture();
        start_log(&mut e);
        e.call(0x0049_63e0, &args![record, vector]);
        assert!(arguments_of(&end_log(&mut e), QUATERNION_STORE_AT_4).is_empty());
        assert_eq!(e.mem.f32(record + 0xc), 1.0);
        assert_eq!(e.mem.f32(record + 0x18), 4.0);
        assert_eq!(e.mem.f32(record), 0.0);
    }

    #[test]
    fn transform_scale_setter_stores_at_0x1c() {
        let (mut e, record, _) = transform_fixture();
        e.call(0x0049_6420, &args![record, 0.5f32]);
        assert_eq!(e.mem.f32(record + 0x1c), 0.5);
    }

    #[test]
    fn translation_marker_uses_the_lowest_float() {
        let (mut e, record, _) = transform_fixture();
        e.call(0x0049_6440, &args![record, 1u8]);
        assert_eq!(e.mem.f32(record), 0.0);
        e.call(0x0049_6440, &args![record, 0u8]);
        assert_eq!(e.mem.f32(record), -1000.0);
    }

    #[test]
    fn rotation_marker_stores_the_lowest_float_in_the_quaternion() {
        let (mut e, record, _) = transform_fixture();
        start_log(&mut e);
        e.call(0x0049_6470, &args![record, 1u8]);
        assert!(arguments_of(&end_log(&mut e), QUATERNION_STORE_AT_4).is_empty());
        start_log(&mut e);
        e.call(0x0049_6470, &args![record, 0u8]);
        assert_eq!(
            arguments_of(&end_log(&mut e), QUATERNION_STORE_AT_4),
            [[record + 0xc, (-1000.0f32).to_bits()]]
        );
    }

    #[test]
    fn scale_marker_uses_the_lowest_float() {
        let (mut e, record, _) = transform_fixture();
        e.call(0x0049_64a0, &args![record, 1u8]);
        assert_eq!(e.mem.f32(record + 0x1c), 0.0);
        e.call(0x0049_64a0, &args![record, 0u8]);
        assert_eq!(e.mem.f32(record + 0x1c), -1000.0);
    }

    #[test]
    fn interpolator_reset_uses_the_default_or_the_identity_transform() {
        let mut e = engine();
        e.set_global(FLOAT_MAX, 1000.0f32);
        e.map(0x011a_8000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        e.map(0x011f_3000, 0x1000);
        e.map(0x0109_6000, 0x1000);
        set_floats(&mut e, ZERO_VECTOR, &[1.5, 2.5, 3.5]);
        set_floats(&mut e, QUATERNION_IDENTITY, &[0.5, 0.25, 0.125, 0.0625]);
        set_floats(&mut e, RECORD_DEFAULT_TRANSLATE, &[-9.0, -8.0, -7.0]);
        set_floats(&mut e, RECORD_DEFAULT_ROTATE, &[-6.0, -5.0, -4.0, -3.0]);
        set_floats(&mut e, RECORD_DEFAULT_SCALE, &[-2.0]);
        // The byte at +0xe is not 1: the zero vector, the identity quaternion
        // and scale 1.
        let this = e.mem.alloc(0x60);
        e.call(0x0049_6340, &args![this]);
        let words: Vec<f32> = (0..8).map(|i| e.mem.f32(this + 0x30 + 4 * i)).collect();
        assert_eq!(words, [1.5, 2.5, 3.5, 0.5, 0.25, 0.125, 0.0625, 1.0]);
        assert_eq!(e.mem.u8(this + 0x54), 1);
        // The byte at +0xe is 1: the invalid default transform.
        let this = e.mem.alloc(0x60);
        e.mem.set_u8(this + 0xe, 1);
        e.call(0x0049_6340, &args![this]);
        let words: Vec<f32> = (0..8).map(|i| e.mem.f32(this + 0x30 + 4 * i)).collect();
        assert_eq!(words, [-9.0, -8.0, -7.0, -6.0, -5.0, -4.0, -3.0, -2.0]);
        assert_eq!(e.mem.u8(this + 0x54), 1);
    }

    #[test]
    fn accumulation_root_reset_resets_each_matching_controller_target() {
        let mut e = engine();
        e.map(0x011a_9000, 0x1000);
        e.register(OBJECT_CONTROLLERS, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(CONTROLLER_NEXT, |e, a| ret(e.mem.u32(a[0] + 0x30)));
        e.register(ACCUM_ROOT_SETUP, |_, _| Ret::default());
        e.register(NODE_SET_WORLD_TRANSLATION, |_, _| Ret::default());
        e.register(0x00f0_00c4, |e, a| ret(e.mem.u32(a[0] + 0x38)));
        let new_controller = |e: &mut Engine, target: u32| {
            let controller = object_with_vtable(e, 0x40, &[(SLOT_CONTROLLER_TARGET, 0x00f0_00c4)]);
            e.mem.set_u32(controller + 0x38, target);
            controller
        };
        // Four targets: the second controller does not cast; the third has no
        // target; the fourth's target does not cast.
        let targets: Vec<u32> = (0..4).map(|_| e.mem.alloc(0x60)).collect();
        let controllers = [
            new_controller(&mut e, targets[0]),
            new_controller(&mut e, targets[1]),
            new_controller(&mut e, 0),
            new_controller(&mut e, targets[3]),
        ];
        for pair in controllers.windows(2) {
            e.mem.set_u32(pair[0] + 0x30, pair[1]);
        }
        let root = e.mem.alloc(0x80);
        e.mem.set_u32(root + 0xc, controllers[0]);
        let kind_ok = vec![controllers[0], controllers[2], controllers[3]];
        let target_ok = vec![targets[0], targets[1]];
        e.register_double(DYNAMIC_CAST, move |_, a| {
            let list = if a[0] == RTTI_CONTROLLER_KIND {
                &kind_ok
            } else {
                &target_ok
            };
            ret(if list.contains(&a[1]) { a[1] } else { 0 })
        });
        let this: Ptr<Animation> = e.new_object();
        e.set(this, Animation::pAccumRoot, Ptr::new(root));
        e.mem.set_u32(this.addr() + 0x1c, 0x9999);
        set_floats(&mut e, ZERO_VECTOR, &[1.0, 2.0, 3.0]);
        start_log(&mut e);
        e.call(0x0049_6280, &args![this]);
        let log = end_log(&mut e);
        assert_eq!(e.mem.f32(this.addr() + 0x1c), 1.0);
        assert_eq!(e.mem.f32(this.addr() + 0x24), 3.0);
        assert_eq!(
            arguments_of(&log, ACCUM_ROOT_SETUP),
            [[root, ACCUM_ROOT_SETUP_ARGUMENT]]
        );
        assert_eq!(
            arguments_of(&log, NODE_SET_WORLD_TRANSLATION),
            [[root, ZERO_VECTOR]]
        );
        // Only the first target passed both casts (the second controller's
        // does not cast).
        let reset: Vec<u8> = targets.iter().map(|t| e.mem.u8(t + 0x54)).collect();
        assert_eq!(reset, [1, 0, 0, 0]);

        // Without an accumulation root nothing happens.
        e.set(this, Animation::pAccumRoot, Ptr::NULL);
        start_log(&mut e);
        e.call(0x0049_6280, &args![this]);
        assert_eq!(called(&mut e), Vec::<u32>::new());
    }

    #[test]
    fn accumulation_root_translate_is_set_to_the_zero_vector() {
        let mut e = engine();
        set_floats(&mut e, ZERO_VECTOR, &[1.0, 2.0, 3.0]);
        let this: Ptr<Animation> = e.new_object();
        e.call(0x0049_64d0, &args![this]);
        let words: Vec<f32> = (0..3)
            .map(|i| e.mem.f32(this.addr() + 0x1c + 4 * i))
            .collect();
        assert_eq!(words, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn get_tes_anim_group_asks_the_entry_for_its_current_sequence() {
        let mut e = engine();
        e.register(MAP_GET_AT, |e, a| {
            if a[1] != 7 {
                return ret(0);
            }
            e.mem.set_u32(a[2], e.mem.u32(a[0] + 4));
            ret(1)
        });
        e.register(0x00f0_0010, |e, a| ret(e.mem.u32(a[0] + 8)));
        let group = e.mem.alloc(0x20);
        let sequence = e.mem.alloc(0x80);
        e.mem.set_u32(sequence + 0x74, group);
        let entry = object_with_vtable(&mut e, 0x10, &[(0x10, 0x00f0_0010)]);
        e.mem.set_u32(entry + 8, sequence);
        let map = e.mem.alloc(8);
        e.mem.set_u32(map + 4, entry);
        let this: Ptr<Animation> = e.new_object();
        e.set(this, Animation::pAnimSequenceMap, Ptr::new(map));
        start_log(&mut e);
        assert_eq!(e.call(0x0049_6500, &args![this, 7u16]).u32(), group);
        // The entry is asked for the sequence index -1.
        assert_eq!(
            arguments_of(&end_log(&mut e), 0x00f0_0010),
            [[entry, 0xffff_ffff]]
        );
        // An unknown key gives null.
        assert_eq!(e.call(0x0049_6500, &args![this, 8u16]).u32(), 0);
    }

    #[test]
    fn lowest_object_value_goes_up_to_the_root_object() {
        let mut e = engine();
        e.register(OBJECT_VALUE, |e, a| e.mem.f32(a[0] + 0x30).into_ret());
        e.register(OBJECT_PARENT, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        let chain: Vec<u32> = (0..4).map(|_| e.mem.alloc(0x40)).collect();
        // chain[3] -> chain[2] -> chain[1] (the root) -> chain[0].
        for (i, value) in [0.1f32, 0.5, 2.0, 0.9].iter().enumerate() {
            e.mem.set_f32(chain[i] + 0x30, *value);
        }
        for i in 1..4 {
            e.mem.set_u32(chain[i] + 0x18, chain[i - 1]);
        }
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, chain[1]);
        // The parent of the root, chain[0] (0.1), is not looked at.
        let lowest = e.call(0x0049_6550, &args![this, chain[3]]).f32();
        assert_eq!(lowest, 0.5);
        // Values above 1 never raise the result; no object gives 1.
        e.mem.set_f32(chain[1] + 0x30, 3.0);
        assert_eq!(e.call(0x0049_6550, &args![this, chain[3]]).f32(), 0.9);
        assert_eq!(e.call(0x0049_6550, &args![this, 0u32]).f32(), 1.0);
    }

    // ---- AnimIdle

    /// An actor whose animation (virtual slot 0x1e4) is `animation`.
    fn actor_with_animation(e: &mut Engine, animation: u32) -> u32 {
        e.register(0x00f0_01e4, |e, a| ret(e.mem.u32(a[0] + 8)));
        let actor = object_with_vtable(e, 0x400, &[(SLOT_ACTOR_ANIMATION, 0x00f0_01e4)]);
        e.mem.set_u32(actor + 8, animation);
        actor
    }

    /// Everything `AnimIdle::AnimIdle` touches. The idle form's model name
    /// is `"x.kf"`, its first animation object is `objects[0]`.
    struct IdleBuild {
        e: Engine,
        idle: Ptr<AnimIdle>,
        idle_form: u32,
        actor: u32,
        animation: u32,
        objects: [u32; 2],
        kf: u32,
    }

    fn idle_build(kf: u32, first_object: bool) -> IdleBuild {
        let mut e = engine();
        e.register(INTERLOCKED_INCREMENT, |_, _| ret(1));
        e.register(KF_MODEL_POINTER_INIT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_DESTRUCT, |_, _| Ret::default());
        e.register(KF_MODEL_POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(KF_MODEL_ADD_REF, |_, _| Ret::default());
        e.register(MANAGER_PALETTE, |_, _| ret(0x8000));
        e.register(PALETTE_ADD, |_, _| Ret::default());
        e.register(LOAD_AND_ATTACH_ADD_ON, |_, _| ret(0x7000));
        let text = cstring(&mut e, "Meshes\\x.kf");
        e.register_double(STRING_FORMAT, move |e, a| {
            e.mem.set_u32(a[0], text);
            Ret::default()
        });
        e.register_double(MODEL_LOADER_LOAD_IDLE_KF, move |_, _| ret(kf));
        e.register_double(MODEL_LOADER_LOAD_KF, move |_, _| ret(kf));
        let first = if first_object { e.mem.alloc(0x40) } else { 0 };
        e.register_double(IDLE_FORM_ANIM_OBJECT, move |_, a| {
            ret(if a[2] == 0 { first } else { 0 })
        });
        // The idle form's name sub-object (+0x18) answers with the model name.
        let name = cstring(&mut e, "x.kf");
        e.register_double(0x00f0_0014, move |_, _| ret(name));
        let idle_form = e.mem.alloc(0x40);
        let sub_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(sub_vtable + SLOT_MODEL_NAME, 0x00f0_0014);
        e.mem.set_u32(idle_form + 0x18, sub_vtable);
        let animation = e.mem.alloc(0x140);
        e.mem.set_u32(animation + 0xd8, 0x6000);
        let actor = actor_with_animation(&mut e, animation);
        let idle = e.new_object::<AnimIdle>();
        IdleBuild {
            e,
            idle,
            idle_form,
            actor,
            animation,
            objects: [first, 0],
            kf,
        }
    }

    impl IdleBuild {
        fn construct(&mut self, load_kf: u8) -> Ptr<AnimIdle> {
            let args = args![
                self.idle,
                self.idle_form,
                7u32,
                2u32,
                self.actor,
                load_kf,
                self.animation
            ];
            self.e.call(0x0049_65d0, &args).ptr()
        }
    }

    #[test]
    fn idle_constructor_loads_the_model_and_attaches_the_add_ons() {
        let mut b = idle_build(0x5000, true);
        start_log(&mut b.e);
        assert_eq!(b.construct(0), b.idle);
        let log = end_log(&mut b.e);
        let a = b.idle.addr();
        // Members.
        assert_eq!(b.e.mem.u32(a), VTABLE_ANIM_IDLE);
        assert_eq!(b.e.get(b.idle, AnimIdle::eSection), 7);
        assert_eq!(b.e.get(b.idle, AnimIdle::eType), 2);
        assert_eq!(b.e.get(b.idle, AnimIdle::pIdleForm).addr(), b.idle_form);
        assert_eq!(b.e.get(b.idle, AnimIdle::pActor).addr(), b.actor);
        assert_eq!(b.e.get(b.idle, AnimIdle::pAnimation).addr(), b.animation);
        assert_eq!(b.e.mem.u32(a + 0x1c), b.objects[0]);
        assert_eq!(b.e.mem.u32(a + 0x20), 0);
        assert_eq!(b.e.get(b.idle, AnimIdle::spKFModel).addr(), b.kf);
        assert_eq!(b.e.get(b.idle, AnimIdle::eFlags), 1);
        assert_eq!(b.e.mem.u32(a + 0x24), 0x7000);
        assert_eq!(b.e.mem.u8(b.actor + 0x14e), 1);
        // The model path is built from the folder and the model name, and the
        // idle KF is loaded with the idle and the actor.
        let format = arguments_of(&log, STRING_FORMAT);
        assert_eq!(format[0][1..3], [PATH_FORMAT, MESHES_FOLDER]);
        assert_eq!(
            arguments_of(&log, MODEL_LOADER_LOAD_IDLE_KF)[0][2..],
            [a, 0, b.actor]
        );
        assert!(arguments_of(&log, MODEL_LOADER_LOAD_KF).is_empty());
        // The add-on object is attached and put in the manager's palette.
        assert_eq!(
            arguments_of(&log, LOAD_AND_ATTACH_ADD_ON),
            [[b.objects[0], b.objects[0] + 0x18, 0xffff_ffff, b.actor, 0]]
        );
        assert_eq!(arguments_of(&log, MANAGER_PALETTE), [[0x6000]]);
        assert_eq!(arguments_of(&log, PALETTE_ADD), [[0x7000, 0x8000]]);
        // The model gets its extra reference and the object is counted.
        assert_eq!(arguments_of(&log, KF_MODEL_ADD_REF), [[b.kf]]);
        assert_eq!(
            arguments_of(&log, INTERLOCKED_INCREMENT),
            [[NI_REF_OBJECT_COUNT]]
        );
        assert_eq!(arguments_of(&log, STRING_DESTRUCT).len(), 1);
    }

    #[test]
    fn idle_constructor_with_the_kf_flag_loads_through_load_kf() {
        let mut b = idle_build(0x5000, false);
        start_log(&mut b.e);
        b.construct(1);
        let log = end_log(&mut b.e);
        assert!(arguments_of(&log, MODEL_LOADER_LOAD_IDLE_KF).is_empty());
        assert_eq!(arguments_of(&log, MODEL_LOADER_LOAD_KF).len(), 1);
        // No extra reference, no add-on without an animation object.
        assert!(arguments_of(&log, KF_MODEL_ADD_REF).is_empty());
        assert!(arguments_of(&log, LOAD_AND_ATTACH_ADD_ON).is_empty());
        assert_eq!(b.e.get(b.idle, AnimIdle::eFlags), 1);
    }

    #[test]
    fn idle_constructor_without_a_model_is_not_loaded() {
        let mut b = idle_build(0, true);
        start_log(&mut b.e);
        b.construct(0);
        let log = end_log(&mut b.e);
        assert_eq!(b.e.get(b.idle, AnimIdle::eFlags), 0);
        assert!(arguments_of(&log, LOAD_AND_ATTACH_ADD_ON).is_empty());
        assert!(arguments_of(&log, KF_MODEL_ADD_REF).is_empty());
    }

    #[test]
    fn ni_ref_object_helpers_count_the_objects() {
        let mut e = engine();
        e.register(INTERLOCKED_INCREMENT, |_, _| ret(1));
        e.register(INTERLOCKED_DECREMENT, |_, _| ret(0));
        let this = Ptr::<()>::new(e.mem.alloc(8));
        e.mem.set_u32(this.addr() + 4, 9);
        start_log(&mut e);
        assert_eq!(e.call(0x0049_68b0, &args![this]).ptr::<()>(), this);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_NI_REF_OBJECT);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(
            arguments_of(&end_log(&mut e), INTERLOCKED_INCREMENT),
            [[NI_REF_OBJECT_COUNT]]
        );
        // The destructor body and the scalar deleting destructor.
        e.mem.set_u32(this.addr(), 0x1234);
        start_log(&mut e);
        e.call(0x0049_6910, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE_NI_REF_OBJECT);
        assert_eq!(
            arguments_of(&end_log(&mut e), INTERLOCKED_DECREMENT),
            [[NI_REF_OBJECT_COUNT]]
        );
        e.register(NI_OPERATOR_DELETE, |_, _| Ret::default());
        start_log(&mut e);
        assert_eq!(e.call(0x0049_68e0, &args![this, 0u32]).ptr::<()>(), this);
        assert_eq!(called(&mut e), [INTERLOCKED_DECREMENT]);
        start_log(&mut e);
        e.call(0x0049_68e0, &args![this, 1u32]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, NI_OPERATOR_DELETE), [[this.addr(), 8]]);
    }

    #[test]
    fn animation_manager_getter_reads_the_pointer_at_0xd8() {
        let mut e = engine();
        let this: Ptr<Animation> = e.new_object();
        e.mem.set_u32(this.addr() + 0xd8, 0x6000);
        assert_eq!(e.call(0x0049_6940, &args![this]).u32(), 0x6000);
    }

    #[test]
    fn actor_byte_setter_stores_at_0x14e() {
        let mut e = engine();
        let actor = e.mem.alloc(0x200);
        e.call(0x0049_6960, &args![actor, 1u8]);
        assert_eq!(e.mem.u8(actor + 0x14e), 1);
        e.call(0x0049_6960, &args![actor, 0u8]);
        assert_eq!(e.mem.u8(actor + 0x14e), 0);
    }

    #[test]
    fn animation_sequence_map_getter_reads_the_pointer_at_0xdc() {
        let mut e = engine();
        let this: Ptr<Animation> = e.new_object();
        e.set(this, Animation::pAnimSequenceMap, Ptr::new(0x4321));
        assert_eq!(e.call(0x0049_7280, &args![this]).u32(), 0x4321);
    }

    #[test]
    fn skip_next_blend_setter_sets_the_flag() {
        let mut e = engine();
        let this: Ptr<Animation> = e.new_object();
        e.call(0x0049_74a0, &args![this]);
        assert_eq!(e.get(this, Animation::cSkipNextBlend), 1);
    }

    /// Doubles for the calls `AnimIdle`'s detach function makes; the actor's
    /// process answers slot 0x3e8 with `held`.
    fn detach_fixture(add_on: bool, actor_set: bool) -> (Engine, Ptr<AnimIdle>, u32, u32) {
        let mut e = engine();
        e.register(TASK_QUEUE_ACTIVE, |_, _| ret(0));
        e.register(TASK_QUEUE, |_, _| ret(0x5100));
        e.register(TASK_QUEUE_ATTACH, |_, _| Ret::default());
        e.register(TASK_QUEUE_DETACH, |_, _| Ret::default());
        e.register(SHADOW_SCENE_NODE, |_, _| ret(0x5200));
        e.register(SHADOW_SCENE_NODE_REMOVE, |_, _| Ret::default());
        e.register(MANAGER_PALETTE, |_, _| ret(0x8000));
        e.register(PALETTE_REMOVE, |_, _| Ret::default());
        e.register(ANIMATION_ADD_ON_REMOVED, |_, _| Ret::default());
        e.register(OBJECT_PARENT, |e, a| ret(e.mem.u32(a[0] + 0x18)));
        e.register(MODEL_LOADER_DROP_MODEL, |_, _| Ret::default());
        e.register(MODEL_LOADER_DROP_IDLE, |_, _| Ret::default());
        e.register(SEQUENCE_RELEASE_HELPER, |_, _| Ret::default());
        e.register(PIPBOY_QUERY, |_, _| ret(0));
        e.register(ACTOR_GET_ANIM_ACTION, |_, _| ret(0));
        e.register(ACTOR_SET_ANIM_ACTION, |_, _| Ret::default());
        e.register(0x00f0_00e8, |_, _| Ret::default());
        e.register(0x00f0_0014, |e, _| ret(e.mem.u32(0x2000_0000)));
        let animation = e.mem.alloc(0x140);
        e.mem.set_u32(animation + 0xd8, 0x6000);
        let actor = actor_with_animation(&mut e, animation);
        let idle = e.new_object::<AnimIdle>();
        let a = idle.addr();
        if actor_set {
            e.mem.set_u32(a + 0x34, actor);
        }
        // The add-on object (its parent has a detach-child slot) and the
        // animation object whose name sub-object answers slot 0x14.
        let parent = object_with_vtable(&mut e, 0x40, &[(SLOT_DETACH_CHILD, 0x00f0_00e8)]);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0x18, parent);
        let sub_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(sub_vtable + SLOT_MODEL_NAME, 0x00f0_0014);
        let animation_object = e.mem.alloc(0x40);
        e.mem.set_u32(animation_object + 0x18, sub_vtable);
        e.map(0x2000_0000, 0x1000);
        e.mem.set_u32(0x2000_0000, 0xabc0);
        if add_on {
            e.mem.set_u32(a + 0x24, object);
            e.mem.set_u32(a + 0x1c, animation_object);
        }
        (e, idle, object, parent)
    }

    #[test]
    fn detaching_an_idle_removes_its_add_on_through_the_scene() {
        let (mut e, idle, object, parent) = detach_fixture(true, true);
        let actor = e.get(idle, AnimIdle::pActor).addr();
        let a = idle.addr();
        start_log(&mut e);
        e.call(0x0049_6a50, &args![idle]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, SHADOW_SCENE_NODE_REMOVE),
            [[0x5200, object]]
        );
        assert!(arguments_of(&log, TASK_QUEUE_ATTACH).is_empty());
        assert_eq!(arguments_of(&log, PALETTE_REMOVE), [[object, 0x8000]]);
        let animation = e.mem.u32(actor + 8);
        assert_eq!(
            arguments_of(&log, ANIMATION_ADD_ON_REMOVED),
            [[animation, object]]
        );
        // The parent drops the child; the model of the animation object is
        // dropped by name and the pointer cleared.
        assert_eq!(arguments_of(&log, 0x00f0_00e8), [[parent, object]]);
        assert_eq!(
            arguments_of(&log, MODEL_LOADER_DROP_MODEL),
            [[0x2222_0000, 0xabc0]]
        );
        assert_eq!(e.mem.u32(a + 0x24), 0);
        // The state is still 0 here, so the idle is dropped from the loader,
        // and the actor is forgotten.
        assert_eq!(
            arguments_of(&log, MODEL_LOADER_DROP_IDLE),
            [[0x2222_0000, a]]
        );
        assert_eq!(e.get(idle, AnimIdle::pActor).addr(), 0);
    }

    #[test]
    fn detaching_an_idle_uses_the_task_queue_when_it_takes_the_work() {
        let (mut e, idle, object, _) = detach_fixture(true, true);
        e.register(TASK_QUEUE_ACTIVE, |_, _| ret(1));
        e.set(idle, AnimIdle::eFlags, 2u32);
        start_log(&mut e);
        e.call(0x0049_6a50, &args![idle]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, TASK_QUEUE_ATTACH), [[0x5100, object, 0]]);
        assert_eq!(arguments_of(&log, TASK_QUEUE_DETACH), [[0x5100, object]]);
        assert!(arguments_of(&log, SHADOW_SCENE_NODE_REMOVE).is_empty());
        assert!(arguments_of(&log, 0x00f0_00e8).is_empty());
        // A state other than 0 is not dropped from the loader.
        assert!(arguments_of(&log, MODEL_LOADER_DROP_IDLE).is_empty());
    }

    #[test]
    fn detaching_an_idle_without_an_actor_only_unloads() {
        let (mut e, idle, object, parent) = detach_fixture(true, false);
        start_log(&mut e);
        e.call(0x0049_6a50, &args![idle]);
        let log = end_log(&mut e);
        assert!(arguments_of(&log, PALETTE_REMOVE).is_empty());
        assert!(arguments_of(&log, ANIMATION_ADD_ON_REMOVED).is_empty());
        // The shared tail still runs: the parent drops the child.
        assert_eq!(arguments_of(&log, 0x00f0_00e8), [[parent, object]]);
        assert_eq!(arguments_of(&log, MODEL_LOADER_DROP_MODEL).len(), 1);
        // Nothing for a null idle.
        start_log(&mut e);
        e.call(0x0049_6a50, &args![0u32]);
        assert_eq!(called(&mut e), Vec::<u32>::new());
    }

    #[test]
    fn detaching_an_idle_clears_the_actor_anim_action_it_set() {
        let (mut e, idle, _, _) = detach_fixture(false, true);
        let actor = e.get(idle, AnimIdle::pActor).addr();
        e.register(ACTOR_GET_ANIM_ACTION, |_, _| ret(0xd));
        // The actor's process holds (slot 0x3e8) the idle's sequence.
        e.register(ACTOR_PROCESS, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(0x00f0_03e8, |e, a| ret(e.mem.u32(a[0] + 8)));
        let process = object_with_vtable(&mut e, 0x40, &[(SLOT_PROCESS_HELD_OBJECT, 0x00f0_03e8)]);
        e.mem.set_u32(process + 8, 0x4242);
        e.mem.set_u32(actor + 0x10, process);
        e.mem.set_u32(idle.addr() + 0x18, 0x4242);
        e.set(idle, AnimIdle::eFlags, 1u32);
        start_log(&mut e);
        e.call(0x0049_6a50, &args![idle]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, ACTOR_SET_ANIM_ACTION),
            [[actor, 0xffff_ffff, 0]]
        );
        // Another held object leaves the action alone.
        let (mut e, idle, _, _) = detach_fixture(false, true);
        let actor = e.get(idle, AnimIdle::pActor).addr();
        e.register(ACTOR_GET_ANIM_ACTION, |_, _| ret(0xd));
        e.register(ACTOR_PROCESS, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(0x00f0_03e8, |e, a| ret(e.mem.u32(a[0] + 8)));
        let process = object_with_vtable(&mut e, 0x40, &[(SLOT_PROCESS_HELD_OBJECT, 0x00f0_03e8)]);
        e.mem.set_u32(process + 8, 0x1111);
        e.mem.set_u32(actor + 0x10, process);
        e.mem.set_u32(idle.addr() + 0x18, 0x4242);
        e.set(idle, AnimIdle::eFlags, 1u32);
        start_log(&mut e);
        e.call(0x0049_6a50, &args![idle]);
        assert!(arguments_of(&end_log(&mut e), ACTOR_SET_ANIM_ACTION).is_empty());
    }

    #[test]
    fn idle_destructors_tear_the_members_down_in_order() {
        let (mut e, idle, _, _) = detach_fixture(false, false);
        e.register(INTERLOCKED_DECREMENT, |_, _| ret(0));
        e.register(VECTOR_DESTRUCT, |_, _| Ret::default());
        e.register(NI_POINTER_RELEASE, |_, _| Ret::default());
        e.register(KF_MODEL_POINTER_RELEASE, |_, _| Ret::default());
        e.register(NI_OPERATOR_DELETE, |_, _| Ret::default());
        let a = idle.addr();
        start_log(&mut e);
        assert_eq!(e.call(0x0049_69b0, &args![idle]).u32(), 0);
        let log = end_log(&mut e);
        assert_eq!(e.mem.u32(a), VTABLE_NI_REF_OBJECT);
        let addresses: Vec<u32> = log.iter().map(|(address, _)| *address).collect();
        // The detach function runs first (it drops the idle from the loader),
        // then the members, then the `NiRefObject` base.
        let tail: Vec<u32> = addresses
            .iter()
            .copied()
            .filter(|address| {
                [
                    MODEL_LOADER_DROP_IDLE,
                    VECTOR_DESTRUCT,
                    NI_POINTER_RELEASE,
                    KF_MODEL_POINTER_RELEASE,
                    INTERLOCKED_DECREMENT,
                ]
                .contains(address)
            })
            .collect();
        assert_eq!(
            tail,
            [
                MODEL_LOADER_DROP_IDLE,
                VECTOR_DESTRUCT,
                NI_POINTER_RELEASE,
                KF_MODEL_POINTER_RELEASE,
                INTERLOCKED_DECREMENT
            ]
        );
        assert_eq!(
            arguments_of(&log, VECTOR_DESTRUCT),
            [[a + 0x24, 4, 2, NI_POINTER_RELEASE]]
        );
        assert_eq!(arguments_of(&log, NI_POINTER_RELEASE), [[a + 0x18]]);
        assert_eq!(arguments_of(&log, KF_MODEL_POINTER_RELEASE), [[a + 0x10]]);
        // The scalar deleting destructor frees 0x38 bytes on request.
        start_log(&mut e);
        e.call(0x0049_6980, &args![idle, 0u32]);
        assert!(arguments_of(&end_log(&mut e), NI_OPERATOR_DELETE).is_empty());
        start_log(&mut e);
        assert_eq!(e.call(0x0049_6980, &args![idle, 1u32]).u32(), a);
        assert_eq!(
            arguments_of(&end_log(&mut e), NI_OPERATOR_DELETE),
            [[a, 0x38]]
        );
    }

    // ---- Loaded and the state steps

    /// An idle with an actor, ready for the `Loaded` tests.
    fn loaded_fixture(play_type: u32) -> (Engine, Ptr<AnimIdle>, u32, u32) {
        let mut e = engine();
        e.register(KF_MODEL_POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(KF_MODEL_ADD_REF, |_, _| Ret::default());
        e.register(SPECIAL_IDLE_CHECK, |_, _| ret(1));
        e.register(SPECIAL_IDLE_FREE, |_, _| Ret::default());
        e.register(SPECIAL_IDLE_REPLACE_OV2, |_, _| ret(1));
        e.register(ACTOR_SET_ANIM_ACTION, |_, _| Ret::default());
        let animation = e.mem.alloc(0x140);
        let actor = actor_with_animation(&mut e, animation);
        let idle = e.new_object::<AnimIdle>();
        e.set(idle, AnimIdle::pActor, Ptr::new(actor));
        e.set(idle, AnimIdle::eType, play_type);
        (e, idle, actor, animation)
    }

    #[test]
    fn loaded_without_a_model_resets_the_state() {
        let (mut e, idle, _, _) = loaded_fixture(0);
        e.set(idle, AnimIdle::eFlags, 2u32);
        start_log(&mut e);
        e.call(0x0049_6cd0, &args![idle, 0u32]);
        let log = end_log(&mut e);
        assert_eq!(e.get(idle, AnimIdle::eFlags), 0);
        assert_eq!(
            arguments_of(&log, KF_MODEL_POINTER_SET),
            [[idle.addr() + 0x10, 0]]
        );
        // The scope guard of source line 0x1101 wraps it.
        assert_eq!(
            arguments_of(&log, SCOPE_GUARD_CTOR)[0][1..],
            [SCOPE_GUARD_TAG, 1, SOURCE_FILE, 0x1101]
        );
        assert_eq!(log.last().unwrap().0, SCOPE_GUARD_DTOR);
    }

    #[test]
    fn loaded_type_3_tells_the_actor_to_play_the_sequence() {
        let (mut e, idle, actor, animation) = loaded_fixture(3);
        e.mem.set_u32(idle.addr() + 0x18, 0x4242);
        e.set(idle, AnimIdle::pAnimation, Ptr::new(animation));
        start_log(&mut e);
        e.call(0x0049_6cd0, &args![idle, 0x5000u32]);
        let log = end_log(&mut e);
        assert_eq!(e.get(idle, AnimIdle::eFlags), 1);
        assert_eq!(arguments_of(&log, KF_MODEL_ADD_REF), [[0x5000]]);
        assert_eq!(arguments_of(&log, SPECIAL_IDLE_CHECK), [[animation]]);
        assert_eq!(
            arguments_of(&log, ACTOR_SET_ANIM_ACTION),
            [[actor, 0xd, 0x4242]]
        );
        assert!(arguments_of(&log, SPECIAL_IDLE_FREE).is_empty());
    }

    #[test]
    fn loaded_type_2_frees_the_special_idle_when_the_check_fails() {
        let (mut e, idle, _, _) = loaded_fixture(2);
        e.register(SPECIAL_IDLE_CHECK, |_, _| ret(0));
        // The animation comes from the actor when the idle has none.
        start_log(&mut e);
        e.call(0x0049_6cd0, &args![idle, 0x5000u32]);
        let log = end_log(&mut e);
        let animation = e.mem.u32(e.get(idle, AnimIdle::pActor).addr() + 8);
        assert_eq!(arguments_of(&log, SPECIAL_IDLE_FREE), [[animation, 1, 0]]);
        assert!(arguments_of(&log, ACTOR_SET_ANIM_ACTION).is_empty());
    }

    #[test]
    fn loaded_idle_without_an_animation_deletes_itself() {
        let (mut e, idle, actor, _) = loaded_fixture(3);
        e.mem.set_u32(actor + 8, 0);
        e.register(0x00f0_0000, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x00f0_0000);
        e.mem.set_u32(idle.addr(), vtable);
        start_log(&mut e);
        e.call(0x0049_6cd0, &args![idle, 0x5000u32]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, 0x00f0_0000), [[idle.addr(), 1]]);
        assert!(arguments_of(&log, SPECIAL_IDLE_CHECK).is_empty());
        assert_eq!(log.last().unwrap().0, SCOPE_GUARD_DTOR);
    }

    #[test]
    fn loaded_type_0_asks_the_animation_to_replace_the_idle() {
        let (mut e, idle, _, animation) = loaded_fixture(0);
        e.set(idle, AnimIdle::pAnimation, Ptr::new(animation));
        start_log(&mut e);
        e.call(0x0049_6cd0, &args![idle, 0x5000u32]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, SPECIAL_IDLE_REPLACE_OV2), [[animation]]);
        assert!(arguments_of(&log, SPECIAL_IDLE_CHECK).is_empty());
        // Without an actor only the state changes.
        let (mut e, idle, _, _) = loaded_fixture(0);
        e.set(idle, AnimIdle::pActor, Ptr::NULL);
        start_log(&mut e);
        e.call(0x0049_6cd0, &args![idle, 0x5000u32]);
        let log = end_log(&mut e);
        assert_eq!(e.get(idle, AnimIdle::eFlags), 1);
        assert!(arguments_of(&log, SPECIAL_IDLE_REPLACE_OV2).is_empty());
    }

    #[test]
    fn idle_start_moves_a_loaded_idle_to_playing() {
        let (mut e, idle, actor, _) = loaded_fixture(3);
        // Only a loaded idle (state 1) starts.
        assert_eq!(e.call(0x0049_6fe0, &args![idle, 0x4242u32]).u8(), 0);
        e.set(idle, AnimIdle::eFlags, 1u32);
        start_log(&mut e);
        assert_eq!(e.call(0x0049_6fe0, &args![idle, 0x4242u32]).u8(), 1);
        let log = end_log(&mut e);
        assert_eq!(e.get(idle, AnimIdle::eFlags), 2);
        assert_eq!(e.mem.u32(idle.addr() + 0x18), 0x4242);
        assert_eq!(
            arguments_of(&log, ACTOR_SET_ANIM_ACTION),
            [[actor, 0xd, 0x4242]]
        );
        // Another play type does not touch the actor.
        let (mut e, idle, _, _) = loaded_fixture(2);
        e.set(idle, AnimIdle::eFlags, 1u32);
        start_log(&mut e);
        e.call(0x0049_6fe0, &args![idle, 0x4242u32]);
        assert!(arguments_of(&end_log(&mut e), ACTOR_SET_ANIM_ACTION).is_empty());
    }

    #[test]
    fn idle_sequence_setter_stores_the_pointer_at_0x18() {
        let mut e = engine();
        let idle = e.new_object::<AnimIdle>();
        e.call(0x0049_70e0, &args![idle, 0x7777u32]);
        assert_eq!(e.mem.u32(idle.addr() + 0x18), 0x7777);
    }

    #[test]
    fn idle_finish_waits_for_the_sequence_to_stop() {
        let (mut e, idle, actor, animation) = loaded_fixture(3);
        e.register(SEQUENCE_STATE, |e, a| ret(e.mem.u32(a[0] + 0x44)));
        e.register(ACTOR_GET_ANIM_ACTION, |_, _| ret(0xd));
        let sequence = e.mem.alloc(0x80);
        e.mem.set_u32(idle.addr() + 0x18, sequence);
        // Not playing: nothing happens.
        e.call(0x0049_7040, &args![idle, animation]);
        assert_eq!(e.get(idle, AnimIdle::eFlags), 0);
        e.set(idle, AnimIdle::eFlags, 2u32);
        // The sequence still has a state: it keeps playing.
        e.mem.set_u32(sequence + 0x44, 2);
        e.call(0x0049_7040, &args![idle, animation]);
        assert_eq!(e.get(idle, AnimIdle::eFlags), 2);
        // Stopped: done, the actor's anim action 0xd is cleared and a type 3
        // idle frees the animation's special idle.
        e.mem.set_u32(sequence + 0x44, 0);
        start_log(&mut e);
        e.call(0x0049_7040, &args![idle, animation]);
        let log = end_log(&mut e);
        assert_eq!(e.get(idle, AnimIdle::eFlags), 3);
        assert_eq!(
            arguments_of(&log, ACTOR_SET_ANIM_ACTION),
            [[actor, 0xffff_ffff, 0]]
        );
        assert_eq!(arguments_of(&log, SPECIAL_IDLE_FREE), [[animation, 1, 0]]);
        // Play type 1 and no sequence at all: done, nothing freed.
        let (mut e, idle, _, animation) = loaded_fixture(1);
        e.register(ACTOR_GET_ANIM_ACTION, |_, _| ret(3));
        e.set(idle, AnimIdle::eFlags, 2u32);
        start_log(&mut e);
        e.call(0x0049_7040, &args![idle, animation]);
        let log = end_log(&mut e);
        assert_eq!(e.get(idle, AnimIdle::eFlags), 3);
        assert!(arguments_of(&log, ACTOR_SET_ANIM_ACTION).is_empty());
        assert!(arguments_of(&log, SPECIAL_IDLE_FREE).is_empty());
    }

    // ---- The save size, the save, the load

    #[test]
    fn idle_save_size_adds_the_sequence() {
        let mut e = engine();
        e.register(SEQUENCE_SAVE_SIZE, |_, _| ret(0x20));
        let idle = e.new_object::<AnimIdle>();
        assert_eq!(e.call(0x0049_7100, &args![idle]).u16(), 13);
        e.mem.set_u32(idle.addr() + 0x18, 0x4242);
        assert_eq!(e.call(0x0049_7100, &args![idle]).u16(), 13 + 0x20 + 1);
    }

    /// The writes a double of the save object recorded: (size, bytes).
    type SaveWrites = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u8>)>>>;

    /// The save object doubles: writes go to the returned log as (size, bytes).
    fn register_save_object(e: &mut Engine, blocks: bool) -> SaveWrites {
        let log = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let sink = log.clone();
        e.register_double(SAVE_WRITE, move |e, a| {
            let bytes = (0..a[2]).map(|i| e.mem.u8(a[1] + i)).collect();
            sink.borrow_mut().push((a[2], bytes));
            Ret::default()
        });
        e.register_double(SAVE_USES_BLOCKS, move |_, _| ret(blocks as u32));
        e.mem.set_u32(SAVE_GAME_OBJECT, 0x3000_0000);
        log
    }

    /// A map entry whose slot 0x10 answers the sequence `sequence` and whose
    /// slot 0x18 answers the index `index`.
    fn idle_map_fixture(e: &mut Engine, sequence: u32, index: u32) -> Ptr<Animation> {
        e.register_double(0x00f0_0010, move |_, _| ret(sequence));
        e.register_double(0x00f0_0018, move |_, _| ret(index));
        e.register(0x00f0_000c, |_, _| ret(1));
        let entry = object_with_vtable(
            e,
            0x10,
            &[(0xc, 0x00f0_000c), (0x10, 0x00f0_0010), (0x18, 0x00f0_0018)],
        );
        register_single_entry_map(e);
        e.register(KF_MODEL_ANIM_GROUP, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ANIM_GROUP_SEQUENCE_TYPE, |_, _| ret(5));
        let map = e.mem.alloc(8);
        e.mem.set_u32(map + 4, entry);
        let animation: Ptr<Animation> = e.new_object();
        e.set(animation, Animation::pAnimSequenceMap, Ptr::new(map));
        animation
    }

    /// The map entry [`idle_map_fixture`] made.
    fn idle_map_entry_of(e: &Engine, animation: Ptr<Animation>) -> u32 {
        let map = e.get(animation, Animation::pAnimSequenceMap).addr();
        e.mem.u32(map + 4)
    }

    #[test]
    fn idle_save_writes_the_state_and_the_sequence_selector() {
        let mut e = engine();
        let writes = register_save_object(&mut e, false);
        e.register(SEQUENCE_SAVE, |_, _| Ret::default());
        let sequence = e.mem.alloc(0x80);
        let animation = idle_map_fixture(&mut e, sequence, 7);
        let idle = e.new_object::<AnimIdle>();
        e.set(idle, AnimIdle::eFlags, 2u32);
        e.set(idle, AnimIdle::eType, 1u32);
        e.set(idle, AnimIdle::eSection, 5u32);
        e.mem.set_u32(idle.addr() + 0x18, sequence);
        let kf = e.mem.alloc(0x20);
        e.mem.set_u32(kf + 8, 0x1111);
        e.mem.set_u32(idle.addr() + 0x10, kf);
        start_log(&mut e);
        e.call(0x0049_7180, &args![idle, 0.5f32, animation]);
        let log = end_log(&mut e);
        assert_eq!(
            *writes.borrow(),
            [
                (4, vec![2, 0, 0, 0]),
                (4, vec![1, 0, 0, 0]),
                (4, vec![5, 0, 0, 0]),
                (1, vec![1]),
                (1, vec![7]),
            ]
        );
        assert_eq!(
            arguments_of(&log, SEQUENCE_SAVE),
            [[sequence, 0.5f32.to_bits()]]
        );
        // The entry is asked for the index of the idle's sequence.
        assert_eq!(
            arguments_of(&log, 0x00f0_0018),
            [[idle_map_entry_of(&e, animation), sequence]]
        );
    }

    #[test]
    fn idle_save_without_a_sequence_writes_only_the_flag() {
        let mut e = engine();
        let writes = register_save_object(&mut e, false);
        let idle = e.new_object::<AnimIdle>();
        let animation: Ptr<Animation> = e.new_object();
        e.call(0x0049_7180, &args![idle, 0.5f32, animation]);
        assert_eq!(writes.borrow().len(), 4);
        assert_eq!(writes.borrow()[3], (1, vec![0]));
    }

    #[test]
    fn idle_save_uses_ff_when_the_animation_has_no_entry() {
        let mut e = engine();
        let writes = register_save_object(&mut e, false);
        e.register(SEQUENCE_SAVE, |_, _| Ret::default());
        e.register(MAP_GET_AT, |_, _| ret(0));
        e.register(KF_MODEL_ANIM_GROUP, |_, _| ret(0));
        e.register(ANIM_GROUP_SEQUENCE_TYPE, |_, _| ret(5));
        let idle = e.new_object::<AnimIdle>();
        e.mem.set_u32(idle.addr() + 0x18, 0x4242);
        let animation: Ptr<Animation> = e.new_object();
        e.call(0x0049_7180, &args![idle, 0.5f32, animation]);
        assert_eq!(writes.borrow()[4], (1, vec![0xff]));
    }

    /// Reads: the save object hands out `bytes` in order.
    fn register_load_object(e: &mut Engine, bytes: Vec<u8>) {
        let mut position = 0;
        e.register_double(SAVE_READ, move |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, bytes[position]);
                position += 1;
            }
            Ret::default()
        });
        e.mem.set_u32(SAVE_GAME_OBJECT, 0x3000_0000);
    }

    #[test]
    fn idle_load_without_a_sequence_starts_loaded() {
        let mut e = engine();
        // state 0, type 1, section 5, no sequence.
        register_load_object(&mut e, vec![0, 0, 0, 0, 1, 0, 0, 0, 5, 0, 0, 0, 0]);
        let idle = e.new_object::<AnimIdle>();
        let animation: Ptr<Animation> = e.new_object();
        e.call(0x0049_72a0, &args![idle, 0.5f32, animation]);
        assert_eq!(e.get(idle, AnimIdle::eFlags), 1);
        assert_eq!(e.get(idle, AnimIdle::eType), 1);
        assert_eq!(e.get(idle, AnimIdle::eSection), 5);
    }

    /// Everything the load of an idle with a sequence needs; the animation
    /// already holds the idle's group, so `AddAnimation` has nothing to do.
    fn idle_load_fixture(state: u8) -> (Engine, Ptr<AnimIdle>, Ptr<Animation>, u32) {
        let mut e = engine();
        let group = e.mem.alloc(0x40);
        let held = e.mem.alloc(0x80);
        e.mem.set_u32(held + 0x74, group);
        let animation = idle_map_fixture(&mut e, held, 7);
        install_zero_sequence_type(&mut e);
        e.register(ANIM_GROUP_ID, |_, _| ret(7));
        e.register(SPECIAL_IDLE_WORKING, |_, _| ret(1));
        e.register(SEQUENCE_LOAD, |_, _| Ret::default());
        e.register(SEQUENCE_SAVE_BASE_SIZE, |_, _| ret(0x14));
        e.register(SAVE_SKIP, |_, _| Ret::default());
        e.map(0x0119_7000, 0x1000);
        let idle = e.new_object::<AnimIdle>();
        let kf = e.mem.alloc(0x20);
        e.mem.set_u32(kf + 8, group);
        e.mem.set_u32(idle.addr() + 0x10, kf);
        // state, type 1, section 5, flag 1, selector 3.
        register_load_object(&mut e, vec![state, 0, 0, 0, 1, 0, 0, 0, 5, 0, 0, 0, 1, 3]);
        (e, idle, animation, held)
    }

    /// The sequence type table is mapped (all zero: no type keeps several
    /// sequences) for `AddAnimation`.
    fn install_zero_sequence_type(e: &mut Engine) {
        e.map(0x0119_7000, 0x1000);
    }

    #[test]
    fn idle_load_in_state_2_starts_the_group_again() {
        let (mut e, idle, animation, held) = idle_load_fixture(2);
        start_log(&mut e);
        e.call(0x0049_72a0, &args![idle, 0.5f32, animation]);
        let log = end_log(&mut e);
        assert_eq!(e.get(animation, Animation::cSkipNextBlend), 1);
        // The entry is asked for its sequence with the saved selector 3 and the
        // sequence is started in the idle's section; `StartGroup_ov2` (stood in
        // for by a refusal) gives nothing back.
        assert_eq!(
            arguments_of(&log, 0x00f0_0010).last(),
            Some(&vec![idle_map_entry_of(&e, animation), 3])
        );
        assert_eq!(
            arguments_of(&log, SPECIAL_IDLE_WORKING),
            [[animation.addr(), held]]
        );
        assert_eq!(e.mem.u32(idle.addr() + 0x18), 0);
        // The sequence's own data is not read without a sequence; the bytes it
        // would need are skipped all the same.
        assert!(arguments_of(&log, SEQUENCE_LOAD).is_empty());
        assert_eq!(arguments_of(&log, SAVE_SKIP), [[0x3000_0000, 0x14]]);
        assert_eq!(e.get(idle, AnimIdle::eType), 1);
        assert_eq!(e.get(idle, AnimIdle::eSection), 5);
    }

    #[test]
    fn idle_load_in_state_3_keeps_the_sequence_without_starting_it() {
        let (mut e, idle, animation, held) = idle_load_fixture(3);
        start_log(&mut e);
        e.call(0x0049_72a0, &args![idle, 0.5f32, animation]);
        let log = end_log(&mut e);
        assert_eq!(e.mem.u32(idle.addr() + 0x18), held);
        assert!(arguments_of(&log, SPECIAL_IDLE_WORKING).is_empty());
        assert_eq!(
            arguments_of(&log, SEQUENCE_LOAD),
            [[held, 0.5f32.to_bits()]]
        );
        // State 0 becomes 1 and nothing else changes.
        let (mut e, idle, animation, _) = idle_load_fixture(0);
        e.call(0x0049_72a0, &args![idle, 0.5f32, animation]);
        assert_eq!(e.get(idle, AnimIdle::eFlags), 1);
        assert_eq!(e.mem.u32(idle.addr() + 0x18), 0);
    }

    #[test]
    fn idle_save_size_includes_the_block_header_and_logs_on_request() {
        let mut e = engine();
        e.mem.set_u32(SAVE_GAME_OBJECT, 0x3000_0000);
        e.register(SAVE_USES_BLOCKS, |_, _| ret(1));
        e.register(IDLE_FORM_OF, |_, a| ret(a[0] + 0x100));
        e.register(SEQUENCE_SAVE_SIZE, |_, _| ret(0x20));
        e.register(SETTING_BYTE_ADDRESS, |_, _| ret(0x3000_0100));
        e.map(0x3000_0000, 0x1000);
        e.register(ERROR_LOG, |_, _| Ret::default());
        e.register(SAVE_RECORD_WRITTEN, |_, _| ret(0));
        let idle = e.new_object::<AnimIdle>();
        // 4 (state) + 6 (block header) + 2 + the idle's own 13 bytes.
        assert_eq!(
            e.call(0x0049_74c0, &args![0u32, idle]).u16(),
            4 + 6 + 2 + 13
        );
        // Without an idle form only the 4 + 6 stay.
        e.register(IDLE_FORM_OF, |_, _| ret(0));
        assert_eq!(e.call(0x0049_74c0, &args![0u32, idle]).u16(), 10);
        // Without blocks and without an idle: 4.
        e.register(SAVE_USES_BLOCKS, |_, _| ret(0));
        assert_eq!(e.call(0x0049_74c0, &args![0u32, 0u32]).u16(), 4);
        // With the diagnostics on and no record, the size is logged.
        e.mem.set_u8(0x3000_0100, 1);
        start_log(&mut e);
        e.call(0x0049_74c0, &args![0u32, 0u32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, ERROR_LOG),
            [[LOG_SAVE_SIZE, 4, 0x11f3, SOURCE_FILE]]
        );
    }

    #[test]
    fn idle_save_size_logs_the_record_being_written() {
        let mut e = engine();
        e.mem.set_u32(SAVE_GAME_OBJECT, 0x3000_0000);
        e.map(0x3000_0000, 0x1000);
        e.register(SAVE_USES_BLOCKS, |_, _| ret(0));
        e.register(SETTING_BYTE_ADDRESS, |_, _| ret(0x3000_0100));
        e.mem.set_u8(0x3000_0100, 1);
        // A record: form ID at +0, flags at +5; its form type answers slot
        // 0x130 with a name.
        let record = e.mem.alloc(0x20);
        e.mem.set_u32(record, 0x1234);
        e.mem.set_u32(record + 5, 0x77);
        e.register_double(SAVE_RECORD_WRITTEN, move |_, _| ret(record));
        e.register(0x00f0_0130, |_, _| ret(0x9999));
        let kind = object_with_vtable(&mut e, 0x10, &[(SLOT_FORM_TYPE_NAME, 0x00f0_0130)]);
        e.register_double(FORM_BY_ID, move |_, _| ret(kind));
        e.register(ERROR_LOG, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0049_74c0, &args![0u32, 0u32]);
        assert_eq!(
            arguments_of(&end_log(&mut e), ERROR_LOG),
            [[
                LOG_SAVE_SIZE_FORM,
                4,
                0x1234,
                0x9999,
                0x77,
                0x11f3,
                SOURCE_FILE
            ]]
        );
    }

    /// The save object for the whole-idle save and load tests, at `save`.
    fn save_object_fixture(e: &mut Engine, blocks: bool) {
        e.map(0x3000_0000, 0x1000);
        e.mem.set_u32(SAVE_GAME_OBJECT, 0x3000_0000);
        e.register_double(SAVE_USES_BLOCKS, move |_, _| ret(blocks as u32));
        e.register(SETTING_BYTE_ADDRESS, |_, _| ret(0x3000_0100));
        e.register(SAVE_RECORD_WRITTEN, |_, _| ret(0));
        e.register(SAVE_RECORD_READ, |_, _| ret(0));
        e.register(ERROR_LOG, |_, _| Ret::default());
        e.register(LOG, |_, _| Ret::default());
    }

    #[test]
    fn saving_an_idle_writes_a_block_with_its_length() {
        let mut e = engine();
        save_object_fixture(&mut e, true);
        let writes = register_save_object(&mut e, true);
        // The buffer position starts at 0x3000_0200 and moves with the bytes
        // written.
        let position = std::rc::Rc::new(std::cell::Cell::new(0x3000_0200u32));
        let moved = position.clone();
        e.register_double(SAVE_POSITION, move |_, _| ret(moved.get()));
        let sink = writes.clone();
        let advance = position.clone();
        e.register_double(SAVE_WRITE, move |e, a| {
            let bytes = (0..a[2]).map(|i| e.mem.u8(a[1] + i)).collect();
            sink.borrow_mut().push((a[2], bytes));
            advance.set(advance.get() + a[2]);
            Ret::default()
        });
        e.register(IDLE_FORM_OF, |_, a| ret(a[0] + 0x100));
        e.register(WORD_AT_C, |_, _| ret(0x0000_1234));
        e.register(SAVE_FORM_ID, |_, _| Ret::default());
        e.register(SEQUENCE_SAVE_SIZE, |_, _| ret(0));
        let idle = e.new_object::<AnimIdle>();
        let animation: Ptr<Animation> = e.new_object();
        start_log(&mut e);
        e.call(0x0049_75e0, &args![0u32, idle, animation, 0.5f32]);
        let log = end_log(&mut e);
        let writes = writes.borrow();
        // The tag and the placeholder length come first.
        assert_eq!(writes[0], (4, SAVE_BLOCK_TAG.to_le_bytes().to_vec()));
        assert_eq!(writes[1], (2, vec![0, 0]));
        assert_eq!(arguments_of(&log, SAVE_FORM_ID).len(), 1);
        // The ID of the idle form was put in the cell handed to the save.
        let cell = arguments_of(&log, SAVE_FORM_ID)[0][1];
        assert_eq!(e.mem.u32(cell), 0x1234);
        // The size of the idle (13 bytes), then its state: 3 words, the flag.
        assert_eq!(writes[2], (2, vec![13, 0]));
        let block_length = e.mem.u16(0x3000_0200 + 4);
        let total = position.get() - (0x3000_0200 + 4);
        assert_eq!(block_length as u32, total);
    }

    #[test]
    fn saving_an_idle_without_a_form_writes_a_zero_id() {
        let mut e = engine();
        save_object_fixture(&mut e, false);
        let writes = register_save_object(&mut e, false);
        e.register(SAVE_POSITION, |_, _| ret(0x3000_0200));
        e.register(IDLE_FORM_OF, |_, _| ret(0));
        e.register(SAVE_FORM_ID, |_, _| Ret::default());
        let idle = e.new_object::<AnimIdle>();
        let animation: Ptr<Animation> = e.new_object();
        start_log(&mut e);
        e.call(0x0049_75e0, &args![0u32, idle, animation, 0.5f32]);
        let log = end_log(&mut e);
        assert!(writes.borrow().is_empty());
        let cell = arguments_of(&log, SAVE_FORM_ID)[0][1];
        assert_eq!(e.mem.u32(cell), 0);
    }

    #[test]
    fn saving_an_idle_logs_a_block_longer_than_16_bits() {
        let mut e = engine();
        save_object_fixture(&mut e, true);
        let writes = register_save_object(&mut e, true);
        let _ = writes;
        let calls = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let counter = calls.clone();
        // The position jumps by 0x20000 after the block header.
        e.register_double(SAVE_POSITION, move |_, _| {
            counter.set(counter.get() + 1);
            ret(0x3000_0200 + 0x20000 * (counter.get() > 2) as u32)
        });
        e.register(IDLE_FORM_OF, |_, _| ret(0));
        e.register(SAVE_FORM_ID, |_, _| Ret::default());
        let idle = e.new_object::<AnimIdle>();
        let animation: Ptr<Animation> = e.new_object();
        e.map(0x3002_0000, 0x1000);
        start_log(&mut e);
        e.call(0x0049_75e0, &args![0u32, idle, animation, 0.5f32]);
        assert_eq!(
            arguments_of(&end_log(&mut e), LOG),
            [[LOG_SAVE_BLOCK_TOO_BIG, SOURCE_FILE, 0x120e]]
        );
    }

    /// The load of a whole idle through the save game object: the bytes read
    /// come from `bytes`, the buffer position from `positions` (the first
    /// value, then the last one forever).
    fn idle_stream_fixture(
        b: &mut IdleBuild,
        blocks: bool,
        form_id: u32,
        bytes: Vec<u8>,
        positions: Vec<u32>,
    ) {
        let e = &mut b.e;
        save_object_fixture(e, blocks);
        register_load_object(e, bytes);
        let mut next = 0;
        e.register_double(SAVE_POSITION, move |_, _| {
            let value = positions[next.min(positions.len() - 1)];
            next += 1;
            ret(value)
        });
        e.register(SAVE_SKIP, |_, _| Ret::default());
        e.register_double(LOAD_FORM_ID, move |e, a| {
            e.mem.set_u32(a[1], form_id);
            ret(0)
        });
        let idle_form = b.idle_form;
        e.register_double(FORM_BY_ID, move |_, _| ret(idle_form));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(SEQUENCE_SAVE_BASE_SIZE, |_, _| ret(0));
        e.register(SPECIAL_IDLE_WORKING, |_, _| ret(1));
    }

    #[test]
    fn loading_an_idle_builds_it_and_reads_its_state() {
        let mut b = idle_build(0, false);
        // Bytes: the count that follows the ID (2 bytes), then the idle's state,
        // type and section (4 bytes each) and the flag byte (no sequence).
        let mut bytes = vec![2, 0];
        bytes.extend_from_slice(&[0, 0, 0, 0, 1, 0, 0, 0, 5, 0, 0, 0, 0]);
        idle_stream_fixture(&mut b, false, 0x77, bytes, vec![0]);
        b.e.register(IDLE_FORM_OF, |_, _| ret(0));
        let (actor, animation) = (b.actor, b.animation);
        start_log(&mut b.e);
        let idle =
            b.e.call(0x0049_77e0, &args![actor, animation, 0.5f32])
                .ptr::<AnimIdle>();
        let log = end_log(&mut b.e);
        assert!(!idle.is_null());
        // Built for section 7, play type 1, with the KF flag, for the actor
        // and no animation; the load then overwrote state, type and section.
        assert_eq!(b.e.get(idle, AnimIdle::pActor).addr(), actor);
        assert_eq!(b.e.get(idle, AnimIdle::pIdleForm).addr(), b.idle_form);
        assert!(b.e.get(idle, AnimIdle::pAnimation).is_null());
        assert_eq!(b.e.get(idle, AnimIdle::eType), 1);
        assert_eq!(b.e.get(idle, AnimIdle::eSection), 5);
        assert_eq!(b.e.get(idle, AnimIdle::eFlags), 1);
        assert_eq!(arguments_of(&log, MODEL_LOADER_LOAD_KF).len(), 1);
        assert_eq!(
            arguments_of(&log, RT_DYNAMIC_CAST),
            [[
                b.idle_form,
                0,
                TYPE_DESCRIPTOR_FORM,
                TYPE_DESCRIPTOR_IDLE_FORM,
                0
            ]]
        );
        assert_eq!(arguments_of(&log, FORM_BY_ID), [[0x77]]);
        // The scope guard of source line 0x1218 wraps it.
        assert_eq!(
            arguments_of(&log, SCOPE_GUARD_CTOR)[0][1..],
            [SCOPE_GUARD_TAG, 1, SOURCE_FILE, 0x1218]
        );
        assert_eq!(log.last().unwrap().0, SCOPE_GUARD_DTOR);
    }

    #[test]
    fn loading_an_unknown_idle_logs_and_skips_its_bytes() {
        let mut b = idle_build(0, false);
        idle_stream_fixture(&mut b, false, 0x77, vec![4, 0], vec![0]);
        b.e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        b.e.register(LOG, |_, _| Ret::default());
        b.e.register(0x00f0_0130, |_, _| ret(0x9999));
        let actor = object_with_vtable(&mut b.e, 0x400, &[(SLOT_FORM_TYPE_NAME, 0x00f0_0130)]);
        let animation = b.animation;
        start_log(&mut b.e);
        let idle =
            b.e.call(0x0049_77e0, &args![actor, animation, 0.5f32])
                .u32();
        let log = end_log(&mut b.e);
        assert_eq!(idle, 0);
        assert_eq!(
            arguments_of(&log, LOG),
            [[LOG_LOAD_UNKNOWN_IDLE, 0x77, 0x9999]]
        );
        // The two-byte count read after the ID (4) is skipped.
        assert_eq!(arguments_of(&log, SAVE_SKIP), [[0x3000_0000, 4]]);
        assert!(arguments_of(&log, MODEL_LOADER_LOAD_KF).is_empty());
    }

    #[test]
    fn loading_nothing_reads_neither_the_count_nor_an_idle() {
        let mut b = idle_build(0, false);
        idle_stream_fixture(&mut b, false, 0, vec![], vec![0]);
        let (actor, animation) = (b.actor, b.animation);
        start_log(&mut b.e);
        assert_eq!(
            b.e.call(0x0049_77e0, &args![actor, animation, 0.5f32])
                .u32(),
            0
        );
        let log = end_log(&mut b.e);
        assert!(arguments_of(&log, SAVE_READ).is_empty());
        assert!(arguments_of(&log, FORM_BY_ID).is_empty());
    }

    #[test]
    fn loading_an_idle_checks_the_block_tag_and_length() {
        // A wrong tag: logged with the record being read when there is one.
        let mut b = idle_build(0, false);
        let mut bytes = vec![0x58, 0x58, 0x58, 0x58];
        bytes.extend_from_slice(&[10, 0]);
        idle_stream_fixture(&mut b, true, 0, bytes.clone(), vec![0x1000, 0x1000 + 10]);
        b.e.register(LOG, |_, _| Ret::default());
        let record = b.e.mem.alloc(0x20);
        b.e.mem.set_u32(record, 0x1234);
        b.e.mem.set_u32(record + 5, 0x77);
        b.e.mem.set_u8(record + 9, 3);
        b.e.register_double(SAVE_RECORD_READ, move |_, _| ret(record));
        b.e.register(0x00f0_0130, |_, _| ret(0x9999));
        let kind = object_with_vtable(&mut b.e, 0x10, &[(SLOT_FORM_TYPE_NAME, 0x00f0_0130)]);
        b.e.register_double(FORM_BY_ID, move |_, _| ret(kind));
        let (actor, animation) = (b.actor, b.animation);
        start_log(&mut b.e);
        b.e.call(0x0049_77e0, &args![actor, animation, 0.5f32]);
        let log = end_log(&mut b.e);
        assert_eq!(
            arguments_of(&log, LOG),
            [[
                LOG_LOAD_NO_BLOCK_FORM,
                SOURCE_FILE,
                0x121c,
                0x1234,
                0x9999,
                3,
                0x77
            ]]
        );
        // Without a record the version byte is logged instead.
        let mut b = idle_build(0, false);
        idle_stream_fixture(&mut b, true, 0, bytes, vec![0x1000, 0x1000 + 10]);
        b.e.register(LOG, |_, _| Ret::default());
        b.e.register(SAVE_BYTE_80, |_, _| ret(0x2a));
        let (actor, animation) = (b.actor, b.animation);
        start_log(&mut b.e);
        b.e.call(0x0049_77e0, &args![actor, animation, 0.5f32]);
        assert_eq!(
            arguments_of(&end_log(&mut b.e), LOG),
            [[LOG_LOAD_NO_BLOCK, SOURCE_FILE, 0x121c, 0x2a]]
        );
    }

    #[test]
    fn loading_an_idle_logs_a_block_read_too_long_or_too_short() {
        let tag = SAVE_BLOCK_TAG.to_le_bytes();
        let mut bytes = tag.to_vec();
        bytes.extend_from_slice(&[10, 0]);
        let run = |end: u32, record: bool| {
            let mut b = idle_build(0, false);
            // The block starts at 0x1000 with length 10; reading ends at `end`.
            idle_stream_fixture(&mut b, true, 0, bytes.clone(), vec![0x1000, end]);
            b.e.register(LOG, |_, _| Ret::default());
            b.e.register(SAVE_BYTE_80, |_, _| ret(0x2a));
            if record {
                let record = b.e.mem.alloc(0x20);
                b.e.mem.set_u32(record, 0x1234);
                b.e.mem.set_u32(record + 5, 0x77);
                b.e.mem.set_u8(record + 9, 3);
                b.e.register_double(SAVE_RECORD_READ, move |_, _| ret(record));
                b.e.register(0x00f0_0130, |_, _| ret(0x9999));
                let kind =
                    object_with_vtable(&mut b.e, 0x10, &[(SLOT_FORM_TYPE_NAME, 0x00f0_0130)]);
                b.e.register_double(FORM_BY_ID, move |_, _| ret(kind));
            }
            let (actor, animation) = (b.actor, b.animation);
            start_log(&mut b.e);
            b.e.call(0x0049_77e0, &args![actor, animation, 0.5f32]);
            arguments_of(&end_log(&mut b.e), LOG)
        };
        // Exactly the block: no message.
        assert!(run(0x100a, false).is_empty());
        assert_eq!(
            run(0x1010, false),
            [[LOG_LOAD_LONG, 6, SOURCE_FILE, 0x1237, 0x2a]]
        );
        assert_eq!(
            run(0x1004, false),
            [[LOG_LOAD_SHORT, 6, SOURCE_FILE, 0x1237, 0x2a]]
        );
        assert_eq!(
            run(0x1010, true),
            [[
                LOG_LOAD_LONG_FORM,
                6,
                SOURCE_FILE,
                0x1237,
                0x1234,
                0x9999,
                3,
                0x77
            ]]
        );
        assert_eq!(
            run(0x1004, true),
            [[
                LOG_LOAD_SHORT_FORM,
                6,
                SOURCE_FILE,
                0x1237,
                0x1234,
                0x9999,
                3,
                0x77
            ]]
        );
    }

    // ---- The save buffer variants

    #[test]
    fn save_buffer_variants_move_the_idle_state_through_the_buffer() {
        let mut e = engine();
        e.register(SAVE_BUFFER_FIELD, |_, _| Ret::default());
        let idle = e.new_object::<AnimIdle>();
        let a = idle.addr();
        start_log(&mut e);
        e.call(0x0049_7bc0, &args![idle, 0x9000u32, 0x1111u32]);
        assert_eq!(
            arguments_of(&end_log(&mut e), SAVE_BUFFER_FIELD),
            [
                [0x9000, a + 8, 4, 0],
                [0x9000, a + 0xc, 4, 0],
                [0x9000, a + 0x14, 4, 0]
            ]
        );
    }

    #[test]
    fn load_buffer_variant_restores_the_state() {
        let mut e = engine();
        let idle = e.new_object::<AnimIdle>();
        let base = idle.addr();
        e.register_double(LOAD_BUFFER_FIELD, move |e, a| {
            // The state, type and section read are 0, 3 and 9.
            let value = match a[1] - base {
                0x08 => 0,
                0x0c => 3,
                _ => 9,
            };
            e.mem.set_u32(a[1], value);
            Ret::default()
        });
        // State 0 becomes 1; without state 2 nothing is added.
        e.call(0x0049_7c10, &args![idle, 0x9000u32, 0x1111u32]);
        assert_eq!(e.get(idle, AnimIdle::eFlags), 1);
        assert_eq!(e.get(idle, AnimIdle::eType), 3);
        assert_eq!(e.get(idle, AnimIdle::eSection), 9);
    }

    #[test]
    fn load_buffer_variant_adds_the_animation_in_state_2() {
        let mut b = idle_build(0, false);
        let e = &mut b.e;
        let base = b.idle.addr();
        e.register_double(LOAD_BUFFER_FIELD, move |e, a| {
            if a[1] - base == 0x08 {
                e.mem.set_u32(a[1], 2);
            }
            Ret::default()
        });
        e.register(KF_MODEL_ANIM_GROUP, |_, _| ret(0));
        let animation: Ptr<Animation> = e.new_object();
        let kf = e.mem.alloc(0x20);
        e.mem.set_u32(b.idle.addr() + 0x10, kf);
        start_log(e);
        e.call(0x0049_7c10, &args![b.idle, 0x9000u32, animation]);
        let log = end_log(e);
        // `AddAnimation` asks for the model's group (there is none).
        assert_eq!(arguments_of(&log, KF_MODEL_ANIM_GROUP), [[kf]]);
        // Without a model nothing is added.
        e.mem.set_u32(b.idle.addr() + 0x10, 0);
        start_log(e);
        e.call(0x0049_7c10, &args![b.idle, 0x9000u32, animation]);
        assert!(arguments_of(&end_log(e), KF_MODEL_ANIM_GROUP).is_empty());
    }

    // ---- SpecialIdleQueue and the queued idle

    /// An animation (for `actor`) in an engine that can construct idles and
    /// run the queue's other calls.
    fn queue_fixture() -> (IdleBuild, Ptr<Animation>) {
        let mut b = idle_build(0x5000, false);
        let this: Ptr<Animation> = b.e.new_object();
        b.e.set(this, Animation::pActorRef, Ptr::new(b.actor));
        b.e.register(WORD_AT_14, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        b.e.register(SEQUENCE_STATE, |e, a| ret(e.mem.u32(a[0] + 0x44)));
        b.e.register(LOG, |_, _| Ret::default());
        b.e.register(BLEND_OUT, |_, _| Ret::default());
        b.e.register(ANIM_IDLE_FREE, |_, _| Ret::default());
        b.e.register(NI_POINTER_COPY, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret(a[0])
        });
        (b, this)
    }

    /// An idle that plays `sequence` in `section`, current in the queue
    /// fixture's animation.
    fn current_idle(b: &mut IdleBuild, this: Ptr<Animation>, section: u32, state: u32) -> u32 {
        let sequence = named_object(&mut b.e, "seq.kf");
        b.e.mem.set_u32(sequence + 0x44, state);
        let idle = b.e.mem.alloc(0x38);
        b.e.mem.set_u32(idle + 0x14, section);
        b.e.mem.set_u32(idle + 0x18, sequence);
        b.e.mem.set_u32(this.addr() + 0x124, idle);
        let slot = match section {
            0x14 => 1,
            0x15 => 4,
            other => other,
        };
        b.e.mem.set_u32(this.addr() + 0xe0 + 4 * slot, sequence);
        idle
    }

    #[test]
    fn special_idle_queue_builds_the_idle_and_stores_it() {
        let (mut b, this) = queue_fixture();
        let idle_form = b.idle_form;
        start_log(&mut b.e);
        b.e.call(0x0049_7ca0, &args![this, idle_form, 5u32]);
        let log = end_log(&mut b.e);
        let idle = b.e.mem.u32(this.addr() + 0x124);
        assert_ne!(idle, 0);
        let idle = Ptr::<AnimIdle>::new(idle);
        assert_eq!(b.e.get(idle, AnimIdle::eSection), 5);
        assert_eq!(b.e.get(idle, AnimIdle::eType), 1);
        assert_eq!(b.e.get(idle, AnimIdle::pActor).addr(), b.actor);
        assert!(b.e.get(idle, AnimIdle::pAnimation).is_null());
        assert_eq!(b.e.get(idle, AnimIdle::pIdleForm).addr(), idle_form);
        // Built without the KF flag: the idle KF loader runs.
        assert_eq!(arguments_of(&log, MODEL_LOADER_LOAD_IDLE_KF).len(), 1);
        // The scope guard of source line 0x125d wraps it.
        assert_eq!(
            arguments_of(&log, SCOPE_GUARD_CTOR)[0][1..],
            [SCOPE_GUARD_TAG, 1, SOURCE_FILE, 0x125d]
        );
        assert_eq!(log.last().unwrap().0, SCOPE_GUARD_DTOR);
    }

    #[test]
    fn special_idle_queue_blends_out_and_parks_the_current_idle() {
        let (mut b, this) = queue_fixture();
        let old = current_idle(&mut b, this, 0x14, 1);
        let idle_form = b.idle_form;
        start_log(&mut b.e);
        b.e.call(0x0049_7ca0, &args![this, idle_form, 5u32]);
        let log = end_log(&mut b.e);
        // Section 0x14 is slot 1, where the idle's sequence is current: the
        // section is blended out, and the old idle moves to the first free
        // parking place.
        assert_eq!(arguments_of(&log, BLEND_OUT), [[this.addr(), 0x14, 0]]);
        assert_eq!(
            arguments_of(&log, NI_POINTER_COPY),
            [[this.addr() + 0x12c, this.addr() + 0x124]]
        );
        assert_eq!(b.e.mem.u32(this.addr() + 0x12c), old);
        assert_ne!(b.e.mem.u32(this.addr() + 0x124), old);
        assert_ne!(b.e.mem.u32(this.addr() + 0x124), 0);
        assert!(arguments_of(&log, ANIM_IDLE_FREE).is_empty());
    }

    #[test]
    fn special_idle_queue_uses_the_second_parking_place_and_then_frees() {
        let (mut b, this) = queue_fixture();
        let old = current_idle(&mut b, this, 0x15, 1);
        b.e.mem.set_u32(this.addr() + 0x12c, 0x4321);
        let idle_form = b.idle_form;
        start_log(&mut b.e);
        b.e.call(0x0049_7ca0, &args![this, idle_form, 5u32]);
        let log = end_log(&mut b.e);
        // Section 0x15 is slot 4.
        assert_eq!(arguments_of(&log, BLEND_OUT).len(), 1);
        assert_eq!(b.e.mem.u32(this.addr() + 0x130), old);
        assert_eq!(b.e.mem.u32(this.addr() + 0x12c), 0x4321);

        let (mut b, this) = queue_fixture();
        let old = current_idle(&mut b, this, 2, 1);
        b.e.mem.set_u32(this.addr() + 0x12c, 0x4321);
        b.e.mem.set_u32(this.addr() + 0x130, 0x4322);
        let idle_form = b.idle_form;
        start_log(&mut b.e);
        b.e.call(0x0049_7ca0, &args![this, idle_form, 5u32]);
        let log = end_log(&mut b.e);
        assert_eq!(
            arguments_of(&log, ANIM_IDLE_FREE),
            [[this.addr(), this.addr() + 0x124]]
        );
        let _ = old;
    }

    #[test]
    fn special_idle_queue_gives_up_on_an_idle_that_just_started() {
        let (mut b, this) = queue_fixture();
        let old = current_idle(&mut b, this, 3, 2);
        let idle_form = b.idle_form;
        start_log(&mut b.e);
        b.e.call(0x0049_7ca0, &args![this, idle_form, 5u32]);
        let log = end_log(&mut b.e);
        let sequence = b.e.mem.u32(old + 0x18);
        let name = b.e.mem.u32(sequence + 8);
        assert_eq!(arguments_of(&log, LOG), [[LOG_QUEUE_JUST_STARTED, name]]);
        // Nothing changed: the old idle is still current, no new idle.
        assert_eq!(b.e.mem.u32(this.addr() + 0x124), old);
        assert!(arguments_of(&log, MODEL_LOADER_LOAD_IDLE_KF).is_empty());
        assert!(arguments_of(&log, BLEND_OUT).is_empty());
        assert_eq!(log.last().unwrap().0, SCOPE_GUARD_DTOR);
        // State 5 counts as just started too.
        let (mut b, this) = queue_fixture();
        let old = current_idle(&mut b, this, 3, 5);
        let idle_form = b.idle_form;
        b.e.call(0x0049_7ca0, &args![this, idle_form, 5u32]);
        assert_eq!(b.e.mem.u32(this.addr() + 0x124), old);
    }

    #[test]
    fn special_idle_queue_keeps_an_idle_that_plays_elsewhere() {
        let (mut b, this) = queue_fixture();
        let old = current_idle(&mut b, this, 3, 1);
        // The slot holds another sequence: no blend out, but the idle is still
        // parked.
        b.e.mem.set_u32(this.addr() + 0xe0 + 12, 0x7777);
        let idle_form = b.idle_form;
        start_log(&mut b.e);
        b.e.call(0x0049_7ca0, &args![this, idle_form, 5u32]);
        let log = end_log(&mut b.e);
        assert!(arguments_of(&log, BLEND_OUT).is_empty());
        assert_eq!(b.e.mem.u32(this.addr() + 0x12c), old);
    }

    #[test]
    fn queued_idle_is_built_for_the_actor_and_announced_to_its_process() {
        let mut b = idle_build(0x5000, false);
        let this: Ptr<Animation> = b.e.new_object();
        b.e.register(SPECIAL_IDLE_CHECK, |_, _| ret(1));
        b.e.register(ACTOR_PROCESS, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        b.e.register(0x00f0_0390, |_, _| Ret::default());
        let process = object_with_vtable(&mut b.e, 0x40, &[(SLOT_PROCESS_SET_IDLE, 0x00f0_0390)]);
        b.e.mem.set_u32(b.actor + 0x10, process);
        let (idle_form, actor) = (b.idle_form, b.actor);
        start_log(&mut b.e);
        b.e.call(0x0049_7f20, &args![this, idle_form, actor, 4i32, 2u32]);
        let log = end_log(&mut b.e);
        let idle = Ptr::<AnimIdle>::new(b.e.mem.u32(this.addr() + 0x128));
        assert_ne!(idle.addr(), 0);
        assert_eq!(b.e.get(idle, AnimIdle::eSection), 4);
        assert_eq!(b.e.get(idle, AnimIdle::eType), 2);
        assert_eq!(b.e.get(idle, AnimIdle::pAnimation).addr(), this.addr());
        assert_eq!(arguments_of(&log, 0x00f0_0390), [[process, idle_form]]);
        assert_eq!(arguments_of(&log, SPECIAL_IDLE_CHECK), [[this.addr()]]);
        assert_eq!(
            arguments_of(&log, SCOPE_GUARD_CTOR)[0][1..],
            [SCOPE_GUARD_TAG, 1, SOURCE_FILE, 0x129d]
        );
        // The sections 0, 1 and 0x14 force play type 3; others keep theirs.
        for (section, expected) in [(0, 3), (1, 3), (0x14, 3), (2, 2), (0x15, 2), (-1, 2)] {
            let mut b = idle_build(0x5000, false);
            let this: Ptr<Animation> = b.e.new_object();
            b.e.register(SPECIAL_IDLE_CHECK, |_, _| ret(1));
            b.e.register(ACTOR_PROCESS, |e, a| ret(e.mem.u32(a[0] + 0x10)));
            b.e.register(0x00f0_0390, |_, _| Ret::default());
            let process =
                object_with_vtable(&mut b.e, 0x40, &[(SLOT_PROCESS_SET_IDLE, 0x00f0_0390)]);
            b.e.mem.set_u32(b.actor + 0x10, process);
            let (idle_form, actor) = (b.idle_form, b.actor);
            b.e.call(0x0049_7f20, &args![this, idle_form, actor, section, 2u32]);
            let idle = Ptr::<AnimIdle>::new(b.e.mem.u32(this.addr() + 0x128));
            assert_eq!(
                b.e.get(idle, AnimIdle::eType),
                expected,
                "section {section}"
            );
        }
    }

    // ---- Fourth session (`00498030` onward) ---------------------------------------------

    /// `engine_two()` plus doubles for the getters the special idle functions
    /// go through: the state (+8), play type (+0xc), section (+0x14), idle form
    /// (+0x2c) and KF model (+0x10) of an idle, the state of a sequence (+0x44)
    /// and the copy of a `NiPointer`.
    fn idle_engine() -> Engine {
        let mut e = engine_two();
        e.register(WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(WORD_AT_14, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        e.register(IDLE_FORM_OF, |e, a| ret(e.mem.u32(a[0] + 0x2c)));
        e.register(IDLE_KF_MODEL, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(LOG, |_, _| Ret::default());
        e.register(NI_POINTER_COPY, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            ret(a[0])
        });
        e.register(PIPBOY_QUERY, |_, _| ret(0));
        e.register(KF_MODEL_ANIM_GROUP, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(SETTING_FLOAT_ADDRESS, |e, a| {
            // The blend time setting reads 0.5, every other float setting 2.0.
            let cell = e.global::<u32>(0x011c_5000);
            e.mem.set_f32(cell, 0.5);
            e.mem.set_f32(cell + 4, 2.0);
            ret(if a[0] == SETTING_BLEND_TIME {
                cell
            } else {
                cell + 4
            })
        });
        let cell = e.mem.alloc(8);
        e.set_global(0x011c_5000, cell);
        e
    }

    /// An idle with the given state, play type, section and sequence.
    fn fake_idle(e: &mut Engine, state: u32, kind: u32, section: u32, sequence: u32) -> u32 {
        let idle = e.mem.alloc(0x38);
        e.mem.set_u32(idle + 8, state);
        e.mem.set_u32(idle + 0xc, kind);
        e.mem.set_u32(idle + 0x14, section);
        e.mem.set_u32(idle + 0x18, sequence);
        idle
    }

    /// Stores the four idle fields: current, queued and the two parking places.
    fn put_idles(e: &mut Engine, this: Ptr<Animation>, idles: [u32; 4]) {
        for (i, idle) in idles.iter().enumerate() {
            e.mem.set_u32(this.addr() + 0x124 + 4 * i as u32, *idle);
        }
    }

    fn idle_fields(e: &Engine, this: Ptr<Animation>) -> [u32; 4] {
        let mut fields = [0; 4];
        for (i, field) in fields.iter_mut().enumerate() {
            *field = e.mem.u32(this.addr() + 0x124 + 4 * i as u32);
        }
        fields
    }

    /// A KF model object (the animation group at +8) for a group made by
    /// `group_with_id`, whose id is added to the animation's sequence map with
    /// the sequence `sequence`: `AddAnimation` then finds the sequence in the
    /// map, with that group, and has nothing to do. Returns the model.
    fn addable_model(e: &mut Engine, this: Ptr<Animation>, id: u16, sequence: u32) -> u32 {
        let group = e.mem.u32(sequence + 0x74);
        assert_eq!(e.mem.u16(group + 0x10), id);
        let map = if this.addr() != 0 && e.get(this, Animation::pAnimSequenceMap).addr() != 0 {
            e.get(this, Animation::pAnimSequenceMap).addr()
        } else {
            map_for(e, this)
        };
        let entry = map_entry(e, sequence, true);
        map_add(e, map, id as u32, entry);
        let kf = e.mem.alloc(0x20);
        e.mem.set_u32(kf + 8, group);
        kf
    }

    #[test]
    fn special_idle_replace_builds_the_queued_idle() {
        // The flag object decides how the idle is built.
        for (flag, load_kf) in [(0u32, false), (1, true)] {
            let mut b = idle_build(0, false);
            idle_getters_for_build(&mut b.e);
            b.e.set_global(FLAG_OBJECT_GLOBAL, 0x7000u32);
            b.e.register_double(FLAG_OBJECT_CHECK, move |_, _| ret(flag));
            let this: Ptr<Animation> = b.e.new_object();
            let (idle_form, actor) = (b.idle_form, b.actor);
            start_log(&mut b.e);
            b.e.call(0x0049_8030, &args![this, idle_form, actor, 5u32]);
            let log = end_log(&mut b.e);
            let idle = Ptr::<AnimIdle>::new(b.e.mem.u32(this.addr() + 0x128));
            assert_ne!(idle.addr(), 0);
            assert_eq!(b.e.get(idle, AnimIdle::eSection), 5);
            assert_eq!(b.e.get(idle, AnimIdle::eType), 0);
            assert_eq!(b.e.get(idle, AnimIdle::pActor).addr(), actor);
            let owner = b.e.get(idle, AnimIdle::pAnimation).addr();
            assert_eq!(owner, if load_kf { 0 } else { this.addr() });
            assert_eq!(
                arguments_of(&log, MODEL_LOADER_LOAD_KF).len(),
                load_kf as usize
            );
            assert_eq!(
                arguments_of(&log, MODEL_LOADER_LOAD_IDLE_KF).len(),
                !load_kf as usize
            );
            assert_eq!(
                arguments_of(&log, SCOPE_GUARD_CTOR)[0][1..],
                [SCOPE_GUARD_TAG, 1, SOURCE_FILE, 0x12bd]
            );
        }
    }

    /// The state and play type doubles `idle_build` engines lack.
    fn idle_getters_for_build(e: &mut Engine) {
        e.register(WORD_AT_8, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(WORD_AT_C, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(WORD_AT_14, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        e.register(IDLE_KF_MODEL, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(KF_MODEL_ANIM_GROUP, |e, a| ret(e.mem.u32(a[0] + 8)));
    }

    #[test]
    fn special_idle_replace_ov2_adds_the_loaded_queued_idle_and_drops_it() {
        let mut e = idle_engine();
        let this: Ptr<Animation> = e.new_object();
        // No queued idle.
        assert!(!e.call(0x0049_8170, &args![this]).bool());
        // A queued idle that is not loaded.
        let idle = fake_idle(&mut e, 0, 0, 0, 0);
        put_idles(&mut e, this, [0, idle, 0, 0]);
        assert!(!e.call(0x0049_8170, &args![this]).bool());
        // Loaded, but its model has no group: AddAnimation fails and the idle stays.
        e.mem.set_u32(idle + 8, 1);
        let kf = e.mem.alloc(0x20);
        e.mem.set_u32(idle + 0x10, kf);
        assert!(!e.call(0x0049_8170, &args![this]).bool());
        assert_eq!(idle_fields(&e, this)[1], idle);
        // The model's group is known to the animation: added, the idle is dropped.
        let group = group_with_id(&mut e, 1);
        let sequence = sequence_with_group(&mut e, 0, group);
        let kf = addable_model(&mut e, this, 1, sequence);
        e.mem.set_u32(idle + 0x10, kf);
        start_log(&mut e);
        assert!(e.call(0x0049_8170, &args![this]).bool());
        let log = end_log(&mut e);
        assert_eq!(idle_fields(&e, this)[1], 0);
        assert_eq!(arguments_of(&log, IDLE_KF_MODEL), [[idle]]);
    }

    #[test]
    fn special_idle_loaded_needs_a_current_idle_in_state_1() {
        let mut e = idle_engine();
        let this: Ptr<Animation> = e.new_object();
        assert!(!e.call(0x0049_81f0, &args![this]).bool());
        for (state, expected) in [(0, false), (1, true), (2, false)] {
            let idle = fake_idle(&mut e, state, 0, 0, 0);
            put_idles(&mut e, this, [idle, 0, 0, 0]);
            assert_eq!(e.call(0x0049_81f0, &args![this]).bool(), expected);
        }
    }

    #[test]
    fn special_idle_playing_needs_a_current_idle_in_state_2() {
        let mut e = idle_engine();
        let this: Ptr<Animation> = e.new_object();
        assert!(!e.call(0x0049_85b0, &args![this]).bool());
        for (state, expected) in [(0, false), (1, false), (2, true)] {
            let idle = fake_idle(&mut e, state, 0, 0, 0);
            put_idles(&mut e, this, [idle, 0, 0, 0]);
            assert_eq!(e.call(0x0049_85b0, &args![this]).bool(), expected);
        }
    }

    #[test]
    fn special_idle_done_playing_follows_the_queue_and_the_state() {
        let mut e = idle_engine();
        let this: Ptr<Animation> = e.new_object();
        // Nothing at all: done.
        assert!(e.call(0x0049_85f0, &args![this]).bool());
        // A queued idle: not done.
        let queued = fake_idle(&mut e, 1, 0, 0, 0);
        put_idles(&mut e, this, [0, queued, 0, 0]);
        assert!(!e.call(0x0049_85f0, &args![this]).bool());
        // A current idle without a sequence: not done; with one, done in state 3.
        let idle = fake_idle(&mut e, 3, 0, 0, 0);
        put_idles(&mut e, this, [idle, 0, 0, 0]);
        assert!(!e.call(0x0049_85f0, &args![this]).bool());
        e.mem.set_u32(idle + 0x18, 0x5000);
        assert!(e.call(0x0049_85f0, &args![this]).bool());
        e.mem.set_u32(idle + 8, 2);
        assert!(!e.call(0x0049_85f0, &args![this]).bool());
    }

    #[test]
    fn current_idle_sequence_is_the_sequence_of_the_current_idle() {
        let mut e = idle_engine();
        let this: Ptr<Animation> = e.new_object();
        assert_eq!(e.call(0x0049_8cf0, &args![this]).u32(), 0);
        let idle = fake_idle(&mut e, 1, 0, 0, 0x5000);
        put_idles(&mut e, this, [idle, 0, 0, 0]);
        assert_eq!(e.call(0x0049_8cf0, &args![this]).u32(), 0x5000);
    }

    #[test]
    fn special_idle_working_looks_at_the_forms_and_states_of_the_idles() {
        let mut e = idle_engine();
        e.register(WORD_AT_8, |e, a| ret(e.mem.u32(a[0] + 8)));
        let this: Ptr<Animation> = e.new_object();
        let form = 0x6000u32;
        e.map(0x011f_2000, 0x1000);
        assert!(!e.call(0x0049_8d30, &args![this, form]).bool());
        // The current idle of the form works unless it is only loaded.
        let idle = fake_idle(&mut e, 1, 0, 0, 0);
        e.mem.set_u32(idle + 0x2c, form);
        put_idles(&mut e, this, [idle, 0, 0, 0]);
        assert!(!e.call(0x0049_8d30, &args![this, form]).bool());
        e.mem.set_u32(idle + 8, 2);
        assert!(e.call(0x0049_8d30, &args![this, form]).bool());
        assert!(!e.call(0x0049_8d30, &args![this, 0x7000u32]).bool());
        // The same goes for the queued idle.
        put_idles(&mut e, this, [0, idle, 0, 0]);
        assert!(e.call(0x0049_8d30, &args![this, form]).bool());
        // A parked idle of the form counts whatever its state is; another
        // parked idle counts only while the VATS word is 4 and it has a sequence.
        let parked = fake_idle(&mut e, 1, 0, 0, 0);
        e.mem.set_u32(parked + 0x2c, form);
        put_idles(&mut e, this, [0, 0, parked, 0]);
        assert!(e.call(0x0049_8d30, &args![this, form]).bool());
        let other = fake_idle(&mut e, 1, 0, 0, 0x5000);
        put_idles(&mut e, this, [0, 0, 0, other]);
        assert!(!e.call(0x0049_8d30, &args![this, form]).bool());
        e.mem.set_u32(VATS_OBJECT + 8, 4);
        assert!(e.call(0x0049_8d30, &args![this, form]).bool());
        e.mem.set_u32(other + 0x18, 0);
        assert!(!e.call(0x0049_8d30, &args![this, form]).bool());
    }

    #[test]
    fn special_idle_working_ov2_asks_the_four_idles_for_the_sequence() {
        let mut e = idle_engine();
        let this: Ptr<Animation> = e.new_object();
        assert!(!e.call(0x0049_8ea0, &args![this, 0x5000u32]).bool());
        let mut idles = [0u32; 4];
        for (i, idle) in idles.iter_mut().enumerate() {
            *idle = fake_idle(&mut e, 1, 0, 0, 0x5000 + i as u32);
        }
        put_idles(&mut e, this, idles);
        for i in 0..4u32 {
            assert!(e.call(0x0049_8ea0, &args![this, 0x5000 + i]).bool());
        }
        assert!(!e.call(0x0049_8ea0, &args![this, 0x5004u32]).bool());
    }

    #[test]
    fn special_idle_working_ov3_is_true_while_an_idle_starts_or_is_unfinished() {
        let mut e = idle_engine();
        e.register(WORD_AT_8, |e, a| ret(e.mem.u32(a[0] + 8)));
        let this: Ptr<Animation> = e.new_object();
        assert!(!e.call(0x0049_8f80, &args![this]).bool());
        // A current idle without a sequence: working unless it is done.
        let idle = fake_idle(&mut e, 1, 0, 0, 0);
        put_idles(&mut e, this, [idle, 0, 0, 0]);
        assert!(e.call(0x0049_8f80, &args![this]).bool());
        e.mem.set_u32(idle + 8, 3);
        assert!(!e.call(0x0049_8f80, &args![this]).bool());
        // With a sequence: working while the sequence just started (2 or 5).
        let sequence = e.mem.alloc(0x80);
        e.mem.set_u32(idle + 0x18, sequence);
        for (state, expected) in [(0, false), (2, true), (5, true), (1, false)] {
            e.mem.set_u32(sequence + 0x44, state);
            assert_eq!(e.call(0x0049_8f80, &args![this]).bool(), expected);
        }
        // The queued idle: working without a sequence, or while it just started.
        let queued = fake_idle(&mut e, 1, 0, 0, 0);
        put_idles(&mut e, this, [0, queued, 0, 0]);
        assert!(e.call(0x0049_8f80, &args![this]).bool());
        e.mem.set_u32(queued + 0x18, sequence);
        for (state, expected) in [(0, false), (2, true), (5, true)] {
            e.mem.set_u32(sequence + 0x44, state);
            assert_eq!(e.call(0x0049_8f80, &args![this]).bool(), expected);
        }
    }

    /// An animation with a controller manager (+0xd8) and a sequence map, and
    /// doubles for the calls the idle functions make to remove things from them.
    fn managed_animation(e: &mut Engine) -> (Ptr<Animation>, u32, u32) {
        let this: Ptr<Animation> = e.new_object();
        let manager = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0xd8, manager);
        let map = map_for(e, this);
        e.register(MANAGER_REMOVE_SEQUENCE, |_, _| Ret::default());
        e.register(MAP_REMOVE_AT, |_, _| ret(1));
        e.register(NODE_ACCEPTS_OBJECT, |_, a| ret((a[1] != 0) as u32));
        e.register(MENU_MODE_TYPE, |_, _| ret(0));
        (this, manager, map)
    }

    /// A sequence in state `state` whose group has the id `id`, named
    /// `"seq.kf"`.
    fn named_sequence(e: &mut Engine, state: u32, id: u16) -> u32 {
        let group = group_with_id(e, id);
        let sequence = sequence_with_group(e, state, group);
        let name = cstring(e, "seq.kf");
        e.mem.set_u32(sequence + 8, name);
        sequence
    }

    #[test]
    fn anim_idle_free_without_a_map_entry_removes_the_held_sequence() {
        let mut e = idle_engine();
        let (this, manager, _map) = managed_animation(&mut e);
        let sequence = named_sequence(&mut e, 0, 3);
        let idle = fake_idle(&mut e, 1, 0, 3, sequence);
        put_idles(&mut e, this, [idle, 0, 0, 0]);
        e.mem.set_u32(this.addr() + 0x100, 0x7777);
        start_log(&mut e);
        e.call(0x0049_8670, &args![this, this.addr() + 0x124]);
        let log = end_log(&mut e);
        // The sequence is taken out of the manager and forgotten as the last
        // movement sequence (the double accepts any non-null object).
        assert_eq!(
            arguments_of(&log, MANAGER_REMOVE_SEQUENCE),
            [[manager, sequence]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x100), 0);
        // The idle field is emptied; the local pointer is released; nothing is
        // removed from the map as the held sequence is still set.
        assert_eq!(idle_fields(&e, this), [0; 4]);
        assert_eq!(arguments_of(&log, NI_POINTER_RELEASE).len(), 1);
        assert!(arguments_of(&log, MAP_REMOVE_AT).is_empty());
        // A shut down animation leaves the manager alone.
        put_idles(&mut e, this, [idle, 0, 0, 0]);
        e.set(this, Animation::bShutDown, true);
        start_log(&mut e);
        e.call(0x0049_8670, &args![this, this.addr() + 0x124]);
        let log = end_log(&mut e);
        assert!(arguments_of(&log, MANAGER_REMOVE_SEQUENCE).is_empty());
        // An idle without a sequence removes the map key 0xff instead.
        let bare = fake_idle(&mut e, 1, 0, 3, 0);
        put_idles(&mut e, this, [bare, 0, 0, 0]);
        start_log(&mut e);
        e.call(0x0049_8670, &args![this, this.addr() + 0x124]);
        let log = end_log(&mut e);
        let map = e.get(this, Animation::pAnimSequenceMap).addr();
        assert_eq!(arguments_of(&log, MAP_REMOVE_AT), [[map, 0xff]]);
    }

    #[test]
    fn anim_idle_free_with_a_matching_map_entry_deletes_the_entry() {
        let mut e = idle_engine();
        let (this, manager, map) = managed_animation(&mut e);
        let sequence = named_sequence(&mut e, 0, 7);
        let kf = addable_model(&mut e, this, 7, sequence);
        let entry = e.mem.u32(map + 8);
        // The entry's scalar deleting destructor (slot 0).
        e.register(0x00f1_0000, |_, _| Ret::default());
        let vtable = e.mem.u32(entry);
        e.mem.set_u32(vtable, 0x00f1_0000);
        let idle = fake_idle(&mut e, 1, 0, 3, sequence);
        e.mem.set_u32(idle + 0x10, kf);
        put_idles(&mut e, this, [idle, 0, 0, 0]);
        e.mem.set_u32(this.addr() + 0xe0 + 12, sequence);
        e.mem.set_u32(this.addr() + 0x100, sequence);
        start_log(&mut e);
        e.call(0x0049_8670, &args![this, this.addr() + 0x124]);
        let log = end_log(&mut e);
        // Slot 3 held the idle's sequence: it is cleared.
        assert_eq!(e.mem.u32(this.addr() + 0xe0 + 12), 0);
        assert_eq!(e.mem.u16(this.addr() + 0x4c + 6), 0xff);
        // The sequence is removed from the manager and forgotten; the map entry
        // is removed and deleted.
        assert_eq!(
            arguments_of(&log, MANAGER_REMOVE_SEQUENCE),
            [[manager, sequence]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x100), 0);
        // The key is removed twice: once with the entry, and once more as there
        // is neither an entry nor a held sequence any more.
        assert_eq!(arguments_of(&log, MAP_REMOVE_AT), [[map, 7], [map, 7]]);
        assert_eq!(arguments_of(&log, 0x00f1_0000), [[entry, 1]]);
        assert_eq!(idle_fields(&e, this), [0; 4]);
        // An entry whose sequence is another one is ignored.
        let other = named_sequence(&mut e, 0, 7);
        let vtable = e.mem.u32(entry);
        let _ = vtable;
        e.mem.set_u32(entry + 4, other);
        put_idles(&mut e, this, [idle, 0, 0, 0]);
        start_log(&mut e);
        e.call(0x0049_8670, &args![this, this.addr() + 0x124]);
        let log = end_log(&mut e);
        assert!(arguments_of(&log, 0x00f1_0000).is_empty());
        assert!(arguments_of(&log, MAP_REMOVE_AT).is_empty());
        assert_eq!(
            arguments_of(&log, MANAGER_REMOVE_SEQUENCE),
            [[manager, sequence]]
        );
    }

    /// A current idle whose sequence plays in slot `slot`.
    fn playing_idle(e: &mut Engine, this: Ptr<Animation>, section: u32, state: u32) -> (u32, u32) {
        let sequence = named_sequence(e, state, 3);
        let idle = fake_idle(e, 1, 0, section, sequence);
        let slot = match section {
            0x14 => 1,
            0x15 => 4,
            other => other,
        };
        e.mem.set_u32(this.addr() + 0xe0 + 4 * slot, sequence);
        (idle, sequence)
    }

    #[test]
    fn special_idle_free_blends_the_current_idle_out_and_parks_it() {
        let mut e = idle_engine();
        let (this, _manager, _map) = managed_animation(&mut e);
        let (idle, _sequence) = playing_idle(&mut e, this, 2, 0);
        put_idles(&mut e, this, [idle, 0, 0, 0]);
        start_log(&mut e);
        e.call(0x0049_8910, &args![this, 0u32, 0u32]);
        let log = end_log(&mut e);
        // The section is blended out (the slot is cleared) and the idle is parked
        // in the first waiting place; the queued idle (none) takes its place.
        assert_eq!(e.mem.u32(this.addr() + 0xe0 + 8), 0);
        assert_eq!(idle_fields(&e, this), [0, 0, idle, 0]);
        assert_eq!(
            arguments_of(&log, NI_POINTER_COPY)[0],
            [this.addr() + 0x12c, this.addr() + 0x124]
        );
        // Section 0x14 is blended out as slot 1, which resets the movement.
        let (this, _, _) = managed_animation(&mut e);
        let (idle, _) = playing_idle(&mut e, this, 0x14, 0);
        put_idles(&mut e, this, [idle, 0, 0x1111, 0]);
        e.mem.set_f32(this.addr() + 0x10, 1.0);
        e.call(0x0049_8910, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.f32(this.addr() + 0x10), 0.0);
        assert_eq!(e.mem.u32(this.addr() + 0xe0 + 4), 0);
        assert_eq!(idle_fields(&e, this), [0, 0, 0x1111, idle]);
    }

    #[test]
    fn special_idle_free_with_immediate_forgets_the_sequence_and_frees_the_idle() {
        let mut e = idle_engine();
        let (this, manager, _map) = managed_animation(&mut e);
        let (idle, sequence) = playing_idle(&mut e, this, 2, 0);
        put_idles(&mut e, this, [idle, 0, 0, 0]);
        start_log(&mut e);
        e.call(0x0049_8910, &args![this, 0u32, 1u32]);
        let log = end_log(&mut e);
        assert_eq!(e.mem.u32(this.addr() + 0xe0 + 8), 0);
        // Freed on the spot: removed from the manager, nothing parked.
        assert_eq!(
            arguments_of(&log, MANAGER_REMOVE_SEQUENCE),
            [[manager, sequence]]
        );
        assert_eq!(idle_fields(&e, this), [0; 4]);
    }

    #[test]
    fn special_idle_free_gives_up_on_an_idle_that_just_started() {
        for state in [2, 5] {
            let mut e = idle_engine();
            let (this, _manager, _map) = managed_animation(&mut e);
            let (idle, sequence) = playing_idle(&mut e, this, 2, state);
            put_idles(&mut e, this, [idle, 0, 0, 0]);
            start_log(&mut e);
            e.call(0x0049_8910, &args![this, 0u32, 0u32]);
            let log = end_log(&mut e);
            let name = e.mem.u32(sequence + 8);
            assert_eq!(
                arguments_of(&log, LOG),
                [[LOG_IDLE_FREE_JUST_STARTED, name]]
            );
            assert_eq!(idle_fields(&e, this), [idle, 0, 0, 0]);
            assert_eq!(e.mem.u32(this.addr() + 0xe0 + 8), sequence);
        }
    }

    #[test]
    fn special_idle_free_frees_the_idle_when_both_waiting_places_are_taken() {
        let mut e = idle_engine();
        let (this, manager, _map) = managed_animation(&mut e);
        let (idle, sequence) = playing_idle(&mut e, this, 2, 0);
        put_idles(&mut e, this, [idle, 0, 0x1111, 0x2222]);
        start_log(&mut e);
        e.call(0x0049_8910, &args![this, 0u32, 0u32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, MANAGER_REMOVE_SEQUENCE),
            [[manager, sequence]]
        );
        assert_eq!(idle_fields(&e, this), [0, 0, 0x1111, 0x2222]);
    }

    #[test]
    fn special_idle_free_handles_the_queued_idle() {
        // Without `free_queued` the queued idle becomes the current one.
        let mut e = idle_engine();
        let (this, _manager, _map) = managed_animation(&mut e);
        let queued = fake_idle(&mut e, 1, 0, 0, 0);
        put_idles(&mut e, this, [0, queued, 0, 0]);
        e.call(0x0049_8910, &args![this, 0u32, 0u32]);
        assert_eq!(idle_fields(&e, this), [queued, 0, 0, 0]);
        // With it, the queued idle is blended out by slot and parked.
        let (this, _, _) = managed_animation(&mut e);
        let (queued, _sequence) = playing_idle(&mut e, this, 0x15, 0);
        put_idles(&mut e, this, [0, queued, 0, 0]);
        e.call(0x0049_8910, &args![this, 1u32, 0u32]);
        assert_eq!(e.mem.u32(this.addr() + 0xe0 + 16), 0);
        assert_eq!(idle_fields(&e, this), [0, 0, queued, 0]);
        // A queued idle that just started is kept and stops the function.
        let (this, _, _) = managed_animation(&mut e);
        let (queued, _) = playing_idle(&mut e, this, 3, 5);
        put_idles(&mut e, this, [0, queued, 0, 0]);
        e.call(0x0049_8910, &args![this, 1u32, 0u32]);
        assert_eq!(idle_fields(&e, this), [0, queued, 0, 0]);
    }

    /// The state `fn_00498290` needs to start the current idle: a model whose
    /// group the animation already knows, a sequence that plays in slot 0 (so
    /// `StartGroup` returns it at once) and doubles for the idle form's getters.
    struct IdleStart {
        e: Engine,
        this: Ptr<Animation>,
        idle: u32,
        sequence: u32,
        form: u32,
    }

    fn idle_start() -> IdleStart {
        let mut e = idle_engine();
        let (this, _manager, _map) = managed_animation(&mut e);
        set_type(&mut e, 1, 0, 4);
        let sequence = named_sequence(&mut e, 1, 1);
        let kf = addable_model(&mut e, this, 1, sequence);
        e.mem.set_u32(this.addr() + 0xe0, sequence);
        e.register(UPDATE_BIP_ONLY, |_, _| Ret::default());
        e.register(SPECIAL_IDLE_WORKING, |_, _| ret(0));
        e.register(IS_IN_MENU_MODE, |_, _| ret(0));
        e.register(IDLE_FORM_BYTE, |e, a| ret(e.mem.u8(a[0] + 0x39) as u32));
        e.register(IDLE_FORM_WORD, |e, a| ret(e.mem.u16(a[0] + 0x3c) as u32));
        e.register(IDLE_SET_WORD_8, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(IDLE_SET_ACTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x34, a[1]);
            Ret::default()
        });
        e.register(ACTOR_SET_ANIM_ACTION, |_, _| Ret::default());
        e.register(SIMPLE_LIST_PUSH_BACK, |e, a| {
            // Record the pushed item at a fixed place.
            let item = e.mem.u32(a[1]);
            let form = e.mem.u32(item);
            let delay = e.mem.u32(item + 4);
            e.mem.set_u32(0x011c_5100, form);
            e.mem.set_u32(0x011c_5104, delay);
            e.mem.set_u32(0x011c_5108, a[0]);
            Ret::default()
        });
        let form = e.mem.alloc(0x80);
        let idle = fake_idle(&mut e, 1, 1, 0, 0);
        e.mem.set_u32(idle + 0x10, kf);
        e.mem.set_u32(idle + 0x2c, form);
        put_idles(&mut e, this, [idle, 0, 0, 0]);
        IdleStart {
            e,
            this,
            idle,
            sequence,
            form,
        }
    }

    #[test]
    fn fn_00498290_starts_the_loaded_idle() {
        let mut f = idle_start();
        let (this, idle, sequence) = (f.this, f.idle, f.sequence);
        let e = &mut f.e;
        assert!(e.call(0x0049_8290, &args![this]).bool());
        // The idle plays the sequence the group started.
        assert_eq!(e.mem.u32(idle + 8), 2);
        assert_eq!(e.mem.u32(idle + 0x18), sequence);
        // No replay delay: the idle form's word is 0.
        assert_eq!(e.mem.u32(0x011c_5108), 0);
    }

    #[test]
    fn fn_00498290_queues_a_replay_delay_for_the_idle_form() {
        let mut f = idle_start();
        let (this, form) = (f.this, f.form);
        let e = &mut f.e;
        e.mem.set_u16(form + 0x3c, 5);
        assert!(e.call(0x0049_8290, &args![this]).bool());
        assert_eq!(e.mem.u32(0x011c_5100), form);
        assert_eq!(e.mem.f32(0x011c_5104), 5.0);
        assert_eq!(e.mem.u32(0x011c_5108), this.addr() + 0x134);
    }

    #[test]
    fn fn_00498290_plays_other_sections_with_the_loop_byte_of_the_form() {
        let mut f = idle_start();
        let (this, idle, form, sequence) = (f.this, f.idle, f.form, f.sequence);
        let e = &mut f.e;
        e.mem.set_u32(idle + 0x14, 3);
        e.mem.set_u8(form + 0x39, 4);
        // A category 1 group takes the loop count.
        set_type(e, 1, 0, 1);
        assert!(e.call(0x0049_8290, &args![this]).bool());
        assert_eq!(e.mem.u32(idle + 0x18), sequence);
        // The loop count of the slot is the form's byte.
        assert_eq!(e.mem.i32(this.addr() + 0x7c + 12), 4);
    }

    #[test]
    fn fn_00498290_gives_up_on_a_model_it_cannot_add() {
        let mut f = idle_start();
        let (this, idle) = (f.this, f.idle);
        let e = &mut f.e;
        // The model has no group.
        let kf = e.mem.alloc(0x20);
        e.mem.set_u32(idle + 0x10, kf);
        assert!(!e.call(0x0049_8290, &args![this]).bool());
        // The idle is done; a play type of 1 leaves it where it is.
        assert_eq!(e.mem.u32(idle + 8), 3);
        assert_eq!(idle_fields(e, this), [idle, 0, 0, 0]);
        // A play type of 2 or 3 frees the idle (parked, as its sequence is empty).
        for kind in [2, 3] {
            e.mem.set_u32(idle + 0xc, kind);
            put_idles(e, this, [idle, 0, 0, 0]);
            e.mem.set_u32(idle + 8, 1);
            assert!(!e.call(0x0049_8290, &args![this]).bool());
            assert_eq!(idle_fields(e, this), [0, 0, idle, 0]);
            e.mem.set_u32(this.addr() + 0x12c, 0);
        }
    }

    #[test]
    fn fn_00498290_needs_a_loaded_current_idle() {
        let mut f = idle_start();
        let (this, idle) = (f.this, f.idle);
        let e = &mut f.e;
        e.mem.set_u32(idle + 8, 0);
        assert!(!e.call(0x0049_8290, &args![this]).bool());
        put_idles(e, this, [0, 0, 0, 0]);
        assert!(!e.call(0x0049_8290, &args![this]).bool());
        // A queued idle that is not loaded stops it as well.
        let queued = fake_idle(e, 0, 0, 0, 0);
        put_idles(e, this, [idle, queued, 0, 0]);
        e.mem.set_u32(idle + 8, 1);
        assert!(!e.call(0x0049_8290, &args![this]).bool());
        assert_eq!(idle_fields(e, this), [idle, queued, 0, 0]);
        // A loaded queued idle replaces the current one, which is started.
        e.mem.set_u32(queued + 8, 1);
        let kf = e.mem.u32(idle + 0x10);
        let form = e.mem.u32(idle + 0x2c);
        e.mem.set_u32(queued + 0x10, kf);
        e.mem.set_u32(queued + 0x2c, form);
        e.mem.set_u32(queued + 0xc, 1);
        put_idles(e, this, [0, queued, 0, 0]);
        assert!(e.call(0x0049_8290, &args![this]).bool());
        assert_eq!(idle_fields(e, this)[..2], [queued, 0]);
    }

    #[test]
    fn fn_00498230_hands_the_actor_to_the_started_idle() {
        let mut f = idle_start();
        let (this, idle, sequence) = (f.this, f.idle, f.sequence);
        let e = &mut f.e;
        let actor = e.mem.alloc(0x40);
        e.register_double(ACTOR_SET_ANIM_ACTION, move |e, a| {
            e.mem.set_u32(0x011c_5110, a[0]);
            e.mem.set_u32(0x011c_5114, a[1]);
            e.mem.set_u32(0x011c_5118, a[2]);
            Ret::default()
        });
        assert!(e.call(0x0049_8230, &args![this, actor]).bool());
        assert_eq!(e.mem.u32(idle + 0x34), actor);
        assert_eq!(
            [
                e.mem.u32(0x011c_5110),
                e.mem.u32(0x011c_5114),
                e.mem.u32(0x011c_5118)
            ],
            [actor, 0xd, sequence]
        );
        // Nothing to start: false, and the actor is left alone.
        put_idles(e, this, [0, 0, 0, 0]);
        e.mem.set_u32(0x011c_5110, 0);
        assert!(!e.call(0x0049_8230, &args![this, actor]).bool());
        assert_eq!(e.mem.u32(0x011c_5110), 0);
    }

    /// A controller manager (the one of `this`, made if needed) whose sequence
    /// array holds `sequences` (the array at +0x34: the elements and the count).
    fn manager_with_sequences(e: &mut Engine, this: Ptr<Animation>, sequences: &[u32]) -> u32 {
        let mut manager = e.mem.u32(this.addr() + 0xd8);
        if manager == 0 {
            manager = e.mem.alloc(0x80);
            e.mem.set_u32(this.addr() + 0xd8, manager);
        }
        let elements = e.mem.alloc(4 * sequences.len() as u32 + 4);
        for (i, sequence) in sequences.iter().enumerate() {
            e.mem.set_u32(elements + 4 * i as u32, *sequence);
        }
        e.mem.set_u32(manager + 0x34, elements);
        e.mem.set_u32(manager + 0x38, sequences.len() as u32);
        e.register(SEQUENCE_ARRAY_COUNT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(SEQUENCE_ARRAY_ELEMENT, |e, a| {
            ret(e.mem.u32(a[0]) + 4 * a[1])
        });
        manager
    }

    /// `kfModelList` (embedded in the animation at +0x104) holding `models`,
    /// and the doubles of the `BSSimpleList` accessors.
    fn kf_model_list(e: &mut Engine, this: Ptr<Animation>, models: &[u32]) {
        let list = make_simple_list(e, models);
        let (item, next) = (e.mem.u32(list), e.mem.u32(list + 4));
        e.mem.set_u32(this.addr() + 0x104, item);
        e.mem.set_u32(this.addr() + 0x108, next);
        e.register(MODEL_QUEUE_NOT_EMPTY, |e, a| {
            ret((e.mem.u32(a[0] + 0x104) != 0 || e.mem.u32(a[0] + 0x108) != 0) as u32)
        });
    }

    #[test]
    fn clear_blend_interps_clears_the_sequences_and_releases_the_queued_models() {
        let mut e = idle_engine();
        let this: Ptr<Animation> = e.new_object();
        // Without a manager nothing happens.
        e.call(0x0049_9080, &args![this]);
        let first = e.mem.alloc(0x80);
        let second = e.mem.alloc(0x80);
        e.mem.set_u32(first + 0x70, 1);
        manager_with_sequences(&mut e, this, &[first, second]);
        e.register(DYNAMIC_CAST, |e, a| {
            assert_eq!(a[0], RTTI_ANIM_GROUP_SEQUENCE);
            ret(if e.mem.u32(a[1] + 0x70) == 1 { a[1] } else { 0 })
        });
        e.register(SEQUENCE_CLEAR_INTERPOLATORS, |_, _| Ret::default());
        e.register(KF_MODEL_RELEASE, |_, _| Ret::default());
        kf_model_list(&mut e, this, &[0x1111, 0x2222]);
        start_log(&mut e);
        e.call(0x0049_9080, &args![this]);
        let log = end_log(&mut e);
        // Only the object that casts to a sequence is cleared.
        assert_eq!(arguments_of(&log, SEQUENCE_CLEAR_INTERPOLATORS), [[first]]);
        // Each queued model is released twice and the list ends up empty.
        let releases: Vec<u32> = arguments_of(&log, KF_MODEL_RELEASE)
            .iter()
            .map(|words| words[0])
            .collect();
        assert_eq!(releases, [0x1111, 0x1111, 0x2222, 0x2222]);
        assert_eq!(e.mem.u32(this.addr() + 0x104), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x108), 0);
    }

    #[test]
    fn clear_controllers_interpolators_walks_the_controllers_and_the_children() {
        let mut e = idle_engine();
        e.register(OBJECT_CONTROLLERS, |e, a| ret(e.mem.u32(a[0] + 0x50)));
        e.register(CONTROLLER_NEXT, |e, a| ret(e.mem.u32(a[0] + 0x30)));
        e.register(DYNAMIC_CAST, |e, a| {
            let kind = e.mem.u32(a[1] + 0x40);
            let matches = (a[0] == RTTI_CONTROLLER_KIND && kind == 1)
                || (a[0] == RTTI_CONTROLLER_FIRST && kind == 2);
            ret(if matches { a[1] } else { 0 })
        });
        e.register(CONTROLLER_RESET, |_, _| Ret::default());
        e.register(0x00f2_00c8, |_, _| Ret::default());
        e.register(0x00f2_000c, |e, a| ret(e.mem.u32(a[0] + 0x60)));
        e.register(NODE_CHILD_COUNT, |e, a| ret(e.mem.u32(a[0] + 0x9c)));
        e.register(NODE_CHILD_AT, |e, a| ret(e.mem.u32(a[0] + 0xa0 + 4 * a[1])));
        let controller = |e: &mut Engine, kind: u32| {
            let object = object_with_vtable(e, 0x80, &[(0xc8, 0x00f2_00c8)]);
            e.mem.set_u32(object + 0x40, kind);
            object
        };
        let node = |e: &mut Engine| object_with_vtable(e, 0x100, &[(0xc, 0x00f2_000c)]);
        // The root has a kind-1 controller, an unrelated one and a kind-2 one;
        // its only child has a kind-2 controller and is not a node itself.
        let (kind_one, other, kind_two) = (
            controller(&mut e, 1),
            controller(&mut e, 0),
            controller(&mut e, 2),
        );
        e.mem.set_u32(kind_one + 0x30, other);
        e.mem.set_u32(other + 0x30, kind_two);
        let root = node(&mut e);
        e.mem.set_u32(root + 0x50, kind_one);
        e.mem.set_u32(root + 0x60, root);
        let child_controller = controller(&mut e, 2);
        let child = node(&mut e);
        e.mem.set_u32(child + 0x50, child_controller);
        e.mem.set_u32(root + 0x9c, 1);
        e.mem.set_u32(root + 0xa0, child);
        start_log(&mut e);
        e.call(0x0049_9160, &args![0u32]);
        e.call(0x0049_9160, &args![root]);
        let log = end_log(&mut e);
        // The kind-1 controller gets its slot 0xc8 called with (0, 0); the
        // kind-2 controllers of the root and of the child are reset.
        assert_eq!(arguments_of(&log, 0x00f2_00c8), [[kind_one, 0, 0]]);
        assert_eq!(
            arguments_of(&log, CONTROLLER_RESET),
            [[kind_two], [child_controller]]
        );
    }

    #[test]
    fn reload_targets_retargets_every_sequence_of_the_manager() {
        let mut e = idle_engine();
        let this: Ptr<Animation> = e.new_object();
        // Without a manager nothing happens.
        e.call(0x0049_9240, &args![this, 1u32]);
        let (first, second) = (named_sequence(&mut e, 0, 1), named_sequence(&mut e, 0, 2));
        e.mem.set_u32(first + 0xc, 3);
        e.mem.set_u32(second + 0xc, 5);
        let hidden = cstring(&mut e, "__hidden");
        e.mem.set_u32(second + 8, hidden);
        let manager = manager_with_sequences(&mut e, this, &[first, second]);
        let root = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 8, root);
        e.register(MANAGER_PALETTE, |_, _| ret(0x9000));
        e.register(DYNAMIC_CAST, |_, a| {
            ret(if a[0] == RTTI_PALETTE_KIND { a[1] } else { 0 })
        });
        e.register(PALETTE_OBJECT_RESET, |_, _| Ret::default());
        e.register(GET_CONTROLLER, |e, a| {
            // The root holds the two controllers; the first one is removed.
            let first = e.mem.u32(a[0]);
            let second = e.mem.u32(a[0] + 4);
            ret(if a[1] == RTTI_CONTROLLER_FIRST {
                first
            } else {
                second
            })
        });
        e.register(REMOVE_CONTROLLER, |e, a| {
            if a[1] == e.mem.u32(a[0]) {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
        e.register(SEQUENCE_RETARGET, |_, _| Ret::default());
        e.register(SEQUENCE_ATTACH, |_, _| Ret::default());
        e.register(SEQUENCE_PREPARE, |_, _| Ret::default());
        e.register(MANAGER_ACTIVATE, |_, _| Ret::default());
        e.mem.set_u32(root, 0xc1);
        e.mem.set_u32(root + 4, 0xc2);
        let current = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0xe0 + 8, current);
        start_log(&mut e);
        e.call(0x0049_9240, &args![this, 1u32]);
        let log = end_log(&mut e);
        // The palette object is reset and both controllers are removed.
        assert_eq!(arguments_of(&log, PALETTE_OBJECT_RESET), [[0x9000]]);
        assert_eq!(
            arguments_of(&log, REMOVE_CONTROLLER),
            [[root, 0xc1], [root, 0xc2]]
        );
        // The first controller is gone, so every sequence gets the root and the
        // largest object count (5), and the one named "__..." is not attached.
        assert_eq!(
            arguments_of(&log, SEQUENCE_RETARGET),
            [[first, root, 5], [second, root, 5]]
        );
        assert_eq!(arguments_of(&log, SEQUENCE_ATTACH), [[first, root]]);
        assert!(arguments_of(&log, SEQUENCE_PREPARE).is_empty());
        // The sequence in a slot is activated again.
        assert_eq!(
            arguments_of(&log, MANAGER_ACTIVATE),
            [[manager, current, 0, 0, 1.0f32.to_bits(), 0, 0]]
        );
        // Without the flag the sequences are prepared and re-aimed instead.
        start_log(&mut e);
        e.call(0x0049_9240, &args![this, 0u32]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, SEQUENCE_PREPARE), [[first], [second]]);
        assert_eq!(
            arguments_of(&log, SEQUENCE_RETARGET),
            [[first, root, 5], [second, root, 5]]
        );
        assert!(arguments_of(&log, SEQUENCE_ATTACH).is_empty());
    }

    /// The doubles `BlendOut` goes through for an actor: whether a weapon is
    /// drawn (+0x10 of the actor), its anim action (+0x20), the process (+0x68)
    /// and the move type of a group (+0x30).
    fn blend_actor(e: &mut Engine) -> u32 {
        e.register(ACTOR_IS_WEAPON_DRAWN, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(ACTOR_GET_ANIM_ACTION, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(DATA_HANDLER_FLAG, |_, _| ret(0));
        e.register(GROUP_MOVE_TYPE, |e, a| ret(e.mem.u32(a[0] + 0x30)));
        e.register(ACTOR_FLAGS_WORD, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        e.register(ACTOR_START_ANIMATION, |_, _| Ret::default());
        e.register(MANAGER_DEACTIVATE, |_, _| Ret::default());
        e.register(SEQUENCE_WORD_58, |_, _| ret(0));
        e.register(GROUP_TYPE_IS_IRON_SIGHTS, |_, a| ret((a[0] == 0x44) as u32));
        e.set_global(DATA_HANDLER, 0x6000u32);
        let process = object_with_vtable(e, 0x400, &[(SLOT_PROCESS_HELD_OBJECT, 0x00f2_03e8)]);
        e.register(0x00f2_03e8, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        let actor = e.mem.alloc(0x100);
        e.mem.set_u32(actor + 0x10, 1);
        e.mem.set_u32(actor + 0x68, process);
        actor
    }

    #[test]
    fn blend_out_clears_the_slot_with_the_blend_time() {
        let mut e = idle_engine();
        let (this, manager, _map) = managed_animation(&mut e);
        blend_actor(&mut e);
        // Slot 2 holds a sequence in state 1; its group has a byte of 30 (1 s),
        // divided by the movement scale 2.0.
        let sequence = named_sequence(&mut e, 1, 3);
        let group = e.mem.u32(sequence + 0x74);
        e.mem.set_u8(group + 0x29, 30);
        e.mem.set_u32(this.addr() + 0xe0 + 8, sequence);
        start_log(&mut e);
        e.call(0x0049_94f0, &args![this, 2u32, 0u32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, MANAGER_DEACTIVATE),
            [[manager, sequence, 0.5f32.to_bits()]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0xe0 + 8), 0);
        // Without the group's byte the blend time setting is used (0.5 / 2.0).
        e.mem.set_u8(group + 0x29, 0);
        e.mem.set_u32(this.addr() + 0xe0 + 8, sequence);
        start_log(&mut e);
        e.call(0x0049_94f0, &args![this, 2u32, 0u32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, MANAGER_DEACTIVATE),
            [[manager, sequence, 0.25f32.to_bits()]]
        );
        // `cSkipNextBlend` makes it 0.
        e.mem.set_u32(this.addr() + 0xe0 + 8, sequence);
        e.set(this, Animation::cSkipNextBlend, 1u8);
        start_log(&mut e);
        e.call(0x0049_94f0, &args![this, 2u32, 0u32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, MANAGER_DEACTIVATE),
            [[manager, sequence, 0]]
        );
    }

    #[test]
    fn blend_out_of_the_movement_slot_resets_the_movement() {
        let mut e = idle_engine();
        let (this, _manager, _map) = managed_animation(&mut e);
        blend_actor(&mut e);
        e.mem.set_f32(this.addr() + 0x10, 2.0);
        e.call(0x0049_94f0, &args![this, 0x14u32, 0u32]);
        assert_eq!(e.mem.f32(this.addr() + 0x10), 0.0);
        e.mem.set_f32(this.addr() + 0x10, 2.0);
        e.call(0x0049_94f0, &args![this, 2u32, 0u32]);
        assert_eq!(e.mem.f32(this.addr() + 0x10), 2.0);
    }

    #[test]
    fn blend_out_of_the_weapon_slot_starts_the_actors_animation() {
        let mut e = idle_engine();
        let (this, manager, _map) = managed_animation(&mut e);
        let actor = blend_actor(&mut e);
        e.set(this, Animation::pActorRef, Ptr::new(actor));
        // Slots 4, 5 and 6 hold sequences in state 1.
        let mut sequences = [0u32; 3];
        for (i, sequence) in sequences.iter_mut().enumerate() {
            *sequence = named_sequence(&mut e, 1, 3);
            e.mem
                .set_u32(this.addr() + 0xe0 + 16 + 4 * i as u32, *sequence);
        }
        // The anim action is 9, the actor is not the player and the blend time
        // (0.25) is short: with move type 1 and no flag 0x400 the time becomes
        // 0.5 for slots 5 and 6, and the actor starts the plain animation 0x11.
        e.mem.set_u32(actor + 0x20, 9);
        let group = e.mem.u32(sequences[0] + 0x74);
        e.mem.set_u32(group + 0x30, 1);
        start_log(&mut e);
        e.call(0x0049_94f0, &args![this, 4u32, 0u32]);
        let log = end_log(&mut e);
        let deactivations = arguments_of(&log, MANAGER_DEACTIVATE);
        assert_eq!(
            deactivations,
            [
                [manager, sequences[1], 0.5f32.to_bits()],
                [manager, sequences[2], 0.5f32.to_bits()]
            ]
        );
        assert_eq!(
            arguments_of(&log, ACTOR_START_ANIMATION),
            [[actor, 0x11, this.addr()]]
        );
        // Slot 4 itself stays: the weapon slot is cleared by its own ladder, not
        // by this call (the drawn weapon skips it).
        assert_eq!(e.mem.u32(this.addr() + 0xe0 + 16), sequences[0]);
    }

    #[test]
    fn blend_out_with_the_flag_restarts_the_held_animation() {
        let mut e = idle_engine();
        let (this, _manager, _map) = managed_animation(&mut e);
        let actor = blend_actor(&mut e);
        e.set(this, Animation::pActorRef, Ptr::new(actor));
        // The anim action is 2 and the process holds a sequence whose group has
        // type 0x44, an iron sights action: the actor starts type 0x41.
        e.mem.set_u32(actor + 0x20, 2);
        let held = named_sequence(&mut e, 0, 0x44);
        let process = e.mem.u32(actor + 0x68);
        e.mem.set_u32(process + 0x10, held);
        start_log(&mut e);
        e.call(0x0049_94f0, &args![this, 0x14u32, 1u32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, ACTOR_START_ANIMATION),
            [[actor, 0x41, this.addr()]]
        );
        // Without a held sequence it is the plain animation.
        e.mem.set_u32(process + 0x10, 0);
        start_log(&mut e);
        e.call(0x0049_94f0, &args![this, 0x14u32, 1u32]);
        let log = end_log(&mut e);
        assert_eq!(
            arguments_of(&log, ACTOR_START_ANIMATION),
            [[actor, 0x11, this.addr()]]
        );
        // No drawn weapon: nothing is started.
        e.mem.set_u32(actor + 0x10, 0);
        start_log(&mut e);
        e.call(0x0049_94f0, &args![this, 0x14u32, 1u32]);
        assert!(arguments_of(&end_log(&mut e), ACTOR_START_ANIMATION).is_empty());
    }

    #[test]
    fn fn_004997b0_wants_flag_0x400_without_0x800() {
        let mut e = idle_engine();
        e.register(ACTOR_FLAGS_WORD, |e, a| ret(e.mem.u32(a[0])));
        let actor = e.mem.alloc(8);
        for (flags, expected) in [(0, false), (0x400, true), (0x800, false), (0xc00, false)] {
            e.mem.set_u32(actor, flags);
            assert_eq!(e.call(0x0049_97b0, &args![actor]).bool(), expected);
        }
    }

    #[test]
    fn the_saved_sequence_keys_are_two_globals() {
        let mut e = Engine::new();
        e.map(0x011c_6000, 0x1000);
        e.set_global(SAVED_SEQUENCE_KEY_GLOBAL, 0x1234u32);
        e.set_global(SAVED_SEQUENCE_LOOKUP_GLOBAL, 0x5678u32);
        assert_eq!(e.call(0x0049_9b70, &args![]).u32(), 0x1234);
        assert_eq!(e.call(0x0049_9b80, &args![]).u32(), 0x5678);
    }

    /// The save game object's doubles. Writes append to a buffer in game memory
    /// (`out`, with its length in `len`); reads take their bytes from `input`
    /// (advancing `pos`); `blocks` and `version` are the save game's, `diag` is
    /// the diagnostics byte and `record` the pointer to the record being
    /// written or read.
    struct SaveIo {
        out: u32,
        len: u32,
        pos: u32,
        diag: u32,
        record: u32,
    }

    fn save_io(e: &mut Engine, blocks: bool, version: u32, reading: bool, input: &[u8]) -> SaveIo {
        e.map(0x011c_6000, 0x1000);
        e.set_global(SAVE_GAME_OBJECT, 0x7000u32);
        let out = e.mem.alloc(0x400);
        let len = e.mem.alloc(4);
        let inp = e.mem.alloc(0x400);
        e.mem.write(inp, input);
        let pos = e.mem.alloc(4);
        let diag = e.mem.alloc(4);
        let record = e.mem.alloc(4);
        let write = move |e: &mut Engine, a: &[u32]| {
            let n = e.mem.u32(len);
            for k in 0..a[2] {
                let byte = e.mem.u8(a[1] + k);
                e.mem.set_u8(out + n + k, byte);
            }
            e.mem.set_u32(len, n + a[2]);
            Ret::default()
        };
        e.register_double(SAVE_WRITE, write);
        e.register_double(SAVE_FORM_ID, write);
        e.register_double(SAVE_POSITION, move |e, _| {
            ret(if reading {
                inp + e.mem.u32(pos)
            } else {
                out + e.mem.u32(len)
            })
        });
        let read = move |e: &mut Engine, a: &[u32]| {
            let n = e.mem.u32(pos);
            for k in 0..a[2] {
                let byte = e.mem.u8(inp + n + k);
                e.mem.set_u8(a[1] + k, byte);
            }
            e.mem.set_u32(pos, n + a[2]);
            ret(0)
        };
        e.register_double(SAVE_READ, read);
        e.register_double(LOAD_FORM_ID, read);
        e.register_double(SAVE_SKIP, move |e, a| {
            let n = e.mem.u32(pos);
            e.mem.set_u32(pos, n + a[1]);
            Ret::default()
        });
        e.register_double(SAVE_USES_BLOCKS, move |_, _| ret(blocks as u32));
        e.register_double(SAVE_BYTE_80, move |_, _| ret(version));
        e.register_double(SETTING_BYTE_ADDRESS, move |_, _| ret(diag));
        e.register_double(SAVE_RECORD_WRITTEN, move |e, _| ret(e.mem.u32(record)));
        e.register_double(SAVE_RECORD_READ, move |e, _| ret(e.mem.u32(record)));
        e.register(ERROR_LOG, |_, _| Ret::default());
        e.register(SEQUENCE_SAVE_BASE_SIZE, |_, _| ret(0x20));
        SaveIo {
            out,
            len,
            pos,
            diag,
            record,
        }
    }

    /// What the save functions wrote so far.
    fn written(e: &Engine, io: &SaveIo) -> Vec<u8> {
        (0..e.mem.u32(io.len))
            .map(|i| e.mem.u8(io.out + i))
            .collect()
    }

    /// An actor for the save and load functions: it has a process (slot 0x100),
    /// answers slot 0x22c with `false`, has the form ID `0x1234` and the type
    /// name `0xabcdef`, and its sub-object (+0x88) answers slot 0x34 with
    /// `answers`. When it does, the animation's controller chain finds
    /// `sequence`, held by the controller object that is returned too; the
    /// light at +0x8c of the actor has its float at +0x10.
    fn saved_actor(
        e: &mut Engine,
        this: Ptr<Animation>,
        answers: bool,
        sequence: u32,
    ) -> (u32, u32) {
        e.register(0x00f3_0100, |_, _| ret(1));
        e.register(0x00f3_0130, |_, _| ret(0x00ab_cdef));
        e.register(0x00f3_022c, |_, _| ret(0));
        e.register(0x00f3_0034, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(0x00f3_008c, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(0x00f3_000c, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        let actor = object_with_vtable(
            e,
            0x200,
            &[
                (0x100, 0x00f3_0100),
                (0x130, 0x00f3_0130),
                (0x22c, 0x00f3_022c),
            ],
        );
        e.mem.set_u32(actor + 0xc, 0x1234);
        let sub_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(sub_vtable + 0x34, 0x00f3_0034);
        e.mem.set_u32(actor + 0x88, sub_vtable);
        e.mem.set_u32(actor + 0x98, answers as u32);
        let light = e.mem.alloc(0x40);
        e.mem.set_f32(light + 0x10, 0.75);
        e.mem.set_u32(actor + 0x8c, light);
        // The chain: manager -> palette -> object -> node -> child -> controller.
        let mut manager = e.mem.u32(this.addr() + 0xd8);
        if manager == 0 {
            manager = e.mem.alloc(0x80);
            e.mem.set_u32(this.addr() + 0xd8, manager);
        }
        let palette = object_with_vtable(e, 0x40, &[(0x8c, 0x00f3_008c)]);
        let object = object_with_vtable(e, 0x40, &[(0xc, 0x00f3_000c)]);
        let node = e.mem.alloc(0x40);
        let child = e.mem.alloc(0x40);
        let controller = e.mem.alloc(0x80);
        e.mem.set_u32(manager + 0x10, palette);
        e.mem.set_u32(palette + 0x10, object);
        e.mem.set_u32(object + 0x10, node);
        e.mem.set_u32(node + 0x20, child);
        e.mem.set_u32(child + 0x20, controller);
        e.mem.set_u32(controller + 0x30, sequence);
        e.register(MANAGER_PALETTE, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(NODE_ARRAY_ELEMENT, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(OBJECT_CONTROLLERS, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.register(DYNAMIC_CAST, |_, a| {
            ret(if a[0] == RTTI_SAVED_CONTROLLER {
                a[1]
            } else {
                0
            })
        });
        e.register(SEQUENCE_LOOKUP, |e, a| ret(e.mem.u32(a[0] + 0x30)));
        e.set_global(SAVED_SEQUENCE_KEY_GLOBAL, 0x1111u32);
        e.set_global(SAVED_SEQUENCE_LOOKUP_GLOBAL, 0x2222u32);
        (actor, controller)
    }

    /// Empties every group slot of an animation.
    fn empty_groups(e: &mut Engine, this: Ptr<Animation>) {
        for i in 0..8 {
            e.mem.set_u16(group_at(this, i), 0xff);
        }
    }

    #[test]
    fn fn_004997f0_adds_up_the_saved_parts() {
        let mut e = idle_engine();
        let (this, _manager, _map) = managed_animation(&mut e);
        let io = save_io(&mut e, false, 0x3f, false, &[]);
        e.register(SEQUENCE_SAVE_SIZE, |_, a| {
            ret(if a[0] == 0x5555 { 0x10 } else { 0x20 })
        });
        let (actor, _controller) = saved_actor(&mut e, this, false, 0);
        empty_groups(&mut e, this);
        e.mem.set_u16(group_at(this, 0), 5);
        e.mem.set_u32(current_sequence_at(this, 0), 0x5555);
        e.mem.set_u16(group_at(this, 2), 6);
        // 26 fixed bytes, 17 for each of the two used slots, the 16 bytes of
        // the sequence, 4 for the idle (none) and 1.
        let size = |e: &mut Engine| e.call(0x0049_97f0, &args![this, actor]).u16();
        assert_eq!(size(&mut e), 26 + 34 + 16 + 4 + 1);
        // From save version 0x40 on there is one more.
        e.register_double(SAVE_BYTE_80, |_, _| ret(0x40));
        assert_eq!(size(&mut e), 26 + 34 + 16 + 4 + 2);
        // The queued idle is the one sized, with its form and sequence.
        let idle = fake_idle(&mut e, 1, 0, 0, 0x6666);
        e.mem.set_u32(idle + 0x2c, 0x7000);
        put_idles(&mut e, this, [0, idle, 0, 0]);
        // 4 + 2, then the idle's own 13 + 0x20 + 1.
        assert_eq!(size(&mut e), 26 + 34 + 16 + 4 + 2 + 2 + 13 + 0x20 + 1);
        put_idles(&mut e, this, [0; 4]);
        // An actor whose sub-object answers adds the sequence found and 4.
        let (actor, _controller) = saved_actor(&mut e, this, true, 0x7777);
        assert_eq!(
            e.call(0x0049_97f0, &args![this, actor]).u16(),
            26 + 34 + 16 + 4 + 2 + 0x20 + 4
        );
        // The diagnostics setting logs the size (no record: the plain format).
        e.mem.set_u32(io.diag, 1);
        e.call_log = Some(vec![]);
        let total = e.call(0x0049_97f0, &args![this, actor]).u16();
        let log = peek_log(&e);
        // (The idle size function logs as well, with its own source line.)
        assert_eq!(
            arguments_of(&log, ERROR_LOG).last().cloned(),
            Some(vec![LOG_SAVE_SIZE, total as u32, 0x15b8, SOURCE_FILE])
        );
        // With a record: its form ID, the form type's name and its flags.
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record, 0x4321);
        e.mem.set_u32(record + 5, 0x99);
        e.mem.set_u32(io.record, record);
        let kind = object_with_vtable(&mut e, 0x200, &[(SLOT_FORM_TYPE_NAME, 0x00f3_0130)]);
        e.register(FORM_BY_ID, |e, _| ret(e.mem.u32(0x011c_6100)));
        e.mem.set_u32(0x011c_6100, kind);
        e.call_log = Some(vec![]);
        e.call(0x0049_97f0, &args![this, actor]);
        let log = peek_log(&e);
        assert_eq!(
            arguments_of(&log, ERROR_LOG).last().cloned(),
            Some(vec![
                LOG_SAVE_SIZE_FORM,
                total as u32,
                0x4321,
                0x00ab_cdef,
                0x99,
                0x15b8,
                SOURCE_FILE
            ])
        );
        // Saving in blocks adds 6, and the idle's size another 6.
        e.register(SAVE_USES_BLOCKS, |_, _| ret(1));
        e.mem.set_u32(io.diag, 0);
        assert_eq!(e.call(0x0049_97f0, &args![this, actor]).u16(), total + 12);
    }

    /// The little-endian bytes of the given parts, for the expected streams.
    fn bytes_of(parts: &[&[u8]]) -> Vec<u8> {
        parts.iter().flat_map(|p| p.iter().copied()).collect()
    }

    /// An animation with the fields `SaveGame` and `LoadGame` store: speeds,
    /// movement delta, looking and skip update, and one used slot (1) with
    /// the group 5.
    fn saved_animation(e: &mut Engine) -> (Ptr<Animation>, u32) {
        let (this, _manager, map) = managed_animation(e);
        empty_groups(e, this);
        let a = this.addr();
        e.mem.set_f32(a + 0x10c, 1.5);
        e.mem.set_f32(a + 0x110, 2.5);
        for (i, value) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(a + 0x10 + 4 * i as u32, *value);
        }
        e.mem.set_f32(a + 0x48, 0.25);
        e.mem.set_u8(a + 0xcc, 7);
        e.mem.set_u8(a + 0x120, 1);
        e.set(this, Animation::time, 3.5f32);
        e.mem.set_u16(group_at(this, 1), 5);
        e.mem.set_i32(action_at(this, 1), 0x11);
        e.mem.set_i32(loop_count_at(this, 1), 3);
        e.mem.set_u16(next_group_at(this, 1), 0x22);
        e.mem.set_i32(next_loops_at(this, 1), 4);
        (this, map)
    }

    /// The bytes `saved_animation`'s fixed fields and slot 1 take in a save,
    /// with `selector` as the slot's selector byte.
    fn saved_bytes(selector: u8) -> Vec<u8> {
        bytes_of(&[
            &1.5f32.to_le_bytes(),
            &2.5f32.to_le_bytes(),
            &1.0f32.to_le_bytes(),
            &2.0f32.to_le_bytes(),
            &3.0f32.to_le_bytes(),
            &0.25f32.to_le_bytes(),
            &[7, 0b10],
            &5u16.to_le_bytes(),
            &0x11u32.to_le_bytes(),
            &3u32.to_le_bytes(),
            &0x22u16.to_le_bytes(),
            &4u32.to_le_bytes(),
            &[selector],
        ])
    }

    #[test]
    fn save_game_writes_the_fields_and_the_used_slots() {
        let mut e = idle_engine();
        let (this, map) = saved_animation(&mut e);
        let io = save_io(&mut e, false, 0x3f, false, &[]);
        let (actor, _) = saved_actor(&mut e, this, false, 0);
        e.register(SEQUENCE_SAVE, |_, _| Ret::default());
        // The map entry of group 5 says the sequence has the selector 9.
        let entry = map_entry(&mut e, 0x5555, true);
        e.register(0x00f1_0018, |_, _| ret(9));
        let vtable = e.mem.u32(entry);
        e.mem.set_u32(vtable + 0x18, 0x00f1_0018);
        map_add(&mut e, map, 5, entry);
        e.mem.set_u32(current_sequence_at(this, 1), 0x5555);
        start_log(&mut e);
        e.call(0x0049_9b90, &args![this, actor]);
        let log = end_log(&mut e);
        // Fields, mask, slot 1, the idle's form ID (0: no idle) and
        // `cSkipNextBlend`.
        let mut expected = saved_bytes(9);
        expected.extend([0, 0, 0, 0, 1]);
        assert_eq!(written(&e, &io), expected);
        assert_eq!(
            arguments_of(&log, SEQUENCE_SAVE),
            [[0x5555, 3.5f32.to_bits()]]
        );
        // A null sequence is logged and saved with the selector 0xfe.
        e.mem.set_u32(current_sequence_at(this, 1), 0);
        e.mem.set_u32(io.len, 0);
        start_log(&mut e);
        e.call(0x0049_9b90, &args![this, actor]);
        let log = end_log(&mut e);
        let mut expected = saved_bytes(0xfe);
        expected.extend([0, 0, 0, 0, 1]);
        assert_eq!(written(&e, &io), expected);
        assert_eq!(
            arguments_of(&log, LOG),
            [[LOG_SAVE_NULL_SEQUENCE, 0x00ab_cdef, 0x1234, 1, 5, 0x11]]
        );
        assert!(arguments_of(&log, SEQUENCE_SAVE).is_empty());
    }

    #[test]
    fn save_game_saves_the_controller_sequence_from_version_0x40() {
        let mut e = idle_engine();
        let (this, _map) = saved_animation(&mut e);
        e.mem.set_u16(group_at(this, 1), 0xff);
        let io = save_io(&mut e, false, 0x40, false, &[]);
        e.register(SEQUENCE_SAVE, |_, _| Ret::default());
        e.register(FLOAT_AT_10, |e, a| e.mem.f32(a[0] + 0x10).into_ret());
        // An actor whose sub-object does not answer: a zero flag byte.
        let (actor, _) = saved_actor(&mut e, this, false, 0);
        e.call(0x0049_9b90, &args![this, actor]);
        let bytes = written(&e, &io);
        assert_eq!(bytes[bytes.len() - 2..], [1, 0]);
        // One that answers: the flag, the sequence and the light's float.
        let (actor, _) = saved_actor(&mut e, this, true, 0x8888);
        e.mem.set_u32(io.len, 0);
        start_log(&mut e);
        e.call(0x0049_9b90, &args![this, actor]);
        let log = end_log(&mut e);
        let bytes = written(&e, &io);
        let mut tail = vec![1u8, 1];
        tail.extend(0.75f32.to_le_bytes());
        assert_eq!(bytes[bytes.len() - 6..], tail[..]);
        assert_eq!(
            arguments_of(&log, SEQUENCE_SAVE),
            [[0x8888, 3.5f32.to_bits()]]
        );
        // Before version 0x40 the flag byte is not written.
        e.register_double(SAVE_BYTE_80, |_, _| ret(0x3f));
        e.mem.set_u32(io.len, 0);
        e.call(0x0049_9b90, &args![this, actor]);
        let bytes = written(&e, &io);
        let mut tail = vec![1u8];
        tail.extend(0.75f32.to_le_bytes());
        assert_eq!(bytes[bytes.len() - 5..], tail[..]);
    }

    #[test]
    fn save_game_saves_in_a_block_and_logs_its_size() {
        let mut e = idle_engine();
        let (this, _map) = saved_animation(&mut e);
        let io = save_io(&mut e, true, 0x3f, false, &[]);
        let (actor, _) = saved_actor(&mut e, this, false, 0);
        e.register(SEQUENCE_SAVE, |_, _| Ret::default());
        e.call(0x0049_9b90, &args![this, actor]);
        let bytes = written(&e, &io);
        // The block tag, then a length that counts from itself to the end.
        assert_eq!(bytes[..4], *b"KOLB");
        let length = u16::from_le_bytes([bytes[4], bytes[5]]) as usize;
        assert_eq!(length, bytes.len() - 4);
        // With the diagnostics setting on, the size written is logged (no
        // record: the plain format, source line 0x1628).
        e.mem.set_u32(io.diag, 1);
        e.mem.set_u32(io.len, 0);
        start_log(&mut e);
        e.call(0x0049_9b90, &args![this, actor]);
        let log = end_log(&mut e);
        let size = e.mem.u32(io.len);
        let ours: Vec<_> = arguments_of(&log, ERROR_LOG)
            .into_iter()
            .filter(|words| words[2] == 0x1628)
            .collect();
        assert_eq!(ours, [[LOG_SAVE_GAME, size, 0x1628, SOURCE_FILE]]);
    }

    #[test]
    fn fn_0049a1c0_reads_the_fields_and_empties_slots_that_did_not_come_back() {
        let mut e = idle_engine();
        let (this, _) = saved_animation(&mut e);
        // Everything is cleared before: the loader must write it all.
        for offset in [0x10c, 0x110, 0x10, 0x14, 0x18, 0x48, 0xcc] {
            e.mem.set_u32(this.addr() + offset, 0);
        }
        e.mem.set_u16(group_at(this, 1), 0xff);
        e.mem.set_u32(current_sequence_at(this, 1), 0x9999);
        let mut stream = saved_bytes(0xfe);
        stream.extend([0u8; 8]);
        stream.push(1);
        let io = save_io(&mut e, false, 0x3f, true, &stream);
        let (actor, _) = saved_actor(&mut e, this, false, 0);
        e.register(SEQUENCE_LOAD, |_, _| Ret::default());
        e.register(BIP_NODE_UPDATE_BIP_ONLY, |_, _| Ret::default());
        e.mem.set_u32(this.addr() + 0x120, 0);
        e.call(0x0049_a1c0, &args![this, actor]);
        let a = this.addr();
        assert_eq!(e.mem.f32(a + 0x10c), 1.5);
        assert_eq!(e.mem.f32(a + 0x110), 2.5);
        assert_eq!(e.mem.f32(a + 0x10), 1.0);
        assert_eq!(e.mem.f32(a + 0x18), 3.0);
        assert_eq!(e.mem.f32(a + 0x48), 0.25);
        assert_eq!(e.mem.u8(a + 0xcc), 7);
        assert_eq!(e.mem.u16(group_at(this, 1)), 5);
        assert_eq!(e.mem.i32(action_at(this, 1)), 0x11);
        assert_eq!(e.mem.i32(loop_count_at(this, 1)), 3);
        assert_eq!(e.mem.u16(next_group_at(this, 1)), 0x22);
        assert_eq!(e.mem.i32(next_loops_at(this, 1)), 4);
        // The slot comes back empty (selector -2), and `cSkipNextBlend` is read.
        assert_eq!(e.mem.u32(current_sequence_at(this, 1)), 0);
        assert_eq!(e.mem.u8(a + 0x120), 1);
        assert_eq!(e.mem.u32(io.pos) as usize, stream.len());
    }

    #[test]
    fn fn_0049a1c0_forces_the_section_and_skips_the_bytes_of_a_missing_sequence() {
        let mut e = idle_engine();
        let (this, _) = saved_animation(&mut e);
        e.mem.set_u16(group_at(this, 1), 0xff);
        // Selector 0xff: the group is forced; the map does not know it, so no
        // sequence comes back: the bytes the sequence would take (0x20) are
        // skipped and the slot is cleared.
        let mut stream = saved_bytes(0xff);
        stream.extend([0xeeu8; 0x20]);
        stream.extend([0u8; 8]);
        stream.push(1);
        let io = save_io(&mut e, false, 0x3f, true, &stream);
        let (actor, _) = saved_actor(&mut e, this, false, 0);
        e.register(BIP_NODE_UPDATE_BIP_ONLY, |_, _| Ret::default());
        e.register(SEQUENCE_LOAD, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0049_a1c0, &args![this, actor]);
        let log = end_log(&mut e);
        assert_eq!(e.mem.u32(io.pos) as usize, stream.len());
        assert_eq!(e.mem.u16(group_at(this, 1)), 0xff);
        assert_eq!(e.mem.i32(action_at(this, 1)), -1);
        assert_eq!(e.mem.u8(this.addr() + 0x120), 1);
        assert!(arguments_of(&log, SEQUENCE_LOAD).is_empty());
    }

    #[test]
    fn fn_0049a1c0_loads_the_sequence_of_a_group_it_started() {
        // `StartGroup_ov2`'s fixture: group 1 plays in slot 1 and the sequence
        // is not playing yet, so forcing the group puts it into the slot.
        let mut f = start_fixture();
        let (this, sequence) = (f.this, f.sequence);
        let e = &mut f.e;
        let map = map_for(e, this);
        let entry = map_entry(e, sequence, true);
        map_add(e, map, 1, entry);
        e.register(LOG, |_, _| Ret::default());
        let stream = bytes_of(&[
            &[0u8; 8],
            &[0u8; 12],
            &[0u8; 4],
            &[0, 0b10],
            &1u16.to_le_bytes(),
            &5u32.to_le_bytes(),
            &0u32.to_le_bytes(),
            &0xffu16.to_le_bytes(),
            &0u32.to_le_bytes(),
            &[0],
            &[0u8; 8],
            &[1],
        ]);
        let io = save_io(e, false, 0x3f, true, &stream);
        let (actor, _) = saved_actor(e, this, false, 0);
        e.register(SEQUENCE_LOAD, |_, _| Ret::default());
        e.register(BIP_NODE_UPDATE_BIP_ONLY, |_, _| Ret::default());
        start_log(e);
        e.call(0x0049_a1c0, &args![this, actor]);
        let log = end_log(e);
        // The sequence was started in the slot and its saved state is loaded.
        assert_eq!(e.mem.u32(this.addr() + 0xe0 + 4), sequence);
        // (`ForceSection` stores the saved action after the start.)
        assert_eq!(e.mem.i32(this.addr() + 0x5c + 4), 5);
        assert_eq!(arguments_of(&log, SEQUENCE_LOAD), [[sequence, 0]]);
        assert_eq!(e.mem.u32(io.pos) as usize, stream.len());
    }

    /// A save stream with no group, two empty idles and `cSkipNextBlend`, for
    /// the controller sequence tests; `tail` follows.
    fn bare_stream(tail: &[u8]) -> Vec<u8> {
        bytes_of(&[
            &1.5f32.to_le_bytes(),
            &2.5f32.to_le_bytes(),
            &[0u8; 12],
            &0.25f32.to_le_bytes(),
            &[7, 0],
            &[0u8; 8],
            &[1],
            tail,
        ])
    }

    #[test]
    fn fn_0049a1c0_restores_the_controller_sequence_and_the_light() {
        let mut e = idle_engine();
        let (this, _) = saved_animation(&mut e);
        e.mem.set_u16(group_at(this, 1), 0xff);
        let sequence = e.mem.alloc(0x80);
        e.mem.set_f32(sequence + 0x30, 4.0);
        // Before version 0x40 the controller sequence is always there: its
        // light value follows.
        e.set_global(DOUBLE_THREE_QUARTERS, 0.75f64);
        let stream = bare_stream(&0.5f32.to_le_bytes());
        let io = save_io(&mut e, false, 0x3f, true, &stream);
        let (actor, controller) = saved_actor(&mut e, this, true, sequence);
        e.register(MANAGER_ACTIVATE, |_, _| Ret::default());
        e.register(MANAGER_SET_FLAG, |_, _| Ret::default());
        for address in [
            SEQUENCE_LOAD,
            LIGHT_FADE,
            SET_FLOAT_AT_10,
            BIP_NODE_UPDATE_BIP_ONLY,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        start_log(&mut e);
        e.call(0x0049_a1c0, &args![this, actor]);
        let log = end_log(&mut e);
        let light = e.mem.u32(actor + 0x8c);
        assert_eq!(
            arguments_of(&log, MANAGER_ACTIVATE),
            [[controller, sequence, 0, 0, 1.0f32.to_bits(), 0, 0]]
        );
        assert_eq!(arguments_of(&log, MANAGER_SET_FLAG), [[controller, 1]]);
        assert_eq!(
            arguments_of(&log, SEQUENCE_LOAD),
            [[sequence, 3.5f32.to_bits()]]
        );
        // The light fades over three quarters of the sequence's length, and its
        // float is the value read from the save.
        assert_eq!(
            arguments_of(&log, LIGHT_FADE),
            [[light, 1, 3.0f32.to_bits()]]
        );
        assert_eq!(
            arguments_of(&log, SET_FLOAT_AT_10),
            [[light, 0.5f32.to_bits()]]
        );
        assert_eq!(e.mem.u32(io.pos) as usize, stream.len());
    }

    #[test]
    fn fn_0049a1c0_skips_the_controller_sequence_it_cannot_restore() {
        // From version 0x40 a zero flag byte means there is none.
        let mut e = idle_engine();
        let (this, _) = saved_animation(&mut e);
        e.mem.set_u16(group_at(this, 1), 0xff);
        let stream = bare_stream(&[0]);
        let io = save_io(&mut e, false, 0x40, true, &stream);
        let (actor, _) = saved_actor(&mut e, this, false, 0);
        e.register(BIP_NODE_UPDATE_BIP_ONLY, |_, _| Ret::default());
        e.call(0x0049_a1c0, &args![this, actor]);
        assert_eq!(e.mem.u32(io.pos) as usize, stream.len());
        // A set flag with no sequence found skips the bytes it would have
        // taken: the sequence's base size (0x20) and 4.
        let mut stream = bare_stream(&[1]);
        stream.extend([0u8; 0x24]);
        let io = save_io(&mut e, false, 0x40, true, &stream);
        e.call(0x0049_a1c0, &args![this, actor]);
        assert_eq!(e.mem.u32(io.pos) as usize, stream.len());
    }

    #[test]
    fn fn_0049a1c0_checks_the_block_it_reads() {
        let build = |tag: &[u8; 4], length: u16| {
            // The block, version 0x4c: the fields (25 bytes), the mask, one idle
            // block (tag, its length 6 and a zero form ID), `cSkipNextBlend`
            // and the flag byte (0).
            let mut stream = tag.to_vec();
            stream.extend(length.to_le_bytes());
            stream.extend([0u8; 25]);
            stream.push(0);
            stream.extend(*b"KOLB");
            stream.extend(6u16.to_le_bytes());
            stream.extend([0u8; 4]);
            stream.extend([1, 0]);
            stream
        };
        let run = |tag: &[u8; 4], length: u16| {
            let mut e = idle_engine();
            let (this, _) = saved_animation(&mut e);
            e.mem.set_u16(group_at(this, 1), 0xff);
            let stream = build(tag, length);
            let _io = save_io(&mut e, true, 0x4c, true, &stream);
            let (actor, _) = saved_actor(&mut e, this, false, 0);
            e.register(BIP_NODE_UPDATE_BIP_ONLY, |_, _| Ret::default());
            start_log(&mut e);
            e.call(0x0049_a1c0, &args![this, actor]);
            end_log(&mut e)
        };
        // The block is read exactly: no message.
        assert!(arguments_of(&run(b"KOLB", 40), LOG).is_empty());
        // One byte more than the block says (overrun) or less (underrun).
        assert_eq!(
            arguments_of(&run(b"KOLB", 39), LOG),
            [[LOG_LOAD_LONG, 1, SOURCE_FILE, 0x16ac, 0x4c]]
        );
        assert_eq!(
            arguments_of(&run(b"KOLB", 41), LOG),
            [[LOG_LOAD_SHORT, 1, SOURCE_FILE, 0x16ac, 0x4c]]
        );
        // A wrong tag is logged.
        let logged = arguments_of(&run(b"XXXX", 40), LOG);
        assert_eq!(logged[0], [LOG_LOAD_NO_BLOCK, SOURCE_FILE, 0x1630, 0x4c]);
    }

    #[test]
    fn fn_0049a920_resets_the_groups_idles_and_controllers() {
        let mut e = idle_engine();
        let (this, manager, _map) = managed_animation(&mut e);
        manager_with_sequences(&mut e, this, &[]);
        kf_model_list(&mut e, this, &[]);
        // The animation root, a node that is not a node (slot 0xc gives 0).
        e.register(0x00f2_000c, |_, _| ret(0));
        let root = object_with_vtable(&mut e, 0x100, &[(0xc, 0x00f2_000c)]);
        e.mem.set_u32(this.addr() + 8, root);
        e.register(OBJECT_CONTROLLERS, |_, _| ret(0));
        e.register(GET_CONTROLLER, |_, _| ret(0));
        e.register(MANAGER_PALETTE, |_, _| ret(0));
        let (idle, sequence) = playing_idle(&mut e, this, 2, 0);
        let queued = fake_idle(&mut e, 1, 0, 3, 0);
        put_idles(&mut e, this, [idle, queued, 0, 0]);
        let stale = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0xfc, stale);
        start_log(&mut e);
        e.call(0x0049_a920, &args![this, 0u32]);
        let log = end_log(&mut e);
        // Both idles are freed and emptied, the last slot and the ladder's
        // slots are cleared.
        assert_eq!(idle_fields(&e, this), [0; 4]);
        assert_eq!(e.mem.u32(this.addr() + 0xfc), 0);
        assert_eq!(e.mem.u32(current_sequence_at(this, 2)), 0);
        assert_eq!(
            arguments_of(&log, MANAGER_REMOVE_SEQUENCE),
            [[manager, sequence]]
        );
        // Then the controllers of the animation root are cleared.
        assert_eq!(arguments_of(&log, OBJECT_CONTROLLERS), [[root]]);
    }

    #[test]
    fn fn_0049aa20_is_the_size_of_what_save_animation_writes() {
        let mut e = idle_engine();
        let (this, _manager, _map) = managed_animation(&mut e);
        let _io = save_io(&mut e, false, 0x3f, false, &[]);
        e.register(SEQUENCE_SAVE_SIZE, |_, _| ret(0x20));
        let (actor, _) = saved_actor(&mut e, this, false, 0);
        empty_groups(&mut e, this);
        let size = e.call(0x0049_97f0, &args![this, actor]).u16();
        assert_eq!(e.call(0x0049_aa20, &args![actor, this]).u16(), size + 2);
        // Without an animation, or for an actor whose slot 0x22c says yes,
        // there are only the two size bytes.
        assert_eq!(e.call(0x0049_aa20, &args![actor, 0u32]).u16(), 2);
        e.register(0x00f3_022c, |_, _| ret(1));
        assert_eq!(e.call(0x0049_aa20, &args![actor, this]).u16(), 2);
    }

    #[test]
    fn save_animation_writes_the_size_and_the_animation() {
        let mut e = idle_engine();
        let (this, _manager, _map) = managed_animation(&mut e);
        let io = save_io(&mut e, false, 0x3f, false, &[]);
        e.register(SEQUENCE_SAVE_SIZE, |_, _| ret(0x20));
        e.register(SEQUENCE_SAVE, |_, _| Ret::default());
        let (actor, _) = saved_actor(&mut e, this, false, 0);
        empty_groups(&mut e, this);
        let size = e.call(0x0049_97f0, &args![this, actor]).u16();
        e.call(0x0049_aa80, &args![actor, this]);
        let bytes = written(&e, &io);
        assert_eq!(bytes[..2], size.to_le_bytes());
        // The animation's fields follow (the save game function ran).
        assert!(bytes.len() > 2);
        assert_eq!(bytes[2..6], 0u32.to_le_bytes());
        // No animation: a zero size and nothing else.
        e.mem.set_u32(io.len, 0);
        e.call(0x0049_aa80, &args![actor, 0u32]);
        assert_eq!(written(&e, &io), [0, 0]);
        // An actor whose slot 0x22c says yes: a zero size as well.
        e.register(0x00f3_022c, |_, _| ret(1));
        e.mem.set_u32(io.len, 0);
        e.call(0x0049_aa80, &args![actor, this]);
        assert_eq!(written(&e, &io), [0, 0]);
    }

    #[test]
    fn fn_0049ab00_has_the_save_game_consume_the_length_it_reads() {
        let mut e = idle_engine();
        let io = save_io(&mut e, false, 0x3f, true, &[3, 0, 0, 0]);
        e.register(SAVE_GAME_CONSUME, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0049_ab00, &args![0x5000u32, 0x1c0u32]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, SAVE_GAME_CONSUME), [[0x7000, 0x5000, 3]]);
        assert_eq!(e.mem.u32(io.pos), 2);
        // A length of 0 consumes nothing.
        e.mem.set_u32(io.pos, 2);
        start_log(&mut e);
        e.call(0x0049_ab00, &args![0x5000u32, 0x1c0u32]);
        assert!(arguments_of(&end_log(&mut e), SAVE_GAME_CONSUME).is_empty());
    }

    /// The doubles of a `BGSSaveGameBuffer` and a `BGSLoadGameBuffer`: the
    /// buffer object (virtual slots 8 and 0xc give the owner stored at +0x10;
    /// the word at +0xc counts the bytes written), and the stream the writes go
    /// to (`out`, `len`) or the reads come from (`input`, `pos`). Returns the
    /// buffer, `out`, `len` and `pos`.
    fn buffer_io(e: &mut Engine, owner: u32, input: &[u8]) -> (u32, u32, u32, u32) {
        e.register(0x00f4_0008, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(0x00f4_000c, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        let buffer = object_with_vtable(e, 0x60, &[(8, 0x00f4_0008), (0xc, 0x00f4_000c)]);
        e.mem.set_u32(buffer + 0x10, owner);
        let out = e.mem.alloc(0x400);
        let len = e.mem.alloc(4);
        let inp = e.mem.alloc(0x400);
        e.mem.write(inp, input);
        let pos = e.mem.alloc(4);
        e.register_double(SAVE_BUFFER_FIELD, move |e, a| {
            let n = e.mem.u32(len);
            for k in 0..a[2] {
                let byte = e.mem.u8(a[1] + k);
                e.mem.set_u8(out + n + k, byte);
            }
            e.mem.set_u32(len, n + a[2]);
            let counted = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[0] + 0xc, counted + a[2]);
            Ret::default()
        });
        e.register_double(SAVE_BUFFER_FORM_ID, move |e, a| {
            let n = e.mem.u32(len);
            e.mem.set_u32(out + n, a[1]);
            e.mem.set_u32(len, n + 4);
            let counted = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[0] + 0xc, counted + 4);
            Ret::default()
        });
        e.register(SAVE_BUFFER_START_VARIABLE, |_, _| ret(0x7700));
        e.register_double(SAVE_BUFFER_END_VARIABLE, move |e, a| {
            let n = e.mem.u32(len);
            e.mem.set_u32(out + n, a[1]);
            e.mem.set_u32(len, n + 4);
            Ret::default()
        });
        e.register_double(LOAD_BUFFER_FIELD, move |e, a| {
            let n = e.mem.u32(pos);
            for k in 0..a[2] {
                let byte = e.mem.u8(inp + n + k);
                e.mem.set_u8(a[1] + k, byte);
            }
            e.mem.set_u32(pos, n + a[2]);
            Ret::default()
        });
        (buffer, out, len, pos)
    }

    /// A process with the given virtual slots (all doubles that return what the
    /// object holds at +0x10).
    fn process_with_slots(e: &mut Engine, slots: &[u32]) -> u32 {
        let mut entries = vec![];
        for slot in slots {
            entries.push((*slot, 0x00f5_0000 + *slot));
            e.register(0x00f5_0000 + *slot, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        }
        object_with_vtable(e, 0x400, &entries)
    }

    #[test]
    fn fn_0049ab40_saves_the_animation_into_a_buffer() {
        let mut e = idle_engine();
        let (this, map) = saved_animation(&mut e);
        let a = this.addr();
        e.mem.set_u32(a + 0xd4, 0x0bad_f00d);
        e.mem.set_u16(a + 0x122, 0x77);
        // The actor and its process: the 16-bit value (slot 0x3e4) and the held
        // sequence (slot 0x3e8).
        let process = process_with_slots(&mut e, &[0x3e4, 0x3e8]);
        let actor = e.mem.alloc(0x100);
        e.mem.set_u32(actor + 0x68, process);
        let (buffer, out, len, _pos) = buffer_io(&mut e, actor, &[]);
        // The held sequence is the one of slot 1; slot 3 plays the current
        // idle's sequence, which is left out because an idle is queued.
        let (sequence, shared) = (0x5555, 0x6666);
        e.mem.set_u32(process + 0x10, sequence);
        e.mem.set_u32(current_sequence_at(this, 1), sequence);
        e.mem.set_u16(group_at(this, 3), 8);
        e.mem.set_u32(current_sequence_at(this, 3), shared);
        let entry = map_entry(&mut e, sequence, true);
        e.register(0x00f1_0018, |_, _| ret(9));
        let vtable = e.mem.u32(entry);
        e.mem.set_u32(vtable + 0x18, 0x00f1_0018);
        map_add(&mut e, map, 5, entry);
        e.register(SEQUENCE_SAVE_TO_BUFFER, |_, _| Ret::default());
        // The replay delay list: one item {form, delay}.
        let item = e.mem.alloc(8);
        e.mem.set_u32(item, 0xaa00_0001);
        e.mem.set_f32(item + 4, 6.0);
        e.mem.set_u32(a + 0x134, item);
        e.mem.set_u32(a + 0x138, 0);
        e.register(RECORD_ADDRESS, |_, a| ret(a[0]));
        // The queued idle has the form 0x7007 and no sequence; the current idle
        // plays the shared sequence.
        let queued = fake_idle(&mut e, 1, 2, 7, 0);
        e.mem.set_u32(queued + 0x2c, 0x7007);
        let current = fake_idle(&mut e, 1, 0, 3, shared);
        put_idles(&mut e, this, [current, queued, 0, 0]);
        start_log(&mut e);
        e.call(0x0049_ab40, &args![this, buffer]);
        let log = end_log(&mut e);
        let bytes: Vec<u8> = (0..e.mem.u32(len)).map(|i| e.mem.u8(out + i)).collect();
        let expected = bytes_of(&[
            &1.5f32.to_le_bytes(),
            &2.5f32.to_le_bytes(),
            &1.0f32.to_le_bytes(),
            &2.0f32.to_le_bytes(),
            &3.0f32.to_le_bytes(),
            &0.25f32.to_le_bytes(),
            &[7],
            &0x0bad_f00du32.to_le_bytes(),
            &0x77u16.to_le_bytes(),
            // The replay delay list: the item, then the count.
            &0xaa00_0001u32.to_le_bytes(),
            &6.0f32.to_le_bytes(),
            &1u32.to_le_bytes(),
            // The queued idle's form, then its state (as a sized value).
            &0x7007u32.to_le_bytes(),
            &1u32.to_le_bytes(),
            &2u32.to_le_bytes(),
            &7u32.to_le_bytes(),
            &12u32.to_le_bytes(),
            // The mask (slot 1 only), slot 1, `cSkipNextBlend`, the process's
            // value, the held sequence's slot and the idle's (none).
            &[0b10],
            &5u16.to_le_bytes(),
            &0x11u32.to_le_bytes(),
            &0x22u16.to_le_bytes(),
            &3u32.to_le_bytes(),
            &4u32.to_le_bytes(),
            &[9],
            &[1],
            &0x5555u16.to_le_bytes(),
            &[1],
            &[0xff],
        ]);
        // (The process's 16-bit value is what the double returns at +0x10, the
        // held sequence's low word.)
        assert_eq!(bytes, expected);
        assert_eq!(
            arguments_of(&log, SAVE_BUFFER_END_VARIABLE),
            [[buffer, 1, 0x7700], [buffer, 12, 0x7700]]
        );
        assert_eq!(
            arguments_of(&log, SEQUENCE_SAVE_TO_BUFFER),
            [[sequence, buffer, 3.5f32.to_bits()]]
        );
    }

    /// The doubles `fn_0049b050` needs besides the stream: an actor (with a
    /// process) for the load buffer, the replay list and the idle form lookups.
    /// `variables` are the values `LoadVariableSizedValue` returns in turn,
    /// `found` and `id` what `LoadFormID_ov2` reports, and `cast` what the
    /// `TESIdleForm` cast gives. Returns the buffer, the actor, the process
    /// and the stream position cell.
    fn load_scene(
        e: &mut Engine,
        input: &[u8],
        variables: Vec<u32>,
        found: u32,
        id: u32,
        cast: u32,
    ) -> (u32, u32, u32, u32) {
        e.register(0x00f6_022c, |_, _| ret(0));
        e.register(0x00f6_02e8, |_, _| ret(0));
        e.register(0x00f6_01e8, |_, _| ret(0xb1b1));
        let actor = object_with_vtable(
            e,
            0x200,
            &[
                (0x22c, 0x00f6_022c),
                (0x2e8, 0x00f6_02e8),
                (0x1e8, 0x00f6_01e8),
            ],
        );
        let process = process_with_slots(e, &[0x4dc, 0x454, 0x1cc, 0x1d8, 0x61c, 0x3ec]);
        e.mem.set_u32(process + 0x10, 0x5a);
        e.mem.set_u32(actor + 0x68, process);
        e.register(ACTOR_PROCESS, |e, a| ret(e.mem.u32(a[0] + 0x68)));
        let (buffer, _out, _len, pos) = buffer_io(e, actor, input);
        let mut values = variables.into_iter();
        e.register_double(LOAD_BUFFER_VARIABLE, move |_, _| {
            ret(values.next().unwrap_or(0))
        });
        e.register(LOAD_BUFFER_FORM_ID, |_, _| ret(0xf1));
        e.register_double(LOAD_BUFFER_FORM_ID_FOUND, move |e, a| {
            e.mem.set_u32(a[1], id);
            ret(found)
        });
        e.register(FORM_BY_ID, |_, a| ret(0x00ab_0000 + a[0]));
        e.register_double(RT_DYNAMIC_CAST, move |_, _| ret(cast));
        e.register(REPLAY_LIST_PUSH, |e, a| {
            let item = e.mem.u32(a[1]);
            let form = e.mem.u32(item);
            let delay = e.mem.u32(item + 4);
            e.mem.set_u32(0x011c_5100, form);
            e.mem.set_u32(0x011c_5104, delay);
            e.mem.set_u32(0x011c_5108, a[0]);
            Ret::default()
        });
        for address in [
            BIP_NODE_UPDATE_BIP_ONLY,
            BIP_UPDATE_ALL_BUT_BIP,
            SEQUENCE_LOAD_FROM_BUFFER,
            SEQUENCE_DESTRUCT,
            SET_FLOAT_AT_48,
            SEQUENCE_BASE_CONSTRUCT,
            MODEL_LOADER_DROP_IDLE,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(FLOAT_AT_48, |e, a| e.mem.f32(a[0] + 0x48).into_ret());
        e.register(FIND_NEXT_COLLISION_OBJECT, |_, _| ret(0));
        e.register(MENU_MODE_TYPE, |_, _| ret(0));
        e.register(WORD_AT_4, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(KF_MODEL_RELEASE, |_, _| Ret::default());
        (buffer, actor, process, pos)
    }

    /// The stream of `fn_0049b050` up to the idle: the 31 bytes of fields and
    /// one replay delay value.
    fn load_fields(replay_delay: u32) -> Vec<u8> {
        let mut stream = vec![0u8; 31];
        stream.extend(replay_delay.to_le_bytes());
        stream
    }

    #[test]
    fn fn_0049b050_does_nothing_without_an_animation_root() {
        let mut e = idle_engine();
        let this: Ptr<Animation> = e.new_object();
        let (buffer, _actor, _process, _pos) = load_scene(&mut e, &[], vec![], 0, 0, 0);
        start_log(&mut e);
        e.call(0x0049_b050, &args![this, buffer]);
        assert!(arguments_of(&end_log(&mut e), ACTOR_PROCESS).is_empty());
    }

    #[test]
    fn fn_0049b050_restores_the_groups_the_delays_and_the_held_sequence() {
        let mut f = start_fixture();
        let (this, sequence) = (f.this, f.sequence);
        let e = &mut f.e;
        let a = this.addr();
        let map = map_for(e, this);
        let entry = map_entry(e, sequence, true);
        map_add(e, map, 1, entry);
        e.register(KF_MODEL_ANIM_GROUP, |e, k| ret(e.mem.u32(k[0] + 8)));
        e.register(MANAGER_REMOVE_SEQUENCE, |_, _| Ret::default());
        // Two queued models (without a group: nothing to add), and a sequence in
        // slot 0 that gets its end time added to its float at +0x48.
        let (first, second) = (e.mem.alloc(0x20), e.mem.alloc(0x20));
        kf_model_list(e, this, &[first, second]);
        let playing = e.mem.alloc(0x80);
        e.mem.set_f32(playing + 0x48, 1.5);
        e.mem.set_f32(playing + 0x30, 4.0);
        e.mem.set_u32(current_sequence_at(this, 0), playing);
        e.set(this, Animation::time, 2.0f32);
        // The stream: fields, one replay delay (value 9), no idle, the mask (slots 1
        // and 2), slot 1 (group 1, started from the map), slot 2 (group 7, unknown:
        // a throw-away sequence loads its bytes), `cSkipNextBlend`, the phase -2,
        // the held slot 1 and no idle sequence.
        let mut stream = load_fields(9);
        stream.push(0b110);
        stream.extend(bytes_of(&[
            &1u16.to_le_bytes(),
            &5u32.to_le_bytes(),
            &0xffu16.to_le_bytes(),
            &0u32.to_le_bytes(),
            &0u32.to_le_bytes(),
            &[0],
            &7u16.to_le_bytes(),
            &6u32.to_le_bytes(),
            &0xffu16.to_le_bytes(),
            &0u32.to_le_bytes(),
            &0u32.to_le_bytes(),
            &[0xff],
            &[1],
            &0xfffeu16.to_le_bytes(),
            &[1],
            &[0xff],
        ]));
        let (buffer, actor, process, _pos) = load_scene(e, &stream, vec![1, 0], 0, 0, 0x00cd_0001);
        // (The first value 1 is the number of delays; the stream has one.)
        // The idle is absent: no form ID (0).
        let _ = actor;
        e.mem.set_u32(a + 0x120, 0);
        start_log(e);
        e.call(0x0049_b050, &args![this, buffer]);
        let log = end_log(e);
        // The queued models are released.
        assert_eq!(arguments_of(&log, KF_MODEL_RELEASE), [[first], [second]]);
        // The process takes the animation, hands out the biped and refreshes.
        let player: u32 = e.global(PLAYER_SINGLETON);
        let _ = player;
        assert_eq!(arguments_of(&log, 0x00f5_04dc), [[process, actor, a]]);
        assert_eq!(
            arguments_of(&log, 0x00f5_01cc),
            [[process, 0x5a, 0xb1b1, a, actor]]
        );
        assert_eq!(arguments_of(&log, 0x00f5_01d8), [[process, actor]]);
        assert_eq!(arguments_of(&log, 0x00f5_061c), [[process]]);
        // The held sequence is the sequence of slot 1 with the phase -2.
        let held = e.mem.u32(current_sequence_at(this, 1));
        assert_eq!(held, sequence);
        assert_eq!(
            arguments_of(&log, 0x00f5_03ec),
            [[process, 0xffff_ffff, 0], [process, 0xffff_fffe, sequence]]
        );
        // The delay is queued with the idle form the cast gave.
        assert_eq!(e.mem.u32(0x011c_5100), 0x00cd_0001);
        assert_eq!(e.mem.u32(0x011c_5108), a + 0x134);
        // The slots: groups and actions are read, slot 1 loads its sequence from
        // the buffer, slot 2's bytes go into a throw-away sequence.
        assert_eq!(e.mem.u16(group_at(this, 1)), 1);
        // (Slot 2 came back without a sequence: it is cleared again.)
        assert_eq!(e.mem.u16(group_at(this, 2)), 0xff);
        assert_eq!(e.mem.i32(action_at(this, 1)), 5);
        assert_eq!(e.mem.i32(action_at(this, 2)), -1);
        let loads = arguments_of(&log, SEQUENCE_LOAD_FROM_BUFFER);
        assert_eq!(loads.len(), 2);
        assert_eq!(loads[0], [sequence, buffer, 2.0f32.to_bits()]);
        let temporary = loads[1][0];
        assert_eq!(arguments_of(&log, SEQUENCE_BASE_CONSTRUCT), [[temporary]]);
        assert_eq!(arguments_of(&log, SEQUENCE_DESTRUCT), [[temporary]]);
        // The sequence of slot 0 had its end time added to the float at +0x48.
        assert_eq!(
            arguments_of(&log, SET_FLOAT_AT_48),
            [[playing, 5.5f32.to_bits()]]
        );
        // The bip nodes of the root are updated before and after.
        let root = e.mem.u32(a + 8);
        assert_eq!(
            arguments_of(&log, BIP_UPDATE_ALL_BUT_BIP),
            [[root, 2.0f32.to_bits()], [root, 2.0f32.to_bits()]]
        );
    }

    #[test]
    fn fn_0049b050_skips_an_idle_it_cannot_use() {
        // The stream: fields, no delays, the idle record, the mask (0) and the
        // three trailing values.
        let tail = [0u8, 1, 0xff, 0xff, 0xff, 0xff];
        let mut stream = load_fields(0);
        stream.truncate(31);
        stream.extend(tail);
        let run = |found: u32, id: u32, cast: u32| {
            let mut f = start_fixture();
            let this = f.this;
            let e = &mut f.e;
            e.register(KF_MODEL_ANIM_GROUP, |e, k| ret(e.mem.u32(k[0] + 8)));
            kf_model_list(e, this, &[]);
            let (buffer, _actor, _process, _pos) =
                load_scene(e, &stream, vec![0, 7], found, id, cast);
            e.call(0x0049_b050, &args![this, buffer]);
            (e.mem.u32(buffer + 0xc), idle_fields(e, this))
        };
        // The form ID is missing from the save: the 7 bytes are skipped.
        assert_eq!(run(1, 0xf1, 0).0, 7);
        // The ID is there but is not an idle form: skipped as well.
        assert_eq!(run(0, 0xf1, 0).0, 7);
        // No ID at all: nothing is read or skipped.
        let (skipped, idles) = run(0, 0, 0x00cd_0001);
        assert_eq!(skipped, 0);
        assert_eq!(idles, [0; 4]);
    }

    #[test]
    fn fn_0049b050_builds_the_idle_it_finds_in_the_save() {
        let mut b = idle_build(0, false);
        let idle_form = b.idle_form;
        let e = &mut b.e;
        let (this, _manager, _map) = managed_animation(e);
        e.register(WORD_AT_C, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(WORD_AT_8, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.map(0x011e_0000, 0x1000);
        e.set_global(PLAYER_SINGLETON, 0x3000_0000u32);
        e.register(SEQUENCE_STATE, |e, a| ret(e.mem.u32(a[0] + 0x44)));
        e.register(MODEL_LOADER_LOAD_KF, |_, _| ret(0));
        kf_model_list(e, this, &[]);
        let root = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + 8, root);
        // The stream: fields, no delays, the idle's state (3), play type (2) and
        // section (5) read by `fn_00497c10`, the empty mask and the trailing values.
        let mut stream = vec![0u8; 31];
        for word in [3u32, 2, 5] {
            stream.extend(word.to_le_bytes());
        }
        stream.extend([0u8, 1, 0xff, 0xff, 0xff, 0xff]);
        let (buffer, actor, _process, _pos) =
            load_scene(e, &stream, vec![0, 12], 0, 0xf1, idle_form);
        start_log(e);
        e.call(0x0049_b050, &args![this, buffer]);
        let log = end_log(e);
        let idle = idle_fields(e, this)[0];
        assert_ne!(idle, 0);
        let idle = Ptr::<AnimIdle>::new(idle);
        // Built for the actor in section 7 with play type 1, then given the
        // state, play type and section of the save.
        assert_eq!(e.get(idle, AnimIdle::pActor).addr(), actor);
        assert_eq!(e.get(idle, AnimIdle::pIdleForm).addr(), idle_form);
        assert_eq!(e.get(idle, AnimIdle::eFlags), 3);
        assert_eq!(e.get(idle, AnimIdle::eType), 2);
        assert_eq!(e.get(idle, AnimIdle::eSection), 5);
        // The length value is read and ignored; the KF is loaded, not queued.
        assert_eq!(arguments_of(&log, MODEL_LOADER_LOAD_KF).len(), 1);
        assert!(arguments_of(&log, MODEL_LOADER_LOAD_IDLE_KF).is_empty());
    }

    #[test]
    fn fn_0049baa0_builds_an_empty_group_sequence() {
        let mut e = engine_two();
        e.register(SEQUENCE_BASE_CONSTRUCT, |_, _| Ret::default());
        let object = e.mem.alloc(ANIM_GROUP_SEQUENCE_SIZE);
        e.mem.set_u32(object + 0x74, 0x5555);
        start_log(&mut e);
        assert_eq!(e.call(0x0049_baa0, &args![object]).u32(), object);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, SEQUENCE_BASE_CONSTRUCT), [[object]]);
        assert_eq!(e.mem.u32(object), VTABLE_ANIM_GROUP_SEQUENCE);
        assert_eq!(e.mem.u32(object + 0x74), 0);
    }

    #[test]
    fn group_sequence_get_rtti_returns_the_class_rtti() {
        let mut e = engine_two();
        assert_eq!(
            e.call(0x0049_bb10, &args![0x1234u32]).u32(),
            RTTI_ANIM_GROUP_SEQUENCE
        );
    }

    #[test]
    fn group_sequence_scalar_deleting_destructor_deletes_when_asked() {
        let mut e = engine_two();
        e.register(SEQUENCE_DESTRUCT, |_, _| Ret::default());
        e.register(NI_OPERATOR_DELETE, |_, _| Ret::default());
        start_log(&mut e);
        assert_eq!(e.call(0x0049_bb20, &args![0x4000u32, 0u32]).u32(), 0x4000);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, SEQUENCE_DESTRUCT), [[0x4000]]);
        assert!(arguments_of(&log, NI_OPERATOR_DELETE).is_empty());
        start_log(&mut e);
        e.call(0x0049_bb20, &args![0x4000u32, 1u32]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, NI_OPERATOR_DELETE), [[0x4000, 0x78]]);
    }

    #[test]
    fn fn_0049bb50_adds_the_count_to_the_word_at_0xc() {
        let mut e = Engine::new();
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u32(buffer + 0xc, 100);
        e.call(0x0049_bb50, &args![buffer, 23u32]);
        assert_eq!(e.mem.u32(buffer + 0xc), 123);
    }

    #[test]
    fn fn_0049bb70_frees_the_idles_and_the_replay_delays() {
        let mut e = idle_engine();
        let (this, _manager, _map) = managed_animation(&mut e);
        manager_with_sequences(&mut e, this, &[]);
        e.register(SIMPLE_LIST_ITEM, |_, a| ret(a[0]));
        e.register(SIMPLE_LIST_CLEAR, |_, _| Ret::default());
        e.register(GET_CONTROLLER, |_, _| ret(0));
        e.mem.set_u8(this.addr() + 0x120, 1);
        let stale = e.mem.alloc(0x80);
        e.mem.set_u32(this.addr() + 0xfc, stale);
        let (idle, _sequence) = playing_idle(&mut e, this, 2, 0);
        put_idles(&mut e, this, [idle, 0, 0, 0]);
        // Two replay delays: {item0, next} and {item1, 0}.
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, 0xd1);
        e.mem.set_u32(this.addr() + 0x134, 0xd0);
        e.mem.set_u32(this.addr() + 0x138, node);
        start_log(&mut e);
        e.call(0x0049_bb70, &args![this, 0u32]);
        let log = end_log(&mut e);
        assert_eq!(e.mem.u8(this.addr() + 0x120), 0);
        assert_eq!(idle_fields(&e, this), [0; 4]);
        assert_eq!(e.mem.u32(this.addr() + 0xfc), 0);
        assert_eq!(arguments_of(&log, OPERATOR_DELETE), [[0xd0], [0xd1]]);
        assert_eq!(
            arguments_of(&log, SIMPLE_LIST_CLEAR),
            [[this.addr() + 0x134]]
        );
    }

    #[test]
    fn update_bip_only_updates_the_bip_nodes_and_reports_the_accumulation_translation() {
        let mut e = engine_two();
        e.register(NODE_SET_WORLD_TRANSLATION, |_, _| Ret::default());
        e.register(BIP_NODE_UPDATE_BIP_ONLY, |_, _| Ret::default());
        e.register(NODE_WORLD_TRANSLATION, |_, a| ret(a[0] + 0x58));
        let this: Ptr<Animation> = e.new_object();
        let (root, accum) = (e.mem.alloc(0x40), e.mem.alloc(0x80));
        e.mem.set_u32(this.addr() + 8, root);
        e.mem.set_u32(this.addr() + 0xc, accum);
        for (i, value) in [4.0f32, 5.0, 6.0].iter().enumerate() {
            e.mem.set_f32(accum + 0x58 + 4 * i as u32, *value);
        }
        let out = e.mem.alloc(12);
        start_log(&mut e);
        e.call(0x0049_bca0, &args![this, 0.5f32, out, 1u32]);
        let log = end_log(&mut e);
        // The root is moved to `AccumRootTranslate`, the bip nodes are updated
        // with it, the translation is handed out and the root is moved back.
        assert_eq!(
            arguments_of(&log, NODE_SET_WORLD_TRANSLATION),
            [[accum, this.addr() + 0x1c], [accum, ZERO_VECTOR]]
        );
        assert_eq!(
            arguments_of(&log, BIP_NODE_UPDATE_BIP_ONLY),
            [[root, 0.5f32.to_bits(), accum]]
        );
        assert_eq!(
            [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)],
            [4.0, 5.0, 6.0]
        );
        // Without an output pointer nothing is copied; without an accumulation
        // root only the bip nodes are updated.
        e.mem.set_f32(out, 9.0);
        e.call(0x0049_bca0, &args![this, 0.5f32, 0u32, 1u32]);
        assert_eq!(e.mem.f32(out), 9.0);
        e.mem.set_u32(this.addr() + 0xc, 0);
        start_log(&mut e);
        e.call(0x0049_bca0, &args![this, 0.5f32, out, 1u32]);
        let log = end_log(&mut e);
        assert!(arguments_of(&log, NODE_SET_WORLD_TRANSLATION).is_empty());
        assert_eq!(
            arguments_of(&log, BIP_NODE_UPDATE_BIP_ONLY),
            [[root, 0.5f32.to_bits(), 0]]
        );
    }

    #[test]
    fn fn_0049bd30_counts_the_queued_models() {
        let mut e = idle_engine();
        let this: Ptr<Animation> = e.new_object();
        kf_model_list(&mut e, this, &[]);
        e.register(SIMPLE_LIST_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        assert_eq!(e.call(0x0049_bd30, &args![this]).u32(), 0);
        kf_model_list(&mut e, this, &[0x11, 0x22, 0x33]);
        e.register(SIMPLE_LIST_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        assert_eq!(e.call(0x0049_bd30, &args![this]).u32(), 3);
    }

    #[test]
    fn fn_0049bd90_hands_an_object_to_the_animation_root() {
        let mut e = idle_engine();
        e.register(ROOT_NODE_NOTIFY, |_, _| Ret::default());
        let this: Ptr<Animation> = e.new_object();
        start_log(&mut e);
        e.call(0x0049_bd90, &args![this, 0x5000u32]);
        assert!(arguments_of(&end_log(&mut e), ROOT_NODE_NOTIFY).is_empty());
        e.mem.set_u32(this.addr() + 8, 0x6000);
        start_log(&mut e);
        e.call(0x0049_bd90, &args![this, 0x5000u32]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, ROOT_NODE_NOTIFY), [[0x6000, 0x5000]]);
    }

    #[test]
    fn fn_0049bdc0_drives_the_group_in_slot_3_by_weight() {
        let mut e = idle_engine();
        let (this, _manager, _map) = managed_animation(&mut e);
        e.register(SEQUENCE_UPDATE, |_, _| Ret::default());
        e.register(SEQUENCE_SET_PHASE, |_, _| Ret::default());
        e.register(UPDATE_BIP_ONLY, |_, _| Ret::default());
        e.register(SEQUENCE_BEGIN_TIME, |e, a| {
            e.mem.f32(a[0] + 0x2c).into_ret()
        });
        // Weight 0 with a sequence: update it to time 0 and clear the slot.
        let sequence = e.mem.alloc(0x80);
        e.mem.set_u32(current_sequence_at(this, 3), sequence);
        start_log(&mut e);
        e.call(0x0049_bdc0, &args![this, 0.0f32]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, SEQUENCE_UPDATE), [[sequence, 0, 1]]);
        assert_eq!(e.mem.u32(current_sequence_at(this, 3)), 0);
        // Weight 0 with an empty slot does nothing.
        start_log(&mut e);
        e.call(0x0049_bdc0, &args![this, 0.0f32]);
        assert!(called(&mut e).is_empty());
        // A sequence in state 1 gets a lip time between its begin (1.0) and end
        // (3.0) times by the weight, and the phase is that minus the time.
        e.mem.set_u32(sequence + 0x44, 1);
        e.mem.set_f32(sequence + 0x2c, 1.0);
        e.mem.set_f32(sequence + 0x30, 3.0);
        e.mem.set_u32(current_sequence_at(this, 3), sequence);
        e.set(this, Animation::time, 0.5f32);
        start_log(&mut e);
        e.call(0x0049_bdc0, &args![this, 0.25f32]);
        let log = end_log(&mut e);
        assert_eq!(e.get(this, Animation::fLipTime), 1.5);
        assert_eq!(
            arguments_of(&log, SEQUENCE_SET_PHASE),
            [[sequence, 1.0f32.to_bits(), 1]]
        );
        // Any other state leaves the lip time alone.
        e.mem.set_u32(sequence + 0x44, 2);
        e.set(this, Animation::fLipTime, 0.0f32);
        e.call(0x0049_bdc0, &args![this, 0.25f32]);
        assert_eq!(e.get(this, Animation::fLipTime), 0.0);
        // An empty slot starts the group 0xe1: the map does not know it, so
        // nothing else happens.
        e.mem.set_u32(current_sequence_at(this, 3), 0);
        start_log(&mut e);
        e.call(0x0049_bdc0, &args![this, 0.25f32]);
        let log = end_log(&mut e);
        assert!(arguments_of(&log, SEQUENCE_UPDATE).is_empty());
    }

    #[test]
    fn fn_0049bf10_drives_a_sequence_on_a_manager() {
        let mut e = idle_engine();
        e.register(SEQUENCE_UPDATE, |_, _| Ret::default());
        e.register(MANAGER_SET_FLAG, |_, _| Ret::default());
        e.register(MANAGER_ACTIVATE, |_, _| Ret::default());
        e.register(SEQUENCE_BEGIN_TIME, |e, a| {
            e.mem.f32(a[0] + 0x2c).into_ret()
        });
        e.register(SEQUENCE_SET_PHASE, |_, _| Ret::default());
        e.map(0x011f_6000, 0x1000);
        e.set_global(DOUBLE_THOUSAND, 1000.0f64);
        // The time scale object keeps 1500 milliseconds at +0x14.
        e.mem.set_u32(TIME_SCALE_OBJECT + 0x14, 1500);
        let manager = e.mem.alloc(0x40);
        let sequence = e.mem.alloc(0x80);
        e.mem.set_f32(sequence + 0x2c, 1.0);
        e.mem.set_f32(sequence + 0x30, 3.0);
        let call = |e: &mut Engine, weight: f32, manager: u32, sequence: u32| {
            start_log(e);
            e.call(0x0049_bf10, &args![weight, manager, sequence]);
            end_log(e)
        };
        // Without a manager or a sequence nothing happens.
        assert!(call(&mut e, 0.5, 0, sequence).is_empty());
        assert!(call(&mut e, 0.5, manager, 0).is_empty());
        // Weight 0: a sequence in state 1 is reset.
        e.mem.set_u32(sequence + 0x44, 1);
        let log = call(&mut e, 0.0, manager, sequence);
        assert_eq!(arguments_of(&log, SEQUENCE_SET_PHASE), [[sequence, 0, 1]]);
        assert_eq!(arguments_of(&log, SEQUENCE_UPDATE), [[sequence, 0, 1]]);
        e.mem.set_u32(sequence + 0x44, 0);
        let log = call(&mut e, 0.0, manager, sequence);
        assert!(arguments_of(&log, SEQUENCE_UPDATE).is_empty());
        // A sequence in state 0 is activated and updated to begin + 1.5 s.
        let log = call(&mut e, 0.5, manager, sequence);
        assert_eq!(arguments_of(&log, MANAGER_SET_FLAG), [[manager, 1]]);
        assert_eq!(
            arguments_of(&log, MANAGER_ACTIVATE),
            [[manager, sequence, 0, 0, 1.0f32.to_bits(), 0, 0]]
        );
        assert_eq!(
            arguments_of(&log, SEQUENCE_UPDATE),
            [[sequence, 2.5f32.to_bits(), 1]]
        );
        // In state 1 the phase is begin + (end - begin) * weight - 1.5 s.
        e.mem.set_u32(sequence + 0x44, 1);
        let log = call(&mut e, 0.5, manager, sequence);
        assert_eq!(
            arguments_of(&log, SEQUENCE_SET_PHASE),
            [[sequence, 0.5f32.to_bits(), 1]]
        );
        assert!(arguments_of(&log, MANAGER_ACTIVATE).is_empty());
    }

    #[test]
    fn fn_0049c050_builds_the_sequence_map() {
        let mut e = engine_two();
        e.register(MAP_BASE_CONSTRUCT, |_, _| Ret::default());
        let object = e.mem.alloc(0x20);
        start_log(&mut e);
        assert_eq!(e.call(0x0049_c050, &args![object, 37u32]).u32(), object);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, MAP_BASE_CONSTRUCT), [[object, 37]]);
        assert_eq!(e.mem.u32(object), VTABLE_ANIM_SEQUENCE_MAP);
    }

    // ---- Fifth session: the sequence map and the tail ------------------------------

    /// A map of `size` buckets whose vtable has the slots the map code calls: the
    /// hash (`key % size`), the comparison (equal words), the node allocator, and
    /// the real `SetValue` (`0049c4e0`). Returns the map.
    fn bucket_map(e: &mut Engine, size: u32) -> u32 {
        e.register(0x00f1_0004, |e, a| ret(a[1] % e.mem.u32(a[0] + 4)));
        e.register(0x00f1_0008, |_, a| ret((a[1] == a[2]) as u32));
        e.register(0x00f1_0014, |e, _| ret(e.mem.alloc(12)));
        let map = object_with_vtable(
            e,
            0x10,
            &[
                (4, 0x00f1_0004),
                (8, 0x00f1_0008),
                (0x14, 0x00f1_0014),
                (0xc, 0x0049_c4e0),
            ],
        );
        let table = e.mem.alloc(size * 4);
        e.mem.set_u32(map + 4, size);
        e.mem.set_u32(map + 8, table);
        map
    }

    /// Stores `value` under `key` with `SetAt` and returns nothing.
    fn set_at(e: &mut Engine, map: u32, key: u32, value: u32) {
        e.call(0x0049_c170, &args![map, key, value]);
    }

    /// `GetAt` of `key`: the value, or `None`.
    fn get_at(e: &mut Engine, map: u32, key: u32) -> Option<u32> {
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xdead_beef);
        if e.call(0x0049_c390, &args![map, key, out]).bool() {
            Some(e.mem.u32(out))
        } else {
            None
        }
    }

    #[test]
    fn fn_0049c170_adds_a_node_replaces_a_value_and_chains_a_collision() {
        let mut e = engine();
        let map = bucket_map(&mut e, 4);
        set_at(&mut e, map, 5, 0x500);
        assert_eq!(e.mem.u32(map + 0xc), 1);
        // The node is in bucket 1 with the key and value `SetValue` wrote.
        let table = e.mem.u32(map + 8);
        let node = e.mem.u32(table + 4);
        assert_eq!(e.mem.u16(node + 4), 5);
        assert_eq!(e.mem.u32(node + 8), 0x500);
        assert_eq!(e.mem.u32(node), 0);
        // The same key replaces the value and does not count again.
        set_at(&mut e, map, 5, 0x501);
        assert_eq!(e.mem.u32(map + 0xc), 1);
        assert_eq!(e.mem.u32(node + 8), 0x501);
        // Key 9 hashes to bucket 1 too: it becomes the head, linking the old one.
        set_at(&mut e, map, 9, 0x900);
        assert_eq!(e.mem.u32(map + 0xc), 2);
        let head = e.mem.u32(table + 4);
        assert_ne!(head, node);
        assert_eq!(e.mem.u32(head), node);
        assert_eq!(e.mem.u16(head + 4), 9);
    }

    #[test]
    fn fn_0049c390_finds_values_in_a_chain_and_reports_a_miss() {
        let mut e = engine();
        let map = bucket_map(&mut e, 4);
        set_at(&mut e, map, 5, 0x500);
        set_at(&mut e, map, 9, 0x900);
        assert_eq!(get_at(&mut e, map, 5), Some(0x500));
        assert_eq!(get_at(&mut e, map, 9), Some(0x900));
        // Same bucket, absent key; and an empty bucket.
        assert_eq!(get_at(&mut e, map, 13), None);
        assert_eq!(get_at(&mut e, map, 2), None);
        // A miss leaves the out word alone.
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 77);
        assert!(!e.call(0x0049_c390, &args![map, 2u32, out]).bool());
        assert_eq!(e.mem.u32(out), 77);
    }

    #[test]
    fn fn_0049c410_walks_a_chain_then_the_next_buckets_then_ends() {
        let mut e = engine();
        let map = bucket_map(&mut e, 4);
        set_at(&mut e, map, 5, 0x500);
        set_at(&mut e, map, 9, 0x900);
        set_at(&mut e, map, 3, 0x300);
        let table = e.mem.u32(map + 8);
        let position = e.mem.alloc(4);
        let key = e.mem.alloc(4);
        let value = e.mem.alloc(4);
        e.mem.set_u32(key, 0xffff_ffff);
        // Start at bucket 1's head (key 9): next is key 5 in the same bucket.
        let first = e.mem.u32(table + 4);
        let second = e.mem.u32(first);
        e.mem.set_u32(position, first);
        e.call(0x0049_c410, &args![map, position, key, value]);
        // Only the low half of the key word is written.
        assert_eq!(e.mem.u32(key), 0xffff_0009);
        assert_eq!(e.mem.u32(value), 0x900);
        assert_eq!(e.mem.u32(position), second);
        // Key 5 ends the chain: buckets 2 is empty, bucket 3 holds key 3.
        e.call(0x0049_c410, &args![map, position, key, value]);
        assert_eq!(e.mem.u16(key), 5);
        assert_eq!(e.mem.u32(value), 0x500);
        assert_eq!(e.mem.u32(position), e.mem.u32(table + 12));
        // Key 3 is in the last bucket: the position becomes null.
        e.call(0x0049_c410, &args![map, position, key, value]);
        assert_eq!(e.mem.u16(key), 3);
        assert_eq!(e.mem.u32(value), 0x300);
        assert_eq!(e.mem.u32(position), 0);
    }

    #[test]
    fn fn_0049c4e0_fills_the_node_without_touching_the_rest() {
        let mut e = engine();
        let node = e.mem.alloc(12);
        e.mem.set_u32(node, 0x1111);
        e.mem.set_u16(node + 6, 0x7777);
        e.call(0x0049_c4e0, &args![0u32, node, 0x1234_5678u32, 0xabcdu32]);
        assert_eq!(e.mem.u16(node + 4), 0x5678);
        assert_eq!(e.mem.u32(node + 8), 0xabcd);
        assert_eq!(e.mem.u32(node), 0x1111);
        assert_eq!(e.mem.u16(node + 6), 0x7777);
    }

    #[test]
    fn fn_0049c100_allocates_and_clears_the_buckets() {
        let mut e = engine();
        e.register(ARRAY_ALLOC, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMSET, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 0xc, 99);
        start_log(&mut e);
        assert_eq!(e.call(0x0049_c100, &args![map, 37u32]).u32(), map);
        let log = end_log(&mut e);
        assert_eq!(e.mem.u32(map), VTABLE_MAP_BASE);
        assert_eq!(e.mem.u32(map + 4), 37);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        let table = e.mem.u32(map + 8);
        assert_ne!(table, 0);
        assert_eq!(arguments_of(&log, ARRAY_ALLOC), [[148]]);
        assert_eq!(arguments_of(&log, MEMSET), [[table, 0, 148]]);
    }

    #[test]
    fn destructors_reset_the_vtables_and_release_the_buckets() {
        let mut e = engine();
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(ARRAY_FREE, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x4444_0000);
        // 0049c570: the base destructor.
        start_log(&mut e);
        e.call(0x0049_c570, &args![map]);
        let log = end_log(&mut e);
        assert_eq!(e.mem.u32(map), VTABLE_MAP_BASE);
        assert_eq!(arguments_of(&log, MAP_REMOVE_ALL), [[map]]);
        assert_eq!(arguments_of(&log, ARRAY_FREE), [[0x4444_0000]]);
        // 0049c510: its own vtable first (the base destructor then overwrites it).
        start_log(&mut e);
        e.call(0x0049_c510, &args![map]);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, MAP_REMOVE_ALL).len(), 2);
        assert_eq!(arguments_of(&log, ARRAY_FREE).len(), 1);
        assert!(arguments_of(&log, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn scalar_deleting_destructors_delete_only_with_bit_zero() {
        let mut e = engine();
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(ARRAY_FREE, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let map = e.mem.alloc(0x10);
        for address in [0x0049_c080u32, 0x0049_c5a0] {
            start_log(&mut e);
            assert_eq!(e.call(address, &args![map, 2u32]).u32(), map);
            let log = end_log(&mut e);
            assert!(arguments_of(&log, OPERATOR_DELETE).is_empty());
            assert_eq!(arguments_of(&log, ARRAY_FREE).len(), 1);
            start_log(&mut e);
            assert_eq!(e.call(address, &args![map, 1u32]).u32(), map);
            let log = end_log(&mut e);
            assert_eq!(arguments_of(&log, OPERATOR_DELETE), [[map]]);
            assert_eq!(arguments_of(&log, ARRAY_FREE).len(), 1);
        }
    }

    #[test]
    fn fn_0049c0b0_removes_the_found_node_or_returns_the_item_word() {
        let mut e = engine();
        e.register(LIST_FIND_NODE, |e, a| {
            ret(if e.mem.u32(a[1]) == 7 { 0x5555_0000 } else { 0 })
        });
        e.register(LIST_REMOVE_NODE, |e, a| ret(e.mem.u32(a[1]) + 1));
        let list = e.mem.alloc(0xc);
        let item = e.mem.alloc(4);
        // Found: the remove function gets the address of a cell holding the node.
        e.mem.set_u32(item, 7);
        start_log(&mut e);
        assert_eq!(e.call(0x0049_c0b0, &args![list, item]).u32(), 0x5555_0001);
        let log = end_log(&mut e);
        assert_eq!(arguments_of(&log, LIST_FIND_NODE), [[list, item, 0]]);
        assert_eq!(arguments_of(&log, LIST_REMOVE_NODE).len(), 1);
        // Not found: the item's own word comes back and nothing is removed.
        e.mem.set_u32(item, 8);
        start_log(&mut e);
        assert_eq!(e.call(0x0049_c0b0, &args![list, item]).u32(), 8);
        let log = end_log(&mut e);
        assert!(arguments_of(&log, LIST_REMOVE_NODE).is_empty());
    }

    #[test]
    fn fn_0058cb60_copies_four_fields() {
        let mut e = engine();
        let object = e.mem.alloc(0x30);
        e.mem.set_u32(object + 8, 0xaaaa_1234);
        e.mem.set_u32(object + 0x10, 0x1000_0001);
        e.mem.set_u8(object + 0x1c, 0x9c);
        e.mem.set_u32(object + 0x20, 0x2000_0002);
        let outs = e.mem.alloc(12);
        e.mem.set_u32(outs + 8, 0xffff_ffff);
        let result = e.call(0x0058_cb60, &args![object, outs, outs + 4, outs + 8]);
        assert_eq!(result.u32(), 0x2000_0002);
        assert_eq!(e.mem.u32(outs), 0x1234);
        assert_eq!(e.mem.u32(outs + 4), 0x1000_0001);
        // The third out is a byte.
        assert_eq!(e.mem.u32(outs + 8), 0xffff_ff9c);
    }

    #[test]
    fn fn_0058cb00_reads_through_the_pointer_at_0x2c() {
        let mut e = engine();
        e.register(READ_WORD, |e, a| ret(e.mem.u32(a[0])));
        let owner = e.mem.alloc(0x40);
        let outs = e.mem.alloc(12);
        e.mem.set_u32(outs, 5);
        e.mem.set_u32(outs + 4, 6);
        e.mem.set_u32(outs + 8, 0xffff_ffff);
        // No object: zeros, the third out only a byte.
        assert_eq!(
            e.call(0x0058_cb00, &args![owner, outs, outs + 4, outs + 8])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(outs), 0);
        assert_eq!(e.mem.u32(outs + 4), 0);
        assert_eq!(e.mem.u32(outs + 8), 0xffff_ff00);
        // With an object the fields come from it.
        let object = e.mem.alloc(0x30);
        e.mem.set_u16(object + 8, 0x0102);
        e.mem.set_u32(object + 0x10, 0x33);
        e.mem.set_u8(object + 0x1c, 4);
        e.mem.set_u32(object + 0x20, 0x55);
        e.mem.set_u32(owner + 0x2c, object);
        let result = e.call(0x0058_cb00, &args![owner, outs, outs + 4, outs + 8]);
        assert_eq!(result.u32(), 0x55);
        assert_eq!(e.mem.u32(outs), 0x0102);
        assert_eq!(e.mem.u32(outs + 4), 0x33);
        assert_eq!(e.mem.u8(outs + 8), 4);
    }

    #[test]
    fn fn_00a3f9e0_reads_the_object_at_0x2c_or_zeros() {
        let mut e = engine();
        let owner = e.mem.alloc(0x40);
        let outs = e.mem.alloc(12);
        e.mem.set_u32(outs, 5);
        e.mem.set_u32(outs + 4, 6);
        e.mem.set_u32(outs + 8, 0xffff_ffff);
        assert_eq!(
            e.call(0x00a3_f9e0, &args![owner, outs, outs + 4, outs + 8])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(outs), 0);
        assert_eq!(e.mem.u32(outs + 4), 0);
        assert_eq!(e.mem.u32(outs + 8), 0xffff_ff00);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object + 0xc, 0xbbbb_0102);
        e.mem.set_u32(object + 0x18, 0x66);
        e.mem.set_u8(object + 0x1e, 7);
        e.mem.set_u32(object + 0x28, 0x88);
        e.mem.set_u32(owner + 0x2c, object);
        let result = e.call(0x00a3_f9e0, &args![owner, outs, outs + 4, outs + 8]);
        assert_eq!(result.u32(), 0x88);
        assert_eq!(e.mem.u32(outs), 0x0102);
        assert_eq!(e.mem.u32(outs + 4), 0x66);
        assert_eq!(e.mem.u8(outs + 8), 7);
    }

    #[test]
    fn animation_skip_update_stores_the_byte() {
        let mut e = engine();
        let this = e.new_object::<Animation>();
        e.mem.set_u8(this.addr() + 0xcd, 0x5a);
        e.call(0x008e_eaa0, &args![this, 0x14u32]);
        assert_eq!(e.mem.u8(this.addr() + 0xcc), 0x14);
        assert_eq!(e.mem.u8(this.addr() + 0xcd), 0x5a);
    }

    #[test]
    fn special_idle_auto_constructs_the_door_physics_setting_and_registers_exit() {
        let mut e = engine();
        e.register(0x0045_cf30, |_, a| ret(a[0]));
        e.register(0x00ec_658f, |_, _| ret(0));
        start_log(&mut e);
        e.call(0x00f3_9c20, &args![]);
        assert_eq!(
            e.call_log.clone().unwrap(),
            vec![
                (0x00f3_9c20, vec![]),
                (0x0045_cf30, vec![0x011c_4c64, 0x0101_abb4, 1]),
                (0x00ec_658f, vec![0x00fc_8470]),
            ]
        );
    }
}

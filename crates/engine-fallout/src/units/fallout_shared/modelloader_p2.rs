//! `fallout shared/modelloader.cpp` (Xbox PDB source unit), part 2: its functions from `0043fed0` up to
//! (not including) `004457d0` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::modelloader`]; anything public there may be used here.
//!
//! State of this file: all 120 functions of its range, `0043fed0` to
//! `00445750` (the range is complete). The third 40, `004431b0` to `00445750`,
//! are the loader's report (`OutputModelMapContents`), its texture, model,
//! KF, tree, helmet, animation idle and reference queueing (`QueueTexture`,
//! `QueueModel`, `QueueReference` and the helpers they use), `ReleaseModel`,
//! the release of KF models, the `CancelReference` family, the iterator and
//! model deleting destructors and the small counters and flag tests of the
//! cells and references.
//!
//! The second 40 (`004414c0` to `00443190`) are the rest of the
//! character, creature and player tasks (`QueueModels`, `BackgroundClone`,
//! `FinishAttach`, constructors, destructors), `QueuedFileLoad` and
//! `QueuedFaceGenFile`, the `BackgroundCloneThread` (constructor, destructor,
//! `ThreadUpdate`), `ModelLoader`'s constructor and destructor, and the
//! reference-count sum of a model. The first 40, `0043fed0` to `00441440`,
//! are the reference tasks
//! of the loader: `QueuedReference`
//! (`QueueMe`, `QueueModels`, `UseDistant3D`, `AttachDistant3D`,
//! `CheckFinished`, `Cancel`, `BackgroundClone`, `Attach`, `GetDescription`),
//! the `AttachDistant3DTask` and `IOTask` helpers they use, the counters of
//! the parent cell, and the subclasses `QueuedTree`, `QueuedActor` and
//! `QueuedCharacter` (constructors, destructors, `QueueModels`, `QueueMe`,
//! `UseDistant3D`).
//!
//! Not translated: the compiler's exception-unwinding frames (the `FS:[0]`
//! chains and state variables) and the stack-cookie check of `00440190`.
//! The memory-context guard the game keeps on its stack (`00404eb0` /
//! `00404ee0`) and the other locals it passes by address are stack blocks of
//! the engine here.
//!
//! Stack arguments: as in the main file, several functions push an argument
//! for a `thiscall` helper before the `this` of an earlier, argument-less
//! getter is loaded; the translations follow the disassembly.

#[allow(unused_imports)]
use super::modelloader::*;
#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::TESForm;
use crate::units::fallout_shared::tesobjectcell::TESObjectCELL;

/// `MemoryManager` allocation, `__cdecl(size) -> block` (`00401000`), and
/// deallocation, `__cdecl(block)` (`00401030`).
const MEMORY_ALLOC: u32 = 0x0040_1000;
const MEMORY_FREE: u32 = 0x0040_1030;
/// The memory-context guard of the game's stack frame: constructor
/// `__thiscall(guard, context, 1, source file name, source line)`
/// (`00404eb0`) and destructor (`00404ee0`).
const MEMORY_CONTEXT_ENTER: u32 = 0x0040_4eb0;
const MEMORY_CONTEXT_LEAVE: u32 = 0x0040_4ee0;
/// The source file name the guard is given (`"D:\_Fallout3\Platforms\Common\
/// Code\Fallout Shared\ModelLoader.cpp"`).
const MODEL_LOADER_SOURCE: u32 = 0x0101_6840;
/// `NiPointer<T>` functions: `operator T*` (`00559450`),
/// `operator=(T*)` (`0066b0d0`), `NiPointer(T*)` (`00633c90`) and
/// `~NiPointer` (`0045cec0`).
const NI_POINTER_GET: u32 = 0x0055_9450;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// The task pointer functions of this unit: `NiPointer(task)` (`00528cb0`),
/// `~NiPointer` (`0044cbf0`) and `operator=(task)` (`006f74f0`, named
/// `NiPointer<QueuedFile>::operator=` in the engine map).
const TASK_POINTER_CONSTRUCT: u32 = 0x0052_8cb0;
const TASK_POINTER_DESTRUCT: u32 = 0x0044_cbf0;
const TASK_POINTER_ASSIGN: u32 = 0x006f_74f0;
/// `NiPointer<Model>::operator=(Model*)` (`0044aed0`) and the `NiPointer<
/// Model>` destructor (`0040c110`).
const MODEL_POINTER_ASSIGN: u32 = 0x0044_aed0;
const MODEL_POINTER_RELEASE: u32 = 0x0040_c110;
/// `InterlockedDecrement` wrapper, `__cdecl(address)` (`004019a0`).
const INTERLOCKED_DECREMENT: u32 = 0x0040_19a0;
/// `sprintf_s`-style formatter, `__cdecl(buffer, size, format, ...)`
/// (`00406d00`).
const FORMAT_STRING: u32 = 0x0040_6d00;

/// The global holding the `ModelLoader` (`011c3b3c`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// The global holding the `TES` object (`011dea10`).
const TES_GLOBAL: u32 = 0x011d_ea10;
/// The global holding the reference the loader treats specially
/// (`011dea3c`; `QueuedReference::CheckFinished` and `QueuedActor::QueueMe`
/// compare the task's reference with it).
const SPECIAL_REFERENCE: u32 = 0x011d_ea3c;
/// The global holding the object whose dword at +0x10 is the id of the
/// thread that may attach 3D directly (`011dea0c`; getter `0044edb0`).
const THREAD_OWNER_OBJECT: u32 = 0x011d_ea0c;
/// The global holding the IO manager (`01202d98`).
const TASK_QUEUE: u32 = 0x0120_2d98;
/// The table of 12-byte entries per `TESForm` type whose first dword is the
/// type's name text (`01187004`, used by `00440e30`).
const FORM_TYPE_TABLE: u32 = 0x0118_7004;
/// The thread-local byte at +0x25C that `00441290` swaps (the same byte
/// `0043c130` reads).
const TLS_QUEUED_FLAG: u32 = 0x25c;

/// `"Queued ref '%s' (%08X) of type %s"`: the description format of
/// `QueuedReference::GetDescription`.
const QUEUED_REF_FORMAT: u32 = 0x0101_6c30;
/// The virtual tables of `AttachDistant3DTask` (`01016bec`), `IOTask`
/// (`01016c10`), `QueuedTree` (`01016c5c`), `QueuedActor` (`01016ca4`) and
/// `QueuedCharacter` (`01016cec`).
const ATTACH_DISTANT_3D_TASK_VTABLE: u32 = 0x0101_6bec;
const IO_TASK_VTABLE: u32 = 0x0101_6c10;
const QUEUED_TREE_VTABLE: u32 = 0x0101_6c5c;
const QUEUED_ACTOR_VTABLE: u32 = 0x0101_6ca4;
const QUEUED_CHARACTER_VTABLE: u32 = 0x0101_6cec;

/// Source lines the memory-context guards of the functions are given.
const QUEUE_ME_SOURCE_LINE: u32 = 0x72a;
const QUEUE_MODELS_SOURCE_LINE: u32 = 0x752;
const BACKGROUND_CLONE_SOURCE_LINE: u32 = 0x7ea;
const ATTACH_SOURCE_LINE: u32 = 0x804;
const TREE_QUEUE_MODELS_SOURCE_LINE: u32 = 0x847;
const ACTOR_QUEUE_MODELS_SOURCE_LINE: u32 = 0x8a2;

/// `TESObjectREFR` methods: `GetTESModel` (Xbox PDB, `00571630`), the
/// address of the embedded extra data list (`005d43c0`, `this + 0x44`),
/// `00570f70(node)` (gives the reference its 3D), the parent cell getter
/// (`008d6f30`), `Set3DVerySimple` (Xbox PDB, `00571080`), `Is3DCritical`
/// (Xbox PDB, `00572350`), `TESActorBase::SetStartsDead` (Xbox PDB,
/// `00565210`), `GetOrientation(out)` (Xbox PDB, `0056fa00`, returns the
/// matrix it wrote) and `GetScale` (Xbox PDB, `00567400`, a `float` in ST0).
const REFERENCE_GET_TES_MODEL: u32 = 0x0057_1630;
const REFERENCE_EXTRA_LIST: u32 = 0x005d_43c0;
const REFERENCE_SET_3D: u32 = 0x0057_0f70;
const REFERENCE_CELL: u32 = 0x008d_6f30;
const REFERENCE_SET_3D_VERY_SIMPLE: u32 = 0x0057_1080;
const REFERENCE_IS_3D_CRITICAL: u32 = 0x0057_2350;
const SET_STARTS_DEAD: u32 = 0x0056_5210;
const REFERENCE_GET_ORIENTATION: u32 = 0x0056_fa00;
const REFERENCE_GET_SCALE: u32 = 0x0056_7400;
/// Two predicates of `tesobjectrefr.cpp`'s range on a reference
/// (`00564e60`, `00564f00`; no names in the engine map). The first is true
/// when bit `0x8000` of the form flags is set, or when the base form passes
/// `00564e40`.
const REFERENCE_TEST_00564E60: u32 = 0x0056_4e60;
const REFERENCE_TEST_00564F00: u32 = 0x0056_4f00;
/// `BGSSaveFormBuffer::GetForm` (the engine map's name for the getter of the
/// dword at +0x20; of a reference it gives the base form).
const GET_BASE_FORM: u32 = 0x007a_f430;
/// The getter of the dword at +0xC of a form, its form ID (`0084e3a0`).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// The form type byte of a form (`00401170`, `this + 4` zero-extended).
const FORM_TYPE: u32 = 0x0040_1170;
/// `ExtraDataList::GetDecalRefs` (Xbox PDB, `0041f050`) and
/// `ExtraDataList::GetDismembermentExtra` (Xbox PDB, `0042e8c0`).
const EXTRA_GET_DECAL_REFS: u32 = 0x0041_f050;
const EXTRA_GET_DISMEMBERMENT: u32 = 0x0042_e8c0;
/// Singly linked list node helpers: `IsEmpty` (`008256d0`: no item and no
/// next node), the address of the item (`006815c0`, the node itself) and the
/// next node (`00726070`, `node + 4`).
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// The element count of a `BSSimpleArray` (the dword at +8, `0044ddc0`) and
/// the element address getter (`006a7ad0`, `__thiscall(index)`).
const ARRAY_COUNT_AT_8: u32 = 0x0044_ddc0;
const ARRAY_ELEMENT_ADDRESS: u32 = 0x006a_7ad0;
/// `BGSTextureSet::QueueTextureSet` (Xbox PDB, `005930e0`),
/// `__thiscall(priority, parent task, 0)`.
const QUEUE_TEXTURE_SET: u32 = 0x0059_30e0;
/// `QueuedFile::Cancel` (Xbox PDB, `00c3cb70`), `__thiscall(2 arguments)`.
const QUEUED_FILE_CANCEL: u32 = 0x00c3_cb70;
/// `IOManager::RequeueTask` (Xbox PDB, `00c3df40`), `__thiscall(task, 1
/// argument)`.
const IO_MANAGER_REQUEUE_TASK: u32 = 0x00c3_df40;
/// `BSTask` state helpers: set the state to 5 (`00449150`), state at least 4
/// (`00449110`), state is 6 (`00449130`), state is 0 (`0069b080`) and set the
/// state to 6 (`00449170`).
const TASK_SET_STATE_5: u32 = 0x0044_9150;
const TASK_STATE_AT_LEAST_4: u32 = 0x0044_9110;
const TASK_STATE_IS_6: u32 = 0x0044_9130;
const TASK_STATE_IS_0: u32 = 0x0069_b080;
const TASK_SET_STATE_6: u32 = 0x0044_9170;
/// The `IOTask` base constructor (`00449090`) and destructor (`004490e0`).
const IO_TASK_BASE_CONSTRUCT: u32 = 0x0044_9090;
const IO_TASK_BASE_DESTRUCT: u32 = 0x0044_90e0;
/// The `ModelLoader`'s queue of finished tasks: `__thiscall(task pointer)` on
/// the object at `loader + 0x1c` (`00449240`).
const FINISHED_QUEUE_ADD: u32 = 0x0044_9240;
/// `ModelLoader` methods used by the reference tasks, `__thiscall` on the
/// loader: `FindModel(name, NiPointer<Model> out)` (`004472a0`), the
/// reference-model queueing `00444dc0(form, NiPointer<QueuedModel> slot,
/// priority, parent task, reference)`, `00444540(reference, form, priority,
/// parent task, LOD multiplier)`, `00444d40(value, priority, parent task,
/// reference)` and `LoadAddonNodes(reference, node)` (`004489b0`).
const MODEL_LOADER_FIND_MODEL: u32 = 0x0044_72a0;
const MODEL_LOADER_QUEUE_REFERENCE_MODEL: u32 = 0x0044_4dc0;
const MODEL_LOADER_QUEUE_TREE_MODEL: u32 = 0x0044_4540;
const MODEL_LOADER_QUEUE_DISMEMBER_PART: u32 = 0x0044_4d40;
const MODEL_LOADER_LOAD_ADDON_NODES: u32 = 0x0044_89b0;
/// `ModelLoader` method `00442580(task)`, called on the object at
/// `loader + 0x28`.
const BACKGROUND_CLONE_QUEUE_ADD: u32 = 0x0044_2580;
/// `QueuedModel` getter of the `NiPointer<Model>` at +0x30 (`004a8a90`).
const QUEUED_MODEL_GET_MODEL: u32 = 0x004a_8a90;
/// `TES` methods (`tes.cpp`): `IsCellLoaded(cell, flag)` (`004511e0`),
/// `00451e40(reference)` and the reference loader `00451ef0(reference, cell,
/// task or 0, 0)`.
const TES_IS_CELL_LOADED: u32 = 0x0045_11e0;
const TES_REFERENCE_PREDICATE: u32 = 0x0045_1e40;
const TES_LOAD_REFERENCE: u32 = 0x0045_1ef0;
/// `TESObjectCELL` methods (`tesobjectcell.cpp`): `00541ac0` and `00541ae0`
/// are called on the parent cell before and after the attach.
const CELL_ENTER_ATTACH: u32 = 0x0054_1ac0;
const CELL_LEAVE_ATTACH: u32 = 0x0054_1ae0;
/// `TESObjectTREE::BuildDistant3D` (Xbox PDB, `0051cba0`, `__thiscall(0)`).
const BUILD_DISTANT_3D: u32 = 0x0051_cba0;
/// `TESModel::GetDistantModelName_ov2` (Xbox PDB, `00489190`),
/// `__cdecl(buffer, form)`.
const GET_DISTANT_MODEL_NAME: u32 = 0x0048_9190;
/// `BGSDestructibleObjectForm::GetDestructionForm` (Xbox PDB, `00475400`,
/// `__cdecl(form)`) and `BGSDestructibleObjectForm::QueueFiles` (Xbox PDB,
/// `00477640`, `__thiscall(form, priority, parent task)`).
const GET_DESTRUCTION_FORM: u32 = 0x0047_5400;
const DESTRUCTIBLE_QUEUE_FILES: u32 = 0x0047_7640;
/// `NiObject::Clone_ov2` (Xbox PDB, `00a5d680`) and `BSFadeNode::
/// SetLODMultType` (Xbox PDB, `00b4dec0`).
const NI_OBJECT_CLONE: u32 = 0x00a5_d680;
const FADE_NODE_SET_LOD_MULT_TYPE: u32 = 0x00b4_dec0;
/// `TES::GetLODMult` (Xbox PDB, `0045c6b0`, `__cdecl(form)`).
const GET_LOD_MULT: u32 = 0x0045_c6b0;
/// `NiAVObject::GetProperty` (Xbox PDB, `00a59d30`, `__thiscall(type)`),
/// `NiGeometryData::SetConsistency` (Xbox PDB, `00a67050`) and
/// `BSShaderManager::PrepareObject(node, flag, flag)` (Xbox PDB, `00b57e30`,
/// `__cdecl`).
const GET_PROPERTY: u32 = 0x00a5_9d30;
const SET_CONSISTENCY: u32 = 0x00a6_7050;
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
/// A method of `tesobjectcell.cpp`'s range (`005495f0`): the `NiPointer` at
/// +0xB8 of the object dereferenced.
const OBJECT_POINTER_AT_B8: u32 = 0x0054_95f0;
/// Two setters on the geometry data (`004410d0`, `004410f0`; no names in the
/// engine map, `__thiscall(one argument)`) and the type getter of a
/// property (`00441110`).
const GEOMETRY_DATA_SETTER_FIRST: u32 = 0x0044_10d0;
const GEOMETRY_DATA_SETTER_SECOND: u32 = 0x0044_10f0;
const PROPERTY_GET_TYPE: u32 = 0x0044_1110;
/// The type `00441110` gives for the property whose shader flags
/// `QueuedTree::UseDistant3D` sets (`0xb`).
const SHADER_PROPERTY_TYPE: u32 = 0xb;
/// The property type passed to `GetProperty` by `QueuedTree::UseDistant3D`.
const SHADER_PROPERTY_SLOT: u32 = 3;
/// `Actor` methods of `actor.cpp` / `actor_magic.cpp`: `008b7b70` (a
/// predicate), `008b7360(1, task)` and `008c2e50` (no result used), and
/// `00936ef0` (a getter whose result is only tested against 0).
const ACTOR_TEST_008B7B70: u32 = 0x008b_7b70;
const ACTOR_ATTACH_TASK: u32 = 0x008b_7360;
const ACTOR_PREPARE: u32 = 0x008c_2e50;
const REFERENCE_TEST_00936EF0: u32 = 0x0093_6ef0;
/// `GetCurrentThreadId` through the game's wrapper (`0040fc90`) and the
/// getter of the thread id (`0044edb0`, `this + 0x10`).
const CURRENT_THREAD_ID: u32 = 0x0040_fc90;
const OWNER_THREAD_ID: u32 = 0x0044_edb0;
/// A `float -> float` helper of `effectsetting.cpp`'s range (`00408820`).
const FLOAT_FILTER: u32 = 0x0040_8820;
/// `__thiscall()` on a reference-counted object (`00401970`): decrements the
/// count at +4 and calls the object's virtual function 1 when it reaches 0.
const OBJECT_RELEASE: u32 = 0x0040_1970;

/// Source lines of the memory-context guards of `QueuedCharacter::
/// QueueModels`, `BackgroundClone` and `FinishAttach`, `QueuedCreature::
/// QueueModels`, `QueuedPlayer::QueueModels` and `QueuedFileLoad::Run`.
const CHARACTER_QUEUE_MODELS_SOURCE_LINE: u32 = 0x8ca;
const CHARACTER_BACKGROUND_CLONE_SOURCE_LINE: u32 = 0x8e4;
const CHARACTER_FINISH_ATTACH_SOURCE_LINE: u32 = 0x8ff;
const CREATURE_QUEUE_MODELS_SOURCE_LINE: u32 = 0x925;
const PLAYER_QUEUE_MODELS_SOURCE_LINE: u32 = 0x94e;
const FILE_LOAD_RUN_SOURCE_LINE: u32 = 0x976;
/// The virtual tables of `QueuedCreature` (`01016d34`), `QueuedPlayer`
/// (`01016d7c`), `QueuedFileLoad` (`01016dc4`), `QueuedFaceGenFile`
/// (`01016e1c`) and `BackgroundCloneThread` (`01016e50`).
const QUEUED_CREATURE_VTABLE: u32 = 0x0101_6d34;
const QUEUED_PLAYER_VTABLE: u32 = 0x0101_6d7c;
const QUEUED_FILE_LOAD_VTABLE: u32 = 0x0101_6dc4;
const QUEUED_FACE_GEN_FILE_VTABLE: u32 = 0x0101_6e1c;
const BACKGROUND_CLONE_THREAD_VTABLE: u32 = 0x0101_6e50;
/// `"BackgroundCloneThread"`, the thread's name.
const BACKGROUND_CLONE_THREAD_NAME: u32 = 0x0101_6e58;
/// `"MODELS: Could not get file for %s."`.
const FILE_LOAD_ERROR_FORMAT: u32 = 0x0101_6df4;
/// `"MODELS: ModelLoader still contains %d NIF files.\r\n"` and
/// `"ANIMATION: ModelLoader still has %d KF files.\r\n"`.
const MODEL_FILES_LEFT_FORMAT: u32 = 0x0101_6ea0;
const KF_FILES_LEFT_FORMAT: u32 = 0x0101_6e70;
/// The thread-local words at +0x260 and +0x264 of the TLS block that
/// `00441780` sets and `004417c0` clears.
const TLS_CLONE_WORD: u32 = 0x260;
const TLS_CLONE_REFERENCE: u32 = 0x264;
/// `TESNPC::CreateBipedAnim` (Xbox PDB, `006055f0`, `__thiscall(reference)`
/// on the base form), `00606540(reference, biped, 0)` on the base form (no
/// name in the engine map), `004abd30(priority, slot, task)` on the biped
/// animation (no name in the engine map), `00444fe0(base form, slot,
/// priority, task)` on the loader (no name in the engine map),
/// `ModelLoader::QueueAnimations` (Xbox PDB, `00445a10`, `__thiscall(slot,
/// priority, task, reference, flag, flag)` on the loader) and
/// `ModelLoader::QueueCreatureParts` (Xbox PDB, `004457d0`, `__thiscall(slot,
/// priority, task, reference)` on the loader).
const CREATE_BIPED_ANIM: u32 = 0x0060_55f0;
const NPC_SET_BIPED_ANIM: u32 = 0x0060_6540;
const BIPED_QUEUE_FILES: u32 = 0x004a_bd30;
const MODEL_LOADER_QUEUE_HEAD: u32 = 0x0044_4fe0;
const MODEL_LOADER_QUEUE_ANIMATIONS: u32 = 0x0044_5a10;
const MODEL_LOADER_QUEUE_CREATURE_PARTS: u32 = 0x0044_57d0;
/// `00483710` (an empty function of `tesform.cpp`'s range, `__thiscall()`),
/// the getter of the `NiPointer<NiAVObject>` at +0x34 of a reference task
/// (`005d8710`: `this + 0x34` dereferenced), `005d9f90` on a reference (no
/// name in the engine map), the head's attach `QueuedHead` method
/// (`0043ed90(reference, argument)`) and `QueuedHelmet::Attach` (Xbox PDB,
/// `0043fb80`).
const FINISH_ATTACH_BASE: u32 = 0x0048_3710;
const GET_CLONED_3D: u32 = 0x005d_8710;
const REFERENCE_QUERY_005D9F90: u32 = 0x005d_9f90;
const HEAD_ATTACH: u32 = 0x0043_ed90;
const HELMET_ATTACH: u32 = 0x0043_fb80;
/// `MobileObject::GetCurrentPackage` (Xbox PDB, `009344a0`),
/// `TESActorBase::ShouldRunInitDefaultWorn` (Xbox PDB, `005f1590`),
/// `TESCreature::InitWorn` (Xbox PDB, `005fa0a0`) and `TESCreature::
/// InitDefaultWorn` (Xbox PDB, `005f9e00`).
const GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
const SHOULD_RUN_INIT_DEFAULT_WORN: u32 = 0x005f_1590;
const CREATURE_INIT_WORN: u32 = 0x005f_a0a0;
const CREATURE_INIT_DEFAULT_WORN: u32 = 0x005f_9e00;
/// The method `0095f210(priority, task)` of `playercharacter.cpp` that
/// `QueuedPlayer::QueueModels` runs on the player reference.
const PLAYER_QUEUE_FILES: u32 = 0x0095_f210;
/// The bit of the dword at +0x1C of a package that `00441b00` tests.
const PACKAGE_FLAG_MASK: u32 = 0x0020_0000;
/// `QueuedFileEntry` functions (`queuedfiles.cpp`): the constructor
/// (`00c3ce60`, `__thiscall(context)`), the destructor (`00c3cea0`), the
/// file-name setter (`00c3cee0`, copies the name), the file-index setter
/// (`00c3cf60`, finds the file entry for the index) and `QueuedFileEntry::
/// GetFile` (Xbox PDB, `00c3cff0`, `__thiscall(index, 0xffff)`).
const QUEUED_FILE_ENTRY_CONSTRUCT: u32 = 0x00c3_ce60;
const QUEUED_FILE_ENTRY_DESTRUCT: u32 = 0x00c3_cea0;
const QUEUED_FILE_ENTRY_SET_NAME: u32 = 0x00c3_cee0;
const QUEUED_FILE_ENTRY_SET_INDEX: u32 = 0x00c3_cf60;
const QUEUED_FILE_ENTRY_GET_FILE: u32 = 0x00c3_cff0;
/// `NiPointer<LoadedFile>` functions: `NiPointer(file)` (`0044b0c0`),
/// `operator=(file)` (`0044b220`) and `~NiPointer` (`0044b160`).
const LOADED_FILE_POINTER_CONSTRUCT: u32 = 0x0044_b0c0;
const LOADED_FILE_POINTER_ASSIGN: u32 = 0x0044_b220;
const LOADED_FILE_POINTER_DESTRUCT: u32 = 0x0044_b160;
/// `NiPointer<BSFaceGenModelMeshData>` functions: `NiPointer(data)`
/// (`0044b270`), `operator=(data)` (`0044b300`) and `~NiPointer`
/// (`0044b2c0`).
const MESH_DATA_POINTER_CONSTRUCT: u32 = 0x0044_b270;
const MESH_DATA_POINTER_ASSIGN: u32 = 0x0044_b300;
const MESH_DATA_POINTER_DESTRUCT: u32 = 0x0044_b2c0;
/// The `IO_FILE_INDEX` of a file kind (`00449050`, `__cdecl(kind)`).
const FILE_INDEX_OF_KIND: u32 = 0x0044_9050;
/// The getter of the file name of a `QueuedFileEntry` (`0045cd60`, the dword
/// at +0x28) and the loader's loaded-file lookup (`004483a0(name)` on the
/// loader, named `LoadedFile::IncRefCount` in the engine map).
const QUEUED_FILE_ENTRY_GET_NAME: u32 = 0x0045_cd60;
const MODEL_LOADER_FIND_LOADED_FILE: u32 = 0x0044_83a0;
/// The log function, `__cdecl(format, ...)` (`005b5e40`).
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// `NiAlloc`, `__cdecl(size)` (`00aa13e0`).
const NI_ALLOC: u32 = 0x00aa_13e0;
/// `BSTaskThread::BSTaskThread` (Xbox PDB, `00c42dd0`, `__thiscall(3,
/// name)`) and the `BSTaskThread` destructor body (`00c42f00`).
const TASK_THREAD_CONSTRUCT: u32 = 0x00c4_2dd0;
const TASK_THREAD_DESTRUCT: u32 = 0x00c4_2f00;
/// The queue of the background clone thread: constructor
/// `0044b350(argument, 8)` on a 0x40 byte block, and the pop `00449280(slot)`
/// on the queue that fills a task pointer.
const CLONE_QUEUE_CONSTRUCT: u32 = 0x0044_b350;
const CLONE_QUEUE_POP: u32 = 0x0044_9280;
/// Methods of the background clone thread: wait for work (`004424c0`), the
/// two calls around a task (`00442510`, `00442530`) and the wake call
/// (`00442600`).
const THREAD_WAIT_FOR_WORK: u32 = 0x0044_24c0;
const THREAD_BEFORE_TASK: u32 = 0x0044_2510;
const THREAD_AFTER_TASK: u32 = 0x0044_2530;
const THREAD_WAKE: u32 = 0x0044_2600;
/// `InterlockedIncrement` through the game's wrapper, `__cdecl(address)`
/// (`0040b460`).
const INTERLOCKED_INCREMENT: u32 = 0x0040_b460;
/// An empty function (`0040fbe0`, named `Error` in the engine map).
const EMPTY_FUNCTION: u32 = 0x0040_fbe0;
/// `BSTask` state getter (`0084e3a0`, the dword at +0xC) and the function
/// `00449190(from, to)` that changes the state when it is `from` (true when
/// it did).
const TASK_GET_STATE: u32 = 0x0084_e3a0;
const TASK_CHANGE_STATE: u32 = 0x0044_9190;
/// The virtual function of a task that says whether it needs post processing.
const TASK_NEEDS_POST_PROCESS_SLOT: u32 = 0x38;
/// `IOManager::AddPostProcessTask` (Xbox PDB, `0043d690`, `__thiscall(task)`)
/// and the `LoadedFile` constructor (`0043baf0(name, file)`).
const IOMANAGER_ADD_POST_PROCESS_TASK: u32 = 0x0043_d690;
const LOADED_FILE_CONSTRUCT: u32 = 0x0043_baf0;
/// `ModelLoader::OutputModelMapContents` (Xbox PDB, `00443270`,
/// `__thiscall(0, 0)`).
const OUTPUT_MODEL_MAP_CONTENTS: u32 = 0x0044_3270;
/// Functions of the lock-free maps the loader owns: the iterator constructors
/// of the model map (`004498e0`) and of the KF map (`00449960`), the iterator
/// end test (`006ebde0`), `next(iterator, &key, &value, 1)` (`0044c640`, on
/// the map), the iterator destructors (`004431d0` after the model map,
/// `00443220` after the KF map), the KF model reference count (`004431b0`),
/// the deleting destructors of a model (`004431f0`) and of a KF model
/// (`00443240`), and two tests on the animation of a KF model (`005585e0`
/// and `005f26c0`).
const MODEL_ITERATOR_CONSTRUCT: u32 = 0x0044_98e0;
const KF_ITERATOR_CONSTRUCT: u32 = 0x0044_9960;
const ITERATOR_AT_END: u32 = 0x006e_bde0;
const MAP_NEXT: u32 = 0x0044_c640;
const MODEL_ITERATOR_DESTRUCT: u32 = 0x0044_31d0;
const KF_ITERATOR_DESTRUCT: u32 = 0x0044_3220;
const KF_MODEL_REF_COUNT: u32 = 0x0044_31b0;
const MODEL_DELETE: u32 = 0x0044_31f0;
const KF_MODEL_DELETE: u32 = 0x0044_3240;
const KF_MODEL_ANIMATION: u32 = 0x0055_85e0;
const ANIMATION_TEST: u32 = 0x005f_26c0;
/// The size of an iterator the destructor keeps on its stack.
const MAP_ITERATOR_SIZE: u32 = 0x114;
/// The constructors of the loader's maps and queue, `__thiscall` on a block
/// of 0x40 bytes: `pModelMap` `004492c0(10, 0x3f1, 0xc)`, `pKFModelMap`
/// `004496a0(10, 0x3f1, 0xc)`, `pQueuedReferencesMap` and
/// `pReferencesToQueueMap` `0044b750(10, 0x13af, 0xc)`, `pQueuedAnimIdleMap`
/// `0044ba10(10, 0xfb, 0xc)`, `pQueuedReplacementKFListMap`
/// `0044bbe0(10, 0xfb, 0xc)`, `pQueuedHelmetMap` `0044be80(10, 0xfb, 0xc)`,
/// `pAttachDistant3DTaskQueue` `0044b500(10, 8)`, `pQueuedTextureMap`
/// `0044c760(10, 0x3f1, 0xc)` and `pLoadedFileMap` `004497c0(10, 0x3f1,
/// 0xc)`.
const MODEL_MAP_CONSTRUCT: u32 = 0x0044_92c0;
const KF_MODEL_MAP_CONSTRUCT: u32 = 0x0044_96a0;
const REFERENCE_MAP_CONSTRUCT: u32 = 0x0044_b750;
const ANIM_IDLE_MAP_CONSTRUCT: u32 = 0x0044_ba10;
const REPLACEMENT_KF_LIST_MAP_CONSTRUCT: u32 = 0x0044_bbe0;
const HELMET_MAP_CONSTRUCT: u32 = 0x0044_be80;
const ATTACH_TASK_QUEUE_CONSTRUCT: u32 = 0x0044_b500;
const TEXTURE_MAP_CONSTRUCT: u32 = 0x0044_c760;
const LOADED_FILE_MAP_CONSTRUCT: u32 = 0x0044_97c0;
/// `00c42f50` on the background clone thread (starts the thread) and the
/// registrations the loader makes on the IO manager when there is one, each
/// `__thiscall(callback)`: `0057bd60`, `004febb0`, the setter [`fn_00442a80`],
/// `004fbf00`, `004febd0` and `008ac890`. The callbacks are the functions of
/// the loader at `00446e30`, `00446e40`, `00446ea0`, `00446f00`, `00446f10`
/// and `00446f90`.
const THREAD_START: u32 = 0x00c4_2f50;
const IO_MANAGER_REGISTER_FIRST: u32 = 0x0057_bd60;
const IO_MANAGER_REGISTER_SECOND: u32 = 0x004f_ebb0;
const IO_MANAGER_REGISTER_FOURTH: u32 = 0x004f_bf00;
const IO_MANAGER_REGISTER_FIFTH: u32 = 0x004f_ebd0;
const IO_MANAGER_REGISTER_SIXTH: u32 = 0x008a_c890;
const LOADER_CALLBACK_00446E30: u32 = 0x0044_6e30;
const LOADER_CALLBACK_00446E40: u32 = 0x0044_6e40;
const LOADER_CALLBACK_00446EA0: u32 = 0x0044_6ea0;
const LOADER_CALLBACK_00446F00: u32 = 0x0044_6f00;
const LOADER_CALLBACK_00446F10: u32 = 0x0044_6f10;
const LOADER_CALLBACK_00446F90: u32 = 0x0044_6f90;

layout! {
    /// `QueuedFileLoad` (Xbox PDB), 0x38 bytes (the same on PC): a
    /// `QueuedFileEntry` and the file it loads.
    pub struct QueuedFileLoad: 0x38 {
        /// `eContext` (`QueuedFile`, Xbox PDB).
        0x18 eContext: u32,
        /// `pFileName` (`QueuedFileEntry`, Xbox PDB): `char*`.
        0x28 pFileName: u32,
        /// `pFileEntry` (`QueuedFileEntry`, Xbox PDB): `BSFileEntry*`.
        0x2C pFileEntry: Ptr,
        /// `spLoadedFile` (Xbox PDB): `NiPointer<LoadedFile>`.
        0x30 spLoadedFile: Ptr,
        /// `eFileIndex` (Xbox PDB): `IO_FILE_INDEX`.
        0x34 eFileIndex: u32,
    }

    /// `QueuedFaceGenFile` (Xbox PDB), 0x40 bytes (the same on PC): a
    /// `QueuedFileLoad` and the face model it loads.
    pub struct QueuedFaceGenFile: 0x40 {
        /// `spLoadedFile` (`QueuedFileLoad`, Xbox PDB).
        0x30 spLoadedFile: Ptr,
        /// `eFileIndex` (`QueuedFileLoad`, Xbox PDB).
        0x34 eFileIndex: u32,
        /// `spFaceGenModel` (Xbox PDB): `NiPointer<BSFaceGenModel>`.
        0x38 spFaceGenModel: Ptr,
        /// `spModelMeshData` (Xbox PDB):
        /// `NiPointer<BSFaceGenModelMeshData>`.
        0x3C spModelMeshData: Ptr,
    }

    /// `BackgroundCloneThread` (Xbox PDB), 0x3c bytes (the same on PC): a
    /// `BSTaskThread` (0x30 bytes) and these.
    pub struct BackgroundCloneThread: 0x3c {
        /// `bExit` (Xbox PDB): the thread's loop ends when it is 1.
        0x30 bExit: u8,
        /// `iRunningCount` (Xbox PDB): changed with interlocked operations.
        0x34 iRunningCount: i32,
        /// `pCloneReferencesQueue` (Xbox PDB):
        /// `LockFreeQueue<NiPointer<QueuedReference> >*`.
        0x38 pCloneReferencesQueue: Ptr,
    }

    /// `ModelLoader` (Xbox PDB), 0x30 bytes: the loader's maps and queues.
    pub struct ModelLoader: 0x30 {
        /// `pModelMap` (Xbox PDB):
        /// `LockFreeCaseInsensitiveStringMap<Model *>*`.
        0x00 pModelMap: Ptr,
        /// `pKFModelMap` (Xbox PDB):
        /// `LockFreeCaseInsensitiveStringMap<KFModel *>*`.
        0x04 pKFModelMap: Ptr,
        /// `pQueuedReferencesMap` (Xbox PDB).
        0x08 pQueuedReferencesMap: Ptr,
        /// `pReferencesToQueueMap` (Xbox PDB).
        0x0C pReferencesToQueueMap: Ptr,
        /// `pQueuedAnimIdleMap` (Xbox PDB).
        0x10 pQueuedAnimIdleMap: Ptr,
        /// `pQueuedReplacementKFListMap` (Xbox PDB).
        0x14 pQueuedReplacementKFListMap: Ptr,
        /// `pQueuedHelmetMap` (Xbox PDB).
        0x18 pQueuedHelmetMap: Ptr,
        /// `pAttachDistant3DTaskQueue` (Xbox PDB).
        0x1C pAttachDistant3DTaskQueue: Ptr,
        /// `pQueuedTextureMap` (Xbox PDB).
        0x20 pQueuedTextureMap: Ptr,
        /// `pLoadedFileMap` (Xbox PDB).
        0x24 pLoadedFileMap: Ptr,
        /// `pBackgroundCloneThread` (Xbox PDB).
        0x28 pBackgroundCloneThread: Ptr,
        /// `bHasDelayedFree` (Xbox PDB).
        0x2C bHasDelayedFree: u8,
    }
}

layout! {
    /// `QueuedCharacter` (Xbox PDB), 0x48 bytes: a `QueuedActor` (the same
    /// 0x40 bytes as `QueuedReference`) and the head and helmet tasks.
    pub struct QueuedCharacter: 0x48 {
        /// `eContext` (`QueuedFile`, Xbox PDB).
        0x18 eContext: u32,
        /// `pRef` (`QueuedReference`, Xbox PDB): `TESObjectREFR*`.
        0x28 pRef: Ptr,
        /// `spQueuedHead` (Xbox PDB): `NiPointer<QueuedHead>`.
        0x40 spQueuedHead: Ptr,
        /// `spQueuedHelmet` (Xbox PDB): `NiPointer<QueuedHelmet>`.
        0x44 spQueuedHelmet: Ptr,
    }
}

/// Runs `body` inside the memory-context guard the game keeps on its stack
/// (`context`, a source line of `ModelLoader.cpp`), leaving it afterwards.
fn in_context<R>(
    e: &mut Engine,
    context: u32,
    line: u32,
    body: impl FnOnce(&mut Engine) -> R,
) -> R {
    e.with_stack(4, |e, guard| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![guard, context, 1u32, MODEL_LOADER_SOURCE, line],
        );
        let result = body(e);
        e.call(MEMORY_CONTEXT_LEAVE, &args![guard]);
        result
    })
}

/// The `NiPointer` stored at `slot`, dereferenced (`00559450`).
fn ni_pointer_value(e: &mut Engine, slot: Ptr) -> u32 {
    e.call(NI_POINTER_GET, &args![slot]).u32()
}

/// The priority byte of a task (`0043cc60`), as the word the game pushes.
fn task_priority(e: &mut Engine, task: Ptr) -> u32 {
    fn_0043cc60(e, task.cast()) as u32
}

// Translated from 0043fed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReference::QueueMe` (Xbox PDB): inside the memory context of the
/// task (source line `0x72a`), a reference that passes `00564e60` and whose
/// virtual function `0x1d0` gives 0 makes the task call its virtual function
/// `0x30` (`UseDistant3D`). Then every item of the reference's decal list
/// (`ExtraDataList::GetDecalRefs` of its extra data) whose form has type 4 is
/// queued as a texture set (`BGSTextureSet::QueueTextureSet`) with the task's
/// priority. Last the task calls its virtual function `0x2c` (`QueueModels`);
/// a task still in state 0 is then put into state 5 (`00449150`), and the
/// virtual function `0x28` (`CheckFinished`) runs.
///
/// The exception-unwinding frame is not translated.
pub fn queued_reference_queue_me(e: &mut Engine, this: Ptr<QueuedReference>) {
    let context = e.get(this, QueuedReference::eContext);
    in_context(e, context, QUEUE_ME_SOURCE_LINE, |e| {
        let reference = e.get(this, QueuedReference::pRef);
        if !reference.is_null()
            && e.call(REFERENCE_TEST_00564E60, &args![reference]).bool()
            && e.vcall(reference.addr(), 0x1d0, &args![]).u32() == 0
        {
            e.vcall(this.addr(), 0x30, &args![]);
        }
        let reference = e.get(this, QueuedReference::pRef);
        if !reference.is_null() {
            let extra = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
            let mut node = e.call(EXTRA_GET_DECAL_REFS, &args![extra]).u32();
            while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
                let item_slot = e.call(LIST_NODE_ITEM_ADDRESS, &args![node]).u32();
                let item = e.mem.u32(item_slot);
                node = e.call(LIST_NODE_NEXT, &args![node]).u32();
                if e.mem.u32(item) != 0 {
                    let holder = e.mem.u32(item);
                    let form = e.call(GET_BASE_FORM, &args![holder]).u32();
                    if e.call(FORM_TYPE, &args![form]).u32() == 4 {
                        let holder = e.mem.u32(item);
                        let texture_set = e.call(GET_BASE_FORM, &args![holder]).u32();
                        let priority = fn_0043cc80(e, this.cast()) as u8 as u32;
                        e.call(QUEUE_TEXTURE_SET, &args![texture_set, priority, this, 0u32]);
                    }
                }
            }
        }
        e.vcall(this.addr(), 0x2c, &args![]);
        if e.call(TASK_STATE_IS_0, &args![this]).bool() {
            e.call(TASK_SET_STATE_5, &args![this]);
        }
        e.vcall(this.addr(), 0x28, &args![]);
    });
}

// Translated from 00440050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReference::QueueModels` (Xbox PDB): inside the memory context of the
/// task (source line `0x752`), a reference whose `TESModel`
/// (`TESObjectREFR::GetTESModel`) has a non-empty name (virtual function
/// `0x14`) has its model queued by the loader (`00444dc0` with the base form,
/// the `spQueuedModel` slot, the task's priority, the task and the
/// reference). When that left `spQueuedModel` empty the loader's `FindModel`
/// looks the model up by name into `spModel`. The base form's destruction
/// form (`BGSDestructibleObjectForm::GetDestructionForm`) has its files queued
/// (`QueueFiles`).
///
/// The exception-unwinding frame is not translated.
pub fn queued_reference_queue_models(e: &mut Engine, this: Ptr<QueuedReference>) {
    let context = e.get(this, QueuedReference::eContext);
    in_context(e, context, QUEUE_MODELS_SOURCE_LINE, |e| {
        let reference = e.get(this, QueuedReference::pRef);
        let form = e.call(GET_BASE_FORM, &args![reference]).u32();
        let tes_model = e.call(REFERENCE_GET_TES_MODEL, &args![reference]).u32();
        if tes_model == 0 {
            return;
        }
        let name = e.vcall(tes_model, 0x14, &args![]).u32();
        if e.mem.i8(name) == 0 {
            return;
        }
        let reference = e.get(this, QueuedReference::pRef);
        let priority = task_priority(e, this.cast());
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(
            MODEL_LOADER_QUEUE_REFERENCE_MODEL,
            &args![
                loader,
                form,
                this.byte_add(QueuedReference::spQueuedModel.off),
                priority,
                this,
                reference
            ],
        );
        if ni_pointer_value(e, this.byte_add(QueuedReference::spQueuedModel.off)) == 0 {
            let name = e.vcall(tes_model, 0x14, &args![]).u32();
            let loader = e.global::<u32>(MODEL_LOADER);
            e.call(
                MODEL_LOADER_FIND_MODEL,
                &args![loader, name, this.byte_add(QueuedReference::spModel.off)],
            );
        }
        let destruction = e.call(GET_DESTRUCTION_FORM, &args![form]).u32();
        if destruction != 0 {
            let priority = task_priority(e, this.cast());
            e.call(
                DESTRUCTIBLE_QUEUE_FILES,
                &args![destruction, form, priority, this],
            );
        }
    });
}

// Translated from 00440190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReference::UseDistant3D` (Xbox PDB): looks up the loader's model
/// for the base form's distant model name (`TESModel::GetDistantModelName_ov2`
/// into a 0x108-byte buffer, then `ModelLoader::FindModel`); if there is one
/// with a root node, the node is cloned (`NiObject::Clone_ov2`). When the
/// clone's virtual function `0xc` gives a node (the fade node), and its
/// virtual function `0x10` is non-zero, its LOD multiplier type is set to 6;
/// a clone without such a node is only given a counted reference and let go.
/// A fade node is handed to the task's virtual function `0x34`
/// (`AttachDistant3D`). The model pointer is released at the end.
///
/// The exception-unwinding frame and the stack-cookie check are not
/// translated.
pub fn queued_reference_use_distant_3d(e: &mut Engine, this: Ptr<QueuedReference>) {
    let reference = e.get(this, QueuedReference::pRef);
    let form = e.call(GET_BASE_FORM, &args![reference]).u32();
    e.with_stack(0x108, |e, name| {
        e.call(GET_DISTANT_MODEL_NAME, &args![name, form]);
        // The `NiPointer<Model>` of the game's stack frame.
        e.with_stack(4, |e, model| {
            e.call(NI_POINTER_CONSTRUCT, &args![model, 0u32]);
            let loader = e.global::<u32>(MODEL_LOADER);
            e.call(MODEL_LOADER_FIND_MODEL, &args![loader, name, model]);
            let mut fade_node = 0;
            let found = ni_pointer_value(e, model);
            if found != 0 {
                let root = fn_0043b230(e, Ptr::new(found));
                if !root.is_null() {
                    let clone = e.call(NI_OBJECT_CLONE, &args![root]).u32();
                    if clone != 0 {
                        fade_node = e.vcall(clone, 0xc, &args![]).u32();
                        if fade_node == 0 {
                            // A temporary `NiPointer` takes the clone and
                            // releases it again.
                            e.with_stack(4, |e, temporary| {
                                e.call(NI_POINTER_CONSTRUCT, &args![temporary, clone]);
                                e.call(NI_POINTER_DESTRUCT, &args![temporary]);
                            });
                        } else if e.vcall(fade_node, 0x10, &args![]).u32() != 0 {
                            e.call(FADE_NODE_SET_LOD_MULT_TYPE, &args![fade_node, 6u32]);
                        }
                    }
                }
            }
            if fade_node != 0 {
                e.vcall(this.addr(), 0x34, &args![fade_node]);
            }
            e.call(MODEL_POINTER_RELEASE, &args![model]);
        });
    });
}

// Translated from 00440310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReference::AttachDistant3D` (Xbox PDB): gives the node the
/// reference's position (virtual function `0x1f4` of the reference, stored
/// by [`fn_00440460`]), orientation (`TESObjectREFR::GetOrientation` into a
/// 12-byte buffer, copied by `0043fa80`) and scale (`GetScale`, filtered by
/// [`fn_00440490`]). When the current thread is the one recorded at +0x10 of
/// the object at global `011dea0c`, the node is attached at once: the
/// reference gets it as its 3D (`00570f70`), `TES` loads the reference
/// (`00451ef0` with its parent cell) and `TESActorBase::SetStartsDead(1)`
/// is called. Otherwise a new 0x20-byte `AttachDistant3DTask`
/// ([`fn_004404c0`]) for the reference and node is stored in
/// `spAttachDistant3DTask` and handed to the loader ([`fn_00440710`]).
///
/// The exception-unwinding frame is not translated.
pub fn queued_reference_attach_distant_3d(e: &mut Engine, this: Ptr<QueuedReference>, node: Ptr) {
    let reference = e.get(this, QueuedReference::pRef);
    let position = e.vcall(reference.addr(), 0x1f4, &args![]).u32();
    fn_00440460(e, node, Ptr::new(position));
    // The 12-byte orientation buffer of the game's stack frame.
    e.with_stack(12, |e, buffer| {
        let orientation = e
            .call(REFERENCE_GET_ORIENTATION, &args![reference, buffer])
            .u32();
        fn_0043fa80(e, node, Ptr::new(orientation));
    });
    let scale = e.call(REFERENCE_GET_SCALE, &args![reference]).f32();
    fn_00440490(e, node, scale);
    let owner = e.global::<u32>(THREAD_OWNER_OBJECT);
    let owner_thread = e.call(OWNER_THREAD_ID, &args![owner]).u32();
    let current_thread = e.call(CURRENT_THREAD_ID, &args![]).u32();
    if owner_thread == current_thread {
        e.call(REFERENCE_SET_3D, &args![reference, node]);
        let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
        let tes = e.global::<u32>(TES_GLOBAL);
        e.call(TES_LOAD_REFERENCE, &args![tes, reference, cell, 0u32, 0u32]);
        e.call(SET_STARTS_DEAD, &args![reference, 1u32]);
    } else {
        let block = e.call(MEMORY_ALLOC, &args![0x20u32]).u32();
        let task = if block != 0 {
            fn_004404c0(e, Ptr::new(block), reference, node).addr()
        } else {
            0
        };
        let slot = this.byte_add(QueuedReference::spAttachDistant3DTask.off);
        e.call(TASK_POINTER_ASSIGN, &args![slot, task]);
        let task = ni_pointer_value(e, slot);
        let loader = e.global::<u32>(MODEL_LOADER);
        fn_00440710(e, Ptr::new(loader), Ptr::new(task));
    }
}

// Translated from 00440460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject` method (no name in the engine map): copies the three words
/// at `source` (a position) into the local translation at `this + 0x58`.
pub fn fn_00440460(e: &mut Engine, this: Ptr, source: Ptr) {
    for i in 0..3 {
        let word = e.mem.u32(source.addr() + 4 * i);
        e.mem.set_u32(this.addr() + 0x58 + 4 * i, word);
    }
}

// Translated from 00440490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiAVObject` method (no name in the engine map): stores `scale` passed
/// through the `float` helper `00408820` into the local scale at
/// `this + 0x64`.
pub fn fn_00440490(e: &mut Engine, this: Ptr, scale: f32) {
    let filtered = e.call(FLOAT_FILTER, &args![scale]).f32();
    e.mem.set_f32(this.addr() + 0x64, filtered);
}

// Translated from 004404c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AttachDistant3DTask` constructor (no name in the engine map): the
/// `IOTask` constructor with priority 0 ([`fn_00440540`]), the task's virtual
/// table, `pRef`, and `spNode` set to `node`. Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_004404c0(
    e: &mut Engine,
    this: Ptr<AttachDistant3DTask>,
    reference: Ptr,
    node: Ptr,
) -> Ptr<AttachDistant3DTask> {
    fn_00440540(e, this.cast(), 0);
    e.mem.set_u32(this.addr(), ATTACH_DISTANT_3D_TASK_VTABLE);
    e.set(this, AttachDistant3DTask::pRef, reference);
    e.call(
        NI_POINTER_CONSTRUCT,
        &args![this.byte_add(AttachDistant3DTask::spNode.off), node],
    );
    this
}

// Translated from 00440540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IOTask` constructor taking a priority (no name in the engine map): the
/// base constructor (`00449090`), the `IOTask` virtual table, then
/// [`fn_004405b0`] with `priority`. Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_00440540(e: &mut Engine, this: Ptr<IOTask>, priority: u32) -> Ptr<IOTask> {
    e.call(IO_TASK_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), IO_TASK_VTABLE);
    fn_004405b0(e, this, priority as u8);
    this
}

// Translated from 004405b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IOTask` method (no name in the engine map): [`fn_004405d0`] with the
/// priority byte.
pub fn fn_004405b0(e: &mut Engine, this: Ptr<IOTask>, priority: u8) {
    fn_004405d0(e, this, priority);
}

// Translated from 004405d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IOTask` method (no name in the engine map): puts `priority` into bits 16
/// to 23 of the task's 64-bit `Key`: the key with the bits `0x00ff0000` of
/// its low dword cleared, plus `priority << 16` (a 64-bit add).
pub fn fn_004405d0(e: &mut Engine, this: Ptr<IOTask>, priority: u8) {
    let key = e.get(this, IOTask::Key);
    let cleared = key & 0xffff_ffff_ff00_ffff;
    e.set(
        this,
        IOTask::Key,
        cleared.wrapping_add((priority as u64) << 16),
    );
}

// Translated from 00440610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IOTask` scalar deleting destructor (no name in the engine map): runs the
/// destructor ([`fn_00440640`]) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn fn_00440610(e: &mut Engine, this: Ptr<IOTask>, flags: u32) -> Ptr<IOTask> {
    fn_00440640(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 00440640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IOTask` destructor (no name in the engine map): the base destructor
/// (`004490e0`).
pub fn fn_00440640(e: &mut Engine, this: Ptr<IOTask>) {
    e.call(IO_TASK_BASE_DESTRUCT, &args![this]);
}

// Translated from 00440660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IOTask::Requeue` (Xbox PDB): `IOManager::RequeueTask` on the IO manager
/// (the global at `01202d98`) with the task and `requeue_argument`.
pub fn io_task_requeue(e: &mut Engine, this: Ptr<IOTask>, requeue_argument: u32) {
    let manager = e.global::<u32>(TASK_QUEUE);
    e.call(
        IO_MANAGER_REQUEUE_TASK,
        &args![manager, this, requeue_argument],
    );
}

// Translated from 00440680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AttachDistant3DTask::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor ([`fn_004406b0`]) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn attach_distant_3d_task_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<AttachDistant3DTask>,
    flags: u32,
) -> Ptr<AttachDistant3DTask> {
    fn_004406b0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 004406b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AttachDistant3DTask` destructor (no name in the engine map): releases
/// `spNode` and runs the `IOTask` destructor ([`fn_00440640`]). It does not
/// restore the virtual table.
///
/// The exception-unwinding frame is not translated.
pub fn fn_004406b0(e: &mut Engine, this: Ptr<AttachDistant3DTask>) {
    e.call(
        NI_POINTER_DESTRUCT,
        &args![this.byte_add(AttachDistant3DTask::spNode.off)],
    );
    fn_00440640(e, this.cast());
}

// Translated from 00440710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map) on the loader at
/// `this`: puts `task` into a task pointer of the game's stack frame and
/// hands it to the object at `this + 0x1c` (`00449240`).
///
/// The exception-unwinding frame is not translated.
pub fn fn_00440710(e: &mut Engine, this: Ptr, task: Ptr) {
    e.with_stack(4, |e, holder| {
        e.call(TASK_POINTER_CONSTRUCT, &args![holder, task]);
        let queue = e.mem.u32(this.addr() + 0x1c);
        e.call(FINISHED_QUEUE_ADD, &args![queue, holder]);
        e.call(TASK_POINTER_DESTRUCT, &args![holder]);
    });
}

// Translated from 00440780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReference::CheckFinished` (Xbox PDB): once the task's state is 4 or
/// more but not 6 and its children are all finished ([`fn_0043caa0`]): a task
/// that has a queued model but no model takes the queued model's
/// (`004a8a90`). It is then attached by the loader (`00442580` through
/// [`fn_004408d0`]) when it has a model, its reference is not the one at
/// global `011dea3c`, the reference's parent cell is loaded (`TES::
/// IsCellLoaded`), `00451e40` accepts the reference and [`fn_0043fcd0`] finds
/// none; except that a reference whose virtual functions `0x224` or `0x220`
/// answer yes is never attached this way. All other tasks go to the
/// post-process queue ([`iomanager_add_post_process_task`]).
pub fn queued_reference_check_finished(e: &mut Engine, this: Ptr<QueuedReference>) {
    if !(e.call(TASK_STATE_AT_LEAST_4, &args![this]).bool()
        && fn_0043caa0(e, this.cast())
        && !e.call(TASK_STATE_IS_6, &args![this]).bool())
    {
        return;
    }
    let queued_slot = this.byte_add(QueuedReference::spQueuedModel.off);
    let model_slot = this.byte_add(QueuedReference::spModel.off);
    if ni_pointer_value(e, queued_slot) != 0 && ni_pointer_value(e, model_slot) == 0 {
        let queued = ni_pointer_value(e, queued_slot);
        let model = e.call(QUEUED_MODEL_GET_MODEL, &args![queued]).u32();
        e.call(MODEL_POINTER_ASSIGN, &args![model_slot, model]);
    }
    let mut attach = false;
    let reference = e.get(this, QueuedReference::pRef);
    if ni_pointer_value(e, model_slot) != 0
        && reference.addr() != e.global::<u32>(SPECIAL_REFERENCE)
    {
        let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
        let tes = e.global::<u32>(TES_GLOBAL);
        if e.call(TES_IS_CELL_LOADED, &args![tes, cell, 0u32]).bool()
            && e.call(TES_REFERENCE_PREDICATE, &args![tes, reference])
                .bool()
            && fn_0043fcd0(e, reference).is_null()
        {
            attach = true;
        }
    }
    if e.vcall(reference.addr(), 0x224, &args![]).bool()
        || e.vcall(reference.addr(), 0x220, &args![]).bool()
    {
        attach = false;
    }
    if attach {
        let loader = e.global::<u32>(MODEL_LOADER);
        fn_004408d0(e, Ptr::new(loader), this.cast());
    } else {
        iomanager_add_post_process_task(e, this.cast());
    }
}

// Translated from 004408d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map) on the loader at `this`:
/// `00442580(task)` on the object at `this + 0x28`.
pub fn fn_004408d0(e: &mut Engine, this: Ptr, task: Ptr) {
    let queue = e.mem.u32(this.addr() + 0x28);
    e.call(BACKGROUND_CLONE_QUEUE_ADD, &args![queue, task]);
}

// Translated from 004408f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReference::Cancel` (Xbox PDB): a pending attach task is set to
/// state 6 (`00449170`), `QueuedFile::Cancel` runs with the two arguments,
/// and a cloned 3D is taken off the reference (`Set3DVerySimple`, the
/// `NiPointer` cleared, the reference's virtual function `0x1cc` called with
/// two zeros). The loader's object at `loader + 8` is told about the
/// reference ([`fn_004409e0`]); the parent cell's queued-reference count
/// is lowered ([`fn_00440a10`]), and, for a reference whose 3D is critical,
/// its critical count too ([`fn_00440a50`]).
pub fn queued_reference_cancel(
    e: &mut Engine,
    this: Ptr<QueuedReference>,
    first: u32,
    second: u32,
) {
    let attach_slot = this.byte_add(QueuedReference::spAttachDistant3DTask.off);
    let attach_task = ni_pointer_value(e, attach_slot);
    if attach_task != 0 {
        let attach_task = ni_pointer_value(e, attach_slot);
        e.call(TASK_SET_STATE_6, &args![attach_task]);
    }
    e.call(QUEUED_FILE_CANCEL, &args![this, first, second]);
    let cloned_slot = this.byte_add(QueuedReference::spCloned3D.off);
    let reference = e.get(this, QueuedReference::pRef);
    if ni_pointer_value(e, cloned_slot) != 0 {
        let cloned = ni_pointer_value(e, cloned_slot);
        e.call(REFERENCE_SET_3D_VERY_SIMPLE, &args![reference, cloned]);
        e.call(NI_POINTER_ASSIGN, &args![cloned_slot, 0u32]);
        e.vcall(reference.addr(), 0x1cc, &args![0u32, 0u32]);
    }
    let loader = e.global::<u32>(MODEL_LOADER);
    fn_004409e0(e, Ptr::new(loader), reference);
    let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
    if cell != 0 {
        let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
        fn_00440a10(e, Ptr::new(cell));
        if e.call(REFERENCE_IS_3D_CRITICAL, &args![reference]).bool() {
            let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
            fn_00440a50(e, Ptr::new(cell));
        }
    }
}

// Translated from 004409e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map) on the loader at `this`:
/// calls the virtual function `0x14` of the object at `this + 8` with
/// `reference`. What that slot does is not confirmed; the reference tasks
/// call it when they are over.
pub fn fn_004409e0(e: &mut Engine, this: Ptr, reference: Ptr) {
    let object = e.mem.u32(this.addr() + 8);
    e.vcall(object, 0x14, &args![reference]);
}

// Translated from 00440a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL` method (no name in the engine map): lowers
/// `iQueuedRefCount` (Xbox PDB, an interlocked decrement) and keeps it from
/// going below 0.
pub fn fn_00440a10(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    e.call(
        INTERLOCKED_DECREMENT,
        &args![this.byte_add(TESObjectCELL::iQueuedRefCount.off)],
    );
    if e.get(this, TESObjectCELL::iQueuedRefCount) < 0 {
        e.set(this, TESObjectCELL::iQueuedRefCount, 0);
    }
}

// Translated from 00440a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL` method (no name in the engine map): lowers
/// `iCriticalQueuedRefCount` (Xbox PDB, an interlocked decrement) and keeps
/// it from going below 0.
pub fn fn_00440a50(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    e.call(
        INTERLOCKED_DECREMENT,
        &args![this.byte_add(TESObjectCELL::iCriticalQueuedRefCount.off)],
    );
    if e.get(this, TESObjectCELL::iCriticalQueuedRefCount) < 0 {
        e.set(this, TESObjectCELL::iCriticalQueuedRefCount, 0);
    }
}

// Translated from 00440a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReference::BackgroundClone` (Xbox PDB): inside the memory context
/// of the task (source line `0x7ea`), when the reference's parent cell is
/// loaded (`TES::IsCellLoaded`) and `00451e40` accepts the reference and
/// [`fn_0043fcd0`] finds no object for it, the reference's virtual function
/// `0x1c8` (called with 1) gives the clone, which is stored in `spCloned3D`
/// and then released once (`00401970`, balancing the reference the virtual
/// function returned). Returns whether the cell was loaded.
///
/// The exception-unwinding frame is not translated.
pub fn queued_reference_background_clone(e: &mut Engine, this: Ptr<QueuedReference>) -> bool {
    let context = e.get(this, QueuedReference::eContext);
    in_context(e, context, BACKGROUND_CLONE_SOURCE_LINE, |e| {
        let reference = e.get(this, QueuedReference::pRef);
        let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
        let tes = e.global::<u32>(TES_GLOBAL);
        if !e.call(TES_IS_CELL_LOADED, &args![tes, cell, 0u32]).bool() {
            return false;
        }
        let tes = e.global::<u32>(TES_GLOBAL);
        if e.call(TES_REFERENCE_PREDICATE, &args![tes, reference])
            .bool()
            && fn_0043fcd0(e, reference).is_null()
        {
            let clone = e.vcall(reference.addr(), 0x1c8, &args![1u32]).u32();
            let cloned_slot = this.byte_add(QueuedReference::spCloned3D.off);
            e.call(NI_POINTER_ASSIGN, &args![cloned_slot, clone]);
            if ni_pointer_value(e, cloned_slot) != 0 {
                let cloned = ni_pointer_value(e, cloned_slot);
                e.call(OBJECT_RELEASE, &args![cloned]);
            }
        }
        true
    })
}

// Translated from 00440ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReference::Attach` (Xbox PDB): nothing happens for a task in state
/// 6. Otherwise, inside the memory context of the task (source line
/// `0x804`), a reference with form flag `0x800` or `0x20` ([`fn_00440da0`],
/// [`fn_00440d80`]) is left alone; any other reference whose parent cell is
/// loaded (`TES::IsCellLoaded`) gets its addon nodes loaded
/// (`ModelLoader::LoadAddonNodes` with the model's root node, when the task
/// has a model) and is loaded by `TES` (`00451ef0` with the task as source).
/// In both cases the loader's object at `loader + 8` is told about the
/// reference ([`fn_004409e0`]), and the parent cell's queued-reference count
/// is lowered (and its critical count, for a reference whose 3D is critical);
/// `00541ac0` is called on the parent cell at the start and `00541ae0` at the
/// end.
///
/// The exception-unwinding frame is not translated.
pub fn queued_reference_attach(e: &mut Engine, this: Ptr<QueuedReference>) {
    let context = e.get(this, QueuedReference::eContext);
    in_context(e, context, ATTACH_SOURCE_LINE, |e| {
        if e.call(TASK_STATE_IS_6, &args![this]).bool() {
            return;
        }
        let reference = e.get(this, QueuedReference::pRef);
        let skip = fn_00440da0(e, reference.cast()) || fn_00440d80(e, reference.cast());
        let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
        if cell != 0 {
            let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
            e.call(CELL_ENTER_ATTACH, &args![cell]);
        }
        if !skip {
            let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
            let tes = e.global::<u32>(TES_GLOBAL);
            if e.call(TES_IS_CELL_LOADED, &args![tes, cell, 0u32]).bool() {
                let model_slot = this.byte_add(QueuedReference::spModel.off);
                if ni_pointer_value(e, model_slot) != 0 {
                    let model = ni_pointer_value(e, model_slot);
                    let node = fn_0043b230(e, Ptr::new(model)).addr();
                    let loader = e.global::<u32>(MODEL_LOADER);
                    e.call(
                        MODEL_LOADER_LOAD_ADDON_NODES,
                        &args![loader, reference, node],
                    );
                }
                let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
                let tes = e.global::<u32>(TES_GLOBAL);
                e.call(TES_LOAD_REFERENCE, &args![tes, reference, cell, this, 0u32]);
            }
        }
        let loader = e.global::<u32>(MODEL_LOADER);
        fn_004409e0(e, Ptr::new(loader), reference);
        let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
        if cell != 0 {
            let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
            fn_00440a10(e, Ptr::new(cell));
            if e.call(REFERENCE_IS_3D_CRITICAL, &args![reference]).bool() {
                let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
                fn_00440a50(e, Ptr::new(cell));
            }
            let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
            e.call(CELL_LEAVE_ATTACH, &args![cell]);
        }
    });
}

// Translated from 00440d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm` method (no name in the engine map): whether bit `0x20` of
/// `iFormFlags` is set.
pub fn fn_00440d80(e: &mut Engine, this: Ptr<TESForm>) -> bool {
    e.get(this, TESForm::iFormFlags) & 0x20 != 0
}

// Translated from 00440da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm` method (no name in the engine map): whether bit `0x800` of
/// `iFormFlags` is set.
pub fn fn_00440da0(e: &mut Engine, this: Ptr<TESForm>) -> bool {
    e.get(this, TESForm::iFormFlags) & 0x800 != 0
}

// Translated from 00440dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReference::GetDescription` (Xbox PDB): for a task not in state 6
/// writes `"Queued ref '%s' (%08X) of type %s"` into `buffer` (`size` bytes)
/// and returns true: the first text is the result of the reference's virtual
/// function `0x130`, the number is its form ID (`0084e3a0`), and the last is
/// the name of the base form's type ([`fn_00440e30`]). A task in state 6
/// returns false.
pub fn queued_reference_get_description(
    e: &mut Engine,
    this: Ptr<QueuedReference>,
    buffer: u32,
    size: u32,
) -> bool {
    if e.call(TASK_STATE_IS_6, &args![this]).bool() {
        return false;
    }
    let reference = e.get(this, QueuedReference::pRef);
    let form = e.call(GET_BASE_FORM, &args![reference]).u32();
    let type_name = fn_00440e30(e, Ptr::new(form));
    let form_id = e.call(GET_FORM_ID, &args![reference]).u32();
    let text = e.vcall(reference.addr(), 0x130, &args![]).u32();
    e.call(
        FORMAT_STRING,
        &args![buffer, size, QUEUED_REF_FORMAT, text, form_id, type_name],
    );
    true
}

// Translated from 00440e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm` method (no name in the engine map): the name text of the
/// form's type, the first dword of the 12-byte entry `cFormType` of the table
/// at `01187004`.
pub fn fn_00440e30(e: &mut Engine, this: Ptr<TESForm>) -> u32 {
    let form_type = e.get(this, TESForm::cFormType) as u32;
    e.global::<u32>(FORM_TYPE_TABLE + form_type * 12)
}

// Translated from 00440e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTree` constructor (no name in the engine map): the
/// `QueuedReference` constructor ([`fn_0043fd40`]) with `reference` and
/// `context`, then the tree task's virtual table. Returns `this`.
pub fn fn_00440e50(
    e: &mut Engine,
    this: Ptr<QueuedReference>,
    reference: Ptr,
    context: u32,
) -> Ptr<QueuedReference> {
    fn_0043fd40(e, this, reference, context);
    e.mem.set_u32(this.addr(), QUEUED_TREE_VTABLE);
    this
}

// Translated from 00440e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTree` scalar deleting destructor (no name in the engine map): runs
/// the destructor ([`fn_00440eb0`]) and, when bit 0 of `flags` is set, frees
/// the object. Returns `this`.
pub fn fn_00440e80(e: &mut Engine, this: Ptr<QueuedReference>, flags: u32) -> Ptr<QueuedReference> {
    fn_00440eb0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 00440eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTree` destructor (no name in the engine map): the
/// `QueuedReference` destructor ([`fn_0043fe40`]).
pub fn fn_00440eb0(e: &mut Engine, this: Ptr<QueuedReference>) {
    fn_0043fe40(e, this);
}

// Translated from 00440ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTree::QueueModels` (Xbox PDB): inside the memory context of the
/// task (source line `0x847`), the loader queues the tree model
/// (`00444540(reference, base form, priority, task, LOD multiplier)`). The LOD
/// multiplier is `TES::GetLODMult(base form)`, replaced by 6 when `00564e60`
/// accepts the reference and then by 10 when `00564f00` does.
///
/// The exception-unwinding frame is not translated.
pub fn queued_tree_queue_models(e: &mut Engine, this: Ptr<QueuedReference>) {
    let context = e.get(this, QueuedReference::eContext);
    in_context(e, context, TREE_QUEUE_MODELS_SOURCE_LINE, |e| {
        let reference = e.get(this, QueuedReference::pRef);
        let form = e.call(GET_BASE_FORM, &args![reference]).u32();
        let mut lod_mult = e.call(GET_LOD_MULT, &args![form]).u32();
        let reference = e.get(this, QueuedReference::pRef);
        if !reference.is_null() && e.call(REFERENCE_TEST_00564E60, &args![reference]).bool() {
            lod_mult = 6;
        }
        let reference = e.get(this, QueuedReference::pRef);
        if !reference.is_null() && e.call(REFERENCE_TEST_00564F00, &args![reference]).bool() {
            lod_mult = 0xa;
        }
        let priority = task_priority(e, this.cast());
        let reference = e.get(this, QueuedReference::pRef);
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(
            MODEL_LOADER_QUEUE_TREE_MODEL,
            &args![loader, reference, form, priority, this, lod_mult],
        );
    });
}

// Translated from 00440fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedTree::UseDistant3D` (Xbox PDB): builds the tree's distant 3D
/// (`TESObjectTREE::BuildDistant3D` on the base form). Of the node it
/// returns, child 0 has its geometry data (`005495f0`) set to consistency
/// `0x4000` (`NiGeometryData::SetConsistency`), given `0x11` and `1` by the
/// two setters `004410d0` and `004410f0`, and the child is prepared
/// (`BSShaderManager::PrepareObject(child, 0, 0)`). If the child has a
/// property of type 3 whose own type is `0xb`, that property gets shader
/// flags `0xd` and `0x16` set ([`bs_shader_property_set_flag`]). The node is
/// handed to the task's virtual function `0x34` (`AttachDistant3D`).
pub fn queued_tree_use_distant_3d(e: &mut Engine, this: Ptr<QueuedReference>) {
    let reference = e.get(this, QueuedReference::pRef);
    let form = e.call(GET_BASE_FORM, &args![reference]).u32();
    let node = e.call(BUILD_DISTANT_3D, &args![form, 0u32]).u32();
    if node == 0 {
        return;
    }
    let child = fn_0043b4a0(e, Ptr::new(node), 0);
    let data = e.call(OBJECT_POINTER_AT_B8, &args![child]).u32();
    e.call(SET_CONSISTENCY, &args![data, 0x4000u32]);
    e.call(GEOMETRY_DATA_SETTER_FIRST, &args![data, 0x11u32]);
    e.call(GEOMETRY_DATA_SETTER_SECOND, &args![data, 1u32]);
    e.call(PREPARE_OBJECT, &args![child, 0u32, 0u32]);
    let mut property = 0;
    if e.call(GET_PROPERTY, &args![child, SHADER_PROPERTY_SLOT])
        .u32()
        != 0
    {
        let candidate = e
            .call(GET_PROPERTY, &args![child, SHADER_PROPERTY_SLOT])
            .u32();
        if e.call(PROPERTY_GET_TYPE, &args![candidate]).u32() == SHADER_PROPERTY_TYPE {
            property = e
                .call(GET_PROPERTY, &args![child, SHADER_PROPERTY_SLOT])
                .u32();
        }
    }
    if property != 0 {
        bs_shader_property_set_flag(e, Ptr::new(property), 0xd, 1);
        bs_shader_property_set_flag(e, Ptr::new(property), 0x16, 1);
    }
    e.vcall(this.addr(), 0x34, &args![node]);
}

// Translated from 00441130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderProperty::SetFlag` (Xbox PDB name in the engine map): sets
/// (`set` non-zero) or clears bit number `bit` of the array of flag dwords at
/// `this + 0x20` (dword `bit / 32`, bit `bit % 32`). When that changes the
/// bit, the dword at `this + 0x38` is cleared.
pub fn bs_shader_property_set_flag(e: &mut Engine, this: Ptr, bit: u32, set: u8) {
    let word = this.addr().wrapping_add(0x20).wrapping_add((bit >> 5) * 4);
    let mask = 1u32 << (bit % 32);
    let flags = e.mem.u32(word);
    if set != 0 {
        if flags & mask == 0 {
            e.mem.set_u32(this.addr() + 0x38, 0);
        }
        let flags = e.mem.u32(word);
        e.mem.set_u32(word, flags | mask);
    } else {
        if flags & mask != 0 {
            e.mem.set_u32(this.addr() + 0x38, 0);
        }
        let flags = e.mem.u32(word);
        e.mem.set_u32(word, !mask & flags);
    }
}

// Translated from 004411d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedActor` constructor (no name in the engine map): the
/// `QueuedReference` constructor ([`fn_0043fd40`]) with `reference` and
/// `context`, then the actor task's virtual table. Returns `this`.
pub fn fn_004411d0(
    e: &mut Engine,
    this: Ptr<QueuedReference>,
    reference: Ptr,
    context: u32,
) -> Ptr<QueuedReference> {
    fn_0043fd40(e, this, reference, context);
    e.mem.set_u32(this.addr(), QUEUED_ACTOR_VTABLE);
    this
}

// Translated from 00441200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedActor::QueueMe` (Xbox PDB): calls the reference's virtual function
/// `0x260`, then takes the thread-local queued flag ([`fn_0043c130`]). A
/// reference other than the one at global `011dea3c` that `00936ef0` does not
/// accept makes the flag 1 meanwhile ([`fn_00441290`]); an actor that
/// `008b7b70` accepts gets `008b7360(1, task)`. Then `QueuedReference::
/// QueueMe` runs and the flag is put back.
pub fn queued_actor_queue_me(e: &mut Engine, this: Ptr<QueuedReference>) {
    let reference = e.get(this, QueuedReference::pRef);
    e.vcall(reference.addr(), 0x260, &args![]);
    // 00441228: ECX is the model loader.
    let old_flag = fn_0043c130(e, Ptr::new(e.global::<u32>(MODEL_LOADER)));
    let loader = e.global::<u32>(MODEL_LOADER);
    if reference.addr() != e.global::<u32>(SPECIAL_REFERENCE)
        && e.call(REFERENCE_TEST_00936EF0, &args![reference]).u32() == 0
    {
        fn_00441290(e, Ptr::new(loader), 1);
    }
    if e.call(ACTOR_TEST_008B7B70, &args![reference]).bool() {
        e.call(ACTOR_ATTACH_TASK, &args![reference, 1u32, this]);
    }
    queued_reference_queue_me(e, this);
    let loader = e.global::<u32>(MODEL_LOADER);
    fn_00441290(e, Ptr::new(loader), old_flag);
}

// Translated from 00441290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map; `this` is not used):
/// stores `value` in the thread's queued flag (the byte at +0x25C of the TLS
/// block, the one [`fn_0043c130`] reads) and returns the byte that was there.
pub fn fn_00441290(e: &mut Engine, _this: Ptr, value: u8) -> u8 {
    let address = e.tls() + TLS_QUEUED_FLAG;
    let old = e.mem.u8(address);
    e.mem.set_u8(address, value);
    old
}

// Translated from 004412e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedActor::QueueModels` (Xbox PDB): inside the memory context of the
/// task (source line `0x8a2`), the reference is prepared (`008c2e50`) and
/// its extra data list is asked for the dismemberment extra
/// (`ExtraDataList::GetDismembermentExtra`). For every entry of the extra's
/// array at +0x20, and every element of that entry's array at +4, the loader
/// queues the element (`00444d40(element, priority, task, reference)`). Last
/// `QueuedReference::QueueModels` runs.
///
/// The exception-unwinding frame is not translated.
pub fn queued_actor_queue_models(e: &mut Engine, this: Ptr<QueuedReference>) {
    let context = e.get(this, QueuedReference::eContext);
    in_context(e, context, ACTOR_QUEUE_MODELS_SOURCE_LINE, |e| {
        let reference = e.get(this, QueuedReference::pRef);
        e.call(ACTOR_PREPARE, &args![reference]);
        let extra = e.call(REFERENCE_EXTRA_LIST, &args![reference]).u32();
        let dismember = e.call(EXTRA_GET_DISMEMBERMENT, &args![extra]).u32();
        if dismember != 0 {
            let priority = task_priority(e, this.cast());
            let mut index = 0;
            while index < e.call(ARRAY_COUNT_AT_8, &args![dismember + 0x20]).u32() {
                let entry = fn_00441420(e, Ptr::new(dismember), index);
                let mut inner = 0;
                while inner < e.call(ARRAY_COUNT_AT_8, &args![entry + 4]).u32() {
                    let slot = e
                        .call(ARRAY_ELEMENT_ADDRESS, &args![entry + 4, inner])
                        .u32();
                    let element = e.mem.u32(slot);
                    let reference = e.get(this, QueuedReference::pRef);
                    let loader = e.global::<u32>(MODEL_LOADER);
                    e.call(
                        MODEL_LOADER_QUEUE_DISMEMBER_PART,
                        &args![loader, element, priority, this, reference],
                    );
                    inner += 1;
                }
                index += 1;
            }
        }
        queued_reference_queue_models(e, this);
    });
}

// Translated from 00441420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dismemberment extra method (no name in the engine map): the dword stored
/// at the element `index` of the array at `this + 0x20`.
pub fn fn_00441420(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let slot = e
        .call(ARRAY_ELEMENT_ADDRESS, &args![this.addr() + 0x20, index])
        .u32();
    e.mem.u32(slot)
}

// Translated from 00441440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedCharacter` constructor (no name in the engine map): the
/// `QueuedActor` constructor ([`fn_004411d0`]) with `reference` and
/// `context`, the character task's virtual table, and empty task pointers in
/// `spQueuedHead` and `spQueuedHelmet`. Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_00441440(
    e: &mut Engine,
    this: Ptr<QueuedCharacter>,
    reference: Ptr,
    context: u32,
) -> Ptr<QueuedCharacter> {
    fn_004411d0(e, this.cast(), reference, context);
    e.mem.set_u32(this.addr(), QUEUED_CHARACTER_VTABLE);
    e.call(
        TASK_POINTER_CONSTRUCT,
        &args![this.byte_add(QueuedCharacter::spQueuedHead.off), 0u32],
    );
    e.call(
        TASK_POINTER_CONSTRUCT,
        &args![this.byte_add(QueuedCharacter::spQueuedHelmet.off), 0u32],
    );
    this
}

// Translated from 004414c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedCharacter` scalar deleting destructor (no name in the engine map):
/// runs the destructor ([`fn_004414f0`]) and, when bit 0 of `flags` is set,
/// frees the object. Returns `this`.
pub fn fn_004414c0(e: &mut Engine, this: Ptr<QueuedCharacter>, flags: u32) -> Ptr<QueuedCharacter> {
    fn_004414f0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 004414f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedCharacter` destructor (no name in the engine map): releases the
/// helmet task (`spQueuedHelmet`) and the head task (`spQueuedHead`), then
/// runs the `QueuedReference` destructor ([`fn_00440eb0`]). It does not
/// restore the virtual table.
///
/// The exception-unwinding frame is not translated.
pub fn fn_004414f0(e: &mut Engine, this: Ptr<QueuedCharacter>) {
    e.call(
        TASK_POINTER_DESTRUCT,
        &args![this.byte_add(QueuedCharacter::spQueuedHelmet.off)],
    );
    e.call(
        TASK_POINTER_DESTRUCT,
        &args![this.byte_add(QueuedCharacter::spQueuedHead.off)],
    );
    fn_00440eb0(e, this.cast());
}

// Translated from 00441560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedCharacter::QueueModels` (Xbox PDB): inside the memory context of
/// the task (source line `0x8ca`), the biped animation of the reference
/// (virtual function `0x1e8`, or a new one from `TESNPC::CreateBipedAnim`
/// when there is none) is given to the base form (`00606540`); when there is
/// one, it queues its files (`004abd30(priority, helmet slot, task)`) and the
/// loader queues the head (`00444fe0(base form, head slot, priority,
/// task)`). Then the loader queues the animations of the base form's
/// animation data (the base form + `0xdc`, or 0) with `ModelLoader::
/// QueueAnimations` and `QueuedActor::QueueModels` runs.
///
/// The exception-unwinding frame is not translated.
pub fn queued_character_queue_models(e: &mut Engine, this: Ptr<QueuedCharacter>) {
    let context = e.get(this, QueuedCharacter::eContext);
    in_context(e, context, CHARACTER_QUEUE_MODELS_SOURCE_LINE, |e| {
        let reference = e.get(this, QueuedCharacter::pRef);
        let base_form = e.call(GET_BASE_FORM, &args![reference]).u32();
        let mut biped = e.vcall(reference.addr(), 0x1e8, &args![]).u32();
        if biped == 0 {
            let reference = e.get(this, QueuedCharacter::pRef);
            biped = e
                .call(CREATE_BIPED_ANIM, &args![base_form, reference])
                .u32();
        }
        if biped != 0 {
            let reference = e.get(this, QueuedCharacter::pRef);
            e.call(
                NPC_SET_BIPED_ANIM,
                &args![base_form, reference, biped, 0u32],
            );
            let priority = task_priority(e, this.cast());
            let helmet_slot = this.byte_add(QueuedCharacter::spQueuedHelmet.off);
            e.call(
                BIPED_QUEUE_FILES,
                &args![biped, priority, helmet_slot, this],
            );
            let priority = task_priority(e, this.cast());
            let head_slot = this.byte_add(QueuedCharacter::spQueuedHead.off);
            let loader = e.global::<u32>(MODEL_LOADER);
            e.call(
                MODEL_LOADER_QUEUE_HEAD,
                &args![loader, base_form, head_slot, priority, this],
            );
        }
        let animation_data = if base_form != 0 { base_form + 0xdc } else { 0 };
        let reference = e.get(this, QueuedCharacter::pRef);
        let priority = task_priority(e, this.cast());
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(
            MODEL_LOADER_QUEUE_ANIMATIONS,
            &args![
                loader,
                animation_data,
                priority,
                this,
                reference,
                0u32,
                0u32
            ],
        );
        queued_actor_queue_models(e, this.cast());
    });
}

// Translated from 004416c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedCharacter::BackgroundClone` (Xbox PDB): inside the memory context
/// of the task (source line `0x8e4`), `QueuedReference::BackgroundClone`
/// runs; when it succeeded and the task has a cloned 3D (`005d8710`), the
/// thread-local clone words are set to the reference and that 3D
/// ([`fn_00441780`]), the head and helmet are attached ([`fn_004418b0`]) and
/// the words are cleared ([`fn_004417c0`]). Returns the result of
/// `QueuedReference::BackgroundClone`.
///
/// The exception-unwinding frame is not translated.
pub fn queued_character_background_clone(e: &mut Engine, this: Ptr<QueuedCharacter>) -> bool {
    let context = e.get(this, QueuedCharacter::eContext);
    in_context(e, context, CHARACTER_BACKGROUND_CLONE_SOURCE_LINE, |e| {
        let cloned = queued_reference_background_clone(e, this.cast());
        if cloned && e.call(GET_CLONED_3D, &args![this]).u32() != 0 {
            let node = e.call(GET_CLONED_3D, &args![this]).u32();
            let reference = e.get(this, QueuedCharacter::pRef);
            fn_00441780(e, reference, node);
            fn_004418b0(e, this);
            fn_004417c0(e);
        }
        cloned
    })
}

// Translated from 00441780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` helper (no name in the engine map): stores `reference` and
/// `node` in the thread-local words at +0x264 and +0x260 of the TLS block.
pub fn fn_00441780(e: &mut Engine, reference: Ptr, node: u32) {
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_CLONE_REFERENCE, reference.addr());
    e.mem.set_u32(tls + TLS_CLONE_WORD, node);
}

// Translated from 004417c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` helper (no name in the engine map): clears the thread-local
/// words that [`fn_00441780`] sets.
pub fn fn_004417c0(e: &mut Engine) {
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_CLONE_REFERENCE, 0);
    e.mem.set_u32(tls + TLS_CLONE_WORD, 0);
}

// Translated from 00441800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedCharacter::Attach` (no name in the engine map): the
/// `QueuedReference::Attach` ([`queued_reference_attach`]).
pub fn fn_00441800(e: &mut Engine, this: Ptr<QueuedReference>) {
    queued_reference_attach(e, this);
}

// Translated from 00441820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedCharacter::FinishAttach` (Xbox PDB): inside the memory context of
/// the task (source line `0x8ff`), the base class's (empty) `FinishAttach`
/// runs; a task without a cloned 3D (`005d8710`) then attaches its head and
/// helmet ([`fn_004418b0`]).
///
/// The exception-unwinding frame is not translated.
pub fn queued_character_finish_attach(e: &mut Engine, this: Ptr<QueuedCharacter>) {
    let context = e.get(this, QueuedCharacter::eContext);
    in_context(e, context, CHARACTER_FINISH_ATTACH_SOURCE_LINE, |e| {
        e.call(FINISH_ATTACH_BASE, &args![this]);
        if e.call(GET_CLONED_3D, &args![this]).u32() == 0 {
            fn_004418b0(e, this);
        }
    });
}

// Translated from 004418b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedCharacter` method (no name in the engine map): asks `005d9f90`
/// about the reference, attaches the head task if there is one
/// (`0043ed90(reference, answer)`) and then the helmet task
/// (`QueuedHelmet::Attach`).
pub fn fn_004418b0(e: &mut Engine, this: Ptr<QueuedCharacter>) {
    let reference = e.get(this, QueuedCharacter::pRef);
    let answer = e.call(REFERENCE_QUERY_005D9F90, &args![reference]).u32();
    let head_slot = this.byte_add(QueuedCharacter::spQueuedHead.off);
    if ni_pointer_value(e, head_slot) != 0 {
        let reference = e.get(this, QueuedCharacter::pRef);
        let head = ni_pointer_value(e, head_slot);
        e.call(HEAD_ATTACH, &args![head, reference, answer]);
    }
    let helmet_slot = this.byte_add(QueuedCharacter::spQueuedHelmet.off);
    if ni_pointer_value(e, helmet_slot) != 0 {
        let helmet = ni_pointer_value(e, helmet_slot);
        e.call(HELMET_ATTACH, &args![helmet]);
    }
}

// Translated from 00441920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedCreature` constructor (no name in the engine map): the
/// `QueuedActor` constructor ([`fn_004411d0`]) with `reference` and
/// `context`, then the creature task's virtual table. Returns `this`.
pub fn fn_00441920(
    e: &mut Engine,
    this: Ptr<QueuedReference>,
    reference: Ptr,
    context: u32,
) -> Ptr<QueuedReference> {
    fn_004411d0(e, this, reference, context);
    e.mem.set_u32(this.addr(), QUEUED_CREATURE_VTABLE);
    this
}

// Translated from 00441950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedCreature` scalar deleting destructor (no name in the engine map):
/// runs the destructor ([`fn_00441980`]) and, when bit 0 of `flags` is set,
/// frees the object. Returns `this`.
pub fn fn_00441950(e: &mut Engine, this: Ptr<QueuedReference>, flags: u32) -> Ptr<QueuedReference> {
    fn_00441980(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 00441980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedCreature` destructor (no name in the engine map): the
/// `QueuedReference` destructor ([`fn_00440eb0`]).
pub fn fn_00441980(e: &mut Engine, this: Ptr<QueuedReference>) {
    fn_00440eb0(e, this);
}

// Translated from 004419a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedCreature::QueueModels` (Xbox PDB): inside the memory context of the
/// task (source line `0x925`), the creature's worn items are initialised: the
/// package of the reference (`MobileObject::GetCurrentPackage`) with bit
/// `0x200000` at +0x1C ([`fn_00441b00`]) turns the second flag off; a base
/// form that `TESActorBase::ShouldRunInitDefaultWorn` accepts gets
/// `TESCreature::InitDefaultWorn(reference, 1, flag, 0)`, any other
/// `TESCreature::InitWorn(reference)`. The loader then queues the
/// animations (`ModelLoader::QueueAnimations` with the flags 1 and 0) and the
/// creature parts (`ModelLoader::QueueCreatureParts`) of the base form's
/// animation data (the base form + `0xdc`, or 0), and `QueuedActor::
/// QueueModels` runs.
///
/// The exception-unwinding frame is not translated.
pub fn queued_creature_queue_models(e: &mut Engine, this: Ptr<QueuedReference>) {
    let context = e.get(this, QueuedReference::eContext);
    in_context(e, context, CREATURE_QUEUE_MODELS_SOURCE_LINE, |e| {
        let reference = e.get(this, QueuedReference::pRef);
        let base_form = e.call(GET_BASE_FORM, &args![reference]).u32();
        let reference = e.get(this, QueuedReference::pRef);
        let mut flag = 1u32;
        let package = e.call(GET_CURRENT_PACKAGE, &args![reference]).u32();
        if package != 0 && fn_00441b00(e, Ptr::new(package)) {
            flag = 0;
        }
        let reference = e.get(this, QueuedReference::pRef);
        if !e
            .call(SHOULD_RUN_INIT_DEFAULT_WORN, &args![base_form, reference])
            .bool()
        {
            let reference = e.get(this, QueuedReference::pRef);
            e.call(CREATURE_INIT_WORN, &args![base_form, reference]);
        } else {
            let reference = e.get(this, QueuedReference::pRef);
            e.call(
                CREATURE_INIT_DEFAULT_WORN,
                &args![base_form, reference, 1u32, flag, 0u32],
            );
        }
        let animation_data = if base_form != 0 { base_form + 0xdc } else { 0 };
        let reference = e.get(this, QueuedReference::pRef);
        let priority = task_priority(e, this.cast());
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(
            MODEL_LOADER_QUEUE_ANIMATIONS,
            &args![
                loader,
                animation_data,
                priority,
                this,
                reference,
                1u32,
                0u32
            ],
        );
        let reference = e.get(this, QueuedReference::pRef);
        let priority = task_priority(e, this.cast());
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(
            MODEL_LOADER_QUEUE_CREATURE_PARTS,
            &args![loader, animation_data, priority, this, reference],
        );
        queued_actor_queue_models(e, this);
    });
}

// Translated from 00441b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Package predicate (no name in the engine map): bit `0x200000` of the
/// dword at +0x1C is set.
pub fn fn_00441b00(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x1c) & PACKAGE_FLAG_MASK != 0
}

// Translated from 00441b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedPlayer` constructor (no name in the engine map): the
/// `QueuedCharacter` constructor ([`fn_00441440`]) for the reference in the
/// global `011dea3c` and `context`, then the player task's virtual table.
/// Returns `this`.
pub fn fn_00441b20(
    e: &mut Engine,
    this: Ptr<QueuedCharacter>,
    context: u32,
) -> Ptr<QueuedCharacter> {
    let player = Ptr::new(e.global::<u32>(SPECIAL_REFERENCE));
    fn_00441440(e, this, player, context);
    e.mem.set_u32(this.addr(), QUEUED_PLAYER_VTABLE);
    this
}

// Translated from 00441b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedPlayer` scalar deleting destructor (no name in the engine map):
/// runs the destructor ([`fn_00441b80`]) and, when bit 0 of `flags` is set,
/// frees the object. Returns `this`.
pub fn fn_00441b50(e: &mut Engine, this: Ptr<QueuedCharacter>, flags: u32) -> Ptr<QueuedCharacter> {
    fn_00441b80(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 00441b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedPlayer` destructor (no name in the engine map): the
/// `QueuedCharacter` destructor ([`fn_004414f0`]).
pub fn fn_00441b80(e: &mut Engine, this: Ptr<QueuedCharacter>) {
    fn_004414f0(e, this);
}

// Translated from 00441ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedPlayer::QueueModels` (Xbox PDB): inside the memory context of the
/// task (source line `0x94e`), the player reference (global `011dea3c`) is
/// asked to queue its files (`0095f210(priority, task)`), then
/// `QueuedCharacter::QueueModels` runs.
///
/// The exception-unwinding frame is not translated.
pub fn queued_player_queue_models(e: &mut Engine, this: Ptr<QueuedCharacter>) {
    let context = e.get(this, QueuedCharacter::eContext);
    in_context(e, context, PLAYER_QUEUE_MODELS_SOURCE_LINE, |e| {
        let priority = task_priority(e, this.cast());
        let player = e.global::<u32>(SPECIAL_REFERENCE);
        e.call(PLAYER_QUEUE_FILES, &args![player, priority, this]);
        queued_character_queue_models(e, this);
    });
}

// Translated from 00441c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedPlayer::Attach` (no name in the engine map): [`fn_00441800`].
pub fn fn_00441c30(e: &mut Engine, this: Ptr<QueuedReference>) {
    fn_00441800(e, this);
}

// Translated from 00441c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedFileLoad` constructor (no name in the engine map): the
/// `QueuedFileEntry` constructor with `context`, the file load task's virtual
/// table, an empty `spLoadedFile`, `eFileIndex` = `file_kind`, a copy of the
/// file name `name` (`00c3cee0`) and the file entry found for the index that
/// `00449050` gives for `file_kind` (`00c3cf60`). Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_00441c50(
    e: &mut Engine,
    this: Ptr<QueuedFileLoad>,
    name: u32,
    context: u32,
    file_kind: u32,
) -> Ptr<QueuedFileLoad> {
    e.call(QUEUED_FILE_ENTRY_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(this.addr(), QUEUED_FILE_LOAD_VTABLE);
    e.call(
        LOADED_FILE_POINTER_CONSTRUCT,
        &args![this.byte_add(QueuedFileLoad::spLoadedFile.off), 0u32],
    );
    e.set(this, QueuedFileLoad::eFileIndex, file_kind);
    e.call(QUEUED_FILE_ENTRY_SET_NAME, &args![this, name]);
    let kind = e.get(this, QueuedFileLoad::eFileIndex);
    let index = e.call(FILE_INDEX_OF_KIND, &args![kind]).u32();
    e.call(QUEUED_FILE_ENTRY_SET_INDEX, &args![this, index]);
    this
}

// Translated from 00441cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedFileLoad::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor ([`fn_00441d20`]) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn queued_file_load_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<QueuedFileLoad>,
    flags: u32,
) -> Ptr<QueuedFileLoad> {
    fn_00441d20(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 00441d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedFileLoad` destructor (no name in the engine map; the map's name
/// belongs to folded code): releases `spLoadedFile` and runs the
/// `QueuedFileEntry` destructor (`00c3cea0`).
///
/// The exception-unwinding frame is not translated.
pub fn fn_00441d20(e: &mut Engine, this: Ptr<QueuedFileLoad>) {
    e.call(
        LOADED_FILE_POINTER_DESTRUCT,
        &args![this.byte_add(QueuedFileLoad::spLoadedFile.off)],
    );
    e.call(QUEUED_FILE_ENTRY_DESTRUCT, &args![this]);
}

// Translated from 00441d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedFileLoad` constructor for an already loaded file (no name in the
/// engine map): the `QueuedFileEntry` constructor with `context`, the file
/// load task's virtual table, `eFileIndex` = 3, `spLoadedFile` = `file`, and
/// the task put into state 5 (`00449150`). Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_00441d80(
    e: &mut Engine,
    this: Ptr<QueuedFileLoad>,
    file: Ptr,
    context: u32,
) -> Ptr<QueuedFileLoad> {
    e.call(QUEUED_FILE_ENTRY_CONSTRUCT, &args![this, context]);
    e.mem.set_u32(this.addr(), QUEUED_FILE_LOAD_VTABLE);
    let file_slot = this.byte_add(QueuedFileLoad::spLoadedFile.off);
    e.call(LOADED_FILE_POINTER_CONSTRUCT, &args![file_slot, 0u32]);
    e.set(this, QueuedFileLoad::eFileIndex, 3);
    e.call(LOADED_FILE_POINTER_ASSIGN, &args![file_slot, file]);
    e.call(TASK_SET_STATE_5, &args![this]);
    this
}

// Translated from 00441e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedKF::QueueMe` (Xbox PDB; the body is shared with other tasks): asks
/// the IO manager (global `01202d98`, virtual function `0x48`) to queue the
/// task.
pub fn queued_kf_queue_me(e: &mut Engine, this: Ptr<QueuedKF>) {
    let manager = e.global::<u32>(TASK_QUEUE);
    e.vcall(manager, 0x48, &args![this]);
}

// Translated from 00441e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedFileLoad::Run` (Xbox PDB): inside the memory context of the task
/// (source line `0x976`), the loader (`004483a0`) is asked for the loaded
/// file of the task's file name and `spLoadedFile` takes it. A task still
/// without one finds the file of its file index with `QueuedFileEntry::
/// GetFile` (the index from `00449050`, `0xffff`); with a file it builds a
/// `LoadedFile` (0x10 bytes from `NiAlloc`, [`fn_0043baf0`]) for the file
/// name and stores that, without one it logs `"MODELS: Could not get file for
/// %s."`.
///
/// The exception-unwinding frame is not translated.
pub fn queued_file_load_run(e: &mut Engine, this: Ptr<QueuedFileLoad>) {
    let context = e.get(this, QueuedFileLoad::eContext);
    in_context(e, context, FILE_LOAD_RUN_SOURCE_LINE, |e| {
        let file_slot = this.byte_add(QueuedFileLoad::spLoadedFile.off);
        let name = e.call(QUEUED_FILE_ENTRY_GET_NAME, &args![this]).u32();
        let loader = e.global::<u32>(MODEL_LOADER);
        let found = e
            .call(MODEL_LOADER_FIND_LOADED_FILE, &args![loader, name])
            .u32();
        e.call(LOADED_FILE_POINTER_ASSIGN, &args![file_slot, found]);
        if ni_pointer_value(e, file_slot) != 0 {
            return;
        }
        let kind = e.get(this, QueuedFileLoad::eFileIndex);
        let index = e.call(FILE_INDEX_OF_KIND, &args![kind]).u32();
        let file = e
            .call(QUEUED_FILE_ENTRY_GET_FILE, &args![this, index, 0xffffu32])
            .u32();
        if file != 0 {
            let block = e.call(NI_ALLOC, &args![0x10u32]).u32();
            let loaded = if block != 0 {
                let name = e.call(QUEUED_FILE_ENTRY_GET_NAME, &args![this]).u32();
                e.call(LOADED_FILE_CONSTRUCT, &args![block, name, file])
                    .u32()
            } else {
                0
            };
            e.call(LOADED_FILE_POINTER_ASSIGN, &args![file_slot, loaded]);
        } else {
            let name = e.call(QUEUED_FILE_ENTRY_GET_NAME, &args![this]).u32();
            e.call(LOG_MESSAGE, &args![FILE_LOAD_ERROR_FORMAT, name]);
        }
    });
}

// Translated from 00441f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedFaceGenFile` constructor (no name in the engine map): the
/// `QueuedFileLoad` constructor ([`fn_00441c50`]), the face-gen task's
/// virtual table and empty `spFaceGenModel` and `spModelMeshData`. Returns
/// `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_00441f80(
    e: &mut Engine,
    this: Ptr<QueuedFaceGenFile>,
    name: u32,
    context: u32,
    file_kind: u32,
) -> Ptr<QueuedFaceGenFile> {
    fn_00441c50(e, this.cast(), name, context, file_kind);
    e.mem.set_u32(this.addr(), QUEUED_FACE_GEN_FILE_VTABLE);
    e.call(
        NI_POINTER_CONSTRUCT,
        &args![this.byte_add(QueuedFaceGenFile::spFaceGenModel.off), 0u32],
    );
    e.call(
        MESH_DATA_POINTER_CONSTRUCT,
        &args![this.byte_add(QueuedFaceGenFile::spModelMeshData.off), 0u32],
    );
    this
}

// Translated from 00442010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedFaceGenFile::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor ([`fn_00442040`]) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn queued_face_gen_file_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<QueuedFaceGenFile>,
    flags: u32,
) -> Ptr<QueuedFaceGenFile> {
    fn_00442040(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 00442040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedFaceGenFile` destructor (no name in the engine map): releases
/// `spModelMeshData` and `spFaceGenModel`, then runs the `QueuedFileLoad`
/// destructor ([`fn_00441d20`]).
///
/// The exception-unwinding frame is not translated.
pub fn fn_00442040(e: &mut Engine, this: Ptr<QueuedFaceGenFile>) {
    e.call(
        MESH_DATA_POINTER_DESTRUCT,
        &args![this.byte_add(QueuedFaceGenFile::spModelMeshData.off)],
    );
    e.call(
        NI_POINTER_DESTRUCT,
        &args![this.byte_add(QueuedFaceGenFile::spFaceGenModel.off)],
    );
    fn_00441d20(e, this.cast());
}

// Translated from 004420b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedFaceGenFile` constructor for an already loaded file (no name in the
/// engine map): the `QueuedFileLoad` constructor for a loaded file
/// ([`fn_00441d80`]), the face-gen task's virtual table and empty
/// `spFaceGenModel` and `spModelMeshData`. Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_004420b0(
    e: &mut Engine,
    this: Ptr<QueuedFaceGenFile>,
    file: Ptr,
    context: u32,
) -> Ptr<QueuedFaceGenFile> {
    fn_00441d80(e, this.cast(), file, context);
    e.mem.set_u32(this.addr(), QUEUED_FACE_GEN_FILE_VTABLE);
    e.call(
        NI_POINTER_CONSTRUCT,
        &args![this.byte_add(QueuedFaceGenFile::spFaceGenModel.off), 0u32],
    );
    e.call(
        MESH_DATA_POINTER_CONSTRUCT,
        &args![this.byte_add(QueuedFaceGenFile::spModelMeshData.off), 0u32],
    );
    this
}

// Translated from 00442130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedFaceGenFile` constructor from a face model and its mesh data (no
/// name in the engine map): the `QueuedFileLoad` constructor for a loaded file
/// ([`fn_00441d80`]) with no file and the context 5, the face-gen task's
/// virtual table, then `spFaceGenModel` = `face_model` and `spModelMeshData`
/// = `mesh_data`. Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn fn_00442130(
    e: &mut Engine,
    this: Ptr<QueuedFaceGenFile>,
    face_model: Ptr,
    mesh_data: Ptr,
) -> Ptr<QueuedFaceGenFile> {
    fn_00441d80(e, this.cast(), Ptr::NULL, 5);
    e.mem.set_u32(this.addr(), QUEUED_FACE_GEN_FILE_VTABLE);
    let model_slot = this.byte_add(QueuedFaceGenFile::spFaceGenModel.off);
    let mesh_slot = this.byte_add(QueuedFaceGenFile::spModelMeshData.off);
    e.call(NI_POINTER_CONSTRUCT, &args![model_slot, 0u32]);
    e.call(MESH_DATA_POINTER_CONSTRUCT, &args![mesh_slot, 0u32]);
    e.call(NI_POINTER_ASSIGN, &args![model_slot, face_model]);
    e.call(MESH_DATA_POINTER_ASSIGN, &args![mesh_slot, mesh_data]);
    this
}

// Translated from 004421d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BackgroundCloneThread::BackgroundCloneThread` (Xbox PDB): the
/// `BSTaskThread` constructor (`00c42dd0(3, "BackgroundCloneThread")`), the
/// thread's virtual table, `iRunningCount` = 0, `bExit` = 0 and a new queue
/// (a 0x40 byte block of the memory manager constructed with `queue_size`
/// and 8, `0044b350`; 0 when the allocation failed) in
/// `pCloneReferencesQueue`. Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn background_clone_thread_background_clone_thread(
    e: &mut Engine,
    this: Ptr<BackgroundCloneThread>,
    queue_size: u32,
) -> Ptr<BackgroundCloneThread> {
    e.call(
        TASK_THREAD_CONSTRUCT,
        &args![this, 3u32, BACKGROUND_CLONE_THREAD_NAME],
    );
    e.mem.set_u32(this.addr(), BACKGROUND_CLONE_THREAD_VTABLE);
    e.set(this, BackgroundCloneThread::iRunningCount, 0);
    e.set(this, BackgroundCloneThread::bExit, 0);
    let block = e.call(MEMORY_ALLOC, &args![0x40u32]).u32();
    let queue = if block != 0 {
        e.call(CLONE_QUEUE_CONSTRUCT, &args![block, queue_size, 8u32])
            .ptr::<()>()
    } else {
        Ptr::NULL
    };
    e.set(this, BackgroundCloneThread::pCloneReferencesQueue, queue);
    this
}

// Translated from 00442290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BackgroundCloneThread::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor ([`fn_004422c0`]) and, when bit 0 of `flags` is set, frees the
/// object. Returns `this`.
pub fn background_clone_thread_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BackgroundCloneThread>,
    flags: u32,
) -> Ptr<BackgroundCloneThread> {
    fn_004422c0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 004422c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BackgroundCloneThread` destructor (no name in the engine map): sets the
/// thread's virtual table, deletes the queue (virtual destructor, flag 1)
/// when there is one, and runs the `BSTaskThread` destructor body
/// (`00c42f00`).
///
/// The exception-unwinding frame is not translated.
pub fn fn_004422c0(e: &mut Engine, this: Ptr<BackgroundCloneThread>) {
    e.mem.set_u32(this.addr(), BACKGROUND_CLONE_THREAD_VTABLE);
    let queue = e.get(this, BackgroundCloneThread::pCloneReferencesQueue);
    if !queue.is_null() {
        e.vcall(queue.addr(), 0, &args![1u32]);
    }
    e.call(TASK_THREAD_DESTRUCT, &args![this]);
}

// Translated from 00442350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BackgroundCloneThread::ThreadUpdate` (Xbox PDB): until `bExit` is 1, the
/// thread waits for work (`004424c0`), counts itself running
/// (`iRunningCount`, `0040b460`) and pops a task from the queue (`00449280`).
/// Without a task it only runs the two bracketing calls (`00442510`,
/// `00442530`). With one, for each task popped: when it is not in state 6 and
/// its state could be changed from the state it was in to 3 (`00449190`), a
/// task whose virtual function `0x38` says it needs it is added to the IO
/// manager's post-process tasks (`IOManager::AddPostProcessTask`), and the
/// state changes from 3 to 5; the next task is then popped. Last the running
/// count is decremented.
///
/// The exception-unwinding frame is not translated.
pub fn background_clone_thread_thread_update(e: &mut Engine, this: Ptr<BackgroundCloneThread>) {
    let running = this.addr() + BackgroundCloneThread::iRunningCount.off;
    while e.get(this, BackgroundCloneThread::bExit) != 1 {
        e.call(THREAD_WAIT_FOR_WORK, &args![this]);
        e.call(INTERLOCKED_INCREMENT, &args![running]);
        e.with_stack(4, |e, slot| {
            e.call(TASK_POINTER_CONSTRUCT, &args![slot, 0u32]);
            let queue = e.get(this, BackgroundCloneThread::pCloneReferencesQueue);
            e.call(CLONE_QUEUE_POP, &args![queue, slot]);
            if ni_pointer_value(e, slot) == 0 {
                e.call(THREAD_BEFORE_TASK, &args![this]);
                e.call(THREAD_AFTER_TASK, &args![this]);
            } else {
                while ni_pointer_value(e, slot) != 0 {
                    e.call(THREAD_BEFORE_TASK, &args![this]);
                    e.call(EMPTY_FUNCTION, &args![]);
                    let task = ni_pointer_value(e, slot);
                    let state = e.call(TASK_GET_STATE, &args![task]).u32();
                    if !e.call(TASK_STATE_IS_6, &args![task]).bool()
                        && e.call(TASK_CHANGE_STATE, &args![task, state, 3u32]).bool()
                    {
                        if e.vcall(task, TASK_NEEDS_POST_PROCESS_SLOT, &args![]).bool() {
                            e.call(IOMANAGER_ADD_POST_PROCESS_TASK, &args![task]);
                        }
                        e.call(TASK_CHANGE_STATE, &args![task, 3u32, 5u32]);
                    }
                    let queue = e.get(this, BackgroundCloneThread::pCloneReferencesQueue);
                    e.call(CLONE_QUEUE_POP, &args![queue, slot]);
                    e.call(EMPTY_FUNCTION, &args![]);
                    e.call(THREAD_AFTER_TASK, &args![this]);
                }
            }
            e.call(INTERLOCKED_DECREMENT, &args![running]);
            e.call(TASK_POINTER_DESTRUCT, &args![slot]);
        });
    }
}

// Translated from 00442580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BackgroundCloneThread` method (no name in the engine map): puts `task` in
/// a temporary task pointer, adds it to the thread's queue (`00449240` on
/// `pCloneReferencesQueue`) and wakes the thread (`00442600`).
///
/// The exception-unwinding frame is not translated.
pub fn fn_00442580(e: &mut Engine, this: Ptr<BackgroundCloneThread>, task: Ptr) {
    e.with_stack(4, |e, slot| {
        e.call(TASK_POINTER_CONSTRUCT, &args![slot, task]);
        let queue = e.get(this, BackgroundCloneThread::pCloneReferencesQueue);
        e.call(FINISHED_QUEUE_ADD, &args![queue, slot]);
        e.call(TASK_POINTER_DESTRUCT, &args![slot]);
    });
    e.call(THREAD_WAKE, &args![this]);
}

// Translated from 00442630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BackgroundCloneThread` method (no name in the engine map): sets `bExit`
/// to 1.
pub fn fn_00442630(e: &mut Engine, this: Ptr<BackgroundCloneThread>) {
    e.set(this, BackgroundCloneThread::bExit, 1);
}

/// A fresh block of `size` bytes from the memory manager, constructed by
/// `constructor(block, arguments...)`; null when the allocation failed.
fn new_loader_member(e: &mut Engine, size: u32, constructor: u32, arguments: &[u32]) -> Ptr {
    let block = e.call(MEMORY_ALLOC, &args![size]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    let mut words = vec![block];
    words.extend_from_slice(arguments);
    e.call(constructor, &words).ptr()
}

// Translated from 00442650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::ModelLoader` (Xbox PDB): creates the loader's maps and
/// queues, each a 0x40 byte block of the memory manager (0 when the
/// allocation failed) constructed with its own constructor: the model map,
/// the KF model map, the queued references and references-to-queue maps, the
/// anim idle, replacement KF list and helmet maps, the attach-distant-3D
/// queue, the texture map and the loaded file map; then the background clone
/// thread (0x3c bytes, [`background_clone_thread_background_clone_thread`]
/// with 10), which is started (`00c42f50`). `bHasDelayedFree` is cleared.
/// When the IO manager (global `01202d98`) exists, six loader callbacks are
/// registered on it (`0057bd60`, `004febb0`, [`fn_00442a80`], `004fbf00`,
/// `004febd0`, `008ac890`). Returns `this`.
///
/// The exception-unwinding frame is not translated.
pub fn model_loader_model_loader(e: &mut Engine, this: Ptr<ModelLoader>) -> Ptr<ModelLoader> {
    let map = new_loader_member(e, 0x40, MODEL_MAP_CONSTRUCT, &[10, 0x3f1, 0xc]);
    e.set(this, ModelLoader::pModelMap, map);
    let map = new_loader_member(e, 0x40, KF_MODEL_MAP_CONSTRUCT, &[10, 0x3f1, 0xc]);
    e.set(this, ModelLoader::pKFModelMap, map);
    let map = new_loader_member(e, 0x40, REFERENCE_MAP_CONSTRUCT, &[10, 0x13af, 0xc]);
    e.set(this, ModelLoader::pQueuedReferencesMap, map);
    let map = new_loader_member(e, 0x40, REFERENCE_MAP_CONSTRUCT, &[10, 0x13af, 0xc]);
    e.set(this, ModelLoader::pReferencesToQueueMap, map);
    let map = new_loader_member(e, 0x40, ANIM_IDLE_MAP_CONSTRUCT, &[10, 0xfb, 0xc]);
    e.set(this, ModelLoader::pQueuedAnimIdleMap, map);
    let map = new_loader_member(e, 0x40, REPLACEMENT_KF_LIST_MAP_CONSTRUCT, &[10, 0xfb, 0xc]);
    e.set(this, ModelLoader::pQueuedReplacementKFListMap, map);
    let map = new_loader_member(e, 0x40, HELMET_MAP_CONSTRUCT, &[10, 0xfb, 0xc]);
    e.set(this, ModelLoader::pQueuedHelmetMap, map);
    let queue = new_loader_member(e, 0x40, ATTACH_TASK_QUEUE_CONSTRUCT, &[10, 8]);
    e.set(this, ModelLoader::pAttachDistant3DTaskQueue, queue);
    let map = new_loader_member(e, 0x40, TEXTURE_MAP_CONSTRUCT, &[10, 0x3f1, 0xc]);
    e.set(this, ModelLoader::pQueuedTextureMap, map);
    let map = new_loader_member(e, 0x40, LOADED_FILE_MAP_CONSTRUCT, &[10, 0x3f1, 0xc]);
    e.set(this, ModelLoader::pLoadedFileMap, map);
    let block = e.call(MEMORY_ALLOC, &args![0x3cu32]).u32();
    let thread = if block != 0 {
        background_clone_thread_background_clone_thread(e, Ptr::new(block), 10).cast::<()>()
    } else {
        Ptr::NULL
    };
    e.set(this, ModelLoader::pBackgroundCloneThread, thread);
    e.call(THREAD_START, &args![thread]);
    e.set(this, ModelLoader::bHasDelayedFree, 0);
    if e.global::<u32>(TASK_QUEUE) != 0 {
        let manager = e.global::<u32>(TASK_QUEUE);
        e.call(
            IO_MANAGER_REGISTER_FIRST,
            &args![manager, LOADER_CALLBACK_00446E30],
        );
        let manager = e.global::<u32>(TASK_QUEUE);
        e.call(
            IO_MANAGER_REGISTER_SECOND,
            &args![manager, LOADER_CALLBACK_00446E40],
        );
        let manager = e.global::<u32>(TASK_QUEUE);
        fn_00442a80(e, Ptr::new(manager), LOADER_CALLBACK_00446EA0);
        let manager = e.global::<u32>(TASK_QUEUE);
        e.call(
            IO_MANAGER_REGISTER_FOURTH,
            &args![manager, LOADER_CALLBACK_00446F00],
        );
        let manager = e.global::<u32>(TASK_QUEUE);
        e.call(
            IO_MANAGER_REGISTER_FIFTH,
            &args![manager, LOADER_CALLBACK_00446F10],
        );
        let manager = e.global::<u32>(TASK_QUEUE);
        e.call(
            IO_MANAGER_REGISTER_SIXTH,
            &args![manager, LOADER_CALLBACK_00446F90],
        );
    }
    this
}

// Translated from 00442a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// IO manager setter (no name in the engine map): stores `callback` in the
/// dword at +0x78 of the manager.
pub fn fn_00442a80(e: &mut Engine, this: Ptr, callback: u32) {
    e.mem.set_u32(this.addr() + 0x78, callback);
}

/// `next(iterator, &key, &value, 1)` (`0044c640`) on `map`: the key and the
/// value it wrote, or `None` when it found no entry.
fn map_next(e: &mut Engine, map: Ptr, iterator: Ptr) -> Option<(u32, u32)> {
    e.with_stack(8, |e, out| {
        let value_slot = out.addr() + 4;
        e.mem.set_u32(out.addr(), 0);
        e.mem.set_u32(value_slot, 0);
        let found = e
            .call(MAP_NEXT, &args![map, iterator, out, value_slot, 1u32])
            .bool();
        found.then(|| (e.mem.u32(out.addr()), e.mem.u32(value_slot)))
    })
}

/// Deletes `object` through its virtual destructor (slot 0, flag 1) when it
/// is not null.
fn delete_object(e: &mut Engine, object: Ptr) {
    if !object.is_null() {
        e.vcall(object.addr(), 0, &args![1u32]);
    }
}

// Translated from 00442aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::~ModelLoader` (Xbox PDB): first `OutputModelMapContents(0,
/// 0)` runs. A model map with entries (virtual function `0x44` not 0) is
/// walked: the models whose reference count ([`fn_00443190`]) is above 0 are
/// counted and every model is deleted (`004431f0`); a non-zero count is
/// logged (`"MODELS: ModelLoader still contains %d NIF files."`). Then the
/// KF model map is walked: an entry whose animation is missing (`005585e0` is
/// 0) or fails `005f26c0` is removed from the map (virtual function `0x14`
/// with the key) and deleted (`00443240`). A KF map with entries is walked
/// again, counting the KF models whose count (`004431b0`) is above 0 and
/// deleting them all, with the same kind of log message. Last the maps, the
/// queue and the background clone thread are deleted through their virtual
/// destructors, in the order `pModelMap`, `pKFModelMap`,
/// `pAttachDistant3DTaskQueue`, `pQueuedHelmetMap`,
/// `pQueuedReplacementKFListMap`, `pQueuedAnimIdleMap`,
/// `pQueuedReferencesMap`, `pReferencesToQueueMap`,
/// `pBackgroundCloneThread`, `pQueuedTextureMap`, `pLoadedFileMap`.
///
/// The exception-unwinding frame and the stack-cookie check are not
/// translated.
pub fn model_loader_destructor(e: &mut Engine, this: Ptr<ModelLoader>) {
    e.call(OUTPUT_MODEL_MAP_CONTENTS, &args![this, 0u32, 0u32]);
    let model_map = e.get(this, ModelLoader::pModelMap);
    if e.vcall(model_map.addr(), 0x44, &args![]).u32() != 0 {
        let mut left = 0u32;
        e.with_stack(MAP_ITERATOR_SIZE, |e, iterator| {
            e.call(MODEL_ITERATOR_CONSTRUCT, &args![iterator]);
            while !e.call(ITERATOR_AT_END, &args![iterator]).bool() {
                let model_map = e.get(this, ModelLoader::pModelMap);
                if let Some((_, model)) = map_next(e, model_map, iterator) {
                    if fn_00443190(e, Ptr::new(model)) > 0 {
                        left += 1;
                    }
                    if model != 0 {
                        e.call(MODEL_DELETE, &args![model, 1u32]);
                    }
                }
            }
            if left != 0 {
                e.call(LOG_MESSAGE, &args![MODEL_FILES_LEFT_FORMAT, left]);
            }
            e.call(MODEL_ITERATOR_DESTRUCT, &args![iterator]);
        });
    }
    e.with_stack(MAP_ITERATOR_SIZE, |e, iterator| {
        e.call(KF_ITERATOR_CONSTRUCT, &args![iterator]);
        while !e.call(ITERATOR_AT_END, &args![iterator]).bool() {
            let kf_map = e.get(this, ModelLoader::pKFModelMap);
            let Some((key, model)) = map_next(e, kf_map, iterator) else {
                continue;
            };
            if model == 0 {
                continue;
            }
            let animation = e.call(KF_MODEL_ANIMATION, &args![model]).u32();
            let remove = animation == 0 || {
                let animation = e.call(KF_MODEL_ANIMATION, &args![model]).u32();
                e.call(ANIMATION_TEST, &args![animation]).bool()
            };
            if remove {
                let kf_map = e.get(this, ModelLoader::pKFModelMap);
                e.vcall(kf_map.addr(), 0x14, &args![key]);
                e.call(KF_MODEL_DELETE, &args![model, 1u32]);
            }
        }
        let kf_map = e.get(this, ModelLoader::pKFModelMap);
        if e.vcall(kf_map.addr(), 0x44, &args![]).u32() != 0 {
            let mut left = 0u32;
            e.with_stack(MAP_ITERATOR_SIZE, |e, second| {
                e.call(KF_ITERATOR_CONSTRUCT, &args![second]);
                while !e.call(ITERATOR_AT_END, &args![second]).bool() {
                    let kf_map = e.get(this, ModelLoader::pKFModelMap);
                    if let Some((_, model)) = map_next(e, kf_map, second) {
                        if e.call(KF_MODEL_REF_COUNT, &args![model]).u32() as i32 > 0 {
                            left += 1;
                        }
                        if model != 0 {
                            e.call(KF_MODEL_DELETE, &args![model, 1u32]);
                        }
                    }
                }
                if left != 0 {
                    e.call(LOG_MESSAGE, &args![KF_FILES_LEFT_FORMAT, left]);
                }
                e.call(KF_ITERATOR_DESTRUCT, &args![second]);
            });
        }
        for member in [
            ModelLoader::pModelMap,
            ModelLoader::pKFModelMap,
            ModelLoader::pAttachDistant3DTaskQueue,
            ModelLoader::pQueuedHelmetMap,
            ModelLoader::pQueuedReplacementKFListMap,
            ModelLoader::pQueuedAnimIdleMap,
            ModelLoader::pQueuedReferencesMap,
            ModelLoader::pReferencesToQueueMap,
            ModelLoader::pBackgroundCloneThread,
            ModelLoader::pQueuedTextureMap,
            ModelLoader::pLoadedFileMap,
        ] {
            let object = e.get(this, member);
            delete_object(e, object);
        }
        e.call(KF_ITERATOR_DESTRUCT, &args![iterator]);
    });
}

// Translated from 00443190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Model` method (no name in the engine map): the sum of `iRefCount` and
/// `iManualRefCount`.
pub fn fn_00443190(e: &mut Engine, this: Ptr<Model>) -> i32 {
    let count = e.get(this, Model::iRefCount);
    count.wrapping_add(e.get(this, Model::iManualRefCount))
}

// ---- Third batch: the model map report, the texture, model, KF and reference
// ---- queueing of the loader, the cancel functions and their helpers.

/// Destructor bodies the report's iterators and the deleting destructors use:
/// the iterator base of the model map (`00449940`, stores its virtual table
/// `01017294`) and of the KF map (`004499a0`, stores `010172ac`), and the
/// `Model` (`0043ab70`) and `KFModel` (`0043b750`) destructors.
const MODEL_ITERATOR_BASE_DESTRUCT: u32 = 0x0044_9940;
const KF_ITERATOR_BASE_DESTRUCT: u32 = 0x0044_99a0;
const MODEL_DESTRUCTOR_BODY: u32 = 0x0043_ab70;
const KF_MODEL_DESTRUCTOR_BODY: u32 = 0x0043_b750;
/// `"MODELS:\t%3d\t%3d\t%3d\t%s\r\n"` and `"ANIMATION:\t%3d\t%3d\t%3d\t%s\r\n"`:
/// the report lines of `OutputModelMapContents`.
const MODELS_REPORT_FORMAT: u32 = 0x0101_6ef0;
const ANIMATION_REPORT_FORMAT: u32 = 0x0101_6ed4;
/// `_strlen` of the CRT (`00ec6130`).
const CRT_STRING_LENGTH: u32 = 0x00ec_6130;
/// The game's string object (4 bytes): constructor (`004037b0`),
/// `Format(format, ...)` (`00406f60`, `__cdecl(string, format, ...)`) and the
/// release of its buffer (`004037d0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_FORMAT: u32 = 0x0040_6f60;
const STRING_RELEASE: u32 = 0x0040_37d0;
/// Size of the line buffer of the model report.
const REPORT_LINE_SIZE: u32 = 0x400;
/// Message sink slots used by the report: the sink's virtual function `0x48`
/// takes `(text, byte count)` and `0x38` takes `(string object, 1)`.
const SINK_WRITE_TEXT_SLOT: u32 = 0x48;
const SINK_WRITE_STRING_SLOT: u32 = 0x38;

/// `ModelLoader` method (no name in the engine map), `__thiscall(entry)`: the
/// task the loader's map of queued textures (`+0x20`) holds for a file entry,
/// or 0.
const LOADER_QUEUED_TEXTURE: u32 = 0x0044_8f80;
/// `QueuedTexture` (`modelloader.cpp`) methods: the test of the flag bit that
/// `0043c3d0` reads (`this + 0x34`), the setter of that bit
/// (`0043c0d0(value)`), the state getter (`0043c430`, 3 means loaded), the
/// thread flag test (`0043c130`, the byte at `+0x25c` of the TLS block) and
/// `BSTexturePalette::GetTexture` by file entry (Xbox PDB, `0043c4c0`,
/// `__cdecl(entry, NiPointer out)`).
const QUEUED_TEXTURE_FLAG_TEST: u32 = 0x0043_c3d0;
const QUEUED_TEXTURE_FLAG_SET: u32 = 0x0043_c0d0;
const QUEUED_TEXTURE_STATE: u32 = 0x0043_c430;
const THREAD_QUEUED_FLAG_TEST: u32 = 0x0043_c130;
const TEXTURE_PALETTE_GET_TEXTURE_BY_ENTRY: u32 = 0x0043_c4c0;
/// `QueuedFile` methods of `queuedfiles.obj`: `AddParent(parent)` (`00c3c7e0`,
/// appends to `pAdditionalParents`, +0x24) and `AddChild(child)` (`00c3c700`,
/// on the parent, appends to `pChildren`, +0x20).
const QUEUED_FILE_ADD_ADDITIONAL_PARENT: u32 = 0x00c3_c7e0;
const QUEUED_FILE_ADD_CHILD: u32 = 0x00c3_c700;
/// `IOManager` method `0044adb0` (no argument but the manager).
const IO_MANAGER_WAKE: u32 = 0x0044_adb0;
/// `BSTaskManager<__int64>::CancelTask` (Xbox PDB, `0044ac40`,
/// `__thiscall(task, 0)` on the manager at `01202d98`).
const TASK_MANAGER_CANCEL_TASK: u32 = 0x0044_ac40;
/// `QueuedTexture` constructors (0x38 bytes): from a texture
/// (`0043bef0(texture, priority)`), from a file entry (`0043be60(entry,
/// priority)`) and from a file name (`0043bd10(name, priority)`).
const QUEUED_TEXTURE_FROM_TEXTURE: u32 = 0x0043_bef0;
const QUEUED_TEXTURE_FROM_ENTRY: u32 = 0x0043_be60;
const QUEUED_TEXTURE_FROM_NAME: u32 = 0x0043_bd10;
/// `QueuedModel` constructors (0x48 bytes): from a `TESModel`
/// (`0043c890(model, priority, LOD fade multiplier, flag, flag)`), from a file
/// name (`0043c6e0`, the same arguments) and from a loaded `Model`
/// (`0043c960(model, priority)`).
const QUEUED_MODEL_FROM_TES_MODEL: u32 = 0x0043_c890;
const QUEUED_MODEL_FROM_NAME: u32 = 0x0043_c6e0;
const QUEUED_MODEL_FROM_MODEL: u32 = 0x0043_c960;
/// The other task constructors of the loader: `QueuedTree` model task
/// (0x50 bytes, `0043d850(reference, form, priority, LOD multiplier)`),
/// `QueuedKF` from a name (0x38 bytes, `0043e0f0(name, priority)`) and from a
/// loaded `KFModel` (`0043e210(model, priority)`), the 0x30-byte task
/// `0043db30(a, b, c)`, the 0x40-byte task `0043e580(a, c, b, d)`, the
/// 0x38-byte task `0043eaf0(a, c)` and the 0x128-byte `QueuedHelmet`
/// (`0043ef10(reference, a, b)`).
const QUEUED_TREE_MODEL_CONSTRUCT: u32 = 0x0043_d850;
const QUEUED_KF_FROM_NAME: u32 = 0x0043_e0f0;
const QUEUED_KF_FROM_MODEL: u32 = 0x0043_e210;
const TASK_CONSTRUCT_0043DB30: u32 = 0x0043_db30;
const TASK_CONSTRUCT_0043E580: u32 = 0x0043_e580;
const TASK_CONSTRUCT_0043EAF0: u32 = 0x0043_eaf0;
const QUEUED_HELMET_CONSTRUCT: u32 = 0x0043_ef10;
/// `QueuedModel` flag setter (`0044afb0(value, &flags)`, cdecl: sets or clears
/// bit `0x20` of the byte) and `NiPointer<QueuedModel>::operator=(NiPointer&)`
/// (`0092c820`, `__thiscall(other)`).
const QUEUED_MODEL_SET_FLAG_BIT_20: u32 = 0x0044_afb0;
const NI_POINTER_ASSIGN_POINTER: u32 = 0x0092_c820;
/// The `TESModel` of a form for a reference (`00446a60(form, reference)` on
/// the loader), the loader's priority getter (`0043cc60`) and the helmet key
/// getter (`0043f010`, the dword at `+0x2b0` of a reference).
const LOADER_GET_TES_MODEL: u32 = 0x0044_6a60;
const REFERENCE_HELMET_KEY: u32 = 0x0043_f010;
/// `TESObjectREFR` / `Actor` helpers: the getter `00891170`
/// (`this + 0x20`), the data handler flag tests (`0042ce10`: bit 2 and
/// `00444d20`: bit 8 of the dword at `+0x244` of the object at `011ddf38`),
/// the flag test `00444d00` and the menu-mode test `Interface::IsInMenuMode`
/// (Xbox PDB, `00702360`).
const REFERENCE_PROCESS_SLOT: u32 = 0x0089_1170;
const DATA_HANDLER_FLAG_BIT_2: u32 = 0x0042_ce10;
const IS_IN_MENU_MODE: u32 = 0x0070_2360;
/// The object at `011ddf38` (the one `0042ce10` reads the flags of).
const DATA_HANDLER: u32 = 0x011d_df38;
/// `ModelLoader::TryAndRemoveModel` (Xbox PDB, `00448920`,
/// `__thiscall(model, name)`), `Model::ModManualRefCount` (Xbox PDB,
/// `0043b410(delta)`) and the `Model` method `0043acb0` (the count-1 case of
/// `ReleaseModel`), and the KF model method `0043bac0`.
const TRY_AND_REMOVE_MODEL: u32 = 0x0044_8920;
const MODEL_MOD_MANUAL_REF_COUNT: u32 = 0x0043_b410;
const MODEL_RELEASE_ONE: u32 = 0x0043_acb0;
const KF_MODEL_PREPARE: u32 = 0x0043_bac0;
/// File path helpers of `archive.cpp`: `__cdecl(path, split piece, split
/// piece)` (`00afd270`) and `__cdecl(1, piece, piece, path)` that gives the
/// file entry (`00af6540`).
const SPLIT_FILE_PATH: u32 = 0x00af_d270;
const FIND_FILE_ENTRY: u32 = 0x00af_6540;
/// The cell predicate `005e3fc0` (the dword at `+0xa4` of the cell), the
/// iterator constructor of the queued references map (`0044cc10`), its
/// `next(iterator, &reference, &task, 1)` (`00906570`, on the map) and its
/// destructor (`004499c0`).
const CELL_QUEUED_COUNT_GETTER: u32 = 0x005e_3fc0;
const REFERENCE_ITERATOR_CONSTRUCT: u32 = 0x0044_cc10;
const REFERENCE_MAP_NEXT: u32 = 0x0090_6570;
const REFERENCE_ITERATOR_DESTRUCT: u32 = 0x0044_99c0;

/// The reference task constructors `QueueReference` picks from, each
/// `__thiscall(reference, priority)` (the player's takes the priority only):
/// `00441b20` (player), `00440e50` (type `0x25`), `00441440` (type `0x2a`),
/// `00441920` (type `0x2b`) and `0043fd40` (any other).
const REFERENCE_TASK_PLAYER: u32 = 0x0044_1b20;
const REFERENCE_TASK_TREE: u32 = 0x0044_0e50;
const REFERENCE_TASK_CREATURE: u32 = 0x0044_1440;
const REFERENCE_TASK_ACTOR: u32 = 0x0044_1920;
const REFERENCE_TASK_DEFAULT: u32 = 0x0043_fd40;
/// Source lines of the memory-context guards of `QueueReference`.
const QUEUE_REFERENCE_SOURCE_LINE: u32 = 0xc4e;
const QUEUE_REFERENCE_PLAYER_SOURCE_LINE: u32 = 0xc53;
const QUEUE_REFERENCE_CREATURE_SOURCE_LINE: u32 = 0xc6b;
const QUEUE_REFERENCE_ACTOR_SOURCE_LINE: u32 = 0xc72;
/// The visual distance `00444dc0` passes for a reference that `00564f00`
/// accepts (a float at `01016970`), and the size of the path buffer.
const VISUAL_DISTANCE_LIMIT: u32 = 0x0101_6970;
const REPORT_PATH_SIZE: u32 = 0x104;

/// The task pointer (`NiPointer`) holding `task` on the stack for the duration
/// of `body`, destructed afterwards (`00528cb0`, `0044cbf0`).
fn with_task_pointer<R>(e: &mut Engine, task: u32, body: impl FnOnce(&mut Engine, Ptr) -> R) -> R {
    e.with_stack(4, |e, holder| {
        e.call(TASK_POINTER_CONSTRUCT, &args![holder, task]);
        let result = body(e, holder);
        e.call(TASK_POINTER_DESTRUCT, &args![holder]);
        result
    })
}

/// A new object of `size` bytes from the memory manager, built by `build`
/// (the constructor, called with the block); 0 when the allocation gave 0.
fn construct_object(e: &mut Engine, size: u32, build: impl FnOnce(&mut Engine, u32) -> u32) -> u32 {
    let block = e.call(MEMORY_ALLOC, &args![size]).u32();
    if block == 0 {
        0
    } else {
        build(e, block)
    }
}

/// `holder.get()->AddParent...`: the pointer in `holder` is given `parent`
/// as its parent task ([`fn_00443aa0`]).
fn set_task_parent(e: &mut Engine, holder: Ptr, parent: Ptr) {
    let task = ni_pointer_value(e, holder);
    fn_00443aa0(e, Ptr::new(task), parent);
}

/// Virtual function `slot` of the task in `holder`.
fn call_task_slot(e: &mut Engine, holder: Ptr, slot: u32) {
    let task = ni_pointer_value(e, holder);
    e.vcall(task, slot, &args![]);
}

/// The map's `Find(key, &out)` (virtual function `8`).
fn map_find(e: &mut Engine, map: Ptr, key: u32, out: Ptr) -> bool {
    e.vcall(map.addr(), 0x8, &args![key, out]).bool()
}

/// The map's `Add(key, &value, flag)` (virtual function `0x10`).
fn map_add(e: &mut Engine, map: Ptr, key: u32, value: Ptr, flag: u32) -> bool {
    e.vcall(map.addr(), 0x10, &args![key, value, flag]).bool()
}

// Translated from 004431b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `KFModel` method (no name in the engine map): the sum of `iRefCount` and
/// `iManualRefCount`.
pub fn fn_004431b0(e: &mut Engine, this: Ptr<KFModel>) -> i32 {
    let count = e.get(this, KFModel::iRefCount);
    count.wrapping_add(e.get(this, KFModel::iManualRefCount))
}

// Translated from 004431d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the iterator of the model map (no name in the engine map):
/// the base destructor `00449940`, which stores the base virtual table.
pub fn fn_004431d0(e: &mut Engine, this: Ptr) {
    e.call(MODEL_ITERATOR_BASE_DESTRUCT, &args![this]);
}

// Translated from 004431f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Model` scalar deleting destructor (no name in the engine map): the model
/// destructor body (`0043ab70`) and, when bit 0 of `flags` is set, the free of
/// the block. Returns `this`.
pub fn fn_004431f0(e: &mut Engine, this: Ptr<Model>, flags: u32) -> Ptr<Model> {
    e.call(MODEL_DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 00443220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the iterator of the KF model map (no name in the engine
/// map): the base destructor `004499a0`, which stores the base virtual table.
pub fn fn_00443220(e: &mut Engine, this: Ptr) {
    e.call(KF_ITERATOR_BASE_DESTRUCT, &args![this]);
}

// Translated from 00443240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `KFModel` scalar deleting destructor (no name in the engine map): the KF
/// model destructor body (`0043b750`) and, when bit 0 of `flags` is set, the
/// free of the block. Returns `this`.
pub fn fn_00443240(e: &mut Engine, this: Ptr<KFModel>, flags: u32) -> Ptr<KFModel> {
    e.call(KF_MODEL_DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 00443270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::OutputModelMapContents` (Xbox PDB): writes a line per
/// loaded model and per loaded KF model to the log (`005b5e40`) and, when
/// `sink` is not 0, to the sink (its virtual function `0x48` with the text
/// and its length plus one for models, `0x38` with the string object and 1
/// for animations). Models are listed when their count ([`fn_00443190`]) is
/// above 0 or `show_all` is not 0, as `"MODELS:\t%3d\t%3d\t%3d\t%s\r\n"` with
/// the sum, the `iRefCount` (`+4`, the getter `00726070`), the
/// `iManualRefCount` (`+8`, `0044ddc0`) and the name; KF models only when
/// their count ([`fn_004431b0`]) is above 0, as `"ANIMATION:\t..."` with the
/// sum, `iRefCount` (`0084e3a0`), `iManualRefCount` (`0044edb0`) and the
/// name. Each map is walked only when its virtual function `0x44` is not 0.
///
/// The exception-unwinding frame and the stack-cookie check are not
/// translated.
pub fn model_loader_output_model_map_contents(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    sink: Ptr,
    show_all: u8,
) {
    let model_map = e.get(this, ModelLoader::pModelMap);
    if e.vcall(model_map.addr(), 0x44, &args![]).u32() != 0 {
        e.with_stack(MAP_ITERATOR_SIZE, |e, iterator| {
            e.call(MODEL_ITERATOR_CONSTRUCT, &args![iterator]);
            while !e.call(ITERATOR_AT_END, &args![iterator]).bool() {
                let model_map = e.get(this, ModelLoader::pModelMap);
                let Some((key, model)) = map_next(e, model_map, iterator) else {
                    continue;
                };
                let model: Ptr = Ptr::new(model);
                if fn_00443190(e, model.cast()) > 0 || show_all != 0 {
                    let manual = e.call(ARRAY_COUNT_AT_8, &args![model]).u32();
                    let count = e.call(LIST_NODE_NEXT, &args![model]).u32();
                    let sum = fn_00443190(e, model.cast());
                    e.with_stack(REPORT_LINE_SIZE, |e, line| {
                        e.call(
                            FORMAT_STRING,
                            &args![
                                line,
                                REPORT_LINE_SIZE,
                                MODELS_REPORT_FORMAT,
                                sum,
                                count,
                                manual,
                                key
                            ],
                        );
                        e.call(LOG_MESSAGE, &args![line]);
                        if !sink.is_null() {
                            let length = e.call(CRT_STRING_LENGTH, &args![line]).u32();
                            e.vcall(
                                sink.addr(),
                                SINK_WRITE_TEXT_SLOT,
                                &args![line, length.wrapping_add(1)],
                            );
                        }
                    });
                }
            }
            fn_004431d0(e, iterator);
        });
    }
    let kf_map = e.get(this, ModelLoader::pKFModelMap);
    if e.vcall(kf_map.addr(), 0x44, &args![]).u32() != 0 {
        e.with_stack(MAP_ITERATOR_SIZE, |e, iterator| {
            e.call(KF_ITERATOR_CONSTRUCT, &args![iterator]);
            while !e.call(ITERATOR_AT_END, &args![iterator]).bool() {
                let kf_map = e.get(this, ModelLoader::pKFModelMap);
                let Some((key, model)) = map_next(e, kf_map, iterator) else {
                    continue;
                };
                let model: Ptr = Ptr::new(model);
                if fn_004431b0(e, model.cast()) > 0 {
                    // A `BSString` local (8 bytes: the text, its length and capacity, `004037b0`).
                    e.with_stack(8, |e, text| {
                        e.call(STRING_CONSTRUCT, &args![text]);
                        let manual = e.call(OWNER_THREAD_ID, &args![model]).u32();
                        let count = e.call(TASK_GET_STATE, &args![model]).u32();
                        let sum = fn_004431b0(e, model.cast());
                        e.call(
                            STRING_FORMAT,
                            &args![text, ANIMATION_REPORT_FORMAT, sum, count, manual, key],
                        );
                        let line = ni_pointer_value(e, text);
                        e.call(LOG_MESSAGE, &args![line]);
                        if !sink.is_null() {
                            e.vcall(sink.addr(), SINK_WRITE_STRING_SLOT, &args![text, 1u32]);
                        }
                        e.call(STRING_RELEASE, &args![text]);
                    });
                }
            }
            fn_00443220(e, iterator);
        });
    }
}

// Translated from 00443540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): hands a texture that is
/// already queued for the file `entry` (`00448f80`) over to a new owner. The
/// task must exist and be in a state below 3; its state is moved to 2
/// (`00449190(state, 2)`). A task whose flag `0043c3d0` is set and while the
/// thread flag `0043c130` is clear gets that flag cleared (`0043c0d0(0)`). A
/// `parent` is added as an additional parent (`00c3c7e0`). The state is then
/// moved back to the old one (`00449190(2, state)`); if that worked, a task
/// whose priority is above `priority` is requeued (virtual function `0x1c`),
/// otherwise the IO manager is woken (`0044adb0`). Returns whether the task
/// was taken over.
///
/// The exception-unwinding frame is not translated.
pub fn fn_00443540(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    entry: u32,
    priority: i32,
    parent: Ptr,
) -> bool {
    let queued = e.call(LOADER_QUEUED_TEXTURE, &args![this, entry]).u32();
    with_task_pointer(e, queued, |e, holder| {
        if ni_pointer_value(e, holder) == 0 {
            return false;
        }
        let task = ni_pointer_value(e, holder);
        let state = e.call(TASK_GET_STATE, &args![task]).i32();
        if state >= 3 {
            return false;
        }
        let task = ni_pointer_value(e, holder);
        if !e.call(TASK_CHANGE_STATE, &args![task, state, 2u32]).bool() {
            return false;
        }
        let task = ni_pointer_value(e, holder);
        if e.call(QUEUED_TEXTURE_FLAG_TEST, &args![task]).bool()
            && !e.call(THREAD_QUEUED_FLAG_TEST, &args![this]).bool()
        {
            let task = ni_pointer_value(e, holder);
            e.call(QUEUED_TEXTURE_FLAG_SET, &args![task, 0u32]);
        }
        if !parent.is_null() {
            let task = ni_pointer_value(e, holder);
            e.call(QUEUED_FILE_ADD_ADDITIONAL_PARENT, &args![task, parent]);
        }
        let task = ni_pointer_value(e, holder);
        if !e.call(TASK_CHANGE_STATE, &args![task, 2u32, state]).bool() {
            return false;
        }
        let task = ni_pointer_value(e, holder);
        let current = task_priority(e, Ptr::new(task)) as i32;
        if current > priority {
            let task = ni_pointer_value(e, holder);
            e.vcall(task, 0x1c, &args![priority]);
        } else {
            let manager = e.global::<u32>(TASK_QUEUE);
            e.call(IO_MANAGER_WAKE, &args![manager]);
        }
        true
    })
}

/// The tail both `QueueTexture` overloads share for a texture that is already
/// loaded: without a `parent` nothing is queued (false); otherwise a
/// `QueuedTexture` is built from the texture (`0043bef0`), given the parent
/// and finished by its virtual function `0x28` (`CheckFinished`).
fn queue_loaded_texture(e: &mut Engine, texture: Ptr, priority: i32, parent: Ptr) -> bool {
    if parent.is_null() {
        return false;
    }
    let object = construct_object(e, 0x38, |e, block| {
        let texture = ni_pointer_value(e, texture);
        e.call(
            QUEUED_TEXTURE_FROM_TEXTURE,
            &args![block, texture, priority],
        )
        .u32()
    });
    with_task_pointer(e, object, |e, holder| {
        set_task_parent(e, holder, parent);
        call_task_slot(e, holder, 0x28);
    });
    true
}

/// Whether the texture in `texture` is loaded (state 3) while the thread flag
/// `0043c130` is clear.
fn texture_is_loaded_for_this_thread(e: &mut Engine, this: Ptr, texture: Ptr) -> bool {
    let task = ni_pointer_value(e, texture);
    e.call(QUEUED_TEXTURE_STATE, &args![task]).u32() == 3
        && !e.call(THREAD_QUEUED_FLAG_TEST, &args![this]).bool()
}

// Translated from 004436c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::QueueTexture` (Xbox PDB), the overload by file name: the
/// texture palette is asked for the texture of `name` (`00a61b90`). A texture
/// found there that is not (loaded and the thread flag clear) is queued by
/// [`queue_loaded_texture`]. Otherwise the name is normalised to a path
/// (`00af4200`, 0x104 bytes) and split (`00afd270`), the file entry looked up
/// (`00af6540`) and, when found, [`fn_00443540`] may take over a task that is
/// already queued for it. If not, a `QueuedTexture` is built from the entry
/// (`0043be60`, the path set as its file name by `00c3cee0`) or, with no
/// entry, from the name (`0043bd10`), given `parent` and queued with the
/// virtual function `0x20` (`QueueMe`). Returns whether a task was queued or
/// taken over.
///
/// The exception-unwinding frame and the stack-cookie check are not
/// translated.
pub fn model_loader_queue_texture(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    name: u32,
    priority: i32,
    parent: Ptr,
) -> bool {
    e.with_stack(4, |e, texture| {
        e.call(NI_POINTER_CONSTRUCT, &args![texture, 0u32]);
        e.call(TEXTURE_PALETTE_GET_TEXTURE_BY_NAME, &args![name, texture]);
        let result = queue_texture_by_name(e, this, name, priority, parent, texture);
        e.call(NI_POINTER_DESTRUCT, &args![texture]);
        result
    })
}

fn queue_texture_by_name(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    name: u32,
    priority: i32,
    parent: Ptr,
    texture: Ptr,
) -> bool {
    if ni_pointer_value(e, texture) != 0 {
        let loaded = texture_is_loaded_for_this_thread(e, this.cast(), texture);
        if !loaded {
            return queue_loaded_texture(e, texture, priority, parent);
        }
    }
    e.with_stack(REPORT_PATH_SIZE, |e, path| {
        e.call(NORMALIZE_PATH, &args![name, path, REPORT_PATH_SIZE]);
        e.with_stack(8, |e, first| {
            e.with_stack(8, |e, second| {
                // `006815c0` gives the address of each piece back.
                e.call(LIST_NODE_ITEM_ADDRESS, &args![first]);
                e.call(LIST_NODE_ITEM_ADDRESS, &args![second]);
                e.call(SPLIT_FILE_PATH, &args![path, first, second]);
                let entry = e
                    .call(FIND_FILE_ENTRY, &args![1u32, first, second, path])
                    .u32();
                if entry != 0 && fn_00443540(e, this, entry, priority, parent) {
                    return true;
                }
                with_task_pointer(e, 0, |e, holder| {
                    if entry == 0 {
                        let object = construct_object(e, 0x38, |e, block| {
                            e.call(QUEUED_TEXTURE_FROM_NAME, &args![block, name, priority])
                                .u32()
                        });
                        e.call(TASK_POINTER_ASSIGN, &args![holder, object]);
                    } else {
                        let object = construct_object(e, 0x38, |e, block| {
                            e.call(QUEUED_TEXTURE_FROM_ENTRY, &args![block, entry, priority])
                                .u32()
                        });
                        e.call(TASK_POINTER_ASSIGN, &args![holder, object]);
                        let task = ni_pointer_value(e, holder);
                        e.call(QUEUED_FILE_ENTRY_SET_FILE_NAME, &args![task, path]);
                    }
                    set_task_parent(e, holder, parent);
                    call_task_slot(e, holder, 0x20);
                });
                true
            })
        })
    })
}

// Translated from 00443aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedFile` method (no name in the engine map): stores `parent` in
/// `spParent` (Xbox PDB, `+0x1c`) and, when it is not 0, adds this task to
/// the parent's children (`00c3c700`).
pub fn fn_00443aa0(e: &mut Engine, this: Ptr<QueuedFile>, parent: Ptr) {
    // QueuedFile::spParent (Xbox PDB) +0x1C
    let slot = this.byte_add(0x1c);
    e.call(TASK_POINTER_ASSIGN, &args![slot, parent]);
    if ni_pointer_value(e, slot) != 0 {
        let parent = ni_pointer_value(e, slot);
        e.call(QUEUED_FILE_ADD_CHILD, &args![parent, this]);
    }
}

// Translated from 00443af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::QueueTexture` (Xbox PDB), the overload by file entry: as
/// [`model_loader_queue_texture`], with the texture found by entry
/// (`BSTexturePalette::GetTexture`, `0043c4c0`); with no usable texture,
/// [`fn_00443540`] may take over a queued task, otherwise a `QueuedTexture` is
/// built from the entry (`0043be60`), given `parent` and queued with the
/// virtual function `0x20`. Returns whether a task was queued or taken over.
///
/// The exception-unwinding frame is not translated.
pub fn model_loader_queue_texture_ov2(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    entry: u32,
    priority: i32,
    parent: Ptr,
) -> bool {
    e.with_stack(4, |e, texture| {
        e.call(NI_POINTER_CONSTRUCT, &args![texture, 0u32]);
        e.call(TEXTURE_PALETTE_GET_TEXTURE_BY_ENTRY, &args![entry, texture]);
        let result = (|| {
            if ni_pointer_value(e, texture) != 0 {
                let loaded = texture_is_loaded_for_this_thread(e, this.cast(), texture);
                if !loaded {
                    return queue_loaded_texture(e, texture, priority, parent);
                }
            }
            if fn_00443540(e, this, entry, priority, parent) {
                return true;
            }
            let object = construct_object(e, 0x38, |e, block| {
                e.call(QUEUED_TEXTURE_FROM_ENTRY, &args![block, entry, priority])
                    .u32()
            });
            with_task_pointer(e, object, |e, holder| {
                set_task_parent(e, holder, parent);
                call_task_slot(e, holder, 0x20);
            });
            true
        })();
        e.call(NI_POINTER_DESTRUCT, &args![texture]);
        result
    })
}

// Translated from 00443d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): [`fn_00443dc0`] with a
/// null result pointer of its own and a visual distance of 0.
#[allow(clippy::too_many_arguments)]
pub fn fn_00443d30(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    tes_model: Ptr,
    priority: u32,
    parent: Ptr,
    lod_fade_mult: u32,
    flag_first: u8,
    flag_second: u8,
    flag_third: u8,
) {
    with_task_pointer(e, 0, |e, holder| {
        fn_00443dc0(
            e,
            this,
            tes_model,
            holder,
            priority,
            parent,
            lod_fade_mult,
            flag_first,
            flag_second,
            flag_third,
            0.0,
        );
    });
}

// Translated from 00443dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map), queueing the model of a
/// `TESModel` into the `NiPointer` at `out`. The model's name is the result of
/// the `TESModel` virtual function `0x14`; the loader's model map is asked
/// for it (virtual function `8`, which also gives the loaded `Model`).
///
/// Not loaded yet: a `QueuedModel` (0x48 bytes) is built from the `TESModel`
/// (`0043c890(tes_model, priority, lod_fade_mult, flag_first, flag_second)`),
/// its visual distance set ([`fn_00444020`]), `parent` added ([`fn_00443aa0`]),
/// bit `0x20` of its flags set to `flag_third` ([`fn_00443ff0`]) and it is
/// queued with the virtual function `0x20`; the task goes to `out`
/// (`0092c820`).
///
/// Already loaded: `flag_second` gives the model a reference
/// (`0092c870`); with a `parent`, a `QueuedModel` is built from the loaded
/// model (`0043c960(model, priority)`), given the visual distance and the
/// parent, finished with the virtual function `0x28` (`CheckFinished`) and
/// stored in `out`; without a parent `out` is cleared.
///
/// The exception-unwinding frame is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_00443dc0(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    tes_model: Ptr,
    out: Ptr,
    priority: u32,
    parent: Ptr,
    lod_fade_mult: u32,
    flag_first: u8,
    flag_second: u8,
    flag_third: u8,
    visual_distance: f32,
) {
    e.with_stack(4, |e, loaded| {
        e.mem.set_u32(loaded.addr(), 0);
        let name = e.vcall(tes_model.addr(), 0x14, &args![]).u32();
        let model_map = e.get(this, ModelLoader::pModelMap);
        if !map_find(e, model_map, name, loaded) {
            let object = construct_object(e, 0x48, |e, block| {
                e.call(
                    QUEUED_MODEL_FROM_TES_MODEL,
                    &args![
                        block,
                        tes_model,
                        priority,
                        lod_fade_mult,
                        flag_first,
                        flag_second
                    ],
                )
                .u32()
            });
            with_task_pointer(e, object, |e, holder| {
                let task = ni_pointer_value(e, holder);
                fn_00444020(e, Ptr::new(task), visual_distance);
                set_task_parent(e, holder, parent);
                let task = ni_pointer_value(e, holder);
                fn_00443ff0(e, Ptr::new(task), flag_third);
                call_task_slot(e, holder, 0x20);
                e.call(NI_POINTER_ASSIGN_POINTER, &args![out, holder]);
            });
        } else {
            let model = e.mem.u32(loaded.addr());
            if flag_second != 0 {
                e.call(ADD_REFERENCE, &args![model]);
            }
            if parent.is_null() {
                e.call(TASK_POINTER_ASSIGN, &args![out, 0u32]);
            } else {
                let object = construct_object(e, 0x48, |e, block| {
                    e.call(QUEUED_MODEL_FROM_MODEL, &args![block, model, priority])
                        .u32()
                });
                with_task_pointer(e, object, |e, holder| {
                    let task = ni_pointer_value(e, holder);
                    fn_00444020(e, Ptr::new(task), visual_distance);
                    set_task_parent(e, holder, parent);
                    call_task_slot(e, holder, 0x28);
                    e.call(NI_POINTER_ASSIGN_POINTER, &args![out, holder]);
                });
            }
        }
    });
}

// Translated from 00443ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedModel` method (no name in the engine map): sets or clears bit `0x20`
/// of `cFlags` (`0044afb0(value, &cFlags)`).
pub fn fn_00443ff0(e: &mut Engine, this: Ptr<QueuedModel>, value: u8) {
    let flags = this.byte_add(QueuedModel::cFlags.off);
    e.call(QUEUED_MODEL_SET_FLAG_BIT_20, &args![value, flags]);
}

// Translated from 00444020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedModel` setter (no name in the engine map): stores
/// `mfOverriddenVisualDistance` (Xbox PDB).
pub fn fn_00444020(e: &mut Engine, this: Ptr<QueuedModel>, distance: f32) {
    e.set(
        this,
        QueuedModel::mfOverriddenVisualDistance,
        x87_quiet(distance),
    );
}

// Translated from 00444040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::QueueModel` (Xbox PDB): [`fn_004440d0`] for a model file
/// `name`, with a null result pointer of its own.
#[allow(clippy::too_many_arguments)]
pub fn model_loader_queue_model(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    name: u32,
    priority: u32,
    parent: Ptr,
    lod_fade_mult: u32,
    flag_first: u8,
    flag_second: u8,
    flag_third: u8,
) {
    with_task_pointer(e, 0, |e, holder| {
        fn_004440d0(
            e,
            this,
            name,
            holder,
            priority,
            parent,
            lod_fade_mult,
            flag_first,
            flag_second,
            flag_third,
        );
    });
}

// Translated from 004440d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): queues the model file
/// `name` into the `NiPointer` at `out`, like [`fn_00443dc0`] but keyed by the
/// name itself. Not loaded: a `QueuedModel` is built from the name
/// (`0043c6e0(name, priority, lod_fade_mult, flag_first, flag_second)`), bit
/// `0x20` of its flags set to `flag_third`, `parent` added and the task queued
/// (virtual function `0x20`); it goes to `out`. Already loaded:
/// `flag_second` adds a reference to the model (`0092c870`); with a `parent`
/// a `QueuedModel` is built from it (`0043c960`), given the parent, finished
/// (virtual function `0x28`) and stored in `out`, without a parent `out` is
/// cleared.
///
/// The exception-unwinding frame is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_004440d0(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    name: u32,
    out: Ptr,
    priority: u32,
    parent: Ptr,
    lod_fade_mult: u32,
    flag_first: u8,
    flag_second: u8,
    flag_third: u8,
) {
    queue_model_by_name(
        e,
        this,
        name,
        out,
        priority,
        parent,
        lod_fade_mult,
        flag_first,
        flag_second,
        flag_third,
        None,
    );
}

/// The body shared by [`fn_004440d0`] and [`fn_00444350`]; `extra` is the
/// argument the second one gives to the new task's virtual function `0x30`
/// (which the first one does not call: it uses `0x20`).
#[allow(clippy::too_many_arguments)]
fn queue_model_by_name(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    name: u32,
    out: Ptr,
    priority: u32,
    parent: Ptr,
    lod_fade_mult: u32,
    flag_first: u8,
    flag_second: u8,
    flag_third: u8,
    extra: Option<u32>,
) {
    e.with_stack(4, |e, loaded| {
        e.mem.set_u32(loaded.addr(), 0);
        let model_map = e.get(this, ModelLoader::pModelMap);
        if !map_find(e, model_map, name, loaded) {
            let object = construct_object(e, 0x48, |e, block| {
                e.call(
                    QUEUED_MODEL_FROM_NAME,
                    &args![
                        block,
                        name,
                        priority,
                        lod_fade_mult,
                        flag_first,
                        flag_second
                    ],
                )
                .u32()
            });
            with_task_pointer(e, object, |e, holder| {
                let task = ni_pointer_value(e, holder);
                fn_00443ff0(e, Ptr::new(task), flag_third);
                set_task_parent(e, holder, parent);
                let task = ni_pointer_value(e, holder);
                match extra {
                    None => {
                        e.vcall(task, 0x20, &args![]);
                    }
                    Some(extra) => {
                        e.vcall(task, 0x30, &args![extra]);
                    }
                }
                e.call(NI_POINTER_ASSIGN_POINTER, &args![out, holder]);
            });
        } else {
            let model = e.mem.u32(loaded.addr());
            if flag_second != 0 {
                e.call(ADD_REFERENCE, &args![model]);
            }
            if parent.is_null() {
                e.call(TASK_POINTER_ASSIGN, &args![out, 0u32]);
            } else {
                let object = construct_object(e, 0x48, |e, block| {
                    e.call(QUEUED_MODEL_FROM_MODEL, &args![block, model, priority])
                        .u32()
                });
                with_task_pointer(e, object, |e, holder| {
                    set_task_parent(e, holder, parent);
                    call_task_slot(e, holder, 0x28);
                    e.call(NI_POINTER_ASSIGN_POINTER, &args![out, holder]);
                });
            }
        }
    });
}

// Translated from 004442c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): [`fn_00444350`] with a
/// null result pointer of its own.
#[allow(clippy::too_many_arguments)]
pub fn fn_004442c0(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    name: u32,
    extra: u32,
    priority: u32,
    parent: Ptr,
    lod_fade_mult: u32,
    flag_first: u8,
    flag_second: u8,
    flag_third: u8,
) {
    with_task_pointer(e, 0, |e, holder| {
        fn_00444350(
            e,
            this,
            name,
            extra,
            holder,
            priority,
            parent,
            lod_fade_mult,
            flag_first,
            flag_second,
            flag_third,
        );
    });
}

// Translated from 00444350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): [`fn_004440d0`] for a
/// new task that is started with its virtual function `0x30(extra)` instead
/// of `0x20` (the other branches, the already-loaded one included, are the
/// same).
#[allow(clippy::too_many_arguments)]
pub fn fn_00444350(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    name: u32,
    extra: u32,
    out: Ptr,
    priority: u32,
    parent: Ptr,
    lod_fade_mult: u32,
    flag_first: u8,
    flag_second: u8,
    flag_third: u8,
) {
    queue_model_by_name(
        e,
        this,
        name,
        out,
        priority,
        parent,
        lod_fade_mult,
        flag_first,
        flag_second,
        flag_third,
        Some(extra),
    );
}

// Translated from 00444540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): [`fn_004445c0`] with a
/// null result pointer of its own.
pub fn fn_00444540(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    reference: Ptr,
    form: Ptr,
    priority: u32,
    parent: Ptr,
    lod_mult: u32,
) {
    with_task_pointer(e, 0, |e, holder| {
        fn_004445c0(e, this, reference, form, holder, priority, parent, lod_mult);
    });
}

// Translated from 004445c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): builds the tree model
/// task (0x50 bytes, `0043d850(reference, form, priority, lod_mult)`), gives
/// it `parent`, queues it with the virtual function `0x20` and stores it in
/// the `NiPointer` at `out`.
#[allow(clippy::too_many_arguments)]
pub fn fn_004445c0(
    e: &mut Engine,
    _this: Ptr<ModelLoader>,
    reference: Ptr,
    form: Ptr,
    out: Ptr,
    priority: u32,
    parent: Ptr,
    lod_mult: u32,
) {
    let object = construct_object(e, 0x50, |e, block| {
        e.call(
            QUEUED_TREE_MODEL_CONSTRUCT,
            &args![block, reference, form, priority, lod_mult],
        )
        .u32()
    });
    with_task_pointer(e, object, |e, holder| {
        set_task_parent(e, holder, parent);
        call_task_slot(e, holder, 0x20);
        e.call(NI_POINTER_ASSIGN_POINTER, &args![out, holder]);
    });
}

// Translated from 004446a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): queues the KF file
/// `name` through the loader's map of KF models (`+4`). Not loaded: a
/// `QueuedKF` is built from the name (`0043e0f0(name, priority)`), given
/// `parent` and queued with the virtual function `0x20`; true. Already loaded:
/// without a `parent` false; otherwise a `QueuedKF` is built from the loaded
/// KF model (`0043e210`), given the parent and finished with the virtual
/// function `0x28`; true.
///
/// The exception-unwinding frame is not translated.
pub fn fn_004446a0(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    name: u32,
    priority: u32,
    parent: Ptr,
) -> bool {
    e.with_stack(4, |e, loaded| {
        e.mem.set_u32(loaded.addr(), 0);
        let kf_map = e.get(this, ModelLoader::pKFModelMap);
        if !map_find(e, kf_map, name, loaded) {
            let object = construct_object(e, 0x38, |e, block| {
                e.call(QUEUED_KF_FROM_NAME, &args![block, name, priority])
                    .u32()
            });
            with_task_pointer(e, object, |e, holder| {
                set_task_parent(e, holder, parent);
                call_task_slot(e, holder, 0x20);
            });
            true
        } else if parent.is_null() {
            false
        } else {
            let model = e.mem.u32(loaded.addr());
            let object = construct_object(e, 0x38, |e, block| {
                e.call(QUEUED_KF_FROM_MODEL, &args![block, model, priority])
                    .u32()
            });
            with_task_pointer(e, object, |e, holder| {
                set_task_parent(e, holder, parent);
                call_task_slot(e, holder, 0x28);
            });
            true
        }
    })
}

// Translated from 00444850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::QueueReference` (Xbox PDB): queues the 3D of a reference.
/// Nothing happens when the data handler's flag `0042ce10` is set and its
/// flag `00444d20` is clear, when the base form is of type `0xd`, or when the
/// reference's virtual function `0x224` says yes and its flag `00444d00(2)`
/// is set.
///
/// A task already in the loader's map of queued references (`+8`) is only
/// requeued with the new `priority` when its own differs (virtual function
/// `0x1c`). Otherwise, inside the memory context `0x31` (source line
/// `0xc4e`), the task is built by the kind of the reference: the player
/// (global `011dea3c`, context `0x34`) `00441b20`; base form type `0x25`
/// `00440e50`; `0x2a` (context `0x32`) `00441440`; `0x2b` (context `0x32`)
/// `00441920`; any other `0043fd40`. It is added to the queued map
/// (virtual function `0x10` with flag 0); if that refused, the task pointer is
/// cleared. Otherwise, when `on_loader_thread_only` is set, the thread is the
/// owner thread (`0044edb0` of `011dea0c`) and not in menu mode (`00702360`),
/// the task goes to the second map (`+0xc`, flag 1) instead of being
/// started; else it is started with its virtual function `0x20`. The parent
/// cell then counts the reference (`00444cc0`), and a reference with critical
/// 3D also counts it as critical (`00444ce0`).
///
/// The exception-unwinding frame is not translated.
pub fn model_loader_queue_reference(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    reference: Ptr,
    priority: i32,
    on_loader_thread_only: u8,
) {
    let handler = e.global::<u32>(DATA_HANDLER);
    if e.call(DATA_HANDLER_FLAG_BIT_2, &args![handler]).bool() && !fn_00444d20(e, Ptr::new(handler))
    {
        return;
    }
    let form = e.call(GET_BASE_FORM, &args![reference]).u32();
    if e.call(FORM_TYPE, &args![form]).u32() == 0xd {
        return;
    }
    if e.vcall(reference.addr(), 0x224, &args![]).bool() && fn_00444d00(e, reference, 2) {
        return;
    }
    with_task_pointer(e, 0, |e, holder| {
        let queued_map = e.get(this, ModelLoader::pQueuedReferencesMap);
        if map_find(e, queued_map, reference.addr(), holder) {
            let task = ni_pointer_value(e, holder);
            let current = task_priority(e, Ptr::new(task)) as i32;
            if current != priority {
                let task = ni_pointer_value(e, holder);
                e.vcall(task, 0x1c, &args![priority]);
            }
            return;
        }
        in_context(e, 0x31, QUEUE_REFERENCE_SOURCE_LINE, |e| {
            let object = if reference.addr() == e.global::<u32>(SPECIAL_REFERENCE) {
                in_context(e, 0x34, QUEUE_REFERENCE_PLAYER_SOURCE_LINE, |e| {
                    construct_object(e, 0x48, |e, block| {
                        e.call(REFERENCE_TASK_PLAYER, &args![block, priority]).u32()
                    })
                })
            } else {
                let form = e.call(GET_BASE_FORM, &args![reference]).u32();
                match e.call(FORM_TYPE, &args![form]).u32() {
                    0x25 => construct_object(e, 0x40, |e, block| {
                        e.call(REFERENCE_TASK_TREE, &args![block, reference, priority])
                            .u32()
                    }),
                    0x2a => in_context(e, 0x32, QUEUE_REFERENCE_CREATURE_SOURCE_LINE, |e| {
                        construct_object(e, 0x48, |e, block| {
                            e.call(REFERENCE_TASK_CREATURE, &args![block, reference, priority])
                                .u32()
                        })
                    }),
                    0x2b => in_context(e, 0x32, QUEUE_REFERENCE_ACTOR_SOURCE_LINE, |e| {
                        construct_object(e, 0x40, |e, block| {
                            e.call(REFERENCE_TASK_ACTOR, &args![block, reference, priority])
                                .u32()
                        })
                    }),
                    _ => construct_object(e, 0x40, |e, block| {
                        e.call(REFERENCE_TASK_DEFAULT, &args![block, reference, priority])
                            .u32()
                    }),
                }
            };
            e.call(TASK_POINTER_ASSIGN, &args![holder, object]);
            let queued_map = e.get(this, ModelLoader::pQueuedReferencesMap);
            if !map_add(e, queued_map, reference.addr(), holder, 0) {
                e.call(TASK_POINTER_ASSIGN, &args![holder, 0u32]);
            } else {
                let mut on_thread = false;
                if on_loader_thread_only != 0 {
                    let thread = e.call(CURRENT_THREAD_ID, &args![]).u32();
                    let owner_object = e.global::<u32>(THREAD_OWNER_OBJECT);
                    let owner = e.call(OWNER_THREAD_ID, &args![owner_object]).u32();
                    if thread == owner && !e.call(IS_IN_MENU_MODE, &args![]).bool() {
                        on_thread = true;
                    }
                }
                if on_thread {
                    let later_map = e.get(this, ModelLoader::pReferencesToQueueMap);
                    map_add(e, later_map, reference.addr(), holder, 1);
                } else {
                    call_task_slot(e, holder, 0x20);
                }
                let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
                if cell != 0 {
                    let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
                    fn_00444cc0(e, Ptr::new(cell));
                    if e.call(REFERENCE_IS_3D_CRITICAL, &args![reference]).bool() {
                        let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
                        fn_00444ce0(e, Ptr::new(cell));
                    }
                }
            }
        });
    });
}

// Translated from 00444cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL` method (no name in the engine map): counts one more
/// `iQueuedRefCount` (Xbox PDB, an interlocked increment).
pub fn fn_00444cc0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    e.call(
        INTERLOCKED_INCREMENT,
        &args![this.byte_add(TESObjectCELL::iQueuedRefCount.off)],
    );
}

// Translated from 00444ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectCELL` method (no name in the engine map): counts one more
/// `iCriticalQueuedRefCount` (Xbox PDB, an interlocked increment).
pub fn fn_00444ce0(e: &mut Engine, this: Ptr<TESObjectCELL>) {
    e.call(
        INTERLOCKED_INCREMENT,
        &args![this.byte_add(TESObjectCELL::iCriticalQueuedRefCount.off)],
    );
}

// Translated from 00444d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flag test (no name in the engine map) on the dword at `+0xc8` of a
/// reference-derived object: whether any bit of `mask` is set. The class that
/// owns the field is not confirmed.
pub fn fn_00444d00(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr() + 0xc8) & mask != 0
}

// Translated from 00444d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flag test (no name in the engine map) on the dword at `+0x244` of the
/// object at `011ddf38` (`this`): bit 8.
pub fn fn_00444d20(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x244) & 8 != 0
}

// Translated from 00444d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): [`fn_00444dc0`] with a
/// null result pointer of its own.
pub fn fn_00444d40(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    form: Ptr,
    priority: u32,
    parent: Ptr,
    reference: Ptr,
) {
    with_task_pointer(e, 0, |e, holder| {
        fn_00444dc0(e, this, form, holder, priority, parent, reference);
    });
}

// Translated from 00444dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): queues the model that the
/// loader picks for `form` and `reference` (`00446a60`) into the `NiPointer`
/// at `out`, through [`fn_00443dc0`] on the loader of the global `011c3b3c`.
/// The LOD multiplier is `TES::GetLODMult(form)`, replaced by 6 when a
/// reference is given and `00564e60` or [`fn_00444ed0`] accepts it. The visual
/// distance is 0, or the float at `01016970` when a reference is given and
/// `00564f00` accepts it. The flags passed are 1, 0, 0.
pub fn fn_00444dc0(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    form: Ptr,
    out: Ptr,
    priority: u32,
    parent: Ptr,
    reference: Ptr,
) {
    let model = e
        .call(LOADER_GET_TES_MODEL, &args![this, form, reference])
        .u32();
    let mut lod_mult = e.call(GET_LOD_MULT, &args![form]).u32();
    if !reference.is_null()
        && (e.call(REFERENCE_TEST_00564E60, &args![reference]).bool() || fn_00444ed0(e, reference))
    {
        lod_mult = 6;
    }
    with_task_pointer(e, 0, |e, holder| {
        let mut distance = 0.0f32;
        if !reference.is_null() && e.call(REFERENCE_TEST_00564F00, &args![reference]).bool() {
            distance = e.global::<f32>(VISUAL_DISTANCE_LIMIT);
        }
        let loader = Ptr::new(e.global::<u32>(MODEL_LOADER));
        fn_00443dc0(
            e,
            loader,
            Ptr::new(model),
            holder,
            priority,
            parent,
            lod_mult,
            1,
            0,
            0,
            distance,
        );
        e.call(NI_POINTER_ASSIGN_POINTER, &args![out, holder]);
    });
}

// Translated from 00444ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectREFR` helper (no name in the engine map): the object that
/// `00891170` finds in the reference (`this + 0x20`) holds a pointer; its
/// virtual function `0x154` is the answer, false when the pointer is 0.
pub fn fn_00444ed0(e: &mut Engine, this: Ptr) -> bool {
    let slot = e.call(REFERENCE_PROCESS_SLOT, &args![this]).u32();
    let object = e.mem.u32(slot);
    if object == 0 {
        false
    } else {
        e.vcall(object, 0x154, &args![]).bool()
    }
}

// Translated from 00444f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): when both `first` and
/// `second` are not 0, builds the 0x30-byte task `0043db30(first, second,
/// third)` and queues it with its virtual function `0x20`.
pub fn fn_00444f10(e: &mut Engine, _this: Ptr<ModelLoader>, first: u32, second: u32, third: u32) {
    if first == 0 || second == 0 {
        return;
    }
    let object = construct_object(e, 0x30, |e, block| {
        e.call(TASK_CONSTRUCT_0043DB30, &args![block, first, second, third])
            .u32()
    });
    with_task_pointer(e, object, |e, holder| {
        call_task_slot(e, holder, 0x20);
    });
}

// Translated from 00444fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): builds the 0x38-byte
/// task `0043eaf0(first, third)`, gives it `parent`, queues it with its
/// virtual function `0x20` and stores it in the `NiPointer` at `out`.
pub fn fn_00444fe0(
    e: &mut Engine,
    _this: Ptr<ModelLoader>,
    first: u32,
    out: Ptr,
    third: u32,
    parent: Ptr,
) {
    let object = construct_object(e, 0x38, |e, block| {
        e.call(TASK_CONSTRUCT_0043EAF0, &args![block, first, third])
            .u32()
    });
    with_task_pointer(e, object, |e, holder| {
        set_task_parent(e, holder, parent);
        call_task_slot(e, holder, 0x20);
        e.call(NI_POINTER_ASSIGN_POINTER, &args![out, holder]);
    });
}

// Translated from 004450c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): queues the helmet task
/// of a reference. The loader's map of queued helmets (`+0x18`) is asked for
/// the key `0043f010` gives (the dword at `+0x2b0` of `reference`). When it
/// has none, a `QueuedHelmet` (0x128 bytes, `0043ef10(reference, third,
/// fifth)`) is added to the map (virtual function `0x10`, flag 0); if that
/// refused the pointer is cleared, otherwise the task gets `parent`
/// ([`fn_00443aa0`]) and is queued with its virtual function `0x20`. The
/// `NiPointer` at `out` receives the task pointer in every case (the
/// existing task, when the map already had one).
///
/// The exception-unwinding frame is not translated.
pub fn fn_004450c0(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    reference: Ptr,
    out: Ptr,
    third: u32,
    parent: Ptr,
    fifth: u32,
) {
    with_task_pointer(e, 0, |e, holder| {
        let helmet_map = e.get(this, ModelLoader::pQueuedHelmetMap);
        let key = e.call(REFERENCE_HELMET_KEY, &args![reference]).u32();
        if !map_find(e, helmet_map, key, holder) {
            let object = construct_object(e, 0x128, |e, block| {
                e.call(
                    QUEUED_HELMET_CONSTRUCT,
                    &args![block, reference, third, fifth],
                )
                .u32()
            });
            e.call(TASK_POINTER_ASSIGN, &args![holder, object]);
            let key = e.call(REFERENCE_HELMET_KEY, &args![reference]).u32();
            let helmet_map = e.get(this, ModelLoader::pQueuedHelmetMap);
            if !map_add(e, helmet_map, key, holder, 0) {
                e.call(TASK_POINTER_ASSIGN, &args![holder, 0u32]);
            } else {
                set_task_parent(e, holder, parent);
                call_task_slot(e, holder, 0x20);
            }
        }
        e.call(NI_POINTER_ASSIGN_POINTER, &args![out, holder]);
    });
}

// Translated from 00445200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): builds the 0x40-byte task
/// `0043e580(first, third, second, fourth)` and adds it to the loader's map of
/// queued animation idles (`+0x10`) under the key `second` (virtual function
/// `0x10`, flag 0). If the map refused, the pointer is cleared, otherwise the
/// task is queued with its virtual function `0x20`. Returns 0.
///
/// The exception-unwinding frame is not translated.
pub fn fn_00445200(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) -> u32 {
    let object = construct_object(e, 0x40, |e, block| {
        e.call(
            TASK_CONSTRUCT_0043E580,
            &args![block, first, third, second, fourth],
        )
        .u32()
    });
    with_task_pointer(e, object, |e, holder| {
        let idle_map = e.get(this, ModelLoader::pQueuedAnimIdleMap);
        if map_add(e, idle_map, second, holder, 0) {
            call_task_slot(e, holder, 0x20);
        } else {
            e.call(TASK_POINTER_ASSIGN, &args![holder, 0u32]);
        }
    });
    0
}

// Translated from 00445300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::ReleaseModel` (Xbox PDB): when the model map has a model for
/// `name`, its count is lowered: a `count` of 1 through `0043acb0`, any other
/// by `ModManualRefCount` of the negated low 16 bits of `count`
/// (`0043b410`). With `try_remove` set the loader then tries to remove the
/// model (`TryAndRemoveModel`, `00448920`).
pub fn model_loader_release_model(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    name: u32,
    try_remove: u8,
    count: i32,
) {
    e.with_stack(4, |e, loaded| {
        e.mem.set_u32(loaded.addr(), 0);
        let model_map = e.get(this, ModelLoader::pModelMap);
        if !map_find(e, model_map, name, loaded) {
            return;
        }
        let model = e.mem.u32(loaded.addr());
        if count == 1 {
            e.call(MODEL_RELEASE_ONE, &args![model]);
        } else {
            let delta = (count as i16 as i32).wrapping_neg();
            e.call(MODEL_MOD_MANUAL_REF_COUNT, &args![model, delta]);
        }
        if try_remove != 0 {
            e.call(TRY_AND_REMOVE_MODEL, &args![this, model, name]);
        }
    });
}

// Translated from 00445370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): releases the KF model
/// `name` of the KF model map (`+4`). A found model is prepared (`0043bac0`);
/// when the data handler (`011ddf38`) exists and its flag `0042ce10` is set,
/// removal is turned off and `bHasDelayedFree` (Xbox PDB, `+0x2c`) is set
/// instead. With removal on and a model whose count ([`fn_004431b0`]) is 0,
/// the entry is removed from the map (virtual function `0x14`) and the model
/// deleted ([`fn_00443240`], flag 1).
pub fn fn_00445370(e: &mut Engine, this: Ptr<ModelLoader>, name: u32, remove: u8) {
    e.with_stack(4, |e, loaded| {
        e.mem.set_u32(loaded.addr(), 0);
        let kf_map = e.get(this, ModelLoader::pKFModelMap);
        if !map_find(e, kf_map, name, loaded) {
            return;
        }
        let model = e.mem.u32(loaded.addr());
        e.call(KF_MODEL_PREPARE, &args![model]);
        let mut remove = remove;
        let handler = e.global::<u32>(DATA_HANDLER);
        if handler != 0 && e.call(DATA_HANDLER_FLAG_BIT_2, &args![handler]).bool() {
            remove = 0;
            e.set(this, ModelLoader::bHasDelayedFree, 1);
        }
        if remove != 0 && fn_004431b0(e, Ptr::new(model)) == 0 {
            let kf_map = e.get(this, ModelLoader::pKFModelMap);
            e.vcall(kf_map.addr(), 0x14, &args![name]);
            if model != 0 {
                fn_00443240(e, Ptr::new(model), 1);
            }
        }
    });
}

/// Cancels the task the map `map` (a member of the loader) holds for `key`,
/// if any (`CancelTask` on the manager of `01202d98`, with 0).
fn cancel_mapped_task(e: &mut Engine, map: Ptr, key: u32) {
    with_task_pointer(e, 0, |e, holder| {
        if map_find(e, map, key, holder) {
            let task = ni_pointer_value(e, holder);
            let manager = e.global::<u32>(TASK_QUEUE);
            e.call(TASK_MANAGER_CANCEL_TASK, &args![manager, task, 0u32]);
        }
    })
}

// Translated from 00445430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::CancelReplacementKFList` (Xbox PDB): the task that the map of
/// queued replacement KF lists (`+0x14`) holds for `animation` is cancelled
/// (`BSTaskManager::CancelTask`, `0044ac40(task, 0)`).
pub fn model_loader_cancel_replacement_kf_list(
    e: &mut Engine,
    this: Ptr<ModelLoader>,
    animation: u32,
) {
    let map = e.get(this, ModelLoader::pQueuedReplacementKFListMap);
    cancel_mapped_task(e, map, animation);
}

// Translated from 004454d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): the task that the map of
/// queued animation idles (`+0x10`) holds for `idle` is cancelled
/// (`0044ac40(task, 0)`).
pub fn fn_004454d0(e: &mut Engine, this: Ptr<ModelLoader>, idle: u32) {
    let map = e.get(this, ModelLoader::pQueuedAnimIdleMap);
    cancel_mapped_task(e, map, idle);
}

// Translated from 00445570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::CancelReference` (Xbox PDB): the task that the map of queued
/// references (`+8`) holds for `reference` is cancelled (`0044ac40(task, 0)`)
/// and the entry is removed from the map of references to queue (`+0xc`,
/// virtual function `0x14`). The helmet task of the map `+0x18` is cancelled
/// too.
///
/// The exception-unwinding frame is not translated.
pub fn model_loader_cancel_reference(e: &mut Engine, this: Ptr<ModelLoader>, reference: Ptr) {
    with_task_pointer(e, 0, |e, holder| {
        let queued_map = e.get(this, ModelLoader::pQueuedReferencesMap);
        if map_find(e, queued_map, reference.addr(), holder) {
            let task = ni_pointer_value(e, holder);
            let manager = e.global::<u32>(TASK_QUEUE);
            e.call(TASK_MANAGER_CANCEL_TASK, &args![manager, task, 0u32]);
            let later_map = e.get(this, ModelLoader::pReferencesToQueueMap);
            e.vcall(later_map.addr(), 0x14, &args![reference]);
        }
        let helmet_map = e.get(this, ModelLoader::pQueuedHelmetMap);
        cancel_mapped_task(e, helmet_map, reference.addr());
    });
}

// Translated from 00445670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::CancelReferencesForCell` (Xbox PDB): for a cell that is not
/// 0 and whose `005e3fc0` (the dword at `+0xa4`) is not 0, every reference of
/// the map of queued references (`+8`) whose parent cell is `cell` is
/// cancelled ([`model_loader_cancel_reference`]).
///
/// The exception-unwinding frame is not translated.
pub fn model_loader_cancel_references_for_cell(e: &mut Engine, this: Ptr<ModelLoader>, cell: Ptr) {
    if cell.is_null() || e.call(CELL_QUEUED_COUNT_GETTER, &args![cell]).u32() == 0 {
        return;
    }
    e.with_stack(0x10, |e, iterator| {
        e.call(REFERENCE_ITERATOR_CONSTRUCT, &args![iterator]);
        while !e.call(ITERATOR_AT_END, &args![iterator]).bool() {
            e.with_stack(4, |e, reference| {
                e.mem.set_u32(reference.addr(), 0);
                with_task_pointer(e, 0, |e, holder| {
                    let queued_map = e.get(this, ModelLoader::pQueuedReferencesMap);
                    let found = e
                        .call(
                            REFERENCE_MAP_NEXT,
                            &args![queued_map, iterator, reference, holder, 1u32],
                        )
                        .bool();
                    if found {
                        let found_reference = e.mem.u32(reference.addr());
                        if e.call(REFERENCE_CELL, &args![found_reference]).u32() == cell.addr() {
                            model_loader_cancel_reference(e, this, Ptr::new(found_reference));
                        }
                    }
                });
            });
        }
        e.call(REFERENCE_ITERATOR_DESTRUCT, &args![iterator]);
    });
}

// Translated from 00445750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): whether the map of queued
/// references (`+8`) has a task for `reference`.
pub fn fn_00445750(e: &mut Engine, this: Ptr<ModelLoader>, reference: Ptr) -> bool {
    with_task_pointer(e, 0, |e, holder| {
        let queued_map = e.get(this, ModelLoader::pQueuedReferencesMap);
        map_find(e, queued_map, reference.addr(), holder)
    })
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0043fed0, queued_reference_queue_me(Ptr<QueuedReference>)),
        entry!(
            0x00440050,
            queued_reference_queue_models(Ptr<QueuedReference>)
        ),
        entry!(
            0x00440190,
            queued_reference_use_distant_3d(Ptr<QueuedReference>)
        ),
        entry!(
            0x00440310,
            queued_reference_attach_distant_3d(Ptr<QueuedReference>, Ptr)
        ),
        entry!(0x00440460, fn_00440460(Ptr, Ptr)),
        entry!(0x00440490, fn_00440490(Ptr, f32)),
        entry!(
            0x004404c0,
            fn_004404c0(Ptr<AttachDistant3DTask>, Ptr, Ptr) -> Ptr<AttachDistant3DTask>
        ),
        entry!(0x00440540, fn_00440540(Ptr<IOTask>, u32) -> Ptr<IOTask>),
        entry!(0x004405b0, fn_004405b0(Ptr<IOTask>, u8)),
        entry!(0x004405d0, fn_004405d0(Ptr<IOTask>, u8)),
        entry!(0x00440610, fn_00440610(Ptr<IOTask>, u32) -> Ptr<IOTask>),
        entry!(0x00440640, fn_00440640(Ptr<IOTask>)),
        entry!(0x00440660, io_task_requeue(Ptr<IOTask>, u32)),
        entry!(
            0x00440680,
            attach_distant_3d_task_scalar_deleting_destructor(
                Ptr<AttachDistant3DTask>,
                u32,
            )
                -> Ptr<AttachDistant3DTask>
        ),
        entry!(0x004406b0, fn_004406b0(Ptr<AttachDistant3DTask>)),
        entry!(0x00440710, fn_00440710(Ptr, Ptr)),
        entry!(
            0x00440780,
            queued_reference_check_finished(Ptr<QueuedReference>)
        ),
        entry!(0x004408d0, fn_004408d0(Ptr, Ptr)),
        entry!(
            0x004408f0,
            queued_reference_cancel(Ptr<QueuedReference>, u32, u32)
        ),
        entry!(0x004409e0, fn_004409e0(Ptr, Ptr)),
        entry!(0x00440a10, fn_00440a10(Ptr<TESObjectCELL>)),
        entry!(0x00440a50, fn_00440a50(Ptr<TESObjectCELL>)),
        entry!(
            0x00440a90,
            queued_reference_background_clone(Ptr<QueuedReference>) -> bool
        ),
        entry!(0x00440ba0, queued_reference_attach(Ptr<QueuedReference>)),
        entry!(0x00440d80, fn_00440d80(Ptr<TESForm>) -> bool),
        entry!(0x00440da0, fn_00440da0(Ptr<TESForm>) -> bool),
        entry!(
            0x00440dc0,
            queued_reference_get_description(Ptr<QueuedReference>, u32, u32) -> bool
        ),
        entry!(0x00440e30, fn_00440e30(Ptr<TESForm>) -> u32),
        entry!(
            0x00440e50,
            fn_00440e50(Ptr<QueuedReference>, Ptr, u32) -> Ptr<QueuedReference>
        ),
        entry!(
            0x00440e80,
            fn_00440e80(Ptr<QueuedReference>, u32) -> Ptr<QueuedReference>
        ),
        entry!(0x00440eb0, fn_00440eb0(Ptr<QueuedReference>)),
        entry!(0x00440ed0, queued_tree_queue_models(Ptr<QueuedReference>)),
        entry!(0x00440fd0, queued_tree_use_distant_3d(Ptr<QueuedReference>)),
        entry!(0x00441130, bs_shader_property_set_flag(Ptr, u32, u8)),
        entry!(
            0x004411d0,
            fn_004411d0(Ptr<QueuedReference>, Ptr, u32) -> Ptr<QueuedReference>
        ),
        entry!(0x00441200, queued_actor_queue_me(Ptr<QueuedReference>)),
        entry!(0x00441290, fn_00441290(Ptr, u8) -> u8),
        entry!(0x004412e0, queued_actor_queue_models(Ptr<QueuedReference>)),
        entry!(0x00441420, fn_00441420(Ptr, u32) -> u32),
        entry!(
            0x00441440,
            fn_00441440(Ptr<QueuedCharacter>, Ptr, u32) -> Ptr<QueuedCharacter>
        ),
        entry!(
            0x004414c0,
            fn_004414c0(Ptr<QueuedCharacter>, u32) -> Ptr<QueuedCharacter>
        ),
        entry!(0x004414f0, fn_004414f0(Ptr<QueuedCharacter>)),
        entry!(
            0x00441560,
            queued_character_queue_models(Ptr<QueuedCharacter>)
        ),
        entry!(
            0x004416c0,
            queued_character_background_clone(Ptr<QueuedCharacter>) -> bool
        ),
        entry!(0x00441780, fn_00441780(Ptr, u32)),
        entry!(0x004417c0, fn_004417c0()),
        entry!(0x00441800, fn_00441800(Ptr<QueuedReference>)),
        entry!(
            0x00441820,
            queued_character_finish_attach(Ptr<QueuedCharacter>)
        ),
        entry!(0x004418b0, fn_004418b0(Ptr<QueuedCharacter>)),
        entry!(
            0x00441920,
            fn_00441920(Ptr<QueuedReference>, Ptr, u32) -> Ptr<QueuedReference>
        ),
        entry!(
            0x00441950,
            fn_00441950(Ptr<QueuedReference>, u32) -> Ptr<QueuedReference>
        ),
        entry!(0x00441980, fn_00441980(Ptr<QueuedReference>)),
        entry!(
            0x004419a0,
            queued_creature_queue_models(Ptr<QueuedReference>)
        ),
        entry!(0x00441b00, fn_00441b00(Ptr) -> bool),
        entry!(
            0x00441b20,
            fn_00441b20(Ptr<QueuedCharacter>, u32) -> Ptr<QueuedCharacter>
        ),
        entry!(
            0x00441b50,
            fn_00441b50(Ptr<QueuedCharacter>, u32) -> Ptr<QueuedCharacter>
        ),
        entry!(0x00441b80, fn_00441b80(Ptr<QueuedCharacter>)),
        entry!(0x00441ba0, queued_player_queue_models(Ptr<QueuedCharacter>)),
        entry!(0x00441c30, fn_00441c30(Ptr<QueuedReference>)),
        entry!(
            0x00441c50,
            fn_00441c50(Ptr<QueuedFileLoad>, u32, u32, u32) -> Ptr<QueuedFileLoad>
        ),
        entry!(
            0x00441cf0,
            queued_file_load_scalar_deleting_destructor(
                Ptr<QueuedFileLoad>,
                u32,
            ) -> Ptr<QueuedFileLoad>
        ),
        entry!(0x00441d20, fn_00441d20(Ptr<QueuedFileLoad>)),
        entry!(
            0x00441d80,
            fn_00441d80(Ptr<QueuedFileLoad>, Ptr, u32) -> Ptr<QueuedFileLoad>
        ),
        entry!(0x00441e10, queued_kf_queue_me(Ptr<QueuedKF>)),
        entry!(0x00441e40, queued_file_load_run(Ptr<QueuedFileLoad>)),
        entry!(
            0x00441f80,
            fn_00441f80(Ptr<QueuedFaceGenFile>, u32, u32, u32) -> Ptr<QueuedFaceGenFile>
        ),
        entry!(
            0x00442010,
            queued_face_gen_file_scalar_deleting_destructor(
                Ptr<QueuedFaceGenFile>,
                u32,
            ) -> Ptr<QueuedFaceGenFile>
        ),
        entry!(0x00442040, fn_00442040(Ptr<QueuedFaceGenFile>)),
        entry!(
            0x004420b0,
            fn_004420b0(Ptr<QueuedFaceGenFile>, Ptr, u32) -> Ptr<QueuedFaceGenFile>
        ),
        entry!(
            0x00442130,
            fn_00442130(Ptr<QueuedFaceGenFile>, Ptr, Ptr) -> Ptr<QueuedFaceGenFile>
        ),
        entry!(
            0x004421d0,
            background_clone_thread_background_clone_thread(
                Ptr<BackgroundCloneThread>,
                u32,
            )
                -> Ptr<BackgroundCloneThread>
        ),
        entry!(
            0x00442290,
            background_clone_thread_scalar_deleting_destructor(
                Ptr<BackgroundCloneThread>,
                u32,
            )
                -> Ptr<BackgroundCloneThread>
        ),
        entry!(0x004422c0, fn_004422c0(Ptr<BackgroundCloneThread>)),
        entry!(
            0x00442350,
            background_clone_thread_thread_update(Ptr<BackgroundCloneThread>)
        ),
        entry!(0x00442580, fn_00442580(Ptr<BackgroundCloneThread>, Ptr)),
        entry!(0x00442630, fn_00442630(Ptr<BackgroundCloneThread>)),
        entry!(
            0x00442650,
            model_loader_model_loader(Ptr<ModelLoader>) -> Ptr<ModelLoader>
        ),
        entry!(0x00442a80, fn_00442a80(Ptr, u32)),
        entry!(0x00442aa0, model_loader_destructor(Ptr<ModelLoader>)),
        entry!(0x00443190, fn_00443190(Ptr<Model>) -> i32),
        entry!(0x004431b0, fn_004431b0(Ptr<KFModel>) -> i32),
        entry!(0x004431d0, fn_004431d0(Ptr)),
        entry!(0x004431f0, fn_004431f0(Ptr<Model>, u32) -> Ptr<Model>),
        entry!(0x00443220, fn_00443220(Ptr)),
        entry!(0x00443240, fn_00443240(Ptr<KFModel>, u32) -> Ptr<KFModel>),
        entry!(
            0x00443270,
            model_loader_output_model_map_contents(Ptr<ModelLoader>, Ptr, u8)
        ),
        entry!(
            0x00443540,
            fn_00443540(Ptr<ModelLoader>, u32, i32, Ptr) -> bool
        ),
        entry!(
            0x004436c0,
            model_loader_queue_texture(Ptr<ModelLoader>, u32, i32, Ptr) -> bool
        ),
        entry!(0x00443aa0, fn_00443aa0(Ptr<QueuedFile>, Ptr)),
        entry!(
            0x00443af0,
            model_loader_queue_texture_ov2(Ptr<ModelLoader>, u32, i32, Ptr) -> bool
        ),
        entry!(
            0x00443d30,
            fn_00443d30(Ptr<ModelLoader>, Ptr, u32, Ptr, u32, u8, u8, u8)
        ),
        entry!(
            0x00443dc0,
            fn_00443dc0(Ptr<ModelLoader>, Ptr, Ptr, u32, Ptr, u32, u8, u8, u8, f32)
        ),
        entry!(0x00443ff0, fn_00443ff0(Ptr<QueuedModel>, u8)),
        entry!(0x00444020, fn_00444020(Ptr<QueuedModel>, f32)),
        entry!(
            0x00444040,
            model_loader_queue_model(Ptr<ModelLoader>, u32, u32, Ptr, u32, u8, u8, u8)
        ),
        entry!(
            0x004440d0,
            fn_004440d0(Ptr<ModelLoader>, u32, Ptr, u32, Ptr, u32, u8, u8, u8)
        ),
        entry!(
            0x004442c0,
            fn_004442c0(Ptr<ModelLoader>, u32, u32, u32, Ptr, u32, u8, u8, u8)
        ),
        entry!(
            0x00444350,
            fn_00444350(Ptr<ModelLoader>, u32, u32, Ptr, u32, Ptr, u32, u8, u8, u8)
        ),
        entry!(
            0x00444540,
            fn_00444540(Ptr<ModelLoader>, Ptr, Ptr, u32, Ptr, u32)
        ),
        entry!(
            0x004445c0,
            fn_004445c0(Ptr<ModelLoader>, Ptr, Ptr, Ptr, u32, Ptr, u32)
        ),
        entry!(
            0x004446a0,
            fn_004446a0(Ptr<ModelLoader>, u32, u32, Ptr) -> bool
        ),
        entry!(
            0x00444850,
            model_loader_queue_reference(Ptr<ModelLoader>, Ptr, i32, u8)
        ),
        entry!(0x00444cc0, fn_00444cc0(Ptr<TESObjectCELL>)),
        entry!(0x00444ce0, fn_00444ce0(Ptr<TESObjectCELL>)),
        entry!(0x00444d00, fn_00444d00(Ptr, u32) -> bool),
        entry!(0x00444d20, fn_00444d20(Ptr) -> bool),
        entry!(
            0x00444d40,
            fn_00444d40(Ptr<ModelLoader>, Ptr, u32, Ptr, Ptr)
        ),
        entry!(
            0x00444dc0,
            fn_00444dc0(Ptr<ModelLoader>, Ptr, Ptr, u32, Ptr, Ptr)
        ),
        entry!(0x00444ed0, fn_00444ed0(Ptr) -> bool),
        entry!(0x00444f10, fn_00444f10(Ptr<ModelLoader>, u32, u32, u32)),
        entry!(
            0x00444fe0,
            fn_00444fe0(Ptr<ModelLoader>, u32, Ptr, u32, Ptr)
        ),
        entry!(
            0x004450c0,
            fn_004450c0(Ptr<ModelLoader>, Ptr, Ptr, u32, Ptr, u32)
        ),
        entry!(
            0x00445200,
            fn_00445200(Ptr<ModelLoader>, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00445300,
            model_loader_release_model(Ptr<ModelLoader>, u32, u8, i32)
        ),
        entry!(0x00445370, fn_00445370(Ptr<ModelLoader>, u32, u8)),
        entry!(
            0x00445430,
            model_loader_cancel_replacement_kf_list(Ptr<ModelLoader>, u32)
        ),
        entry!(0x004454d0, fn_004454d0(Ptr<ModelLoader>, u32)),
        entry!(
            0x00445570,
            model_loader_cancel_reference(Ptr<ModelLoader>, Ptr)
        ),
        entry!(
            0x00445670,
            model_loader_cancel_references_for_cell(Ptr<ModelLoader>, Ptr)
        ),
        entry!(0x00445750, fn_00445750(Ptr<ModelLoader>, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test doubles that answer 1 and 0, and ones that do nothing; they
    /// stand in for virtual functions (any slot) and plain callees.
    const YES: u32 = 0x0ff0_0001;
    const NO: u32 = 0x0ff0_0000;
    const NOTHING: u32 = 0x0ff0_0010;
    /// Virtual functions the tests tell apart in the call log.
    const SLOT_A: u32 = 0x0ff0_0020;
    const SLOT_B: u32 = 0x0ff0_0021;
    const SLOT_C: u32 = 0x0ff0_0022;
    const SLOT_D: u32 = 0x0ff0_0023;
    /// Virtual functions that return a value chosen by the test.
    const ANSWER_A: u32 = 0x0ff0_0100;
    const ANSWER_B: u32 = 0x0ff0_0101;
    const REFERENCE_VTABLE: u32 = 0x0ff1_0000;
    const TASK_VTABLE: u32 = 0x0ff1_1000;
    const OBJECT_VTABLE: u32 = 0x0ff1_2000;
    const OTHER_VTABLE: u32 = 0x0ff1_3000;
    /// The memory context and the priority of the tasks in the tests.
    const CONTEXT: u32 = 0x31;
    const PRIORITY: u8 = 7;
    /// A mapped word where doubles leave what they observed.
    const OBSERVED: u32 = 0x011d_e800;
    /// `QueueTable::Add` (`00449210`) and the array-element getter
    /// `0043b4a0` uses (`00877a30`).
    const QUEUE_TABLE_ADD: u32 = 0x0044_9210;
    const NODE_ARRAY_ELEMENT: u32 = 0x0087_7a30;
    /// The `QueuedFile` constructor and destructor body (`00c3c590`, `00c3c620`).
    const QUEUED_FILE_CONSTRUCT: u32 = 0x00c3_c590;
    const QUEUED_FILE_DESTRUCT: u32 = 0x00c3_c620;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The addresses among `addrs` that were called, in order.
    fn order_of(e: &Engine, addrs: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| addrs.contains(a))
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    fn decrement(e: &mut Engine) {
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.i32(a[0]) - 1;
            e.mem.set_i32(a[0], value);
            value.into_ret()
        });
    }

    /// An engine with the memory manager, memory context, `NiPointer` and
    /// task pointer functions as doubles, the pages of the globals the code
    /// reads, a loader object in the loader global, and the `YES`, `NO` and
    /// `NOTHING` doubles.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x011c_3000, 0x011d_e000, 0x0118_7000, 0x0120_2000] {
            e.map(page, 0x1000);
        }
        e.register(MEMORY_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMORY_FREE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(MEMORY_CONTEXT_ENTER, |_, _| Ret::default());
        e.register(MEMORY_CONTEXT_LEAVE, |_, _| Ret::default());
        e.register(NI_POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        for addr in [
            NI_POINTER_ASSIGN,
            NI_POINTER_CONSTRUCT,
            TASK_POINTER_CONSTRUCT,
        ] {
            e.register(addr, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                a[0].into_ret()
            });
        }
        e.register(NI_POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(TASK_POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(YES, |_, _| 1u32.into_ret());
        e.register(NO, |_, _| 0u32.into_ret());
        for addr in [NOTHING, SLOT_A, SLOT_B, SLOT_C, SLOT_D] {
            e.register(addr, |_, _| Ret::default());
        }
        let loader = e.mem.alloc(0x40);
        e.set_global(MODEL_LOADER, loader);
        e
    }

    fn loader(e: &Engine) -> u32 {
        e.global::<u32>(MODEL_LOADER)
    }

    /// Writes a virtual table at `at` whose slots all point to `NO` except
    /// the given (byte offset, function) pairs.
    fn vtable(e: &mut Engine, at: u32, slots: &[(u32, u32)]) {
        let mut table = vec![NO; 0x100];
        for (slot, function) in slots {
            table[(slot / 4) as usize] = *function;
        }
        e.put_vtable(at, &table);
    }

    /// An object of `size` bytes whose virtual table is `table`.
    fn object(e: &mut Engine, table: u32, size: u32) -> u32 {
        let address = e.mem.alloc(size);
        e.mem.set_u32(address, table);
        address
    }

    /// A reference object whose base form (`+0x20`) is `form`.
    fn reference(e: &mut Engine, slots: &[(u32, u32)], form: u32) -> Ptr {
        vtable(e, REFERENCE_VTABLE, slots);
        let address = object(e, REFERENCE_VTABLE, 0x100);
        e.mem.set_u32(address + 0x20, form);
        Ptr::new(address)
    }

    /// The getter of the base form (`+0x20`) as a double.
    fn base_form_double(e: &mut Engine) {
        e.register(GET_BASE_FORM, |e, a| e.mem.u32(a[0] + 0x20).into_ret());
    }

    /// A reference task for `reference`, with the test priority and context;
    /// virtual functions `0x28` to `0x34` are `SLOT_A` to `SLOT_D`.
    fn task(e: &mut Engine, reference: Ptr) -> Ptr<QueuedReference> {
        vtable(
            e,
            TASK_VTABLE,
            &[
                (0x28, SLOT_A),
                (0x2c, SLOT_B),
                (0x30, SLOT_C),
                (0x34, SLOT_D),
            ],
        );
        let this = e.new_object::<QueuedReference>();
        e.mem.set_u32(this.addr(), TASK_VTABLE);
        e.set(this, QueuedReference::eContext, CONTEXT);
        e.set(this, QueuedReference::pRef, reference);
        e.set(this.cast::<IOTask>(), IOTask::Key, (PRIORITY as u64) << 16);
        this
    }

    /// The doubles for a singly linked list of forms and the getters of the
    /// base form and type.
    fn list_doubles(e: &mut Engine) {
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            ((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32).into_ret()
        });
        e.register(LIST_NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(LIST_NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        base_form_double(e);
        e.register(FORM_TYPE, |e, a| (e.mem.u8(a[0] + 4) as u32).into_ret());
    }

    /// A form of type `kind`.
    fn form(e: &mut Engine, kind: u8) -> u32 {
        let address = e.mem.alloc(0x30);
        e.mem.set_u8(address + 4, kind);
        address
    }

    /// A list node holding an item that points to a holder of `form`.
    fn list_node(e: &mut Engine, form: u32, next: u32) -> u32 {
        let holder = e.mem.alloc(0x30);
        e.mem.set_u32(holder + 0x20, form);
        let item = e.mem.alloc(8);
        e.mem.set_u32(item, holder);
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, next);
        node
    }

    #[test]
    fn queue_me_queues_decal_texture_sets_of_type_4_and_runs_the_task_slots() {
        let mut e = engine();
        list_doubles(&mut e);
        e.register(REFERENCE_TEST_00564E60, |_, _| 1u32.into_ret());
        e.register(REFERENCE_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        let texture_set = form(&mut e, 4);
        let other = form(&mut e, 5);
        let second = list_node(&mut e, other, 0);
        let first = list_node(&mut e, texture_set, second);
        answer(&mut e, EXTRA_GET_DECAL_REFS, first);
        e.register(QUEUE_TEXTURE_SET, |_, _| Ret::default());
        e.register(TASK_STATE_IS_0, |_, _| 1u32.into_ret());
        e.register(TASK_SET_STATE_5, |_, _| Ret::default());
        let base = form(&mut e, 1);
        let reference = reference(&mut e, &[(0x1d0, NO)], base);
        let this = task(&mut e, reference);
        e.call_log = Some(vec![]);
        e.call(0x0043_fed0, &args![this]);
        assert_eq!(
            calls_to(&e, MEMORY_CONTEXT_ENTER)
                .iter()
                .map(|a| a[1..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![CONTEXT, 1, MODEL_LOADER_SOURCE, 0x72a]]
        );
        assert_eq!(
            calls_to(&e, QUEUE_TEXTURE_SET),
            vec![vec![texture_set, PRIORITY as u32, this.addr(), 0]]
        );
        // UseDistant3D (0x30), QueueModels (0x2c), state 5, CheckFinished (0x28).
        assert_eq!(
            order_of(
                &e,
                &[
                    SLOT_A,
                    SLOT_B,
                    SLOT_C,
                    TASK_SET_STATE_5,
                    MEMORY_CONTEXT_LEAVE
                ]
            ),
            vec![
                SLOT_C,
                SLOT_B,
                TASK_SET_STATE_5,
                SLOT_A,
                MEMORY_CONTEXT_LEAVE
            ]
        );
    }

    #[test]
    fn queue_me_without_use_distant_3d_or_decals_only_queues_models() {
        let mut e = engine();
        e.register(REFERENCE_TEST_00564E60, |_, _| 0u32.into_ret());
        e.register(REFERENCE_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(EXTRA_GET_DECAL_REFS, |_, _| 0u32.into_ret());
        e.register(TASK_STATE_IS_0, |_, _| 0u32.into_ret());
        e.register(TASK_SET_STATE_5, |_, _| Ret::default());
        let reference = reference(&mut e, &[], 0);
        let this = task(&mut e, reference);
        e.call_log = Some(vec![]);
        e.call(0x0043_fed0, &args![this]);
        assert_eq!(
            order_of(&e, &[SLOT_A, SLOT_B, SLOT_C, TASK_SET_STATE_5]),
            vec![SLOT_B, SLOT_A]
        );
        // A reference that passes the test but whose slot 0x1d0 answers
        // non-zero does not use the distant 3D either.
        e.register(REFERENCE_TEST_00564E60, |_, _| 1u32.into_ret());
        answer(&mut e, ANSWER_A, 5);
        let busy = self::reference(&mut e, &[(0x1d0, ANSWER_A)], 0);
        e.set(this, QueuedReference::pRef, busy);
        e.call_log = Some(vec![]);
        e.call(0x0043_fed0, &args![this]);
        assert!(calls_to(&e, SLOT_C).is_empty());
        // A null reference has no decals.
        e.set(this, QueuedReference::pRef, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0043_fed0, &args![this]);
        assert!(calls_to(&e, EXTRA_GET_DECAL_REFS).is_empty());
    }

    /// A `TESModel` object whose virtual function `0x14` gives the name
    /// `text`.
    fn tes_model(e: &mut Engine, text: &[u8]) -> u32 {
        let name = e.mem.alloc(0x20);
        e.mem.set_cstr(name, text);
        answer(e, ANSWER_B, name);
        vtable(e, OBJECT_VTABLE, &[(0x14, ANSWER_B)]);
        object(e, OBJECT_VTABLE, 0x40)
    }

    #[test]
    fn queue_models_queues_the_model_the_lookup_and_the_destruction_files() {
        let mut e = engine();
        let model = tes_model(&mut e, b"meshes\\a.nif");
        answer(&mut e, REFERENCE_GET_TES_MODEL, model);
        base_form_double(&mut e);
        e.register(MODEL_LOADER_QUEUE_REFERENCE_MODEL, |_, _| Ret::default());
        e.register(MODEL_LOADER_FIND_MODEL, |_, _| Ret::default());
        let destruction = form(&mut e, 9);
        answer(&mut e, GET_DESTRUCTION_FORM, destruction);
        e.register(DESTRUCTIBLE_QUEUE_FILES, |_, _| Ret::default());
        let base = form(&mut e, 1);
        let reference = reference(&mut e, &[], base);
        let this = task(&mut e, reference);
        e.call_log = Some(vec![]);
        e.call(0x0044_0050, &args![this]);
        let loader = loader(&e);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_REFERENCE_MODEL),
            vec![vec![
                loader,
                base,
                this.addr() + 0x2c,
                PRIORITY as u32,
                this.addr(),
                reference.addr()
            ]]
        );
        // The queued model slot stayed empty: the model is looked up by name.
        let find = calls_to(&e, MODEL_LOADER_FIND_MODEL);
        assert_eq!(find.len(), 1);
        assert_eq!(find[0][0], loader);
        assert_eq!(find[0][2], this.addr() + 0x30);
        assert_eq!(e.mem.cstr(find[0][1]), b"meshes\\a.nif".to_vec());
        assert_eq!(
            calls_to(&e, DESTRUCTIBLE_QUEUE_FILES),
            vec![vec![destruction, base, PRIORITY as u32, this.addr()]]
        );
    }

    #[test]
    fn queue_models_skips_the_lookup_when_the_loader_queued_a_model() {
        let mut e = engine();
        let model = tes_model(&mut e, b"a");
        answer(&mut e, REFERENCE_GET_TES_MODEL, model);
        base_form_double(&mut e);
        e.register(MODEL_LOADER_QUEUE_REFERENCE_MODEL, |e, a| {
            e.mem.set_u32(a[2], 0x1234);
            Ret::default()
        });
        e.register(MODEL_LOADER_FIND_MODEL, |_, _| Ret::default());
        answer(&mut e, GET_DESTRUCTION_FORM, 0);
        e.register(DESTRUCTIBLE_QUEUE_FILES, |_, _| Ret::default());
        let reference = reference(&mut e, &[], 0x5000);
        let this = task(&mut e, reference);
        e.call_log = Some(vec![]);
        e.call(0x0044_0050, &args![this]);
        assert!(calls_to(&e, MODEL_LOADER_FIND_MODEL).is_empty());
        assert!(calls_to(&e, DESTRUCTIBLE_QUEUE_FILES).is_empty());
    }

    #[test]
    fn queue_models_does_nothing_without_a_model_name() {
        let mut e = engine();
        base_form_double(&mut e);
        e.register(MODEL_LOADER_QUEUE_REFERENCE_MODEL, |_, _| Ret::default());
        let reference = reference(&mut e, &[], 0x5000);
        let this = task(&mut e, reference);
        // No TESModel.
        answer(&mut e, REFERENCE_GET_TES_MODEL, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_0050, &args![this]);
        assert!(calls_to(&e, MODEL_LOADER_QUEUE_REFERENCE_MODEL).is_empty());
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
        // An empty name.
        let model = tes_model(&mut e, b"");
        answer(&mut e, REFERENCE_GET_TES_MODEL, model);
        e.call_log = Some(vec![]);
        e.call(0x0044_0050, &args![this]);
        assert!(calls_to(&e, MODEL_LOADER_QUEUE_REFERENCE_MODEL).is_empty());
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
    }

    /// Doubles for `UseDistant3D`: the distant model name, a loader that
    /// finds a model (or not), and a clone of its root whose virtual
    /// function `0xc` gives a fade node (or 0) whose virtual function `0x10`
    /// answers `fade_answer`. Returns the task and the fade node.
    fn use_distant_3d_setup(
        e: &mut Engine,
        found: bool,
        has_fade_node: bool,
        fade_answer: u32,
    ) -> (Ptr<QueuedReference>, u32) {
        base_form_double(e);
        e.register(GET_DISTANT_MODEL_NAME, |e, a| {
            e.mem.set_cstr(a[0], b"distant.nif");
            Ret::default()
        });
        let model = e.mem.alloc(0x20);
        let root = e.mem.alloc(0x20);
        e.mem.set_u32(model + 0xc, root);
        e.register_double(MODEL_LOADER_FIND_MODEL, move |e, a| {
            if found {
                e.mem.set_u32(a[2], model);
            }
            Ret::default()
        });
        answer(e, ANSWER_A, fade_answer);
        vtable(e, OTHER_VTABLE, &[(0x10, ANSWER_A)]);
        let fade = if has_fade_node {
            object(e, OTHER_VTABLE, 0x20)
        } else {
            0
        };
        answer(e, ANSWER_B, fade);
        vtable(e, OBJECT_VTABLE, &[(0xc, ANSWER_B)]);
        let clone = object(e, OBJECT_VTABLE, 0x20);
        answer(e, NI_OBJECT_CLONE, clone);
        e.register(FADE_NODE_SET_LOD_MULT_TYPE, |_, _| Ret::default());
        e.register(MODEL_POINTER_RELEASE, |_, _| Ret::default());
        let reference = reference(e, &[], 0x5000);
        (task(e, reference), fade)
    }

    #[test]
    fn use_distant_3d_sets_the_lod_type_of_the_fade_node_and_attaches_it() {
        let mut e = engine();
        let (this, fade) = use_distant_3d_setup(&mut e, true, true, 1);
        e.call_log = Some(vec![]);
        e.call(0x0044_0190, &args![this]);
        assert_eq!(
            calls_to(&e, FADE_NODE_SET_LOD_MULT_TYPE),
            vec![vec![fade, 6]]
        );
        // SLOT_D is the task's virtual function 0x34.
        assert_eq!(calls_to(&e, SLOT_D), vec![vec![this.addr(), fade]]);
        assert_eq!(calls_to(&e, MODEL_POINTER_RELEASE).len(), 1);
        let find = calls_to(&e, MODEL_LOADER_FIND_MODEL);
        assert_eq!(e.mem.cstr(find[0][1]), b"distant.nif".to_vec());
    }

    #[test]
    fn use_distant_3d_attaches_a_fade_node_without_the_lod_change_when_slot_10_is_zero() {
        let mut e = engine();
        let (this, fade) = use_distant_3d_setup(&mut e, true, true, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_0190, &args![this]);
        assert!(calls_to(&e, FADE_NODE_SET_LOD_MULT_TYPE).is_empty());
        assert_eq!(calls_to(&e, SLOT_D), vec![vec![this.addr(), fade]]);
    }

    #[test]
    fn use_distant_3d_lets_a_clone_without_fade_node_go() {
        let mut e = engine();
        let (this, _) = use_distant_3d_setup(&mut e, true, false, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_0190, &args![this]);
        assert_eq!(calls_to(&e, NI_OBJECT_CLONE).len(), 1);
        // The temporary NiPointer takes the clone and releases it.
        let holding: Vec<_> = calls_to(&e, NI_POINTER_CONSTRUCT)
            .into_iter()
            .filter(|a| a[1] != 0)
            .collect();
        assert_eq!(holding.len(), 1);
        assert_eq!(calls_to(&e, NI_POINTER_DESTRUCT).len(), 1);
        assert!(calls_to(&e, SLOT_D).is_empty());
        assert!(calls_to(&e, FADE_NODE_SET_LOD_MULT_TYPE).is_empty());
    }

    #[test]
    fn use_distant_3d_without_a_model_only_releases_the_pointer() {
        let mut e = engine();
        let (this, _) = use_distant_3d_setup(&mut e, false, true, 1);
        e.call_log = Some(vec![]);
        e.call(0x0044_0190, &args![this]);
        assert!(calls_to(&e, NI_OBJECT_CLONE).is_empty());
        assert!(calls_to(&e, SLOT_D).is_empty());
        assert_eq!(calls_to(&e, MODEL_POINTER_RELEASE).len(), 1);
    }

    /// Doubles for `AttachDistant3D`; returns the task, the reference and the
    /// node.
    fn attach_distant_3d_setup(
        e: &mut Engine,
        owner_thread: u32,
    ) -> (Ptr<QueuedReference>, Ptr, Ptr) {
        let position = e.mem.alloc(12);
        for (i, v) in [1.5f32, 2.5, 3.5].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *v);
        }
        answer(e, ANSWER_A, position);
        let reference = reference(e, &[(0x1f4, ANSWER_A)], 0x5000);
        e.register(REFERENCE_GET_ORIENTATION, |e, a| {
            for i in 0..9 {
                e.mem.set_u32(a[1] + 4 * i, 100 + i);
            }
            a[1].into_ret()
        });
        e.register(REFERENCE_GET_SCALE, |_, _| 1.25f32.into_ret());
        e.register(FLOAT_FILTER, |_, a| (f32::from_bits(a[0]) * 2.0).into_ret());
        let owner = e.mem.alloc(0x20);
        e.mem.set_u32(owner + 0x10, owner_thread);
        e.set_global(THREAD_OWNER_OBJECT, owner);
        e.register(OWNER_THREAD_ID, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        answer(e, CURRENT_THREAD_ID, 5);
        answer(e, REFERENCE_CELL, 0x7000);
        let tes = e.mem.alloc(0x10);
        e.set_global(TES_GLOBAL, tes);
        e.register(REFERENCE_SET_3D, |_, _| Ret::default());
        e.register(TES_LOAD_REFERENCE, |_, _| Ret::default());
        e.register(SET_STARTS_DEAD, |_, _| Ret::default());
        e.register(IO_TASK_BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(TASK_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        let queue = e.mem.alloc(0x10);
        let loader = loader(e);
        e.mem.set_u32(loader + 0x1c, queue);
        e.register(FINISHED_QUEUE_ADD, |e, a| {
            let observed = e.mem.u32(a[1]);
            e.mem.set_u32(OBSERVED, observed);
            Ret::default()
        });
        let this = task(e, reference);
        let node = Ptr::new(e.mem.alloc(0x70));
        (this, reference, node)
    }

    #[test]
    fn attach_distant_3d_copies_the_transform_and_attaches_on_the_owner_thread() {
        let mut e = engine();
        let (this, reference, node) = attach_distant_3d_setup(&mut e, 5);
        e.call_log = Some(vec![]);
        e.call(0x0044_0310, &args![this, node]);
        let n = node.addr();
        assert_eq!(e.mem.f32(n + 0x58), 1.5);
        assert_eq!(e.mem.f32(n + 0x5c), 2.5);
        assert_eq!(e.mem.f32(n + 0x60), 3.5);
        assert_eq!(e.mem.u32(n + 0x34), 100);
        assert_eq!(e.mem.u32(n + 0x34 + 32), 108);
        assert_eq!(e.mem.f32(n + 0x64), 2.5);
        assert_eq!(
            calls_to(&e, REFERENCE_SET_3D),
            vec![vec![reference.addr(), n]]
        );
        let tes = e.global::<u32>(TES_GLOBAL);
        assert_eq!(
            calls_to(&e, TES_LOAD_REFERENCE),
            vec![vec![tes, reference.addr(), 0x7000, 0, 0]]
        );
        assert_eq!(
            calls_to(&e, SET_STARTS_DEAD),
            vec![vec![reference.addr(), 1]]
        );
        assert!(calls_to(&e, MEMORY_ALLOC).is_empty());
    }

    #[test]
    fn attach_distant_3d_queues_a_task_on_another_thread() {
        let mut e = engine();
        let (this, reference, node) = attach_distant_3d_setup(&mut e, 6);
        e.call_log = Some(vec![]);
        e.call(0x0044_0310, &args![this, node]);
        assert!(calls_to(&e, REFERENCE_SET_3D).is_empty());
        assert_eq!(calls_to(&e, MEMORY_ALLOC), vec![vec![0x20]]);
        let task = e.get(this, QueuedReference::spAttachDistant3DTask);
        assert!(!task.is_null());
        let task = task.cast::<AttachDistant3DTask>();
        assert_eq!(e.mem.u32(task.addr()), ATTACH_DISTANT_3D_TASK_VTABLE);
        assert_eq!(e.get(task, AttachDistant3DTask::pRef), reference);
        assert_eq!(e.get(task, AttachDistant3DTask::spNode), node);
        // The loader's queue was handed a task pointer holding the task.
        assert_eq!(calls_to(&e, FINISHED_QUEUE_ADD).len(), 1);
        assert_eq!(e.mem.u32(OBSERVED), task.addr());
    }

    #[test]
    fn position_is_copied_to_offset_0x58() {
        let mut e = engine();
        let source = e.mem.alloc(12);
        for i in 0..3 {
            e.mem.set_u32(source + 4 * i, 0x10 + i);
        }
        let node = e.mem.alloc(0x70);
        e.call(
            0x0044_0460,
            &args![Ptr::<()>::new(node), Ptr::<()>::new(source)],
        );
        assert_eq!(e.mem.u32(node + 0x58), 0x10);
        assert_eq!(e.mem.u32(node + 0x5c), 0x11);
        assert_eq!(e.mem.u32(node + 0x60), 0x12);
        assert_eq!(e.mem.u32(node + 0x54), 0);
    }

    #[test]
    fn scale_goes_through_the_float_helper() {
        let mut e = engine();
        e.register(FLOAT_FILTER, |_, a| (f32::from_bits(a[0]) + 1.0).into_ret());
        let node = e.mem.alloc(0x70);
        e.call(0x0044_0490, &args![Ptr::<()>::new(node), 1.5f32]);
        assert_eq!(e.mem.f32(node + 0x64), 2.5);
    }

    #[test]
    fn io_task_constructor_sets_the_table_and_the_priority() {
        let mut e = engine();
        e.register(IO_TASK_BASE_CONSTRUCT, |_, _| Ret::default());
        let task = e.new_object::<IOTask>();
        e.set(task, IOTask::Key, 0x1111_2222_0033_4455);
        e.call_log = Some(vec![]);
        let back = e.call(0x0044_0540, &args![task, 0x1c5u32]).ptr::<IOTask>();
        assert_eq!(back, task);
        assert_eq!(e.mem.u32(task.addr()), IO_TASK_VTABLE);
        assert_eq!(
            calls_to(&e, IO_TASK_BASE_CONSTRUCT),
            vec![vec![task.addr()]]
        );
        // The priority is the low byte of the argument.
        assert_eq!(e.get(task, IOTask::Key), 0x1111_2222_00c5_4455);
        assert_eq!(fn_0043cc60(&mut e, task), 0xc5);
    }

    #[test]
    fn priority_replaces_bits_16_to_23_of_the_key() {
        let mut e = engine();
        let task = e.new_object::<IOTask>();
        e.set(task, IOTask::Key, 0xffff_ffff_ffff_ffff);
        e.call(0x0044_05d0, &args![task, 0x12u8]);
        assert_eq!(e.get(task, IOTask::Key), 0xffff_ffff_ff12_ffff);
        e.call(0x0044_05b0, &args![task, 0u8]);
        assert_eq!(e.get(task, IOTask::Key), 0xffff_ffff_ff00_ffff);
        e.call(0x0044_05b0, &args![task, 0xffu8]);
        assert_eq!(e.get(task, IOTask::Key), 0xffff_ffff_ffff_ffff);
    }

    #[test]
    fn attach_distant_3d_task_constructor_builds_the_task() {
        let mut e = engine();
        e.register(IO_TASK_BASE_CONSTRUCT, |_, _| Ret::default());
        let task = e.new_object::<AttachDistant3DTask>();
        let reference = Ptr::<()>::new(0x7100);
        let node = Ptr::<()>::new(0x7200);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x0044_04c0, &args![task, reference, node])
            .ptr::<AttachDistant3DTask>();
        assert_eq!(back, task);
        assert_eq!(
            calls_to(&e, IO_TASK_BASE_CONSTRUCT),
            vec![vec![task.addr()]]
        );
        assert_eq!(e.mem.u32(task.addr()), ATTACH_DISTANT_3D_TASK_VTABLE);
        assert_eq!(e.get(task, AttachDistant3DTask::pRef), reference);
        assert_eq!(e.get(task, AttachDistant3DTask::spNode), node);
        // The priority of the new task is 0.
        assert_eq!(e.get(task.cast::<IOTask>(), IOTask::Key), 0);
    }

    #[test]
    fn io_task_scalar_deleting_destructor_frees_only_with_bit_0() {
        let mut e = engine();
        e.register(IO_TASK_BASE_DESTRUCT, |_, _| Ret::default());
        let task = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_0610, &args![task, 0u32]).u32(), task);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        assert_eq!(calls_to(&e, IO_TASK_BASE_DESTRUCT), vec![vec![task]]);
        assert_eq!(e.call(0x0044_0610, &args![task, 3u32]).u32(), task);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![task]]);
    }

    #[test]
    fn io_task_destructor_calls_the_base_destructor() {
        let mut e = engine();
        e.register(IO_TASK_BASE_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0044_0640, &args![Ptr::<()>::new(0x6000)]);
        assert_eq!(calls_to(&e, IO_TASK_BASE_DESTRUCT), vec![vec![0x6000]]);
    }

    #[test]
    fn requeue_goes_to_the_io_manager() {
        let mut e = engine();
        let manager = e.mem.alloc(0x10);
        e.set_global(TASK_QUEUE, manager);
        e.register(IO_MANAGER_REQUEUE_TASK, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0044_0660, &args![Ptr::<()>::new(0x6000), 9u32]);
        assert_eq!(
            calls_to(&e, IO_MANAGER_REQUEUE_TASK),
            vec![vec![manager, 0x6000, 9]]
        );
    }

    #[test]
    fn attach_distant_3d_task_destructor_releases_the_node_and_the_base() {
        let mut e = engine();
        e.register(IO_TASK_BASE_DESTRUCT, |_, _| Ret::default());
        let task = e.new_object::<AttachDistant3DTask>();
        e.call_log = Some(vec![]);
        e.call(0x0044_06b0, &args![task]);
        assert_eq!(
            order_of(&e, &[NI_POINTER_DESTRUCT, IO_TASK_BASE_DESTRUCT]),
            vec![NI_POINTER_DESTRUCT, IO_TASK_BASE_DESTRUCT]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_DESTRUCT),
            vec![vec![task.addr() + 0x1c]]
        );
        // The scalar deleting form frees the object with bit 0.
        e.call(0x0044_0680, &args![task, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![task.addr()]]);
        let back = e.call(0x0044_0680, &args![task, 0u32]).u32();
        assert_eq!(back, task.addr());
        assert_eq!(calls_to(&e, MEMORY_FREE).len(), 1);
    }

    #[test]
    fn finished_queue_entry_wraps_the_task_in_a_task_pointer() {
        let mut e = engine();
        let queue = e.mem.alloc(0x10);
        let loader = loader(&e);
        e.mem.set_u32(loader + 0x1c, queue);
        e.register(FINISHED_QUEUE_ADD, |e, a| {
            let held = e.mem.u32(a[1]);
            e.mem.set_u32(OBSERVED, held);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(
            0x0044_0710,
            &args![Ptr::<()>::new(loader), Ptr::<()>::new(0x6100)],
        );
        assert_eq!(e.mem.u32(OBSERVED), 0x6100);
        let adds = calls_to(&e, FINISHED_QUEUE_ADD);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], queue);
        assert_eq!(
            order_of(
                &e,
                &[
                    TASK_POINTER_CONSTRUCT,
                    FINISHED_QUEUE_ADD,
                    TASK_POINTER_DESTRUCT
                ]
            ),
            vec![
                TASK_POINTER_CONSTRUCT,
                FINISHED_QUEUE_ADD,
                TASK_POINTER_DESTRUCT
            ]
        );
    }

    /// A ready task with a queued model and no model, and a reference that
    /// may be attached.
    struct Finishing {
        this: Ptr<QueuedReference>,
        reference: Ptr,
        queue: u32,
    }

    fn finishing(e: &mut Engine) -> Finishing {
        answer(e, TASK_STATE_AT_LEAST_4, 1);
        answer(e, TASK_STATE_IS_6, 0);
        answer(e, QUEUED_MODEL_GET_MODEL, 0x6400);
        e.register(MODEL_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        answer(e, REFERENCE_CELL, 0x7000);
        let tes = e.mem.alloc(0x10);
        e.set_global(TES_GLOBAL, tes);
        answer(e, TES_IS_CELL_LOADED, 1);
        answer(e, TES_REFERENCE_PREDICATE, 1);
        let other = e.mem.alloc(0x10);
        e.set_global(SPECIAL_REFERENCE, other);
        let queue = e.mem.alloc(0x10);
        let loader = loader(e);
        e.mem.set_u32(loader + 0x28, queue);
        e.register(BACKGROUND_CLONE_QUEUE_ADD, |_, _| Ret::default());
        // The post-process queue of the IO manager.
        let manager = e.mem.alloc(0x80);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(manager + 0x64, table);
        e.set_global(TASK_QUEUE, manager);
        e.register(QUEUE_TABLE_ADD, |_, _| Ret::default());
        let reference = reference(e, &[(0x224, NO), (0x220, NO)], 0x5000);
        let this = task(e, reference);
        e.set(this, QueuedReference::spQueuedModel, Ptr::new(0x6300));
        Finishing {
            this,
            reference,
            queue,
        }
    }

    #[test]
    fn check_finished_attaches_a_ready_task_through_the_loader() {
        let mut e = engine();
        let f = finishing(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0044_0780, &args![f.this]);
        // The queued model's model was taken.
        assert_eq!(e.get(f.this, QueuedReference::spModel), Ptr::new(0x6400));
        assert_eq!(
            calls_to(&e, BACKGROUND_CLONE_QUEUE_ADD),
            vec![vec![f.queue, f.this.addr()]]
        );
        assert!(calls_to(&e, QUEUE_TABLE_ADD).is_empty());
        let tes = e.global::<u32>(TES_GLOBAL);
        assert_eq!(calls_to(&e, TES_IS_CELL_LOADED), vec![vec![tes, 0x7000, 0]]);
        assert_eq!(
            calls_to(&e, TES_REFERENCE_PREDICATE),
            vec![vec![tes, f.reference.addr()]]
        );
    }

    #[test]
    fn check_finished_post_processes_when_a_reference_slot_refuses() {
        for slot in [0x224u32, 0x220] {
            let mut e = engine();
            let f = finishing(&mut e);
            vtable(&mut e, REFERENCE_VTABLE, &[(slot, YES)]);
            e.call_log = Some(vec![]);
            e.call(0x0044_0780, &args![f.this]);
            assert!(calls_to(&e, BACKGROUND_CLONE_QUEUE_ADD).is_empty());
            // The post-process queue is the one of the task's priority.
            let adds = calls_to(&e, QUEUE_TABLE_ADD);
            assert_eq!(adds.len(), 1);
            assert_eq!(adds[0][1], PRIORITY as u32);
        }
    }

    #[test]
    fn check_finished_post_processes_when_a_condition_fails() {
        // The cell is not loaded.
        let mut e = engine();
        let f = finishing(&mut e);
        answer(&mut e, TES_IS_CELL_LOADED, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_0780, &args![f.this]);
        assert_eq!(calls_to(&e, QUEUE_TABLE_ADD).len(), 1);
        // The reference is the special one.
        let mut e = engine();
        let f = finishing(&mut e);
        e.set_global(SPECIAL_REFERENCE, f.reference.addr());
        e.call_log = Some(vec![]);
        e.call(0x0044_0780, &args![f.this]);
        assert!(calls_to(&e, TES_IS_CELL_LOADED).is_empty());
        assert_eq!(calls_to(&e, QUEUE_TABLE_ADD).len(), 1);
        // 00451e40 refuses.
        let mut e = engine();
        let f = finishing(&mut e);
        answer(&mut e, TES_REFERENCE_PREDICATE, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_0780, &args![f.this]);
        assert_eq!(calls_to(&e, QUEUE_TABLE_ADD).len(), 1);
        // The thread has a 3D object for the reference (0043fcd0).
        let mut e = engine();
        let f = finishing(&mut e);
        let tls = e.tls();
        e.mem.set_u32(tls + 0x264, f.reference.addr());
        e.mem.set_u32(tls + 0x260, 0x6500);
        e.call_log = Some(vec![]);
        e.call(0x0044_0780, &args![f.this]);
        assert_eq!(calls_to(&e, QUEUE_TABLE_ADD).len(), 1);
        // No queued model: no model is taken, and nothing is attached.
        let mut e = engine();
        let f = finishing(&mut e);
        e.set(f.this, QueuedReference::spQueuedModel, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0044_0780, &args![f.this]);
        assert!(calls_to(&e, MODEL_POINTER_ASSIGN).is_empty());
        assert_eq!(calls_to(&e, QUEUE_TABLE_ADD).len(), 1);
    }

    #[test]
    fn check_finished_does_nothing_for_unready_tasks() {
        // State below 4.
        let mut e = engine();
        let f = finishing(&mut e);
        answer(&mut e, TASK_STATE_AT_LEAST_4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_0780, &args![f.this]);
        assert!(calls_to(&e, QUEUE_TABLE_ADD).is_empty());
        assert!(calls_to(&e, BACKGROUND_CLONE_QUEUE_ADD).is_empty());
        // Unfinished children (2 expected, 1 finished).
        let mut e = engine();
        let f = finishing(&mut e);
        let children = e.mem.alloc(0x20);
        e.mem.set_u32(f.this.addr() + 0x20, children);
        answer(&mut e, 0x0044_ddc0, 2);
        answer(&mut e, 0x0044_edb0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0044_0780, &args![f.this]);
        assert!(calls_to(&e, QUEUE_TABLE_ADD).is_empty());
        assert!(calls_to(&e, BACKGROUND_CLONE_QUEUE_ADD).is_empty());
        // State 6.
        let mut e = engine();
        let f = finishing(&mut e);
        answer(&mut e, TASK_STATE_IS_6, 1);
        e.call_log = Some(vec![]);
        e.call(0x0044_0780, &args![f.this]);
        assert!(calls_to(&e, QUEUE_TABLE_ADD).is_empty());
        assert!(calls_to(&e, BACKGROUND_CLONE_QUEUE_ADD).is_empty());
    }

    #[test]
    fn loader_queue_helper_calls_the_object_at_offset_0x28() {
        let mut e = engine();
        let queue = e.mem.alloc(0x10);
        let loader = loader(&e);
        e.mem.set_u32(loader + 0x28, queue);
        e.register(BACKGROUND_CLONE_QUEUE_ADD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(
            0x0044_08d0,
            &args![Ptr::<()>::new(loader), Ptr::<()>::new(0x6600)],
        );
        assert_eq!(
            calls_to(&e, BACKGROUND_CLONE_QUEUE_ADD),
            vec![vec![queue, 0x6600]]
        );
    }

    /// A loader whose object at +8 has the virtual function `0x14` = `SLOT_A`.
    fn observer(e: &mut Engine) -> u32 {
        vtable(e, OTHER_VTABLE, &[(0x14, SLOT_A)]);
        let observer = object(e, OTHER_VTABLE, 0x20);
        let loader = loader(e);
        e.mem.set_u32(loader + 8, observer);
        observer
    }

    #[test]
    fn cancel_takes_the_cloned_3d_off_the_reference_and_lowers_the_cell_counts() {
        let mut e = engine();
        e.register(TASK_SET_STATE_6, |_, _| Ret::default());
        e.register(QUEUED_FILE_CANCEL, |_, _| Ret::default());
        e.register(REFERENCE_SET_3D_VERY_SIMPLE, |_, _| Ret::default());
        decrement(&mut e);
        e.register(REFERENCE_IS_3D_CRITICAL, |_, _| 1u32.into_ret());
        let observer = observer(&mut e);
        let cell = e.new_object::<TESObjectCELL>();
        e.set(cell, TESObjectCELL::iQueuedRefCount, 3);
        e.set(cell, TESObjectCELL::iCriticalQueuedRefCount, 1);
        answer(&mut e, REFERENCE_CELL, cell.addr());
        let reference = reference(&mut e, &[(0x1cc, SLOT_B)], 0);
        let this = task(&mut e, reference);
        e.set(this, QueuedReference::spCloned3D, Ptr::new(0x6700));
        e.set(
            this,
            QueuedReference::spAttachDistant3DTask,
            Ptr::new(0x6800),
        );
        e.call_log = Some(vec![]);
        e.call(0x0044_08f0, &args![this, 11u32, 12u32]);
        assert_eq!(calls_to(&e, TASK_SET_STATE_6), vec![vec![0x6800]]);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CANCEL),
            vec![vec![this.addr(), 11, 12]]
        );
        assert_eq!(
            calls_to(&e, REFERENCE_SET_3D_VERY_SIMPLE),
            vec![vec![reference.addr(), 0x6700]]
        );
        assert!(e.get(this, QueuedReference::spCloned3D).is_null());
        assert_eq!(calls_to(&e, SLOT_B), vec![vec![reference.addr(), 0, 0]]);
        assert_eq!(calls_to(&e, SLOT_A), vec![vec![observer, reference.addr()]]);
        assert_eq!(e.get(cell, TESObjectCELL::iQueuedRefCount), 2);
        assert_eq!(e.get(cell, TESObjectCELL::iCriticalQueuedRefCount), 0);
    }

    #[test]
    fn cancel_without_pending_work_only_cancels_the_file() {
        let mut e = engine();
        e.register(QUEUED_FILE_CANCEL, |_, _| Ret::default());
        observer(&mut e);
        answer(&mut e, REFERENCE_CELL, 0);
        let reference = reference(&mut e, &[], 0);
        let this = task(&mut e, reference);
        e.call_log = Some(vec![]);
        e.call(0x0044_08f0, &args![this, 1u32, 2u32]);
        assert!(calls_to(&e, TASK_SET_STATE_6).is_empty());
        assert!(calls_to(&e, REFERENCE_SET_3D_VERY_SIMPLE).is_empty());
        assert_eq!(calls_to(&e, QUEUED_FILE_CANCEL).len(), 1);
        assert_eq!(calls_to(&e, SLOT_A).len(), 1);
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn loader_observer_slot_0x14_gets_the_reference() {
        let mut e = engine();
        let observer = observer(&mut e);
        let loader = loader(&e);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_09e0,
            &args![Ptr::<()>::new(loader), Ptr::<()>::new(0x6900)],
        );
        assert_eq!(calls_to(&e, SLOT_A), vec![vec![observer, 0x6900]]);
    }

    #[test]
    fn cell_counters_go_down_and_stop_at_zero() {
        let mut e = engine();
        decrement(&mut e);
        let cell = e.new_object::<TESObjectCELL>();
        e.set(cell, TESObjectCELL::iQueuedRefCount, 3);
        e.call(0x0044_0a10, &args![cell]);
        assert_eq!(e.get(cell, TESObjectCELL::iQueuedRefCount), 2);
        e.set(cell, TESObjectCELL::iQueuedRefCount, 0);
        e.call(0x0044_0a10, &args![cell]);
        assert_eq!(e.get(cell, TESObjectCELL::iQueuedRefCount), 0);
        // The other counter is untouched by the first.
        assert_eq!(e.get(cell, TESObjectCELL::iCriticalQueuedRefCount), 0);
        e.set(cell, TESObjectCELL::iCriticalQueuedRefCount, 5);
        e.call(0x0044_0a50, &args![cell]);
        assert_eq!(e.get(cell, TESObjectCELL::iCriticalQueuedRefCount), 4);
        e.set(cell, TESObjectCELL::iCriticalQueuedRefCount, 0);
        e.call(0x0044_0a50, &args![cell]);
        assert_eq!(e.get(cell, TESObjectCELL::iCriticalQueuedRefCount), 0);
        assert_eq!(e.get(cell, TESObjectCELL::iQueuedRefCount), 0);
    }

    /// Doubles for `BackgroundClone`: a reference whose virtual function
    /// `0x1c8` returns a clone.
    fn background_clone_setup(
        e: &mut Engine,
        loaded: bool,
        accepted: bool,
    ) -> (Ptr<QueuedReference>, u32) {
        answer(e, REFERENCE_CELL, 0x7000);
        let tes = e.mem.alloc(0x10);
        e.set_global(TES_GLOBAL, tes);
        answer(e, TES_IS_CELL_LOADED, loaded as u32);
        answer(e, TES_REFERENCE_PREDICATE, accepted as u32);
        e.register(OBJECT_RELEASE, |_, _| Ret::default());
        let clone = e.mem.alloc(0x20);
        answer(e, ANSWER_A, clone);
        let reference = reference(e, &[(0x1c8, ANSWER_A)], 0);
        (task(e, reference), clone)
    }

    #[test]
    fn background_clone_stores_and_releases_the_clone_of_a_loaded_cell() {
        let mut e = engine();
        let (this, clone) = background_clone_setup(&mut e, true, true);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_0a90, &args![this]).bool());
        assert_eq!(e.get(this, QueuedReference::spCloned3D), Ptr::new(clone));
        assert_eq!(calls_to(&e, OBJECT_RELEASE), vec![vec![clone]]);
        let reference = e.get(this, QueuedReference::pRef);
        assert_eq!(calls_to(&e, ANSWER_A), vec![vec![reference.addr(), 1]]);
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
    }

    #[test]
    fn background_clone_answers_true_for_a_loaded_cell_without_cloning() {
        // The predicate refuses.
        let mut e = engine();
        let (this, _) = background_clone_setup(&mut e, true, false);
        assert!(e.call(0x0044_0a90, &args![this]).bool());
        assert!(e.get(this, QueuedReference::spCloned3D).is_null());
        // A 3D object is already known for the reference.
        let mut e = engine();
        let (this, _) = background_clone_setup(&mut e, true, true);
        let reference = e.get(this, QueuedReference::pRef);
        let tls = e.tls();
        e.mem.set_u32(tls + 0x264, reference.addr());
        e.mem.set_u32(tls + 0x260, 0x6500);
        assert!(e.call(0x0044_0a90, &args![this]).bool());
        assert!(e.get(this, QueuedReference::spCloned3D).is_null());
    }

    #[test]
    fn background_clone_answers_false_for_an_unloaded_cell() {
        let mut e = engine();
        let (this, _) = background_clone_setup(&mut e, false, true);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0044_0a90, &args![this]).bool());
        assert!(calls_to(&e, TES_REFERENCE_PREDICATE).is_empty());
        assert!(e.get(this, QueuedReference::spCloned3D).is_null());
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
    }

    /// Doubles for `Attach`: a loaded cell with queued-reference counts, a
    /// model with a root node, and the loader's observer.
    fn attach_setup(
        e: &mut Engine,
        form_flags: u32,
    ) -> (Ptr<QueuedReference>, Ptr<TESObjectCELL>, u32) {
        answer(e, TASK_STATE_IS_6, 0);
        decrement(e);
        let cell = e.new_object::<TESObjectCELL>();
        e.set(cell, TESObjectCELL::iQueuedRefCount, 2);
        e.set(cell, TESObjectCELL::iCriticalQueuedRefCount, 2);
        answer(e, REFERENCE_CELL, cell.addr());
        let tes = e.mem.alloc(0x10);
        e.set_global(TES_GLOBAL, tes);
        answer(e, TES_IS_CELL_LOADED, 1);
        e.register(CELL_ENTER_ATTACH, |_, _| Ret::default());
        e.register(CELL_LEAVE_ATTACH, |_, _| Ret::default());
        e.register(MODEL_LOADER_LOAD_ADDON_NODES, |_, _| Ret::default());
        e.register(TES_LOAD_REFERENCE, |_, _| Ret::default());
        answer(e, REFERENCE_IS_3D_CRITICAL, 0);
        observer(e);
        let reference = reference(e, &[], 0);
        e.mem.set_u32(reference.addr() + 8, form_flags);
        let this = task(e, reference);
        let model = e.mem.alloc(0x20);
        let root = e.mem.alloc(0x20);
        e.mem.set_u32(model + 0xc, root);
        e.set(this, QueuedReference::spModel, Ptr::new(model));
        (this, cell, root)
    }

    #[test]
    fn attach_loads_the_reference_and_lowers_the_cell_count() {
        let mut e = engine();
        let (this, cell, root) = attach_setup(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_0ba0, &args![this]);
        let reference = e.get(this, QueuedReference::pRef);
        let loader = loader(&e);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_LOAD_ADDON_NODES),
            vec![vec![loader, reference.addr(), root]]
        );
        let tes = e.global::<u32>(TES_GLOBAL);
        assert_eq!(
            calls_to(&e, TES_LOAD_REFERENCE),
            vec![vec![tes, reference.addr(), cell.addr(), this.addr(), 0]]
        );
        assert_eq!(e.get(cell, TESObjectCELL::iQueuedRefCount), 1);
        // Not a critical reference.
        assert_eq!(e.get(cell, TESObjectCELL::iCriticalQueuedRefCount), 2);
        assert_eq!(
            order_of(
                &e,
                &[
                    CELL_ENTER_ATTACH,
                    TES_LOAD_REFERENCE,
                    SLOT_A,
                    CELL_LEAVE_ATTACH
                ]
            ),
            vec![
                CELL_ENTER_ATTACH,
                TES_LOAD_REFERENCE,
                SLOT_A,
                CELL_LEAVE_ATTACH
            ]
        );
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
    }

    #[test]
    fn attach_lowers_the_critical_count_of_a_critical_reference() {
        let mut e = engine();
        let (this, cell, _) = attach_setup(&mut e, 0);
        answer(&mut e, REFERENCE_IS_3D_CRITICAL, 1);
        e.call(0x0044_0ba0, &args![this]);
        assert_eq!(e.get(cell, TESObjectCELL::iQueuedRefCount), 1);
        assert_eq!(e.get(cell, TESObjectCELL::iCriticalQueuedRefCount), 1);
    }

    #[test]
    fn attach_leaves_flagged_references_alone_but_still_lowers_the_count() {
        for flags in [0x20u32, 0x800] {
            let mut e = engine();
            let (this, cell, _) = attach_setup(&mut e, flags);
            e.call_log = Some(vec![]);
            e.call(0x0044_0ba0, &args![this]);
            assert!(calls_to(&e, TES_LOAD_REFERENCE).is_empty());
            assert!(calls_to(&e, MODEL_LOADER_LOAD_ADDON_NODES).is_empty());
            assert_eq!(calls_to(&e, SLOT_A).len(), 1);
            assert_eq!(e.get(cell, TESObjectCELL::iQueuedRefCount), 1);
            assert_eq!(calls_to(&e, CELL_ENTER_ATTACH).len(), 1);
            assert_eq!(calls_to(&e, CELL_LEAVE_ATTACH).len(), 1);
        }
    }

    #[test]
    fn attach_without_a_loaded_cell_or_model_skips_the_loading() {
        let mut e = engine();
        let (this, cell, _) = attach_setup(&mut e, 0);
        answer(&mut e, TES_IS_CELL_LOADED, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_0ba0, &args![this]);
        assert!(calls_to(&e, TES_LOAD_REFERENCE).is_empty());
        assert_eq!(e.get(cell, TESObjectCELL::iQueuedRefCount), 1);
        // Loaded cell, but no model: no addon nodes, still loaded.
        let mut e = engine();
        let (this, _, _) = attach_setup(&mut e, 0);
        e.set(this, QueuedReference::spModel, Ptr::NULL);
        e.call_log = Some(vec![]);
        e.call(0x0044_0ba0, &args![this]);
        assert!(calls_to(&e, MODEL_LOADER_LOAD_ADDON_NODES).is_empty());
        assert_eq!(calls_to(&e, TES_LOAD_REFERENCE).len(), 1);
    }

    #[test]
    fn attach_does_nothing_in_state_6() {
        let mut e = engine();
        let (this, cell, _) = attach_setup(&mut e, 0);
        answer(&mut e, TASK_STATE_IS_6, 1);
        e.call_log = Some(vec![]);
        e.call(0x0044_0ba0, &args![this]);
        assert!(calls_to(&e, CELL_ENTER_ATTACH).is_empty());
        assert!(calls_to(&e, SLOT_A).is_empty());
        assert_eq!(e.get(cell, TESObjectCELL::iQueuedRefCount), 2);
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
    }

    #[test]
    fn form_flag_tests_look_at_bits_0x20_and_0x800() {
        let mut e = engine();
        let form = e.new_object::<TESForm>();
        for (flags, bit_20, bit_800) in [
            (0u32, false, false),
            (0x20, true, false),
            (0x800, false, true),
            (0xffff_f7df, false, false),
        ] {
            e.set(form, TESForm::iFormFlags, flags);
            assert_eq!(e.call(0x0044_0d80, &args![form]).bool(), bit_20);
            assert_eq!(e.call(0x0044_0da0, &args![form]).bool(), bit_800);
        }
    }

    #[test]
    fn form_type_name_comes_from_the_table() {
        let mut e = engine();
        e.set_global(FORM_TYPE_TABLE + 12 * 4, 0x0101_c600u32);
        e.set_global(FORM_TYPE_TABLE + 12 * 5, 0x0101_c700u32);
        let form = e.new_object::<TESForm>();
        e.set(form, TESForm::cFormType, 4);
        assert_eq!(e.call(0x0044_0e30, &args![form]).u32(), 0x0101_c600);
        e.set(form, TESForm::cFormType, 5);
        assert_eq!(e.call(0x0044_0e30, &args![form]).u32(), 0x0101_c700);
    }

    #[test]
    fn get_description_formats_the_reference_and_its_type() {
        let mut e = engine();
        let form = e.new_object::<TESForm>();
        e.set(form, TESForm::cFormType, 4);
        e.set_global(FORM_TYPE_TABLE + 12 * 4, 0x0101_c600u32);
        answer(&mut e, GET_BASE_FORM, form.addr());
        answer(&mut e, GET_FORM_ID, 0x000f_00aa);
        answer(&mut e, ANSWER_A, 0x6a00);
        e.register(FORMAT_STRING, |_, _| Ret::default());
        answer(&mut e, TASK_STATE_IS_6, 0);
        let reference = reference(&mut e, &[(0x130, ANSWER_A)], 0);
        let this = task(&mut e, reference);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_0dc0, &args![this, 0x6b00u32, 0x80u32]).bool());
        assert_eq!(
            calls_to(&e, FORMAT_STRING),
            vec![vec![
                0x6b00,
                0x80,
                QUEUED_REF_FORMAT,
                0x6a00,
                0x000f_00aa,
                0x0101_c600
            ]]
        );
        // A task in state 6 has no description.
        answer(&mut e, TASK_STATE_IS_6, 1);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0044_0dc0, &args![this, 0x6b00u32, 0x80u32]).bool());
        assert!(calls_to(&e, FORMAT_STRING).is_empty());
    }

    #[test]
    fn tree_and_actor_constructors_set_their_tables() {
        let mut e = engine();
        e.register(QUEUED_FILE_CONSTRUCT, |_, _| Ret::default());
        for (address, table) in [
            (0x0044_0e50u32, QUEUED_TREE_VTABLE),
            (0x0044_11d0, QUEUED_ACTOR_VTABLE),
        ] {
            let this = e.new_object::<QueuedReference>();
            e.call_log = Some(vec![]);
            let back = e
                .call(address, &args![this, 0x7100u32, 0x1eu32])
                .ptr::<QueuedReference>();
            assert_eq!(back, this);
            assert_eq!(e.mem.u32(this.addr()), table);
            // The `QueuedReference` constructor ran with the reference and
            // the context.
            assert_eq!(e.get(this, QueuedReference::pRef), Ptr::new(0x7100));
            assert_eq!(
                calls_to(&e, QUEUED_FILE_CONSTRUCT),
                vec![vec![this.addr(), 0x1e]]
            );
        }
    }

    #[test]
    fn character_constructor_adds_two_empty_task_pointers() {
        let mut e = engine();
        e.register(QUEUED_FILE_CONSTRUCT, |_, _| Ret::default());
        let this = e.new_object::<QueuedCharacter>();
        e.mem.set_u32(this.addr() + 0x40, 0xdead);
        e.mem.set_u32(this.addr() + 0x44, 0xbeef);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x0044_1440, &args![this, 0x7100u32, 0x1eu32])
            .ptr::<QueuedCharacter>();
        assert_eq!(back, this);
        assert_eq!(e.mem.u32(this.addr()), QUEUED_CHARACTER_VTABLE);
        assert_eq!(e.get(this, QueuedCharacter::pRef), Ptr::new(0x7100));
        assert!(e.get(this, QueuedCharacter::spQueuedHead).is_null());
        assert!(e.get(this, QueuedCharacter::spQueuedHelmet).is_null());
        let own: Vec<_> = calls_to(&e, TASK_POINTER_CONSTRUCT)
            .into_iter()
            .filter(|a| a[0] >= this.addr() + 0x40)
            .collect();
        assert_eq!(
            own,
            vec![vec![this.addr() + 0x40, 0], vec![this.addr() + 0x44, 0]]
        );
    }

    #[test]
    fn tree_destructor_and_scalar_deleting_destructor() {
        let mut e = engine();
        e.register(QUEUED_FILE_DESTRUCT, |_, _| Ret::default());
        e.register(MODEL_POINTER_RELEASE, |_, _| Ret::default());
        let this = e.new_object::<QueuedReference>();
        e.call_log = Some(vec![]);
        e.call(0x0044_0eb0, &args![this]);
        // The `QueuedReference` destructor ran.
        assert_eq!(calls_to(&e, QUEUED_FILE_DESTRUCT), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, MODEL_POINTER_RELEASE).len(), 1);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        e.call(0x0044_0e80, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, QUEUED_FILE_DESTRUCT).len(), 2);
        assert_eq!(e.call(0x0044_0e80, &args![this, 0u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, MEMORY_FREE).len(), 1);
    }

    /// Doubles for the queueing of trees; `first` and `second` are the
    /// answers of the two reference predicates.
    fn tree_queue_setup(e: &mut Engine, first: u32, second: u32) -> Ptr<QueuedReference> {
        base_form_double(e);
        answer(e, GET_LOD_MULT, 3);
        answer(e, REFERENCE_TEST_00564E60, first);
        answer(e, REFERENCE_TEST_00564F00, second);
        e.register(MODEL_LOADER_QUEUE_TREE_MODEL, |_, _| Ret::default());
        let reference = reference(e, &[], 0x5000);
        task(e, reference)
    }

    #[test]
    fn tree_queue_models_picks_the_lod_multiplier() {
        for (first, second, lod) in [(0u32, 0u32, 3u32), (1, 0, 6), (0, 1, 0xa), (1, 1, 0xa)] {
            let mut e = engine();
            let this = tree_queue_setup(&mut e, first, second);
            let reference = e.get(this, QueuedReference::pRef);
            e.call_log = Some(vec![]);
            e.call(0x0044_0ed0, &args![this]);
            let loader = loader(&e);
            assert_eq!(
                calls_to(&e, MODEL_LOADER_QUEUE_TREE_MODEL),
                vec![vec![
                    loader,
                    reference.addr(),
                    0x5000,
                    PRIORITY as u32,
                    this.addr(),
                    lod
                ]]
            );
            assert_eq!(
                calls_to(&e, MEMORY_CONTEXT_ENTER)[0][1..].to_vec(),
                vec![CONTEXT, 1, MODEL_LOADER_SOURCE, 0x847]
            );
            assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
        }
    }

    #[test]
    fn tree_queue_models_without_a_reference_keeps_the_base_multiplier() {
        let mut e = engine();
        let this = tree_queue_setup(&mut e, 1, 1);
        e.set(this, QueuedReference::pRef, Ptr::NULL);
        // The base form getter reads the reference unconditionally.
        answer(&mut e, GET_BASE_FORM, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_0ed0, &args![this]);
        let queued = calls_to(&e, MODEL_LOADER_QUEUE_TREE_MODEL);
        assert_eq!(queued[0][5], 3);
        assert!(calls_to(&e, REFERENCE_TEST_00564E60).is_empty());
    }

    /// Doubles for `QueuedTree::UseDistant3D`: a distant node with one child,
    /// geometry data, and a property of type `property_type` (0 for none).
    /// Returns the task, the node, the geometry data, the child and the
    /// property.
    fn tree_distant_setup(
        e: &mut Engine,
        property_type: u32,
    ) -> (Ptr<QueuedReference>, u32, u32, u32, u32) {
        base_form_double(e);
        let node = e.mem.alloc(0xb0);
        let child = e.mem.alloc(0x40);
        // The array at +0x9c: element address = array[1] + index * 4.
        let elements = e.mem.alloc(0x10);
        e.mem.set_u32(elements, child);
        e.mem.set_u32(node + 0x9c + 4, elements);
        e.register(NODE_ARRAY_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 4).into_ret()
        });
        answer(e, BUILD_DISTANT_3D, node);
        let data = e.mem.alloc(0x20);
        answer(e, OBJECT_POINTER_AT_B8, data);
        for address in [
            SET_CONSISTENCY,
            GEOMETRY_DATA_SETTER_FIRST,
            GEOMETRY_DATA_SETTER_SECOND,
            PREPARE_OBJECT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let property = e.mem.alloc(0x60);
        answer(
            e,
            GET_PROPERTY,
            if property_type == 0 { 0 } else { property },
        );
        answer(e, PROPERTY_GET_TYPE, property_type);
        let reference = reference(e, &[], 0x5000);
        (task(e, reference), node, data, child, property)
    }

    #[test]
    fn tree_use_distant_3d_prepares_the_child_and_sets_the_shader_flags() {
        let mut e = engine();
        let (this, node, data, child, property) = tree_distant_setup(&mut e, 0xb);
        e.mem.set_u32(property + 0x20, 1);
        e.mem.set_u32(property + 0x38, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x0044_0fd0, &args![this]);
        assert_eq!(calls_to(&e, BUILD_DISTANT_3D), vec![vec![0x5000, 0]]);
        assert_eq!(calls_to(&e, OBJECT_POINTER_AT_B8), vec![vec![child]]);
        assert_eq!(calls_to(&e, SET_CONSISTENCY), vec![vec![data, 0x4000]]);
        assert_eq!(
            calls_to(&e, GEOMETRY_DATA_SETTER_FIRST),
            vec![vec![data, 0x11]]
        );
        assert_eq!(
            calls_to(&e, GEOMETRY_DATA_SETTER_SECOND),
            vec![vec![data, 1]]
        );
        assert_eq!(calls_to(&e, PREPARE_OBJECT), vec![vec![child, 0, 0]]);
        // Bits 0xd and 0x16 of the first flag dword, besides the one set.
        assert_eq!(e.mem.u32(property + 0x20), 1 | (1 << 0xd) | (1 << 0x16));
        assert_eq!(e.mem.u32(property + 0x38), 0);
        assert_eq!(calls_to(&e, SLOT_D), vec![vec![this.addr(), node]]);
    }

    #[test]
    fn tree_use_distant_3d_leaves_other_properties_and_missing_nodes_alone() {
        // A property of another type.
        let mut e = engine();
        let (this, node, _, _, property) = tree_distant_setup(&mut e, 0xc);
        e.call_log = Some(vec![]);
        e.call(0x0044_0fd0, &args![this]);
        assert_eq!(e.mem.u32(property + 0x20), 0);
        assert_eq!(calls_to(&e, SLOT_D), vec![vec![this.addr(), node]]);
        // No property at all.
        let mut e = engine();
        let (this, node, _, _, _) = tree_distant_setup(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_0fd0, &args![this]);
        assert!(calls_to(&e, PROPERTY_GET_TYPE).is_empty());
        assert_eq!(calls_to(&e, SLOT_D), vec![vec![this.addr(), node]]);
        // No distant 3D.
        let mut e = engine();
        let (this, _, _, _, _) = tree_distant_setup(&mut e, 0xb);
        answer(&mut e, BUILD_DISTANT_3D, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_0fd0, &args![this]);
        assert!(calls_to(&e, SET_CONSISTENCY).is_empty());
        assert!(calls_to(&e, SLOT_D).is_empty());
    }

    #[test]
    fn shader_property_flags_are_set_and_cleared_by_bit_number() {
        let mut e = engine();
        let property = e.mem.alloc(0x60);
        let flags = |e: &Engine, word: u32| e.mem.u32(property + 0x20 + 4 * word);
        let this = Ptr::<()>::new(property);
        e.mem.set_u32(property + 0x38, 9);
        // Setting a clear bit clears the dword at +0x38.
        e.call(0x0044_1130, &args![this, 0x23u32, 1u8]);
        assert_eq!(flags(&e, 1), 1 << 3);
        assert_eq!(flags(&e, 0), 0);
        assert_eq!(e.mem.u32(property + 0x38), 0);
        // Setting an already set bit leaves it.
        e.mem.set_u32(property + 0x38, 9);
        e.call(0x0044_1130, &args![this, 0x23u32, 1u8]);
        assert_eq!(flags(&e, 1), 1 << 3);
        assert_eq!(e.mem.u32(property + 0x38), 9);
        // Clearing a clear bit does nothing.
        e.call(0x0044_1130, &args![this, 0x24u32, 0u8]);
        assert_eq!(e.mem.u32(property + 0x38), 9);
        assert_eq!(flags(&e, 1), 1 << 3);
        // Clearing a set bit clears it and the dword at +0x38.
        e.mem.set_u32(property + 0x20 + 4 * 2, 0xffff_ffff);
        e.call(0x0044_1130, &args![this, 0x5fu32, 0u8]);
        assert_eq!(flags(&e, 2), 0x7fff_ffff);
        assert_eq!(e.mem.u32(property + 0x38), 0);
    }

    /// Doubles for `QueuedActor::QueueMe`; the virtual function `0x28` of the
    /// task (the end of `QueuedReference::QueueMe`) records the thread flag.
    fn actor_queue_me_setup(
        e: &mut Engine,
        special: bool,
        accepted: u32,
        actor: u32,
    ) -> Ptr<QueuedReference> {
        e.register(REFERENCE_TEST_00564E60, |_, _| 0u32.into_ret());
        e.register(REFERENCE_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        e.register(EXTRA_GET_DECAL_REFS, |_, _| 0u32.into_ret());
        e.register(TASK_STATE_IS_0, |_, _| 0u32.into_ret());
        answer(e, REFERENCE_TEST_00936EF0, accepted);
        answer(e, ACTOR_TEST_008B7B70, actor);
        e.register(ACTOR_ATTACH_TASK, |_, _| Ret::default());
        let reference = reference(e, &[(0x260, NOTHING)], 0);
        if special {
            e.set_global(SPECIAL_REFERENCE, reference.addr());
        }
        let this = task(e, reference);
        e.register(SLOT_A, |e, _| {
            let tls = e.tls();
            let flag = e.mem.u8(tls + TLS_QUEUED_FLAG) as u32;
            e.mem.set_u32(OBSERVED, flag);
            Ret::default()
        });
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_QUEUED_FLAG, 9);
        this
    }

    #[test]
    fn actor_queue_me_raises_the_thread_flag_around_the_reference_task() {
        let mut e = engine();
        let this = actor_queue_me_setup(&mut e, false, 0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0044_1200, &args![this]);
        // The flag was 1 while the task ran, and is put back.
        assert_eq!(e.mem.u32(OBSERVED), 1);
        let tls = e.tls();
        assert_eq!(e.mem.u8(tls + TLS_QUEUED_FLAG), 9);
        let reference = e.get(this, QueuedReference::pRef);
        assert_eq!(calls_to(&e, NOTHING), vec![vec![reference.addr()]]);
        assert_eq!(
            calls_to(&e, ACTOR_ATTACH_TASK),
            vec![vec![reference.addr(), 1, this.addr()]]
        );
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
    }

    #[test]
    fn actor_queue_me_keeps_the_flag_for_the_special_or_accepted_reference() {
        // The special reference.
        let mut e = engine();
        let this = actor_queue_me_setup(&mut e, true, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_1200, &args![this]);
        assert_eq!(e.mem.u32(OBSERVED), 9);
        assert!(calls_to(&e, ACTOR_ATTACH_TASK).is_empty());
        // A reference that 00936ef0 accepts.
        let mut e = engine();
        let this = actor_queue_me_setup(&mut e, false, 0x1234, 0);
        e.call(0x0044_1200, &args![this]);
        assert_eq!(e.mem.u32(OBSERVED), 9);
        let tls = e.tls();
        assert_eq!(e.mem.u8(tls + TLS_QUEUED_FLAG), 9);
    }

    #[test]
    fn thread_flag_swap_returns_the_old_value() {
        let mut e = engine();
        let tls = e.tls();
        e.mem.set_u8(tls + 0x25c, 4);
        assert_eq!(e.call(0x0044_1290, &args![Ptr::<()>::new(0), 1u8]).u8(), 4);
        assert_eq!(e.mem.u8(tls + 0x25c), 1);
        assert_eq!(e.call(0x0044_1290, &args![Ptr::<()>::new(0), 0u8]).u8(), 1);
        assert_eq!(e.mem.u8(tls + 0x25c), 0);
    }

    #[test]
    fn dismemberment_entry_is_read_from_the_array_at_0x20() {
        let mut e = engine();
        e.register(ARRAY_ELEMENT_ADDRESS, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 4).into_ret()
        });
        let extra = e.mem.alloc(0x40);
        let elements = e.mem.alloc(0x10);
        e.mem.set_u32(elements + 4, 0x7a7a);
        e.mem.set_u32(extra + 0x20 + 4, elements);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0044_1420, &args![Ptr::<()>::new(extra), 1u32])
                .u32(),
            0x7a7a
        );
        assert_eq!(
            calls_to(&e, ARRAY_ELEMENT_ADDRESS),
            vec![vec![extra + 0x20, 1]]
        );
    }

    #[test]
    fn actor_queue_models_queues_every_dismemberment_part_then_the_models() {
        let mut e = engine();
        e.register(ARRAY_ELEMENT_ADDRESS, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 4).into_ret()
        });
        e.register(ARRAY_COUNT_AT_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(ACTOR_PREPARE, |_, _| Ret::default());
        e.register(REFERENCE_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        // The extra: 2 entries at +0x20; entry 0 has 2 parts, entry 1 one.
        let extra = e.mem.alloc(0x40);
        answer(&mut e, EXTRA_GET_DISMEMBERMENT, extra);
        let entries = e.mem.alloc(0x10);
        e.mem.set_u32(extra + 0x20 + 8, 2);
        e.mem.set_u32(extra + 0x20 + 4, entries);
        let entry_a = e.mem.alloc(0x20);
        let entry_b = e.mem.alloc(0x20);
        e.mem.set_u32(entries, entry_a);
        e.mem.set_u32(entries + 4, entry_b);
        let parts_a = e.mem.alloc(0x10);
        let parts_b = e.mem.alloc(0x10);
        e.mem.set_u32(parts_a, 0xa1);
        e.mem.set_u32(parts_a + 4, 0xa2);
        e.mem.set_u32(parts_b, 0xb1);
        e.mem.set_u32(entry_a + 4 + 8, 2);
        e.mem.set_u32(entry_a + 4 + 4, parts_a);
        e.mem.set_u32(entry_b + 4 + 8, 1);
        e.mem.set_u32(entry_b + 4 + 4, parts_b);
        e.register(MODEL_LOADER_QUEUE_DISMEMBER_PART, |_, _| Ret::default());
        // QueuedReference::QueueModels finds no TESModel here.
        base_form_double(&mut e);
        answer(&mut e, REFERENCE_GET_TES_MODEL, 0);
        let reference = reference(&mut e, &[], 0x5000);
        let this = task(&mut e, reference);
        e.call_log = Some(vec![]);
        e.call(0x0044_12e0, &args![this]);
        let loader = loader(&e);
        let queued = calls_to(&e, MODEL_LOADER_QUEUE_DISMEMBER_PART);
        assert_eq!(
            queued.iter().map(|a| a[1]).collect::<Vec<_>>(),
            vec![0xa1, 0xa2, 0xb1]
        );
        assert_eq!(
            queued[0],
            vec![loader, 0xa1, PRIORITY as u32, this.addr(), reference.addr()]
        );
        assert_eq!(calls_to(&e, ACTOR_PREPARE), vec![vec![reference.addr()]]);
        // The memory context is entered twice, with the lines of the two
        // functions, the inner one being `QueuedReference::QueueModels`.
        assert_eq!(
            calls_to(&e, MEMORY_CONTEXT_ENTER)
                .iter()
                .map(|a| a[4])
                .collect::<Vec<_>>(),
            vec![0x8a2, QUEUE_MODELS_SOURCE_LINE]
        );
        assert_eq!(calls_to(&e, REFERENCE_GET_TES_MODEL).len(), 1);
    }

    #[test]
    fn actor_queue_models_without_a_dismemberment_extra_queues_the_models_only() {
        let mut e = engine();
        e.register(ACTOR_PREPARE, |_, _| Ret::default());
        e.register(REFERENCE_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        answer(&mut e, EXTRA_GET_DISMEMBERMENT, 0);
        e.register(MODEL_LOADER_QUEUE_DISMEMBER_PART, |_, _| Ret::default());
        base_form_double(&mut e);
        answer(&mut e, REFERENCE_GET_TES_MODEL, 0);
        let reference = reference(&mut e, &[], 0x5000);
        let this = task(&mut e, reference);
        e.call_log = Some(vec![]);
        e.call(0x0044_12e0, &args![this]);
        assert!(calls_to(&e, MODEL_LOADER_QUEUE_DISMEMBER_PART).is_empty());
        assert_eq!(calls_to(&e, REFERENCE_GET_TES_MODEL).len(), 1);
    }

    // ---- the batch of 004414c0 to 00443190 ----

    /// Words where doubles of this batch leave what they observed.
    const SEEN_A: u32 = OBSERVED + 0x10;
    const SEEN_B: u32 = OBSERVED + 0x14;
    const SEEN_C: u32 = OBSERVED + 0x18;

    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// Doubles for `QueuedActor::QueueModels` and `QueuedReference::
    /// QueueModels` of a reference without a dismemberment extra or a model.
    fn actor_models_doubles(e: &mut Engine) {
        e.register(ACTOR_PREPARE, |_, _| Ret::default());
        e.register(REFERENCE_EXTRA_LIST, |_, a| (a[0] + 0x44).into_ret());
        answer(e, EXTRA_GET_DISMEMBERMENT, 0);
        base_form_double(e);
        answer(e, REFERENCE_GET_TES_MODEL, 0);
    }

    /// The doubles for the `QueuedReference` destructor's callees.
    fn destructor_doubles(e: &mut Engine) {
        noop(e, &[MODEL_POINTER_RELEASE, QUEUED_FILE_DESTRUCT]);
    }

    /// A `QueuedCharacter` for `reference` with the test context and priority.
    fn character(e: &mut Engine, reference: Ptr) -> Ptr<QueuedCharacter> {
        vtable(e, TASK_VTABLE, &[]);
        let this = e.new_object::<QueuedCharacter>();
        e.mem.set_u32(this.addr(), TASK_VTABLE);
        e.set(this, QueuedCharacter::eContext, CONTEXT);
        e.set(this, QueuedCharacter::pRef, reference);
        e.set(this.cast::<IOTask>(), IOTask::Key, (PRIORITY as u64) << 16);
        this
    }

    fn entered_lines(e: &Engine) -> Vec<u32> {
        calls_to(e, MEMORY_CONTEXT_ENTER)
            .iter()
            .map(|a| a[4])
            .collect()
    }

    #[test]
    fn character_destructor_releases_helmet_then_head_and_frees_on_request() {
        let mut e = engine();
        destructor_doubles(&mut e);
        let reference = Ptr::<()>::new(0x1000);
        let this = character(&mut e, reference);
        e.call_log = Some(vec![]);
        // Without the delete flag the object is kept.
        assert_eq!(e.call(0x0044_14c0, &args![this, 0u32]).u32(), this.addr());
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        let released = calls_to(&e, TASK_POINTER_DESTRUCT);
        assert_eq!(released[0], vec![this.addr() + 0x44]);
        assert_eq!(released[1], vec![this.addr() + 0x40]);
        assert_eq!(calls_to(&e, QUEUED_FILE_DESTRUCT), vec![vec![this.addr()]]);
        // With it, the block is freed after the destructor ran.
        e.call_log = Some(vec![]);
        e.call(0x0044_14c0, &args![this, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
        assert_eq!(
            order_of(&e, &[QUEUED_FILE_DESTRUCT, MEMORY_FREE]),
            vec![QUEUED_FILE_DESTRUCT, MEMORY_FREE]
        );
    }

    #[test]
    fn character_destructor_body_runs_the_reference_destructor_last() {
        let mut e = engine();
        destructor_doubles(&mut e);
        let this = character(&mut e, Ptr::<()>::new(0x1000));
        e.call_log = Some(vec![]);
        e.call(0x0044_14f0, &args![this]);
        assert_eq!(
            order_of(&e, &[TASK_POINTER_DESTRUCT, QUEUED_FILE_DESTRUCT]),
            vec![
                TASK_POINTER_DESTRUCT,
                TASK_POINTER_DESTRUCT,
                TASK_POINTER_DESTRUCT,
                TASK_POINTER_DESTRUCT,
                QUEUED_FILE_DESTRUCT
            ]
        );
    }

    #[test]
    fn character_queue_models_queues_biped_files_head_and_animations() {
        let mut e = engine();
        actor_models_doubles(&mut e);
        e.register(CREATE_BIPED_ANIM, |_, _| 0u32.into_ret());
        noop(
            &mut e,
            &[
                NPC_SET_BIPED_ANIM,
                BIPED_QUEUE_FILES,
                MODEL_LOADER_QUEUE_HEAD,
                MODEL_LOADER_QUEUE_ANIMATIONS,
            ],
        );
        answer(&mut e, ANSWER_A, 0xb1b1);
        let reference = reference(&mut e, &[(0x1e8, ANSWER_A)], 0x5000);
        let this = character(&mut e, reference);
        e.call_log = Some(vec![]);
        e.call(0x0044_1560, &args![this]);
        let loader = loader(&e);
        assert!(calls_to(&e, CREATE_BIPED_ANIM).is_empty());
        assert_eq!(
            calls_to(&e, NPC_SET_BIPED_ANIM),
            vec![vec![0x5000, reference.addr(), 0xb1b1, 0]]
        );
        assert_eq!(
            calls_to(&e, BIPED_QUEUE_FILES),
            vec![vec![
                0xb1b1,
                PRIORITY as u32,
                this.addr() + 0x44,
                this.addr()
            ]]
        );
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_HEAD),
            vec![vec![
                loader,
                0x5000,
                this.addr() + 0x40,
                PRIORITY as u32,
                this.addr()
            ]]
        );
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_ANIMATIONS),
            vec![vec![
                loader,
                0x5000 + 0xdc,
                PRIORITY as u32,
                this.addr(),
                reference.addr(),
                0,
                0
            ]]
        );
        // The actor and reference tasks ran inside this one's context.
        assert_eq!(
            entered_lines(&e),
            vec![
                0x8ca,
                ACTOR_QUEUE_MODELS_SOURCE_LINE,
                QUEUE_MODELS_SOURCE_LINE
            ]
        );
    }

    #[test]
    fn character_queue_models_creates_a_biped_when_there_is_none() {
        let mut e = engine();
        actor_models_doubles(&mut e);
        answer(&mut e, CREATE_BIPED_ANIM, 0);
        noop(
            &mut e,
            &[
                NPC_SET_BIPED_ANIM,
                BIPED_QUEUE_FILES,
                MODEL_LOADER_QUEUE_HEAD,
                MODEL_LOADER_QUEUE_ANIMATIONS,
            ],
        );
        answer(&mut e, ANSWER_A, 0);
        // A reference without a base form: no animation data either.
        let reference = reference(&mut e, &[(0x1e8, ANSWER_A)], 0);
        let this = character(&mut e, reference);
        e.call_log = Some(vec![]);
        e.call(0x0044_1560, &args![this]);
        assert_eq!(
            calls_to(&e, CREATE_BIPED_ANIM),
            vec![vec![0, reference.addr()]]
        );
        assert!(calls_to(&e, NPC_SET_BIPED_ANIM).is_empty());
        assert!(calls_to(&e, MODEL_LOADER_QUEUE_HEAD).is_empty());
        let animations = calls_to(&e, MODEL_LOADER_QUEUE_ANIMATIONS);
        assert_eq!(animations[0][1], 0);
    }

    #[test]
    fn character_queue_models_uses_the_biped_the_creation_gives() {
        let mut e = engine();
        actor_models_doubles(&mut e);
        answer(&mut e, CREATE_BIPED_ANIM, 0xc0c0);
        noop(
            &mut e,
            &[
                NPC_SET_BIPED_ANIM,
                BIPED_QUEUE_FILES,
                MODEL_LOADER_QUEUE_HEAD,
                MODEL_LOADER_QUEUE_ANIMATIONS,
            ],
        );
        answer(&mut e, ANSWER_A, 0);
        let reference = reference(&mut e, &[(0x1e8, ANSWER_A)], 0x5000);
        let this = character(&mut e, reference);
        e.call_log = Some(vec![]);
        e.call(0x0044_1560, &args![this]);
        assert_eq!(
            calls_to(&e, NPC_SET_BIPED_ANIM),
            vec![vec![0x5000, reference.addr(), 0xc0c0, 0]]
        );
        assert_eq!(calls_to(&e, BIPED_QUEUE_FILES)[0][0], 0xc0c0);
    }

    /// Doubles for the character `BackgroundClone` tests: the parent cell is
    /// loaded (or not), the reference needs no clone, the cloned 3D getter
    /// reads +0x34, and the attach calls leave the thread-local words they
    /// see.
    fn clone_setup(e: &mut Engine, cell_loaded: bool, node: u32) -> Ptr<QueuedCharacter> {
        e.register(REFERENCE_CELL, |_, _| 0x7000u32.into_ret());
        answer(e, TES_IS_CELL_LOADED, cell_loaded as u32);
        answer(e, TES_REFERENCE_PREDICATE, 0);
        e.register(GET_CLONED_3D, |e, a| e.mem.u32(a[0] + 0x34).into_ret());
        e.register(REFERENCE_QUERY_005D9F90, |_, _| 0x99u32.into_ret());
        e.register(HEAD_ATTACH, |e, _| {
            let tls = e.tls();
            let reference = e.mem.u32(tls + TLS_CLONE_REFERENCE);
            let word = e.mem.u32(tls + TLS_CLONE_WORD);
            e.mem.set_u32(SEEN_A, reference);
            e.mem.set_u32(SEEN_B, word);
            Ret::default()
        });
        e.register(HELMET_ATTACH, |_, _| Ret::default());
        let reference = reference(e, &[], 0x5000);
        let this = character(e, reference);
        e.mem.set_u32(this.addr() + 0x34, node);
        let head = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + 0x40, head);
        this
    }

    #[test]
    fn character_background_clone_attaches_head_and_helmet_with_the_clone_words() {
        let mut e = engine();
        let this = clone_setup(&mut e, true, 0x3333);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_16c0, &args![this]).bool());
        let reference = e.get(this, QueuedCharacter::pRef).addr();
        // The head saw the reference and the cloned 3D in the thread words.
        assert_eq!(e.mem.u32(SEEN_A), reference);
        assert_eq!(e.mem.u32(SEEN_B), 0x3333);
        let head = e.mem.u32(this.addr() + 0x40);
        assert_eq!(calls_to(&e, HEAD_ATTACH), vec![vec![head, reference, 0x99]]);
        // They are cleared again, and no helmet was set.
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_CLONE_REFERENCE), 0);
        assert_eq!(e.mem.u32(tls + TLS_CLONE_WORD), 0);
        assert!(calls_to(&e, HELMET_ATTACH).is_empty());
        assert_eq!(entered_lines(&e), vec![0x8e4, BACKGROUND_CLONE_SOURCE_LINE]);
    }

    #[test]
    fn character_background_clone_does_nothing_more_without_a_cloned_3d_or_cell() {
        // The cell is loaded but the task has no cloned 3D.
        let mut e = engine();
        let this = clone_setup(&mut e, true, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_16c0, &args![this]).bool());
        assert!(calls_to(&e, HEAD_ATTACH).is_empty());
        // The cell is not loaded: the result is false.
        let mut e = engine();
        let this = clone_setup(&mut e, false, 0x3333);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0044_16c0, &args![this]).bool());
        assert!(calls_to(&e, GET_CLONED_3D).is_empty());
    }

    #[test]
    fn clone_words_are_set_and_cleared_in_the_tls_block() {
        let mut e = engine();
        e.call(0x0044_1780, &args![Ptr::<()>::new(0x1234), 0x5678u32]);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + 0x264), 0x1234);
        assert_eq!(e.mem.u32(tls + 0x260), 0x5678);
        e.call(0x0044_17c0, &args![]);
        assert_eq!(e.mem.u32(tls + 0x264), 0);
        assert_eq!(e.mem.u32(tls + 0x260), 0);
    }

    #[test]
    fn character_attach_is_the_reference_attach() {
        // A task in state 6 leaves the attach after entering its context.
        let mut e = engine();
        answer(&mut e, TASK_STATE_IS_6, 1);
        let this = task(&mut e, Ptr::<()>::new(0x1000));
        e.call_log = Some(vec![]);
        e.call(0x0044_1800, &args![this]);
        assert_eq!(entered_lines(&e), vec![ATTACH_SOURCE_LINE]);
        assert!(calls_to(&e, REFERENCE_CELL).is_empty());
        // The player's attach is the same function.
        e.call_log = Some(vec![]);
        e.call(0x0044_1c30, &args![this]);
        assert_eq!(entered_lines(&e), vec![ATTACH_SOURCE_LINE]);
    }

    #[test]
    fn character_finish_attach_attaches_only_without_a_cloned_3d() {
        let mut e = engine();
        let this = clone_setup(&mut e, true, 0);
        noop(&mut e, &[FINISH_ATTACH_BASE]);
        e.call_log = Some(vec![]);
        e.call(0x0044_1820, &args![this]);
        assert_eq!(calls_to(&e, FINISH_ATTACH_BASE), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, HEAD_ATTACH).len(), 1);
        assert_eq!(entered_lines(&e), vec![0x8ff]);
        // With a cloned 3D nothing is attached.
        e.mem.set_u32(this.addr() + 0x34, 0x3333);
        e.call_log = Some(vec![]);
        e.call(0x0044_1820, &args![this]);
        assert!(calls_to(&e, HEAD_ATTACH).is_empty());
    }

    #[test]
    fn character_attach_helper_attaches_the_head_then_the_helmet() {
        let mut e = engine();
        let this = clone_setup(&mut e, true, 0);
        let helmet = e.mem.alloc(8);
        e.mem.set_u32(this.addr() + 0x44, helmet);
        e.call_log = Some(vec![]);
        e.call(0x0044_18b0, &args![this]);
        let reference = e.get(this, QueuedCharacter::pRef).addr();
        let head = e.mem.u32(this.addr() + 0x40);
        assert_eq!(
            order_of(&e, &[HEAD_ATTACH, HELMET_ATTACH]),
            vec![HEAD_ATTACH, HELMET_ATTACH]
        );
        assert_eq!(calls_to(&e, HEAD_ATTACH), vec![vec![head, reference, 0x99]]);
        assert_eq!(calls_to(&e, HELMET_ATTACH), vec![vec![helmet]]);
        // Neither task: nothing is attached.
        e.mem.set_u32(this.addr() + 0x40, 0);
        e.mem.set_u32(this.addr() + 0x44, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_18b0, &args![this]);
        assert!(calls_to(&e, HEAD_ATTACH).is_empty());
        assert!(calls_to(&e, HELMET_ATTACH).is_empty());
    }

    /// The doubles of the `QueuedReference` constructor.
    fn constructor_doubles(e: &mut Engine) {
        e.register(QUEUED_FILE_CONSTRUCT, |_, a| a[0].into_ret());
    }

    #[test]
    fn creature_constructor_sets_the_reference_and_the_creature_table() {
        let mut e = engine();
        constructor_doubles(&mut e);
        let this = e.new_object::<QueuedReference>();
        let result = e.call(0x0044_1920, &args![this, Ptr::<()>::new(0x4444), 0x31u32]);
        assert_eq!(result.u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), 0x0101_6d34);
        assert_eq!(e.get(this, QueuedReference::pRef).addr(), 0x4444);
    }

    #[test]
    fn creature_destructor_runs_the_reference_destructor() {
        let mut e = engine();
        destructor_doubles(&mut e);
        let this = e.new_object::<QueuedReference>();
        e.call_log = Some(vec![]);
        e.call(0x0044_1980, &args![this]);
        assert_eq!(calls_to(&e, QUEUED_FILE_DESTRUCT), vec![vec![this.addr()]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_1950, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
        e.call_log = Some(vec![]);
        e.call(0x0044_1950, &args![this, 0u32]);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
    }

    /// A creature task whose reference has the given package (0 for none)
    /// and base form; `default_worn` is the answer of `005f1590`.
    fn creature_setup(
        e: &mut Engine,
        package: u32,
        default_worn: u32,
        form: u32,
    ) -> Ptr<QueuedReference> {
        actor_models_doubles(e);
        answer(e, GET_CURRENT_PACKAGE, package);
        answer(e, SHOULD_RUN_INIT_DEFAULT_WORN, default_worn);
        noop(
            e,
            &[
                CREATURE_INIT_WORN,
                CREATURE_INIT_DEFAULT_WORN,
                MODEL_LOADER_QUEUE_ANIMATIONS,
                MODEL_LOADER_QUEUE_CREATURE_PARTS,
            ],
        );
        let reference = reference(e, &[], form);
        task(e, reference)
    }

    #[test]
    fn creature_queue_models_inits_the_default_worn_items_with_the_flag() {
        // No package: the flag stays 1.
        let mut e = engine();
        let this = creature_setup(&mut e, 0, 1, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0044_19a0, &args![this]);
        let reference = e.get(this, QueuedReference::pRef).addr();
        let loader = loader(&e);
        assert_eq!(
            calls_to(&e, CREATURE_INIT_DEFAULT_WORN),
            vec![vec![0x5000, reference, 1, 1, 0]]
        );
        assert!(calls_to(&e, CREATURE_INIT_WORN).is_empty());
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_ANIMATIONS),
            vec![vec![
                loader,
                0x5000 + 0xdc,
                PRIORITY as u32,
                this.addr(),
                reference,
                1,
                0
            ]]
        );
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_CREATURE_PARTS),
            vec![vec![
                loader,
                0x5000 + 0xdc,
                PRIORITY as u32,
                this.addr(),
                reference
            ]]
        );
        assert_eq!(
            order_of(
                &e,
                &[
                    CREATURE_INIT_DEFAULT_WORN,
                    MODEL_LOADER_QUEUE_ANIMATIONS,
                    MODEL_LOADER_QUEUE_CREATURE_PARTS,
                    REFERENCE_GET_TES_MODEL
                ]
            ),
            vec![
                CREATURE_INIT_DEFAULT_WORN,
                MODEL_LOADER_QUEUE_ANIMATIONS,
                MODEL_LOADER_QUEUE_CREATURE_PARTS,
                REFERENCE_GET_TES_MODEL
            ]
        );
        assert_eq!(entered_lines(&e)[0], 0x925);
    }

    #[test]
    fn creature_queue_models_clears_the_flag_for_a_package_with_the_bit() {
        let mut e = engine();
        let package = e.mem.alloc(0x40);
        e.mem.set_u32(package + 0x1c, 0x0020_0000);
        let this = creature_setup(&mut e, package, 1, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0044_19a0, &args![this]);
        let defaults = calls_to(&e, CREATURE_INIT_DEFAULT_WORN);
        assert_eq!(defaults[0][3], 0);
        // A package without the bit keeps it.
        let mut e = engine();
        let package = e.mem.alloc(0x40);
        e.mem.set_u32(package + 0x1c, 0x0010_0000);
        let this = creature_setup(&mut e, package, 1, 0x5000);
        e.call_log = Some(vec![]);
        e.call(0x0044_19a0, &args![this]);
        assert_eq!(calls_to(&e, CREATURE_INIT_DEFAULT_WORN)[0][3], 1);
    }

    #[test]
    fn creature_queue_models_inits_plain_worn_items_and_handles_no_base_form() {
        let mut e = engine();
        let this = creature_setup(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_19a0, &args![this]);
        let reference = e.get(this, QueuedReference::pRef).addr();
        assert_eq!(calls_to(&e, CREATURE_INIT_WORN), vec![vec![0, reference]]);
        assert!(calls_to(&e, CREATURE_INIT_DEFAULT_WORN).is_empty());
        // No base form: no animation data.
        assert_eq!(calls_to(&e, MODEL_LOADER_QUEUE_ANIMATIONS)[0][1], 0);
        assert_eq!(calls_to(&e, MODEL_LOADER_QUEUE_CREATURE_PARTS)[0][1], 0);
    }

    #[test]
    fn package_predicate_tests_bit_0x200000_at_0x1c() {
        let mut e = engine();
        let package = e.mem.alloc(0x40);
        assert!(!e.call(0x0044_1b00, &args![Ptr::<()>::new(package)]).bool());
        e.mem.set_u32(package + 0x1c, 0x0020_0000);
        assert!(e.call(0x0044_1b00, &args![Ptr::<()>::new(package)]).bool());
        e.mem.set_u32(package + 0x1c, 0xffdf_ffff);
        assert!(!e.call(0x0044_1b00, &args![Ptr::<()>::new(package)]).bool());
    }

    #[test]
    fn player_constructor_uses_the_special_reference() {
        let mut e = engine();
        constructor_doubles(&mut e);
        e.set_global(SPECIAL_REFERENCE, 0x6666u32);
        let this = e.new_object::<QueuedCharacter>();
        let result = e.call(0x0044_1b20, &args![this, 0x31u32]);
        assert_eq!(result.u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), 0x0101_6d7c);
        assert_eq!(e.get(this, QueuedCharacter::pRef).addr(), 0x6666);
        // The head and helmet pointers were made empty.
        assert_eq!(e.mem.u32(this.addr() + 0x40), 0);
        assert_eq!(e.mem.u32(this.addr() + 0x44), 0);
    }

    #[test]
    fn player_destructor_runs_the_character_destructor() {
        let mut e = engine();
        destructor_doubles(&mut e);
        let this = character(&mut e, Ptr::<()>::new(0x1000));
        e.call_log = Some(vec![]);
        e.call(0x0044_1b80, &args![this]);
        assert_eq!(calls_to(&e, TASK_POINTER_DESTRUCT).len(), 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_1b50, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
        e.call_log = Some(vec![]);
        e.call(0x0044_1b50, &args![this, 0u32]);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
    }

    #[test]
    fn player_queue_models_asks_the_player_then_queues_the_character() {
        let mut e = engine();
        actor_models_doubles(&mut e);
        e.set_global(SPECIAL_REFERENCE, 0x6666u32);
        noop(
            &mut e,
            &[
                PLAYER_QUEUE_FILES,
                NPC_SET_BIPED_ANIM,
                BIPED_QUEUE_FILES,
                MODEL_LOADER_QUEUE_HEAD,
                MODEL_LOADER_QUEUE_ANIMATIONS,
            ],
        );
        answer(&mut e, ANSWER_A, 0xb1b1);
        let reference = reference(&mut e, &[(0x1e8, ANSWER_A)], 0x5000);
        let this = character(&mut e, reference);
        e.call_log = Some(vec![]);
        e.call(0x0044_1ba0, &args![this]);
        assert_eq!(
            calls_to(&e, PLAYER_QUEUE_FILES),
            vec![vec![0x6666, PRIORITY as u32, this.addr()]]
        );
        assert_eq!(
            order_of(&e, &[PLAYER_QUEUE_FILES, MODEL_LOADER_QUEUE_HEAD]),
            vec![PLAYER_QUEUE_FILES, MODEL_LOADER_QUEUE_HEAD]
        );
        assert_eq!(entered_lines(&e)[..2], [0x94e, 0x8ca]);
    }

    /// Doubles for the `QueuedFileEntry` functions the file load tasks use;
    /// `00449050` adds 100 to its kind so that the tests can tell it apart.
    fn file_entry_doubles(e: &mut Engine) {
        e.register(QUEUED_FILE_ENTRY_CONSTRUCT, |_, a| a[0].into_ret());
        noop(
            e,
            &[
                QUEUED_FILE_ENTRY_DESTRUCT,
                QUEUED_FILE_ENTRY_SET_NAME,
                QUEUED_FILE_ENTRY_SET_INDEX,
                MESH_DATA_POINTER_CONSTRUCT,
                MESH_DATA_POINTER_DESTRUCT,
                LOADED_FILE_POINTER_DESTRUCT,
                TASK_SET_STATE_5,
            ],
        );
        e.register(FILE_INDEX_OF_KIND, |_, a| (a[0] + 100).into_ret());
        for addr in [LOADED_FILE_POINTER_CONSTRUCT, LOADED_FILE_POINTER_ASSIGN] {
            e.register(addr, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                a[0].into_ret()
            });
        }
        e.register(MESH_DATA_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
    }

    #[test]
    fn file_load_constructor_names_the_file_and_finds_its_entry() {
        let mut e = engine();
        file_entry_doubles(&mut e);
        let this = e.new_object::<QueuedFileLoad>();
        e.call_log = Some(vec![]);
        let result = e.call(0x0044_1c50, &args![this, 0x7777u32, 0x31u32, 2u32]);
        assert_eq!(result.u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), 0x0101_6dc4);
        assert_eq!(e.get(this, QueuedFileLoad::eFileIndex), 2);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_CONSTRUCT),
            vec![vec![this.addr(), 0x31]]
        );
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_SET_NAME),
            vec![vec![this.addr(), 0x7777]]
        );
        assert_eq!(calls_to(&e, FILE_INDEX_OF_KIND), vec![vec![2]]);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_SET_INDEX),
            vec![vec![this.addr(), 102]]
        );
        assert_eq!(
            order_of(
                &e,
                &[
                    QUEUED_FILE_ENTRY_CONSTRUCT,
                    QUEUED_FILE_ENTRY_SET_NAME,
                    QUEUED_FILE_ENTRY_SET_INDEX
                ]
            ),
            vec![
                QUEUED_FILE_ENTRY_CONSTRUCT,
                QUEUED_FILE_ENTRY_SET_NAME,
                QUEUED_FILE_ENTRY_SET_INDEX
            ]
        );
    }

    #[test]
    fn file_load_destructors_release_the_loaded_file_and_free_on_request() {
        let mut e = engine();
        file_entry_doubles(&mut e);
        let this = e.new_object::<QueuedFileLoad>();
        e.call_log = Some(vec![]);
        e.call(0x0044_1d20, &args![this]);
        assert_eq!(
            calls_to(&e, LOADED_FILE_POINTER_DESTRUCT),
            vec![vec![this.addr() + 0x30]]
        );
        assert_eq!(
            order_of(
                &e,
                &[LOADED_FILE_POINTER_DESTRUCT, QUEUED_FILE_ENTRY_DESTRUCT]
            ),
            vec![LOADED_FILE_POINTER_DESTRUCT, QUEUED_FILE_ENTRY_DESTRUCT]
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_1cf0, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
        e.call_log = Some(vec![]);
        e.call(0x0044_1cf0, &args![this, 0u32]);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        assert_eq!(calls_to(&e, QUEUED_FILE_ENTRY_DESTRUCT).len(), 1);
    }

    #[test]
    fn loaded_file_constructor_takes_the_file_and_finishes_the_task() {
        let mut e = engine();
        file_entry_doubles(&mut e);
        let this = e.new_object::<QueuedFileLoad>();
        e.call_log = Some(vec![]);
        let result = e.call(0x0044_1d80, &args![this, Ptr::<()>::new(0x8888), 0x31u32]);
        assert_eq!(result.u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), 0x0101_6dc4);
        assert_eq!(e.get(this, QueuedFileLoad::eFileIndex), 3);
        assert_eq!(e.mem.u32(this.addr() + 0x30), 0x8888);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_CONSTRUCT),
            vec![vec![this.addr(), 0x31]]
        );
        assert_eq!(calls_to(&e, TASK_SET_STATE_5), vec![vec![this.addr()]]);
    }

    #[test]
    fn kf_queue_me_calls_the_io_manager_virtual_function_0x48() {
        let mut e = engine();
        vtable(&mut e, OBJECT_VTABLE, &[(0x48, SLOT_A)]);
        let manager = object(&mut e, OBJECT_VTABLE, 0x20);
        e.set_global(TASK_QUEUE, manager);
        let this = e.new_object::<QueuedKF>();
        e.call_log = Some(vec![]);
        e.call(0x0044_1e10, &args![this]);
        assert_eq!(calls_to(&e, SLOT_A), vec![vec![manager, this.addr()]]);
    }

    /// A file load task for `name` with the given kind, and doubles for the
    /// loader's lookup (`found`), `GetFile` (`file`) and the `LoadedFile`
    /// construction.
    fn file_load_setup(e: &mut Engine, found: u32, file: u32) -> Ptr<QueuedFileLoad> {
        file_entry_doubles(e);
        e.register(QUEUED_FILE_ENTRY_GET_NAME, |_, _| 0x7777u32.into_ret());
        answer(e, MODEL_LOADER_FIND_LOADED_FILE, found);
        answer(e, QUEUED_FILE_ENTRY_GET_FILE, file);
        e.register(NI_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LOADED_FILE_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(LOG_MESSAGE, |_, _| Ret::default());
        let this = e.new_object::<QueuedFileLoad>();
        e.set(this, QueuedFileLoad::eContext, CONTEXT);
        e.set(this, QueuedFileLoad::eFileIndex, 1);
        this
    }

    #[test]
    fn file_load_run_keeps_a_file_the_loader_already_has() {
        let mut e = engine();
        let this = file_load_setup(&mut e, 0x4242, 0x1111);
        e.call_log = Some(vec![]);
        e.call(0x0044_1e40, &args![this]);
        let loader = loader(&e);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_FIND_LOADED_FILE),
            vec![vec![loader, 0x7777]]
        );
        assert_eq!(e.mem.u32(this.addr() + 0x30), 0x4242);
        assert!(calls_to(&e, QUEUED_FILE_ENTRY_GET_FILE).is_empty());
        assert_eq!(entered_lines(&e), vec![0x976]);
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
    }

    #[test]
    fn file_load_run_builds_a_loaded_file_for_the_file_it_finds() {
        let mut e = engine();
        let this = file_load_setup(&mut e, 0, 0x1111);
        e.call_log = Some(vec![]);
        e.call(0x0044_1e40, &args![this]);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_GET_FILE),
            vec![vec![this.addr(), 101, 0xffff]]
        );
        assert_eq!(calls_to(&e, NI_ALLOC), vec![vec![0x10]]);
        let block = e.mem.u32(this.addr() + 0x30);
        assert_ne!(block, 0);
        assert_eq!(
            calls_to(&e, LOADED_FILE_CONSTRUCT),
            vec![vec![block, 0x7777, 0x1111]]
        );
        assert!(calls_to(&e, LOG_MESSAGE).is_empty());
    }

    #[test]
    fn file_load_run_stores_nothing_when_the_allocation_fails() {
        let mut e = engine();
        let this = file_load_setup(&mut e, 0, 0x1111);
        e.register(NI_ALLOC, |_, _| 0u32.into_ret());
        e.mem.set_u32(this.addr() + 0x30, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_1e40, &args![this]);
        assert!(calls_to(&e, LOADED_FILE_CONSTRUCT).is_empty());
        assert_eq!(e.mem.u32(this.addr() + 0x30), 0);
        // The assign function was still called, with a null file.
        assert_eq!(
            calls_to(&e, LOADED_FILE_POINTER_ASSIGN).last().unwrap(),
            &vec![this.addr() + 0x30, 0]
        );
    }

    #[test]
    fn file_load_run_logs_a_missing_file() {
        let mut e = engine();
        let this = file_load_setup(&mut e, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_1e40, &args![this]);
        assert_eq!(calls_to(&e, LOG_MESSAGE), vec![vec![0x0101_6df4, 0x7777]]);
        assert!(calls_to(&e, NI_ALLOC).is_empty());
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
    }

    #[test]
    fn face_gen_file_constructors_set_the_table_and_empty_pointers() {
        // From a file name.
        let mut e = engine();
        file_entry_doubles(&mut e);
        let this = e.new_object::<QueuedFaceGenFile>();
        e.call_log = Some(vec![]);
        let result = e.call(0x0044_1f80, &args![this, 0x7777u32, 0x31u32, 0u32]);
        assert_eq!(result.u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), 0x0101_6e1c);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_SET_NAME),
            vec![vec![this.addr(), 0x7777]]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_CONSTRUCT),
            vec![vec![this.addr() + 0x38, 0]]
        );
        assert_eq!(
            calls_to(&e, MESH_DATA_POINTER_CONSTRUCT),
            vec![vec![this.addr() + 0x3c, 0]]
        );
        // From an already loaded file.
        let mut e = engine();
        file_entry_doubles(&mut e);
        let this = e.new_object::<QueuedFaceGenFile>();
        e.call_log = Some(vec![]);
        let result = e.call(0x0044_20b0, &args![this, Ptr::<()>::new(0x8888), 0x31u32]);
        assert_eq!(result.u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), 0x0101_6e1c);
        assert_eq!(e.mem.u32(this.addr() + 0x30), 0x8888);
        assert_eq!(calls_to(&e, TASK_SET_STATE_5), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, NI_POINTER_CONSTRUCT).len(), 1);
        assert_eq!(calls_to(&e, MESH_DATA_POINTER_CONSTRUCT).len(), 1);
    }

    #[test]
    fn face_gen_file_constructor_from_models_assigns_both_pointers() {
        let mut e = engine();
        file_entry_doubles(&mut e);
        let this = e.new_object::<QueuedFaceGenFile>();
        e.call_log = Some(vec![]);
        let result = e.call(
            0x0044_2130,
            &args![this, Ptr::<()>::new(0xaaaa), Ptr::<()>::new(0xbbbb)],
        );
        assert_eq!(result.u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), 0x0101_6e1c);
        assert_eq!(e.mem.u32(this.addr() + 0x38), 0xaaaa);
        assert_eq!(e.mem.u32(this.addr() + 0x3c), 0xbbbb);
        // The loaded file constructor ran with no file and the context 5.
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_CONSTRUCT),
            vec![vec![this.addr(), 5]]
        );
        assert_eq!(
            e.get(this.cast::<QueuedFileLoad>(), QueuedFileLoad::eFileIndex),
            3
        );
    }

    #[test]
    fn face_gen_file_destructors_release_both_pointers_then_the_file_load() {
        let mut e = engine();
        file_entry_doubles(&mut e);
        let this = e.new_object::<QueuedFaceGenFile>();
        e.call_log = Some(vec![]);
        e.call(0x0044_2040, &args![this]);
        assert_eq!(
            calls_to(&e, MESH_DATA_POINTER_DESTRUCT),
            vec![vec![this.addr() + 0x3c]]
        );
        assert_eq!(
            calls_to(&e, NI_POINTER_DESTRUCT),
            vec![vec![this.addr() + 0x38]]
        );
        assert_eq!(
            order_of(
                &e,
                &[
                    MESH_DATA_POINTER_DESTRUCT,
                    NI_POINTER_DESTRUCT,
                    LOADED_FILE_POINTER_DESTRUCT,
                    QUEUED_FILE_ENTRY_DESTRUCT
                ]
            ),
            vec![
                MESH_DATA_POINTER_DESTRUCT,
                NI_POINTER_DESTRUCT,
                LOADED_FILE_POINTER_DESTRUCT,
                QUEUED_FILE_ENTRY_DESTRUCT
            ]
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_2010, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
        e.call_log = Some(vec![]);
        e.call(0x0044_2010, &args![this, 0u32]);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
    }

    /// Doubles for the `BSTaskThread` constructor and the clone queue.
    fn thread_doubles(e: &mut Engine) {
        e.register(TASK_THREAD_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(CLONE_QUEUE_CONSTRUCT, |_, a| a[0].into_ret());
        noop(e, &[TASK_THREAD_DESTRUCT]);
    }

    #[test]
    fn background_clone_thread_constructor_makes_the_queue() {
        let mut e = engine();
        thread_doubles(&mut e);
        let this = e.new_object::<BackgroundCloneThread>();
        e.mem.set_u8(this.addr() + 0x30, 9);
        e.mem.set_u32(this.addr() + 0x34, 9);
        e.call_log = Some(vec![]);
        let result = e.call(0x0044_21d0, &args![this, 10u32]);
        assert_eq!(result.u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), 0x0101_6e50);
        assert_eq!(
            calls_to(&e, TASK_THREAD_CONSTRUCT),
            vec![vec![this.addr(), 3, 0x0101_6e58]]
        );
        assert_eq!(e.get(this, BackgroundCloneThread::bExit), 0);
        assert_eq!(e.get(this, BackgroundCloneThread::iRunningCount), 0);
        let queue = e.get(this, BackgroundCloneThread::pCloneReferencesQueue);
        assert!(!queue.is_null());
        assert_eq!(calls_to(&e, MEMORY_ALLOC), vec![vec![0x40]]);
        assert_eq!(
            calls_to(&e, CLONE_QUEUE_CONSTRUCT),
            vec![vec![queue.addr(), 10, 8]]
        );
    }

    #[test]
    fn background_clone_thread_constructor_leaves_the_queue_null_when_allocation_fails() {
        let mut e = engine();
        thread_doubles(&mut e);
        e.register(MEMORY_ALLOC, |_, _| 0u32.into_ret());
        let this = e.new_object::<BackgroundCloneThread>();
        e.mem.set_u32(this.addr() + 0x38, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0044_21d0, &args![this, 10u32]);
        assert!(e
            .get(this, BackgroundCloneThread::pCloneReferencesQueue)
            .is_null());
        assert!(calls_to(&e, CLONE_QUEUE_CONSTRUCT).is_empty());
    }

    #[test]
    fn background_clone_thread_destructor_deletes_the_queue() {
        let mut e = engine();
        thread_doubles(&mut e);
        vtable(&mut e, OBJECT_VTABLE, &[(0x0, SLOT_A)]);
        let queue = object(&mut e, OBJECT_VTABLE, 0x20);
        let this = e.new_object::<BackgroundCloneThread>();
        e.set(
            this,
            BackgroundCloneThread::pCloneReferencesQueue,
            Ptr::<()>::new(queue),
        );
        e.call_log = Some(vec![]);
        e.call(0x0044_22c0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), 0x0101_6e50);
        assert_eq!(calls_to(&e, SLOT_A), vec![vec![queue, 1]]);
        assert_eq!(calls_to(&e, TASK_THREAD_DESTRUCT), vec![vec![this.addr()]]);
        // Without a queue only the base destructor runs.
        e.set(
            this,
            BackgroundCloneThread::pCloneReferencesQueue,
            Ptr::<()>::NULL,
        );
        e.call_log = Some(vec![]);
        e.call(0x0044_22c0, &args![this]);
        assert!(calls_to(&e, SLOT_A).is_empty());
        assert_eq!(calls_to(&e, TASK_THREAD_DESTRUCT).len(), 1);
    }

    #[test]
    fn background_clone_thread_scalar_deleting_destructor_frees_on_request() {
        let mut e = engine();
        thread_doubles(&mut e);
        let this = e.new_object::<BackgroundCloneThread>();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_2290, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, TASK_THREAD_DESTRUCT).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x0044_2290, &args![this, 0u32]);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
    }

    /// A task for the thread tests: state at +0xC.
    fn clone_task(e: &mut Engine, state: u32) -> u32 {
        let task = object(e, TASK_VTABLE, 0x40);
        e.mem.set_u32(task + 0xc, state);
        task
    }

    /// Doubles for `ThreadUpdate`: the wait sets `bExit` the second time it
    /// is called, the pops hand out the given tasks in order and then
    /// nothing, and the state functions follow the state word.
    fn thread_update_setup(e: &mut Engine, tasks: Vec<u32>) -> Ptr<BackgroundCloneThread> {
        let mut waits = 0;
        e.register_double(THREAD_WAIT_FOR_WORK, move |e, a| {
            waits += 1;
            if waits == 2 {
                e.mem.set_u8(a[0] + 0x30, 1);
            }
            Ret::default()
        });
        let mut tasks = tasks.into_iter();
        e.register_double(CLONE_QUEUE_POP, move |e, a| {
            e.mem.set_u32(a[1], tasks.next().unwrap_or(0));
            Ret::default()
        });
        noop(
            e,
            &[
                INTERLOCKED_INCREMENT,
                THREAD_BEFORE_TASK,
                THREAD_AFTER_TASK,
                EMPTY_FUNCTION,
                IOMANAGER_ADD_POST_PROCESS_TASK,
            ],
        );
        decrement(e);
        e.register(TASK_GET_STATE, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(TASK_STATE_IS_6, |e, a| {
            ((e.mem.u32(a[0] + 0xc) == 6) as u32).into_ret()
        });
        e.register(TASK_CHANGE_STATE, |e, a| {
            // Changes the state word when it is the expected one.
            let ok = e.mem.u32(a[0] + 0xc) == a[1];
            if ok {
                e.mem.set_u32(a[0] + 0xc, a[2]);
            }
            (ok as u32).into_ret()
        });
        vtable(e, TASK_VTABLE, &[(0x38, ANSWER_A)]);
        let this = e.new_object::<BackgroundCloneThread>();
        let queue = e.mem.alloc(0x40);
        e.set(
            this,
            BackgroundCloneThread::pCloneReferencesQueue,
            Ptr::<()>::new(queue),
        );
        e.mem.set_u32(this.addr() + 0x34, 5);
        this
    }

    #[test]
    fn thread_update_post_processes_a_task_that_wants_it() {
        let mut e = engine();
        // The task is in state 2; the thread moves it to 3 and then 5.
        let this = thread_update_setup(&mut e, vec![]);
        let task = clone_task(&mut e, 2);
        // Replace the pops: the first round hands out the task.
        let mut once = vec![task].into_iter();
        e.register_double(CLONE_QUEUE_POP, move |e, a| {
            e.mem.set_u32(a[1], once.next().unwrap_or(0));
            Ret::default()
        });
        answer(&mut e, ANSWER_A, 1);
        e.call_log = Some(vec![]);
        e.call(0x0044_2350, &args![this]);
        assert_eq!(
            calls_to(&e, TASK_CHANGE_STATE),
            vec![vec![task, 2, 3], vec![task, 3, 5]]
        );
        assert_eq!(
            calls_to(&e, IOMANAGER_ADD_POST_PROCESS_TASK),
            vec![vec![task]]
        );
        assert_eq!(e.mem.u32(task + 0xc), 5);
        // Two rounds ran, each counted running and then done.
        assert_eq!(calls_to(&e, INTERLOCKED_INCREMENT).len(), 2);
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 2);
        assert_eq!(e.mem.i32(this.addr() + 0x34), 3);
        // The task bracketing calls: once per task, and once for the
        // empty round.
        assert_eq!(calls_to(&e, THREAD_BEFORE_TASK).len(), 2);
        assert_eq!(calls_to(&e, THREAD_AFTER_TASK).len(), 2);
        assert_eq!(calls_to(&e, TASK_POINTER_DESTRUCT).len(), 2);
    }

    #[test]
    fn thread_update_skips_finished_and_unchangeable_tasks_and_unwanted_post_processing() {
        let mut e = engine();
        let this = thread_update_setup(&mut e, vec![]);
        let finished = clone_task(&mut e, 6);
        let changed_elsewhere = clone_task(&mut e, 4);
        let quiet = clone_task(&mut e, 2);
        // One round hands out three tasks one after the other.
        let mut tasks = vec![finished, changed_elsewhere, quiet].into_iter();
        e.register_double(CLONE_QUEUE_POP, move |e, a| {
            e.mem.set_u32(a[1], tasks.next().unwrap_or(0));
            Ret::default()
        });
        // The state getter reports 2 for the second task so that the
        // exchange to 3 fails.
        e.register_double(TASK_GET_STATE, move |e, a| {
            if a[0] == changed_elsewhere {
                2u32.into_ret()
            } else {
                e.mem.u32(a[0] + 0xc).into_ret()
            }
        });
        answer(&mut e, ANSWER_A, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_2350, &args![this]);
        // Only the third task changed state, and it needs no post processing.
        assert_eq!(
            calls_to(&e, TASK_CHANGE_STATE),
            vec![
                vec![changed_elsewhere, 2, 3],
                vec![quiet, 2, 3],
                vec![quiet, 3, 5]
            ]
        );
        assert!(calls_to(&e, IOMANAGER_ADD_POST_PROCESS_TASK).is_empty());
        assert_eq!(e.mem.u32(finished + 0xc), 6);
        assert_eq!(e.mem.u32(changed_elsewhere + 0xc), 4);
        assert_eq!(e.mem.u32(quiet + 0xc), 5);
        assert_eq!(calls_to(&e, THREAD_BEFORE_TASK).len(), 4);
    }

    #[test]
    fn thread_update_without_a_flag_change_does_not_run_when_exit_is_set() {
        let mut e = engine();
        let this = thread_update_setup(&mut e, vec![]);
        e.mem.set_u8(this.addr() + 0x30, 1);
        e.call_log = Some(vec![]);
        e.call(0x0044_2350, &args![this]);
        assert!(calls_to(&e, THREAD_WAIT_FOR_WORK).is_empty());
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
    }

    #[test]
    fn thread_queue_add_wraps_the_task_and_wakes_the_thread() {
        let mut e = engine();
        noop(&mut e, &[FINISHED_QUEUE_ADD, THREAD_WAKE]);
        let this = e.new_object::<BackgroundCloneThread>();
        let queue = e.mem.alloc(0x40);
        e.set(
            this,
            BackgroundCloneThread::pCloneReferencesQueue,
            Ptr::<()>::new(queue),
        );
        e.register(FINISHED_QUEUE_ADD, |e, a| {
            // The temporary task pointer holds the task.
            let task = e.mem.u32(a[1]);
            e.mem.set_u32(SEEN_C, task);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0044_2580, &args![this, Ptr::<()>::new(0x9999)]);
        assert_eq!(e.mem.u32(SEEN_C), 0x9999);
        let added = calls_to(&e, FINISHED_QUEUE_ADD);
        assert_eq!(added[0][0], queue);
        assert_eq!(calls_to(&e, TASK_POINTER_CONSTRUCT)[0][1], 0x9999);
        assert_eq!(calls_to(&e, TASK_POINTER_DESTRUCT)[0][0], added[0][1]);
        assert_eq!(calls_to(&e, THREAD_WAKE), vec![vec![this.addr()]]);
        assert_eq!(
            order_of(
                &e,
                &[
                    TASK_POINTER_CONSTRUCT,
                    FINISHED_QUEUE_ADD,
                    TASK_POINTER_DESTRUCT,
                    THREAD_WAKE
                ]
            ),
            vec![
                TASK_POINTER_CONSTRUCT,
                FINISHED_QUEUE_ADD,
                TASK_POINTER_DESTRUCT,
                THREAD_WAKE
            ]
        );
    }

    #[test]
    fn thread_exit_sets_the_exit_flag() {
        let mut e = engine();
        let this = e.new_object::<BackgroundCloneThread>();
        e.call(0x0044_2630, &args![this]);
        assert_eq!(e.get(this, BackgroundCloneThread::bExit), 1);
    }

    /// Doubles that make every loader member constructor return its block,
    /// and the thread, the IO manager registrations.
    fn loader_constructor_doubles(e: &mut Engine) {
        for addr in [
            MODEL_MAP_CONSTRUCT,
            KF_MODEL_MAP_CONSTRUCT,
            REFERENCE_MAP_CONSTRUCT,
            ANIM_IDLE_MAP_CONSTRUCT,
            REPLACEMENT_KF_LIST_MAP_CONSTRUCT,
            HELMET_MAP_CONSTRUCT,
            ATTACH_TASK_QUEUE_CONSTRUCT,
            TEXTURE_MAP_CONSTRUCT,
            LOADED_FILE_MAP_CONSTRUCT,
        ] {
            e.register(addr, |_, a| a[0].into_ret());
        }
        thread_doubles(e);
        noop(
            e,
            &[
                THREAD_START,
                IO_MANAGER_REGISTER_FIRST,
                IO_MANAGER_REGISTER_SECOND,
                IO_MANAGER_REGISTER_FOURTH,
                IO_MANAGER_REGISTER_FIFTH,
                IO_MANAGER_REGISTER_SIXTH,
            ],
        );
    }

    #[test]
    fn model_loader_constructor_builds_the_maps_and_the_thread() {
        let mut e = engine();
        loader_constructor_doubles(&mut e);
        e.set_global(TASK_QUEUE, 0u32);
        let this = e.new_object::<ModelLoader>();
        e.mem.set_u8(this.addr() + 0x2c, 7);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_2650, &args![this]).u32(), this.addr());
        // Every member holds the block its constructor was given.
        let members = [
            (0x00, MODEL_MAP_CONSTRUCT, vec![10, 0x3f1, 0xc]),
            (0x04, KF_MODEL_MAP_CONSTRUCT, vec![10, 0x3f1, 0xc]),
            (0x08, REFERENCE_MAP_CONSTRUCT, vec![10, 0x13af, 0xc]),
            (0x0c, REFERENCE_MAP_CONSTRUCT, vec![10, 0x13af, 0xc]),
            (0x10, ANIM_IDLE_MAP_CONSTRUCT, vec![10, 0xfb, 0xc]),
            (0x14, REPLACEMENT_KF_LIST_MAP_CONSTRUCT, vec![10, 0xfb, 0xc]),
            (0x18, HELMET_MAP_CONSTRUCT, vec![10, 0xfb, 0xc]),
            (0x1c, ATTACH_TASK_QUEUE_CONSTRUCT, vec![10, 8]),
            (0x20, TEXTURE_MAP_CONSTRUCT, vec![10, 0x3f1, 0xc]),
            (0x24, LOADED_FILE_MAP_CONSTRUCT, vec![10, 0x3f1, 0xc]),
        ];
        let mut seen = Vec::new();
        for (offset, constructor, arguments) in members {
            let block = e.mem.u32(this.addr() + offset);
            assert_ne!(block, 0);
            assert!(!seen.contains(&block));
            seen.push(block);
            let mut expected = vec![block];
            expected.extend(arguments);
            assert!(calls_to(&e, constructor).contains(&expected));
        }
        // The thread is made with 10 and started; the delayed free flag is
        // cleared.
        let thread = e.mem.u32(this.addr() + 0x28);
        assert_ne!(thread, 0);
        assert_eq!(e.mem.u32(thread), 0x0101_6e50);
        assert_eq!(calls_to(&e, THREAD_START), vec![vec![thread]]);
        assert_eq!(calls_to(&e, CLONE_QUEUE_CONSTRUCT)[0][1..], [10, 8]);
        assert_eq!(e.get(this, ModelLoader::bHasDelayedFree), 0);
        // No IO manager: no registrations.
        assert!(calls_to(&e, IO_MANAGER_REGISTER_FIRST).is_empty());
    }

    #[test]
    fn model_loader_constructor_registers_the_callbacks_with_the_io_manager() {
        let mut e = engine();
        loader_constructor_doubles(&mut e);
        let manager = e.mem.alloc(0x100);
        e.set_global(TASK_QUEUE, manager);
        let this = e.new_object::<ModelLoader>();
        e.call_log = Some(vec![]);
        e.call(0x0044_2650, &args![this]);
        assert_eq!(
            calls_to(&e, IO_MANAGER_REGISTER_FIRST),
            vec![vec![manager, 0x0044_6e30]]
        );
        assert_eq!(
            calls_to(&e, IO_MANAGER_REGISTER_SECOND),
            vec![vec![manager, 0x0044_6e40]]
        );
        assert_eq!(e.mem.u32(manager + 0x78), 0x0044_6ea0);
        assert_eq!(
            calls_to(&e, IO_MANAGER_REGISTER_FOURTH),
            vec![vec![manager, 0x0044_6f00]]
        );
        assert_eq!(
            calls_to(&e, IO_MANAGER_REGISTER_FIFTH),
            vec![vec![manager, 0x0044_6f10]]
        );
        assert_eq!(
            calls_to(&e, IO_MANAGER_REGISTER_SIXTH),
            vec![vec![manager, 0x0044_6f90]]
        );
    }

    #[test]
    fn model_loader_constructor_stores_null_for_a_failed_allocation() {
        let mut e = engine();
        loader_constructor_doubles(&mut e);
        e.set_global(TASK_QUEUE, 0u32);
        // The very first allocation (the model map) fails.
        let mut failed = false;
        e.register_double(MEMORY_ALLOC, move |e, a| {
            if !failed {
                failed = true;
                0u32.into_ret()
            } else {
                e.mem.alloc(a[0]).into_ret()
            }
        });
        let this = e.new_object::<ModelLoader>();
        e.mem.set_u32(this.addr(), 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0044_2650, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), 0);
        assert_eq!(calls_to(&e, MODEL_MAP_CONSTRUCT).len(), 0);
        assert_ne!(e.mem.u32(this.addr() + 4), 0);
    }

    #[test]
    fn io_manager_setter_stores_the_callback_at_0x78() {
        let mut e = engine();
        let manager = e.mem.alloc(0x100);
        e.call(0x0044_2a80, &args![Ptr::<()>::new(manager), 0x4545u32]);
        assert_eq!(e.mem.u32(manager + 0x78), 0x4545);
    }

    #[test]
    fn model_ref_count_adds_both_counts() {
        let mut e = engine();
        let model = e.new_object::<Model>();
        e.set(model, Model::iRefCount, 3);
        e.set(model, Model::iManualRefCount, 4);
        assert_eq!(e.call(0x0044_3190, &args![model]).u32(), 7);
        e.set(model, Model::iRefCount, -2);
        e.set(model, Model::iManualRefCount, 1);
        assert_eq!(e.call(0x0044_3190, &args![model]).u32() as i32, -1);
    }

    // ---- ModelLoader::~ModelLoader ----

    const MAP_VTABLE: u32 = 0x0ff2_0000;
    const EMPTY_MAP_VTABLE: u32 = 0x0ff2_1000;
    /// The map's virtual functions: delete (slot 0), remove (0x14).
    const MAP_DELETE: u32 = 0x0ff0_0030;
    const MAP_REMOVE: u32 = 0x0ff0_0031;
    const MAP_COUNT_ONE: u32 = 0x0ff0_0032;

    /// What the destructor's iterator doubles hand out: per walk (model map,
    /// KF map first walk, KF map second walk) the entries (found, key,
    /// value).
    type Walks = Vec<Vec<(bool, u32, u32)>>;

    fn walk_doubles(e: &mut Engine, walks: Walks) {
        use std::cell::RefCell;
        use std::rc::Rc;
        let state = Rc::new(RefCell::new((walks, 0usize, 0usize)));
        for addr in [MODEL_ITERATOR_CONSTRUCT, KF_ITERATOR_CONSTRUCT] {
            let state = state.clone();
            e.register_double(addr, move |_, _| {
                let mut state = state.borrow_mut();
                state.1 += 1;
                state.2 = 0;
                Ret::default()
            });
        }
        let at_end = state.clone();
        e.register_double(ITERATOR_AT_END, move |_, _| {
            let state = at_end.borrow();
            ((state.2 >= state.0[state.1 - 1].len()) as u32).into_ret()
        });
        let next = state;
        e.register_double(MAP_NEXT, move |e, a| {
            let mut state = next.borrow_mut();
            let (found, key, value) = state.0[state.1 - 1][state.2];
            state.2 += 1;
            if found {
                e.mem.set_u32(a[2], key);
                e.mem.set_u32(a[3], value);
            }
            (found as u32).into_ret()
        });
        noop(
            e,
            &[
                MODEL_ITERATOR_DESTRUCT,
                KF_ITERATOR_DESTRUCT,
                MODEL_DELETE,
                KF_MODEL_DELETE,
                OUTPUT_MODEL_MAP_CONTENTS,
                LOG_MESSAGE,
            ],
        );
    }

    fn model_with_counts(e: &mut Engine, references: i32, manual: i32) -> u32 {
        let model = e.new_object::<Model>();
        e.set(model, Model::iRefCount, references);
        e.set(model, Model::iManualRefCount, manual);
        model.addr()
    }

    fn map_object(e: &mut Engine, table: u32) -> Ptr {
        Ptr::new(object(e, table, 0x40))
    }

    /// A loader whose members are all maps with the table `table` (the
    /// thread and queue included); returns it and the member addresses in
    /// the order of the fields.
    fn loader_with_members(e: &mut Engine, table: u32) -> (Ptr<ModelLoader>, Vec<u32>) {
        e.register(MAP_DELETE, |_, _| Ret::default());
        e.register(MAP_REMOVE, |_, _| Ret::default());
        e.register(MAP_COUNT_ONE, |_, _| 1u32.into_ret());
        vtable(
            e,
            MAP_VTABLE,
            &[(0x0, MAP_DELETE), (0x14, MAP_REMOVE), (0x44, MAP_COUNT_ONE)],
        );
        vtable(
            e,
            EMPTY_MAP_VTABLE,
            &[(0x0, MAP_DELETE), (0x14, MAP_REMOVE), (0x44, NO)],
        );
        let this = e.new_object::<ModelLoader>();
        let mut members = Vec::new();
        for offset in (0..0x2c).step_by(4) {
            let member = map_object(e, table);
            e.mem.set_u32(this.addr() + offset, member.addr());
            members.push(member.addr());
        }
        (this, members)
    }

    #[test]
    fn model_loader_destructor_counts_and_deletes_the_models_it_still_holds() {
        let mut e = engine();
        let (this, members) = loader_with_members(&mut e, MAP_VTABLE);
        let used = model_with_counts(&mut e, 1, 0);
        let manual = model_with_counts(&mut e, 0, 2);
        let free = model_with_counts(&mut e, 0, 0);
        // Model walk: three models and a miss; KF walks find nothing.
        walk_doubles(
            &mut e,
            vec![
                vec![
                    (true, 0, used),
                    (false, 0, 0),
                    (true, 0, manual),
                    (true, 0, free),
                ],
                vec![],
                vec![],
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0044_2aa0, &args![this]);
        assert_eq!(
            calls_to(&e, OUTPUT_MODEL_MAP_CONTENTS),
            vec![vec![this.addr(), 0, 0]]
        );
        assert_eq!(
            calls_to(&e, MODEL_DELETE),
            vec![vec![used, 1], vec![manual, 1], vec![free, 1]]
        );
        // Two models are still referenced.
        assert_eq!(calls_to(&e, LOG_MESSAGE)[0], vec![0x0101_6ea0, 2]);
        assert_eq!(calls_to(&e, MODEL_ITERATOR_DESTRUCT).len(), 1);
        // The members are deleted in the destructor's order.
        let by_offset = |offset: usize| members[offset / 4];
        let expected: Vec<Vec<u32>> = [
            0x00, 0x04, 0x1c, 0x18, 0x14, 0x10, 0x08, 0x0c, 0x28, 0x20, 0x24,
        ]
        .iter()
        .map(|offset| vec![by_offset(*offset), 1])
        .collect();
        assert_eq!(calls_to(&e, MAP_DELETE), expected);
    }

    #[test]
    fn model_loader_destructor_removes_kf_models_without_animation_and_counts_the_rest() {
        let mut e = engine();
        let (this, members) = loader_with_members(&mut e, MAP_VTABLE);
        let no_animation = model_with_counts(&mut e, 0, 0);
        let kept = model_with_counts(&mut e, 0, 0);
        let rejected = model_with_counts(&mut e, 0, 0);
        let animation_for_kept = e.mem.alloc(8);
        let animation_for_rejected = e.mem.alloc(8);
        // 005585e0 gives the animation, 005f26c0 accepts the second.
        let mut table = std::collections::HashMap::new();
        table.insert(no_animation, 0u32);
        table.insert(kept, animation_for_kept);
        table.insert(rejected, animation_for_rejected);
        e.register_double(KF_MODEL_ANIMATION, move |_, a| table[&a[0]].into_ret());
        e.register_double(ANIMATION_TEST, move |_, a| {
            ((a[0] == animation_for_rejected) as u32).into_ret()
        });
        e.register(KF_MODEL_REF_COUNT, |e, a| {
            // Only the model with a reference counts.
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.mem.set_u32(kept + 4, 1);
        walk_doubles(
            &mut e,
            vec![
                vec![],
                vec![
                    (true, 0xa1, no_animation),
                    (true, 0xa2, kept),
                    (true, 0xa3, rejected),
                    (false, 0, 0),
                    (true, 0xa4, 0),
                ],
                vec![(true, 0xb1, kept), (true, 0xb2, no_animation)],
            ],
        );
        e.call_log = Some(vec![]);
        e.call(0x0044_2aa0, &args![this]);
        let kf_map = members[1];
        assert_eq!(
            calls_to(&e, MAP_REMOVE),
            vec![vec![kf_map, 0xa1], vec![kf_map, 0xa3]]
        );
        let deleted = calls_to(&e, KF_MODEL_DELETE);
        assert_eq!(
            deleted,
            vec![
                vec![no_animation, 1],
                vec![rejected, 1],
                vec![kept, 1],
                vec![no_animation, 1]
            ]
        );
        // One KF model is still referenced.
        assert_eq!(calls_to(&e, LOG_MESSAGE), vec![vec![0x0101_6e70, 1]]);
        assert_eq!(calls_to(&e, KF_ITERATOR_DESTRUCT).len(), 2);
        // The model map walk did not start a second iterator.
        assert_eq!(calls_to(&e, MODEL_ITERATOR_CONSTRUCT).len(), 1);
    }

    #[test]
    fn model_loader_destructor_skips_maps_without_entries_and_null_members() {
        let mut e = engine();
        let (this, members) = loader_with_members(&mut e, EMPTY_MAP_VTABLE);
        // The KF walk still happens (it is unconditional); the others do not.
        walk_doubles(&mut e, vec![vec![]]);
        // Null members are skipped.
        e.mem.set_u32(this.addr() + 0x28, 0);
        e.mem.set_u32(this.addr() + 0x1c, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_2aa0, &args![this]);
        assert!(calls_to(&e, MODEL_ITERATOR_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&e, KF_ITERATOR_CONSTRUCT).len(), 1);
        assert!(calls_to(&e, LOG_MESSAGE).is_empty());
        let deleted: Vec<u32> = calls_to(&e, MAP_DELETE).iter().map(|a| a[0]).collect();
        assert_eq!(deleted.len(), 9);
        assert!(!deleted.contains(&members[0x28 / 4]));
        assert!(!deleted.contains(&members[0x1c / 4]));
        assert_eq!(deleted[0], members[0]);
    }

    // ---- Third batch (0x004431b0 to 0x00445750).

    const QUEUE_TASK_VTABLE: u32 = 0x0ff1_4000;
    const TES_MODEL_VTABLE: u32 = 0x0ff1_5000;
    const SINK_VTABLE: u32 = 0x0ff1_6000;
    const POINTER_VTABLE: u32 = 0x0ff1_7000;
    /// The value the `TESModel` virtual function `0x14` gives (its name).
    const TES_MODEL_NAME: u32 = 0x0000_7100;
    /// The text buffer the string formatter double leaves in the string object.
    const REPORT_BUFFER: u32 = 0x0000_9990;

    /// The doubles of the numbered map `id`: `Find` (slot 8), `Add` (0x10),
    /// `Remove` (0x14) and the count (0x44).
    fn find_double(id: u32) -> u32 {
        0x0ff5_0000 + id * 0x10
    }
    fn add_double(id: u32) -> u32 {
        find_double(id) + 1
    }
    fn remove_double(id: u32) -> u32 {
        find_double(id) + 2
    }

    /// A map object with the numbered doubles: `Find` writes `found` into the
    /// out pointer and answers yes when it is given, `Add` answers
    /// `add_result`.
    fn q_map(e: &mut Engine, id: u32, found: Option<u32>, add_result: bool) -> Ptr {
        e.register_double(find_double(id), move |e, a| match found {
            Some(value) => {
                e.mem.set_u32(a[2], value);
                1u32.into_ret()
            }
            None => 0u32.into_ret(),
        });
        answer(e, add_double(id), add_result as u32);
        e.register(remove_double(id), |_, _| Ret::default());
        let table = 0x0ff6_0000 + id * 0x400;
        vtable(
            e,
            table,
            &[
                (0x8, find_double(id)),
                (0x10, add_double(id)),
                (0x14, remove_double(id)),
            ],
        );
        Ptr::new(object(e, table, 0x20))
    }

    /// An engine with the pointer assignments (`006f74f0`, `0092c820`) and
    /// the pages the third batch reads.
    fn qe() -> Engine {
        let mut e = engine();
        e.map(0x011d_d000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.register(TASK_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(NI_POINTER_ASSIGN_POINTER, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            a[0].into_ret()
        });
        e
    }

    /// A task object: virtual functions `0x1c`, `0x20`, `0x28` and `0x30` are
    /// `SLOT_A` to `SLOT_D`; its priority is `priority`.
    fn q_task(e: &mut Engine, priority: u8) -> u32 {
        vtable(
            e,
            QUEUE_TASK_VTABLE,
            &[
                (0x1c, SLOT_A),
                (0x20, SLOT_B),
                (0x28, SLOT_C),
                (0x30, SLOT_D),
            ],
        );
        let task = object(e, QUEUE_TASK_VTABLE, 0x60);
        e.set(
            Ptr::<IOTask>::new(task),
            IOTask::Key,
            (priority as u64) << 16,
        );
        task
    }

    /// A constructor at `addr` that answers `task`.
    fn q_constructor(e: &mut Engine, addr: u32, task: u32) {
        answer(e, addr, task);
    }

    /// A loader with the given (offset, map) members.
    fn q_loader(e: &mut Engine, maps: &[(u32, Ptr)]) -> Ptr<ModelLoader> {
        let this = e.new_object::<ModelLoader>();
        for (offset, map) in maps {
            e.mem.set_u32(this.addr() + offset, map.addr());
        }
        this
    }

    /// A zeroed block of `size` bytes.
    fn block(e: &mut Engine, size: u32) -> Ptr {
        Ptr::new(e.mem.alloc(size))
    }

    #[test]
    fn kf_model_count_is_the_sum_of_both_counts() {
        let mut e = qe();
        let model = e.new_object::<KFModel>();
        e.set(model, KFModel::iRefCount, 3);
        e.set(model, KFModel::iManualRefCount, 4);
        assert_eq!(e.call(0x0044_31b0, &args![model]).i32(), 7);
        e.set(model, KFModel::iManualRefCount, -3);
        assert_eq!(e.call(0x0044_31b0, &args![model]).i32(), 0);
    }

    #[test]
    fn map_iterator_destructors_run_their_base_destructors() {
        let mut e = qe();
        noop(
            &mut e,
            &[MODEL_ITERATOR_BASE_DESTRUCT, KF_ITERATOR_BASE_DESTRUCT],
        );
        let iterator = block(&mut e, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x0044_31d0, &args![iterator]);
        assert_eq!(
            calls_to(&e, MODEL_ITERATOR_BASE_DESTRUCT),
            vec![vec![iterator.addr()]]
        );
        assert!(calls_to(&e, KF_ITERATOR_BASE_DESTRUCT).is_empty());
        e.call(0x0044_3220, &args![iterator]);
        assert_eq!(
            calls_to(&e, KF_ITERATOR_BASE_DESTRUCT),
            vec![vec![iterator.addr()]]
        );
    }

    #[test]
    fn model_and_kf_model_deleting_destructors_free_on_bit_0() {
        let mut e = qe();
        noop(&mut e, &[MODEL_DESTRUCTOR_BODY, KF_MODEL_DESTRUCTOR_BODY]);
        for (function, body) in [
            (0x0044_31f0u32, MODEL_DESTRUCTOR_BODY),
            (0x0044_3240, KF_MODEL_DESTRUCTOR_BODY),
        ] {
            let this = block(&mut e, 0x20);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(function, &args![this, 0u32]).u32(), this.addr());
            assert_eq!(calls_to(&e, body), vec![vec![this.addr()]]);
            assert!(calls_to(&e, MEMORY_FREE).is_empty());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(function, &args![this, 1u32]).u32(), this.addr());
            assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this.addr()]]);
        }
    }

    /// Doubles for the report: the getters of the counts (`+4`, `+8`,
    /// `+0xc`, `+0x10`), the formatter, the string object and the sink.
    fn report_doubles(e: &mut Engine) -> Ptr {
        e.register(LIST_NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ARRAY_COUNT_AT_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(TASK_GET_STATE, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(OWNER_THREAD_ID, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        noop(
            e,
            &[
                FORMAT_STRING,
                STRING_CONSTRUCT,
                STRING_RELEASE,
                MODEL_ITERATOR_BASE_DESTRUCT,
                KF_ITERATOR_BASE_DESTRUCT,
            ],
        );
        e.register(STRING_FORMAT, |e, a| {
            e.mem.set_u32(a[0], REPORT_BUFFER);
            Ret::default()
        });
        answer(e, CRT_STRING_LENGTH, 6);
        vtable(
            e,
            SINK_VTABLE,
            &[
                (SINK_WRITE_TEXT_SLOT, SLOT_A),
                (SINK_WRITE_STRING_SLOT, SLOT_B),
            ],
        );
        Ptr::new(object(e, SINK_VTABLE, 0x10))
    }

    #[test]
    fn output_model_map_contents_lists_models_and_animations_to_the_log_and_sink() {
        let mut e = qe();
        let (this, _) = loader_with_members(&mut e, MAP_VTABLE);
        let sink = report_doubles(&mut e);
        let heavy = model_with_counts(&mut e, 2, 1);
        let idle = model_with_counts(&mut e, 0, 0);
        let animation = e.new_object::<KFModel>();
        e.set(animation, KFModel::iRefCount, 4);
        e.set(animation, KFModel::iManualRefCount, 5);
        let unused = e.new_object::<KFModel>();
        walk_doubles(
            &mut e,
            vec![
                vec![(true, 0x7001, heavy), (true, 0x7002, idle), (false, 0, 0)],
                vec![
                    (true, 0x7003, animation.addr()),
                    (true, 0x7004, unused.addr()),
                ],
            ],
        );
        e.call_log = Some(vec![]);
        // (walk_doubles stands in for 00443270 itself; run the function directly.)
        model_loader_output_model_map_contents(&mut e, this, sink, 0);
        // Only the model with a count is listed: sum, +4, +8 and the name.
        assert_eq!(
            calls_to(&e, FORMAT_STRING)
                .iter()
                .map(|a| a[2..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![MODELS_REPORT_FORMAT, 3, 2, 1, 0x7001]]
        );
        let line = calls_to(&e, FORMAT_STRING)[0][0];
        assert_eq!(calls_to(&e, FORMAT_STRING)[0][1], 0x400);
        // The text goes to the sink with its length plus one.
        assert_eq!(calls_to(&e, SLOT_A), vec![vec![sink.addr(), line, 7]]);
        // The animation: sum, +0xc (the old count), +0x10, and the name.
        let formats = calls_to(&e, STRING_FORMAT);
        assert_eq!(formats.len(), 1);
        assert_eq!(formats[0][1..], [ANIMATION_REPORT_FORMAT, 9, 4, 5, 0x7003]);
        assert_eq!(
            calls_to(&e, SLOT_B),
            vec![vec![sink.addr(), formats[0][0], 1]]
        );
        // Both lines are logged; the iterators are finished.
        assert_eq!(
            calls_to(&e, LOG_MESSAGE),
            vec![vec![line], vec![REPORT_BUFFER]]
        );
        assert_eq!(calls_to(&e, MODEL_ITERATOR_BASE_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&e, KF_ITERATOR_BASE_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&e, STRING_RELEASE).len(), 1);
    }

    #[test]
    fn output_model_map_contents_without_sink_lists_everything_when_asked() {
        let mut e = qe();
        let (this, _) = loader_with_members(&mut e, MAP_VTABLE);
        let _ = report_doubles(&mut e);
        let idle = model_with_counts(&mut e, 0, 0);
        walk_doubles(&mut e, vec![vec![(true, 0x7002, idle)], vec![]]);
        e.call_log = Some(vec![]);
        model_loader_output_model_map_contents(&mut e, this, Ptr::new(0), 1);
        assert_eq!(calls_to(&e, FORMAT_STRING).len(), 1);
        assert_eq!(calls_to(&e, FORMAT_STRING)[0][3..6], [0, 0, 0]);
        assert_eq!(calls_to(&e, LOG_MESSAGE).len(), 1);
        assert!(calls_to(&e, SLOT_A).is_empty());
        assert!(calls_to(&e, SLOT_B).is_empty());
    }

    #[test]
    fn output_model_map_contents_skips_maps_without_entries() {
        let mut e = qe();
        let (this, _) = loader_with_members(&mut e, EMPTY_MAP_VTABLE);
        let _ = report_doubles(&mut e);
        walk_doubles(&mut e, vec![]);
        e.call_log = Some(vec![]);
        model_loader_output_model_map_contents(&mut e, this, Ptr::new(0), 1);
        assert!(calls_to(&e, MODEL_ITERATOR_CONSTRUCT).is_empty());
        assert!(calls_to(&e, KF_ITERATOR_CONSTRUCT).is_empty());
        assert!(calls_to(&e, LOG_MESSAGE).is_empty());
    }

    /// Doubles for `fn_00443540` and a queued texture task in state `state`
    /// with priority 5; returns the loader and the task.
    fn takeover_setup(e: &mut Engine, state: u32) -> (Ptr<ModelLoader>, u32) {
        let task = q_task(e, 5);
        answer(e, LOADER_QUEUED_TEXTURE, task);
        answer(e, TASK_GET_STATE, state);
        answer(e, TASK_CHANGE_STATE, 1);
        answer(e, QUEUED_TEXTURE_FLAG_TEST, 0);
        answer(e, THREAD_QUEUED_FLAG_TEST, 0);
        noop(
            e,
            &[
                QUEUED_TEXTURE_FLAG_SET,
                QUEUED_FILE_ADD_ADDITIONAL_PARENT,
                IO_MANAGER_WAKE,
            ],
        );
        let this = e.new_object::<ModelLoader>();
        (this, task)
    }

    #[test]
    fn takeover_requeues_a_task_of_a_higher_priority_number() {
        let mut e = qe();
        let (this, task) = takeover_setup(&mut e, 1);
        answer(&mut e, QUEUED_TEXTURE_FLAG_TEST, 1);
        let parent = block(&mut e, 0x40);
        e.call_log = Some(vec![]);
        let taken = e.call(0x0044_3540, &args![this, 0x4444u32, 2i32, parent]);
        assert!(taken.bool());
        assert_eq!(
            calls_to(&e, LOADER_QUEUED_TEXTURE),
            vec![vec![this.addr(), 0x4444]]
        );
        // The state goes to 2 and back.
        assert_eq!(
            calls_to(&e, TASK_CHANGE_STATE),
            vec![vec![task, 1, 2], vec![task, 2, 1]]
        );
        // The flag is cleared while the thread flag is clear, the parent added.
        assert_eq!(calls_to(&e, QUEUED_TEXTURE_FLAG_SET), vec![vec![task, 0]]);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ADD_ADDITIONAL_PARENT),
            vec![vec![task, parent.addr()]]
        );
        // Priority 5 is above the requested 2: requeued.
        assert_eq!(calls_to(&e, SLOT_A), vec![vec![task, 2]]);
        assert!(calls_to(&e, IO_MANAGER_WAKE).is_empty());
    }

    #[test]
    fn takeover_wakes_the_manager_when_the_priority_is_not_worse() {
        let mut e = qe();
        let (this, task) = takeover_setup(&mut e, 0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x0044_3540,
                &args![this, 0x4444u32, 5i32, Ptr::<()>::new(0)]
            )
            .bool());
        let manager = e.global::<u32>(TASK_QUEUE);
        assert_eq!(calls_to(&e, IO_MANAGER_WAKE), vec![vec![manager]]);
        assert!(calls_to(&e, SLOT_A).is_empty());
        // No parent, flag not set: neither is touched.
        assert!(calls_to(&e, QUEUED_FILE_ADD_ADDITIONAL_PARENT).is_empty());
        assert!(calls_to(&e, QUEUED_TEXTURE_FLAG_SET).is_empty());
        let _ = task;
    }

    #[test]
    fn takeover_keeps_the_flag_while_the_thread_flag_is_set() {
        let mut e = qe();
        let (this, _) = takeover_setup(&mut e, 0);
        answer(&mut e, QUEUED_TEXTURE_FLAG_TEST, 1);
        answer(&mut e, THREAD_QUEUED_FLAG_TEST, 1);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_3540,
            &args![this, 0x4444u32, 5i32, Ptr::<()>::new(0)],
        );
        assert!(calls_to(&e, QUEUED_TEXTURE_FLAG_SET).is_empty());
    }

    #[test]
    fn takeover_refuses_without_a_task_a_late_state_or_a_state_change() {
        // No queued task.
        let mut e = qe();
        let (this, _) = takeover_setup(&mut e, 1);
        answer(&mut e, LOADER_QUEUED_TEXTURE, 0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0044_3540, &args![this, 1u32, 2i32, Ptr::<()>::new(0)])
            .bool());
        assert!(calls_to(&e, TASK_CHANGE_STATE).is_empty());
        // State 3 or more.
        let mut e = qe();
        let (this, _) = takeover_setup(&mut e, 3);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0044_3540, &args![this, 1u32, 2i32, Ptr::<()>::new(0)])
            .bool());
        assert!(calls_to(&e, TASK_CHANGE_STATE).is_empty());
        // The first state change is refused.
        let mut e = qe();
        let (this, _) = takeover_setup(&mut e, 1);
        answer(&mut e, TASK_CHANGE_STATE, 0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0044_3540, &args![this, 1u32, 2i32, Ptr::<()>::new(0)])
            .bool());
        assert_eq!(calls_to(&e, TASK_CHANGE_STATE).len(), 1);
        assert!(calls_to(&e, SLOT_A).is_empty());
    }

    #[test]
    fn takeover_refuses_when_the_state_cannot_be_put_back() {
        let mut e = qe();
        let (this, _) = takeover_setup(&mut e, 1);
        let mut first = true;
        e.register_double(TASK_CHANGE_STATE, move |_, _| {
            let result = first as u32;
            first = false;
            result.into_ret()
        });
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0044_3540, &args![this, 1u32, 2i32, Ptr::<()>::new(0)])
            .bool());
        assert_eq!(calls_to(&e, TASK_CHANGE_STATE).len(), 2);
        assert!(calls_to(&e, SLOT_A).is_empty());
        assert!(calls_to(&e, IO_MANAGER_WAKE).is_empty());
    }

    /// Doubles for the texture queueing: the palette gives `texture` (0 for
    /// none) in the state `state`, the file lookup gives `entry`; every
    /// queued texture constructor answers the returned task.
    fn texture_setup(texture: u32, state: u32, entry: u32) -> (Engine, Ptr<ModelLoader>, u32, Ptr) {
        let mut e = qe();
        let task = q_task(&mut e, 5);
        for palette in [
            TEXTURE_PALETTE_GET_TEXTURE_BY_NAME,
            TEXTURE_PALETTE_GET_TEXTURE_BY_ENTRY,
        ] {
            e.register_double(palette, move |e, a| {
                e.mem.set_u32(a[1], texture);
                Ret::default()
            });
        }
        answer(&mut e, QUEUED_TEXTURE_STATE, state);
        answer(&mut e, THREAD_QUEUED_FLAG_TEST, 0);
        answer(&mut e, FIND_FILE_ENTRY, entry);
        noop(
            &mut e,
            &[
                NORMALIZE_PATH,
                SPLIT_FILE_PATH,
                LIST_NODE_ITEM_ADDRESS,
                QUEUED_FILE_ENTRY_SET_FILE_NAME,
                QUEUED_FILE_ADD_CHILD,
                QUEUED_FILE_ADD_ADDITIONAL_PARENT,
                QUEUED_TEXTURE_FLAG_SET,
                IO_MANAGER_WAKE,
            ],
        );
        answer(&mut e, QUEUED_TEXTURE_FLAG_TEST, 0);
        answer(&mut e, LOADER_QUEUED_TEXTURE, 0);
        for constructor in [
            QUEUED_TEXTURE_FROM_TEXTURE,
            QUEUED_TEXTURE_FROM_ENTRY,
            QUEUED_TEXTURE_FROM_NAME,
        ] {
            q_constructor(&mut e, constructor, task);
        }
        let this = e.new_object::<ModelLoader>();
        let parent = block(&mut e, 0x40);
        (e, this, task, parent)
    }

    #[test]
    fn queue_texture_queues_a_texture_found_in_the_palette() {
        let (mut e, this, task, parent) = texture_setup(0x5555, 1, 0);
        e.call_log = Some(vec![]);
        let queued = e.call(0x0044_36c0, &args![this, 0x7777u32, 4i32, parent]);
        assert!(queued.bool());
        assert_eq!(
            calls_to(&e, TEXTURE_PALETTE_GET_TEXTURE_BY_NAME)
                .iter()
                .map(|a| a[0])
                .collect::<Vec<_>>(),
            vec![0x7777]
        );
        let built = calls_to(&e, QUEUED_TEXTURE_FROM_TEXTURE);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1..], [0x5555, 4]);
        // The parent is stored and told about its child; CheckFinished runs.
        assert_eq!(e.mem.u32(task + 0x1c), parent.addr());
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ADD_CHILD),
            vec![vec![parent.addr(), task]]
        );
        assert_eq!(calls_to(&e, SLOT_C), vec![vec![task]]);
        assert!(calls_to(&e, SLOT_B).is_empty());
        assert!(calls_to(&e, NORMALIZE_PATH).is_empty());
        assert_eq!(calls_to(&e, NI_POINTER_DESTRUCT).len(), 1);
    }

    #[test]
    fn queue_texture_reports_false_for_a_found_texture_without_a_parent() {
        let (mut e, this, _, _) = texture_setup(0x5555, 1, 0);
        e.call_log = Some(vec![]);
        let queued = e.call(
            0x0044_36c0,
            &args![this, 0x7777u32, 4i32, Ptr::<()>::new(0)],
        );
        assert!(!queued.bool());
        assert!(calls_to(&e, QUEUED_TEXTURE_FROM_TEXTURE).is_empty());
        assert!(calls_to(&e, NORMALIZE_PATH).is_empty());
    }

    #[test]
    fn queue_texture_by_name_builds_a_task_from_the_name_without_a_file_entry() {
        // No texture; a loaded texture seen by this thread counts as none too.
        for (texture, state) in [(0u32, 0u32), (0x5555, 3)] {
            let (mut e, this, task, parent) = texture_setup(texture, state, 0);
            e.call_log = Some(vec![]);
            let queued = e.call(0x0044_36c0, &args![this, 0x7777u32, 4i32, parent]);
            assert!(queued.bool());
            assert_eq!(calls_to(&e, NORMALIZE_PATH)[0][0], 0x7777);
            assert_eq!(calls_to(&e, NORMALIZE_PATH)[0][2], 0x104);
            assert_eq!(calls_to(&e, FIND_FILE_ENTRY)[0][0], 1);
            let built = calls_to(&e, QUEUED_TEXTURE_FROM_NAME);
            assert_eq!(built.len(), 1);
            assert_eq!(built[0][1..], [0x7777, 4]);
            assert!(calls_to(&e, QUEUED_FILE_ENTRY_SET_FILE_NAME).is_empty());
            assert_eq!(calls_to(&e, SLOT_B), vec![vec![task]]);
            assert_eq!(e.mem.u32(task + 0x1c), parent.addr());
        }
    }

    #[test]
    fn queue_texture_by_name_builds_a_task_from_the_file_entry() {
        let (mut e, this, task, parent) = texture_setup(0, 0, 0x6666);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0044_36c0, &args![this, 0x7777u32, 4i32, parent])
            .bool());
        let built = calls_to(&e, QUEUED_TEXTURE_FROM_ENTRY);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1..], [0x6666, 4]);
        // The normalised path becomes the task's file name.
        let path = calls_to(&e, NORMALIZE_PATH)[0][1];
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ENTRY_SET_FILE_NAME),
            vec![vec![task, path]]
        );
        assert_eq!(calls_to(&e, SLOT_B), vec![vec![task]]);
        assert!(calls_to(&e, QUEUED_TEXTURE_FROM_NAME).is_empty());
    }

    #[test]
    fn queue_texture_by_name_lets_a_queued_task_for_the_entry_be_taken_over() {
        let (mut e, this, task, parent) = texture_setup(0, 0, 0x6666);
        answer(&mut e, LOADER_QUEUED_TEXTURE, task);
        answer(&mut e, TASK_GET_STATE, 1);
        answer(&mut e, TASK_CHANGE_STATE, 1);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0044_36c0, &args![this, 0x7777u32, 4i32, parent])
            .bool());
        assert_eq!(
            calls_to(&e, LOADER_QUEUED_TEXTURE),
            vec![vec![this.addr(), 0x6666]]
        );
        assert!(calls_to(&e, QUEUED_TEXTURE_FROM_ENTRY).is_empty());
        assert!(calls_to(&e, SLOT_B).is_empty());
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ADD_ADDITIONAL_PARENT),
            vec![vec![task, parent.addr()]]
        );
    }

    #[test]
    fn queued_file_parent_setter_stores_the_parent_and_adds_the_child() {
        let mut e = qe();
        noop(&mut e, &[QUEUED_FILE_ADD_CHILD]);
        let task = block(&mut e, 0x28);
        let parent = block(&mut e, 0x28);
        e.call_log = Some(vec![]);
        e.call(0x0044_3aa0, &args![task, parent]);
        assert_eq!(e.mem.u32(task.addr() + 0x1c), parent.addr());
        assert_eq!(
            calls_to(&e, QUEUED_FILE_ADD_CHILD),
            vec![vec![parent.addr(), task.addr()]]
        );
        // A null parent is stored and nothing is added.
        e.call_log = Some(vec![]);
        e.call(0x0044_3aa0, &args![task, Ptr::<()>::new(0)]);
        assert_eq!(e.mem.u32(task.addr() + 0x1c), 0);
        assert!(calls_to(&e, QUEUED_FILE_ADD_CHILD).is_empty());
    }

    #[test]
    fn queue_texture_by_entry_queues_a_found_texture_or_none_without_a_parent() {
        let (mut e, this, task, parent) = texture_setup(0x5555, 2, 0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0044_3af0, &args![this, 0x6666u32, 3i32, parent])
            .bool());
        assert_eq!(
            calls_to(&e, TEXTURE_PALETTE_GET_TEXTURE_BY_ENTRY)[0][0],
            0x6666
        );
        assert_eq!(
            calls_to(&e, QUEUED_TEXTURE_FROM_TEXTURE)[0][1..],
            [0x5555, 3]
        );
        assert_eq!(calls_to(&e, SLOT_C), vec![vec![task]]);
        let (mut e, this, _, _) = texture_setup(0x5555, 2, 0);
        assert!(!e
            .call(
                0x0044_3af0,
                &args![this, 0x6666u32, 3i32, Ptr::<()>::new(0)]
            )
            .bool());
    }

    #[test]
    fn queue_texture_by_entry_builds_a_new_task_when_nothing_is_taken_over() {
        let (mut e, this, task, parent) = texture_setup(0, 0, 0);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0044_3af0, &args![this, 0x6666u32, 3i32, parent])
            .bool());
        assert_eq!(calls_to(&e, QUEUED_TEXTURE_FROM_ENTRY)[0][1..], [0x6666, 3]);
        assert_eq!(calls_to(&e, SLOT_B), vec![vec![task]]);
        assert_eq!(e.mem.u32(task + 0x1c), parent.addr());
        // A task queued for the entry is taken over instead.
        let (mut e, this, task, parent) = texture_setup(0, 0, 0);
        answer(&mut e, LOADER_QUEUED_TEXTURE, task);
        answer(&mut e, TASK_GET_STATE, 1);
        answer(&mut e, TASK_CHANGE_STATE, 1);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0044_3af0, &args![this, 0x6666u32, 3i32, parent])
            .bool());
        assert!(calls_to(&e, QUEUED_TEXTURE_FROM_ENTRY).is_empty());
        assert!(calls_to(&e, SLOT_B).is_empty());
    }

    /// Doubles for the model queueing: a `TESModel` whose name is
    /// `TES_MODEL_NAME`, every queued model constructor answering `task`, a
    /// loader whose model map (`+0`) finds `found`.
    fn model_setup(found: Option<u32>) -> (Engine, Ptr<ModelLoader>, u32, Ptr, Ptr) {
        let mut e = qe();
        let task = q_task(&mut e, 5);
        for constructor in [
            QUEUED_MODEL_FROM_TES_MODEL,
            QUEUED_MODEL_FROM_NAME,
            QUEUED_MODEL_FROM_MODEL,
        ] {
            q_constructor(&mut e, constructor, task);
        }
        e.register(QUEUED_MODEL_SET_FLAG_BIT_20, |_, _| Ret::default());
        noop(&mut e, &[QUEUED_FILE_ADD_CHILD, ADD_REFERENCE]);
        answer(&mut e, ANSWER_A, TES_MODEL_NAME);
        vtable(&mut e, TES_MODEL_VTABLE, &[(0x14, ANSWER_A)]);
        let tes_model = Ptr::new(object(&mut e, TES_MODEL_VTABLE, 0x20));
        let map = q_map(&mut e, 1, found, true);
        let this = q_loader(&mut e, &[(0x0, map)]);
        let out = block(&mut e, 4);
        (e, this, task, tes_model, out)
    }

    #[test]
    fn tes_model_queueing_builds_a_task_for_a_model_that_is_not_loaded() {
        let (mut e, this, task, tes_model, out) = model_setup(None);
        let parent = block(&mut e, 0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_3dc0,
            &args![this, tes_model, out, 5u32, parent, 6u32, 1u8, 0u8, 1u8, 2.5f32],
        );
        // The map is asked for the name the TESModel gives.
        let map = e.get(this, ModelLoader::pModelMap);
        assert_eq!(calls_to(&e, find_double(1)).len(), 1);
        assert_eq!(
            calls_to(&e, find_double(1))[0][..2],
            [map.addr(), TES_MODEL_NAME]
        );
        assert_eq!(
            calls_to(&e, QUEUED_MODEL_FROM_TES_MODEL)[0][1..],
            [tes_model.addr(), 5, 6, 1, 0]
        );
        // Visual distance, flag bit, parent, QueueMe, and the result pointer.
        assert_eq!(
            e.get(
                Ptr::<QueuedModel>::new(task),
                QueuedModel::mfOverriddenVisualDistance
            ),
            2.5
        );
        assert_eq!(
            calls_to(&e, QUEUED_MODEL_SET_FLAG_BIT_20),
            vec![vec![1, task + 0x3c]]
        );
        assert_eq!(e.mem.u32(task + 0x1c), parent.addr());
        assert_eq!(calls_to(&e, SLOT_B), vec![vec![task]]);
        assert_eq!(e.mem.u32(out.addr()), task);
        assert!(calls_to(&e, ADD_REFERENCE).is_empty());
    }

    #[test]
    fn tes_model_queueing_uses_a_loaded_model_for_the_wanted_parent() {
        let (mut e, this, task, tes_model, out) = model_setup(Some(0x6060));
        let parent = block(&mut e, 0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_3dc0,
            &args![this, tes_model, out, 5u32, parent, 6u32, 0u8, 1u8, 0u8, 1.5f32],
        );
        // The second flag gives the model a reference.
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![0x6060]]);
        assert_eq!(calls_to(&e, QUEUED_MODEL_FROM_MODEL)[0][1..], [0x6060, 5]);
        assert!(calls_to(&e, QUEUED_MODEL_FROM_TES_MODEL).is_empty());
        assert_eq!(
            e.get(
                Ptr::<QueuedModel>::new(task),
                QueuedModel::mfOverriddenVisualDistance
            ),
            1.5
        );
        assert_eq!(calls_to(&e, SLOT_C), vec![vec![task]]);
        assert!(calls_to(&e, SLOT_B).is_empty());
        assert!(calls_to(&e, QUEUED_MODEL_SET_FLAG_BIT_20).is_empty());
        assert_eq!(e.mem.u32(out.addr()), task);
    }

    #[test]
    fn tes_model_queueing_clears_the_result_for_a_loaded_model_without_parent() {
        let (mut e, this, _, tes_model, out) = model_setup(Some(0x6060));
        e.mem.set_u32(out.addr(), 0x1234);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_3dc0,
            &args![
                this,
                tes_model,
                out,
                5u32,
                Ptr::<()>::new(0),
                6u32,
                0u8,
                0u8,
                0u8,
                0.0f32
            ],
        );
        assert_eq!(e.mem.u32(out.addr()), 0);
        assert!(calls_to(&e, ADD_REFERENCE).is_empty());
        assert!(calls_to(&e, QUEUED_MODEL_FROM_MODEL).is_empty());
    }

    #[test]
    fn tes_model_queueing_wrapper_gives_a_null_result_pointer_and_no_distance() {
        let (mut e, this, task, tes_model, _) = model_setup(None);
        let parent = block(&mut e, 0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_3d30,
            &args![this, tes_model, 5u32, parent, 6u32, 1u8, 0u8, 0u8],
        );
        assert_eq!(calls_to(&e, SLOT_B), vec![vec![task]]);
        assert_eq!(
            e.get(
                Ptr::<QueuedModel>::new(task),
                QueuedModel::mfOverriddenVisualDistance
            ),
            0.0
        );
        // The holder constructed for the result started as null.
        assert_eq!(calls_to(&e, TASK_POINTER_CONSTRUCT)[0][1], 0);
    }

    #[test]
    fn queued_model_flag_and_distance_setters() {
        let mut e = qe();
        e.register(QUEUED_MODEL_SET_FLAG_BIT_20, |_, _| Ret::default());
        let model = e.new_object::<QueuedModel>();
        e.call_log = Some(vec![]);
        e.call(0x0044_3ff0, &args![model, 1u8]);
        assert_eq!(
            calls_to(&e, QUEUED_MODEL_SET_FLAG_BIT_20),
            vec![vec![1, model.addr() + 0x3c]]
        );
        e.call(0x0044_4020, &args![model, 123.5f32]);
        assert_eq!(e.get(model, QueuedModel::mfOverriddenVisualDistance), 123.5);
    }

    #[test]
    fn name_model_queueing_builds_a_task_for_a_model_that_is_not_loaded() {
        let (mut e, this, task, _, out) = model_setup(None);
        let parent = block(&mut e, 0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_40d0,
            &args![this, 0x7878u32, out, 3u32, parent, 8u32, 1u8, 1u8, 1u8],
        );
        assert_eq!(calls_to(&e, find_double(1))[0][1], 0x7878);
        assert_eq!(
            calls_to(&e, QUEUED_MODEL_FROM_NAME)[0][1..],
            [0x7878, 3, 8, 1, 1]
        );
        // The flag bit is set before the parent, then QueueMe runs.
        assert_eq!(
            calls_to(&e, QUEUED_MODEL_SET_FLAG_BIT_20),
            vec![vec![1, task + 0x3c]]
        );
        assert_eq!(calls_to(&e, SLOT_B), vec![vec![task]]);
        assert!(calls_to(&e, SLOT_D).is_empty());
        assert_eq!(e.mem.u32(out.addr()), task);
    }

    #[test]
    fn name_model_queueing_handles_a_loaded_model() {
        let (mut e, this, task, _, out) = model_setup(Some(0x6060));
        let parent = block(&mut e, 0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_40d0,
            &args![this, 0x7878u32, out, 3u32, parent, 8u32, 0u8, 1u8, 0u8],
        );
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![0x6060]]);
        assert_eq!(calls_to(&e, QUEUED_MODEL_FROM_MODEL)[0][1..], [0x6060, 3]);
        assert_eq!(calls_to(&e, SLOT_C), vec![vec![task]]);
        assert_eq!(e.mem.u32(out.addr()), task);
        // Without a parent the result is cleared.
        e.call(
            0x0044_40d0,
            &args![
                this,
                0x7878u32,
                out,
                3u32,
                Ptr::<()>::new(0),
                8u32,
                0u8,
                0u8,
                0u8
            ],
        );
        assert_eq!(e.mem.u32(out.addr()), 0);
    }

    #[test]
    fn model_queueing_wrapper_runs_the_name_overload_with_a_null_result() {
        let (mut e, this, task, _, _) = model_setup(None);
        let parent = block(&mut e, 0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_4040,
            &args![this, 0x7878u32, 3u32, parent, 8u32, 0u8, 0u8, 0u8],
        );
        assert_eq!(calls_to(&e, TASK_POINTER_CONSTRUCT)[0][1], 0);
        assert_eq!(calls_to(&e, SLOT_B), vec![vec![task]]);
    }

    #[test]
    fn model_queueing_with_an_argument_starts_the_task_with_slot_0x30() {
        let (mut e, this, task, _, out) = model_setup(None);
        let parent = block(&mut e, 0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_4350,
            &args![this, 0x7878u32, 0x99u32, out, 3u32, parent, 8u32, 1u8, 0u8, 1u8],
        );
        assert_eq!(calls_to(&e, SLOT_D), vec![vec![task, 0x99]]);
        assert!(calls_to(&e, SLOT_B).is_empty());
        assert_eq!(
            calls_to(&e, QUEUED_MODEL_FROM_NAME)[0][1..],
            [0x7878, 3, 8, 1, 0]
        );
        assert_eq!(e.mem.u32(out.addr()), task);
        // A loaded model is handled as by the name overload.
        let (mut e, this, task, _, out) = model_setup(Some(0x6060));
        e.call_log = Some(vec![]);
        e.call(
            0x0044_4350,
            &args![this, 0x7878u32, 0x99u32, out, 3u32, parent, 8u32, 0u8, 1u8, 0u8],
        );
        assert_eq!(calls_to(&e, SLOT_C), vec![vec![task]]);
        assert!(calls_to(&e, SLOT_D).is_empty());
    }

    #[test]
    fn model_queueing_with_an_argument_wrapper_gives_a_null_result_pointer() {
        let (mut e, this, task, _, _) = model_setup(None);
        let parent = block(&mut e, 0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_42c0,
            &args![this, 0x7878u32, 0x99u32, 3u32, parent, 8u32, 0u8, 0u8, 0u8],
        );
        assert_eq!(calls_to(&e, TASK_POINTER_CONSTRUCT)[0][1], 0);
        assert_eq!(calls_to(&e, SLOT_D), vec![vec![task, 0x99]]);
    }

    #[test]
    fn tree_model_queueing_builds_the_task_and_stores_it() {
        let mut e = qe();
        let task = q_task(&mut e, 5);
        q_constructor(&mut e, QUEUED_TREE_MODEL_CONSTRUCT, task);
        noop(&mut e, &[QUEUED_FILE_ADD_CHILD]);
        let this = e.new_object::<ModelLoader>();
        let parent = block(&mut e, 0x40);
        let out = block(&mut e, 4);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_45c0,
            &args![this, 0x1111u32, 0x2222u32, out, 4u32, parent, 6u32],
        );
        assert_eq!(
            calls_to(&e, QUEUED_TREE_MODEL_CONSTRUCT)[0][1..],
            [0x1111, 0x2222, 4, 6]
        );
        assert_eq!(e.mem.u32(task + 0x1c), parent.addr());
        assert_eq!(calls_to(&e, SLOT_B), vec![vec![task]]);
        assert_eq!(e.mem.u32(out.addr()), task);
        // The wrapper has a null result pointer of its own.
        e.mem.set_u32(out.addr(), 0);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_4540,
            &args![this, 0x1111u32, 0x2222u32, 4u32, parent, 6u32],
        );
        assert_eq!(calls_to(&e, TASK_POINTER_CONSTRUCT)[0][1], 0);
        assert_eq!(calls_to(&e, SLOT_B), vec![vec![task]]);
    }

    /// A loader whose KF model map (`+4`) finds `found`, and the doubles of
    /// the `QueuedKF` constructors.
    fn kf_setup(found: Option<u32>) -> (Engine, Ptr<ModelLoader>, u32, Ptr) {
        let mut e = qe();
        let task = q_task(&mut e, 5);
        q_constructor(&mut e, QUEUED_KF_FROM_NAME, task);
        q_constructor(&mut e, QUEUED_KF_FROM_MODEL, task);
        noop(&mut e, &[QUEUED_FILE_ADD_CHILD]);
        let map = q_map(&mut e, 2, found, true);
        let this = q_loader(&mut e, &[(0x4, map)]);
        let parent = block(&mut e, 0x40);
        (e, this, task, parent)
    }

    #[test]
    fn kf_queueing_builds_a_task_from_the_name_or_the_loaded_model() {
        let (mut e, this, task, parent) = kf_setup(None);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0044_46a0, &args![this, 0x7a7au32, 9u32, parent])
            .bool());
        assert_eq!(calls_to(&e, QUEUED_KF_FROM_NAME)[0][1..], [0x7a7a, 9]);
        assert_eq!(calls_to(&e, SLOT_B), vec![vec![task]]);
        assert_eq!(e.mem.u32(task + 0x1c), parent.addr());
        // A loaded KF model with a parent: a task from it, CheckFinished.
        let (mut e, this, task, parent) = kf_setup(Some(0x6161));
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0044_46a0, &args![this, 0x7a7au32, 9u32, parent])
            .bool());
        assert_eq!(calls_to(&e, QUEUED_KF_FROM_MODEL)[0][1..], [0x6161, 9]);
        assert_eq!(calls_to(&e, SLOT_C), vec![vec![task]]);
        assert!(calls_to(&e, QUEUED_KF_FROM_NAME).is_empty());
    }

    #[test]
    fn kf_queueing_without_a_parent_ignores_a_loaded_model() {
        let (mut e, this, _, _) = kf_setup(Some(0x6161));
        e.call_log = Some(vec![]);
        assert!(!e
            .call(
                0x0044_46a0,
                &args![this, 0x7a7au32, 9u32, Ptr::<()>::new(0)]
            )
            .bool());
        assert!(calls_to(&e, QUEUED_KF_FROM_MODEL).is_empty());
        assert!(calls_to(&e, SLOT_C).is_empty());
    }

    /// Everything `QueueReference` needs: a loader with the map of queued
    /// references (`+8`, numbered 3) and the second map (`+0xc`, numbered 4),
    /// a reference of form type `kind`, a cell, and every constructor
    /// answering the returned task.
    struct QueueReference {
        e: Engine,
        this: Ptr<ModelLoader>,
        reference: Ptr,
        task: u32,
        cell: u32,
    }

    fn queue_reference_setup(kind: u8, found: Option<u32>, add_result: bool) -> QueueReference {
        let mut e = qe();
        let task = q_task(&mut e, 5);
        for constructor in [
            REFERENCE_TASK_PLAYER,
            REFERENCE_TASK_TREE,
            REFERENCE_TASK_CREATURE,
            REFERENCE_TASK_ACTOR,
            REFERENCE_TASK_DEFAULT,
        ] {
            q_constructor(&mut e, constructor, task);
        }
        // The data handler: both flags clear.
        let handler = e.mem.alloc(0x250);
        e.set_global(DATA_HANDLER, handler);
        answer(&mut e, DATA_HANDLER_FLAG_BIT_2, 0);
        list_doubles(&mut e);
        let form = e.mem.alloc(0x20);
        e.mem.set_u8(form + 4, kind);
        let reference = reference(&mut e, &[], form);
        e.mem.set_u32(reference.addr() + 0xc8, 0);
        let cell = e.mem.alloc(0x100);
        answer(&mut e, REFERENCE_CELL, cell);
        answer(&mut e, REFERENCE_IS_3D_CRITICAL, 0);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.i32(a[0]) + 1;
            e.mem.set_i32(a[0], value);
            value.into_ret()
        });
        answer(&mut e, CURRENT_THREAD_ID, 7);
        answer(&mut e, OWNER_THREAD_ID, 8);
        answer(&mut e, IS_IN_MENU_MODE, 0);
        let queued = q_map(&mut e, 3, found, add_result);
        let later = q_map(&mut e, 4, None, true);
        let this = q_loader(&mut e, &[(0x8, queued), (0xc, later)]);
        QueueReference {
            e,
            this,
            reference,
            task,
            cell,
        }
    }

    fn run_queue_reference(q: &mut QueueReference, priority: i32, thread_only: u8) {
        q.e.call(
            0x0044_4850,
            &args![q.this, q.reference, priority, thread_only],
        );
    }

    #[test]
    fn queue_reference_queues_a_new_task_and_counts_it_in_the_cell() {
        let mut q = queue_reference_setup(0x10, None, true);
        q.e.call_log = Some(vec![]);
        run_queue_reference(&mut q, 4, 0);
        let e = &mut q.e;
        // A reference of another type gets the default task, inside the
        // memory context 0x31 (source line 0xc4e).
        assert_eq!(
            calls_to(e, REFERENCE_TASK_DEFAULT)[0][1..],
            [q.reference.addr(), 4]
        );
        let guards = calls_to(e, MEMORY_CONTEXT_ENTER);
        assert_eq!(guards.len(), 1);
        assert_eq!(guards[0][1..], [0x31, 1, MODEL_LOADER_SOURCE, 0xc4e]);
        assert_eq!(calls_to(e, MEMORY_CONTEXT_LEAVE).len(), 1);
        // It was added to the first map and started; the cell counts it.
        let adds = calls_to(e, add_double(3));
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][1], q.reference.addr());
        assert_eq!(adds[0][3], 0);
        assert_eq!(calls_to(e, SLOT_B), vec![vec![q.task]]);
        assert!(calls_to(e, add_double(4)).is_empty());
        assert_eq!(e.mem.i32(q.cell + 0xa4), 1);
        assert_eq!(e.mem.i32(q.cell + 0xa0), 0);
    }

    #[test]
    fn queue_reference_counts_a_critical_reference_twice() {
        let mut q = queue_reference_setup(0x10, None, true);
        answer(&mut q.e, REFERENCE_IS_3D_CRITICAL, 1);
        run_queue_reference(&mut q, 4, 0);
        assert_eq!(q.e.mem.i32(q.cell + 0xa4), 1);
        assert_eq!(q.e.mem.i32(q.cell + 0xa0), 1);
        // Without a parent cell nothing is counted.
        let mut q = queue_reference_setup(0x10, None, true);
        answer(&mut q.e, REFERENCE_CELL, 0);
        q.e.call_log = Some(vec![]);
        run_queue_reference(&mut q, 4, 0);
        assert!(calls_to(&q.e, INTERLOCKED_INCREMENT).is_empty());
        assert_eq!(calls_to(&q.e, SLOT_B).len(), 1);
    }

    #[test]
    fn queue_reference_picks_the_task_by_the_type_of_the_base_form() {
        for (kind, constructor, context) in [
            (0x25u8, REFERENCE_TASK_TREE, None),
            (0x2a, REFERENCE_TASK_CREATURE, Some((0x32u32, 0xc6bu32))),
            (0x2b, REFERENCE_TASK_ACTOR, Some((0x32, 0xc72))),
            (0x01, REFERENCE_TASK_DEFAULT, None),
        ] {
            let mut q = queue_reference_setup(kind, None, true);
            q.e.call_log = Some(vec![]);
            run_queue_reference(&mut q, 4, 0);
            let built = calls_to(&q.e, constructor);
            assert_eq!(built.len(), 1, "type {kind:#x}");
            assert_eq!(built[0][1..], [q.reference.addr(), 4]);
            let guards = order_of(&q.e, &[MEMORY_CONTEXT_ENTER]);
            assert_eq!(guards.len(), if context.is_some() { 2 } else { 1 });
            if let Some((context, line)) = context {
                let entered = calls_to(&q.e, MEMORY_CONTEXT_ENTER);
                assert_eq!(entered[1][1..], [context, 1, MODEL_LOADER_SOURCE, line]);
            }
        }
    }

    #[test]
    fn queue_reference_gives_the_player_its_own_task() {
        let mut q = queue_reference_setup(0x10, None, true);
        q.e.set_global(SPECIAL_REFERENCE, q.reference.addr());
        q.e.call_log = Some(vec![]);
        run_queue_reference(&mut q, 4, 0);
        assert_eq!(calls_to(&q.e, REFERENCE_TASK_PLAYER)[0][1..], [4]);
        let entered = calls_to(&q.e, MEMORY_CONTEXT_ENTER);
        assert_eq!(entered.len(), 2);
        assert_eq!(entered[1][1..], [0x34, 1, MODEL_LOADER_SOURCE, 0xc53]);
        assert!(calls_to(&q.e, REFERENCE_TASK_DEFAULT).is_empty());
        assert_eq!(calls_to(&q.e, SLOT_B), vec![vec![q.task]]);
    }

    #[test]
    fn queue_reference_requeues_a_known_task_only_when_the_priority_differs() {
        let mut q = queue_reference_setup(0x10, Some(0), true);
        let task = q_task(&mut q.e, 5);
        q.e.register_double(find_double(3), move |e, a| {
            e.mem.set_u32(a[2], task);
            1u32.into_ret()
        });
        q.e.call_log = Some(vec![]);
        run_queue_reference(&mut q, 2, 0);
        assert_eq!(calls_to(&q.e, SLOT_A), vec![vec![task, 2]]);
        assert!(calls_to(&q.e, REFERENCE_TASK_DEFAULT).is_empty());
        assert!(calls_to(&q.e, MEMORY_CONTEXT_ENTER).is_empty());
        // The same priority leaves it alone.
        q.e.call_log = Some(vec![]);
        run_queue_reference(&mut q, 5, 0);
        assert!(calls_to(&q.e, SLOT_A).is_empty());
    }

    #[test]
    fn queue_reference_clears_the_task_when_the_map_refuses_it() {
        let mut q = queue_reference_setup(0x10, None, false);
        q.e.call_log = Some(vec![]);
        run_queue_reference(&mut q, 4, 0);
        assert!(calls_to(&q.e, SLOT_B).is_empty());
        assert!(calls_to(&q.e, INTERLOCKED_INCREMENT).is_empty());
        let assigned = calls_to(&q.e, TASK_POINTER_ASSIGN);
        assert_eq!(assigned.last().unwrap()[1], 0);
    }

    #[test]
    fn queue_reference_defers_to_the_second_map_on_the_owner_thread() {
        let mut q = queue_reference_setup(0x10, None, true);
        answer(&mut q.e, OWNER_THREAD_ID, 7);
        q.e.call_log = Some(vec![]);
        run_queue_reference(&mut q, 4, 1);
        let later = calls_to(&q.e, add_double(4));
        assert_eq!(later.len(), 1);
        assert_eq!(later[0][1], q.reference.addr());
        assert_eq!(later[0][3], 1);
        assert!(calls_to(&q.e, SLOT_B).is_empty());
        // The cell still counts it.
        assert_eq!(q.e.mem.i32(q.cell + 0xa4), 1);
        // In menu mode, or without the request, the task is started.
        for (menu, request) in [(1u32, 1u8), (0, 0)] {
            let mut q = queue_reference_setup(0x10, None, true);
            answer(&mut q.e, OWNER_THREAD_ID, 7);
            answer(&mut q.e, IS_IN_MENU_MODE, menu);
            q.e.call_log = Some(vec![]);
            run_queue_reference(&mut q, 4, request);
            assert!(calls_to(&q.e, add_double(4)).is_empty());
            assert_eq!(calls_to(&q.e, SLOT_B).len(), 1);
        }
    }

    #[test]
    fn queue_reference_does_nothing_for_the_excluded_references() {
        // The data handler's bit 2 is set and bit 8 is clear.
        let mut q = queue_reference_setup(0x10, None, true);
        answer(&mut q.e, DATA_HANDLER_FLAG_BIT_2, 1);
        q.e.call_log = Some(vec![]);
        run_queue_reference(&mut q, 4, 0);
        assert!(calls_to(&q.e, find_double(3)).is_empty());
        // With bit 8 set as well the reference goes on.
        let handler = q.e.global::<u32>(DATA_HANDLER);
        q.e.mem.set_u32(handler + 0x244, 8);
        run_queue_reference(&mut q, 4, 0);
        assert_eq!(calls_to(&q.e, find_double(3)).len(), 1);
        // A base form of type 0xd.
        let mut q = queue_reference_setup(0xd, None, true);
        q.e.call_log = Some(vec![]);
        run_queue_reference(&mut q, 4, 0);
        assert!(calls_to(&q.e, find_double(3)).is_empty());
        // A reference whose slot 0x224 says yes and whose flag 2 is set.
        let mut q = queue_reference_setup(0x10, None, true);
        vtable(&mut q.e, REFERENCE_VTABLE, &[(0x224, YES)]);
        q.e.mem.set_u32(q.reference.addr() + 0xc8, 2);
        q.e.call_log = Some(vec![]);
        run_queue_reference(&mut q, 4, 0);
        assert!(calls_to(&q.e, find_double(3)).is_empty());
        // Without the flag it is queued.
        q.e.mem.set_u32(q.reference.addr() + 0xc8, 4);
        run_queue_reference(&mut q, 4, 0);
        assert_eq!(calls_to(&q.e, find_double(3)).len(), 1);
    }

    #[test]
    fn cell_queued_counts_are_interlocked_increments() {
        let mut e = qe();
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.i32(a[0]) + 1;
            e.mem.set_i32(a[0], value);
            value.into_ret()
        });
        let cell = block(&mut e, 0x100);
        e.call(0x0044_4cc0, &args![cell]);
        e.call(0x0044_4cc0, &args![cell]);
        e.call(0x0044_4ce0, &args![cell]);
        assert_eq!(e.mem.i32(cell.addr() + 0xa4), 2);
        assert_eq!(e.mem.i32(cell.addr() + 0xa0), 1);
    }

    #[test]
    fn flag_tests_read_the_dwords_at_0xc8_and_0x244() {
        let mut e = qe();
        let object = block(&mut e, 0x250);
        e.mem.set_u32(object.addr() + 0xc8, 6);
        assert!(e.call(0x0044_4d00, &args![object, 2u32]).bool());
        assert!(e.call(0x0044_4d00, &args![object, 7u32]).bool());
        assert!(!e.call(0x0044_4d00, &args![object, 9u32]).bool());
        e.mem.set_u32(object.addr() + 0x244, 7);
        assert!(!e.call(0x0044_4d20, &args![object]).bool());
        e.mem.set_u32(object.addr() + 0x244, 0xf);
        assert!(e.call(0x0044_4d20, &args![object]).bool());
    }

    /// The model queueing through `00444dc0`: the loader of the global has a
    /// model map that finds nothing; `LOADER_GET_TES_MODEL` answers a
    /// `TESModel`; `GET_LOD_MULT` answers 9.
    fn form_model_setup() -> (Engine, Ptr<ModelLoader>, u32, Ptr, Ptr) {
        let (mut e, this, task, tes_model, out) = model_setup(None);
        let global_loader: Ptr = Ptr::new(e.global::<u32>(MODEL_LOADER));
        let map = e.get(this, ModelLoader::pModelMap);
        e.mem.set_u32(global_loader.addr(), map.addr());
        answer(&mut e, LOADER_GET_TES_MODEL, tes_model.addr());
        answer(&mut e, GET_LOD_MULT, 9);
        answer(&mut e, REFERENCE_TEST_00564E60, 0);
        answer(&mut e, REFERENCE_TEST_00564F00, 0);
        e.register(REFERENCE_PROCESS_SLOT, |_, a| (a[0] + 0x20).into_ret());
        (e, this, task, tes_model, out)
    }

    #[test]
    fn form_model_queueing_without_a_reference_uses_the_lod_of_the_form() {
        let (mut e, this, task, tes_model, out) = form_model_setup();
        let parent = block(&mut e, 0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_4dc0,
            &args![this, 0x3333u32, out, 4u32, parent, Ptr::<()>::new(0)],
        );
        assert_eq!(
            calls_to(&e, LOADER_GET_TES_MODEL),
            vec![vec![this.addr(), 0x3333, 0]]
        );
        assert_eq!(calls_to(&e, GET_LOD_MULT), vec![vec![0x3333]]);
        // Flags 1, 0, 0 and the LOD multiplier of the form.
        assert_eq!(
            calls_to(&e, QUEUED_MODEL_FROM_TES_MODEL)[0][1..],
            [tes_model.addr(), 4, 9, 1, 0]
        );
        assert_eq!(
            e.get(
                Ptr::<QueuedModel>::new(task),
                QueuedModel::mfOverriddenVisualDistance
            ),
            0.0
        );
        assert_eq!(e.mem.u32(out.addr()), task);
    }

    #[test]
    fn form_model_queueing_adjusts_lod_and_distance_for_some_references() {
        let (mut e, this, task, _, out) = form_model_setup();
        e.set_global(VISUAL_DISTANCE_LIMIT, 3.0e38f32);
        let reference = block(&mut e, 0x100);
        let parent = block(&mut e, 0x40);
        // 00564e60 accepts the reference: multiplier 6; 00564f00: distance.
        answer(&mut e, REFERENCE_TEST_00564E60, 1);
        answer(&mut e, REFERENCE_TEST_00564F00, 1);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_4dc0,
            &args![this, 0x3333u32, out, 4u32, parent, reference],
        );
        assert_eq!(calls_to(&e, QUEUED_MODEL_FROM_TES_MODEL)[0][3], 6);
        assert_eq!(
            e.get(
                Ptr::<QueuedModel>::new(task),
                QueuedModel::mfOverriddenVisualDistance
            ),
            3.0e38
        );
        // Neither: the multiplier of the form, no distance.
        let (mut e, this, task, _, out) = form_model_setup();
        let reference = block(&mut e, 0x100);
        let parent = block(&mut e, 0x40);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_4dc0,
            &args![this, 0x3333u32, out, 4u32, parent, reference],
        );
        assert_eq!(calls_to(&e, QUEUED_MODEL_FROM_TES_MODEL)[0][3], 9);
        assert_eq!(
            e.get(
                Ptr::<QueuedModel>::new(task),
                QueuedModel::mfOverriddenVisualDistance
            ),
            0.0
        );
    }

    #[test]
    fn form_model_queueing_uses_the_reference_process_for_the_lod() {
        let (mut e, this, _, _, out) = form_model_setup();
        let reference = block(&mut e, 0x100);
        let parent = block(&mut e, 0x40);
        // 00891170 gives the address of a pointer to an object whose slot
        // 0x154 answers yes.
        vtable(&mut e, POINTER_VTABLE, &[(0x154, YES)]);
        let process = object(&mut e, POINTER_VTABLE, 0x20);
        e.mem.set_u32(reference.addr() + 0x20, process);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_4dc0,
            &args![this, 0x3333u32, out, 4u32, parent, reference],
        );
        assert_eq!(calls_to(&e, QUEUED_MODEL_FROM_TES_MODEL)[0][3], 6);
        // The wrapper has a null result pointer of its own.
        e.call_log = Some(vec![]);
        e.call(
            0x0044_4d40,
            &args![this, 0x3333u32, 4u32, parent, reference],
        );
        assert_eq!(calls_to(&e, TASK_POINTER_CONSTRUCT)[0][1], 0);
    }

    #[test]
    fn process_slot_test_reads_the_virtual_function_when_there_is_an_object() {
        let mut e = qe();
        e.register(REFERENCE_PROCESS_SLOT, |_, a| (a[0] + 0x20).into_ret());
        let reference = block(&mut e, 0x100);
        // No object.
        assert!(!e.call(0x0044_4ed0, &args![reference]).bool());
        vtable(&mut e, POINTER_VTABLE, &[(0x154, YES)]);
        let process = object(&mut e, POINTER_VTABLE, 0x20);
        e.mem.set_u32(reference.addr() + 0x20, process);
        assert!(e.call(0x0044_4ed0, &args![reference]).bool());
        vtable(&mut e, POINTER_VTABLE, &[(0x154, NO)]);
        assert!(!e.call(0x0044_4ed0, &args![reference]).bool());
    }

    #[test]
    fn task_0043db30_is_queued_only_with_both_arguments() {
        let mut e = qe();
        let task = q_task(&mut e, 5);
        q_constructor(&mut e, TASK_CONSTRUCT_0043DB30, task);
        let this = e.new_object::<ModelLoader>();
        e.call_log = Some(vec![]);
        e.call(0x0044_4f10, &args![this, 0u32, 2u32, 3u32]);
        e.call(0x0044_4f10, &args![this, 1u32, 0u32, 3u32]);
        assert!(calls_to(&e, TASK_CONSTRUCT_0043DB30).is_empty());
        e.call(0x0044_4f10, &args![this, 1u32, 2u32, 3u32]);
        assert_eq!(calls_to(&e, TASK_CONSTRUCT_0043DB30)[0][1..], [1, 2, 3]);
        assert_eq!(calls_to(&e, SLOT_B), vec![vec![task]]);
    }

    #[test]
    fn task_0043eaf0_gets_its_parent_and_is_stored() {
        let mut e = qe();
        let task = q_task(&mut e, 5);
        q_constructor(&mut e, TASK_CONSTRUCT_0043EAF0, task);
        noop(&mut e, &[QUEUED_FILE_ADD_CHILD]);
        let this = e.new_object::<ModelLoader>();
        let out = block(&mut e, 4);
        let parent = block(&mut e, 0x40);
        e.call_log = Some(vec![]);
        e.call(0x0044_4fe0, &args![this, 0x11u32, out, 0x33u32, parent]);
        assert_eq!(calls_to(&e, TASK_CONSTRUCT_0043EAF0)[0][1..], [0x11, 0x33]);
        assert_eq!(e.mem.u32(task + 0x1c), parent.addr());
        assert_eq!(calls_to(&e, SLOT_B), vec![vec![task]]);
        assert_eq!(e.mem.u32(out.addr()), task);
    }

    /// A loader whose helmet map (`+0x18`, number 5) finds `found`, with the
    /// helmet constructor and key getter as doubles.
    fn helmet_setup(found: Option<u32>, add_result: bool) -> (Engine, Ptr<ModelLoader>, u32) {
        let mut e = qe();
        let task = q_task(&mut e, 5);
        q_constructor(&mut e, QUEUED_HELMET_CONSTRUCT, task);
        answer(&mut e, REFERENCE_HELMET_KEY, 0x7777);
        noop(&mut e, &[QUEUED_FILE_ADD_CHILD]);
        let map = q_map(&mut e, 5, found, add_result);
        let this = q_loader(&mut e, &[(0x18, map)]);
        (e, this, task)
    }

    #[test]
    fn helmet_queueing_adds_a_new_task_to_the_map_and_starts_it() {
        let (mut e, this, task) = helmet_setup(None, true);
        let out = block(&mut e, 4);
        let parent = block(&mut e, 0x40);
        let reference = block(&mut e, 0x100);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_50c0,
            &args![this, reference, out, 0x55u32, parent, 0x66u32],
        );
        assert_eq!(
            calls_to(&e, QUEUED_HELMET_CONSTRUCT)[0][1..],
            [reference.addr(), 0x55, 0x66]
        );
        assert_eq!(calls_to(&e, REFERENCE_HELMET_KEY).len(), 2);
        let adds = calls_to(&e, add_double(5));
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][1], 0x7777);
        assert_eq!(adds[0][3], 0);
        assert_eq!(e.mem.u32(task + 0x1c), parent.addr());
        assert_eq!(calls_to(&e, SLOT_B), vec![vec![task]]);
        assert_eq!(e.mem.u32(out.addr()), task);
    }

    #[test]
    fn helmet_queueing_clears_a_refused_task_and_keeps_a_known_one() {
        let (mut e, this, _) = helmet_setup(None, false);
        let out = block(&mut e, 4);
        let reference = block(&mut e, 0x100);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_50c0,
            &args![this, reference, out, 0x55u32, Ptr::<()>::new(0), 0x66u32],
        );
        assert!(calls_to(&e, SLOT_B).is_empty());
        assert_eq!(e.mem.u32(out.addr()), 0);
        // The map already has a task: it is only handed to the result.
        let (mut e, this, _) = helmet_setup(Some(0x4242), true);
        let out = block(&mut e, 4);
        let reference = block(&mut e, 0x100);
        e.call_log = Some(vec![]);
        e.call(
            0x0044_50c0,
            &args![this, reference, out, 0x55u32, Ptr::<()>::new(0), 0x66u32],
        );
        assert!(calls_to(&e, QUEUED_HELMET_CONSTRUCT).is_empty());
        assert!(calls_to(&e, add_double(5)).is_empty());
        assert_eq!(e.mem.u32(out.addr()), 0x4242);
    }

    #[test]
    fn animation_idle_queueing_adds_the_task_to_the_idle_map_and_starts_it() {
        for accepted in [true, false] {
            let mut e = qe();
            let task = q_task(&mut e, 5);
            q_constructor(&mut e, TASK_CONSTRUCT_0043E580, task);
            let map = q_map(&mut e, 6, None, accepted);
            let this = q_loader(&mut e, &[(0x10, map)]);
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_5200, &args![this, 1u32, 2u32, 3u32, 4u32]);
            assert_eq!(result.u32(), 0);
            // The constructor takes (first, third, second, fourth).
            assert_eq!(calls_to(&e, TASK_CONSTRUCT_0043E580)[0][1..], [1, 3, 2, 4]);
            let adds = calls_to(&e, add_double(6));
            assert_eq!(adds[0][1], 2);
            assert_eq!(adds[0][3], 0);
            assert_eq!(calls_to(&e, SLOT_B).len(), accepted as usize);
        }
    }

    #[test]
    fn release_model_lowers_the_count_and_may_remove_the_model() {
        let mut e = qe();
        noop(
            &mut e,
            &[
                MODEL_RELEASE_ONE,
                MODEL_MOD_MANUAL_REF_COUNT,
                TRY_AND_REMOVE_MODEL,
            ],
        );
        let map = q_map(&mut e, 7, Some(0x6400), true);
        let this = q_loader(&mut e, &[(0x0, map)]);
        e.call_log = Some(vec![]);
        e.call(0x0044_5300, &args![this, 0x7878u32, 0u8, 1i32]);
        assert_eq!(calls_to(&e, MODEL_RELEASE_ONE), vec![vec![0x6400]]);
        assert!(calls_to(&e, MODEL_MOD_MANUAL_REF_COUNT).is_empty());
        assert!(calls_to(&e, TRY_AND_REMOVE_MODEL).is_empty());
        // Another count lowers the manual count by its low 16 bits (signed).
        e.call(0x0044_5300, &args![this, 0x7878u32, 1u8, 3i32]);
        e.call(0x0044_5300, &args![this, 0x7878u32, 0u8, 131_071i32]);
        assert_eq!(
            calls_to(&e, MODEL_MOD_MANUAL_REF_COUNT),
            vec![vec![0x6400, (-3i32) as u32], vec![0x6400, 1]]
        );
        assert_eq!(
            calls_to(&e, TRY_AND_REMOVE_MODEL),
            vec![vec![this.addr(), 0x6400, 0x7878]]
        );
    }

    #[test]
    fn release_model_ignores_unknown_models() {
        let mut e = qe();
        let map = q_map(&mut e, 7, None, true);
        let this = q_loader(&mut e, &[(0x0, map)]);
        e.call_log = Some(vec![]);
        e.call(0x0044_5300, &args![this, 0x7878u32, 1u8, 1i32]);
        assert_eq!(calls_to(&e, find_double(7)).len(), 1);
    }

    /// A loader whose KF model map (`+4`, number 8) finds a KF model with the
    /// given counts; returns the loader and the model.
    fn kf_release_setup(references: i32, manual: i32) -> (Engine, Ptr<ModelLoader>, Ptr<KFModel>) {
        let mut e = qe();
        noop(&mut e, &[KF_MODEL_PREPARE, KF_MODEL_DESTRUCTOR_BODY]);
        let model = e.new_object::<KFModel>();
        e.set(model, KFModel::iRefCount, references);
        e.set(model, KFModel::iManualRefCount, manual);
        let map = q_map(&mut e, 8, Some(model.addr()), true);
        let this = q_loader(&mut e, &[(0x4, map)]);
        answer(&mut e, DATA_HANDLER_FLAG_BIT_2, 0);
        (e, this, model)
    }

    #[test]
    fn kf_release_removes_and_deletes_an_unreferenced_model() {
        let (mut e, this, model) = kf_release_setup(0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_5370, &args![this, 0x7a7au32, 1u8]);
        assert_eq!(calls_to(&e, KF_MODEL_PREPARE), vec![vec![model.addr()]]);
        let map = e.get(this, ModelLoader::pKFModelMap);
        assert_eq!(
            calls_to(&e, remove_double(8)),
            vec![vec![map.addr(), 0x7a7a]]
        );
        assert_eq!(
            calls_to(&e, KF_MODEL_DESTRUCTOR_BODY),
            vec![vec![model.addr()]]
        );
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![model.addr()]]);
    }

    #[test]
    fn kf_release_keeps_referenced_models_and_removal_off() {
        // A model that is still referenced.
        let (mut e, this, _) = kf_release_setup(1, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_5370, &args![this, 0x7a7au32, 1u8]);
        assert_eq!(calls_to(&e, KF_MODEL_PREPARE).len(), 1);
        assert!(calls_to(&e, remove_double(8)).is_empty());
        // Removal off.
        let (mut e, this, _) = kf_release_setup(0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_5370, &args![this, 0x7a7au32, 0u8]);
        assert!(calls_to(&e, remove_double(8)).is_empty());
        assert_eq!(e.get(this, ModelLoader::bHasDelayedFree), 0);
    }

    #[test]
    fn kf_release_delays_the_free_while_the_data_handler_flag_is_set() {
        let (mut e, this, _) = kf_release_setup(0, 0);
        let handler = e.mem.alloc(0x250);
        e.set_global(DATA_HANDLER, handler);
        answer(&mut e, DATA_HANDLER_FLAG_BIT_2, 1);
        e.call_log = Some(vec![]);
        e.call(0x0044_5370, &args![this, 0x7a7au32, 1u8]);
        assert_eq!(e.get(this, ModelLoader::bHasDelayedFree), 1);
        assert!(calls_to(&e, remove_double(8)).is_empty());
        assert!(calls_to(&e, KF_MODEL_DESTRUCTOR_BODY).is_empty());
        // The flag is only tested when the handler exists.
        let (mut e, this, _) = kf_release_setup(0, 0);
        e.set_global(DATA_HANDLER, 0u32);
        answer(&mut e, DATA_HANDLER_FLAG_BIT_2, 1);
        e.call_log = Some(vec![]);
        e.call(0x0044_5370, &args![this, 0x7a7au32, 1u8]);
        assert!(calls_to(&e, DATA_HANDLER_FLAG_BIT_2).is_empty());
        assert_eq!(calls_to(&e, remove_double(8)).len(), 1);
    }

    #[test]
    fn kf_release_ignores_unknown_models() {
        let mut e = qe();
        let map = q_map(&mut e, 8, None, true);
        let this = q_loader(&mut e, &[(0x4, map)]);
        e.call_log = Some(vec![]);
        e.call(0x0044_5370, &args![this, 0x7a7au32, 1u8]);
        assert_eq!(calls_to(&e, KF_MODEL_PREPARE).len(), 0);
    }

    #[test]
    fn cancel_of_a_list_or_idle_cancels_the_task_of_its_map() {
        for (function, offset) in [(0x0044_5430u32, 0x14u32), (0x0044_54d0, 0x10)] {
            let mut e = qe();
            e.register(TASK_MANAGER_CANCEL_TASK, |_, _| Ret::default());
            let map = q_map(&mut e, 9, Some(0x4343), true);
            let this = q_loader(&mut e, &[(offset, map)]);
            e.call_log = Some(vec![]);
            e.call(function, &args![this, 0x1234u32]);
            assert_eq!(calls_to(&e, find_double(9))[0][1], 0x1234);
            let manager = e.global::<u32>(TASK_QUEUE);
            assert_eq!(
                calls_to(&e, TASK_MANAGER_CANCEL_TASK),
                vec![vec![manager, 0x4343, 0]]
            );
            // An unknown key cancels nothing.
            let map = q_map(&mut e, 9, None, true);
            e.mem.set_u32(this.addr() + offset, map.addr());
            e.call_log = Some(vec![]);
            e.call(function, &args![this, 0x1234u32]);
            assert!(calls_to(&e, TASK_MANAGER_CANCEL_TASK).is_empty());
        }
    }

    #[test]
    fn cancel_reference_cancels_the_queued_task_and_the_helmet() {
        let mut e = qe();
        e.register(TASK_MANAGER_CANCEL_TASK, |_, _| Ret::default());
        let queued = q_map(&mut e, 10, Some(0x4343), true);
        let later = q_map(&mut e, 11, None, true);
        let helmets = q_map(&mut e, 12, Some(0x4545), true);
        let this = q_loader(&mut e, &[(0x8, queued), (0xc, later), (0x18, helmets)]);
        let reference = block(&mut e, 0x100);
        e.call_log = Some(vec![]);
        e.call(0x0044_5570, &args![this, reference]);
        let manager = e.global::<u32>(TASK_QUEUE);
        assert_eq!(
            calls_to(&e, TASK_MANAGER_CANCEL_TASK),
            vec![vec![manager, 0x4343, 0], vec![manager, 0x4545, 0]]
        );
        assert_eq!(
            calls_to(&e, remove_double(11)),
            vec![vec![later.addr(), reference.addr()]]
        );
        // Nothing known: nothing is cancelled or removed.
        let queued = q_map(&mut e, 10, None, true);
        let helmets = q_map(&mut e, 12, None, true);
        e.mem.set_u32(this.addr() + 0x8, queued.addr());
        e.mem.set_u32(this.addr() + 0x18, helmets.addr());
        e.call_log = Some(vec![]);
        e.call(0x0044_5570, &args![this, reference]);
        assert!(calls_to(&e, TASK_MANAGER_CANCEL_TASK).is_empty());
        assert!(calls_to(&e, remove_double(11)).is_empty());
    }

    #[test]
    fn cancel_references_for_cell_cancels_the_references_of_that_cell_only() {
        let mut e = qe();
        e.register(TASK_MANAGER_CANCEL_TASK, |_, _| Ret::default());
        noop(
            &mut e,
            &[REFERENCE_ITERATOR_CONSTRUCT, REFERENCE_ITERATOR_DESTRUCT],
        );
        let cell = block(&mut e, 0x100);
        let other = block(&mut e, 0x100);
        answer(&mut e, CELL_QUEUED_COUNT_GETTER, 3);
        let inside = block(&mut e, 0x100);
        let outside = block(&mut e, 0x100);
        e.mem.set_u32(inside.addr() + 0x40, cell.addr());
        e.mem.set_u32(outside.addr() + 0x40, other.addr());
        e.register(REFERENCE_CELL, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        // The walk gives the two references, then ends.
        let references = [inside.addr(), outside.addr()];
        let given = std::rc::Rc::new(std::cell::Cell::new(0usize));
        let ended = given.clone();
        e.register_double(ITERATOR_AT_END, move |_, _| {
            ((ended.get() >= references.len()) as u32).into_ret()
        });
        e.register_double(REFERENCE_MAP_NEXT, move |e, a| {
            e.mem.set_u32(a[2], references[given.get()]);
            given.set(given.get() + 1);
            1u32.into_ret()
        });
        let queued = q_map(&mut e, 13, Some(0x4343), true);
        let later = q_map(&mut e, 14, None, true);
        let helmets = q_map(&mut e, 15, None, true);
        let this = q_loader(&mut e, &[(0x8, queued), (0xc, later), (0x18, helmets)]);
        e.call_log = Some(vec![]);
        e.call(0x0044_5670, &args![this, cell]);
        // Only the reference of the cell was cancelled.
        let removed = calls_to(&e, remove_double(14));
        assert_eq!(removed, vec![vec![later.addr(), inside.addr()]]);
        assert_eq!(calls_to(&e, REFERENCE_ITERATOR_DESTRUCT).len(), 1);
    }

    #[test]
    fn cancel_references_for_cell_needs_a_cell_with_queued_references() {
        let mut e = qe();
        noop(
            &mut e,
            &[REFERENCE_ITERATOR_CONSTRUCT, REFERENCE_ITERATOR_DESTRUCT],
        );
        let this = e.new_object::<ModelLoader>();
        let cell = block(&mut e, 0x100);
        answer(&mut e, CELL_QUEUED_COUNT_GETTER, 0);
        e.call_log = Some(vec![]);
        e.call(0x0044_5670, &args![this, Ptr::<()>::new(0)]);
        e.call(0x0044_5670, &args![this, cell]);
        assert!(calls_to(&e, REFERENCE_ITERATOR_CONSTRUCT).is_empty());
        assert_eq!(calls_to(&e, CELL_QUEUED_COUNT_GETTER).len(), 1);
    }

    #[test]
    fn queued_reference_test_asks_the_reference_map() {
        let mut e = qe();
        let map = q_map(&mut e, 16, Some(0x4343), true);
        let this = q_loader(&mut e, &[(0x8, map)]);
        let reference = block(&mut e, 0x100);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0044_5750, &args![this, reference]).bool());
        assert_eq!(
            calls_to(&e, find_double(16))[0][..2],
            [map.addr(), reference.addr()]
        );
        let map = q_map(&mut e, 16, None, true);
        e.mem.set_u32(this.addr() + 0x8, map.addr());
        assert!(!e.call(0x0044_5750, &args![this, reference]).bool());
    }
}

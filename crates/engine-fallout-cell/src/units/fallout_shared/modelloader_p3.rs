//! `fallout shared/modelloader.cpp` (Xbox PDB source unit), part 3: its functions from `004457d0` up to
//! (not including) `00449c00` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::modelloader`]; anything public there may be used here.
//!
//! State of this file: the whole range, `004457d0` to `00449bd0` (120 functions).
//! First session: `QueueCreatureParts`, `QueueAnimations` and the list
//! helpers they use, the replacement KF list, the loading-status text, the
//! clone thread getters, the map inserts, `LoadFile`, `LoadKF`, `FindModel`
//! and `BuildFileList` (to `00447300`). Second session: `BuildKFFileList`,
//! `CopyFilenameList`, `QueueFaceGenFile`, `QueueEGMFile`, `QueueTRIFile`,
//! the loader's map forwarders, the cancel-all and clean-up loops, the
//! add-on loading (`LoadAddonNodes`, `LoadAddons`), the timed task walk and
//! its timer, and the task state helpers (`00447330` to `004491c0`). Third
//! session: the map wrappers, name helpers and vtable setters (`004491f0` to
//! `00449bd0`); the range is complete. The previously planned next
//! function was `004491f0`.
//!
//! The game's `char *` lists passed around here are `BSSimpleList<char *>`
//! heads: the head node holds the first item inline (item at +0, next node
//! at +4); `006815c0` returns the node's item address and `00726070` the
//! next node. `0063f7b0` removes the head (the next node's contents move
//! into it and the next node is deleted); `004702f0(list, 1)` deletes a
//! list.
//!
//! Not translated: the compiler's exception-unwinding frames (`FS:[0]`
//! chains and state variables) and the stack-cookie checks. Locals the game
//! keeps on its stack and passes by address are `with_stack` blocks.
//!
//! Callees in other files of the unit are called by address.

#[allow(unused_imports)]
use super::modelloader::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// `MemoryManager` allocation, `__cdecl(size) -> block` (`00401000`).
const MEMORY_ALLOC: u32 = 0x0040_1000;
/// `MemoryManager` deallocation, `__cdecl(block)` (`00401030`).
const MEMORY_FREE: u32 = 0x0040_1030;
/// `strcpy_s(destination, size, source)` through the game's wrapper
/// (`00406d30`).
const STRING_COPY: u32 = 0x0040_6d30;
/// `strcat_s(destination, size, source)` through the game's wrapper
/// (`00406d50`).
const STRING_CONCAT: u32 = 0x0040_6d50;
/// `sprintf_s(buffer, size, format, ...)` through the game's wrapper
/// (`00406d00`).
const FORMAT_BUFFER: u32 = 0x0040_6d00;
/// `strrchr(text, character)` (`0040ab30`).
const STRRCHR: u32 = 0x0040_ab30;
/// `_strnicmp(first, second, count)` (`00ec7ec0`).
const STRNICMP: u32 = 0x00ec_7ec0;
/// `sprintf(buffer, format, ...)` (`00ec623a`).
const SPRINTF: u32 = 0x00ec_623a;
/// `strlen` through the game's wrapper (`0044a670`).
const STRLEN: u32 = 0x0044_a670;
/// `__RTDynamicCast(object, offset, source type, target type, reference)`
/// (`00ec43fb`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The game's `printf`-style log (`005b5e40`), `__cdecl(format, ...)`.
const LOG: u32 = 0x005b_5e40;

/// List node accessors: the item address of a node (`006815c0`, the node
/// itself) and the next node (`00726070`, the dword at +4).
const LIST_NODE_ITEM_ADDRESS: u32 = 0x0068_15c0;
const LIST_NEXT_NODE: u32 = 0x0072_6070;
/// Removes the head of a list (`0063f7b0`).
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// Deletes a list, `__thiscall(list, flags)` (`004702f0`).
const LIST_DELETE: u32 = 0x0047_02f0;
/// Constructs an empty list in a block of 8 bytes (`0096a2d0`; the engine
/// map files it under another unit).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// Appends the item whose address is passed (`005ae3d0`).
const LIST_PUSH_BACK: u32 = 0x005a_e3d0;
/// The dword at +8 of an object (`0044ddc0`).
const FIELD_AT_8: u32 = 0x0044_ddc0;
/// `NiPointer` read (`00559450`): the pointer stored at the address.
const POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer<QueuedFile>::operator_` (Xbox PDB, `006f74f0`),
/// `__thiscall(object)`.
const QUEUED_FILE_POINTER_ASSIGN: u32 = 0x006f_74f0;
/// Smart pointer to a task: constructor with a reference (`00528cb0`) and
/// destructor (`0044cbf0`).
const TASK_POINTER_CONSTRUCT: u32 = 0x0052_8cb0;
const TASK_POINTER_DESTRUCT: u32 = 0x0044_cbf0;
/// `NiPointer<Model>::operator=(Model*)` (`0044aed0`).
const MODEL_POINTER_ASSIGN: u32 = 0x0044_aed0;
/// The scope guard that sets the memory context for the rest of a function:
/// `__thiscall(context, 1, source file, line)` and its destructor.
const MEMORY_CONTEXT_ENTER: u32 = 0x0040_4eb0;
const MEMORY_CONTEXT_LEAVE: u32 = 0x0040_4ee0;
/// `QueuedFile::QueuedFile(context)` (Xbox PDB, `00c3c590`).
const QUEUED_FILE_CONSTRUCT: u32 = 0x00c3_c590;
/// `QueuedFile::CheckFinished` (Xbox PDB, `00c3c930`).
const QUEUED_FILE_CHECK_FINISHED: u32 = 0x00c3_c930;
/// The `QueuedFile` destructor body the queued classes share (`0043dc50`).
const QUEUED_FILE_DESTRUCT: u32 = 0x0043_dc50;
/// `QueuedFile` method (`00443aa0`, no name in the engine map),
/// `__thiscall(parent)`: stores `spParent` and tells the parent.
const QUEUED_FILE_SET_PARENT: u32 = 0x0044_3aa0;
/// Puts a queued task into state 5 (`00449150`: `this + 0xC = 5`).
const TASK_SET_DONE: u32 = 0x0044_9150;
/// `QueuedReplacementKF` constructor (`0043e8b0`), `__thiscall(name,
/// context, animation, owner)`.
const QUEUED_REPLACEMENT_KF_CONSTRUCT: u32 = 0x0043_e8b0;
/// `QueuedModel` constructor taking a file name (`0043c6e0`), `__thiscall(name,
/// context, LOD multiplier, flag, flag)`.
const QUEUED_MODEL_CONSTRUCT: u32 = 0x0043_c6e0;
/// `QueuedModel` method (`00443ff0`, `__thiscall(flag)`): sets or clears
/// bit 0x20 of `cFlags`.
const QUEUED_MODEL_SET_FLAG_20: u32 = 0x0044_3ff0;
/// `QueuedModel::Run` (Xbox PDB, `0043ccf0`), the finishing call `LoadFile`
/// makes (`0043d180`) and the destructor (`0043c830`).
const QUEUED_MODEL_RUN: u32 = 0x0043_ccf0;
const QUEUED_MODEL_FINISH: u32 = 0x0043_d180;
const QUEUED_MODEL_DESTRUCT: u32 = 0x0043_c830;
/// `__cdecl(flag, byte pointer)` (`0044af70`): sets (non-zero `flag`) or
/// clears bit 0x10 of the byte.
const SET_FLAG_BIT_10: u32 = 0x0044_af70;
/// `QueuedKF` constructor (`0043e0f0`, `__thiscall(name, context)`), `Run`
/// (`0043e2a0`), the finishing call (`0043e4b0`) and destructor
/// (`0043e1b0`).
const QUEUED_KF_CONSTRUCT: u32 = 0x0043_e0f0;
const QUEUED_KF_RUN: u32 = 0x0043_e2a0;
const QUEUED_KF_FINISH: u32 = 0x0043_e4b0;
const QUEUED_KF_DESTRUCT: u32 = 0x0043_e1b0;
/// `Animation` method (`00490fa0`), `__thiscall(KFModel)`: takes the loaded
/// `KFModel` into the animation.
const ANIMATION_KF_LOADED: u32 = 0x0049_0fa0;
/// Interlocked increment of the dword at +0x10 (`0043be10`): for a
/// `KFModel` that is `iManualRefCount`.
const KF_MODEL_ADD_MANUAL_REFERENCE: u32 = 0x0043_be10;
/// `NiRefObject::IncRefCount` (`0092c870`).
const ADD_REFERENCE: u32 = 0x0092_c870;
/// The `NiPointer` at +0xC of an object, dereferenced (`0043b230`): for a
/// `Model` that is `spObject3D`.
const MODEL_OBJECT_3D: u32 = 0x0043_b230;
/// `iRefCount + iManualRefCount` of a `Model` (`00443190`) and of a
/// `KFModel` (`004431b0`).
const MODEL_REFERENCE_TOTAL: u32 = 0x0044_3190;
const KF_MODEL_REFERENCE_TOTAL: u32 = 0x0044_31b0;
/// `ModelLoader::QueueModel` taking a name (`00444040`), `__thiscall(name,
/// key, parent, 0, 1, 0, 0)`.
const MODEL_LOADER_QUEUE_MODEL: u32 = 0x0044_4040;
/// `ModelLoader` method (`004442c0`), `__thiscall(name, entry, key, parent,
/// 0, 1, 0, 0)`: queues a model together with an entry of the model's list.
const MODEL_LOADER_QUEUE_MODEL_WITH_ENTRY: u32 = 0x0044_42c0;
/// `ModelLoader` method (`004446a0`), `__thiscall(name, key, parent)`:
/// queues one KF file.
const MODEL_LOADER_QUEUE_KF: u32 = 0x0044_46a0;
/// `ModelLoader` method (`00444d40`), `__thiscall(queued object, key, parent,
/// actor)`.
const MODEL_LOADER_QUEUE_PARTS: u32 = 0x0044_4d40;
/// `ModelLoader::CopyFilenameList` (Xbox PDB, `00447850`), `__thiscall(source
/// list, destination list)`.
const MODEL_LOADER_COPY_FILENAME_LIST: u32 = 0x0044_7850;
/// `ModelLoader::BuildKFFileList` (Xbox PDB, `00447330`), `__thiscall(path,
/// 1, 1, creature)`.
const MODEL_LOADER_BUILD_KF_FILE_LIST: u32 = 0x0044_7330;
/// `afe420(path, name, 1, flag)` (cdecl), which `BuildFileList` forwards to.
const BUILD_FILE_LIST: u32 = 0x00af_e420;
/// `TESIdleManager::GetRootFilenameList` (Xbox PDB, `00600700`),
/// `__thiscall(path, 0)` on the idle manager at [`IDLE_MANAGER`].
const GET_ROOT_FILENAME_LIST: u32 = 0x0060_0700;
/// `Actor::GetCurrentWeapon` (Xbox PDB, `008a1710`).
const ACTOR_GET_CURRENT_WEAPON: u32 = 0x008a_1710;
/// Actor method (`008a1760`, no name in the engine map) the KF list takes as
/// its creature argument.
const ACTOR_CREATURE_FORM: u32 = 0x008a_1760;
/// Actor method (`0087f4c0`, in the actor unit): 1 for a female actor.
const ACTOR_GET_SEX: u32 = 0x0087_f4c0;
/// `TESAnimation::HasKFFiles` (Xbox PDB, `0047fd90`).
const TESANIMATION_HAS_KF_FILES: u32 = 0x0047_fd90;
/// The list of an object cast to its animation or creature class
/// (`00717e50`: `this + 4`).
const LIST_HEAD_OF_OBJECT: u32 = 0x0071_7e50;
/// `TESModel` accessor (`0048d150`: `this + 0xC`), the entry array.
const MODEL_ENTRIES: u32 = 0x0048_d150;
/// `InventoryChanges::GetInventoryChanges` (Xbox PDB, `004bf220`), `__cdecl(actor)`.
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
/// `InventoryChanges` methods (unit `inventorychanges.cpp`; `004c7400`,
/// `__thiscall(form, float out, 6, 0)`; `004c6f60`, `__thiscall(form, 0)`),
/// each giving an object that `QueueCreatureParts` hands to `00444d40`
/// and then deletes ([`fn_004459e0`]).
const INVENTORY_FIND_FIRST: u32 = 0x004c_7400;
const INVENTORY_FIND_SECOND: u32 = 0x004c_6f60;
/// Destructor body of the object `004c7400` returns (`004bc5f0`).
const INVENTORY_RESULT_DESTRUCT: u32 = 0x004b_c5f0;

/// Form type ids (`00401170`) that `00446a60` tests: `0x28` (its
/// `00522d80` is filed under `tesobjectweap.cpp` in the engine map) and
/// `0x2a` (the owner whose sex it asks for).
const WEAPON_FORM_TYPE: u32 = 0x28;
const ACTOR_BASE_FORM_TYPE: u32 = 0x2a;
/// A reference's form (`007af430`, the engine map's folded
/// `BGSSaveFormBuffer::GetForm`) and `TESObjectREFR::GetTESModel` (Xbox
/// PDB, `00571630`).
const REFERENCE_GET_FORM: u32 = 0x007a_f430;
const REFERENCE_GET_TES_MODEL: u32 = 0x0057_1630;
/// Form type id getter (`00401170`).
const FORM_TYPE: u32 = 0x0040_1170;
/// `00522d80` (model of a type `0x28` form), `TESObjectREFR::GetOwner` (Xbox
/// PDB, `00567790`), `TESActorBase::GetSex` (Xbox PDB, `005f0cc0`) and
/// `TESBipedModelForm::GetWorldTESModel_ov2` (Xbox PDB, `00481110`).
const WEAPON_GET_MODEL: u32 = 0x0052_2d80;
const REFERENCE_GET_OWNER: u32 = 0x0056_7790;
const ACTOR_BASE_GET_SEX: u32 = 0x005f_0cc0;
const BIPED_GET_WORLD_MODEL: u32 = 0x0048_1110;

/// `LockFreeMapIterator` constructor (`0044cc10`) and destructor
/// (`004499c0`), the "is done" test (`006ebde0`) and the map's
/// next-entry call (`00906570`, `__thiscall(iterator, key out, value out,
/// 1) -> bool`).
const MAP_ITERATOR_CONSTRUCT: u32 = 0x0044_cc10;
const MAP_ITERATOR_DESTRUCT: u32 = 0x0044_99c0;
const MAP_ITERATOR_IS_DONE: u32 = 0x006e_bde0;
const MAP_NEXT_ENTRY: u32 = 0x0090_6570;
/// The cell of a reference (`008d6f30`, no name in the engine map; its
/// result is the cell `TES::GetCellPriority` is asked about).
const REFERENCE_CELL: u32 = 0x008d_6f30;
/// `TESObjectREFR::Is3DCritical` (Xbox PDB, `00572350`).
const REFERENCE_IS_3D_CRITICAL: u32 = 0x0057_2350;
/// `TES::GetCellPriority` (Xbox PDB, `00458be0`), `__thiscall(cell, 0)` on
/// the object at [`TES_GLOBAL`].
const GET_CELL_PRIORITY: u32 = 0x0045_8be0;
/// The priority byte of a task's key (`0043cc60`).
const TASK_PRIORITY: u32 = 0x0043_cc60;

/// `0040fc90`: the current thread id. `00442600`, `00442510` and `00442530`
/// are the calls the loader makes on the clone thread (no names in the
/// engine map).
const CURRENT_THREAD_ID: u32 = 0x0040_fc90;
const CLONE_THREAD_CALL_FIRST: u32 = 0x0044_2600;
const CLONE_THREAD_CALL_SECOND: u32 = 0x0044_2510;
const CLONE_THREAD_CALL_THIRD: u32 = 0x0044_2530;
/// `004491f0(task manager)`: the task count.
const TASK_MANAGER_TASK_COUNT: u32 = 0x0044_91f0;
/// Setting getter (`00408d60`, `__thiscall` on a setting object: `this + 4`,
/// or the address of a zero scratch byte when `this` is 0).
const SETTING_VALUE_POINTER: u32 = 0x0040_8d60;
/// `SetWindowTextA` (import slot `00fdf2dc`), `__stdcall(window, text)`.
const SET_WINDOW_TEXT: u32 = 0x00fd_f2dc;
/// `ModelLoader` methods `00448c10` and `00448cc0` (`00446e40` ANDs them).
const LOADER_CHECK_FIRST: u32 = 0x0044_8c10;
const LOADER_CHECK_SECOND: u32 = 0x0044_8cc0;
/// `00878080(0x11deffc, 0)`, `0082fb70()`, the test `0047c850` and
/// `00861ea0`, the last two called on the object at [`MAIN_LOOP_OBJECT`].
const REFRESH_FIRST: u32 = 0x0087_8080;
const REFRESH_SECOND: u32 = 0x0082_fb70;
const MAIN_LOOP_TEST: u32 = 0x0047_c850;
const MAIN_LOOP_UPDATE: u32 = 0x0086_1ea0;

/// The loader object (`011c3b3c`).
const LOADER: u32 = 0x011c_3b3c;
/// The task manager (`01202d98`).
const TASK_MANAGER: u32 = 0x0120_2d98;
/// The object of `TES::GetCellPriority` (`011dea10`).
const TES_GLOBAL: u32 = 0x011d_ea10;
/// The idle manager (`011cb6a0`).
const IDLE_MANAGER: u32 = 0x011c_b6a0;
/// The object whose +8 is the window `00446cb0` sets the text of
/// (`011dea0c`) and the text (`011a2fe8`).
const WINDOW_OWNER: u32 = 0x011d_ea0c;
const WINDOW_TEXT: u32 = 0x011a_2fe8;
/// The object `00446ea0` tests (`011de45c`).
const MAIN_LOOP_OBJECT: u32 = 0x011d_e45c;
/// The setting object `00446e10` reads (`011c77b4`).
const LOADING_SETTING: u32 = 0x011c_77b4;
/// The byte flag `00446cb0` keeps (`011c3bfc`): set while it shows a status,
/// cleared when it resets the window text.
const STATUS_SHOWN_FLAG: u32 = 0x011c_3bfc;
/// The address `00446ef0` returns (`011deffc`).
const REFRESH_OBJECT: u32 = 0x011d_effc;
/// Two tables `QueueAnimations` indexes with the weapon type: dwords at
/// `0118a838` give an index into the string pointers at `011977a4` (the
/// holster file prefix of the weapon type).
const HOLSTER_INDEX_TABLE: u32 = 0x0118_a838;
const HOLSTER_PREFIX_TABLE: u32 = 0x0119_77a4;

/// Type descriptors `__RTDynamicCast` is given: the class every cast starts
/// from (`011831e8`; the model class), the creature class (`01183a00`) and
/// the animation class (`0118584c`) of `QueueCreatureParts` and
/// `QueueAnimations`; and `00446a60`'s source (`01183108`) with its targets
/// `011831e8` and `01183978` (the biped class).
const SOURCE_TYPE: u32 = 0x0118_31e8;
const CREATURE_TYPE: u32 = 0x0118_3a00;
const ANIMATION_TYPE: u32 = 0x0118_584c;
const FORM_SOURCE_TYPE: u32 = 0x0118_3108;
const FORM_MODEL_TYPE: u32 = 0x0118_31e8;
const FORM_BIPED_TYPE: u32 = 0x0118_3978;

/// `QueuedReplacementKFList`'s virtual table (`01016fc0`).
const QUEUED_REPLACEMENT_KF_LIST_VTABLE: u32 = 0x0101_6fc0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\ModelLoader.cpp"`.
const MODEL_LOADER_SOURCE: u32 = 0x0101_6840;
/// The memory context (`0x33`) and source line of the scope `004465f0`
/// opens.
const REPLACEMENT_CONTEXT: u32 = 0x33;
const REPLACEMENT_SOURCE_LINE: u32 = 0xe9f;

/// Strings: `"Skeleton"`, `"Data\\"`, `"Meshes\\"`, `"\\MTIdle.KF"`,
/// `"\\%sHolster.KF"`, `"\\PA%sHolster.KF"`, `"\\Death.KF"`,
/// `"\\Locomotion\\Child\\IdleAnims"`, `"\\Locomotion\\Female\\IdleAnims"`,
/// `"\\Locomotion\\Male\\IdleAnims"`, `"\\SpecialAnims\\"`.
const SKELETON_WORD: u32 = 0x0101_3458;
const DATA_PREFIX: u32 = 0x0101_6fb4;
const MESHES_PREFIX: u32 = 0x0101_6fac;
const MTIDLE_FILE: u32 = 0x0101_6fa0;
const HOLSTER_FORMAT: u32 = 0x0101_6f90;
const POWER_ARMOR_HOLSTER_FORMAT: u32 = 0x0101_6f80;
const DEATH_FILE: u32 = 0x0101_6f74;
const CHILD_IDLE_DIRECTORY: u32 = 0x0101_6f58;
const FEMALE_IDLE_DIRECTORY: u32 = 0x0101_6f38;
const MALE_IDLE_DIRECTORY: u32 = 0x0101_6f1c;
const SPECIAL_ANIMS_DIRECTORY: u32 = 0x0101_6f0c;
/// `"Loading %d references( %d background cloning ) and %d tasks( %d post
/// processing )"`.
const LOADING_STATUS_FORMAT: u32 = 0x0101_6ff8;
/// `"MODELS: LoadFile retrieved model %s with a zero ref count"` and the
/// KF version.
const LOAD_FILE_ZERO_REFERENCE_MESSAGE: u32 = 0x0101_704c;
const LOAD_KF_ZERO_REFERENCE_MESSAGE: u32 = 0x0101_7088;

/// Size of the path buffers (`strcpy_s` is told this size).
const PATH_BUFFER_SIZE: u32 = 0x104;
/// The backslash `strrchr` looks for.
const BACKSLASH: u32 = 0x5c;
/// Size of the blocks the lists allocate: a name list, a
/// `QueuedReplacementKFList` and a `QueuedReplacementKF`.
const LIST_SIZE: u32 = 8;
const QUEUED_REPLACEMENT_KF_LIST_SIZE: u32 = 0x38;
const QUEUED_REPLACEMENT_KF_SIZE: u32 = 0x40;
/// Size of a `QueuedKF` the game keeps on the stack in `LoadKF`.
const QUEUED_KF_SIZE: u32 = 0x38;

/// Runs `body` with the file name to queue for the list item `item`: the
/// item itself, or, when `prefix` is not null, the prefix and the item
/// concatenated in a 260-byte buffer.
fn with_prefixed_name<R>(
    e: &mut Engine,
    prefix: u32,
    item: u32,
    body: impl FnOnce(&mut Engine, u32) -> R,
) -> R {
    if prefix == 0 {
        return body(e, item);
    }
    e.with_stack(PATH_BUFFER_SIZE, |e, buffer| {
        e.call(STRING_COPY, &args![buffer, PATH_BUFFER_SIZE, prefix]);
        e.call(STRING_CONCAT, &args![buffer, PATH_BUFFER_SIZE, item]);
        body(e, buffer.addr())
    })
}

/// The item of the list node `node` (the dword at the node's item address).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(LIST_NODE_ITEM_ADDRESS, &args![node]).u32();
    e.mem.u32(slot)
}

/// Hands the list's files to `00446500` and deletes the list afterwards (the
/// pattern `QueueAnimations` repeats for each list it builds).
fn queue_and_delete_list(e: &mut Engine, this: Ptr, list: u32, key: u32, parent: Ptr) {
    fn_00446500(e, this, Ptr::new(list), key, parent, 0);
    if list != 0 {
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
}

/// Whether the name after the separator at `separator` is `Skeleton`.
fn is_skeleton_name(e: &mut Engine, separator: u32) -> bool {
    e.call(STRNICMP, &args![separator + 1, SKELETON_WORD, 8u32])
        .u32()
        == 0
}

/// A new empty list in an 8-byte block (0 when the allocation failed).
fn new_list(e: &mut Engine) -> u32 {
    let block = e.call(MEMORY_ALLOC, &args![LIST_SIZE]).u32();
    if block == 0 {
        0
    } else {
        e.call(LIST_CONSTRUCT, &args![block]).u32()
    }
}

// Translated from 004457d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::QueueCreatureParts` (Xbox PDB): for a model that casts to
/// the creature class, queues the models of the creature's list (the list
/// at `+0x118` of the cast, with the entry array `+0x120`, and the directory
/// of the model's own path as prefix) with [`fn_004463b0`]. With an `actor`,
/// the two objects `InventoryChanges` finds for the creature
/// (`004c7400(cast, float out, 6, 0)` and `004c6f60(cast, 0)`) are each
/// handed to `00444d40` (their dword at +8) and deleted
/// ([`fn_004459e0`]).
pub fn model_loader_queue_creature_parts(
    e: &mut Engine,
    this: Ptr,
    model: Ptr,
    key: u32,
    parent: Ptr,
    actor: Ptr,
) {
    let creature = e
        .call(
            RT_DYNAMIC_CAST,
            &args![model, 0u32, SOURCE_TYPE, CREATURE_TYPE, 0u32],
        )
        .ptr::<()>();
    if creature.is_null() {
        return;
    }
    // The creature's sub-object at +0x114 holds the list and the entries.
    let part = creature.addr() + 0x114;
    let list = e.call(LIST_HEAD_OF_OBJECT, &args![part]).u32();
    let model_path = e.vcall(model.addr(), 0x14, &args![]).u32();
    e.with_stack(PATH_BUFFER_SIZE, |e, directory| {
        e.call(STRING_COPY, &args![directory, PATH_BUFFER_SIZE, model_path]);
        let separator = e.call(STRRCHR, &args![directory, BACKSLASH]).u32();
        if separator != 0 {
            e.mem.set_u8(separator + 1, 0);
        }
        let entries = e.call(MODEL_ENTRIES, &args![part]).u32();
        fn_004463b0(
            e,
            this,
            Ptr::new(list),
            Ptr::new(entries),
            key,
            parent,
            directory.addr(),
        );
        if actor.is_null() {
            return;
        }
        let changes = e.call(GET_INVENTORY_CHANGES, &args![actor]).u32();
        let first = e.with_stack(4, |e, float_out| {
            e.call(
                INVENTORY_FIND_FIRST,
                &args![changes, creature, float_out, 6u32, 0u32],
            )
            .u32()
        });
        if first != 0 {
            let queued = e.call(FIELD_AT_8, &args![first]).u32();
            e.call(
                MODEL_LOADER_QUEUE_PARTS,
                &args![this, queued, key, parent, actor],
            );
            fn_004459e0(e, Ptr::new(first), 1);
        }
        let second = e
            .call(INVENTORY_FIND_SECOND, &args![changes, creature, 0u32])
            .u32();
        if second != 0 {
            let queued = e.call(FIELD_AT_8, &args![second]).u32();
            e.call(
                MODEL_LOADER_QUEUE_PARTS,
                &args![this, queued, key, parent, actor],
            );
            fn_004459e0(e, Ptr::new(second), 1);
        }
    });
}

// Translated from 004459e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of the object `InventoryChanges` hands to
/// `QueueCreatureParts` (no name in the engine map): runs its destructor
/// body (`004bc5f0`) and, when bit 0 of `flags` is set, frees the object.
/// Returns `this`.
pub fn fn_004459e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(INVENTORY_RESULT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 00445a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::QueueAnimations` (Xbox PDB): queues the animation files of
/// the model `model` (its virtual function `0x14` gives the path).
///
/// With an `actor` whose virtual function `0x22c(1)` is true only an actor
/// skeleton is handled ([`queue_skeleton_animations`]) and only for a path
/// whose last component is `Skeleton...`; the function ends there.
///
/// Otherwise the files come from one of two sources. When the model is a
/// skeleton or `second_flag` is set, and `first_flag` is set, it is the KF
/// file list of the path (`BuildKFFileList`, with the actor's `008a1760` as
/// creature when `actor` passes its `0x100` test). Else the actor's virtual
/// function `0x218` is tested (the game does not check `actor` for null
/// here) and, when true, the idle list ([`queue_idle_animations`]). Last, a
/// model that casts to the animation class and `HasKFFiles` queues the
/// cast's own list of file names ([`queue_special_animations`]).
///
/// Every list that is built is passed to `00446500` and deleted.
#[allow(clippy::too_many_arguments)]
pub fn model_loader_queue_animations(
    e: &mut Engine,
    this: Ptr,
    model: Ptr,
    key: u32,
    parent: Ptr,
    actor: Ptr,
    first_flag: u8,
    second_flag: u8,
) {
    let model_path = e.vcall(model.addr(), 0x14, &args![]).u32();
    e.with_stack(PATH_BUFFER_SIZE, |e, path| {
        e.call(STRING_COPY, &args![path, PATH_BUFFER_SIZE, model_path]);
        let last_separator = e.call(STRRCHR, &args![path, BACKSLASH]).u32();
        let skeleton_actor = !actor.is_null() && e.vcall(actor.addr(), 0x22c, &args![1u32]).bool();
        if skeleton_actor {
            if last_separator != 0 && is_skeleton_name(e, last_separator) {
                queue_skeleton_animations(e, this, key, parent, actor, path);
            }
            return;
        }
        let separator = e.call(STRRCHR, &args![path, BACKSLASH]).u32();
        let is_skeleton = separator != 0 && is_skeleton_name(e, separator);
        if (is_skeleton || second_flag != 0) && first_flag != 0 {
            let mut creature = 0;
            if !actor.is_null() && e.vcall(actor.addr(), 0x100, &args![]).bool() {
                creature = e.call(ACTOR_CREATURE_FORM, &args![actor]).u32();
            }
            let list = e
                .call(
                    MODEL_LOADER_BUILD_KF_FILE_LIST,
                    &args![this, path, 1u32, 1u32, creature],
                )
                .u32();
            queue_and_delete_list(e, this, list, key, parent);
        } else if e.vcall(actor.addr(), 0x218, &args![]).bool() {
            queue_idle_animations(e, this, key, parent, actor, path);
        }
        let animation = e
            .call(
                RT_DYNAMIC_CAST,
                &args![model, 0u32, SOURCE_TYPE, ANIMATION_TYPE, 0u32],
            )
            .u32();
        if animation != 0 && e.call(TESANIMATION_HAS_KF_FILES, &args![animation]).bool() {
            queue_special_animations(e, this, key, parent, model, animation);
        }
    });
}

/// The skeleton part of [`model_loader_queue_animations`]: the path
/// `Data\` (+ `Meshes\` unless the model path has it) + the model path, then
/// its last component replaced in turn by `MTIdle.KF`, when the actor passes
/// its virtual function `0x100` and has a current weapon whose type has a
/// holster prefix (tables at `0118a838` and `011977a4`) `<prefix>Holster.KF`
/// and `PA<prefix>Holster.KF`, and `Death.KF`; each is a `BuildFileList`
/// that is queued and deleted.
fn queue_skeleton_animations(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    parent: Ptr,
    actor: Ptr,
    model_path: Ptr,
) {
    e.with_stack(PATH_BUFFER_SIZE, |e, full| {
        e.call(STRING_COPY, &args![full, PATH_BUFFER_SIZE, DATA_PREFIX]);
        if e.call(STRNICMP, &args![model_path, MESHES_PREFIX, 7u32])
            .u32()
            != 0
        {
            e.call(STRING_CONCAT, &args![full, PATH_BUFFER_SIZE, MESHES_PREFIX]);
        }
        e.call(STRING_CONCAT, &args![full, PATH_BUFFER_SIZE, model_path]);
        let file_part = e.call(STRRCHR, &args![full, BACKSLASH]).u32();
        let room = PATH_BUFFER_SIZE.wrapping_sub(file_part.wrapping_sub(full.addr()));
        e.call(STRING_COPY, &args![file_part, room, MTIDLE_FILE]);
        let list = model_loader_build_file_list(e, this, full.addr(), model_path.addr(), 0);
        queue_and_delete_list(e, this, list, key, parent);
        let armed = e.vcall(actor.addr(), 0x100, &args![]).bool();
        if armed && e.call(ACTOR_GET_CURRENT_WEAPON, &args![actor]).u32() != 0 {
            let weapon = e.call(ACTOR_GET_CURRENT_WEAPON, &args![actor]).ptr::<()>();
            let weapon_type = fn_00446390(e, weapon);
            let index = e
                .mem
                .u32(HOLSTER_INDEX_TABLE.wrapping_add((weapon_type as u32) << 2));
            let prefix = e.mem.u32(HOLSTER_PREFIX_TABLE.wrapping_add(index << 2));
            if prefix != 0 && e.mem.i8(prefix) != 0 {
                for format in [HOLSTER_FORMAT, POWER_ARMOR_HOLSTER_FORMAT] {
                    e.call(SPRINTF, &args![file_part, format, prefix]);
                    let list =
                        model_loader_build_file_list(e, this, full.addr(), model_path.addr(), 0);
                    queue_and_delete_list(e, this, list, key, parent);
                }
            }
        }
        e.call(STRING_COPY, &args![file_part, room, DEATH_FILE]);
        let list = model_loader_build_file_list(e, this, full.addr(), model_path.addr(), 0);
        queue_and_delete_list(e, this, list, key, parent);
    });
}

/// The idle part of [`model_loader_queue_animations`]: the model's directory
/// with `\Locomotion\Child\IdleAnims` (actor virtual function `0x1a0(1)`),
/// `\Locomotion\Female\IdleAnims` (`0087f4c0 == 1`) or
/// `\Locomotion\Male\IdleAnims` as the last component; the idle manager's
/// root file name list for it is copied into a new list
/// (`CopyFilenameList`, on the loader at `011c3b3c`), which is queued and
/// deleted.
fn queue_idle_animations(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    parent: Ptr,
    actor: Ptr,
    model_path: Ptr,
) {
    e.with_stack(PATH_BUFFER_SIZE, |e, directory| {
        e.call(STRING_COPY, &args![directory, PATH_BUFFER_SIZE, model_path]);
        let separator = e.call(STRRCHR, &args![directory, BACKSLASH]).u32();
        let room = PATH_BUFFER_SIZE
            .wrapping_sub(separator.wrapping_sub(directory.addr()))
            .wrapping_sub(1);
        let name = if e.vcall(actor.addr(), 0x1a0, &args![1u32]).bool() {
            CHILD_IDLE_DIRECTORY
        } else if e.call(ACTOR_GET_SEX, &args![actor]).u32() == 1 {
            FEMALE_IDLE_DIRECTORY
        } else {
            MALE_IDLE_DIRECTORY
        };
        e.call(STRING_COPY, &args![separator, room, name]);
        let list = new_list(e);
        let idle_manager = e.global::<u32>(IDLE_MANAGER);
        let roots = e
            .call(
                GET_ROOT_FILENAME_LIST,
                &args![idle_manager, directory, 0u32],
            )
            .u32();
        let loader = e.global::<u32>(LOADER);
        e.call(MODEL_LOADER_COPY_FILENAME_LIST, &args![loader, roots, list]);
        queue_and_delete_list(e, this, list, key, parent);
    });
}

/// The last part of [`model_loader_queue_animations`]: a copy of every file
/// name of the cast's list, queued with `<model directory>\SpecialAnims\` as
/// prefix, then the list is deleted.
fn queue_special_animations(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    parent: Ptr,
    model: Ptr,
    animation: u32,
) {
    let list = new_list(e);
    let mut node = e.call(LIST_HEAD_OF_OBJECT, &args![animation]).u32();
    while node != 0 {
        let item = node_item(e, node);
        let size = e.call(STRLEN, &args![item]).u32().wrapping_add(1);
        let copy = e.call(MEMORY_ALLOC, &args![size]).u32();
        e.with_stack(4, |e, holder| {
            e.mem.set_u32(holder.addr(), copy);
            e.call(STRING_COPY, &args![copy, size, item]);
            e.call(LIST_PUSH_BACK, &args![list, holder]);
        });
        node = e.call(LIST_NEXT_NODE, &args![node]).u32();
    }
    let model_path = e.vcall(model.addr(), 0x14, &args![]).u32();
    e.with_stack(PATH_BUFFER_SIZE, |e, directory| {
        e.call(STRING_COPY, &args![directory, PATH_BUFFER_SIZE, model_path]);
        let separator = e.call(STRRCHR, &args![directory, BACKSLASH]).u32();
        if separator != 0 {
            let room = PATH_BUFFER_SIZE.wrapping_sub(separator.wrapping_sub(directory.addr()));
            e.call(
                STRING_COPY,
                &args![separator, room, SPECIAL_ANIMS_DIRECTORY],
            );
        }
        fn_00446500(e, this, Ptr::new(list), key, parent, directory.addr());
    });
    if list != 0 {
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
}

// Translated from 00446390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The signed byte at +0xF4 of the object `Actor::GetCurrentWeapon` returns
/// (no name in the engine map): the weapon type `QueueAnimations` indexes
/// its holster table with.
pub fn fn_00446390(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.i8(this.addr() + 0xf4) as i32
}

// Translated from 004463b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): queues every name of
/// the list `list`, in order. A name, with `prefix` put in front when it is
/// not null, goes to `QueueModel` (`00444040(name, key, parent, 0, 1, 0, 0)`)
/// when `entries` is null, otherwise to `004442c0` together with the
/// `index`th entry of `entries` ([`fn_004464c0`], 0 past the end). Empty
/// names are skipped but still count in `index`.
pub fn fn_004463b0(
    e: &mut Engine,
    this: Ptr,
    list: Ptr,
    entries: Ptr,
    key: u32,
    parent: Ptr,
    prefix: u32,
) {
    let mut node = list.addr();
    let mut index = 0u32;
    while node != 0 {
        let item = node_item(e, node);
        if item != 0 {
            with_prefixed_name(e, prefix, item, |e, name| {
                if entries.is_null() {
                    e.call(
                        MODEL_LOADER_QUEUE_MODEL,
                        &args![this, name, key, parent, 0u32, 1u32, 0u32, 0u32],
                    );
                } else {
                    let entry = fn_004464c0(e, entries, index);
                    e.call(
                        MODEL_LOADER_QUEUE_MODEL_WITH_ENTRY,
                        &args![this, name, entry, key, parent, 0u32, 1u32, 0u32, 0u32],
                    );
                }
            });
        }
        index += 1;
        node = e.call(LIST_NEXT_NODE, &args![node]).u32();
    }
}

// Translated from 004464c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `index`th entry of an array (count at +0, entry pointer at +4), or 0
/// when `index` is not below the count.
pub fn fn_004464c0(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    if index < e.mem.u32(this.addr()) {
        let entries = e.mem.u32(this.addr() + 4);
        e.mem.u32(entries + index * 4)
    } else {
        0
    }
}

// Translated from 00446500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): drains the list `list`
/// of file names. Each non-null name (with `prefix` put in front when it is
/// not null) goes to `004446a0(name, key, parent)`; the item is freed and the
/// head removed. An empty head moves on to the next node.
pub fn fn_00446500(e: &mut Engine, this: Ptr, list: Ptr, key: u32, parent: Ptr, prefix: u32) {
    let mut node = list.addr();
    while node != 0 {
        let item = node_item(e, node);
        if item == 0 {
            node = e.call(LIST_NEXT_NODE, &args![node]).u32();
            continue;
        }
        with_prefixed_name(e, prefix, item, |e, name| {
            e.call(MODEL_LOADER_QUEUE_KF, &args![this, name, key, parent]);
        });
        e.call(MEMORY_FREE, &args![item]);
        e.call(LIST_REMOVE_HEAD, &args![node]);
    }
}

// Translated from 004465f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): queues the replacement
/// KF files of `animation`. Nothing happens for a null list or animation, or
/// an empty first item. In a memory context (`0x33`, source line `0xe9f`)
/// each file name of the list (prefixed as in [`fn_00446500`]) is looked up
/// in the map of loaded KF models (`this + 4`): if found, the animation takes
/// the model (`00490fa0`). If not, the `QueuedReplacementKFList` of the
/// animation is taken from the map at `this + 0x14` or created
/// ([`fn_004469c0`]), once, and a `QueuedReplacementKF` for the name is
/// created, made a child of the list and queued (virtual function `0x20`).
/// The item is freed and the head removed. Afterwards a list without
/// children is dropped; one with children gets `parent` as parent, goes into
/// the map (virtual function `0x10` of `this + 0x14`, with 1), is put in
/// state 5 and has `CheckFinished` (virtual function `0x28`) called.
pub fn fn_004465f0(
    e: &mut Engine,
    this: Ptr,
    list: Ptr,
    animation: Ptr,
    key: u32,
    parent: Ptr,
    prefix: u32,
) {
    if list.is_null() || animation.is_null() {
        return;
    }
    if node_item(e, list.addr()) == 0 {
        return;
    }
    e.with_stack(4, |e, scope| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![
                scope,
                REPLACEMENT_CONTEXT,
                1u32,
                MODEL_LOADER_SOURCE,
                REPLACEMENT_SOURCE_LINE
            ],
        );
        e.with_stack(4, |e, queued_list| {
            e.call(TASK_POINTER_CONSTRUCT, &args![queued_list, 0u32]);
            let kf_map = e.mem.u32(this.addr() + 4);
            let list_map = e.mem.u32(this.addr() + 0x14);
            let mut node = list.addr();
            while node != 0 {
                let item = node_item(e, node);
                if item == 0 {
                    node = e.call(LIST_NEXT_NODE, &args![node]).u32();
                    continue;
                }
                with_prefixed_name(e, prefix, item, |e, name| {
                    let (found, model) = e.with_stack(4, |e, out| {
                        let hit = e.vcall(kf_map, 8, &args![name, out]).bool();
                        (hit, e.mem.u32(out.addr()))
                    });
                    if found {
                        e.call(ANIMATION_KF_LOADED, &args![animation, model]);
                        return;
                    }
                    if e.call(POINTER_GET, &args![queued_list]).u32() == 0
                        && !e.vcall(list_map, 8, &args![animation, queued_list]).bool()
                    {
                        let block = e
                            .call(MEMORY_ALLOC, &args![QUEUED_REPLACEMENT_KF_LIST_SIZE])
                            .u32();
                        let created = if block != 0 {
                            fn_004469c0(e, Ptr::new(block), animation, key).addr()
                        } else {
                            0
                        };
                        e.call(QUEUED_FILE_POINTER_ASSIGN, &args![queued_list, created]);
                    }
                    let block = e
                        .call(MEMORY_ALLOC, &args![QUEUED_REPLACEMENT_KF_SIZE])
                        .u32();
                    let kf = if block != 0 {
                        let owner = e.call(POINTER_GET, &args![queued_list]).u32();
                        e.call(
                            QUEUED_REPLACEMENT_KF_CONSTRUCT,
                            &args![block, name, key, animation, owner],
                        )
                        .u32()
                    } else {
                        0
                    };
                    e.with_stack(4, |e, kf_pointer| {
                        e.call(TASK_POINTER_CONSTRUCT, &args![kf_pointer, kf]);
                        let owner = e.call(POINTER_GET, &args![queued_list]).u32();
                        let child = e.call(POINTER_GET, &args![kf_pointer]).u32();
                        e.call(QUEUED_FILE_SET_PARENT, &args![child, owner]);
                        let child = e.call(POINTER_GET, &args![kf_pointer]).u32();
                        e.vcall(child, 0x20, &args![]);
                        e.call(TASK_POINTER_DESTRUCT, &args![kf_pointer]);
                    });
                });
                e.call(MEMORY_FREE, &args![item]);
                e.call(LIST_REMOVE_HEAD, &args![node]);
            }
            let queued = e.call(POINTER_GET, &args![queued_list]).u32();
            if queued == 0 || fn_00446990(e, Ptr::new(queued)) == 0 {
                e.call(QUEUED_FILE_POINTER_ASSIGN, &args![queued_list, 0u32]);
            } else {
                let queued = e.call(POINTER_GET, &args![queued_list]).u32();
                e.call(QUEUED_FILE_SET_PARENT, &args![queued, parent]);
                e.vcall(list_map, 0x10, &args![animation, queued_list, 1u32]);
                let queued = e.call(POINTER_GET, &args![queued_list]).u32();
                e.call(TASK_SET_DONE, &args![queued]);
                let queued = e.call(POINTER_GET, &args![queued_list]).u32();
                e.vcall(queued, 0x28, &args![]);
            }
            e.call(TASK_POINTER_DESTRUCT, &args![queued_list]);
        });
        e.call(MEMORY_CONTEXT_LEAVE, &args![scope]);
    });
}

// Translated from 00446990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at +8 of the queued file's `pChildren` (the number of
/// children), 0 when there is no `pChildren`.
pub fn fn_00446990(e: &mut Engine, this: Ptr) -> u32 {
    let children = e.mem.u32(this.addr() + QueuedFile::pChildren.off);
    if children == 0 {
        0
    } else {
        e.call(FIELD_AT_8, &args![children]).u32()
    }
}

// Translated from 004469c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReplacementKFList` constructor (no name in the engine map): the
/// `QueuedFile` constructor with `context`, the list's virtual table, `pAnim`
/// set to `animation` and both child counters 0. Returns `this`.
pub fn fn_004469c0(
    e: &mut Engine,
    this: Ptr<QueuedReplacementKFList>,
    animation: Ptr,
    context: u32,
) -> Ptr<QueuedReplacementKFList> {
    e.call(QUEUED_FILE_CONSTRUCT, &args![this, context]);
    e.mem
        .set_u32(this.addr(), QUEUED_REPLACEMENT_KF_LIST_VTABLE);
    e.set(this, QueuedReplacementKFList::pAnim, animation);
    e.set(this, QueuedReplacementKFList::iPostProcessingChildCount, 0);
    e.set(this, QueuedReplacementKFList::iPostProcessedChildCount, 0);
    this
}

// Translated from 00446a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReplacementKFList::_scalar_deleting_destructor_` (Xbox PDB): runs
/// the destructor body (`0043dc50`) and, when bit 0 of `flags` is set, frees
/// the object. Returns `this`.
pub fn queued_replacement_kf_list_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<QueuedReplacementKFList>,
    flags: u32,
) -> Ptr<QueuedReplacementKFList> {
    e.call(QUEUED_FILE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
    this
}

// Translated from 00446a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `QueuedReplacementKFList::CheckFinished` (Xbox PDB):
/// `QueuedFile::CheckFinished`.
pub fn queued_replacement_kf_list_check_finished(
    e: &mut Engine,
    this: Ptr<QueuedReplacementKFList>,
) {
    e.call(QUEUED_FILE_CHECK_FINISHED, &args![this]);
}

// Translated from 00446a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map; `this` is not used):
/// the model of `form` for `reference`. When the reference's form
/// (`007af430`) is `form`, it is the reference's `GetTESModel`. Otherwise:
/// for a form of type `0x28` it is `00522d80(form)`; else a form that casts
/// to the model class is itself the result; else a form that casts to the
/// biped class gives `GetWorldTESModel_ov2` with the sex of the
/// reference's owner when the owner (`GetOwner`) has type `0x2a` (else 0).
/// Anything else gives 0.
pub fn fn_00446a60(e: &mut Engine, _this: Ptr, form: Ptr, reference: Ptr) -> Ptr {
    if !reference.is_null() && e.call(REFERENCE_GET_FORM, &args![reference]).ptr::<()>() == form {
        return e.call(REFERENCE_GET_TES_MODEL, &args![reference]).ptr();
    }
    if !form.is_null() && e.call(FORM_TYPE, &args![form]).u32() == WEAPON_FORM_TYPE {
        return e.call(WEAPON_GET_MODEL, &args![form]).ptr();
    }
    let as_model = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, FORM_SOURCE_TYPE, FORM_MODEL_TYPE, 0u32],
        )
        .ptr::<()>();
    if !as_model.is_null() {
        return as_model;
    }
    let as_biped = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, FORM_SOURCE_TYPE, FORM_BIPED_TYPE, 0u32],
        )
        .u32();
    if as_biped == 0 {
        return Ptr::NULL;
    }
    let mut sex = 0;
    if !reference.is_null() {
        let owner = e.call(REFERENCE_GET_OWNER, &args![reference]).u32();
        if owner != 0 && e.call(FORM_TYPE, &args![owner]).u32() == ACTOR_BASE_FORM_TYPE {
            sex = e.call(ACTOR_BASE_GET_SEX, &args![owner]).u32();
        }
    }
    e.call(BIPED_GET_WORLD_MODEL, &args![as_biped, sex]).ptr()
}

// Translated from 00446b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): walks the map of
/// queued references (`this + 8`) with an iterator. For each entry whose
/// reference has a cell (`008d6f30`) the wanted priority is 0 when that cell
/// is `cell` and the reference is `Is3DCritical`, otherwise
/// `TES::GetCellPriority(cell, 0)`; when it differs from the priority byte of
/// the queued task's key (`0043cc60`), the task's virtual function `0x1c` is
/// called with the new priority.
pub fn fn_00446b50(e: &mut Engine, this: Ptr, cell: Ptr) {
    let map = e.mem.u32(this.addr() + 8);
    e.with_stack(0x10, |e, iterator| {
        e.call(MAP_ITERATOR_CONSTRUCT, &args![iterator]);
        while !e.call(MAP_ITERATOR_IS_DONE, &args![iterator]).bool() {
            e.with_stack(8, |e, slots| {
                let key = slots;
                let value = slots.byte_add(4);
                e.call(TASK_POINTER_CONSTRUCT, &args![value, 0u32]);
                let more = e
                    .call(MAP_NEXT_ENTRY, &args![map, iterator, key, value, 1u32])
                    .bool();
                if more {
                    let reference = e.mem.u32(key.addr());
                    let entry_cell = e.call(REFERENCE_CELL, &args![reference]).u32();
                    if entry_cell != 0 {
                        let task = e.call(POINTER_GET, &args![value]).u32();
                        let current = e.call(TASK_PRIORITY, &args![task]).u32();
                        let critical = entry_cell == cell.addr()
                            && e.call(REFERENCE_IS_3D_CRITICAL, &args![reference]).bool();
                        let wanted = if critical {
                            0
                        } else {
                            let tes = e.global::<u32>(TES_GLOBAL);
                            e.call(GET_CELL_PRIORITY, &args![tes, entry_cell, 0u32])
                                .u32()
                        };
                        if current != wanted {
                            let task = e.call(POINTER_GET, &args![value]).u32();
                            e.vcall(task, 0x1c, &args![wanted]);
                        }
                    }
                }
                e.call(TASK_POINTER_DESTRUCT, &args![value]);
            });
        }
        e.call(MAP_ITERATOR_DESTRUCT, &args![iterator]);
    });
}

// Translated from 00446c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual function `0x44` of the map of queued references (`this + 8`),
/// whose result `00446cb0` prints as the number of references.
pub fn fn_00446c90(e: &mut Engine, this: Ptr) -> u32 {
    let map = e.mem.u32(this.addr() + 8);
    e.vcall(map, 0x44, &args![]).u32()
}

// Translated from 00446cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader` method (no name in the engine map): the loading status.
/// Unless the setting `00446e10` is set, it counts the queued references
/// ([`fn_00446c90`]), the background cloning work ([`fn_00446dc0`]), the
/// tasks of the task manager (`004491f0`) and its post-processing tasks
/// ([`fn_00446da0`]). When any is non-zero the text `Loading %d
/// references( %d background cloning ) and %d tasks( %d post processing )`
/// is formatted into a local buffer (the game never uses the text) and the
/// flag `011c3bfc` is set. When all are zero and the flag is set it is
/// cleared and the text `011a2fe8` is set on the window at +8 of the object
/// at `011dea0c` (`SetWindowTextA`).
pub fn fn_00446cb0(e: &mut Engine, this: Ptr) {
    if fn_00446e10(e) != 0 {
        return;
    }
    let references = fn_00446c90(e, this);
    let cloning = fn_00446dc0(e, this);
    let manager = e.global::<u32>(TASK_MANAGER);
    let tasks = e.call(TASK_MANAGER_TASK_COUNT, &args![manager]).u32();
    let post_processing = fn_00446da0(e, Ptr::new(manager));
    if tasks != 0 || cloning != 0 || references != 0 || post_processing != 0 {
        e.with_stack(PATH_BUFFER_SIZE, |e, buffer| {
            e.call(
                FORMAT_BUFFER,
                &args![
                    buffer,
                    PATH_BUFFER_SIZE,
                    LOADING_STATUS_FORMAT,
                    references,
                    cloning,
                    tasks,
                    post_processing
                ],
            );
        });
        e.mem.set_u8(STATUS_SHOWN_FLAG, 1);
    } else if e.mem.u8(STATUS_SHOWN_FLAG) != 0 {
        e.mem.set_u8(STATUS_SHOWN_FLAG, 0);
        let text = e.global::<u32>(WINDOW_TEXT);
        let owner = e.global::<u32>(WINDOW_OWNER);
        let window = e.call(FIELD_AT_8, &args![owner]).u32();
        e.call(SET_WINDOW_TEXT, &args![window, text]);
    }
}

// Translated from 00446da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual function `0xc` of the object at +0x64 of the task manager: the
/// number of post-processing tasks.
pub fn fn_00446da0(e: &mut Engine, this: Ptr) -> u32 {
    let queues = e.mem.u32(this.addr() + 0x64);
    e.vcall(queues, 0xc, &args![]).u32()
}

// Translated from 00446dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The background clone thread's work count ([`fn_00446de0`] of
/// `pBackgroundCloneThread`, at `this + 0x28`).
pub fn fn_00446dc0(e: &mut Engine, this: Ptr) -> u32 {
    let thread = e.mem.u32(this.addr() + 0x28);
    fn_00446de0(e, Ptr::new(thread))
}

// Translated from 00446de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BackgroundCloneThread` method (no name in the engine map): virtual
/// function `0x10` of `pCloneReferencesQueue` (+0x38) plus `iRunningCount`
/// (+0x34).
pub fn fn_00446de0(e: &mut Engine, this: Ptr) -> u32 {
    let queue = e.mem.u32(this.addr() + 0x38);
    let queued = e.vcall(queue, 0x10, &args![]).u32();
    queued.wrapping_add(e.mem.u32(this.addr() + 0x34))
}

// Translated from 00446e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte value of the setting object at `011c77b4`: the object is the static itself
/// (00446e13 `MOV ECX,0x11c77b4`), and `00408d60` answers where its value is.
pub fn fn_00446e10(e: &mut Engine) -> u8 {
    let value = e.call(SETTING_VALUE_POINTER, &args![LOADING_SETTING]).u32();
    e.mem.u8(value)
}

// Translated from 00446e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_00446cb0`] on the loader at `011c3b3c`.
pub fn fn_00446e30(e: &mut Engine) {
    let loader = e.global::<u32>(LOADER);
    fn_00446cb0(e, Ptr::new(loader));
}

// Translated from 00446e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether both `00448c10` and `00448cc0` (called on the loader at
/// `011c3b3c`, both always) answer true.
pub fn fn_00446e40(e: &mut Engine) -> bool {
    let loader = e.global::<u32>(LOADER);
    let first = e.call(LOADER_CHECK_FIRST, &args![loader]).bool();
    let second = e.call(LOADER_CHECK_SECOND, &args![loader]).bool();
    first && second
}

// Translated from 00446ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refreshes the screen state: `00878080(0x11deffc, 0)`, `0082fb70()`, then,
/// when `0047c850` of the object at `011de45c` is true (the game tests it
/// twice), `00861ea0` on it.
pub fn fn_00446ea0(e: &mut Engine) {
    let object = fn_00446ef0(e);
    e.call(REFRESH_FIRST, &args![object, 0u32]);
    e.call(REFRESH_SECOND, &args![]);
    let main_loop = e.global::<u32>(MAIN_LOOP_OBJECT);
    if e.call(MAIN_LOOP_TEST, &args![main_loop]).bool() {
        let main_loop = e.global::<u32>(MAIN_LOOP_OBJECT);
        if e.call(MAIN_LOOP_TEST, &args![main_loop]).bool() {
            let main_loop = e.global::<u32>(MAIN_LOOP_OBJECT);
            e.call(MAIN_LOOP_UPDATE, &args![main_loop]);
        }
    }
}

// Translated from 00446ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address `011deffc`.
pub fn fn_00446ef0(_e: &mut Engine) -> u32 {
    REFRESH_OBJECT
}

// Translated from 00446f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_00446dc0`] on the loader at `011c3b3c`.
pub fn fn_00446f00(e: &mut Engine) -> u32 {
    let loader = e.global::<u32>(LOADER);
    fn_00446dc0(e, Ptr::new(loader))
}

// Translated from 00446f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_00446f20`] on the loader at `011c3b3c`.
pub fn fn_00446f10(e: &mut Engine) {
    let loader = e.global::<u32>(LOADER);
    fn_00446f20(e, Ptr::new(loader));
}

// Translated from 00446f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_00446f40`] on `pBackgroundCloneThread` (`this + 0x28`).
pub fn fn_00446f20(e: &mut Engine, this: Ptr) {
    let thread = e.mem.u32(this.addr() + 0x28);
    fn_00446f40(e, Ptr::new(thread));
}

// Translated from 00446f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BackgroundCloneThread` method (no name in the engine map): when the
/// current thread id differs from the thread's `iThreadID` (+8), runs
/// [`fn_00446f70`].
pub fn fn_00446f40(e: &mut Engine, this: Ptr) {
    let thread_id = e.call(FIELD_AT_8, &args![this]).u32();
    if thread_id != e.call(CURRENT_THREAD_ID, &args![]).u32() {
        fn_00446f70(e, this);
    }
}

// Translated from 00446f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BackgroundCloneThread` method (no name in the engine map): `00442600`
/// then `00442510` on the thread.
pub fn fn_00446f70(e: &mut Engine, this: Ptr) {
    e.call(CLONE_THREAD_CALL_FIRST, &args![this]);
    e.call(CLONE_THREAD_CALL_SECOND, &args![this]);
}

// Translated from 00446f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_00446fa0`] on the loader at `011c3b3c`.
pub fn fn_00446f90(e: &mut Engine) {
    let loader = e.global::<u32>(LOADER);
    fn_00446fa0(e, Ptr::new(loader));
}

// Translated from 00446fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_00446fc0`] on `pBackgroundCloneThread` (`this + 0x28`).
pub fn fn_00446fa0(e: &mut Engine, this: Ptr) {
    let thread = e.mem.u32(this.addr() + 0x28);
    fn_00446fc0(e, Ptr::new(thread));
}

// Translated from 00446fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BackgroundCloneThread` method (no name in the engine map): when the
/// current thread id differs from the thread's `iThreadID` (+8), runs
/// [`fn_00446ff0`].
pub fn fn_00446fc0(e: &mut Engine, this: Ptr) {
    let thread_id = e.call(FIELD_AT_8, &args![this]).u32();
    if thread_id != e.call(CURRENT_THREAD_ID, &args![]).u32() {
        fn_00446ff0(e, this);
    }
}

// Translated from 00446ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BackgroundCloneThread` method (no name in the engine map): `00442600`
/// then `00442530` on the thread.
pub fn fn_00446ff0(e: &mut Engine, this: Ptr) {
    e.call(CLONE_THREAD_CALL_FIRST, &args![this]);
    e.call(CLONE_THREAD_CALL_THIRD, &args![this]);
}

// Translated from 00447010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a model to the loader's map of models (no name in the engine map):
/// virtual function `0x10` of the map (`this + 0`) with the name, the
/// address of the model word and 0. False when the name is already there.
pub fn fn_00447010(e: &mut Engine, this: Ptr, name: u32, model: Ptr) -> bool {
    let map = e.mem.u32(this.addr());
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), model.addr());
        e.vcall(map, 0x10, &args![name, slot, 0u32]).bool()
    })
}

// Translated from 00447040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a `KFModel` to the loader's map of KF models (no name in the engine
/// map): virtual function `0x10` of the map at `this + 4` with the name, the
/// address of the model word and 0. False when the name is already there.
pub fn fn_00447040(e: &mut Engine, this: Ptr, name: u32, kf_model: Ptr) -> bool {
    let map = e.mem.u32(this.addr() + 4);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), kf_model.addr());
        e.vcall(map, 0x10, &args![name, slot, 0u32]).bool()
    })
}

// Translated from 00447080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::LoadFile` (Xbox PDB): the 3D object of the model of file
/// `name`. The map of models (`this + 0`) is asked first (virtual function
/// 8); a model found with a zero reference total (`00443190`) is logged.
/// Otherwise a `QueuedModel` on the stack is built for the name
/// (`0043c6e0(name, 0, lod_fade_mult, first_flag, 0)`), `override_flag` is
/// given to `00443ff0`, bit 0x10 of `cFlags` is set, and the task is run and
/// finished in place; its `spModel` is the model. A model gets one more
/// reference (`0092c870`) unless `no_reference` is set, and the result is its
/// 3D object (`0043b230`); 0 when there is no model. The word after
/// `first_flag` is not read.
#[allow(clippy::too_many_arguments)]
pub fn model_loader_load_file(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    lod_fade_mult: u32,
    first_flag: u8,
    _unused_4: u32,
    override_flag: u8,
    no_reference: u8,
) -> Ptr {
    let map = e.mem.u32(this.addr());
    let (found, mut model) = e.with_stack(4, |e, out| {
        let hit = e.vcall(map, 8, &args![name, out]).bool();
        (hit, e.mem.u32(out.addr()))
    });
    if !found {
        model = e.with_stack(<QueuedModel as Layout>::SIZE, |e, task| {
            e.call(
                QUEUED_MODEL_CONSTRUCT,
                &args![task, name, 0u32, lod_fade_mult, first_flag, 0u8],
            );
            e.call(QUEUED_MODEL_SET_FLAG_20, &args![task, override_flag]);
            fn_00447190(e, task.cast(), 1);
            e.call(QUEUED_MODEL_RUN, &args![task]);
            e.call(QUEUED_MODEL_FINISH, &args![task]);
            let loaded = e
                .call(POINTER_GET, &args![task.byte_add(QueuedModel::spModel.off)])
                .u32();
            e.call(QUEUED_MODEL_DESTRUCT, &args![task]);
            loaded
        });
    } else if e.call(MODEL_REFERENCE_TOTAL, &args![model]).u32() == 0 {
        e.call(LOG, &args![LOAD_FILE_ZERO_REFERENCE_MESSAGE, name]);
    }
    if model == 0 {
        return Ptr::NULL;
    }
    if no_reference == 0 {
        e.call(ADD_REFERENCE, &args![model]);
    }
    e.call(MODEL_OBJECT_3D, &args![model]).ptr()
}

// Translated from 00447190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`flag` non-zero) or clears bit 0x10 of the queued model's `cFlags`.
pub fn fn_00447190(e: &mut Engine, this: Ptr<QueuedModel>, flag: u8) {
    e.call(
        SET_FLAG_BIT_10,
        &args![flag, this.byte_add(QueuedModel::cFlags.off)],
    );
}

// Translated from 004471c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::LoadKF` (Xbox PDB): the `KFModel` of file `name`. The map of
/// KF models (`this + 4`) is asked first (virtual function 8); one found with
/// a zero reference total (`004431b0`) is logged. Otherwise a `QueuedKF` on
/// the stack (`0043e0f0(name, 0)`) is run and finished in place and its
/// `spKFModel` is the model, and the stack object is destroyed. A model
/// gets one more manual reference (`0043be10`). Returns it (0 when none).
pub fn model_loader_load_kf(e: &mut Engine, this: Ptr, name: u32) -> Ptr {
    let map = e.mem.u32(this.addr() + 4);
    let (found, mut model) = e.with_stack(4, |e, out| {
        let hit = e.vcall(map, 8, &args![name, out]).bool();
        (hit, e.mem.u32(out.addr()))
    });
    if !found {
        model = e.with_stack(QUEUED_KF_SIZE, |e, task| {
            e.call(QUEUED_KF_CONSTRUCT, &args![task, name, 0u32]);
            e.call(QUEUED_KF_RUN, &args![task]);
            e.call(QUEUED_KF_FINISH, &args![task]);
            let loaded = e
                .call(POINTER_GET, &args![task.byte_add(QueuedKF::spKFModel.off)])
                .u32();
            e.call(QUEUED_KF_DESTRUCT, &args![task]);
            loaded
        });
    } else if e.call(KF_MODEL_REFERENCE_TOTAL, &args![model]).u32() == 0 {
        e.call(LOG, &args![LOAD_KF_ZERO_REFERENCE_MESSAGE, name]);
    }
    if model != 0 {
        e.call(KF_MODEL_ADD_MANUAL_REFERENCE, &args![model]);
    }
    Ptr::new(model)
}

// Translated from 004472a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::FindModel` (Xbox PDB): clears the `NiPointer<Model>` `out`,
/// asks the map of models (`this + 0`, virtual function 8) for `name` and,
/// when found, stores the model into `out`. Returns whether it was found.
pub fn model_loader_find_model(e: &mut Engine, this: Ptr, name: u32, out: Ptr) -> bool {
    e.call(MODEL_POINTER_ASSIGN, &args![out, 0u32]);
    let map = e.mem.u32(this.addr());
    let (found, model) = e.with_stack(4, |e, slot| {
        let hit = e.vcall(map, 8, &args![name, slot]).bool();
        (hit, e.mem.u32(slot.addr()))
    });
    if found {
        e.call(MODEL_POINTER_ASSIGN, &args![out, model]);
    }
    found
}

// Translated from 00447300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::BuildFileList` (Xbox PDB; `this` is not used): the file
/// list `00afe420(path, name, 1, flag)` builds (cdecl). Returns it.
pub fn model_loader_build_file_list(
    e: &mut Engine,
    _this: Ptr,
    path: u32,
    name: u32,
    flag: u32,
) -> u32 {
    e.call(BUILD_FILE_LIST, &args![path, name, 1u32, flag])
        .u32()
}

// ---- Second session: `00447330` up to `004491c0` ----

/// `TESIdleManager::AddRootIdleArray` (Xbox PDB, `00600170`),
/// `__thiscall(path, list, flag)` on the idle manager.
const ADD_ROOT_IDLE_ARRAY: u32 = 0x0060_0170;
/// `BSSimpleList::IsEmpty` (`008256d0`): no item and no next node.
const SIMPLE_LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// Source line of the memory scope `BuildKFFileList` opens (context `0x33`).
const KF_LIST_SOURCE_LINE: u32 = 0x108e;
/// Strings: `"Data\\Meshes\\"`, `"\\*.KF"`, `"\\Locomotion\\"`,
/// `"\\Locomotion\\*.KF"`, `"Hurt\\"`, `"IdleAnims"` and
/// `"\\Locomotion\\Hurt\\*.KF"`.
const DATA_MESHES_DIRECTORY: u32 = 0x0101_70c4;
const KF_WILDCARD: u32 = 0x0101_7124;
const LOCOMOTION_DIRECTORY: u32 = 0x0101_7114;
const LOCOMOTION_KF_WILDCARD: u32 = 0x0101_7100;
const HURT_DIRECTORY: u32 = 0x0101_70f8;
const IDLE_ANIMS_WORD: u32 = 0x0101_70ec;
const LOCOMOTION_HURT_KF_WILDCARD: u32 = 0x0101_70d4;
/// Number of root idle lists `BuildKFFileList` merges when it is given the
/// creature argument `0xc`.
const ROOT_LIST_COUNT: u32 = 0xc;

/// The queued file the loader builds in a 0x40-byte block for a file name
/// (`0043d540`, `__thiscall(name, 4, key)`).
const QUEUED_NAMED_FILE_CONSTRUCT: u32 = 0x0043_d540;
/// Constructors of the 0x40-byte queued face generation files:
/// `00441f80(file name, key, flag)` and `004420b0(loaded file, key)`
/// (`QueueFaceGenFile`), `00442130(first, second)` (`QueueEGMFile`,
/// `QueueTRIFile`).
const QUEUED_FACE_GEN_FROM_NAME: u32 = 0x0044_1f80;
const QUEUED_FACE_GEN_FROM_FILE: u32 = 0x0044_20b0;
const QUEUED_FACE_GEN_FROM_PAIR: u32 = 0x0044_2130;
/// The smart pointer to a loaded file that `QueueFaceGenFile` keeps on the
/// stack: constructor with a file (`0044b0c0`) and destructor (`0044b160`).
const LOADED_FILE_POINTER_CONSTRUCT: u32 = 0x0044_b0c0;
const LOADED_FILE_POINTER_DESTRUCT: u32 = 0x0044_b160;
/// The two smart pointer slots `QueueEGMFile` and `QueueTRIFile` keep on the
/// stack, with their constructors (`00633c90`, `0044b270`, one argument)
/// and destructors (`0045cec0`, `0044b2c0`).
const FIRST_SLOT_CONSTRUCT: u32 = 0x0063_3c90;
const FIRST_SLOT_DESTRUCT: u32 = 0x0045_cec0;
const SECOND_SLOT_CONSTRUCT: u32 = 0x0044_b270;
const SECOND_SLOT_DESTRUCT: u32 = 0x0044_b2c0;
/// The name object of the face generation file functions: constructor
/// (`004037b0`) and destructor (`004037d0`).
const NAME_OBJECT_CONSTRUCT: u32 = 0x0040_37b0;
const NAME_OBJECT_DESTRUCT: u32 = 0x0040_37d0;
/// `BSFaceGenManager::GetAsEGMFile` (Xbox PDB, `00653520`) and
/// `GetAsTRIFile` (`006536d0`), `__cdecl(name object, path, index)`.
const FACE_GEN_GET_AS_EGM_FILE: u32 = 0x0065_3520;
const FACE_GEN_GET_AS_TRI_FILE: u32 = 0x0065_36d0;
/// `BSFaceGenManager::GetModelCache` (Xbox PDB, `00652110`) and the cache's
/// lookup (`006502f0`, `__thiscall(file, first slot, second slot) -> bool`).
const FACE_GEN_GET_MODEL_CACHE: u32 = 0x0065_2110;
const FACE_GEN_CACHE_LOOKUP: u32 = 0x0065_02f0;
/// Virtual slots of the queued files that `QueueFaceGenFile` and the
/// functions around it start (`0x20`) or, when a loaded file already
/// exists, start after the cache lookup (`0x28`).
const QUEUED_RUN_SLOT: u32 = 0x20;
const QUEUED_CACHED_RUN_SLOT: u32 = 0x28;

/// Task manager helpers: `00c3e310` before and `00c3e340` after
/// `00448420` cancels everything, `0044ac40` is
/// `BSTaskManager::CancelTask` (Xbox PDB, `__thiscall(task, 0)`), and virtual
/// slot `0x54` is the call that follows the loops.
const TASK_MANAGER_BEFORE_CANCEL: u32 = 0x00c3_e310;
const TASK_MANAGER_AFTER_CANCEL: u32 = 0x00c3_e340;
const TASK_MANAGER_CANCEL_TASK: u32 = 0x0044_ac40;
const TASK_MANAGER_FLUSH_SLOT: u32 = 0x54;
/// `00528e40(0)`, a `__thiscall(flag)` on the object at +0xC of the loader.
const LOADER_THIRD_CLEAR: u32 = 0x0052_8e40;
/// Map iterators of the loader's maps: constructors (`0044cc50`, `0044cc90`,
/// `004498e0`, `00449960`) and destructors (`004499e0`, `00449a00`,
/// `004431d0`, `00443220`).
const MAP_ITERATOR_CONSTRUCT_SECOND: u32 = 0x0044_cc50;
const MAP_ITERATOR_CONSTRUCT_THIRD: u32 = 0x0044_cc90;
const MAP_ITERATOR_DESTRUCT_SECOND: u32 = 0x0044_99e0;
const MAP_ITERATOR_DESTRUCT_THIRD: u32 = 0x0044_9a00;
const MODEL_ITERATOR_CONSTRUCT: u32 = 0x0044_98e0;
const MODEL_ITERATOR_DESTRUCT: u32 = 0x0044_31d0;
const KF_ITERATOR_CONSTRUCT: u32 = 0x0044_9960;
const KF_ITERATOR_DESTRUCT: u32 = 0x0044_3220;
/// Next entry of the maps `00448620` and `00448cc0` walk (`0044c640`,
/// `0044b8d0`), `__thiscall(map, iterator, key out, value out, 1) -> bool`.
const MODEL_MAP_NEXT_ENTRY: u32 = 0x0044_c640;
const FOURTH_MAP_NEXT_ENTRY: u32 = 0x0044_b8d0;
/// The test of the main loop object (`0042ce10`, on the object at
/// [`MAIN_LOOP_FLAG_OBJECT`]).
const MAIN_LOOP_FLAG_TEST: u32 = 0x0042_ce10;
const MAIN_LOOP_FLAG_OBJECT: u32 = 0x011d_df38;
/// Model destructor wrapper (`004431f0`, `__thiscall(model, 1)`) and KF model
/// one (`00443240`).
const MODEL_RELEASE: u32 = 0x0044_31f0;
const KF_MODEL_RELEASE: u32 = 0x0044_3240;
/// `005585e0` (a `__thiscall` getter whose result `005f2420` reads) and
/// `005f2420`: the KF model's owner and the number `00448620` compares with
/// `0x5c` and `0x66`.
const KF_MODEL_OWNER: u32 = 0x0055_85e0;
const KF_MODEL_OWNER_NUMBER: u32 = 0x005f_2420;
/// Add-on lookup chain of `LoadAddons`: the add-on index of a node
/// (`009ee040`), the data handler's lookup (`004617e0`, `__thiscall(index)`
/// on the object at [`DATA_HANDLER`]) and the extra data lookup of
/// `LoadAddonNodes` (`00a5bdd0`, `__thiscall(name)`).
const NODE_ADDON_INDEX: u32 = 0x009e_e040;
const DATA_HANDLER_GET_ADDON_NODE: u32 = 0x0046_17e0;
const OBJECT_GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// The extra data name `LoadAddonNodes` asks for (`01202ddc`) and the type
/// descriptor `LoadAddons` casts with (`01202de8`).
const ADDON_NODE_NAME: u32 = 0x0120_2ddc;
const ADDON_NODE_TYPE: u32 = 0x0120_2de8;
/// `0043b300(type descriptor, object)` (`__cdecl`), the child count of a node
/// (`0043b480`), a child (`0043b4a0`, `__thiscall(index)`) and the model
/// destroyer `0043acb0`.
const OBJECT_IS_OF_TYPE: u32 = 0x0043_b300;
const NODE_CHILD_COUNT: u32 = 0x0043_b480;
const NODE_CHILD_AT: u32 = 0x0043_b4a0;
const MODEL_DESTROY: u32 = 0x0043_acb0;
/// `ModelLoader::LoadFile` (Xbox PDB, `00447080`).
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
/// The queue of the loader (`+0x1c`) is drained with `00449280(queue,
/// slot)`.
const QUEUE_NEXT_TASK: u32 = 0x0044_9280;
/// The object at +0x28 of the loader gets `00442630` called on it.
const LOADER_CLONE_THREAD_CALL: u32 = 0x0044_2630;
/// `BSPrecisionTimer::GetTimer` (Xbox PDB, `00aa4d80`) returns the current
/// time in `EDX:EAX`; `00ec62f6` converts the `ST0` value to a 64-bit integer
/// the same way; the scale `00448e30` multiplies the seconds by is the float
/// at `011ac39c`.
const TIMER_GET_TIME: u32 = 0x00aa_4d80;
const FLOAT_TO_INT64: u32 = 0x00ec_62f6;
const TIMER_SCALE: u32 = 0x011a_c39c;
/// The two timeouts (`0101712c`: 5.0, `01017130`: 100000.0) `00448cc0`
/// picks between.
const SHORT_TIMEOUT: u32 = 0x0101_712c;
const LONG_TIMEOUT: u32 = 0x0101_7130;
/// Object `00449090` constructs and `004490e0` releases (`011c3b38`), with
/// the calls it makes on it (`0040b460`, `004019a0`, both `__cdecl(object)`),
/// and the virtual table both set (`01017138`).
const SYNC_OBJECT: u32 = 0x011c_3b38;
const SYNC_OBJECT_INIT: u32 = 0x0040_b460;
const SYNC_OBJECT_RELEASE: u32 = 0x0040_19a0;
const SYNC_VTABLE: u32 = 0x0101_7138;
/// The value `0043b460` compare-exchanges (`004491c0`) and the call the
/// function makes afterwards (`0040fbe0`).
const COMPARE_EXCHANGE: u32 = 0x0043_b460;
const AFTER_COMPARE_EXCHANGE: u32 = 0x0040_fbe0;
/// `004fd400(object) -> i32`, which `00447950` tests for a positive value.
const OBJECT_POSITIVE_COUNT: u32 = 0x004f_d400;

/// Builds a `QueuedFile`-derived task, wraps it in a task pointer, sets its
/// parent and calls its virtual function `slot` (the pattern the queueing
/// functions repeat).
fn start_queued_task(e: &mut Engine, task: u32, parent: u32, slot: u32) {
    e.with_stack(4, |e, pointer| {
        e.call(TASK_POINTER_CONSTRUCT, &args![pointer, task]);
        let child = e.call(POINTER_GET, &args![pointer]).u32();
        e.call(QUEUED_FILE_SET_PARENT, &args![child, parent]);
        let child = e.call(POINTER_GET, &args![pointer]).u32();
        e.vcall(child, slot, &args![]);
        e.call(TASK_POINTER_DESTRUCT, &args![pointer]);
    });
}

/// A 0x40-byte block for a queued task; 0 when the allocation failed.
fn allocate_task_block(e: &mut Engine) -> u32 {
    e.call(MEMORY_ALLOC, &args![QUEUED_REPLACEMENT_KF_SIZE])
        .u32()
}

// Translated from 00447330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::BuildKFFileList` (Xbox PDB): the list of the KF files of
/// `path` (and, with `locomotion`, of its `Locomotion` directories). With an
/// idle manager that already knows the path, the lists the manager holds are
/// copied ([`model_loader_copy_filename_list`]): all twelve of them when
/// `creature` is `0xc`, the first one and the one of `creature` when
/// `append_creature` is set and `creature` is not 0, otherwise the one of
/// `creature`. Without them the files are searched on disk
/// (`Data\Meshes\<path>\*.KF`, then the `Locomotion` directory) and handed
/// to the idle manager (`AddRootIdleArray`). Runs inside a memory scope
/// (context `0x33`, `ModelLoader.cpp` line `0x108e`). Returns the list (0
/// when the path has no backslash).
pub fn model_loader_build_kf_file_list(
    e: &mut Engine,
    this: Ptr,
    path: u32,
    locomotion: u8,
    append_creature: u8,
    creature: u32,
) -> u32 {
    e.with_stack(4, |e, scope| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![
                scope,
                REPLACEMENT_CONTEXT,
                1u32,
                MODEL_LOADER_SOURCE,
                KF_LIST_SOURCE_LINE
            ],
        );
        let manager = e.global::<u32>(IDLE_MANAGER);
        let known = manager != 0
            && e.call(GET_ROOT_FILENAME_LIST, &args![manager, path, 0u32])
                .u32()
                != 0;
        let result = if known {
            let mut list = 0;
            if creature == ROOT_LIST_COUNT {
                for index in 0..ROOT_LIST_COUNT {
                    let source = e
                        .call(GET_ROOT_FILENAME_LIST, &args![manager, path, index])
                        .u32();
                    if list == 0 {
                        list = model_loader_copy_filename_list(e, this, source, 0);
                    } else {
                        model_loader_copy_filename_list(e, this, source, list);
                    }
                }
            } else if creature != 0 && append_creature != 0 {
                let source = e
                    .call(GET_ROOT_FILENAME_LIST, &args![manager, path, 0u32])
                    .u32();
                list = model_loader_copy_filename_list(e, this, source, 0);
                let source = e
                    .call(GET_ROOT_FILENAME_LIST, &args![manager, path, creature])
                    .u32();
                model_loader_copy_filename_list(e, this, source, list);
            } else {
                let source = e
                    .call(GET_ROOT_FILENAME_LIST, &args![manager, path, creature])
                    .u32();
                list = model_loader_copy_filename_list(e, this, source, 0);
            }
            list
        } else {
            build_kf_file_list_from_disk(e, this, path, locomotion != 0)
        };
        e.call(MEMORY_CONTEXT_LEAVE, &args![scope]);
        result
    })
}

/// The disk search of [`model_loader_build_kf_file_list`] (the branch for a
/// path the idle manager does not know).
fn build_kf_file_list_from_disk(e: &mut Engine, this: Ptr, path: u32, locomotion: bool) -> u32 {
    e.with_stack(PATH_BUFFER_SIZE, |e, pattern| {
        e.call(
            STRING_COPY,
            &args![pattern, PATH_BUFFER_SIZE, DATA_MESHES_DIRECTORY],
        );
        e.call(STRING_CONCAT, &args![pattern, PATH_BUFFER_SIZE, path]);
        let separator = e.call(STRRCHR, &args![pattern, BACKSLASH]).u32();
        if separator == 0 {
            return 0;
        }
        let room = PATH_BUFFER_SIZE - (separator - pattern.addr());
        e.call(STRING_COPY, &args![separator, room, KF_WILDCARD]);
        let mut list = model_loader_build_file_list(e, this, pattern.addr(), path, 0);
        if locomotion && e.call(STRLEN, &args![path]).u32() != 0 {
            e.with_stack(PATH_BUFFER_SIZE, |e, directory| {
                e.call(STRING_COPY, &args![directory, PATH_BUFFER_SIZE, path]);
                let name_end = e.call(STRRCHR, &args![directory, BACKSLASH]).u32();
                if name_end == 0 {
                    return;
                }
                e.mem.set_u8(name_end, 0);
                let name_room = PATH_BUFFER_SIZE - (name_end - directory.addr());
                e.call(
                    STRING_CONCAT,
                    &args![name_end, name_room, LOCOMOTION_DIRECTORY],
                );
                e.call(STRING_COPY, &args![separator, room, LOCOMOTION_KF_WILDCARD]);
                if list == 0 {
                    list =
                        model_loader_build_file_list(e, this, pattern.addr(), directory.addr(), 0);
                } else {
                    model_loader_build_file_list(e, this, pattern.addr(), directory.addr(), list);
                }
                e.call(STRING_CONCAT, &args![name_end, name_room, HURT_DIRECTORY]);
                e.call(STRING_CONCAT, &args![name_end, name_room, IDLE_ANIMS_WORD]);
                e.call(
                    STRING_COPY,
                    &args![separator, room, LOCOMOTION_HURT_KF_WILDCARD],
                );
                let hurt =
                    model_loader_build_file_list(e, this, pattern.addr(), directory.addr(), 0);
                if hurt != 0 {
                    let manager = e.global::<u32>(IDLE_MANAGER);
                    e.call(ADD_ROOT_IDLE_ARRAY, &args![manager, directory, hurt, 1u32]);
                    while !e.call(SIMPLE_LIST_IS_EMPTY, &args![hurt]).bool() {
                        let item = node_item(e, hurt);
                        e.call(MEMORY_FREE, &args![item]);
                        e.call(LIST_REMOVE_HEAD, &args![hurt]);
                    }
                    e.call(LIST_DELETE, &args![hurt, 1u32]);
                }
            });
        }
        let manager = e.global::<u32>(IDLE_MANAGER);
        e.call(ADD_ROOT_IDLE_ARRAY, &args![manager, path, list, 0u32]);
        list
    })
}

// Translated from 00447850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::CopyFilenameList` (Xbox PDB; `this` is not used): appends a
/// copy of every file name of the list `source` to `destination` (a new empty
/// list when it is 0) and returns the destination.
pub fn model_loader_copy_filename_list(
    e: &mut Engine,
    _this: Ptr,
    source: u32,
    destination: u32,
) -> u32 {
    let destination = if destination == 0 {
        new_list(e)
    } else {
        destination
    };
    let mut node = source;
    while node != 0 {
        let item = node_item(e, node);
        let size = e.call(STRLEN, &args![item]).u32().wrapping_add(1);
        let copy = e.call(MEMORY_ALLOC, &args![size]).u32();
        e.with_stack(4, |e, holder| {
            e.mem.set_u32(holder.addr(), copy);
            e.call(STRING_COPY, &args![copy, size, item]);
            e.call(LIST_PUSH_BACK, &args![destination, holder]);
        });
        node = e.call(LIST_NEXT_NODE, &args![node]).u32();
    }
    destination
}

// Translated from 00447950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `object` is not null and `004fd400(object)` is positive (`this`
/// is not used).
pub fn fn_00447950(e: &mut Engine, _this: Ptr, object: u32) -> bool {
    object != 0 && e.call(OBJECT_POSITIVE_COUNT, &args![object]).i32() > 0
}

// Translated from 00447980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the 0x40-byte queued file `0043d540(first, 4, second)` (nothing
/// when the allocation fails) and starts it (virtual function `0x20`);
/// `this` is not used.
pub fn fn_00447980(e: &mut Engine, _this: Ptr, first: u32, second: u32) {
    let block = e
        .call(MEMORY_ALLOC, &args![QUEUED_REPLACEMENT_KF_SIZE])
        .u32();
    let task = if block != 0 {
        e.call(
            QUEUED_NAMED_FILE_CONSTRUCT,
            &args![block, first, 4u32, second],
        )
        .u32()
    } else {
        0
    };
    e.with_stack(4, |e, pointer| {
        e.call(TASK_POINTER_CONSTRUCT, &args![pointer, task]);
        let queued = e.call(POINTER_GET, &args![pointer]).u32();
        e.vcall(queued, QUEUED_RUN_SLOT, &args![]);
        e.call(TASK_POINTER_DESTRUCT, &args![pointer]);
    });
}

// Translated from 00447a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::QueueFaceGenFile` (Xbox PDB): looks the loaded file `name`
/// up in the loader's file map ([`fn_004483a0`]) into a smart pointer. When
/// none is loaded, a queued file `00441f80(name, key, flag)` is built, given
/// `parent` and started (virtual function `0x20`). When one is, and `parent`
/// is not 0, a queued file `004420b0(loaded file, key)` is built, given
/// `parent` and started with virtual function `0x28`.
pub fn model_loader_queue_face_gen_file(
    e: &mut Engine,
    this: Ptr,
    name: u32,
    key: u32,
    parent: u32,
    flag: u32,
) {
    e.with_stack(4, |e, loaded| {
        let found = fn_004483a0(e, this, name);
        e.call(LOADED_FILE_POINTER_CONSTRUCT, &args![loaded, found]);
        if e.call(POINTER_GET, &args![loaded]).u32() == 0 {
            let block = allocate_task_block(e);
            let task = if block != 0 {
                e.call(QUEUED_FACE_GEN_FROM_NAME, &args![block, name, key, flag])
                    .u32()
            } else {
                0
            };
            start_queued_task(e, task, parent, QUEUED_RUN_SLOT);
        } else if parent != 0 {
            let block = allocate_task_block(e);
            let task = if block != 0 {
                let file = e.call(POINTER_GET, &args![loaded]).u32();
                e.call(QUEUED_FACE_GEN_FROM_FILE, &args![block, file, key])
                    .u32()
            } else {
                0
            };
            start_queued_task(e, task, parent, QUEUED_CACHED_RUN_SLOT);
        }
        e.call(LOADED_FILE_POINTER_DESTRUCT, &args![loaded]);
    });
}

/// What `QueueEGMFile` and `QueueTRIFile` ask the face generation model
/// cache: whether the cache exists and its lookup of the file in the name
/// object (`name_object`) fills the two slots.
fn face_gen_cache_lookup(e: &mut Engine, name_object: Ptr, first: Ptr, second: Ptr) -> bool {
    if e.call(FACE_GEN_GET_MODEL_CACHE, &args![]).u32() == 0 {
        return false;
    }
    let file = e.call(POINTER_GET, &args![name_object]).u32();
    let cache = e.call(FACE_GEN_GET_MODEL_CACHE, &args![]).u32();
    e.call(FACE_GEN_CACHE_LOOKUP, &args![cache, file, first, second])
        .bool()
}

/// Builds the queued file `00442130(first, second)` from the two slots, gives
/// it `parent` and starts it with virtual function `0x28` (nothing when
/// `parent` is 0).
fn start_face_gen_pair(e: &mut Engine, first: Ptr, second: Ptr, parent: u32) {
    if parent == 0 {
        return;
    }
    let block = allocate_task_block(e);
    let task = if block != 0 {
        let second_value = e.call(POINTER_GET, &args![second]).u32();
        let first_value = e.call(POINTER_GET, &args![first]).u32();
        e.call(
            QUEUED_FACE_GEN_FROM_PAIR,
            &args![block, first_value, second_value],
        )
        .u32()
    } else {
        0
    };
    start_queued_task(e, task, parent, QUEUED_CACHED_RUN_SLOT);
}

/// The state `QueueEGMFile` and `QueueTRIFile` pass around: the loader, the
/// two smart pointer slots the cache lookup fills, the task key and the
/// parent.
struct FaceGenQueue {
    this: Ptr,
    first: Ptr,
    second: Ptr,
    key: u32,
    parent: u32,
}

/// One file of `QueueEGMFile`: the name object for `GetAsEGMFile(path,
/// index)`; when the cache has the file (both slots set and the second one's
/// dword at +8 not 0) the pair is queued, otherwise
/// [`model_loader_queue_face_gen_file`] is asked for the name.
fn queue_egm_part(e: &mut Engine, queue: &FaceGenQueue, path: u32, index: u32) {
    e.with_stack(8, |e, name_object| {
        e.call(NAME_OBJECT_CONSTRUCT, &args![name_object]);
        e.call(FACE_GEN_GET_AS_EGM_FILE, &args![name_object, path, index]);
        let (first, second) = (queue.first, queue.second);
        let mut found = face_gen_cache_lookup(e, name_object, first, second);
        if found {
            found = e.call(POINTER_GET, &args![first]).u32() != 0
                && e.call(POINTER_GET, &args![second]).u32() != 0;
        }
        if found {
            let second_value = e.call(POINTER_GET, &args![second]).u32();
            // Slot object +8 (no layout known).
            found = e.mem.u32(second_value + 8) != 0;
        }
        if found {
            start_face_gen_pair(e, first, second, queue.parent);
        } else {
            let name = e.call(POINTER_GET, &args![name_object]).u32();
            model_loader_queue_face_gen_file(e, queue.this, name, queue.key, queue.parent, 1);
        }
        e.call(NAME_OBJECT_DESTRUCT, &args![name_object]);
    });
}

// Translated from 00447bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::QueueEGMFile` (Xbox PDB): the face generation file of the
/// model `model` (`Meshes\` and the model's path, virtual function `0x14`).
/// With `kind` 9 both sides (indices 0 and 1) are queued, otherwise the one
/// file (index -1); each through [`queue_egm_part`].
pub fn model_loader_queue_egm_file(
    e: &mut Engine,
    this: Ptr,
    model: Ptr,
    key: u32,
    parent: u32,
    kind: u32,
) {
    e.with_stack(4, |e, first| {
        e.call(FIRST_SLOT_CONSTRUCT, &args![first, 0u32]);
        e.with_stack(4, |e, second| {
            e.call(SECOND_SLOT_CONSTRUCT, &args![second, 0u32]);
            let queue = FaceGenQueue {
                this,
                first,
                second,
                key,
                parent,
            };
            e.with_stack(PATH_BUFFER_SIZE, |e, path| {
                e.call(STRING_COPY, &args![path, PATH_BUFFER_SIZE, MESHES_PREFIX]);
                let model_path = e.vcall(model.addr(), 0x14, &args![]).u32();
                e.call(STRING_CONCAT, &args![path, PATH_BUFFER_SIZE, model_path]);
                if kind == 9 {
                    for index in 0..2u32 {
                        queue_egm_part(e, &queue, path.addr(), index);
                    }
                } else {
                    queue_egm_part(e, &queue, path.addr(), u32::MAX);
                }
            });
            e.call(SECOND_SLOT_DESTRUCT, &args![second]);
        });
        e.call(FIRST_SLOT_DESTRUCT, &args![first]);
    });
}

// Translated from 00448080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::QueueTRIFile` (Xbox PDB): like [`model_loader_queue_egm_file`]
/// for the single file (index -1), but the cache is asked about the EGM name
/// and only both slots being set is needed; without that, the TRI file name
/// (`GetAsTRIFile(path, -1)`) goes to [`model_loader_queue_face_gen_file`].
/// The fourth word is not read.
pub fn model_loader_queue_tri_file(
    e: &mut Engine,
    this: Ptr,
    model: Ptr,
    key: u32,
    parent: u32,
    _unused_4: u32,
) {
    e.with_stack(4, |e, first| {
        e.call(FIRST_SLOT_CONSTRUCT, &args![first, 0u32]);
        e.with_stack(4, |e, second| {
            e.call(SECOND_SLOT_CONSTRUCT, &args![second, 0u32]);
            e.with_stack(PATH_BUFFER_SIZE, |e, path| {
                e.call(STRING_COPY, &args![path, PATH_BUFFER_SIZE, MESHES_PREFIX]);
                let model_path = e.vcall(model.addr(), 0x14, &args![]).u32();
                e.call(STRING_CONCAT, &args![path, PATH_BUFFER_SIZE, model_path]);
                e.with_stack(8, |e, egm_name| {
                    e.call(NAME_OBJECT_CONSTRUCT, &args![egm_name]);
                    e.call(FACE_GEN_GET_AS_EGM_FILE, &args![egm_name, path, u32::MAX]);
                    let mut found = face_gen_cache_lookup(e, egm_name, first, second);
                    if found {
                        found = e.call(POINTER_GET, &args![first]).u32() != 0
                            && e.call(POINTER_GET, &args![second]).u32() != 0;
                    }
                    if found {
                        start_face_gen_pair(e, first, second, parent);
                    } else {
                        e.with_stack(8, |e, tri_name| {
                            e.call(NAME_OBJECT_CONSTRUCT, &args![tri_name]);
                            e.call(FACE_GEN_GET_AS_TRI_FILE, &args![tri_name, path, u32::MAX]);
                            let name = e.call(POINTER_GET, &args![tri_name]).u32();
                            model_loader_queue_face_gen_file(e, this, name, key, parent, 1);
                            e.call(NAME_OBJECT_DESTRUCT, &args![tri_name]);
                        });
                    }
                    e.call(NAME_OBJECT_DESTRUCT, &args![egm_name]);
                });
            });
            e.call(SECOND_SLOT_DESTRUCT, &args![second]);
        });
        e.call(FIRST_SLOT_DESTRUCT, &args![first]);
    });
}

// Translated from 00448330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual function `0x10` of the map at `this + 0x24`, called with
/// `(first, address of a local holding second, 0)`; returns its result.
pub fn fn_00448330(e: &mut Engine, this: Ptr, first: u32, second: u32) -> bool {
    let map = e.mem.u32(this.addr() + 0x24);
    e.with_stack(4, |e, holder| {
        e.mem.set_u32(holder.addr(), second);
        e.vcall(map, 0x10, &args![first, holder, 0u32]).bool()
    })
}

// Translated from 00448370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual function `0x14` of the map at `this + 0x24`, called with `name`.
pub fn fn_00448370(e: &mut Engine, this: Ptr, name: u32) {
    let map = e.mem.u32(this.addr() + 0x24);
    e.vcall(map, 0x14, &args![name]);
}

// Translated from 004483a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The loaded file the map at `this + 0x24` has for `name` (virtual
/// function 8), or 0. (The engine map names this `LoadedFile::IncRefCount`,
/// a folded name; the body is a map lookup.)
pub fn fn_004483a0(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let map = e.mem.u32(this.addr() + 0x24);
    e.with_stack(4, |e, out| {
        if e.vcall(map, 8, &args![name, out]).bool() {
            e.mem.u32(out.addr())
        } else {
            0
        }
    })
}

// Translated from 004483e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same lookup as [`fn_004483a0`] in the map at `this + 4`.
pub fn fn_004483e0(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let map = e.mem.u32(this.addr() + 4);
    e.with_stack(4, |e, out| {
        if e.vcall(map, 8, &args![name, out]).bool() {
            e.mem.u32(out.addr())
        } else {
            0
        }
    })
}

/// Walks a map with its iterator and cancels every task in it (the loop
/// `00448420` repeats for three maps).
fn cancel_tasks_in_map(e: &mut Engine, map: u32, iterator: Ptr) {
    while !e.call(MAP_ITERATOR_IS_DONE, &args![iterator]).bool() {
        e.with_stack(4, |e, key| {
            e.with_stack(4, |e, task| {
                e.call(TASK_POINTER_CONSTRUCT, &args![task, 0u32]);
                if e.call(MAP_NEXT_ENTRY, &args![map, iterator, key, task, 1u32])
                    .bool()
                {
                    let manager = e.global::<u32>(TASK_MANAGER);
                    let queued = e.call(POINTER_GET, &args![task]).u32();
                    e.call(TASK_MANAGER_CANCEL_TASK, &args![manager, queued, 0u32]);
                }
                e.call(TASK_POINTER_DESTRUCT, &args![task]);
            });
        });
    }
}

// Translated from 00448420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cancels every task of the three maps at `this + 8`, `this + 0x10` and
/// `this + 0x18` (the task manager's cancel call for each), clearing the
/// object at `this + 0xC` between the first and the second, then calls the
/// task manager's virtual function `0x54`. `00c3e310` is called on the task
/// manager first and `00c3e340` last.
pub fn fn_00448420(e: &mut Engine, this: Ptr) {
    let manager = e.global::<u32>(TASK_MANAGER);
    e.call(TASK_MANAGER_BEFORE_CANCEL, &args![manager]);
    e.with_stack(0x10, |e, first| {
        e.call(MAP_ITERATOR_CONSTRUCT, &args![first]);
        let map = e.mem.u32(this.addr() + 8);
        cancel_tasks_in_map(e, map, first);
        let cleared = e.mem.u32(this.addr() + 0xc);
        e.call(LOADER_THIRD_CLEAR, &args![cleared, 0u32]);
        e.with_stack(0x10, |e, second| {
            e.call(MAP_ITERATOR_CONSTRUCT_SECOND, &args![second]);
            let map = e.mem.u32(this.addr() + 0x10);
            cancel_tasks_in_map(e, map, second);
            e.with_stack(0x10, |e, third| {
                e.call(MAP_ITERATOR_CONSTRUCT_THIRD, &args![third]);
                let map = e.mem.u32(this.addr() + 0x18);
                cancel_tasks_in_map(e, map, third);
                let manager = e.global::<u32>(TASK_MANAGER);
                e.vcall(manager, TASK_MANAGER_FLUSH_SLOT, &args![]);
                let manager = e.global::<u32>(TASK_MANAGER);
                e.call(TASK_MANAGER_AFTER_CANCEL, &args![manager]);
                e.call(MAP_ITERATOR_DESTRUCT_THIRD, &args![third]);
            });
            e.call(MAP_ITERATOR_DESTRUCT_SECOND, &args![second]);
        });
        e.call(MAP_ITERATOR_DESTRUCT, &args![first]);
    });
}

// Translated from 00448620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the models and KF models nobody uses from the loader's maps
/// (`this + 0` and `this + 4`). With `force` 0 and the main loop object's
/// test (`0042ce10`) true, only the byte at `this + 0x2c` is set to 1 and
/// nothing is removed. Otherwise a model with a zero reference total
/// (`00443190`) is removed from its map (virtual function `0x14` with its
/// key) and released (`004431f0(model, 1)`); a KF model with a zero total
/// (`004431b0`) likewise, unless its owner's number (`005f2420`) is within
/// `0x5c` to `0x65`. The byte at `this + 0x2c` is then 0.
pub fn fn_00448620(e: &mut Engine, this: Ptr, force: u8) {
    let main_loop = e.global::<u32>(MAIN_LOOP_FLAG_OBJECT);
    if force == 0 && main_loop != 0 && e.call(MAIN_LOOP_FLAG_TEST, &args![main_loop]).bool() {
        e.mem.set_u8(this.addr() + 0x2c, 1);
        return;
    }
    let models = e.mem.u32(this.addr());
    if models != 0 {
        e.with_stack(0x114, |e, iterator| {
            e.call(MODEL_ITERATOR_CONSTRUCT, &args![iterator]);
            while !e.call(MAP_ITERATOR_IS_DONE, &args![iterator]).bool() {
                let (key, model) = e.with_stack(8, |e, out| {
                    let found = e
                        .call(
                            MODEL_MAP_NEXT_ENTRY,
                            &args![models, iterator, out, out.addr() + 4, 1u32],
                        )
                        .bool();
                    (
                        e.mem.u32(out.addr()),
                        if found { e.mem.u32(out.addr() + 4) } else { 0 },
                    )
                });
                if model != 0 && e.call(MODEL_REFERENCE_TOTAL, &args![model]).u32() == 0 {
                    e.vcall(models, 0x14, &args![key]);
                    e.call(MODEL_RELEASE, &args![model, 1u32]);
                }
            }
            e.call(MODEL_ITERATOR_DESTRUCT, &args![iterator]);
        });
    }
    let kf_models = e.mem.u32(this.addr() + 4);
    if kf_models != 0 {
        e.with_stack(0x114, |e, iterator| {
            e.call(KF_ITERATOR_CONSTRUCT, &args![iterator]);
            while !e.call(MAP_ITERATOR_IS_DONE, &args![iterator]).bool() {
                let (key, model) = e.with_stack(8, |e, out| {
                    let found = e
                        .call(
                            MODEL_MAP_NEXT_ENTRY,
                            &args![kf_models, iterator, out, out.addr() + 4, 1u32],
                        )
                        .bool();
                    (
                        e.mem.u32(out.addr()),
                        if found { e.mem.u32(out.addr() + 4) } else { 0 },
                    )
                });
                if model != 0 && e.call(KF_MODEL_REFERENCE_TOTAL, &args![model]).u32() == 0 {
                    let mut remove = true;
                    if e.call(KF_MODEL_OWNER, &args![model]).u32() != 0 {
                        let owner = e.call(KF_MODEL_OWNER, &args![model]).u32();
                        if e.call(KF_MODEL_OWNER_NUMBER, &args![owner]).i32() >= 0x5c {
                            let owner = e.call(KF_MODEL_OWNER, &args![model]).u32();
                            if e.call(KF_MODEL_OWNER_NUMBER, &args![owner]).i32() < 0x66 {
                                remove = false;
                            }
                        }
                    }
                    if remove {
                        e.vcall(kf_models, 0x14, &args![key]);
                        e.call(KF_MODEL_RELEASE, &args![model, 1u32]);
                    }
                }
            }
            e.call(KF_ITERATOR_DESTRUCT, &args![iterator]);
        });
    }
    e.mem.set_u8(this.addr() + 0x2c, 0);
}

// Translated from 00448920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::TryAndRemoveModel` (Xbox PDB): for a model with a zero
/// reference total (`00443190`): unless the main loop object's test is true
/// (then the byte at `this + 0x2c` is set to 1 instead), removes `key` from
/// the model map (virtual function `0x14` of `this + 0`) and releases the
/// model (`004431f0(model, 1)`).
pub fn model_loader_try_and_remove_model(e: &mut Engine, this: Ptr, model: u32, key: u32) {
    if e.call(MODEL_REFERENCE_TOTAL, &args![model]).u32() != 0 {
        return;
    }
    let mut remove = true;
    let main_loop = e.global::<u32>(MAIN_LOOP_FLAG_OBJECT);
    if main_loop != 0 && e.call(MAIN_LOOP_FLAG_TEST, &args![main_loop]).bool() {
        remove = false;
        e.mem.set_u8(this.addr() + 0x2c, 1);
    }
    if remove {
        let models = e.mem.u32(this.addr());
        e.vcall(models, 0x14, &args![key]);
        if model != 0 {
            e.call(MODEL_RELEASE, &args![model, 1u32]);
        }
    }
}

// Translated from 004489b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::LoadAddonNodes` (Xbox PDB): for `node` not null whose extra
/// data of the name at `01202ddc` exists, with `object` not null, that extra
/// data having bit 0x10 of its dword at +0xC ([`fn_00448a40`]) and `object`
/// not having bit 0x80 of its dword at +8 ([`fn_00448a20`]):
/// [`model_loader_load_addons`] of the node.
pub fn model_loader_load_addon_nodes(e: &mut Engine, this: Ptr, object: Ptr, node: u32) {
    if node == 0 {
        return;
    }
    let name = fn_00448a80(e);
    let extra = e.call(OBJECT_GET_EXTRA_DATA, &args![node, name]).u32();
    if extra != 0 && !object.is_null() && fn_00448a40(e, Ptr::new(extra)) && !fn_00448a20(e, object)
    {
        model_loader_load_addons(e, this, node);
    }
}

// Translated from 00448a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x80 of the dword at +8 is set.
pub fn fn_00448a20(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 8) & 0x80 != 0
}

// Translated from 00448a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x10 of the dword at +0xC is set ([`fn_00448a60`] with 0x10).
pub fn fn_00448a40(e: &mut Engine, this: Ptr) -> bool {
    fn_00448a60(e, this, 0x10) != 0
}

// Translated from 00448a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at +0xC ANDed with `mask`.
pub fn fn_00448a60(e: &mut Engine, this: Ptr, mask: u32) -> u32 {
    e.mem.u32(this.addr() + 0xc) & mask
}

// Translated from 00448a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dword at `01202ddc` (the extra data name `LoadAddonNodes` asks for).
pub fn fn_00448a80(e: &mut Engine) -> u32 {
    e.global::<u32>(ADDON_NODE_NAME)
}

// Translated from 00448a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ModelLoader::LoadAddons` (Xbox PDB): for a `node` that is of the type at
/// `01202de8` (`0043b300`) the add-on index (`009ee040`) gives the data
/// handler's add-on (`004617e0`); unless its byte at +0x5a has bit 0
/// ([`fn_00448bf0`]) the name from the object at add-on `+ 0x30` (virtual
/// function `0x14`) is looked up in the model map (virtual function 8) and,
/// when it is not there, loaded with [`model_loader_load_file`] and looked up
/// again; a found model is destroyed (`0043acb0`). Then every child of the
/// node that has a virtual function `0xc` result is handled recursively.
pub fn model_loader_load_addons(e: &mut Engine, this: Ptr, node: u32) {
    if node == 0 {
        return;
    }
    if e.call(OBJECT_IS_OF_TYPE, &args![ADDON_NODE_TYPE, node])
        .bool()
    {
        let index = e.call(NODE_ADDON_INDEX, &args![node]).u32();
        let handler = e.global::<u32>(DATA_HANDLER);
        let addon = e
            .call(DATA_HANDLER_GET_ADDON_NODE, &args![handler, index])
            .u32();
        if addon != 0 && !fn_00448bf0(e, Ptr::new(addon)) {
            let models = e.mem.u32(this.addr());
            e.with_stack(4, |e, found| {
                let name = e.vcall(addon + 0x30, 0x14, &args![]).u32();
                if !e.vcall(models, 8, &args![name, found]).bool() {
                    let name = e.vcall(addon + 0x30, 0x14, &args![]).u32();
                    let loaded = e.call(
                        MODEL_LOADER_LOAD_FILE,
                        &args![this, name, 0u32, 1u32, 0u32, 0u32, 0u32],
                    );
                    if loaded.u32() != 0 {
                        let name = e.vcall(addon + 0x30, 0x14, &args![]).u32();
                        if e.vcall(models, 8, &args![name, found]).bool() {
                            let model = e.mem.u32(found.addr());
                            e.call(MODEL_DESTROY, &args![model]);
                        }
                    }
                }
            });
        }
    }
    // The child count is asked again on every pass, as the game does.
    let mut index = 0;
    while index < e.call(NODE_CHILD_COUNT, &args![node]).u32() {
        let child = e.call(NODE_CHILD_AT, &args![node, index]).u32();
        if child != 0 {
            let sub = e.vcall(child, 0xc, &args![]).u32();
            if sub != 0 {
                model_loader_load_addons(e, this, sub);
            }
        }
        index += 1;
    }
}

// Translated from 00448bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the byte at +0x5a is set.
pub fn fn_00448bf0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x5a) & 1 != 0
}

// Translated from 00448c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drains the queue at the dword `this + 0x1c`: while `00449280(queue, slot)`
/// leaves a task in the slot, virtual function `0x14` of that task is
/// called. Always returns true.
pub fn fn_00448c10(e: &mut Engine, this: Ptr) -> bool {
    let queue = e.mem.u32(this.addr() + 0x1c);
    e.with_stack(4, |e, slot| {
        e.call(TASK_POINTER_CONSTRUCT, &args![slot, 0u32]);
        e.call(QUEUE_NEXT_TASK, &args![queue, slot]);
        loop {
            let task = e.call(POINTER_GET, &args![slot]).u32();
            if task == 0 {
                break;
            }
            let task = e.call(POINTER_GET, &args![slot]).u32();
            e.vcall(task, 0x14, &args![]);
            e.call(QUEUE_NEXT_TASK, &args![queue, slot]);
        }
        e.call(TASK_POINTER_DESTRUCT, &args![slot]);
    });
    true
}

// Translated from 00448cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts a timer ([`fn_00448e00`], [`bs_precision_timer_reset`]) of 5.0
/// seconds (100000.0 when the task manager's dword at +0x68 is 6,
/// [`fn_00448de0`]) and walks the map at `this + 0xC`: until the iterator is
/// done or the timer has run out ([`fn_00448e70`]), every task it yields has
/// its virtual function `0x20` called. Returns whether the iterator is done.
pub fn fn_00448cc0(e: &mut Engine, this: Ptr) -> bool {
    e.with_stack(0x10, |e, timer| {
        fn_00448e00(e, timer);
        let manager = e.global::<u32>(TASK_MANAGER);
        let seconds = if fn_00448de0(e, Ptr::new(manager)) {
            e.global::<f32>(LONG_TIMEOUT)
        } else {
            e.global::<f32>(SHORT_TIMEOUT)
        };
        bs_precision_timer_reset(e, timer, seconds);
        e.with_stack(0x10, |e, iterator| {
            e.call(MAP_ITERATOR_CONSTRUCT, &args![iterator]);
            let map = e.mem.u32(this.addr() + 0xc);
            while !e.call(MAP_ITERATOR_IS_DONE, &args![iterator]).bool() && !fn_00448e70(e, timer) {
                e.with_stack(4, |e, key| {
                    e.with_stack(4, |e, task| {
                        e.call(TASK_POINTER_CONSTRUCT, &args![task, 0u32]);
                        if e.call(
                            FOURTH_MAP_NEXT_ENTRY,
                            &args![map, iterator, key, task, 1u32],
                        )
                        .bool()
                        {
                            let queued = e.call(POINTER_GET, &args![task]).u32();
                            e.vcall(queued, 0x20, &args![]);
                        }
                        e.call(TASK_POINTER_DESTRUCT, &args![task]);
                    });
                });
            }
            let done = e.call(MAP_ITERATOR_IS_DONE, &args![iterator]).bool();
            e.call(MAP_ITERATOR_DESTRUCT, &args![iterator]);
            done
        })
    })
}

// Translated from 00448de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the dword at +0x68 is not 6.
pub fn fn_00448de0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x68) != 6
}

// Translated from 00448e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the four dwords of a timer; returns it.
pub fn fn_00448e00(e: &mut Engine, this: Ptr) -> Ptr {
    for offset in [0, 4, 8, 0xc] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    this
}

// Translated from 00448e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSPrecisionTimer::Reset` (Xbox PDB): stores the current time
/// (`GetTimer`, 64 bits) at +0 and the 64-bit integer of `seconds` times the
/// float at `011ac39c` (`00ec62f6`) at +8.
pub fn bs_precision_timer_reset(e: &mut Engine, this: Ptr, seconds: f32) {
    let now = e.call(TIMER_GET_TIME, &args![]).u64();
    e.mem.set_u32(this.addr(), now as u32);
    e.mem.set_u32(this.addr() + 4, (now >> 32) as u32);
    let scale = e.global::<f32>(TIMER_SCALE);
    let ticks = e
        .call(
            FLOAT_TO_INT64,
            &args![f64::from(seconds) * f64::from(scale)],
        )
        .u64();
    e.mem.set_u32(this.addr() + 8, ticks as u32);
    e.mem.set_u32(this.addr() + 0xc, (ticks >> 32) as u32);
}

// Translated from 00448e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the time since the timer's start (+0) is above its limit (+8), as
/// signed 64-bit numbers.
pub fn fn_00448e70(e: &mut Engine, this: Ptr) -> bool {
    let now = e.call(TIMER_GET_TIME, &args![]).u64();
    let start = u64::from(e.mem.u32(this.addr())) | u64::from(e.mem.u32(this.addr() + 4)) << 32;
    let limit =
        u64::from(e.mem.u32(this.addr() + 8)) | u64::from(e.mem.u32(this.addr() + 0xc)) << 32;
    (now.wrapping_sub(start) as i64) > (limit as i64)
}

// Translated from 00448ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual function `0x10` of the object at `this + 0x20`, called with
/// `(first, a smart pointer holding second, 0)`; returns its result.
pub fn fn_00448ed0(e: &mut Engine, this: Ptr, first: u32, second: u32) -> bool {
    let target = e.mem.u32(this.addr() + 0x20);
    e.with_stack(4, |e, pointer| {
        e.call(TASK_POINTER_CONSTRUCT, &args![pointer, second]);
        let result = e.vcall(target, 0x10, &args![first, pointer, 0u32]).bool();
        e.call(TASK_POINTER_DESTRUCT, &args![pointer]);
        result
    })
}

// Translated from 00448f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual function `0x14` of the object at `this + 0x20`, called with `name`.
pub fn fn_00448f50(e: &mut Engine, this: Ptr, name: u32) {
    let target = e.mem.u32(this.addr() + 0x20);
    e.vcall(target, 0x14, &args![name]);
}

// Translated from 00448f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pointer the object at `this + 0x20` has for `name` (virtual function
/// 8 fills a smart pointer), or 0.
pub fn fn_00448f80(e: &mut Engine, this: Ptr, name: u32) -> u32 {
    let target = e.mem.u32(this.addr() + 0x20);
    e.with_stack(4, |e, pointer| {
        e.call(TASK_POINTER_CONSTRUCT, &args![pointer, 0u32]);
        let found = e.vcall(target, 8, &args![name, pointer]).bool();
        let result = if found {
            e.call(POINTER_GET, &args![pointer]).u32()
        } else {
            0
        };
        e.call(TASK_POINTER_DESTRUCT, &args![pointer]);
        result
    })
}

// Translated from 00449030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `00442630` on the dword at `this + 0x28`.
pub fn fn_00449030(e: &mut Engine, this: Ptr) {
    let target = e.mem.u32(this.addr() + 0x28);
    e.call(LOADER_CLONE_THREAD_CALL, &args![target]);
}

// Translated from 00449050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Maps 0 to 1, 1 to 0, 2 to 3 and everything else to 8.
pub fn fn_00449050(_e: &mut Engine, value: u32) -> u32 {
    match value {
        0 => 1,
        1 => 0,
        2 => 3,
        _ => 8,
    }
}

// Translated from 00449090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the object whose virtual table is `01017138`: clears the
/// dwords at +8 to +0x14 and initialises the object at `011c3b38`
/// (`0040b460`); returns it.
pub fn fn_00449090(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), SYNC_VTABLE);
    for offset in [8, 0xc, 0x10, 0x14] {
        e.mem.set_u32(this.addr() + offset, 0);
    }
    e.call(SYNC_OBJECT_INIT, &args![SYNC_OBJECT]);
    if engine_core::queue_check::enabled() {
        // Diagnostic only (`NV_QUEUE_CHECK`, not exe behaviour).
        engine_core::queue_check::created(e, this.addr());
    }
    this
}

// Translated from 004490e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of that object: sets the virtual table `01017138` and calls
/// `004019a0` on the object at `011c3b38`.
pub fn fn_004490e0(e: &mut Engine, this: Ptr) {
    if engine_core::queue_check::enabled() {
        // Diagnostic only (`NV_QUEUE_CHECK`, not exe behaviour).
        engine_core::queue_check::destroyed(e, this.addr());
    }
    e.mem.set_u32(this.addr(), SYNC_VTABLE);
    e.call(SYNC_OBJECT_RELEASE, &args![SYNC_OBJECT]);
}

// Translated from 00449110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the task's state (the dword at +0xC) is 4 or more (signed).
pub fn fn_00449110(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0xc) as i32 >= 4
}

// Translated from 00449130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the task's state (the dword at +0xC) is 6.
pub fn fn_00449130(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0xc) == 6
}

// Translated from 00449150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the task's state (the dword at +0xC) to 5.
pub fn fn_00449150(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0xc, 5);
}

// Translated from 00449170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the task's state (the dword at +0xC) to 6.
pub fn fn_00449170(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0xc, 6);
}

// Translated from 00449190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Changes the task's state (the dword at +0xC) from `expected` to `new`
/// with [`fn_004491c0`]; returns whether it succeeded.
pub fn fn_00449190(e: &mut Engine, this: Ptr, expected: u32, new: u32) -> bool {
    fn_004491c0(e, Ptr::new(this.addr() + 0xc), new, expected)
}

// Translated from 004491c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0043b460(target, expected, new)` (compare-exchange of the dword at
/// `target`; returns the previous value), then `0040fbe0()`; true when the
/// previous value was `expected`.
pub fn fn_004491c0(e: &mut Engine, target: Ptr, new: u32, expected: u32) -> bool {
    let previous = e
        .call(COMPARE_EXCHANGE, &args![target, expected, new])
        .u32();
    e.call(AFTER_COMPARE_EXCHANGE, &args![]);
    previous == expected
}

// --- 004491f0 .. 00449bd0: the map wrappers, the name helpers, the vtable setters ---

/// Critical-section style lock of the map object (`0040fbf0`,
/// `__thiscall(lock, 0)`) and its release (`0040fba0`), both on `this + 0x20`.
const LOCK_ENTER: u32 = 0x0040_fbf0;
const LOCK_LEAVE: u32 = 0x0040_fba0;
/// Gives the object the wrapper forwards to (`00449f80`, `__thiscall(this)`:
/// `0044d5c0` of the object at `this + 0x14` with `this`).
const MAP_TARGET: u32 = 0x0044_9f80;
/// `tolower(character)` (`00ec67aa`).
const TOLOWER_CHARACTER: u32 = 0x00ec_67aa;
/// `strcmp(first, second)` (`00ec6da0`).
const STRING_COMPARE: u32 = 0x00ec_6da0;
/// Source file name the memory-context guard of [`fn_004492f0`] records, and
/// its context number and line.
const THREAD_SAFE_STRUCTURES_SOURCE: u32 = 0x0101_71a0;
const NAME_COPY_CONTEXT: u32 = 6;
const NAME_COPY_SOURCE_LINE: u32 = 0x1fd;
/// Size of the stack buffer that [`lowercase_into_buffer`] fills (the
/// game's own: 1000 bytes, without a bounds check).
const LOWERCASE_BUFFER_SIZE: u32 = 1000;

/// Runs `body` with the object the wrapper forwards to, between the lock on
/// `this + 0x20` being taken and released.
fn with_locked_target<R>(e: &mut Engine, this: Ptr, body: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.call(LOCK_ENTER, &args![this.addr() + 0x20, 0u32]);
    let target = e.call(MAP_TARGET, &args![this]).u32();
    let result = body(e, target);
    e.call(LOCK_LEAVE, &args![this.addr() + 0x20]);
    result
}

/// Writes `tolower` of each character of `source` and a terminator to
/// `destination` (the loop the game repeats in each wrapper).
fn lowercase_into(e: &mut Engine, source: u32, destination: u32) {
    let (mut from, mut to) = (source, destination);
    loop {
        let character = e.mem.u8(from);
        if character == 0 {
            break;
        }
        // `char` is signed: the character is sign-extended for `tolower`.
        let lowered = e
            .call(TOLOWER_CHARACTER, &args![character as i8 as i32 as u32])
            .u8();
        e.mem.set_u8(to, lowered);
        from += 1;
        to += 1;
    }
    e.mem.set_u8(to, 0);
}

/// Runs `body` with the lower-cased copy of `source` in a 1000-byte buffer
/// on the stack.
fn lowercase_into_buffer<R>(
    e: &mut Engine,
    source: u32,
    body: impl FnOnce(&mut Engine, u32) -> R,
) -> R {
    e.with_stack(LOWERCASE_BUFFER_SIZE, |e, buffer| {
        lowercase_into(e, source, buffer.addr());
        body(e, buffer.addr())
    })
}

// Translated from 004491f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `009611e0(this)` (filed under `playercharacter.cpp` in the
/// engine map) and answers its value: the exe ends on that call, leaving its
/// `EAX` (the callers read it as the task count).
pub fn fn_004491f0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0096_11e0, &args![this]).u32()
}

// Translated from 00449210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the element `index` of the array at `this + 4`, runs
/// [`fn_00449240`] for it with `argument`, then the virtual function at
/// offset 4 of `this`.
pub fn fn_00449210(e: &mut Engine, this: Ptr, index: u32, argument: u32) {
    let elements = e.mem.u32(this.addr() + 4);
    let element = e.mem.u32(elements + index * 4);
    fn_00449240(e, Ptr::new(element), argument);
    e.vcall(this.addr(), 4, &args![]);
}

// Translated from 00449240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the lock at `this + 0x20`: `0044cd40` of the forwarding target
/// ([`MAP_TARGET`]) with `argument`.
pub fn fn_00449240(e: &mut Engine, this: Ptr, argument: u32) {
    with_locked_target(e, this, |e, target| {
        e.call(0x0044_cd40, &args![target, argument]);
    });
}

// Translated from 00449280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the lock at `this + 0x20`: the byte answered by `0044cec0` of the
/// forwarding target with `argument`.
pub fn fn_00449280(e: &mut Engine, this: Ptr, argument: u32) -> u8 {
    with_locked_target(e, this, |e, target| {
        e.call(0x0044_cec0, &args![target, argument]).u8()
    })
}

// Translated from 004492c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor: the base constructor `00449f50(first, second, third)`, then
/// the virtual table `01017154`; returns `this`.
pub fn fn_004492c0(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> Ptr {
    e.call(0x0044_9f50, &args![this, first, second, third]);
    e.mem.set_u32(this.addr(), 0x0101_7154);
    this
}

// Translated from 004492f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A lower-cased copy of the text, in a block from the allocator allocated
/// under the memory context 6 (`ThreadSafeStructures.h` line 0x1fd). The
/// frame's exception handler is not translated; `this` is not read.
pub fn fn_004492f0(e: &mut Engine, _this: Ptr, text: u32) -> u32 {
    e.with_stack(4, |e, scope| {
        e.call(
            MEMORY_CONTEXT_ENTER,
            &args![
                scope,
                NAME_COPY_CONTEXT,
                1u32,
                THREAD_SAFE_STRUCTURES_SOURCE,
                NAME_COPY_SOURCE_LINE
            ],
        );
        let length = e.call(STRLEN, &args![text]).u32();
        let copy = e.call(MEMORY_ALLOC, &args![length + 1]).u32();
        lowercase_into(e, text, copy);
        e.call(MEMORY_CONTEXT_LEAVE, &args![scope]);
        copy
    })
}

// Translated from 004493d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004493d0 - the /GS stack cookie check (00ec408c) is not translated
/// Lower-cases the text into a 1000-byte buffer and runs [`fn_00449530`]
/// with it and `argument`, answering its byte: the exe ends on that call and
/// keeps its `AL` through the cookie check and the epilogue
/// (`ModelLoader::FindModel` 004472a0 tests it).
pub fn fn_004493d0(e: &mut Engine, this: Ptr, text: u32, argument: u32) -> u8 {
    lowercase_into_buffer(e, text, |e, buffer| fn_00449530(e, this, buffer, argument))
}

// Translated from 00449460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00449460 - the /GS stack cookie check (00ec408c) is not translated
/// Lower-cases the text into a 1000-byte buffer and runs [`fn_00449580`]
/// with it, `argument` and `flag`, answering its byte: the exe ends on that
/// call and keeps its `AL` through the cookie check and the epilogue.
pub fn fn_00449460(e: &mut Engine, this: Ptr, text: u32, argument: u32, flag: u8) -> u8 {
    lowercase_into_buffer(e, text, |e, buffer| {
        fn_00449580(e, this, buffer, argument, flag)
    })
}

// Translated from 00449500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strcpy_s(*destination, 0x100, source)`; `this` is not read.
pub fn fn_00449500(e: &mut Engine, _this: Ptr, source: u32, destination: Ptr) {
    let buffer = e.mem.u32(destination.addr());
    e.call(STRING_COPY, &args![buffer, 0x100u32, source]);
}

// Translated from 00449530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the lock at `this + 0x20`: the byte answered by `006ec830` of the
/// forwarding target with `key` and `argument`.
pub fn fn_00449530(e: &mut Engine, this: Ptr, key: u32, argument: u32) -> u8 {
    with_locked_target(e, this, |e, target| {
        e.call(0x006e_c830, &args![target, key, argument]).u8()
    })
}

// Translated from 00449580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the lock at `this + 0x20`: the byte answered by `0044a130` of the
/// forwarding target with `key`, `argument` and `flag`.
pub fn fn_00449580(e: &mut Engine, this: Ptr, key: u32, argument: u32, flag: u8) -> u8 {
    with_locked_target(e, this, |e, target| {
        e.call(0x0044_a130, &args![target, key, argument, u32::from(flag)])
            .u8()
    })
}

// Translated from 004495d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the lock at `this + 0x20`: the byte answered by `00665f20` of the
/// forwarding target with `key`.
pub fn fn_004495d0(e: &mut Engine, this: Ptr, key: u32) -> u8 {
    with_locked_target(e, this, |e, target| {
        e.call(0x0066_5f20, &args![target, key]).u8()
    })
}

// Translated from 00449610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor: the base constructor `00449fd0`, clears the dwords at +0x18
/// and +0x1c, then `0044a0a0`; returns `this`. The frame's exception handler
/// is not translated.
pub fn fn_00449610(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0044_9fd0, &args![this]);
    e.mem.set_u32(this.addr() + 0x18, 0);
    e.mem.set_u32(this.addr() + 0x1c, 0);
    e.call(0x0044_a0a0, &args![this]);
    this
}

// Translated from 00449680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `0044a040(this)`.
pub fn fn_00449680(e: &mut Engine, this: Ptr) {
    e.call(0x0044_a040, &args![this]);
}

// Translated from 004496a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor: the base constructor `00449fa0(first, second, third)`, then
/// the virtual table `010171ec`; returns `this`.
pub fn fn_004496a0(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> Ptr {
    e.call(0x0044_9fa0, &args![this, first, second, third]);
    e.mem.set_u32(this.addr(), 0x0101_71ec);
    this
}

// Translated from 004496d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `strcmp(first, second)` is not negative; `this` is not read.
pub fn fn_004496d0(e: &mut Engine, _this: Ptr, first: u32, second: u32) -> bool {
    e.call(STRING_COMPARE, &args![first, second]).u32() as i32 >= 0
}

// Translated from 00449700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the lock at `this + 0x20`: the byte answered by `0044d3f0` of the
/// forwarding target with `key` and the address of the second argument's
/// slot (the game passes the address of its stack argument).
pub fn fn_00449700(e: &mut Engine, this: Ptr, key: u32, argument: u32) -> u8 {
    with_locked_target(e, this, |e, target| {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), argument);
            e.call(0x0044_d3f0, &args![target, key, slot]).u8()
        })
    })
}

// Translated from 00449750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hash of a name: `hash * 0x21 + tolower(character)` over the characters
/// (a signed `char`), modulo the dword at +8 of `this` (`0044ddc0`, the
/// bucket count); the game divides by it without a check.
pub fn fn_00449750(e: &mut Engine, this: Ptr, text: u32) -> u32 {
    let mut hash: u32 = 0;
    let mut at = text;
    loop {
        let character = e.mem.u8(at);
        if character == 0 {
            break;
        }
        let lowered = e
            .call(TOLOWER_CHARACTER, &args![character as i8 as i32 as u32])
            .u32();
        hash = hash.wrapping_mul(0x21).wrapping_add(lowered);
        at += 1;
    }
    let buckets = e.call(FIELD_AT_8, &args![this]).u32();
    hash % buckets
}

// Translated from 004497c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor: the base constructor `0044a100(first, second, third)`, then
/// the virtual table `0101723c`; returns `this`.
pub fn fn_004497c0(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> Ptr {
    e.call(0x0044_a100, &args![this, first, second, third]);
    e.mem.set_u32(this.addr(), 0x0101_723c);
    this
}

// Translated from 004497f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004497f0 - the /GS stack cookie check (00ec408c) is not translated
/// Lower-cases the text into a 1000-byte buffer and runs [`fn_004495d0`]
/// with it, answering its byte: the exe ends on that call and keeps its `AL`
/// through the cookie check and the epilogue. It is slot `+0x14` of the
/// `LockFreeCaseInsensitiveStringMap` vtables, whose `LockFreeMap` base
/// implementation (`004495d0`) answers that byte too.
pub fn fn_004497f0(e: &mut Engine, this: Ptr, text: u32) -> u8 {
    lowercase_into_buffer(e, text, |e, buffer| fn_004495d0(e, this, buffer))
}

// Translated from 00449880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `strcmp(first, second)` is 0; `this` is not read (the engine map
/// gives it a `CRect::operator!=` name that its body does not have).
pub fn fn_00449880(e: &mut Engine, _this: Ptr, first: u32, second: u32) -> bool {
    e.call(STRING_COMPARE, &args![first, second]).u32() == 0
}

// Translated from 004498b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte answered by `006ec830` of the forwarding target with `key` and
/// `argument`, without the lock.
pub fn fn_004498b0(e: &mut Engine, this: Ptr, key: u32, argument: u32) -> u8 {
    let target = e.call(MAP_TARGET, &args![this]).u32();
    e.call(0x006e_c830, &args![target, key, argument]).u8()
}

// Translated from 004498e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor: the base constructor `0044cb70`, then the virtual table
/// `01017288`, the dword at +8 pointing at the inline byte at +0x10, which
/// is cleared; returns `this`.
pub fn fn_004498e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0044_cb70, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_7288);
    e.mem.set_u32(this.addr() + 8, this.addr() + 0x10);
    e.mem.set_u8(this.addr() + 0x10, 0);
    this
}

// Translated from 00449920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeStringMap<Model_P>::LockFreeStringMapIterator::ClearKey`
/// (Xbox PDB): clears the byte at +0x10.
pub fn lock_free_string_map_iterator_clear_key(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x10, 0);
}

// Translated from 00449940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the virtual table `01017294`.
pub fn fn_00449940(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_7294);
}

// Translated from 00449960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor like [`fn_004498e0`]: base constructor `0044cbb0`, virtual
/// table `010172a0`, the dword at +8 pointing at the inline byte at +0x10,
/// which is cleared; returns `this`.
pub fn fn_00449960(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0044_cbb0, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_72a0);
    e.mem.set_u32(this.addr() + 8, this.addr() + 0x10);
    e.mem.set_u8(this.addr() + 0x10, 0);
    this
}

// Translated from 004499a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the virtual table `010172ac`.
pub fn fn_004499a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_72ac);
}

// Translated from 004499c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the virtual table `010172b8`.
pub fn fn_004499c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_72b8);
}

// Translated from 004499e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the virtual table `010172c4`.
pub fn fn_004499e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_72c4);
}

// Translated from 00449a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the virtual table `010172d0`.
pub fn fn_00449a00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_72d0);
}

/// Frees `this` when bit 0 of the delete flags is set.
fn free_when_flagged(e: &mut Engine, this: Ptr, flags: u32) {
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![this]);
    }
}

// Translated from 00449a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor: [`fn_004490e0`], then frees the block when
/// bit 0 of `flags` is set; returns `this`.
pub fn fn_00449a20(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_004490e0(e, this);
    free_when_flagged(e, this, flags);
    this
}

// Translated from 00449a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor: `00449c30(this)`, then frees the block when
/// bit 0 of `flags` is set; returns `this`.
pub fn fn_00449a50(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_9c30, &args![this]);
    free_when_flagged(e, this, flags);
    this
}

// Translated from 00449a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor: `00449d40(this)`, then frees the block when
/// bit 0 of `flags` is set; returns `this`.
pub fn fn_00449a80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_9d40, &args![this]);
    free_when_flagged(e, this, flags);
    this
}

// Translated from 00449ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor: `00449ea0(this)`, then frees the block when
/// bit 0 of `flags` is set; returns `this`.
pub fn fn_00449ab0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_9ea0, &args![this]);
    free_when_flagged(e, this, flags);
    this
}

// Translated from 00449ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor: `004431d0(this)`, then frees the block when
/// bit 0 of `flags` is set; returns `this`.
pub fn fn_00449ae0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_31d0, &args![this]);
    free_when_flagged(e, this, flags);
    this
}

// Translated from 00449b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor: [`fn_00449940`], then frees the block when
/// bit 0 of `flags` is set; returns `this`.
pub fn fn_00449b10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00449940(e, this);
    free_when_flagged(e, this, flags);
    this
}

// Translated from 00449b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor: `00443220(this)`, then frees the block when
/// bit 0 of `flags` is set; returns `this`.
pub fn fn_00449b40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_3220, &args![this]);
    free_when_flagged(e, this, flags);
    this
}

// Translated from 00449b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor: [`fn_004499a0`], then frees the block when
/// bit 0 of `flags` is set; returns `this`.
pub fn fn_00449b70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_004499a0(e, this);
    free_when_flagged(e, this, flags);
    this
}

// Translated from 00449ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeMap<TESObjectREFR_P_NiPointer<QueuedReference>_>::LockFreeMapIterator::_scalar_deleting_destructor_`
/// (Xbox PDB): [`fn_004499c0`], then frees the block when bit 0 of `flags`
/// is set; returns `this`.
pub fn lock_free_map_reference_iterator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_004499c0(e, this);
    free_when_flagged(e, this, flags);
    this
}

// Translated from 00449bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeMap<AnimIdle_P_NiPointer<QueuedAnimIdle>_>::LockFreeMapIterator::_scalar_deleting_destructor_`
/// (Xbox PDB): [`fn_004499e0`], then frees the block when bit 0 of `flags`
/// is set; returns `this`.
pub fn lock_free_map_anim_idle_iterator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_004499e0(e, this);
    free_when_flagged(e, this, flags);
    this
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004457d0,
            model_loader_queue_creature_parts(Ptr, Ptr, u32, Ptr, Ptr)
        ),
        entry!(0x004459e0, fn_004459e0(Ptr, u32) -> Ptr),
        entry!(
            0x00445a10,
            model_loader_queue_animations(Ptr, Ptr, u32, Ptr, Ptr, u8, u8)
        ),
        entry!(0x00446390, fn_00446390(Ptr) -> i32),
        entry!(0x004463b0, fn_004463b0(Ptr, Ptr, Ptr, u32, Ptr, u32)),
        entry!(0x004464c0, fn_004464c0(Ptr, u32) -> u32),
        entry!(0x00446500, fn_00446500(Ptr, Ptr, u32, Ptr, u32)),
        entry!(0x004465f0, fn_004465f0(Ptr, Ptr, Ptr, u32, Ptr, u32)),
        entry!(0x00446990, fn_00446990(Ptr) -> u32),
        entry!(
            0x004469c0,
            fn_004469c0(Ptr<QueuedReplacementKFList>, Ptr, u32) -> Ptr<QueuedReplacementKFList>
        ),
        entry!(
            0x00446a10,
            queued_replacement_kf_list_scalar_deleting_destructor(
                Ptr<QueuedReplacementKFList>,
                u32,
            )
                -> Ptr<QueuedReplacementKFList>
        ),
        entry!(
            0x00446a40,
            queued_replacement_kf_list_check_finished(Ptr<QueuedReplacementKFList>)
        ),
        entry!(0x00446a60, fn_00446a60(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00446b50, fn_00446b50(Ptr, Ptr)),
        entry!(0x00446c90, fn_00446c90(Ptr) -> u32),
        entry!(0x00446cb0, fn_00446cb0(Ptr)),
        entry!(0x00446da0, fn_00446da0(Ptr) -> u32),
        entry!(0x00446dc0, fn_00446dc0(Ptr) -> u32),
        entry!(0x00446de0, fn_00446de0(Ptr) -> u32),
        entry!(0x00446e10, fn_00446e10() -> u8),
        entry!(0x00446e30, fn_00446e30()),
        entry!(0x00446e40, fn_00446e40() -> bool),
        entry!(0x00446ea0, fn_00446ea0()),
        entry!(0x00446ef0, fn_00446ef0() -> u32),
        entry!(0x00446f00, fn_00446f00() -> u32),
        entry!(0x00446f10, fn_00446f10()),
        entry!(0x00446f20, fn_00446f20(Ptr)),
        entry!(0x00446f40, fn_00446f40(Ptr)),
        entry!(0x00446f70, fn_00446f70(Ptr)),
        entry!(0x00446f90, fn_00446f90()),
        entry!(0x00446fa0, fn_00446fa0(Ptr)),
        entry!(0x00446fc0, fn_00446fc0(Ptr)),
        entry!(0x00446ff0, fn_00446ff0(Ptr)),
        entry!(0x00447010, fn_00447010(Ptr, u32, Ptr) -> bool),
        entry!(0x00447040, fn_00447040(Ptr, u32, Ptr) -> bool),
        entry!(
            0x00447080,
            model_loader_load_file(Ptr, u32, u32, u8, u32, u8, u8) -> Ptr
        ),
        entry!(0x00447190, fn_00447190(Ptr<QueuedModel>, u8)),
        entry!(0x004471c0, model_loader_load_kf(Ptr, u32) -> Ptr),
        entry!(0x004472a0, model_loader_find_model(Ptr, u32, Ptr) -> bool),
        entry!(
            0x00447300,
            model_loader_build_file_list(Ptr, u32, u32, u32) -> u32
        ),
        entry!(
            0x00447330,
            model_loader_build_kf_file_list(Ptr, u32, u8, u8, u32) -> u32
        ),
        entry!(
            0x00447850,
            model_loader_copy_filename_list(Ptr, u32, u32) -> u32
        ),
        entry!(0x00447950, fn_00447950(Ptr, u32) -> bool),
        entry!(0x00447980, fn_00447980(Ptr, u32, u32)),
        entry!(
            0x00447a40,
            model_loader_queue_face_gen_file(Ptr, u32, u32, u32, u32)
        ),
        entry!(
            0x00447bf0,
            model_loader_queue_egm_file(Ptr, Ptr, u32, u32, u32)
        ),
        entry!(
            0x00448080,
            model_loader_queue_tri_file(Ptr, Ptr, u32, u32, u32)
        ),
        entry!(0x00448330, fn_00448330(Ptr, u32, u32) -> bool),
        entry!(0x00448370, fn_00448370(Ptr, u32)),
        entry!(0x004483a0, fn_004483a0(Ptr, u32) -> u32),
        entry!(0x004483e0, fn_004483e0(Ptr, u32) -> u32),
        entry!(0x00448420, fn_00448420(Ptr)),
        entry!(0x00448620, fn_00448620(Ptr, u8)),
        entry!(0x00448920, model_loader_try_and_remove_model(Ptr, u32, u32)),
        entry!(0x004489b0, model_loader_load_addon_nodes(Ptr, Ptr, u32)),
        entry!(0x00448a20, fn_00448a20(Ptr) -> bool),
        entry!(0x00448a40, fn_00448a40(Ptr) -> bool),
        entry!(0x00448a60, fn_00448a60(Ptr, u32) -> u32),
        entry!(0x00448a80, fn_00448a80() -> u32),
        entry!(0x00448a90, model_loader_load_addons(Ptr, u32)),
        entry!(0x00448bf0, fn_00448bf0(Ptr) -> bool),
        entry!(0x00448c10, fn_00448c10(Ptr) -> bool),
        entry!(0x00448cc0, fn_00448cc0(Ptr) -> bool),
        entry!(0x00448de0, fn_00448de0(Ptr) -> bool),
        entry!(0x00448e00, fn_00448e00(Ptr) -> Ptr),
        entry!(0x00448e30, bs_precision_timer_reset(Ptr, f32)),
        entry!(0x00448e70, fn_00448e70(Ptr) -> bool),
        entry!(0x00448ed0, fn_00448ed0(Ptr, u32, u32) -> bool),
        entry!(0x00448f50, fn_00448f50(Ptr, u32)),
        entry!(0x00448f80, fn_00448f80(Ptr, u32) -> u32),
        entry!(0x00449030, fn_00449030(Ptr)),
        entry!(0x00449050, fn_00449050(u32) -> u32),
        entry!(0x00449090, fn_00449090(Ptr) -> Ptr),
        entry!(0x004490e0, fn_004490e0(Ptr)),
        entry!(0x00449110, fn_00449110(Ptr) -> bool),
        entry!(0x00449130, fn_00449130(Ptr) -> bool),
        entry!(0x00449150, fn_00449150(Ptr)),
        entry!(0x00449170, fn_00449170(Ptr)),
        entry!(0x00449190, fn_00449190(Ptr, u32, u32) -> bool),
        entry!(0x004491c0, fn_004491c0(Ptr, u32, u32) -> bool),
        entry!(0x004491f0, fn_004491f0(Ptr) -> u32),
        entry!(0x00449210, fn_00449210(Ptr, u32, u32)),
        entry!(0x00449240, fn_00449240(Ptr, u32)),
        entry!(0x00449280, fn_00449280(Ptr, u32) -> u8),
        entry!(0x004492c0, fn_004492c0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x004492f0, fn_004492f0(Ptr, u32) -> u32),
        entry!(0x004493d0, fn_004493d0(Ptr, u32, u32) -> u8),
        entry!(0x00449460, fn_00449460(Ptr, u32, u32, u8) -> u8),
        entry!(0x00449500, fn_00449500(Ptr, u32, Ptr)),
        entry!(0x00449530, fn_00449530(Ptr, u32, u32) -> u8),
        entry!(0x00449580, fn_00449580(Ptr, u32, u32, u8) -> u8),
        entry!(0x004495d0, fn_004495d0(Ptr, u32) -> u8),
        entry!(0x00449610, fn_00449610(Ptr) -> Ptr),
        entry!(0x00449680, fn_00449680(Ptr)),
        entry!(0x004496a0, fn_004496a0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x004496d0, fn_004496d0(Ptr, u32, u32) -> bool),
        entry!(0x00449700, fn_00449700(Ptr, u32, u32) -> u8),
        entry!(0x00449750, fn_00449750(Ptr, u32) -> u32),
        entry!(0x004497c0, fn_004497c0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x004497f0, fn_004497f0(Ptr, u32) -> u8),
        entry!(0x00449880, fn_00449880(Ptr, u32, u32) -> bool),
        entry!(0x004498b0, fn_004498b0(Ptr, u32, u32) -> u8),
        entry!(0x004498e0, fn_004498e0(Ptr) -> Ptr),
        entry!(0x00449920, lock_free_string_map_iterator_clear_key(Ptr)),
        entry!(0x00449940, fn_00449940(Ptr)),
        entry!(0x00449960, fn_00449960(Ptr) -> Ptr),
        entry!(0x004499a0, fn_004499a0(Ptr)),
        entry!(0x004499c0, fn_004499c0(Ptr)),
        entry!(0x004499e0, fn_004499e0(Ptr)),
        entry!(0x00449a00, fn_00449a00(Ptr)),
        entry!(0x00449a20, fn_00449a20(Ptr, u32) -> Ptr),
        entry!(0x00449a50, fn_00449a50(Ptr, u32) -> Ptr),
        entry!(0x00449a80, fn_00449a80(Ptr, u32) -> Ptr),
        entry!(0x00449ab0, fn_00449ab0(Ptr, u32) -> Ptr),
        entry!(0x00449ae0, fn_00449ae0(Ptr, u32) -> Ptr),
        entry!(0x00449b10, fn_00449b10(Ptr, u32) -> Ptr),
        entry!(0x00449b40, fn_00449b40(Ptr, u32) -> Ptr),
        entry!(0x00449b70, fn_00449b70(Ptr, u32) -> Ptr),
        entry!(
            0x00449ba0,
            lock_free_map_reference_iterator_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00449bd0,
            lock_free_map_anim_idle_iterator_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// Addresses of test doubles that stand in for virtual functions.
    const SLOT_A: u32 = 0x7000_0001;
    const SLOT_B: u32 = 0x7000_0002;
    const SLOT_C: u32 = 0x7000_0003;

    /// An engine with the pages the functions read, and doubles for the
    /// allocator, the string functions, the list helpers and the smart
    /// pointer helpers.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_3000,
            0x0101_6000,
            0x0101_7000,
            0x0118_a000,
            0x0119_7000,
            0x011a_2000,
            0x011a_c000,
            0x011c_3000,
            0x011c_7000,
            0x011c_b000,
            0x011d_d000,
            0x011d_e000,
            0x0120_2000,
        ] {
            e.map(page, 0x1000);
        }
        e.register(MEMORY_ALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MEMORY_FREE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(STRING_COPY, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRING_CONCAT, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register(STRRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            match text.iter().rposition(|c| *c as u32 == a[1]) {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
        e.register(STRNICMP, |e, a| {
            let lower = |text: Vec<u8>| -> Vec<u8> {
                text.iter()
                    .take(a[2] as usize)
                    .map(|c| c.to_ascii_lowercase())
                    .collect()
            };
            u32::from(lower(e.mem.cstr(a[0])) != lower(e.mem.cstr(a[1]))).into_ret()
        });
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(SPRINTF, |e, a| {
            let format = String::from_utf8(e.mem.cstr(a[1])).unwrap();
            let argument = String::from_utf8(e.mem.cstr(a[2])).unwrap();
            let text = format.replacen("%s", &argument, 1);
            e.mem.set_cstr(a[0], text.as_bytes());
            Ret::default()
        });
        e.register(LIST_NODE_ITEM_ADDRESS, |_, a| a[0].into_ret());
        e.register(LIST_NEXT_NODE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let (item, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
        e.register(LIST_DELETE, |_, _| Ret::default());
        e.register(FIELD_AT_8, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(POINTER_GET, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(TASK_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(TASK_POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(QUEUED_FILE_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(MODEL_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[0].into_ret()
        });
        e.register(MEMORY_CONTEXT_ENTER, |_, _| Ret::default());
        e.register(MEMORY_CONTEXT_LEAVE, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.register(LIST_CONSTRUCT, |_, a| a[0].into_ret());
        for (address, value) in [
            (SKELETON_WORD, "Skeleton"),
            (DATA_PREFIX, "Data\\"),
            (MESHES_PREFIX, "Meshes\\"),
            (MTIDLE_FILE, "\\MTIdle.KF"),
            (HOLSTER_FORMAT, "\\%sHolster.KF"),
            (POWER_ARMOR_HOLSTER_FORMAT, "\\PA%sHolster.KF"),
            (DEATH_FILE, "\\Death.KF"),
            (CHILD_IDLE_DIRECTORY, "\\Locomotion\\Child\\IdleAnims"),
            (FEMALE_IDLE_DIRECTORY, "\\Locomotion\\Female\\IdleAnims"),
            (MALE_IDLE_DIRECTORY, "\\Locomotion\\Male\\IdleAnims"),
            (SPECIAL_ANIMS_DIRECTORY, "\\SpecialAnims\\"),
            (DATA_MESHES_DIRECTORY, "Data\\Meshes\\"),
            (KF_WILDCARD, "\\*.KF"),
            (LOCOMOTION_DIRECTORY, "\\Locomotion\\"),
            (LOCOMOTION_KF_WILDCARD, "\\Locomotion\\*.KF"),
            (HURT_DIRECTORY, "Hurt\\"),
            (IDLE_ANIMS_WORD, "IdleAnims"),
            (LOCOMOTION_HURT_KF_WILDCARD, "\\Locomotion\\Hurt\\*.KF"),
        ] {
            e.mem.set_cstr(address, value.as_bytes());
        }
        e
    }

    /// A NUL-terminated copy of `value` in the heap.
    fn text(e: &mut Engine, value: &str) -> u32 {
        let block = e.mem.alloc(value.len() as u32 + 1);
        e.mem.set_cstr(block, value.as_bytes());
        block
    }

    /// Writes a list into `head` (an 8-byte node): the first item inline,
    /// the others in fresh nodes. The empty string is a null item.
    fn build_list(e: &mut Engine, head: u32, items: &[&str]) {
        let mut node = head;
        for (index, item) in items.iter().enumerate() {
            let value = if item.is_empty() { 0 } else { text(e, item) };
            e.mem.set_u32(node, value);
            if index + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    fn make_list(e: &mut Engine, items: &[&str]) -> u32 {
        let head = e.mem.alloc(8);
        build_list(e, head, items);
        head
    }

    /// An object whose virtual table at `table` has the given (byte offset,
    /// function) slots.
    fn object(e: &mut Engine, table: u32, slots: &[(u32, u32)], size: u32) -> u32 {
        let mut words = vec![0u32; 0x90];
        for (offset, target) in slots {
            words[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(table, &words);
        let block = e.mem.alloc(size);
        e.mem.set_u32(block, table);
        block
    }

    /// A double that answers `value` to every call.
    fn constant(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| value.into_ret());
    }

    /// An actor whose virtual functions at the given offsets answer the
    /// given values.
    fn actor(e: &mut Engine, table: u32, answers: &[(u32, u32)]) -> u32 {
        let mut slots = Vec::new();
        for (offset, value) in answers {
            let function = 0x7100_0000 + offset;
            constant(e, function, *value);
            slots.push((*offset, function));
        }
        object(e, table, &slots, 0x20)
    }

    /// Logs the calls the next function makes.
    fn log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A double that records the string at argument `arg` of each call and
    /// returns `result`.
    fn recorder(e: &mut Engine, address: u32, arg: usize, result: u32) -> Rc<RefCell<Vec<String>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(address, move |e, a| {
            sink.borrow_mut()
                .push(String::from_utf8_lossy(&e.mem.cstr(a[arg])).into_owned());
            result.into_ret()
        });
        seen
    }

    // --- 004457d0 -------------------------------------------------------

    #[test]
    fn queue_creature_parts_queues_the_list_and_the_inventory_objects() {
        let mut e = engine();
        let path = text(&mut e, "Creatures\\Rat\\rat.nif");
        let model = object(&mut e, 0x0300_0000, &[(0x14, SLOT_A)], 0x20);
        constant(&mut e, SLOT_A, path);
        let creature = e.mem.alloc(0x200);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            assert_eq!((a[2], a[3]), (SOURCE_TYPE, CREATURE_TYPE));
            creature.into_ret()
        });
        e.register(LIST_HEAD_OF_OBJECT, |_, a| (a[0] + 4).into_ret());
        build_list(&mut e, creature + 0x118, &["Rat.nif"]);
        constant(&mut e, MODEL_ENTRIES, 0);
        let names = recorder(&mut e, MODEL_LOADER_QUEUE_MODEL, 1, 0);
        constant(&mut e, GET_INVENTORY_CHANGES, 0x5000);
        let first = e.mem.alloc(0x20);
        e.mem.set_u32(first + 8, 0x1234);
        constant(&mut e, INVENTORY_FIND_FIRST, first);
        constant(&mut e, INVENTORY_FIND_SECOND, 0);
        e.register(MODEL_LOADER_QUEUE_PARTS, |_, _| Ret::default());
        e.register(INVENTORY_RESULT_DESTRUCT, |_, _| Ret::default());
        let (this, parent, actor) = (0x100, 0x200, 0x300);
        log(&mut e);
        e.call(0x0044_57d0, &args![this, model, 7u32, parent, actor]);
        assert_eq!(*names.borrow(), vec!["Creatures\\Rat\\Rat.nif".to_string()]);
        let find = &calls_to(&e, INVENTORY_FIND_FIRST)[0];
        assert_eq!(
            (find[0], find[1], find[3], find[4]),
            (0x5000, creature, 6, 0)
        );
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_PARTS),
            vec![vec![this, 0x1234, 7, parent, actor]]
        );
        assert_eq!(calls_to(&e, INVENTORY_RESULT_DESTRUCT), vec![vec![first]]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![first]]);
    }

    #[test]
    fn queue_creature_parts_ignores_models_that_are_not_creatures() {
        let mut e = engine();
        let model = object(&mut e, 0x0300_0000, &[], 0x20);
        log(&mut e);
        e.call(0x0044_57d0, &args![0x100u32, model, 7u32, 0u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    // --- 004459e0 -------------------------------------------------------

    #[test]
    fn fn_004459e0_frees_only_with_the_flag() {
        let mut e = engine();
        e.register(INVENTORY_RESULT_DESTRUCT, |_, _| Ret::default());
        let block = e.mem.alloc(8);
        log(&mut e);
        assert_eq!(e.call(0x0044_59e0, &args![block, 0u32]).u32(), block);
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        e.call(0x0044_59e0, &args![block, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![block]]);
        assert_eq!(calls_to(&e, INVENTORY_RESULT_DESTRUCT).len(), 2);
    }

    // --- 00445a10 -------------------------------------------------------

    /// A model object whose virtual function 0x14 gives `path`.
    fn model_with_path(e: &mut Engine, table: u32, path: &str) -> u32 {
        let path = text(e, path);
        constant(e, SLOT_A, path);
        object(e, table, &[(0x14, SLOT_A)], 0x20)
    }

    #[test]
    fn queue_animations_queues_the_skeleton_files_of_an_actor() {
        let mut e = engine();
        let model = model_with_path(&mut e, 0x0300_0000, "Characters\\Skeleton.nif");
        let actor = actor(&mut e, 0x0301_0000, &[(0x22c, 1), (0x100, 1)]);
        let built = recorder(&mut e, BUILD_FILE_LIST, 0, 0);
        let weapon = e.mem.alloc(0x100);
        e.mem.set_u8(weapon + 0xf4, 2);
        constant(&mut e, ACTOR_GET_CURRENT_WEAPON, weapon);
        e.mem.set_u32(HOLSTER_INDEX_TABLE + 8, 1);
        let prefix = text(&mut e, "1HM");
        e.mem.set_u32(HOLSTER_PREFIX_TABLE + 4, prefix);
        e.call(
            0x0044_5a10,
            &args![0x100u32, model, 3u32, 0x200u32, actor, 1u8, 1u8],
        );
        assert_eq!(
            *built.borrow(),
            vec![
                "Data\\Meshes\\Characters\\MTIdle.KF",
                "Data\\Meshes\\Characters\\1HMHolster.KF",
                "Data\\Meshes\\Characters\\PA1HMHolster.KF",
                "Data\\Meshes\\Characters\\Death.KF",
            ]
        );
    }

    #[test]
    fn queue_animations_skeleton_actor_with_another_model_does_nothing() {
        let mut e = engine();
        let model = model_with_path(&mut e, 0x0300_0000, "Characters\\Body.nif");
        let actor = actor(&mut e, 0x0301_0000, &[(0x22c, 1)]);
        log(&mut e);
        e.call(
            0x0044_5a10,
            &args![0x100u32, model, 3u32, 0x200u32, actor, 1u8, 1u8],
        );
        assert!(calls_to(&e, BUILD_FILE_LIST).is_empty());
        assert!(calls_to(&e, RT_DYNAMIC_CAST).is_empty());
    }

    #[test]
    fn queue_animations_queues_the_kf_file_list_of_a_skeleton() {
        let mut e = engine();
        let model = model_with_path(&mut e, 0x0300_0000, "Characters\\Skeleton.nif");
        let list = make_list(&mut e, &["walk.kf"]);
        constant(&mut e, MODEL_LOADER_BUILD_KF_FILE_LIST, list);
        let queued = recorder(&mut e, MODEL_LOADER_QUEUE_KF, 1, 0);
        log(&mut e);
        e.call(
            0x0044_5a10,
            &args![0x100u32, model, 3u32, 0x200u32, 0u32, 1u8, 0u8],
        );
        assert_eq!(*queued.borrow(), vec!["walk.kf".to_string()]);
        let calls = calls_to(&e, MODEL_LOADER_BUILD_KF_FILE_LIST);
        assert_eq!(calls.len(), 1);
        assert_eq!(
            (calls[0][0], calls[0][2..].to_vec()),
            (0x100, vec![1, 1, 0])
        );
        assert_eq!(calls_to(&e, LIST_DELETE), vec![vec![list, 1]]);
        assert_eq!(calls_to(&e, MODEL_LOADER_QUEUE_KF)[0][2..], [3, 0x200]);
    }

    #[test]
    fn queue_animations_queues_the_idle_list_of_a_female_actor() {
        let mut e = engine();
        let model = model_with_path(&mut e, 0x0300_0000, "Characters\\Body.nif");
        let actor = actor(&mut e, 0x0301_0000, &[(0x22c, 0), (0x218, 1), (0x1a0, 0)]);
        constant(&mut e, ACTOR_GET_SEX, 1);
        let roots_path = recorder(&mut e, GET_ROOT_FILENAME_LIST, 1, 0xabc0);
        e.mem.set_u32(LOADER, 0x1111);
        e.mem.set_u32(IDLE_MANAGER, 0x2222);
        e.register_double(MODEL_LOADER_COPY_FILENAME_LIST, |e, a| {
            let item = text(e, "idle.kf");
            e.mem.set_u32(a[2], item);
            Ret::default()
        });
        let queued = recorder(&mut e, MODEL_LOADER_QUEUE_KF, 1, 0);
        log(&mut e);
        e.call(
            0x0044_5a10,
            &args![0x100u32, model, 3u32, 0x200u32, actor, 0u8, 0u8],
        );
        assert_eq!(
            *roots_path.borrow(),
            vec!["Characters\\Locomotion\\Female\\IdleAnims".to_string()]
        );
        let copies = calls_to(&e, MODEL_LOADER_COPY_FILENAME_LIST);
        assert_eq!((copies[0][0], copies[0][1]), (0x1111, 0xabc0));
        assert_eq!(calls_to(&e, GET_ROOT_FILENAME_LIST)[0][0], 0x2222);
        assert_eq!(*queued.borrow(), vec!["idle.kf".to_string()]);
    }

    #[test]
    fn queue_animations_picks_the_child_and_male_idle_directories() {
        for (child, directory) in [
            (1u32, "Characters\\Locomotion\\Child\\IdleAnims"),
            (0, "Characters\\Locomotion\\Male\\IdleAnims"),
        ] {
            let mut e = engine();
            let model = model_with_path(&mut e, 0x0300_0000, "Characters\\Body.nif");
            let actor = actor(
                &mut e,
                0x0301_0000,
                &[(0x22c, 0), (0x218, 1), (0x1a0, child)],
            );
            constant(&mut e, ACTOR_GET_SEX, 0);
            let roots_path = recorder(&mut e, GET_ROOT_FILENAME_LIST, 1, 0);
            e.register(MODEL_LOADER_COPY_FILENAME_LIST, |_, _| Ret::default());
            e.call(
                0x0044_5a10,
                &args![0x100u32, model, 3u32, 0x200u32, actor, 0u8, 0u8],
            );
            assert_eq!(*roots_path.borrow(), vec![directory.to_string()]);
        }
    }

    #[test]
    fn queue_animations_queues_the_special_animations_of_the_cast() {
        let mut e = engine();
        let model = model_with_path(&mut e, 0x0300_0000, "Characters\\Body.nif");
        let actor = actor(&mut e, 0x0301_0000, &[(0x22c, 0), (0x218, 0)]);
        let animation = e.mem.alloc(0x20);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            assert_eq!((a[2], a[3]), (SOURCE_TYPE, ANIMATION_TYPE));
            animation.into_ret()
        });
        constant(&mut e, TESANIMATION_HAS_KF_FILES, 1);
        e.register(LIST_HEAD_OF_OBJECT, |_, a| (a[0] + 4).into_ret());
        build_list(&mut e, animation + 4, &["a.kf", "b.kf"]);
        e.register(LIST_PUSH_BACK, |e, a| {
            // Appends the item whose address is a[1] to the list at a[0].
            let item = e.mem.u32(a[1]);
            let mut node = a[0];
            if e.mem.u32(node) != 0 {
                while e.mem.u32(node + 4) != 0 {
                    node = e.mem.u32(node + 4);
                }
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
            e.mem.set_u32(node, item);
            Ret::default()
        });
        let queued = recorder(&mut e, MODEL_LOADER_QUEUE_KF, 1, 0);
        e.call(
            0x0044_5a10,
            &args![0x100u32, model, 3u32, 0x200u32, actor, 0u8, 0u8],
        );
        assert_eq!(
            *queued.borrow(),
            vec![
                "Characters\\SpecialAnims\\a.kf".to_string(),
                "Characters\\SpecialAnims\\b.kf".to_string()
            ]
        );
    }

    // --- 00446390 .. 00446500 ---------------------------------------------

    #[test]
    fn fn_00446390_reads_the_signed_weapon_type() {
        let mut e = engine();
        let weapon = e.mem.alloc(0x100);
        e.mem.set_u8(weapon + 0xf4, 0xfe);
        assert_eq!(e.call(0x0044_6390, &args![weapon]).i32(), -2);
    }

    #[test]
    fn fn_004463b0_queues_models_with_and_without_entries() {
        let mut e = engine();
        let list = make_list(&mut e, &["a", "", "c"]);
        let plain = recorder(&mut e, MODEL_LOADER_QUEUE_MODEL, 1, 0);
        let with_entry = recorder(&mut e, MODEL_LOADER_QUEUE_MODEL_WITH_ENTRY, 1, 0);
        log(&mut e);
        e.call(
            0x0044_63b0,
            &args![0x100u32, list, 0u32, 5u32, 0x300u32, 0u32],
        );
        assert_eq!(*plain.borrow(), vec!["a".to_string(), "c".to_string()]);
        assert_eq!(
            calls_to(&e, MODEL_LOADER_QUEUE_MODEL)[0][2..],
            [5, 0x300, 0, 1, 0, 0]
        );
        // With entries: the name after the null item takes entry index 2,
        // which is past the end of the two entries.
        let entries = e.mem.alloc(8);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, 2);
        e.mem.set_u32(array + 4, entries);
        e.mem.set_u32(entries, 0xe0);
        e.mem.set_u32(entries + 4, 0xe1);
        let prefix = text(&mut e, "P\\");
        log(&mut e);
        e.call(
            0x0044_63b0,
            &args![0x100u32, list, array, 5u32, 0x300u32, prefix],
        );
        assert_eq!(
            *with_entry.borrow(),
            vec!["P\\a".to_string(), "P\\c".to_string()]
        );
        let calls = calls_to(&e, MODEL_LOADER_QUEUE_MODEL_WITH_ENTRY);
        assert_eq!((calls[0][2], calls[1][2]), (0xe0, 0));
        assert_eq!(calls[0][3..], [5, 0x300, 0, 1, 0, 0]);
    }

    #[test]
    fn fn_004464c0_returns_zero_past_the_count() {
        let mut e = engine();
        let entries = e.mem.alloc(8);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, 2);
        e.mem.set_u32(array + 4, entries);
        e.mem.set_u32(entries + 4, 0xe1);
        assert_eq!(e.call(0x0044_64c0, &args![array, 1u32]).u32(), 0xe1);
        assert_eq!(e.call(0x0044_64c0, &args![array, 2u32]).u32(), 0);
    }

    #[test]
    fn fn_00446500_drains_the_list() {
        let mut e = engine();
        let list = make_list(&mut e, &["x", "y"]);
        let queued = recorder(&mut e, MODEL_LOADER_QUEUE_KF, 1, 0);
        log(&mut e);
        e.call(0x0044_6500, &args![0x100u32, list, 4u32, 0x300u32, 0u32]);
        assert_eq!(*queued.borrow(), vec!["x".to_string(), "y".to_string()]);
        assert_eq!(calls_to(&e, MODEL_LOADER_QUEUE_KF)[0][2..], [4, 0x300]);
        assert_eq!(calls_to(&e, LIST_REMOVE_HEAD).len(), 2);
        assert_eq!(calls_to(&e, MEMORY_FREE).len(), 2);
        assert_eq!(e.mem.u32(list), 0);
    }

    #[test]
    fn fn_00446500_skips_empty_heads_and_applies_the_prefix() {
        let mut e = engine();
        let list = make_list(&mut e, &["", "z"]);
        let prefix = text(&mut e, "Dir\\");
        let queued = recorder(&mut e, MODEL_LOADER_QUEUE_KF, 1, 0);
        e.call(0x0044_6500, &args![0x100u32, list, 4u32, 0x300u32, prefix]);
        assert_eq!(*queued.borrow(), vec!["Dir\\z".to_string()]);
    }

    // --- 004465f0 -------------------------------------------------------

    /// The pieces `004465f0` works with: the loader (maps at +4 and +0x14),
    /// the queued objects' virtual tables and the callee doubles. The
    /// calls that matter are noted in `calls`.
    struct Replacement {
        this: u32,
        animation: u32,
        calls: Rc<RefCell<Vec<&'static str>>>,
    }

    fn replacement(e: &mut Engine, children: u32, known_model: u32) -> Replacement {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let kf_map = object(e, 0x0300_0000, &[(8, SLOT_A)], 0x10);
        let list_map = object(e, 0x0301_0000, &[(8, SLOT_B), (0x10, SLOT_C)], 0x10);
        e.register_double(SLOT_A, move |e, a| {
            if known_model != 0 {
                e.mem.set_u32(a[2], known_model);
            }
            u32::from(known_model != 0).into_ret()
        });
        constant(e, SLOT_B, 0);
        let log = calls.clone();
        e.register_double(SLOT_C, move |_, a| {
            assert_eq!(a[3], 1);
            log.borrow_mut().push("map add");
            Ret::default()
        });
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 4, kf_map);
        e.mem.set_u32(this + 0x14, list_map);
        let animation = e.mem.alloc(0x10);
        let log = calls.clone();
        e.register_double(ANIMATION_KF_LOADED, move |_, _| {
            log.borrow_mut().push("animation takes model");
            Ret::default()
        });
        e.register_double(QUEUED_FILE_CONSTRUCT, move |e, a| {
            let counter = e.mem.alloc(0x10);
            e.mem.set_u32(counter + 8, children);
            e.mem.set_u32(a[0] + 0x20, counter);
            a[0].into_ret()
        });
        let mut words = vec![0u32; 0x40];
        words[8] = SLOT_A + 0x10;
        e.put_vtable(0x0302_0000, &words);
        let log = calls.clone();
        e.register_double(SLOT_A + 0x10, move |_, _| {
            log.borrow_mut().push("kf queued");
            Ret::default()
        });
        e.register_double(QUEUED_REPLACEMENT_KF_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x0302_0000);
            a[0].into_ret()
        });
        let log = calls.clone();
        e.register_double(QUEUED_FILE_SET_PARENT, move |_, _| {
            log.borrow_mut().push("set parent");
            Ret::default()
        });
        let log = calls.clone();
        e.register_double(TASK_SET_DONE, move |_, _| {
            log.borrow_mut().push("state done");
            Ret::default()
        });
        e.mem
            .set_u32(QUEUED_REPLACEMENT_KF_LIST_VTABLE + 0x28, SLOT_B + 0x10);
        let log = calls.clone();
        e.register_double(SLOT_B + 0x10, move |_, _| {
            log.borrow_mut().push("check finished");
            Ret::default()
        });
        Replacement {
            this,
            animation,
            calls,
        }
    }

    #[test]
    fn fn_004465f0_hands_a_loaded_model_to_the_animation() {
        let mut e = engine();
        let r = replacement(&mut e, 1, 0xabc);
        let list = make_list(&mut e, &["a.kf"]);
        log(&mut e);
        e.call(
            0x0044_65f0,
            &args![r.this, list, r.animation, 2u32, 0x300u32, 0u32],
        );
        assert_eq!(*r.calls.borrow(), vec!["animation takes model"]);
        assert_eq!(
            calls_to(&e, ANIMATION_KF_LOADED),
            vec![vec![r.animation, 0xabc]]
        );
        assert_eq!(
            calls_to(&e, MEMORY_CONTEXT_ENTER)[0][1..],
            [0x33, 1, MODEL_LOADER_SOURCE, 0xe9f]
        );
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
        assert_eq!(e.mem.u32(list), 0);
    }

    #[test]
    fn fn_004465f0_queues_a_replacement_list_with_children() {
        let mut e = engine();
        let r = replacement(&mut e, 1, 0);
        let list = make_list(&mut e, &["a.kf", "b.kf"]);
        let prefix = text(&mut e, "Dir\\");
        log(&mut e);
        e.call(
            0x0044_65f0,
            &args![r.this, list, r.animation, 2u32, 0x300u32, prefix],
        );
        assert_eq!(
            *r.calls.borrow(),
            vec![
                "set parent",
                "kf queued",
                "set parent",
                "kf queued",
                "set parent",
                "map add",
                "state done",
                "check finished"
            ]
        );
        // One list was created, with the key as its context.
        let created = calls_to(&e, QUEUED_FILE_CONSTRUCT);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][1], 2);
        let kfs = calls_to(&e, QUEUED_REPLACEMENT_KF_CONSTRUCT);
        assert_eq!(kfs.len(), 2);
        assert_eq!(kfs[0][2..4], [2, r.animation]);
        assert_eq!(kfs[0][4], created[0][0]);
        assert_eq!(calls_to(&e, MEMORY_FREE).len(), 2);
    }

    #[test]
    fn fn_004465f0_drops_a_list_without_children() {
        let mut e = engine();
        let r = replacement(&mut e, 0, 0);
        let list = make_list(&mut e, &["a.kf"]);
        e.call(
            0x0044_65f0,
            &args![r.this, list, r.animation, 2u32, 0x300u32, 0u32],
        );
        assert_eq!(*r.calls.borrow(), vec!["set parent", "kf queued"]);
    }

    #[test]
    fn fn_004465f0_does_nothing_without_a_list_animation_or_item() {
        let mut e = engine();
        let r = replacement(&mut e, 1, 0);
        let empty = make_list(&mut e, &[""]);
        log(&mut e);
        e.call(
            0x0044_65f0,
            &args![r.this, 0u32, r.animation, 2u32, 0u32, 0u32],
        );
        e.call(0x0044_65f0, &args![r.this, empty, 0u32, 2u32, 0u32, 0u32]);
        e.call(
            0x0044_65f0,
            &args![r.this, empty, r.animation, 2u32, 0u32, 0u32],
        );
        assert!(calls_to(&e, MEMORY_CONTEXT_ENTER).is_empty());
    }

    // --- 00446990 .. 00446a60 -----------------------------------------------

    #[test]
    fn fn_00446990_counts_the_children() {
        let mut e = engine();
        let file = e.mem.alloc(0x30);
        assert_eq!(e.call(0x0044_6990, &args![file]).u32(), 0);
        let children = e.mem.alloc(0x10);
        e.mem.set_u32(children + 8, 3);
        e.mem.set_u32(file + 0x20, children);
        assert_eq!(e.call(0x0044_6990, &args![file]).u32(), 3);
    }

    #[test]
    fn fn_004469c0_builds_the_replacement_list() {
        let mut e = engine();
        e.register(QUEUED_FILE_CONSTRUCT, |_, a| a[0].into_ret());
        let list = e.new_object::<QueuedReplacementKFList>();
        e.mem.set_u32(list.addr() + 0x2c, 9);
        e.mem.set_u32(list.addr() + 0x30, 9);
        log(&mut e);
        e.call(0x0044_69c0, &args![list, 0xaa0u32, 0x33u32]);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_CONSTRUCT),
            vec![vec![list.addr(), 0x33]]
        );
        assert_eq!(e.mem.u32(list.addr()), QUEUED_REPLACEMENT_KF_LIST_VTABLE);
        assert_eq!(e.get(list, QueuedReplacementKFList::pAnim), Ptr::new(0xaa0));
        assert_eq!(
            e.get(list, QueuedReplacementKFList::iPostProcessingChildCount),
            0
        );
        assert_eq!(
            e.get(list, QueuedReplacementKFList::iPostProcessedChildCount),
            0
        );
    }

    #[test]
    fn queued_replacement_kf_list_scalar_deleting_destructor_frees_with_the_flag() {
        let mut e = engine();
        e.register(QUEUED_FILE_DESTRUCT, |_, _| Ret::default());
        let list = e.new_object::<QueuedReplacementKFList>();
        log(&mut e);
        assert_eq!(e.call(0x0044_6a10, &args![list, 0u32]).u32(), list.addr());
        assert!(calls_to(&e, MEMORY_FREE).is_empty());
        e.call(0x0044_6a10, &args![list, 1u32]);
        assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![list.addr()]]);
        assert_eq!(calls_to(&e, QUEUED_FILE_DESTRUCT).len(), 2);
    }

    #[test]
    fn queued_replacement_kf_list_check_finished_calls_the_base() {
        let mut e = engine();
        e.register(QUEUED_FILE_CHECK_FINISHED, |_, _| Ret::default());
        log(&mut e);
        e.call(0x0044_6a40, &args![0x1234u32]);
        assert_eq!(calls_to(&e, QUEUED_FILE_CHECK_FINISHED), vec![vec![0x1234]]);
    }

    #[test]
    fn fn_00446a60_chooses_the_model_of_the_form() {
        let mut e = engine();
        let (form, reference, owner) = (0x1000, 0x2000, 0x3000);
        e.register_double(REFERENCE_GET_FORM, move |_, a| {
            assert_eq!(a[0], reference);
            form.into_ret()
        });
        constant(&mut e, REFERENCE_GET_TES_MODEL, 0xaa);
        // The reference's own form: its model.
        assert_eq!(
            e.call(0x0044_6a60, &args![0u32, form, reference]).u32(),
            0xaa
        );
        // A form of type 0x28 (the reference has another form).
        constant(&mut e, REFERENCE_GET_FORM, 0xdead);
        e.register(FORM_TYPE, |_, a| {
            (if a[0] == 0x3000 { 0x2a } else { 0x28 }).into_ret()
        });
        constant(&mut e, WEAPON_GET_MODEL, 0xbb);
        assert_eq!(
            e.call(0x0044_6a60, &args![0u32, form, reference]).u32(),
            0xbb
        );
        // Another form that casts to the model class is itself the model.
        e.register(FORM_TYPE, |_, a| {
            (if a[0] == 0x3000 { 0x2a } else { 0x30 }).into_ret()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| {
            (if a[3] == FORM_MODEL_TYPE { a[0] } else { 0 }).into_ret()
        });
        assert_eq!(
            e.call(0x0044_6a60, &args![0u32, form, reference]).u32(),
            form
        );
        // A biped form takes the sex of an owner of type 0x2a.
        e.register(RT_DYNAMIC_CAST, |_, a| {
            (if a[3] == FORM_BIPED_TYPE { a[0] } else { 0 }).into_ret()
        });
        constant(&mut e, REFERENCE_GET_OWNER, owner);
        constant(&mut e, ACTOR_BASE_GET_SEX, 1);
        e.register(BIPED_GET_WORLD_MODEL, |_, a| (0xcc00 + a[1]).into_ret());
        assert_eq!(
            e.call(0x0044_6a60, &args![0u32, form, reference]).u32(),
            0xcc01
        );
        // Without a reference the sex stays 0; a form of no class gives 0.
        assert_eq!(e.call(0x0044_6a60, &args![0u32, form, 0u32]).u32(), 0xcc00);
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0044_6a60, &args![0u32, form, 0u32]).u32(), 0);
    }

    // --- 00446b50 -------------------------------------------------------

    #[test]
    fn fn_00446b50_requeues_references_with_a_new_priority() {
        let mut e = engine();
        let map = e.mem.alloc(0x10);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, map);
        e.register(MAP_ITERATOR_CONSTRUCT, |_, _| Ret::default());
        e.register(MAP_ITERATOR_DESTRUCT, |_, _| Ret::default());
        let step = Rc::new(Cell::new(0u32));
        let counter = step.clone();
        e.register_double(MAP_ITERATOR_IS_DONE, move |_, _| {
            u32::from(counter.get() >= 2).into_ret()
        });
        // Entry 1: reference 0x10 with a priority 5 task; entry 2:
        // reference 0x20 with a priority 3 task.
        let task_a = object(&mut e, 0x0300_0000, &[(0x1c, SLOT_A)], 0x20);
        let task_b = object(&mut e, 0x0301_0000, &[(0x1c, SLOT_B)], 0x20);
        let counter = step.clone();
        e.register_double(MAP_NEXT_ENTRY, move |e, a| {
            let (reference, task) = if counter.get() == 0 {
                (0x10, task_a)
            } else {
                (0x20, task_b)
            };
            counter.set(counter.get() + 1);
            e.mem.set_u32(a[2], reference);
            e.mem.set_u32(a[3], task);
            1u32.into_ret()
        });
        constant(&mut e, REFERENCE_CELL, 0x7000);
        e.register_double(TASK_PRIORITY, move |_, a| {
            (if a[0] == task_a { 5 } else { 3 }).into_ret()
        });
        e.register(REFERENCE_IS_3D_CRITICAL, |_, a| {
            u32::from(a[0] == 0x10).into_ret()
        });
        e.register_double(GET_CELL_PRIORITY, |_, a| {
            assert_eq!(a[1..], [0x7000, 0]);
            3u32.into_ret()
        });
        e.mem.set_u32(TES_GLOBAL, 0x9999);
        let seen = Rc::new(RefCell::new(Vec::new()));
        for slot in [SLOT_A, SLOT_B] {
            let sink = seen.clone();
            e.register_double(slot, move |_, a| {
                sink.borrow_mut().push((slot, a[1]));
                Ret::default()
            });
        }
        // Task A (priority 5) is in the given cell and critical: wanted 0.
        // Task B is not critical: the cell priority 3 equals its own.
        e.call(0x0044_6b50, &args![this, 0x7000u32]);
        assert_eq!(*seen.borrow(), vec![(SLOT_A, 0)]);
    }

    // --- 00446c90 .. 00446ff0 ---------------------------------------------

    #[test]
    fn fn_00446c90_asks_the_reference_map() {
        let mut e = engine();
        let map = object(&mut e, 0x0300_0000, &[(0x44, SLOT_A)], 0x10);
        constant(&mut e, SLOT_A, 17);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 8, map);
        assert_eq!(e.call(0x0044_6c90, &args![this]).u32(), 17);
    }

    /// The loader's clone thread: id, running count and a queue whose
    /// virtual function 0x10 gives `queued`.
    fn clone_thread(e: &mut Engine, id: u32, running: u32, queued: u32) -> u32 {
        let queue = object(e, 0x0302_0000, &[(0x10, SLOT_C)], 0x10);
        constant(e, SLOT_C, queued);
        let thread = e.mem.alloc(0x40);
        e.mem.set_u32(thread + 8, id);
        e.mem.set_u32(thread + 0x34, running);
        e.mem.set_u32(thread + 0x38, queue);
        thread
    }

    fn status_engine(setting: u8, references: u32, cloning: u32, tasks: u32, post: u32) -> Engine {
        let mut e = engine();
        let scratch = e.mem.alloc(8);
        e.register_double(SETTING_VALUE_POINTER, move |e, a| {
            assert_eq!(a[0], LOADING_SETTING);
            e.mem.set_u8(scratch, setting);
            scratch.into_ret()
        });
        let map = object(&mut e, 0x0300_0000, &[(0x44, SLOT_A)], 0x10);
        constant(&mut e, SLOT_A, references);
        let thread = clone_thread(&mut e, 1, cloning, 0);
        let loader = e.mem.alloc(0x40);
        e.mem.set_u32(loader + 8, map);
        e.mem.set_u32(loader + 0x28, thread);
        constant(&mut e, TASK_MANAGER_TASK_COUNT, tasks);
        let queues = object(&mut e, 0x0303_0000, &[(0xc, SLOT_B)], 0x10);
        constant(&mut e, SLOT_B, post);
        let manager = e.mem.alloc(0x80);
        e.mem.set_u32(manager + 0x64, queues);
        e.mem.set_u32(TASK_MANAGER, manager);
        e.mem.set_u32(LOADER, loader);
        e.register(FORMAT_BUFFER, |_, _| Ret::default());
        e.register(SET_WINDOW_TEXT, |_, _| Ret::default());
        let owner = e.mem.alloc(0x10);
        e.mem.set_u32(owner + 8, 0x4444);
        e.mem.set_u32(WINDOW_OWNER, owner);
        e.mem.set_u32(WINDOW_TEXT, 0x5555);
        e.mem.set_u32(LOADING_SETTING, 0x6666);
        e
    }

    #[test]
    fn fn_00446cb0_formats_the_status_while_work_remains() {
        let mut e = status_engine(0, 4, 3, 2, 1);
        let loader = e.mem.u32(LOADER);
        log(&mut e);
        e.call(0x0044_6cb0, &args![loader]);
        let calls = calls_to(&e, FORMAT_BUFFER);
        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0][1..],
            [PATH_BUFFER_SIZE, LOADING_STATUS_FORMAT, 4, 3, 2, 1]
        );
        assert_eq!(e.mem.u8(STATUS_SHOWN_FLAG), 1);
        assert!(calls_to(&e, SET_WINDOW_TEXT).is_empty());
    }

    #[test]
    fn fn_00446cb0_resets_the_window_text_once() {
        let mut e = status_engine(0, 0, 0, 0, 0);
        let loader = e.mem.u32(LOADER);
        e.mem.set_u8(STATUS_SHOWN_FLAG, 1);
        log(&mut e);
        e.call(0x0044_6cb0, &args![loader]);
        assert_eq!(calls_to(&e, SET_WINDOW_TEXT), vec![vec![0x4444, 0x5555]]);
        assert_eq!(e.mem.u8(STATUS_SHOWN_FLAG), 0);
        e.call(0x0044_6cb0, &args![loader]);
        assert_eq!(calls_to(&e, SET_WINDOW_TEXT).len(), 1);
    }

    #[test]
    fn fn_00446cb0_does_nothing_when_the_setting_is_set() {
        let mut e = status_engine(1, 4, 3, 2, 1);
        let loader = e.mem.u32(LOADER);
        log(&mut e);
        e.call(0x0044_6cb0, &args![loader]);
        assert!(calls_to(&e, FORMAT_BUFFER).is_empty());
        assert_eq!(e.mem.u8(STATUS_SHOWN_FLAG), 0);
    }

    #[test]
    fn fn_00446da0_asks_the_queue_table() {
        let mut e = engine();
        let queues = object(&mut e, 0x0300_0000, &[(0xc, SLOT_A)], 0x10);
        constant(&mut e, SLOT_A, 6);
        let manager = e.mem.alloc(0x80);
        e.mem.set_u32(manager + 0x64, queues);
        assert_eq!(e.call(0x0044_6da0, &args![manager]).u32(), 6);
    }

    #[test]
    fn fn_00446dc0_reads_the_clone_thread_of_the_loader() {
        let mut e = engine();
        let thread = clone_thread(&mut e, 1, 2, 5);
        let loader = e.mem.alloc(0x40);
        e.mem.set_u32(loader + 0x28, thread);
        assert_eq!(e.call(0x0044_6dc0, &args![loader]).u32(), 7);
    }

    #[test]
    fn fn_00446de0_adds_the_running_count_to_the_queue() {
        let mut e = engine();
        let thread = clone_thread(&mut e, 1, 2, 5);
        assert_eq!(e.call(0x0044_6de0, &args![thread]).u32(), 7);
    }

    #[test]
    fn fn_00446e10_reads_the_setting_byte() {
        let mut e = engine();
        e.mem.set_u32(LOADING_SETTING, 0x6666);
        let scratch = e.mem.alloc(8);
        e.mem.set_u8(scratch, 3);
        e.register_double(SETTING_VALUE_POINTER, move |_, a| {
            // 00446e13: the setting object is the static itself, not the word stored there.
            assert_eq!(a[0], LOADING_SETTING);
            scratch.into_ret()
        });
        assert_eq!(e.call(0x0044_6e10, &args![]).u8(), 3);
    }

    #[test]
    fn fn_00446e30_runs_the_status_on_the_loader() {
        let mut e = status_engine(1, 0, 0, 0, 0);
        log(&mut e);
        e.call(0x0044_6e30, &args![]);
        assert_eq!(calls_to(&e, SETTING_VALUE_POINTER).len(), 1);
    }

    #[test]
    fn fn_00446e40_needs_both_answers_and_asks_both() {
        for (first, second) in [(1u32, 1u32), (1, 0), (0, 1), (0, 0)] {
            let mut e = engine();
            e.mem.set_u32(LOADER, 0x1234);
            constant(&mut e, LOADER_CHECK_FIRST, first);
            constant(&mut e, LOADER_CHECK_SECOND, second);
            log(&mut e);
            assert_eq!(
                e.call(0x0044_6e40, &args![]).bool(),
                first == 1 && second == 1
            );
            assert_eq!(calls_to(&e, LOADER_CHECK_FIRST), vec![vec![0x1234]]);
            assert_eq!(calls_to(&e, LOADER_CHECK_SECOND), vec![vec![0x1234]]);
        }
    }

    #[test]
    fn fn_00446ea0_updates_only_when_the_test_holds() {
        for (answer, updates) in [(1u32, 1usize), (0, 0)] {
            let mut e = engine();
            e.mem.set_u32(MAIN_LOOP_OBJECT, 0x777);
            e.register(REFRESH_FIRST, |_, _| Ret::default());
            e.register(REFRESH_SECOND, |_, _| Ret::default());
            constant(&mut e, MAIN_LOOP_TEST, answer);
            e.register(MAIN_LOOP_UPDATE, |_, _| Ret::default());
            log(&mut e);
            e.call(0x0044_6ea0, &args![]);
            assert_eq!(calls_to(&e, REFRESH_FIRST), vec![vec![REFRESH_OBJECT, 0]]);
            assert_eq!(calls_to(&e, REFRESH_SECOND).len(), 1);
            assert_eq!(
                calls_to(&e, MAIN_LOOP_TEST).len(),
                if answer == 1 { 2 } else { 1 }
            );
            assert_eq!(calls_to(&e, MAIN_LOOP_UPDATE).len(), updates);
        }
    }

    #[test]
    fn fn_00446ef0_returns_the_object_address() {
        let mut e = engine();
        assert_eq!(e.call(0x0044_6ef0, &args![]).u32(), 0x011d_effc);
    }

    #[test]
    fn fn_00446f00_reads_the_clone_thread_of_the_global_loader() {
        let mut e = engine();
        let thread = clone_thread(&mut e, 1, 1, 1);
        let loader = e.mem.alloc(0x40);
        e.mem.set_u32(loader + 0x28, thread);
        e.mem.set_u32(LOADER, loader);
        assert_eq!(e.call(0x0044_6f00, &args![]).u32(), 2);
    }

    /// The thread-id calls: the current thread is 9; the clone thread has
    /// `id`.
    fn thread_engine(id: u32) -> (Engine, u32) {
        let mut e = engine();
        let thread = clone_thread(&mut e, id, 0, 0);
        let loader = e.mem.alloc(0x40);
        e.mem.set_u32(loader + 0x28, thread);
        e.mem.set_u32(LOADER, loader);
        constant(&mut e, CURRENT_THREAD_ID, 9);
        for address in [
            CLONE_THREAD_CALL_FIRST,
            CLONE_THREAD_CALL_SECOND,
            CLONE_THREAD_CALL_THIRD,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        (e, thread)
    }

    #[test]
    fn fn_00446f10_and_fn_00446f20_reach_the_thread_through_the_loader() {
        let (mut e, thread) = thread_engine(4);
        let loader = e.mem.u32(LOADER);
        log(&mut e);
        e.call(0x0044_6f10, &args![]);
        e.call(0x0044_6f20, &args![loader]);
        assert_eq!(
            calls_to(&e, CLONE_THREAD_CALL_SECOND),
            vec![vec![thread], vec![thread]]
        );
    }

    #[test]
    fn fn_00446f40_runs_only_from_another_thread() {
        let (mut e, thread) = thread_engine(9);
        log(&mut e);
        e.call(0x0044_6f40, &args![thread]);
        assert!(calls_to(&e, CLONE_THREAD_CALL_FIRST).is_empty());
        e.mem.set_u32(thread + 8, 4);
        e.call(0x0044_6f40, &args![thread]);
        assert_eq!(calls_to(&e, CLONE_THREAD_CALL_FIRST), vec![vec![thread]]);
        assert_eq!(calls_to(&e, CLONE_THREAD_CALL_SECOND), vec![vec![thread]]);
    }

    #[test]
    fn fn_00446f70_calls_both_thread_functions_in_order() {
        let (mut e, thread) = thread_engine(9);
        log(&mut e);
        e.call(0x0044_6f70, &args![thread]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect();
        assert_eq!(
            order,
            vec![
                0x0044_6f70,
                CLONE_THREAD_CALL_FIRST,
                CLONE_THREAD_CALL_SECOND
            ]
        );
    }

    #[test]
    fn fn_00446f90_and_fn_00446fa0_reach_the_thread_through_the_loader() {
        let (mut e, thread) = thread_engine(4);
        let loader = e.mem.u32(LOADER);
        log(&mut e);
        e.call(0x0044_6f90, &args![]);
        e.call(0x0044_6fa0, &args![loader]);
        assert_eq!(
            calls_to(&e, CLONE_THREAD_CALL_THIRD),
            vec![vec![thread], vec![thread]]
        );
    }

    #[test]
    fn fn_00446fc0_runs_only_from_another_thread() {
        let (mut e, thread) = thread_engine(9);
        log(&mut e);
        e.call(0x0044_6fc0, &args![thread]);
        assert!(calls_to(&e, CLONE_THREAD_CALL_THIRD).is_empty());
        e.mem.set_u32(thread + 8, 4);
        e.call(0x0044_6fc0, &args![thread]);
        assert_eq!(calls_to(&e, CLONE_THREAD_CALL_THIRD), vec![vec![thread]]);
    }

    #[test]
    fn fn_00446ff0_calls_both_thread_functions_in_order() {
        let (mut e, thread) = thread_engine(9);
        log(&mut e);
        e.call(0x0044_6ff0, &args![thread]);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect();
        assert_eq!(
            order,
            vec![
                0x0044_6ff0,
                CLONE_THREAD_CALL_FIRST,
                CLONE_THREAD_CALL_THIRD
            ]
        );
    }

    // --- 00447010 .. 00447300 ---------------------------------------------

    #[test]
    fn fn_00447010_and_fn_00447040_add_to_their_maps() {
        let mut e = engine();
        let models = object(&mut e, 0x0300_0000, &[(0x10, SLOT_A)], 0x10);
        let kf_models = object(&mut e, 0x0301_0000, &[(0x10, SLOT_B)], 0x10);
        e.register_double(SLOT_A, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0xbeef);
            assert_eq!(a[3], 0);
            1u32.into_ret()
        });
        e.register_double(SLOT_B, |e, a| {
            assert_eq!(e.mem.u32(a[2]), 0xfeed);
            0u32.into_ret()
        });
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this, models);
        e.mem.set_u32(this + 4, kf_models);
        assert!(e
            .call(0x0044_7010, &args![this, 0x111u32, 0xbeefu32])
            .bool());
        assert!(!e
            .call(0x0044_7040, &args![this, 0x111u32, 0xfeedu32])
            .bool());
    }

    /// A map whose lookup (virtual function 8) answers `answer` (a model,
    /// or 0 for not found).
    fn lookup_map(e: &mut Engine, table: u32, answer: u32) -> u32 {
        let map = object(e, table, &[(8, SLOT_A)], 0x10);
        e.register_double(SLOT_A, move |e, a| {
            if answer != 0 {
                e.mem.set_u32(a[2], answer);
            }
            u32::from(answer != 0).into_ret()
        });
        map
    }

    #[test]
    fn load_file_builds_a_queued_model_when_the_map_has_none() {
        let mut e = engine();
        let map = lookup_map(&mut e, 0x0300_0000, 0);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this, map);
        let model = 0x5000;
        e.register(QUEUED_MODEL_CONSTRUCT, |_, _| Ret::default());
        e.register(QUEUED_MODEL_SET_FLAG_20, |_, _| Ret::default());
        e.register(SET_FLAG_BIT_10, |_, _| Ret::default());
        e.register(QUEUED_MODEL_RUN, |_, _| Ret::default());
        e.register_double(QUEUED_MODEL_FINISH, move |e, a| {
            e.mem.set_u32(a[0] + 0x30, model);
            Ret::default()
        });
        e.register(QUEUED_MODEL_DESTRUCT, |_, _| Ret::default());
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        constant(&mut e, MODEL_OBJECT_3D, 0x6000);
        log(&mut e);
        let name = 0x400;
        let result = e.call(0x0044_7080, &args![this, name, 3u32, 1u8, 0u32, 1u8, 0u8]);
        assert_eq!(result.u32(), 0x6000);
        let construct = calls_to(&e, QUEUED_MODEL_CONSTRUCT);
        assert_eq!(construct[0][1..], [name, 0, 3, 1, 0]);
        let task = construct[0][0];
        assert_eq!(calls_to(&e, QUEUED_MODEL_SET_FLAG_20), vec![vec![task, 1]]);
        assert_eq!(calls_to(&e, SET_FLAG_BIT_10), vec![vec![1, task + 0x3c]]);
        assert_eq!(calls_to(&e, QUEUED_MODEL_DESTRUCT), vec![vec![task]]);
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![model]]);
        assert_eq!(calls_to(&e, MODEL_OBJECT_3D), vec![vec![model]]);
    }

    #[test]
    fn load_file_uses_a_known_model_and_logs_a_zero_reference_total() {
        let mut e = engine();
        let map = lookup_map(&mut e, 0x0300_0000, 0x5000);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this, map);
        constant(&mut e, MODEL_REFERENCE_TOTAL, 0);
        e.register(LOG, |_, _| Ret::default());
        e.register(ADD_REFERENCE, |_, _| Ret::default());
        constant(&mut e, MODEL_OBJECT_3D, 0x6000);
        log(&mut e);
        // `no_reference` set: no extra reference.
        let result = e.call(
            0x0044_7080,
            &args![this, 0x400u32, 3u32, 1u8, 0u32, 1u8, 1u8],
        );
        assert_eq!(result.u32(), 0x6000);
        assert_eq!(
            calls_to(&e, LOG),
            vec![vec![LOAD_FILE_ZERO_REFERENCE_MESSAGE, 0x400]]
        );
        assert!(calls_to(&e, ADD_REFERENCE).is_empty());
        assert!(calls_to(&e, QUEUED_MODEL_CONSTRUCT).is_empty());
    }

    #[test]
    fn load_file_returns_zero_without_a_model() {
        let mut e = engine();
        let map = lookup_map(&mut e, 0x0300_0000, 0);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this, map);
        for address in [
            QUEUED_MODEL_CONSTRUCT,
            QUEUED_MODEL_SET_FLAG_20,
            SET_FLAG_BIT_10,
            QUEUED_MODEL_RUN,
            QUEUED_MODEL_FINISH,
            QUEUED_MODEL_DESTRUCT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        assert_eq!(
            e.call(
                0x0044_7080,
                &args![this, 0x400u32, 3u32, 1u8, 0u32, 1u8, 0u8]
            )
            .u32(),
            0
        );
    }

    #[test]
    fn fn_00447190_sets_bit_10_of_the_flags() {
        let mut e = engine();
        e.register(SET_FLAG_BIT_10, |_, _| Ret::default());
        log(&mut e);
        e.call(0x0044_7190, &args![0x1000u32, 1u8]);
        assert_eq!(calls_to(&e, SET_FLAG_BIT_10), vec![vec![1, 0x1000 + 0x3c]]);
    }

    #[test]
    fn load_kf_builds_a_queued_kf_when_the_map_has_none() {
        let mut e = engine();
        let map = lookup_map(&mut e, 0x0300_0000, 0);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 4, map);
        e.register(QUEUED_KF_CONSTRUCT, |_, _| Ret::default());
        e.register(QUEUED_KF_RUN, |_, _| Ret::default());
        e.register_double(QUEUED_KF_FINISH, |e, a| {
            e.mem.set_u32(a[0] + 0x30, 0x7000);
            Ret::default()
        });
        e.register(QUEUED_KF_DESTRUCT, |_, _| Ret::default());
        e.register(KF_MODEL_ADD_MANUAL_REFERENCE, |_, _| Ret::default());
        log(&mut e);
        assert_eq!(e.call(0x0044_71c0, &args![this, 0x400u32]).u32(), 0x7000);
        let construct = calls_to(&e, QUEUED_KF_CONSTRUCT);
        assert_eq!(construct[0][1..], [0x400, 0]);
        assert_eq!(
            calls_to(&e, QUEUED_KF_DESTRUCT),
            vec![vec![construct[0][0]]]
        );
        assert_eq!(
            calls_to(&e, KF_MODEL_ADD_MANUAL_REFERENCE),
            vec![vec![0x7000]]
        );
    }

    #[test]
    fn load_kf_logs_a_known_model_with_a_zero_reference_total() {
        let mut e = engine();
        let map = lookup_map(&mut e, 0x0300_0000, 0x7000);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 4, map);
        constant(&mut e, KF_MODEL_REFERENCE_TOTAL, 0);
        e.register(LOG, |_, _| Ret::default());
        e.register(KF_MODEL_ADD_MANUAL_REFERENCE, |_, _| Ret::default());
        log(&mut e);
        assert_eq!(e.call(0x0044_71c0, &args![this, 0x400u32]).u32(), 0x7000);
        assert_eq!(
            calls_to(&e, LOG),
            vec![vec![LOAD_KF_ZERO_REFERENCE_MESSAGE, 0x400]]
        );
        assert!(calls_to(&e, QUEUED_KF_CONSTRUCT).is_empty());
    }

    #[test]
    fn load_kf_returns_zero_without_a_model() {
        let mut e = engine();
        let map = lookup_map(&mut e, 0x0300_0000, 0);
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 4, map);
        for address in [
            QUEUED_KF_CONSTRUCT,
            QUEUED_KF_RUN,
            QUEUED_KF_FINISH,
            QUEUED_KF_DESTRUCT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        log(&mut e);
        assert_eq!(e.call(0x0044_71c0, &args![this, 0x400u32]).u32(), 0);
        assert!(calls_to(&e, KF_MODEL_ADD_MANUAL_REFERENCE).is_empty());
    }

    #[test]
    fn find_model_stores_the_model_when_found() {
        let mut e = engine();
        let this = e.mem.alloc(0x30);
        let out = e.mem.alloc(8);
        e.mem.set_u32(out, 0x1111);
        let map = lookup_map(&mut e, 0x0300_0000, 0x5000);
        e.mem.set_u32(this, map);
        assert!(e.call(0x0044_72a0, &args![this, 0x400u32, out]).bool());
        assert_eq!(e.mem.u32(out), 0x5000);
        let map = lookup_map(&mut e, 0x0301_0000, 0);
        e.mem.set_u32(this, map);
        assert!(!e.call(0x0044_72a0, &args![this, 0x400u32, out]).bool());
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn build_file_list_forwards_to_the_builder() {
        let mut e = engine();
        e.register(BUILD_FILE_LIST, |_, _| 0x8000u32.into_ret());
        log(&mut e);
        assert_eq!(
            e.call(0x0044_7300, &args![0x100u32, 0x200u32, 0x300u32, 0u32])
                .u32(),
            0x8000
        );
        assert_eq!(
            calls_to(&e, BUILD_FILE_LIST),
            vec![vec![0x200, 0x300, 1, 0]]
        );
    }

    // --- 00447330 .. 004491c0 (second session) ---------------------------

    type Events = Rc<RefCell<Vec<String>>>;

    fn events() -> Events {
        Rc::new(RefCell::new(Vec::new()))
    }

    /// A double that does nothing.
    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register_double(*address, |_, _| Ret::default());
        }
    }

    /// A task object (table `table`) whose virtual functions `0x20` and
    /// `0x28` note `run` and `cached run` in `events`, with the task's address.
    fn task_object(e: &mut Engine, table: u32, events: &Events) -> u32 {
        let (run, cached) = (table + 0x7000_0000, table + 0x7000_0001);
        for (address, label) in [(run, "run"), (cached, "cached run")] {
            let sink = events.clone();
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push(format!("{label} {:x}", a[0]));
                Ret::default()
            });
        }
        object(e, table, &[(0x20, run), (0x28, cached)], 0x40)
    }

    /// Doubles for the task pointers, parents and slots the face generation
    /// functions use.
    fn queue_doubles(e: &mut Engine) {
        for address in [
            LOADED_FILE_POINTER_CONSTRUCT,
            FIRST_SLOT_CONSTRUCT,
            SECOND_SLOT_CONSTRUCT,
        ] {
            e.register_double(address, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                Ret::default()
            });
        }
        quiet(
            e,
            &[
                LOADED_FILE_POINTER_DESTRUCT,
                FIRST_SLOT_DESTRUCT,
                SECOND_SLOT_DESTRUCT,
                NAME_OBJECT_CONSTRUCT,
                NAME_OBJECT_DESTRUCT,
                QUEUED_FILE_SET_PARENT,
            ],
        );
    }

    /// A loader whose map of loaded files (at +0x24, lookup in virtual
    /// function 8) answers `loaded` (0: not found).
    fn loader_with_files(e: &mut Engine, table: u32, loaded: u32) -> u32 {
        let lookup = table + 0x7000_0000;
        e.register_double(lookup, move |e, a| {
            if loaded != 0 {
                e.mem.set_u32(a[2], loaded);
            }
            u32::from(loaded != 0).into_ret()
        });
        let map = object(e, table, &[(8, lookup)], 0x20);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x24, map);
        this
    }

    // --- 00447330 / 00447850 --------------------------------------------

    /// An idle manager whose lists hold the one file `<index>.kf`.
    fn idle_manager(e: &mut Engine, known: bool) {
        e.mem.set_u32(IDLE_MANAGER, 0x2222);
        e.register_double(GET_ROOT_FILENAME_LIST, move |e, a| {
            if !known {
                return 0u32.into_ret();
            }
            make_list(e, &[&format!("{}.kf", a[2])]).into_ret()
        });
    }

    /// A recorder of the names `005ae3d0` appends: (list, name).
    fn push_back_recorder(e: &mut Engine) -> Rc<RefCell<Vec<(u32, String)>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(LIST_PUSH_BACK, move |e, a| {
            let item = e.mem.u32(a[1]);
            sink.borrow_mut().push((
                a[0],
                String::from_utf8_lossy(&e.mem.cstr(item)).into_owned(),
            ));
            Ret::default()
        });
        seen
    }

    #[test]
    fn build_kf_file_list_merges_the_twelve_root_lists() {
        let mut e = engine();
        idle_manager(&mut e, true);
        let pushed = push_back_recorder(&mut e);
        let path = text(&mut e, "Creatures\\Rat");
        log(&mut e);
        let list = e
            .call(
                0x0044_7330,
                &args![0x100u32, path, 0u32, 0u32, ROOT_LIST_COUNT],
            )
            .u32();
        let roots = calls_to(&e, GET_ROOT_FILENAME_LIST);
        assert_eq!(roots.len(), 13);
        assert_eq!(roots[1][2], 0);
        assert_eq!(roots[12][2], 11);
        let pushed = pushed.borrow();
        assert_eq!(pushed.len(), 12);
        assert!(pushed.iter().all(|(destination, _)| *destination == list));
        assert_eq!(pushed[11].1, "11.kf");
        assert_eq!(
            calls_to(&e, MEMORY_CONTEXT_ENTER)[0][1..],
            [0x33, 1, MODEL_LOADER_SOURCE, 0x108e]
        );
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
    }

    #[test]
    fn build_kf_file_list_appends_the_creature_list_to_the_first() {
        let mut e = engine();
        idle_manager(&mut e, true);
        let pushed = push_back_recorder(&mut e);
        let path = text(&mut e, "Creatures\\Rat");
        log(&mut e);
        let list = e
            .call(0x0044_7330, &args![0x100u32, path, 0u32, 1u32, 5u32])
            .u32();
        let roots = calls_to(&e, GET_ROOT_FILENAME_LIST);
        assert_eq!(
            roots.iter().map(|r| r[2]).collect::<Vec<_>>(),
            vec![0, 0, 5]
        );
        let pushed = pushed.borrow();
        assert_eq!(
            *pushed,
            vec![(list, "0.kf".to_string()), (list, "5.kf".to_string())]
        );
    }

    #[test]
    fn build_kf_file_list_copies_only_the_creature_list_without_the_flag() {
        let mut e = engine();
        idle_manager(&mut e, true);
        let pushed = push_back_recorder(&mut e);
        let path = text(&mut e, "Creatures\\Rat");
        for (flag, creature) in [(0u32, 5u32), (1, 0)] {
            pushed.borrow_mut().clear();
            e.call(0x0044_7330, &args![0x100u32, path, 0u32, flag, creature]);
            assert_eq!(pushed.borrow().len(), 1);
            assert_eq!(pushed.borrow()[0].1, format!("{creature}.kf"));
        }
    }

    /// The doubles of the disk branch: the file list builder records the
    /// pattern and the name it is given and answers a list with one file.
    fn disk_doubles(e: &mut Engine) -> Events {
        let seen = events();
        let sink = seen.clone();
        e.register_double(BUILD_FILE_LIST, move |e, a| {
            sink.borrow_mut().push(format!(
                "{} | {} | {}",
                String::from_utf8_lossy(&e.mem.cstr(a[0])),
                String::from_utf8_lossy(&e.mem.cstr(a[1])),
                if a[3] == 0 { "new list" } else { "appended" }
            ));
            make_list(e, &["found.kf"]).into_ret()
        });
        e.register_double(SIMPLE_LIST_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        quiet(e, &[ADD_ROOT_IDLE_ARRAY]);
        seen
    }

    #[test]
    fn build_kf_file_list_searches_the_disk_without_an_idle_manager() {
        let mut e = engine();
        e.mem.set_u32(IDLE_MANAGER, 0);
        let seen = disk_doubles(&mut e);
        let path = text(&mut e, "Creatures\\Rat");
        log(&mut e);
        let list = e
            .call(0x0044_7330, &args![0x100u32, path, 0u32, 0u32, 0u32])
            .u32();
        assert_eq!(
            *seen.borrow(),
            vec!["Data\\Meshes\\Creatures\\*.KF | Creatures\\Rat | new list".to_string()]
        );
        let added = calls_to(&e, ADD_ROOT_IDLE_ARRAY);
        assert_eq!(added, vec![vec![0, path, list, 0]]);
    }

    #[test]
    fn build_kf_file_list_searches_the_meshes_directory_for_a_bare_name() {
        // The prefix ends with a backslash, so the search pattern is always
        // found; a name without one has no locomotion directory to add.
        let mut e = engine();
        e.mem.set_u32(IDLE_MANAGER, 0);
        let seen = disk_doubles(&mut e);
        let path = text(&mut e, "Rat");
        log(&mut e);
        let list = e
            .call(0x0044_7330, &args![0x100u32, path, 1u32, 0u32, 0u32])
            .u32();
        assert_eq!(
            *seen.borrow(),
            vec!["Data\\Meshes\\*.KF | Rat | new list".to_string()]
        );
        assert_eq!(
            calls_to(&e, ADD_ROOT_IDLE_ARRAY),
            vec![vec![0, path, list, 0]]
        );
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
    }

    #[test]
    fn build_kf_file_list_adds_the_locomotion_directories() {
        let mut e = engine();
        e.mem.set_u32(IDLE_MANAGER, 0);
        let seen = disk_doubles(&mut e);
        let path = text(&mut e, "Characters\\Male\\Skeleton");
        log(&mut e);
        let list = e
            .call(0x0044_7330, &args![0x100u32, path, 1u32, 0u32, 0u32])
            .u32();
        assert_eq!(
            *seen.borrow(),
            vec![
                "Data\\Meshes\\Characters\\Male\\*.KF | Characters\\Male\\Skeleton | new list",
                "Data\\Meshes\\Characters\\Male\\Locomotion\\*.KF | Characters\\Male\\Locomotion\\ | appended",
                "Data\\Meshes\\Characters\\Male\\Locomotion\\Hurt\\*.KF | Characters\\Male\\Locomotion\\Hurt\\IdleAnims | new list",
            ]
        );
        let added = calls_to(&e, ADD_ROOT_IDLE_ARRAY);
        assert_eq!(added.len(), 2);
        // The hurt list is handed over with 1, emptied and deleted; the main
        // list with 0 is returned.
        assert_eq!(added[0][3], 1);
        assert_eq!(added[1], vec![0, path, list, 0]);
        assert_eq!(calls_to(&e, LIST_DELETE), vec![vec![added[0][2], 1]]);
        assert_eq!(calls_to(&e, MEMORY_FREE).len(), 1);
    }

    #[test]
    fn copy_filename_list_makes_a_list_when_none_is_given() {
        let mut e = engine();
        let pushed = push_back_recorder(&mut e);
        let source = make_list(&mut e, &["a.kf", "b.kf"]);
        let list = e.call(0x0044_7850, &args![0x100u32, source, 0u32]).u32();
        assert_ne!(list, 0);
        assert_eq!(
            *pushed.borrow(),
            vec![(list, "a.kf".to_string()), (list, "b.kf".to_string())]
        );
        // Into an existing list, nothing is created.
        pushed.borrow_mut().clear();
        let source = make_list(&mut e, &["c.kf"]);
        assert_eq!(
            e.call(0x0044_7850, &args![0x100u32, source, 0x9000u32])
                .u32(),
            0x9000
        );
        assert_eq!(*pushed.borrow(), vec![(0x9000, "c.kf".to_string())]);
    }

    // --- 00447950 / 00447980 ---------------------------------------------

    #[test]
    fn fn_00447950_needs_a_positive_count() {
        let mut e = engine();
        let counts = Rc::new(Cell::new(0i32));
        let shared = counts.clone();
        e.register_double(OBJECT_POSITIVE_COUNT, move |_, _| {
            (shared.get() as u32).into_ret()
        });
        assert!(!e.call(0x0044_7950, &args![0u32, 0u32]).bool());
        counts.set(1);
        assert!(e.call(0x0044_7950, &args![0u32, 0x500u32]).bool());
        counts.set(0);
        assert!(!e.call(0x0044_7950, &args![0u32, 0x500u32]).bool());
        counts.set(-3);
        assert!(!e.call(0x0044_7950, &args![0u32, 0x500u32]).bool());
    }

    #[test]
    fn fn_00447980_builds_and_starts_a_queued_file() {
        let mut e = engine();
        let seen = events();
        let task = task_object(&mut e, 0x0300_3000, &seen);
        constant(&mut e, QUEUED_NAMED_FILE_CONSTRUCT, task);
        log(&mut e);
        e.call(0x0044_7980, &args![0x100u32, 0x4000u32, 0x77u32]);
        let built = &calls_to(&e, QUEUED_NAMED_FILE_CONSTRUCT)[0];
        assert_eq!(built[1..], [0x4000, 4, 0x77]);
        assert_eq!(*seen.borrow(), vec![format!("run {task:x}")]);
        assert_eq!(calls_to(&e, TASK_POINTER_DESTRUCT).len(), 1);
    }

    // --- 00447a40 ---------------------------------------------------------

    #[test]
    fn queue_face_gen_file_builds_a_file_when_none_is_loaded() {
        let mut e = engine();
        queue_doubles(&mut e);
        let seen = events();
        let task = task_object(&mut e, 0x0300_3000, &seen);
        constant(&mut e, QUEUED_FACE_GEN_FROM_NAME, task);
        let this = loader_with_files(&mut e, 0x0300_4000, 0);
        log(&mut e);
        e.call(0x0044_7a40, &args![this, 0x4000u32, 5u32, 0x300u32, 1u32]);
        assert_eq!(
            calls_to(&e, QUEUED_FACE_GEN_FROM_NAME)[0][1..],
            [0x4000, 5, 1]
        );
        assert_eq!(*seen.borrow(), vec![format!("run {task:x}")]);
        assert_eq!(
            calls_to(&e, QUEUED_FILE_SET_PARENT),
            vec![vec![task, 0x300]]
        );
        assert_eq!(calls_to(&e, LOADED_FILE_POINTER_DESTRUCT).len(), 1);
    }

    #[test]
    fn queue_face_gen_file_uses_the_loaded_file_when_there_is_a_parent() {
        let mut e = engine();
        queue_doubles(&mut e);
        let seen = events();
        let task = task_object(&mut e, 0x0300_3000, &seen);
        constant(&mut e, QUEUED_FACE_GEN_FROM_FILE, task);
        let this = loader_with_files(&mut e, 0x0300_4000, 0x6000);
        log(&mut e);
        e.call(0x0044_7a40, &args![this, 0x4000u32, 5u32, 0x300u32, 1u32]);
        assert_eq!(calls_to(&e, QUEUED_FACE_GEN_FROM_FILE)[0][1..], [0x6000, 5]);
        assert_eq!(*seen.borrow(), vec![format!("cached run {task:x}")]);
        assert!(calls_to(&e, QUEUED_FACE_GEN_FROM_NAME).is_empty());
    }

    #[test]
    fn queue_face_gen_file_does_nothing_for_a_loaded_file_without_parent() {
        let mut e = engine();
        queue_doubles(&mut e);
        let this = loader_with_files(&mut e, 0x0300_4000, 0x6000);
        log(&mut e);
        e.call(0x0044_7a40, &args![this, 0x4000u32, 5u32, 0u32, 1u32]);
        assert!(calls_to(&e, MEMORY_ALLOC).is_empty());
        assert_eq!(calls_to(&e, LOADED_FILE_POINTER_DESTRUCT).len(), 1);
    }

    // --- 00447bf0 / 00448080 ----------------------------------------------

    /// What the face generation functions need: the cache answers `hit`
    /// (with slots whose second object has `+8` set to `field`), the EGM and
    /// TRI name objects point at texts, and a model answers its path.
    struct FaceGen {
        this: u32,
        model: u32,
        pair_task: u32,
        name_task: u32,
        seen: Events,
        names: Events,
    }

    fn face_gen(e: &mut Engine, cache: u32, hit: bool, field: u32) -> FaceGen {
        queue_doubles(e);
        let seen = events();
        let pair_task = task_object(e, 0x0300_3000, &seen);
        let name_task = task_object(e, 0x0300_5000, &seen);
        constant(e, QUEUED_FACE_GEN_FROM_PAIR, pair_task);
        constant(e, QUEUED_FACE_GEN_FROM_NAME, name_task);
        constant(e, FACE_GEN_GET_MODEL_CACHE, cache);
        let first_object = e.mem.alloc(0x10);
        let second_object = e.mem.alloc(0x10);
        e.mem.set_u32(second_object + 8, field);
        e.register_double(FACE_GEN_CACHE_LOOKUP, move |e, a| {
            if hit {
                e.mem.set_u32(a[2], first_object);
                e.mem.set_u32(a[3], second_object);
            }
            u32::from(hit).into_ret()
        });
        let names = events();
        for (address, name) in [
            (FACE_GEN_GET_AS_EGM_FILE, "EGM"),
            (FACE_GEN_GET_AS_TRI_FILE, "TRI"),
        ] {
            let sink = names.clone();
            e.register_double(address, move |e, a| {
                let file = text(e, &format!("{name}{}", a[2] as i32));
                e.mem.set_u32(a[0], file);
                sink.borrow_mut().push(format!(
                    "{name} {} {}",
                    String::from_utf8_lossy(&e.mem.cstr(a[1])),
                    a[2] as i32
                ));
                Ret::default()
            });
        }
        let path = text(e, "Characters\\Head.nif");
        constant(e, 0x7300_0000, path);
        let model = object(e, 0x0300_6000, &[(0x14, 0x7300_0000)], 0x20);
        let this = loader_with_files(e, 0x0300_4000, 0);
        FaceGen {
            this,
            model,
            pair_task,
            name_task,
            seen,
            names,
        }
    }

    #[test]
    fn queue_egm_file_queues_the_cached_pair() {
        let mut e = engine();
        let g = face_gen(&mut e, 0x5000, true, 1);
        log(&mut e);
        e.call(0x0044_7bf0, &args![g.this, g.model, 5u32, 0x300u32, 3u32]);
        assert_eq!(
            *g.names.borrow(),
            vec!["EGM Meshes\\Characters\\Head.nif -1"]
        );
        assert_eq!(
            *g.seen.borrow(),
            vec![format!("cached run {:x}", g.pair_task)]
        );
        assert_eq!(calls_to(&e, QUEUED_FACE_GEN_FROM_PAIR).len(), 1);
        assert_eq!(calls_to(&e, NAME_OBJECT_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&e, FIRST_SLOT_DESTRUCT).len(), 1);
        assert_eq!(calls_to(&e, SECOND_SLOT_DESTRUCT).len(), 1);
    }

    #[test]
    fn queue_egm_file_queues_both_sides_for_kind_nine() {
        let mut e = engine();
        let g = face_gen(&mut e, 0x5000, true, 1);
        log(&mut e);
        e.call(0x0044_7bf0, &args![g.this, g.model, 5u32, 0x300u32, 9u32]);
        assert_eq!(
            *g.names.borrow(),
            vec![
                "EGM Meshes\\Characters\\Head.nif 0",
                "EGM Meshes\\Characters\\Head.nif 1"
            ]
        );
        assert_eq!(g.seen.borrow().len(), 2);
        assert_eq!(calls_to(&e, NAME_OBJECT_DESTRUCT).len(), 2);
        assert_eq!(calls_to(&e, FIRST_SLOT_DESTRUCT).len(), 1);
    }

    #[test]
    fn queue_egm_file_falls_back_to_the_name_without_a_cache_hit() {
        for (cache, hit, field) in [(0u32, true, 1u32), (0x5000, false, 1), (0x5000, true, 0)] {
            let mut e = engine();
            let g = face_gen(&mut e, cache, hit, field);
            log(&mut e);
            e.call(0x0044_7bf0, &args![g.this, g.model, 5u32, 0x300u32, 3u32]);
            let built = &calls_to(&e, QUEUED_FACE_GEN_FROM_NAME)[0];
            assert_eq!(built[2..], [5, 1]);
            assert_eq!(e.mem.cstr(built[1]), b"EGM-1".to_vec());
            assert_eq!(*g.seen.borrow(), vec![format!("run {:x}", g.name_task)]);
            assert!(calls_to(&e, QUEUED_FACE_GEN_FROM_PAIR).is_empty());
        }
    }

    #[test]
    fn queue_egm_file_without_a_parent_queues_nothing_for_a_hit() {
        let mut e = engine();
        let g = face_gen(&mut e, 0x5000, true, 1);
        log(&mut e);
        e.call(0x0044_7bf0, &args![g.this, g.model, 5u32, 0u32, 3u32]);
        assert!(g.seen.borrow().is_empty());
        assert!(calls_to(&e, QUEUED_FACE_GEN_FROM_PAIR).is_empty());
    }

    #[test]
    fn queue_tri_file_queues_the_cached_pair() {
        let mut e = engine();
        // The second object's +8 is not needed here.
        let g = face_gen(&mut e, 0x5000, true, 0);
        log(&mut e);
        e.call(0x0044_8080, &args![g.this, g.model, 5u32, 0x300u32, 0u32]);
        assert_eq!(
            *g.names.borrow(),
            vec!["EGM Meshes\\Characters\\Head.nif -1"]
        );
        assert_eq!(
            *g.seen.borrow(),
            vec![format!("cached run {:x}", g.pair_task)]
        );
        assert_eq!(calls_to(&e, NAME_OBJECT_DESTRUCT).len(), 1);
    }

    #[test]
    fn queue_tri_file_falls_back_to_the_tri_name() {
        for (cache, hit) in [(0u32, true), (0x5000, false)] {
            let mut e = engine();
            let g = face_gen(&mut e, cache, hit, 1);
            log(&mut e);
            e.call(0x0044_8080, &args![g.this, g.model, 5u32, 0x300u32, 0u32]);
            assert_eq!(
                *g.names.borrow(),
                vec![
                    "EGM Meshes\\Characters\\Head.nif -1",
                    "TRI Meshes\\Characters\\Head.nif -1"
                ]
            );
            let built = &calls_to(&e, QUEUED_FACE_GEN_FROM_NAME)[0];
            assert_eq!(e.mem.cstr(built[1]), b"TRI-1".to_vec());
            assert_eq!(*g.seen.borrow(), vec![format!("run {:x}", g.name_task)]);
            assert_eq!(calls_to(&e, NAME_OBJECT_DESTRUCT).len(), 2);
        }
    }

    // --- 00448330 .. 004483e0 ---------------------------------------------

    #[test]
    fn the_map_forwarders_call_the_maps_virtual_functions() {
        let mut e = engine();
        let seen = events();
        for (slot, label) in [(0x10u32, "ten"), (0x14, "fourteen")] {
            let sink = seen.clone();
            let address = 0x7400_0000 + slot;
            e.register_double(address, move |e, a| {
                let extra = if a.len() > 2 { e.mem.u32(a[2]) } else { 0 };
                sink.borrow_mut().push(format!("{label} {} {extra}", a[1]));
                1u32.into_ret()
            });
        }
        let map = object(
            &mut e,
            0x0300_7000,
            &[(0x10, 0x7400_0010), (0x14, 0x7400_0014)],
            0x20,
        );
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x24, map);
        assert!(e.call(0x0044_8330, &args![this, 7u32, 9u32]).bool());
        e.call(0x0044_8370, &args![this, 8u32]);
        assert_eq!(*seen.borrow(), vec!["ten 7 9", "fourteen 8 0"]);
    }

    #[test]
    fn the_file_lookups_answer_zero_when_not_found() {
        let mut e = engine();
        for (address, offset) in [(0x0044_83a0u32, 0x24u32), (0x0044_83e0, 4)] {
            let this = loader_with_files(&mut e, 0x0300_8000 + offset * 0x1000, 0x6000);
            if offset == 4 {
                let map = e.mem.u32(this + 0x24);
                e.mem.set_u32(this + 4, map);
            }
            assert_eq!(e.call(address, &args![this, 0x4000u32]).u32(), 0x6000);
            let empty = loader_with_files(&mut e, 0x0300_9000 + offset * 0x1000, 0);
            if offset == 4 {
                let map = e.mem.u32(empty + 0x24);
                e.mem.set_u32(empty + 4, map);
            }
            assert_eq!(e.call(address, &args![empty, 0x4000u32]).u32(), 0);
        }
    }

    // --- 00448420 / 00448620 / 00448920 ------------------------------------

    /// Iterator doubles shared by the loops over the loader's maps: the
    /// iterator's first dword is the map it walks (set by the constructor
    /// double); `entries` gives the (key, value) pairs each map yields.
    fn map_doubles(
        e: &mut Engine,
        constructors: &[(u32, u32)],
        next_entry: u32,
        entries: Vec<(u32, Vec<(u32, u32)>)>,
    ) {
        for (address, tag) in constructors {
            let tag = *tag;
            e.register_double(*address, move |e, a| {
                e.mem.set_u32(a[0], tag);
                Ret::default()
            });
        }
        let pending = Rc::new(RefCell::new(entries));
        let queues = pending.clone();
        e.register_double(MAP_ITERATOR_IS_DONE, move |e, a| {
            let tag = e.mem.u32(a[0]);
            let queues = queues.borrow();
            let left = queues
                .iter()
                .find(|(t, _)| *t == tag)
                .map_or(0, |q| q.1.len());
            u32::from(left == 0).into_ret()
        });
        e.register_double(next_entry, move |e, a| {
            let tag = e.mem.u32(a[1]);
            let mut queues = pending.borrow_mut();
            let queue = &mut queues.iter_mut().find(|(t, _)| *t == tag).unwrap().1;
            let (key, value) = queue.remove(0);
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            1u32.into_ret()
        });
    }

    #[test]
    fn fn_00448420_cancels_the_tasks_of_the_three_maps() {
        let mut e = engine();
        e.mem.set_u32(TASK_MANAGER, 0x1200);
        let manager = object(
            &mut e,
            0x0300_a000,
            &[(TASK_MANAGER_FLUSH_SLOT, 0x7500_0000)],
            0x20,
        );
        e.mem.set_u32(TASK_MANAGER, manager);
        quiet(
            &mut e,
            &[
                TASK_MANAGER_BEFORE_CANCEL,
                TASK_MANAGER_AFTER_CANCEL,
                TASK_MANAGER_CANCEL_TASK,
                LOADER_THIRD_CLEAR,
                0x7500_0000,
                MAP_ITERATOR_DESTRUCT,
                MAP_ITERATOR_DESTRUCT_SECOND,
                MAP_ITERATOR_DESTRUCT_THIRD,
            ],
        );
        e.register_double(TASK_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        // MAP_NEXT_ENTRY writes the task through the fourth word (the
        // smart pointer), so the shared double is told the maps by tag.
        map_doubles(
            &mut e,
            &[
                (MAP_ITERATOR_CONSTRUCT, 1),
                (MAP_ITERATOR_CONSTRUCT_SECOND, 2),
                (MAP_ITERATOR_CONSTRUCT_THIRD, 3),
            ],
            MAP_NEXT_ENTRY,
            vec![
                (1, vec![(0, 0x71), (0, 0x72)]),
                (2, vec![(0, 0x73)]),
                (3, vec![]),
            ],
        );
        let this = e.mem.alloc(0x40);
        for (offset, map) in [(8, 0x81u32), (0xc, 0x82), (0x10, 0x83), (0x18, 0x84)] {
            e.mem.set_u32(this + offset, map);
        }
        log(&mut e);
        e.call(0x0044_8420, &args![this]);
        let cancelled: Vec<Vec<u32>> = calls_to(&e, TASK_MANAGER_CANCEL_TASK);
        assert_eq!(
            cancelled,
            vec![
                vec![manager, 0x71, 0],
                vec![manager, 0x72, 0],
                vec![manager, 0x73, 0]
            ]
        );
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| {
                [
                    TASK_MANAGER_BEFORE_CANCEL,
                    LOADER_THIRD_CLEAR,
                    0x7500_0000,
                    TASK_MANAGER_AFTER_CANCEL,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                TASK_MANAGER_BEFORE_CANCEL,
                LOADER_THIRD_CLEAR,
                0x7500_0000,
                TASK_MANAGER_AFTER_CANCEL
            ]
        );
        assert_eq!(calls_to(&e, LOADER_THIRD_CLEAR), vec![vec![0x82, 0]]);
    }

    /// A loader with a model map (virtual function `0x14` notes the key it
    /// removes) at +0 and a KF map at +4.
    fn cleanup_loader(e: &mut Engine, removed: &Events) -> u32 {
        let this = e.mem.alloc(0x40);
        for (offset, table, label) in [(0u32, 0x0300_b000u32, "model"), (4, 0x0300_c000, "kf")] {
            let address = table + 0x7000_0000;
            let sink = removed.clone();
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push(format!("{label} {}", a[1]));
                Ret::default()
            });
            let map = object(e, table, &[(0x14, address)], 0x20);
            e.mem.set_u32(this + offset, map);
        }
        this
    }

    #[test]
    fn fn_00448620_removes_the_unused_models_and_kf_models() {
        let mut e = engine();
        let removed = events();
        let this = cleanup_loader(&mut e, &removed);
        e.mem.set_u32(this + 0x2c, 0xffff_ffff);
        let totals = |model: u32| u32::from(matches!(model, 0x62 | 0x75));
        e.register_double(MODEL_REFERENCE_TOTAL, move |_, a| totals(a[0]).into_ret());
        e.register_double(KF_MODEL_REFERENCE_TOTAL, move |_, a| {
            totals(a[0]).into_ret()
        });
        e.register_double(KF_MODEL_OWNER, |_, a| {
            match a[0] {
                0x72 => 0x500u32,
                0x73 => 0x501,
                0x74 => 0x502,
                _ => 0,
            }
            .into_ret()
        });
        e.register_double(KF_MODEL_OWNER_NUMBER, |_, a| {
            match a[0] {
                0x500 => 0x5cu32,
                0x501 => 0x66,
                _ => 0x50,
            }
            .into_ret()
        });
        quiet(
            &mut e,
            &[
                MODEL_ITERATOR_DESTRUCT,
                KF_ITERATOR_DESTRUCT,
                MODEL_RELEASE,
                KF_MODEL_RELEASE,
            ],
        );
        map_doubles(
            &mut e,
            &[(MODEL_ITERATOR_CONSTRUCT, 1), (KF_ITERATOR_CONSTRUCT, 2)],
            MODEL_MAP_NEXT_ENTRY,
            vec![
                (1, vec![(1, 0x61), (2, 0x62), (3, 0)]),
                (
                    2,
                    vec![(10, 0x71), (11, 0x72), (12, 0x73), (13, 0x74), (14, 0x75)],
                ),
            ],
        );
        log(&mut e);
        e.call(0x0044_8620, &args![this, 1u32]);
        assert_eq!(
            *removed.borrow(),
            vec!["model 1", "kf 10", "kf 12", "kf 13"]
        );
        assert_eq!(calls_to(&e, MODEL_RELEASE), vec![vec![0x61, 1]]);
        assert_eq!(
            calls_to(&e, KF_MODEL_RELEASE),
            vec![vec![0x71, 1], vec![0x73, 1], vec![0x74, 1]]
        );
        assert_eq!(e.mem.u8(this + 0x2c), 0);
    }

    #[test]
    fn fn_00448620_only_flags_the_loader_while_the_main_loop_is_busy() {
        let mut e = engine();
        let removed = events();
        let this = cleanup_loader(&mut e, &removed);
        e.mem.set_u32(MAIN_LOOP_FLAG_OBJECT, 0x9000);
        constant(&mut e, MAIN_LOOP_FLAG_TEST, 1);
        log(&mut e);
        e.call(0x0044_8620, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 0x2c), 1);
        assert!(calls_to(&e, MODEL_ITERATOR_CONSTRUCT).is_empty());
        assert!(removed.borrow().is_empty());
        // Forced, it walks anyway and clears the flag.
        quiet(
            &mut e,
            &[
                MODEL_ITERATOR_CONSTRUCT,
                KF_ITERATOR_CONSTRUCT,
                MODEL_ITERATOR_DESTRUCT,
                KF_ITERATOR_DESTRUCT,
            ],
        );
        constant(&mut e, MAP_ITERATOR_IS_DONE, 1);
        e.call(0x0044_8620, &args![this, 1u32]);
        assert_eq!(e.mem.u8(this + 0x2c), 0);
    }

    #[test]
    fn try_and_remove_model_removes_unused_models() {
        let mut e = engine();
        let removed = events();
        let this = cleanup_loader(&mut e, &removed);
        constant(&mut e, MODEL_REFERENCE_TOTAL, 1);
        quiet(&mut e, &[MODEL_RELEASE]);
        log(&mut e);
        e.call(0x0044_8920, &args![this, 0x61u32, 7u32]);
        assert!(removed.borrow().is_empty());
        constant(&mut e, MODEL_REFERENCE_TOTAL, 0);
        e.call(0x0044_8920, &args![this, 0x61u32, 7u32]);
        assert_eq!(*removed.borrow(), vec!["model 7"]);
        assert_eq!(calls_to(&e, MODEL_RELEASE), vec![vec![0x61, 1]]);
        // While the main loop object says so, the loader is only flagged.
        e.mem.set_u32(MAIN_LOOP_FLAG_OBJECT, 0x9000);
        constant(&mut e, MAIN_LOOP_FLAG_TEST, 1);
        e.call(0x0044_8920, &args![this, 0x61u32, 8u32]);
        assert_eq!(removed.borrow().len(), 1);
        assert_eq!(e.mem.u8(this + 0x2c), 1);
    }

    // --- 004489b0 .. 00448bf0 ------------------------------------------------

    #[test]
    fn the_flag_getters_test_their_bits() {
        let mut e = engine();
        let block = e.mem.alloc(0x80);
        assert!(!e.call(0x0044_8a20, &args![block]).bool());
        e.mem.set_u32(block + 8, 0x80);
        assert!(e.call(0x0044_8a20, &args![block]).bool());
        assert!(!e.call(0x0044_8a40, &args![block]).bool());
        e.mem.set_u32(block + 0xc, 0x10);
        assert!(e.call(0x0044_8a40, &args![block]).bool());
        assert_eq!(e.call(0x0044_8a60, &args![block, 0x30u32]).u32(), 0x10);
        e.mem.set_u32(ADDON_NODE_NAME, 0x4242);
        assert_eq!(e.call(0x0044_8a80, &args![]).u32(), 0x4242);
        assert!(!e.call(0x0044_8bf0, &args![block]).bool());
        e.mem.set_u8(block + 0x5a, 0xfe);
        assert!(!e.call(0x0044_8bf0, &args![block]).bool());
        e.mem.set_u8(block + 0x5a, 0x01);
        assert!(e.call(0x0044_8bf0, &args![block]).bool());
    }

    #[test]
    fn load_addon_nodes_needs_all_its_conditions() {
        let mut e = engine();
        e.mem.set_u32(ADDON_NODE_NAME, 0x4242);
        let extra = e.mem.alloc(0x20);
        e.mem.set_u32(extra + 0xc, 0x10);
        let object_block = e.mem.alloc(0x20);
        e.register_double(OBJECT_GET_EXTRA_DATA, move |_, a| {
            assert_eq!(a[1], 0x4242);
            if a[0] == 0x500 { extra } else { 0 }.into_ret()
        });
        constant(&mut e, OBJECT_IS_OF_TYPE, 0);
        constant(&mut e, NODE_CHILD_COUNT, 0);
        log(&mut e);
        let this = 0x100u32;
        // Not asked at all without a node.
        e.call(0x0044_89b0, &args![this, object_block, 0u32]);
        assert!(calls_to(&e, OBJECT_GET_EXTRA_DATA).is_empty());
        // No extra data, no object, the object's bit 0x80, the extra data's
        // missing bit 0x10: nothing is loaded.
        e.call(0x0044_89b0, &args![this, object_block, 0x600u32]);
        e.call(0x0044_89b0, &args![this, 0u32, 0x500u32]);
        e.mem.set_u32(object_block + 8, 0x80);
        e.call(0x0044_89b0, &args![this, object_block, 0x500u32]);
        e.mem.set_u32(object_block + 8, 0);
        e.mem.set_u32(extra + 0xc, 0);
        e.call(0x0044_89b0, &args![this, object_block, 0x500u32]);
        assert!(calls_to(&e, OBJECT_IS_OF_TYPE).is_empty());
        e.mem.set_u32(extra + 0xc, 0x10);
        e.call(0x0044_89b0, &args![this, object_block, 0x500u32]);
        assert_eq!(
            calls_to(&e, OBJECT_IS_OF_TYPE),
            vec![vec![ADDON_NODE_TYPE, 0x500]]
        );
    }

    /// The doubles of `LoadAddons`: a node of the add-on type whose add-on
    /// has the name `"addon.nif"`; the model map (virtual function 8) finds
    /// the name from its `found_on`-th lookup on, storing `0x8800`.
    struct Addons {
        this: u32,
        looked_up: Rc<Cell<u32>>,
    }

    fn addons(e: &mut Engine, found_on: u32, flags: u8) -> Addons {
        let name = text(e, "addon.nif");
        constant(e, 0x7600_0000, name);
        let name_holder = object(e, 0x0300_d000, &[(0x14, 0x7600_0000)], 0x20);
        let addon = e.mem.alloc(0x80);
        let table = e.mem.u32(name_holder);
        e.mem.set_u32(addon + 0x30, table);
        e.mem.set_u8(addon + 0x5a, flags);
        let looked_up = Rc::new(Cell::new(0));
        let counter = looked_up.clone();
        e.register_double(0x7600_0001, move |e, a| {
            counter.set(counter.get() + 1);
            let found = counter.get() >= found_on;
            if found {
                e.mem.set_u32(a[2], 0x8800);
            }
            u32::from(found).into_ret()
        });
        let models = object(e, 0x0300_e000, &[(8, 0x7600_0001)], 0x20);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this, models);
        constant(e, OBJECT_IS_OF_TYPE, 1);
        constant(e, NODE_ADDON_INDEX, 3);
        e.mem.set_u32(DATA_HANDLER, 0x9100);
        constant(e, DATA_HANDLER_GET_ADDON_NODE, addon);
        constant(e, NODE_CHILD_COUNT, 0);
        constant(e, MODEL_LOADER_LOAD_FILE, 1);
        quiet(e, &[MODEL_DESTROY]);
        Addons { this, looked_up }
    }

    #[test]
    fn load_addons_loads_a_missing_addon_and_destroys_its_model() {
        let mut e = engine();
        let a = addons(&mut e, 2, 0);
        log(&mut e);
        e.call(0x0044_8a90, &args![a.this, 0x500u32]);
        assert_eq!(
            calls_to(&e, DATA_HANDLER_GET_ADDON_NODE),
            vec![vec![0x9100, 3]]
        );
        let loaded = &calls_to(&e, MODEL_LOADER_LOAD_FILE)[0];
        assert_eq!(loaded[0], a.this);
        assert_eq!(e.mem.cstr(loaded[1]), b"addon.nif".to_vec());
        assert_eq!(loaded[2..], [0, 1, 0, 0, 0]);
        assert_eq!(a.looked_up.get(), 2);
        assert_eq!(calls_to(&e, MODEL_DESTROY), vec![vec![0x8800]]);
    }

    #[test]
    fn load_addons_leaves_a_known_or_flagged_addon_alone() {
        let mut e = engine();
        let a = addons(&mut e, 1, 0);
        log(&mut e);
        e.call(0x0044_8a90, &args![a.this, 0x500u32]);
        assert!(calls_to(&e, MODEL_LOADER_LOAD_FILE).is_empty());
        assert!(calls_to(&e, MODEL_DESTROY).is_empty());
        let mut e = engine();
        let a = addons(&mut e, 2, 1);
        log(&mut e);
        e.call(0x0044_8a90, &args![a.this, 0x500u32]);
        assert_eq!(a.looked_up.get(), 0);
        // Not of the type: no lookup either, and a null node is ignored.
        constant(&mut e, OBJECT_IS_OF_TYPE, 0);
        log(&mut e);
        e.call(0x0044_8a90, &args![a.this, 0x500u32]);
        e.call(0x0044_8a90, &args![a.this, 0u32]);
        assert!(calls_to(&e, NODE_ADDON_INDEX).is_empty());
    }

    #[test]
    fn load_addons_visits_the_children() {
        let mut e = engine();
        let a = addons(&mut e, 1, 1);
        // The node has one child, whose virtual function 0xc gives the node
        // 0x600.
        e.register_double(NODE_CHILD_COUNT, |_, a| u32::from(a[0] == 0x500).into_ret());
        constant(&mut e, 0x7600_0002, 0x600);
        let child = object(&mut e, 0x0300_f000, &[(0xc, 0x7600_0002)], 0x20);
        constant(&mut e, NODE_CHILD_AT, child);
        log(&mut e);
        e.call(0x0044_8a90, &args![a.this, 0x500u32]);
        assert_eq!(
            calls_to(&e, OBJECT_IS_OF_TYPE),
            vec![vec![ADDON_NODE_TYPE, 0x500], vec![ADDON_NODE_TYPE, 0x600]]
        );
        // The count is asked again on each pass: twice for the node, once
        // for the child node.
        assert_eq!(calls_to(&e, NODE_CHILD_COUNT).len(), 3);
    }

    // --- 00448c10 .. 00448f80 ----------------------------------------------

    #[test]
    fn fn_00448c10_runs_every_queued_task() {
        let mut e = engine();
        let seen = events();
        let sink = seen.clone();
        e.register_double(0x7700_0000, move |_, a| {
            sink.borrow_mut().push(format!("task {:x}", a[0]));
            Ret::default()
        });
        let tasks = [
            object(&mut e, 0x0301_0000, &[(0x14, 0x7700_0000)], 0x20),
            object(&mut e, 0x0301_1000, &[(0x14, 0x7700_0000)], 0x20),
        ];
        let mut upcoming = vec![0, tasks[1], tasks[0]];
        e.register_double(QUEUE_NEXT_TASK, move |e, a| {
            assert_eq!(a[0], 0x5555);
            e.mem.set_u32(a[1], upcoming.pop().unwrap());
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x1c, 0x5555);
        assert!(e.call(0x0044_8c10, &args![this]).bool());
        assert_eq!(
            *seen.borrow(),
            vec![
                format!("task {:x}", tasks[0]),
                format!("task {:x}", tasks[1])
            ]
        );
    }

    #[test]
    fn the_timer_functions_keep_a_start_and_a_limit() {
        let mut e = engine();
        let timer = e.mem.alloc(0x10);
        e.mem.set_u32(timer + 4, 7);
        assert_eq!(e.call(0x0044_8e00, &args![timer]).u32(), timer);
        assert!((0..4).all(|i| e.mem.u32(timer + 4 * i) == 0));
        e.mem.set_u32(TIMER_SCALE, 2.0f32.to_bits());
        let now = Rc::new(Cell::new(0x0000_0001_0000_0010u64));
        let clock = now.clone();
        e.register_double(TIMER_GET_TIME, move |_, _| clock.get().into_ret());
        e.register_double(FLOAT_TO_INT64, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i64 as u64).into_ret()
        });
        e.call(0x0044_8e30, &args![timer, 1.5f32]);
        assert_eq!(e.mem.u32(timer), 0x10);
        assert_eq!(e.mem.u32(timer + 4), 1);
        assert_eq!((e.mem.u32(timer + 8), e.mem.u32(timer + 0xc)), (3, 0));
        // Elapsed 3 is not above the limit 3; 4 is; the comparison carries
        // across the two halves.
        now.set(0x0000_0001_0000_0013);
        assert!(!e.call(0x0044_8e70, &args![timer]).bool());
        now.set(0x0000_0001_0000_0014);
        assert!(e.call(0x0044_8e70, &args![timer]).bool());
        now.set(0x0000_0002_0000_000f);
        assert!(e.call(0x0044_8e70, &args![timer]).bool());
        // A limit below zero is exceeded by any elapsed time.
        e.mem.set_u32(timer + 8, 0xffff_fff0);
        e.mem.set_u32(timer + 0xc, 0xffff_ffff);
        now.set(0x0000_0001_0000_0010);
        assert!(e.call(0x0044_8e70, &args![timer]).bool());
    }

    /// The doubles of `00448cc0`: the task manager's dword at +0x68 is
    /// `state`, the timer reads 40 ticks more at each look, the scale is
    /// 10, and the fourth map yields three tasks.
    fn timed_walk(e: &mut Engine, state: u32) -> (u32, Events) {
        let manager = e.mem.alloc(0x80);
        e.mem.set_u32(manager + 0x68, state);
        e.mem.set_u32(TASK_MANAGER, manager);
        e.mem.set_u32(SHORT_TIMEOUT, 5.0f32.to_bits());
        e.mem.set_u32(LONG_TIMEOUT, 100000.0f32.to_bits());
        e.mem.set_u32(TIMER_SCALE, 10.0f32.to_bits());
        let mut clock = 0u64;
        e.register_double(TIMER_GET_TIME, move |_, _| {
            let now = clock;
            clock += 40;
            now.into_ret()
        });
        e.register_double(FLOAT_TO_INT64, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i64 as u64).into_ret()
        });
        let seen = events();
        let mut entries = Vec::new();
        for index in 1..=3u32 {
            let sink = seen.clone();
            let slot = 0x7800_0000 + index;
            e.register_double(slot, move |_, _| {
                sink.borrow_mut().push(format!("task {index}"));
                Ret::default()
            });
            let table = 0x0302_0000 + index * 0x1000;
            let task = object(e, table, &[(0x20, slot)], 0x20);
            entries.push((0, task));
        }
        map_doubles(
            e,
            &[(MAP_ITERATOR_CONSTRUCT, 1)],
            FOURTH_MAP_NEXT_ENTRY,
            vec![(1, entries)],
        );
        quiet(e, &[MAP_ITERATOR_DESTRUCT]);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xc, 0x82);
        (this, seen)
    }

    /// The seconds-times-scale value `00448cc0` converted to ticks.
    fn converted_ticks(e: &Engine) -> f64 {
        let converted = &calls_to(e, FLOAT_TO_INT64)[0];
        f64::from_bits(u64::from(converted[0]) | u64::from(converted[1]) << 32)
    }

    #[test]
    fn fn_00448cc0_stops_when_the_timer_runs_out() {
        let mut e = engine();
        let (this, seen) = timed_walk(&mut e, 6);
        log(&mut e);
        // Limit 50 ticks: the look at 40 passes, 80 and 120 do not.
        assert!(!e.call(0x0044_8cc0, &args![this]).bool());
        assert_eq!(*seen.borrow(), vec!["task 1"]);
        assert_eq!(converted_ticks(&e), 50.0);
        assert_eq!(calls_to(&e, MAP_ITERATOR_DESTRUCT).len(), 1);
    }

    #[test]
    fn fn_00448cc0_walks_the_whole_map_with_the_long_timeout() {
        let mut e = engine();
        let (this, seen) = timed_walk(&mut e, 3);
        log(&mut e);
        assert!(e.call(0x0044_8cc0, &args![this]).bool());
        assert_eq!(*seen.borrow(), vec!["task 1", "task 2", "task 3"]);
        assert_eq!(converted_ticks(&e), 1_000_000.0);
    }

    #[test]
    fn the_forwarders_to_the_object_at_0x20() {
        let mut e = engine();
        let seen = events();
        let sink = seen.clone();
        e.register_double(0x7900_0010, move |e, a| {
            let held = e.mem.u32(a[2]);
            sink.borrow_mut()
                .push(format!("ten {} {held} {}", a[1], a[3]));
            1u32.into_ret()
        });
        let sink = seen.clone();
        e.register_double(0x7900_0014, move |_, a| {
            sink.borrow_mut().push(format!("fourteen {}", a[1]));
            Ret::default()
        });
        e.register_double(0x7900_0008, |e, a| {
            e.mem.set_u32(a[2], 0x5151);
            u32::from(a[1] == 1).into_ret()
        });
        let target = object(
            &mut e,
            0x0303_0000,
            &[(8, 0x7900_0008), (0x10, 0x7900_0010), (0x14, 0x7900_0014)],
            0x20,
        );
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x20, target);
        assert!(e.call(0x0044_8ed0, &args![this, 3u32, 0x66u32]).bool());
        e.call(0x0044_8f50, &args![this, 4u32]);
        assert_eq!(*seen.borrow(), vec!["ten 3 102 0", "fourteen 4"]);
        // The lookup answers the pointer it stored, or 0.
        assert_eq!(e.call(0x0044_8f80, &args![this, 1u32]).u32(), 0x5151);
        assert_eq!(e.call(0x0044_8f80, &args![this, 2u32]).u32(), 0);
    }

    // --- 00449030 .. 004491c0 ---------------------------------------------

    #[test]
    fn fn_00449030_calls_the_object_at_0x28() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x28, 0x4400);
        quiet(&mut e, &[LOADER_CLONE_THREAD_CALL]);
        log(&mut e);
        e.call(0x0044_9030, &args![this]);
        assert_eq!(calls_to(&e, LOADER_CLONE_THREAD_CALL), vec![vec![0x4400]]);
    }

    #[test]
    fn fn_00449050_maps_four_values() {
        let mut e = engine();
        let answers: Vec<u32> = [0u32, 1, 2, 3, 99]
            .iter()
            .map(|v| e.call(0x0044_9050, &args![*v]).u32())
            .collect();
        assert_eq!(answers, vec![1, 0, 3, 8, 8]);
    }

    #[test]
    fn the_sync_object_is_initialised_and_released() {
        let mut e = engine();
        quiet(&mut e, &[SYNC_OBJECT_INIT, SYNC_OBJECT_RELEASE]);
        let this = e.mem.alloc(0x20);
        for offset in [8, 0xc, 0x10, 0x14] {
            e.mem.set_u32(this + offset, 0xdead);
        }
        log(&mut e);
        assert_eq!(e.call(0x0044_9090, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SYNC_VTABLE);
        assert!([8, 0xc, 0x10, 0x14]
            .iter()
            .all(|o| e.mem.u32(this + o) == 0));
        assert_eq!(calls_to(&e, SYNC_OBJECT_INIT), vec![vec![SYNC_OBJECT]]);
        e.mem.set_u32(this, 0);
        e.call(0x0044_90e0, &args![this]);
        assert_eq!(e.mem.u32(this), SYNC_VTABLE);
        assert_eq!(calls_to(&e, SYNC_OBJECT_RELEASE), vec![vec![SYNC_OBJECT]]);
    }

    #[test]
    fn the_task_state_functions() {
        let mut e = engine();
        let task = e.mem.alloc(0x20);
        for (state, at_least_four, finished) in [
            (0u32, false, false),
            (3, false, false),
            (4, true, false),
            (6, true, true),
        ] {
            e.mem.set_u32(task + 0xc, state);
            assert_eq!(e.call(0x0044_9110, &args![task]).bool(), at_least_four);
            assert_eq!(e.call(0x0044_9130, &args![task]).bool(), finished);
        }
        e.mem.set_u32(task + 0xc, 0xffff_ffff);
        assert!(!e.call(0x0044_9110, &args![task]).bool());
        e.call(0x0044_9150, &args![task]);
        assert_eq!(e.mem.u32(task + 0xc), 5);
        e.call(0x0044_9170, &args![task]);
        assert_eq!(e.mem.u32(task + 0xc), 6);
    }

    #[test]
    fn the_state_change_compares_and_exchanges() {
        let mut e = engine();
        e.register_double(COMPARE_EXCHANGE, |e, a| {
            let previous = e.mem.u32(a[0]);
            if previous == a[1] {
                e.mem.set_u32(a[0], a[2]);
            }
            previous.into_ret()
        });
        quiet(&mut e, &[AFTER_COMPARE_EXCHANGE]);
        let task = e.mem.alloc(0x20);
        e.mem.set_u32(task + 0xc, 2);
        log(&mut e);
        // From 2 to 6: succeeds; from 2 again: fails and changes nothing.
        assert!(e.call(0x0044_9190, &args![task, 2u32, 6u32]).bool());
        assert_eq!(e.mem.u32(task + 0xc), 6);
        assert!(!e.call(0x0044_9190, &args![task, 2u32, 7u32]).bool());
        assert_eq!(e.mem.u32(task + 0xc), 6);
        assert_eq!(calls_to(&e, AFTER_COMPARE_EXCHANGE).len(), 2);
        assert_eq!(calls_to(&e, COMPARE_EXCHANGE)[0], vec![task + 0xc, 2, 6]);
        // The exchange itself takes (target, new, expected).
        assert!(e.call(0x0044_91c0, &args![task + 0xc, 1u32, 6u32]).bool());
        assert_eq!(e.mem.u32(task + 0xc), 1);
    }

    // --- 004491f0 .. 00449bd0 ---------------------------------------------

    /// A character-wise `tolower` double (C locale) that answers the
    /// sign-extended value for characters it leaves alone.
    fn lowering(e: &mut Engine) {
        e.register_double(TOLOWER_CHARACTER, |_, a| {
            let c = a[0] as i32;
            (if (65..=90).contains(&c) { c + 32 } else { c } as u32).into_ret()
        });
    }

    /// The addresses of the calls logged so far, in order.
    fn call_order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(a, _)| *a)
            .collect()
    }

    /// A wrapper object whose forwarding target ([`MAP_TARGET`]) is
    /// `0x5000`, with the lock functions and `callee` (answering `answer`)
    /// as test doubles.
    fn locked_wrapper(e: &mut Engine, callee: u32, answer: u32) -> u32 {
        quiet(e, &[LOCK_ENTER, LOCK_LEAVE]);
        constant(e, MAP_TARGET, 0x5000);
        constant(e, callee, answer);
        e.mem.alloc(0x40)
    }

    #[test]
    fn fn_004491f0_forwards_this() {
        let mut e = engine();
        quiet(&mut e, &[0x0096_11e0]);
        log(&mut e);
        e.call(0x0044_91f0, &args![0x1234u32]);
        assert_eq!(calls_to(&e, 0x0096_11e0), vec![vec![0x1234]]);
    }

    #[test]
    fn fn_00449210_runs_the_element_then_the_virtual_function() {
        let mut e = engine();
        let callee = 0x0044_cd40;
        let this = locked_wrapper(&mut e, callee, 0);
        // `this` is an element of its own array: `this + 4` points at an
        // array whose element 1 is a wrapper.
        let element = e.mem.alloc(0x40);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array + 4, element);
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(this, table);
        e.mem.set_u32(table + 4, SLOT_A);
        e.mem.set_u32(this + 4, array);
        quiet(&mut e, &[SLOT_A]);
        log(&mut e);
        e.call(0x0044_9210, &args![this, 1u32, 0x77u32]);
        assert_eq!(
            call_order(&e),
            vec![
                0x0044_9210,
                LOCK_ENTER,
                MAP_TARGET,
                callee,
                LOCK_LEAVE,
                SLOT_A
            ]
        );
        assert_eq!(calls_to(&e, LOCK_ENTER), vec![vec![element + 0x20, 0]]);
        assert_eq!(calls_to(&e, callee), vec![vec![0x5000, 0x77]]);
        assert_eq!(calls_to(&e, SLOT_A), vec![vec![this]]);
    }

    #[test]
    fn fn_00449240_forwards_under_the_lock() {
        let mut e = engine();
        let this = locked_wrapper(&mut e, 0x0044_cd40, 0);
        log(&mut e);
        e.call(0x0044_9240, &args![this, 0x77u32]);
        assert_eq!(
            call_order(&e),
            vec![0x0044_9240, LOCK_ENTER, MAP_TARGET, 0x0044_cd40, LOCK_LEAVE]
        );
        assert_eq!(calls_to(&e, LOCK_ENTER), vec![vec![this + 0x20, 0]]);
        assert_eq!(calls_to(&e, MAP_TARGET), vec![vec![this]]);
        assert_eq!(calls_to(&e, 0x0044_cd40), vec![vec![0x5000, 0x77]]);
        assert_eq!(calls_to(&e, LOCK_LEAVE), vec![vec![this + 0x20]]);
    }

    #[test]
    fn fn_00449280_answers_the_byte_under_the_lock() {
        let mut e = engine();
        let this = locked_wrapper(&mut e, 0x0044_cec0, 0x1_0001);
        log(&mut e);
        assert_eq!(e.call(0x0044_9280, &args![this, 0x77u32]).u8(), 1);
        assert_eq!(
            call_order(&e),
            vec![0x0044_9280, LOCK_ENTER, MAP_TARGET, 0x0044_cec0, LOCK_LEAVE]
        );
        assert_eq!(calls_to(&e, 0x0044_cec0), vec![vec![0x5000, 0x77]]);
    }

    #[test]
    fn the_constructors_call_their_base_and_set_the_table() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        for (address, base, table) in [
            (0x0044_92c0u32, 0x0044_9f50u32, 0x0101_7154u32),
            (0x0044_96a0, 0x0044_9fa0, 0x0101_71ec),
            (0x0044_97c0, 0x0044_a100, 0x0101_723c),
        ] {
            constant(&mut e, base, 0);
            log(&mut e);
            assert_eq!(e.call(address, &args![this, 1u32, 2u32, 3u32]).u32(), this);
            assert_eq!(calls_to(&e, base), vec![vec![this, 1, 2, 3]]);
            assert_eq!(e.mem.u32(this), table);
        }
    }

    #[test]
    fn fn_004492f0_copies_the_name_in_lower_case() {
        let mut e = engine();
        lowering(&mut e);
        let source = text(&mut e, "Meshes\\ABC");
        log(&mut e);
        let copy = e.call(0x0044_92f0, &args![0u32, source]).u32();
        assert_eq!(e.mem.cstr(copy), b"meshes\\abc".to_vec());
        assert_ne!(copy, source);
        // The guard is entered with context 6, then left.
        assert_eq!(
            calls_to(&e, MEMORY_CONTEXT_ENTER)[0][1..],
            [6, 1, 0x0101_71a0, 0x1fd]
        );
        assert_eq!(calls_to(&e, MEMORY_CONTEXT_LEAVE).len(), 1);
        assert_eq!(calls_to(&e, MEMORY_ALLOC), vec![vec![11]]);
    }

    #[test]
    fn fn_004493d0_passes_the_lower_case_copy() {
        let mut e = engine();
        lowering(&mut e);
        let this = locked_wrapper(&mut e, 0x006e_c830, 0);
        let source = text(&mut e, "AbC");
        let seen = events();
        let sink = seen.clone();
        e.register_double(0x006e_c830, move |e, a| {
            sink.borrow_mut()
                .push(String::from_utf8(e.mem.cstr(a[1])).unwrap());
            1u32.into_ret()
        });
        // The callee's byte comes back.
        assert_eq!(e.call(0x0044_93d0, &args![this, source, 9u32]).u8(), 1);
        assert_eq!(*seen.borrow(), vec!["abc"]);
    }

    #[test]
    fn fn_00449460_passes_the_lower_case_copy_and_the_flag() {
        let mut e = engine();
        lowering(&mut e);
        let this = locked_wrapper(&mut e, 0x0044_a130, 0);
        let source = text(&mut e, "XyZ");
        let seen = events();
        let sink = seen.clone();
        e.register_double(0x0044_a130, move |e, a| {
            sink.borrow_mut().push(format!(
                "{} {} {}",
                String::from_utf8(e.mem.cstr(a[1])).unwrap(),
                a[2],
                a[3]
            ));
            1u32.into_ret()
        });
        // The callee's byte comes back.
        assert_eq!(e.call(0x0044_9460, &args![this, source, 9u32, 1u8]).u8(), 1);
        assert_eq!(*seen.borrow(), vec!["xyz 9 1"]);
    }

    #[test]
    fn fn_00449500_copies_into_the_buffer_the_pointer_holds() {
        let mut e = engine();
        let source = text(&mut e, "hello");
        let buffer = e.mem.alloc(0x100);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, buffer);
        log(&mut e);
        e.call(0x0044_9500, &args![0u32, source, holder]);
        assert_eq!(e.mem.cstr(buffer), b"hello".to_vec());
        assert_eq!(calls_to(&e, STRING_COPY)[0][1], 0x100);
    }

    #[test]
    fn fn_00449530_forwards_the_key_and_argument_under_the_lock() {
        let mut e = engine();
        let this = locked_wrapper(&mut e, 0x006e_c830, 1);
        log(&mut e);
        assert_eq!(e.call(0x0044_9530, &args![this, 0x11u32, 0x22u32]).u8(), 1);
        assert_eq!(
            call_order(&e),
            vec![0x0044_9530, LOCK_ENTER, MAP_TARGET, 0x006e_c830, LOCK_LEAVE]
        );
        assert_eq!(calls_to(&e, 0x006e_c830), vec![vec![0x5000, 0x11, 0x22]]);
    }

    #[test]
    fn fn_00449580_forwards_the_flag_as_a_word() {
        let mut e = engine();
        let this = locked_wrapper(&mut e, 0x0044_a130, 0);
        log(&mut e);
        assert_eq!(
            e.call(0x0044_9580, &args![this, 0x11u32, 0x22u32, 1u8])
                .u8(),
            0
        );
        assert_eq!(calls_to(&e, 0x0044_a130), vec![vec![0x5000, 0x11, 0x22, 1]]);
        assert_eq!(calls_to(&e, LOCK_LEAVE).len(), 1);
    }

    #[test]
    fn fn_004495d0_forwards_the_key() {
        let mut e = engine();
        let this = locked_wrapper(&mut e, 0x0066_5f20, 1);
        log(&mut e);
        assert_eq!(e.call(0x0044_95d0, &args![this, 0x11u32]).u8(), 1);
        assert_eq!(calls_to(&e, 0x0066_5f20), vec![vec![0x5000, 0x11]]);
        assert_eq!(calls_to(&e, LOCK_ENTER), vec![vec![this + 0x20, 0]]);
    }

    #[test]
    fn fn_00449610_constructs_and_clears_two_fields() {
        let mut e = engine();
        quiet(&mut e, &[0x0044_9fd0, 0x0044_a0a0]);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x18, 5);
        e.mem.set_u32(this + 0x1c, 6);
        log(&mut e);
        assert_eq!(e.call(0x0044_9610, &args![this]).u32(), this);
        assert_eq!(call_order(&e), vec![0x0044_9610, 0x0044_9fd0, 0x0044_a0a0]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
    }

    #[test]
    fn fn_00449680_forwards_this() {
        let mut e = engine();
        quiet(&mut e, &[0x0044_a040]);
        log(&mut e);
        e.call(0x0044_9680, &args![0x4321u32]);
        assert_eq!(calls_to(&e, 0x0044_a040), vec![vec![0x4321]]);
    }

    #[test]
    fn fn_004496d0_is_true_unless_the_first_text_is_smaller() {
        let mut e = engine();
        e.register_double(STRING_COMPARE, |e, a| {
            let (x, y) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            (x.cmp(&y) as i32 as u32).into_ret()
        });
        let (a, b) = (text(&mut e, "apple"), text(&mut e, "pear"));
        assert!(!e.call(0x0044_96d0, &args![0u32, a, b]).bool());
        assert!(e.call(0x0044_96d0, &args![0u32, b, a]).bool());
        assert!(e.call(0x0044_96d0, &args![0u32, a, a]).bool());
    }

    #[test]
    fn fn_00449700_passes_the_address_of_the_argument() {
        let mut e = engine();
        let this = locked_wrapper(&mut e, 0x0044_d3f0, 1);
        let seen = events();
        let sink = seen.clone();
        e.register_double(0x0044_d3f0, move |e, a| {
            sink.borrow_mut()
                .push(format!("{:x} {:x} {:x}", a[0], a[1], e.mem.u32(a[2])));
            1u32.into_ret()
        });
        log(&mut e);
        assert_eq!(
            e.call(0x0044_9700, &args![this, 0x11u32, 0x2222u32]).u8(),
            1
        );
        assert_eq!(*seen.borrow(), vec!["5000 11 2222"]);
        assert_eq!(
            call_order(&e),
            vec![0x0044_9700, LOCK_ENTER, MAP_TARGET, 0x0044_d3f0, LOCK_LEAVE]
        );
    }

    #[test]
    fn fn_00449750_hashes_the_lower_case_name_modulo_the_bucket_count() {
        let mut e = engine();
        lowering(&mut e);
        constant(&mut e, FIELD_AT_8, 101);
        let upper = text(&mut e, "AB");
        let lower = text(&mut e, "ab");
        // ('a' * 33 + 'b') % 101
        let expected = (97 * 33 + 98) % 101;
        assert_eq!(e.call(0x0044_9750, &args![0x10u32, upper]).u32(), expected);
        assert_eq!(e.call(0x0044_9750, &args![0x10u32, lower]).u32(), expected);
        // The empty name hashes to 0.
        let empty = text(&mut e, "");
        assert_eq!(e.call(0x0044_9750, &args![0x10u32, empty]).u32(), 0);
    }

    #[test]
    fn fn_00449750_adds_a_high_character_sign_extended() {
        let mut e = engine();
        lowering(&mut e);
        constant(&mut e, FIELD_AT_8, 1000);
        let name = e.mem.alloc(4);
        e.mem.set_u8(name, 0xc9);
        e.mem.set_u8(name + 1, 0);
        let expected = 0xffff_ffc9u32 % 1000;
        assert_eq!(e.call(0x0044_9750, &args![0x10u32, name]).u32(), expected);
    }

    #[test]
    fn fn_004497f0_passes_the_lower_case_copy() {
        let mut e = engine();
        lowering(&mut e);
        let this = locked_wrapper(&mut e, 0x0066_5f20, 0);
        let source = text(&mut e, "QQ");
        let seen = events();
        let sink = seen.clone();
        e.register_double(0x0066_5f20, move |e, a| {
            sink.borrow_mut()
                .push(String::from_utf8(e.mem.cstr(a[1])).unwrap());
            Ret::default()
        });
        e.call(0x0044_97f0, &args![this, source]);
        assert_eq!(*seen.borrow(), vec!["qq"]);
    }

    #[test]
    fn fn_004497f0_answers_the_lookup_byte() {
        let mut e = engine();
        lowering(&mut e);
        let this = locked_wrapper(&mut e, 0x0066_5f20, 0x0000_0101);
        let source = text(&mut e, "QQ");
        assert_eq!(e.call(0x0044_97f0, &args![this, source]).u32(), 1);
    }

    #[test]
    fn fn_00449880_is_true_for_equal_texts() {
        let mut e = engine();
        e.register_double(STRING_COMPARE, |e, a| {
            u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])).into_ret()
        });
        let (a, b) = (text(&mut e, "x"), text(&mut e, "y"));
        assert!(e.call(0x0044_9880, &args![0u32, a, a]).bool());
        assert!(!e.call(0x0044_9880, &args![0u32, a, b]).bool());
    }

    #[test]
    fn fn_004498b0_forwards_without_the_lock() {
        let mut e = engine();
        let this = locked_wrapper(&mut e, 0x006e_c830, 1);
        log(&mut e);
        assert_eq!(e.call(0x0044_98b0, &args![this, 0x11u32, 0x22u32]).u8(), 1);
        assert!(calls_to(&e, LOCK_ENTER).is_empty());
        assert_eq!(calls_to(&e, 0x006e_c830), vec![vec![0x5000, 0x11, 0x22]]);
    }

    #[test]
    fn the_inline_flag_constructors() {
        let mut e = engine();
        for (address, base, table) in [
            (0x0044_98e0u32, 0x0044_cb70u32, 0x0101_7288u32),
            (0x0044_9960, 0x0044_cbb0, 0x0101_72a0),
        ] {
            quiet(&mut e, &[base]);
            let this = e.mem.alloc(0x40);
            e.mem.set_u8(this + 0x10, 9);
            log(&mut e);
            assert_eq!(e.call(address, &args![this]).u32(), this);
            assert_eq!(calls_to(&e, base), vec![vec![this]]);
            assert_eq!(e.mem.u32(this), table);
            assert_eq!(e.mem.u32(this + 8), this + 0x10);
            assert_eq!(e.mem.u8(this + 0x10), 0);
        }
    }

    #[test]
    fn fn_00449920_clears_the_key_byte() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u8(this + 0x10, 7);
        e.call(0x0044_9920, &args![this]);
        assert_eq!(e.mem.u8(this + 0x10), 0);
    }

    #[test]
    fn the_table_setters() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        for (address, table) in [
            (0x0044_9940u32, 0x0101_7294u32),
            (0x0044_99a0, 0x0101_72ac),
            (0x0044_99c0, 0x0101_72b8),
            (0x0044_99e0, 0x0101_72c4),
            (0x0044_9a00, 0x0101_72d0),
        ] {
            e.call(address, &args![this]);
            assert_eq!(e.mem.u32(this), table);
        }
    }

    #[test]
    fn the_deleting_destructors_free_only_when_flagged() {
        let mut e = engine();
        // (address, the destructor body it runs, or None when it is a table setter)
        let external = [
            (0x0044_9a20u32, 0x0044_90e0u32),
            (0x0044_9a50, 0x0044_9c30),
            (0x0044_9a80, 0x0044_9d40),
            (0x0044_9ab0, 0x0044_9ea0),
            (0x0044_9ae0, 0x0044_31d0),
            (0x0044_9b40, 0x0044_3220),
        ];
        // 00449a20 runs the translated 004490e0, which uses the sync object.
        quiet(&mut e, &[SYNC_OBJECT_RELEASE]);
        for (address, body) in external {
            if address != 0x0044_9a20 {
                quiet(&mut e, &[body]);
            }
            for (flags, freed) in [(0u32, 0usize), (1, 1), (2, 0), (3, 1)] {
                let this = e.mem.alloc(0x40);
                log(&mut e);
                assert_eq!(e.call(address, &args![this, flags]).u32(), this);
                assert_eq!(
                    calls_to(&e, MEMORY_FREE).len(),
                    freed,
                    "{address:x} {flags}"
                );
                if address != 0x0044_9a20 {
                    assert_eq!(calls_to(&e, body), vec![vec![this]]);
                }
            }
        }
        // The ones that set a table.
        for (address, table) in [
            (0x0044_9b10u32, 0x0101_7294u32),
            (0x0044_9b70, 0x0101_72ac),
            (0x0044_9ba0, 0x0101_72b8),
            (0x0044_9bd0, 0x0101_72c4),
        ] {
            let kept = e.mem.alloc(0x40);
            log(&mut e);
            assert_eq!(e.call(address, &args![kept, 0u32]).u32(), kept);
            assert_eq!(e.mem.u32(kept), table);
            assert!(calls_to(&e, MEMORY_FREE).is_empty());
            let this = e.mem.alloc(0x40);
            log(&mut e);
            e.call(address, &args![this, 1u32]);
            assert_eq!(calls_to(&e, MEMORY_FREE), vec![vec![this]]);
        }
    }

    #[test]
    fn fn_004491f0_answers_the_callees_value() {
        let mut e = engine();
        constant(&mut e, 0x0096_11e0, 7);
        assert_eq!(e.call(0x0044_91f0, &args![0x1234u32]).u32(), 7);
    }
}

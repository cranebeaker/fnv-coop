//! `fallout shared/pathfinding/navmeshobstaclemanager.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds `NavMeshObstacleManager` (one instance, a function-local
//! static at `011d71e0` made by `006c0720`), its helper classes
//! (`ReferenceObstacleArray`, `ObstacleData`, the two Havok listeners) and
//! the template instances they use (`NiTMap`, `BSSimpleList`, lock-free
//! queues).
//!
//! Session 1 (b0328) translated the 40 lowest-address open functions, up to
//! and including `006c1dd0`.
//!
//! Session 2 (b0328) translated the next 40: `006c1e00` up to and including
//! `006c5b20` (the Havok listener removal, the obstacle build from a
//! reference's 3D, the per-frame update `006c3640` with its steps, the portal
//! reconnection and the debug bounding boxes); the queue continued at
//! `006c5c50`.
//!
//! Session 3 (b0328) translated the next 40: `006c5c50` up to and including
//! `006c6e60` (the task hand-over to the tasklet group, the constructors and
//! destructors of the manager's maps and arrays, `SetAt` of the reference
//! map and the lock-free queue with its push); the queue continues at
//! `006c6fe0`.
//!
//! Session 4 (b0328) translated the last 14: `006c6fe0` up to and including
//! `006c77a0` (the lock-free queue pop and node retirement, the deleting
//! destructors of the containers, the interface manager's destructor and the
//! `ObstacleTaskData` constructor). The unit is finished; `006c7850` and the
//! functions after it belong to other units.
//!
//! Conventions of this file:
//!
//! - The compiler built this unit without optimisation, so every accessor
//!   (`NiPointer::operator->`, `BSSimpleArray::operator[]`, `GetSize`, ...)
//!   is a real call. The translations call each of them by address through
//!   the small wrappers below, as the game does; the docs of the constants
//!   say what the body of the callee does, which is all that is known of it.
//! - A local the game keeps on its stack and passes by address (a
//!   `NiPointer`, an iterator cursor) is a slot in game memory
//!   ([`Engine::with_stack`]). `NiPointer<T>` is one word: the pointer.
//! - Several callees take no parameters although the caller pushed a word
//!   before calling them (`006c0720`, `00b00a00`): the word stays on the
//!   stack and is the argument of the next call
//!   (`PUSH x; CALL getter; MOV ECX,EAX; CALL method`). The translations
//!   pass it to that next call.
//! - The compiler's exception-unwinding frames are not translated.
//!
//! Offsets of the classes are the PC ones: the Xbox PDB's with the 0x10
//! bytes of 16-byte alignment padding removed after the first fields.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleArray, BSSimpleList, NiTMap, NiTPointerMap};

layout! {
    /// `NavMeshObstacleManager` (Xbox PDB), PC offsets. The real object is a
    /// static of `0x1c0` bytes (`011d71e0`..`011d73a0`, the word after it is
    /// the initialisation guard of `006c0720`); the size here is the end of
    /// the last field.
    pub struct NavMeshObstacleManager: 0x1a4 {
        /// `UpdateCriticalSection` (Xbox PDB): built by `0044fae0`, torn
        /// down by `0044fb00`.
        0x00 UpdateCriticalSection: Inline<()>,
        /// `bUpdateAllObstacles` (Xbox PDB).
        0x18 bUpdateAllObstacles: bool,
        /// `FormIDMap` (Xbox PDB): `NiTMap<unsigned int, NiPointer<ReferenceObstacleArray>>`
        /// (`0x25` hash buckets).
        0x1c FormIDMap: Inline<NiTMap>,
        /// `QueuedListOfRefsToAdd` (Xbox PDB): `BSSimpleList<TESObjectREFR *>`.
        0x2c QueuedListOfRefsToAdd: Inline<BSSimpleList>,
        /// `QueuedListOfRefsToRemove` (Xbox PDB): `BSSimpleList<unsigned int>`.
        0x34 QueuedListOfRefsToRemove: Inline<BSSimpleList>,
        /// `ObstaclesUpdateList` (Xbox PDB): `BSSimpleList<NiPointer<ObstacleData>>`.
        0x3c ObstaclesUpdateList: Inline<BSSimpleList>,
        /// `ObstaclesToAddToUpdateList` (Xbox PDB): a `LockFreeQueue<bhkRigidBody *>`
        /// (built by `006c6450(7, 8)`, emptied by `006c6700`).
        0x60 ObstaclesToAddToUpdateList: Inline<()>,
        /// `ObstaclesToRemoveFromUpdateList` (Xbox PDB): a `LockFreeQueue<bhkRigidBody *>`.
        0xa0 ObstaclesToRemoveFromUpdateList: Inline<()>,
        /// `RigidBodyToObstacleMap` (Xbox PDB): `NiTMap<bhkRigidBody *, NiPointer<ObstacleData>>`.
        0xe0 RigidBodyToObstacleMap: Inline<NiTMap>,
        /// `QueuedListOfDoorsToAdd` (Xbox PDB): `BSSimpleList<TESObjectREFR *>`.
        0xf0 QueuedListOfDoorsToAdd: Inline<BSSimpleList>,
        /// `QueuedListOfDoorsToRemove` (Xbox PDB): `BSSimpleList<unsigned int>`.
        0xf8 QueuedListOfDoorsToRemove: Inline<BSSimpleList>,
        /// `OpenDoorMap` (Xbox PDB): `NiTMap<unsigned int, NiPointer<ReferenceObstacleArray>>`.
        0x100 OpenDoorMap: Inline<NiTMap>,
        /// `ClosedDoorMap` (Xbox PDB): the same kind of map.
        0x110 ClosedDoorMap: Inline<NiTMap>,
        /// `pListener` (Xbox PDB): `bhkObstacleDeactivationListener *`.
        0x120 pListener: u32,
        /// `pRemoveListener` (Xbox PDB): `bhkObstacleRemovalListener *`.
        0x124 pRemoveListener: u32,
        /// `ObstacleTaskletGroup` (Xbox PDB): a `BSTaskletGroup`, made by
        /// `BSTaskletManager::CreateTaskGroup` (`00b00a80`).
        0x128 ObstacleTaskletGroup: Inline<()>,
        /// `CurrentNavMeshTaskMap` (Xbox PDB): `NiTMap<unsigned int, ObstacleTaskData *>`.
        0x12c CurrentNavMeshTaskMap: Inline<NiTMap>,
        /// `BackgroundTasks` (Xbox PDB): `BSSimpleArray<ObstacleTaskData *, 1024>`.
        0x13c BackgroundTasks: Inline<BSSimpleArray>,
        /// `ProcessedTasks` (Xbox PDB): `BSSimpleArray<ObstacleTaskData *, 1024>`.
        0x14c ProcessedTasks: Inline<BSSimpleArray>,
        /// Built with `0096a2d0` (the list constructor) and never torn down;
        /// the Xbox PDB has a `ProcessedTaskLock` (`BSSpinLock`) between the
        /// tasks and the navmesh lists, which this may be.
        0x160 ProcessedTaskLock: Inline<()>,
        /// `QueuedListOfNavmeshesToEnable` (Xbox PDB): `BSSimpleList<unsigned int>`.
        0x180 QueuedListOfNavmeshesToEnable: Inline<BSSimpleList>,
        /// `QueuedListOfNavmeshesToDisable` (Xbox PDB): `BSSimpleList<unsigned int>`.
        0x188 QueuedListOfNavmeshesToDisable: Inline<BSSimpleList>,
        /// `eState` (Xbox PDB): `OBSTACLE_MANAGER_BACKGROUND_STATE`.
        0x190 eState: u32,
        /// `fTimeToNextSwap` (Xbox PDB).
        0x194 fTimeToNextSwap: f32,
        /// `bDrawObstacles` (Xbox PDB).
        0x198 bDrawObstacles: bool,
        /// `spObstacleRootNode` (Xbox PDB): `NiPointer<NiNode>`.
        0x19c spObstacleRootNode: u32,
        /// `MainThreadPerformaceTimer` (Xbox PDB), registered by the
        /// constructor with `BSPerformanceTimerManager::Register`.
        0x1a0 MainThreadPerformaceTimer: Inline<()>,
        /// `TaskletsPerformaceTimer` (Xbox PDB).
        0x1a1 TaskletsPerformaceTimer: Inline<()>,
    }

    /// `ReferenceObstacleArray` (Xbox PDB), `0x1c` bytes on both builds: a
    /// reference-counted base, the form id of the reference and the array of
    /// its obstacles.
    pub struct ReferenceObstacleArray: 0x1c {
        /// `iFormID` (Xbox PDB).
        0x08 iFormID: u32,
        /// `Obstacles` (Xbox PDB): `BSSimpleArray<NiPointer<ObstacleData>, 1024>`.
        0x0c Obstacles: Inline<BSSimpleArray>,
    }

    /// `ObstacleData` (Xbox PDB), `0x8c` bytes on PC (the allocation size in
    /// `006c2490`); the fields used so far, at PC offsets (the Xbox PDB's
    /// `+0x84`/`+0x88` are `0x10` lower here).
    pub struct ObstacleData: 0x8c {
        /// `spParentArray` (Xbox PDB): `ReferenceObstacleArray *`.
        0x08 spParentArray: u32,
        /// `pRigidBody` (Xbox PDB): `NiPointer<bhkRigidBody>`.
        0x0c pRigidBody: u32,
        /// `Center` (Xbox PDB): `NiPoint3`, the rigid body's position.
        0x10 Center: Inline<()>,
        /// `Orientation` (Xbox PDB): `NiMatrix3` (`0x24` bytes).
        0x1c Orientation: Inline<()>,
        /// `BoxMin` (Xbox PDB): `NiPoint3`, the corner of the box in the
        /// obstacle's own frame.
        0x40 BoxMin: Inline<()>,
        /// `BoxMax` (Xbox PDB): `NiPoint3`.
        0x4c BoxMax: Inline<()>,
        /// `aabbMin` (Xbox PDB): `NiPoint3`, the world-space bounds.
        0x58 aabbMin: Inline<()>,
        /// `aabbMax` (Xbox PDB): `NiPoint3`.
        0x64 aabbMax: Inline<()>,
        /// `iLastUpdateTime` (Xbox PDB): the word `006c2650` stores from
        /// `00825c00` on `011f6394`.
        0x70 iLastUpdateTime: u32,
        /// `bActive` (Xbox PDB).
        0x74 bActive: bool,
        /// `Navmeshes` (Xbox PDB): `BSSimpleArray<NavMeshInfo *, 1024>`.
        0x78 Navmeshes: Inline<BSSimpleArray>,
        /// `sp3DNode` (Xbox PDB): `NiPointer<NiTriShape>`, the debug bounding
        /// box.
        0x88 sp3DNode: u32,
    }

    /// `BSTasklet` (Xbox PDB), `0x8` bytes: a vtable and the `BSTaskletData`
    /// it runs. The vtable is the one at `0106c5d8`.
    pub struct BSTasklet: 0x08 {
        /// `pData` (Xbox PDB): `BSTaskletData *`.
        0x04 pData: u32,
    }

    /// `LockFreeQueue<bhkRigidBody *>` (Xbox PDB), as the PC build has it:
    /// the Xbox class is `0x1c` bytes; the PC object continues with a
    /// spin lock at `+0x20` (taken by the queue's `Push`/`Pop` and by
    /// `006c6700`) and takes `0x40` bytes inside the manager. Virtual slots
    /// `0x04` `AllocateInterface`, `0x08` `IncrementCount`, `0x0c`
    /// `DecrementCount`, `0x10` `GetCount`.
    pub struct LockFreeQueue: 0x40 {
        /// `pHead` (Xbox PDB): `LockFreeQueueNode *`.
        0x04 pHead: u32,
        /// `pTail` (Xbox PDB): `LockFreeQueueNode *`.
        0x08 pTail: u32,
        /// `iDeleteBatchSize` (Xbox PDB).
        0x0c iDeleteBatchSize: u32,
        /// `pReferencedNodes` (Xbox PDB): two words per interface.
        0x10 pReferencedNodes: u32,
        /// `pInterfaceManager` (Xbox PDB): `ThreadSpecificInterfaceManager *`
        /// (`0x10` bytes, built by `006c73b0`).
        0x14 pInterfaceManager: u32,
        /// `iCount` (Xbox PDB).
        0x18 iCount: u32,
    }

    /// `LockFreeQueue<bhkRigidBody *>::LockFreeQueueInterface` (Xbox PDB),
    /// `0x14` bytes: what one thread uses to push and pop.
    pub struct LockFreeQueueInterface: 0x14 {
        /// `pOwner` (Xbox PDB).
        0x00 pOwner: u32,
        /// `pReferencedNodes` (Xbox PDB): the address of the first word of
        /// this interface's pair of protected-node slots (the second slot is
        /// at `+8`).
        0x04 pReferencedNodes: u32,
        /// The second protected-node slot address (`+0x08`) of the pair
        /// `pReferencedNodes` starts (the Xbox PDB has `[2]` pointers at
        /// `+0x04`); the pop keeps the successor of the head in it.
        0x08 pReferencedNodesSecond: u32,
        /// `iDeleteCount` (Xbox PDB): nodes retired and not yet freed.
        0x0c iDeleteCount: u32,
        /// `pDeleteHead` (Xbox PDB): the first retired node.
        0x10 pDeleteHead: u32,
    }

    /// `ObstacleTaskData` (Xbox PDB), `0x58` bytes: a `BSXenonTaskletData`
    /// base (vtable, `bYielding`, `pLink`, `pGroupData`, `bRunOnStartup`,
    /// `TaskDataLock`; built by `006c7850`) and the fields below.
    pub struct ObstacleTaskData: 0x58 {
        /// `pInfo` (Xbox PDB): `NavMeshInfo *`.
        0x18 pInfo: u32,
        /// `spSrcMesh` (Xbox PDB): a navmesh holder (`NavMeshPtr`).
        0x1c spSrcMesh: Inline<()>,
        /// `spNewNavMesh` (Xbox PDB): a navmesh holder.
        0x20 spNewNavMesh: Inline<()>,
        /// `Operations` (Xbox PDB): `BSSimpleArray<ObstacleTaskNavMeshOperation, 1024>`.
        0x24 Operations: Inline<()>,
        /// `PortalSwaps` (Xbox PDB): a `TaskPortalSwap`.
        0x34 PortalSwaps: Inline<()>,
        /// `bPortalModified` (Xbox PDB).
        0x54 bPortalModified: bool,
    }

    /// One item of an `NiTMap` bucket chain (`NiTMapItem<K, V>`): next item,
    /// key, value.
    pub struct NiTMapItem: 0x0c {
        /// `m_pkNext`.
        0x00 m_pkNext: u32,
        /// `m_key`.
        0x04 m_key: u32,
        /// `m_val`.
        0x08 m_val: u32,
    }
}

/// The manager instance (a function-local static) and the word holding the
/// bit that says it has been constructed.
const MANAGER_INSTANCE: u32 = 0x011d_71e0;
const MANAGER_INSTANCE_GUARD: u32 = 0x011d_73a0;
/// The atexit destructor registered for the instance.
const MANAGER_ATEXIT_DESTRUCTOR: u32 = 0x00fd_7810;
/// `_atexit` (`cdecl`).
const ATEXIT: u32 = 0x00ec_658f;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\Pathfinding\NavMeshObstacleManager.cpp"`.
const SOURCE_PATH: u32 = 0x0106_c3f0;
/// Debug scope guard (a 4-byte local): constructor (`thiscall`: category
/// byte, flag, file name, line), destructor.
const SCOPE_GUARD_CTOR: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DTOR: u32 = 0x0040_4ee0;
const SCOPE_CATEGORY: u32 = 0x2e;
/// `BSPerformanceTimerManager::Register` (Xbox PDB), on the timer byte:
/// category string, timer name, two flags.
const PERFORMANCE_TIMER_REGISTER: u32 = 0x0045_d260;
/// `"Pathing"`, `"Obstacle Manager Update"`, `"Navmesh Background Update"`.
const PATHING_CATEGORY_NAME: u32 = 0x0103_a1bc;
const MAIN_TIMER_NAME: u32 = 0x0106_c468;
const TASKLET_TIMER_NAME: u32 = 0x0106_c44c;
/// `fTimeToNextSwap`'s initial value (`float`).
const INITIAL_TIME_TO_NEXT_SWAP: u32 = 0x0106_c398;

/// Vtables set by the constructors of the listeners and of
/// `ReferenceObstacleArray`.
const DEACTIVATION_LISTENER_VTABLE: u32 = 0x0106_c3b4;
const REMOVAL_LISTENER_VTABLE: u32 = 0x0106_c3d4;
const REFERENCE_OBSTACLE_ARRAY_VTABLE: u32 = 0x0106_c484;
/// Constructors of the bases of the listeners (`hkpEntityActivationListener`
/// and `hkpEntityListener`, as `thiscall` on the object) and the destructor
/// of the second.
const ACTIVATION_LISTENER_BASE_CTOR: u32 = 0x006c_0100;
const ENTITY_LISTENER_BASE_CTOR: u32 = 0x0061_f580;
const ENTITY_LISTENER_BASE_DTOR: u32 = 0x0055_4650;

/// `operator new(size)` and `operator delete(ptr)` (`cdecl`), and the
/// sized forms `00aa13e0(size)` / `00aa1460(ptr, size)` (`cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
const OPERATOR_NEW_SIZED: u32 = 0x00aa_13e0;
const OPERATOR_DELETE_SIZED: u32 = 0x00aa_1460;

/// `NiPointer<T>`: constructor from a raw pointer (`thiscall`, adds a
/// reference), release (drops one), `operator=(T *)` and
/// `operator=(const NiPointer &)`. The engine map names the last one
/// `NiPointer<PathingDebugGeometryData>::operator=`; it takes the address of
/// the source pointer.
const NI_POINTER_INIT: u32 = 0x0063_3c90;
const NI_POINTER_RELEASE: u32 = 0x0045_cec0;
const NI_POINTER_SET: u32 = 0x0066_b0d0;
const NI_POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// Copy constructor `NiPointer(const NiPointer &)`: takes the address of the
/// source pointer and adds a reference.
const NI_POINTER_COPY: u32 = 0x0055_9a40;
/// `NiPointer::operator->` / `operator T *`: the first word of the object.
const NI_POINTER_GET: u32 = 0x0055_9450;
/// `00822510`: the first word equals the argument.
const FIRST_WORD_EQUALS: u32 = 0x0082_2510;

/// `BSSimpleArray<T, 1024>`: size (`+8`), element address (`buffer + 4 *
/// index`, argument: the index), `Add(const T &)` (the engine map names
/// `007cb2e0` `AddUninitialized` of another instance; the body adds the
/// element at the argument's address), `Clear(bool)` (`008454f0`) and
/// `IsEmpty` (`0076b610`: the size is zero).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
const ARRAY_AT: u32 = 0x0087_7a30;
const ARRAY_ADD: u32 = 0x007c_b2e0;
const ARRAY_CLEAR: u32 = 0x0084_54f0;
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// `BSSimpleArray<NavMesh *, 1024>` local (`0x10` bytes): constructor and
/// destructor.
const NAVMESH_ARRAY_CTOR: u32 = 0x006c_6c60;
const NAVMESH_ARRAY_DTOR: u32 = 0x006c_6ce0;
/// `BSSimpleList<T>` on a head node: `Contains(const T &)`, `AddHead`,
/// `AddTail` and `Remove` (observed bodies; each takes the address of the
/// value), and `IsEmpty` (`004a4460`: no next node and no item).
const LIST_CONTAINS: u32 = 0x005f_65d0;
const LIST_ADD_HEAD: u32 = 0x005a_e3d0;
const LIST_ADD_TAIL: u32 = 0x0090_5820;
const LIST_REMOVE: u32 = 0x0090_5330;
const LIST_IS_EMPTY: u32 = 0x004a_4460;
/// `00631820`: the node's item differs from the value at the argument's
/// address. `004ee360`: the node's deleting destructor (flag 1 frees it).
const LIST_NODE_DIFFERS: u32 = 0x0063_1820;
const LIST_NODE_DELETE: u32 = 0x004e_e360;
/// `NiTMap<K, V>` instances of this unit: `GetAt(key, out)` (`006c62d0`, the
/// out argument is the address of a `NiPointer`; `AL` says whether the key
/// is present), `SetAt(key, value)` (`006c6920`, the value is an `NiPointer`
/// passed by value, which the callee destroys), `RemoveAt(key)` (`00405430`)
/// and the first item of the map (`004b9ba0`).
const MAP_GET: u32 = 0x006c_62d0;
const MAP_SET: u32 = 0x006c_6920;
const MAP_REMOVE: u32 = 0x0040_5430;
const MAP_FIRST_ITEM: u32 = 0x004b_9ba0;
/// `NiTMap` allocator: frees an item (`00659080`, argument: the item).
const MAP_FREE_ITEM: u32 = 0x0065_9080;
/// `TESForm::GetFormID` (`0084e3a0`: the word at `+0x0c`), the form lookup
/// by id (`004839c0`, `cdecl`), the base form of a reference (`007af430`:
/// the word at `+0x20`), the form type byte (`00401170`: the byte at `+4`)
/// and `TESObjectDOOR::IsSlidingDoor` (`00518080`).
const FORM_ID: u32 = 0x0084_e3a0;
const FORM_LOOKUP_BY_ID: u32 = 0x0048_39c0;
const REFERENCE_BASE_FORM: u32 = 0x007a_f430;
const FORM_TYPE: u32 = 0x0040_1170;
const IS_SLIDING_DOOR: u32 = 0x0051_8080;
/// The form type that `006c0dc0`, `006c0f10` and `006c1060` check before
/// asking `IsSlidingDoor`.
const DOOR_FORM_TYPE: u32 = 0x1c;
/// `00408d60` on the setting object at `011d73e4`: the address of its value
/// (`+4`); the obstacle queues do nothing while that byte is zero.
const SETTING_VALUE_ADDRESS: u32 = 0x0040_8d60;
const OBSTACLES_SETTING: u32 = 0x011d_73e4;
/// `004b5a80` (`cdecl`): the word at `+0x0c` of the entity a Havok listener
/// callback gets (null for null), which is the entity's user data.
const ENTITY_USER_DATA: u32 = 0x004b_5a80;
/// `004ae750` on a rigid body: the Havok entity it wraps (null for null),
/// and the two registrations on it (`hkpEntity` functions of
/// `hkpentity.cpp`; the argument is the listener). `bhkRigidBody::IsActive`
/// (Xbox PDB, `005609b0`).
const RIGID_BODY_ENTITY: u32 = 0x004a_e750;
const ENTITY_ADD_ACTIVATION_LISTENER: u32 = 0x00c9_c700;
const ENTITY_ADD_ENTITY_LISTENER: u32 = 0x00c9_c5b0;
const RIGID_BODY_IS_ACTIVE: u32 = 0x0056_09b0;
/// `BSTaskletManager`: the instance getter (`00b00a00`, no parameters),
/// `CreateTaskGroup`, `DestroyTaskGroup` (Xbox PDB) and the two group
/// functions `00b00ae0` / `00b00bc0`, all taking the group's address.
const TASKLET_MANAGER_INSTANCE: u32 = 0x00b0_0a00;
const TASKLET_CREATE_TASK_GROUP: u32 = 0x00b0_0a80;
const TASKLET_GROUP_CHECK: u32 = 0x00b0_0ae0;
const TASKLET_GROUP_FLUSH: u32 = 0x00b0_0bc0;
const TASKLET_DESTROY_TASK_GROUP: u32 = 0x00b0_0c20;
/// `006c5ef0(this, navmeshinfo *)`: the `ObstacleTaskData` for a navmesh
/// (made and entered into `CurrentNavMeshTaskMap` when missing; the map key
/// is the first word of the argument, `NavMeshInfo::NavMeshID`), and
/// `ObstacleTaskData::AddOperation` (Xbox PDB, `006c7a30`): the address of
/// the obstacle's `NiPointer` and the operation number.
const OBSTACLE_TASK_FOR_NAVMESH: u32 = 0x006c_5ef0;
const OBSTACLE_TASK_ADD_OPERATION: u32 = 0x006c_7a30;
/// `0068efb0` on a navmesh: the word at `+0x104` (its `NavMeshInfo`).
const NAVMESH_INFO: u32 = 0x0068_efb0;
/// Navmeshes overlapped by an obstacle: `006c2050(&obstacle, &array)`
/// (`cdecl`, `AL`).
const OBSTACLE_OVERLAPPING_NAVMESHES: u32 = 0x006c_2050;
/// `006c2650(obstacle)`, `006c3440(&obstacle)` and `006c3520(arg, &array)`
/// (all `cdecl`): obstacle updates of later sessions.
const OBSTACLE_PREPARE: u32 = 0x006c_2650;
const OBSTACLE_REFRESH: u32 = 0x006c_3440;
const OBSTACLE_ARRAY_REFRESH: u32 = 0x006c_3520;
/// Debug drawing: `DrawObstacles(this, flag)`, `AddBoundingBox3DNode(this,
/// &obstacle)` and the removal of a node (`006c5c50(this, &obstacle)`).
const DRAW_OBSTACLES: u32 = 0x006c_5460;
const ADD_BOUNDING_BOX_NODE: u32 = 0x006c_5b20;
const REMOVE_BOUNDING_BOX_NODE: u32 = 0x006c_5c50;
/// `006c1e00` (`this`, rigid body): not translated in this session.
const REMOVE_RIGID_BODY: u32 = 0x006c_1e00;
/// The lock-free queue operations: push (`006c5ff0`, address of the value)
/// and clear (`006c6700`, byte flag).
const QUEUE_PUSH: u32 = 0x006c_5ff0;
const QUEUE_CLEAR: u32 = 0x006c_6700;
/// A navmesh holder local (`0042fb00` constructs it, `0042fa40` destroys
/// it), the `NavMeshInfo` function filling one from `info + 0x54`
/// (`00690860(info, &holder)`: `AL` true when the info has a navmesh) and
/// a function on the navmesh (`006993c0`).
const NAVMESH_HOLDER_CTOR: u32 = 0x0042_fb00;
const NAVMESH_HOLDER_DTOR: u32 = 0x0042_fa40;
const NAVMESH_INFO_LOAD: u32 = 0x0069_0860;
const NAVMESH_UTIL_CALL: u32 = 0x0069_93c0;
/// `ReferenceObstacleArray` helpers: base constructor/destructor (`004968b0`
/// / `00496910`), `BSSimpleArray` constructor/destructor (`006c6160` /
/// `006c6190`).
const REFERENCE_COUNTED_CTOR: u32 = 0x0049_68b0;
const REFERENCE_COUNTED_DTOR: u32 = 0x0049_6910;
const OBSTACLE_ARRAY_CTOR: u32 = 0x006c_6160;
const OBSTACLE_ARRAY_DTOR: u32 = 0x006c_6190;
/// Fills a `NiPointer<ReferenceObstacleArray>` for a reference:
/// `006c34f0(reference, &pointer)` (`cdecl`).
const FILL_REFERENCE_OBSTACLES: u32 = 0x006c_34f0;
/// Constructors and destructors of the manager's members.
const CRITICAL_SECTION_CTOR: u32 = 0x0044_fae0;
const CRITICAL_SECTION_DTOR: u32 = 0x0044_fb00;
const MAP_CTOR: u32 = 0x006c_5fc0;
const MAP_DTOR: u32 = 0x006c_6350;
const TASK_MAP_CTOR: u32 = 0x006c_60a0;
const TASK_MAP_DTOR: u32 = 0x006c_6b80;
const RIGID_BODY_MAP_CTOR: u32 = 0x006c_6070;
const RIGID_BODY_MAP_DTOR: u32 = 0x006c_6a80;
const REFS_LIST_CTOR: u32 = 0x0096_a2d0;
const REFS_LIST_DTOR: u32 = 0x0046_ffb0;
const UPDATE_LIST_CTOR: u32 = 0x004e_e810;
const UPDATE_LIST_DTOR: u32 = 0x004e_e840;
const QUEUE_CTOR: u32 = 0x006c_6450;
const QUEUE_DTOR: u32 = 0x006c_6680;
const TASK_ARRAY_CTOR: u32 = 0x006c_6c10;
const TASK_ARRAY_DTOR: u32 = 0x006c_6c40;
const TASKLET_GROUP_CTOR: u32 = 0x0044_dee0;
const TASKLET_GROUP_DTOR: u32 = 0x0048_3710;
/// Clears of `ClearAllObstacles`: maps (`00438af0`), lists of ids
/// (`00470470`), the update list (`004ee920`).
const MAP_CLEAR: u32 = 0x0043_8af0;
const LIST_CLEAR: u32 = 0x0047_0470;
const UPDATE_LIST_CLEAR: u32 = 0x004e_e920;

// ---------------------------------------------------------------------------
// Constants of session 2 (`006c1e00`..`006c5b20`). The docs say what the body
// of each callee does, as observed in the disassembly.

/// The word at `011dea10` is the `TES` object. `TES::GetCurrentCell` (Xbox
/// PDB, `00457070`) is a `thiscall` on it; `00425fd0` is true when bit 0 of
/// the byte at `+0x24` of a cell is set (the code treats it as the cell being
/// an interior); `TESObjectCELL::GetWorldSpace` (Xbox PDB, `0054ddd0`) and
/// `TESWorldSpace::GetCellFromCellCoord` (Xbox PDB, `005875a0`, arguments the
/// two cell coordinates); `00450ff0` is true when `00450fd0` of a cell gives
/// 6. `TES::GetFormID`-like `0084e3a0` (the word at `+0x0c`) gives the root of
/// the scene graph when called on the `TES` object.
const TES_POINTER: u32 = 0x011d_ea10;
const TES_GET_CURRENT_CELL: u32 = 0x0045_7070;
const CELL_IS_INTERIOR: u32 = 0x0042_5fd0;
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
const WORLD_SPACE_GET_CELL_FROM_CELL_COORD: u32 = 0x0058_75a0;
const CELL_IS_IN_STATE_SIX: u32 = 0x0045_0ff0;
/// `006d93d0(min, max, obstacle_min, obstacle_max)` (`cdecl`, `AL`): a
/// two-point box overlap test of `pathfind.cpp` (the arguments are the
/// addresses of four `NiPoint3`).
const BOX_OVERLAP: u32 = 0x006d_93d0;
/// `_ftol2_sse` (`00ec62c0`): truncates the value the game holds in `ST0`; in
/// the uniform form a leading `f64`.
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// Float constants the code reads from `.rdata`: `-FLT_MAX` (`01015f5c`),
/// `FLT_MAX` (`01016970`), `4096.0` (`01017a3c`, a `float`), and the `double`
/// constants `4096.0` (`01017a10`), `128.0` (`0102e430`), `0.0` (`01012060`)
/// and `1000.0` (`01017b70`).
const NEGATIVE_FLOAT_MAX: u32 = 0x0101_5f5c;
const POSITIVE_FLOAT_MAX: u32 = 0x0101_6970;
const CELL_SIZE_FLOAT: u32 = 0x0101_7a3c;
const CELL_SIZE_DOUBLE: u32 = 0x0101_7a10;
const NAVMESH_TOP_MARGIN_DOUBLE: u32 = 0x0102_e430;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const MILLISECONDS_PER_SECOND_DOUBLE: u32 = 0x0101_7b70;
/// The `float` constants of this unit's `.rdata`: `2500.0` (`0106c3a0`, the
/// squared distance of two portal ends that may be joined), `48.0`
/// (`0106c3a4`), `16.0` (`0106c3a8`), `25.0` (`0106c39c`, a squared
/// distance) and `15.0` (`0106c3ac`).
const PORTAL_SQUARED_DISTANCE: u32 = 0x0106_c3a0;
const OBSTACLE_MIN_HEIGHT: u32 = 0x0106_c3a4;
const OBSTACLE_MIN_WIDTH: u32 = 0x0106_c3a8;
const OBSTACLE_MOVED_SQUARED_DISTANCE: u32 = 0x0106_c39c;
const BOX_EDGE_MARGIN: u32 = 0x0106_c3ac;
/// `0.25` (`0101622c`) and `0.5` (`01016248`): the debug box's alpha and the
/// fourth component of its colour.
const DEBUG_BOX_ALPHA: u32 = 0x0101_622c;
const DEBUG_BOX_COLOR_ALPHA: u32 = 0x0101_6248;
/// The `NiPoint3` at `011a9484`: the vector `(0, 0, 1)`.
const UP_VECTOR: u32 = 0x011a_9484;

/// `NiPoint3` and `NiMatrix3` members: the default constructor (`006815c0`,
/// does nothing and returns `this`), `NiPoint3(x, y, z)` (`00416870`),
/// `NiColorA(r, g, b, a)` (`00414430`), `a - b` and `a + b` (`00439ef0` /
/// `00439e90`: `this`, the result's address, the other operand's address),
/// `|v|^2` (`004a7290`, result in `ST0`) and `x*x + y*y` (`00595c80`), `-v` (`004a0bd0`: `this`, the result's
/// address), the unit cross product (`0053d1a0`) and the cross product
/// (`004b3800`) with the same arguments, `NiMatrix3 * NiPoint3` (`004b4500`),
/// `NiMatrix3 * NiMatrix3` (`0043f8d0`), the transpose (`004768c0`, the
/// result's address), `NiMatrix3::SetCol`-like `004769c0(index, &vector)`
/// and `NiMatrix3::GetCol`-like `00439f50(index, &vector)`, and
/// `vector * matrix` (`004b3ae0`, `cdecl`: result, vector, matrix).
const POINT3_DEFAULT: u32 = 0x0068_15c0;
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
const COLOR_CONSTRUCT: u32 = 0x0041_4430;
const POINT3_SUBTRACT: u32 = 0x0043_9ef0;
const POINT3_ADD: u32 = 0x0043_9e90;
const POINT3_SQUARED_LENGTH: u32 = 0x004a_7290;
const POINT3_SQUARED_LENGTH_XY: u32 = 0x0059_5c80;
const POINT3_NEGATE: u32 = 0x004a_0bd0;
const POINT3_UNIT_CROSS: u32 = 0x0053_d1a0;
const POINT3_CROSS: u32 = 0x004b_3800;
const MATRIX_TIMES_POINT: u32 = 0x004b_4500;
const MATRIX_TIMES_MATRIX: u32 = 0x0043_f8d0;
const MATRIX_TRANSPOSE: u32 = 0x0047_68c0;
const MATRIX_SET_COLUMN: u32 = 0x0047_69c0;
const MATRIX_GET_COLUMN: u32 = 0x0043_9f50;
const POINT_TIMES_MATRIX: u32 = 0x004b_3ae0;
/// `NiTransform` default constructor (`00476a80`), `bhkRigidBody::GetAabbLocal`
/// (Xbox PDB, `00c8d390`: the aabb as two 16-byte vectors),
/// `NiMatrix3::ToEulerAnglesXYZ` (Xbox PDB, `00a592c0`: three result
/// addresses), the two-`hkVector4` default constructor (`00437c10`), the
/// `hkVector4` to `NiPoint3` conversion (`00458620(result, vector)`,
/// `cdecl`) and the copy of an `hkVector4` (`004a3c90(this, &source)`).
const TRANSFORM_DEFAULT: u32 = 0x0047_6a80;
const RIGID_BODY_GET_AABB_LOCAL: u32 = 0x00c8_d390;
const MATRIX_TO_EULER_ANGLES: u32 = 0x00a5_92c0;
const AABB_DEFAULT: u32 = 0x0043_7c10;
const HK_VECTOR_TO_POINT3: u32 = 0x0045_8620;
const HK_VECTOR_COPY: u32 = 0x004a_3c90;
/// `00620b80`: the word at `+8` of `this` (a folded body: the Havok object of
/// a rigid body, the size of a `BSSimpleArray` such as a cell's navmesh
/// array). Then the transform source `004b4f00` of that Havok object (reads
/// through `009d9f40`), `00632b40` (made from it) and `00624ad0(transform,
/// &that)` (`cdecl`), which fills the `NiTransform` of the rigid body.
const GET_WORD_AT_8: u32 = 0x0062_0b80;
/// Virtual slot `0x94` of a rigid body (answers whether the obstacle can be
/// used; the exe's meaning of it is not confirmed).
const RIGID_BODY_SLOT_0X94: u32 = 0x94;
const HAVOK_TRANSFORM_SOURCE: u32 = 0x004b_4f00;
const HAVOK_TRANSFORM_BUILD: u32 = 0x0063_2b40;
const TRANSFORM_FROM_HAVOK: u32 = 0x0062_4ad0;
/// The rigid body's position (`004b4ec0` of the Havok object; a pointer to an
/// `hkVector4`).
const HAVOK_POSITION: u32 = 0x004b_4ec0;
/// `_vector_constructor_iterator_` (`00401050`, `stdcall`: array, element
/// size, count, constructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// `fabs` (`00408840`, `cdecl`, `float` on the stack, result in `ST0`), the
/// float remainder (`004b1500(x, y)`), `min` (`0040ebd0(a, b)`: `b` when `b <=
/// a`, else `a`) and `max` (`00404010(a, b)`: `a` when `b < a`, else `b`).
const FLOAT_ABS: u32 = 0x0040_8840;
const FLOAT_REMAINDER: u32 = 0x004b_1500;
const FLOAT_MIN: u32 = 0x0040_ebd0;
const FLOAT_MAX: u32 = 0x0040_4010;
/// `00825c00` on the object at `011f6394`: the word at `+0x14` (a time in
/// milliseconds); `0084d030` on the same object: the `float` at `+0x0c` (the
/// time of the last frame, in seconds).
const CLOCK_OBJECT: u32 = 0x011f_6394;
const CLOCK_MILLISECONDS: u32 = 0x0082_5c00;
const CLOCK_FRAME_SECONDS: u32 = 0x0084_d030;
/// The Havok entity of a rigid body (`004ae750`) and the two listener
/// removals on it, mirrors of `ENTITY_ADD_ACTIVATION_LISTENER` and
/// `ENTITY_ADD_ENTITY_LISTENER` (`hkpentity.cpp` in the engine map).
const ENTITY_REMOVE_ACTIVATION_LISTENER: u32 = 0x00c9_c0f0;
const ENTITY_REMOVE_ENTITY_LISTENER: u32 = 0x00c9_c670;
/// Scene-graph walking: `bhkCollisionObject::GetbhkCollisionObject` (Xbox
/// PDB, `0043b610`, `cdecl`, the collision object of a node or null), its
/// rigid body (`006fa820`: the pointer at `+0x10`), the `hkpRigidBody` motion
/// type of a rigid body (`0043b4f0(this, &out)` then `0043b4d0`, the low seven
/// bits of the first word) and the children of a node: the container returned
/// by virtual slot `0x0c` with its count (`0043b480`, or `00453470` in
/// `006c2550`) and its child at an index (`0043b4a0`).
const COLLISION_OBJECT_OF_NODE: u32 = 0x0043_b610;
const COLLISION_OBJECT_RIGID_BODY: u32 = 0x006f_a820;
const RIGID_BODY_MOTION_INFO: u32 = 0x0043_b4f0;
const MOTION_INFO_TYPE: u32 = 0x0043_b4d0;
const CHILDREN_COUNT: u32 = 0x0043_b480;
const CHILDREN_COUNT_SHALLOW: u32 = 0x0045_3470;
const CHILDREN_AT: u32 = 0x0043_b4a0;
/// Virtual slot `0x0c` of a node (its container of children) and slot
/// `0x1d0` of a reference (its 3D node; null when not loaded).
const NODE_CHILDREN_SLOT: u32 = 0x0c;
const REFERENCE_NODE_SLOT: u32 = 0x1d0;
/// `ObstacleData` construction: the allocation size (`0x8c`), the vtable
/// (`0106c490`), the array destructor (`0069af10`), the array constructor
/// (`0069aee0`) and `006c6200(array, 1)`, which empties a
/// `BSSimpleArray<NiPointer<ObstacleData>>` (the engine map names the folded
/// body `_DestructItems` of another instance). `006c61b0(array, &pointer)`
/// adds an `NiPointer`.
const OBSTACLE_DATA_SIZE: u32 = 0x8c;
const OBSTACLE_DATA_VTABLE: u32 = 0x0106_c490;
const OBSTACLE_NAVMESH_ARRAY_CTOR: u32 = 0x0069_aee0;
const OBSTACLE_NAVMESH_ARRAY_DTOR: u32 = 0x0069_af10;
const OBSTACLE_ARRAY_EMPTY: u32 = 0x006c_6200;
const OBSTACLE_ARRAY_ADD: u32 = 0x006c_61b0;
/// A cell array (`BSSimpleArray<TESObjectCELL *>`, `0x14` bytes) local:
/// constructor `006c6d40(this, 0x20, 0, 0)` and destructor `006c6e00`; the
/// navmesh array of a cell (`0070ec90`: the word at `+0x64`);
/// `NavMeshArray::GetNavMeshByIndex` (Xbox PDB, `00464f60(this, &holder,
/// index)`: fills a holder with a copy of the element and returns it);
/// `006969e0(navmesh, &low, &high)` (passes the two points to `00696a10` on the
/// member at `+0xa8`, which presumably fills them, and returns true); `0042f850(array, &holder)` adds a navmesh holder
/// to an array of holders; `005e04c0` / `005e04f0` construct / destroy a
/// `BSSimpleArray<TESObjectREFR *>` local.
const CELL_ARRAY_CTOR: u32 = 0x006c_6d40;
const CELL_ARRAY_DTOR: u32 = 0x006c_6e00;
const CELL_NAVMESHES: u32 = 0x0070_ec90;
const NAVMESH_ARRAY_GET_BY_INDEX: u32 = 0x0046_4f60;
const NAVMESH_BOUNDS: u32 = 0x0069_69e0;
const NAVMESH_HOLDER_ARRAY_ADD: u32 = 0x0042_f850;
const REFERENCE_ARRAY_CTOR: u32 = 0x005e_04c0;
const REFERENCE_ARRAY_DTOR: u32 = 0x005e_04f0;
/// The list traversal helpers: `008256d0` / `004a4460` (true for the empty
/// head node: no next node and no item), the node's own address (`006815c0`),
/// the next node (`00726070`: the word at `+4`).
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// `006c6030(queue, &out)` (`thiscall`, `AL`): pops a value from a
/// lock-free queue; virtual slot `0x10` of a queue gives its count.
const QUEUE_POP: u32 = 0x006c_6030;
const QUEUE_COUNT_SLOT: u32 = 0x10;
/// `006c6400(list, &pointer)`: `BSSimpleList<NiPointer<ObstacleData>>`
/// contains the pointer; `00631540(list, &pointer)`: adds it when the pointer is
/// not null and not already in.
const OBSTACLE_LIST_CONTAINS: u32 = 0x006c_6400;
const OBSTACLE_LIST_ADD: u32 = 0x0063_1540;
/// `006c85e0(task)` (`AL`): runs the finishing step of an
/// `ObstacleTaskData`; `006b7f20(map, &position, &key, &value)` is the
/// `GetNext` of `CurrentNavMeshTaskMap`; `006c5ce0(manager, task)` hands a
/// task to the tasklet group; `00438af0(map)` clears a map.
const OBSTACLE_TASK_FINISH: u32 = 0x006c_85e0;
const TASK_MAP_GET_NEXT: u32 = 0x006b_7f20;
const QUEUE_TASK: u32 = 0x006c_5ce0;
/// Locks: `BSCriticalSection::Enter` (`004538a0(this, &name)`) and `Leave`
/// (`004538c0`), `BSSpinLock::Lock` (`0040fbf0(this, &name)`) and `Unlock`
/// (`0040fba0`), `Sleep(0)` through its import (`0040fca0(0)`).
const CRITICAL_SECTION_ENTER: u32 = 0x0045_38a0;
const CRITICAL_SECTION_LEAVE: u32 = 0x0045_38c0;
const SPIN_LOCK_LOCK: u32 = 0x0040_fbf0;
const SPIN_LOCK_UNLOCK: u32 = 0x0040_fba0;
const SLEEP: u32 = 0x0040_fca0;
/// The performance timer step (`00483710`, does nothing) on
/// `MainThreadPerformaceTimer`, the strings given to the locks, and the
/// settings read by the update: `00408d60` on `011d73e4` gives the address of
/// a byte, `00403e20` on `011d71b0` the address of a `float` (seconds), and
/// `00408d60` on `011d71a0` the address of a byte.
const PERFORMANCE_TIMER_STEP: u32 = 0x0048_3710;
const UPDATE_LOCK_NAME: u32 = 0x0106_c498;
const PROCESSED_LOCK_NAME: u32 = 0x0106_c4bc;
const BACKGROUND_LOCK_NAME: u32 = 0x0106_c4ec;
const FLUSH_LOCK_NAME: u32 = 0x0106_c520;
const ADD_PROCESSED_LOCK_NAME: u32 = 0x0106_c5a8;
const FLOAT_SETTING_VALUE_ADDRESS: u32 = 0x0040_3e20;
const OBSTACLE_REFRESH_DELAY_SETTING: u32 = 0x011d_71b0;
const BACKGROUND_TASKS_SETTING: u32 = 0x011d_71a0;
/// The path manager: `PathManager::QInstance` (Xbox PDB, `0047d0b0`) and four
/// thiscalls on it (`006ebc70`, `006ebc90` returning a bool, `006ebcd0`,
/// `006ebc50`).
const PATH_MANAGER_INSTANCE: u32 = 0x0047_d0b0;
const PATH_MANAGER_START: u32 = 0x006e_bc70;
const PATH_MANAGER_IS_READY: u32 = 0x006e_bc90;
const PATH_MANAGER_FINISH: u32 = 0x006e_bcd0;
const PATH_MANAGER_WAIT: u32 = 0x006e_bc50;
/// The word at `011d6e2c` (`00552ba0`) is an object whose byte at `+0x210`
/// (`00552bb0`) says that navmesh drawing is on; `006a5ca0` redraws it.
const NAVMESH_RENDER_OBJECT: u32 = 0x0055_2ba0;
const NAVMESH_RENDER_ENABLED: u32 = 0x0055_2bb0;
const NAVMESH_RENDER_REDRAW: u32 = 0x006a_5ca0;
/// `NavMesh::SetDisableNavMesh` (Xbox PDB, `00694500(id, disable)`, `cdecl`).
const SET_DISABLE_NAVMESH: u32 = 0x0069_4500;
/// Navmesh geometry: `NavMesh::GetEdgeVertices` (Xbox PDB, `0068f040(this,
/// &out, triangle, edge)`: fills the two vertex addresses at `out` and returns
/// it), `NavMesh::GetMatchingEdge` (Xbox PDB, `0068f340(this, &edge, &out)`,
/// `AL`), `NavMesh::GetEdgeExtraInfo` (Xbox PDB, `0068f230(this, triangle,
/// edge)`), the triangle count of a navmesh (`00558200`), a triangle of it
/// (`005582f0(index)`), the triangle's flag test (`00691140(mask)`), the
/// edge-to-triangle record (`00691020(&record)`), `00690e30(this, edge,
/// value)` (sets bit `edge` of the triangle's flags and the neighbour number
/// of that edge), the default constructor of the local edge record
/// (`006923c0`, sets the first word to `-1`) and `0069a660(this, a, b)`
/// (fills a record).
const NAVMESH_EDGE_VERTICES: u32 = 0x0068_f040;
const NAVMESH_MATCHING_EDGE: u32 = 0x0068_f340;
const NAVMESH_EDGE_EXTRA_INFO: u32 = 0x0068_f230;
const NAVMESH_TRIANGLE_COUNT: u32 = 0x0055_8200;
const NAVMESH_TRIANGLE: u32 = 0x0055_82f0;
const TRIANGLE_HAS_FLAG: u32 = 0x0069_1140;
const NAVMESH_EDGE_RECORD_INDEX: u32 = 0x0069_1020;
const TRIANGLE_LINK_EDGE: u32 = 0x0069_0e30;
const EDGE_RECORD_DEFAULT: u32 = 0x0069_23c0;
const EDGE_RECORD_SET: u32 = 0x0069_a660;
/// `BSSimpleArray` of 12-byte records: the element address (`006a1440`,
/// `buffer + 12 * index`) and the removal of `count` elements from an index
/// (`00691c90(this, index, count)`).
const RECORD_ARRAY_AT: u32 = 0x006a_1440;
const RECORD_ARRAY_REMOVE: u32 = 0x0069_1c90;
/// The error text logged by `005b5e40` (`cdecl`) for a bad triangle index.
const LOG_MESSAGE: u32 = 0x005b_5e40;
const BAD_TRIANGLE_MESSAGE: u32 = 0x0106_c548;
/// The navmesh in a task or `NavMeshInfo` (`00458b50`: the pointer held at
/// the address given), `0042f4c0(holder, &source)` assigns a navmesh holder,
/// and `0069ad00(info, &holder)` loads the info's navmesh into a holder
/// (true when there is one).
const HOLDER_POINTER: u32 = 0x0045_8b50;
const NAVMESH_HOLDER_ASSIGN: u32 = 0x0042_f4c0;
const NAVMESH_INFO_LOAD_HOLDER: u32 = 0x0069_ad00;
/// Debug bounding box: `NiNode` constructor (`00a5ecb0(this, 0)`), the alpha
/// property (`004391c0`, size `0x1c`; `0049ed90(property, 1)` turns it on),
/// the material property (`00a75650`, size `0x4c`), its emissive colour
/// (`004bc450`) and alpha (`0068c9f0`), `NiAVObject::AttachProperty` (Xbox
/// PDB, `00439410`), `NiAVObject::UpdateProperties` (Xbox PDB, `00a5a040`),
/// `NiUpdateData(time, flag, flag)` (`0043d410`), the update of a node with
/// it (`00a59c60(node, &data)`), the node's child attach (virtual slot
/// `0xdc`) and detach (`0xe8`), the removal of properties (`004def90`),
/// `MakeQuadBox` (Xbox PDB, `004b2eb0`, `cdecl`, ten arguments), and the
/// placement of a shape (`00440460(shape, &center)`, `0043fa80(shape,
/// &orientation)`).
const NODE_CONSTRUCT: u32 = 0x00a5_ecb0;
const NODE_SIZE: u32 = 0xac;
const ALPHA_PROPERTY_CONSTRUCT: u32 = 0x0043_91c0;
const ALPHA_PROPERTY_SIZE: u32 = 0x1c;
const ALPHA_PROPERTY_ENABLE: u32 = 0x0049_ed90;
const MATERIAL_PROPERTY_CONSTRUCT: u32 = 0x00a7_5650;
const MATERIAL_PROPERTY_SIZE: u32 = 0x4c;
const MATERIAL_SET_COLOR: u32 = 0x004b_c450;
const MATERIAL_SET_ALPHA: u32 = 0x0068_c9f0;
const ATTACH_PROPERTY: u32 = 0x0043_9410;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
const NODE_UPDATE: u32 = 0x00a5_9c60;
const NODE_ATTACH_CHILD_SLOT: u32 = 0xdc;
const NODE_DETACH_CHILD_SLOT: u32 = 0xe8;
const NODE_REMOVE_PROPERTIES: u32 = 0x004d_ef90;
const MAKE_QUAD_BOX: u32 = 0x004b_2eb0;
const SHAPE_SET_CENTER: u32 = 0x0044_0460;
const SHAPE_SET_ORIENTATION: u32 = 0x0043_fa80;

/// The word of game memory at `address` (an `NiPointer` slot, a field).
fn word(e: &Engine, address: u32) -> u32 {
    e.mem.u32(address)
}

/// `NiPointer::operator->`: the pointer held in the slot.
fn pointer_get(e: &mut Engine, slot: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![slot]).u32()
}

fn pointer_init(e: &mut Engine, slot: u32, value: u32) {
    e.call(NI_POINTER_INIT, &args![slot, value]);
}

fn pointer_release(e: &mut Engine, slot: u32) {
    e.call(NI_POINTER_RELEASE, &args![slot]);
}

/// `NiPointer(const NiPointer &)` into a fresh slot.
fn pointer_copy(e: &mut Engine, slot: u32, source: u32) {
    e.call(NI_POINTER_COPY, &args![slot, source]);
}

/// `BSSimpleArray::GetSize`.
fn array_size(e: &mut Engine, array: u32) -> u32 {
    e.call(ARRAY_SIZE, &args![array]).u32()
}

/// `BSSimpleArray::operator[]`: the address of element `index`.
fn array_at(e: &mut Engine, array: u32, index: u32) -> u32 {
    e.call(ARRAY_AT, &args![array, index]).u32()
}

/// The `Obstacles` array of the `ReferenceObstacleArray` held by `slot`.
fn obstacles_of(e: &mut Engine, slot: u32) -> u32 {
    pointer_get(e, slot) + ReferenceObstacleArray::Obstacles.off
}

/// The `Navmeshes` array of the `ObstacleData` held by `slot`.
fn navmeshes_of(e: &mut Engine, slot: u32) -> u32 {
    pointer_get(e, slot) + ObstacleData::Navmeshes.off
}

/// Number of obstacles in the `ReferenceObstacleArray` held by `slot`.
fn obstacle_count(e: &mut Engine, slot: u32) -> u32 {
    let obstacles = obstacles_of(e, slot);
    array_size(e, obstacles)
}

/// Number of navmeshes recorded by the `ObstacleData` held by `slot`.
fn navmesh_count(e: &mut Engine, slot: u32) -> u32 {
    let navmeshes = navmeshes_of(e, slot);
    array_size(e, navmeshes)
}

/// The rigid body of the `ObstacleData` held by `slot`.
fn rigid_body_of(e: &mut Engine, slot: u32) -> u32 {
    let obstacle = pointer_get(e, slot);
    pointer_get(e, obstacle + ObstacleData::pRigidBody.off)
}

/// Whether the `ObstacleData` held by `slot` is active.
fn obstacle_is_active(e: &mut Engine, slot: u32) -> bool {
    let obstacle = pointer_get(e, slot);
    e.mem.u8(obstacle + ObstacleData::bActive.off) != 0
}

/// `list.Contains(value)` for a word value (the game passes its address).
fn list_contains(e: &mut Engine, list: u32, value: u32) -> bool {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(LIST_CONTAINS, &args![list, slot]).bool()
    })
}

/// A list or queue operation that takes the address of a word value.
fn list_operation(e: &mut Engine, operation: u32, list: u32, value: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        e.call(operation, &args![list, slot]);
    })
}

/// `map.GetAt(key, &out)`: true when the key is present; the value is put
/// in the `NiPointer` at `out`.
fn map_get(e: &mut Engine, map: u32, key: u32, out: u32) -> bool {
    e.call(MAP_GET, &args![map, key, out]).bool()
}

/// `map.SetAt(key, value)` with the value a fresh copy of the `NiPointer` at
/// `source` (the game builds the by-value argument on its stack with the
/// copy constructor; `SetAt` destroys it).
fn map_set_copy(e: &mut Engine, map: u32, key: u32, source: u32) {
    e.with_stack(4, |e, temporary| {
        pointer_copy(e, temporary.addr(), source);
        let value = word(e, temporary.addr());
        e.call(MAP_SET, &args![map, key, value]);
    })
}

/// The value of the setting at `011d73e4` (a byte); the queues do nothing
/// while it is zero.
fn obstacles_enabled(e: &mut Engine) -> bool {
    let value = e
        .call(SETTING_VALUE_ADDRESS, &args![OBSTACLES_SETTING])
        .u32();
    e.mem.u8(value) != 0
}

/// A scope guard around `body`, as `00404eb0` / `00404ee0` make it.
fn with_scope_guard<R>(e: &mut Engine, line: u32, body: impl FnOnce(&mut Engine) -> R) -> R {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, SCOPE_CATEGORY, 1u32, SOURCE_PATH, line],
        );
        let result = body(e);
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
        result
    })
}

/// `ObstacleTaskData::AddOperation` on the task of `nav_mesh_info`.
fn queue_operation(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
    nav_mesh_info: u32,
    obstacle: u32,
    operation: u32,
) {
    let task = e
        .call(OBSTACLE_TASK_FOR_NAVMESH, &args![this, nav_mesh_info])
        .u32();
    e.call(
        OBSTACLE_TASK_ADD_OPERATION,
        &args![task, obstacle, operation],
    );
}

/// For an obstacle that is active: asks `006c2050` for the navmeshes it
/// overlaps; for each one queues `operation` on its task and records its
/// `NavMeshInfo` in the obstacle's `Navmeshes`.
fn queue_overlapping_navmeshes(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
    obstacle: u32,
    operation: u32,
) {
    // The navmesh scrap array is 0x14 bytes (006c6c60 sets its scrap heap at +0x10; EBP-0x2c in
    // 006c1190, EBP-0x24 in 006c1760), one word more than the 0x10 this frame had.
    e.with_stack(0x14, |e, found| {
        let found = found.addr();
        e.call(NAVMESH_ARRAY_CTOR, &args![found]);
        if e.call(OBSTACLE_OVERLAPPING_NAVMESHES, &args![obstacle, found])
            .bool()
        {
            let mut index = 0;
            while index < array_size(e, found) {
                let element = array_at(e, found, index);
                let navmesh = pointer_get(e, element);
                let info = e.call(NAVMESH_INFO, &args![navmesh]).u32();
                queue_operation(e, this, info, obstacle, operation);
                let element = array_at(e, found, index);
                let navmesh = pointer_get(e, element);
                let info = e.call(NAVMESH_INFO, &args![navmesh]).u32();
                let navmeshes = navmeshes_of(e, obstacle);
                list_operation(e, ARRAY_ADD, navmeshes, info);
                index += 1;
            }
        }
        e.call(NAVMESH_ARRAY_DTOR, &args![found]);
    });
}

/// Queues `operation` on the task of every `NavMeshInfo` in the obstacle's
/// `Navmeshes`, then clears that array (`Clear(1)`).
fn queue_recorded_navmeshes(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
    obstacle: u32,
    operation: u32,
) {
    let mut index = 0;
    while index < navmesh_count(e, obstacle) {
        let navmeshes = navmeshes_of(e, obstacle);
        let element = array_at(e, navmeshes, index);
        let info = word(e, element);
        queue_operation(e, this, info, obstacle, operation);
        index += 1;
    }
    let navmeshes = navmeshes_of(e, obstacle);
    e.call(ARRAY_CLEAR, &args![navmeshes, 1u32]);
}

// Translated from 00559120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned int, NiPointer<ReferenceObstacleArray>>>, unsigned int,
/// NiPointer<ReferenceObstacleArray>>::GetNext` (Xbox PDB): reads the item
/// at `*position`, gives its key and (through `NiPointer::operator=`) its
/// value, and moves `*position` to the next item: the rest of the bucket
/// chain, else the first item of a later bucket (the bucket of the key comes
/// from the map's hash function, vtable slot `+4`), else null.
pub fn ni_tmap_base_reference_obstacle_array_get_next(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    position: Ptr,
    key_out: Ptr,
    value_out: Ptr,
) {
    let item: Ptr<NiTMapItem> = Ptr::new(word(e, position.addr()));
    let key = e.get(item, NiTMapItem::m_key);
    e.mem.set_u32(key_out.addr(), key);
    e.call(
        NI_POINTER_ASSIGN,
        &args![value_out, item.addr() + NiTMapItem::m_val.off],
    );
    let next = e.get(item, NiTMapItem::m_pkNext);
    if next != 0 {
        e.mem.set_u32(position.addr(), next);
        return;
    }
    let mut index = e.vcall(this.addr(), 4, &args![key]).u32();
    loop {
        index = index.wrapping_add(1);
        if index >= e.get(this, NiTPointerMap::m_uiHashSize) {
            e.mem.set_u32(position.addr(), 0);
            return;
        }
        let table = e.get(this, NiTPointerMap::m_ppkHashTable);
        let candidate = word(e, table + index * 4);
        if candidate != 0 {
            e.mem.set_u32(position.addr(), candidate);
            return;
        }
    }
}

// Translated from 00631620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleList<NiPointer<ObstacleData>>::Remove` (Xbox PDB): removes the
/// first node whose item equals the pointer at `item`. Does nothing when
/// that pointer is null or the list is empty. The head node stays; when it
/// is the match it takes the next node's item and that node is freed.
pub fn bs_simple_list_obstacle_data_remove(e: &mut Engine, this: Ptr<BSSimpleList>, item: Ptr) {
    if e.call(FIRST_WORD_EQUALS, &args![item, 0u32]).bool() {
        return;
    }
    if e.call(LIST_IS_EMPTY, &args![this]).bool() {
        return;
    }
    let head = this.addr();
    let mut previous = head;
    let mut node = head;
    while node != 0 && e.call(LIST_NODE_DIFFERS, &args![node, item]).bool() {
        previous = node;
        node = e.mem.u32(node + BSSimpleList::m_pkNext.off);
    }
    if node == 0 {
        return;
    }
    if node == head {
        let next = e.get(this, BSSimpleList::m_pkNext);
        if next == 0 {
            e.call(NI_POINTER_SET, &args![this, 0u32]);
        } else {
            let after = e.mem.u32(next + BSSimpleList::m_pkNext.off);
            e.set(this, BSSimpleList::m_pkNext, after);
            e.call(NI_POINTER_ASSIGN, &args![this, next]);
            e.mem.set_u32(next + BSSimpleList::m_pkNext.off, 0);
            e.call(LIST_NODE_DELETE, &args![next, 1u32]);
        }
    } else {
        let after = e.mem.u32(node + BSSimpleList::m_pkNext.off);
        e.mem.set_u32(previous + BSSimpleList::m_pkNext.off, after);
        e.mem.set_u32(node + BSSimpleList::m_pkNext.off, 0);
        e.call(LIST_NODE_DELETE, &args![node, 1u32]);
    }
}

// Translated from 00691a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned int, NiPointer<ReferenceObstacleArray>>::DeleteItem`
/// (Xbox PDB): releases the item's value and frees the item through the
/// map's allocator (`+0x0c`).
pub fn ni_tmap_reference_obstacle_array_delete_item(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    item: Ptr<NiTMapItem>,
) {
    e.call(
        NI_POINTER_SET,
        &args![item.addr() + NiTMapItem::m_val.off, 0u32],
    );
    e.call(MAP_FREE_ITEM, &args![this.addr() + 0x0c, item]);
}

// Translated from 006a63e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<bhkRigidBody *, NiPointer<ObstacleData>>>,
/// bhkRigidBody *, NiPointer<ObstacleData>>::SetValue` (Xbox PDB): stores the
/// key and assigns the value to the item. The value is an `NiPointer` passed
/// by value, which the function destroys at its end.
pub fn ni_tmap_base_rigid_body_obstacle_data_set_value(
    e: &mut Engine,
    _this: Ptr<NiTMap>,
    item: Ptr<NiTMapItem>,
    key: u32,
    value: u32,
) {
    e.set(item, NiTMapItem::m_key, key);
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), value);
        e.call(
            NI_POINTER_ASSIGN,
            &args![item.addr() + NiTMapItem::m_val.off, argument],
        );
        e.call(NI_POINTER_RELEASE, &args![argument]);
    });
}

// Translated from 006c00b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `bhkObstacleDeactivationListener` (engine map: unnamed):
/// the base constructor `006c0100`, then the class's vtable. Returns `this`.
pub fn fn_006c00b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(ACTIVATION_LISTENER_BASE_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), DEACTIVATION_LISTENER_VTABLE);
    this
}

// Translated from 006c0190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkObstacleDeactivationListener::entityDeactivatedCallback` (Xbox PDB):
/// for an entity with user data, `006c1c90` on the manager with that data.
pub fn bhk_obstacle_deactivation_listener_entity_deactivated_callback(
    e: &mut Engine,
    _this: Ptr,
    entity: u32,
) {
    let user_data = e.call(ENTITY_USER_DATA, &args![entity]).u32();
    if user_data != 0 {
        let manager = fn_006c0720(e);
        fn_006c1c90(e, manager, user_data);
    }
}

// Translated from 006c01d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkObstacleDeactivationListener::entityActivatedCallback` (Xbox PDB):
/// for an entity with user data, `006c1c70` on the manager with that data.
pub fn bhk_obstacle_deactivation_listener_entity_activated_callback(
    e: &mut Engine,
    _this: Ptr,
    entity: u32,
) {
    let user_data = e.call(ENTITY_USER_DATA, &args![entity]).u32();
    if user_data != 0 {
        let manager = fn_006c0720(e);
        fn_006c1c70(e, manager, user_data);
    }
}

// Translated from 006c0210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `bhkObstacleRemovalListener` (engine map: unnamed): the
/// base constructor `0061f580`, then the class's vtable. Returns `this`.
pub fn fn_006c0210(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(ENTITY_LISTENER_BASE_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), REMOVAL_LISTENER_VTABLE);
    this
}

// Translated from 006c0230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `bhkObstacleRemovalListener` (engine map:
/// unnamed): the destructor, then `operator delete` when bit 0 of `flags` is
/// set. Returns `this`.
pub fn fn_006c0230(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006c0260(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006c0260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `bhkObstacleRemovalListener` (engine map: unnamed): sets the
/// class's vtable and runs the base destructor (`00554650`).
pub fn fn_006c0260(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), REMOVAL_LISTENER_VTABLE);
    e.call(ENTITY_LISTENER_BASE_DTOR, &args![this]);
}

// Translated from 006c0280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkObstacleRemovalListener::entityRemovedCallback` (Xbox PDB): for an
/// entity with user data, `OnObstacleLost3D` on the manager with that data.
pub fn bhk_obstacle_removal_listener_entity_removed_callback(
    e: &mut Engine,
    _this: Ptr,
    entity: u32,
) {
    let user_data = e.call(ENTITY_USER_DATA, &args![entity]).u32();
    if user_data != 0 {
        let manager = fn_006c0720(e);
        nav_mesh_obstacle_manager_on_obstacle_lost3d(e, manager, user_data);
    }
}

// Translated from 006c02c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleManager::NavMeshObstacleManager` (Xbox PDB): builds every
/// member (critical section, the maps with `0x25` buckets, the lists, the two
/// lock-free queues with `(7, 8)`, the tasklet group, a null
/// `spObstacleRootNode`), registers the two performance timers
/// (`"Pathing"`: `"Obstacle Manager Update"` and `"Navmesh Background
/// Update"`), then, inside a debug scope (line `0xb8`), clears the listener
/// pointers, the draw flag, the state and the update-all flag and sets
/// `fTimeToNextSwap` from the float at `0106c398`. Returns `this`.
pub fn nav_mesh_obstacle_manager_nav_mesh_obstacle_manager(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
) -> Ptr<NavMeshObstacleManager> {
    let base = this.addr();
    e.call(CRITICAL_SECTION_CTOR, &args![this]);
    e.call(MAP_CTOR, &args![base + 0x1c, 0x25u32]);
    e.call(REFS_LIST_CTOR, &args![base + 0x2c]);
    e.call(REFS_LIST_CTOR, &args![base + 0x34]);
    e.call(UPDATE_LIST_CTOR, &args![base + 0x3c]);
    e.call(QUEUE_CTOR, &args![base + 0x60, 7u32, 8u32]);
    e.call(QUEUE_CTOR, &args![base + 0xa0, 7u32, 8u32]);
    e.call(RIGID_BODY_MAP_CTOR, &args![base + 0xe0, 0x25u32]);
    e.call(REFS_LIST_CTOR, &args![base + 0xf0]);
    e.call(REFS_LIST_CTOR, &args![base + 0xf8]);
    e.call(MAP_CTOR, &args![base + 0x100, 0x25u32]);
    e.call(MAP_CTOR, &args![base + 0x110, 0x25u32]);
    e.call(TASKLET_GROUP_CTOR, &args![base + 0x128]);
    e.call(TASK_MAP_CTOR, &args![base + 0x12c, 0x25u32]);
    e.call(TASK_ARRAY_CTOR, &args![base + 0x13c]);
    e.call(TASK_ARRAY_CTOR, &args![base + 0x14c]);
    e.call(REFS_LIST_CTOR, &args![base + 0x160]);
    e.call(REFS_LIST_CTOR, &args![base + 0x180]);
    e.call(REFS_LIST_CTOR, &args![base + 0x188]);
    e.call(NI_POINTER_INIT, &args![base + 0x19c, 0u32]);
    e.call(
        PERFORMANCE_TIMER_REGISTER,
        &args![
            base + 0x1a0,
            PATHING_CATEGORY_NAME,
            MAIN_TIMER_NAME,
            1u32,
            1u32
        ],
    );
    e.call(
        PERFORMANCE_TIMER_REGISTER,
        &args![
            base + 0x1a1,
            PATHING_CATEGORY_NAME,
            TASKLET_TIMER_NAME,
            0u32,
            1u32
        ],
    );
    with_scope_guard(e, 0xb8, |e| {
        e.set(this, NavMeshObstacleManager::pListener, 0);
        e.set(this, NavMeshObstacleManager::pRemoveListener, 0);
        e.set(this, NavMeshObstacleManager::bDrawObstacles, false);
        e.set(this, NavMeshObstacleManager::eState, 0);
        let time_to_next_swap = e.global::<f32>(INITIAL_TIME_TO_NEXT_SWAP);
        e.set(
            this,
            NavMeshObstacleManager::fTimeToNextSwap,
            time_to_next_swap,
        );
        e.set(this, NavMeshObstacleManager::bUpdateAllObstacles, false);
    });
    this
}

// Translated from 006c0510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `NavMeshObstacleManager` (engine map: unnamed): clears all
/// obstacles, deletes the two listeners (virtual deleting destructor, slot 0,
/// flag 1; the fields keep the dead pointers), then destroys the members in
/// reverse order. The two performance timers and the lock at `+0x160` are
/// not destroyed.
pub fn fn_006c0510(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    nav_mesh_obstacle_manager_clear_all_obstacles(e, this);
    for field in [
        NavMeshObstacleManager::pListener,
        NavMeshObstacleManager::pRemoveListener,
    ] {
        let listener = e.get(this, field);
        if listener != 0 {
            e.vcall(listener, 0, &args![1u32]);
        }
    }
    let base = this.addr();
    e.call(NI_POINTER_RELEASE, &args![base + 0x19c]);
    e.call(REFS_LIST_DTOR, &args![base + 0x188]);
    e.call(REFS_LIST_DTOR, &args![base + 0x180]);
    e.call(TASK_ARRAY_DTOR, &args![base + 0x14c]);
    e.call(TASK_ARRAY_DTOR, &args![base + 0x13c]);
    e.call(TASK_MAP_DTOR, &args![base + 0x12c]);
    e.call(TASKLET_GROUP_DTOR, &args![base + 0x128]);
    e.call(MAP_DTOR, &args![base + 0x110]);
    e.call(MAP_DTOR, &args![base + 0x100]);
    e.call(REFS_LIST_DTOR, &args![base + 0xf8]);
    e.call(REFS_LIST_DTOR, &args![base + 0xf0]);
    e.call(RIGID_BODY_MAP_DTOR, &args![base + 0xe0]);
    e.call(QUEUE_DTOR, &args![base + 0xa0]);
    e.call(QUEUE_DTOR, &args![base + 0x60]);
    e.call(UPDATE_LIST_DTOR, &args![base + 0x3c]);
    e.call(REFS_LIST_DTOR, &args![base + 0x34]);
    e.call(REFS_LIST_DTOR, &args![base + 0x2c]);
    e.call(MAP_DTOR, &args![base + 0x1c]);
    e.call(CRITICAL_SECTION_DTOR, &args![this]);
}

// Translated from 006c0720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The manager's accessor (engine map: unnamed; the function-local static of
/// `NavMeshObstacleManager::GetInstance`): inside a debug scope (line
/// `0xd2`), constructs the instance at `011d71e0` on first use (guard bit 0
/// of `011d73a0`) and registers its `atexit` destructor. Takes no
/// parameters (callers push a word that stays on the stack for their next
/// call).
pub fn fn_006c0720(e: &mut Engine) -> Ptr<NavMeshObstacleManager> {
    with_scope_guard(e, 0xd2, |e| {
        let guard = e.global::<u32>(MANAGER_INSTANCE_GUARD);
        if guard & 1 == 0 {
            let guard = e.global::<u32>(MANAGER_INSTANCE_GUARD);
            e.set_global(MANAGER_INSTANCE_GUARD, guard | 1);
            nav_mesh_obstacle_manager_nav_mesh_obstacle_manager(e, Ptr::new(MANAGER_INSTANCE));
            e.call(ATEXIT, &args![MANAGER_ATEXIT_DESTRUCTOR]);
        }
    });
    Ptr::new(MANAGER_INSTANCE)
}

// Translated from 006c07d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleManager::Initialize` (Xbox PDB): creates the tasklet group
/// (`BSTaskletManager::CreateTaskGroup`), checks it (`006c08e0`), then makes
/// the two listeners (`0x4` bytes each). Returns false when the group
/// cannot be created or checked.
pub fn nav_mesh_obstacle_manager_initialize(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
) -> bool {
    let group = this.addr() + NavMeshObstacleManager::ObstacleTaskletGroup.off;
    let manager = e.call(TASKLET_MANAGER_INSTANCE, &[]).u32();
    if !e
        .call(TASKLET_CREATE_TASK_GROUP, &args![manager, group])
        .bool()
    {
        return false;
    }
    if !fn_006c08e0(e, Ptr::new(group)) {
        return false;
    }
    let memory = e.call(OPERATOR_NEW, &args![4u32]).u32();
    let listener = if memory != 0 {
        fn_006c00b0(e, Ptr::new(memory)).addr()
    } else {
        0
    };
    e.set(this, NavMeshObstacleManager::pListener, listener);
    let memory = e.call(OPERATOR_NEW, &args![4u32]).u32();
    let remove_listener = if memory != 0 {
        fn_006c0210(e, Ptr::new(memory)).addr()
    } else {
        0
    };
    e.set(
        this,
        NavMeshObstacleManager::pRemoveListener,
        remove_listener,
    );
    true
}

// Translated from 006c08e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Check of the manager's tasklet group (engine map: unnamed): the result of
/// `00b00ae0` on the tasklet manager with the group's address.
pub fn fn_006c08e0(e: &mut Engine, this: Ptr) -> bool {
    let manager = e.call(TASKLET_MANAGER_INSTANCE, &[]).u32();
    e.call(TASKLET_GROUP_CHECK, &args![manager, this]).bool()
}

// Translated from 006c0900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleManager::Release` (Xbox PDB): clears all obstacles,
/// deletes the two listeners (virtual deleting destructor) and nulls the
/// fields, runs `006c09d0` on the tasklet group and destroys it
/// (`BSTaskletManager::DestroyTaskGroup`).
pub fn nav_mesh_obstacle_manager_release(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    nav_mesh_obstacle_manager_clear_all_obstacles(e, this);
    for field in [
        NavMeshObstacleManager::pListener,
        NavMeshObstacleManager::pRemoveListener,
    ] {
        let listener = e.get(this, field);
        if listener != 0 {
            e.vcall(listener, 0, &args![1u32]);
            e.set(this, field, 0);
        }
    }
    let group = this.addr() + NavMeshObstacleManager::ObstacleTaskletGroup.off;
    fn_006c09d0(e, Ptr::new(group));
    let manager = e.call(TASKLET_MANAGER_INSTANCE, &[]).u32();
    e.call(TASKLET_DESTROY_TASK_GROUP, &args![manager, group]);
}

// Translated from 006c09d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flush of the manager's tasklet group (engine map: unnamed): `00b00bc0` on
/// the tasklet manager with the group's address.
pub fn fn_006c09d0(e: &mut Engine, this: Ptr) {
    let manager = e.call(TASKLET_MANAGER_INSTANCE, &[]).u32();
    e.call(TASKLET_GROUP_FLUSH, &args![manager, this]);
}

// Translated from 006c09f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleManager::ClearAllObstacles` (Xbox PDB): walks `FormIDMap`
/// (`GetNext`), and for every obstacle of every reference array whose form id
/// is not queued for removal calls `006c1e00` with its rigid body; for each
/// navmesh the obstacle recorded asks it to load (`00690860`) and runs
/// `006993c0` on the loaded navmesh. Then clears the maps (`FormIDMap`, both
/// door maps, the rigid body map), the id lists, the update list and the two
/// queues, and, when `bDrawObstacles` is set, calls `DrawObstacles(0)`.
pub fn nav_mesh_obstacle_manager_clear_all_obstacles(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
) {
    let base = this.addr();
    let form_id_map = base + NavMeshObstacleManager::FormIDMap.off;
    let queued_to_remove = base + NavMeshObstacleManager::QueuedListOfRefsToRemove.off;
    e.with_stack(0x20, |e, frame| {
        let cursor = frame.addr();
        let key = frame.addr() + 4;
        let current = frame.addr() + 8;
        let obstacle = frame.addr() + 0x0c;
        let holder = frame.addr() + 0x10;
        pointer_init(e, current, 0);
        let first = e.call(MAP_FIRST_ITEM, &args![form_id_map]).u32();
        e.mem.set_u32(cursor, first);
        while word(e, cursor) != 0 {
            ni_tmap_base_reference_obstacle_array_get_next(
                e,
                Ptr::new(form_id_map),
                Ptr::new(cursor),
                Ptr::new(key),
                Ptr::new(current),
            );
            let mut index = 0;
            while index < obstacle_count(e, current) {
                let obstacles = obstacles_of(e, current);
                let element = array_at(e, obstacles, index);
                pointer_copy(e, obstacle, element);
                if !e.call(LIST_CONTAINS, &args![queued_to_remove, key]).bool() {
                    let rigid_body = rigid_body_of(e, obstacle);
                    e.call(REMOVE_RIGID_BODY, &args![this, rigid_body]);
                }
                let mut nav_index = 0;
                while nav_index < navmesh_count(e, obstacle) {
                    e.call(NAVMESH_HOLDER_CTOR, &args![holder]);
                    let navmeshes = navmeshes_of(e, obstacle);
                    let element = array_at(e, navmeshes, nav_index);
                    let info = word(e, element);
                    if e.call(NAVMESH_INFO_LOAD, &args![info, holder]).bool() {
                        let navmesh = pointer_get(e, holder);
                        e.call(NAVMESH_UTIL_CALL, &args![navmesh]);
                    }
                    e.call(NAVMESH_HOLDER_DTOR, &args![holder]);
                    nav_index += 1;
                }
                pointer_release(e, obstacle);
                index += 1;
            }
        }
        pointer_release(e, current);
    });
    for field in [
        NavMeshObstacleManager::FormIDMap.off,
        NavMeshObstacleManager::OpenDoorMap.off,
        NavMeshObstacleManager::ClosedDoorMap.off,
        NavMeshObstacleManager::RigidBodyToObstacleMap.off,
    ] {
        e.call(MAP_CLEAR, &args![base + field]);
    }
    for field in [
        NavMeshObstacleManager::QueuedListOfDoorsToAdd.off,
        NavMeshObstacleManager::QueuedListOfDoorsToRemove.off,
        NavMeshObstacleManager::QueuedListOfRefsToAdd.off,
        NavMeshObstacleManager::QueuedListOfRefsToRemove.off,
    ] {
        e.call(LIST_CLEAR, &args![base + field]);
    }
    e.call(
        UPDATE_LIST_CLEAR,
        &args![base + NavMeshObstacleManager::ObstaclesUpdateList.off],
    );
    e.call(
        QUEUE_CLEAR,
        &args![
            base + NavMeshObstacleManager::ObstaclesToAddToUpdateList.off,
            0u32
        ],
    );
    e.call(
        QUEUE_CLEAR,
        &args![
            base + NavMeshObstacleManager::ObstaclesToRemoveFromUpdateList.off,
            0u32
        ],
    );
    if e.get(this, NavMeshObstacleManager::bDrawObstacles) {
        e.call(DRAW_OBSTACLES, &args![this, 0u32]);
    }
}

// Translated from 006c0c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleManager::AddObstacleForReference` (Xbox PDB): when the
/// obstacle setting (`011d73e4`) is on and the reference is not in
/// `QueuedListOfRefsToAdd` yet, adds it at the head.
pub fn nav_mesh_obstacle_manager_add_obstacle_for_reference(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
    reference: u32,
) {
    if !obstacles_enabled(e) {
        return;
    }
    let queue = this.addr() + NavMeshObstacleManager::QueuedListOfRefsToAdd.off;
    if !list_contains(e, queue, reference) {
        list_operation(e, LIST_ADD_HEAD, queue, reference);
    }
}

// Translated from 006c0c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleManager::RemoveObstacleForReference` (Xbox PDB): when the
/// obstacle setting is on, takes the reference out of
/// `QueuedListOfRefsToAdd`; unless its form id is already queued for removal,
/// asks `006c1e00` to remove the rigid body of every obstacle recorded for
/// it in `FormIDMap` (when there is an entry) and queues the form id in
/// `QueuedListOfRefsToRemove` (`AddTail`).
pub fn nav_mesh_obstacle_manager_remove_obstacle_for_reference(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
    reference: u32,
) {
    if !obstacles_enabled(e) {
        return;
    }
    let base = this.addr();
    list_operation(
        e,
        LIST_REMOVE,
        base + NavMeshObstacleManager::QueuedListOfRefsToAdd.off,
        reference,
    );
    let queued_to_remove = base + NavMeshObstacleManager::QueuedListOfRefsToRemove.off;
    let form_id = e.call(FORM_ID, &args![reference]).u32();
    if list_contains(e, queued_to_remove, form_id) {
        return;
    }
    e.with_stack(4, |e, array| {
        let array = array.addr();
        pointer_init(e, array, 0);
        let form_id = e.call(FORM_ID, &args![reference]).u32();
        if map_get(
            e,
            base + NavMeshObstacleManager::FormIDMap.off,
            form_id,
            array,
        ) {
            let mut index = 0;
            while index < obstacle_count(e, array) {
                let obstacles = obstacles_of(e, array);
                let element = array_at(e, obstacles, index);
                let obstacle = pointer_get(e, element);
                let rigid_body = pointer_get(e, obstacle + ObstacleData::pRigidBody.off);
                e.call(REMOVE_RIGID_BODY, &args![this, rigid_body]);
                index += 1;
            }
            let form_id = e.call(FORM_ID, &args![reference]).u32();
            list_operation(e, LIST_ADD_TAIL, queued_to_remove, form_id);
        }
        pointer_release(e, array);
    });
}

/// True for a reference whose base form is a door that is not a sliding
/// door (the test `006c0dc0`, `006c0f10` and `006c1060` open with): the
/// form type is `0x1c` and `IsSlidingDoor` is false.
fn is_ordinary_door(e: &mut Engine, reference: u32) -> bool {
    let form = e.call(REFERENCE_BASE_FORM, &args![reference]).u32();
    if e.call(FORM_TYPE, &args![form]).u32() == DOOR_FORM_TYPE {
        return !e.call(IS_SLIDING_DOOR, &args![form]).bool();
    }
    false
}

// Translated from 006c0dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Door handler (engine map: unnamed, between `RemoveObstacleForReference`
/// and `OnDoorClose`): for a non-sliding door reference calls
/// `AddObstacleForReference`; then, unless the reference is in
/// `ClosedDoorMap`, takes it out of `QueuedListOfDoorsToAdd` when it is
/// there; then, unless it is in `OpenDoorMap`, queues its form id in
/// `QueuedListOfDoorsToRemove` (`AddHead`) when not queued yet. That this is
/// the "door opened" handler is inferred from its neighbour `OnDoorClose`
/// (Xbox PDB) and not confirmed.
pub fn fn_006c0dc0(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, reference: u32) {
    let base = this.addr();
    if is_ordinary_door(e, reference) {
        nav_mesh_obstacle_manager_add_obstacle_for_reference(e, this, reference);
    }
    e.with_stack(4, |e, array| {
        let array = array.addr();
        pointer_init(e, array, 0);
        let form_id = e.call(FORM_ID, &args![reference]).u32();
        let in_closed = map_get(
            e,
            base + NavMeshObstacleManager::ClosedDoorMap.off,
            form_id,
            array,
        );
        let doors_to_add = base + NavMeshObstacleManager::QueuedListOfDoorsToAdd.off;
        if !in_closed && list_contains(e, doors_to_add, reference) {
            list_operation(e, LIST_REMOVE, doors_to_add, reference);
        }
        let form_id = e.call(FORM_ID, &args![reference]).u32();
        let in_open = map_get(
            e,
            base + NavMeshObstacleManager::OpenDoorMap.off,
            form_id,
            array,
        );
        if !in_open {
            let queued = base + NavMeshObstacleManager::QueuedListOfDoorsToRemove.off;
            let form_id = e.call(FORM_ID, &args![reference]).u32();
            if !list_contains(e, queued, form_id) {
                let form_id = e.call(FORM_ID, &args![reference]).u32();
                list_operation(e, LIST_ADD_HEAD, queued, form_id);
            }
        }
        pointer_release(e, array);
    });
}

// Translated from 006c0f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleManager::OnDoorClose` (Xbox PDB): for a non-sliding door
/// reference calls `RemoveObstacleForReference`; then, unless the reference is
/// in `OpenDoorMap`, takes its form id out of `QueuedListOfDoorsToRemove`
/// when it is there, and, unless the reference is in `ClosedDoorMap`, queues
/// the reference in `QueuedListOfDoorsToAdd` (`AddHead`) when it is not there.
pub fn nav_mesh_obstacle_manager_on_door_close(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
    reference: u32,
) {
    let base = this.addr();
    if is_ordinary_door(e, reference) {
        nav_mesh_obstacle_manager_remove_obstacle_for_reference(e, this, reference);
    }
    e.with_stack(4, |e, array| {
        let array = array.addr();
        pointer_init(e, array, 0);
        let form_id = e.call(FORM_ID, &args![reference]).u32();
        let in_open = map_get(
            e,
            base + NavMeshObstacleManager::OpenDoorMap.off,
            form_id,
            array,
        );
        if !in_open {
            let queued = base + NavMeshObstacleManager::QueuedListOfDoorsToRemove.off;
            let form_id = e.call(FORM_ID, &args![reference]).u32();
            if list_contains(e, queued, form_id) {
                let form_id = e.call(FORM_ID, &args![reference]).u32();
                list_operation(e, LIST_REMOVE, queued, form_id);
            }
        }
        let form_id = e.call(FORM_ID, &args![reference]).u32();
        let in_closed = map_get(
            e,
            base + NavMeshObstacleManager::ClosedDoorMap.off,
            form_id,
            array,
        );
        if !in_closed {
            let queued = base + NavMeshObstacleManager::QueuedListOfDoorsToAdd.off;
            if !list_contains(e, queued, reference) {
                list_operation(e, LIST_ADD_HEAD, queued, reference);
            }
        }
        pointer_release(e, array);
    });
}

// Translated from 006c1060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Door-gone handler (engine map: unnamed): for a non-sliding door reference
/// calls `RemoveObstacleForReference`; then takes the reference out of
/// `QueuedListOfRefsToAdd`; when it is in `QueuedListOfDoorsToAdd` takes it
/// out of there, else queues its form id in `QueuedListOfDoorsToRemove`
/// (`AddHead`) unless already queued.
pub fn fn_006c1060(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, reference: u32) {
    let base = this.addr();
    if is_ordinary_door(e, reference) {
        nav_mesh_obstacle_manager_remove_obstacle_for_reference(e, this, reference);
    }
    list_operation(
        e,
        LIST_REMOVE,
        base + NavMeshObstacleManager::QueuedListOfRefsToAdd.off,
        reference,
    );
    let doors_to_add = base + NavMeshObstacleManager::QueuedListOfDoorsToAdd.off;
    if list_contains(e, doors_to_add, reference) {
        list_operation(e, LIST_REMOVE, doors_to_add, reference);
    } else {
        let queued = base + NavMeshObstacleManager::QueuedListOfDoorsToRemove.off;
        let form_id = e.call(FORM_ID, &args![reference]).u32();
        if !list_contains(e, queued, form_id) {
            let form_id = e.call(FORM_ID, &args![reference]).u32();
            list_operation(e, LIST_ADD_HEAD, queued, form_id);
        }
    }
}

// Translated from 006c1130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues a navmesh form id in `QueuedListOfNavmeshesToEnable` (engine map:
/// unnamed): `AddHead` of the value (no duplicate check).
pub fn fn_006c1130(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, navmesh_id: u32) {
    let list = this.addr() + NavMeshObstacleManager::QueuedListOfNavmeshesToEnable.off;
    list_operation(e, LIST_ADD_HEAD, list, navmesh_id);
}

// Translated from 006c1150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues a navmesh form id in `QueuedListOfNavmeshesToDisable` (engine map:
/// unnamed): `AddHead` of the value (no duplicate check).
pub fn fn_006c1150(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, navmesh_id: u32) {
    let list = this.addr() + NavMeshObstacleManager::QueuedListOfNavmeshesToDisable.off;
    list_operation(e, LIST_ADD_HEAD, list, navmesh_id);
}

// Translated from 006c1170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Requests an update of all obstacles (engine map: unnamed): sets
/// `bUpdateAllObstacles`.
pub fn fn_006c1170(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    e.set(this, NavMeshObstacleManager::bUpdateAllObstacles, true);
}

// Translated from 006c1190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the obstacles of a reference (engine map: unnamed): finds or builds
/// the reference's `ReferenceObstacleArray` in `FormIDMap` (a new one is
/// filled by `006c34f0`); then, for each of its obstacles: enters it in
/// `RigidBodyToObstacleMap` under its rigid body, registers the Havok
/// listeners (`006c1d60`), queues the rigid body when it is active
/// (`006c1c70`), refreshes the obstacle (`006c3440`), adds its debug node
/// when `bDrawObstacles` is set and, when the obstacle is active, queues
/// operation `0x10` on the task of every navmesh it overlaps and records
/// those navmeshes in its `Navmeshes`.
pub fn fn_006c1190(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, reference: u32) {
    let base = this.addr();
    let form_id_map = base + NavMeshObstacleManager::FormIDMap.off;
    e.with_stack(0x10, |e, frame| {
        let array = frame.addr();
        let obstacle = frame.addr() + 4;
        pointer_init(e, array, 0);
        let form_id = e.call(FORM_ID, &args![reference]).u32();
        if !map_get(e, form_id_map, form_id, array) {
            let memory = e.call(OPERATOR_NEW_SIZED, &args![0x1cu32]).u32();
            let created = if memory != 0 {
                reference_obstacle_array_construct(e, Ptr::new(memory)).addr()
            } else {
                0
            };
            e.call(NI_POINTER_SET, &args![array, created]);
            e.call(FILL_REFERENCE_OBSTACLES, &args![reference, array]);
            let form_id = e.call(FORM_ID, &args![reference]).u32();
            map_set_copy(e, form_id_map, form_id, array);
        }
        let mut index = 0;
        while index < obstacle_count(e, array) {
            let obstacles = obstacles_of(e, array);
            let element = array_at(e, obstacles, index);
            pointer_copy(e, obstacle, element);
            let rigid_body = rigid_body_of(e, obstacle);
            map_set_copy(
                e,
                base + NavMeshObstacleManager::RigidBodyToObstacleMap.off,
                rigid_body,
                obstacle,
            );
            let rigid_body = rigid_body_of(e, obstacle);
            fn_006c1d60(e, this, rigid_body);
            let rigid_body = rigid_body_of(e, obstacle);
            if e.call(RIGID_BODY_IS_ACTIVE, &args![rigid_body]).bool() {
                let rigid_body = rigid_body_of(e, obstacle);
                fn_006c1c70(e, this, rigid_body);
            }
            e.call(OBSTACLE_REFRESH, &args![obstacle]);
            if e.get(this, NavMeshObstacleManager::bDrawObstacles) {
                e.call(ADD_BOUNDING_BOX_NODE, &args![this, obstacle]);
            }
            if obstacle_is_active(e, obstacle) {
                queue_overlapping_navmeshes(e, this, obstacle, 0x10);
            }
            pointer_release(e, obstacle);
            index += 1;
        }
        pointer_release(e, array);
    });
}

// Translated from 006c1480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ReferenceObstacleArray` (engine map: unnamed): the
/// reference-counted base constructor (`004968b0`), the class's vtable, the
/// `Obstacles` array constructor (`006c6160`) and a zero `iFormID`. Returns
/// `this`.
pub fn reference_obstacle_array_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(REFERENCE_COUNTED_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), REFERENCE_OBSTACLE_ARRAY_VTABLE);
    e.call(
        OBSTACLE_ARRAY_CTOR,
        &args![this.addr() + ReferenceObstacleArray::Obstacles.off],
    );
    e.mem
        .set_u32(this.addr() + ReferenceObstacleArray::iFormID.off, 0);
    this
}

// Translated from 006c14f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReferenceObstacleArray::_scalar_deleting_destructor_` (Xbox PDB): the
/// destructor, then the sized `operator delete` (`0x1c` bytes) when bit 0 of
/// `flags` is set. Returns `this`.
pub fn reference_obstacle_array_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    reference_obstacle_array_destructor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_SIZED, &args![this, 0x1cu32]);
    }
    this
}

// Translated from 006c1520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ReferenceObstacleArray::~ReferenceObstacleArray` (Xbox PDB): the class's
/// vtable, a zero `iFormID`, the `Obstacles` array destructor (`006c6190`)
/// and the base destructor (`00496910`).
pub fn reference_obstacle_array_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), REFERENCE_OBSTACLE_ARRAY_VTABLE);
    e.mem
        .set_u32(this.addr() + ReferenceObstacleArray::iFormID.off, 0);
    e.call(
        OBSTACLE_ARRAY_DTOR,
        &args![this.addr() + ReferenceObstacleArray::Obstacles.off],
    );
    e.call(REFERENCE_COUNTED_DTOR, &args![this]);
}

// Translated from 006c1590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the obstacles of a reference by form id (engine map: unnamed): if
/// `FormIDMap` has the id, removes the entry and, for each of its
/// obstacles, the `RigidBodyToObstacleMap` entry of its rigid body, the node
/// in `ObstaclesUpdateList`, the debug node when `bDrawObstacles` is set and,
/// when it recorded navmeshes, queues operation `0x20` on each of their tasks
/// and clears the record.
pub fn fn_006c1590(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, form_id: u32) {
    let base = this.addr();
    e.with_stack(8, |e, frame| {
        let array = frame.addr();
        let obstacle = frame.addr() + 4;
        pointer_init(e, array, 0);
        if !map_get(
            e,
            base + NavMeshObstacleManager::FormIDMap.off,
            form_id,
            array,
        ) {
            pointer_release(e, array);
            return;
        }
        e.call(
            MAP_REMOVE,
            &args![base + NavMeshObstacleManager::FormIDMap.off, form_id],
        );
        let mut index = 0;
        while index < obstacle_count(e, array) {
            let obstacles = obstacles_of(e, array);
            let element = array_at(e, obstacles, index);
            pointer_copy(e, obstacle, element);
            let rigid_body = rigid_body_of(e, obstacle);
            e.call(
                MAP_REMOVE,
                &args![
                    base + NavMeshObstacleManager::RigidBodyToObstacleMap.off,
                    rigid_body
                ],
            );
            bs_simple_list_obstacle_data_remove(
                e,
                Ptr::new(base + NavMeshObstacleManager::ObstaclesUpdateList.off),
                Ptr::new(obstacle),
            );
            if e.get(this, NavMeshObstacleManager::bDrawObstacles) {
                e.call(REMOVE_BOUNDING_BOX_NODE, &args![this, obstacle]);
            }
            let navmeshes = navmeshes_of(e, obstacle);
            if !e.call(ARRAY_IS_EMPTY, &args![navmeshes]).bool() {
                queue_recorded_navmeshes(e, this, obstacle, 0x20);
            }
            pointer_release(e, obstacle);
            index += 1;
        }
        pointer_release(e, array);
    });
}

// Translated from 006c1760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Re-registers one obstacle (engine map: unnamed; `obstacle` is the address
/// of its `NiPointer<ObstacleData>`): for an active obstacle queues operation
/// `0x21` on the task of each navmesh it recorded and clears the record;
/// then refreshes it (`006c2650`, `006c3440`), adds its debug node when
/// `bDrawObstacles` is set and, when it is active, queues operation `0x11` on
/// the tasks of the navmeshes it now overlaps and records them.
pub fn fn_006c1760(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, obstacle: u32) {
    if obstacle_is_active(e, obstacle) {
        queue_recorded_navmeshes(e, this, obstacle, 0x21);
    }
    let data = pointer_get(e, obstacle);
    e.call(OBSTACLE_PREPARE, &args![data]);
    e.call(OBSTACLE_REFRESH, &args![obstacle]);
    if e.get(this, NavMeshObstacleManager::bDrawObstacles) {
        e.call(ADD_BOUNDING_BOX_NODE, &args![this, obstacle]);
    }
    if obstacle_is_active(e, obstacle) {
        queue_overlapping_navmeshes(e, this, obstacle, 0x11);
    }
}

// Translated from 006c1920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the obstacles of a reference (engine map: unnamed). If
/// `FormIDMap` has `form_id`: for each old obstacle queues operation `0x21`
/// on the tasks of its recorded navmeshes (when it is active, then clears the
/// record), removes its debug node when `bDrawObstacles` is set, calls
/// `006c1e00` for its rigid body and removes the entry from
/// `RigidBodyToObstacleMap` and the node from `ObstaclesUpdateList`. Then
/// `006c3520(other, &array)` refills the array and for each of its obstacles:
/// refreshes it (`006c3440`), queues operation `0x11` on the navmeshes it
/// overlaps (when active), adds its debug node when `bDrawObstacles` is set
/// and registers the Havok listeners (`006c1d60`). `other` is the second
/// parameter, passed unchanged to `006c3520`.
pub fn fn_006c1920(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, form_id: u32, other: u32) {
    let base = this.addr();
    e.with_stack(8, |e, frame| {
        let array = frame.addr();
        let obstacle = frame.addr() + 4;
        pointer_init(e, array, 0);
        if !map_get(
            e,
            base + NavMeshObstacleManager::FormIDMap.off,
            form_id,
            array,
        ) {
            pointer_release(e, array);
            return;
        }
        let mut index = 0;
        while index < obstacle_count(e, array) {
            let obstacles = obstacles_of(e, array);
            let element = array_at(e, obstacles, index);
            pointer_copy(e, obstacle, element);
            if obstacle_is_active(e, obstacle) {
                queue_recorded_navmeshes(e, this, obstacle, 0x21);
            }
            if e.get(this, NavMeshObstacleManager::bDrawObstacles) {
                e.call(REMOVE_BOUNDING_BOX_NODE, &args![this, obstacle]);
            }
            let rigid_body = rigid_body_of(e, obstacle);
            e.call(REMOVE_RIGID_BODY, &args![this, rigid_body]);
            let rigid_body = rigid_body_of(e, obstacle);
            e.call(
                MAP_REMOVE,
                &args![
                    base + NavMeshObstacleManager::RigidBodyToObstacleMap.off,
                    rigid_body
                ],
            );
            bs_simple_list_obstacle_data_remove(
                e,
                Ptr::new(base + NavMeshObstacleManager::ObstaclesUpdateList.off),
                Ptr::new(obstacle),
            );
            pointer_release(e, obstacle);
            index += 1;
        }
        e.call(OBSTACLE_ARRAY_REFRESH, &args![other, array]);
        let mut index = 0;
        while index < obstacle_count(e, array) {
            let obstacles = obstacles_of(e, array);
            let element = array_at(e, obstacles, index);
            pointer_copy(e, obstacle, element);
            e.call(OBSTACLE_REFRESH, &args![obstacle]);
            if obstacle_is_active(e, obstacle) {
                queue_overlapping_navmeshes(e, this, obstacle, 0x11);
            }
            if e.get(this, NavMeshObstacleManager::bDrawObstacles) {
                e.call(ADD_BOUNDING_BOX_NODE, &args![this, obstacle]);
            }
            let rigid_body = rigid_body_of(e, obstacle);
            fn_006c1d60(e, this, rigid_body);
            pointer_release(e, obstacle);
            index += 1;
        }
        pointer_release(e, array);
    });
}

// Translated from 006c1c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pushes a rigid body on `ObstaclesToAddToUpdateList` (engine map: unnamed;
/// the lock-free queue push `006c5ff0` takes the value's address).
pub fn fn_006c1c70(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, rigid_body: u32) {
    let queue = this.addr() + NavMeshObstacleManager::ObstaclesToAddToUpdateList.off;
    list_operation(e, QUEUE_PUSH, queue, rigid_body);
}

// Translated from 006c1c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pushes a rigid body on `ObstaclesToRemoveFromUpdateList` (engine map:
/// unnamed).
pub fn fn_006c1c90(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, rigid_body: u32) {
    let queue = this.addr() + NavMeshObstacleManager::ObstaclesToRemoveFromUpdateList.off;
    list_operation(e, QUEUE_PUSH, queue, rigid_body);
}

// Translated from 006c1cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleManager::OnObstacleLost3D` (Xbox PDB): looks the rigid
/// body up in `RigidBodyToObstacleMap`; for an obstacle found, takes the form
/// id of its parent `ReferenceObstacleArray`, looks the form up (`cdecl`
/// `004839c0`) and, when it exists, calls `RemoveObstacleForReference` with
/// it.
pub fn nav_mesh_obstacle_manager_on_obstacle_lost3d(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
    rigid_body: u32,
) {
    e.with_stack(4, |e, obstacle| {
        let obstacle = obstacle.addr();
        pointer_init(e, obstacle, 0);
        let map = this.addr() + NavMeshObstacleManager::RigidBodyToObstacleMap.off;
        if map_get(e, map, rigid_body, obstacle) {
            let data = pointer_get(e, obstacle);
            let parent = word(e, data + ObstacleData::spParentArray.off);
            let form_id = word(e, parent + ReferenceObstacleArray::iFormID.off);
            let form = e.call(FORM_LOOKUP_BY_ID, &args![form_id]).u32();
            if form != 0 {
                nav_mesh_obstacle_manager_remove_obstacle_for_reference(e, this, form);
            }
        }
        pointer_release(e, obstacle);
    });
}

// Translated from 006c1d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers the manager's two listeners on a rigid body's Havok entity
/// (engine map: unnamed): `006c1dd0` with `pListener`, then `006c1da0` with
/// `pRemoveListener`.
pub fn fn_006c1d60(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, rigid_body: u32) {
    let listener = e.get(this, NavMeshObstacleManager::pListener);
    fn_006c1dd0(e, Ptr::new(rigid_body), listener);
    let remove_listener = e.get(this, NavMeshObstacleManager::pRemoveListener);
    fn_006c1da0(e, Ptr::new(rigid_body), remove_listener);
}

// Translated from 006c1da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds an entity listener to a rigid body's Havok entity (engine map:
/// unnamed): `00c9c5b0` on the entity (`004ae750` of the rigid body) when
/// there is one.
pub fn fn_006c1da0(e: &mut Engine, this: Ptr, listener: u32) {
    let entity = e.call(RIGID_BODY_ENTITY, &args![this]).u32();
    if entity != 0 {
        e.call(ENTITY_ADD_ENTITY_LISTENER, &args![entity, listener]);
    }
}

// Translated from 006c1dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds an activation listener to a rigid body's Havok entity (engine map:
/// unnamed): `00c9c700` on the entity (`004ae750` of the rigid body) when
/// there is one.
pub fn fn_006c1dd0(e: &mut Engine, this: Ptr, listener: u32) {
    let entity = e.call(RIGID_BODY_ENTITY, &args![this]).u32();
    if entity != 0 {
        e.call(ENTITY_ADD_ACTIVATION_LISTENER, &args![entity, listener]);
    }
}

// Translated from 006c1e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the manager's two Havok listeners from a rigid body (engine map:
/// unnamed), the mirror of [`fn_006c1d60`]: `006c1e70` with `pListener`, then
/// `006c1e40` with `pRemoveListener`. `rigid_body` is the stack argument.
pub fn fn_006c1e00(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, rigid_body: u32) {
    let listener = e.get(this, NavMeshObstacleManager::pListener);
    fn_006c1e70(e, Ptr::new(rigid_body), listener);
    let remove_listener = e.get(this, NavMeshObstacleManager::pRemoveListener);
    fn_006c1e40(e, Ptr::new(rigid_body), remove_listener);
}

// Translated from 006c1e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes an entity listener from a rigid body's Havok entity (engine map:
/// unnamed): `00c9c670` on the entity (`004ae750` of the rigid body) when
/// there is one. The mirror of [`fn_006c1da0`].
pub fn fn_006c1e40(e: &mut Engine, this: Ptr, listener: u32) {
    let entity = e.call(RIGID_BODY_ENTITY, &args![this]).u32();
    if entity != 0 {
        e.call(ENTITY_REMOVE_ENTITY_LISTENER, &args![entity, listener]);
    }
}

// Translated from 006c1e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes an activation listener from a rigid body's Havok entity (engine
/// map: unnamed): `00c9c0f0` on the entity (`004ae750` of the rigid body) when
/// there is one. The mirror of [`fn_006c1dd0`].
pub fn fn_006c1e70(e: &mut Engine, this: Ptr, listener: u32) {
    let entity = e.call(RIGID_BODY_ENTITY, &args![this]).u32();
    if entity != 0 {
        e.call(ENTITY_REMOVE_ACTIVATION_LISTENER, &args![entity, listener]);
    }
}

/// `a < b` as the x87 compare says it: false when either is not a number.
fn is_less(a: f32, b: f32) -> bool {
    a.partial_cmp(&b) == Some(std::cmp::Ordering::Less)
}

/// `_ftol2_sse` on a `float` the game holds in `ST0`: the integer part.
fn float_to_int(e: &mut Engine, value: f32) -> i32 {
    e.call(FLOAT_TO_INT, &args![value as f64]).u32() as i32
}

// Translated from 006c1ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleManager::GetCellListForObstacle` (Xbox PDB; `cdecl`, a
/// plain function): empties `cells`, then adds to it the cells the obstacle
/// (`obstacle` is the address of its `NiPointer<ObstacleData>`) may touch.
/// Returns false when there is no current cell. In an interior only the
/// current cell is added. Outside, the cell coordinates of the obstacle's
/// `Center` give a 3 by 3 block of cells around it: a cell of the current
/// world space that is in state six (`00450ff0`) is added when the box
/// of its square (`4096` units a side, the full height range) overlaps the
/// obstacle's world bounds (`006d93d0`). The stack argument order is
/// `(obstacle, cells)`.
pub fn nav_mesh_obstacle_manager_get_cell_list_for_obstacle(
    e: &mut Engine,
    obstacle: u32,
    cells: u32,
) -> bool {
    e.call(ARRAY_CLEAR, &args![cells, 0u32]);
    let tes = e.global::<u32>(TES_POINTER);
    let current = e.call(TES_GET_CURRENT_CELL, &args![tes]).u32();
    if current == 0 {
        return false;
    }
    if e.call(CELL_IS_INTERIOR, &args![current]).bool() {
        list_operation(e, ARRAY_ADD, cells, current);
        return true;
    }
    let data = pointer_get(e, obstacle);
    let center_x = e.mem.f32(data + ObstacleData::Center.off);
    let around_x = float_to_int(e, center_x) >> 12;
    let data = pointer_get(e, obstacle);
    let center_y = e.mem.f32(data + ObstacleData::Center.off + 4);
    let around_y = float_to_int(e, center_y) >> 12;
    let mut cell_x = around_x - 1;
    while cell_x <= around_x + 1 {
        let mut cell_y = around_y - 1;
        while cell_y <= around_y + 1 {
            let world_space = e.call(CELL_GET_WORLD_SPACE, &args![current]).u32();
            let cell = e
                .call(
                    WORLD_SPACE_GET_CELL_FROM_CELL_COORD,
                    &args![world_space, cell_x, cell_y],
                )
                .u32();
            if cell != 0 && e.call(CELL_IS_IN_STATE_SIX, &args![cell]).bool() {
                e.with_stack(0x18, |e, frame| {
                    let low = frame.addr();
                    let high = low + 12;
                    let lowest = e.global::<f32>(NEGATIVE_FLOAT_MAX);
                    e.call(
                        POINT3_CONSTRUCT,
                        &args![
                            low,
                            cell_x.wrapping_shl(12) as f32,
                            cell_y.wrapping_shl(12) as f32,
                            lowest
                        ],
                    );
                    let highest = e.global::<f32>(POSITIVE_FLOAT_MAX);
                    let cell_size = e.global::<f64>(CELL_SIZE_DOUBLE);
                    let high_y = (e.mem.f32(low + 4) as f64 + cell_size) as f32;
                    let high_x = (e.mem.f32(low) as f64 + cell_size) as f32;
                    e.call(POINT3_CONSTRUCT, &args![high, high_x, high_y, highest]);
                    let data = pointer_get(e, obstacle);
                    let bounds_max = data + ObstacleData::aabbMax.off;
                    let data = pointer_get(e, obstacle);
                    let bounds_min = data + ObstacleData::aabbMin.off;
                    if e.call(BOX_OVERLAP, &args![low, high, bounds_min, bounds_max])
                        .bool()
                    {
                        list_operation(e, ARRAY_ADD, cells, cell);
                    }
                });
            }
            cell_y += 1;
        }
        cell_x += 1;
    }
    true
}

// Translated from 006c2050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The navmeshes an obstacle overlaps (engine map: unnamed; `cdecl`, the
/// decompiler lost the second parameter): `obstacle` is the address of its
/// `NiPointer<ObstacleData>`, `found` an array of navmesh holders that this
/// adds to. For every cell of [`nav_mesh_obstacle_manager_get_cell_list_for_obstacle`]
/// and every navmesh of the cell (`0070ec90` is its navmesh array,
/// `00464f60` the element in a holder): `006969e0` fills the navmesh's
/// bounds; with the top raised by `128.0` the bounds are tested against the
/// obstacle's world bounds (`006d93d0`) and an overlapping navmesh is added to
/// `found` (`0042f850`). Always returns true.
pub fn fn_006c2050(e: &mut Engine, obstacle: u32, found: u32) -> bool {
    e.with_stack(0x60, |e, frame| {
        let frame = frame.addr() + 0x60;
        let low = frame - 0x18;
        let high = frame - 0x24;
        let cells = frame - 0x38;
        let first_holder = frame - 0x4c;
        let second_holder = frame - 0x50;
        e.call(POINT3_DEFAULT, &args![low]);
        e.call(POINT3_DEFAULT, &args![high]);
        e.call(CELL_ARRAY_CTOR, &args![cells, 0x20u32, 0u32, 0u32]);
        if nav_mesh_obstacle_manager_get_cell_list_for_obstacle(e, obstacle, cells) {
            let mut cell_index = 0;
            while cell_index < array_size(e, cells) {
                let element = array_at(e, cells, cell_index);
                let cell = word(e, element);
                let navmeshes = e.call(CELL_NAVMESHES, &args![cell]).u32();
                if navmeshes != 0 {
                    let mut navmesh_index = 0;
                    while navmesh_index < e.call(GET_WORD_AT_8, &args![navmeshes]).u32() {
                        let holder = e
                            .call(
                                NAVMESH_ARRAY_GET_BY_INDEX,
                                &args![navmeshes, first_holder, navmesh_index],
                            )
                            .u32();
                        let navmesh = pointer_get(e, holder);
                        let bounds_known =
                            e.call(NAVMESH_BOUNDS, &args![navmesh, low, high]).bool();
                        e.call(NAVMESH_HOLDER_DTOR, &args![first_holder]);
                        if bounds_known {
                            let top = e.mem.f32(high + 8);
                            let margin = e.global::<f64>(NAVMESH_TOP_MARGIN_DOUBLE);
                            e.mem.set_f32(high + 8, (top as f64 + margin) as f32);
                            let data = pointer_get(e, obstacle);
                            let bounds_max = data + ObstacleData::aabbMax.off;
                            let data = pointer_get(e, obstacle);
                            let bounds_min = data + ObstacleData::aabbMin.off;
                            if e.call(BOX_OVERLAP, &args![low, high, bounds_min, bounds_max])
                                .bool()
                            {
                                let holder = e
                                    .call(
                                        NAVMESH_ARRAY_GET_BY_INDEX,
                                        &args![navmeshes, second_holder, navmesh_index],
                                    )
                                    .u32();
                                e.call(NAVMESH_HOLDER_ARRAY_ADD, &args![found, holder]);
                                e.call(NAVMESH_HOLDER_DTOR, &args![second_holder]);
                            }
                        }
                        navmesh_index += 1;
                    }
                }
                cell_index += 1;
            }
        }
        e.call(CELL_ARRAY_DTOR, &args![cells]);
        true
    })
}

/// Whether the collision rigid body `006fa820` finds under `node` has one of
/// the motion types `types` (`0043b4f0` fills a small record, `0043b4d0` gives
/// its low seven bits).
fn rigid_body_has_motion_type(e: &mut Engine, rigid_body: u32, types: &[u32]) -> bool {
    e.with_stack(0x10, |e, record| {
        let record = e
            .call(RIGID_BODY_MOTION_INFO, &args![rigid_body, record])
            .u32();
        let kind = e.call(MOTION_INFO_TYPE, &args![record]).u32();
        types.contains(&kind)
    })
}

// Translated from 006c2210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the obstacles of a node's tree (engine map: unnamed; `cdecl`):
/// `array` is the address of the `NiPointer<ReferenceObstacleArray>` that
/// receives them. If the node has a collision object (`0043b610`) with a rigid
/// body (`006fa820`) whose motion type is 2, 4, 10, 20 or 28, a new
/// `ObstacleData` is made (`0x8c` bytes, `006c23c0`), its `spParentArray` set
/// to the array (without taking a reference), its rigid body set, filled by
/// `006c2650` and added to the array's `Obstacles` (`006c61b0`). The children
/// of the node (virtual slot `0x0c`) are then visited the same way. Always
/// returns true.
pub fn fn_006c2210(e: &mut Engine, node: u32, array: u32) -> bool {
    let collision_object = e.call(COLLISION_OBJECT_OF_NODE, &args![node]).u32();
    if collision_object != 0 {
        let rigid_body = e
            .call(COLLISION_OBJECT_RIGID_BODY, &args![collision_object])
            .u32();
        if rigid_body != 0 && rigid_body_has_motion_type(e, rigid_body, &[2, 4, 0x0a, 0x14, 0x1c]) {
            let memory = e.call(OPERATOR_NEW_SIZED, &args![OBSTACLE_DATA_SIZE]).u32();
            let created = if memory != 0 {
                fn_006c23c0(e, Ptr::new(memory)).addr()
            } else {
                0
            };
            e.with_stack(4, |e, slot| {
                let slot = slot.addr();
                pointer_init(e, slot, created);
                let parent = pointer_get(e, array);
                let data = pointer_get(e, slot);
                e.mem
                    .set_u32(data + ObstacleData::spParentArray.off, parent);
                let data = pointer_get(e, slot);
                e.call(
                    NI_POINTER_SET,
                    &args![data + ObstacleData::pRigidBody.off, rigid_body],
                );
                let data = pointer_get(e, slot);
                fn_006c2650(e, data);
                let parent = pointer_get(e, array);
                e.call(
                    OBSTACLE_ARRAY_ADD,
                    &args![parent + ReferenceObstacleArray::Obstacles.off, slot],
                );
                pointer_release(e, slot);
            });
        }
    }
    let children = e.vcall(node, NODE_CHILDREN_SLOT, &args![]).u32();
    if children != 0 {
        let mut index = 0;
        while index < e.call(CHILDREN_COUNT, &args![children]).u32() {
            if e.call(CHILDREN_AT, &args![children, index]).u32() != 0 {
                let child = e.call(CHILDREN_AT, &args![children, index]).u32();
                fn_006c2210(e, child, array);
            }
            index += 1;
        }
    }
    true
}

// Translated from 006c23c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ObstacleData` (engine map: unnamed): the reference-counted
/// base constructor (`004968b0`), the class's vtable, a null `pRigidBody`,
/// the default constructors of `Center`, `Orientation`, `BoxMin`, `BoxMax`,
/// `aabbMin` and `aabbMax`, `bActive` set, the `Navmeshes` array
/// constructor (`0069aee0`) and a null `sp3DNode`. Returns `this`.
pub fn fn_006c23c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(REFERENCE_COUNTED_CTOR, &args![this]);
    e.mem.set_u32(base, OBSTACLE_DATA_VTABLE);
    pointer_init(e, base + ObstacleData::pRigidBody.off, 0);
    for field in [
        ObstacleData::Center.off,
        ObstacleData::Orientation.off,
        ObstacleData::BoxMin.off,
        ObstacleData::BoxMax.off,
        ObstacleData::aabbMin.off,
        ObstacleData::aabbMax.off,
    ] {
        e.call(POINT3_DEFAULT, &args![base + field]);
    }
    e.mem.set_u8(base + ObstacleData::bActive.off, 1);
    e.call(
        OBSTACLE_NAVMESH_ARRAY_CTOR,
        &args![base + ObstacleData::Navmeshes.off],
    );
    pointer_init(e, base + ObstacleData::sp3DNode.off, 0);
    this
}

// Translated from 006c2490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ObstacleData::~ObstacleData`'s scalar deleting destructor (Xbox PDB): the
/// destructor `006c24d0`, then the sized `operator delete` (`0x8c` bytes) when
/// bit 0 of `flags` is set. Returns `this`.
pub fn obstacle_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006c24d0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE_SIZED, &args![this, OBSTACLE_DATA_SIZE]);
    }
    this
}

// Translated from 006c24d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `ObstacleData` (engine map: unnamed): releases `sp3DNode`,
/// destroys the `Navmeshes` array (`0069af10`), releases `pRigidBody` and runs
/// the reference-counted base destructor (`00496910`). The vtable is left as
/// it is.
pub fn fn_006c24d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    pointer_release(e, base + ObstacleData::sp3DNode.off);
    e.call(
        OBSTACLE_NAVMESH_ARRAY_DTOR,
        &args![base + ObstacleData::Navmeshes.off],
    );
    pointer_release(e, base + ObstacleData::pRigidBody.off);
    e.call(REFERENCE_COUNTED_DTOR, &args![this]);
}

// Translated from 006c2550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills one `ObstacleData` from a node's tree (engine map: unnamed; `cdecl`):
/// the first node of the tree whose collision rigid body has motion type 1,
/// 2, 4, 10, 20 or 28 clears `spParentArray`, takes that rigid body, is
/// filled by `006c2650` and ends the search with true. Otherwise the children
/// (virtual slot `0x0c`, counted by `00453470`) are tried in turn; false when
/// none qualifies.
pub fn fn_006c2550(e: &mut Engine, node: u32, data: u32) -> bool {
    let collision_object = e.call(COLLISION_OBJECT_OF_NODE, &args![node]).u32();
    if collision_object != 0 {
        let rigid_body = e
            .call(COLLISION_OBJECT_RIGID_BODY, &args![collision_object])
            .u32();
        if rigid_body != 0
            && rigid_body_has_motion_type(e, rigid_body, &[2, 1, 4, 0x0a, 0x14, 0x1c])
        {
            e.mem.set_u32(data + ObstacleData::spParentArray.off, 0);
            e.call(
                NI_POINTER_SET,
                &args![data + ObstacleData::pRigidBody.off, rigid_body],
            );
            fn_006c2650(e, data);
            return true;
        }
    }
    let children = e.vcall(node, NODE_CHILDREN_SLOT, &args![]).u32();
    if children != 0 {
        let mut index = 0;
        while index < e.call(CHILDREN_COUNT_SHALLOW, &args![children]).u32() {
            if e.call(CHILDREN_AT, &args![children, index]).u32() != 0 {
                let child = e.call(CHILDREN_AT, &args![children, index]).u32();
                if fn_006c2550(e, child, data) {
                    return true;
                }
            }
            index += 1;
        }
    }
    false
}

// Translated from 006c3440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Decides whether an obstacle counts (engine map: unnamed; `cdecl`, `slot` is
/// the address of its `NiPointer<ObstacleData>`): with the size
/// `aabbMax - aabbMin` of its world bounds, `bActive` is cleared when the
/// height is under `48.0`, set when it is `96.0` or more, and otherwise set
/// when the width or the depth is `16.0` or more (cleared for a bound that is
/// both low and thin).
pub fn fn_006c3440(e: &mut Engine, slot: u32) {
    e.with_stack(0x10, |e, size| {
        let size = size.addr();
        let data = pointer_get(e, slot);
        let minimum = data + ObstacleData::aabbMin.off;
        let data = pointer_get(e, slot);
        let maximum = data + ObstacleData::aabbMax.off;
        e.call(POINT3_SUBTRACT, &args![maximum, size, minimum]);
        let (x, y, z) = (e.mem.f32(size), e.mem.f32(size + 4), e.mem.f32(size + 8));
        let minimum_height = e.global::<f32>(OBSTACLE_MIN_HEIGHT);
        let minimum_width = e.global::<f32>(OBSTACLE_MIN_WIDTH);
        let active = if z < minimum_height {
            false
        } else {
            // The comparisons are unordered-true (`FCOMPP` then `JNZ` / `JP`).
            let double_height = minimum_height + minimum_height;
            !is_less(z, double_height) || !is_less(x, minimum_width) || !is_less(y, minimum_width)
        };
        let data = pointer_get(e, slot);
        e.mem.set_u8(data + ObstacleData::bActive.off, active as u8);
    });
}

// Translated from 006c34f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the obstacle array of a reference (engine map: unnamed; `cdecl`):
/// stores the reference's form id in the array's `iFormID` and rebuilds the
/// obstacles with `006c3520`. `array` is the address of the
/// `NiPointer<ReferenceObstacleArray>`. Returns what `006c3520` returns.
pub fn fn_006c34f0(e: &mut Engine, reference: u32, array: u32) -> bool {
    let form_id = e.call(FORM_ID, &args![reference]).u32();
    let data = pointer_get(e, array);
    e.mem
        .set_u32(data + ReferenceObstacleArray::iFormID.off, form_id);
    fn_006c3520(e, reference, array)
}

// Translated from 006c3520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the obstacles of a reference (engine map: unnamed; `cdecl`):
/// empties the array's `Obstacles` (`006c6200`) and, when the reference has
/// a 3D node (virtual slot `0x1d0`), builds them from that node's tree with
/// `006c2210`. False when it has none.
pub fn fn_006c3520(e: &mut Engine, reference: u32, array: u32) -> bool {
    let data = pointer_get(e, array);
    e.call(
        OBSTACLE_ARRAY_EMPTY,
        &args![data + ReferenceObstacleArray::Obstacles.off, 1u32],
    );
    if e.vcall(reference, REFERENCE_NODE_SLOT, &args![]).u32() == 0 {
        return false;
    }
    let node = e.vcall(reference, REFERENCE_NODE_SLOT, &args![]).u32();
    fn_006c2210(e, node, array)
}

// Translated from 006c3570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a point lies inside an obstacle's box, with a vertical range
/// (engine map: unnamed; `cdecl`): `point` is the address of a `NiPoint3`,
/// `slot` the address of the obstacle's `NiPointer<ObstacleData>`. The point
/// relative to `Center`, turned into the obstacle's frame with
/// `Orientation` (`004b3ae0`, vector times matrix), must have its z not below
/// `-below`, not above `above`, and its x and y inside `BoxMin` and `BoxMax`.
/// An unordered comparison does not reject the point.
pub fn fn_006c3570(e: &mut Engine, point: u32, slot: u32, below: f32, above: f32) -> bool {
    e.with_stack(0x20, |e, frame| {
        let relative = frame.addr();
        let local = frame.addr() + 0x0c;
        let data = pointer_get(e, slot);
        let orientation = data + ObstacleData::Orientation.off;
        let data = pointer_get(e, slot);
        let center = data + ObstacleData::Center.off;
        let difference = e
            .call(POINT3_SUBTRACT, &args![point, relative, center])
            .u32();
        e.call(POINT_TIMES_MATRIX, &args![local, difference, orientation]);
        let (x, y, z) = (e.mem.f32(local), e.mem.f32(local + 4), e.mem.f32(local + 8));
        if z < -below {
            return false;
        }
        if above < z {
            return false;
        }
        let data = pointer_get(e, slot);
        if x < e.mem.f32(data + ObstacleData::BoxMin.off) {
            return false;
        }
        let data = pointer_get(e, slot);
        if e.mem.f32(data + ObstacleData::BoxMax.off) < x {
            return false;
        }
        let data = pointer_get(e, slot);
        if y < e.mem.f32(data + ObstacleData::BoxMin.off + 4) {
            return false;
        }
        let data = pointer_get(e, slot);
        if e.mem.f32(data + ObstacleData::BoxMax.off + 4) < y {
            return false;
        }
        true
    })
}

/// Copies the three words of a point (`NiPoint3`) from `from` to `to`.
fn copy_point(e: &mut Engine, to: u32, from: u32) {
    for index in 0..3 {
        let value = e.mem.u32(from + 4 * index);
        e.mem.set_u32(to + 4 * index, value);
    }
}

/// `fabs` (`00408840`) of a `float`.
fn float_abs(e: &mut Engine, value: f32) -> f32 {
    e.call(FLOAT_ABS, &args![value]).f32()
}

// Translated from 006c2650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes the box and the bounds of an `ObstacleData` from its rigid body
/// (engine map: unnamed; `cdecl`, `data` the `ObstacleData`). False, with
/// nothing changed, when `pRigidBody` is null or its Havok object (`00620b80`)
/// is. Otherwise: the rigid body's transform is read into a local
/// `NiTransform`, its local aabb (`GetAabbLocal`) turned into the points
/// `lower` and `upper`, and `Center` set to the transform's translation. The
/// three columns of the transform's rotation become `u` (the column with the
/// largest `|z|`), `v` (the next column, cyclically) and `w` (the other one);
/// `u` and `v` are negated when `u.z` is negative, `u` is then replaced by
/// `(0, 0, 1)` (`011a9484`), `w = unit cross(u, v)` and `v = -(w x u)`; `v`,
/// `w`, `u` become the columns `0`, `1`, `2` of `Orientation`. The eight
/// corners of the aabb are mapped by
/// `transpose(Orientation) * rotation` into the obstacle's frame, then to
/// world space (`Orientation * corner + Center`); a world corner whose x or y
/// is within `15.0` of a multiple of `4096.0` (a cell border) is moved by
/// `30.0` and its local counterpart recomputed. `BoxMin` and `BoxMax` are
/// the bounds of the local corners, `aabbMin` and `aabbMax` those of the
/// world corners, and `iLastUpdateTime` is set from the clock
/// (`00825c00` on `011f6394`). Returns true. The stack protector and the
/// 16-byte alignment prologue are not translated.
pub fn fn_006c2650(e: &mut Engine, data: u32) -> bool {
    let rigid_body = pointer_get(e, data + ObstacleData::pRigidBody.off);
    if rigid_body == 0 {
        return false;
    }
    let havok_object = e.call(GET_WORD_AT_8, &args![rigid_body]).u32();
    if havok_object == 0 {
        return false;
    }
    e.with_stack(0x400, |e, frame| {
        // `at(n)` is the address of the local the game keeps `n` bytes
        // below its frame pointer.
        let top = frame.addr() + 0x400;
        let at = |below: u32| top - below;
        let orientation = data + ObstacleData::Orientation.off;
        let center = data + ObstacleData::Center.off;
        let transform = at(0x84);
        let transform_translation = at(0x60);
        let transform_scale = at(0x54);
        let transform_source = at(0x50);
        let aabb = at(0xb0);
        let lower = at(0xc8);
        let upper = at(0xd4);
        let columns = at(0xf8);
        let (u, w, v) = (at(0x108), at(0x114), at(0x120));
        let corners_local = at(0x220);
        let corners_world = at(0x340);

        let source = e.call(HAVOK_TRANSFORM_SOURCE, &args![havok_object]).u32();
        e.call(HAVOK_TRANSFORM_BUILD, &args![transform_source, source]);
        e.call(TRANSFORM_DEFAULT, &args![transform]);
        e.mem.set_f32(transform_scale, 1.0);
        e.call(TRANSFORM_FROM_HAVOK, &args![transform, transform_source]);
        e.call(AABB_DEFAULT, &args![aabb]);
        e.call(RIGID_BODY_GET_AABB_LOCAL, &args![rigid_body, aabb]);
        e.call(
            MATRIX_TO_EULER_ANGLES,
            &args![transform, at(0xbc), at(0xb8), at(0xb4)],
        );
        e.call(POINT3_DEFAULT, &args![lower]);
        e.call(POINT3_DEFAULT, &args![upper]);
        e.call(HK_VECTOR_TO_POINT3, &args![lower, aabb]);
        e.call(HK_VECTOR_TO_POINT3, &args![upper, aabb + 0x10]);
        copy_point(e, center, transform_translation);

        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![columns, 12u32, 3u32, POINT3_DEFAULT],
        );
        for index in 0..3 {
            e.call(
                MATRIX_GET_COLUMN,
                &args![transform, index, columns + 12 * index],
            );
        }
        e.call(POINT3_DEFAULT, &args![u]);
        e.call(POINT3_DEFAULT, &args![w]);
        e.call(POINT3_DEFAULT, &args![v]);

        // Which columns become `u`, `v` and `w`, by the size of their z.
        let first_z = e.mem.f32(columns + 8);
        let second_z = e.mem.f32(columns + 20);
        let third_z = e.mem.f32(columns + 32);
        let first_abs = float_abs(e, first_z);
        let second_abs = float_abs(e, second_z);
        let (u_column, v_column, w_column) = if second_abs < first_abs {
            let first_abs = float_abs(e, first_z);
            let third_abs = float_abs(e, third_z);
            if third_abs < first_abs {
                (0, 1, 2)
            } else {
                (2, 0, 1)
            }
        } else {
            let second_abs = float_abs(e, second_z);
            let third_abs = float_abs(e, third_z);
            if third_abs < second_abs {
                (1, 2, 0)
            } else {
                (2, 0, 1)
            }
        };
        copy_point(e, u, columns + 12 * u_column);
        copy_point(e, v, columns + 12 * v_column);
        copy_point(e, w, columns + 12 * w_column);

        let zero = e.global::<f64>(ZERO_DOUBLE);
        if (e.mem.f32(u + 8) as f64) < zero {
            let negated = e.call(POINT3_NEGATE, &args![u, at(0x144)]).u32();
            copy_point(e, u, negated);
            let negated = e.call(POINT3_NEGATE, &args![v, at(0x150)]).u32();
            copy_point(e, v, negated);
        }
        copy_point(e, u, UP_VECTOR);
        let cross = e.call(POINT3_UNIT_CROSS, &args![u, at(0x15c), v]).u32();
        copy_point(e, w, cross);
        let cross = e.call(POINT3_CROSS, &args![w, at(0x174), u]).u32();
        let negated = e.call(POINT3_NEGATE, &args![cross, at(0x168)]).u32();
        copy_point(e, v, negated);
        e.call(MATRIX_SET_COLUMN, &args![orientation, 0u32, v]);
        e.call(MATRIX_SET_COLUMN, &args![orientation, 1u32, w]);
        e.call(MATRIX_SET_COLUMN, &args![orientation, 2u32, u]);

        // The aabb's eight corners in the obstacle's frame.
        let transposed = e
            .call(MATRIX_TRANSPOSE, &args![orientation, at(0x1bc)])
            .u32();
        let local_rotation = at(0x198);
        e.call(
            MATRIX_TIMES_MATRIX,
            &args![transposed, local_rotation, transform],
        );
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![corners_local, 12u32, 8u32, POINT3_DEFAULT],
        );
        // (x from upper, y from upper, z from upper, the corner's temporary
        // point, its temporary result), the corners in the order of the game.
        let corners: [(bool, bool, bool, u32, u32); 8] = [
            (false, false, false, 0x22c, 0x238),
            (true, false, false, 0x244, 0x250),
            (false, true, false, 0x25c, 0x268),
            (true, true, false, 0x274, 0x280),
            (false, false, true, 0x28c, 0x298),
            (true, false, true, 0x2a4, 0x2b0),
            (false, true, true, 0x2bc, 0x2c8),
            (true, true, true, 0x2d4, 0x2e0),
        ];
        for (index, (x_upper, y_upper, z_upper, point, result)) in corners.iter().enumerate() {
            let pick =
                |upper_side: bool, axis: u32| (if upper_side { upper } else { lower }) + 4 * axis;
            let z = e.mem.f32(pick(*z_upper, 2));
            let y = e.mem.f32(pick(*y_upper, 1));
            let x = e.mem.f32(pick(*x_upper, 0));
            let corner = e.call(POINT3_CONSTRUCT, &args![at(*point), x, y, z]).u32();
            let mapped = e
                .call(
                    MATRIX_TIMES_POINT,
                    &args![local_rotation, at(*result), corner],
                )
                .u32();
            copy_point(e, corners_local + 12 * index as u32, mapped);
        }

        // ... and in world space.
        e.call(
            VECTOR_CONSTRUCTOR_ITERATOR,
            &args![corners_world, 12u32, 8u32, POINT3_DEFAULT],
        );
        for index in 0..8u32 {
            let rotated = e
                .call(
                    MATRIX_TIMES_POINT,
                    &args![orientation, at(0x35c), corners_local + 12 * index],
                )
                .u32();
            let moved = e.call(POINT3_ADD, &args![rotated, at(0x350), center]).u32();
            copy_point(e, corners_world + 12 * index, moved);
        }
        let to_local = at(0x380);
        e.call(MATRIX_TRANSPOSE, &args![orientation, to_local]);
        let cell_size = e.global::<f32>(CELL_SIZE_FLOAT);
        let edge = e.global::<f32>(BOX_EDGE_MARGIN);
        for index in 0..8u32 {
            let world = corners_world + 12 * index;
            // x, then y: a coordinate near a cell border is pushed past it.
            for (axis, difference, result) in [(0u32, 0x390u32, 0x39cu32), (1, 0x3a8, 0x3b4)] {
                let coordinate = e.mem.f32(world + 4 * axis);
                let remainder = e.call(FLOAT_REMAINDER, &args![coordinate, cell_size]).f32();
                if float_abs(e, remainder) < edge {
                    let pushed = (edge as f64 + edge as f64 + coordinate as f64) as f32;
                    e.mem.set_f32(world + 4 * axis, pushed);
                    let offset = e
                        .call(POINT3_SUBTRACT, &args![world, at(difference), center])
                        .u32();
                    let mapped = e
                        .call(MATRIX_TIMES_POINT, &args![to_local, at(result), offset])
                        .u32();
                    copy_point(e, corners_local + 12 * index, mapped);
                }
            }
        }

        // The bounds of the local corners (`BoxMin`, `BoxMax`) ...
        let highest = e.global::<f32>(POSITIVE_FLOAT_MAX);
        let lowest = e.global::<f32>(NEGATIVE_FLOAT_MAX);
        let box_min = data + ObstacleData::BoxMin.off;
        let box_max = data + ObstacleData::BoxMax.off;
        let start = e
            .call(
                POINT3_CONSTRUCT,
                &args![at(0x3c0), highest, highest, highest],
            )
            .u32();
        copy_point(e, box_min, start);
        let start = e
            .call(POINT3_CONSTRUCT, &args![at(0x3cc), lowest, lowest, lowest])
            .u32();
        copy_point(e, box_max, start);
        grow_bounds(e, box_min, box_max, corners_local);

        // ... and of the world corners (`aabbMin`, `aabbMax`).
        let aabb_min = data + ObstacleData::aabbMin.off;
        let aabb_max = data + ObstacleData::aabbMax.off;
        let start = e
            .call(
                POINT3_CONSTRUCT,
                &args![at(0x3dc), highest, highest, highest],
            )
            .u32();
        copy_point(e, aabb_min, start);
        let start = e
            .call(POINT3_CONSTRUCT, &args![at(0x3e8), lowest, lowest, lowest])
            .u32();
        copy_point(e, aabb_max, start);
        grow_bounds(e, aabb_min, aabb_max, corners_world);

        let now = e.call(CLOCK_MILLISECONDS, &args![CLOCK_OBJECT]).u32();
        e.mem.set_u32(data + ObstacleData::iLastUpdateTime.off, now);
        true
    })
}

/// The eight corners at `corners` (12 bytes each) folded into the bounds
/// `minimum` / `maximum` (three floats each): the `min` helper (`0040ebd0`)
/// on the three coordinates of every corner, then the `max` helper
/// (`00404010`), as `006c2650` does.
fn grow_bounds(e: &mut Engine, minimum: u32, maximum: u32, corners: u32) {
    for index in 0..8u32 {
        let corner = corners + 12 * index;
        for axis in 0..3u32 {
            let current = e.mem.f32(minimum + 4 * axis);
            let value = e.mem.f32(corner + 4 * axis);
            let result = e.call(FLOAT_MIN, &args![current, value]).f32();
            e.mem.set_f32(minimum + 4 * axis, result);
        }
        for axis in 0..3u32 {
            let current = e.mem.f32(maximum + 4 * axis);
            let value = e.mem.f32(corner + 4 * axis);
            let result = e.call(FLOAT_MAX, &args![current, value]).f32();
            e.mem.set_f32(maximum + 4 * axis, result);
        }
    }
}

/// `PathManager::QInstance` (Xbox PDB) with one of its thiscalls.
fn path_manager_call(e: &mut Engine, function: u32) -> Ret {
    let instance = e.call(PATH_MANAGER_INSTANCE, &args![]).u32();
    e.call(function, &args![instance])
}

// Translated from 006c3640 (decompiled, FalloutNV.exe 1.4.0.525)
/// The per-frame update of the manager (engine map: unnamed), inside a debug
/// scope (line `0x508`) and a no-op when the obstacle setting is off. Under
/// the critical section (`004538a0` / `004538c0`), `fTimeToNextSwap` is
/// reduced by the last frame's time (`0084d030` on `011f6394`). In state 0
/// (collecting) it runs the queue steps `006c47e0`, `006c45e0`, `006c4fb0`,
/// `006c5190`, `006c4970`, `006c4900`, then either `006c4c30` (when
/// `bUpdateAllObstacles`, which it clears) or `006c4ad0` and `006c49e0`, then
/// `006c4d30`. It then starts the path manager (`006ebc70`) and moves to
/// state 1 when `fTimeToNextSwap` has run out (`<= 0`) and either the
/// background and processed task counts (`006c3970`, `006c3920`) are equal and
/// not zero, or there are no background tasks but `CurrentNavMeshTaskMap` has
/// entries. In state 1 (waiting), once
/// the path manager is ready (`006ebc90`) and there are background tasks it
/// reconnects the portals (`006c3ba0`), finishes the tasks (`006c44d0`, whose
/// result says that something changed) and frees them (`006c4540`); then
/// `006c4ec0` queues the next tasks, the path manager is released
/// (`006ebcd0`), the state goes back to 0 with the swap time reset from
/// `0106c398` and, if something changed and navmesh drawing is on, the
/// navmesh display is redrawn (`006a5ca0`).
pub fn fn_006c3640(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    with_scope_guard(e, 0x508, |e| {
        if !obstacles_enabled(e) {
            return;
        }
        let manager = this.addr();
        e.call(CRITICAL_SECTION_ENTER, &args![this, UPDATE_LOCK_NAME]);
        e.call(
            PERFORMANCE_TIMER_STEP,
            &args![manager + NavMeshObstacleManager::MainThreadPerformaceTimer.off],
        );
        let mut changed = false;
        let frame_time = e.call(CLOCK_FRAME_SECONDS, &args![CLOCK_OBJECT]).f32();
        let remaining =
            e.get(this, NavMeshObstacleManager::fTimeToNextSwap) as f64 - frame_time as f64;
        e.set(
            this,
            NavMeshObstacleManager::fTimeToNextSwap,
            remaining as f32,
        );
        match e.get(this, NavMeshObstacleManager::eState) {
            0 => {
                fn_006c47e0(e, this);
                fn_006c45e0(e, this);
                fn_006c4fb0(e, this);
                fn_006c5190(e, this);
                fn_006c4970(e, this);
                fn_006c4900(e, this);
                if e.get(this, NavMeshObstacleManager::bUpdateAllObstacles) {
                    fn_006c4c30(e, this);
                    e.set(this, NavMeshObstacleManager::bUpdateAllObstacles, false);
                } else {
                    fn_006c4ad0(e, this);
                    fn_006c49e0(e, this);
                }
                fn_006c4d30(e, this);
                let processed = fn_006c3920(e, this);
                let background = fn_006c3970(e, this);
                let queued_tasks = e
                    .call(
                        FORM_ID,
                        &args![manager + NavMeshObstacleManager::CurrentNavMeshTaskMap.off],
                    )
                    .u32()
                    > 0;
                let time_is_up = (e.get(this, NavMeshObstacleManager::fTimeToNextSwap) as f64)
                    <= e.global::<f64>(ZERO_DOUBLE);
                // With tasks queued in `CurrentNavMeshTaskMap`, no background
                // task at all also lets the swap happen.
                let go = if queued_tasks && background == 0 {
                    time_is_up
                } else {
                    background > 0 && background == processed && time_is_up
                };
                if go {
                    path_manager_call(e, PATH_MANAGER_START);
                    e.set(this, NavMeshObstacleManager::eState, 1);
                }
            }
            1 if path_manager_call(e, PATH_MANAGER_IS_READY).bool() => {
                let _processed = fn_006c3920(e, this);
                let background = fn_006c3970(e, this);
                if background > 0 {
                    nav_mesh_obstacle_manager_reconnect_portals(e, this);
                    changed = fn_006c44d0(e, this);
                    fn_006c4540(e, this);
                }
                fn_006c4ec0(e, this);
                path_manager_call(e, PATH_MANAGER_FINISH);
                e.set(this, NavMeshObstacleManager::eState, 0);
                let swap_time = e.global::<f32>(INITIAL_TIME_TO_NEXT_SWAP);
                e.set(this, NavMeshObstacleManager::fTimeToNextSwap, swap_time);
                if changed {
                    let render = e.call(NAVMESH_RENDER_OBJECT, &args![]).u32();
                    if e.call(NAVMESH_RENDER_ENABLED, &args![render]).bool() {
                        let render = e.call(NAVMESH_RENDER_OBJECT, &args![]).u32();
                        e.call(NAVMESH_RENDER_REDRAW, &args![render]);
                    }
                }
            }
            _ => {}
        }
        e.call(
            PERFORMANCE_TIMER_STEP,
            &args![manager + NavMeshObstacleManager::MainThreadPerformaceTimer.off],
        );
        e.call(CRITICAL_SECTION_LEAVE, &args![this]);
    })
}

// Translated from 006c3920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of processed tasks (engine map: unnamed): the size of
/// `ProcessedTasks`, read under `ProcessedTaskLock`.
pub fn fn_006c3920(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) -> u32 {
    let lock = this.addr() + NavMeshObstacleManager::ProcessedTaskLock.off;
    e.call(SPIN_LOCK_LOCK, &args![lock, PROCESSED_LOCK_NAME]);
    let count = array_size(e, this.addr() + NavMeshObstacleManager::ProcessedTasks.off);
    e.call(SPIN_LOCK_UNLOCK, &args![lock]);
    count
}

// Translated from 006c3970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of background tasks (engine map: unnamed): the size of
/// `BackgroundTasks`, read under `ProcessedTaskLock`.
pub fn fn_006c3970(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) -> u32 {
    let lock = this.addr() + NavMeshObstacleManager::ProcessedTaskLock.off;
    e.call(SPIN_LOCK_LOCK, &args![lock, BACKGROUND_LOCK_NAME]);
    let count = array_size(e, this.addr() + NavMeshObstacleManager::BackgroundTasks.off);
    e.call(SPIN_LOCK_UNLOCK, &args![lock]);
    count
}

/// Waits (`Sleep(0)`) until the processed and background task counts agree.
fn wait_for_tasks(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    loop {
        let processed = fn_006c3920(e, this);
        let background = fn_006c3970(e, this);
        if processed == background {
            return;
        }
        e.call(SLEEP, &args![0u32]);
    }
}

// Translated from 006c39c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs a whole update cycle to completion (engine map: unnamed), under the
/// critical section: waits until the processed and background task counts
/// agree (`Sleep(0)`), starts the path manager and waits (`006ebc50`) until it
/// is ready, reconnects the portals, finishes and frees the tasks and queues
/// the next ones (`006c3ba0`, `006c44d0`, `006c4540`, `006c4ec0`); waits for
/// the tasks again, repeats the reconnection, then runs every queue step
/// (`006c47e0`, `006c45e0`, `006c4c30`, `006c4d30`, `006c4fb0`, `006c5190`,
/// `006c4ec0`), waits and reconnects a last time. It leaves state 0 with the
/// swap time reset from `0106c398`, redraws the navmesh display when it is on
/// and releases the path manager (`006ebcd0`).
pub fn fn_006c39c0(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    e.call(CRITICAL_SECTION_ENTER, &args![this, FLUSH_LOCK_NAME]);
    wait_for_tasks(e, this);
    path_manager_call(e, PATH_MANAGER_START);
    while !path_manager_call(e, PATH_MANAGER_IS_READY).bool() {
        path_manager_call(e, PATH_MANAGER_WAIT);
    }
    nav_mesh_obstacle_manager_reconnect_portals(e, this);
    fn_006c44d0(e, this);
    fn_006c4540(e, this);
    fn_006c4ec0(e, this);
    wait_for_tasks(e, this);
    nav_mesh_obstacle_manager_reconnect_portals(e, this);
    fn_006c44d0(e, this);
    fn_006c4540(e, this);
    fn_006c47e0(e, this);
    fn_006c45e0(e, this);
    fn_006c4c30(e, this);
    fn_006c4d30(e, this);
    fn_006c4fb0(e, this);
    fn_006c5190(e, this);
    fn_006c4ec0(e, this);
    wait_for_tasks(e, this);
    nav_mesh_obstacle_manager_reconnect_portals(e, this);
    fn_006c44d0(e, this);
    fn_006c4540(e, this);
    e.set(this, NavMeshObstacleManager::eState, 0);
    let swap_time = e.global::<f32>(INITIAL_TIME_TO_NEXT_SWAP);
    e.set(this, NavMeshObstacleManager::fTimeToNextSwap, swap_time);
    let render = e.call(NAVMESH_RENDER_OBJECT, &args![]).u32();
    if e.call(NAVMESH_RENDER_ENABLED, &args![render]).bool() {
        let render = e.call(NAVMESH_RENDER_OBJECT, &args![]).u32();
        e.call(NAVMESH_RENDER_REDRAW, &args![render]);
    }
    path_manager_call(e, PATH_MANAGER_FINISH);
    e.call(CRITICAL_SECTION_LEAVE, &args![this]);
}

/// `ObstacleTaskData` (Xbox PDB, class of another unit) fields at their
/// offsets, which are the same on PC: `pInfo` (`NavMeshInfo *`), `spSrcMesh`
/// and `spNewNavMesh` (`NavMeshPtr`, an `NiPointer<NavMesh>`), `PortalSwaps`
/// (`TaskPortalSwap`: two arrays of 12-byte portal records, at `+0` and
/// `+0x10` of it) and `bPortalModified`.
const TASK_INFO: u32 = 0x18;
const TASK_SOURCE_MESH: u32 = 0x1c;
const TASK_NEW_MESH: u32 = 0x20;
const TASK_PORTAL_SWAPS: u32 = 0x34;
const TASK_SECOND_PORTAL_ARRAY: u32 = 0x44;
const TASK_PORTAL_MODIFIED: u32 = 0x54;
/// A 12-byte portal record: a word, the triangle index (`u16` at `+4`) and the
/// edge (`u32` at `+8`); the matching edge `0068f340` fills one the same way
/// with the `NavMeshInfo` pointer in its first word.
const RECORD_TRIANGLE: u32 = 4;
const RECORD_EDGE: u32 = 8;
/// `006240d0`: the default constructor of the record `0068f340` fills.
const MATCHING_EDGE_DEFAULT: u32 = 0x0062_40d0;

// Translated from 006c4350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy constructor of a 12-byte portal record (engine map: unnamed): the
/// default constructor `006815c0`, then the word at `+0`, the `u16` at `+4`
/// and the word at `+8` copied from `source`. Returns `this`.
pub fn fn_006c4350(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    let base = this.addr();
    e.call(POINT3_DEFAULT, &args![this]);
    let first = e.mem.u32(source);
    e.mem.set_u32(base, first);
    let triangle = e.mem.u16(source + RECORD_TRIANGLE);
    e.mem.set_u16(base + RECORD_TRIANGLE, triangle);
    let edge = e.mem.u32(source + RECORD_EDGE);
    e.mem.set_u32(base + RECORD_EDGE, edge);
    this
}

/// What `006c4390` and `006c4460` do with one side of a portal: builds a
/// local edge record from `source` (default constructor `006923c0`, then the
/// word at `+0` and the `u16` at `+4` copied to `+4` and `+8`), asks the
/// navmesh held at `navmesh_slot` for its edge number (`00691020`), and links
/// the edge `triangle_record` names (its triangle, edge and the number) with
/// `00690e30` on that triangle (`005582f0`).
fn link_portal_side(e: &mut Engine, navmesh_slot: u32, triangle_record: u32, source: u32) {
    e.with_stack(0x10, |e, record| {
        let record = record.addr();
        e.call(EDGE_RECORD_DEFAULT, &args![record]);
        let first = e.mem.u32(source);
        e.mem.set_u32(record + 4, first);
        let second = e.mem.u16(source + RECORD_TRIANGLE);
        e.mem.set_u16(record + 8, second);
        let navmesh = pointer_get(e, navmesh_slot);
        let number = e
            .call(NAVMESH_EDGE_RECORD_INDEX, &args![navmesh, record])
            .u16() as u32;
        let edge = e.mem.u32(triangle_record + RECORD_EDGE);
        let triangle_index = e.mem.u16(triangle_record + RECORD_TRIANGLE) as u32;
        let navmesh = pointer_get(e, navmesh_slot);
        let triangle = e
            .call(NAVMESH_TRIANGLE, &args![navmesh, triangle_index])
            .u32();
        e.call(TRIANGLE_LINK_EDGE, &args![triangle, edge, number]);
    });
}

// Translated from 006c4390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Joins two navmeshes' portals (engine map: unnamed; `thiscall` on the
/// manager, which it does not use, with four stack words): `first_navmesh`
/// and `second_navmesh` are the addresses of `NiPointer<NavMesh>` slots and
/// `first_portal` / `second_portal` portal records. Each side is linked with
/// [`link_portal_side`]: first the triangle `first_portal` names in the first
/// navmesh, with the edge record built from `second_portal`; then the
/// triangle `second_portal` names in the second navmesh, with the edge
/// record built from `first_portal`. Returns true.
pub fn fn_006c4390(
    e: &mut Engine,
    _this: Ptr<NavMeshObstacleManager>,
    first_navmesh: u32,
    first_portal: u32,
    second_navmesh: u32,
    second_portal: u32,
) -> bool {
    link_portal_side(e, first_navmesh, first_portal, second_portal);
    link_portal_side(e, second_navmesh, second_portal, first_portal);
    true
}

// Translated from 006c4460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Links one side of a portal (engine map: unnamed; `thiscall` on the manager,
/// which it does not use, with three stack words): the second half of
/// [`fn_006c4390`] for the navmesh in `navmesh` and the triangle `portal`
/// names, with the edge record built from `other`. Returns true.
pub fn fn_006c4460(
    e: &mut Engine,
    _this: Ptr<NavMeshObstacleManager>,
    navmesh: u32,
    portal: u32,
    other: u32,
) -> bool {
    link_portal_side(e, navmesh, portal, other);
    true
}

// Translated from 006c44d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finishes the processed tasks (engine map: unnamed): runs `006c85e0` on
/// every task of `ProcessedTasks`; true when any of them returned true.
pub fn fn_006c44d0(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) -> bool {
    let tasks = this.addr() + NavMeshObstacleManager::ProcessedTasks.off;
    let mut any = false;
    let count = array_size(e, tasks);
    let mut index = 0;
    while index < count {
        let element = array_at(e, tasks, index);
        let task = word(e, element);
        if e.call(OBSTACLE_TASK_FINISH, &args![task]).bool() {
            any = true;
        }
        index += 1;
    }
    any
}

// Translated from 006c4540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the processed tasks (engine map: unnamed): every non-null task of
/// `ProcessedTasks` gets its deleting destructor (virtual slot 0, flag 1);
/// then `ProcessedTasks` and `BackgroundTasks` are cleared (`Clear(1)`).
/// Returns true.
pub fn fn_006c4540(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) -> bool {
    let tasks = this.addr() + NavMeshObstacleManager::ProcessedTasks.off;
    let count = array_size(e, tasks);
    let mut index = 0;
    while index < count {
        let element = array_at(e, tasks, index);
        let task = word(e, element);
        if task != 0 {
            e.vcall(task, 0, &args![1u32]);
        }
        index += 1;
    }
    e.call(ARRAY_CLEAR, &args![tasks, 1u32]);
    e.call(
        ARRAY_CLEAR,
        &args![
            this.addr() + NavMeshObstacleManager::BackgroundTasks.off,
            1u32
        ],
    );
    true
}

// Translated from 006c3ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleManager::ReconnectPortals` (Xbox PDB): after a round of
/// obstacle tasks, joins the portals the tasks cut. For every processed task
/// that has a new navmesh (`spNewNavMesh`), every portal record of its
/// `PortalSwaps` is looked at: a record whose triangle index is out of range
/// logs "PATHFINDING: (NavMesh Obstacle Manager) Bad triangle index while
/// trying to reconnect portals" and is skipped. Otherwise the two end points
/// of the record's edge are compared with those of the portal records of
/// every other task: when one task's end is within `50` units (squared x-y distance
/// under `2500.0`) of the other's opposite end, the two navmeshes' triangles are
/// linked (`006c4390`), both tasks get `bPortalModified` and both records are
/// removed. A record without a partner is tried against the task's second
/// array: `GetMatchingEdge` on the source mesh gives the partner's
/// `NavMeshInfo`; its navmesh is found among the processed tasks, or loaded
/// from the info (`0069ad00`), and when the partner triangle is not flagged
/// `0x20` and its edge ends are within the same distance the portals are
/// joined; when there is no navmesh but the source mesh has extra edge
/// information (`GetEdgeExtraInfo`), only this side is linked (`006c4460`).
/// Always returns true.
pub fn nav_mesh_obstacle_manager_reconnect_portals(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
) -> bool {
    let tasks = this.addr() + NavMeshObstacleManager::ProcessedTasks.off;
    let count = array_size(e, tasks);
    e.with_stack(0x150, |e, frame| {
        let top = frame.addr() + 0x150;
        let mut task_index = 0;
        while task_index < count {
            let element = array_at(e, tasks, task_index);
            let task = word(e, element);
            if e.call(HOLDER_POINTER, &args![task + TASK_NEW_MESH]).u32() != 0 {
                let portals = task + TASK_PORTAL_SWAPS;
                let mut portal_index = 0;
                while portal_index < array_size(e, portals) {
                    let removed = reconnect_portal(
                        e,
                        this,
                        top,
                        (tasks, count, task_index, task),
                        portal_index,
                    );
                    if !removed {
                        portal_index += 1;
                    }
                }
            }
            task_index += 1;
        }
    });
    true
}

/// One portal record of [`nav_mesh_obstacle_manager_reconnect_portals`]
/// (index `portal_index` of the `PortalSwaps` of `task`, which is element
/// `task_index` of `tasks`): returns true when the record was removed (the
/// caller then looks at the same index again) and false when it stays. `top`
/// is the frame the game keeps its locals in.
fn reconnect_portal(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
    top: u32,
    (tasks, count, task_index, task): (u32, u32, u32, u32),
    portal_index: u32,
) -> bool {
    let at = |below: u32| top - below;
    let portals = task + TASK_PORTAL_SWAPS;
    let record = at(0x38);
    let limit = e.global::<f32>(PORTAL_SQUARED_DISTANCE);
    let entry = e.call(RECORD_ARRAY_AT, &args![portals, portal_index]).u32();
    fn_006c4350(e, Ptr::new(record), entry);
    let triangle = e.mem.u16(record + RECORD_TRIANGLE) as u32;
    let edge = e.mem.u32(record + RECORD_EDGE);
    let navmesh = pointer_get(e, task + TASK_NEW_MESH);
    let triangle_count = e.call(NAVMESH_TRIANGLE_COUNT, &args![navmesh]).u32();
    if triangle >= triangle_count {
        e.call(LOG_MESSAGE, &args![BAD_TRIANGLE_MESSAGE]);
        return false;
    }
    // The end points of the record's edge: `first_end`, `second_end`.
    let (first_end, second_end) = (at(0x48), at(0x2c));
    edge_end_points(
        e,
        task + TASK_NEW_MESH,
        (triangle, edge),
        (at(0xdc), at(0xe4)),
        (first_end, second_end),
    );

    // The same edge among the portals of the other tasks.
    let mut found = false;
    let mut other_index = 0;
    while other_index < count {
        if other_index != task_index {
            let element = array_at(e, tasks, other_index);
            let other = word(e, element);
            if e.call(HOLDER_POINTER, &args![other + TASK_NEW_MESH]).u32() != 0 {
                let other_portals = other + TASK_PORTAL_SWAPS;
                let other_record = at(0x70);
                let mut other_portal = 0;
                while other_portal < array_size(e, other_portals) {
                    let entry = e
                        .call(RECORD_ARRAY_AT, &args![other_portals, other_portal])
                        .u32();
                    fn_006c4350(e, Ptr::new(other_record), entry);
                    let other_triangle = e.mem.u16(other_record + RECORD_TRIANGLE) as u32;
                    let other_edge = e.mem.u32(other_record + RECORD_EDGE);
                    let navmesh = pointer_get(e, other + TASK_NEW_MESH);
                    let other_count = e.call(NAVMESH_TRIANGLE_COUNT, &args![navmesh]).u32();
                    if other_triangle >= other_count {
                        e.call(LOG_MESSAGE, &args![BAD_TRIANGLE_MESSAGE]);
                        other_portal += 1;
                        continue;
                    }
                    let (other_first, other_second) = (at(0x64), at(0x7c));
                    edge_end_points(
                        e,
                        other + TASK_NEW_MESH,
                        (other_triangle, other_edge),
                        (at(0xec), at(0xf4)),
                        (other_first, other_second),
                    );
                    if ends_are_close(e, second_end, other_first, at(0x100), limit)
                        && ends_are_close(e, first_end, other_second, at(0x10c), limit)
                    {
                        fn_006c4390(
                            e,
                            this,
                            task + TASK_NEW_MESH,
                            record,
                            other + TASK_NEW_MESH,
                            other_record,
                        );
                        e.mem.set_u8(task + TASK_PORTAL_MODIFIED, 1);
                        e.mem.set_u8(other + TASK_PORTAL_MODIFIED, 1);
                        e.call(RECORD_ARRAY_REMOVE, &args![portals, portal_index, 1u32]);
                        e.call(
                            RECORD_ARRAY_REMOVE,
                            &args![other_portals, other_portal, 1u32],
                        );
                        found = true;
                        break;
                    }
                    other_portal += 1;
                }
                if found {
                    break;
                }
            }
        }
        other_index += 1;
    }
    if found {
        return true;
    }

    // No partner among the new navmeshes: the second array of the task.
    let second_portals = task + TASK_SECOND_PORTAL_ARRAY;
    let mut second_index = 0;
    while second_index < array_size(e, second_portals) {
        let entry = e
            .call(RECORD_ARRAY_AT, &args![second_portals, second_index])
            .u32();
        let partner_edge = at(0x8c);
        fn_006c4350(e, Ptr::new(partner_edge), entry);
        let matching = at(0x98);
        e.call(MATCHING_EDGE_DEFAULT, &args![matching]);
        let source_mesh = pointer_get(e, task + TASK_SOURCE_MESH);
        let has_match = e
            .call(
                NAVMESH_MATCHING_EDGE,
                &args![source_mesh, partner_edge, matching],
            )
            .bool();
        if !has_match || e.mem.u32(partner_edge) == e.mem.u32(matching) {
            second_index += 1;
            continue;
        }
        let holder = at(0xa0);
        let partner_task_slot = at(0x9c);
        e.call(NAVMESH_HOLDER_CTOR, &args![holder]);
        e.mem.set_u32(partner_task_slot, 0);
        let partner_info = e.mem.u32(matching);
        let mut scan = 0;
        while scan < count {
            let element = array_at(e, tasks, scan);
            let candidate = word(e, element);
            if e.mem.u32(candidate + TASK_INFO) == partner_info {
                let element = array_at(e, tasks, scan);
                let candidate = word(e, element);
                e.call(
                    NAVMESH_HOLDER_ASSIGN,
                    &args![holder, candidate + TASK_NEW_MESH],
                );
                let element = array_at(e, tasks, scan);
                let candidate = word(e, element);
                e.mem.set_u32(partner_task_slot, candidate);
                break;
            }
            scan += 1;
        }
        if pointer_get(e, holder) == 0 {
            e.call(NAVMESH_INFO_LOAD_HOLDER, &args![partner_info, holder]);
            e.mem.set_u32(partner_task_slot, 0);
        }
        if pointer_get(e, holder) == 0 {
            // The partner navmesh is not loaded: link this side from the
            // extra edge information, when the source mesh has it.
            let entry = e
                .call(RECORD_ARRAY_AT, &args![second_portals, second_index])
                .u32();
            let own_edge = at(0xb0);
            fn_006c4350(e, Ptr::new(own_edge), entry);
            let triangle = e.mem.u16(own_edge + RECORD_TRIANGLE) as u32;
            let edge = e.mem.u32(own_edge + RECORD_EDGE);
            let source_mesh = pointer_get(e, task + TASK_SOURCE_MESH);
            let extra = e
                .call(NAVMESH_EDGE_EXTRA_INFO, &args![source_mesh, triangle, edge])
                .u32();
            if extra != 0 {
                let extra_record = at(0xbc);
                let extra_first = e.mem.u32(extra + 4);
                let extra_second = e.mem.u16(extra + 8) as u32;
                e.call(
                    EDGE_RECORD_SET,
                    &args![extra_record, extra_first, extra_second],
                );
                fn_006c4460(e, this, task + TASK_NEW_MESH, record, extra_record);
                e.call(RECORD_ARRAY_REMOVE, &args![portals, portal_index, 1u32]);
                e.call(NAVMESH_HOLDER_DTOR, &args![holder]);
                return true;
            }
            e.call(NAVMESH_HOLDER_DTOR, &args![holder]);
            second_index += 1;
            continue;
        }
        let partner_triangle = e.mem.u16(matching + RECORD_TRIANGLE) as u32;
        let partner_mesh = pointer_get(e, holder);
        let partner_count = e.call(NAVMESH_TRIANGLE_COUNT, &args![partner_mesh]).u32();
        if partner_triangle >= partner_count {
            e.call(NAVMESH_HOLDER_DTOR, &args![holder]);
            second_index += 1;
            continue;
        }
        let partner_mesh = pointer_get(e, holder);
        let triangle_entry = e
            .call(NAVMESH_TRIANGLE, &args![partner_mesh, partner_triangle])
            .u32();
        if e.call(TRIANGLE_HAS_FLAG, &args![triangle_entry, 0x20u32])
            .bool()
        {
            e.call(NAVMESH_HOLDER_DTOR, &args![holder]);
            second_index += 1;
            continue;
        }
        let partner_edge_number = e.mem.u32(matching + RECORD_EDGE);
        let (partner_first, partner_second) = (at(0xc8), at(0xd4));
        edge_end_points(
            e,
            holder,
            (partner_triangle, partner_edge_number),
            (at(0x114), at(0x11c)),
            (partner_first, partner_second),
        );
        if ends_are_close(e, second_end, partner_first, at(0x128), limit)
            && ends_are_close(e, first_end, partner_second, at(0x134), limit)
        {
            fn_006c4390(e, this, task + TASK_NEW_MESH, record, holder, matching);
            e.mem.set_u8(task + TASK_PORTAL_MODIFIED, 1);
            let partner_task = e.mem.u32(partner_task_slot);
            if partner_task != 0 {
                e.mem.set_u8(partner_task + TASK_PORTAL_MODIFIED, 1);
            }
            e.call(RECORD_ARRAY_REMOVE, &args![portals, portal_index, 1u32]);
            e.call(NAVMESH_HOLDER_DTOR, &args![holder]);
            return true;
        }
        e.call(NAVMESH_HOLDER_DTOR, &args![holder]);
        second_index += 1;
    }
    false
}

/// The two end points of an edge: `NavMesh::GetEdgeVertices` on the navmesh
/// held at `navmesh_slot` fills the pair of vertex addresses at `outputs.0`
/// (for the first end) and `outputs.1` (for the second); the vertices are
/// copied to the points `points.0` and `points.1`.
fn edge_end_points(
    e: &mut Engine,
    navmesh_slot: u32,
    (triangle, edge): (u32, u32),
    outputs: (u32, u32),
    points: (u32, u32),
) {
    let navmesh = pointer_get(e, navmesh_slot);
    let pair = e
        .call(
            NAVMESH_EDGE_VERTICES,
            &args![navmesh, outputs.0, triangle, edge],
        )
        .u32();
    let vertex = word(e, pair);
    copy_point(e, points.0, vertex);
    let navmesh = pointer_get(e, navmesh_slot);
    let pair = e
        .call(
            NAVMESH_EDGE_VERTICES,
            &args![navmesh, outputs.1, triangle, edge],
        )
        .u32();
    let vertex = word(e, pair + 4);
    copy_point(e, points.1, vertex);
}

/// Whether `first - second` is shorter than the portal distance: the difference
/// goes to `difference` (`00439ef0`) and its squared length in the x-y plane
/// (`00595c80`) must be under `limit`.
fn ends_are_close(e: &mut Engine, first: u32, second: u32, difference: u32, limit: f32) -> bool {
    let difference = e
        .call(POINT3_SUBTRACT, &args![first, difference, second])
        .u32();
    let squared = e.call(POINT3_SQUARED_LENGTH_XY, &args![difference]).f32();
    squared < limit
}

// Translated from 006c45e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Handles the queued references to add (engine map: unnamed): for every
/// reference of `QueuedListOfRefsToAdd`: if `FormIDMap` already has its form
/// id, its obstacles are replaced (`006c1920`) and the reference is kept to be
/// removed from the queue; otherwise, if it has a 3D node (virtual slot
/// `0x1d0`), a scratch `ObstacleData` is filled from it (`006c2550`) and,
/// when its rigid body answers true to virtual slot `0x94`, the reference's
/// obstacles are added (`006c1190`) and the reference kept. At the end the kept
/// references are removed from the queued list (`00905330`).
pub fn fn_006c45e0(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    let manager = this.addr();
    e.with_stack(0x100, |e, frame| {
        let top = frame.addr() + 0x100;
        let handled = top - 0x20;
        let array_slot = top - 0x24;
        let reference_slot = top - 0x28;
        let scratch = top - 0xbc;
        e.call(REFERENCE_ARRAY_CTOR, &args![handled]);
        let queue = manager + NavMeshObstacleManager::QueuedListOfRefsToAdd.off;
        let mut node = queue;
        while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
            let item = e.call(POINT3_DEFAULT, &args![node]).u32();
            let reference = word(e, item);
            e.mem.set_u32(reference_slot, reference);
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
            pointer_init(e, array_slot, 0);
            let form_id = e.call(FORM_ID, &args![reference]).u32();
            let form_id_map = manager + NavMeshObstacleManager::FormIDMap.off;
            if map_get(e, form_id_map, form_id, array_slot) {
                let form_id = e.call(FORM_ID, &args![reference]).u32();
                fn_006c1920(e, this, form_id, reference);
                e.call(ARRAY_ADD, &args![handled, reference_slot]);
            } else if e.vcall(reference, REFERENCE_NODE_SLOT, &args![]).u32() != 0 {
                fn_006c23c0(e, Ptr::new(scratch));
                let node_3d = e.vcall(reference, REFERENCE_NODE_SLOT, &args![]).u32();
                if fn_006c2550(e, node_3d, scratch) {
                    let rigid_body = pointer_get(e, scratch + ObstacleData::pRigidBody.off);
                    if e.vcall(rigid_body, RIGID_BODY_SLOT_0X94, &args![]).u32() != 0 {
                        fn_006c1190(e, this, reference);
                        e.call(ARRAY_ADD, &args![handled, reference_slot]);
                    }
                }
                fn_006c24d0(e, Ptr::new(scratch));
            }
            pointer_release(e, array_slot);
        }
        let mut index = 0;
        while index < array_size(e, handled) {
            let element = array_at(e, handled, index);
            e.call(LIST_REMOVE, &args![queue, element]);
            index += 1;
        }
        e.call(REFERENCE_ARRAY_DTOR, &args![handled]);
    });
}

// Translated from 006c47e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Handles the queued form ids to remove (engine map: unnamed): for every id
/// of `QueuedListOfRefsToRemove` the obstacles of its `FormIDMap` entry
/// leave `ObstaclesUpdateList` (`00631620`) and the reference's obstacles are
/// removed (`006c1590`); the queued list is then cleared.
pub fn fn_006c47e0(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    let manager = this.addr();
    e.with_stack(8, |e, frame| {
        let array_slot = frame.addr();
        let mut node = manager + NavMeshObstacleManager::QueuedListOfRefsToRemove.off;
        while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
            let item = e.call(POINT3_DEFAULT, &args![node]).u32();
            let form_id = word(e, item);
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
            pointer_init(e, array_slot, 0);
            let form_id_map = manager + NavMeshObstacleManager::FormIDMap.off;
            if map_get(e, form_id_map, form_id, array_slot) {
                let mut index = 0;
                while index < obstacle_count(e, array_slot) {
                    let obstacles = obstacles_of(e, array_slot);
                    let element = array_at(e, obstacles, index);
                    bs_simple_list_obstacle_data_remove(
                        e,
                        Ptr::new(manager + NavMeshObstacleManager::ObstaclesUpdateList.off),
                        Ptr::new(element),
                    );
                    index += 1;
                }
            }
            fn_006c1590(e, this, form_id);
            pointer_release(e, array_slot);
        }
    });
    e.call(
        LIST_CLEAR,
        &args![manager + NavMeshObstacleManager::QueuedListOfRefsToRemove.off],
    );
}

/// Applies `SetDisableNavMesh(id, disable)` (`00694500`, `cdecl`) to every id
/// of the queued list at `list`, then clears the list.
fn apply_queued_navmeshes(e: &mut Engine, list: u32, disable: u32) {
    let mut node = list;
    while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
        let item = e.call(POINT3_DEFAULT, &args![node]).u32();
        let id = word(e, item);
        e.call(SET_DISABLE_NAVMESH, &args![id, disable]);
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    e.call(LIST_CLEAR, &args![list]);
}

// Translated from 006c4900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Enables the queued navmeshes (engine map: unnamed): `SetDisableNavMesh(id,
/// 0)` for every id of `QueuedListOfNavmeshesToEnable`, then clears the list.
/// Returns true.
pub fn fn_006c4900(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) -> bool {
    let list = this.addr() + NavMeshObstacleManager::QueuedListOfNavmeshesToEnable.off;
    apply_queued_navmeshes(e, list, 0);
    true
}

// Translated from 006c4970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Disables the queued navmeshes (engine map: unnamed): `SetDisableNavMesh(id,
/// 1)` for every id of `QueuedListOfNavmeshesToDisable`, then clears the
/// list. Returns true.
pub fn fn_006c4970(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) -> bool {
    let list = this.addr() + NavMeshObstacleManager::QueuedListOfNavmeshesToDisable.off;
    apply_queued_navmeshes(e, list, 1);
    true
}

// Translated from 006c49e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drains `ObstaclesToAddToUpdateList` (engine map: unnamed): for each value
/// the queue holds (count from virtual slot `0x10`, value from `006c6030`),
/// the obstacle `RigidBodyToObstacleMap` has for that rigid body, if any, is
/// added to `ObstaclesUpdateList` (`00631540`) unless it is already there
/// (`006c6400`).
pub fn fn_006c49e0(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    let manager = this.addr();
    let queue = manager + NavMeshObstacleManager::ObstaclesToAddToUpdateList.off;
    let count = e.vcall(queue, QUEUE_COUNT_SLOT, &args![]).u32();
    let mut index = 0;
    while index < count {
        e.with_stack(8, |e, frame| {
            let rigid_body_slot = frame.addr();
            let obstacle = frame.addr() + 4;
            e.call(QUEUE_POP, &args![queue, rigid_body_slot]);
            pointer_init(e, obstacle, 0);
            let rigid_body = word(e, rigid_body_slot);
            let map = manager + NavMeshObstacleManager::RigidBodyToObstacleMap.off;
            if map_get(e, map, rigid_body, obstacle) {
                let list = manager + NavMeshObstacleManager::ObstaclesUpdateList.off;
                if !e
                    .call(OBSTACLE_LIST_CONTAINS, &args![list, obstacle])
                    .bool()
                {
                    e.call(OBSTACLE_LIST_ADD, &args![list, obstacle]);
                }
            }
            pointer_release(e, obstacle);
        });
        index += 1;
    }
}

// Translated from 006c4ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Drains `ObstaclesToRemoveFromUpdateList` (engine map: unnamed): for each
/// value the queue holds, the obstacle `RigidBodyToObstacleMap` has for that
/// rigid body, if any, is re-registered with `006c1760` when the rigid body
/// has moved more than `5` units (its position, `004b4ec0` of the Havok
/// object `00620b80`, against the obstacle's `Center`: squared distance over
/// `25.0`), and then leaves `ObstaclesUpdateList` (`00631620`). The
/// compiler aligned this function's stack to 16 bytes for the `hkVector4`
/// local; that is not translated.
pub fn fn_006c4ad0(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    let manager = this.addr();
    let queue = manager + NavMeshObstacleManager::ObstaclesToRemoveFromUpdateList.off;
    let count = e.vcall(queue, QUEUE_COUNT_SLOT, &args![]).u32();
    let mut index = 0;
    while index < count {
        e.with_stack(0x40, |e, frame| {
            let top = frame.addr() + 0x40;
            let rigid_body_slot = top - 0x04;
            let obstacle = top - 0x08;
            let position = top - 0x20;
            let point = top - 0x2c;
            let difference = top - 0x38;
            e.call(QUEUE_POP, &args![queue, rigid_body_slot]);
            pointer_init(e, obstacle, 0);
            let rigid_body = word(e, rigid_body_slot);
            let map = manager + NavMeshObstacleManager::RigidBodyToObstacleMap.off;
            if map_get(e, map, rigid_body, obstacle) {
                let havok_object = e.call(GET_WORD_AT_8, &args![rigid_body]).u32();
                let source = e.call(HAVOK_POSITION, &args![havok_object]).u32();
                e.call(HK_VECTOR_COPY, &args![position, source]);
                e.call(POINT3_DEFAULT, &args![point]);
                e.call(HK_VECTOR_TO_POINT3, &args![point, position]);
                let data = pointer_get(e, obstacle);
                let center = data + ObstacleData::Center.off;
                let difference = e
                    .call(POINT3_SUBTRACT, &args![point, difference, center])
                    .u32();
                let squared = e.call(POINT3_SQUARED_LENGTH, &args![difference]).f32();
                if e.global::<f32>(OBSTACLE_MOVED_SQUARED_DISTANCE) < squared {
                    fn_006c1760(e, this, obstacle);
                }
                bs_simple_list_obstacle_data_remove(
                    e,
                    Ptr::new(manager + NavMeshObstacleManager::ObstaclesUpdateList.off),
                    Ptr::new(obstacle),
                );
            }
            pointer_release(e, obstacle);
        });
        index += 1;
    }
}

// Translated from 006c4c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Re-registers every obstacle (engine map: unnamed): walks `FormIDMap` from
/// its first item (`GetNext`) and calls `006c1760` on each obstacle of each
/// entry.
pub fn fn_006c4c30(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    let map = this.addr() + NavMeshObstacleManager::FormIDMap.off;
    e.with_stack(0x10, |e, frame| {
        let position = frame.addr();
        let key = frame.addr() + 4;
        let array = frame.addr() + 8;
        let obstacle = frame.addr() + 12;
        let first = e.call(MAP_FIRST_ITEM, &args![map]).u32();
        e.mem.set_u32(position, first);
        while word(e, position) != 0 {
            pointer_init(e, array, 0);
            ni_tmap_base_reference_obstacle_array_get_next(
                e,
                Ptr::new(map),
                Ptr::new(position),
                Ptr::new(key),
                Ptr::new(array),
            );
            let mut index = 0;
            while index < obstacle_count(e, array) {
                let obstacles = obstacles_of(e, array);
                let element = array_at(e, obstacles, index);
                pointer_copy(e, obstacle, element);
                fn_006c1760(e, this, obstacle);
                pointer_release(e, obstacle);
                index += 1;
            }
            pointer_release(e, array);
        }
    });
}

/// The low word of the 64-bit integer `FISTP` makes of `value` with the
/// control word set to truncate (the "integer indefinite" `0x8000000000000000`
/// for a value that does not fit).
fn truncate_to_low_word(value: f64) -> u32 {
    if value.is_nan()
        || value >= 9_223_372_036_854_775_808.0
        || value < -9_223_372_036_854_775_808.0
    {
        0
    } else {
        value.trunc() as i64 as u32
    }
}

// Translated from 006c4d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Re-registers the obstacles that moved (engine map: unnamed): the delay is
/// the setting `011d71b0` (seconds) times `1000.0`, truncated to
/// milliseconds. Every obstacle of `ObstaclesUpdateList` whose rigid body is
/// more than `5` units (squared `25.0`) from its `Center` and whose
/// `iLastUpdateTime` is at least that delay before the clock (`00825c00` on
/// `011f6394`, unsigned difference) is passed to `006c1760`. The compiler
/// aligned this function's stack; not translated.
pub fn fn_006c4d30(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    let manager = this.addr();
    let setting = e
        .call(
            FLOAT_SETTING_VALUE_ADDRESS,
            &args![OBSTACLE_REFRESH_DELAY_SETTING],
        )
        .u32();
    let seconds = e.mem.f32(setting);
    let delay_in_milliseconds = e.global::<f64>(MILLISECONDS_PER_SECOND_DOUBLE);
    let delay = truncate_to_low_word(seconds as f64 * delay_in_milliseconds);
    let mut node = manager + NavMeshObstacleManager::ObstaclesUpdateList.off;
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        e.with_stack(0x40, |e, frame| {
            let top = frame.addr() + 0x40;
            let obstacle = top - 0x04;
            let position = top - 0x20;
            let point = top - 0x2c;
            let difference = top - 0x38;
            let item = e.call(POINT3_DEFAULT, &args![node]).u32();
            pointer_copy(e, obstacle, item);
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
            let data = pointer_get(e, obstacle);
            let rigid_body = pointer_get(e, data + ObstacleData::pRigidBody.off);
            let havok_object = e.call(GET_WORD_AT_8, &args![rigid_body]).u32();
            let source = e.call(HAVOK_POSITION, &args![havok_object]).u32();
            e.call(HK_VECTOR_COPY, &args![position, source]);
            e.call(POINT3_DEFAULT, &args![point]);
            e.call(HK_VECTOR_TO_POINT3, &args![point, position]);
            let data = pointer_get(e, obstacle);
            let center = data + ObstacleData::Center.off;
            let difference = e
                .call(POINT3_SUBTRACT, &args![point, difference, center])
                .u32();
            let squared = e.call(POINT3_SQUARED_LENGTH, &args![difference]).f32();
            if e.global::<f32>(OBSTACLE_MOVED_SQUARED_DISTANCE) < squared {
                let now = e.call(CLOCK_MILLISECONDS, &args![CLOCK_OBJECT]).u32();
                let data = pointer_get(e, obstacle);
                let last = e.mem.u32(data + ObstacleData::iLastUpdateTime.off);
                if now.wrapping_sub(last) >= delay {
                    fn_006c1760(e, this, obstacle);
                }
            }
            pointer_release(e, obstacle);
        });
    }
}

// Translated from 006c4ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands the collected tasks on (engine map: unnamed): for every entry of
/// `CurrentNavMeshTaskMap` (`GetNext` of the map, `006b7f20`): when the
/// setting `011d71a0` is on the task goes to `006c5ce0`; otherwise it is added
/// to `BackgroundTasks` under `ProcessedTaskLock` and started through its
/// virtual slot 2 (`+8`). The map is cleared (`00438af0`) at the end.
pub fn fn_006c4ec0(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    let manager = this.addr();
    let map = manager + NavMeshObstacleManager::CurrentNavMeshTaskMap.off;
    e.with_stack(0x10, |e, frame| {
        let position = frame.addr();
        let key = frame.addr() + 4;
        let task_slot = frame.addr() + 8;
        let first = e.call(MAP_FIRST_ITEM, &args![map]).u32();
        e.mem.set_u32(position, first);
        while word(e, position) != 0 {
            e.call(TASK_MAP_GET_NEXT, &args![map, position, key, task_slot]);
            let setting = e
                .call(SETTING_VALUE_ADDRESS, &args![BACKGROUND_TASKS_SETTING])
                .u32();
            let task = word(e, task_slot);
            if e.mem.u8(setting) != 0 {
                e.call(QUEUE_TASK, &args![this, task]);
            } else {
                let lock = manager + NavMeshObstacleManager::ProcessedTaskLock.off;
                e.call(SPIN_LOCK_LOCK, &args![lock, 0u32]);
                e.call(
                    ARRAY_ADD,
                    &args![
                        manager + NavMeshObstacleManager::BackgroundTasks.off,
                        task_slot
                    ],
                );
                e.call(SPIN_LOCK_UNLOCK, &args![lock]);
                let task = word(e, task_slot);
                e.vcall(task, 8, &args![]);
            }
        }
    });
    e.call(MAP_CLEAR, &args![map]);
}

// Translated from 006c4f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleManager::AddProcessedTask` (Xbox PDB): adds a task to
/// `ProcessedTasks` under `ProcessedTaskLock`.
pub fn nav_mesh_obstacle_manager_add_processed_task(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
    task: u32,
) {
    let lock = this.addr() + NavMeshObstacleManager::ProcessedTaskLock.off;
    e.call(SPIN_LOCK_LOCK, &args![lock, ADD_PROCESSED_LOCK_NAME]);
    list_operation(
        e,
        ARRAY_ADD,
        this.addr() + NavMeshObstacleManager::ProcessedTasks.off,
        task,
    );
    e.call(SPIN_LOCK_UNLOCK, &args![lock]);
}

// Translated from 006c4fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Handles the doors queued to open (engine map: unnamed): false when
/// `QueuedListOfDoorsToRemove` is empty. For each queued form id found in
/// `ClosedDoorMap`, operation `0x40` is queued on the task of every navmesh
/// each of the entry's obstacles has recorded, the id leaves `ClosedDoorMap`
/// and the entry is stored in `OpenDoorMap`. The queued list is cleared and
/// true returned.
pub fn fn_006c4fb0(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) -> bool {
    let manager = this.addr();
    let queue = manager + NavMeshObstacleManager::QueuedListOfDoorsToRemove.off;
    if e.call(LIST_NODE_IS_EMPTY, &args![queue]).bool() {
        return false;
    }
    e.with_stack(8, |e, frame| {
        let array = frame.addr();
        let mut node = queue;
        while node != 0 {
            let item = e.call(POINT3_DEFAULT, &args![node]).u32();
            let form_id = word(e, item);
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
            pointer_init(e, array, 0);
            let closed = manager + NavMeshObstacleManager::ClosedDoorMap.off;
            if map_get(e, closed, form_id, array) {
                let mut index = 0;
                while index < obstacle_count(e, array) {
                    let mut navmesh_index = 0;
                    loop {
                        let obstacles = obstacles_of(e, array);
                        let obstacle = array_at(e, obstacles, index);
                        if navmesh_index >= navmesh_count(e, obstacle) {
                            break;
                        }
                        let obstacles = obstacles_of(e, array);
                        let obstacle = array_at(e, obstacles, index);
                        let obstacles_again = obstacles_of(e, array);
                        let same = array_at(e, obstacles_again, index);
                        let navmeshes = navmeshes_of(e, same);
                        let recorded = array_at(e, navmeshes, navmesh_index);
                        let info = word(e, recorded);
                        queue_operation(e, this, info, obstacle, 0x40);
                        navmesh_index += 1;
                    }
                    index += 1;
                }
                e.call(MAP_REMOVE, &args![closed, form_id]);
                map_set_copy(
                    e,
                    manager + NavMeshObstacleManager::OpenDoorMap.off,
                    form_id,
                    array,
                );
            }
            pointer_release(e, array);
        }
    });
    e.call(LIST_CLEAR, &args![queue]);
    true
}

// Translated from 006c5190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Handles the doors queued to close (engine map: unnamed): false when
/// `QueuedListOfDoorsToAdd` is empty. For the first reference of the list that
/// has a 3D node (virtual slot `0x1d0`) a new `ReferenceObstacleArray` is made
/// (`0x1c` bytes, `006c1480`) and filled (`006c34f0`); on success operation
/// `0x30` is queued on the task of every navmesh each obstacle overlaps
/// (recorded in its `Navmeshes`), the reference's id leaves `OpenDoorMap` and
/// the array is stored in `ClosedDoorMap`. The reference then leaves the
/// queued list and the walk starts again at the head; a reference without a 3D
/// node is skipped. Returns whether any door was handled.
pub fn fn_006c5190(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) -> bool {
    let manager = this.addr();
    let queue = manager + NavMeshObstacleManager::QueuedListOfDoorsToAdd.off;
    if e.call(LIST_NODE_IS_EMPTY, &args![queue]).bool() {
        return false;
    }
    let mut handled = false;
    e.with_stack(0x10, |e, frame| {
        let reference_slot = frame.addr();
        let array = frame.addr() + 4;
        let mut node = queue;
        while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
            let item = e.call(POINT3_DEFAULT, &args![node]).u32();
            let reference = word(e, item);
            e.mem.set_u32(reference_slot, reference);
            if e.vcall(reference, REFERENCE_NODE_SLOT, &args![]).u32() != 0 {
                let memory = e.call(OPERATOR_NEW_SIZED, &args![0x1cu32]).u32();
                let created = if memory != 0 {
                    reference_obstacle_array_construct(e, Ptr::new(memory)).addr()
                } else {
                    0
                };
                pointer_init(e, array, created);
                if fn_006c34f0(e, reference, array) {
                    let mut index = 0;
                    while index < obstacle_count(e, array) {
                        let obstacles = obstacles_of(e, array);
                        let obstacle = array_at(e, obstacles, index);
                        queue_overlapping_navmeshes(e, this, obstacle, 0x30);
                        index += 1;
                    }
                    let form_id = e.call(FORM_ID, &args![reference]).u32();
                    e.call(
                        MAP_REMOVE,
                        &args![manager + NavMeshObstacleManager::OpenDoorMap.off, form_id],
                    );
                    let form_id = e.call(FORM_ID, &args![reference]).u32();
                    map_set_copy(
                        e,
                        manager + NavMeshObstacleManager::ClosedDoorMap.off,
                        form_id,
                        array,
                    );
                    handled = true;
                }
                e.call(LIST_REMOVE, &args![queue, reference_slot]);
                node = queue;
                pointer_release(e, array);
            } else {
                node = e.call(LIST_NODE_NEXT, &args![node]).u32();
            }
        }
    });
    handled
}

/// Attaches the child held at `child_slot` to the obstacle root node (virtual
/// slot `0xdc` of the node held in `spObstacleRootNode`, arguments the child
/// and `1`).
fn attach_to_root(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, child_slot: u32) {
    let root_slot = this.addr() + NavMeshObstacleManager::spObstacleRootNode.off;
    let root = pointer_get(e, root_slot);
    let child = pointer_get(e, child_slot);
    e.vcall(root, NODE_ATTACH_CHILD_SLOT, &args![child, 1u32]);
}

/// Updates the obstacle root node: `NiAVObject::UpdateProperties` on it, then
/// `NiUpdateData(0.0, 0, 0)` (`0043d410`) and the node's update (`00a59c60`)
/// with it, as the game does in `DrawObstacles` and `AddBoundingBox3DNode`.
fn update_root(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    let root_slot = this.addr() + NavMeshObstacleManager::spObstacleRootNode.off;
    let root = pointer_get(e, root_slot);
    e.call(UPDATE_PROPERTIES, &args![root]);
    e.with_stack(0x10, |e, update_data| {
        let update_data = update_data.addr();
        e.call(
            UPDATE_DATA_CONSTRUCT,
            &args![update_data, 0.0f32, 0u32, 0u32],
        );
        let root = pointer_get(e, root_slot);
        e.call(NODE_UPDATE, &args![root, update_data]);
    });
}

// Translated from 006c5460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleManager::DrawObstacles` (Xbox PDB): shows or hides the
/// debug boxes of the obstacles. With `draw` zero, the properties of the root
/// node are removed (`004def90`) when there is one and `bDrawObstacles` is
/// cleared. Otherwise a missing root node is made (an `NiNode` of `0xac`
/// bytes, a `0x1c`-byte alpha property switched on and a `0x4c`-byte
/// material property with the colour `(1, 0, 1)` and alpha `0.25`, both
/// attached to it, and the node attached under the scene root, the `TES`
/// object's `+0x0c`, with `NiUpdateData(0.0, 0, 0)` for its update); an
/// existing one only has its properties removed. A debug box
/// (`CreateBoundingBox3DNode`) is then made and attached for every obstacle
/// of every `FormIDMap` entry, the root's properties are updated, the root is
/// updated and `bDrawObstacles` is set.
pub fn nav_mesh_obstacle_manager_draw_obstacles(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
    draw: u8,
) {
    let manager = this.addr();
    let root_slot = manager + NavMeshObstacleManager::spObstacleRootNode.off;
    if draw == 0 {
        let root = pointer_get(e, root_slot);
        if root != 0 {
            let root = pointer_get(e, root_slot);
            e.call(NODE_REMOVE_PROPERTIES, &args![root]);
        }
        e.set(this, NavMeshObstacleManager::bDrawObstacles, false);
        return;
    }
    if pointer_get(e, root_slot) == 0 {
        make_obstacle_root(e, this);
    } else {
        let root = pointer_get(e, root_slot);
        e.call(NODE_REMOVE_PROPERTIES, &args![root]);
    }
    let form_id_map = manager + NavMeshObstacleManager::FormIDMap.off;
    e.with_stack(0x20, |e, frame| {
        let position = frame.addr();
        let key = frame.addr() + 4;
        let array = frame.addr() + 8;
        let obstacle = frame.addr() + 12;
        let node = frame.addr() + 16;
        let first = e.call(MAP_FIRST_ITEM, &args![form_id_map]).u32();
        e.mem.set_u32(position, first);
        while word(e, position) != 0 {
            pointer_init(e, array, 0);
            ni_tmap_base_reference_obstacle_array_get_next(
                e,
                Ptr::new(form_id_map),
                Ptr::new(position),
                Ptr::new(key),
                Ptr::new(array),
            );
            let mut index = 0;
            while index < obstacle_count(e, array) {
                let obstacles = obstacles_of(e, array);
                let element = array_at(e, obstacles, index);
                pointer_copy(e, obstacle, element);
                pointer_init(e, node, 0);
                nav_mesh_obstacle_manager_create_bounding_box3d_node(e, this, obstacle, node);
                attach_to_root(e, this, node);
                pointer_release(e, node);
                pointer_release(e, obstacle);
                index += 1;
            }
            pointer_release(e, array);
        }
    });
    update_root(e, this);
    e.set(this, NavMeshObstacleManager::bDrawObstacles, true);
}

/// The part of `DrawObstacles` that makes the obstacle root node: see
/// [`nav_mesh_obstacle_manager_draw_obstacles`].
fn make_obstacle_root(e: &mut Engine, this: Ptr<NavMeshObstacleManager>) {
    let root_slot = this.addr() + NavMeshObstacleManager::spObstacleRootNode.off;
    let memory = e.call(OPERATOR_NEW_SIZED, &args![NODE_SIZE]).u32();
    let node = if memory != 0 {
        e.call(NODE_CONSTRUCT, &args![memory, 0u32]).u32()
    } else {
        0
    };
    e.call(NI_POINTER_SET, &args![root_slot, node]);
    e.with_stack(0x30, |e, frame| {
        let alpha_slot = frame.addr();
        let material_slot = frame.addr() + 4;
        let color = frame.addr() + 8;
        let update_data = frame.addr() + 0x18;
        let memory = e
            .call(OPERATOR_NEW_SIZED, &args![ALPHA_PROPERTY_SIZE])
            .u32();
        let alpha = if memory != 0 {
            e.call(ALPHA_PROPERTY_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
        pointer_init(e, alpha_slot, alpha);
        let alpha = pointer_get(e, alpha_slot);
        e.call(ALPHA_PROPERTY_ENABLE, &args![alpha, 1u32]);
        let memory = e
            .call(OPERATOR_NEW_SIZED, &args![MATERIAL_PROPERTY_SIZE])
            .u32();
        let material = if memory != 0 {
            e.call(MATERIAL_PROPERTY_CONSTRUCT, &args![memory]).u32()
        } else {
            0
        };
        pointer_init(e, material_slot, material);
        let magenta = e
            .call(POINT3_CONSTRUCT, &args![color, 1.0f32, 0.0f32, 1.0f32])
            .u32();
        let material = pointer_get(e, material_slot);
        e.call(MATERIAL_SET_COLOR, &args![material, magenta]);
        let alpha_value = e.global::<f32>(DEBUG_BOX_ALPHA);
        let material = pointer_get(e, material_slot);
        e.call(MATERIAL_SET_ALPHA, &args![material, alpha_value]);
        let alpha = pointer_get(e, alpha_slot);
        let root = pointer_get(e, root_slot);
        e.call(ATTACH_PROPERTY, &args![root, alpha]);
        let material = pointer_get(e, material_slot);
        let root = pointer_get(e, root_slot);
        e.call(ATTACH_PROPERTY, &args![root, material]);
        let tes = e.global::<u32>(TES_POINTER);
        let scene_root = e.call(FORM_ID, &args![tes]).u32();
        let root = pointer_get(e, root_slot);
        e.vcall(scene_root, NODE_ATTACH_CHILD_SLOT, &args![root, 1u32]);
        e.call(
            UPDATE_DATA_CONSTRUCT,
            &args![update_data, 0.0f32, 0u32, 0u32],
        );
        let tes = e.global::<u32>(TES_POINTER);
        let scene_root = e.call(FORM_ID, &args![tes]).u32();
        e.call(NODE_UPDATE, &args![scene_root, update_data]);
        pointer_release(e, material_slot);
        pointer_release(e, alpha_slot);
    });
}

/// `data + offset` read as a `float` through the obstacle slot, the way the
/// game re-reads `NiPointer::operator->` for every component.
fn obstacle_float(e: &mut Engine, obstacle: u32, offset: u32) -> f32 {
    let data = pointer_get(e, obstacle);
    e.mem.f32(data + offset)
}

// Translated from 006c5840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleManager::CreateBoundingBox3DNode` (Xbox PDB): makes the
/// debug box of an obstacle. `obstacle` is the address of its
/// `NiPointer<ObstacleData>`, `node` the address of the `NiPointer` that
/// receives the shape. The eight corners of `BoxMin`..`BoxMax` (the order
/// `min`, `(max.x, min.y, min.z)`, `(min.x, max.y, min.z)`...: see the code)
/// and the colour `(1, 0, 1, 0.5)` go to `MakeQuadBox`; the shape is placed at
/// `Center` and `Orientation` (`00440460`, `0043fa80`) and stored in the
/// obstacle's `sp3DNode` and in `node`. (The subtraction `BoxMax - BoxMin` the
/// game makes first has no use.)
pub fn nav_mesh_obstacle_manager_create_bounding_box3d_node(
    e: &mut Engine,
    _this: Ptr<NavMeshObstacleManager>,
    obstacle: u32,
    node: u32,
) {
    e.with_stack(0x90, |e, frame| {
        let top = frame.addr() + 0x90;
        let at = |below: u32| top - below;
        let data = pointer_get(e, obstacle);
        let box_min_arg = data + ObstacleData::BoxMin.off;
        let data = pointer_get(e, obstacle);
        let box_max = data + ObstacleData::BoxMax.off;
        e.call(POINT3_SUBTRACT, &args![box_max, at(0x58), box_min_arg]);
        let half = e.global::<f32>(DEBUG_BOX_COLOR_ALPHA);
        e.call(
            COLOR_CONSTRUCT,
            &args![at(0x40), 1.0f32, 0.0f32, 1.0f32, half],
        );
        // Each corner takes the x, y and z from the minimum (`0x40`, `0x44`,
        // `0x48`) or the maximum (`0x4c`, `0x50`, `0x54`); the game reads z,
        // then y, then x. (point address, [x, y, z] offsets in the game's
        // order of arguments.)
        let corners: [(u32, [u32; 3]); 8] = [
            (0x24, [0x40, 0x44, 0x48]),
            (0x8c, [0x4c, 0x44, 0x48]),
            (0x70, [0x4c, 0x44, 0x54]),
            (0x18, [0x40, 0x44, 0x54]),
            (0x30, [0x40, 0x50, 0x48]),
            (0x4c, [0x4c, 0x50, 0x48]),
            (0x64, [0x4c, 0x50, 0x54]),
            (0x7c, [0x40, 0x50, 0x54]),
        ];
        for (point, [x, y, z]) in corners {
            let z = obstacle_float(e, obstacle, z);
            let y = obstacle_float(e, obstacle, y);
            let x = obstacle_float(e, obstacle, x);
            e.call(POINT3_CONSTRUCT, &args![at(point), x, y, z]);
        }
        let shape_slot = at(0x80);
        let shape = e
            .call(
                MAKE_QUAD_BOX,
                &args![
                    at(0x7c),
                    at(0x64),
                    at(0x70),
                    at(0x18),
                    at(0x30),
                    at(0x4c),
                    at(0x8c),
                    at(0x24),
                    at(0x40),
                    1u32
                ],
            )
            .u32();
        pointer_init(e, shape_slot, shape);
        let data = pointer_get(e, obstacle);
        let center = data + ObstacleData::Center.off;
        let shape_object = pointer_get(e, shape_slot);
        e.call(SHAPE_SET_CENTER, &args![shape_object, center]);
        let data = pointer_get(e, obstacle);
        let orientation = data + ObstacleData::Orientation.off;
        let shape_object = pointer_get(e, shape_slot);
        e.call(SHAPE_SET_ORIENTATION, &args![shape_object, orientation]);
        let data = pointer_get(e, obstacle);
        e.call(
            NI_POINTER_ASSIGN,
            &args![data + ObstacleData::sp3DNode.off, shape_slot],
        );
        e.call(NI_POINTER_ASSIGN, &args![node, shape_slot]);
        pointer_release(e, shape_slot);
    });
}

// Translated from 006c5b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshObstacleManager::AddBoundingBox3DNode` (Xbox PDB): gives an
/// obstacle a fresh debug box. `obstacle` is the address of its
/// `NiPointer<ObstacleData>`. An existing `sp3DNode` is first detached from
/// the root node (virtual slot `0xe8`); a new box is created
/// ([`nav_mesh_obstacle_manager_create_bounding_box3d_node`]), attached to the
/// root (slot `0xdc`) and the root's properties and update are refreshed.
pub fn nav_mesh_obstacle_manager_add_bounding_box3d_node(
    e: &mut Engine,
    this: Ptr<NavMeshObstacleManager>,
    obstacle: u32,
) {
    let root_slot = this.addr() + NavMeshObstacleManager::spObstacleRootNode.off;
    let data = pointer_get(e, obstacle);
    if pointer_get(e, data + ObstacleData::sp3DNode.off) != 0 {
        let root = pointer_get(e, root_slot);
        let data = pointer_get(e, obstacle);
        let old = pointer_get(e, data + ObstacleData::sp3DNode.off);
        e.vcall(root, NODE_DETACH_CHILD_SLOT, &args![old]);
    }
    e.with_stack(4, |e, node| {
        let node = node.addr();
        pointer_init(e, node, 0);
        nav_mesh_obstacle_manager_create_bounding_box3d_node(e, this, obstacle, node);
        attach_to_root(e, this, node);
        update_root(e, this);
        pointer_release(e, node);
    });
}

// ---------------------------------------------------------------------------
// Session 3 (`006c5c50`..`006c6e60`): the task hand-over to the tasklet group,
// the constructors and destructors of the manager's containers, and the
// lock-free queue. The docs say what the body of each callee does, as
// observed in the disassembly.

/// `BSTasklet` vtable (`0106c5d8`), and `BSTaskletManager`'s add of a tasklet
/// to a group (`00b00b40(manager, group, tasklet, flag)`, `AL`).
const TASKLET_VTABLE: u32 = 0x0106_c5d8;
const TASKLET_GROUP_ADD: u32 = 0x00b0_0b40;
/// `006ecd40(tasklet, data)`: stores the argument at `+4` of the tasklet
/// (the engine map names the folded body `CombatProcedureAttackMelee::Initialize`).
const TASKLET_SET_DATA: u32 = 0x006e_cd40;
/// `BSSimpleArray::Find` (`00719b20(array, &value, start, comparator)`, the
/// index or `-1`; the comparator is `009a3830`) and `Remove` (`006bf8f0(array,
/// index, count)`), used on `BackgroundTasks`.
const ARRAY_FIND: u32 = 0x0071_9b20;
const ARRAY_REMOVE: u32 = 0x006b_f8f0;
const TASK_COMPARATOR: u32 = 0x009a_3830;
/// How many times `006c5ce0` offers a task to the group before it gives up.
const TASK_HAND_OVER_TRIES: u32 = 100;
/// `CurrentNavMeshTaskMap`'s `GetAt(key, &out)` (`00853130`, `AL`) and
/// `SetAt(key, value)` (`00844700`), and the `ObstacleTaskData` constructor
/// (`006c77a0(task, navmeshinfo)`) with the size of the object (`0x58`).
const TASK_MAP_GET_AT: u32 = 0x0085_3130;
const TASK_MAP_SET_AT: u32 = 0x0084_4700;
const TASK_CONSTRUCT: u32 = 0x006c_77a0;
const TASK_SIZE: u32 = 0x58;

/// Vtables of the map types: `NiTMap<unsigned int, NiPointer<ReferenceObstacleArray>>`
/// (`0106c5e0`; its base `NiTMapBase`, `0106c654`), the rigid body map
/// (`0106c600`; base `0106c68c`) and the task map (`0106c620`; base
/// `0106c6ac`). The three bases differ only in the vtable.
const REFERENCE_MAP_VTABLE: u32 = 0x0106_c5e0;
const REFERENCE_MAP_BASE_VTABLE: u32 = 0x0106_c654;
const RIGID_BODY_MAP_VTABLE: u32 = 0x0106_c600;
const RIGID_BODY_MAP_BASE_VTABLE: u32 = 0x0106_c68c;
const TASK_MAP_VTABLE: u32 = 0x0106_c620;
const TASK_MAP_BASE_VTABLE: u32 = 0x0106_c6ac;
/// The map's bucket array: `00aa1070(bytes)` allocates, `00aa10f0(block)`
/// frees (both `cdecl`); `memset` is `00403d30(block, value, bytes)`; the base
/// destructor of the first map type is `006c63b0(this)`.
const BUCKETS_ALLOCATE: u32 = 0x00aa_1070;
const BUCKETS_FREE: u32 = 0x00aa_10f0;
const MEMSET: u32 = 0x0040_3d30;
const REFERENCE_MAP_BASE_DTOR: u32 = 0x006c_63b0;
/// Vtables of the array types: `BSSimpleArray<NiPointer<ObstacleData>, 1024>`
/// (`0106c640`), `BSSimpleArray<ObstacleTaskData *, 1024>` (`0106c6cc`), the
/// navmesh holder array (`0106c6e0`), and the cell array (`0106c6f4`; its base
/// `0106c708`).
const OBSTACLE_ARRAY_VTABLE: u32 = 0x0106_c640;
const TASK_ARRAY_VTABLE: u32 = 0x0106_c6cc;
const NAVMESH_HOLDER_ARRAY_VTABLE: u32 = 0x0106_c6e0;
const CELL_ARRAY_VTABLE: u32 = 0x0106_c6f4;
const CELL_ARRAY_BASE_VTABLE: u32 = 0x0106_c708;
/// `BSSimpleArray` helpers: the setup `00822860(array, 0, 0)` of the obstacle
/// array, the one `006b3eb0(array, size, reserve)` of the task and cell
/// arrays; the cell array's base constructor `006c7530(array)` (its base
/// vtable `0106c708`, then `006b3eb0(array, 0, 0)`);
/// the navmesh holder array's base constructor `0042f800(array)`,
/// its setup `0042fcb0(array, 0, 0)` and its `0042f9c0(array, 1)` /
/// `0042f830(array)` teardown; `00401020()` (the address `011f6238`, a
/// memory pool object) and `00aa42e0(pool)` (the allocator of an array,
/// stored at `+0x10`).
const OBSTACLE_ARRAY_INIT: u32 = 0x0082_2860;
const SIMPLE_ARRAY_INIT: u32 = 0x006b_3eb0;
const CELL_ARRAY_BASE_CTOR: u32 = 0x006c_7530;
const HOLDER_ARRAY_BASE_CTOR: u32 = 0x0042_f800;
const HOLDER_ARRAY_INIT: u32 = 0x0042_fcb0;
const HOLDER_ARRAY_CLEAR: u32 = 0x0042_f9c0;
const HOLDER_ARRAY_BASE_DTOR: u32 = 0x0042_f830;
const ARRAY_POOL: u32 = 0x0040_1020;
const ARRAY_ALLOCATOR: u32 = 0x00aa_42e0;
/// `LockFreeQueue<bhkRigidBody *>` vtable (`0106c674`) and helpers:
/// `InterfacedClass` constructor and destructor (`0044d7f0` / `0044b4b0`: only
/// the vtable), the interface of the calling thread (`00449f80(queue)`, which
/// asks the interface manager at `+0x14` through `0044d5c0(manager, queue)`),
/// the `LockFreeQueueInterface` constructor (`0044cd00(interface, queue, slot,
/// next_slot)`), the interface manager constructor (`006c73b0(manager,
/// capacity)`) and its deleting destructor (`006c71a0(manager, 1)`), the pop of
/// an interface (`006c6fe0(interface, &out)`, `AL`) and the compare-and-swap
/// `004491c0(address, new, expected)` (`AL`: the word was `expected`).
const LOCK_FREE_QUEUE_VTABLE: u32 = 0x0106_c674;
const QUEUE_BASE_CTOR: u32 = 0x0044_d7f0;
const QUEUE_BASE_DTOR: u32 = 0x0044_b4b0;
const QUEUE_THREAD_INTERFACE: u32 = 0x0044_9f80;
const QUEUE_INTERFACE_CTOR: u32 = 0x0044_cd00;
const QUEUE_INTERFACE_MANAGER_CTOR: u32 = 0x006c_73b0;
const QUEUE_INTERFACE_MANAGER_DELETE: u32 = 0x006c_71a0;
const QUEUE_INTERFACE_POP: u32 = 0x006c_6fe0;
const COMPARE_AND_SWAP: u32 = 0x0044_91c0;
/// The spin lock of a queue (`+0x20`). The scope guard of the queue code: the
/// category (`6`), the file name of the constructor, push and clear
/// (`010173f8`) and that of `AllocateInterface` (`0106649c`).
/// `IncrementCount` is virtual slot `0x08` of a queue.
const QUEUE_LOCK_OFFSET: u32 = 0x20;
const QUEUE_SCOPE_CATEGORY: u32 = 6;
const QUEUE_SOURCE_PATH: u32 = 0x0101_73f8;
const QUEUE_INTERFACE_SOURCE_PATH: u32 = 0x0106_649c;
const QUEUE_INCREMENT_COUNT_SLOT: u32 = 0x08;
/// Virtual slot `0x0c` of a queue: `DecrementCount`.
const QUEUE_DECREMENT_COUNT_SLOT: u32 = 0x0c;
/// Session 4 (`006c6fe0`..`006c77a0`): the interface of a thread. `006c75c0
/// (interface, flags)` destroys one, `006c7610(interface)` frees its batch of
/// retired nodes, `0044e360(word)` is run by the manager's destructor on its
/// word at `+4`.
const QUEUE_INTERFACE_DESTROY: u32 = 0x006c_75c0;
const QUEUE_INTERFACE_FREE_BATCH: u32 = 0x006c_7610;
const QUEUE_MANAGER_RELEASE: u32 = 0x0044_e360;
/// `ObstacleTaskData`: the vtable (`0106c7ec`), the base constructor
/// (`006c7850`), the constructors of `Operations` (`006c87c0`) and
/// `PortalSwaps` (`00699450`).
const TASK_DATA_VTABLE: u32 = 0x0106_c7ec;
const TASK_BASE_CTOR: u32 = 0x006c_7850;
const TASK_OPERATIONS_CTOR: u32 = 0x006c_87c0;
const TASK_PORTAL_SWAPS_CTOR: u32 = 0x0069_9450;

/// A scope guard around `body` with the category, file name and line the
/// lock-free queue code gives it (see [`with_scope_guard`] for the other one).
fn with_queue_guard<R>(
    e: &mut Engine,
    path: u32,
    line: u32,
    body: impl FnOnce(&mut Engine) -> R,
) -> R {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, QUEUE_SCOPE_CATEGORY, 1u32, path, line],
        );
        let result = body(e);
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
        result
    })
}

/// A queue node: `operator new(8)` and the list constructor `0096a2d0` (0
/// when the allocation fails).
fn new_queue_node(e: &mut Engine) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![8u32]).u32();
    if block != 0 {
        e.call(REFS_LIST_CTOR, &args![block]).u32()
    } else {
        0
    }
}

/// The queue's interface manager: `operator new(0x10)` and `006c73b0` (0 when
/// the allocation fails).
fn new_interface_manager(e: &mut Engine, capacity: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    if block != 0 {
        e.call(QUEUE_INTERFACE_MANAGER_CTOR, &args![block, capacity])
            .u32()
    } else {
        0
    }
}

// Translated from 006c5c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the debug box of an obstacle (engine map: unnamed). When the
/// manager has a root node and the obstacle's `sp3DNode` is set, the box is
/// detached from the root (virtual slot `0xe8`) and `sp3DNode` is cleared.
/// `obstacle` is the address of the obstacle's `NiPointer<ObstacleData>`.
pub fn fn_006c5c50(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, obstacle: u32) {
    let root_slot = this.addr() + NavMeshObstacleManager::spObstacleRootNode.off;
    if pointer_get(e, root_slot) == 0 {
        return;
    }
    let data = pointer_get(e, obstacle);
    if pointer_get(e, data + ObstacleData::sp3DNode.off) == 0 {
        return;
    }
    let root = pointer_get(e, root_slot);
    let data = pointer_get(e, obstacle);
    let node = pointer_get(e, data + ObstacleData::sp3DNode.off);
    e.vcall(root, NODE_DETACH_CHILD_SLOT, &args![node]);
    let data = pointer_get(e, obstacle);
    e.call(
        NI_POINTER_SET,
        &args![data + ObstacleData::sp3DNode.off, 0u32],
    );
}

// Translated from 006c5ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands an `ObstacleTaskData` to the tasklet group (engine map: unnamed):
/// the task is added to `BackgroundTasks` under `ProcessedTaskLock`, a
/// `BSTasklet` holding it is offered to `ObstacleTaskletGroup` up to 100
/// times (a `Sleep(0)` between the tries) and, when the group never takes it,
/// the task leaves `BackgroundTasks` again. True when the group took it.
pub fn fn_006c5ce0(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, task: u32) -> bool {
    let manager = this.addr();
    let lock = manager + NavMeshObstacleManager::ProcessedTaskLock.off;
    let tasks = manager + NavMeshObstacleManager::BackgroundTasks.off;
    let group = manager + NavMeshObstacleManager::ObstacleTaskletGroup.off;
    // The 8-byte tasklet, then the stack word holding the task argument.
    e.with_stack(12, |e, frame| {
        let tasklet = frame.addr();
        let task_slot = tasklet + 8;
        e.mem.set_u32(task_slot, task);
        fn_006c5e40(e, Ptr::new(tasklet));
        e.call(TASKLET_SET_DATA, &args![tasklet, task]);
        e.call(SPIN_LOCK_LOCK, &args![lock, 0u32]);
        e.call(ARRAY_ADD, &args![tasks, task_slot]);
        e.call(SPIN_LOCK_UNLOCK, &args![lock]);
        let mut tries = 0;
        loop {
            if fn_006c5ec0(e, Ptr::new(group), tasklet, 0) {
                fn_006c5e70(e, Ptr::new(tasklet));
                return true;
            }
            tries += 1;
            if tries >= TASK_HAND_OVER_TRIES {
                e.call(SPIN_LOCK_LOCK, &args![lock, 0u32]);
                let index = e
                    .call(ARRAY_FIND, &args![tasks, task_slot, 0u32, TASK_COMPARATOR])
                    .i32();
                if index != -1 {
                    e.call(ARRAY_REMOVE, &args![tasks, index, 1u32]);
                }
                e.call(SPIN_LOCK_UNLOCK, &args![lock]);
                fn_006c5e70(e, Ptr::new(tasklet));
                return false;
            }
            e.call(SLEEP, &args![0u32]);
        }
    })
}

// Translated from 006c5e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTasklet` constructor (engine map: unnamed; the vtable `0106c5d8` is the
/// one whose deleting destructor the map names `BSTasklet`): sets the vtable
/// and clears `pData`. Returns `this`.
pub fn fn_006c5e40(e: &mut Engine, this: Ptr<BSTasklet>) -> Ptr<BSTasklet> {
    e.mem.set_u32(this.addr(), TASKLET_VTABLE);
    e.set(this, BSTasklet::pData, 0);
    this
}

// Translated from 006c5e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTasklet` destructor body (engine map: unnamed): puts the class's own
/// vtable back.
pub fn fn_006c5e70(e: &mut Engine, this: Ptr<BSTasklet>) {
    e.mem.set_u32(this.addr(), TASKLET_VTABLE);
}

// Translated from 006c5e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTasklet::_scalar_deleting_destructor_` (Xbox PDB): the destructor, then
/// `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn bs_tasklet_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSTasklet>,
    flags: u32,
) -> Ptr<BSTasklet> {
    fn_006c5e70(e, this);
    delete_if_requested(e, this, flags);
    this
}

// Translated from 006c5ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTaskletGroup` add (engine map: unnamed): offers a tasklet to the group
/// through the tasklet manager (`00b00b40`); true when it was taken.
pub fn fn_006c5ec0(e: &mut Engine, this: Ptr, tasklet: u32, flag: u8) -> bool {
    let manager = e.call(TASKLET_MANAGER_INSTANCE, &[]).u32();
    e.call(
        TASKLET_GROUP_ADD,
        &args![manager, this, tasklet, u32::from(flag)],
    )
    .bool()
}

// Translated from 006c5ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ObstacleTaskData` of a navmesh (engine map: unnamed): looked up in
/// `CurrentNavMeshTaskMap` by the first word of the `NavMeshInfo`; when
/// missing, a task of `0x58` bytes is built (`006c77a0`) and entered into the
/// map. `nav_mesh_info` is the address of the info's `NiPointer`.
pub fn fn_006c5ef0(e: &mut Engine, this: Ptr<NavMeshObstacleManager>, nav_mesh_info: u32) -> u32 {
    let map = this.addr() + NavMeshObstacleManager::CurrentNavMeshTaskMap.off;
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let key = pointer_get(e, nav_mesh_info);
        if !e.call(TASK_MAP_GET_AT, &args![map, key, slot]).bool() {
            let block = e.call(OPERATOR_NEW, &args![TASK_SIZE]).u32();
            let task = if block != 0 {
                e.call(TASK_CONSTRUCT, &args![block, nav_mesh_info]).u32()
            } else {
                0
            };
            e.mem.set_u32(slot, task);
            let key = pointer_get(e, nav_mesh_info);
            e.call(TASK_MAP_SET_AT, &args![map, key, task]);
        }
        word(e, slot)
    })
}

// Translated from 006c5fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `NiTMap<unsigned int, NiPointer<ReferenceObstacleArray>>`
/// (engine map: unnamed): the base constructor `006c6260`, then its own
/// vtable. Returns `this`.
pub fn fn_006c5fc0(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    fn_006c6260(e, this, buckets);
    e.mem.set_u32(this.addr(), REFERENCE_MAP_VTABLE);
    this
}

// Translated from 006c5ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeQueue<bhkRigidBody *>::Push` (engine map: unnamed): under the
/// queue's spin lock, pushes through the calling thread's interface
/// (`00449f80`, then [`fn_006c6e60`]). `value` is the address of the word to
/// push.
pub fn fn_006c5ff0(e: &mut Engine, this: Ptr<LockFreeQueue>, value: u32) {
    let lock = this.addr() + QUEUE_LOCK_OFFSET;
    e.call(SPIN_LOCK_LOCK, &args![lock, 0u32]);
    let interface = e.call(QUEUE_THREAD_INTERFACE, &args![this]).u32();
    fn_006c6e60(e, Ptr::new(interface), value);
    e.call(SPIN_LOCK_UNLOCK, &args![lock]);
}

// Translated from 006c6030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeQueue<bhkRigidBody *>::Pop` (engine map: unnamed): under the
/// queue's spin lock, pops through the calling thread's interface (`00449f80`,
/// then [`fn_006c6fe0`]) into the word at `out`. True when a
/// value was popped.
pub fn fn_006c6030(e: &mut Engine, this: Ptr<LockFreeQueue>, out: u32) -> bool {
    let lock = this.addr() + QUEUE_LOCK_OFFSET;
    e.call(SPIN_LOCK_LOCK, &args![lock, 0u32]);
    let interface = e.call(QUEUE_THREAD_INTERFACE, &args![this]).u32();
    let popped = e.call(QUEUE_INTERFACE_POP, &args![interface, out]).bool();
    e.call(SPIN_LOCK_UNLOCK, &args![lock]);
    popped
}

// Translated from 006c6070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `NiTMap<bhkRigidBody *, NiPointer<ObstacleData>>`
/// (engine map: unnamed): the base constructor `006c68b0`, then its own
/// vtable. Returns `this`.
pub fn fn_006c6070(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    fn_006c68b0(e, this, buckets);
    e.mem.set_u32(this.addr(), RIGID_BODY_MAP_VTABLE);
    this
}

// Translated from 006c60a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `NiTMap<unsigned int, ObstacleTaskData *>` (engine map:
/// unnamed): the base constructor `006c6b10`, then its own vtable. Returns
/// `this`.
pub fn fn_006c60a0(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    fn_006c6b10(e, this, buckets);
    e.mem.set_u32(this.addr(), TASK_MAP_VTABLE);
    this
}

/// `operator delete(this)` when bit 0 of `flags` is set; the tail of every
/// deleting destructor.
fn delete_if_requested<T>(e: &mut Engine, this: Ptr<T>, flags: u32) {
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
}

// Translated from 006c60d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned_int_NiPointer<ReferenceObstacleArray>_>::_scalar_deleting_destructor_`
/// (Xbox PDB): the destructor `006c6350`, then `operator delete` when bit 0
/// of `flags` is set. Returns `this`.
pub fn ni_tmap_reference_obstacle_array_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_006c6350(e, this);
    delete_if_requested(e, this, flags);
    this
}

// Translated from 006c6100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<bhkRigidBody_P_NiPointer<ObstacleData>_>::_scalar_deleting_destructor_`
/// (Xbox PDB): the destructor `006c6a80`, then `operator delete` when bit 0
/// of `flags` is set. Returns `this`.
pub fn ni_tmap_rigid_body_obstacle_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_006c6a80(e, this);
    delete_if_requested(e, this, flags);
    this
}

// Translated from 006c6130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<unsigned_int_ObstacleTaskData_P>::_scalar_deleting_destructor_`
/// (Xbox PDB): the destructor `006c6b80`, then `operator delete` when bit 0
/// of `flags` is set. Returns `this`.
pub fn ni_tmap_obstacle_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_006c6b80(e, this);
    delete_if_requested(e, this, flags);
    this
}

// Translated from 006c6160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray<NiPointer<ObstacleData>, 1024>` (engine
/// map: unnamed): sets the vtable and runs `00822860(array, 0, 0)`. Returns
/// `this`.
pub fn fn_006c6160(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), OBSTACLE_ARRAY_VTABLE);
    e.call(OBSTACLE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006c6190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPointer<ObstacleData>_1024>::~BSSimpleArray<NiPointer<ObstacleData>_1024>`
/// (Xbox PDB): sets the vtable and empties the array (`006c6200(array, 1)`).
pub fn bs_simple_array_obstacle_data_destructor(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), OBSTACLE_ARRAY_VTABLE);
    e.call(OBSTACLE_ARRAY_EMPTY, &args![this, 1u32]);
}

/// The body shared by the three `NiTMapBase` constructors, which differ only
/// in the vtable they set: the bucket count, no items and a zeroed bucket
/// array of `4 * buckets` bytes.
fn construct_map_base(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32, vtable: u32) -> Ptr<NiTMap> {
    e.mem.set_u32(this.addr(), vtable);
    e.set(this, NiTMap::m_uiHashSize, buckets);
    e.set(this, NiTMap::m_uiCount, 0);
    let bytes = buckets << 2;
    let table = e.call(BUCKETS_ALLOCATE, &args![bytes]).u32();
    e.set(this, NiTMap::m_ppkHashTable, table);
    let bytes = e.get(this, NiTMap::m_uiHashSize) << 2;
    e.call(MEMSET, &args![table, 0u32, bytes]);
    this
}

// Translated from 006c6260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` constructor of the reference obstacle map (engine map:
/// unnamed): vtable `0106c654`, the bucket count, no items, and a zeroed
/// bucket array of `4 * buckets` bytes. Returns `this`.
pub fn fn_006c6260(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    construct_map_base(e, this, buckets, REFERENCE_MAP_BASE_VTABLE)
}

// Translated from 006c6350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the reference obstacle map (engine map: unnamed): sets the
/// vtable, clears the map (`00438af0`) and runs the base destructor
/// `006c63b0`.
pub fn fn_006c6350(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), REFERENCE_MAP_VTABLE);
    e.call(MAP_CLEAR, &args![this]);
    e.call(REFERENCE_MAP_BASE_DTOR, &args![this]);
}

// Translated from 006c6400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleList<NiPointer<ObstacleData>>::Contains` (engine map: unnamed):
/// walks the nodes while `00631820` says the node's item differs from the
/// value at `value`; true when a node was found.
pub fn fn_006c6400(e: &mut Engine, this: Ptr<BSSimpleList>, value: u32) -> bool {
    let mut node = this.addr();
    while node != 0 {
        if !e.call(LIST_NODE_DIFFERS, &args![node, value]).bool() {
            break;
        }
        node = word(e, node + BSSimpleList::m_pkNext.off);
    }
    node != 0
}

// Translated from 006c6450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeQueue<bhkRigidBody *>` constructor (engine map: unnamed). The
/// `InterfacedClass` base (`0044d7f0`), the vtable `0106c674`, the spin lock
/// at `+0x20` (`0096a2d0`), a list node as head and tail, `iDeleteBatchSize`,
/// the node pointer array (`2 * capacity` words, saturating at 4 GiB) and the
/// interface manager (`006c73b0`). The source line given to the scope guard is
/// `0xfd`. Returns `this`.
pub fn fn_006c6450(
    e: &mut Engine,
    this: Ptr<LockFreeQueue>,
    capacity: u32,
    delete_batch_size: u32,
) -> Ptr<LockFreeQueue> {
    e.call(QUEUE_BASE_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), LOCK_FREE_QUEUE_VTABLE);
    e.call(REFS_LIST_CTOR, &args![this.addr() + QUEUE_LOCK_OFFSET]);
    with_queue_guard(e, QUEUE_SOURCE_PATH, 0xfd, |e| {
        e.set(this, LockFreeQueue::iCount, 0);
        let node = new_queue_node(e);
        e.set(this, LockFreeQueue::pHead, node);
        let head = e.get(this, LockFreeQueue::pHead);
        e.set(this, LockFreeQueue::pTail, head);
        e.set(this, LockFreeQueue::iDeleteBatchSize, delete_batch_size);
        let bytes = u64::from(capacity << 1) * 4;
        let bytes = u32::try_from(bytes).unwrap_or(u32::MAX);
        let nodes = e.call(OPERATOR_NEW, &args![bytes]).u32();
        e.set(this, LockFreeQueue::pReferencedNodes, nodes);
        let manager = new_interface_manager(e, capacity);
        e.set(this, LockFreeQueue::pInterfaceManager, manager);
    });
    this
}

// Translated from 006c65b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeQueue<bhkRigidBody_P>::AllocateInterface` (Xbox PDB): builds the
/// `0x14`-byte `LockFreeQueueInterface` (`0044cd00`) of the thread numbered
/// `index`, over its pair of slots in `pReferencedNodes` (`+ 8 * index`). The
/// source line given to the scope guard is `0x95`. Returns the interface (0
/// when the allocation fails).
pub fn lock_free_queue_rigid_body_allocate_interface(
    e: &mut Engine,
    this: Ptr<LockFreeQueue>,
    index: u32,
) -> u32 {
    with_queue_guard(e, QUEUE_INTERFACE_SOURCE_PATH, 0x95, |e| {
        let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
        if block == 0 {
            return 0;
        }
        let slots = e
            .get(this, LockFreeQueue::pReferencedNodes)
            .wrapping_add(index.wrapping_shl(1).wrapping_mul(4));
        e.call(
            QUEUE_INTERFACE_CTOR,
            &args![block, this, slots, slots.wrapping_add(4)],
        )
        .u32()
    })
}

// Translated from 006c6680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeQueue<bhkRigidBody *>` destructor (engine map: unnamed): sets the
/// vtable, clears the queue completely ([`fn_006c6700`] with the flag set),
/// frees the node pointer array and runs the `InterfacedClass` destructor.
pub fn fn_006c6680(e: &mut Engine, this: Ptr<LockFreeQueue>) {
    e.mem.set_u32(this.addr(), LOCK_FREE_QUEUE_VTABLE);
    fn_006c6700(e, this, 1);
    let nodes = e.get(this, LockFreeQueue::pReferencedNodes);
    e.call(OPERATOR_DELETE, &args![nodes]);
    e.call(QUEUE_BASE_DTOR, &args![this]);
}

// Translated from 006c6700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clear of the `LockFreeQueue<bhkRigidBody *>` (engine map: unnamed), under
/// the queue's spin lock: the first word of the interface manager is read
/// (its capacity), the manager is destroyed (`006c71a0`), `pInterfaceManager`
/// and `iCount` cleared and every node freed. Unless `release_only` is set, a
/// fresh node becomes head and tail and a new interface manager is built with
/// the old capacity. The source line given to the scope guard is `0x115`.
pub fn fn_006c6700(e: &mut Engine, this: Ptr<LockFreeQueue>, release_only: u8) {
    let lock = this.addr() + QUEUE_LOCK_OFFSET;
    e.call(SPIN_LOCK_LOCK, &args![lock, 0u32]);
    with_queue_guard(e, QUEUE_SOURCE_PATH, 0x115, |e| {
        let old_manager = e.get(this, LockFreeQueue::pInterfaceManager);
        let capacity = pointer_get(e, old_manager);
        if old_manager != 0 {
            e.call(QUEUE_INTERFACE_MANAGER_DELETE, &args![old_manager, 1u32]);
        }
        e.set(this, LockFreeQueue::pInterfaceManager, 0);
        e.set(this, LockFreeQueue::iCount, 0);
        while e.get(this, LockFreeQueue::pHead) != 0 {
            let head = e.get(this, LockFreeQueue::pHead);
            let next = word(e, head);
            e.mem.set_u32(head + 4, 0);
            e.call(OPERATOR_DELETE, &args![head]);
            e.set(this, LockFreeQueue::pHead, next);
        }
        if release_only == 0 {
            let node = new_queue_node(e);
            e.set(this, LockFreeQueue::pHead, node);
            let head = e.get(this, LockFreeQueue::pHead);
            e.set(this, LockFreeQueue::pTail, head);
            let manager = new_interface_manager(e, capacity);
            e.set(this, LockFreeQueue::pInterfaceManager, manager);
        }
        e.call(SPIN_LOCK_UNLOCK, &args![lock]);
    });
}

// Translated from 006c68b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` constructor of the rigid body map (engine map: unnamed): as
/// [`fn_006c6260`] with the vtable `0106c68c`. Returns `this`.
pub fn fn_006c68b0(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    construct_map_base(e, this, buckets, RIGID_BODY_MAP_BASE_VTABLE)
}

// Translated from 006c6920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..>::SetAt` of the reference obstacle map (engine map:
/// unnamed). `value` is an `NiPointer` passed by value, which the function
/// destroys. The bucket of `key` is found through virtual slot `0x04`; the
/// chain is searched with virtual slot `0x08` (key comparison). A matching
/// item gets the value assigned (`006e5cc0`) and the function ends. Otherwise
/// a new item is made (slot `0x14`), given the key and a fresh copy of the
/// value (slot `0x0c`, which takes the copy by value), linked at the head of
/// the bucket and counted.
pub fn fn_006c6920(e: &mut Engine, this: Ptr<NiTMap>, key: u32, value: u32) {
    let map = this.addr();
    e.with_stack(4, |e, value_slot| {
        let value_slot = value_slot.addr();
        e.mem.set_u32(value_slot, value);
        let bucket = e.vcall(map, 0x04, &args![key]).u32();
        let table = e.get(this, NiTMap::m_ppkHashTable);
        let bucket_address = table.wrapping_add(bucket.wrapping_mul(4));
        let mut item = word(e, bucket_address);
        while item != 0 {
            let item_key = word(e, item + 4);
            if e.vcall(map, 0x08, &args![key, item_key]).bool() {
                e.call(NI_POINTER_ASSIGN, &args![item + 8, value_slot]);
                pointer_release(e, value_slot);
                return;
            }
            item = word(e, item);
        }
        let item = e.vcall(map, 0x14, &args![]).u32();
        e.with_stack(4, |e, copy| {
            let copy = copy.addr();
            pointer_copy(e, copy, value_slot);
            let copied = word(e, copy);
            e.vcall(map, 0x0c, &args![item, key, copied]);
        });
        let table = e.get(this, NiTMap::m_ppkHashTable);
        let bucket_address = table.wrapping_add(bucket.wrapping_mul(4));
        let first = word(e, bucket_address);
        e.mem.set_u32(item, first);
        e.mem.set_u32(bucket_address, item);
        let count = e.get(this, NiTMap::m_uiCount);
        e.set(this, NiTMap::m_uiCount, count.wrapping_add(1));
        pointer_release(e, value_slot);
    });
}

// Translated from 006c6a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase::GetHashValue` of the unsigned-int-keyed maps (engine map:
/// unnamed): `key % bucket count`.
pub fn fn_006c6a60(e: &mut Engine, this: Ptr<NiTMap>, key: u32) -> u32 {
    key % e.get(this, NiTMap::m_uiHashSize)
}

/// The base destructor body shared by [`fn_006c6ae0`] and [`fn_006c6be0`]: the
/// vtable, the clear and the bucket array freed.
fn destruct_map_base(e: &mut Engine, this: Ptr<NiTMap>, vtable: u32) {
    e.mem.set_u32(this.addr(), vtable);
    e.call(MAP_CLEAR, &args![this]);
    let table = e.get(this, NiTMap::m_ppkHashTable);
    e.call(BUCKETS_FREE, &args![table]);
}

// Translated from 006c6a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the rigid body map (engine map: unnamed): sets the vtable,
/// clears the map and runs the base destructor [`fn_006c6ae0`].
pub fn fn_006c6a80(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), RIGID_BODY_MAP_VTABLE);
    e.call(MAP_CLEAR, &args![this]);
    fn_006c6ae0(e, this);
}

// Translated from 006c6ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` destructor of the rigid body map (engine map: unnamed): the
/// vtable `0106c68c`, the clear and the bucket array freed.
pub fn fn_006c6ae0(e: &mut Engine, this: Ptr<NiTMap>) {
    destruct_map_base(e, this, RIGID_BODY_MAP_BASE_VTABLE);
}

// Translated from 006c6b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` constructor of the task map (engine map: unnamed): as
/// [`fn_006c6260`] with the vtable `0106c6ac`. Returns `this`.
pub fn fn_006c6b10(e: &mut Engine, this: Ptr<NiTMap>, buckets: u32) -> Ptr<NiTMap> {
    construct_map_base(e, this, buckets, TASK_MAP_BASE_VTABLE)
}

// Translated from 006c6b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the task map (engine map: unnamed): sets the vtable, clears
/// the map and runs the base destructor [`fn_006c6be0`].
pub fn fn_006c6b80(e: &mut Engine, this: Ptr<NiTMap>) {
    e.mem.set_u32(this.addr(), TASK_MAP_VTABLE);
    e.call(MAP_CLEAR, &args![this]);
    fn_006c6be0(e, this);
}

// Translated from 006c6be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` destructor of the task map (engine map: unnamed): the vtable
/// `0106c6ac`, the clear and the bucket array freed.
pub fn fn_006c6be0(e: &mut Engine, this: Ptr<NiTMap>) {
    destruct_map_base(e, this, TASK_MAP_BASE_VTABLE);
}

// Translated from 006c6c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray<ObstacleTaskData *, 1024>` (engine map:
/// unnamed): sets the vtable and runs `006b3eb0(array, 0, 0)`. Returns `this`.
pub fn fn_006c6c10(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), TASK_ARRAY_VTABLE);
    e.call(SIMPLE_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006c6c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray<ObstacleTaskData *, 1024>` (engine map:
/// unnamed): sets the vtable and clears the array (`008454f0(array, 1)`).
pub fn fn_006c6c40(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), TASK_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 006c6c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the navmesh holder array (`BSScrapArray<NavMeshPtr, 1024>`,
/// engine map: unnamed): the base constructor `0042f800`, the vtable
/// `0106c6e0`, the allocator of the pool at `011f6238` stored at `+0x10`
/// (`00aa42e0`) and the setup `0042fcb0(array, 0, 0)`. Returns `this`.
pub fn fn_006c6c60(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.call(HOLDER_ARRAY_BASE_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), NAVMESH_HOLDER_ARRAY_VTABLE);
    let pool = e.call(ARRAY_POOL, &[]).u32();
    let allocator = e.call(ARRAY_ALLOCATOR, &args![pool]).u32();
    e.mem.set_u32(this.addr() + 0x10, allocator);
    e.call(HOLDER_ARRAY_INIT, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006c6ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the navmesh holder array (engine map: unnamed): sets the
/// vtable, clears it (`0042f9c0(array, 1)`) and runs the base destructor
/// `0042f830`.
pub fn fn_006c6ce0(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), NAVMESH_HOLDER_ARRAY_VTABLE);
    e.call(HOLDER_ARRAY_CLEAR, &args![this, 1u32]);
    e.call(HOLDER_ARRAY_BASE_DTOR, &args![this]);
}

// Translated from 006c6d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the cell array (`BSSimpleArray<TESObjectCELL *, 1024>`,
/// engine map: unnamed): the base constructor `006c7530`, the vtable
/// `0106c6f4`, `+0x10` set to `allocator` or, when that is 0, to the pool's
/// allocator (`00aa42e0` on `00401020()`), then `006b3eb0(array, size,
/// reserve)`. Returns `this`.
pub fn fn_006c6d40(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    size: u32,
    reserve: u32,
    allocator: u32,
) -> Ptr<BSSimpleArray> {
    e.call(CELL_ARRAY_BASE_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), CELL_ARRAY_VTABLE);
    let allocator = if allocator != 0 {
        allocator
    } else {
        let pool = e.call(ARRAY_POOL, &[]).u32();
        e.call(ARRAY_ALLOCATOR, &args![pool]).u32()
    };
    e.mem.set_u32(this.addr() + 0x10, allocator);
    e.call(SIMPLE_ARRAY_INIT, &args![this, size, reserve]);
    this
}

// Translated from 006c6de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base destructor of the cell array (engine map: unnamed): the base vtable
/// `0106c708` and the array cleared (`008454f0(array, 1)`).
pub fn fn_006c6de0(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), CELL_ARRAY_BASE_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
}

// Translated from 006c6e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the cell array (engine map: unnamed): the vtable
/// `0106c6f4`, the array cleared and the base destructor [`fn_006c6de0`].
pub fn fn_006c6e00(e: &mut Engine, this: Ptr<BSSimpleArray>) {
    e.mem.set_u32(this.addr(), CELL_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR, &args![this, 1u32]);
    fn_006c6de0(e, this);
}

// Translated from 006c6e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeQueue<bhkRigidBody *>::LockFreeQueueInterface` push (engine map:
/// unnamed): appends a node holding the word at `value`. The tail is read and
/// published in this interface's first protected slot, re-read to check it did
/// not move, and its `pNext` examined: a tail that already has a successor is
/// moved on with a compare-and-swap (`004491c0`) and the loop starts again;
/// otherwise the node is linked after the tail with a compare-and-swap and
/// the queue's virtual slot `0x08` (`IncrementCount`) runs. The tail is then
/// swung to the new node and the protected slot cleared. The source line
/// given to the scope guard is `0x76`.
pub fn fn_006c6e60(e: &mut Engine, this: Ptr<LockFreeQueueInterface>, value: u32) {
    with_queue_guard(e, QUEUE_SOURCE_PATH, 0x76, |e| {
        let queue = e.get(this, LockFreeQueueInterface::pOwner);
        let tail_slot = queue + LockFreeQueue::pTail.off;
        let node = new_queue_node(e);
        let item = word(e, value);
        e.mem.set_u32(node + 4, item);
        let mut tail;
        loop {
            tail = word(e, tail_slot);
            let protected_slot = e.get(this, LockFreeQueueInterface::pReferencedNodes);
            e.mem.set_u32(protected_slot, tail);
            if tail != word(e, tail_slot) {
                continue;
            }
            let next = word(e, tail);
            if tail != word(e, tail_slot) {
                continue;
            }
            if next != 0 {
                e.call(COMPARE_AND_SWAP, &args![tail_slot, next, tail]);
                continue;
            }
            if e.call(COMPARE_AND_SWAP, &args![tail, node, 0u32]).bool() {
                e.vcall(queue, QUEUE_INCREMENT_COUNT_SLOT, &args![]);
                break;
            }
        }
        e.call(COMPARE_AND_SWAP, &args![tail_slot, node, tail]);
        let protected_slot = e.get(this, LockFreeQueueInterface::pReferencedNodes);
        e.mem.set_u32(protected_slot, 0);
    });
}

// Translated from 006c6fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeQueue<bhkRigidBody *>::LockFreeQueueInterface` pop (engine map:
/// unnamed): stores the head node in this interface's first protected slot and
/// its successor in the second, re-reading the head to check it did not move.
/// An empty queue (no successor) clears the first slot, stores 0 at `out` and
/// answers false. A head that equals the tail has the tail moved on with a
/// compare-and-swap (`004491c0`) and the loop starts again. Otherwise the
/// successor's value (`+4`) is copied to `out` and the head swung to the
/// successor with a compare-and-swap; when that succeeds the queue's virtual
/// slot `0x0c` (`DecrementCount`) runs, the successor's value is cleared, both
/// slots are cleared and the old head is handed to [`fn_006c7560`]; true.
pub fn fn_006c6fe0(e: &mut Engine, this: Ptr<LockFreeQueueInterface>, out: u32) -> bool {
    let queue = e.get(this, LockFreeQueueInterface::pOwner);
    let head_slot = queue + LockFreeQueue::pHead.off;
    let tail_slot = queue + LockFreeQueue::pTail.off;
    let first_slot = e.get(this, LockFreeQueueInterface::pReferencedNodes);
    let second_slot = e.get(this, LockFreeQueueInterface::pReferencedNodesSecond);
    let head = loop {
        let head = word(e, head_slot);
        e.mem.set_u32(first_slot, head);
        if head != word(e, head_slot) {
            continue;
        }
        let tail = word(e, tail_slot);
        let next = word(e, head);
        e.mem.set_u32(second_slot, next);
        if head != word(e, head_slot) {
            continue;
        }
        if next == 0 {
            e.mem.set_u32(first_slot, 0);
            e.mem.set_u32(out, 0);
            return false;
        }
        if head == tail {
            e.call(COMPARE_AND_SWAP, &args![tail_slot, next, tail]);
            continue;
        }
        let value = word(e, next + 4);
        e.mem.set_u32(out, value);
        if e.call(COMPARE_AND_SWAP, &args![head_slot, next, head])
            .bool()
        {
            e.vcall(queue, QUEUE_DECREMENT_COUNT_SLOT, &args![]);
            e.mem.set_u32(next + 4, 0);
            break head;
        }
    };
    e.mem.set_u32(first_slot, 0);
    e.mem.set_u32(second_slot, 0);
    fn_006c7560(e, this, head);
    true
}

// Translated from 006c7110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiPointer<ObstacleData>_1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): the destructor `006c6190`, then `operator delete` when bit 0
/// of `flags` is set. Returns `this`.
pub fn bs_simple_array_obstacle_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    bs_simple_array_obstacle_data_destructor(e, this);
    delete_if_requested(e, this, flags);
    this
}

// Translated from 006c7140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned_int_NiPointer<ReferenceObstacleArray>_>_>_unsigned_int_NiPointer<ReferenceObstacleArray>_>::_scalar_deleting_destructor_`
/// (Xbox PDB): the base destructor `006c63b0`, then `operator delete` when
/// bit 0 of `flags` is set. Returns `this`.
pub fn ni_tmap_base_reference_obstacle_array_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    e.call(REFERENCE_MAP_BASE_DTOR, &args![this]);
    delete_if_requested(e, this, flags);
    this
}

// Translated from 006c7170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeQueue<bhkRigidBody_P>::_scalar_deleting_destructor_` (Xbox PDB):
/// the destructor `006c6680`, then `operator delete` when bit 0 of `flags` is
/// set. Returns `this`.
pub fn lock_free_queue_rigid_body_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<LockFreeQueue>,
    flags: u32,
) -> Ptr<LockFreeQueue> {
    fn_006c6680(e, this);
    delete_if_requested(e, this, flags);
    this
}

// Translated from 006c71a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of the queue's interface manager (engine map:
/// unnamed; the object `006c73b0` builds): the destructor [`fn_006c74a0`],
/// then `operator delete` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_006c71a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006c74a0(e, this);
    delete_if_requested(e, this, flags);
    this
}

// Translated from 006c71d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<bhkRigidBody_P_NiPointer<ObstacleData>_>_>_bhkRigidBody_P_NiPointer<ObstacleData>_>::_scalar_deleting_destructor_`
/// (Xbox PDB): the base destructor `006c6ae0`, then `operator delete` when
/// bit 0 of `flags` is set. Returns `this`.
pub fn ni_tmap_base_rigid_body_obstacle_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_006c6ae0(e, this);
    delete_if_requested(e, this, flags);
    this
}

// Translated from 006c7200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<unsigned_int_ObstacleTaskData_P>_>_unsigned_int_ObstacleTaskData_P>::_scalar_deleting_destructor_`
/// (Xbox PDB): the base destructor `006c6be0`, then `operator delete` when
/// bit 0 of `flags` is set. Returns `this`.
pub fn ni_tmap_base_obstacle_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiTMap>,
    flags: u32,
) -> Ptr<NiTMap> {
    fn_006c6be0(e, this);
    delete_if_requested(e, this, flags);
    this
}

// Translated from 006c7230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<ObstacleTaskData_P_1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): the destructor `006c6c40`, then `operator delete` when bit 0
/// of `flags` is set. Returns `this`.
pub fn bs_simple_array_obstacle_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_006c6c40(e, this);
    delete_if_requested(e, this, flags);
    this
}

// Translated from 006c7260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<NavMeshPtr_1024>::_scalar_deleting_destructor_` (Xbox PDB):
/// the destructor `006c6ce0`, then `operator delete` when bit 0 of `flags` is
/// set. Returns `this`.
pub fn bs_scrap_array_nav_mesh_ptr_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_006c6ce0(e, this);
    delete_if_requested(e, this, flags);
    this
}

// Translated from 006c7290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<TESObjectCELL_P_1024>::_scalar_deleting_destructor_` (Xbox
/// PDB): the base destructor `006c6de0`, then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn bs_simple_array_tes_object_cell_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_006c6de0(e, this);
    delete_if_requested(e, this, flags);
    this
}

// Translated from 006c72c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSScrapArray<TESObjectCELL_P_1024>::_scalar_deleting_destructor_` (Xbox
/// PDB): the destructor `006c6e00`, then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn bs_scrap_array_tes_object_cell_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    flags: u32,
) -> Ptr<BSSimpleArray> {
    fn_006c6e00(e, this);
    delete_if_requested(e, this, flags);
    this
}

// Translated from 006c74a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the queue's interface manager (engine map: unnamed; a
/// `fastcall` on the object, which `006c73b0` builds): its first word is the
/// number of interfaces; the interface array at `+8` holds eight-byte entries
/// whose second word is an interface, and each interface that exists is
/// destroyed with `006c75c0(interface, 1)`. Then the array is freed with
/// `operator delete` and `0044e360` runs on the word at `+4`.
pub fn fn_006c74a0(e: &mut Engine, this: Ptr) {
    let mut index = 0;
    while index < e.call(NI_POINTER_GET, &args![this]).u32() {
        // Entry `index` of the array at +8; its second word.
        let entries = word(e, this.addr() + 8);
        let interface = word(e, entries + index * 8 + 4);
        if interface != 0 {
            e.call(QUEUE_INTERFACE_DESTROY, &args![interface, 1u32]);
        }
        index += 1;
    }
    let entries = word(e, this.addr() + 8);
    e.call(OPERATOR_DELETE, &args![entries]);
    let word_at_4 = word(e, this.addr() + 4);
    e.call(QUEUE_MANAGER_RELEASE, &args![word_at_4]);
}

// Translated from 006c7560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LockFreeQueue<bhkRigidBody *>::LockFreeQueueInterface` retirement of a
/// popped node (engine map: unnamed): the node's value is cleared, the node
/// is chained in front of the interface's delete list (`006ecd40(node,
/// previous head)` stores the previous head at the node's `+4`), the count of
/// retired nodes goes up, and when it reaches the queue's delete batch size
/// (`0084e3a0` on the owner) `006c7610` frees the batch.
pub fn fn_006c7560(e: &mut Engine, this: Ptr<LockFreeQueueInterface>, node: u32) {
    e.mem.set_u32(node + 4, 0);
    let previous = e.get(this, LockFreeQueueInterface::pDeleteHead);
    e.call(TASKLET_SET_DATA, &args![node, previous]);
    e.set(this, LockFreeQueueInterface::pDeleteHead, node);
    let count = e
        .get(this, LockFreeQueueInterface::iDeleteCount)
        .wrapping_add(1);
    e.set(this, LockFreeQueueInterface::iDeleteCount, count);
    let owner = e.get(this, LockFreeQueueInterface::pOwner);
    let batch = e.call(FORM_ID, &args![owner]).u32();
    if count == batch {
        e.call(QUEUE_INTERFACE_FREE_BATCH, &args![this]);
    }
}

// Translated from 006c77a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ObstacleTaskData::ObstacleTaskData` (engine map: unnamed): the base
/// constructor `006c7850`, the vtable `0106c7ec`, the two navmesh holders
/// `spSrcMesh` (`+0x1c`) and `spNewNavMesh` (`+0x20`) with `0042fb00`, the
/// `Operations` array (`+0x24`, `006c87c0`) and `PortalSwaps` (`+0x34`,
/// `00699450`), then `pInfo` (`+0x18`) is the argument and `bPortalModified`
/// (`+0x54`) is false. Returns `this`. The exception-unwinding frame is not
/// translated.
pub fn fn_006c77a0(
    e: &mut Engine,
    this: Ptr<ObstacleTaskData>,
    nav_mesh_info: u32,
) -> Ptr<ObstacleTaskData> {
    e.call(TASK_BASE_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), TASK_DATA_VTABLE);
    let source = this.addr() + ObstacleTaskData::spSrcMesh.off;
    e.call(NAVMESH_HOLDER_CTOR, &args![source]);
    let new_mesh = this.addr() + ObstacleTaskData::spNewNavMesh.off;
    e.call(NAVMESH_HOLDER_CTOR, &args![new_mesh]);
    let operations = this.addr() + ObstacleTaskData::Operations.off;
    e.call(TASK_OPERATIONS_CTOR, &args![operations]);
    let swaps = this.addr() + ObstacleTaskData::PortalSwaps.off;
    e.call(TASK_PORTAL_SWAPS_CTOR, &args![swaps]);
    e.set(this, ObstacleTaskData::pInfo, nav_mesh_info);
    e.set(this, ObstacleTaskData::bPortalModified, false);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00559120,
            ni_tmap_base_reference_obstacle_array_get_next(Ptr<NiTMap>, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00631620,
            bs_simple_list_obstacle_data_remove(Ptr<BSSimpleList>, Ptr)
        ),
        entry!(
            0x00691a30,
            ni_tmap_reference_obstacle_array_delete_item(Ptr<NiTMap>, Ptr<NiTMapItem>)
        ),
        entry!(
            0x006a63e0,
            ni_tmap_base_rigid_body_obstacle_data_set_value(Ptr<NiTMap>, Ptr<NiTMapItem>, u32, u32)
        ),
        entry!(0x006c00b0, fn_006c00b0(Ptr) -> Ptr),
        entry!(
            0x006c0190,
            bhk_obstacle_deactivation_listener_entity_deactivated_callback(Ptr, u32)
        ),
        entry!(
            0x006c01d0,
            bhk_obstacle_deactivation_listener_entity_activated_callback(Ptr, u32)
        ),
        entry!(0x006c0210, fn_006c0210(Ptr) -> Ptr),
        entry!(0x006c0230, fn_006c0230(Ptr, u32) -> Ptr),
        entry!(0x006c0260, fn_006c0260(Ptr)),
        entry!(
            0x006c0280,
            bhk_obstacle_removal_listener_entity_removed_callback(Ptr, u32)
        ),
        entry!(
            0x006c02c0,
            nav_mesh_obstacle_manager_nav_mesh_obstacle_manager(
                Ptr<NavMeshObstacleManager>,
            )
                -> Ptr<NavMeshObstacleManager>
        ),
        entry!(0x006c0510, fn_006c0510(Ptr<NavMeshObstacleManager>)),
        entry!(0x006c0720, fn_006c0720() -> Ptr<NavMeshObstacleManager>),
        entry!(
            0x006c07d0,
            nav_mesh_obstacle_manager_initialize(Ptr<NavMeshObstacleManager>) -> bool
        ),
        entry!(0x006c08e0, fn_006c08e0(Ptr) -> bool),
        entry!(
            0x006c0900,
            nav_mesh_obstacle_manager_release(Ptr<NavMeshObstacleManager>)
        ),
        entry!(0x006c09d0, fn_006c09d0(Ptr)),
        entry!(
            0x006c09f0,
            nav_mesh_obstacle_manager_clear_all_obstacles(Ptr<NavMeshObstacleManager>)
        ),
        entry!(
            0x006c0c30,
            nav_mesh_obstacle_manager_add_obstacle_for_reference(Ptr<NavMeshObstacleManager>, u32)
        ),
        entry!(
            0x006c0c80,
            nav_mesh_obstacle_manager_remove_obstacle_for_reference(
                Ptr<NavMeshObstacleManager>,
                u32,
            )
        ),
        entry!(0x006c0dc0, fn_006c0dc0(Ptr<NavMeshObstacleManager>, u32)),
        entry!(
            0x006c0f10,
            nav_mesh_obstacle_manager_on_door_close(Ptr<NavMeshObstacleManager>, u32)
        ),
        entry!(0x006c1060, fn_006c1060(Ptr<NavMeshObstacleManager>, u32)),
        entry!(0x006c1130, fn_006c1130(Ptr<NavMeshObstacleManager>, u32)),
        entry!(0x006c1150, fn_006c1150(Ptr<NavMeshObstacleManager>, u32)),
        entry!(0x006c1170, fn_006c1170(Ptr<NavMeshObstacleManager>)),
        entry!(0x006c1190, fn_006c1190(Ptr<NavMeshObstacleManager>, u32)),
        entry!(0x006c1480, reference_obstacle_array_construct(Ptr) -> Ptr),
        entry!(0x006c14f0, reference_obstacle_array_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x006c1520, reference_obstacle_array_destructor(Ptr)),
        entry!(0x006c1590, fn_006c1590(Ptr<NavMeshObstacleManager>, u32)),
        entry!(0x006c1760, fn_006c1760(Ptr<NavMeshObstacleManager>, u32)),
        entry!(
            0x006c1920,
            fn_006c1920(Ptr<NavMeshObstacleManager>, u32, u32)
        ),
        entry!(0x006c1c70, fn_006c1c70(Ptr<NavMeshObstacleManager>, u32)),
        entry!(0x006c1c90, fn_006c1c90(Ptr<NavMeshObstacleManager>, u32)),
        entry!(
            0x006c1cb0,
            nav_mesh_obstacle_manager_on_obstacle_lost3d(Ptr<NavMeshObstacleManager>, u32)
        ),
        entry!(0x006c1d60, fn_006c1d60(Ptr<NavMeshObstacleManager>, u32)),
        entry!(0x006c1da0, fn_006c1da0(Ptr, u32)),
        entry!(0x006c1dd0, fn_006c1dd0(Ptr, u32)),
        entry!(0x006c1e00, fn_006c1e00(Ptr<NavMeshObstacleManager>, u32)),
        entry!(0x006c1e40, fn_006c1e40(Ptr, u32)),
        entry!(0x006c1e70, fn_006c1e70(Ptr, u32)),
        entry!(
            0x006c1ea0,
            nav_mesh_obstacle_manager_get_cell_list_for_obstacle(u32, u32) -> bool
        ),
        entry!(0x006c2050, fn_006c2050(u32, u32) -> bool),
        entry!(0x006c2210, fn_006c2210(u32, u32) -> bool),
        entry!(0x006c23c0, fn_006c23c0(Ptr) -> Ptr),
        entry!(
            0x006c2490,
            obstacle_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006c24d0, fn_006c24d0(Ptr)),
        entry!(0x006c2550, fn_006c2550(u32, u32) -> bool),
        entry!(0x006c2650, fn_006c2650(u32) -> bool),
        entry!(0x006c3440, fn_006c3440(u32)),
        entry!(0x006c34f0, fn_006c34f0(u32, u32) -> bool),
        entry!(0x006c3520, fn_006c3520(u32, u32) -> bool),
        entry!(0x006c3570, fn_006c3570(u32, u32, f32, f32) -> bool),
        entry!(0x006c3640, fn_006c3640(Ptr<NavMeshObstacleManager>)),
        entry!(0x006c3920, fn_006c3920(Ptr<NavMeshObstacleManager>) -> u32),
        entry!(0x006c3970, fn_006c3970(Ptr<NavMeshObstacleManager>) -> u32),
        entry!(0x006c39c0, fn_006c39c0(Ptr<NavMeshObstacleManager>)),
        entry!(
            0x006c3ba0,
            nav_mesh_obstacle_manager_reconnect_portals(Ptr<NavMeshObstacleManager>) -> bool
        ),
        entry!(0x006c4350, fn_006c4350(Ptr, u32) -> Ptr),
        entry!(
            0x006c4390,
            fn_006c4390(Ptr<NavMeshObstacleManager>, u32, u32, u32, u32) -> bool
        ),
        entry!(
            0x006c4460,
            fn_006c4460(Ptr<NavMeshObstacleManager>, u32, u32, u32) -> bool
        ),
        entry!(0x006c44d0, fn_006c44d0(Ptr<NavMeshObstacleManager>) -> bool),
        entry!(0x006c4540, fn_006c4540(Ptr<NavMeshObstacleManager>) -> bool),
        entry!(0x006c45e0, fn_006c45e0(Ptr<NavMeshObstacleManager>)),
        entry!(0x006c47e0, fn_006c47e0(Ptr<NavMeshObstacleManager>)),
        entry!(0x006c4900, fn_006c4900(Ptr<NavMeshObstacleManager>) -> bool),
        entry!(0x006c4970, fn_006c4970(Ptr<NavMeshObstacleManager>) -> bool),
        entry!(0x006c49e0, fn_006c49e0(Ptr<NavMeshObstacleManager>)),
        entry!(0x006c4ad0, fn_006c4ad0(Ptr<NavMeshObstacleManager>)),
        entry!(0x006c4c30, fn_006c4c30(Ptr<NavMeshObstacleManager>)),
        entry!(0x006c4d30, fn_006c4d30(Ptr<NavMeshObstacleManager>)),
        entry!(0x006c4ec0, fn_006c4ec0(Ptr<NavMeshObstacleManager>)),
        entry!(
            0x006c4f70,
            nav_mesh_obstacle_manager_add_processed_task(Ptr<NavMeshObstacleManager>, u32)
        ),
        entry!(0x006c4fb0, fn_006c4fb0(Ptr<NavMeshObstacleManager>) -> bool),
        entry!(0x006c5190, fn_006c5190(Ptr<NavMeshObstacleManager>) -> bool),
        entry!(
            0x006c5460,
            nav_mesh_obstacle_manager_draw_obstacles(Ptr<NavMeshObstacleManager>, u8)
        ),
        entry!(
            0x006c5840,
            nav_mesh_obstacle_manager_create_bounding_box3d_node(
                Ptr<NavMeshObstacleManager>,
                u32,
                u32,
            )
        ),
        entry!(
            0x006c5b20,
            nav_mesh_obstacle_manager_add_bounding_box3d_node(Ptr<NavMeshObstacleManager>, u32)
        ),
        entry!(0x006c5c50, fn_006c5c50(Ptr<NavMeshObstacleManager>, u32)),
        entry!(
            0x006c5ce0,
            fn_006c5ce0(Ptr<NavMeshObstacleManager>, u32) -> bool
        ),
        entry!(0x006c5e40, fn_006c5e40(Ptr<BSTasklet>) -> Ptr<BSTasklet>),
        entry!(0x006c5e70, fn_006c5e70(Ptr<BSTasklet>)),
        entry!(
            0x006c5e90,
            bs_tasklet_scalar_deleting_destructor(Ptr<BSTasklet>, u32) -> Ptr<BSTasklet>
        ),
        entry!(0x006c5ec0, fn_006c5ec0(Ptr, u32, u8) -> bool),
        entry!(
            0x006c5ef0,
            fn_006c5ef0(Ptr<NavMeshObstacleManager>, u32) -> u32
        ),
        entry!(0x006c5fc0, fn_006c5fc0(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x006c5ff0, fn_006c5ff0(Ptr<LockFreeQueue>, u32)),
        entry!(0x006c6030, fn_006c6030(Ptr<LockFreeQueue>, u32) -> bool),
        entry!(0x006c6070, fn_006c6070(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x006c60a0, fn_006c60a0(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(
            0x006c60d0,
            ni_tmap_reference_obstacle_array_scalar_deleting_destructor(
                Ptr<NiTMap>,
                u32,
            ) -> Ptr<NiTMap>
        ),
        entry!(
            0x006c6100,
            ni_tmap_rigid_body_obstacle_data_scalar_deleting_destructor(
                Ptr<NiTMap>,
                u32,
            ) -> Ptr<NiTMap>
        ),
        entry!(
            0x006c6130,
            ni_tmap_obstacle_task_data_scalar_deleting_destructor(Ptr<NiTMap>, u32) -> Ptr<NiTMap>
        ),
        entry!(
            0x006c6160,
            fn_006c6160(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006c6190,
            bs_simple_array_obstacle_data_destructor(Ptr<BSSimpleArray>)
        ),
        entry!(0x006c6260, fn_006c6260(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x006c6350, fn_006c6350(Ptr<NiTMap>)),
        entry!(0x006c6400, fn_006c6400(Ptr<BSSimpleList>, u32) -> bool),
        entry!(
            0x006c6450,
            fn_006c6450(Ptr<LockFreeQueue>, u32, u32) -> Ptr<LockFreeQueue>
        ),
        entry!(
            0x006c65b0,
            lock_free_queue_rigid_body_allocate_interface(Ptr<LockFreeQueue>, u32) -> u32
        ),
        entry!(0x006c6680, fn_006c6680(Ptr<LockFreeQueue>)),
        entry!(0x006c6700, fn_006c6700(Ptr<LockFreeQueue>, u8)),
        entry!(0x006c68b0, fn_006c68b0(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x006c6920, fn_006c6920(Ptr<NiTMap>, u32, u32)),
        entry!(0x006c6a60, fn_006c6a60(Ptr<NiTMap>, u32) -> u32),
        entry!(0x006c6a80, fn_006c6a80(Ptr<NiTMap>)),
        entry!(0x006c6ae0, fn_006c6ae0(Ptr<NiTMap>)),
        entry!(0x006c6b10, fn_006c6b10(Ptr<NiTMap>, u32) -> Ptr<NiTMap>),
        entry!(0x006c6b80, fn_006c6b80(Ptr<NiTMap>)),
        entry!(0x006c6be0, fn_006c6be0(Ptr<NiTMap>)),
        entry!(
            0x006c6c10,
            fn_006c6c10(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x006c6c40, fn_006c6c40(Ptr<BSSimpleArray>)),
        entry!(
            0x006c6c60,
            fn_006c6c60(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x006c6ce0, fn_006c6ce0(Ptr<BSSimpleArray>)),
        entry!(
            0x006c6d40,
            fn_006c6d40(Ptr<BSSimpleArray>, u32, u32, u32) -> Ptr<BSSimpleArray>
        ),
        entry!(0x006c6de0, fn_006c6de0(Ptr<BSSimpleArray>)),
        entry!(0x006c6e00, fn_006c6e00(Ptr<BSSimpleArray>)),
        entry!(0x006c6e60, fn_006c6e60(Ptr<LockFreeQueueInterface>, u32)),
        entry!(
            0x006c6fe0,
            fn_006c6fe0(Ptr<LockFreeQueueInterface>, u32) -> bool
        ),
        entry!(
            0x006c7110,
            bs_simple_array_obstacle_data_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006c7140,
            ni_tmap_base_reference_obstacle_array_scalar_deleting_destructor(
                Ptr<NiTMap>,
                u32,
            )
                -> Ptr<NiTMap>
        ),
        entry!(
            0x006c7170,
            lock_free_queue_rigid_body_scalar_deleting_destructor(
                Ptr<LockFreeQueue>,
                u32,
            )
                -> Ptr<LockFreeQueue>
        ),
        entry!(0x006c71a0, fn_006c71a0(Ptr, u32) -> Ptr),
        entry!(
            0x006c71d0,
            ni_tmap_base_rigid_body_obstacle_data_scalar_deleting_destructor(
                Ptr<NiTMap>,
                u32,
            )
                -> Ptr<NiTMap>
        ),
        entry!(
            0x006c7200,
            ni_tmap_base_obstacle_task_data_scalar_deleting_destructor(
                Ptr<NiTMap>,
                u32,
            ) -> Ptr<NiTMap>
        ),
        entry!(
            0x006c7230,
            bs_simple_array_obstacle_task_data_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006c7260,
            bs_scrap_array_nav_mesh_ptr_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006c7290,
            bs_simple_array_tes_object_cell_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(
            0x006c72c0,
            bs_scrap_array_tes_object_cell_scalar_deleting_destructor(
                Ptr<BSSimpleArray>,
                u32,
            )
                -> Ptr<BSSimpleArray>
        ),
        entry!(0x006c74a0, fn_006c74a0(Ptr)),
        entry!(0x006c7560, fn_006c7560(Ptr<LockFreeQueueInterface>, u32)),
        entry!(
            0x006c77a0,
            fn_006c77a0(Ptr<ObstacleTaskData>, u32) -> Ptr<ObstacleTaskData>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// Doubles that do nothing and return zero.
    fn stub(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    /// A double that returns a fixed value.
    fn stub_ret(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| ret(value));
    }

    /// A double that records the argument words of every call and returns
    /// `value`.
    fn record(e: &mut Engine, address: u32, value: u32) -> Log {
        let log: Log = Rc::default();
        let shared = log.clone();
        e.register_double(address, move |_, a| {
            shared.borrow_mut().push(a.to_vec());
            ret(value)
        });
        log
    }

    /// A double for a function that takes a list or queue and the address of
    /// a word: records (first argument, the word).
    fn record_word(e: &mut Engine, address: u32) -> Log {
        let log: Log = Rc::default();
        let shared = log.clone();
        e.register_double(address, move |e, a| {
            let value = e.mem.u32(a[1]);
            shared.borrow_mut().push(vec![a[0], value]);
            Ret::default()
        });
        log
    }

    /// An engine with the pages the code reads mapped and the accessors of
    /// `NiPointer`, `BSSimpleArray` and `TESForm` working on game memory.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_7000, 0x1000);
        e.map(0x0106_c000, 0x1000);
        e.register(NI_POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(NI_POINTER_INIT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        stub(&mut e, &[NI_POINTER_RELEASE]);
        for address in [NI_POINTER_COPY, NI_POINTER_ASSIGN] {
            e.register(address, |e, a| {
                let value = e.mem.u32(a[1]);
                e.mem.set_u32(a[0], value);
                ret(a[0])
            });
        }
        e.register(NI_POINTER_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_AT, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 4));
        e.register(FORM_ID, |e, a| ret(e.mem.u32(a[0] + 0x0c)));
        e
    }

    /// The recorded calls to `addresses`, in order.
    fn calls(e: &Engine, addresses: &[u32]) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| addresses.contains(a))
            .cloned()
            .collect()
    }

    fn start_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// A `BSSimpleArray` header written at `at` (buffer at `+4`, size at
    /// `+8`).
    fn set_array(e: &mut Engine, at: u32, elements: &[u32]) {
        let buffer = e.mem.alloc(4 * elements.len().max(1) as u32);
        for (i, value) in elements.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *value);
        }
        e.mem.set_u32(at + 4, buffer);
        e.mem.set_u32(at + 8, elements.len() as u32);
    }

    /// A `NiPointer` slot holding `value`.
    fn slot_with(e: &mut Engine, value: u32) -> u32 {
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, value);
        slot
    }

    /// An `ObstacleData`: rigid body, parent array, active flag and the
    /// recorded `NavMeshInfo` pointers.
    fn obstacle(e: &mut Engine, rigid_body: u32, parent: u32, active: bool, infos: &[u32]) -> u32 {
        let data = e.mem.alloc(0x8c);
        e.mem.set_u32(data + 8, parent);
        e.mem.set_u32(data + 0x0c, rigid_body);
        e.mem.set_u8(data + 0x74, active as u8);
        set_array(e, data + 0x78, infos);
        data
    }

    /// A `ReferenceObstacleArray` with the given form id and obstacles.
    fn reference_array(e: &mut Engine, form_id: u32, obstacles: &[u32]) -> u32 {
        let array = e.mem.alloc(0x1c);
        e.mem.set_u32(array + 8, form_id);
        set_array(e, array + 0x0c, obstacles);
        array
    }

    fn manager(e: &mut Engine) -> Ptr<NavMeshObstacleManager> {
        e.new_object()
    }

    /// A bare `TESForm`-like object: only the word at `+0x0c` (the id).
    fn reference_with_id(e: &mut Engine, form_id: u32) -> u32 {
        let reference = e.mem.alloc(0x30);
        e.mem.set_u32(reference + 0x0c, form_id);
        reference
    }

    /// A map item `[next, key, value]`.
    fn map_item(e: &mut Engine, next: u32, key: u32, value: u32) -> u32 {
        let item = e.mem.alloc(12);
        e.mem.set_u32(item, next);
        e.mem.set_u32(item + 4, key);
        e.mem.set_u32(item + 8, value);
        item
    }

    /// A list node `[item, next]`.
    fn list_node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, next);
        node
    }

    /// The obstacle setting: a byte the setting getter points at.
    fn set_setting(e: &mut Engine, on: bool) {
        let value = e.mem.alloc(4);
        e.mem.set_u8(value, on as u8);
        stub_ret(e, SETTING_VALUE_ADDRESS, value);
    }

    /// A map whose hash function is `key % 4`, with four buckets.
    fn hashed_map(e: &mut Engine, at: u32) -> u32 {
        e.put_vtable(0x0080_0000, &[0, 0x0070_0000]);
        e.register(0x0070_0000, |_, a| ret(a[1] % 4));
        let table = e.mem.alloc(16);
        e.mem.set_u32(at, 0x0080_0000);
        e.mem.set_u32(at + 4, 4);
        e.mem.set_u32(at + 8, table);
        table
    }

    #[test]
    fn get_next_moves_along_the_chain_then_through_the_buckets() {
        let mut e = engine();
        let map = e.mem.alloc(0x10);
        let table = hashed_map(&mut e, map);
        let last = map_item(&mut e, 0, 3, 0xccc);
        let second = map_item(&mut e, 0, 1, 0xbbb);
        let first = map_item(&mut e, second, 5, 0xaaa);
        e.mem.set_u32(table + 12, last);
        let frame = e.mem.alloc(12);
        let (position, key, value) = (frame, frame + 4, frame + 8);
        let step = |e: &mut Engine| {
            ni_tmap_base_reference_obstacle_array_get_next(
                e,
                Ptr::new(map),
                Ptr::new(position),
                Ptr::new(key),
                Ptr::new(value),
            )
        };

        // An item with a successor in its chain.
        e.mem.set_u32(position, first);
        step(&mut e);
        assert_eq!(e.mem.u32(position), second);
        assert_eq!(e.mem.u32(key), 5);
        assert_eq!(e.mem.u32(value), 0xaaa);

        // The last of its chain: the next non-empty bucket after bucket 1.
        step(&mut e);
        assert_eq!(e.mem.u32(position), last);
        assert_eq!(e.mem.u32(key), 1);
        assert_eq!(e.mem.u32(value), 0xbbb);

        // The last bucket: the walk ends.
        step(&mut e);
        assert_eq!(e.mem.u32(position), 0);
        assert_eq!(e.mem.u32(key), 3);
    }

    /// The doubles `BSSimpleList<NiPointer<ObstacleData>>::Remove` needs.
    fn list_engine() -> (Engine, Log) {
        let mut e = engine();
        e.register(FIRST_WORD_EQUALS, |e, a| {
            ret((e.mem.u32(a[0]) == a[1]) as u32)
        });
        e.register(LIST_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0] + 4) == 0 && e.mem.u32(a[0]) == 0) as u32)
        });
        e.register(LIST_NODE_DIFFERS, |e, a| {
            ret((e.mem.u32(a[0]) != e.mem.u32(a[1])) as u32)
        });
        let deleted = record(&mut e, LIST_NODE_DELETE, 0);
        (e, deleted)
    }

    #[test]
    fn list_remove_unlinks_a_middle_node() {
        let (mut e, deleted) = list_engine();
        let third = list_node(&mut e, 30, 0);
        let second = list_node(&mut e, 20, third);
        let head = list_node(&mut e, 10, second);
        let item = slot_with(&mut e, 20);
        bs_simple_list_obstacle_data_remove(&mut e, Ptr::new(head), Ptr::new(item));
        assert_eq!(e.mem.u32(head + 4), third);
        assert_eq!(e.mem.u32(second + 4), 0);
        assert_eq!(*deleted.borrow(), vec![vec![second, 1]]);
    }

    #[test]
    fn list_remove_of_the_head_takes_the_next_nodes_item() {
        let (mut e, deleted) = list_engine();
        let third = list_node(&mut e, 30, 0);
        let second = list_node(&mut e, 20, third);
        let head = list_node(&mut e, 10, second);
        let item = slot_with(&mut e, 10);
        bs_simple_list_obstacle_data_remove(&mut e, Ptr::new(head), Ptr::new(item));
        assert_eq!(e.mem.u32(head), 20);
        assert_eq!(e.mem.u32(head + 4), third);
        assert_eq!(*deleted.borrow(), vec![vec![second, 1]]);
    }

    #[test]
    fn list_remove_of_the_only_node_clears_its_item() {
        let (mut e, deleted) = list_engine();
        let head = list_node(&mut e, 10, 0);
        let item = slot_with(&mut e, 10);
        bs_simple_list_obstacle_data_remove(&mut e, Ptr::new(head), Ptr::new(item));
        assert_eq!(e.mem.u32(head), 0);
        assert!(deleted.borrow().is_empty());
    }

    #[test]
    fn list_remove_ignores_null_items_and_missing_ones() {
        let (mut e, deleted) = list_engine();
        let second = list_node(&mut e, 20, 0);
        let head = list_node(&mut e, 10, second);
        let null_item = slot_with(&mut e, 0);
        bs_simple_list_obstacle_data_remove(&mut e, Ptr::new(head), Ptr::new(null_item));
        let missing = slot_with(&mut e, 99);
        bs_simple_list_obstacle_data_remove(&mut e, Ptr::new(head), Ptr::new(missing));
        assert_eq!(e.mem.u32(head + 4), second);
        assert_eq!(e.mem.u32(head), 10);
        assert!(deleted.borrow().is_empty());
    }

    #[test]
    fn delete_item_releases_the_value_and_frees_the_item() {
        let mut e = engine();
        let free = record(&mut e, MAP_FREE_ITEM, 0);
        let map = e.mem.alloc(0x10);
        let item = map_item(&mut e, 0, 7, 0x1234);
        ni_tmap_reference_obstacle_array_delete_item(&mut e, Ptr::new(map), Ptr::new(item));
        assert_eq!(e.mem.u32(item + 8), 0);
        assert_eq!(*free.borrow(), vec![vec![map + 0x0c, item]]);
    }

    #[test]
    fn set_value_stores_the_key_and_assigns_then_destroys_the_argument() {
        let mut e = engine();
        start_log(&mut e);
        let item = map_item(&mut e, 0, 0, 0);
        ni_tmap_base_rigid_body_obstacle_data_set_value(
            &mut e,
            Ptr::new(0),
            Ptr::new(item),
            0x77,
            0x1234,
        );
        assert_eq!(e.mem.u32(item + 4), 0x77);
        assert_eq!(e.mem.u32(item + 8), 0x1234);
        let log = calls(&e, &[NI_POINTER_ASSIGN, NI_POINTER_RELEASE]);
        assert_eq!(log.len(), 2);
        assert_eq!(log[0].0, NI_POINTER_ASSIGN);
        assert_eq!(log[0].1[0], item + 8);
        assert_eq!(log[1].0, NI_POINTER_RELEASE);
        assert_eq!(log[1].1[0], log[0].1[1]);
    }

    #[test]
    fn the_listener_constructors_set_their_vtables() {
        let mut e = engine();
        let activation_base = record(&mut e, ACTIVATION_LISTENER_BASE_CTOR, 0);
        let entity_base = record(&mut e, ENTITY_LISTENER_BASE_CTOR, 0);
        let deactivation = e.mem.alloc(4);
        assert_eq!(
            fn_006c00b0(&mut e, Ptr::new(deactivation)).addr(),
            deactivation
        );
        assert_eq!(e.mem.u32(deactivation), DEACTIVATION_LISTENER_VTABLE);
        assert_eq!(*activation_base.borrow(), vec![vec![deactivation]]);
        let removal = e.mem.alloc(4);
        assert_eq!(fn_006c0210(&mut e, Ptr::new(removal)).addr(), removal);
        assert_eq!(e.mem.u32(removal), REMOVAL_LISTENER_VTABLE);
        assert_eq!(*entity_base.borrow(), vec![vec![removal]]);
    }

    #[test]
    fn the_removal_listener_destructor_runs_the_base_destructor() {
        let mut e = engine();
        let base = record(&mut e, ENTITY_LISTENER_BASE_DTOR, 0);
        let listener = e.mem.alloc(4);
        fn_006c0260(&mut e, Ptr::new(listener));
        assert_eq!(e.mem.u32(listener), REMOVAL_LISTENER_VTABLE);
        assert_eq!(*base.borrow(), vec![vec![listener]]);
    }

    #[test]
    fn the_scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = engine();
        stub(&mut e, &[ENTITY_LISTENER_BASE_DTOR]);
        let freed = record(&mut e, OPERATOR_DELETE, 0);
        let listener = e.mem.alloc(4);
        assert_eq!(fn_006c0230(&mut e, Ptr::new(listener), 0).addr(), listener);
        assert!(freed.borrow().is_empty());
        assert_eq!(fn_006c0230(&mut e, Ptr::new(listener), 3).addr(), listener);
        assert_eq!(*freed.borrow(), vec![vec![listener]]);
    }

    /// An engine whose manager instance exists (so `006c0720` only returns
    /// it), with the queue pushes recorded by value and the entity user
    /// data `0x4321`.
    fn callback_engine() -> (Engine, Log) {
        let mut e = engine();
        e.set_global(MANAGER_INSTANCE_GUARD, 1u32);
        stub(&mut e, &[SCOPE_GUARD_CTOR, SCOPE_GUARD_DTOR]);
        let pushes = record_word(&mut e, QUEUE_PUSH);
        stub_ret(&mut e, ENTITY_USER_DATA, 0x4321);
        (e, pushes)
    }

    #[test]
    fn the_deactivation_callback_queues_the_rigid_body_for_removal() {
        let (mut e, pushes) = callback_engine();
        bhk_obstacle_deactivation_listener_entity_deactivated_callback(&mut e, Ptr::NULL, 0x99);
        assert_eq!(
            *pushes.borrow(),
            vec![vec![MANAGER_INSTANCE + 0xa0, 0x4321]]
        );
        // No user data: nothing is queued.
        stub_ret(&mut e, ENTITY_USER_DATA, 0);
        bhk_obstacle_deactivation_listener_entity_deactivated_callback(&mut e, Ptr::NULL, 0x99);
        assert_eq!(pushes.borrow().len(), 1);
    }

    #[test]
    fn the_activation_callback_queues_the_rigid_body_for_adding() {
        let (mut e, pushes) = callback_engine();
        bhk_obstacle_deactivation_listener_entity_activated_callback(&mut e, Ptr::NULL, 0x99);
        assert_eq!(
            *pushes.borrow(),
            vec![vec![MANAGER_INSTANCE + 0x60, 0x4321]]
        );
        stub_ret(&mut e, ENTITY_USER_DATA, 0);
        bhk_obstacle_deactivation_listener_entity_activated_callback(&mut e, Ptr::NULL, 0x99);
        assert_eq!(pushes.borrow().len(), 1);
    }

    #[test]
    fn the_removal_callback_reports_the_lost_obstacle() {
        let (mut e, _) = callback_engine();
        // OnObstacleLost3D: the rigid body is not in the map, so only the
        // lookup happens.
        let lookup = record(&mut e, MAP_GET, 0);
        bhk_obstacle_removal_listener_entity_removed_callback(&mut e, Ptr::NULL, 0x99);
        assert_eq!(lookup.borrow().len(), 1);
        assert_eq!(lookup.borrow()[0][..2], [MANAGER_INSTANCE + 0xe0, 0x4321]);
        stub_ret(&mut e, ENTITY_USER_DATA, 0);
        bhk_obstacle_removal_listener_entity_removed_callback(&mut e, Ptr::NULL, 0x99);
        assert_eq!(lookup.borrow().len(), 1);
    }

    /// Every constructor and destructor the manager's members use, and the
    /// calls around them.
    const MEMBER_FUNCTIONS: [u32; 24] = [
        CRITICAL_SECTION_CTOR,
        CRITICAL_SECTION_DTOR,
        MAP_CTOR,
        MAP_DTOR,
        TASK_MAP_CTOR,
        TASK_MAP_DTOR,
        RIGID_BODY_MAP_CTOR,
        RIGID_BODY_MAP_DTOR,
        REFS_LIST_CTOR,
        REFS_LIST_DTOR,
        UPDATE_LIST_CTOR,
        UPDATE_LIST_DTOR,
        QUEUE_CTOR,
        QUEUE_DTOR,
        TASK_ARRAY_CTOR,
        TASK_ARRAY_DTOR,
        TASKLET_GROUP_CTOR,
        TASKLET_GROUP_DTOR,
        PERFORMANCE_TIMER_REGISTER,
        SCOPE_GUARD_CTOR,
        SCOPE_GUARD_DTOR,
        MAP_CLEAR,
        LIST_CLEAR,
        UPDATE_LIST_CLEAR,
    ];

    /// Doubles for the member functions and `ClearAllObstacles`' callees,
    /// with an empty `FormIDMap`.
    fn member_engine() -> Engine {
        let mut e = engine();
        stub(&mut e, &MEMBER_FUNCTIONS);
        stub(&mut e, &[QUEUE_CLEAR, MAP_FIRST_ITEM, DRAW_OBSTACLES]);
        e
    }

    #[test]
    fn the_constructor_builds_the_members_in_order_and_sets_the_defaults() {
        let mut e = member_engine();
        e.set_global(INITIAL_TIME_TO_NEXT_SWAP, 0.5f32);
        start_log(&mut e);
        let this = manager(&mut e);
        let base = this.addr();
        // Junk the constructor must overwrite.
        e.mem.set_u32(base + 0x19c, 0xbeef);
        e.mem.set_u32(base + 0x120, 0xdead);
        e.mem.set_u32(base + 0x124, 0xdead);
        e.mem.set_u32(base + 0x190, 0xdead);
        e.mem.set_u8(base + 0x198, 1);
        e.mem.set_u8(base + 0x18, 1);
        let result = nav_mesh_obstacle_manager_nav_mesh_obstacle_manager(&mut e, this);
        assert_eq!(result, this);
        let expected = vec![
            (CRITICAL_SECTION_CTOR, vec![base]),
            (MAP_CTOR, vec![base + 0x1c, 0x25]),
            (REFS_LIST_CTOR, vec![base + 0x2c]),
            (REFS_LIST_CTOR, vec![base + 0x34]),
            (UPDATE_LIST_CTOR, vec![base + 0x3c]),
            (QUEUE_CTOR, vec![base + 0x60, 7, 8]),
            (QUEUE_CTOR, vec![base + 0xa0, 7, 8]),
            (RIGID_BODY_MAP_CTOR, vec![base + 0xe0, 0x25]),
            (REFS_LIST_CTOR, vec![base + 0xf0]),
            (REFS_LIST_CTOR, vec![base + 0xf8]),
            (MAP_CTOR, vec![base + 0x100, 0x25]),
            (MAP_CTOR, vec![base + 0x110, 0x25]),
            (TASKLET_GROUP_CTOR, vec![base + 0x128]),
            (TASK_MAP_CTOR, vec![base + 0x12c, 0x25]),
            (TASK_ARRAY_CTOR, vec![base + 0x13c]),
            (TASK_ARRAY_CTOR, vec![base + 0x14c]),
            (REFS_LIST_CTOR, vec![base + 0x160]),
            (REFS_LIST_CTOR, vec![base + 0x180]),
            (REFS_LIST_CTOR, vec![base + 0x188]),
            (
                PERFORMANCE_TIMER_REGISTER,
                vec![base + 0x1a0, PATHING_CATEGORY_NAME, MAIN_TIMER_NAME, 1, 1],
            ),
            (
                PERFORMANCE_TIMER_REGISTER,
                vec![
                    base + 0x1a1,
                    PATHING_CATEGORY_NAME,
                    TASKLET_TIMER_NAME,
                    0,
                    1,
                ],
            ),
        ];
        let mut observed = calls(&e, &MEMBER_FUNCTIONS);
        observed.retain(|(a, _)| *a != SCOPE_GUARD_CTOR && *a != SCOPE_GUARD_DTOR);
        assert_eq!(observed, expected);
        // The shared-object pointer is initialised to null.
        assert_eq!(e.mem.u32(base + 0x19c), 0);
        // The scope guard wraps the field defaults (category 0x2e, line 0xb8).
        let guards = calls(&e, &[SCOPE_GUARD_CTOR, SCOPE_GUARD_DTOR]);
        assert_eq!(guards.len(), 2);
        assert_eq!(guards[0].1[1..], [0x2e, 1, SOURCE_PATH, 0xb8]);
        assert_eq!(e.mem.u32(base + 0x120), 0);
        assert_eq!(e.mem.u32(base + 0x124), 0);
        assert_eq!(e.mem.u32(base + 0x190), 0);
        assert_eq!(e.mem.f32(base + 0x194), 0.5);
        assert_eq!(e.mem.u8(base + 0x198), 0);
        assert_eq!(e.mem.u8(base + 0x18), 0);
    }

    #[test]
    fn the_destructor_clears_deletes_the_listeners_and_destroys_the_members() {
        let mut e = member_engine();
        // Two listeners whose virtual deleting destructor is recorded.
        let deleted = record(&mut e, 0x0070_0100, 0);
        e.put_vtable(0x0080_0100, &[0x0070_0100]);
        let this = manager(&mut e);
        let base = this.addr();
        let first = slot_with(&mut e, 0x0080_0100);
        let second = slot_with(&mut e, 0x0080_0100);
        e.mem.set_u32(base + 0x120, first);
        e.mem.set_u32(base + 0x124, second);
        start_log(&mut e);
        fn_006c0510(&mut e, this);
        assert_eq!(*deleted.borrow(), vec![vec![first, 1], vec![second, 1]]);
        // The fields keep the dead pointers.
        assert_eq!(e.mem.u32(base + 0x120), first);
        let destroyed: Vec<_> = calls(
            &e,
            &[
                NI_POINTER_RELEASE,
                REFS_LIST_DTOR,
                TASK_ARRAY_DTOR,
                TASK_MAP_DTOR,
                TASKLET_GROUP_DTOR,
                MAP_DTOR,
                RIGID_BODY_MAP_DTOR,
                QUEUE_DTOR,
                UPDATE_LIST_DTOR,
                CRITICAL_SECTION_DTOR,
            ],
        )
        .into_iter()
        .filter(|(a, args)| *a != NI_POINTER_RELEASE || args[0] == base + 0x19c)
        .collect();
        let expected = vec![
            (NI_POINTER_RELEASE, vec![base + 0x19c]),
            (REFS_LIST_DTOR, vec![base + 0x188]),
            (REFS_LIST_DTOR, vec![base + 0x180]),
            (TASK_ARRAY_DTOR, vec![base + 0x14c]),
            (TASK_ARRAY_DTOR, vec![base + 0x13c]),
            (TASK_MAP_DTOR, vec![base + 0x12c]),
            (TASKLET_GROUP_DTOR, vec![base + 0x128]),
            (MAP_DTOR, vec![base + 0x110]),
            (MAP_DTOR, vec![base + 0x100]),
            (REFS_LIST_DTOR, vec![base + 0xf8]),
            (REFS_LIST_DTOR, vec![base + 0xf0]),
            (RIGID_BODY_MAP_DTOR, vec![base + 0xe0]),
            (QUEUE_DTOR, vec![base + 0xa0]),
            (QUEUE_DTOR, vec![base + 0x60]),
            (UPDATE_LIST_DTOR, vec![base + 0x3c]),
            (REFS_LIST_DTOR, vec![base + 0x34]),
            (REFS_LIST_DTOR, vec![base + 0x2c]),
            (MAP_DTOR, vec![base + 0x1c]),
            (CRITICAL_SECTION_DTOR, vec![base]),
        ];
        assert_eq!(destroyed, expected);
    }

    #[test]
    fn the_accessor_constructs_the_instance_once() {
        let mut e = member_engine();
        let exit = record(&mut e, ATEXIT, 0);
        start_log(&mut e);
        assert_eq!(fn_006c0720(&mut e).addr(), MANAGER_INSTANCE);
        assert_eq!(e.global::<u32>(MANAGER_INSTANCE_GUARD) & 1, 1);
        assert_eq!(*exit.borrow(), vec![vec![MANAGER_ATEXIT_DESTRUCTOR]]);
        assert_eq!(
            calls(&e, &[CRITICAL_SECTION_CTOR]),
            vec![(CRITICAL_SECTION_CTOR, vec![MANAGER_INSTANCE])]
        );
        let guards = calls(&e, &[SCOPE_GUARD_CTOR]);
        assert_eq!(guards[0].1[1..], [0x2e, 1, SOURCE_PATH, 0xd2]);
        // Second use: no second construction.
        assert_eq!(fn_006c0720(&mut e).addr(), MANAGER_INSTANCE);
        assert_eq!(exit.borrow().len(), 1);
        assert_eq!(calls(&e, &[CRITICAL_SECTION_CTOR]).len(), 1);
    }

    /// Doubles for `Initialize`: the tasklet manager, its group functions and
    /// an allocator that hands out four-byte blocks.
    fn initialize_engine(create: u32, check: u32) -> (Engine, Log, Log) {
        let mut e = engine();
        stub_ret(&mut e, TASKLET_MANAGER_INSTANCE, 0x7000);
        let create_log = record(&mut e, TASKLET_CREATE_TASK_GROUP, create);
        stub_ret(&mut e, TASKLET_GROUP_CHECK, check);
        let allocated: Log = Rc::default();
        let shared = allocated.clone();
        e.register_double(OPERATOR_NEW, move |e, a| {
            let block = e.mem.alloc(a[0]);
            shared.borrow_mut().push(vec![a[0], block]);
            ret(block)
        });
        stub(
            &mut e,
            &[ACTIVATION_LISTENER_BASE_CTOR, ENTITY_LISTENER_BASE_CTOR],
        );
        (e, create_log, allocated)
    }

    #[test]
    fn initialize_stops_when_the_group_cannot_be_made() {
        let (mut e, create, allocated) = initialize_engine(0, 1);
        let this = manager(&mut e);
        assert!(!nav_mesh_obstacle_manager_initialize(&mut e, this));
        assert_eq!(*create.borrow(), vec![vec![0x7000, this.addr() + 0x128]]);
        assert!(allocated.borrow().is_empty());
    }

    #[test]
    fn initialize_stops_when_the_group_check_fails() {
        let (mut e, _, allocated) = initialize_engine(1, 0);
        let this = manager(&mut e);
        assert!(!nav_mesh_obstacle_manager_initialize(&mut e, this));
        assert!(allocated.borrow().is_empty());
    }

    #[test]
    fn initialize_makes_both_listeners() {
        let (mut e, _, allocated) = initialize_engine(1, 1);
        let this = manager(&mut e);
        assert!(nav_mesh_obstacle_manager_initialize(&mut e, this));
        let blocks = allocated.borrow().clone();
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0][0], 4);
        assert_eq!(e.mem.u32(this.addr() + 0x120), blocks[0][1]);
        assert_eq!(e.mem.u32(this.addr() + 0x124), blocks[1][1]);
        assert_eq!(e.mem.u32(blocks[0][1]), DEACTIVATION_LISTENER_VTABLE);
        assert_eq!(e.mem.u32(blocks[1][1]), REMOVAL_LISTENER_VTABLE);
    }

    #[test]
    fn the_group_check_and_flush_pass_the_group_to_the_tasklet_manager() {
        let mut e = engine();
        stub_ret(&mut e, TASKLET_MANAGER_INSTANCE, 0x7000);
        let check = record(&mut e, TASKLET_GROUP_CHECK, 1);
        let flush = record(&mut e, TASKLET_GROUP_FLUSH, 0);
        assert!(fn_006c08e0(&mut e, Ptr::new(0x1230)));
        assert_eq!(*check.borrow(), vec![vec![0x7000, 0x1230]]);
        fn_006c09d0(&mut e, Ptr::new(0x1230));
        assert_eq!(*flush.borrow(), vec![vec![0x7000, 0x1230]]);
        stub_ret(&mut e, TASKLET_GROUP_CHECK, 0);
        assert!(!fn_006c08e0(&mut e, Ptr::new(0x1230)));
    }

    #[test]
    fn release_deletes_the_listeners_and_destroys_the_group() {
        let mut e = member_engine();
        stub_ret(&mut e, TASKLET_MANAGER_INSTANCE, 0x7000);
        let flush = record(&mut e, TASKLET_GROUP_FLUSH, 0);
        let destroy = record(&mut e, TASKLET_DESTROY_TASK_GROUP, 0);
        let deleted = record(&mut e, 0x0070_0100, 0);
        e.put_vtable(0x0080_0100, &[0x0070_0100]);
        let this = manager(&mut e);
        let base = this.addr();
        let listener = slot_with(&mut e, 0x0080_0100);
        e.mem.set_u32(base + 0x120, listener);
        // The removal listener is already null.
        nav_mesh_obstacle_manager_release(&mut e, this);
        assert_eq!(*deleted.borrow(), vec![vec![listener, 1]]);
        assert_eq!(e.mem.u32(base + 0x120), 0);
        assert_eq!(*flush.borrow(), vec![vec![0x7000, base + 0x128]]);
        assert_eq!(*destroy.borrow(), vec![vec![0x7000, base + 0x128]]);
    }

    /// A double that records the argument words plus the word the argument
    /// at `index` points at, and returns `value`.
    fn record_deref(e: &mut Engine, address: u32, value: u32, index: usize) -> Log {
        let log: Log = Rc::default();
        let shared = log.clone();
        e.register_double(address, move |e, a| {
            let mut row = a.to_vec();
            row.push(e.mem.u32(a[index]));
            shared.borrow_mut().push(row);
            ret(value)
        });
        log
    }

    /// One reference array in `FormIDMap`, with two obstacles; the first has
    /// one recorded navmesh.
    struct ClearScene {
        e: Engine,
        this: Ptr<NavMeshObstacleManager>,
        removed: Log,
        loaded: Log,
        util: Log,
    }

    fn clear_scene(queued_to_remove: bool, draw: bool) -> ClearScene {
        let mut e = member_engine();
        let this = manager(&mut e);
        let base = this.addr();
        let table = hashed_map(&mut e, base + 0x1c);
        let _ = table;
        let key = 0x42;
        let first = obstacle(&mut e, 0x501, 0, false, &[0x6001]);
        let second = obstacle(&mut e, 0x502, 0, false, &[]);
        let array = reference_array(&mut e, key, &[first, second]);
        let item = map_item(&mut e, 0, key, array);
        stub_ret(&mut e, MAP_FIRST_ITEM, item);
        stub_ret(&mut e, LIST_CONTAINS, queued_to_remove as u32);
        let removed = record(&mut e, REMOVE_RIGID_BODY, 0);
        e.register(NAVMESH_HOLDER_CTOR, |e, a| {
            e.mem.set_u32(a[0], 0x9999);
            Ret::default()
        });
        let loaded = record(&mut e, NAVMESH_INFO_LOAD, 1);
        let util = record(&mut e, NAVMESH_UTIL_CALL, 0);
        stub(&mut e, &[NAVMESH_HOLDER_DTOR]);
        e.mem.set_u8(base + 0x198, draw as u8);
        ClearScene {
            e,
            this,
            removed,
            loaded,
            util,
        }
    }

    #[test]
    fn clear_all_obstacles_removes_rigid_bodies_and_notifies_navmeshes() {
        let mut scene = clear_scene(false, true);
        let base = scene.this.addr();
        start_log(&mut scene.e);
        nav_mesh_obstacle_manager_clear_all_obstacles(&mut scene.e, scene.this);
        assert_eq!(
            *scene.removed.borrow(),
            vec![vec![base, 0x501], vec![base, 0x502]]
        );
        assert_eq!(scene.loaded.borrow().len(), 1);
        assert_eq!(scene.loaded.borrow()[0][0], 0x6001);
        assert_eq!(*scene.util.borrow(), vec![vec![0x9999]]);
        let tail = calls(
            &scene.e,
            &[
                MAP_CLEAR,
                LIST_CLEAR,
                UPDATE_LIST_CLEAR,
                QUEUE_CLEAR,
                DRAW_OBSTACLES,
            ],
        );
        assert_eq!(
            tail,
            vec![
                (MAP_CLEAR, vec![base + 0x1c]),
                (MAP_CLEAR, vec![base + 0x100]),
                (MAP_CLEAR, vec![base + 0x110]),
                (MAP_CLEAR, vec![base + 0xe0]),
                (LIST_CLEAR, vec![base + 0xf0]),
                (LIST_CLEAR, vec![base + 0xf8]),
                (LIST_CLEAR, vec![base + 0x2c]),
                (LIST_CLEAR, vec![base + 0x34]),
                (UPDATE_LIST_CLEAR, vec![base + 0x3c]),
                (QUEUE_CLEAR, vec![base + 0x60, 0]),
                (QUEUE_CLEAR, vec![base + 0xa0, 0]),
                (DRAW_OBSTACLES, vec![base, 0]),
            ]
        );
    }

    #[test]
    fn clear_all_obstacles_keeps_queued_removals_and_skips_the_drawing() {
        let mut scene = clear_scene(true, false);
        start_log(&mut scene.e);
        nav_mesh_obstacle_manager_clear_all_obstacles(&mut scene.e, scene.this);
        assert!(scene.removed.borrow().is_empty());
        assert_eq!(scene.loaded.borrow().len(), 1);
        assert!(calls(&scene.e, &[DRAW_OBSTACLES]).is_empty());
    }

    #[test]
    fn adding_a_reference_queues_it_once_when_the_setting_is_on() {
        let mut e = engine();
        let this = manager(&mut e);
        let queue = this.addr() + 0x2c;
        let added = record_word(&mut e, LIST_ADD_HEAD);
        let contains = record(&mut e, LIST_CONTAINS, 0);
        // Setting off: nothing happens.
        set_setting(&mut e, false);
        nav_mesh_obstacle_manager_add_obstacle_for_reference(&mut e, this, 0xabc);
        assert!(contains.borrow().is_empty());
        // Setting on, not queued yet.
        set_setting(&mut e, true);
        nav_mesh_obstacle_manager_add_obstacle_for_reference(&mut e, this, 0xabc);
        assert_eq!(*added.borrow(), vec![vec![queue, 0xabc]]);
        // Already queued.
        stub_ret(&mut e, LIST_CONTAINS, 1);
        nav_mesh_obstacle_manager_add_obstacle_for_reference(&mut e, this, 0xabc);
        assert_eq!(added.borrow().len(), 1);
    }

    struct RemoveScene {
        e: Engine,
        this: Ptr<NavMeshObstacleManager>,
        reference: u32,
        from_add_queue: Log,
        rigid_bodies: Log,
        tail: Log,
    }

    /// `RemoveObstacleForReference` with the setting on; `FormIDMap` has an
    /// entry (two obstacles) when `entry` is set.
    fn remove_scene(already_queued: bool, entry: bool) -> RemoveScene {
        let mut e = engine();
        set_setting(&mut e, true);
        let from_add_queue = record_word(&mut e, LIST_REMOVE);
        stub_ret(&mut e, LIST_CONTAINS, already_queued as u32);
        let rigid_bodies = record(&mut e, REMOVE_RIGID_BODY, 0);
        let tail = record_word(&mut e, LIST_ADD_TAIL);
        let first = obstacle(&mut e, 0x501, 0, false, &[]);
        let second = obstacle(&mut e, 0x502, 0, false, &[]);
        let array = reference_array(&mut e, 0x44, &[first, second]);
        e.register_double(MAP_GET, move |e, a| {
            if entry {
                e.mem.set_u32(a[2], array);
            }
            ret(entry as u32)
        });
        let this = manager(&mut e);
        let reference = reference_with_id(&mut e, 0x44);
        RemoveScene {
            e,
            this,
            reference,
            from_add_queue,
            rigid_bodies,
            tail,
        }
    }

    #[test]
    fn removing_a_reference_asks_for_each_rigid_body_and_queues_the_id() {
        let mut s = remove_scene(false, true);
        let base = s.this.addr();
        nav_mesh_obstacle_manager_remove_obstacle_for_reference(&mut s.e, s.this, s.reference);
        assert_eq!(
            *s.from_add_queue.borrow(),
            vec![vec![base + 0x2c, s.reference]]
        );
        assert_eq!(
            *s.rigid_bodies.borrow(),
            vec![vec![base, 0x501], vec![base, 0x502]]
        );
        assert_eq!(*s.tail.borrow(), vec![vec![base + 0x34, 0x44]]);
    }

    #[test]
    fn removing_a_reference_without_an_entry_queues_nothing() {
        let mut s = remove_scene(false, false);
        nav_mesh_obstacle_manager_remove_obstacle_for_reference(&mut s.e, s.this, s.reference);
        assert_eq!(s.from_add_queue.borrow().len(), 1);
        assert!(s.rigid_bodies.borrow().is_empty());
        assert!(s.tail.borrow().is_empty());
    }

    #[test]
    fn removing_a_reference_already_queued_for_removal_stops_early() {
        let mut s = remove_scene(true, true);
        nav_mesh_obstacle_manager_remove_obstacle_for_reference(&mut s.e, s.this, s.reference);
        assert_eq!(s.from_add_queue.borrow().len(), 1);
        assert!(s.rigid_bodies.borrow().is_empty());
        assert!(s.tail.borrow().is_empty());
        // Setting off: not even the add queue is touched.
        let mut s = remove_scene(false, true);
        set_setting(&mut s.e, false);
        nav_mesh_obstacle_manager_remove_obstacle_for_reference(&mut s.e, s.this, s.reference);
        assert!(s.from_add_queue.borrow().is_empty());
    }

    /// The door handlers' scene: a reference of form id `0x44` whose base
    /// form has the given type; the obstacle setting is off (so the
    /// add/remove calls stop at the setting, which is counted).
    struct DoorScene {
        e: Engine,
        this: Ptr<NavMeshObstacleManager>,
        reference: u32,
        setting_reads: Log,
        added: Log,
        removed: Log,
    }

    fn door_scene(
        form_type: u32,
        sliding: bool,
        in_closed: bool,
        in_open: bool,
        contained_offsets: &[u32],
    ) -> DoorScene {
        let mut e = engine();
        let this = manager(&mut e);
        let base = this.addr();
        let reference = reference_with_id(&mut e, 0x44);
        stub_ret(&mut e, REFERENCE_BASE_FORM, 0x3333);
        stub_ret(&mut e, FORM_TYPE, form_type);
        stub_ret(&mut e, IS_SLIDING_DOOR, sliding as u32);
        let value = e.mem.alloc(4);
        let setting_reads = record(&mut e, SETTING_VALUE_ADDRESS, value);
        e.register_double(MAP_GET, move |_, a| {
            ret(((a[0] == base + 0x110 && in_closed) || (a[0] == base + 0x100 && in_open)) as u32)
        });
        let contained: Vec<u32> = contained_offsets
            .iter()
            .map(|offset| base + offset)
            .collect();
        e.register_double(LIST_CONTAINS, move |_, a| {
            ret(contained.contains(&a[0]) as u32)
        });
        let added = record_word(&mut e, LIST_ADD_HEAD);
        let removed = record_word(&mut e, LIST_REMOVE);
        DoorScene {
            e,
            this,
            reference,
            setting_reads,
            added,
            removed,
        }
    }

    #[test]
    fn the_navmesh_queues_take_the_id_at_the_head() {
        let mut e = engine();
        let this = manager(&mut e);
        let base = this.addr();
        let added = record_word(&mut e, LIST_ADD_HEAD);
        fn_006c1130(&mut e, this, 0x51);
        fn_006c1150(&mut e, this, 0x52);
        assert_eq!(
            *added.borrow(),
            vec![vec![base + 0x180, 0x51], vec![base + 0x188, 0x52]]
        );
    }

    #[test]
    fn the_update_request_sets_the_flag() {
        let mut e = engine();
        let this = manager(&mut e);
        assert_eq!(e.mem.u8(this.addr() + 0x18), 0);
        fn_006c1170(&mut e, this);
        assert_eq!(e.mem.u8(this.addr() + 0x18), 1);
    }

    #[test]
    fn the_rigid_body_queues_push_the_value() {
        let mut e = engine();
        let this = manager(&mut e);
        let base = this.addr();
        let pushes = record_word(&mut e, QUEUE_PUSH);
        fn_006c1c70(&mut e, this, 0x501);
        fn_006c1c90(&mut e, this, 0x502);
        assert_eq!(
            *pushes.borrow(),
            vec![vec![base + 0x60, 0x501], vec![base + 0xa0, 0x502]]
        );
    }

    #[test]
    fn the_reference_obstacle_array_constructor_builds_the_base_and_the_array() {
        let mut e = engine();
        let base = record(&mut e, REFERENCE_COUNTED_CTOR, 0);
        let array = record(&mut e, OBSTACLE_ARRAY_CTOR, 0);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 8, 0xdead);
        assert_eq!(
            reference_obstacle_array_construct(&mut e, Ptr::new(this)).addr(),
            this
        );
        assert_eq!(e.mem.u32(this), REFERENCE_OBSTACLE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(*base.borrow(), vec![vec![this]]);
        assert_eq!(*array.borrow(), vec![vec![this + 0x0c]]);
    }

    #[test]
    fn the_reference_obstacle_array_destructor_tears_down_the_array_and_the_base() {
        let mut e = engine();
        let base = record(&mut e, REFERENCE_COUNTED_DTOR, 0);
        let array = record(&mut e, OBSTACLE_ARRAY_DTOR, 0);
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 8, 0xdead);
        reference_obstacle_array_destructor(&mut e, Ptr::new(this));
        assert_eq!(e.mem.u32(this), REFERENCE_OBSTACLE_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(*array.borrow(), vec![vec![this + 0x0c]]);
        assert_eq!(*base.borrow(), vec![vec![this]]);
    }

    #[test]
    fn the_reference_obstacle_array_scalar_deleting_destructor_frees_when_asked() {
        let mut e = engine();
        stub(&mut e, &[REFERENCE_COUNTED_DTOR, OBSTACLE_ARRAY_DTOR]);
        let freed = record(&mut e, OPERATOR_DELETE_SIZED, 0);
        let this = e.mem.alloc(0x1c);
        assert_eq!(
            reference_obstacle_array_scalar_deleting_destructor(&mut e, Ptr::new(this), 0).addr(),
            this
        );
        assert!(freed.borrow().is_empty());
        reference_obstacle_array_scalar_deleting_destructor(&mut e, Ptr::new(this), 1);
        assert_eq!(*freed.borrow(), vec![vec![this, 0x1c]]);
    }

    #[test]
    fn the_havok_registrations_need_an_entity() {
        let mut e = engine();
        let activation = record(&mut e, ENTITY_ADD_ACTIVATION_LISTENER, 0);
        let entity_listener = record(&mut e, ENTITY_ADD_ENTITY_LISTENER, 0);
        stub_ret(&mut e, RIGID_BODY_ENTITY, 0);
        fn_006c1dd0(&mut e, Ptr::new(0x501), 0x111);
        fn_006c1da0(&mut e, Ptr::new(0x501), 0x222);
        assert!(activation.borrow().is_empty());
        assert!(entity_listener.borrow().is_empty());
        stub_ret(&mut e, RIGID_BODY_ENTITY, 0x9000);
        fn_006c1dd0(&mut e, Ptr::new(0x501), 0x111);
        fn_006c1da0(&mut e, Ptr::new(0x501), 0x222);
        assert_eq!(*activation.borrow(), vec![vec![0x9000, 0x111]]);
        assert_eq!(*entity_listener.borrow(), vec![vec![0x9000, 0x222]]);
    }

    #[test]
    fn both_listeners_are_registered_on_a_rigid_body() {
        let mut e = engine();
        let this = manager(&mut e);
        e.mem.set_u32(this.addr() + 0x120, 0x111);
        e.mem.set_u32(this.addr() + 0x124, 0x222);
        let activation = record(&mut e, ENTITY_ADD_ACTIVATION_LISTENER, 0);
        let entity_listener = record(&mut e, ENTITY_ADD_ENTITY_LISTENER, 0);
        let entities = record(&mut e, RIGID_BODY_ENTITY, 0x9000);
        fn_006c1d60(&mut e, this, 0x501);
        assert_eq!(*entities.borrow(), vec![vec![0x501], vec![0x501]]);
        assert_eq!(*activation.borrow(), vec![vec![0x9000, 0x111]]);
        assert_eq!(*entity_listener.borrow(), vec![vec![0x9000, 0x222]]);
    }

    #[test]
    fn on_obstacle_lost_removes_the_reference_the_obstacle_belongs_to() {
        let mut e = engine();
        let this = manager(&mut e);
        let array = reference_array(&mut e, 0x44, &[]);
        let data = obstacle(&mut e, 0x501, array, false, &[]);
        e.register_double(MAP_GET, move |e, a| {
            e.mem.set_u32(a[2], data);
            ret(1)
        });
        let lookups = record(&mut e, FORM_LOOKUP_BY_ID, 0x7777);
        // The setting is off, so RemoveObstacleForReference stops at its
        // first read of it.
        let value = e.mem.alloc(4);
        let reads = record(&mut e, SETTING_VALUE_ADDRESS, value);
        nav_mesh_obstacle_manager_on_obstacle_lost3d(&mut e, this, 0x501);
        assert_eq!(*lookups.borrow(), vec![vec![0x44]]);
        assert_eq!(reads.borrow().len(), 1);
        // The form is gone: nothing more.
        stub_ret(&mut e, FORM_LOOKUP_BY_ID, 0);
        nav_mesh_obstacle_manager_on_obstacle_lost3d(&mut e, this, 0x501);
        assert_eq!(reads.borrow().len(), 1);
        // The obstacle is not known: no form lookup at all.
        e.register_double(MAP_GET, |_, _| ret(0));
        let lookups = record(&mut e, FORM_LOOKUP_BY_ID, 0x7777);
        nav_mesh_obstacle_manager_on_obstacle_lost3d(&mut e, this, 0x501);
        assert!(lookups.borrow().is_empty());
    }

    #[test]
    fn the_first_door_handler_takes_a_waiting_door_off_and_queues_its_id() {
        // The door waits in QueuedListOfDoorsToAdd; it is in neither map.
        let mut s = door_scene(0x1c, false, false, false, &[0xf0]);
        let base = s.this.addr();
        fn_006c0dc0(&mut s.e, s.this, s.reference);
        assert_eq!(s.setting_reads.borrow().len(), 1);
        assert_eq!(*s.removed.borrow(), vec![vec![base + 0xf0, s.reference]]);
        assert_eq!(*s.added.borrow(), vec![vec![base + 0xf8, 0x44]]);
        // Not waiting: only the removal queue changes.
        let mut s = door_scene(0x1c, false, false, false, &[]);
        let base = s.this.addr();
        fn_006c0dc0(&mut s.e, s.this, s.reference);
        assert!(s.removed.borrow().is_empty());
        assert_eq!(*s.added.borrow(), vec![vec![base + 0xf8, 0x44]]);
    }

    #[test]
    fn the_first_door_handler_leaves_doors_in_the_maps_alone() {
        let mut s = door_scene(0x1c, false, true, true, &[0xf0]);
        fn_006c0dc0(&mut s.e, s.this, s.reference);
        assert!(s.removed.borrow().is_empty());
        assert!(s.added.borrow().is_empty());
        // Already queued for removal.
        let mut s = door_scene(0x1c, false, false, false, &[0xf8]);
        fn_006c0dc0(&mut s.e, s.this, s.reference);
        assert!(s.added.borrow().is_empty());
    }

    #[test]
    fn the_first_door_handler_skips_the_obstacle_for_other_forms_and_sliding_doors() {
        let mut s = door_scene(0x1b, false, true, true, &[]);
        fn_006c0dc0(&mut s.e, s.this, s.reference);
        assert!(s.setting_reads.borrow().is_empty());
        let mut s = door_scene(0x1c, true, true, true, &[]);
        fn_006c0dc0(&mut s.e, s.this, s.reference);
        assert!(s.setting_reads.borrow().is_empty());
    }

    #[test]
    fn on_door_close_dequeues_the_removal_and_queues_the_door_for_adding() {
        // The form id is queued for removal; the door is not queued for
        // adding; it is in neither map.
        let mut s = door_scene(0x1c, false, false, false, &[0xf8]);
        let base = s.this.addr();
        nav_mesh_obstacle_manager_on_door_close(&mut s.e, s.this, s.reference);
        assert_eq!(s.setting_reads.borrow().len(), 1);
        assert_eq!(*s.removed.borrow(), vec![vec![base + 0xf8, 0x44]]);
        assert_eq!(*s.added.borrow(), vec![vec![base + 0xf0, s.reference]]);
    }

    #[test]
    fn on_door_close_changes_nothing_for_doors_in_the_maps() {
        let mut s = door_scene(0x1b, false, true, true, &[0xf8, 0xf0]);
        nav_mesh_obstacle_manager_on_door_close(&mut s.e, s.this, s.reference);
        assert!(s.setting_reads.borrow().is_empty());
        assert!(s.removed.borrow().is_empty());
        assert!(s.added.borrow().is_empty());
        // Not in the maps, already queued for adding: nothing is added.
        let mut s = door_scene(0x1b, false, false, false, &[0xf0]);
        nav_mesh_obstacle_manager_on_door_close(&mut s.e, s.this, s.reference);
        assert!(s.added.borrow().is_empty());
    }

    #[test]
    fn the_third_door_handler_forgets_a_door_waiting_to_be_added() {
        let mut s = door_scene(0x1c, false, false, false, &[0xf0]);
        let base = s.this.addr();
        fn_006c1060(&mut s.e, s.this, s.reference);
        assert_eq!(s.setting_reads.borrow().len(), 1);
        assert_eq!(
            *s.removed.borrow(),
            vec![
                vec![base + 0x2c, s.reference],
                vec![base + 0xf0, s.reference]
            ]
        );
        assert!(s.added.borrow().is_empty());
    }

    #[test]
    fn the_third_door_handler_queues_other_doors_for_removal_once() {
        let mut s = door_scene(0x1b, false, false, false, &[]);
        let base = s.this.addr();
        fn_006c1060(&mut s.e, s.this, s.reference);
        assert!(s.setting_reads.borrow().is_empty());
        assert_eq!(*s.removed.borrow(), vec![vec![base + 0x2c, s.reference]]);
        assert_eq!(*s.added.borrow(), vec![vec![base + 0xf8, 0x44]]);
        // Already queued.
        let mut s = door_scene(0x1b, false, false, false, &[0xf8]);
        fn_006c1060(&mut s.e, s.this, s.reference);
        assert!(s.added.borrow().is_empty());
    }

    /// A manager with every callee of the obstacle (re)registration
    /// functions replaced by a recording double.
    struct Scene {
        e: Engine,
        this: Ptr<NavMeshObstacleManager>,
        map_set: Log,
        map_remove: Log,
        queue_push: Log,
        refresh: Log,
        add_node: Log,
        remove_node: Log,
        operations: Log,
        array_add: Log,
        array_clear: Log,
        remove_rigid_body: Log,
        entities: Log,
        list_checks: Log,
    }

    /// `draw` sets `bDrawObstacles`; every rigid body is active; the
    /// overlap query finds one navmesh (`0x8000`, whose info is `0x8100`
    /// and whose task is `0x9100`).
    fn scene(draw: bool) -> Scene {
        let mut e = engine();
        let this = manager(&mut e);
        let base = this.addr();
        e.mem.set_u8(base + 0x198, draw as u8);
        e.mem.set_u32(base + 0x120, 0x111);
        e.mem.set_u32(base + 0x124, 0x222);
        let map_set = record(&mut e, MAP_SET, 0);
        let map_remove = record(&mut e, MAP_REMOVE, 0);
        let queue_push = record_word(&mut e, QUEUE_PUSH);
        let refresh = record_deref(&mut e, OBSTACLE_REFRESH, 0, 0);
        let add_node = record_deref(&mut e, ADD_BOUNDING_BOX_NODE, 0, 1);
        let remove_node = record_deref(&mut e, REMOVE_BOUNDING_BOX_NODE, 0, 1);
        e.register(OBSTACLE_TASK_FOR_NAVMESH, |_, a| ret(a[1] + 0x1000));
        let operations = record_deref(&mut e, OBSTACLE_TASK_ADD_OPERATION, 0, 1);
        let array_add = record_word(&mut e, ARRAY_ADD);
        let array_clear = record(&mut e, ARRAY_CLEAR, 0);
        e.register(
            ARRAY_IS_EMPTY,
            |e, a| ret((e.mem.u32(a[0] + 8) == 0) as u32),
        );
        let remove_rigid_body = record(&mut e, REMOVE_RIGID_BODY, 0);
        let entities = record(&mut e, RIGID_BODY_ENTITY, 0);
        stub_ret(&mut e, RIGID_BODY_IS_ACTIVE, 1);
        stub(
            &mut e,
            &[NAVMESH_ARRAY_CTOR, NAVMESH_ARRAY_DTOR, OBSTACLE_PREPARE],
        );
        e.register(OBSTACLE_OVERLAPPING_NAVMESHES, |e, a| {
            set_array(e, a[1], &[0x8000]);
            ret(1)
        });
        e.register(NAVMESH_INFO, |_, a| ret(a[0] + 0x100));
        e.register(FIRST_WORD_EQUALS, |_, _| ret(0));
        let list_checks = record(&mut e, LIST_IS_EMPTY, 1);
        Scene {
            e,
            this,
            map_set,
            map_remove,
            queue_push,
            refresh,
            add_node,
            remove_node,
            operations,
            array_add,
            array_clear,
            remove_rigid_body,
            entities,
            list_checks,
        }
    }

    /// Makes `MAP_GET` find `array` for any key.
    fn find_array(e: &mut Engine, array: u32) {
        e.register_double(MAP_GET, move |e, a| {
            e.mem.set_u32(a[2], array);
            ret(1)
        });
    }

    #[test]
    fn registering_a_reference_with_an_existing_array_enters_its_obstacles() {
        let mut s = scene(true);
        let base = s.this.addr();
        let o1 = obstacle(&mut s.e, 0x501, 0, true, &[]);
        let array = reference_array(&mut s.e, 0x44, &[o1]);
        find_array(&mut s.e, array);
        let reference = reference_with_id(&mut s.e, 0x44);
        fn_006c1190(&mut s.e, s.this, reference);
        // No new array: the only SetAt is the rigid body map's.
        assert_eq!(*s.map_set.borrow(), vec![vec![base + 0xe0, 0x501, o1]]);
        // Havok listeners on the rigid body, and it is queued (it is active).
        assert_eq!(*s.entities.borrow(), vec![vec![0x501], vec![0x501]]);
        assert_eq!(*s.queue_push.borrow(), vec![vec![base + 0x60, 0x501]]);
        // Refresh and the debug node get the address of the obstacle's pointer.
        let slot = s.refresh.borrow()[0][0];
        assert_eq!(*s.refresh.borrow(), vec![vec![slot, o1]]);
        assert_eq!(*s.add_node.borrow(), vec![vec![base, slot, o1]]);
        // Operation 0x10 on the overlapped navmesh's task, and the navmesh's
        // info recorded in the obstacle.
        assert_eq!(*s.operations.borrow(), vec![vec![0x9100, slot, 0x10, o1]]);
        assert_eq!(*s.array_add.borrow(), vec![vec![o1 + 0x78, 0x8100]]);
    }

    #[test]
    fn registering_a_reference_builds_its_array_when_there_is_none() {
        let mut s = scene(false);
        let base = s.this.addr();
        let o1 = obstacle(&mut s.e, 0x501, 0, false, &[]);
        let block = s.e.mem.alloc(0x1c);
        let allocated = record(&mut s.e, OPERATOR_NEW_SIZED, block);
        stub(&mut s.e, &[REFERENCE_COUNTED_CTOR, OBSTACLE_ARRAY_CTOR]);
        s.e.register_double(FILL_REFERENCE_OBSTACLES, move |e, a| {
            let array = e.mem.u32(a[1]);
            set_array(e, array + 0x0c, &[o1]);
            Ret::default()
        });
        stub_ret(&mut s.e, MAP_GET, 0);
        let reference = reference_with_id(&mut s.e, 0x44);
        fn_006c1190(&mut s.e, s.this, reference);
        assert_eq!(*allocated.borrow(), vec![vec![0x1c]]);
        assert_eq!(s.e.mem.u32(block), REFERENCE_OBSTACLE_ARRAY_VTABLE);
        assert_eq!(
            *s.map_set.borrow(),
            vec![vec![base + 0x1c, 0x44, block], vec![base + 0xe0, 0x501, o1]]
        );
        // Not active, no drawing: no navmesh work.
        assert!(s.operations.borrow().is_empty());
        assert!(s.add_node.borrow().is_empty());
        assert_eq!(s.refresh.borrow().len(), 1);
    }

    #[test]
    fn unregistering_a_reference_by_id_removes_its_obstacles() {
        let mut s = scene(true);
        let base = s.this.addr();
        let o1 = obstacle(&mut s.e, 0x501, 0, true, &[0x6001]);
        let o2 = obstacle(&mut s.e, 0x502, 0, true, &[]);
        let array = reference_array(&mut s.e, 0x44, &[o1, o2]);
        find_array(&mut s.e, array);
        fn_006c1590(&mut s.e, s.this, 0x44);
        assert_eq!(
            *s.map_remove.borrow(),
            vec![
                vec![base + 0x1c, 0x44],
                vec![base + 0xe0, 0x501],
                vec![base + 0xe0, 0x502],
            ]
        );
        // The update list is asked to drop each obstacle (it is empty here).
        assert_eq!(
            *s.list_checks.borrow(),
            vec![vec![base + 0x3c], vec![base + 0x3c]]
        );
        assert_eq!(s.remove_node.borrow().len(), 2);
        // Only the obstacle with recorded navmeshes queues operation 0x20 and
        // is cleared.
        let slot = s.remove_node.borrow()[0][1];
        assert_eq!(*s.operations.borrow(), vec![vec![0x7001, slot, 0x20, o1]]);
        assert_eq!(*s.array_clear.borrow(), vec![vec![o1 + 0x78, 1]]);
    }

    #[test]
    fn unregistering_an_unknown_reference_does_nothing() {
        let mut s = scene(true);
        stub_ret(&mut s.e, MAP_GET, 0);
        fn_006c1590(&mut s.e, s.this, 0x44);
        assert!(s.map_remove.borrow().is_empty());
        assert!(s.operations.borrow().is_empty());
    }

    #[test]
    fn re_registering_an_active_obstacle_cancels_and_redoes_its_navmesh_work() {
        let mut s = scene(true);
        let base = s.this.addr();
        let o1 = obstacle(&mut s.e, 0x501, 0, true, &[0x6001]);
        let slot = slot_with(&mut s.e, o1);
        let prepare = record(&mut s.e, OBSTACLE_PREPARE, 0);
        fn_006c1760(&mut s.e, s.this, slot);
        assert_eq!(
            *s.operations.borrow(),
            vec![vec![0x7001, slot, 0x21, o1], vec![0x9100, slot, 0x11, o1],]
        );
        assert_eq!(*s.array_clear.borrow(), vec![vec![o1 + 0x78, 1]]);
        assert_eq!(*prepare.borrow(), vec![vec![o1]]);
        assert_eq!(*s.refresh.borrow(), vec![vec![slot, o1]]);
        assert_eq!(*s.add_node.borrow(), vec![vec![base, slot, o1]]);
        assert_eq!(*s.array_add.borrow(), vec![vec![o1 + 0x78, 0x8100]]);
    }

    #[test]
    fn re_registering_an_inactive_obstacle_only_refreshes_it() {
        let mut s = scene(false);
        let o1 = obstacle(&mut s.e, 0x501, 0, false, &[0x6001]);
        let slot = slot_with(&mut s.e, o1);
        let prepare = record(&mut s.e, OBSTACLE_PREPARE, 0);
        fn_006c1760(&mut s.e, s.this, slot);
        assert!(s.operations.borrow().is_empty());
        assert!(s.array_clear.borrow().is_empty());
        assert!(s.add_node.borrow().is_empty());
        assert_eq!(*prepare.borrow(), vec![vec![o1]]);
        assert_eq!(*s.refresh.borrow(), vec![vec![slot, o1]]);
    }

    #[test]
    fn replacing_the_obstacles_of_a_reference_removes_the_old_and_registers_the_new() {
        let mut s = scene(true);
        let base = s.this.addr();
        let o1 = obstacle(&mut s.e, 0x501, 0, true, &[0x6001]);
        let o2 = obstacle(&mut s.e, 0x502, 0, false, &[]);
        let array = reference_array(&mut s.e, 0x44, &[o1, o2]);
        find_array(&mut s.e, array);
        let n1 = obstacle(&mut s.e, 0x503, 0, false, &[]);
        let refill = record(&mut s.e, OBSTACLE_ARRAY_REFRESH, 0);
        s.e.register_double(OBSTACLE_ARRAY_REFRESH, move |e, a| {
            let array = e.mem.u32(a[1]);
            set_array(e, array + 0x0c, &[n1]);
            Ret::default()
        });
        let _ = refill;
        fn_006c1920(&mut s.e, s.this, 0x44, 0xabcd);
        // Old obstacles: navmesh work of the active one, debug nodes, rigid
        // bodies, map entries, update list.
        assert_eq!(s.operations.borrow().len(), 1);
        assert_eq!(s.operations.borrow()[0][2..], [0x21, o1]);
        assert_eq!(s.operations.borrow()[0][0], 0x7001);
        assert_eq!(*s.array_clear.borrow(), vec![vec![o1 + 0x78, 1]]);
        assert_eq!(s.remove_node.borrow().len(), 2);
        assert_eq!(
            *s.remove_rigid_body.borrow(),
            vec![vec![base, 0x501], vec![base, 0x502]]
        );
        assert_eq!(
            *s.map_remove.borrow(),
            vec![vec![base + 0xe0, 0x501], vec![base + 0xe0, 0x502]]
        );
        assert_eq!(s.list_checks.borrow().len(), 2);
        // New obstacle: refreshed, drawn, its rigid body gets the listeners.
        assert_eq!(s.refresh.borrow().len(), 1);
        assert_eq!(s.refresh.borrow()[0][1], n1);
        assert_eq!(s.add_node.borrow().len(), 1);
        assert_eq!(*s.entities.borrow(), vec![vec![0x503], vec![0x503]]);
    }

    #[test]
    fn replacing_the_obstacles_of_an_unknown_reference_does_nothing() {
        let mut s = scene(true);
        stub_ret(&mut s.e, MAP_GET, 0);
        let refill = record(&mut s.e, OBSTACLE_ARRAY_REFRESH, 0);
        fn_006c1920(&mut s.e, s.this, 0x44, 0xabcd);
        assert!(refill.borrow().is_empty());
        assert!(s.remove_rigid_body.borrow().is_empty());
    }

    // ---------------------------------------------------------------------
    // Session 2 (`006c1e00`..`006c5b20`)

    /// Maps the pages of the `.rdata` constants these functions read and
    /// gives them the exe's values.
    fn constants(e: &mut Engine) {
        for page in [
            0x0101_2000u32,
            0x0101_5000,
            0x0101_6000,
            0x0101_7000,
            0x0102_e000,
            0x0106_c000,
            0x011a_9000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global::<f32>(NEGATIVE_FLOAT_MAX, f32::MIN);
        e.set_global::<f32>(POSITIVE_FLOAT_MAX, f32::MAX);
        e.set_global::<f32>(CELL_SIZE_FLOAT, 4096.0);
        e.set_global::<f64>(CELL_SIZE_DOUBLE, 4096.0);
        e.set_global::<f64>(NAVMESH_TOP_MARGIN_DOUBLE, 128.0);
        e.set_global::<f64>(ZERO_DOUBLE, 0.0);
        e.set_global::<f64>(MILLISECONDS_PER_SECOND_DOUBLE, 1000.0);
        e.set_global::<f32>(PORTAL_SQUARED_DISTANCE, 2500.0);
        e.set_global::<f32>(OBSTACLE_MIN_HEIGHT, 48.0);
        e.set_global::<f32>(OBSTACLE_MIN_WIDTH, 16.0);
        e.set_global::<f32>(OBSTACLE_MOVED_SQUARED_DISTANCE, 25.0);
        e.set_global::<f32>(BOX_EDGE_MARGIN, 15.0);
        e.set_global::<f32>(DEBUG_BOX_ALPHA, 0.25);
        e.set_global::<f32>(DEBUG_BOX_COLOR_ALPHA, 0.5);
        e.set_global::<f32>(UP_VECTOR + 8, 1.0);
        e.set_global::<f32>(INITIAL_TIME_TO_NEXT_SWAP, 0.5);
    }

    fn float_ret(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    fn point_at(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn set_point(e: &mut Engine, at: u32, point: [f32; 3]) {
        for (index, value) in point.iter().enumerate() {
            e.mem.set_f32(at + 4 * index as u32, *value);
        }
    }

    /// `BSSimpleArray` doubles that work on game memory: `Add` appends the
    /// word at the argument's address (allocating the buffer on first use)
    /// and `Clear` empties the array.
    fn working_arrays(e: &mut Engine) {
        e.register(ARRAY_ADD, |e, a| {
            let mut buffer = e.mem.u32(a[0] + 4);
            if buffer == 0 {
                buffer = e.mem.alloc(0x100);
                e.mem.set_u32(a[0] + 4, buffer);
            }
            let size = e.mem.u32(a[0] + 8);
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            ret(size)
        });
        e.register(ARRAY_CLEAR, |e, a| {
            e.mem.set_u32(a[0] + 8, 0);
            Ret::default()
        });
    }

    /// The elements of a `BSSimpleArray`.
    fn array_elements(e: &Engine, array: u32) -> Vec<u32> {
        let buffer = e.mem.u32(array + 4);
        (0..e.mem.u32(array + 8))
            .map(|index| e.mem.u32(buffer + 4 * index))
            .collect()
    }

    /// The `NiPoint3` / `NiMatrix3` helpers as plain math on game memory
    /// (matrices are nine floats, row-major; the column accessors take the
    /// index first).
    fn math_engine() -> Engine {
        let mut e = engine();
        add_math(&mut e);
        e
    }

    /// Registers the doubles of [`math_engine`] (and the constants) on `e`.
    fn add_math(e: &mut Engine) {
        constants(e);
        e.register(POINT3_DEFAULT, |_, a| ret(a[0]));
        e.register(POINT3_CONSTRUCT, |e, a| {
            for index in 0..3 {
                e.mem.set_u32(a[0] + 4 * index, a[1 + index as usize]);
            }
            ret(a[0])
        });
        e.register(POINT3_SUBTRACT, |e, a| {
            for index in 0..3 {
                let value = e.mem.f32(a[0] + 4 * index) - e.mem.f32(a[2] + 4 * index);
                e.mem.set_f32(a[1] + 4 * index, value);
            }
            ret(a[1])
        });
        e.register(POINT3_ADD, |e, a| {
            for index in 0..3 {
                let value = e.mem.f32(a[0] + 4 * index) + e.mem.f32(a[2] + 4 * index);
                e.mem.set_f32(a[1] + 4 * index, value);
            }
            ret(a[1])
        });
        e.register(POINT3_NEGATE, |e, a| {
            for index in 0..3 {
                let value = -e.mem.f32(a[0] + 4 * index);
                e.mem.set_f32(a[1] + 4 * index, value);
            }
            ret(a[1])
        });
        e.register(POINT3_SQUARED_LENGTH, |e, a| {
            let p = point_at(e, a[0]);
            float_ret(p[0] * p[0] + p[1] * p[1] + p[2] * p[2])
        });
        e.register(POINT3_SQUARED_LENGTH_XY, |e, a| {
            let p = point_at(e, a[0]);
            float_ret(p[0] * p[0] + p[1] * p[1])
        });
        e.register(POINT3_CROSS, |e, a| {
            let (p, q) = (point_at(e, a[0]), point_at(e, a[2]));
            set_point(
                e,
                a[1],
                [
                    p[1] * q[2] - p[2] * q[1],
                    p[2] * q[0] - p[0] * q[2],
                    p[0] * q[1] - p[1] * q[0],
                ],
            );
            ret(a[1])
        });
        e.register(POINT3_UNIT_CROSS, |e, a| {
            let (p, q) = (point_at(e, a[0]), point_at(e, a[2]));
            let c = [
                p[1] * q[2] - p[2] * q[1],
                p[2] * q[0] - p[0] * q[2],
                p[0] * q[1] - p[1] * q[0],
            ];
            let length = (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();
            set_point(e, a[1], [c[0] / length, c[1] / length, c[2] / length]);
            ret(a[1])
        });
        e.register(MATRIX_TIMES_POINT, |e, a| {
            let v = point_at(e, a[2]);
            let mut out = [0.0; 3];
            for (row, slot) in out.iter_mut().enumerate() {
                for (column, value) in v.iter().enumerate() {
                    *slot += e.mem.f32(a[0] + 4 * (3 * row + column) as u32) * value;
                }
            }
            set_point(e, a[1], out);
            ret(a[1])
        });
        e.register(MATRIX_TRANSPOSE, |e, a| {
            for row in 0..3 {
                for column in 0..3 {
                    let value = e.mem.f32(a[0] + 4 * (3 * row + column));
                    e.mem.set_f32(a[1] + 4 * (3 * column + row), value);
                }
            }
            ret(a[1])
        });
        e.register(MATRIX_TIMES_MATRIX, |e, a| {
            for row in 0..3 {
                for column in 0..3 {
                    let mut sum = 0.0;
                    for k in 0..3 {
                        sum += e.mem.f32(a[0] + 4 * (3 * row + k))
                            * e.mem.f32(a[2] + 4 * (3 * k + column));
                    }
                    e.mem.set_f32(a[1] + 4 * (3 * row + column), sum);
                }
            }
            ret(a[1])
        });
        e.register(MATRIX_SET_COLUMN, |e, a| {
            for row in 0..3 {
                let value = e.mem.f32(a[2] + 4 * row);
                e.mem.set_f32(a[0] + 4 * (3 * row + a[1]), value);
            }
            Ret::default()
        });
        e.register(MATRIX_GET_COLUMN, |e, a| {
            for row in 0..3 {
                let value = e.mem.f32(a[0] + 4 * (3 * row + a[1]));
                e.mem.set_f32(a[2] + 4 * row, value);
            }
            Ret::default()
        });
        e.register(FLOAT_ABS, |_, a| float_ret(f32::from_bits(a[0]).abs()));
        e.register(FLOAT_REMAINDER, |_, a| {
            float_ret(f32::from_bits(a[0]) % f32::from_bits(a[1]))
        });
        e.register(FLOAT_MIN, |_, a| {
            let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            float_ret(if second <= first { second } else { first })
        });
        e.register(FLOAT_MAX, |_, a| {
            let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            float_ret(if second < first { first } else { second })
        });
    }

    #[test]
    fn the_listener_removal_asks_each_registration_for_the_entity() {
        let mut e = engine();
        let this = manager(&mut e);
        e.mem.set_u32(this.addr() + 0x120, 0x111);
        e.mem.set_u32(this.addr() + 0x124, 0x222);
        stub_ret(&mut e, RIGID_BODY_ENTITY, 0x7000);
        let activation = record(&mut e, ENTITY_REMOVE_ACTIVATION_LISTENER, 0);
        let entity_listener = record(&mut e, ENTITY_REMOVE_ENTITY_LISTENER, 0);
        fn_006c1e00(&mut e, this, 0x500);
        assert_eq!(*activation.borrow(), vec![vec![0x7000, 0x111]]);
        assert_eq!(*entity_listener.borrow(), vec![vec![0x7000, 0x222]]);
    }

    #[test]
    fn the_havok_removals_need_an_entity() {
        let mut e = engine();
        let activation = record(&mut e, ENTITY_REMOVE_ACTIVATION_LISTENER, 0);
        let entity_listener = record(&mut e, ENTITY_REMOVE_ENTITY_LISTENER, 0);
        stub_ret(&mut e, RIGID_BODY_ENTITY, 0);
        fn_006c1e70(&mut e, Ptr::new(0x500), 0x111);
        fn_006c1e40(&mut e, Ptr::new(0x500), 0x222);
        assert!(activation.borrow().is_empty() && entity_listener.borrow().is_empty());
        stub_ret(&mut e, RIGID_BODY_ENTITY, 0x7000);
        fn_006c1e70(&mut e, Ptr::new(0x500), 0x111);
        assert_eq!(*activation.borrow(), vec![vec![0x7000, 0x111]]);
        assert!(entity_listener.borrow().is_empty());
        fn_006c1e40(&mut e, Ptr::new(0x500), 0x222);
        assert_eq!(*entity_listener.borrow(), vec![vec![0x7000, 0x222]]);
    }

    /// The cell queries of `GetCellListForObstacle`: the current cell is
    /// `0x8000` (an interior when `interior`); the obstacle's center is
    /// `(5000, -3000)`, so its cell is `(1, -1)`; the world space `0x9000`
    /// has cells only at `(1, -1)` (`0xc001`, in state six) and `(2, -1)`
    /// (`0xc002`, not in state six) .
    struct CellScene {
        e: Engine,
        obstacle: u32,
        cells: u32,
        overlaps: Rc<RefCell<Vec<[f32; 8]>>>,
    }

    fn cell_scene(interior: bool, current: u32) -> CellScene {
        let mut e = math_engine();
        working_arrays(&mut e);
        e.set_global::<u32>(TES_POINTER, 0x7000);
        stub_ret(&mut e, TES_GET_CURRENT_CELL, current);
        stub_ret(&mut e, CELL_IS_INTERIOR, interior as u32);
        stub_ret(&mut e, CELL_GET_WORLD_SPACE, 0x9000);
        e.register(WORLD_SPACE_GET_CELL_FROM_CELL_COORD, |_, a| {
            ret(match (a[1] as i32, a[2] as i32) {
                (1, -1) => 0xc001,
                (2, -1) => 0xc002,
                _ => 0,
            })
        });
        e.register(CELL_IS_IN_STATE_SIX, |_, a| ret((a[0] == 0xc001) as u32));
        let data = obstacle(&mut e, 0x501, 0, true, &[]);
        set_point(&mut e, data + 0x10, [5000.0, -3000.0, 0.0]);
        let slot = slot_with(&mut e, data);
        let cells = e.mem.alloc(0x14);
        let overlaps: Rc<RefCell<Vec<[f32; 8]>>> = Rc::default();
        let shared = overlaps.clone();
        e.register_double(BOX_OVERLAP, move |e, a| {
            let (low, high) = (point_at(e, a[0]), point_at(e, a[1]));
            shared.borrow_mut().push([
                low[0],
                low[1],
                low[2],
                high[0],
                high[1],
                high[2],
                (a[2] - 0x58) as f32,
                (a[3] - 0x64) as f32,
            ]);
            ret(1)
        });
        CellScene {
            e,
            obstacle: slot,
            cells,
            overlaps,
        }
    }

    #[test]
    fn the_cell_list_is_empty_and_false_without_a_current_cell() {
        let mut s = cell_scene(false, 0);
        let clears = record(&mut s.e, ARRAY_CLEAR, 0);
        assert!(!nav_mesh_obstacle_manager_get_cell_list_for_obstacle(
            &mut s.e, s.obstacle, s.cells
        ));
        assert_eq!(*clears.borrow(), vec![vec![s.cells, 0]]);
    }

    #[test]
    fn the_cell_list_of_an_interior_is_the_current_cell() {
        let mut s = cell_scene(true, 0x8000);
        assert!(nav_mesh_obstacle_manager_get_cell_list_for_obstacle(
            &mut s.e, s.obstacle, s.cells
        ));
        assert_eq!(array_elements(&s.e, s.cells), vec![0x8000]);
        assert!(s.overlaps.borrow().is_empty());
    }

    #[test]
    fn the_cell_list_outside_takes_the_overlapping_cells_around_the_obstacle() {
        let mut s = cell_scene(false, 0x8000);
        assert!(nav_mesh_obstacle_manager_get_cell_list_for_obstacle(
            &mut s.e, s.obstacle, s.cells
        ));
        // Only cell (1, -1) exists in state six; the box test saw its square
        // and the obstacle's bounds (`+0x58`, `+0x64` of the data).
        let data = s.e.mem.u32(s.obstacle);
        assert_eq!(array_elements(&s.e, s.cells), vec![0xc001]);
        assert_eq!(
            *s.overlaps.borrow(),
            vec![[
                4096.0,
                -4096.0,
                f32::MIN,
                8192.0,
                0.0,
                f32::MAX,
                data as f32,
                data as f32
            ]]
        );
    }

    #[test]
    fn the_cell_list_skips_a_cell_whose_box_does_not_overlap() {
        let mut s = cell_scene(false, 0x8000);
        stub_ret(&mut s.e, BOX_OVERLAP, 0);
        assert!(nav_mesh_obstacle_manager_get_cell_list_for_obstacle(
            &mut s.e, s.obstacle, s.cells
        ));
        assert!(array_elements(&s.e, s.cells).is_empty());
    }

    #[test]
    fn the_overlapping_navmeshes_are_those_of_the_cells_that_pass_the_box_test() {
        // One interior cell with two navmeshes; only the second overlaps.
        let mut s = cell_scene(true, 0x8000);
        let navmeshes = s.e.mem.alloc(0x10);
        set_array(&mut s.e, navmeshes, &[0xa000, 0xa001]);
        s.e.register(GET_WORD_AT_8, |e, a| ret(e.mem.u32(a[0] + 8)));
        stub_ret(&mut s.e, CELL_NAVMESHES, navmeshes);
        stub(
            &mut s.e,
            &[CELL_ARRAY_CTOR, CELL_ARRAY_DTOR, NAVMESH_HOLDER_DTOR],
        );
        s.e.register(NAVMESH_ARRAY_GET_BY_INDEX, |e, a| {
            e.mem.set_u32(a[1], 0xa000 + a[2]);
            ret(a[1])
        });
        s.e.register(NAVMESH_BOUNDS, |e, a| {
            set_point(e, a[1], [1.0, 2.0, 3.0]);
            set_point(e, a[2], [4.0, 5.0, 10.0]);
            ret(1)
        });
        let tops = Rc::new(RefCell::new(vec![]));
        let shared = tops.clone();
        s.e.register_double(BOX_OVERLAP, move |e, a| {
            shared.borrow_mut().push(point_at(e, a[1])[2]);
            ret((shared.borrow().len() == 2) as u32)
        });
        let added = record_word(&mut s.e, NAVMESH_HOLDER_ARRAY_ADD);
        let found = s.e.mem.alloc(0x10);
        assert!(fn_006c2050(&mut s.e, s.obstacle, found));
        // The top of the navmesh's bounds is raised by 128 for the test.
        assert_eq!(*tops.borrow(), vec![138.0, 138.0]);
        assert_eq!(*added.borrow(), vec![vec![found, 0xa001]]);
    }

    /// An object whose vtable has, at each given slot offset, a function
    /// returning the given value (the function's key is `object + slot`, so a
    /// test can replace it with `register_double`).
    fn object_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x200);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        for (slot, value) in slots {
            let function = object + slot;
            let value = *value;
            e.register_double(function, move |_, _| ret(value));
            e.mem.set_u32(vtable + slot, function);
        }
        object
    }

    #[test]
    fn the_obstacle_data_constructor_builds_the_members() {
        let mut e = engine();
        let data = e.mem.alloc(0x8c);
        let base = record(&mut e, REFERENCE_COUNTED_CTOR, 0);
        let points = record(&mut e, POINT3_DEFAULT, 0);
        let navmeshes = record(&mut e, OBSTACLE_NAVMESH_ARRAY_CTOR, 0);
        assert_eq!(fn_006c23c0(&mut e, Ptr::new(data)), Ptr::new(data));
        assert_eq!(*base.borrow(), vec![vec![data]]);
        assert_eq!(e.mem.u32(data), OBSTACLE_DATA_VTABLE);
        assert_eq!(e.mem.u32(data + 0x0c), 0);
        assert_eq!(e.mem.u32(data + 0x88), 0);
        assert_eq!(e.mem.u8(data + 0x74), 1);
        let expected: Vec<Vec<u32>> = [0x10, 0x1c, 0x40, 0x4c, 0x58, 0x64]
            .iter()
            .map(|offset| vec![data + offset])
            .collect();
        assert_eq!(*points.borrow(), expected);
        assert_eq!(*navmeshes.borrow(), vec![vec![data + 0x78]]);
    }

    #[test]
    fn the_obstacle_data_destructor_releases_then_destroys_in_order() {
        let mut e = engine();
        stub(
            &mut e,
            &[OBSTACLE_NAVMESH_ARRAY_DTOR, REFERENCE_COUNTED_DTOR],
        );
        start_log(&mut e);
        fn_006c24d0(&mut e, Ptr::new(0x1000));
        assert_eq!(
            calls(
                &e,
                &[
                    NI_POINTER_RELEASE,
                    OBSTACLE_NAVMESH_ARRAY_DTOR,
                    REFERENCE_COUNTED_DTOR
                ]
            ),
            vec![
                (NI_POINTER_RELEASE, vec![0x1088]),
                (OBSTACLE_NAVMESH_ARRAY_DTOR, vec![0x1078]),
                (NI_POINTER_RELEASE, vec![0x100c]),
                (REFERENCE_COUNTED_DTOR, vec![0x1000]),
            ]
        );
    }

    #[test]
    fn the_obstacle_data_deleting_destructor_frees_only_when_asked() {
        let mut e = engine();
        let destroyed = record(&mut e, REFERENCE_COUNTED_DTOR, 0);
        stub(&mut e, &[OBSTACLE_NAVMESH_ARRAY_DTOR]);
        let freed = record(&mut e, OPERATOR_DELETE_SIZED, 0);
        let this = Ptr::new(0x1000);
        assert_eq!(
            obstacle_data_scalar_deleting_destructor(&mut e, this, 0),
            this
        );
        assert!(freed.borrow().is_empty());
        assert_eq!(
            obstacle_data_scalar_deleting_destructor(&mut e, this, 1),
            this
        );
        assert_eq!(*freed.borrow(), vec![vec![0x1000, 0x8c]]);
        assert_eq!(destroyed.borrow().len(), 2);
    }

    /// A scene tree for the node walks: the collision object of `0x1000`
    /// is `0xc100`, whose rigid body `0xb100` has motion type `kind`.
    struct NodeScene {
        e: Engine,
        root: u32,
        child: u32,
        array: u32,
        visited: Log,
    }

    fn node_scene(kind: u32, child_has_the_rigid_body: bool) -> NodeScene {
        let mut e = engine();
        let container = e.mem.alloc(0x10);
        let child = object_with_slots(&mut e, &[(NODE_CHILDREN_SLOT, 0)]);
        let root = object_with_slots(&mut e, &[(NODE_CHILDREN_SLOT, container)]);
        let with_rigid_body = if child_has_the_rigid_body {
            child
        } else {
            root
        };
        let visited: Log = Rc::default();
        let shared = visited.clone();
        e.register_double(COLLISION_OBJECT_OF_NODE, move |_, a| {
            shared.borrow_mut().push(a.to_vec());
            ret(if a[0] == with_rigid_body { 0xc100 } else { 0 })
        });
        e.register(COLLISION_OBJECT_RIGID_BODY, |_, a| {
            assert_eq!(a[0], 0xc100);
            ret(0xb100)
        });
        e.register_double(RIGID_BODY_MOTION_INFO, move |e, a| {
            e.mem.set_u32(a[1], kind);
            ret(a[1])
        });
        e.register(MOTION_INFO_TYPE, |e, a| ret(e.mem.u32(a[0]) & 0x7f));
        // The tree is [root, child]; `006c2650` stops at once (no Havok
        // object).
        stub_ret(&mut e, GET_WORD_AT_8, 0);
        stub_ret(&mut e, CHILDREN_COUNT, 2);
        stub_ret(&mut e, CHILDREN_COUNT_SHALLOW, 2);
        e.register_double(CHILDREN_AT, move |_, a| {
            ret(match a[1] {
                0 => child,
                _ => 0,
            })
        });
        let array = reference_array(&mut e, 0x44, &[]);
        NodeScene {
            e,
            root,
            child,
            array,
            visited,
        }
    }

    #[test]
    fn building_obstacles_adds_one_for_the_node_with_a_usable_rigid_body() {
        let mut s = node_scene(4, false);
        let slot = slot_with(&mut s.e, s.array);
        let block = s.e.mem.alloc(0x8c);
        let allocated = record(&mut s.e, OPERATOR_NEW_SIZED, block);
        stub(
            &mut s.e,
            &[
                REFERENCE_COUNTED_CTOR,
                POINT3_DEFAULT,
                OBSTACLE_NAVMESH_ARRAY_CTOR,
            ],
        );
        let added = record_word(&mut s.e, OBSTACLE_ARRAY_ADD);
        assert!(fn_006c2210(&mut s.e, s.root, slot));
        assert_eq!(*allocated.borrow(), vec![vec![0x8c]]);
        // The new data points back at the array and holds the rigid body.
        assert_eq!(s.e.mem.u32(block), OBSTACLE_DATA_VTABLE);
        assert_eq!(s.e.mem.u32(block + 8), s.array);
        assert_eq!(s.e.mem.u32(block + 0x0c), 0xb100);
        assert_eq!(*added.borrow(), vec![vec![s.array + 0x0c, block]]);
        // The children were visited too (the second one is null).
        let visited: Vec<u32> = s.visited.borrow().iter().map(|call| call[0]).collect();
        assert_eq!(visited, vec![s.root, s.child]);
    }

    #[test]
    fn building_obstacles_ignores_other_motion_types_and_finds_the_children() {
        let mut s = node_scene(3, true);
        let slot = slot_with(&mut s.e, s.array);
        let allocated = record(&mut s.e, OPERATOR_NEW_SIZED, 0);
        assert!(fn_006c2210(&mut s.e, s.root, slot));
        assert!(allocated.borrow().is_empty());
        // A usable motion type on the child makes one there.
        let mut s = node_scene(0x1c, true);
        let slot = slot_with(&mut s.e, s.array);
        let block = s.e.mem.alloc(0x8c);
        stub_ret(&mut s.e, OPERATOR_NEW_SIZED, block);
        stub(
            &mut s.e,
            &[
                REFERENCE_COUNTED_CTOR,
                POINT3_DEFAULT,
                OBSTACLE_NAVMESH_ARRAY_CTOR,
            ],
        );
        let added = record_word(&mut s.e, OBSTACLE_ARRAY_ADD);
        assert!(fn_006c2210(&mut s.e, s.root, slot));
        assert_eq!(*added.borrow(), vec![vec![s.array + 0x0c, block]]);
        let _ = s.child;
    }

    #[test]
    fn filling_one_obstacle_takes_the_first_node_with_a_usable_rigid_body() {
        let mut s = node_scene(1, true);
        let data = obstacle(&mut s.e, 0, 0xdead, true, &[]);
        assert!(fn_006c2550(&mut s.e, s.root, data));
        assert_eq!(s.e.mem.u32(data + 8), 0);
        assert_eq!(s.e.mem.u32(data + 0x0c), 0xb100);
        // Motion type 3 is not usable, so nothing qualifies.
        let mut s = node_scene(3, true);
        let data = obstacle(&mut s.e, 0, 0xdead, true, &[]);
        assert!(!fn_006c2550(&mut s.e, s.root, data));
        assert_eq!(s.e.mem.u32(data + 8), 0xdead);
        assert_eq!(s.e.mem.u32(data + 0x0c), 0);
    }

    #[test]
    fn filling_one_obstacle_on_the_node_itself_does_not_look_at_the_children() {
        let mut s = node_scene(10, false);
        let data = obstacle(&mut s.e, 0, 0xdead, true, &[]);
        assert!(fn_006c2550(&mut s.e, s.root, data));
        assert_eq!(s.e.mem.u32(data + 0x0c), 0xb100);
        let visited: Vec<u32> = s.visited.borrow().iter().map(|call| call[0]).collect();
        assert_eq!(visited, vec![s.root]);
    }

    /// `006c2650` with a rigid body whose rotation has the given columns,
    /// translation and local aabb; the obstacle data is returned.
    fn prepare_scene(
        columns: [[f32; 3]; 3],
        translation: [f32; 3],
        lower: [f32; 3],
        upper: [f32; 3],
    ) -> (Engine, u32) {
        let mut e = math_engine();
        let data = obstacle(&mut e, 0xb100, 0, false, &[]);
        stub_ret(&mut e, GET_WORD_AT_8, 0x7700);
        stub(
            &mut e,
            &[
                HAVOK_TRANSFORM_SOURCE,
                HAVOK_TRANSFORM_BUILD,
                AABB_DEFAULT,
                MATRIX_TO_EULER_ANGLES,
                VECTOR_CONSTRUCTOR_ITERATOR,
            ],
        );
        e.register(TRANSFORM_DEFAULT, |_, a| ret(a[0]));
        e.register_double(TRANSFORM_FROM_HAVOK, move |e, a| {
            for (column, values) in columns.iter().enumerate() {
                for (row, value) in values.iter().enumerate() {
                    e.mem.set_f32(a[0] + 4 * (3 * row + column) as u32, *value);
                }
            }
            set_point(e, a[0] + 0x24, translation);
            ret(a[0])
        });
        e.register_double(RIGID_BODY_GET_AABB_LOCAL, move |e, a| {
            assert_eq!(a[0], 0xb100);
            set_point(e, a[1], lower);
            set_point(e, a[1] + 0x10, upper);
            Ret::default()
        });
        e.register(HK_VECTOR_TO_POINT3, |e, a| {
            let point = point_at(e, a[1]);
            set_point(e, a[0], point);
            Ret::default()
        });
        stub_ret(&mut e, CLOCK_MILLISECONDS, 0x1234);
        (e, data)
    }

    fn matrix_at(e: &Engine, at: u32) -> [f32; 9] {
        let mut values = [0.0; 9];
        for (index, value) in values.iter_mut().enumerate() {
            *value = e.mem.f32(at + 4 * index as u32);
        }
        values
    }

    #[test]
    fn preparing_an_obstacle_gives_its_box_and_world_bounds() {
        let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let (mut e, data) = prepare_scene(
            identity,
            [12293.0, 200.0, 300.0],
            [-1.0, -2.0, -3.0],
            [1.0, 2.0, 3.0],
        );
        assert!(fn_006c2650(&mut e, data));
        assert_eq!(point_at(&e, data + 0x10), [12293.0, 200.0, 300.0]);
        // Columns (c0, c1, c2) = identity: the largest |z| is c2, so
        // u = c2, v = c0, w = c1; v = -(w x u) = (-1, 0, 0).
        assert_eq!(
            matrix_at(&e, data + 0x1c),
            [-1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
        // The x of the world corners (12292, 12294) is within 15 of a cell
        // border, so both move by 30 and the local x follows (mirrored).
        assert_eq!(point_at(&e, data + 0x40), [-31.0, -2.0, -3.0]);
        assert_eq!(point_at(&e, data + 0x4c), [-29.0, 2.0, 3.0]);
        assert_eq!(point_at(&e, data + 0x58), [12322.0, 198.0, 297.0]);
        assert_eq!(point_at(&e, data + 0x64), [12324.0, 202.0, 303.0]);
        assert_eq!(e.mem.u32(data + 0x70), 0x1234);
    }

    #[test]
    fn preparing_an_obstacle_away_from_cell_borders_moves_nothing() {
        let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let (mut e, data) = prepare_scene(
            identity,
            [2000.0, 3000.0, 100.0],
            [-1.0, -2.0, -3.0],
            [1.0, 2.0, 3.0],
        );
        assert!(fn_006c2650(&mut e, data));
        assert_eq!(point_at(&e, data + 0x40), [-1.0, -2.0, -3.0]);
        assert_eq!(point_at(&e, data + 0x4c), [1.0, 2.0, 3.0]);
        assert_eq!(point_at(&e, data + 0x58), [1999.0, 2998.0, 97.0]);
        assert_eq!(point_at(&e, data + 0x64), [2001.0, 3002.0, 103.0]);
    }

    #[test]
    fn preparing_an_obstacle_orders_the_columns_by_the_size_of_their_z() {
        // c0 has the largest |z|: u = c0, v = c1, w = c2.
        let (mut e, data) = prepare_scene(
            [[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]],
            [2000.0, 3000.0, 100.0],
            [-1.0, -1.0, -1.0],
            [1.0, 1.0, 1.0],
        );
        assert!(fn_006c2650(&mut e, data));
        assert_eq!(
            matrix_at(&e, data + 0x1c),
            [0.0, -1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
        );
        // The same with a negative z on c0: u and v are negated first.
        let (mut e, data) = prepare_scene(
            [[0.0, 0.0, -1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]],
            [2000.0, 3000.0, 100.0],
            [-1.0, -1.0, -1.0],
            [1.0, 1.0, 1.0],
        );
        assert!(fn_006c2650(&mut e, data));
        assert_eq!(
            matrix_at(&e, data + 0x1c),
            [0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
        );
        // c1 the largest: u = c1, v = c2, w = c0.
        let (mut e, data) = prepare_scene(
            [[1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0]],
            [2000.0, 3000.0, 100.0],
            [-1.0, -1.0, -1.0],
            [1.0, 1.0, 1.0],
        );
        assert!(fn_006c2650(&mut e, data));
        // v = c2 = (0, 1, 0), w = unit cross(up, v) = (-1, 0, 0),
        // v = -(w x up) = (0, -1, 0).
        assert_eq!(
            matrix_at(&e, data + 0x1c),
            [0.0, -1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
        );
    }

    #[test]
    fn preparing_an_obstacle_needs_a_rigid_body_with_a_havok_object() {
        let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let (mut e, data) = prepare_scene(identity, [0.0; 3], [0.0; 3], [1.0; 3]);
        stub_ret(&mut e, GET_WORD_AT_8, 0);
        e.mem.set_u32(data + 0x70, 0x77);
        assert!(!fn_006c2650(&mut e, data));
        assert_eq!(e.mem.u32(data + 0x70), 0x77);
        e.mem.set_u32(data + 0x0c, 0);
        assert!(!fn_006c2650(&mut e, data));
    }

    #[test]
    fn the_activity_of_an_obstacle_follows_the_size_of_its_bounds() {
        let mut e = math_engine();
        let data = obstacle(&mut e, 0x501, 0, true, &[]);
        let slot = slot_with(&mut e, data);
        // (width, depth, height, active)
        let cases = [
            (100.0, 100.0, 40.0, 0),
            (1.0, 1.0, 100.0, 1),
            (20.0, 1.0, 60.0, 1),
            (1.0, 16.0, 60.0, 1),
            (10.0, 10.0, 60.0, 0),
            (10.0, 10.0, 48.0, 0),
            (16.0, 1.0, 95.0, 1),
        ];
        for (width, depth, height, active) in cases {
            set_point(&mut e, data + 0x58, [0.0, 0.0, 0.0]);
            set_point(&mut e, data + 0x64, [width, depth, height]);
            e.mem.set_u8(data + 0x74, 1 - active);
            fn_006c3440(&mut e, slot);
            assert_eq!(e.mem.u8(data + 0x74), active, "{width} {depth} {height}");
        }
    }

    #[test]
    fn a_reference_without_3d_has_no_obstacles() {
        let mut e = engine();
        let array = reference_array(&mut e, 0, &[]);
        let slot = slot_with(&mut e, array);
        let emptied = record(&mut e, OBSTACLE_ARRAY_EMPTY, 0);
        let reference = object_with_slots(&mut e, &[(REFERENCE_NODE_SLOT, 0)]);
        e.mem.set_u32(reference + 0x0c, 0x44);
        assert!(!fn_006c3520(&mut e, reference, slot));
        assert_eq!(*emptied.borrow(), vec![vec![array + 0x0c, 1]]);
        // `006c34f0` stores the form id first and gives the same answer.
        assert!(!fn_006c34f0(&mut e, reference, slot));
        assert_eq!(e.mem.u32(array + 8), 0x44);
    }

    #[test]
    fn a_reference_with_3d_gets_its_obstacles_from_the_tree() {
        let mut e = engine();
        let array = reference_array(&mut e, 0, &[]);
        let slot = slot_with(&mut e, array);
        let node = object_with_slots(&mut e, &[(NODE_CHILDREN_SLOT, 0)]);
        let reference = object_with_slots(&mut e, &[(REFERENCE_NODE_SLOT, node)]);
        e.mem.set_u32(reference + 0x0c, 0x55);
        stub_ret(&mut e, COLLISION_OBJECT_OF_NODE, 0);
        let emptied = record(&mut e, OBSTACLE_ARRAY_EMPTY, 0);
        assert!(fn_006c34f0(&mut e, reference, slot));
        assert_eq!(e.mem.u32(array + 8), 0x55);
        assert_eq!(emptied.borrow().len(), 1);
    }

    #[test]
    fn a_point_inside_the_obstacle_box_and_height_range_is_inside() {
        let mut e = math_engine();
        e.register(POINT_TIMES_MATRIX, |e, a| {
            let v = point_at(e, a[1]);
            let mut out = [0.0; 3];
            for (column, slot) in out.iter_mut().enumerate() {
                for (row, value) in v.iter().enumerate() {
                    *slot += value * e.mem.f32(a[2] + 4 * (3 * row + column) as u32);
                }
            }
            set_point(e, a[0], out);
            Ret::default()
        });
        let data = obstacle(&mut e, 0x501, 0, true, &[]);
        let slot = slot_with(&mut e, data);
        set_point(&mut e, data + 0x10, [10.0, 20.0, 0.0]);
        for (index, value) in [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(data + 0x1c + 4 * index as u32, *value);
        }
        set_point(&mut e, data + 0x40, [-5.0, -6.0, 0.0]);
        set_point(&mut e, data + 0x4c, [5.0, 6.0, 0.0]);
        let point = e.mem.alloc(12);
        let inside = |e: &mut Engine, at: [f32; 3]| {
            set_point(e, point, at);
            fn_006c3570(e, point, slot, 2.0, 4.0)
        };
        assert!(inside(&mut e, [12.0, 22.0, 3.0]));
        assert!(inside(&mut e, [5.0, 14.0, -2.0]));
        assert!(inside(&mut e, [15.0, 26.0, 4.0]));
        assert!(!inside(&mut e, [12.0, 22.0, -2.5]));
        assert!(!inside(&mut e, [12.0, 22.0, 4.5]));
        assert!(!inside(&mut e, [4.5, 22.0, 0.0]));
        assert!(!inside(&mut e, [15.5, 22.0, 0.0]));
        assert!(!inside(&mut e, [12.0, 13.5, 0.0]));
        assert!(!inside(&mut e, [12.0, 26.5, 0.0]));
    }

    #[test]
    fn the_task_counts_are_read_under_the_lock() {
        let mut e = engine();
        let this = manager(&mut e);
        let base = this.addr();
        set_array(&mut e, base + 0x14c, &[1, 2, 3]);
        set_array(&mut e, base + 0x13c, &[1]);
        let locks = record(&mut e, SPIN_LOCK_LOCK, 0);
        let unlocks = record(&mut e, SPIN_LOCK_UNLOCK, 0);
        assert_eq!(fn_006c3920(&mut e, this), 3);
        assert_eq!(fn_006c3970(&mut e, this), 1);
        assert_eq!(
            *locks.borrow(),
            vec![
                vec![base + 0x160, PROCESSED_LOCK_NAME],
                vec![base + 0x160, BACKGROUND_LOCK_NAME]
            ]
        );
        assert_eq!(*unlocks.borrow(), vec![vec![base + 0x160]; 2]);
    }

    #[test]
    fn a_processed_task_is_added_under_the_lock() {
        let mut e = engine();
        let this = manager(&mut e);
        let base = this.addr();
        let locks = record(&mut e, SPIN_LOCK_LOCK, 0);
        let unlocks = record(&mut e, SPIN_LOCK_UNLOCK, 0);
        let added = record_word(&mut e, ARRAY_ADD);
        nav_mesh_obstacle_manager_add_processed_task(&mut e, this, 0x7001);
        assert_eq!(
            *locks.borrow(),
            vec![vec![base + 0x160, ADD_PROCESSED_LOCK_NAME]]
        );
        assert_eq!(*added.borrow(), vec![vec![base + 0x14c, 0x7001]]);
        assert_eq!(*unlocks.borrow(), vec![vec![base + 0x160]]);
    }

    #[test]
    fn the_portal_record_copy_keeps_all_three_fields() {
        let mut e = engine();
        let source = e.mem.alloc(12);
        e.mem.set_u32(source, 0xaaaa);
        e.mem.set_u16(source + 4, 0x1234);
        e.mem.set_u32(source + 8, 0xcccc);
        let copy = e.mem.alloc(12);
        let defaults = record(&mut e, POINT3_DEFAULT, 0);
        assert_eq!(fn_006c4350(&mut e, Ptr::new(copy), source), Ptr::new(copy));
        assert_eq!(*defaults.borrow(), vec![vec![copy]]);
        assert_eq!(e.mem.u32(copy), 0xaaaa);
        assert_eq!(e.mem.u16(copy + 4), 0x1234);
        assert_eq!(e.mem.u32(copy + 8), 0xcccc);
    }

    /// Portal linking: the edge number is `record[+4] + 0x100`, a navmesh's
    /// triangle `n` is at `navmesh * 0x10 + n`; the calls are logged as
    /// `(navmesh, edge record words)` and `(triangle, edge, number)`.
    struct LinkScene {
        e: Engine,
        numbers: Log,
        links: Log,
    }

    /// The doubles of the portal linking; returns the two logs.
    fn link_doubles(e: &mut Engine) -> (Log, Log) {
        stub(e, &[EDGE_RECORD_DEFAULT]);
        let numbers: Log = Rc::default();
        let shared = numbers.clone();
        e.register_double(NAVMESH_EDGE_RECORD_INDEX, move |e, a| {
            let (first, second) = (e.mem.u32(a[1] + 4), e.mem.u16(a[1] + 8) as u32);
            shared.borrow_mut().push(vec![a[0], first, second]);
            ret(first + 0x100)
        });
        e.register(NAVMESH_TRIANGLE, |_, a| ret(a[0] * 0x10 + a[1]));
        let links = record(e, TRIANGLE_LINK_EDGE, 0);
        (numbers, links)
    }

    fn link_scene() -> LinkScene {
        let mut e = engine();
        let (numbers, links) = link_doubles(&mut e);
        LinkScene { e, numbers, links }
    }

    /// A portal record `{word, triangle, edge}`.
    fn portal(e: &mut Engine, word: u32, triangle: u16, edge: u32) -> u32 {
        let record = e.mem.alloc(12);
        e.mem.set_u32(record, word);
        e.mem.set_u16(record + 4, triangle);
        e.mem.set_u32(record + 8, edge);
        record
    }

    #[test]
    fn joining_two_portals_links_each_side_with_the_number_of_the_other() {
        let mut s = link_scene();
        let this = manager(&mut s.e);
        let (first_mesh, second_mesh) = (slot_with(&mut s.e, 0xa1), slot_with(&mut s.e, 0xa2));
        let first = portal(&mut s.e, 0x11, 3, 1);
        let second = portal(&mut s.e, 0x22, 5, 2);
        assert!(fn_006c4390(
            &mut s.e,
            this,
            first_mesh,
            first,
            second_mesh,
            second
        ));
        // Each navmesh is asked for the edge built from the other record.
        assert_eq!(
            *s.numbers.borrow(),
            vec![vec![0xa1, 0x22, 5], vec![0xa2, 0x11, 3]]
        );
        assert_eq!(
            *s.links.borrow(),
            vec![
                vec![0xa1 * 0x10 + 3, 1, 0x122],
                vec![0xa2 * 0x10 + 5, 2, 0x111]
            ]
        );
    }

    #[test]
    fn linking_one_side_uses_the_other_record_for_the_edge() {
        let mut s = link_scene();
        let this = manager(&mut s.e);
        let mesh = slot_with(&mut s.e, 0xa1);
        let own = portal(&mut s.e, 0x11, 3, 1);
        let other = portal(&mut s.e, 0x33, 7, 0);
        assert!(fn_006c4460(&mut s.e, this, mesh, own, other));
        assert_eq!(*s.numbers.borrow(), vec![vec![0xa1, 0x33, 7]]);
        assert_eq!(*s.links.borrow(), vec![vec![0xa1 * 0x10 + 3, 1, 0x133]]);
    }

    /// A manager whose `ProcessedTasks` are `tasks` (objects whose virtual
    /// slot 0 is the deleting destructor, logged as `(task, flags)`).
    fn task_scene(count: usize) -> (Engine, Ptr<NavMeshObstacleManager>, Vec<u32>, Log) {
        let mut e = engine();
        let this = manager(&mut e);
        let log: Log = Rc::default();
        let mut tasks = vec![];
        for _ in 0..count {
            let task = object_with_slots(&mut e, &[(0, 0)]);
            let shared = log.clone();
            e.register_double(task, move |_, a| {
                shared.borrow_mut().push(a.to_vec());
                Ret::default()
            });
            tasks.push(task);
        }
        set_array(&mut e, this.addr() + 0x14c, &tasks);
        (e, this, tasks, log)
    }

    #[test]
    fn finishing_the_processed_tasks_says_whether_any_changed_something() {
        let (mut e, this, tasks, _) = task_scene(3);
        let second = tasks[1];
        let finished = Rc::new(RefCell::new(vec![]));
        let shared = finished.clone();
        e.register_double(OBSTACLE_TASK_FINISH, move |_, a| {
            shared.borrow_mut().push(a[0]);
            ret((a[0] == second) as u32)
        });
        assert!(fn_006c44d0(&mut e, this));
        assert_eq!(*finished.borrow(), tasks);
        stub_ret(&mut e, OBSTACLE_TASK_FINISH, 0);
        assert!(!fn_006c44d0(&mut e, this));
    }

    #[test]
    fn freeing_the_processed_tasks_deletes_them_and_clears_both_arrays() {
        let (mut e, this, tasks, deleted) = task_scene(2);
        // A null entry is skipped.
        let buffer = e.mem.u32(this.addr() + 0x14c + 4);
        e.mem.set_u32(buffer + 4, 0);
        e.mem.set_u32(this.addr() + 0x14c + 8, 2);
        let cleared = record(&mut e, ARRAY_CLEAR, 0);
        assert!(fn_006c4540(&mut e, this));
        assert_eq!(*deleted.borrow(), vec![vec![tasks[0], 1]]);
        assert_eq!(
            *cleared.borrow(),
            vec![vec![this.addr() + 0x14c, 1], vec![this.addr() + 0x13c, 1]]
        );
    }

    /// A manager in the middle of its update cycle: every list is empty
    /// (head nodes), the queues report no entries, the obstacle setting is
    /// on, the frame time is `0.25` and the path manager is `0x7200`.
    struct UpdateScene {
        e: Engine,
        this: Ptr<NavMeshObstacleManager>,
        /// The calls to the functions the update tests look at.
        watched: Vec<u32>,
    }

    const WATCHED: [u32; 11] = [
        CRITICAL_SECTION_ENTER,
        CRITICAL_SECTION_LEAVE,
        PATH_MANAGER_START,
        PATH_MANAGER_IS_READY,
        PATH_MANAGER_FINISH,
        PATH_MANAGER_WAIT,
        NAVMESH_RENDER_REDRAW,
        MAP_FIRST_ITEM,
        LIST_CLEAR,
        SLEEP,
        SCOPE_GUARD_CTOR,
    ];

    fn update_scene() -> UpdateScene {
        let mut e = engine();
        constants(&mut e);
        let this = manager(&mut e);
        let base = this.addr();
        set_setting(&mut e, true);
        stub(
            &mut e,
            &[
                SCOPE_GUARD_CTOR,
                SCOPE_GUARD_DTOR,
                CRITICAL_SECTION_ENTER,
                CRITICAL_SECTION_LEAVE,
                PERFORMANCE_TIMER_STEP,
                SPIN_LOCK_LOCK,
                SPIN_LOCK_UNLOCK,
                PATH_MANAGER_START,
                PATH_MANAGER_FINISH,
                PATH_MANAGER_WAIT,
                NAVMESH_RENDER_REDRAW,
                LIST_CLEAR,
                REFERENCE_ARRAY_CTOR,
                REFERENCE_ARRAY_DTOR,
                MAP_CLEAR,
                SLEEP,
                ARRAY_CLEAR,
            ],
        );
        e.register(CLOCK_FRAME_SECONDS, |_, _| float_ret(0.25));
        stub_ret(&mut e, PATH_MANAGER_INSTANCE, 0x7200);
        stub_ret(&mut e, PATH_MANAGER_IS_READY, 1);
        stub_ret(&mut e, MAP_FIRST_ITEM, 0);
        stub_ret(&mut e, NAVMESH_RENDER_OBJECT, 0x7300);
        stub_ret(&mut e, NAVMESH_RENDER_ENABLED, 1);
        for check in [LIST_NODE_IS_EMPTY, LIST_IS_EMPTY] {
            e.register(check, |e, a| {
                ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
            });
        }
        // The two lock-free queues report no entries.
        let seconds = e.mem.alloc(4);
        e.mem.set_f32(seconds, 0.5);
        stub_ret(&mut e, FLOAT_SETTING_VALUE_ADDRESS, seconds);
        for queue in [0x60, 0xa0] {
            let object = base + queue;
            let vtable = e.mem.alloc(0x20);
            let function = vtable + 0x10;
            e.register_double(function, |_, _| ret(0));
            e.mem.set_u32(vtable + 0x10, function);
            e.mem.set_u32(object, vtable);
        }
        start_log(&mut e);
        UpdateScene {
            e,
            this,
            watched: WATCHED.to_vec(),
        }
    }

    impl UpdateScene {
        fn base(&self) -> u32 {
            self.this.addr()
        }

        fn calls(&self) -> Vec<u32> {
            let watched = self.watched.clone();
            calls(&self.e, &watched).into_iter().map(|c| c.0).collect()
        }
    }

    #[test]
    fn the_update_does_nothing_while_the_obstacle_setting_is_off() {
        let mut s = update_scene();
        set_setting(&mut s.e, false);
        s.e.mem.set_f32(s.base() + 0x194, 5.0);
        fn_006c3640(&mut s.e, s.this);
        assert_eq!(s.calls(), vec![SCOPE_GUARD_CTOR]);
        assert_eq!(s.e.mem.f32(s.base() + 0x194), 5.0);
        // The scope guard of line 0x508 is still closed.
        let guards = calls(&s.e, &[SCOPE_GUARD_CTOR, SCOPE_GUARD_DTOR]);
        assert_eq!(guards.len(), 2);
        assert_eq!(guards[0].1[1..], [SCOPE_CATEGORY, 1, SOURCE_PATH, 0x508]);
    }

    #[test]
    fn collecting_runs_the_queue_steps_and_waits_while_the_time_is_not_up() {
        let mut s = update_scene();
        let base = s.base();
        s.e.mem.set_f32(base + 0x194, 1.0);
        set_array(&mut s.e, base + 0x13c, &[1, 2]);
        set_array(&mut s.e, base + 0x14c, &[1, 2]);
        fn_006c3640(&mut s.e, s.this);
        assert_eq!(s.e.mem.f32(base + 0x194), 0.75);
        assert_eq!(s.e.mem.u32(base + 0x190), 0);
        // Under the critical section; the steps cleared the queued id lists
        // (references to remove, navmeshes to enable and to disable).
        assert_eq!(
            s.calls(),
            vec![
                SCOPE_GUARD_CTOR,
                CRITICAL_SECTION_ENTER,
                LIST_CLEAR,
                LIST_CLEAR,
                LIST_CLEAR,
                CRITICAL_SECTION_LEAVE
            ]
        );
    }

    #[test]
    fn collecting_starts_the_path_manager_when_the_time_is_up_and_the_tasks_agree() {
        let mut s = update_scene();
        let base = s.base();
        s.e.mem.set_f32(base + 0x194, 0.1);
        set_array(&mut s.e, base + 0x13c, &[1, 2]);
        set_array(&mut s.e, base + 0x14c, &[1, 2]);
        fn_006c3640(&mut s.e, s.this);
        assert!(s.e.mem.f32(base + 0x194) < 0.0);
        assert_eq!(s.e.mem.u32(base + 0x190), 1);
        assert!(s.calls().contains(&PATH_MANAGER_START));
    }

    #[test]
    fn collecting_with_other_task_counts_or_no_tasks_does_not_start() {
        // More background than processed tasks: not yet.
        let mut s = update_scene();
        let base = s.base();
        s.e.mem.set_f32(base + 0x194, 0.0);
        set_array(&mut s.e, base + 0x13c, &[1, 2, 3]);
        set_array(&mut s.e, base + 0x14c, &[1, 2]);
        fn_006c3640(&mut s.e, s.this);
        assert_eq!(s.e.mem.u32(base + 0x190), 0);
        // No task at all and nothing queued in the task map: not either.
        let mut s = update_scene();
        let base = s.base();
        s.e.mem.set_f32(base + 0x194, 0.0);
        fn_006c3640(&mut s.e, s.this);
        assert_eq!(s.e.mem.u32(base + 0x190), 0);
        assert!(!s.calls().contains(&PATH_MANAGER_START));
        // Tasks queued in the map but none in the background: the swap goes.
        let mut s = update_scene();
        let base = s.base();
        s.e.mem.set_f32(base + 0x194, 0.0);
        s.e.mem.set_u32(base + 0x12c + 0x0c, 2);
        fn_006c3640(&mut s.e, s.this);
        assert_eq!(s.e.mem.u32(base + 0x190), 1);
        assert!(s.calls().contains(&PATH_MANAGER_START));
        // ... but not before the time is up.
        let mut s = update_scene();
        let base = s.base();
        s.e.mem.set_f32(base + 0x194, 1.0);
        s.e.mem.set_u32(base + 0x12c + 0x0c, 2);
        fn_006c3640(&mut s.e, s.this);
        assert_eq!(s.e.mem.u32(base + 0x190), 0);
    }

    #[test]
    fn collecting_refreshes_every_obstacle_when_asked_to() {
        let mut s = update_scene();
        let base = s.base();
        s.e.mem.set_u8(base + 0x18, 1);
        s.e.mem.set_f32(base + 0x194, 1.0);
        fn_006c3640(&mut s.e, s.this);
        assert_eq!(s.e.mem.u8(base + 0x18), 0);
        // `006c4c30` walks the map from its first item.
        assert!(s.calls().contains(&MAP_FIRST_ITEM));
    }

    #[test]
    fn waiting_does_nothing_until_the_path_manager_is_ready() {
        let mut s = update_scene();
        let base = s.base();
        s.e.mem.set_u32(base + 0x190, 1);
        s.e.mem.set_f32(base + 0x194, 3.0);
        stub_ret(&mut s.e, PATH_MANAGER_IS_READY, 0);
        fn_006c3640(&mut s.e, s.this);
        assert_eq!(s.e.mem.u32(base + 0x190), 1);
        assert_eq!(
            s.calls(),
            vec![
                SCOPE_GUARD_CTOR,
                CRITICAL_SECTION_ENTER,
                PATH_MANAGER_IS_READY,
                CRITICAL_SECTION_LEAVE
            ]
        );
    }

    #[test]
    fn waiting_without_background_tasks_just_releases_the_path_manager() {
        let mut s = update_scene();
        let base = s.base();
        s.e.mem.set_u32(base + 0x190, 1);
        s.e.mem.set_f32(base + 0x194, 3.0);
        fn_006c3640(&mut s.e, s.this);
        assert_eq!(s.e.mem.u32(base + 0x190), 0);
        assert_eq!(s.e.mem.f32(base + 0x194), 0.5);
        let calls = s.calls();
        assert!(calls.contains(&PATH_MANAGER_FINISH));
        assert!(!calls.contains(&NAVMESH_RENDER_REDRAW));
    }

    #[test]
    fn waiting_with_background_tasks_reconnects_and_redraws_when_something_changed() {
        let mut s = update_scene();
        let base = s.base();
        s.e.mem.set_u32(base + 0x190, 1);
        let task = object_with_slots(&mut s.e, &[(0, 0)]);
        set_array(&mut s.e, base + 0x13c, &[task]);
        set_array(&mut s.e, base + 0x14c, &[task]);
        // The processed task has no new navmesh to reconnect and finishes
        // with a change.
        stub_ret(&mut s.e, HOLDER_POINTER, 0);
        stub_ret(&mut s.e, OBSTACLE_TASK_FINISH, 1);
        let redraw = record(&mut s.e, NAVMESH_RENDER_REDRAW, 0);
        let freed = record(&mut s.e, task, 0);
        let cleared = record(&mut s.e, ARRAY_CLEAR, 0);
        fn_006c3640(&mut s.e, s.this);
        assert_eq!(s.e.mem.u32(base + 0x190), 0);
        assert_eq!(*freed.borrow(), vec![vec![task, 1]]);
        assert_eq!(cleared.borrow().len(), 2);
        assert_eq!(*redraw.borrow(), vec![vec![0x7300]]);
        // Without the navmesh display nothing is redrawn.
        stub_ret(&mut s.e, NAVMESH_RENDER_ENABLED, 0);
        s.e.mem.set_u32(base + 0x190, 1);
        set_array(&mut s.e, base + 0x13c, &[task]);
        set_array(&mut s.e, base + 0x14c, &[task]);
        fn_006c3640(&mut s.e, s.this);
        assert_eq!(redraw.borrow().len(), 1);
    }

    #[test]
    fn a_flush_runs_the_whole_cycle_and_waits_for_the_tasks() {
        let mut s = update_scene();
        let base = s.base();
        s.e.mem.set_u32(base + 0x190, 1);
        s.e.mem.set_f32(base + 0x194, 3.0);
        // One background task less than processed ones until the first
        // `Sleep(0)`, which lets it finish.
        let task = object_with_slots(&mut s.e, &[(0, 0)]);
        set_array(&mut s.e, base + 0x14c, &[task, task]);
        set_array(&mut s.e, base + 0x13c, &[task]);
        s.e.register_double(SLEEP, move |e, _| {
            e.mem.set_u32(base + 0x13c + 8, 2);
            Ret::default()
        });
        // The path manager needs one wait before it is ready.
        let mut polls = 0;
        s.e.register_double(PATH_MANAGER_IS_READY, move |_, _| {
            polls += 1;
            ret((polls > 1) as u32)
        });
        stub_ret(&mut s.e, HOLDER_POINTER, 0);
        stub_ret(&mut s.e, OBSTACLE_TASK_FINISH, 0);
        s.e.mem.set_u8(base + 0x18, 0);
        s.watched = vec![
            CRITICAL_SECTION_ENTER,
            PATH_MANAGER_START,
            PATH_MANAGER_IS_READY,
            PATH_MANAGER_WAIT,
            SLEEP,
            PATH_MANAGER_FINISH,
            CRITICAL_SECTION_LEAVE,
        ];
        fn_006c39c0(&mut s.e, s.this);
        assert_eq!(
            s.calls(),
            vec![
                CRITICAL_SECTION_ENTER,
                SLEEP,
                PATH_MANAGER_START,
                PATH_MANAGER_IS_READY,
                PATH_MANAGER_WAIT,
                PATH_MANAGER_IS_READY,
                PATH_MANAGER_FINISH,
                CRITICAL_SECTION_LEAVE
            ]
        );
        assert_eq!(s.e.mem.u32(base + 0x190), 0);
        assert_eq!(s.e.mem.f32(base + 0x194), 0.5);
    }

    /// The vertices of an edge: `(navmesh, triangle, edge)` to its two end
    /// points.
    type EdgeTable = Rc<RefCell<std::collections::HashMap<(u32, u32, u32), ([f32; 3], [f32; 3])>>>;

    /// Portals of two processed tasks. Each task has a new navmesh (`0xa1`,
    /// `0xa2`), ten triangles in every navmesh, and the portal records the
    /// test gives it.
    struct PortalScene {
        e: Engine,
        this: Ptr<NavMeshObstacleManager>,
        tasks: Vec<u32>,
        numbers: Log,
        links: Log,
        messages: Log,
        edges: EdgeTable,
    }

    /// Writes `records` as a `BSSimpleArray` of 12-byte records at `array`.
    fn set_records(e: &mut Engine, array: u32, records: &[(u32, u16, u32)]) {
        let buffer = e.mem.alloc(12 * records.len().max(1) as u32);
        for (index, (word, triangle, edge)) in records.iter().enumerate() {
            let at = buffer + 12 * index as u32;
            e.mem.set_u32(at, *word);
            e.mem.set_u16(at + 4, *triangle);
            e.mem.set_u32(at + 8, *edge);
        }
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, records.len() as u32);
    }

    /// An `ObstacleTaskData`: `pInfo`, `spSrcMesh`, `spNewNavMesh`, the
    /// portal records and the second array.
    fn portal_task(
        e: &mut Engine,
        info: u32,
        source_mesh: u32,
        new_mesh: u32,
        portals: &[(u32, u16, u32)],
        second: &[(u32, u16, u32)],
    ) -> u32 {
        let task = e.mem.alloc(0x60);
        e.mem.set_u32(task + 0x18, info);
        e.mem.set_u32(task + 0x1c, source_mesh);
        e.mem.set_u32(task + 0x20, new_mesh);
        set_records(e, task + 0x34, portals);
        set_records(e, task + 0x44, second);
        task
    }

    fn portal_scene() -> PortalScene {
        let mut e = math_engine();
        let this = manager(&mut e);
        let (numbers, links) = link_doubles(&mut e);
        e.register(HOLDER_POINTER, |e, a| ret(e.mem.u32(a[0])));
        e.register(RECORD_ARRAY_AT, |e, a| ret(e.mem.u32(a[0] + 4) + 12 * a[1]));
        e.register(RECORD_ARRAY_REMOVE, |e, a| {
            // Removes `a[2]` records from index `a[1]`.
            let buffer = e.mem.u32(a[0] + 4);
            let size = e.mem.u32(a[0] + 8);
            for index in a[1]..size - a[2] {
                for word in 0..3 {
                    let value = e.mem.u32(buffer + 12 * (index + a[2]) + 4 * word);
                    e.mem.set_u32(buffer + 12 * index + 4 * word, value);
                }
            }
            e.mem.set_u32(a[0] + 8, size - a[2]);
            Ret::default()
        });
        stub_ret(&mut e, NAVMESH_TRIANGLE_COUNT, 10);
        let messages = record(&mut e, LOG_MESSAGE, 0);
        let edges: EdgeTable = Rc::default();
        let table = edges.clone();
        e.register_double(NAVMESH_EDGE_VERTICES, move |e, a| {
            let (first, second) = table
                .borrow()
                .get(&(a[0], a[2], a[3]))
                .copied()
                .unwrap_or_else(|| panic!("no edge {:x?}", (a[0], a[2], a[3])));
            let points = e.mem.alloc(24);
            set_point(e, points, first);
            set_point(e, points + 12, second);
            e.mem.set_u32(a[1], points);
            e.mem.set_u32(a[1] + 4, points + 12);
            ret(a[1])
        });
        PortalScene {
            e,
            this,
            tasks: vec![],
            numbers,
            links,
            messages,
            edges,
        }
    }

    impl PortalScene {
        fn set_tasks(&mut self, tasks: &[u32]) {
            set_array(&mut self.e, self.this.addr() + 0x14c, tasks);
            self.tasks = tasks.to_vec();
        }

        fn edge(&self, navmesh: u32, triangle: u32, edge: u32, ends: ([f32; 3], [f32; 3])) {
            self.edges
                .borrow_mut()
                .insert((navmesh, triangle, edge), ends);
        }
    }

    fn portal_count(e: &Engine, task: u32) -> u32 {
        e.mem.u32(task + 0x34 + 8)
    }

    #[test]
    fn two_portals_with_opposite_ends_close_together_are_joined() {
        let mut s = portal_scene();
        let first = portal_task(&mut s.e, 0x5001, 0xa1, 0xa1, &[(1, 2, 0)], &[]);
        let second = portal_task(&mut s.e, 0x5002, 0xa2, 0xa2, &[(2, 4, 1)], &[]);
        s.set_tasks(&[first, second]);
        // A = (0,0)-(100,0); B = (100,10)-(0,10): A1~B0 and A0~B1.
        s.edge(0xa1, 2, 0, ([0.0, 0.0, 0.0], [100.0, 0.0, 0.0]));
        s.edge(0xa2, 4, 1, ([100.0, 10.0, 7.0], [0.0, 10.0, 7.0]));
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        // Linked from both sides, both records gone, both tasks marked.
        assert_eq!(
            *s.numbers.borrow(),
            vec![vec![0xa1, 2, 4], vec![0xa2, 1, 2]]
        );
        assert_eq!(
            *s.links.borrow(),
            vec![
                vec![0xa1 * 0x10 + 2, 0, 0x102],
                vec![0xa2 * 0x10 + 4, 1, 0x101]
            ]
        );
        assert_eq!(portal_count(&s.e, first), 0);
        assert_eq!(portal_count(&s.e, second), 0);
        assert_eq!(s.e.mem.u8(first + 0x54), 1);
        assert_eq!(s.e.mem.u8(second + 0x54), 1);
        assert!(s.messages.borrow().is_empty());
    }

    #[test]
    fn portals_with_distant_ends_stay() {
        let mut s = portal_scene();
        let first = portal_task(&mut s.e, 0x5001, 0xa1, 0xa1, &[(1, 2, 0)], &[]);
        let second = portal_task(&mut s.e, 0x5002, 0xa2, 0xa2, &[(2, 4, 1)], &[]);
        s.set_tasks(&[first, second]);
        s.edge(0xa1, 2, 0, ([0.0, 0.0, 0.0], [100.0, 0.0, 0.0]));
        // One end is 60 away (3600 > 2500), the other close; no join.
        s.edge(0xa2, 4, 1, ([100.0, 60.0, 0.0], [0.0, 10.0, 0.0]));
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        assert!(s.links.borrow().is_empty());
        assert_eq!(portal_count(&s.e, first), 1);
        assert_eq!(portal_count(&s.e, second), 1);
        assert_eq!(s.e.mem.u8(first + 0x54), 0);
        // The same with the other end far away.
        s.edge(0xa2, 4, 1, ([100.0, 10.0, 0.0], [0.0, 60.0, 0.0]));
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        assert!(s.links.borrow().is_empty());
    }

    #[test]
    fn a_portal_with_a_bad_triangle_index_is_logged_and_kept() {
        let mut s = portal_scene();
        let first = portal_task(&mut s.e, 0x5001, 0xa1, 0xa1, &[(1, 12, 0)], &[]);
        s.set_tasks(&[first]);
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        assert_eq!(*s.messages.borrow(), vec![vec![BAD_TRIANGLE_MESSAGE]]);
        assert_eq!(portal_count(&s.e, first), 1);
        // The partner's bad index is logged too and does not stop the walk.
        let mut s = portal_scene();
        let first = portal_task(&mut s.e, 0x5001, 0xa1, 0xa1, &[(1, 2, 0)], &[]);
        let second = portal_task(&mut s.e, 0x5002, 0xa2, 0xa2, &[(2, 40, 1)], &[]);
        s.set_tasks(&[first, second]);
        s.edge(0xa1, 2, 0, ([0.0; 3], [100.0, 0.0, 0.0]));
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        // Logged once for the second task's own pass and once as a partner.
        assert_eq!(s.messages.borrow().len(), 2);
        assert!(s.links.borrow().is_empty());
    }

    #[test]
    fn a_task_without_a_new_navmesh_is_skipped() {
        let mut s = portal_scene();
        let first = portal_task(&mut s.e, 0x5001, 0xa1, 0, &[(1, 2, 0)], &[]);
        s.set_tasks(&[first]);
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        assert_eq!(portal_count(&s.e, first), 1);
        assert!(s.messages.borrow().is_empty());
    }

    /// A first task with a portal that has no partner among the new
    /// navmeshes and a record in its second array; the matching edge is in
    /// the navmesh of the task with info `0x5002`.
    fn second_array_scene() -> (PortalScene, u32, u32) {
        let mut s = portal_scene();
        let first = portal_task(&mut s.e, 0x5001, 0xa1, 0xa1, &[(1, 2, 0)], &[(0x77, 3, 1)]);
        let second = portal_task(&mut s.e, 0x5002, 0xa2, 0xa2, &[], &[]);
        s.set_tasks(&[first, second]);
        s.edge(0xa1, 2, 0, ([0.0, 0.0, 0.0], [100.0, 0.0, 0.0]));
        // `GetMatchingEdge` finds triangle 6, edge 3 of the navmesh of
        // `0x5002`.
        s.e.register_double(NAVMESH_MATCHING_EDGE, |e, a| {
            assert_eq!(a[0], 0xa1);
            assert_eq!(e.mem.u32(a[1] + 4) & 0xffff, 3);
            e.mem.set_u32(a[2], 0x5002);
            e.mem.set_u16(a[2] + 4, 6);
            e.mem.set_u32(a[2] + 8, 3);
            ret(1)
        });
        s.e.register(NAVMESH_HOLDER_CTOR, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        s.e.register(NAVMESH_HOLDER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        stub(&mut s.e, &[NAVMESH_HOLDER_DTOR, MATCHING_EDGE_DEFAULT]);
        stub_ret(&mut s.e, TRIANGLE_HAS_FLAG, 0);
        (s, first, second)
    }

    #[test]
    fn a_portal_without_a_partner_is_joined_to_the_matching_edge_of_another_task() {
        let (mut s, first, second) = second_array_scene();
        s.edge(0xa2, 6, 3, ([100.0, 20.0, 0.0], [0.0, 20.0, 0.0]));
        let flags = record(&mut s.e, TRIANGLE_HAS_FLAG, 0);
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        assert_eq!(*flags.borrow(), vec![vec![0xa2 * 0x10 + 6, 0x20]]);
        // Both sides are linked: the first task's triangle 2 (edge 0) and
        // the partner's triangle 6 (edge 3).
        let links = s.links.borrow();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0][0..2], [0xa1 * 0x10 + 2, 0]);
        assert_eq!(links[1][0..2], [0xa2 * 0x10 + 6, 3]);
        assert_eq!(portal_count(&s.e, first), 0);
        assert_eq!(s.e.mem.u8(first + 0x54), 1);
        assert_eq!(s.e.mem.u8(second + 0x54), 1);
    }

    #[test]
    fn a_partner_triangle_with_the_flag_or_far_ends_is_not_joined() {
        let (mut s, first, second) = second_array_scene();
        s.edge(0xa2, 6, 3, ([100.0, 20.0, 0.0], [0.0, 20.0, 0.0]));
        stub_ret(&mut s.e, TRIANGLE_HAS_FLAG, 1);
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        assert!(s.links.borrow().is_empty());
        assert_eq!(portal_count(&s.e, first), 1);
        assert_eq!(s.e.mem.u8(second + 0x54), 0);
        // Far ends.
        stub_ret(&mut s.e, TRIANGLE_HAS_FLAG, 0);
        s.edge(0xa2, 6, 3, ([500.0, 20.0, 0.0], [0.0, 20.0, 0.0]));
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        assert!(s.links.borrow().is_empty());
        s.edge(0xa2, 6, 3, ([100.0, 20.0, 0.0], [0.0, 520.0, 0.0]));
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        assert!(s.links.borrow().is_empty());
        assert_eq!(portal_count(&s.e, first), 1);
    }

    #[test]
    fn a_matching_edge_in_the_same_navmesh_or_a_bad_triangle_is_ignored() {
        let (mut s, first, _) = second_array_scene();
        // The matching edge belongs to the record's own info word.
        s.e.register_double(NAVMESH_MATCHING_EDGE, |e, a| {
            e.mem.set_u32(a[2], 0x77);
            e.mem.set_u16(a[2] + 4, 6);
            ret(1)
        });
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        assert_eq!(portal_count(&s.e, first), 1);
        // A triangle number out of range in the partner navmesh.
        s.e.register_double(NAVMESH_MATCHING_EDGE, |e, a| {
            e.mem.set_u32(a[2], 0x5002);
            e.mem.set_u16(a[2] + 4, 30);
            ret(1)
        });
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        assert_eq!(portal_count(&s.e, first), 1);
        assert!(s.links.borrow().is_empty());
        // No matching edge at all.
        stub_ret(&mut s.e, NAVMESH_MATCHING_EDGE, 0);
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        assert_eq!(portal_count(&s.e, first), 1);
    }

    #[test]
    fn an_unloaded_partner_navmesh_is_linked_from_the_extra_edge_information() {
        let (mut s, first, _) = second_array_scene();
        // No task has the info `0x5002` and loading it gives nothing.
        s.e.mem.set_u32(s.tasks[1] + 0x18, 0x6000);
        let loads = record(&mut s.e, NAVMESH_INFO_LOAD_HOLDER, 0);
        let extra = s.e.mem.alloc(12);
        s.e.mem.set_u32(extra + 4, 0x31);
        s.e.mem.set_u16(extra + 8, 4);
        let extra_info = record(&mut s.e, NAVMESH_EDGE_EXTRA_INFO, extra);
        let record_set: Log = Rc::default();
        let shared = record_set.clone();
        s.e.register_double(EDGE_RECORD_SET, move |e, a| {
            shared.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u16(a[0] + 4, a[2] as u16);
            ret(a[0])
        });
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        assert_eq!(loads.borrow().len(), 1);
        assert_eq!(loads.borrow()[0][0], 0x5002);
        // The source mesh is asked for the extra information of the record's
        // triangle and edge.
        assert_eq!(*extra_info.borrow(), vec![vec![0xa1, 3, 1]]);
        assert_eq!(record_set.borrow()[0][1..], [0x31, 4]);
        // One side is linked and the portal is gone.
        assert_eq!(*s.numbers.borrow(), vec![vec![0xa1, 0x31, 4]]);
        assert_eq!(portal_count(&s.e, first), 0);
        // Without extra information the portal stays.
        let (mut s, first, _) = second_array_scene();
        s.e.mem.set_u32(s.tasks[1] + 0x18, 0x6000);
        stub_ret(&mut s.e, NAVMESH_INFO_LOAD_HOLDER, 0);
        stub_ret(&mut s.e, NAVMESH_EDGE_EXTRA_INFO, 0);
        assert!(nav_mesh_obstacle_manager_reconnect_portals(
            &mut s.e, s.this
        ));
        assert_eq!(portal_count(&s.e, first), 1);
        assert!(s.links.borrow().is_empty());
    }

    /// Builds a `BSSimpleList` chain in place: the head node at `head` holds
    /// the first item, the other nodes are allocated.
    fn list_chain(e: &mut Engine, head: u32, items: &[u32]) {
        let mut at = head;
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(at, *item);
            if index + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(at + 4, next);
                at = next;
            }
        }
    }

    /// The traversal helpers of the queued lists: `IsEmpty` of a node (no item
    /// and no next node), its next node and its own address.
    fn list_doubles(e: &mut Engine) {
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(POINT3_DEFAULT, |_, a| ret(a[0]));
    }

    /// `BSSimpleList::Remove(&value)` as the game does it: the first node
    /// holding the value goes (the head takes the next node's item and link);
    /// logs `(list, value)`.
    fn removing_list_double(e: &mut Engine) -> Log {
        let log: Log = Rc::default();
        let shared = log.clone();
        e.register_double(LIST_REMOVE, move |e, a| {
            let value = e.mem.u32(a[1]);
            shared.borrow_mut().push(vec![a[0], value]);
            let mut previous = 0;
            let mut node = a[0];
            while node != 0 && e.mem.u32(node) != value {
                previous = node;
                node = e.mem.u32(node + 4);
            }
            if node == a[0] {
                let next = e.mem.u32(node + 4);
                if next == 0 {
                    e.mem.set_u32(node, 0);
                } else {
                    let (item, after) = (e.mem.u32(next), e.mem.u32(next + 4));
                    e.mem.set_u32(node, item);
                    e.mem.set_u32(node + 4, after);
                }
            } else if node != 0 {
                let after = e.mem.u32(node + 4);
                e.mem.set_u32(previous + 4, after);
            }
            Ret::default()
        });
        log
    }

    /// `NiTMap::GetAt(key, &out)`: `(map, key)` pairs found with the array.
    fn keyed_map_get(e: &mut Engine, entries: &[(u32, u32, u32)]) {
        let entries = entries.to_vec();
        e.register_double(MAP_GET, move |e, a| {
            match entries
                .iter()
                .find(|entry| entry.0 == a[0] && entry.1 == a[1])
            {
                Some(entry) => {
                    e.mem.set_u32(a[2], entry.2);
                    ret(1)
                }
                None => ret(0),
            }
        });
    }

    /// A lock-free queue at `at`: virtual slot `0x10` gives its count and
    /// `006c6030` pops the values in order.
    fn queue_with(e: &mut Engine, at: u32, values: &[u32]) {
        let vtable = e.mem.alloc(0x20);
        let count = values.len() as u32;
        e.register_double(vtable + 0x10, move |_, _| ret(count));
        e.mem.set_u32(vtable + 0x10, vtable + 0x10);
        e.mem.set_u32(at, vtable);
        let mut remaining: Vec<u32> = values.iter().rev().copied().collect();
        e.register_double(QUEUE_POP, move |e, a| {
            let value = remaining.pop().unwrap_or(0);
            e.mem.set_u32(a[1], value);
            ret(1)
        });
    }

    /// An `hkVector4` holding `position`; a rigid body "is" its position in
    /// the tests below (see [`position_doubles`]).
    fn position_vector(e: &mut Engine, position: [f32; 3]) -> u32 {
        let vector = e.mem.alloc(16);
        set_point(e, vector, position);
        vector
    }

    /// The position queries: the Havok object of a rigid body (`00620b80`)
    /// and its position (`004b4ec0`) are the rigid body itself, which is an
    /// `hkVector4` in memory; copies and conversions are plain.
    fn position_doubles(e: &mut Engine) {
        e.register(GET_WORD_AT_8, |_, a| ret(a[0]));
        e.register(HAVOK_POSITION, |_, a| ret(a[0]));
        e.register(HK_VECTOR_COPY, |e, a| {
            for word in 0..4 {
                let value = e.mem.u32(a[1] + 4 * word);
                e.mem.set_u32(a[0] + 4 * word, value);
            }
            Ret::default()
        });
        e.register(HK_VECTOR_TO_POINT3, |e, a| {
            let point = point_at(e, a[1]);
            set_point(e, a[0], point);
            Ret::default()
        });
    }

    #[test]
    fn queued_removals_drop_the_entry_and_its_obstacles() {
        let mut s = scene(false);
        list_doubles(&mut s.e);
        let base = s.this.addr();
        let first = obstacle(&mut s.e, 0x501, 0, false, &[]);
        let second = obstacle(&mut s.e, 0x502, 0, false, &[]);
        let array = reference_array(&mut s.e, 0x44, &[first, second]);
        keyed_map_get(&mut s.e, &[(base + 0x1c, 0x44, array)]);
        list_chain(&mut s.e, base + 0x34, &[0x44, 0x55]);
        let cleared = record(&mut s.e, LIST_CLEAR, 0);
        fn_006c47e0(&mut s.e, s.this);
        // `006c1590` removes the entry of 0x44 and its rigid bodies; 0x55 has
        // no entry.
        assert_eq!(
            *s.map_remove.borrow(),
            vec![
                vec![base + 0x1c, 0x44],
                vec![base + 0xe0, 0x501],
                vec![base + 0xe0, 0x502]
            ]
        );
        // The update list is asked to drop each of the two obstacles twice:
        // by this step, then by `006c1590`.
        assert_eq!(*s.list_checks.borrow(), vec![vec![base + 0x3c]; 4]);
        assert_eq!(*cleared.borrow(), vec![vec![base + 0x34]]);
    }

    #[test]
    fn queued_navmeshes_are_enabled_and_disabled_by_id() {
        let mut e = engine();
        list_doubles(&mut e);
        let this = manager(&mut e);
        let base = this.addr();
        list_chain(&mut e, base + 0x180, &[5, 6]);
        list_chain(&mut e, base + 0x188, &[7]);
        let set = record(&mut e, SET_DISABLE_NAVMESH, 0);
        let cleared = record(&mut e, LIST_CLEAR, 0);
        assert!(fn_006c4900(&mut e, this));
        assert!(fn_006c4970(&mut e, this));
        assert_eq!(*set.borrow(), vec![vec![5, 0], vec![6, 0], vec![7, 1]]);
        assert_eq!(
            *cleared.borrow(),
            vec![vec![base + 0x180], vec![base + 0x188]]
        );
    }

    #[test]
    fn an_empty_queued_navmesh_list_only_gets_cleared() {
        let mut e = engine();
        list_doubles(&mut e);
        let this = manager(&mut e);
        let set = record(&mut e, SET_DISABLE_NAVMESH, 0);
        let cleared = record(&mut e, LIST_CLEAR, 0);
        assert!(fn_006c4900(&mut e, this));
        assert!(set.borrow().is_empty());
        assert_eq!(cleared.borrow().len(), 1);
    }

    #[test]
    fn rigid_bodies_queued_for_adding_enter_the_update_list_once() {
        let mut e = engine();
        let this = manager(&mut e);
        let base = this.addr();
        queue_with(&mut e, base + 0x60, &[0x501, 0x502, 0x503]);
        let first = obstacle(&mut e, 0x501, 0, true, &[]);
        let second = obstacle(&mut e, 0x502, 0, true, &[]);
        keyed_map_get(
            &mut e,
            &[(base + 0xe0, 0x501, first), (base + 0xe0, 0x502, second)],
        );
        // The second obstacle is in the update list already.
        e.register_double(OBSTACLE_LIST_CONTAINS, move |e, a| {
            ret((e.mem.u32(a[1]) == second) as u32)
        });
        let added = record_word(&mut e, OBSTACLE_LIST_ADD);
        fn_006c49e0(&mut e, this);
        assert_eq!(*added.borrow(), vec![vec![base + 0x3c, first]]);
    }

    #[test]
    fn rigid_bodies_queued_for_removal_are_refreshed_only_when_they_moved() {
        let mut s = scene(false);
        add_math(&mut s.e);
        position_doubles(&mut s.e);
        let base = s.this.addr();
        // Rigid bodies are their own positions here: 5 units away (squared
        // 25, not over) and 6 units away.
        let near = position_vector(&mut s.e, [3.0, 4.0, 0.0]);
        let far = position_vector(&mut s.e, [6.0, 0.0, 0.0]);
        // The queue holds the rigid bodies (here: the vectors); the map
        // finds an obstacle for each.
        queue_with(&mut s.e, base + 0xa0, &[near, far]);
        let first = obstacle(&mut s.e, 0x501, 0, false, &[]);
        let second = obstacle(&mut s.e, 0x502, 0, false, &[]);
        keyed_map_get(
            &mut s.e,
            &[(base + 0xe0, near, first), (base + 0xe0, far, second)],
        );
        fn_006c4ad0(&mut s.e, s.this);
        // Only the far one is re-registered; both leave the update list.
        assert_eq!(s.refresh.borrow().len(), 1);
        assert_eq!(s.refresh.borrow()[0][1], second);
        assert_eq!(*s.list_checks.borrow(), vec![vec![base + 0x3c]; 2]);
    }

    #[test]
    fn every_obstacle_of_every_reference_is_refreshed_when_all_are_updated() {
        let mut s = scene(false);
        let base = s.this.addr();
        let table = hashed_map(&mut s.e, base + 0x1c);
        let first_obstacle = obstacle(&mut s.e, 0x501, 0, false, &[]);
        let second_obstacle = obstacle(&mut s.e, 0x502, 0, false, &[]);
        let last_obstacle = obstacle(&mut s.e, 0x503, 0, false, &[]);
        let last_array = reference_array(&mut s.e, 3, &[last_obstacle]);
        let second_array = reference_array(&mut s.e, 1, &[second_obstacle]);
        let first_array = reference_array(&mut s.e, 5, &[first_obstacle]);
        let last = map_item(&mut s.e, 0, 3, last_array);
        let second = map_item(&mut s.e, 0, 1, second_array);
        let first = map_item(&mut s.e, second, 5, first_array);
        s.e.mem.set_u32(table + 12, last);
        stub_ret(&mut s.e, MAP_FIRST_ITEM, first);
        fn_006c4c30(&mut s.e, s.this);
        let refreshed: Vec<u32> = s.refresh.borrow().iter().map(|call| call[1]).collect();
        assert_eq!(
            refreshed,
            vec![first_obstacle, second_obstacle, last_obstacle]
        );
    }

    #[test]
    fn a_moved_obstacle_is_refreshed_once_its_delay_has_passed() {
        let mut s = scene(false);
        add_math(&mut s.e);
        position_doubles(&mut s.e);
        list_doubles(&mut s.e);
        let base = s.this.addr();
        s.e.register(LIST_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        // The delay is 0.5 s: 500 ms; the clock reads 10000.
        let seconds = s.e.mem.alloc(4);
        s.e.mem.set_f32(seconds, 0.5);
        stub_ret(&mut s.e, FLOAT_SETTING_VALUE_ADDRESS, seconds);
        stub_ret(&mut s.e, CLOCK_MILLISECONDS, 10_000);
        let moved_old = position_vector(&mut s.e, [6.0, 0.0, 0.0]);
        let moved_new = position_vector(&mut s.e, [6.0, 0.0, 0.0]);
        let still = position_vector(&mut s.e, [1.0, 1.0, 0.0]);
        let first = obstacle(&mut s.e, moved_old, 0, false, &[]);
        let second = obstacle(&mut s.e, moved_new, 0, false, &[]);
        let third = obstacle(&mut s.e, still, 0, false, &[]);
        s.e.mem.set_u32(first + 0x70, 9_000);
        s.e.mem.set_u32(second + 0x70, 9_800);
        s.e.mem.set_u32(third + 0x70, 0);
        list_chain(&mut s.e, base + 0x3c, &[first, second, third]);
        fn_006c4d30(&mut s.e, s.this);
        // Only the first one moved and has waited long enough.
        assert_eq!(s.refresh.borrow().len(), 1);
        assert_eq!(s.refresh.borrow()[0][1], first);
    }

    #[test]
    fn collected_tasks_go_to_the_tasklets_or_to_the_background_list() {
        // Tasks are handed to `006c5ce0` when the setting is on ...
        let mut e = engine();
        let this = manager(&mut e);
        let base = this.addr();
        let tasks: Vec<u32> = (0..2)
            .map(|_| object_with_slots(&mut e, &[(8, 0)]))
            .collect();
        let queue = tasks.clone();
        let mut remaining = queue.clone();
        remaining.reverse();
        e.register_double(MAP_FIRST_ITEM, |_, _| ret(1));
        let mut left = 2;
        e.register_double(TASK_MAP_GET_NEXT, move |e, a| {
            left -= 1;
            e.mem.set_u32(a[3], remaining.pop().unwrap());
            e.mem.set_u32(a[1], (left > 0) as u32);
            Ret::default()
        });
        let handed = record(&mut e, QUEUE_TASK, 0);
        let cleared = record(&mut e, MAP_CLEAR, 0);
        set_setting(&mut e, true);
        fn_006c4ec0(&mut e, this);
        assert_eq!(
            *handed.borrow(),
            vec![vec![base, tasks[0]], vec![base, tasks[1]]]
        );
        assert_eq!(*cleared.borrow(), vec![vec![base + 0x12c]]);

        // ... and started through virtual slot 2 after being listed when it
        // is off.
        let mut e = engine();
        let this = manager(&mut e);
        let base = this.addr();
        let task = object_with_slots(&mut e, &[(8, 0)]);
        let started = record(&mut e, task + 8, 0);
        e.register_double(MAP_FIRST_ITEM, |_, _| ret(1));
        e.register_double(TASK_MAP_GET_NEXT, move |e, a| {
            e.mem.set_u32(a[3], task);
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        let locks = record(&mut e, SPIN_LOCK_LOCK, 0);
        let unlocks = record(&mut e, SPIN_LOCK_UNLOCK, 0);
        let listed = record_word(&mut e, ARRAY_ADD);
        stub(&mut e, &[MAP_CLEAR]);
        let handed = record(&mut e, QUEUE_TASK, 0);
        set_setting(&mut e, false);
        fn_006c4ec0(&mut e, this);
        assert!(handed.borrow().is_empty());
        assert_eq!(*locks.borrow(), vec![vec![base + 0x160, 0]]);
        assert_eq!(*listed.borrow(), vec![vec![base + 0x13c, task]]);
        assert_eq!(*unlocks.borrow(), vec![vec![base + 0x160]]);
        assert_eq!(*started.borrow(), vec![vec![task]]);
    }

    /// Doors: `FORM_ID` is the word at `+0x0c` of a reference.
    #[test]
    fn doors_queued_to_open_queue_the_removal_on_every_recorded_navmesh() {
        let mut s = scene(false);
        list_doubles(&mut s.e);
        let base = s.this.addr();
        let first = obstacle(&mut s.e, 0x501, 0, true, &[0x8100, 0x8200]);
        let second = obstacle(&mut s.e, 0x502, 0, true, &[0x8300]);
        let array = reference_array(&mut s.e, 0x44, &[first, second]);
        keyed_map_get(&mut s.e, &[(base + 0x110, 0x44, array)]);
        list_chain(&mut s.e, base + 0xf8, &[0x44, 0x99]);
        let cleared = record(&mut s.e, LIST_CLEAR, 0);
        assert!(fn_006c4fb0(&mut s.e, s.this));
        // Operation 0x40 on the task of each recorded navmesh (the double
        // gives `info + 0x1000`), with the obstacle's pointer slot.
        let operations = s.operations.borrow();
        let seen: Vec<(u32, u32, u32)> = operations
            .iter()
            .map(|call| (call[0], call[2], call[3]))
            .collect();
        assert_eq!(
            seen,
            vec![
                (0x9100, 0x40, first),
                (0x9200, 0x40, first),
                (0x9300, 0x40, second)
            ]
        );
        // The entry moves from the closed to the open door map.
        assert_eq!(*s.map_remove.borrow(), vec![vec![base + 0x110, 0x44]]);
        assert_eq!(*s.map_set.borrow(), vec![vec![base + 0x100, 0x44, array]]);
        assert_eq!(*cleared.borrow(), vec![vec![base + 0xf8]]);
    }

    #[test]
    fn no_doors_queued_to_open_does_nothing() {
        let mut s = scene(false);
        list_doubles(&mut s.e);
        let cleared = record(&mut s.e, LIST_CLEAR, 0);
        assert!(!fn_006c4fb0(&mut s.e, s.this));
        assert!(cleared.borrow().is_empty());
        assert!(!fn_006c5190(&mut s.e, s.this));
        assert!(cleared.borrow().is_empty());
    }

    #[test]
    fn doors_queued_to_close_get_obstacles_and_leave_the_queue() {
        let mut s = scene(false);
        list_doubles(&mut s.e);
        let base = s.this.addr();
        let removed = removing_list_double(&mut s.e);
        // Two references: the first has a 3D node, the second does not.
        let node = object_with_slots(&mut s.e, &[(NODE_CHILDREN_SLOT, 0)]);
        let with_3d = object_with_slots(&mut s.e, &[(REFERENCE_NODE_SLOT, node)]);
        let without_3d = object_with_slots(&mut s.e, &[(REFERENCE_NODE_SLOT, 0)]);
        s.e.mem.set_u32(with_3d + 0x0c, 0x44);
        s.e.mem.set_u32(without_3d + 0x0c, 0x55);
        list_chain(&mut s.e, base + 0xf0, &[with_3d, without_3d]);
        // The new array gets one obstacle from the tree walk.
        let block = s.e.mem.alloc(0x1c);
        let allocated = record(&mut s.e, OPERATOR_NEW_SIZED, block);
        stub(&mut s.e, &[REFERENCE_COUNTED_CTOR, OBSTACLE_ARRAY_CTOR]);
        let obstacle_data = obstacle(&mut s.e, 0x501, 0, true, &[]);
        s.e.register_double(OBSTACLE_ARRAY_EMPTY, move |e, a| {
            set_array(e, a[0], &[obstacle_data]);
            Ret::default()
        });
        stub_ret(&mut s.e, COLLISION_OBJECT_OF_NODE, 0);
        assert!(fn_006c5190(&mut s.e, s.this));
        assert_eq!(*allocated.borrow(), vec![vec![0x1c]]);
        // The array knows its reference; operation 0x30 goes to the task of
        // the overlapped navmesh and is recorded in the obstacle.
        assert_eq!(s.e.mem.u32(block + 8), 0x44);
        let operations = s.operations.borrow();
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0][0], 0x9100);
        assert_eq!(operations[0][2], 0x30);
        assert_eq!(
            *s.array_add.borrow(),
            vec![vec![obstacle_data + 0x78, 0x8100]]
        );
        // The id leaves the open door map and the array enters the closed
        // one.
        assert_eq!(*s.map_remove.borrow(), vec![vec![base + 0x100, 0x44]]);
        assert_eq!(*s.map_set.borrow(), vec![vec![base + 0x110, 0x44, block]]);
        // The handled reference left the queue; the other one is skipped.
        assert_eq!(*removed.borrow(), vec![vec![base + 0xf0, with_3d]]);
        assert_eq!(s.e.mem.u32(base + 0xf0), without_3d);
    }

    #[test]
    fn queued_references_are_added_replaced_or_left_waiting() {
        let mut s = scene(false);
        list_doubles(&mut s.e);
        let base = s.this.addr();
        let removed = removing_list_double(&mut s.e);
        working_arrays(&mut s.e);
        // [known, new and usable, new but unusable, without 3D]
        let rigid_usable = object_with_slots(&mut s.e, &[(RIGID_BODY_SLOT_0X94, 1)]);
        let rigid_unusable = object_with_slots(&mut s.e, &[(RIGID_BODY_SLOT_0X94, 0)]);
        let node_usable = object_with_slots(&mut s.e, &[(NODE_CHILDREN_SLOT, 0)]);
        let node_unusable = object_with_slots(&mut s.e, &[(NODE_CHILDREN_SLOT, 0)]);
        let known = object_with_slots(&mut s.e, &[(REFERENCE_NODE_SLOT, 0)]);
        let usable = object_with_slots(&mut s.e, &[(REFERENCE_NODE_SLOT, node_usable)]);
        let unusable = object_with_slots(&mut s.e, &[(REFERENCE_NODE_SLOT, node_unusable)]);
        let without_3d = object_with_slots(&mut s.e, &[(REFERENCE_NODE_SLOT, 0)]);
        for (reference, id) in [(known, 1), (usable, 2), (unusable, 3), (without_3d, 4)] {
            s.e.mem.set_u32(reference + 0x0c, id);
        }
        list_chain(
            &mut s.e,
            base + 0x2c,
            &[known, usable, unusable, without_3d],
        );
        // Only the first reference has an entry in FormIDMap, with no
        // obstacles.
        let known_array = reference_array(&mut s.e, 1, &[]);
        keyed_map_get(&mut s.e, &[(base + 0x1c, 1, known_array)]);
        stub(
            &mut s.e,
            &[
                OBSTACLE_ARRAY_REFRESH,
                REFERENCE_COUNTED_CTOR,
                OBSTACLE_NAVMESH_ARRAY_CTOR,
                OBSTACLE_ARRAY_CTOR,
                OBSTACLE_NAVMESH_ARRAY_DTOR,
                REFERENCE_COUNTED_DTOR,
                REFERENCE_ARRAY_CTOR,
                REFERENCE_ARRAY_DTOR,
            ],
        );
        // The collision object of a node is the node itself, whose rigid
        // body is one of the two objects above (motion type 4).
        s.e.register(COLLISION_OBJECT_OF_NODE, |_, a| ret(a[0]));
        s.e.register_double(COLLISION_OBJECT_RIGID_BODY, move |_, a| {
            ret(if a[0] == node_usable {
                rigid_usable
            } else {
                rigid_unusable
            })
        });
        s.e.register_double(RIGID_BODY_MOTION_INFO, |e, a| {
            e.mem.set_u32(a[1], 4);
            ret(a[1])
        });
        s.e.register(MOTION_INFO_TYPE, |e, a| ret(e.mem.u32(a[0]) & 0x7f));
        stub_ret(&mut s.e, GET_WORD_AT_8, 0);
        // The usable reference's obstacles are added by `006c1190`, which
        // creates an array for it.
        let block = s.e.mem.alloc(0x1c);
        stub_ret(&mut s.e, OPERATOR_NEW_SIZED, block);
        s.e.register_double(FILL_REFERENCE_OBSTACLES, |_, _| Ret::default());
        fn_006c45e0(&mut s.e, s.this);
        // Replaced (known) and added (usable) references left the queue.
        assert_eq!(
            *removed.borrow(),
            vec![vec![base + 0x2c, known], vec![base + 0x2c, usable]]
        );
        // The known one was handled by `006c1920`; the usable one got an
        // array entered in FormIDMap.
        assert!(s.map_set.borrow().contains(&vec![base + 0x1c, 2, block]));
        assert!(!s.map_set.borrow().iter().any(|call| call[1] == 3));
        // The queue keeps the unusable one and the one without 3D.
        assert_eq!(s.e.mem.u32(base + 0x2c), unusable);
        let next = s.e.mem.u32(base + 0x2c + 4);
        assert_eq!(s.e.mem.u32(next), without_3d);
    }

    /// The doubles of the debug box: the math, the colour constructor and
    /// `MakeQuadBox`, which logs the eight corners (x, y, z each), the colour
    /// (r, g, b, a) and the last argument, and returns the shape `0xb500`.
    fn box_doubles(e: &mut Engine) -> Rc<RefCell<Vec<Vec<f32>>>> {
        add_math(e);
        e.register(COLOR_CONSTRUCT, |e, a| {
            for index in 0..4 {
                e.mem.set_u32(a[0] + 4 * index, a[1 + index as usize]);
            }
            ret(a[0])
        });
        let log: Rc<RefCell<Vec<Vec<f32>>>> = Rc::default();
        let shared = log.clone();
        e.register_double(MAKE_QUAD_BOX, move |e, a| {
            let mut row = vec![];
            for corner in a.iter().take(8) {
                row.extend(point_at(e, *corner));
            }
            for index in 0..4 {
                row.push(e.mem.f32(a[8] + 4 * index));
            }
            row.push(a[9] as f32);
            shared.borrow_mut().push(row);
            ret(0xb500)
        });
        log
    }

    #[test]
    fn the_debug_box_has_the_corners_of_box_min_and_box_max() {
        let mut e = engine();
        let boxes = box_doubles(&mut e);
        let this = manager(&mut e);
        let data = obstacle(&mut e, 0x501, 0, true, &[]);
        set_point(&mut e, data + 0x40, [1.0, 2.0, 3.0]);
        set_point(&mut e, data + 0x4c, [4.0, 5.0, 6.0]);
        let slot = slot_with(&mut e, data);
        let node = slot_with(&mut e, 0);
        let center = record(&mut e, SHAPE_SET_CENTER, 0);
        let orientation = record(&mut e, SHAPE_SET_ORIENTATION, 0);
        nav_mesh_obstacle_manager_create_bounding_box3d_node(&mut e, this, slot, node);
        // The corners go to `MakeQuadBox` in the order H, G, C, D, E, F, B, A,
        // then the colour (1, 0, 1, 0.5) and 1.
        assert_eq!(
            *boxes.borrow(),
            vec![vec![
                1.0, 5.0, 6.0, 4.0, 5.0, 6.0, 4.0, 2.0, 6.0, 1.0, 2.0, 6.0, 1.0, 5.0, 3.0, 4.0,
                5.0, 3.0, 4.0, 2.0, 3.0, 1.0, 2.0, 3.0, 1.0, 0.0, 1.0, 0.5, 1.0
            ]]
        );
        // The shape is placed at the obstacle and kept by it and by `node`.
        assert_eq!(*center.borrow(), vec![vec![0xb500, data + 0x10]]);
        assert_eq!(*orientation.borrow(), vec![vec![0xb500, data + 0x1c]]);
        assert_eq!(e.mem.u32(data + 0x88), 0xb500);
        assert_eq!(e.mem.u32(node), 0xb500);
    }

    /// A manager whose obstacle root node (`spObstacleRootNode`) is an
    /// object with the attach (`0xdc`) and detach (`0xe8`) slots logged as
    /// `(node, argument, ...)`.
    fn root_scene(with_root: bool) -> (Engine, Ptr<NavMeshObstacleManager>, u32, Log, Log) {
        let mut e = engine();
        let boxes = box_doubles(&mut e);
        let _ = boxes;
        let this = manager(&mut e);
        let root = object_with_slots(&mut e, &[(0xdc, 0), (0xe8, 0)]);
        if with_root {
            e.mem.set_u32(this.addr() + 0x19c, root);
        }
        let attached = record(&mut e, root + 0xdc, 0);
        let detached = record(&mut e, root + 0xe8, 0);
        stub(&mut e, &[SHAPE_SET_CENTER, SHAPE_SET_ORIENTATION]);
        (e, this, root, attached, detached)
    }

    #[test]
    fn a_new_debug_box_replaces_the_old_one_under_the_root() {
        let (mut e, this, root, attached, detached) = root_scene(true);
        let data = obstacle(&mut e, 0x501, 0, true, &[]);
        e.mem.set_u32(data + 0x88, 0xb400);
        let slot = slot_with(&mut e, data);
        let properties = record(&mut e, UPDATE_PROPERTIES, 0);
        stub(&mut e, &[UPDATE_DATA_CONSTRUCT]);
        let updates = record(&mut e, NODE_UPDATE, 0);
        nav_mesh_obstacle_manager_add_bounding_box3d_node(&mut e, this, slot);
        assert_eq!(*detached.borrow(), vec![vec![root, 0xb400]]);
        assert_eq!(*attached.borrow(), vec![vec![root, 0xb500, 1]]);
        assert_eq!(e.mem.u32(data + 0x88), 0xb500);
        assert_eq!(*properties.borrow(), vec![vec![root]]);
        assert_eq!(updates.borrow().len(), 1);
        assert_eq!(updates.borrow()[0][0], root);
        // An obstacle without a box detaches nothing.
        let fresh = obstacle(&mut e, 0x502, 0, true, &[]);
        let slot = slot_with(&mut e, fresh);
        nav_mesh_obstacle_manager_add_bounding_box3d_node(&mut e, this, slot);
        assert_eq!(detached.borrow().len(), 1);
        assert_eq!(attached.borrow().len(), 2);
    }

    #[test]
    fn hiding_the_obstacles_removes_the_properties_of_the_root() {
        let (mut e, this, root, _, _) = root_scene(true);
        e.mem.set_u8(this.addr() + 0x198, 1);
        let removed = record(&mut e, NODE_REMOVE_PROPERTIES, 0);
        nav_mesh_obstacle_manager_draw_obstacles(&mut e, this, 0);
        assert_eq!(*removed.borrow(), vec![vec![root]]);
        assert_eq!(e.mem.u8(this.addr() + 0x198), 0);
        // Without a root there is nothing to remove.
        let (mut e, this, _, _, _) = root_scene(false);
        let removed = record(&mut e, NODE_REMOVE_PROPERTIES, 0);
        e.mem.set_u8(this.addr() + 0x198, 1);
        nav_mesh_obstacle_manager_draw_obstacles(&mut e, this, 0);
        assert!(removed.borrow().is_empty());
        assert_eq!(e.mem.u8(this.addr() + 0x198), 0);
    }

    #[test]
    fn showing_the_obstacles_gives_every_obstacle_a_box_under_the_root() {
        let (mut e, this, root, attached, _) = root_scene(true);
        let base = this.addr();
        let removed = record(&mut e, NODE_REMOVE_PROPERTIES, 0);
        let properties = record(&mut e, UPDATE_PROPERTIES, 0);
        stub(&mut e, &[UPDATE_DATA_CONSTRUCT, NODE_UPDATE]);
        // One entry in FormIDMap with two obstacles.
        let table = hashed_map(&mut e, base + 0x1c);
        let _ = table;
        let first = obstacle(&mut e, 0x501, 0, true, &[]);
        let second = obstacle(&mut e, 0x502, 0, true, &[]);
        let array = reference_array(&mut e, 3, &[first, second]);
        let item = map_item(&mut e, 0, 3, array);
        stub_ret(&mut e, MAP_FIRST_ITEM, item);
        nav_mesh_obstacle_manager_draw_obstacles(&mut e, this, 1);
        // An existing root only has its properties removed first.
        assert_eq!(*removed.borrow(), vec![vec![root]]);
        assert_eq!(*attached.borrow(), vec![vec![root, 0xb500, 1]; 2]);
        assert_eq!(e.mem.u32(first + 0x88), 0xb500);
        assert_eq!(e.mem.u32(second + 0x88), 0xb500);
        assert_eq!(*properties.borrow(), vec![vec![root]]);
        assert_eq!(e.mem.u8(base + 0x198), 1);
    }

    #[test]
    fn showing_the_obstacles_without_a_root_makes_the_root_and_its_properties() {
        let (mut e, this, _, _, _) = root_scene(false);
        let base = this.addr();
        stub_ret(&mut e, MAP_FIRST_ITEM, 0);
        let (node_block, alpha_block, material_block) =
            (e.mem.alloc(0xac), e.mem.alloc(0x1c), e.mem.alloc(0x4c));
        let sizes = record(&mut e, OPERATOR_NEW_SIZED, 0);
        e.register_double(OPERATOR_NEW_SIZED, {
            let sizes = sizes.clone();
            move |_, a| {
                sizes.borrow_mut().push(a.to_vec());
                ret(match a[0] {
                    0xac => node_block,
                    0x1c => alpha_block,
                    _ => material_block,
                })
            }
        });
        e.register(NODE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(ALPHA_PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
        e.register(MATERIAL_PROPERTY_CONSTRUCT, |_, a| ret(a[0]));
        let alpha_enabled = record(&mut e, ALPHA_PROPERTY_ENABLE, 0);
        let colors = Rc::new(RefCell::new(vec![]));
        let shared = colors.clone();
        e.register_double(MATERIAL_SET_COLOR, move |e, a| {
            shared.borrow_mut().push((a[0], point_at(e, a[1])));
            Ret::default()
        });
        let alphas = record(&mut e, MATERIAL_SET_ALPHA, 0);
        let attached_properties = record(&mut e, ATTACH_PROPERTY, 0);
        // The scene root is the word at `+0x0c` of the TES object.
        let scene_root = object_with_slots(&mut e, &[(0xdc, 0)]);
        let tes = e.mem.alloc(0x20);
        e.mem.set_u32(tes + 0x0c, scene_root);
        e.set_global::<u32>(TES_POINTER, tes);
        let scene_attached = record(&mut e, scene_root + 0xdc, 0);
        stub(&mut e, &[UPDATE_PROPERTIES, UPDATE_DATA_CONSTRUCT]);
        let updates = record(&mut e, NODE_UPDATE, 0);
        nav_mesh_obstacle_manager_draw_obstacles(&mut e, this, 1);
        assert_eq!(*sizes.borrow(), vec![vec![0xac], vec![0x1c], vec![0x4c]]);
        assert_eq!(e.mem.u32(base + 0x19c), node_block);
        assert_eq!(*alpha_enabled.borrow(), vec![vec![alpha_block, 1]]);
        assert_eq!(*colors.borrow(), vec![(material_block, [1.0, 0.0, 1.0])]);
        assert_eq!(
            *alphas.borrow(),
            vec![vec![material_block, 0.25f32.to_bits()]]
        );
        assert_eq!(
            *attached_properties.borrow(),
            vec![
                vec![node_block, alpha_block],
                vec![node_block, material_block]
            ]
        );
        // The new root goes under the scene root, which is updated, and the
        // root itself is updated at the end.
        assert_eq!(
            *scene_attached.borrow(),
            vec![vec![scene_root, node_block, 1]]
        );
        assert_eq!(updates.borrow().len(), 2);
        assert_eq!(updates.borrow()[0][0], scene_root);
        assert_eq!(updates.borrow()[1][0], node_block);
        assert_eq!(e.mem.u8(base + 0x198), 1);
    }

    // ---- Session 3 (`006c5c50`..`006c6e60`) ----

    /// `operator new` handing out blocks of game memory; logs (size, block).
    fn record_allocations(e: &mut Engine) -> Log {
        let log: Log = Rc::default();
        let shared = log.clone();
        e.register_double(OPERATOR_NEW, move |e, a| {
            let block = e.mem.alloc(a[0].clamp(8, 0x1000));
            shared.borrow_mut().push(vec![a[0], block]);
            ret(block)
        });
        log
    }

    /// The doubles the queue code needs besides the allocator: the scope guard
    /// and the base constructors returning their `this`.
    fn queue_engine() -> (Engine, Log) {
        let mut e = engine();
        stub(
            &mut e,
            &[SCOPE_GUARD_CTOR, SCOPE_GUARD_DTOR, QUEUE_BASE_CTOR],
        );
        stub(&mut e, &[SPIN_LOCK_LOCK, SPIN_LOCK_UNLOCK, QUEUE_BASE_DTOR]);
        e.register(REFS_LIST_CTOR, |_, a| ret(a[0]));
        e.register(QUEUE_INTERFACE_MANAGER_CTOR, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        let allocations = record_allocations(&mut e);
        (e, allocations)
    }

    #[test]
    fn removing_the_debug_box_detaches_it_from_the_root_and_clears_it() {
        let mut e = engine();
        let manager = e.new_object::<NavMeshObstacleManager>();
        let root = object_with_slots(&mut e, &[(0xe8, 0)]);
        let detached = record(&mut e, root + 0xe8, 0);
        let cleared = record(&mut e, NI_POINTER_SET, 0);
        let box_node = 0x7770;
        let data = e.mem.alloc(0x8c);
        e.mem.set_u32(data + 0x88, box_node);
        let obstacle = slot_with(&mut e, data);
        // Without a root node nothing happens.
        fn_006c5c50(&mut e, manager, obstacle);
        assert!(detached.borrow().is_empty());
        e.mem.set_u32(manager.addr() + 0x19c, root);
        fn_006c5c50(&mut e, manager, obstacle);
        assert_eq!(*detached.borrow(), vec![vec![root, box_node]]);
        assert_eq!(*cleared.borrow(), vec![vec![data + 0x88, 0]]);
        // An obstacle without a box is left alone.
        e.mem.set_u32(data + 0x88, 0);
        fn_006c5c50(&mut e, manager, obstacle);
        assert_eq!(detached.borrow().len(), 1);
    }

    /// An engine for `006c5ce0`: the group takes the tasklet on the `accept`-th
    /// offer (0: never) and `find` is what the array search answers.
    fn hand_over_engine(accept: u32, find: i32) -> (Engine, Log, Log, Log, Log) {
        let mut e = engine();
        stub(&mut e, &[SPIN_LOCK_LOCK, SPIN_LOCK_UNLOCK]);
        stub_ret(&mut e, TASKLET_MANAGER_INSTANCE, 0x7000);
        let sleeps = record(&mut e, SLEEP, 0);
        let removed = record(&mut e, ARRAY_REMOVE, 0);
        let added = record_word(&mut e, ARRAY_ADD);
        stub_ret(&mut e, ARRAY_FIND, find as u32);
        let offers: Log = Rc::default();
        let shared = offers.clone();
        e.register_double(TASKLET_GROUP_ADD, move |e, a| {
            let mut offers = shared.borrow_mut();
            // The tasklet is built (vtable set, data stored) before it is offered.
            offers.push(vec![a[0], a[1], a[3], e.mem.u32(a[2]), e.mem.u32(a[2] + 4)]);
            ret(u32::from(offers.len() as u32 == accept))
        });
        e.register(TASKLET_SET_DATA, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            ret(a[0])
        });
        (e, offers, sleeps, removed, added)
    }

    #[test]
    fn a_task_the_group_takes_stays_in_the_background_list() {
        let (mut e, offers, sleeps, removed, added) = hand_over_engine(3, -1);
        let manager = e.new_object::<NavMeshObstacleManager>();
        let base = manager.addr();
        assert!(fn_006c5ce0(&mut e, manager, 0x4242));
        // Added to BackgroundTasks (the stack word holds the task).
        assert_eq!(
            *added.borrow(),
            vec![vec![
                base + NavMeshObstacleManager::BackgroundTasks.off,
                0x4242
            ]]
        );
        // Three offers of the same tasklet, with a Sleep(0) after each refusal.
        let group = base + NavMeshObstacleManager::ObstacleTaskletGroup.off;
        assert_eq!(offers.borrow().len(), 3);
        for offer in offers.borrow().iter() {
            assert_eq!(offer[..3], [0x7000, group, 0]);
            assert_eq!(offer[3..], [TASKLET_VTABLE, 0x4242]);
        }
        assert_eq!(sleeps.borrow().len(), 2);
        assert!(removed.borrow().is_empty());
    }

    #[test]
    fn a_task_the_group_never_takes_is_taken_out_of_the_list() {
        let (mut e, offers, sleeps, removed, _) = hand_over_engine(0, 2);
        let manager = e.new_object::<NavMeshObstacleManager>();
        start_log(&mut e);
        assert!(!fn_006c5ce0(&mut e, manager, 0x4242));
        assert_eq!(offers.borrow().len(), 100);
        assert_eq!(sleeps.borrow().len(), 99);
        let tasks = manager.addr() + NavMeshObstacleManager::BackgroundTasks.off;
        assert_eq!(*removed.borrow(), vec![vec![tasks, 2, 1]]);
        let finds = calls(&e, &[ARRAY_FIND]);
        assert_eq!(finds.len(), 1);
        assert_eq!(finds[0].1[0], tasks);
        assert_eq!(finds[0].1[2..], [0, TASK_COMPARATOR]);
        // The lock is taken and released around the add and around the removal.
        let locks = calls(&e, &[SPIN_LOCK_LOCK, SPIN_LOCK_UNLOCK]);
        assert_eq!(locks.len(), 4);
        // Not found: nothing is removed.
        let (mut e, _, _, removed, _) = hand_over_engine(0, -1);
        assert!(!fn_006c5ce0(&mut e, manager, 0x4242));
        assert!(removed.borrow().is_empty());
    }

    #[test]
    fn the_tasklet_constructor_and_destructor_set_the_vtable() {
        let mut e = engine();
        let tasklet = e.new_object::<BSTasklet>();
        e.mem.set_u32(tasklet.addr() + 4, 0x55);
        assert_eq!(fn_006c5e40(&mut e, tasklet), tasklet);
        assert_eq!(e.mem.u32(tasklet.addr()), TASKLET_VTABLE);
        assert_eq!(e.mem.u32(tasklet.addr() + 4), 0);
        e.mem.set_u32(tasklet.addr(), 0);
        e.mem.set_u32(tasklet.addr() + 4, 0x55);
        fn_006c5e70(&mut e, tasklet);
        assert_eq!(e.mem.u32(tasklet.addr()), TASKLET_VTABLE);
        assert_eq!(e.mem.u32(tasklet.addr() + 4), 0x55);
    }

    #[test]
    fn the_tasklet_deleting_destructor_frees_only_when_asked() {
        let mut e = engine();
        let tasklet = e.new_object::<BSTasklet>();
        let freed = record(&mut e, OPERATOR_DELETE, 0);
        assert_eq!(
            bs_tasklet_scalar_deleting_destructor(&mut e, tasklet, 0),
            tasklet
        );
        assert!(freed.borrow().is_empty());
        assert_eq!(e.mem.u32(tasklet.addr()), TASKLET_VTABLE);
        // Only bit 0 matters.
        assert_eq!(
            bs_tasklet_scalar_deleting_destructor(&mut e, tasklet, 3),
            tasklet
        );
        assert_eq!(*freed.borrow(), vec![vec![tasklet.addr()]]);
    }

    #[test]
    fn the_group_add_goes_through_the_tasklet_manager() {
        let mut e = engine();
        stub_ret(&mut e, TASKLET_MANAGER_INSTANCE, 0x7000);
        let add = record(&mut e, TASKLET_GROUP_ADD, 1);
        assert!(fn_006c5ec0(&mut e, Ptr::new(0x1230), 0x5550, 1));
        assert_eq!(*add.borrow(), vec![vec![0x7000, 0x1230, 0x5550, 1]]);
        stub_ret(&mut e, TASKLET_GROUP_ADD, 0);
        assert!(!fn_006c5ec0(&mut e, Ptr::new(0x1230), 0x5550, 0));
    }

    #[test]
    fn the_navmesh_task_is_found_or_made_and_entered_in_the_map() {
        let mut e = engine();
        let manager = e.new_object::<NavMeshObstacleManager>();
        let map = manager.addr() + NavMeshObstacleManager::CurrentNavMeshTaskMap.off;
        let info = slot_with(&mut e, 0x31);
        // The key is present: its task comes back through the out slot.
        e.register(TASK_MAP_GET_AT, |e, a| {
            e.mem.set_u32(a[2], 0xbeef);
            ret(1)
        });
        let set = record(&mut e, TASK_MAP_SET_AT, 0);
        assert_eq!(fn_006c5ef0(&mut e, manager, info), 0xbeef);
        assert!(set.borrow().is_empty());
        // Missing: a 0x58-byte task is built for the info and entered.
        let get = record(&mut e, TASK_MAP_GET_AT, 0);
        let sizes = record_allocations(&mut e);
        let built = record(&mut e, TASK_CONSTRUCT, 0xcafe);
        assert_eq!(fn_006c5ef0(&mut e, manager, info), 0xcafe);
        assert_eq!(get.borrow()[0][..2], [map, 0x31]);
        assert_eq!(sizes.borrow()[0][0], 0x58);
        assert_eq!(*built.borrow(), vec![vec![sizes.borrow()[0][1], info]]);
        assert_eq!(*set.borrow(), vec![vec![map, 0x31, 0xcafe]]);
        // A failed allocation enters a null task.
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(fn_006c5ef0(&mut e, manager, info), 0);
        assert_eq!(set.borrow()[1], vec![map, 0x31, 0]);
    }

    /// A map constructor builds the bucket array and clears it; returns the
    /// map.
    fn check_map_construction(
        construct: fn(&mut Engine, Ptr<NiTMap>, u32) -> Ptr<NiTMap>,
        vtable: u32,
    ) {
        let mut e = engine();
        let map = e.new_object::<NiTMap>();
        e.mem.set_u32(map.addr() + 0xc, 9);
        let allocated = record(&mut e, BUCKETS_ALLOCATE, 0x6000);
        let cleared = record(&mut e, MEMSET, 0);
        assert_eq!(construct(&mut e, map, 0x25), map);
        assert_eq!(e.mem.u32(map.addr()), vtable);
        assert_eq!(e.mem.u32(map.addr() + 4), 0x25);
        assert_eq!(e.mem.u32(map.addr() + 8), 0x6000);
        assert_eq!(e.mem.u32(map.addr() + 0xc), 0);
        assert_eq!(*allocated.borrow(), vec![vec![0x94]]);
        assert_eq!(*cleared.borrow(), vec![vec![0x6000, 0, 0x94]]);
    }

    #[test]
    fn the_reference_map_base_constructor_builds_the_buckets() {
        check_map_construction(fn_006c6260, REFERENCE_MAP_BASE_VTABLE);
    }

    #[test]
    fn the_rigid_body_map_base_constructor_builds_the_buckets() {
        check_map_construction(fn_006c68b0, RIGID_BODY_MAP_BASE_VTABLE);
    }

    #[test]
    fn the_task_map_base_constructor_builds_the_buckets() {
        check_map_construction(fn_006c6b10, TASK_MAP_BASE_VTABLE);
    }

    #[test]
    fn the_reference_map_constructor_sets_its_own_vtable() {
        check_map_construction(fn_006c5fc0, REFERENCE_MAP_VTABLE);
    }

    #[test]
    fn the_rigid_body_map_constructor_sets_its_own_vtable() {
        check_map_construction(fn_006c6070, RIGID_BODY_MAP_VTABLE);
    }

    #[test]
    fn the_task_map_constructor_sets_its_own_vtable() {
        check_map_construction(fn_006c60a0, TASK_MAP_VTABLE);
    }

    /// A map destructor test: the vtable at the time of the clear, and the
    /// calls that follow.
    fn map_destruction_engine() -> (Engine, Ptr<NiTMap>, Rc<RefCell<Vec<u32>>>) {
        let mut e = engine();
        let map = e.new_object::<NiTMap>();
        e.mem.set_u32(map.addr() + 8, 0x6000);
        let vtables = Rc::new(RefCell::new(vec![]));
        let shared = vtables.clone();
        e.register_double(MAP_CLEAR, move |e, a| {
            shared.borrow_mut().push(e.mem.u32(a[0]));
            Ret::default()
        });
        stub(&mut e, &[BUCKETS_FREE, REFERENCE_MAP_BASE_DTOR]);
        (e, map, vtables)
    }

    #[test]
    fn the_reference_map_destructor_clears_and_runs_the_base_destructor() {
        let (mut e, map, vtables) = map_destruction_engine();
        start_log(&mut e);
        fn_006c6350(&mut e, map);
        assert_eq!(*vtables.borrow(), vec![REFERENCE_MAP_VTABLE]);
        assert_eq!(
            calls(&e, &[REFERENCE_MAP_BASE_DTOR]),
            vec![(REFERENCE_MAP_BASE_DTOR, vec![map.addr()])]
        );
    }

    #[test]
    fn the_rigid_body_map_destructor_clears_twice_and_frees_the_buckets() {
        let (mut e, map, vtables) = map_destruction_engine();
        start_log(&mut e);
        fn_006c6a80(&mut e, map);
        // The derived clear, then the base's after it took the base vtable.
        assert_eq!(
            *vtables.borrow(),
            vec![RIGID_BODY_MAP_VTABLE, RIGID_BODY_MAP_BASE_VTABLE]
        );
        assert_eq!(
            calls(&e, &[BUCKETS_FREE]),
            vec![(BUCKETS_FREE, vec![0x6000])]
        );
    }

    #[test]
    fn the_task_map_destructor_clears_twice_and_frees_the_buckets() {
        let (mut e, map, vtables) = map_destruction_engine();
        start_log(&mut e);
        fn_006c6b80(&mut e, map);
        assert_eq!(
            *vtables.borrow(),
            vec![TASK_MAP_VTABLE, TASK_MAP_BASE_VTABLE]
        );
        assert_eq!(
            calls(&e, &[BUCKETS_FREE]),
            vec![(BUCKETS_FREE, vec![0x6000])]
        );
    }

    #[test]
    fn the_map_base_destructors_free_the_buckets() {
        let (mut e, map, vtables) = map_destruction_engine();
        start_log(&mut e);
        fn_006c6ae0(&mut e, map);
        assert_eq!(e.mem.u32(map.addr()), RIGID_BODY_MAP_BASE_VTABLE);
        fn_006c6be0(&mut e, map);
        assert_eq!(e.mem.u32(map.addr()), TASK_MAP_BASE_VTABLE);
        assert_eq!(
            *vtables.borrow(),
            vec![RIGID_BODY_MAP_BASE_VTABLE, TASK_MAP_BASE_VTABLE]
        );
        assert_eq!(calls(&e, &[BUCKETS_FREE]).len(), 2);
    }

    #[test]
    fn the_map_deleting_destructors_delete_only_when_asked() {
        type Destructor = fn(&mut Engine, Ptr<NiTMap>, u32) -> Ptr<NiTMap>;
        let destructors: [Destructor; 3] = [
            ni_tmap_reference_obstacle_array_scalar_deleting_destructor,
            ni_tmap_rigid_body_obstacle_data_scalar_deleting_destructor,
            ni_tmap_obstacle_task_data_scalar_deleting_destructor,
        ];
        for destructor in destructors {
            let (mut e, map, _) = map_destruction_engine();
            let freed = record(&mut e, OPERATOR_DELETE, 0);
            assert_eq!(destructor(&mut e, map, 0), map);
            assert!(freed.borrow().is_empty());
            assert_eq!(destructor(&mut e, map, 1), map);
            assert_eq!(*freed.borrow(), vec![vec![map.addr()]]);
        }
    }

    #[test]
    fn the_map_hash_is_the_key_modulo_the_bucket_count() {
        let mut e = engine();
        let map = e.new_object::<NiTMap>();
        e.mem.set_u32(map.addr() + 4, 0x25);
        assert_eq!(fn_006c6a60(&mut e, map, 100), 26);
        assert_eq!(fn_006c6a60(&mut e, map, 0x25), 0);
    }

    /// A map object whose virtual slots are doubles: hash (`0x04`, key modulo
    /// 4), key equality (`0x08`), assignment of an item (`0x0c`) and a new item
    /// (`0x14`).
    fn set_at_fixture(e: &mut Engine) -> (Ptr<NiTMap>, u32, Log, u32) {
        let object = object_with_slots(e, &[(4, 0), (8, 0), (0xc, 0), (0x14, 0)]);
        let map = Ptr::<NiTMap>::new(object);
        let table = e.mem.alloc(16);
        e.mem.set_u32(object + 4, 4);
        e.mem.set_u32(object + 8, table);
        e.register(object + 4, |_, a| ret(a[1] % 4));
        e.register(object + 8, |_, a| ret(u32::from(a[1] == a[2])));
        let assigned = record(e, object + 0xc, 0);
        let item = e.mem.alloc(12);
        stub_ret(e, object + 0x14, item);
        (map, table, assigned, item)
    }

    #[test]
    fn set_at_replaces_the_value_of_an_existing_key() {
        let mut e = engine();
        let (map, table, assigned, _) = set_at_fixture(&mut e);
        // Bucket 1 holds an item for key 5.
        let existing = e.mem.alloc(12);
        e.mem.set_u32(existing + 4, 5);
        e.mem.set_u32(existing + 8, 0x11);
        e.mem.set_u32(table + 4, existing);
        let released = record(&mut e, NI_POINTER_RELEASE, 0);
        fn_006c6920(&mut e, map, 5, 0x77);
        assert_eq!(e.mem.u32(existing + 8), 0x77);
        assert!(assigned.borrow().is_empty());
        assert_eq!(e.mem.u32(map.addr() + 0xc), 0);
        // The by-value argument is destroyed.
        assert_eq!(released.borrow().len(), 1);
    }

    #[test]
    fn set_at_links_a_new_item_at_the_head_of_its_bucket() {
        let mut e = engine();
        let (map, table, assigned, item) = set_at_fixture(&mut e);
        // Bucket 1 holds an item for another key (9 is 1 modulo 4 as well).
        let other = e.mem.alloc(12);
        e.mem.set_u32(other + 4, 9);
        e.mem.set_u32(table + 4, other);
        e.mem.set_u32(map.addr() + 0xc, 3);
        let released = record(&mut e, NI_POINTER_RELEASE, 0);
        fn_006c6920(&mut e, map, 5, 0x77);
        // The value reaches the assignment slot as a fresh copy.
        assert_eq!(*assigned.borrow(), vec![vec![map.addr(), item, 5, 0x77]]);
        assert_eq!(e.mem.u32(item), other);
        assert_eq!(e.mem.u32(table + 4), item);
        assert_eq!(e.mem.u32(map.addr() + 0xc), 4);
        assert_eq!(released.borrow().len(), 1);
    }

    #[test]
    fn the_obstacle_list_contains_walks_until_the_item_matches() {
        let mut e = engine();
        e.register(LIST_NODE_DIFFERS, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])))
        });
        let nodes: Vec<u32> = (0..3).map(|_| e.mem.alloc(8)).collect();
        for (i, node) in nodes.iter().enumerate() {
            e.mem.set_u32(*node, i as u32 + 1);
            e.mem
                .set_u32(*node + 4, nodes.get(i + 1).copied().unwrap_or(0));
        }
        let wanted = slot_with(&mut e, 3);
        assert!(fn_006c6400(&mut e, Ptr::new(nodes[0]), wanted));
        let missing = slot_with(&mut e, 9);
        assert!(!fn_006c6400(&mut e, Ptr::new(nodes[0]), missing));
    }

    #[test]
    fn the_array_constructors_and_destructors_set_vtables_and_call_the_helpers() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        let init = record(&mut e, OBSTACLE_ARRAY_INIT, 0);
        let init_simple = record(&mut e, SIMPLE_ARRAY_INIT, 0);
        let emptied = record(&mut e, OBSTACLE_ARRAY_EMPTY, 0);
        let cleared = record(&mut e, ARRAY_CLEAR, 0);
        assert_eq!(fn_006c6160(&mut e, array), array);
        assert_eq!(e.mem.u32(array.addr()), OBSTACLE_ARRAY_VTABLE);
        assert_eq!(*init.borrow(), vec![vec![array.addr(), 0, 0]]);
        e.mem.set_u32(array.addr(), 0);
        bs_simple_array_obstacle_data_destructor(&mut e, array);
        assert_eq!(e.mem.u32(array.addr()), OBSTACLE_ARRAY_VTABLE);
        assert_eq!(*emptied.borrow(), vec![vec![array.addr(), 1]]);
        assert_eq!(fn_006c6c10(&mut e, array), array);
        assert_eq!(e.mem.u32(array.addr()), TASK_ARRAY_VTABLE);
        assert_eq!(*init_simple.borrow(), vec![vec![array.addr(), 0, 0]]);
        e.mem.set_u32(array.addr(), 0);
        fn_006c6c40(&mut e, array);
        assert_eq!(e.mem.u32(array.addr()), TASK_ARRAY_VTABLE);
        assert_eq!(*cleared.borrow(), vec![vec![array.addr(), 1]]);
    }

    #[test]
    fn the_navmesh_holder_array_uses_the_pool_allocator() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        stub_ret(&mut e, HOLDER_ARRAY_BASE_CTOR, 0);
        stub_ret(&mut e, ARRAY_POOL, 0x011f_6238);
        let allocator = record(&mut e, ARRAY_ALLOCATOR, 0xa110);
        let init = record(&mut e, HOLDER_ARRAY_INIT, 0);
        assert_eq!(fn_006c6c60(&mut e, array), array);
        assert_eq!(e.mem.u32(array.addr()), NAVMESH_HOLDER_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(array.addr() + 0x10), 0xa110);
        assert_eq!(*allocator.borrow(), vec![vec![0x011f_6238]]);
        assert_eq!(*init.borrow(), vec![vec![array.addr(), 0, 0]]);
        let cleared = record(&mut e, HOLDER_ARRAY_CLEAR, 0);
        let base = record(&mut e, HOLDER_ARRAY_BASE_DTOR, 0);
        e.mem.set_u32(array.addr(), 0);
        fn_006c6ce0(&mut e, array);
        assert_eq!(e.mem.u32(array.addr()), NAVMESH_HOLDER_ARRAY_VTABLE);
        assert_eq!(*cleared.borrow(), vec![vec![array.addr(), 1]]);
        assert_eq!(*base.borrow(), vec![vec![array.addr()]]);
    }

    #[test]
    fn the_cell_array_takes_the_given_allocator_or_the_pools() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        let base = record(&mut e, CELL_ARRAY_BASE_CTOR, 0);
        let init = record(&mut e, SIMPLE_ARRAY_INIT, 0);
        let pool = record(&mut e, ARRAY_POOL, 0x011f_6238);
        stub_ret(&mut e, ARRAY_ALLOCATOR, 0xa110);
        assert_eq!(fn_006c6d40(&mut e, array, 0x20, 0, 0x5150), array);
        assert_eq!(e.mem.u32(array.addr()), CELL_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(array.addr() + 0x10), 0x5150);
        assert!(pool.borrow().is_empty());
        assert_eq!(*base.borrow(), vec![vec![array.addr()]]);
        assert_eq!(*init.borrow(), vec![vec![array.addr(), 0x20, 0]]);
        fn_006c6d40(&mut e, array, 4, 8, 0);
        assert_eq!(e.mem.u32(array.addr() + 0x10), 0xa110);
        assert_eq!(pool.borrow().len(), 1);
        assert_eq!(init.borrow()[1], vec![array.addr(), 4, 8]);
    }

    #[test]
    fn the_cell_array_destructors_clear_the_array() {
        let mut e = engine();
        let array = e.new_object::<BSSimpleArray>();
        let cleared = record(&mut e, ARRAY_CLEAR, 0);
        fn_006c6de0(&mut e, array);
        assert_eq!(e.mem.u32(array.addr()), CELL_ARRAY_BASE_VTABLE);
        fn_006c6e00(&mut e, array);
        // The derived destructor clears, then the base one clears and leaves
        // its own vtable.
        assert_eq!(e.mem.u32(array.addr()), CELL_ARRAY_BASE_VTABLE);
        assert_eq!(cleared.borrow().len(), 3);
        assert!(cleared.borrow().iter().all(|c| *c == vec![array.addr(), 1]));
    }

    #[test]
    fn the_queue_constructor_builds_the_list_the_buffers_and_the_manager() {
        let (mut e, allocations) = queue_engine();
        let queue = e.new_object::<LockFreeQueue>();
        e.mem.set_u32(queue.addr() + 0x18, 5);
        start_log(&mut e);
        assert_eq!(fn_006c6450(&mut e, queue, 7, 8), queue);
        let base = queue.addr();
        assert_eq!(e.mem.u32(base), LOCK_FREE_QUEUE_VTABLE);
        let allocations = allocations.borrow();
        // Head node (8), pointer array (7 * 2 words), interface manager (0x10).
        assert_eq!(
            allocations.iter().map(|a| a[0]).collect::<Vec<_>>(),
            [8, 56, 0x10]
        );
        assert_eq!(e.mem.u32(base + 4), allocations[0][1]);
        assert_eq!(e.mem.u32(base + 8), allocations[0][1]);
        assert_eq!(e.mem.u32(base + 0xc), 8);
        assert_eq!(e.mem.u32(base + 0x10), allocations[1][1]);
        assert_eq!(e.mem.u32(base + 0x14), allocations[2][1]);
        assert_eq!(e.mem.u32(base + 0x18), 0);
        // The manager gets the capacity.
        assert_eq!(e.mem.u32(allocations[2][1]), 7);
        let guard = calls(&e, &[SCOPE_GUARD_CTOR]);
        assert_eq!(guard[0].1[1..], [6, 1, 0x0101_73f8, 0xfd]);
        assert_eq!(calls(&e, &[REFS_LIST_CTOR])[0].1, vec![base + 0x20]);
    }

    #[test]
    fn the_queue_buffer_size_saturates() {
        let (mut e, allocations) = queue_engine();
        let queue = e.new_object::<LockFreeQueue>();
        fn_006c6450(&mut e, queue, 0x2000_0000, 1);
        assert_eq!(allocations.borrow()[1][0], u32::MAX);
        fn_006c6450(&mut e, queue, 0x1fff_ffff, 1);
        assert_eq!(allocations.borrow()[4][0], 0xffff_fff8);
    }

    #[test]
    fn allocate_interface_builds_the_interface_over_the_threads_slots() {
        let (mut e, allocations) = queue_engine();
        let queue = e.new_object::<LockFreeQueue>();
        e.mem.set_u32(queue.addr() + 0x10, 0x5000);
        let built = record(&mut e, QUEUE_INTERFACE_CTOR, 0x9990);
        start_log(&mut e);
        assert_eq!(
            lock_free_queue_rigid_body_allocate_interface(&mut e, queue, 3),
            0x9990
        );
        let block = allocations.borrow()[0].clone();
        assert_eq!(block[0], 0x14);
        assert_eq!(
            *built.borrow(),
            vec![vec![block[1], queue.addr(), 0x5018, 0x501c]]
        );
        let guard = calls(&e, &[SCOPE_GUARD_CTOR, SCOPE_GUARD_DTOR]);
        assert_eq!(guard[0].1[1..], [6, 1, 0x0106_649c, 0x95]);
        assert_eq!(guard.len(), 2);
        // A failed allocation gives 0 and builds nothing.
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(
            lock_free_queue_rigid_body_allocate_interface(&mut e, queue, 3),
            0
        );
        assert_eq!(built.borrow().len(), 1);
    }

    /// A queue with two nodes and a manager whose first word is 7.
    fn queue_with_nodes(e: &mut Engine) -> (Ptr<LockFreeQueue>, u32, [u32; 2]) {
        let queue = e.new_object::<LockFreeQueue>();
        let nodes = [e.mem.alloc(8), e.mem.alloc(8)];
        e.mem.set_u32(nodes[0], nodes[1]);
        e.mem.set_u32(nodes[0] + 4, 0x11);
        let manager = e.mem.alloc(0x10);
        e.mem.set_u32(manager, 7);
        let base = queue.addr();
        e.mem.set_u32(base + 4, nodes[0]);
        e.mem.set_u32(base + 8, nodes[1]);
        e.mem.set_u32(base + 0x14, manager);
        e.mem.set_u32(base + 0x18, 2);
        (queue, manager, nodes)
    }

    #[test]
    fn clearing_a_queue_frees_the_nodes_and_builds_an_empty_one() {
        let (mut e, allocations) = queue_engine();
        let (queue, manager, nodes) = queue_with_nodes(&mut e);
        let freed = record(&mut e, OPERATOR_DELETE, 0);
        let manager_freed = record(&mut e, QUEUE_INTERFACE_MANAGER_DELETE, 0);
        start_log(&mut e);
        fn_006c6700(&mut e, queue, 0);
        assert_eq!(*freed.borrow(), vec![vec![nodes[0]], vec![nodes[1]]]);
        assert_eq!(e.mem.u32(nodes[0] + 4), 0);
        assert_eq!(*manager_freed.borrow(), vec![vec![manager, 1]]);
        let allocations = allocations.borrow();
        assert_eq!(
            allocations.iter().map(|a| a[0]).collect::<Vec<_>>(),
            [8, 0x10]
        );
        let base = queue.addr();
        assert_eq!(e.mem.u32(base + 4), allocations[0][1]);
        assert_eq!(e.mem.u32(base + 8), allocations[0][1]);
        assert_eq!(e.mem.u32(base + 0x14), allocations[1][1]);
        assert_eq!(e.mem.u32(allocations[1][1]), 7);
        assert_eq!(e.mem.u32(base + 0x18), 0);
        let order = calls(
            &e,
            &[
                SPIN_LOCK_LOCK,
                SCOPE_GUARD_CTOR,
                SPIN_LOCK_UNLOCK,
                SCOPE_GUARD_DTOR,
            ],
        );
        let addresses: Vec<u32> = order.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            addresses,
            [
                SPIN_LOCK_LOCK,
                SCOPE_GUARD_CTOR,
                SPIN_LOCK_UNLOCK,
                SCOPE_GUARD_DTOR
            ]
        );
        assert_eq!(order[0].1, vec![base + 0x20, 0]);
        assert_eq!(order[1].1[1..], [6, 1, 0x0101_73f8, 0x115]);
    }

    #[test]
    fn clearing_a_queue_for_release_leaves_it_without_nodes() {
        let (mut e, allocations) = queue_engine();
        let (queue, _, _) = queue_with_nodes(&mut e);
        stub(&mut e, &[OPERATOR_DELETE, QUEUE_INTERFACE_MANAGER_DELETE]);
        fn_006c6700(&mut e, queue, 1);
        assert!(allocations.borrow().is_empty());
        let base = queue.addr();
        assert_eq!(e.mem.u32(base + 4), 0);
        assert_eq!(e.mem.u32(base + 0x14), 0);
    }

    #[test]
    fn the_queue_destructor_clears_and_frees_the_buffer() {
        let (mut e, allocations) = queue_engine();
        let (queue, _, nodes) = queue_with_nodes(&mut e);
        e.mem.set_u32(queue.addr() + 0x10, 0x5000);
        let freed = record(&mut e, OPERATOR_DELETE, 0);
        stub(&mut e, &[QUEUE_INTERFACE_MANAGER_DELETE]);
        let base_destructor = record(&mut e, QUEUE_BASE_DTOR, 0);
        fn_006c6680(&mut e, queue);
        assert_eq!(e.mem.u32(queue.addr()), LOCK_FREE_QUEUE_VTABLE);
        assert_eq!(
            *freed.borrow(),
            vec![vec![nodes[0]], vec![nodes[1]], vec![0x5000]]
        );
        assert!(allocations.borrow().is_empty());
        assert_eq!(*base_destructor.borrow(), vec![vec![queue.addr()]]);
    }

    /// A queue object (virtual slot `0x08` is `IncrementCount`) with a dummy
    /// node as tail, and an interface for it whose protected slot is a
    /// word of memory. Returns (queue, interface, dummy node, protected slot,
    /// the log of `IncrementCount`).
    fn push_fixture(e: &mut Engine) -> (u32, Ptr<LockFreeQueueInterface>, u32, u32, Log) {
        let queue = object_with_slots(e, &[(8, 0)]);
        let count = record(e, queue + 8, 0);
        let dummy = e.mem.alloc(8);
        e.mem.set_u32(queue + 8, dummy);
        let slot = e.mem.alloc(8);
        let interface = e.new_object::<LockFreeQueueInterface>();
        e.mem.set_u32(interface.addr(), queue);
        e.mem.set_u32(interface.addr() + 4, slot);
        // A compare-and-swap on game memory: true when the word was `expected`.
        e.register(COMPARE_AND_SWAP, |e, a| {
            if e.mem.u32(a[0]) == a[2] {
                e.mem.set_u32(a[0], a[1]);
                ret(1)
            } else {
                ret(0)
            }
        });
        (queue, interface, dummy, slot, count)
    }

    #[test]
    fn a_push_links_a_node_after_the_tail_and_moves_the_tail() {
        let (mut e, allocations) = queue_engine();
        let (queue, interface, dummy, slot, count) = push_fixture(&mut e);
        let value = slot_with(&mut e, 0x1234);
        start_log(&mut e);
        fn_006c6e60(&mut e, interface, value);
        let node = allocations.borrow()[0][1];
        assert_eq!(e.mem.u32(node + 4), 0x1234);
        assert_eq!(e.mem.u32(dummy), node);
        assert_eq!(e.mem.u32(queue + 8), node);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(count.borrow().len(), 1);
        let swaps: Vec<_> = calls(&e, &[COMPARE_AND_SWAP])
            .into_iter()
            .map(|c| c.1)
            .collect();
        assert_eq!(
            swaps,
            vec![vec![dummy, node, 0], vec![queue + 8, node, dummy]]
        );
        let guard = calls(&e, &[SCOPE_GUARD_CTOR]);
        assert_eq!(guard[0].1[1..], [6, 1, 0x0101_73f8, 0x76]);
    }

    #[test]
    fn a_push_first_helps_a_tail_that_already_has_a_successor() {
        let (mut e, allocations) = queue_engine();
        let (queue, interface, dummy, _, count) = push_fixture(&mut e);
        // Another thread linked a node but did not move the tail yet.
        let linked = e.mem.alloc(8);
        e.mem.set_u32(dummy, linked);
        let value = slot_with(&mut e, 0x99);
        start_log(&mut e);
        fn_006c6e60(&mut e, interface, value);
        let node = allocations.borrow()[0][1];
        let swaps: Vec<_> = calls(&e, &[COMPARE_AND_SWAP])
            .into_iter()
            .map(|c| c.1)
            .collect();
        assert_eq!(
            swaps,
            vec![
                vec![queue + 8, linked, dummy],
                vec![linked, node, 0],
                vec![queue + 8, node, linked]
            ]
        );
        assert_eq!(e.mem.u32(linked), node);
        assert_eq!(e.mem.u32(queue + 8), node);
        assert_eq!(count.borrow().len(), 1);
    }

    #[test]
    fn a_push_runs_through_the_threads_interface_under_the_lock() {
        let (mut e, _) = queue_engine();
        let (queue, interface, _, _, count) = push_fixture(&mut e);
        // The queue object of the fixture stands in for the `LockFreeQueue`.
        stub_ret(&mut e, QUEUE_THREAD_INTERFACE, interface.addr());
        let value = slot_with(&mut e, 0x55);
        start_log(&mut e);
        fn_006c5ff0(&mut e, Ptr::new(queue), value);
        let addresses: Vec<u32> = calls(
            &e,
            &[SPIN_LOCK_LOCK, QUEUE_THREAD_INTERFACE, SPIN_LOCK_UNLOCK],
        )
        .iter()
        .map(|(a, _)| *a)
        .collect();
        assert_eq!(
            addresses,
            [SPIN_LOCK_LOCK, QUEUE_THREAD_INTERFACE, SPIN_LOCK_UNLOCK]
        );
        assert_eq!(count.borrow().len(), 1);
        assert_eq!(calls(&e, &[SPIN_LOCK_LOCK])[0].1, vec![queue + 0x20, 0]);
    }

    #[test]
    fn a_pop_reports_what_the_interface_answers_under_the_lock() {
        let mut e = engine();
        stub(&mut e, &[SPIN_LOCK_LOCK, SPIN_LOCK_UNLOCK]);
        stub_ret(&mut e, QUEUE_THREAD_INTERFACE, 0x9990);
        let popped = record(&mut e, QUEUE_INTERFACE_POP, 1);
        start_log(&mut e);
        assert!(fn_006c6030(&mut e, Ptr::new(0x4000), 0x7770));
        assert_eq!(*popped.borrow(), vec![vec![0x9990, 0x7770]]);
        let addresses: Vec<u32> =
            calls(&e, &[SPIN_LOCK_LOCK, QUEUE_INTERFACE_POP, SPIN_LOCK_UNLOCK])
                .iter()
                .map(|(a, _)| *a)
                .collect();
        assert_eq!(
            addresses,
            [SPIN_LOCK_LOCK, QUEUE_INTERFACE_POP, SPIN_LOCK_UNLOCK]
        );
        assert_eq!(calls(&e, &[SPIN_LOCK_LOCK])[0].1, vec![0x4020, 0]);
        stub_ret(&mut e, QUEUE_INTERFACE_POP, 0);
        assert!(!fn_006c6030(&mut e, Ptr::new(0x4000), 0x7770));
    }

    // ---- Session 4 (`006c6fe0`..`006c77a0`) ----

    /// A queue object (virtual slot `0x0c` is `DecrementCount`) holding the
    /// dummy head `nodes[0]` and one node per value after it (each node: next,
    /// value), the tail at the last node, and an interface for it whose two
    /// protected slots are words of memory. Returns (queue, interface, nodes,
    /// first slot, second slot, the log of `DecrementCount`).
    #[allow(clippy::type_complexity)]
    fn pop_fixture(
        e: &mut Engine,
        values: &[u32],
        batch: u32,
    ) -> (u32, Ptr<LockFreeQueueInterface>, Vec<u32>, u32, u32, Log) {
        let queue = object_with_slots(e, &[(0x0c, 0)]);
        let decrements = record(e, queue + 0x0c, 0);
        let nodes: Vec<u32> = (0..=values.len()).map(|_| e.mem.alloc(8)).collect();
        for (i, value) in values.iter().enumerate() {
            e.mem.set_u32(nodes[i], nodes[i + 1]);
            e.mem.set_u32(nodes[i + 1] + 4, *value);
        }
        e.mem.set_u32(queue + 4, nodes[0]);
        e.mem.set_u32(queue + 8, *nodes.last().unwrap());
        e.mem.set_u32(queue + 0x0c, batch);
        let slots = e.mem.alloc(8);
        let interface = e.new_object::<LockFreeQueueInterface>();
        e.mem.set_u32(interface.addr(), queue);
        e.mem.set_u32(interface.addr() + 4, slots);
        e.mem.set_u32(interface.addr() + 8, slots + 4);
        e.register(COMPARE_AND_SWAP, |e, a| {
            if e.mem.u32(a[0]) == a[2] {
                e.mem.set_u32(a[0], a[1]);
                ret(1)
            } else {
                ret(0)
            }
        });
        e.register(TASKLET_SET_DATA, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            ret(a[0])
        });
        stub(e, &[QUEUE_INTERFACE_FREE_BATCH]);
        (queue, interface, nodes, slots, slots + 4, decrements)
    }

    #[test]
    fn a_pop_takes_the_first_value_and_retires_the_old_head() {
        let mut e = engine();
        let (queue, interface, nodes, first, second, decrements) =
            pop_fixture(&mut e, &[0x77, 0x88], 8);
        let out = slot_with(&mut e, 0xdead);
        start_log(&mut e);
        assert!(fn_006c6fe0(&mut e, interface, out));
        assert_eq!(e.mem.u32(out), 0x77);
        assert_eq!(e.mem.u32(queue + 4), nodes[1]);
        // The new head's value is cleared; the old head is chained into the
        // interface's list of retired nodes.
        assert_eq!(e.mem.u32(nodes[1] + 4), 0);
        assert_eq!(e.mem.u32(interface.addr() + 0x10), nodes[0]);
        assert_eq!(e.mem.u32(interface.addr() + 0x0c), 1);
        assert_eq!(e.mem.u32(nodes[0] + 4), 0);
        assert_eq!((e.mem.u32(first), e.mem.u32(second)), (0, 0));
        assert_eq!(decrements.borrow().len(), 1);
        assert!(calls(&e, &[QUEUE_INTERFACE_FREE_BATCH]).is_empty());
        let swaps: Vec<_> = calls(&e, &[COMPARE_AND_SWAP])
            .into_iter()
            .map(|c| c.1)
            .collect();
        assert_eq!(swaps, vec![vec![queue + 4, nodes[1], nodes[0]]]);
    }

    #[test]
    fn a_pop_from_an_empty_queue_answers_false() {
        let mut e = engine();
        let (queue, interface, nodes, first, second, decrements) = pop_fixture(&mut e, &[], 8);
        let out = slot_with(&mut e, 0xdead);
        assert!(!fn_006c6fe0(&mut e, interface, out));
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(e.mem.u32(queue + 4), nodes[0]);
        assert_eq!(e.mem.u32(first), 0);
        // The second slot keeps the (null) successor it was given.
        assert_eq!(e.mem.u32(second), 0);
        assert!(decrements.borrow().is_empty());
        assert_eq!(e.mem.u32(interface.addr() + 0x0c), 0);
    }

    #[test]
    fn a_pop_first_moves_a_lagging_tail_on() {
        let mut e = engine();
        let (queue, interface, nodes, _, _, _) = pop_fixture(&mut e, &[0x55], 8);
        // The tail still points at the head although a node follows it.
        e.mem.set_u32(queue + 8, nodes[0]);
        let out = slot_with(&mut e, 0);
        start_log(&mut e);
        assert!(fn_006c6fe0(&mut e, interface, out));
        assert_eq!(e.mem.u32(out), 0x55);
        assert_eq!(e.mem.u32(queue + 8), nodes[1]);
        let swaps: Vec<_> = calls(&e, &[COMPARE_AND_SWAP])
            .into_iter()
            .map(|c| c.1)
            .collect();
        assert_eq!(
            swaps,
            vec![
                vec![queue + 8, nodes[1], nodes[0]],
                vec![queue + 4, nodes[1], nodes[0]]
            ]
        );
    }

    #[test]
    fn a_pop_retries_when_another_thread_took_the_head() {
        let mut e = engine();
        let (queue, interface, nodes, _, _, decrements) = pop_fixture(&mut e, &[0x11, 0x22], 8);
        // The first compare-and-swap fails because another thread popped.
        let attempts = Rc::new(RefCell::new(0));
        let shared = attempts.clone();
        e.register_double(COMPARE_AND_SWAP, move |e, a| {
            *shared.borrow_mut() += 1;
            if *shared.borrow() == 1 {
                e.mem.set_u32(a[0], a[1]);
                ret(0)
            } else if e.mem.u32(a[0]) == a[2] {
                e.mem.set_u32(a[0], a[1]);
                ret(1)
            } else {
                ret(0)
            }
        });
        let out = slot_with(&mut e, 0);
        assert!(fn_006c6fe0(&mut e, interface, out));
        assert_eq!(*attempts.borrow(), 2);
        assert_eq!(e.mem.u32(out), 0x22);
        assert_eq!(e.mem.u32(queue + 4), nodes[2]);
        assert_eq!(decrements.borrow().len(), 1);
    }

    #[test]
    fn retiring_a_node_frees_the_batch_when_it_is_full() {
        let mut e = engine();
        let (_, interface, _, _, _, _) = pop_fixture(&mut e, &[], 2);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first + 4, 0x99);
        start_log(&mut e);
        fn_006c7560(&mut e, interface, first);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.mem.u32(interface.addr() + 0x10), first);
        assert_eq!(e.mem.u32(interface.addr() + 0x0c), 1);
        assert!(calls(&e, &[QUEUE_INTERFACE_FREE_BATCH]).is_empty());
        fn_006c7560(&mut e, interface, second);
        // The second node points at the first through its `+4` word.
        assert_eq!(e.mem.u32(second + 4), first);
        assert_eq!(e.mem.u32(interface.addr() + 0x10), second);
        assert_eq!(e.mem.u32(interface.addr() + 0x0c), 2);
        assert_eq!(
            calls(&e, &[QUEUE_INTERFACE_FREE_BATCH]),
            vec![(QUEUE_INTERFACE_FREE_BATCH, vec![interface.addr()])]
        );
    }

    #[test]
    fn the_interface_manager_destructor_destroys_each_existing_interface() {
        let mut e = engine();
        let manager = e.mem.alloc(0x10);
        let entries = e.mem.alloc(24);
        e.mem.set_u32(manager, 3);
        e.mem.set_u32(manager + 4, 0x7001);
        e.mem.set_u32(manager + 8, entries);
        e.mem.set_u32(entries + 4, 0xa000);
        e.mem.set_u32(entries + 12, 0);
        e.mem.set_u32(entries + 20, 0xc000);
        let destroyed = record(&mut e, QUEUE_INTERFACE_DESTROY, 0);
        let freed = record(&mut e, OPERATOR_DELETE, 0);
        let released = record(&mut e, QUEUE_MANAGER_RELEASE, 0);
        fn_006c74a0(&mut e, Ptr::new(manager));
        assert_eq!(*destroyed.borrow(), vec![vec![0xa000, 1], vec![0xc000, 1]]);
        assert_eq!(*freed.borrow(), vec![vec![entries]]);
        assert_eq!(*released.borrow(), vec![vec![0x7001]]);
    }

    #[test]
    fn the_interface_manager_deleting_destructor_deletes_only_when_asked() {
        let mut e = engine();
        let manager = e.mem.alloc(0x10);
        let entries = e.mem.alloc(8);
        e.mem.set_u32(manager + 8, entries);
        stub(&mut e, &[QUEUE_INTERFACE_DESTROY, QUEUE_MANAGER_RELEASE]);
        let freed = record(&mut e, OPERATOR_DELETE, 0);
        assert_eq!(fn_006c71a0(&mut e, Ptr::new(manager), 0).addr(), manager);
        assert_eq!(*freed.borrow(), vec![vec![entries]]);
        assert_eq!(fn_006c71a0(&mut e, Ptr::new(manager), 1).addr(), manager);
        assert_eq!(
            *freed.borrow(),
            vec![vec![entries], vec![entries], vec![manager]]
        );
    }

    #[test]
    fn the_container_deleting_destructors_delete_only_when_asked() {
        type Array = fn(&mut Engine, Ptr<BSSimpleArray>, u32) -> Ptr<BSSimpleArray>;
        let arrays: [Array; 5] = [
            bs_simple_array_obstacle_data_scalar_deleting_destructor,
            bs_simple_array_obstacle_task_data_scalar_deleting_destructor,
            bs_scrap_array_nav_mesh_ptr_scalar_deleting_destructor,
            bs_simple_array_tes_object_cell_scalar_deleting_destructor,
            bs_scrap_array_tes_object_cell_scalar_deleting_destructor,
        ];
        for destructor in arrays {
            let mut e = engine();
            let array = e.new_object::<BSSimpleArray>();
            stub(
                &mut e,
                &[
                    ARRAY_CLEAR,
                    OBSTACLE_ARRAY_EMPTY,
                    HOLDER_ARRAY_CLEAR,
                    HOLDER_ARRAY_BASE_DTOR,
                ],
            );
            let freed = record(&mut e, OPERATOR_DELETE, 0);
            assert_eq!(destructor(&mut e, array, 0), array);
            assert!(freed.borrow().is_empty());
            assert_eq!(destructor(&mut e, array, 1), array);
            assert_eq!(*freed.borrow(), vec![vec![array.addr()]]);
        }
        type Map = fn(&mut Engine, Ptr<NiTMap>, u32) -> Ptr<NiTMap>;
        let maps: [Map; 3] = [
            ni_tmap_base_reference_obstacle_array_scalar_deleting_destructor,
            ni_tmap_base_rigid_body_obstacle_data_scalar_deleting_destructor,
            ni_tmap_base_obstacle_task_data_scalar_deleting_destructor,
        ];
        for destructor in maps {
            let (mut e, map, _) = map_destruction_engine();
            let freed = record(&mut e, OPERATOR_DELETE, 0);
            assert_eq!(destructor(&mut e, map, 0), map);
            assert!(freed.borrow().iter().all(|call| call[0] != map.addr()));
            assert_eq!(destructor(&mut e, map, 1), map);
            assert_eq!(freed.borrow().last(), Some(&vec![map.addr()]));
        }
    }

    #[test]
    fn the_queue_deleting_destructor_deletes_only_when_asked() {
        for flags in [0, 1] {
            let (mut e, _) = queue_engine();
            let (queue, _, _) = queue_with_nodes(&mut e);
            e.mem.set_u32(queue.addr() + 0x10, 0x5000);
            let freed = record(&mut e, OPERATOR_DELETE, 0);
            stub(&mut e, &[QUEUE_INTERFACE_MANAGER_DELETE, QUEUE_BASE_DTOR]);
            assert_eq!(
                lock_free_queue_rigid_body_scalar_deleting_destructor(&mut e, queue, flags),
                queue
            );
            let deleted = freed.borrow().iter().any(|call| call[0] == queue.addr());
            assert_eq!(deleted, flags == 1);
        }
    }

    #[test]
    fn the_task_constructor_builds_the_members_and_records_the_info() {
        let mut e = engine();
        let task = e.new_object::<ObstacleTaskData>();
        e.mem.set_u8(task.addr() + 0x54, 1);
        let base = record(&mut e, TASK_BASE_CTOR, 0);
        let holders = record(&mut e, NAVMESH_HOLDER_CTOR, 0);
        let operations = record(&mut e, TASK_OPERATIONS_CTOR, 0);
        let swaps = record(&mut e, TASK_PORTAL_SWAPS_CTOR, 0);
        assert_eq!(fn_006c77a0(&mut e, task, 0x4321), task);
        let at = task.addr();
        assert_eq!(e.mem.u32(at), TASK_DATA_VTABLE);
        assert_eq!(*base.borrow(), vec![vec![at]]);
        assert_eq!(*holders.borrow(), vec![vec![at + 0x1c], vec![at + 0x20]]);
        assert_eq!(*operations.borrow(), vec![vec![at + 0x24]]);
        assert_eq!(*swaps.borrow(), vec![vec![at + 0x34]]);
        assert_eq!(e.mem.u32(at + 0x18), 0x4321);
        assert_eq!(e.mem.u8(at + 0x54), 0);
    }
}

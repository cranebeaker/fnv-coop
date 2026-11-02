//! `fallout shared/pathfinding/navmeshobstacletask.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds `ObstacleTaskData` (the work a tasklet does for one
//! navmesh: a list of obstacle operations applied to a copy of the navmesh),
//! the operation record `ObstacleTaskNavMeshOperation` (16 bytes) and the
//! `BSSimpleArray<ObstacleTaskNavMeshOperation, 1024>` instance that holds
//! them.
//!
//! Conventions (as in `navmeshobstaclemanager.rs`): the compiler built the
//! unit without optimisation, so every accessor is a real call and is called
//! here by address; `NiPointer<T>` is one word (the pointer); a local the
//! game keeps on its stack and passes by address is a slot from
//! [`Engine::with_stack`]. The compiler's exception-unwinding frames are not
//! translated: every function that has one carries a
//! `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_pathfinding::navmeshobstaclemanager::ObstacleTaskData;

layout! {
    /// `ObstacleTaskNavMeshOperation` (Xbox PDB), `0x10` bytes.
    pub struct ObstacleTaskNavMeshOperation: 0x10 {
        /// `spObstacle` (Xbox PDB): `NiPointer<ObstacleData>`.
        0x00 spObstacle: u32,
        /// `eType` (Xbox PDB): `OBSTACLE_TASK_TYPE`. The code tests the
        /// values `0x10`, `0x11` (add), `0x20`, `0x21` (remove), `0x30` and
        /// `0x40`; their names are not confirmed.
        0x04 eType: u32,
        /// `eResult` (Xbox PDB): `OBSTACLE_TASK_RESULT`. The task writes 0
        /// when the operation was applied and 1 when it was not.
        0x08 eResult: u32,
        /// `spNavMesh` (Xbox PDB): `NavMeshPtr`, a navmesh holder.
        0x0c spNavMesh: u32,
    }
}

/// `operator delete` (`cdecl`, one argument).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The vtable `006c78f0` installs (its class is not confirmed: the map names
/// the deleting destructor `006c7910` `BSTaskletData`'s).
const TASKLET_VTABLE: u32 = 0x0106_c814;
/// The `ObstacleTaskData` vtable (also installed by `006c77a0`).
const TASK_VTABLE: u32 = 0x0106_c7ec;
/// The vtable of the operations array; slot `+4` allocates `n` elements.
const OPERATIONS_VTABLE: u32 = 0x0106_c8d4;
/// The destructor body of the task's base class (`this`).
const TASK_BASE_DESTRUCTOR: u32 = 0x00b0_0ec0;
/// The destructor of the `PortalSwaps` member (`+0x34`).
const PORTAL_SWAPS_DESTRUCTOR: u32 = 0x0069_94b0;
/// Navmesh holder (`NavMeshPtr`, one word): constructor (null), destructor,
/// assignment from another holder and copy constructor (both take the
/// address of the other holder).
const HOLDER_CONSTRUCTOR: u32 = 0x0042_fb00;
const HOLDER_DESTRUCTOR: u32 = 0x0042_fa40;
const HOLDER_ASSIGN: u32 = 0x0042_f4c0;
const HOLDER_COPY: u32 = 0x0042_fa20;
/// `NiPointer::operator->` on the slot in `ECX`: the word it holds. The
/// engine map names `00458b50` the same way; its body calls `00559450`.
const POINTER_GET: u32 = 0x0055_9450;
const HOLDER_GET: u32 = 0x0045_8b50;
/// The pointer member of an operation: constructor from a pointer word
/// (`this`, word), assignment from the address of another `NiPointer`
/// (`this`, address) and release (`this`).
const POINTER_INIT: u32 = 0x0063_3c90;
const POINTER_ASSIGN: u32 = 0x006e_5cc0;
const POINTER_RELEASE: u32 = 0x0045_cec0;
/// `BSSimpleArray::GetSize` (the word at `+8`), `operator[]` on the 16-byte
/// operations (`this`, index; element address) and `operator[]` on the
/// 12-byte portal swap array at `+0x44`.
const ARRAY_SIZE: u32 = 0x0044_ddc0;
const OPERATION_AT: u32 = 0x0055_8dd0;
const PORTAL_SWAP_AT: u32 = 0x006a_1440;
/// Whether the `NiPointer` slot `this` holds the same pointer as the slot at
/// the argument (`AL`).
const SAME_OBSTACLE: u32 = 0x004b_0460;
/// `Error` (`cdecl`, format string and arguments; a five-byte stub in the PC
/// build) and the message of the removed add: `"Removing previous operation
/// ADD for Task on Navmesh 0x%08x (srcptr = 0x%08x)"`.
const ERROR: u32 = 0x0040_fbe0;
const REMOVING_ADD_MESSAGE: u32 = 0x0106_c828;
/// `BSSimpleArray` helpers shared with the other element types: whether the
/// array is small enough to shrink (`AL`), the capacity to shrink to, the
/// growth of one element (returns the index of the new slot), the
/// constructor of `n` elements at an address (`this`, address, `n`), the
/// destructor of `n` elements (`this`, address, `n`), the element move
/// (`this`, destination, source, `n`) and the buffer release (`this`).
const ARRAY_SHOULD_SHRINK: u32 = 0x006f_3170;
const ARRAY_SHRUNK_CAPACITY: u32 = 0x0086_9600;
const ARRAY_GROW_ONE: u32 = 0x006f_31f0;
const ARRAY_CONSTRUCT_RANGE: u32 = 0x006c_8a90;
const ARRAY_DESTROY_RANGE: u32 = 0x006c_8b90;
const ARRAY_MOVE_RANGE: u32 = 0x006c_8bd0;
const ARRAY_FREE_BUFFER: u32 = 0x006a_8500;
/// `qsort` (`cdecl`: base, count, element size, comparison function).
const QSORT: u32 = 0x00ec_6f20;
/// Debug scope guard (a 4-byte local): constructor (`thiscall`: category
/// byte, flag, file name, line) and destructor.
const SCOPE_GUARD_CTOR: u32 = 0x0040_4eb0;
const SCOPE_GUARD_DTOR: u32 = 0x0040_4ee0;
/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\Pathfinding\NavMeshObstacleTask.cpp"`.
const SOURCE_PATH: u32 = 0x0106_c878;
/// The manager getter (`006c0720`, no parameters; its function-local static)
/// and the performance timer step (`00483710`, called on a pointer) with
/// the pointer `006c85c0` derives from the manager (`+0x1a1`).
const MANAGER_INSTANCE: u32 = 0x006c_0720;
const PERFORMANCE_TIMER_STEP: u32 = 0x0048_3710;
/// `NavMeshObstacleManager::AddProcessedTask` (`this`, task).
const ADD_PROCESSED_TASK: u32 = 0x006c_4f70;
/// `NavMeshInfo` method filling the holder at the argument (`AL`: true when
/// it did).
const INFO_LOAD_MESH: u32 = 0x0069_ad00;
/// The allocator the code uses for a new navmesh (`cdecl`, size) and the
/// `NavMesh` constructor (`this`; the engine map names it
/// `NavMesh::NavMesh`), then `00464fc0(&holder, mesh)` (`cdecl`) builds a
/// holder for it and returns its address.
const ALLOCATE: u32 = 0x00aa_13e0;
const NAV_MESH_SIZE: u32 = 0x108;
const NAV_MESH_CONSTRUCTOR: u32 = 0x0068_eb80;
const HOLDER_FROM_MESH: u32 = 0x0046_4fc0;
/// A function on the source navmesh with the new navmesh's holder address
/// as the argument (`006942e0`, called on the source mesh in `Process`).
const MESH_PREPARE_COPY: u32 = 0x0069_42e0;
/// Navmesh methods `Process` applies, called on the new navmesh:
/// `NavMesh::AddObstacle` (`698f40`, element address and the `+0x34` portal
/// swaps), `698fc0` (same arguments), `6997e0` (two words: the word at
/// `obstacle + 8 + 8` and the obstacle pointer) and `699a30` (the word at
/// `obstacle + 8 + 8`); all return `AL`, true when the operation applied.
const NAV_MESH_ADD_OBSTACLE: u32 = 0x0069_8f40;
const NAV_MESH_REMOVE_OBSTACLE: u32 = 0x0069_8fc0;
const NAV_MESH_OPERATION_30: u32 = 0x0069_97e0;
const NAV_MESH_OPERATION_40: u32 = 0x0069_9a30;
/// `006995c0(mesh, element)`: the sort key of an element in the navmesh.
const NAV_MESH_SORT_KEY: u32 = 0x0069_95c0;
/// The finishing step's functions (see `fn_006c85e0`).
const SOURCE_MESH_INFO: u32 = 0x0084_e3a0;
const SOURCE_MESH_OWNER: u32 = 0x0059_bb30;
const OWNER_STATE: u32 = 0x0070_ec90;
const OWNER_SET_NEW: u32 = 0x0046_9500;
const OWNER_SET_OLD: u32 = 0x0069_bb00;
const MESH_SET_A: u32 = 0x0069_42c0;
const MESH_SET_B: u32 = 0x006d_3100;
const INFO_SET_MESH: u32 = 0x005a_8040;

/// The operation type's group: 1 for `0x10`/`0x11`, 2 for `0x20`/`0x21`, 3
/// for `0x30`, 4 for `0x40`, 0 for every other value (the exe's jump table).
fn operation_group(kind: u32) -> u32 {
    match kind {
        0x10 | 0x11 => 1,
        0x20 | 0x21 => 2,
        0x30 => 3,
        0x40 => 4,
        _ => 0,
    }
}

fn array_size(e: &mut Engine, array: u32) -> u32 {
    e.call(ARRAY_SIZE, &args![array]).u32()
}

fn operation_at(e: &mut Engine, array: u32, index: u32) -> u32 {
    e.call(OPERATION_AT, &args![array, index]).u32()
}

fn pointer_get(e: &mut Engine, slot: u32) -> u32 {
    e.call(POINTER_GET, &args![slot]).u32()
}

fn set_result(e: &mut Engine, array: u32, index: u32, applied: bool) {
    let element = operation_at(e, array, index);
    e.mem.set_u32(
        element + ObstacleTaskNavMeshOperation::eResult.off,
        u32::from(!applied),
    );
}

// Translated from 006c78f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (engine map): installs the vtable `0106c814` (the base class of
/// `ObstacleTaskData`'s tasklet data; the class name is not confirmed).
pub fn fn_006c78f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TASKLET_VTABLE);
}

// Translated from 006c7910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTaskletData::_scalar_deleting_destructor_` (Xbox PDB): runs
/// `006c78f0`, then frees `this` when bit 0 of `flags` is set. Returns
/// `this`.
pub fn bs_tasklet_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006c78f0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006c7940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (engine map): the task base class's deleting destructor: runs the
/// base destructor body `00b00ec0`, then frees `this` when bit 0 of `flags`
/// is set. Returns `this`.
pub fn fn_006c7940(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(TASK_BASE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006c7970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ObstacleTaskData::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body `006c79a0`, then frees `this` when bit 0 of `flags` is
/// set. Returns `this`.
pub fn obstacle_task_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<ObstacleTaskData>,
    flags: u32,
) -> Ptr<ObstacleTaskData> {
    fn_006c79a0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006c79a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c79a0 - the exception-unwinding frame (FS:[0], states 3..0) is not translated
/// Unnamed (engine map): the destructor body of `ObstacleTaskData` (it
/// installs the vtable `0106c7ec` that `006c77a0` installs): destroys
/// `PortalSwaps` (`+0x34`), `Operations` (`+0x24`, `006c87f0`),
/// `spNewNavMesh` (`+0x20`) and `spSrcMesh` (`+0x1c`), then the base class
/// body `00b00ec0`.
pub fn fn_006c79a0(e: &mut Engine, this: Ptr<ObstacleTaskData>) {
    let task = this.addr();
    e.mem.set_u32(task, TASK_VTABLE);
    e.call(
        PORTAL_SWAPS_DESTRUCTOR,
        &args![task + ObstacleTaskData::PortalSwaps.off],
    );
    fn_006c87f0(e, Ptr::new(task + ObstacleTaskData::Operations.off));
    e.call(
        HOLDER_DESTRUCTOR,
        &args![task + ObstacleTaskData::spNewNavMesh.off],
    );
    e.call(
        HOLDER_DESTRUCTOR,
        &args![task + ObstacleTaskData::spSrcMesh.off],
    );
    e.call(TASK_BASE_DESTRUCTOR, &args![task]);
}

// Translated from 006c7a30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c7a30 - the exception-unwinding frame (FS:[0], states 1..0) is not translated
/// `ObstacleTaskData::AddOperation` (Xbox PDB): queues operation `operation`
/// for the obstacle whose `NiPointer` is at `obstacle`. Returns false when
/// the operation is refused (or the task has no navmesh to apply it to):
///
/// - `0x10`/`0x11`: refused when an operation of the same obstacle with type
///   bits `0x30` equal to `0x10` is queued;
/// - `0x20`/`0x21`: queued adds of the same obstacle (type bits `0x30` equal
///   to `0x10`) are removed from the array (`Error` is called for each);
///   refused when another operation of the same obstacle is queued;
/// - `0x30` and `0x40`: refused when the same type is already queued for the
///   obstacle;
/// - any other value: refused.
///
/// Otherwise it fills a holder from `pInfo` (`0069ad00`); with a navmesh the
/// operation (obstacle, type, holder) is appended and the result is true.
pub fn obstacle_task_data_add_operation(
    e: &mut Engine,
    this: Ptr<ObstacleTaskData>,
    obstacle: u32,
    operation: u32,
) -> bool {
    let task = this.addr();
    let operations = task + ObstacleTaskData::Operations.off;
    match operation {
        0x10 | 0x11 => {
            let mut index = 0u32;
            while index < array_size(e, operations) {
                let element = operation_at(e, operations, index);
                if e.call(SAME_OBSTACLE, &args![element, obstacle]).bool() {
                    let element = operation_at(e, operations, index);
                    if e.mem.u32(element + 4) & 0x30 == 0x10 {
                        return false;
                    }
                }
                index = index.wrapping_add(1);
            }
        }
        0x20 | 0x21 => {
            let mut none_else = true;
            let mut index = 0u32;
            while index < array_size(e, operations) {
                let element = operation_at(e, operations, index);
                if e.call(SAME_OBSTACLE, &args![element, obstacle]).bool() {
                    let element = operation_at(e, operations, index);
                    if e.mem.u32(element + 4) & 0x30 == 0x10 {
                        let info = e.mem.u32(task + ObstacleTaskData::pInfo.off);
                        let info_id = e.mem.u32(info);
                        let source = e.mem.u32(task + ObstacleTaskData::spSrcMesh.off);
                        e.call(ERROR, &args![REMOVING_ADD_MESSAGE, info_id, source]);
                        fn_006c8860(e, Ptr::new(operations), index, true);
                        index = index.wrapping_sub(1);
                    } else {
                        none_else = false;
                    }
                }
                index = index.wrapping_add(1);
            }
            if !none_else {
                return false;
            }
        }
        0x30 | 0x40 => {
            let mut index = 0u32;
            while index < array_size(e, operations) {
                let element = operation_at(e, operations, index);
                if e.call(SAME_OBSTACLE, &args![element, obstacle]).bool() {
                    let element = operation_at(e, operations, index);
                    if e.mem.u32(element + 4) == operation {
                        return false;
                    }
                }
                index = index.wrapping_add(1);
            }
        }
        _ => return false,
    }
    e.with_stack(4, |e, holder| {
        e.call(HOLDER_CONSTRUCTOR, &args![holder]);
        let info = e.mem.u32(task + ObstacleTaskData::pInfo.off);
        if e.call(INFO_LOAD_MESH, &args![info, holder]).bool() {
            e.with_stack(0x10, |e, record| {
                fn_006c7d70(e, record, obstacle, operation, holder.addr());
                fn_006c8810(e, Ptr::new(operations), record);
                fn_006c7e00(e, record);
            });
            e.call(HOLDER_DESTRUCTOR, &args![holder]);
            true
        } else {
            e.call(HOLDER_DESTRUCTOR, &args![holder]);
            false
        }
    })
}

// Translated from 006c7d70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c7d70 - the exception-unwinding frame (FS:[0], states 1..0) is not translated
/// Unnamed (engine map): the constructor of an `ObstacleTaskNavMeshOperation`:
/// a null `spObstacle`, an empty `spNavMesh`, then `spObstacle` assigned from
/// the `NiPointer` at `obstacle`, `eType` = `operation` and `spNavMesh`
/// assigned from the holder at `holder`. `eResult` is left as it was.
/// Returns `this`.
pub fn fn_006c7d70(e: &mut Engine, this: Ptr, obstacle: u32, operation: u32, holder: u32) -> Ptr {
    e.call(POINTER_INIT, &args![this, 0u32]);
    e.call(
        HOLDER_CONSTRUCTOR,
        &args![this.addr() + ObstacleTaskNavMeshOperation::spNavMesh.off],
    );
    e.call(POINTER_ASSIGN, &args![this, obstacle]);
    e.mem.set_u32(
        this.addr() + ObstacleTaskNavMeshOperation::eType.off,
        operation,
    );
    e.call(
        HOLDER_ASSIGN,
        &args![
            this.addr() + ObstacleTaskNavMeshOperation::spNavMesh.off,
            holder
        ],
    );
    this
}

// Translated from 006c7e00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c7e00 - the exception-unwinding frame (FS:[0], state 0) is not translated
/// Unnamed (engine map): the destructor of an
/// `ObstacleTaskNavMeshOperation`: destroys `spNavMesh` (`+0xc`), then
/// releases `spObstacle`.
pub fn fn_006c7e00(e: &mut Engine, this: Ptr) {
    e.call(
        HOLDER_DESTRUCTOR,
        &args![this.addr() + ObstacleTaskNavMeshOperation::spNavMesh.off],
    );
    e.call(POINTER_RELEASE, &args![this]);
}

// Translated from 006c7e60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c7e60 - the exception-unwinding frame (FS:[0], state 0) is not translated
/// Unnamed (engine map): the comparison function `006c8150` gives `qsort`
/// (`cdecl`, two element addresses). Elements order by type group (the exe's
/// jump table): `0x10`/`0x11` before `0x20`/`0x21`, which come before `0x30`,
/// with `0x40` last; within the first group by the word at
/// `obstacle + 8 + 8` (`first - second`); within the second group by the
/// navmesh sort key `006995c0` of each element (second minus first), zero
/// when the `spNavMesh` holder is empty. Equal groups `0x30` and `0x40`
/// compare equal; a type outside the groups gives 0.
pub fn fn_006c7e60(e: &mut Engine, first: u32, second: u32) -> i32 {
    let first_kind = e.mem.u32(first + 4);
    let second_kind = e.mem.u32(second + 4);
    match (operation_group(first_kind), operation_group(second_kind)) {
        (1, 1) => {
            let first_obstacle = pointer_get(e, first);
            let first_word = e.mem.u32(first_obstacle + 8);
            let second_obstacle = pointer_get(e, second);
            let second_word = e.mem.u32(second_obstacle + 8);
            e.mem
                .u32(first_word + 8)
                .wrapping_sub(e.mem.u32(second_word + 8)) as i32
        }
        (1, 2) | (1, 4) | (2, 4) => 1,
        (1, 3) | (2, 1) | (2, 3) => -1,
        (2, 2) => e.with_stack(4, |e, holder| {
            e.call(
                HOLDER_COPY,
                &args![holder, first + ObstacleTaskNavMeshOperation::spNavMesh.off],
            );
            let result = if e.call(HOLDER_GET, &args![holder]).u32() != 0 {
                let mesh = pointer_get(e, holder.addr());
                let first_key = e.call(NAV_MESH_SORT_KEY, &args![mesh, first]).u32();
                let mesh = pointer_get(e, holder.addr());
                let second_key = e.call(NAV_MESH_SORT_KEY, &args![mesh, second]).u32();
                second_key.wrapping_sub(first_key) as i32
            } else {
                0
            };
            e.call(HOLDER_DESTRUCTOR, &args![holder]);
            result
        }),
        (3, 3) | (4, 4) => 0,
        (3, _) => 1,
        (4, _) => -1,
        _ => 0,
    }
}

// Translated from 006c8150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (engine map): sorts the task's `Operations` (`+0x24`) with
/// `006c8980` and the comparison function `006c7e60`.
pub fn fn_006c8150(e: &mut Engine, this: Ptr<ObstacleTaskData>) {
    fn_006c8980(
        e,
        Ptr::new(this.addr() + ObstacleTaskData::Operations.off),
        0x006c_7e60,
    );
}

// Translated from 006c8170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c8170 - the exception-unwinding frame (FS:[0], states 2..0) is not translated
/// `ObstacleTaskData::Process` (Xbox PDB): runs the task's operations.
/// Inside a scope guard and the manager's timer step, it loads the source
/// navmesh (`0069ad00` on `pInfo`). Without one every operation gets result
/// 1. With one it builds a new navmesh (`0x108` bytes, `NavMesh::NavMesh`)
/// into `spNewNavMesh`, prepares it from the source (`006942e0`), sorts the
/// operations and applies each by type on the new navmesh (add `0x10/0x11`:
/// `698f40`, remove `0x20/0x21`: `698fc0`, `0x30`: `6997e0`, `0x40`:
/// `699a30`); the result of an operation is 0 when the call returned true,
/// otherwise 1. Both portal swap arrays (`+0x44` and `+0x34`) then get the
/// task's `pInfo` in the first word of each entry. At the end the manager
/// gets the task (`AddProcessedTask`).
pub fn obstacle_task_data_process(e: &mut Engine, this: Ptr<ObstacleTaskData>) {
    let task = this.addr();
    let operations = task + ObstacleTaskData::Operations.off;
    let swaps = task + ObstacleTaskData::PortalSwaps.off;
    let swap_second = swaps + 0x10;
    let new_mesh = task + ObstacleTaskData::spNewNavMesh.off;
    let source_mesh = task + ObstacleTaskData::spSrcMesh.off;
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_CTOR,
            &args![guard, 0x2eu32, 1u32, SOURCE_PATH, 0x165u32],
        );
        timer_step(e);
        let info = e.mem.u32(task + ObstacleTaskData::pInfo.off);
        if e.call(INFO_LOAD_MESH, &args![info, source_mesh]).bool() {
            let allocation = e.call(ALLOCATE, &args![NAV_MESH_SIZE]).u32();
            let mesh = if allocation != 0 {
                e.call(NAV_MESH_CONSTRUCTOR, &args![allocation]).u32()
            } else {
                0
            };
            e.with_stack(4, |e, holder| {
                let built = e.call(HOLDER_FROM_MESH, &args![holder, mesh]).u32();
                e.call(HOLDER_ASSIGN, &args![new_mesh, built]);
                e.call(HOLDER_DESTRUCTOR, &args![holder]);
            });
            let source = pointer_get(e, source_mesh);
            e.call(MESH_PREPARE_COPY, &args![source, new_mesh]);
            fn_006c8150(e, this);
            let mut index = 0u32;
            while index < array_size(e, operations) {
                let element = operation_at(e, operations, index);
                match operation_group(e.mem.u32(element + 4)) {
                    1 | 2 => {
                        let element = operation_at(e, operations, index);
                        let mesh = pointer_get(e, new_mesh);
                        let function = if operation_group(e.mem.u32(element + 4)) == 1 {
                            NAV_MESH_ADD_OBSTACLE
                        } else {
                            NAV_MESH_REMOVE_OBSTACLE
                        };
                        let applied = e.call(function, &args![mesh, element, swaps]).bool();
                        set_result(e, operations, index, applied);
                    }
                    3 => {
                        let first = operation_at(e, operations, index);
                        let obstacle = pointer_get(e, first);
                        let second = operation_at(e, operations, index);
                        let second_obstacle = pointer_get(e, second);
                        let word = e.mem.u32(second_obstacle + 8);
                        let word = e.mem.u32(word + 8);
                        let mesh = pointer_get(e, new_mesh);
                        let applied = e
                            .call(NAV_MESH_OPERATION_30, &args![mesh, word, obstacle])
                            .bool();
                        set_result(e, operations, index, applied);
                    }
                    4 => {
                        let first = operation_at(e, operations, index);
                        let obstacle = pointer_get(e, first);
                        let word = e.mem.u32(obstacle + 8);
                        let word = e.mem.u32(word + 8);
                        let mesh = pointer_get(e, new_mesh);
                        let applied = e.call(NAV_MESH_OPERATION_40, &args![mesh, word]).bool();
                        set_result(e, operations, index, applied);
                    }
                    _ => {}
                }
                index = index.wrapping_add(1);
            }
            let mut index = 0u32;
            while index < array_size(e, swap_second) {
                let entry = e.call(PORTAL_SWAP_AT, &args![swap_second, index]).u32();
                e.mem.set_u32(entry, info);
                index = index.wrapping_add(1);
            }
            let mut index = 0u32;
            while index < array_size(e, swaps) {
                let entry = e.call(PORTAL_SWAP_AT, &args![swaps, index]).u32();
                e.mem.set_u32(entry, info);
                index = index.wrapping_add(1);
            }
        } else {
            let mut index = 0u32;
            while index < array_size(e, operations) {
                let element = operation_at(e, operations, index);
                e.mem
                    .set_u32(element + ObstacleTaskNavMeshOperation::eResult.off, 1);
                index = index.wrapping_add(1);
            }
        }
        let manager = e.call(MANAGER_INSTANCE, &args![]).u32();
        e.call(ADD_PROCESSED_TASK, &args![manager, task]);
        timer_step(e);
        e.call(SCOPE_GUARD_DTOR, &args![guard]);
    });
}

/// `006c0720()`, `006c85c0` on its result and the timer step `00483710` on
/// that.
fn timer_step(e: &mut Engine) {
    let manager = e.call(MANAGER_INSTANCE, &args![]).u32();
    let pointer = fn_006c85c0(e, manager);
    e.call(PERFORMANCE_TIMER_STEP, &args![pointer]);
}

// Translated from 006c85c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (engine map): the address `this + 0x1a1` (a byte of the
/// `NavMeshObstacleManager`, its performance timer).
pub fn fn_006c85c0(_e: &mut Engine, this: u32) -> u32 {
    this.wrapping_add(0x1a1)
}

// Translated from 006c85e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006c85e0 - the exception-unwinding frame (FS:[0], states 1..0) is not translated
/// Unnamed (engine map): the finishing step of a processed task (the
/// manager's update calls it). Returns false when the source or the new
/// navmesh holder is empty, or when `bPortalModified` is clear and no
/// operation has result 0 (nothing was applied). Otherwise: when `0059bb30`
/// on the source navmesh gives a non-null object, that object's `0070ec90`
/// result gets a fresh copy of the new navmesh holder (`00469500`) and of
/// the source holder (`0069bb00`) as by-value arguments; then
/// `006942c0`/`006d3100` run on the source navmesh with 0, and on the new
/// navmesh with the word `0084e3a0` gave for the source and with `pInfo`,
/// and `005a8040` gives `pInfo` the new navmesh. Returns true. The callees'
/// identities are not confirmed (the engine map's `D3DTexture_LockRect` for
/// `0059bb30` and `DetailedActorPathHandler::GetCurrentNodeIndex` for
/// `0070ec90` are folded names).
pub fn fn_006c85e0(e: &mut Engine, this: Ptr<ObstacleTaskData>) -> bool {
    let task = this.addr();
    let source_mesh = task + ObstacleTaskData::spSrcMesh.off;
    let new_mesh = task + ObstacleTaskData::spNewNavMesh.off;
    let operations = task + ObstacleTaskData::Operations.off;
    if pointer_get(e, source_mesh) == 0 {
        return false;
    }
    if pointer_get(e, new_mesh) == 0 {
        return false;
    }
    let mut applied = false;
    if e.mem.u8(task + ObstacleTaskData::bPortalModified.off) == 0 {
        let mut index = 0u32;
        while index < array_size(e, operations) {
            let element = operation_at(e, operations, index);
            if e.mem
                .u32(element + ObstacleTaskNavMeshOperation::eResult.off)
                == 0
            {
                applied = true;
                break;
            }
            index = index.wrapping_add(1);
        }
    } else {
        applied = true;
    }
    if !applied {
        return false;
    }
    let source = pointer_get(e, source_mesh);
    let source_info = e.call(SOURCE_MESH_INFO, &args![source]).u32();
    let source = pointer_get(e, source_mesh);
    let owner = e.call(SOURCE_MESH_OWNER, &args![source]).u32();
    if owner != 0 {
        e.with_stack(4, |e, copy| {
            e.call(HOLDER_COPY, &args![copy, new_mesh]);
            let value = e.mem.u32(copy.addr());
            let state = e.call(OWNER_STATE, &args![owner]).u32();
            e.call(OWNER_SET_NEW, &args![state, value]);
        });
        e.with_stack(4, |e, copy| {
            e.call(HOLDER_COPY, &args![copy, source_mesh]);
            let value = e.mem.u32(copy.addr());
            let state = e.call(OWNER_STATE, &args![owner]).u32();
            e.call(OWNER_SET_OLD, &args![state, value]);
        });
    }
    let mesh = pointer_get(e, source_mesh);
    e.call(MESH_SET_A, &args![mesh, 0u32]);
    let mesh = pointer_get(e, source_mesh);
    e.call(MESH_SET_B, &args![mesh, 0u32]);
    let mesh = pointer_get(e, new_mesh);
    e.call(MESH_SET_A, &args![mesh, source_info]);
    let info = e.mem.u32(task + ObstacleTaskData::pInfo.off);
    let mesh = pointer_get(e, new_mesh);
    e.call(MESH_SET_B, &args![mesh, info]);
    let new_mesh_pointer = e.call(HOLDER_GET, &args![new_mesh]).u32();
    let info = e.mem.u32(task + ObstacleTaskData::pInfo.off);
    e.call(INFO_SET_MESH, &args![info, new_mesh_pointer]);
    true
}

// Translated from 006c87c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (engine map): the constructor of the `Operations` array:
/// installs the vtable `0106c8d4` and runs `006c8c70(0, 0)` (empty, nothing
/// reserved). Returns `this`.
pub fn fn_006c87c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), OPERATIONS_VTABLE);
    fn_006c8c70(e, this, 0, 0);
    this
}

// Translated from 006c87f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (engine map): the destructor body of the `Operations` array:
/// installs the vtable `0106c8d4` and runs `006c8a30(true)` (destroy the
/// elements and free the buffer).
pub fn fn_006c87f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), OPERATIONS_VTABLE);
    fn_006c8a30(e, this, true);
}

// Translated from 006c8810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (engine map): appends a copy of the operation at `value` to the
/// array: `006f31f0` makes room and gives the new index, `006c8a90`
/// constructs one element there and `006c89e0` copies `value` into it.
/// Returns the new index.
pub fn fn_006c8810(e: &mut Engine, this: Ptr, value: Ptr) -> u32 {
    let index = e.call(ARRAY_GROW_ONE, &args![this]).u32();
    let buffer = e.mem.u32(this.addr() + 4);
    let slot = (index << 4).wrapping_add(buffer);
    e.call(ARRAY_CONSTRUCT_RANGE, &args![this, slot, 1u32]);
    let buffer = e.mem.u32(this.addr() + 4);
    fn_006c89e0(e, Ptr::new((index << 4).wrapping_add(buffer)), value);
    index
}

// Translated from 006c8860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (engine map): removes element `index` from the array. With
/// `allow_shrink` and when `006f3170` says the array is small for its
/// capacity, the buffer is reallocated (`vtable + 4` with the capacity
/// `00869600` gives): the elements before `index` move over, `index` is
/// destroyed, and the elements after it move over. The exe passes the count
/// `size - 1` (not `size - index - 1`) for that second move; kept. Otherwise
/// the tail moves down one slot in place. The size drops by one.
pub fn fn_006c8860(e: &mut Engine, this: Ptr, index: u32, allow_shrink: bool) {
    let array = this.addr();
    if allow_shrink && e.call(ARRAY_SHOULD_SHRINK, &args![array]).bool() {
        let capacity = e.call(ARRAY_SHRUNK_CAPACITY, &args![array]).u32();
        let buffer = e.vcall(array, 4, &args![capacity]).u32();
        let old = e.mem.u32(array + 4);
        e.call(ARRAY_MOVE_RANGE, &args![array, buffer, old, index]);
        let old = e.mem.u32(array + 4);
        e.call(
            ARRAY_DESTROY_RANGE,
            &args![array, (index << 4).wrapping_add(old), 1u32],
        );
        let old = e.mem.u32(array + 4);
        let size = e.mem.u32(array + 8);
        let after = (index << 4).wrapping_add(old).wrapping_add(0x10);
        e.call(
            ARRAY_MOVE_RANGE,
            &args![
                array,
                (index << 4).wrapping_add(buffer),
                after,
                size.wrapping_sub(1)
            ],
        );
        e.call(ARRAY_FREE_BUFFER, &args![array]);
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 0xc, capacity);
    } else {
        let old = e.mem.u32(array + 4);
        e.call(
            ARRAY_DESTROY_RANGE,
            &args![array, (index << 4).wrapping_add(old), 1u32],
        );
        let size = e.mem.u32(array + 8);
        let old = e.mem.u32(array + 4);
        let slot = (index << 4).wrapping_add(old);
        e.call(
            ARRAY_MOVE_RANGE,
            &args![
                array,
                slot,
                slot.wrapping_add(0x10),
                size.wrapping_sub(index).wrapping_sub(1)
            ],
        );
    }
    let size = e.mem.u32(array + 8);
    e.mem.set_u32(array + 8, size.wrapping_sub(1));
}

// Translated from 006c8980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (engine map): `qsort` of the array's elements (`+4`, count at
/// `+8`, 16 bytes each) with the comparison function `compare`.
pub fn fn_006c8980(e: &mut Engine, this: Ptr, compare: u32) {
    let buffer = e.mem.u32(this.addr() + 4);
    let count = e.mem.u32(this.addr() + 8);
    e.call(QSORT, &args![buffer, count, 0x10u32, compare]);
}

// Translated from 006c89b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<ObstacleTaskNavMeshOperation_1024>::_scalar_deleting_destructor_`
/// (Xbox PDB): runs `006c87f0`, then frees `this` when bit 0 of `flags` is
/// set. Returns `this`.
pub fn bs_simple_array_obstacle_task_nav_mesh_operation_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_006c87f0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006c89e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (engine map): the copy assignment of an
/// `ObstacleTaskNavMeshOperation`: `spObstacle` is assigned from the other
/// (`006e5cc0`), `eType` and `eResult` are copied, and `spNavMesh` is assigned
/// from the other's. Returns `this`.
pub fn fn_006c89e0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(POINTER_ASSIGN, &args![this, other]);
    let kind = e
        .mem
        .u32(other.addr() + ObstacleTaskNavMeshOperation::eType.off);
    e.mem
        .set_u32(this.addr() + ObstacleTaskNavMeshOperation::eType.off, kind);
    let result = e
        .mem
        .u32(other.addr() + ObstacleTaskNavMeshOperation::eResult.off);
    e.mem.set_u32(
        this.addr() + ObstacleTaskNavMeshOperation::eResult.off,
        result,
    );
    e.call(
        HOLDER_ASSIGN,
        &args![
            this.addr() + ObstacleTaskNavMeshOperation::spNavMesh.off,
            other.addr() + ObstacleTaskNavMeshOperation::spNavMesh.off
        ],
    );
    this
}

// Translated from 006c8a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (engine map): empties the array. When it has a buffer, its
/// elements are destroyed (`006c8b90`); with `free_buffer` the buffer is
/// released (`006a8500`) and the buffer and capacity words are cleared. The
/// size becomes 0.
pub fn fn_006c8a30(e: &mut Engine, this: Ptr, free_buffer: bool) {
    let array = this.addr();
    let buffer = e.mem.u32(array + 4);
    if buffer != 0 {
        let size = e.mem.u32(array + 8);
        e.call(ARRAY_DESTROY_RANGE, &args![array, buffer, size]);
        if free_buffer {
            e.call(ARRAY_FREE_BUFFER, &args![array]);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 0xc, 0);
        }
        e.mem.set_u32(array + 8, 0);
    }
}

// Translated from 006c8c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (engine map): the array constructor body: clears the buffer, size
/// and capacity words; the capacity is the larger of `reserved` and `size`;
/// with a capacity the buffer comes from the vtable's allocator (`+4`, the
/// capacity) and with a size that many elements are constructed (`006c8a90`)
/// and counted.
pub fn fn_006c8c70(e: &mut Engine, this: Ptr, reserved: u32, size: u32) {
    let array = this.addr();
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0);
    e.mem.set_u32(array + 0xc, 0);
    let capacity = reserved.max(size);
    if capacity != 0 {
        let buffer = e.vcall(array, 4, &args![capacity]).u32();
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 0xc, capacity);
    }
    if size != 0 {
        let buffer = e.mem.u32(array + 4);
        e.call(ARRAY_CONSTRUCT_RANGE, &args![array, buffer, size]);
        e.mem.set_u32(array + 8, size);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006c78f0, fn_006c78f0(Ptr)),
        entry!(
            0x006c7910,
            bs_tasklet_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x006c7940, fn_006c7940(Ptr, u32) -> Ptr),
        entry!(
            0x006c7970,
            obstacle_task_data_scalar_deleting_destructor(
                Ptr<ObstacleTaskData>,
                u32,
            ) -> Ptr<ObstacleTaskData>
        ),
        entry!(0x006c79a0, fn_006c79a0(Ptr<ObstacleTaskData>)),
        entry!(
            0x006c7a30,
            obstacle_task_data_add_operation(Ptr<ObstacleTaskData>, u32, u32) -> bool
        ),
        entry!(0x006c7d70, fn_006c7d70(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006c7e00, fn_006c7e00(Ptr)),
        entry!(0x006c7e60, fn_006c7e60(u32, u32) -> i32),
        entry!(0x006c8150, fn_006c8150(Ptr<ObstacleTaskData>)),
        entry!(
            0x006c8170,
            obstacle_task_data_process(Ptr<ObstacleTaskData>)
        ),
        entry!(0x006c85c0, fn_006c85c0(u32) -> u32),
        entry!(0x006c85e0, fn_006c85e0(Ptr<ObstacleTaskData>) -> bool),
        entry!(0x006c87c0, fn_006c87c0(Ptr) -> Ptr),
        entry!(0x006c87f0, fn_006c87f0(Ptr)),
        entry!(0x006c8810, fn_006c8810(Ptr, Ptr) -> u32),
        entry!(0x006c8860, fn_006c8860(Ptr, u32, bool)),
        entry!(0x006c8980, fn_006c8980(Ptr, u32)),
        entry!(
            0x006c89b0,
            bs_simple_array_obstacle_task_nav_mesh_operation_scalar_deleting_destructor(Ptr, u32)
                -> Ptr
        ),
        entry!(0x006c89e0, fn_006c89e0(Ptr, Ptr) -> Ptr),
        entry!(0x006c8a30, fn_006c8a30(Ptr, bool)),
        entry!(0x006c8c70, fn_006c8c70(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// One recorded call: address and argument words.
    type Call = (u32, Vec<u32>);

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// The addresses of every callee outside this file.
    const OUTSIDE: &[u32] = &[
        OPERATOR_DELETE,
        TASK_BASE_DESTRUCTOR,
        PORTAL_SWAPS_DESTRUCTOR,
        HOLDER_CONSTRUCTOR,
        HOLDER_DESTRUCTOR,
        HOLDER_ASSIGN,
        HOLDER_COPY,
        POINTER_GET,
        HOLDER_GET,
        POINTER_INIT,
        POINTER_ASSIGN,
        POINTER_RELEASE,
        ARRAY_SIZE,
        OPERATION_AT,
        PORTAL_SWAP_AT,
        SAME_OBSTACLE,
        ERROR,
        ARRAY_SHOULD_SHRINK,
        ARRAY_SHRUNK_CAPACITY,
        ARRAY_GROW_ONE,
        ARRAY_CONSTRUCT_RANGE,
        ARRAY_DESTROY_RANGE,
        ARRAY_MOVE_RANGE,
        ARRAY_FREE_BUFFER,
        QSORT,
        SCOPE_GUARD_CTOR,
        SCOPE_GUARD_DTOR,
        MANAGER_INSTANCE,
        PERFORMANCE_TIMER_STEP,
        ADD_PROCESSED_TASK,
        INFO_LOAD_MESH,
        ALLOCATE,
        NAV_MESH_CONSTRUCTOR,
        HOLDER_FROM_MESH,
        MESH_PREPARE_COPY,
        NAV_MESH_SORT_KEY,
        SOURCE_MESH_INFO,
        SOURCE_MESH_OWNER,
        OWNER_STATE,
        OWNER_SET_NEW,
        OWNER_SET_OLD,
        MESH_SET_A,
        MESH_SET_B,
        INFO_SET_MESH,
        NAV_MESH_ADD_OBSTACLE,
        NAV_MESH_REMOVE_OBSTACLE,
        NAV_MESH_OPERATION_30,
        NAV_MESH_OPERATION_40,
    ];

    /// An engine where the accessors work on game memory (`NiPointer` get,
    /// holder copy and assign, array size and element address; `qsort` does
    /// nothing) and every other callee returns 0. Calls are logged.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in OUTSIDE.iter().copied() {
            e.register(address, |_, _| Ret::default());
        }
        for address in [POINTER_GET, HOLDER_GET] {
            e.register(address, |e, a| ret(e.mem.u32(a[0])));
        }
        e.register(ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(OPERATION_AT, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 16));
        e.register(PORTAL_SWAP_AT, |e, a| ret(e.mem.u32(a[0] + 4) + a[1] * 12));
        e.register(SAME_OBSTACLE, |e, a| {
            ret(u32::from(e.mem.u32(a[0]) == e.mem.u32(a[1])))
        });
        for address in [HOLDER_COPY, HOLDER_ASSIGN, POINTER_ASSIGN] {
            e.register(address, |e, a| {
                let value = e.mem.u32(a[1]);
                e.mem.set_u32(a[0], value);
                ret(a[0])
            });
        }
        e.register(POINTER_INIT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(HOLDER_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0);
            ret(a[0])
        });
        e.register(HOLDER_FROM_MESH, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &mut Engine) -> Vec<Call> {
        e.call_log.replace(vec![]).unwrap_or_default()
    }

    fn calls_to(log: &[Call], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    fn task(e: &mut Engine) -> Ptr<ObstacleTaskData> {
        e.new_object::<ObstacleTaskData>()
    }

    /// Gives the array object at `array` a buffer of `capacity` elements of
    /// `element` bytes; returns the buffer.
    fn array_with(e: &mut Engine, array: u32, capacity: u32, element: u32) -> u32 {
        let buffer = e.mem.alloc(capacity * element);
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 0xc, capacity);
        buffer
    }

    /// Fills operation `index` of the array at `array` (result word 7).
    fn put_operation(e: &mut Engine, array: u32, index: u32, obstacle: u32, kind: u32) {
        let element = e.mem.u32(array + 4) + index * 16;
        e.mem.set_u32(element, obstacle);
        e.mem.set_u32(element + 4, kind);
        e.mem.set_u32(element + 8, 7);
        let size = e.mem.u32(array + 8);
        e.mem.set_u32(array + 8, size.max(index + 1));
    }

    /// An `ObstacleData`-like object: `[+8]` is a block whose `[+8]` is `key`.
    fn obstacle_with_key(e: &mut Engine, key: u32) -> u32 {
        let obstacle = e.mem.alloc(0x10);
        let inner = e.mem.alloc(0x10);
        e.mem.set_u32(obstacle + 8, inner);
        e.mem.set_u32(inner + 8, key);
        obstacle
    }

    /// A 16-byte record of type `kind` whose obstacle has `key`, with the
    /// navmesh holder word `mesh`.
    fn record_of(e: &mut Engine, kind: u32, key: u32, mesh: u32) -> u32 {
        let record = e.mem.alloc(0x10);
        let obstacle = obstacle_with_key(e, key);
        e.mem.set_u32(record, obstacle);
        e.mem.set_u32(record + 4, kind);
        e.mem.set_u32(record + 0xc, mesh);
        record
    }

    #[test]
    fn the_tasklet_base_installs_its_vtable() {
        let mut e = engine();
        let object = Ptr::new(e.mem.alloc(8));
        fn_006c78f0(&mut e, object);
        assert_eq!(e.mem.u32(object.addr()), TASKLET_VTABLE);
    }

    #[test]
    fn the_deleting_destructors_free_only_when_asked() {
        for flags in [0u32, 1, 2, 3] {
            let mut e = engine();
            let object = Ptr::new(e.mem.alloc(0x60));
            let frees = (flags & 1) as usize;
            assert_eq!(
                bs_tasklet_data_scalar_deleting_destructor(&mut e, object, flags),
                object
            );
            assert_eq!(e.mem.u32(object.addr()), TASKLET_VTABLE);
            assert_eq!(calls_to(&calls(&mut e), OPERATOR_DELETE).len(), frees);

            assert_eq!(fn_006c7940(&mut e, object, flags), object);
            let log = calls(&mut e);
            assert_eq!(
                calls_to(&log, TASK_BASE_DESTRUCTOR),
                vec![vec![object.addr()]]
            );
            assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), frees);

            let task = Ptr::<ObstacleTaskData>::new(object.addr());
            assert_eq!(
                obstacle_task_data_scalar_deleting_destructor(&mut e, task, flags),
                task
            );
            let log = calls(&mut e);
            assert_eq!(calls_to(&log, TASK_BASE_DESTRUCTOR).len(), 1);
            assert_eq!(calls_to(&log, OPERATOR_DELETE).len(), frees);

            assert_eq!(
                bs_simple_array_obstacle_task_nav_mesh_operation_scalar_deleting_destructor(
                    &mut e, object, flags
                ),
                object
            );
            assert_eq!(e.mem.u32(object.addr()), OPERATIONS_VTABLE);
            assert_eq!(calls_to(&calls(&mut e), OPERATOR_DELETE).len(), frees);
        }
    }

    #[test]
    fn the_task_destructor_destroys_the_members_in_reverse_order() {
        let mut e = engine();
        let task = task(&mut e);
        let at = task.addr();
        fn_006c79a0(&mut e, task);
        assert_eq!(e.mem.u32(at), TASK_VTABLE);
        let order: Vec<(u32, u32)> = calls(&mut e)
            .iter()
            .map(|call| (call.0, call.1[0]))
            .collect();
        // The empty operations array has no buffer, so it calls nothing.
        assert_eq!(
            order,
            vec![
                (PORTAL_SWAPS_DESTRUCTOR, at + 0x34),
                (HOLDER_DESTRUCTOR, at + 0x20),
                (HOLDER_DESTRUCTOR, at + 0x1c),
                (TASK_BASE_DESTRUCTOR, at),
            ]
        );
        assert_eq!(e.mem.u32(at + 0x24), OPERATIONS_VTABLE);
    }

    /// A task with an `Operations` array of capacity 8 holding `existing`
    /// (obstacle word, type), a loadable mesh, and the obstacle slot (word
    /// `0xaaaa`) to add.
    fn add_scene(e: &mut Engine, existing: &[(u32, u32)]) -> (Ptr<ObstacleTaskData>, u32, u32) {
        let task = task(e);
        let at = task.addr();
        let operations = at + 0x24;
        array_with(e, operations, 8, 16);
        for (index, (obstacle, kind)) in existing.iter().enumerate() {
            put_operation(e, operations, index as u32, *obstacle, *kind);
        }
        let info = e.mem.alloc(8);
        e.mem.set_u32(info, 0x1234);
        e.mem.set_u32(at + 0x18, info);
        e.mem.set_u32(at + 0x1c, 0x5151);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0xaaaa);
        e.register(INFO_LOAD_MESH, |e, a| {
            e.mem.set_u32(a[1], 0xbeef);
            ret(1)
        });
        e.register(ARRAY_GROW_ONE, |e, a| {
            let index = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, index + 1);
            ret(index)
        });
        // The element move is a real move, as `006c8860` relies on it.
        e.register(ARRAY_MOVE_RANGE, |e, a| {
            let words: Vec<u32> = (0..a[3] * 4).map(|i| e.mem.u32(a[2] + i * 4)).collect();
            for (i, word) in words.iter().enumerate() {
                e.mem.set_u32(a[1] + i as u32 * 4, *word);
            }
            Ret::default()
        });
        (task, slot, operations)
    }

    #[test]
    fn add_operation_refuses_unknown_types() {
        let mut e = engine();
        let (task, slot, operations) = add_scene(&mut e, &[]);
        for kind in [0u32, 0x0f, 0x12, 0x22, 0x31, 0x41, 0x50] {
            assert!(!obstacle_task_data_add_operation(&mut e, task, slot, kind));
        }
        assert_eq!(e.mem.u32(operations + 8), 0);
        assert!(calls_to(&calls(&mut e), INFO_LOAD_MESH).is_empty());
    }

    #[test]
    fn add_operation_appends_the_operation_with_the_loaded_mesh() {
        let mut e = engine();
        let (task, slot, operations) = add_scene(&mut e, &[(0xbbbb, 0x10)]);
        let at = task.addr();
        assert!(obstacle_task_data_add_operation(&mut e, task, slot, 0x11));
        let log = calls(&mut e);
        let info = e.mem.u32(at + 0x18);
        assert_eq!(calls_to(&log, INFO_LOAD_MESH).len(), 1);
        assert_eq!(calls_to(&log, INFO_LOAD_MESH)[0][0], info);
        assert_eq!(e.mem.u32(operations + 8), 2);
        let element = e.mem.u32(operations + 4) + 16;
        assert_eq!(e.mem.u32(element), 0xaaaa);
        assert_eq!(e.mem.u32(element + 4), 0x11);
        assert_eq!(e.mem.u32(element + 0xc), 0xbeef);
        // The holder local, the record's mesh and the record's copy.
        assert_eq!(calls_to(&log, HOLDER_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn add_operation_without_a_mesh_appends_nothing() {
        let mut e = engine();
        let (task, slot, operations) = add_scene(&mut e, &[]);
        e.register(INFO_LOAD_MESH, |_, _| ret(0));
        assert!(!obstacle_task_data_add_operation(&mut e, task, slot, 0x30));
        assert_eq!(e.mem.u32(operations + 8), 0);
        assert_eq!(calls_to(&calls(&mut e), HOLDER_DESTRUCTOR).len(), 1);
    }

    #[test]
    fn add_operation_0x10_is_refused_for_a_queued_add_of_the_same_obstacle() {
        let mut e = engine();
        let (task, slot, operations) = add_scene(&mut e, &[(0xaaaa, 0x11), (0xcccc, 0x30)]);
        assert!(!obstacle_task_data_add_operation(&mut e, task, slot, 0x10));
        assert_eq!(e.mem.u32(operations + 8), 2);
        // A different queued type for the same obstacle does not matter.
        let (task, slot, _) = add_scene(&mut e, &[(0xaaaa, 0x20)]);
        assert!(obstacle_task_data_add_operation(&mut e, task, slot, 0x10));
    }

    #[test]
    fn add_operation_0x20_removes_queued_adds() {
        let mut e = engine();
        let (task, slot, operations) =
            add_scene(&mut e, &[(0xaaaa, 0x10), (0xdddd, 0x10), (0xaaaa, 0x11)]);
        assert!(obstacle_task_data_add_operation(&mut e, task, slot, 0x21));
        let log = calls(&mut e);
        assert_eq!(calls_to(&log, ERROR).len(), 2);
        assert_eq!(
            calls_to(&log, ERROR)[0],
            vec![REMOVING_ADD_MESSAGE, 0x1234, 0x5151]
        );
        // Three queued, two removed (the survivor is the other obstacle's),
        // one appended.
        assert_eq!(e.mem.u32(operations + 8), 2);
        let buffer = e.mem.u32(operations + 4);
        assert_eq!(e.mem.u32(buffer), 0xdddd);
        assert_eq!(e.mem.u32(buffer + 16), 0xaaaa);
        assert_eq!(e.mem.u32(buffer + 20), 0x21);
    }

    #[test]
    fn add_operation_0x20_is_refused_when_another_operation_of_the_obstacle_is_queued() {
        let mut e = engine();
        let (task, slot, operations) = add_scene(&mut e, &[(0xaaaa, 0x30)]);
        assert!(!obstacle_task_data_add_operation(&mut e, task, slot, 0x20));
        assert_eq!(e.mem.u32(operations + 8), 1);
        assert!(calls_to(&calls(&mut e), ERROR).is_empty());
    }

    #[test]
    fn add_operation_0x30_and_0x40_refuse_only_their_own_type() {
        for kind in [0x30u32, 0x40] {
            let mut e = engine();
            let other = if kind == 0x30 { 0x40 } else { 0x30 };
            let (task, slot, _) = add_scene(&mut e, &[(0xaaaa, other), (0xbbbb, kind)]);
            assert!(obstacle_task_data_add_operation(&mut e, task, slot, kind));
            let (task, slot, _) = add_scene(&mut e, &[(0xbbbb, other), (0xaaaa, kind)]);
            assert!(!obstacle_task_data_add_operation(&mut e, task, slot, kind));
        }
    }

    #[test]
    fn the_operation_constructor_fills_the_record() {
        let mut e = engine();
        let record = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(record.addr() + 8, 0x99);
        let obstacle = e.mem.alloc(4);
        e.mem.set_u32(obstacle, 0x7001);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, 0x7002);
        assert_eq!(fn_006c7d70(&mut e, record, obstacle, 0x21, holder), record);
        let at = record.addr();
        assert_eq!(e.mem.u32(at), 0x7001);
        assert_eq!(e.mem.u32(at + 4), 0x21);
        assert_eq!(e.mem.u32(at + 8), 0x99);
        assert_eq!(e.mem.u32(at + 0xc), 0x7002);
        let order: Vec<u32> = calls(&mut e).iter().map(|call| call.0).collect();
        assert_eq!(
            order,
            vec![
                POINTER_INIT,
                HOLDER_CONSTRUCTOR,
                POINTER_ASSIGN,
                HOLDER_ASSIGN
            ]
        );
    }

    #[test]
    fn the_operation_destructor_destroys_the_mesh_then_the_pointer() {
        let mut e = engine();
        let record = Ptr::new(e.mem.alloc(0x10));
        fn_006c7e00(&mut e, record);
        let at = record.addr();
        assert_eq!(
            calls(&mut e),
            vec![
                (HOLDER_DESTRUCTOR, vec![at + 0xc]),
                (POINTER_RELEASE, vec![at])
            ]
        );
    }

    #[test]
    fn the_operation_copy_copies_pointer_type_result_and_mesh() {
        let mut e = engine();
        let source = Ptr::new(e.mem.alloc(0x10));
        let target = Ptr::new(e.mem.alloc(0x10));
        let s = source.addr();
        e.mem.set_u32(s, 0x1111);
        e.mem.set_u32(s + 4, 0x30);
        e.mem.set_u32(s + 8, 1);
        e.mem.set_u32(s + 0xc, 0x2222);
        assert_eq!(fn_006c89e0(&mut e, target, source), target);
        let t = target.addr();
        assert_eq!(e.mem.u32(t), 0x1111);
        assert_eq!(e.mem.u32(t + 4), 0x30);
        assert_eq!(e.mem.u32(t + 8), 1);
        assert_eq!(e.mem.u32(t + 0xc), 0x2222);
    }

    #[test]
    fn the_comparison_orders_the_groups() {
        let mut e = engine();
        // (first type, second type, result) with obstacle keys 10 and 3 for
        // the first and second record.
        let cases: [(u32, u32, i32); 22] = [
            (0x10, 0x11, 7),
            (0x11, 0x10, 7),
            (0x10, 0x20, 1),
            (0x11, 0x21, 1),
            (0x10, 0x30, -1),
            (0x10, 0x40, 1),
            (0x10, 0x50, 0),
            (0x20, 0x10, -1),
            (0x21, 0x11, -1),
            (0x20, 0x21, 0),
            (0x20, 0x30, -1),
            (0x21, 0x40, 1),
            (0x20, 0x50, 0),
            (0x30, 0x30, 0),
            (0x30, 0x10, 1),
            (0x30, 0x40, 1),
            (0x40, 0x40, 0),
            (0x40, 0x10, -1),
            (0x40, 0x30, -1),
            (0x50, 0x10, 0),
            (0x50, 0x50, 0),
            (0x0f, 0x30, 0),
        ];
        for (first, second, expected) in cases {
            let a = record_of(&mut e, first, 10, 0);
            let b = record_of(&mut e, second, 3, 0);
            assert_eq!(
                fn_006c7e60(&mut e, a, b),
                expected,
                "{first:#x} against {second:#x}"
            );
        }
    }

    #[test]
    fn the_comparison_of_removes_uses_the_sort_key_of_the_mesh() {
        let mut e = engine();
        e.register(NAV_MESH_SORT_KEY, |_, a| ret(a[1] & 0xff));
        let first = record_of(&mut e, 0x20, 0, 0x4000);
        let second = record_of(&mut e, 0x21, 0, 0x4000);
        // The key is the low byte of the element address; second minus first.
        let result = fn_006c7e60(&mut e, first, second);
        assert_eq!(result, (second & 0xff).wrapping_sub(first & 0xff) as i32);
        let log = calls(&mut e);
        assert_eq!(calls_to(&log, HOLDER_DESTRUCTOR).len(), 1);
        assert_eq!(calls_to(&log, NAV_MESH_SORT_KEY).len(), 2);
        assert_eq!(calls_to(&log, NAV_MESH_SORT_KEY)[0], vec![0x4000, first]);
    }

    #[test]
    fn the_sort_calls_qsort_with_the_comparison_function() {
        let mut e = engine();
        let task = task(&mut e);
        let at = task.addr();
        let buffer = array_with(&mut e, at + 0x24, 4, 16);
        e.mem.set_u32(at + 0x24 + 8, 3);
        fn_006c8150(&mut e, task);
        assert_eq!(
            calls(&mut e),
            vec![(QSORT, vec![buffer, 3, 0x10, 0x006c_7e60])]
        );
        fn_006c8980(&mut e, Ptr::new(at + 0x24), 0x1234);
        assert_eq!(calls(&mut e), vec![(QSORT, vec![buffer, 3, 0x10, 0x1234])]);
    }

    #[test]
    fn the_timer_pointer_is_the_manager_plus_0x1a1() {
        let mut e = engine();
        assert_eq!(fn_006c85c0(&mut e, 0x1000), 0x11a1);
    }

    /// A task ready for `Process` with five operations (types `0x10`, `0x21`,
    /// `0x30`, `0x40`, `0x50`) and two entries in each portal swap array.
    /// `loaded` is what the info load answers.
    fn process_scene(e: &mut Engine, loaded: bool) -> (Ptr<ObstacleTaskData>, u32) {
        let task = task(e);
        let at = task.addr();
        let operations = at + 0x24;
        array_with(e, operations, 8, 16);
        for (index, kind) in [0x10u32, 0x21, 0x30, 0x40, 0x50].into_iter().enumerate() {
            let record = record_of(e, kind, 30 + index as u32, 0);
            let obstacle = e.mem.u32(record);
            put_operation(e, operations, index as u32, obstacle, kind);
        }
        let info = e.mem.alloc(8);
        e.mem.set_u32(at + 0x18, info);
        for swaps in [at + 0x34, at + 0x44] {
            array_with(e, swaps, 2, 12);
            e.mem.set_u32(swaps + 8, 2);
        }
        e.register_double(INFO_LOAD_MESH, move |e, a| {
            e.mem.set_u32(a[1], 0xaaa0);
            ret(u32::from(loaded))
        });
        let mesh = e.mem.alloc(0x108);
        e.register_double(ALLOCATE, move |_, _| ret(mesh));
        e.register(NAV_MESH_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register(MANAGER_INSTANCE, |_, _| ret(0x5000));
        e.register(NAV_MESH_ADD_OBSTACLE, |_, _| ret(1));
        e.register(NAV_MESH_REMOVE_OBSTACLE, |_, _| ret(0));
        e.register(NAV_MESH_OPERATION_30, |_, _| ret(1));
        e.register(NAV_MESH_OPERATION_40, |_, _| ret(0));
        (task, info)
    }

    #[test]
    fn process_applies_each_operation_to_the_new_mesh() {
        let mut e = engine();
        let (task, info) = process_scene(&mut e, true);
        let at = task.addr();
        obstacle_task_data_process(&mut e, task);
        let buffer = e.mem.u32(at + 0x24 + 4);
        let results: Vec<u32> = (0..5).map(|i| e.mem.u32(buffer + i * 16 + 8)).collect();
        // Applied (true) gives 0, refused gives 1, other types stay.
        assert_eq!(results, vec![0, 1, 0, 1, 7]);
        let new_mesh = e.mem.u32(at + 0x20);
        assert_ne!(new_mesh, 0);
        let log = calls(&mut e);
        assert_eq!(calls_to(&log, ALLOCATE), vec![vec![0x108]]);
        assert_eq!(
            calls_to(&log, SCOPE_GUARD_CTOR)[0][1..],
            [0x2e, 1, SOURCE_PATH, 0x165]
        );
        assert_eq!(calls_to(&log, SCOPE_GUARD_DTOR).len(), 1);
        assert_eq!(calls_to(&log, MESH_PREPARE_COPY).len(), 1);
        assert_eq!(calls_to(&log, MESH_PREPARE_COPY)[0][1], at + 0x20);
        assert_eq!(
            calls_to(&log, QSORT),
            vec![vec![buffer, 5, 0x10, 0x006c_7e60]]
        );
        assert_eq!(
            calls_to(&log, NAV_MESH_ADD_OBSTACLE),
            vec![vec![new_mesh, buffer, at + 0x34]]
        );
        assert_eq!(
            calls_to(&log, NAV_MESH_REMOVE_OBSTACLE),
            vec![vec![new_mesh, buffer + 16, at + 0x34]]
        );
        let third = e.mem.u32(buffer + 2 * 16);
        let key = e.mem.u32(e.mem.u32(third + 8) + 8);
        assert_eq!(key, 32);
        assert_eq!(
            calls_to(&log, NAV_MESH_OPERATION_30),
            vec![vec![new_mesh, key, third]]
        );
        let fourth = e.mem.u32(buffer + 3 * 16);
        let key = e.mem.u32(e.mem.u32(fourth + 8) + 8);
        assert_eq!(key, 33);
        assert_eq!(
            calls_to(&log, NAV_MESH_OPERATION_40),
            vec![vec![new_mesh, key]]
        );
        // The portal swap entries get the info, then the task is handed over.
        for array in [at + 0x44, at + 0x34] {
            let entries = e.mem.u32(array + 4);
            for index in 0..2 {
                assert_eq!(e.mem.u32(entries + index * 12), info);
            }
        }
        assert_eq!(calls_to(&log, ADD_PROCESSED_TASK), vec![vec![0x5000, at]]);
        // The timer step before and after.
        let steps = calls_to(&log, PERFORMANCE_TIMER_STEP);
        assert_eq!(steps, vec![vec![0x5000 + 0x1a1]; 2]);
    }

    #[test]
    fn process_without_a_source_mesh_refuses_every_operation() {
        let mut e = engine();
        let (task, _) = process_scene(&mut e, false);
        let at = task.addr();
        obstacle_task_data_process(&mut e, task);
        let buffer = e.mem.u32(at + 0x24 + 4);
        let results: Vec<u32> = (0..5).map(|i| e.mem.u32(buffer + i * 16 + 8)).collect();
        assert_eq!(results, vec![1; 5]);
        let log = calls(&mut e);
        assert!(calls_to(&log, ALLOCATE).is_empty());
        assert!(calls_to(&log, QSORT).is_empty());
        assert_eq!(calls_to(&log, ADD_PROCESSED_TASK), vec![vec![0x5000, at]]);
        assert_eq!(calls_to(&log, SCOPE_GUARD_DTOR).len(), 1);
    }

    /// A task for the finishing step: both mesh holders filled and the
    /// operation results as given.
    fn finish_scene(e: &mut Engine, results: &[u32], modified: bool) -> Ptr<ObstacleTaskData> {
        let task = task(e);
        let at = task.addr();
        array_with(e, at + 0x24, 4, 16);
        for (index, result) in results.iter().enumerate() {
            put_operation(e, at + 0x24, index as u32, 0x10, 0x10);
            let element = e.mem.u32(at + 0x24 + 4) + index as u32 * 16;
            e.mem.set_u32(element + 8, *result);
        }
        e.mem.set_u32(at + 0x1c, 0x6100);
        e.mem.set_u32(at + 0x20, 0x6200);
        e.mem.set_u32(at + 0x18, 0x6300);
        e.mem.set_u8(at + 0x54, u8::from(modified));
        e.register(SOURCE_MESH_INFO, |_, _| ret(0x77));
        e.register(OWNER_STATE, |_, a| ret(a[0] + 1));
        task
    }

    #[test]
    fn finish_needs_both_meshes() {
        let mut e = engine();
        let task = finish_scene(&mut e, &[0], false);
        e.mem.set_u32(task.addr() + 0x1c, 0);
        assert!(!fn_006c85e0(&mut e, task));
        e.mem.set_u32(task.addr() + 0x1c, 0x6100);
        e.mem.set_u32(task.addr() + 0x20, 0);
        assert!(!fn_006c85e0(&mut e, task));
        assert!(calls_to(&calls(&mut e), INFO_SET_MESH).is_empty());
    }

    #[test]
    fn finish_needs_an_applied_operation_unless_the_portal_was_modified() {
        let mut e = engine();
        let task = finish_scene(&mut e, &[1, 1], false);
        assert!(!fn_006c85e0(&mut e, task));
        let task = finish_scene(&mut e, &[], false);
        assert!(!fn_006c85e0(&mut e, task));
        let task = finish_scene(&mut e, &[1, 1], true);
        assert!(fn_006c85e0(&mut e, task));
        let task = finish_scene(&mut e, &[1, 0], false);
        assert!(fn_006c85e0(&mut e, task));
    }

    #[test]
    fn finish_hands_the_new_mesh_over() {
        let mut e = engine();
        let task = finish_scene(&mut e, &[0], false);
        e.register(SOURCE_MESH_OWNER, |_, _| ret(0x9000));
        calls(&mut e);
        assert!(fn_006c85e0(&mut e, task));
        let log = calls(&mut e);
        assert_eq!(calls_to(&log, SOURCE_MESH_INFO), vec![vec![0x6100]]);
        assert_eq!(calls_to(&log, SOURCE_MESH_OWNER), vec![vec![0x6100]]);
        // The by-value copies of the new and source holders.
        assert_eq!(calls_to(&log, OWNER_SET_NEW), vec![vec![0x9001, 0x6200]]);
        assert_eq!(calls_to(&log, OWNER_SET_OLD), vec![vec![0x9001, 0x6100]]);
        assert_eq!(
            calls_to(&log, MESH_SET_A),
            vec![vec![0x6100, 0], vec![0x6200, 0x77]]
        );
        assert_eq!(
            calls_to(&log, MESH_SET_B),
            vec![vec![0x6100, 0], vec![0x6200, 0x6300]]
        );
        assert_eq!(calls_to(&log, INFO_SET_MESH), vec![vec![0x6300, 0x6200]]);
    }

    #[test]
    fn finish_skips_the_owner_when_there_is_none() {
        let mut e = engine();
        let task = finish_scene(&mut e, &[0], false);
        assert!(fn_006c85e0(&mut e, task));
        let log = calls(&mut e);
        assert!(calls_to(&log, OWNER_SET_NEW).is_empty());
        assert!(calls_to(&log, OWNER_SET_OLD).is_empty());
        assert_eq!(calls_to(&log, INFO_SET_MESH).len(), 1);
    }

    /// An array object with the operations vtable whose allocator (slot
    /// `+4`) hands out `buffer`.
    fn allocating_array(e: &mut Engine, buffer: u32) -> Ptr {
        let array = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(array.addr(), OPERATIONS_VTABLE);
        e.put_vtable(OPERATIONS_VTABLE, &[0, 0x0070_0100]);
        e.register_double(0x0070_0100, move |_, _| ret(buffer));
        array
    }

    fn words(e: &Engine, array: u32) -> (u32, u32, u32) {
        (
            e.mem.u32(array + 4),
            e.mem.u32(array + 8),
            e.mem.u32(array + 0xc),
        )
    }

    #[test]
    fn the_array_constructors_set_up_empty_arrays() {
        let mut e = engine();
        let buffer = e.mem.alloc(0x100);
        let array = allocating_array(&mut e, buffer);
        let a = array.addr();
        e.mem.set_u32(a + 4, 9);
        assert_eq!(fn_006c87c0(&mut e, array), array);
        assert_eq!(e.mem.u32(a), OPERATIONS_VTABLE);
        assert_eq!(words(&e, a), (0, 0, 0));
        assert!(calls(&mut e).is_empty());

        // Reserved only: allocates, constructs nothing.
        fn_006c8c70(&mut e, array, 5, 0);
        assert_eq!(words(&e, a), (buffer, 0, 5));
        assert!(calls_to(&calls(&mut e), ARRAY_CONSTRUCT_RANGE).is_empty());

        // The size wins when larger; elements are constructed and counted.
        fn_006c8c70(&mut e, array, 2, 3);
        assert_eq!(words(&e, a), (buffer, 3, 3));
        assert_eq!(
            calls_to(&calls(&mut e), ARRAY_CONSTRUCT_RANGE),
            vec![vec![a, buffer, 3]]
        );
    }

    #[test]
    fn the_array_destructor_body_frees_the_buffer() {
        let mut e = engine();
        let array = Ptr::new(e.mem.alloc(0x10));
        let a = array.addr();
        e.mem.set_u32(a + 4, 0x4000);
        e.mem.set_u32(a + 8, 3);
        e.mem.set_u32(a + 0xc, 8);
        fn_006c87f0(&mut e, array);
        assert_eq!(e.mem.u32(a), OPERATIONS_VTABLE);
        assert_eq!(words(&e, a), (0, 0, 0));
        assert_eq!(
            calls(&mut e),
            vec![
                (ARRAY_DESTROY_RANGE, vec![a, 0x4000, 3]),
                (ARRAY_FREE_BUFFER, vec![a])
            ]
        );
    }

    #[test]
    fn clearing_keeps_the_buffer_unless_asked_and_ignores_empty_arrays() {
        let mut e = engine();
        let array = Ptr::new(e.mem.alloc(0x10));
        let a = array.addr();
        fn_006c8a30(&mut e, array, true);
        assert!(calls(&mut e).is_empty());
        e.mem.set_u32(a + 4, 0x4000);
        e.mem.set_u32(a + 8, 3);
        e.mem.set_u32(a + 0xc, 8);
        fn_006c8a30(&mut e, array, false);
        assert_eq!(
            calls(&mut e),
            vec![(ARRAY_DESTROY_RANGE, vec![a, 0x4000, 3])]
        );
        assert_eq!(words(&e, a), (0x4000, 0, 8));
    }

    #[test]
    fn appending_constructs_and_copies_the_record() {
        let mut e = engine();
        let array = Ptr::new(e.mem.alloc(0x10));
        let a = array.addr();
        let buffer = array_with(&mut e, a, 4, 16);
        e.register(ARRAY_GROW_ONE, |_, _| ret(2));
        let value = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(value.addr(), 0x1357);
        e.mem.set_u32(value.addr() + 4, 0x40);
        assert_eq!(fn_006c8810(&mut e, array, value), 2);
        let slot = buffer + 32;
        let log = calls(&mut e);
        assert_eq!(
            calls_to(&log, ARRAY_CONSTRUCT_RANGE),
            vec![vec![a, slot, 1]]
        );
        assert_eq!(e.mem.u32(slot), 0x1357);
        assert_eq!(e.mem.u32(slot + 4), 0x40);
    }

    #[test]
    fn removing_in_place_moves_the_tail_down() {
        let mut e = engine();
        let array = Ptr::new(e.mem.alloc(0x10));
        let a = array.addr();
        let buffer = array_with(&mut e, a, 8, 16);
        e.mem.set_u32(a + 8, 5);
        fn_006c8860(&mut e, array, 1, false);
        assert_eq!(
            calls(&mut e),
            vec![
                (ARRAY_DESTROY_RANGE, vec![a, buffer + 16, 1]),
                (ARRAY_MOVE_RANGE, vec![a, buffer + 16, buffer + 32, 3]),
            ]
        );
        assert_eq!(e.mem.u32(a + 8), 4);
        // Not small enough to shrink: still in place with the flag set.
        fn_006c8860(&mut e, array, 0, true);
        let log = calls(&mut e);
        assert_eq!(calls_to(&log, ARRAY_SHOULD_SHRINK), vec![vec![a]]);
        assert_eq!(
            calls_to(&log, ARRAY_MOVE_RANGE),
            vec![vec![a, buffer, buffer + 16, 3]]
        );
        assert_eq!(e.mem.u32(a + 8), 3);
    }

    #[test]
    fn removing_with_shrink_reallocates_and_keeps_the_exe_move_count() {
        let mut e = engine();
        let new_buffer = e.mem.alloc(0x100);
        let array = allocating_array(&mut e, new_buffer);
        let a = array.addr();
        let buffer = array_with(&mut e, a, 32, 16);
        e.mem.set_u32(a + 8, 6);
        e.register(ARRAY_SHOULD_SHRINK, |_, _| ret(1));
        e.register(ARRAY_SHRUNK_CAPACITY, |_, _| ret(12));
        let log = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(0x0070_0100, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(new_buffer)
        });
        fn_006c8860(&mut e, array, 2, true);
        assert_eq!(*log.borrow(), vec![vec![a, 12]]);
        assert_eq!(
            calls(&mut e)
                .into_iter()
                .filter(|call| call.0 != 0x0070_0100)
                .collect::<Vec<_>>(),
            vec![
                (ARRAY_SHOULD_SHRINK, vec![a]),
                (ARRAY_SHRUNK_CAPACITY, vec![a]),
                (ARRAY_MOVE_RANGE, vec![a, new_buffer, buffer, 2]),
                (ARRAY_DESTROY_RANGE, vec![a, buffer + 32, 1]),
                // The count is size - 1, not size - index - 1.
                (ARRAY_MOVE_RANGE, vec![a, new_buffer + 32, buffer + 48, 5]),
                (ARRAY_FREE_BUFFER, vec![a]),
            ]
        );
        assert_eq!(words(&e, a), (new_buffer, 5, 12));
    }
}

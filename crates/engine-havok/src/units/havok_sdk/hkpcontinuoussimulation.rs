//! `Havok SDK/hkpcontinuoussimulation.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpContinuousSimulation` runs the Havok step with time-of-impact (TOI)
//! events: after the discrete collision phase, fast bodies whose swept
//! volumes meet get an event with the time of impact; the simulation handles
//! the events in time order, either with a simple response or with a
//! localized solve of the island part the event reaches.
//!
//! The class is not in the PDB, so the fields of [`HkpContinuousSimulation`]
//! are named by what the code does with them (unconfirmed). `hkpWorld`,
//! `hkpEntity` and `hkpSimulationIsland` offsets named below are the PDB's
//! (`m_simulation`, `m_activeSimulationIslands`, `m_broadPhase`,
//! `m_collisionInput`, `m_collisionFilter`, `m_collisionDispatcher`,
//! `m_criticalOperationsLockCount`, `m_toiCollisionResponseRotateNormal`,
//! `m_dynamicsStepInfo`, `m_sizeOfToiEventQueue`, `m_world`,
//! `m_collidable`, `m_solverData`, `m_storageIndex`, `m_constraintsMaster`,
//! `m_constraintsSlave`, `m_simulationIsland`, `m_uid`, `m_motion`).
//! Other offsets are used as plain numbers or `<ROLE>`-named constants whose
//! role is read from the code. The memory router `00c85750` hands out the
//! stack allocator (the word at `+0` is its top; a block of `n` bytes takes
//! `(n + 0x7f) & !0x7f` and is released by putting the old top back) and the
//! heap allocator (the object at `+0x10`: virtual slot `4` allocates, slot `8`
//! deletes an object, slot `0x10` frees `(pointer, bytes)`).
//!
//! The functions that use `00ec8660` at their start (`_chkstk`, the stack
//! probe for frames over 4 KB) are not modelled: the probe has no effect on
//! the translated behaviour. None of this unit's functions uses the `/GS`
//! cookie check (`00ec408c`) or an exception frame.
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` where the
//! code stores a `float`; values copied through the x87 stack come out as
//! quiet NaNs ([`x87_quiet`]).

#[allow(unused_imports)]
use crate::prelude::*;

// Profiling.
/// `TlsGetValue` (import slot).
const TLS_GET_VALUE: u32 = 0x00fd_f044;
/// The thread-local slot number of the profiling record stream.
const PROFILE_SLOT: u32 = 0x0126_835c;
const PROFILE_END: u32 = 0x010c_41f8;
const PROFILE_LT: u32 = 0x0107_6f38;
const PROFILE_LT_BROAD_PHASE: u32 = 0x010c_c624;
const PROFILE_ST_GATHER_AABBS: u32 = 0x010c_d3e8;
const PROFILE_ST_3AXIS_SWEEP: u32 = 0x010c_c5fc;
const PROFILE_ST_REMOVE_DUP: u32 = 0x010c_c5f0;
const PROFILE_TT_WAIT_FOR_EXPORT: u32 = 0x010c_d3d8;
const PROFILE_ST_REMOVE_AGENTS: u32 = 0x010c_c5e4;
const PROFILE_ST_ADD_AGENTS: u32 = 0x010c_c5d8;
const PROFILE_TT_NARROW_PHASE_TOI: u32 = 0x010c_d440;
const PROFILE_TT_NARROW_PHASE: u32 = 0x010c_c54c;
const PROFILE_TT_RECOLLIDE_PSI: u32 = 0x010c_cd50;
const PROFILE_TT_SIMPLE_TOI: u32 = 0x010c_d4e4;
const PROFILE_TT_PHYSICS: u32 = 0x010c_c568;
const PROFILE_TT_RECOLLIDE_AFTER_STEP_FAILURE: u32 = 0x010c_d4f8;
const PROFILE_TT_TOIS: u32 = 0x010c_d4f0;
const PROFILE_TT_COLLIDE: u32 = 0x010c_c64c;
const PROFILE_TT_ISLAND_POST_COLLIDE_CB: u32 = 0x010c_c634;
const PROFILE_TT_POST_SIM_CB: u32 = 0x010c_c540;
const PROFILE_LT_TOI: u32 = 0x010c_d540;
const PROFILE_ST_SOLVE_TOI: u32 = 0x010c_d534;
const PROFILE_ST_PENDING_OPERATIONS: u32 = 0x010c_d520;
const PROFILE_ST_COLLIDE: u32 = 0x010c_d514;

// Memory and containers.
/// Returns the memory router (stack allocator top at `+0`, heap allocator at
/// `+0x10`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArray` growth: (allocator, array, element size).
const ARRAY_GROW: u32 = 0x00c9_0940;
/// `hkArray` reserve: (allocator, array, count, element size).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkArray::insertAt`-like: `this` = the array, (allocator, index,
/// elements, count).
const ARRAY_INSERT_AT: u32 = 0x00cd_1e20;
/// `memset(pointer, value, bytes)`.
const MEM_SET: u32 = 0x00cb_1280;
/// Pointer map (`hkMap`): slots of 8 bytes (key, value), empty key
/// `0xffffffff`; `this` = the map. Reserve (allocator, count).
const MAP_RESERVE: u32 = 0x00cb_1fe0;
/// Insert (allocator, key, value).
const MAP_INSERT: u32 = 0x00cb_1870;
/// Find (key): the slot index, at most the hash mask when found.
const MAP_FIND_KEY: u32 = 0x00cb_18f0;
/// Get (key, default).
const MAP_GET_WITH_DEFAULT: u32 = 0x00cb_1930;
/// Clear and free the storage (allocator).
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
/// Clear.
const MAP_CLEAR: u32 = 0x00cb_1a60;
/// Memory size of the map.
const MAP_GET_MEM_SIZE: u32 = 0x00cb_17e0;
/// Map destructor.
const MAP_DESTRUCTOR: u32 = 0x00a2_9680;
/// `(pointer, bytes)`: the size the allocator reports for a block.
const ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// `hkReferencedObject::addReferences` (array, count, element size).
const ADD_REFERENCES: u32 = 0x00c9_0670;
/// The function after `addReferences` in `hkreferencedobject.cpp` (array,
/// count, element size); not named in the engine map.
const REFERENCE_ARRAY_FN_00C90740: u32 = 0x00c9_0740;
/// `hkReferencedObject::addReference`.
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// `hkReferencedObject::removeReference`.
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

// Havok callees.
/// `hkVector4::setRotatedDir` (Xbox PDB), called with `this` = the result
/// vector and the two source vectors.
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `hkpWorld` function that sets the rotation of a motion state (`this` =
/// the motion state, argument `ms + 0x70`).
const SET_ROTATION: u32 = 0x00cb_2d90;
const PROCESS_AGENT: u32 = 0x00cf_edd0;
/// Recalculates the bounding data of bodies (collision input, bodies, count).
const ENTITY_BATCH_RECALC_AABB: u32 = 0x00d1_a330;
const REMOVE_DUPLICATES: u32 = 0x00cf_7130;
const GET_COLLISION_ENTRIES_SORTED: u32 = 0x00d0_8260;
const ENTRIES_DETERMINISTIC_UNCHECKED: u32 = 0x00d0_81d0;
/// `hkSweptTransformUtil::backStepMotionState` (time, motion state).
const BACK_STEP_MOTION_STATE: u32 = 0x00cf_1b10;
/// `hkSweptTransformUtil::lerp2` (swept transform, time, motion state).
const INTERPOLATE_MOTION_STATE: u32 = 0x00cf_1510;
/// `hkSweptTransformUtil::freezeMotionState` (time, motion state).
const FREEZE_MOTION_STATE: u32 = 0x00cf_1d50;
/// `hkpWorldAgentUtil::invalidateTim` (body, collision input).
const INVALIDATE_TIM: u32 = 0x00cc_0ea0;
const SIMPLE_COLLISION_RESPONSE: u32 = 0x00d9_f250;
const FIRE_CONTACT_IMPULSE_LIMIT_BREACHED: u32 = 0x00d0_1700;
const FIRE_POST_SIMULATION_CALLBACK: u32 = 0x00d0_1380;
const FIRE_ISLAND_POST_COLLIDE_CALLBACK: u32 = 0x00d0_1650;
const GET_TOI_VIOLATING_VELOCITIES: u32 = 0x00d9_f6f0;
const QUICK_SORT_RECURSIVE: u32 = 0x00dd_7d00;
/// Comparator callbacks of `QUICK_SORT_RECURSIVE` (result pointer, a, b);
/// they have no function symbol of their own.
const COMPARE_ENTITIES: u32 = 0x00d0_bc60;
const COMPARE_CONSTRAINTS: u32 = 0x00d0_bc80;
/// `this` = the body, argument = a byte pointer; returns the byte pointer.
const FIXED_OR_KEYFRAMED_BYTE: u32 = 0x00c7_61f0;
const BASE_RESET_COLLISION_INFORMATION: u32 = 0x00cf_a430;
const CALC_CONTENT_STATISTICS_BASE: u32 = 0x00c9_0360;
const BASE_SIMULATION_CONSTRUCTOR: u32 = 0x00cf_87c0;
const BASE_SIMULATION_DESTRUCTOR: u32 = 0x00cf_86d0;
/// `hkpSimulation::reCollideAfterStepFailure`.
const RECOLLIDE_AFTER_STEP_FAILURE: u32 = 0x00cf_8820;
/// `hkpSimulation::snapSimulateTimeAndGetTimeToAdvanceTo` (result in ST0).
const SNAP_SIMULATE_TIME: u32 = 0x00cf_89c0;
/// The base simulation function `00cf9480` (`this`; first, second, third,
/// callback, 0).
const BASE_SIMULATION_CALLBACK_FUNCTION: u32 = 0x00cf_9480;
/// This unit's callback at `00d0daa0` (passed to `00cf9480`).
const CALLBACK_00D0DAA0: u32 = 0x00d0_daa0;
const RESOURCE_MANAGER_CONSTRUCTOR: u32 = 0x00d9_fb90;
/// `hkpWorld` function returning the current psi time in ST0.
const WORLD_GET_CURRENT_PSI_TIME: u32 = 0x00c9_1030;
const WORLD_UNLOCK_CALLBACK_A: u32 = 0x00c9_1120;
const WORLD_UNLOCK_CALLBACK_B: u32 = 0x00c9_1140;
const WORLD_LOCK_ISLAND: u32 = 0x00c9_11d0;
const WORLD_UNLOCK_ISLAND: u32 = 0x00c9_1210;
/// Function of `hkpworld.cpp` called on the solver block (two floats).
const WORLD_00C90E60: u32 = 0x00c9_0e60;
const BROAD_PHASE_LOCK: u32 = 0x00d9_7050;
const BROAD_PHASE_UNLOCK: u32 = 0x00d9_7060;
const DISPATCHER_REMOVE_PAIRS: u32 = 0x00cf_70f0;
const DISPATCHER_ADD_PAIRS: u32 = 0x00cf_7080;
/// `hkpConstraintQueryIn::set`.
const CONSTRAINT_QUERY_IN_SET: u32 = 0x00cf_a6e0;
/// `hkRigidMotionUtilStep` (pointer to a float, bodies, count, offset).
const RIGID_MOTION_UTIL_STEP: u32 = 0x00d2_8680;
/// `hkSolveUpload`.
const HK_SOLVE_UPLOAD: u32 = 0x00d8_92d0;
const INTERNAL_ADD_ACCUMULATORS: u32 = 0x00d8_8670;
const INTERNAL_ADD_JACOBIAN_SCHEMAS: u32 = 0x00d8_8760;
const SUB_SOLVE: u32 = 0x00d8_8340;
const ONE_STEP_INTEGRATE: u32 = 0x00d8_8580;
const SHUTDOWN_SOLVER: u32 = 0x00d8_8300;
/// `hkSolveStepJacobians`.
const SOLVE_STEP_JACOBIANS: u32 = 0x00d8_aac0;
/// Functions of `hkpconstraintsolversetup.cpp` and the solver set-up that
/// the engine map does not name.
const SOLVER_SETUP_00D881D0: u32 = 0x00d8_81d0;
const SOLVER_SETUP_00D88310: u32 = 0x00d8_8310;
const SOLVER_SETUP_00D88470: u32 = 0x00d8_8470;

// Error stream.
const STREAM_CONSTRUCTOR: u32 = 0x00cb_1600;
const STREAM_WRITE_TEXT: u32 = 0x00cc_01c0;
const STREAM_DESTRUCTOR: u32 = 0x00cc_00e0;
const ERROR_OBJECT: u32 = 0x0126_8360;
const ERROR_SOURCE_NAME: u32 = 0x010c_d4a8;
const TOI_QUEUE_FULL_TEXT: u32 = 0x010c_cc70;

// Data.
/// The abort flag (1 aborts the step).
const ABORT_FLAG: u32 = 0x0126_9200;
/// `float` "time of impact not set" (bits `0x7f7fffee`).
const TOI_TIME_NOT_SET: u32 = 0x0104_da48;
/// `double` the output's time is compared with.
const TOI_TIME_SENTINEL: u32 = 0x0104_e138;
/// `float[256]`.
const FLOAT_TABLE: u32 = 0x010c_7948;
const SIMPLE_RESPONSE_TOLERANCE: u32 = 0x0101_e2bc;
/// `double` 0.0078125.
const SIMPLE_RESPONSE_SCALE: u32 = 0x010c_c8d0;
const SIMPLE_RESPONSE_DEFAULT: u32 = 0x0104_da48;
const CONSTANT_01016248: u32 = 0x0101_6248;
const CONSTANT_0104F498: u32 = 0x0104_f498;
const SIMULATION_VTABLE: u32 = 0x010c_d45c;
const STATISTICS_NAME_TOI_EVENTS: u32 = 0x010c_d434;
const STATISTICS_NAME_TOI_RESOURCE_MANAGER: u32 = 0x010c_d420;
const STATISTICS_NAME_ENTITIES_MAP: u32 = 0x010c_d3f8;
const STATISTICS_BASE_ARGUMENT: u32 = 0x0126_9514;

// Layouts and offsets.
/// Size of a time-of-impact event.
const TOI_EVENT_SIZE: u32 = 0x70;
const EVENT_TIME: u32 = 0;
const EVENT_SEPARATING_VELOCITY: u32 = 4;
const EVENT_QUALITY_BYTE: u32 = 8;
const EVENT_ENTITY_A: u32 = 0xc;
const EVENT_ENTITY_B: u32 = 0x10;
const EVENT_CONTACT_MANAGER: u32 = 0x14;
const EVENT_PROPERTIES: u32 = 0x18;
const EVENT_FLAGS: u32 = 0x27;
const EVENT_CONTACT_POINT: u32 = 0x50;
const ENTRY_CONTACT_MANAGER: u32 = 8;
const ENTRY_QUALITY_INDEX: u32 = 0xc;
const ENTRY_COLLIDABLE_A: u32 = 0x10;
const ENTRY_COLLIDABLE_B: u32 = 0x14;
const COLLIDABLE_OWNER_OFFSET: u32 = 0x10;
const OUTPUT_CONTACT_POINT: u32 = 0x3010;
const OUTPUT_TOI_TIME: u32 = 0x3030;
const OUTPUT_SEPARATING_VELOCITY: u32 = 0x3034;
const OUTPUT_PROPERTIES: u32 = 0x3050;
const OUTPUT_EXTRA: u32 = 0x3080;
// `hkpEntity` (Xbox PDB names).
const ENTITY_WORLD: u32 = 8;
const ENTITY_COLLIDABLE: u32 = 0x10;
const ENTITY_QUALITY_TYPE: u32 = 0x2a;
const ENTITY_SOLVER_DATA: u32 = 0xa4;
const ENTITY_STORAGE_INDEX: u32 = 0xa8;
const ENTITY_MASTER_CONSTRAINTS: u32 = 0xac;
const ENTITY_MASTER_CONSTRAINT_COUNT: u32 = 0xb0;
const ENTITY_SLAVE_CONSTRAINTS: u32 = 0xb4;
const ENTITY_SLAVE_CONSTRAINT_COUNT: u32 = 0xb8;
const ENTITY_SIMULATION_ISLAND: u32 = 0xcc;
const ENTITY_UID: u32 = 0xd4;
const ENTITY_MOTION: u32 = 0xe0;
const ENTITY_MOTION_TYPE: u32 = 0xe8;
const ENTITY_MOTION_STATE: u32 = 0xf0;
const ENTITY_SWEPT_TRANSFORM: u32 = 0x130;
const ENTITY_CENTER_OF_MASS_0: u32 = 0x130;
const ENTITY_CENTER_OF_MASS_1: u32 = 0x140;
const ENTITY_INV_DELTA_TIME: u32 = 0x14c;
const ENTITY_DELTA_ANGLE: u32 = 0x180;
const ENTITY_LINEAR_VELOCITY: u32 = 0x1b0;
const ENTITY_ANGULAR_VELOCITY: u32 = 0x1c0;
// Constraint instance and its internal block.
const INSTANCE_DATA: u32 = 0xc;
const INSTANCE_PRIORITY: u32 = 0x1c;
const INSTANCE_INTERNAL: u32 = 0x30;
const INTERNAL_ENTITY_A: u32 = 4;
const INTERNAL_ENTITY_B: u32 = 8;
// `hkpSimulationIsland` (Xbox PDB names for the first two).
const ISLAND_ENTITY_DATA: u32 = 0x48;
const ISLAND_ENTITY_COUNT: u32 = 0x4c;
const ISLAND_BYTES_USED: u32 = 0x58;
const ISLAND_SECTORS: u32 = 0x5c;
const ISLAND_SECTOR_COUNT: u32 = 0x60;
// `hkpWorld` (Xbox PDB names).
const WORLD_SIMULATION: u32 = 8;
const WORLD_ACTIVE_ISLANDS: u32 = 0x28;
const WORLD_BROAD_PHASE: u32 = 0x58;
const WORLD_BROAD_PHASE_DISPATCHER: u32 = 0x64;
const WORLD_COLLISION_INPUT: u32 = 0x78;
const WORLD_COLLISION_FILTER: u32 = 0x7c;
const WORLD_COLLISION_DISPATCHER: u32 = 0x80;
const WORLD_PENDING_OPERATIONS_COUNT: u32 = 0x8c;
const WORLD_PENDING_BODY_OPERATIONS_COUNT: u32 = 0x90;
const WORLD_CRITICAL_OPERATIONS_LOCK_COUNT: u32 = 0x94;
const WORLD_BLOCK_EXECUTING_PENDING_OPERATIONS: u32 = 0x9c;
const WORLD_PENDING_OPERATION_QUEUE_COUNT: u32 = 0xa4;
const WORLD_ROTATE_NORMAL: u32 = 0xdc;
const WORLD_POST_SIMULATION_LISTENER_COUNT: u32 = 0x168;
const WORLD_ISLAND_POST_COLLIDE_LISTENER_COUNT: u32 = 0x198;
const WORLD_VIOLATED_CONSTRAINT_ARRAY: u32 = 0x1b8;
const WORLD_STEP_INFO: u32 = 0x1d0;
const WORLD_SCALED_2EC: u32 = 0x2ec;
const WORLD_SCALED_2F0: u32 = 0x2f0;
const WORLD_INTEGER_2F4: u32 = 0x2f4;
const WORLD_VALUE_300: u32 = 0x300;
const WORLD_SIZE_OF_TOI_EVENT_QUEUE: u32 = 0x334;
const WORLD_BROAD_PHASE_UPDATE_SIZE: u32 = 0x33c;
/// Step info words in the collision input (start, end, delta, inverse delta).
const COLLISION_INPUT_STEP_INFO: u32 = 0x50;
/// Fields of the simulation: the event array (data, size, capacity) and the
/// entities map at `+0x3c`.
const SIM_EVENT_ARRAY: u32 = 0x30;
const SIM_ENTITIES_MAP: u32 = 0x3c;

layout! {
    /// `hkpContinuousSimulation` (not in the Xbox PDB; names are by role and
    /// unconfirmed). 0x50 bytes: the base `hkpSimulation` (vtable, world at
    /// +0xc, ...) and the continuous simulation's own fields.
    pub struct HkpContinuousSimulation: 0x50 {
        /// Virtual table (`010cd45c`).
        0x00 vtable: u32,
        /// The world.
        0x0c world: Ptr,
        /// Byte set to 1 by the constructor.
        0x10 byte_10: u8,
        /// `float` the simulation time is set to.
        0x14 float_14: f32,
        /// `float` read as the end of a step.
        0x18 float_18: f32,
        /// `float` handed to the event handlers.
        0x1c float_1c: f32,
        /// `float` the target time is compared with.
        0x20 float_20: f32,
        /// Result of the last interrupted step.
        0x28 word_28: u32,
        /// Queued time-of-impact events (0x70 bytes each): data.
        0x30 event_data: u32,
        /// Number of queued events.
        0x34 event_count: i32,
        /// Capacity word (bit 31: not heap-allocated).
        0x38 event_capacity: u32,
        /// Resource manager of the time-of-impact solve.
        0x48 toi_resource_manager: Ptr,
        /// Counter of the handled events.
        0x4c processed_toi_count: i32,
    }
}

// Helpers.

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// The calling thread's profiling record stream (`TlsGetValue` of the slot in
/// `0126835c`).
fn profile_stream(e: &mut Engine) -> u32 {
    let slot = e.global::<u32>(PROFILE_SLOT);
    e.call(TLS_GET_VALUE, &args![slot]).u32()
}

/// Appends a profiling record (`label`, time stamp) to the stream while the
/// cursor at `+4` is below the limit at `+0xC`; each record takes 12 bytes.
fn profile_record(e: &mut Engine, label: u32) {
    let stream = profile_stream(e);
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// Like [`profile_record`] for a record with two labels (at `+0` and
/// `+0xC`) and the time stamp at `+4`: 16 bytes.
fn profile_record_pair(e: &mut Engine, label: u32, second: u32) {
    let stream = profile_stream(e);
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 0xc, second);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0x10);
    }
}

/// Takes `bytes` from the stack allocator and returns the old top.
fn stack_alloc(e: &mut Engine, bytes: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let top = e.mem.u32(router);
    e.mem.set_u32(
        router,
        top.wrapping_add(bytes.wrapping_add(0x7f) & 0xffff_ff80),
    );
    top
}

/// Puts the stack allocator's top back.
fn stack_release(e: &mut Engine, top: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, top);
}

/// The heap allocator (the router's object at `+0x10`).
fn heap_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Frees `bytes` at `pointer` through the heap allocator's virtual slot
/// `0x10`.
fn heap_free(e: &mut Engine, pointer: u32, bytes: u32) {
    let allocator = heap_allocator(e);
    e.vcall(allocator, 0x10, &args![pointer, bytes]);
}

/// Empties an array the way the exe's local arrays are destroyed: the size
/// becomes 0, the storage is freed (`capacity & 0x3fffffff` elements of
/// `element_size` bytes) unless bit 31 of the capacity word says it is not
/// the heap's, then the data pointer is cleared and the capacity word set to
/// `0x80000000`.
fn release_array(e: &mut Engine, array: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(element_size)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// Releases the local arrays `hkLs_localizedSolveToi` and `simulateToi`
/// share: the manager's body array, the stack allocator memory behind the
/// local body array (its size is cleared when it still points at the saved
/// top) and the record array.
fn release_solve_arrays(
    e: &mut Engine,
    manager_bodies: u32,
    solve_bodies: u32,
    records: u32,
    saved_top_slot: u32,
) {
    release_array(e, manager_bodies, 4);
    let saved = e.mem.u32(saved_top_slot);
    if saved == e.mem.u32(solve_bodies) {
        e.mem.set_u32(solve_bodies + 4, 0);
    }
    stack_release(e, saved);
    release_array(e, solve_bodies, 4);
    release_array(e, records, 12);
}

/// Locks the world's critical operations (`+0x94` counter).
fn world_lock(e: &mut Engine, world: u32) {
    let count = e.mem.u32(world + WORLD_CRITICAL_OPERATIONS_LOCK_COUNT);
    e.mem.set_u32(
        world + WORLD_CRITICAL_OPERATIONS_LOCK_COUNT,
        count.wrapping_add(1),
    );
}

/// Unlocks the world's critical operations: the counter goes down and, at 0
/// with the block byte (`+0x9c`) clear, the pending operations run (`+0x8c`
/// non-zero: `00c91120`; `+0xa4` equal to 1 and `+0x90` non-zero: `00c91140`).
fn world_unlock(e: &mut Engine, world: u32) {
    let count = e
        .mem
        .u32(world + WORLD_CRITICAL_OPERATIONS_LOCK_COUNT)
        .wrapping_sub(1);
    e.mem
        .set_u32(world + WORLD_CRITICAL_OPERATIONS_LOCK_COUNT, count);
    if count == 0 && e.mem.u8(world + WORLD_BLOCK_EXECUTING_PENDING_OPERATIONS) == 0 {
        if e.mem.u32(world + WORLD_PENDING_OPERATIONS_COUNT) != 0 {
            e.call(WORLD_UNLOCK_CALLBACK_A, &args![world]);
        }
        if e.mem.u32(world + WORLD_PENDING_OPERATION_QUEUE_COUNT) == 1
            && e.mem.u32(world + WORLD_PENDING_BODY_OPERATIONS_COUNT) != 0
        {
            e.call(WORLD_UNLOCK_CALLBACK_B, &args![world]);
        }
    }
}

/// Locks the broad phase when its word at `+0x10` is set.
fn broad_phase_lock(e: &mut Engine, broad_phase: u32) {
    if e.mem.u32(broad_phase + 0x10) != 0 {
        e.call(BROAD_PHASE_LOCK, &args![broad_phase]);
    }
}

/// Unlocks the broad phase when its word at `+0x10` is set.
fn broad_phase_unlock(e: &mut Engine, broad_phase: u32) {
    if e.mem.u32(broad_phase + 0x10) != 0 {
        e.call(BROAD_PHASE_UNLOCK, &args![broad_phase]);
    }
}

/// Reads four `float`s (an `hkVector4`).
fn read_vec4(e: &Engine, address: u32) -> [f32; 4] {
    [
        e.mem.f32(address),
        e.mem.f32(address + 4),
        e.mem.f32(address + 8),
        e.mem.f32(address + 12),
    ]
}

/// Writes four `float`s.
fn write_vec4(e: &mut Engine, address: u32, value: [f32; 4]) {
    for (i, lane) in value.iter().enumerate() {
        e.mem.set_f32(address + 4 * i as u32, *lane);
    }
}

/// Swaps two 12-byte records (equal addresses leave it unchanged).
fn swap_records(e: &mut Engine, first: u32, second: u32) {
    let a = [e.mem.u32(first), e.mem.u32(first + 4), e.mem.u32(first + 8)];
    let b = [
        e.mem.u32(second),
        e.mem.u32(second + 4),
        e.mem.u32(second + 8),
    ];
    for (i, word) in b.iter().enumerate() {
        e.mem.set_u32(first + 4 * i as u32, *word);
    }
    for (i, word) in a.iter().enumerate() {
        e.mem.set_u32(second + 4 * i as u32, *word);
    }
}

/// Swaps `record` with the record at `*counter` and counts it.
fn take_record_to_front(e: &mut Engine, records: u32, counter: u32, record: u32) {
    let slot = e.mem.u32(counter);
    e.mem.set_u32(counter, slot.wrapping_add(1));
    let target = e.mem.u32(records) + 12u32.wrapping_mul(slot);
    swap_records(e, record, target);
}

/// Whether the body's motion type (`+0xe8`) is 4 or 5.
fn is_fixed_or_keyframed(e: &Engine, body: u32) -> bool {
    let kind = e.mem.u8(body + ENTITY_MOTION_TYPE);
    kind == 5 || kind == 4
}

/// Takes event `index` out of the queue: the last event takes its place.
fn remove_event_at(e: &mut Engine, this: Ptr<HkpContinuousSimulation>, index: i32) {
    let size = e.get(this, HkpContinuousSimulation::event_count) - 1;
    e.set(this, HkpContinuousSimulation::event_count, size);
    if size != index {
        let data = e.get(this, HkpContinuousSimulation::event_data);
        let last = e
            .mem
            .bytes(data + size as u32 * TOI_EVENT_SIZE, TOI_EVENT_SIZE);
        e.mem.write(data + index as u32 * TOI_EVENT_SIZE, &last);
    }
}

/// Tells the event's contact manager (virtual slot `0x20`) about the event:
/// the island of the first body, or of the second when the first has motion
/// type 5, and the properties at `+0x18`.
fn notify_event_island(e: &mut Engine, event: u32) {
    let first = e.mem.u32(event + EVENT_ENTITY_A);
    let island = if e.mem.u8(first + ENTITY_MOTION_TYPE) == 5 {
        e.mem
            .u32(e.mem.u32(event + EVENT_ENTITY_B) + ENTITY_SIMULATION_ISLAND)
    } else {
        e.mem.u32(first + ENTITY_SIMULATION_ISLAND)
    };
    let manager = e.mem.u32(event + EVENT_CONTACT_MANAGER);
    e.vcall(manager, 0x20, &args![island, event + EVENT_PROPERTIES]);
}

/// Appends `value` to a word array that has room for it.
fn push_word(e: &mut Engine, array: u32, value: u32) {
    let size = e.mem.u32(array + 4);
    e.mem.set_u32(e.mem.u32(array) + 4 * size, value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Appends `value` to a word array, growing it first when full (`00c90940`
/// with the heap allocator and element size 4).
fn push_word_grow(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
    }
    push_word(e, array, value);
}

/// Appends the pair (`first`, `second`) to an array of 8-byte elements,
/// growing it first when full (element size 8).
fn push_pair_grow(e: &mut Engine, array: u32, first: u32, second: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, 8u32]);
    }
    let size = e.mem.u32(array + 4);
    let element = e.mem.u32(array) + 8 * size;
    e.mem.set_u32(element, first);
    e.mem.set_u32(element + 4, second);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Whether the output's time of impact (`float` at `+0x3030`) differs from
/// the `double` the exe compares it with (true for a NaN).
fn time_was_set(e: &Engine, output: u32) -> bool {
    f64::from(e.mem.f32(output + OUTPUT_TOI_TIME)) != e.mem.f64(TOI_TIME_SENTINEL)
}

/// The first used slot of a pointer map at or after `from`, or one past the
/// hash mask when there is none.
fn map_scan(e: &Engine, map: u32, from: i32) -> i32 {
    let hash_mask = e.mem.i32(map + 8);
    let mut slot = from;
    while slot <= hash_mask && e.mem.u32(e.mem.u32(map) + 8 * slot as u32) == 0xffff_ffff {
        slot += 1;
    }
    slot
}

/// The first used slot of a pointer map.
fn map_first_used(e: &Engine, map: u32) -> i32 {
    map_scan(e, map, 0)
}

/// The next used slot after `slot`.
fn map_next_used(e: &Engine, map: u32, slot: i32) -> i32 {
    map_scan(e, map, slot + 1)
}

/// Sets the state byte of a body in the table `*flags` (indexed by the `u16`
/// at body `+0xa8`).
fn set_body_state(e: &mut Engine, flags: u32, body: u32, state: u8) {
    let index = e.mem.u16(body + ENTITY_STORAGE_INDEX) as u32;
    let table = e.mem.u32(flags);
    e.mem.set_u8(index.wrapping_add(table), state);
}

/// Invalidates the time-of-impact data of every body in `bodies` and empties
/// the array.
fn invalidate_bodies_and_clear(e: &mut Engine, bodies: u32, input: u32) {
    let mut k = 0;
    while e.mem.i32(bodies + 4) > k {
        let body = e.mem.u32(e.mem.u32(bodies) + 4 * k as u32);
        e.call(INVALIDATE_TIM, &args![body, input]);
        k += 1;
    }
    e.mem.set_u32(bodies + 4, 0);
}

/// Gathers per body its broad-phase handle (`body + 0x24`) into `handles`
/// and its bounding box into `aabbs` (32 bytes each: the minimum at `+0`, the
/// maximum at `+0x10`, three words each) from the bodies' bounding data
/// (minimum words at `+0x30`, maximum words at `+0x40`, the offset bytes at
/// `+0x3c` and `+0x4c` scaled by the shift in the byte at `+0x3f`). A body
/// whose minimum x is above its maximum x first has its bounding data
/// recalculated (`00d1a330`).
fn gather_aabbs(e: &mut Engine, entities: u32, count: i32, world: u32, aabbs: u32, handles: u32) {
    e.with_stack(4, |e, slot| {
        for i in 0..count {
            let body = e.mem.u32(entities + 4 * i as u32);
            e.mem.set_u32(handles + 4 * i as u32, body + 0x24);
            let bounds = body + 0x30;
            if e.mem.u32(bounds) > e.mem.u32(bounds + 0x10) {
                e.mem.set_u32(slot.addr(), body);
                let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
                e.call(ENTITY_BATCH_RECALC_AABB, &args![input, slot.addr(), 1u32]);
            }
            let shift = u32::from(e.mem.u8(bounds + 0xf)) & 31;
            let out = aabbs + 0x20 * i as u32;
            for axis in 0..3u32 {
                let low = e.mem.u32(bounds + 4 * axis);
                let low_offset = u32::from(e.mem.u8(bounds + 0xc + axis)) << shift;
                e.mem.set_u32(out + 4 * axis, low.wrapping_sub(low_offset));
                let high = e.mem.u32(bounds + 0x10 + 4 * axis);
                let high_offset = u32::from(e.mem.u8(bounds + 0x1c + axis)) << shift;
                e.mem
                    .set_u32(out + 0x10 + 4 * axis, high.wrapping_add(high_offset));
            }
        }
    });
}

/// Handles one constraint of a body for `hkLs_toiActivateEntitiesAndCheckConstraints`.
/// `internal` is the constraint's internal block (instance at `+0`, bodies
/// at `+4` and `+8`). A constraint whose instance priority (`+0x1c` of the
/// instance) is below `min_priority` is skipped. The other body is the
/// one of the two that is not `body`; with motion type 5 the instance goes
/// into `constraints` and that is all. Otherwise nothing happens when the
/// other body's state is `0x10`; else the instance goes into `constraints`
/// and, unless the other body's state is 2 or more, the other body into
/// `new_bodies`, interpolated to `time` first when its state is 0, and its
/// state becomes 2.
#[allow(clippy::too_many_arguments)]
fn toi_visit_constraint(
    e: &mut Engine,
    internal: u32,
    body: u32,
    min_priority: i32,
    time: f32,
    flags: u32,
    new_bodies: u32,
    constraints: u32,
) {
    let instance = e.mem.u32(internal);
    let priority = i32::from(e.mem.u8(instance + INSTANCE_PRIORITY));
    if min_priority > priority {
        return;
    }
    let other =
        e.mem.u32(internal + INTERNAL_ENTITY_B) ^ e.mem.u32(internal + INTERNAL_ENTITY_A) ^ body;
    if e.mem.u8(other + ENTITY_MOTION_TYPE) == 5 {
        push_word_grow(e, constraints, instance);
        return;
    }
    let state_address = |e: &Engine| {
        e.mem
            .u32(flags)
            .wrapping_add(u32::from(e.mem.u16(other + ENTITY_STORAGE_INDEX)))
    };
    if e.mem.u8(state_address(e)) == 0x10 {
        return;
    }
    push_word_grow(e, constraints, instance);
    if e.mem.u8(state_address(e)) >= 2 {
        return;
    }
    push_word_grow(e, new_bodies, other);
    let address = state_address(e);
    if e.mem.u8(address) == 0 {
        e.mem.set_u8(address, 1);
        e.call(
            INTERPOLATE_MOTION_STATE,
            &args![
                other + ENTITY_SWEPT_TRANSFORM,
                x87_quiet(time),
                other + ENTITY_MOTION_STATE
            ],
        );
    }
    let address = state_address(e);
    e.mem.set_u8(address, 2);
}

// Translated from 00d0bc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the engine map. Works on a time-of-impact event: sets bit 0
/// of the flag byte at `event + 0x27`, asks the event's contact manager (the
/// word at `+0x14`; virtual slot `0x44`, arguments `event` and `arg`) to look
/// at the event, then stores the byte `!(flags >> 3) & 1` at `result` and
/// returns `result`. The register `ECX` is not used.
pub fn fn_00d0bc20(e: &mut Engine, result: u32, event: u32, arg: u32) -> u32 {
    let flags = e.mem.u8(event + EVENT_FLAGS);
    e.mem.set_u8(event + EVENT_FLAGS, flags | 1);
    let manager = e.mem.u32(event + EVENT_CONTACT_MANAGER);
    e.vcall(manager, 0x44, &args![event, arg]);
    let flags = e.mem.u8(event + EVENT_FLAGS);
    e.mem.set_u8(result, !(flags >> 3) & 1);
    result
}

// Translated from 00d0bc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::assertThereIsNoCollisionInformationForEntities`
/// (Xbox PDB): empty in this build. The body reads none of its three
/// arguments.
pub fn hkp_continuous_simulation_assert_there_is_no_collision_information_for_entities(
    _e: &mut Engine,
    _this: Ptr<HkpContinuousSimulation>,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) {
}

// Translated from 00d0bcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::assertThereIsNoCollisionInformationForAgent`
/// (Xbox PDB): empty in this build. The body does not read its argument.
pub fn hkp_continuous_simulation_assert_there_is_no_collision_information_for_agent(
    _e: &mut Engine,
    _this: Ptr<HkpContinuousSimulation>,
    _unused_1: u32,
) {
}

// Translated from 00d0bd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::warpTime` (Xbox PDB): adds `delta` to the time
/// of every queued time-of-impact event. The event list is read through the
/// simulation the world holds at `world + 8`, not through `this`.
pub fn hkp_continuous_simulation_warp_time(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    delta: f32,
) {
    let world = e.get(this, HkpContinuousSimulation::world).addr();
    let simulation = Ptr::<HkpContinuousSimulation>::new(e.mem.u32(world + WORLD_SIMULATION));
    let count = e.get(simulation, HkpContinuousSimulation::event_count);
    for i in 0..count {
        let events = e.get(simulation, HkpContinuousSimulation::event_data);
        let time_address = events + i as u32 * TOI_EVENT_SIZE + EVENT_TIME;
        let time = e.mem.f32(time_address);
        e.mem
            .set_f32(time_address, (time as f64 + delta as f64) as f32);
    }
}

// Translated from 00d0bd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLs_areVelocitiesOk` (Xbox PDB): index of the last element that raised
/// the limit, or -1.
///
/// The limit starts as the float at `+0x1c54 + priority * 0x40` of the
/// collision dispatcher (the word at `context`; `priority` is the signed byte
/// at `+0x1c` of the constraint instance in `info[0]`) times the float at
/// `info[2]`. Each of the `count` floats at `velocities` is taken as
/// `|v| * scale`, rounded to a float; an element strictly larger than the
/// limit (an ordered compare, so NaN never wins) becomes the new limit and the
/// result.
pub fn hk_ls_are_velocities_ok(
    e: &mut Engine,
    scale: f32,
    info: u32,
    velocities: u32,
    count: i32,
    context: u32,
) -> i32 {
    let instance = e.mem.u32(info);
    let priority = e.mem.i8(instance + INSTANCE_PRIORITY) as i32;
    let dispatcher = e.mem.u32(context);
    let table_limit = e
        .mem
        .f32(dispatcher.wrapping_add((priority << 6) as u32) + 0x1c54);
    let mut limit = table_limit * e.mem.f32(info + 8);
    let mut found = -1;
    for i in 0..count {
        let velocity = e.mem.f32(velocities + 4 * i as u32);
        let scaled = velocity.abs() * scale;
        if limit < scaled {
            limit = scaled;
            found = i;
        }
    }
    found
}

// Translated from 00d0be80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the engine map. Moves the constraint records that touch a
/// body of motion type 4 or 5 to the front of the records it looks at.
///
/// `array` is an `hkArray` of 12-byte records whose first word is a
/// constraint instance (its internal, at `+0x30`, names the two bodies at
/// `+4` and `+8`). Records `size - n ..` are scanned; every one whose first or
/// second body has motion type 4 or 5 is swapped with the record at
/// `*counter`, and `*counter` grows by one.
pub fn fn_00d0be80(e: &mut Engine, array: u32, counter: u32, n: i32) {
    let size = e.mem.i32(array + 4);
    let mut i = size.wrapping_sub(n);
    while i < size {
        let record = e.mem.u32(array) + 12 * i as u32;
        let instance = e.mem.u32(record);
        let internal = e.mem.u32(instance + INSTANCE_INTERNAL);
        let first = e.mem.u32(internal + INTERNAL_ENTITY_A);
        let second = e.mem.u32(internal + INTERNAL_ENTITY_B);
        if is_fixed_or_keyframed(e, first) || is_fixed_or_keyframed(e, second) {
            let slot = e.mem.u32(counter);
            e.mem.set_u32(counter, slot.wrapping_add(1));
            let target = e.mem.u32(array) + 12 * slot;
            swap_records(e, record, target);
        }
        i += 1;
    }
}

// Translated from 00d0bf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLs_toiResetVelocityAccumulatorsForEntities` (Xbox PDB): for every body
/// in the `hkArray` `entities`, writes the body's linear velocity (`+0x1b0`)
/// into its solver accumulator and rebuilds the accumulator's angular part.
///
/// The accumulator is `context[0xc] + body[0xa4]`; its 16 bytes at `+0x10`
/// receive the linear velocity. The 16 bytes at `+0x50` are saved, replaced
/// by the 16 bytes at `context[0x18] + (body[0xa4] >> 7) * 32 + 0x10`, and
/// `00c7f700` (a vector rotation, `this` = the accumulator + 0x20) is called
/// with them and the body's angular velocity (`+0x1c0`); then the saved bytes
/// are put back.
pub fn hk_ls_toi_reset_velocity_accumulators_for_entities(
    e: &mut Engine,
    context: u32,
    entities: u32,
) {
    let mut i = 0;
    while e.mem.i32(entities + 4) > i {
        let body = e.mem.u32(e.mem.u32(entities) + 4 * i as u32);
        let solver_data = e.mem.u32(body + ENTITY_SOLVER_DATA);
        let accumulator = e.mem.u32(context + 0xc).wrapping_add(solver_data);
        let velocity = e.mem.bytes(body + ENTITY_LINEAR_VELOCITY, 16);
        e.mem.write(accumulator + 0x10, &velocity);
        let saved = e.mem.bytes(accumulator + 0x50, 16);
        let table = e.mem.u32(context + 0x18);
        let source = table.wrapping_add((solver_data >> 7) << 5) + 0x10;
        let replacement = e.mem.bytes(source, 16);
        e.mem.write(accumulator + 0x50, &replacement);
        e.call(
            SET_ROTATED_DIR,
            &args![
                accumulator + 0x20,
                accumulator + 0x50,
                body + ENTITY_ANGULAR_VELOCITY
            ],
        );
        e.mem.write(accumulator + 0x50, &saved);
        i += 1;
    }
}

// Translated from 00d0bfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::removeCollisionInformationForAgent` (Xbox PDB):
/// drops every queued time-of-impact event whose contact manager (the word
/// at `+0x14`) is the one of `entry` (the word at `entry + 8`). A removed
/// event is replaced by the last one.
pub fn hkp_continuous_simulation_remove_collision_information_for_agent(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    entry: u32,
) {
    let mut i = e.get(this, HkpContinuousSimulation::event_count) - 1;
    while i >= 0 {
        let events = e.get(this, HkpContinuousSimulation::event_data);
        let manager = e
            .mem
            .u32(events + i as u32 * TOI_EVENT_SIZE + EVENT_CONTACT_MANAGER);
        if manager == e.mem.u32(entry + ENTRY_CONTACT_MANAGER) {
            remove_event_at(e, this, i);
        }
        i -= 1;
    }
}

// Translated from 00d0c030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::collideEntitiesBroadPhaseContinuousFindPairs`
/// (Xbox PDB): asks the broad phase which pairs of the `n` bodies in
/// `entities` start or stop overlapping, without changing the agents.
///
/// Does nothing when `n` is zero. Otherwise (between the profiling records
/// `LtBroadPhase`/`StGatherAabbs`, `St3AxisSweep`, `StRemoveDup` and the end
/// record `01076f38`) it builds, in the stack allocator, a bounding box (32
/// bytes) and a broad-phase handle (`body + 0x24`) per body, recomputes a
/// body's bounding data first when its minimum is above its maximum
/// (`00d1a330`), calls the world's broad phase (virtual slot `0x2c`, locked
/// around the call while the word at `+0x10` of the broad phase is set) with
/// the two pair arrays `added` and `removed`, releases the stack memory and,
/// when the arrays hold any pair, removes the duplicates between them
/// (`00cf7130`).
pub fn hkp_continuous_simulation_collide_entities_broad_phase_continuous_find_pairs(
    e: &mut Engine,
    entities: u32,
    n: i32,
    world: u32,
    added: u32,
    removed: u32,
) {
    if n == 0 {
        return;
    }
    profile_record_pair(e, PROFILE_LT_BROAD_PHASE, PROFILE_ST_GATHER_AABBS);
    let aabbs = stack_alloc(e, (n as u32) << 5);
    let handles = stack_alloc(e, (n as u32) << 2);
    gather_aabbs(e, entities, n, world, aabbs, handles);
    profile_record(e, PROFILE_ST_3AXIS_SWEEP);
    let broad_phase = e.mem.u32(world + WORLD_BROAD_PHASE);
    broad_phase_lock(e, broad_phase);
    e.vcall(broad_phase, 0x2c, &args![handles, aabbs, n, added, removed]);
    let broad_phase = e.mem.u32(world + WORLD_BROAD_PHASE);
    broad_phase_unlock(e, broad_phase);
    stack_release(e, handles);
    stack_release(e, aabbs);
    if e.mem.i32(added + 4).wrapping_add(e.mem.i32(removed + 4)) > 0 {
        profile_record(e, PROFILE_ST_REMOVE_DUP);
        e.call(REMOVE_DUPLICATES, &args![added, removed]);
    }
    profile_record(e, PROFILE_LT);
}

// Translated from 00d0c290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the engine map. Cancels the queued time-of-impact events that
/// involve any of the `count` bodies in `entities`.
///
/// Does nothing when no event is queued. The bodies go into a local pointer
/// map (`00cb1fe0` reserve, `00cb1870` insert with value 0); an event is
/// cancelled when `00cb18f0` finds either of its two bodies (a result at or
/// below the map's hash mask means found). A cancelled event is reported to
/// its contact manager (virtual slot `0x20`, with the simulation island of the
/// event's first body, or of the second when the first has motion type 5, and
/// the event's properties at `+0x18`) and removed, the last event taking its
/// place. The map is cleared (`00cb1a90`) and destroyed (`00a29680`) at the
/// end.
pub fn fn_00d0c290(e: &mut Engine, this: Ptr<HkpContinuousSimulation>, entities: u32, count: i32) {
    if e.get(this, HkpContinuousSimulation::event_count) == 0 {
        return;
    }
    // The exe's frame (`SUB ESP,0xc`, four pushes): 0x1c bytes, the map at +0x10 (00d0c2ac).
    e.with_stack(0x1c, |e, frame| {
        let map = frame.addr() + 0x10;
        e.mem.set_u32(map, 0);
        e.mem.set_u32(map + 4, 0);
        e.mem.set_u32(map + 8, 0xffff_ffff);
        let allocator = heap_allocator(e);
        e.call(MAP_RESERVE, &args![map, allocator, count]);
        for i in 0..count {
            let key = e.mem.u32(entities + 4 * i as u32);
            let allocator = heap_allocator(e);
            e.call(MAP_INSERT, &args![map, allocator, key, 0u32]);
        }
        let mut i = 0;
        while i < e.get(this, HkpContinuousSimulation::event_count) {
            let event =
                e.get(this, HkpContinuousSimulation::event_data) + i as u32 * TOI_EVENT_SIZE;
            let first_key = e.mem.u32(event + EVENT_ENTITY_A);
            let first = e.call(MAP_FIND_KEY, &args![map, first_key]).i32();
            let second_key = e.mem.u32(event + EVENT_ENTITY_B);
            let second = e.call(MAP_FIND_KEY, &args![map, second_key]).i32();
            let hash_mod = e.mem.i32(map + 8);
            if first <= hash_mod || second <= hash_mod {
                notify_event_island(e, event);
                remove_event_at(e, this, i);
                i -= 1;
            }
            i += 1;
        }
        let allocator = heap_allocator(e);
        e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
        e.call(MAP_DESTRUCTOR, &args![map]);
    });
}

// Translated from 00d0c3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the engine map. Cancels the queued time-of-impact events whose
/// first or second body is `entity`: each is reported to its contact manager
/// (virtual slot `0x20`, the island of the event's first body or, when that
/// has motion type 5, of the second, and the properties at `+0x18`) and
/// removed, the last event taking its place.
pub fn fn_00d0c3e0(e: &mut Engine, this: Ptr<HkpContinuousSimulation>, entity: u32) {
    let mut i = 0;
    while e.get(this, HkpContinuousSimulation::event_count) > i {
        let event = e.get(this, HkpContinuousSimulation::event_data) + i as u32 * TOI_EVENT_SIZE;
        if e.mem.u32(event + EVENT_ENTITY_A) == entity
            || e.mem.u32(event + EVENT_ENTITY_B) == entity
        {
            notify_event_island(e, event);
            remove_event_at(e, this, i);
            i -= 1;
        }
        i += 1;
    }
}

// Translated from 00d0c480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLs_doSimpleCollisionResponse` (Xbox PDB): resolves the contact of a
/// time-of-impact `event` with `00d9f250` (the simple collision response) and
/// collects the bodies that should be re-activated into the `hkArray`
/// `output` (no capacity check, bodies pushed at `output[size++]`).
///
/// The response gets the contact point (`event + 0x50`), the event time, a
/// 24-byte settings block (the float table `010c7948` indexed by the bytes at
/// `event + 0x24` and `+0x26`, the byte at `+0x25` times 0.0078125, the
/// event's separating velocity at `+4`, 0.1 from `0101e2bc`, and
/// `extra_float`; a zero byte at `+0x26` selects the float at `0104da48`
/// instead of the table), both bodies' motions (`+0xe0`), the contact manager
/// and a result block whose floats at `+4` and `+8` (called `first` and
/// `second` here) and byte at `+0x38` are read back.
///
/// When the byte is set, a 16-byte report (`{u16 2, u16 1, manager + 0x44,
/// event + 0x18, event + 0x50}`) goes to `00d01700` and nothing is collected.
/// Otherwise (a body "movable" means motion type neither 4 nor 5): with
/// `first < 0`, both movable bodies when `second < 0`, the first when
/// `second >= 0` and it is movable (else the second); with `first >= 0` and
/// `second < 0`, the first when `00c761f0` on the second body says its byte
/// is set (else the second); every other case, NaN included, pushes the
/// second body when the first is not movable, else the first when the second
/// is not movable, else the second only when `second < first`.
pub fn hk_ls_do_simple_collision_response(
    e: &mut Engine,
    world: u32,
    event: u32,
    extra_float: f32,
    output: u32,
) {
    let body_a = e.mem.u32(event + EVENT_ENTITY_A);
    let body_b = e.mem.u32(event + EVENT_ENTITY_B);
    let manager = e.mem.u32(event + EVENT_CONTACT_MANAGER);
    e.with_stack(0xa0, |e, block| {
        let settings = block.addr();
        let result = settings + 0x20;
        let report = settings + 0x70;
        let byte_out = settings + 0x80;
        let table_value =
            |e: &Engine, index: u8| x87_quiet_bits(e.mem.u32(FLOAT_TABLE + 4 * index as u32));
        let index_24 = e.mem.u8(event + 0x24);
        let scale_25 = e.mem.u8(event + 0x25);
        let selector_26 = e.mem.u8(event + 0x26);
        let separating_velocity = x87_quiet_bits(e.mem.u32(event + EVENT_SEPARATING_VELOCITY));
        let tolerance = x87_quiet_bits(e.mem.u32(SIMPLE_RESPONSE_TOLERANCE));
        let extra = x87_quiet_bits(extra_float.to_bits());
        let first_setting = table_value(e, index_24);
        let scale: f64 = e.global(SIMPLE_RESPONSE_SCALE);
        e.mem.set_u32(settings, first_setting);
        e.mem
            .set_f32(settings + 4, (scale_25 as f64 * scale) as f32);
        let third = if selector_26 == 0 {
            x87_quiet_bits(e.mem.u32(SIMPLE_RESPONSE_DEFAULT))
        } else {
            table_value(e, selector_26)
        };
        e.mem.set_u32(settings + 8, third);
        e.mem.set_u32(settings + 0xc, separating_velocity);
        e.mem.set_u32(settings + 0x10, tolerance);
        e.mem.set_u32(settings + 0x14, extra);
        let time = x87_quiet_bits(e.mem.u32(event + EVENT_TIME));
        e.call(
            SIMPLE_COLLISION_RESPONSE,
            &args![
                event + 0x50,
                time,
                settings,
                body_a + ENTITY_MOTION,
                body_b + ENTITY_MOTION,
                manager,
                result
            ],
        );
        if e.mem.u8(result + 0x38) != 0 {
            e.mem.set_u16(report, 2);
            e.mem.set_u16(report + 2, 1);
            e.mem.set_u32(report + 4, manager + 0x44);
            e.mem.set_u32(report + 8, event + 0x18);
            e.mem.set_u32(report + 0xc, event + 0x50);
            e.call(
                FIRE_CONTACT_IMPULSE_LIMIT_BREACHED,
                &args![world, report, 1u32],
            );
            return;
        }
        let first = e.mem.f32(result + 4);
        let second = e.mem.f32(result + 8);
        let movable = |e: &Engine, body: u32| !is_fixed_or_keyframed(e, body);
        // The branch that every case without its own rule ends in.
        let pair_rule = |e: &mut Engine| {
            if !movable(e, body_a) {
                push_word(e, output, body_b);
            } else if !movable(e, body_b) {
                push_word(e, output, body_a);
            } else if second < first {
                push_word(e, output, body_b);
            } else {
                push_word(e, output, body_a);
            }
        };
        if first < 0.0 {
            if second < 0.0 {
                if movable(e, body_a) {
                    push_word(e, output, body_a);
                }
                if movable(e, body_b) {
                    push_word(e, output, body_b);
                }
            } else if second >= 0.0 {
                if movable(e, body_a) {
                    push_word(e, output, body_a);
                } else {
                    push_word(e, output, body_b);
                }
            } else {
                pair_rule(e);
            }
        } else if first >= 0.0 && second < 0.0 {
            let flag = e
                .call(FIXED_OR_KEYFRAMED_BYTE, &args![body_b, byte_out])
                .u32();
            if e.mem.u8(flag) != 0 {
                push_word(e, output, body_a);
            } else {
                push_word(e, output, body_b);
            }
        } else {
            pair_rule(e);
        }
    });
}

// Translated from 00d0c6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the engine map. Moves the constraint records whose two bodies
/// both carry the state byte `0x10` in the table `*flags` (indexed by each
/// body's storage index, the `u16` at `+0xa8`) to the front of the records
/// it looks at.
///
/// `array` is an `hkArray` of 12-byte records whose first word is a
/// constraint instance (its internal, at `+0x30`, names the two bodies at
/// `+4` and `+8`). Records from `*counter` to the end are scanned; each match
/// is swapped with the record at `*counter`, and `*counter` grows by one.
pub fn fn_00d0c6d0(e: &mut Engine, array: u32, counter: u32, flags: u32) {
    let mut i = e.mem.i32(counter);
    while i < e.mem.i32(array + 4) {
        let record = e.mem.u32(array) + 12 * i as u32;
        let instance = e.mem.u32(record);
        let internal = e.mem.u32(instance + INSTANCE_INTERNAL);
        let first = e.mem.u32(internal + INTERNAL_ENTITY_A);
        let table = e.mem.u32(flags);
        let first_state = e
            .mem
            .u8(table + e.mem.u16(first + ENTITY_STORAGE_INDEX) as u32);
        if first_state == 0x10 {
            let second = e.mem.u32(internal + INTERNAL_ENTITY_B);
            let second_state = e
                .mem
                .u8(table + e.mem.u16(second + ENTITY_STORAGE_INDEX) as u32);
            if second_state == 0x10 {
                let slot = e.mem.u32(counter);
                e.mem.set_u32(counter, slot.wrapping_add(1));
                let target = e.mem.u32(array) + 12 * slot;
                swap_records(e, record, target);
            }
        }
        i += 1;
    }
}

// Translated from 00d0c790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLs_restoreTransformOnBodiesWithUpdatedTransform` (Xbox PDB): for each
/// body of the island (the `hkArray` at `island + 0x48`) whose state byte in
/// `*flags` is 1 or 2, rebuilds the body's motion-state rotation from its
/// quaternion (`00cb2d90`, `this` = body + 0xf0, argument body + 0x160) and
/// recomputes the translation at `+0x30` as `[+0x50] - (rotation * [+0x80])`
/// (lane by lane: `y * row1 + x * row0`, then `+ z * row2`, subtracted from
/// the vector at `+0x50`). For a state byte of 8 or 2 it also copies the
/// body's solver-accumulator velocities back: the accumulator
/// (`context[0xc] + body[0xa4]`) gets the 16 bytes at
/// `context[0x18] + (body[0xa4] >> 7) * 32 + 0x10` at `+0x50`, its `+0x10`
/// goes to the body's linear velocity (`+0x1b0`), and the body's angular
/// velocity (`+0x1c0`) is the accumulator's vector at `+0x20` multiplied by
/// the 3x3 matrix whose rows are the accumulator's `+0x50`, `+0x60`, `+0x70`
/// (as the shuffles build it, including the code's fourth lane).
pub fn hk_ls_restore_transform_on_bodies_with_updated_transform(
    e: &mut Engine,
    island: u32,
    flags: u32,
    context: u32,
) {
    let mut i = 0;
    while e.mem.i32(island + ISLAND_ENTITY_COUNT) > i {
        let state = e.mem.u8(e.mem.u32(flags) + i as u32);
        if state == 1 || state == 2 {
            let body = e
                .mem
                .u32(e.mem.u32(island + ISLAND_ENTITY_DATA) + 4 * i as u32);
            let motion_state = body + ENTITY_MOTION_STATE;
            e.call(SET_ROTATION, &args![motion_state, motion_state + 0x70]);
            let vector = read_vec4(e, motion_state + 0x80);
            let row0 = read_vec4(e, motion_state);
            let row1 = read_vec4(e, motion_state + 0x10);
            let row2 = read_vec4(e, motion_state + 0x20);
            let base = read_vec4(e, motion_state + 0x50);
            let mut result = [0.0f32; 4];
            for lane in 0..4 {
                let sum = vector[1] * row1[lane] + vector[0] * row0[lane];
                result[lane] = base[lane] - (sum + vector[2] * row2[lane]);
            }
            write_vec4(e, motion_state + 0x30, result);
        }
        if state == 8 || state == 2 {
            let body = e
                .mem
                .u32(e.mem.u32(island + ISLAND_ENTITY_DATA) + 4 * i as u32);
            let solver_data = e.mem.u32(body + ENTITY_SOLVER_DATA);
            let accumulator = e.mem.u32(context + 0xc).wrapping_add(solver_data);
            let table = e.mem.u32(context + 0x18);
            let source = table.wrapping_add((solver_data >> 7) << 5) + 0x10;
            let replacement = e.mem.bytes(source, 16);
            e.mem.write(accumulator + 0x50, &replacement);
            let velocity = e.mem.bytes(accumulator + 0x10, 16);
            e.mem.write(body + ENTITY_LINEAR_VELOCITY, &velocity);
            let r0 = read_vec4(e, accumulator + 0x50);
            let r1 = read_vec4(e, accumulator + 0x60);
            let r2 = read_vec4(e, accumulator + 0x70);
            let v = read_vec4(e, accumulator + 0x20);
            // The shuffles build three columns out of the three rows.
            let column0 = [r0[0], r1[0], r2[0], r2[1]];
            let column1 = [r0[1], r1[1], r2[1], r2[3]];
            let column2 = [r0[2], r1[2], r2[2], r2[3]];
            let mut result = [0.0f32; 4];
            for lane in 0..4 {
                let sum = v[1] * column1[lane] + v[0] * column0[lane];
                result[lane] = sum + v[2] * column2[lane];
            }
            write_vec4(e, body + ENTITY_ANGULAR_VELOCITY, result);
        }
        i += 1;
    }
}

// Translated from 00d0c8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::calcContentStatistics` (Xbox PDB): reports the
/// simulation's memory to the statistics `collector` (the second argument is
/// not read).
///
/// When the event array was heap-allocated (capacity word not flagged) or the
/// collector's byte at `+0x10` is set, the event array is reported under
/// `"toiEvents"` (virtual slot `0x14`: kind 3, name, data, used bytes, the
/// allocated size from `00d06f80`). The resource manager is reported under
/// `"toiResourceManager"` (slot `0x0c`), the pointer map under
/// `"entitiesNeedingPsiCollisionDetection"` (slot `0x14`, kind 1, size from
/// `00cb17e0`), and `00c90360` reports the base object.
pub fn hkp_continuous_simulation_calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    collector: u32,
    _unused_1: u32,
) {
    let this_address = this.addr();
    let capacity = e.get(this, HkpContinuousSimulation::event_capacity);
    if (capacity as i32) >= 0 || e.mem.u8(collector + 0x10) != 0 {
        let data = e.get(this, HkpContinuousSimulation::event_data);
        let allocated = e
            .call(
                ALLOCATED_SIZE,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(TOI_EVENT_SIZE)],
            )
            .u32();
        let count = e.get(this, HkpContinuousSimulation::event_count);
        e.vcall(
            collector,
            0x14,
            &args![
                3u32,
                STATISTICS_NAME_TOI_EVENTS,
                data,
                (count as u32).wrapping_mul(TOI_EVENT_SIZE),
                allocated
            ],
        );
    }
    let manager = e.get(this, HkpContinuousSimulation::toi_resource_manager);
    e.vcall(
        collector,
        0xc,
        &args![STATISTICS_NAME_TOI_RESOURCE_MANAGER, manager.addr(), 0u32],
    );
    let map = this_address + SIM_ENTITIES_MAP;
    let map_size = e.call(MAP_GET_MEM_SIZE, &args![map]).u32();
    let map_data = e.mem.u32(map);
    let allocated = e.call(ALLOCATED_SIZE, &args![map_data, map_size]).u32();
    let map_data = e.mem.u32(map);
    e.vcall(
        collector,
        0x14,
        &args![
            1u32,
            STATISTICS_NAME_ENTITIES_MAP,
            map_data,
            map_size,
            allocated
        ],
    );
    e.call(
        CALC_CONTENT_STATISTICS_BASE,
        &args![this_address, collector, STATISTICS_BASE_ARGUMENT],
    );
}

// Translated from 00d0c970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::resetCollisionInformationForEntities`
/// (Xbox PDB): lets the base simulation reset the collision information
/// (`00cfa430`, same four arguments) and, when bit 0 of `flags` is set,
/// cancels the queued time-of-impact events of the `count` bodies in
/// `entities`: with one body through `00d0c3e0`, otherwise through
/// `00d0c290`. `unused_3` is passed on to the base function only.
pub fn hkp_continuous_simulation_reset_collision_information_for_entities(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    entities: u32,
    count: i32,
    unused_3: u32,
    flags: u32,
) {
    e.call(
        BASE_RESET_COLLISION_INFORMATION,
        &args![this, entities, count, unused_3, flags],
    );
    if flags & 1 != 0 {
        if count == 1 {
            let entity = e.mem.u32(entities);
            fn_00d0c3e0(e, this, entity);
        } else {
            fn_00d0c290(e, this, entities, count);
        }
    }
}

// Translated from 00d0c9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::processAgentNnEntries` (Xbox PDB): runs the
/// narrow phase on each of the `count` agent entries in `entries` and hands a
/// produced contact to the entry's contact manager.
///
/// Between the profiling records `TtNarrowPhaseTOI` and `Et`. Each entry gets
/// a fresh output block (a pointer to its own buffer at `+0`, `owner` at
/// `+4`, the float at `0104da48` as the time at `+0x3030`, zero at `+0x3050`,
/// `+0x3054` and `+0x3080`), the collision input gets the quality block of the
/// dispatcher (`0x1de0` when `use_toi_quality` is set, else
/// `0x1c20 + 0x40 * quality` for the entry's signed quality byte at `+0xc`)
/// and the creation word at `0x1c30 + 0x40 * quality`, and `00cfedd0` runs the
/// agent. When the output no longer points at its own buffer the manager's
/// virtual slot `0x18` is called with the entry's two collidables, the input
/// and the output. The compiler's stack probe (`00ec8660`, 0x3094 bytes) is
/// not modelled.
/// Virtual method: the first word is `this` (`ECX` in the exe), which the body never reads.
pub fn hkp_continuous_simulation_process_agent_nn_entries(
    e: &mut Engine,
    _this: u32,
    entries: u32,
    count: i32,
    input: u32,
    owner: u32,
    use_toi_quality: u32,
) {
    profile_record(e, PROFILE_TT_NARROW_PHASE_TOI);
    e.with_stack(0x3090, |e, output| {
        let output = output.addr();
        let mut i = 0;
        while i < count {
            let entry = e.mem.u32(entries + 4 * i as u32);
            let dispatcher = e.mem.u32(input);
            e.mem.set_u32(output + OUTPUT_PROPERTIES, 0);
            e.mem.set_u32(output + OUTPUT_PROPERTIES + 4, 0);
            e.mem.set_u32(
                output + OUTPUT_TOI_TIME,
                x87_quiet_bits(e.mem.u32(TOI_TIME_NOT_SET)),
            );
            e.mem.set_u32(output, output + 0x10);
            e.mem.set_u32(output + 4, owner);
            e.mem.set_u32(output + OUTPUT_EXTRA, 0);
            let quality = (e.mem.i8(entry + ENTRY_QUALITY_INDEX) as i32) << 6;
            let quality_info = if use_toi_quality as u8 != 0 {
                dispatcher.wrapping_add(0x1de0)
            } else {
                dispatcher.wrapping_add(quality as u32).wrapping_add(0x1c20)
            };
            e.mem.set_u32(input + 0x60, quality_info);
            let creation = e
                .mem
                .u32(dispatcher.wrapping_add(quality as u32).wrapping_add(0x1c30));
            e.mem.set_u32(input + 0x14, creation);
            let manager = e.mem.u32(entry + ENTRY_CONTACT_MANAGER);
            e.call(PROCESS_AGENT, &args![entry, input, output, manager]);
            if e.mem.u32(output) != output + 0x10 {
                let manager = e.mem.u32(entry + ENTRY_CONTACT_MANAGER);
                let first = e.mem.u32(entry + ENTRY_COLLIDABLE_A);
                let second = e.mem.u32(entry + ENTRY_COLLIDABLE_B);
                e.vcall(manager, 0x18, &args![first, second, input, output]);
            }
            i += 1;
        }
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00d0cb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLs_toiCheckValidityOfConstraints` (Xbox PDB): looks at the constraint
/// records of a time-of-impact solve and flags the bodies whose constraints
/// are violated.
///
/// First the solver accumulators of the `entities` (an `hkArray` of bodies;
/// accumulator = `context[0xc] + body[0xa4]`) are saved into stack memory
/// (their 16-byte vectors at `+0x10` and `+0x20`, 32 bytes per body) and
/// replaced by the body's displacement over the step (`[+0x140] - [+0x130]`
/// times the float at `+0x14c`) and its angular displacement (`[+0x180]`
/// times the same float). Then every record of `records` (12 bytes, the
/// first word a constraint instance) from `*counter` on whose priority byte
/// (`+0x1c`) is at least `min_priority` is looked at: the "other" body is the
/// internal's second body when the first has state `0x10` in the table
/// `*flags` (indexed by `u16` at body `+0xa8`), else the first. When the
/// other body is already flagged 4 the record is swapped to `*counter`
/// (which grows by one); otherwise `00d9f6f0` fills a buffer of up to 0x100
/// velocities for the record and `hkLs_areVelocitiesOk` decides: when it
/// returns anything but -1 the other body gets flag 4, is appended to the
/// `out_entities` array (growing it through `00c90940`) and the record is
/// swapped to `*counter`. Finally the saved accumulators are restored and
/// the stack allocator is released.
pub fn hk_ls_toi_check_validity_of_constraints(
    e: &mut Engine,
    context: u32,
    conversion_info: u32,
    records: u32,
    counter: u32,
    min_priority: i32,
    flags: u32,
    entities: u32,
    out_entities: u32,
) {
    let entity_count = e.mem.u32(entities + 4);
    let saved_top = stack_alloc(e, entity_count << 5);
    let mut cursor = saved_top;
    let mut i = 0;
    while e.mem.i32(entities + 4) > i {
        let body = e.mem.u32(e.mem.u32(entities) + 4 * i as u32);
        let accumulator = e
            .mem
            .u32(body + ENTITY_SOLVER_DATA)
            .wrapping_add(e.mem.u32(context + 0xc));
        let linear = e.mem.bytes(accumulator + 0x10, 16);
        e.mem.write(cursor, &linear);
        cursor += 0x10;
        let angular = e.mem.bytes(accumulator + 0x20, 16);
        e.mem.write(cursor, &angular);
        cursor += 0x10;
        let start = read_vec4(e, body + ENTITY_CENTER_OF_MASS_0);
        let end = read_vec4(e, body + ENTITY_CENTER_OF_MASS_1);
        let factor = x87_quiet(e.mem.f32(body + ENTITY_INV_DELTA_TIME));
        let delta_angle = read_vec4(e, body + ENTITY_DELTA_ANGLE);
        let mut displacement = [0.0f32; 4];
        let mut angular_displacement = [0.0f32; 4];
        for lane in 0..4 {
            displacement[lane] = factor * (end[lane] - start[lane]);
            angular_displacement[lane] = factor * delta_angle[lane];
        }
        write_vec4(e, accumulator + 0x10, displacement);
        write_vec4(e, accumulator + 0x20, angular_displacement);
        i += 1;
    }
    let mut i = e.mem.i32(counter);
    if i < e.mem.i32(records + 4) {
        e.with_stack(0x400, |e, velocities| {
            while i < e.mem.i32(records + 4) {
                let record = e.mem.u32(records) + 12 * i as u32;
                let instance = e.mem.u32(record);
                let priority = e.mem.u8(instance + INSTANCE_PRIORITY) as i32;
                if min_priority <= priority {
                    let internal = e.mem.u32(instance + INSTANCE_INTERNAL);
                    let first = e.mem.u32(internal + INTERNAL_ENTITY_A);
                    let table = e.mem.u32(flags);
                    let first_state = e
                        .mem
                        .u8(table + e.mem.u16(first + ENTITY_STORAGE_INDEX) as u32);
                    let other = if first_state == 0x10 {
                        e.mem.u32(internal + INTERNAL_ENTITY_B)
                    } else {
                        first
                    };
                    let other_slot = table + e.mem.u16(other + ENTITY_STORAGE_INDEX) as u32;
                    if e.mem.u8(other_slot) & 4 != 0 {
                        take_record_to_front(e, records, counter, record);
                    } else {
                        let count = e
                            .call(
                                GET_TOI_VIOLATING_VELOCITIES,
                                &args![
                                    e.mem.u32(context + 4),
                                    e.mem.u32(record + 4),
                                    e.mem.u32(context + 0xc),
                                    0x100u32,
                                    velocities
                                ],
                            )
                            .i32();
                        let scale = x87_quiet(e.mem.f32(e.mem.u32(context) + 8));
                        let worst = hk_ls_are_velocities_ok(
                            e,
                            scale,
                            record,
                            velocities.addr(),
                            count,
                            conversion_info,
                        );
                        if worst != -1 {
                            let state = e.mem.u8(other_slot);
                            e.mem.set_u8(other_slot, state | 4);
                            push_word_grow(e, out_entities, other);
                            take_record_to_front(e, records, counter, record);
                        }
                    }
                }
                i += 1;
            }
        });
    }
    let mut cursor = saved_top;
    let mut i = 0;
    while e.mem.i32(entities + 4) > i {
        let body = e.mem.u32(e.mem.u32(entities) + 4 * i as u32);
        let accumulator = e
            .mem
            .u32(body + ENTITY_SOLVER_DATA)
            .wrapping_add(e.mem.u32(context + 0xc));
        let linear = e.mem.bytes(cursor, 16);
        e.mem.write(accumulator + 0x10, &linear);
        cursor += 0x10;
        let angular = e.mem.bytes(cursor, 16);
        e.mem.write(accumulator + 0x20, &angular);
        cursor += 0x10;
        i += 1;
    }
    stack_release(e, saved_top);
}

// Translated from 00d0cde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLs_backstepAndFreezeEntireIsland` (Xbox PDB): moves every body of the
/// island (the `hkArray` at `island + 0x48`) back to time `time` and makes
/// that its swept-transform start.
///
/// For each body: `00cf1b10` (back-step the motion state at `+0xf0`); when
/// its state byte in the table `*flags` is not `0x10` the body is appended to
/// `added` (growing it through `00c90940`) and marked `0x10`; the 16-byte
/// vectors at `+0x130` and `+0x150` are overwritten with those at `+0x140`
/// and `+0x160`; the float at `+0x13c` becomes `time` and the float at
/// `+0x14c` becomes `1 / (psiTime - time)`, `psiTime` being what `00c91030`
/// returns for the world at `island + 0x18`.
pub fn hk_ls_backstep_and_freeze_entire_island(
    e: &mut Engine,
    time: f32,
    island: u32,
    flags: u32,
    added: u32,
) {
    let mut i = 0;
    while e.mem.i32(island + ISLAND_ENTITY_COUNT) > i {
        let body = e
            .mem
            .u32(e.mem.u32(island + ISLAND_ENTITY_DATA) + 4 * i as u32);
        let motion_state = body + ENTITY_MOTION_STATE;
        e.call(
            BACK_STEP_MOTION_STATE,
            &args![x87_quiet(time), motion_state],
        );
        let state_slot = e.mem.u32(flags) + i as u32;
        if e.mem.u8(state_slot) != 0x10 {
            push_word_grow(e, added, body);
            e.mem.set_u8(e.mem.u32(flags) + i as u32, 0x10);
        }
        let end_center = e.mem.bytes(motion_state + 0x50, 16);
        e.mem.write(motion_state + 0x40, &end_center);
        let end_rotation = e.mem.bytes(motion_state + 0x70, 16);
        e.mem.write(motion_state + 0x60, &end_rotation);
        e.mem
            .set_u32(motion_state + 0x4c, x87_quiet_bits(time.to_bits()));
        let world = e.mem.u32(island + 0x18);
        let psi_time = e.call(WORLD_GET_CURRENT_PSI_TIME, &args![world]).f32();
        let inverse = (1.0 / (psi_time as f64 - time as f64)) as f32;
        e.mem.set_f32(motion_state + 0x5c, inverse);
        i += 1;
    }
}

// Translated from 00d0ceb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLs_toiCheckFinalValidityOfCriticalConstraints` (Xbox PDB): checks the
/// constraint records of priority 5 (the instance's priority byte at +0x1c)
/// one more time and stores at `*result` a byte that says whether all of them
/// are fine.
///
/// For each such record `00d9f6f0` fills a buffer of up to 0x100 velocities
/// and `hkLs_areVelocitiesOk` (with `conversion_info`) is asked. A record with
/// a non-negative answer `n` makes the result 0; when `out_records` is null
/// the function stops right there with 0. Otherwise the record is appended
/// to `out_records` (12-byte elements, growing through `00c90940`): the
/// instance, and two words that are 0 unless the constraint data's virtual
/// slot `0x20` returns `0xb`, in which case they point into the block at
/// `data[0x28]`: `X + 0x30 + (n << 5)` and
/// `X + 0x30 + u8[X + 0xa] * (i16)n + (u16[X + 6] << 5)`. Returns `result`.
pub fn hk_ls_toi_check_final_validity_of_critical_constraints(
    e: &mut Engine,
    result: u32,
    context: u32,
    records: u32,
    conversion_info: u32,
    out_records: u32,
) -> u32 {
    let mut all_fine = 1u8;
    let stopped = e.mem.i32(records + 4) > 0
        && e.with_stack(0x400, |e, velocities| {
            let mut i = 0;
            while i < e.mem.i32(records + 4) {
                let record = e.mem.u32(records) + 12 * i as u32;
                let instance = e.mem.u32(record);
                if e.mem.u8(instance + INSTANCE_PRIORITY) == 5 {
                    let count = e
                        .call(
                            GET_TOI_VIOLATING_VELOCITIES,
                            &args![
                                e.mem.u32(context + 4),
                                e.mem.u32(record + 4),
                                e.mem.u32(context + 0xc),
                                0x100u32,
                                velocities
                            ],
                        )
                        .i32();
                    let scale = x87_quiet(e.mem.f32(e.mem.u32(context) + 8));
                    let worst = hk_ls_are_velocities_ok(
                        e,
                        scale,
                        record,
                        velocities.addr(),
                        count,
                        conversion_info,
                    );
                    if worst >= 0 {
                        if out_records == 0 {
                            return true;
                        }
                        all_fine = 0;
                        let router = e.call(MEMORY_ROUTER, &args![]).u32();
                        let capacity = e.mem.u32(out_records + 8) & 0x3fff_ffff;
                        if e.mem.u32(out_records + 4) == capacity {
                            let allocator = e.mem.u32(router + 0x10);
                            e.call(ARRAY_GROW, &args![allocator, out_records, 0xcu32]);
                        }
                        let size = e.mem.u32(out_records + 4);
                        let element = e.mem.u32(out_records) + 12 * size;
                        e.mem.set_u32(out_records + 4, size + 1);
                        e.mem.set_u32(element, instance);
                        let data = e.mem.u32(instance + INSTANCE_DATA);
                        let kind = e.vcall(data, 0x20, &args![]).u32();
                        let (second, third) = if kind == 0xb {
                            let block = e.mem.u32(e.mem.u32(instance + INSTANCE_DATA) + 0x28);
                            let second =
                                ((worst as u32) << 5).wrapping_add(block).wrapping_add(0x30);
                            let block = e.mem.u32(e.mem.u32(instance + INSTANCE_DATA) + 0x28);
                            let stride = e.mem.u8(block + 0xa) as i32;
                            let by_index = stride.wrapping_mul(worst as i16 as i32) as u32;
                            let third = (e.mem.u16(block + 6) as u32)
                                .wrapping_shl(5)
                                .wrapping_add(by_index.wrapping_add(block))
                                .wrapping_add(0x30);
                            (second, third)
                        } else {
                            (0, 0)
                        };
                        e.mem.set_u32(element + 4, second);
                        e.mem.set_u32(element + 8, third);
                    }
                }
                i += 1;
            }
            false
        });
    // Stopping early stores 0 and leaves the rest of the records alone.
    e.mem.set_u8(result, if stopped { 0 } else { all_fine });
    result
}

// Translated from 00d0d020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::hkpContinuousSimulation` (Xbox PDB): builds the
/// simulation for `world` and returns `this`.
///
/// Runs the base constructor `00cf87c0(world)`, sets the vtable `010cd45c`,
/// makes the event array empty (not heap-allocated: capacity word
/// `0x80000000`) and the pointer map at `+0x3c` empty (hash mask -1, reserved
/// for 0 entries through `00cb1fe0`), sets the byte at `+0x10`, allocates the
/// 16-byte resource manager through the heap allocator (virtual slot 4; its
/// size field, the `u16` at `+4`, is set to 0x10) and constructs it with
/// `00d9fb90`, clears the counter at `+0x4c` and reserves room for
/// `world[0x334]` events (at least twice the current capacity) through
/// `00c908c0`.
pub fn hkp_continuous_simulation_new(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    world: u32,
) -> Ptr<HkpContinuousSimulation> {
    let this_address = this.addr();
    e.call(BASE_SIMULATION_CONSTRUCTOR, &args![this, world]);
    e.set(this, HkpContinuousSimulation::vtable, SIMULATION_VTABLE);
    e.set(this, HkpContinuousSimulation::event_data, 0);
    e.set(this, HkpContinuousSimulation::event_count, 0);
    e.set(this, HkpContinuousSimulation::event_capacity, 0x8000_0000);
    let map = this_address + SIM_ENTITIES_MAP;
    e.mem.set_u32(map, 0);
    e.mem.set_u32(map + 4, 0);
    e.mem.set_u32(map + 8, 0xffff_ffff);
    let allocator = heap_allocator(e);
    e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
    e.set(this, HkpContinuousSimulation::byte_10, 1);
    let allocator = heap_allocator(e);
    let manager = e.vcall(allocator, 4, &args![0x10u32]).u32();
    e.mem.set_u16(manager + 4, 0x10);
    let manager = e.call(RESOURCE_MANAGER_CONSTRUCTOR, &args![manager]).u32();
    e.set(
        this,
        HkpContinuousSimulation::toi_resource_manager,
        Ptr::new(manager),
    );
    e.set(this, HkpContinuousSimulation::processed_toi_count, 0);
    let wanted = e.mem.i32(world + WORLD_SIZE_OF_TOI_EVENT_QUEUE);
    let allocator = heap_allocator(e);
    let capacity = (e.get(this, HkpContinuousSimulation::event_capacity) & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let doubled = capacity.wrapping_add(capacity);
        let reserve = if wanted >= doubled { wanted } else { doubled };
        e.call(
            ARRAY_RESERVE,
            &args![
                allocator,
                this_address + SIM_EVENT_ARRAY,
                reserve,
                TOI_EVENT_SIZE
            ],
        );
    }
    this
}

// Translated from 00d0d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::~hkpContinuousSimulation` (Xbox PDB): resets the
/// vtable to this class's, releases the resource manager (virtual slot 0 with
/// argument 1), clears and destroys the pointer map (`00cb1a90`, `00a29680`),
/// empties the event array (freeing it when it was heap-allocated) and ends
/// with the base destructor `00cf86d0`.
pub fn hkp_continuous_simulation_destroy(e: &mut Engine, this: Ptr<HkpContinuousSimulation>) {
    let this_address = this.addr();
    let manager = e.get(this, HkpContinuousSimulation::toi_resource_manager);
    e.set(this, HkpContinuousSimulation::vtable, SIMULATION_VTABLE);
    if manager.addr() != 0 {
        e.vcall(manager.addr(), 0, &args![1u32]);
    }
    let map = this_address + SIM_ENTITIES_MAP;
    let allocator = heap_allocator(e);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
    e.call(MAP_DESTRUCTOR, &args![map]);
    let allocator = heap_allocator(e);
    let capacity = e.get(this, HkpContinuousSimulation::event_capacity);
    e.set(this, HkpContinuousSimulation::event_count, 0);
    if (capacity as i32) >= 0 {
        let data = e.get(this, HkpContinuousSimulation::event_data);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(TOI_EVENT_SIZE)],
        );
    }
    e.set(this, HkpContinuousSimulation::event_data, 0);
    e.set(this, HkpContinuousSimulation::event_capacity, 0x8000_0000);
    e.call(BASE_SIMULATION_DESTRUCTOR, &args![this]);
}

// Translated from 00d0d150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::addToiEvent` (Xbox PDB): queues a time-of-impact
/// event built from the narrow-phase `output` block and the agent `entry`.
///
/// When the event array is full (size equal to the capacity bits of the
/// capacity word) it only reports an error: the text at `010ccc70` goes to a
/// 0x200-byte stream (`00cb1600`, `00cc01c0`, destroyed by `00cc00e0`) and to
/// the error object in the global `01268360` (virtual slot `0x0c`: severity 1,
/// id `0xf0323454`, the text, the source name `010cd4a8` and line `0x43b`).
/// Otherwise a 0x70-byte event is appended: its floats at `+0x18`/`+0x1c`
/// start at 0, then `+0` is the output's time (`+0x3030`), `+4` its separating
/// velocity (`+0x3034`), the byte at `+8` is the byte at `+0x1c34 + 0x40 *
/// quality` of the world's collision dispatcher (`world + 0x80`, quality the
/// signed byte at `entry + 0xc`), `+0xc` and `+0x10` are the two bodies (each
/// collidable of the entry plus its signed owner offset, the byte at `+0x10`),
/// `+0x14` the contact manager (`entry + 8`), `+0x18..+0x2c` the five words
/// of the output at `+0x3050`, `+0x2c..+0x48` its seven words from `+0x3064`,
/// and `+0x50..+0x70` the 32 bytes at `+0x3010`.
pub fn hkp_continuous_simulation_add_toi_event(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    output: u32,
    entry: u32,
) {
    let size = e.get(this, HkpContinuousSimulation::event_count);
    let capacity = e.get(this, HkpContinuousSimulation::event_capacity) & 0x3fff_ffff;
    if size as u32 == capacity {
        // The exe's frame (`AND ESP,0xfffffff0`, `SUB ESP,0x218`, two pushes): 0x220 bytes from a
        // 16-byte boundary, the stream at +0x14 (00d0d175) and its text buffer at +0x20
        // (00d0d170).
        e.with_stack(0x220 + 0x10, |e, raw| {
            let frame = (raw.addr() + 0xf) & !0xf;
            let stream = frame + 0x14;
            let text_buffer = frame + 0x20;
            e.call(STREAM_CONSTRUCTOR, &args![stream, text_buffer, 0x200u32]);
            e.call(STREAM_WRITE_TEXT, &args![stream, TOI_QUEUE_FULL_TEXT]);
            let error_object = e.mem.u32(ERROR_OBJECT);
            e.vcall(
                error_object,
                0xc,
                &args![
                    1u32,
                    0xf032_3454u32,
                    text_buffer,
                    ERROR_SOURCE_NAME,
                    0x43bu32
                ],
            );
            e.call(STREAM_DESTRUCTOR, &args![stream]);
        });
        return;
    }
    let data = e.get(this, HkpContinuousSimulation::event_data);
    e.set(this, HkpContinuousSimulation::event_count, size + 1);
    let event = data + size as u32 * TOI_EVENT_SIZE;
    e.mem.set_f32(event + 0x18, 0.0);
    e.mem.set_f32(event + 0x1c, 0.0);
    e.mem.set_u32(
        event + EVENT_TIME,
        x87_quiet_bits(e.mem.u32(output + OUTPUT_TOI_TIME)),
    );
    let world = e.get(this, HkpContinuousSimulation::world).addr();
    let dispatcher = e.mem.u32(world + WORLD_COLLISION_DISPATCHER);
    let quality = (e.mem.i8(entry + ENTRY_QUALITY_INDEX) as i32) << 6;
    let quality_byte = e
        .mem
        .u8(dispatcher.wrapping_add(quality as u32).wrapping_add(0x1c34));
    e.mem.set_u8(event + EVENT_QUALITY_BYTE, quality_byte);
    e.mem.set_u32(
        event + EVENT_SEPARATING_VELOCITY,
        x87_quiet_bits(e.mem.u32(output + OUTPUT_SEPARATING_VELOCITY)),
    );
    let contact_point = e.mem.bytes(output + OUTPUT_CONTACT_POINT, 32);
    e.mem.write(event + EVENT_CONTACT_POINT, &contact_point);
    let collidable_a = e.mem.u32(entry + ENTRY_COLLIDABLE_A);
    let body_a =
        collidable_a.wrapping_add(e.mem.i8(collidable_a + COLLIDABLE_OWNER_OFFSET) as i32 as u32);
    e.mem.set_u32(event + EVENT_ENTITY_A, body_a);
    let collidable_b = e.mem.u32(entry + ENTRY_COLLIDABLE_B);
    let body_b =
        collidable_b.wrapping_add(e.mem.i8(collidable_b + COLLIDABLE_OWNER_OFFSET) as i32 as u32);
    e.mem.set_u32(event + EVENT_ENTITY_B, body_b);
    for k in 0..5 {
        let word = e.mem.u32(output + OUTPUT_PROPERTIES + 4 * k);
        e.mem.set_u32(event + EVENT_PROPERTIES + 4 * k, word);
    }
    let manager = e.mem.u32(entry + ENTRY_CONTACT_MANAGER);
    e.mem.set_u32(event + EVENT_CONTACT_MANAGER, manager);
    for k in 0..7 {
        let word = e.mem.u32(output + OUTPUT_PROPERTIES + 0x14 + 4 * k);
        e.mem.set_u32(event + EVENT_PROPERTIES + 0x14 + 4 * k, word);
    }
}

// Translated from 00d0d2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLs_collectAgentEntriesToProcess` (Xbox PDB): appends to the `hkArray`
/// `output` (growing it through `00c90940`) the agent entries of `entity`
/// that the time-of-impact solve has to process.
///
/// `00d08260` fills a local array with (entry, other collidable) pairs for
/// the entity's collidable. A pair is used when `min_quality` is not above
/// the entry's unsigned quality byte (`+0xc`). The other body is the
/// collidable plus its signed owner offset. A body of motion type 5 is
/// always used; any other needs its state byte in the table `*flags` (index:
/// the `u16` at `+0xa8`) to be below 8, and a state of 0 is changed to 1 and
/// the body's swept transform (`+0x130`) interpolated to `time` into its
/// motion state (`00cf1510`). The local array is freed at the end when it was
/// heap-allocated.
pub fn hk_ls_collect_agent_entries_to_process(
    e: &mut Engine,
    entity: u32,
    min_quality: i32,
    time: f32,
    flags: u32,
    output: u32,
) {
    // The exe's frame (`SUB ESP,0xc`, four pushes): 0x1c bytes, the sorted entries at +0x10
    // (00d0d2c8).
    e.with_stack(0x1c, |e, frame| {
        let local = frame.addr() + 0x10;
        e.mem.set_u32(local, 0);
        e.mem.set_u32(local + 4, 0);
        e.mem.set_u32(local + 8, 0x8000_0000);
        e.call(
            GET_COLLISION_ENTRIES_SORTED,
            &args![entity + ENTITY_COLLIDABLE, local],
        );
        let mut i = 0;
        while e.mem.i32(local + 4) > i {
            let pair = e.mem.u32(local) + 8 * i as u32;
            let entry = e.mem.u32(pair);
            if min_quality <= e.mem.u8(entry + ENTRY_QUALITY_INDEX) as i32 {
                let collidable = e.mem.u32(pair + 4);
                let other = collidable
                    .wrapping_add(e.mem.i8(collidable + COLLIDABLE_OWNER_OFFSET) as i32 as u32);
                let mut collect = true;
                if e.mem.u8(other + ENTITY_MOTION_TYPE) != 5 {
                    let slot = e.mem.u32(flags) + e.mem.u16(other + ENTITY_STORAGE_INDEX) as u32;
                    let state = e.mem.u8(slot);
                    if state >= 8 {
                        collect = false;
                    } else if state == 0 {
                        e.mem.set_u8(slot, 1);
                        e.call(
                            INTERPOLATE_MOTION_STATE,
                            &args![
                                other + ENTITY_SWEPT_TRANSFORM,
                                x87_quiet(time),
                                other + ENTITY_MOTION_STATE
                            ],
                        );
                    }
                }
                if collect {
                    push_word_grow(e, output, entry);
                }
            }
            i += 1;
        }
        let capacity = e.mem.u32(local + 8);
        e.mem.set_u32(local + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(local);
            heap_free(e, data, (capacity & 0x3fff_ffff).wrapping_mul(8));
        }
    });
}

// Translated from 00d0d3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::collideEntitiesBroadPhaseContinuous` (Xbox PDB):
/// like `...FindPairs` (`00d0c030`), but then also updates the agents: it
/// removes the agents of the pairs that stopped overlapping and adds the ones
/// that started.
///
/// Does nothing when `n` is zero. Between the profiling records
/// `LtBroadPhase`/`StGatherAabbs`, `St3AxisSweep`, `StRemoveDup`, `StRemoveAgt`,
/// `StAddAgt` and the end record `01076f38`: two local pair arrays (stack
/// allocator, `world[0x33c]` elements of 8 bytes each, capacity word flagged
/// "not owned"), the bounding boxes and handles (as in `00d0c030`), the broad
/// phase call (virtual slot `0x2c`, locked while the word at `+0x10` of the
/// broad phase is set), duplicate removal (`00cf7130`) when there are pairs.
/// When `export_flag` is not null the function waits (records `TtWaitForExport`
/// and `Et`) until the byte there is 1 or 2. When there are pairs the island
/// of the first body is locked (`00c911d0`), the pairs to remove go to
/// `00cf70f0`, and, if the allocator's virtual slot `0x24` accepts
/// `added * 1024` bytes, the pairs to add go to `00cf7080` with the world
/// collision filter plus 8 (or 0 when there is none); the island is unlocked
/// (`00c91210`). The stack memory is released in the reverse order and the
/// arrays are freed when they outgrew the stack memory.
pub fn hkp_continuous_simulation_collide_entities_broad_phase_continuous(
    e: &mut Engine,
    entities: u32,
    n: i32,
    world: u32,
    export_flag: u32,
) {
    if n == 0 {
        return;
    }
    profile_record_pair(e, PROFILE_LT_BROAD_PHASE, PROFILE_ST_GATHER_AABBS);
    // The exe's frame (`SUB ESP,0x38`, four pushes): 0x48 bytes, the removed pairs at +0x28 and
    // the added pairs at +0x38 (00d0d445 / 00d0d482; each 16 bytes with the saved stack top).
    e.with_stack(0x48, |e, frame| {
        let added = frame.addr() + 0x38;
        let removed = frame.addr() + 0x28;
        let pair_capacity = e.mem.u32(world + WORLD_BROAD_PHASE_UPDATE_SIZE);
        for array in [added, removed] {
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
        }
        let added_top = stack_alloc(e, pair_capacity.wrapping_mul(8));
        e.mem.set_u32(added + 8, pair_capacity | 0x8000_0000);
        e.mem.set_u32(added, added_top);
        let pair_capacity = e.mem.u32(world + WORLD_BROAD_PHASE_UPDATE_SIZE);
        let removed_top = stack_alloc(e, pair_capacity.wrapping_mul(8));
        e.mem.set_u32(removed, removed_top);
        e.mem.set_u32(removed + 8, pair_capacity | 0x8000_0000);
        let aabbs = stack_alloc(e, (n as u32) << 5);
        let handles = stack_alloc(e, (n as u32) << 2);
        gather_aabbs(e, entities, n, world, aabbs, handles);
        profile_record(e, PROFILE_ST_3AXIS_SWEEP);
        let broad_phase = e.mem.u32(world + WORLD_BROAD_PHASE);
        broad_phase_lock(e, broad_phase);
        e.vcall(broad_phase, 0x2c, &args![handles, aabbs, n, added, removed]);
        let broad_phase = e.mem.u32(world + WORLD_BROAD_PHASE);
        broad_phase_unlock(e, broad_phase);
        stack_release(e, handles);
        stack_release(e, aabbs);
        if e.mem.i32(removed + 4).wrapping_add(e.mem.i32(added + 4)) > 0 {
            profile_record(e, PROFILE_ST_REMOVE_DUP);
            e.call(REMOVE_DUPLICATES, &args![added, removed]);
        }
        if export_flag != 0 {
            profile_record(e, PROFILE_TT_WAIT_FOR_EXPORT);
            loop {
                if e.mem.u8(export_flag) == 1 {
                    break;
                }
                if e.mem.u8(export_flag) == 2 {
                    break;
                }
            }
            profile_record(e, PROFILE_END);
        }
        if e.mem.i32(removed + 4).wrapping_add(e.mem.i32(added + 4)) > 0 {
            profile_record(e, PROFILE_ST_REMOVE_AGENTS);
            let island = e.mem.u32(e.mem.u32(entities) + ENTITY_SIMULATION_ISLAND);
            e.call(WORLD_LOCK_ISLAND, &args![world, island]);
            let dispatcher = e.mem.u32(world + WORLD_BROAD_PHASE_DISPATCHER);
            let removed_data = e.mem.u32(removed);
            let removed_size = e.mem.u32(removed + 4);
            e.call(
                DISPATCHER_REMOVE_PAIRS,
                &args![dispatcher, removed_data, removed_size],
            );
            let added_size = e.mem.u32(added + 4);
            let allocator = heap_allocator(e);
            let accepted = e.vcall(allocator, 0x24, &args![added_size << 10]).u8() != 0;
            if accepted {
                profile_record(e, PROFILE_ST_ADD_AGENTS);
                let filter = e.mem.u32(world + WORLD_COLLISION_FILTER);
                let filter_part = if filter != 0 { filter + 8 } else { 0 };
                let dispatcher = e.mem.u32(world + WORLD_BROAD_PHASE_DISPATCHER);
                let added_data = e.mem.u32(added);
                e.call(
                    DISPATCHER_ADD_PAIRS,
                    &args![dispatcher, added_data, added_size, filter_part],
                );
            }
            let island = e.mem.u32(e.mem.u32(entities) + ENTITY_SIMULATION_ISLAND);
            e.call(WORLD_UNLOCK_ISLAND, &args![world, island]);
        }
        profile_record(e, PROFILE_LT);
        // Release the pair arrays: the stack memory in reverse order, and the
        // heap memory of an array that outgrew it.
        for (array, top) in [(removed, removed_top), (added, added_top)] {
            if e.mem.u32(array) == top {
                e.mem.set_u32(array + 4, 0);
            }
            stack_release(e, top);
            let capacity = e.mem.u32(array + 8);
            e.mem.set_u32(array + 4, 0);
            if (capacity as i32) >= 0 {
                let data = e.mem.u32(array);
                heap_free(e, data, (capacity & 0x3fff_ffff).wrapping_mul(8));
            }
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
        }
    });
}

// Translated from 00d0d8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::collideIslandNarrowPhaseContinuous` (Xbox PDB):
/// runs the narrow phase on every agent entry of the island and queues the
/// time-of-impact events they produce.
///
/// Between the profiling records `TtNarrowPhase` and `Et`. The island keeps
/// its agent entries in sectors of 0x200 bytes (pointer array at `+0x5c`,
/// count at `+0x60`; the last sector holds the number of bytes at `+0x58`);
/// each entry's size is its byte at `+3`. For each entry the output block is
/// reset (pointer to its own buffer at `+0`, the island at `+4`, the float at
/// `0104da48` as time at `+0x3030`, `+0x3080` zero; `+0x3050`/`+0x3054` are
/// zeroed once), the collision input gets the quality block `0x1c20 + 0x40 *
/// quality` of the dispatcher (and the word at its `+0x10`), and `00cfedd0`
/// runs the agent. A contact goes to the contact manager (virtual slot
/// `0x18`); a time different from the double at `0104e138` becomes an event
/// (`addToiEvent`). The abort flag at `01269200` stops the loop. The compiler's
/// stack probe (`00ec8660`, 0x30a4 bytes) is not modelled.
pub fn hkp_continuous_simulation_collide_island_narrow_phase_continuous(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    island: u32,
    input: u32,
) {
    profile_record(e, PROFILE_TT_NARROW_PHASE);
    e.with_stack(0x30b0, |e, output| {
        let output = output.addr();
        e.mem.set_u32(
            output + OUTPUT_TOI_TIME,
            x87_quiet_bits(e.mem.u32(TOI_TIME_NOT_SET)),
        );
        e.mem.set_u32(output + OUTPUT_PROPERTIES, 0);
        e.mem.set_u32(output + OUTPUT_PROPERTIES + 4, 0);
        e.mem.set_u32(output + 4, island);
        let mut sector = 0;
        while sector < e.mem.i32(island + ISLAND_SECTOR_COUNT) {
            let start = e
                .mem
                .u32(e.mem.u32(island + ISLAND_SECTORS) + 4 * sector as u32);
            sector += 1;
            let end = if sector == e.mem.i32(island + ISLAND_SECTOR_COUNT) {
                e.mem.u32(island + ISLAND_BYTES_USED).wrapping_add(start)
            } else {
                start + 0x200
            };
            let mut entry = start;
            while entry < end {
                let collidable_b = e.mem.u32(entry + ENTRY_COLLIDABLE_B);
                let collidable_a = e.mem.u32(entry + ENTRY_COLLIDABLE_A);
                let dispatcher = e.mem.u32(input);
                let quality = (e.mem.i8(entry + ENTRY_QUALITY_INDEX) as i32) << 6;
                let quality_info = dispatcher.wrapping_add(quality as u32).wrapping_add(0x1c20);
                e.mem.set_u32(input + 0x60, quality_info);
                e.mem.set_u32(
                    output + OUTPUT_TOI_TIME,
                    x87_quiet_bits(e.mem.u32(TOI_TIME_NOT_SET)),
                );
                let creation = e.mem.u32(quality_info + 0x10);
                e.mem.set_u32(input + 0x14, creation);
                e.mem.set_u32(output, output + 0x10);
                e.mem.set_u32(output + OUTPUT_EXTRA, 0);
                let manager = e.mem.u32(entry + ENTRY_CONTACT_MANAGER);
                e.call(PROCESS_AGENT, &args![entry, input, output, manager]);
                if e.global::<u32>(ABORT_FLAG) == 1 {
                    return;
                }
                if e.mem.u32(output) != output + 0x10 {
                    let manager = e.mem.u32(entry + ENTRY_CONTACT_MANAGER);
                    e.vcall(
                        manager,
                        0x18,
                        &args![collidable_a, collidable_b, input, output],
                    );
                }
                if time_was_set(e, output) {
                    hkp_continuous_simulation_add_toi_event(e, this, output, entry);
                }
                entry = entry.wrapping_add(e.mem.u8(entry + 3) as u32);
            }
        }
    });
    profile_record(e, PROFILE_END);
}

// Translated from 00d0db40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::collideEntitiesOfOneIslandNarrowPhaseContinuous_toiOnly`
/// (Xbox PDB): runs the time-of-impact narrow phase for the `count` bodies in
/// `entities`, all of one island, and queues the events.
///
/// A byte table with one entry per body of the island (storage index; the
/// island's body count at `+0x4c`) is allocated on the stack allocator and
/// cleared (`00cb1280`); between the profiling records `TtNarrowPhaseTOI` and
/// `Et` each listed body marks its entry 1 and its collision entries
/// (`00d08260`, pairs of entry and collidable) are looked at. An entry whose
/// other body is in the same island and already marked is skipped. When the
/// dispatcher's quality block (`world.dispatcher + 0x1c20 + 0x40 * quality`)
/// has a zero creation word at `+0x10` or a non-zero byte at `+0x14`, the
/// agent is processed now (`00cfedd0`, the contact manager's virtual slot
/// `0x18`, `addToiEvent` for a time different from the double at `0104e138`).
/// Otherwise the body is put, under its uid (`+0xd4`), into `map` with a
/// reference (`00c90510`) unless the uid is already there (`00cb1930` /
/// `00cb1870`), to be handled by the PSI pass. The abort flag at `01269200`
/// ends the function early. The compiler's stack probe (`00ec8660`) is not
/// modelled.
pub fn hkp_continuous_simulation_collide_entities_of_one_island_narrow_phase_continuous_toi_only(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    entities: u32,
    count: i32,
    input: u32,
    map: u32,
) {
    let first = e.mem.u32(entities);
    let island = e.mem.u32(first + ENTITY_SIMULATION_ISLAND);
    let island_size = e.mem.u32(island + ISLAND_ENTITY_COUNT);
    let flags = stack_alloc(e, island_size);
    let flags_capacity = island_size | 0x8000_0000;
    e.call(MEM_SET, &args![flags, 0u32, island_size]);
    e.with_stack(0x30c8, |e, output| {
        let output = output.addr();
        e.mem.set_u32(
            output + OUTPUT_TOI_TIME,
            x87_quiet_bits(e.mem.u32(TOI_TIME_NOT_SET)),
        );
        e.mem.set_u32(output + OUTPUT_PROPERTIES, 0);
        e.mem.set_u32(output + OUTPUT_PROPERTIES + 4, 0);
        e.mem.set_u32(output + 4, island);
        profile_record(e, PROFILE_TT_NARROW_PHASE_TOI);
        let local = e.mem.alloc(12);
        let mut i = 0;
        'bodies: while i < count {
            let body = e.mem.u32(entities + 4 * i as u32);
            let storage_index = e.mem.u16(body + ENTITY_STORAGE_INDEX) as u32;
            e.mem.set_u8(flags + storage_index, 1);
            e.mem.set_u32(local, 0);
            e.mem.set_u32(local + 4, 0);
            e.mem.set_u32(local + 8, 0x8000_0000);
            e.call(
                GET_COLLISION_ENTRIES_SORTED,
                &args![body + ENTITY_COLLIDABLE, local],
            );
            let mut j = 0;
            while j < e.mem.i32(local + 4) {
                let pair = e.mem.u32(local) + 8 * j as u32;
                let other_collidable = e.mem.u32(pair + 4);
                let other = other_collidable.wrapping_add(
                    e.mem.i8(other_collidable + COLLIDABLE_OWNER_OFFSET) as i32 as u32,
                );
                let skip = e.mem.u32(other + ENTITY_SIMULATION_ISLAND) == island
                    && e.mem
                        .u8(flags + e.mem.u16(other + ENTITY_STORAGE_INDEX) as u32)
                        != 0;
                if !skip {
                    let entry = e.mem.u32(pair);
                    let world = e.get(this, HkpContinuousSimulation::world).addr();
                    let dispatcher = e.mem.u32(world + WORLD_COLLISION_DISPATCHER);
                    let quality = (e.mem.i8(entry + ENTRY_QUALITY_INDEX) as i32) << 6;
                    let quality_base = dispatcher.wrapping_add(quality as u32);
                    if e.mem.u32(quality_base + 0x1c30) != 0 && e.mem.u8(quality_base + 0x1c34) == 0
                    {
                        e.mem.set_u32(
                            output + OUTPUT_TOI_TIME,
                            x87_quiet_bits(e.mem.u32(TOI_TIME_NOT_SET)),
                        );
                        e.mem.set_u32(output, output + 0x10);
                        e.mem.set_u32(output + OUTPUT_EXTRA, 0);
                        let input_dispatcher = e.mem.u32(input);
                        let quality_info = input_dispatcher
                            .wrapping_add(
                                ((e.mem.i8(entry + ENTRY_QUALITY_INDEX) as i32) << 6) as u32,
                            )
                            .wrapping_add(0x1c20);
                        e.mem.set_u32(input + 0x60, quality_info);
                        let creation = e.mem.u32(quality_info + 0x10);
                        e.mem.set_u32(input + 0x14, creation);
                        let manager = e.mem.u32(entry + ENTRY_CONTACT_MANAGER);
                        e.call(PROCESS_AGENT, &args![entry, input, output, manager]);
                        if e.global::<u32>(ABORT_FLAG) == 1 {
                            release_array(e, local, 8);
                            break 'bodies;
                        }
                        if e.mem.u32(output) != output + 0x10 {
                            let manager = e.mem.u32(entry + ENTRY_CONTACT_MANAGER);
                            let collidable_a = e.mem.u32(entry + ENTRY_COLLIDABLE_A);
                            let collidable_b = e.mem.u32(entry + ENTRY_COLLIDABLE_B);
                            e.vcall(
                                manager,
                                0x18,
                                &args![collidable_a, collidable_b, input, output],
                            );
                        }
                        if time_was_set(e, output) {
                            hkp_continuous_simulation_add_toi_event(e, this, output, entry);
                        }
                    } else {
                        let uid = e.mem.u32(body + ENTITY_UID);
                        let known = e.call(MAP_GET_WITH_DEFAULT, &args![map, uid, 0u32]).u32();
                        if known == 0 {
                            e.call(ADD_REFERENCE, &args![body]);
                            let uid = e.mem.u32(body + ENTITY_UID);
                            let allocator = heap_allocator(e);
                            e.call(MAP_INSERT, &args![map, allocator, uid, body]);
                        }
                    }
                }
                j += 1;
            }
            release_array(e, local, 8);
            i += 1;
        }
        e.mem.free(local);
    });
    profile_record(e, PROFILE_END);
    stack_release(e, flags);
    if (flags_capacity as i32) >= 0 {
        heap_free(e, flags, flags_capacity & 0x3fff_ffff);
    }
}

// Translated from 00d0de70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::collideEntitiesNeedingPsiCollisionDetectionNarrowPhase_toiOnly`
/// (Xbox PDB): runs the PSI-quality narrow phase for the bodies that
/// `...OneIslandNarrowPhaseContinuous_toiOnly` left in `map` (body uid to
/// body).
///
/// Between the profiling records `TtRecollide PSI` and `Et`, with the world's
/// critical-operation lock held. Every body of the map (empty slots hold the
/// key -1) contributes its collision entries (`00d081d0`, pairs of entry and
/// collidable) that are not yet in a local visited map (`00cb1930`) and whose
/// dispatcher quality block (`0x1c20 + 0x40 * quality`) has a zero creation
/// word at `+0x10`, or a non-zero byte at `+0x14` together with a body whose
/// float at `+0x14c` is exactly 0; each such entry is remembered in the
/// visited map and in a local array of (entry, island) pairs, the island being
/// the body's (or, for a body of motion type 5, that of the entry's other
/// body). The body's reference is released (`00c905b0`) and the map cleared
/// (`00cb1a60`). Then each remembered entry is processed (`00cfedd0` with the
/// island in the output block's owner word, and the manager's virtual slot
/// `0x18` for a contact; no event is queued) unless the abort flag at
/// `01269200` is set. The lock is released as in the world's unlock, and the
/// local array and map are freed. The compiler's stack probe (`00ec8660`) is
/// not modelled.
pub fn hkp_continuous_simulation_collide_entities_needing_psi_collision_detection_narrow_phase_toi_only(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    input: u32,
    map: u32,
) {
    profile_record(e, PROFILE_TT_RECOLLIDE_PSI);
    let world = e.get(this, HkpContinuousSimulation::world).addr();
    world_lock(e, world);
    let visited = e.mem.alloc(12);
    e.mem.set_u32(visited, 0);
    e.mem.set_u32(visited + 4, 0);
    e.mem.set_u32(visited + 8, 0xffff_ffff);
    let allocator = heap_allocator(e);
    e.call(MAP_RESERVE, &args![visited, allocator, 0u32]);
    let pairs = e.mem.alloc(12);
    e.mem.set_u32(pairs, 0);
    e.mem.set_u32(pairs + 4, 0);
    e.mem.set_u32(pairs + 8, 0x8000_0000);
    let mut slot = map_first_used(e, map);
    while slot <= e.mem.i32(map + 8) {
        let body = e.mem.u32(e.mem.u32(map) + 8 * slot as u32 + 4);
        let entries = e
            .call(
                ENTRIES_DETERMINISTIC_UNCHECKED,
                &args![body + ENTITY_COLLIDABLE],
            )
            .u32();
        let entry_count = e.mem.i32(entries + 4);
        let mut j = 0;
        while j < entry_count {
            let entry = e.mem.u32(e.mem.u32(entries) + 8 * j as u32);
            let seen = e
                .call(MAP_GET_WITH_DEFAULT, &args![visited, entry, 0u32])
                .u32();
            if seen == 0 {
                let quality = (e.mem.i8(entry + ENTRY_QUALITY_INDEX) as i32) << 6;
                let dispatcher = e.mem.u32(world + WORLD_COLLISION_DISPATCHER);
                let block = dispatcher.wrapping_add(quality as u32).wrapping_add(0x1c20);
                let creation = e.mem.u32(block + 0x10);
                let byte = e.mem.u8(block + 0x14);
                let at_rest = 0.0f32 == e.mem.f32(body + ENTITY_INV_DELTA_TIME);
                if creation == 0 || (byte != 0 && at_rest) {
                    let mut island = e.mem.u32(body + ENTITY_SIMULATION_ISLAND);
                    if e.mem.u8(body + ENTITY_MOTION_TYPE) == 5 {
                        let collidable_b = e.mem.u32(entry + ENTRY_COLLIDABLE_B);
                        let body_b = collidable_b.wrapping_add(
                            e.mem.i8(collidable_b + COLLIDABLE_OWNER_OFFSET) as i32 as u32,
                        );
                        let collidable_a = e.mem.u32(entry + ENTRY_COLLIDABLE_A);
                        let body_a = collidable_a.wrapping_add(
                            e.mem.i8(collidable_a + COLLIDABLE_OWNER_OFFSET) as i32 as u32,
                        );
                        let other = body_b ^ body_a ^ body;
                        island = e.mem.u32(other + ENTITY_SIMULATION_ISLAND);
                    }
                    let allocator = heap_allocator(e);
                    e.call(MAP_INSERT, &args![visited, allocator, entry, 0u32]);
                    push_pair_grow(e, pairs, entry, island);
                }
            }
            j += 1;
        }
        e.call(REMOVE_REFERENCE, &args![body]);
        slot = map_next_used(e, map, slot);
    }
    e.call(MAP_CLEAR, &args![map]);
    let pair_count = e.mem.i32(pairs + 4);
    if pair_count > 0 {
        e.with_stack(0x30c8, |e, output| {
            let output = output.addr();
            let mut k = 0;
            while k < pair_count {
                let pair = e.mem.u32(pairs) + 8 * k as u32;
                let entry = e.mem.u32(pair);
                let island = e.mem.u32(pair + 4);
                e.mem.set_u32(output + OUTPUT_PROPERTIES, 0);
                e.mem.set_u32(output + OUTPUT_PROPERTIES + 4, 0);
                e.mem.set_u32(output + 4, island);
                e.mem.set_u32(
                    output + OUTPUT_TOI_TIME,
                    x87_quiet_bits(e.mem.u32(TOI_TIME_NOT_SET)),
                );
                e.mem.set_u32(output, output + 0x10);
                e.mem.set_u32(output + OUTPUT_EXTRA, 0);
                let dispatcher = e.mem.u32(input);
                let quality_info = dispatcher
                    .wrapping_add(((e.mem.i8(entry + ENTRY_QUALITY_INDEX) as i32) << 6) as u32)
                    .wrapping_add(0x1c20);
                e.mem.set_u32(input + 0x60, quality_info);
                let creation = e.mem.u32(quality_info + 0x10);
                e.mem.set_u32(input + 0x14, creation);
                let manager = e.mem.u32(entry + ENTRY_CONTACT_MANAGER);
                e.call(PROCESS_AGENT, &args![entry, input, output, manager]);
                if e.global::<u32>(ABORT_FLAG) == 1 {
                    return;
                }
                if e.mem.u32(output) != output + 0x10 {
                    let manager = e.mem.u32(entry + ENTRY_CONTACT_MANAGER);
                    let collidable_a = e.mem.u32(entry + ENTRY_COLLIDABLE_A);
                    let collidable_b = e.mem.u32(entry + ENTRY_COLLIDABLE_B);
                    e.vcall(
                        manager,
                        0x18,
                        &args![collidable_a, collidable_b, input, output],
                    );
                }
                k += 1;
            }
        });
    }
    let world = e.get(this, HkpContinuousSimulation::world).addr();
    world_unlock(e, world);
    profile_record(e, PROFILE_END);
    let capacity = e.mem.u32(pairs + 8);
    e.mem.set_u32(pairs + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(pairs);
        heap_free(e, data, (capacity & 0x3fff_ffff).wrapping_mul(8));
    }
    e.mem.free(pairs);
    let allocator = heap_allocator(e);
    e.call(MAP_CLEAR_AND_DEALLOCATE, &args![visited, allocator]);
    e.call(MAP_DESTRUCTOR, &args![visited]);
    e.mem.free(visited);
}

// Translated from 00d0e210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::handleSimpleToi` (Xbox PDB): handles a
/// time-of-impact `event` that needs no solve.
///
/// Between the profiling records `TtSimpleTOI` and `Et`, with the world's
/// critical-operation lock held. The event's contact manager is asked
/// (virtual slot `0x48`: the event, `time`, a two-element local body array)
/// what to do with it. Then for each of the event's two bodies (`+0xc` and
/// `+0x10`) whose quality type byte (`+0x2a`) is 3 the body's motion state is
/// frozen at the event time (`00cf1d50`), its time-of-impact data
/// invalidated (`00cc0ea0` with `world[0x78]`), every queued event that
/// involves it is reported to its contact manager and removed (as in
/// `00d0c3e0`), and the body goes into a second two-element local array. The
/// event's contact manager is told about the event (virtual slot `0x20`, the
/// island of the first body or, when that has motion type 5, of the second,
/// and the properties at `+0x18`); if the second array holds bodies this
/// simulation's virtual slot `0x3c` runs the narrow phase for them (bodies,
/// count, `world[0x78]`, the map at `this + 0x3c`). The local arrays are freed
/// when they outgrew their storage and the lock is released. The third
/// argument is not read.
pub fn hkp_continuous_simulation_handle_simple_toi(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    world: u32,
    event: u32,
    _unused_1: u32,
    time: f32,
) {
    profile_record(e, PROFILE_TT_SIMPLE_TOI);
    let simulation_world = e.get(this, HkpContinuousSimulation::world).addr();
    world_lock(e, simulation_world);
    e.with_stack(0x30, |e, block| {
        let first = block.addr();
        let second = first + 0xc;
        e.mem.set_u32(first, first + 0x18);
        e.mem.set_u32(first + 4, 0);
        e.mem.set_u32(first + 8, 0x8000_0002);
        let manager = e.mem.u32(event + EVENT_CONTACT_MANAGER);
        e.vcall(manager, 0x48, &args![event, x87_quiet(time), first]);
        e.mem.set_u32(second, first + 0x20);
        e.mem.set_u32(second + 4, 0);
        e.mem.set_u32(second + 8, 0x8000_0002);
        for slot in [EVENT_ENTITY_A, EVENT_ENTITY_B] {
            let body = e.mem.u32(event + slot);
            if e.mem.u8(body + ENTITY_QUALITY_TYPE) == 3 {
                let event_time = x87_quiet(e.mem.f32(event + EVENT_TIME));
                e.call(
                    FREEZE_MOTION_STATE,
                    &args![event_time, body + ENTITY_MOTION_STATE],
                );
                let collision_input = e.mem.u32(world + WORLD_COLLISION_INPUT);
                e.call(INVALIDATE_TIM, &args![body, collision_input]);
                let mut i = e.get(this, HkpContinuousSimulation::event_count) - 1;
                while i >= 0 {
                    let queued = e.get(this, HkpContinuousSimulation::event_data)
                        + i as u32 * TOI_EVENT_SIZE;
                    if e.mem.u32(queued + EVENT_ENTITY_A) == body
                        || e.mem.u32(queued + EVENT_ENTITY_B) == body
                    {
                        notify_event_island(e, queued);
                        remove_event_at(e, this, i);
                    }
                    i -= 1;
                }
                push_word(e, second, body);
            }
        }
        notify_event_island(e, event);
        let moved = e.mem.u32(second + 4);
        if moved != 0 {
            let collision_input = e.mem.u32(world + WORLD_COLLISION_INPUT);
            let this_address = this.addr();
            let bodies = e.mem.u32(second);
            e.vcall(
                this_address,
                0x3c,
                &args![
                    bodies,
                    moved,
                    collision_input,
                    this_address + SIM_ENTITIES_MAP
                ],
            );
        }
        for array in [second, first] {
            let capacity = e.mem.u32(array + 8);
            e.mem.set_u32(array + 4, 0);
            if (capacity as i32) >= 0 {
                let data = e.mem.u32(array);
                heap_free(e, data, (capacity & 0x3fff_ffff).wrapping_mul(4));
            }
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
        }
    });
    let simulation_world = e.get(this, HkpContinuousSimulation::world).addr();
    world_unlock(e, simulation_world);
    profile_record(e, PROFILE_END);
}

// Translated from 00d0e4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLs_toiActivateEntitiesAndCheckConstraints` (Xbox PDB): after a
/// time-of-impact solve, wakes the bodies the solve touched, lets the agents
/// of the island run the narrow phase for them and collects the constraints
/// and bodies that must take part in the next solve.
///
/// `entities` (an `hkArray` of bodies) gets, for every body whose state byte in
/// the table `*flags` is 0, the state 1 and its swept transform interpolated
/// to `time` (`00cf1510`); then the state 8, and its agent entries are
/// collected (`00d0d2c0` with `min_priority`) into a local array whose first
/// 0x1000 bytes come from the stack allocator. The collision input's step
/// info (`input + 0x50`: start, end, delta, inverse delta) is set to the end
/// `time` with the delta and its inverse (0 unless the delta is positive),
/// the island's simulation (`entities[0][8][8]`) runs the narrow phase on the
/// collected entries through its virtual slot `0x48` (entries, count, input,
/// the island of `entities[0]` and a word made of the simulation's address
/// with its low byte set to 1), and the step info and the stack allocator are
/// put back.
///
/// Then each body gets state `0x10`, and each of its constraints (the master
/// constraints from the array at `+0xac`, the slave constraints from `+0xb4`)
/// whose priority byte (`+0x1c`) is at least `min_priority` is appended to
/// `constraints` unless the other body is at state `0x10`; the other body
/// (the one of the constraint's internal that is not this body) with motion
/// type 5 stops there, any other with state below 2 is appended to
/// `new_entities`, interpolated as above when its state is 0, and gets
/// state 2. The constraints and bodies added for the slave constraints are
/// sorted (`00dd7d00`) by `0d0bc80` (constraints) and `00d0bc60` (bodies).
pub fn hk_ls_toi_activate_entities_and_check_constraints(
    e: &mut Engine,
    input: u32,
    min_priority: i32,
    time: f32,
    entities: u32,
    flags: u32,
    new_entities: u32,
    constraints: u32,
) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let saved_top = e.mem.u32(router);
    e.mem.set_u32(router, saved_top + 0x1000);
    // The exe's frame (`AND ESP,0xfffffff0`, `SUB ESP,0x44`, three pushes): 0x50 bytes from a
    // 16-byte boundary, the collected entries at +0x30 (00d0e4c6).
    e.with_stack(0x50 + 0x10, |e, raw| {
        let collected = ((raw.addr() + 0xf) & !0xf) + 0x30;
        e.mem.set_u32(collected, saved_top);
        e.mem.set_u32(collected + 4, 0);
        e.mem.set_u32(collected + 8, 0x8000_03e8);
        let mut i = 0;
        while i < e.mem.i32(entities + 4) {
            let body = e.mem.u32(e.mem.u32(entities) + 4 * i as u32);
            let slot = e.mem.u32(flags) + e.mem.u16(body + ENTITY_STORAGE_INDEX) as u32;
            if e.mem.u8(slot) == 0 {
                e.mem.set_u8(slot, 1);
                e.call(
                    INTERPOLATE_MOTION_STATE,
                    &args![
                        body + ENTITY_SWEPT_TRANSFORM,
                        x87_quiet(time),
                        body + ENTITY_MOTION_STATE
                    ],
                );
            }
            let slot = e.mem.u32(flags) + e.mem.u16(body + ENTITY_STORAGE_INDEX) as u32;
            e.mem.set_u8(slot, 8);
            hk_ls_collect_agent_entries_to_process(e, body, min_priority, time, flags, collected);
            i += 1;
        }
        let old_start = e.mem.u32(input + 0x50);
        let old_end = e.mem.u32(input + 0x54);
        let old_delta = e.mem.u32(input + 0x58);
        let old_inverse = e.mem.u32(input + 0x5c);
        e.mem.set_u32(input + 0x54, x87_quiet_bits(time.to_bits()));
        let delta = (time as f64 - e.mem.f32(input + 0x50) as f64) as f32;
        e.mem.set_u32(input + 0x58, x87_quiet_bits(delta.to_bits()));
        let inverse = if 0.0 < delta {
            (1.0 / delta as f64) as f32
        } else {
            0.0
        };
        e.mem.set_f32(input + 0x5c, inverse);
        let first_body = e.mem.u32(e.mem.u32(entities));
        let simulation = e
            .mem
            .u32(e.mem.u32(first_body + ENTITY_WORLD) + WORLD_SIMULATION);
        let island = e.mem.u32(first_body + ENTITY_SIMULATION_ISLAND);
        let flagged_simulation = (simulation & 0xffff_ff00) | 1;
        let collected_data = e.mem.u32(collected);
        let collected_size = e.mem.u32(collected + 4);
        e.vcall(
            simulation,
            0x48,
            &args![
                collected_data,
                collected_size,
                input,
                island,
                flagged_simulation
            ],
        );
        e.mem.set_u32(input + 0x50, old_start);
        e.mem.set_u32(input + 0x54, old_end);
        e.mem.set_u32(input + 0x58, old_delta);
        e.mem.set_u32(input + 0x5c, old_inverse);
        if saved_top == e.mem.u32(collected) {
            e.mem.set_u32(collected + 4, 0);
        }
        stack_release(e, saved_top);
        let capacity = e.mem.u32(collected + 8);
        e.mem.set_u32(collected + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(collected);
            heap_free(e, data, (capacity & 0x3fff_ffff).wrapping_mul(4));
        }
    });
    let mut k = 0;
    while k < e.mem.i32(entities + 4) {
        let body = e.mem.u32(e.mem.u32(entities) + 4 * k as u32);
        let slot = e.mem.u32(flags) + e.mem.u16(body + ENTITY_STORAGE_INDEX) as u32;
        e.mem.set_u8(slot, 0x10);
        let master_count = e.mem.u16(body + ENTITY_MASTER_CONSTRAINT_COUNT) as u32;
        for j in 0..master_count {
            let internal = e.mem.u32(body + ENTITY_MASTER_CONSTRAINTS) + 0x30 * j;
            toi_visit_constraint(
                e,
                internal,
                body,
                min_priority,
                time,
                flags,
                new_entities,
                constraints,
            );
        }
        let constraints_before = e.mem.i32(constraints + 4);
        let entities_before = e.mem.i32(new_entities + 4);
        let slave_count = e.mem.i32(body + ENTITY_SLAVE_CONSTRAINT_COUNT);
        for m in 0..slave_count {
            let instance = e
                .mem
                .u32(e.mem.u32(body + ENTITY_SLAVE_CONSTRAINTS) + 4 * m as u32);
            let internal = e.mem.u32(instance + INSTANCE_INTERNAL);
            toi_visit_constraint(
                e,
                internal,
                body,
                min_priority,
                time,
                flags,
                new_entities,
                constraints,
            );
        }
        let added = e.mem.i32(constraints + 4) - constraints_before;
        if added > 1 {
            let start = e.mem.u32(constraints) + 4 * constraints_before as u32;
            e.call(
                QUICK_SORT_RECURSIVE,
                &args![start, 0u32, added - 1, COMPARE_CONSTRAINTS],
            );
        }
        let added = e.mem.i32(new_entities + 4) - entities_before;
        if added > 1 {
            let start = e.mem.u32(new_entities) + 4 * entities_before as u32;
            e.call(
                QUICK_SORT_RECURSIVE,
                &args![start, 0u32, added - 1, COMPARE_ENTITIES],
            );
        }
        k += 1;
    }
}

// Translated from 00d0e9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::handleAllToisTill` (Xbox PDB): handles the
/// queued time-of-impact events in time order, returns 0 when no event is
/// left to handle or 3 when the abort flag is set.
///
/// Each pass takes the world's critical-operation lock and looks for the
/// queued event with the earliest time that is below `time`; events with an
/// equal time are ordered by the unique ids of their bodies (the first body's
/// id, then the second's, both unsigned, smaller first). Nothing found:
/// the lock is released and 0 returned. The chosen event is shown to its
/// contact manager through `fn_00d0bc20` (with the world's
/// `m_toiCollisionResponseRotateNormal` at `+0xdc`); when that leaves the
/// byte 0 the event is dropped (the last event takes its place) and the
/// search starts again. Otherwise a copy of the event is taken and removed,
/// `this + 0x14` becomes its time, and the step info (start = event time,
/// end = `this + 0x18`, their difference and its inverse, 0 for a zero
/// difference) is written to the world's step info (`+0x1d0`) and the
/// collision input's (`+0x50`). The event's quality byte (`+8`) then picks
/// the handler: non-zero runs `handleSimpleToi` (`00d0e210`), zero this
/// simulation's virtual `simulateToi` (slot `0x44`); both get the world, the
/// copy, `this + 0x1c` and the rotate-normal value. After counting the event
/// in `this + 0x4c` the lock is released; with the abort flag set 3 is
/// returned, otherwise the pass starts again.
pub fn hkp_continuous_simulation_handle_all_tois_till(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    time: f32,
) -> u32 {
    // +0 result byte, +4 rotate-normal value, +0x10 copy of the event.
    e.with_stack(0x90, |e, scratch| {
        let result_byte = scratch.addr();
        let rotate_normal = scratch.addr() + 4;
        let copy = scratch.addr() + 0x10;
        loop {
            let locked_world = e.get(this, HkpContinuousSimulation::world).addr();
            world_lock(e, locked_world);
            loop {
                let mut best_time = x87_quiet(time);
                let world = e.get(this, HkpContinuousSimulation::world).addr();
                let rotate = x87_quiet(e.mem.f32(world + WORLD_ROTATE_NORMAL));
                e.mem.set_f32(rotate_normal, rotate);
                let mut best: i32 = -1;
                let events = e.get(this, HkpContinuousSimulation::event_data);
                let mut best_event = 0u32;
                let count = e.get(this, HkpContinuousSimulation::event_count);
                for i in 0..count {
                    let event = events + i as u32 * TOI_EVENT_SIZE;
                    let event_time = e.mem.f32(event + EVENT_TIME);
                    if best_time > event_time {
                        best = i;
                        best_time = x87_quiet(event_time);
                        best_event = event;
                    } else if best_time == event_time && best != -1 {
                        let first = e.mem.u32(e.mem.u32(event + EVENT_ENTITY_A) + ENTITY_UID);
                        let second = e.mem.u32(e.mem.u32(event + EVENT_ENTITY_B) + ENTITY_UID);
                        let best_first = e
                            .mem
                            .u32(e.mem.u32(best_event + EVENT_ENTITY_A) + ENTITY_UID);
                        let best_second = e
                            .mem
                            .u32(e.mem.u32(best_event + EVENT_ENTITY_B) + ENTITY_UID);
                        if first < best_first || (first == best_first && second < best_second) {
                            best = i;
                            best_time = x87_quiet(event_time);
                            best_event = event;
                        }
                    }
                }
                if best < 0 {
                    world_unlock(e, locked_world);
                    return 0;
                }
                fn_00d0bc20(e, result_byte, best_event, rotate_normal);
                if e.mem.u8(result_byte) == 0 {
                    remove_event_at(e, this, best);
                    continue;
                }
                let bytes = e.mem.bytes(best_event, TOI_EVENT_SIZE);
                e.mem.write(copy, &bytes);
                remove_event_at(e, this, best);
                let event_time = x87_quiet(e.mem.f32(copy + EVENT_TIME));
                e.set(this, HkpContinuousSimulation::float_14, event_time);
                let end = x87_quiet(e.get(this, HkpContinuousSimulation::float_18));
                let delta = (end as f64 - event_time as f64) as f32;
                let inverse = if delta == 0.0 {
                    0.0
                } else {
                    (1.0 / delta as f64) as f32
                };
                let step = [
                    event_time.to_bits(),
                    end.to_bits(),
                    delta.to_bits(),
                    inverse.to_bits(),
                ];
                let world = e.get(this, HkpContinuousSimulation::world).addr();
                let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
                for (k, word) in step.iter().enumerate() {
                    e.mem.set_u32(world + WORLD_STEP_INFO + 4 * k as u32, *word);
                    e.mem
                        .set_u32(input + COLLISION_INPUT_STEP_INFO + 4 * k as u32, *word);
                }
                let rotate = e.mem.f32(rotate_normal);
                let this_value = x87_quiet(e.get(this, HkpContinuousSimulation::float_1c));
                let world = e.get(this, HkpContinuousSimulation::world).addr();
                if e.mem.u8(copy + EVENT_QUALITY_BYTE) != 0 {
                    hkp_continuous_simulation_handle_simple_toi(
                        e,
                        this,
                        world,
                        copy,
                        this_value.to_bits(),
                        rotate,
                    );
                } else {
                    e.vcall(this.addr(), 0x44, &args![world, copy, this_value, rotate]);
                }
                let processed = e.get(this, HkpContinuousSimulation::processed_toi_count);
                e.set(
                    this,
                    HkpContinuousSimulation::processed_toi_count,
                    processed.wrapping_add(1),
                );
                if e.global::<u32>(ABORT_FLAG) == 1 {
                    world_unlock(e, locked_world);
                    return 3;
                }
                world_unlock(e, locked_world);
                break;
            }
        }
    })
}

// Translated from 00d0ed10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::advanceTimeInternal` (Xbox PDB): between the
/// profiling records `TtPhysics` and `Et`, first repeats an interrupted step
/// and then handles the time-of-impact events.
///
/// When `this + 0x28` is non-zero (a step was interrupted) the base
/// simulation's `reCollideAfterStepFailure` (`00cf8820`, record
/// `TtRecollideAfterStepFailure`) runs and a non-zero result is returned at
/// once. Then the base simulation's `snapSimulateTimeAndGetTimeToAdvanceTo`
/// (`00cf89c0`, result in ST0, stored as a `float`) gives the target time.
/// With queued events (`this + 0x34`) `handleAllToisTill` runs for it (record
/// `TtTois`); a non-zero result is stored in `this + 0x28` and returned. Else
/// `this + 0x28` is cleared, `this + 0x14` becomes the target time and 0 is
/// returned.
pub fn hkp_continuous_simulation_advance_time_internal(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
) -> u32 {
    profile_record(e, PROFILE_TT_PHYSICS);
    if e.get(this, HkpContinuousSimulation::word_28) != 0 {
        profile_record(e, PROFILE_TT_RECOLLIDE_AFTER_STEP_FAILURE);
        let result = e.call(RECOLLIDE_AFTER_STEP_FAILURE, &args![this]).u32();
        if result != 0 {
            profile_record(e, PROFILE_END);
            profile_record(e, PROFILE_END);
            return result;
        }
        profile_record(e, PROFILE_END);
    }
    let target = x87_quiet(e.call(SNAP_SIMULATE_TIME, &args![this]).f32());
    if e.get(this, HkpContinuousSimulation::event_count) != 0 {
        profile_record(e, PROFILE_TT_TOIS);
        let result = hkp_continuous_simulation_handle_all_tois_till(e, this, target);
        if result != 0 {
            e.set(this, HkpContinuousSimulation::word_28, result);
            profile_record(e, PROFILE_END);
            profile_record(e, PROFILE_END);
            return result;
        }
        profile_record(e, PROFILE_END);
    }
    e.set(this, HkpContinuousSimulation::word_28, 0);
    e.set(this, HkpContinuousSimulation::float_14, target);
    profile_record(e, PROFILE_END);
    0
}

// Translated from 00d0ef40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::collideInternal` (Xbox PDB): the collision
/// phase of a step, between the profiling records `TtCollide` and `Et`.
/// `step_info` points at the four step info words (start, end, delta,
/// inverse delta).
///
/// With `this + 0x28` non-zero the queued events are reported to their
/// contact managers (as in `00d0bc20`'s siblings: the event's contact manager
/// gets virtual slot `0x20` with the island of the first body, or of the
/// second when the first has motion type 5, and the properties at `+0x18`),
/// last event first, and the queue is emptied. The step info goes to the
/// world (`+0x1d0`) and the collision input (`+0x50`); the world's `+0x2ec`
/// becomes `float(world[0x300]) * step_info[2]` and `+0x2f0` becomes
/// `int(world[0x2f4]) * step_info[3]`. With the world locked, every active
/// island (array at `world + 0x28`) runs the static broad phase
/// `00d0d3f0` on its entities (`+0x48`, count `+0x4c`); the abort flag stops
/// everything (lock released, record `Et`, return). After unlocking and
/// locking again every island runs `collideIslandNarrowPhaseContinuous` with
/// the collision input; the abort flag stops as before, and with island
/// post-collide listeners (`world[0x198]`) each island is shown to
/// `fireIslandPostCollideCallback` (`00d01650`: world, island, step info)
/// between the records `TtIslandPostCollideCb` and `Et`.
pub fn hkp_continuous_simulation_collide_internal(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    step_info: u32,
) {
    profile_record(e, PROFILE_TT_COLLIDE);
    if e.get(this, HkpContinuousSimulation::word_28) != 0 {
        let mut i = e.get(this, HkpContinuousSimulation::event_count) - 1;
        while i >= 0 {
            let event =
                e.get(this, HkpContinuousSimulation::event_data) + i as u32 * TOI_EVENT_SIZE;
            notify_event_island(e, event);
            i -= 1;
        }
        e.set(this, HkpContinuousSimulation::event_count, 0);
    }
    let world = e.get(this, HkpContinuousSimulation::world).addr();
    for k in 0..4 {
        let word = e.mem.u32(step_info + 4 * k);
        e.mem.set_u32(world + WORLD_STEP_INFO + 4 * k, word);
    }
    let input = e
        .mem
        .u32(e.get(this, HkpContinuousSimulation::world).addr() + WORLD_COLLISION_INPUT);
    for k in 0..4 {
        let word = e.mem.u32(step_info + 4 * k);
        e.mem
            .set_u32(input + COLLISION_INPUT_STEP_INFO + 4 * k, word);
    }
    let world = e.get(this, HkpContinuousSimulation::world).addr();
    let scaled = e.mem.f32(world + WORLD_VALUE_300) as f64 * e.mem.f32(step_info + 8) as f64;
    e.mem.set_f32(world + WORLD_SCALED_2EC, scaled as f32);
    let scaled = e.mem.i32(world + WORLD_INTEGER_2F4) as f64 * e.mem.f32(step_info + 12) as f64;
    e.mem.set_f32(world + WORLD_SCALED_2F0, scaled as f32);
    let world = e.get(this, HkpContinuousSimulation::world).addr();
    world_lock(e, world);
    let islands = e.get(this, HkpContinuousSimulation::world).addr() + WORLD_ACTIVE_ISLANDS;
    let mut i = 0;
    while e.mem.i32(islands + 4) > i {
        let island = e.mem.u32(e.mem.u32(islands) + 4 * i as u32);
        let world = e.get(this, HkpContinuousSimulation::world).addr();
        let entities = e.mem.u32(island + ISLAND_ENTITY_DATA);
        let count = e.mem.i32(island + ISLAND_ENTITY_COUNT);
        hkp_continuous_simulation_collide_entities_broad_phase_continuous(
            e, entities, count, world, 0,
        );
        if e.global::<u32>(ABORT_FLAG) == 1 {
            let world = e.get(this, HkpContinuousSimulation::world).addr();
            world_unlock(e, world);
            profile_record(e, PROFILE_END);
            return;
        }
        i += 1;
    }
    let world = e.get(this, HkpContinuousSimulation::world).addr();
    world_unlock(e, world);
    let world = e.get(this, HkpContinuousSimulation::world).addr();
    world_lock(e, world);
    let islands = e.get(this, HkpContinuousSimulation::world).addr() + WORLD_ACTIVE_ISLANDS;
    let mut i = 0;
    while e.mem.i32(islands + 4) > i {
        let island = e.mem.u32(e.mem.u32(islands) + 4 * i as u32);
        let world = e.get(this, HkpContinuousSimulation::world).addr();
        let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
        hkp_continuous_simulation_collide_island_narrow_phase_continuous(e, this, island, input);
        if e.global::<u32>(ABORT_FLAG) == 1 {
            let world = e.get(this, HkpContinuousSimulation::world).addr();
            world_unlock(e, world);
            profile_record(e, PROFILE_END);
            return;
        }
        let world = e.get(this, HkpContinuousSimulation::world).addr();
        if e.mem.u32(world + WORLD_ISLAND_POST_COLLIDE_LISTENER_COUNT) != 0 {
            profile_record(e, PROFILE_TT_ISLAND_POST_COLLIDE_CB);
            let world = e.get(this, HkpContinuousSimulation::world).addr();
            e.call(
                FIRE_ISLAND_POST_COLLIDE_CALLBACK,
                &args![world, island, step_info],
            );
            profile_record(e, PROFILE_END);
        }
        i += 1;
    }
    let world = e.get(this, HkpContinuousSimulation::world).addr();
    world_unlock(e, world);
    profile_record(e, PROFILE_END);
}

// Translated from 00d0f2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the engine map. Hands its three arguments to the base
/// simulation's function `00cf9480` together with `fn_00d0daa0` (this unit's
/// callback at `00d0daa0`) and a zero. `ECX` is forwarded to `00cf9480`
/// untouched, so `this` is the first argument.
pub fn fn_00d0f2e0(e: &mut Engine, this: u32, first: u32, second: u32, third: u32) {
    e.call(
        BASE_SIMULATION_CALLBACK_FUNCTION,
        &args![this, first, second, third, CALLBACK_00D0DAA0, 0u32],
    );
}
// Translated from 00d0f300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkLs_localizedSolveToi` (Xbox PDB): the solve for one time-of-impact
/// event, run on the part of the island the event reaches. Returns 1 when
/// the manager asked for nothing to be solved (the event's bodies only get
/// their motion state updated) and 0 otherwise.
///
/// Arguments: `limits` holds the counts the solve works with (`+0` minimum
/// constraint priority, `+4`, `+8` and `+0xc` limits on the sizes of what is
/// added to the solver, `+0x10` the number of rounds, `+0x14` the number of
/// final validity checks); `solver` is the constraint solver set-up block
/// (words at `+0`, `+4`, `+0xc`, `+0x30`, `+0x38` and `+0x40` are used);
/// `event` is the time-of-impact event (time at `+0`, bodies at `+0xc` and
/// `+0x10`, contact manager at `+0x14`); `manager` is the resource manager;
/// `world` the world; `bodies` an `hkArray` of bodies that receives the
/// solved set; `flags` points at the table of per-body state bytes (indexed
/// by the `u16` at body `+0xa8`); `time` is handed to the event's contact
/// manager.
///
/// The bodies of the event that are not fixed (motion type 5) go into a
/// local body array (sized by the event's island, from the stack allocator)
/// with their swept transform interpolated to the event time (`00cf1510`)
/// and state 2. The event's contact manager (`+0x14`, not the resource
/// manager) is asked through its virtual slot `0x48` (event, `time`, a local
/// body array) what to do. With nothing returned each non-fixed
/// body's motion state gets its rotation set (`00cb2d90`) and its
/// translation (`+0x30`) becomes `ms[0x50] - (ms[0x80].y * ms[0x10] +
/// ms[0x80].x * ms[0] + ms[0x80].z * ms[0x20])` (four `float` lanes), the
/// local arrays are released and 1 is returned.
///
/// Otherwise the solver is set up (`00d892d0`, `00d88670` over the local body
/// array) and up to `limits[0x10]` rounds run. A round (the first one skips
/// the validity check, later ones run `hkLs_toiCheckValidityOfConstraints`)
/// activates the bodies and constraints it reaches
/// (`hkLs_toiActivateEntitiesAndCheckConstraints`) and, when the solver
/// accepts them (`00d88310`, `00d88470` and the three size limits), adds them
/// (`00d88670`, `00d88760`), appends the manager's bodies to `bodies`, drops
/// the bodies that reached state `0x10` from the local array, takes in the
/// new non-keyframed bodies and solves (`00d88340`); it repeats while it
/// found something new. When the solver does not accept them, the manager's
/// bodies are put back to state 8 and the new ones to state 1 and the
/// manager's virtual slot `0x14` is asked; the answer 2 backsteps and
/// freezes the whole island (`hkLs_backstepAndFreezeEntireIsland`),
/// invalidates the time-of-impact data of `bodies`, empties it and returns 0.
/// Other answers end the round (`00d88340` with 0, one round fewer).
///
/// After the rounds, unless `solver[0x30] == solver[0x38]`, the final
/// validity of the critical constraints is checked (after resetting the
/// velocity accumulators) and, when it fails, up to `limits[0x14]` times more
/// with `00d8aac0` in between; when it still fails the manager's virtual slot
/// `0x10` is asked and the answer 2 does the backstep and freeze as above and
/// returns 0. With `bodies` still empty the event's first (and second, if
/// there is one) manager body go into it in state `0x10`. Every body in it
/// is stepped back to the event time (`00cf1b10`), integrated (`00d88580`),
/// checked (`hkLs_restoreTransformOnBodiesWithUpdatedTransform`) and has its
/// time-of-impact data invalidated (`00cc0ea0`). The local arrays are
/// released and 0 returned.
#[allow(clippy::too_many_arguments)]
pub fn hk_ls_localized_solve_toi(
    e: &mut Engine,
    limits: u32,
    solver: u32,
    event: u32,
    manager: u32,
    world: u32,
    bodies: u32,
    flags: u32,
    time: f32,
) -> u32 {
    e.with_stack(0x810, |e, frame| {
        let b = frame.addr();
        // Local arrays (data, size, capacity words; storage behind them).
        let records = b + 0x500;
        let solve_bodies = b + 0x10;
        let manager_bodies = b + 0x40;
        let new_bodies = b + 0x150;
        let constraints = b + 0x260;
        let critical = b + 0x370;
        let counter = b + 0x24;
        let rounds_left = b + 0x28;
        let island_slot = b + 0x2c;
        let saved_top_slot = b + 0x1c;
        let first_slot = b + 0x34;
        let second_slot = b + 0x38;
        let counter_copy = b + 0x3c;
        let flag_byte = b + 0xf;
        let byte_a = b + 0x23;
        let byte_b = b + 0x33;

        e.mem.set_u32(records, b + 0x50c);
        e.mem.set_u32(records + 4, 0);
        e.mem.set_u32(records + 8, 0x8000_0040);
        e.mem.set_u32(counter, 0);
        let first_body = e.mem.u32(event + EVENT_ENTITY_A);
        let island = if e.mem.u8(first_body + ENTITY_MOTION_TYPE) == 5 {
            e.mem
                .u32(e.mem.u32(event + EVENT_ENTITY_B) + ENTITY_SIMULATION_ISLAND)
        } else {
            e.mem.u32(first_body + ENTITY_SIMULATION_ISLAND)
        };
        let island_count = e.mem.u32(island + ISLAND_ENTITY_COUNT);
        e.mem.set_u32(island_slot, island);
        e.mem.set_u32(solve_bodies, 0);
        e.mem.set_u32(solve_bodies + 4, 0);
        e.mem.set_u32(solve_bodies + 8, 0x8000_0000);
        let saved_top = stack_alloc(e, island_count.wrapping_mul(4));
        e.mem.set_u32(solve_bodies + 8, island_count | 0x8000_0000);
        e.mem.set_u32(solve_bodies, saved_top);
        e.mem.set_u32(saved_top_slot, saved_top);

        let event_time = e.mem.f32(event + EVENT_TIME);
        for slot in [EVENT_ENTITY_A, EVENT_ENTITY_B] {
            let body = e.mem.u32(event + slot);
            if e.mem.u8(body + ENTITY_MOTION_TYPE) != 5 {
                let size = e.mem.u32(solve_bodies + 4);
                e.mem.set_u32(e.mem.u32(solve_bodies) + 4 * size, body);
                e.mem.set_u32(solve_bodies + 4, size + 1);
                e.call(
                    INTERPOLATE_MOTION_STATE,
                    &args![
                        body + ENTITY_SWEPT_TRANSFORM,
                        x87_quiet(event_time),
                        body + ENTITY_MOTION_STATE
                    ],
                );
                set_body_state(e, flags, body, 2);
            }
        }

        e.mem.set_u32(manager_bodies, b + 0x4c);
        e.mem.set_u32(manager_bodies + 4, 0);
        e.mem.set_u32(manager_bodies + 8, 0x8000_0040);
        // 00d0f41e-00d0f447: `MOV ECX,[EBX+0x14]` (EBX the event) then `CALL [[ECX]+0x48]`:
        // the event's contact manager is asked, not the resource manager at `[EBP+0x14]`
        // (whose vtable `010d4310` holds 0 at +0x48: the call stopped on "open function
        // 00000000" in 00d100a0 when a time-of-impact had to be solved).
        let contact_manager = e.mem.u32(event + EVENT_CONTACT_MANAGER);
        e.vcall(
            contact_manager,
            0x48,
            &args![event, x87_quiet(time), manager_bodies],
        );

        if e.mem.u32(manager_bodies + 4) == 0 {
            for slot in [EVENT_ENTITY_A, EVENT_ENTITY_B] {
                let body = e.mem.u32(event + slot);
                if e.mem.u8(body + ENTITY_MOTION_TYPE) != 5 {
                    let motion_state = body + ENTITY_MOTION_STATE;
                    e.call(SET_ROTATION, &args![motion_state, motion_state + 0x70]);
                    let vector = read_vec4(e, motion_state + 0x80);
                    let row_1 = read_vec4(e, motion_state + 0x10);
                    let row_0 = read_vec4(e, motion_state);
                    let row_2 = read_vec4(e, motion_state + 0x20);
                    let base = read_vec4(e, motion_state + 0x50);
                    let result: [f32; 4] = std::array::from_fn(|lane| {
                        let sum = vector[1] * row_1[lane] + vector[0] * row_0[lane];
                        base[lane] - (sum + vector[2] * row_2[lane])
                    });
                    write_vec4(e, motion_state + 0x30, result);
                }
            }
            release_solve_arrays(e, manager_bodies, solve_bodies, records, saved_top_slot);
            return 1;
        }

        // The first two bodies the manager returned.
        let manager_data = e.mem.u32(manager_bodies);
        let first = e.mem.u32(manager_data);
        e.mem.set_u32(first_slot, first);
        if e.mem.i32(manager_bodies + 4) > 1 {
            let second = e.mem.u32(manager_data + 4);
            e.mem.set_u32(second_slot, second);
        } else {
            e.mem.set_u32(second_slot, 0);
        }
        e.call(HK_SOLVE_UPLOAD, &args![]);
        let data = e.mem.u32(solve_bodies);
        let size = e.mem.u32(solve_bodies + 4);
        e.call(INTERNAL_ADD_ACCUMULATORS, &args![solver, data, size]);
        e.mem.set_u32(new_bodies, b + 0x15c);
        e.mem.set_u32(new_bodies + 8, 0x8000_0040);
        e.mem.set_u32(constraints + 8, 0x8000_0040);
        e.mem.set_u8(flag_byte, 1);
        e.mem.set_u32(new_bodies + 4, 0);
        e.mem.set_u32(constraints, b + 0x26c);
        e.mem.set_u32(constraints + 4, 0);
        let rounds = e.mem.i32(limits + 0x10);
        e.mem.set_i32(rounds_left, rounds);
        let input = e.mem.u32(world + WORLD_COLLISION_INPUT);

        if rounds > 0 {
            'rounds: loop {
                e.mem.set_u32(constraints + 4, 0);
                e.mem.set_u32(new_bodies + 4, 0);
                let current = e.mem.u32(counter);
                e.mem.set_u32(counter_copy, current);
                'round: {
                    if e.mem.u8(flag_byte) == 0 {
                        if (current as i32) >= e.mem.i32(records + 4) {
                            break 'round;
                        }
                        e.mem.set_u32(manager_bodies + 4, 0);
                        let min_priority = e.mem.i32(limits);
                        hk_ls_toi_check_validity_of_constraints(
                            e,
                            solver,
                            input,
                            records,
                            counter_copy,
                            min_priority,
                            flags,
                            solve_bodies,
                            manager_bodies,
                        );
                        if e.mem.u32(manager_bodies + 4) == 0 {
                            break 'round;
                        }
                    }
                    let event_time = x87_quiet(e.mem.f32(event + EVENT_TIME));
                    let min_priority = e.mem.i32(limits);
                    hk_ls_toi_activate_entities_and_check_constraints(
                        e,
                        input,
                        min_priority,
                        event_time,
                        manager_bodies,
                        flags,
                        new_bodies,
                        constraints,
                    );
                    e.mem.set_u8(flag_byte, 0);
                    let fits = 'fits: {
                        let data = e.mem.u32(new_bodies);
                        let size = e.mem.u32(new_bodies + 4);
                        let r = e
                            .call(SOLVER_SETUP_00D88310, &args![byte_b, solver, data, size])
                            .u32();
                        if e.mem.u8(r) == 0 {
                            break 'fits false;
                        }
                        let data = e.mem.u32(constraints);
                        let size = e.mem.u32(constraints + 4);
                        let r = e
                            .call(SOLVER_SETUP_00D88470, &args![byte_a, solver, data, size])
                            .u32();
                        if e.mem.u8(r) == 0 {
                            break 'fits false;
                        }
                        let record_count = e.mem.i32(records + 4);
                        let constraint_count = e.mem.i32(constraints + 4);
                        if e.mem.i32(limits + 0xc) < constraint_count.wrapping_add(record_count) {
                            break 'fits false;
                        }
                        let result_size = e.mem.i32(bodies + 4);
                        let total = e
                            .mem
                            .i32(new_bodies + 4)
                            .wrapping_add(result_size)
                            .wrapping_add(e.mem.i32(solve_bodies + 4));
                        if e.mem.i32(limits + 4) < total {
                            break 'fits false;
                        }
                        let activated = e.mem.i32(manager_bodies + 4).wrapping_add(result_size);
                        if e.mem.i32(limits + 8) < activated {
                            break 'fits false;
                        }
                        true
                    };
                    if fits {
                        let data = e.mem.u32(new_bodies);
                        let size = e.mem.u32(new_bodies + 4);
                        e.call(INTERNAL_ADD_ACCUMULATORS, &args![solver, data, size]);
                        let data = e.mem.u32(constraints);
                        let size = e.mem.u32(constraints + 4);
                        e.call(
                            INTERNAL_ADD_JACOBIAN_SCHEMAS,
                            &args![solver, data, size, records],
                        );
                        let updated = e.mem.u32(counter_copy);
                        e.mem.set_u32(counter, updated);
                        // Bodies that reached state 0x10 leave the local array.
                        let mut size = e.mem.i32(solve_bodies + 4);
                        let mut i = 0;
                        while i < size {
                            let data = e.mem.u32(solve_bodies);
                            let body = e.mem.u32(data + 4 * i as u32);
                            let slot =
                                e.mem.u32(flags) + e.mem.u16(body + ENTITY_STORAGE_INDEX) as u32;
                            if e.mem.u8(slot) == 0x10 {
                                size -= 1;
                                e.mem.set_i32(solve_bodies + 4, size);
                                if size != i {
                                    let last = e.mem.u32(data + 4 * size as u32);
                                    e.mem.set_u32(data + 4 * i as u32, last);
                                }
                                i -= 1;
                            }
                            i += 1;
                        }
                        // The manager's bodies are appended to the caller's array.
                        let append_at = e.mem.u32(bodies + 4);
                        let allocator = heap_allocator(e);
                        let data = e.mem.u32(manager_bodies);
                        let size = e.mem.u32(manager_bodies + 4);
                        e.call(
                            ARRAY_INSERT_AT,
                            &args![bodies, allocator, append_at, data, size],
                        );
                        let needed = e
                            .mem
                            .i32(new_bodies + 4)
                            .wrapping_add(e.mem.i32(solve_bodies + 4));
                        let router = e.call(MEMORY_ROUTER, &args![]).u32();
                        let allocator = e.mem.u32(router + 0x10);
                        let capacity = (e.mem.u32(solve_bodies + 8) & 0x3fff_ffff) as i32;
                        if capacity < needed {
                            let doubled = capacity.wrapping_add(capacity);
                            let wanted = if needed < doubled { doubled } else { needed };
                            e.call(ARRAY_RESERVE, &args![allocator, solve_bodies, wanted, 4u32]);
                        }
                        let mut k = 0;
                        while e.mem.i32(new_bodies + 4) > k {
                            let body = e.mem.u32(e.mem.u32(new_bodies) + 4 * k as u32);
                            if e.mem.u8(body + ENTITY_MOTION_TYPE) != 4 {
                                let size = e.mem.u32(solve_bodies + 4);
                                e.mem.set_u32(e.mem.u32(solve_bodies) + 4 * size, body);
                                e.mem.set_u32(solve_bodies + 4, size.wrapping_add(1));
                            }
                            k += 1;
                        }
                        let constraint_count = e.mem.i32(constraints + 4);
                        fn_00d0be80(e, records, counter, constraint_count);
                        fn_00d0c6d0(e, records, counter, flags);
                        e.call(SUB_SOLVE, &args![solver, 1u32]);
                        if e.mem.u32(constraints + 4) != 0 || e.mem.u32(new_bodies + 4) != 0 {
                            continue 'rounds;
                        }
                        break 'round;
                    }
                    // The solver does not accept the additions.
                    let mut k = 0;
                    while e.mem.i32(manager_bodies + 4) > k {
                        let body = e.mem.u32(e.mem.u32(manager_bodies) + 4 * k as u32);
                        set_body_state(e, flags, body, 8);
                        k += 1;
                    }
                    let mut k = 0;
                    while e.mem.i32(new_bodies + 4) > k {
                        let body = e.mem.u32(e.mem.u32(new_bodies) + 4 * k as u32);
                        set_body_state(e, flags, body, 1);
                        k += 1;
                    }
                    if e.vcall(manager, 0x14, &args![]).u32() == 2 {
                        let event_time = x87_quiet(e.mem.f32(event + EVENT_TIME));
                        let island = e.mem.u32(island_slot);
                        hk_ls_backstep_and_freeze_entire_island(
                            e, event_time, island, flags, bodies,
                        );
                        invalidate_bodies_and_clear(e, bodies, input);
                        release_array(e, constraints, 4);
                        release_array(e, new_bodies, 4);
                        release_solve_arrays(
                            e,
                            manager_bodies,
                            solve_bodies,
                            records,
                            saved_top_slot,
                        );
                        return 0;
                    }
                }
                // End of a round.
                e.call(SUB_SOLVE, &args![solver, 0u32]);
                let left = e.mem.i32(rounds_left).wrapping_sub(1);
                e.mem.set_i32(rounds_left, left);
                if left <= 0 {
                    break;
                }
            }
        }

        // After the rounds.
        'checked: {
            if e.mem.u32(solver + 0x30) == e.mem.u32(solver + 0x38) {
                break 'checked;
            }
            hk_ls_toi_reset_velocity_accumulators_for_entities(e, solver, solve_bodies);
            hk_ls_toi_check_final_validity_of_critical_constraints(
                e, flag_byte, solver, records, input, 0,
            );
            if e.mem.u8(flag_byte) != 0 {
                break 'checked;
            }
            let mut n = e.mem.i32(limits + 0x14).wrapping_sub(1);
            while n >= 0 {
                let first = e.mem.u32(solver + 4);
                let second = e.mem.u32(solver + 0x30);
                let third = e.mem.u32(solver + 0xc);
                let fourth = e.mem.u32(solver + 0x40);
                e.call(SOLVE_STEP_JACOBIANS, &args![first, second, third, fourth]);
                hk_ls_toi_reset_velocity_accumulators_for_entities(e, solver, solve_bodies);
                let r = hk_ls_toi_check_final_validity_of_critical_constraints(
                    e, byte_a, solver, records, input, 0,
                );
                if e.mem.u8(r) != 0 {
                    break 'checked;
                }
                n = n.wrapping_sub(1);
            }
            e.mem.set_u32(critical, b + 0x37c);
            e.mem.set_u32(critical + 4, 0);
            e.mem.set_u32(critical + 8, 0x8000_0020);
            hk_ls_toi_check_final_validity_of_critical_constraints(
                e, byte_a, solver, records, input, critical,
            );
            if e.vcall(manager, 0x10, &args![critical]).u32() == 2 {
                let event_time = x87_quiet(e.mem.f32(event + EVENT_TIME));
                let island = e.mem.u32(island_slot);
                hk_ls_backstep_and_freeze_entire_island(e, event_time, island, flags, bodies);
                invalidate_bodies_and_clear(e, bodies, input);
                release_array(e, critical, 12);
                release_array(e, constraints, 4);
                release_array(e, new_bodies, 4);
                release_solve_arrays(e, manager_bodies, solve_bodies, records, saved_top_slot);
                return 0;
            }
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            let capacity = e.mem.u32(critical + 8);
            e.mem.set_u32(critical + 4, 0);
            if (capacity as i32) >= 0 {
                let data = e.mem.u32(critical);
                e.vcall(
                    allocator,
                    0x10,
                    &args![data, (capacity & 0x3fff_ffff).wrapping_mul(12)],
                );
            }
        }

        // The event's bodies go into an empty result array.
        if e.mem.u32(bodies + 4) == 0 {
            let allocator = heap_allocator(e);
            fn_00d10710(e, bodies, allocator, 0, first_slot);
            let first = e.mem.u32(e.mem.u32(bodies));
            set_body_state(e, flags, first, 0x10);
            if e.mem.u32(second_slot) != 0 {
                let allocator = heap_allocator(e);
                fn_00d10710(e, bodies, allocator, 1, second_slot);
                let second = e.mem.u32(e.mem.u32(bodies) + 4);
                set_body_state(e, flags, second, 0x10);
            }
        }
        let mut k = 0;
        while e.mem.i32(bodies + 4) > k {
            let body = e.mem.u32(e.mem.u32(bodies) + 4 * k as u32);
            let event_time = x87_quiet(e.mem.f32(event + EVENT_TIME));
            e.call(
                BACK_STEP_MOTION_STATE,
                &args![event_time, body + ENTITY_MOTION_STATE],
            );
            k += 1;
        }
        let data = e.mem.u32(bodies);
        let size = e.mem.u32(bodies + 4);
        let first = e.mem.u32(solver + 4);
        let second = e.mem.u32(solver);
        let third = e.mem.u32(solver + 0xc);
        e.call(ONE_STEP_INTEGRATE, &args![first, second, third, data, size]);
        let island = e.mem.u32(island_slot);
        hk_ls_restore_transform_on_bodies_with_updated_transform(e, island, flags, solver);
        let mut k = 0;
        while e.mem.i32(bodies + 4) > k {
            let body = e.mem.u32(e.mem.u32(bodies) + 4 * k as u32);
            e.call(INVALIDATE_TIM, &args![body, input]);
            k += 1;
        }
        release_array(e, constraints, 4);
        release_array(e, new_bodies, 4);
        release_solve_arrays(e, manager_bodies, solve_bodies, records, saved_top_slot);
        0
    })
}

// Translated from 00d0fe00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::advanceTime` (Xbox PDB): runs
/// `advanceTimeInternal` and, once the simulation has reached its target
/// time (`this + 0x20 <= this + 0x14`, both `float`, false for NaN), lets
/// the simulation's virtual slot `0x40` run the narrow phase with the
/// collision input and the entities map (`this + 0x3c`) when the map has
/// entries (`(word at this + 0x40) & 0x7fffffff != 0`). The abort flag
/// returns 3 at that point. With post-simulation listeners (`world[0x168]`)
/// `fireWorldPostSimulationCallback` (`00d01380`) runs between the profiling
/// records `TtPostSimCb` and `Et`. Returns the result of `advanceTimeInternal`.
pub fn hkp_continuous_simulation_advance_time(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
) -> u32 {
    let result = hkp_continuous_simulation_advance_time_internal(e, this);
    let target = e.get(this, HkpContinuousSimulation::float_20);
    let current = e.get(this, HkpContinuousSimulation::float_14);
    if target <= current {
        if e.mem.u32(this.addr() + SIM_ENTITIES_MAP + 4) & 0x7fff_ffff != 0 {
            let world = e.get(this, HkpContinuousSimulation::world).addr();
            let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
            e.vcall(
                this.addr(),
                0x40,
                &args![input, this.addr() + SIM_ENTITIES_MAP],
            );
        }
        if e.global::<u32>(ABORT_FLAG) == 1 {
            return 3;
        }
        let world = e.get(this, HkpContinuousSimulation::world).addr();
        if e.mem.u32(world + WORLD_POST_SIMULATION_LISTENER_COUNT) != 0 {
            profile_record(e, PROFILE_TT_POST_SIM_CB);
            let world = e.get(this, HkpContinuousSimulation::world).addr();
            e.call(FIRE_POST_SIMULATION_CALLBACK, &args![world]);
            profile_record(e, PROFILE_END);
        }
    }
    result
}

// Translated from 00d0fed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::reintegrateAndRecollideEntities` (Xbox PDB):
/// puts the given bodies back at the simulation's current step and runs the
/// collision detection for them again. `mode` bit 0 reintegrates, bit 1 runs
/// the broad phase and bit 2 the narrow phase through `fn_00d0f2e0`.
///
/// With the world locked, the collision input's step info (`+0x50`: start,
/// end, delta, inverse delta) is saved and replaced by `this + 0x14` as the
/// start, `this + 0x18` as the end, their difference and its inverse (0 for
/// a zero difference). Bit 0: every body that is not fixed (motion type 5)
/// has its motion state stepped back to the start (`00cf1b10`), then
/// `00d28680` runs over the bodies (with a pointer to the new step info and
/// the offset `0xe0`) and the bodies' bounding volumes are recalculated
/// (`00d1a330`). The simulation's virtual slot `0x20` is always called with
/// (`entities`, `count`, `world`, 1). Bit 1: the static broad phase
/// `00d0d3f0`. Bit 2: `fn_00d0f2e0` with the bodies and the collision input;
/// without bit 2 but with bit 0 the virtual slot `0x20` is called once more
/// with 2. Finally the lock is released and the saved step info restored.
pub fn hkp_continuous_simulation_reintegrate_and_recollide_entities(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    entities: u32,
    count: i32,
    world: u32,
    mode: u32,
) {
    world_lock(e, world);
    let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
    let start = x87_quiet(e.get(this, HkpContinuousSimulation::float_14));
    let saved = [
        e.mem.u32(input + COLLISION_INPUT_STEP_INFO),
        e.mem.u32(input + COLLISION_INPUT_STEP_INFO + 4),
        e.mem.u32(input + COLLISION_INPUT_STEP_INFO + 8),
        e.mem.u32(input + COLLISION_INPUT_STEP_INFO + 12),
    ];
    let end = x87_quiet(e.get(this, HkpContinuousSimulation::float_18));
    let delta = (end as f64 - start as f64) as f32;
    let inverse = if delta == 0.0 {
        0.0
    } else {
        (1.0 / delta as f64) as f32
    };
    let step = [
        start.to_bits(),
        end.to_bits(),
        delta.to_bits(),
        inverse.to_bits(),
    ];
    for (k, word) in step.iter().enumerate() {
        e.mem
            .set_u32(input + COLLISION_INPUT_STEP_INFO + 4 * k as u32, *word);
    }
    let reintegrate = mode & 1 != 0;
    e.with_stack(0x10, |e, step_block| {
        for (k, word) in step.iter().enumerate() {
            e.mem.set_u32(step_block.addr() + 4 * k as u32, *word);
        }
        if reintegrate {
            for i in 0..count {
                let body = e.mem.u32(entities + 4 * i as u32);
                if e.mem.u8(body + ENTITY_MOTION_TYPE) != 5 {
                    let current = x87_quiet(e.get(this, HkpContinuousSimulation::float_14));
                    e.call(
                        BACK_STEP_MOTION_STATE,
                        &args![current, body + ENTITY_MOTION_STATE],
                    );
                }
            }
            e.call(
                RIGID_MOTION_UTIL_STEP,
                &args![step_block.addr(), entities, count, 0xe0u32],
            );
            let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
            e.call(ENTITY_BATCH_RECALC_AABB, &args![input, entities, count]);
        }
    });
    e.vcall(this.addr(), 0x20, &args![entities, count, world, 1u32]);
    if mode & 2 != 0 {
        hkp_continuous_simulation_collide_entities_broad_phase_continuous(
            e, entities, count, world, 0,
        );
    }
    if mode & 4 != 0 {
        let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
        fn_00d0f2e0(e, this.addr(), entities, count as u32, input);
    } else if reintegrate {
        e.vcall(this.addr(), 0x20, &args![entities, count, world, 2u32]);
    }
    world_unlock(e, world);
    let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
    for (k, word) in saved.iter().enumerate() {
        e.mem
            .set_u32(input + COLLISION_INPUT_STEP_INFO + 4 * k as u32, *word);
    }
}

// Translated from 00d100a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::simulateToi` (Xbox PDB): solves one
/// time-of-impact event. `first_value` and `second_value` are the two
/// `float` arguments; `handleAllToisTill` passes `this + 0x1c` and the
/// world's rotate-normal value.
///
/// The resource manager (`this + 0x48`) is asked through its virtual slot
/// `0xc` (the event, the event array at `this + 0x30`, a block of eight
/// words holding 3, 1000, 1000, 1000, 4, 4, 0, 0); the answer 1 ends the
/// function. With the world locked a local `0x12c`-byte block is copied from
/// `world + 0x1e0` (`fn_00d105b0`) and set up: `+0x10c` the first value, `+0x110`
/// its inverse (a plain `1 / value`), `+0x114` and `+0x118` 1, `+0x120` 1.0,
/// `+0x10` and `+0x20` zeroed (16 bytes each), then `00c90e60` gets the
/// floats at `0x01016248` and `0x0104f498`, `hkpConstraintQueryIn::set`
/// (`00cfa6e0`) the block, the event time and the world's `+0x1b8`, and
/// `00d881d0` the solver set-up. The event time is kept next to the world's
/// current psi time (`00c91030`), their difference and its inverse. After the
/// profiling record pair `LtTOI`/`StSolveToi` a state table is taken from the
/// stack allocator (one byte per body of the event's island, zeroed) and
/// `hkLs_localizedSolveToi` runs with a local body array; the set-up is shut
/// down (`00d88300`) and the contact manager told about the event.
///
/// When the solve returns 0 the queued events of bodies that ended in state
/// `0x10` in this island are reported and dropped (as `0d0c3e0` does); with a
/// non-zero result the body array is emptied. The allocator is restored and
/// the manager's virtual slot `0x18` called (event, event array, the word
/// block). After the records `StPendingOperations` and `StCollide` the bodies in the
/// local array whose world pointer (`+8`) is 0 are removed (`00c905b0`);
/// the rest go through the static broad phase `00d0d3f0` and, unless the
/// abort flag is set, this simulation's virtual slot `0x3c` (bodies, count,
/// collision input, entities map); `00c90740` is called on the array with 4.
/// The lock is released, the record `lt` (the string at `0x01076f38`) written and the array released.
pub fn hkp_continuous_simulation_simulate_toi(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    world: u32,
    event: u32,
    first_value: f32,
    second_value: f32,
) {
    e.with_stack(0x400, |e, frame| {
        let b = frame.addr();
        let limits = b + 0x30;
        let event_time_slot = b + 0x20;
        let solve_bodies = b + 0x50;
        let solver = b + 0x160;
        let solver_block = b + 0x1b0;
        let query_in = b + 0x2e0;
        let table_slot = b + 0x18;
        let island_slot = b + 0x14;
        let this_address = this.addr();
        let events_array = this_address + SIM_EVENT_ARRAY;

        for (k, word) in [3u32, 0x3e8, 0x3e8, 0x3e8, 4, 4, 0, 0].iter().enumerate() {
            e.mem.set_u32(limits + 4 * k as u32, *word);
        }
        let manager = e
            .get(this, HkpContinuousSimulation::toi_resource_manager)
            .addr();
        let answer = e
            .vcall(manager, 0xc, &args![event, events_array, limits])
            .u32();
        if answer == 1 {
            return;
        }
        let simulation_world = e.get(this, HkpContinuousSimulation::world).addr();
        world_lock(e, simulation_world);
        let event_time = x87_quiet(e.mem.f32(event));
        e.mem.set_f32(event_time_slot, event_time);
        e.mem.set_u32(solve_bodies, b + 0x5c);
        e.mem.set_u32(solve_bodies + 4, 0);
        e.mem.set_u32(solve_bodies + 8, 0x8000_0040);
        let psi_time = x87_quiet(e.call(WORLD_GET_CURRENT_PSI_TIME, &args![world]).f32());
        e.mem.set_f32(b + 0x24, psi_time);
        let delta = (psi_time as f64 - event_time as f64) as f32;
        e.mem.set_f32(b + 0x28, delta);
        let inverse = (1.0 / delta as f64) as f32;
        e.mem.set_f32(b + 0x2c, inverse);
        fn_00d105b0(e, solver_block, world + 0x1e0);
        e.mem.set_f32(solver_block + 0x120, 1.0);
        let first = x87_quiet(first_value);
        e.mem.set_f32(solver_block + 0x10c, first);
        e.mem.set_u32(solver_block + 0x114, 1);
        e.mem
            .set_f32(solver_block + 0x110, (1.0 / first as f64) as f32);
        e.mem.set_u32(solver_block + 0x118, 1);
        write_vec4(e, solver_block + 0x10, [0.0; 4]);
        write_vec4(e, solver_block + 0x20, [0.0; 4]);
        let first_float = x87_quiet(e.global::<f32>(CONSTANT_01016248));
        let second_float = x87_quiet(e.global::<f32>(CONSTANT_0104F498));
        e.call(
            WORLD_00C90E60,
            &args![solver_block, first_float, second_float],
        );
        let violated = e.mem.u32(world + WORLD_VIOLATED_CONSTRAINT_ARRAY);
        e.call(
            CONSTRAINT_QUERY_IN_SET,
            &args![query_in, solver_block, event_time_slot, violated],
        );
        let zero_a = e.mem.u32(b + 0x48);
        let zero_b = e.mem.u32(b + 0x4c);
        e.call(
            SOLVER_SETUP_00D881D0,
            &args![
                event_time_slot,
                solver_block,
                query_in,
                zero_a,
                zero_b,
                solver
            ],
        );
        profile_record_pair(e, PROFILE_LT_TOI, PROFILE_ST_SOLVE_TOI);
        let first_body = e.mem.u32(event + EVENT_ENTITY_A);
        let island = if e.mem.u8(first_body + ENTITY_MOTION_TYPE) == 5 {
            e.mem
                .u32(e.mem.u32(event + EVENT_ENTITY_B) + ENTITY_SIMULATION_ISLAND)
        } else {
            e.mem.u32(first_body + ENTITY_SIMULATION_ISLAND)
        };
        let island_count = e.mem.u32(island + ISLAND_ENTITY_COUNT);
        e.mem.set_u32(island_slot, island);
        let table = stack_alloc(e, island_count);
        e.mem.set_u32(b + 0xc, table);
        e.call(MEM_SET, &args![table, 0u32, island_count]);
        e.mem.set_u32(table_slot, table);
        let solved = hk_ls_localized_solve_toi(
            e,
            limits,
            solver,
            event,
            manager,
            world,
            solve_bodies,
            table_slot,
            x87_quiet(second_value),
        );
        e.call(SHUTDOWN_SOLVER, &args![solver]);
        notify_event_island(e, event);
        if solved == 0 {
            let mut i = e.get(this, HkpContinuousSimulation::event_count) - 1;
            while i >= 0 {
                let queued =
                    e.get(this, HkpContinuousSimulation::event_data) + i as u32 * TOI_EVENT_SIZE;
                let body_a = e.mem.u32(queued + EVENT_ENTITY_A);
                let body_b = e.mem.u32(queued + EVENT_ENTITY_B);
                let state_of = |e: &Engine, body: u32| {
                    e.mem
                        .u8(e.mem.u16(body + ENTITY_STORAGE_INDEX) as u32 + table)
                };
                let matches = (e.mem.u32(body_a + ENTITY_SIMULATION_ISLAND) == island
                    && state_of(e, body_a) == 0x10)
                    || (e.mem.u32(body_b + ENTITY_SIMULATION_ISLAND) == island
                        && state_of(e, body_b) == 0x10);
                if matches {
                    notify_event_island(e, queued);
                    remove_event_at(e, this, i);
                }
                i -= 1;
            }
            let data = e.mem.u32(solve_bodies);
            let size = e.mem.u32(solve_bodies + 4);
            e.call(ADD_REFERENCES, &args![data, size, 4u32]);
        } else {
            e.mem.set_u32(solve_bodies + 4, 0);
        }
        stack_release(e, table);
        e.vcall(manager, 0x18, &args![event, events_array, limits]);
        profile_record(e, PROFILE_ST_PENDING_OPERATIONS);
        profile_record(e, PROFILE_ST_COLLIDE);
        let mut size = e.mem.i32(solve_bodies + 4);
        if size != 0 {
            let mut i = size - 1;
            while i >= 0 {
                let data = e.mem.u32(solve_bodies);
                let body = e.mem.u32(data + 4 * i as u32);
                if e.mem.u32(body + ENTITY_WORLD) == 0 {
                    size -= 1;
                    e.mem.set_i32(solve_bodies + 4, size);
                    if size != i {
                        let last = e.mem.u32(data + 4 * size as u32);
                        e.mem.set_u32(data + 4 * i as u32, last);
                    }
                    e.call(REMOVE_REFERENCE, &args![body]);
                    size = e.mem.i32(solve_bodies + 4);
                }
                i -= 1;
            }
            let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
            let data = e.mem.u32(solve_bodies);
            if size != 0 {
                hkp_continuous_simulation_collide_entities_broad_phase_continuous(
                    e, data, size, world, 0,
                );
                if e.global::<u32>(ABORT_FLAG) != 1 {
                    e.vcall(
                        this_address,
                        0x3c,
                        &args![data, size, input, this_address + SIM_ENTITIES_MAP],
                    );
                }
            }
            let data = e.mem.u32(solve_bodies);
            let size = e.mem.u32(solve_bodies + 4);
            e.call(REFERENCE_ARRAY_FN_00C90740, &args![data, size, 4u32]);
        }
        let simulation_world = e.get(this, HkpContinuousSimulation::world).addr();
        world_unlock(e, simulation_world);
        profile_record(e, PROFILE_LT);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(solve_bodies + 8);
        e.mem.set_u32(solve_bodies + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(solve_bodies);
            e.vcall(
                allocator,
                0x10,
                &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
            );
        }
    });
}

// Translated from 00d105b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the engine map. Copies a `0x12c`-byte block from `source`
/// to `this`: the `float`s at `+0` to `+0xc`, `+0x50` to `+0x60`, `+0x10c`,
/// `+0x110`, `+0x11c`, `+0x120` and `+0x128` through the x87 stack (a
/// signalling NaN comes out quiet), everything else (`+0x10` to `+0x4f`,
/// `+0x64` to `+0x10b`, `+0x114`, `+0x118` and the four bytes at `+0x124`)
/// unchanged.
pub fn fn_00d105b0(e: &mut Engine, this: u32, source: u32) {
    const FLOATS: [u32; 14] = [
        0x0, 0x4, 0x8, 0xc, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x10c, 0x110, 0x11c, 0x120, 0x128,
    ];
    for offset in FLOATS {
        let bits = e.mem.u32(source + offset);
        e.mem.set_u32(this + offset, x87_quiet_bits(bits));
    }
    let bytes = e.mem.bytes(source + 0x10, 0x40);
    e.mem.write(this + 0x10, &bytes);
    let bytes = e.mem.bytes(source + 0x64, 0xa8);
    e.mem.write(this + 0x64, &bytes);
    let bytes = e.mem.bytes(source + 0x114, 8);
    e.mem.write(this + 0x114, &bytes);
    let bytes = e.mem.bytes(source + 0x124, 4);
    e.mem.write(this + 0x124, &bytes);
}

// Translated from 00d10710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Not named in the engine map. Calls `00cd1e20` (an `hkArray` function that
/// takes the array in `ECX`, which is forwarded untouched, and four stack
/// arguments) with its three arguments and a fourth, 1.
pub fn fn_00d10710(e: &mut Engine, this: u32, first: u32, second: u32, third: u32) {
    e.call(ARRAY_INSERT_AT, &args![this, first, second, third, 1u32]);
}

// Translated from 00d10730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContinuousSimulation::_scalar_deleting_destructor_` (Xbox PDB): runs
/// the destructor (`00d0d0d0`) and, when bit 0 of `flags` is set, frees the
/// object through the heap allocator's virtual slot `0x08` (the object and
/// its `u16` at `+4`). Returns `this`.
pub fn hkp_continuous_simulation_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<HkpContinuousSimulation>,
    flags: u32,
) -> Ptr<HkpContinuousSimulation> {
    hkp_continuous_simulation_destroy(e, this);
    if flags & 1 != 0 {
        let allocator = heap_allocator(e);
        let size = e.mem.u16(this.addr() + 4) as u32;
        e.vcall(allocator, 0x08, &args![this.addr(), size]);
    }
    this
}

// Translated from 00d0bc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words `out`, `first`, `second`; descriptions, not
/// confirmed): a "less than" test stored as a byte at `out` (code in
/// `hkLs_toiActivateEntitiesAndCheckConstraints` stores its address at `00d0e986`) and answered as
/// `out`. The byte is 1 when the word at `+0xd4` of `first` is below that of `second`
/// (unsigned).
pub fn fn_00d0bc60(e: &mut Engine, out: Ptr, first: Ptr, second: Ptr) -> u32 {
    let first_key = e.mem.u32(first.addr().wrapping_add(0xd4));
    let second_key = e.mem.u32(second.addr().wrapping_add(0xd4));
    e.mem.set_u8(out.addr(), u8::from(first_key < second_key));
    out.addr()
}

// Translated from 00d0bc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, three words `out`, `first`, `second`; descriptions, not
/// confirmed): a "less than" test stored as a byte at `out` (the code at `00d0e963` stores its
/// address) and answered as `out`. Each argument's word at `+0x30` points to a list whose byte
/// at `+0x1a` picks one of the pointers from `+4`; the key is the word at `+0xd4` of that
/// object. The byte is 1 when `first`'s key is below `second`'s (unsigned); equal keys are
/// decided by comparing the two `+0x30` pointers (unsigned); a larger key is 0.
pub fn fn_00d0bc80(e: &mut Engine, out: Ptr, first: Ptr, second: Ptr) -> u32 {
    let key = |e: &Engine, list: u32| -> u32 {
        let index = u32::from(e.mem.u8(list.wrapping_add(0x1a)));
        let object = e
            .mem
            .u32(list.wrapping_add(index.wrapping_mul(4)).wrapping_add(4));
        e.mem.u32(object.wrapping_add(0xd4))
    };
    let first_list = e.mem.u32(first.addr().wrapping_add(0x30));
    let second_list = e.mem.u32(second.addr().wrapping_add(0x30));
    let less = if key(e, first_list) < key(e, second_list) {
        true
    } else if key(e, first_list) == key(e, second_list) {
        first_list < second_list
    } else {
        false
    };
    e.mem.set_u8(out.addr(), u8::from(less));
    out.addr()
}

// Translated from 00d0daa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three stack words `first`, `second`, `third`,
/// `RET 0xc`; descriptions, not confirmed; the code at `00d0f2ea` stores its address): sets
/// up and runs one pair. The float at `0104da48` goes to `third + 0x3030`, `third + 0x3080`
/// becomes 0 and the word at `third` points at `third + 0x10`. The signed byte at `first + 0xc`
/// (times 0x40) plus the word at `second` plus `0x1c20` goes to `second + 0x60`, and the word
/// at its `+0x10` to `second + 0x14`. Then `00cfedd0` runs with `(first, second, third, [first +
/// 8])` (cdecl). Unless the word at `01269200` is 1: when the word at `third` no longer points
/// at `third + 0x10`, the object at `[first + 8]` is asked through its slot `0x18` with
/// `([first + 0x10], [first + 0x14], second, third)`; and when the float at `third + 0x3030` is
/// not equal to the double at `0104e138` (a NaN is not equal either) `00d0d150` runs on `this`
/// with `(third, first)`.
pub fn fn_00d0daa0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr, third: Ptr) {
    let (first, second, third) = (first.addr(), second.addr(), third.addr());
    let start = x87_quiet_bits(e.global::<u32>(0x0104_da48));
    e.mem.set_u32(third.wrapping_add(0x3030), start);
    e.mem.set_u32(third.wrapping_add(0x3080), 0);
    let list = third.wrapping_add(0x10);
    e.mem.set_u32(third, list);
    let offset = (i32::from(e.mem.i8(first.wrapping_add(0xc))) << 6) as u32;
    let base = e.mem.u32(second);
    let record = offset.wrapping_add(base).wrapping_add(0x1c20);
    e.mem.set_u32(second.wrapping_add(0x60), record);
    let word = e.mem.u32(record.wrapping_add(0x10));
    e.mem.set_u32(second.wrapping_add(0x14), word);
    let owner = e.mem.u32(first.wrapping_add(8));
    e.call(0x00cf_edd0, &args![first, second, third, owner]);
    if e.global::<u32>(0x0126_9200) != 1 {
        if e.mem.u32(third) != list {
            let object = e.mem.u32(first.wrapping_add(8));
            let second_word = e.mem.u32(first.wrapping_add(0x14));
            let first_word = e.mem.u32(first.wrapping_add(0x10));
            e.vcall(object, 0x18, &args![first_word, second_word, second, third]);
        }
        let value = x87_load32(e.mem.u32(third.wrapping_add(0x3030)));
        let limit: f64 = e.global(0x0104_e138);
        // `FCOMP` then `TEST AH,0x44` / `JNP`: the call is skipped only when the two are equal;
        // for a NaN `!=` is true, as the flags say.
        if value != limit {
            e.call(0x00d0_d150, &args![this, third, first]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0bc60, fn_00d0bc60(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d0bc80, fn_00d0bc80(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d0daa0, fn_00d0daa0(Ptr, Ptr, Ptr, Ptr)),

        entry!(0x00d0bc20, fn_00d0bc20(u32, u32, u32) -> u32),
        entry!(
            0x00d0bc50,
            hkp_continuous_simulation_assert_there_is_no_collision_information_for_entities(
                Ptr<HkpContinuousSimulation>,
                u32,
                u32,
                u32
            )
        ),
        entry!(
            0x00d0bcf0,
            hkp_continuous_simulation_assert_there_is_no_collision_information_for_agent(
                Ptr<HkpContinuousSimulation>,
                u32
            )
        ),
        entry!(
            0x00d0bd00,
            hkp_continuous_simulation_warp_time(Ptr<HkpContinuousSimulation>, f32)
        ),
        entry!(
            0x00d0bd30,
            hk_ls_are_velocities_ok(f32, u32, u32, i32, u32) -> i32
        ),
        entry!(0x00d0be80, fn_00d0be80(u32, u32, i32)),
        entry!(
            0x00d0bf30,
            hk_ls_toi_reset_velocity_accumulators_for_entities(u32, u32)
        ),
        entry!(
            0x00d0bfc0,
            hkp_continuous_simulation_remove_collision_information_for_agent(
                Ptr<HkpContinuousSimulation>,
                u32
            )
        ),
        entry!(
            0x00d0c030,
            hkp_continuous_simulation_collide_entities_broad_phase_continuous_find_pairs(
                u32, i32, u32, u32, u32
            )
        ),
        entry!(0x00d0c290, fn_00d0c290(Ptr<HkpContinuousSimulation>, u32, i32)),
        entry!(0x00d0c3e0, fn_00d0c3e0(Ptr<HkpContinuousSimulation>, u32)),
        entry!(
            0x00d0c480,
            hk_ls_do_simple_collision_response(u32, u32, f32, u32)
        ),
        entry!(0x00d0c6d0, fn_00d0c6d0(u32, u32, u32)),
        entry!(
            0x00d0c790,
            hk_ls_restore_transform_on_bodies_with_updated_transform(u32, u32, u32)
        ),
        entry!(
            0x00d0c8d0,
            hkp_continuous_simulation_calc_content_statistics(
                Ptr<HkpContinuousSimulation>,
                u32,
                u32
            )
        ),
        entry!(
            0x00d0c970,
            hkp_continuous_simulation_reset_collision_information_for_entities(
                Ptr<HkpContinuousSimulation>,
                u32,
                i32,
                u32,
                u32
            )
        ),
        entry!(
            0x00d0c9c0,
            hkp_continuous_simulation_process_agent_nn_entries(u32, u32, i32, u32, u32, u32)
        ),
        entry!(
            0x00d0cb00,
            hk_ls_toi_check_validity_of_constraints(u32, u32, u32, u32, i32, u32, u32, u32)
        ),
        entry!(
            0x00d0cde0,
            hk_ls_backstep_and_freeze_entire_island(f32, u32, u32, u32)
        ),
        entry!(
            0x00d0ceb0,
            hk_ls_toi_check_final_validity_of_critical_constraints(u32, u32, u32, u32, u32)
                -> u32
        ),
        entry!(
            0x00d0d020,
            hkp_continuous_simulation_new(Ptr<HkpContinuousSimulation>, u32)
                -> Ptr<HkpContinuousSimulation>
        ),
        entry!(
            0x00d0d0d0,
            hkp_continuous_simulation_destroy(Ptr<HkpContinuousSimulation>)
        ),
        entry!(
            0x00d0d150,
            hkp_continuous_simulation_add_toi_event(Ptr<HkpContinuousSimulation>, u32, u32)
        ),
        entry!(
            0x00d0d2c0,
            hk_ls_collect_agent_entries_to_process(u32, i32, f32, u32, u32)
        ),
        entry!(
            0x00d0d3f0,
            hkp_continuous_simulation_collide_entities_broad_phase_continuous(
                u32, i32, u32, u32
            )
        ),
        entry!(
            0x00d0d8b0,
            hkp_continuous_simulation_collide_island_narrow_phase_continuous(
                Ptr<HkpContinuousSimulation>,
                u32,
                u32
            )
        ),
        entry!(
            0x00d0db40,
            hkp_continuous_simulation_collide_entities_of_one_island_narrow_phase_continuous_toi_only(
                Ptr<HkpContinuousSimulation>,
                u32,
                i32,
                u32,
                u32
            )
        ),
        entry!(
            0x00d0de70,
            hkp_continuous_simulation_collide_entities_needing_psi_collision_detection_narrow_phase_toi_only(
                Ptr<HkpContinuousSimulation>,
                u32,
                u32
            )
        ),
        entry!(
            0x00d0e210,
            hkp_continuous_simulation_handle_simple_toi(
                Ptr<HkpContinuousSimulation>,
                u32,
                u32,
                u32,
                f32
            )
        ),
        entry!(
            0x00d0e4b0,
            hk_ls_toi_activate_entities_and_check_constraints(
                u32, i32, f32, u32, u32, u32, u32
            )
        ),
        entry!(
            0x00d0e9c0,
            hkp_continuous_simulation_handle_all_tois_till(Ptr<HkpContinuousSimulation>, f32)
                -> u32
        ),
        entry!(
            0x00d0ed10,
            hkp_continuous_simulation_advance_time_internal(Ptr<HkpContinuousSimulation>)
                -> u32
        ),
        entry!(
            0x00d0ef40,
            hkp_continuous_simulation_collide_internal(Ptr<HkpContinuousSimulation>, u32)
        ),
        entry!(0x00d0f2e0, fn_00d0f2e0(u32, u32, u32, u32)),
        entry!(
            0x00d0f300,
            hk_ls_localized_solve_toi(u32, u32, u32, u32, u32, u32, u32, f32) -> u32
        ),
        entry!(
            0x00d0fe00,
            hkp_continuous_simulation_advance_time(Ptr<HkpContinuousSimulation>) -> u32
        ),
        entry!(
            0x00d0fed0,
            hkp_continuous_simulation_reintegrate_and_recollide_entities(
                Ptr<HkpContinuousSimulation>,
                u32,
                i32,
                u32,
                u32
            )
        ),
        entry!(
            0x00d100a0,
            hkp_continuous_simulation_simulate_toi(
                Ptr<HkpContinuousSimulation>,
                u32,
                u32,
                f32,
                f32
            )
        ),
        entry!(0x00d105b0, fn_00d105b0(u32, u32)),
        entry!(0x00d10710, fn_00d10710(u32, u32, u32, u32)),
        entry!(
            0x00d10730,
            hkp_continuous_simulation_scalar_deleting_destructor(
                Ptr<HkpContinuousSimulation>,
                u32
            ) -> Ptr<HkpContinuousSimulation>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    #[test]
    fn fn_00d0bc60_stores_whether_the_first_key_is_below_the_second() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        let low = e.mem.alloc(0x100);
        let high = e.mem.alloc(0x100);
        e.mem.set_u32(low + 0xd4, 2);
        e.mem.set_u32(high + 0xd4, 0xffff_fff0);
        assert_eq!(e.call(0x00d0_bc60, &args![out, low, high]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x00d0_bc60, &args![out, high, low]);
        assert_eq!(e.mem.u8(out), 0);
        e.call(0x00d0_bc60, &args![out, low, low]);
        assert_eq!(e.mem.u8(out), 0);
    }

    /// An argument for `00d0bc80`: its word at `+0x30` is a list whose byte at `+0x1a` picks the
    /// second pointer from `+4`, an object with the key `key` at `+0xd4`. Returns (argument, list).
    fn listed_00d0bc80(e: &mut Engine, key: u32) -> (u32, u32) {
        let argument = e.mem.alloc(0x40);
        let list = e.mem.alloc(0x40);
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0xd4, key);
        e.mem.set_u8(list + 0x1a, 1);
        e.mem.set_u32(list + 0x8, object);
        e.mem.set_u32(argument + 0x30, list);
        (argument, list)
    }

    #[test]
    fn fn_00d0bc80_orders_by_the_key_of_the_picked_object() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        let (low, _) = listed_00d0bc80(&mut e, 1);
        let (high, _) = listed_00d0bc80(&mut e, 9);
        assert_eq!(e.call(0x00d0_bc80, &args![out, low, high]).u32(), out);
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x00d0_bc80, &args![out, high, low]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn fn_00d0bc80_breaks_a_tie_with_the_list_addresses() {
        let mut e = Engine::new();
        let out = e.mem.alloc(8);
        let (first, first_list) = listed_00d0bc80(&mut e, 5);
        let (second, second_list) = listed_00d0bc80(&mut e, 5);
        assert!(first_list < second_list);
        e.call(0x00d0_bc80, &args![out, first, second]);
        assert_eq!(e.mem.u8(out), 1);
        e.call(0x00d0_bc80, &args![out, second, first]);
        assert_eq!(e.mem.u8(out), 0);
        e.call(0x00d0_bc80, &args![out, first, first]);
        assert_eq!(e.mem.u8(out), 0);
    }

    /// Everything `00d0daa0` reads: the float and double constants, the flag word, `first` (signed
    /// byte 2 at `+0xc`, an owner object at `+8` with a logging vtable slot `0x18`), `second`
    /// (a base pointer at `+0`) and `third` (0x3100 bytes).
    struct Pair00d0daa0 {
        e: Engine,
        this: u32,
        first: u32,
        second: u32,
        third: u32,
        owner: u32,
    }

    fn pair_00d0daa0(float: f32, flag: u32) -> Pair00d0daa0 {
        let mut e = Engine::new();
        e.map(0x0104_d000, 0x2000);
        e.map(0x0126_9000, 0x1000);
        e.mem.set_f32(0x0104_da48, float);
        e.mem.set_f64(0x0104_e138, 0.0);
        e.mem.set_u32(0x0126_9200, flag);
        let this = e.mem.alloc(0x20);
        let first = e.mem.alloc(0x40);
        let second = e.mem.alloc(0x80);
        let third = e.mem.alloc(0x3100);
        let base = e.mem.alloc(0x2000);
        e.mem.set_u32(second, base);
        e.mem.set_u8(first + 0xc, 2);
        e.mem.set_u32(base + 0x80 + 0x1c20 + 0x10, 0xabcd);
        let owner = e.mem.alloc(8);
        e.put_vtable(0x0900_0000, &[0, 0, 0, 0, 0, 0, 0x0900_0118]);
        e.mem.set_u32(owner, 0x0900_0000);
        e.mem.set_u32(first + 8, owner);
        e.mem.set_u32(first + 0x10, 0x11);
        e.mem.set_u32(first + 0x14, 0x22);
        e.register(0x0900_0118, |_, _| Ret::default());
        e.register(0x00cf_edd0, |_, _| Ret::default());
        e.register(0x00d0_d150, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        Pair00d0daa0 {
            e,
            this,
            first,
            second,
            third,
            owner,
        }
    }

    fn logged_00d0daa0(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fn_00d0daa0_sets_up_the_pair_and_runs_the_helper() {
        let mut p = pair_00d0daa0(1.5, 0);
        let (this, first, second, third) = (p.this, p.first, p.second, p.third);
        p.e.call(0x00d0_daa0, &args![this, first, second, third]);
        let base = p.e.mem.u32(second);
        assert_eq!(p.e.mem.f32(third + 0x3030), 1.5);
        assert_eq!(p.e.mem.u32(third + 0x3080), 0);
        assert_eq!(p.e.mem.u32(third), third + 0x10);
        assert_eq!(p.e.mem.u32(second + 0x60), base + 0x80 + 0x1c20);
        assert_eq!(p.e.mem.u32(second + 0x14), 0xabcd);
        assert_eq!(
            logged_00d0daa0(&p.e, 0x00cf_edd0),
            vec![vec![first, second, third, p.owner]]
        );
        // The list word still points at its own storage, so the owner is not asked; 1.5 is not
        // the compared double 0.0, so the follow-up helper runs.
        assert!(logged_00d0daa0(&p.e, 0x0900_0118).is_empty());
        assert_eq!(
            logged_00d0daa0(&p.e, 0x00d0_d150),
            vec![vec![this, third, first]]
        );
    }

    #[test]
    fn fn_00d0daa0_skips_the_helper_when_the_float_equals_the_double() {
        let mut p = pair_00d0daa0(0.0, 0);
        let (this, first, second, third) = (p.this, p.first, p.second, p.third);
        p.e.call(0x00d0_daa0, &args![this, first, second, third]);
        assert!(logged_00d0daa0(&p.e, 0x00d0_d150).is_empty());
    }

    #[test]
    fn fn_00d0daa0_asks_the_owner_when_the_list_word_moved() {
        let mut p = pair_00d0daa0(0.0, 0);
        let (this, first, second, third) = (p.this, p.first, p.second, p.third);
        // The first helper leaves the list word pointing elsewhere.
        p.e.register_double(0x00cf_edd0, move |e, _| {
            e.mem.set_u32(third, 0x5555);
            Ret::default()
        });
        p.e.call(0x00d0_daa0, &args![this, first, second, third]);
        assert_eq!(
            logged_00d0daa0(&p.e, 0x0900_0118),
            vec![vec![p.owner, 0x11, 0x22, second, third]]
        );
    }

    #[test]
    fn fn_00d0daa0_stops_after_the_first_helper_when_the_flag_word_is_one() {
        let mut p = pair_00d0daa0(1.5, 1);
        let (this, first, second, third) = (p.this, p.first, p.second, p.third);
        p.e.register_double(0x00cf_edd0, move |e, _| {
            e.mem.set_u32(third, 0x5555);
            Ret::default()
        });
        p.e.call(0x00d0_daa0, &args![this, first, second, third]);
        assert!(logged_00d0daa0(&p.e, 0x0900_0118).is_empty());
        assert!(logged_00d0daa0(&p.e, 0x00d0_d150).is_empty());
    }

    /// The `this` word of the virtual calls: these functions never read `ECX`.
    const VCALL_THIS: u32 = 0x7777_0000;

    type Calls = Vec<(u32, Vec<u32>)>;
    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    const ROUTER_BLOCK: u32 = 0x0400_1000;
    const ALLOCATOR: u32 = ROUTER_BLOCK + 0x40;
    const ALLOCATOR_VTABLE: u32 = 0x0400_2000;
    const ALLOCATE_FN: u32 = 0x0400_2104;
    const DELETE_FN: u32 = 0x0400_2108;
    const FREE_FN: u32 = 0x0400_2110;
    const CAN_ALLOCATE_FN: u32 = 0x0400_2124;
    const STREAM: u32 = 0x0400_3000;
    const STREAM_RECORDS: u32 = 0x0400_3100;
    const STACK_BASE: u32 = 0x0500_0000;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with the pages the constants and globals live in, the
    /// memory router (stack allocator at `STACK_BASE`, a heap allocator whose
    /// allocate/delete/free calls are doubles), the profile stream and
    /// `TlsGetValue`.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.map(ROUTER_BLOCK, 0x1000);
        e.map(STREAM, 0x1000);
        e.map(STACK_BASE, 0x4_0000);
        e.mem.set_u32(ROUTER_BLOCK, STACK_BASE);
        e.mem.set_u32(ROUTER_BLOCK + 0x10, ALLOCATOR);
        e.mem.set_u32(ALLOCATOR, ALLOCATOR_VTABLE);
        e.put_vtable(
            ALLOCATOR_VTABLE,
            &[
                0,
                ALLOCATE_FN,
                DELETE_FN,
                0,
                FREE_FN,
                0,
                0,
                0,
                0,
                CAN_ALLOCATE_FN,
            ],
        );
        e.register_double(MEMORY_ROUTER, |_, _| ret(ROUTER_BLOCK));
        e.register_double(ALLOCATE_FN, |e, a| ret(e.mem.alloc(a[1])));
        e.register_double(DELETE_FN, |_, _| Ret::default());
        e.register_double(FREE_FN, |_, _| Ret::default());
        e.register_double(CAN_ALLOCATE_FN, |_, _| ret(1));
        e.mem.set_u32(STREAM + 4, STREAM_RECORDS);
        e.mem.set_u32(STREAM + 0xc, STREAM_RECORDS + 0x400);
        e.set_global(PROFILE_SLOT, 5u32);
        e.register_double(TLS_GET_VALUE, |_, a| {
            assert_eq!(a, [5]);
            ret(STREAM)
        });
        e
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register_double(address, |_, _| Ret::default());
        }
    }

    fn log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    fn take_log(e: &mut Engine) -> Calls {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(address, _)| !funcs().iter().any(|(own, _)| own == address))
            .collect()
    }

    /// Growing an array: the capacity doubles (at least one element).
    fn grow_double(e: &mut Engine) {
        e.register_double(ARRAY_GROW, |e, a| {
            let (array, element) = (a[1], a[2]);
            let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
            let data = e.mem.alloc(element * (capacity * 2).max(1));
            let old = e.mem.bytes(e.mem.u32(array), element * capacity);
            e.mem.write(data, &old);
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, (capacity * 2).max(1));
            Ret::default()
        });
    }

    fn calls_to(calls: &Calls, address: u32) -> Vec<Vec<u32>> {
        calls
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// An object whose vtable (heap block) holds `slots` (byte offset,
    /// target).
    fn virtual_object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A body with a motion type, uid, storage index and island.
    fn body(e: &mut Engine, motion_type: u8, uid: u32, storage_index: u16, island: u32) -> u32 {
        let body = e.mem.alloc(0x220);
        e.mem.set_u8(body + ENTITY_MOTION_TYPE, motion_type);
        e.mem.set_u32(body + ENTITY_UID, uid);
        e.mem.set_u16(body + ENTITY_STORAGE_INDEX, storage_index);
        e.mem.set_u32(body + ENTITY_SIMULATION_ISLAND, island);
        body
    }

    /// A world with its collision input and a dispatcher block.
    fn world(e: &mut Engine) -> u32 {
        let world = e.mem.alloc(0x400);
        let input = e.mem.alloc(0x100);
        e.mem.set_u32(world + WORLD_COLLISION_INPUT, input);
        let dispatcher = e.mem.alloc(0x2000);
        e.mem
            .set_u32(world + WORLD_COLLISION_DISPATCHER, dispatcher);
        e.mem.set_u32(input, dispatcher);
        world
    }

    fn simulation(e: &mut Engine, world: u32) -> Ptr<HkpContinuousSimulation> {
        let this: Ptr<HkpContinuousSimulation> = e.new_object();
        e.set(this, HkpContinuousSimulation::world, Ptr::new(world));
        e.mem.set_u32(world + WORLD_SIMULATION, this.addr());
        this
    }

    /// Fills the event array of the simulation with `count` zeroed events.
    fn events(e: &mut Engine, this: Ptr<HkpContinuousSimulation>, count: i32) -> u32 {
        let data = e.mem.alloc(TOI_EVENT_SIZE * count.max(1) as u32);
        e.set(this, HkpContinuousSimulation::event_data, data);
        e.set(this, HkpContinuousSimulation::event_count, count);
        e.set(
            this,
            HkpContinuousSimulation::event_capacity,
            0x8000_0000 | count as u32,
        );
        data
    }

    /// An hkArray (data, size, capacity word) of words.
    fn word_array(e: &mut Engine, items: &[u32], capacity: u32) -> u32 {
        let array = e.mem.alloc(16);
        let data = e.mem.alloc(4 * capacity.max(1));
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *item);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, items.len() as u32);
        e.mem.set_u32(array + 8, capacity);
        array
    }

    fn array_items(e: &Engine, array: u32) -> Vec<u32> {
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u32(e.mem.u32(array) + 4 * i))
            .collect()
    }

    /// A constraint record between two bodies: the instance with its
    /// internal block (bodies at +4 and +8).
    fn instance(e: &mut Engine, first: u32, second: u32, priority: u8) -> u32 {
        let instance = e.mem.alloc(0x40);
        let internal = e.mem.alloc(0x40);
        e.mem.set_u32(instance + INSTANCE_INTERNAL, internal);
        e.mem.set_u32(internal, instance);
        e.mem.set_u32(internal + INTERNAL_ENTITY_A, first);
        e.mem.set_u32(internal + INTERNAL_ENTITY_B, second);
        e.mem.set_u8(instance + INSTANCE_PRIORITY, priority);
        instance
    }

    /// An hkArray of 12-byte records starting with the given instances.
    fn record_array(e: &mut Engine, instances: &[u32], capacity: u32) -> u32 {
        let array = e.mem.alloc(16);
        let data = e.mem.alloc(12 * capacity.max(1));
        for (i, instance) in instances.iter().enumerate() {
            e.mem.set_u32(data + 12 * i as u32, *instance);
            e.mem.set_u32(data + 12 * i as u32 + 4, 0x100 + i as u32);
            e.mem.set_u32(data + 12 * i as u32 + 8, 0x200 + i as u32);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, instances.len() as u32);
        e.mem.set_u32(array + 8, capacity);
        array
    }

    fn record_instances(e: &Engine, array: u32) -> Vec<u32> {
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u32(e.mem.u32(array) + 12 * i))
            .collect()
    }

    /// An event at `address` with the bodies, the contact manager and a time.
    fn set_event(e: &mut Engine, event: u32, time: f32, first: u32, second: u32, manager: u32) {
        e.mem.set_f32(event + EVENT_TIME, time);
        e.mem.set_u32(event + EVENT_ENTITY_A, first);
        e.mem.set_u32(event + EVENT_ENTITY_B, second);
        e.mem.set_u32(event + EVENT_CONTACT_MANAGER, manager);
    }

    fn event_at(e: &Engine, this: Ptr<HkpContinuousSimulation>, index: i32) -> u32 {
        e.get(this, HkpContinuousSimulation::event_data) + index as u32 * TOI_EVENT_SIZE
    }

    /// A contact manager whose slot 0x20 (at `address`) records its arguments
    /// (manager, island, properties) into `seen`.
    fn reporting_manager(e: &mut Engine, address: u32, seen: Seen) -> u32 {
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        virtual_object(e, &[(0x20, address)])
    }

    /// The addresses in `calls` that are in `wanted`, in call order.
    fn only(calls: &Calls, wanted: &[u32]) -> Vec<u32> {
        calls
            .iter()
            .map(|(address, _)| *address)
            .filter(|address| wanted.contains(address))
            .collect()
    }

    /// A body with bounding data: the low words at +0x30, the high words at
    /// +0x40, the offset bytes 1, 2, 3 and 1, 1, 1 and the shift 1.
    fn bounded_body(e: &mut Engine, low: [u32; 3], high: [u32; 3]) -> u32 {
        let body = e.mem.alloc(0x220);
        for axis in 0..3u32 {
            e.mem.set_u32(body + 0x30 + 4 * axis, low[axis as usize]);
            e.mem.set_u32(body + 0x40 + 4 * axis, high[axis as usize]);
            e.mem.set_u8(body + 0x3c + axis, axis as u8 + 1);
            e.mem.set_u8(body + 0x4c + axis, 1);
        }
        e.mem.set_u8(body + 0x3f, 1);
        body
    }

    /// Doubles for the pointer-map functions. A map is (data, size, hash
    /// mask): reserve sets the mask to 7, insert remembers the key and find
    /// answers 0 for a remembered key and 8 (past the mask) for any other.
    /// Returns the remembered keys.
    fn map_doubles(e: &mut Engine) -> Rc<RefCell<Vec<u32>>> {
        let keys: Rc<RefCell<Vec<u32>>> = Rc::default();
        e.register_double(MAP_RESERVE, |e, a| {
            e.mem.set_i32(a[0] + 8, 7);
            Ret::default()
        });
        let inserted = keys.clone();
        e.register_double(MAP_INSERT, move |_, a| {
            inserted.borrow_mut().push(a[2]);
            Ret::default()
        });
        let known = keys.clone();
        e.register_double(MAP_FIND_KEY, move |_, a| {
            ret(if known.borrow().contains(&a[1]) { 0 } else { 8 })
        });
        quiet(e, &[MAP_CLEAR_AND_DEALLOCATE, MAP_DESTRUCTOR]);
        keys
    }

    #[test]
    fn engine_registers_every_function_of_the_unit() {
        let e = Engine::new();
        for (address, _) in funcs() {
            assert!(e.is_translated(address), "{address:08x}");
        }
    }

    #[test]
    fn bc20_marks_the_event_and_stores_whether_bit_three_is_clear() {
        let mut e = engine();
        let event = e.mem.alloc(0x70);
        let result = e.mem.alloc(8);
        let seen = Rc::new(RefCell::new(vec![]));
        let seen_in = seen.clone();
        e.register_double(0x0600_0044, move |e, a| {
            seen_in.borrow_mut().push(a.to_vec());
            // The manager clears or sets bit 3 of the flags.
            let flags = e.mem.u8(a[1] + EVENT_FLAGS);
            e.mem.set_u8(a[1] + EVENT_FLAGS, flags | 0x08);
            Ret::default()
        });
        let manager = virtual_object(&mut e, &[(0x44, 0x0600_0044)]);
        e.mem.set_u32(event + EVENT_CONTACT_MANAGER, manager);
        e.mem.set_u8(event + EVENT_FLAGS, 0x20);
        let r = e.call(0x00d0_bc20, &args![result, event, 0x1234u32]).u32();
        assert_eq!(r, result);
        // Bit 0 was set before the call, the manager set bit 3.
        assert_eq!(*seen.borrow(), [vec![manager, event, 0x1234]]);
        assert_eq!(e.mem.u8(event + EVENT_FLAGS), 0x29);
        assert_eq!(e.mem.u8(result), 0);
        // Without the manager touching bit 3 the stored byte is 1.
        e.register_double(0x0600_0044, |_, _| Ret::default());
        e.mem.set_u8(event + EVENT_FLAGS, 0);
        e.call(0x00d0_bc20, &args![result, event, 0u32]);
        assert_eq!(e.mem.u8(result), 1);
        assert_eq!(e.mem.u8(event + EVENT_FLAGS), 1);
    }

    #[test]
    fn bc50_and_bcf0_do_nothing() {
        let mut e = engine();
        let this = simulation(&mut e, 0x0400_1800);
        let before = e.mem.bytes(this.addr(), 0x50);
        log(&mut e);
        e.call(0x00d0_bc50, &args![this, 1u32, 2u32, 3u32]);
        e.call(0x00d0_bcf0, &args![this, 1u32]);
        assert!(take_log(&mut e).is_empty());
        assert_eq!(e.mem.bytes(this.addr(), 0x50), before);
    }

    #[test]
    fn warp_time_adds_the_delta_to_the_events_of_the_worlds_simulation() {
        let mut e = engine();
        let world = world(&mut e);
        let real = simulation(&mut e, world);
        events(&mut e, real, 3);
        for (i, time) in [1.0f32, 2.5, -4.0].iter().enumerate() {
            e.mem
                .set_f32(event_at(&e, real, i as i32) + EVENT_TIME, *time);
        }
        // `this` is another object: the events are read through the world.
        let this: Ptr<HkpContinuousSimulation> = e.new_object();
        e.set(this, HkpContinuousSimulation::world, Ptr::new(world));
        e.call(0x00d0_bd00, &args![this, 0.5f32]);
        let times: Vec<f32> = (0..3)
            .map(|i| e.mem.f32(event_at(&e, real, i) + EVENT_TIME))
            .collect();
        assert_eq!(times, [1.5, 3.0, -3.5]);
    }

    #[test]
    fn are_velocities_ok_returns_the_last_element_that_raised_the_limit() {
        let mut e = engine();
        let dispatcher = e.mem.alloc(0x2000);
        let context = e.mem.alloc(8);
        e.mem.set_u32(context, dispatcher);
        let constraint = e.mem.alloc(0x40);
        e.mem.set_i8(constraint + INSTANCE_PRIORITY, 3);
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(info, constraint);
        e.mem.set_f32(info + 8, 1.5);
        e.mem.set_f32(dispatcher + (3 << 6) + 0x1c54, 1.0);
        let velocities = e.mem.alloc(0x40);
        for (i, v) in [1.0f32, -3.0, 2.0, 5.0, f32::NAN].iter().enumerate() {
            e.mem.set_f32(velocities + 4 * i as u32, *v);
        }
        // Limit 1.5; scaled |v| * 2: 2 raises it, 6 raises it, 4 does not,
        // 10 raises it, NaN never does.
        let worst = e
            .call(0x00d0_bd30, &args![2.0f32, info, velocities, 5i32, context])
            .i32();
        assert_eq!(worst, 3);
        let none = e
            .call(
                0x00d0_bd30,
                &args![0.25f32, info, velocities, 5i32, context],
            )
            .i32();
        assert_eq!(none, -1);
        assert_eq!(
            e.call(0x00d0_bd30, &args![2.0f32, info, velocities, 0i32, context])
                .i32(),
            -1
        );
    }

    #[test]
    fn be80_moves_records_touching_fixed_or_keyframed_bodies_to_the_front() {
        let mut e = engine();
        let moving = body(&mut e, 1, 1, 0, 0);
        let keyframed = body(&mut e, 4, 2, 0, 0);
        let fixed = body(&mut e, 5, 3, 0, 0);
        let instances = [
            instance(&mut e, moving, moving, 1),
            instance(&mut e, keyframed, moving, 1),
            instance(&mut e, moving, moving, 1),
            instance(&mut e, moving, fixed, 1),
        ];
        let array = record_array(&mut e, &instances, 4);
        let counter = e.mem.alloc(4);
        e.call(0x00d0_be80, &args![array, counter, 4i32]);
        assert_eq!(
            record_instances(&e, array),
            [instances[1], instances[3], instances[2], instances[0]]
        );
        assert_eq!(e.mem.u32(counter), 2);
        // The other two words of a record travel with it.
        let data = e.mem.u32(array);
        assert_eq!((e.mem.u32(data + 4), e.mem.u32(data + 8)), (0x101, 0x201));
        assert_eq!(
            (e.mem.u32(data + 12 + 4), e.mem.u32(data + 12 + 8)),
            (0x103, 0x203)
        );
    }

    #[test]
    fn be80_only_scans_the_last_n_records() {
        let mut e = engine();
        let moving = body(&mut e, 1, 1, 0, 0);
        let fixed = body(&mut e, 5, 3, 0, 0);
        let instances = [
            instance(&mut e, fixed, moving, 1),
            instance(&mut e, moving, moving, 1),
            instance(&mut e, moving, fixed, 1),
        ];
        let array = record_array(&mut e, &instances, 3);
        let counter = e.mem.alloc(4);
        e.mem.set_u32(counter, 1);
        e.call(0x00d0_be80, &args![array, counter, 2i32]);
        // Record 0 is outside the scan; record 2 swaps with record 1.
        assert_eq!(
            record_instances(&e, array),
            [instances[0], instances[2], instances[1]]
        );
        assert_eq!(e.mem.u32(counter), 2);
    }

    #[test]
    fn bf30_resets_the_accumulators_from_the_bodies_velocities() {
        let mut e = engine();
        let accumulators = e.mem.alloc(0x400);
        let table = e.mem.alloc(0x400);
        let context = e.mem.alloc(0x40);
        e.mem.set_u32(context + 0xc, accumulators);
        e.mem.set_u32(context + 0x18, table);
        let solver_data = 0x80u32;
        let accumulator = accumulators + solver_data;
        let body = body(&mut e, 1, 1, 0, 0);
        e.mem.set_u32(body + ENTITY_SOLVER_DATA, solver_data);
        e.mem.write(body + ENTITY_LINEAR_VELOCITY, &[7; 16]);
        let source = table + ((solver_data >> 7) << 5) + 0x10;
        e.mem.write(source, &[9; 16]);
        e.mem.write(accumulator + 0x50, &[5; 16]);
        let entities = word_array(&mut e, &[body], 1);
        let during = Rc::new(RefCell::new(None));
        let during_in = during.clone();
        e.register_double(SET_ROTATED_DIR, move |e, a| {
            *during_in.borrow_mut() = Some((a.to_vec(), e.mem.bytes(a[1], 16)));
            Ret::default()
        });
        e.call(0x00d0_bf30, &args![context, entities]);
        assert_eq!(e.mem.bytes(accumulator + 0x10, 16), [7; 16]);
        // During the rotation call the +0x50 vector was the table's.
        let (args_seen, vector) = during.borrow_mut().take().unwrap();
        assert_eq!(
            args_seen,
            [
                accumulator + 0x20,
                accumulator + 0x50,
                body + ENTITY_ANGULAR_VELOCITY
            ]
        );
        assert_eq!(vector, [9; 16]);
        // Afterwards the saved vector is back.
        assert_eq!(e.mem.bytes(accumulator + 0x50, 16), [5; 16]);
    }

    #[test]
    fn bfc0_drops_the_events_of_the_agents_contact_manager() {
        let mut e = engine();
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        events(&mut e, this, 5);
        let (m, x, y) = (0x7001u32, 0x7002u32, 0x7003u32);
        for (i, manager) in [m, x, m, y, m].iter().enumerate() {
            let event = event_at(&e, this, i as i32);
            set_event(&mut e, event, i as f32, 0, 0, *manager);
        }
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + ENTRY_CONTACT_MANAGER, m);
        e.call(0x00d0_bfc0, &args![this, entry]);
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 2);
        let remaining: Vec<(u32, f32)> = (0..2)
            .map(|i| {
                let event = event_at(&e, this, i);
                (
                    e.mem.u32(event + EVENT_CONTACT_MANAGER),
                    e.mem.f32(event + EVENT_TIME),
                )
            })
            .collect();
        assert_eq!(remaining, [(y, 3.0), (x, 1.0)]);
    }

    #[test]
    fn c030_gathers_boxes_and_handles_for_the_broad_phase_and_removes_duplicates() {
        let mut e = engine();
        let world = world(&mut e);
        let broad_phase = virtual_object(&mut e, &[(0x2c, 0x0600_002c)]);
        e.mem.set_u32(broad_phase + 0x10, 1);
        e.mem.set_u32(world + WORLD_BROAD_PHASE, broad_phase);
        let first = bounded_body(&mut e, [10, 20, 30], [50, 60, 70]);
        // The second body's minimum x is above its maximum x.
        let second = bounded_body(&mut e, [100, 20, 30], [5, 60, 70]);
        let entities = word_array(&mut e, &[first, second], 2);
        let added = word_array(&mut e, &[], 4);
        let removed = word_array(&mut e, &[], 4);
        quiet(
            &mut e,
            &[BROAD_PHASE_LOCK, BROAD_PHASE_UNLOCK, REMOVE_DUPLICATES],
        );
        e.register_double(ENTITY_BATCH_RECALC_AABB, |e, a| {
            assert_eq!(a[2], 1);
            let body = e.mem.u32(a[1]);
            for (axis, (low, high)) in [(11u32, 9u32), (22, 9), (33, 9)].iter().enumerate() {
                e.mem.set_u32(body + 0x30 + 4 * axis as u32, *low);
                e.mem.set_u32(body + 0x40 + 4 * axis as u32, *high);
            }
            Ret::default()
        });
        let captured = Rc::new(RefCell::new((vec![], vec![])));
        let captured_in = captured.clone();
        e.register_double(0x0600_002c, move |e, a| {
            *captured_in.borrow_mut() = (e.mem.bytes(a[1], 8), e.mem.bytes(a[2], 64));
            assert_eq!(a[3], 2);
            // One pair was found.
            e.mem.set_u32(a[4] + 4, 1);
            Ret::default()
        });
        let list = e.mem.u32(entities);
        log(&mut e);
        e.call(0x00d0_c030, &args![list, 2i32, world, added, removed]);
        let (handles, boxes) = captured.borrow().clone();
        let words = |bytes: &[u8]| -> Vec<u32> {
            bytes
                .chunks(4)
                .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect()
        };
        assert_eq!(words(&handles), [first + 0x24, second + 0x24]);
        // Minimum = low - (offset << 1), maximum = high + (offset << 1).
        let expected_first = [8, 16, 24, 0, 52, 62, 72, 0];
        let boxes = words(&boxes);
        assert_eq!(boxes[..3], expected_first[..3]);
        assert_eq!(boxes[4..7], expected_first[4..7]);
        assert_eq!(boxes[8..11], [9, 18, 27]);
        assert_eq!(boxes[12..15], [11, 11, 11]);
        let calls = take_log(&mut e);
        assert_eq!(
            only(
                &calls,
                &[
                    ENTITY_BATCH_RECALC_AABB,
                    BROAD_PHASE_LOCK,
                    0x0600_002c,
                    BROAD_PHASE_UNLOCK,
                    REMOVE_DUPLICATES
                ]
            ),
            [
                ENTITY_BATCH_RECALC_AABB,
                BROAD_PHASE_LOCK,
                0x0600_002c,
                BROAD_PHASE_UNLOCK,
                REMOVE_DUPLICATES
            ]
        );
        assert_eq!(calls_to(&calls, REMOVE_DUPLICATES), [vec![added, removed]]);
        // The stack memory is back and the five records were written.
        assert_eq!(e.mem.u32(ROUTER_BLOCK), STACK_BASE);
        assert_eq!(e.mem.u32(STREAM + 4), STREAM_RECORDS + 0x34);
        assert_eq!(e.mem.u32(STREAM_RECORDS), PROFILE_LT_BROAD_PHASE);
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0xc), PROFILE_ST_GATHER_AABBS);
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0x10), PROFILE_ST_3AXIS_SWEEP);
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0x1c), PROFILE_ST_REMOVE_DUP);
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0x28), PROFILE_LT);
    }

    #[test]
    fn c030_does_nothing_for_no_bodies() {
        let mut e = engine();
        log(&mut e);
        e.call(0x00d0_c030, &args![0u32, 0i32, 0u32, 0u32, 0u32]);
        assert!(take_log(&mut e).is_empty());
        assert_eq!(e.mem.u32(STREAM + 4), STREAM_RECORDS);
    }

    #[test]
    fn c290_cancels_the_events_of_the_listed_bodies() {
        let mut e = engine();
        let keys = map_doubles(&mut e);
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        let seen = Seen::default();
        let a = body(&mut e, 1, 1, 0, 0x111);
        let b = body(&mut e, 1, 2, 0, 0x222);
        let c = body(&mut e, 5, 3, 0, 0x333);
        let d = body(&mut e, 1, 4, 0, 0x444);
        let f = body(&mut e, 1, 5, 0, 0x555);
        let data = events(&mut e, this, 3);
        let managers: Vec<u32> = (0..3)
            .map(|_| reporting_manager(&mut e, 0x0600_0020, seen.clone()))
            .collect();
        // The first body of the first event has motion type 5: the island of
        // its second body is reported.
        set_event(&mut e, data, 1.0, c, a, managers[0]);
        set_event(&mut e, data + TOI_EVENT_SIZE, 2.0, d, f, managers[1]);
        set_event(&mut e, data + 2 * TOI_EVENT_SIZE, 3.0, f, b, managers[2]);
        let entities = word_array(&mut e, &[a, b], 2);
        let list = e.mem.u32(entities);
        log(&mut e);
        // The map sits at +0x10 of the exe's 0x1c-byte frame.
        let frame = e.next_stack_local(0x1c);
        e.call(0x00d0_c290, &args![this, list, 2i32]);
        let calls = take_log(&mut e);
        assert_eq!(*keys.borrow(), [a, b]);
        assert_eq!(calls_to(&calls, MAP_RESERVE).len(), 1);
        assert_eq!(calls_to(&calls, MAP_RESERVE)[0][0], frame + 0x10);
        assert_eq!(calls_to(&calls, MAP_RESERVE)[0][2], 2);
        assert_eq!(calls_to(&calls, MAP_CLEAR_AND_DEALLOCATE).len(), 1);
        assert_eq!(calls_to(&calls, MAP_DESTRUCTOR).len(), 1);
        assert_eq!(
            *seen.borrow(),
            [
                vec![managers[0], 0x111, data + EVENT_PROPERTIES],
                vec![managers[2], 0x555, data + EVENT_PROPERTIES],
            ]
        );
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 1);
        assert_eq!(e.mem.u32(data + EVENT_ENTITY_A), d);
        assert_eq!(e.mem.u32(data + EVENT_ENTITY_B), f);
    }

    #[test]
    fn c290_does_nothing_without_events() {
        let mut e = engine();
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        log(&mut e);
        e.call(0x00d0_c290, &args![this, 0x1000u32, 3i32]);
        assert!(take_log(&mut e).is_empty());
    }

    #[test]
    fn c3e0_cancels_the_events_of_one_body() {
        let mut e = engine();
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        let seen = Seen::default();
        let a = body(&mut e, 1, 1, 0, 0x111);
        let b = body(&mut e, 1, 2, 0, 0x222);
        let d = body(&mut e, 1, 4, 0, 0x444);
        let f = body(&mut e, 1, 5, 0, 0x555);
        let data = events(&mut e, this, 3);
        let managers: Vec<u32> = (0..3)
            .map(|_| reporting_manager(&mut e, 0x0600_0020, seen.clone()))
            .collect();
        set_event(&mut e, data, 1.0, d, f, managers[0]);
        set_event(&mut e, data + TOI_EVENT_SIZE, 2.0, a, b, managers[1]);
        set_event(&mut e, data + 2 * TOI_EVENT_SIZE, 3.0, f, a, managers[2]);
        e.call(0x00d0_c3e0, &args![this, f]);
        assert_eq!(
            *seen.borrow(),
            [
                vec![managers[0], 0x444, data + EVENT_PROPERTIES],
                vec![managers[2], 0x555, data + EVENT_PROPERTIES],
            ]
        );
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 1);
        assert_eq!(e.mem.u32(data + EVENT_ENTITY_A), a);
        assert_eq!(e.mem.f32(data + EVENT_TIME), 2.0);
    }

    #[test]
    fn c480_hands_the_contact_to_the_response_and_collects_the_bodies_to_wake() {
        let mut e = engine();
        // Settings sources.
        e.mem.set_f32(FLOAT_TABLE + 4 * 3, 0.5);
        e.mem.set_f32(FLOAT_TABLE + 4 * 5, 0.25);
        e.set_global(SIMPLE_RESPONSE_SCALE, 0.0078125f64);
        e.set_global(SIMPLE_RESPONSE_TOLERANCE, 0.1f32);
        e.set_global(SIMPLE_RESPONSE_DEFAULT, 7.0f32);
        let scenario = Rc::new(Cell::new((0.0f32, 0.0f32, 0u8)));
        let flag = Rc::new(Cell::new(0u8));
        let settings = Rc::new(RefCell::new(vec![]));
        let response_args = Rc::new(RefCell::new(vec![]));
        {
            let scenario = scenario.clone();
            let settings = settings.clone();
            let response_args = response_args.clone();
            e.register_double(SIMPLE_COLLISION_RESPONSE, move |e, a| {
                let (first, second, breach) = scenario.get();
                e.mem.set_f32(a[6] + 4, first);
                e.mem.set_f32(a[6] + 8, second);
                e.mem.set_u8(a[6] + 0x38, breach);
                *settings.borrow_mut() = e.mem.bytes(a[2], 24);
                *response_args.borrow_mut() = a.to_vec();
                Ret::default()
            });
        }
        let flag_in = flag.clone();
        e.register_double(FIXED_OR_KEYFRAMED_BYTE, move |e, a| {
            e.mem.set_u8(a[1], flag_in.get());
            ret(a[1])
        });
        let reports = Rc::new(RefCell::new(vec![]));
        let reports_in = reports.clone();
        e.register_double(FIRE_CONTACT_IMPULSE_LIMIT_BREACHED, move |e, a| {
            reports_in
                .borrow_mut()
                .push((a.to_vec(), e.mem.bytes(a[1], 16)));
            Ret::default()
        });
        let body_a = body(&mut e, 1, 1, 0, 0);
        let body_b = body(&mut e, 1, 2, 0, 0);
        let manager = e.mem.alloc(0x80);
        let event = e.mem.alloc(TOI_EVENT_SIZE);
        set_event(&mut e, event, 0.75, body_a, body_b, manager);
        e.mem.set_f32(event + EVENT_SEPARATING_VELOCITY, -2.0);
        e.mem.set_u8(event + 0x24, 3);
        e.mem.set_u8(event + 0x25, 4);
        e.mem.set_u8(event + 0x26, 5);
        let output = word_array(&mut e, &[], 4);
        let run = |e: &mut Engine, sc: (f32, f32, u8), types: (u8, u8), byte: u8| {
            scenario.set(sc);
            flag.set(byte);
            e.mem.set_u8(body_a + ENTITY_MOTION_TYPE, types.0);
            e.mem.set_u8(body_b + ENTITY_MOTION_TYPE, types.1);
            e.mem.set_u32(output + 4, 0);
            e.call(0x00d0_c480, &args![0x4242u32, event, 9.0f32, output]);
            array_items(e, output)
        };
        // Both values negative: every movable body.
        assert_eq!(run(&mut e, (-1.0, -1.0, 0), (1, 1), 0), [body_a, body_b]);
        // The settings and the response arguments.
        let floats: Vec<f32> = settings
            .borrow()
            .chunks(4)
            .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect();
        assert_eq!(floats, [0.5, 0.03125, 0.25, -2.0, 0.1, 9.0]);
        let response = response_args.borrow().clone();
        assert_eq!(response[0], event + EVENT_CONTACT_POINT);
        assert_eq!(f32::from_bits(response[1]), 0.75);
        assert_eq!(response[3], body_a + ENTITY_MOTION);
        assert_eq!(response[4], body_b + ENTITY_MOTION);
        assert_eq!(response[5], manager);
        // A zero selector byte takes the default float instead of the table.
        e.mem.set_u8(event + 0x26, 0);
        run(&mut e, (-1.0, -1.0, 0), (1, 1), 0);
        let floats: Vec<f32> = settings
            .borrow()
            .chunks(4)
            .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect();
        assert_eq!(floats[2], 7.0);
        e.mem.set_u8(event + 0x26, 5);
        // The first value is negative, the second not: the first body when
        // it is movable, else the second.
        assert_eq!(run(&mut e, (-1.0, 0.0, 0), (1, 1), 0), [body_a]);
        assert_eq!(run(&mut e, (-1.0, 0.0, 0), (5, 1), 0), [body_b]);
        // The first value is not negative, the second is: the first body
        // when the flag of the second body is set, else the second.
        assert_eq!(run(&mut e, (1.0, -1.0, 0), (1, 1), 1), [body_a]);
        assert_eq!(run(&mut e, (1.0, -1.0, 0), (1, 1), 0), [body_b]);
        // Everything else: a fixed first body leaves the second, a fixed
        // second body the first, else the second when it is the smaller.
        assert_eq!(run(&mut e, (2.0, 1.0, 0), (1, 1), 0), [body_b]);
        assert_eq!(run(&mut e, (1.0, 2.0, 0), (1, 1), 0), [body_a]);
        assert_eq!(run(&mut e, (2.0, 1.0, 0), (4, 1), 0), [body_b]);
        assert_eq!(run(&mut e, (1.0, 2.0, 0), (1, 5), 0), [body_a]);
        // A NaN first value is neither below nor above zero.
        assert_eq!(run(&mut e, (f32::NAN, 1.0, 0), (1, 1), 0), [body_a]);
        assert_eq!(run(&mut e, (1.0, f32::NAN, 0), (1, 1), 0), [body_a]);
        assert!(reports.borrow().is_empty());
        // A breach is reported and nothing is collected.
        assert_eq!(run(&mut e, (-1.0, -1.0, 1), (1, 1), 0), Vec::<u32>::new());
        let reports = reports.borrow();
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].0[0], 0x4242);
        assert_eq!(reports[0].0[2], 1);
        let mut expected = vec![2u8, 0, 1, 0];
        for word in [manager + 0x44, event + 0x18, event + 0x50] {
            expected.extend_from_slice(&word.to_le_bytes());
        }
        assert_eq!(reports[0].1, expected);
    }

    #[test]
    fn c6d0_moves_the_records_whose_bodies_are_both_at_state_0x10() {
        let mut e = engine();
        let bodies: Vec<u32> = (0..4)
            .map(|i| body(&mut e, 1, i + 1, i as u16, 0))
            .collect();
        let table = e.mem.alloc(8);
        for (i, state) in [0x10u8, 0x10, 0x10, 0].iter().enumerate() {
            e.mem.set_u8(table + i as u32, *state);
        }
        let flags = e.mem.alloc(4);
        e.mem.set_u32(flags, table);
        let instances = [
            instance(&mut e, bodies[0], bodies[3], 1),
            instance(&mut e, bodies[0], bodies[1], 1),
            instance(&mut e, bodies[1], bodies[2], 1),
        ];
        let array = record_array(&mut e, &instances, 3);
        let counter = e.mem.alloc(4);
        e.call(0x00d0_c6d0, &args![array, counter, flags]);
        assert_eq!(
            record_instances(&e, array),
            [instances[1], instances[2], instances[0]]
        );
        assert_eq!(e.mem.u32(counter), 2);
    }

    #[test]
    fn c790_rebuilds_the_translation_and_copies_the_velocities_back() {
        let mut e = engine();
        let accumulators = e.mem.alloc(0x400);
        let table = e.mem.alloc(0x400);
        let context = e.mem.alloc(0x40);
        e.mem.set_u32(context + 0xc, accumulators);
        e.mem.set_u32(context + 0x18, table);
        let rotated = Rc::new(RefCell::new(vec![]));
        let rotated_in = rotated.clone();
        e.register_double(SET_ROTATION, move |e, a| {
            rotated_in.borrow_mut().push(a.to_vec());
            // The identity rotation.
            write_vec4(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            write_vec4(e, a[0] + 0x10, [0.0, 1.0, 0.0, 0.0]);
            write_vec4(e, a[0] + 0x20, [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        let island = e.mem.alloc(0x80);
        let first = body(&mut e, 1, 1, 0, island);
        let second = body(&mut e, 1, 2, 1, island);
        let third = body(&mut e, 1, 3, 2, island);
        let bodies = word_array(&mut e, &[first, second, third], 3);
        e.mem
            .set_u32(island + ISLAND_ENTITY_DATA, e.mem.u32(bodies));
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, 3);
        let states = e.mem.alloc(8);
        e.mem.write(states, &[1, 8, 0]);
        let flags = e.mem.alloc(4);
        e.mem.set_u32(flags, states);
        // Body 0 (state 1): translation = base - rotation * vector.
        let motion_state = first + ENTITY_MOTION_STATE;
        write_vec4(&mut e, motion_state + 0x80, [1.0, 2.0, 3.0, 0.0]);
        write_vec4(&mut e, motion_state + 0x50, [10.0, 20.0, 30.0, 40.0]);
        // Body 1 (state 8): the velocities come from its accumulator.
        e.mem.set_u32(second + ENTITY_SOLVER_DATA, 0x80);
        let accumulator = accumulators + 0x80;
        write_vec4(&mut e, accumulator + 0x10, [1.5, 2.5, 3.5, 4.5]);
        write_vec4(&mut e, accumulator + 0x20, [1.0, 1.0, 1.0, 0.0]);
        write_vec4(&mut e, table + 0x20 + 0x10, [1.0, 2.0, 3.0, 0.0]);
        write_vec4(&mut e, accumulator + 0x60, [4.0, 5.0, 6.0, 0.0]);
        write_vec4(&mut e, accumulator + 0x70, [7.0, 8.0, 9.0, 10.0]);
        e.call(0x00d0_c790, &args![island, flags, context]);
        assert_eq!(*rotated.borrow(), [vec![motion_state, motion_state + 0x70]]);
        assert_eq!(read_vec4(&e, motion_state + 0x30), [9.0, 18.0, 27.0, 40.0]);
        assert_eq!(
            read_vec4(&e, second + ENTITY_LINEAR_VELOCITY),
            [1.5, 2.5, 3.5, 4.5]
        );
        assert_eq!(read_vec4(&e, accumulator + 0x50), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(
            read_vec4(&e, second + ENTITY_ANGULAR_VELOCITY),
            [6.0, 15.0, 24.0, 28.0]
        );
        // The third body (state 0) is untouched.
        assert_eq!(read_vec4(&e, third + ENTITY_LINEAR_VELOCITY), [0.0; 4]);
    }

    #[test]
    fn c8d0_reports_the_memory_of_the_simulation() {
        let mut e = engine();
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        let data = events(&mut e, this, 2);
        // A heap-allocated event array: capacity word without bit 31.
        e.set(this, HkpContinuousSimulation::event_capacity, 4);
        let manager = e.mem.alloc(0x20);
        e.set(
            this,
            HkpContinuousSimulation::toi_resource_manager,
            Ptr::new(manager),
        );
        let map_data = e.mem.alloc(0x40);
        e.mem.set_u32(this.addr() + SIM_ENTITIES_MAP, map_data);
        e.register_double(ALLOCATED_SIZE, |_, a| ret(a[1] + 1));
        e.register_double(MAP_GET_MEM_SIZE, |_, a| {
            assert_eq!(a.len(), 1);
            ret(0x40)
        });
        quiet(
            &mut e,
            &[CALC_CONTENT_STATISTICS_BASE, 0x0600_000c, 0x0600_0014],
        );
        let collector = virtual_object(&mut e, &[(0xc, 0x0600_000c), (0x14, 0x0600_0014)]);
        log(&mut e);
        e.call(0x00d0_c8d0, &args![this, collector, 0x77u32]);
        let log_1 = take_log(&mut e);
        assert_eq!(
            calls_to(&log_1, 0x0600_0014),
            [
                vec![
                    collector,
                    3,
                    STATISTICS_NAME_TOI_EVENTS,
                    data,
                    2 * TOI_EVENT_SIZE,
                    4 * TOI_EVENT_SIZE + 1
                ],
                vec![
                    collector,
                    1,
                    STATISTICS_NAME_ENTITIES_MAP,
                    map_data,
                    0x40,
                    0x41
                ],
            ]
        );
        assert_eq!(
            calls_to(&log_1, 0x0600_000c),
            [vec![
                collector,
                STATISTICS_NAME_TOI_RESOURCE_MANAGER,
                manager,
                0
            ]]
        );
        assert_eq!(
            calls_to(&log_1, CALC_CONTENT_STATISTICS_BASE),
            [vec![this.addr(), collector, STATISTICS_BASE_ARGUMENT]]
        );
        // An event array that is not heap-allocated is skipped unless the
        // collector asks for everything.
        e.set(this, HkpContinuousSimulation::event_capacity, 0x8000_0004);
        log(&mut e);
        e.call(0x00d0_c8d0, &args![this, collector, 0x77u32]);
        assert_eq!(calls_to(&take_log(&mut e), 0x0600_0014).len(), 1);
        e.mem.set_u8(collector + 0x10, 1);
        log(&mut e);
        e.call(0x00d0_c8d0, &args![this, collector, 0x77u32]);
        assert_eq!(calls_to(&take_log(&mut e), 0x0600_0014).len(), 2);
    }

    #[test]
    fn c970_resets_the_base_information_and_cancels_events_when_asked() {
        let mut e = engine();
        let keys = map_doubles(&mut e);
        quiet(&mut e, &[BASE_RESET_COLLISION_INFORMATION]);
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        let seen = Seen::default();
        let a = body(&mut e, 1, 1, 0, 0x111);
        let b = body(&mut e, 1, 2, 0, 0x222);
        let data = events(&mut e, this, 2);
        let manager = reporting_manager(&mut e, 0x0600_0020, seen.clone());
        set_event(&mut e, data, 1.0, a, b, manager);
        set_event(&mut e, data + TOI_EVENT_SIZE, 2.0, b, b, manager);
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, a);
        e.mem.set_u32(list + 4, b);
        // Bit 0 clear: only the base function runs.
        log(&mut e);
        e.call(0x00d0_c970, &args![this, list, 2i32, 0x55u32, 0u32]);
        let log_1 = take_log(&mut e);
        assert_eq!(
            log_1,
            vec![(
                BASE_RESET_COLLISION_INFORMATION,
                vec![this.addr(), list, 2, 0x55, 0]
            )]
        );
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 2);
        // One body: its events go through the single-body function.
        log(&mut e);
        e.call(0x00d0_c970, &args![this, list, 1i32, 0x55u32, 1u32]);
        let log_2 = take_log(&mut e);
        assert!(calls_to(&log_2, MAP_RESERVE).is_empty());
        assert_eq!(
            calls_to(&log_2, BASE_RESET_COLLISION_INFORMATION),
            [vec![this.addr(), list, 1, 0x55, 1]]
        );
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 1);
        // Two bodies: the map function.
        e.set(this, HkpContinuousSimulation::event_count, 2);
        set_event(&mut e, data, 1.0, a, b, manager);
        set_event(&mut e, data + TOI_EVENT_SIZE, 2.0, a, a, manager);
        log(&mut e);
        e.call(0x00d0_c970, &args![this, list, 2i32, 0x55u32, 1u32]);
        let log_3 = take_log(&mut e);
        assert_eq!(calls_to(&log_3, MAP_RESERVE).len(), 1);
        assert_eq!(*keys.borrow(), [a, b]);
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 0);
        assert_eq!(seen.borrow().len(), 3);
    }

    #[test]
    fn c9c0_runs_the_narrow_phase_on_each_entry_and_reports_contacts() {
        let mut e = engine();
        e.set_global(TOI_TIME_NOT_SET, 123.0f32);
        let dispatcher = e.mem.alloc(0x2000);
        let input = e.mem.alloc(0x100);
        e.mem.set_u32(input, dispatcher);
        // Creation words per quality block.
        e.mem.set_u32(dispatcher + 0x1c30 + 0x80, 0xaaaa);
        e.mem.set_u32(dispatcher + 0x1c30 - 0x40, 0xbbbb);
        let seen = Rc::new(RefCell::new(vec![]));
        let contact_seen = seen.clone();
        e.register_double(0x0600_0018, move |_, a| {
            contact_seen.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let manager_a = virtual_object(&mut e, &[(0x18, 0x0600_0018)]);
        let manager_b = virtual_object(&mut e, &[(0x18, 0x0600_0018)]);
        let entry_a = e.mem.alloc(0x20);
        let entry_b = e.mem.alloc(0x20);
        for (entry, manager, quality) in [(entry_a, manager_a, 2i8), (entry_b, manager_b, -1)] {
            e.mem.set_u32(entry + ENTRY_CONTACT_MANAGER, manager);
            e.mem.set_i8(entry + ENTRY_QUALITY_INDEX, quality);
            e.mem.set_u32(entry + ENTRY_COLLIDABLE_A, entry + 0x100);
            e.mem.set_u32(entry + ENTRY_COLLIDABLE_B, entry + 0x200);
        }
        let processed = Rc::new(RefCell::new(vec![]));
        let processed_in = processed.clone();
        e.register_double(PROCESS_AGENT, move |e, a| {
            let output = a[2];
            processed_in.borrow_mut().push((
                a.to_vec(),
                e.mem.u32(a[1] + 0x60),
                e.mem.u32(a[1] + 0x14),
                e.mem.u32(output + 4),
                e.mem.u32(output + OUTPUT_TOI_TIME),
                e.mem.u32(output),
                output,
            ));
            // The second entry produces a contact: the pointer moves.
            if e.mem.i8(a[0] + ENTRY_QUALITY_INDEX) == -1 {
                e.mem.set_u32(output, output + 0x20);
            }
            Ret::default()
        });
        let entries = word_array(&mut e, &[entry_a, entry_b], 2);
        let list = e.mem.u32(entries);
        e.call(
            0x00d0_c9c0,
            &args![VCALL_THIS, list, 2i32, input, 0x9999u32, 0u32],
        );
        let processed = processed.borrow().clone();
        assert_eq!(processed.len(), 2);
        let (first_args, quality_info, creation, owner, time, pointer, output) =
            processed[0].clone();
        assert_eq!(first_args[0], entry_a);
        assert_eq!(first_args[1], input);
        assert_eq!(first_args[3], manager_a);
        assert_eq!(quality_info, dispatcher + 0x1c20 + 0x80);
        assert_eq!(creation, 0xaaaa);
        assert_eq!(owner, 0x9999);
        assert_eq!(f32::from_bits(time), 123.0);
        assert_eq!(pointer, output + 0x10);
        let (_, quality_info, creation, _, _, _, _) = processed[1].clone();
        assert_eq!(quality_info, dispatcher + 0x1c20 - 0x40);
        assert_eq!(creation, 0xbbbb);
        // Only the second entry produced a contact.
        assert_eq!(
            *seen.borrow(),
            [vec![
                manager_b,
                entry_b + 0x100,
                entry_b + 0x200,
                input,
                output
            ]]
        );
        // The profile records: start and end.
        assert_eq!(e.mem.u32(STREAM_RECORDS), PROFILE_TT_NARROW_PHASE_TOI);
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0xc), PROFILE_END);
        // With the time-of-impact quality flag the block at 0x1de0 is used.
        e.call(
            0x00d0_c9c0,
            &args![VCALL_THIS, list, 1i32, input, 0x9999u32, 1u32],
        );
        assert_eq!(e.mem.u32(input + 0x60), dispatcher + 0x1de0);
    }

    /// What the accumulator of `check_validity_of_constraints` looked like
    /// while the velocities were asked for.
    type Accumulator = ([f32; 4], [f32; 4]);

    #[test]
    fn cb00_flags_the_bodies_of_violated_constraints_and_restores_the_accumulators() {
        let mut e = engine();
        let accumulators = e.mem.alloc(0x400);
        let dispatcher = e.mem.alloc(0x2000);
        e.mem.set_f32(dispatcher + (3 << 6) + 0x1c54, 1.0);
        let conversion = e.mem.alloc(4);
        e.mem.set_u32(conversion, dispatcher);
        let scale_block = e.mem.alloc(0x10);
        e.mem.set_f32(scale_block + 8, 1.0);
        let context = e.mem.alloc(0x40);
        e.mem.set_u32(context, scale_block);
        e.mem.set_u32(context + 4, 0x77);
        e.mem.set_u32(context + 0xc, accumulators);
        let bodies: Vec<u32> = (0..4)
            .map(|i| body(&mut e, 1, i + 1, i as u16, 0))
            .collect();
        let (x, y, z, w) = (bodies[0], bodies[1], bodies[2], bodies[3]);
        // The displacement of x over the step.
        e.mem.set_u32(x + ENTITY_SOLVER_DATA, 0x80);
        write_vec4(&mut e, x + ENTITY_CENTER_OF_MASS_1, [2.0, 4.0, 6.0, 0.0]);
        e.mem.set_f32(x + ENTITY_INV_DELTA_TIME, 0.5);
        write_vec4(&mut e, x + ENTITY_DELTA_ANGLE, [1.0, 1.0, 1.0, 0.0]);
        let accumulator = accumulators + 0x80;
        write_vec4(&mut e, accumulator + 0x10, [9.0; 4]);
        write_vec4(&mut e, accumulator + 0x20, [8.0; 4]);
        let table = e.mem.alloc(8);
        e.mem.write(table, &[0x10, 0, 4, 0]);
        let flags = e.mem.alloc(4);
        e.mem.set_u32(flags, table);
        let instances = [
            instance(&mut e, x, y, 3),
            instance(&mut e, x, y, 0),
            instance(&mut e, z, x, 3),
            instance(&mut e, x, y, 3),
            instance(&mut e, x, w, 3),
        ];
        let records = record_array(&mut e, &instances, 5);
        for i in 0..5 {
            e.mem.set_f32(e.mem.u32(records) + 12 * i + 8, 1.0);
        }
        let entities = word_array(&mut e, &[x], 1);
        let out = word_array(&mut e, &[], 1);
        let counter = e.mem.alloc(4);
        let during: Rc<RefCell<Vec<Accumulator>>> = Rc::default();
        let during_in = during.clone();
        e.register_double(GET_TOI_VIOLATING_VELOCITIES, move |e, a| {
            during_in.borrow_mut().push((
                read_vec4(e, accumulator + 0x10),
                read_vec4(e, accumulator + 0x20),
            ));
            assert_eq!((a[0], a[2], a[3]), (0x77, accumulators, 0x100));
            if a[1] == 0x100 {
                e.mem.set_f32(a[4], 0.5);
                e.mem.set_f32(a[4] + 4, 5.0);
                ret(2)
            } else {
                e.mem.set_f32(a[4], 0.1);
                ret(1)
            }
        });
        // Growing the output array: the capacity doubles.
        e.register_double(ARRAY_GROW, |e, a| {
            let (array, element) = (a[1], a[2]);
            let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
            let data = e.mem.alloc(element * capacity * 2);
            let old = e.mem.bytes(e.mem.u32(array), element * capacity);
            e.mem.write(data, &old);
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, capacity * 2);
            Ret::default()
        });
        e.call(
            0x00d0_cb00,
            &args![context, conversion, records, counter, 1i32, flags, entities, out],
        );
        // Only the two records that were asked got to the velocities.
        assert_eq!(during.borrow().len(), 2);
        assert_eq!(
            during.borrow()[0],
            ([1.0, 2.0, 3.0, 0.25], [0.5, 0.5, 0.5, 0.0])
        );
        // y is flagged (4) and went to the output once; w was fine.
        assert_eq!(e.mem.bytes(table, 4), [0x10, 4, 4, 0]);
        assert_eq!(array_items(&e, out), [y]);
        assert_eq!(
            record_instances(&e, records),
            [
                instances[0],
                instances[2],
                instances[3],
                instances[1],
                instances[4]
            ]
        );
        assert_eq!(e.mem.u32(counter), 3);
        // The accumulator has its old vectors back.
        assert_eq!(read_vec4(&e, accumulator + 0x10), [9.0; 4]);
        assert_eq!(read_vec4(&e, accumulator + 0x20), [8.0; 4]);
        assert_eq!(e.mem.u32(ROUTER_BLOCK), STACK_BASE);
    }

    #[test]
    fn cde0_backsteps_the_bodies_and_freezes_them_at_the_time() {
        let mut e = engine();
        let island = e.mem.alloc(0x80);
        let world = e.mem.alloc(0x20);
        e.mem.set_u32(island + 0x18, world);
        let first = body(&mut e, 1, 1, 0, island);
        let second = body(&mut e, 1, 2, 1, island);
        let bodies = word_array(&mut e, &[first, second], 2);
        e.mem
            .set_u32(island + ISLAND_ENTITY_DATA, e.mem.u32(bodies));
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, 2);
        let table = e.mem.alloc(4);
        e.mem.write(table, &[0, 0x10]);
        let flags = e.mem.alloc(4);
        e.mem.set_u32(flags, table);
        let added = word_array(&mut e, &[], 4);
        for (i, body) in [first, second].iter().enumerate() {
            let ms = body + ENTITY_MOTION_STATE;
            write_vec4(&mut e, ms + 0x50, [i as f32 + 1.0; 4]);
            write_vec4(&mut e, ms + 0x70, [i as f32 + 5.0; 4]);
        }
        e.register_double(WORLD_GET_CURRENT_PSI_TIME, |_, a| {
            assert_eq!(a.len(), 1);
            Ret {
                st0: 3.0,
                ..Ret::default()
            }
        });
        quiet(&mut e, &[BACK_STEP_MOTION_STATE]);
        log(&mut e);
        e.call(0x00d0_cde0, &args![1.0f32, island, flags, added]);
        let calls = take_log(&mut e);
        assert_eq!(
            calls_to(&calls, BACK_STEP_MOTION_STATE),
            [
                vec![1.0f32.to_bits(), first + ENTITY_MOTION_STATE],
                vec![1.0f32.to_bits(), second + ENTITY_MOTION_STATE]
            ]
        );
        // The first body was not yet at state 0x10.
        assert_eq!(array_items(&e, added), [first]);
        assert_eq!(e.mem.bytes(table, 2), [0x10, 0x10]);
        for (i, body) in [first, second].iter().enumerate() {
            let ms = body + ENTITY_MOTION_STATE;
            let start = i as f32 + 1.0;
            assert_eq!(read_vec4(&e, ms + 0x40), [start, start, start, 1.0]);
            assert_eq!(read_vec4(&e, ms + 0x60), [i as f32 + 5.0; 4]);
            assert_eq!(e.mem.f32(ms + 0x4c), 1.0);
            assert_eq!(e.mem.f32(ms + 0x5c), 0.5);
        }
    }

    #[test]
    fn ceb0_checks_the_critical_constraints_once_more() {
        let mut e = engine();
        let dispatcher = e.mem.alloc(0x2000);
        e.mem.set_f32(dispatcher + (5 << 6) + 0x1c54, 1.0);
        let conversion = e.mem.alloc(4);
        e.mem.set_u32(conversion, dispatcher);
        let scale_block = e.mem.alloc(0x10);
        e.mem.set_f32(scale_block + 8, 1.0);
        let context = e.mem.alloc(0x40);
        e.mem.set_u32(context, scale_block);
        e.mem.set_u32(context + 4, 0x77);
        e.mem.set_u32(context + 0xc, 0x88);
        let a = body(&mut e, 1, 1, 0, 0);
        let b = body(&mut e, 1, 2, 1, 0);
        let violating = instance(&mut e, a, b, 5);
        let other = instance(&mut e, a, b, 2);
        // The constraint data of the violating one: kind 0xb with a block.
        let block = e.mem.alloc(0x80);
        e.mem.set_u16(block + 6, 2);
        e.mem.set_u8(block + 0xa, 3);
        let kind = Rc::new(Cell::new(0xbu32));
        let kind_in = kind.clone();
        e.register_double(0x0600_0020, move |_, _| ret(kind_in.get()));
        let data = virtual_object(&mut e, &[(0x20, 0x0600_0020)]);
        e.mem.set_u32(data + 0x28, block);
        e.mem.set_u32(violating + INSTANCE_DATA, data);
        let velocity_calls = Rc::new(Cell::new(0));
        let velocity_calls_in = velocity_calls.clone();
        let violated = Rc::new(Cell::new(true));
        let violated_in = violated.clone();
        e.register_double(GET_TOI_VIOLATING_VELOCITIES, move |e, a| {
            velocity_calls_in.set(velocity_calls_in.get() + 1);
            assert_eq!((a[0], a[2], a[3]), (0x77, 0x88, 0x100));
            e.mem.set_f32(a[4], 0.5);
            e.mem
                .set_f32(a[4] + 4, if violated_in.get() { 5.0 } else { 0.5 });
            ret(2)
        });
        let records = record_array(&mut e, &[other, violating, violating], 3);
        for i in 0..3 {
            e.mem.set_f32(e.mem.u32(records) + 12 * i + 8, 1.0);
        }
        let result = e.mem.alloc(4);
        let out = record_array(&mut e, &[], 4);
        // Without an output array the first violation ends it with 0.
        let r = e
            .call(
                0x00d0_ceb0,
                &args![result, context, records, conversion, 0u32],
            )
            .u32();
        assert_eq!(r, result);
        assert_eq!(e.mem.u8(result), 0);
        assert_eq!(velocity_calls.get(), 1);
        // With one the violations are collected.
        velocity_calls.set(0);
        e.call(
            0x00d0_ceb0,
            &args![result, context, records, conversion, out],
        );
        assert_eq!(e.mem.u8(result), 0);
        assert_eq!(velocity_calls.get(), 2);
        assert_eq!(record_instances(&e, out), [violating, violating]);
        let element = e.mem.u32(out);
        // n = 1: X + 0x30 + (1 << 5) and X + 0x30 + 3 * 1 + (2 << 5).
        assert_eq!(e.mem.u32(element + 4), block + 0x30 + 32);
        assert_eq!(e.mem.u32(element + 8), block + 0x30 + 3 + 64);
        // Another kind leaves the two words zero.
        kind.set(7);
        e.mem.set_u32(out + 4, 0);
        e.call(
            0x00d0_ceb0,
            &args![result, context, records, conversion, out],
        );
        let element = e.mem.u32(out);
        assert_eq!((e.mem.u32(element + 4), e.mem.u32(element + 8)), (0, 0));
        // Nothing violated: 1.
        violated.set(false);
        e.mem.set_u32(out + 4, 0);
        e.call(
            0x00d0_ceb0,
            &args![result, context, records, conversion, out],
        );
        assert_eq!(e.mem.u8(result), 1);
        assert_eq!(e.mem.u32(out + 4), 0);
    }

    #[test]
    fn d020_builds_the_simulation_for_the_world() {
        let mut e = engine();
        let world = e.mem.alloc(0x400);
        e.mem.set_u32(world + WORLD_SIZE_OF_TOI_EVENT_QUEUE, 64);
        let this: Ptr<HkpContinuousSimulation> = e.new_object();
        // Leftovers that the constructor replaces.
        e.set(this, HkpContinuousSimulation::event_count, 9);
        e.set(this, HkpContinuousSimulation::processed_toi_count, 9);
        e.mem.set_u32(this.addr() + SIM_ENTITIES_MAP + 8, 5);
        quiet(
            &mut e,
            &[BASE_SIMULATION_CONSTRUCTOR, MAP_RESERVE, ARRAY_RESERVE],
        );
        e.register_double(RESOURCE_MANAGER_CONSTRUCTOR, |_, a| ret(a[0]));
        log(&mut e);
        let r = e.call(0x00d0_d020, &args![this, world]).u32();
        assert_eq!(r, this.addr());
        let calls = take_log(&mut e);
        assert_eq!(
            only(
                &calls,
                &[
                    BASE_SIMULATION_CONSTRUCTOR,
                    MAP_RESERVE,
                    ALLOCATE_FN,
                    RESOURCE_MANAGER_CONSTRUCTOR,
                    ARRAY_RESERVE
                ]
            ),
            [
                BASE_SIMULATION_CONSTRUCTOR,
                MAP_RESERVE,
                ALLOCATE_FN,
                RESOURCE_MANAGER_CONSTRUCTOR,
                ARRAY_RESERVE
            ]
        );
        assert_eq!(
            calls_to(&calls, BASE_SIMULATION_CONSTRUCTOR),
            [vec![this.addr(), world]]
        );
        assert_eq!(
            calls_to(&calls, MAP_RESERVE),
            [vec![this.addr() + SIM_ENTITIES_MAP, ALLOCATOR, 0]]
        );
        assert_eq!(calls_to(&calls, ALLOCATE_FN), [vec![ALLOCATOR, 0x10]]);
        assert_eq!(
            calls_to(&calls, ARRAY_RESERVE),
            [vec![
                ALLOCATOR,
                this.addr() + SIM_EVENT_ARRAY,
                64,
                TOI_EVENT_SIZE
            ]]
        );
        assert_eq!(
            e.get(this, HkpContinuousSimulation::vtable),
            SIMULATION_VTABLE
        );
        assert_eq!(e.get(this, HkpContinuousSimulation::event_data), 0);
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 0);
        assert_eq!(
            e.get(this, HkpContinuousSimulation::event_capacity),
            0x8000_0000
        );
        assert_eq!(e.mem.bytes(this.addr() + SIM_ENTITIES_MAP, 8), [0; 8]);
        assert_eq!(e.mem.i32(this.addr() + SIM_ENTITIES_MAP + 8), -1);
        assert_eq!(e.get(this, HkpContinuousSimulation::byte_10), 1);
        assert_eq!(e.get(this, HkpContinuousSimulation::processed_toi_count), 0);
        let manager = e.get(this, HkpContinuousSimulation::toi_resource_manager);
        assert_eq!(e.mem.u16(manager.addr() + 4), 0x10);
        // A queue size of 0 reserves nothing.
        e.mem.set_u32(world + WORLD_SIZE_OF_TOI_EVENT_QUEUE, 0);
        log(&mut e);
        e.call(0x00d0_d020, &args![this, world]);
        assert!(calls_to(&take_log(&mut e), ARRAY_RESERVE).is_empty());
    }

    #[test]
    fn d0d0_releases_the_manager_the_map_and_the_events() {
        let mut e = engine();
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        let data = events(&mut e, this, 2);
        e.set(this, HkpContinuousSimulation::event_capacity, 3);
        let manager = virtual_object(&mut e, &[(0, 0x0600_0000)]);
        e.set(
            this,
            HkpContinuousSimulation::toi_resource_manager,
            Ptr::new(manager),
        );
        quiet(
            &mut e,
            &[
                0x0600_0000,
                MAP_CLEAR_AND_DEALLOCATE,
                MAP_DESTRUCTOR,
                BASE_SIMULATION_DESTRUCTOR,
            ],
        );
        log(&mut e);
        e.call(0x00d0_d0d0, &args![this]);
        let calls = take_log(&mut e);
        assert_eq!(
            only(
                &calls,
                &[
                    0x0600_0000,
                    MAP_CLEAR_AND_DEALLOCATE,
                    MAP_DESTRUCTOR,
                    FREE_FN,
                    BASE_SIMULATION_DESTRUCTOR
                ]
            ),
            [
                0x0600_0000,
                MAP_CLEAR_AND_DEALLOCATE,
                MAP_DESTRUCTOR,
                FREE_FN,
                BASE_SIMULATION_DESTRUCTOR
            ]
        );
        assert_eq!(calls_to(&calls, 0x0600_0000), [vec![manager, 1]]);
        assert_eq!(
            calls_to(&calls, MAP_CLEAR_AND_DEALLOCATE),
            [vec![this.addr() + SIM_ENTITIES_MAP, ALLOCATOR]]
        );
        assert_eq!(
            calls_to(&calls, FREE_FN),
            [vec![ALLOCATOR, data, 3 * TOI_EVENT_SIZE]]
        );
        assert_eq!(
            calls_to(&calls, BASE_SIMULATION_DESTRUCTOR),
            [vec![this.addr()]]
        );
        assert_eq!(
            e.get(this, HkpContinuousSimulation::vtable),
            SIMULATION_VTABLE
        );
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 0);
        assert_eq!(e.get(this, HkpContinuousSimulation::event_data), 0);
        assert_eq!(
            e.get(this, HkpContinuousSimulation::event_capacity),
            0x8000_0000
        );
        // Without a manager and with events in foreign storage nothing else
        // is released.
        e.set(
            this,
            HkpContinuousSimulation::toi_resource_manager,
            Ptr::new(0),
        );
        e.set(this, HkpContinuousSimulation::event_capacity, 0x8000_0003);
        log(&mut e);
        e.call(0x00d0_d0d0, &args![this]);
        let calls = take_log(&mut e);
        assert!(calls_to(&calls, 0x0600_0000).is_empty());
        assert!(calls_to(&calls, FREE_FN).is_empty());
    }

    #[test]
    fn d150_queues_an_event_built_from_the_output_and_the_entry() {
        let mut e = engine();
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        let data = e.mem.alloc(TOI_EVENT_SIZE * 2);
        e.set(this, HkpContinuousSimulation::event_data, data);
        e.set(this, HkpContinuousSimulation::event_capacity, 0x8000_0002);
        let dispatcher = e.mem.u32(world + WORLD_COLLISION_DISPATCHER);
        e.mem.set_u8(dispatcher + 0x1c34 + (2 << 6), 0x5a);
        let output = e.mem.alloc(0x3100);
        e.mem.set_f32(output + OUTPUT_TOI_TIME, 0.25);
        e.mem.set_f32(output + OUTPUT_SEPARATING_VELOCITY, -1.5);
        for i in 0..32 {
            e.mem.set_u8(output + OUTPUT_CONTACT_POINT + i, i as u8 + 1);
        }
        for k in 0..12 {
            e.mem
                .set_u32(output + OUTPUT_PROPERTIES + 4 * k, 0x1000 + k);
        }
        let body_a = body(&mut e, 1, 1, 0, 0);
        let body_b = body(&mut e, 1, 2, 0, 0);
        // The collidables sit inside the bodies; the owner offset is negative.
        let collidable_a = body_a + ENTITY_COLLIDABLE;
        let collidable_b = body_b + ENTITY_COLLIDABLE;
        e.mem.set_i8(
            collidable_a + COLLIDABLE_OWNER_OFFSET,
            -(ENTITY_COLLIDABLE as i8),
        );
        e.mem.set_i8(
            collidable_b + COLLIDABLE_OWNER_OFFSET,
            -(ENTITY_COLLIDABLE as i8),
        );
        let manager = e.mem.alloc(0x80);
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + ENTRY_CONTACT_MANAGER, manager);
        e.mem.set_i8(entry + ENTRY_QUALITY_INDEX, 2);
        e.mem.set_u32(entry + ENTRY_COLLIDABLE_A, collidable_a);
        e.mem.set_u32(entry + ENTRY_COLLIDABLE_B, collidable_b);
        // Garbage in the two floats that the event starts with 0.
        e.mem.set_f32(data + 0x18, 7.0);
        e.call(0x00d0_d150, &args![this, output, entry]);
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 1);
        assert_eq!(e.mem.f32(data + EVENT_TIME), 0.25);
        assert_eq!(e.mem.f32(data + EVENT_SEPARATING_VELOCITY), -1.5);
        assert_eq!(e.mem.u8(data + EVENT_QUALITY_BYTE), 0x5a);
        assert_eq!(e.mem.u32(data + EVENT_ENTITY_A), body_a);
        assert_eq!(e.mem.u32(data + EVENT_ENTITY_B), body_b);
        assert_eq!(e.mem.u32(data + EVENT_CONTACT_MANAGER), manager);
        // The first five words are overwritten with the output's.
        assert_eq!(e.mem.u32(data + EVENT_PROPERTIES), 0x1000);
        for k in 0..12 {
            assert_eq!(e.mem.u32(data + EVENT_PROPERTIES + 4 * k), 0x1000 + k);
        }
        assert_eq!(
            e.mem.bytes(data + EVENT_CONTACT_POINT, 32),
            e.mem.bytes(output + OUTPUT_CONTACT_POINT, 32)
        );
        // A signalling NaN time comes out quiet.
        e.mem.set_u32(output + OUTPUT_TOI_TIME, 0x7f80_0001);
        e.call(0x00d0_d150, &args![this, output, entry]);
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 2);
        assert_eq!(e.mem.u32(data + TOI_EVENT_SIZE + EVENT_TIME), 0x7fc0_0001);
    }

    #[test]
    fn d150_reports_an_error_when_the_queue_is_full() {
        let mut e = engine();
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        events(&mut e, this, 2);
        let error = virtual_object(&mut e, &[(0xc, 0x0600_000c)]);
        e.set_global(ERROR_OBJECT, error);
        quiet(
            &mut e,
            &[
                STREAM_CONSTRUCTOR,
                STREAM_WRITE_TEXT,
                STREAM_DESTRUCTOR,
                0x0600_000c,
            ],
        );
        log(&mut e);
        // The stream at +0x14 and its text at +0x20 of the exe's aligned 0x220-byte frame.
        let block = e.next_stack_local(0x230);
        let frame = (block + 0xf) & !0xf;
        e.call(0x00d0_d150, &args![this, 0x1000u32, 0x2000u32]);
        let calls = take_log(&mut e);
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 2);
        let constructed = calls_to(&calls, STREAM_CONSTRUCTOR);
        assert_eq!(constructed.len(), 1);
        let (stream, buffer) = (constructed[0][0], constructed[0][1]);
        assert_eq!((stream, buffer), (frame + 0x14, frame + 0x20));
        assert_eq!(constructed[0][2], 0x200);
        assert_eq!(
            calls_to(&calls, STREAM_WRITE_TEXT),
            [vec![stream, TOI_QUEUE_FULL_TEXT]]
        );
        assert_eq!(
            calls_to(&calls, 0x0600_000c),
            [vec![
                error,
                1,
                0xf032_3454,
                buffer,
                ERROR_SOURCE_NAME,
                0x43b
            ]]
        );
        assert_eq!(calls_to(&calls, STREAM_DESTRUCTOR), [vec![stream]]);
        assert_eq!(
            only(
                &calls,
                &[
                    STREAM_CONSTRUCTOR,
                    STREAM_WRITE_TEXT,
                    0x0600_000c,
                    STREAM_DESTRUCTOR
                ]
            ),
            [
                STREAM_CONSTRUCTOR,
                STREAM_WRITE_TEXT,
                0x0600_000c,
                STREAM_DESTRUCTOR
            ]
        );
    }

    #[test]
    fn d2c0_collects_the_agent_entries_that_have_to_be_processed() {
        let mut e = engine();
        let entity = body(&mut e, 1, 1, 0, 0);
        let table = e.mem.alloc(8);
        let flags = e.mem.alloc(4);
        e.mem.set_u32(flags, table);
        // Five pairs (entry quality, other body motion type, other state).
        let setups = [(1u8, 1u8, 0u8), (3, 5, 0), (3, 1, 0), (2, 1, 8), (2, 1, 1)];
        let pairs = e.mem.alloc(8 * 5);
        let mut entries = vec![];
        let mut others = vec![];
        for (i, (quality, motion_type, state)) in setups.iter().enumerate() {
            let other = body(&mut e, *motion_type, 10 + i as u32, i as u16, 0);
            e.mem.set_u8(table + i as u32, *state);
            let collidable = other + ENTITY_COLLIDABLE;
            e.mem.set_i8(
                collidable + COLLIDABLE_OWNER_OFFSET,
                -(ENTITY_COLLIDABLE as i8),
            );
            let entry = e.mem.alloc(0x20);
            e.mem.set_u8(entry + ENTRY_QUALITY_INDEX, *quality);
            e.mem.set_u32(pairs + 8 * i as u32, entry);
            e.mem.set_u32(pairs + 8 * i as u32 + 4, collidable);
            entries.push(entry);
            others.push(other);
        }
        e.register_double(GET_COLLISION_ENTRIES_SORTED, move |e, a| {
            assert_eq!(a[0], entity + ENTITY_COLLIDABLE);
            e.mem.set_u32(a[1], pairs);
            e.mem.set_u32(a[1] + 4, 5);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        quiet(&mut e, &[INTERPOLATE_MOTION_STATE]);
        e.register_double(ARRAY_GROW, |e, a| {
            let (array, element) = (a[1], a[2]);
            let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
            let data = e.mem.alloc(element * capacity * 2);
            let old = e.mem.bytes(e.mem.u32(array), element * capacity);
            e.mem.write(data, &old);
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, capacity * 2);
            Ret::default()
        });
        // An output array that is full after two entries.
        let output = word_array(&mut e, &[], 2);
        log(&mut e);
        // The sorted entries sit at +0x10 of the exe's 0x1c-byte frame.
        let frame = e.next_stack_local(0x1c);
        e.call(0x00d0_d2c0, &args![entity, 2i32, 1.5f32, flags, output]);
        let calls = take_log(&mut e);
        assert_eq!(
            calls_to(&calls, GET_COLLISION_ENTRIES_SORTED)[0][1],
            frame + 0x10
        );
        assert_eq!(
            array_items(&e, output),
            [entries[1], entries[2], entries[4]]
        );
        assert_eq!(calls_to(&calls, ARRAY_GROW), [vec![ALLOCATOR, output, 4]]);
        // The body at state 0 moved to state 1 and was interpolated.
        assert_eq!(e.mem.bytes(table, 5), [0, 0, 1, 8, 1]);
        assert_eq!(
            calls_to(&calls, INTERPOLATE_MOTION_STATE),
            [vec![
                others[2] + ENTITY_SWEPT_TRANSFORM,
                1.5f32.to_bits(),
                others[2] + ENTITY_MOTION_STATE
            ]]
        );
        // The pair array (heap-allocated, 8 elements of 8 bytes) is freed.
        assert_eq!(calls_to(&calls, FREE_FN), [vec![ALLOCATOR, pairs, 64]]);
    }

    #[test]
    fn d3f0_updates_the_agents_for_the_pairs_the_broad_phase_found() {
        let mut e = engine();
        let world = world(&mut e);
        let broad_phase = virtual_object(&mut e, &[(0x2c, 0x0600_002c)]);
        e.mem.set_u32(world + WORLD_BROAD_PHASE, broad_phase);
        let pair_dispatcher = e.mem.alloc(0x40);
        e.mem
            .set_u32(world + WORLD_BROAD_PHASE_DISPATCHER, pair_dispatcher);
        e.mem.set_u32(world + WORLD_BROAD_PHASE_UPDATE_SIZE, 8);
        let filter = e.mem.alloc(0x40);
        e.mem.set_u32(world + WORLD_COLLISION_FILTER, filter);
        let island = 0x7777u32;
        let first = bounded_body(&mut e, [10, 20, 30], [50, 60, 70]);
        e.mem.set_u32(first + ENTITY_SIMULATION_ISLAND, island);
        let entities = word_array(&mut e, &[first], 1);
        let list = e.mem.u32(entities);
        quiet(
            &mut e,
            &[
                BROAD_PHASE_LOCK,
                BROAD_PHASE_UNLOCK,
                REMOVE_DUPLICATES,
                WORLD_LOCK_ISLAND,
                WORLD_UNLOCK_ISLAND,
                DISPATCHER_REMOVE_PAIRS,
                DISPATCHER_ADD_PAIRS,
            ],
        );
        e.mem.set_u32(broad_phase + 0x10, 1);
        let found = Rc::new(RefCell::new((0u32, 0u32)));
        let found_in = found.clone();
        e.register_double(0x0600_002c, move |e, a| {
            // One pair to add, two to remove (in the stack memory arrays).
            assert_eq!(a[3], 1);
            let (added, removed) = (a[4], a[5]);
            for (array, pairs) in [(added, 1u32), (removed, 2)] {
                for k in 0..pairs {
                    let at = e.mem.u32(array) + 8 * k;
                    e.mem.set_u32(at, 0x100 * (k + 1));
                    e.mem.set_u32(at + 4, 0x200 * (k + 1));
                }
                e.mem.set_u32(array + 4, pairs);
            }
            *found_in.borrow_mut() = (e.mem.u32(added), e.mem.u32(removed));
            Ret::default()
        });
        let accept = Rc::new(Cell::new(1u32));
        let accept_in = accept.clone();
        e.register_double(0x0400_2124, move |_, a| {
            assert_eq!(a, [ALLOCATOR, 1 << 10]);
            ret(accept_in.get())
        });
        let export = e.mem.alloc(4);
        e.mem.set_u8(export, 2);
        log(&mut e);
        // The added pairs at +0x38 and the removed at +0x28 of the exe's 0x48-byte frame.
        let frame = e.next_stack_local(0x48);
        e.call(0x00d0_d3f0, &args![list, 1i32, world, export]);
        let calls = take_log(&mut e);
        assert_eq!(
            calls_to(&calls, 0x0600_002c)[0][4..],
            [frame + 0x38, frame + 0x28]
        );
        let (added_data, removed_data) = *found.borrow();
        assert_eq!(
            only(
                &calls,
                &[
                    BROAD_PHASE_LOCK,
                    0x0600_002c,
                    BROAD_PHASE_UNLOCK,
                    REMOVE_DUPLICATES,
                    WORLD_LOCK_ISLAND,
                    DISPATCHER_REMOVE_PAIRS,
                    0x0400_2124,
                    DISPATCHER_ADD_PAIRS,
                    WORLD_UNLOCK_ISLAND
                ]
            ),
            [
                BROAD_PHASE_LOCK,
                0x0600_002c,
                BROAD_PHASE_UNLOCK,
                REMOVE_DUPLICATES,
                WORLD_LOCK_ISLAND,
                DISPATCHER_REMOVE_PAIRS,
                0x0400_2124,
                DISPATCHER_ADD_PAIRS,
                WORLD_UNLOCK_ISLAND
            ]
        );
        assert_eq!(calls_to(&calls, WORLD_LOCK_ISLAND), [vec![world, island]]);
        assert_eq!(
            calls_to(&calls, DISPATCHER_REMOVE_PAIRS),
            [vec![pair_dispatcher, removed_data, 2]]
        );
        assert_eq!(
            calls_to(&calls, DISPATCHER_ADD_PAIRS),
            [vec![pair_dispatcher, added_data, 1, filter + 8]]
        );
        assert_eq!(calls_to(&calls, WORLD_UNLOCK_ISLAND), [vec![world, island]]);
        // The stack memory is back.
        assert_eq!(e.mem.u32(ROUTER_BLOCK), STACK_BASE);
        // The export wait records TtWaitForExport and its end.
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0x28), PROFILE_TT_WAIT_FOR_EXPORT);
        // The allocator refuses and there is no filter: no agents are added.
        accept.set(0);
        e.mem.set_u32(world + WORLD_COLLISION_FILTER, 0);
        log(&mut e);
        e.call(0x00d0_d3f0, &args![list, 1i32, world, 0u32]);
        let calls = take_log(&mut e);
        assert!(calls_to(&calls, DISPATCHER_ADD_PAIRS).is_empty());
        assert_eq!(calls_to(&calls, DISPATCHER_REMOVE_PAIRS).len(), 1);
        // No bodies: nothing happens.
        log(&mut e);
        e.call(0x00d0_d3f0, &args![list, 0i32, world, 0u32]);
        assert!(take_log(&mut e).is_empty());
    }

    #[test]
    fn d8b0_runs_the_narrow_phase_on_every_entry_of_the_island() {
        let mut e = engine();
        e.set_global(TOI_TIME_NOT_SET, 123.0f32);
        e.set_global(TOI_TIME_SENTINEL, 123.0f64);
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        let data = events(&mut e, this, 0);
        e.set(this, HkpContinuousSimulation::event_capacity, 0x8000_0004);
        let _ = data;
        let dispatcher = e.mem.alloc(0x2000);
        let input = e.mem.alloc(0x100);
        e.mem.set_u32(input, dispatcher);
        e.mem.set_u32(dispatcher + 0x1c20 + 0x10 + 0x40, 0xcafe);
        let island = e.mem.alloc(0x80);
        let sector = e.mem.alloc(0x200);
        let sectors = e.mem.alloc(4);
        e.mem.set_u32(sectors, sector);
        e.mem.set_u32(island + ISLAND_SECTORS, sectors);
        e.mem.set_u32(island + ISLAND_SECTOR_COUNT, 1);
        e.mem.set_u32(island + ISLAND_BYTES_USED, 0x60);
        let seen = Rc::new(RefCell::new(vec![]));
        let seen_in = seen.clone();
        e.register_double(0x0600_0018, move |_, a| {
            seen_in.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let manager = virtual_object(&mut e, &[(0x18, 0x0600_0018)]);
        let body_a = body(&mut e, 1, 1, 0, 0);
        let body_b = body(&mut e, 1, 2, 0, 0);
        let mut entries = vec![];
        for i in 0..2u32 {
            let entry = sector + 0x30 * i;
            e.mem.set_u8(entry + 3, 0x30);
            e.mem.set_u32(entry + ENTRY_CONTACT_MANAGER, manager);
            e.mem.set_i8(entry + ENTRY_QUALITY_INDEX, 1);
            let collidable_a = body_a + ENTITY_COLLIDABLE;
            let collidable_b = body_b + ENTITY_COLLIDABLE;
            e.mem.set_i8(
                collidable_a + COLLIDABLE_OWNER_OFFSET,
                -(ENTITY_COLLIDABLE as i8),
            );
            e.mem.set_i8(
                collidable_b + COLLIDABLE_OWNER_OFFSET,
                -(ENTITY_COLLIDABLE as i8),
            );
            e.mem.set_u32(entry + ENTRY_COLLIDABLE_A, collidable_a);
            e.mem.set_u32(entry + ENTRY_COLLIDABLE_B, collidable_b);
            entries.push(entry);
        }
        let processed = Rc::new(RefCell::new(vec![]));
        let processed_in = processed.clone();
        let first_entry = entries[0];
        e.register_double(PROCESS_AGENT, move |e, a| {
            let output = a[2];
            processed_in.borrow_mut().push((
                a[0],
                e.mem.u32(a[1] + 0x60),
                e.mem.u32(a[1] + 0x14),
                e.mem.u32(output + 4),
            ));
            if a[0] == first_entry {
                // A time of impact is found.
                e.mem.set_f32(output + OUTPUT_TOI_TIME, 0.5);
            } else {
                // A contact is found.
                e.mem.set_u32(output, output + 0x40);
            }
            Ret::default()
        });
        e.call(0x00d0_d8b0, &args![this, island, input]);
        let quality_info = dispatcher + 0x1c20 + 0x40;
        assert_eq!(
            *processed.borrow(),
            [
                (entries[0], quality_info, 0xcafe, island),
                (entries[1], quality_info, 0xcafe, island)
            ]
        );
        assert_eq!(e.mem.u32(input + 0x60), quality_info);
        // The second entry's contact went to its manager.
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(
            &seen[0][..4],
            &[
                manager,
                body_a + ENTITY_COLLIDABLE,
                body_b + ENTITY_COLLIDABLE,
                input
            ]
        );
        // The first entry's time of impact became an event.
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 1);
        assert_eq!(
            e.mem
                .f32(e.get(this, HkpContinuousSimulation::event_data) + EVENT_TIME),
            0.5
        );
        assert_eq!(e.mem.u32(STREAM_RECORDS), PROFILE_TT_NARROW_PHASE);
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0xc), PROFILE_END);
    }

    #[test]
    fn d8b0_stops_at_the_abort_flag() {
        let mut e = engine();
        e.set_global(TOI_TIME_NOT_SET, 123.0f32);
        e.set_global(ABORT_FLAG, 1u32);
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        let dispatcher = e.mem.alloc(0x2000);
        let input = e.mem.alloc(0x100);
        e.mem.set_u32(input, dispatcher);
        let island = e.mem.alloc(0x80);
        let sector = e.mem.alloc(0x200);
        let sectors = e.mem.alloc(4);
        e.mem.set_u32(sectors, sector);
        e.mem.set_u32(island + ISLAND_SECTORS, sectors);
        e.mem.set_u32(island + ISLAND_SECTOR_COUNT, 1);
        e.mem.set_u32(island + ISLAND_BYTES_USED, 0x60);
        e.mem.set_u8(sector + 3, 0x30);
        e.mem.set_u8(sector + 0x30 + 3, 0x30);
        let count = Rc::new(Cell::new(0));
        let count_in = count.clone();
        e.register_double(PROCESS_AGENT, move |_, _| {
            count_in.set(count_in.get() + 1);
            Ret::default()
        });
        e.call(0x00d0_d8b0, &args![this, island, input]);
        assert_eq!(count.get(), 1);
        // The abort path writes the end record too.
        assert_eq!(e.mem.u32(STREAM + 4), STREAM_RECORDS + 0x18);
    }

    /// Gives `object` a virtual table of its own with the given slots.
    fn set_vtable(e: &mut Engine, object: u32, slots: &[(u32, u32)]) {
        let vtable = e.mem.alloc(0x100);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        e.mem.set_u32(object, vtable);
    }

    /// An agent entry with its contact manager, quality and the two
    /// collidables (embedded in the given bodies).
    fn agent_entry(e: &mut Engine, manager: u32, quality: i8, first: u32, second: u32) -> u32 {
        let entry = e.mem.alloc(0x20);
        e.mem.set_u32(entry + ENTRY_CONTACT_MANAGER, manager);
        e.mem.set_i8(entry + ENTRY_QUALITY_INDEX, quality);
        for (slot, body) in [(ENTRY_COLLIDABLE_A, first), (ENTRY_COLLIDABLE_B, second)] {
            let collidable = body + ENTITY_COLLIDABLE;
            // The owner offset leads back from the collidable to the body.
            e.mem.set_i8(
                collidable + COLLIDABLE_OWNER_OFFSET,
                -(ENTITY_COLLIDABLE as i8),
            );
            e.mem.set_u32(entry + slot, collidable);
        }
        entry
    }

    /// An array of (entry, collidable) pairs the way the collision entry
    /// lookups answer: (data, size, capacity word).
    fn pair_array(e: &mut Engine, pairs: &[(u32, u32)]) -> u32 {
        let array = e.mem.alloc(16);
        let data = e.mem.alloc(8 * pairs.len().max(1) as u32);
        for (i, (entry, other)) in pairs.iter().enumerate() {
            e.mem.set_u32(data + 8 * i as u32, *entry);
            e.mem.set_u32(data + 8 * i as u32 + 4, *other);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, pairs.len() as u32);
        e.mem.set_u32(array + 8, 0x8000_0000 | pairs.len() as u32);
        array
    }

    #[test]
    fn db40_processes_now_or_leaves_the_body_to_the_psi_pass() {
        let mut e = engine();
        e.set_global(TOI_TIME_NOT_SET, 123.0f32);
        e.set_global(TOI_TIME_SENTINEL, 123.0f64);
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        let data = e.mem.alloc(TOI_EVENT_SIZE * 4);
        e.set(this, HkpContinuousSimulation::event_data, data);
        e.set(this, HkpContinuousSimulation::event_capacity, 0x8000_0004);
        let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
        let dispatcher = e.mem.u32(world + WORLD_COLLISION_DISPATCHER);
        // Quality 0 has a creation word and no flag byte (process now);
        // quality 1 has no creation word (the PSI pass handles it).
        e.mem.set_u32(dispatcher + 0x1c30, 0x11);
        let island = e.mem.alloc(0x80);
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, 2);
        let other_island = e.mem.alloc(0x80);
        let x = body(&mut e, 1, 5, 0, island);
        let y = body(&mut e, 1, 6, 1, island);
        let z = body(&mut e, 1, 7, 2, other_island);
        let manager = virtual_object(&mut e, &[(0x18, 0x0600_0018)]);
        let entry_a = agent_entry(&mut e, manager, 0, x, y);
        let entry_b = agent_entry(&mut e, manager, 1, y, z);
        let x_pairs = pair_array(&mut e, &[(entry_a, y + ENTITY_COLLIDABLE)]);
        let y_pairs = pair_array(
            &mut e,
            &[
                (entry_a, x + ENTITY_COLLIDABLE),
                (entry_b, z + ENTITY_COLLIDABLE),
            ],
        );
        e.register_double(GET_COLLISION_ENTRIES_SORTED, move |e, a| {
            let source = if a[0] == x + ENTITY_COLLIDABLE {
                x_pairs
            } else {
                y_pairs
            };
            for k in 0..3 {
                let word = e.mem.u32(source + 4 * k);
                e.mem.set_u32(a[1] + 4 * k, word);
            }
            Ret::default()
        });
        quiet(&mut e, &[MEM_SET, ADD_REFERENCE, 0x0600_0018]);
        e.register_double(MAP_GET_WITH_DEFAULT, |_, _| ret(0));
        quiet(&mut e, &[MAP_INSERT]);
        let processed = Rc::new(RefCell::new(vec![]));
        let processed_in = processed.clone();
        e.register_double(PROCESS_AGENT, move |e, a| {
            processed_in.borrow_mut().push(a.to_vec());
            // A time of impact.
            e.mem.set_f32(a[2] + OUTPUT_TOI_TIME, 0.5);
            Ret::default()
        });
        let entities = word_array(&mut e, &[x, y], 2);
        let list = e.mem.u32(entities);
        let map = e.mem.alloc(12);
        log(&mut e);
        e.call(0x00d0_db40, &args![this, list, 2i32, input, map]);
        let calls = take_log(&mut e);
        // Only the first entry was processed, once.
        let processed = processed.borrow();
        assert_eq!(processed.len(), 1);
        assert_eq!(processed[0][0], entry_a);
        assert_eq!(processed[0][1], input);
        assert_eq!(processed[0][3], manager);
        assert_eq!(calls_to(&calls, MEM_SET).len(), 1);
        assert_eq!(calls_to(&calls, MEM_SET)[0][1..], [0, 2]);
        // The second entry's body goes to the map under its uid.
        assert_eq!(calls_to(&calls, MAP_GET_WITH_DEFAULT), [vec![map, 6, 0]]);
        assert_eq!(calls_to(&calls, ADD_REFERENCE), [vec![y]]);
        assert_eq!(calls_to(&calls, MAP_INSERT), [vec![map, ALLOCATOR, 6, y]]);
        // The time of impact became an event.
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 1);
        assert_eq!(e.mem.f32(data + EVENT_TIME), 0.5);
        assert_eq!(e.mem.u32(ROUTER_BLOCK), STACK_BASE);
        assert_eq!(e.mem.u32(STREAM_RECORDS), PROFILE_TT_NARROW_PHASE_TOI);
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0xc), PROFILE_END);
    }

    #[test]
    fn db40_keeps_a_body_the_map_already_knows_out_of_the_map() {
        let mut e = engine();
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
        let island = e.mem.alloc(0x80);
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, 1);
        let x = body(&mut e, 1, 5, 0, island);
        let z = body(&mut e, 1, 7, 2, 0);
        let manager = e.mem.alloc(0x40);
        // Quality 0 has no creation word: the PSI pass is the one to do it.
        let entry = agent_entry(&mut e, manager, 0, x, z);
        let pairs = pair_array(&mut e, &[(entry, z + ENTITY_COLLIDABLE)]);
        e.register_double(GET_COLLISION_ENTRIES_SORTED, move |e, a| {
            for k in 0..3 {
                let word = e.mem.u32(pairs + 4 * k);
                e.mem.set_u32(a[1] + 4 * k, word);
            }
            Ret::default()
        });
        quiet(&mut e, &[MEM_SET, ADD_REFERENCE, MAP_INSERT]);
        e.register_double(MAP_GET_WITH_DEFAULT, |_, _| ret(0x1234));
        let entities = word_array(&mut e, &[x], 1);
        let list = e.mem.u32(entities);
        let map = e.mem.alloc(12);
        log(&mut e);
        e.call(0x00d0_db40, &args![this, list, 1i32, input, map]);
        let calls = take_log(&mut e);
        assert_eq!(calls_to(&calls, MAP_GET_WITH_DEFAULT).len(), 1);
        assert!(calls_to(&calls, ADD_REFERENCE).is_empty());
        assert!(calls_to(&calls, MAP_INSERT).is_empty());
        assert!(calls_to(&calls, PROCESS_AGENT).is_empty());
    }

    #[test]
    fn de70_processes_the_entries_the_toi_pass_left_to_the_psi_pass() {
        let mut e = engine();
        grow_double(&mut e);
        e.set_global(TOI_TIME_NOT_SET, 123.0f32);
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
        let dispatcher = e.mem.u32(world + WORLD_COLLISION_DISPATCHER);
        // Quality 0: no creation word. Quality 1: a creation word and a flag
        // byte (only taken for a body that does not move).
        e.mem.set_u32(dispatcher + 0x1c70, 0x22);
        e.mem.set_u8(dispatcher + 0x1c74, 1);
        let island_1 = 0x1111u32;
        let island_z = 0x9999u32;
        let first = body(&mut e, 1, 1, 0, island_1);
        let second = body(&mut e, 5, 2, 1, 0x2222);
        let z = body(&mut e, 1, 3, 2, island_z);
        e.mem.set_f32(first + ENTITY_INV_DELTA_TIME, 1.0);
        let manager = virtual_object(&mut e, &[(0x18, 0x0600_0018)]);
        let entry_1 = agent_entry(&mut e, manager, 0, first, z);
        let entry_2 = agent_entry(&mut e, manager, 1, second, z);
        let entry_3 = agent_entry(&mut e, manager, 1, first, z);
        let first_entries = pair_array(&mut e, &[(entry_1, 0), (entry_3, 0)]);
        let second_entries = pair_array(&mut e, &[(entry_1, 0), (entry_2, 0)]);
        e.register_double(ENTRIES_DETERMINISTIC_UNCHECKED, move |_, a| {
            ret(if a[0] == first + ENTITY_COLLIDABLE {
                first_entries
            } else {
                second_entries
            })
        });
        // The map of bodies (uid to body): slots 1 and 3 are in use.
        let map = e.mem.alloc(12);
        let slots = e.mem.alloc(8 * 4);
        for k in 0..4 {
            e.mem.set_u32(slots + 8 * k, 0xffff_ffff);
        }
        e.mem.set_u32(slots + 8, 1);
        e.mem.set_u32(slots + 12, first);
        e.mem.set_u32(slots + 24, 2);
        e.mem.set_u32(slots + 28, second);
        e.mem.set_u32(map, slots);
        e.mem.set_i32(map + 8, 3);
        let visited = Rc::new(RefCell::new(vec![]));
        let visited_in = visited.clone();
        e.register_double(MAP_INSERT, move |_, a| {
            visited_in.borrow_mut().push(a[2]);
            Ret::default()
        });
        let visited_in = visited.clone();
        e.register_double(MAP_GET_WITH_DEFAULT, move |_, a| {
            ret(u32::from(visited_in.borrow().contains(&a[1])))
        });
        quiet(
            &mut e,
            &[
                MAP_RESERVE,
                MAP_CLEAR,
                MAP_CLEAR_AND_DEALLOCATE,
                MAP_DESTRUCTOR,
                REMOVE_REFERENCE,
                WORLD_UNLOCK_CALLBACK_A,
                0x0600_0018,
            ],
        );
        e.mem.set_u32(world + WORLD_PENDING_OPERATIONS_COUNT, 1);
        let processed = Rc::new(RefCell::new(vec![]));
        let processed_in = processed.clone();
        e.register_double(PROCESS_AGENT, move |e, a| {
            processed_in.borrow_mut().push((a[0], e.mem.u32(a[2] + 4)));
            // The second entry produces a contact.
            if a[0] == entry_2 {
                e.mem.set_u32(a[2], a[2] + 0x40);
            }
            Ret::default()
        });
        log(&mut e);
        e.call(0x00d0_de70, &args![this, input, map]);
        let calls = take_log(&mut e);
        // Entry 1 qualifies (no creation word), entry 3 does not (the body
        // is moving), entry 2 does (a flag byte and the body is at rest).
        assert_eq!(*visited.borrow(), [entry_1, entry_2]);
        // The island of entry 2 is the one of its other body.
        assert_eq!(
            *processed.borrow(),
            [(entry_1, island_1), (entry_2, island_z)]
        );
        assert_eq!(
            calls_to(&calls, REMOVE_REFERENCE),
            [vec![first], vec![second]]
        );
        assert_eq!(calls_to(&calls, MAP_CLEAR), [vec![map]]);
        assert_eq!(calls_to(&calls, MAP_DESTRUCTOR).len(), 1);
        assert_eq!(calls_to(&calls, 0x0600_0018).len(), 1);
        assert_eq!(
            calls_to(&calls, 0x0600_0018)[0][..2],
            [manager, second + ENTITY_COLLIDABLE]
        );
        // The critical operations lock was taken and released: the pending
        // operations ran.
        assert_eq!(e.mem.u32(world + WORLD_CRITICAL_OPERATIONS_LOCK_COUNT), 0);
        assert_eq!(calls_to(&calls, WORLD_UNLOCK_CALLBACK_A), [vec![world]]);
        assert_eq!(e.mem.u32(STREAM_RECORDS), PROFILE_TT_RECOLLIDE_PSI);
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0xc), PROFILE_END);
    }

    #[test]
    fn e210_handles_a_time_of_impact_that_needs_no_solve() {
        let mut e = engine();
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
        let queued_seen = Seen::default();
        let data = events(&mut e, this, 2);
        let queued_manager = reporting_manager(&mut e, 0x0600_0120, queued_seen.clone());
        // The body that is frozen (quality type 3), the other one is not.
        let frozen = body(&mut e, 1, 1, 0, 0x111);
        let other = body(&mut e, 1, 2, 0, 0x222);
        let third = body(&mut e, 1, 3, 0, 0x333);
        e.mem.set_u8(frozen + ENTITY_QUALITY_TYPE, 3);
        set_event(&mut e, data, 1.0, frozen, third, queued_manager);
        set_event(
            &mut e,
            data + TOI_EVENT_SIZE,
            2.0,
            other,
            third,
            queued_manager,
        );
        let event_seen = Seen::default();
        let manager = virtual_object(&mut e, &[(0x20, 0x0600_0020), (0x48, 0x0600_0048)]);
        e.register_double(0x0600_0048, |_, _| Ret::default());
        e.register_double(0x0600_0020, {
            let seen = event_seen.clone();
            move |_, a| {
                seen.borrow_mut().push(a.to_vec());
                Ret::default()
            }
        });
        set_vtable(&mut e, this.addr(), &[(0x3c, 0x0600_003c)]);
        let narrow = Rc::new(RefCell::new(vec![]));
        let narrow_in = narrow.clone();
        e.register_double(0x0600_003c, move |e, a| {
            narrow_in.borrow_mut().push((a.to_vec(), e.mem.u32(a[1])));
            Ret::default()
        });
        quiet(&mut e, &[FREEZE_MOTION_STATE, INVALIDATE_TIM]);
        let event = e.mem.alloc(TOI_EVENT_SIZE);
        set_event(&mut e, event, 0.5, frozen, other, manager);
        e.mem.set_f32(event + EVENT_TIME, 0.5);
        log(&mut e);
        e.call(0x00d0_e210, &args![this, world, event, 0u32, 1.5f32]);
        let calls = take_log(&mut e);
        // The manager was asked, with the time and a local body array.
        let asked = calls_to(&calls, 0x0600_0048);
        assert_eq!(asked.len(), 1);
        assert_eq!(
            (asked[0][0], asked[0][1], asked[0][2]),
            (manager, event, 1.5f32.to_bits())
        );
        // The frozen body: frozen at the event time, its data invalidated.
        assert_eq!(
            calls_to(&calls, FREEZE_MOTION_STATE),
            [vec![0.5f32.to_bits(), frozen + ENTITY_MOTION_STATE]]
        );
        assert_eq!(calls_to(&calls, INVALIDATE_TIM), [vec![frozen, input]]);
        // The queued event of the frozen body was reported and dropped.
        assert_eq!(
            *queued_seen.borrow(),
            [vec![queued_manager, 0x111, data + EVENT_PROPERTIES]]
        );
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 1);
        assert_eq!(e.mem.u32(data + EVENT_ENTITY_A), other);
        // The event itself was reported to its manager.
        assert_eq!(
            *event_seen.borrow(),
            [vec![manager, 0x111, event + EVENT_PROPERTIES]]
        );
        // The narrow phase runs for the frozen body.
        let narrow = narrow.borrow();
        assert_eq!(narrow.len(), 1);
        assert_eq!(narrow[0].0[0], this.addr());
        assert_eq!(narrow[0].0[2..], [1, input, this.addr() + SIM_ENTITIES_MAP]);
        assert_eq!(narrow[0].1, frozen);
        // Lock released, two records.
        assert_eq!(e.mem.u32(world + WORLD_CRITICAL_OPERATIONS_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(STREAM_RECORDS), PROFILE_TT_SIMPLE_TOI);
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0xc), PROFILE_END);
    }

    #[test]
    fn e4b0_wakes_the_bodies_and_collects_the_constraints_to_check() {
        let mut e = engine();
        let world = world(&mut e);
        let input = e.mem.alloc(0x100);
        // The old step info.
        for (k, value) in [0.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
            e.mem.set_f32(input + 0x50 + 4 * k as u32, *value);
        }
        let simulation_object = virtual_object(&mut e, &[(0x48, 0x0600_0048)]);
        e.mem.set_u32(world + WORLD_SIMULATION, simulation_object);
        let island = 0x4242u32;
        let make = |e: &mut Engine, motion: u8, uid: u32, index: u16| {
            let b = body(e, motion, uid, index, island);
            e.mem.set_u32(b + ENTITY_WORLD, world);
            b
        };
        let main = make(&mut e, 1, 1, 0);
        let p = make(&mut e, 1, 2, 1);
        let q = make(&mut e, 5, 3, 2);
        let r = make(&mut e, 1, 4, 3);
        let t = make(&mut e, 1, 5, 4);
        let u = make(&mut e, 1, 6, 5);
        let table = e.mem.alloc(8);
        e.mem.write(table, &[0, 0, 0, 0x10, 1, 0]);
        let flags = e.mem.alloc(4);
        e.mem.set_u32(flags, table);
        // Master constraints: one that counts, one below the priority.
        let masters = e.mem.alloc(0x60);
        let master_instances = [instance(&mut e, main, p, 3), instance(&mut e, main, p, 0)];
        for (k, inst) in master_instances.iter().enumerate() {
            let block = masters + 0x30 * k as u32;
            e.mem.set_u32(block, *inst);
            e.mem.set_u32(block + INTERNAL_ENTITY_A, main);
            e.mem.set_u32(block + INTERNAL_ENTITY_B, p);
        }
        e.mem.set_u32(main + ENTITY_MASTER_CONSTRAINTS, masters);
        e.mem.set_u16(main + ENTITY_MASTER_CONSTRAINT_COUNT, 2);
        // Slave constraints: fixed other body, other at state 0x10, other at
        // state 1, other at state 0.
        let slave_instances = [
            instance(&mut e, q, main, 3),
            instance(&mut e, main, r, 3),
            instance(&mut e, t, main, 3),
            instance(&mut e, main, u, 3),
        ];
        let slaves = word_array(&mut e, &slave_instances, 4);
        e.mem
            .set_u32(main + ENTITY_SLAVE_CONSTRAINTS, e.mem.u32(slaves));
        e.mem.set_i32(main + ENTITY_SLAVE_CONSTRAINT_COUNT, 4);
        // The agent entries: one of the body, with a fixed other body.
        let manager = e.mem.alloc(0x40);
        let entry = agent_entry(&mut e, manager, 3, main, q);
        let pairs = pair_array(&mut e, &[(entry, q + ENTITY_COLLIDABLE)]);
        e.register_double(GET_COLLISION_ENTRIES_SORTED, move |e, a| {
            for k in 0..3 {
                let word = e.mem.u32(pairs + 4 * k);
                e.mem.set_u32(a[1] + 4 * k, word);
            }
            Ret::default()
        });
        let narrow = Rc::new(RefCell::new(vec![]));
        let narrow_in = narrow.clone();
        e.register_double(0x0600_0048, move |e, a| {
            narrow_in.borrow_mut().push((
                a.to_vec(),
                e.mem.u32(a[1]),
                e.mem.bytes(input + 0x50, 16),
            ));
            Ret::default()
        });
        quiet(&mut e, &[INTERPOLATE_MOTION_STATE, QUICK_SORT_RECURSIVE]);
        let entities = word_array(&mut e, &[main], 1);
        let new_entities = word_array(&mut e, &[], 8);
        let constraints = word_array(&mut e, &[], 8);
        log(&mut e);
        // The collected entries sit at +0x30 of the exe's aligned 0x50-byte frame (their
        // capacity word, 0x3e8 not owned, at +0x38; read after the call: the ended frame's bytes
        // stay on the thread's stack).
        let block = e.next_stack_local(0x60);
        let frame = (block + 0xf) & !0xf;
        e.call(
            0x00d0_e4b0,
            &args![
                input,
                1i32,
                1.5f32,
                entities,
                flags,
                new_entities,
                constraints
            ],
        );
        let calls = take_log(&mut e);
        assert_eq!(e.mem.u32(frame + 0x38), 0x8000_03e8);
        // The narrow phase of the island's simulation, with the one entry.
        let narrow = narrow.borrow();
        assert_eq!(narrow.len(), 1);
        let simulation_word = (simulation_object & 0xffff_ff00) | 1;
        assert_eq!(narrow[0].0[0], simulation_object);
        assert_eq!(narrow[0].0[2], 1);
        assert_eq!(narrow[0].0[3], input);
        assert_eq!(narrow[0].0[4], island);
        assert_eq!(narrow[0].0[5], simulation_word);
        assert_eq!(narrow[0].1, entry);
        // During that call the step info ran from the old start to the time.
        let words: Vec<f32> = narrow[0]
            .2
            .chunks(4)
            .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect();
        assert_eq!(words, [0.0, 1.5, 1.5, (1.0f64 / 1.5) as f32]);
        // Afterwards the old step info is back.
        assert_eq!(read_vec4(&e, input + 0x50), [0.0, 2.0, 3.0, 4.0]);
        // The states: the main body is done, the touched ones are awake.
        assert_eq!(e.mem.bytes(table, 6), [0x10, 2, 0, 0x10, 2, 2]);
        assert_eq!(array_items(&e, new_entities), [p, t, u]);
        assert_eq!(
            array_items(&e, constraints),
            [
                master_instances[0],
                slave_instances[0],
                slave_instances[2],
                slave_instances[3]
            ]
        );
        // Interpolated: the main body, then the bodies that were at state 0.
        let interpolated: Vec<u32> = calls_to(&calls, INTERPOLATE_MOTION_STATE)
            .iter()
            .map(|a| a[0] - ENTITY_SWEPT_TRANSFORM)
            .collect();
        assert_eq!(interpolated, [main, p, u]);
        // What the slave constraints added is sorted.
        assert_eq!(
            calls_to(&calls, QUICK_SORT_RECURSIVE),
            [
                vec![e.mem.u32(constraints) + 4, 0, 2, COMPARE_CONSTRAINTS],
                vec![e.mem.u32(new_entities) + 4, 0, 1, COMPARE_ENTITIES],
            ]
        );
        assert_eq!(e.mem.u32(ROUTER_BLOCK), STACK_BASE);
    }

    /// A simulation with its world, a manager for the queued events and the
    /// doubles for the handlers of one event.
    fn toi_simulation(
        e: &mut Engine,
        queued: &[(f32, u32, u32)],
    ) -> (u32, Ptr<HkpContinuousSimulation>, u32) {
        let world = world(e);
        let this = simulation(e, world);
        set_vtable(e, this.addr(), &[(0x44, 0x0600_1044), (0x3c, 0x0600_103c)]);
        e.register_double(0x0600_1044, |_, _| Ret::default());
        e.register_double(0x0600_103c, |_, _| Ret::default());
        let manager = virtual_object(
            e,
            &[
                (0x44, 0x0600_0044),
                (0x20, 0x0600_0020),
                (0x48, 0x0600_0048),
            ],
        );
        e.register_double(0x0600_0044, |_, _| Ret::default());
        e.register_double(0x0600_0020, |_, _| Ret::default());
        e.register_double(0x0600_0048, |_, _| Ret::default());
        let data = events(e, this, queued.len() as i32);
        for (i, (time, uid_a, uid_b)) in queued.iter().enumerate() {
            let a = body(e, 1, *uid_a, 0, 0x111);
            let b = body(e, 1, *uid_b, 0, 0x222);
            set_event(e, data + i as u32 * TOI_EVENT_SIZE, *time, a, b, manager);
        }
        (world, this, manager)
    }

    #[test]
    fn e9c0_handles_the_events_in_time_order_and_stops_when_none_is_due() {
        let mut e = engine();
        let (world, this, _) = toi_simulation(&mut e, &[(2.0, 1, 1), (0.5, 5, 9), (0.5, 5, 7)]);
        e.set(this, HkpContinuousSimulation::float_18, 2.0);
        e.set(this, HkpContinuousSimulation::float_1c, 0.75);
        e.mem.set_f32(world + WORLD_ROTATE_NORMAL, 0.25);
        let data = e.get(this, HkpContinuousSimulation::event_data);
        // The first event that the manager is shown is dropped, the next one
        // goes through.
        let shown = Rc::new(RefCell::new(vec![]));
        let drop_next = Rc::new(Cell::new(true));
        {
            let (shown, drop_next) = (shown.clone(), drop_next.clone());
            e.register_double(0x0600_0044, move |e, a| {
                let uid_b = e.mem.u32(e.mem.u32(a[1] + EVENT_ENTITY_B) + ENTITY_UID);
                shown.borrow_mut().push((uid_b, a[2]));
                if drop_next.replace(false) {
                    let flags = e.mem.u8(a[1] + EVENT_FLAGS);
                    e.mem.set_u8(a[1] + EVENT_FLAGS, flags | 8);
                }
                Ret::default()
            });
        }
        let handled = Rc::new(RefCell::new(vec![]));
        {
            let handled = handled.clone();
            e.register_double(0x0600_1044, move |e, a| {
                let uid_b = e.mem.u32(e.mem.u32(a[2] + EVENT_ENTITY_B) + ENTITY_UID);
                handled
                    .borrow_mut()
                    .push((a.to_vec(), e.mem.f32(a[2] + EVENT_TIME), uid_b));
                Ret::default()
            });
        }
        let r = e.call(0x00d0_e9c0, &args![this, 1.0f32]).u32();
        assert_eq!(r, 0);
        // The tie between the two events at 0.5 goes to the smaller uid pair.
        let shown = shown.borrow();
        assert_eq!(shown.len(), 2);
        assert_eq!(shown[0].0, 7);
        assert_eq!(shown[1].0, 9);
        let handled = handled.borrow();
        assert_eq!(handled.len(), 1);
        assert_eq!(handled[0].1, 0.5);
        assert_eq!(handled[0].2, 9);
        assert_eq!(handled[0].0[0], this.addr());
        assert_eq!(handled[0].0[1], world);
        assert_eq!(handled[0].0[3], 0.75f32.to_bits());
        assert_eq!(handled[0].0[4], 0.25f32.to_bits());
        // One event is left and the counters moved on.
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 1);
        assert_eq!(e.mem.f32(data + EVENT_TIME), 2.0);
        assert_eq!(e.get(this, HkpContinuousSimulation::processed_toi_count), 1);
        assert_eq!(e.get(this, HkpContinuousSimulation::float_14), 0.5);
        // The step info: start, end, difference and its inverse.
        let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
        let expected = [
            0.5f32.to_bits(),
            2.0f32.to_bits(),
            1.5f32.to_bits(),
            ((1.0f64 / 1.5) as f32).to_bits(),
        ];
        for (k, word) in expected.iter().enumerate() {
            assert_eq!(e.mem.u32(world + WORLD_STEP_INFO + 4 * k as u32), *word);
            assert_eq!(
                e.mem.u32(input + COLLISION_INPUT_STEP_INFO + 4 * k as u32),
                *word
            );
        }
        assert_eq!(e.mem.u32(world + WORLD_CRITICAL_OPERATIONS_LOCK_COUNT), 0);
    }

    #[test]
    fn e9c0_returns_3_when_the_abort_flag_is_set() {
        let mut e = engine();
        let (world, this, _) = toi_simulation(&mut e, &[(0.5, 5, 9)]);
        e.set_global(ABORT_FLAG, 1u32);
        let r = e.call(0x00d0_e9c0, &args![this, 1.0f32]).u32();
        assert_eq!(r, 3);
        assert_eq!(e.get(this, HkpContinuousSimulation::processed_toi_count), 1);
        assert_eq!(e.mem.u32(world + WORLD_CRITICAL_OPERATIONS_LOCK_COUNT), 0);
    }

    #[test]
    fn e9c0_hands_an_event_with_a_quality_byte_to_handle_simple_toi() {
        let mut e = engine();
        let (_, this, manager) = toi_simulation(&mut e, &[(0.5, 5, 9)]);
        let data = e.get(this, HkpContinuousSimulation::event_data);
        e.mem.set_u8(data + EVENT_QUALITY_BYTE, 1);
        let asked = Rc::new(RefCell::new(vec![]));
        let asked_in = asked.clone();
        e.register_double(0x0600_0048, move |_, a| {
            asked_in.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        log(&mut e);
        let r = e.call(0x00d0_e9c0, &args![this, 1.0f32]).u32();
        assert_eq!(r, 0);
        let calls = take_log(&mut e);
        // handleSimpleToi asked the manager (slot 0x48) and reported (0x20);
        // the virtual simulateToi was not used.
        assert_eq!(asked.borrow().len(), 1);
        assert_eq!(asked.borrow()[0][0], manager);
        assert_eq!(asked.borrow()[0][2], 0);
        assert_eq!(calls_to(&calls, 0x0600_0020).len(), 1);
        assert!(calls_to(&calls, 0x0600_1044).is_empty());
    }

    #[test]
    fn ed10_repeats_an_interrupted_step_first() {
        let mut e = engine();
        let (_, this, _) = toi_simulation(&mut e, &[]);
        e.set(this, HkpContinuousSimulation::word_28, 7);
        e.register_double(RECOLLIDE_AFTER_STEP_FAILURE, |_, a| {
            assert_eq!(a.len(), 1);
            ret(5)
        });
        quiet(&mut e, &[SNAP_SIMULATE_TIME]);
        log(&mut e);
        let r = e.call(0x00d0_ed10, &args![this]).u32();
        assert_eq!(r, 5);
        let calls = take_log(&mut e);
        assert_eq!(
            calls_to(&calls, RECOLLIDE_AFTER_STEP_FAILURE),
            [vec![this.addr()]]
        );
        assert!(calls_to(&calls, SNAP_SIMULATE_TIME).is_empty());
        assert_eq!(e.get(this, HkpContinuousSimulation::float_14), 0.0);
        // Records: TtPhysics, TtRecollideAfterStepFailure and two ends.
        assert_eq!(e.mem.u32(STREAM_RECORDS), PROFILE_TT_PHYSICS);
        assert_eq!(
            e.mem.u32(STREAM_RECORDS + 0xc),
            PROFILE_TT_RECOLLIDE_AFTER_STEP_FAILURE
        );
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0x18), PROFILE_END);
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0x24), PROFILE_END);
        assert_eq!(e.mem.u32(STREAM + 4), STREAM_RECORDS + 0x30);
    }

    #[test]
    fn ed10_advances_to_the_snapped_time() {
        let mut e = engine();
        let (_, this, _) = toi_simulation(&mut e, &[]);
        e.set(this, HkpContinuousSimulation::word_28, 7);
        e.register_double(RECOLLIDE_AFTER_STEP_FAILURE, |_, _| ret(0));
        e.register_double(SNAP_SIMULATE_TIME, |_, a| {
            assert_eq!(a.len(), 1);
            Ret {
                st0: 2.5,
                ..Ret::default()
            }
        });
        let r = e.call(0x00d0_ed10, &args![this]).u32();
        assert_eq!(r, 0);
        assert_eq!(e.get(this, HkpContinuousSimulation::word_28), 0);
        assert_eq!(e.get(this, HkpContinuousSimulation::float_14), 2.5);
        // An event that is not due leaves the result 0.
        let data = events(&mut e, this, 1);
        set_event(&mut e, data, 5.0, 0x1000, 0x2000, 0);
        e.set(this, HkpContinuousSimulation::float_14, 0.0);
        e.set(this, HkpContinuousSimulation::word_28, 7);
        let r = e.call(0x00d0_ed10, &args![this]).u32();
        assert_eq!(r, 0);
        assert_eq!(e.get(this, HkpContinuousSimulation::float_14), 2.5);
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 1);
    }

    #[test]
    fn ed10_keeps_the_time_when_the_events_stop_it() {
        let mut e = engine();
        let (world, this, _) = toi_simulation(&mut e, &[(0.5, 5, 9)]);
        e.register_double(SNAP_SIMULATE_TIME, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        e.set_global(ABORT_FLAG, 1u32);
        let r = e.call(0x00d0_ed10, &args![this]).u32();
        assert_eq!(r, 3);
        assert_eq!(e.get(this, HkpContinuousSimulation::word_28), 3);
        assert_eq!(e.get(this, HkpContinuousSimulation::float_14), 0.5);
        assert_eq!(e.mem.u32(world + WORLD_CRITICAL_OPERATIONS_LOCK_COUNT), 0);
        // TtPhysics, TtTois and the two ends.
        assert_eq!(e.mem.u32(STREAM_RECORDS), PROFILE_TT_PHYSICS);
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0xc), PROFILE_TT_TOIS);
    }

    /// A world with one island of no bodies, and a simulation that has it.
    fn collide_fixture(e: &mut Engine) -> (u32, Ptr<HkpContinuousSimulation>, u32, u32) {
        let world = world(e);
        let this = simulation(e, world);
        let island = e.mem.alloc(0x80);
        let islands = word_array(e, &[island], 1);
        for k in 0..3 {
            let word = e.mem.u32(islands + 4 * k);
            e.mem.set_u32(world + WORLD_ACTIVE_ISLANDS + 4 * k, word);
        }
        let step_info = e.mem.alloc(0x10);
        for (k, value) in [1.0f32, 3.0, 2.0, 0.5].iter().enumerate() {
            e.mem.set_f32(step_info + 4 * k as u32, *value);
        }
        e.mem.set_f32(world + WORLD_VALUE_300, 2.0);
        e.mem.set_i32(world + WORLD_INTEGER_2F4, 3);
        e.set_global(TOI_TIME_NOT_SET, 123.0f32);
        (world, this, island, step_info)
    }

    #[test]
    fn ef40_collides_every_island_and_reports_the_post_collide_callbacks() {
        let mut e = engine();
        let (world, this, island, step_info) = collide_fixture(&mut e);
        e.mem
            .set_u32(world + WORLD_ISLAND_POST_COLLIDE_LISTENER_COUNT, 1);
        e.register_double(FIRE_ISLAND_POST_COLLIDE_CALLBACK, |_, _| Ret::default());
        // An interrupted step: the queued events are reported, last first.
        e.set(this, HkpContinuousSimulation::word_28, 1);
        let seen = Seen::default();
        let data = events(&mut e, this, 2);
        let manager = reporting_manager(&mut e, 0x0600_0020, seen.clone());
        let a = body(&mut e, 1, 1, 0, 0x111);
        let b = body(&mut e, 1, 2, 0, 0x222);
        set_event(&mut e, data, 1.0, a, b, manager);
        set_event(&mut e, data + TOI_EVENT_SIZE, 2.0, b, a, manager);
        log(&mut e);
        e.call(0x00d0_ef40, &args![this, step_info]);
        let calls = take_log(&mut e);
        assert_eq!(
            *seen.borrow(),
            [
                vec![manager, 0x222, data + TOI_EVENT_SIZE + EVENT_PROPERTIES],
                vec![manager, 0x111, data + EVENT_PROPERTIES]
            ]
        );
        assert_eq!(e.get(this, HkpContinuousSimulation::event_count), 0);
        // The step info went to the world and the collision input.
        let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
        for k in 0..4 {
            let word = e.mem.u32(step_info + 4 * k);
            assert_eq!(e.mem.u32(world + WORLD_STEP_INFO + 4 * k), word);
            assert_eq!(e.mem.u32(input + COLLISION_INPUT_STEP_INFO + 4 * k), word);
        }
        // world[0x2ec] = 2.0 * delta, world[0x2f0] = 3 * inverse delta.
        assert_eq!(e.mem.f32(world + WORLD_SCALED_2EC), 4.0);
        assert_eq!(e.mem.f32(world + WORLD_SCALED_2F0), 1.5);
        assert_eq!(
            calls_to(&calls, FIRE_ISLAND_POST_COLLIDE_CALLBACK),
            [vec![world, island, step_info]]
        );
        assert_eq!(e.mem.u32(world + WORLD_CRITICAL_OPERATIONS_LOCK_COUNT), 0);
        // TtCollide, the narrow phase, the callback and the ends.
        let labels: Vec<u32> = [0u32, 0xc, 0x18, 0x24, 0x30, 0x3c]
            .iter()
            .map(|at| e.mem.u32(STREAM_RECORDS + at))
            .collect();
        assert_eq!(
            labels,
            [
                PROFILE_TT_COLLIDE,
                PROFILE_TT_NARROW_PHASE,
                PROFILE_END,
                PROFILE_TT_ISLAND_POST_COLLIDE_CB,
                PROFILE_END,
                PROFILE_END
            ]
        );
    }

    #[test]
    fn ef40_stops_at_the_abort_flag() {
        let mut e = engine();
        let (world, this, _, step_info) = collide_fixture(&mut e);
        e.set_global(ABORT_FLAG, 1u32);
        e.mem
            .set_u32(world + WORLD_ISLAND_POST_COLLIDE_LISTENER_COUNT, 1);
        log(&mut e);
        e.call(0x00d0_ef40, &args![this, step_info]);
        let calls = take_log(&mut e);
        assert!(calls_to(&calls, FIRE_ISLAND_POST_COLLIDE_CALLBACK).is_empty());
        assert_eq!(e.mem.u32(world + WORLD_CRITICAL_OPERATIONS_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(STREAM_RECORDS), PROFILE_TT_COLLIDE);
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0xc), PROFILE_END);
        assert_eq!(e.mem.u32(STREAM + 4), STREAM_RECORDS + 0x18);
    }

    #[test]
    fn f2e0_hands_the_arguments_to_the_base_simulation() {
        let mut e = engine();
        quiet(&mut e, &[BASE_SIMULATION_CALLBACK_FUNCTION]);
        log(&mut e);
        e.call(0x00d0_f2e0, &args![0x10u32, 0x20u32, 0x30u32, 0x40u32]);
        assert_eq!(
            take_log(&mut e),
            vec![(
                BASE_SIMULATION_CALLBACK_FUNCTION,
                vec![0x10, 0x20, 0x30, 0x40, CALLBACK_00D0DAA0, 0]
            )]
        );
    }

    /// The state the localized solve works on.
    struct Solve {
        world: u32,
        input: u32,
        island: u32,
        a: u32,
        b: u32,
        event: u32,
        manager: u32,
        contact_manager: u32,
        limits: u32,
        solver: u32,
        flags: u32,
        table: u32,
        bodies: u32,
    }

    /// An island of two bodies and an event between them, with quiet doubles
    /// for everything the solve calls.
    fn solve_fixture(e: &mut Engine, rounds: i32) -> Solve {
        let world = world(e);
        let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
        let island = e.mem.alloc(0x80);
        e.mem.set_u32(island + 0x18, world);
        let simulation_object = virtual_object(e, &[(0x48, 0x0600_0148)]);
        e.mem.set_u32(world + WORLD_SIMULATION, simulation_object);
        let a = body(e, 1, 1, 0, island);
        let b = body(e, 1, 2, 1, island);
        for body in [a, b] {
            e.mem.set_u32(body + ENTITY_WORLD, world);
            let ms = body + ENTITY_MOTION_STATE;
            write_vec4(e, ms + 0x80, [1.0, 2.0, 3.0, 0.0]);
            write_vec4(e, ms + 0x50, [10.0, 20.0, 30.0, 40.0]);
        }
        let entity_array = word_array(e, &[a, b], 2);
        e.mem
            .set_u32(island + ISLAND_ENTITY_DATA, e.mem.u32(entity_array));
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, 2);
        // The resource manager has no slot 0x48 (its vtable `010d4310` holds 0 there); the
        // event's contact manager answers it.
        let manager = virtual_object(e, &[(0x14, 0x0600_0214), (0x10, 0x0600_0210)]);
        let contact_manager = virtual_object(e, &[(0x48, 0x0600_0248)]);
        let event = e.mem.alloc(TOI_EVENT_SIZE);
        set_event(e, event, 1.0, a, b, contact_manager);
        let limits = e.mem.alloc(0x20);
        for (k, word) in [1u32, 100, 100, 100, rounds as u32, 1].iter().enumerate() {
            e.mem.set_u32(limits + 4 * k as u32, *word);
        }
        let solver = e.mem.alloc(0x80);
        let accumulators = e.mem.alloc(0x400);
        e.mem.set_u32(solver + 0xc, accumulators);
        let solver_table = e.mem.alloc(0x400);
        e.mem.set_u32(solver + 0x18, solver_table);
        let table = e.mem.alloc(8);
        let flags = e.mem.alloc(4);
        e.mem.set_u32(flags, table);
        let bodies = word_array(e, &[], 8);
        // The identity rotation.
        e.register_double(SET_ROTATION, |e, a| {
            write_vec4(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            write_vec4(e, a[0] + 0x10, [0.0, 1.0, 0.0, 0.0]);
            write_vec4(e, a[0] + 0x20, [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        quiet(
            e,
            &[
                INTERPOLATE_MOTION_STATE,
                BACK_STEP_MOTION_STATE,
                INVALIDATE_TIM,
                HK_SOLVE_UPLOAD,
                INTERNAL_ADD_ACCUMULATORS,
                INTERNAL_ADD_JACOBIAN_SCHEMAS,
                SUB_SOLVE,
                ONE_STEP_INTEGRATE,
                SOLVE_STEP_JACOBIANS,
                GET_COLLISION_ENTRIES_SORTED,
                0x0600_0148,
                0x0600_0248,
                0x0600_0214,
                0x0600_0210,
            ],
        );
        Solve {
            world,
            input,
            island,
            a,
            b,
            event,
            manager,
            contact_manager,
            limits,
            solver,
            flags,
            table,
            bodies,
        }
    }

    #[test]
    fn f300_only_updates_the_motion_states_when_the_manager_has_nothing_to_solve() {
        let mut e = engine();
        let s = solve_fixture(&mut e, 1);
        let asked = Rc::new(RefCell::new(vec![]));
        let asked_in = asked.clone();
        e.register_double(0x0600_0248, move |_, a| {
            asked_in.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        log(&mut e);
        let r = e
            .call(
                0x00d0_f300,
                &args![s.limits, s.solver, s.event, s.manager, s.world, s.bodies, s.flags, 2.5f32],
            )
            .u32();
        assert_eq!(r, 1);
        let calls = take_log(&mut e);
        // The event's contact manager (event + 0x14, 00d0f41e) was asked with the event, the
        // time and an array.
        let asked = asked.borrow();
        assert_eq!(asked.len(), 1);
        assert_eq!(
            asked[0][..3],
            [s.contact_manager, s.event, 2.5f32.to_bits()]
        );
        // Both bodies were interpolated to the event time and set to state 2.
        assert_eq!(
            calls_to(&calls, INTERPOLATE_MOTION_STATE),
            [
                vec![
                    s.a + ENTITY_SWEPT_TRANSFORM,
                    1.0f32.to_bits(),
                    s.a + ENTITY_MOTION_STATE
                ],
                vec![
                    s.b + ENTITY_SWEPT_TRANSFORM,
                    1.0f32.to_bits(),
                    s.b + ENTITY_MOTION_STATE
                ]
            ]
        );
        assert_eq!(e.mem.bytes(s.table, 2), [2, 2]);
        // Translation = base - rotation * vector.
        for body in [s.a, s.b] {
            assert_eq!(
                read_vec4(&e, body + ENTITY_MOTION_STATE + 0x30),
                [9.0, 18.0, 27.0, 40.0]
            );
        }
        assert!(calls_to(&calls, HK_SOLVE_UPLOAD).is_empty());
        assert_eq!(e.mem.u32(ROUTER_BLOCK), STACK_BASE);
        assert_eq!(e.mem.u32(s.bodies + 4), 0);
    }

    #[test]
    fn f300_solves_with_the_bodies_the_manager_gave() {
        let mut e = engine();
        let s = solve_fixture(&mut e, 1);
        // The manager wants body a solved.
        let (a, island) = (s.a, s.island);
        e.register_double(0x0600_0248, move |e, args| {
            let data = e.mem.u32(args[3]);
            e.mem.set_u32(data, a);
            e.mem.set_u32(args[3] + 4, 1);
            Ret::default()
        });
        // The solver takes everything.
        for address in [SOLVER_SETUP_00D88310, SOLVER_SETUP_00D88470] {
            e.register_double(address, |e, args| {
                e.mem.set_u8(args[0], 1);
                ret(args[0])
            });
        }
        // Appending the manager's bodies to the result array.
        e.register_double(ARRAY_INSERT_AT, |e, args| {
            let (array, at, source, count) = (args[0], args[2], args[3], args[4]);
            let data = e.mem.u32(array);
            for k in 0..count {
                let word = e.mem.u32(source + 4 * k);
                e.mem.set_u32(data + 4 * (at + k), word);
            }
            e.mem.set_u32(array + 4, at + count);
            Ret::default()
        });
        log(&mut e);
        let r = e
            .call(
                0x00d0_f300,
                &args![s.limits, s.solver, s.event, s.manager, s.world, s.bodies, s.flags, 2.5f32],
            )
            .u32();
        assert_eq!(r, 0);
        let calls = take_log(&mut e);
        // The order of the solver's steps.
        let steps = [
            HK_SOLVE_UPLOAD,
            INTERNAL_ADD_ACCUMULATORS,
            SOLVER_SETUP_00D88310,
            SOLVER_SETUP_00D88470,
            INTERNAL_ADD_ACCUMULATORS,
            INTERNAL_ADD_JACOBIAN_SCHEMAS,
            ARRAY_INSERT_AT,
            SUB_SOLVE,
            SUB_SOLVE,
            BACK_STEP_MOTION_STATE,
            ONE_STEP_INTEGRATE,
            INVALIDATE_TIM,
        ];
        assert_eq!(only(&calls, &steps), steps);
        let sub_solves = calls_to(&calls, SUB_SOLVE);
        assert_eq!(sub_solves, [vec![s.solver, 1], vec![s.solver, 0]]);
        // The result array holds the manager's body, stepped back to the
        // event time, integrated and invalidated.
        assert_eq!(array_items(&e, s.bodies), [s.a]);
        assert_eq!(
            calls_to(&calls, BACK_STEP_MOTION_STATE),
            [vec![1.0f32.to_bits(), s.a + ENTITY_MOTION_STATE]]
        );
        let integrate = calls_to(&calls, ONE_STEP_INTEGRATE);
        assert_eq!(integrate.len(), 1);
        assert_eq!(
            integrate[0][..3],
            [
                e.mem.u32(s.solver + 4),
                e.mem.u32(s.solver),
                e.mem.u32(s.solver + 0xc)
            ]
        );
        assert_eq!(integrate[0][4], 1);
        assert_eq!(calls_to(&calls, INVALIDATE_TIM), [vec![s.a, s.input]]);
        // The activation ran the narrow phase of the island's simulation.
        assert_eq!(calls_to(&calls, 0x0600_0148).len(), 1);
        assert_eq!(calls_to(&calls, 0x0600_0148)[0][4], island);
        // Body a reached state 0x10, the other one stayed at 2.
        assert_eq!(e.mem.bytes(s.table, 2), [0x10, 2]);
        assert_eq!(e.mem.u32(ROUTER_BLOCK), STACK_BASE);
    }

    #[test]
    fn f300_freezes_the_whole_island_when_the_solver_refuses_and_the_manager_says_2() {
        let mut e = engine();
        let s = solve_fixture(&mut e, 1);
        let a = s.a;
        e.register_double(0x0600_0248, move |e, args| {
            let data = e.mem.u32(args[3]);
            e.mem.set_u32(data, a);
            e.mem.set_u32(args[3] + 4, 1);
            Ret::default()
        });
        // The solver does not take the additions.
        e.register_double(SOLVER_SETUP_00D88310, |e, args| {
            e.mem.set_u8(args[0], 0);
            ret(args[0])
        });
        e.register_double(0x0600_0214, |_, _| ret(2));
        e.register_double(WORLD_GET_CURRENT_PSI_TIME, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        log(&mut e);
        let r = e
            .call(
                0x00d0_f300,
                &args![s.limits, s.solver, s.event, s.manager, s.world, s.bodies, s.flags, 2.5f32],
            )
            .u32();
        assert_eq!(r, 0);
        let calls = take_log(&mut e);
        // The manager was asked (slot 0x14), the island was backstepped to
        // the event time and its bodies' data invalidated.
        assert_eq!(calls_to(&calls, 0x0600_0214).len(), 1);
        assert_eq!(
            calls_to(&calls, BACK_STEP_MOTION_STATE),
            [
                vec![1.0f32.to_bits(), s.a + ENTITY_MOTION_STATE],
                vec![1.0f32.to_bits(), s.b + ENTITY_MOTION_STATE]
            ]
        );
        // Both bodies of the island are marked 0x10 and handed back.
        assert_eq!(e.mem.bytes(s.table, 2), [0x10, 0x10]);
        assert_eq!(
            calls_to(&calls, INVALIDATE_TIM),
            [vec![s.a, s.input], vec![s.b, s.input]]
        );
        assert_eq!(e.mem.u32(s.bodies + 4), 0);
        // The motion states: the end was copied over the start, the time
        // set and the inverse of the remaining time stored.
        for body in [s.a, s.b] {
            let ms = body + ENTITY_MOTION_STATE;
            assert_eq!(e.mem.f32(ms + 0x4c), 1.0);
            assert_eq!(e.mem.f32(ms + 0x5c), 0.5);
        }
        assert!(calls_to(&calls, ONE_STEP_INTEGRATE).is_empty());
        assert_eq!(e.mem.u32(ROUTER_BLOCK), STACK_BASE);
    }

    #[test]
    fn fe00_runs_the_final_phases_once_the_target_time_is_reached() {
        let mut e = engine();
        let (world, this, _) = toi_simulation(&mut e, &[]);
        e.register_double(SNAP_SIMULATE_TIME, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        e.register_double(0x0600_1040, |_, _| Ret::default());
        e.register_double(FIRE_POST_SIMULATION_CALLBACK, |_, _| Ret::default());
        set_vtable(
            &mut e,
            this.addr(),
            &[(0x40, 0x0600_1040), (0x3c, 0x0600_103c)],
        );
        e.mem
            .set_u32(world + WORLD_POST_SIMULATION_LISTENER_COUNT, 1);
        let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
        e.set(this, HkpContinuousSimulation::float_20, 2.0);
        e.mem.set_u32(this.addr() + SIM_ENTITIES_MAP + 4, 3);
        log(&mut e);
        let r = e.call(0x00d0_fe00, &args![this]).u32();
        assert_eq!(r, 0);
        let calls = take_log(&mut e);
        assert_eq!(
            calls_to(&calls, 0x0600_1040),
            [vec![this.addr(), input, this.addr() + SIM_ENTITIES_MAP]]
        );
        assert_eq!(
            calls_to(&calls, FIRE_POST_SIMULATION_CALLBACK),
            [vec![world]]
        );
        // The post-simulation callback is recorded between TtPostSimCb and Et.
        let cursor = e.mem.u32(STREAM + 4);
        assert_eq!(e.mem.u32(cursor - 0x18), PROFILE_TT_POST_SIM_CB);
        assert_eq!(e.mem.u32(cursor - 0xc), PROFILE_END);
        // Not yet at the target time, or a NaN target: nothing else runs.
        for target in [9.0f32, f32::NAN] {
            e.set(this, HkpContinuousSimulation::float_20, target);
            log(&mut e);
            e.call(0x00d0_fe00, &args![this]);
            let calls = take_log(&mut e);
            assert!(calls_to(&calls, 0x0600_1040).is_empty());
            assert!(calls_to(&calls, FIRE_POST_SIMULATION_CALLBACK).is_empty());
        }
    }

    #[test]
    fn fe00_returns_3_at_the_abort_flag_and_skips_an_empty_map() {
        let mut e = engine();
        let (world, this, _) = toi_simulation(&mut e, &[]);
        e.register_double(SNAP_SIMULATE_TIME, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        e.register_double(FIRE_POST_SIMULATION_CALLBACK, |_, _| Ret::default());
        e.mem
            .set_u32(world + WORLD_POST_SIMULATION_LISTENER_COUNT, 1);
        e.set(this, HkpContinuousSimulation::float_20, 2.0);
        // Only the flag bit of the map's size word: it counts as empty.
        e.mem
            .set_u32(this.addr() + SIM_ENTITIES_MAP + 4, 0x8000_0000);
        e.set_global(ABORT_FLAG, 1u32);
        log(&mut e);
        let r = e.call(0x00d0_fe00, &args![this]).u32();
        assert_eq!(r, 3);
        let calls = take_log(&mut e);
        assert!(calls_to(&calls, FIRE_POST_SIMULATION_CALLBACK).is_empty());
        assert!(calls_to(&calls, 0x0600_1040).is_empty());
    }

    #[test]
    fn fed0_puts_the_bodies_back_at_the_step_and_collides_them_again() {
        let mut e = engine();
        let (world, this, _) = toi_simulation(&mut e, &[]);
        let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
        e.set(this, HkpContinuousSimulation::float_14, 1.0);
        e.set(this, HkpContinuousSimulation::float_18, 3.0);
        set_vtable(&mut e, this.addr(), &[(0x20, 0x0600_1020)]);
        // The old step info.
        for k in 0..4 {
            e.mem
                .set_f32(input + COLLISION_INPUT_STEP_INFO + 4 * k, 9.0 + k as f32);
        }
        let moving = body(&mut e, 1, 1, 0, 0);
        let fixed = body(&mut e, 5, 2, 1, 0);
        let entities = word_array(&mut e, &[moving, fixed], 2);
        let list = e.mem.u32(entities);
        let step_words =
            |e: &Engine, at: u32| -> Vec<f32> { (0..4).map(|k| e.mem.f32(at + 4 * k)).collect() };
        let during = Rc::new(RefCell::new(vec![]));
        {
            let during = during.clone();
            e.register_double(0x0600_1020, move |e, a| {
                during.borrow_mut().push((
                    a.to_vec(),
                    (0..4)
                        .map(|k| e.mem.f32(input + COLLISION_INPUT_STEP_INFO + 4 * k))
                        .collect::<Vec<f32>>(),
                ));
                Ret::default()
            });
        }
        let utility = Rc::new(RefCell::new(vec![]));
        {
            let utility = utility.clone();
            e.register_double(RIGID_MOTION_UTIL_STEP, move |e, a| {
                utility.borrow_mut().push((
                    a.to_vec(),
                    (0..4)
                        .map(|k| e.mem.f32(a[0] + 4 * k))
                        .collect::<Vec<f32>>(),
                ));
                Ret::default()
            });
        }
        quiet(&mut e, &[BACK_STEP_MOTION_STATE, ENTITY_BATCH_RECALC_AABB]);
        log(&mut e);
        e.call(0x00d0_fed0, &args![this, list, 2i32, world, 1u32]);
        let calls = take_log(&mut e);
        // Only the body that is not fixed is stepped back to the start.
        assert_eq!(
            calls_to(&calls, BACK_STEP_MOTION_STATE),
            [vec![1.0f32.to_bits(), moving + ENTITY_MOTION_STATE]]
        );
        let utility = utility.borrow();
        assert_eq!(utility.len(), 1);
        assert_eq!(utility[0].0[1..], [list, 2, 0xe0]);
        assert_eq!(utility[0].1, [1.0, 3.0, 2.0, 0.5]);
        assert_eq!(
            calls_to(&calls, ENTITY_BATCH_RECALC_AABB),
            [vec![input, list, 2]]
        );
        // The virtual function is called with 1 and then with 2, both times
        // with the new step info in the collision input.
        let during = during.borrow();
        assert_eq!(during.len(), 2);
        assert_eq!(during[0].0, [this.addr(), list, 2, world, 1]);
        assert_eq!(during[1].0, [this.addr(), list, 2, world, 2]);
        assert_eq!(during[0].1, [1.0, 3.0, 2.0, 0.5]);
        // Afterwards the old step info is back and the lock is released.
        assert_eq!(
            step_words(&e, input + COLLISION_INPUT_STEP_INFO),
            [9.0, 10.0, 11.0, 12.0]
        );
        assert_eq!(e.mem.u32(world + WORLD_CRITICAL_OPERATIONS_LOCK_COUNT), 0);
    }

    #[test]
    fn fed0_runs_the_narrow_phase_through_the_base_simulation_for_mode_4() {
        let mut e = engine();
        let (world, this, _) = toi_simulation(&mut e, &[]);
        let input = e.mem.u32(world + WORLD_COLLISION_INPUT);
        set_vtable(&mut e, this.addr(), &[(0x20, 0x0600_1020)]);
        e.register_double(0x0600_1020, |_, _| Ret::default());
        quiet(&mut e, &[BASE_SIMULATION_CALLBACK_FUNCTION]);
        let list = e.mem.alloc(8);
        log(&mut e);
        e.call(0x00d0_fed0, &args![this, list, 2i32, world, 4u32]);
        let calls = take_log(&mut e);
        assert_eq!(calls_to(&calls, 0x0600_1020).len(), 1);
        assert_eq!(
            calls_to(&calls, BASE_SIMULATION_CALLBACK_FUNCTION),
            [vec![this.addr(), list, 2, input, CALLBACK_00D0DAA0, 0]]
        );
        assert!(calls_to(&calls, BACK_STEP_MOTION_STATE).is_empty());
    }

    #[test]
    fn fed0_runs_the_broad_phase_for_mode_2() {
        let mut e = engine();
        let (world, this, _) = toi_simulation(&mut e, &[]);
        set_vtable(&mut e, this.addr(), &[(0x20, 0x0600_1020)]);
        e.register_double(0x0600_1020, |_, _| Ret::default());
        let broad_phase = virtual_object(&mut e, &[(0x2c, 0x0600_002c)]);
        e.mem.set_u32(world + WORLD_BROAD_PHASE, broad_phase);
        e.register_double(0x0600_002c, |_, _| Ret::default());
        let first = bounded_body(&mut e, [10, 20, 30], [50, 60, 70]);
        let entities = word_array(&mut e, &[first], 1);
        let list = e.mem.u32(entities);
        log(&mut e);
        e.call(0x00d0_fed0, &args![this, list, 1i32, world, 2u32]);
        let calls = take_log(&mut e);
        assert_eq!(calls_to(&calls, 0x0600_002c).len(), 1);
        assert_eq!(calls_to(&calls, 0x0600_1020).len(), 1);
        assert!(calls_to(&calls, RIGID_MOTION_UTIL_STEP).is_empty());
    }

    #[test]
    fn d100a0_does_nothing_more_when_the_manager_answers_1() {
        let mut e = engine();
        let (world, this, _) = toi_simulation(&mut e, &[]);
        let asked = Rc::new(RefCell::new(vec![]));
        let asked_in = asked.clone();
        e.register_double(0x0600_030c, move |e, a| {
            asked_in.borrow_mut().push((
                a.to_vec(),
                (0..8)
                    .map(|k| e.mem.u32(a[3] + 4 * k))
                    .collect::<Vec<u32>>(),
            ));
            ret(1)
        });
        let manager = virtual_object(&mut e, &[(0xc, 0x0600_030c)]);
        e.set(
            this,
            HkpContinuousSimulation::toi_resource_manager,
            Ptr::new(manager),
        );
        let event = e.mem.alloc(TOI_EVENT_SIZE);
        log(&mut e);
        e.call(0x00d1_00a0, &args![this, world, event, 2.0f32, 0.5f32]);
        let calls = take_log(&mut e);
        let asked = asked.borrow();
        assert_eq!(asked.len(), 1);
        assert_eq!(
            asked[0].0[..3],
            [manager, event, this.addr() + SIM_EVENT_ARRAY]
        );
        assert_eq!(asked[0].1, [3, 1000, 1000, 1000, 4, 4, 0, 0]);
        // Nothing else happened.
        assert_eq!(calls.len(), 1);
        assert_eq!(e.mem.u32(world + WORLD_CRITICAL_OPERATIONS_LOCK_COUNT), 0);
    }

    #[test]
    fn d100a0_sets_up_the_solver_and_runs_the_localized_solve() {
        let mut e = engine();
        let (world, this, _) = toi_simulation(&mut e, &[]);
        e.set_global(CONSTANT_01016248, 3.0f32);
        e.set_global(CONSTANT_0104F498, 4.0f32);
        // The block the solver set-up starts from.
        e.mem.set_u32(world + 0x1e0 + 0x64, 0xaabb_ccdd);
        e.mem.write(world + 0x1e0 + 0x10, &[0x11; 16]);
        e.mem.write(world + 0x1e0 + 0x20, &[0x22; 16]);
        e.mem
            .set_u32(world + WORLD_VIOLATED_CONSTRAINT_ARRAY, 0x5555);
        let island = e.mem.alloc(0x80);
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, 1);
        let a = body(&mut e, 1, 1, 0, island);
        let b = body(&mut e, 5, 2, 1, island);
        e.mem.set_u32(a + ENTITY_WORLD, world);
        let event_manager = virtual_object(&mut e, &[(0x20, 0x0600_0020), (0x48, 0x0600_0348)]);
        let event = e.mem.alloc(TOI_EVENT_SIZE);
        set_event(&mut e, event, 1.0, a, b, event_manager);
        // The resource manager: answers 0, wants nothing solved.
        let limits_seen = Rc::new(RefCell::new(vec![]));
        {
            let limits_seen = limits_seen.clone();
            e.register_double(0x0600_030c, move |e, args| {
                limits_seen.borrow_mut().push(
                    (0..8)
                        .map(|k| e.mem.u32(args[3] + 4 * k))
                        .collect::<Vec<u32>>(),
                );
                ret(0)
            });
        }
        // The resource manager's vtable has no slot 0x48 (0 there, as in `010d4310`).
        let manager = virtual_object(&mut e, &[(0xc, 0x0600_030c), (0x18, 0x0600_0318)]);
        e.set(
            this,
            HkpContinuousSimulation::toi_resource_manager,
            Ptr::new(manager),
        );
        quiet(
            &mut e,
            &[
                0x0600_0318,
                0x0600_0348,
                MEM_SET,
                SHUTDOWN_SOLVER,
                INTERPOLATE_MOTION_STATE,
            ],
        );
        e.register_double(SET_ROTATION, |e, a| {
            write_vec4(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            write_vec4(e, a[0] + 0x10, [0.0, 1.0, 0.0, 0.0]);
            write_vec4(e, a[0] + 0x20, [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        e.register_double(WORLD_GET_CURRENT_PSI_TIME, |_, a| {
            assert_eq!(a.len(), 1);
            Ret {
                st0: 4.0,
                ..Ret::default()
            }
        });
        let block_seen = Rc::new(RefCell::new(vec![]));
        {
            let block_seen = block_seen.clone();
            e.register_double(WORLD_00C90E60, move |e, args| {
                let block = args[0];
                block_seen.borrow_mut().push((
                    args.to_vec(),
                    e.mem.f32(block + 0x10c),
                    e.mem.f32(block + 0x110),
                    (e.mem.u32(block + 0x114), e.mem.u32(block + 0x118)),
                    e.mem.f32(block + 0x120),
                    e.mem.u32(block + 0x64),
                    (e.mem.bytes(block + 0x10, 16), e.mem.bytes(block + 0x20, 16)),
                ));
                Ret::default()
            });
        }
        let query_seen = Rc::new(RefCell::new(vec![]));
        {
            let query_seen = query_seen.clone();
            e.register_double(CONSTRAINT_QUERY_IN_SET, move |e, args| {
                query_seen
                    .borrow_mut()
                    .push((args.to_vec(), e.mem.f32(args[2])));
                Ret::default()
            });
        }
        let setup_seen = Rc::new(RefCell::new(vec![]));
        {
            let setup_seen = setup_seen.clone();
            e.register_double(SOLVER_SETUP_00D881D0, move |e, args| {
                setup_seen.borrow_mut().push((
                    args.to_vec(),
                    (0..4)
                        .map(|k| e.mem.f32(args[0] + 4 * k))
                        .collect::<Vec<f32>>(),
                ));
                Ret::default()
            });
        }
        let notified = Rc::new(RefCell::new(vec![]));
        {
            let notified = notified.clone();
            e.register_double(0x0600_0020, move |_, args| {
                notified.borrow_mut().push(args.to_vec());
                Ret::default()
            });
        }
        log(&mut e);
        e.call(0x00d1_00a0, &args![this, world, event, 2.0f32, 0.5f32]);
        let calls = take_log(&mut e);
        assert_eq!(*limits_seen.borrow(), [[3, 1000, 1000, 1000, 4, 4, 0, 0]]);
        // The block: copied from the world, then the values stored.
        let block_seen = block_seen.borrow();
        assert_eq!(block_seen.len(), 1);
        let (block_args, first, inverse, ones, one, copied, vectors) = block_seen[0].clone();
        assert_eq!(block_args[1..], [3.0f32.to_bits(), 4.0f32.to_bits()]);
        assert_eq!(first, 2.0);
        assert_eq!(inverse, 0.5);
        assert_eq!(ones, (1, 1));
        assert_eq!(one, 1.0);
        assert_eq!(copied, 0xaabb_ccdd);
        assert_eq!(vectors, (vec![0; 16], vec![0; 16]));
        let query_seen = query_seen.borrow();
        assert_eq!(query_seen.len(), 1);
        assert_eq!(query_seen[0].0[1], block_args[0]);
        assert_eq!(query_seen[0].0[3], 0x5555);
        assert_eq!(query_seen[0].1, 1.0);
        // The event time, the psi time, their difference and its inverse.
        let setup_seen = setup_seen.borrow();
        assert_eq!(setup_seen.len(), 1);
        assert_eq!(setup_seen[0].0[1], block_args[0]);
        assert_eq!(setup_seen[0].0[2], query_seen[0].0[0]);
        assert_eq!(setup_seen[0].0[3..5], [0, 0]);
        assert_eq!(setup_seen[0].1, [1.0, 4.0, 3.0, (1.0f64 / 3.0) as f32]);
        // The table of body states has one byte per body of the island.
        let memset = calls_to(&calls, MEM_SET);
        assert_eq!(memset.len(), 1);
        assert_eq!(memset[0][1..], [0, 1]);
        // The localized solve asked the event's contact manager (slot 0x48), not the resource
        // manager.
        let confirmed = calls_to(&calls, 0x0600_0348);
        assert_eq!(confirmed.len(), 1);
        assert_eq!(confirmed[0][..3], [event_manager, event, 0.5f32.to_bits()]);
        // The event's manager was told, the solver shut down.
        assert_eq!(
            *notified.borrow(),
            [vec![event_manager, island, event + EVENT_PROPERTIES]]
        );
        assert_eq!(calls_to(&calls, SHUTDOWN_SOLVER).len(), 1);
        let after = calls_to(&calls, 0x0600_0318);
        assert_eq!(after.len(), 1);
        assert_eq!(
            after[0][..3],
            [manager, event, this.addr() + SIM_EVENT_ARRAY]
        );
        // The records: LtTOI/StSolveToi, StPendingOperations, StCollide, lt.
        assert_eq!(e.mem.u32(STREAM_RECORDS), PROFILE_LT_TOI);
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0xc), PROFILE_ST_SOLVE_TOI);
        assert_eq!(
            e.mem.u32(STREAM_RECORDS + 0x10),
            PROFILE_ST_PENDING_OPERATIONS
        );
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0x1c), PROFILE_ST_COLLIDE);
        assert_eq!(e.mem.u32(STREAM_RECORDS + 0x28), PROFILE_LT);
        assert_eq!(e.mem.u32(world + WORLD_CRITICAL_OPERATIONS_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(ROUTER_BLOCK), STACK_BASE);
    }

    #[test]
    fn d105b0_copies_the_block_and_quiets_the_floats() {
        let mut e = engine();
        let source = e.mem.alloc(0x140);
        let dest = e.mem.alloc(0x140);
        for i in 0..0x140 {
            e.mem.set_u8(source + i, (i % 251) as u8 + 1);
        }
        const FLOATS: [u32; 14] = [
            0x0, 0x4, 0x8, 0xc, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x10c, 0x110, 0x11c, 0x120, 0x128,
        ];
        for offset in FLOATS {
            e.mem.set_u32(source + offset, 0x7f80_0001);
        }
        // A signalling NaN outside the float fields travels unchanged.
        e.mem.set_u32(source + 0x10, 0x7f80_0003);
        e.call(0x00d1_05b0, &args![dest, source]);
        for offset in FLOATS {
            assert_eq!(e.mem.u32(dest + offset), 0x7fc0_0001, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(dest + 0x10), 0x7f80_0003);
        for i in 0..0x12c {
            if FLOATS.iter().any(|o| (*o..*o + 4).contains(&i)) || (0x10..0x14).contains(&i) {
                continue;
            }
            assert_eq!(e.mem.u8(dest + i), e.mem.u8(source + i), "{i:#x}");
        }
        // Nothing past the block.
        assert_eq!(e.mem.bytes(dest + 0x12c, 0x14), vec![0; 0x14]);
    }

    #[test]
    fn d10710_inserts_with_the_array_function_and_a_final_1() {
        let mut e = engine();
        quiet(&mut e, &[ARRAY_INSERT_AT]);
        log(&mut e);
        e.call(0x00d1_0710, &args![0x10u32, 0x20u32, 0x30u32, 0x40u32]);
        assert_eq!(
            take_log(&mut e),
            vec![(ARRAY_INSERT_AT, vec![0x10, 0x20, 0x30, 0x40, 1])]
        );
    }

    #[test]
    fn d10730_destroys_and_frees_the_simulation_when_asked() {
        let mut e = engine();
        let world = world(&mut e);
        let this = simulation(&mut e, world);
        e.set(this, HkpContinuousSimulation::event_capacity, 0x8000_0000);
        e.mem.set_u16(this.addr() + 4, 0x50);
        quiet(
            &mut e,
            &[
                MAP_CLEAR_AND_DEALLOCATE,
                MAP_DESTRUCTOR,
                BASE_SIMULATION_DESTRUCTOR,
            ],
        );
        log(&mut e);
        let r = e.call(0x00d1_0730, &args![this, 0u32]).u32();
        assert_eq!(r, this.addr());
        let calls = take_log(&mut e);
        assert_eq!(
            calls_to(&calls, BASE_SIMULATION_DESTRUCTOR),
            [vec![this.addr()]]
        );
        assert!(calls_to(&calls, DELETE_FN).is_empty());
        // Bit 0 of the flags also frees the object (the u16 at +4 is its size).
        log(&mut e);
        let r = e.call(0x00d1_0730, &args![this, 1u32]).u32();
        assert_eq!(r, this.addr());
        let calls = take_log(&mut e);
        assert_eq!(
            calls_to(&calls, DELETE_FN),
            [vec![ALLOCATOR, this.addr(), 0x50]]
        );
        assert_eq!(
            only(&calls, &[BASE_SIMULATION_DESTRUCTOR, DELETE_FN]),
            [BASE_SIMULATION_DESTRUCTOR, DELETE_FN]
        );
    }
}

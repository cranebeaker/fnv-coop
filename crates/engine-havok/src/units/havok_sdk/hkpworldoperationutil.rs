//! `Havok SDK/hkpworldoperationutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! This is the group of `hkpWorldOperationUtil` functions that move constraints, actions and
//! entities between simulation islands and the broad phase, and the motion-type switch
//! (`setRigidBodyMotionType`) built on them.
//!
//! The objects are read by PC offset (names from the Xbox PDB):
//! - `hkpWorld`: +0x08 `m_simulation`, +0x20 `m_fixedIsland`, +0x28 `m_activeSimulationIslands`,
//!   +0x34 `m_inactiveSimulationIslands`, +0x40 `m_dirtySimulationIslands` (`hkArray`: data,
//!   size, capacity-and-flags), +0x58 `m_broadPhase`, +0x64 `m_broadPhaseDispatcher`, +0x78
//!   `m_collisionInput`, +0x7c `m_collisionFilter`, +0x8c `m_pendingOperationsCount`, +0x90
//!   `m_pendingBodyOperationsCount`, +0x94 `m_criticalOperationsLockCount`, +0x9c
//!   `m_blockExecutingPendingOperations`, +0x9d `m_criticalOperationsAllowed`, +0xa4
//!   `m_pendingOperationQueueCount`, +0xb4 `m_minDesiredIslandSize`, +0xc8
//!   `m_wantSimulationIslands`, +0x140 `m_constraintListeners` (size at +0x144), +0x338 the
//!   broad-phase query buffer size.
//! - `hkpSimulationIsland` (0x6c bytes): +0x1c constraint count, +0x20 index in its island
//!   array (16 bits), +0x22 index in the world's dirty list (16 bits, 0xffff = not in it),
//!   +0x24 a byte, +0x25 and +0x26 flag bytes, +0x4c entity count.
//! - `hkpEntity`: +0x08 `m_world`, +0x10 the collidable (its shape at +0x10, broad-phase
//!   handle at +0x24, AABB at +0x30), +0xac/+0xb0 `m_constraintsMaster` (small array of 0x30
//!   byte `hkConstraintInternal`, whose first word is the instance), +0xb4/+0xb8
//!   `m_constraintsSlave`, +0xcc `m_simulationIsland`, +0xd4 the unique id, +0xe0 the embedded
//!   `hkpMotion`, +0xe8 the motion type byte (5 is `MOTION_FIXED`, 4 `MOTION_KEYFRAMED`),
//!   +0x208/+0x20c `m_actions` (small array).
//!
//! Locks and arrays follow the exe: `hkArray` is `(data, size, capacityAndFlags)` with the
//! "do not free" flag in bit 31 and the capacity in the low 30 bits; an `hkLocalArray` is
//! the same plus the saved top of the memory router's stack in a fourth word.
//!
//! `00cb3d10` is `findSmallSparseActiveIsland`, which takes the world in `EAX` and the array
//! in `ECX` and two stack words; the uniform form passes them in that order (world, array,
//! the two stack words). Its one caller is `00cb4640`.

#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------------------------
// Callees outside this file, by address.
// ---------------------------------------------------------------------------------------

/// The memory router: word +0 is the top of its stack, +0x10 the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` growing one more element: `(allocator, array, element size)`
/// (`00c90940`, unnamed in the engine map).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkReferencedObject` add and remove reference (`00c90510`, `00c905b0`; the engine map has
/// only the unit), `this` only.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// Unnamed `hkpWorld` pieces run when the critical-operations lock is released: `00c91120`
/// when operations are pending, `00c91140` when exactly one queue holds body operations.
const WORLD_RUN_PENDING_OPERATIONS: u32 = 0x00c9_1120;
const WORLD_RUN_PENDING_BODY_OPERATIONS: u32 = 0x00c9_1140;
/// Unnamed `hkpWorld` function that appends a 12-byte operation record to the pending queue
/// (`this` = world, one stack word).
const WORLD_ENQUEUE_OPERATION: u32 = 0x00c9_1160;
/// `hkpWorld::getCurrentTime` (Xbox PDB), result in `ST0`.
const WORLD_GET_CURRENT_TIME: u32 = 0x00c9_1020;
/// `hkpWorld::updateCollisionFilterOnEntity` (Xbox PDB): `this` = world, then the entity and two
/// stack words.
const WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY: u32 = 0x00c9_46e0;
/// `hkpWorldConstraintUtil::addConstraint` and `removeConstraint` (Xbox PDB).
const WORLD_CONSTRAINT_UTIL_ADD: u32 = 0x00d2_7ac0;
const WORLD_CONSTRAINT_UTIL_REMOVE: u32 = 0x00d2_7820;
/// `hkpWorldCallbackUtil::fireConstraintAdded`, `fireConstraintRemoved` and
/// `fireInactiveEntityMoved` (Xbox PDB).
const FIRE_CONSTRAINT_ADDED: u32 = 0x00d0_0960;
const FIRE_CONSTRAINT_REMOVED: u32 = 0x00d0_0a00;
const FIRE_INACTIVE_ENTITY_MOVED: u32 = 0x00d0_17b0;
/// `hkpSimulationIsland::internalAddEntity` and `addAction` (Xbox PDB), `this` = island.
const ISLAND_ADD_ENTITY: u32 = 0x00d0_7420;
const ISLAND_ADD_ACTION: u32 = 0x00d0_7470;
/// Unnamed `hkpSimulationIsland` functions: remove an entity (`00d071b0`) and register an
/// action (`00d07130`), `this` = island.
const ISLAND_REMOVE_ENTITY: u32 = 0x00d0_71b0;
const ISLAND_REGISTER_ACTION: u32 = 0x00d0_7130;
/// `hkpSimulationIsland::hkpSimulationIsland` (Xbox PDB): `this` = memory, one stack word (the
/// world), returns `this`.
const ISLAND_CONSTRUCTOR: u32 = 0x00d0_7830;
/// `hkpPhantom::setBoundingVolumeData` (Xbox PDB): `this` = phantom, the AABB buffer.
const PHANTOM_SET_BOUNDING_VOLUME_DATA: u32 = 0x00d0_7950;
/// `hkpLinkedCollidable` entries (`00d081e0`, translated in `hkplinkedcollidable.rs`),
/// sorted entries (`00d08210`) and sorted collection (`00d08260`), `this` = collidable.
const LINKED_COLLIDABLE_ENTRIES: u32 = 0x00d0_81e0;
const LINKED_COLLIDABLE_SORT_ENTRIES: u32 = 0x00d0_8210;
const LINKED_COLLIDABLE_GET_ENTRIES_SORTED: u32 = 0x00d0_8260;
/// `hkAgentNnMachine_CopyAndRelinkAgentEntry`, `InternalDeallocateEntry` and `AppendTrack`
/// (Xbox PDB).
const AGENT_COPY_AND_RELINK_ENTRY: u32 = 0x00cf_eca0;
const AGENT_DEALLOCATE_ENTRY: u32 = 0x00cf_e020;
const AGENT_APPEND_TRACK: u32 = 0x00cf_f290;
/// `hkpWorldAgentUtil::getIslandFromAgentEntry` (Xbox PDB): `(entry, island, island)`.
const AGENT_UTIL_GET_ISLAND_FROM_ENTRY: u32 = 0x00cc_0e20;
/// `hkpEntityAabbUtil::entityBatchRecalcAabb` (Xbox PDB): `(collision input, entity slots, count)`.
const ENTITY_BATCH_RECALC_AABB: u32 = 0x00d1_a330;
/// `hkpTypedBroadPhaseDispatcher` (Xbox PDB): `addPairs` (`this` = dispatcher, pairs, count,
/// filter), `removePairs` (`this`, pairs, count) and the cdecl `removeDuplicates`.
const DISPATCHER_ADD_PAIRS: u32 = 0x00cf_7080;
const DISPATCHER_REMOVE_PAIRS: u32 = 0x00cf_70f0;
const DISPATCHER_REMOVE_DUPLICATES: u32 = 0x00cf_7130;
/// `hkpWorldOperationUtil::internalMergeTwoIslands` (Xbox PDB).
const INTERNAL_MERGE_TWO_ISLANDS: u32 = 0x00cb_4c60;
/// `hkpEntity::sortConstraintsSlavesDeterministically` and `setCachedShapeData` (Xbox PDB).
const ENTITY_SORT_CONSTRAINTS_SLAVES: u32 = 0x00c9_c860;
const ENTITY_SET_CACHED_SHAPE_DATA: u32 = 0x00c9_c040;
/// `hkpMotion` (Xbox PDB): `getMass` (`this` = the copy, no stack word), `hkMotionState::
/// operator=`, and `getMotionStateAndVelocitiesAndDeactivationType` (`this` = source, the
/// destination motion).
const MOTION_GET_MASS: u32 = 0x00c8_6430;
const MOTION_STATE_ASSIGN: u32 = 0x00c8_6660;
const MOTION_COPY_STATE_AND_VELOCITIES: u32 = 0x00c8_6610;
/// `hkpRigidBody::createDynamicRigidMotion` (Xbox PDB), cdecl (nine words, ADD ESP,0x24).
const RIGID_BODY_CREATE_DYNAMIC_MOTION: u32 = 0x00ca_8240;
/// `hkSweptTransformUtil::freezeMotionState` (Xbox PDB), cdecl: `(time, motion state)`.
const FREEZE_MOTION_STATE: u32 = 0x00cf_1d50;
/// `hkpKeyframedRigidMotion` constructor and destructor, `hkpFixedRigidMotion` constructor
/// (Xbox PDB); `this` = the motion, constructors take two vector pointers.
const KEYFRAMED_MOTION_CONSTRUCTOR: u32 = 0x00d1_dfa0;
const KEYFRAMED_MOTION_DESTRUCTOR: u32 = 0x00d1_dff0;
const FIXED_MOTION_CONSTRUCTOR: u32 = 0x00d1_e160;
/// `hkString::memSet` (Xbox PDB), cdecl `(destination, value, count)`.
const MEM_SET: u32 = 0x00cb_1280;
/// Unnamed `hkpWorld` function called with the three flag bytes of the dynamics step info
/// (`00c90ed0`, `this` = the motion).
const WORLD_SET_MOTION_FLAGS: u32 = 0x00c9_0ed0;
/// `hkpSimulation::collideEntitiesBroadPhaseDiscrete` (Xbox PDB), cdecl.
const SIMULATION_COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE: u32 = 0x00cf_9c60;
/// The table of floats indexed by the two bytes at +0xbc/+0xbd of an `hkpMotion` copy.
const MOTION_FLOAT_TABLE: u32 = 0x010c_7948;

// Fields of `hkpWorld` (see the header).
const WORLD_SIMULATION: u32 = 0x08;
const WORLD_FIXED_ISLAND: u32 = 0x20;
const WORLD_ACTIVE_ISLANDS: u32 = 0x28;
const WORLD_INACTIVE_ISLANDS: u32 = 0x34;
const WORLD_DIRTY_ISLANDS: u32 = 0x40;
const WORLD_BROAD_PHASE: u32 = 0x58;
const WORLD_DISPATCHER: u32 = 0x64;
const WORLD_COLLISION_INPUT: u32 = 0x78;
const WORLD_COLLISION_FILTER: u32 = 0x7c;
const WORLD_PENDING_OPERATIONS: u32 = 0x8c;
const WORLD_PENDING_BODY_OPERATIONS: u32 = 0x90;
const WORLD_LOCK_COUNT: u32 = 0x94;
const WORLD_BLOCK_PENDING: u32 = 0x9c;
const WORLD_CRITICAL_ALLOWED: u32 = 0x9d;
const WORLD_PENDING_QUEUE_COUNT: u32 = 0xa4;
const WORLD_MIN_ISLAND_SIZE: u32 = 0xb4;
const WORLD_WANT_ISLANDS: u32 = 0xc8;
const WORLD_CONSTRAINT_LISTENERS_SIZE: u32 = 0x144;
const WORLD_STEP_TIME: u32 = 0x1d0;
const WORLD_BROAD_PHASE_QUERY_SIZE: u32 = 0x338;

// Fields of `hkpSimulationIsland`.
const ISLAND_CONSTRAINT_COUNT: u32 = 0x1c;
const ISLAND_INDEX: u32 = 0x20;
const ISLAND_DIRTY_INDEX: u32 = 0x22;
const ISLAND_BYTE_24: u32 = 0x24;
const ISLAND_FLAGS_25: u32 = 0x25;
const ISLAND_FLAGS_26: u32 = 0x26;
const ISLAND_AGENT_TRACK: u32 = 0x58;
const ISLAND_ENTITY_COUNT: u32 = 0x4c;

// Fields of `hkpEntity`.
const ENTITY_WORLD: u32 = 0x08;
const ENTITY_COLLIDABLE: u32 = 0x10;
const ENTITY_BYTE_2A: u32 = 0x2a;
const ENTITY_MOTION: u32 = 0xe0;
const ENTITY_MOTION_TYPE: u32 = 0xe8;
const ENTITY_ISLAND: u32 = 0xcc;
const ENTITY_UNIQUE_ID: u32 = 0xd4;
const ENTITY_MOTION_STATE: u32 = 0xf0;
const ENTITY_ACTIONS: u32 = 0x208;
const ENTITY_ACTIONS_SIZE: u32 = 0x20c;

/// `MOTION_FIXED` and `MOTION_KEYFRAMED` (Xbox PDB `hkpMotion::MotionType`).
const MOTION_FIXED: u32 = 5;
const MOTION_KEYFRAMED: u32 = 4;
/// Size of the embedded `hkpMotion` copied when the motion object is replaced.
const MOTION_BYTES: u32 = 0x120;
/// The vtable pointer the exe stores at the start of its motion copy before the copy loop.
const MOTION_COPY_VTABLE: u32 = 0x010c_6c64;
/// The "not in the dirty list" value of an island's 16-bit dirty index.
const NOT_IN_DIRTY_LIST: u16 = 0xffff;

// ---------------------------------------------------------------------------------------
// Helpers.
// ---------------------------------------------------------------------------------------

/// Adds `delta` (wrapping) to the word at `addr` and returns the new value.
fn add_word(e: &mut Engine, addr: u32, delta: u32) -> u32 {
    let value = e.mem.u32(addr).wrapping_add(delta);
    e.mem.set_u32(addr, value);
    value
}

/// Appends one word to the `hkArray` at `array`, growing it first when it is full.
fn push_word(e: &mut Engine, array: u32, value: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Puts the island in the world's dirty list unless it is already there.
fn mark_island_dirty(e: &mut Engine, world: u32, island: u32) {
    if e.mem.u16(island + ISLAND_DIRTY_INDEX) == NOT_IN_DIRTY_LIST {
        let dirty_count = e.mem.u16(world + WORLD_DIRTY_ISLANDS + 4);
        e.mem.set_u16(island + ISLAND_DIRTY_INDEX, dirty_count);
        push_word(e, world + WORLD_DIRTY_ISLANDS, island);
    }
}

/// Runs the world's pending operations once a critical-operations lock is released.
fn run_pending_operations(e: &mut Engine, world: u32) {
    if e.mem.u32(world + WORLD_PENDING_OPERATIONS) != 0 {
        e.call(WORLD_RUN_PENDING_OPERATIONS, &args![world]);
    }
    if e.mem.u32(world + WORLD_PENDING_QUEUE_COUNT) == 1
        && e.mem.u32(world + WORLD_PENDING_BODY_OPERATIONS) != 0
    {
        e.call(WORLD_RUN_PENDING_BODY_OPERATIONS, &args![world]);
    }
}

/// Drops the lock count by one and, when it reaches zero with nothing blocking, runs the
/// pending operations.
fn release_critical_lock(e: &mut Engine, world: u32) {
    if add_word(e, world + WORLD_LOCK_COUNT, u32::MAX) == 0
        && e.mem.u8(world + WORLD_BLOCK_PENDING) == 0
    {
        run_pending_operations(e, world);
    }
}

/// The collision filter interface (`m_collisionFilter` + 8) or null.
fn collision_filter_interface(e: &Engine, world: u32) -> u32 {
    let filter = e.mem.u32(world + WORLD_COLLISION_FILTER);
    if filter != 0 {
        filter + 8
    } else {
        0
    }
}

/// Creates the `hkLocalArray` (data, size, capacity-and-flags, saved stack top; 8-byte
/// elements) at `array`, sized by the world's broad-phase query buffer size, taking its
/// storage from the memory router's stack.
/// Runs `f` with a `size`-byte frame that starts on a 16-byte boundary (the exe's
/// `AND ESP,0xfffffff0` frames); `f` gets the frame's address.
fn with_aligned_stack<R>(e: &mut Engine, size: u32, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(size + 0x10, |e, raw| f(e, (raw.addr() + 0xf) & !0xf))
}

fn scratch_array_create(e: &mut Engine, array: u32, world: u32) {
    let capacity = e.mem.u32(world + WORLD_BROAD_PHASE_QUERY_SIZE);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let saved = e.mem.u32(router);
    let bytes = capacity.wrapping_mul(8).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, bytes.wrapping_add(saved));
    e.mem.set_u32(array, saved);
    e.mem.set_u32(array + 8, capacity | 0x8000_0000);
    e.mem.set_u32(array + 0xc, saved);
}

/// Destroys an array: frees its storage through the allocator when it owns it
/// (`element_bytes` per element) and clears it.
fn array_free_storage(e: &mut Engine, array: u32, element_bytes: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.mem.set_u32(array + 4, 0);
    let capacity = e.mem.u32(array + 8);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(element_bytes);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// Destroys a local array made by [`scratch_array_create`]: gives its storage back to the
/// router's stack, then frees it when it outgrew the stack.
fn scratch_array_destroy(e: &mut Engine, array: u32) {
    let saved = e.mem.u32(array + 0xc);
    if saved == e.mem.u32(array) {
        e.mem.set_u32(array + 4, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, saved);
    array_free_storage(e, array, 8);
}

/// Initialises an inline array of 16 words at `array` (data, size, capacity-and-flags, storage).
fn inplace_array_init(e: &mut Engine, array: u32) {
    e.mem.set_u32(array, array + 12);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0010);
}

/// `hkUnionFind::reindex` (Xbox PDB, `00d282c0`, `this` = the union-find object, `RET 0xC`:
/// the address of the word holding the group array, the number of groups, the parent array).
const UNION_FIND_REINDEX: u32 = 0x00d2_82c0;
/// The `KERNEL32` import slots of `EnterCriticalSection` and `LeaveCriticalSection`.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `hkpWorld` words used by the island-splitting functions below: +0xC0 the critical section
/// pointer, +0xD5 a byte choosing how entities are ranked, +0x305 and +0x306 the two bytes the
/// ranks are compared against (PC offsets; the names are not confirmed).
const WORLD_CRITICAL_SECTION: u32 = 0xc0;
const WORLD_RANKING_ENABLED: u32 = 0xd5;
const WORLD_RANK_BYTE_FIRST: u32 = 0x305;
const WORLD_RANK_BYTE_SECOND: u32 = 0x306;
/// The 16-bit words of an `hkpEntity` the rank is read from (+0xEA, +0xEC) and the 16-bit
/// count of its master constraints (+0xB0).
const ENTITY_RANK_WORD_FIRST: u32 = 0xea;
const ENTITY_RANK_WORD_SECOND: u32 = 0xec;
const ENTITY_MASTER_COUNT: u32 = 0xb0;

/// The rank of an entity as the splitting code computes it: each 16-bit word is shifted
/// right by 7 when its top two bits equal the world's byte, and masked with 0x7f; the larger
/// of the two (signed) is the rank.
fn entity_rank(e: &Engine, world: u32, entity: u32) -> i32 {
    let mut second = i32::from(e.mem.u16(entity + ENTITY_RANK_WORD_SECOND));
    if i32::from(e.mem.u8(world + WORLD_RANK_BYTE_SECOND)) == second >> 14 {
        second >>= 7;
    }
    let mut first = i32::from(e.mem.u16(entity + ENTITY_RANK_WORD_FIRST));
    if i32::from(e.mem.u8(world + WORLD_RANK_BYTE_FIRST)) == first >> 14 {
        first >>= 7;
    }
    (second & 0x7f).max(first & 0x7f)
}

/// Entry `index` of the integer array that the union-find object points at through two
/// levels (`**union_find`).
fn union_find_entry(e: &Engine, union_find: u32, index: u32) -> u32 {
    let inner = e.mem.u32(union_find);
    let data = e.mem.u32(inner);
    e.mem.u32(data.wrapping_add(index.wrapping_mul(4)))
}

/// Address of entry `index` of an array of words.
fn word_at(array: u32, index: i32) -> u32 {
    array.wrapping_add((index as u32).wrapping_mul(4))
}

/// Calls the sort's comparison function (cdecl, `(result byte, a, b)`) and reads the byte it
/// answers the address of.
fn sort_compare(e: &mut Engine, compare: u32, slot: u32, a: u32, b: u32) -> bool {
    let result = e.call(compare, &args![slot, a, b]).u32();
    e.mem.u8(result) != 0
}

// Translated from 00cb3c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::addConstraintToCriticalLockedIsland` (Xbox PDB), cdecl `(world,
/// constraint)`: `hkpWorldConstraintUtil::addConstraint` (`00d27ac0`) then
/// `hkpWorldCallbackUtil::fireConstraintAdded` (`00d00960`), both with the same two words.
pub fn hkp_world_operation_util_add_constraint_to_critical_locked_island(
    e: &mut Engine,
    world: Ptr,
    constraint: Ptr,
) {
    e.call(WORLD_CONSTRAINT_UTIL_ADD, &args![world, constraint]);
    e.call(FIRE_CONSTRAINT_ADDED, &args![world, constraint]);
}

// Translated from 00cb3da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::removeConstraintFromCriticalLockedIsland` (Xbox PDB), cdecl `(world,
/// constraint)`: when the world's constraint listener count (+0x144) is not 0, runs
/// `hkpWorldCallbackUtil::fireConstraintRemoved` (`00d00a00`) with both words; then
/// `hkpWorldConstraintUtil::removeConstraint` (`00d27820`) on the constraint.
pub fn hkp_world_operation_util_remove_constraint_from_critical_locked_island(
    e: &mut Engine,
    world: Ptr,
    constraint: Ptr,
) {
    if e.mem.u32(world.addr() + WORLD_CONSTRAINT_LISTENERS_SIZE) != 0 {
        e.call(FIRE_CONSTRAINT_REMOVED, &args![world, constraint]);
    }
    e.call(WORLD_CONSTRAINT_UTIL_REMOVE, &args![constraint]);
}

// Translated from 00cb4890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no caller in the exe). Takes `entities` in `EAX` (an `hkArray` of entity pointers),
/// then cdecl `(union_find, world, parents, group_flags)`; the register value comes first.
/// On the memory router's stack it makes a zeroed array of one word per element of `parents`
/// (count at `parents + 4`; the router's top moves up by the byte size rounded to 0x80). Every
/// entity whose rank (`entity_rank`) is at most 5 sets the word, at the index
/// `**union_find` holds for it, to 1. Then, with `group_flags` the address of a word holding
/// a byte array, byte 0 is set to 1 and each word is numbered: a set word is cleared, a clear
/// one gets the next number (from 1) and its byte (at the number) is cleared. If the next
/// number is above the count, byte 0 is cleared and the router's top is restored; otherwise
/// `hkUnionFind::reindex` (`00d282c0`, `this` = `union_find`) is called with the address of a
/// stack word holding the array, the next number and `parents`, and the router's top is set
/// to the word's value afterwards.
pub fn fn_00cb4890(
    e: &mut Engine,
    entities: u32,
    union_find: u32,
    world: u32,
    parents: u32,
    group_flags: u32,
) {
    let count = e.mem.i32(parents + 4);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let marks = e.mem.u32(router);
    let size = (count as u32).wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, size.wrapping_add(marks));
    for index in 0..count.max(0) as u32 {
        e.mem.set_u32(marks + index * 4, 0);
    }
    let mut index = 0i32;
    while index < e.mem.i32(entities + 4) {
        let data = e.mem.u32(entities);
        let entity = e.mem.u32(word_at(data, index));
        if entity_rank(e, world, entity) <= 5 {
            let group = union_find_entry(e, union_find, index as u32);
            e.mem.set_u32(word_at(marks, group as i32), 1);
        }
        index += 1;
    }
    let flags = e.mem.u32(group_flags);
    e.mem.set_u8(flags, 1);
    let mut next = 1u32;
    for index in 0..count.max(0) {
        let slot = word_at(marks, index);
        if e.mem.u32(slot) != 0 {
            e.mem.set_u32(slot, 0);
        } else {
            e.mem.set_u32(slot, next);
            let flags = e.mem.u32(group_flags);
            e.mem.set_u8(flags.wrapping_add(next), 0);
            next += 1;
        }
    }
    if (next as i32) <= count {
        let restored = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), marks);
            e.call(UNION_FIND_REINDEX, &args![union_find, slot, next, parents]);
            e.mem.u32(slot.addr())
        });
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, restored);
    } else {
        let flags = e.mem.u32(group_flags);
        e.mem.set_u8(flags, 0);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, marks);
    }
}

// Translated from 00cb49e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no caller in the exe), cdecl `(world, union_find, entities, parents, group_flags)`:
/// splits entities into groups for new islands. Two zeroed word arrays (weights, then marks;
/// one word per element of `parents`, count at `parents + 4`) are taken from the memory
/// router's stack. Every entity adds its master constraint count (+0xB0, at least 1) to the
/// weight at the index `**union_find` holds for it and sets the mark to 1: with the world's
/// byte at +0xD5 clear always, otherwise only when its rank (`entity_rank`) is at most 5.
/// Weights with no mark become -1. Then the weights are walked: a weight of -2 is skipped;
/// otherwise the next group number is given to the element in the marks array, its flag byte
/// (the byte array `group_flags` points at) is cleared and, for a weight of 0 or more, the
/// later weights are merged in while the running total plus 10 stays below the world's
/// word at +0xB4: each later weight of 0 or more that keeps the total below it gets the
/// group number, becomes -2, adds to the total and sets the group's flag byte to 1. At the
/// end `hkUnionFind::reindex` (`00d282c0`, `this` = `union_find`) runs with the address of a
/// stack word holding the marks, the group count and `parents`; the router's top is set to the
/// word's value and then to the start of the weights.
pub fn fn_00cb49e0(
    e: &mut Engine,
    world: u32,
    union_find: u32,
    entities: u32,
    parents: u32,
    group_flags: u32,
) {
    let count = e.mem.i32(parents + 4);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let weights = e.mem.u32(router);
    let size = (count as u32).wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem.set_u32(router, size.wrapping_add(weights));
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let marks = e.mem.u32(router);
    e.mem.set_u32(router, size.wrapping_add(marks));
    for index in 0..count.max(0) {
        e.mem.set_u32(word_at(marks, index), 0);
        e.mem.set_u32(word_at(weights, index), 0);
    }
    let ranked = e.mem.u8(world + WORLD_RANKING_ENABLED) != 0;
    let mut index = 0i32;
    while index < e.mem.i32(entities + 4) {
        let data = e.mem.u32(entities);
        let entity = e.mem.u32(word_at(data, index));
        let group = union_find_entry(e, union_find, index as u32);
        if !ranked || entity_rank(e, world, entity) <= 5 {
            e.mem.set_u32(word_at(marks, group as i32), 1);
        }
        let weight = u32::from(e.mem.u16(entity + ENTITY_MASTER_COUNT)).max(1);
        add_word(e, word_at(weights, group as i32), weight);
        index += 1;
    }
    for index in 0..count.max(0) {
        if e.mem.u32(word_at(marks, index)) == 0 {
            e.mem.set_u32(word_at(weights, index), 0xffff_ffff);
        }
    }
    let mut next_group = 0i32;
    if count > 0 {
        let mut weight_cursor = weights;
        let mut mark_cursor = marks;
        let mut first_later = 1i32;
        let mut remaining = count;
        loop {
            let mut running = e.mem.i32(weight_cursor);
            if running != -2 {
                let group = next_group;
                next_group += 1;
                let flags = e.mem.u32(group_flags);
                e.mem.set_u8(flags.wrapping_add(group as u32), 0);
                e.mem.set_i32(mark_cursor, group);
                if running >= 0 && first_later < count {
                    let mut later = first_later;
                    let mut cursor = weight_cursor + 4;
                    loop {
                        let limit = e.mem.i32(world + WORLD_MIN_ISLAND_SIZE);
                        if running.wrapping_add(10) >= limit {
                            break;
                        }
                        let other = e.mem.i32(cursor);
                        if other >= 0 {
                            let total = other.wrapping_add(running);
                            if total < limit {
                                let flags = e.mem.u32(group_flags);
                                e.mem.set_u8(flags.wrapping_add(group as u32), 1);
                                let mark = marks.wrapping_sub(weights).wrapping_add(cursor);
                                e.mem.set_i32(mark, group);
                                running = total;
                                e.mem.set_i32(cursor, -2);
                            }
                        }
                        later += 1;
                        cursor += 4;
                        if later >= count {
                            break;
                        }
                    }
                }
            }
            mark_cursor += 4;
            first_later += 1;
            weight_cursor += 4;
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }
    let restored = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), marks);
        e.call(
            UNION_FIND_REINDEX,
            &args![union_find, slot, next_group as u32, parents],
        );
        e.mem.u32(slot.addr())
    });
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, restored);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, weights);
}

// Translated from 00cb5480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, cdecl `(world, island)`: clears bits 2 and 3 of the island's flag byte at +0x26,
/// then, inside the world's critical section (the pointer at +0xC0, `EnterCriticalSection` and
/// `LeaveCriticalSection`), puts the island in the world's dirty list (+0x40) unless its
/// 16-bit index at +0x22 shows it already is (`mark_island_dirty`).
pub fn fn_00cb5480(e: &mut Engine, world: Ptr, island: Ptr) {
    let (world, island) = (world.addr(), island.addr());
    let flags = e.mem.u8(island + ISLAND_FLAGS_26);
    e.mem.set_u8(island + ISLAND_FLAGS_26, flags & 0xf3);
    let lock = e.mem.u32(world + WORLD_CRITICAL_SECTION);
    e.call(ENTER_CRITICAL_SECTION, &args![lock]);
    mark_island_dirty(e, world, island);
    let lock = e.mem.u32(world + WORLD_CRITICAL_SECTION);
    e.call(LEAVE_CRITICAL_SECTION, &args![lock]);
}

// Translated from 00cb7480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed, thiscall `(instance, info)` on the constraint owner (`RET 8`): the opposite of
/// `hkpConstraintOwner::addConstraintInfo` for an instance with a record (+0x30; without one
/// nothing happens). The owner's words at +0xC, +0x10 and +0x14 lose the info's words at +4,
/// +8 and +0xC; the record's 16-bit +0x14 loses the info's +4, its +0x16 loses the 16 bits of
/// the info's +8 and its +0x18 loses the info's +0xC.
pub fn fn_00cb7480(e: &mut Engine, this: Ptr, instance: Ptr, info: Ptr) {
    if e.mem.u32(instance.addr() + 0x30) == 0 {
        return;
    }
    let (owner, info) = (this.addr(), info.addr());
    for (field, word) in [(0xc, 4), (0x10, 8), (0x14, 0xc)] {
        let amount = e.mem.u32(info + word);
        add_word(e, owner + field, amount.wrapping_neg());
    }
    let record = e.mem.u32(instance.addr() + 0x30);
    let first = u32::from(e.mem.u16(record + 0x14));
    let third = u32::from(e.mem.u16(record + 0x18));
    let first = first.wrapping_sub(e.mem.u32(info + 4));
    let third = third.wrapping_sub(e.mem.u32(info + 0xc));
    let second = e.mem.u16(info + 8);
    let current = e.mem.u16(record + 0x16);
    e.mem.set_u16(record + 0x16, current.wrapping_sub(second));
    e.mem.set_u16(record + 0x14, first as u16);
    e.mem.set_u16(record + 0x18, third as u16);
}

// Translated from 00cb3f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The comparison `cleanupDirtyIslands` sorts the dirty islands with (Ghidra made no function
/// of it; no name in the engine map), cdecl `(result, a, b)`: stores in the byte at `result`
/// whether island `a` goes before island `b`, and answers `result`. A null `b` gives 0, else a
/// null `a` gives 1, else the answer is whether the unique id (`+0xd4`) of `a`'s first entity
/// (the first word of the entity array at `+0x48`) is below `b`'s, unsigned.
pub fn fn_00cb3f90(e: &mut Engine, result: Ptr, a: Ptr, b: Ptr) -> Ptr {
    let before = if b.addr() == 0 {
        false
    } else if a.addr() == 0 {
        true
    } else {
        let first_of_a = e.mem.u32(e.mem.u32(a.addr() + ISLAND_ENTITIES));
        let first_of_b = e.mem.u32(e.mem.u32(b.addr() + ISLAND_ENTITIES));
        e.mem.u32(first_of_a + 0xd4) < e.mem.u32(first_of_b + 0xd4)
    };
    e.mem.set_u8(result.addr(), before as u8);
    result
}

// Translated from 00dd7d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlgorithm::quickSortRecursive<hkpSimulationIsland*, hkBool (*)(const hkpSimulationIsland*
/// const&, ...)>` (Xbox PDB name abbreviated), cdecl `(array, low, high, compare)`: sorts the
/// pointers `array[low..=high]` in place. The pivot is the element at `(low + high) >> 1`;
/// the scans use `compare(result slot, a, b)` (cdecl, answering the address of a byte): the
/// lower index advances while `compare(array[i], pivot)` is true, the upper one retreats
/// while `compare(pivot, array[j])` is true, elements are swapped while `i <= j`, the lower
/// part is sorted by recursion and the upper part by the outer loop. Index compares are
/// signed.
pub fn hk_algorithm_quick_sort_recursive_hkp_simulation_island(
    e: &mut Engine,
    array: u32,
    low: i32,
    high: i32,
    compare: u32,
) {
    e.with_stack(4, |e, slots| {
        let (first_slot, second_slot) = (slots.addr() + 2, slots.addr() + 3);
        let mut low = low;
        loop {
            let mut lower = low;
            let mut upper = high;
            let pivot = e.mem.u32(word_at(array, lower.wrapping_add(upper) >> 1));
            loop {
                let mut candidate = e.mem.u32(word_at(array, lower));
                while sort_compare(e, compare, first_slot, candidate, pivot) {
                    candidate = e.mem.u32(word_at(array, lower.wrapping_add(1)));
                    lower = lower.wrapping_add(1);
                }
                let mut candidate = e.mem.u32(word_at(array, upper));
                while sort_compare(e, compare, second_slot, pivot, candidate) {
                    candidate = e.mem.u32(word_at(array, upper.wrapping_sub(1)));
                    upper = upper.wrapping_sub(1);
                }
                if upper < lower {
                    break;
                }
                if upper != lower {
                    let at_lower = e.mem.u32(word_at(array, lower));
                    let at_upper = e.mem.u32(word_at(array, upper));
                    e.mem.set_u32(word_at(array, upper), at_lower);
                    e.mem.set_u32(word_at(array, lower), at_upper);
                }
                upper = upper.wrapping_sub(1);
                lower = lower.wrapping_add(1);
                if lower > upper {
                    break;
                }
            }
            if low < upper {
                hk_algorithm_quick_sort_recursive_hkp_simulation_island(
                    e, array, low, upper, compare,
                );
            }
            low = lower;
            if lower >= high {
                break;
            }
        }
    });
}

/// `hkArrayUtil::_reserve` to an exact capacity: `(allocator, array, capacity, element size)`
/// (`00c908c0`, cdecl).
const ARRAY_RESERVE_EXACT: u32 = 0x00c9_08c0;
/// `hkSweptTransformUtil::setTimeInformation` (Xbox PDB, `00cf1480`, cdecl): two floats (both 0
/// here) and the motion state.
const SET_TIME_INFORMATION: u32 = 0x00cf_1480;
/// `hkpWorldAgentUtil::warpTime` (Xbox PDB, `00cc0ed0`, cdecl): `(island, island time, world step
/// time, collision input)`.
const WARP_TIME: u32 = 0x00cc_0ed0;
/// `hkpWorldCallbackUtil::fireIslandActivated` and `fireIslandDeactivated` (Xbox PDB,
/// `00d00fe0`, `00d011b0`, cdecl `(world, island)`).
const FIRE_ISLAND_ACTIVATED: u32 = 0x00d0_0fe0;
const FIRE_ISLAND_DEACTIVATED: u32 = 0x00d0_11b0;
/// The check `(world + 0x1d0, entity array, entity count, 0xe0)` that `internalDeactivateIsland`
/// runs first (`00d28560`, cdecl, a `bool` in `AL`; unnamed in the engine map).
const ENTITIES_CHECK: u32 = 0x00d2_8560;
/// The address passed to the motion's virtual functions at +0x40 and +0x44 (an object in the
/// exe's data).
const MOTION_DEACTIVATION_ARGUMENT: u32 = 0x0126_7e30;
/// The comparison function `cleanupDirtyIslands` sorts the dirty list with (`00cb3f90`).
const DIRTY_ISLAND_LESS: u32 = 0x00cb_3f90;
/// `hkpWorld::checkDeterminism` (Xbox PDB, `00c910b0`, a bare `RET`; `this` = the world).
const WORLD_CHECK_DETERMINISM: u32 = 0x00c9_10b0;
/// The `hkpWorld` byte at +0x307 (with +0x305 and +0x306, the tag bytes that go into the top two
/// bits of an entity's two 16-bit words; names unconfirmed) and the entity byte at +0xe9 it is
/// compared with.
const WORLD_TAG_BYTE_307: u32 = 0x307;
const ENTITY_TAG_BYTE_E9: u32 = 0xe9;
/// The `hkpSimulationIsland` float at +0x44 (the island's time), the entity array at +0x48 and
/// the `hkArray` at +0x38 that the cleanup squeezes.
const ISLAND_TIME: u32 = 0x44;
const ISLAND_ENTITIES: u32 = 0x48;
const ISLAND_ARRAY_38: u32 = 0x38;

// Translated from 00cb5100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::internalActivateIsland` (Xbox PDB), cdecl `(world, island, flag)`
/// (only the low byte of `flag` is read): moves the island from the world's inactive list
/// (the last island fills its slot) to the end of the active list, sets its index and clears its
/// floats at +0x30/+0x34, sets bit 0 and clears bit 1 of the flag byte at +0x26, and tags every
/// entity's two 16-bit words (+0xea, +0xec) from the world's bytes at +0x305 to +0x307
/// (inverted when the entity's byte at +0xe9 is larger). It then warps the island's agents to the
/// world's time, takes a reference on the first entity while firing the "island activated"
/// callback, and, when the first entity still belongs to the island and `flag` is set, merges the
/// island with a small sparse active island found for it (`internalMergeTwoIslands`); otherwise a
/// larger island moves to the front of the active list.
pub fn hkp_world_operation_util_internal_activate_island(
    e: &mut Engine,
    world: Ptr,
    island: Ptr,
    flag: u32,
) {
    let (world, island) = (world.addr(), island.addr());
    // Out of the inactive list.
    let inactive_data = e.mem.u32(world + WORLD_INACTIVE_ISLANDS);
    let inactive_size = e.mem.u32(world + WORLD_INACTIVE_ISLANDS + 4);
    let last = e.mem.u32(
        inactive_data
            .wrapping_add(inactive_size.wrapping_mul(4))
            .wrapping_sub(4),
    );
    let slot = u32::from(e.mem.u16(island + ISLAND_INDEX));
    e.mem.set_u32(inactive_data.wrapping_add(slot * 4), last);
    let slot = e.mem.u16(island + ISLAND_INDEX);
    let inactive_data = e.mem.u32(world + WORLD_INACTIVE_ISLANDS);
    let moved = e.mem.u32(inactive_data.wrapping_add(u32::from(slot) * 4));
    e.mem.set_u16(moved + ISLAND_INDEX, slot);
    add_word(e, world + WORLD_INACTIVE_ISLANDS + 4, u32::MAX);
    // Onto the active list.
    push_word(e, world + WORLD_ACTIVE_ISLANDS, island);
    let active_size = e.mem.u16(world + WORLD_ACTIVE_ISLANDS + 4);
    e.mem.set_u32(island + 0x30, 0);
    e.mem.set_u32(island + 0x34, 0);
    let flags = e.mem.u8(island + ISLAND_FLAGS_26);
    e.mem
        .set_u16(island + ISLAND_INDEX, active_size.wrapping_sub(1));
    e.mem.set_u8(island + ISLAND_FLAGS_26, (flags & 0xfd) | 1);
    let mut index = 0u32;
    while (index as i32) < (e.mem.u32(island + ISLAND_ENTITY_COUNT) as i32) {
        let entities = e.mem.u32(island + ISLAND_ENTITIES);
        let entity = e.mem.u32(entities.wrapping_add(index * 4));
        e.call(
            SET_TIME_INFORMATION,
            &args![0u32, 0u32, entity + ENTITY_MOTION_STATE],
        );
        e.mem.set_u16(entity + ENTITY_RANK_WORD_SECOND, 0);
        e.mem.set_u16(entity + ENTITY_RANK_WORD_FIRST, 0);
        let first_byte = u32::from(e.mem.u8(world + WORLD_RANK_BYTE_FIRST));
        let second_byte = u32::from(e.mem.u8(world + WORLD_RANK_BYTE_SECOND));
        let world_tag = e.mem.u8(world + WORLD_TAG_BYTE_307);
        let entity_tag = e.mem.u8(entity + ENTITY_TAG_BYTE_E9);
        let first = if (world_tag & 3) < (entity_tag & 3) {
            !first_byte
        } else {
            first_byte
        };
        let second = if world_tag < entity_tag {
            !second_byte
        } else {
            second_byte
        };
        e.mem
            .set_u16(entity + ENTITY_RANK_WORD_FIRST, (first << 14) as u16);
        e.mem
            .set_u16(entity + ENTITY_RANK_WORD_SECOND, (second << 14) as u16);
        index += 1;
    }
    let island_time = x87_quiet_bits(e.mem.u32(island + ISLAND_TIME));
    let step_time = x87_quiet_bits(e.mem.u32(world + WORLD_STEP_TIME));
    let collision_input = e.mem.u32(world + WORLD_COLLISION_INPUT);
    e.call(
        WARP_TIME,
        &args![island, island_time, step_time, collision_input],
    );
    let entities = e.mem.u32(island + ISLAND_ENTITIES);
    let first_entity = e.mem.u32(entities);
    e.call(ADD_REFERENCE, &args![first_entity]);
    e.call(FIRE_ISLAND_ACTIVATED, &args![world, island]);
    if e.mem.u32(first_entity + ENTITY_ISLAND) != island {
        e.call(REMOVE_REFERENCE, &args![first_entity]);
        return;
    }
    e.call(REMOVE_REFERENCE, &args![first_entity]);
    if flag as u8 != 0 {
        let by_constraints = (e.mem.u32(island + ISLAND_CONSTRAINT_COUNT) as i32).wrapping_add(3);
        let by_entities = (e.mem.u32(island + ISLAND_ENTITY_COUNT) as i32).wrapping_add(1);
        let wanted = if by_entities > by_constraints {
            by_entities
        } else {
            by_constraints
        };
        let partner = find_small_sparse_active_island(
            e,
            Ptr::new(world),
            Ptr::new(world + WORLD_ACTIVE_ISLANDS),
            wanted,
            Ptr::new(island),
        );
        if !partner.is_null() {
            hkp_world_operation_util_internal_merge_two_islands(
                e,
                Ptr::new(world),
                partner,
                Ptr::new(island),
            );
            return;
        }
    }
    let index = e.mem.u16(island + ISLAND_INDEX);
    if index != 0 {
        let active_data = e.mem.u32(world + WORLD_ACTIVE_ISLANDS);
        let first_island = e.mem.u32(active_data);
        if (e.mem.u32(island + ISLAND_ENTITY_COUNT) as i32)
            > (e.mem.u32(first_island + ISLAND_ENTITY_COUNT) as i32)
        {
            e.mem.set_u16(island + ISLAND_INDEX, 0);
            e.mem.set_u16(first_island + ISLAND_INDEX, index);
            let active_data = e.mem.u32(world + WORLD_ACTIVE_ISLANDS);
            e.mem.set_u32(active_data, island);
            let active_data = e.mem.u32(world + WORLD_ACTIVE_ISLANDS);
            e.mem
                .set_u32(active_data.wrapping_add(u32::from(index) * 4), first_island);
        }
    }
}

// Translated from 00cb5310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::internalDeactivateIsland` (Xbox PDB), cdecl `(world, island)`: asks
/// `00d28560(world + 0x1d0, entities, entity count, 0xe0)`; when that answers false the
/// island's flag byte at +0x26 gets bit 2 set and bit 3 cleared and nothing else happens.
/// Otherwise the island goes to the end of the inactive list, the last active island fills its
/// old slot, its index is the new last slot of the inactive list, bits 0 and 1 of the flag byte
/// are cleared, its time (+0x44) is the world's (+0x1d0), every entity's motion state is frozen
/// at that time and the motion's virtual functions at +0x40 and +0x44 are called with a constant
/// object, and `fireIslandDeactivated` runs.
pub fn hkp_world_operation_util_internal_deactivate_island(
    e: &mut Engine,
    world: Ptr,
    island: Ptr,
) {
    let (world, island) = (world.addr(), island.addr());
    let entities = e.mem.u32(island + ISLAND_ENTITIES);
    let count = e.mem.u32(island + ISLAND_ENTITY_COUNT);
    let passes = e
        .call(
            ENTITIES_CHECK,
            &args![world + WORLD_STEP_TIME, entities, count, 0xe0u32],
        )
        .bool();
    if !passes {
        let flags = e.mem.u8(island + ISLAND_FLAGS_26);
        e.mem.set_u8(island + ISLAND_FLAGS_26, (flags & 0xf7) | 4);
        return;
    }
    push_word(e, world + WORLD_INACTIVE_ISLANDS, island);
    let active_size = e.mem.u32(world + WORLD_ACTIVE_ISLANDS + 4);
    let active_data = e.mem.u32(world + WORLD_ACTIVE_ISLANDS);
    let last = e.mem.u32(
        active_data
            .wrapping_add(active_size.wrapping_mul(4))
            .wrapping_sub(4),
    );
    let slot = u32::from(e.mem.u16(island + ISLAND_INDEX));
    e.mem.set_u32(active_data.wrapping_add(slot * 4), last);
    let slot = e.mem.u16(island + ISLAND_INDEX);
    let active_data = e.mem.u32(world + WORLD_ACTIVE_ISLANDS);
    let moved = e.mem.u32(active_data.wrapping_add(u32::from(slot) * 4));
    e.mem.set_u16(moved + ISLAND_INDEX, slot);
    add_word(e, world + WORLD_ACTIVE_ISLANDS + 4, u32::MAX);
    let inactive_size = e.mem.u16(world + WORLD_INACTIVE_ISLANDS + 4);
    let flags = e.mem.u8(island + ISLAND_FLAGS_26);
    e.mem.set_u8(island + ISLAND_FLAGS_26, flags & 0xfc);
    e.mem
        .set_u16(island + ISLAND_INDEX, inactive_size.wrapping_sub(1));
    let step_time = x87_quiet_bits(e.mem.u32(world + WORLD_STEP_TIME));
    e.mem.set_u32(island + ISLAND_TIME, step_time);
    let mut index = 0u32;
    while (index as i32) < (e.mem.u32(island + ISLAND_ENTITY_COUNT) as i32) {
        let entities = e.mem.u32(island + ISLAND_ENTITIES);
        let entity = e.mem.u32(entities.wrapping_add(index * 4));
        let island_time = x87_quiet_bits(e.mem.u32(island + ISLAND_TIME));
        e.call(
            FREEZE_MOTION_STATE,
            &args![island_time, entity + ENTITY_MOTION_STATE],
        );
        e.vcall(
            entity + ENTITY_MOTION,
            0x40,
            &args![MOTION_DEACTIVATION_ARGUMENT],
        );
        e.vcall(
            entity + ENTITY_MOTION,
            0x44,
            &args![MOTION_DEACTIVATION_ARGUMENT],
        );
        index += 1;
    }
    e.call(FIRE_ISLAND_DEACTIVATED, &args![world, island]);
}

// Translated from 00cb5420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::markIslandInactive` (Xbox PDB), cdecl `(world, island)`: clears bits
/// 2 and 3 of the island's flag byte at +0x26 and, unless the island is already in the world's
/// dirty list (index at +0x22 not 0xffff), puts it at the end of that list.
pub fn hkp_world_operation_util_mark_island_inactive(e: &mut Engine, world: Ptr, island: Ptr) {
    let (world, island) = (world.addr(), island.addr());
    let flags = e.mem.u8(island + ISLAND_FLAGS_26);
    e.mem.set_u8(island + ISLAND_FLAGS_26, flags & 0xf3);
    mark_island_dirty(e, world, island);
}

// Translated from 00cb55d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::cleanupDirtyIslands` (Xbox PDB), cdecl `(world)`: sorts the dirty
/// list (more than one island) with `00cb3f90`, renumbers the islands' dirty indices, then
/// takes islands off its end one at a time. For each: its dirty index becomes 0xffff; when bits
/// 4 or 5 of its flag byte at +0x25 are set, the null entries of its array at +0x38 are squeezed
/// out (growing the capacity with `00c908c0` when that is below the new count) and those bits
/// are cleared; then, comparing the two 2-bit fields of the byte at +0x26 (bits 0-1 and 2-3):
/// differing, it is activated when the second is non-zero (merge flag set) or deactivated;
/// equal with bit 4 or 5 set, those bits are cleared and the island merged with a small sparse
/// active island, if one is found. Ends with `checkDeterminism`.
pub fn hkp_world_operation_util_cleanup_dirty_islands(e: &mut Engine, world: Ptr) {
    let world = world.addr();
    let count = e.mem.u32(world + WORLD_DIRTY_ISLANDS + 4);
    if count == 0 {
        return;
    }
    if (count as i32) > 1 {
        let array = e.mem.u32(world + WORLD_DIRTY_ISLANDS);
        hk_algorithm_quick_sort_recursive_hkp_simulation_island(
            e,
            array,
            0,
            (count as i32).wrapping_sub(1),
            DIRTY_ISLAND_LESS,
        );
    }
    let mut index = 0u32;
    while (index as i32) < (e.mem.u32(world + WORLD_DIRTY_ISLANDS + 4) as i32) {
        let array = e.mem.u32(world + WORLD_DIRTY_ISLANDS);
        let island = e.mem.u32(array.wrapping_add(index * 4));
        if island != 0 {
            e.mem.set_u16(island + ISLAND_DIRTY_INDEX, index as u16);
        }
        index += 1;
    }
    while e.mem.u32(world + WORLD_DIRTY_ISLANDS + 4) != 0 {
        let size = e.mem.u32(world + WORLD_DIRTY_ISLANDS + 4);
        let array = e.mem.u32(world + WORLD_DIRTY_ISLANDS);
        let island = e
            .mem
            .u32(array.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4));
        add_word(e, world + WORLD_DIRTY_ISLANDS + 4, u32::MAX);
        if island == 0 {
            continue;
        }
        e.mem
            .set_u16(island + ISLAND_DIRTY_INDEX, NOT_IN_DIRTY_LIST);
        if e.mem.u8(island + ISLAND_FLAGS_25) & 0x30 != 0 {
            squeeze_null_entries(e, island + ISLAND_ARRAY_38);
            let flags = e.mem.u8(island + ISLAND_FLAGS_25);
            e.mem.set_u8(island + ISLAND_FLAGS_25, flags & 0xcf);
        }
        let flags = e.mem.u8(island + ISLAND_FLAGS_26);
        let wanted_state = (flags >> 2) & 3;
        let state = flags & 3;
        if wanted_state != state {
            if wanted_state != 0 {
                hkp_world_operation_util_internal_activate_island(
                    e,
                    Ptr::new(world),
                    Ptr::new(island),
                    1,
                );
            } else {
                hkp_world_operation_util_internal_deactivate_island(
                    e,
                    Ptr::new(world),
                    Ptr::new(island),
                );
            }
        } else if flags & 0x30 != 0 {
            e.mem.set_u8(island + ISLAND_FLAGS_26, flags & 0xcf);
            let by_constraints = e.mem.u32(island + ISLAND_CONSTRAINT_COUNT) as i32;
            let by_entities = e.mem.u32(island + ISLAND_ENTITY_COUNT) as i32;
            let wanted = if by_entities > by_constraints {
                by_entities
            } else {
                by_constraints
            };
            let partner = find_small_sparse_active_island(
                e,
                Ptr::new(world),
                Ptr::new(world + WORLD_ACTIVE_ISLANDS),
                wanted,
                Ptr::new(island),
            );
            if !partner.is_null() {
                let entities = e.mem.u32(island + ISLAND_ENTITIES);
                let first_of_island = e.mem.u32(entities);
                let partner_entities = e.mem.u32(partner.addr() + ISLAND_ENTITIES);
                let first_of_partner = e.mem.u32(partner_entities);
                hkp_world_operation_util_merge_islands(
                    e,
                    Ptr::new(world),
                    Ptr::new(first_of_island),
                    Ptr::new(first_of_partner),
                );
            }
        }
    }
    e.call(WORLD_CHECK_DETERMINISM, &args![world]);
}

/// The compaction loop of `cleanupDirtyIslands`: the non-null words of the `hkArray` at
/// `list` move down over the first null one, and the size becomes the number kept (the
/// capacity is raised first when it is below that number); an array without null words keeps its
/// size (`write` stays -1 and nothing is stored).
fn squeeze_null_entries(e: &mut Engine, list: u32) {
    let size = e.mem.u32(list + 4) as i32;
    let mut write: i32 = -1;
    let mut read: i32 = 0;
    if size > 0 {
        let mut at = e.mem.u32(list);
        while read < size {
            if e.mem.u32(at) == 0 {
                write = read;
                read += 1;
                break;
            }
            read += 1;
            at = at.wrapping_add(4);
        }
    }
    if read < size {
        loop {
            let data = e.mem.u32(list);
            let value = e.mem.u32(data.wrapping_add((read as u32).wrapping_mul(4)));
            if value != 0 {
                e.mem
                    .set_u32(data.wrapping_add((write as u32).wrapping_mul(4)), value);
                write = write.wrapping_add(1);
            }
            read += 1;
            if read >= e.mem.u32(list + 4) as i32 {
                break;
            }
        }
    }
    if write != -1 {
        reserve_for_count(e, list, write);
        e.mem.set_u32(list + 4, write as u32);
    }
}

/// The capacity check in front of an `hkArray` store of `count` words: calls the memory router,
/// and when the capacity (low 30 bits of the third word) is below `count`, grows the array to
/// the larger of `count` and twice the capacity with `hkArrayUtil::_reserve` (`00c908c0`) from
/// the router's allocator (+0x10), 4-byte elements.
fn reserve_for_count(e: &mut Engine, array: u32, count: i32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if count < doubled { doubled } else { count };
        e.call(ARRAY_RESERVE_EXACT, &args![allocator, array, wanted, 4u32]);
    }
}

/// `00d07120` (`hkpSimulationIsland`, one `this` word; the engine map has only the unit): called
/// on both islands and the fixed island at the start of a merge.
const ISLAND_CHECK_00D07120: u32 = 0x00d0_7120;
/// `00d07180` (`hkpSimulationIsland`, `this` and one stack word): folds the counters of the second
/// island (+0x8 to +0x14) into the first.
const ISLAND_ADD_COUNTERS: u32 = 0x00d0_7180;
/// `00c910a0` (`hkpWorld`, `this` = the world, one stack word, a bare `RET 4`): called with the
/// kept island at the end of a merge.
const WORLD_AFTER_MERGE: u32 = 0x00c9_10a0;
/// The `hkpSimulationIsland` array at +0x38 (null-able entries whose word +0xc is the island
/// they belong to) and the entity field +0xa8 (index in the island) and +0xac/+0xb0 (the master
/// constraints: an array of 0x30-byte records and its 16-bit count).
const ENTITY_INDEX_IN_ISLAND: u32 = 0xa8;
const ENTITY_MASTERS: u32 = 0xac;
const ENTITY_MASTERS_COUNT: u32 = 0xb0;
/// Size of one master constraint record of an entity.
const CONSTRAINT_RECORD_BYTES: u32 = 0x30;

// Translated from 00cb4c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::internalMergeTwoIslands` (Xbox PDB), cdecl `(world, first, second)`:
/// under the critical-operations lock merges one island into the other. The island with more
/// entities (on a tie the one with the lower index in its list) is kept. When either is active
/// the inactive one is flagged (bit 2 set, bit 3 cleared in the byte at +0x26) and activated
/// first. The gone island's agent track, entities (their island and index fields updated),
/// the non-null words of its array at +0x38 (their island word +0xc updated), the master
/// constraints' owner (+8 of each constraint's first word) and counters move to the kept one;
/// the gone island leaves its list (its slot refilled by the last), the kept island's flag
/// bits are updated, it enters the dirty list when the gone one was in it, and the gone island is
/// removed from the dirty list and destroyed through its virtual destructor (slot 0, flag 1).
/// An active kept island with more entities than the first active island takes its place at the
/// front. The lock is released at the end.
pub fn hkp_world_operation_util_internal_merge_two_islands(
    e: &mut Engine,
    world: Ptr,
    first: Ptr,
    second: Ptr,
) {
    let world = world.addr();
    let (mut kept, mut gone) = (first.addr(), second.addr());
    let kept_entities = e.mem.u32(kept + ISLAND_ENTITY_COUNT) as i32;
    let gone_entities = e.mem.u32(gone + ISLAND_ENTITY_COUNT) as i32;
    let keep_first = kept_entities > gone_entities
        || (kept_entities == gone_entities
            && e.mem.u16(kept + ISLAND_INDEX) <= e.mem.u16(gone + ISLAND_INDEX));
    if !keep_first {
        std::mem::swap(&mut kept, &mut gone);
    }
    add_word(e, world + WORLD_LOCK_COUNT, 1);
    let kept_flags = e.mem.u8(kept + ISLAND_FLAGS_26);
    let kept_state = kept_flags & 3;
    let either_active = kept_state != 0 || e.mem.u8(gone + ISLAND_FLAGS_26) & 3 != 0;
    let either_marked = kept_flags & 0xc != 0 || e.mem.u8(gone + ISLAND_FLAGS_26) & 0xc != 0;
    if either_active {
        if kept_state == 0 {
            e.mem
                .set_u8(kept + ISLAND_FLAGS_26, (kept_flags & 0xf7) | 4);
            hkp_world_operation_util_internal_activate_island(
                e,
                Ptr::new(world),
                Ptr::new(kept),
                0,
            );
        } else {
            let gone_flags = e.mem.u8(gone + ISLAND_FLAGS_26);
            if gone_flags & 3 == 0 {
                e.mem
                    .set_u8(gone + ISLAND_FLAGS_26, (gone_flags & 0xf7) | 4);
                hkp_world_operation_util_internal_activate_island(
                    e,
                    Ptr::new(world),
                    Ptr::new(gone),
                    0,
                );
            }
        }
    }
    let merged_bits = e.mem.u8(gone + ISLAND_FLAGS_25) & 0xc;
    let flags = e.mem.u8(kept + ISLAND_FLAGS_25);
    e.mem.set_u8(kept + ISLAND_FLAGS_25, flags | merged_bits);
    e.call(ISLAND_CHECK_00D07120, &args![kept]);
    e.call(ISLAND_CHECK_00D07120, &args![gone]);
    let fixed_island = e.mem.u32(world + WORLD_FIXED_ISLAND);
    e.call(ISLAND_CHECK_00D07120, &args![fixed_island]);
    e.call(
        AGENT_APPEND_TRACK,
        &args![kept + ISLAND_AGENT_TRACK, gone + ISLAND_AGENT_TRACK],
    );
    // The entities of the gone island follow those of the kept one.
    let kept_count = e.mem.u32(kept + ISLAND_ENTITY_COUNT);
    let total = (e.mem.u32(gone + ISLAND_ENTITY_COUNT) as i32).wrapping_add(kept_count as i32);
    let mut position = kept_count & 0xffff;
    let entities = kept + ISLAND_ENTITIES;
    reserve_for_count(e, entities, total);
    e.mem.set_u32(entities + 4, total as u32);
    let mut index = 0u32;
    while (index as i32) < (e.mem.u32(gone + ISLAND_ENTITY_COUNT) as i32) {
        let gone_data = e.mem.u32(gone + ISLAND_ENTITIES);
        let entity = e.mem.u32(gone_data.wrapping_add(index * 4));
        let kept_data = e.mem.u32(entities);
        e.mem
            .set_u32(kept_data.wrapping_add((position & 0xffff) * 4), entity);
        let gone_data = e.mem.u32(gone + ISLAND_ENTITIES);
        let entity = e.mem.u32(gone_data.wrapping_add(index * 4));
        e.mem.set_u32(entity + ENTITY_ISLAND, kept);
        let gone_data = e.mem.u32(gone + ISLAND_ENTITIES);
        let entity = e.mem.u32(gone_data.wrapping_add(index * 4));
        e.mem
            .set_u16(entity + ENTITY_INDEX_IN_ISLAND, position as u16);
        index += 1;
        position = position.wrapping_add(1);
    }
    // The words of the array at +0x38.
    let mut at = e.mem.u32(kept + ISLAND_ARRAY_38 + 4) as i32;
    let total = (e.mem.u32(gone + ISLAND_ARRAY_38 + 4) as i32).wrapping_add(at);
    let list = kept + ISLAND_ARRAY_38;
    reserve_for_count(e, list, total);
    e.mem.set_u32(list + 4, total as u32);
    let mut index = 0u32;
    while (index as i32) < (e.mem.u32(gone + ISLAND_ARRAY_38 + 4) as i32) {
        let gone_list = e.mem.u32(gone + ISLAND_ARRAY_38);
        let value = e.mem.u32(gone_list.wrapping_add(index * 4));
        if value != 0 {
            let kept_list = e.mem.u32(list);
            e.mem
                .set_u32(kept_list.wrapping_add((at as u32).wrapping_mul(4)), value);
            let kept_list = e.mem.u32(list);
            let stored = e
                .mem
                .u32(kept_list.wrapping_add((at as u32).wrapping_mul(4)));
            e.mem.set_u32(stored + 0xc, kept);
            at = at.wrapping_add(1);
        }
        index += 1;
    }
    reserve_for_count(e, list, at);
    e.mem.set_u32(list + 4, at as u32);
    // The master constraints now belong to the kept island.
    let mut index = 0u32;
    while (index as i32) < (e.mem.u32(gone + ISLAND_ENTITY_COUNT) as i32) {
        let gone_data = e.mem.u32(gone + ISLAND_ENTITIES);
        let entity = e.mem.u32(gone_data.wrapping_add(index * 4));
        let mut record = e.mem.u32(entity + ENTITY_MASTERS);
        let mut remaining = i32::from(e.mem.u16(entity + ENTITY_MASTERS_COUNT)) - 1;
        while remaining >= 0 {
            let instance = e.mem.u32(record);
            record = record.wrapping_add(CONSTRAINT_RECORD_BYTES);
            remaining -= 1;
            e.mem.set_u32(instance + 8, kept);
        }
        index += 1;
    }
    e.call(ISLAND_ADD_COUNTERS, &args![kept, gone]);
    let moved = e.mem.u32(gone + ISLAND_CONSTRAINT_COUNT);
    let total = e
        .mem
        .u32(kept + ISLAND_CONSTRAINT_COUNT)
        .wrapping_add(moved);
    e.mem.set_u32(kept + ISLAND_CONSTRAINT_COUNT, total);
    let gone_index = e.mem.u16(gone + ISLAND_INDEX);
    e.mem.set_u32(gone + ISLAND_CONSTRAINT_COUNT, 0);
    // Out of the list it is in.
    let gone_list = if e.mem.u8(gone + ISLAND_FLAGS_26) & 3 != 0 {
        world + WORLD_ACTIVE_ISLANDS
    } else {
        world + WORLD_INACTIVE_ISLANDS
    };
    let list_size = e.mem.u32(gone_list + 4) as i32;
    if i32::from(gone_index) < list_size.wrapping_sub(1) {
        let data = e.mem.u32(gone_list);
        let last = e.mem.u32(
            data.wrapping_add((list_size as u32).wrapping_mul(4))
                .wrapping_sub(4),
        );
        e.mem
            .set_u32(data.wrapping_add(u32::from(gone_index) * 4), last);
        let data = e.mem.u32(gone_list);
        let moved = e.mem.u32(data.wrapping_add(u32::from(gone_index) * 4));
        e.mem.set_u16(moved + ISLAND_INDEX, gone_index);
    }
    add_word(e, gone_list + 4, u32::MAX);
    // The flag bytes of the kept island.
    let any_bits_01 = (e.mem.u8(kept + ISLAND_FLAGS_25) & 3 != 0
        || e.mem.u8(gone + ISLAND_FLAGS_25) & 3 != 0) as u8;
    let flags = e.mem.u8(kept + ISLAND_FLAGS_25);
    e.mem
        .set_u8(kept + ISLAND_FLAGS_25, flags ^ ((flags ^ any_bits_01) & 3));
    let flags = e.mem.u8(kept + ISLAND_FLAGS_25);
    let any_bits_45 = (flags & 0x30 != 0 || e.mem.u8(gone + ISLAND_FLAGS_25) & 0x30 != 0) as u8;
    e.mem.set_u8(
        kept + ISLAND_FLAGS_25,
        (((any_bits_45 << 4) ^ flags) & 0x30) ^ flags,
    );
    let marked = (u8::from(either_marked)) << 2;
    let flags = e.mem.u8(kept + ISLAND_FLAGS_26);
    e.mem
        .set_u8(kept + ISLAND_FLAGS_26, flags ^ ((marked ^ flags) & 0xc));
    if e.mem.u16(gone + ISLAND_DIRTY_INDEX) != NOT_IN_DIRTY_LIST
        && e.mem.u16(kept + ISLAND_DIRTY_INDEX) == NOT_IN_DIRTY_LIST
    {
        mark_island_dirty(e, world, kept);
    }
    hkp_world_operation_util_remove_island_from_dirty_list(e, Ptr::new(world), Ptr::new(gone));
    e.vcall(gone, 0, &args![1u32]);
    if e.mem.u8(kept + ISLAND_FLAGS_26) & 3 != 0 {
        let index = e.mem.u16(kept + ISLAND_INDEX);
        if index != 0 {
            let active = e.mem.u32(world + WORLD_ACTIVE_ISLANDS);
            let front = e.mem.u32(active);
            if (e.mem.u32(kept + ISLAND_ENTITY_COUNT) as i32)
                > (e.mem.u32(front + ISLAND_ENTITY_COUNT) as i32)
            {
                e.mem.set_u16(kept + ISLAND_INDEX, 0);
                e.mem.set_u16(front + ISLAND_INDEX, index);
                let active = e.mem.u32(world + WORLD_ACTIVE_ISLANDS);
                e.mem.set_u32(active, kept);
                let active = e.mem.u32(world + WORLD_ACTIVE_ISLANDS);
                e.mem
                    .set_u32(active.wrapping_add(u32::from(index) * 4), front);
            }
        }
    }
    e.call(WORLD_AFTER_MERGE, &args![world, kept]);
    release_critical_lock(e, world);
}

/// `hkUnionFind` methods (`run_00d27f70.rs`): constructor `(this, array, count)`, the one that
/// fills the group-size array `(this, array)` and the one that answers the index of a group
/// (`(this, array)`); `00d074c0` (`hkpSimulationIsland`, `this` = island, one stack word: the
/// union-find object; answers a byte that is true when the island needs no splitting).
const UNION_FIND_CONSTRUCTOR: u32 = 0x00d2_7f70;
const UNION_FIND_FILL_SIZES: u32 = 0x00d2_8120;
const UNION_FIND_PICK_GROUP: u32 = 0x00d2_81b0;
const ISLAND_BUILD_UNION_FIND: u32 = 0x00d0_74c0;
/// `hkArrayUtil::_reduce` (`00c909d0`, cdecl `(allocator, array, element size, in-place buffer,
/// capacity)`).
const ARRAY_REDUCE: u32 = 0x00c9_09d0;
/// `00d9ec10` (`this` and one stack word): swaps the three words (data, size, capacity) of two
/// `hkArray`s.
const SWAP_ARRAY_WORDS: u32 = 0x00d9_ec10;
/// `hkpWorld` pieces called with the island (`this` = world, one stack word): `00c911f0` after the
/// constraints and entities moved, `00c91230` at the end.
const WORLD_AFTER_SPLIT_MOVE: u32 = 0x00c9_11f0;
const WORLD_AFTER_SPLIT: u32 = 0x00c9_1230;
/// The allocator slots used here (byte offsets in its vtable): 4 allocates one block of the given
/// size, 8 and 0x10 free `(block, size)`.
const ALLOCATOR_ALLOCATE: u32 = 0x04;
const ALLOCATOR_FREE_BLOCK: u32 = 0x08;
const ALLOCATOR_FREE_ARRAY: u32 = 0x10;
/// Constraint virtual function (slot 0x10) that fills an `hkArray` with the entities it connects.
const CONSTRAINT_GET_ENTITIES: u32 = 0x10;
/// Fields of `hkpSimulationIsland`: +0x58 the first word of the agent track (bytes used in the
/// last sector), +0x5c/+0x60/+0x64 its `hkArray` of sectors, +0x8 the owner word, +0xc to +0x14
/// three counters, +0x24 a byte index.
const ISLAND_TRACK_USED: u32 = 0x58;
const ISLAND_TRACK_SECTORS: u32 = 0x5c;
const ISLAND_TRACK_SECTOR_COUNT: u32 = 0x60;
const ISLAND_TRACK_CAPACITY: u32 = 0x64;
const ISLAND_BYTE_INDEX: u32 = 0x24;
const ISLAND_FLAGS_MASK_MARKED: u8 = 0xc;
/// Bytes of one agent-track sector.
const SECTOR_BYTES: u32 = 0x200;
/// The scratch frame of `splitSimulationIsland`: the union-find object (+0), the router top word
/// the union-find array lives at (+0x10), the group-size array `hkLocalArray<int,32>` (+0x20, its
/// storage at +0x30), the word holding the pointer to the groups' flag bytes (+0xc0), the
/// constraint-entity array `hkLocalArray<hkpEntity*,16>` (+0x100) and the sector array
/// `hkLocalArray<byte*,1>` (+0x180, its storage at +0x18c).
const SPLIT_FRAME_BYTES: u32 = 0x1c0;
const SPLIT_UNION_FIND: u32 = 0;
const SPLIT_UNION_FIND_ARRAY: u32 = 0x10;
const SPLIT_GROUP_SIZES: u32 = 0x20;
const SPLIT_FLAGS_HOLDER: u32 = 0xc0;
const SPLIT_ENTITY_ARRAY: u32 = 0x100;
const SPLIT_SECTOR_ARRAY: u32 = 0x180;

/// The island an agent entry belongs to: the island of the entity behind the entry's first
/// collidable (`[entry + 0x10]`, adjusted by the signed byte at its +0x10) or, when that is the
/// fixed island (index 0xffff), the second collidable's (`[entry + 0x14]`).
fn agent_entry_island(e: &Engine, entry: u32) -> u32 {
    let island_behind = |collidable: u32| {
        let adjust = i32::from(e.mem.i8(collidable + 0x10)) as u32;
        e.mem
            .u32(collidable.wrapping_add(adjust).wrapping_add(0xcc))
    };
    let first = island_behind(e.mem.u32(entry + 0x10));
    if e.mem.u16(first + ISLAND_INDEX) == NOT_IN_DIRTY_LIST {
        island_behind(e.mem.u32(entry + 0x14))
    } else {
        first
    }
}

/// Frees an `hkArray` made on the stack (`hkLocalArray`): when its capacity word is not
/// negative (it owns memory) the memory goes back through the router's allocator (slot 0x10,
/// `capacity & 0x3fffffff` words of 4 bytes). The size is cleared first.
fn free_stack_array(e: &mut Engine, array: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, ALLOCATOR_FREE_ARRAY, &args![data, bytes]);
    }
}

// Translated from 00cb6060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::splitSimulationIsland` (Xbox PDB), cdecl `(island, world, list,
/// extra)`: splits the island into the connected groups of its entities (an `hkUnionFind` built
/// by `00d074c0` on the memory router's stack) and moves the groups' entities, constraints,
/// words of the array at +0x38 and agents into new islands appended to `list`.
///
/// When `00d074c0` answers true the island's marked bits (2 and 3 of the byte at +0x25) are
/// cleared and nothing else happens. Otherwise the group sizes are filled in and a flag byte per
/// group is prepared: with the marked bits set the groups of entities that are at rest are
/// merged by `00cb4890` (world byte +0xd5 set) or `00cb49e0` (clear), with them clear and the
/// world's minimum island size at +0xb4 non-zero by `00cb49e0`, and with the size zero every
/// flag is cleared. The inlined copies of those two functions in this body are the same code
/// (`00cb4890` and `00cb49e0` have no other callers), so they are called here. The flag of the
/// group the union-find picks (`00d281b0`) is swapped to index 0, group 0 stays in the island
/// (its marked bit 2 follows the flag); each further group makes a new island (allocator slot 4,
/// 0x6c bytes, constructor `00d07830`; byte +0x26 gets bit 0 and bit 2 set and bit 1 and 3
/// cleared, marked bit 2 from the flag, index in `list`, entity array reserved). When `extra`
/// is non-zero the island's entity array is swapped with it first. Every entity moves to its
/// group's island (island and index fields); the master constraints of entities that left have their
/// counters (+0xc to +0x14 of the island, three 16-bit counters of the constraint's info record)
/// and constraint count moved with `addConstraintInfo`. The words of the array at +0x38 go to the
/// island of the first entity of the constraint behind them (`getEntities`, virtual slot 0x10)
/// that is not in the world's fixed island. The agents follow: when few entities remain
/// (`(old count - first group size) * 8 < old count`) the entries of each sector of the island's
/// track that belong to another island are copied there and deallocated, otherwise the whole
/// track is taken apart sector by sector and every entry is copied to its island's track. The
/// sector array shrinks to the next power of two above its size plus 3, new islands' byte at
/// +0x24 become the island's plus their index, and the router's top is restored.
pub fn hkp_world_operation_util_split_simulation_island(
    e: &mut Engine,
    island: Ptr,
    world: Ptr,
    list: Ptr,
    extra: Ptr,
) {
    let frame = e.mem.alloc(SPLIT_FRAME_BYTES);
    split_simulation_island_body(
        e,
        frame,
        island.addr(),
        world.addr(),
        list.addr(),
        extra.addr(),
    );
    e.mem.free(frame);
}

fn split_simulation_island_body(
    e: &mut Engine,
    frame: u32,
    island: u32,
    world: u32,
    list: u32,
    extra: u32,
) {
    let union_find = frame + SPLIT_UNION_FIND;
    let union_find_array = frame + SPLIT_UNION_FIND_ARRAY;
    let sizes = frame + SPLIT_GROUP_SIZES;
    let flags_holder = frame + SPLIT_FLAGS_HOLDER;
    let entity_count = e.mem.u32(island + ISLAND_ENTITY_COUNT);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let router_start = e.mem.u32(router);
    let table_bytes = entity_count.wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80;
    e.mem
        .set_u32(router, table_bytes.wrapping_add(router_start));
    e.mem.set_u32(union_find_array, router_start);
    let count = e.mem.u32(island + ISLAND_ENTITY_COUNT);
    e.call(
        UNION_FIND_CONSTRUCTOR,
        &args![union_find, union_find_array, count],
    );
    let nothing_to_split = e
        .call(ISLAND_BUILD_UNION_FIND, &args![island, union_find])
        .bool();
    if nothing_to_split {
        let flags = e.mem.u8(island + ISLAND_FLAGS_25);
        e.mem.set_u8(island + ISLAND_FLAGS_25, flags & 0xf3);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, router_start);
        return;
    }
    // The group sizes: an `hkLocalArray<int, 32>` filled by the union-find.
    e.mem.set_u32(sizes, sizes + 0xc);
    e.mem.set_u32(sizes + 4, 0);
    e.mem.set_u32(sizes + 8, 0x8000_0020);
    e.call(UNION_FIND_FILL_SIZES, &args![union_find, sizes]);
    let groups = e.mem.u32(sizes + 4);
    // One flag byte per group (and one more), on the router's stack.
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let flags = e.mem.u32(router);
    e.mem.set_u32(
        router,
        (groups.wrapping_add(0x80) & 0xffff_ff80).wrapping_add(flags),
    );
    e.mem.set_u32(flags_holder, flags);
    let marked = e.mem.u8(island + ISLAND_FLAGS_25) & ISLAND_FLAGS_MASK_MARKED != 0;
    let mut only_one_group = false;
    if marked {
        if e.mem.u8(world + WORLD_RANKING_ENABLED) == 0 {
            fn_00cb49e0(
                e,
                world,
                union_find,
                island + ISLAND_ENTITIES,
                sizes,
                flags_holder,
            );
        } else {
            fn_00cb4890(
                e,
                island + ISLAND_ENTITIES,
                union_find,
                world,
                sizes,
                flags_holder,
            );
            only_one_group = e.mem.u32(sizes + 4) == 1;
        }
    } else if e.mem.u32(world + WORLD_MIN_ISLAND_SIZE) == 0 {
        e.call(
            MEM_SET,
            &args![flags, 0u32, e.mem.u32(sizes + 4).wrapping_add(1)],
        );
    } else {
        fn_00cb49e0(
            e,
            world,
            union_find,
            island + ISLAND_ENTITIES,
            sizes,
            flags_holder,
        );
    }
    if only_one_group {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        e.mem.set_u32(router, flags);
        release_split_frame(e, sizes, union_find_array);
        return;
    }
    // The group the union-find picks goes to index 0 of the flags.
    let picked = e
        .call(UNION_FIND_PICK_GROUP, &args![union_find, sizes])
        .i32() as u32;
    let at_picked = e.mem.u8(flags.wrapping_add(picked));
    let at_first = e.mem.u8(flags);
    e.mem.set_u8(flags, at_picked);
    e.mem.set_u8(flags.wrapping_add(picked), at_first);
    let group_sizes = e.mem.u32(sizes);
    let first_group_size = e.mem.u32(group_sizes);
    let old_count = e.mem.u32(island + ISLAND_ENTITY_COUNT);
    let groups = e.mem.u32(sizes + 4);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    if groups == 1 {
        e.mem.set_u32(router, flags);
        release_split_frame(e, sizes, union_find_array);
        return;
    }
    // The array of the groups' islands, group 0 being the island itself.
    let top = e.mem.u32(router);
    e.mem.set_u32(
        router,
        (groups.wrapping_mul(4).wrapping_add(0x7f) & 0xffff_ff80).wrapping_add(top),
    );
    let islands = top;
    let marked_bit = (e.mem.u8(flags) != 0) as u8 * 4;
    let island_flags = e.mem.u8(island + ISLAND_FLAGS_25);
    e.mem.set_u8(
        island + ISLAND_FLAGS_25,
        island_flags ^ ((marked_bit ^ island_flags) & ISLAND_FLAGS_MASK_MARKED),
    );
    e.mem.set_u32(islands, island);
    let mut group = 1u32;
    while (group as i32) < (groups as i32) {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let block = e
            .vcall(allocator, ALLOCATOR_ALLOCATE, &args![0x6cu32])
            .u32();
        e.mem.set_u16(block + 4, 0x6c);
        let created = e.call(ISLAND_CONSTRUCTOR, &args![block, world]).u32();
        let created_flags = e.mem.u8(created + ISLAND_FLAGS_26);
        e.mem
            .set_u8(created + ISLAND_FLAGS_26, (created_flags & 0xf5) | 5);
        let marked_bit = (e.mem.u8(flags.wrapping_add(group)) != 0) as u8 * 4;
        let created_flags = e.mem.u8(created + ISLAND_FLAGS_25);
        e.mem.set_u8(
            created + ISLAND_FLAGS_25,
            created_flags ^ ((marked_bit ^ created_flags) & ISLAND_FLAGS_MASK_MARKED),
        );
        let list_size = e.mem.u16(list + 4);
        e.mem.set_u16(created + ISLAND_INDEX, list_size);
        push_word(e, list, created);
        e.mem.set_u32(islands + group * 4, created);
        let size_data = e.mem.u32(sizes);
        let group_size = e.mem.u32(size_data.wrapping_add(group * 4));
        let entities = created + ISLAND_ENTITIES;
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = (e.mem.u32(entities + 8) & 0x3fff_ffff) as i32;
        if capacity < group_size as i32 {
            let allocator = e.mem.u32(router + 0x10);
            e.call(
                ARRAY_RESERVE_EXACT,
                &args![allocator, entities, group_size, 4u32],
            );
        }
        group += 1;
    }
    // The island's entities go to their groups' islands; the island keeps an empty array
    // (swapped with `extra` when given) that has room for the first group.
    let old_entities = e.mem.u32(island + ISLAND_ENTITIES);
    let moving = e.mem.u32(island + ISLAND_ENTITY_COUNT);
    let entities_array = island + ISLAND_ENTITIES;
    if extra != 0 {
        e.call(SWAP_ARRAY_WORDS, &args![entities_array, extra]);
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, 0);
        let first_size = e.mem.u32(e.mem.u32(sizes));
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = (e.mem.u32(entities_array + 8) & 0x3fff_ffff) as i32;
        if capacity < first_size as i32 {
            let allocator = e.mem.u32(router + 0x10);
            e.call(
                ARRAY_RESERVE_EXACT,
                &args![allocator, entities_array, first_size, 4u32],
            );
        }
    } else {
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, 0);
    }
    let mut index = 0u32;
    while (index as i32) < (moving as i32) {
        let union_find_values = e.mem.u32(union_find_array);
        let group = e.mem.u32(union_find_values.wrapping_add(index * 4));
        let target = e.mem.u32(islands.wrapping_add(group * 4));
        let entity = e.mem.u32(old_entities.wrapping_add(index * 4));
        e.mem.set_u32(entity + ENTITY_ISLAND, target);
        let position = e.mem.u32(target + ISLAND_ENTITY_COUNT);
        e.mem
            .set_u16(entity + ENTITY_INDEX_IN_ISLAND, position as u16);
        let target_entities = e.mem.u32(target + ISLAND_ENTITIES);
        e.mem.set_u32(
            target_entities.wrapping_add(position.wrapping_mul(4)),
            entity,
        );
        e.mem
            .set_u32(target + ISLAND_ENTITY_COUNT, position.wrapping_add(1));
        if island != target {
            let mut record = e.mem.u32(entity + ENTITY_MASTERS);
            let mut done = 0u32;
            if 0 < e.mem.u16(entity + ENTITY_MASTERS_COUNT) {
                loop {
                    let owner = e.mem.u32(island + 8);
                    let first_counter = u32::from(e.mem.u16(record + 0x14));
                    let second_counter = u32::from(e.mem.u16(record + 0x16));
                    let third_counter = u32::from(e.mem.u16(record + 0x18));
                    add_word(e, island + ISLAND_CONSTRAINT_COUNT, u32::MAX);
                    let instance = e.mem.u32(record);
                    let linked = e.mem.u32(instance + 0x30);
                    if linked != 0 {
                        for (offset, counter) in [
                            (0xcu32, first_counter),
                            (0x10, second_counter),
                            (0x14, third_counter),
                        ] {
                            let value = e.mem.u32(island + offset).wrapping_sub(counter);
                            e.mem.set_u32(island + offset, value);
                        }
                        let linked = e.mem.u32(instance + 0x30);
                        for (offset, counter) in [
                            (0x16u32, second_counter),
                            (0x18, third_counter),
                            (0x14, first_counter),
                        ] {
                            let value = e.mem.u16(linked + offset).wrapping_sub(counter as u16);
                            e.mem.set_u16(linked + offset, value);
                        }
                    }
                    add_word(e, target + ISLAND_CONSTRAINT_COUNT, 1);
                    let instance = e.mem.u32(record);
                    e.mem.set_u32(instance + 8, target);
                    let info = e.mem.alloc(0x10);
                    e.mem.set_u32(info, owner);
                    e.mem.set_u32(info + 4, first_counter);
                    e.mem.set_u32(info + 8, second_counter);
                    e.mem.set_u32(info + 0xc, third_counter);
                    let instance = e.mem.u32(record);
                    hkp_constraint_owner_add_constraint_info(
                        e,
                        Ptr::new(target),
                        Ptr::new(instance),
                        Ptr::new(info),
                    );
                    e.mem.free(info);
                    done += 1;
                    record = record.wrapping_add(CONSTRAINT_RECORD_BYTES);
                    if (done as i32) >= i32::from(e.mem.u16(entity + ENTITY_MASTERS_COUNT)) {
                        break;
                    }
                }
            }
        }
        index += 1;
    }
    // The words of the array at +0x38 follow the entities of the constraints behind them.
    let fixed_island = e.mem.u32(world + WORLD_FIXED_ISLAND);
    let words_data = e.mem.u32(island + ISLAND_ARRAY_38);
    let words_count = e.mem.u32(island + ISLAND_ARRAY_38 + 4);
    e.mem.set_u32(island + ISLAND_ARRAY_38 + 4, 0);
    let entity_array = frame + SPLIT_ENTITY_ARRAY;
    let mut word_index = 0u32;
    while (word_index as i32) < (words_count as i32) {
        let item = e.mem.u32(words_data.wrapping_add(word_index * 4));
        if item != 0 {
            e.mem.set_u32(entity_array, entity_array + 0xc);
            e.mem.set_u32(entity_array + 4, 0);
            e.mem.set_u32(entity_array + 8, 0x8000_0010);
            e.vcall(item, CONSTRAINT_GET_ENTITIES, &args![entity_array]);
            let entity_total = e.mem.u32(entity_array + 4);
            let mut owner = 0u32;
            if (entity_total as i32) > 0 {
                let entity_data = e.mem.u32(entity_array);
                let mut at = 0u32;
                loop {
                    let entity = e.mem.u32(entity_data.wrapping_add(at * 4));
                    owner = e.mem.u32(entity + ENTITY_ISLAND);
                    if owner != fixed_island {
                        break;
                    }
                    at += 1;
                    if (at as i32) >= (entity_total as i32) {
                        break;
                    }
                }
            }
            free_stack_array(e, entity_array);
            e.mem.set_u32(item + 0xc, owner);
            e.mem.set_u32(entity_array, 0);
            e.mem.set_u32(entity_array + 8, 0x8000_0000);
            push_word(e, owner + ISLAND_ARRAY_38, item);
        }
        word_index += 1;
    }
    e.call(WORLD_AFTER_SPLIT_MOVE, &args![world, island]);
    // The agents.
    let few_remain =
        (old_count.wrapping_sub(first_group_size) as i32).wrapping_mul(8) < (old_count as i32);
    if few_remain {
        let track = island + ISLAND_TRACK_USED;
        let mut sector_index = 0u32;
        while (sector_index as i32) < (e.mem.u32(island + ISLAND_TRACK_SECTOR_COUNT) as i32) {
            let sectors = e.mem.u32(island + ISLAND_TRACK_SECTORS);
            let mut start = e.mem.u32(sectors.wrapping_add(sector_index * 4));
            let length = fn_00cb7460(e, Ptr::new(track), sector_index);
            let mut cursor = start;
            if start < length.wrapping_add(start) {
                loop {
                    let owner = agent_entry_island(e, cursor);
                    if owner == island {
                        cursor = cursor.wrapping_add(u32::from(e.mem.u8(cursor + 3)));
                    } else {
                        let offset = cursor.wrapping_sub(start);
                        e.call(AGENT_COPY_AND_RELINK_ENTRY, &args![owner + 0x58, cursor]);
                        e.call(AGENT_DEALLOCATE_ENTRY, &args![track, cursor]);
                        if (sector_index as i32)
                            >= (e.mem.u32(island + ISLAND_TRACK_SECTOR_COUNT) as i32)
                        {
                            break;
                        }
                        let sectors = e.mem.u32(island + ISLAND_TRACK_SECTORS);
                        start = e.mem.u32(sectors.wrapping_add(sector_index * 4));
                        cursor = start.wrapping_add(offset);
                    }
                    let length = fn_00cb7460(e, Ptr::new(track), sector_index);
                    if cursor >= length.wrapping_add(start) {
                        break;
                    }
                }
            }
            sector_index += 1;
        }
    } else {
        let sector_array = frame + SPLIT_SECTOR_ARRAY;
        let storage = sector_array + 0xc;
        let used_word = sector_array + 0x20;
        e.mem.set_u32(sector_array, storage);
        e.mem.set_u32(sector_array + 4, 0);
        e.mem.set_u32(sector_array + 8, 0x8000_0001);
        e.mem.set_u32(used_word, SECTOR_BYTES);
        let sector_count = e.mem.u32(island + ISLAND_TRACK_SECTOR_COUNT) as i32;
        if sector_count == 1 {
            let sectors = e.mem.u32(island + ISLAND_TRACK_SECTORS);
            let only = e.mem.u32(sectors);
            e.mem.set_u32(storage, only);
            e.mem.set_u32(sector_array + 4, 1);
            e.mem.set_u32(island + ISLAND_TRACK_SECTOR_COUNT, 0);
        } else if sector_count > 1 {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let capacity = (e.mem.u32(sector_array + 8) & 0x3fff_ffff) as i32;
            if capacity < 2 {
                let allocator = e.mem.u32(router + 0x10);
                e.call(
                    ARRAY_RESERVE_EXACT,
                    &args![allocator, sector_array, 2u32, 4u32],
                );
            }
            e.call(
                SWAP_ARRAY_WORDS,
                &args![sector_array, island + ISLAND_TRACK_SECTORS],
            );
        }
        let sector_total = e.mem.u32(sector_array + 4) as i32;
        let last_used = e.mem.u32(island + ISLAND_TRACK_USED);
        e.mem
            .set_u32(island + ISLAND_TRACK_USED, e.mem.u32(used_word));
        e.mem.set_u32(used_word, last_used);
        let mut sector_index = 0i32;
        while sector_index < sector_total {
            let sector_data = e.mem.u32(sector_array);
            let sector = e
                .mem
                .u32(sector_data.wrapping_add((sector_index as u32) * 4));
            sector_index += 1;
            let sector_bytes = if sector_index == (e.mem.u32(sector_array + 4) as i32) {
                e.mem.u32(used_word)
            } else {
                SECTOR_BYTES
            };
            let end = sector.wrapping_add(sector_bytes);
            let mut cursor = sector;
            if cursor < end {
                loop {
                    let owner = agent_entry_island(e, cursor);
                    e.call(AGENT_COPY_AND_RELINK_ENTRY, &args![owner + 0x58, cursor]);
                    cursor = cursor.wrapping_add(u32::from(e.mem.u8(cursor + 3)));
                    if cursor >= end {
                        break;
                    }
                }
            }
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.vcall(
                allocator,
                ALLOCATOR_FREE_BLOCK,
                &args![sector, SECTOR_BYTES],
            );
            if sector_index >= (e.mem.u32(sector_array + 4) as i32) {
                break;
            }
        }
        free_stack_array(e, sector_array);
    }
    e.call(WORLD_AFTER_SPLIT, &args![world, island]);
    // The sector array shrinks to the next power of two above its size plus 3.
    let capacity_word = e.mem.u32(island + ISLAND_TRACK_CAPACITY);
    let track_sectors = island + ISLAND_TRACK_SECTORS;
    if (capacity_word as i32) >= 0 {
        let mut smeared = e.mem.u32(track_sectors + 4).wrapping_add(3);
        smeared |= smeared >> 16;
        smeared |= smeared >> 8;
        smeared |= smeared >> 4;
        smeared |= smeared >> 2;
        let wanted = ((smeared >> 1) | smeared).wrapping_add(1) as i32;
        if wanted < 1 {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.call(
                ARRAY_REDUCE,
                &args![allocator, track_sectors, 4u32, track_sectors + 0xc, 1u32],
            );
        } else if wanted < (capacity_word & 0x3fff_ffff) as i32 {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let allocator = e.mem.u32(router + 0x10);
            e.call(
                ARRAY_REDUCE,
                &args![allocator, track_sectors, 4u32, 0u32, wanted as u32],
            );
        }
    }
    let mut group = 1u32;
    while (group as i32) < (groups as i32) {
        let created = e.mem.u32(islands.wrapping_add(group * 4));
        let byte = e
            .mem
            .u8(island + ISLAND_BYTE_INDEX)
            .wrapping_add(group as u8);
        e.mem.set_u8(created + ISLAND_BYTE_INDEX, byte);
        group += 1;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, islands);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, flags);
    release_split_frame(e, sizes, union_find_array);
}

/// The common exit of `splitSimulationIsland`: the group-size array is freed and cleared, and the
/// router's top goes back to where the union-find array started.
fn release_split_frame(e: &mut Engine, sizes: u32, union_find_array: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(sizes + 8);
    e.mem.set_u32(sizes + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(sizes);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, ALLOCATOR_FREE_ARRAY, &args![data, bytes]);
    }
    e.mem.set_u32(sizes, 0);
    let start = e.mem.u32(union_find_array);
    e.mem.set_u32(sizes + 8, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, start);
}

// Translated from 00cb6e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::splitSimulationIsland` second overload (Xbox PDB name
/// `splitSimulationIsland_ov2`), cdecl `(world, island)`: splits the island with
/// `splitSimulationIsland(island, world, list, 0)` into the world's active list (island state in
/// bits 0-1 of the byte at +0x26 not zero) or inactive list, after clearing bits 0 and 1 of its
/// byte at +0x25. Every island that was added to the list takes the original's state in bits 0-1
/// and 2-3 of its +0x26 byte; when the original was active but the split left its wanted state
/// (bits 2-3) at zero, each new island is also marked inactive (`markIslandInactive`). The new
/// islands' byte at +0x24 is the original's plus the island's offset in the new part.
pub fn hkp_world_operation_util_split_simulation_island_ov2(
    e: &mut Engine,
    world: Ptr,
    island: Ptr,
) {
    let (world, island) = (world.addr(), island.addr());
    let original_flags = e.mem.u8(island + ISLAND_FLAGS_26);
    let state = original_flags & 3;
    let list = if state != 0 {
        world + WORLD_ACTIVE_ISLANDS
    } else {
        world + WORLD_INACTIVE_ISLANDS
    };
    let flags = e.mem.u8(island + ISLAND_FLAGS_25);
    e.mem.set_u8(island + ISLAND_FLAGS_25, flags & 0xfc);
    let old_size = e.mem.u32(list + 4);
    hkp_world_operation_util_split_simulation_island(
        e,
        Ptr::new(island),
        Ptr::new(world),
        Ptr::new(list),
        Ptr::NULL,
    );
    let wanted_state = (e.mem.u8(island + ISLAND_FLAGS_26) >> 2) & 3;
    let mut index = old_size;
    while (index as i32) < (e.mem.u32(list + 4) as i32) {
        let data = e.mem.u32(list);
        let created = e.mem.u32(data.wrapping_add(index.wrapping_mul(4)));
        let created_flags = e.mem.u8(created + ISLAND_FLAGS_26);
        e.mem.set_u8(
            created + ISLAND_FLAGS_26,
            (((created_flags & 0xf3) | (state << 2)) & 0xfc) | state,
        );
        if state != 0 && wanted_state == 0 {
            hkp_world_operation_util_mark_island_inactive(e, Ptr::new(world), Ptr::new(created));
        }
        let byte = e
            .mem
            .u8(island + ISLAND_BYTE_INDEX)
            .wrapping_sub(old_size as u8)
            .wrapping_add(index as u8);
        e.mem.set_u8(created + ISLAND_BYTE_INDEX, byte);
        index += 1;
    }
}

// Translated from 00cb6f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::splitSimulationIslands` (Xbox PDB), cdecl `(world)`: when the world
/// wants island management (byte +0xc8), walks the active islands from the last to the first
/// and splits each one whose byte at +0x25 has bit 0 or 1 set (`splitSimulationIsland`, second
/// overload).
pub fn hkp_world_operation_util_split_simulation_islands(e: &mut Engine, world: Ptr) {
    let world = world.addr();
    if e.mem.u8(world + WORLD_WANT_ISLANDS) == 0 {
        return;
    }
    let mut index = (e.mem.u32(world + WORLD_ACTIVE_ISLANDS + 4) as i32).wrapping_sub(1);
    while index >= 0 {
        let data = e.mem.u32(world + WORLD_ACTIVE_ISLANDS);
        let island = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
        if e.mem.u8(island + ISLAND_FLAGS_25) & 3 != 0 {
            hkp_world_operation_util_split_simulation_island_ov2(
                e,
                Ptr::new(world),
                Ptr::new(island),
            );
        }
        index = index.wrapping_sub(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb6060, hkp_world_operation_util_split_simulation_island(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00cb6e70, hkp_world_operation_util_split_simulation_island_ov2(Ptr, Ptr)),
        entry!(0x00cb6f30, hkp_world_operation_util_split_simulation_islands(Ptr)),
        entry!(0x00cb4c60, hkp_world_operation_util_internal_merge_two_islands(Ptr, Ptr, Ptr)),
        entry!(0x00cb5100, hkp_world_operation_util_internal_activate_island(Ptr, Ptr, u32)),
        entry!(0x00cb5310, hkp_world_operation_util_internal_deactivate_island(Ptr, Ptr)),
        entry!(0x00cb5420, hkp_world_operation_util_mark_island_inactive(Ptr, Ptr)),
        entry!(0x00cb55d0, hkp_world_operation_util_cleanup_dirty_islands(Ptr)),
        entry!(0x00cb3ca0, hkp_world_operation_util_add_constraint_immediately(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00cb3d10, find_small_sparse_active_island(Ptr, Ptr, i32, Ptr) -> Ptr),
        entry!(0x00cb3dd0, hkp_world_operation_util_remove_constraint_immediately(Ptr, Ptr, u32)),
        entry!(0x00cb3ee0, hkp_world_operation_util_remove_attached_agents_connecting_the_entity_and_a_fixed_partner_entity_plus(Ptr, Ptr, Ptr, u32)),
        entry!(0x00cb3fd0, hkp_world_operation_util_remove_island_from_dirty_list(Ptr, Ptr)),
        entry!(0x00cb4000, fn_00cb4000(Ptr, Ptr)),
        entry!(0x00cb4230, hkp_world_operation_util_replace_motion_object(Ptr, u32, u32, u32, Ptr)),
        entry!(0x00cb4640, hkp_world_operation_util_add_entity_si(Ptr, Ptr, u32)),
        entry!(0x00cb4740, hkp_world_operation_util_remove_entity_si(Ptr, Ptr)),
        entry!(0x00cb4790, fn_00cb4790(Ptr, Ptr, Ptr)),
        entry!(0x00cb5000, fn_00cb5000(Ptr, Ptr)),
        entry!(0x00cb5500, fn_00cb5500(Ptr, Ptr)),
        entry!(0x00cb5570, hkp_world_operation_util_merge_islands(Ptr, Ptr, Ptr)),
        entry!(0x00cb5750, hkp_world_operation_util_update_entity_bp(Ptr, Ptr)),
        entry!(0x00cb59f0, hkp_world_operation_util_add_entity_bp(Ptr, Ptr)),
        entry!(0x00cb5b10, hkp_world_operation_util_add_phantom_bp(Ptr, Ptr)),
        entry!(0x00cb5c10, hkp_world_operation_util_remove_entity_bp(Ptr, Ptr)),
        entry!(0x00cb5d00, hkp_world_operation_util_remove_phantom_bp(Ptr, Ptr)),
        entry!(0x00cb5dd0, fn_00cb5dd0(Ptr, Ptr, Ptr)),
        entry!(0x00cb5f50, hkp_world_operation_util_add_actions_to_entitys_island(Ptr, Ptr, Ptr)),
        entry!(0x00cb6f70, hkp_world_operation_util_set_rigid_body_motion_type(Ptr, u32, u32, u32)),
        entry!(0x00cb7460, fn_00cb7460(Ptr, u32) -> u32),
        entry!(0x00cb74d0, hkp_constraint_owner_add_constraint_info(Ptr, Ptr, Ptr)),
        entry!(0x00cb_3c80, hkp_world_operation_util_add_constraint_to_critical_locked_island(Ptr, Ptr)),
        entry!(0x00cb_3da0, hkp_world_operation_util_remove_constraint_from_critical_locked_island(Ptr, Ptr)),
        entry!(0x00cb_4890, fn_00cb4890(u32, u32, u32, u32, u32)),
        entry!(0x00cb_49e0, fn_00cb49e0(u32, u32, u32, u32, u32)),
        entry!(0x00cb_5480, fn_00cb5480(Ptr, Ptr)),
        entry!(0x00cb_7480, fn_00cb7480(Ptr, Ptr, Ptr)),
        entry!(0x00dd_7d00, hk_algorithm_quick_sort_recursive_hkp_simulation_island(u32, i32, i32, u32)),
        entry!(0x00cb_3f90, fn_00cb3f90(Ptr, Ptr, Ptr) -> Ptr),
    ]
}

// Translated from 00cb3ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::addConstraintImmediately` (Xbox PDB): under the critical-operations
/// lock adds the constraint to the world (`hkpWorldConstraintUtil::addConstraint`), tells the
/// constraint listeners when `notify` is set, and returns the constraint.
pub fn hkp_world_operation_util_add_constraint_immediately(
    e: &mut Engine,
    world: Ptr,
    constraint: Ptr,
    notify: u32,
) -> Ptr {
    let world_addr = world.addr();
    add_word(e, world_addr + WORLD_LOCK_COUNT, 1);
    e.call(
        WORLD_CONSTRAINT_UTIL_ADD,
        &args![world_addr, constraint.addr()],
    );
    if notify != 0 {
        e.call(FIRE_CONSTRAINT_ADDED, &args![world_addr, constraint.addr()]);
    }
    release_critical_lock(e, world_addr);
    constraint
}

// Translated from 00cb3d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `findSmallSparseActiveIsland` (Xbox PDB): looks through the last 20 islands of `array`
/// (the world's active islands, from the end) for one other than `skip` whose size (the larger of
/// its entity and constraint counts) plus `extra` stays under the world's minimum desired size
/// and that is marked active. The island found gets its "sparse" flag set and is returned;
/// null otherwise. Takes the world in `EAX` and the array in `ECX` in the exe.
pub fn find_small_sparse_active_island(
    e: &mut Engine,
    world: Ptr,
    array: Ptr,
    extra: i32,
    skip: Ptr,
) -> Ptr {
    let world_addr = world.addr();
    let minimum = e.mem.i32(world_addr + WORLD_MIN_ISLAND_SIZE);
    if extra >= minimum || e.mem.u8(world_addr + WORLD_WANT_ISLANDS) == 0 {
        return Ptr::new(0);
    }
    let size = e.mem.i32(array.addr() + 4);
    let below_window = size.wrapping_sub(0x14);
    let lowest = if below_window < 0 { 0 } else { below_window };
    let data = e.mem.u32(array.addr());
    let mut index = size.wrapping_sub(1);
    while index >= lowest {
        let island = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
        if island != skip.addr() {
            let entities = e.mem.i32(island + ISLAND_ENTITY_COUNT);
            let constraints = e.mem.i32(island + ISLAND_CONSTRAINT_COUNT);
            let island_size = if entities > constraints {
                entities
            } else {
                constraints
            };
            if e.mem.u8(island + ISLAND_FLAGS_26) & 0xc != 0
                && island_size.wrapping_add(extra) < minimum
            {
                let flags = e.mem.u8(island + ISLAND_FLAGS_25);
                e.mem.set_u8(island + ISLAND_FLAGS_25, (flags & 0xf7) | 4);
                return Ptr::new(island);
            }
        }
        index -= 1;
    }
    Ptr::new(0)
}

// Translated from 00cb3dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::removeConstraintImmediately` (Xbox PDB): marks the object at
/// `constraint + 8` (flag byte +0x25: `& 0xfd | 1`), then removes the constraint
/// (`hkpWorldConstraintUtil::removeConstraint`), telling the constraint listeners first when
/// `notify` is set and the world has any. When no critical lock is held it takes one for the
/// removal and releases it afterwards.
pub fn hkp_world_operation_util_remove_constraint_immediately(
    e: &mut Engine,
    world: Ptr,
    constraint: Ptr,
    notify: u32,
) {
    let world_addr = world.addr();
    let constraint_addr = constraint.addr();
    let owner = e.mem.u32(constraint_addr + 8);
    let flags = e.mem.u8(owner + ISLAND_FLAGS_25);
    e.mem.set_u8(owner + ISLAND_FLAGS_25, (flags & 0xfd) | 1);
    let locked = e.mem.u32(world_addr + WORLD_LOCK_COUNT) != 0;
    if !locked {
        e.mem.set_u32(world_addr + WORLD_LOCK_COUNT, 1);
    }
    if notify != 0 && e.mem.u32(world_addr + WORLD_CONSTRAINT_LISTENERS_SIZE) != 0 {
        e.call(FIRE_CONSTRAINT_REMOVED, &args![world_addr, constraint_addr]);
    }
    e.call(WORLD_CONSTRAINT_UTIL_REMOVE, &args![constraint_addr]);
    if !locked {
        release_critical_lock(e, world_addr);
    }
}

// Translated from 00cb3ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::removeAttachedAgentsConnectingTheEntityAndAFixedPartnerEntityPlus`
/// (Xbox PDB): for each collision entry of the entity (cdecl, four words): when the partner's
/// owner is fixed, the agent is copied to `fixed_partner_track` and deallocated from
/// `island_track`; otherwise, when `new_type` is `MOTION_FIXED` and the agent's island is the
/// entity's, it is copied to the partner island's track (+0x58) and deallocated.
pub fn hkp_world_operation_util_remove_attached_agents_connecting_the_entity_and_a_fixed_partner_entity_plus(
    e: &mut Engine,
    island_track: Ptr,
    entity: Ptr,
    fixed_partner_track: Ptr,
    new_type: u32,
) {
    let entity_addr = entity.addr();
    let entries = e
        .call(
            LINKED_COLLIDABLE_ENTRIES,
            &args![entity_addr + ENTITY_COLLIDABLE],
        )
        .u32();
    let mut index = 0i32;
    while index < e.mem.i32(entries + 4) {
        let entry = e
            .mem
            .u32(entries)
            .wrapping_add((index as u32).wrapping_mul(8));
        let partner = e.mem.u32(entry + 4);
        let owner = partner.wrapping_add(e.mem.i8(partner + 0x10) as i32 as u32);
        if e.mem.u8(owner + ENTITY_MOTION_TYPE) as u32 == MOTION_FIXED {
            let agent = e.mem.u32(entry);
            e.call(
                AGENT_COPY_AND_RELINK_ENTRY,
                &args![fixed_partner_track.addr(), agent],
            );
            e.call(AGENT_DEALLOCATE_ENTRY, &args![island_track.addr(), agent]);
        } else if new_type == MOTION_FIXED {
            let agent = e.mem.u32(entry);
            let entity_island = e.mem.u32(entity_addr + ENTITY_ISLAND);
            let owner_island = e.mem.u32(owner + ENTITY_ISLAND);
            let found = e
                .call(
                    AGENT_UTIL_GET_ISLAND_FROM_ENTRY,
                    &args![agent, entity_island, owner_island],
                )
                .u32();
            if found == e.mem.u32(entity_addr + ENTITY_ISLAND) {
                let agent = e.mem.u32(entry);
                let owner_island = e.mem.u32(owner + ENTITY_ISLAND);
                e.call(
                    AGENT_COPY_AND_RELINK_ENTRY,
                    &args![owner_island + ISLAND_AGENT_TRACK, agent],
                );
                e.call(AGENT_DEALLOCATE_ENTRY, &args![island_track.addr(), agent]);
            }
        }
        index += 1;
    }
}

// Translated from 00cb3fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::removeIslandFromDirtyList` (Xbox PDB): when the island is in the
/// world's dirty list, nulls its slot there and marks it as not in the list.
pub fn hkp_world_operation_util_remove_island_from_dirty_list(
    e: &mut Engine,
    world: Ptr,
    island: Ptr,
) {
    let index = e.mem.u16(island.addr() + ISLAND_DIRTY_INDEX);
    if index != NOT_IN_DIRTY_LIST {
        let data = e.mem.u32(world.addr() + WORLD_DIRTY_ISLANDS);
        e.mem.set_u32(data + index as u32 * 4, 0);
        e.mem
            .set_u16(island.addr() + ISLAND_DIRTY_INDEX, NOT_IN_DIRTY_LIST);
    }
}

// Translated from 00cb4000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (inside `hkpWorldOperationUtil`): takes the island out of the world's active
/// islands (when its flag byte +0x26 has bits 0-1 set) or inactive islands, filling the hole
/// with the last island, then removes it from the dirty list.
pub fn fn_00cb4000(e: &mut Engine, world: Ptr, island: Ptr) {
    let world_addr = world.addr();
    let island_addr = island.addr();
    let list = if e.mem.u8(island_addr + ISLAND_FLAGS_26) & 3 != 0 {
        world_addr + WORLD_ACTIVE_ISLANDS
    } else {
        world_addr + WORLD_INACTIVE_ISLANDS
    };
    let slot = e.mem.u16(island_addr + ISLAND_INDEX) as u32;
    let data = e.mem.u32(list);
    let size = e.mem.u32(list + 4);
    let last = e
        .mem
        .u32(data.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4));
    e.mem.set_u32(data.wrapping_add(slot * 4), last);
    let index = e.mem.u16(island_addr + ISLAND_INDEX);
    let data = e.mem.u32(list);
    let moved = e.mem.u32(data.wrapping_add(index as u32 * 4));
    e.mem.set_u16(moved + ISLAND_INDEX, index);
    add_word(e, list + 4, u32::MAX);
    hkp_world_operation_util_remove_island_from_dirty_list(e, world, island);
}

// Translated from 00cb4230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::replaceMotionObject` (Xbox PDB), cdecl, five words:
/// `new_is_dynamic` and `old_is_dynamic` are the bytes the caller computed from the new and
/// old motion types (neither fixed nor keyframed); `world` may be null.
///
/// Switching to a fixed or keyframed motion (`new_is_dynamic` clear): copies the entity's
/// 0x120-byte motion, builds the fixed or keyframed motion in place from the entity's position
/// and rotation (+0x120, +0x160), copies the motion state (a fixed motion is also frozen at
/// the world's time, or at `1 / rate + time` of the old state without a world), and either
/// keeps a heap copy of the old dynamic motion at +0x118 of the new one (`old_is_dynamic`) or
/// hands the old saved motion on. Switching to a dynamic motion: either restores the motion
/// saved at +0x1f8 (when the old one was not dynamic) or builds a dynamic motion with
/// `hkpRigidBody::createDynamicRigidMotion` from the old one's mass and inertia. The exe's
/// first stores to the copy (vtable pointer `0x010c6c64` and a zero saved-motion word at
/// +0x118) are overwritten by the copy loop; they are kept as the exe makes them. The two
/// `if (this)` tests of the placement new in front of the fixed and keyframed motion
/// constructors (`ECX = entity + 0xe0`) are kept too: a null motion gives a null new motion.
pub fn hkp_world_operation_util_replace_motion_object(
    e: &mut Engine,
    entity: Ptr,
    new_type: u32,
    new_is_dynamic: u32,
    old_is_dynamic: u32,
    world: Ptr,
) {
    let entity_addr = entity.addr();
    let world_addr = world.addr();
    let motion = entity_addr + ENTITY_MOTION;
    let new_is_dynamic = new_is_dynamic as u8 != 0;
    let old_is_dynamic = old_is_dynamic as u8 != 0;
    // The exe's frame (`AND ESP,0xfffffff0`, `SUB ESP,0x164`, three pushes): 0x170 bytes from a
    // 16-byte boundary, the 0x120-byte motion copy at +0x20 (its vtable stored at 00cb4312)
    // followed by the inertia matrix at +0x140 (00cb434c).
    with_aligned_stack(e, 0x170, |e, frame| {
        let copy = frame + 0x20;
        let mut copied = false;
        if !new_is_dynamic {
            // The exe stores the copy's vtable pointer and a zero at +0x118 first.
            e.mem.set_u32(copy, MOTION_COPY_VTABLE);
            e.mem.set_u32(copy + 0x118, 0);
            let bytes = e.mem.bytes(motion, MOTION_BYTES);
            e.mem.write(copy, &bytes);
            copied = true;
            let new_motion;
            if new_type == MOTION_FIXED {
                new_motion = if motion != 0 {
                    e.call(
                        FIXED_MOTION_CONSTRUCTOR,
                        &args![motion, entity_addr + 0x120, entity_addr + 0x160],
                    )
                    .u32()
                } else {
                    0
                };
                e.call(MOTION_STATE_ASSIGN, &args![new_motion + 0x10, copy + 0x10]);
                let flag = e.mem.u8(copy + 9);
                e.mem.set_u8(new_motion + 9, flag);
                let rate = e.mem.f32(copy + 0x6c);
                if rate != 0.0 {
                    let time = if world_addr != 0 {
                        e.call(WORLD_GET_CURRENT_TIME, &args![world_addr]).f32()
                    } else {
                        let start = e.mem.f32(copy + 0x5c);
                        (1.0f64 / rate as f64 + start as f64) as f32
                    };
                    e.call(
                        FREEZE_MOTION_STATE,
                        &args![time.to_bits(), new_motion + 0x10],
                    );
                }
            } else {
                new_motion = if motion != 0 {
                    e.call(
                        KEYFRAMED_MOTION_CONSTRUCTOR,
                        &args![motion, entity_addr + 0x120, entity_addr + 0x160],
                    )
                    .u32()
                } else {
                    0
                };
                e.call(MOTION_COPY_STATE_AND_VELOCITIES, &args![copy, new_motion]);
            }
            if old_is_dynamic {
                let router = e.call(MEMORY_ROUTER, &args![]).u32();
                let allocator = e.mem.u32(router + 0x10);
                let block = e.vcall(allocator, 4, &args![MOTION_BYTES]).u32();
                // The exe's stores into the new block that the copy overwrites.
                e.mem.set_u16(block + 4, MOTION_BYTES as u16);
                e.mem.set_u32(block + 0x118, 0);
                e.mem.set_u32(block, MOTION_COPY_VTABLE);
                let bytes = e.mem.bytes(copy, MOTION_BYTES);
                e.mem.write(block, &bytes);
                e.mem.set_u16(block + 4, MOTION_BYTES as u16);
                e.mem.set_u16(block + 6, 1);
                e.mem.set_u32(new_motion + 0x118, block);
                let byte_2a = e.mem.i8(entity_addr + ENTITY_BYTE_2A);
                e.mem.set_i16(new_motion + 0x11c, byte_2a as i16);
                let shape = e.mem.u32(entity_addr + ENTITY_COLLIDABLE);
                e.call(
                    ENTITY_SET_CACHED_SHAPE_DATA,
                    &args![entity_addr, world_addr, shape],
                );
            } else {
                let saved = e.mem.u32(copy + 0x118);
                e.vcall(new_motion, 0x6c, &args![saved]);
                let word = e.mem.u16(copy + 0x11c);
                e.mem.set_u16(new_motion + 0x11c, word);
            }
            e.mem.set_u32(entity_addr + 0xa4, 0);
            e.mem.set_u8(
                entity_addr + ENTITY_BYTE_2A,
                (new_type != MOTION_FIXED) as u8,
            );
        } else {
            if !old_is_dynamic {
                let saved = e.mem.u32(entity_addr + 0x1f8);
                e.vcall(motion, 0x64, &args![saved]);
                e.mem.set_u16(saved + 0xa, 0);
                e.mem.set_u16(saved + 0xc, 0);
                let byte = e.mem.u8(motion + 0x11c);
                e.mem.set_u8(entity_addr + ENTITY_BYTE_2A, byte);
                let bytes = e.mem.bytes(saved, MOTION_BYTES);
                e.mem.write(motion, &bytes);
                e.call(MEM_SET, &args![entity_addr + 0x200, 0u32, 0u32]);
                e.call(REMOVE_REFERENCE, &args![saved]);
                let shape = e.mem.u32(entity_addr + ENTITY_COLLIDABLE);
                e.call(
                    ENTITY_SET_CACHED_SHAPE_DATA,
                    &args![entity_addr, world_addr, shape],
                );
            }
            let old_type = e.mem.u8(entity_addr + ENTITY_MOTION_TYPE) as u32;
            if old_type != new_type && new_type != 1 {
                e.mem.set_u32(copy, MOTION_COPY_VTABLE);
                e.mem.set_u32(copy + 0x118, 0);
                let bytes = e.mem.bytes(motion, MOTION_BYTES);
                e.mem.write(copy, &bytes);
                copied = true;
                e.vcall(motion, 0x14, &args![copy + 0x120]);
                let second = e.mem.u8(copy + 0xbd) as u32;
                let first = e.mem.u8(copy + 0xbc) as u32;
                let second_value = x87_quiet_bits(e.mem.u32(MOTION_FLOAT_TABLE + second * 4));
                let first_value = x87_quiet_bits(e.mem.u32(MOTION_FLOAT_TABLE + first * 4));
                // 00cb43a5: `getMass` takes no stack word (plain RET); the motion, the two
                // floats and the two addresses pushed before it are the last five words of
                // the cdecl `00ca8240` (ADD ESP,0x24).
                let mass = e.call(MOTION_GET_MASS, &args![copy]).f32();
                e.call(
                    RIGID_BODY_CREATE_DYNAMIC_MOTION,
                    &args![
                        new_type,
                        copy + 0x40,
                        copy + 0x80,
                        mass.to_bits(),
                        copy + 0x120,
                        copy + 0x90,
                        first_value,
                        second_value,
                        motion
                    ],
                );
                e.call(MOTION_COPY_STATE_AND_VELOCITIES, &args![copy, motion]);
                let first_bits = x87_quiet_bits(e.mem.u32(copy + 0xb4));
                e.mem.set_u32(entity_addr + 0x194, first_bits);
                let second_bits = x87_quiet_bits(e.mem.u32(copy + 0xb8));
                e.mem.set_u32(entity_addr + 0x198, second_bits);
            }
        }
        if copied {
            e.call(KEYFRAMED_MOTION_DESTRUCTOR, &args![copy]);
        }
    });
    e.mem
        .set_u32(entity_addr + 0x18, entity_addr + ENTITY_MOTION_STATE);
    if world_addr != 0 {
        let first = e.mem.u8(world_addr + 0x305) as u32;
        let second = e.mem.u8(world_addr + 0x306) as u32;
        let third = e.mem.u8(world_addr + 0x307) as u32;
        e.call(WORLD_SET_MOTION_FLAGS, &args![motion, first, second, third]);
    }
}

// Translated from 00cb4640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::addEntitySI` (Xbox PDB), cdecl: puts the entity in the world and
/// in a simulation island. A fixed entity goes to the fixed island. Without island
/// management it goes to the first active island. Otherwise (`mode == 1`: active list, first
/// trying `findSmallSparseActiveIsland` with 4; else inactive list) a new island is built
/// and appended to the list. Ends with the entity's unique-id byte copied into the island's
/// byte +0x24.
pub fn hkp_world_operation_util_add_entity_si(e: &mut Engine, world: Ptr, entity: Ptr, mode: u32) {
    let world_addr = world.addr();
    let entity_addr = entity.addr();
    e.mem.set_u32(entity_addr + ENTITY_WORLD, world_addr);
    if e.mem.u8(entity_addr + ENTITY_MOTION_TYPE) as u32 == MOTION_FIXED {
        let fixed = e.mem.u32(world_addr + WORLD_FIXED_ISLAND);
        e.call(ISLAND_ADD_ENTITY, &args![fixed, entity_addr]);
        return;
    }
    let island;
    if e.mem.u8(world_addr + WORLD_WANT_ISLANDS) == 0 {
        let active = e.mem.u32(world_addr + WORLD_ACTIVE_ISLANDS);
        island = e.mem.u32(active);
    } else {
        let sparse = mode == 1;
        let list;
        if sparse {
            list = world_addr + WORLD_ACTIVE_ISLANDS;
            let found = e
                .call(0x00cb_3d10, &args![world_addr, list, 4u32, 0u32])
                .u32();
            if found != 0 {
                e.call(ISLAND_ADD_ENTITY, &args![found, entity_addr]);
                return;
            }
        } else {
            list = world_addr + WORLD_INACTIVE_ISLANDS;
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let block = e.vcall(allocator, 4, &args![0x6cu32]).u32();
        e.mem.set_u16(block + 4, 0x6c);
        island = e.call(ISLAND_CONSTRUCTOR, &args![block, world_addr]).u32();
        let flag = sparse as u8;
        let old = e.mem.u8(island + ISLAND_FLAGS_26);
        e.mem.set_u8(
            island + ISLAND_FLAGS_26,
            (((flag << 2) | (old & 0xf3)) & 0xfc) | flag,
        );
        let count = e.mem.u16(list + 4);
        e.mem.set_u16(island + ISLAND_INDEX, count);
        e.mem.set_u8(island + ISLAND_BYTE_24, count as u8);
        push_word(e, list, island);
    }
    e.call(ISLAND_ADD_ENTITY, &args![island, entity_addr]);
    let id = e.mem.u8(entity_addr + ENTITY_UNIQUE_ID);
    e.mem.set_u8(island + ISLAND_BYTE_24, id);
}

// Translated from 00cb4740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::removeEntitySI` (Xbox PDB), cdecl: clears the entity's world and
/// removes it from its island. An island that is in its list, is now empty and belongs to a
/// world with island management is taken out of the list (`00cb4000`) and deleted (virtual
/// destructor, slot 0, flag 1).
pub fn hkp_world_operation_util_remove_entity_si(e: &mut Engine, world: Ptr, entity: Ptr) {
    let entity_addr = entity.addr();
    let island = e.mem.u32(entity_addr + ENTITY_ISLAND);
    e.mem.set_u32(entity_addr + ENTITY_WORLD, 0);
    e.call(ISLAND_REMOVE_ENTITY, &args![island, entity_addr]);
    if e.mem.u16(island + ISLAND_INDEX) != 0xffff
        && e.mem.u32(island + ISLAND_ENTITY_COUNT) == 0
        && e.mem.u8(world.addr() + WORLD_WANT_ISLANDS) != 0
    {
        e.call(0x00cb_4000, &args![world.addr(), island]);
        e.vcall(island, 0, &args![1u32]);
    }
}

// Translated from 00cb4790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (inside `hkpWorldOperationUtil`), cdecl: for each action of the entity that belongs
/// to the world's fixed island: appends it to `out`, adds a reference, registers it with the
/// island (`00d07130`), sets the island's "action cleanup needed" bit (flags +0x25 `& 0xdf |
/// 0x10`) and puts the island in the dirty list.
pub fn fn_00cb4790(e: &mut Engine, world: Ptr, entity: Ptr, out: Ptr) {
    let world_addr = world.addr();
    let entity_addr = entity.addr();
    if e.mem.u16(entity_addr + ENTITY_ACTIONS_SIZE) == 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let actions = e.mem.u32(entity_addr + ENTITY_ACTIONS);
        let action = e.mem.u32(actions + index * 4);
        if e.mem.u32(action + 0xc) == e.mem.u32(world_addr + WORLD_FIXED_ISLAND) {
            push_word(e, out.addr(), action);
            e.call(ADD_REFERENCE, &args![action]);
            let island = e.mem.u32(world_addr + WORLD_FIXED_ISLAND);
            e.call(ISLAND_REGISTER_ACTION, &args![island, action]);
            let island = e.mem.u32(world_addr + WORLD_FIXED_ISLAND);
            let flags = e.mem.u8(island + ISLAND_FLAGS_25);
            e.mem
                .set_u8(island + ISLAND_FLAGS_25, (flags & 0xdf) | 0x10);
            let island = e.mem.u32(world_addr + WORLD_FIXED_ISLAND);
            mark_island_dirty(e, world_addr, island);
        }
        index += 1;
        if index >= e.mem.u16(entity_addr + ENTITY_ACTIONS_SIZE) as u32 {
            break;
        }
    }
}

// Translated from 00cb5000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (inside `hkpWorldOperationUtil`), cdecl: removes all the entity's constraints
/// from its world without notifying (`removeConstraintImmediately`), holding a reference to
/// each and appending it to `out`: first the master constraints from the last to the first (the
/// instance is the first word of each 0x30-byte element), then the slave constraints.
pub fn fn_00cb5000(e: &mut Engine, entity: Ptr, out: Ptr) {
    let entity_addr = entity.addr();
    let world = e.mem.u32(entity_addr + ENTITY_WORLD);
    let mut index = e.mem.u16(entity_addr + 0xb0) as i32 - 1;
    while index >= 0 {
        let data = e.mem.u32(entity_addr + 0xac);
        let constraint = e.mem.u32(data.wrapping_add(index as u32 * 0x30));
        e.call(ADD_REFERENCE, &args![constraint]);
        e.call(0x00cb_3dd0, &args![world, constraint, 0u32]);
        push_word(e, out.addr(), constraint);
        index -= 1;
    }
    let mut index = e.mem.i32(entity_addr + 0xb8) - 1;
    while index >= 0 {
        let data = e.mem.u32(entity_addr + 0xb4);
        let constraint = e.mem.u32(data.wrapping_add(index as u32 * 4));
        e.call(ADD_REFERENCE, &args![constraint]);
        e.call(0x00cb_3dd0, &args![world, constraint, 0u32]);
        push_word(e, out.addr(), constraint);
        index -= 1;
    }
}

// Translated from 00cb5500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (inside `hkpWorldOperationUtil`), cdecl: marks an island for the next cleanup.
/// An island that is in the active list but not yet marked (flag byte +0x26 bits 2-3 clear,
/// bits 0-1 set) gets the "try to increase" bits (`& 0xdf | 0x10`); then it is marked
/// (`& 0xf7 | 4`) and put in the world's dirty list.
pub fn fn_00cb5500(e: &mut Engine, world: Ptr, island: Ptr) {
    let island_addr = island.addr();
    let flags = e.mem.u8(island_addr + ISLAND_FLAGS_26);
    if flags & 0xc == 0 && flags & 3 != 0 {
        e.mem
            .set_u8(island_addr + ISLAND_FLAGS_26, (flags & 0xdf) | 0x10);
    }
    let flags = e.mem.u8(island_addr + ISLAND_FLAGS_26);
    e.mem
        .set_u8(island_addr + ISLAND_FLAGS_26, (flags & 0xf7) | 4);
    mark_island_dirty(e, world.addr(), island_addr);
}

// Translated from 00cb5570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::mergeIslands` (Xbox PDB), cdecl: merges the islands of two
/// entities. While the critical-operations lock is held, a 12-byte operation record (byte
/// 0xc, then the two entities) is appended to the world's pending queue instead.
pub fn hkp_world_operation_util_merge_islands(e: &mut Engine, world: Ptr, first: Ptr, second: Ptr) {
    let world_addr = world.addr();
    if e.mem.u32(world_addr + WORLD_LOCK_COUNT) != 0 {
        // The exe's frame (`SUB ESP,0xc`, two pushes): 0x14 bytes, the record at +0x8 (00cb559a).
        e.with_stack(0x14, |e, frame| {
            let record = frame.addr() + 8;
            e.mem.set_u8(record, 0xc);
            e.mem.set_u32(record + 4, first.addr());
            e.mem.set_u32(record + 8, second.addr());
            e.call(WORLD_ENQUEUE_OPERATION, &args![world_addr, record]);
        });
    } else {
        let first_island = e.mem.u32(first.addr() + ENTITY_ISLAND);
        let second_island = e.mem.u32(second.addr() + ENTITY_ISLAND);
        e.call(
            INTERNAL_MERGE_TWO_ISLANDS,
            &args![world_addr, first_island, second_island],
        );
    }
}

// Translated from 00cb5750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::updateEntityBP` (Xbox PDB), cdecl: re-queries the broad phase for
/// the entity's new AABB. While the critical lock is held a record (byte 3, then the entity)
/// is queued instead. Otherwise, for an entity with a shape: recalculates its AABB when
/// the new minimum (+0x30) is above the old (+0x40, unsigned), builds the integer box (the
/// byte extents at +0x3c..+0x3e and +0x4c..+0x4e shifted by the byte at +0x3f), asks the broad
/// phase (vtable slot `0x2c`) for added and removed pairs, removes the duplicates, and
/// removes and adds the pairs on the dispatcher.
pub fn hkp_world_operation_util_update_entity_bp(e: &mut Engine, world: Ptr, entity: Ptr) {
    let world_addr = world.addr();
    let entity_addr = entity.addr();
    // The exe's frame (`AND ESP,0xfffffff0`, `SUB ESP,0x54`, three pushes): 0x60 bytes from a
    // 16-byte boundary. The queued record at +0x18 (00cb576c); the handle slot at +0x18
    // (00cb5849), the removed pairs at +0x20 (00cb57e8), the added pairs at +0x30 (00cb57a9) and
    // the integer box at +0x40 (min x, y, z, spare, max x, y, z; 00cb5855..00cb5897). The entity
    // goes to the AABB update through its own argument slot (`LEA ECX,[EBP+0xc]`, 00cb582c);
    // `EBP` is taken on the boundary, so the slot is +0x6c.
    with_aligned_stack(e, 0x70, |e, frame| {
        update_entity_bp_in_frame(e, world_addr, entity_addr, frame)
    });
}

/// [`hkp_world_operation_util_update_entity_bp`] in its frame.
fn update_entity_bp_in_frame(e: &mut Engine, world_addr: u32, entity_addr: u32, frame: u32) {
    e.mem.set_u32(frame + 0x68, world_addr);
    e.mem.set_u32(frame + 0x6c, entity_addr);
    if e.mem.u32(world_addr + WORLD_LOCK_COUNT) != 0 {
        let record = frame + 0x18;
        e.mem.set_u8(record, 3);
        e.mem.set_u32(record + 4, entity_addr);
        e.call(WORLD_ENQUEUE_OPERATION, &args![world_addr, record]);
        return;
    }
    e.mem.set_u32(world_addr + WORLD_LOCK_COUNT, 1);
    let collidable = entity_addr + ENTITY_COLLIDABLE;
    if e.mem.u32(collidable) != 0 {
        let added = frame + 0x30;
        let removed = frame + 0x20;
        let handle_slot = frame + 0x18;
        let entity_slot = frame + 0x6c;
        let aabb = frame + 0x40;
        scratch_array_create(e, added, world_addr);
        scratch_array_create(e, removed, world_addr);
        if e.mem.u32(entity_addr + 0x30) > e.mem.u32(entity_addr + 0x40) {
            let input = e.mem.u32(world_addr + WORLD_COLLISION_INPUT);
            e.call(ENTITY_BATCH_RECALC_AABB, &args![input, entity_slot, 1u32]);
        }
        let shift = e.mem.u8(entity_addr + 0x3f) as u32;
        for lane in 0..3u32 {
            let low = e.mem.u32(entity_addr + 0x30 + lane * 4);
            let low_extent = (e.mem.u8(entity_addr + 0x3c + lane) as u32).wrapping_shl(shift);
            e.mem.set_u32(aabb + lane * 4, low.wrapping_sub(low_extent));
            let high = e.mem.u32(entity_addr + 0x40 + lane * 4);
            let high_extent = (e.mem.u8(entity_addr + 0x4c + lane) as u32).wrapping_shl(shift);
            e.mem
                .set_u32(aabb + 0x10 + lane * 4, high_extent.wrapping_add(high));
        }
        e.mem.set_u32(handle_slot, collidable + 0x14);
        let broad_phase = e.mem.u32(world_addr + WORLD_BROAD_PHASE);
        e.vcall(
            broad_phase,
            0x2c,
            &args![handle_slot, aabb, 1u32, added, removed],
        );
        let total = e.mem.i32(removed + 4).wrapping_add(e.mem.i32(added + 4));
        if total > 0 {
            e.call(DISPATCHER_REMOVE_DUPLICATES, &args![added, removed]);
            let dispatcher = e.mem.u32(world_addr + WORLD_DISPATCHER);
            let removed_data = e.mem.u32(removed);
            let removed_size = e.mem.u32(removed + 4);
            e.call(
                DISPATCHER_REMOVE_PAIRS,
                &args![dispatcher, removed_data, removed_size],
            );
            let filter = collision_filter_interface(e, world_addr);
            let added_data = e.mem.u32(added);
            let added_size = e.mem.u32(added + 4);
            e.call(
                DISPATCHER_ADD_PAIRS,
                &args![dispatcher, added_data, added_size, filter],
            );
        }
        scratch_array_destroy(e, removed);
        scratch_array_destroy(e, added);
    }
    release_critical_lock(e, world_addr);
}

// Translated from 00cb59f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::addEntityBP` (Xbox PDB), cdecl: for an entity with a shape,
/// recalculates its AABB, asks the broad phase (vtable slot `0x14`) for the new pairs with a
/// copy of the AABB (32 bytes from +0x30) and adds them on the dispatcher.
pub fn hkp_world_operation_util_add_entity_bp(e: &mut Engine, world: Ptr, entity: Ptr) {
    let world_addr = world.addr();
    let entity_addr = entity.addr();
    let collidable = entity_addr + ENTITY_COLLIDABLE;
    if e.mem.u32(collidable) == 0 {
        return;
    }
    // The exe's frame (`AND ESP,0xfffffff0`, `SUB ESP,0x34`, three pushes): 0x40 bytes from a
    // 16-byte boundary, the pair array at +0x10 (00cb5a19), the AABB copy at +0x20 (00cb5a6a).
    // The entity goes to the AABB update through its own argument slot (`LEA EAX,[EBP+0xc]`,
    // 00cb5a40); `EBP` is taken on the boundary, so the slot is +0x4c.
    with_aligned_stack(e, 0x50, |e, frame| {
        let array = frame + 0x10;
        let entity_slot = frame + 0x4c;
        e.mem.set_u32(frame + 0x48, world_addr);
        e.mem.set_u32(entity_slot, entity_addr);
        scratch_array_create(e, array, world_addr);
        let input = e.mem.u32(world_addr + WORLD_COLLISION_INPUT);
        e.call(ENTITY_BATCH_RECALC_AABB, &args![input, entity_slot, 1u32]);
        let aabb = e.mem.bytes(entity_addr + 0x30, 32);
        e.mem.write(frame + 0x20, &aabb);
        let broad_phase = e.mem.u32(world_addr + WORLD_BROAD_PHASE);
        e.vcall(
            broad_phase,
            0x14,
            &args![collidable + 0x14, frame + 0x20, array],
        );
        let count = e.mem.i32(array + 4);
        if count > 0 {
            let filter = collision_filter_interface(e, world_addr);
            let dispatcher = e.mem.u32(world_addr + WORLD_DISPATCHER);
            let data = e.mem.u32(array);
            e.call(
                DISPATCHER_ADD_PAIRS,
                &args![dispatcher, data, count as u32, filter],
            );
        }
        scratch_array_destroy(e, array);
    });
}

// Translated from 00cb5b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::addPhantomBP` (Xbox PDB), cdecl: fills an AABB buffer from the
/// phantom (vtable slot `0x18`), asks the broad phase (slot `0x10`) for the new pairs, stores
/// the AABB in the phantom (`hkpPhantom::setBoundingVolumeData`) and adds the pairs on the
/// dispatcher.
pub fn hkp_world_operation_util_add_phantom_bp(e: &mut Engine, world: Ptr, phantom: Ptr) {
    let world_addr = world.addr();
    let phantom_addr = phantom.addr();
    // The exe's frame (`AND ESP,0xfffffff0`, `SUB ESP,0x34`, three pushes): 0x40 bytes from a
    // 16-byte boundary, the pair array at +0x10 (00cb5b27), the AABB buffer at +0x20 (00cb5b66).
    with_aligned_stack(e, 0x40, |e, frame| {
        let array = frame + 0x10;
        let aabb = frame + 0x20;
        scratch_array_create(e, array, world_addr);
        e.vcall(phantom_addr, 0x18, &args![aabb]);
        let broad_phase = e.mem.u32(world_addr + WORLD_BROAD_PHASE);
        e.vcall(broad_phase, 0x10, &args![phantom_addr + 0x24, aabb, array]);
        e.call(PHANTOM_SET_BOUNDING_VOLUME_DATA, &args![phantom_addr, aabb]);
        let count = e.mem.u32(array + 4);
        if count != 0 {
            let filter = collision_filter_interface(e, world_addr);
            let dispatcher = e.mem.u32(world_addr + WORLD_DISPATCHER);
            let data = e.mem.u32(array);
            e.call(
                DISPATCHER_ADD_PAIRS,
                &args![dispatcher, data, count, filter],
            );
        }
        scratch_array_destroy(e, array);
    });
}

// Translated from 00cb5c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::removeEntityBP` (Xbox PDB), cdecl: for an entity with a shape:
/// resets its collision information through the simulation (vtable slot `0x20`), asks the
/// broad phase (slot `0x1c`) for the pairs to remove and removes them on the dispatcher.
pub fn hkp_world_operation_util_remove_entity_bp(e: &mut Engine, world: Ptr, entity: Ptr) {
    let world_addr = world.addr();
    let entity_addr = entity.addr();
    let collidable = entity_addr + ENTITY_COLLIDABLE;
    if e.mem.u32(collidable) == 0 {
        return;
    }
    // The exe's frame (`SUB ESP,0x10`, four pushes): 0x20 bytes, the pair array at +0x10
    // (00cb5c46). The entity goes to the simulation through its own argument slot, +0x28 past
    // the return address at +0x20 (`LEA EDX,[ESP+0x34]` after three pushes, 00cb5c39).
    e.with_stack(0x2c, |e, frame| {
        let frame = frame.addr();
        let array = frame + 0x10;
        let entity_slot = frame + 0x28;
        e.mem.set_u32(frame + 0x24, world_addr);
        e.mem.set_u32(entity_slot, entity_addr);
        let simulation = e.mem.u32(world_addr + WORLD_SIMULATION);
        e.vcall(
            simulation,
            0x20,
            &args![entity_slot, 1u32, world_addr, 1u32],
        );
        scratch_array_create(e, array, world_addr);
        let broad_phase = e.mem.u32(world_addr + WORLD_BROAD_PHASE);
        e.vcall(broad_phase, 0x1c, &args![collidable + 0x14, array]);
        let count = e.mem.i32(array + 4);
        if count > 0 {
            let dispatcher = e.mem.u32(world_addr + WORLD_DISPATCHER);
            let data = e.mem.u32(array);
            e.call(
                DISPATCHER_REMOVE_PAIRS,
                &args![dispatcher, data, count as u32],
            );
        }
        scratch_array_destroy(e, array);
    });
}

// Translated from 00cb5d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::removePhantomBP` (Xbox PDB), cdecl: asks the broad phase (vtable
/// slot `0x1c`) for the phantom's pairs and removes them on the dispatcher.
pub fn hkp_world_operation_util_remove_phantom_bp(e: &mut Engine, world: Ptr, phantom: Ptr) {
    let world_addr = world.addr();
    // The exe's frame (`SUB ESP,0x10`, three pushes): 0x1c bytes, the pair array at +0xc
    // (00cb5d12).
    e.with_stack(0x1c, |e, frame| {
        let array = frame.addr() + 0xc;
        scratch_array_create(e, array, world_addr);
        let broad_phase = e.mem.u32(world_addr + WORLD_BROAD_PHASE);
        e.vcall(broad_phase, 0x1c, &args![phantom.addr() + 0x24, array]);
        let count = e.mem.u32(array + 4);
        if count != 0 {
            let dispatcher = e.mem.u32(world_addr + WORLD_DISPATCHER);
            let data = e.mem.u32(array);
            e.call(DISPATCHER_REMOVE_PAIRS, &args![dispatcher, data, count]);
        }
        scratch_array_destroy(e, array);
    });
}

// Translated from 00cb5dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (inside `hkpWorldOperationUtil`), cdecl: moves the entity's actions that involve
/// only fixed entities and this entity (each action lists its entities through vtable slot
/// `0x10` into an inline array that is not cleared between actions) to the entity's island:
/// appended to `out`, referenced, registered with the island (`00d07130`), the island's
/// "action cleanup needed" bit set and the island put in the dirty list.
pub fn fn_00cb5dd0(e: &mut Engine, world: Ptr, entity: Ptr, out: Ptr) {
    let world_addr = world.addr();
    let entity_addr = entity.addr();
    let island = e.mem.u32(entity_addr + ENTITY_ISLAND);
    // The inline array (data, size, capacity-and-flags, 16 words of storage). The exe's frame
    // (`SUB ESP,0x50`, four pushes): 0x60 bytes, the array at +0x14 with its storage at +0x20
    // (00cb5de8).
    e.with_stack(0x60, |e, frame| {
        let list = frame.addr() + 0x14;
        inplace_array_init(e, list);
        if e.mem.u16(entity_addr + ENTITY_ACTIONS_SIZE) != 0 {
            let mut index = 0u32;
            loop {
                let actions = e.mem.u32(entity_addr + ENTITY_ACTIONS);
                let action = e.mem.u32(actions + index * 4);
                e.vcall(action, 0x10, &args![list]);
                let count = e.mem.i32(list + 4);
                let data = e.mem.u32(list);
                let mut only_fixed_or_this = true;
                let mut position = 0i32;
                while position < count {
                    let other = e.mem.u32(data.wrapping_add(position as u32 * 4));
                    if e.mem.u8(other + ENTITY_MOTION_TYPE) as u32 != MOTION_FIXED
                        && other != entity_addr
                    {
                        only_fixed_or_this = false;
                        break;
                    }
                    position += 1;
                }
                if only_fixed_or_this {
                    push_word(e, out.addr(), action);
                    e.call(ADD_REFERENCE, &args![action]);
                    e.call(ISLAND_REGISTER_ACTION, &args![island, action]);
                    let flags = e.mem.u8(island + ISLAND_FLAGS_25);
                    e.mem
                        .set_u8(island + ISLAND_FLAGS_25, (flags & 0xdf) | 0x10);
                    mark_island_dirty(e, world_addr, island);
                }
                index += 1;
                if index >= e.mem.u16(entity_addr + ENTITY_ACTIONS_SIZE) as u32 {
                    break;
                }
            }
        }
        array_free_storage(e, list, 4);
    });
}

// Translated from 00cb5f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::addActionsToEntitysIsland` (Xbox PDB), cdecl: adds each action of
/// the array to the entity's island (`hkpSimulationIsland::addAction`) and drops the
/// reference the array held. Unless the island is the world's fixed island, each action also
/// lists its entities (vtable slot `0x10`) and every other non-fixed entity in a different island
/// than this entity's is merged with it (`mergeIslands`).
pub fn hkp_world_operation_util_add_actions_to_entitys_island(
    e: &mut Engine,
    world: Ptr,
    entity: Ptr,
    actions: Ptr,
) {
    let world_addr = world.addr();
    let entity_addr = entity.addr();
    let island = e.mem.u32(entity_addr + ENTITY_ISLAND);
    // The exe's frame (`SUB ESP,0x4c`, four pushes): 0x5c bytes, the action's entity array at
    // +0x10 with its storage at +0x1c (00cb5f6d).
    e.with_stack(0x5c, |e, frame| {
        let list = frame.addr() + 0x10;
        inplace_array_init(e, list);
        if e.mem.i32(actions.addr() + 4) > 0 {
            let mut index = 0u32;
            loop {
                let data = e.mem.u32(actions.addr());
                let action = e.mem.u32(data + index * 4);
                let fixed_island = e.mem.u32(world_addr + WORLD_FIXED_ISLAND);
                e.call(ISLAND_ADD_ACTION, &args![island, action]);
                e.call(REMOVE_REFERENCE, &args![action]);
                if island != fixed_island {
                    e.mem.set_u32(list + 4, 0);
                    e.vcall(action, 0x10, &args![list]);
                    let mut position = 0i32;
                    while position < e.mem.i32(list + 4) {
                        let other = e.mem.u32(e.mem.u32(list).wrapping_add(position as u32 * 4));
                        if other != entity_addr
                            && e.mem.u8(other + ENTITY_MOTION_TYPE) as u32 != MOTION_FIXED
                            && e.mem.u8(entity_addr + ENTITY_MOTION_TYPE) as u32 != MOTION_FIXED
                            && e.mem.u32(other + ENTITY_ISLAND)
                                != e.mem.u32(entity_addr + ENTITY_ISLAND)
                        {
                            let other_world = e.mem.u32(other + ENTITY_WORLD);
                            e.call(0x00cb_5570, &args![other_world, other, entity_addr]);
                        }
                        position += 1;
                    }
                }
                index += 1;
                if index as i32 >= e.mem.i32(actions.addr() + 4) {
                    break;
                }
            }
        }
        array_free_storage(e, list, 4);
    });
}

// Translated from 00cb6f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorldOperationUtil::setRigidBodyMotionType` (Xbox PDB), cdecl: `mode` goes to
/// `addEntitySI` (1 selects the sparse active list; its exact meaning is unconfirmed) and
/// `filter_update` to `hkpWorld::updateCollisionFilterOnEntity`.
///
/// Nothing happens when the type is unchanged, or when a fixed or keyframed entity is
/// switched to a dynamic type and has no saved dynamic motion (+0x1f8).
/// Leaving a fixed motion first sorts the collidable's entries and the entity's slave
/// constraints. The entity is held by a reference during the change. Without a world, or when
/// both or neither of the old and new type are fixed, only the motion object is replaced
/// (`00cb4230`) and the collision filter updated. Otherwise the entity is taken out of
/// the simulation: constraints removed (`00cb5000`), actions collected (`00cb5dd0` or
/// `00cb4790`), the agents to fixed partners moved (`00cb3ee0`), the island flagged
/// when it has more than 2 entities, the entity removed from its island (`00cb4740`), the
/// motion replaced, the entity re-added (`00cb4640`), actions re-added (`00cb5f50`),
/// constraints re-added (`00cb3ca0`, no notification), islands merged with the partners of
/// the sorted collision entries (unless fixed), the agent track appended to the island's, and
/// for a fixed entity the motion state frozen and the entity collided in the broad phase
/// (simulation slot `0x20`, then `collideEntitiesBroadPhaseDiscrete` and
/// `fireInactiveEntityMoved`). The pending operations then run if nothing holds the lock.
pub fn hkp_world_operation_util_set_rigid_body_motion_type(
    e: &mut Engine,
    entity: Ptr,
    new_type: u32,
    mode: u32,
    filter_update: u32,
) {
    let entity_addr = entity.addr();
    let old_type = e.mem.u8(entity_addr + ENTITY_MOTION_TYPE) as u32;
    if old_type == new_type {
        return;
    }
    if old_type == MOTION_FIXED {
        e.call(
            LINKED_COLLIDABLE_SORT_ENTRIES,
            &args![entity_addr + ENTITY_COLLIDABLE],
        );
        e.call(ENTITY_SORT_CONSTRAINTS_SLAVES, &args![entity_addr]);
    }
    let new_is_dynamic = !(new_type == MOTION_FIXED || new_type == MOTION_KEYFRAMED);
    let old_is_dynamic = !(old_type == MOTION_FIXED || old_type == MOTION_KEYFRAMED);
    if new_is_dynamic && !old_is_dynamic && e.mem.u32(entity_addr + 0x1f8) == 0 {
        return;
    }
    e.call(ADD_REFERENCE, &args![entity_addr]);
    let world = e.mem.u32(entity_addr + ENTITY_WORLD);
    if world == 0 || (old_type == MOTION_FIXED) == (new_type == MOTION_FIXED) {
        e.call(
            0x00cb_4230,
            &args![
                entity_addr,
                new_type,
                new_is_dynamic as u32,
                old_is_dynamic as u32,
                world
            ],
        );
        if world != 0 {
            e.call(
                WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY,
                &args![world, entity_addr, filter_update, 1u32],
            );
        }
    } else {
        // Frame: the constraints array 0 (inline, 16 words from 0xc), the actions array 0x50
        // (inline, from 0x5c), the agent track 0xa0 (0x200, then an inline array of one word
        // from 0xa4), the collision entries array 0xc0, the entity slot 0xd0.
        let frame = e.mem.alloc(0x100);
        let constraints = frame;
        let actions = frame + 0x50;
        let track = frame + 0xa0;
        let entries = frame + 0xc0;
        inplace_array_init(e, constraints);
        inplace_array_init(e, actions);
        e.mem.set_u32(track, 0x200);
        e.mem.set_u32(track + 4, track + 0x10);
        e.mem.set_u32(track + 8, 0);
        e.mem.set_u32(track + 0xc, 0x8000_0001);
        e.mem.set_u8(world + WORLD_BLOCK_PENDING, 1);
        e.mem.set_u8(world + WORLD_CRITICAL_ALLOWED, 0);
        e.call(0x00cb_5000, &args![entity_addr, constraints]);
        if new_type == MOTION_FIXED {
            e.call(0x00cb_5dd0, &args![world, entity_addr, actions]);
        } else {
            e.call(0x00cb_4790, &args![world, entity_addr, actions]);
        }
        let island = e.mem.u32(entity_addr + ENTITY_ISLAND);
        e.call(
            0x00cb_3ee0,
            &args![island + ISLAND_AGENT_TRACK, entity_addr, track, new_type],
        );
        if old_type != MOTION_FIXED {
            let island = e.mem.u32(entity_addr + ENTITY_ISLAND);
            if e.mem.i32(island + ISLAND_ENTITY_COUNT) > 2 {
                let flags = e.mem.u8(island + ISLAND_FLAGS_25);
                e.mem.set_u8(island + ISLAND_FLAGS_25, (flags & 0xfd) | 1);
            }
        }
        e.call(0x00cb_4740, &args![world, entity_addr]);
        e.call(
            0x00cb_4230,
            &args![
                entity_addr,
                new_type,
                new_is_dynamic as u32,
                old_is_dynamic as u32,
                world
            ],
        );
        e.call(0x00cb_4640, &args![world, entity_addr, mode]);
        e.call(0x00cb_5f50, &args![world, entity_addr, actions]);
        e.mem.set_u8(world + WORLD_CRITICAL_ALLOWED, 1);
        e.mem.set_u32(entries, 0);
        e.mem.set_u32(entries + 4, 0);
        e.mem.set_u32(entries + 8, 0x8000_0000);
        let mut index = 0i32;
        while index < e.mem.i32(constraints + 4) {
            let data = e.mem.u32(constraints);
            let constraint = e.mem.u32(data.wrapping_add(index as u32 * 4));
            e.call(0x00cb_3ca0, &args![world, constraint, 0u32]);
            e.call(REMOVE_REFERENCE, &args![constraint]);
            index += 1;
        }
        if new_type != MOTION_FIXED {
            e.call(
                LINKED_COLLIDABLE_GET_ENTRIES_SORTED,
                &args![entity_addr + ENTITY_COLLIDABLE, entries],
            );
            let mut index = 0i32;
            while index < e.mem.i32(entries + 4) {
                let data = e.mem.u32(entries);
                let partner = e.mem.u32(data.wrapping_add(index as u32 * 8 + 4));
                let owner = partner.wrapping_add(e.mem.i8(partner + 0x10) as i32 as u32);
                if e.mem.u8(entity_addr + ENTITY_MOTION_TYPE) as u32 != MOTION_FIXED
                    && e.mem.u8(owner + ENTITY_MOTION_TYPE) as u32 != MOTION_FIXED
                    && e.mem.u32(entity_addr + ENTITY_ISLAND) != e.mem.u32(owner + ENTITY_ISLAND)
                {
                    let entity_world = e.mem.u32(entity_addr + ENTITY_WORLD);
                    e.call(0x00cb_5570, &args![entity_world, entity_addr, owner]);
                }
                index += 1;
            }
        }
        let island = e.mem.u32(entity_addr + ENTITY_ISLAND);
        e.call(
            AGENT_APPEND_TRACK,
            &args![island + ISLAND_AGENT_TRACK, track],
        );
        if new_type == MOTION_FIXED {
            let step_time = x87_quiet_bits(e.mem.u32(world + WORLD_STEP_TIME));
            e.call(
                FREEZE_MOTION_STATE,
                &args![step_time, entity_addr + ENTITY_MOTION_STATE],
            );
            add_word(e, world + WORLD_LOCK_COUNT, 1);
            e.mem.set_u32(frame + 0xd0, entity_addr);
            let simulation = e.mem.u32(world + WORLD_SIMULATION);
            e.vcall(simulation, 0x20, &args![frame + 0xd0, 1u32, world, 7u32]);
            e.call(
                SIMULATION_COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE,
                &args![frame + 0xd0, 1u32, world],
            );
            let entity_world = e.mem.u32(entity_addr + ENTITY_WORLD);
            e.call(
                FIRE_INACTIVE_ENTITY_MOVED,
                &args![entity_world, entity_addr],
            );
            add_word(e, world + WORLD_LOCK_COUNT, u32::MAX);
        }
        e.call(
            WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY,
            &args![world, entity_addr, filter_update, 1u32],
        );
        e.mem.set_u8(world + WORLD_BLOCK_PENDING, 0);
        if e.mem.u32(world + WORLD_LOCK_COUNT) == 0 {
            run_pending_operations(e, world);
        }
        array_free_storage(e, entries, 8);
        array_free_storage(e, track + 4, 4);
        array_free_storage(e, actions, 4);
        array_free_storage(e, constraints, 4);
        e.mem.free(frame);
    }
    e.call(REMOVE_REFERENCE, &args![entity_addr]);
}

// Translated from 00cb7460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (inside `hkpWorldOperationUtil`), `RET 4`: answers the word at +0 when `key + 1`
/// equals the word at +8, otherwise 0x200.
pub fn fn_00cb7460(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    if key.wrapping_add(1) == e.mem.u32(this.addr() + 8) {
        e.mem.u32(this.addr())
    } else {
        0x200
    }
}

// Translated from 00cb74d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstraintOwner::addConstraintInfo` (Xbox PDB), `RET 8`: for a constraint with an
/// internal block (+0x30) folds the four-word `info` into the owner (`this`): the maximum
/// at +8 of the first and second word, sums of the second, third and fourth word at +0xc,
/// +0x10 and +0x14; adds them to the internal block's 16-bit counters (+0x14, +0x16, +0x18);
/// and finally raises +8 to the block's +0x14 counter.
pub fn hkp_constraint_owner_add_constraint_info(
    e: &mut Engine,
    this: Ptr,
    constraint: Ptr,
    info: Ptr,
) {
    let internal = e.mem.u32(constraint.addr() + 0x30);
    if internal == 0 {
        return;
    }
    let owner = this.addr();
    let info = info.addr();
    let first = e.mem.i32(info);
    let current = e.mem.i32(owner + 8);
    let mut maximum = if current > first { current } else { first };
    e.mem.set_i32(owner + 8, maximum);
    let second = e.mem.i32(info + 4);
    if maximum <= second {
        maximum = second;
    }
    e.mem.set_i32(owner + 8, maximum);
    let (second_word, third_word, fourth_word) = (
        e.mem.u32(info + 4),
        e.mem.u32(info + 8),
        e.mem.u32(info + 0xc),
    );
    add_word(e, owner + 0xc, second_word);
    add_word(e, owner + 0x10, third_word);
    add_word(e, owner + 0x14, fourth_word);
    let fourth_sum = (e.mem.u16(internal + 0x18) as u32).wrapping_add(e.mem.u32(info + 0xc));
    let second_sum = (e.mem.u16(internal + 0x14) as u32).wrapping_add(e.mem.u32(info + 4));
    let third = e.mem.u16(info + 8);
    let counter = e.mem.u16(internal + 0x16).wrapping_add(third);
    e.mem.set_u16(internal + 0x16, counter);
    e.mem.set_u16(internal + 0x18, fourth_sum as u16);
    e.mem.set_u16(internal + 0x14, second_sum as u16);
    let counter = e.mem.u16(internal + 0x14) as i32;
    let current = e.mem.i32(owner + 8);
    e.mem
        .set_i32(owner + 8, if current > counter { current } else { counter });
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALLOCATOR_VTABLE: u32 = 0x0200_0000;
    const ALLOCATE: u32 = 0x0200_0100;
    const FREE: u32 = 0x0200_0110;
    const OBJECT_VTABLE: u32 = 0x0200_0200;
    const OBJECT_SLOT_10: u32 = 0x0200_0300;
    const OBJECT_SLOT_14: u32 = 0x0200_0304;
    const OBJECT_SLOT_18: u32 = 0x0200_0308;
    const OBJECT_SLOT_1C: u32 = 0x0200_030c;
    const OBJECT_SLOT_20: u32 = 0x0200_0310;
    const OBJECT_SLOT_2C: u32 = 0x0200_0314;
    const OBJECT_SLOT_64: u32 = 0x0200_0318;
    const OBJECT_SLOT_6C: u32 = 0x0200_031c;

    /// An engine with the memory router, an allocator and the array-grow function doubled.
    /// Returns the engine and the router object.
    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(Vec::new());
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(0x10);
        let stack = e.mem.alloc(0x1000);
        e.mem.set_u32(router, stack);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        e.put_vtable(ALLOCATOR_VTABLE, &[0, ALLOCATE, 0, 0, FREE]);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(FREE, |_, _| 0u32.into_ret());
        e.register_double(MEMORY_ROUTER, move |_, _| router.into_ret());
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            let (array, element) = (a[1], a[2]);
            let size = e.mem.u32(array + 4);
            let capacity = ((e.mem.u32(array + 8) & 0x3fff_ffff) * 2).max(4);
            let fresh = e.mem.alloc(capacity * element);
            let old = e.mem.bytes(e.mem.u32(array), size * element);
            e.mem.write(fresh, &old);
            e.mem.set_u32(array, fresh);
            e.mem.set_u32(array + 8, capacity);
            0u32.into_ret()
        });
        // A generic object whose slots are all recorded.
        let mut words = vec![0u32; 28];
        words[0x10 / 4] = OBJECT_SLOT_10;
        words[0x14 / 4] = OBJECT_SLOT_14;
        words[0x18 / 4] = OBJECT_SLOT_18;
        words[0x1c / 4] = OBJECT_SLOT_1C;
        words[0x20 / 4] = OBJECT_SLOT_20;
        words[0x2c / 4] = OBJECT_SLOT_2C;
        words[0x64 / 4] = OBJECT_SLOT_64;
        words[0x6c / 4] = OBJECT_SLOT_6C;
        e.put_vtable(OBJECT_VTABLE, &words);
        for slot in [
            OBJECT_SLOT_10,
            OBJECT_SLOT_14,
            OBJECT_SLOT_18,
            OBJECT_SLOT_1C,
            OBJECT_SLOT_20,
            OBJECT_SLOT_2C,
            OBJECT_SLOT_64,
            OBJECT_SLOT_6C,
        ] {
            e.register(slot, |_, _| 0u32.into_ret());
        }
        (e, router)
    }

    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| 0u32.into_ret());
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Order of the logged calls whose address is in `wanted`.
    fn order(e: &Engine, wanted: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| wanted.contains(a))
            .map(|(a, _)| *a)
            .collect()
    }

    /// A zeroed block holding an object whose vtable is [`OBJECT_VTABLE`].
    fn object(e: &mut Engine, size: u32) -> u32 {
        let block = e.mem.alloc(size);
        e.mem.set_u32(block, OBJECT_VTABLE);
        block
    }

    /// An `hkArray` header with `values` as its word elements.
    fn word_array(e: &mut Engine, header: u32, values: &[u32]) {
        let data = e.mem.alloc(values.len() as u32 * 4 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(data + i as u32 * 4, *v);
        }
        e.mem.set_u32(header, data);
        e.mem.set_u32(header + 4, values.len() as u32);
        e.mem.set_u32(header + 8, values.len() as u32 | 0x8000_0000);
    }

    fn array_words(e: &Engine, header: u32) -> Vec<u32> {
        let data = e.mem.u32(header);
        (0..e.mem.u32(header + 4))
            .map(|i| e.mem.u32(data + i * 4))
            .collect()
    }

    const PENDING: [u32; 2] = [
        WORLD_RUN_PENDING_OPERATIONS,
        WORLD_RUN_PENDING_BODY_OPERATIONS,
    ];

    #[test]
    fn add_constraint_runs_pending_operations_after_the_lock() {
        let (mut e, _) = rig();
        stub(
            &mut e,
            &[
                WORLD_CONSTRAINT_UTIL_ADD,
                FIRE_CONSTRAINT_ADDED,
                WORLD_RUN_PENDING_OPERATIONS,
                WORLD_RUN_PENDING_BODY_OPERATIONS,
            ],
        );
        let world = e.mem.alloc(0x350);
        e.mem.set_u32(world + WORLD_PENDING_OPERATIONS, 3);
        e.mem.set_u32(world + WORLD_PENDING_QUEUE_COUNT, 1);
        e.mem.set_u32(world + WORLD_PENDING_BODY_OPERATIONS, 2);
        let returned = e.call(0x00cb_3ca0, &args![world, 0x5000u32, 1u32]).u32();
        assert_eq!(returned, 0x5000);
        assert_eq!(
            calls(&e, WORLD_CONSTRAINT_UTIL_ADD),
            vec![vec![world, 0x5000]]
        );
        assert_eq!(calls(&e, FIRE_CONSTRAINT_ADDED), vec![vec![world, 0x5000]]);
        assert_eq!(
            order(
                &e,
                &[
                    WORLD_CONSTRAINT_UTIL_ADD,
                    FIRE_CONSTRAINT_ADDED,
                    PENDING[0],
                    PENDING[1]
                ]
            ),
            vec![
                WORLD_CONSTRAINT_UTIL_ADD,
                FIRE_CONSTRAINT_ADDED,
                PENDING[0],
                PENDING[1]
            ]
        );
        assert_eq!(e.mem.u32(world + WORLD_LOCK_COUNT), 0);

        // Already locked by the caller: no notification, nothing run, the lock stays.
        e.call_log = Some(Vec::new());
        e.mem.set_u32(world + WORLD_LOCK_COUNT, 2);
        e.call(0x00cb_3ca0, &args![world, 0x5000u32, 0u32]);
        assert!(calls(&e, FIRE_CONSTRAINT_ADDED).is_empty());
        assert!(calls(&e, PENDING[0]).is_empty());
        assert_eq!(e.mem.u32(world + WORLD_LOCK_COUNT), 2);

        // Blocked pending operations are not run.
        e.mem.set_u32(world + WORLD_LOCK_COUNT, 0);
        e.mem.set_u8(world + WORLD_BLOCK_PENDING, 1);
        e.call(0x00cb_3ca0, &args![world, 0x5000u32, 0u32]);
        assert!(calls(&e, PENDING[0]).is_empty());
    }

    #[test]
    fn find_small_sparse_island_picks_a_marked_small_island() {
        let (mut e, _) = rig();
        let world = e.mem.alloc(0x350);
        e.mem.set_u32(world + WORLD_MIN_ISLAND_SIZE, 10);
        e.mem.set_u8(world + WORLD_WANT_ISLANDS, 1);
        let islands: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x6c)).collect();
        // Island 2 (last) is the skipped one, island 1 is too big, island 0 qualifies.
        e.mem.set_u32(islands[0] + ISLAND_ENTITY_COUNT, 2);
        e.mem.set_u32(islands[0] + ISLAND_CONSTRAINT_COUNT, 3);
        e.mem.set_u8(islands[0] + ISLAND_FLAGS_26, 4);
        e.mem.set_u32(islands[1] + ISLAND_ENTITY_COUNT, 20);
        e.mem.set_u8(islands[1] + ISLAND_FLAGS_26, 4);
        e.mem.set_u8(islands[2] + ISLAND_FLAGS_26, 4);
        let array = e.mem.alloc(12);
        word_array(&mut e, array, &islands);
        let found = e
            .call(0x00cb_3d10, &args![world, array, 4i32, islands[2]])
            .u32();
        assert_eq!(found, islands[0]);
        assert_eq!(e.mem.u8(islands[0] + ISLAND_FLAGS_25), 4);
        // An island without the active mark is passed over.
        e.mem.set_u8(islands[0] + ISLAND_FLAGS_26, 0);
        let found = e
            .call(0x00cb_3d10, &args![world, array, 4i32, islands[2]])
            .u32();
        assert_eq!(found, 0);
        // The requested extra size at the minimum, or no island management: nothing.
        e.mem.set_u8(islands[0] + ISLAND_FLAGS_26, 4);
        assert_eq!(
            e.call(0x00cb_3d10, &args![world, array, 10i32, 0u32]).u32(),
            0
        );
        e.mem.set_u8(world + WORLD_WANT_ISLANDS, 0);
        assert_eq!(
            e.call(0x00cb_3d10, &args![world, array, 4i32, 0u32]).u32(),
            0
        );
    }

    #[test]
    fn remove_constraint_takes_a_lock_only_when_none_is_held() {
        let (mut e, _) = rig();
        stub(
            &mut e,
            &[
                WORLD_CONSTRAINT_UTIL_REMOVE,
                FIRE_CONSTRAINT_REMOVED,
                WORLD_RUN_PENDING_OPERATIONS,
                WORLD_RUN_PENDING_BODY_OPERATIONS,
            ],
        );
        let world = e.mem.alloc(0x350);
        e.mem.set_u32(world + WORLD_CONSTRAINT_LISTENERS_SIZE, 1);
        e.mem.set_u32(world + WORLD_PENDING_OPERATIONS, 1);
        let owner = e.mem.alloc(0x40);
        e.mem.set_u8(owner + ISLAND_FLAGS_25, 0xff);
        let constraint = e.mem.alloc(0x20);
        e.mem.set_u32(constraint + 8, owner);
        e.call(0x00cb_3dd0, &args![world, constraint, 1u32]);
        assert_eq!(e.mem.u8(owner + ISLAND_FLAGS_25), 0xfd | 1);
        assert_eq!(
            order(
                &e,
                &[
                    FIRE_CONSTRAINT_REMOVED,
                    WORLD_CONSTRAINT_UTIL_REMOVE,
                    PENDING[0]
                ]
            ),
            vec![
                FIRE_CONSTRAINT_REMOVED,
                WORLD_CONSTRAINT_UTIL_REMOVE,
                PENDING[0]
            ]
        );
        assert_eq!(
            calls(&e, WORLD_CONSTRAINT_UTIL_REMOVE),
            vec![vec![constraint]]
        );
        assert_eq!(e.mem.u32(world + WORLD_LOCK_COUNT), 0);

        // Under a lock: removal only, the lock count untouched, no notification when not asked.
        e.call_log = Some(Vec::new());
        e.mem.set_u32(world + WORLD_LOCK_COUNT, 3);
        e.call(0x00cb_3dd0, &args![world, constraint, 0u32]);
        assert!(calls(&e, FIRE_CONSTRAINT_REMOVED).is_empty());
        assert!(calls(&e, PENDING[0]).is_empty());
        assert_eq!(calls(&e, WORLD_CONSTRAINT_UTIL_REMOVE).len(), 1);
        assert_eq!(e.mem.u32(world + WORLD_LOCK_COUNT), 3);

        // Under a lock with a notification and listeners.
        e.call(0x00cb_3dd0, &args![world, constraint, 1u32]);
        assert_eq!(calls(&e, FIRE_CONSTRAINT_REMOVED).len(), 1);
    }

    #[test]
    fn remove_attached_agents_moves_agents_to_the_fixed_tracks() {
        let (mut e, _) = rig();
        stub(
            &mut e,
            &[AGENT_COPY_AND_RELINK_ENTRY, AGENT_DEALLOCATE_ENTRY],
        );
        let entity = e.mem.alloc(0x220);
        let entity_island = e.mem.alloc(0x6c);
        e.mem.set_u32(entity + ENTITY_ISLAND, entity_island);
        // Two partners: a fixed one and a dynamic one in another island.
        let fixed = e.mem.alloc(0x220);
        e.mem.set_u8(fixed + ENTITY_MOTION_TYPE, 5);
        let dynamic = e.mem.alloc(0x220);
        e.mem.set_u8(dynamic + ENTITY_MOTION_TYPE, 1);
        let partner_island = e.mem.alloc(0x6c);
        e.mem.set_u32(dynamic + ENTITY_ISLAND, partner_island);
        // Each partner's collidable is at +0x10 with the owner offset byte (-0x10) at +0x10.
        for partner in [fixed, dynamic] {
            e.mem.set_i8(partner + 0x20, -0x10);
        }
        let entries = e.mem.alloc(12);
        let data = e.mem.alloc(16);
        e.mem.set_u32(data, 0xa1);
        e.mem.set_u32(data + 4, fixed + 0x10);
        e.mem.set_u32(data + 8, 0xa2);
        e.mem.set_u32(data + 12, dynamic + 0x10);
        e.mem.set_u32(entries, data);
        e.mem.set_u32(entries + 4, 2);
        e.register_double(LINKED_COLLIDABLE_ENTRIES, move |_, _| entries.into_ret());
        e.register_double(AGENT_UTIL_GET_ISLAND_FROM_ENTRY, move |_, _| {
            entity_island.into_ret()
        });
        e.call(
            0x00cb_3ee0,
            &args![0x7000u32, entity, 0x8000u32, MOTION_FIXED],
        );
        assert_eq!(
            calls(&e, AGENT_COPY_AND_RELINK_ENTRY),
            vec![vec![0x8000, 0xa1], vec![partner_island + 0x58, 0xa2]]
        );
        assert_eq!(
            calls(&e, AGENT_DEALLOCATE_ENTRY),
            vec![vec![0x7000, 0xa1], vec![0x7000, 0xa2]]
        );
        assert_eq!(
            calls(&e, AGENT_UTIL_GET_ISLAND_FROM_ENTRY),
            vec![vec![0xa2, entity_island, partner_island]]
        );
        // Not turning fixed: the dynamic partner is left alone.
        e.call_log = Some(Vec::new());
        e.call(0x00cb_3ee0, &args![0x7000u32, entity, 0x8000u32, 1u32]);
        assert_eq!(calls(&e, AGENT_COPY_AND_RELINK_ENTRY).len(), 1);
    }

    #[test]
    fn remove_island_from_dirty_list_nulls_the_slot() {
        let (mut e, _) = rig();
        let world = e.mem.alloc(0x350);
        let island = e.mem.alloc(0x6c);
        let list = e.mem.alloc(12);
        word_array(&mut e, list, &[0x11, island, 0x33]);
        e.mem.set_u32(world + WORLD_DIRTY_ISLANDS, e.mem.u32(list));
        e.mem.set_u32(world + WORLD_DIRTY_ISLANDS + 4, 3);
        e.mem.set_u16(island + ISLAND_DIRTY_INDEX, 1);
        e.call(0x00cb_3fd0, &args![world, island]);
        assert_eq!(array_words(&e, list), vec![0x11, 0, 0x33]);
        assert_eq!(e.mem.u16(island + ISLAND_DIRTY_INDEX), 0xffff);
        // Not in the list: nothing changes.
        e.call(0x00cb_3fd0, &args![world, island]);
        assert_eq!(array_words(&e, list), vec![0x11, 0, 0x33]);
    }

    #[test]
    fn fn_00cb4000_fills_the_hole_with_the_last_island() {
        let (mut e, _) = rig();
        let world = e.mem.alloc(0x350);
        let islands: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x6c)).collect();
        for (i, island) in islands.iter().enumerate() {
            e.mem.set_u16(island + ISLAND_INDEX, i as u16);
            e.mem.set_u16(island + ISLAND_DIRTY_INDEX, 0xffff);
        }
        // Inactive list (flag bits 0-1 clear): remove island 0.
        let list = e.mem.alloc(12);
        word_array(&mut e, list, &islands);
        e.mem
            .set_u32(world + WORLD_INACTIVE_ISLANDS, e.mem.u32(list));
        e.mem.set_u32(world + WORLD_INACTIVE_ISLANDS + 4, 3);
        e.call(0x00cb_4000, &args![world, islands[0]]);
        assert_eq!(
            array_words(&e, world + WORLD_INACTIVE_ISLANDS)[..2],
            [islands[2], islands[1]]
        );
        assert_eq!(e.mem.u32(world + WORLD_INACTIVE_ISLANDS + 4), 2);
        assert_eq!(e.mem.u16(islands[2] + ISLAND_INDEX), 0);
        // Active list: the island's flag selects it.
        let list = e.mem.alloc(12);
        word_array(&mut e, list, &islands[..2]);
        e.mem.set_u32(world + WORLD_ACTIVE_ISLANDS, e.mem.u32(list));
        e.mem.set_u32(world + WORLD_ACTIVE_ISLANDS + 4, 2);
        e.mem.set_u8(islands[1] + ISLAND_FLAGS_26, 1);
        e.mem.set_u16(islands[1] + ISLAND_INDEX, 1);
        e.call(0x00cb_4000, &args![world, islands[1]]);
        assert_eq!(e.mem.u32(world + WORLD_ACTIVE_ISLANDS + 4), 1);
    }

    /// Motion fixture: an entity with an embedded motion whose vtable is the generic object.
    fn motion_entity(e: &mut Engine) -> u32 {
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + ENTITY_MOTION, OBJECT_VTABLE);
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 1);
        e.mem.write(entity + 0xe4, &[0x11, 0x22, 0x33, 0x44]);
        entity
    }

    const MOTION_DOUBLES: [u32; 11] = [
        KEYFRAMED_MOTION_DESTRUCTOR,
        MOTION_STATE_ASSIGN,
        MOTION_COPY_STATE_AND_VELOCITIES,
        MOTION_GET_MASS,
        RIGID_BODY_CREATE_DYNAMIC_MOTION,
        FREEZE_MOTION_STATE,
        ENTITY_SET_CACHED_SHAPE_DATA,
        WORLD_SET_MOTION_FLAGS,
        MEM_SET,
        REMOVE_REFERENCE,
        WORLD_GET_CURRENT_TIME,
    ];

    #[test]
    fn replace_motion_object_to_fixed_freezes_the_state_and_keeps_the_old_motion() {
        let (mut e, _) = rig();
        stub(&mut e, &MOTION_DOUBLES);
        e.register(FIXED_MOTION_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(WORLD_GET_CURRENT_TIME, |_, _| Ret {
            st0: 2.5,
            ..Ret::default()
        });
        let entity = motion_entity(&mut e);
        e.mem.set_u8(entity + ENTITY_BYTE_2A, 0xfe);
        e.mem.set_u8(entity + ENTITY_MOTION + 9, 0x77);
        e.mem.set_f32(entity + ENTITY_MOTION + 0x6c, 4.0);
        e.mem.set_f32(entity + ENTITY_MOTION + 0x5c, 10.0);
        e.mem.set_u32(entity + 0xa4, 9);
        let world = e.mem.alloc(0x350);
        e.mem.set_u8(world + 0x305, 1);
        e.mem.set_u8(world + 0x306, 2);
        e.mem.set_u8(world + 0x307, 3);
        // Fixed, from a dynamic motion, with a world: frozen at the world's time. The motion
        // copy sits at +0x20 of the exe's aligned 0x170-byte frame.
        let frame = next_aligned_frame(&mut e, 0x170);
        e.call(0x00cb_4230, &args![entity, 5u32, 0u32, 1u32, world]);
        assert_eq!(
            calls(&e, KEYFRAMED_MOTION_DESTRUCTOR),
            vec![vec![frame + 0x20]]
        );
        assert_eq!(
            calls(&e, FIXED_MOTION_CONSTRUCTOR),
            vec![vec![entity + 0xe0, entity + 0x120, entity + 0x160]]
        );
        let freeze = calls(&e, FREEZE_MOTION_STATE);
        assert_eq!(freeze, vec![vec![2.5f32.to_bits(), entity + 0xf0]]);
        assert_eq!(e.mem.u8(entity + ENTITY_MOTION + 9), 0x77);
        assert_eq!(e.mem.u32(entity + 0xa4), 0);
        assert_eq!(e.mem.u8(entity + ENTITY_BYTE_2A), 0);
        assert_eq!(e.mem.u32(entity + 0x18), entity + 0xf0);
        assert_eq!(
            calls(&e, WORLD_SET_MOTION_FLAGS),
            vec![vec![entity + 0xe0, 1, 2, 3]]
        );
        // The old motion was copied to a heap block hung off the new one at +0x118 with size
        // and reference count in its header.
        let block = e.mem.u32(entity + ENTITY_MOTION + 0x118);
        assert_eq!(e.mem.u16(block + 4), 0x120);
        assert_eq!(e.mem.u16(block + 6), 1);
        assert_eq!(e.mem.u32(block), OBJECT_VTABLE);
        assert_eq!(e.mem.u16(entity + ENTITY_MOTION + 0x11c), 0xfffe);
        assert_eq!(calls(&e, ENTITY_SET_CACHED_SHAPE_DATA).len(), 1);
        assert_eq!(calls(&e, KEYFRAMED_MOTION_DESTRUCTOR).len(), 1);

        // Without a world the freeze time is 1 / rate + start; no flags call.
        e.call_log = Some(Vec::new());
        e.call(0x00cb_4230, &args![entity, 5u32, 0u32, 0u32, 0u32]);
        let expected = (1.0f64 / 4.0 + 10.0) as f32;
        assert_eq!(
            calls(&e, FREEZE_MOTION_STATE),
            vec![vec![expected.to_bits(), entity + 0xf0]]
        );
        assert!(calls(&e, WORLD_SET_MOTION_FLAGS).is_empty());
        // A zero rate means no freeze.
        e.call_log = Some(Vec::new());
        e.mem.set_f32(entity + ENTITY_MOTION + 0x6c, 0.0);
        e.call(0x00cb_4230, &args![entity, 5u32, 0u32, 0u32, 0u32]);
        assert!(calls(&e, FREEZE_MOTION_STATE).is_empty());
    }

    #[test]
    fn replace_motion_object_to_keyframed_hands_on_the_saved_motion() {
        let (mut e, _) = rig();
        stub(&mut e, &MOTION_DOUBLES);
        e.register(KEYFRAMED_MOTION_CONSTRUCTOR, |_, a| a[0].into_ret());
        let entity = motion_entity(&mut e);
        e.mem.set_u32(entity + 0x1f8, 0xabcd);
        e.mem.set_u16(entity + 0x1fc, 0x1234);
        e.call(0x00cb_4230, &args![entity, 4u32, 0u32, 0u32, 0u32]);
        assert_eq!(calls(&e, KEYFRAMED_MOTION_CONSTRUCTOR).len(), 1);
        assert_eq!(calls(&e, MOTION_COPY_STATE_AND_VELOCITIES).len(), 1);
        assert_eq!(calls(&e, OBJECT_SLOT_6C), vec![vec![entity + 0xe0, 0xabcd]]);
        assert_eq!(e.mem.u16(entity + ENTITY_MOTION + 0x11c), 0x1234);
        assert_eq!(e.mem.u8(entity + ENTITY_BYTE_2A), 1);
        assert!(calls(&e, FREEZE_MOTION_STATE).is_empty());
    }

    #[test]
    fn replace_motion_object_to_dynamic_restores_or_rebuilds() {
        let (mut e, _) = rig();
        stub(&mut e, &MOTION_DOUBLES);
        e.register(RIGID_BODY_CREATE_DYNAMIC_MOTION, |_, _| 0u32.into_ret());
        e.register(MOTION_GET_MASS, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        // The saved motion block and the table entries.
        let entity = motion_entity(&mut e);
        e.mem.set_u32(MOTION_FLOAT_TABLE + 4, 1.5f32.to_bits());
        e.mem.set_u32(MOTION_FLOAT_TABLE + 8, 2.5f32.to_bits());
        // From a non-dynamic type: restore the motion saved at +0x1f8.
        let saved = e.mem.alloc(0x120);
        e.mem.set_u32(saved, OBJECT_VTABLE);
        e.mem.set_u8(saved + 8, 2);
        e.mem.set_u8(saved + 9, 0x55);
        e.mem.set_u16(saved + 0xa, 0x99);
        e.mem.set_u8(entity + ENTITY_MOTION + 0x11c, 7);
        e.mem.set_u32(entity + 0x1f8, saved);
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 4);
        e.call(0x00cb_4230, &args![entity, 2u32, 1u32, 0u32, 0u32]);
        assert_eq!(calls(&e, OBJECT_SLOT_64), vec![vec![entity + 0xe0, saved]]);
        assert_eq!(e.mem.u8(entity + ENTITY_BYTE_2A), 7);
        assert_eq!(e.mem.u8(entity + ENTITY_MOTION_TYPE), 2);
        assert_eq!(e.mem.u16(entity + 0xea), 0);
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![saved]]);
        assert_eq!(calls(&e, MEM_SET), vec![vec![entity + 0x200, 0, 0]]);
        assert_eq!(calls(&e, ENTITY_SET_CACHED_SHAPE_DATA).len(), 1);
        // Type 2 is now the entity's type: nothing is rebuilt.
        assert!(calls(&e, RIGID_BODY_CREATE_DYNAMIC_MOTION).is_empty());

        // From a dynamic type to another one: mass and inertia rebuild.
        e.call_log = Some(Vec::new());
        e.mem.set_u8(entity + ENTITY_MOTION + 0xbc, 2);
        e.mem.set_u8(entity + ENTITY_MOTION + 0xbd, 1);
        e.mem.set_f32(entity + ENTITY_MOTION + 0xb4, 0.25);
        e.mem.set_f32(entity + ENTITY_MOTION + 0xb8, 0.5);
        e.call(0x00cb_4230, &args![entity, 3u32, 1u32, 1u32, 0u32]);
        let mass = calls(&e, MOTION_GET_MASS);
        assert_eq!(mass.len(), 1);
        assert_eq!(mass[0].len(), 1);
        let create = calls(&e, RIGID_BODY_CREATE_DYNAMIC_MOTION);
        assert_eq!(create.len(), 1);
        assert_eq!(create[0].len(), 9);
        assert_eq!(create[0][0], 3);
        assert_eq!(create[0][3], 3.0f32.to_bits());
        assert_eq!(create[0][4], mass[0][0] + 0x120);
        assert_eq!(create[0][5], mass[0][0] + 0x90);
        assert_eq!(create[0][6], 2.5f32.to_bits());
        assert_eq!(create[0][7], 1.5f32.to_bits());
        assert_eq!(create[0][8], entity + 0xe0);
        assert_eq!(calls(&e, OBJECT_SLOT_14).len(), 1);
        assert_eq!(e.mem.f32(entity + 0x194), 0.25);
        assert_eq!(e.mem.f32(entity + 0x198), 0.5);
        assert_eq!(calls(&e, KEYFRAMED_MOTION_DESTRUCTOR).len(), 1);

        // The same type, or the generic type 1: unchanged.
        e.call_log = Some(Vec::new());
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 3);
        e.call(0x00cb_4230, &args![entity, 3u32, 1u32, 1u32, 0u32]);
        e.call(0x00cb_4230, &args![entity, 1u32, 1u32, 1u32, 0u32]);
        assert!(calls(&e, RIGID_BODY_CREATE_DYNAMIC_MOTION).is_empty());
        assert!(calls(&e, KEYFRAMED_MOTION_DESTRUCTOR).is_empty());
    }

    /// A world with one active island and an island vtable of recorded calls.
    fn island_world(e: &mut Engine) -> (u32, u32) {
        let world = e.mem.alloc(0x350);
        let fixed = e.mem.alloc(0x6c);
        e.mem.set_u32(world + WORLD_FIXED_ISLAND, fixed);
        (world, fixed)
    }

    #[test]
    fn add_entity_si_picks_the_island() {
        let (mut e, _) = rig();
        stub(&mut e, &[ISLAND_ADD_ENTITY, 0x00cb_3d10]);
        let (world, fixed) = island_world(&mut e);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u8(entity + ENTITY_UNIQUE_ID, 0x42);
        // A fixed entity goes to the fixed island and gets no id byte.
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 5);
        e.call(0x00cb_4640, &args![world, entity, 0u32]);
        assert_eq!(e.mem.u32(entity + ENTITY_WORLD), world);
        assert_eq!(calls(&e, ISLAND_ADD_ENTITY), vec![vec![fixed, entity]]);
        assert_eq!(e.mem.u8(fixed + ISLAND_BYTE_24), 0);

        // Without island management: the first active island.
        e.call_log = Some(Vec::new());
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 1);
        let first = e.mem.alloc(0x6c);
        let active = e.mem.alloc(12);
        word_array(&mut e, active, &[first]);
        e.mem
            .set_u32(world + WORLD_ACTIVE_ISLANDS, e.mem.u32(active));
        e.mem.set_u32(world + WORLD_ACTIVE_ISLANDS + 4, 1);
        e.call(0x00cb_4640, &args![world, entity, 1u32]);
        assert_eq!(calls(&e, ISLAND_ADD_ENTITY), vec![vec![first, entity]]);
        assert_eq!(e.mem.u8(first + ISLAND_BYTE_24), 0x42);

        // Island management, mode 1 with a small sparse island found: no id byte.
        e.call_log = Some(Vec::new());
        e.mem.set_u8(world + WORLD_WANT_ISLANDS, 1);
        let small = e.mem.alloc(0x6c);
        e.register_double(0x00cb_3d10, move |_, _| small.into_ret());
        e.call(0x00cb_4640, &args![world, entity, 1u32]);
        assert_eq!(
            calls(&e, 0x00cb_3d10),
            vec![vec![world, world + 0x28, 4, 0]]
        );
        assert_eq!(calls(&e, ISLAND_ADD_ENTITY), vec![vec![small, entity]]);
        assert_eq!(e.mem.u8(small + ISLAND_BYTE_24), 0);
    }

    #[test]
    fn add_entity_si_builds_a_new_island() {
        let (mut e, _) = rig();
        stub(&mut e, &[ISLAND_ADD_ENTITY]);
        let (world, _) = island_world(&mut e);
        e.mem.set_u8(world + WORLD_WANT_ISLANDS, 1);
        let built = e.mem.alloc(0x6c);
        e.register_double(ISLAND_CONSTRUCTOR, move |_, _| built.into_ret());
        e.register(0x00cb_3d10, |_, _| 0u32.into_ret());
        let entity = e.mem.alloc(0x220);
        e.mem.set_u8(entity + ENTITY_UNIQUE_ID, 0x21);
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 1);
        // Mode 1: a sparse island appended to the active list.
        let active = e.mem.alloc(12);
        word_array(&mut e, active, &[0x1111, 0x2222]);
        e.mem
            .set_u32(world + WORLD_ACTIVE_ISLANDS, e.mem.u32(active));
        e.mem.set_u32(world + WORLD_ACTIVE_ISLANDS + 4, 2);
        e.mem.set_u32(world + WORLD_ACTIVE_ISLANDS + 8, 2);
        e.mem.set_u8(built + ISLAND_FLAGS_26, 0xff);
        e.call(0x00cb_4640, &args![world, entity, 1u32]);
        assert_eq!(calls(&e, ISLAND_CONSTRUCTOR).len(), 1);
        assert_eq!(calls(&e, ISLAND_CONSTRUCTOR)[0][1], world);
        // 4 | (0xff & 0xf3), then & 0xfc, | 1.
        assert_eq!(e.mem.u8(built + ISLAND_FLAGS_26), 0xf5);
        assert_eq!(e.mem.u16(built + ISLAND_INDEX), 2);
        assert_eq!(
            array_words(&e, world + WORLD_ACTIVE_ISLANDS),
            vec![0x1111, 0x2222, built]
        );
        assert_eq!(e.mem.u16(calls(&e, ISLAND_CONSTRUCTOR)[0][0] + 4), 0x6c);
        assert_eq!(e.mem.u8(built + ISLAND_BYTE_24), 0x21);

        // Mode 0: appended to the inactive list; the flag bits are 0.
        e.mem.set_u8(built + ISLAND_FLAGS_26, 0xff);
        e.call(0x00cb_4640, &args![world, entity, 0u32]);
        assert_eq!(e.mem.u8(built + ISLAND_FLAGS_26), 0xf0);
        assert_eq!(array_words(&e, world + WORLD_INACTIVE_ISLANDS), vec![built]);
    }

    #[test]
    fn remove_entity_si_deletes_an_emptied_island() {
        let (mut e, _) = rig();
        stub(&mut e, &[ISLAND_REMOVE_ENTITY, 0x00cb_4000]);
        let world = e.mem.alloc(0x350);
        e.mem.set_u8(world + WORLD_WANT_ISLANDS, 1);
        let island = object(&mut e, 0x6c);
        e.put_vtable(OBJECT_VTABLE, &[OBJECT_SLOT_10]);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + ENTITY_ISLAND, island);
        e.mem.set_u32(entity + ENTITY_WORLD, world);
        e.mem.set_u16(island + ISLAND_INDEX, 3);
        e.call(0x00cb_4740, &args![world, entity]);
        assert_eq!(e.mem.u32(entity + ENTITY_WORLD), 0);
        assert_eq!(calls(&e, ISLAND_REMOVE_ENTITY), vec![vec![island, entity]]);
        assert_eq!(calls(&e, 0x00cb_4000), vec![vec![world, island]]);
        assert_eq!(calls(&e, OBJECT_SLOT_10), vec![vec![island, 1]]);

        // An island that still holds entities, or is not in a list, is kept.
        e.call_log = Some(Vec::new());
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, 1);
        e.call(0x00cb_4740, &args![world, entity]);
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, 0);
        e.mem.set_u16(island + ISLAND_INDEX, 0xffff);
        e.call(0x00cb_4740, &args![world, entity]);
        assert!(calls(&e, 0x00cb_4000).is_empty());
        assert_eq!(calls(&e, ISLAND_REMOVE_ENTITY).len(), 2);
    }

    #[test]
    fn fn_00cb4790_takes_over_the_actions_of_the_fixed_island() {
        let (mut e, _) = rig();
        stub(&mut e, &[ADD_REFERENCE, ISLAND_REGISTER_ACTION]);
        let (world, fixed) = island_world(&mut e);
        e.mem.set_u16(fixed + ISLAND_DIRTY_INDEX, 0xffff);
        let entity = e.mem.alloc(0x220);
        let mine = e.mem.alloc(0x20);
        e.mem.set_u32(mine + 0xc, fixed);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other + 0xc, 0x1234);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, other);
        e.mem.set_u32(data + 4, mine);
        e.mem.set_u32(entity + ENTITY_ACTIONS, data);
        e.mem.set_u16(entity + ENTITY_ACTIONS_SIZE, 2);
        let out = e.mem.alloc(12);
        e.mem.set_u32(out + 8, 0x8000_0000);
        e.call(0x00cb_4790, &args![world, entity, out]);
        assert_eq!(array_words(&e, out), vec![mine]);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![mine]]);
        assert_eq!(calls(&e, ISLAND_REGISTER_ACTION), vec![vec![fixed, mine]]);
        assert_eq!(e.mem.u8(fixed + ISLAND_FLAGS_25), 0x10);
        assert_eq!(array_words(&e, world + WORLD_DIRTY_ISLANDS), vec![fixed]);
        assert_eq!(e.mem.u16(fixed + ISLAND_DIRTY_INDEX), 0);
        // No actions: nothing.
        e.call_log = Some(Vec::new());
        e.mem.set_u16(entity + ENTITY_ACTIONS_SIZE, 0);
        e.call(0x00cb_4790, &args![world, entity, out]);
        assert!(calls(&e, ADD_REFERENCE).is_empty());
    }

    #[test]
    fn fn_00cb5000_removes_masters_then_slaves_last_first() {
        let (mut e, _) = rig();
        stub(&mut e, &[ADD_REFERENCE, 0x00cb_3dd0]);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + ENTITY_WORLD, 0x9000);
        let masters = e.mem.alloc(0x60);
        e.mem.set_u32(masters, 0xa0);
        e.mem.set_u32(masters + 0x30, 0xa1);
        e.mem.set_u32(entity + 0xac, masters);
        e.mem.set_u16(entity + 0xb0, 2);
        let slaves = e.mem.alloc(8);
        e.mem.set_u32(slaves, 0xb0);
        e.mem.set_u32(slaves + 4, 0xb1);
        e.mem.set_u32(entity + 0xb4, slaves);
        e.mem.set_u32(entity + 0xb8, 2);
        let out = e.mem.alloc(12);
        e.mem.set_u32(out + 8, 0x8000_0000);
        e.call(0x00cb_5000, &args![entity, out]);
        assert_eq!(array_words(&e, out), vec![0xa1, 0xa0, 0xb1, 0xb0]);
        assert_eq!(
            calls(&e, 0x00cb_3dd0),
            vec![
                vec![0x9000, 0xa1, 0],
                vec![0x9000, 0xa0, 0],
                vec![0x9000, 0xb1, 0],
                vec![0x9000, 0xb0, 0]
            ]
        );
        assert_eq!(calls(&e, ADD_REFERENCE).len(), 4);
    }

    #[test]
    fn fn_00cb5500_marks_and_lists_the_island() {
        let (mut e, _) = rig();
        let world = e.mem.alloc(0x350);
        let island = e.mem.alloc(0x6c);
        e.mem.set_u16(island + ISLAND_DIRTY_INDEX, 0xffff);
        // In the active list, unmarked: the try-increase bits then the mark.
        e.mem.set_u8(island + ISLAND_FLAGS_26, 0x21);
        e.call(0x00cb_5500, &args![world, island]);
        assert_eq!(e.mem.u8(island + ISLAND_FLAGS_26), 0x15);
        assert_eq!(array_words(&e, world + WORLD_DIRTY_ISLANDS), vec![island]);
        // Already marked: only the mark bits are normalised; already listed: not listed twice.
        e.mem.set_u8(island + ISLAND_FLAGS_26, 0x28);
        e.call(0x00cb_5500, &args![world, island]);
        assert_eq!(e.mem.u8(island + ISLAND_FLAGS_26), 0x24);
        assert_eq!(array_words(&e, world + WORLD_DIRTY_ISLANDS).len(), 1);
    }

    #[test]
    fn merge_islands_queues_while_locked() {
        let (mut e, _) = rig();
        stub(
            &mut e,
            &[WORLD_ENQUEUE_OPERATION, INTERNAL_MERGE_TWO_ISLANDS],
        );
        let world = e.mem.alloc(0x350);
        let (first, second) = (e.mem.alloc(0x220), e.mem.alloc(0x220));
        e.mem.set_u32(first + ENTITY_ISLAND, 0x111);
        e.mem.set_u32(second + ENTITY_ISLAND, 0x222);
        e.call(0x00cb_5570, &args![world, first, second]);
        assert_eq!(
            calls(&e, INTERNAL_MERGE_TWO_ISLANDS),
            vec![vec![world, 0x111, 0x222]]
        );
        assert!(calls(&e, WORLD_ENQUEUE_OPERATION).is_empty());
        e.call_log = Some(Vec::new());
        e.mem.set_u32(world + WORLD_LOCK_COUNT, 1);
        e.register_double(WORLD_ENQUEUE_OPERATION, move |e, a| {
            // The record: byte 0xc, then the two entities.
            assert_eq!(e.mem.u8(a[1]), 0xc);
            assert_eq!(e.mem.u32(a[1] + 4), first);
            assert_eq!(e.mem.u32(a[1] + 8), second);
            0u32.into_ret()
        });
        e.call(0x00cb_5570, &args![world, first, second]);
        assert_eq!(calls(&e, WORLD_ENQUEUE_OPERATION).len(), 1);
        assert!(calls(&e, INTERNAL_MERGE_TWO_ISLANDS).is_empty());
    }

    /// A world with a broad phase and a dispatcher (both generic objects) and a filter.
    fn broad_phase_world(e: &mut Engine) -> (u32, u32, u32) {
        let world = e.mem.alloc(0x350);
        let broad_phase = object(e, 0x10);
        let dispatcher = e.mem.alloc(0x10);
        e.mem.set_u32(world + WORLD_BROAD_PHASE, broad_phase);
        e.mem.set_u32(world + WORLD_DISPATCHER, dispatcher);
        e.mem.set_u32(world + WORLD_COLLISION_INPUT, 0x4000);
        e.mem.set_u32(world + WORLD_BROAD_PHASE_QUERY_SIZE, 4);
        e.mem.set_u32(world + WORLD_COLLISION_FILTER, 0x6000);
        (world, broad_phase, dispatcher)
    }

    #[test]
    fn update_entity_bp_exchanges_the_pairs() {
        let (mut e, router) = rig();
        stub(
            &mut e,
            &[
                ENTITY_BATCH_RECALC_AABB,
                DISPATCHER_REMOVE_DUPLICATES,
                DISPATCHER_REMOVE_PAIRS,
                DISPATCHER_ADD_PAIRS,
                WORLD_ENQUEUE_OPERATION,
                WORLD_RUN_PENDING_OPERATIONS,
                WORLD_RUN_PENDING_BODY_OPERATIONS,
            ],
        );
        let (world, _broad_phase, dispatcher) = broad_phase_world(&mut e);
        let entity = e.mem.alloc(0x220);
        // The AABB recalculation gets the entity slot.
        e.register_double(ENTITY_BATCH_RECALC_AABB, move |e, a| {
            assert_eq!((a[0], a[2]), (0x4000, 1));
            assert_eq!(e.mem.u32(a[1]), entity);
            0u32.into_ret()
        });
        // The broad phase gets the handle slot, the box and 1, and fills the "added" array
        // with two pairs and "removed" with one.
        e.register_double(OBJECT_SLOT_2C, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), entity + 0x24);
            assert_eq!(a[3], 1);
            let low: Vec<u32> = (0..3).map(|i| e.mem.u32(a[2] + i * 4)).collect();
            assert_eq!(low, vec![100 - 4, 200 - 8, 300 - 12]);
            let high: Vec<u32> = (0..3).map(|i| e.mem.u32(a[2] + 0x10 + i * 4)).collect();
            assert_eq!(high, vec![(4 << 2) + 50, (5 << 2) + 150, (6 << 2) + 250]);
            let (added, removed) = (a[4], a[5]);
            let data = e.mem.u32(added);
            e.mem.set_u32(data, 1);
            e.mem.set_u32(added + 4, 2);
            let data = e.mem.u32(removed);
            e.mem.set_u32(data, 2);
            e.mem.set_u32(removed + 4, 1);
            0u32.into_ret()
        });
        e.mem.set_u32(entity + 0x10, 0x7777);
        for (i, v) in [100u32, 200, 300].iter().enumerate() {
            e.mem.set_u32(entity + 0x30 + i as u32 * 4, *v);
            e.mem.set_u32(entity + 0x40 + i as u32 * 4, *v - 50);
        }
        // Extents 1, 2, 3 and 4, 5, 6 shifted by 2.
        e.mem.write(entity + 0x3c, &[1, 2, 3, 2]);
        e.mem.write(entity + 0x4c, &[4, 5, 6]);
        let top = e.mem.u32(router);
        e.call(0x00cb_5750, &args![world, entity]);
        assert_eq!(calls(&e, ENTITY_BATCH_RECALC_AABB).len(), 1);
        assert_eq!(calls(&e, OBJECT_SLOT_2C).len(), 1);
        assert_eq!(calls(&e, DISPATCHER_REMOVE_DUPLICATES).len(), 1);
        let removed = calls(&e, DISPATCHER_REMOVE_PAIRS);
        assert_eq!((removed[0][0], removed[0][2]), (dispatcher, 1));
        let added = calls(&e, DISPATCHER_ADD_PAIRS);
        assert_eq!(
            (added[0][0], added[0][2], added[0][3]),
            (dispatcher, 2, 0x6008)
        );
        assert_eq!(e.mem.u32(world + WORLD_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(router), top);

        // A locked world only queues the entity.
        e.call_log = Some(Vec::new());
        e.mem.set_u32(world + WORLD_LOCK_COUNT, 1);
        e.call(0x00cb_5750, &args![world, entity]);
        assert_eq!(calls(&e, WORLD_ENQUEUE_OPERATION).len(), 1);
        assert!(calls(&e, OBJECT_SLOT_2C).is_empty());

        // An entity without a shape only takes and drops the lock.
        e.call_log = Some(Vec::new());
        e.mem.set_u32(world + WORLD_LOCK_COUNT, 0);
        e.mem.set_u32(entity + 0x10, 0);
        e.call(0x00cb_5750, &args![world, entity]);
        assert!(calls(&e, OBJECT_SLOT_2C).is_empty());
        assert_eq!(e.mem.u32(world + WORLD_LOCK_COUNT), 0);
    }

    #[test]
    fn add_entity_bp_adds_the_new_pairs() {
        let (mut e, router) = rig();
        stub(&mut e, &[ENTITY_BATCH_RECALC_AABB, DISPATCHER_ADD_PAIRS]);
        let (world, _, dispatcher) = broad_phase_world(&mut e);
        let entity = e.mem.alloc(0x220);
        e.register_double(OBJECT_SLOT_14, move |e, a| {
            // The handle, the copy of the entity's AABB, the pair array.
            assert_eq!(a[1], entity + 0x24);
            assert_eq!(e.mem.bytes(a[2], 32), vec![9u8; 32]);
            let array = a[3];
            let data = e.mem.u32(array);
            e.mem.set_u32(data, 0xdead);
            e.mem.set_u32(array + 4, 1);
            0u32.into_ret()
        });
        e.mem.set_u32(entity + 0x10, 0x7777);
        e.mem.write(entity + 0x30, &[9u8; 32]);
        let top = e.mem.u32(router);
        e.call(0x00cb_59f0, &args![world, entity]);
        assert_eq!(calls(&e, OBJECT_SLOT_14).len(), 1);
        let added = calls(&e, DISPATCHER_ADD_PAIRS);
        assert_eq!(
            (added[0][0], added[0][2], added[0][3]),
            (dispatcher, 1, 0x6008)
        );
        assert_eq!(e.mem.u32(added[0][1]), 0xdead);
        assert_eq!(calls(&e, ENTITY_BATCH_RECALC_AABB).len(), 1);
        assert_eq!(e.mem.u32(router), top);
        // No pairs: nothing is added; no shape: nothing at all.
        e.call_log = Some(Vec::new());
        e.register(OBJECT_SLOT_14, |_, _| 0u32.into_ret());
        e.call(0x00cb_59f0, &args![world, entity]);
        assert!(calls(&e, DISPATCHER_ADD_PAIRS).is_empty());
        e.mem.set_u32(entity + 0x10, 0);
        e.call_log = Some(Vec::new());
        e.call(0x00cb_59f0, &args![world, entity]);
        assert!(calls(&e, ENTITY_BATCH_RECALC_AABB).is_empty());
    }

    #[test]
    fn add_phantom_bp_stores_the_bounding_volume() {
        let (mut e, router) = rig();
        stub(
            &mut e,
            &[PHANTOM_SET_BOUNDING_VOLUME_DATA, DISPATCHER_ADD_PAIRS],
        );
        let (world, _, dispatcher) = broad_phase_world(&mut e);
        let phantom = object(&mut e, 0x40);
        e.register(OBJECT_SLOT_10, |e, a| {
            let array = a[3];
            let data = e.mem.u32(array);
            e.mem.set_u32(data, 0xbeef);
            e.mem.set_u32(array + 4, 1);
            0u32.into_ret()
        });
        let top = e.mem.u32(router);
        e.call(0x00cb_5b10, &args![world, phantom]);
        // The phantom's own slot 0x18 filled the buffer; the broad phase was asked (slot 0x10).
        let fill = calls(&e, OBJECT_SLOT_18);
        assert_eq!(fill.len(), 1);
        let query = calls(&e, OBJECT_SLOT_10);
        assert_eq!(query.len(), 1);
        assert_eq!(query[0][1], phantom + 0x24);
        assert_eq!(query[0][2], fill[0][1]);
        assert_eq!(
            calls(&e, PHANTOM_SET_BOUNDING_VOLUME_DATA),
            vec![vec![phantom, fill[0][1]]]
        );
        let added = calls(&e, DISPATCHER_ADD_PAIRS);
        assert_eq!(
            (added[0][0], added[0][2], added[0][3]),
            (dispatcher, 1, 0x6008)
        );
        assert_eq!(e.mem.u32(router), top);
    }

    #[test]
    fn remove_entity_bp_resets_collision_information_and_removes_pairs() {
        let (mut e, router) = rig();
        stub(&mut e, &[DISPATCHER_REMOVE_PAIRS]);
        let (world, _, dispatcher) = broad_phase_world(&mut e);
        let simulation = object(&mut e, 0x10);
        e.mem.set_u32(world + WORLD_SIMULATION, simulation);
        e.register(OBJECT_SLOT_1C, |e, a| {
            let array = a[2];
            let data = e.mem.u32(array);
            e.mem.set_u32(data, 0xfeed);
            e.mem.set_u32(array + 4, 1);
            0u32.into_ret()
        });
        let entity = e.mem.alloc(0x220);
        // The collision information reset gets the entity slot, 1, the world and 1.
        e.register_double(OBJECT_SLOT_20, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), entity);
            assert_eq!(a[2..], [1, world, 1]);
            0u32.into_ret()
        });
        e.mem.set_u32(entity + 0x10, 0x7777);
        let top = e.mem.u32(router);
        e.call(0x00cb_5c10, &args![world, entity]);
        assert_eq!(calls(&e, OBJECT_SLOT_20).len(), 1);
        let removed = calls(&e, DISPATCHER_REMOVE_PAIRS);
        assert_eq!((removed[0][0], removed[0][2]), (dispatcher, 1));
        assert_eq!(e.mem.u32(router), top);
        // No shape: nothing at all.
        e.call_log = Some(Vec::new());
        e.mem.set_u32(entity + 0x10, 0);
        e.call(0x00cb_5c10, &args![world, entity]);
        assert!(calls(&e, OBJECT_SLOT_20).is_empty());
    }

    /// The stack local the next aligned `size`-byte frame takes, and its 16-byte boundary.
    fn next_aligned_frame(e: &mut Engine, size: u32) -> u32 {
        (e.next_stack_local(size + 0x10) + 0xf) & !0xf
    }

    #[test]
    fn merge_and_update_keep_their_records_at_the_exe_frame_offsets() {
        let (mut e, _) = rig();
        stub(
            &mut e,
            &[
                WORLD_ENQUEUE_OPERATION,
                ENTITY_BATCH_RECALC_AABB,
                WORLD_RUN_PENDING_OPERATIONS,
                WORLD_RUN_PENDING_BODY_OPERATIONS,
                DISPATCHER_REMOVE_DUPLICATES,
                DISPATCHER_REMOVE_PAIRS,
                DISPATCHER_ADD_PAIRS,
            ],
        );
        let (world, _, _) = broad_phase_world(&mut e);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(world + WORLD_LOCK_COUNT, 1);
        // mergeIslands queues its record at +0x8 of its 0x14-byte frame.
        let block = e.next_stack_local(0x14);
        e.call(0x00cb_5570, &args![world, entity, entity]);
        assert_eq!(calls(&e, WORLD_ENQUEUE_OPERATION)[0][1], block + 8);
        // updateEntityBP queues its record at +0x18 of its aligned frame.
        let frame = next_aligned_frame(&mut e, 0x70);
        e.call(0x00cb_5750, &args![world, entity]);
        assert_eq!(calls(&e, WORLD_ENQUEUE_OPERATION)[1][1], frame + 0x18);
        // Unlocked: the handle slot +0x18, the box +0x40, added +0x30, removed +0x20 and the
        // entity's argument slot +0x6c.
        e.mem.set_u32(world + WORLD_LOCK_COUNT, 0);
        e.mem.set_u32(entity + 0x10, 0x7777);
        e.mem.set_u32(entity + 0x30, 2);
        e.mem.set_u32(entity + 0x40, 1);
        let frame = next_aligned_frame(&mut e, 0x70);
        e.call(0x00cb_5750, &args![world, entity]);
        assert_eq!(calls(&e, ENTITY_BATCH_RECALC_AABB)[0][1], frame + 0x6c);
        assert_eq!(
            calls(&e, OBJECT_SLOT_2C)[0][1..],
            [frame + 0x18, frame + 0x40, 1, frame + 0x30, frame + 0x20]
        );
    }

    #[test]
    fn the_bp_updates_keep_their_locals_at_the_exe_frame_offsets() {
        let (mut e, _) = rig();
        stub(
            &mut e,
            &[
                ENTITY_BATCH_RECALC_AABB,
                DISPATCHER_ADD_PAIRS,
                DISPATCHER_REMOVE_PAIRS,
                PHANTOM_SET_BOUNDING_VOLUME_DATA,
            ],
        );
        let (world, _, _) = broad_phase_world(&mut e);
        let simulation = object(&mut e, 0x10);
        e.mem.set_u32(world + WORLD_SIMULATION, simulation);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + 0x10, 0x7777);
        // addEntityBP: the entity's argument slot +0x4c, the AABB +0x20, the pairs +0x10.
        let frame = next_aligned_frame(&mut e, 0x50);
        e.call(0x00cb_59f0, &args![world, entity]);
        assert_eq!(calls(&e, ENTITY_BATCH_RECALC_AABB)[0][1], frame + 0x4c);
        let query = &calls(&e, OBJECT_SLOT_14)[0];
        assert_eq!((query[2], query[3]), (frame + 0x20, frame + 0x10));
        // addPhantomBP: the AABB buffer +0x20, the pairs +0x10.
        let phantom = object(&mut e, 0x40);
        let frame = next_aligned_frame(&mut e, 0x40);
        e.call(0x00cb_5b10, &args![world, phantom]);
        assert_eq!(calls(&e, OBJECT_SLOT_18)[0][1], frame + 0x20);
        assert_eq!(calls(&e, OBJECT_SLOT_10)[0][3], frame + 0x10);
        // removeEntityBP: the entity's argument slot +0x28, the pairs +0x10.
        let block = e.next_stack_local(0x2c);
        e.call(0x00cb_5c10, &args![world, entity]);
        assert_eq!(calls(&e, OBJECT_SLOT_20)[0][1], block + 0x28);
        assert_eq!(e.mem.u32(block + 0x28), entity);
        assert_eq!(calls(&e, OBJECT_SLOT_1C)[0][2], block + 0x10);
        // removePhantomBP: the pairs +0xc.
        e.call_log = Some(Vec::new());
        let block = e.next_stack_local(0x1c);
        e.call(0x00cb_5d00, &args![world, phantom]);
        assert_eq!(calls(&e, OBJECT_SLOT_1C)[0][2], block + 0xc);
    }

    #[test]
    fn remove_phantom_bp_removes_the_pairs() {
        let (mut e, router) = rig();
        stub(&mut e, &[DISPATCHER_REMOVE_PAIRS]);
        let (world, _, dispatcher) = broad_phase_world(&mut e);
        e.register(OBJECT_SLOT_1C, |e, a| {
            let array = a[2];
            let data = e.mem.u32(array);
            e.mem.set_u32(data, 0xfeed);
            e.mem.set_u32(array + 4, 2);
            0u32.into_ret()
        });
        let top = e.mem.u32(router);
        e.call(0x00cb_5d00, &args![world, 0x3000u32]);
        let query = calls(&e, OBJECT_SLOT_1C);
        assert_eq!(query[0][1], 0x3000 + 0x24);
        let removed = calls(&e, DISPATCHER_REMOVE_PAIRS);
        assert_eq!((removed[0][0], removed[0][2]), (dispatcher, 2));
        assert_eq!(e.mem.u32(router), top);
        e.call_log = Some(Vec::new());
        e.register(OBJECT_SLOT_1C, |_, _| 0u32.into_ret());
        e.call(0x00cb_5d00, &args![world, 0x3000u32]);
        assert!(calls(&e, DISPATCHER_REMOVE_PAIRS).is_empty());
    }

    #[test]
    fn fn_00cb5dd0_moves_actions_that_involve_only_fixed_entities() {
        let (mut e, _) = rig();
        stub(&mut e, &[ADD_REFERENCE, ISLAND_REGISTER_ACTION]);
        let world = e.mem.alloc(0x350);
        let island = e.mem.alloc(0x6c);
        e.mem.set_u16(island + ISLAND_DIRTY_INDEX, 0xffff);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + ENTITY_ISLAND, island);
        let fixed = e.mem.alloc(0x220);
        e.mem.set_u8(fixed + ENTITY_MOTION_TYPE, 5);
        let stranger = e.mem.alloc(0x220);
        e.mem.set_u8(stranger + ENTITY_MOTION_TYPE, 1);
        // Action 0 involves the fixed one and this entity; action 1 a stranger.
        let actions = e.mem.alloc(8);
        let action_a = object(&mut e, 0x20);
        let action_b = e.mem.alloc(0x20);
        e.mem.set_u32(action_b, OBJECT_VTABLE);
        e.mem.set_u32(actions, action_a);
        e.mem.set_u32(actions + 4, action_b);
        e.mem.set_u32(entity + ENTITY_ACTIONS, actions);
        e.mem.set_u16(entity + ENTITY_ACTIONS_SIZE, 2);
        e.register_double(OBJECT_SLOT_10, move |e, a| {
            let (list, this) = (a[1], a[0]);
            let data = e.mem.u32(list);
            let members: Vec<u32> = if this == action_a {
                vec![fixed, entity]
            } else {
                vec![stranger]
            };
            for (i, m) in members.iter().enumerate() {
                e.mem.set_u32(data + i as u32 * 4, *m);
            }
            e.mem.set_u32(list + 4, members.len() as u32);
            0u32.into_ret()
        });
        let out = e.mem.alloc(12);
        e.mem.set_u32(out + 8, 0x8000_0000);
        // The exe frame (0x60 bytes) holds the inline array at +0x14.
        let frame = e.next_stack_local(0x60);
        e.call(0x00cb_5dd0, &args![world, entity, out]);
        assert_eq!(calls(&e, OBJECT_SLOT_10)[0][1], frame + 0x14);
        assert_eq!(array_words(&e, out), vec![action_a]);
        assert_eq!(
            calls(&e, ISLAND_REGISTER_ACTION),
            vec![vec![island, action_a]]
        );
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![action_a]]);
        assert_eq!(e.mem.u8(island + ISLAND_FLAGS_25), 0x10);
        assert_eq!(array_words(&e, world + WORLD_DIRTY_ISLANDS), vec![island]);
    }

    #[test]
    fn add_actions_to_entitys_island_merges_other_islands() {
        let (mut e, _) = rig();
        stub(&mut e, &[ISLAND_ADD_ACTION, REMOVE_REFERENCE, 0x00cb_5570]);
        let (world, fixed) = island_world(&mut e);
        let island = e.mem.alloc(0x6c);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + ENTITY_ISLAND, island);
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 1);
        let other = e.mem.alloc(0x220);
        e.mem.set_u32(other + ENTITY_WORLD, world);
        e.mem.set_u32(other + ENTITY_ISLAND, 0x5555);
        e.mem.set_u8(other + ENTITY_MOTION_TYPE, 1);
        let action = object(&mut e, 0x20);
        e.register_double(OBJECT_SLOT_10, move |e, a| {
            let data = e.mem.u32(a[1]);
            e.mem.set_u32(data, entity);
            e.mem.set_u32(data + 4, other);
            e.mem.set_u32(a[1] + 4, 2);
            0u32.into_ret()
        });
        let actions = e.mem.alloc(12);
        word_array(&mut e, actions, &[action]);
        // The exe frame (0x5c bytes) holds the entity array at +0x10.
        let frame = e.next_stack_local(0x5c);
        e.call(0x00cb_5f50, &args![world, entity, actions]);
        assert_eq!(calls(&e, OBJECT_SLOT_10)[0][1], frame + 0x10);
        assert_eq!(calls(&e, ISLAND_ADD_ACTION), vec![vec![island, action]]);
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![action]]);
        assert_eq!(calls(&e, 0x00cb_5570), vec![vec![world, other, entity]]);

        // The fixed island: just added, no entity listing.
        e.call_log = Some(Vec::new());
        e.mem.set_u32(entity + ENTITY_ISLAND, fixed);
        e.call(0x00cb_5f50, &args![world, entity, actions]);
        assert_eq!(calls(&e, ISLAND_ADD_ACTION), vec![vec![fixed, action]]);
        assert!(calls(&e, OBJECT_SLOT_10).is_empty());
    }

    const SET_TYPE_DOUBLES: [u32; 20] = [
        LINKED_COLLIDABLE_SORT_ENTRIES,
        ENTITY_SORT_CONSTRAINTS_SLAVES,
        ADD_REFERENCE,
        REMOVE_REFERENCE,
        0x00cb_4230,
        WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY,
        0x00cb_5000,
        0x00cb_5dd0,
        0x00cb_4790,
        0x00cb_3ee0,
        0x00cb_4740,
        0x00cb_4640,
        0x00cb_5f50,
        0x00cb_3ca0,
        0x00cb_5570,
        AGENT_APPEND_TRACK,
        FREEZE_MOTION_STATE,
        SIMULATION_COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE,
        FIRE_INACTIVE_ENTITY_MOVED,
        WORLD_RUN_PENDING_OPERATIONS,
    ];

    #[test]
    fn set_rigid_body_motion_type_without_leaving_fixedness() {
        let (mut e, _) = rig();
        stub(&mut e, &SET_TYPE_DOUBLES);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 1);
        // Same type: nothing at all.
        e.call(0x00cb_6f70, &args![entity, 1u32, 0u32, 7u32]);
        assert!(e.call_log.as_ref().unwrap().len() == 1);
        // Keyframed to dynamic with no saved motion: nothing happens.
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 4);
        e.call_log = Some(Vec::new());
        e.call(0x00cb_6f70, &args![entity, 1u32, 0u32, 7u32]);
        assert!(calls(&e, ADD_REFERENCE).is_empty());
        // Dynamic to dynamic without a world: motion replaced, held by a reference.
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 1);
        e.call_log = Some(Vec::new());
        e.call(0x00cb_6f70, &args![entity, 2u32, 0u32, 7u32]);
        assert_eq!(calls(&e, 0x00cb_4230), vec![vec![entity, 2, 1, 1, 0]]);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![entity]]);
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![entity]]);
        assert!(calls(&e, WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY).is_empty());
        // With a world, the collision filter is updated.
        let world = e.mem.alloc(0x350);
        e.mem.set_u32(entity + ENTITY_WORLD, world);
        e.call_log = Some(Vec::new());
        e.call(0x00cb_6f70, &args![entity, 3u32, 0u32, 7u32]);
        assert_eq!(
            calls(&e, WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY),
            vec![vec![world, entity, 7, 1]]
        );
        // Leaving fixed sorts the collidable entries and the slave constraints first.
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 5);
        e.mem.set_u32(entity + ENTITY_WORLD, 0);
        e.call_log = Some(Vec::new());
        e.call(0x00cb_6f70, &args![entity, 4u32, 0u32, 7u32]);
        assert_eq!(
            calls(&e, LINKED_COLLIDABLE_SORT_ENTRIES),
            vec![vec![entity + 0x10]]
        );
        assert_eq!(
            calls(&e, ENTITY_SORT_CONSTRAINTS_SLAVES),
            vec![vec![entity]]
        );
    }

    #[test]
    fn set_rigid_body_motion_type_to_fixed_runs_the_whole_sequence() {
        let (mut e, router) = rig();
        stub(&mut e, &SET_TYPE_DOUBLES);
        stub(&mut e, &[WORLD_RUN_PENDING_BODY_OPERATIONS]);
        let simulation = object(&mut e, 0x10);
        let world = e.mem.alloc(0x350);
        e.mem.set_u32(world + WORLD_SIMULATION, simulation);
        e.mem.set_u32(world + WORLD_STEP_TIME, 0.75f32.to_bits());
        e.mem.set_u32(world + WORLD_PENDING_OPERATIONS, 1);
        let island = e.mem.alloc(0x6c);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + ENTITY_WORLD, world);
        e.mem.set_u32(entity + ENTITY_ISLAND, island);
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, 3);
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 1);
        // The constraint collector puts two constraints in the array.
        e.register(0x00cb_5000, |e, a| {
            let array = a[1];
            let data = e.mem.u32(array);
            e.mem.set_u32(data, 0xc1);
            e.mem.set_u32(data + 4, 0xc2);
            e.mem.set_u32(array + 4, 2);
            0u32.into_ret()
        });
        let top = e.mem.u32(router);
        e.call(0x00cb_6f70, &args![entity, 5u32, 1u32, 9u32]);
        assert_eq!(
            order(
                &e,
                &[
                    ADD_REFERENCE,
                    0x00cb_5000,
                    0x00cb_5dd0,
                    0x00cb_3ee0,
                    0x00cb_4740,
                    0x00cb_4230,
                    0x00cb_4640,
                    0x00cb_5f50,
                    0x00cb_3ca0,
                    AGENT_APPEND_TRACK,
                    FREEZE_MOTION_STATE,
                    SIMULATION_COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE,
                    FIRE_INACTIVE_ENTITY_MOVED,
                    WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY,
                    WORLD_RUN_PENDING_OPERATIONS,
                    REMOVE_REFERENCE,
                ]
            ),
            vec![
                ADD_REFERENCE,
                0x00cb_5000,
                0x00cb_5dd0,
                0x00cb_3ee0,
                0x00cb_4740,
                0x00cb_4230,
                0x00cb_4640,
                0x00cb_5f50,
                0x00cb_3ca0,
                REMOVE_REFERENCE,
                0x00cb_3ca0,
                REMOVE_REFERENCE,
                AGENT_APPEND_TRACK,
                FREEZE_MOTION_STATE,
                SIMULATION_COLLIDE_ENTITIES_BROAD_PHASE_DISCRETE,
                FIRE_INACTIVE_ENTITY_MOVED,
                WORLD_UPDATE_COLLISION_FILTER_ON_ENTITY,
                WORLD_RUN_PENDING_OPERATIONS,
                REMOVE_REFERENCE,
            ]
        );
        assert_eq!(
            calls(&e, 0x00cb_3ca0),
            vec![vec![world, 0xc1, 0], vec![world, 0xc2, 0]]
        );
        assert_eq!(calls(&e, 0x00cb_4640), vec![vec![world, entity, 1]]);
        assert_eq!(
            calls(&e, FREEZE_MOTION_STATE),
            vec![vec![0.75f32.to_bits(), entity + 0xf0]]
        );
        // The island with more than 2 entities is flagged before the entity leaves it.
        assert_eq!(e.mem.u8(island + ISLAND_FLAGS_25), 1);
        assert_eq!(e.mem.u32(world + WORLD_LOCK_COUNT), 0);
        assert_eq!(e.mem.u8(world + WORLD_BLOCK_PENDING), 0);
        assert_eq!(e.mem.u8(world + WORLD_CRITICAL_ALLOWED), 1);
        let broadcast = calls(&e, OBJECT_SLOT_20);
        assert_eq!(broadcast.len(), 1);
        assert_eq!(&broadcast[0][2..], &[1, world, 7]);
        assert_eq!(e.mem.u32(router), top);
        // The fixed entity goes through the fixed-island action collector.
        assert!(calls(&e, 0x00cb_4790).is_empty());
    }

    #[test]
    fn set_rigid_body_motion_type_from_fixed_merges_with_the_partners() {
        let (mut e, _) = rig();
        stub(&mut e, &SET_TYPE_DOUBLES);
        let world = e.mem.alloc(0x350);
        let island = e.mem.alloc(0x6c);
        let entity = e.mem.alloc(0x220);
        e.mem.set_u32(entity + ENTITY_WORLD, world);
        e.mem.set_u32(entity + ENTITY_ISLAND, island);
        e.mem.set_u8(entity + ENTITY_MOTION_TYPE, 5);
        e.mem.set_u32(entity + 0x1f8, 0x1234);
        let partner = e.mem.alloc(0x220);
        e.mem.set_u32(partner + ENTITY_ISLAND, 0x7777);
        e.mem.set_u8(partner + ENTITY_MOTION_TYPE, 1);
        e.mem.set_i8(partner + 0x20, -0x10);
        e.register_double(0x00cb_4230, move |e, a| {
            e.mem.set_u8(a[0] + ENTITY_MOTION_TYPE, a[1] as u8);
            0u32.into_ret()
        });
        e.register_double(LINKED_COLLIDABLE_GET_ENTRIES_SORTED, move |e, a| {
            let array = a[1];
            let data = e.mem.alloc(8);
            e.mem.set_u32(data + 4, partner + 0x10);
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 4, 1);
            0u32.into_ret()
        });
        // The entity becomes dynamic: actions via the fixed-island collector, no flagging of
        // the island (it was fixed), a merge with the partner, no freeze.
        e.call(0x00cb_6f70, &args![entity, 1u32, 0u32, 7u32]);
        assert_eq!(calls(&e, 0x00cb_4790).len(), 1);
        assert!(calls(&e, 0x00cb_5dd0).is_empty());
        assert_eq!(e.mem.u8(island + ISLAND_FLAGS_25), 0);
        assert_eq!(calls(&e, 0x00cb_5570), vec![vec![world, entity, partner]]);
        assert!(calls(&e, FREEZE_MOTION_STATE).is_empty());
        assert_eq!(calls(&e, AGENT_APPEND_TRACK).len(), 1);
    }

    #[test]
    fn fn_00cb7460_answers_the_word_or_0x200() {
        let (mut e, _) = rig();
        let this = e.mem.alloc(16);
        e.mem.set_u32(this, 0x77);
        e.mem.set_u32(this + 8, 5);
        assert_eq!(e.call(0x00cb_7460, &args![this, 4u32]).u32(), 0x77);
        assert_eq!(e.call(0x00cb_7460, &args![this, 5u32]).u32(), 0x200);
        // -1 + 1 wraps to 0.
        e.mem.set_u32(this + 8, 0);
        assert_eq!(e.call(0x00cb_7460, &args![this, u32::MAX]).u32(), 0x77);
    }

    #[test]
    fn add_constraint_info_folds_the_counts() {
        let (mut e, _) = rig();
        let owner = e.mem.alloc(0x20);
        let internal = e.mem.alloc(0x20);
        let constraint = e.mem.alloc(0x40);
        let info = e.mem.alloc(0x10);
        for (i, v) in [5u32, 7, 9, 11].iter().enumerate() {
            e.mem.set_u32(info + i as u32 * 4, *v);
        }
        e.mem.set_u32(owner + 8, 3);
        e.mem.set_u32(owner + 0xc, 100);
        e.mem.set_u32(owner + 0x10, 200);
        e.mem.set_u32(owner + 0x14, 300);
        e.mem.set_u16(internal + 0x14, 20);
        e.mem.set_u16(internal + 0x16, 0xfffe);
        e.mem.set_u16(internal + 0x18, 40);
        // No internal block: nothing changes.
        e.call(0x00cb_74d0, &args![owner, constraint, info]);
        assert_eq!(e.mem.u32(owner + 8), 3);
        e.mem.set_u32(constraint + 0x30, internal);
        e.call(0x00cb_74d0, &args![owner, constraint, info]);
        assert_eq!(e.mem.u32(owner + 0xc), 107);
        assert_eq!(e.mem.u32(owner + 0x10), 209);
        assert_eq!(e.mem.u32(owner + 0x14), 311);
        assert_eq!(e.mem.u16(internal + 0x14), 27);
        assert_eq!(e.mem.u16(internal + 0x16), 7);
        assert_eq!(e.mem.u16(internal + 0x18), 51);
        // max(3, 5, 7) = 7, then raised to the new counter 27.
        assert_eq!(e.mem.i32(owner + 8), 27);
        // A counter below the running maximum leaves it.
        e.mem.set_u32(owner + 8, 99);
        e.call(0x00cb_74d0, &args![owner, constraint, info]);
        assert_eq!(e.mem.i32(owner + 8), 99);
    }
}

#[cfg(test)]
mod island_split_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn log_double(e: &mut Engine, log: &Log, address: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret::default()
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A memory router with a stack of 0x1000 bytes; returns the router and the stack base.
    fn router_with_stack(e: &mut Engine) -> (u32, u32) {
        let router = e.mem.alloc(0x20);
        let stack = e.mem.alloc(0x1000);
        e.mem.set_u32(router, stack);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (router, stack)
    }

    #[test]
    fn constraint_added_and_removed_in_a_critical_locked_island() {
        let mut e = engine();
        let log: Log = Rc::default();
        for address in [
            WORLD_CONSTRAINT_UTIL_ADD,
            FIRE_CONSTRAINT_ADDED,
            FIRE_CONSTRAINT_REMOVED,
            WORLD_CONSTRAINT_UTIL_REMOVE,
        ] {
            log_double(&mut e, &log, address);
        }
        e.call(0x00cb_3c80, &args![0x1000u32, 0x2000u32]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![WORLD_CONSTRAINT_UTIL_ADD, FIRE_CONSTRAINT_ADDED]
        );
        assert_eq!(
            calls(&log, WORLD_CONSTRAINT_UTIL_ADD),
            vec![vec![0x1000, 0x2000]]
        );
        assert_eq!(
            calls(&log, FIRE_CONSTRAINT_ADDED),
            vec![vec![0x1000, 0x2000]]
        );
        // No listeners: only the removal itself.
        log.borrow_mut().clear();
        let world = e.mem.alloc(0x200);
        e.call(0x00cb_3da0, &args![world, 0x2000u32]);
        assert!(calls(&log, FIRE_CONSTRAINT_REMOVED).is_empty());
        assert_eq!(
            calls(&log, WORLD_CONSTRAINT_UTIL_REMOVE),
            vec![vec![0x2000]]
        );
        // With listeners the callback runs first.
        log.borrow_mut().clear();
        e.mem.set_u32(world + 0x144, 1);
        e.call(0x00cb_3da0, &args![world, 0x2000u32]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(
            order,
            vec![FIRE_CONSTRAINT_REMOVED, WORLD_CONSTRAINT_UTIL_REMOVE]
        );
        assert_eq!(
            calls(&log, FIRE_CONSTRAINT_REMOVED),
            vec![vec![world, 0x2000]]
        );
    }

    #[test]
    fn island_goes_into_the_dirty_list_once_inside_the_critical_section() {
        let mut e = engine();
        let log: Log = Rc::default();
        log_double(&mut e, &log, ENTER_CRITICAL_SECTION);
        log_double(&mut e, &log, LEAVE_CRITICAL_SECTION);
        let allocator = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        log_double(&mut e, &log, ARRAY_RESERVE_MORE);
        let world = e.mem.alloc(0x100);
        e.mem.set_u32(world + 0xc0, 0x7000);
        let list = e.mem.alloc(16);
        e.mem.set_u32(world + 0x40, list);
        e.mem.set_u32(world + 0x44, 2);
        e.mem.set_u32(world + 0x48, 0x8000_0004);
        let island = e.mem.alloc(0x80);
        e.mem.set_u8(island + 0x26, 0xff);
        e.mem.set_u16(island + 0x22, 0xffff);
        e.call(0x00cb_5480, &args![world, island]);
        assert_eq!(e.mem.u8(island + 0x26), 0xf3);
        assert_eq!(e.mem.u16(island + 0x22), 2);
        assert_eq!(e.mem.u32(world + 0x44), 3);
        assert_eq!(e.mem.u32(list + 8), island);
        assert_eq!(calls(&log, ENTER_CRITICAL_SECTION), vec![vec![0x7000]]);
        assert_eq!(calls(&log, LEAVE_CRITICAL_SECTION), vec![vec![0x7000]]);
        let order: Vec<u32> = log.borrow().iter().map(|(a, _)| *a).collect();
        assert_eq!(order, vec![ENTER_CRITICAL_SECTION, LEAVE_CRITICAL_SECTION]);
        // Already listed: the index stays and nothing is added, but the section is still used.
        e.mem.set_u16(island + 0x22, 1);
        e.call(0x00cb_5480, &args![world, island]);
        assert_eq!(e.mem.u16(island + 0x22), 1);
        assert_eq!(e.mem.u32(world + 0x44), 3);
        assert_eq!(calls(&log, ENTER_CRITICAL_SECTION).len(), 2);
        assert_eq!(calls(&log, LEAVE_CRITICAL_SECTION).len(), 2);
    }

    #[test]
    fn owner_info_is_taken_off_the_owner_and_the_record() {
        let mut e = engine();
        let owner = e.mem.alloc(0x40);
        for (offset, value) in [(0xc, 100u32), (0x10, 200), (0x14, 300)] {
            e.mem.set_u32(owner + offset, value);
        }
        let info = e.mem.alloc(0x20);
        for (offset, value) in [(4, 5u32), (8, 6), (0xc, 7)] {
            e.mem.set_u32(info + offset, value);
        }
        let instance = e.mem.alloc(0x40);
        // Without a record nothing changes.
        e.call(0x00cb_7480, &args![owner, instance, info]);
        assert_eq!(e.mem.u32(owner + 0xc), 100);
        let record = e.mem.alloc(0x40);
        e.mem.set_u16(record + 0x14, 50);
        e.mem.set_u16(record + 0x16, 3);
        e.mem.set_u16(record + 0x18, 60);
        e.mem.set_u32(instance + 0x30, record);
        e.call(0x00cb_7480, &args![owner, instance, info]);
        assert_eq!(e.mem.u32(owner + 0xc), 95);
        assert_eq!(e.mem.u32(owner + 0x10), 194);
        assert_eq!(e.mem.u32(owner + 0x14), 293);
        assert_eq!(e.mem.u16(record + 0x14), 45);
        // 3 - 6 wraps in 16 bits.
        assert_eq!(e.mem.u16(record + 0x16), 0xfffd);
        assert_eq!(e.mem.u16(record + 0x18), 53);
    }

    /// A comparison that is "a is less than b" on the pointer values themselves.
    fn less_double(e: &mut Engine, log: &Log) {
        let log = Rc::clone(log);
        e.register_double(0x0900_0001, move |e, a| {
            log.borrow_mut().push((0x0900_0001, a.to_vec()));
            e.mem.set_u8(a[0], u8::from(a[1] < a[2]));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
    }

    #[test]
    fn quick_sort_orders_the_given_range_only() {
        let mut e = engine();
        let log: Log = Rc::default();
        less_double(&mut e, &log);
        let values = [9u32, 3, 7, 3, 1, 8, 2, 5, 4, 6, 0];
        let array = e.mem.alloc(4 * values.len() as u32);
        for (i, &v) in values.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, v);
        }
        e.call(0x00dd_7d00, &args![array, 1i32, 9i32, 0x0900_0001u32]);
        let got: Vec<u32> = (0..values.len() as u32)
            .map(|i| e.mem.u32(array + 4 * i))
            .collect();
        let mut expected = values.to_vec();
        expected[1..=9].sort_unstable();
        assert_eq!(got, expected);
        // Two comparison slots are used, and every comparison goes through the callback.
        assert!(log.borrow().len() >= 9);
        let slots: Vec<u32> = log.borrow().iter().map(|(_, a)| a[0]).collect();
        assert!(slots.iter().any(|&s| s != slots[0]));
        // A single element and an already sorted run need no change.
        e.call(0x00dd_7d00, &args![array, 4i32, 4i32, 0x0900_0001u32]);
        e.call(0x00dd_7d00, &args![array, 1i32, 9i32, 0x0900_0001u32]);
        let again: Vec<u32> = (0..values.len() as u32)
            .map(|i| e.mem.u32(array + 4 * i))
            .collect();
        assert_eq!(again, expected);
    }

    /// The structures the splitting functions read: a world with the rank bytes, `count`
    /// parents, the entities `(first rank word, second rank word, master count)`, a union-find
    /// whose two-level array maps entity `i` to `groups[i]`, and a flag byte array.
    struct Rig {
        world: u32,
        parents: u32,
        entities: u32,
        union_find: u32,
        flags: u32,
        flag_bytes: u32,
    }

    fn rig(e: &mut Engine, count: u32, entities: &[(u16, u16, u16)], groups: &[u32]) -> Rig {
        let world = e.mem.alloc(0x400);
        e.mem.set_u8(world + 0x305, 1);
        e.mem.set_u8(world + 0x306, 2);
        e.mem.set_u32(world + 0xb4, 30);
        let parents = e.mem.alloc(0x10);
        e.mem.set_u32(parents + 4, count);
        let entity_data = e.mem.alloc(4 * entities.len().max(1) as u32);
        for (i, &(first, second, masters)) in entities.iter().enumerate() {
            let entity = e.mem.alloc(0x100);
            e.mem.set_u16(entity + 0xea, first);
            e.mem.set_u16(entity + 0xec, second);
            e.mem.set_u16(entity + 0xb0, masters);
            e.mem.set_u32(entity_data + 4 * i as u32, entity);
        }
        let entity_array = e.mem.alloc(0x10);
        e.mem.set_u32(entity_array, entity_data);
        e.mem.set_u32(entity_array + 4, entities.len() as u32);
        let group_data = e.mem.alloc(4 * groups.len().max(1) as u32);
        for (i, &g) in groups.iter().enumerate() {
            e.mem.set_u32(group_data + 4 * i as u32, g);
        }
        let inner = e.mem.alloc(0x10);
        e.mem.set_u32(inner, group_data);
        let union_find = e.mem.alloc(0x10);
        e.mem.set_u32(union_find, inner);
        let flag_bytes = e.mem.alloc(0x20);
        for i in 0..0x20 {
            e.mem.set_u8(flag_bytes + i, 0x77);
        }
        let flags = e.mem.alloc(8);
        e.mem.set_u32(flags, flag_bytes);
        Rig {
            world,
            parents,
            entities: entity_array,
            union_find,
            flags,
            flag_bytes,
        }
    }

    /// The reindex double: records the marks array (`count` words) it is given through the
    /// slot, the group count and the parents.
    type Seen = Rc<RefCell<Vec<(Vec<i32>, u32, u32)>>>;

    fn reindex_double(e: &mut Engine, count: u32, seen: &Seen) {
        let seen = Rc::clone(seen);
        e.register_double(UNION_FIND_REINDEX, move |e, a| {
            let marks = e.mem.u32(a[1]);
            let words = (0..count)
                .map(|i| e.mem.i32(marks + 4 * i))
                .collect::<Vec<_>>();
            seen.borrow_mut().push((words, a[2], a[3]));
            Ret::default()
        });
    }

    // Entities `(word at +0xEA, word at +0xEC, master count)` with the ranks 4 (0x4185 is
    // `1 << 14 | 3 << 7 | 5` and 0x8209 is `2 << 14 | 4 << 7 | 9`), 32 and 3.
    const ENTITIES: [(u16, u16, u16); 3] = [(0x4185, 0x8209, 2), (0x0020, 0, 0), (0, 3, 1)];

    #[test]
    fn marking_numbers_the_marked_groups_and_reindexes() {
        let mut e = engine();
        let (router, stack) = router_with_stack(&mut e);
        let seen: Seen = Rc::default();
        reindex_double(&mut e, 3, &seen);
        let rig = rig(&mut e, 3, &ENTITIES, &[2, 0, 1]);
        e.call(
            0x00cb_4890,
            &args![
                rig.entities,
                rig.union_find,
                rig.world,
                rig.parents,
                rig.flags
            ],
        );
        // Entities 0 and 2 (groups 2 and 1) have a rank of at most 5; element 0 is unmarked
        // and gets number 1, the marked ones are cleared.
        assert_eq!(*seen.borrow(), vec![(vec![1, 0, 0], 2, rig.parents)]);
        assert_eq!(e.mem.u8(rig.flag_bytes), 1);
        assert_eq!(e.mem.u8(rig.flag_bytes + 1), 0);
        assert_eq!(e.mem.u8(rig.flag_bytes + 2), 0x77);
        // The router's top is back where it was.
        assert_eq!(e.mem.u32(router), stack);
    }

    #[test]
    fn marking_with_nothing_marked_gives_up_without_reindexing() {
        let mut e = engine();
        let (router, stack) = router_with_stack(&mut e);
        let seen: Seen = Rc::default();
        reindex_double(&mut e, 2, &seen);
        let rig = rig(&mut e, 2, &[], &[]);
        e.mem.set_u8(rig.flag_bytes, 9);
        e.call(
            0x00cb_4890,
            &args![
                rig.entities,
                rig.union_find,
                rig.world,
                rig.parents,
                rig.flags
            ],
        );
        assert!(seen.borrow().is_empty());
        assert_eq!(e.mem.u8(rig.flag_bytes), 0);
        assert_eq!(e.mem.u8(rig.flag_bytes + 1), 0);
        assert_eq!(e.mem.u8(rig.flag_bytes + 2), 0);
        assert_eq!(e.mem.u32(router), stack);
    }

    #[test]
    fn splitting_merges_light_groups_without_ranking() {
        let mut e = engine();
        let (router, stack) = router_with_stack(&mut e);
        let seen: Seen = Rc::default();
        reindex_double(&mut e, 3, &seen);
        let rig = rig(&mut e, 3, &ENTITIES, &[0, 1, 2]);
        // The byte at +0xD5 is clear: every entity marks its group, weights are 2, 1 and 1.
        e.call(
            0x00cb_49e0,
            &args![
                rig.world,
                rig.union_find,
                rig.entities,
                rig.parents,
                rig.flags
            ],
        );
        // One group takes all three (total 4 stays below the limit of 30): marks 0, 0, 0.
        assert_eq!(*seen.borrow(), vec![(vec![0, 0, 0], 1, rig.parents)]);
        assert_eq!(e.mem.u8(rig.flag_bytes), 1);
        assert_eq!(e.mem.u8(rig.flag_bytes + 1), 0x77);
        assert_eq!(e.mem.u32(router), stack);
    }

    #[test]
    fn splitting_with_ranks_leaves_unmarked_elements_apart() {
        let mut e = engine();
        let (router, stack) = router_with_stack(&mut e);
        let seen: Seen = Rc::default();
        reindex_double(&mut e, 3, &seen);
        let rig = rig(&mut e, 3, &ENTITIES, &[0, 1, 2]);
        e.mem.set_u8(rig.world + 0xd5, 1);
        e.call(
            0x00cb_49e0,
            &args![
                rig.world,
                rig.union_find,
                rig.entities,
                rig.parents,
                rig.flags
            ],
        );
        // Weights are 2, -1 (the middle entity's rank is above 5, so nothing marks its group)
        // and 1; the first element takes the third (total 3), so groups 0 and 1 remain:
        // marks 0, 1, 0.
        assert_eq!(*seen.borrow(), vec![(vec![0, 1, 0], 2, rig.parents)]);
        assert_eq!(e.mem.u8(rig.flag_bytes), 1);
        assert_eq!(e.mem.u8(rig.flag_bytes + 1), 0);
        assert_eq!(e.mem.u32(router), stack);
    }

    #[test]
    fn splitting_stops_merging_when_the_total_reaches_the_limit() {
        let mut e = engine();
        let (router, stack) = router_with_stack(&mut e);
        let seen: Seen = Rc::default();
        reindex_double(&mut e, 3, &seen);
        let rig = rig(&mut e, 3, &ENTITIES, &[0, 1, 2]);
        // With a limit of 11 even the smallest total (1 + 10) is not below it: nothing merges
        // and every element is its own group.
        e.mem.set_u32(rig.world + 0xb4, 11);
        e.call(
            0x00cb_49e0,
            &args![
                rig.world,
                rig.union_find,
                rig.entities,
                rig.parents,
                rig.flags
            ],
        );
        assert_eq!(*seen.borrow(), vec![(vec![0, 1, 2], 3, rig.parents)]);
        assert_eq!(e.mem.u8(rig.flag_bytes), 0);
        assert_eq!(e.mem.u8(rig.flag_bytes + 1), 0);
        assert_eq!(e.mem.u8(rig.flag_bytes + 2), 0);
        assert_eq!(e.mem.u32(router), stack);
    }
}

#[cfg(test)]
mod island_state_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const SLOT_40: u32 = 0x0300_0040;
    const SLOT_44: u32 = 0x0300_0044;
    const MOTION_VTABLE: u32 = 0x0300_0100;
    const ISLAND_VTABLE: u32 = 0x0300_0200;
    const ISLAND_DESTRUCTOR: u32 = 0x0300_0300;

    /// An engine whose memory router answers a block with an allocator word, whose array
    /// growers reallocate, and whose other callees only record their words.
    fn rig(recorded: &[u32]) -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let router = e.mem.alloc(0x20);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| router.into_ret());
        e.register(ARRAY_RESERVE_MORE, |e, a| grow(e, a[1], 8));
        e.register(ARRAY_RESERVE_EXACT, |e, a| grow(e, a[1], a[2]));
        for &address in recorded {
            let log = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        e.put_vtable(MOTION_VTABLE, &[0; 0x12]);
        e.mem.set_u32(MOTION_VTABLE + 0x40, SLOT_40);
        e.mem.set_u32(MOTION_VTABLE + 0x44, SLOT_44);
        for slot in [SLOT_40, SLOT_44] {
            let log = Rc::clone(&log);
            e.register_double(slot, move |_, a| {
                log.borrow_mut().push((slot, a.to_vec()));
                Ret::default()
            });
        }
        e.put_vtable(ISLAND_VTABLE, &[ISLAND_DESTRUCTOR]);
        let seen = Rc::clone(&log);
        e.register_double(ISLAND_DESTRUCTOR, move |_, a| {
            seen.borrow_mut().push((ISLAND_DESTRUCTOR, a.to_vec()));
            Ret::default()
        });
        (e, log)
    }

    /// The array grower: a fresh block of `capacity` words with the old words copied.
    fn grow(e: &mut Engine, array: u32, capacity: u32) -> Ret {
        let size = e.mem.u32(array + 4);
        let fresh = e.mem.alloc(capacity * 4);
        let old = e.mem.bytes(e.mem.u32(array), size * 4);
        e.mem.write(fresh, &old);
        e.mem.set_u32(array, fresh);
        e.mem.set_u32(array + 8, capacity);
        Ret::default()
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An `hkArray` header at `header` holding `values`, with room for `capacity` words.
    fn array(e: &mut Engine, header: u32, values: &[u32], capacity: u32) {
        let data = e.mem.alloc(capacity * 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *v);
        }
        e.mem.set_u32(header, data);
        e.mem.set_u32(header + 4, values.len() as u32);
        e.mem.set_u32(header + 8, capacity);
    }

    fn words(e: &Engine, header: u32) -> Vec<u32> {
        let data = e.mem.u32(header);
        (0..e.mem.u32(header + 4))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    fn world(e: &mut Engine) -> u32 {
        e.mem.alloc(0x400)
    }

    /// An island with `entities` and its list index.
    fn island(e: &mut Engine, index: u16, entities: &[u32]) -> u32 {
        let island = e.mem.alloc(0x80);
        e.mem.set_u32(island, ISLAND_VTABLE);
        e.mem.set_u16(island + ISLAND_INDEX, index);
        e.mem.set_u16(island + ISLAND_DIRTY_INDEX, 0xffff);
        array(e, island + ISLAND_ENTITIES, entities, 8);
        e.mem
            .set_u32(island + ISLAND_ENTITY_COUNT, entities.len() as u32);
        island
    }

    fn entity(e: &mut Engine, island: u32) -> u32 {
        let entity = e.mem.alloc(0x120);
        e.mem.set_u32(entity + ENTITY_ISLAND, island);
        e.mem.set_u32(entity + ENTITY_MOTION, MOTION_VTABLE);
        entity
    }

    const ACTIVATION_CALLEES: [u32; 6] = [
        SET_TIME_INFORMATION,
        WARP_TIME,
        ADD_REFERENCE,
        REMOVE_REFERENCE,
        FIRE_ISLAND_ACTIVATED,
        ISLAND_CHECK_00D07120,
    ];

    /// A world with inactive islands `[other, target]` and one active island, `target` having
    /// two entities against the active one's one.
    fn activation_scene(e: &mut Engine) -> (u32, [u32; 3], [u32; 2]) {
        let w = world(e);
        let (other, target, active) = (island(e, 0, &[]), island(e, 1, &[]), island(e, 0, &[]));
        let first = entity(e, target);
        let second = entity(e, target);
        array(e, target + ISLAND_ENTITIES, &[first, second], 8);
        e.mem.set_u32(target + ISLAND_ENTITY_COUNT, 2);
        let sole = entity(e, active);
        array(e, active + ISLAND_ENTITIES, &[sole], 8);
        e.mem.set_u32(active + ISLAND_ENTITY_COUNT, 1);
        array(e, w + WORLD_INACTIVE_ISLANDS, &[other, target], 4);
        array(e, w + WORLD_ACTIVE_ISLANDS, &[active], 4);
        e.mem.set_u8(w + WORLD_RANK_BYTE_FIRST, 1);
        e.mem.set_u8(w + WORLD_RANK_BYTE_SECOND, 2);
        e.mem.set_u8(w + WORLD_TAG_BYTE_307, 1);
        e.mem.set_u8(first + ENTITY_TAG_BYTE_E9, 3);
        e.mem.set_f32(target + ISLAND_TIME, 1.5);
        e.mem.set_f32(w + WORLD_STEP_TIME, 0.25);
        e.mem.set_u32(w + WORLD_COLLISION_INPUT, 0xc0ffee);
        e.mem.set_u32(target + 0x30, 7);
        e.mem.set_u32(target + 0x34, 7);
        (w, [other, target, active], [first, second])
    }

    #[test]
    fn activating_moves_the_island_tags_the_entities_and_fronts_a_larger_island() {
        let (mut e, log) = rig(&ACTIVATION_CALLEES);
        let (w, [other, target, active], [first, second]) = activation_scene(&mut e);
        e.call(0x00cb_5100, &args![w, target, 0u32]);
        assert_eq!(words(&e, w + WORLD_INACTIVE_ISLANDS), vec![other]);
        // Appended at index 1, then moved to the front over the smaller island.
        assert_eq!(words(&e, w + WORLD_ACTIVE_ISLANDS), vec![target, active]);
        assert_eq!(e.mem.u16(target + ISLAND_INDEX), 0);
        assert_eq!(e.mem.u16(active + ISLAND_INDEX), 1);
        assert_eq!(e.mem.u8(target + ISLAND_FLAGS_26), 1);
        assert_eq!((e.mem.u32(target + 0x30), e.mem.u32(target + 0x34)), (0, 0));
        // World bytes 1, 2, 1: the first entity (byte 3) gets both inverted, the second not.
        assert_eq!(e.mem.u16(first + ENTITY_RANK_WORD_FIRST), 0x8000);
        assert_eq!(e.mem.u16(first + ENTITY_RANK_WORD_SECOND), 0x4000);
        assert_eq!(e.mem.u16(second + ENTITY_RANK_WORD_FIRST), 0x4000);
        assert_eq!(e.mem.u16(second + ENTITY_RANK_WORD_SECOND), 0x8000);
        assert_eq!(
            calls(&log, SET_TIME_INFORMATION),
            vec![
                vec![0, 0, first + ENTITY_MOTION_STATE],
                vec![0, 0, second + ENTITY_MOTION_STATE]
            ]
        );
        assert_eq!(
            calls(&log, WARP_TIME),
            vec![vec![target, 1.5f32.to_bits(), 0.25f32.to_bits(), 0xc0ffee]]
        );
        assert_eq!(calls(&log, ADD_REFERENCE), vec![vec![first]]);
        assert_eq!(calls(&log, FIRE_ISLAND_ACTIVATED), vec![vec![w, target]]);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![first]]);
    }

    #[test]
    fn activating_stops_when_the_first_entity_left_the_island() {
        let (mut e, log) = rig(&ACTIVATION_CALLEES);
        let (w, [_, target, active], [first, _]) = activation_scene(&mut e);
        e.mem.set_u32(first + ENTITY_ISLAND, 0x1234);
        e.call(0x00cb_5100, &args![w, target, 1u32]);
        assert_eq!(words(&e, w + WORLD_ACTIVE_ISLANDS), vec![active, target]);
        assert_eq!(e.mem.u16(target + ISLAND_INDEX), 1);
        assert_eq!(calls(&log, REMOVE_REFERENCE), vec![vec![first]]);
    }

    #[test]
    fn activating_with_the_flag_merges_with_a_small_sparse_island() {
        let (mut e, log) = rig(&[
            SET_TIME_INFORMATION,
            WARP_TIME,
            ADD_REFERENCE,
            REMOVE_REFERENCE,
            FIRE_ISLAND_ACTIVATED,
            AGENT_APPEND_TRACK,
            WORLD_AFTER_MERGE,
        ]);
        let (w, [_, target, active], _) = activation_scene(&mut e);
        // The one active island is small and marked, and the world manages islands.
        e.mem.set_u32(w + WORLD_MIN_ISLAND_SIZE, 10);
        e.mem.set_u8(w + WORLD_WANT_ISLANDS, 1);
        e.mem.set_u8(active + ISLAND_FLAGS_26, 5);
        e.mem.set_u8(active + ISLAND_FLAGS_25, 0);
        e.call(0x00cb_5100, &args![w, target, 0x100u32]);
        // Only the low byte is the flag: 0x100 does not merge.
        assert_eq!(words(&e, w + WORLD_ACTIVE_ISLANDS), vec![target, active]);
        assert!(calls(&log, ISLAND_DESTRUCTOR).is_empty());
        let (w, [_, target, active], _) = activation_scene(&mut e);
        e.mem.set_u32(w + WORLD_MIN_ISLAND_SIZE, 10);
        e.mem.set_u8(w + WORLD_WANT_ISLANDS, 1);
        e.mem.set_u8(active + ISLAND_FLAGS_26, 5);
        e.call(0x00cb_5100, &args![w, target, 1u32]);
        // The two islands merged: the one with more entities (the target) is kept and the
        // other destroyed.
        assert_eq!(calls(&log, ISLAND_DESTRUCTOR), vec![vec![active, 1]]);
        assert_eq!(words(&e, w + WORLD_ACTIVE_ISLANDS), vec![target]);
        assert_eq!(e.mem.u32(target + ISLAND_ENTITY_COUNT), 3);
    }

    #[test]
    fn deactivating_needs_the_entities_check_to_pass() {
        let (mut e, log) = rig(&[FIRE_ISLAND_DEACTIVATED, FREEZE_MOTION_STATE, ENTITIES_CHECK]);
        let w = world(&mut e);
        let (resting, other) = (island(&mut e, 1, &[]), island(&mut e, 0, &[]));
        let (first, second) = (entity(&mut e, resting), entity(&mut e, resting));
        array(&mut e, resting + ISLAND_ENTITIES, &[first, second], 4);
        e.mem.set_u32(resting + ISLAND_ENTITY_COUNT, 2);
        e.mem.set_u8(resting + ISLAND_FLAGS_26, 0b1011);
        array(&mut e, w + WORLD_ACTIVE_ISLANDS, &[other, resting], 4);
        array(&mut e, w + WORLD_INACTIVE_ISLANDS, &[0x1111], 4);
        e.mem.set_f32(w + WORLD_STEP_TIME, 2.5);
        // The check fails: only the flag byte changes.
        e.register_double(ENTITIES_CHECK, |_, _| 0u32.into_ret());
        e.call(0x00cb_5310, &args![w, resting]);
        assert_eq!(e.mem.u8(resting + ISLAND_FLAGS_26), 0b0111);
        assert_eq!(words(&e, w + WORLD_ACTIVE_ISLANDS), vec![other, resting]);
        assert!(calls(&log, FIRE_ISLAND_DEACTIVATED).is_empty());
        // The check passes.
        let seen = Rc::clone(&log);
        e.register_double(ENTITIES_CHECK, move |_, a| {
            seen.borrow_mut().push((ENTITIES_CHECK, a.to_vec()));
            1u32.into_ret()
        });
        e.mem.set_u8(resting + ISLAND_FLAGS_26, 0b1011);
        e.call(0x00cb_5310, &args![w, resting]);
        let entities = e.mem.u32(resting + ISLAND_ENTITIES);
        assert_eq!(
            calls(&log, ENTITIES_CHECK)[0],
            vec![w + WORLD_STEP_TIME, entities, 2, 0xe0]
        );
        assert_eq!(words(&e, w + WORLD_ACTIVE_ISLANDS), vec![other]);
        assert_eq!(words(&e, w + WORLD_INACTIVE_ISLANDS), vec![0x1111, resting]);
        assert_eq!(e.mem.u16(resting + ISLAND_INDEX), 1);
        assert_eq!(e.mem.u8(resting + ISLAND_FLAGS_26), 0b1000);
        assert_eq!(e.mem.f32(resting + ISLAND_TIME), 2.5);
        assert_eq!(
            calls(&log, FREEZE_MOTION_STATE),
            vec![
                vec![2.5f32.to_bits(), first + ENTITY_MOTION_STATE],
                vec![2.5f32.to_bits(), second + ENTITY_MOTION_STATE]
            ]
        );
        assert_eq!(
            calls(&log, SLOT_40),
            vec![
                vec![first + ENTITY_MOTION, MOTION_DEACTIVATION_ARGUMENT],
                vec![second + ENTITY_MOTION, MOTION_DEACTIVATION_ARGUMENT]
            ]
        );
        assert_eq!(calls(&log, SLOT_44).len(), 2);
        assert_eq!(calls(&log, FIRE_ISLAND_DEACTIVATED), vec![vec![w, resting]]);
    }

    #[test]
    fn marking_an_island_inactive_puts_it_in_the_dirty_list_once() {
        let (mut e, _) = rig(&[]);
        let w = world(&mut e);
        array(&mut e, w + WORLD_DIRTY_ISLANDS, &[0x2222], 4);
        let island = island(&mut e, 0, &[]);
        e.mem.set_u8(island + ISLAND_FLAGS_26, 0xff);
        e.call(0x00cb_5420, &args![w, island]);
        assert_eq!(e.mem.u8(island + ISLAND_FLAGS_26), 0xf3);
        assert_eq!(e.mem.u16(island + ISLAND_DIRTY_INDEX), 1);
        assert_eq!(words(&e, w + WORLD_DIRTY_ISLANDS), vec![0x2222, island]);
        e.call(0x00cb_5420, &args![w, island]);
        assert_eq!(words(&e, w + WORLD_DIRTY_ISLANDS), vec![0x2222, island]);
    }

    #[test]
    fn cleaning_up_sorts_squeezes_and_activates_or_deactivates() {
        let (mut e, log) = rig(&[
            WORLD_CHECK_DETERMINISM,
            FIRE_ISLAND_DEACTIVATED,
            ENTITIES_CHECK,
            SET_TIME_INFORMATION,
            WARP_TIME,
            ADD_REFERENCE,
            REMOVE_REFERENCE,
            FIRE_ISLAND_ACTIVATED,
        ]);
        // `a < b` as an address compare, answered through the result slot.
        e.register(DIRTY_ISLAND_LESS, |e, a| {
            e.mem.set_u8(a[0], (a[1] < a[2]) as u8);
            a[0].into_ret()
        });
        let w = world(&mut e);
        let to_deactivate = island(&mut e, 0, &[]);
        let to_activate = island(&mut e, 0, &[]);
        let probe = entity(&mut e, 0x77);
        array(&mut e, to_activate + ISLAND_ENTITIES, &[probe], 4);
        array(
            &mut e,
            w + WORLD_DIRTY_ISLANDS,
            &[to_activate, to_deactivate],
            4,
        );
        // Active state 1 wanted 0 for the first; wanted 1, state 0 for the second.
        e.mem.set_u8(to_deactivate + ISLAND_FLAGS_26, 0b0001);
        e.mem.set_u8(to_activate + ISLAND_FLAGS_26, 0b0100);
        array(&mut e, w + WORLD_ACTIVE_ISLANDS, &[to_deactivate], 4);
        array(&mut e, w + WORLD_INACTIVE_ISLANDS, &[to_activate], 4);
        e.mem.set_u16(to_activate + ISLAND_INDEX, 0);
        // The squeeze: null words fall out of the island's array at +0x38.
        e.mem.set_u8(to_deactivate + ISLAND_FLAGS_25, 0x10);
        array(
            &mut e,
            to_deactivate + ISLAND_ARRAY_38,
            &[0, 0xa1, 0, 0xb2],
            4,
        );
        e.register_double(ENTITIES_CHECK, |_, _| 0u32.into_ret());
        e.call(0x00cb_55d0, &args![w]);
        assert_eq!(e.mem.u32(w + WORLD_DIRTY_ISLANDS + 4), 0);
        assert_eq!(words(&e, to_deactivate + ISLAND_ARRAY_38), vec![0xa1, 0xb2]);
        assert_eq!(e.mem.u8(to_deactivate + ISLAND_FLAGS_25), 0);
        // The sort put them in address order, so the later island came off the list first;
        // both lost their dirty index.
        assert_eq!(e.mem.u16(to_activate + ISLAND_DIRTY_INDEX), 0xffff);
        assert_eq!(e.mem.u16(to_deactivate + ISLAND_DIRTY_INDEX), 0xffff);
        // Activated: now on the active list. Deactivation refused (check false): flag byte.
        assert_eq!(
            words(&e, w + WORLD_ACTIVE_ISLANDS),
            vec![to_deactivate, to_activate]
        );
        assert_eq!(e.mem.u8(to_deactivate + ISLAND_FLAGS_26), 0b0101);
        assert_eq!(
            calls(&log, FIRE_ISLAND_ACTIVATED),
            vec![vec![w, to_activate]]
        );
        assert_eq!(calls(&log, WORLD_CHECK_DETERMINISM), vec![vec![w]]);
        // Nothing dirty: nothing happens at all, not even the determinism check.
        e.call(0x00cb_55d0, &args![w]);
        assert_eq!(calls(&log, WORLD_CHECK_DETERMINISM).len(), 1);
    }

    #[test]
    fn cleaning_up_merges_an_island_with_equal_state_bits_set() {
        let (mut e, log) = rig(&[WORLD_CHECK_DETERMINISM, INTERNAL_MERGE_TWO_ISLANDS]);
        let w = world(&mut e);
        let (small, mine) = (island(&mut e, 0, &[]), island(&mut e, 1, &[]));
        let (small_entity, my_entity) = (entity(&mut e, small), entity(&mut e, mine));
        array(&mut e, small + ISLAND_ENTITIES, &[small_entity], 4);
        array(&mut e, mine + ISLAND_ENTITIES, &[my_entity], 4);
        e.mem.set_u32(mine + ISLAND_ENTITY_COUNT, 1);
        array(&mut e, w + WORLD_ACTIVE_ISLANDS, &[small, mine], 4);
        array(&mut e, w + WORLD_DIRTY_ISLANDS, &[mine], 4);
        e.mem.set_u32(w + WORLD_MIN_ISLAND_SIZE, 10);
        e.mem.set_u8(w + WORLD_WANT_ISLANDS, 1);
        e.mem.set_u8(small + ISLAND_FLAGS_26, 4);
        // Both state fields equal (1, 1) with bit 4 set: cleared, then merged.
        e.mem.set_u8(mine + ISLAND_FLAGS_26, 0b0001_0101);
        e.call(0x00cb_55d0, &args![w]);
        assert_eq!(
            calls(&log, INTERNAL_MERGE_TWO_ISLANDS),
            vec![vec![w, mine, small]]
        );

        assert_eq!(e.mem.u8(mine + ISLAND_FLAGS_26), 0b0000_0101);
        assert_eq!(calls(&log, WORLD_CHECK_DETERMINISM).len(), 1);
    }

    /// Two inactive islands: `kept` with two entities and `gone` with one that has two master
    /// constraints, `gone` in the dirty list.
    fn merge_scene(e: &mut Engine) -> (u32, [u32; 2], [u32; 3], [u32; 2]) {
        let w = world(e);
        let (kept, gone) = (island(e, 0, &[]), island(e, 1, &[]));
        let kept_entities = [entity(e, kept), entity(e, kept)];
        let gone_entity = entity(e, gone);
        array(e, kept + ISLAND_ENTITIES, &kept_entities, 4);
        e.mem.set_u32(kept + ISLAND_ENTITY_COUNT, 2);
        array(e, gone + ISLAND_ENTITIES, &[gone_entity], 4);
        e.mem.set_u32(gone + ISLAND_ENTITY_COUNT, 1);
        let instances = [e.mem.alloc(0x20), e.mem.alloc(0x20)];
        let masters = e.mem.alloc(0x60);
        e.mem.set_u32(masters, instances[0]);
        e.mem.set_u32(masters + 0x30, instances[1]);
        e.mem.set_u32(gone_entity + ENTITY_MASTERS, masters);
        e.mem.set_u16(gone_entity + ENTITY_MASTERS_COUNT, 2);
        let owners = [e.mem.alloc(0x20), e.mem.alloc(0x20), e.mem.alloc(0x20)];
        array(e, kept + ISLAND_ARRAY_38, &[owners[0]], 4);
        array(e, gone + ISLAND_ARRAY_38, &[owners[1], 0, owners[2]], 4);
        array(e, w + WORLD_INACTIVE_ISLANDS, &[kept, gone], 4);
        array(e, w + WORLD_DIRTY_ISLANDS, &[gone], 4);
        e.mem.set_u16(gone + ISLAND_DIRTY_INDEX, 0);
        e.mem.set_u32(kept + ISLAND_CONSTRAINT_COUNT, 5);
        e.mem.set_u32(gone + ISLAND_CONSTRAINT_COUNT, 3);
        e.mem.set_u8(gone + ISLAND_FLAGS_25, 0x0c | 0x10);
        e.mem.set_u8(gone + ISLAND_FLAGS_26, 0x04);
        (
            w,
            [kept, gone],
            [kept_entities[0], kept_entities[1], gone_entity],
            instances,
        )
    }

    const MERGE_CALLEES: [u32; 2] = [AGENT_APPEND_TRACK, WORLD_AFTER_MERGE];

    #[test]
    fn merging_moves_the_entities_words_and_constraints_into_the_bigger_island() {
        let (mut e, log) = rig(&MERGE_CALLEES);
        let (w, [kept, gone], [_, _, moved], instances) = merge_scene(&mut e);
        let owners = words(&e, gone + ISLAND_ARRAY_38);
        e.call(0x00cb_4c60, &args![w, gone, kept]);
        // Entities: the gone island's follow the kept ones, with their island and index.
        assert_eq!(e.mem.u32(kept + ISLAND_ENTITY_COUNT), 3);
        let entities = e.mem.u32(kept + ISLAND_ENTITIES);
        assert_eq!(e.mem.u32(entities + 8), moved);
        assert_eq!(e.mem.u32(moved + ENTITY_ISLAND), kept);
        assert_eq!(e.mem.u16(moved + ENTITY_INDEX_IN_ISLAND), 2);
        // The words array: non-null words only, each pointing at the kept island.
        assert_eq!(words(&e, kept + ISLAND_ARRAY_38).len(), 3);
        for owner in [owners[0], owners[2]] {
            assert_eq!(e.mem.u32(owner + 0xc), kept);
        }
        for instance in instances {
            assert_eq!(e.mem.u32(instance + 8), kept);
        }
        assert_eq!(e.mem.u32(kept + ISLAND_CONSTRAINT_COUNT), 8);
        assert_eq!(e.mem.u32(gone + ISLAND_CONSTRAINT_COUNT), 0);
        // The gone island left the inactive list, was destroyed, and the kept one took its
        // place in the dirty list.
        assert_eq!(words(&e, w + WORLD_INACTIVE_ISLANDS), vec![kept]);
        assert_eq!(calls(&log, ISLAND_DESTRUCTOR), vec![vec![gone, 1]]);
        assert_eq!(e.mem.u16(kept + ISLAND_DIRTY_INDEX), 1);
        assert_eq!(
            calls(&log, AGENT_APPEND_TRACK),
            vec![vec![kept + ISLAND_AGENT_TRACK, gone + ISLAND_AGENT_TRACK]]
        );
        // Flag bits: marked (bit 2) and the 0x30 group carried over, the lock released.
        assert_eq!(e.mem.u8(kept + ISLAND_FLAGS_26) & 0xc, 0x4);
        assert_eq!(e.mem.u8(kept + ISLAND_FLAGS_25) & 0x30, 0x10);
        assert_eq!(e.mem.u32(w + WORLD_LOCK_COUNT), 0);
        assert_eq!(calls(&log, WORLD_AFTER_MERGE), vec![vec![w, kept]]);
    }

    #[test]
    fn merging_equal_islands_keeps_the_one_with_the_lower_index() {
        let (mut e, log) = rig(&MERGE_CALLEES);
        let (w, [kept, gone], _, _) = merge_scene(&mut e);
        // Equal entity counts: the lower list index (the first island) survives.
        e.mem.set_u32(kept + ISLAND_ENTITY_COUNT, 1);
        e.call(0x00cb_4c60, &args![w, gone, kept]);
        assert_eq!(calls(&log, ISLAND_DESTRUCTOR), vec![vec![gone, 1]]);
    }

    #[test]
    fn merging_activates_the_inactive_island_first_when_the_other_is_active() {
        let (mut e, log) = rig(&[
            AGENT_APPEND_TRACK,
            WORLD_AFTER_MERGE,
            SET_TIME_INFORMATION,
            WARP_TIME,
            ADD_REFERENCE,
            REMOVE_REFERENCE,
            FIRE_ISLAND_ACTIVATED,
        ]);
        let (w, [kept, gone], [first, ..], _) = merge_scene(&mut e);
        // The gone island is active (in the active list); the kept one is not.
        e.mem.set_u8(gone + ISLAND_FLAGS_26, 1);
        array(&mut e, w + WORLD_ACTIVE_ISLANDS, &[gone], 4);
        array(&mut e, w + WORLD_INACTIVE_ISLANDS, &[kept], 4);
        e.mem.set_u16(gone + ISLAND_INDEX, 0);
        e.mem.set_u16(kept + ISLAND_INDEX, 0);
        e.mem.set_u32(first + ENTITY_ISLAND, 0x1234);
        e.call(0x00cb_4c60, &args![w, kept, gone]);
        assert_eq!(calls(&log, FIRE_ISLAND_ACTIVATED), vec![vec![w, kept]]);
        assert_eq!(calls(&log, ISLAND_DESTRUCTOR), vec![vec![gone, 1]]);
        // The kept island is active now, alone in the active list.
        assert_eq!(words(&e, w + WORLD_ACTIVE_ISLANDS), vec![kept]);
        assert_eq!(e.mem.u8(kept + ISLAND_FLAGS_26) & 3, 1);
    }
}

#[cfg(test)]
mod split_flow_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const ALLOCATOR_VTABLE: u32 = 0x0300_0000;
    const ALLOCATE_BLOCK: u32 = 0x0300_0100;
    const FREE_BLOCK: u32 = 0x0300_0110;
    const FREE_ARRAY: u32 = 0x0300_0120;
    const CONSTRAINT_VTABLE: u32 = 0x0300_0200;
    const CONSTRAINT_ENTITIES: u32 = 0x0300_0210;
    const NEW_ISLAND_BYTES: u32 = 0x70;

    struct Scene {
        e: Engine,
        log: Log,
        router: u32,
        stack_top: u32,
        world: u32,
        island: u32,
        list: u32,
        /// The block the allocator hands out for the one new island.
        new_island: u32,
    }

    fn record(e: &mut Engine, log: &Log, address: u32, answer: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            log.borrow_mut().push((address, a.to_vec()));
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An island of the scene with `count` entities (`entity_island` for their island field).
    fn array(e: &mut Engine, header: u32, values: &[u32], capacity: u32) {
        let data = e.mem.alloc(capacity * 4 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *v);
        }
        e.mem.set_u32(header, data);
        e.mem.set_u32(header + 4, values.len() as u32);
        e.mem.set_u32(header + 8, capacity);
    }

    /// A world with a router (stack of 0x1000 bytes holding `groups_of_entities` at its start,
    /// the union-find array), an allocator, an island with entities `entities` (group `i` for
    /// entity `i`) and an empty island list. `sizes` are the group sizes the union-find fill
    /// answers, `picked` the group it picks. The callees only record their words.
    fn scene(entity_groups: &[u32], sizes: &[u32], picked: u32, nothing_to_split: bool) -> Scene {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let router = e.mem.alloc(0x20);
        let stack = e.mem.alloc(0x1000);
        e.mem.set_u32(router, stack);
        for (i, group) in entity_groups.iter().enumerate() {
            e.mem.set_u32(stack + 4 * i as u32, *group);
        }
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        e.put_vtable(
            ALLOCATOR_VTABLE,
            &[0, ALLOCATE_BLOCK, FREE_BLOCK, 0, FREE_ARRAY],
        );
        e.register_double(MEMORY_ROUTER, move |_, _| router.into_ret());
        let new_island = e.mem.alloc(NEW_ISLAND_BYTES);
        e.register_double(ALLOCATE_BLOCK, move |_, a| {
            assert_eq!(a[1], 0x6c);
            new_island.into_ret()
        });
        for address in [FREE_BLOCK, FREE_ARRAY] {
            record(&mut e, &log, address, 0);
        }
        for address in [
            MEM_SET,
            WORLD_AFTER_SPLIT_MOVE,
            WORLD_AFTER_SPLIT,
            AGENT_COPY_AND_RELINK_ENTRY,
            ARRAY_REDUCE,
            UNION_FIND_REINDEX,
        ] {
            record(&mut e, &log, address, 0);
        }
        let seen = Rc::clone(&log);
        e.register_double(UNION_FIND_CONSTRUCTOR, move |e, a| {
            seen.borrow_mut().push((UNION_FIND_CONSTRUCTOR, a.to_vec()));
            // The structure remembers the array and the node count.
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            Ret::default()
        });
        record(
            &mut e,
            &log,
            ISLAND_BUILD_UNION_FIND,
            nothing_to_split as u32,
        );
        record(&mut e, &log, UNION_FIND_PICK_GROUP, picked);
        let group_sizes = sizes.to_vec();
        let seen = Rc::clone(&log);
        e.register_double(UNION_FIND_FILL_SIZES, move |e, a| {
            seen.borrow_mut().push((UNION_FIND_FILL_SIZES, a.to_vec()));
            let array = a[1];
            let data = e.mem.u32(array);
            for (i, size) in group_sizes.iter().enumerate() {
                e.mem.set_u32(data + 4 * i as u32, *size);
            }
            e.mem.set_u32(array + 4, group_sizes.len() as u32);
            Ret::default()
        });
        e.register(ISLAND_CONSTRUCTOR, |e, a| {
            let island = a[0];
            e.mem.set_u16(island + ISLAND_DIRTY_INDEX, 0xffff);
            array_header(e, island + ISLAND_ENTITIES, 8);
            a[0].into_ret()
        });
        e.register(ARRAY_RESERVE_MORE, |_, _| Ret::default());
        e.register(ARRAY_RESERVE_EXACT, |_, _| Ret::default());
        // Agent entries are deallocated by shortening the island's last sector.
        e.register(AGENT_DEALLOCATE_ENTRY, |e, a| {
            let used = e.mem.u32(a[0]).wrapping_sub(0x20);
            e.mem.set_u32(a[0], used);
            Ret::default()
        });
        e.put_vtable(CONSTRAINT_VTABLE, &[0, 0, 0, 0, CONSTRAINT_ENTITIES]);
        let world = e.mem.alloc(0x400);
        let island = e.mem.alloc(0x80);
        e.mem.set_u32(island, 0);
        let list = e.mem.alloc(0x10);
        array(&mut e, list, &[], 4);
        // The scratch sizes array reads a group count of its own storage.
        let stack_top = stack;
        Scene {
            e,
            log,
            router,
            stack_top,
            world,
            island,
            list,
            new_island,
        }
    }

    /// An empty `hkArray` with room for `capacity` words.
    fn array_header(e: &mut Engine, header: u32, capacity: u32) {
        array(e, header, &[], capacity);
    }

    fn entity(e: &mut Engine, island: u32) -> u32 {
        let entity = e.mem.alloc(0x120);
        e.mem.set_u32(entity + ENTITY_ISLAND, island);
        entity
    }

    /// Gives the scene's island its entities and an empty `+0x38` array.
    fn fill_island(s: &mut Scene, count: u32) -> Vec<u32> {
        let (island, e) = (s.island, &mut s.e);
        let entities: Vec<u32> = (0..count).map(|_| entity(e, island)).collect();
        array(e, island + ISLAND_ENTITIES, &entities, 8);
        e.mem.set_u32(island + ISLAND_ENTITY_COUNT, count);
        array(e, island + ISLAND_ARRAY_38, &[], 4);
        e.mem.set_u16(island + ISLAND_INDEX, 0);
        e.mem.set_u32(s.world + WORLD_FIXED_ISLAND, 0x7777);
        // No sectors, and a sector array that owns no memory.
        e.mem.set_u32(island + ISLAND_TRACK_CAPACITY, 0x8000_0000);
        entities
    }

    fn split(s: &mut Scene) {
        let (island, world, list) = (s.island, s.world, s.list);
        s.e.call(0x00cb_6060, &args![island, world, list, 0u32]);
    }

    #[test]
    fn nothing_to_split_clears_the_marked_bits_and_restores_the_router() {
        let mut s = scene(&[], &[], 0, true);
        fill_island(&mut s, 2);
        s.e.mem.set_u8(s.island + ISLAND_FLAGS_25, 0xff);
        let start = s.e.mem.u32(s.router);
        split(&mut s);
        assert_eq!(s.e.mem.u8(s.island + ISLAND_FLAGS_25), 0xf3);
        assert_eq!(s.e.mem.u32(s.router), start);
        assert_eq!(calls(&s.log, UNION_FIND_CONSTRUCTOR).len(), 1);
        assert_eq!(calls(&s.log, ISLAND_BUILD_UNION_FIND)[0][0], s.island);
        assert!(calls(&s.log, UNION_FIND_FILL_SIZES).is_empty());
    }

    #[test]
    fn a_single_group_leaves_the_island_alone() {
        let mut s = scene(&[0, 0], &[2], 0, false);
        fill_island(&mut s, 2);
        let start = s.e.mem.u32(s.router);
        // Unmarked island and a world with no minimum size: every flag is cleared by memSet.
        split(&mut s);
        let mem_set = calls(&s.log, MEM_SET);
        assert_eq!(mem_set.len(), 1);
        assert_eq!((mem_set[0][1], mem_set[0][2]), (0, 2));
        assert_eq!(s.e.mem.u32(s.router), start);
        assert_eq!(s.e.mem.u32(s.list + 4), 0);
        assert!(calls(&s.log, WORLD_AFTER_SPLIT).is_empty());
        // The union-find and its parents were built on the island's entity count.
        let constructor = &calls(&s.log, UNION_FIND_CONSTRUCTOR)[0];
        assert_eq!(constructor[2], 2);
    }

    #[test]
    fn two_groups_make_a_new_island_and_move_the_agents_that_left() {
        let mut s = scene(&[0, 0, 0], &[3, 0], 0, false);
        let entities = fill_island(&mut s, 3);
        let (island, world, new_island) = (s.island, s.world, s.new_island);
        let start = s.stack_top;
        let first_entity = entities[0];
        let e = &mut s.e;
        // A constraint word behind the island whose entity still lives in the island.
        let constraint = e.mem.alloc(0x40);
        e.mem.set_u32(constraint, CONSTRAINT_VTABLE);
        let slot_log = Rc::clone(&s.log);
        e.register_double(CONSTRAINT_ENTITIES, move |e, a| {
            slot_log
                .borrow_mut()
                .push((CONSTRAINT_ENTITIES, a.to_vec()));
            // One entity, in the island: the array's size and storage.
            let data = e.mem.u32(a[1]);
            e.mem.set_u32(data, first_entity);
            e.mem.set_u32(a[1] + 4, 1);
            Ret::default()
        });
        array(e, island + ISLAND_ARRAY_38, &[constraint], 4);
        // One agent sector with two entries of 0x20 bytes: the first belongs to the island, the
        // second to the new island.
        let sector = e.mem.alloc(0x200);
        let (own_object, new_object) = (e.mem.alloc(0x120), e.mem.alloc(0x120));
        e.mem.set_u32(own_object + ENTITY_ISLAND, island);
        e.mem.set_u32(new_object + ENTITY_ISLAND, new_island);
        e.mem.set_u32(sector + 0x10, own_object);
        e.mem.set_u8(sector + 3, 0x20);
        e.mem.set_u32(sector + 0x20 + 0x10, new_object);
        e.mem.set_u8(sector + 0x20 + 3, 0x20);
        array(e, island + ISLAND_TRACK_SECTORS, &[sector], 4);
        e.mem.set_u32(island + ISLAND_TRACK_SECTOR_COUNT, 1);
        e.mem.set_u32(island + ISLAND_TRACK_USED, 0x40);
        e.mem.set_u8(island + ISLAND_BYTE_INDEX, 9);
        // The marked bits make `00cb49e0` the group builder (the world does not rank entities).
        e.mem.set_u8(island + ISLAND_FLAGS_25, 0xc);
        split(&mut s);
        let created = s.e.mem.u32(s.e.mem.u32(s.list));
        assert_eq!(created, new_island);
        assert_eq!(s.e.mem.u32(s.list + 4), 1);
        assert_eq!(s.e.mem.u32(s.router), start);
        assert_eq!(s.e.mem.u8(created + ISLAND_BYTE_INDEX), 10);
        assert_eq!(s.e.mem.u16(created + ISLAND_INDEX), 0);
        assert_eq!(s.e.mem.u8(created + ISLAND_FLAGS_26), 5);
        assert_eq!(
            calls(&s.log, WORLD_AFTER_SPLIT_MOVE),
            vec![vec![world, island]]
        );
        assert_eq!(calls(&s.log, WORLD_AFTER_SPLIT), vec![vec![world, island]]);
        assert_eq!(calls(&s.log, CONSTRAINT_ENTITIES).len(), 1);
        // The constraint word stays with the island: its entity is there. Its owner word is set.
        assert_eq!(s.e.mem.u32(constraint + 0xc), island);
        assert_eq!(s.e.mem.u32(island + ISLAND_ARRAY_38 + 4), 1);
        // Only the second entry left the island: copied to the new island's track and removed.
        assert_eq!(
            calls(&s.log, AGENT_COPY_AND_RELINK_ENTRY),
            vec![vec![new_island + 0x58, sector + 0x20]]
        );
        assert_eq!(s.e.mem.u32(island + ISLAND_TRACK_USED), 0x20);
    }

    #[test]
    fn leftover_agents_are_copied_to_their_islands_when_most_entities_leave() {
        // Entities 0 and 2 go to group 1 (the new island), entity 1 stays.
        let mut s = scene(&[1, 0, 1], &[1, 2], 0, false);
        let entities = fill_island(&mut s, 3);
        let (island, new_island) = (s.island, s.new_island);
        let start = s.stack_top;
        let e = &mut s.e;
        // Entity 0 has one master constraint with counters (2, 3, 4) and an internal block.
        let record = e.mem.alloc(0x30);
        let instance = e.mem.alloc(0x40);
        let block = e.mem.alloc(0x40);
        e.mem.set_u32(record, instance);
        e.mem.set_u16(record + 0x14, 2);
        e.mem.set_u16(record + 0x16, 3);
        e.mem.set_u16(record + 0x18, 4);
        e.mem.set_u32(instance + 0x30, block);
        e.mem.set_u32(entities[0] + ENTITY_MASTERS, record);
        e.mem.set_u16(entities[0] + ENTITY_MASTERS_COUNT, 1);
        e.mem.set_u32(island + 8, 7);
        e.mem.set_u32(island + ISLAND_CONSTRAINT_COUNT, 10);
        for offset in [0xc, 0x10, 0x14] {
            e.mem.set_u32(island + offset, 100);
        }
        for offset in [0x14, 0x16, 0x18] {
            e.mem.set_u16(block + offset, 9);
        }
        // One sector holding two entries: the first is in the island, the second in a third
        // island at 0x5000. Most entities leave, so the whole track is taken apart.
        e.map(0x5000, 0x1000);
        e.mem.set_u16(0x5000 + ISLAND_INDEX, 3);
        let sector = e.mem.alloc(0x200);
        let (own_object, other_object) = (e.mem.alloc(0x120), e.mem.alloc(0x120));
        e.mem.set_u32(own_object + ENTITY_ISLAND, island);
        e.mem.set_u32(other_object + ENTITY_ISLAND, 0x5000);
        e.mem.set_u32(sector + 0x10, own_object);
        e.mem.set_u8(sector + 3, 0x20);
        e.mem.set_u32(sector + 0x20 + 0x10, other_object);
        e.mem.set_u8(sector + 0x20 + 3, 0x20);
        array(e, island + ISLAND_TRACK_SECTORS, &[sector], 4);
        e.mem.set_u32(island + ISLAND_TRACK_SECTOR_COUNT, 1);
        e.mem.set_u32(island + ISLAND_TRACK_USED, 0x40);
        split(&mut s);
        let created = s.e.mem.u32(s.e.mem.u32(s.list));
        assert_eq!(created, new_island);
        // Entities 0 and 2 moved to the new island with their island field and index.
        assert_eq!(s.e.mem.u32(entities[0] + ENTITY_ISLAND), created);
        assert_eq!(s.e.mem.u16(entities[0] + ENTITY_INDEX_IN_ISLAND), 0);
        assert_eq!(s.e.mem.u16(entities[2] + ENTITY_INDEX_IN_ISLAND), 1);
        assert_eq!(s.e.mem.u32(entities[1] + ENTITY_ISLAND), island);
        assert_eq!(s.e.mem.u32(created + ISLAND_ENTITY_COUNT), 2);
        assert_eq!(s.e.mem.u32(island + ISLAND_ENTITY_COUNT), 1);
        // The moved master constraint: counters moved from the island to the new one.
        assert_eq!(s.e.mem.u32(island + ISLAND_CONSTRAINT_COUNT), 9);
        assert_eq!(s.e.mem.u32(created + ISLAND_CONSTRAINT_COUNT), 1);
        assert_eq!(s.e.mem.u32(instance + 8), created);
        let counters = |s: &Scene, at: u32| {
            (
                s.e.mem.u32(at + 0xc),
                s.e.mem.u32(at + 0x10),
                s.e.mem.u32(at + 0x14),
            )
        };
        assert_eq!(counters(&s, island), (98, 97, 96));
        assert_eq!(counters(&s, created), (2, 3, 4));
        // The agent sector is taken apart: each entry is copied to its island's track.
        assert_eq!(
            calls(&s.log, AGENT_COPY_AND_RELINK_ENTRY),
            vec![
                vec![island + 0x58, sector],
                vec![0x5000 + 0x58, sector + 0x20]
            ]
        );
        let allocator = s.e.mem.u32(s.router + 0x10);
        assert_eq!(
            calls(&s.log, FREE_BLOCK),
            vec![vec![allocator, sector, 0x200]]
        );
        assert_eq!(s.e.mem.u32(island + ISLAND_TRACK_SECTOR_COUNT), 0);
        assert_eq!(s.e.mem.u32(island + ISLAND_TRACK_USED), 0x200);
        assert_eq!(s.e.mem.u32(s.router), start);
    }

    #[test]
    fn the_second_overload_gives_new_islands_the_state_of_the_original() {
        let mut s = scene(&[0, 0, 0], &[3, 0], 0, false);
        fill_island(&mut s, 3);
        let (island, world) = (s.island, s.world);
        // Active island that wants to be inactive: new islands are marked inactive too.
        s.e.mem.set_u8(island + ISLAND_FLAGS_26, 0b01);
        s.e.mem.set_u8(island + ISLAND_BYTE_INDEX, 20);
        array(&mut s.e, world + WORLD_ACTIVE_ISLANDS, &[island, 0x4242], 4);
        array(&mut s.e, world + WORLD_DIRTY_ISLANDS, &[], 4);
        // Island flags 0x25 had bits 0 and 1 set: cleared.
        s.e.mem.set_u8(island + ISLAND_FLAGS_25, 0x03);
        s.e.call(0x00cb_6e70, &args![world, island]);
        assert_eq!(s.e.mem.u8(island + ISLAND_FLAGS_25), 0);
        let active = world + WORLD_ACTIVE_ISLANDS;
        assert_eq!(s.e.mem.u32(active + 4), 3);
        let created = s.e.mem.u32(s.e.mem.u32(active) + 8);
        // State 1 in bits 0-1 and bits 2-3 (the shifted state), marked inactive afterwards
        // clears bits 2-3 again; the byte at +0x24 counts from the old list size.
        assert_eq!(s.e.mem.u8(created + ISLAND_FLAGS_26) & 0xf, 0b0001);
        assert_eq!(s.e.mem.u8(created + ISLAND_BYTE_INDEX), 20);
        assert_eq!(s.e.mem.u32(world + WORLD_DIRTY_ISLANDS + 4), 1);
    }

    #[test]
    fn splitting_all_islands_walks_the_active_list_from_the_end() {
        let mut s = scene(&[], &[], 0, true);
        fill_island(&mut s, 0);
        let world = s.world;
        let (first, second, third) = (
            s.e.mem.alloc(0x80),
            s.e.mem.alloc(0x80),
            s.e.mem.alloc(0x80),
        );
        array(
            &mut s.e,
            world + WORLD_ACTIVE_ISLANDS,
            &[first, second, third],
            4,
        );
        for island in [first, second, third] {
            s.e.mem.set_u16(island + ISLAND_INDEX, 0);
        }
        // Bit 0 or 1 of the byte at +0x25 selects the islands to split: the third and the first.
        s.e.mem.set_u8(third + ISLAND_FLAGS_25, 1);
        s.e.mem.set_u8(first + ISLAND_FLAGS_25, 2);
        // The world does not want islands: nothing at all.
        s.e.call(0x00cb_6f30, &args![world]);
        assert!(calls(&s.log, ISLAND_BUILD_UNION_FIND).is_empty());
        s.e.mem.set_u8(world + WORLD_WANT_ISLANDS, 1);
        s.e.call(0x00cb_6f30, &args![world]);
        let split: Vec<u32> = calls(&s.log, ISLAND_BUILD_UNION_FIND)
            .iter()
            .map(|words| words[0])
            .collect();
        assert_eq!(split, vec![third, first]);
    }
}

#[cfg(test)]
mod dirty_island_order_tests {
    use super::*;

    /// An island whose first entity has the unique id `id`.
    fn island(e: &mut Engine, id: u32) -> u32 {
        let island = e.mem.alloc(0x6c);
        let entities = e.mem.alloc(4);
        let entity = e.mem.alloc(0x210);
        e.mem.set_u32(entity + 0xd4, id);
        e.mem.set_u32(entities, entity);
        e.mem.set_u32(island + ISLAND_ENTITIES, entities);
        island
    }

    fn before(e: &mut Engine, a: u32, b: u32) -> u8 {
        let result = e.mem.alloc(4);
        e.mem.set_u32(result, 0xaaaa_aaaa);
        let answer = e.call(DIRTY_ISLAND_LESS, &args![result, a, b]).u32();
        assert_eq!(answer, result);
        // Only the byte is written.
        assert_eq!(e.mem.u32(result) & 0xffff_ff00, 0xaaaa_aa00);
        e.mem.u8(result)
    }

    #[test]
    fn fn_00cb3f90_orders_islands_by_the_unique_id_of_their_first_entity() {
        let mut e = Engine::new();
        let low = island(&mut e, 5);
        let high = island(&mut e, 0x8000_0000);
        assert_eq!(before(&mut e, low, high), 1);
        assert_eq!(before(&mut e, high, low), 0);
        assert_eq!(before(&mut e, low, low), 0);
        // A null second island is never after; a null first one is before any other.
        assert_eq!(before(&mut e, low, 0), 0);
        assert_eq!(before(&mut e, 0, 0), 0);
        assert_eq!(before(&mut e, 0, high), 1);
    }
}

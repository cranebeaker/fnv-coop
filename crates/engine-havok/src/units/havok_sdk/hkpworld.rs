//! `Havok SDK/hkpworld.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit is large (115 functions). Sessions translate it in address order; the first
//! session did the 60 queue entries `00c90e40` to `00c929e0`; a later one added `00c90f40` and
//! the phantom, batch, action, constraint and filter-update functions `00c92c70` to
//! `00c97f50` (15 functions); the next one continues at `00c92a30` with what is left.
//!
//! Layouts and helpers come first. `HkpWorld` is the PC `hkpWorld` (0x350 bytes): its field
//! names and offsets are the Xbox PDB's, and every offset used here is read by the PC code at
//! the same place. Fields inside `m_dynamicsStepInfo` (+0x1d0) that the PDB does not name
//! separately are read by offset (the `DYNAMICS_*` constants).
//!
//! Conventions of this file:
//! - `hkpWorld` methods are `__thiscall` (`this` in ECX); the uniform form passes it first.
//! - A float passed through the x87 stack (`FLD`/`FSTP`) goes through [`x87_quiet`], which
//!   quiets a signalling NaN as the hardware does.
//! - Pieces the compiler outlined from a bigger function take registers the calling
//!   convention does not have (`ESI`, `EDI`, `EAX` set by the caller). They cannot be called
//!   by address in the uniform form: they are translated as plain Rust functions with the
//!   registers as leading parameters, are not registered, and carry a `Translation
//!   incomplete` marker saying so. `00c90f40` is such a piece (EDI, ESI and one stack
//!   argument).
//! - `RDTSC` (the profiling records of the ray and linear casts) has no model in the engine:
//!   the host clock stands in for the time stamp, which nothing but the profiler reads.

#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------------------------
// Callees outside this file, by address.
// ---------------------------------------------------------------------------------------

/// `TlsGetValue` import slot, and the TLS index of the profiling record stream.
pub(crate) const TLS_GET_VALUE: u32 = 0x00fd_f044;
pub(crate) const PROFILE_SLOT: u32 = 0x0126_835c;
/// `EnterCriticalSection` and `LeaveCriticalSection` import slots.
const ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
const LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// Profiling labels: `"TtworldCastRayCollector"`, `"TtworldCastRay"`, `"TtworldLinCast"` and
/// the end-of-section label `"Et"`.
const PROFILE_CAST_RAY_COLLECTOR: u32 = 0x010c_6490;
const PROFILE_CAST_RAY: u32 = 0x010c_64a8;
const PROFILE_LINEAR_CAST: u32 = 0x010c_64b8;
pub(crate) const PROFILE_END: u32 = 0x010c_41f8;
/// Increment and decrement of the reference count of an `hkReferencedObject` (`this` only);
/// the count is the signed word at +6.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// Hands out the thread's memory router (no arguments); `+0x10` of it is the allocator whose
/// vtable slot +4 allocates `(size)` and slot +8 frees `(block, size)`.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `00c90490` / `00c904e0`: the calls around the `m_isLocked` count at +0xbc (`this` only).
const LOCK_ENTER: u32 = 0x00c9_0490;
const LOCK_LEAVE: u32 = 0x00c9_04e0;
/// The `m_pendingOperations` queue (`hkpWorldOperationQueue` in the Xbox PDB): `00cf7c10` and
/// `00cf7ae0` take only the queue; `00cf7690` takes the operation record.
const QUEUE_FLUSH_PENDING: u32 = 0x00cf_7c10;
const QUEUE_FLUSH_BODY_PENDING: u32 = 0x00cf_7ae0;
const QUEUE_ENQUEUE: u32 = 0x00cf_7690;
/// `hkpSimulation` calls: `00cf86e0` (one float), `00cf86f0` (bool result).
const SIMULATION_SET_TIME: u32 = 0x00cf_86e0;
const SIMULATION_TEST: u32 = 0x00cf_86f0;
/// Entity callees (`00c9c040(entity, world, shape)` ...).
const SET_CACHED_SHAPE_DATA: u32 = 0x00c9_c040;
const SET_TIME_INFORMATION: u32 = 0x00cf_1480;
const ADD_ENTITY_SI: u32 = 0x00cb_4640;
const ADD_ENTITY_BP: u32 = 0x00cb_59f0;
const FIRE_WORLD_ENTITY_ADDED: u32 = 0x00d0_0500;
const FIRE_ENTITY_ADDED: u32 = 0x00cf_fa00;
const REMOVE_ENTITY_SI: u32 = 0x00cb_5c10;
const FIRE_WORLD_ENTITY_REMOVED: u32 = 0x00d0_05a0;
const FIRE_ENTITY_REMOVED: u32 = 0x00cf_fab0;
const REMOVE_ENTITY_BP: u32 = 0x00cb_4740;
const ENTITY_RELEASE_BOUNDING_DATA: u32 = 0x00d0_1a80;
const FIND_AGENT: u32 = 0x00cf_dd70;
const ADD_PAIRS: u32 = 0x00cf_7080;
/// `00d01d20(world)` (cdecl, one argument) and `00d03290(simulation, a, b)` (thiscall).
const WATCH_HEAP_MEMORY: u32 = 0x00d0_1d20;
const SIMULATION_FINISH_MT_STEP: u32 = 0x00d0_3290;
/// Ray and linear casters (`this` = a caller-built object): `00cbf860`, `00cfab30`, `00cfaed0`.
const WORLD_RAY_CASTER_CAST_RAY: u32 = 0x00cb_f860;
const SIMPLE_RAY_CASTER_CAST_RAY: u32 = 0x00cf_ab30;
const LINEAR_CASTER_LINEAR_CAST: u32 = 0x00cf_aed0;
/// Vtables of the objects the casts build on their stack.
const RAY_CASTER_VTABLE: u32 = 0x010c_3bd4;
const SIMPLE_RAY_CASTER_VTABLE: u32 = 0x010c_647c;
const LINEAR_CASTER_VTABLE: u32 = 0x010c_6488;
/// `float` constant stored into the linear caster object at +0x80.
const LINEAR_CASTER_DEFAULT: u32 = 0x0104_da4c;
/// Job queue calls: `00d04b90(queue, job, 1)` and `00d04380(semaphore)` (acquire).
const JOB_QUEUE_ADD_JOB: u32 = 0x00d0_4b90;
const SEMAPHORE_ACQUIRE: u32 = 0x00d0_4380;
/// `00d05de0(job)`: initializer of a job header (a bare `RET`).
const JOB_HEADER_INIT: u32 = 0x00d0_5de0;
/// `hkpPhysicsSystem` calls (`this` = the system).
const PHYSICS_SYSTEM_CONSTRUCT: u32 = 0x00d0_65e0;
const PHYSICS_SYSTEM_ADD_RIGID_BODY: u32 = 0x00d0_64a0;
const PHYSICS_SYSTEM_ADD_CONSTRAINT: u32 = 0x00d0_6540;
const PHYSICS_SYSTEM_ADD_ACTION: u32 = 0x00d0_6590;
const PHYSICS_SYSTEM_ADD_PHANTOM: u32 = 0x00d0_64f0;
/// The x87 square root helper (`_CIsqrt`): argument in ST0.
const X87_SQRT: u32 = 0x00ec_6040;
/// `00cfd450(object, world)`, `00cfca20(object, world)` and `00cfb240(registry, object,
/// kind)`: constructors and registration used by the outlined constructor piece `00c912e0`.
const MAKE_HELPER_FIRST: u32 = 0x00cf_d450;
const MAKE_HELPER_SECOND: u32 = 0x00cf_ca20;
const REGISTER_HELPER: u32 = 0x00cf_b240;
/// The `double` 0.5 that `00c90e60` multiplies with.
const HALF: u32 = 0x0101_1588;

// ---------------------------------------------------------------------------------------
// Layouts.
// ---------------------------------------------------------------------------------------

/// Offsets inside `m_dynamicsStepInfo` (+0x1d0), named only by their place in the world.
const DYNAMICS_FLOAT_1E4: u32 = 0x1e4;
const DYNAMICS_FLOAT_1E8: u32 = 0x1e8;
const DYNAMICS_FLOAT_240: u32 = 0x240;
const DYNAMICS_WORD_2F4: u32 = 0x2f4;
const DYNAMICS_WORD_2F8: u32 = 0x2f8;
const DYNAMICS_BYTE_304: u32 = 0x304;
/// Three consecutive bytes (+0x305, +0x306, +0x307).
const DYNAMICS_BYTES_305: u32 = 0x305;
const DYNAMICS_FLOAT_308: u32 = 0x308;

layout! {
    /// `hkpWorld` (Xbox PDB), 0x350 bytes. Only the fields the translations of this file use
    /// are listed; names and offsets are the PDB's, confirmed against the PC code.
    pub struct HkpWorld: 0x350 {
        /// `m_simulation` (Xbox PDB): `hkpSimulation*`.
        0x08 m_simulation: Ptr,
        /// `m_fixedIsland` (Xbox PDB): `hkpSimulationIsland*`.
        0x20 m_fixedIsland: Ptr,
        /// `m_activeSimulationIslands` (Xbox PDB): `hkArray` data pointer.
        0x28 m_activeSimulationIslands: Ptr,
        /// Size of `m_activeSimulationIslands`.
        0x2c m_activeSimulationIslandsSize: i32,
        /// `m_inactiveSimulationIslands` (Xbox PDB): `hkArray` data pointer.
        0x34 m_inactiveSimulationIslands: Ptr,
        /// Size of `m_inactiveSimulationIslands`.
        0x38 m_inactiveSimulationIslandsSize: i32,
        /// `m_memoryWatchDog` (Xbox PDB): `hkRefPtr`.
        0x50 m_memoryWatchDog: Ptr,
        /// `m_broadPhase` (Xbox PDB): `hkpBroadPhase*`.
        0x58 m_broadPhase: Ptr,
        /// `m_kdTreeManager` (Xbox PDB): `hkpTreeWorldManager*`.
        0x5c m_kdTreeManager: Ptr,
        /// `m_autoUpdateTree` (Xbox PDB).
        0x60 m_autoUpdateTree: bool,
        /// `m_broadPhaseDispatcher` (Xbox PDB): `hkpTypedBroadPhaseDispatcher*`.
        0x64 m_broadPhaseDispatcher: Ptr,
        /// `m_collisionInput` (Xbox PDB): `hkpProcessCollisionInput*`.
        0x78 m_collisionInput: Ptr,
        /// `m_collisionFilter` (Xbox PDB): `hkpCollisionFilter*`.
        0x7c m_collisionFilter: Ptr,
        /// `m_collisionDispatcher` (Xbox PDB).
        0x80 m_collisionDispatcher: Ptr,
        /// `m_convexListFilter` (Xbox PDB).
        0x84 m_convexListFilter: Ptr,
        /// `m_pendingOperations` (Xbox PDB): `hkpWorldOperationQueue*`.
        0x88 m_pendingOperations: Ptr,
        /// `m_pendingOperationsCount` (Xbox PDB).
        0x8c m_pendingOperationsCount: i32,
        /// `m_pendingBodyOperationsCount` (Xbox PDB).
        0x90 m_pendingBodyOperationsCount: i32,
        /// `m_criticalOperationsLockCount` (Xbox PDB).
        0x94 m_criticalOperationsLockCount: i32,
        /// A second lock count the phantom functions add to the first (name unconfirmed).
        0x98 m_phantomOperationsLockCount: i32,
        /// `m_blockExecutingPendingOperations` (Xbox PDB).
        0x9c m_blockExecutingPendingOperations: bool,
        /// `m_criticalOperationsAllowed` (Xbox PDB).
        0x9d m_criticalOperationsAllowed: bool,
        /// `m_pendingOperationQueueCount` (Xbox PDB).
        0xa4 m_pendingOperationQueueCount: i32,
        /// `m_modifyConstraintCriticalSection` (Xbox PDB): `hkCriticalSection*`.
        0xb8 m_modifyConstraintCriticalSection: Ptr,
        /// `m_isLocked` (Xbox PDB).
        0xbc m_isLocked: i32,
        /// A flag (name unconfirmed): set, `addEntityBatch` builds a new island for the batch.
        0xc8 m_addsToNewIsland: bool,
        /// `m_lastEntityUid` (Xbox PDB).
        0x104 m_lastEntityUid: u32,
        /// `m_lastConstraintUid` (name unconfirmed): `addConstraint` counts it up.
        0x10c m_lastConstraintUid: u32,
        /// `m_phantoms` (Xbox PDB): `hkArray` data pointer.
        0x110 m_phantoms: Ptr,
        /// Size of `m_phantoms`.
        0x114 m_phantomsSize: i32,
        /// `m_broadPhaseBorder` (Xbox PDB): `hkpBroadPhaseBorder*`.
        0x1bc m_broadPhaseBorder: Ptr,
        /// `m_collisionCallbackUtil` (Xbox PDB).
        0x1c0 m_collisionCallbackUtil: Ptr,
    }

    /// `hkpEntity` (Xbox PDB) as far as the world's functions read it (PC offsets).
    pub struct HkpEntity: 0x220 {
        /// `m_memSizeAndFlags` (Xbox PDB).
        0x04 m_memSizeAndFlags: u16,
        /// `m_collidable.m_shape` (Xbox PDB; `m_collidable` is at +0x10).
        0x10 m_collidableShape: Ptr,
        /// `m_collidable.m_motion` (Xbox PDB).
        0x18 m_collidableMotion: Ptr,
        /// A pointer inside the collidable's bounding-volume data (+0x54).
        0x54 m_boundingVolumeChildData: Ptr,
        /// `m_uid` (Xbox PDB).
        0xd4 m_uid: u32,
        /// Two words of the entity (+0xea, +0xec) the world clears when it adds the entity.
        0xea m_wordEa: u16,
        0xec m_wordEc: u16,
    }
}

/// `m_collidable` (Xbox PDB), +0x10 of the entity.
const ENTITY_COLLIDABLE: u32 = 0x10;
/// `m_collidable.m_broadPhaseHandle` (Xbox PDB), +0x24 of the entity.
const ENTITY_BROADPHASE_HANDLE: u32 = 0x24;
/// The entity's bounding-volume data (`m_collidable.m_boundingVolumeData`, +0x30).
const ENTITY_BOUNDING_DATA: u32 = 0x30;
/// The motion embedded in the entity (`m_motion`, +0xe0) and the swept transform inside it.
const ENTITY_MOTION: u32 = 0xe0;
const ENTITY_SWEPT_TRANSFORM: u32 = 0xf0;

// ---------------------------------------------------------------------------------------
// Helpers.
// ---------------------------------------------------------------------------------------

/// Low word of the time-stamp counter (`RDTSC`). The engine has no model of it: the host
/// clock stands in (only the profiling records take it).
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a profiling record (`label`, time stamp) to the thread's record stream
/// (`TlsGetValue` of the index at `0126835c`) while its cursor (`+4`) is below its limit
/// (`+0xC`); a record is 12 bytes.
pub(crate) fn profile_record(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// Allocates `size` bytes through the memory router's allocator (`00c85750`, vtable slot 4).
fn router_alloc(e: &mut Engine, size: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 4, &args![size]).u32()
}

/// Frees a block of `size` bytes through the router's allocator (vtable slot 8).
fn router_free(e: &mut Engine, block: u32, size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    e.vcall(allocator, 8, &args![block, size]);
}

/// `FLD float` / `FSTP float` copy of the `float` at `from` to `to`.
fn copy_float(e: &mut Engine, from: u32, to: u32) {
    let value = x87_quiet(e.mem.f32(from));
    e.mem.set_f32(to, value);
}

fn copy_word(e: &mut Engine, from: u32, to: u32) {
    let value = e.mem.u32(from);
    e.mem.set_u32(to, value);
}

fn copy_byte(e: &mut Engine, from: u32, to: u32) {
    let value = e.mem.u8(from);
    e.mem.set_u8(to, value);
}

/// `MOVAPS` copy of a 16-byte vector.
fn copy_vector(e: &mut Engine, from: u32, to: u32) {
    let bytes = e.mem.bytes(from, 16);
    e.mem.write(to, &bytes);
}

/// `SHUFPS` splat: the 32-bit pattern `bits` into the four lanes at `to`.
fn splat_float(e: &mut Engine, bits: u32, to: u32) {
    for lane in 0..4 {
        e.mem.set_u32(to + 4 * lane, bits);
    }
}

/// The world has a kd-tree manager (+0x5c) and its slot +0x10 says it is in use: the test
/// every cast makes before preferring the tree.
fn kd_tree_in_use(e: &mut Engine, this: Ptr<HkpWorld>) -> bool {
    let tree = e.get(this, HkpWorld::m_kdTreeManager);
    !tree.is_null() && e.vcall(tree.addr(), 0x10, &args![]).bool()
}

/// The number of 64-element jobs `count` elements split into, as the dispatchers compute it:
/// `((count - 1 + ((count - 1) >> 31 & 0x3f)) >> 6) + 1`.
fn job_block_count(count: i32) -> i32 {
    let last = count.wrapping_sub(1);
    (last.wrapping_add((last >> 31) & 0x3f) >> 6).wrapping_add(1)
}

/// The entry `index` of the `hkArray` whose data pointer is `data`.
fn array_entry(e: &Engine, data: u32, index: i32) -> u32 {
    e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)))
}

// ---------------------------------------------------------------------------------------
// Translations.
// ---------------------------------------------------------------------------------------

// Translated from 00c90e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpContactMgrFactory::calcContentStatistics` (Xbox PDB): tells the statistics collector
/// (first stack argument; its vtable slot +0x18 takes the class name `"hkpContactMgrFactory"`,
/// slot +0x1c takes nothing). The second stack argument is not read.
pub fn hkp_contact_mgr_factory_calc_content_statistics(
    e: &mut Engine,
    _this: Ptr,
    collector: Ptr,
    _unused_3: u32,
) {
    e.vcall(collector.addr(), 0x18, &args![0x010c_63a0u32]);
    e.vcall(collector.addr(), 0x1c, &args![]);
}

// Translated from 00c90e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Xbox name unknown). Stores a first `float` `a` at +4, half of it (`a * 0.5`, the
/// `double` at `01011588`) at +0xC and a second `float` `b` at +8, then the ratios `b / a`
/// (+0x50), `b / (a * 0.5)` (+0x58), `a / b` (+0x54) and `(a * 0.5) / b` (+0x5C). The
/// 16-byte vectors at +0x30 and +0x40 hold `a / b` and `b / a` in all four lanes.
pub fn fn_00c90e60(e: &mut Engine, this: Ptr, a: f32, b: f32) {
    let base = this.addr();
    let a = x87_quiet(a);
    e.mem.set_f32(base + 4, a);
    let half: f64 = e.global(HALF);
    let half_a = (half * a as f64) as f32;
    e.mem.set_f32(base + 0xc, half_a);
    let b = x87_quiet(b);
    e.mem.set_f32(base + 8, b);
    e.mem.set_f32(base + 0x50, (b as f64 / a as f64) as f32);
    e.mem
        .set_f32(base + 0x58, (b as f64 / half_a as f64) as f32);
    e.mem.set_f32(base + 0x54, (a as f64 / b as f64) as f32);
    let a_over_b = e.mem.u32(base + 0x54);
    splat_float(e, a_over_b, base + 0x30);
    let b_over_a = e.mem.u32(base + 0x50);
    splat_float(e, b_over_a, base + 0x40);
    e.mem
        .set_f32(base + 0x5c, (half_a as f64 / b as f64) as f32);
}

// Translated from 00c90ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Called on the motion embedded in an entity (`+0xE0`) with the world's three
/// bytes at +0x305..+0x307. Compares `c & 3` with the low two bits of the byte at +9 (and
/// `c` with the whole byte): the first two arguments are complemented when the comparison
/// says "below", then shifted into the top two bits of the words at +0xA and +0xC.
pub fn fn_00c90ed0(e: &mut Engine, this: Ptr, a: u32, b: u32, c: i32) {
    let base = this.addr();
    let reference = e.mem.u8(base + 9) as u32;
    let mut first = a;
    if ((c as u32 & 3) as u8) < (reference & 3) as u8 {
        first = !first;
    }
    let mut second = b;
    if c < reference as i32 {
        second = !second;
    }
    let word_a = e.mem.u16(base + 0xa);
    e.mem
        .set_u16(base + 0xa, (word_a & 0x3fff) | (first << 14) as u16);
    let word_b = e.mem.u16(base + 0xc);
    e.mem
        .set_u16(base + 0xc, (word_b & 0x3fff) | (second << 14) as u16);
}

// Translated from 00c90f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultithreadConfig::hkpMultithreadConfig` (Xbox PDB): stores 0x46 in its first word and
/// returns `this`.
pub fn hkp_multithread_config_hkp_multithread_config(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x46);
    this
}

// Translated from 00c91000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Copies the 16-byte vector at `source` to +0x10 (where the Xbox PDB layout has the
/// world's `m_gravity`).
pub fn fn_00c91000(e: &mut Engine, this: Ptr<HkpWorld>, source: Ptr) {
    copy_vector(e, source.addr(), this.addr() + 0x10);
}

// Translated from 00c91020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::getCurrentTime` (Xbox PDB): the `float` at +0x14 of the simulation, in `ST0`.
pub fn hkp_world_get_current_time(e: &mut Engine, this: Ptr<HkpWorld>) -> f32 {
    let simulation = e.get(this, HkpWorld::m_simulation);
    x87_quiet(e.mem.f32(simulation.addr() + 0x14))
}

// Translated from 00c91030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::getCurrentPsiTime` (Xbox PDB): the `float` at +0x18 of the simulation, in `ST0`.
pub fn hkp_world_get_current_psi_time(e: &mut Engine, this: Ptr<HkpWorld>) -> f32 {
    let simulation = e.get(this, HkpWorld::m_simulation);
    x87_quiet(e.mem.f32(simulation.addr() + 0x18))
}

// Translated from 00c91040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Passes a `float` (through `FLD`/`FSTP`) to the simulation's `00cf86e0`.
pub fn fn_00c91040(e: &mut Engine, this: Ptr<HkpWorld>, time: f32) {
    let simulation = e.get(this, HkpWorld::m_simulation);
    e.call(SIMULATION_SET_TIME, &args![simulation, x87_quiet(time)]);
}

// Translated from 00c91060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Tail-calls the simulation's `00cf86f0` and returns what it leaves in `EAX` (the
/// caller tests `AL`).
pub fn fn_00c91060(e: &mut Engine, this: Ptr<HkpWorld>) -> u32 {
    let simulation = e.get(this, HkpWorld::m_simulation);
    e.call(SIMULATION_TEST, &args![simulation]).u32()
}

// Translated from 00c910a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Does nothing; takes one stack argument.
pub fn fn_00c910a0(_e: &mut Engine, _this: Ptr<HkpWorld>, _unused_2: u32) {}

// Translated from 00c910b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::checkDeterminism` (Xbox PDB): does nothing in this build.
pub fn hkp_world_check_determinism(_e: &mut Engine, _this: Ptr<HkpWorld>) {}

// Translated from 00c910d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Replaces the world's `m_broadPhaseBorder` (+0x1bc): a previous border gets its
/// vtable slot +0x10 called and a reference removed (`00c905b0`), the new one (when not null)
/// gets a reference added (`00c90510`).
pub fn fn_00c910d0(e: &mut Engine, this: Ptr<HkpWorld>, border: Ptr) {
    let old = e.get(this, HkpWorld::m_broadPhaseBorder);
    if !old.is_null() {
        e.vcall(old.addr(), 0x10, &args![]);
        let current = e.get(this, HkpWorld::m_broadPhaseBorder);
        e.call(REMOVE_REFERENCE, &args![current]);
    }
    e.set(this, HkpWorld::m_broadPhaseBorder, border);
    if !border.is_null() {
        e.call(ADD_REFERENCE, &args![border]);
    }
}

// Translated from 00c91110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed getter of the world's `m_broadPhaseBorder` (+0x1bc).
pub fn fn_00c91110(e: &mut Engine, this: Ptr<HkpWorld>) -> Ptr {
    e.get(this, HkpWorld::m_broadPhaseBorder)
}

// Translated from 00c91120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Clears `m_pendingOperationsCount` and tail-calls `00cf7c10` on the pending
/// operation queue (+0x88).
pub fn fn_00c91120(e: &mut Engine, this: Ptr<HkpWorld>) {
    e.set(this, HkpWorld::m_pendingOperationsCount, 0);
    let queue = e.get(this, HkpWorld::m_pendingOperations);
    e.call(QUEUE_FLUSH_PENDING, &args![queue]);
}

// Translated from 00c91140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Clears `m_pendingBodyOperationsCount` and tail-calls `00cf7ae0` on the pending
/// operation queue.
pub fn fn_00c91140(e: &mut Engine, this: Ptr<HkpWorld>) {
    e.set(this, HkpWorld::m_pendingBodyOperationsCount, 0);
    let queue = e.get(this, HkpWorld::m_pendingOperations);
    e.call(QUEUE_FLUSH_BODY_PENDING, &args![queue]);
}

// Translated from 00c91160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Hands one operation record to the pending operation queue (`00cf7690`). Every
/// caller builds the record on its stack with the operation number in its first byte.
pub fn fn_00c91160(e: &mut Engine, this: Ptr<HkpWorld>, operation: Ptr) {
    let queue = e.get(this, HkpWorld::m_pendingOperations);
    e.call(QUEUE_ENQUEUE, &args![queue, operation]);
}

// Translated from 00c91180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Calls `00c90490` on the world, then counts `m_isLocked` up.
pub fn fn_00c91180(e: &mut Engine, this: Ptr<HkpWorld>) {
    e.call(LOCK_ENTER, &args![this]);
    let locked = e.get(this, HkpWorld::m_isLocked);
    e.set(this, HkpWorld::m_isLocked, locked.wrapping_add(1));
}

// Translated from 00c91190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Counts `m_isLocked` down, then tail-calls `00c904e0` on the world.
pub fn fn_00c91190(e: &mut Engine, this: Ptr<HkpWorld>) {
    let locked = e.get(this, HkpWorld::m_isLocked);
    e.set(this, HkpWorld::m_isLocked, locked.wrapping_sub(1));
    e.call(LOCK_LEAVE, &args![this]);
}

// Translated from 00c911a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Does nothing.
pub fn fn_00c911a0(_e: &mut Engine, _this: Ptr<HkpWorld>) {}

// Translated from 00c911b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::unlockReadOnly` (Xbox PDB): does nothing in this build.
pub fn hkp_world_unlock_read_only(_e: &mut Engine, _this: Ptr<HkpWorld>) {}

/// Calls the import at `import_slot` (`EnterCriticalSection` or `LeaveCriticalSection`) on the
/// world's constraint-update critical section (+0xb8) when it has one; the one stack
/// argument of these functions is overwritten with the section and never read.
fn critical_section_call(e: &mut Engine, this: Ptr<HkpWorld>, import_slot: u32) {
    let section = e.get(this, HkpWorld::m_modifyConstraintCriticalSection);
    if !section.is_null() {
        e.call(import_slot, &args![section]);
    }
}

// Translated from 00c911d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::lockIslandForConstraintUpdate` (Xbox PDB): enters the world's
/// `m_modifyConstraintCriticalSection` (+0xb8) when it exists.
pub fn hkp_world_lock_island_for_constraint_update(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    _unused_2: u32,
) {
    critical_section_call(e, this, ENTER_CRITICAL_SECTION);
}

// Translated from 00c911f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed twin of `00c911d0` (same body): enters the critical section at +0xb8 when it
/// exists.
pub fn fn_00c911f0(e: &mut Engine, this: Ptr<HkpWorld>, _unused_2: u32) {
    critical_section_call(e, this, ENTER_CRITICAL_SECTION);
}

// Translated from 00c91210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::unlockIslandForConstraintUpdate` (Xbox PDB): leaves the critical section at
/// +0xb8 when it exists.
pub fn hkp_world_unlock_island_for_constraint_update(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    _unused_2: u32,
) {
    critical_section_call(e, this, LEAVE_CRITICAL_SECTION);
}

// Translated from 00c91230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed twin of `00c91210` (same body): leaves the critical section at +0xb8 when it
/// exists.
pub fn fn_00c91230(e: &mut Engine, this: Ptr<HkpWorld>, _unused_2: u32) {
    critical_section_call(e, this, LEAVE_CRITICAL_SECTION);
}

// Translated from 00c91250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. When the world has a kd-tree manager (+0x5c), calls its vtable slot +0x14 with 0.
pub fn fn_00c91250(e: &mut Engine, this: Ptr<HkpWorld>) {
    let tree = e.get(this, HkpWorld::m_kdTreeManager);
    if !tree.is_null() {
        e.vcall(tree.addr(), 0x14, &args![0u32]);
    }
}

// Translated from 00c91270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Tail-calls the kd-tree manager's vtable slot +0xc with two arguments (no null
/// check).
pub fn fn_00c91270(e: &mut Engine, this: Ptr<HkpWorld>, first: u32, second: u32) {
    let tree = e.get(this, HkpWorld::m_kdTreeManager);
    e.vcall(tree.addr(), 0xc, &args![first, second]);
}

// Translated from 00c912e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c912e0 - takes EAX (the world) and EDI (a registry object) as register inputs, which have no uniform-ABI form; translated as a Rust function with them as parameters and not registered.
/// Unnamed fragment the compiler outlined from the `hkpWorld` constructor (`EAX` = the
/// world, `EDI` = an object whose `00cfb240` takes `(object, kind)`). Makes three small
/// helper objects: two of 0xC bytes built by `00cfd450(world)` and `00cfca20(world)`, and one
/// of 0x14 bytes built inline (vtable `010c6420`, +8 = `010c6378`, +0x10 = 3); registers them
/// with `registry` as kinds 1, 2 and 3 and drops the creator's reference on each.
pub fn fn_00c912e0(e: &mut Engine, world: Ptr, registry: Ptr) {
    let first = router_alloc(e, 0xc);
    e.mem.set_u16(first + 4, 0xc);
    let first = e.call(MAKE_HELPER_FIRST, &args![first, world]).u32();
    let second = router_alloc(e, 0xc);
    e.mem.set_u16(second + 4, 0xc);
    let second = e.call(MAKE_HELPER_SECOND, &args![second, world]).u32();
    let third = router_alloc(e, 0x14);
    e.mem.set_u16(third + 4, 0x14);
    e.mem.set_u16(third + 6, 1);
    e.mem.set_u32(third, 0x010c_6420);
    e.mem.set_u16(third + 0xe, 1);
    e.mem.set_u32(third + 0x10, 3);
    e.mem.set_u32(third + 8, 0x010c_6378);
    e.call(REGISTER_HELPER, &args![registry, first, 1u32]);
    e.call(REGISTER_HELPER, &args![registry, second, 2u32]);
    e.call(REGISTER_HELPER, &args![registry, third, 3u32]);
    e.call(REMOVE_REFERENCE, &args![first]);
    e.call(REMOVE_REFERENCE, &args![second]);
    e.call(REMOVE_REFERENCE, &args![third]);
}

// Translated from 00c913b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::reenableCollisionBetweenEntityPair` (Xbox PDB). While critical operations are
/// locked (+0x94 non-zero) the request is queued as operation 0x13 (the two entities at +4
/// and +8). Otherwise, when the broad phase (slot +0x4c) accepts the two broad-phase handles
/// (entity +0x24), the collision filter (+0x7c, used as its base at +8; not null-checked)
/// enables the two collidables (entity +0x10), and no agent exists yet between them
/// (`00cfdd70`), the pair is added through the broad-phase dispatcher (`00cf7080`).
pub fn hkp_world_reenable_collision_between_entity_pair(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    entity_a: Ptr,
    entity_b: Ptr,
) {
    if e.get(this, HkpWorld::m_criticalOperationsLockCount) != 0 {
        e.with_stack(0x10, |e, operation| {
            e.mem.set_u8(operation.addr(), 0x13);
            e.mem.set_u32(operation.addr() + 4, entity_a.addr());
            e.mem.set_u32(operation.addr() + 8, entity_b.addr());
            fn_00c91160(e, this, operation);
        });
        return;
    }
    let handle_a = entity_a.addr() + ENTITY_BROADPHASE_HANDLE;
    let handle_b = entity_b.addr() + ENTITY_BROADPHASE_HANDLE;
    let broad_phase = e.get(this, HkpWorld::m_broadPhase);
    if !e
        .vcall(broad_phase.addr(), 0x4c, &args![handle_a, handle_b])
        .bool()
    {
        return;
    }
    let filter = e.get(this, HkpWorld::m_collisionFilter).addr();
    let collidable_a = entity_a.addr() + ENTITY_COLLIDABLE;
    let collidable_b = entity_b.addr() + ENTITY_COLLIDABLE;
    // The filter's base at +8 answers "enabled" through a result pointer (slot +4).
    let enabled = e.with_stack(8, |e, result| {
        let base = filter.wrapping_add(8);
        let vtable = e.mem.u32(base);
        let method = e.mem.u32(vtable + 4);
        let returned = e
            .call(method, &args![base, result, collidable_a, collidable_b])
            .u32();
        e.mem.u8(returned) != 0
    });
    if !enabled {
        return;
    }
    if e.call(FIND_AGENT, &args![collidable_a, collidable_b]).u32() != 0 {
        return;
    }
    let filter_base = if filter == 0 { 0 } else { filter + 8 };
    let dispatcher = e.get(this, HkpWorld::m_broadPhaseDispatcher);
    e.with_stack(8, |e, pair| {
        e.mem.set_u32(pair.addr(), handle_a);
        e.mem.set_u32(pair.addr() + 4, handle_b);
        e.call(ADD_PAIRS, &args![dispatcher, pair, 1u32, filter_base]);
    });
}

// Translated from 00c91470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. While critical operations are locked, queues operation 0x17 (first argument at
/// +4, the low word of the second at +8, the low byte of the third at +0xA; the rest of the
/// record is uninitialized stack in the original and zero here). Otherwise calls the
/// simulation's vtable slot +0x30 with `(first, second, world, third)`.
pub fn fn_00c91470(e: &mut Engine, this: Ptr<HkpWorld>, first: u32, second: u32, third: u32) {
    if e.get(this, HkpWorld::m_criticalOperationsLockCount) != 0 {
        e.with_stack(0x10, |e, operation| {
            e.mem.set_u8(operation.addr(), 0x17);
            e.mem.set_u32(operation.addr() + 4, first);
            e.mem.set_u16(operation.addr() + 8, second as u16);
            e.mem.set_u8(operation.addr() + 0xa, third as u8);
            fn_00c91160(e, this, operation);
        });
        return;
    }
    let simulation = e.get(this, HkpWorld::m_simulation);
    e.vcall(simulation.addr(), 0x30, &args![first, second, this, third]);
}

/// What `addEntity` and `removeEntity` do once the outermost critical section ends: unless
/// executing pending operations is blocked (+0x9c), flush the pending operations when there
/// are some and the body operations when there are some and one queue is in use.
fn leave_critical_section(e: &mut Engine, this: Ptr<HkpWorld>) {
    if e.get(this, HkpWorld::m_blockExecutingPendingOperations) {
        return;
    }
    if e.get(this, HkpWorld::m_pendingOperationsCount) != 0 {
        fn_00c91120(e, this);
    }
    if e.get(this, HkpWorld::m_pendingOperationQueueCount) == 1
        && e.get(this, HkpWorld::m_pendingBodyOperationsCount) != 0
    {
        fn_00c91140(e, this);
    }
}

// Translated from 00c914d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::addEntity` (Xbox PDB). While critical operations are locked the request is
/// queued (operation 1: entity at +4, second argument at +8) and 0 is returned. Otherwise:
/// caches the shape data, makes sure the collidable has its motion (entity vtable slot
/// +0x10), resets the swept transform's time information, clears two words, gives the entity
/// the next uid (`m_lastEntityUid`), adds a reference, adds it to the simulation islands
/// (`00cb4640`), updates the motion's filter bits from the world's bytes at +0x305..+0x307
/// (`00c90ed0`), then inside the critical section adds it to the broad phase and fires the
/// world and entity "added" callbacks. When the outermost critical section ends the pending
/// operations are flushed. Returns the entity.
pub fn hkp_world_add_entity(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    entity: Ptr<HkpEntity>,
    second: u32,
) -> Ptr<HkpEntity> {
    if e.get(this, HkpWorld::m_criticalOperationsLockCount) != 0 {
        e.with_stack(0x10, |e, operation| {
            e.mem.set_u8(operation.addr(), 1);
            e.mem.set_u32(operation.addr() + 4, entity.addr());
            e.mem.set_u32(operation.addr() + 8, second);
            fn_00c91160(e, this, operation);
        });
        return Ptr::NULL;
    }
    let shape = e.get(entity, HkpEntity::m_collidableShape);
    e.call(SET_CACHED_SHAPE_DATA, &args![entity, this, shape]);
    if e.get(entity, HkpEntity::m_collidableMotion).is_null() {
        let motion = e.vcall(entity.addr(), 0x10, &args![]).ptr::<()>();
        e.set(entity, HkpEntity::m_collidableMotion, motion);
    }
    e.call(
        SET_TIME_INFORMATION,
        &args![0.0f32, 0.0f32, entity.addr() + ENTITY_SWEPT_TRANSFORM],
    );
    e.set(entity, HkpEntity::m_wordEa, 0);
    e.set(entity, HkpEntity::m_wordEc, 0);
    e.set(this, HkpWorld::m_criticalOperationsAllowed, false);
    let uid = e.get(this, HkpWorld::m_lastEntityUid).wrapping_add(1);
    e.set(this, HkpWorld::m_lastEntityUid, uid);
    e.set(entity, HkpEntity::m_uid, uid);
    e.call(ADD_REFERENCE, &args![entity]);
    e.call(ADD_ENTITY_SI, &args![this, entity, second]);
    let world = this.addr();
    let byte_305 = e.mem.u8(world + DYNAMICS_BYTES_305) as u32;
    let byte_306 = e.mem.u8(world + DYNAMICS_BYTES_305 + 1) as u32;
    let byte_307 = e.mem.u8(world + DYNAMICS_BYTES_305 + 2) as i32;
    fn_00c90ed0(
        e,
        Ptr::new(entity.addr() + ENTITY_MOTION),
        byte_305,
        byte_306,
        byte_307,
    );
    let lock = e.get(this, HkpWorld::m_criticalOperationsLockCount);
    e.set(
        this,
        HkpWorld::m_criticalOperationsLockCount,
        lock.wrapping_add(1),
    );
    e.set(this, HkpWorld::m_criticalOperationsAllowed, true);
    e.call(ADD_ENTITY_BP, &args![this, entity]);
    e.call(FIRE_WORLD_ENTITY_ADDED, &args![this, entity]);
    e.call(FIRE_ENTITY_ADDED, &args![entity]);
    let lock = e
        .get(this, HkpWorld::m_criticalOperationsLockCount)
        .wrapping_sub(1);
    e.set(this, HkpWorld::m_criticalOperationsLockCount, lock);
    if lock == 0 {
        leave_critical_section(e, this);
    }
    entity
}

// Translated from 00c91620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removeEntity` (Xbox PDB), with the `hkBool*` result as the first stack
/// argument. While critical operations are locked the request is queued (operation 2, entity
/// at +4) and `*result` is 0. Otherwise the critical section is entered; the entity is
/// taken out of the simulation islands (`00cb5c10`), the world and entity "removed"
/// callbacks fire, the broad phase drops it (`00cb4740`); a bounding-volume pointer at +0x54
/// is released (`00d01a80` on +0x30); an entity whose `m_memSizeAndFlags` is 0 is deleted
/// through its vtable slot +0x14; its reference is removed. When the outermost critical
/// section ends the pending operations are flushed. `*result` is 1.
/// Answers `result` (`EAX` on both paths; callers read the flag through it, 00c68c93).
pub fn hkp_world_remove_entity(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    result: Ptr,
    entity: Ptr<HkpEntity>,
) -> Ptr {
    if e.get(this, HkpWorld::m_criticalOperationsLockCount) != 0 {
        e.with_stack(0x10, |e, operation| {
            e.mem.set_u8(operation.addr(), 2);
            e.mem.set_u32(operation.addr() + 4, entity.addr());
            fn_00c91160(e, this, operation);
        });
        e.mem.set_u8(result.addr(), 0);
        return result;
    }
    e.set(this, HkpWorld::m_criticalOperationsLockCount, 1);
    e.call(REMOVE_ENTITY_SI, &args![this, entity]);
    e.call(FIRE_WORLD_ENTITY_REMOVED, &args![this, entity]);
    e.call(FIRE_ENTITY_REMOVED, &args![entity]);
    e.set(this, HkpWorld::m_criticalOperationsAllowed, false);
    e.call(REMOVE_ENTITY_BP, &args![this, entity]);
    if !e
        .get(entity, HkpEntity::m_boundingVolumeChildData)
        .is_null()
    {
        e.call(
            ENTITY_RELEASE_BOUNDING_DATA,
            &args![entity.addr() + ENTITY_BOUNDING_DATA],
        );
    }
    if e.get(entity, HkpEntity::m_memSizeAndFlags) == 0 {
        e.vcall(entity.addr(), 0x14, &args![]);
    }
    e.call(REMOVE_REFERENCE, &args![entity]);
    let lock = e
        .get(this, HkpWorld::m_criticalOperationsLockCount)
        .wrapping_sub(1);
    e.set(this, HkpWorld::m_criticalOperationsLockCount, lock);
    e.set(this, HkpWorld::m_criticalOperationsAllowed, true);
    if lock == 0 {
        leave_critical_section(e, this);
    }
    e.mem.set_u8(result.addr(), 1);
    result
}

/// Nulls the first entry equal to `listener` in the `hkArray` (data pointer at `data_offset`,
/// size at `data_offset + 4`) of the world; when no entry matches (or the array is empty),
/// the word before the array is written (the original indexes with -1, and so do we).
fn null_listener(e: &mut Engine, this: Ptr<HkpWorld>, data_offset: u32, listener: u32) {
    let world = this.addr();
    let size = e.mem.i32(world + data_offset + 4);
    if size > 0 {
        let mut data = e.mem.u32(world + data_offset);
        for index in 0..size {
            if e.mem.u32(data) == listener {
                let data = e.mem.u32(world + data_offset);
                e.mem
                    .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), 0);
                return;
            }
            data = data.wrapping_add(4);
        }
    }
    let data = e.mem.u32(world + data_offset);
    e.mem.set_u32(data.wrapping_sub(4), 0);
}

// Translated from 00c91700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Nulls the entry equal to `listener` in the array at +0x11c (`m_actionListeners`
/// in the Xbox PDB layout); see `null_listener` for the no-match case.
pub fn fn_00c91700(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x11c, listener);
}

// Translated from 00c91750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91700` on the array at +0x140 (`m_constraintListeners` in the Xbox PDB
/// layout).
pub fn fn_00c91750(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x140, listener);
}

// Translated from 00c917a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91700` on the array at +0x128 (`m_entityListeners` in the Xbox PDB layout).
pub fn fn_00c917a0(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x128, listener);
}

// Translated from 00c917f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91700` on the array at +0x134 (`m_phantomListeners` in the Xbox PDB
/// layout).
pub fn fn_00c917f0(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x134, listener);
}

// Translated from 00c91840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91700` on the array at +0x158 (`m_islandActivationListeners` in the Xbox
/// PDB layout).
pub fn fn_00c91840(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x158, listener);
}

// Translated from 00c918e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91700` on the array at +0x164 (`m_worldPostSimulationListeners` in the
/// Xbox PDB layout).
pub fn fn_00c918e0(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x164, listener);
}

// Translated from 00c91a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91700` on the array at +0x1a0 (`m_contactListeners` in the Xbox PDB
/// layout).
pub fn fn_00c91a20(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x1a0, listener);
}

// Translated from 00c91a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91700` on the array at +0x14c (`m_worldDeletionListeners` in the Xbox PDB
/// layout).
pub fn fn_00c91a70(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    null_listener(e, this, 0x14c, listener);
}

// Translated from 00c91b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::stepDeltaTime` (Xbox PDB). Locks the world (`00c91180`), lets the memory
/// watch-dog check (`00d01d20`), steps the simulation (vtable slot +0xc, the time through
/// `FLD`/`FSTP`); a kd-tree manager is told next (`00c91270(0, 0)` when `m_autoUpdateTree`,
/// else `00c91250`), then the world unlocks (`00c91190`). Returns the simulation's result.
pub fn hkp_world_step_delta_time(e: &mut Engine, this: Ptr<HkpWorld>, delta_time: f32) -> u32 {
    fn_00c91180(e, this);
    e.call(WATCH_HEAP_MEMORY, &args![this]);
    let simulation = e.get(this, HkpWorld::m_simulation);
    let result = e
        .vcall(simulation.addr(), 0xc, &args![x87_quiet(delta_time)])
        .u32();
    if !e.get(this, HkpWorld::m_kdTreeManager).is_null() {
        if e.get(this, HkpWorld::m_autoUpdateTree) {
            fn_00c91270(e, this, 0, 0);
        } else {
            fn_00c91250(e, this);
        }
    }
    fn_00c91190(e, this);
    result
}

// Translated from 00c91b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Locks the world, calls the simulation's vtable slot +0x10 with a `float` (through
/// `FLD`/`FSTP`), unlocks and returns the result.
pub fn fn_00c91b70(e: &mut Engine, this: Ptr<HkpWorld>, time: f32) -> u32 {
    fn_00c91180(e, this);
    let simulation = e.get(this, HkpWorld::m_simulation);
    let result = e
        .vcall(simulation.addr(), 0x10, &args![x87_quiet(time)])
        .u32();
    fn_00c91190(e, this);
    result
}

// Translated from 00c91ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Locks the world, calls the simulation's vtable slot +0x14, unlocks and returns
/// the result.
pub fn fn_00c91ba0(e: &mut Engine, this: Ptr<HkpWorld>) -> u32 {
    fn_00c91180(e, this);
    let simulation = e.get(this, HkpWorld::m_simulation);
    let result = e.vcall(simulation.addr(), 0x14, &args![]).u32();
    fn_00c91190(e, this);
    result
}

// Translated from 00c91bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. As `00c91ba0` with the simulation's vtable slot +0x18.
pub fn fn_00c91bd0(e: &mut Engine, this: Ptr<HkpWorld>) -> u32 {
    fn_00c91180(e, this);
    let simulation = e.get(this, HkpWorld::m_simulation);
    let result = e.vcall(simulation.addr(), 0x18, &args![]).u32();
    fn_00c91190(e, this);
    result
}

// Translated from 00c91c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::finishMtStep` (Xbox PDB): finishes the simulation's multithreaded step
/// (`00d03290(simulation, a, b)`); a non-zero result is returned at once. Otherwise a
/// kd-tree manager is told (`00c91270(a, b)` when `m_autoUpdateTree`, else `00c91250`) and 0
/// is returned.
pub fn hkp_world_finish_mt_step(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    first: u32,
    second: u32,
) -> u32 {
    let simulation = e.get(this, HkpWorld::m_simulation);
    let result = e
        .call(SIMULATION_FINISH_MT_STEP, &args![simulation, first, second])
        .u32();
    if result != 0 {
        return result;
    }
    if !e.get(this, HkpWorld::m_kdTreeManager).is_null() {
        if e.get(this, HkpWorld::m_autoUpdateTree) {
            fn_00c91270(e, this, first, second);
        } else {
            fn_00c91250(e, this);
        }
    }
    0
}

/// Copies the reference-counted pointer `new` into the word at `slot`: a reference is added to
/// the new object, one removed from the one replaced.
fn replace_reference(e: &mut Engine, slot: u32, new: u32) {
    if new != 0 {
        e.call(ADD_REFERENCE, &args![new]);
    }
    let replaced = e.mem.u32(slot);
    if replaced != 0 {
        e.call(REMOVE_REFERENCE, &args![replaced]);
    }
    e.mem.set_u32(slot, new);
}

// Translated from 00c91c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Fills the structure at `info` from the world's settings (the layout looks like
/// the world construction info; unconfirmed): vectors and words copied as they are, `float`s
/// through `FLD`/`FSTP`, the collision filter (+0x7c, into +0x58), the convex-list filter
/// (+0x84, into +0x5c) and the memory watch-dog (+0x50, into +0x6c) with a reference added
/// to each new one and removed from each one replaced, the border's two bytes (+0x30, +0x34;
/// 3 and 0 without a border), `info+0x50` = kd-tree manager present, and
/// `info+0x88` = the square root (`00ec6040`) of the step info float at +0x308 (the float at
/// `info+0x90` is written twice, the second value, from +0xd0, stays).
pub fn fn_00c91c50(e: &mut Engine, this: Ptr<HkpWorld>, info: Ptr) {
    let world = this.addr();
    let info = info.addr();
    copy_vector(e, world + 0x10, info + 0x10);
    copy_word(e, world + 0x338, info + 0x20);
    copy_vector(e, world + 0x310, info + 0x30);
    copy_vector(e, world + 0x320, info + 0x40);
    let collision_input = e.get(this, HkpWorld::m_collisionInput).addr();
    copy_float(e, collision_input + 8, info + 0x54);
    let filter = e.get(this, HkpWorld::m_collisionFilter).addr();
    replace_reference(e, info + 0x58, filter);
    let convex = e.get(this, HkpWorld::m_convexListFilter).addr();
    replace_reference(e, info + 0x5c, convex);
    let border = e.get(this, HkpWorld::m_broadPhaseBorder).addr();
    let border_kind = if border == 0 {
        3
    } else {
        e.mem.u32(border + 0x30) as u8
    };
    e.mem.set_u8(info + 0x28, border_kind);
    let border_flag = if border == 0 {
        0
    } else {
        e.mem.u8(border + 0x34)
    };
    e.mem.set_u8(info + 0x29, border_flag);
    let tree_present = !e.get(this, HkpWorld::m_kdTreeManager).is_null();
    e.mem.set_u8(info + 0x50, tree_present as u8);
    copy_byte(e, world + 0x60, info + 0x53);
    let dispatcher = e.get(this, HkpWorld::m_collisionDispatcher).addr();
    copy_float(e, dispatcher + 0x1e40, info + 0x60);
    copy_float(e, dispatcher + 0x1e44, info + 0x68);
    copy_float(e, world + 0xcc, info + 0x90);
    copy_float(e, world + 0xd0, info + 0x90);
    copy_byte(e, world + 0xd4, info + 0x98);
    let watchdog = e.get(this, HkpWorld::m_memoryWatchDog).addr();
    replace_reference(e, info + 0x6c, watchdog);
    copy_word(e, world + 0x330, info + 0x70);
    copy_word(e, world + 0x334, info + 0x64);
    copy_byte(e, world + 0x340, info + 0x74);
    copy_byte(e, collision_input + 0x6d, info + 0x75);
    copy_float(e, world + DYNAMICS_FLOAT_240, info + 0x24);
    copy_float(e, world + DYNAMICS_FLOAT_1E4, info + 0x78);
    copy_float(e, world + DYNAMICS_FLOAT_1E8, info + 0x7c);
    copy_word(e, world + DYNAMICS_WORD_2F4, info + 0x80);
    copy_word(e, world + DYNAMICS_WORD_2F8, info + 0x84);
    let argument = x87_quiet(e.mem.f32(world + DYNAMICS_FLOAT_308));
    let root = e.call(X87_SQRT, &args![argument as f64]).f32();
    e.mem.set_f32(info + 0x88, x87_quiet(root));
    copy_byte(e, world + DYNAMICS_BYTE_304, info + 0x8c);
    copy_byte(e, world + DYNAMICS_BYTES_305, info + 0xa4);
    copy_byte(e, world + DYNAMICS_BYTES_305 + 1, info + 0xa5);
    copy_byte(e, world + DYNAMICS_BYTES_305 + 2, info + 0xa6);
    // The collision input's pointer at +0x70 leads to a float and a word.
    let input_extra = e.mem.u32(collision_input + 0x70);
    copy_float(e, input_extra, info + 0x9c);
    copy_word(e, input_extra + 4, info + 0xa0);
    copy_byte(e, collision_input + 0x6c, info + 0x99);
    copy_byte(e, world + 0xd6, info + 0xa7);
    copy_float(e, world + 0xdc, info + 0xac);
    copy_word(e, world + 0xe0, info + 0xb0);
    copy_byte(e, world + 0xe4, info + 0xb4);
    copy_word(e, world + 0xe8, info + 0xb8);
    copy_word(e, world + 0xec, info + 0xbc);
    copy_float(e, world + 0xd8, info + 0xa8);
    copy_float(e, world + 0xf4, info + 0xc0);
    copy_float(e, world + 0xf8, info + 0xc4);
    copy_float(e, world + 0xfc, info + 0xc8);
    copy_float(e, world + 0x100, info + 0xcc);
    copy_byte(e, world + 0xd5, info + 0xd0);
    copy_byte(e, world + 0xf0, info + 0xd1);
    let simulation = e.get(this, HkpWorld::m_simulation).addr();
    copy_float(e, simulation + 0x24, info + 0xdc);
    copy_byte(e, world + 0xc8, info + 0xd2);
    copy_byte(e, world + 0xb0, info + 0xd8);
    copy_byte(e, world + 0xb1, info + 0xd9);
    copy_word(e, world + 0xb4, info + 0xd4);
    let callbacks = !e.get(this, HkpWorld::m_collisionCallbackUtil).is_null();
    e.mem.set_u8(info + 0xe0, callbacks as u8);
}

// Translated from 00c91f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::castRay` (Xbox PDB; the collector variant): brackets the work in the profiling
/// records `"TtworldCastRayCollector"` and `"Et"`. With a kd-tree manager in use (+0x5c,
/// slot +0x10) its slot +0x18 gets `(input, collision filter, collector)`; otherwise a
/// ray-caster object (vtable `010c3bd4`, words at +0x40 and +0x44 zero) runs
/// `00cbf860(broad phase, input, filter, 0, collector)`.
pub fn hkp_world_cast_ray(e: &mut Engine, this: Ptr<HkpWorld>, input: u32, collector: u32) {
    profile_record(e, PROFILE_CAST_RAY_COLLECTOR);
    let filter = e.get(this, HkpWorld::m_collisionFilter).addr();
    if kd_tree_in_use(e, this) {
        let tree = e.get(this, HkpWorld::m_kdTreeManager);
        e.vcall(tree.addr(), 0x18, &args![input, filter, collector]);
    } else {
        let broad_phase = e.get(this, HkpWorld::m_broadPhase);
        e.with_stack(0x54, |e, caster| {
            e.mem.set_u32(caster.addr(), RAY_CASTER_VTABLE);
            e.mem.set_u32(caster.addr() + 0x40, 0);
            e.mem.set_u32(caster.addr() + 0x44, 0);
            e.call(
                WORLD_RAY_CASTER_CAST_RAY,
                &args![caster, broad_phase, input, filter, 0u32, collector],
            );
        });
    }
    profile_record(e, PROFILE_END);
}

// Translated from 00c92040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::castRay_ov2` (Xbox PDB; the plain variant): as `castRay` with the record
/// `"TtworldCastRay"`, the kd-tree manager's slot +0x1c, and a simple ray-caster object
/// (vtable `010c647c`, words at +0x30 and +0x34 zero) running `00cfab30`.
pub fn hkp_world_cast_ray_ov2(e: &mut Engine, this: Ptr<HkpWorld>, input: u32, output: u32) {
    profile_record(e, PROFILE_CAST_RAY);
    let filter = e.get(this, HkpWorld::m_collisionFilter).addr();
    if kd_tree_in_use(e, this) {
        let tree = e.get(this, HkpWorld::m_kdTreeManager);
        e.vcall(tree.addr(), 0x1c, &args![input, filter, output]);
    } else {
        let broad_phase = e.get(this, HkpWorld::m_broadPhase);
        e.with_stack(0x44, |e, caster| {
            e.mem.set_u32(caster.addr(), SIMPLE_RAY_CASTER_VTABLE);
            e.mem.set_u32(caster.addr() + 0x30, 0);
            e.mem.set_u32(caster.addr() + 0x34, 0);
            e.call(
                SIMPLE_RAY_CASTER_CAST_RAY,
                &args![caster, broad_phase, input, filter, 0u32, output],
            );
        });
    }
    profile_record(e, PROFILE_END);
}

/// Tail of the three job dispatchers: waits for the jobs. Calls the notifier object's vtable
/// slot +0xc with `(queue, 0xb)` and slot +0x10, acquires the semaphore, and frees the
/// 0x10-byte completion block.
fn finish_job_dispatch(e: &mut Engine, queue: u32, notifier: u32, semaphore: u32, block: u32) {
    e.vcall(notifier, 0xc, &args![queue, 0xbu32]);
    e.vcall(notifier, 0x10, &args![]);
    e.call(SEMAPHORE_ACQUIRE, &args![semaphore]);
    router_free(e, block, 0x10);
}

/// Where a job keeps what the batching changes.
struct JobBatching {
    /// The job (game memory) and its size in bytes.
    job: u32,
    size: u32,
    /// Offsets of the element count and of the element pointer inside the job.
    count_offset: u32,
    pointer_offset: u32,
    /// Elements per batch, and bytes the element pointer advances per batch.
    batch: i32,
    stride: u32,
}

/// Splits the job into batches and adds each to `queue` (`00d04b90(queue, copy, 1)`): the
/// copy handed to the queue carries the smaller of the remaining count and the batch size;
/// the job's element pointer advances by the stride and its count drops by one batch per
/// round, until nothing is left.
fn add_batched_jobs(e: &mut Engine, queue: u32, batching: &JobBatching) {
    let job = batching.job;
    let mut count = e.mem.i32(job + batching.count_offset);
    if count <= 0 {
        return;
    }
    e.with_stack(batching.size, |e, copy| loop {
        let bytes = e.mem.bytes(job, batching.size);
        e.mem.write(copy.addr(), &bytes);
        let batch_count = if count < batching.batch {
            count
        } else {
            batching.batch
        };
        e.mem
            .set_i32(copy.addr() + batching.count_offset, batch_count);
        let pointer = e.mem.u32(job + batching.pointer_offset);
        e.mem.set_u32(
            job + batching.pointer_offset,
            pointer.wrapping_add(batching.stride),
        );
        count = count.wrapping_sub(batching.batch);
        e.mem.set_i32(job + batching.count_offset, count);
        e.call(JOB_QUEUE_ADD_JOB, &args![queue, copy, 1u32]);
        if count <= 0 {
            break;
        }
    });
}

// Translated from 00c92110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Dispatches `count` elements at `array` as batched jobs on `queue`: allocates a
/// 0x10-byte block whose first word is the number of 64-element jobs, builds a 0x50-byte job
/// (bytes 1, 3, 1, size 0x50, id 0xffff, the semaphore, the block, the collision input
/// `+0x78`, 0x40, the array, the count, the broad phase at +0x34; a kd-tree manager in use
/// swaps the broad phase for its words +0xc/+0x10 at +0x3c/+0x40), adds a copy per `batch`
/// elements (`batch * 0x40` bytes further each), then waits through `notifier` and
/// `semaphore`. The job's bytes the original never writes (+3, +8..+0xf, +0x44..+0x4f) are
/// stack garbage there and zero here.
#[allow(clippy::too_many_arguments)]
pub fn fn_00c92110(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    array: u32,
    count: i32,
    queue: u32,
    notifier: u32,
    semaphore: u32,
    batch: i32,
) {
    let block = router_alloc(e, 0x10);
    let collision_input = e.get(this, HkpWorld::m_collisionInput).addr();
    let broad_phase = e.get(this, HkpWorld::m_broadPhase).addr();
    e.mem.set_i32(block, job_block_count(count));
    e.with_stack(0x50, |e, job| {
        let job = job.addr();
        e.mem.set_u8(job, 1);
        e.mem.set_u8(job + 1, 3);
        e.mem.set_u8(job + 2, 1);
        e.mem.set_u16(job + 4, 0x50);
        e.mem.set_u16(job + 6, 0xffff);
        e.mem.set_u32(job + 0x10, semaphore);
        e.mem.set_u32(job + 0x14, block);
        e.mem.set_u32(job + 0x1c, collision_input);
        e.mem.set_u32(job + 0x20, 0x40);
        e.mem.set_u32(job + 0x24, array);
        e.mem.set_u32(job + 0x28, count as u32);
        e.mem.set_u32(job + 0x34, broad_phase);
        let tree = e.get(this, HkpWorld::m_kdTreeManager);
        if !tree.is_null() && e.vcall(tree.addr(), 0x10, &args![]).bool() {
            let first = e.mem.u32(tree.addr() + 0xc);
            let second = e.mem.u32(tree.addr() + 0x10);
            e.mem.set_u32(job + 0x3c, first);
            e.mem.set_u32(job + 0x40, second);
            e.mem.set_u32(job + 0x34, 0);
        }
        let batching = JobBatching {
            job,
            size: 0x50,
            count_offset: 0x28,
            pointer_offset: 0x24,
            batch,
            stride: (batch as u32).wrapping_shl(6),
        };
        add_batched_jobs(e, queue, &batching);
    });
    finish_job_dispatch(e, queue, notifier, semaphore, block);
}

// Translated from 00c92290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Like `00c92110` with a 0x40-byte job (bytes 2, 2, 2, size 0x40, id 0xffff, the
/// semaphore, the block, the collision input at +0x18, the broad phase at +0x20, 0x40 at
/// +0x30, the array at +0x34, the count at +0x38, and at +0x2c the `float` at +8 of the
/// collision input through `FLD`/`FSTP`) after clearing the word at +0xc of each 0x10-byte
/// element of `array`; the job header is initialized by `00d05de0`; a kd-tree manager in use
/// puts its words +0xc/+0x10 at +0x24/+0x28 and clears +0x20. Batches advance `batch * 0x10`
/// bytes. Bytes the original leaves as stack garbage (+3, +8..+0xf, +0x3c) are zero here.
#[allow(clippy::too_many_arguments)]
pub fn fn_00c92290(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    array: u32,
    count: i32,
    queue: u32,
    notifier: u32,
    semaphore: u32,
    batch: i32,
) {
    let block = router_alloc(e, 0x10);
    let collision_input = e.get(this, HkpWorld::m_collisionInput).addr();
    let tolerance = x87_quiet(e.mem.f32(collision_input + 8));
    let broad_phase = e.get(this, HkpWorld::m_broadPhase).addr();
    for index in 0..count.max(0) as u32 {
        e.mem.set_u32(array.wrapping_add(0xc + 0x10 * index), 0);
    }
    e.mem.set_i32(block, job_block_count(count));
    e.with_stack(0x40, |e, job| {
        let job = job.addr();
        e.mem.set_u8(job, 2);
        e.mem.set_u8(job + 1, 2);
        e.mem.set_u8(job + 2, 2);
        e.mem.set_u16(job + 4, 0x40);
        e.mem.set_u16(job + 6, 0xffff);
        e.mem.set_u32(job + 0x10, semaphore);
        e.mem.set_u32(job + 0x14, block);
        e.mem.set_u32(job + 0x18, collision_input);
        e.mem.set_u32(job + 0x20, broad_phase);
        e.mem.set_f32(job + 0x2c, tolerance);
        e.mem.set_u32(job + 0x30, 0x40);
        e.mem.set_u32(job + 0x34, array);
        e.mem.set_u32(job + 0x38, count as u32);
        e.call(JOB_HEADER_INIT, &args![job]);
        let tree = e.get(this, HkpWorld::m_kdTreeManager);
        if !tree.is_null() && e.vcall(tree.addr(), 0x10, &args![]).bool() {
            let first = e.mem.u32(tree.addr() + 0xc);
            let second = e.mem.u32(tree.addr() + 0x10);
            e.mem.set_u32(job + 0x24, first);
            e.mem.set_u32(job + 0x28, second);
            e.mem.set_u32(job + 0x20, 0);
        }
        let batching = JobBatching {
            job,
            size: 0x40,
            count_offset: 0x38,
            pointer_offset: 0x34,
            batch,
            stride: (batch as u32).wrapping_shl(4),
        };
        add_batched_jobs(e, queue, &batching);
    });
    finish_job_dispatch(e, queue, notifier, semaphore, block);
}

// Translated from 00c92430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::linearCast` (Xbox PDB): brackets the work in the records `"TtworldLinCast"` and
/// `"Et"`. The collision filter's base (`+7c`, plus 8, or 0) and the collision input's word
/// at +0x70 are passed on. With a kd-tree manager in use its slot +0x20 gets `(a, b, filter
/// base, collision input, input+0x70, c, d)`; otherwise a linear-caster object (vtable
/// `010c6488`, +0x24 = 0, +0x80 = the `float` at `0104da4c`) runs `00cfaed0(broad phase, a,
/// b, filter base, collision input, input+0x70, 0, c, d)`.
pub fn hkp_world_linear_cast(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
) {
    profile_record(e, PROFILE_LINEAR_CAST);
    let filter = e.get(this, HkpWorld::m_collisionFilter).addr();
    let filter_base = if filter == 0 { 0 } else { filter + 8 };
    let collision_input = e.get(this, HkpWorld::m_collisionInput).addr();
    if kd_tree_in_use(e, this) {
        let tree = e.get(this, HkpWorld::m_kdTreeManager);
        let extra = e.mem.u32(collision_input + 0x70);
        e.vcall(
            tree.addr(),
            0x20,
            &args![
                first,
                second,
                filter_base,
                collision_input,
                extra,
                third,
                fourth
            ],
        );
    } else {
        let default_value = x87_quiet(e.global::<f32>(LINEAR_CASTER_DEFAULT));
        let broad_phase = e.get(this, HkpWorld::m_broadPhase);
        let extra = e.mem.u32(collision_input + 0x70);
        // The caster runs past +0x84: 00cfaed0 also writes +0x84 and the word at +0x88 (the exe
        // keeps it at ESP+0x10 of 00c92430, its +0x80 at ESP+0x90).
        e.with_stack(0x90, |e, caster| {
            e.mem.set_f32(caster.addr() + 0x80, default_value);
            e.mem.set_u32(caster.addr(), LINEAR_CASTER_VTABLE);
            e.mem.set_u32(caster.addr() + 0x24, 0);
            e.call(
                LINEAR_CASTER_LINEAR_CAST,
                &args![
                    caster,
                    broad_phase,
                    first,
                    second,
                    filter_base,
                    collision_input,
                    extra,
                    0u32,
                    third,
                    fourth
                ],
            );
        });
    }
    profile_record(e, PROFILE_END);
}

// Translated from 00c92540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Like `00c92110` with a 0x40-byte job (bytes 4, 2, 1, size 0x40, id 0xffff, the
/// semaphore, the block, the collision input at +0x18, 0x40 at +0x20, the array at +0x24,
/// the count at +0x28, the broad phase at +0x2c); a kd-tree manager in use clears +0x2c and
/// puts its words +0xc/+0x10 at +0x34/+0x38. Batches advance `batch * 0x30` bytes. Bytes the
/// original leaves as stack garbage (+3, +8..+0xf, +0x3c) are zero here.
#[allow(clippy::too_many_arguments)]
pub fn fn_00c92540(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    array: u32,
    count: i32,
    queue: u32,
    notifier: u32,
    semaphore: u32,
    batch: i32,
) {
    let block = router_alloc(e, 0x10);
    let collision_input = e.get(this, HkpWorld::m_collisionInput).addr();
    let broad_phase = e.get(this, HkpWorld::m_broadPhase).addr();
    e.mem.set_i32(block, job_block_count(count));
    e.with_stack(0x40, |e, job| {
        let job = job.addr();
        e.mem.set_u8(job, 4);
        e.mem.set_u8(job + 1, 2);
        e.mem.set_u16(job + 4, 0x40);
        e.mem.set_u16(job + 6, 0xffff);
        e.mem.set_u32(job + 0x10, semaphore);
        e.mem.set_u32(job + 0x14, block);
        e.mem.set_u32(job + 0x18, collision_input);
        e.mem.set_u32(job + 0x20, 0x40);
        e.mem.set_u32(job + 0x24, array);
        e.mem.set_u32(job + 0x28, count as u32);
        e.mem.set_u32(job + 0x2c, broad_phase);
        let tree = e.get(this, HkpWorld::m_kdTreeManager);
        if !tree.is_null() && e.vcall(tree.addr(), 0x10, &args![]).bool() {
            let first = e.mem.u32(tree.addr() + 0xc);
            let second = e.mem.u32(tree.addr() + 0x10);
            e.mem.set_u32(job + 0x2c, 0);
            e.mem.set_u32(job + 0x34, first);
            e.mem.set_u32(job + 0x38, second);
        }
        e.mem.set_u8(job + 2, 1);
        let batching = JobBatching {
            job,
            size: 0x40,
            count_offset: 0x28,
            pointer_offset: 0x24,
            batch,
            stride: (batch as u32).wrapping_mul(0x30),
        };
        add_batched_jobs(e, queue, &batching);
    });
    finish_job_dispatch(e, queue, notifier, semaphore, block);
}

/// Adds every entity of the island's array (data at +0x48, size at +0x4c) to the physics
/// system as a rigid body.
fn add_island_bodies(e: &mut Engine, island: u32, system: u32) {
    let mut index = 0i32;
    while index < e.mem.i32(island + 0x4c) {
        let data = e.mem.u32(island + 0x48);
        let body = array_entry(e, data, index);
        e.call(PHYSICS_SYSTEM_ADD_RIGID_BODY, &args![system, body]);
        index += 1;
    }
}

// Translated from 00c926a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl `(out, world, system)`, returns `out`). Adds the rigid bodies of the
/// world's fixed island (+0x20; the first one only when its word at +0x10 is not null) and
/// of every inactive island (+0x34) to the physics system (`00d064a0`); `*out` is 1 when
/// the fixed island had bodies or there are inactive islands.
pub fn fn_00c926a0(e: &mut Engine, out: Ptr, world: Ptr<HkpWorld>, system: Ptr) -> Ptr {
    let fixed = e.get(world, HkpWorld::m_fixedIsland).addr();
    let mut fixed_had_bodies = false;
    if fixed != 0 {
        let mut index = 0i32;
        while index < e.mem.i32(fixed + 0x4c) {
            let data = e.mem.u32(fixed + 0x48);
            let body = array_entry(e, data, index);
            let first_body = e.mem.u32(data);
            if index != 0 || e.mem.u32(first_body + 0x10) != 0 {
                e.call(PHYSICS_SYSTEM_ADD_RIGID_BODY, &args![system, body]);
            }
            index += 1;
        }
        fixed_had_bodies = e.mem.i32(fixed + 0x4c) > 0;
    }
    let mut cursor = e.get(world, HkpWorld::m_inactiveSimulationIslands).addr();
    loop {
        let data = e.get(world, HkpWorld::m_inactiveSimulationIslands).addr();
        let size = e.get(world, HkpWorld::m_inactiveSimulationIslandsSize);
        if cursor == data.wrapping_add((size as u32).wrapping_mul(4)) {
            break;
        }
        let island = e.mem.u32(cursor);
        add_island_bodies(e, island, system.addr());
        cursor = cursor.wrapping_add(4);
    }
    let any = e.get(world, HkpWorld::m_inactiveSimulationIslandsSize) > 0 || fixed_had_bodies;
    e.mem.set_u8(out.addr(), any as u8);
    out
}

// Translated from 00c92760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl `(out, world, system)`, returns `out`). Adds the rigid bodies of every
/// active island (+0x28) of the world to the physics system; `*out` is 1 when there are
/// active islands.
pub fn fn_00c92760(e: &mut Engine, out: Ptr, world: Ptr<HkpWorld>, system: Ptr) -> Ptr {
    let mut cursor = e.get(world, HkpWorld::m_activeSimulationIslands).addr();
    loop {
        let data = e.get(world, HkpWorld::m_activeSimulationIslands).addr();
        let size = e.get(world, HkpWorld::m_activeSimulationIslandsSize);
        if cursor == data.wrapping_add((size as u32).wrapping_mul(4)) {
            break;
        }
        let island = e.mem.u32(cursor);
        add_island_bodies(e, island, system.addr());
        cursor = cursor.wrapping_add(4);
    }
    let any = e.get(world, HkpWorld::m_activeSimulationIslandsSize) > 0;
    e.mem.set_u8(out.addr(), any as u8);
    out
}

// Translated from 00c927c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c927c0 - takes ESI (the physics system) and EDI (the world) as register inputs, which have no uniform-ABI form; translated as a Rust function with them as parameters and not registered.
/// Unnamed fragment outlined from `00c929e0`: runs `00c926a0` and `00c92760` on the world
/// into a local flag, then stores the second flag (active islands present) in the byte at
/// +0x40 of the physics system.
pub fn fn_00c927c0(e: &mut Engine, system: Ptr, world: Ptr<HkpWorld>) {
    e.with_stack(8, |e, flag| {
        fn_00c926a0(e, flag, world, system);
        let flag = fn_00c92760(e, flag, world, system);
        let value = e.mem.u8(flag.addr());
        e.mem.set_u8(system.addr() + 0x40, value);
    });
}

// Translated from 00c927f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl `(island, system)`). For every entity of the island (array at +0x48, size
/// +0x4c) and each of the entity's constraint entries (an array of 0x30-byte entries: data
/// at +0xac, count as a word at +0xb0): follows the entry's pointer at +0xc through the
/// chain of words at +0x14 while the entry's leading word is at least 0x17, and adds the
/// entry's first word to the physics system (`00d06540`) unless the chain ends in a leading
/// word of 0x16.
pub fn fn_00c927f0(e: &mut Engine, island: Ptr, system: Ptr) {
    let island = island.addr();
    let mut entity_index = 0i32;
    while entity_index < e.mem.i32(island + 0x4c) {
        let entities = e.mem.u32(island + 0x48);
        let entity = array_entry(e, entities, entity_index);
        let mut constraint = 0i32;
        while constraint < e.mem.u16(entity + 0xb0) as i32 {
            let data = e.mem.u32(entity + 0xac);
            let entry = data.wrapping_add(0x30 * constraint as u32);
            let mut chain = e.mem.u32(entry + 0xc);
            while e.mem.u16(chain) >= 0x17 {
                chain = e.mem.u32(chain + 0x14);
            }
            if e.mem.u16(chain) != 0x16 {
                let first = e.mem.u32(entry);
                e.call(PHYSICS_SYSTEM_ADD_CONSTRAINT, &args![system, first]);
            }
            constraint += 1;
        }
        entity_index += 1;
    }
}

// Translated from 00c92860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c92860 - takes ESI (the world) as a register input, which has no uniform-ABI form; translated as a Rust function with it as the first parameter and not registered.
/// Unnamed fragment outlined from `00c929e0` (`ESI` = the world, the physics system on the
/// stack): runs `00c927f0` for every active island (+0x28) and then every inactive island
/// (+0x34).
pub fn fn_00c92860(e: &mut Engine, world: Ptr<HkpWorld>, system: Ptr) {
    let mut cursor = e.get(world, HkpWorld::m_activeSimulationIslands).addr();
    loop {
        let data = e.get(world, HkpWorld::m_activeSimulationIslands).addr();
        let size = e.get(world, HkpWorld::m_activeSimulationIslandsSize);
        if cursor == data.wrapping_add((size as u32).wrapping_mul(4)) {
            break;
        }
        let island = e.mem.u32(cursor);
        fn_00c927f0(e, Ptr::new(island), system);
        cursor = cursor.wrapping_add(4);
    }
    let mut cursor = e.get(world, HkpWorld::m_inactiveSimulationIslands).addr();
    loop {
        let data = e.get(world, HkpWorld::m_inactiveSimulationIslands).addr();
        let size = e.get(world, HkpWorld::m_inactiveSimulationIslandsSize);
        if cursor == data.wrapping_add((size as u32).wrapping_mul(4)) {
            break;
        }
        let island = e.mem.u32(cursor);
        fn_00c927f0(e, Ptr::new(island), system);
        cursor = cursor.wrapping_add(4);
    }
}

/// Adds every action of the island's action array (data at +0x38, size at +0x3c) to the
/// physics system (`00d06590`).
fn add_island_actions(e: &mut Engine, island: u32, system: u32) {
    let mut index = 0i32;
    while index < e.mem.i32(island + 0x3c) {
        let data = e.mem.u32(island + 0x38);
        let action = array_entry(e, data, index);
        e.call(PHYSICS_SYSTEM_ADD_ACTION, &args![system, action]);
        index += 1;
    }
}

// Translated from 00c928c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl `(world, system)`). Adds the actions of every active island (+0x28) and
/// then of every inactive island (+0x34) to the physics system.
pub fn fn_00c928c0(e: &mut Engine, world: Ptr<HkpWorld>, system: Ptr) {
    let mut cursor = e.get(world, HkpWorld::m_activeSimulationIslands).addr();
    loop {
        let data = e.get(world, HkpWorld::m_activeSimulationIslands).addr();
        let size = e.get(world, HkpWorld::m_activeSimulationIslandsSize);
        if cursor == data.wrapping_add((size as u32).wrapping_mul(4)) {
            break;
        }
        let island = e.mem.u32(cursor);
        add_island_actions(e, island, system.addr());
        cursor = cursor.wrapping_add(4);
    }
    let mut cursor = e.get(world, HkpWorld::m_inactiveSimulationIslands).addr();
    loop {
        let data = e.get(world, HkpWorld::m_inactiveSimulationIslands).addr();
        let size = e.get(world, HkpWorld::m_inactiveSimulationIslandsSize);
        if cursor == data.wrapping_add((size as u32).wrapping_mul(4)) {
            break;
        }
        let island = e.mem.u32(cursor);
        add_island_actions(e, island, system.addr());
        cursor = cursor.wrapping_add(4);
    }
}

// Translated from 00c92950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c92950 - takes EDI (the world) as a register input, which has no uniform-ABI form; translated as a Rust function with it as the first parameter and not registered.
/// Unnamed fragment outlined from `00c929e0` (`EDI` = the world, the physics system on the
/// stack): adds the world's phantoms (array at +0x110, size +0x114) to the physics system
/// (`00d064f0`), skipping those equal to one of the six words at +0x18..+0x2c of the broad
/// phase border (+0x1bc) when the world has one.
pub fn fn_00c92950(e: &mut Engine, world: Ptr<HkpWorld>, system: Ptr) {
    let border = fn_00c91110(e, world).addr();
    let mut index = 0i32;
    while index < e.get(world, HkpWorld::m_phantomsSize) {
        let data = e.get(world, HkpWorld::m_phantoms).addr();
        let phantom = array_entry(e, data, index);
        let skipped =
            border != 0 && (0..6).any(|slot| e.mem.u32(border + 0x18 + 4 * slot) == phantom);
        if !skipped {
            e.call(PHYSICS_SYSTEM_ADD_PHANTOM, &args![system, phantom]);
        }
        index += 1;
    }
}

// Translated from 00c929e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Builds a physics system holding the world's contents: allocates 0x44 bytes
/// through the memory router (word at +4 = 0x44), constructs it (`00d065e0`), then adds the
/// bodies (`00c927c0`), phantoms (`00c92950`), constraints (`00c92860`) and actions
/// (`00c928c0`) of the world. Returns the system.
pub fn fn_00c929e0(e: &mut Engine, this: Ptr<HkpWorld>) -> Ptr {
    let block = router_alloc(e, 0x44);
    e.mem.set_u16(block + 4, 0x44);
    let system = e.call(PHYSICS_SYSTEM_CONSTRUCT, &args![block]).ptr::<()>();
    fn_00c927c0(e, system, this);
    fn_00c92950(e, this, system);
    fn_00c92860(e, this, system);
    fn_00c928c0(e, this, system);
    system
}

// ---------------------------------------------------------------------------------------
// Callees and helpers of the actions, constraints, phantoms and collision filter updates
// (00c92c70 and the functions after it in address order).
// ---------------------------------------------------------------------------------------

/// `00c908c0(allocator, array, count, element_size)` and `00c90940(allocator, array,
/// element_size)`: reserve room for `count` elements in an `hkArray` / grow it by one
/// step (identities unconfirmed).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_GROW: u32 = 0x00c9_0940;
/// `00d003c0(world, action)` and `00d00460(world, action)`: the world's action added and
/// removed callbacks. `00d00780` / `00d00820` are the same for phantoms.
const FIRE_ACTION_ADDED: u32 = 0x00d0_03c0;
const FIRE_ACTION_REMOVED: u32 = 0x00d0_0460;
const FIRE_PHANTOM_ADDED: u32 = 0x00d0_0780;
const FIRE_PHANTOM_REMOVED: u32 = 0x00d0_0820;
/// `00d07130(island, action)` and `00d07470(island, action)`: take an action out of an
/// island / put it in (both with `this` = island, one stack argument).
const ISLAND_REMOVE_ACTION: u32 = 0x00d0_7130;
const ISLAND_ADD_ACTION: u32 = 0x00d0_7470;
/// `00d06cb0(array, element_size)`: grows an entity's action list.
const ACTION_LIST_GROW: u32 = 0x00d0_6cb0;
/// `00cb5570(world, first_entity, entity)`: merges the islands of two entities.
const MERGE_ISLANDS: u32 = 0x00cb_5570;
/// `00cf3e60(constraint)`, `00cb3ca0(world, constraint, 1)`, `00cb3dd0(world, constraint,
/// 1)` and `00cf3040(constraint, world)`: constraint bookkeeping around adding / removing.
const CONSTRAINT_PREPARE: u32 = 0x00cf_3e60;
const ADD_CONSTRAINT_SI: u32 = 0x00cb_3ca0;
const REMOVE_CONSTRAINT_SI: u32 = 0x00cb_3dd0;
const CONSTRAINT_DETACH: u32 = 0x00cf_3040;
/// Phantom steps: `00cb5b10(world, phantom)` / `00cb5d00(world, phantom)` add it to /
/// remove it from the broad phase; `00d07b90(phantom)` / `00d07b50(phantom)` are the steps
/// after adding / before removing; `00d07950(phantom, aabb)` sets its bounds.
const ADD_PHANTOM_BP: u32 = 0x00cb_5b10;
const REMOVE_PHANTOM_BP: u32 = 0x00cb_5d00;
const PHANTOM_AFTER_ADD: u32 = 0x00d0_7b90;
const PHANTOM_BEFORE_REMOVE: u32 = 0x00d0_7b50;
const PHANTOM_SET_AABB: u32 = 0x00d0_7950;
/// `00cb1260(destination, source, size)`: a memory copy.
const MEM_COPY: u32 = 0x00cb_1260;
/// `00c9c1d0(entity_collidable_owner)`: called by `activateRegion` on the bodies found.
const ACTIVATE_BODY: u32 = 0x00c9_c1d0;
/// `00d08210(collidable)` and `00d081e0(collidable)`: the collidable's agent list.
const COLLIDABLE_RESET_AGENTS: u32 = 0x00d0_8210;
const COLLIDABLE_AGENT_ENTRIES: u32 = 0x00d0_81e0;
/// `00cf7130(pairs, other_pairs)` removes the pairs of the first array from the second.
const REMOVE_DUPLICATE_PAIRS: u32 = 0x00cf_7130;
/// `00cc1160(agent_entry)`: removes an agent and its time-of-impact events.
const REMOVE_AGENT: u32 = 0x00cc_1160;
/// `00cfeba0(agent_entry, collision_input, island)`: updates a shape-collection agent.
const UPDATE_SHAPE_COLLECTION_FILTER: u32 = 0x00cf_eba0;
/// `00d07830(block, world)` constructs a simulation island, `00d07420(island, entity)` adds
/// an entity to it.
const ISLAND_CONSTRUCT: u32 = 0x00d0_7830;
const ISLAND_ADD_ENTITY: u32 = 0x00d0_7420;
/// `00cf70f0(dispatcher, data, size)`: the broad phase pairs to remove.
const REMOVE_PAIRS: u32 = 0x00cf_70f0;

/// Profiling labels of the functions below (strings in the exe).
const PROFILE_UPDATE_FILTER_ON_WORLD: u32 = 0x010c_6908;
const PROFILE_UPDATE_FILTER_ON_PHANTOM: u32 = 0x010c_6758;
const PROFILE_BROADPHASE_SECOND: u32 = 0x010c_6748;
const PROFILE_UPDATE_OVERLAPS: u32 = 0x010c_6734;
const PROFILE_COLLECTION_FILTER: u32 = 0x010c_6720;
const PROFILE_SECTION_END: u32 = 0x0107_6f38;
const PROFILE_UPDATE_FILTER_ON_ENTITY: u32 = 0x010c_679c;
const PROFILE_INIT: u32 = 0x010c_6794;
const PROFILE_PHANTOMS: u32 = 0x010c_6788;
const PROFILE_CHECK_AGENTS: u32 = 0x010c_677c;
const PROFILE_ADD_AGENTS: u32 = 0x010c_6770;
const PROFILE_ADD_ENTITIES: u32 = 0x010c_67e0;
const PROFILE_ADD_BROADPHASE: u32 = 0x010c_67d0;
const PROFILE_CREATE_AGENTS: u32 = 0x010c_67c0;
const PROFILE_ADDED_CALLBACKS: u32 = 0x010c_67b4;
const PROFILE_REMOVE_ENTITIES: u32 = 0x010c_6828;
const PROFILE_REMOVE_INIT: u32 = 0x010c_6818;
const PROFILE_REMOVE_BROADPHASE: u32 = 0x010c_6808;
const PROFILE_DELETE_AGENTS: u32 = 0x010c_67fc;
const PROFILE_REMOVE_CALLBACKS: u32 = 0x010c_67f0;

/// Appends the 16-byte profiling record of the functions that name a second label: `label`
/// at +0, the time stamp at +4, `second` at +0xC.
fn profile_record_pair(e: &mut Engine, label: u32, second: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 0xc, second);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0x10);
    }
}

/// Hands an operation record (built by `fill`, kind byte first) to the pending queue.
fn queue_operation(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    kind: u8,
    fill: impl FnOnce(&mut Engine, u32),
) {
    e.with_stack(0x20, |e, operation| {
        e.mem.set_u8(operation.addr(), kind);
        fill(e, operation.addr());
        fn_00c91160(e, this, operation);
    });
}

/// `m_criticalOperationsLockCount != 0`: the request has to be queued.
fn operations_locked(e: &mut Engine, this: Ptr<HkpWorld>) -> bool {
    e.get(this, HkpWorld::m_criticalOperationsLockCount) != 0
}

/// The phantom functions queue when the two lock counts (+0x94, +0x98) add up to non-zero.
fn phantom_operations_locked(e: &mut Engine, this: Ptr<HkpWorld>) -> bool {
    e.get(this, HkpWorld::m_phantomOperationsLockCount)
        .wrapping_add(e.get(this, HkpWorld::m_criticalOperationsLockCount))
        != 0
}

fn begin_critical_section(e: &mut Engine, this: Ptr<HkpWorld>) {
    let lock = e
        .get(this, HkpWorld::m_criticalOperationsLockCount)
        .wrapping_add(1);
    e.set(this, HkpWorld::m_criticalOperationsLockCount, lock);
}

/// Counts the lock down and, when it reaches 0, flushes the pending operations.
fn end_critical_section(e: &mut Engine, this: Ptr<HkpWorld>) {
    let lock = e
        .get(this, HkpWorld::m_criticalOperationsLockCount)
        .wrapping_sub(1);
    e.set(this, HkpWorld::m_criticalOperationsLockCount, lock);
    if lock == 0 {
        leave_critical_section(e, this);
    }
}

/// A signed byte offset applied to a pointer (`MOVSX` then `ADD`).
fn offset_by_byte(e: &Engine, base: u32, at: u32) -> u32 {
    base.wrapping_add(e.mem.i8(at) as i32 as u32)
}

/// Grows the `hkArray` at `array` by one step through `00c90940` when it is full.
fn array_make_room(e: &mut Engine, array: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_GROW, &args![allocator, array, element_size]);
    }
}

/// `array.pushBack(value)` for 4-byte elements.
fn array_push_word(e: &mut Engine, array: u32, value: u32) {
    array_make_room(e, array, 4);
    let data = e.mem.u32(array);
    let size = e.mem.u32(array + 4);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// Empties the array and returns its storage to the allocator at `router + allocator_offset`
/// unless the storage is the array's own (capacity flag bit 31 set).
fn array_release(e: &mut Engine, array: u32, element_size: u32, allocator_offset: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + allocator_offset);
    let data = e.mem.u32(array);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(element_size);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

/// Reserves `bytes` (rounded up to 0x80) from the router's bump cursor (`router[0]`) and
/// returns the cursor before the allocation.
fn bump_allocate(e: &mut Engine, bytes: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let saved = e.mem.u32(router);
    e.mem
        .set_u32(router, saved.wrapping_add(bytes.wrapping_add(0x7f) & !0x7f));
    saved
}

/// Starts an `hkArray` on the bump cursor: `capacity` elements of `element_size` bytes, the
/// storage flagged as not owned. Returns the saved cursor to give back at the end.
fn bump_array(e: &mut Engine, array: u32, capacity: u32, element_size: u32) -> u32 {
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let saved = bump_allocate(e, capacity.wrapping_mul(element_size));
    e.mem.set_u32(array, saved);
    e.mem.set_u32(array + 8, capacity | 0x8000_0000);
    saved
}

/// Gives a bump-allocated array back: an array still on its storage is emptied, the cursor
/// goes back to `saved`, then the array is released like any other (a grown array owns
/// heap storage and frees it).
fn bump_array_release(e: &mut Engine, array: u32, saved: u32, element_size: u32) {
    if e.mem.u32(array) == saved {
        e.mem.set_u32(array + 4, 0);
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.set_u32(router, saved);
    array_release(e, array, element_size, 0x10);
}

/// `m_collisionFilter + 8`, or 0 without a filter: the argument the pair creation takes.
fn filter_base(e: &Engine, world: u32) -> u32 {
    match e.mem.u32(world + 0x7c) {
        0 => 0,
        filter => filter + 8,
    }
}

/// The `bool` the collision filter's pair test (the vtable slot 4 of the object at
/// `filter + 8`) answers for the two collidables.
fn filter_accepts(e: &mut Engine, filter: u32, first: u32, second: u32) -> bool {
    e.with_stack(8, |e, scratch| {
        let answer = e
            .vcall(filter + 8, 4, &args![scratch.addr(), first, second])
            .u32();
        e.mem.u8(answer) != 0
    })
}

/// The collision group table lookup the filter updates make: a byte of the collision
/// dispatcher's table (+0x1bb0) indexed by the two collidables' group bytes (+0x1a).
fn group_pair_enabled(e: &Engine, world: u32, first_group: i8, second_group: i8) -> bool {
    let index = (second_group as i32) + (first_group as i32) * 10;
    let table = e.mem.u32(world + 0x80);
    e.mem
        .u8(table.wrapping_add(index as u32).wrapping_add(0x1bb0))
        != 0
}

// Translated from 00c90f40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c90f40 - takes EDI (the phantom) and ESI (the other collidable) as register inputs, which have no uniform-ABI form; translated as a Rust function with them as the leading parameters and not registered.
/// Piece of `hkpWorld::updateCollisionFilterOnPhantom` (Xbox PDB) that the compiler
/// outlined: `phantom` is in EDI, `other` (the other body's collidable owner, or the entity's
/// collidable) in ESI and the collision filter is the one stack argument. The pair is asked
/// of the filter; the phantom's overlap callbacks (vtable slots 0x1c / 0x24) and, when the
/// other body is a phantom (byte +0x18 = 2), that body's callbacks (slots 0x1c / 0x24, with
/// the phantom's collidable `phantom + 0x10`) are called so that both sides agree on
/// whether they overlap. Not registered: the register arguments cannot be called by
/// address.
pub fn fn_00c90f40(e: &mut Engine, phantom: u32, other: u32, filter: u32) {
    e.with_stack(8, |e, frame| {
        let flag = frame.addr();
        let scratch = frame.addr() + 4;
        e.mem.set_u32(scratch, filter);
        e.vcall(phantom, 0x20, &args![flag, other]);
        let collidable = phantom + 0x10;
        let answer = e
            .vcall(filter + 8, 4, &args![scratch, collidable, other])
            .u32();
        let accepted = e.mem.u8(answer) != 0;
        let was_overlapping = e.mem.u8(flag) != 0;
        if accepted {
            if !was_overlapping {
                e.vcall(phantom, 0x1c, &args![other]);
            }
        } else if was_overlapping {
            e.vcall(phantom, 0x24, &args![other]);
        }
        if e.mem.u8(other + 0x18) == 2 {
            let body = offset_by_byte(e, other, other + 0x10);
            e.vcall(body, 0x20, &args![scratch, collidable]);
            let overlapping = e.mem.u8(scratch) != 0;
            if accepted {
                if !overlapping {
                    e.vcall(body, 0x1c, &args![collidable]);
                }
            } else if overlapping {
                e.vcall(body, 0x24, &args![collidable]);
            }
        }
    });
}

// Translated from 00c92c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removePhantom` (Xbox PDB). Queued (operation 0xE, phantom at +4) while either
/// lock count (+0x94, +0x98) is non-zero. Otherwise: fires the phantom's "removed" callback
/// (`00d00820`), runs `00d07b50` on it, takes it out of the broad phase (`00cb5d00`), removes
/// it from `m_phantoms` (the last phantom moves into its place), clears its world pointer
/// (+8), deletes it through vtable slot 0x34 when its `m_memSizeAndFlags` is 0, removes the
/// reference and flushes the pending operations when the critical section ends.
pub fn hkp_world_remove_phantom(e: &mut Engine, this: Ptr<HkpWorld>, phantom: u32) {
    if phantom_operations_locked(e, this) {
        queue_operation(e, this, 0xe, |e, operation| {
            e.mem.set_u32(operation + 4, phantom);
        });
        return;
    }
    begin_critical_section(e, this);
    e.call(FIRE_PHANTOM_REMOVED, &args![this, phantom]);
    e.call(PHANTOM_BEFORE_REMOVE, &args![phantom]);
    e.call(REMOVE_PHANTOM_BP, &args![this, phantom]);
    remove_phantom_from_list(e, this, phantom);
    if e.mem.u16(phantom + 4) == 0 {
        e.vcall(phantom, 0x34, &args![]);
    }
    e.call(REMOVE_REFERENCE, &args![phantom]);
    end_critical_section(e, this);
}

/// Takes `phantom` out of `m_phantoms` (+0x110, size +0x114): its slot (-1 when absent, as
/// the exe indexes) gets the last entry, then the size is one less. Clears the phantom's
/// world pointer first.
fn remove_phantom_from_list(e: &mut Engine, this: Ptr<HkpWorld>, phantom: u32) {
    let world = this.addr();
    e.mem.set_u32(phantom + 8, 0);
    let size = e.mem.i32(world + 0x114);
    let data = e.mem.u32(world + 0x110);
    let mut index = -1i32;
    for candidate in 0..size.max(0) {
        if e.mem.u32(data.wrapping_add(candidate as u32 * 4)) == phantom {
            index = candidate;
            break;
        }
    }
    let new_size = size.wrapping_sub(1);
    e.mem.set_i32(world + 0x114, new_size);
    if new_size != index {
        let data = e.mem.u32(world + 0x110);
        let last = e
            .mem
            .u32(data.wrapping_add((new_size as u32).wrapping_mul(4)));
        e.mem
            .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), last);
    }
}

// Translated from 00c936a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::addPhantom` (Xbox PDB). Queued (operation 0xD, phantom at +4; returns 0)
/// while either lock count is non-zero. Otherwise: gives the phantom its motion-like object
/// when it has none (+0x18, vtable slot 0x10), sets its world pointer (+8), adds a
/// reference, appends it to `m_phantoms`, adds it to the broad phase (`00cb5b10`), fires the
/// world's "phantom added" callback (`00d00780`), runs `00d07b90` on it and flushes the
/// pending operations when the critical section ends. Returns the phantom.
pub fn hkp_world_add_phantom(e: &mut Engine, this: Ptr<HkpWorld>, phantom: u32) -> u32 {
    if phantom_operations_locked(e, this) {
        queue_operation(e, this, 0xd, |e, operation| {
            e.mem.set_u32(operation + 4, phantom);
        });
        return 0;
    }
    begin_critical_section(e, this);
    prepare_phantom(e, this, phantom);
    e.call(ADD_PHANTOM_BP, &args![this, phantom]);
    e.call(FIRE_PHANTOM_ADDED, &args![this, phantom]);
    e.call(PHANTOM_AFTER_ADD, &args![phantom]);
    end_critical_section(e, this);
    phantom
}

/// The first steps of `addPhantom`: the phantom's object at +0x18, the world pointer at +8,
/// the reference and the entry in `m_phantoms`.
fn prepare_phantom(e: &mut Engine, this: Ptr<HkpWorld>, phantom: u32) {
    if e.mem.u32(phantom + 0x18) == 0 {
        let made = e.vcall(phantom, 0x10, &args![]).u32();
        e.mem.set_u32(phantom + 0x18, made);
    }
    e.mem.set_u32(phantom + 8, this.addr());
    e.call(ADD_REFERENCE, &args![phantom]);
    array_push_word(e, this.addr() + 0x110, phantom);
}

// Translated from 00c93c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::activateRegion` (Xbox PDB). While critical operations are locked, copies the
/// 0x20-byte AABB into a router block and queues operation 0x20 with the copy at +4.
/// Otherwise asks the broad phase (vtable slot 0x44) for the collidables overlapping the
/// AABB and calls `00c9c1d0` on the body behind each one that is a rigid body (byte +0x18
/// = 1) with a non-null body pointer.
pub fn hkp_world_activate_region(e: &mut Engine, this: Ptr<HkpWorld>, aabb: u32) {
    if operations_locked(e, this) {
        let copy = router_alloc(e, 0x20);
        e.call(MEM_COPY, &args![copy, aabb, 0x20u32]);
        queue_operation(e, this, 0x20, |e, operation| {
            e.mem.set_u32(operation + 4, copy);
        });
        return;
    }
    let broad_phase = e.get(this, HkpWorld::m_broadPhase).addr();
    e.with_stack(0x10, |e, found| {
        let found = found.addr();
        e.mem.set_u32(found + 8, 0x8000_0000);
        e.vcall(broad_phase, 0x44, &args![aabb, found]);
        let mut index = 0i32;
        while index < e.mem.i32(found + 4) {
            let data = e.mem.u32(found);
            let collidable = e
                .mem
                .u32(data.wrapping_add((index as u32).wrapping_mul(8)) + 4);
            let owner = offset_by_byte(e, collidable, collidable + 5);
            if e.mem.u8(owner + 0x18) == 1 {
                let body = offset_by_byte(e, owner, owner + 0x10);
                if body != 0 {
                    e.call(ACTIVATE_BODY, &args![body]);
                }
            }
            index += 1;
        }
        array_release(e, found, 8, 0x10);
    });
}

// Translated from 00c944a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::updateCollisionFilterOnPhantom` (Xbox PDB). While critical operations are
/// locked the request is queued (operation 0x14: phantom at +4, the flag byte at +8).
/// Otherwise inside the critical section: asks the broad phase (vtable slot 0x48) for the
/// collidables overlapping the phantom's collidable (`phantom + 0x24` is its broad phase
/// handle) and, for every other one, runs the outlined `00c90f40` with the other body and
/// the world's collision filter; when `flag` is non-zero, calls the phantom's vtable slot
/// 0x30 afterwards. Profiling records mark the steps.
pub fn hkp_world_update_collision_filter_on_phantom(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    phantom: u32,
    flag: u32,
) {
    if operations_locked(e, this) {
        queue_operation(e, this, 0x14, |e, operation| {
            e.mem.set_u32(operation + 4, phantom);
            e.mem.set_u8(operation + 8, flag as u8);
        });
        return;
    }
    let world = this.addr();
    e.set(this, HkpWorld::m_criticalOperationsLockCount, 1);
    profile_record_pair(
        e,
        PROFILE_UPDATE_FILTER_ON_PHANTOM,
        PROFILE_BROADPHASE_SECOND,
    );
    let capacity = e.mem.u32(world + 0x338);
    e.with_stack(0x10, |e, found| {
        let found = found.addr();
        let saved = bump_array(e, found, capacity, 8);
        let broad_phase = e.mem.u32(world + 0x58);
        e.vcall(broad_phase, 0x48, &args![phantom + 0x24, found]);
        profile_record(e, PROFILE_UPDATE_OVERLAPS);
        let mut index = 0i32;
        while index < e.mem.i32(found + 4) {
            let data = e.mem.u32(found);
            let collidable = e
                .mem
                .u32(data.wrapping_add((index as u32).wrapping_mul(8)) + 4);
            if collidable != phantom + 0x24 {
                let owner = offset_by_byte(e, collidable, collidable + 5);
                let filter = e.mem.u32(world + 0x7c);
                fn_00c90f40(e, phantom, owner, filter);
            }
            index += 1;
        }
        if flag != 0 {
            profile_record(e, PROFILE_COLLECTION_FILTER);
            e.vcall(phantom, 0x30, &args![]);
        }
        end_critical_section(e, this);
        profile_record(e, PROFILE_SECTION_END);
        bump_array_release(e, found, saved, 8);
    });
}

// Translated from 00c946e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::updateCollisionFilterOnEntity` (Xbox PDB). While critical operations are
/// locked the request is queued (operation 0x12: entity at +4, mode byte at +8, flag byte at
/// +9). Otherwise inside the critical section:
/// - an entity of type 5 first runs `00d08210` on its collidable;
/// - with `mode` 0 the broad phase (slot 0x48) is asked for the collidables overlapping the
///   entity; for each that is a phantom (byte +0x18 = 2) the outlined `00c90f40` runs with
///   the entity's collidable (and the phantom's slot 0x30 follows when `flag` is set) and
///   the entry leaves the list; then every agent of the entity (`00d081e0`) contributes the
///   pair of its two collidables' broad phase handles to a second list and `00cf7130` takes
///   those out of the first;
/// - every agent entry of the entity is asked of the collision filter and the group table;
///   a refused one has its agent removed (`00cc1160`) and the entity's island marked
///   (byte +0x25), an accepted one is updated (`00cfeba0`) when `flag` is 1;
/// - the pairs left in the first list are added to the broad phase dispatcher (`00cf7080`).
pub fn hkp_world_update_collision_filter_on_entity(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    entity: u32,
    mode: i32,
    flag: u32,
) {
    if operations_locked(e, this) {
        queue_operation(e, this, 0x12, |e, operation| {
            e.mem.set_u32(operation + 4, entity);
            e.mem.set_u8(operation + 8, mode as u8);
            e.mem.set_u8(operation + 9, flag as u8);
        });
        return;
    }
    let world = this.addr();
    profile_record_pair(e, PROFILE_UPDATE_FILTER_ON_ENTITY, PROFILE_INIT);
    begin_critical_section(e, this);
    if e.mem.u8(entity + 0xe8) == 5 {
        e.call(COLLIDABLE_RESET_AGENTS, &args![entity + 0x10]);
    }
    e.with_stack(0x830, |e, frame| {
        let pairs = frame.addr();
        let pairs_storage = pairs + 0x20;
        let second = pairs + 0x10;
        let second_storage = pairs + 0x420;
        e.mem.set_u32(pairs, pairs_storage);
        e.mem.set_u32(pairs + 8, 0x8000_0080);
        if mode == 0 {
            profile_record(e, PROFILE_BROADPHASE_SECOND);
            let broad_phase = e.mem.u32(world + 0x58);
            e.vcall(broad_phase, 0x48, &args![entity + 0x24, pairs]);
            profile_record(e, PROFILE_PHANTOMS);
            let mut index = 0i32;
            while index < e.mem.i32(pairs + 4) {
                let data = e.mem.u32(pairs);
                let collidable = e
                    .mem
                    .u32(data.wrapping_add((index as u32).wrapping_mul(8)) + 4);
                let owner = offset_by_byte(e, collidable, collidable + 5);
                if e.mem.u8(owner + 0x18) == 2 {
                    let phantom = offset_by_byte(e, owner, owner + 0x10);
                    if phantom != 0 {
                        let filter = e.mem.u32(world + 0x7c);
                        fn_00c90f40(e, phantom, entity + 0x10, filter);
                        if flag != 0 {
                            e.vcall(phantom, 0x30, &args![]);
                        }
                        let size = e.mem.i32(pairs + 4).wrapping_sub(1);
                        e.mem.set_i32(pairs + 4, size);
                        if size != index {
                            let data = e.mem.u32(pairs);
                            let last = e
                                .mem
                                .bytes(data.wrapping_add((size as u32).wrapping_mul(8)), 8);
                            e.mem
                                .write(data.wrapping_add((index as u32).wrapping_mul(8)), &last);
                        }
                        index -= 1;
                    }
                }
                index += 1;
            }
            let entries = e
                .call(COLLIDABLE_AGENT_ENTRIES, &args![entity + 0x10])
                .u32();
            e.mem.set_u32(second, second_storage);
            e.mem.set_u32(second + 4, 0);
            e.mem.set_u32(second + 8, 0x8000_0080);
            let mut entry_index = 0u32;
            while (entry_index as i32) < e.mem.i32(entries + 4) {
                array_make_room(e, second, 8);
                let element = e
                    .mem
                    .u32(second)
                    .wrapping_add(e.mem.u32(second + 4).wrapping_mul(8));
                let size = e.mem.u32(second + 4);
                e.mem.set_u32(second + 4, size.wrapping_add(1));
                let entry = e.mem.u32(e.mem.u32(entries) + entry_index * 8);
                let first_handle = e.mem.u32(entry + 0x10) + 0x14;
                e.mem.set_u32(element, first_handle);
                let entry = e.mem.u32(e.mem.u32(entries) + entry_index * 8);
                let second_handle = e.mem.u32(entry + 0x14) + 0x14;
                e.mem.set_u32(element + 4, second_handle);
                entry_index += 1;
            }
            e.call(REMOVE_DUPLICATE_PAIRS, &args![second, pairs]);
            array_release(e, second, 8, 0x10);
        }
        profile_record(e, PROFILE_CHECK_AGENTS);
        let family = e.mem.i8(entity + 0x2a);
        let entries = e
            .call(COLLIDABLE_AGENT_ENTRIES, &args![entity + 0x10])
            .u32();
        let mut index = 0i32;
        while index < e.mem.i32(entries + 4) {
            let element = e
                .mem
                .u32(entries)
                .wrapping_add((index as u32).wrapping_mul(8));
            let partner = e.mem.u32(element + 4);
            let filter = e.mem.u32(world + 0x7c);
            let accepted = filter_accepts(e, filter, entity + 0x10, partner);
            let partner_group = e.mem.i8(partner + 0x1a);
            if !accepted || !group_pair_enabled(e, world, family, partner_group) {
                let agent = e.mem.u32(element);
                e.call(REMOVE_AGENT, &args![agent]);
                let island = e.mem.u32(entity + 0xcc);
                let marks = e.mem.u8(island + 0x25);
                e.mem.set_u8(island + 0x25, (marks & 0xfd) | 1);
                index -= 1;
            } else if flag == 1 {
                // 00c94a73: CMP dword [ESP+0x84c],1 tests the third argument (`flag`), not
                // `mode`.
                let island = if e.mem.u8(entity + 0xe8) == 5 {
                    let body = offset_by_byte(e, partner, partner + 0x10);
                    e.mem.u32(body + 0xcc)
                } else {
                    e.mem.u32(entity + 0xcc)
                };
                let agent = e.mem.u32(element);
                let input = e.mem.u32(world + 0x78);
                e.call(UPDATE_SHAPE_COLLECTION_FILTER, &args![agent, input, island]);
            }
            index += 1;
        }
        if e.mem.i32(pairs + 4) > 0 {
            profile_record(e, PROFILE_ADD_AGENTS);
            let dispatcher = e.mem.u32(world + 0x64);
            let data = e.mem.u32(pairs);
            let size = e.mem.u32(pairs + 4);
            let base = filter_base(e, world);
            e.call(ADD_PAIRS, &args![dispatcher, data, size, base]);
        }
        end_critical_section(e, this);
        array_release(e, pairs, 8, 0x10);
        profile_record(e, PROFILE_SECTION_END);
    });
}

// Translated from 00c94bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::addEntityBatch` (Xbox PDB), `(entities, count, mode)`. Nothing for a count of
/// 0. While critical operations are locked the request is queued (operation 6: entities at
/// +4, the 16-bit count at +8, the mode byte at +0xA). Otherwise inside the critical
/// section: reserves two lists (the entities' broad phase handles, 4 bytes each, and their
/// 0x20-byte bounding boxes); picks the island for the new bodies (the first active island
/// when `m_addsToNewIsland` is clear, with room made for them; otherwise a new island
/// constructed with `00d07830` whose flags and id follow `mode`); for every entity gives it
/// a uid (+0xD4), adds a reference, makes its motion (vtable slot 0x10) when it has none,
/// resets the swept transform times, clears two words, sets its world and the collision
/// filter bits from the world's bytes (`00c90ed0`), adds it to the island (`00d07420`; a
/// type 5 entity goes to the fixed island), asks its shape (slot 0x1c) for the bounding box
/// from the motion and half the collision input's delta time, and caches its shape data
/// (`00c9c040`). A new island is then registered with the world (or destroyed when no
/// entity used it). The broad phase (slot 0x18) adds the whole batch, the new overlaps
/// become agents (`00cf7080`), and the world's and the entities' "added" callbacks fire.
pub fn hkp_world_add_entity_batch(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    entities: u32,
    count: i32,
    mode: i32,
) {
    if count == 0 {
        return;
    }
    if operations_locked(e, this) {
        queue_operation(e, this, 6, |e, operation| {
            e.mem.set_u32(operation + 4, entities);
            e.mem.set_u16(operation + 8, count as u16);
            e.mem.set_u8(operation + 0xa, mode as u8);
        });
        return;
    }
    let world = this.addr();
    profile_record_pair(e, PROFILE_ADD_ENTITIES, PROFILE_INIT);
    begin_critical_section(e, this);
    e.with_stack(0x40, |e, frame| {
        let handles = frame.addr();
        let bounds = frame.addr() + 0x10;
        let broad_phase_results = frame.addr() + 0x20;
        for array in [handles, bounds] {
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
        }
        reserve_if_needed(e, handles, count as u32, 4);
        reserve_if_needed(e, bounds, count as u32, 0x20);
        let mut used_new_island = false;
        let island = if e.get(this, HkpWorld::m_addsToNewIsland) {
            construct_island(e, this, mode)
        } else {
            first_active_island_with_room(e, this, count)
        };
        let input = e.mem.u32(world + 0x78);
        let half: f64 = e.global(HALF);
        let half_time = (f64::from(x87_quiet(e.mem.f32(input + 8))) * half) as f32;
        let mut index = 0i32;
        while index < count {
            let entity = e
                .mem
                .u32(entities.wrapping_add((index as u32).wrapping_mul(4)));
            let uid = e.mem.u32(world + 0x104).wrapping_add(1);
            e.mem.set_u32(world + 0x104, uid);
            e.mem.set_u32(entity + 0xd4, uid);
            e.call(ADD_REFERENCE, &args![entity]);
            if e.mem.u32(entity + 0x18) == 0 {
                let motion = e.vcall(entity, 0x10, &args![]).u32();
                e.mem.set_u32(entity + 0x18, motion);
            }
            e.call(
                SET_TIME_INFORMATION,
                &args![0.0f32, 0.0f32, entity + ENTITY_SWEPT_TRANSFORM],
            );
            e.mem.set_u16(entity + 0xec, 0);
            e.mem.set_u16(entity + 0xea, 0);
            e.mem.set_u32(entity + 8, world);
            let first = e.mem.u8(world + DYNAMICS_BYTES_305) as u32;
            let second = e.mem.u8(world + DYNAMICS_BYTES_305 + 1) as u32;
            let third = e.mem.u8(world + DYNAMICS_BYTES_305 + 2) as i32;
            fn_00c90ed0(e, Ptr::new(entity + ENTITY_MOTION), first, second, third);
            if e.mem.u8(entity + 0xe8) == 5 {
                let fixed = e.mem.u32(world + 0x20);
                e.call(ISLAND_ADD_ENTITY, &args![fixed, entity]);
            } else {
                used_new_island = true;
                e.call(ISLAND_ADD_ENTITY, &args![island, entity]);
            }
            let shape = e.mem.u32(entity + 0x10);
            e.mem.set_u32(entity + 0x30, 1);
            e.mem.set_u32(entity + 0x40, 0);
            if shape != 0 {
                let slot = e
                    .mem
                    .u32(bounds)
                    .wrapping_add(e.mem.u32(bounds + 4).wrapping_mul(0x20));
                let used = e.mem.u32(bounds + 4);
                e.mem.set_u32(bounds + 4, used.wrapping_add(1));
                let motion = e.mem.u32(entity + 0x18);
                e.vcall(shape, 0x1c, &args![motion, half_time, slot]);
                let size = e.mem.u32(handles + 4);
                let data = e.mem.u32(handles);
                e.mem
                    .set_u32(data.wrapping_add(size.wrapping_mul(4)), entity + 0x24);
                e.mem.set_u32(handles + 4, size.wrapping_add(1));
                e.call(SET_CACHED_SHAPE_DATA, &args![entity, this, shape]);
            }
            index += 1;
        }
        if e.get(this, HkpWorld::m_addsToNewIsland) {
            if used_new_island {
                let first_body = e.mem.u32(e.mem.u32(island + 0x48));
                let group = e.mem.u8(first_body + 0xd4);
                e.mem.set_u8(island + 0x24, group);
                let list = if mode == 1 {
                    world + 0x28
                } else {
                    world + 0x34
                };
                array_push_word(e, list, island);
            } else {
                e.vcall(island, 0, &args![1u32]);
            }
        }
        let capacity = e.mem.u32(world + 0x338);
        let saved = bump_array(e, broad_phase_results, capacity, 8);
        profile_record(e, PROFILE_ADD_BROADPHASE);
        let broad_phase = e.mem.u32(world + 0x58);
        e.vcall(
            broad_phase,
            0x18,
            &args![handles, bounds, broad_phase_results],
        );
        profile_record(e, PROFILE_CREATE_AGENTS);
        let dispatcher = e.mem.u32(world + 0x64);
        let data = e.mem.u32(broad_phase_results);
        let size = e.mem.u32(broad_phase_results + 4);
        let base = filter_base(e, world);
        e.call(ADD_PAIRS, &args![dispatcher, data, size, base]);
        profile_record(e, PROFILE_ADDED_CALLBACKS);
        let mut index = 0i32;
        while index < count {
            let entity = e
                .mem
                .u32(entities.wrapping_add((index as u32).wrapping_mul(4)));
            e.call(FIRE_WORLD_ENTITY_ADDED, &args![this, entity]);
            e.call(FIRE_ENTITY_ADDED, &args![entity]);
            index += 1;
        }
        end_critical_section(e, this);
        profile_record(e, PROFILE_SECTION_END);
        bump_array_release(e, broad_phase_results, saved, 8);
        array_release(e, bounds, 0x20, 0xc);
        array_release(e, handles, 4, 0xc);
    });
}

/// The part of `addEntityBatch` that takes the first active island and makes room in its
/// body list for `count` more bodies (at least twice the capacity when it has to grow).
fn first_active_island_with_room(e: &mut Engine, this: Ptr<HkpWorld>, count: i32) -> u32 {
    let first = e.get(this, HkpWorld::m_activeSimulationIslands).addr();
    let island = e.mem.u32(first);
    let bodies = island + 0x48;
    let need = e.mem.i32(island + 0x4c).wrapping_add(count);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(bodies + 8) & 0x3fff_ffff) as i32;
    if capacity < need {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if need < doubled { doubled } else { need };
        e.call(ARRAY_RESERVE, &args![allocator, bodies, wanted, 4u32]);
    }
    island
}

/// The part of `addEntityBatch` that builds a new simulation island (`00d07830` on a 0x6c
/// byte router block): flagged for a rebuild, its id and flags follow `mode`.
fn construct_island(e: &mut Engine, this: Ptr<HkpWorld>, mode: i32) -> u32 {
    let world = this.addr();
    let block = router_alloc(e, 0x6c);
    e.mem.set_u16(block + 4, 0x6c);
    let island = e.call(ISLAND_CONSTRUCT, &args![block, this]).u32();
    let marks = e.mem.u8(island + 0x25);
    e.mem.set_u8(island + 0x25, (marks & 0xfd) | 1);
    let flags = e.mem.u8(island + 0x26);
    if mode == 1 {
        let id = e.mem.u16(world + 0x2c);
        e.mem.set_u16(island + 0x20, id);
        e.mem.set_u8(island + 0x26, (flags & 0xf5) | 5);
    } else {
        e.mem.set_u8(island + 0x26, flags & 0xf0);
        let id = e.mem.u16(world + 0x38);
        e.mem.set_u16(island + 0x20, id);
    }
    island
}

/// Reserves room for `count` elements (`00c908c0` with the router's second allocator) when
/// the array holds fewer.
fn reserve_if_needed(e: &mut Engine, array: u32, count: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if (capacity as i32) < count as i32 {
        let allocator = e.mem.u32(router + 0xc);
        e.call(ARRAY_RESERVE, &args![allocator, array, count, element_size]);
    }
}

// Translated from 00c951b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removeEntityBatch` (Xbox PDB), `(entities, count)`. Nothing for a count below
/// 1. While critical operations are locked the request is queued (operation 7: entities at
/// +4, the 16-bit count at +8). Otherwise inside the critical section: the simulation
/// (vtable slot 0x20) is told first; the broad phase handles (`entity + 0x24`) of the
/// entities that have a shape are collected, the broad phase (slot 0x20) removes them and
/// the pairs that results in are removed (`00cf70f0`); then every entity gets the world's
/// and its own "removed" callbacks, leaves the broad phase (`00cb4740`), releases its
/// bounding data (`00d01a80` when +0x54 is set), is deleted through vtable slot 0x14 when its
/// `m_memSizeAndFlags` is 0 and loses a reference.
pub fn hkp_world_remove_entity_batch(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    entities: u32,
    count: i32,
) {
    if count < 1 {
        return;
    }
    if operations_locked(e, this) {
        queue_operation(e, this, 7, |e, operation| {
            e.mem.set_u32(operation + 4, entities);
            e.mem.set_u16(operation + 8, count as u16);
        });
        return;
    }
    let world = this.addr();
    e.set(this, HkpWorld::m_criticalOperationsLockCount, 1);
    profile_record_pair(e, PROFILE_REMOVE_ENTITIES, PROFILE_REMOVE_INIT);
    let simulation = e.mem.u32(world + 8);
    e.vcall(simulation, 0x20, &args![entities, count, this, 1u32]);
    e.with_stack(0x20, |e, frame| {
        let handles = frame.addr();
        let found = frame.addr() + 0x10;
        e.mem.set_u32(handles, 0);
        e.mem.set_u32(handles + 4, 0);
        e.mem.set_u32(handles + 8, 0x8000_0000);
        reserve_if_needed(e, handles, count as u32, 4);
        let end = entities.wrapping_add((count as u32).wrapping_mul(4));
        let mut at = entities;
        while at != end {
            let entity = e.mem.u32(at);
            if e.mem.u32(entity + 0x10) != 0 {
                let size = e.mem.u32(handles + 4);
                let data = e.mem.u32(handles);
                e.mem
                    .set_u32(data.wrapping_add(size.wrapping_mul(4)), entity + 0x24);
                e.mem.set_u32(handles + 4, size.wrapping_add(1));
            }
            at = at.wrapping_add(4);
        }
        if e.mem.u32(handles + 4) != 0 {
            profile_record(e, PROFILE_REMOVE_BROADPHASE);
            let capacity = e.mem.u32(world + 0x338);
            let saved = bump_array(e, found, capacity, 8);
            let broad_phase = e.mem.u32(world + 0x58);
            e.vcall(broad_phase, 0x20, &args![handles, found]);
            profile_record(e, PROFILE_DELETE_AGENTS);
            let dispatcher = e.mem.u32(world + 0x64);
            let data = e.mem.u32(found);
            let size = e.mem.u32(found + 4);
            e.call(REMOVE_PAIRS, &args![dispatcher, data, size]);
            bump_array_release(e, found, saved, 8);
        }
        array_release(e, handles, 4, 0xc);
    });
    profile_record(e, PROFILE_REMOVE_CALLBACKS);
    let end = entities.wrapping_add((count as u32).wrapping_mul(4));
    let mut at = entities;
    while at != end {
        // 00c95424..00c9545f: the entity is loaded from the array again for each call.
        let entity = e.mem.u32(at);
        e.call(FIRE_WORLD_ENTITY_REMOVED, &args![this, entity]);
        let entity = e.mem.u32(at);
        e.call(FIRE_ENTITY_REMOVED, &args![entity]);
        let entity = e.mem.u32(at);
        e.call(REMOVE_ENTITY_BP, &args![this, entity]);
        let entity = e.mem.u32(at);
        if e.mem.u32(entity.wrapping_add(0x54)) != 0 {
            e.call(
                ENTITY_RELEASE_BOUNDING_DATA,
                &args![entity.wrapping_add(ENTITY_BOUNDING_DATA)],
            );
        }
        let entity = e.mem.u32(at);
        if e.mem.u16(entity.wrapping_add(4)) == 0 {
            e.vcall(entity, 0x14, &args![]);
        }
        let entity = e.mem.u32(at);
        e.call(REMOVE_REFERENCE, &args![entity]);
        at = at.wrapping_add(4);
    }
    profile_record(e, PROFILE_SECTION_END);
    end_critical_section(e, this);
}

// Translated from 00c954e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::addPhantomBatch` (Xbox PDB), `(phantoms, count)`. While either lock count is
/// non-zero the request is queued (operation 0xF: phantoms at +4, the 16-bit count at +8).
/// Otherwise inside the critical section: for every phantom gives it its motion-like object
/// (+0x18) when it has none, sets its world pointer, records its broad phase handle
/// (`phantom + 0x24`), asks it for its bounding box (vtable slot 0x18), runs `00d07950` with
/// the box, adds a reference, appends it to `m_phantoms`, fires the world's "phantom added"
/// callback (`00d00780`) and runs `00d07b90`; then the broad phase (slot 0x18) adds all
/// handles with their boxes and the new overlaps become agents (`00cf7080`).
pub fn hkp_world_add_phantom_batch(e: &mut Engine, this: Ptr<HkpWorld>, phantoms: u32, count: i32) {
    if phantom_operations_locked(e, this) {
        queue_operation(e, this, 0xf, |e, operation| {
            e.mem.set_u32(operation + 4, phantoms);
            e.mem.set_u16(operation + 8, count as u16);
        });
        return;
    }
    let world = this.addr();
    begin_critical_section(e, this);
    e.with_stack(0x30, |e, frame| {
        let handles = frame.addr();
        let bounds = frame.addr() + 0x10;
        let found = frame.addr() + 0x20;
        let saved_handles = bump_array(e, handles, count as u32, 4);
        e.mem.set_i32(handles + 4, count);
        let saved_bounds = bump_array(e, bounds, count as u32, 0x20);
        e.mem.set_i32(bounds + 4, count);
        let mut index = 0i32;
        while index < count {
            let phantom = e
                .mem
                .u32(phantoms.wrapping_add((index as u32).wrapping_mul(4)));
            if e.mem.u32(phantom + 0x18) == 0 {
                let made = e.vcall(phantom, 0x10, &args![]).u32();
                e.mem.set_u32(phantom + 0x18, made);
            }
            e.mem.set_u32(phantom + 8, world);
            let data = e.mem.u32(handles);
            e.mem.set_u32(
                data.wrapping_add((index as u32).wrapping_mul(4)),
                phantom + 0x24,
            );
            let slot = e
                .mem
                .u32(bounds)
                .wrapping_add((index as u32).wrapping_mul(0x20));
            e.vcall(phantom, 0x18, &args![slot]);
            let slot = e
                .mem
                .u32(bounds)
                .wrapping_add((index as u32).wrapping_mul(0x20));
            e.call(PHANTOM_SET_AABB, &args![phantom, slot]);
            e.call(ADD_REFERENCE, &args![phantom]);
            array_push_word(e, world + 0x110, phantom);
            e.call(FIRE_PHANTOM_ADDED, &args![this, phantom]);
            e.call(PHANTOM_AFTER_ADD, &args![phantom]);
            index += 1;
        }
        let capacity = e.mem.u32(world + 0x338);
        let saved = bump_array(e, found, capacity, 8);
        let broad_phase = e.mem.u32(world + 0x58);
        e.vcall(broad_phase, 0x18, &args![handles, bounds, found]);
        let dispatcher = e.mem.u32(world + 0x64);
        let data = e.mem.u32(found);
        let size = e.mem.u32(found + 4);
        let base = filter_base(e, world);
        e.call(ADD_PAIRS, &args![dispatcher, data, size, base]);
        end_critical_section(e, this);
        bump_array_release(e, found, saved, 8);
        bump_array_release(e, bounds, saved_bounds, 0x20);
        bump_array_release(e, handles, saved_handles, 4);
    });
}

// Translated from 00c95800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removePhantomBatch` (Xbox PDB), `(phantoms, count)`. While either lock count
/// is non-zero the request is queued (operation 0x10: phantoms at +4, the 16-bit count at
/// +8). Otherwise inside the critical section: for every phantom records its broad phase
/// handle (`phantom + 0x24`), fires the world's "phantom removed" callback (`00d00820`) and
/// runs `00d07b50`; the broad phase (slot 0x20) removes the handles and the resulting pairs
/// are removed (`00cf70f0`); then every phantom loses its world pointer, leaves
/// `m_phantoms`, is deleted through vtable slot 0x34 when its `m_memSizeAndFlags` is 0 and
/// loses a reference.
pub fn hkp_world_remove_phantom_batch(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    phantoms: u32,
    count: i32,
) {
    if phantom_operations_locked(e, this) {
        queue_operation(e, this, 0x10, |e, operation| {
            e.mem.set_u32(operation + 4, phantoms);
            e.mem.set_u16(operation + 8, count as u16);
        });
        return;
    }
    let world = this.addr();
    begin_critical_section(e, this);
    e.with_stack(0x20, |e, frame| {
        let handles = frame.addr();
        let found = frame.addr() + 0x10;
        let saved_handles = bump_array(e, handles, count as u32, 4);
        let mut index = 0i32;
        while index < count {
            let phantom = e
                .mem
                .u32(phantoms.wrapping_add((index as u32).wrapping_mul(4)));
            let size = e.mem.u32(handles + 4);
            let data = e.mem.u32(handles);
            e.mem
                .set_u32(data.wrapping_add(size.wrapping_mul(4)), phantom + 0x24);
            e.mem.set_u32(handles + 4, size.wrapping_add(1));
            e.call(FIRE_PHANTOM_REMOVED, &args![this, phantom]);
            e.call(PHANTOM_BEFORE_REMOVE, &args![phantom]);
            index += 1;
        }
        if e.mem.u32(handles + 4) != 0 {
            let capacity = e.mem.u32(world + 0x338);
            let saved = bump_array(e, found, capacity, 8);
            let broad_phase = e.mem.u32(world + 0x58);
            e.vcall(broad_phase, 0x20, &args![handles, found]);
            let dispatcher = e.mem.u32(world + 0x64);
            let data = e.mem.u32(found);
            let size = e.mem.u32(found + 4);
            e.call(REMOVE_PAIRS, &args![dispatcher, data, size]);
            bump_array_release(e, found, saved, 8);
        }
        let mut index = 0i32;
        while index < count {
            let phantom = e
                .mem
                .u32(phantoms.wrapping_add((index as u32).wrapping_mul(4)));
            remove_phantom_from_list(e, this, phantom);
            if e.mem.u16(phantom + 4) == 0 {
                e.vcall(phantom, 0x34, &args![]);
            }
            e.call(REMOVE_REFERENCE, &args![phantom]);
            index += 1;
        }
        end_critical_section(e, this);
        bump_array_release(e, handles, saved_handles, 4);
    });
}

// ---------------------------------------------------------------------------------------
// The constructor (`00c95a80`) and its pieces.
// ---------------------------------------------------------------------------------------

/// The `hkpWorld` vtable the constructor installs.
const CTOR_WORLD_VTABLE: u32 = 0x010c_6714;
/// The Havok version the libraries were built with; the constructor compares the caller's
/// version (its second stack argument) with it.
const CTOR_HAVOK_VERSION: u32 = 0x111d4;
/// Global function pointer (cdecl `(min, max, markers)`) that creates the broad phase.
const CTOR_CREATE_BROAD_PHASE: u32 = 0x011b_37a0;
/// A global byte: when set, the cinfo's `m_simulationType` (+0xd1) is forced to 3.
const CTOR_FORCE_MULTITHREADED: u32 = 0x0126_81dc;
/// The error reporter object (a pointer); its slot +0xc takes `(3, id, message, file, line)`
/// and answers non-zero to ask for a break (`INT3`).
const CTOR_ERROR_REPORTER: u32 = 0x0126_8360;
/// The version message pieces, the source file name, the message id and the line (0xbb5).
const CTOR_VERSION_TEXT_FIRST: u32 = 0x010c_687c;
const CTOR_VERSION_TEXT_SECOND: u32 = 0x010c_68a0;
const CTOR_VERSION_TEXT_LAST: u32 = 0x010c_68c0;
const CTOR_SOURCE_FILE: u32 = 0x010c_6864;
const CTOR_VERSION_MESSAGE_ID: u32 = 0x53c9_4b42;
const CTOR_VERSION_MESSAGE_LINE: u32 = 0xbb5;
/// The error stream (`00cb1600(stream, buffer, size)`), its text and integer appends
/// (`00cc01c0`, `00cc0280`, `00cc02d0`; each returns the stream) and its destructor.
const CTOR_ERROR_STREAM_CONSTRUCT: u32 = 0x00cb_1600;
const CTOR_STREAM_APPEND_TEXT: u32 = 0x00cc_01c0;
const CTOR_STREAM_APPEND_FIRST_INT: u32 = 0x00cc_0280;
const CTOR_STREAM_APPEND_SECOND_INT: u32 = 0x00cc_02d0;
const CTOR_ERROR_STREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// Constructors and helpers the world's constructor calls (`this` first, as registered).
const CTOR_OPERATION_QUEUE: u32 = 0x00cf_75c0;
const CTOR_SOLVER_TABLES: u32 = 0x00d1_0890;
const CTOR_BROAD_PHASE_DISPATCHER: u32 = 0x00cf_6f90;
const CTOR_ENTITY_ENTITY_LISTENER: u32 = 0x00d1_07d0;
const CTOR_CONTACT_MGR_FACTORY: u32 = 0x00cf_d450;
const CTOR_COLLISION_DISPATCHER: u32 = 0x00cf_c030;
/// The default agent function `00d10790` handed to the collision dispatcher.
const CTOR_DEFAULT_AGENT_FUNCTION: u32 = 0x00d1_0790;
const CTOR_DEFAULT_COLLISION_FILTER: u32 = 0x00cb_fba0;
const CTOR_SIMULATION: u32 = 0x00cf_87c0;
const CTOR_CONTINUOUS_SIMULATION: u32 = 0x00d0_d020;
const CTOR_MULTI_THREADED_SIMULATION: u32 = 0x00d0_2c80;
const CTOR_CRITICAL_SECTION: u32 = 0x00cf_6da0;
const CTOR_COLLISION_QUALITY: u32 = 0x00cf_b570;
const CTOR_SIMULATION_ISLAND: u32 = 0x00d0_7830;
const CTOR_ARRAY_GROW: u32 = 0x00c9_0940;
const CTOR_RIGID_BODY_CINFO: u32 = 0x00cf_5ea0;
const CTOR_RIGID_BODY: u32 = 0x00ca_8c90;
const CTOR_BROAD_PHASE_BORDER: u32 = 0x00d0_b710;
const CTOR_MAINTENANCE_MGR: u32 = 0x00d0_b250;
const CTOR_COLLISION_CALLBACKS: u32 = 0x00d0_b040;
const CTOR_KD_TREE_MANAGER: u32 = 0x00d0_a2d0;
const CTOR_AABB_TREE_MANAGER: u32 = 0x00d0_8cf0;
const CTOR_MULTI_THREAD_CHECK_DISABLE: u32 = 0x00cf_6d20;
/// Vtables (and second table words) of the small objects built inline.
const CTOR_PHANTOM_LISTENER_VTABLE: u32 = 0x010c_63f0;
const CTOR_PHANTOM_LISTENER_TABLE: u32 = 0x010c_63e0;
const CTOR_BORDER_LISTENER_VTABLE: u32 = 0x010c_6410;
const CTOR_BORDER_LISTENER_TABLE: u32 = 0x010c_6400;
const CTOR_FILTER_TABLES: [u32; 5] = [
    0x010c_646c,
    0x010c_6460,
    0x010c_644c,
    0x010c_6440,
    0x010c_6434,
];
const CTOR_CONVEX_LIST_FILTER_VTABLE: u32 = 0x010c_63cc;
/// `float` constants: the default gravity length (9.81, used when the gravity is zero), the
/// 0.01 the agent configuration starts with, the large value for a zero resting threshold,
/// the 1/60 time step of the step information, and the three SSE factors of the broad-phase
/// conversion (lane 3 filler, scale, inverse scale).
const CTOR_DEFAULT_GRAVITY: u32 = 0x010c_6860;
const CTOR_HUNDREDTH: u32 = 0x0101_3ea4;
const CTOR_NO_THRESHOLD: u32 = 0x010c_6848;
const CTOR_STEP_TIME: u32 = 0x010c_42e8;
const CTOR_LANE_FILLER: u32 = 0x0101_1d78;
const CTOR_EXTENT_SCALE: u32 = 0x010c_6844;
const CTOR_EXTENT_INVERSE_SCALE: u32 = 0x010c_6840;
/// The 16-byte sign mask the minimum corner is XORed with.
const CTOR_SIGN_MASK: u32 = 0x010c_44d0;
/// `double` constants: 0.1, 0.016, 4.0, 8.0 and the step time 1/60.
const CTOR_TENTH: u32 = 0x0101_ffa0;
const CTOR_FRAME_FACTOR: u32 = 0x010c_6858;
const CTOR_FOUR: u32 = 0x0101_db80;
const CTOR_EIGHT: u32 = 0x0102_40b0;
const CTOR_STEP_DOUBLE: u32 = 0x010c_6838;
/// The `float` pairs (linear factor, angular factor) of the deactivation levels 2, 3 and 4
/// and of the last level (the jump table at `00c96a0c`); levels 0 and 1 take the `float`
/// at `0104da4c` and 0.0.
const CTOR_LEVEL_LOW_FACTOR: u32 = 0x0104_da4c;
const CTOR_LEVEL_FACTORS: [(u32, u32); 4] = [
    (0x0101_3ea4, 0x010c_6850),
    (0x010c_684c, 0x0101_df44),
    (0x0102_d934, 0x0101_7870),
    (0x0103_13d4, 0x0102_31a8),
];

/// The SSE length of the vector at `at`: `MULPS` of the vector with itself, then
/// `sqrt(z + (y + x))` with `ADDSS` and `SQRTSS`.
fn ctor_vector_length(e: &Engine, at: u32) -> f32 {
    let x = e.mem.f32(at);
    let y = e.mem.f32(at + 4);
    let z = e.mem.f32(at + 8);
    let (xx, yy, zz) = (x * x, y * y, z * z);
    (zz + (yy + xx)).sqrt()
}

/// The version mismatch report (`00c95ceb`..`00c95d6d`): builds the message in the error
/// stream at `frame + 0x20` over the 0x200-byte buffer at `frame + 0x120`, hands it to the
/// error reporter and destroys the stream. A non-zero answer of the reporter reaches the
/// `INT3` at `00c95d68`.
fn ctor_report_version_mismatch(e: &mut Engine, frame: u32, version: u32) {
    let stream = frame + 0x20;
    let buffer = frame + 0x120;
    e.call(
        CTOR_ERROR_STREAM_CONSTRUCT,
        &args![stream, buffer, 0x200u32],
    );
    let out = e
        .call(
            CTOR_STREAM_APPEND_TEXT,
            &args![stream, CTOR_VERSION_TEXT_FIRST],
        )
        .u32();
    let out = e
        .call(
            CTOR_STREAM_APPEND_FIRST_INT,
            &args![out, CTOR_HAVOK_VERSION],
        )
        .u32();
    let out = e
        .call(
            CTOR_STREAM_APPEND_TEXT,
            &args![out, CTOR_VERSION_TEXT_SECOND],
        )
        .u32();
    let out = e
        .call(CTOR_STREAM_APPEND_SECOND_INT, &args![out, version])
        .u32();
    e.call(CTOR_STREAM_APPEND_TEXT, &args![out, CTOR_VERSION_TEXT_LAST]);
    let reporter: u32 = e.global(CTOR_ERROR_REPORTER);
    let answer = e
        .vcall(
            reporter,
            0xc,
            &args![
                3u32,
                CTOR_VERSION_MESSAGE_ID,
                buffer,
                CTOR_SOURCE_FILE,
                CTOR_VERSION_MESSAGE_LINE
            ],
        )
        .u32();
    if answer != 0 {
        panic!("INT3 at 00c95d68: the error reporter asked for a break");
    }
    e.call(CTOR_ERROR_STREAM_DESTRUCT, &args![stream]);
}

/// The six deactivation levels of `m_dynamicsStepInfo.m_solverInfo.m_deactivationInfo`
/// (`00c95f06`..`00c96060`, rows of 0x1c bytes at `world + 0x244`), x87 code. `gravity` is
/// the gravity length, `zero` and `one` the 0.0 and 1.0 the x87 stack holds through the loop.
fn ctor_deactivation_levels(e: &mut Engine, world: u32, cinfo: u32, gravity: f64) {
    let one = 1.0f64;
    let zero = 0.0f64;
    // FLD double 0.1; FMUL ST0,ST1 (D8 C9); FSTP: the angular scale.
    let tenth = x87_load64(e.global::<u64>(CTOR_TENTH));
    let angular_scale = x87_store32(x87_mul(tenth, gravity));
    let frame_factor = x87_load64(e.global::<u64>(CTOR_FRAME_FACTOR));
    for level in 0..6u32 {
        // The switch on the level (jump table `00c96a0c` for 0..4, default above 4).
        let (linear_bits, angular_bits) = match level {
            0 | 1 => {
                let linear = x87_store32(x87_load32(e.global::<u32>(CTOR_LEVEL_LOW_FACTOR)));
                // FXCH ST2; FST: the 0.0 of the stack.
                (linear, x87_store32(zero))
            }
            _ => {
                let index = match level {
                    2 => 0,
                    3 => 1,
                    4 => 2,
                    _ => 3,
                };
                let (linear_at, angular_at) = CTOR_LEVEL_FACTORS[index];
                let linear = x87_store32(x87_load32(e.global::<u32>(linear_at)));
                let angular = x87_store32(x87_load32(e.global::<u32>(angular_at)));
                (linear, angular)
            }
        };
        let row = world + 0x244 + 0x1c * level;
        // 00c95f8a: the linear factor times the gravity (FMUL ST0,ST2).
        let linear = x87_store32(x87_mul(x87_load32(linear_bits), gravity));
        let angular = x87_load32(angular_bits);
        let angular_gravity = x87_mul(angular, gravity);
        let linear = x87_load32(linear);
        // FDIVP ST2,ST0: the angular product over the linear one, through a float.
        let ratio = x87_load32(x87_store32(x87_div(angular_gravity, linear)));
        // m_invNumSteps (+0x300) times the frame factor (FMUL ST0,ST6), times the ratio.
        let inverse_steps = x87_load32(e.mem.u32(world + 0x300));
        let product = x87_mul(ratio, x87_mul(inverse_steps, frame_factor));
        // +8 (m_slowObjectVelocityMultiplier): FSUBR ST0,ST6.
        e.mem.set_u32(row + 8, x87_store32(x87_sub(one, product)));
        // +0 (m_linearVelocityThresholdInv): FDIVR ST0,ST6.
        e.mem.set_u32(row, x87_store32(x87_div(one, linear)));
        // +4 (m_angularVelocityThresholdInv): FMUL dword (the angular scale), FDIVR ST0,ST5.
        let scaled = x87_mul(linear, x87_m32(angular_scale));
        e.mem.set_u32(row + 4, x87_store32(x87_div(one, scaled)));
        // +0xc (m_relativeSleepVelocityThreshold): FCOM with 0.0; the jump (C0 or C3) takes
        // a negative, zero or unordered factor to the large constant.
        let threshold = if angular > zero {
            let inverse_steps = x87_load32(e.mem.u32(world + 0x300));
            x87_store32(x87_div(x87_mul(inverse_steps, frame_factor), angular))
        } else {
            x87_store32(x87_load32(e.global::<u32>(CTOR_NO_THRESHOLD)))
        };
        e.mem.set_u32(row + 0xc, threshold);
        // m_deactivationReferenceDistance (cinfo +0xa8) through a float slot.
        let distance = x87_load32(x87_quiet_bits(e.mem.u32(cinfo + 0xa8)));
        // +0x10 / +0x14 (m_maxDistSqrd): d * d and (4 d)^2.
        e.mem
            .set_u32(row + 0x10, x87_store32(x87_mul(distance, distance)));
        let four = x87_mul(x87_load64(e.global::<u64>(CTOR_FOUR)), distance);
        e.mem.set_u32(row + 0x14, x87_store32(x87_mul(four, four)));
        // +0x18 / +0x1a (m_maxRotSqrd, hkHalf): the top 16 bits of (2 d)^2 and (8 d)^2.
        let twice = x87_add(distance, distance);
        let twice_bits = x87_store32(x87_mul(twice, twice));
        e.mem
            .set_u16(row + 0x18, ((twice_bits as i32) >> 16) as u16);
        let eight = x87_mul(distance, f64::from_bits(e.global::<u64>(CTOR_EIGHT)));
        let eight_bits = x87_store32(x87_mul(eight, eight));
        e.mem
            .set_u16(row + 0x1a, ((eight_bits as i32) >> 16) as u16);
    }
}

/// The collision input's step information and broad-phase conversion (`00c963b1`..
/// `00c964cc`): `+0x50`..`+0x5c` (`m_stepInfo`: start, end, delta = 0 - 0 and its inverse,
/// 0.0 when the delta is 0.0), then `m_aabb32Info` from the cinfo's world AABB (+0x30 min,
/// +0x40 max): `+0x40` = the per-axis inverse spans times the scale, `+0x20` = the minimum
/// with the sign mask applied, `+0x30` = the inverse scale times the span plus `+0x20`, each
/// with lane 3 cleared at the end.
fn ctor_collision_input_extents(e: &mut Engine, input: u32, cinfo: u32) {
    let zero = 0.0f64;
    let one = 1.0f64;
    e.mem.set_u32(input + 0x50, x87_store32(zero));
    e.mem.set_u32(input + 0x54, x87_store32(zero));
    let delta = x87_load32(x87_store32(x87_sub(zero, zero)));
    e.mem.set_u32(input + 0x58, x87_store32(delta));
    // FUCOM against the stack's 0.0; JP takes "not equal or unordered" to the division.
    // The delta is 0 - 0, so the equal path always runs.
    if delta == zero {
        e.mem.set_u32(input + 0x5c, x87_store32(zero));
    } else {
        e.mem
            .set_u32(input + 0x5c, x87_store32(x87_div(one, delta)));
    }
    let lane_filler = e.global::<f32>(CTOR_LANE_FILLER);
    let mut min = [0f32; 4];
    let mut span = [0f32; 4];
    for lane in 0..4u32 {
        let low = e.mem.f32(cinfo + 0x30 + 4 * lane);
        let high = e.mem.f32(cinfo + 0x40 + 4 * lane);
        min[lane as usize] = low;
        span[lane as usize] = high - low;
    }
    // FLD dword (x and y) and FDIVR; FDIV dword (z), all with the stack's 1.0.
    let inverse_x = f32::from_bits(x87_store32(x87_div(one, x87_load32(span[0].to_bits()))));
    let inverse_y = f32::from_bits(x87_store32(x87_div(one, x87_load32(span[1].to_bits()))));
    let inverse_z = f32::from_bits(x87_store32(x87_div(one, x87_m32(span[2].to_bits()))));
    let scale = e.global::<f32>(CTOR_EXTENT_SCALE);
    let scaled = [
        inverse_x * scale,
        inverse_y * scale,
        inverse_z * scale,
        lane_filler * scale,
    ];
    let inverse_scale = e.global::<f32>(CTOR_EXTENT_INVERSE_SCALE);
    for lane in 0..4u32 {
        let index = lane as usize;
        e.mem.set_f32(input + 0x40 + 4 * lane, scaled[index]);
    }
    for lane in 0..4u32 {
        let index = lane as usize;
        let mask = e.global::<u32>(CTOR_SIGN_MASK + 4 * lane);
        let negated = min[index].to_bits() ^ mask;
        e.mem.set_u32(input + 0x20 + 4 * lane, negated);
        let high = inverse_scale * span[index] + f32::from_bits(negated);
        e.mem.set_f32(input + 0x30 + 4 * lane, high);
    }
    for vector in [0x40, 0x20, 0x30] {
        e.mem.set_u32(input + vector + 0xc, 0);
    }
}

// Translated from 00c95a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::hkpWorld` (Xbox PDB), `(cinfo, version)`: the world's constructor. Field
/// names below are the Xbox PDB's `hkpWorld` and `hkpWorldCinfo`, at their PC offsets.
///
/// Empties the island, phantom and listener arrays, then allocates through the memory
/// router: the violated-constraint array (0x204), the pending operation queue (0x28,
/// `00cf75c0`), and the broad phase (the creator at `011b37a0` with the world's copy of the
/// AABB and the marker count). A `version` other than `0x111d4` is reported (the reporter's
/// non-zero answer is an `INT3`). Copies gravity and the cinfo's settings, builds the solver
/// information (`00d10890`, `00c90e60`, the six deactivation levels), takes the memory
/// watchdog, builds the broad-phase dispatcher and its three listeners, the collision
/// dispatcher (with the contact manager factory `00cfd450` and the default agent
/// function), the collision filter and convex list filter (default ones unless the cinfo has
/// them, which then get a reference), the collision input with its agent configuration and
/// broad-phase conversion, runs the outlined `00c912e0` and creates the simulation chosen by
/// `m_simulationType` (forced to 3 by the global byte `012681dc`; 1 single-threaded, 2
/// continuous, 3 multi-threaded with two critical sections; others none), the island-dirty
/// critical section, the collision-quality information (`00cfb570`), the fixed island (and
/// one active island when simulation islands are off), the fixed rigid body (mass 0, motion
/// type 5) added with `addEntity`, the step information, the broad-phase border (unless the
/// behaviour is 3), the maintenance manager (initialized through its slot +0xc), the
/// collision callbacks, the kd-tree or AABB-tree manager, the multi-thread check, and the
/// multi-threaded job data (0x120). Returns `this`.
pub fn hkp_world_hkp_world(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    cinfo: Ptr,
    version: u32,
) -> Ptr<HkpWorld> {
    let w = this.addr();
    let c = cinfo.addr();
    // The function's stack frame (0x320 bytes after the aligned base): the error stream at
    // +0x20 and its buffer at +0x120, the collision-quality input at +0x34 and the rigid
    // body cinfo at +0x60.
    e.with_stack(0x320, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(w, CTOR_WORLD_VTABLE);
        // m_referenceCount.
        e.mem.set_u16(w + 6, 1);
        // m_activeSimulationIslands, m_inactiveSimulationIslands and
        // m_dirtySimulationIslands: empty, capacity word 0x80000000 (not owned).
        e.mem.set_u32(w + 0x30, 0x8000_0000);
        e.mem.set_u32(w + 0x28, 0);
        e.mem.set_u32(w + 0x2c, 0);
        e.mem.set_u32(w + 0x3c, 0x8000_0000);
        e.mem.set_u32(w + 0x34, 0);
        e.mem.set_u32(w + 0x38, 0);
        e.mem.set_u32(w + 0x48, 0x8000_0000);
        e.mem.set_u32(w + 0x40, 0);
        e.mem.set_u32(w + 0x44, 0);
        // m_memoryWatchDog.
        e.mem.set_u32(w + 0x50, 0);
        // m_multiThreadCheck (+0xa8, +0xac).
        e.mem.set_u32(w + 0xa8, 0xffff_fff1);
        e.mem.set_u16(w + 0xac, 0x8000);
        // m_phantoms and the thirteen listener arrays (+0x110..+0x1ac).
        let mut array = 0x110;
        while array <= 0x1ac {
            e.mem.set_u32(w + array + 8, 0x8000_0000);
            e.mem.set_u32(w + array, 0);
            e.mem.set_u32(w + array + 4, 0);
            array += 0xc;
        }
        // m_lastEntityUid, m_lastConstraintUid, m_isLocked,
        // m_assertOnRunningOutOfSolverMemory.
        e.mem.set_u32(w + 0x104, 0xffff_ffff);
        e.mem.set_u32(w + 0x10c, 0xffff_ffff);
        e.mem.set_u32(w + 0xbc, 0);
        e.mem.set_u8(w + 0x54, 1);
        // m_violatedConstraintArray.
        let violated = router_alloc(e, 0x204);
        if violated != 0 {
            e.mem.set_u32(violated, 0);
        }
        e.mem.set_u32(w + 0x1b8, violated);
        // m_pendingOperations.
        let block = router_alloc(e, 0x28);
        let queue = if block != 0 {
            e.call(CTOR_OPERATION_QUEUE, &args![block, w]).u32()
        } else {
            0
        };
        e.mem.set_u32(w + 0x88, queue);
        // m_pendingOperationQueues, the pending counts, m_blockExecutingPendingOperations,
        // m_criticalOperationsAllowed, the three critical section pointers,
        // m_pendingOperationQueueCount, m_destructionWorld.
        e.mem.set_u32(w + 0xa0, 0);
        e.mem.set_u32(w + 0x8c, 0);
        e.mem.set_u32(w + 0x90, 0);
        e.mem.set_u32(w + 0x94, 0);
        e.mem.set_u32(w + 0x98, 0);
        e.mem.set_u8(w + 0x9c, 0);
        e.mem.set_u8(w + 0x9d, 1);
        e.mem.set_u32(w + 0xb8, 0);
        e.mem.set_u32(w + 0xc4, 0);
        e.mem.set_u32(w + 0xc0, 0);
        e.mem.set_u32(w + 0xa4, 1);
        e.mem.set_u32(w + 0x1c4, 0);
        // m_broadPhaseExtents[0..2] from m_broadPhaseWorldAabb, m_broadPhaseNumMarkers.
        copy_vector(e, c + 0x30, w + 0x310);
        copy_vector(e, c + 0x40, w + 0x320);
        copy_word(e, c + 0x70, w + 0x330);
        let markers = e.mem.u32(c + 0x70);
        let create_broad_phase: u32 = e.global(CTOR_CREATE_BROAD_PHASE);
        let broad_phase = e
            .call(create_broad_phase, &args![w + 0x310, w + 0x320, markers])
            .u32();
        e.mem.set_u32(w + 0x58, broad_phase);
        // m_broadPhaseQuerySize, m_broadPhaseUpdateSize (half of it, rounded toward zero),
        // m_sizeOfToiEventQueue.
        let query_size = e.mem.u32(c + 0x20) as i32;
        e.mem.set_u32(w + 0x338, query_size as u32);
        e.mem.set_u32(w + 0x33c, (query_size / 2) as u32);
        copy_word(e, c + 0x64, w + 0x334);
        if version != CTOR_HAVOK_VERSION {
            ctor_report_version_mismatch(e, frame, version);
        }
        // 00c95d72: m_gravity.
        copy_vector(e, c + 0x10, w + 0x10);
        // The two snap thresholds as floats through the stack (cdecl).
        let convex_snap = x87_quiet_bits(e.mem.u32(c + 0x90));
        let concave_snap = x87_quiet_bits(e.mem.u32(c + 0x94));
        e.call(CTOR_SOLVER_TABLES, &args![convex_snap, concave_snap]);
        copy_float(e, c + 0x90, w + 0xcc);
        copy_float(e, c + 0x90, w + 0xd0);
        copy_byte(e, c + 0x98, w + 0xd4);
        copy_byte(e, c + 0xa7, w + 0xd6);
        copy_float(e, c + 0xac, w + 0xdc);
        copy_word(e, c + 0xb0, w + 0xe0);
        copy_byte(e, c + 0xb4, w + 0xe4);
        copy_word(e, c + 0xb8, w + 0xe8);
        copy_word(e, c + 0xbc, w + 0xec);
        copy_float(e, c + 0xa8, w + 0xd8);
        // m_dynamicsStepInfo.m_solverInfo (+0x1e0): m_one, then tau and damping.
        e.mem.set_u32(w + 0x1e0, x87_store32(1.0));
        let tau = f32::from_bits(x87_quiet_bits(e.mem.u32(c + 0x78)));
        let damping = f32::from_bits(x87_quiet_bits(e.mem.u32(c + 0x7c)));
        fn_00c90e60(e, Ptr::new(w + 0x1e0), tau, damping);
        // m_contactRestingVelocity, m_numSteps and m_invNumSteps (FILD, FDIVRP).
        copy_float(e, c + 0x24, w + DYNAMICS_FLOAT_240);
        copy_word(e, c + 0x80, w + DYNAMICS_WORD_2F4);
        let iterations = f64::from(e.mem.u32(c + 0x80) as i32);
        e.mem
            .set_u32(w + 0x300, x87_store32(x87_div(1.0, iterations)));
        // m_numMicroSteps, m_maxConstraintViolationSqrd, m_invNumMicroSteps (FDIVR).
        copy_word(e, c + 0x84, w + DYNAMICS_WORD_2F8);
        let violation = x87_load32(e.mem.u32(c + 0x88));
        e.mem.set_u32(
            w + DYNAMICS_FLOAT_308,
            x87_store32(x87_mul(violation, violation)),
        );
        let micro_steps = f64::from(e.mem.u32(c + 0x84) as i32);
        e.mem
            .set_u32(w + 0x2fc, x87_store32(x87_div(1.0, micro_steps)));
        // m_forceCoherentConstraintOrderingInSolver, the two inactive-frame flags and the
        // integrate counter; m_deltaTime and m_invDeltaTime are 0.
        copy_byte(e, c + 0x8c, w + DYNAMICS_BYTE_304);
        copy_byte(e, c + 0xa4, w + DYNAMICS_BYTES_305);
        copy_byte(e, c + 0xa5, w + DYNAMICS_BYTES_305 + 1);
        e.mem.set_u32(w + 0x2ec, x87_store32(0.0));
        e.mem.set_u32(w + 0x2f0, x87_store32(0.0));
        copy_byte(e, c + 0xa6, w + DYNAMICS_BYTES_305 + 2);
        // The gravity length of the cinfo (SSE), 9.81 when it is 0.0 (FUCOMPP; JP skips
        // the default for "not equal" and unordered).
        let length = x87_load32(ctor_vector_length(e, c + 0x10).to_bits());
        let mut gravity_bits = x87_store32(length);
        if length == 0.0 {
            gravity_bits = x87_store32(x87_load32(e.global::<u32>(CTOR_DEFAULT_GRAVITY)));
        }
        ctor_deactivation_levels(e, w, c, x87_load32(gravity_bits));
        // 00c96057: m_memoryWatchDog takes the cinfo's, with its reference.
        let watchdog = e.mem.u32(c + 0x6c);
        if watchdog != 0 {
            e.call(ADD_REFERENCE, &args![watchdog]);
        }
        let previous = e.mem.u32(w + 0x50);
        if previous != 0 {
            e.call(REMOVE_REFERENCE, &args![previous]);
        }
        copy_word(e, c + 0x6c, w + 0x50);
        // m_wantSimulationIslands and m_wantDeactivation (off without islands).
        let islands = e.mem.u8(c + 0xd2);
        e.mem.set_u8(w + 0xc8, islands);
        let deactivation = e.mem.u8(c + 0xd0);
        e.mem.set_u8(w + 0xd5, deactivation);
        if islands == 0 && deactivation != 0 {
            e.mem.set_u8(w + 0xd5, islands);
        }
        copy_byte(e, c + 0xd8, w + 0xb0);
        e.mem.set_u8(w + 0xb1, 0);
        // m_broadPhaseDispatcher.
        let block = router_alloc(e, 0x10c);
        let dispatcher = if block != 0 {
            e.call(CTOR_BROAD_PHASE_DISPATCHER, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(w + 0x64, dispatcher);
        // m_phantomBroadPhaseListener (no null check).
        let listener = router_alloc(e, 0xc);
        e.mem.set_u16(listener + 4, 0xc);
        e.mem.set_u16(listener + 6, 1);
        e.mem.set_u32(listener, CTOR_PHANTOM_LISTENER_VTABLE);
        e.mem.set_u32(listener + 8, CTOR_PHANTOM_LISTENER_TABLE);
        e.mem.set_u32(w + 0x68, listener);
        // m_entityEntityBroadPhaseListener.
        let block = router_alloc(e, 0x10);
        e.mem.set_u16(block + 4, 0x10);
        let listener = e.call(CTOR_ENTITY_ENTITY_LISTENER, &args![block, w]).u32();
        e.mem.set_u32(w + 0x6c, listener);
        // m_broadPhaseBorderListener.
        let listener = router_alloc(e, 0xc);
        e.mem.set_u16(listener + 4, 0xc);
        e.mem.set_u16(listener + 6, 1);
        e.mem.set_u32(listener, CTOR_BORDER_LISTENER_VTABLE);
        e.mem.set_u32(listener + 8, CTOR_BORDER_LISTENER_TABLE);
        e.mem.set_u32(w + 0x70, listener);
        // The dispatcher's listener slots: each listener's interface at +8 (0 for none),
        // both pointers read again for every slot.
        for (source, slot) in [
            (0x68, 0x28),
            (0x68, 0x44),
            (0x68, 0x48),
            (0x6c, 0x24),
            (0x70, 0x2c),
            (0x70, 0x64),
            (0x70, 0x4c),
            (0x70, 0x68),
            (0x70, 0x6c),
        ] {
            let listener = e.mem.u32(w + source);
            let interface = if listener != 0 { listener + 8 } else { 0 };
            let dispatcher = e.mem.u32(w + 0x64);
            e.mem.set_u32(dispatcher + slot, interface);
        }
        // m_collisionDispatcher, built with a contact manager factory whose creation
        // reference is dropped afterwards.
        let block = router_alloc(e, 0xc);
        e.mem.set_u16(block + 4, 0xc);
        let factory = e.call(CTOR_CONTACT_MGR_FACTORY, &args![block, w]).u32();
        let block = router_alloc(e, 0x1e50);
        e.mem.set_u16(block + 4, 0x1e50);
        let collision_dispatcher = e
            .call(
                CTOR_COLLISION_DISPATCHER,
                &args![block, CTOR_DEFAULT_AGENT_FUNCTION, factory],
            )
            .u32();
        e.mem.set_u32(w + 0x80, collision_dispatcher);
        e.call(REMOVE_REFERENCE, &args![factory]);
        // m_collisionFilter.
        let filter = e.mem.u32(c + 0x58);
        if filter == 0 {
            let filter = router_alloc(e, 0x30);
            e.mem.set_u16(filter + 4, 0x30);
            e.call(CTOR_DEFAULT_COLLISION_FILTER, &args![filter]);
            e.mem.set_u32(filter, CTOR_FILTER_TABLES[0]);
            e.mem.set_u32(filter + 8, CTOR_FILTER_TABLES[1]);
            e.mem.set_u32(filter + 0xc, CTOR_FILTER_TABLES[2]);
            e.mem.set_u32(filter + 0x10, CTOR_FILTER_TABLES[3]);
            e.mem.set_u32(filter + 0x14, CTOR_FILTER_TABLES[4]);
            e.mem.set_u32(filter + 0x20, 1);
            e.mem.set_u32(w + 0x7c, filter);
        } else {
            e.mem.set_u32(w + 0x7c, filter);
            e.call(ADD_REFERENCE, &args![filter]);
        }
        // m_convexListFilter.
        let convex_filter = e.mem.u32(c + 0x5c);
        if convex_filter == 0 {
            let convex_filter = router_alloc(e, 8);
            e.mem.set_u16(convex_filter + 4, 8);
            e.mem.set_u16(convex_filter + 6, 1);
            e.mem.set_u32(convex_filter, CTOR_CONVEX_LIST_FILTER_VTABLE);
            e.mem.set_u32(w + 0x84, convex_filter);
        } else {
            e.mem.set_u32(w + 0x84, convex_filter);
            e.call(ADD_REFERENCE, &args![convex_filter]);
        }
        // m_collisionInput (`hkpProcessCollisionInput`, 0x80 bytes; used without a null
        // check after the allocation's).
        let block = router_alloc(e, 0x80);
        let input = if block != 0 {
            e.mem.set_u32(block + 0x64, 0);
            e.mem.set_u32(block + 4, 0);
            block
        } else {
            0
        };
        e.mem.set_u32(w + 0x78, input);
        e.mem.set_u32(input + 4, 0);
        copy_word(e, w + 0x80, input);
        copy_float(e, c + 0x54, input + 8);
        copy_word(e, w + 0x7c, input + 0xc);
        copy_word(e, w + 0x84, input + 0x10);
        copy_byte(e, c + 0x74, w + 0x340);
        copy_byte(e, c + 0x75, input + 0x6d);
        // m_config (`hkpCollisionAgentConfig`): starts at 0.01 and 0x14, then takes the
        // cinfo's iterative linear cast settings (no null check).
        let block = router_alloc(e, 0x10);
        let config = if block != 0 {
            e.mem.set_u32(block + 4, 0x14);
            let hundredth = x87_store32(x87_load32(e.global::<u32>(CTOR_HUNDREDTH)));
            e.mem.set_u32(block, hundredth);
            block
        } else {
            0
        };
        e.mem.set_u32(input + 0x70, config);
        copy_float(e, c + 0x9c, config);
        let iterations = e.mem.u32(c + 0xa0);
        let config = e.mem.u32(input + 0x70);
        e.mem.set_u32(config + 4, iterations);
        copy_byte(e, c + 0x99, input + 0x6c);
        let quality = e.mem.u32(input).wrapping_add(0x1c60);
        e.mem.set_u32(input + 0x14, 0);
        e.mem.set_u32(input + 0x60, quality);
        ctor_collision_input_extents(e, input, c);
        // The broad phase's slot +0x70 with (+0x20, +0x30, +0x40) of the input.
        let broad_phase = e.mem.u32(w + 0x58);
        e.vcall(
            broad_phase,
            0x70,
            &args![input + 0x20, input + 0x30, input + 0x40],
        );
        let collision_dispatcher = e.mem.u32(w + 0x80);
        fn_00c912e0(e, Ptr::new(w), Ptr::new(collision_dispatcher));
        if e.global::<u8>(CTOR_FORCE_MULTITHREADED) != 0 {
            e.mem.set_u8(c + 0xd1, 3);
        }
        // m_minDesiredIslandSize, m_simulationType and the simulation.
        e.mem.set_u32(w + 0xb4, 0);
        let simulation_type = e.mem.u8(c + 0xd1) as i8 as i32;
        e.mem.set_u32(w + 0xf0, simulation_type as u32);
        match simulation_type {
            1 => {
                let block = router_alloc(e, 0x2c);
                e.mem.set_u16(block + 4, 0x2c);
                let simulation = e.call(CTOR_SIMULATION, &args![block, w]).u32();
                e.mem.set_u32(w + 8, simulation);
            }
            2 => {
                let block = router_alloc(e, 0x50);
                e.mem.set_u16(block + 4, 0x50);
                let simulation = e.call(CTOR_CONTINUOUS_SIMULATION, &args![block, w]).u32();
                e.mem.set_u32(w + 8, simulation);
            }
            3 => {
                for slot in [0xb8, 0xc4] {
                    let block = router_alloc(e, 0x18);
                    let section = if block != 0 {
                        e.call(CTOR_CRITICAL_SECTION, &args![block, 0xfa0u32]).u32()
                    } else {
                        0
                    };
                    e.mem.set_u32(w + slot, section);
                }
                let block = router_alloc(e, 0x180);
                e.mem.set_u16(block + 4, 0x180);
                let simulation = e
                    .call(CTOR_MULTI_THREADED_SIMULATION, &args![block, w])
                    .u32();
                e.mem.set_u32(w + 8, simulation);
                copy_word(e, c + 0xd4, w + 0xb4);
            }
            _ => {}
        }
        // 00c96600: m_islandDirtyListCriticalSection.
        let block = router_alloc(e, 0x18);
        let section = if block != 0 {
            e.call(CTOR_CRITICAL_SECTION, &args![block, 0xfa0u32]).u32()
        } else {
            0
        };
        let simulation = e.mem.u32(w + 8);
        e.mem.set_u32(w + 0xc0, section);
        // The simulation's +0x24 (m_frameMarkerPsiSnap).
        copy_float(e, c + 0xdc, simulation + 0x24);
        // The world's gravity length (SSE), 9.81 when 0.0.
        let length = x87_load32(ctor_vector_length(e, w + 0x10).to_bits());
        let mut gravity_bits = x87_store32(length);
        if length == 0.0 {
            gravity_bits = x87_store32(x87_load32(e.global::<u32>(CTOR_DEFAULT_GRAVITY)));
        }
        // The four penetration counts.
        copy_float(e, c + 0xc0, w + 0xf4);
        copy_float(e, c + 0xc4, w + 0xf8);
        copy_float(e, c + 0xc8, w + 0xfc);
        copy_float(e, c + 0xcc, w + 0x100);
        // The collision-quality input (`frame + 0x34`, see `00cfb570`).
        let info = frame + 0x34;
        let wants_continuous = e.mem.u8(c + 0xd1) as i8 >= 2;
        let input = e.mem.u32(w + 0x78);
        e.mem.set_u32(info, x87_store32(x87_load32(gravity_bits)));
        copy_float(e, input + 8, info + 4);
        let generation = e.mem.u8(c + 0x74) as i8;
        e.mem.set_u8(info + 0x28, u8::from(wants_continuous));
        copy_float(e, c + 0x68, info + 8);
        copy_float(e, c + 0x60, info + 0xc);
        e.mem.set_u8(info + 0x29, u8::from(generation == 2));
        copy_float(e, c + 0xc0, info + 0x10);
        e.mem.set_u8(info + 0x2a, u8::from(generation >= 1));
        copy_float(e, c + 0xc4, info + 0x14);
        copy_float(e, c + 0xc8, info + 0x18);
        e.mem.set_u16(info + 0x22, 3);
        copy_float(e, c + 0xcc, info + 0x1c);
        e.mem.set_u16(info + 0x24, 4);
        e.mem.set_u16(info + 0x20, 1);
        copy_byte(e, c + 0x98, info + 0x2b);
        e.mem.set_u16(info + 0x26, 5);
        let collision_dispatcher = e.mem.u32(w + 0x80);
        e.call(CTOR_COLLISION_QUALITY, &args![collision_dispatcher, info]);
        let quality = e.mem.u32(w + 0x80).wrapping_add(0x1c60);
        let input = e.mem.u32(w + 0x78);
        e.mem.set_u32(input + 0x60, quality);
        // m_fixedIsland: storage index 0xffff, two flag pairs of byte +0x26 cleared.
        let block = router_alloc(e, 0x6c);
        e.mem.set_u16(block + 4, 0x6c);
        let island = e.call(CTOR_SIMULATION_ISLAND, &args![block, w]).u32();
        e.mem.set_u32(w + 0x20, island);
        e.mem.set_u16(island + 0x20, 0xffff);
        let island = e.mem.u32(w + 0x20);
        let flags = e.mem.u8(island + 0x26);
        e.mem.set_u8(island + 0x26, flags & 0xf3);
        let island = e.mem.u32(w + 0x20);
        let flags = e.mem.u8(island + 0x26);
        e.mem.set_u8(island + 0x26, flags & 0xfc);
        if e.mem.u8(w + 0xc8) == 0 {
            // Without simulation islands: one active island, storage index 0.
            let block = router_alloc(e, 0x6c);
            e.mem.set_u16(block + 4, 0x6c);
            let island = e.call(CTOR_SIMULATION_ISLAND, &args![block, w]).u32();
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let capacity = e.mem.u32(w + 0x30) & 0x3fff_ffff;
            if e.mem.u32(w + 0x2c) == capacity {
                let allocator = e.mem.u32(router + 0x10);
                e.call(CTOR_ARRAY_GROW, &args![allocator, w + 0x28, 4u32]);
            }
            let size = e.mem.u32(w + 0x2c);
            let data = e.mem.u32(w + 0x28);
            e.mem
                .set_u32(data.wrapping_add(size.wrapping_mul(4)), island);
            e.mem.set_u32(w + 0x2c, size.wrapping_add(1));
            e.mem.set_u16(island + 0x20, 0);
        }
        // m_fixedRigidBody: a rigid body cinfo with mass 0 and motion type 5.
        let body_cinfo = frame + 0x60;
        e.call(CTOR_RIGID_BODY_CINFO, &args![body_cinfo]);
        e.mem.set_u32(body_cinfo + 0x90, x87_store32(0.0));
        e.mem.set_u8(body_cinfo + 0xb4, 5);
        let block = router_alloc(e, 0x220);
        e.mem.set_u16(block + 4, 0x220);
        let body = e.call(CTOR_RIGID_BODY, &args![block, body_cinfo]).u32();
        e.mem.set_u32(w + 0x24, body);
        hkp_world_add_entity(e, this, Ptr::new(body), 1);
        let body = e.mem.u32(w + 0x24);
        e.call(REMOVE_REFERENCE, &args![body]);
        // m_dynamicsStepInfo.m_stepInfo (+0x1d0): start 0, end 1/60 (float), delta =
        // 1/60 (double) - 0 through a float, and its inverse (0.0 for a zero delta: FUCOM,
        // JP takes "not equal" and unordered to the division).
        e.mem.set_u32(w + 0x1d0, x87_store32(0.0));
        let step_end = x87_store32(x87_load32(e.global::<u32>(CTOR_STEP_TIME)));
        e.mem.set_u32(w + 0x1d4, step_end);
        let step = x87_load64(e.global::<u64>(CTOR_STEP_DOUBLE));
        let delta = x87_load32(x87_store32(x87_sub(step, 0.0)));
        e.mem.set_u32(w + 0x1d8, x87_store32(delta));
        if delta == 0.0 {
            e.mem.set_u32(w + 0x1dc, x87_store32(0.0));
        } else {
            e.mem.set_u32(w + 0x1dc, x87_store32(x87_div(1.0, delta)));
        }
        let input = e.mem.u32(w + 0x78);
        e.mem.set_u32(input + 0x68, w + 0x1d0);
        // m_broadPhaseBorder, unless m_broadPhaseBorderBehaviour is 3.
        if e.mem.u8(c + 0x28) == 3 {
            e.mem.set_u32(w + 0x1bc, 0);
        } else {
            let block = router_alloc(e, 0x44);
            e.mem.set_u16(block + 4, 0x44);
            let behaviour = e.mem.u8(c + 0x28) as i8 as i32 as u32;
            let postpone = u32::from(e.mem.u8(c + 0x29));
            let border = e
                .call(
                    CTOR_BROAD_PHASE_BORDER,
                    &args![block, w, behaviour, postpone],
                )
                .u32();
            e.mem.set_u32(w + 0x1bc, border);
        }
        // m_maintenanceMgr, initialized through its slot +0xc with the world.
        let block = router_alloc(e, 0x10);
        e.mem.set_u16(block + 4, 0x10);
        let manager = e.call(CTOR_MAINTENANCE_MGR, &args![block]).u32();
        e.mem.set_u32(w + 0x4c, manager);
        e.vcall(manager, 0xc, &args![w]);
        // m_collisionCallbackUtil.
        e.mem.set_u32(w + 0x1c0, 0);
        if e.mem.u8(c + 0xe0) != 0 {
            e.call(CTOR_COLLISION_CALLBACKS, &args![w]);
        }
        // m_kdTreeManager and m_autoUpdateTree.
        if e.mem.u8(c + 0x50) != 0 {
            let block = router_alloc(e, 0x24);
            e.mem.set_u16(block + 4, 0x24);
            let manager = e.call(CTOR_KD_TREE_MANAGER, &args![block, w, 0u32]).u32();
            e.mem.set_u32(w + 0x5c, manager);
            copy_byte(e, c + 0x53, w + 0x60);
        } else if e.mem.u8(c + 0x51) != 0 {
            let block = router_alloc(e, 0x70);
            e.mem.set_u16(block + 4, 0x70);
            let update_type = e.mem.u8(c + 0x52) as i8 as i32 as u32;
            let manager = e
                .call(
                    CTOR_AABB_TREE_MANAGER,
                    &args![block, w, update_type, x87_store32(0.0)],
                )
                .u32();
            e.mem.set_u32(w + 0x5c, manager);
            copy_byte(e, c + 0x53, w + 0x60);
        } else {
            e.mem.set_u32(w + 0x5c, 0);
        }
        if e.mem.u8(c + 0xd1) != 3 {
            e.call(CTOR_MULTI_THREAD_CHECK_DISABLE, &args![w + 0xa8]);
        }
        // m_multithreadedSimulationJobData.
        let block = router_alloc(e, 0x120);
        if block != 0 {
            e.mem.set_u32(block + 0xf4, 0);
            e.mem.set_u32(block + 0x94, 0);
            e.mem.set_u32(w + 0x74, block);
        } else {
            e.mem.set_u32(w + 0x74, 0);
        }
    });
    this
}

// Translated from 00c96ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::updateCollisionFilterOnWorld` (Xbox PDB), `(mode, flag)`. While critical
/// operations are locked the request is queued (operation 0x15: mode byte at +1, flag byte
/// at +2). Otherwise blocks the execution of pending operations (+0x9C) while it works:
/// - mode 0: runs `updateCollisionFilterOnEntity(entity, 0, flag)` for every entity of every
///   active and then inactive island, and `updateCollisionFilterOnPhantom(phantom, flag)` for
///   every phantom;
/// - any other mode: inside the critical section, walks the agent entries of every island
///   (blocks of 0x200 bytes at the island's +0x5C, the last one `+0x58` bytes long; an
///   entry's length is its byte +3). An entry whose collidables (+0x10, +0x14) are refused
///   by the collision filter or whose group pair is disabled is collected, the island is
///   marked (byte +0x25), and its agents removed after the island's walk (`00cc1160`); with
///   flag 1 an accepted entry is updated instead (`00cfeba0`).
/// Then lifts the block, flushes the pending operations when allowed and the lock is 0.
pub fn hkp_world_update_collision_filter_on_world(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    mode: i32,
    flag: u32,
) {
    if operations_locked(e, this) {
        queue_operation(e, this, 0x15, |e, operation| {
            e.mem.set_u8(operation + 1, mode as u8);
            e.mem.set_u8(operation + 2, flag as u8);
        });
        return;
    }
    let world = this.addr();
    e.set(this, HkpWorld::m_blockExecutingPendingOperations, true);
    profile_record(e, PROFILE_UPDATE_FILTER_ON_WORLD);
    if mode == 0 {
        for (list, size) in [(0x28u32, 0x2cu32), (0x34, 0x38)] {
            let mut island_index = 0i32;
            while island_index < e.mem.i32(world + size) {
                let islands = e.mem.u32(world + list);
                let island = e
                    .mem
                    .u32(islands.wrapping_add((island_index as u32).wrapping_mul(4)));
                let mut body_index = 0i32;
                while body_index < e.mem.i32(island + 0x4c) {
                    let bodies = e.mem.u32(island + 0x48);
                    let entity = e
                        .mem
                        .u32(bodies.wrapping_add((body_index as u32).wrapping_mul(4)));
                    hkp_world_update_collision_filter_on_entity(e, this, entity, 0, flag);
                    body_index += 1;
                }
                island_index += 1;
            }
        }
        let mut index = 0i32;
        while index < e.mem.i32(world + 0x114) {
            let phantoms = e.mem.u32(world + 0x110);
            let phantom = e
                .mem
                .u32(phantoms.wrapping_add((index as u32).wrapping_mul(4)));
            hkp_world_update_collision_filter_on_phantom(e, this, phantom, flag);
            index += 1;
        }
    } else {
        begin_critical_section(e, this);
        for list in [world + 0x28, world + 0x34] {
            e.with_stack(0x90, |e, collected| {
                let collected = collected.addr();
                e.mem.set_u32(collected, collected + 0x10);
                e.mem.set_u32(collected + 8, 0x8000_0020);
                let mut island_index = 0i32;
                while island_index < e.mem.i32(list + 4) {
                    let islands = e.mem.u32(list);
                    let island = e
                        .mem
                        .u32(islands.wrapping_add((island_index as u32).wrapping_mul(4)));
                    e.mem.set_u32(collected + 4, 0);
                    update_filter_on_island(e, this, island, collected, flag);
                    island_index += 1;
                }
                array_release(e, collected, 4, 0x10);
            });
        }
        let lock = e
            .get(this, HkpWorld::m_criticalOperationsLockCount)
            .wrapping_sub(1);
        e.set(this, HkpWorld::m_criticalOperationsLockCount, lock);
    }
    e.set(this, HkpWorld::m_blockExecutingPendingOperations, false);
    if e.get(this, HkpWorld::m_criticalOperationsLockCount) == 0 {
        leave_critical_section(e, this);
    }
    profile_record(e, PROFILE_END);
}

/// The agent entry walk of `updateCollisionFilterOnWorld` for one island: `collected`
/// receives the entries to remove, which are removed once the island's blocks are done.
/// `flag` is the world call's second argument: when it is 1 an accepted entry is updated.
fn update_filter_on_island(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    island: u32,
    collected: u32,
    flag: u32,
) {
    let world = this.addr();
    if e.mem.i32(island + 0x60) <= 0 {
        return;
    }
    let mut block = 0i32;
    loop {
        let start = e.mem.u32(
            e.mem
                .u32(island + 0x5c)
                .wrapping_add((block as u32).wrapping_mul(4)),
        );
        block += 1;
        let end = if block == e.mem.i32(island + 0x60) {
            e.mem.u32(island + 0x58).wrapping_add(start)
        } else {
            start.wrapping_add(0x200)
        };
        let mut entry = start;
        while entry < end {
            let first = e.mem.u32(entry + 0x10);
            let second = e.mem.u32(entry + 0x14);
            let filter = e.mem.u32(world + 0x7c);
            let accepted = filter_accepts(e, filter, first, second);
            let first_group = e.mem.i8(e.mem.u32(entry + 0x10) + 0x1a);
            let second_group = e.mem.i8(e.mem.u32(entry + 0x14) + 0x1a);
            if !accepted || !group_pair_enabled(e, world, first_group, second_group) {
                array_push_word(e, collected, entry);
                let marks = e.mem.u8(island + 0x25);
                e.mem.set_u8(island + 0x25, (marks & 0xfd) | 1);
            } else if flag == 1 {
                // 00c972b6: CMP dword [ESP+0xc0],1 tests the second argument (`flag`), not
                // `mode`.
                let input = e.mem.u32(world + 0x78);
                e.call(UPDATE_SHAPE_COLLECTION_FILTER, &args![entry, input, island]);
            }
            entry = entry.wrapping_add(e.mem.u8(entry + 3) as u32);
        }
        if block >= e.mem.i32(island + 0x60) {
            break;
        }
    }
    while e.mem.u32(collected + 4) != 0 {
        let size = e.mem.u32(collected + 4).wrapping_sub(1);
        e.mem.set_u32(collected + 4, size);
        let agent = e
            .mem
            .u32(e.mem.u32(collected).wrapping_add(size.wrapping_mul(4)));
        e.call(REMOVE_AGENT, &args![agent]);
    }
}

// Translated from 00c97370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::addAction` (Xbox PDB). While critical operations are locked the request is
/// queued (operation 0xA: action at +4) and 0 is returned. Otherwise inside the critical
/// section: adds a reference, asks the action for its entities (vtable slot 0x10, into a
/// list of up to 4 on the stack), sets the action's world (+8), appends the action to each
/// entity's action list (+0x208; grown with `00d06cb0` when full), and for each entity whose
/// island (+0xCC) is a real one (word +0x20 not 0xFFFF) puts the action into the first such
/// island (`00d07470`) and merges the islands of the other entities with it (`00cb5570`).
/// Without such an entity the action goes to the island of the first listed entity. Then
/// the critical section ends, the world's "action added" callback fires (`00d003c0`).
/// Returns the action.
pub fn hkp_world_add_action(e: &mut Engine, this: Ptr<HkpWorld>, action: u32) -> u32 {
    if operations_locked(e, this) {
        queue_operation(e, this, 0xa, |e, operation| {
            e.mem.set_u32(operation + 4, action);
        });
        return 0;
    }
    e.call(ADD_REFERENCE, &args![action]);
    begin_critical_section(e, this);
    e.with_stack(0x20, |e, frame| {
        let entities = frame.addr();
        e.mem.set_u32(entities, entities + 0x10);
        e.mem.set_u32(entities + 8, 0x8000_0004);
        e.vcall(action, 0x10, &args![entities]);
        e.mem.set_u32(action + 8, this.addr());
        let mut first_entity = 0u32;
        let mut index = 0i32;
        while index < e.mem.i32(entities + 4) {
            let entity = e.mem.u32(
                e.mem
                    .u32(entities)
                    .wrapping_add((index as u32).wrapping_mul(4)),
            );
            let list = entity + 0x208;
            let capacity = e.mem.u16(list + 6) & 0x3fff;
            if e.mem.u16(list + 4) == capacity {
                e.call(ACTION_LIST_GROW, &args![list, 4u32]);
            }
            let size = e.mem.u16(list + 4);
            let data = e.mem.u32(list);
            e.mem
                .set_u32(data.wrapping_add(u32::from(size) * 4), action);
            e.mem.set_u16(list + 4, size.wrapping_add(1));
            let entity = e.mem.u32(
                e.mem
                    .u32(entities)
                    .wrapping_add((index as u32).wrapping_mul(4)),
            );
            let island = e.mem.u32(entity + 0xcc);
            if e.mem.u16(island + 0x20) != 0xffff {
                if first_entity == 0 {
                    e.call(ISLAND_ADD_ACTION, &args![island, action]);
                    first_entity = entity;
                } else if e.mem.u32(first_entity + 0xcc) != island {
                    e.call(MERGE_ISLANDS, &args![this, first_entity, entity]);
                }
            }
            index += 1;
        }
        if first_entity == 0 {
            let first = e.mem.u32(e.mem.u32(entities));
            let island = e.mem.u32(first + 0xcc);
            e.call(ISLAND_ADD_ACTION, &args![island, action]);
        }
        end_critical_section(e, this);
        e.call(FIRE_ACTION_ADDED, &args![this, action]);
        array_release(e, entities, 4, 0x10);
    });
    action
}

// Translated from 00c97510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removeActionImmediately` (Xbox PDB). Adds a reference to the action, enters
/// the critical section, fires the world's "action removed" callback (`00d00460`), asks the
/// action for its entities (vtable slot 0x10) and takes it out of each one's action list
/// (+0x208: the slot found, or -1, gets the last entry; the 16-bit size drops by one), takes
/// it out of its island (+0xC; `00d07130`), clears its world (+8) and drops one reference.
/// An island whose word +0x22 is 0xFFFF is then recorded in the island's owner list (owner
/// at +0x18, list at +0x40) and given the list's size as its index. The critical section
/// ends (pending operations flushed) and the reference from the start is removed.
pub fn hkp_world_remove_action_immediately(e: &mut Engine, this: Ptr<HkpWorld>, action: u32) {
    e.call(ADD_REFERENCE, &args![action]);
    begin_critical_section(e, this);
    e.call(FIRE_ACTION_REMOVED, &args![this, action]);
    e.with_stack(0x20, |e, frame| {
        let entities = frame.addr();
        e.mem.set_u32(entities, entities + 0x10);
        e.mem.set_u32(entities + 8, 0x8000_0004);
        e.vcall(action, 0x10, &args![entities]);
        let mut index = 0i32;
        while index < e.mem.i32(entities + 4) {
            let entity = e.mem.u32(
                e.mem
                    .u32(entities)
                    .wrapping_add((index as u32).wrapping_mul(4)),
            );
            let list = entity + 0x208;
            let count = i32::from(e.mem.u16(list + 4));
            let mut slot = -1i32;
            if count > 0 {
                let data = e.mem.u32(list);
                for candidate in 0..count {
                    if e.mem.u32(data.wrapping_add(candidate as u32 * 4)) == action {
                        slot = candidate;
                        break;
                    }
                }
            }
            let new_size = e.mem.u16(list + 4).wrapping_add(0xffff);
            e.mem.set_u16(list + 4, new_size);
            let data = e.mem.u32(list);
            let last = e.mem.u32(data.wrapping_add(u32::from(new_size) * 4));
            e.mem
                .set_u32(data.wrapping_add((slot as u32).wrapping_mul(4)), last);
            index += 1;
        }
        let island = e.mem.u32(action + 0xc);
        e.call(ISLAND_REMOVE_ACTION, &args![island, action]);
        e.mem.set_u32(action + 8, 0);
        e.call(REMOVE_REFERENCE, &args![action]);
        let owner = e.mem.u32(island + 0x18);
        if e.mem.u16(island + 0x22) == 0xffff {
            let index = e.mem.u16(owner + 0x44);
            e.mem.set_u16(island + 0x22, index);
            array_push_word(e, owner + 0x40, island);
        }
        array_release(e, entities, 4, 0x10);
    });
    end_critical_section(e, this);
    e.call(REMOVE_REFERENCE, &args![action]);
}

// Translated from 00c97d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::addConstraint` (Xbox PDB). While critical operations are locked the request is
/// queued (operation 8: constraint at +4) and 0 is returned. Otherwise: when the constraint's
/// data (+0xC) says it is of type 0xC (vtable slot 0x20) and its word +0x1C is 0, the
/// object at its +0x18 (slot 0x24) is asked for two words that go to +0x1C and +0x1E;
/// blocks pending operations (+0x9C) while it runs `00cf3e60`, gives the constraint the
/// next uid (`m_lastConstraintUid`, stored at +0x34), adds it to the simulation islands
/// (`00cb3ca0`); a constraint of kind 1 (vtable slot 0x18) also adds its action (+0x44) when
/// that has no world yet, and calls slot 0x40 of its data. Finally lifts the block and flushes
/// the pending operations when no critical section is open. Returns the `00cb3ca0` result.
pub fn hkp_world_add_constraint(e: &mut Engine, this: Ptr<HkpWorld>, constraint: u32) -> u32 {
    if operations_locked(e, this) {
        queue_operation(e, this, 8, |e, operation| {
            e.mem.set_u32(operation + 4, constraint);
        });
        return 0;
    }
    let world = this.addr();
    let data = e.mem.u32(constraint + 0xc);
    if e.vcall(data, 0x20, &args![]).u32() == 0xc && e.mem.u16(data + 0x1c) == 0 {
        let owner = e.mem.u32(data + 0x18);
        e.with_stack(8, |e, local| {
            let local = local.addr();
            // The flag byte is the low byte of a pushed register that held the local's
            // address: the upper bytes are whatever the address was.
            e.vcall(owner, 0x24, &args![(local & !0xff) | 1, local]);
            let first = e.mem.u16(local);
            let second = e.mem.u16(local + 4);
            e.mem.set_u16(data + 0x1c, first);
            e.mem.set_u16(data + 0x1e, second);
        });
    }
    e.set(this, HkpWorld::m_blockExecutingPendingOperations, true);
    e.call(CONSTRAINT_PREPARE, &args![constraint]);
    let uid = e.get(this, HkpWorld::m_lastConstraintUid).wrapping_add(1);
    e.set(this, HkpWorld::m_lastConstraintUid, uid);
    e.mem.set_u32(constraint + 0x34, uid);
    let result = e
        .call(ADD_CONSTRAINT_SI, &args![this, constraint, 1u32])
        .u32();
    if e.vcall(constraint, 0x18, &args![]).u32() == 1 {
        let action = e.mem.u32(constraint + 0x44);
        if e.mem.u32(action + 8) == 0 {
            hkp_world_add_action(e, this, action);
        }
        let data = e.mem.u32(constraint + 0xc);
        e.vcall(data, 0x40, &args![]);
    }
    let unlocked = !operations_locked(e, this);
    e.set(this, HkpWorld::m_blockExecutingPendingOperations, false);
    if unlocked && e.mem.u32(world + 0x8c) != 0 {
        fn_00c91120(e, this);
    }
    result
}

// Translated from 00c97e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removeConstraint` (Xbox PDB), with the `hkBool*` result as the first stack
/// argument. While critical operations are locked the request is queued (operation 9:
/// constraint at +4) and `*result` is 0. Otherwise inside the critical section: a constraint
/// of kind 1 (vtable slot 0x18) whose action (+0x44) belongs to this world has the action
/// removed immediately; the constraint gets a reference, leaves the simulation islands
/// (`00cb3dd0`), is detached (`00cf3040`) and loses the reference; when the critical
/// section ends the pending operations are flushed. `*result` is 1.
/// Answers `result`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hkp_world_remove_constraint(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    result: Ptr,
    constraint: u32,
) -> Ptr {
    if operations_locked(e, this) {
        queue_operation(e, this, 9, |e, operation| {
            e.mem.set_u32(operation + 4, constraint);
        });
        e.mem.set_u8(result.addr(), 0);
        return result;
    }
    e.set(this, HkpWorld::m_criticalOperationsLockCount, 1);
    if e.vcall(constraint, 0x18, &args![]).u32() == 1 {
        let action = e.mem.u32(constraint + 0x44);
        if e.mem.u32(action + 8) == this.addr() {
            hkp_world_remove_action_immediately(e, this, action);
        }
    }
    e.call(ADD_REFERENCE, &args![constraint]);
    e.call(REMOVE_CONSTRAINT_SI, &args![this, constraint, 1u32]);
    e.call(CONSTRAINT_DETACH, &args![constraint, this]);
    e.call(REMOVE_REFERENCE, &args![constraint]);
    end_critical_section(e, this);
    e.mem.set_u8(result.addr(), 1);
    result
}

// Translated from 00c97f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removeAction` (Xbox PDB). While critical operations are locked the request is
/// queued (operation 0xB: action at +4); otherwise it is `removeActionImmediately`.
pub fn hkp_world_remove_action(e: &mut Engine, this: Ptr<HkpWorld>, action: u32) {
    if operations_locked(e, this) {
        queue_operation(e, this, 0xb, |e, operation| {
            e.mem.set_u32(operation + 4, action);
        });
        return;
    }
    hkp_world_remove_action_immediately(e, this, action);
}

// Translated from 00c98310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x14 (`SUB ECX,0x14`) and jumps to `00c982e0`. `00c982e0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00c98310(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00c9_82e0, &args![this.addr().wrapping_sub(0x14), flags])
        .u32()
}

// Translated from 00c98320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `00c982e0`. `00c982e0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00c98320(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00c9_82e0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00c98330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0xc (`SUB ECX,0xc`) and jumps to `00c982e0`. `00c982e0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00c98330(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00c9_82e0, &args![this.addr().wrapping_sub(0xc), flags])
        .u32()
}

// Translated from 00c98340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x10 (`SUB ECX,0x10`) and jumps to `00c982e0`. `00c982e0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_00c98340(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00c9_82e0, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

// ---------------------------------------------------------------------------------------
// The second batch of `hkpWorld` functions (`00c92a30` to `00c983b0`): constants.
// ---------------------------------------------------------------------------------------

/// `hkpWorld` functions the translations of this batch call by address (so that tests can
/// replace them): `addEntity` (`00c914d0`, `(world, entity, flag)`), `removeEntity`
/// (`00c91620`, `(world, result byte, entity)`), `removePhantom` (`00c92c70`), `removeAll`
/// (`00c93580`), `addEntityBatch` (`00c94bd0`, `(world, entities, count, mode)`), `addPhantom`
/// (`00c936a0`), `addPhantomBatch` (`00c954e0`, `(world, phantoms, count)`), `addAction`
/// (`00c97370`), `addConstraint` (`00c97d30`) and `updateCollisionFilterOnWorld` (`00c96ff0`,
/// `(world, first, second)`).
const ADD_ENTITY: u32 = 0x00c9_14d0;
const REMOVE_ENTITY: u32 = 0x00c9_1620;
const REMOVE_PHANTOM: u32 = 0x00c9_2c70;
const REMOVE_ALL: u32 = 0x00c9_3580;
const ADD_ENTITY_BATCH: u32 = 0x00c9_4bd0;
const ADD_PHANTOM: u32 = 0x00c9_36a0;
const ADD_PHANTOM_BATCH: u32 = 0x00c9_54e0;
const ADD_ACTION: u32 = 0x00c9_7370;
const ADD_CONSTRAINT: u32 = 0x00c9_7d30;
const UPDATE_COLLISION_FILTER_ON_WORLD: u32 = 0x00c9_6ff0;
/// The destructor `hkpWorld::~hkpWorld` (`00c93e50`), called by the deleting destructor.
const WORLD_DESTRUCTOR: u32 = 0x00c9_3e50;
/// Takes an island off the world's dirty list (`00cb3fd0`, `(world, island)`).
const REMOVE_ISLAND_FROM_DIRTY_LIST: u32 = 0x00cb_3fd0;
/// The world listener broadcasts "remove all" (`00d00f30`) and "deleted" (`00d00e80`), `(world)`.
const FIRE_WORLD_REMOVE_ALL: u32 = 0x00d0_0f30;
const FIRE_WORLD_DELETED: u32 = 0x00d0_0e80;
/// `hkpSimulation::stepBeginSt` (`00d022b0`, `(simulation, queue, time)`).
const STEP_BEGIN_ST: u32 = 0x00d0_22b0;
/// Processes all the jobs of a job queue (`00d051b0`, `ECX` the queue).
const PROCESS_ALL_JOBS: u32 = 0x00d0_51b0;
/// Statistics collector helpers: the allocated size of a block (`00d06f80`, `(pointer,
/// requested size)`) and `addNormalChunk` (`00d06fb0`, `(collector, name, pointer, size, 0)`).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
const ADD_NORMAL_CHUNK: u32 = 0x00d0_6fb0;
/// Recalculates the bounding volumes of entities (`00d1a330`, `(collision input, bodies,
/// count)`).
const ENTITY_BATCH_RECALC_AABB: u32 = 0x00d1_a330;
/// The tree queries of `getClosestPoints`: the AABB tree's (`00d1d4a0`) and the kd tree's
/// (`00d1b850`), each `(tree, bounds, pointer array)`.
const AABB_TREE_QUERY: u32 = 0x00d1_d4a0;
const KD_TREE_QUERY: u32 = 0x00d1_b850;
/// The default collision filter's constructor (`00cbfba0`) and the five vtables of the 0x30-byte
/// object `setCollisionFilter` makes (`.rdata`): at +0, +8, +0xC, +0x10 and +0x14.
const COLLISION_FILTER_CONSTRUCT: u32 = 0x00cb_fba0;
const NULL_FILTER_VTABLE_0: u32 = 0x010c_646c;
const NULL_FILTER_VTABLE_8: u32 = 0x010c_6460;
const NULL_FILTER_VTABLE_C: u32 = 0x010c_644c;
const NULL_FILTER_VTABLE_10: u32 = 0x010c_6440;
const NULL_FILTER_VTABLE_14: u32 = 0x010c_6434;
/// `hkpConstraintInstance`'s constructor (`00cf3ea0`, `(block, entity a, entity b, data,
/// priority)`).
const CONSTRAINT_INSTANCE_CONSTRUCT: u32 = 0x00cf_3ea0;
/// `DeleteCriticalSection` (the import's thunk slot, `00fdf058`).
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// Destructors the world's destructor calls: the dispatch record's (`00cf7060`) and the
/// operation queue's (`00cf75f0`), `ECX` the object.
const DISPATCH_RECORD_DESTRUCT: u32 = 0x00cf_7060;
const OPERATION_QUEUE_DESTRUCT: u32 = 0x00cf_75f0;
/// The base destructor of the ray caster (`00cfa860`, `ECX` the object) and the exe's `free`
/// of an object (`00401030`, cdecl).
const RAY_CASTER_BASE_DESTRUCT: u32 = 0x00cf_a860;
const FREE_OBJECT: u32 = 0x0040_1030;
/// Vtables (`.rdata`): the world's (`010c6714`), the `hkReferencedObject` base (`0102e388`),
/// the ray caster base (`010c3bb8`) and the collision filter base (`010c63bc`).
const WORLD_VTABLE: u32 = 0x010c_6714;
const REFERENCED_OBJECT_VTABLE: u32 = 0x0102_e388;
const RAY_CASTER_BASE_VTABLE: u32 = 0x010c_3bb8;
const FILTER_BASE_VTABLE: u32 = 0x010c_63bc;
/// `0.0` as a `double` (`.rdata`), what `getPenetrations` clamps the tolerance with.
const ZERO_DOUBLE: u32 = 0x0101_2060;

/// Profile labels (`.rdata`) of `shiftBroadPhase`, `getPenetrations` and `getClosestPoints`
/// (a label is the address of its text).
const PROFILE_SHIFT_BROAD_PHASE: u32 = 0x010c_68c8;
const PROFILE_GET_PENETRATIONS: u32 = 0x010c_68ec;
const PROFILE_GET_PENETRATIONS_NARROW: u32 = 0x010c_68dc;
const PROFILE_CLOSEST_TREES: u32 = 0x010c_699c;
const PROFILE_CLOSEST_TREES_BROAD_PHASE: u32 = 0x010c_698c;
const PROFILE_CLOSEST_AABB_TREE_NARROW: u32 = 0x010c_697c;
const PROFILE_CLOSEST_KD_TREE_BROAD_PHASE: u32 = 0x010c_6970;
const PROFILE_CLOSEST_KD_TREE_NARROW: u32 = 0x010c_6964;
const PROFILE_CLOSEST_BROAD_PHASE: u32 = 0x010c_6938;
const PROFILE_CLOSEST_BROAD_PHASE_SECOND: u32 = 0x010c_692c;
const PROFILE_CLOSEST_NARROW: u32 = 0x010c_6920;

/// The names `calcContentStatistics` reports to the collector (the addresses of the exe's
/// strings).
const STATISTICS_ACTIVE_ISLANDS: u32 = 0x010c_6700;
const STATISTICS_ACTIVE: u32 = 0x0109_c938;
const STATISTICS_INACTIVE_ISLANDS: u32 = 0x010c_66f0;
const STATISTICS_INACTIVE: u32 = 0x010c_66e4;
const STATISTICS_FIXED: u32 = 0x010c_6084;
const STATISTICS_PHANTOM_POINTERS: u32 = 0x010c_66d8;
const STATISTICS_PHANTOMS: u32 = 0x010c_41a4;
const STATISTICS_MODIFY_CONSTRAINT_SECTION: u32 = 0x010c_66b8;
const STATISTICS_ISLAND_DIRTY_LIST_SECTION: u32 = 0x010c_6698;
const STATISTICS_PROPERTY_MASTER_LOCK: u32 = 0x010c_6684;
const STATISTICS_JOB_DATA: u32 = 0x010c_6664;
const STATISTICS_PENDING_QUEUES: u32 = 0x010c_664c;
const STATISTICS_PENDING_QUEUES_ARRAY: u32 = 0x010c_6630;
const STATISTICS_INTERNAL: u32 = 0x010c_6624;
const STATISTICS_SIMULATION: u32 = 0x010c_6618;
const STATISTICS_DIRTY_ISLAND_POINTERS: u32 = 0x010c_660c;
const STATISTICS_MAINTENANCE_MANAGER: u32 = 0x010c_6600;
const STATISTICS_OPERATION_QUEUE: u32 = 0x010c_65f8;
const STATISTICS_ISLAND_MERGES: u32 = 0x010c_65e8;
const STATISTICS_BODY_OPERATIONS: u32 = 0x010c_65d8;
const STATISTICS_BROAD_PHASE_BORDER: u32 = 0x010c_65c4;
const STATISTICS_COLLIDE: u32 = 0x010c_65bc;
const STATISTICS_BROAD_PHASE: u32 = 0x010c_65b0;
const STATISTICS_BROAD_PHASE_DISPATCH: u32 = 0x010c_65a4;
const STATISTICS_COLLISION_INPUT: u32 = 0x010c_6598;
const STATISTICS_AGENT_CONFIG: u32 = 0x010c_6580;
const STATISTICS_FILTER: u32 = 0x010c_6578;
const STATISTICS_CONVEX_LIST_FILTER: u32 = 0x010c_6564;
const STATISTICS_DISPATCHER: u32 = 0x010c_6558;
const STATISTICS_LISTENERS: u32 = 0x010c_654c;
const STATISTICS_ENTITY_LISTENERS: u32 = 0x010c_6540;
const STATISTICS_PHANTOM_LISTENERS: u32 = 0x010c_6534;
const STATISTICS_CONSTRAINT_LISTENERS: u32 = 0x010c_6524;
const STATISTICS_DELETION_LISTENERS: u32 = 0x010c_6514;
const STATISTICS_ISLAND_ACTIVATION_LISTENERS: u32 = 0x010c_6504;
const STATISTICS_POST_SIMULATION_LISTENERS: u32 = 0x010c_64f4;
const STATISTICS_POST_INTEGRATE_LISTENERS: u32 = 0x010c_64e4;
const STATISTICS_POST_COLLIDE_LISTENERS: u32 = 0x010c_64d4;
const STATISTICS_COLLISION_LISTENERS: u32 = 0x010c_64c8;

use crate::units::havok_sdk::hkpconvexradiusbuilder::addps;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, store_vec};

// Translated from 00c92a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Two stack words (`RET 8`): `object` and `argument`. While critical operations
/// are locked the request is queued (operation 0x24: object at +4, argument at +8); otherwise
/// the object's virtual function at vtable offset 0xC runs with `argument`. Returns `object`.
pub fn fn_00c92a30(e: &mut Engine, this: Ptr<HkpWorld>, object: u32, argument: u32) -> u32 {
    if operations_locked(e, this) {
        queue_operation(e, this, 0x24, |e, operation| {
            e.mem.set_u32(operation + 4, object);
            e.mem.set_u32(operation + 8, argument);
        });
        return object;
    }
    e.vcall(object, 0xc, &args![argument]);
    object
}

// Translated from 00c92a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Two stack words (`RET 8`): `first` and `second`. Reads the simulation's float at
/// +0x18 twice through the x87 stack and builds a four-float record on its stack: the two
/// copies, their difference and the reciprocal of the difference (0 when the difference
/// compares equal to zero). Calls the simulation's virtual function at vtable offset 0x1C
/// with `first`, `second`, the world, the record and 1.
pub fn fn_00c92a80(e: &mut Engine, this: Ptr<HkpWorld>, first: u32, second: u32) {
    let simulation = e.get(this, HkpWorld::m_simulation).addr();
    let lower = x87_quiet(e.mem.f32(simulation + 0x18));
    let upper = x87_quiet(e.mem.f32(simulation + 0x18));
    let span = (f64::from(upper) - f64::from(lower)) as f32;
    let inverse = if span == 0.0 {
        0.0
    } else {
        (1.0 / f64::from(span)) as f32
    };
    let simulation = e.get(this, HkpWorld::m_simulation).addr();
    e.with_stack(0x10, |e, record| {
        e.mem.set_f32(record.addr(), lower);
        e.mem.set_f32(record.addr() + 4, upper);
        e.mem.set_f32(record.addr() + 8, span);
        e.mem.set_f32(record.addr() + 0xc, inverse);
        e.vcall(simulation, 0x1c, &args![first, second, this, record, 1u32]);
    });
}

// Translated from 00c92b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Fastcall, no stack word. Calls `00c92a80` with the words at +0x48 and +0x4C of
/// every active island (`+0x28`, size `+0x2C`), then of every inactive one (`+0x34`, size
/// `+0x38`); the sizes are read again after each call.
pub fn fn_00c92b00(e: &mut Engine, this: Ptr<HkpWorld>) {
    let mut index = 0i32;
    while index < e.get(this, HkpWorld::m_activeSimulationIslandsSize) {
        let data = e.get(this, HkpWorld::m_activeSimulationIslands).addr();
        let island = array_entry(e, data, index);
        let (first, second) = (e.mem.u32(island + 0x48), e.mem.u32(island + 0x4c));
        fn_00c92a80(e, this, first, second);
        index = index.wrapping_add(1);
    }
    index = 0;
    while index < e.get(this, HkpWorld::m_inactiveSimulationIslandsSize) {
        let data = e.get(this, HkpWorld::m_inactiveSimulationIslands).addr();
        let island = array_entry(e, data, index);
        let (first, second) = (e.mem.u32(island + 0x48), e.mem.u32(island + 0x4c));
        fn_00c92a80(e, this, first, second);
        index = index.wrapping_add(1);
    }
}

// Translated from 00c92b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::getMemUsageForIntegration` (Xbox PDB). One stack word (`RET 4`): `usage`, three
/// words that are cleared and then filled: the largest single island's estimate (`[0]`), the
/// sum over the active islands (`[1]`) and the island with the largest estimate (`[2]`, the
/// first found from the last island down). An island's estimate is
/// `((((2 * (word[+0x1C] / 80) + 8) * 3 + word[+0x14]) * 4 + 0x17) & !0xF)
/// + ((2 * (word[+0x1C] / 80) + 0x12 + word[+0x4C] * 8) << 4) + word[+0xC]`.
pub fn hkp_world_get_mem_usage_for_integration(e: &mut Engine, this: Ptr<HkpWorld>, usage: u32) {
    e.mem.set_u32(usage, 0);
    e.mem.set_u32(usage + 4, 0);
    e.mem.set_u32(usage + 8, 0);
    let mut index = e
        .get(this, HkpWorld::m_activeSimulationIslandsSize)
        .wrapping_sub(1);
    while index >= 0 {
        let data = e.get(this, HkpWorld::m_activeSimulationIslands).addr();
        let island = array_entry(e, data, index);
        let rows = (e.mem.i32(island + 0x1c) / 80).wrapping_mul(2);
        let base = rows.wrapping_add(8);
        let words = base.wrapping_mul(3).wrapping_add(e.mem.i32(island + 0x14));
        let header = words.wrapping_mul(4).wrapping_add(0x17) & !0xf;
        let body = base
            .wrapping_add(e.mem.i32(island + 0x4c).wrapping_mul(8))
            .wrapping_add(10)
            .wrapping_shl(4);
        let estimate = header
            .wrapping_add(body)
            .wrapping_add(e.mem.i32(island + 0xc));
        let total = e.mem.i32(usage + 4).wrapping_add(estimate);
        e.mem.set_i32(usage + 4, total);
        if estimate > e.mem.i32(usage) {
            e.mem.set_i32(usage, estimate);
            e.mem.set_u32(usage + 8, island);
        }
        index = index.wrapping_sub(1);
    }
}

// Translated from 00c92bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Two stack words (`RET 8`): `action` and `entity`. Appends the action to the
/// entity's action list (+0x208, grown through `00d06cb0` when the 16-bit size equals the
/// capacity). If the action's island (+0xC) has the 16-bit word 0xFFFF at +0x20 and the
/// entity's byte at +0xE8 is not 5, the action leaves that island (`00d07130`) and joins the
/// entity's island (+0xCC, `00d07470`). Otherwise, when the two islands differ and the byte is
/// not 5, `00cb5570` merges them (the world, the entity and the first entity of the action's
/// island).
pub fn fn_00c92bd0(e: &mut Engine, this: Ptr<HkpWorld>, action: u32, entity: u32) {
    let list = entity + 0x208;
    if e.mem.u16(list + 4) == e.mem.u16(list + 6) & 0x3fff {
        e.call(ACTION_LIST_GROW, &args![list, 4u32]);
    }
    let size = e.mem.u16(list + 4);
    let data = e.mem.u32(list);
    e.mem
        .set_u32(data.wrapping_add(u32::from(size) * 4), action);
    e.mem.set_u16(list + 4, size.wrapping_add(1));
    let island = e.mem.u32(action + 0xc);
    if e.mem.u16(island + 0x20) == 0xffff && e.mem.u8(entity + 0xe8) != 5 {
        e.call(ISLAND_REMOVE_ACTION, &args![island, action]);
        let entity_island = e.mem.u32(entity + 0xcc);
        e.call(ISLAND_ADD_ACTION, &args![entity_island, action]);
        return;
    }
    if e.mem.u32(entity + 0xcc) != island && e.mem.u8(entity + 0xe8) != 5 {
        let first = e.mem.u32(e.mem.u32(island + 0x48));
        e.call(MERGE_ISLANDS, &args![this, entity, first]);
    }
}

// Translated from 00c92d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::initMtStep` (Xbox PDB). Two stack words (`RET 8`): `queue` and `time` (a float,
/// passed through the x87 stack). When the world has a memory watch dog (+0x50) it counts the
/// lock up (`00c91180`), runs `watchHeapMemory` (`00d01d20`) and counts it down (`00c91190`).
/// Then it tail-calls the simulation's (+8) `stepBeginSt` (`00d022b0`) and returns its `EAX`.
pub fn hkp_world_init_mt_step(e: &mut Engine, this: Ptr<HkpWorld>, queue: u32, time: f32) -> u32 {
    if !e.get(this, HkpWorld::m_memoryWatchDog).is_null() {
        fn_00c91180(e, this);
        e.call(WATCH_HEAP_MEMORY, &args![this]);
        fn_00c91190(e, this);
    }
    let time = x87_quiet(time);
    let simulation = e.get(this, HkpWorld::m_simulation);
    e.call(STEP_BEGIN_ST, &args![simulation, queue, time.to_bits()])
        .u32()
}

/// Reports one `hkArray` (data +0, size +4, capacity word +8) of `element_size`-byte elements to
/// the statistics collector the way every array of `calcContentStatistics` is: when the
/// capacity word's top bit is clear or the collector's byte at +0x10 is set, the collector's
/// virtual function at 0x14 gets `(3, name, data, size * element_size, allocated)`, where
/// `allocated` is `getAllocatedSize(data, (capacity & 0x3FFFFFFF) * element_size)`.
fn report_array(e: &mut Engine, collector: u32, array: u32, name: u32, element_size: u32) {
    let capacity = e.mem.u32(array + 8);
    if (capacity as i32) >= 0 || e.mem.u8(collector + 0x10) != 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(element_size);
        let allocated = e.call(GET_ALLOCATED_SIZE, &args![data, bytes]).u32();
        let data = e.mem.u32(array);
        let used = e.mem.u32(array + 4).wrapping_mul(element_size);
        e.vcall(collector, 0x14, &args![3u32, name, data, used, allocated]);
    }
}

/// `collector.addNormalChunk(name, pointer, size, 0)` (`00d06fb0`).
fn add_normal_chunk(e: &mut Engine, collector: u32, name: u32, pointer: u32, size: u32) {
    e.call(
        ADD_NORMAL_CHUNK,
        &args![collector, name, pointer, size, 0u32],
    );
}

/// `collector.addChildObject(name, object, 0)` (virtual function at 0xC).
fn add_child_object(e: &mut Engine, collector: u32, name: u32, object: u32) {
    e.vcall(collector, 0xc, &args![name, object, 0u32]);
}

// Translated from 00c92db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::calcContentStatistics` (Xbox PDB). Two stack words (`RET 8`): the statistics
/// `collector` and one that is not read. Stores the collision input (+0x78) at collector +8
/// for the duration, reports to the collector (the names are the exe's strings): the active
/// islands array (+0x28) and each active island (last to first), the inactive ones (+0x34),
/// the fixed island, the phantoms array (+0x110) and each phantom, the critical sections
/// (+0xB8 when present, +0xC0, +0xC4 when present), the job data (+0x74), the chain of pending
/// operation queues (+0xA0), then inside "Internal" the simulation (+8), the dirty islands
/// array (+0x40), the maintenance manager (+0x4C) and the operation queue (+0x88) with its
/// two arrays, the broad phase border (+0x1BC), inside "Collide" the broad phase (+0x58), the
/// dispatch records (+0x64, +0x6C, +0x68, +0x70), the collision input and its agent config,
/// the filters (+0x7C, +0x84) and the dispatcher (+0x80), and inside "Listeners" the
/// listener arrays at +0x128, +0x134, +0x140, +0x14C, +0x158, +0x164, +0x170, +0x17C, +0x188
/// and +0x1A0. The collector's virtual functions: 0xC child object, 0x14 array, 0x18 begin
/// section, 0x1C end section. Finally clears collector +8.
pub fn hkp_world_calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    collector: u32,
    _unused_3: u32,
) {
    let t = this.addr();
    let collision_input = e.get(this, HkpWorld::m_collisionInput);
    e.mem.set_u32(collector + 8, collision_input.addr());
    report_array(e, collector, t + 0x28, STATISTICS_ACTIVE_ISLANDS, 4);
    let mut index = e.mem.i32(t + 0x2c).wrapping_sub(1);
    while index >= 0 {
        let island = array_entry(e, e.mem.u32(t + 0x28), index);
        add_child_object(e, collector, STATISTICS_ACTIVE, island);
        index = index.wrapping_sub(1);
    }
    report_array(e, collector, t + 0x34, STATISTICS_INACTIVE_ISLANDS, 4);
    index = e.mem.i32(t + 0x38).wrapping_sub(1);
    while index >= 0 {
        let island = array_entry(e, e.mem.u32(t + 0x34), index);
        add_child_object(e, collector, STATISTICS_INACTIVE, island);
        index = index.wrapping_sub(1);
    }
    let fixed_island = e.mem.u32(t + 0x20);
    add_child_object(e, collector, STATISTICS_FIXED, fixed_island);
    report_array(e, collector, t + 0x110, STATISTICS_PHANTOM_POINTERS, 4);
    index = 0;
    while e.mem.i32(t + 0x114) > index {
        let phantom = array_entry(e, e.mem.u32(t + 0x110), index);
        add_child_object(e, collector, STATISTICS_PHANTOMS, phantom);
        index = index.wrapping_add(1);
    }
    let section = e.mem.u32(t + 0xb8);
    if section != 0 {
        add_normal_chunk(
            e,
            collector,
            STATISTICS_MODIFY_CONSTRAINT_SECTION,
            section,
            0x18,
        );
    }
    let section = e.mem.u32(t + 0xc0);
    add_normal_chunk(
        e,
        collector,
        STATISTICS_ISLAND_DIRTY_LIST_SECTION,
        section,
        0x18,
    );
    let lock = e.mem.u32(t + 0xc4);
    if lock != 0 {
        add_normal_chunk(e, collector, STATISTICS_PROPERTY_MASTER_LOCK, lock, 0x18);
    }
    let job_data = e.mem.u32(t + 0x74);
    add_normal_chunk(e, collector, STATISTICS_JOB_DATA, job_data, 0x120);
    let mut queue = e.mem.u32(t + 0xa0);
    while queue != 0 {
        let first = e.mem.u32(t + 0xa0);
        add_normal_chunk(e, collector, STATISTICS_PENDING_QUEUES, first, 0x10);
        let array = e.mem.u32(e.mem.u32(t + 0xa0));
        if array != 0 {
            report_array(e, collector, array, STATISTICS_PENDING_QUEUES_ARRAY, 0x14);
        }
        queue = e.mem.u32(queue + 8);
    }
    e.vcall(collector, 0x18, &args![STATISTICS_INTERNAL]);
    let simulation = e.mem.u32(t + 8);
    add_child_object(e, collector, STATISTICS_SIMULATION, simulation);
    report_array(e, collector, t + 0x40, STATISTICS_DIRTY_ISLAND_POINTERS, 4);
    let manager = e.mem.u32(t + 0x4c);
    add_child_object(e, collector, STATISTICS_MAINTENANCE_MANAGER, manager);
    let operations = e.mem.u32(t + 0x88);
    add_normal_chunk(e, collector, STATISTICS_OPERATION_QUEUE, operations, 0x28);
    e.vcall(collector, 0x18, &args![STATISTICS_OPERATION_QUEUE]);
    let operations = e.mem.u32(t + 0x88);
    report_array(
        e,
        collector,
        operations + 0x10,
        STATISTICS_ISLAND_MERGES,
        0x14,
    );
    let operations = e.mem.u32(t + 0x88);
    report_array(
        e,
        collector,
        operations + 0x1c,
        STATISTICS_BODY_OPERATIONS,
        0x10,
    );
    e.vcall(collector, 0x1c, &args![]);
    let border = e.mem.u32(t + 0x1bc);
    add_child_object(e, collector, STATISTICS_BROAD_PHASE_BORDER, border);
    e.vcall(collector, 0x18, &args![STATISTICS_COLLIDE]);
    let broad_phase = e.mem.u32(t + 0x58);
    add_child_object(e, collector, STATISTICS_BROAD_PHASE, broad_phase);
    for (offset, size) in [(0x64, 0x10c), (0x6c, 0x10), (0x68, 0xc), (0x70, 0xc)] {
        let record = e.mem.u32(t + offset);
        add_normal_chunk(e, collector, STATISTICS_BROAD_PHASE_DISPATCH, record, size);
    }
    let collision_input = e.mem.u32(t + 0x78);
    add_normal_chunk(
        e,
        collector,
        STATISTICS_COLLISION_INPUT,
        collision_input,
        0x80,
    );
    let collision_input = e.mem.u32(t + 0x78);
    if collision_input != 0 {
        let config = e.mem.u32(collision_input + 0x70);
        add_normal_chunk(e, collector, STATISTICS_AGENT_CONFIG, config, 0x10);
    }
    let filter = e.mem.u32(t + 0x7c);
    add_child_object(e, collector, STATISTICS_FILTER, filter);
    let filter = e.mem.u32(t + 0x84);
    add_child_object(e, collector, STATISTICS_CONVEX_LIST_FILTER, filter);
    let dispatcher = e.mem.u32(t + 0x80);
    add_child_object(e, collector, STATISTICS_DISPATCHER, dispatcher);
    e.vcall(collector, 0x1c, &args![]);
    e.vcall(collector, 0x1c, &args![]);
    e.vcall(collector, 0x18, &args![STATISTICS_LISTENERS]);
    for (array, name) in [
        (0x128, STATISTICS_ENTITY_LISTENERS),
        (0x134, STATISTICS_PHANTOM_LISTENERS),
        (0x140, STATISTICS_CONSTRAINT_LISTENERS),
        (0x14c, STATISTICS_DELETION_LISTENERS),
        (0x158, STATISTICS_ISLAND_ACTIVATION_LISTENERS),
        (0x164, STATISTICS_POST_SIMULATION_LISTENERS),
        (0x170, STATISTICS_POST_INTEGRATE_LISTENERS),
        (0x17c, STATISTICS_POST_COLLIDE_LISTENERS),
        (0x188, STATISTICS_POST_INTEGRATE_LISTENERS),
        (0x1a0, STATISTICS_COLLISION_LISTENERS),
    ] {
        report_array(e, collector, t + array, name, 4);
    }
    e.vcall(collector, 0x1c, &args![]);
    e.mem.set_u32(collector + 8, 0);
}

// Translated from 00c93570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::getMemoryWatchDog` (Xbox PDB): `m_memoryWatchDog` (+0x50).
pub fn hkp_world_get_memory_watch_dog(e: &mut Engine, this: Ptr<HkpWorld>) -> Ptr {
    e.get(this, HkpWorld::m_memoryWatchDog)
}

// Translated from 00c93580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::removeAll` (Xbox PDB). Fastcall, no stack word. Removes every phantom (`00c92c70`
/// on the first, until `m_phantoms` is empty), then the entities of the active islands (while
/// the first island has any), of the inactive islands and of the fixed island, each through
/// `removeEntity` (`00c91620`, first with the fixed body: a reference is added, the body
/// removed, and later added back with `addEntity` (`00c914d0`, second argument 1) and the
/// reference dropped again). Unless `+0xC8` is set the last active island leaves the dirty
/// list (`00cb3fd0`) and is deleted through its vtable slot 0, and the active island count
/// drops by one. Ends with the world's "remove all" callback (`00d00f30`).
pub fn hkp_world_remove_all(e: &mut Engine, this: Ptr<HkpWorld>) {
    let t = this.addr();
    e.with_stack(4, |e, scratch| {
        let result: Ptr = scratch.byte_add(3);
        while e.mem.i32(t + 0x114) > 0 {
            let phantom = e.mem.u32(e.mem.u32(t + 0x110));
            e.call(REMOVE_PHANTOM, &args![this, phantom]);
        }
        while e.mem.i32(t + 0x2c) > 0 {
            let island = e.mem.u32(e.mem.u32(t + 0x28));
            if e.mem.i32(island + 0x4c) == 0 {
                break;
            }
            let entity = e.mem.u32(e.mem.u32(island + 0x48));
            e.call(REMOVE_ENTITY, &args![this, result, entity]);
        }
        while e.mem.i32(t + 0x38) > 0 {
            let island = e.mem.u32(e.mem.u32(t + 0x34));
            let entity = e.mem.u32(e.mem.u32(island + 0x48));
            e.call(REMOVE_ENTITY, &args![this, result, entity]);
        }
        let fixed_body = e.mem.u32(t + 0x24);
        e.call(ADD_REFERENCE, &args![fixed_body]);
        let fixed_body = e.mem.u32(t + 0x24);
        e.call(REMOVE_ENTITY, &args![this, result, fixed_body]);
        let mut fixed_island = e.mem.u32(t + 0x20);
        while e.mem.i32(fixed_island + 0x4c) > 0 {
            let entity = e.mem.u32(e.mem.u32(fixed_island + 0x48));
            e.call(REMOVE_ENTITY, &args![this, result, entity]);
            fixed_island = e.mem.u32(t + 0x20);
        }
        let fixed_body = e.mem.u32(t + 0x24);
        e.call(ADD_ENTITY, &args![this, fixed_body, 1u32]);
        let fixed_body = e.mem.u32(t + 0x24);
        e.call(REMOVE_REFERENCE, &args![fixed_body]);
    });
    if !e.get(this, HkpWorld::m_addsToNewIsland) {
        let last = array_entry(e, e.mem.u32(t + 0x28), e.mem.i32(t + 0x2c).wrapping_sub(1));
        e.call(REMOVE_ISLAND_FROM_DIRTY_LIST, &args![this, last]);
        let last = array_entry(e, e.mem.u32(t + 0x28), e.mem.i32(t + 0x2c).wrapping_sub(1));
        if last != 0 {
            e.vcall(last, 0, &args![1u32]);
        }
        let count = e.mem.i32(t + 0x2c).wrapping_sub(1);
        e.mem.set_i32(t + 0x2c, count);
    }
    e.call(FIRE_WORLD_REMOVE_ALL, &args![this]);
}

// Translated from 00c937a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. One stack word (`RET 4`): `value`. Appends it to the pointer array at +0x11C
/// (data, size +0x120, capacity word +0x124; grown through `00c90940` when full).
pub fn fn_00c937a0(e: &mut Engine, this: Ptr<HkpWorld>, value: u32) {
    array_push_word(e, this.addr() + 0x11c, value);
}

// Translated from 00c937e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the "ConstrLisPtr" array of the statistics). One stack word: `listener`. Appends
/// it to the pointer array at +0x140.
pub fn fn_00c937e0(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    array_push_word(e, this.addr() + 0x140, listener);
}

// Translated from 00c93820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the "EntLisPtr" array of the statistics). One stack word: `listener`. Appends it
/// to the pointer array at +0x128.
pub fn fn_00c93820(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    array_push_word(e, this.addr() + 0x128, listener);
}

// Translated from 00c93860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the "PhantLisPtr" array of the statistics). One stack word: `listener`. Appends
/// it to the pointer array at +0x134.
pub fn fn_00c93860(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    array_push_word(e, this.addr() + 0x134, listener);
}

// Translated from 00c938a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the "IslActLisPtr" array of the statistics). One stack word: `listener`. Appends
/// it to the pointer array at +0x158.
pub fn fn_00c938a0(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    array_push_word(e, this.addr() + 0x158, listener);
}

// Translated from 00c93920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::addWorldPostSimulationListener` (Xbox PDB). One stack word: `listener`.
/// Appends it to the pointer array at +0x164.
pub fn hkp_world_add_world_post_simulation_listener(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    listener: u32,
) {
    array_push_word(e, this.addr() + 0x164, listener);
}

// Translated from 00c93a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the "CollLisPtr" array of the statistics). One stack word: `listener`. Appends it
/// to the pointer array at +0x1A0.
pub fn fn_00c93a20(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    array_push_word(e, this.addr() + 0x1a0, listener);
}

// Translated from 00c93a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the "WldDelLisPtr" array of the statistics). One stack word: `listener`. Appends
/// it to the pointer array at +0x14C.
pub fn fn_00c93a60(e: &mut Engine, this: Ptr<HkpWorld>, listener: u32) {
    array_push_word(e, this.addr() + 0x14c, listener);
}

// Translated from 00c93ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::stepMultithreaded` (Xbox PDB). Three stack words (`RET 0xC`): `queue`,
/// `listener` and `time` (a float, passed through the x87 stack). Runs `initMtStep`
/// (`00c92d70`); a non-zero result is returned at once. Otherwise calls the listener's virtual
/// function at 0xC with the queue and 0xB, processes all the jobs of the queue (`00d051b0`,
/// `ECX` the queue), calls the listener's virtual function at 0x10 and returns `finishMtStep`
/// (`00c91c00`) of the queue and the listener.
pub fn hkp_world_step_multithreaded(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    queue: u32,
    listener: u32,
    time: f32,
) -> u32 {
    let started = hkp_world_init_mt_step(e, this, queue, x87_quiet(time));
    if started != 0 {
        return started;
    }
    e.vcall(listener, 0xc, &args![queue, 0xbu32]);
    e.call(PROCESS_ALL_JOBS, &args![queue]);
    e.vcall(listener, 0x10, &args![]);
    hkp_world_finish_mt_step(e, this, queue, listener)
}

// Translated from 00c93b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. One stack word (`RET 4`): `systems`, an `hkArray` of pointers. Builds a physics
/// system (0x44 bytes through the memory router, word +4 = 0x44, constructed by `00d065e0`)
/// and fills it with the fixed and inactive bodies (`00c926a0`). If there were any, the byte
/// at +0x40 is cleared, the system is appended to `systems` and a second system is built the
/// same way (also when the first allocation came back null); the system is appended to
/// `systems` and gets the active islands' bodies (`00c92760`), the phantoms (`00c92950`), the
/// constraints (`00c92860`) and the actions (`00c928c0`) of the world.
pub fn fn_00c93b30(e: &mut Engine, this: Ptr<HkpWorld>, systems: u32) {
    let mut system = new_physics_system(e);
    let had_bodies = e.with_stack(4, |e, flag| {
        let flag = fn_00c926a0(e, flag, this, Ptr::new(system));
        e.mem.u8(flag.addr()) != 0
    });
    if had_bodies {
        e.mem.set_u8(system + 0x40, 0);
        array_push_word(e, systems, system);
        system = new_physics_system(e);
    } else if system == 0 {
        system = new_physics_system(e);
    }
    array_push_word(e, systems, system);
    e.with_stack(4, |e, flag| {
        fn_00c92760(e, flag, this, Ptr::new(system));
    });
    fn_00c92950(e, this, Ptr::new(system));
    fn_00c92860(e, this, Ptr::new(system));
    fn_00c928c0(e, this, Ptr::new(system));
}

/// Allocates and constructs one physics system: 0x44 bytes from the memory router with the
/// 16-bit word +4 set to 0x44, then `00d065e0` on the block.
fn new_physics_system(e: &mut Engine) -> u32 {
    let block = router_alloc(e, 0x44);
    e.mem.set_u16(block + 4, 0x44);
    e.call(PHYSICS_SYSTEM_CONSTRUCT, &args![block]).u32()
}

// Translated from 00c93d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the "finish loading" constructor of `hkpWorld`; one stack word, `RET 4`, is not
/// read). Stores the vtable `010c6714`, the 16-bit word +6 = 1, empty `hkArray`s (data 0, size
/// 0, capacity word 0x80000000) at +0x28, +0x34 and +0x40, zeroes +0x50, stores +0xA8 =
/// 0xFFFFFFF1 and the 16-bit word +0xAC = 0x8000, and makes the same empty arrays at +0x11C,
/// +0x128, +0x134, +0x140, +0x14C, +0x158, +0x164, +0x170, +0x17C, +0x188, +0x194, +0x1A0 and
/// +0x1AC. Returns `this`.
pub fn fn_00c93d10(e: &mut Engine, this: Ptr<HkpWorld>, _unused_2: u32) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, WORLD_VTABLE);
    e.mem.set_u16(t + 6, 1);
    for array in [0x28, 0x34, 0x40] {
        e.mem.set_u32(t + array, 0);
        e.mem.set_u32(t + array + 4, 0);
        e.mem.set_u32(t + array + 8, 0x8000_0000);
    }
    e.mem.set_u32(t + 0x50, 0);
    e.mem.set_u32(t + 0xa8, 0xffff_fff1);
    e.mem.set_u16(t + 0xac, 0x8000);
    for array in [
        0x11c, 0x128, 0x134, 0x140, 0x14c, 0x158, 0x164, 0x170, 0x17c, 0x188, 0x194, 0x1a0, 0x1ac,
    ] {
        e.mem.set_u32(t + array, 0);
        e.mem.set_u32(t + array + 4, 0);
        e.mem.set_u32(t + array + 8, 0x8000_0000);
    }
    Ptr::new(t)
}

/// Empties the `hkArray` of 4-byte elements at `array` and gives its storage back: the size
/// is cleared, the storage freed through the router allocator (slot 0x10) unless the capacity
/// word's top bit is set, the data pointer is cleared and the capacity word set to
/// 0x80000000.
fn reset_array(e: &mut Engine, array: u32) {
    array_release(e, array, 4, 0x10);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// Removes a reference from the object in the word at `slot` (the call is made even when the
/// word is null).
fn drop_reference_at(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    e.call(REMOVE_REFERENCE, &args![object]);
}

/// Deletes the object in the word at `slot` through its vtable slot 0 (argument 1) when it is
/// not null.
fn delete_object_at(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0 {
        e.vcall(object, 0, &args![1u32]);
    }
}

// Translated from 00c93e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::~hkpWorld` (Xbox PDB). Fastcall, no stack word. Stores the vtable `010c6714`;
/// frees the job data block (+0x74: its buffer +0xF4 of 0x200 bytes, then the 0x120-byte
/// block); drops the reference at +0x4C; deletes the critical sections at +0xB8, +0xC4 and
/// +0xC0 (`DeleteCriticalSection` and a 0x18-byte free each); drops the reference at +0x50
/// (when set) and runs `removeAll`; removes the fixed body (+0x24) and takes the fixed island
/// (+0x20) off the dirty list, deletes it and fires the world "deleted" callback (`00d00e80`);
/// drops the references at +0x58 and +0x80, destroys the 0x10C-byte dispatch record (+0x64,
/// `00cf7060`), drops +0x1BC (when set), deletes the objects at +0x68, +0x6C and +0x70, drops
/// +0x7C and +0x84, frees the collision input's agent config (+0x70 of it, 0x10 bytes) and the
/// collision input (its +0x64 buffer of 0x200 bytes, then 0x80 bytes), drops +8, destroys the
/// operation queue (+0x88, `00cf75f0`, 0x28 bytes), drops +0x5C when set, frees +0x1B8 (0x204
/// bytes), empties the pointer arrays at +0x1AC down to +0x110 (step 0xC), drops +0x50 again
/// and empties the arrays at +0x40, +0x34 and +0x28. Ends with the vtable `0102e388`.
pub fn hkp_world_destructor(e: &mut Engine, this: Ptr<HkpWorld>) {
    let t = this.addr();
    let job_data = e.mem.u32(t + 0x74);
    e.mem.set_u32(t, WORLD_VTABLE);
    if job_data != 0 {
        let buffer = e.mem.u32(job_data + 0xf4);
        if buffer != 0 {
            router_free(e, buffer, 0x200);
        }
        router_free(e, job_data, 0x120);
    }
    drop_reference_at(e, t + 0x4c);
    e.mem.set_u32(t + 0x4c, 0);
    for section_slot in [t + 0xb8, t + 0xc4, t + 0xc0] {
        let section = e.mem.u32(section_slot);
        if section != 0 {
            e.call(DELETE_CRITICAL_SECTION, &args![section]);
            router_free(e, section, 0x18);
            e.mem.set_u32(section_slot, 0);
        }
    }
    if e.mem.u32(t + 0x50) != 0 {
        drop_reference_at(e, t + 0x50);
    }
    e.mem.set_u32(t + 0x50, 0);
    e.call(REMOVE_ALL, &args![this]);
    e.with_stack(4, |e, scratch| {
        let fixed_body = e.mem.u32(t + 0x24);
        e.call(REMOVE_ENTITY, &args![this, scratch.byte_add(3), fixed_body]);
    });
    e.mem.set_u32(t + 0x24, 0);
    let fixed_island = e.mem.u32(t + 0x20);
    e.call(REMOVE_ISLAND_FROM_DIRTY_LIST, &args![this, fixed_island]);
    delete_object_at(e, t + 0x20);
    e.mem.set_u32(t + 0x20, 0);
    e.call(FIRE_WORLD_DELETED, &args![this]);
    drop_reference_at(e, t + 0x58);
    let held = e.mem.u32(t + 0x80);
    e.mem.set_u32(t + 0x58, 0);
    e.call(REMOVE_REFERENCE, &args![held]);
    let dispatch = e.mem.u32(t + 0x64);
    e.mem.set_u32(t + 0x80, 0);
    if dispatch != 0 {
        e.call(DISPATCH_RECORD_DESTRUCT, &args![dispatch]);
        router_free(e, dispatch, 0x10c);
    }
    if e.mem.u32(t + 0x1bc) != 0 {
        drop_reference_at(e, t + 0x1bc);
    }
    delete_object_at(e, t + 0x68);
    delete_object_at(e, t + 0x6c);
    delete_object_at(e, t + 0x70);
    drop_reference_at(e, t + 0x7c);
    drop_reference_at(e, t + 0x84);
    let collision_input = e.mem.u32(t + 0x78);
    let agent_config = e.mem.u32(collision_input + 0x70);
    if agent_config != 0 {
        router_free(e, agent_config, 0x10);
    }
    let collision_input = e.mem.u32(t + 0x78);
    if collision_input != 0 {
        let buffer = e.mem.u32(collision_input + 0x64);
        if buffer != 0 {
            router_free(e, buffer, 0x200);
        }
        router_free(e, collision_input, 0x80);
    }
    drop_reference_at(e, t + 8);
    let operations = e.mem.u32(t + 0x88);
    if operations != 0 {
        e.call(OPERATION_QUEUE_DESTRUCT, &args![operations]);
        router_free(e, operations, 0x28);
    }
    if e.mem.u32(t + 0x5c) != 0 {
        drop_reference_at(e, t + 0x5c);
    }
    let buffer = e.mem.u32(t + 0x1b8);
    if buffer != 0 {
        router_free(e, buffer, 0x204);
    }
    for array in [
        0x1ac, 0x1a0, 0x194, 0x188, 0x17c, 0x170, 0x164, 0x158, 0x14c, 0x140, 0x134, 0x128, 0x11c,
        0x110,
    ] {
        reset_array(e, t + array);
    }
    if e.mem.u32(t + 0x50) != 0 {
        drop_reference_at(e, t + 0x50);
    }
    e.mem.set_u32(t + 0x50, 0);
    for array in [0x40, 0x34, 0x28] {
        reset_array(e, t + array);
    }
    e.mem.set_u32(t, REFERENCED_OBJECT_VTABLE);
}

// Translated from 00c96a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::shiftBroadPhase` (Xbox PDB). Three stack words (`RET 0xC`): `region`, `offset`
/// (a pointer to a vector) and `skip_update` (a flag). Writes a profiling record, asks the
/// broad phase (virtual function 0x54) to shift by `offset`, collecting new pairs in a local
/// array, and tells it (0x58) the collision input's two vectors (+0x20, +0x30). The vector is
/// added to the world's vectors at +0x310 and +0x320 and to the broad phase border's six
/// objects (by the kind their virtual function 0x14 reports: kind 0 gets +0xB0 and +0xC0, kinds
/// 1 and 2 get +0xE0). With the lock count up, the new pairs become agents (`00cf7080`). Unless
/// `skip_update` is non-zero, the bodies of the fixed island and every active and inactive
/// island that have a shape are gathered (the buffer comes from the allocator's virtual
/// function 0xC) and their bounding volumes recalculated (`00d1a330`). The critical section
/// then ends, the closing record is written and the pair array freed.
pub fn hkp_world_shift_broad_phase(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    region: u32,
    offset: u32,
    skip_update: u32,
) {
    let t = this.addr();
    profile_record(e, PROFILE_SHIFT_BROAD_PHASE);
    e.with_stack(0x40, |e, frame| {
        let f = frame.addr();
        let new_pairs = f + 0x28;
        e.mem.set_u32(new_pairs, 0);
        e.mem.set_u32(new_pairs + 4, 0);
        e.mem.set_u32(new_pairs + 8, 0x8000_0000);
        let broad_phase = e.get(this, HkpWorld::m_broadPhase).addr();
        e.vcall(broad_phase, 0x54, &args![region, offset, new_pairs]);
        let input = e.get(this, HkpWorld::m_collisionInput).addr();
        let broad_phase = e.get(this, HkpWorld::m_broadPhase).addr();
        e.vcall(broad_phase, 0x58, &args![input + 0x20, input + 0x30]);
        for field in [t + 0x310, t + 0x320] {
            let moved = addps(load_vec(e, field), load_vec(e, offset));
            store_vec(e, field, moved);
        }
        let base = filter_base(e, t);
        begin_critical_section(e, this);
        let data = e.mem.u32(new_pairs);
        let size = e.mem.u32(new_pairs + 4);
        let dispatcher = e.get(this, HkpWorld::m_broadPhaseDispatcher);
        e.call(ADD_PAIRS, &args![dispatcher, data, size, base]);
        let border = e.get(this, HkpWorld::m_broadPhaseBorder).addr();
        if border != 0 {
            for slot in 0..6 {
                let object = e.mem.u32(border + 0x18 + 4 * slot);
                let kind = e.vcall(object, 0x14, &args![]).u32();
                if kind == 0 {
                    let first = addps(load_vec(e, object + 0xb0), load_vec(e, offset));
                    store_vec(e, object + 0xb0, first);
                    let second = addps(load_vec(e, offset), load_vec(e, object + 0xc0));
                    store_vec(e, object + 0xc0, second);
                } else if kind.wrapping_sub(1) <= 1 {
                    let moved = addps(load_vec(e, object + 0xe0), load_vec(e, offset));
                    store_vec(e, object + 0xe0, moved);
                }
            }
        }
        if skip_update == 0 {
            recalculate_bounding_volumes(e, this, f);
        }
        end_critical_section(e, this);
        profile_record(e, PROFILE_END);
        array_release(e, new_pairs, 8, 0x10);
    });
}

/// The part of `shiftBroadPhase` that runs unless it is told to skip: gathers the shaped
/// bodies of the fixed island and of every active and inactive island and recalculates their
/// bounding volumes (`00d1a330` on the collision input). `frame` is the caller's frame: +0xC
/// is the size word for the allocator's virtual function 0xC and +0x1C an `hkArray` of the
/// gathered bodies.
fn recalculate_bounding_volumes(e: &mut Engine, this: Ptr<HkpWorld>, frame: u32) {
    let t = this.addr();
    let fixed = e.get(this, HkpWorld::m_fixedIsland).addr();
    let total = e
        .mem
        .i32(t + 0x38)
        .wrapping_add(e.mem.i32(t + 0x2c))
        .wrapping_add(1);
    let bodies = frame + 0x1c;
    e.mem.set_u32(bodies, 0);
    e.mem.set_u32(bodies + 4, 0);
    e.mem.set_u32(bodies + 8, 0x8000_0000);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let (buffer, count, capacity) = if total == 0 {
        (0u32, total, 0x8000_0000u32)
    } else {
        let size_word = frame + 0xc;
        e.mem.set_u32(size_word, (total as u32).wrapping_mul(4));
        let buffer = e.vcall(allocator, 0xc, &args![size_word]).u32();
        let granted = e.mem.i32(size_word);
        let count = granted.wrapping_add((granted >> 31) & 3) >> 2;
        let capacity = if count == 0 {
            0x8000_0000
        } else {
            count as u32
        };
        (buffer, count, capacity)
    };
    let has_fixed = u32::from(fixed != 0);
    if has_fixed != 0 {
        e.mem.set_u32(buffer, fixed);
    }
    let mut cursor = buffer.wrapping_add(has_fixed * 4);
    let source = e.mem.u32(t + 0x28);
    for index in 0..e.mem.i32(t + 0x2c).max(0) as u32 {
        let island = e.mem.u32(source + 4 * index);
        e.mem.set_u32(cursor, island);
        cursor = cursor.wrapping_add(4);
    }
    let active = e.mem.u32(t + 0x2c);
    cursor = buffer.wrapping_add(active.wrapping_add(has_fixed).wrapping_mul(4));
    let source = e.mem.u32(t + 0x34);
    for index in 0..e.mem.i32(t + 0x38).max(0) as u32 {
        let island = e.mem.u32(source + 4 * index);
        e.mem.set_u32(cursor, island);
        cursor = cursor.wrapping_add(4);
    }
    let mut island_index = 0i32;
    while island_index < count {
        let island = array_entry(e, buffer, island_index);
        let mut body_index = 0i32;
        while body_index < e.mem.i32(island + 0x4c) {
            let entity = array_entry(e, e.mem.u32(island + 0x48), body_index);
            if e.mem.u32(entity + 0x10) != 0 {
                array_push_word(e, bodies, entity);
            }
            body_index += 1;
        }
        island_index += 1;
    }
    let input = e.get(this, HkpWorld::m_collisionInput);
    let data = e.mem.u32(bodies);
    let size = e.mem.u32(bodies + 4);
    e.call(ENTITY_BATCH_RECALC_AABB, &args![input, data, size]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    if (capacity as i32) >= 0 {
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![buffer, bytes]);
    }
    array_release(e, bodies, 4, 0x10);
}

/// Hands one candidate of the narrow phase loops to the collision dispatcher's function table
/// (`query` points to the dispatcher; the table of 0x14-byte entries starts at `table_offset`
/// inside it and is indexed by the byte at `(first_type + 0xD) * 0x20 + second shape's type`
/// in the dispatcher). Skips the candidate when it is the collidable itself, when the world's
/// collision filter (+0x7C, virtual function 4 of the object 8 bytes in) refuses the pair or
/// when it has no shape. Returns whether the function was called, with `first`, `second`,
/// `query` and `collector`.
fn dispatch_pair(
    e: &mut Engine,
    world: u32,
    pair: (u32, u32),
    first_type: u32,
    query: u32,
    table: (u32, u32),
) -> bool {
    let (first, second) = pair;
    if first == second {
        return false;
    }
    let filter = e.mem.u32(world + 0x7c);
    if !filter_accepts(e, filter, first, second) {
        return false;
    }
    let shape = e.mem.u32(second);
    if shape == 0 {
        return false;
    }
    let (table_offset, collector) = table;
    let base = e.mem.u32(query);
    let index = first_type
        .wrapping_add(0xd)
        .wrapping_shl(5)
        .wrapping_add(e.mem.u32(shape + 0xc));
    let entry = u32::from(e.mem.u8(base.wrapping_add(index)));
    let function = e.mem.u32(
        base.wrapping_add(entry.wrapping_mul(0x14))
            .wrapping_add(table_offset),
    );
    e.call(function, &args![first, second, query, collector]);
    true
}

/// The shape query tolerance `getPenetrations` and `getClosestPoints` pass: the `float` at
/// +8 of `query` minus half (the `double` at `01011588`) the float at +8 of the world's
/// collision input, rounded to a `float`.
fn query_tolerance(e: &Engine, world: u32, query: u32) -> f32 {
    let wanted = f64::from(x87_quiet(e.mem.f32(query + 8)));
    let input = e.mem.u32(world + 0x78);
    let margin = f64::from(x87_quiet(e.mem.f32(input + 8)));
    (wanted - margin * e.mem.f64(HALF)) as f32
}

// Translated from 00c96df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::getPenetrations` (Xbox PDB). Three stack words (`RET 0xC`): `collidable`,
/// `query` (the dispatcher whose first word points to the table; its float at +8 is the
/// distance) and `collector` (a byte at +4 stops the search). Writes a profiling record pair,
/// asks the collidable's shape (virtual function 0x1C, with the collidable's transform, the
/// tolerance clamped at the `double` at `01012060` from below and a local box) for its
/// bounds, asks the broad phase (0x44) for the overlapping pairs (a local array of 0x80 pairs),
/// then walks the second members (the handle plus the signed byte at +5): for each one other
/// than the collidable that the world's collision filter accepts and that has a shape the
/// dispatcher's function at `table + 0x9A4` runs. The loop ends early once the collector's
/// byte at +4 is set. Writes the closing records and frees the pair array.
pub fn hkp_world_get_penetrations(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    collidable: u32,
    query: u32,
    collector: u32,
) {
    let t = this.addr();
    profile_record_pair(e, PROFILE_GET_PENETRATIONS, PROFILE_BROADPHASE_SECOND);
    let distance = query_tolerance(e, t, query);
    let tolerance = if f64::from(distance) < e.mem.f64(ZERO_DOUBLE) {
        0.0
    } else {
        distance
    };
    e.with_stack(0x440, |e, frame| {
        let f = frame.addr();
        let (bounds, pairs) = (f + 0x14, f + 0x34);
        let shape = e.mem.u32(collidable);
        let transform = e.mem.u32(collidable + 8);
        e.vcall(
            shape,
            0x1c,
            &args![transform, x87_quiet(tolerance).to_bits(), bounds],
        );
        e.mem.set_u32(pairs, f + 0x40);
        e.mem.set_u32(pairs + 4, 0);
        e.mem.set_u32(pairs + 8, 0x8000_0080);
        let broad_phase = e.get(this, HkpWorld::m_broadPhase).addr();
        e.vcall(broad_phase, 0x44, &args![bounds, pairs]);
        let first_type = e.mem.u32(e.mem.u32(collidable) + 0xc);
        profile_record(e, PROFILE_GET_PENETRATIONS_NARROW);
        let mut index = e.mem.i32(pairs + 4).wrapping_sub(1);
        let mut cursor = e.mem.u32(pairs).wrapping_add(4);
        while index >= 0 {
            let handle = e.mem.u32(cursor);
            let other = offset_by_byte(e, handle, handle + 5);
            if dispatch_pair(
                e,
                t,
                (collidable, other),
                first_type,
                query,
                (0x9a4, collector),
            ) && e.mem.u8(collector + 4) != 0
            {
                break;
            }
            cursor = cursor.wrapping_add(8);
            index = index.wrapping_sub(1);
        }
        profile_record(e, PROFILE_SECTION_END);
        array_release(e, pairs, 8, 0x10);
    });
}

// Translated from 00c976a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Two stack words (`RET 8`, `ECX` not read): `action` and `entity`. Takes the
/// action out of the entity's action list (+0x208, 16-bit size +0x20C: the last entry moves
/// into its place; an action that is not in the list is index -1), sets bit 0 and clears bit
/// 1 of the byte at +0x25 of the entity's island (+0xCC), asks the action for its entities
/// (virtual function 0x10, into a local array) and picks the first entity other than this one
/// whose island (+0xCC) has its word at +0x20 not equal to 0xFFFF (the island of the last
/// one examined when there is none, 0 without any other). When that island differs from the
/// action's (+0xC) the action leaves its island (`00d07130`) and joins the picked one
/// (`00d07470`).
pub fn fn_00c976a0(e: &mut Engine, action: u32, entity: u32) {
    let count = e.mem.u16(entity + 0x20c);
    let data = e.mem.u32(entity + 0x208);
    let slot = (0..u32::from(count))
        .find(|&candidate| e.mem.u32(data + 4 * candidate) == action)
        .map_or(-1, |candidate| candidate as i32);
    let remaining = count.wrapping_add(0xffff);
    e.mem.set_u16(entity + 0x20c, remaining);
    let data = e.mem.u32(entity + 0x208);
    let last = e.mem.u32(data.wrapping_add(u32::from(remaining) * 4));
    e.mem
        .set_u32(data.wrapping_add((slot as u32).wrapping_mul(4)), last);
    let island = e.mem.u32(entity + 0xcc);
    let flags = e.mem.u8(island + 0x25);
    e.mem.set_u8(island + 0x25, (flags & 0xfd) | 1);
    e.with_stack(0x20, |e, frame| {
        let entities = frame.addr();
        e.mem.set_u32(entities, entities + 0xc);
        e.mem.set_u32(entities + 4, 0);
        e.mem.set_u32(entities + 8, 0x8000_0004);
        e.vcall(action, 0x10, &args![entities]);
        let size = e.mem.i32(entities + 4);
        let mut picked = 0u32;
        let mut index = 0i32;
        while index < size {
            let other = array_entry(e, e.mem.u32(entities), index);
            if other != entity {
                picked = e.mem.u32(other + 0xcc);
                if e.mem.u16(picked + 0x20) != 0xffff {
                    break;
                }
            }
            index += 1;
        }
        let action_island = e.mem.u32(action + 0xc);
        if picked != action_island {
            e.call(ISLAND_REMOVE_ACTION, &args![action_island, action]);
            e.call(ISLAND_ADD_ACTION, &args![picked, action]);
        }
        array_release(e, entities, 4, 0x10);
    });
}

// Translated from 00c977c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::setCollisionFilter` (Xbox PDB). Four stack words (`RET 0x10`): `filter`,
/// `update` (a byte) and the two words `first` and `second`. Without a filter a 0x30-byte
/// default one is allocated through the memory router (word +4 = 0x30), constructed
/// (`00cbfba0`), given its five vtables (`010c646c` at +0, `010c6460` at +8, `010c644c` at
/// +0xC, `010c6440` at +0x10, `010c6434` at +0x14) and the word +0x20 = 1; a given filter gets
/// a reference. The reference on the old filter (+0x7C) is dropped, the new one is stored there
/// and in the collision input (+0x78, word +0xC), and when `update` is non-zero the filter is
/// applied to the world (`updateCollisionFilterOnWorld`, `00c96ff0`, with `first` and
/// `second`).
pub fn hkp_world_set_collision_filter(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    filter: u32,
    update: u8,
    first: u32,
    second: u32,
) {
    let filter = if filter == 0 {
        let block = router_alloc(e, 0x30);
        e.mem.set_u16(block + 4, 0x30);
        e.call(COLLISION_FILTER_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, NULL_FILTER_VTABLE_0);
        e.mem.set_u32(block + 8, NULL_FILTER_VTABLE_8);
        e.mem.set_u32(block + 0xc, NULL_FILTER_VTABLE_C);
        e.mem.set_u32(block + 0x10, NULL_FILTER_VTABLE_10);
        e.mem.set_u32(block + 0x14, NULL_FILTER_VTABLE_14);
        e.mem.set_u32(block + 0x20, 1);
        block
    } else {
        e.call(ADD_REFERENCE, &args![filter]);
        filter
    };
    let old = e.get(this, HkpWorld::m_collisionFilter);
    e.call(REMOVE_REFERENCE, &args![old]);
    let input = e.get(this, HkpWorld::m_collisionInput);
    e.set(this, HkpWorld::m_collisionFilter, Ptr::new(filter));
    e.mem.set_u32(input.addr() + 0xc, filter);
    if update != 0 {
        e.call(
            UPDATE_COLLISION_FILTER_ON_WORLD,
            &args![this, first, second],
        );
    }
}

/// The candidate loop of the tree paths of `getClosestPoints`: every collidable pointer of
/// the array at `array` (data +0, size +4) goes through [`dispatch_pair`] with the function
/// table at +0x9A8.
fn closest_points_over_array(
    e: &mut Engine,
    world: u32,
    array: u32,
    first: (u32, u32),
    query: (u32, u32),
) {
    let (collidable, first_type) = first;
    let (query, collector) = query;
    let mut index = 0i32;
    while index < e.mem.i32(array + 4) {
        let other = array_entry(e, e.mem.u32(array), index);
        dispatch_pair(
            e,
            world,
            (collidable, other),
            first_type,
            query,
            (0x9a8, collector),
        );
        index += 1;
    }
}

// Translated from 00c97850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::getClosestPoints` (Xbox PDB). Three stack words (`RET 0xC`): `collidable`,
/// `query` (the dispatcher whose first word points to the table; its float at +8 is the
/// distance) and `collector`. Asks the collidable's shape (virtual function 0x1C, with its
/// transform, the tolerance and a local box) for its bounds. Without a kd-tree manager
/// (+0x5C) in use (its virtual function 0x10 non-zero) the broad phase (0x44) lists the
/// overlapping pairs and the second members (the handle plus the signed byte at +5) go
/// through the dispatcher's table at +0x9A8, each between profiling records. With one, its
/// AABB tree (+0xC, `00d1d4a0`) and kd tree (+0x10, `00d1b850`), when present, list
/// collidable pointers directly; the kd tree's array is freed through the router's second
/// allocator (+0xC). Ends with the closing record.
pub fn hkp_world_get_closest_points(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    collidable: u32,
    query: u32,
    collector: u32,
) {
    let t = this.addr();
    let tolerance = query_tolerance(e, t, query);
    e.with_stack(0x464, |e, frame| {
        let f = frame.addr();
        let bounds = f + 0x24;
        let shape = e.mem.u32(collidable);
        let transform = e.mem.u32(collidable + 8);
        e.vcall(
            shape,
            0x1c,
            &args![transform, x87_quiet(tolerance).to_bits(), bounds],
        );
        if kd_tree_in_use(e, this) {
            profile_record_pair(e, PROFILE_CLOSEST_TREES, PROFILE_CLOSEST_TREES_BROAD_PHASE);
            let manager = e.get(this, HkpWorld::m_kdTreeManager).addr();
            let aabb_tree = e.mem.u32(manager + 0xc);
            let kd_tree = e.mem.u32(manager + 0x10);
            if aabb_tree != 0 {
                let array = f + 0x54;
                e.mem.set_u32(array, f + 0x60);
                e.mem.set_u32(array + 4, 0);
                e.mem.set_u32(array + 8, 0x8000_0080);
                e.call(AABB_TREE_QUERY, &args![aabb_tree, bounds, array]);
                profile_record(e, PROFILE_CLOSEST_AABB_TREE_NARROW);
                let first_type = e.mem.u32(e.mem.u32(collidable) + 0xc);
                closest_points_over_array(
                    e,
                    t,
                    array,
                    (collidable, first_type),
                    (query, collector),
                );
                array_release(e, array, 4, 0x10);
            }
            if kd_tree != 0 {
                profile_record(e, PROFILE_CLOSEST_KD_TREE_BROAD_PHASE);
                let array = f + 0x14;
                e.mem.set_u32(array, 0);
                e.mem.set_u32(array + 4, 0);
                e.mem.set_u32(array + 8, 0x8000_0000);
                e.call(KD_TREE_QUERY, &args![kd_tree, bounds, array]);
                profile_record(e, PROFILE_CLOSEST_KD_TREE_NARROW);
                let first_type = e.mem.u32(e.mem.u32(collidable) + 0xc);
                closest_points_over_array(
                    e,
                    t,
                    array,
                    (collidable, first_type),
                    (query, collector),
                );
                array_release(e, array, 4, 0xc);
            }
            profile_record(e, PROFILE_SECTION_END);
        } else {
            profile_record_pair(
                e,
                PROFILE_CLOSEST_BROAD_PHASE,
                PROFILE_CLOSEST_BROAD_PHASE_SECOND,
            );
            let pairs = f + 0x54;
            e.mem.set_u32(pairs, f + 0x60);
            e.mem.set_u32(pairs + 4, 0);
            e.mem.set_u32(pairs + 8, 0x8000_0080);
            let broad_phase = e.get(this, HkpWorld::m_broadPhase).addr();
            e.vcall(broad_phase, 0x44, &args![bounds, pairs]);
            let first_type = e.mem.u32(e.mem.u32(collidable) + 0xc);
            profile_record(e, PROFILE_CLOSEST_NARROW);
            let mut index = e.mem.i32(pairs + 4).wrapping_sub(1);
            let mut cursor = e.mem.u32(pairs).wrapping_add(4);
            while index >= 0 {
                let handle = e.mem.u32(cursor);
                let other = offset_by_byte(e, handle, handle + 5);
                dispatch_pair(
                    e,
                    t,
                    (collidable, other),
                    first_type,
                    query,
                    (0x9a8, collector),
                );
                cursor = cursor.wrapping_add(8);
                index = index.wrapping_sub(1);
            }
            profile_record(e, PROFILE_SECTION_END);
            array_release(e, pairs, 8, 0x10);
        }
    });
}

// Translated from 00c97e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::createAndAddConstraintInstance` (Xbox PDB). Three stack words (`RET 0xC`):
/// `entity_a`, `entity_b` and `data`. Allocates 0x38 bytes through the memory router (word
/// +4 = 0x38), constructs an `hkpConstraintInstance` (`00cf3ea0`: the three words and 1),
/// copies the word at +8 of `data` to +0x2C of the instance, adds it with `addConstraint`
/// (`00c97d30`) and returns it.
pub fn hkp_world_create_and_add_constraint_instance(
    e: &mut Engine,
    this: Ptr<HkpWorld>,
    entity_a: u32,
    entity_b: u32,
    data: u32,
) -> Ptr {
    let block = router_alloc(e, 0x38);
    e.mem.set_u16(block + 4, 0x38);
    let instance = e
        .call(
            CONSTRAINT_INSTANCE_CONSTRUCT,
            &args![block, entity_a, entity_b, data, 1u32],
        )
        .u32();
    let word = e.mem.u32(data + 8);
    e.mem.set_u32(instance + 0x2c, word);
    e.call(ADD_CONSTRAINT, &args![this, instance]);
    Ptr::new(instance)
}

// Translated from 00c97f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::addPhysicsSystem` (Xbox PDB). One stack word (`RET 4`): `system`. Adds the
/// entities (array +8, size +0xC) with `addEntityBatch` (`00c94bd0`, flag = the byte at +0x40
/// non-zero) when none is null, else one by one with `addEntity` (`00c914d0`) skipping the
/// null ones; the phantoms (array +0x2C, size +0x30) likewise (`00c954e0` /
/// `addPhantom` `00c936a0`); every non-null action (array +0x20, size +0x24) with `addAction`
/// (`00c97370`) and every non-null constraint (array +0x14, size +0x18) with a non-zero word
/// at +0x14 or +0x18 with `addConstraint` (`00c97d30`).
pub fn hkp_world_add_physics_system(e: &mut Engine, this: Ptr<HkpWorld>, system: u32) {
    let count = e.mem.i32(system + 0xc);
    if count > 0 {
        let data = e.mem.u32(system + 8);
        let first_null = (0..count).find(|&index| e.mem.u32(data + 4 * index as u32) == 0);
        if first_null.map_or(true, |index| index == -1) {
            let data = e.mem.u32(system + 8);
            let mode = i32::from(e.mem.u8(system + 0x40) != 0);
            e.call(ADD_ENTITY_BATCH, &args![this, data, count, mode]);
        } else {
            let mut index = 0i32;
            while index < e.mem.i32(system + 0xc) {
                let entity = array_entry(e, e.mem.u32(system + 8), index);
                if entity != 0 {
                    let mode = u32::from(e.mem.u8(system + 0x40) != 0);
                    e.call(ADD_ENTITY, &args![this, entity, mode]);
                }
                index += 1;
            }
        }
    }
    let count = e.mem.i32(system + 0x30);
    if count > 0 {
        let data = e.mem.u32(system + 0x2c);
        let first_null = (0..count).find(|&index| e.mem.u32(data + 4 * index as u32) == 0);
        if first_null.map_or(true, |index| index == -1) {
            let data = e.mem.u32(system + 0x2c);
            e.call(ADD_PHANTOM_BATCH, &args![this, data, count]);
        } else {
            let mut index = 0i32;
            while index < e.mem.i32(system + 0x30) {
                let phantom = array_entry(e, e.mem.u32(system + 0x2c), index);
                if phantom != 0 {
                    e.call(ADD_PHANTOM, &args![this, phantom]);
                }
                index += 1;
            }
        }
    }
    let mut index = 0i32;
    while index < e.mem.i32(system + 0x24) {
        let action = array_entry(e, e.mem.u32(system + 0x20), index);
        if action != 0 {
            e.call(ADD_ACTION, &args![this, action]);
        }
        index += 1;
    }
    index = 0;
    while index < e.mem.i32(system + 0x18) {
        let constraint = array_entry(e, e.mem.u32(system + 0x14), index);
        if constraint != 0
            && (e.mem.u32(constraint + 0x14) != 0 || e.mem.u32(constraint + 0x18) != 0)
        {
            e.call(ADD_CONSTRAINT, &args![this, constraint]);
        }
        index += 1;
    }
}

/// Frees an object of the size in its 16-bit word at +4 through the memory router when bit 0
/// of the deleting destructor's flags is set (the tail every `_scalar_deleting_destructor_`
/// here shares).
fn free_if_flagged(e: &mut Engine, object: Ptr, flags: u32) {
    if flags & 1 != 0 {
        let size = u32::from(e.mem.u16(object.addr() + 4));
        router_free(e, object.addr(), size);
    }
}

// Translated from 00c98260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpNullContactMgrFactory::_scalar_deleting_destructor_` (Xbox PDB). One stack word
/// (`RET 4`): `flags`. Stores the base vtable `0102e388` at +0 and +8; when bit 0 of `flags`
/// is set the object is freed through the router (its size is the 16-bit word at +4).
/// Returns `this`.
pub fn hkp_null_contact_mgr_factory_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr() + 8, REFERENCED_OBJECT_VTABLE);
    e.mem.set_u32(this.addr(), REFERENCED_OBJECT_VTABLE);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 00c982a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Five further stack words after `out` (`RET 0x18`; `ECX` is not read): stores the
/// byte 1 at `out` (the null collision filter accepts every pair).
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00cfae7e).
#[allow(clippy::too_many_arguments)]
pub fn fn_00c982a0(
    e: &mut Engine,
    _this: Ptr,
    out: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
) -> u32 {
    e.mem.set_u8(out, 1);
    out
}

// Translated from 00c982b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Seven further stack words after `out` (`RET 0x20`; `ECX` is not read): stores the
/// byte 1 at `out`.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
#[allow(clippy::too_many_arguments)]
pub fn fn_00c982b0(
    e: &mut Engine,
    _this: Ptr,
    out: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
    _unused_7: u32,
    _unused_8: u32,
    _unused_9: u32,
) -> u32 {
    e.mem.set_u8(out, 1);
    out
}

// Translated from 00c982c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Four further stack words after `out` (`RET 0x14`; `ECX` is not read): stores the
/// byte 1 at `out`.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn fn_00c982c0(
    e: &mut Engine,
    _this: Ptr,
    out: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    e.mem.set_u8(out, 1);
    out
}

// Translated from 00c982d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. Two further stack words after `out` (`RET 0xC`; `ECX` is not read): stores the
/// byte 1 at `out`.
/// Answers `out`: the exe leaves it in `EAX` at every `RET` (the hkBool result returned
/// through a pointer), and virtual call sites test the flag through `EAX` (e.g. 00c90f6a).
pub fn fn_00c982d0(e: &mut Engine, _this: Ptr, out: u32, _unused_3: u32, _unused_4: u32) -> u32 {
    e.mem.set_u8(out, 1);
    out
}

// Translated from 00c982e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (the destructor body `00cfa860` plus the deleting tail). One stack word (`RET 4`):
/// `flags`. Runs `00cfa860` on the object; when bit 0 of `flags` is set it is freed through the
/// router (size: the 16-bit word at +4). Returns `this`.
pub fn fn_00c982e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(RAY_CASTER_BASE_DESTRUCT, &args![this]);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 00c98350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleWorldRayCaster::_scalar_deleting_destructor_` (Xbox PDB). One stack word (`RET
/// 4`): `flags`. Stores the vtable `010c3bb8`; when bit 0 of `flags` is set it frees the object
/// with `00401030` (cdecl). Returns `this`.
pub fn hkp_simple_world_ray_caster_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), RAY_CASTER_BASE_VTABLE);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 00c98370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed. One stack word (`RET 4`): `flags`. Stores the vtable `010c63bc` at +8 and the base
/// vtable `0102e388` at +0; when bit 0 of `flags` is set the object is freed through the router
/// (size: the 16-bit word at +4). Returns `this`.
pub fn fn_00c98370(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr() + 8, FILTER_BASE_VTABLE);
    e.mem.set_u32(this.addr(), REFERENCED_OBJECT_VTABLE);
    free_if_flagged(e, this, flags);
    this
}

// Translated from 00c983b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpWorld::_scalar_deleting_destructor_` (Xbox PDB). One stack word (`RET 4`): `flags`.
/// Runs the destructor (`00c93e50`); when bit 0 of `flags` is set the world is freed through
/// the router (size: the 16-bit word at +4). Returns `this`.
pub fn hkp_world_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(WORLD_DESTRUCTOR, &args![this]);
    free_if_flagged(e, this, flags);
    this
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c9_0e40,
            hkp_contact_mgr_factory_calc_content_statistics(Ptr, Ptr, u32)
        ),
        entry!(0x00c9_0e60, fn_00c90e60(Ptr, f32, f32)),
        entry!(0x00c9_0ed0, fn_00c90ed0(Ptr, u32, u32, i32)),
        entry!(0x00c9_0f30, hkp_multithread_config_hkp_multithread_config(Ptr) -> Ptr),
        entry!(0x00c9_1000, fn_00c91000(Ptr<HkpWorld>, Ptr)),
        entry!(
            0x00c9_1020,
            hkp_world_get_current_time(Ptr<HkpWorld>) -> f32
        ),
        entry!(
            0x00c9_1030,
            hkp_world_get_current_psi_time(Ptr<HkpWorld>) -> f32
        ),
        entry!(0x00c9_1040, fn_00c91040(Ptr<HkpWorld>, f32)),
        entry!(0x00c9_1060, fn_00c91060(Ptr<HkpWorld>) -> u32),
        entry!(0x00c9_10a0, fn_00c910a0(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_10b0, hkp_world_check_determinism(Ptr<HkpWorld>)),
        entry!(0x00c9_10d0, fn_00c910d0(Ptr<HkpWorld>, Ptr)),
        entry!(0x00c9_1110, fn_00c91110(Ptr<HkpWorld>) -> Ptr),
        entry!(0x00c9_1120, fn_00c91120(Ptr<HkpWorld>)),
        entry!(0x00c9_1140, fn_00c91140(Ptr<HkpWorld>)),
        entry!(0x00c9_1160, fn_00c91160(Ptr<HkpWorld>, Ptr)),
        entry!(0x00c9_1180, fn_00c91180(Ptr<HkpWorld>)),
        entry!(0x00c9_1190, fn_00c91190(Ptr<HkpWorld>)),
        entry!(0x00c9_11a0, fn_00c911a0(Ptr<HkpWorld>)),
        entry!(0x00c9_11b0, hkp_world_unlock_read_only(Ptr<HkpWorld>)),
        entry!(
            0x00c9_11d0,
            hkp_world_lock_island_for_constraint_update(Ptr<HkpWorld>, u32)
        ),
        entry!(0x00c9_11f0, fn_00c911f0(Ptr<HkpWorld>, u32)),
        entry!(
            0x00c9_1210,
            hkp_world_unlock_island_for_constraint_update(Ptr<HkpWorld>, u32)
        ),
        entry!(0x00c9_1230, fn_00c91230(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_1250, fn_00c91250(Ptr<HkpWorld>)),
        entry!(0x00c9_1270, fn_00c91270(Ptr<HkpWorld>, u32, u32)),
        entry!(
            0x00c9_13b0,
            hkp_world_reenable_collision_between_entity_pair(Ptr<HkpWorld>, Ptr, Ptr)
        ),
        entry!(0x00c9_1470, fn_00c91470(Ptr<HkpWorld>, u32, u32, u32)),
        entry!(
            0x00c9_14d0,
            hkp_world_add_entity(Ptr<HkpWorld>, Ptr<HkpEntity>, u32) -> Ptr<HkpEntity>
        ),
        entry!(
            0x00c9_1620,
            hkp_world_remove_entity(Ptr<HkpWorld>, Ptr, Ptr<HkpEntity>) -> Ptr
        ),
        entry!(0x00c9_1700, fn_00c91700(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_1750, fn_00c91750(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_17a0, fn_00c917a0(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_17f0, fn_00c917f0(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_1840, fn_00c91840(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_18e0, fn_00c918e0(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_1a20, fn_00c91a20(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_1a70, fn_00c91a70(Ptr<HkpWorld>, u32)),
        entry!(
            0x00c9_1b10,
            hkp_world_step_delta_time(Ptr<HkpWorld>, f32) -> u32
        ),
        entry!(0x00c9_1b70, fn_00c91b70(Ptr<HkpWorld>, f32) -> u32),
        entry!(0x00c9_1ba0, fn_00c91ba0(Ptr<HkpWorld>) -> u32),
        entry!(0x00c9_1bd0, fn_00c91bd0(Ptr<HkpWorld>) -> u32),
        entry!(
            0x00c9_1c00,
            hkp_world_finish_mt_step(Ptr<HkpWorld>, u32, u32) -> u32
        ),
        entry!(0x00c9_1c50, fn_00c91c50(Ptr<HkpWorld>, Ptr)),
        entry!(0x00c9_1f70, hkp_world_cast_ray(Ptr<HkpWorld>, u32, u32)),
        entry!(0x00c9_2040, hkp_world_cast_ray_ov2(Ptr<HkpWorld>, u32, u32)),
        entry!(
            0x00c9_2110,
            fn_00c92110(Ptr<HkpWorld>, u32, i32, u32, u32, u32, i32)
        ),
        entry!(
            0x00c9_2290,
            fn_00c92290(Ptr<HkpWorld>, u32, i32, u32, u32, u32, i32)
        ),
        entry!(
            0x00c9_2430,
            hkp_world_linear_cast(Ptr<HkpWorld>, u32, u32, u32, u32)
        ),
        entry!(
            0x00c9_2540,
            fn_00c92540(Ptr<HkpWorld>, u32, i32, u32, u32, u32, i32)
        ),
        entry!(0x00c9_26a0, fn_00c926a0(Ptr, Ptr<HkpWorld>, Ptr) -> Ptr),
        entry!(0x00c9_2760, fn_00c92760(Ptr, Ptr<HkpWorld>, Ptr) -> Ptr),
        entry!(0x00c9_27f0, fn_00c927f0(Ptr, Ptr)),
        entry!(0x00c9_28c0, fn_00c928c0(Ptr<HkpWorld>, Ptr)),
        entry!(0x00c9_29e0, fn_00c929e0(Ptr<HkpWorld>) -> Ptr),
        entry!(0x00c9_2c70, hkp_world_remove_phantom(Ptr<HkpWorld>, u32)),
        entry!(
            0x00c9_36a0,
            hkp_world_add_phantom(Ptr<HkpWorld>, u32) -> u32
        ),
        entry!(0x00c9_3c30, hkp_world_activate_region(Ptr<HkpWorld>, u32)),
        entry!(
            0x00c9_44a0,
            hkp_world_update_collision_filter_on_phantom(Ptr<HkpWorld>, u32, u32)
        ),
        entry!(
            0x00c9_46e0,
            hkp_world_update_collision_filter_on_entity(Ptr<HkpWorld>, u32, i32, u32)
        ),
        entry!(
            0x00c9_4bd0,
            hkp_world_add_entity_batch(Ptr<HkpWorld>, u32, i32, i32)
        ),
        entry!(
            0x00c9_51b0,
            hkp_world_remove_entity_batch(Ptr<HkpWorld>, u32, i32)
        ),
        entry!(
            0x00c9_54e0,
            hkp_world_add_phantom_batch(Ptr<HkpWorld>, u32, i32)
        ),
        entry!(
            0x00c9_5800,
            hkp_world_remove_phantom_batch(Ptr<HkpWorld>, u32, i32)
        ),
        entry!(
            0x00c9_5a80,
            hkp_world_hkp_world(Ptr<HkpWorld>, Ptr, u32) -> Ptr<HkpWorld>
        ),
        entry!(
            0x00c9_6ff0,
            hkp_world_update_collision_filter_on_world(Ptr<HkpWorld>, i32, u32)
        ),
        entry!(0x00c9_7370, hkp_world_add_action(Ptr<HkpWorld>, u32) -> u32),
        entry!(
            0x00c9_7510,
            hkp_world_remove_action_immediately(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00c9_7d30,
            hkp_world_add_constraint(Ptr<HkpWorld>, u32) -> u32
        ),
        entry!(
            0x00c9_7e80,
            hkp_world_remove_constraint(Ptr<HkpWorld>, Ptr, u32) -> Ptr
        ),
        entry!(0x00c9_7f50, hkp_world_remove_action(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_8310, fn_00c98310(Ptr, u32) -> u32),
        entry!(0x00c9_8320, fn_00c98320(Ptr, u32) -> u32),
        entry!(0x00c9_8330, fn_00c98330(Ptr, u32) -> u32),
        entry!(0x00c9_8340, fn_00c98340(Ptr, u32) -> u32),
        entry!(0x00c9_2a30, fn_00c92a30(Ptr<HkpWorld>, u32, u32) -> u32),
        entry!(0x00c9_2a80, fn_00c92a80(Ptr<HkpWorld>, u32, u32)),
        entry!(0x00c9_2b00, fn_00c92b00(Ptr<HkpWorld>)),
        entry!(
            0x00c9_2b50,
            hkp_world_get_mem_usage_for_integration(Ptr<HkpWorld>, u32)
        ),
        entry!(0x00c9_2bd0, fn_00c92bd0(Ptr<HkpWorld>, u32, u32)),
        entry!(
            0x00c9_2d70,
            hkp_world_init_mt_step(Ptr<HkpWorld>, u32, f32) -> u32
        ),
        entry!(
            0x00c9_2db0,
            hkp_world_calc_content_statistics(Ptr<HkpWorld>, u32, u32)
        ),
        entry!(
            0x00c9_3570,
            hkp_world_get_memory_watch_dog(Ptr<HkpWorld>) -> Ptr
        ),
        entry!(0x00c9_3580, hkp_world_remove_all(Ptr<HkpWorld>)),
        entry!(0x00c9_37a0, fn_00c937a0(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_37e0, fn_00c937e0(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_3820, fn_00c93820(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_3860, fn_00c93860(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_38a0, fn_00c938a0(Ptr<HkpWorld>, u32)),
        entry!(
            0x00c9_3920,
            hkp_world_add_world_post_simulation_listener(Ptr<HkpWorld>, u32)
        ),
        entry!(0x00c9_3a20, fn_00c93a20(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_3a60, fn_00c93a60(Ptr<HkpWorld>, u32)),
        entry!(
            0x00c9_3ae0,
            hkp_world_step_multithreaded(Ptr<HkpWorld>, u32, u32, f32) -> u32
        ),
        entry!(0x00c9_3b30, fn_00c93b30(Ptr<HkpWorld>, u32)),
        entry!(0x00c9_3d10, fn_00c93d10(Ptr<HkpWorld>, u32) -> Ptr),
        entry!(0x00c9_3e50, hkp_world_destructor(Ptr<HkpWorld>)),
        entry!(
            0x00c9_6a20,
            hkp_world_shift_broad_phase(Ptr<HkpWorld>, u32, u32, u32)
        ),
        entry!(
            0x00c9_6df0,
            hkp_world_get_penetrations(Ptr<HkpWorld>, u32, u32, u32)
        ),
        entry!(0x00c9_76a0, fn_00c976a0(u32, u32)),
        entry!(
            0x00c9_77c0,
            hkp_world_set_collision_filter(Ptr<HkpWorld>, u32, u8, u32, u32)
        ),
        entry!(
            0x00c9_7850,
            hkp_world_get_closest_points(Ptr<HkpWorld>, u32, u32, u32)
        ),
        entry!(
            0x00c9_7e30,
            hkp_world_create_and_add_constraint_instance(Ptr<HkpWorld>, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x00c9_7f80,
            hkp_world_add_physics_system(Ptr<HkpWorld>, u32)
        ),
        entry!(
            0x00c9_8260,
            hkp_null_contact_mgr_factory_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c9_82a0, fn_00c982a0(Ptr, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(
            0x00c9_82b0,
            fn_00c982b0(Ptr, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00c9_82c0, fn_00c982c0(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00c9_82d0, fn_00c982d0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00c9_82e0, fn_00c982e0(Ptr, u32) -> Ptr),
        entry!(
            0x00c9_8350,
            hkp_simple_world_ray_caster_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c9_8370, fn_00c98370(Ptr, u32) -> Ptr),
        entry!(
            0x00c9_83b0,
            hkp_world_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Addresses for test doubles that stand in for virtual functions.
    const FAKE_1: u32 = 0x0900_0001;
    const FAKE_2: u32 = 0x0900_0002;
    const FAKE_3: u32 = 0x0900_0003;
    const FAKE_4: u32 = 0x0900_0004;
    const FAKE_5: u32 = 0x0900_0005;

    type Records = Rc<RefCell<Vec<Vec<u32>>>>;
    type Operations = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;

    fn logging() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e
    }

    /// The argument words of every logged call to `address`, in order.
    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .map(|log| {
                log.iter()
                    .filter(|(called, _)| *called == address)
                    .map(|(_, words)| words.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// The addresses of the logged calls that are in `wanted`, in order.
    fn order_of(e: &Engine, wanted: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .map(|log| {
                log.iter()
                    .map(|(called, _)| *called)
                    .filter(|called| wanted.contains(called))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Doubles that do nothing and return 0.
    fn silent(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn returning(e: &mut Engine, address: u32, value: u32) {
        e.register_double(address, move |_, _| Ret {
            eax: value,
            ..Ret::default()
        });
    }

    /// A double that stores its argument words.
    fn recording(e: &mut Engine, address: u32) -> Records {
        let records: Records = Rc::new(RefCell::new(vec![]));
        let sink = records.clone();
        e.register_double(address, move |_, words| {
            sink.borrow_mut().push(words.to_vec());
            Ret::default()
        });
        records
    }

    /// An object of `size` bytes whose vtable (also on the heap) holds `(slot, target)`.
    fn object_with_vtable(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for &(slot, target) in slots {
            e.mem.set_u32(table + slot, target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        object
    }

    fn world(e: &mut Engine) -> Ptr<HkpWorld> {
        e.new_object()
    }

    /// A heap array of the words; one spare word sits before its data.
    fn array_of(e: &mut Engine, values: &[u32]) -> u32 {
        let block = e.mem.alloc(8 + 4 * values.len() as u32);
        for (index, value) in values.iter().enumerate() {
            e.mem.set_u32(block + 4 + 4 * index as u32, *value);
        }
        block + 4
    }

    /// The 16 bytes of an operation record as the queue functions build them.
    fn op_bytes(kind: u8, word_4: u32, word_8: u32, byte_a: u8) -> Vec<u8> {
        let mut bytes = vec![0u8; 16];
        bytes[0] = kind;
        bytes[4..8].copy_from_slice(&word_4.to_le_bytes());
        bytes[8..12].copy_from_slice(&word_8.to_le_bytes());
        bytes[10] = byte_a;
        bytes
    }

    /// Records `(queue, record bytes)` of every operation handed to the queue.
    fn operation_recorder(e: &mut Engine) -> Operations {
        let records: Operations = Rc::new(RefCell::new(vec![]));
        let sink = records.clone();
        e.register_double(QUEUE_ENQUEUE, move |e, words| {
            sink.borrow_mut()
                .push((words[0], e.mem.bytes(words[1], 16)));
            Ret::default()
        });
        records
    }

    #[test]
    fn contact_mgr_factory_names_its_class_to_the_collector() {
        let mut e = logging();
        silent(&mut e, &[FAKE_1, FAKE_2]);
        let collector = object_with_vtable(&mut e, 0x10, &[(0x18, FAKE_1), (0x1c, FAKE_2)]);
        e.call(0x00c9_0e40, &args![0u32, collector, 0u32]);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![collector, 0x010c_63a0]]);
        assert_eq!(calls_to(&e, FAKE_2), vec![vec![collector]]);
    }

    #[test]
    fn fn_00c90e60_stores_the_values_and_ratios() {
        let mut e = Engine::new();
        e.map(0x0101_1000, 0x1000);
        e.set_global(HALF, 0.5f64);
        let object = e.mem.alloc(0x60);
        e.call(0x00c9_0e60, &args![object, 4.0f32, 2.0f32]);
        assert_eq!(e.mem.f32(object + 4), 4.0);
        assert_eq!(e.mem.f32(object + 0xc), 2.0);
        assert_eq!(e.mem.f32(object + 8), 2.0);
        assert_eq!(e.mem.f32(object + 0x50), 0.5);
        assert_eq!(e.mem.f32(object + 0x58), 1.0);
        assert_eq!(e.mem.f32(object + 0x54), 2.0);
        assert_eq!(e.mem.f32(object + 0x5c), 1.0);
        for lane in 0..4 {
            assert_eq!(e.mem.f32(object + 0x30 + 4 * lane), 2.0);
            assert_eq!(e.mem.f32(object + 0x40 + 4 * lane), 0.5);
        }
        // A signalling NaN is quieted by the FLD/FSTP pair.
        let signalling = f32::from_bits(0x7f80_0001);
        e.call(0x00c9_0e60, &args![object, signalling, 2.0f32]);
        assert_eq!(e.mem.u32(object + 4), 0x7fc0_0001);
    }

    #[test]
    fn fn_00c90ed0_shifts_the_arguments_into_the_top_bits() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        // The byte at +9 is 1: `c = 3` is not below it, so nothing is complemented.
        e.mem.set_u8(object + 9, 1);
        e.mem.set_u16(object + 0xa, 0x1234);
        e.mem.set_u16(object + 0xc, 0xffff);
        e.call(0x00c9_0ed0, &args![object, 1u32, 2u32, 3i32]);
        assert_eq!(e.mem.u16(object + 0xa), 0x1234 | 0x4000);
        assert_eq!(e.mem.u16(object + 0xc), 0x3fff | 0x8000);
        // The byte is 3 and `c = 1` is below it in both comparisons: both are complemented.
        e.mem.set_u8(object + 9, 3);
        e.mem.set_u16(object + 0xa, 0);
        e.mem.set_u16(object + 0xc, 0);
        e.call(0x00c9_0ed0, &args![object, 0u32, 1u32, 1i32]);
        assert_eq!(e.mem.u16(object + 0xa), 0xc000);
        assert_eq!(e.mem.u16(object + 0xc), 0x8000);
    }

    #[test]
    fn multithread_config_constructor_sets_its_word() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        let returned = e.call(0x00c9_0f30, &args![object]).u32();
        assert_eq!(returned, object);
        assert_eq!(e.mem.u32(object), 0x46);
    }

    #[test]
    fn fn_00c91000_copies_a_vector() {
        let mut e = Engine::new();
        let world = world(&mut e);
        let source = e.mem.alloc(0x10);
        for lane in 0..4 {
            e.mem.set_u32(source + 4 * lane, 0x1111_1111 * (lane + 1));
        }
        e.call(0x00c9_1000, &args![world, source]);
        for lane in 0..4 {
            let copied = e.mem.u32(world.addr() + 0x10 + 4 * lane);
            assert_eq!(copied, 0x1111_1111 * (lane + 1));
        }
    }

    #[test]
    fn current_times_come_from_the_simulation() {
        let mut e = Engine::new();
        let world = world(&mut e);
        let simulation = e.mem.alloc(0x40);
        e.set(world, HkpWorld::m_simulation, Ptr::new(simulation));
        e.mem.set_f32(simulation + 0x14, 1.5);
        e.mem.set_f32(simulation + 0x18, 2.5);
        assert_eq!(e.call(0x00c9_1020, &args![world]).f32(), 1.5);
        assert_eq!(e.call(0x00c9_1030, &args![world]).f32(), 2.5);
        // FLD quiets a signalling NaN.
        e.mem.set_u32(simulation + 0x14, 0x7f80_0001);
        let quieted = e.call(0x00c9_1020, &args![world]).f32();
        assert_eq!(quieted.to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn fn_00c91040_passes_the_time_to_the_simulation() {
        let mut e = logging();
        silent(&mut e, &[SIMULATION_SET_TIME]);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_simulation, Ptr::new(0x1234));
        e.call(0x00c9_1040, &args![world, 0.25f32]);
        assert_eq!(
            calls_to(&e, SIMULATION_SET_TIME),
            vec![vec![0x1234, 0.25f32.to_bits()]]
        );
    }

    #[test]
    fn fn_00c91060_returns_the_simulations_answer() {
        let mut e = logging();
        returning(&mut e, SIMULATION_TEST, 0x0102);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_simulation, Ptr::new(0x1234));
        assert_eq!(e.call(0x00c9_1060, &args![world]).u32(), 0x0102);
        assert_eq!(calls_to(&e, SIMULATION_TEST), vec![vec![0x1234]]);
    }

    /// A function that does nothing makes no calls of its own.
    fn assert_does_nothing(address: u32, words: &[u32]) {
        let mut e = logging();
        e.call(address, words);
        assert_eq!(e.call_log.as_ref().map(Vec::len), Some(1));
    }

    #[test]
    fn fn_00c910a0_does_nothing() {
        assert_does_nothing(0x00c9_10a0, &[0x1000, 5]);
    }

    #[test]
    fn check_determinism_does_nothing() {
        assert_does_nothing(0x00c9_10b0, &[0x1000]);
    }

    #[test]
    fn fn_00c911a0_does_nothing() {
        assert_does_nothing(0x00c9_11a0, &[0x1000]);
    }

    #[test]
    fn unlock_read_only_does_nothing() {
        assert_does_nothing(0x00c9_11b0, &[0x1000]);
    }

    #[test]
    fn fn_00c910d0_swaps_the_border_and_its_references() {
        let mut e = logging();
        silent(&mut e, &[FAKE_1, ADD_REFERENCE, REMOVE_REFERENCE]);
        let world = world(&mut e);
        let old = object_with_vtable(&mut e, 0x40, &[(0x10, FAKE_1)]);
        e.set(world, HkpWorld::m_broadPhaseBorder, Ptr::new(old));
        e.call(0x00c9_10d0, &args![world, 0x5000u32]);
        assert_eq!(e.get(world, HkpWorld::m_broadPhaseBorder).addr(), 0x5000);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![old]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![old]]);
        assert_eq!(calls_to(&e, ADD_REFERENCE), vec![vec![0x5000]]);
        // Clearing the border releases the old one and adds nothing.
        let second = object_with_vtable(&mut e, 0x40, &[(0x10, FAKE_1)]);
        e.set(world, HkpWorld::m_broadPhaseBorder, Ptr::new(second));
        e.call(0x00c9_10d0, &args![world, 0u32]);
        assert_eq!(e.get(world, HkpWorld::m_broadPhaseBorder).addr(), 0);
        assert_eq!(calls_to(&e, ADD_REFERENCE).len(), 1);
        assert_eq!(
            calls_to(&e, REMOVE_REFERENCE),
            vec![vec![old], vec![second]]
        );
        // With no old border nothing is released.
        e.call(0x00c9_10d0, &args![world, 0x6000u32]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE).len(), 2);
        assert_eq!(calls_to(&e, ADD_REFERENCE).len(), 2);
    }

    #[test]
    fn fn_00c91110_returns_the_border() {
        let mut e = Engine::new();
        let world = world(&mut e);
        e.set(world, HkpWorld::m_broadPhaseBorder, Ptr::new(0x4321));
        assert_eq!(e.call(0x00c9_1110, &args![world]).u32(), 0x4321);
    }

    #[test]
    fn fn_00c91120_clears_the_count_and_flushes_the_queue() {
        let mut e = logging();
        silent(&mut e, &[QUEUE_FLUSH_PENDING]);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        e.set(world, HkpWorld::m_pendingOperationsCount, 7);
        e.call(0x00c9_1120, &args![world]);
        assert_eq!(e.get(world, HkpWorld::m_pendingOperationsCount), 0);
        assert_eq!(calls_to(&e, QUEUE_FLUSH_PENDING), vec![vec![0x5000]]);
    }

    #[test]
    fn fn_00c91140_clears_the_body_count_and_flushes_the_queue() {
        let mut e = logging();
        silent(&mut e, &[QUEUE_FLUSH_BODY_PENDING]);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        e.set(world, HkpWorld::m_pendingBodyOperationsCount, 7);
        e.call(0x00c9_1140, &args![world]);
        assert_eq!(e.get(world, HkpWorld::m_pendingBodyOperationsCount), 0);
        assert_eq!(calls_to(&e, QUEUE_FLUSH_BODY_PENDING), vec![vec![0x5000]]);
    }

    #[test]
    fn fn_00c91160_enqueues_the_operation() {
        let mut e = logging();
        silent(&mut e, &[QUEUE_ENQUEUE]);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        e.call(0x00c9_1160, &args![world, 0x6000u32]);
        assert_eq!(calls_to(&e, QUEUE_ENQUEUE), vec![vec![0x5000, 0x6000]]);
    }

    #[test]
    fn lock_and_unlock_count_the_world_and_call_the_helpers() {
        let mut e = logging();
        silent(&mut e, &[LOCK_ENTER, LOCK_LEAVE]);
        let world = world(&mut e);
        e.call(0x00c9_1180, &args![world]);
        assert_eq!(e.get(world, HkpWorld::m_isLocked), 1);
        assert_eq!(calls_to(&e, LOCK_ENTER), vec![vec![world.addr()]]);
        e.call(0x00c9_1190, &args![world]);
        assert_eq!(e.get(world, HkpWorld::m_isLocked), 0);
        assert_eq!(calls_to(&e, LOCK_LEAVE), vec![vec![world.addr()]]);
    }

    /// Runs the critical-section function at `address` with and without a section.
    fn check_critical_section(address: u32, import_slot: u32) {
        let mut e = logging();
        silent(&mut e, &[import_slot]);
        let world = world(&mut e);
        e.call(address, &args![world, 0u32]);
        assert!(calls_to(&e, import_slot).is_empty());
        e.set(
            world,
            HkpWorld::m_modifyConstraintCriticalSection,
            Ptr::new(0x7000),
        );
        e.call(address, &args![world, 0u32]);
        assert_eq!(calls_to(&e, import_slot), vec![vec![0x7000]]);
    }

    #[test]
    fn lock_island_for_constraint_update_enters_the_section() {
        check_critical_section(0x00c9_11d0, ENTER_CRITICAL_SECTION);
    }

    #[test]
    fn fn_00c911f0_enters_the_section() {
        check_critical_section(0x00c9_11f0, ENTER_CRITICAL_SECTION);
    }

    #[test]
    fn unlock_island_for_constraint_update_leaves_the_section() {
        check_critical_section(0x00c9_1210, LEAVE_CRITICAL_SECTION);
    }

    #[test]
    fn fn_00c91230_leaves_the_section() {
        check_critical_section(0x00c9_1230, LEAVE_CRITICAL_SECTION);
    }

    #[test]
    fn fn_00c91250_tells_the_tree_when_there_is_one() {
        let mut e = logging();
        silent(&mut e, &[FAKE_1]);
        let world = world(&mut e);
        e.call(0x00c9_1250, &args![world]);
        assert!(calls_to(&e, FAKE_1).is_empty());
        let tree = object_with_vtable(&mut e, 0x20, &[(0x14, FAKE_1)]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
        e.call(0x00c9_1250, &args![world]);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![tree, 0]]);
    }

    #[test]
    fn fn_00c91270_calls_the_trees_slot_with_both_arguments() {
        let mut e = logging();
        silent(&mut e, &[FAKE_1]);
        let world = world(&mut e);
        let tree = object_with_vtable(&mut e, 0x20, &[(0xc, FAKE_1)]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
        e.call(0x00c9_1270, &args![world, 5u32, 6u32]);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![tree, 5, 6]]);
    }

    #[test]
    fn fn_00c912e0_registers_three_helpers() {
        let mut e = logging();
        let blocks: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x20)).collect();
        let next = Rc::new(RefCell::new(blocks.clone().into_iter()));
        e.register_double(FAKE_1, move |_, _| Ret {
            eax: next.borrow_mut().next().unwrap_or(0),
            ..Ret::default()
        });
        let allocator = object_with_vtable(&mut e, 0x20, &[(4, FAKE_1)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        returning(&mut e, MEMORY_ROUTER, router);
        returning(&mut e, MAKE_HELPER_FIRST, 0xf100);
        returning(&mut e, MAKE_HELPER_SECOND, 0xf200);
        silent(&mut e, &[REGISTER_HELPER, REMOVE_REFERENCE]);
        fn_00c912e0(&mut e, Ptr::new(0x1000), Ptr::new(0x2000));
        assert_eq!(
            calls_to(&e, MAKE_HELPER_FIRST),
            vec![vec![blocks[0], 0x1000]]
        );
        assert_eq!(
            calls_to(&e, MAKE_HELPER_SECOND),
            vec![vec![blocks[1], 0x1000]]
        );
        assert_eq!(e.mem.u16(blocks[0] + 4), 0xc);
        assert_eq!(e.mem.u16(blocks[1] + 4), 0xc);
        let third = blocks[2];
        assert_eq!(e.mem.u16(third + 4), 0x14);
        assert_eq!(e.mem.u16(third + 6), 1);
        assert_eq!(e.mem.u16(third + 0xe), 1);
        assert_eq!(e.mem.u32(third), 0x010c_6420);
        assert_eq!(e.mem.u32(third + 8), 0x010c_6378);
        assert_eq!(e.mem.u32(third + 0x10), 3);
        assert_eq!(
            calls_to(&e, REGISTER_HELPER),
            vec![
                vec![0x2000, 0xf100, 1],
                vec![0x2000, 0xf200, 2],
                vec![0x2000, third, 3]
            ]
        );
        assert_eq!(
            calls_to(&e, REMOVE_REFERENCE),
            vec![vec![0xf100], vec![0xf200], vec![third]]
        );
    }

    /// Runs `reenableCollisionBetweenEntityPair` on a world whose broad phase answers
    /// `accept`, whose filter says `enabled` and whose agent search finds `agent`. Returns
    /// the pairs handed to the dispatcher as `[dispatcher, handle a, handle b, count, filter
    /// base]`, and the two entities and the filter.
    fn reenable_scenario(accept: u32, enabled: u8, agent: u32) -> (Vec<Vec<u32>>, [u32; 3]) {
        let mut e = logging();
        let pairs: Records = Rc::new(RefCell::new(vec![]));
        let sink = pairs.clone();
        e.register_double(ADD_PAIRS, move |e, words| {
            let pair = words[1];
            sink.borrow_mut().push(vec![
                words[0],
                e.mem.u32(pair),
                e.mem.u32(pair + 4),
                words[2],
                words[3],
            ]);
            Ret::default()
        });
        returning(&mut e, FIND_AGENT, agent);
        returning(&mut e, FAKE_3, accept);
        e.register_double(FAKE_2, move |e, words| {
            e.mem.set_u8(words[1], enabled);
            Ret {
                eax: words[1],
                ..Ret::default()
            }
        });
        let world = world(&mut e);
        let broad_phase = object_with_vtable(&mut e, 0x20, &[(0x4c, FAKE_3)]);
        e.set(world, HkpWorld::m_broadPhase, Ptr::new(broad_phase));
        // The filter's base is at +8: its vtable pointer sits there and slot +4 answers.
        let filter = e.mem.alloc(0x40);
        let filter_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(filter + 8, filter_vtable);
        e.mem.set_u32(filter_vtable + 4, FAKE_2);
        e.set(world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        e.set(world, HkpWorld::m_broadPhaseDispatcher, Ptr::new(0x8000));
        let entity_a = e.mem.alloc(0x40);
        let entity_b = e.mem.alloc(0x40);
        e.call(0x00c9_13b0, &args![world, entity_a, entity_b]);
        let recorded = pairs.borrow().clone();
        (recorded, [entity_a, entity_b, filter])
    }

    #[test]
    fn reenable_collision_adds_the_pair_when_everything_allows() {
        let (pairs, [entity_a, entity_b, filter]) = reenable_scenario(1, 1, 0);
        assert_eq!(
            pairs,
            vec![vec![
                0x8000,
                entity_a + 0x24,
                entity_b + 0x24,
                1,
                filter + 8
            ]]
        );
    }

    #[test]
    fn reenable_collision_stops_at_each_refusal() {
        assert!(reenable_scenario(0, 1, 0).0.is_empty());
        assert!(reenable_scenario(1, 0, 0).0.is_empty());
        assert!(reenable_scenario(1, 1, 0x99).0.is_empty());
    }

    #[test]
    fn reenable_collision_is_queued_while_locked() {
        let mut e = logging();
        let queued = operation_recorder(&mut e);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_criticalOperationsLockCount, 1);
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        e.call(0x00c9_13b0, &args![world, 0x111u32, 0x222u32]);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(0x13, 0x111, 0x222, 0))]
        );
    }

    #[test]
    fn fn_00c91470_queues_or_calls_the_simulation() {
        let mut e = logging();
        let queued = operation_recorder(&mut e);
        silent(&mut e, &[FAKE_1]);
        let world = world(&mut e);
        let simulation = object_with_vtable(&mut e, 0x40, &[(0x30, FAKE_1)]);
        e.set(world, HkpWorld::m_simulation, Ptr::new(simulation));
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        e.call(0x00c9_1470, &args![world, 1u32, 0x12345u32, 0x1ffu32]);
        assert_eq!(
            calls_to(&e, FAKE_1),
            vec![vec![simulation, 1, 0x12345, world.addr(), 0x1ff]]
        );
        assert!(queued.borrow().is_empty());
        e.set(world, HkpWorld::m_criticalOperationsLockCount, 1);
        e.call(0x00c9_1470, &args![world, 1u32, 0x12345u32, 0x1ffu32]);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(0x17, 1, 0x2345, 0xff))]
        );
        assert_eq!(calls_to(&e, FAKE_1).len(), 1);
    }

    /// An entity whose vtable slot +0x10 makes a motion and slot +0x14 is the deleter.
    fn entity(e: &mut Engine) -> Ptr<HkpEntity> {
        Ptr::new(object_with_vtable(
            e,
            0x220,
            &[(0x10, FAKE_1), (0x14, FAKE_2)],
        ))
    }

    fn add_entity_engine() -> Engine {
        let mut e = logging();
        silent(
            &mut e,
            &[
                SET_CACHED_SHAPE_DATA,
                SET_TIME_INFORMATION,
                ADD_REFERENCE,
                ADD_ENTITY_SI,
                ADD_ENTITY_BP,
                FIRE_WORLD_ENTITY_ADDED,
                FIRE_ENTITY_ADDED,
                QUEUE_FLUSH_PENDING,
                QUEUE_FLUSH_BODY_PENDING,
                FAKE_2,
            ],
        );
        returning(&mut e, FAKE_1, 0x5555);
        e
    }

    #[test]
    fn add_entity_runs_the_whole_sequence() {
        let mut e = add_entity_engine();
        let world = world(&mut e);
        let entity = entity(&mut e);
        e.set(world, HkpWorld::m_lastEntityUid, 41);
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        e.set(world, HkpWorld::m_pendingOperationsCount, 3);
        e.set(world, HkpWorld::m_pendingOperationQueueCount, 1);
        e.set(world, HkpWorld::m_pendingBodyOperationsCount, 2);
        e.mem.set_u8(world.addr() + 0x305, 3);
        e.mem.set_u8(world.addr() + 0x306, 2);
        e.mem.set_u8(world.addr() + 0x307, 1);
        e.mem.set_u8(entity.addr() + 0xe9, 2);
        e.set(entity, HkpEntity::m_collidableShape, Ptr::new(0x777));
        e.set(entity, HkpEntity::m_wordEa, 0x1111);
        e.set(entity, HkpEntity::m_wordEc, 0x2222);
        let returned = e.call(0x00c9_14d0, &args![world, entity, 9u32]).u32();
        assert_eq!(returned, entity.addr());
        assert_eq!(e.get(entity, HkpEntity::m_collidableMotion).addr(), 0x5555);
        assert_eq!(e.get(entity, HkpEntity::m_uid), 42);
        assert_eq!(e.get(world, HkpWorld::m_lastEntityUid), 42);
        assert_eq!(e.get(world, HkpWorld::m_criticalOperationsLockCount), 0);
        assert!(e.get(world, HkpWorld::m_criticalOperationsAllowed));
        // The two words were cleared, then `00c90ed0(3, 2, 1)` on the motion with the byte 2
        // at +9: both arguments are complemented, giving 0 and 0x4000.
        assert_eq!(e.get(entity, HkpEntity::m_wordEa), 0);
        assert_eq!(e.get(entity, HkpEntity::m_wordEc), 0x4000);
        assert_eq!(
            calls_to(&e, SET_CACHED_SHAPE_DATA),
            vec![vec![entity.addr(), world.addr(), 0x777]]
        );
        assert_eq!(
            calls_to(&e, SET_TIME_INFORMATION),
            vec![vec![0, 0, entity.addr() + 0xf0]]
        );
        assert_eq!(
            calls_to(&e, ADD_ENTITY_SI),
            vec![vec![world.addr(), entity.addr(), 9]]
        );
        let sequence = [
            SET_CACHED_SHAPE_DATA,
            SET_TIME_INFORMATION,
            ADD_REFERENCE,
            ADD_ENTITY_SI,
            ADD_ENTITY_BP,
            FIRE_WORLD_ENTITY_ADDED,
            FIRE_ENTITY_ADDED,
            QUEUE_FLUSH_PENDING,
            QUEUE_FLUSH_BODY_PENDING,
        ];
        assert_eq!(order_of(&e, &sequence), sequence.to_vec());
        assert_eq!(e.get(world, HkpWorld::m_pendingOperationsCount), 0);
        assert_eq!(e.get(world, HkpWorld::m_pendingBodyOperationsCount), 0);
        assert_eq!(calls_to(&e, QUEUE_FLUSH_PENDING), vec![vec![0x5000]]);
    }

    #[test]
    fn add_entity_keeps_an_existing_motion_and_respects_the_block_flag() {
        let mut e = add_entity_engine();
        let world = world(&mut e);
        let entity = entity(&mut e);
        e.set(entity, HkpEntity::m_collidableMotion, Ptr::new(0x6666));
        e.set(world, HkpWorld::m_pendingOperationsCount, 3);
        e.set(world, HkpWorld::m_blockExecutingPendingOperations, true);
        e.call(0x00c9_14d0, &args![world, entity, 9u32]);
        assert_eq!(e.get(entity, HkpEntity::m_collidableMotion).addr(), 0x6666);
        assert!(calls_to(&e, FAKE_1).is_empty());
        assert_eq!(e.get(world, HkpWorld::m_pendingOperationsCount), 3);
        assert!(calls_to(&e, QUEUE_FLUSH_PENDING).is_empty());
    }

    #[test]
    fn add_entity_is_queued_inside_a_critical_section() {
        let mut e = add_entity_engine();
        let queued = operation_recorder(&mut e);
        let world = world(&mut e);
        let entity = entity(&mut e);
        e.set(world, HkpWorld::m_criticalOperationsLockCount, 2);
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        let returned = e.call(0x00c9_14d0, &args![world, entity, 9u32]).u32();
        assert_eq!(returned, 0);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(1, entity.addr(), 9, 0))]
        );
        assert!(calls_to(&e, ADD_ENTITY_SI).is_empty());
    }

    fn remove_entity_engine() -> Engine {
        let mut e = logging();
        silent(
            &mut e,
            &[
                REMOVE_ENTITY_SI,
                FIRE_WORLD_ENTITY_REMOVED,
                FIRE_ENTITY_REMOVED,
                REMOVE_ENTITY_BP,
                ENTITY_RELEASE_BOUNDING_DATA,
                REMOVE_REFERENCE,
                QUEUE_FLUSH_PENDING,
                QUEUE_FLUSH_BODY_PENDING,
                FAKE_1,
                FAKE_2,
            ],
        );
        e
    }

    #[test]
    fn remove_entity_runs_the_whole_sequence() {
        let mut e = remove_entity_engine();
        let world = world(&mut e);
        let entity = entity(&mut e);
        let result = e.mem.alloc(8);
        e.set(world, HkpWorld::m_pendingOperationsCount, 1);
        e.set(
            entity,
            HkpEntity::m_boundingVolumeChildData,
            Ptr::new(0x9000),
        );
        e.call(0x00c9_1620, &args![world, result, entity]);
        assert_eq!(e.mem.u8(result), 1);
        assert_eq!(e.get(world, HkpWorld::m_criticalOperationsLockCount), 0);
        assert!(e.get(world, HkpWorld::m_criticalOperationsAllowed));
        let sequence = [
            REMOVE_ENTITY_SI,
            FIRE_WORLD_ENTITY_REMOVED,
            FIRE_ENTITY_REMOVED,
            REMOVE_ENTITY_BP,
            ENTITY_RELEASE_BOUNDING_DATA,
            FAKE_2,
            REMOVE_REFERENCE,
            QUEUE_FLUSH_PENDING,
        ];
        assert_eq!(order_of(&e, &sequence), sequence.to_vec());
        assert_eq!(
            calls_to(&e, ENTITY_RELEASE_BOUNDING_DATA),
            vec![vec![entity.addr() + 0x30]]
        );
        assert_eq!(calls_to(&e, FAKE_2), vec![vec![entity.addr()]]);
        assert_eq!(calls_to(&e, REMOVE_REFERENCE), vec![vec![entity.addr()]]);
    }

    #[test]
    fn remove_entity_answers_its_result_pointer() {
        let mut e = remove_entity_engine();
        let world = world(&mut e);
        let entity = entity(&mut e);
        let result = e.mem.alloc(8);
        e.set(world, HkpWorld::m_pendingOperationsCount, 1);
        e.set(
            entity,
            HkpEntity::m_boundingVolumeChildData,
            Ptr::new(0x9000),
        );
        // EAX is `result` on both paths; the callers test the flag through it (00c68c93).
        assert_eq!(
            e.call(0x00c9_1620, &args![world, result, entity]).u32(),
            result
        );
    }

    #[test]
    fn remove_entity_skips_the_optional_steps() {
        let mut e = remove_entity_engine();
        let world = world(&mut e);
        let entity = entity(&mut e);
        let result = e.mem.alloc(8);
        e.set(entity, HkpEntity::m_memSizeAndFlags, 0x20);
        e.set(world, HkpWorld::m_blockExecutingPendingOperations, true);
        e.set(world, HkpWorld::m_pendingOperationsCount, 1);
        e.call(0x00c9_1620, &args![world, result, entity]);
        assert!(calls_to(&e, ENTITY_RELEASE_BOUNDING_DATA).is_empty());
        assert!(calls_to(&e, FAKE_2).is_empty());
        assert!(calls_to(&e, QUEUE_FLUSH_PENDING).is_empty());
        assert_eq!(e.mem.u8(result), 1);
    }

    #[test]
    fn remove_entity_is_queued_inside_a_critical_section() {
        let mut e = remove_entity_engine();
        let queued = operation_recorder(&mut e);
        let world = world(&mut e);
        let entity = entity(&mut e);
        let result = e.mem.alloc(8);
        e.mem.set_u8(result, 7);
        e.set(world, HkpWorld::m_criticalOperationsLockCount, 1);
        e.set(world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        e.call(0x00c9_1620, &args![world, result, entity]);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(2, entity.addr(), 0, 0))]
        );
        assert_eq!(e.mem.u8(result), 0);
        assert!(calls_to(&e, REMOVE_ENTITY_SI).is_empty());
    }

    /// Nulls a listener in the array at `data_offset` through the function at `address`.
    fn check_listener_removal(address: u32, data_offset: u32) {
        let mut e = Engine::new();
        let world = world(&mut e);
        let data = array_of(&mut e, &[0x10, 0x20, 0x30]);
        e.mem.set_u32(world.addr() + data_offset, data);
        e.mem.set_i32(world.addr() + data_offset + 4, 3);
        e.call(address, &args![world, 0x20u32]);
        assert_eq!(
            (e.mem.u32(data), e.mem.u32(data + 4), e.mem.u32(data + 8)),
            (0x10, 0, 0x30)
        );
        // No match: the word before the array is cleared.
        e.mem.set_u32(data - 4, 0xdead);
        e.call(address, &args![world, 0x99u32]);
        assert_eq!(e.mem.u32(data - 4), 0);
        assert_eq!(e.mem.u32(data), 0x10);
        // An empty array behaves the same way.
        e.mem.set_u32(data - 4, 0xdead);
        e.mem.set_i32(world.addr() + data_offset + 4, 0);
        e.call(address, &args![world, 0x10u32]);
        assert_eq!(e.mem.u32(data - 4), 0);
        assert_eq!(e.mem.u32(data), 0x10);
    }

    #[test]
    fn fn_00c91700_removes_a_listener() {
        check_listener_removal(0x00c9_1700, 0x11c);
    }

    #[test]
    fn fn_00c91750_removes_a_listener() {
        check_listener_removal(0x00c9_1750, 0x140);
    }

    #[test]
    fn fn_00c917a0_removes_a_listener() {
        check_listener_removal(0x00c9_17a0, 0x128);
    }

    #[test]
    fn fn_00c917f0_removes_a_listener() {
        check_listener_removal(0x00c9_17f0, 0x134);
    }

    #[test]
    fn fn_00c91840_removes_a_listener() {
        check_listener_removal(0x00c9_1840, 0x158);
    }

    #[test]
    fn fn_00c918e0_removes_a_listener() {
        check_listener_removal(0x00c9_18e0, 0x164);
    }

    #[test]
    fn fn_00c91a20_removes_a_listener() {
        check_listener_removal(0x00c9_1a20, 0x1a0);
    }

    #[test]
    fn fn_00c91a70_removes_a_listener() {
        check_listener_removal(0x00c9_1a70, 0x14c);
    }

    /// A world whose simulation's slots +0xc, +0x10, +0x14 and +0x18 all answer 0x77.
    fn stepping_world(e: &mut Engine) -> (Ptr<HkpWorld>, u32) {
        silent(e, &[LOCK_ENTER, LOCK_LEAVE, WATCH_HEAP_MEMORY]);
        let world = world(e);
        let slots = [
            (0xc, FAKE_1),
            (0x10, FAKE_1),
            (0x14, FAKE_1),
            (0x18, FAKE_1),
        ];
        let simulation = object_with_vtable(e, 0x40, &slots);
        returning(e, FAKE_1, 0x77);
        e.set(world, HkpWorld::m_simulation, Ptr::new(simulation));
        (world, simulation)
    }

    #[test]
    fn step_delta_time_steps_inside_the_lock() {
        let mut e = logging();
        let (world, simulation) = stepping_world(&mut e);
        let result = e.call(0x00c9_1b10, &args![world, 0.5f32]).u32();
        assert_eq!(result, 0x77);
        let sequence = [LOCK_ENTER, WATCH_HEAP_MEMORY, FAKE_1, LOCK_LEAVE];
        assert_eq!(order_of(&e, &sequence), sequence.to_vec());
        assert_eq!(calls_to(&e, WATCH_HEAP_MEMORY), vec![vec![world.addr()]]);
        assert_eq!(
            calls_to(&e, FAKE_1),
            vec![vec![simulation, 0.5f32.to_bits()]]
        );
        assert_eq!(e.get(world, HkpWorld::m_isLocked), 0);
    }

    #[test]
    fn step_delta_time_tells_the_tree_afterwards() {
        let mut e = logging();
        let (world, _) = stepping_world(&mut e);
        silent(&mut e, &[FAKE_2, FAKE_3]);
        let tree = object_with_vtable(&mut e, 0x20, &[(0xc, FAKE_2), (0x14, FAKE_3)]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
        e.call(0x00c9_1b10, &args![world, 0.5f32]);
        assert!(calls_to(&e, FAKE_2).is_empty());
        assert_eq!(calls_to(&e, FAKE_3), vec![vec![tree, 0]]);
        e.set(world, HkpWorld::m_autoUpdateTree, true);
        e.call(0x00c9_1b10, &args![world, 0.5f32]);
        assert_eq!(calls_to(&e, FAKE_2), vec![vec![tree, 0, 0]]);
        assert_eq!(calls_to(&e, FAKE_3).len(), 1);
    }

    #[test]
    fn fn_00c91b70_passes_the_time_to_slot_0x10() {
        let mut e = logging();
        let (world, simulation) = stepping_world(&mut e);
        let result = e.call(0x00c9_1b70, &args![world, 2.0f32]).u32();
        assert_eq!(result, 0x77);
        assert_eq!(
            calls_to(&e, FAKE_1),
            vec![vec![simulation, 2.0f32.to_bits()]]
        );
        assert_eq!(calls_to(&e, LOCK_ENTER).len(), 1);
        assert_eq!(calls_to(&e, LOCK_LEAVE).len(), 1);
    }

    #[test]
    fn fn_00c91ba0_calls_slot_0x14_locked() {
        let mut e = logging();
        let (world, simulation) = stepping_world(&mut e);
        assert_eq!(e.call(0x00c9_1ba0, &args![world]).u32(), 0x77);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![simulation]]);
        assert_eq!(calls_to(&e, LOCK_ENTER).len(), 1);
        assert_eq!(calls_to(&e, LOCK_LEAVE).len(), 1);
    }

    #[test]
    fn fn_00c91bd0_calls_slot_0x18_locked() {
        let mut e = logging();
        let (world, simulation) = stepping_world(&mut e);
        assert_eq!(e.call(0x00c9_1bd0, &args![world]).u32(), 0x77);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![simulation]]);
        assert_eq!(calls_to(&e, LOCK_ENTER).len(), 1);
        assert_eq!(calls_to(&e, LOCK_LEAVE).len(), 1);
    }

    #[test]
    fn finish_mt_step_returns_a_failure_and_otherwise_tells_the_tree() {
        let mut e = logging();
        silent(&mut e, &[FAKE_2, FAKE_3]);
        returning(&mut e, SIMULATION_FINISH_MT_STEP, 5);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_simulation, Ptr::new(0x1234));
        let tree = object_with_vtable(&mut e, 0x20, &[(0xc, FAKE_2), (0x14, FAKE_3)]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
        assert_eq!(e.call(0x00c9_1c00, &args![world, 7u32, 8u32]).u32(), 5);
        assert_eq!(
            calls_to(&e, SIMULATION_FINISH_MT_STEP),
            vec![vec![0x1234, 7, 8]]
        );
        assert!(calls_to(&e, FAKE_2).is_empty() && calls_to(&e, FAKE_3).is_empty());
        returning(&mut e, SIMULATION_FINISH_MT_STEP, 0);
        assert_eq!(e.call(0x00c9_1c00, &args![world, 7u32, 8u32]).u32(), 0);
        assert_eq!(calls_to(&e, FAKE_3), vec![vec![tree, 0]]);
        e.set(world, HkpWorld::m_autoUpdateTree, true);
        assert_eq!(e.call(0x00c9_1c00, &args![world, 7u32, 8u32]).u32(), 0);
        assert_eq!(calls_to(&e, FAKE_2), vec![vec![tree, 7, 8]]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::NULL);
        assert_eq!(e.call(0x00c9_1c00, &args![world, 7u32, 8u32]).u32(), 0);
        assert_eq!(calls_to(&e, FAKE_2).len(), 1);
    }

    /// The plain copies of `00c91c50`: (offset in the world, offset in the info, bytes).
    const INFO_COPIES: [(u32, u32, u32); 33] = [
        (0xd0, 0x90, 4),
        (0xdc, 0xac, 4),
        (0xd8, 0xa8, 4),
        (0xf4, 0xc0, 4),
        (0xf8, 0xc4, 4),
        (0xfc, 0xc8, 4),
        (0x100, 0xcc, 4),
        (0x240, 0x24, 4),
        (0x1e4, 0x78, 4),
        (0x1e8, 0x7c, 4),
        (0x338, 0x20, 4),
        (0x330, 0x70, 4),
        (0x334, 0x64, 4),
        (0x2f4, 0x80, 4),
        (0x2f8, 0x84, 4),
        (0xe0, 0xb0, 4),
        (0xe8, 0xb8, 4),
        (0xec, 0xbc, 4),
        (0xb4, 0xd4, 4),
        (0x60, 0x53, 1),
        (0xd4, 0x98, 1),
        (0x340, 0x74, 1),
        (0x304, 0x8c, 1),
        (0x305, 0xa4, 1),
        (0x306, 0xa5, 1),
        (0x307, 0xa6, 1),
        (0xd6, 0xa7, 1),
        (0xe4, 0xb4, 1),
        (0xd5, 0xd0, 1),
        (0xf0, 0xd1, 1),
        (0xc8, 0xd2, 1),
        (0xb0, 0xd8, 1),
        (0xb1, 0xd9, 1),
    ];

    #[test]
    fn get_info_copies_the_worlds_settings() {
        let mut e = logging();
        silent(&mut e, &[ADD_REFERENCE, REMOVE_REFERENCE]);
        e.register(X87_SQRT, |_, words| Ret {
            st0: f64::take(words, &mut 0).sqrt(),
            ..Ret::default()
        });
        let world = world(&mut e);
        let w = world.addr();
        let info = e.mem.alloc(0x100);
        for (index, (from, _, size)) in INFO_COPIES.iter().enumerate() {
            let value = 0x3f00_0000 + index as u32 + 1;
            if *size == 4 {
                e.mem.set_u32(w + from, value);
            } else {
                e.mem.set_u8(w + from, value as u8);
            }
        }
        // The word at +0xcc is copied first and then overwritten by the one at +0xd0, which
        // is a signalling NaN here and comes out quieted.
        e.mem.set_u32(w + 0xcc, 0x3e99_999a);
        e.mem.set_u32(w + 0xd0, 0x7f80_0001);
        for lane in 0..4 {
            e.mem.set_u32(w + 0x10 + 4 * lane, 0xa0 + lane);
            e.mem.set_u32(w + 0x310 + 4 * lane, 0xb0 + lane);
            e.mem.set_u32(w + 0x320 + 4 * lane, 0xc0 + lane);
        }
        let collision_input = e.mem.alloc(0x80);
        let extra = e.mem.alloc(8);
        e.mem.set_f32(collision_input + 8, 1.25);
        e.mem.set_u8(collision_input + 0x6d, 0x6d);
        e.mem.set_u8(collision_input + 0x6c, 0x6c);
        e.mem.set_u32(collision_input + 0x70, extra);
        e.mem.set_f32(extra, 3.5);
        e.mem.set_u32(extra + 4, 0x1357);
        e.set(world, HkpWorld::m_collisionInput, Ptr::new(collision_input));
        let dispatcher = e.mem.alloc(0x1e50);
        e.mem.set_f32(dispatcher + 0x1e40, 0.75);
        e.mem.set_f32(dispatcher + 0x1e44, 0.125);
        e.set(world, HkpWorld::m_collisionDispatcher, Ptr::new(dispatcher));
        let simulation = e.mem.alloc(0x40);
        e.mem.set_f32(simulation + 0x24, 9.5);
        e.set(world, HkpWorld::m_simulation, Ptr::new(simulation));
        e.mem.set_f32(w + 0x308, 16.0);
        e.set(world, HkpWorld::m_collisionFilter, Ptr::new(0x1001));
        e.set(world, HkpWorld::m_convexListFilter, Ptr::new(0x1002));
        e.set(world, HkpWorld::m_memoryWatchDog, Ptr::new(0x1003));
        e.mem.set_u32(info + 0x58, 0x2001);
        e.mem.set_u32(info + 0x5c, 0);
        e.mem.set_u32(info + 0x6c, 0x2003);
        e.call(0x00c9_1c50, &args![world, info]);
        for (index, (from, to, size)) in INFO_COPIES.iter().enumerate() {
            let value = 0x3f00_0000 + index as u32 + 1;
            if *from == 0xd0 {
                assert_eq!(e.mem.u32(info + to), 0x7fc0_0001);
            } else if *size == 4 {
                assert_eq!(e.mem.u32(info + to), value, "word {from:#x}");
            } else {
                assert_eq!(e.mem.u8(info + to), value as u8, "byte {from:#x}");
            }
        }
        for lane in 0..4 {
            assert_eq!(e.mem.u32(info + 0x10 + 4 * lane), 0xa0 + lane);
            assert_eq!(e.mem.u32(info + 0x30 + 4 * lane), 0xb0 + lane);
            assert_eq!(e.mem.u32(info + 0x40 + 4 * lane), 0xc0 + lane);
        }
        assert_eq!(e.mem.f32(info + 0x54), 1.25);
        assert_eq!(e.mem.u8(info + 0x75), 0x6d);
        assert_eq!(e.mem.u8(info + 0x99), 0x6c);
        assert_eq!(e.mem.f32(info + 0x9c), 3.5);
        assert_eq!(e.mem.u32(info + 0xa0), 0x1357);
        assert_eq!(e.mem.f32(info + 0x60), 0.75);
        assert_eq!(e.mem.f32(info + 0x68), 0.125);
        assert_eq!(e.mem.f32(info + 0xdc), 9.5);
        assert_eq!(e.mem.f32(info + 0x88), 4.0);
        // No border: kind 3 and flag 0; no tree, no callback util.
        assert_eq!(e.mem.u8(info + 0x28), 3);
        assert_eq!(e.mem.u8(info + 0x29), 0);
        assert_eq!(e.mem.u8(info + 0x50), 0);
        assert_eq!(e.mem.u8(info + 0xe0), 0);
        // References: added to each new pointer, removed from each one replaced.
        assert_eq!(
            (
                e.mem.u32(info + 0x58),
                e.mem.u32(info + 0x5c),
                e.mem.u32(info + 0x6c)
            ),
            (0x1001, 0x1002, 0x1003)
        );
        assert_eq!(
            calls_to(&e, ADD_REFERENCE),
            vec![vec![0x1001], vec![0x1002], vec![0x1003]]
        );
        assert_eq!(
            calls_to(&e, REMOVE_REFERENCE),
            vec![vec![0x2001], vec![0x2003]]
        );
        // With a border, a tree and the callback util present.
        let border = e.mem.alloc(0x40);
        e.mem.set_u8(border + 0x30, 5);
        e.mem.set_u8(border + 0x34, 1);
        e.set(world, HkpWorld::m_broadPhaseBorder, Ptr::new(border));
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(0x3000));
        e.set(world, HkpWorld::m_collisionCallbackUtil, Ptr::new(0x3001));
        e.call(0x00c9_1c50, &args![world, info]);
        assert_eq!(e.mem.u8(info + 0x28), 5);
        assert_eq!(e.mem.u8(info + 0x29), 1);
        assert_eq!(e.mem.u8(info + 0x50), 1);
        assert_eq!(e.mem.u8(info + 0xe0), 1);
    }

    /// An engine with a profiling stream that holds `records` records; returns it and the
    /// stream block and the first record address.
    fn profiling_engine(records: u32) -> (Engine, u32, u32) {
        let mut e = logging();
        e.map(0x0126_8000, 0x1000);
        e.set_global(PROFILE_SLOT, 3u32);
        let stream = e.mem.alloc(0x20);
        let area = e.mem.alloc(12 * records);
        e.mem.set_u32(stream + 4, area);
        e.mem.set_u32(stream + 0xc, area + 12 * records);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (e, stream, area)
    }

    /// The labels of the profiling records written so far.
    fn labels(e: &Engine, stream: u32, start: u32) -> Vec<u32> {
        let cursor = e.mem.u32(stream + 4);
        (start..cursor)
            .step_by(12)
            .map(|record| e.mem.u32(record))
            .collect()
    }

    #[test]
    fn cast_ray_uses_the_tree_when_it_is_in_use() {
        let (mut e, stream, start) = profiling_engine(4);
        silent(&mut e, &[FAKE_1]);
        returning(&mut e, FAKE_2, 1);
        let world = world(&mut e);
        let tree = object_with_vtable(&mut e, 0x20, &[(0x10, FAKE_2), (0x18, FAKE_1)]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
        e.set(world, HkpWorld::m_collisionFilter, Ptr::new(0x1234));
        e.call(0x00c9_1f70, &args![world, 0x11u32, 0x22u32]);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![tree, 0x11, 0x1234, 0x22]]);
        assert!(calls_to(&e, WORLD_RAY_CASTER_CAST_RAY).is_empty());
        assert_eq!(
            labels(&e, stream, start),
            vec![PROFILE_CAST_RAY_COLLECTOR, PROFILE_END]
        );
    }

    #[test]
    fn cast_ray_builds_a_caster_without_the_tree() {
        let (mut e, stream, start) = profiling_engine(1);
        let words: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        let sink = words.clone();
        e.register_double(WORLD_RAY_CASTER_CAST_RAY, move |e, a| {
            let caster = a[0];
            let seen = [
                e.mem.u32(caster),
                e.mem.u32(caster + 0x40),
                e.mem.u32(caster + 0x44),
            ];
            sink.borrow_mut().extend(seen);
            Ret::default()
        });
        let world = world(&mut e);
        e.set(world, HkpWorld::m_broadPhase, Ptr::new(0x2000));
        e.set(world, HkpWorld::m_collisionFilter, Ptr::new(0x1234));
        e.call(0x00c9_1f70, &args![world, 0x11u32, 0x22u32]);
        let log = calls_to(&e, WORLD_RAY_CASTER_CAST_RAY);
        assert_eq!(log.len(), 1);
        assert_eq!(log[0][1..], [0x2000, 0x11, 0x1234, 0, 0x22]);
        assert_eq!(words.borrow().clone(), vec![RAY_CASTER_VTABLE, 0, 0]);
        // Only one record fits: the end record is dropped.
        assert_eq!(labels(&e, stream, start), vec![PROFILE_CAST_RAY_COLLECTOR]);
    }

    #[test]
    fn cast_ray_ov2_uses_the_tree_slot_0x1c_or_the_simple_caster() {
        let (mut e, stream, start) = profiling_engine(4);
        silent(&mut e, &[FAKE_1]);
        returning(&mut e, FAKE_2, 1);
        let world = world(&mut e);
        let tree = object_with_vtable(&mut e, 0x20, &[(0x10, FAKE_2), (0x1c, FAKE_1)]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
        e.set(world, HkpWorld::m_collisionFilter, Ptr::new(0x1234));
        e.call(0x00c9_2040, &args![world, 0x11u32, 0x22u32]);
        assert_eq!(calls_to(&e, FAKE_1), vec![vec![tree, 0x11, 0x1234, 0x22]]);
        // The tree reports it is not in use: the simple caster runs.
        returning(&mut e, FAKE_2, 0);
        let words: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        let sink = words.clone();
        e.register_double(SIMPLE_RAY_CASTER_CAST_RAY, move |e, a| {
            let caster = a[0];
            let seen = [
                e.mem.u32(caster),
                e.mem.u32(caster + 0x30),
                e.mem.u32(caster + 0x34),
            ];
            sink.borrow_mut().extend(seen);
            Ret::default()
        });
        e.set(world, HkpWorld::m_broadPhase, Ptr::new(0x2000));
        e.call(0x00c9_2040, &args![world, 0x11u32, 0x22u32]);
        let log = calls_to(&e, SIMPLE_RAY_CASTER_CAST_RAY);
        assert_eq!(log[0][1..], [0x2000, 0x11, 0x1234, 0, 0x22]);
        assert_eq!(words.borrow().clone(), vec![SIMPLE_RAY_CASTER_VTABLE, 0, 0]);
        assert_eq!(
            labels(&e, stream, start),
            vec![PROFILE_CAST_RAY, PROFILE_END, PROFILE_CAST_RAY, PROFILE_END]
        );
    }

    #[test]
    fn linear_cast_uses_the_tree_or_a_linear_caster() {
        let (mut e, stream, start) = profiling_engine(8);
        silent(&mut e, &[FAKE_1]);
        returning(&mut e, FAKE_2, 1);
        let world = world(&mut e);
        let collision_input = e.mem.alloc(0x80);
        e.mem.set_u32(collision_input + 0x70, 0x4444);
        e.set(world, HkpWorld::m_collisionInput, Ptr::new(collision_input));
        let filter = e.mem.alloc(0x20);
        e.set(world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        let tree = object_with_vtable(&mut e, 0x20, &[(0x10, FAKE_2), (0x20, FAKE_1)]);
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
        e.call(0x00c9_2430, &args![world, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(
            calls_to(&e, FAKE_1),
            vec![vec![tree, 1, 2, filter + 8, collision_input, 0x4444, 3, 4]]
        );
        // No tree: the linear caster object is built with the default float.
        e.map(0x0104_d000, 0x1000);
        e.set_global(LINEAR_CASTER_DEFAULT, f32::from_bits(0x7f80_0001));
        e.set(world, HkpWorld::m_kdTreeManager, Ptr::NULL);
        e.set(world, HkpWorld::m_broadPhase, Ptr::new(0x2000));
        let words: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(vec![]));
        let sink = words.clone();
        e.register_double(LINEAR_CASTER_LINEAR_CAST, move |e, a| {
            let caster = a[0];
            let seen = [
                e.mem.u32(caster),
                e.mem.u32(caster + 0x24),
                e.mem.u32(caster + 0x80),
            ];
            sink.borrow_mut().extend(seen);
            Ret::default()
        });
        e.call(0x00c9_2430, &args![world, 1u32, 2u32, 3u32, 4u32]);
        let log = calls_to(&e, LINEAR_CASTER_LINEAR_CAST);
        assert_eq!(
            log[0][1..],
            [0x2000, 1, 2, filter + 8, collision_input, 0x4444, 0, 3, 4]
        );
        assert_eq!(
            words.borrow().clone(),
            vec![LINEAR_CASTER_VTABLE, 0, 0x7fc0_0001]
        );
        assert_eq!(
            labels(&e, stream, start),
            vec![
                PROFILE_LINEAR_CAST,
                PROFILE_END,
                PROFILE_LINEAR_CAST,
                PROFILE_END
            ]
        );
        // Without a filter the base passed on is 0.
        e.set(world, HkpWorld::m_collisionFilter, Ptr::NULL);
        e.call(0x00c9_2430, &args![world, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(calls_to(&e, LINEAR_CASTER_LINEAR_CAST)[1][4], 0);
    }

    /// What a job dispatch test observes: the jobs handed to the queue (their bytes at the
    /// time of the call) and the blocks freed.
    struct Dispatch {
        e: Engine,
        jobs: Rc<RefCell<Vec<Vec<u8>>>>,
        frees: Records,
        world: Ptr<HkpWorld>,
        block: u32,
        notifier: u32,
    }

    fn dispatch_engine(job_size: u32) -> Dispatch {
        let mut e = logging();
        let jobs: Rc<RefCell<Vec<Vec<u8>>>> = Rc::new(RefCell::new(vec![]));
        let sink = jobs.clone();
        e.register_double(JOB_QUEUE_ADD_JOB, move |e, words| {
            sink.borrow_mut().push(e.mem.bytes(words[1], job_size));
            Ret::default()
        });
        let block = e.mem.alloc(0x10);
        returning(&mut e, FAKE_1, block);
        let frees = recording(&mut e, FAKE_2);
        let allocator = object_with_vtable(&mut e, 0x20, &[(4, FAKE_1), (8, FAKE_2)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        returning(&mut e, MEMORY_ROUTER, router);
        silent(
            &mut e,
            &[FAKE_3, FAKE_4, SEMAPHORE_ACQUIRE, JOB_HEADER_INIT],
        );
        let notifier = object_with_vtable(&mut e, 0x20, &[(0xc, FAKE_3), (0x10, FAKE_4)]);
        let world = world(&mut e);
        e.set(world, HkpWorld::m_collisionInput, Ptr::new(0x3000));
        e.set(world, HkpWorld::m_broadPhase, Ptr::new(0x4000));
        Dispatch {
            e,
            jobs,
            frees,
            world,
            block,
            notifier,
        }
    }

    /// Gives the world a kd-tree manager that is in use and holds two words.
    fn use_tree(d: &mut Dispatch) {
        returning(&mut d.e, FAKE_5, 1);
        let tree = object_with_vtable(&mut d.e, 0x20, &[(0x10, FAKE_5)]);
        d.e.mem.set_u32(tree + 0xc, 0xaaaa);
        d.e.mem.set_u32(tree + 0x10, 0xbbbb);
        d.e.set(d.world, HkpWorld::m_kdTreeManager, Ptr::new(tree));
    }

    /// The word at `offset` of a recorded job.
    fn job_word(job: &[u8], offset: usize) -> u32 {
        u32::from_le_bytes([
            job[offset],
            job[offset + 1],
            job[offset + 2],
            job[offset + 3],
        ])
    }

    /// The jobs were added, then the notifier and the semaphore waited, then the block freed.
    fn assert_waited(d: &Dispatch, queue: u32, semaphore: u32, job_count: usize) {
        let mut expected = vec![JOB_QUEUE_ADD_JOB; job_count];
        expected.extend([FAKE_3, FAKE_4, SEMAPHORE_ACQUIRE, FAKE_2]);
        let wanted = [JOB_QUEUE_ADD_JOB, FAKE_3, FAKE_4, SEMAPHORE_ACQUIRE, FAKE_2];
        assert_eq!(order_of(&d.e, &wanted), expected);
        assert_eq!(calls_to(&d.e, FAKE_3), vec![vec![d.notifier, queue, 0xb]]);
        assert_eq!(calls_to(&d.e, FAKE_4), vec![vec![d.notifier]]);
        assert_eq!(calls_to(&d.e, SEMAPHORE_ACQUIRE), vec![vec![semaphore]]);
        let frees = d.frees.borrow().clone();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0][1..], [d.block, 0x10]);
        for add in calls_to(&d.e, JOB_QUEUE_ADD_JOB) {
            assert_eq!(add[0], queue);
            assert_eq!(add[2], 1);
        }
    }

    #[test]
    fn fn_00c92110_splits_the_array_into_batched_jobs() {
        let mut d = dispatch_engine(0x50);
        let (queue, semaphore, array) = (0x5000, 0x6000, 0x7000);
        d.e.call(
            0x00c9_2110,
            &args![d.world, array, 130i32, queue, d.notifier, semaphore, 64i32],
        );
        let jobs = d.jobs.borrow().clone();
        assert_eq!(jobs.len(), 3);
        assert_eq!(d.e.mem.u32(d.block), 3);
        assert_eq!(jobs[0][..3], [1, 3, 1]);
        assert_eq!(u16::from_le_bytes([jobs[0][4], jobs[0][5]]), 0x50);
        assert_eq!(u16::from_le_bytes([jobs[0][6], jobs[0][7]]), 0xffff);
        assert_eq!(job_word(&jobs[0], 0x10), semaphore);
        assert_eq!(job_word(&jobs[0], 0x14), d.block);
        assert_eq!(job_word(&jobs[0], 0x1c), 0x3000);
        assert_eq!(job_word(&jobs[0], 0x20), 0x40);
        assert_eq!(job_word(&jobs[0], 0x34), 0x4000);
        let counts: Vec<u32> = jobs.iter().map(|job| job_word(job, 0x28)).collect();
        let arrays: Vec<u32> = jobs.iter().map(|job| job_word(job, 0x24)).collect();
        assert_eq!(counts, vec![64, 64, 2]);
        assert_eq!(arrays, vec![array, array + 0x1000, array + 0x2000]);
        assert_waited(&d, queue, semaphore, 3);
    }

    #[test]
    fn fn_00c92110_hands_the_trees_words_to_the_job() {
        let mut d = dispatch_engine(0x50);
        use_tree(&mut d);
        d.e.call(
            0x00c9_2110,
            &args![d.world, 0x7000u32, 10i32, 0x5000u32, d.notifier, 0x6000u32, 64i32],
        );
        let jobs = d.jobs.borrow().clone();
        assert_eq!(jobs.len(), 1);
        assert_eq!(job_word(&jobs[0], 0x34), 0);
        assert_eq!(job_word(&jobs[0], 0x3c), 0xaaaa);
        assert_eq!(job_word(&jobs[0], 0x40), 0xbbbb);
        assert_eq!(job_word(&jobs[0], 0x28), 10);
    }

    #[test]
    fn fn_00c92110_with_no_elements_adds_no_job() {
        let mut d = dispatch_engine(0x50);
        d.e.call(
            0x00c9_2110,
            &args![d.world, 0x7000u32, 0i32, 0x5000u32, d.notifier, 0x6000u32, 64i32],
        );
        assert!(d.jobs.borrow().is_empty());
        assert_eq!(d.e.mem.u32(d.block), 1);
        assert_waited(&d, 0x5000, 0x6000, 0);
    }

    #[test]
    fn fn_00c92290_clears_the_elements_and_batches_the_jobs() {
        let mut d = dispatch_engine(0x40);
        let collision_input = d.e.mem.alloc(0x20);
        d.e.mem.set_u32(collision_input + 8, 0x7f80_0001);
        d.e.set(
            d.world,
            HkpWorld::m_collisionInput,
            Ptr::new(collision_input),
        );
        let array = d.e.mem.alloc(0x10 * 5);
        for index in 0..5 {
            d.e.mem.set_u32(array + 0xc + 0x10 * index, 0xffff);
        }
        d.e.call(
            0x00c9_2290,
            &args![d.world, array, 5i32, 0x5000u32, d.notifier, 0x6000u32, 2i32],
        );
        for index in 0..5 {
            assert_eq!(d.e.mem.u32(array + 0xc + 0x10 * index), 0);
        }
        let jobs = d.jobs.borrow().clone();
        assert_eq!(jobs.len(), 3);
        assert_eq!(jobs[0][..3], [2, 2, 2]);
        assert_eq!(job_word(&jobs[0], 0x2c), 0x7fc0_0001);
        assert_eq!(job_word(&jobs[0], 0x30), 0x40);
        assert_eq!(job_word(&jobs[0], 0x20), 0x4000);
        assert_eq!(job_word(&jobs[0], 0x18), collision_input);
        let counts: Vec<u32> = jobs.iter().map(|job| job_word(job, 0x38)).collect();
        let arrays: Vec<u32> = jobs.iter().map(|job| job_word(job, 0x34)).collect();
        assert_eq!(counts, vec![2, 2, 1]);
        assert_eq!(arrays, vec![array, array + 0x20, array + 0x40]);
        assert_eq!(d.e.mem.u32(d.block), 1);
        assert_eq!(calls_to(&d.e, JOB_HEADER_INIT).len(), 1);
        assert_waited(&d, 0x5000, 0x6000, 3);
    }

    #[test]
    fn fn_00c92290_hands_the_trees_words_to_the_job() {
        let mut d = dispatch_engine(0x40);
        let collision_input = d.e.mem.alloc(0x20);
        d.e.set(
            d.world,
            HkpWorld::m_collisionInput,
            Ptr::new(collision_input),
        );
        use_tree(&mut d);
        let array = d.e.mem.alloc(0x40);
        d.e.call(
            0x00c9_2290,
            &args![d.world, array, 1i32, 0x5000u32, d.notifier, 0x6000u32, 8i32],
        );
        let jobs = d.jobs.borrow().clone();
        assert_eq!(jobs.len(), 1);
        assert_eq!(job_word(&jobs[0], 0x20), 0);
        assert_eq!(job_word(&jobs[0], 0x24), 0xaaaa);
        assert_eq!(job_word(&jobs[0], 0x28), 0xbbbb);
    }

    #[test]
    fn fn_00c92540_batches_with_a_stride_of_0x30() {
        let mut d = dispatch_engine(0x40);
        let array = 0x7000u32;
        d.e.call(
            0x00c9_2540,
            &args![d.world, array, 5i32, 0x5000u32, d.notifier, 0x6000u32, 2i32],
        );
        let jobs = d.jobs.borrow().clone();
        assert_eq!(jobs.len(), 3);
        assert_eq!(jobs[0][..3], [4, 2, 1]);
        assert_eq!(job_word(&jobs[0], 0x18), 0x3000);
        assert_eq!(job_word(&jobs[0], 0x20), 0x40);
        assert_eq!(job_word(&jobs[0], 0x2c), 0x4000);
        let counts: Vec<u32> = jobs.iter().map(|job| job_word(job, 0x28)).collect();
        let arrays: Vec<u32> = jobs.iter().map(|job| job_word(job, 0x24)).collect();
        assert_eq!(counts, vec![2, 2, 1]);
        assert_eq!(arrays, vec![array, array + 0x60, array + 0xc0]);
        assert_eq!(d.e.mem.u32(d.block), 1);
        assert_waited(&d, 0x5000, 0x6000, 3);
    }

    #[test]
    fn fn_00c92540_hands_the_trees_words_to_the_job() {
        let mut d = dispatch_engine(0x40);
        use_tree(&mut d);
        d.e.call(
            0x00c9_2540,
            &args![d.world, 0x7000u32, 1i32, 0x5000u32, d.notifier, 0x6000u32, 8i32],
        );
        let jobs = d.jobs.borrow().clone();
        assert_eq!(job_word(&jobs[0], 0x2c), 0);
        assert_eq!(job_word(&jobs[0], 0x34), 0xaaaa);
        assert_eq!(job_word(&jobs[0], 0x38), 0xbbbb);
    }

    /// An island whose entity array holds `bodies`; its action array holds `actions`.
    fn make_island(e: &mut Engine, bodies: &[u32], actions: &[u32]) -> u32 {
        let island = e.mem.alloc(0x80);
        let body_array = array_of(e, bodies);
        e.mem.set_u32(island + 0x48, body_array);
        e.mem.set_i32(island + 0x4c, bodies.len() as i32);
        let action_array = array_of(e, actions);
        e.mem.set_u32(island + 0x38, action_array);
        e.mem.set_i32(island + 0x3c, actions.len() as i32);
        island
    }

    fn set_islands(e: &mut Engine, world: Ptr<HkpWorld>, active: &[u32], inactive: &[u32]) {
        let active_array = array_of(e, active);
        e.set(
            world,
            HkpWorld::m_activeSimulationIslands,
            Ptr::new(active_array),
        );
        e.set(
            world,
            HkpWorld::m_activeSimulationIslandsSize,
            active.len() as i32,
        );
        let inactive_array = array_of(e, inactive);
        e.set(
            world,
            HkpWorld::m_inactiveSimulationIslands,
            Ptr::new(inactive_array),
        );
        e.set(
            world,
            HkpWorld::m_inactiveSimulationIslandsSize,
            inactive.len() as i32,
        );
    }

    #[test]
    fn fn_00c926a0_adds_the_fixed_and_inactive_bodies() {
        let mut e = logging();
        silent(&mut e, &[PHYSICS_SYSTEM_ADD_RIGID_BODY]);
        let world = world(&mut e);
        let first = e.mem.alloc(0x20);
        let (second, third) = (0x2222, 0x3333);
        let fixed = make_island(&mut e, &[first, second], &[]);
        let inactive = make_island(&mut e, &[third], &[]);
        e.set(world, HkpWorld::m_fixedIsland, Ptr::new(fixed));
        set_islands(&mut e, world, &[], &[inactive]);
        let out = e.mem.alloc(8);
        let returned = e.call(0x00c9_26a0, &args![out, world, 0x9000u32]).u32();
        assert_eq!(returned, out);
        assert_eq!(e.mem.u8(out), 1);
        // The first fixed body has no word at +0x10: it is skipped.
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_RIGID_BODY),
            vec![vec![0x9000, second], vec![0x9000, third]]
        );
        // With a word there it is added too; without any island the flag is clear.
        e.mem.set_u32(first + 0x10, 1);
        e.call(0x00c9_26a0, &args![out, world, 0x9000u32]);
        assert_eq!(calls_to(&e, PHYSICS_SYSTEM_ADD_RIGID_BODY).len(), 5);
        e.set(world, HkpWorld::m_fixedIsland, Ptr::NULL);
        set_islands(&mut e, world, &[], &[]);
        e.call(0x00c9_26a0, &args![out, world, 0x9000u32]);
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(calls_to(&e, PHYSICS_SYSTEM_ADD_RIGID_BODY).len(), 5);
    }

    #[test]
    fn fn_00c92760_adds_the_active_bodies() {
        let mut e = logging();
        silent(&mut e, &[PHYSICS_SYSTEM_ADD_RIGID_BODY]);
        let world = world(&mut e);
        let one = make_island(&mut e, &[0x11, 0x22], &[]);
        let two = make_island(&mut e, &[0x33], &[]);
        set_islands(&mut e, world, &[one, two], &[]);
        let out = e.mem.alloc(8);
        assert_eq!(
            e.call(0x00c9_2760, &args![out, world, 0x9000u32]).u32(),
            out
        );
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_RIGID_BODY),
            vec![vec![0x9000, 0x11], vec![0x9000, 0x22], vec![0x9000, 0x33]]
        );
        set_islands(&mut e, world, &[], &[]);
        e.call(0x00c9_2760, &args![out, world, 0x9000u32]);
        assert_eq!(e.mem.u8(out), 0);
    }

    #[test]
    fn fn_00c927c0_stores_the_active_flag_in_the_system() {
        let mut e = logging();
        silent(&mut e, &[PHYSICS_SYSTEM_ADD_RIGID_BODY]);
        let world = world(&mut e);
        let active = make_island(&mut e, &[0x11], &[]);
        set_islands(&mut e, world, &[active], &[]);
        let system = Ptr::new(e.mem.alloc(0x50));
        fn_00c927c0(&mut e, system, world);
        assert_eq!(e.mem.u8(system.addr() + 0x40), 1);
        set_islands(&mut e, world, &[], &[]);
        fn_00c927c0(&mut e, system, world);
        assert_eq!(e.mem.u8(system.addr() + 0x40), 0);
    }

    /// Writes a constraint entry at `entry` whose first word is `first` and whose chain holds
    /// nodes of the given kinds, linked through the word at +0x14.
    fn constraint_entry(e: &mut Engine, entry: u32, first: u32, kinds: &[u16]) {
        e.mem.set_u32(entry, first);
        let mut link = 0;
        for &kind in kinds.iter().rev() {
            let node = e.mem.alloc(0x20);
            e.mem.set_u16(node, kind);
            e.mem.set_u32(node + 0x14, link);
            link = node;
        }
        e.mem.set_u32(entry + 0xc, link);
    }

    #[test]
    fn fn_00c927f0_adds_the_constraints_that_do_not_end_in_0x16() {
        let mut e = logging();
        silent(&mut e, &[PHYSICS_SYSTEM_ADD_CONSTRAINT]);
        let entity = e.mem.alloc(0xc0);
        let entries = e.mem.alloc(0x30 * 4);
        constraint_entry(&mut e, entries, 0xa1, &[0x10]);
        constraint_entry(&mut e, entries + 0x30, 0xa2, &[0x16]);
        constraint_entry(&mut e, entries + 0x60, 0xa3, &[0x20, 0x16]);
        constraint_entry(&mut e, entries + 0x90, 0xa4, &[0x20, 0x21, 0x15]);
        e.mem.set_u32(entity + 0xac, entries);
        e.mem.set_u16(entity + 0xb0, 4);
        let island = make_island(&mut e, &[entity], &[]);
        e.call(0x00c9_27f0, &args![island, 0x9000u32]);
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_CONSTRAINT),
            vec![vec![0x9000, 0xa1], vec![0x9000, 0xa4]]
        );
        // An entity without constraints adds none.
        e.mem.set_u16(entity + 0xb0, 0);
        e.call(0x00c9_27f0, &args![island, 0x9000u32]);
        assert_eq!(calls_to(&e, PHYSICS_SYSTEM_ADD_CONSTRAINT).len(), 2);
    }

    /// An island holding one entity with one constraint entry whose first word is `first`.
    fn island_with_constraint(e: &mut Engine, first: u32) -> u32 {
        let entity = e.mem.alloc(0xc0);
        let entry = e.mem.alloc(0x30);
        constraint_entry(e, entry, first, &[0x10]);
        e.mem.set_u32(entity + 0xac, entry);
        e.mem.set_u16(entity + 0xb0, 1);
        make_island(e, &[entity], &[])
    }

    #[test]
    fn fn_00c92860_visits_active_then_inactive_islands() {
        let mut e = logging();
        silent(&mut e, &[PHYSICS_SYSTEM_ADD_CONSTRAINT]);
        let world = world(&mut e);
        let active = island_with_constraint(&mut e, 0xb1);
        let inactive = island_with_constraint(&mut e, 0xb2);
        set_islands(&mut e, world, &[active], &[inactive]);
        fn_00c92860(&mut e, world, Ptr::new(0x9000));
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_CONSTRAINT),
            vec![vec![0x9000, 0xb1], vec![0x9000, 0xb2]]
        );
    }

    #[test]
    fn fn_00c928c0_adds_the_actions_of_every_island() {
        let mut e = logging();
        silent(&mut e, &[PHYSICS_SYSTEM_ADD_ACTION]);
        let world = world(&mut e);
        let active = make_island(&mut e, &[], &[0xc1, 0xc2]);
        let inactive = make_island(&mut e, &[], &[0xc3]);
        set_islands(&mut e, world, &[active], &[inactive]);
        e.call(0x00c9_28c0, &args![world, 0x9000u32]);
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_ACTION),
            vec![vec![0x9000, 0xc1], vec![0x9000, 0xc2], vec![0x9000, 0xc3]]
        );
    }

    #[test]
    fn fn_00c92950_skips_the_borders_phantoms() {
        let mut e = logging();
        silent(&mut e, &[PHYSICS_SYSTEM_ADD_PHANTOM]);
        let world = world(&mut e);
        let phantoms = array_of(&mut e, &[0x11, 0x22, 0x33]);
        e.set(world, HkpWorld::m_phantoms, Ptr::new(phantoms));
        e.set(world, HkpWorld::m_phantomsSize, 3);
        fn_00c92950(&mut e, world, Ptr::new(0x9000));
        assert_eq!(calls_to(&e, PHYSICS_SYSTEM_ADD_PHANTOM).len(), 3);
        let border = e.mem.alloc(0x40);
        e.mem.set_u32(border + 0x18, 0x22);
        e.mem.set_u32(border + 0x2c, 0x33);
        e.set(world, HkpWorld::m_broadPhaseBorder, Ptr::new(border));
        fn_00c92950(&mut e, world, Ptr::new(0x9000));
        let log = calls_to(&e, PHYSICS_SYSTEM_ADD_PHANTOM);
        assert_eq!(log.len(), 4);
        assert_eq!(log[3], vec![0x9000, 0x11]);
    }

    #[test]
    fn fn_00c929e0_builds_a_system_from_the_world() {
        let mut e = logging();
        let allocated = e.mem.alloc(0x44);
        returning(&mut e, FAKE_1, allocated);
        let allocator = object_with_vtable(&mut e, 0x20, &[(4, FAKE_1)]);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        returning(&mut e, MEMORY_ROUTER, router);
        let system = e.mem.alloc(0x50);
        returning(&mut e, PHYSICS_SYSTEM_CONSTRUCT, system);
        silent(
            &mut e,
            &[
                PHYSICS_SYSTEM_ADD_RIGID_BODY,
                PHYSICS_SYSTEM_ADD_CONSTRAINT,
                PHYSICS_SYSTEM_ADD_ACTION,
                PHYSICS_SYSTEM_ADD_PHANTOM,
            ],
        );
        let world = world(&mut e);
        let body_a = e.mem.alloc(0xc0);
        let body_b = e.mem.alloc(0xc0);
        let active = make_island(&mut e, &[body_a], &[0x21]);
        let inactive = make_island(&mut e, &[body_b], &[0x22]);
        set_islands(&mut e, world, &[active], &[inactive]);
        let phantoms = array_of(&mut e, &[0x31]);
        e.set(world, HkpWorld::m_phantoms, Ptr::new(phantoms));
        e.set(world, HkpWorld::m_phantomsSize, 1);
        let returned = e.call(0x00c9_29e0, &args![world]).u32();
        assert_eq!(returned, system);
        assert_eq!(e.mem.u16(allocated + 4), 0x44);
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_CONSTRUCT),
            vec![vec![allocated]]
        );
        assert_eq!(e.mem.u8(system + 0x40), 1);
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_RIGID_BODY),
            vec![vec![system, body_b], vec![system, body_a]]
        );
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_PHANTOM),
            vec![vec![system, 0x31]]
        );
        assert_eq!(
            calls_to(&e, PHYSICS_SYSTEM_ADD_ACTION),
            vec![vec![system, 0x21], vec![system, 0x22]]
        );
        let sequence = [
            PHYSICS_SYSTEM_CONSTRUCT,
            PHYSICS_SYSTEM_ADD_RIGID_BODY,
            PHYSICS_SYSTEM_ADD_PHANTOM,
            PHYSICS_SYSTEM_ADD_ACTION,
        ];
        assert_eq!(
            order_of(&e, &sequence),
            vec![
                PHYSICS_SYSTEM_CONSTRUCT,
                PHYSICS_SYSTEM_ADD_RIGID_BODY,
                PHYSICS_SYSTEM_ADD_RIGID_BODY,
                PHYSICS_SYSTEM_ADD_PHANTOM,
                PHYSICS_SYSTEM_ADD_ACTION,
                PHYSICS_SYSTEM_ADD_ACTION,
            ]
        );
    }

    // -----------------------------------------------------------------------------------
    // Phantoms, batches, actions, constraints and collision filter updates.
    // -----------------------------------------------------------------------------------

    type CallLog = Rc<RefCell<Vec<(u32, u32)>>>;

    /// The memory router with a bump area and two allocators that log what is handed back,
    /// the array growth callees doubled to really grow, a profiling stream and a world.
    struct Rig {
        e: Engine,
        world: Ptr<HkpWorld>,
        stream: u32,
        area: u32,
        router: u32,
        freed_first: Records,
        freed_second: Records,
    }

    fn rig() -> Rig {
        let (mut e, stream, area) = profiling_engine(40);
        e.map(0x0101_1000, 0x1000);
        e.set_global(HALF, 0.5f64);
        let freed_first = recording(&mut e, FAKE_4);
        let freed_second = recording(&mut e, FAKE_5);
        e.register(FAKE_1, |e, words| Ret {
            eax: e.mem.alloc(words[1].max(8)),
            ..Ret::default()
        });
        let first = object_with_vtable(&mut e, 0x20, &[(4, FAKE_1), (0x10, FAKE_4)]);
        let second = object_with_vtable(&mut e, 0x20, &[(4, FAKE_1), (0x10, FAKE_5)]);
        let cursor = e.mem.alloc(0x4000);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router, cursor);
        e.mem.set_u32(router + 0xc, second);
        e.mem.set_u32(router + 0x10, first);
        returning(&mut e, MEMORY_ROUTER, router);
        e.register(ARRAY_GROW, |e, words| {
            let (array, element) = (words[1], words[2]);
            let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff).max(2) * 2;
            let block = e.mem.alloc(capacity * element);
            let count = e.mem.u32(array + 4) * element;
            if count != 0 {
                let old = e.mem.bytes(e.mem.u32(array), count);
                e.mem.write(block, &old);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, capacity);
            Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, words| {
            let (array, count, element) = (words[1], words[2], words[3]);
            let block = e.mem.alloc((count * element).max(8));
            let used = e.mem.u32(array + 4) * element;
            if used != 0 {
                let old = e.mem.bytes(e.mem.u32(array), used);
                e.mem.write(block, &old);
            }
            e.mem.set_u32(array, block);
            e.mem.set_u32(array + 8, count);
            Ret::default()
        });
        silent(&mut e, &[QUEUE_FLUSH_PENDING, QUEUE_FLUSH_BODY_PENDING]);
        let world = world(&mut e);
        Rig {
            e,
            world,
            stream,
            area,
            router,
            freed_first,
            freed_second,
        }
    }

    /// The labels of the profiling records written so far; the records of the labels in
    /// `wide` are 16 bytes long, the others 12.
    fn record_labels(rig: &Rig, wide: &[u32]) -> Vec<u32> {
        let cursor = rig.e.mem.u32(rig.stream + 4);
        let mut labels = vec![];
        let mut at = rig.area;
        while at < cursor {
            let label = rig.e.mem.u32(at);
            labels.push(label);
            at += if wide.contains(&label) { 16 } else { 12 };
        }
        labels
    }

    fn lock(rig: &mut Rig, count: i32) {
        rig.e
            .set(rig.world, HkpWorld::m_criticalOperationsLockCount, count);
    }

    /// An object whose vtable slots 0x1c / 0x20 / 0x24 are the overlap callbacks of a
    /// phantom: slot 0x20 reports `was` in the byte it is given, the others log
    /// `(slot, this)`.
    fn overlap_object(e: &mut Engine, log: &CallLog, was: u8, base: u32) -> u32 {
        e.register_double(base, move |e, words| {
            e.mem.set_u8(words[1], was);
            Ret::default()
        });
        for (slot, offset) in [(0x1cu32, 1u32), (0x24, 2)] {
            let sink = log.clone();
            e.register_double(base + offset, move |_, words| {
                sink.borrow_mut().push((slot, words[0]));
                Ret::default()
            });
        }
        object_with_vtable(
            e,
            0x240,
            &[(0x20, base), (0x1c, base + 1), (0x24, base + 2)],
        )
    }

    /// A filter object whose pair test answers `accepts`.
    fn filter_answering(e: &mut Engine, accepts: u8) -> u32 {
        let answer = e.mem.alloc(4);
        e.mem.set_u8(answer, accepts);
        e.register_double(FAKE_3, move |_, _| Ret {
            eax: answer,
            ..Ret::default()
        });
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table + 4, FAKE_3);
        let filter = e.mem.alloc(0x20);
        e.mem.set_u32(filter + 8, table);
        filter
    }

    /// Runs the outlined filter piece; the log is `(slot, 1 for the phantom or 2 for the
    /// other body)`.
    fn run_overlap_piece(was: u8, accepts: u8, other_was: Option<u8>) -> Vec<(u32, u32)> {
        let mut e = Engine::new();
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        let phantom = overlap_object(&mut e, &log, was, 0x0900_0010);
        let other = overlap_object(&mut e, &log, other_was.unwrap_or(0), 0x0900_0020);
        e.mem
            .set_u8(other + 0x18, if other_was.is_some() { 2 } else { 1 });
        let filter = filter_answering(&mut e, accepts);
        fn_00c90f40(&mut e, phantom, other, filter);
        let entries = log.borrow().clone();
        entries
            .into_iter()
            .map(|(slot, this)| (slot, if this == phantom { 1 } else { 2 }))
            .collect()
    }

    #[test]
    fn fn_00c90f40_tells_the_phantom_when_the_filter_changes_its_mind() {
        assert_eq!(run_overlap_piece(0, 1, None), vec![(0x1c, 1)]);
        assert_eq!(run_overlap_piece(1, 1, None), vec![]);
        assert_eq!(run_overlap_piece(1, 0, None), vec![(0x24, 1)]);
        assert_eq!(run_overlap_piece(0, 0, None), vec![]);
    }

    #[test]
    fn fn_00c90f40_tells_a_phantom_on_the_other_side_too() {
        assert_eq!(run_overlap_piece(0, 1, Some(0)), vec![(0x1c, 1), (0x1c, 2)]);
        assert_eq!(run_overlap_piece(1, 1, Some(1)), vec![]);
        assert_eq!(run_overlap_piece(0, 0, Some(1)), vec![(0x24, 2)]);
        assert_eq!(run_overlap_piece(1, 0, Some(0)), vec![(0x24, 1)]);
    }

    /// A phantom object (size 0x40) with a deletion callback (slot 0x34) and the motion
    /// maker (slot 0x10) that log themselves.
    fn phantom_object(e: &mut Engine, log: &CallLog) -> u32 {
        let sink = log.clone();
        e.register_double(0x0900_0030, move |_, words| {
            sink.borrow_mut().push((0x34, words[0]));
            Ret::default()
        });
        let sink = log.clone();
        e.register_double(0x0900_0031, move |_, words| {
            sink.borrow_mut().push((0x10, words[0]));
            Ret {
                eax: 0x7777,
                ..Ret::default()
            }
        });
        let sink = log.clone();
        e.register_double(0x0900_0032, move |e, words| {
            e.mem.set_u32(words[1], 0xb0b0_0000 + words[0]);
            sink.borrow_mut().push((0x18, words[0]));
            Ret::default()
        });
        object_with_vtable(
            e,
            0x40,
            &[
                (0x34, 0x0900_0030),
                (0x10, 0x0900_0031),
                (0x18, 0x0900_0032),
            ],
        )
    }

    fn set_phantoms(rig: &mut Rig, phantoms: &[u32]) {
        let data = array_of(&mut rig.e, phantoms);
        rig.e.set(rig.world, HkpWorld::m_phantoms, Ptr::new(data));
        rig.e
            .set(rig.world, HkpWorld::m_phantomsSize, phantoms.len() as i32);
        rig.e
            .mem
            .set_u32(rig.world.addr() + 0x118, phantoms.len() as u32);
    }

    fn phantoms_of(rig: &Rig) -> Vec<u32> {
        let data = rig.e.mem.u32(rig.world.addr() + 0x110);
        (0..rig.e.mem.u32(rig.world.addr() + 0x114))
            .map(|i| rig.e.mem.u32(data + 4 * i))
            .collect()
    }

    const PHANTOM_CALLEES: [u32; 7] = [
        FIRE_PHANTOM_ADDED,
        FIRE_PHANTOM_REMOVED,
        ADD_PHANTOM_BP,
        REMOVE_PHANTOM_BP,
        PHANTOM_AFTER_ADD,
        PHANTOM_BEFORE_REMOVE,
        ADD_REFERENCE,
    ];

    #[test]
    fn remove_phantom_runs_the_whole_sequence() {
        let mut rig = rig();
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        silent(&mut rig.e, &PHANTOM_CALLEES);
        silent(&mut rig.e, &[REMOVE_REFERENCE]);
        let phantom = phantom_object(&mut rig.e, &log);
        set_phantoms(&mut rig, &[phantom, 0x21, 0x22]);
        rig.e.mem.set_u32(phantom + 8, 0x5555);
        rig.e.set(rig.world, HkpWorld::m_pendingOperationsCount, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.call(0x00c9_2c70, &args![rig.world, phantom]);
        let world = rig.world.addr();
        assert_eq!(phantoms_of(&rig), vec![0x22, 0x21]);
        assert_eq!(rig.e.mem.u32(phantom + 8), 0);
        assert_eq!(log.borrow().clone(), vec![(0x34, phantom)]);
        assert_eq!(
            order_of(
                &rig.e,
                &[
                    FIRE_PHANTOM_REMOVED,
                    PHANTOM_BEFORE_REMOVE,
                    REMOVE_PHANTOM_BP,
                    REMOVE_REFERENCE,
                    QUEUE_FLUSH_PENDING
                ]
            ),
            vec![
                FIRE_PHANTOM_REMOVED,
                PHANTOM_BEFORE_REMOVE,
                REMOVE_PHANTOM_BP,
                REMOVE_REFERENCE,
                QUEUE_FLUSH_PENDING
            ]
        );
        assert_eq!(
            calls_to(&rig.e, FIRE_PHANTOM_REMOVED),
            vec![vec![world, phantom]]
        );
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn remove_phantom_keeps_a_phantom_that_still_has_a_size_and_missing_ones_index_minus_one() {
        let mut rig = rig();
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        silent(&mut rig.e, &PHANTOM_CALLEES);
        silent(&mut rig.e, &[REMOVE_REFERENCE]);
        let phantom = phantom_object(&mut rig.e, &log);
        rig.e.mem.set_u16(phantom + 4, 0x40);
        set_phantoms(&mut rig, &[0x21, phantom]);
        rig.e.call(0x00c9_2c70, &args![rig.world, phantom]);
        assert_eq!(phantoms_of(&rig), vec![0x21]);
        assert!(log.borrow().is_empty());
        // Not in the list: the last entry is copied over the word before the data.
        let data = rig.e.mem.u32(rig.world.addr() + 0x110);
        rig.e.call(0x00c9_2c70, &args![rig.world, phantom]);
        assert_eq!(rig.e.mem.u32(rig.world.addr() + 0x114), 0);
        assert_eq!(rig.e.mem.u32(data - 4), 0x21);
    }

    #[test]
    fn phantom_requests_are_queued_while_either_lock_is_held() {
        for (critical, phantom_lock) in [(1, 0), (0, 1)] {
            let mut rig = rig();
            let queued = operation_recorder(&mut rig.e);
            rig.e
                .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
            lock(&mut rig, critical);
            rig.e.set(
                rig.world,
                HkpWorld::m_phantomOperationsLockCount,
                phantom_lock,
            );
            rig.e.call(0x00c9_2c70, &args![rig.world, 0x1234u32]);
            let added = rig.e.call(0x00c9_36a0, &args![rig.world, 0x1235u32]).u32();
            assert_eq!(added, 0);
            let batch = op_bytes(0xf, 0x1236, 3, 0);
            rig.e.call(0x00c9_54e0, &args![rig.world, 0x1236u32, 3i32]);
            rig.e.call(0x00c9_5800, &args![rig.world, 0x1237u32, 4i32]);
            assert_eq!(
                queued.borrow().clone(),
                vec![
                    (0x5000, op_bytes(0xe, 0x1234, 0, 0)),
                    (0x5000, op_bytes(0xd, 0x1235, 0, 0)),
                    (0x5000, batch),
                    (0x5000, op_bytes(0x10, 0x1237, 4, 0)),
                ]
            );
        }
    }

    #[test]
    fn add_phantom_runs_the_whole_sequence() {
        let mut rig = rig();
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        silent(&mut rig.e, &PHANTOM_CALLEES);
        let phantom = phantom_object(&mut rig.e, &log);
        set_phantoms(&mut rig, &[0x21]);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.set(rig.world, HkpWorld::m_pendingOperationsCount, 2);
        let world = rig.world.addr();
        let returned = rig.e.call(0x00c9_36a0, &args![rig.world, phantom]).u32();
        assert_eq!(returned, phantom);
        assert_eq!(rig.e.mem.u32(phantom + 0x18), 0x7777);
        assert_eq!(rig.e.mem.u32(phantom + 8), world);
        assert_eq!(phantoms_of(&rig), vec![0x21, phantom]);
        assert_eq!(
            order_of(
                &rig.e,
                &[
                    ADD_REFERENCE,
                    ADD_PHANTOM_BP,
                    FIRE_PHANTOM_ADDED,
                    PHANTOM_AFTER_ADD,
                    QUEUE_FLUSH_PENDING
                ]
            ),
            vec![
                ADD_REFERENCE,
                ADD_PHANTOM_BP,
                FIRE_PHANTOM_ADDED,
                PHANTOM_AFTER_ADD,
                QUEUE_FLUSH_PENDING
            ]
        );
        assert_eq!(calls_to(&rig.e, ADD_PHANTOM_BP), vec![vec![world, phantom]]);
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
        // A phantom that already has its object keeps it.
        let second = phantom_object(&mut rig.e, &log);
        rig.e.mem.set_u32(second + 0x18, 0x4444);
        rig.e.call(0x00c9_36a0, &args![rig.world, second]);
        assert_eq!(rig.e.mem.u32(second + 0x18), 0x4444);
    }

    /// A broad phase whose slot `slot` stores the `(unused, collidable)` pairs into the array
    /// it is handed (its last argument); returns the object.
    fn broad_phase_finding(e: &mut Engine, slot: u32, collidables: &[u32]) -> u32 {
        let found = collidables.to_vec();
        e.register_double(0x0900_0040 + slot, move |e, words| {
            let array = *words.last().unwrap();
            let data = e.mem.alloc(8 * found.len().max(1) as u32);
            for (i, collidable) in found.iter().enumerate() {
                e.mem.set_u32(data + 8 * i as u32 + 4, *collidable);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 4, found.len() as u32);
            Ret::default()
        });
        object_with_vtable(e, 0x20, &[(slot, 0x0900_0040 + slot)])
    }

    /// A collidable (size 0x40) at +0x10 of an owner: the collidable's byte +5 is the offset
    /// back to the owner; returns `(collidable, owner)`.
    fn collidable_in(e: &mut Engine, kind: u8, to_body: i8) -> (u32, u32) {
        let owner = e.mem.alloc(0x240);
        let collidable = owner + 0x10;
        e.mem.set_i8(collidable + 5, -0x10);
        e.mem.set_u8(owner + 0x18, kind);
        e.mem.set_i8(owner + 0x10, to_body);
        (collidable, owner)
    }

    #[test]
    fn activate_region_activates_the_rigid_bodies_found() {
        let mut rig = rig();
        let activated = recording(&mut rig.e, ACTIVATE_BODY);
        let (rigid, rigid_owner) = collidable_in(&mut rig.e, 1, 0);
        let (phantom, _) = collidable_in(&mut rig.e, 2, 0);
        let (shifted, shifted_owner) = collidable_in(&mut rig.e, 1, 0x20);
        let broad_phase = broad_phase_finding(&mut rig.e, 0x44, &[rigid, phantom, shifted]);
        rig.e
            .set(rig.world, HkpWorld::m_broadPhase, Ptr::new(broad_phase));
        let aabb = rig.e.mem.alloc(0x20);
        rig.e.call(0x00c9_3c30, &args![rig.world, aabb]);
        assert_eq!(
            activated.borrow().clone(),
            vec![vec![rigid_owner], vec![shifted_owner + 0x20]]
        );
        let query = calls_to(&rig.e, 0x0900_0084);
        assert_eq!(query.len(), 1);
        assert_eq!(query[0][..2], [broad_phase, aabb]);
    }

    #[test]
    fn activate_region_copies_the_aabb_when_queued() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e.register(MEM_COPY, |e, words| {
            let bytes = e.mem.bytes(words[1], words[2]);
            e.mem.write(words[0], &bytes);
            Ret::default()
        });
        lock(&mut rig, 1);
        let aabb = rig.e.mem.alloc(0x20);
        rig.e.mem.set_u32(aabb + 0x1c, 0xfeed);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.call(0x00c9_3c30, &args![rig.world, aabb]);
        let queued = queued.borrow().clone();
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0].1[0], 0x20);
        let copy = u32::from_le_bytes(queued[0].1[4..8].try_into().unwrap());
        assert_ne!(copy, aabb);
        assert_eq!(rig.e.mem.u32(copy + 0x1c), 0xfeed);
        assert_eq!(calls_to(&rig.e, MEM_COPY), vec![vec![copy, aabb, 0x20]]);
    }

    /// A phantom the world updates its filter on: slot 0x30 logs, slots 0x1c / 0x20 / 0x24
    /// are the overlap callbacks.
    fn filtered_phantom(e: &mut Engine, log: &CallLog) -> u32 {
        let phantom = overlap_object(e, log, 0, 0x0900_0010);
        let sink = log.clone();
        e.register_double(0x0900_0013, move |_, words| {
            sink.borrow_mut().push((0x30, words[0]));
            Ret::default()
        });
        let table = e.mem.u32(phantom);
        e.mem.set_u32(table + 0x30, 0x0900_0013);
        phantom
    }

    #[test]
    fn update_filter_on_phantom_asks_every_other_overlapping_body() {
        let mut rig = rig();
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        let phantom = filtered_phantom(&mut rig.e, &log);
        let own = phantom + 0x24;
        let (other, _) = collidable_in(&mut rig.e, 1, 0);
        let broad_phase = broad_phase_finding(&mut rig.e, 0x48, &[own, other]);
        rig.e
            .set(rig.world, HkpWorld::m_broadPhase, Ptr::new(broad_phase));
        let filter = filter_answering(&mut rig.e, 1);
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        rig.e.mem.set_u32(rig.world.addr() + 0x338, 8);
        let cursor = rig.e.mem.u32(rig.router);
        rig.e.call(0x00c9_44a0, &args![rig.world, phantom, 1u32]);
        // The phantom itself is skipped; the other body made it overlap, and the flag
        // calls slot 0x30.
        assert_eq!(log.borrow().clone(), vec![(0x1c, phantom), (0x30, phantom)]);
        let query = calls_to(&rig.e, 0x0900_0088);
        assert_eq!(query.len(), 1);
        assert_eq!(query[0][..2], [broad_phase, own]);
        assert_eq!(
            record_labels(&rig, &[PROFILE_UPDATE_FILTER_ON_PHANTOM]),
            vec![
                PROFILE_UPDATE_FILTER_ON_PHANTOM,
                PROFILE_UPDATE_OVERLAPS,
                PROFILE_COLLECTION_FILTER,
                PROFILE_SECTION_END
            ]
        );
        assert_eq!(rig.e.mem.u32(rig.world.addr() + 0x94), 0);
        assert_eq!(rig.e.mem.u32(rig.router), cursor);
    }

    #[test]
    fn update_filter_on_phantom_without_flag_skips_slot_0x30_and_queues_when_locked() {
        let mut rig = rig();
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        let phantom = filtered_phantom(&mut rig.e, &log);
        let broad_phase = broad_phase_finding(&mut rig.e, 0x48, &[]);
        rig.e
            .set(rig.world, HkpWorld::m_broadPhase, Ptr::new(broad_phase));
        rig.e.call(0x00c9_44a0, &args![rig.world, phantom, 0u32]);
        assert!(log.borrow().is_empty());
        assert_eq!(
            record_labels(&rig, &[PROFILE_UPDATE_FILTER_ON_PHANTOM]),
            vec![
                PROFILE_UPDATE_FILTER_ON_PHANTOM,
                PROFILE_UPDATE_OVERLAPS,
                PROFILE_SECTION_END
            ]
        );
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        rig.e.call(0x00c9_44a0, &args![rig.world, phantom, 1u32]);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(0x14, phantom, 1, 0))]
        );
    }

    /// An entity (size 0x240) of the given type and collision family.
    fn filtered_entity(e: &mut Engine, kind: u8, family: i8) -> u32 {
        let entity = e.mem.alloc(0x240);
        e.mem.set_u8(entity + 0xe8, kind);
        e.mem.set_i8(entity + 0x2a, family);
        entity
    }

    const ENTITY_FILTER_CALLEES: [u32; 5] = [
        COLLIDABLE_RESET_AGENTS,
        REMOVE_DUPLICATE_PAIRS,
        UPDATE_SHAPE_COLLECTION_FILTER,
        ADD_PAIRS,
        QUEUE_FLUSH_PENDING,
    ];

    /// The `hkArray` of agent entries `(entry, partner collidable)` that `00d081e0` answers
    /// with; returns its address. `00cc1160` takes the entry it is given out of it, like the
    /// real one.
    fn agent_entries(e: &mut Engine, pairs: &[(u32, u32)]) -> u32 {
        let data = e.mem.alloc(8 * pairs.len().max(1) as u32);
        for (i, (entry, partner)) in pairs.iter().enumerate() {
            e.mem.set_u32(data + 8 * i as u32, *entry);
            e.mem.set_u32(data + 8 * i as u32 + 4, *partner);
        }
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, pairs.len() as u32);
        returning(e, COLLIDABLE_AGENT_ENTRIES, array);
        e.register_double(REMOVE_AGENT, move |e, words| {
            let data = e.mem.u32(array);
            let size = e.mem.u32(array + 4);
            let at = (0..size).find(|i| e.mem.u32(data + 8 * i) == words[0]);
            if let Some(at) = at {
                for i in at..size - 1 {
                    let next = e.mem.bytes(data + 8 * (i + 1), 8);
                    e.mem.write(data + 8 * i, &next);
                }
                e.mem.set_u32(array + 4, size - 1);
            }
            Ret::default()
        });
        array
    }

    /// A collision table of the world (a byte per group pair at +0x1bb0) with the given
    /// pairs enabled.
    fn group_table(rig: &mut Rig, enabled: &[usize]) {
        let table = rig.e.mem.alloc(0x2000);
        for index in enabled {
            rig.e.mem.set_u8(table + 0x1bb0 + *index as u32, 1);
        }
        rig.e
            .set(rig.world, HkpWorld::m_collisionDispatcher, Ptr::new(table));
    }

    #[test]
    fn update_filter_on_entity_removes_refused_agents_and_updates_the_others() {
        let mut rig = rig();
        silent(&mut rig.e, &ENTITY_FILTER_CALLEES);
        let entity = filtered_entity(&mut rig.e, 1, 2);
        let island = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(entity + 0xcc, island);
        rig.e.mem.set_u8(island + 0x25, 0x06);
        // Partner groups 1 and 3: the table allows (family 2, group 3) only.
        let partner_a = rig.e.mem.alloc(0x40);
        rig.e.mem.set_i8(partner_a + 0x1a, 1);
        let partner_b = rig.e.mem.alloc(0x40);
        rig.e.mem.set_i8(partner_b + 0x1a, 3);
        agent_entries(&mut rig.e, &[(0x601, partner_a), (0x602, partner_b)]);
        group_table(&mut rig, &[23]);
        let filter = filter_answering(&mut rig.e, 1);
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(0x7000));
        rig.e
            .call(0x00c9_46e0, &args![rig.world, entity, 1i32, 1u32]);
        assert_eq!(calls_to(&rig.e, REMOVE_AGENT), vec![vec![0x601]]);
        assert_eq!(
            calls_to(&rig.e, UPDATE_SHAPE_COLLECTION_FILTER),
            vec![vec![0x602, 0x7000, island]]
        );
        assert_eq!(rig.e.mem.u8(island + 0x25), 0x05);
        assert!(calls_to(&rig.e, ADD_PAIRS).is_empty());
        assert!(calls_to(&rig.e, COLLIDABLE_RESET_AGENTS).is_empty());
        assert_eq!(
            record_labels(&rig, &[PROFILE_UPDATE_FILTER_ON_ENTITY]),
            vec![
                PROFILE_UPDATE_FILTER_ON_ENTITY,
                PROFILE_CHECK_AGENTS,
                PROFILE_SECTION_END
            ]
        );
    }

    #[test]
    fn update_filter_on_entity_refuses_by_the_filter_and_resets_type_5() {
        let mut rig = rig();
        silent(&mut rig.e, &ENTITY_FILTER_CALLEES);
        let entity = filtered_entity(&mut rig.e, 5, 2);
        let island = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(entity + 0xcc, island);
        let partner = rig.e.mem.alloc(0x40);
        rig.e.mem.set_i8(partner + 0x1a, 3);
        agent_entries(&mut rig.e, &[(0x601, partner)]);
        group_table(&mut rig, &[23]);
        let filter = filter_answering(&mut rig.e, 0);
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        rig.e
            .call(0x00c9_46e0, &args![rig.world, entity, 1i32, 0u32]);
        assert_eq!(calls_to(&rig.e, REMOVE_AGENT), vec![vec![0x601]]);
        assert_eq!(
            calls_to(&rig.e, COLLIDABLE_RESET_AGENTS),
            vec![vec![entity + 0x10]]
        );
        assert!(calls_to(&rig.e, UPDATE_SHAPE_COLLECTION_FILTER).is_empty());
    }

    #[test]
    fn update_filter_on_entity_updates_a_type_5_entity_in_its_partners_island() {
        let mut rig = rig();
        silent(&mut rig.e, &ENTITY_FILTER_CALLEES);
        let entity = filtered_entity(&mut rig.e, 5, 2);
        let partner_island = rig.e.mem.alloc(0x40);
        let partner_owner = rig.e.mem.alloc(0x240);
        rig.e.mem.set_u32(partner_owner + 0xcc, partner_island);
        // The partner collidable is at +0x10 of its owner: byte +0x10 of the partner points
        // back to the owner (offset -0x10).
        let partner = partner_owner + 0x10;
        rig.e.mem.set_i8(partner + 0x1a, 3);
        rig.e.mem.set_i8(partner + 0x10, -0x10);
        agent_entries(&mut rig.e, &[(0x601, partner)]);
        group_table(&mut rig, &[23]);
        let filter = filter_answering(&mut rig.e, 1);
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(0x7000));
        rig.e
            .call(0x00c9_46e0, &args![rig.world, entity, 1i32, 1u32]);
        assert_eq!(
            calls_to(&rig.e, UPDATE_SHAPE_COLLECTION_FILTER),
            vec![vec![0x601, 0x7000, partner_island]]
        );
    }

    /// 00c94a73 tests the third argument (`flag`) against 1, not `mode`: an accepted agent
    /// is left alone with mode 1 and flag 0, and updated with mode 2 and flag 1.
    #[test]
    fn update_filter_on_entity_updates_by_the_flag_not_the_mode() {
        let mut rig = rig();
        silent(&mut rig.e, &ENTITY_FILTER_CALLEES);
        let entity = filtered_entity(&mut rig.e, 1, 2);
        let island = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(entity + 0xcc, island);
        let partner = rig.e.mem.alloc(0x40);
        rig.e.mem.set_i8(partner + 0x1a, 3);
        agent_entries(&mut rig.e, &[(0x601, partner)]);
        group_table(&mut rig, &[23]);
        let filter = filter_answering(&mut rig.e, 1);
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(0x7000));
        rig.e
            .call(0x00c9_46e0, &args![rig.world, entity, 1i32, 0u32]);
        assert!(calls_to(&rig.e, UPDATE_SHAPE_COLLECTION_FILTER).is_empty());
        rig.e
            .call(0x00c9_46e0, &args![rig.world, entity, 2i32, 1u32]);
        assert_eq!(
            calls_to(&rig.e, UPDATE_SHAPE_COLLECTION_FILTER),
            vec![vec![0x601, 0x7000, island]]
        );
        assert!(calls_to(&rig.e, REMOVE_AGENT).is_empty());
    }

    #[test]
    fn update_filter_on_entity_mode_0_collects_the_overlapping_bodies_first() {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[
                COLLIDABLE_RESET_AGENTS,
                UPDATE_SHAPE_COLLECTION_FILTER,
                ADD_PAIRS,
                QUEUE_FLUSH_PENDING,
            ],
        );
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        let entity = filtered_entity(&mut rig.e, 1, 2);
        let island = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(entity + 0xcc, island);
        // A phantom that the broad phase reports through its collidable (+0x10), and a
        // rigid body.
        let phantom = filtered_phantom(&mut rig.e, &log);
        rig.e.mem.set_u8(phantom + 0x18, 2);
        rig.e.mem.set_i8(phantom + 0x15, -0x10);
        let (rigid, _) = collidable_in(&mut rig.e, 1, 0);
        let broad_phase = broad_phase_finding(&mut rig.e, 0x48, &[phantom + 0x10, rigid]);
        rig.e
            .set(rig.world, HkpWorld::m_broadPhase, Ptr::new(broad_phase));
        let filter = filter_answering(&mut rig.e, 1);
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        rig.e.set(
            rig.world,
            HkpWorld::m_broadPhaseDispatcher,
            Ptr::new(0x6000),
        );
        // One agent entry whose two collidables' handles go to the second list.
        let first_body = rig.e.mem.alloc(0x40);
        let second_body = rig.e.mem.alloc(0x40);
        rig.e.mem.set_i8(second_body + 0x1a, 3);
        let entry = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(entry + 0x10, first_body);
        rig.e.mem.set_u32(entry + 0x14, second_body);
        agent_entries(&mut rig.e, &[(entry, second_body)]);
        group_table(&mut rig, &[23]);
        let seen = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        rig.e
            .register_double(REMOVE_DUPLICATE_PAIRS, move |e, words| {
                let second = words[0];
                let data = e.mem.u32(second);
                sink.borrow_mut().push((
                    e.mem.u32(second + 4),
                    e.mem.u32(data),
                    e.mem.u32(data + 4),
                    e.mem.u32(words[1] + 4),
                ));
                Ret::default()
            });
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(0x7000));
        rig.e
            .call(0x00c9_46e0, &args![rig.world, entity, 0i32, 1u32]);
        // The overlapping phantom was asked about the entity's collidable and told to
        // update (flag set); the rigid body stays in the list and gets pairs added.
        assert_eq!(log.borrow().clone(), vec![(0x1c, phantom), (0x30, phantom)]);
        assert_eq!(
            seen.borrow().clone(),
            vec![(1, first_body + 0x14, second_body + 0x14, 1)]
        );
        let added = calls_to(&rig.e, ADD_PAIRS);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], 0x6000);
        assert_eq!(added[0][2], 1);
        assert_eq!(added[0][3], filter + 8);
        // With flag 1 the accepted agent is updated in mode 0 too (00c94a73 tests `flag`).
        assert_eq!(
            calls_to(&rig.e, UPDATE_SHAPE_COLLECTION_FILTER),
            vec![vec![entry, 0x7000, island]]
        );
        assert_eq!(
            record_labels(&rig, &[PROFILE_UPDATE_FILTER_ON_ENTITY]),
            vec![
                PROFILE_UPDATE_FILTER_ON_ENTITY,
                PROFILE_BROADPHASE_SECOND,
                PROFILE_PHANTOMS,
                PROFILE_CHECK_AGENTS,
                PROFILE_ADD_AGENTS,
                PROFILE_SECTION_END
            ]
        );
        assert_eq!(rig.e.mem.u32(rig.world.addr() + 0x94), 0);
    }

    #[test]
    fn update_filter_on_entity_is_queued_when_locked() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        rig.e
            .call(0x00c9_46e0, &args![rig.world, 0x4000u32, 1i32, 1u32]);
        let mut expected = op_bytes(0x12, 0x4000, 1, 0);
        expected[9] = 1;
        assert_eq!(queued.borrow().clone(), vec![(0x5000, expected)]);
    }

    const ADD_BATCH_CALLEES: [u32; 7] = [
        SET_CACHED_SHAPE_DATA,
        SET_TIME_INFORMATION,
        ADD_REFERENCE,
        ISLAND_ADD_ENTITY,
        FIRE_WORLD_ENTITY_ADDED,
        FIRE_ENTITY_ADDED,
        QUEUE_FLUSH_PENDING,
    ];

    /// The arrays a broad phase call received: `(entries of the first array, size of the
    /// second)`.
    type BroadPhaseSeen = Rc<RefCell<Vec<(Vec<u32>, u32)>>>;

    /// A broad phase whose slot `slot` notes the 4-byte entries of its first array and the
    /// size of its second array.
    fn broad_phase_noting(e: &mut Engine, slot: u32) -> (u32, BroadPhaseSeen) {
        let seen: BroadPhaseSeen = Rc::new(RefCell::new(vec![]));
        let sink = seen.clone();
        e.register_double(0x0900_0100 + slot, move |e, words| {
            let data = e.mem.u32(words[1]);
            let entries = (0..e.mem.u32(words[1] + 4))
                .map(|i| e.mem.u32(data + 4 * i))
                .collect();
            sink.borrow_mut().push((entries, e.mem.u32(words[2] + 4)));
            Ret::default()
        });
        (
            object_with_vtable(e, 0x20, &[(slot, 0x0900_0100 + slot)]),
            seen,
        )
    }

    /// An island with `bodies` in its body list.
    fn island_of(rig: &mut Rig, bodies: &[u32]) -> u32 {
        let island = rig.e.mem.alloc(0x80);
        let data = array_of(&mut rig.e, bodies);
        rig.e.mem.set_u32(island + 0x48, data);
        rig.e.mem.set_u32(island + 0x4c, bodies.len() as u32);
        island
    }

    #[test]
    fn add_entity_batch_adds_the_bodies_to_the_first_active_island() {
        let mut rig = rig();
        silent(&mut rig.e, &ADD_BATCH_CALLEES);
        silent(&mut rig.e, &[ADD_PAIRS]);
        let shape_calls = recording(&mut rig.e, FAKE_2);
        returning(&mut rig.e, FAKE_3, 0x5555);
        let shape = object_with_vtable(&mut rig.e, 0x20, &[(0x1c, FAKE_2)]);
        let first = object_with_vtable(&mut rig.e, 0x240, &[(0x10, FAKE_3)]);
        let second = object_with_vtable(&mut rig.e, 0x240, &[(0x10, FAKE_3)]);
        rig.e.mem.set_u32(first + 0x10, shape);
        rig.e.mem.set_u8(second + 0xe8, 5);
        rig.e.mem.set_u32(second + 0x18, 0x4444);
        let island = island_of(&mut rig, &[0x31, 0x32]);
        rig.e.mem.set_u32(island + 0x50, 3);
        let islands = array_of(&mut rig.e, &[island]);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 0x28, islands);
        rig.e.mem.set_u32(world + 0x2c, 1);
        rig.e.mem.set_u32(world + 0x20, 0x9999);
        let input = rig.e.mem.alloc(0x10);
        rig.e.mem.set_f32(input + 8, 1.0);
        rig.e.mem.set_u32(world + 0x78, input);
        rig.e.mem.set_u32(world + 0x338, 4);
        rig.e.mem.set_u32(world + 0x64, 0x6000);
        rig.e.mem.set_u32(world + 0x104, 10);
        rig.e.mem.set_u32(world + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        let (broad_phase, seen) = broad_phase_noting(&mut rig.e, 0x18);
        rig.e.mem.set_u32(world + 0x58, broad_phase);
        let entities = array_of(&mut rig.e, &[first, second]);
        rig.e
            .call(0x00c9_4bd0, &args![rig.world, entities, 2i32, 1i32]);
        // Uids, motions and the world.
        assert_eq!(rig.e.mem.u32(first + 0xd4), 11);
        assert_eq!(rig.e.mem.u32(second + 0xd4), 12);
        assert_eq!(rig.e.mem.u32(world + 0x104), 12);
        assert_eq!(rig.e.mem.u32(first + 0x18), 0x5555);
        assert_eq!(rig.e.mem.u32(second + 0x18), 0x4444);
        assert_eq!(rig.e.mem.u32(first + 8), world);
        assert_eq!(rig.e.mem.u32(first + 0x30), 1);
        assert_eq!(rig.e.mem.u32(first + 0x40), 0);
        // The island of the entity, the fixed island for the type 5 entity.
        assert_eq!(
            calls_to(&rig.e, ISLAND_ADD_ENTITY),
            vec![vec![island, first], vec![0x9999, second]]
        );
        // Room made in the island's body list (at least twice its capacity).
        let reserves = calls_to(&rig.e, ARRAY_RESERVE);
        let allocator_10 = rig.e.mem.u32(rig.router + 0x10);
        let allocator_c = rig.e.mem.u32(rig.router + 0xc);
        assert!(reserves.contains(&vec![allocator_10, island + 0x48, 6, 4]));
        assert!(reserves.contains(&vec![allocator_c, reserves[0][1], 2, 4]));
        assert!(reserves
            .iter()
            .any(|call| call[0] == allocator_c && call[2] == 2 && call[3] == 0x20));
        // The shape got the motion, half the delta time and a slot for its box.
        let shape_calls = shape_calls.borrow().clone();
        assert_eq!(shape_calls.len(), 1);
        assert_eq!(shape_calls[0][..3], [shape, 0x5555, 0.5f32.to_bits()]);
        assert_eq!(
            calls_to(&rig.e, SET_CACHED_SHAPE_DATA),
            vec![vec![first, world, shape]]
        );
        assert_eq!(calls_to(&rig.e, SET_TIME_INFORMATION).len(), 2);
        assert_eq!(seen.borrow().clone(), vec![(vec![first + 0x24], 1)]);
        assert_eq!(
            calls_to(&rig.e, ADD_PAIRS),
            vec![vec![0x6000, calls_to(&rig.e, ADD_PAIRS)[0][1], 0, 0]]
        );
        assert_eq!(
            calls_to(&rig.e, FIRE_WORLD_ENTITY_ADDED),
            vec![vec![world, first], vec![world, second]]
        );
        assert_eq!(
            calls_to(&rig.e, FIRE_ENTITY_ADDED),
            vec![vec![first], vec![second]]
        );
        assert_eq!(
            record_labels(&rig, &[PROFILE_ADD_ENTITIES]),
            vec![
                PROFILE_ADD_ENTITIES,
                PROFILE_ADD_BROADPHASE,
                PROFILE_CREATE_AGENTS,
                PROFILE_ADDED_CALLBACKS,
                PROFILE_SECTION_END
            ]
        );
        // The two lists reserved for the batch go back to the second allocator.
        let freed: Vec<u32> = rig
            .freed_second
            .borrow()
            .iter()
            .map(|call| call[2])
            .collect();
        assert_eq!(freed, vec![0x40, 8]);
        assert!(rig.freed_first.borrow().is_empty());
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
        assert_eq!(calls_to(&rig.e, QUEUE_FLUSH_PENDING), vec![vec![0x5000]]);
    }

    const ISLAND_DESTRUCTOR: u32 = 0x0900_0060;

    /// Sets up a batch that builds a new island; returns the island, which `00d07830`
    /// answers with.
    fn new_island_rig(rig: &mut Rig, bodies: &[u32]) -> u32 {
        silent(&mut rig.e, &ADD_BATCH_CALLEES);
        silent(&mut rig.e, &[ADD_PAIRS]);
        recording(&mut rig.e, ISLAND_DESTRUCTOR);
        let island = object_with_vtable(&mut rig.e, 0x80, &[(0, ISLAND_DESTRUCTOR)]);
        let data = array_of(&mut rig.e, bodies);
        rig.e.mem.set_u32(island + 0x48, data);
        returning(&mut rig.e, ISLAND_CONSTRUCT, island);
        let world = rig.world.addr();
        rig.e.set(rig.world, HkpWorld::m_addsToNewIsland, true);
        let input = rig.e.mem.alloc(0x10);
        rig.e.mem.set_u32(world + 0x78, input);
        let (broad_phase, _) = broad_phase_noting(&mut rig.e, 0x18);
        rig.e.mem.set_u32(world + 0x58, broad_phase);
        island
    }

    #[test]
    fn add_entity_batch_registers_a_new_island_with_the_world() {
        let mut rig = rig();
        let first = rig.e.mem.alloc(0x240);
        rig.e.mem.set_u32(first + 0x18, 0x4444);
        let island = new_island_rig(&mut rig, &[first]);
        let world = rig.world.addr();
        // Seven active islands already, room for exactly those.
        let list = array_of(&mut rig.e, &[0; 7]);
        rig.e.mem.set_u32(world + 0x28, list);
        rig.e.mem.set_u32(world + 0x2c, 7);
        rig.e.mem.set_u32(world + 0x30, 7);
        let entities = array_of(&mut rig.e, &[first]);
        rig.e
            .call(0x00c9_4bd0, &args![rig.world, entities, 1i32, 1i32]);
        let block = calls_to(&rig.e, ISLAND_CONSTRUCT)[0][0];
        assert_eq!(calls_to(&rig.e, ISLAND_CONSTRUCT), vec![vec![block, world]]);
        assert_eq!(rig.e.mem.u16(block + 4), 0x6c);
        assert_eq!(rig.e.mem.u8(island + 0x25), 1);
        assert_eq!(rig.e.mem.u16(island + 0x20), 7);
        assert_eq!(rig.e.mem.u8(island + 0x26), 5);
        assert_eq!(rig.e.mem.u8(island + 0x24), 1);
        assert_eq!(
            calls_to(&rig.e, ISLAND_ADD_ENTITY),
            vec![vec![island, first]]
        );
        assert_eq!(rig.e.mem.u32(world + 0x2c), 8);
        let data = rig.e.mem.u32(world + 0x28);
        assert_eq!(rig.e.mem.u32(data + 28), island);
        assert!(calls_to(&rig.e, ISLAND_DESTRUCTOR).is_empty());
    }

    #[test]
    fn add_entity_batch_deletes_a_new_island_nothing_used() {
        let mut rig = rig();
        let fixed_only = rig.e.mem.alloc(0x240);
        rig.e.mem.set_u32(fixed_only + 0x18, 0x4444);
        let island = new_island_rig(&mut rig, &[]);
        rig.e.mem.set_u8(fixed_only + 0xe8, 5);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 0x20, 0x9999);
        let entities = array_of(&mut rig.e, &[fixed_only]);
        rig.e
            .call(0x00c9_4bd0, &args![rig.world, entities, 1i32, 0i32]);
        // Mode 0: the id comes from the inactive list's size, the flags are cleared.
        assert_eq!(rig.e.mem.u8(island + 0x26), 0);
        assert_eq!(
            calls_to(&rig.e, ISLAND_ADD_ENTITY),
            vec![vec![0x9999, fixed_only]]
        );
        assert_eq!(calls_to(&rig.e, ISLAND_DESTRUCTOR), vec![vec![island, 1]]);
    }

    #[test]
    fn add_entity_batch_does_nothing_for_zero_and_queues_when_locked() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        rig.e
            .call(0x00c9_4bd0, &args![rig.world, 0x4000u32, 0i32, 1i32]);
        assert!(queued.borrow().is_empty());
        rig.e
            .call(0x00c9_4bd0, &args![rig.world, 0x4000u32, 3i32, 1i32]);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(6, 0x4000, 3, 1))]
        );
        assert_eq!(rig.e.mem.u32(rig.stream + 4), rig.area);
    }

    const REMOVE_BATCH_CALLEES: [u32; 6] = [
        FIRE_WORLD_ENTITY_REMOVED,
        FIRE_ENTITY_REMOVED,
        REMOVE_ENTITY_BP,
        ENTITY_RELEASE_BOUNDING_DATA,
        REMOVE_REFERENCE,
        QUEUE_FLUSH_PENDING,
    ];

    #[test]
    fn remove_entity_batch_removes_the_pairs_and_runs_the_callbacks() {
        let mut rig = rig();
        silent(&mut rig.e, &REMOVE_BATCH_CALLEES);
        let removed = recording(&mut rig.e, REMOVE_PAIRS);
        let told = recording(&mut rig.e, FAKE_2);
        let simulation = object_with_vtable(&mut rig.e, 0x20, &[(0x20, FAKE_2)]);
        let deleted = recording(&mut rig.e, FAKE_3);
        let first = object_with_vtable(&mut rig.e, 0x240, &[(0x14, FAKE_3)]);
        let second = object_with_vtable(&mut rig.e, 0x240, &[(0x14, FAKE_3)]);
        rig.e.mem.set_u32(first + 0x10, 1);
        rig.e.mem.set_u32(first + 0x54, 1);
        rig.e.mem.set_u16(second + 4, 0x40);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 8, simulation);
        rig.e.mem.set_u32(world + 0x64, 0x6000);
        rig.e.mem.set_u32(world + 0x338, 4);
        rig.e.mem.set_u32(world + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        let (broad_phase, seen) = broad_phase_noting(&mut rig.e, 0x20);
        rig.e.mem.set_u32(world + 0x58, broad_phase);
        let entities = array_of(&mut rig.e, &[first, second]);
        rig.e.call(0x00c9_51b0, &args![rig.world, entities, 2i32]);
        assert_eq!(
            told.borrow().clone(),
            vec![vec![simulation, entities, 2, world, 1]]
        );
        assert_eq!(seen.borrow().clone(), vec![(vec![first + 0x24], 0)]);
        let removed = removed.borrow().clone();
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], 0x6000);
        assert_eq!(removed[0][2], 0);
        assert_eq!(
            calls_to(&rig.e, FIRE_WORLD_ENTITY_REMOVED),
            vec![vec![world, first], vec![world, second]]
        );
        assert_eq!(
            calls_to(&rig.e, FIRE_ENTITY_REMOVED),
            vec![vec![first], vec![second]]
        );
        assert_eq!(
            calls_to(&rig.e, REMOVE_ENTITY_BP),
            vec![vec![world, first], vec![world, second]]
        );
        assert_eq!(
            calls_to(&rig.e, ENTITY_RELEASE_BOUNDING_DATA),
            vec![vec![first + 0x30]]
        );
        assert_eq!(deleted.borrow().clone(), vec![vec![first]]);
        assert_eq!(
            calls_to(&rig.e, REMOVE_REFERENCE),
            vec![vec![first], vec![second]]
        );
        assert_eq!(
            record_labels(&rig, &[PROFILE_REMOVE_ENTITIES]),
            vec![
                PROFILE_REMOVE_ENTITIES,
                PROFILE_REMOVE_BROADPHASE,
                PROFILE_DELETE_AGENTS,
                PROFILE_REMOVE_CALLBACKS,
                PROFILE_SECTION_END
            ]
        );
        // The handle list was reserved through the second allocator and goes back to it.
        let freed: Vec<u32> = rig
            .freed_second
            .borrow()
            .iter()
            .map(|call| call[2])
            .collect();
        assert_eq!(freed, vec![8]);
        assert_eq!(calls_to(&rig.e, QUEUE_FLUSH_PENDING), vec![vec![0x5000]]);
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn remove_entity_batch_skips_the_broad_phase_without_shapes() {
        let mut rig = rig();
        silent(&mut rig.e, &REMOVE_BATCH_CALLEES);
        let removed = recording(&mut rig.e, REMOVE_PAIRS);
        let simulation = object_with_vtable(&mut rig.e, 0x20, &[(0x20, FAKE_2)]);
        silent(&mut rig.e, &[FAKE_2]);
        let entity = rig.e.mem.alloc(0x240);
        rig.e.mem.set_u16(entity + 4, 0x40);
        rig.e.mem.set_u32(rig.world.addr() + 8, simulation);
        let entities = array_of(&mut rig.e, &[entity]);
        rig.e.call(0x00c9_51b0, &args![rig.world, entities, 1i32]);
        assert!(removed.borrow().is_empty());
        assert_eq!(
            record_labels(&rig, &[PROFILE_REMOVE_ENTITIES]),
            vec![
                PROFILE_REMOVE_ENTITIES,
                PROFILE_REMOVE_CALLBACKS,
                PROFILE_SECTION_END
            ]
        );
    }

    #[test]
    fn remove_entity_batch_loads_the_entity_again_for_every_call() {
        // 00c95424..00c9545f: a callback that rewrites the array slot redirects the later
        // calls of the same pass.
        let mut rig = rig();
        silent(&mut rig.e, &REMOVE_BATCH_CALLEES);
        let simulation = object_with_vtable(&mut rig.e, 0x20, &[(0x20, FAKE_2)]);
        silent(&mut rig.e, &[FAKE_2]);
        let entity = rig.e.mem.alloc(0x240);
        rig.e.mem.set_u16(entity + 4, 0x40);
        let replacement = rig.e.mem.alloc(0x240);
        rig.e.mem.set_u16(replacement + 4, 0x40);
        rig.e.mem.set_u32(replacement + 0x54, 1);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 8, simulation);
        let entities = array_of(&mut rig.e, &[entity]);
        rig.e
            .register_double(FIRE_WORLD_ENTITY_REMOVED, move |e, _| {
                e.mem.set_u32(entities, replacement);
                Ret::default()
            });
        rig.e.call(0x00c9_51b0, &args![rig.world, entities, 1i32]);
        assert_eq!(
            calls_to(&rig.e, FIRE_WORLD_ENTITY_REMOVED),
            vec![vec![world, entity]]
        );
        assert_eq!(
            calls_to(&rig.e, FIRE_ENTITY_REMOVED),
            vec![vec![replacement]]
        );
        assert_eq!(
            calls_to(&rig.e, REMOVE_ENTITY_BP),
            vec![vec![world, replacement]]
        );
        assert_eq!(
            calls_to(&rig.e, ENTITY_RELEASE_BOUNDING_DATA),
            vec![vec![replacement + 0x30]]
        );
        assert_eq!(calls_to(&rig.e, REMOVE_REFERENCE), vec![vec![replacement]]);
    }

    #[test]
    fn remove_entity_batch_does_nothing_for_no_entities_and_queues_when_locked() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.call(0x00c9_51b0, &args![rig.world, 0x4000u32, 0i32]);
        lock(&mut rig, 1);
        rig.e.call(0x00c9_51b0, &args![rig.world, 0x4000u32, 0i32]);
        assert!(queued.borrow().is_empty());
        rig.e.call(0x00c9_51b0, &args![rig.world, 0x4000u32, 2i32]);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(7, 0x4000, 2, 0))]
        );
    }

    #[test]
    fn add_phantom_batch_adds_them_all_to_the_broad_phase() {
        let mut rig = rig();
        silent(&mut rig.e, &PHANTOM_CALLEES);
        silent(&mut rig.e, &[PHANTOM_SET_AABB, ADD_PAIRS]);
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        let first = phantom_object(&mut rig.e, &log);
        let second = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(second + 0x18, 0x4444);
        // The second phantom's own bounding box maker is the same double.
        let table = rig.e.mem.u32(first);
        rig.e.mem.set_u32(second, table);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 0x338, 4);
        rig.e.mem.set_u32(world + 0x64, 0x6000);
        rig.e.mem.set_u32(world + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        let (broad_phase, seen) = broad_phase_noting(&mut rig.e, 0x18);
        rig.e.mem.set_u32(world + 0x58, broad_phase);
        let phantoms = array_of(&mut rig.e, &[first, second]);
        let cursor = rig.e.mem.u32(rig.router);
        rig.e.call(0x00c9_54e0, &args![rig.world, phantoms, 2i32]);
        assert_eq!(
            seen.borrow().clone(),
            vec![(vec![first + 0x24, second + 0x24], 2)]
        );
        assert_eq!(phantoms_of(&rig), vec![first, second]);
        assert_eq!(rig.e.mem.u32(first + 0x18), 0x7777);
        assert_eq!(rig.e.mem.u32(second + 0x18), 0x4444);
        assert_eq!(rig.e.mem.u32(first + 8), world);
        assert_eq!(rig.e.mem.u32(second + 8), world);
        // Each phantom was asked for its box into its own slot, which `00d07950` got too.
        let boxes = calls_to(&rig.e, PHANTOM_SET_AABB);
        assert_eq!(boxes.len(), 2);
        assert_eq!(boxes[1][1], boxes[0][1] + 0x20);
        assert_eq!(rig.e.mem.u32(boxes[0][1]), 0xb0b0_0000 + first);
        assert_eq!(rig.e.mem.u32(boxes[1][1]), 0xb0b0_0000 + second);
        assert_eq!(
            calls_to(&rig.e, FIRE_PHANTOM_ADDED),
            vec![vec![world, first], vec![world, second]]
        );
        assert_eq!(
            calls_to(&rig.e, PHANTOM_AFTER_ADD),
            vec![vec![first], vec![second]]
        );
        assert_eq!(calls_to(&rig.e, ADD_PAIRS).len(), 1);
        assert_eq!(calls_to(&rig.e, QUEUE_FLUSH_PENDING), vec![vec![0x5000]]);
        assert_eq!(rig.e.mem.u32(rig.router), cursor);
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn remove_phantom_batch_removes_them_all() {
        let mut rig = rig();
        silent(&mut rig.e, &PHANTOM_CALLEES);
        silent(&mut rig.e, &[REMOVE_REFERENCE]);
        let removed = recording(&mut rig.e, REMOVE_PAIRS);
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        let first = phantom_object(&mut rig.e, &log);
        let second = rig.e.mem.alloc(0x40);
        let table = rig.e.mem.u32(first);
        rig.e.mem.set_u32(second, table);
        set_phantoms(&mut rig, &[first, second, 0x21]);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 0x338, 4);
        rig.e.mem.set_u32(world + 0x64, 0x6000);
        let (broad_phase, seen) = broad_phase_noting(&mut rig.e, 0x20);
        rig.e.mem.set_u32(world + 0x58, broad_phase);
        let phantoms = array_of(&mut rig.e, &[first, second]);
        let cursor = rig.e.mem.u32(rig.router);
        rig.e.call(0x00c9_5800, &args![rig.world, phantoms, 2i32]);
        assert_eq!(
            seen.borrow().clone(),
            vec![(vec![first + 0x24, second + 0x24], 0)]
        );
        assert_eq!(removed.borrow().len(), 1);
        assert_eq!(phantoms_of(&rig), vec![0x21]);
        assert_eq!(rig.e.mem.u32(first + 8), 0);
        assert_eq!(log.borrow().clone(), vec![(0x34, first), (0x34, second)]);
        assert_eq!(
            calls_to(&rig.e, FIRE_PHANTOM_REMOVED),
            vec![vec![world, first], vec![world, second]]
        );
        assert_eq!(
            calls_to(&rig.e, PHANTOM_BEFORE_REMOVE),
            vec![vec![first], vec![second]]
        );
        assert_eq!(
            calls_to(&rig.e, REMOVE_REFERENCE),
            vec![vec![first], vec![second]]
        );
        assert_eq!(rig.e.mem.u32(rig.router), cursor);
    }

    /// A world with `agents` blocks: a single active island whose single block holds two
    /// agent entries 0x18 bytes long. The first entry's bodies are in groups (1, 2), the
    /// second's in (0, 1). Returns `(island, first entry, second entry)`.
    fn world_with_agent_entries(rig: &mut Rig) -> (u32, u32, u32) {
        let block = rig.e.mem.alloc(0x100);
        let island = rig.e.mem.alloc(0x80);
        let blocks = array_of(&mut rig.e, &[block]);
        rig.e.mem.set_u32(island + 0x5c, blocks);
        rig.e.mem.set_u32(island + 0x60, 1);
        rig.e.mem.set_u32(island + 0x58, 0x30);
        let groups = [(1i8, 2i8), (0, 1)];
        for (i, (first, second)) in groups.into_iter().enumerate() {
            let entry = block + 0x18 * i as u32;
            rig.e.mem.set_u8(entry + 3, 0x18);
            let body_a = rig.e.mem.alloc(0x40);
            let body_b = rig.e.mem.alloc(0x40);
            rig.e.mem.set_i8(body_a + 0x1a, first);
            rig.e.mem.set_i8(body_b + 0x1a, second);
            rig.e.mem.set_u32(entry + 0x10, body_a);
            rig.e.mem.set_u32(entry + 0x14, body_b);
        }
        let islands = array_of(&mut rig.e, &[island]);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 0x28, islands);
        rig.e.mem.set_u32(world + 0x2c, 1);
        group_table(rig, &[12]);
        let filter = filter_answering(&mut rig.e, 1);
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(0x7000));
        (island, block, block + 0x18)
    }

    #[test]
    fn update_filter_on_world_removes_refused_agents_after_each_island() {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[UPDATE_SHAPE_COLLECTION_FILTER, QUEUE_FLUSH_PENDING],
        );
        let removed = recording(&mut rig.e, REMOVE_AGENT);
        let (island, first, second) = world_with_agent_entries(&mut rig);
        rig.e.mem.set_u32(rig.world.addr() + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.call(0x00c9_6ff0, &args![rig.world, 1i32, 1u32]);
        assert_eq!(removed.borrow().clone(), vec![vec![second]]);
        assert_eq!(
            calls_to(&rig.e, UPDATE_SHAPE_COLLECTION_FILTER),
            vec![vec![first, 0x7000, island]]
        );
        assert_eq!(rig.e.mem.u8(island + 0x25) & 3, 1);
        assert_eq!(rig.e.mem.u32(rig.world.addr() + 0x94), 0);
        assert!(!rig
            .e
            .get(rig.world, HkpWorld::m_blockExecutingPendingOperations));
        assert_eq!(calls_to(&rig.e, QUEUE_FLUSH_PENDING), vec![vec![0x5000]]);
        assert_eq!(
            record_labels(&rig, &[]),
            vec![PROFILE_UPDATE_FILTER_ON_WORLD, PROFILE_END]
        );
        // The update follows the flag, not the mode (00c972b6 tests the second argument):
        // mode 2 with flag 1 updates again, mode 1 with flag 0 only removes.
        rig.e.call(0x00c9_6ff0, &args![rig.world, 2i32, 1u32]);
        assert_eq!(
            calls_to(&rig.e, UPDATE_SHAPE_COLLECTION_FILTER),
            vec![vec![first, 0x7000, island], vec![first, 0x7000, island]]
        );
        assert_eq!(removed.borrow().len(), 2);
        rig.e.call(0x00c9_6ff0, &args![rig.world, 1i32, 0u32]);
        assert_eq!(calls_to(&rig.e, UPDATE_SHAPE_COLLECTION_FILTER).len(), 2);
        assert_eq!(removed.borrow().len(), 3);
    }

    #[test]
    fn update_filter_on_world_mode_0_updates_every_entity_and_phantom() {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[
                COLLIDABLE_RESET_AGENTS,
                REMOVE_DUPLICATE_PAIRS,
                QUEUE_FLUSH_PENDING,
            ],
        );
        agent_entries(&mut rig.e, &[]);
        let log: CallLog = Rc::new(RefCell::new(vec![]));
        let broad_phase = broad_phase_finding(&mut rig.e, 0x48, &[]);
        let entity = filtered_entity(&mut rig.e, 1, 0);
        let island = island_of(&mut rig, &[entity]);
        let active = array_of(&mut rig.e, &[island]);
        let phantom = filtered_phantom(&mut rig.e, &log);
        set_phantoms(&mut rig, &[phantom]);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 0x28, active);
        rig.e.mem.set_u32(world + 0x2c, 1);
        rig.e.mem.set_u32(world + 0x58, broad_phase);
        rig.e.mem.set_u32(world + 0x338, 2);
        rig.e.mem.set_u32(world + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.call(0x00c9_6ff0, &args![rig.world, 0i32, 1u32]);
        // One broad phase query for the entity, one for the phantom; the flag makes the
        // phantom's slot 0x30 run once.
        let queries = calls_to(&rig.e, 0x0900_0088);
        assert_eq!(queries.len(), 2);
        assert_eq!(queries[0][1], entity + 0x24);
        assert_eq!(queries[1][1], phantom + 0x24);
        assert_eq!(log.borrow().clone(), vec![(0x30, phantom)]);
        // The pending operations were flushed once, at the end.
        assert_eq!(calls_to(&rig.e, QUEUE_FLUSH_PENDING).len(), 1);
        assert_eq!(
            order_of(&rig.e, &[QUEUE_FLUSH_PENDING, 0x0900_0088]),
            vec![0x0900_0088, 0x0900_0088, QUEUE_FLUSH_PENDING]
        );
        assert_eq!(
            record_labels(
                &rig,
                &[
                    PROFILE_UPDATE_FILTER_ON_ENTITY,
                    PROFILE_UPDATE_FILTER_ON_PHANTOM
                ]
            ),
            vec![
                PROFILE_UPDATE_FILTER_ON_WORLD,
                PROFILE_UPDATE_FILTER_ON_ENTITY,
                PROFILE_BROADPHASE_SECOND,
                PROFILE_PHANTOMS,
                PROFILE_CHECK_AGENTS,
                PROFILE_SECTION_END,
                PROFILE_UPDATE_FILTER_ON_PHANTOM,
                PROFILE_UPDATE_OVERLAPS,
                PROFILE_COLLECTION_FILTER,
                PROFILE_SECTION_END,
                PROFILE_END
            ]
        );
    }

    #[test]
    fn update_filter_on_world_is_queued_when_locked() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        rig.e.call(0x00c9_6ff0, &args![rig.world, 1i32, 1u32]);
        let mut expected = vec![0u8; 16];
        expected[..3].copy_from_slice(&[0x15, 1, 1]);
        assert_eq!(queued.borrow().clone(), vec![(0x5000, expected)]);
        assert!(!rig
            .e
            .get(rig.world, HkpWorld::m_blockExecutingPendingOperations));
    }

    /// An action whose vtable slot 0x10 hands out `entities`.
    fn action_with_entities(e: &mut Engine, entities: &[u32]) -> u32 {
        let list = entities.to_vec();
        e.register_double(0x0900_0070, move |e, words| {
            let array = words[1];
            let data = e.mem.u32(array);
            for (i, entity) in list.iter().enumerate() {
                e.mem.set_u32(data + 4 * i as u32, *entity);
            }
            e.mem.set_u32(array + 4, list.len() as u32);
            Ret::default()
        });
        object_with_vtable(e, 0x50, &[(0x10, 0x0900_0070)])
    }

    /// An entity in `island` whose action list (+0x208) holds `actions` with room for
    /// `capacity`.
    fn entity_with_actions(e: &mut Engine, island: u32, actions: &[u32], capacity: u16) -> u32 {
        let entity = e.mem.alloc(0x240);
        e.mem.set_u32(entity + 0xcc, island);
        let block = e
            .mem
            .alloc(4 * u32::from(capacity).max(actions.len() as u32).max(1) + 8);
        let data = block + 4;
        for (i, action) in actions.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *action);
        }
        e.mem.set_u32(entity + 0x208, data);
        e.mem.set_u16(entity + 0x20c, actions.len() as u16);
        e.mem.set_u16(entity + 0x20e, capacity);
        entity
    }

    fn actions_of(e: &Engine, entity: u32) -> Vec<u32> {
        let data = e.mem.u32(entity + 0x208);
        (0..u32::from(e.mem.u16(entity + 0x20c)))
            .map(|i| e.mem.u32(data + 4 * i))
            .collect()
    }

    fn island_with_id(e: &mut Engine, id: u16) -> u32 {
        let island = e.mem.alloc(0x80);
        e.mem.set_u16(island + 0x20, id);
        island
    }

    const ACTION_CALLEES: [u32; 6] = [
        ADD_REFERENCE,
        REMOVE_REFERENCE,
        ISLAND_ADD_ACTION,
        MERGE_ISLANDS,
        FIRE_ACTION_ADDED,
        QUEUE_FLUSH_PENDING,
    ];

    fn growing_action_lists(e: &mut Engine) {
        e.register(ACTION_LIST_GROW, |e, words| {
            let list = words[0];
            let size = u32::from(e.mem.u16(list + 4));
            let capacity = u32::from(e.mem.u16(list + 6) & 0x3fff).max(2) * 2;
            let block = e.mem.alloc(capacity * 4);
            if size != 0 {
                let old = e.mem.bytes(e.mem.u32(list), size * 4);
                e.mem.write(block, &old);
            }
            e.mem.set_u32(list, block);
            e.mem.set_u16(list + 6, capacity as u16);
            Ret::default()
        });
    }

    #[test]
    fn add_action_puts_the_action_in_the_first_real_island_and_merges_the_others() {
        let mut rig = rig();
        silent(&mut rig.e, &ACTION_CALLEES);
        growing_action_lists(&mut rig.e);
        let first_island = island_with_id(&mut rig.e, 1);
        let second_island = island_with_id(&mut rig.e, 2);
        let same_island_entity = entity_with_actions(&mut rig.e, first_island, &[], 4);
        let first = entity_with_actions(&mut rig.e, first_island, &[], 4);
        let second = entity_with_actions(&mut rig.e, second_island, &[0x61], 1);
        let action = action_with_entities(&mut rig.e, &[first, second, same_island_entity]);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.mem.set_u32(rig.world.addr() + 0x8c, 1);
        let returned = rig.e.call(0x00c9_7370, &args![rig.world, action]).u32();
        let world = rig.world.addr();
        assert_eq!(returned, action);
        assert_eq!(rig.e.mem.u32(action + 8), world);
        assert_eq!(actions_of(&rig.e, first), vec![action]);
        assert_eq!(actions_of(&rig.e, second), vec![0x61, action]);
        assert_eq!(actions_of(&rig.e, same_island_entity), vec![action]);
        assert_eq!(
            calls_to(&rig.e, ISLAND_ADD_ACTION),
            vec![vec![first_island, action]]
        );
        assert_eq!(
            calls_to(&rig.e, MERGE_ISLANDS),
            vec![vec![world, first, second]]
        );
        assert_eq!(
            calls_to(&rig.e, FIRE_ACTION_ADDED),
            vec![vec![world, action]]
        );
        assert_eq!(
            order_of(
                &rig.e,
                &[
                    ADD_REFERENCE,
                    ISLAND_ADD_ACTION,
                    QUEUE_FLUSH_PENDING,
                    FIRE_ACTION_ADDED
                ]
            ),
            vec![
                ADD_REFERENCE,
                ISLAND_ADD_ACTION,
                QUEUE_FLUSH_PENDING,
                FIRE_ACTION_ADDED
            ]
        );
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn add_action_falls_back_to_the_first_entitys_island() {
        let mut rig = rig();
        silent(&mut rig.e, &ACTION_CALLEES);
        growing_action_lists(&mut rig.e);
        let loose = island_with_id(&mut rig.e, 0xffff);
        let first = entity_with_actions(&mut rig.e, loose, &[], 4);
        let second = entity_with_actions(&mut rig.e, loose, &[], 4);
        let action = action_with_entities(&mut rig.e, &[first, second]);
        rig.e.call(0x00c9_7370, &args![rig.world, action]);
        assert_eq!(
            calls_to(&rig.e, ISLAND_ADD_ACTION),
            vec![vec![loose, action]]
        );
        assert!(calls_to(&rig.e, MERGE_ISLANDS).is_empty());
        assert_eq!(actions_of(&rig.e, second), vec![action]);
    }

    #[test]
    fn add_action_is_queued_when_locked() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        let returned = rig.e.call(0x00c9_7370, &args![rig.world, 0x1234u32]).u32();
        assert_eq!(returned, 0);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(0xa, 0x1234, 0, 0))]
        );
    }

    #[test]
    fn remove_action_immediately_takes_the_action_out_of_everything() {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                FIRE_ACTION_REMOVED,
                ISLAND_REMOVE_ACTION,
                QUEUE_FLUSH_PENDING,
            ],
        );
        let owner = rig.e.mem.alloc(0x80);
        let list = array_of(&mut rig.e, &[0x91, 0x92]);
        rig.e.mem.set_u32(owner + 0x40, list);
        rig.e.mem.set_u32(owner + 0x44, 2);
        rig.e.mem.set_u32(owner + 0x48, 2);
        let island = island_with_id(&mut rig.e, 1);
        rig.e.mem.set_u32(island + 0x18, owner);
        rig.e.mem.set_u16(island + 0x22, 0xffff);
        let first = entity_with_actions(&mut rig.e, island, &[0x71, 0x72, 0x73], 3);
        let second = entity_with_actions(&mut rig.e, island, &[0x72], 2);
        let action = action_with_entities(&mut rig.e, &[first, second]);
        // The first entity's list gets the action at index 1, the second does not have it.
        rig.e.mem.set_u32(action + 0xc, island);
        rig.e.mem.set_u32(action + 8, 0x5555);
        let data = rig.e.mem.u32(first + 0x208);
        rig.e.mem.set_u32(data + 4, action);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e.mem.set_u32(rig.world.addr() + 0x8c, 1);
        rig.e.call(0x00c9_7510, &args![rig.world, action]);
        let world = rig.world.addr();
        // The last entry moved into the action's slot.
        assert_eq!(actions_of(&rig.e, first), vec![0x71, 0x73]);
        // The second entity did not hold it: its list lost one entry all the same (the slot
        // -1 got the last entry).
        assert_eq!(rig.e.mem.u16(second + 0x20c), 0);
        assert_eq!(
            calls_to(&rig.e, ISLAND_REMOVE_ACTION),
            vec![vec![island, action]]
        );
        assert_eq!(rig.e.mem.u32(action + 8), 0);
        assert_eq!(
            calls_to(&rig.e, FIRE_ACTION_REMOVED),
            vec![vec![world, action]]
        );
        // The island got its index in the owner's list and was added to it.
        assert_eq!(rig.e.mem.u16(island + 0x22), 2);
        assert_eq!(rig.e.mem.u32(owner + 0x44), 3);
        let data = rig.e.mem.u32(owner + 0x40);
        assert_eq!(rig.e.mem.u32(data + 8), island);
        assert_eq!(
            calls_to(&rig.e, REMOVE_REFERENCE),
            vec![vec![action], vec![action]]
        );
        assert_eq!(calls_to(&rig.e, ADD_REFERENCE), vec![vec![action]]);
        assert_eq!(calls_to(&rig.e, QUEUE_FLUSH_PENDING).len(), 1);
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn remove_action_is_queued_or_immediate() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        rig.e.call(0x00c9_7f50, &args![rig.world, 0x1234u32]);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(0xb, 0x1234, 0, 0))]
        );
        let (mut immediate, action) = rig_with_removable_action();
        let world = immediate.world;
        immediate.e.call(0x00c9_7f50, &args![world, action]);
        assert_eq!(
            calls_to(&immediate.e, FIRE_ACTION_REMOVED),
            vec![vec![world.addr(), action]]
        );
    }

    /// A world and an action with one entity and an island, for the paths that really
    /// remove it.
    fn rig_with_removable_action() -> (Rig, u32) {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                FIRE_ACTION_REMOVED,
                ISLAND_REMOVE_ACTION,
                QUEUE_FLUSH_PENDING,
            ],
        );
        let owner = rig.e.mem.alloc(0x80);
        let island = island_with_id(&mut rig.e, 1);
        rig.e.mem.set_u32(island + 0x18, owner);
        rig.e.mem.set_u16(island + 0x22, 0);
        let entity = entity_with_actions(&mut rig.e, island, &[], 1);
        let action = action_with_entities(&mut rig.e, &[entity]);
        rig.e.mem.set_u32(action + 0xc, island);
        let data = rig.e.mem.u32(entity + 0x208);
        rig.e.mem.set_u32(data, action);
        rig.e.mem.set_u16(entity + 0x20c, 1);
        (rig, action)
    }

    /// A constraint whose data (+0xC) is of `data_kind` (vtable slot 0x20), whose own kind
    /// (slot 0x18) is `kind`, and whose action (+0x44) has the world `action_world`.
    /// The data object's slot 0x40 and its owner's slot 0x24 are logged in `log`.
    fn constraint_object(
        e: &mut Engine,
        log: &Records,
        kind: u32,
        data_kind: u32,
        action_world: u32,
    ) -> u32 {
        returning(e, 0x0900_0081, kind);
        returning(e, 0x0900_0082, data_kind);
        let sink = log.clone();
        e.register_double(0x0900_0083, move |_, words| {
            sink.borrow_mut().push(words.to_vec());
            Ret::default()
        });
        let sink = log.clone();
        e.register_double(0x0900_0084, move |e, words| {
            e.mem.set_u16(words[2], 0x1111);
            e.mem.set_u16(words[2] + 4, 0x2222);
            sink.borrow_mut().push(words.to_vec());
            Ret::default()
        });
        let owner = object_with_vtable(e, 0x20, &[(0x24, 0x0900_0084)]);
        let data = object_with_vtable(e, 0x40, &[(0x20, 0x0900_0082), (0x40, 0x0900_0083)]);
        e.mem.set_u32(data + 0x18, owner);
        let action = e.mem.alloc(0x20);
        e.mem.set_u32(action + 8, action_world);
        let constraint = object_with_vtable(e, 0x60, &[(0x18, 0x0900_0081)]);
        e.mem.set_u32(constraint + 0xc, data);
        e.mem.set_u32(constraint + 0x44, action);
        constraint
    }

    #[test]
    fn add_constraint_numbers_the_constraint_and_adds_it_to_the_islands() {
        let mut rig = rig();
        silent(&mut rig.e, &[CONSTRAINT_PREPARE, QUEUE_FLUSH_PENDING]);
        returning(&mut rig.e, ADD_CONSTRAINT_SI, 0x77);
        let log = Rc::new(RefCell::new(vec![]));
        let constraint = constraint_object(&mut rig.e, &log, 1, 0xc, 0x1000);
        let data = rig.e.mem.u32(constraint + 0xc);
        let owner = rig.e.mem.u32(data + 0x18);
        let world = rig.world.addr();
        rig.e.mem.set_u32(world + 0x10c, 4);
        rig.e.mem.set_u32(world + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        let result = rig.e.call(0x00c9_7d30, &args![rig.world, constraint]).u32();
        assert_eq!(result, 0x77);
        assert_eq!(rig.e.mem.u16(data + 0x1c), 0x1111);
        assert_eq!(rig.e.mem.u16(data + 0x1e), 0x2222);
        assert_eq!(rig.e.mem.u32(world + 0x10c), 5);
        assert_eq!(rig.e.mem.u32(constraint + 0x34), 5);
        assert_eq!(
            calls_to(&rig.e, ADD_CONSTRAINT_SI),
            vec![vec![world, constraint, 1]]
        );
        let log = log.borrow().clone();
        // The owner got the flag in the low byte of its first argument, and the data object
        // was told about the new constraint (kind 1).
        assert_eq!(log.len(), 2);
        assert_eq!(log[0][0], owner);
        assert_eq!(log[0][1] & 0xff, 1);
        assert_eq!(log[1], vec![data]);
        assert_eq!(calls_to(&rig.e, QUEUE_FLUSH_PENDING), vec![vec![0x5000]]);
        assert!(!rig
            .e
            .get(rig.world, HkpWorld::m_blockExecutingPendingOperations));
    }

    #[test]
    fn add_constraint_skips_the_data_update_for_other_kinds() {
        let mut rig = rig();
        silent(&mut rig.e, &[CONSTRAINT_PREPARE, QUEUE_FLUSH_PENDING]);
        returning(&mut rig.e, ADD_CONSTRAINT_SI, 0x77);
        let log = Rc::new(RefCell::new(vec![]));
        let constraint = constraint_object(&mut rig.e, &log, 0, 5, 0x1000);
        let data = rig.e.mem.u32(constraint + 0xc);
        rig.e.call(0x00c9_7d30, &args![rig.world, constraint]);
        assert_eq!(rig.e.mem.u16(data + 0x1c), 0);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn add_constraint_is_queued_when_locked() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        let result = rig.e.call(0x00c9_7d30, &args![rig.world, 0x1234u32]).u32();
        assert_eq!(result, 0);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(8, 0x1234, 0, 0))]
        );
    }

    #[test]
    fn remove_constraint_detaches_it_and_reports_success() {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                REMOVE_CONSTRAINT_SI,
                CONSTRAINT_DETACH,
                QUEUE_FLUSH_PENDING,
            ],
        );
        let log = Rc::new(RefCell::new(vec![]));
        // Kind 1, but its action belongs to another world: it stays.
        let constraint = constraint_object(&mut rig.e, &log, 1, 0, 0x1000);
        let result = rig.e.mem.alloc(4);
        rig.e.mem.set_u8(result, 0x55);
        rig.e.mem.set_u32(rig.world.addr() + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        rig.e
            .call(0x00c9_7e80, &args![rig.world, result, constraint]);
        let world = rig.world.addr();
        assert_eq!(rig.e.mem.u8(result), 1);
        assert_eq!(
            order_of(
                &rig.e,
                &[
                    ADD_REFERENCE,
                    REMOVE_CONSTRAINT_SI,
                    CONSTRAINT_DETACH,
                    REMOVE_REFERENCE,
                    QUEUE_FLUSH_PENDING
                ]
            ),
            vec![
                ADD_REFERENCE,
                REMOVE_CONSTRAINT_SI,
                CONSTRAINT_DETACH,
                REMOVE_REFERENCE,
                QUEUE_FLUSH_PENDING
            ]
        );
        assert_eq!(
            calls_to(&rig.e, REMOVE_CONSTRAINT_SI),
            vec![vec![world, constraint, 1]]
        );
        assert_eq!(
            calls_to(&rig.e, CONSTRAINT_DETACH),
            vec![vec![constraint, world]]
        );
        assert_eq!(rig.e.mem.u32(world + 0x94), 0);
    }

    #[test]
    fn remove_constraint_removes_the_action_of_a_kind_1_constraint_in_this_world() {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                REMOVE_CONSTRAINT_SI,
                CONSTRAINT_DETACH,
                QUEUE_FLUSH_PENDING,
                FIRE_ACTION_REMOVED,
                ISLAND_REMOVE_ACTION,
            ],
        );
        let (action, island) = {
            let owner = rig.e.mem.alloc(0x80);
            let island = island_with_id(&mut rig.e, 1);
            rig.e.mem.set_u32(island + 0x18, owner);
            rig.e.mem.set_u16(island + 0x22, 0);
            let entity = entity_with_actions(&mut rig.e, island, &[], 1);
            let action = action_with_entities(&mut rig.e, &[entity]);
            rig.e.mem.set_u32(action + 0xc, island);
            let data = rig.e.mem.u32(entity + 0x208);
            rig.e.mem.set_u32(data, action);
            rig.e.mem.set_u16(entity + 0x20c, 1);
            (action, island)
        };
        let log = Rc::new(RefCell::new(vec![]));
        let constraint = constraint_object(&mut rig.e, &log, 1, 0, rig.world.addr());
        rig.e.mem.set_u32(constraint + 0x44, action);
        rig.e.mem.set_u32(action + 8, rig.world.addr());
        let result = rig.e.mem.alloc(4);
        rig.e
            .call(0x00c9_7e80, &args![rig.world, result, constraint]);
        assert_eq!(
            calls_to(&rig.e, ISLAND_REMOVE_ACTION),
            vec![vec![island, action]]
        );
        assert_eq!(rig.e.mem.u8(result), 1);
    }

    #[test]
    fn remove_constraint_is_queued_when_locked() {
        let mut rig = rig();
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        let result = rig.e.mem.alloc(4);
        rig.e.mem.set_u8(result, 0x55);
        rig.e
            .call(0x00c9_7e80, &args![rig.world, result, 0x1234u32]);
        assert_eq!(rig.e.mem.u8(result), 0);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(9, 0x1234, 0, 0))]
        );
    }

    // -----------------------------------------------------------------------------------
    // The third batch of `hkpWorld` functions (`00c92a30` to `00c983b0`).
    // -----------------------------------------------------------------------------------

    /// More doubles: the allocator's free by size (slot 8) and allocation that reads its size
    /// from a word (slot 0xC), and four hooks for the vtable slots of the objects below.
    const FAKE_6: u32 = 0x0900_0006;
    const FAKE_7: u32 = 0x0900_0007;
    const HOOK_A: u32 = 0x0900_0101;
    const HOOK_B: u32 = 0x0900_0102;
    const HOOK_C: u32 = 0x0900_0103;
    const HOOK_D: u32 = 0x0900_0104;
    const HOOK_E: u32 = 0x0900_0105;
    const HOOK_F: u32 = 0x0900_0106;

    /// `rig()` with the allocator's slot 8 (free by size, logged as `(allocator, block, size)`
    /// in the returned records) and slot 0xC (allocates the size in the word it is given), and
    /// with the call log on.
    fn batch_rig() -> (Rig, Records) {
        let mut rig = rig();
        let freed = recording(&mut rig.e, FAKE_6);
        rig.e.register(FAKE_7, |e, words| Ret {
            eax: e.mem.alloc(e.mem.u32(words[1]).max(8)),
            ..Ret::default()
        });
        let allocator = rig.e.mem.u32(rig.router + 0x10);
        let table = rig.e.mem.u32(allocator);
        rig.e.mem.set_u32(table + 8, FAKE_6);
        rig.e.mem.set_u32(table + 0xc, FAKE_7);
        rig.e.call_log = Some(vec![]);
        (rig, freed)
    }

    /// Fills the `hkArray` at `array` (data, size, capacity word).
    fn set_array(e: &mut Engine, array: u32, data: u32, size: u32, capacity: u32) {
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, size);
        e.mem.set_u32(array + 8, capacity);
    }

    /// Gives the world's seventeen pointer arrays the "no storage" capacity word, so that the
    /// destructor frees none of them unless the test fills it in.
    fn mark_arrays_unallocated(e: &mut Engine, world: u32) {
        for array in [
            0x28, 0x34, 0x40, 0x110, 0x11c, 0x128, 0x134, 0x140, 0x14c, 0x158, 0x164, 0x170, 0x17c,
            0x188, 0x194, 0x1a0, 0x1ac,
        ] {
            e.mem.set_u32(world + array + 8, 0x8000_0000);
        }
    }

    /// A simulation (installed in the world) whose float at +0x18 is `time` and whose virtual
    /// function 0x1C (`HOOK_A`) records its argument words followed by the four words of the
    /// record it is given.
    fn recording_simulation(rig: &mut Rig, time: f32) -> (u32, Records) {
        let records: Records = Rc::new(RefCell::new(vec![]));
        let sink = records.clone();
        rig.e.register_double(HOOK_A, move |e, words| {
            let mut entry = words.to_vec();
            for lane in 0..4 {
                entry.push(e.mem.u32(words[4] + 4 * lane));
            }
            sink.borrow_mut().push(entry);
            Ret::default()
        });
        let simulation = object_with_vtable(&mut rig.e, 0x40, &[(0x1c, HOOK_A)]);
        rig.e.mem.set_f32(simulation + 0x18, time);
        rig.e
            .set(rig.world, HkpWorld::m_simulation, Ptr::new(simulation));
        (simulation, records)
    }

    #[test]
    fn fn_00c92a30_runs_the_virtual_call_or_queues_it() {
        let (mut rig, _) = batch_rig();
        let calls = recording(&mut rig.e, HOOK_A);
        let object = object_with_vtable(&mut rig.e, 0x20, &[(0xc, HOOK_A)]);
        let returned = rig
            .e
            .call(0x00c9_2a30, &args![rig.world, object, 0x55u32])
            .u32();
        assert_eq!(returned, object);
        assert_eq!(calls.borrow().clone(), vec![vec![object, 0x55]]);
        let queued = operation_recorder(&mut rig.e);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        lock(&mut rig, 1);
        let returned = rig
            .e
            .call(0x00c9_2a30, &args![rig.world, object, 0x66u32])
            .u32();
        assert_eq!(returned, object);
        assert_eq!(calls.borrow().len(), 1);
        assert_eq!(
            queued.borrow().clone(),
            vec![(0x5000, op_bytes(0x24, object, 0x66, 0))]
        );
    }

    #[test]
    fn fn_00c92a80_builds_the_span_record() {
        let (mut rig, _) = batch_rig();
        let (simulation, records) = recording_simulation(&mut rig, 2.0);
        rig.e.call(0x00c9_2a80, &args![rig.world, 7u32, 8u32]);
        let entry = records.borrow()[0].clone();
        assert_eq!(entry[0..4], [simulation, 7, 8, rig.world.addr()]);
        assert_eq!(entry[5], 1);
        let two = 2.0f32.to_bits();
        assert_eq!(entry[6..], [two, two, 0, 0]);
    }

    #[test]
    fn fn_00c92a80_divides_when_the_span_is_not_equal_to_zero() {
        let (mut rig, _) = batch_rig();
        // Infinity minus infinity is not a number, which does not compare equal to zero.
        let (_, records) = recording_simulation(&mut rig, f32::INFINITY);
        rig.e.call(0x00c9_2a80, &args![rig.world, 1u32, 2u32]);
        let entry = records.borrow()[0].clone();
        assert_eq!(entry[6], f32::INFINITY.to_bits());
        assert_eq!(entry[7], f32::INFINITY.to_bits());
        assert!(f32::from_bits(entry[8]).is_nan());
        assert!(f32::from_bits(entry[9]).is_nan());
        // A signalling NaN is quieted by the load.
        let (_, records) = recording_simulation(&mut rig, f32::from_bits(0x7f80_0001));
        rig.e.call(0x00c9_2a80, &args![rig.world, 1u32, 2u32]);
        let entry = records.borrow()[0].clone();
        assert_eq!(entry[6], 0x7fc0_0001);
        assert_eq!(entry[7], 0x7fc0_0001);
    }

    #[test]
    fn fn_00c92b00_runs_the_span_for_every_island() {
        let (mut rig, _) = batch_rig();
        let (simulation, records) = recording_simulation(&mut rig, 1.0);
        let t = rig.world.addr();
        let mut islands = vec![];
        for index in 0..3u32 {
            let island = rig.e.mem.alloc(0x80);
            rig.e.mem.set_u32(island + 0x48, 0x100 + index);
            rig.e.mem.set_u32(island + 0x4c, 0x200 + index);
            islands.push(island);
        }
        let active = array_of(&mut rig.e, &islands[0..2]);
        let inactive = array_of(&mut rig.e, &islands[2..3]);
        set_array(&mut rig.e, t + 0x28, active, 2, 2);
        set_array(&mut rig.e, t + 0x34, inactive, 1, 1);
        rig.e.call(0x00c9_2b00, &args![rig.world]);
        let seen: Vec<Vec<u32>> = records
            .borrow()
            .iter()
            .map(|entry| entry[0..3].to_vec())
            .collect();
        assert_eq!(
            seen,
            vec![
                vec![simulation, 0x100, 0x200],
                vec![simulation, 0x101, 0x201],
                vec![simulation, 0x102, 0x202],
            ]
        );
    }

    /// An island with the words `get_mem_usage_for_integration` reads.
    fn usage_island(e: &mut Engine, rows: i32, words: i32, bodies: i32, base: i32) -> u32 {
        let island = e.mem.alloc(0x80);
        e.mem.set_i32(island + 0x1c, rows);
        e.mem.set_i32(island + 0x14, words);
        e.mem.set_i32(island + 0x4c, bodies);
        e.mem.set_i32(island + 0xc, base);
        island
    }

    #[test]
    fn get_mem_usage_for_integration_sums_and_finds_the_largest_island() {
        let mut e = Engine::new();
        let world = world(&mut e);
        // 160 / 80 = 2 -> 4 rows, base 12; 12 * 3 + 4 = 40 words -> (160 + 23) & !15 = 176;
        // (12 + 8 + 10) << 4 = 480; plus 100: 756.
        let large = usage_island(&mut e, 160, 4, 1, 100);
        // 80 / 80 = 1 -> 2 rows, base 10; 30 words -> (120 + 23) & !15 = 128; (10 + 10) << 4 =
        // 320: 448.
        let small = usage_island(&mut e, 80, 0, 0, 0);
        let active = array_of(&mut e, &[large, small]);
        e.set(world, HkpWorld::m_activeSimulationIslands, Ptr::new(active));
        e.set(world, HkpWorld::m_activeSimulationIslandsSize, 2);
        let usage = e.mem.alloc(0xc);
        for word in 0..3 {
            e.mem.set_u32(usage + 4 * word, 0xdead_beef);
        }
        e.call(0x00c9_2b50, &args![world, usage]);
        assert_eq!(e.mem.u32(usage), 756);
        assert_eq!(e.mem.u32(usage + 4), 1204);
        assert_eq!(e.mem.u32(usage + 8), large);
        // Equal estimates keep the island found first, which is the last of the array.
        let twin = usage_island(&mut e, 160, 4, 1, 100);
        let active = array_of(&mut e, &[large, twin]);
        e.set(world, HkpWorld::m_activeSimulationIslands, Ptr::new(active));
        e.call(0x00c9_2b50, &args![world, usage]);
        assert_eq!(e.mem.u32(usage + 8), twin);
        // Without islands the block is cleared.
        e.set(world, HkpWorld::m_activeSimulationIslandsSize, 0);
        e.call(0x00c9_2b50, &args![world, usage]);
        assert_eq!(e.mem.bytes(usage, 12), vec![0; 12]);
    }

    /// An action, an entity with an action list of one entry (size 1, capacity `capacity`) and
    /// the islands of both. Returns (action, entity, action island, entity island).
    fn action_fixture(rig: &mut Rig, capacity: u16) -> (u32, u32, u32, u32) {
        silent(
            &mut rig.e,
            &[ISLAND_REMOVE_ACTION, ISLAND_ADD_ACTION, MERGE_ISLANDS],
        );
        rig.e.register(ACTION_LIST_GROW, |e, words| {
            let list = words[0];
            let old = e.mem.u32(list);
            let grown = e.mem.alloc(0x40);
            let first = e.mem.u32(old);
            e.mem.set_u32(grown, first);
            e.mem.set_u32(list, grown);
            e.mem.set_u16(list + 6, 8);
            Ret::default()
        });
        let entity = rig.e.mem.alloc(0x300);
        let list = array_of(&mut rig.e, &[0x1111]);
        rig.e.mem.set_u32(entity + 0x208, list);
        rig.e.mem.set_u16(entity + 0x20c, 1);
        rig.e.mem.set_u16(entity + 0x20e, capacity);
        rig.e.mem.set_u8(entity + 0xe8, 1);
        let entity_island = rig.e.mem.alloc(0x80);
        rig.e.mem.set_u32(entity + 0xcc, entity_island);
        let action_island = rig.e.mem.alloc(0x80);
        let members = array_of(&mut rig.e, &[0x777]);
        rig.e.mem.set_u32(action_island + 0x48, members);
        let action = rig.e.mem.alloc(0x20);
        rig.e.mem.set_u32(action + 0xc, action_island);
        (action, entity, action_island, entity_island)
    }

    #[test]
    fn fn_00c92bd0_moves_an_action_off_the_unassigned_island() {
        let (mut rig, _) = batch_rig();
        let (action, entity, action_island, entity_island) = action_fixture(&mut rig, 4);
        rig.e.mem.set_u16(action_island + 0x20, 0xffff);
        rig.e.call(0x00c9_2bd0, &args![rig.world, action, entity]);
        let list = rig.e.mem.u32(entity + 0x208);
        assert_eq!(rig.e.mem.u32(list + 4), action);
        assert_eq!(rig.e.mem.u16(entity + 0x20c), 2);
        assert_eq!(
            calls_to(&rig.e, ISLAND_REMOVE_ACTION),
            vec![vec![action_island, action]]
        );
        assert_eq!(
            calls_to(&rig.e, ISLAND_ADD_ACTION),
            vec![vec![entity_island, action]]
        );
        assert!(calls_to(&rig.e, MERGE_ISLANDS).is_empty());
        assert!(calls_to(&rig.e, ACTION_LIST_GROW).is_empty());
    }

    #[test]
    fn fn_00c92bd0_grows_a_full_list_and_merges_different_islands() {
        let (mut rig, _) = batch_rig();
        let (action, entity, _, _) = action_fixture(&mut rig, 1);
        rig.e.call(0x00c9_2bd0, &args![rig.world, action, entity]);
        assert_eq!(
            calls_to(&rig.e, ACTION_LIST_GROW),
            vec![vec![entity + 0x208, 4]]
        );
        let list = rig.e.mem.u32(entity + 0x208);
        assert_eq!(rig.e.mem.u32(list), 0x1111);
        assert_eq!(rig.e.mem.u32(list + 4), action);
        assert_eq!(
            calls_to(&rig.e, MERGE_ISLANDS),
            vec![vec![rig.world.addr(), entity, 0x777]]
        );
        assert!(calls_to(&rig.e, ISLAND_REMOVE_ACTION).is_empty());
    }

    #[test]
    fn fn_00c92bd0_leaves_the_islands_alone_when_they_match_or_the_byte_is_5() {
        let (mut rig, _) = batch_rig();
        let (action, entity, action_island, _) = action_fixture(&mut rig, 4);
        rig.e.mem.set_u32(entity + 0xcc, action_island);
        rig.e.call(0x00c9_2bd0, &args![rig.world, action, entity]);
        assert!(calls_to(&rig.e, MERGE_ISLANDS).is_empty());
        let (action, entity, action_island, _) = action_fixture(&mut rig, 4);
        rig.e.mem.set_u8(entity + 0xe8, 5);
        rig.e.mem.set_u16(action_island + 0x20, 0xffff);
        rig.e.call(0x00c9_2bd0, &args![rig.world, action, entity]);
        assert!(calls_to(&rig.e, MERGE_ISLANDS).is_empty());
        assert!(calls_to(&rig.e, ISLAND_REMOVE_ACTION).is_empty());
        assert_eq!(rig.e.mem.u16(entity + 0x20c), 2);
    }

    #[test]
    fn init_mt_step_runs_the_watch_dog_inside_the_lock_and_returns_the_step() {
        let (mut rig, _) = batch_rig();
        silent(&mut rig.e, &[LOCK_ENTER, LOCK_LEAVE, WATCH_HEAP_MEMORY]);
        returning(&mut rig.e, STEP_BEGIN_ST, 0x1234);
        let simulation = rig.e.mem.alloc(0x40);
        rig.e
            .set(rig.world, HkpWorld::m_simulation, Ptr::new(simulation));
        rig.e
            .set(rig.world, HkpWorld::m_memoryWatchDog, Ptr::new(0x6000));
        let result = rig
            .e
            .call(0x00c9_2d70, &args![rig.world, 0x77u32, 1.5f32])
            .u32();
        assert_eq!(result, 0x1234);
        assert_eq!(
            order_of(
                &rig.e,
                &[LOCK_ENTER, WATCH_HEAP_MEMORY, LOCK_LEAVE, STEP_BEGIN_ST]
            ),
            vec![LOCK_ENTER, WATCH_HEAP_MEMORY, LOCK_LEAVE, STEP_BEGIN_ST]
        );
        assert_eq!(
            calls_to(&rig.e, STEP_BEGIN_ST),
            vec![vec![simulation, 0x77, 1.5f32.to_bits()]]
        );
        assert_eq!(rig.e.get(rig.world, HkpWorld::m_isLocked), 0);
    }

    #[test]
    fn init_mt_step_without_a_watch_dog_only_steps_and_quiets_the_time() {
        let (mut rig, _) = batch_rig();
        returning(&mut rig.e, STEP_BEGIN_ST, 0);
        let simulation = rig.e.mem.alloc(0x40);
        rig.e
            .set(rig.world, HkpWorld::m_simulation, Ptr::new(simulation));
        let signalling = f32::from_bits(0x7f80_0001);
        let result = rig
            .e
            .call(0x00c9_2d70, &args![rig.world, 0x77u32, signalling])
            .u32();
        assert_eq!(result, 0);
        assert_eq!(
            calls_to(&rig.e, STEP_BEGIN_ST),
            vec![vec![simulation, 0x77, 0x7fc0_0001]]
        );
        assert!(calls_to(&rig.e, WATCH_HEAP_MEMORY).is_empty());
    }

    /// What a statistics collector was told: the virtual function (or `ADD_NORMAL_CHUNK`) and
    /// its words after the collector.
    type Reports = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A collector (returned, with the report log) whose virtual functions 0xC, 0x14, 0x18 and
    /// 0x1C log; `ADD_NORMAL_CHUNK` logs too and the allocated size of every block is 0x99.
    fn statistics_collector(e: &mut Engine) -> (u32, Reports) {
        let reports: Reports = Rc::new(RefCell::new(vec![]));
        for (slot, hook) in [
            (0xc, HOOK_A),
            (0x14, HOOK_B),
            (0x18, HOOK_C),
            (0x1c, HOOK_D),
        ] {
            let sink = reports.clone();
            e.register_double(hook, move |_, words| {
                sink.borrow_mut().push((slot, words[1..].to_vec()));
                Ret::default()
            });
        }
        let sink = reports.clone();
        e.register_double(ADD_NORMAL_CHUNK, move |_, words| {
            sink.borrow_mut()
                .push((ADD_NORMAL_CHUNK, words[1..].to_vec()));
            Ret::default()
        });
        returning(e, GET_ALLOCATED_SIZE, 0x99);
        let collector = object_with_vtable(
            e,
            0x20,
            &[
                (0xc, HOOK_A),
                (0x14, HOOK_B),
                (0x18, HOOK_C),
                (0x1c, HOOK_D),
            ],
        );
        (collector, reports)
    }

    /// The kind and the name of every report: the name is the first word of the function's
    /// words, the second of the array report (after its `3`), and none of the section end.
    fn report_names(reports: &Reports) -> Vec<(u32, u32)> {
        reports
            .borrow()
            .iter()
            .map(|(kind, words)| match *kind {
                0x14 => (*kind, words[1]),
                0x1c => (*kind, 0),
                _ => (*kind, words[0]),
            })
            .collect()
    }

    #[test]
    fn calc_content_statistics_reports_every_part_of_an_empty_world() {
        let (mut rig, _) = batch_rig();
        let input = rig.e.mem.alloc(0x100);
        let operations = rig.e.mem.alloc(0x40);
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(input));
        rig.e.set(
            rig.world,
            HkpWorld::m_pendingOperations,
            Ptr::new(operations),
        );
        let (collector, reports) = statistics_collector(&mut rig.e);
        rig.e.mem.set_u32(collector + 8, 0xdead);
        rig.e.call(0x00c9_2db0, &args![rig.world, collector, 0u32]);
        let chunk = ADD_NORMAL_CHUNK;
        let expected = vec![
            (0x14, STATISTICS_ACTIVE_ISLANDS),
            (0x14, STATISTICS_INACTIVE_ISLANDS),
            (0xc, STATISTICS_FIXED),
            (0x14, STATISTICS_PHANTOM_POINTERS),
            (chunk, STATISTICS_ISLAND_DIRTY_LIST_SECTION),
            (chunk, STATISTICS_JOB_DATA),
            (0x18, STATISTICS_INTERNAL),
            (0xc, STATISTICS_SIMULATION),
            (0x14, STATISTICS_DIRTY_ISLAND_POINTERS),
            (0xc, STATISTICS_MAINTENANCE_MANAGER),
            (chunk, STATISTICS_OPERATION_QUEUE),
            (0x18, STATISTICS_OPERATION_QUEUE),
            (0x14, STATISTICS_ISLAND_MERGES),
            (0x14, STATISTICS_BODY_OPERATIONS),
            (0x1c, 0),
            (0xc, STATISTICS_BROAD_PHASE_BORDER),
            (0x18, STATISTICS_COLLIDE),
            (0xc, STATISTICS_BROAD_PHASE),
            (chunk, STATISTICS_BROAD_PHASE_DISPATCH),
            (chunk, STATISTICS_BROAD_PHASE_DISPATCH),
            (chunk, STATISTICS_BROAD_PHASE_DISPATCH),
            (chunk, STATISTICS_BROAD_PHASE_DISPATCH),
            (chunk, STATISTICS_COLLISION_INPUT),
            (chunk, STATISTICS_AGENT_CONFIG),
            (0xc, STATISTICS_FILTER),
            (0xc, STATISTICS_CONVEX_LIST_FILTER),
            (0xc, STATISTICS_DISPATCHER),
            (0x1c, 0),
            (0x1c, 0),
            (0x18, STATISTICS_LISTENERS),
            (0x14, STATISTICS_ENTITY_LISTENERS),
            (0x14, STATISTICS_PHANTOM_LISTENERS),
            (0x14, STATISTICS_CONSTRAINT_LISTENERS),
            (0x14, STATISTICS_DELETION_LISTENERS),
            (0x14, STATISTICS_ISLAND_ACTIVATION_LISTENERS),
            (0x14, STATISTICS_POST_SIMULATION_LISTENERS),
            (0x14, STATISTICS_POST_INTEGRATE_LISTENERS),
            (0x14, STATISTICS_POST_COLLIDE_LISTENERS),
            (0x14, STATISTICS_POST_INTEGRATE_LISTENERS),
            (0x14, STATISTICS_COLLISION_LISTENERS),
            (0x1c, 0),
        ];
        assert_eq!(report_names(&reports), expected);
        // The array reports carry (3, name, data, used bytes, allocated).
        assert_eq!(
            reports.borrow()[0].1,
            vec![3, STATISTICS_ACTIVE_ISLANDS, 0, 0, 0x99]
        );
        // The collision input is the collector's current object only during the walk.
        assert_eq!(rig.e.mem.u32(collector + 8), 0);
    }

    #[test]
    fn calc_content_statistics_walks_the_islands_phantoms_sections_and_queues() {
        let (mut rig, _) = batch_rig();
        let t = rig.world.addr();
        let input = rig.e.mem.alloc(0x100);
        let operations = rig.e.mem.alloc(0x40);
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(input));
        rig.e.set(
            rig.world,
            HkpWorld::m_pendingOperations,
            Ptr::new(operations),
        );
        let active = array_of(&mut rig.e, &[0xa1, 0xa2]);
        set_array(&mut rig.e, t + 0x28, active, 2, 2);
        let inactive = array_of(&mut rig.e, &[0xb1]);
        set_array(&mut rig.e, t + 0x34, inactive, 1, 0x8000_0001);
        let phantoms = array_of(&mut rig.e, &[0xc1, 0xc2]);
        set_array(&mut rig.e, t + 0x110, phantoms, 2, 2);
        rig.e.mem.set_u32(t + 0xb8, 0x3000);
        rig.e.mem.set_u32(t + 0xc4, 0x3100);
        // Two queues, chained through the word at +8; every round reports the world's first.
        let queue = rig.e.mem.alloc(0x20);
        let second_queue = rig.e.mem.alloc(0x20);
        rig.e.mem.set_u32(queue + 8, second_queue);
        rig.e.mem.set_u32(t + 0xa0, queue);
        let (collector, reports) = statistics_collector(&mut rig.e);
        rig.e.mem.set_u8(collector + 0x10, 1);
        rig.e.call(0x00c9_2db0, &args![rig.world, collector, 0u32]);
        let reports = reports.borrow().clone();
        // The inactive array's storage is not owned, so it is reported only because the
        // collector's byte at +0x10 is set. Reports 1 and 2 are the active islands.
        assert_eq!(
            reports[3].1,
            vec![3, STATISTICS_INACTIVE_ISLANDS, inactive, 4, 0x99]
        );
        let children: Vec<(u32, u32)> = reports
            .iter()
            .filter(|(kind, words)| {
                *kind == 0xc
                    && [STATISTICS_ACTIVE, STATISTICS_INACTIVE, STATISTICS_PHANTOMS]
                        .contains(&words[0])
            })
            .map(|(_, words)| (words[0], words[1]))
            .collect();
        assert_eq!(
            children,
            vec![
                (STATISTICS_ACTIVE, 0xa2),
                (STATISTICS_ACTIVE, 0xa1),
                (STATISTICS_INACTIVE, 0xb1),
                (STATISTICS_PHANTOMS, 0xc1),
                (STATISTICS_PHANTOMS, 0xc2),
            ]
        );
        let chunks: Vec<Vec<u32>> = reports
            .iter()
            .filter(|(kind, _)| *kind == ADD_NORMAL_CHUNK)
            .map(|(_, words)| words.clone())
            .collect();
        assert!(chunks.contains(&vec![STATISTICS_MODIFY_CONSTRAINT_SECTION, 0x3000, 0x18, 0]));
        assert!(chunks.contains(&vec![STATISTICS_PROPERTY_MASTER_LOCK, 0x3100, 0x18, 0]));
        let pending_chunks: Vec<&Vec<u32>> = chunks
            .iter()
            .filter(|words| words[0] == STATISTICS_PENDING_QUEUES)
            .collect();
        assert_eq!(pending_chunks.len(), 2);
        assert_eq!(pending_chunks[0][1..], [queue, 0x10, 0]);
        assert_eq!(pending_chunks[1][1..], [queue, 0x10, 0]);
    }

    #[test]
    fn calc_content_statistics_skips_arrays_with_borrowed_storage() {
        let (mut rig, _) = batch_rig();
        let input = rig.e.mem.alloc(0x100);
        let operations = rig.e.mem.alloc(0x40);
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(input));
        rig.e.set(
            rig.world,
            HkpWorld::m_pendingOperations,
            Ptr::new(operations),
        );
        let t = rig.world.addr();
        set_array(&mut rig.e, t + 0x28, 0x4000, 0, 0x8000_0004);
        let (collector, reports) = statistics_collector(&mut rig.e);
        rig.e.call(0x00c9_2db0, &args![rig.world, collector, 0u32]);
        let names = report_names(&reports);
        assert_eq!(names[0], (0x14, STATISTICS_INACTIVE_ISLANDS));
        assert!(!names.contains(&(0x14, STATISTICS_ACTIVE_ISLANDS)));
    }

    #[test]
    fn get_memory_watch_dog_returns_the_field() {
        let mut e = Engine::new();
        let world = world(&mut e);
        assert_eq!(e.call(0x00c9_3570, &args![world]).u32(), 0);
        e.set(world, HkpWorld::m_memoryWatchDog, Ptr::new(0x6000));
        assert_eq!(e.call(0x00c9_3570, &args![world]).u32(), 0x6000);
    }

    /// A world with one active island holding `active_entity`, one inactive island holding
    /// `inactive_entity`, a fixed island holding the fixed body and one phantom. The doubles
    /// for `removePhantom` / `removeEntity` empty the matching list.
    fn removal_world(rig: &mut Rig) -> (u32, u32) {
        silent(
            &mut rig.e,
            &[
                HOOK_A,
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                ADD_ENTITY,
                REMOVE_ISLAND_FROM_DIRTY_LIST,
                FIRE_WORLD_REMOVE_ALL,
            ],
        );
        let t = rig.world.addr();
        let active_island = object_with_vtable(&mut rig.e, 0x80, &[(0, HOOK_A)]);
        let inactive_island = rig.e.mem.alloc(0x80);
        let fixed_island = rig.e.mem.alloc(0x80);
        for (island, entity) in [
            (active_island, 0xe1),
            (inactive_island, 0xe2),
            (fixed_island, 0xe3),
        ] {
            let entities = array_of(&mut rig.e, &[entity]);
            rig.e.mem.set_u32(island + 0x48, entities);
            rig.e.mem.set_u32(island + 0x4c, 1);
        }
        let active = array_of(&mut rig.e, &[active_island]);
        set_array(&mut rig.e, t + 0x28, active, 1, 1);
        let inactive = array_of(&mut rig.e, &[inactive_island]);
        set_array(&mut rig.e, t + 0x34, inactive, 1, 1);
        rig.e.mem.set_u32(t + 0x20, fixed_island);
        rig.e.mem.set_u32(t + 0x24, 0xe3);
        let phantoms = array_of(&mut rig.e, &[0xf1]);
        set_array(&mut rig.e, t + 0x110, phantoms, 1, 1);
        rig.e.register_double(REMOVE_PHANTOM, move |e, words| {
            assert_eq!(words[1], 0xf1);
            e.mem.set_u32(words[0] + 0x114, 0);
            Ret::default()
        });
        rig.e.register_double(REMOVE_ENTITY, move |e, words| {
            let world = words[0];
            match words[2] {
                0xe1 => e.mem.set_u32(active_island + 0x4c, 0),
                0xe2 => e.mem.set_u32(world + 0x38, 0),
                _ => e.mem.set_u32(fixed_island + 0x4c, 0),
            }
            Ret::default()
        });
        (active_island, fixed_island)
    }

    #[test]
    fn remove_all_empties_the_world_and_deletes_the_last_active_island() {
        let (mut rig, _) = batch_rig();
        let (active_island, _) = removal_world(&mut rig);
        let t = rig.world.addr();
        let deleted = recording(&mut rig.e, HOOK_A);
        rig.e.call(0x00c9_3580, &args![rig.world]);
        let removed = calls_to(&rig.e, REMOVE_ENTITY);
        let entities: Vec<u32> = removed.iter().map(|words| words[2]).collect();
        assert_eq!(entities, vec![0xe1, 0xe2, 0xe3]);
        assert_eq!(
            order_of(
                &rig.e,
                &[
                    REMOVE_PHANTOM,
                    REMOVE_ENTITY,
                    ADD_REFERENCE,
                    ADD_ENTITY,
                    REMOVE_REFERENCE,
                    REMOVE_ISLAND_FROM_DIRTY_LIST,
                    FIRE_WORLD_REMOVE_ALL
                ]
            ),
            vec![
                REMOVE_PHANTOM,
                REMOVE_ENTITY,
                REMOVE_ENTITY,
                ADD_REFERENCE,
                REMOVE_ENTITY,
                ADD_ENTITY,
                REMOVE_REFERENCE,
                REMOVE_ISLAND_FROM_DIRTY_LIST,
                FIRE_WORLD_REMOVE_ALL
            ]
        );
        assert_eq!(calls_to(&rig.e, ADD_REFERENCE), vec![vec![0xe3]]);
        assert_eq!(calls_to(&rig.e, ADD_ENTITY), vec![vec![t, 0xe3, 1]]);
        assert_eq!(calls_to(&rig.e, REMOVE_REFERENCE), vec![vec![0xe3]]);
        assert_eq!(
            calls_to(&rig.e, REMOVE_ISLAND_FROM_DIRTY_LIST),
            vec![vec![t, active_island]]
        );
        assert_eq!(deleted.borrow().clone(), vec![vec![active_island, 1]]);
        assert_eq!(rig.e.mem.i32(t + 0x2c), 0);
        assert_eq!(calls_to(&rig.e, FIRE_WORLD_REMOVE_ALL), vec![vec![t]]);
    }

    #[test]
    fn remove_all_keeps_the_island_list_when_the_world_adds_to_a_new_island() {
        let (mut rig, _) = batch_rig();
        removal_world(&mut rig);
        rig.e.set(rig.world, HkpWorld::m_addsToNewIsland, true);
        rig.e.call(0x00c9_3580, &args![rig.world]);
        assert!(calls_to(&rig.e, REMOVE_ISLAND_FROM_DIRTY_LIST).is_empty());
        assert_eq!(rig.e.mem.i32(rig.world.addr() + 0x2c), 1);
        assert_eq!(calls_to(&rig.e, FIRE_WORLD_REMOVE_ALL).len(), 1);
    }

    #[test]
    fn remove_all_stops_at_an_empty_first_island() {
        let (mut rig, _) = batch_rig();
        let (active_island, _) = removal_world(&mut rig);
        rig.e.mem.set_u32(active_island + 0x4c, 0);
        rig.e.call(0x00c9_3580, &args![rig.world]);
        let entities: Vec<u32> = calls_to(&rig.e, REMOVE_ENTITY)
            .iter()
            .map(|words| words[2])
            .collect();
        assert_eq!(entities, vec![0xe2, 0xe3]);
    }

    /// Calls the listener-adding function at `address` twice and checks the pointer array at
    /// `offset` of the world.
    fn check_pointer_array(address: u32, offset: u32) {
        let (mut rig, _) = batch_rig();
        rig.e.call(address, &args![rig.world, 0x1111u32]);
        rig.e.call(address, &args![rig.world, 0x2222u32]);
        let array = rig.world.addr() + offset;
        assert_eq!(rig.e.mem.u32(array + 4), 2);
        let data = rig.e.mem.u32(array);
        assert_eq!(rig.e.mem.u32(data), 0x1111);
        assert_eq!(rig.e.mem.u32(data + 4), 0x2222);
    }

    #[test]
    fn fn_00c937a0_appends_to_the_array_at_11c() {
        check_pointer_array(0x00c9_37a0, 0x11c);
    }

    #[test]
    fn fn_00c937e0_appends_to_the_array_at_140() {
        check_pointer_array(0x00c9_37e0, 0x140);
    }

    #[test]
    fn fn_00c93820_appends_to_the_array_at_128() {
        check_pointer_array(0x00c9_3820, 0x128);
    }

    #[test]
    fn fn_00c93860_appends_to_the_array_at_134() {
        check_pointer_array(0x00c9_3860, 0x134);
    }

    #[test]
    fn fn_00c938a0_appends_to_the_array_at_158() {
        check_pointer_array(0x00c9_38a0, 0x158);
    }

    #[test]
    fn add_world_post_simulation_listener_appends_to_the_array_at_164() {
        check_pointer_array(0x00c9_3920, 0x164);
    }

    #[test]
    fn fn_00c93a20_appends_to_the_array_at_1a0() {
        check_pointer_array(0x00c9_3a20, 0x1a0);
    }

    #[test]
    fn fn_00c93a60_appends_to_the_array_at_14c() {
        check_pointer_array(0x00c9_3a60, 0x14c);
    }

    #[test]
    fn step_multithreaded_returns_a_failed_begin_at_once() {
        let (mut rig, _) = batch_rig();
        silent(&mut rig.e, &[LOCK_ENTER, LOCK_LEAVE]);
        returning(&mut rig.e, STEP_BEGIN_ST, 5);
        let simulation = rig.e.mem.alloc(0x40);
        rig.e
            .set(rig.world, HkpWorld::m_simulation, Ptr::new(simulation));
        let calls = recording(&mut rig.e, HOOK_A);
        let listener = object_with_vtable(&mut rig.e, 0x20, &[(0xc, HOOK_A), (0x10, HOOK_A)]);
        let result = rig
            .e
            .call(0x00c9_3ae0, &args![rig.world, 0x77u32, listener, 2.0f32])
            .u32();
        assert_eq!(result, 5);
        assert!(calls.borrow().is_empty());
        assert_eq!(
            calls_to(&rig.e, STEP_BEGIN_ST),
            vec![vec![simulation, 0x77, 2.0f32.to_bits()]]
        );
    }

    #[test]
    fn step_multithreaded_runs_the_jobs_between_the_listener_calls() {
        let (mut rig, _) = batch_rig();
        silent(&mut rig.e, &[LOCK_ENTER, LOCK_LEAVE, PROCESS_ALL_JOBS]);
        returning(&mut rig.e, STEP_BEGIN_ST, 0);
        returning(&mut rig.e, SIMULATION_FINISH_MT_STEP, 9);
        let simulation = rig.e.mem.alloc(0x40);
        rig.e
            .set(rig.world, HkpWorld::m_simulation, Ptr::new(simulation));
        silent(&mut rig.e, &[HOOK_A, HOOK_B]);
        let listener = object_with_vtable(&mut rig.e, 0x20, &[(0xc, HOOK_A), (0x10, HOOK_B)]);
        let result = rig
            .e
            .call(0x00c9_3ae0, &args![rig.world, 0x77u32, listener, 2.0f32])
            .u32();
        assert_eq!(result, 9);
        assert_eq!(
            order_of(
                &rig.e,
                &[
                    STEP_BEGIN_ST,
                    HOOK_A,
                    PROCESS_ALL_JOBS,
                    HOOK_B,
                    SIMULATION_FINISH_MT_STEP
                ]
            ),
            vec![
                STEP_BEGIN_ST,
                HOOK_A,
                PROCESS_ALL_JOBS,
                HOOK_B,
                SIMULATION_FINISH_MT_STEP
            ]
        );
        assert_eq!(calls_to(&rig.e, HOOK_A), vec![vec![listener, 0x77, 0xb]]);
        assert_eq!(calls_to(&rig.e, PROCESS_ALL_JOBS), vec![vec![0x77]]);
        assert_eq!(calls_to(&rig.e, HOOK_B), vec![vec![listener]]);
        assert_eq!(
            calls_to(&rig.e, SIMULATION_FINISH_MT_STEP),
            vec![vec![simulation, 0x77, listener]]
        );
    }

    /// The physics system constructor hands back the block it is given.
    fn system_constructor(rig: &mut Rig) {
        rig.e.register(PHYSICS_SYSTEM_CONSTRUCT, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        silent(
            &mut rig.e,
            &[
                PHYSICS_SYSTEM_ADD_RIGID_BODY,
                PHYSICS_SYSTEM_ADD_CONSTRAINT,
                PHYSICS_SYSTEM_ADD_ACTION,
                PHYSICS_SYSTEM_ADD_PHANTOM,
            ],
        );
    }

    #[test]
    fn fn_00c93b30_builds_one_system_for_a_world_without_fixed_or_inactive_bodies() {
        let (mut rig, _) = batch_rig();
        system_constructor(&mut rig);
        let systems = rig.e.mem.alloc(0x10);
        set_array(&mut rig.e, systems, 0, 0, 0x8000_0000);
        rig.e.call(0x00c9_3b30, &args![rig.world, systems]);
        assert_eq!(rig.e.mem.u32(systems + 4), 1);
        let data = rig.e.mem.u32(systems);
        let system = rig.e.mem.u32(data);
        assert_eq!(rig.e.mem.u16(system + 4), 0x44);
        assert_eq!(
            calls_to(&rig.e, PHYSICS_SYSTEM_CONSTRUCT),
            vec![vec![system]]
        );
    }

    #[test]
    fn fn_00c93b30_builds_a_second_system_when_the_first_got_bodies() {
        let (mut rig, _) = batch_rig();
        system_constructor(&mut rig);
        let fixed_island = rig.e.mem.alloc(0x80);
        let body = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(body + 0x10, 1);
        let bodies = array_of(&mut rig.e, &[body]);
        rig.e.mem.set_u32(fixed_island + 0x48, bodies);
        rig.e.mem.set_u32(fixed_island + 0x4c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_fixedIsland, Ptr::new(fixed_island));
        let systems = rig.e.mem.alloc(0x10);
        set_array(&mut rig.e, systems, 0, 0, 0x8000_0000);
        rig.e.call(0x00c9_3b30, &args![rig.world, systems]);
        assert_eq!(rig.e.mem.u32(systems + 4), 2);
        let data = rig.e.mem.u32(systems);
        let first = rig.e.mem.u32(data);
        let second = rig.e.mem.u32(data + 4);
        assert_ne!(first, second);
        assert_eq!(rig.e.mem.u8(first + 0x40), 0);
        assert_eq!(
            calls_to(&rig.e, PHYSICS_SYSTEM_ADD_RIGID_BODY),
            vec![vec![first, body]]
        );
        assert_eq!(calls_to(&rig.e, PHYSICS_SYSTEM_CONSTRUCT).len(), 2);
    }

    #[test]
    fn fn_00c93b30_builds_again_when_the_first_allocation_is_null() {
        let (mut rig, _) = batch_rig();
        system_constructor(&mut rig);
        let mut first = true;
        rig.e
            .register_double(PHYSICS_SYSTEM_CONSTRUCT, move |_, words| {
                let result = if first { 0 } else { words[0] };
                first = false;
                Ret {
                    eax: result,
                    ..Ret::default()
                }
            });
        let systems = rig.e.mem.alloc(0x10);
        set_array(&mut rig.e, systems, 0, 0, 0x8000_0000);
        rig.e.call(0x00c9_3b30, &args![rig.world, systems]);
        // The first system is null and the world had no bodies for it: only the second one is
        // built and stored.
        assert_eq!(rig.e.mem.u32(systems + 4), 1);
        let data = rig.e.mem.u32(systems);
        assert_ne!(rig.e.mem.u32(data), 0);
        assert_eq!(calls_to(&rig.e, PHYSICS_SYSTEM_CONSTRUCT).len(), 2);
    }

    #[test]
    fn fn_00c93d10_builds_the_empty_world_tables() {
        let mut e = Engine::new();
        let world = world(&mut e);
        let t = world.addr();
        let arrays = [
            0x28u32, 0x34, 0x40, 0x11c, 0x128, 0x134, 0x140, 0x14c, 0x158, 0x164, 0x170, 0x17c,
            0x188, 0x194, 0x1a0, 0x1ac,
        ];
        for &array in &arrays {
            for word in 0..3 {
                e.mem.set_u32(t + array + 4 * word, 0xcccc_cccc);
            }
        }
        e.mem.set_u32(t + 0x50, 0xcccc_cccc);
        let returned = e.call(0x00c9_3d10, &args![world, 0u32]).u32();
        assert_eq!(returned, t);
        assert_eq!(e.mem.u32(t), 0x010c_6714);
        assert_eq!(e.mem.u16(t + 6), 1);
        assert_eq!(e.mem.u32(t + 0x50), 0);
        assert_eq!(e.mem.u32(t + 0xa8), 0xffff_fff1);
        assert_eq!(e.mem.u16(t + 0xac), 0x8000);
        for &array in &arrays {
            assert_eq!(e.mem.u32(t + array), 0);
            assert_eq!(e.mem.u32(t + array + 4), 0);
            assert_eq!(e.mem.u32(t + array + 8), 0x8000_0000);
        }
    }

    /// Everything the world is torn down with; the references and objects are made-up words.
    fn full_world(rig: &mut Rig) -> [u32; 4] {
        let t = rig.world.addr();
        silent(
            &mut rig.e,
            &[
                REMOVE_REFERENCE,
                DELETE_CRITICAL_SECTION,
                REMOVE_ALL,
                REMOVE_ENTITY,
                REMOVE_ISLAND_FROM_DIRTY_LIST,
                FIRE_WORLD_DELETED,
                DISPATCH_RECORD_DESTRUCT,
                OPERATION_QUEUE_DESTRUCT,
            ],
        );
        let job_data = rig.e.mem.alloc(0x120);
        let job_buffer = rig.e.mem.alloc(0x200);
        rig.e.mem.set_u32(job_data + 0xf4, job_buffer);
        rig.e.mem.set_u32(t + 0x74, job_data);
        for (offset, value) in [
            (0x4c, 0x101),
            (0x50, 0x102),
            (0x58, 0x103),
            (0x80, 0x104),
            (0x1bc, 0x105),
            (0x7c, 0x106),
            (0x84, 0x107),
            (8, 0x108),
            (0x5c, 0x109),
            (0xb8, 0x201),
            (0xc4, 0x202),
            (0xc0, 0x203),
            (0x24, 0x301),
            (0x64, 0x401),
            (0x88, 0x402),
            (0x1b8, 0x403),
        ] {
            rig.e.mem.set_u32(t + offset, value);
        }
        let fixed_island = object_with_vtable(&mut rig.e, 0x40, &[(0, HOOK_A)]);
        rig.e.mem.set_u32(t + 0x20, fixed_island);
        let mut objects = [0u32; 3];
        for (slot, offset) in objects.iter_mut().zip([0x68, 0x6c, 0x70]) {
            *slot = object_with_vtable(&mut rig.e, 0x40, &[(0, HOOK_A)]);
            rig.e.mem.set_u32(t + offset, *slot);
        }
        let input = rig.e.mem.alloc(0x80);
        rig.e.mem.set_u32(input + 0x70, 0x501);
        rig.e.mem.set_u32(input + 0x64, 0x502);
        rig.e.mem.set_u32(t + 0x78, input);
        mark_arrays_unallocated(&mut rig.e, t);
        set_array(&mut rig.e, t + 0x1ac, 0x601, 2, 3);
        set_array(&mut rig.e, t + 0x28, 0x602, 1, 2);
        [fixed_island, objects[0], objects[1], objects[2]]
    }

    #[test]
    fn world_destructor_tears_down_every_part_in_order() {
        let (mut rig, freed) = batch_rig();
        let t = rig.world.addr();
        let [fixed_island, first, second, third] = full_world(&mut rig);
        let deleted = recording(&mut rig.e, HOOK_A);
        rig.e.call(0x00c9_3e50, &args![rig.world]);
        assert_eq!(
            calls_to(&rig.e, REMOVE_REFERENCE),
            (0x101..=0x109)
                .map(|object| vec![object])
                .collect::<Vec<_>>()
        );
        assert_eq!(
            calls_to(&rig.e, DELETE_CRITICAL_SECTION),
            vec![vec![0x201], vec![0x202], vec![0x203]]
        );
        assert_eq!(
            order_of(
                &rig.e,
                &[
                    DELETE_CRITICAL_SECTION,
                    REMOVE_ALL,
                    REMOVE_ENTITY,
                    REMOVE_ISLAND_FROM_DIRTY_LIST,
                    FIRE_WORLD_DELETED,
                    DISPATCH_RECORD_DESTRUCT,
                    OPERATION_QUEUE_DESTRUCT
                ]
            ),
            vec![
                DELETE_CRITICAL_SECTION,
                DELETE_CRITICAL_SECTION,
                DELETE_CRITICAL_SECTION,
                REMOVE_ALL,
                REMOVE_ENTITY,
                REMOVE_ISLAND_FROM_DIRTY_LIST,
                FIRE_WORLD_DELETED,
                DISPATCH_RECORD_DESTRUCT,
                OPERATION_QUEUE_DESTRUCT
            ]
        );
        assert_eq!(calls_to(&rig.e, REMOVE_ALL), vec![vec![t]]);
        assert_eq!(calls_to(&rig.e, REMOVE_ENTITY)[0][0], t);
        assert_eq!(calls_to(&rig.e, REMOVE_ENTITY)[0][2], 0x301);
        assert_eq!(
            calls_to(&rig.e, REMOVE_ISLAND_FROM_DIRTY_LIST),
            vec![vec![t, fixed_island]]
        );
        assert_eq!(calls_to(&rig.e, FIRE_WORLD_DELETED), vec![vec![t]]);
        assert_eq!(
            calls_to(&rig.e, DISPATCH_RECORD_DESTRUCT),
            vec![vec![0x401]]
        );
        assert_eq!(
            calls_to(&rig.e, OPERATION_QUEUE_DESTRUCT),
            vec![vec![0x402]]
        );
        assert_eq!(
            deleted.borrow().clone(),
            vec![
                vec![fixed_island, 1],
                vec![first, 1],
                vec![second, 1],
                vec![third, 1]
            ]
        );
        let blocks: Vec<(u32, u32)> = freed
            .borrow()
            .iter()
            .map(|words| (words[1], words[2]))
            .collect();
        let job_data = blocks[1].0;
        assert_eq!(
            blocks,
            vec![
                (blocks[0].0, 0x200),
                (job_data, 0x120),
                (0x201, 0x18),
                (0x202, 0x18),
                (0x203, 0x18),
                (0x401, 0x10c),
                (0x501, 0x10),
                (0x502, 0x200),
                (blocks[8].0, 0x80),
                (0x402, 0x28),
                (0x403, 0x204),
            ]
        );
        let allocator = rig.e.mem.u32(rig.router + 0x10);
        assert_eq!(
            rig.freed_first.borrow().clone(),
            vec![vec![allocator, 0x601, 12], vec![allocator, 0x602, 8]]
        );
        assert_eq!(rig.e.mem.u32(t), REFERENCED_OBJECT_VTABLE);
        for offset in [0x4c, 0x50, 0xb8, 0xc4, 0xc0, 0x24, 0x20, 0x58, 0x80] {
            assert_eq!(rig.e.mem.u32(t + offset), 0, "offset {offset:#x}");
        }
        for offset in [0x1ac, 0x28, 0x40, 0x110] {
            assert_eq!(rig.e.mem.u32(t + offset), 0);
            assert_eq!(rig.e.mem.u32(t + offset + 4), 0);
            assert_eq!(rig.e.mem.u32(t + offset + 8), 0x8000_0000);
        }
    }

    #[test]
    fn world_destructor_skips_what_a_bare_world_does_not_have() {
        let (mut rig, freed) = batch_rig();
        let t = rig.world.addr();
        silent(
            &mut rig.e,
            &[
                REMOVE_REFERENCE,
                REMOVE_ALL,
                REMOVE_ENTITY,
                REMOVE_ISLAND_FROM_DIRTY_LIST,
                FIRE_WORLD_DELETED,
            ],
        );
        mark_arrays_unallocated(&mut rig.e, t);
        let input = rig.e.mem.alloc(0x80);
        rig.e.mem.set_u32(t + 0x78, input);
        rig.e.call(0x00c9_3e50, &args![rig.world]);
        assert_eq!(calls_to(&rig.e, REMOVE_REFERENCE), vec![vec![0]; 6]);
        assert_eq!(
            calls_to(&rig.e, REMOVE_ISLAND_FROM_DIRTY_LIST),
            vec![vec![t, 0]]
        );
        let blocks: Vec<(u32, u32)> = freed
            .borrow()
            .iter()
            .map(|words| (words[1], words[2]))
            .collect();
        assert_eq!(blocks, vec![(input, 0x80)]);
        assert!(rig.freed_first.borrow().is_empty());
        assert_eq!(rig.e.mem.u32(t), REFERENCED_OBJECT_VTABLE);
    }
    /// A broad phase object (installed in the world) whose virtual function 0x44 (`HOOK_A`)
    /// writes the handles in `handles` into the pair array it is given, and whose functions
    /// 0x54 and 0x58 (`HOOK_E`, `HOOK_F`) do nothing.
    fn install_broad_phase(rig: &mut Rig, handles: Vec<u32>) -> u32 {
        rig.e.register_double(HOOK_A, move |e, words| {
            let pairs = words[2];
            let data = e.mem.u32(pairs);
            for (index, handle) in handles.iter().enumerate() {
                e.mem.set_u32(data + 8 * index as u32 + 4, *handle);
            }
            e.mem.set_u32(pairs + 4, handles.len() as u32);
            Ret::default()
        });
        silent(&mut rig.e, &[HOOK_E, HOOK_F]);
        let broad_phase = object_with_vtable(
            &mut rig.e,
            0x20,
            &[(0x44, HOOK_A), (0x54, HOOK_E), (0x58, HOOK_F)],
        );
        rig.e
            .set(rig.world, HkpWorld::m_broadPhase, Ptr::new(broad_phase));
        broad_phase
    }

    #[test]
    fn shift_broad_phase_moves_the_border_and_world_vectors() {
        let (mut rig, _) = batch_rig();
        let t = rig.world.addr();
        let broad_phase = install_broad_phase(&mut rig, vec![]);
        let input = rig.e.mem.alloc(0x100);
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(input));
        let offset = rig.e.mem.alloc(0x10);
        for lane in 0..4 {
            rig.e.mem.set_f32(offset + 4 * lane, 1.0 + lane as f32);
        }
        for (vector, base) in [(t + 0x310, 10.0f32), (t + 0x320, 20.0)] {
            for lane in 0..4 {
                rig.e.mem.set_f32(vector + 4 * lane, base + lane as f32);
            }
        }
        // Six border objects whose kind (virtual function 0x14) is 0, 1, 2, 3, 0 and 5.
        let border = rig.e.mem.alloc(0x60);
        let mut kinds = vec![0u32, 1, 2, 3, 0, 5].into_iter();
        rig.e.register_double(HOOK_D, move |_, _| Ret {
            eax: kinds.next().unwrap(),
            ..Ret::default()
        });
        let mut objects = vec![];
        for slot in 0..6 {
            let object = object_with_vtable(&mut rig.e, 0x100, &[(0x14, HOOK_D)]);
            for vector in [0xb0, 0xc0, 0xe0] {
                for lane in 0..4 {
                    rig.e.mem.set_f32(object + vector + 4 * lane, 100.0);
                }
            }
            rig.e.mem.set_u32(border + 0x18 + 4 * slot, object);
            objects.push(object);
        }
        rig.e
            .set(rig.world, HkpWorld::m_broadPhaseBorder, Ptr::new(border));
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(0x7100));
        rig.e.set(
            rig.world,
            HkpWorld::m_broadPhaseDispatcher,
            Ptr::new(0x7200),
        );
        let seen_lock = Rc::new(RefCell::new(vec![]));
        let sink = seen_lock.clone();
        rig.e.register_double(ADD_PAIRS, move |e, words| {
            sink.borrow_mut()
                .push((words.to_vec(), e.mem.i32(t + 0x94)));
            Ret::default()
        });
        rig.e.mem.set_u32(t + 0x94, 0);
        rig.e
            .call(0x00c9_6a20, &args![rig.world, 0x1234u32, offset, 1u32]);
        let lanes = |e: &Engine, address: u32| -> Vec<f32> {
            (0..4).map(|lane| e.mem.f32(address + 4 * lane)).collect()
        };
        assert_eq!(lanes(&rig.e, t + 0x310), vec![11.0, 13.0, 15.0, 17.0]);
        assert_eq!(lanes(&rig.e, t + 0x320), vec![21.0, 23.0, 25.0, 27.0]);
        let moved = vec![101.0, 102.0, 103.0, 104.0];
        let still = vec![100.0; 4];
        for (index, object) in objects.iter().enumerate() {
            let (first, second, third) = match index {
                0 | 4 => (&moved, &moved, &still),
                1 | 2 => (&still, &still, &moved),
                _ => (&still, &still, &still),
            };
            assert_eq!(&lanes(&rig.e, object + 0xb0), first);
            assert_eq!(&lanes(&rig.e, object + 0xc0), second);
            assert_eq!(&lanes(&rig.e, object + 0xe0), third);
        }
        assert_eq!(
            seen_lock.borrow().clone(),
            vec![(vec![0x7200, 0, 0, 0x7108], 1)]
        );
        assert_eq!(
            rig.e
                .get(rig.world, HkpWorld::m_criticalOperationsLockCount),
            0
        );
        let region = calls_to(&rig.e, HOOK_E);
        assert_eq!(region.len(), 1);
        assert_eq!(region[0][..3], [broad_phase, 0x1234, offset]);
        assert_eq!(
            calls_to(&rig.e, HOOK_F),
            vec![vec![broad_phase, input + 0x20, input + 0x30]]
        );
        assert_eq!(
            record_labels(&rig, &[]),
            vec![PROFILE_SHIFT_BROAD_PHASE, PROFILE_END]
        );
    }
    /// An island whose entities are blocks with the given shapes (the word at +0x10).
    fn island_with_bodies(e: &mut Engine, shapes: &[u32]) -> (u32, Vec<u32>) {
        let island = e.mem.alloc(0x80);
        let entities: Vec<u32> = shapes
            .iter()
            .map(|shape| {
                let entity = e.mem.alloc(0x40);
                e.mem.set_u32(entity + 0x10, *shape);
                entity
            })
            .collect();
        let data = array_of(e, &entities);
        e.mem.set_u32(island + 0x48, data);
        e.mem.set_u32(island + 0x4c, entities.len() as u32);
        (island, entities)
    }

    #[test]
    fn shift_broad_phase_recalculates_the_bounding_volumes_of_shaped_bodies() {
        let (mut rig, _) = batch_rig();
        let t = rig.world.addr();
        install_broad_phase(&mut rig, vec![]);
        let input = rig.e.mem.alloc(0x100);
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(input));
        let offset = rig.e.mem.alloc(0x10);
        let (fixed, fixed_bodies) = island_with_bodies(&mut rig.e, &[0, 0x71]);
        let (active, active_bodies) = island_with_bodies(&mut rig.e, &[0x72]);
        let (inactive, inactive_bodies) = island_with_bodies(&mut rig.e, &[0x73, 0]);
        rig.e.mem.set_u32(t + 0x20, fixed);
        let active_array = array_of(&mut rig.e, &[active]);
        set_array(&mut rig.e, t + 0x28, active_array, 1, 1);
        let inactive_array = array_of(&mut rig.e, &[inactive]);
        set_array(&mut rig.e, t + 0x34, inactive_array, 1, 1);
        let recalculated: Records = Rc::new(RefCell::new(vec![]));
        let sink = recalculated.clone();
        rig.e
            .register_double(ENTITY_BATCH_RECALC_AABB, move |e, words| {
                let mut seen = vec![words[0], words[2]];
                for index in 0..words[2] {
                    seen.push(e.mem.u32(words[1] + 4 * index));
                }
                sink.borrow_mut().push(seen);
                Ret::default()
            });
        rig.e
            .call(0x00c9_6a20, &args![rig.world, 0u32, offset, 0u32]);
        assert_eq!(
            recalculated.borrow().clone(),
            vec![vec![
                input,
                3,
                fixed_bodies[1],
                active_bodies[0],
                inactive_bodies[0]
            ]]
        );
        // The island buffer (3 islands) and the grown body array go back to the allocator.
        let freed = rig.freed_first.borrow().clone();
        assert_eq!(freed.len(), 2);
        assert_eq!(freed[0][2], 12);
        assert_eq!(freed[1][2], 16);
        assert_eq!(
            record_labels(&rig, &[]),
            vec![PROFILE_SHIFT_BROAD_PHASE, PROFILE_END]
        );
        assert_eq!(
            rig.e
                .get(rig.world, HkpWorld::m_criticalOperationsLockCount),
            0
        );
    }

    /// What `getPenetrations` and `getClosestPoints` are given: a collidable of shape type 2,
    /// a dispatcher table that sends a pair with a shape of type 3 to `HOOK_B` at the table
    /// offset `table_offset`, and a collector.
    struct Query {
        collidable: u32,
        self_handle: u32,
        shape: u32,
        transform: u32,
        query: u32,
        collector: u32,
        bounds_log: Records,
        dispatched: Records,
    }

    fn query_fixture(
        rig: &mut Rig,
        wanted: f32,
        margin: f32,
        table_offset: u32,
        accepts: u8,
    ) -> Query {
        rig.e.map(0x0101_2000, 0x1000);
        let bounds_log: Records = Rc::new(RefCell::new(vec![]));
        let sink = bounds_log.clone();
        rig.e.register_double(HOOK_C, move |_, words| {
            sink.borrow_mut().push(words.to_vec());
            Ret::default()
        });
        let shape = object_with_vtable(&mut rig.e, 0x20, &[(0x1c, HOOK_C)]);
        rig.e.mem.set_u32(shape + 0xc, 2);
        let block = rig.e.mem.alloc(0x60);
        rig.e.mem.set_i8(block + 5, 0x10);
        let collidable = block + 0x10;
        rig.e.mem.set_u32(collidable, shape);
        rig.e.mem.set_u32(collidable + 8, 0x7800);
        let table = rig.e.mem.alloc(0x1000);
        rig.e.mem.set_u8(table + (2 + 0xd) * 0x20 + 3, 1);
        rig.e.mem.set_u32(table + 0x14 + table_offset, HOOK_B);
        let query = rig.e.mem.alloc(0x20);
        rig.e.mem.set_u32(query, table);
        rig.e.mem.set_f32(query + 8, wanted);
        let input = rig.e.mem.alloc(0x100);
        rig.e.mem.set_f32(input + 8, margin);
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(input));
        let filter = filter_answering(&mut rig.e, accepts);
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(filter));
        let collector = rig.e.mem.alloc(0x10);
        let dispatched = recording(&mut rig.e, HOOK_B);
        Query {
            collidable,
            self_handle: block,
            shape,
            transform: 0x7800,
            query,
            collector,
            bounds_log,
            dispatched,
        }
    }

    /// A broad phase handle whose collidable (the handle plus the signed byte at +5, here
    /// 0x10) has a shape of type 3; returns (handle, collidable).
    fn shaped_handle(e: &mut Engine) -> (u32, u32) {
        let handle = e.mem.alloc(0x40);
        e.mem.set_i8(handle + 5, 0x10);
        let shape = e.mem.alloc(0x20);
        e.mem.set_u32(shape + 0xc, 3);
        e.mem.set_u32(handle + 0x10, shape);
        (handle, handle + 0x10)
    }

    #[test]
    fn get_penetrations_dispatches_every_accepted_pair() {
        let (mut rig, _) = batch_rig();
        let q = query_fixture(&mut rig, 2.0, 1.0, 0x9a4, 1);
        let (first_handle, first) = shaped_handle(&mut rig.e);
        let (second_handle, second) = shaped_handle(&mut rig.e);
        install_broad_phase(&mut rig, vec![first_handle, second_handle]);
        rig.e.call(
            0x00c9_6df0,
            &args![rig.world, q.collidable, q.query, q.collector],
        );
        assert_eq!(
            q.dispatched.borrow().clone(),
            vec![
                vec![q.collidable, first, q.query, q.collector],
                vec![q.collidable, second, q.query, q.collector]
            ]
        );
        let bounds = q.bounds_log.borrow().clone();
        assert_eq!(bounds.len(), 1);
        assert_eq!(bounds[0].len(), 4);
        assert_eq!(bounds[0][..3], [q.shape, q.transform, 1.5f32.to_bits()]);
        assert_eq!(
            record_labels(&rig, &[PROFILE_GET_PENETRATIONS]),
            vec![
                PROFILE_GET_PENETRATIONS,
                PROFILE_GET_PENETRATIONS_NARROW,
                PROFILE_SECTION_END
            ]
        );
        let freed = rig.freed_first.borrow().clone();
        assert!(freed.is_empty());
    }

    #[test]
    fn get_penetrations_stops_when_the_collector_says_so() {
        let (mut rig, _) = batch_rig();
        let q = query_fixture(&mut rig, 2.0, 1.0, 0x9a4, 1);
        let (first_handle, first) = shaped_handle(&mut rig.e);
        let (second_handle, _) = shaped_handle(&mut rig.e);
        install_broad_phase(&mut rig, vec![first_handle, second_handle]);
        let collector = q.collector;
        rig.e.register_double(HOOK_B, move |e, _| {
            e.mem.set_u8(collector + 4, 1);
            Ret::default()
        });
        rig.e.call(
            0x00c9_6df0,
            &args![rig.world, q.collidable, q.query, q.collector],
        );
        assert_eq!(
            calls_to(&rig.e, HOOK_B),
            vec![vec![q.collidable, first, q.query, q.collector]]
        );
    }

    #[test]
    fn get_penetrations_clamps_the_tolerance_and_skips_refused_pairs() {
        let (mut rig, _) = batch_rig();
        // 0.25 - 0.5 * 1.0 is negative: the tolerance is 0.
        let q = query_fixture(&mut rig, 0.25, 1.0, 0x9a4, 0);
        let (handle, _) = shaped_handle(&mut rig.e);
        install_broad_phase(&mut rig, vec![handle]);
        rig.e.call(
            0x00c9_6df0,
            &args![rig.world, q.collidable, q.query, q.collector],
        );
        assert!(q.dispatched.borrow().is_empty());
        assert_eq!(q.bounds_log.borrow()[0][2], 0);
    }

    #[test]
    fn get_penetrations_skips_its_own_collidable_and_shapeless_ones() {
        let (mut rig, _) = batch_rig();
        let q = query_fixture(&mut rig, 2.0, 1.0, 0x9a4, 1);
        // A handle whose collidable is the query's own collidable.
        let own = q.self_handle;
        // A handle whose collidable has no shape.
        let (shapeless, shapeless_collidable) = shaped_handle(&mut rig.e);
        rig.e.mem.set_u32(shapeless_collidable, 0);
        install_broad_phase(&mut rig, vec![own, shapeless]);
        rig.e.call(
            0x00c9_6df0,
            &args![rig.world, q.collidable, q.query, q.collector],
        );
        assert!(q.dispatched.borrow().is_empty());
    }

    #[test]
    fn get_closest_points_dispatches_the_broad_phase_pairs_through_the_second_table() {
        let (mut rig, _) = batch_rig();
        let q = query_fixture(&mut rig, 2.0, 1.0, 0x9a8, 1);
        let (first_handle, first) = shaped_handle(&mut rig.e);
        let (second_handle, second) = shaped_handle(&mut rig.e);
        install_broad_phase(&mut rig, vec![first_handle, second_handle]);
        // The collector's stop byte does not end this search.
        rig.e.mem.set_u8(q.collector + 4, 1);
        rig.e.call(
            0x00c9_7850,
            &args![rig.world, q.collidable, q.query, q.collector],
        );
        assert_eq!(
            q.dispatched.borrow().clone(),
            vec![
                vec![q.collidable, first, q.query, q.collector],
                vec![q.collidable, second, q.query, q.collector]
            ]
        );
        assert_eq!(q.bounds_log.borrow()[0][2], 1.5f32.to_bits());
        assert_eq!(
            record_labels(&rig, &[PROFILE_CLOSEST_BROAD_PHASE]),
            vec![
                PROFILE_CLOSEST_BROAD_PHASE,
                PROFILE_CLOSEST_NARROW,
                PROFILE_SECTION_END
            ]
        );
    }

    /// A tree manager (installed in the world) that reports itself in use, with the trees
    /// `aabb_tree` and `kd_tree` (zero for none).
    fn tree_manager(rig: &mut Rig, aabb_tree: u32, kd_tree: u32) {
        returning(&mut rig.e, HOOK_D, 1);
        let manager = object_with_vtable(&mut rig.e, 0x20, &[(0x10, HOOK_D)]);
        rig.e.mem.set_u32(manager + 0xc, aabb_tree);
        rig.e.mem.set_u32(manager + 0x10, kd_tree);
        rig.e
            .set(rig.world, HkpWorld::m_kdTreeManager, Ptr::new(manager));
    }

    #[test]
    fn get_closest_points_asks_both_trees_when_the_tree_manager_is_in_use() {
        let (mut rig, _) = batch_rig();
        let q = query_fixture(&mut rig, 2.0, 1.0, 0x9a8, 1);
        let (_, first) = shaped_handle(&mut rig.e);
        let (_, second) = shaped_handle(&mut rig.e);
        tree_manager(&mut rig, 0x7a00, 0x7b00);
        rig.e.register_double(AABB_TREE_QUERY, move |e, words| {
            let array = words[2];
            let data = e.mem.u32(array);
            e.mem.set_u32(data, first);
            e.mem.set_u32(array + 4, 1);
            Ret::default()
        });
        let buffer = rig.e.mem.alloc(0x20);
        rig.e.register_double(KD_TREE_QUERY, move |e, words| {
            let array = words[2];
            e.mem.set_u32(buffer, second);
            e.mem.set_u32(array, buffer);
            e.mem.set_u32(array + 4, 1);
            e.mem.set_u32(array + 8, 4);
            Ret::default()
        });
        rig.e.call(
            0x00c9_7850,
            &args![rig.world, q.collidable, q.query, q.collector],
        );
        assert_eq!(
            q.dispatched.borrow().clone(),
            vec![
                vec![q.collidable, first, q.query, q.collector],
                vec![q.collidable, second, q.query, q.collector]
            ]
        );
        let aabb = calls_to(&rig.e, AABB_TREE_QUERY);
        assert_eq!(aabb.len(), 1);
        assert_eq!(aabb[0][0], 0x7a00);
        assert_eq!(calls_to(&rig.e, KD_TREE_QUERY)[0][0], 0x7b00);
        // The kd tree's array is freed through the router's second allocator.
        let second_allocator = rig.e.mem.u32(rig.router + 0xc);
        assert_eq!(
            rig.freed_second.borrow().clone(),
            vec![vec![second_allocator, buffer, 16]]
        );
        assert!(rig.freed_first.borrow().is_empty());
        assert_eq!(
            record_labels(&rig, &[PROFILE_CLOSEST_TREES]),
            vec![
                PROFILE_CLOSEST_TREES,
                PROFILE_CLOSEST_AABB_TREE_NARROW,
                PROFILE_CLOSEST_KD_TREE_BROAD_PHASE,
                PROFILE_CLOSEST_KD_TREE_NARROW,
                PROFILE_SECTION_END
            ]
        );
    }

    #[test]
    fn get_closest_points_without_trees_only_writes_the_records() {
        let (mut rig, _) = batch_rig();
        let q = query_fixture(&mut rig, 2.0, 1.0, 0x9a8, 1);
        tree_manager(&mut rig, 0, 0);
        rig.e.call(
            0x00c9_7850,
            &args![rig.world, q.collidable, q.query, q.collector],
        );
        assert!(q.dispatched.borrow().is_empty());
        assert_eq!(
            record_labels(&rig, &[PROFILE_CLOSEST_TREES]),
            vec![PROFILE_CLOSEST_TREES, PROFILE_SECTION_END]
        );
    }

    /// What `fn_00c976a0` works on.
    struct Removal {
        action: u32,
        entity: u32,
        action_island: u32,
        entity_island: u32,
        list: u32,
    }

    /// An entity whose action list holds `actions` (`None` stands for the action itself),
    /// whose island has the flag byte 0xFF, and an action whose virtual function 0x10
    /// (`HOOK_A`) lists the entities `before`, the entity itself and `after`.
    fn removal_fixture(
        rig: &mut Rig,
        actions: &[Option<u32>],
        before: Vec<u32>,
        after: Vec<u32>,
    ) -> Removal {
        silent(&mut rig.e, &[ISLAND_REMOVE_ACTION, ISLAND_ADD_ACTION]);
        let entity = rig.e.mem.alloc(0x300);
        let action_island = rig.e.mem.alloc(0x80);
        let mut listed = before;
        listed.push(entity);
        listed.extend(after);
        rig.e.register_double(HOOK_A, move |e, words| {
            let array = words[1];
            let data = e.mem.u32(array);
            for (index, member) in listed.iter().enumerate() {
                e.mem.set_u32(data + 4 * index as u32, *member);
            }
            e.mem.set_u32(array + 4, listed.len() as u32);
            Ret::default()
        });
        let action = object_with_vtable(&mut rig.e, 0x20, &[(0x10, HOOK_A)]);
        rig.e.mem.set_u32(action + 0xc, action_island);
        let words: Vec<u32> = actions.iter().map(|a| a.unwrap_or(action)).collect();
        let list = array_of(&mut rig.e, &words);
        rig.e.mem.set_u32(entity + 0x208, list);
        rig.e.mem.set_u16(entity + 0x20c, words.len() as u16);
        let entity_island = rig.e.mem.alloc(0x80);
        rig.e.mem.set_u8(entity_island + 0x25, 0xff);
        rig.e.mem.set_u32(entity + 0xcc, entity_island);
        Removal {
            action,
            entity,
            action_island,
            entity_island,
            list,
        }
    }

    /// An entity on an island whose word at +0x20 is `marker`; returns (entity, island).
    fn entity_on_island(e: &mut Engine, marker: u16) -> (u32, u32) {
        let entity = e.mem.alloc(0x300);
        let island = e.mem.alloc(0x80);
        e.mem.set_u16(island + 0x20, marker);
        e.mem.set_u32(entity + 0xcc, island);
        (entity, island)
    }

    #[test]
    fn fn_00c976a0_removes_the_action_and_moves_it_to_the_first_live_island() {
        let (mut rig, _) = batch_rig();
        let (unassigned, _) = entity_on_island(&mut rig.e, 0xffff);
        let (live, live_island) = entity_on_island(&mut rig.e, 0);
        let r = removal_fixture(
            &mut rig,
            &[Some(0xa1), None, Some(0xa3)],
            vec![unassigned],
            vec![live],
        );
        rig.e.call(0x00c9_76a0, &args![r.action, r.entity]);
        assert_eq!(rig.e.mem.u16(r.entity + 0x20c), 2);
        assert_eq!(rig.e.mem.u32(r.list), 0xa1);
        assert_eq!(rig.e.mem.u32(r.list + 4), 0xa3);
        assert_eq!(rig.e.mem.u8(r.entity_island + 0x25), 0xfd);
        assert_eq!(
            calls_to(&rig.e, ISLAND_REMOVE_ACTION),
            vec![vec![r.action_island, r.action]]
        );
        assert_eq!(
            calls_to(&rig.e, ISLAND_ADD_ACTION),
            vec![vec![live_island, r.action]]
        );
    }

    #[test]
    fn fn_00c976a0_writes_before_the_list_for_an_action_that_is_not_in_it() {
        let (mut rig, _) = batch_rig();
        let r = removal_fixture(&mut rig, &[Some(0xa1), Some(0xa2)], vec![], vec![]);
        rig.e.call(0x00c9_76a0, &args![r.action, r.entity]);
        assert_eq!(rig.e.mem.u16(r.entity + 0x20c), 1);
        // Index -1 receives the last entry.
        assert_eq!(rig.e.mem.u32(r.list - 4), 0xa2);
        // No other entity: the island picked is none.
        assert_eq!(calls_to(&rig.e, ISLAND_ADD_ACTION), vec![vec![0, r.action]]);
    }

    #[test]
    fn fn_00c976a0_keeps_the_action_when_the_picked_island_is_its_own() {
        let (mut rig, _) = batch_rig();
        let (other, _) = entity_on_island(&mut rig.e, 0);
        let r = removal_fixture(&mut rig, &[None], vec![], vec![other]);
        rig.e.mem.set_u32(other + 0xcc, r.action_island);
        rig.e.call(0x00c9_76a0, &args![r.action, r.entity]);
        assert_eq!(rig.e.mem.u16(r.entity + 0x20c), 0);
        assert!(calls_to(&rig.e, ISLAND_REMOVE_ACTION).is_empty());
        assert!(calls_to(&rig.e, ISLAND_ADD_ACTION).is_empty());
    }

    #[test]
    fn set_collision_filter_references_a_given_filter_and_updates_the_world() {
        let (mut rig, _) = batch_rig();
        silent(
            &mut rig.e,
            &[
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                UPDATE_COLLISION_FILTER_ON_WORLD,
            ],
        );
        let input = rig.e.mem.alloc(0x100);
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(input));
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(0x6100));
        rig.e
            .call(0x00c9_77c0, &args![rig.world, 0x6200u32, 1u32, 5u32, 6u32]);
        assert_eq!(
            order_of(
                &rig.e,
                &[
                    ADD_REFERENCE,
                    REMOVE_REFERENCE,
                    UPDATE_COLLISION_FILTER_ON_WORLD
                ]
            ),
            vec![
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                UPDATE_COLLISION_FILTER_ON_WORLD
            ]
        );
        assert_eq!(calls_to(&rig.e, ADD_REFERENCE), vec![vec![0x6200]]);
        assert_eq!(calls_to(&rig.e, REMOVE_REFERENCE), vec![vec![0x6100]]);
        assert_eq!(
            calls_to(&rig.e, UPDATE_COLLISION_FILTER_ON_WORLD),
            vec![vec![rig.world.addr(), 5, 6]]
        );
        assert_eq!(rig.e.mem.u32(rig.world.addr() + 0x7c), 0x6200);
        assert_eq!(rig.e.mem.u32(input + 0xc), 0x6200);
    }

    #[test]
    fn set_collision_filter_builds_a_default_filter_when_given_none() {
        let (mut rig, _) = batch_rig();
        silent(
            &mut rig.e,
            &[
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                COLLISION_FILTER_CONSTRUCT,
                UPDATE_COLLISION_FILTER_ON_WORLD,
            ],
        );
        let input = rig.e.mem.alloc(0x100);
        rig.e
            .set(rig.world, HkpWorld::m_collisionInput, Ptr::new(input));
        rig.e
            .set(rig.world, HkpWorld::m_collisionFilter, Ptr::new(0x6100));
        rig.e
            .call(0x00c9_77c0, &args![rig.world, 0u32, 0u32, 5u32, 6u32]);
        let filter = rig.e.mem.u32(rig.world.addr() + 0x7c);
        assert_ne!(filter, 0);
        assert_eq!(rig.e.mem.u16(filter + 4), 0x30);
        assert_eq!(
            calls_to(&rig.e, COLLISION_FILTER_CONSTRUCT),
            vec![vec![filter]]
        );
        for (offset, table) in [
            (0, NULL_FILTER_VTABLE_0),
            (8, NULL_FILTER_VTABLE_8),
            (0xc, NULL_FILTER_VTABLE_C),
            (0x10, NULL_FILTER_VTABLE_10),
            (0x14, NULL_FILTER_VTABLE_14),
        ] {
            assert_eq!(rig.e.mem.u32(filter + offset), table);
        }
        assert_eq!(rig.e.mem.u32(filter + 0x20), 1);
        assert_eq!(rig.e.mem.u32(input + 0xc), filter);
        assert!(calls_to(&rig.e, ADD_REFERENCE).is_empty());
        assert_eq!(calls_to(&rig.e, REMOVE_REFERENCE), vec![vec![0x6100]]);
        assert!(calls_to(&rig.e, UPDATE_COLLISION_FILTER_ON_WORLD).is_empty());
    }

    #[test]
    fn create_and_add_constraint_instance_builds_and_adds_the_instance() {
        let (mut rig, _) = batch_rig();
        rig.e
            .register(CONSTRAINT_INSTANCE_CONSTRUCT, |_, words| Ret {
                eax: words[0],
                ..Ret::default()
            });
        silent(&mut rig.e, &[ADD_CONSTRAINT]);
        let data = rig.e.mem.alloc(0x20);
        rig.e.mem.set_u32(data + 8, 0xabc);
        let instance = rig
            .e
            .call(0x00c9_7e30, &args![rig.world, 0x11u32, 0x22u32, data])
            .u32();
        assert_eq!(rig.e.mem.u16(instance + 4), 0x38);
        assert_eq!(rig.e.mem.u32(instance + 0x2c), 0xabc);
        assert_eq!(
            calls_to(&rig.e, CONSTRAINT_INSTANCE_CONSTRUCT),
            vec![vec![instance, 0x11, 0x22, data, 1]]
        );
        assert_eq!(
            calls_to(&rig.e, ADD_CONSTRAINT),
            vec![vec![rig.world.addr(), instance]]
        );
    }

    /// A physics system with the given entities, phantoms, actions and constraints (the
    /// byte at +0x40 is `flag`).
    fn physics_system(
        rig: &mut Rig,
        entities: &[u32],
        phantoms: &[u32],
        actions: &[u32],
        constraints: &[u32],
        flag: u8,
    ) -> u32 {
        silent(
            &mut rig.e,
            &[
                ADD_ENTITY_BATCH,
                ADD_ENTITY,
                ADD_PHANTOM_BATCH,
                ADD_PHANTOM,
                ADD_ACTION,
                ADD_CONSTRAINT,
            ],
        );
        let system = rig.e.mem.alloc(0x80);
        for (data, size, list) in [
            (8, 0xc, entities),
            (0x2c, 0x30, phantoms),
            (0x20, 0x24, actions),
            (0x14, 0x18, constraints),
        ] {
            let array = array_of(&mut rig.e, list);
            rig.e.mem.set_u32(system + data, array);
            rig.e.mem.set_u32(system + size, list.len() as u32);
        }
        rig.e.mem.set_u8(system + 0x40, flag);
        system
    }

    /// A constraint whose words at +0x14 and +0x18 are `first` and `second`.
    fn constraint_with_words(e: &mut Engine, first: u32, second: u32) -> u32 {
        let constraint = e.mem.alloc(0x40);
        e.mem.set_u32(constraint + 0x14, first);
        e.mem.set_u32(constraint + 0x18, second);
        constraint
    }

    #[test]
    fn add_physics_system_adds_whole_arrays_in_batches() {
        let (mut rig, _) = batch_rig();
        let first = constraint_with_words(&mut rig.e, 1, 0);
        let skipped = constraint_with_words(&mut rig.e, 0, 0);
        let last = constraint_with_words(&mut rig.e, 0, 1);
        let system = physics_system(
            &mut rig,
            &[0xe1, 0xe2],
            &[0xf1],
            &[0xa1, 0, 0xa2],
            &[first, skipped, 0, last],
            1,
        );
        let world = rig.world.addr();
        rig.e.call(0x00c9_7f80, &args![rig.world, system]);
        let entities = rig.e.mem.u32(system + 8);
        let phantoms = rig.e.mem.u32(system + 0x2c);
        assert_eq!(
            calls_to(&rig.e, ADD_ENTITY_BATCH),
            vec![vec![world, entities, 2, 1]]
        );
        assert!(calls_to(&rig.e, ADD_ENTITY).is_empty());
        assert_eq!(
            calls_to(&rig.e, ADD_PHANTOM_BATCH),
            vec![vec![world, phantoms, 1]]
        );
        assert!(calls_to(&rig.e, ADD_PHANTOM).is_empty());
        assert_eq!(
            calls_to(&rig.e, ADD_ACTION),
            vec![vec![world, 0xa1], vec![world, 0xa2]]
        );
        assert_eq!(
            calls_to(&rig.e, ADD_CONSTRAINT),
            vec![vec![world, first], vec![world, last]]
        );
    }

    #[test]
    fn add_physics_system_adds_one_by_one_around_null_entries() {
        let (mut rig, _) = batch_rig();
        let system = physics_system(&mut rig, &[0xe1, 0, 0xe2], &[0, 0xf2], &[], &[], 0);
        let world = rig.world.addr();
        rig.e.call(0x00c9_7f80, &args![rig.world, system]);
        assert!(calls_to(&rig.e, ADD_ENTITY_BATCH).is_empty());
        assert_eq!(
            calls_to(&rig.e, ADD_ENTITY),
            vec![vec![world, 0xe1, 0], vec![world, 0xe2, 0]]
        );
        assert!(calls_to(&rig.e, ADD_PHANTOM_BATCH).is_empty());
        assert_eq!(calls_to(&rig.e, ADD_PHANTOM), vec![vec![world, 0xf2]]);
        rig.e.mem.set_u8(system + 0x40, 1);
        rig.e.call(0x00c9_7f80, &args![rig.world, system]);
        assert_eq!(calls_to(&rig.e, ADD_ENTITY)[2], vec![world, 0xe1, 1]);
    }

    #[test]
    fn add_physics_system_with_nothing_in_it_calls_nothing() {
        let (mut rig, _) = batch_rig();
        let system = physics_system(&mut rig, &[], &[], &[], &[], 1);
        rig.e.call(0x00c9_7f80, &args![rig.world, system]);
        let log = rig.e.call_log.clone().unwrap();
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn null_contact_mgr_factory_deleting_destructor_resets_the_tables_and_frees() {
        let (mut rig, freed) = batch_rig();
        let object = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u16(object + 4, 0x40);
        let returned = rig.e.call(0x00c9_8260, &args![object, 0u32]).u32();
        assert_eq!(returned, object);
        assert_eq!(rig.e.mem.u32(object), REFERENCED_OBJECT_VTABLE);
        assert_eq!(rig.e.mem.u32(object + 8), REFERENCED_OBJECT_VTABLE);
        assert!(freed.borrow().is_empty());
        rig.e.call(0x00c9_8260, &args![object, 3u32]);
        let allocator = rig.e.mem.u32(rig.router + 0x10);
        assert_eq!(freed.borrow().clone(), vec![vec![allocator, object, 0x40]]);
    }

    /// Calls the filter function at `address` with `words` stack words after `this` and `out`
    /// and checks that it stores the byte 1 at `out`.
    fn check_out_byte(address: u32, words: usize) {
        let mut e = Engine::new();
        let out = e.mem.alloc(4);
        let mut arguments = vec![0u32, out];
        arguments.extend(vec![0u32; words]);
        e.call(address, &arguments);
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn fn_00c982a0_accepts_every_pair() {
        check_out_byte(0x00c9_82a0, 5);
    }

    #[test]
    fn fn_00c982b0_accepts_every_pair() {
        check_out_byte(0x00c9_82b0, 7);
    }

    #[test]
    fn fn_00c982c0_accepts_every_pair() {
        check_out_byte(0x00c9_82c0, 4);
    }

    #[test]
    fn fn_00c982d0_accepts_every_pair() {
        check_out_byte(0x00c9_82d0, 2);
    }

    #[test]
    fn remove_constraint_answers_its_result_pointer() {
        let mut rig = rig();
        silent(
            &mut rig.e,
            &[
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                REMOVE_CONSTRAINT_SI,
                CONSTRAINT_DETACH,
                QUEUE_FLUSH_PENDING,
            ],
        );
        let log = Rc::new(RefCell::new(vec![]));
        // Kind 1, but its action belongs to another world: it stays.
        let constraint = constraint_object(&mut rig.e, &log, 1, 0, 0x1000);
        let result = rig.e.mem.alloc(4);
        rig.e.mem.set_u8(result, 0x55);
        rig.e.mem.set_u32(rig.world.addr() + 0x8c, 1);
        rig.e
            .set(rig.world, HkpWorld::m_pendingOperations, Ptr::new(0x5000));
        // EAX is `result` at both RETs (the hkBool result pointer the callers read through).
        let answered = rig
            .e
            .call(0x00c9_7e80, &args![rig.world, result, constraint])
            .u32();
        assert_eq!(answered, result);
    }

    #[test]
    fn the_accepting_filters_answer_their_out_pointer() {
        for (address, words) in [
            (0x00c9_82a0, 5usize),
            (0x00c9_82b0, 7),
            (0x00c9_82c0, 4),
            (0x00c9_82d0, 2),
        ] {
            let mut e = Engine::new();
            let out = e.mem.alloc(4);
            let mut arguments = vec![0u32, out];
            arguments.extend(vec![0u32; words]);
            // EAX is `out` at the RET; virtual call sites test the flag through it (00c90f6a).
            assert_eq!(e.call(address, &arguments).u32(), out);
        }
    }

    #[test]
    fn fn_00c982e0_runs_the_base_destructor_and_frees_on_request() {
        let (mut rig, freed) = batch_rig();
        silent(&mut rig.e, &[RAY_CASTER_BASE_DESTRUCT]);
        let object = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u16(object + 4, 0x20);
        assert_eq!(rig.e.call(0x00c9_82e0, &args![object, 0u32]).u32(), object);
        assert!(freed.borrow().is_empty());
        assert_eq!(rig.e.call(0x00c9_82e0, &args![object, 1u32]).u32(), object);
        assert_eq!(calls_to(&rig.e, RAY_CASTER_BASE_DESTRUCT).len(), 2);
        let allocator = rig.e.mem.u32(rig.router + 0x10);
        assert_eq!(freed.borrow().clone(), vec![vec![allocator, object, 0x20]]);
    }

    #[test]
    fn simple_world_ray_caster_deleting_destructor_stores_the_base_table_and_frees() {
        let (mut rig, _) = batch_rig();
        silent(&mut rig.e, &[FREE_OBJECT]);
        let object = rig.e.mem.alloc(0x40);
        let returned = rig.e.call(0x00c9_8350, &args![object, 0u32]).u32();
        assert_eq!(returned, object);
        assert_eq!(rig.e.mem.u32(object), RAY_CASTER_BASE_VTABLE);
        assert!(calls_to(&rig.e, FREE_OBJECT).is_empty());
        rig.e.call(0x00c9_8350, &args![object, 1u32]);
        assert_eq!(calls_to(&rig.e, FREE_OBJECT), vec![vec![object]]);
    }

    #[test]
    fn fn_00c98370_resets_the_tables_and_frees_on_request() {
        let (mut rig, freed) = batch_rig();
        let object = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u16(object + 4, 0x30);
        assert_eq!(rig.e.call(0x00c9_8370, &args![object, 0u32]).u32(), object);
        assert_eq!(rig.e.mem.u32(object), REFERENCED_OBJECT_VTABLE);
        assert_eq!(rig.e.mem.u32(object + 8), FILTER_BASE_VTABLE);
        assert!(freed.borrow().is_empty());
        rig.e.call(0x00c9_8370, &args![object, 1u32]);
        let allocator = rig.e.mem.u32(rig.router + 0x10);
        assert_eq!(freed.borrow().clone(), vec![vec![allocator, object, 0x30]]);
    }

    #[test]
    fn world_deleting_destructor_runs_the_destructor_and_frees_on_request() {
        let (mut rig, freed) = batch_rig();
        silent(&mut rig.e, &[WORLD_DESTRUCTOR]);
        let object = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u16(object + 4, 0x350);
        assert_eq!(rig.e.call(0x00c9_83b0, &args![object, 0u32]).u32(), object);
        assert!(freed.borrow().is_empty());
        assert_eq!(rig.e.call(0x00c9_83b0, &args![object, 1u32]).u32(), object);
        assert_eq!(calls_to(&rig.e, WORLD_DESTRUCTOR), vec![vec![object]; 2]);
        let allocator = rig.e.mem.u32(rig.router + 0x10);
        assert_eq!(freed.borrow().clone(), vec![vec![allocator, object, 0x350]]);
    }
}

/// Test fixtures shared with the other `Havok SDK` units that write profiling records.
#[cfg(test)]
pub(crate) mod profiling_support {
    use super::*;

    /// An engine with a profiling stream that holds `records` records; returns it and the
    /// stream block and the first record address.
    pub(crate) fn profiling_engine(records: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        e.set_global(PROFILE_SLOT, 3u32);
        let stream = e.mem.alloc(0x20);
        let area = e.mem.alloc(12 * records.max(1));
        e.mem.set_u32(stream + 4, area);
        e.mem.set_u32(stream + 0xc, area + 12 * records);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (e, stream, area)
    }

    /// The labels of the profiling records written so far.
    pub(crate) fn labels(e: &Engine, stream: u32, start: u32) -> Vec<u32> {
        let cursor = e.mem.u32(stream + 4);
        (start..cursor)
            .step_by(12)
            .map(|record| e.mem.u32(record))
            .collect()
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_00c98310() {
        assert_eq!(
            check(0x00c9_8310, 0x00c9_82e0, 0x14, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00c98320() {
        assert_eq!(
            check(0x00c9_8320, 0x00c9_82e0, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00c98330() {
        assert_eq!(
            check(0x00c9_8330, 0x00c9_82e0, 0xc, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_00c98340() {
        assert_eq!(
            check(0x00c9_8340, 0x00c9_82e0, 0x10, &[0x1]).u32(),
            0xdead_beef
        );
    }
}

/// Tests of the constructor `00c95a80`.
#[cfg(test)]
mod tests_world_ctor {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Addresses for the doubles of virtual functions and of the broad-phase creator.
    const ALLOCATE: u32 = 0x0900_0201;
    const CREATE_BROAD_PHASE_DOUBLE: u32 = 0x0900_0202;
    const BROAD_PHASE_SET_EXTENTS: u32 = 0x0900_0203;
    const MANAGER_INIT: u32 = 0x0900_0204;
    const REPORT: u32 = 0x0900_0205;

    type Words = Rc<RefCell<Vec<u32>>>;

    struct Rig {
        e: Engine,
        world: u32,
        cinfo: u32,
        sizes: Words,
        failing: Words,
        answer: Words,
    }

    /// The constants the constructor reads, with the exe's values (read from FalloutNV.exe
    /// 1.4.0.525 through the read-only Ghidra server).
    const FLOATS: [(u32, u32); 15] = [
        (CTOR_LEVEL_LOW_FACTOR, 0x3400_0000),
        (0x0101_3ea4, 0x3c23_d70a),
        (0x010c_6850, 0x3da3_d70a),
        (0x010c_684c, 0x3c8b_4396),
        (0x0101_df44, 0x3e4c_cccd),
        (0x0102_d934, 0x3ca3_d70a),
        (0x0101_7870, 0x3e99_999a),
        (0x0103_13d4, 0x3ccc_cccd),
        (0x0102_31a8, 0x3ecc_cccd),
        (CTOR_NO_THRESHOLD, 0x7d7f_ffee),
        (CTOR_LANE_FILLER, 0),
        (CTOR_EXTENT_SCALE, 0x4eff_fc00),
        (CTOR_EXTENT_INVERSE_SCALE, 0x3000_0200),
        (CTOR_STEP_TIME, 0x3c88_8889),
        (CTOR_DEFAULT_GRAVITY, 0x411c_f5c3),
    ];
    const DOUBLES: [(u32, u64); 6] = [
        (CTOR_TENTH, 0x3fb9_9999_a000_0000),
        (CTOR_FRAME_FACTOR, 0x3f90_624d_e000_0000),
        (CTOR_FOUR, 0x4010_0000_0000_0000),
        (CTOR_EIGHT, 0x4020_0000_0000_0000),
        (CTOR_STEP_DOUBLE, 0x3f91_1111_2000_0000),
        (HALF, 0x3fe0_0000_0000_0000),
    ];

    /// The callees that only return their `this` (constructors).
    const CONSTRUCTORS: [u32; 14] = [
        CTOR_OPERATION_QUEUE,
        CTOR_BROAD_PHASE_DISPATCHER,
        CTOR_ENTITY_ENTITY_LISTENER,
        CTOR_CONTACT_MGR_FACTORY,
        CTOR_COLLISION_DISPATCHER,
        CTOR_DEFAULT_COLLISION_FILTER,
        CTOR_SIMULATION,
        CTOR_CONTINUOUS_SIMULATION,
        CTOR_MULTI_THREADED_SIMULATION,
        CTOR_CRITICAL_SECTION,
        CTOR_RIGID_BODY_CINFO,
        CTOR_BROAD_PHASE_BORDER,
        CTOR_KD_TREE_MANAGER,
        CTOR_AABB_TREE_MANAGER,
    ];
    /// The callees that do nothing here.
    const SILENT: [u32; 21] = [
        CTOR_SOLVER_TABLES,
        ADD_REFERENCE,
        REMOVE_REFERENCE,
        CTOR_COLLISION_QUALITY,
        CTOR_COLLISION_CALLBACKS,
        CTOR_MULTI_THREAD_CHECK_DISABLE,
        MAKE_HELPER_SECOND,
        REGISTER_HELPER,
        SET_CACHED_SHAPE_DATA,
        SET_TIME_INFORMATION,
        ADD_ENTITY_SI,
        ADD_ENTITY_BP,
        FIRE_WORLD_ENTITY_ADDED,
        FIRE_ENTITY_ADDED,
        BROAD_PHASE_SET_EXTENTS,
        MANAGER_INIT,
        CTOR_ERROR_STREAM_CONSTRUCT,
        CTOR_STREAM_APPEND_TEXT,
        CTOR_STREAM_APPEND_FIRST_INT,
        CTOR_STREAM_APPEND_SECOND_INT,
        CTOR_ERROR_STREAM_DESTRUCT,
    ];

    fn words() -> Words {
        Rc::new(RefCell::new(vec![]))
    }

    fn rig() -> Rig {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0101_3000,
            0x0101_7000,
            0x0101_d000,
            0x0101_f000,
            0x0102_3000,
            0x0102_4000,
            0x0102_d000,
            0x0103_1000,
            0x0104_d000,
            0x010c_4000,
            0x010c_6000,
            0x011b_3000,
            0x0126_8000,
        ] {
            e.map(page, 0x1000);
        }
        for (address, bits) in FLOATS {
            e.set_global(address, bits);
        }
        for (address, bits) in DOUBLES {
            e.set_global(address, bits);
        }
        for lane in 0..4 {
            e.set_global(CTOR_SIGN_MASK + 4 * lane, 0x8000_0000u32);
        }
        // The memory router and its allocator (slot 4 allocates; sizes in `failing` fail).
        let sizes = words();
        let failing = words();
        let (log, refuse) = (sizes.clone(), failing.clone());
        e.register_double(ALLOCATE, move |e, a| {
            log.borrow_mut().push(a[1]);
            let block = if refuse.borrow().contains(&a[1]) {
                0
            } else {
                e.mem.alloc(a[1])
            };
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table + 4, ALLOCATE);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, table);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in CONSTRUCTORS {
            e.register(address, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
        for address in SILENT {
            e.register(address, |_, a| Ret {
                eax: a.first().copied().unwrap_or(0),
                ..Ret::default()
            });
        }
        // The fixed islands come with all flags of byte +0x26 set.
        e.register(CTOR_SIMULATION_ISLAND, |e, a| {
            e.mem.set_u8(a[0] + 0x26, 0xff);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        // The rigid body keeps the mass and motion type it was given and has a motion.
        e.register(CTOR_RIGID_BODY, |e, a| {
            let mass = e.mem.u32(a[1] + 0x90);
            let motion_type = e.mem.u8(a[1] + 0xb4);
            e.mem.set_u32(a[0] + 0x200, mass);
            e.mem.set_u8(a[0] + 0x204, motion_type);
            e.mem.set_u32(a[0] + 0x18, 1);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        // The maintenance manager gets a vtable whose slot +0xc is `MANAGER_INIT`.
        let manager_table = e.mem.alloc(0x10);
        e.mem.set_u32(manager_table + 0xc, MANAGER_INIT);
        e.register_double(CTOR_MAINTENANCE_MGR, move |e, a| {
            e.mem.set_u32(a[0], manager_table);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        // The array growth gives the island array room for four entries.
        e.register(CTOR_ARRAY_GROW, |e, a| {
            let data = e.mem.alloc(0x10);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
        // The broad phase made by the creator: slot +0x70 is `BROAD_PHASE_SET_EXTENTS`.
        let broad_phase_table = e.mem.alloc(0x80);
        e.mem
            .set_u32(broad_phase_table + 0x70, BROAD_PHASE_SET_EXTENTS);
        let broad_phase = e.mem.alloc(0x10);
        e.mem.set_u32(broad_phase, broad_phase_table);
        e.register_double(CREATE_BROAD_PHASE_DOUBLE, move |_, _| Ret {
            eax: broad_phase,
            ..Ret::default()
        });
        e.set_global(CTOR_CREATE_BROAD_PHASE, CREATE_BROAD_PHASE_DOUBLE);
        // The error reporter answers the word in `answer`.
        let answer = words();
        answer.borrow_mut().push(0);
        let reply = answer.clone();
        e.register_double(REPORT, move |_, _| Ret {
            eax: reply.borrow()[0],
            ..Ret::default()
        });
        let reporter_table = e.mem.alloc(0x10);
        e.mem.set_u32(reporter_table + 0xc, REPORT);
        let reporter = e.mem.alloc(8);
        e.mem.set_u32(reporter, reporter_table);
        e.set_global(CTOR_ERROR_REPORTER, reporter);
        let world = e.new_object::<HkpWorld>().addr();
        // A cinfo: gravity (0, -10, 0), world AABB (-100, -50, -200) to (100, 50, 200),
        // 4 solver iterations, 1 micro step, single-threaded with simulation islands.
        let cinfo = e.mem.alloc(0xf0);
        e.mem.set_f32(cinfo + 0x14, -10.0);
        for (lane, (low, high)) in [(-100.0f32, 100.0f32), (-50.0, 50.0), (-200.0, 200.0)]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(cinfo + 0x30 + 4 * lane as u32, low);
            e.mem.set_f32(cinfo + 0x40 + 4 * lane as u32, high);
        }
        e.mem.set_u32(cinfo + 0x20, (-7i32) as u32);
        e.mem.set_f32(cinfo + 0x24, 1.5);
        e.mem.set_f32(cinfo + 0x54, 0.25);
        e.mem.set_f32(cinfo + 0x60, 200.0);
        e.mem.set_u32(cinfo + 0x64, 0x30);
        e.mem.set_f32(cinfo + 0x68, 0.125);
        e.mem.set_u32(cinfo + 0x70, 0x40);
        e.mem.set_u8(cinfo + 0x74, 1);
        e.mem.set_f32(cinfo + 0x78, 0.5);
        e.mem.set_f32(cinfo + 0x7c, 1.0);
        e.mem.set_u32(cinfo + 0x80, 4);
        e.mem.set_u32(cinfo + 0x84, 1);
        e.mem.set_f32(cinfo + 0x88, 3.0);
        e.mem.set_f32(cinfo + 0x90, 0.75);
        e.mem.set_f32(cinfo + 0x94, 0.625);
        e.mem.set_f32(cinfo + 0x9c, 0.0625);
        e.mem.set_u32(cinfo + 0xa0, 9);
        e.mem.set_f32(cinfo + 0xa8, 0.5);
        e.mem.set_u8(cinfo + 0xd0, 1);
        e.mem.set_u8(cinfo + 0xd1, 1);
        e.mem.set_u8(cinfo + 0xd2, 1);
        e.mem.set_u32(cinfo + 0xd4, 77);
        e.call_log = Some(vec![]);
        Rig {
            e,
            world,
            cinfo,
            sizes,
            failing,
            answer,
        }
    }

    fn construct(rig: &mut Rig, version: u32) -> u32 {
        rig.e
            .call(0x00c9_5a80, &args![rig.world, rig.cinfo, version])
            .u32()
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .map(|log| {
                log.iter()
                    .filter(|(called, _)| *called == address)
                    .map(|(_, words)| words.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn f32_at(e: &Engine, at: u32) -> f32 {
        e.mem.f32(at)
    }

    /// One deactivation level as plain `f64` arithmetic rounded to `float` at each store.
    fn expected_level(linear: f32, angular: f32, gravity: f64) -> [u32; 4] {
        let inverse_steps = 0.25f64;
        let frame = f64::from_bits(0x3f90_624d_e000_0000);
        let tenth = f64::from_bits(0x3fb9_9999_a000_0000);
        let linear = f64::from((f64::from(linear) * gravity) as f32);
        let angular = f64::from(angular);
        let ratio = f64::from((angular * gravity / linear) as f32);
        let angular_scale = f64::from((tenth * gravity) as f32);
        let threshold = if angular > 0.0 {
            ((inverse_steps * frame) / angular) as f32
        } else {
            f32::from_bits(0x7d7f_ffee)
        };
        [
            ((1.0 / linear) as f32).to_bits(),
            ((1.0 / (linear * angular_scale)) as f32).to_bits(),
            ((1.0 - ratio * (inverse_steps * frame)) as f32).to_bits(),
            threshold.to_bits(),
        ]
    }

    #[test]
    fn world_constructor_builds_the_single_threaded_world() {
        let mut rig = rig();
        let returned = construct(&mut rig, 0x111d4);
        let (e, w) = (&rig.e, rig.world);
        assert_eq!(returned, w);
        assert_eq!(e.mem.u32(w), CTOR_WORLD_VTABLE);
        assert_eq!(e.mem.u16(w + 6), 1);
        for array in [0x28, 0x34, 0x40, 0x110, 0x14c, 0x1ac] {
            assert_eq!(e.mem.u32(w + array + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(w + 0xa8), 0xffff_fff1);
        assert_eq!(e.mem.u16(w + 0xac), 0x8000);
        assert_eq!(e.mem.u8(w + 0x54), 1);
        assert_eq!(e.mem.u8(w + 0x9d), 1);
        assert_eq!(e.mem.u32(w + 0xa4), 1);
        // No error report.
        assert!(calls_to(e, CTOR_ERROR_STREAM_CONSTRUCT).is_empty());
        // The violated-constraint array and the operation queue.
        let violated = e.mem.u32(w + 0x1b8);
        assert_ne!(violated, 0);
        let queue = e.mem.u32(w + 0x88);
        assert_eq!(calls_to(e, CTOR_OPERATION_QUEUE), vec![vec![queue, w]]);
        // The broad phase from the creator, with the extents and the marker count.
        assert_eq!(
            calls_to(e, CREATE_BROAD_PHASE_DOUBLE),
            vec![vec![w + 0x310, w + 0x320, 0x40]]
        );
        assert_eq!(f32_at(e, w + 0x310), -100.0);
        assert_eq!(f32_at(e, w + 0x328), 200.0);
        let broad_phase = e.mem.u32(w + 0x58);
        assert_ne!(broad_phase, 0);
        assert_eq!(e.mem.u32(w + 0x338) as i32, -7);
        assert_eq!(e.mem.u32(w + 0x33c) as i32, -3);
        assert_eq!(e.mem.u32(w + 0x334), 0x30);
        // Gravity and the solver settings.
        assert_eq!(f32_at(e, w + 0x14), -10.0);
        assert_eq!(
            calls_to(e, CTOR_SOLVER_TABLES),
            vec![vec![0.75f32.to_bits(), 0.625f32.to_bits()]]
        );
        assert_eq!(f32_at(e, w + 0xcc), 0.75);
        assert_eq!(f32_at(e, w + 0xd0), 0.75);
        assert_eq!(f32_at(e, w + 0x1e0), 1.0);
        assert_eq!(f32_at(e, w + 0x1e4), 0.5);
        assert_eq!(f32_at(e, w + 0x1e8), 1.0);
        assert_eq!(f32_at(e, w + 0x240), 1.5);
        assert_eq!(e.mem.u32(w + 0x2f4), 4);
        assert_eq!(f32_at(e, w + 0x300), 0.25);
        assert_eq!(e.mem.u32(w + 0x2f8), 1);
        assert_eq!(f32_at(e, w + 0x2fc), 1.0);
        assert_eq!(f32_at(e, w + 0x308), 9.0);
        // The deactivation levels for a gravity length of 10.
        let levels = [
            (f32::from_bits(0x3400_0000), 0.0f32),
            (f32::from_bits(0x3400_0000), 0.0),
            (f32::from_bits(0x3c23_d70a), f32::from_bits(0x3da3_d70a)),
            (f32::from_bits(0x3c8b_4396), f32::from_bits(0x3e4c_cccd)),
            (f32::from_bits(0x3ca3_d70a), f32::from_bits(0x3e99_999a)),
            (f32::from_bits(0x3ccc_cccd), f32::from_bits(0x3ecc_cccd)),
        ];
        for (level, (linear, angular)) in levels.into_iter().enumerate() {
            let row = w + 0x244 + 0x1c * level as u32;
            let expected = expected_level(linear, angular, 10.0);
            for (index, bits) in expected.into_iter().enumerate() {
                assert_eq!(e.mem.u32(row + 4 * index as u32), bits, "level {level}");
            }
            assert_eq!(f32_at(e, row + 0x10), 0.25);
            assert_eq!(f32_at(e, row + 0x14), 4.0);
            assert_eq!(e.mem.u16(row + 0x18), 0x3f80);
            assert_eq!(e.mem.u16(row + 0x1a), 0x4180);
        }
        // The broad-phase dispatcher and its listener slots.
        let dispatcher = e.mem.u32(w + 0x64);
        let phantom_listener = e.mem.u32(w + 0x68);
        let border_listener = e.mem.u32(w + 0x70);
        assert_eq!(e.mem.u32(phantom_listener), CTOR_PHANTOM_LISTENER_VTABLE);
        assert_eq!(e.mem.u32(border_listener + 8), CTOR_BORDER_LISTENER_TABLE);
        assert_eq!(e.mem.u32(dispatcher + 0x28), phantom_listener + 8);
        assert_eq!(e.mem.u32(dispatcher + 0x24), e.mem.u32(w + 0x6c) + 8);
        assert_eq!(e.mem.u32(dispatcher + 0x6c), border_listener + 8);
        // The collision dispatcher, with the factory whose reference is dropped.
        let factory_call = calls_to(e, CTOR_CONTACT_MGR_FACTORY);
        assert_eq!(factory_call.len(), 2, "the dispatcher's and 00c912e0's");
        let factory = factory_call[0][0];
        let collision_dispatcher = e.mem.u32(w + 0x80);
        assert_eq!(
            calls_to(e, CTOR_COLLISION_DISPATCHER),
            vec![vec![
                collision_dispatcher,
                CTOR_DEFAULT_AGENT_FUNCTION,
                factory
            ]]
        );
        assert!(calls_to(e, REMOVE_REFERENCE).contains(&vec![factory]));
        // The default filters.
        let filter = e.mem.u32(w + 0x7c);
        assert_eq!(e.mem.u32(filter), CTOR_FILTER_TABLES[0]);
        assert_eq!(e.mem.u32(filter + 0x20), 1);
        assert_eq!(
            e.mem.u32(e.mem.u32(w + 0x84)),
            CTOR_CONVEX_LIST_FILTER_VTABLE
        );
        // The collision input.
        let input = e.mem.u32(w + 0x78);
        assert_eq!(e.mem.u32(input), collision_dispatcher);
        assert_eq!(f32_at(e, input + 8), 0.25);
        assert_eq!(e.mem.u32(input + 0xc), filter);
        assert_eq!(e.mem.u8(w + 0x340), 1);
        let config = e.mem.u32(input + 0x70);
        assert_eq!(f32_at(e, config), 0.0625);
        assert_eq!(e.mem.u32(config + 4), 9);
        assert_eq!(e.mem.u32(input + 0x58), 0);
        assert_eq!(e.mem.u32(input + 0x5c), 0);
        let scale = f32::from_bits(0x4eff_fc00);
        let inverse_scale = f32::from_bits(0x3000_0200);
        let spans = [200.0f32, 100.0, 400.0];
        let minimum = [-100.0f32, -50.0, -200.0];
        for (lane, (span, low)) in spans.into_iter().zip(minimum).enumerate() {
            let at = 4 * lane as u32;
            let inverse = (1.0f64 / f64::from(span)) as f32;
            assert_eq!(f32_at(e, input + 0x40 + at), inverse * scale);
            assert_eq!(f32_at(e, input + 0x20 + at), -low);
            assert_eq!(f32_at(e, input + 0x30 + at), inverse_scale * span + -low);
        }
        for vector in [0x20, 0x30, 0x40] {
            assert_eq!(e.mem.u32(input + vector + 0xc), 0);
        }
        assert_eq!(
            calls_to(e, BROAD_PHASE_SET_EXTENTS),
            vec![vec![broad_phase, input + 0x20, input + 0x30, input + 0x40]]
        );
        assert_eq!(e.mem.u32(input + 0x60), collision_dispatcher + 0x1c60);
        assert_eq!(e.mem.u32(input + 0x68), w + 0x1d0);
        // 00c912e0 registered its three helpers with the collision dispatcher.
        let registered = calls_to(e, REGISTER_HELPER);
        assert_eq!(registered.len(), 3);
        assert!(registered
            .iter()
            .all(|call| call[0] == collision_dispatcher));
        // The single-threaded simulation and the island-dirty critical section.
        let simulation = e.mem.u32(w + 8);
        assert_eq!(calls_to(e, CTOR_SIMULATION), vec![vec![simulation, w]]);
        assert_eq!(e.mem.u32(w + 0xf0), 1);
        assert_eq!(e.mem.u32(w + 0xb4), 0);
        let section = e.mem.u32(w + 0xc0);
        assert_eq!(
            calls_to(e, CTOR_CRITICAL_SECTION),
            vec![vec![section, 0xfa0]]
        );
        assert_eq!(e.mem.u32(w + 0xb8), 0);
        // The collision-quality call.
        assert_eq!(
            calls_to(e, CTOR_COLLISION_QUALITY)[0][0],
            collision_dispatcher
        );
        // The fixed island; with simulation islands no active island is made.
        let island = e.mem.u32(w + 0x20);
        assert_eq!(e.mem.u16(island + 0x20), 0xffff);
        assert_eq!(e.mem.u8(island + 0x26), 0xf0);
        assert_eq!(calls_to(e, CTOR_SIMULATION_ISLAND).len(), 1);
        assert_eq!(e.mem.u32(w + 0x2c), 0);
        // The fixed rigid body: mass 0, motion type 5, added and released.
        let body = e.mem.u32(w + 0x24);
        assert_eq!(e.mem.u32(body + 0x200), 0);
        assert_eq!(e.mem.u8(body + 0x204), 5);
        assert_eq!(calls_to(e, ADD_ENTITY_SI), vec![vec![w, body, 1]]);
        assert!(calls_to(e, REMOVE_REFERENCE).contains(&vec![body]));
        // The step information.
        assert_eq!(e.mem.u32(w + 0x1d0), 0);
        assert_eq!(e.mem.u32(w + 0x1d4), 0x3c88_8889);
        let delta = f64::from_bits(0x3f91_1111_2000_0000) as f32;
        assert_eq!(f32_at(e, w + 0x1d8), delta);
        assert_eq!(f32_at(e, w + 0x1dc), (1.0 / f64::from(delta)) as f32);
        // Border, maintenance manager, no tree, the multi-thread check, the job data.
        let border = e.mem.u32(w + 0x1bc);
        assert_eq!(
            calls_to(e, CTOR_BROAD_PHASE_BORDER),
            vec![vec![border, w, 0, 0]]
        );
        let manager = e.mem.u32(w + 0x4c);
        assert_eq!(calls_to(e, MANAGER_INIT), vec![vec![manager, w]]);
        assert_eq!(e.mem.u32(w + 0x5c), 0);
        assert!(calls_to(e, CTOR_COLLISION_CALLBACKS).is_empty());
        assert_eq!(
            calls_to(e, CTOR_MULTI_THREAD_CHECK_DISABLE),
            vec![vec![w + 0xa8]]
        );
        assert_ne!(e.mem.u32(w + 0x74), 0);
        assert!(rig.sizes.borrow().contains(&0x120));
    }

    #[test]
    fn world_constructor_fills_the_collision_quality_input() {
        let mut rig = rig();
        let seen = words();
        let sink = seen.clone();
        rig.e.register_double(CTOR_COLLISION_QUALITY, move |e, a| {
            for offset in (0..0x2c).step_by(4) {
                sink.borrow_mut().push(e.mem.u32(a[1] + offset));
            }
            Ret::default()
        });
        for (offset, value) in [(0xc0, 1.0f32), (0xc4, 2.0), (0xc8, 3.0), (0xcc, 4.0)] {
            rig.e.mem.set_f32(rig.cinfo + offset, value);
        }
        rig.e.mem.set_u8(rig.cinfo + 0x98, 1);
        construct(&mut rig, 0x111d4);
        let seen = seen.borrow().clone();
        assert_eq!(
            seen,
            vec![
                10.0f32.to_bits(),
                0.25f32.to_bits(),
                0.125f32.to_bits(),
                200.0f32.to_bits(),
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                4.0f32.to_bits(),
                0x0003_0001,
                0x0005_0004,
                // Continuous wanted (type 1: no), generation == 2 (no), >= 1 (yes), welding.
                0x0101_0000,
            ]
        );
        assert_eq!(f32_at(&rig.e, rig.world + 0xf4), 1.0);
        assert_eq!(f32_at(&rig.e, rig.world + 0x100), 4.0);
    }

    #[test]
    fn world_constructor_reports_a_version_mismatch() {
        let mut rig = rig();
        construct(&mut rig, 0x1234);
        let e = &rig.e;
        let streams = calls_to(e, CTOR_ERROR_STREAM_CONSTRUCT);
        assert_eq!(streams.len(), 1);
        let stream = streams[0][0];
        assert_eq!(streams[0][1..], [stream + 0x100, 0x200]);
        assert_eq!(
            calls_to(e, CTOR_STREAM_APPEND_TEXT),
            vec![
                vec![stream, CTOR_VERSION_TEXT_FIRST],
                vec![stream, CTOR_VERSION_TEXT_SECOND],
                vec![stream, CTOR_VERSION_TEXT_LAST],
            ]
        );
        assert_eq!(
            calls_to(e, CTOR_STREAM_APPEND_FIRST_INT),
            vec![vec![stream, 0x111d4]]
        );
        assert_eq!(
            calls_to(e, CTOR_STREAM_APPEND_SECOND_INT),
            vec![vec![stream, 0x1234]]
        );
        let reporter: u32 = e.global(CTOR_ERROR_REPORTER);
        assert_eq!(
            calls_to(e, REPORT),
            vec![vec![
                reporter,
                3,
                0x53c9_4b42,
                stream + 0x100,
                0x010c_6864,
                0xbb5
            ]]
        );
        assert_eq!(calls_to(e, CTOR_ERROR_STREAM_DESTRUCT), vec![vec![stream]]);
        // The construction goes on.
        assert_eq!(f32_at(e, rig.world + 0x14), -10.0);
        assert_ne!(e.mem.u32(rig.world + 0x74), 0);
    }

    #[test]
    #[should_panic(expected = "INT3 at 00c95d68")]
    fn world_constructor_breaks_when_the_reporter_asks() {
        let mut rig = rig();
        rig.answer.borrow_mut()[0] = 1;
        construct(&mut rig, 0x1234);
    }

    #[test]
    fn world_constructor_takes_the_cinfo_objects_and_the_continuous_simulation() {
        let mut rig = rig();
        let c = rig.cinfo;
        let (filter, convex, watchdog) = (rig.e.mem.alloc(0x30), rig.e.mem.alloc(8), 0x5555);
        rig.e.mem.set_u32(c + 0x58, filter);
        rig.e.mem.set_u32(c + 0x5c, convex);
        rig.e.mem.set_u32(c + 0x6c, watchdog);
        // No gravity: the length is the default 9.81.
        rig.e.mem.set_f32(c + 0x14, 0.0);
        rig.e.mem.set_u8(c + 0xd1, 2);
        rig.e.mem.set_u8(c + 0xd2, 0);
        rig.e.mem.set_u8(c + 0x28, 3);
        rig.e.mem.set_u8(c + 0x50, 1);
        rig.e.mem.set_u8(c + 0x53, 1);
        rig.e.mem.set_u8(c + 0xe0, 1);
        rig.e.mem.set_u8(c + 0x74, 2);
        let gravity_seen = words();
        let sink = gravity_seen.clone();
        rig.e.register_double(CTOR_COLLISION_QUALITY, move |e, a| {
            sink.borrow_mut().push(e.mem.u32(a[1]));
            sink.borrow_mut().push(e.mem.u32(a[1] + 0x28));
            Ret::default()
        });
        construct(&mut rig, 0x111d4);
        let (e, w) = (&rig.e, rig.world);
        assert_eq!(e.mem.u32(w + 0x7c), filter);
        assert_eq!(e.mem.u32(w + 0x84), convex);
        assert_eq!(e.mem.u32(w + 0x50), watchdog);
        let referenced = calls_to(e, ADD_REFERENCE);
        for object in [watchdog, filter, convex] {
            assert!(referenced.contains(&vec![object]));
        }
        assert!(calls_to(e, CTOR_DEFAULT_COLLISION_FILTER).is_empty());
        // Deactivation without simulation islands is off.
        assert_eq!(e.mem.u8(w + 0xc8), 0);
        assert_eq!(e.mem.u8(w + 0xd5), 0);
        // The continuous simulation.
        let simulation = e.mem.u32(w + 8);
        assert_eq!(
            calls_to(e, CTOR_CONTINUOUS_SIMULATION),
            vec![vec![simulation, w]]
        );
        assert_eq!(e.mem.u32(w + 0xf0), 2);
        // Gravity 9.81 (the default), continuous wanted, generation 2 (== 2 and >= 1).
        let seen = gravity_seen.borrow().clone();
        assert_eq!(seen, vec![0x411c_f5c3, 0x0001_0101]);
        // The deactivation levels with the default gravity length.
        let expected = expected_level(
            f32::from_bits(0x3c23_d70a),
            f32::from_bits(0x3da3_d70a),
            f64::from(f32::from_bits(0x411c_f5c3)),
        );
        for (index, bits) in expected.into_iter().enumerate() {
            assert_eq!(e.mem.u32(w + 0x244 + 0x38 + 4 * index as u32), bits);
        }
        // Without simulation islands: one active island (the array grew first).
        assert_eq!(calls_to(e, CTOR_SIMULATION_ISLAND).len(), 2);
        assert_eq!(calls_to(e, CTOR_ARRAY_GROW).len(), 1);
        assert_eq!(calls_to(e, CTOR_ARRAY_GROW)[0][1..], [w + 0x28, 4]);
        assert_eq!(e.mem.u32(w + 0x2c), 1);
        let active = e.mem.u32(e.mem.u32(w + 0x28));
        assert_ne!(active, e.mem.u32(w + 0x20));
        assert_eq!(e.mem.u16(active + 0x20), 0);
        // Border behaviour 3: no border; the kd-tree manager; the collision callbacks.
        assert_eq!(e.mem.u32(w + 0x1bc), 0);
        assert!(calls_to(e, CTOR_BROAD_PHASE_BORDER).is_empty());
        let manager = e.mem.u32(w + 0x5c);
        assert_eq!(calls_to(e, CTOR_KD_TREE_MANAGER), vec![vec![manager, w, 0]]);
        assert_eq!(e.mem.u8(w + 0x60), 1);
        assert_eq!(calls_to(e, CTOR_COLLISION_CALLBACKS), vec![vec![w]]);
    }

    #[test]
    fn world_constructor_builds_the_multithreaded_simulation_when_forced() {
        let mut rig = rig();
        let c = rig.cinfo;
        rig.e.set_global(CTOR_FORCE_MULTITHREADED, 1u8);
        rig.e.mem.set_u8(c + 0x51, 1);
        rig.e.mem.set_u8(c + 0x52, 0xff);
        rig.e.mem.set_u8(c + 0x28, 0x80);
        rig.e.mem.set_u8(c + 0x29, 1);
        construct(&mut rig, 0x111d4);
        let (e, w) = (&rig.e, rig.world);
        assert_eq!(e.mem.u8(c + 0xd1), 3);
        assert_eq!(e.mem.u32(w + 0xf0), 3);
        let sections = calls_to(e, CTOR_CRITICAL_SECTION);
        assert_eq!(sections.len(), 3);
        assert_eq!(sections[0][0], e.mem.u32(w + 0xb8));
        assert_eq!(sections[1][0], e.mem.u32(w + 0xc4));
        assert_eq!(sections[2][0], e.mem.u32(w + 0xc0));
        let simulation = e.mem.u32(w + 8);
        assert_eq!(
            calls_to(e, CTOR_MULTI_THREADED_SIMULATION),
            vec![vec![simulation, w]]
        );
        assert_eq!(e.mem.u32(w + 0xb4), 77);
        // The AABB-tree manager with the sign-extended update type and 0.0.
        let manager = e.mem.u32(w + 0x5c);
        assert_eq!(
            calls_to(e, CTOR_AABB_TREE_MANAGER),
            vec![vec![manager, w, 0xffff_ffff, 0]]
        );
        // The border behaviour is sign-extended, the second byte zero-extended.
        let border = e.mem.u32(w + 0x1bc);
        assert_eq!(
            calls_to(e, CTOR_BROAD_PHASE_BORDER),
            vec![vec![border, w, 0xffff_ff80, 1]]
        );
        // The multi-thread check stays on.
        assert!(calls_to(e, CTOR_MULTI_THREAD_CHECK_DISABLE).is_empty());
    }

    #[test]
    fn world_constructor_survives_failed_allocations_and_no_simulation() {
        let mut rig = rig();
        let c = rig.cinfo;
        rig.failing
            .borrow_mut()
            .extend_from_slice(&[0x204, 0x28, 0x18, 0x120]);
        rig.e.mem.set_u8(c + 0xd1, 0);
        // A simulation the world already points to (the constructor leaves +8 alone).
        let simulation = rig.e.mem.alloc(0x40);
        rig.e.mem.set_u32(rig.world + 8, simulation);
        rig.e.mem.set_f32(c + 0xdc, 2.5);
        // A zero step time takes the zero inverse.
        rig.e.set_global(CTOR_STEP_DOUBLE, 0u64);
        construct(&mut rig, 0x111d4);
        let (e, w) = (&rig.e, rig.world);
        assert_eq!(e.mem.u32(w + 0x1b8), 0);
        assert_eq!(e.mem.u32(w + 0x88), 0);
        assert!(calls_to(e, CTOR_OPERATION_QUEUE).is_empty());
        assert_eq!(e.mem.u32(w + 0xc0), 0);
        assert!(calls_to(e, CTOR_CRITICAL_SECTION).is_empty());
        assert_eq!(e.mem.u32(w + 0x74), 0);
        assert_eq!(e.mem.u32(w + 8), simulation);
        assert_eq!(f32_at(e, simulation + 0x24), 2.5);
        assert_eq!(e.mem.u32(w + 0xf0), 0);
        for constructor in [
            CTOR_SIMULATION,
            CTOR_CONTINUOUS_SIMULATION,
            CTOR_MULTI_THREADED_SIMULATION,
        ] {
            assert!(calls_to(e, constructor).is_empty());
        }
        assert_eq!(e.mem.u32(w + 0x1d8), 0);
        assert_eq!(e.mem.u32(w + 0x1dc), 0);
    }
}

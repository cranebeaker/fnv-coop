//! `Havok SDK/hkpcharacterproxy.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpCharacterProxy` (Xbox PDB, RTTI `.?AVhkpCharacterProxy@@` confirmed at the vtable
//! `010c84a4`) is the Havok character shape controller: it keeps a manifold of contact
//! points (`hkpRootCdPoint`, 0x70 bytes each), the entities and phantoms it listens to and
//! a list of `hkpCharacterProxyListener`s. The PC offsets used here agree with the Xbox PDB.
//!
//! Its vtables are `010c84a4` (the object: slot 0xC is `updateManifold` = `00caf4d0`, slot
//! 0x10 is `extractSurfaceConstraintInfo` = `00cacd60`), `010c8488` (`hkpEntityListener`
//! at +8, slot 8 = `00cad740`) and `010c8470` (`hkpPhantomListener` at +0xC, slot 8 =
//! `00cad820`).
//!
//! Floats: x87 code is computed in `f64` and rounded to `f32` at each `float` store;
//! `FLD`/`FSTP` copies go through [`x87_quiet`]; SSE code computes in `f32` (the
//! `RSQRTSS` estimate is the host CPU's, `rsqrt_estimate`). Dot products are the
//! compiler's `MULPS`/`SHUFPS`/`ADDSS` triple: `z + (y + x)` of the lane products.
//! Comparisons follow the exe's flag tests: after `FCOM a, b`, `TEST AH,5; JP` is taken
//! unless `a < b`; `TEST AH,0x41; JNZ` is taken unless `a > b`; `TEST AH,0x44; JNP` is
//! taken only when `a == b`.
//!
//! Havok's profiling records (`RDTSC` time stamps appended to the thread's monitor stream,
//! found with `TlsGetValue` of the index at `0126835c`) are written as the exe does; the
//! engine has no model of `RDTSC`, so the host clock stands in for the time stamps.
//!
//! The Havok "stack" allocator (`hkMemoryRouter`, `00c85750`: word +0 is the stack top,
//! word +0x10 the heap allocator) carves the temporary arrays; the translations bump and
//! restore the top as the code does.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// `hkMemoryRouter` lookup (`00c85750`, no arguments).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, elemSize)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkArrayUtil::_reserve` (`00c908c0`, cdecl `(allocator, array, count, elemSize)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkReferencedObject` reference counting (`this` only).
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpShapePhantom::setPosition` (Xbox PDB, `00c9e990`, `(position, float)`).
const SHAPE_PHANTOM_SET_POSITION: u32 = 0x00c9_e990;
/// `hkpWorld::linearCast` (Xbox PDB, `00c92430`, `(collidable, input, castCollector,
/// startCollector)`).
const WORLD_LINEAR_CAST: u32 = 0x00c9_2430;
/// `hkpEntity` listener functions (`this` = the entity, one argument: the listener):
/// `00c9c670` removes it, `00c9c5b0` adds it.
const ENTITY_REMOVE_LISTENER: u32 = 0x00c9_c670;
const ENTITY_ADD_LISTENER: u32 = 0x00c9_c5b0;
/// The same for `hkpPhantom`: `00d07c10` removes, `00d07da0` adds.
const PHANTOM_REMOVE_LISTENER: u32 = 0x00d0_7c10;
const PHANTOM_ADD_LISTENER: u32 = 0x00d0_7da0;
/// `hkpEntity::activate` (Xbox PDB, `00c9c1d0`).
const ENTITY_ACTIVATE: u32 = 0x00c9_c1d0;
/// `hkpWorldObject::addProperty` (Xbox PDB, `00cd2be0`, `(key, value low, value high)`) and
/// `removeProperty` (Xbox PDB, `00cd2a50`, `(out value, key)`).
const WORLD_OBJECT_ADD_PROPERTY: u32 = 0x00cd_2be0;
const WORLD_OBJECT_REMOVE_PROPERTY: u32 = 0x00cd_2a50;
/// The property key under which a phantom stores the proxy that owns it.
const PROXY_PROPERTY_KEY: u32 = 0x1300;
/// `hkVector4::setMul3` (Xbox PDB, `00c7f750`, `this` = result, `(matrix, vector)`).
const VECTOR_SET_MUL3: u32 = 0x00c7_f750;
/// `getAllocatedSize` of the statistics collector (`00d06f80`, cdecl `(pointer, bytes)`).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// `hkpAllCdPointCollector::sortHits` (Xbox PDB, `00cabad0`).
const SORT_HITS: u32 = 0x00ca_bad0;
/// `hkpRootCdPoint::operator=` (`006212c0`, `this` = destination, `(source)`).
const ROOT_CD_POINT_ASSIGN: u32 = 0x0062_12c0;
/// Velocity of a body at a contact (`00ca0c40`, `this` = the body, `(hit, out vector)`).
const BODY_POINT_VELOCITY: u32 = 0x00ca_0c40;
/// The simplex solver (`00d24360`, cdecl `(input, output)`).
const SIMPLEX_SOLVE: u32 = 0x00d2_4360;
/// `TlsGetValue` import slot and the TLS index of the Havok monitor stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// `acos` (`00eca1d0`) and `_CIcos` (`00ec9f70`), argument in ST0 as a leading `f64`.
const ACOS: u32 = 0x00ec_a1d0;
const COS: u32 = 0x00ec_9f70;

/// `float` constants of the exe's data.
const FLOAT_ZERO: u32 = 0x0101_1d78;
const FLOAT_ONE: u32 = 0x0101_2068;
const FLOAT_THREE: u32 = 0x0101_7718;
const FLOAT_HALF: u32 = 0x0101_6248;
const FLOAT_PI: u32 = 0x0102_b3c8;
/// `0.01`.
const ONE_HUNDREDTH: u32 = 0x0101_3ea4;
/// `1.1920929e-7` and its negation.
const EPSILON: u32 = 0x0104_da4c;
const NEGATIVE_EPSILON: u32 = 0x010c_8440;
/// `0.001`, the tolerance of the velocity comparisons.
const TOLERANCE: u32 = 0x0101_7d00;
/// `-0.08`.
const SUPPORT_LIMIT: u32 = 0x010c_851c;
/// `0.1`, the distance below which a hit is not a new contact point.
const SAME_POINT_DISTANCE: u32 = 0x0101_e2bc;
/// `1/60`.
const SOLVER_TIME_STEP: u32 = 0x010c_42e8;
/// `1.1`, the limit of the nearest-point search of `updateManifold`.
const NEAREST_LIMIT: u32 = 0x0105_6528;
/// Mask clearing the sign of four lanes, and the vector of four ones.
const ABS_MASK: u32 = 0x0101_7d10;
const ONES_VECTOR: u32 = 0x010e_c7f0;
/// `double` constants: `0.0`, `0.9`, `0.4`, `0.5`, `0.1`, `10.0`, `0.99`.
const DOUBLE_ZERO: u32 = 0x0101_2060;
const DOUBLE_NINE_TENTHS: u32 = 0x0106_b9e8;
const DOUBLE_FOUR_TENTHS: u32 = 0x0102_90c0;
const DOUBLE_HALF: u32 = 0x0101_1588;
const DOUBLE_TENTH: u32 = 0x0101_ffa0;
const DOUBLE_TEN: u32 = 0x0102_0758;
const DOUBLE_NEAR_ONE: u32 = 0x0106_d508;
/// The distance a collector starts with (the float `0x7f7fffee`).
const FAR_DISTANCE: u32 = 0x0104_da48;

/// Vtables: the proxy's three, the base classes the constructor and destructor write, and
/// the collector's (`hkpAllCdPointCollector`, base `0104da3c`).
const PROXY_VTABLE: u32 = 0x010c_84a4;
const ENTITY_LISTENER_VTABLE: u32 = 0x010c_8488;
const PHANTOM_LISTENER_VTABLE: u32 = 0x010c_8470;
const REFERENCED_OBJECT_VTABLE: u32 = 0x0102_e388;
const BASE_ENTITY_LISTENER_VTABLE: u32 = 0x0102_f13c;
const BASE_PHANTOM_LISTENER_VTABLE: u32 = 0x0104_e3d0;
const ALL_CD_POINT_COLLECTOR_VTABLE: u32 = 0x0104_da28;
const CD_POINT_COLLECTOR_VTABLE: u32 = 0x0104_da3c;

/// Profiling record labels.
const LABEL_CHECK_SUPPORT: u32 = 0x010c_8520;
const LABEL_END: u32 = 0x010c_41f8;
const LABEL_START_FIRST: u32 = 0x010c_8508;
const LABEL_START_SECOND: u32 = 0x010c_8500;
const LABEL_INITIAL_CAST: u32 = 0x010c_84f0;
const LABEL_UPDATE_MANIFOLD: u32 = 0x010c_84dc;
const LABEL_SOLVE: u32 = 0x010c_84d0;
const LABEL_APPLY_SURFACE: u32 = 0x010c_84c4;
const LABEL_CAST_MOVE: u32 = 0x010c_84b8;
const LABEL_INTEGRATE_END: u32 = 0x0107_6f38;
/// Statistics names (`"Manifold"`, `"OverlapsPntr"`, `"ListnrPntr"`).
const NAME_MANIFOLD: u32 = 0x010c_8460;
const NAME_OVERLAPS: u32 = 0x010c_8450;
const NAME_LISTENERS: u32 = 0x010c_8444;

/// Size of an `hkpRootCdPoint` and of the record `applySurfaceInteractions` writes.
const ROOT_CD_POINT_SIZE: u32 = 0x70;
const SURFACE_RECORD_SIZE: u32 = 0x30;
/// Capacity word of an `hkArray` without owned storage / with inline storage for 8.
const NO_STORAGE: u32 = 0x8000_0000;
const INLINE_EIGHT: u32 = 0x8000_0008;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

layout! {
    /// `hkpCharacterProxy` (Xbox PDB), 0xC0 bytes on the Xbox (the PC size is not
    /// confirmed; the constructor writes up to +0xB0). Vectors are 16-byte `hkVector4`s;
    /// the `hkArray`s are `{data, size, capacityAndFlags}`.
    pub struct HkpCharacterProxy: 0xC0 {
        /// Vtable of the object (`010c84a4`).
        0x00 vftable: u32,
        /// `m_referenceCount` (Xbox PDB).
        0x06 m_referenceCount: i16,
        /// Vtable of the `hkpEntityListener` base.
        0x08 entityListenerVftable: u32,
        /// Vtable of the `hkpPhantomListener` base.
        0x0C phantomListenerVftable: u32,
        /// `m_velocity` (Xbox PDB), an `hkVector4`.
        0x10 m_velocity: Inline<()>,
        /// `m_oldDisplacement` (Xbox PDB), an `hkVector4`.
        0x20 m_oldDisplacement: Inline<()>,
        /// `m_shapePhantom` (Xbox PDB): `hkpShapePhantom*`.
        0x30 m_shapePhantom: Ptr,
        /// `m_dynamicFriction` (Xbox PDB).
        0x34 m_dynamicFriction: f32,
        /// `m_staticFriction` (Xbox PDB).
        0x38 m_staticFriction: f32,
        /// `m_up` (Xbox PDB), an `hkVector4`.
        0x40 m_up: Inline<()>,
        /// `m_extraUpStaticFriction` (Xbox PDB).
        0x50 m_extraUpStaticFriction: f32,
        /// `m_extraDownStaticFriction` (Xbox PDB).
        0x54 m_extraDownStaticFriction: f32,
        /// `m_keepDistance` (Xbox PDB).
        0x58 m_keepDistance: f32,
        /// `m_keepContactTolerance` (Xbox PDB).
        0x5C m_keepContactTolerance: f32,
        /// `m_contactAngleSensitivity` (Xbox PDB).
        0x60 m_contactAngleSensitivity: f32,
        /// `m_userPlanes` (Xbox PDB).
        0x64 m_userPlanes: i32,
        /// `m_maxCharacterSpeedForSolver` (Xbox PDB).
        0x68 m_maxCharacterSpeedForSolver: f32,
        /// `m_characterStrength` (Xbox PDB).
        0x6C m_characterStrength: f32,
        /// `m_characterMass` (Xbox PDB).
        0x70 m_characterMass: f32,
        /// `m_manifold` (Xbox PDB), an `hkArray<hkpRootCdPoint>`: data.
        0x74 m_manifold: Ptr,
        /// Size of the manifold.
        0x78 m_manifoldSize: i32,
        /// Capacity and flags of the manifold.
        0x7C m_manifoldCapacityAndFlags: u32,
        /// `m_listeners` (Xbox PDB), an `hkArray<hkpCharacterProxyListener*>`: data.
        0x80 m_listeners: Ptr,
        /// Size of the listener list.
        0x84 m_listenersSize: i32,
        /// Capacity and flags of the listener list.
        0x88 m_listenersCapacityAndFlags: u32,
        /// `m_bodies` (Xbox PDB), an `hkArray<hkpRigidBody*>`: data.
        0x8C m_bodies: Ptr,
        /// Size of the body list.
        0x90 m_bodiesSize: i32,
        /// Capacity and flags of the body list.
        0x94 m_bodiesCapacityAndFlags: u32,
        /// `m_phantoms` (Xbox PDB), an `hkArray<hkpPhantom*>`: data.
        0x98 m_phantoms: Ptr,
        /// Size of the phantom list.
        0x9C m_phantomsSize: i32,
        /// Capacity and flags of the phantom list.
        0xA0 m_phantomsCapacityAndFlags: u32,
        /// `m_maxSlopeCosine` (Xbox PDB).
        0xA4 m_maxSlopeCosine: f32,
        /// `m_penetrationRecoverySpeed` (Xbox PDB).
        0xA8 m_penetrationRecoverySpeed: f32,
        /// `m_maxCastIterations` (Xbox PDB).
        0xAC m_maxCastIterations: i32,
        /// `m_refreshManifoldInCheckSupport` (Xbox PDB).
        0xB0 m_refreshManifoldInCheckSupport: bool,
    }
}

// ---------------------------------------------------------------------------
// Helpers: vectors, floats, Havok arrays and the stack allocator.
// ---------------------------------------------------------------------------

/// The four lanes of the `hkVector4` at `addr`.
fn load4(e: &Engine, addr: u32) -> [f32; 4] {
    [0, 4, 8, 12].map(|o| e.mem.f32(addr.wrapping_add(o)))
}

/// Stores the four lanes at `addr`.
fn store4(e: &mut Engine, addr: u32, v: [f32; 4]) {
    for (i, lane) in v.iter().enumerate() {
        e.mem.set_f32(addr.wrapping_add(4 * i as u32), *lane);
    }
}

/// Copies `n` bytes (`MOVAPS` / `MOVSD` copies keep the bits).
fn copy_bytes(e: &mut Engine, dst: u32, src: u32, n: u32) {
    let bytes = e.mem.bytes(src, n);
    e.mem.write(dst, &bytes);
}

/// An `FLD dword` / `FSTP dword` copy: a signalling NaN comes back quiet.
fn copy_quiet(e: &mut Engine, dst: u32, src: u32) {
    let bits = x87_quiet_bits(e.mem.u32(src));
    e.mem.set_u32(dst, bits);
}

/// The `MULPS`/`SHUFPS`/`ADDSS` dot product of three lanes: `z + (y + x)`.
fn dot3(a: [f32; 4], b: [f32; 4]) -> f32 {
    let product = |i: usize| a[i] * b[i];
    product(2) + (product(1) + product(0))
}

/// The refined `RSQRTSS` scale of the inlined `hkVector4::normalize3`:
/// `(half * r) * (three - (sum * r) * r)`, and `0.0` when `sum == zero`.
fn normalizing_scale(e: &Engine, sum: f32) -> f32 {
    let zero: f32 = e.global(FLOAT_ZERO);
    let three: f32 = e.global(FLOAT_THREE);
    let half: f32 = e.global(FLOAT_HALF);
    let estimate = rsqrt_estimate(sum);
    let value = (half * estimate) * (three - (sum * estimate) * estimate);
    if sum == zero {
        0.0
    } else {
        value
    }
}

/// The vector scaled by [`normalizing_scale`] of its squared length, in all four lanes.
fn normalized(e: &Engine, v: [f32; 4]) -> [f32; 4] {
    let scale = normalizing_scale(e, dot3(v, v));
    v.map(|lane| scale * lane)
}

/// `vector` multiplied lane by lane by `scale`.
fn scaled(v: [f32; 4], scale: f32) -> [f32; 4] {
    v.map(|lane| lane * scale)
}

/// Lane-wise `a + b`.
fn add4(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

/// Lane-wise `a - b`.
fn sub4(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

/// Lane-wise `a * b`.
fn mul4(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

/// The `SHUFPS` pattern `[1, 2, 0, 3]`.
fn shuffle_120(v: [f32; 4]) -> [f32; 4] {
    [v[1], v[2], v[0], v[3]]
}

/// The `SHUFPS` pattern `[2, 0, 1, 3]`.
fn shuffle_201(v: [f32; 4]) -> [f32; 4] {
    [v[2], v[0], v[1], v[3]]
}

/// Whether the first three lanes of `|a - b|` are all at most the tolerance (`ANDPS`,
/// `CMPLEPS`, `MOVMSKPS & 7 == 7`).
fn lanes_within_tolerance(e: &Engine, a: [f32; 4], b: [f32; 4]) -> bool {
    let tolerance: f32 = e.global(TOLERANCE);
    let diff = sub4(a, b);
    (0..3).all(|i| {
        let magnitude = f32::from_bits(diff[i].to_bits() & e.mem.u32(ABS_MASK + 4 * i as u32));
        magnitude <= tolerance
    })
}

/// The hkMemoryRouter of the thread.
fn memory_router(e: &mut Engine) -> u32 {
    e.call(MEMORY_ROUTER, &args![]).u32()
}

/// Carves `bytes` (rounded up to a multiple of 128) off the stack allocator and returns the
/// old top.
fn stack_allocate(e: &mut Engine, bytes: u32) -> u32 {
    let router = memory_router(e);
    let top = e.mem.u32(router);
    e.mem.set_u32(
        router,
        top.wrapping_add(bytes.wrapping_add(0x7f) & 0xffff_ff80),
    );
    top
}

/// Releases a stack array: when its data is still the slab the size drops to 0, the stack top
/// goes back to `slab_start`, heap storage (if the capacity word owns any) is freed through
/// the allocator's slot 0x10, and the array is left empty without storage.
fn release_stack_array(e: &mut Engine, array: u32, slab_start: u32, element_shift: u32) {
    if e.mem.u32(array) == slab_start {
        e.mem.set_u32(array + 4, 0);
    }
    let router = memory_router(e);
    e.mem.set_u32(router, slab_start);
    let router = memory_router(e);
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & NO_STORAGE == 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & CAPACITY_MASK) << element_shift;
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NO_STORAGE);
}

/// The inlined `hkArray::pushBack` growth check: when the array is full,
/// `_reserveMore(allocator, array, elemSize)`.
fn reserve_more_if_full(e: &mut Engine, array: u32, element_size: u32) {
    let router = memory_router(e);
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, element_size]);
    }
}

/// `hkArray<void*>::pushBack`.
fn push_pointer(e: &mut Engine, array: u32, value: u32) {
    reserve_more_if_full(e, array, 4);
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

/// The inlined `hkArray::reserve(needed)` of a stack array: when the capacity is below
/// `needed`, `_reserve(allocator, array, max(needed, 2 * capacity), elemSize)`.
fn reserve_array(e: &mut Engine, array: u32, needed: i32, element_size: u32) {
    let router = memory_router(e);
    let allocator = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
    if capacity < needed {
        let doubled = capacity.wrapping_add(capacity);
        let count = if needed < doubled { doubled } else { needed };
        e.call(ARRAY_RESERVE, &args![allocator, array, count, element_size]);
    }
}

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// The thread's monitor stream (`TlsGetValue` of the index at `0126835c`).
fn monitor_stream(e: &mut Engine) -> u32 {
    let index: u32 = e.global(MONITOR_SLOT);
    e.call(TLS_GET_VALUE, &args![index]).u32()
}

/// Appends a 12-byte record `(label, time stamp)` to the monitor stream while its cursor (+4)
/// is below its limit (+0xC).
fn profile_record(e: &mut Engine, label: u32) {
    let stream = monitor_stream(e);
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// The 16-byte start record of `integrateImplementation`: two labels around the time stamp.
fn profile_record_wide(e: &mut Engine) {
    let stream = monitor_stream(e);
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, LABEL_START_FIRST);
        e.mem.set_u32(cursor + 0xc, LABEL_START_SECOND);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0x10);
    }
}

/// The owner of a collidable: `collidable + (signed byte at +0x10)` when the byte at +0x18
/// is `kind` (1 entity, 2 phantom), else null.
fn collidable_owner(e: &Engine, collidable: u32, kind: u8) -> u32 {
    if e.mem.u8(collidable + 0x18) == kind {
        collidable.wrapping_add(e.mem.i8(collidable + 0x10) as i32 as u32)
    } else {
        0
    }
}

/// The index of `value` in the pointer array `{data, size}` at `array`, `-1` if absent.
fn find_pointer(e: &Engine, array: u32, value: u32) -> i32 {
    let size = e.mem.i32(array + 4);
    let data = e.mem.u32(array);
    (0..size)
        .find(|i| e.mem.u32(data.wrapping_add(4 * *i as u32)) == value)
        .unwrap_or(-1)
}

/// `array.removeAt(index)` of a pointer array as the compiler writes it: the size drops, and
/// when the new size differs from `index` the last element moves into `index` (an `index` of
/// -1 writes before the data, as the code does).
fn remove_pointer_at(e: &mut Engine, array: u32, index: i32) {
    let size = e.mem.i32(array + 4).wrapping_sub(1);
    e.mem.set_i32(array + 4, size);
    if size != index {
        let data = e.mem.u32(array);
        let last = e.mem.u32(data.wrapping_add((size as u32).wrapping_mul(4)));
        e.mem
            .set_u32(data.wrapping_add((index as u32).wrapping_mul(4)), last);
    }
}

/// Copies `bytes` from `src` to `dst` in 8-byte chunks (the `MOVSD`-style copy of the hit
/// arrays; overlapping ranges are copied forward like the code does).
fn copy_points(e: &mut Engine, dst: u32, src: u32, bytes: u32) {
    let mut done = 0;
    while done < bytes {
        let chunk = e.mem.u64(src.wrapping_add(done));
        e.mem.set_u64(dst.wrapping_add(done), chunk);
        done += 8;
    }
}

// ---------------------------------------------------------------------------
// Accessors and small helpers of the proxy.
// ---------------------------------------------------------------------------

// Translated from 00cac890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Re-weights the hits of a linear cast against `vector`: for each of `count` points
/// (0x70 apart, starting at `hits`) the `float` at +0x1C moves to +0x0C and is replaced by
/// `old * -(normal . vector)`, `normal` being the vector at +0x10. `this` (the proxy) is
/// passed but never read.
pub fn fn_00cac890(
    e: &mut Engine,
    _this: Ptr<HkpCharacterProxy>,
    hits: Ptr,
    count: i32,
    vector: Ptr,
) {
    let direction = load4(e, vector.addr());
    let mut remaining = count.wrapping_sub(1);
    let mut hit = hits.addr();
    while remaining >= 0 {
        let weight = x87_quiet(e.mem.f32(hit.wrapping_add(0x1c)));
        let normal = load4(e, hit.wrapping_add(0x10));
        e.mem.set_f32(hit.wrapping_add(0xc), weight);
        let dot = dot3(normal, direction);
        e.mem.set_f32(
            hit.wrapping_add(0x1c),
            (weight as f64 * -(dot as f64)) as f32,
        );
        hit = hit.wrapping_add(ROOT_CD_POINT_SIZE);
        remaining -= 1;
    }
}

// Translated from 00cac910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterProxy::getPosition` (Xbox PDB): the address of the translation of the
/// shape phantom's motion state (`[[phantom + 0x18]] + 0x30`, read as `phantom->+0x18`).
pub fn hkp_character_proxy_get_position(e: &mut Engine, this: Ptr<HkpCharacterProxy>) -> Ptr {
    let phantom = e.get(this, HkpCharacterProxy::m_shapePhantom);
    let motion_state = e.mem.u32(phantom.addr() + 0x18);
    Ptr::new(motion_state.wrapping_add(0x30))
}

// Translated from 00cac920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterProxy::setPosition` (Xbox PDB): moves the shape phantom to `position`,
/// passing `keepContactTolerance + keepDistance` along.
pub fn hkp_character_proxy_set_position(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    position: Ptr,
) {
    let tolerance = e.get(this, HkpCharacterProxy::m_keepContactTolerance);
    let distance = e.get(this, HkpCharacterProxy::m_keepDistance);
    let sum = (tolerance as f64 + distance as f64) as f32;
    let phantom = e.get(this, HkpCharacterProxy::m_shapePhantom);
    e.call(SHAPE_PHANTOM_SET_POSITION, &args![phantom, position, sum]);
}

// Translated from 00cac950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterProxy::getLinearVelocity` (Xbox PDB): the address of `m_velocity`.
pub fn hkp_character_proxy_get_linear_velocity(
    _e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
) -> Ptr {
    this.at(HkpCharacterProxy::m_velocity).cast()
}

// Translated from 00cac960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the `hkVector4` at `velocity` into `m_velocity` (the setter that goes with
/// `getLinearVelocity`; the map has no name for it).
pub fn fn_00cac960(e: &mut Engine, this: Ptr<HkpCharacterProxy>, velocity: Ptr) {
    copy_bytes(e, this.addr() + 0x10, velocity.addr(), 16);
}

// Translated from 00cac980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `m_shapePhantom` getter (the map has no name for it).
pub fn fn_00cac980(e: &mut Engine, this: Ptr<HkpCharacterProxy>) -> Ptr {
    e.get(this, HkpCharacterProxy::m_shapePhantom)
}

// Translated from 00cac9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills the `hkpCharacterProxyCinfo` at `cinfo` from the proxy (the inverse of
/// `00cad500`; the map has no name for it). The cinfo's `m_maxSlope` is the angle whose
/// cosine the proxy keeps in `m_maxSlopeCosine`: `acos` of it when `|cosine| < 1`,
/// otherwise `0` for a positive cosine and pi for the rest.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn fn_00cac9a0(e: &mut Engine, this: Ptr<HkpCharacterProxy>, cinfo: Ptr) {
    let out = cinfo.addr();
    let me = this.addr();
    let position = hkp_character_proxy_get_position(e, this);
    copy_bytes(e, out, position.addr(), 16);
    copy_bytes(e, out + 0x10, me + 0x10, 16);
    let phantom = e.mem.u32(me + 0x30);
    e.mem.set_u32(out + 0x48, phantom);
    copy_quiet(e, out + 0x20, me + 0x34);
    copy_quiet(e, out + 0x24, me + 0x38);
    copy_quiet(e, out + 0x28, me + 0x5c);
    copy_bytes(e, out + 0x30, me + 0x40, 16);
    copy_quiet(e, out + 0x40, me + 0x50);
    copy_quiet(e, out + 0x44, me + 0x54);
    copy_quiet(e, out + 0x4c, me + 0x58);
    copy_quiet(e, out + 0x50, me + 0x60);
    let user_planes = e.mem.u32(me + 0x64);
    e.mem.set_u32(out + 0x54, user_planes);
    copy_quiet(e, out + 0x58, me + 0x68);
    copy_quiet(e, out + 0x5c, me + 0x6c);
    copy_quiet(e, out + 0x60, me + 0x70);
    let cosine = x87_quiet(e.get(this, HkpCharacterProxy::m_maxSlopeCosine));
    let one: f32 = e.global(FLOAT_ONE);
    let slope = if !(cosine.abs() >= one) {
        e.call(ACOS, &args![cosine as f64]).f64() as f32
    } else if 0.0 < cosine {
        0.0
    } else {
        e.global(FLOAT_PI)
    };
    e.mem.set_f32(out + 0x64, slope);
    copy_quiet(e, out + 0x68, me + 0xa8);
    let iterations = e.mem.u32(me + 0xac);
    e.mem.set_u32(out + 0x6c, iterations);
    let refresh = e.mem.u8(me + 0xb0);
    e.mem.set_u8(out + 0x70, refresh);
}

// Translated from 00cacaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterProxy::linearCast` (Xbox PDB). With `use_world` set it hands the shape
/// phantom's collidable to `hkpWorld::linearCast`; otherwise it builds the cast's
/// `hkpLinearCastCollisionInput` (the world's, with the old displacement, the cast's
/// length and the input's float at +0x10) and runs every collision agent the phantom
/// keeps: for an entity phantom (`getType() == 1`) the dispatcher's linear-cast function
/// for each pair of shape types, otherwise the `linearCast` virtual (slot 0x14) of each
/// registered overlapping object.
pub fn hkp_character_proxy_linear_cast(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    use_world: bool,
    input: Ptr,
    cast_collector: Ptr,
    start_collector: Ptr,
) {
    let phantom = e.get(this, HkpCharacterProxy::m_shapePhantom).addr();
    let world = e.mem.u32(phantom + 8);
    let collidable = phantom + 0x10;
    if use_world {
        e.call(
            WORLD_LINEAR_CAST,
            &args![world, collidable, input, cast_collector, start_collector],
        );
        return;
    }
    let world_input = e.mem.u32(world + 0x78);
    let old_displacement = load4(e, this.addr() + 0x20);
    let length = dot3(old_displacement, old_displacement).sqrt();
    e.with_stack(0x80, |e, frame| {
        let q = frame.addr();
        for offset in [0u32, 4, 8, 12, 16, 20] {
            let word = e.mem.u32(world_input + offset);
            e.mem.set_u32(q + offset, word);
        }
        copy_quiet(e, q + 8, input.addr() + 0x14);
        for offset in [0x20u32, 0x30, 0x40] {
            copy_bytes(e, q + offset, world_input + offset, 16);
        }
        store4(e, q + 0x50, old_displacement);
        copy_quiet(e, q + 0x60, input.addr() + 0x10);
        e.mem.set_f32(q + 0x64, x87_quiet(length));
        let tail = e.mem.u32(world_input + 0x70);
        e.mem.set_u32(q + 0x68, tail);
        let kind = e.vcall(phantom, 0x14, &args![]).u32();
        let data = e.mem.u32(phantom + 0x160);
        let mut index = e.mem.i32(phantom + 0x164).wrapping_sub(1);
        if kind == 1 {
            let table = e.mem.u32(world + 0x80);
            while index >= 0 {
                let other = e.mem.u32(data.wrapping_add(4 * index as u32));
                let first_shape = e.mem.u32(collidable);
                let second_shape = e.mem.u32(other);
                let pair = e
                    .mem
                    .u32(first_shape + 0xc)
                    .wrapping_add(13)
                    .wrapping_mul(32)
                    .wrapping_add(e.mem.u32(second_shape + 0xc));
                let cell = e.mem.u8(table.wrapping_add(pair)) as u32;
                let function = e.mem.u32(table.wrapping_add(cell * 0x14 + 0x9ac));
                e.call(
                    function,
                    &args![collidable, other, q, cast_collector, start_collector],
                );
                index -= 1;
            }
        } else {
            while index >= 0 {
                let entry = data.wrapping_add(8 * index as u32);
                let object = e.mem.u32(entry);
                let other = e.mem.u32(entry + 4);
                e.vcall(
                    object,
                    0x14,
                    &args![collidable, other, q, cast_collector, start_collector],
                );
                index -= 1;
            }
        }
    });
}

// Translated from 00cacc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `result` toward `start` by the fraction `s` of a cast that stopped at a hit (the map
/// has no name for it). `displacement` is the vector the cast moved along (its word at +0x20
/// is the weight returned), `cast` points at the hit (`[cast + 0x10]`: fraction at +0xC,
/// normal at +0x10), and `s = clamp(fraction - (keepDistance / -(normal . displacement /
/// length)) / length, 0, 1)`. `result` becomes `(1 - s) * result + start * s` and the return
/// value is `weight * s`.
pub fn fn_00cacc50(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    displacement: Ptr,
    cast: Ptr,
    start: Ptr,
    result: Ptr,
) -> f32 {
    let a = load4(e, displacement.addr());
    let hit = e.mem.u32(cast.addr() + 0x10);
    let fraction = e.mem.f32(hit + 0xc);
    let keep_distance = e.get(this, HkpCharacterProxy::m_keepDistance);
    let length_squared = dot3(a, a);
    let along = dot3(load4(e, hit + 0x10), a);
    let length = x87_quiet(length_squared.sqrt());
    let cosine = (along as f64 / length as f64) as f32;
    let travelled = (keep_distance as f64 / -(cosine as f64)) as f32;
    let t = (fraction as f64 - travelled as f64 / length as f64) as f32;
    let upper = if t < 1.0 { t } else { 1.0 };
    let zero: f64 = e.global(DOUBLE_ZERO);
    let blend = if (upper as f64) < zero { 0.0 } else { upper };
    let ones = load4(e, ONES_VECTOR);
    let current = load4(e, result.addr());
    let target = load4(e, start.addr());
    let keep = mul4(sub4(ones, [blend; 4]), current);
    let move_toward = scaled(target, blend);
    store4(e, result.addr(), add4(keep, move_toward));
    let weight = e.mem.f32(displacement.addr() + 0x20);
    (weight as f64 * blend as f64) as f32
}

// Translated from 00cacd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterProxy::extractSurfaceConstraintInfo` (Xbox PDB, vtable slot 0x10): fills the
/// 0x34-byte surface record `info` for the contact `hit`: the plane (normal and distance
/// minus `keepDistance`), the friction values, and the velocity of the surface at the
/// contact (`00ca0c40` for a keyframed or fixed body, else from the body's linear and angular
/// motion), plus a type word (2 for a body of type 5, 1 for type 4). A penetration deeper
/// than epsilon is turned into extra velocity along the normal.
pub fn hkp_character_proxy_extract_surface_constraint_info(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    hit: Ptr,
    info: Ptr,
    time: f32,
) {
    let me = this.addr();
    let p = hit.addr();
    let r = info.addr();
    copy_bytes(e, r, p + 0x10, 16);
    let distance = x87_quiet(e.mem.f32(r + 0xc));
    let keep_distance = e.mem.f32(me + 0x58);
    e.mem
        .set_f32(r + 0xc, (distance as f64 - keep_distance as f64) as f32);
    copy_quiet(e, r + 0x20, me + 0x38);
    copy_quiet(e, r + 0x2c, me + 0x34);
    copy_quiet(e, r + 0x24, me + 0x50);
    copy_quiet(e, r + 0x28, me + 0x54);
    store4(e, r + 0x10, [0.0; 4]);
    e.mem.set_u32(r + 0x30, 0);
    let collidable = e.mem.u32(p + 0x48);
    let owner = collidable_owner(e, collidable, 1);
    if owner != 0 {
        let body_type = e.mem.u8(owner + 0xe8);
        if body_type == 5 || body_type == 4 {
            e.call(BODY_POINT_VELOCITY, &args![owner, hit, r + 0x10]);
        } else {
            let factor = x87_quiet(e.mem.f32(owner + 0x14c));
            let linear = scaled(
                sub4(load4(e, owner + 0x140), load4(e, owner + 0x130)),
                factor,
            );
            let angular = scaled(load4(e, owner + 0x180), factor);
            let offset = sub4(load4(e, p), load4(e, owner + 0x140));
            let cross = sub4(
                mul4(shuffle_120(angular), shuffle_201(offset)),
                mul4(shuffle_201(angular), shuffle_120(offset)),
            );
            store4(e, r + 0x10, add4(cross, linear));
        }
        let plane = load4(e, r);
        let velocity = load4(e, r + 0x10);
        let rate = x87_quiet(e.mem.f32(r + 0xc));
        let along = dot3(velocity, plane);
        e.mem
            .set_f32(r + 0xc, (rate as f64 - along as f64 * time as f64) as f32);
        if e.mem.u8(owner + 0xe8) == 5 {
            e.mem.set_u32(r + 0x30, 2);
        }
        if e.mem.u8(owner + 0xe8) == 4 {
            e.mem.set_u32(r + 0x30, 1);
        }
    }
    let negative_epsilon: f32 = e.global(NEGATIVE_EPSILON);
    let depth = e.mem.f32(r + 0xc);
    if negative_epsilon > depth {
        let recovery = x87_quiet(e.mem.f32(me + 0xa8));
        let speed = (recovery as f64 * -(x87_quiet(depth) as f64)) as f32;
        let normal = load4(e, p + 0x10);
        let velocity = load4(e, r + 0x10);
        store4(e, r + 0x10, add4(scaled(normal, speed), velocity));
        e.mem.set_f32(r + 0xc, 0.0);
    }
}

// Translated from 00cacf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The index of the manifold entry closest to `hit` by `fn_00cafd90`'s distance, or -1 when
/// none is closer than 0.1 (the map has no name for it).
pub fn fn_00cacf10(e: &mut Engine, this: Ptr<HkpCharacterProxy>, hit: Ptr) -> i32 {
    let mut best = -1;
    let mut nearest = x87_quiet(e.global::<f32>(SAME_POINT_DISTANCE));
    let manifold = e.get(this, HkpCharacterProxy::m_manifold).addr();
    let size = e.get(this, HkpCharacterProxy::m_manifoldSize);
    let mut index = 0;
    while index < size {
        let entry = manifold.wrapping_add(index as u32 * ROOT_CD_POINT_SIZE);
        let distance = x87_quiet(fn_00cafd90(e, this, hit, Ptr::new(entry)));
        if nearest > distance {
            nearest = distance;
            best = index;
        }
        index += 1;
    }
    best
}

// Translated from 00cafd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// How far apart two contact points are, for matching a new hit with a manifold entry (the
/// map has no name for it): `0.1 * |v1 - v2|^2 + 10 * c^2 * (1 - n1 . n2) + (d1 - d2)^2`, with
/// `c` the proxy's `m_contactAngleSensitivity`, `n` the normals (+0x10), `d` the distances
/// (+0x1C) and `v` the velocities of the bodies at the points (zero for a hit that is not an
/// entity).
pub fn fn_00cafd90(e: &mut Engine, this: Ptr<HkpCharacterProxy>, hit: Ptr, other: Ptr) -> f32 {
    let sensitivity = e.get(this, HkpCharacterProxy::m_contactAngleSensitivity);
    let first = hit.addr();
    let second = other.addr();
    let dot = dot3(load4(e, first + 0x10), load4(e, second + 0x10));
    let flat = (1.0f64 - dot as f64) as f32;
    let angle_term = (sensitivity as f64 * sensitivity as f64 * flat as f64) as f32;
    let gap = (e.mem.f32(first + 0x1c) as f64 - e.mem.f32(second + 0x1c) as f64) as f32;
    let gap_squared = (gap as f64 * gap as f64) as f32;
    let mut velocity_first = [0.0f32; 4];
    let mut velocity_second = [0.0f32; 4];
    let first_collidable = e.mem.u32(first + 0x48);
    let first_owner = collidable_owner(e, first_collidable, 1);
    if first_owner != 0 {
        e.with_stack(0x10, |e, out| {
            e.call(BODY_POINT_VELOCITY, &args![first_owner, hit, out]);
            velocity_first = load4(e, out.addr());
        });
    }
    let second_collidable = e.mem.u32(second + 0x48);
    let second_owner = collidable_owner(e, second_collidable, 1);
    if second_owner != 0 {
        e.with_stack(0x10, |e, out| {
            e.call(BODY_POINT_VELOCITY, &args![second_owner, other, out]);
            velocity_second = load4(e, out.addr());
        });
    }
    let difference = sub4(velocity_first, velocity_second);
    let squared = dot3(difference, difference);
    let tenth: f64 = e.global(DOUBLE_TENTH);
    let ten: f64 = e.global(DOUBLE_TEN);
    ((squared as f64 * tenth + angle_term as f64 * ten) + gap_squared as f64) as f32
}

/// Frees an `hkArray`'s heap storage through the allocator's slot 0x10 (when the capacity
/// word owns any) and leaves it empty: size 0, no data, capacity word `0x80000000`.
fn free_hk_array(e: &mut Engine, array: u32, element_size: u32) {
    let router = memory_router(e);
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & NO_STORAGE == 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(element_size);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, NO_STORAGE);
}

// Translated from 00cacf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterProxy::applySurfaceInteractions` (Xbox PDB; the arguments are the map's
/// `(hkStepInfo, hkVector4 velocity, ...)` plus an optional output list). For each of the
/// first `limit` (or all, without `out`) manifold points it
///
/// 1. tells every listener (`slot 0x10`, `(proxy, property value, point)`) when the owner
///    carries the proxy property (key `0x1300`);
/// 2. for a point on a movable entity (type 1, motion type not 4 or 5) works out the impulse
///    pushing the surface: the closing speed along the normal, an extra push for penetration,
///    and the response through the owner's inverse inertia (`setMul3` and its mass term),
///    clamped by `characterStrength * step`;
/// 3. lets the listeners alter it (`slot 0x14`, `(proxy, point, impulse)`);
/// 4. either appends `{impulse, point, owner}` (0x30 bytes at `*out`) or activates the owner
///    and applies the impulse to it (`slot 0x50` of its motion).
///
/// With `out` the list is closed by clearing the owner word of the next record when fewer
/// than `limit` records were written. The `out` argument points at the cursor.
pub fn hkp_character_proxy_apply_surface_interactions(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    step: Ptr,
    velocity: Ptr,
    out: Ptr,
    limit: i32,
) {
    let has_out = out.addr() != 0;
    let mut count = e.get(this, HkpCharacterProxy::m_manifoldSize);
    let mut cursor = 0u32;
    if has_out {
        if limit < count {
            count = limit;
        }
        cursor = e.mem.u32(out.addr());
    }
    e.with_stack(0x100, |e, frame| {
        let mut offset = 0u32;
        let mut remaining = count;
        while remaining > 0 {
            surface_interaction_step(
                e,
                this,
                step.addr(),
                velocity.addr(),
                frame.addr(),
                offset,
                has_out,
                &mut cursor,
            );
            offset = offset.wrapping_add(ROOT_CD_POINT_SIZE);
            remaining -= 1;
        }
    });
    if has_out {
        let written =
            (cursor.wrapping_sub(e.mem.u32(out.addr())) as i32) / SURFACE_RECORD_SIZE as i32;
        if written < limit {
            e.mem.set_u32(cursor + 0x20, 0);
        }
    }
}

/// One manifold point of `applySurfaceInteractions`; `frame` mirrors the locals of the
/// function (position at +0x40, normal +0x50, impulse +0x80, position copy +0x90, moment arm
/// cross product +0xa0, its transformed value +0xb0, inertia matrix +0xc0).
#[allow(clippy::too_many_arguments)]
fn surface_interaction_step(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    step: u32,
    velocity: u32,
    frame: u32,
    offset: u32,
    has_out: bool,
    cursor: &mut u32,
) {
    let me = this.addr();
    let manifold = e.mem.u32(me + 0x74);
    let entry = manifold.wrapping_add(offset);
    let collidable = e.mem.u32(entry + 0x48);
    let owner = collidable.wrapping_add(e.mem.i8(collidable + 0x10) as i32 as u32);
    let property_count = e.mem.i32(owner + 0x7c);
    let properties = e.mem.u32(owner + 0x78);
    let found = (0..property_count)
        .find(|i| e.mem.u32(properties.wrapping_add(16 * *i as u32)) == PROXY_PROPERTY_KEY);
    if let Some(index) = found {
        let value_low = e.mem.u32(properties.wrapping_add(16 * index as u32 + 8));
        let mut listener_index = e.mem.i32(me + 0x84).wrapping_sub(1);
        while listener_index >= 0 {
            let listeners = e.mem.u32(me + 0x80);
            let listener = e.mem.u32(listeners.wrapping_add(4 * listener_index as u32));
            if listener != 0 {
                let point = e.mem.u32(me + 0x74).wrapping_add(offset);
                e.vcall(listener, 0x10, &args![this, value_low, point]);
            }
            listener_index -= 1;
        }
    }
    let entry = e.mem.u32(me + 0x74).wrapping_add(offset);
    let collidable = e.mem.u32(entry + 0x48);
    if e.mem.u8(collidable + 0x18) != 1 {
        return;
    }
    let owner = collidable.wrapping_add(e.mem.i8(collidable + 0x10) as i32 as u32);
    if owner == 0 {
        return;
    }
    let motion_type = e.mem.u8(owner + 0xe8);
    if motion_type == 5 || motion_type == 4 {
        return;
    }
    let mut position = load4(e, entry);
    position[3] = x87_quiet(e.mem.f32(entry + 0x1c));
    store4(e, frame + 0x40, position);
    let normal = load4(e, entry + 0x10);
    store4(e, frame + 0x50, normal);
    e.mem.set_u32(frame + 0x70, owner);
    let motion = owner + 0xe0;
    e.mem.set_u32(frame + 0x30, motion);
    let arm = sub4(position, load4(e, owner + 0x140));
    let spin = load4(e, owner + 0x1c0);
    let cross = sub4(
        mul4(shuffle_201(arm), shuffle_120(spin)),
        mul4(shuffle_120(arm), shuffle_201(spin)),
    );
    let relative = sub4(add4(cross, load4(e, owner + 0x1b0)), load4(e, me + 0x10));
    let closing = dot3(relative, normal);
    e.mem.set_f32(frame + 0x68, closing);
    let nine_tenths: f64 = e.global(DOUBLE_NINE_TENTHS);
    let mut push = (-(closing as f64) * nine_tenths) as f32;
    let distance = position[3];
    if distance < 0.0 {
        let step_value = e.mem.f32(step + 0xc);
        let four_tenths: f64 = e.global(DOUBLE_FOUR_TENTHS);
        push = ((distance as f64 * step_value as f64) * four_tenths + push as f64) as f32;
    }
    let mut impulse = [0.0f32; 4];
    store4(e, frame + 0x90, position);
    if push < 0.0 {
        e.vcall(motion, 0x28, &args![frame + 0xc0]);
        let arm = sub4(load4(e, frame + 0x40), load4(e, owner + 0x140));
        let moment = sub4(
            mul4(shuffle_201(normal), shuffle_120(arm)),
            mul4(shuffle_120(normal), shuffle_201(arm)),
        );
        store4(e, frame + 0xa0, moment);
        e.call(
            VECTOR_SET_MUL3,
            &args![frame + 0xb0, frame + 0xc0, frame + 0xa0],
        );
        let along = dot3(load4(e, frame + 0xb0), load4(e, frame + 0xa0));
        let inverse_mass = e.mem.f32(owner + 0x1ac);
        let denominator = (along as f64 + inverse_mass as f64) as f32;
        let mut strength = (push as f64 / denominator as f64) as f32;
        let strength_limit = (-(e.mem.f32(me + 0x6c) as f64) * e.mem.f32(step + 8) as f64) as f32;
        if strength_limit > strength {
            strength = strength_limit;
        }
        impulse = scaled(normal, strength);
    }
    let step_scaled = x87_quiet(e.mem.f32(step + 8));
    let velocity_vector = load4(e, velocity);
    let product = mul4(mul4([step_scaled; 4], velocity_vector), normal);
    let mut along_velocity = product[2] + (product[1] + product[0]);
    if closing < 0.0 {
        along_velocity = (along_velocity as f64 - closing as f64) as f32;
    }
    let negative_epsilon: f32 = e.global(NEGATIVE_EPSILON);
    if along_velocity < negative_epsilon {
        along_velocity = (along_velocity as f64 * e.mem.f32(me + 0x70) as f64) as f32;
        impulse = add4(scaled(normal, along_velocity), impulse);
    }
    store4(e, frame + 0x80, impulse);
    let mut listener_index = e.mem.i32(me + 0x84).wrapping_sub(1);
    while listener_index >= 0 {
        let listeners = e.mem.u32(me + 0x80);
        let listener = e.mem.u32(listeners.wrapping_add(4 * listener_index as u32));
        if listener != 0 {
            e.vcall(listener, 0x14, &args![this, frame + 0x40, frame + 0x80]);
        }
        listener_index -= 1;
    }
    let impulse = load4(e, frame + 0x80);
    if has_out {
        let record = *cursor;
        e.mem.set_u32(record + 0x20, owner);
        copy_bytes(e, record + 0x10, frame + 0x90, 16);
        store4(e, record, impulse);
        *cursor = record.wrapping_add(SURFACE_RECORD_SIZE);
    } else {
        e.call(ENTITY_ACTIVATE, &args![owner]);
        e.vcall(motion, 0x50, &args![frame + 0x80, frame + 0x90]);
    }
}

// Translated from 00cad440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls slot 4 of every listener, last to first, with `(proxy, first, second)`.
pub fn fn_00cad440(e: &mut Engine, this: Ptr<HkpCharacterProxy>, first: Ptr, second: Ptr) {
    let mut index = e
        .get(this, HkpCharacterProxy::m_listenersSize)
        .wrapping_sub(1);
    while index >= 0 {
        let listener = listener_at(e, this, index);
        e.vcall(listener, 4, &args![this, first, second]);
        index -= 1;
    }
}

// Translated from 00cad480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls slot 8 of every listener, last to first, with `(proxy, argument)`.
pub fn fn_00cad480(e: &mut Engine, this: Ptr<HkpCharacterProxy>, argument: Ptr) {
    let mut index = e
        .get(this, HkpCharacterProxy::m_listenersSize)
        .wrapping_sub(1);
    while index >= 0 {
        let listener = listener_at(e, this, index);
        e.vcall(listener, 8, &args![this, argument]);
        index -= 1;
    }
}

// Translated from 00cad4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls slot 0xC of every listener, last to first, with `(proxy, argument)`.
pub fn fn_00cad4c0(e: &mut Engine, this: Ptr<HkpCharacterProxy>, argument: Ptr) {
    let mut index = e
        .get(this, HkpCharacterProxy::m_listenersSize)
        .wrapping_sub(1);
    while index >= 0 {
        let listener = listener_at(e, this, index);
        e.vcall(listener, 0xc, &args![this, argument]);
        index -= 1;
    }
}

/// The listener at `index` of the proxy's listener list.
fn listener_at(e: &Engine, this: Ptr<HkpCharacterProxy>, index: i32) -> u32 {
    let data = e.get(this, HkpCharacterProxy::m_listeners).addr();
    e.mem.u32(data.wrapping_add(4 * index as u32))
}

// Translated from 00cad500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the proxy up from an `hkpCharacterProxyCinfo` (the constructor's body; the map has no
/// name for it): takes a reference on the cinfo's shape phantom and drops the old one, copies
/// the settings (`m_maxSlopeCosine` is the cosine of the cinfo's slope angle), normalises `m_up`
/// (all four lanes by the refined `RSQRTSS` scale), moves a phantom that is already in a world
/// to the cinfo's position, forgets every listened body and phantom, empties the manifold and
/// gives the phantom the proxy property (key `0x1300`) if it lacks it.
pub fn fn_00cad500(e: &mut Engine, this: Ptr<HkpCharacterProxy>, cinfo: Ptr) {
    let me = this.addr();
    let source = cinfo.addr();
    let new_phantom = e.mem.u32(source + 0x48);
    e.call(ADD_REFERENCE, &args![new_phantom]);
    let old_phantom = e.mem.u32(me + 0x30);
    if old_phantom != 0 {
        e.call(REMOVE_REFERENCE, &args![old_phantom]);
    }
    copy_bytes(e, me + 0x10, source + 0x10, 16);
    copy_quiet(e, me + 0x34, source + 0x20);
    copy_quiet(e, me + 0x38, source + 0x24);
    copy_quiet(e, me + 0x5c, source + 0x28);
    copy_quiet(e, me + 0x50, source + 0x40);
    copy_quiet(e, me + 0x54, source + 0x44);
    copy_quiet(e, me + 0x58, source + 0x4c);
    let phantom = e.mem.u32(source + 0x48);
    e.mem.set_u32(me + 0x30, phantom);
    copy_quiet(e, me + 0x60, source + 0x50);
    let user_planes = e.mem.u32(source + 0x54);
    e.mem.set_u32(me + 0x64, user_planes);
    copy_quiet(e, me + 0x68, source + 0x58);
    copy_quiet(e, me + 0x6c, source + 0x5c);
    copy_quiet(e, me + 0x70, source + 0x60);
    let slope = x87_quiet(e.mem.f32(source + 0x64));
    let cosine = e.call(COS, &args![slope as f64]).f64() as f32;
    e.mem.set_f32(me + 0xa4, cosine);
    copy_quiet(e, me + 0xa8, source + 0x68);
    let iterations = e.mem.u32(source + 0x6c);
    e.mem.set_u32(me + 0xac, iterations);
    let refresh = e.mem.u8(source + 0x70);
    e.mem.set_u8(me + 0xb0, refresh);
    let up = load4(e, source + 0x30);
    store4(e, me + 0x40, up);
    let up = normalized(e, up);
    store4(e, me + 0x40, up);
    let in_world = e.mem.u32(phantom + 8);
    if in_world != 0 {
        hkp_character_proxy_set_position(e, this, cinfo);
    }
    store4(e, me + 0x20, [0.0; 4]);
    let mut index = 0;
    while e.mem.i32(me + 0x90) > index {
        let bodies = e.mem.u32(me + 0x8c);
        let body = e.mem.u32(bodies.wrapping_add(4 * index as u32));
        e.call(ENTITY_REMOVE_LISTENER, &args![body, me + 8]);
        index += 1;
    }
    e.mem.set_u32(me + 0x90, 0);
    let mut index = 0;
    while e.mem.i32(me + 0x9c) > index {
        let phantoms = e.mem.u32(me + 0x98);
        let other = e.mem.u32(phantoms.wrapping_add(4 * index as u32));
        e.call(PHANTOM_REMOVE_LISTENER, &args![other, me + 0xc]);
        index += 1;
    }
    e.mem.set_u32(me + 0x9c, 0);
    e.mem.set_u32(me + 0x78, 0);
    let phantom = e.mem.u32(me + 0x30);
    let property_count = e.mem.i32(phantom + 0x7c);
    let properties = e.mem.u32(phantom + 0x78);
    let has_property = (0..property_count)
        .any(|i| e.mem.u32(properties.wrapping_add(16 * i as u32)) == PROXY_PROPERTY_KEY);
    if !has_property {
        e.call(
            WORLD_OBJECT_ADD_PROPERTY,
            &args![phantom, PROXY_PROPERTY_KEY, me, 0u32],
        );
    }
}

// Translated from 00cad6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes (the counterpart of `addCharacterProxyListener`; the map has no name for it) `listener` from the
/// listener list by moving the last entry into its place. An absent listener has index -1: the
/// size still drops and the entry before the data is overwritten, as the code does.
pub fn fn_00cad6f0(e: &mut Engine, this: Ptr<HkpCharacterProxy>, listener: Ptr) {
    let array = this.addr() + 0x80;
    let index = find_pointer(e, array, listener.addr());
    remove_pointer_at(e, array, index);
}

// Translated from 00cad740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `entityRemovedCallback` (Xbox PDB vslot 8 of `hkpEntityListener`; vtable `010c8488` slot 8), `this` being the
/// entity-listener base at proxy + 8: stops listening to `entity` (`00c9c670`), drops every
/// manifold point whose owner is it and removes it from the body list.
pub fn hkp_character_proxy_entity_removed_callback(e: &mut Engine, this: Ptr, entity: Ptr) {
    let listener = if this.addr() != 8 { this.addr() } else { 0 };
    e.call(ENTITY_REMOVE_LISTENER, &args![entity, listener]);
    let proxy = this.addr().wrapping_sub(8);
    forget_owner(e, proxy, 1, entity.addr(), 0x8c);
}

// Translated from 00cad820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `phantomRemovedCallback` (Xbox PDB vslot 8 of `hkpPhantomListener`; vtable `010c8470`
/// slot 8), `this` being the phantom-listener base at proxy + 0xC: the same for a phantom.
pub fn hkp_character_proxy_phantom_removed_callback(e: &mut Engine, this: Ptr, phantom: Ptr) {
    let listener = if this.addr() != 0xc { this.addr() } else { 0 };
    e.call(PHANTOM_REMOVE_LISTENER, &args![phantom, listener]);
    let proxy = this.addr().wrapping_sub(0xc);
    forget_owner(e, proxy, 2, phantom.addr(), 0x98);
}

/// Shared tail of the two removal callbacks: drops (last to first, moving the last entry into
/// the hole) the manifold points whose collidable has type `kind` and belongs to `who`, then
/// removes `who` from the pointer list at `list` (proxy + 0x8C bodies or + 0x98 phantoms).
fn forget_owner(e: &mut Engine, proxy: u32, kind: u8, who: u32, list: u32) {
    let mut index = e.mem.i32(proxy + 0x78).wrapping_sub(1);
    while index >= 0 {
        let data = e.mem.u32(proxy + 0x74);
        let entry = data.wrapping_add((index as u32).wrapping_mul(ROOT_CD_POINT_SIZE));
        let collidable = e.mem.u32(entry + 0x48);
        let owner = collidable_owner(e, collidable, kind);
        if owner == who {
            let size = e.mem.i32(proxy + 0x78).wrapping_sub(1);
            e.mem.set_i32(proxy + 0x78, size);
            if size != index {
                let last = e
                    .mem
                    .u32(proxy + 0x74)
                    .wrapping_add((size as u32).wrapping_mul(ROOT_CD_POINT_SIZE));
                copy_points(e, entry, last, ROOT_CD_POINT_SIZE);
            }
        }
        index -= 1;
    }
    let found = find_pointer(e, proxy + list, who);
    remove_pointer_at(e, proxy + list, found);
}

// Translated from 00cad900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterProxy::calcContentStatistics` (Xbox PDB, vtable slot 8): reports to the
/// statistics collector (slot 0x14, `(3, name, data, usedBytes, allocatedBytes)`) the
/// manifold, the body list, the phantom list and the listener list, each only when the
/// array owns its storage or the collector asks for all (`byte +0x10`). The second argument is
/// never read.
pub fn hkp_character_proxy_calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    collector: Ptr,
    _unused_1: u32,
) {
    let me = this.addr();
    let blocks = [
        (0x74u32, ROOT_CD_POINT_SIZE, NAME_MANIFOLD),
        (0x8c, 4, NAME_OVERLAPS),
        (0x98, 4, NAME_MANIFOLD),
        (0x80, 4, NAME_LISTENERS),
    ];
    for (array, element_size, name) in blocks {
        let capacity = e.mem.u32(me + array + 8);
        if capacity & NO_STORAGE != 0 && e.mem.u8(collector.addr() + 0x10) == 0 {
            continue;
        }
        let data = e.mem.u32(me + array);
        let allocated = e
            .call(
                GET_ALLOCATED_SIZE,
                &args![data, (capacity & CAPACITY_MASK).wrapping_mul(element_size)],
            )
            .u32();
        let data = e.mem.u32(me + array);
        let used = e.mem.u32(me + array + 4).wrapping_mul(element_size);
        e.vcall(
            collector.addr(),
            0x14,
            &args![3u32, name, data, used, allocated],
        );
    }
}

// Translated from 00cada30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterProxy::addCharacterProxyListener` (Xbox PDB): appends `listener` to the
/// listener list, growing it first when full.
pub fn hkp_character_proxy_add_character_proxy_listener(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    listener: Ptr,
) {
    push_pointer(e, this.addr() + 0x80, listener.addr());
}

// Translated from 00cada70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor body of `hkpCharacterProxy` (the map has no name for it; it writes the
/// vtable `010c84a4` whose RTTI is `.?AVhkpCharacterProxy@@`): reference count 1, the three vtables, empty arrays, then
/// [`fn_00cad500`] with the cinfo.
pub fn fn_00cada70(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    cinfo: Ptr,
) -> Ptr<HkpCharacterProxy> {
    let me = this.addr();
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, BASE_ENTITY_LISTENER_VTABLE);
    e.mem.set_u32(me + 0xc, BASE_PHANTOM_LISTENER_VTABLE);
    e.mem.set_u32(me, PROXY_VTABLE);
    e.mem.set_u32(me + 8, ENTITY_LISTENER_VTABLE);
    e.mem.set_u32(me + 0xc, PHANTOM_LISTENER_VTABLE);
    e.mem.set_u32(me + 0x30, 0);
    e.mem.set_u32(me + 0x74, 0);
    e.mem.set_u32(me + 0x78, 0);
    e.mem.set_u32(me + 0x7c, NO_STORAGE);
    e.mem.set_u32(me + 0x88, NO_STORAGE);
    e.mem.set_u32(me + 0x80, 0);
    e.mem.set_u32(me + 0x84, 0);
    e.mem.set_u32(me + 0x94, NO_STORAGE);
    e.mem.set_u32(me + 0x8c, 0);
    e.mem.set_u32(me + 0x90, 0);
    e.mem.set_u32(me + 0xa0, NO_STORAGE);
    e.mem.set_u32(me + 0x98, 0);
    e.mem.set_u32(me + 0x9c, 0);
    fn_00cad500(e, this, cinfo);
    this
}

// Translated from 00cadb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `hkpCharacterProxy` (the map has no name for it): stops listening to
/// every body and phantom, removes the proxy property from the shape phantom and drops the
/// reference on it (no null check), frees the four arrays and leaves the base-class vtables.
pub fn fn_00cadb00(e: &mut Engine, this: Ptr<HkpCharacterProxy>) {
    let me = this.addr();
    e.mem.set_u32(me, PROXY_VTABLE);
    e.mem.set_u32(me + 8, ENTITY_LISTENER_VTABLE);
    e.mem.set_u32(me + 0xc, PHANTOM_LISTENER_VTABLE);
    let mut index = 0;
    while e.mem.i32(me + 0x90) > index {
        let bodies = e.mem.u32(me + 0x8c);
        let body = e.mem.u32(bodies.wrapping_add(4 * index as u32));
        e.call(ENTITY_REMOVE_LISTENER, &args![body, me + 8]);
        index += 1;
    }
    e.mem.set_u32(me + 0x90, 0);
    let mut index = 0;
    while e.mem.i32(me + 0x9c) > index {
        let phantoms = e.mem.u32(me + 0x98);
        let other = e.mem.u32(phantoms.wrapping_add(4 * index as u32));
        e.call(PHANTOM_REMOVE_LISTENER, &args![other, me + 0xc]);
        index += 1;
    }
    let phantom = e.mem.u32(me + 0x30);
    e.mem.set_u32(me + 0x9c, 0);
    e.with_stack(8, |e, removed| {
        e.call(
            WORLD_OBJECT_REMOVE_PROPERTY,
            &args![phantom, removed, PROXY_PROPERTY_KEY],
        );
    });
    e.call(REMOVE_REFERENCE, &args![phantom]);
    free_hk_array(e, me + 0x98, 4);
    free_hk_array(e, me + 0x8c, 4);
    free_hk_array(e, me + 0x80, 4);
    free_hk_array(e, me + 0x74, ROOT_CD_POINT_SIZE);
    e.mem.set_u32(me + 0xc, BASE_PHANTOM_LISTENER_VTABLE);
    e.mem.set_u32(me, REFERENCED_OBJECT_VTABLE);
    e.mem.set_u32(me + 8, BASE_ENTITY_LISTENER_VTABLE);
}

// Translated from 00cadca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a solver plane into the plane array when it faces the displacement (a helper the
/// compiler inlined as a register-argument function: `EDI` = the array of 0x40-byte planes,
/// `EAX` = the plane's index, stack `(limit, vector)`, plain `RET`; the engine passes the
/// registers first, in that order). The plane `P` qualifies when `0.01 < P . vector` and
/// `P . vector < limit`; it is appended (growing the array if full) with its position moved
/// by `-(P . vector) * vector` and re-normalised. Returns whether a plane was added.
pub fn fn_00cadca0(e: &mut Engine, array: Ptr, index: u32, limit: f32, vector: Ptr) -> bool {
    let array = array.addr();
    let plane_offset = index.wrapping_shl(6);
    let data = e.mem.u32(array);
    let plane = load4(e, data.wrapping_add(plane_offset));
    let direction = load4(e, vector.addr());
    let along = dot3(plane, direction);
    let hundredth: f32 = e.global(ONE_HUNDREDTH);
    if along <= hundredth || along.is_nan() {
        return false;
    }
    if limit <= along || limit.is_nan() {
        return false;
    }
    reserve_more_if_full(e, array, 0x40);
    let data = e.mem.u32(array);
    let size = e.mem.u32(array + 4);
    let target = data.wrapping_add(size.wrapping_shl(6));
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    let source = data.wrapping_add(plane_offset);
    copy_bytes(e, target, source, 32);
    for offset in [0x20u32, 0x24, 0x28, 0x2c] {
        copy_quiet(e, target + offset, source + offset);
    }
    let tail = e.mem.u32(source + 0x30);
    e.mem.set_u32(target + 0x30, tail);
    let moved = add4(scaled(direction, -along), load4(e, target));
    let result = normalized(e, moved);
    store4(e, target, result);
    true
}

// Translated from 00cade20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterProxy::integrateImplementation` (Xbox PDB): moves the character through the
/// step time in up to `maxCastIterations` rounds. Each round casts the shape phantom along
/// `oldDisplacement`, updates the manifold from the hits (vtable slot 0xC), builds a solver
/// plane for every manifold point (vtable slot 0x10 and `00cadca0`), runs the simplex solver
/// (`00d24360`), applies the surface interactions and, when the solved displacement differs
/// from the old one, casts again and keeps the first hit that is not already a manifold
/// point; the step time left (`t`) shrinks by the part of the step the round used. `out`
/// (nullable) receives the surface interaction records (`limit` of them); without it the
/// phantom is moved through its vtable instead of `setPosition`.
///
/// The frame mirrors the locals of the function: position `0x30`, time left `0x14`,
/// iteration `0x48`, plane array `{data 0x18, size 0x1c, capacity 0x20, slab 0x24}`, cast
/// target `0x60`, tolerance `0x74`, solver output `0x90` (velocity `0xA0`, time used `0xB0`,
/// array data `0xB4`), flag `0xC0`, solver input `0xD0..`, the second slab `0x10`
/// (capacity word `0x128`). The profiling time stamps use the host clock (`RDTSC` stand-in).
#[allow(clippy::too_many_arguments)]
pub fn hkp_character_proxy_integrate_implementation(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    step: Ptr,
    velocity: Ptr,
    cast_collector: Ptr,
    start_collector: Ptr,
    out: Ptr,
    limit: i32,
) {
    e.with_stack(0x140, |e, frame| {
        integrate_rounds(
            e,
            this,
            step,
            velocity,
            cast_collector,
            start_collector,
            out,
            limit,
            frame.addr(),
        );
    });
}

/// Body of [`hkp_character_proxy_integrate_implementation`] over its local frame `f`.
#[allow(clippy::too_many_arguments, clippy::neg_cmp_op_on_partial_ord)]
fn integrate_rounds(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    step: Ptr,
    velocity: Ptr,
    cast_collector: Ptr,
    start_collector: Ptr,
    out: Ptr,
    limit: i32,
    f: u32,
) {
    let me = this.addr();
    let c1 = cast_collector.addr();
    let c2 = start_collector.addr();
    profile_record_wide(e);
    let has_out = out.addr() != 0;
    let position = hkp_character_proxy_get_position(e, this);
    copy_bytes(e, f + 0x30, position.addr(), 16);
    let mut time_left = x87_quiet(e.mem.f32(step.addr() + 8));
    e.mem.set_u32(f + 0x48, 0);
    let tolerance_sum = (e.mem.f32(me + 0x5c) as f64 + e.mem.f32(me + 0x58) as f64) as f32;
    e.mem.set_f32(f + 0x74, tolerance_sum);
    copy_quiet(e, f + 0x70, ONE_HUNDREDTH);
    let mut iteration = 0i32;
    loop {
        let epsilon: f32 = e.global(EPSILON);
        if !(epsilon < time_left) {
            break;
        }
        if iteration >= e.get(this, HkpCharacterProxy::m_maxCastIterations) {
            break;
        }
        profile_record(e, LABEL_INITIAL_CAST);
        let old = load4(e, me + 0x20);
        let current = load4(e, f + 0x30);
        store4(e, f + 0x60, add4(old, current));
        e.vcall(c1, 8, &args![]);
        e.vcall(c2, 8, &args![]);
        let phantom = e.mem.u32(me + 0x30);
        if has_out {
            let motion = e.vcall(phantom, 0x10, &args![]).u32();
            copy_bytes(e, motion + 0x30, f + 0x30, 16);
            hkp_character_proxy_linear_cast(
                e,
                this,
                iteration != 0,
                Ptr::new(f + 0x60),
                cast_collector,
                start_collector,
            );
        } else {
            e.vcall(phantom, 0x38, &args![f + 0x30, f + 0x60, c1, c2]);
        }
        if e.mem.i32(c1 + 0x14) > 0 {
            e.call(SORT_HITS, &args![c1]);
            let hits = e.mem.u32(c1 + 0x10);
            let count = e.mem.i32(c1 + 0x14);
            fn_00cac890(e, this, Ptr::new(hits), count, Ptr::new(me + 0x20));
        }
        profile_record(e, LABEL_UPDATE_MANIFOLD);
        e.vcall(me, 0xc, &args![c2, c1, has_out]);
        // The plane array: one plane per manifold point, plus room for the user planes.
        let manifold_size = e.get(this, HkpCharacterProxy::m_manifoldSize);
        let user_planes = e.get(this, HkpCharacterProxy::m_userPlanes);
        let plane_capacity = manifold_size.wrapping_add(user_planes).wrapping_add(10);
        e.mem.set_u32(f + 0x18, 0);
        e.mem.set_u32(f + 0x1c, 0);
        e.mem.set_u32(f + 0x20, NO_STORAGE);
        let planes_slab = stack_allocate(e, (plane_capacity as u32).wrapping_shl(6));
        let manifold_size = e.get(this, HkpCharacterProxy::m_manifoldSize);
        e.mem.set_u32(f + 0x18, planes_slab);
        e.mem
            .set_u32(f + 0x20, (plane_capacity as u32) | NO_STORAGE);
        e.mem.set_u32(f + 0x24, planes_slab);
        e.mem.set_i32(f + 0x1c, manifold_size);
        if manifold_size > 0 {
            let mut plane_offset = 0u32;
            let mut hit_offset = 0u32;
            let mut index = 0i32;
            loop {
                let step_time = x87_quiet(e.mem.f32(step.addr() + 8));
                let planes = e.mem.u32(f + 0x18);
                let remaining = (step_time as f64 - time_left as f64) as f32;
                let hit = e.mem.u32(me + 0x74).wrapping_add(hit_offset);
                e.vcall(
                    me,
                    0x10,
                    &args![hit, planes.wrapping_add(plane_offset), remaining],
                );
                let slope = x87_quiet(e.mem.f32(me + 0xa4));
                fn_00cadca0(
                    e,
                    Ptr::new(f + 0x18),
                    index as u32,
                    slope,
                    Ptr::new(me + 0x40),
                );
                plane_offset = plane_offset.wrapping_add(0x40);
                hit_offset = hit_offset.wrapping_add(ROOT_CD_POINT_SIZE);
                index += 1;
                if index >= e.mem.i32(me + 0x78) {
                    break;
                }
            }
        }
        let plane_count = e.mem.i32(f + 0x1c);
        let free = ((e.mem.u32(f + 0x20) & CAPACITY_MASK) as i32).wrapping_sub(plane_count);
        let user_planes = e.get(this, HkpCharacterProxy::m_userPlanes);
        if free < user_planes {
            let needed = plane_count.wrapping_add(user_planes);
            let router = memory_router(e);
            let allocator = e.mem.u32(router + 0x10);
            let capacity = (e.mem.u32(f + 0x20) & CAPACITY_MASK) as i32;
            if capacity < needed {
                let doubled = capacity.wrapping_add(capacity);
                let count = if needed < doubled { doubled } else { needed };
                e.call(ARRAY_RESERVE, &args![allocator, f + 0x18, count, 0x40u32]);
            }
        }
        profile_record(e, LABEL_SOLVE);
        // The solver input.
        let second_count = e
            .get(this, HkpCharacterProxy::m_userPlanes)
            .wrapping_add(e.mem.i32(f + 0x1c));
        let epsilon: f32 = e.global(EPSILON);
        store4(e, f + 0xf0, [epsilon; 4]);
        store4(e, f + 0x100, [0.0, 1.0, 0.0, 0.0]);
        store4(e, f + 0xc0, [0.0; 4]);
        store4(e, f + 0xd0, [0.0; 4]);
        let second_slab = stack_allocate(e, (second_count as u32).wrapping_shl(4));
        e.mem.set_f32(f + 0x110, x87_quiet(time_left));
        let travelled = load4(e, me + 0x10);
        store4(e, f + 0xe0, travelled);
        let speed_squared = dot3(travelled, travelled);
        e.mem.set_f32(f + 0x88, speed_squared);
        let plane_count = e.mem.u32(f + 0x1c);
        let planes_data = e.mem.u32(f + 0x18);
        e.mem.set_u32(f + 0x11c, plane_count);
        e.mem.set_u32(f + 0x128, (second_count as u32) | NO_STORAGE);
        e.mem.set_u32(f + 0x10, second_slab);
        e.mem.set_u32(f + 0x118, planes_data);
        let mut factor = 0.0f32;
        if 0.0 != speed_squared {
            let scale = normalizing_scale(e, speed_squared);
            let half: f64 = e.global(DOUBLE_HALF);
            let keep_distance = e.mem.f32(me + 0x58);
            factor = ((scale as f64 * keep_distance as f64) * half) as f32;
        }
        e.mem.set_f32(f + 0x114, factor);
        store4(e, f + 0xd0, [0.0; 4]);
        let up = load4(e, me + 0x40);
        store4(e, f + 0x100, up);
        let maximum_speed = e.mem.f32(me + 0x68);
        store4(e, f + 0xf0, [maximum_speed; 4]);
        fn_00cad440(e, this, Ptr::new(me + 0x74), Ptr::new(f + 0xd0));
        e.mem.set_u32(f + 0xb4, second_slab);
        e.call(SIMPLEX_SOLVE, &args![f + 0xd0, f + 0x90]);
        profile_record(e, LABEL_APPLY_SURFACE);
        hkp_character_proxy_apply_surface_interactions(e, this, step, velocity, out, limit);
        profile_record(e, LABEL_CAST_MOVE);
        let old = load4(e, me + 0x20);
        let solved = load4(e, f + 0x90);
        let converged = lanes_within_tolerance(e, old, solved);
        e.mem.set_u32(f + 0xc0, converged as u32);
        let mut use_solved_step = converged;
        if !converged {
            let current = load4(e, f + 0x30);
            store4(e, f + 0x60, add4(solved, current));
            e.vcall(c1, 8, &args![]);
            let phantom = e.mem.u32(me + 0x30);
            if has_out {
                let motion = e.vcall(phantom, 0x10, &args![]).u32();
                copy_bytes(e, motion + 0x30, f + 0x30, 16);
                hkp_character_proxy_linear_cast(
                    e,
                    this,
                    true,
                    Ptr::new(f + 0x60),
                    cast_collector,
                    Ptr::new(0),
                );
            } else {
                e.vcall(phantom, 0x38, &args![f + 0x30, f + 0x60, c1, 0u32]);
            }
            if e.mem.i32(c1 + 0x14) <= 0 {
                use_solved_step = true;
            } else {
                e.call(SORT_HITS, &args![c1]);
                let hits = e.mem.u32(c1 + 0x10);
                let count = e.mem.i32(c1 + 0x14);
                fn_00cac890(e, this, Ptr::new(hits), count, Ptr::new(f + 0x90));
                drop_known_hits(e, this, c1, f);
                if e.mem.i32(c1 + 0x14) > 0 {
                    adopt_first_hit(e, this, c1, has_out);
                }
                if e.mem.u32(f + 0xc0) != 0 {
                    use_solved_step = true;
                }
            }
        }
        if use_solved_step {
            let solved = load4(e, f + 0x90);
            let current = load4(e, f + 0x30);
            let used = e.mem.f32(f + 0xb0);
            time_left = (time_left as f64 - used as f64) as f32;
            store4(e, f + 0x30, add4(solved, current));
        } else {
            e.mem.set_f64(f + 0xc0, time_left as f64);
            let used = fn_00cacc50(
                e,
                this,
                Ptr::new(f + 0x90),
                cast_collector,
                Ptr::new(f + 0x60),
                Ptr::new(f + 0x30),
            );
            time_left = (e.mem.f64(f + 0xc0) - used as f64) as f32;
        }
        let solved = load4(e, f + 0x90);
        store4(e, me + 0x20, solved);
        // Release the second slab, then the plane array.
        let router = memory_router(e);
        e.mem.set_u32(router, second_slab);
        let router = memory_router(e);
        let allocator = e.mem.u32(router + 0x10);
        let second_capacity = e.mem.u32(f + 0x128);
        if second_capacity & NO_STORAGE == 0 {
            let bytes = (second_capacity & CAPACITY_MASK) << 4;
            e.vcall(allocator, 0x10, &args![second_slab, bytes]);
        }
        let slab = e.mem.u32(f + 0x24);
        if slab == e.mem.u32(f + 0x18) {
            e.mem.set_u32(f + 0x1c, 0);
        }
        let router = memory_router(e);
        e.mem.set_u32(router, slab);
        let router = memory_router(e);
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(f + 0x20);
        e.mem.set_u32(f + 0x1c, 0);
        if capacity & NO_STORAGE == 0 {
            let data = e.mem.u32(f + 0x18);
            let bytes = (capacity & CAPACITY_MASK) << 6;
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        iteration += 1;
        e.mem.set_i32(f + 0x48, iteration);
        e.mem.set_u32(f + 0x18, 0);
        e.mem.set_u32(f + 0x20, NO_STORAGE);
    }
    copy_bytes(e, me + 0x10, f + 0xa0, 16);
    let phantom = e.mem.u32(me + 0x30);
    if has_out {
        let motion = e.vcall(phantom, 0x10, &args![]).u32();
        copy_bytes(e, motion + 0x30, f + 0x30, 16);
    } else {
        let tolerance = x87_quiet(e.mem.f32(f + 0x74));
        e.call(
            SHAPE_PHANTOM_SET_POSITION,
            &args![phantom, f + 0x30, tolerance],
        );
    }
    profile_record(e, LABEL_INTEGRATE_END);
}

/// Drops from the cast collector's hits (last hit first moved down over the first) every
/// hit that [`fn_00cacf10`] finds already in the manifold; when none is left the flag at
/// `f + 0xC0` becomes 1.
fn drop_known_hits(e: &mut Engine, this: Ptr<HkpCharacterProxy>, collector: u32, f: u32) {
    let hits = e.mem.u32(collector + 0x10);
    let mut index = fn_00cacf10(e, this, Ptr::new(hits));
    while index != -1 {
        let size = e.mem.i32(collector + 0x14).wrapping_sub(1);
        e.mem.set_i32(collector + 0x14, size);
        let bytes = (size as u32).wrapping_mul(ROOT_CD_POINT_SIZE);
        let data = e.mem.u32(collector + 0x10);
        if (bytes as i32) > 0 {
            copy_points(e, data, data.wrapping_add(ROOT_CD_POINT_SIZE), bytes);
        }
        if e.mem.i32(collector + 0x14) <= 0 {
            e.mem.set_u32(f + 0xc0, 1);
            return;
        }
        let data = e.mem.u32(collector + 0x10);
        index = fn_00cacf10(e, this, Ptr::new(data));
    }
}

/// Takes the first remaining hit of the cast collector into the manifold: lets the listeners
/// know (`00cad480`), appends a copy, and (unless the proxy refreshes the manifold in
/// `checkSupport`) starts listening to the entity or phantom it belongs to, once.
fn adopt_first_hit(e: &mut Engine, this: Ptr<HkpCharacterProxy>, collector: u32, has_out: bool) {
    let me = this.addr();
    let hit = e.mem.u32(collector + 0x10);
    fn_00cad480(e, this, Ptr::new(hit));
    reserve_more_if_full(e, me + 0x74, ROOT_CD_POINT_SIZE);
    let size = e.mem.u32(me + 0x78);
    let data = e.mem.u32(me + 0x74);
    let slot = size.wrapping_mul(ROOT_CD_POINT_SIZE).wrapping_add(data);
    if slot != 0 {
        copy_bytes(e, slot, hit, ROOT_CD_POINT_SIZE);
    }
    let size = e.mem.u32(me + 0x78);
    e.mem.set_u32(me + 0x78, size.wrapping_add(1));
    if e.mem.u8(me + 0xb0) != 0 {
        return;
    }
    let collidable = e.mem.u32(hit + 0x48);
    let body = collidable_owner(e, collidable, 1);
    if body != 0 && find_pointer(e, me + 0x8c, body) == -1 {
        if !has_out {
            e.call(ENTITY_ADD_LISTENER, &args![body, me + 8]);
        }
        push_pointer(e, me + 0x8c, body);
    }
    let collidable = e.mem.u32(hit + 0x48);
    let phantom = collidable_owner(e, collidable, 2);
    if phantom != 0 && find_pointer(e, me + 0x98, phantom) == -1 {
        if !has_out {
            e.call(PHANTOM_ADD_LISTENER, &args![phantom, me + 0xc]);
        }
        push_pointer(e, me + 0x98, phantom);
    }
}

// ---------------------------------------------------------------------------
// Local `hkpAllCdPointCollector`s and the entry points that use them.
// ---------------------------------------------------------------------------

/// Sets up an `hkpAllCdPointCollector` local at `base` the way the compiler inlines its
/// constructor: vtable `0104da28`, the far distance (+4), data at the inline storage at
/// `inline` (+0x10), no hits (+0x14) and room for eight (+0x18, inline storage).
fn construct_collector(e: &mut Engine, base: u32, inline: u32) {
    e.mem.set_u32(base, ALL_CD_POINT_COLLECTOR_VTABLE);
    copy_quiet(e, base + 4, FAR_DISTANCE);
    e.mem.set_u32(base + 0x10, inline);
    e.mem.set_u32(base + 0x14, 0);
    e.mem.set_u32(base + 0x18, INLINE_EIGHT);
}

/// The inlined destructor of a local collector: the hits are dropped and heap storage (if the
/// capacity word owns any) is freed through the allocator's slot 0x10. With `reset` the array
/// is left without storage and the vtable of the base class is written (the collector that
/// is destroyed first in `integrateWithCollectors`).
fn destruct_collector(e: &mut Engine, base: u32, reset: bool) {
    e.mem.set_u32(base, ALL_CD_POINT_COLLECTOR_VTABLE);
    let router = memory_router(e);
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(base + 0x18);
    e.mem.set_u32(base + 0x14, 0);
    if capacity & NO_STORAGE == 0 {
        let data = e.mem.u32(base + 0x10);
        let bytes = (capacity & CAPACITY_MASK).wrapping_mul(ROOT_CD_POINT_SIZE);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    if reset {
        e.mem.set_u32(base + 0x10, 0);
        e.mem.set_u32(base + 0x18, NO_STORAGE);
        e.mem.set_u32(base, CD_POINT_COLLECTOR_VTABLE);
    }
}

// Translated from 00cae7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refreshes the manifold from the shape phantom's current surroundings (the map has no name
/// for it): clears `collector`, asks the phantom (vtable slot 0x40) for the points within
/// `keepContactTolerance + keepDistance` of its shape (the world's collision input with the
/// distance at +8 widened by that sum) and feeds them, with a fresh local collector, to
/// `updateManifold` (slot 0xC, `keep` false).
pub fn fn_00cae7f0(e: &mut Engine, this: Ptr<HkpCharacterProxy>, collector: Ptr) {
    let me = this.addr();
    let a = collector.addr();
    e.vcall(a, 8, &args![]);
    let tolerance = e.mem.f32(me + 0x5c) as f64;
    let phantom = e.mem.u32(me + 0x30);
    let world = e.mem.u32(phantom + 8);
    let input = e.mem.u32(world + 0x78);
    e.with_stack(0x420, |e, frame| {
        let f = frame.addr();
        for offset in [0u32, 4, 8, 0xc, 0x10, 0x14] {
            let word = e.mem.u32(input + offset);
            e.mem.set_u32(f + 0x10 + offset, word);
        }
        let widened =
            ((tolerance + e.mem.f32(me + 0x58) as f64) + e.mem.f32(f + 0x18) as f64) as f32;
        for offset in [0x20u32, 0x30, 0x40] {
            copy_bytes(e, f + 0x10 + offset, input + offset, 16);
        }
        e.mem.set_f32(f + 0x18, widened);
        let phantom = e.mem.u32(me + 0x30);
        e.vcall(phantom, 0x40, &args![a, f + 0x10]);
        construct_collector(e, f + 0x60, f + 0x80);
        e.vcall(me, 0xc, &args![a, f + 0x60, false]);
        destruct_collector(e, f + 0x60, false);
    });
}

// Translated from 00cafad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Integrates with two local collectors and no surface output (the map has no name for it):
/// `integrateImplementation(step, velocity, &castCollector, &startCollector, 0, 0)`.
pub fn fn_00cafad0(e: &mut Engine, this: Ptr<HkpCharacterProxy>, step: Ptr, velocity: Ptr) {
    e.with_stack(0x750, |e, frame| {
        let f = frame.addr();
        construct_collector(e, f + 0x3b0, f + 0x3d0);
        construct_collector(e, f + 0x10, f + 0x30);
        hkp_character_proxy_integrate_implementation(
            e,
            this,
            step,
            velocity,
            Ptr::new(f + 0x3b0),
            Ptr::new(f + 0x10),
            Ptr::new(0),
            0,
        );
        destruct_collector(e, f + 0x10, true);
        destruct_collector(e, f + 0x3b0, false);
    });
}

// Translated from 00cafbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterProxy::integrateWithCollectors` (Xbox PDB):
/// `integrateImplementation(step, velocity, castCollector, startCollector, 0, 0)`.
pub fn hkp_character_proxy_integrate_with_collectors(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    step: Ptr,
    velocity: Ptr,
    cast_collector: Ptr,
    start_collector: Ptr,
) {
    hkp_character_proxy_integrate_implementation(
        e,
        this,
        step,
        velocity,
        cast_collector,
        start_collector,
        Ptr::new(0),
        0,
    );
}

// Translated from 00cafbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Integrates with two local collectors and a surface output (the map has no name for it):
/// `integrateImplementation(step, velocity, &castCollector, &startCollector, out, limit)`.
pub fn fn_00cafbf0(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    step: Ptr,
    velocity: Ptr,
    out: Ptr,
    limit: i32,
) {
    e.with_stack(0x750, |e, frame| {
        let f = frame.addr();
        construct_collector(e, f + 0x3b0, f + 0x3d0);
        construct_collector(e, f + 0x10, f + 0x30);
        hkp_character_proxy_integrate_implementation(
            e,
            this,
            step,
            velocity,
            Ptr::new(f + 0x3b0),
            Ptr::new(f + 0x10),
            out,
            limit,
        );
        destruct_collector(e, f + 0x10, true);
        destruct_collector(e, f + 0x3b0, false);
    });
}

// Translated from 00cafd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterProxy::checkSupport` (Xbox PDB): `checkSupportWithCollector` with a local
/// collector.
pub fn hkp_character_proxy_check_support(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    direction: Ptr,
    info: Ptr,
) {
    e.with_stack(0x3a0, |e, frame| {
        let f = frame.addr();
        construct_collector(e, f, f + 0x20);
        hkp_character_proxy_check_support_with_collector(e, this, direction, info, Ptr::new(f));
        destruct_collector(e, f, false);
    });
}

// ---------------------------------------------------------------------------
// Support checks and the manifold update.
// ---------------------------------------------------------------------------

/// The start of `checkSupport`'s two variants: builds the plane array at `array`
/// (`{data, size, capacity word, slab}`): one plane per manifold point filled through the
/// vtable's `extractSurfaceConstraintInfo` (slot 0x10, time 0) and [`fn_00cadca0`], then grown
/// to hold the user planes.
fn fill_support_planes(e: &mut Engine, this: Ptr<HkpCharacterProxy>, array: u32) {
    let me = this.addr();
    let manifold_size = e.get(this, HkpCharacterProxy::m_manifoldSize);
    let user_planes = e.get(this, HkpCharacterProxy::m_userPlanes);
    let plane_capacity = manifold_size.wrapping_add(user_planes).wrapping_add(10);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, NO_STORAGE);
    let slab = stack_allocate(e, (plane_capacity as u32).wrapping_shl(6));
    let manifold_size = e.get(this, HkpCharacterProxy::m_manifoldSize);
    e.mem.set_u32(array, slab);
    e.mem.set_u32(array + 0xc, slab);
    e.mem
        .set_u32(array + 8, (plane_capacity as u32) | NO_STORAGE);
    e.mem.set_i32(array + 4, manifold_size);
    if manifold_size > 0 {
        let mut plane_offset = 0u32;
        let mut hit_offset = 0u32;
        let mut index = 0i32;
        loop {
            let planes = e.mem.u32(array);
            let hit = e.mem.u32(me + 0x74).wrapping_add(hit_offset);
            e.vcall(
                me,
                0x10,
                &args![hit, planes.wrapping_add(plane_offset), 0.0f32],
            );
            let slope = x87_quiet(e.mem.f32(me + 0xa4));
            fn_00cadca0(e, Ptr::new(array), index as u32, slope, Ptr::new(me + 0x40));
            plane_offset = plane_offset.wrapping_add(0x40);
            hit_offset = hit_offset.wrapping_add(ROOT_CD_POINT_SIZE);
            index += 1;
            if index >= e.mem.i32(me + 0x78) {
                break;
            }
        }
    }
    let size = e.mem.i32(array + 4);
    let free = ((e.mem.u32(array + 8) & CAPACITY_MASK) as i32).wrapping_sub(size);
    let user_planes = e.get(this, HkpCharacterProxy::m_userPlanes);
    if free < user_planes {
        let needed = size.wrapping_add(user_planes);
        let router = memory_router(e);
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(array + 8) & CAPACITY_MASK) as i32;
        if capacity < needed {
            let doubled = capacity.wrapping_add(capacity);
            let count = if needed < doubled { doubled } else { needed };
            e.call(ARRAY_RESERVE, &args![allocator, array, count, 0x40u32]);
        }
    }
}

/// A second stack array of 16-byte elements sized `userPlanes + planes.size`, empty, at `array`.
fn make_vector_array(e: &mut Engine, this: Ptr<HkpCharacterProxy>, array: u32, planes: u32) {
    let user_planes = e.get(this, HkpCharacterProxy::m_userPlanes);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, NO_STORAGE);
    let count = user_planes.wrapping_add(e.mem.i32(planes + 4));
    let slab = stack_allocate(e, (count as u32).wrapping_shl(4));
    e.mem.set_u32(array + 8, (count as u32) | NO_STORAGE);
    e.mem.set_u32(array, slab);
    e.mem.set_u32(array + 0xc, slab);
}

/// Releases a stack array whose slab start is its own fourth word.
fn release_array_with_slab(e: &mut Engine, array: u32, element_shift: u32) {
    let slab = e.mem.u32(array + 0xc);
    release_stack_array(e, array, slab, element_shift);
}

// Translated from 00cae900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterProxy::checkSupportWithCollector` (Xbox PDB): asks whether the character is
/// still supported when it moves along `velocity`. Optionally (`m_refreshManifoldInCheckSupport`)
/// first refreshes the manifold from `collector` (`00cae7f0`); builds the planes, runs the
/// simplex solver with a 1/60 s step and fills `info` (0x34 bytes): status word at +0 (0 =
/// not supported, 1 or 2 = supported by the slope test), the averaged support normal (+0x10)
/// and surface velocity (+0x20) of the planes that oppose the motion (dot product below
/// -0.08) and 0.0 at +0x30.
pub fn hkp_character_proxy_check_support_with_collector(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    velocity: Ptr,
    info: Ptr,
    collector: Ptr,
) {
    e.with_stack(0x100, |e, frame| {
        support_check_body(
            e,
            this,
            velocity.addr(),
            info.addr(),
            collector,
            frame.addr(),
        );
    });
}

/// Body of [`hkp_character_proxy_check_support_with_collector`] over its local frame `f`:
/// plane array `f + 0x20`, array of saved surface velocities `f + 0x30`, solver flags array
/// `f + 0x40`, solver input `f + 0x50`, solver output `f + 0xb0` (velocity at `f + 0xc0`).
fn support_check_body(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    v: u32,
    s: u32,
    collector: Ptr,
    f: u32,
) {
    let me = this.addr();
    profile_record(e, LABEL_CHECK_SUPPORT);
    if e.mem.u8(me + 0xb0) != 0 {
        fn_00cae7f0(e, this, collector);
    }
    fill_support_planes(e, this, f + 0x20);
    make_vector_array(e, this, f + 0x40, f + 0x20);
    make_vector_array(e, this, f + 0x30, f + 0x20);
    copy_quiet(e, f + 0x90, SOLVER_TIME_STEP);
    copy_quiet(e, f + 0x94, SOLVER_TIME_STEP);
    let plane_data = e.mem.u32(f + 0x20);
    let plane_count = e.mem.u32(f + 0x24);
    store4(e, f + 0xa0, [0.0; 4]);
    store4(e, f + 0x50, [0.0; 4]);
    e.mem.set_u32(f + 0x9c, plane_count);
    let direction = load4(e, v);
    store4(e, f + 0x60, direction);
    let up = load4(e, me + 0x40);
    store4(e, f + 0x80, up);
    e.mem.set_u32(f + 0x98, plane_data);
    let maximum_speed = e.mem.f32(me + 0x68);
    store4(e, f + 0x70, [maximum_speed; 4]);
    let flags_data = e.mem.u32(f + 0x40);
    e.mem.set_u32(f + 0xd4, flags_data);
    fn_00cad440(e, this, Ptr::new(me + 0x74), Ptr::new(f + 0x50));
    let count = e.mem.i32(f + 0x9c);
    reserve_array(e, f + 0x30, count, 0x10);
    e.mem.set_i32(f + 0x34, count);
    reserve_array(e, f + 0x40, count, 0x10);
    e.mem.set_i32(f + 0x44, count);
    reserve_array(e, f + 0x20, count, 0x40);
    e.mem.set_i32(f + 0x24, count);
    if e.mem.i32(f + 0x9c) > 0 {
        let zero = load4(e, f + 0xa0);
        let planes = e.mem.u32(f + 0x20);
        let saved = e.mem.u32(f + 0x30);
        for i in 0..e.mem.u32(f + 0x9c) {
            let plane_vector = load4(e, planes.wrapping_add(i * 0x40 + 0x10));
            store4(e, saved.wrapping_add(i * 0x10), plane_vector);
            store4(e, planes.wrapping_add(i * 0x40 + 0x10), zero);
        }
    }
    e.call(SIMPLEX_SOLVE, &args![f + 0x50, f + 0xb0]);
    let zero = load4(e, f + 0xa0);
    let mut solved = load4(e, f + 0xc0);
    store4(e, s + 0x20, zero);
    store4(e, s + 0x10, zero);
    let direction = load4(e, v);
    if lanes_within_tolerance(e, solved, direction) {
        e.mem.set_u32(s, 0);
    } else {
        let tolerance: f32 = e.global(TOLERANCE);
        let length_squared = dot3(solved, solved);
        if tolerance > length_squared {
            e.mem.set_u32(s, 2);
        } else {
            let scale = normalizing_scale(e, length_squared);
            solved = scaled(solved, scale);
            store4(e, f + 0xc0, solved);
            let along = dot3(direction, solved);
            let slope_cosine = e.mem.f32(me + 0xa4) as f64;
            let rest = (1.0f64 - along as f64 * along as f64) as f32;
            let status = if (rest as f64) < slope_cosine * slope_cosine {
                1
            } else {
                2
            };
            e.mem.set_u32(s, status);
        }
        let count = e.mem.i32(f + 0x9c);
        if count <= 0 {
            e.mem.set_u32(s, 0);
        } else {
            let limit: f32 = e.global(SUPPORT_LIMIT);
            let planes = e.mem.u32(f + 0x20);
            let saved = e.mem.u32(f + 0x30);
            let flags = e.mem.u32(f + 0x40);
            let mut contributing = 0i32;
            for i in 0..count as u32 {
                if e.mem.u8(flags.wrapping_add(i * 0x10)) == 0 {
                    continue;
                }
                let plane = load4(e, planes.wrapping_add(i * 0x40));
                let along = dot3(load4(e, v), plane);
                if limit > along {
                    let sum = add4(load4(e, s + 0x10), plane);
                    store4(e, s + 0x10, sum);
                    contributing += 1;
                    let sum = add4(load4(e, saved.wrapping_add(i * 0x10)), load4(e, s + 0x20));
                    store4(e, s + 0x20, sum);
                }
            }
            if contributing <= 0 {
                e.mem.set_u32(s, 0);
            } else {
                let inverse = (1.0f64 / contributing as f64) as f32;
                let normal = normalized(e, load4(e, s + 0x10));
                store4(e, s + 0x10, normal);
                let averaged = scaled(load4(e, s + 0x20), inverse);
                store4(e, s + 0x20, averaged);
            }
        }
    }
    e.mem.set_f32(s + 0x30, 0.0);
    profile_record(e, LABEL_END);
    release_array_with_slab(e, f + 0x30, 4);
    release_array_with_slab(e, f + 0x40, 4);
    release_array_with_slab(e, f + 0x20, 6);
}

// Translated from 00caefd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A variant of `checkSupportWithCollector` without the manifold refresh and the saved
/// velocities (the map has no name for it): the solver is run with the character's velocity
/// plus `velocity`. Byte 0 of `info` says the part of the solved velocity along `velocity`
/// is below 0.99 (cleared again when no plane opposes the motion), byte 1 that the rest of
/// the solved velocity (once the part along `velocity` is taken off) has a squared length
/// above 0.01; +0x10 gets the normalised sum of the opposing plane normals and +0x20 the
/// average of the planes' second vectors.
pub fn fn_00caefd0(e: &mut Engine, this: Ptr<HkpCharacterProxy>, velocity: Ptr, info: Ptr) {
    e.with_stack(0xe0, |e, frame| {
        support_variant_body(e, this, velocity.addr(), info.addr(), frame.addr());
    });
}

/// Body of [`fn_00caefd0`]: plane array `f + 0x1c`, vector array `f + 0x2c`, solver input
/// `f + 0x40`, output `f + 0x90` (velocity at `f + 0xa0`).
fn support_variant_body(e: &mut Engine, this: Ptr<HkpCharacterProxy>, v: u32, s: u32, f: u32) {
    let me = this.addr();
    profile_record(e, LABEL_CHECK_SUPPORT);
    fill_support_planes(e, this, f + 0x1c);
    make_vector_array(e, this, f + 0x2c, f + 0x1c);
    let vector_count = e
        .get(this, HkpCharacterProxy::m_userPlanes)
        .wrapping_add(e.mem.i32(f + 0x20));
    reserve_array(e, f + 0x2c, vector_count, 0x10);
    let flags_data = e.mem.u32(f + 0x2c);
    e.mem.set_i32(f + 0x30, vector_count);
    copy_quiet(e, f + 0x80, SOLVER_TIME_STEP);
    copy_quiet(e, f + 0x84, SOLVER_TIME_STEP);
    store4(e, f + 0xc0, [0.0; 4]);
    store4(e, f + 0x40, [0.0; 4]);
    let planes_data = e.mem.u32(f + 0x1c);
    let manifold_size = e.mem.u32(me + 0x78);
    e.mem.set_u32(f + 0x88, planes_data);
    let combined = add4(load4(e, me + 0x10), load4(e, v));
    store4(e, f + 0x50, combined);
    e.mem.set_u32(f + 0x8c, manifold_size);
    let up = load4(e, me + 0x40);
    store4(e, f + 0x70, up);
    let maximum_speed = e.mem.f32(me + 0x68);
    store4(e, f + 0x60, [maximum_speed; 4]);
    e.mem.set_u32(f + 0xb4, flags_data);
    fn_00cad440(e, this, Ptr::new(me + 0x74), Ptr::new(f + 0x40));
    e.call(SIMPLEX_SOLVE, &args![f + 0x40, f + 0x90]);
    let relative = sub4(load4(e, f + 0xa0), load4(e, me + 0x10));
    let direction = load4(e, v);
    let along = dot3(direction, relative);
    store4(e, f + 0xa0, relative);
    let near_one: f64 = e.global(DOUBLE_NEAR_ONE);
    e.mem.set_u8(s, ((along as f64) < near_one) as u8);
    let negated = -along;
    let hundredth: f32 = e.global(ONE_HUNDREDTH);
    let residual = add4(scaled(direction, negated), relative);
    let residual_squared = dot3(residual, residual);
    e.mem.set_u8(s + 1, (hundredth < residual_squared) as u8);
    store4(e, s + 0x20, [0.0; 4]);
    store4(e, s + 0x10, [0.0; 4]);
    let count = e.mem.i32(f + 0x8c);
    if count <= 0 {
        e.mem.set_u8(s, 0);
    } else {
        let limit: f32 = e.global(SUPPORT_LIMIT);
        let planes = e.mem.u32(f + 0x1c);
        let flags = e.mem.u32(f + 0x2c);
        let mut contributing = 0i32;
        for i in 0..count as u32 {
            if e.mem.u8(flags.wrapping_add(i * 0x10)) == 0 {
                continue;
            }
            let plane = load4(e, planes.wrapping_add(i * 0x40));
            let along = dot3(plane, load4(e, v));
            if limit > along {
                let sum = add4(plane, load4(e, s + 0x10));
                store4(e, s + 0x10, sum);
                let second = add4(
                    load4(e, planes.wrapping_add(i * 0x40 + 0x10)),
                    load4(e, s + 0x20),
                );
                store4(e, s + 0x20, second);
                contributing += 1;
            }
        }
        if contributing <= 0 {
            e.mem.set_u8(s, 0);
        } else {
            let inverse = (1.0f64 / contributing as f64) as f32;
            let normal = normalized(e, load4(e, s + 0x10));
            store4(e, s + 0x10, normal);
            let averaged = scaled(load4(e, s + 0x20), inverse);
            store4(e, s + 0x20, averaged);
        }
    }
    profile_record(e, LABEL_END);
    release_array_with_slab(e, f + 0x2c, 4);
    release_array_with_slab(e, f + 0x1c, 6);
}

// Translated from 00caf4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCharacterProxy::updateManifold` (Xbox PDB vslot 0xC): merges the contact points found
/// around the character into the manifold. `start_points` (first collector) are copied to a
/// temporary array, the closest to each manifold point (by [`fn_00cafd90`], limit 1.1) replaces
/// it (the listeners hear `00cad4c0` and `00cad480` when the other body changed), a manifold
/// point with no match is dropped, and a temporary point at the smallest distance that is not
/// in the manifold is added; then the first of `cast_points` is added if new, points closer
/// than 0.1 to a later one are pruned, and the body and phantom lists are rebuilt from the
/// owners of the manifold (listening to new ones unless `keep`).
pub fn hkp_character_proxy_update_manifold(
    e: &mut Engine,
    this: Ptr<HkpCharacterProxy>,
    start_points: Ptr,
    cast_points: Ptr,
    keep: bool,
) {
    let me = this.addr();
    let first = start_points.addr();
    let second = cast_points.addr();
    if !keep {
        let mut index = 0;
        while e.mem.i32(me + 0x90) > index {
            let bodies = e.mem.u32(me + 0x8c);
            let body = e.mem.u32(bodies.wrapping_add(4 * index as u32));
            e.call(ENTITY_REMOVE_LISTENER, &args![body, me + 8]);
            index += 1;
        }
        let mut index = 0;
        while e.mem.i32(me + 0x9c) > index {
            let phantoms = e.mem.u32(me + 0x98);
            let other = e.mem.u32(phantoms.wrapping_add(4 * index as u32));
            e.call(PHANTOM_REMOVE_LISTENER, &args![other, me + 0xc]);
            index += 1;
        }
    }
    let mut nearest_overall = x87_quiet(e.global::<f32>(FAR_DISTANCE));
    e.mem.set_u32(me + 0x90, 0);
    e.mem.set_u32(me + 0x9c, 0);
    let first_count = e.mem.i32(first + 0x14);
    let slab = stack_allocate(e, (first_count as u32).wrapping_mul(ROOT_CD_POINT_SIZE));
    let mut temp_count = e.mem.i32(first + 0x14);
    let temp_capacity = (first_count as u32) | NO_STORAGE;
    // Copy the start points to the temporary array, tracking the smallest distance.
    let mut offset = 0u32;
    for _ in 0..temp_count.max(0) {
        let source = e.mem.u32(first + 0x10).wrapping_add(offset);
        e.call(
            ROOT_CD_POINT_ASSIGN,
            &args![slab.wrapping_add(offset), source],
        );
        let distance = e.mem.f32(slab.wrapping_add(offset).wrapping_add(0x1c));
        if nearest_overall > distance {
            nearest_overall = x87_quiet(distance);
        }
        offset = offset.wrapping_add(ROOT_CD_POINT_SIZE);
    }
    // Match each manifold point (last to first) with its closest temporary point.
    let mut entry_index = e.mem.i32(me + 0x78).wrapping_sub(1);
    let mut entry_offset = (entry_index as u32).wrapping_mul(ROOT_CD_POINT_SIZE);
    while entry_index >= 0 {
        let entry = e.mem.u32(me + 0x74).wrapping_add(entry_offset);
        let mut nearest = x87_quiet(e.global::<f32>(NEAREST_LIMIT));
        let mut best = -1i32;
        let mut candidate = slab;
        let mut j = 0i32;
        while j < temp_count {
            let distance = x87_quiet(fn_00cafd90(e, this, Ptr::new(candidate), Ptr::new(entry)));
            if nearest > distance {
                nearest = distance;
                best = j;
            }
            candidate = candidate.wrapping_add(ROOT_CD_POINT_SIZE);
            j += 1;
        }
        if best >= 0 {
            let matched = slab.wrapping_add((best as u32).wrapping_mul(ROOT_CD_POINT_SIZE));
            if e.mem.u32(matched + 0x48) != e.mem.u32(entry + 0x48) {
                fn_00cad4c0(e, this, Ptr::new(entry));
                fn_00cad480(e, this, Ptr::new(matched));
            }
            e.call(ROOT_CD_POINT_ASSIGN, &args![entry, matched]);
            temp_count = temp_count.wrapping_sub(1);
            if temp_count != best {
                let last = slab.wrapping_add((temp_count as u32).wrapping_mul(ROOT_CD_POINT_SIZE));
                copy_points(e, matched, last, ROOT_CD_POINT_SIZE);
            }
        } else {
            let entry = e.mem.u32(me + 0x74).wrapping_add(entry_offset);
            fn_00cad4c0(e, this, Ptr::new(entry));
            let size = e.mem.i32(me + 0x78).wrapping_sub(1);
            e.mem.set_i32(me + 0x78, size);
            if size != entry_index {
                let data = e.mem.u32(me + 0x74);
                let last = data.wrapping_add((size as u32).wrapping_mul(ROOT_CD_POINT_SIZE));
                copy_points(e, data.wrapping_add(entry_offset), last, ROOT_CD_POINT_SIZE);
            }
        }
        entry_index -= 1;
        entry_offset = entry_offset.wrapping_sub(ROOT_CD_POINT_SIZE);
    }
    // Add the remaining temporary points at the smallest distance.
    if temp_count > 0 {
        let mut point = slab;
        for _ in 0..temp_count {
            let distance = e.mem.f32(point + 0x1c);
            if distance == nearest_overall {
                let found = fn_00cacf10(e, this, Ptr::new(point));
                if found >= 0 {
                    let entry = e
                        .mem
                        .u32(me + 0x74)
                        .wrapping_add((found as u32).wrapping_mul(ROOT_CD_POINT_SIZE));
                    if e.mem.u32(point + 0x48) != e.mem.u32(entry + 0x48) {
                        fn_00cad4c0(e, this, Ptr::new(entry));
                        fn_00cad480(e, this, Ptr::new(point));
                    }
                    e.call(ROOT_CD_POINT_ASSIGN, &args![entry, point]);
                } else {
                    fn_00cad480(e, this, Ptr::new(point));
                    append_point(e, me, point);
                }
            }
            point = point.wrapping_add(ROOT_CD_POINT_SIZE);
        }
    }
    // The first point of the cast, if it is new.
    if e.mem.i32(second + 0x14) > 0 {
        let hit = e.mem.u32(second + 0x10);
        if fn_00cacf10(e, this, Ptr::new(hit)) == -1 {
            fn_00cad480(e, this, Ptr::new(hit));
            append_point(e, me, hit);
        }
    }
    prune_close_points(e, this);
    rebuild_owner_lists(e, this, keep);
    let router = memory_router(e);
    e.mem.set_u32(router, slab);
    let router = memory_router(e);
    let allocator = e.mem.u32(router + 0x10);
    if temp_capacity & NO_STORAGE == 0 {
        let bytes = (temp_capacity & CAPACITY_MASK).wrapping_mul(ROOT_CD_POINT_SIZE);
        e.vcall(allocator, 0x10, &args![slab, bytes]);
    }
}

/// Appends a copy of the contact point `point` to the manifold (growing it first); the copy
/// is skipped when the new slot's address is null, the size grows regardless.
fn append_point(e: &mut Engine, proxy: u32, point: u32) {
    reserve_more_if_full(e, proxy + 0x74, ROOT_CD_POINT_SIZE);
    let size = e.mem.u32(proxy + 0x78);
    let data = e.mem.u32(proxy + 0x74);
    let slot = size.wrapping_mul(ROOT_CD_POINT_SIZE).wrapping_add(data);
    if slot != 0 {
        copy_bytes(e, slot, point, ROOT_CD_POINT_SIZE);
    }
    e.mem.set_u32(proxy + 0x78, size.wrapping_add(1));
}

/// Drops, last to first, every manifold point that is closer than 0.1 (by [`fn_00cafd90`]) to
/// an earlier one, telling the listeners (`00cad4c0`) and moving the last point into its place.
fn prune_close_points(e: &mut Engine, this: Ptr<HkpCharacterProxy>) {
    let me = this.addr();
    let mut i = e.mem.i32(me + 0x78).wrapping_sub(1);
    let mut offset = (i as u32).wrapping_mul(ROOT_CD_POINT_SIZE);
    while i > 0 {
        let next = i - 1;
        let mut j = next;
        let mut other_offset = offset.wrapping_sub(ROOT_CD_POINT_SIZE);
        while j >= 0 {
            let data = e.mem.u32(me + 0x74);
            let distance = fn_00cafd90(
                e,
                this,
                Ptr::new(data.wrapping_add(offset)),
                Ptr::new(data.wrapping_add(other_offset)),
            );
            let tenth: f64 = e.global(DOUBLE_TENTH);
            if (distance as f64) < tenth {
                let data = e.mem.u32(me + 0x74);
                fn_00cad4c0(e, this, Ptr::new(data.wrapping_add(offset)));
                let size = e.mem.i32(me + 0x78).wrapping_sub(1);
                e.mem.set_i32(me + 0x78, size);
                if size != i {
                    let data = e.mem.u32(me + 0x74);
                    let last = data.wrapping_add((size as u32).wrapping_mul(ROOT_CD_POINT_SIZE));
                    copy_points(e, data.wrapping_add(offset), last, ROOT_CD_POINT_SIZE);
                }
                break;
            }
            j -= 1;
            other_offset = other_offset.wrapping_sub(ROOT_CD_POINT_SIZE);
        }
        i = next;
        offset = offset.wrapping_sub(ROOT_CD_POINT_SIZE);
    }
}

/// Rebuilds the body and phantom lists from the owners of the manifold's collidables (type 1
/// and 2), listening to each new one (`00c9c5b0` / `00d07da0`) unless `keep`.
fn rebuild_owner_lists(e: &mut Engine, this: Ptr<HkpCharacterProxy>, keep: bool) {
    let me = this.addr();
    let mut index = 0i32;
    let mut offset = 0u32;
    if e.mem.i32(me + 0x78) <= 0 {
        return;
    }
    loop {
        let entry = e.mem.u32(me + 0x74).wrapping_add(offset);
        let collidable = e.mem.u32(entry + 0x48);
        let body = collidable_owner(e, collidable, 1);
        if body != 0 && find_pointer(e, me + 0x8c, body) == -1 {
            if !keep {
                e.call(ENTITY_ADD_LISTENER, &args![body, me + 8]);
            }
            push_pointer(e, me + 0x8c, body);
        }
        let entry = e.mem.u32(me + 0x74).wrapping_add(offset);
        let collidable = e.mem.u32(entry + 0x48);
        let phantom = collidable_owner(e, collidable, 2);
        if phantom != 0 && find_pointer(e, me + 0x98, phantom) == -1 {
            if !keep {
                e.call(PHANTOM_ADD_LISTENER, &args![phantom, me + 0xc]);
            }
            push_pointer(e, me + 0x98, phantom);
        }
        index += 1;
        offset = offset.wrapping_add(ROOT_CD_POINT_SIZE);
        if index >= e.mem.i32(me + 0x78) {
            break;
        }
    }
}

type Proxy = Ptr<HkpCharacterProxy>;

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca_c890, fn_00cac890(Proxy, Ptr, i32, Ptr)),
        entry!(0x00ca_c910, hkp_character_proxy_get_position(Proxy) -> Ptr),
        entry!(0x00ca_c920, hkp_character_proxy_set_position(Proxy, Ptr)),
        entry!(0x00ca_c950, hkp_character_proxy_get_linear_velocity(Proxy) -> Ptr),
        entry!(0x00ca_c960, fn_00cac960(Proxy, Ptr)),
        entry!(0x00ca_c980, fn_00cac980(Proxy) -> Ptr),
        entry!(0x00ca_c9a0, fn_00cac9a0(Proxy, Ptr)),
        entry!(
            0x00ca_caa0,
            hkp_character_proxy_linear_cast(Proxy, bool, Ptr, Ptr, Ptr)
        ),
        entry!(0x00ca_cc50, fn_00cacc50(Proxy, Ptr, Ptr, Ptr, Ptr) -> f32),
        entry!(
            0x00ca_cd60,
            hkp_character_proxy_extract_surface_constraint_info(Proxy, Ptr, Ptr, f32)
        ),
        entry!(0x00ca_cf10, fn_00cacf10(Proxy, Ptr) -> i32),
        entry!(
            0x00ca_cf80,
            hkp_character_proxy_apply_surface_interactions(Proxy, Ptr, Ptr, Ptr, i32)
        ),
        entry!(0x00ca_d440, fn_00cad440(Proxy, Ptr, Ptr)),
        entry!(0x00ca_d480, fn_00cad480(Proxy, Ptr)),
        entry!(0x00ca_d4c0, fn_00cad4c0(Proxy, Ptr)),
        entry!(0x00ca_d500, fn_00cad500(Proxy, Ptr)),
        entry!(0x00ca_d6f0, fn_00cad6f0(Proxy, Ptr)),
        entry!(
            0x00ca_d740,
            hkp_character_proxy_entity_removed_callback(Ptr, Ptr)
        ),
        entry!(
            0x00ca_d820,
            hkp_character_proxy_phantom_removed_callback(Ptr, Ptr)
        ),
        entry!(
            0x00ca_d900,
            hkp_character_proxy_calc_content_statistics(Proxy, Ptr, u32)
        ),
        entry!(
            0x00ca_da30,
            hkp_character_proxy_add_character_proxy_listener(Proxy, Ptr)
        ),
        entry!(0x00ca_da70, fn_00cada70(Proxy, Ptr) -> Proxy),
        entry!(0x00ca_db00, fn_00cadb00(Proxy)),
        entry!(0x00ca_dca0, fn_00cadca0(Ptr, u32, f32, Ptr) -> bool),
        entry!(
            0x00ca_de20,
            hkp_character_proxy_integrate_implementation(Proxy, Ptr, Ptr, Ptr, Ptr, Ptr, i32)
        ),
        entry!(0x00ca_e7f0, fn_00cae7f0(Proxy, Ptr)),
        entry!(
            0x00ca_e900,
            hkp_character_proxy_check_support_with_collector(Proxy, Ptr, Ptr, Ptr)
        ),
        entry!(0x00ca_efd0, fn_00caefd0(Proxy, Ptr, Ptr)),
        entry!(
            0x00ca_f4d0,
            hkp_character_proxy_update_manifold(Proxy, Ptr, Ptr, bool)
        ),
        entry!(0x00ca_fad0, fn_00cafad0(Proxy, Ptr, Ptr)),
        entry!(
            0x00ca_fbd0,
            hkp_character_proxy_integrate_with_collectors(Proxy, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00ca_fbf0, fn_00cafbf0(Proxy, Ptr, Ptr, Ptr, i32)),
        entry!(
            0x00ca_fd00,
            hkp_character_proxy_check_support(Proxy, Ptr, Ptr)
        ),
        entry!(0x00ca_fd90, fn_00cafd90(Proxy, Ptr, Ptr) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The allocator's slot 0x10 (free), a fake address.
    const FREE: u32 = 0x0900_0000;
    const ALLOCATOR_VTABLE: u32 = 0x0200_0000;

    struct Rig {
        e: Engine,
        proxy: Proxy,
        stream: u32,
        router: u32,
    }

    /// An engine without the executable: the constants the translations read, the memory
    /// router (stack allocator at word 0, heap allocator at +0x10) and the monitor stream.
    fn rig() -> Rig {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.map(0x0100_0000, 0x0030_0000);
        for (addr, value) in [
            (FLOAT_ZERO, 0.0f32),
            (FLOAT_ONE, 1.0),
            (FLOAT_THREE, 3.0),
            (FLOAT_HALF, 0.5),
            (FLOAT_PI, std::f32::consts::PI),
            (ONE_HUNDREDTH, 0.01),
            (EPSILON, 1.192_092_9e-7),
            (NEGATIVE_EPSILON, -1.192_092_9e-7),
            (TOLERANCE, 0.001),
            (SUPPORT_LIMIT, -0.08),
            (SAME_POINT_DISTANCE, 0.1),
            (SOLVER_TIME_STEP, 1.0 / 60.0),
            (NEAREST_LIMIT, 1.1),
        ] {
            e.set_global(addr, value);
        }
        e.mem.set_u32(FAR_DISTANCE, 0x7f7f_ffee);
        for (addr, value) in [
            (DOUBLE_ZERO, 0.0f64),
            (DOUBLE_NINE_TENTHS, 0.9f32 as f64),
            (DOUBLE_FOUR_TENTHS, 0.4f32 as f64),
            (DOUBLE_HALF, 0.5),
            (DOUBLE_TENTH, 0.1f32 as f64),
            (DOUBLE_TEN, 10.0),
            (DOUBLE_NEAR_ONE, 0.99f32 as f64),
        ] {
            e.mem.set_f64(addr, value);
        }
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
            e.mem.set_f32(ONES_VECTOR + 4 * lane, 1.0);
        }
        let stack = e.mem.alloc(0x2_0000);
        let allocator = e.mem.alloc(8);
        vtable(&mut e, ALLOCATOR_VTABLE, &[(0x10, FREE)]);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        let router = e.mem.alloc(0x40);
        e.mem.set_u32(router, stack);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| router.into_ret());
        let stream = e.mem.alloc(0x400);
        e.mem.set_u32(stream + 4, stream + 0x40);
        e.mem.set_u32(stream + 0xc, stream + 0x3f0);
        e.mem.set_u32(MONITOR_SLOT, 5);
        e.register_double(TLS_GET_VALUE, move |_, _| stream.into_ret());
        noop(&mut e, &[FREE]);
        let proxy = e.new_object::<HkpCharacterProxy>();
        Rig {
            e,
            proxy,
            stream,
            router,
        }
    }

    /// Registers test doubles that do nothing and return 0.
    fn noop(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register_double(*addr, |_, _| 0u32.into_ret());
        }
    }

    /// Writes a vtable with the given `(byte offset, target)` slots.
    fn vtable(e: &mut Engine, at: u32, slots: &[(u32, u32)]) {
        let count = slots.iter().map(|(o, _)| o / 4 + 1).max().unwrap_or(1);
        let mut words = vec![0u32; count as usize];
        for (offset, target) in slots {
            words[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(at, &words);
    }

    /// An object of `size` bytes whose first word is the vtable `vt`.
    fn object(e: &mut Engine, size: u32, vt: u32) -> u32 {
        let o = e.mem.alloc(size);
        e.mem.set_u32(o, vt);
        o
    }

    /// The argument lists of every logged call to `addr`.
    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An `hkArray` of pointers at `at` holding `elems`, with owned storage for `capacity`.
    fn array(e: &mut Engine, at: u32, elems: &[u32], capacity: u32) -> u32 {
        let data = e.mem.alloc((capacity & CAPACITY_MASK).max(1) * 4);
        for (i, word) in elems.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *word);
        }
        e.mem.set_u32(at, data);
        e.mem.set_u32(at + 4, elems.len() as u32);
        e.mem.set_u32(at + 8, capacity);
        data
    }

    /// An owner (entity or phantom) block and its collidable of `kind` inside it.
    fn owner_block(e: &mut Engine, kind: u8) -> (u32, u32) {
        let owner = e.mem.alloc(0x200);
        let collidable = owner + 0x40;
        e.mem.set_u8(collidable + 0x10, 0xc0);
        e.mem.set_u8(collidable + 0x18, kind);
        (owner, collidable)
    }

    /// A contact point: `normal` and `distance` as the plane at +0x10, the collidable at +0x48.
    fn contact(e: &mut Engine, collidable: u32, normal: [f32; 3], distance: f32) -> u32 {
        let c = e.mem.alloc(ROOT_CD_POINT_SIZE);
        store4(e, c + 0x10, [normal[0], normal[1], normal[2], distance]);
        e.mem.set_u32(c + 0x48, collidable);
        c
    }

    /// Puts copies of the `entries` into the proxy's manifold (room for four).
    fn manifold(e: &mut Engine, proxy: Proxy, entries: &[u32]) {
        let me = proxy.addr();
        let data = e.mem.alloc(ROOT_CD_POINT_SIZE * 4);
        for (i, entry) in entries.iter().enumerate() {
            copy_bytes(
                e,
                data + ROOT_CD_POINT_SIZE * i as u32,
                *entry,
                ROOT_CD_POINT_SIZE,
            );
        }
        e.mem.set_u32(me + 0x74, data);
        e.mem.set_u32(me + 0x78, entries.len() as u32);
        e.mem.set_u32(me + 0x7c, 4);
    }

    /// A shape phantom: its motion state (translation at +0x30) and a world with a collision
    /// input block; the proxy uses it.
    fn phantom(e: &mut Engine, proxy: Proxy, vt: u32) -> u32 {
        let phantom = object(e, 0x400, vt);
        let motion_state = e.mem.alloc(0x100);
        let world = e.mem.alloc(0x200);
        let input = e.mem.alloc(0x100);
        e.mem.set_u32(phantom + 0x18, motion_state);
        e.mem.set_u32(phantom + 8, world);
        e.mem.set_u32(world + 0x78, input);
        e.mem.set_u32(proxy.addr() + 0x30, phantom);
        phantom
    }

    fn near(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-3
    }

    fn f64_arg(words: &[u32]) -> f64 {
        f64::from_bits(words[0] as u64 | (words[1] as u64) << 32)
    }

    // ----- the tests, in address order -----

    #[test]
    fn t_00cac890_reweights_the_hits_against_the_vector() {
        let Rig { mut e, proxy, .. } = rig();
        let hits = e.mem.alloc(2 * ROOT_CD_POINT_SIZE);
        for i in 0..2u32 {
            let hit = hits + ROOT_CD_POINT_SIZE * i;
            store4(&mut e, hit + 0x10, [0.0, 1.0, 0.0, 0.0]);
            e.mem.set_f32(hit + 0x1c, 2.0 + i as f32);
        }
        let vector = e.mem.alloc(16);
        store4(&mut e, vector, [0.0, -3.0, 0.0, 0.0]);
        e.call(0x00ca_c890, &args![proxy, hits, 2i32, vector]);
        assert_eq!(e.mem.f32(hits + 0xc), 2.0);
        assert_eq!(e.mem.f32(hits + 0x1c), 6.0);
        assert_eq!(e.mem.f32(hits + ROOT_CD_POINT_SIZE + 0xc), 3.0);
        assert_eq!(e.mem.f32(hits + ROOT_CD_POINT_SIZE + 0x1c), 9.0);
    }

    #[test]
    fn t_00cac910_get_position_is_the_motion_state_translation() {
        let Rig { mut e, proxy, .. } = rig();
        let phantom = phantom(&mut e, proxy, 0);
        let motion_state = e.mem.u32(phantom + 0x18);
        assert_eq!(
            e.call(0x00ca_c910, &args![proxy]).u32(),
            motion_state + 0x30
        );
    }

    #[test]
    fn t_00cac920_set_position_passes_the_tolerance_sum() {
        let Rig { mut e, proxy, .. } = rig();
        let phantom = phantom(&mut e, proxy, 0);
        e.mem.set_f32(proxy.addr() + 0x5c, 0.25);
        e.mem.set_f32(proxy.addr() + 0x58, 0.5);
        noop(&mut e, &[SHAPE_PHANTOM_SET_POSITION]);
        let position = e.mem.alloc(16);
        e.call(0x00ca_c920, &args![proxy, position]);
        assert_eq!(
            calls(&e, SHAPE_PHANTOM_SET_POSITION),
            vec![vec![phantom, position, 0.75f32.to_bits()]]
        );
    }

    #[test]
    fn t_00cac950_get_linear_velocity_is_the_velocity_field() {
        let Rig { mut e, proxy, .. } = rig();
        assert_eq!(
            e.call(0x00ca_c950, &args![proxy]).u32(),
            proxy.addr() + 0x10
        );
    }

    #[test]
    fn t_00cac960_copies_the_velocity_in() {
        let Rig { mut e, proxy, .. } = rig();
        let velocity = e.mem.alloc(16);
        store4(&mut e, velocity, [1.0, 2.0, 3.0, 4.0]);
        e.call(0x00ca_c960, &args![proxy, velocity]);
        assert_eq!(load4(&e, proxy.addr() + 0x10), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn t_00cac980_returns_the_shape_phantom() {
        let Rig { mut e, proxy, .. } = rig();
        let phantom = phantom(&mut e, proxy, 0);
        assert_eq!(e.call(0x00ca_c980, &args![proxy]).u32(), phantom);
    }

    #[test]
    fn t_00cac9a0_fills_the_cinfo_from_the_proxy() {
        let Rig { mut e, proxy, .. } = rig();
        let me = proxy.addr();
        let phantom = phantom(&mut e, proxy, 0);
        let motion_state = e.mem.u32(phantom + 0x18);
        store4(&mut e, motion_state + 0x30, [1.0, 2.0, 3.0, 4.0]);
        store4(&mut e, me + 0x10, [5.0, 6.0, 7.0, 8.0]);
        store4(&mut e, me + 0x40, [0.0, 1.0, 0.0, 0.0]);
        for (offset, value) in [
            (0x34u32, 1.5f32),
            (0x38, 2.5),
            (0x5c, 3.5),
            (0x50, 4.5),
            (0x54, 5.5),
            (0x58, 6.5),
            (0x60, 7.5),
            (0x68, 8.5),
            (0x6c, 9.5),
            (0x70, 10.5),
            (0xa4, 0.5),
            (0xa8, 11.5),
        ] {
            e.mem.set_f32(me + offset, value);
        }
        e.mem.set_u32(me + 0x64, 9);
        e.mem.set_u32(me + 0xac, 12);
        e.mem.set_u8(me + 0xb0, 1);
        e.register_double(ACOS, |_, a| f64_arg(a).acos().into_ret());
        let cinfo = e.mem.alloc(0x80);
        e.call(0x00ca_c9a0, &args![proxy, cinfo]);
        assert_eq!(load4(&e, cinfo), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load4(&e, cinfo + 0x10), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(load4(&e, cinfo + 0x30), [0.0, 1.0, 0.0, 0.0]);
        for (offset, value) in [
            (0x20u32, 1.5f32),
            (0x24, 2.5),
            (0x28, 3.5),
            (0x40, 4.5),
            (0x44, 5.5),
            (0x4c, 6.5),
            (0x50, 7.5),
            (0x58, 8.5),
            (0x5c, 9.5),
            (0x60, 10.5),
            (0x68, 11.5),
        ] {
            assert_eq!(e.mem.f32(cinfo + offset), value, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(cinfo + 0x48), phantom);
        assert_eq!(e.mem.u32(cinfo + 0x54), 9);
        assert_eq!(e.mem.u32(cinfo + 0x6c), 12);
        assert_eq!(e.mem.u8(cinfo + 0x70), 1);
        assert_eq!(e.mem.f32(cinfo + 0x64), (0.5f64).acos() as f32);
        assert_eq!(calls(&e, ACOS).len(), 1);
        // A cosine of at least one in magnitude does not call `acos`.
        e.mem.set_f32(me + 0xa4, 2.0);
        e.call(0x00ca_c9a0, &args![proxy, cinfo]);
        assert_eq!(e.mem.f32(cinfo + 0x64), 0.0);
        e.mem.set_f32(me + 0xa4, -2.0);
        e.call(0x00ca_c9a0, &args![proxy, cinfo]);
        assert_eq!(e.mem.f32(cinfo + 0x64), std::f32::consts::PI);
        assert_eq!(calls(&e, ACOS).len(), 1);
    }

    #[test]
    fn t_00cacaa0_linear_cast_goes_to_the_world_or_to_the_agents() {
        let Rig { mut e, proxy, .. } = rig();
        let me = proxy.addr();
        let phantom_vt = 0x0200_0100;
        let kind_fn = 0x0900_0200;
        let cast_fn = 0x0900_0300;
        vtable(&mut e, phantom_vt, &[(0x14, kind_fn)]);
        let phantom = phantom(&mut e, proxy, phantom_vt);
        let world = e.mem.u32(phantom + 8);
        let input = e.mem.alloc(0x20);
        e.mem.set_u32(input + 0x10, 0.5f32.to_bits());
        e.mem.set_u32(input + 0x14, 2.0f32.to_bits());
        let c1 = e.mem.alloc(0x20);
        let c2 = e.mem.alloc(0x20);
        // With the world flag the world's own linear cast is called.
        noop(&mut e, &[WORLD_LINEAR_CAST]);
        e.call(0x00ca_caa0, &args![proxy, true, input, c1, c2]);
        assert_eq!(
            calls(&e, WORLD_LINEAR_CAST),
            vec![vec![world, phantom + 0x10, input, c1, c2]]
        );
        // Otherwise every agent is asked: a phantom of type 2 goes through its objects.
        store4(&mut e, me + 0x20, [3.0, 4.0, 0.0, 0.0]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(cast_fn, move |e, a| {
            log.borrow_mut().push((
                e.mem.f32(a[3] + 0x64),
                e.mem.u32(a[3] + 0x60),
                e.mem.u32(a[3] + 8),
            ));
            0u32.into_ret()
        });
        e.register_double(kind_fn, |_, _| 2u32.into_ret());
        let object_vt = 0x0200_0200;
        vtable(&mut e, object_vt, &[(0x14, cast_fn)]);
        let object_a = object(&mut e, 8, object_vt);
        let data = e.mem.alloc(16);
        e.mem.set_u32(data, object_a);
        e.mem.set_u32(data + 4, 0x1111);
        e.mem.set_u32(phantom + 0x160, data);
        e.mem.set_u32(phantom + 0x164, 1);
        e.call(0x00ca_caa0, &args![proxy, false, input, c1, c2]);
        assert_eq!(seen.borrow().len(), 1);
        assert_eq!(seen.borrow()[0], (5.0, 0.5f32.to_bits(), 2.0f32.to_bits()));
        let cast = calls(&e, cast_fn);
        assert_eq!(cast[0][0..3], [object_a, phantom + 0x10, 0x1111]);
        assert_eq!(cast[0][4..], [c1, c2]);
        // A phantom of type 1 goes through the dispatcher's table of linear-cast functions.
        e.register_double(kind_fn, |_, _| 1u32.into_ret());
        let shape_a = e.mem.alloc(0x20);
        e.mem.set_u32(shape_a + 0xc, 1);
        e.mem.set_u32(phantom + 0x10, shape_a);
        let shape_b = e.mem.alloc(0x20);
        e.mem.set_u32(shape_b + 0xc, 2);
        let other = e.mem.alloc(0x20);
        e.mem.set_u32(other, shape_b);
        let table = e.mem.alloc(0xa00);
        e.mem.set_u8(table + (1 + 13) * 32 + 2, 3);
        e.mem.set_u32(table + 0x9ac + 3 * 0x14, cast_fn);
        e.mem.set_u32(world + 0x80, table);
        e.mem.set_u32(data, other);
        e.call(0x00ca_caa0, &args![proxy, false, input, c1, c2]);
        let cast = calls(&e, cast_fn);
        assert_eq!(cast.len(), 2);
        assert_eq!(cast[1][0..2], [phantom + 0x10, other]);
        assert_eq!(cast[1][3..], [c1, c2]);
    }

    #[test]
    fn t_00cacc50_blends_the_result_toward_the_start() {
        let Rig { mut e, proxy, .. } = rig();
        e.mem.set_f32(proxy.addr() + 0x58, 0.25);
        let displacement = e.mem.alloc(0x30);
        store4(&mut e, displacement, [0.0, -2.0, 0.0, 0.0]);
        e.mem.set_f32(displacement + 0x20, 8.0);
        let hit = e.mem.alloc(0x30);
        store4(&mut e, hit + 0x10, [0.0, 1.0, 0.0, 0.0]);
        e.mem.set_f32(hit + 0xc, 0.75);
        let cast = e.mem.alloc(0x20);
        e.mem.set_u32(cast + 0x10, hit);
        let start = e.mem.alloc(16);
        store4(&mut e, start, [4.0; 4]);
        let result = e.mem.alloc(16);
        store4(&mut e, result, [2.0; 4]);
        let weight = e
            .call(
                0x00ca_cc50,
                &args![proxy, displacement, cast, start, result],
            )
            .f32();
        assert_eq!(weight, 5.0);
        assert_eq!(load4(&e, result), [3.25; 4]);
    }

    #[test]
    fn t_00cacd60_extracts_the_surface_constraint_info() {
        let Rig { mut e, proxy, .. } = rig();
        let me = proxy.addr();
        e.mem.set_f32(me + 0x58, 0.25);
        e.mem.set_f32(me + 0x38, 2.5);
        e.mem.set_f32(me + 0x34, 1.5);
        e.mem.set_f32(me + 0x50, 4.5);
        e.mem.set_f32(me + 0x54, 5.5);
        e.mem.set_f32(me + 0xa8, 0.5);
        // A movable body: the velocity comes from its motion.
        let (owner, collidable) = owner_block(&mut e, 1);
        e.mem.set_u8(owner + 0xe8, 1);
        store4(&mut e, owner + 0x140, [0.0, 2.0, 0.0, 0.0]);
        e.mem.set_f32(owner + 0x14c, 2.0);
        let hit = contact(&mut e, collidable, [0.0, 1.0, 0.0], 0.5);
        let info = e.mem.alloc(0x40);
        e.call(0x00ca_cd60, &args![proxy, hit, info, 0.5f32]);
        assert_eq!(load4(&e, info), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load4(&e, info + 0x10), [0.0, 4.875, 0.0, 4.4375]);
        assert_eq!(e.mem.f32(info + 0x20), 2.5);
        assert_eq!(e.mem.f32(info + 0x2c), 1.5);
        assert_eq!(e.mem.f32(info + 0x24), 4.5);
        assert_eq!(e.mem.f32(info + 0x28), 5.5);
        assert_eq!(e.mem.u32(info + 0x30), 0);
        // A keyframed body (type 5) asks the body for the velocity at the contact.
        let (fixed, collidable) = owner_block(&mut e, 1);
        e.mem.set_u8(fixed + 0xe8, 5);
        e.register_double(BODY_POINT_VELOCITY, |e, a| {
            store4(e, a[2], [1.0, 2.0, 3.0, 0.0]);
            0u32.into_ret()
        });
        let hit = contact(&mut e, collidable, [0.0, 1.0, 0.0], 0.5);
        e.call(0x00ca_cd60, &args![proxy, hit, info, 0.5f32]);
        assert_eq!(
            calls(&e, BODY_POINT_VELOCITY),
            vec![vec![fixed, hit, info + 0x10]]
        );
        assert_eq!(load4(&e, info + 0x10), [1.0, 2.375, 3.0, 0.1875]);
        assert_eq!(e.mem.f32(info + 0xc), 0.0);
        assert_eq!(e.mem.u32(info + 0x30), 2);
    }

    #[test]
    fn t_00cacf10_finds_the_closest_manifold_point() {
        let Rig { mut e, proxy, .. } = rig();
        e.mem.set_f32(proxy.addr() + 0x60, 1.0);
        let (_, collidable) = owner_block(&mut e, 0);
        let hit = contact(&mut e, collidable, [0.0, 1.0, 0.0], 0.5);
        let far = contact(&mut e, collidable, [1.0, 0.0, 0.0], 0.5);
        let near_one = contact(&mut e, collidable, [0.0, 1.0, 0.0], 0.45);
        manifold(&mut e, proxy, &[far, near_one]);
        assert_eq!(e.call(0x00ca_cf10, &args![proxy, hit]).i32(), 1);
        manifold(&mut e, proxy, &[far]);
        assert_eq!(e.call(0x00ca_cf10, &args![proxy, hit]).i32(), -1);
    }

    #[test]
    fn t_00cacf80_applies_the_surface_interactions() {
        let Rig { mut e, proxy, .. } = rig();
        let me = proxy.addr();
        store4(&mut e, me + 0x10, [0.0, 1.0, 0.0, 0.0]);
        e.mem.set_f32(me + 0x70, 3.0);
        let (owner, collidable) = owner_block(&mut e, 1);
        e.mem.set_u8(owner + 0xe8, 1);
        let motion_vt = 0x0200_0300;
        let apply = 0x0900_0500;
        vtable(&mut e, motion_vt, &[(0x50, apply)]);
        e.mem.set_u32(owner + 0xe0, motion_vt);
        let entry = contact(&mut e, collidable, [0.0, 1.0, 0.0], 0.0);
        manifold(&mut e, proxy, &[entry]);
        let step = e.mem.alloc(0x10);
        e.mem.set_f32(step + 8, 0.5);
        e.mem.set_f32(step + 0xc, 0.25);
        let velocity = e.mem.alloc(16);
        store4(&mut e, velocity, [0.0, -4.0, 0.0, 0.0]);
        // With an output list the records are appended and the list is closed.
        let buffer = e.mem.alloc(4 * SURFACE_RECORD_SIZE);
        e.mem
            .set_u32(buffer + SURFACE_RECORD_SIZE + 0x20, 0xffff_ffff);
        let cursor = e.mem.alloc(8);
        e.mem.set_u32(cursor, buffer);
        e.call(0x00ca_cf80, &args![proxy, step, velocity, cursor, 4i32]);
        assert_eq!(load4(&e, buffer), [0.0, -3.0, 0.0, 0.0]);
        assert_eq!(load4(&e, buffer + 0x10), [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(buffer + 0x20), owner);
        assert_eq!(e.mem.u32(buffer + SURFACE_RECORD_SIZE + 0x20), 0);
        // Without one the owner is activated and the motion is pushed; the listeners hear of
        // the proxy property and of the impulse.
        let properties = e.mem.alloc(32);
        e.mem.set_u32(properties + 16, PROXY_PROPERTY_KEY);
        e.mem.set_u32(properties + 16 + 8, 0xcafe);
        e.mem.set_u32(owner + 0x78, properties);
        e.mem.set_u32(owner + 0x7c, 2);
        let listener_vt = 0x0200_0400;
        let found = 0x0900_0600;
        let altered = 0x0900_0700;
        vtable(&mut e, listener_vt, &[(0x10, found), (0x14, altered)]);
        let listener = object(&mut e, 8, listener_vt);
        array(&mut e, me + 0x80, &[listener], 2);
        let pushed = Rc::new(RefCell::new(Vec::new()));
        let log = pushed.clone();
        e.register_double(apply, move |e, a| {
            log.borrow_mut().push(e.mem.f32(a[1] + 4));
            0u32.into_ret()
        });
        noop(&mut e, &[found, altered, ENTITY_ACTIVATE]);
        e.call(0x00ca_cf80, &args![proxy, step, velocity, 0u32, 0i32]);
        assert_eq!(*pushed.borrow(), vec![-3.0]);
        assert_eq!(calls(&e, ENTITY_ACTIVATE), vec![vec![owner]]);
        let entry = e.mem.u32(me + 0x74);
        assert_eq!(calls(&e, found), vec![vec![listener, me, 0xcafe, entry]]);
        assert_eq!(calls(&e, altered).len(), 1);
        assert_eq!(calls(&e, altered)[0][0..2], [listener, me]);
    }

    /// Runs `address` over two listeners whose vtable slot `slot` is a recording double.
    fn listener_slot_test(address: u32, slot: u32, extra: &[u32]) {
        let Rig { mut e, proxy, .. } = rig();
        let target = 0x0900_0800;
        let listener_vt = 0x0200_0500;
        vtable(&mut e, listener_vt, &[(slot, target)]);
        noop(&mut e, &[target]);
        let first = object(&mut e, 8, listener_vt);
        let second = object(&mut e, 8, listener_vt);
        array(&mut e, proxy.addr() + 0x80, &[first, second], 2);
        let mut words = vec![proxy.addr()];
        words.extend_from_slice(extra);
        e.call(address, &words);
        // Last to first, each with the proxy and the arguments.
        let mut expected_first = vec![second, proxy.addr()];
        expected_first.extend_from_slice(extra);
        let mut expected_second = vec![first, proxy.addr()];
        expected_second.extend_from_slice(extra);
        assert_eq!(calls(&e, target), vec![expected_first, expected_second]);
    }

    #[test]
    fn t_00cad440_calls_slot_4_of_each_listener() {
        listener_slot_test(0x00ca_d440, 4, &[0x1000, 0x2000]);
    }

    #[test]
    fn t_00cad480_calls_slot_8_of_each_listener() {
        listener_slot_test(0x00ca_d480, 8, &[0x1000]);
    }

    #[test]
    fn t_00cad4c0_calls_slot_c_of_each_listener() {
        listener_slot_test(0x00ca_d4c0, 0xc, &[0x1000]);
    }

    /// A cinfo for `phantom` with distinct values in every field the proxy copies.
    fn cinfo_block(e: &mut Engine, phantom: u32) -> u32 {
        let cinfo = e.mem.alloc(0x80);
        store4(e, cinfo + 0x10, [1.0, 2.0, 3.0, 4.0]);
        store4(e, cinfo + 0x30, [0.0, 2.0, 0.0, 0.0]);
        for (offset, value) in [
            (0x20u32, 1.5f32),
            (0x24, 2.5),
            (0x28, 3.5),
            (0x40, 4.5),
            (0x44, 5.5),
            (0x4c, 6.5),
            (0x50, 7.5),
            (0x58, 8.5),
            (0x5c, 9.5),
            (0x60, 10.5),
            (0x64, 1.0),
            (0x68, 11.5),
        ] {
            e.mem.set_f32(cinfo + offset, value);
        }
        e.mem.set_u32(cinfo + 0x48, phantom);
        e.mem.set_u32(cinfo + 0x54, 9);
        e.mem.set_u32(cinfo + 0x6c, 12);
        e.mem.set_u8(cinfo + 0x70, 1);
        cinfo
    }

    /// The doubles for the callees of the constructor body.
    fn setup_callees(e: &mut Engine) {
        noop(
            e,
            &[
                ADD_REFERENCE,
                REMOVE_REFERENCE,
                ENTITY_REMOVE_LISTENER,
                PHANTOM_REMOVE_LISTENER,
                WORLD_OBJECT_ADD_PROPERTY,
            ],
        );
        e.register_double(COS, |_, a| f64_arg(a).cos().into_ret());
    }

    fn check_copied_settings(e: &Engine, me: u32) {
        for (offset, value) in [
            (0x34u32, 1.5f32),
            (0x38, 2.5),
            (0x5c, 3.5),
            (0x50, 4.5),
            (0x54, 5.5),
            (0x58, 6.5),
            (0x60, 7.5),
            (0x68, 8.5),
            (0x6c, 9.5),
            (0x70, 10.5),
            (0xa8, 11.5),
        ] {
            assert_eq!(e.mem.f32(me + offset), value, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(me + 0x64), 9);
        assert_eq!(e.mem.u32(me + 0xac), 12);
        assert_eq!(e.mem.u8(me + 0xb0), 1);
        assert_eq!(load4(e, me + 0x10), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(e.mem.f32(me + 0xa4), (1.0f64).cos() as f32);
        let up = load4(e, me + 0x40);
        assert!(near(up[1], 1.0) && up[0] == 0.0 && up[2] == 0.0 && up[3] == 0.0);
    }

    #[test]
    fn t_00cad500_sets_the_proxy_up_from_the_cinfo() {
        let Rig { mut e, proxy, .. } = rig();
        let me = proxy.addr();
        setup_callees(&mut e);
        let old = e.mem.alloc(0x10);
        e.mem.set_u32(me + 0x30, old);
        let new_phantom = e.mem.alloc(0x200);
        let cinfo = cinfo_block(&mut e, new_phantom);
        array(&mut e, me + 0x8c, &[0xb1, 0xb2], 2);
        array(&mut e, me + 0x98, &[0xc1], 1);
        e.mem.set_u32(me + 0x78, 5);
        e.call(0x00ca_d500, &args![proxy, cinfo]);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![new_phantom]]);
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![old]]);
        assert_eq!(e.mem.u32(me + 0x30), new_phantom);
        check_copied_settings(&e, me);
        assert_eq!(
            calls(&e, ENTITY_REMOVE_LISTENER),
            vec![vec![0xb1, me + 8], vec![0xb2, me + 8]]
        );
        assert_eq!(
            calls(&e, PHANTOM_REMOVE_LISTENER),
            vec![vec![0xc1, me + 0xc]]
        );
        assert_eq!(e.mem.u32(me + 0x90), 0);
        assert_eq!(e.mem.u32(me + 0x9c), 0);
        assert_eq!(e.mem.u32(me + 0x78), 0);
        assert_eq!(load4(&e, me + 0x20), [0.0; 4]);
        assert_eq!(
            calls(&e, WORLD_OBJECT_ADD_PROPERTY),
            vec![vec![new_phantom, PROXY_PROPERTY_KEY, me, 0]]
        );
        assert!(calls(&e, SHAPE_PHANTOM_SET_POSITION).is_empty());
    }

    #[test]
    fn t_00cad6f0_removes_a_listener_by_moving_the_last_one_in() {
        let Rig { mut e, proxy, .. } = rig();
        let me = proxy.addr();
        let data = array(&mut e, me + 0x80, &[0xa1, 0xb2, 0xc3], 4);
        e.call(0x00ca_d6f0, &args![proxy, 0xb2u32]);
        assert_eq!(e.mem.u32(me + 0x84), 2);
        assert_eq!(e.mem.u32(data), 0xa1);
        assert_eq!(e.mem.u32(data + 4), 0xc3);
    }

    /// Two manifold points, one owned by each of two owners of `kind`, and a list of the
    /// owners at `list`; returns `(owner x, owner y, entry y collidable)`.
    fn removal_setup(e: &mut Engine, proxy: Proxy, kind: u8, list: u32) -> (u32, u32, u32) {
        let (owner_x, collidable_x) = owner_block(e, kind);
        let (owner_y, collidable_y) = owner_block(e, kind);
        let entry_x = contact(e, collidable_x, [0.0, 1.0, 0.0], 0.5);
        let entry_y = contact(e, collidable_y, [0.0, 1.0, 0.0], 0.5);
        manifold(e, proxy, &[entry_x, entry_y]);
        array(e, proxy.addr() + list, &[owner_x, owner_y], 2);
        (owner_x, owner_y, collidable_y)
    }

    #[test]
    fn t_00cad740_entity_removed_forgets_the_entity() {
        let Rig { mut e, proxy, .. } = rig();
        let me = proxy.addr();
        noop(&mut e, &[ENTITY_REMOVE_LISTENER]);
        let (owner_x, owner_y, collidable_y) = removal_setup(&mut e, proxy, 1, 0x8c);
        e.call(0x00ca_d740, &args![me + 8, owner_x]);
        assert_eq!(
            calls(&e, ENTITY_REMOVE_LISTENER),
            vec![vec![owner_x, me + 8]]
        );
        assert_eq!(e.mem.u32(me + 0x78), 1);
        assert_eq!(e.mem.u32(e.mem.u32(me + 0x74) + 0x48), collidable_y);
        assert_eq!(e.mem.u32(me + 0x90), 1);
        assert_eq!(e.mem.u32(e.mem.u32(me + 0x8c)), owner_y);
    }

    #[test]
    fn t_00cad820_phantom_removed_forgets_the_phantom() {
        let Rig { mut e, proxy, .. } = rig();
        let me = proxy.addr();
        noop(&mut e, &[PHANTOM_REMOVE_LISTENER]);
        let (owner_x, owner_y, collidable_y) = removal_setup(&mut e, proxy, 2, 0x98);
        e.call(0x00ca_d820, &args![me + 0xc, owner_x]);
        assert_eq!(
            calls(&e, PHANTOM_REMOVE_LISTENER),
            vec![vec![owner_x, me + 0xc]]
        );
        assert_eq!(e.mem.u32(me + 0x78), 1);
        assert_eq!(e.mem.u32(e.mem.u32(me + 0x74) + 0x48), collidable_y);
        assert_eq!(e.mem.u32(me + 0x9c), 1);
        assert_eq!(e.mem.u32(e.mem.u32(me + 0x98)), owner_y);
    }

    #[test]
    fn t_00cad900_reports_the_arrays_to_the_statistics_collector() {
        let Rig { mut e, proxy, .. } = rig();
        let me = proxy.addr();
        let report = 0x0900_0c00;
        let collector_vt = 0x0200_0800;
        vtable(&mut e, collector_vt, &[(0x14, report)]);
        noop(&mut e, &[report]);
        e.register_double(GET_ALLOCATED_SIZE, |_, a| (a[1] + 8).into_ret());
        array(&mut e, me + 0x74, &[0, 0], 4);
        array(&mut e, me + 0x8c, &[0], NO_STORAGE | 1);
        array(&mut e, me + 0x98, &[0, 0, 0], 4);
        array(&mut e, me + 0x80, &[0], 2);
        let collector = object(&mut e, 0x20, collector_vt);
        let data = |e: &Engine, offset: u32| e.mem.u32(me + offset);
        e.call(0x00ca_d900, &args![proxy, collector, 0u32]);
        assert_eq!(
            calls(&e, report),
            vec![
                vec![
                    collector,
                    3,
                    NAME_MANIFOLD,
                    data(&e, 0x74),
                    0xe0,
                    4 * 0x70 + 8
                ],
                vec![collector, 3, NAME_MANIFOLD, data(&e, 0x98), 12, 24],
                vec![collector, 3, NAME_LISTENERS, data(&e, 0x80), 4, 16],
            ]
        );
        // A collector that wants everything also gets the array without storage.
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00ca_d900, &args![proxy, collector, 0u32]);
        let all = calls(&e, report);
        assert_eq!(all.len(), 7);
        assert_eq!(
            all[4],
            vec![collector, 3, NAME_OVERLAPS, data(&e, 0x8c), 4, 12]
        );
    }

    #[test]
    fn t_00cada30_adds_a_listener_growing_the_list_first() {
        let Rig {
            mut e,
            proxy,
            router,
            ..
        } = rig();
        let me = proxy.addr();
        e.mem.set_u32(me + 0x88, NO_STORAGE);
        e.register_double(ARRAY_RESERVE_MORE, |e, a| {
            let data = e.mem.alloc(16);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 4);
            0u32.into_ret()
        });
        e.call(0x00ca_da30, &args![proxy, 0x7001u32]);
        e.call(0x00ca_da30, &args![proxy, 0x7002u32]);
        let allocator = e.mem.u32(router + 0x10);
        assert_eq!(
            calls(&e, ARRAY_RESERVE_MORE),
            vec![vec![allocator, me + 0x80, 4]]
        );
        assert_eq!(e.mem.u32(me + 0x84), 2);
        let data = e.mem.u32(me + 0x80);
        assert_eq!(e.mem.u32(data), 0x7001);
        assert_eq!(e.mem.u32(data + 4), 0x7002);
    }

    #[test]
    fn t_00cada70_constructs_the_proxy() {
        let Rig { mut e, .. } = rig();
        setup_callees(&mut e);
        let this = e.mem.alloc(0xc0);
        let phantom = e.mem.alloc(0x200);
        let cinfo = cinfo_block(&mut e, phantom);
        assert_eq!(e.call(0x00ca_da70, &args![this, cinfo]).u32(), this);
        assert_eq!(e.mem.u32(this), PROXY_VTABLE);
        assert_eq!(e.mem.u32(this + 8), ENTITY_LISTENER_VTABLE);
        assert_eq!(e.mem.u32(this + 0xc), PHANTOM_LISTENER_VTABLE);
        assert_eq!(e.mem.u16(this + 6), 1);
        for offset in [0x7cu32, 0x88, 0x94, 0xa0] {
            assert_eq!(e.mem.u32(this + offset), NO_STORAGE, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x30), phantom);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![phantom]]);
        assert!(calls(&e, REMOVE_REFERENCE).is_empty());
        check_copied_settings(&e, this);
        assert_eq!(
            calls(&e, WORLD_OBJECT_ADD_PROPERTY),
            vec![vec![phantom, PROXY_PROPERTY_KEY, this, 0]]
        );
    }

    #[test]
    fn t_00cadb00_destroys_the_proxy() {
        let Rig {
            mut e,
            proxy,
            router,
            ..
        } = rig();
        let me = proxy.addr();
        noop(
            &mut e,
            &[
                ENTITY_REMOVE_LISTENER,
                PHANTOM_REMOVE_LISTENER,
                WORLD_OBJECT_REMOVE_PROPERTY,
                REMOVE_REFERENCE,
            ],
        );
        let phantom = e.mem.alloc(0x200);
        e.mem.set_u32(me + 0x30, phantom);
        array(&mut e, me + 0x8c, &[0xb1], 1);
        array(&mut e, me + 0x98, &[0xc1, 0xc2], 2);
        array(&mut e, me + 0x80, &[0xd1], 2);
        manifold(&mut e, proxy, &[]);
        let (bodies, phantoms, listeners, points) = (
            e.mem.u32(me + 0x8c),
            e.mem.u32(me + 0x98),
            e.mem.u32(me + 0x80),
            e.mem.u32(me + 0x74),
        );
        e.call(0x00ca_db00, &args![proxy]);
        assert_eq!(calls(&e, ENTITY_REMOVE_LISTENER), vec![vec![0xb1, me + 8]]);
        assert_eq!(
            calls(&e, PHANTOM_REMOVE_LISTENER),
            vec![vec![0xc1, me + 0xc], vec![0xc2, me + 0xc]]
        );
        let removed = calls(&e, WORLD_OBJECT_REMOVE_PROPERTY);
        assert_eq!(removed.len(), 1);
        assert_eq!(
            (removed[0][0], removed[0][2]),
            (phantom, PROXY_PROPERTY_KEY)
        );
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![phantom]]);
        let allocator = e.mem.u32(router + 0x10);
        assert_eq!(
            calls(&e, FREE),
            vec![
                vec![allocator, phantoms, 8],
                vec![allocator, bodies, 4],
                vec![allocator, listeners, 8],
                vec![allocator, points, 4 * ROOT_CD_POINT_SIZE],
            ]
        );
        for offset in [0x74u32, 0x80, 0x8c, 0x98] {
            assert_eq!(e.mem.u32(me + offset), 0);
            assert_eq!(e.mem.u32(me + offset + 4), 0);
            assert_eq!(e.mem.u32(me + offset + 8), NO_STORAGE);
        }
        assert_eq!(e.mem.u32(me), REFERENCED_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(me + 8), BASE_ENTITY_LISTENER_VTABLE);
        assert_eq!(e.mem.u32(me + 0xc), BASE_PHANTOM_LISTENER_VTABLE);
    }

    #[test]
    fn t_00cadca0_copies_a_plane_that_faces_the_displacement() {
        let Rig { mut e, .. } = rig();
        let planes = e.mem.alloc(0x100);
        store4(&mut e, planes, [0.0, 1.0, 0.0, 0.0]);
        store4(&mut e, planes + 0x10, [7.0, 8.0, 9.0, 10.0]);
        e.mem.set_u32(planes + 0x30, 0x1234);
        let array = e.mem.alloc(16);
        e.mem.set_u32(array, planes);
        e.mem.set_u32(array + 4, 1);
        e.mem.set_u32(array + 8, 4);
        let vector = e.mem.alloc(16);
        store4(&mut e, vector, [0.0, 0.5, 0.0, 0.0]);
        // The limit is below the dot product: nothing is added.
        assert!(!e
            .call(0x00ca_dca0, &args![array, 0u32, 0.25f32, vector])
            .bool());
        assert_eq!(e.mem.u32(array + 4), 1);
        assert!(e
            .call(0x00ca_dca0, &args![array, 0u32, 1.0f32, vector])
            .bool());
        assert_eq!(e.mem.u32(array + 4), 2);
        let added = load4(&e, planes + 0x40);
        assert!(added[0] == 0.0 && near(added[1], 1.0) && added[2] == 0.0);
        assert_eq!(load4(&e, planes + 0x50), [7.0, 8.0, 9.0, 10.0]);
        assert_eq!(e.mem.u32(planes + 0x70), 0x1234);
    }

    /// A proxy ready to integrate with `iterations` casts: the doubles for the shape phantom
    /// (its motion at `motion`), the collectors and the profiling stream.
    struct Integration {
        e: Engine,
        proxy: Proxy,
        phantom: u32,
        motion: u32,
        stream: u32,
        c1: u32,
        c2: u32,
        step: u32,
        velocity: u32,
    }

    const GET_MOTION: u32 = 0x0900_0d00;
    const PHANTOM_CAST: u32 = 0x0900_0d10;
    const UPDATE: u32 = 0x0900_0d20;
    const RESET: u32 = 0x0900_0d30;

    fn integration(iterations: i32) -> Integration {
        let Rig {
            mut e,
            proxy,
            stream,
            ..
        } = rig();
        let me = proxy.addr();
        e.mem.set_f32(me + 0x5c, 0.25);
        e.mem.set_f32(me + 0x58, 0.5);
        e.mem.set_i32(me + 0xac, iterations);
        vtable(&mut e, PROXY_VTABLE, &[(0xc, UPDATE)]);
        e.mem.set_u32(me, PROXY_VTABLE);
        let phantom_vt = 0x0200_0900;
        vtable(
            &mut e,
            phantom_vt,
            &[(0x10, GET_MOTION), (0x38, PHANTOM_CAST)],
        );
        let phantom = phantom(&mut e, proxy, phantom_vt);
        let motion_state = e.mem.u32(phantom + 0x18);
        store4(&mut e, motion_state + 0x30, [1.0, 2.0, 3.0, 4.0]);
        let motion = e.mem.alloc(0x100);
        e.register_double(GET_MOTION, move |_, _| motion.into_ret());
        let collector_vt = 0x0200_0a00;
        vtable(&mut e, collector_vt, &[(8, RESET)]);
        let c1 = object(&mut e, 0x40, collector_vt);
        let c2 = object(&mut e, 0x40, collector_vt);
        noop(
            &mut e,
            &[
                PHANTOM_CAST,
                UPDATE,
                RESET,
                SORT_HITS,
                SHAPE_PHANTOM_SET_POSITION,
            ],
        );
        let step = e.mem.alloc(0x10);
        e.mem.set_f32(step + 8, 1.0 / 60.0);
        let velocity = e.mem.alloc(16);
        Integration {
            e,
            proxy,
            phantom,
            motion,
            stream,
            c1,
            c2,
            step,
            velocity,
        }
    }

    #[test]
    fn t_00cade20_integrates_one_round() {
        let Integration {
            mut e,
            proxy,
            phantom,
            stream,
            c1,
            c2,
            step,
            velocity,
            ..
        } = integration(1);
        let me = proxy.addr();
        e.register_double(SIMPLEX_SOLVE, |e, a| {
            store4(e, a[1] + 0x10, [1.0, 2.0, 3.0, 0.0]);
            e.mem.set_f32(a[1] + 0x20, 1.0 / 60.0);
            0u32.into_ret()
        });
        let positions = Rc::new(RefCell::new(Vec::new()));
        let log = positions.clone();
        e.register_double(SHAPE_PHANTOM_SET_POSITION, move |e, a| {
            log.borrow_mut().push(load4(e, a[1]));
            0u32.into_ret()
        });
        e.call(
            0x00ca_de20,
            &args![proxy, step, velocity, c1, c2, 0u32, 0i32],
        );
        assert_eq!(load4(&e, me + 0x10), [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(*positions.borrow(), vec![[1.0, 2.0, 3.0, 4.0]]);
        let set = calls(&e, SHAPE_PHANTOM_SET_POSITION);
        assert_eq!((set[0][0], set[0][2]), (phantom, 0.75f32.to_bits()));
        assert_eq!(calls(&e, SIMPLEX_SOLVE).len(), 1);
        assert_eq!(calls(&e, PHANTOM_CAST).len(), 1);
        assert_eq!(calls(&e, UPDATE), vec![vec![me, c2, c1, 0]]);
        assert_eq!(calls(&e, RESET), vec![vec![c1], vec![c2]]);
        // The wide start record and six more records went to the profiling stream.
        assert_eq!(e.mem.u32(stream + 4), stream + 0x40 + 0x10 + 6 * 12);
    }

    #[test]
    fn t_00cafad0_integrates_with_its_own_collectors() {
        let Integration {
            mut e,
            proxy,
            phantom,
            stream,
            step,
            velocity,
            ..
        } = integration(0);
        e.call(0x00ca_fad0, &args![proxy, step, velocity]);
        let set = calls(&e, SHAPE_PHANTOM_SET_POSITION);
        assert_eq!((set.len(), set[0][0]), (1, phantom));
        assert!(calls(&e, PHANTOM_CAST).is_empty());
        assert_eq!(e.mem.u32(stream + 4), stream + 0x40 + 0x10 + 12);
    }

    #[test]
    fn t_00cafbd0_integrates_with_the_given_collectors() {
        let Integration {
            mut e,
            proxy,
            phantom,
            stream,
            c1,
            c2,
            step,
            velocity,
            ..
        } = integration(0);
        e.call(0x00ca_fbd0, &args![proxy, step, velocity, c1, c2]);
        let set = calls(&e, SHAPE_PHANTOM_SET_POSITION);
        assert_eq!((set.len(), set[0][0]), (1, phantom));
        assert_eq!(e.mem.u32(stream + 4), stream + 0x40 + 0x10 + 12);
    }

    #[test]
    fn t_00cafbf0_integrates_and_moves_the_motion_state() {
        let Integration {
            mut e,
            proxy,
            motion,
            stream,
            step,
            velocity,
            ..
        } = integration(0);
        let out = e.mem.alloc(0x100);
        let cursor = e.mem.alloc(8);
        e.mem.set_u32(cursor, out);
        e.call(0x00ca_fbf0, &args![proxy, step, velocity, cursor, 2i32]);
        assert_eq!(load4(&e, motion + 0x30), [1.0, 2.0, 3.0, 4.0]);
        assert!(calls(&e, SHAPE_PHANTOM_SET_POSITION).is_empty());
        assert_eq!(calls(&e, GET_MOTION).len(), 1);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x40 + 0x10 + 12);
    }

    #[test]
    fn t_00cae7f0_refreshes_the_manifold_around_the_phantom() {
        let Rig { mut e, proxy, .. } = rig();
        let me = proxy.addr();
        e.mem.set_f32(me + 0x5c, 0.25);
        e.mem.set_f32(me + 0x58, 0.5);
        vtable(&mut e, PROXY_VTABLE, &[(0xc, UPDATE)]);
        e.mem.set_u32(me, PROXY_VTABLE);
        let phantom_vt = 0x0200_0b00;
        vtable(&mut e, phantom_vt, &[(0x40, PHANTOM_CAST)]);
        let phantom = phantom(&mut e, proxy, phantom_vt);
        let input = e.mem.u32(e.mem.u32(phantom + 8) + 0x78);
        e.mem.set_u32(input, 0x11);
        e.mem.set_u32(input + 4, 0x22);
        e.mem.set_f32(input + 8, 5.0);
        e.mem.set_u32(input + 0x14, 0x55);
        store4(&mut e, input + 0x20, [1.0, 2.0, 3.0, 4.0]);
        let collector_vt = 0x0200_0c00;
        vtable(&mut e, collector_vt, &[(8, RESET)]);
        let collector = object(&mut e, 0x40, collector_vt);
        noop(&mut e, &[RESET]);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(PHANTOM_CAST, move |e, a| {
            log.borrow_mut().push((
                e.mem.u32(a[2]),
                e.mem.u32(a[2] + 4),
                e.mem.f32(a[2] + 8),
                e.mem.u32(a[2] + 0x14),
                load4(e, a[2] + 0x20),
            ));
            0u32.into_ret()
        });
        let local = Rc::new(RefCell::new(Vec::new()));
        let log = local.clone();
        e.register_double(UPDATE, move |e, a| {
            log.borrow_mut()
                .push((a[1], e.mem.u32(a[2] + 0x14), e.mem.u32(a[2] + 0x18), a[3]));
            0u32.into_ret()
        });
        e.call(0x00ca_e7f0, &args![proxy, collector]);
        assert_eq!(calls(&e, RESET), vec![vec![collector]]);
        assert_eq!(
            *seen.borrow(),
            vec![(0x11, 0x22, 5.75, 0x55, [1.0, 2.0, 3.0, 4.0])]
        );
        assert_eq!(*local.borrow(), vec![(collector, 0, INLINE_EIGHT, 0)]);
    }

    /// A one-point manifold whose plane the proxy's `extractSurfaceConstraintInfo` double
    /// makes `(0, 1, 0)` with the surface velocity `(3, 0, 0)`; the solver double marks the
    /// plane as supporting and answers with `solved`.
    fn support_rig(solved: [f32; 4], flag_offset: u32, velocity_offset: u32) -> Rig {
        let rig = rig();
        let mut e = rig.e;
        let me = rig.proxy.addr();
        let extract = 0x0900_0e00;
        vtable(&mut e, PROXY_VTABLE, &[(0x10, extract)]);
        e.mem.set_u32(me, PROXY_VTABLE);
        e.register_double(extract, |e, a| {
            store4(e, a[2], [0.0, 1.0, 0.0, 0.0]);
            store4(e, a[2] + 0x10, [3.0, 0.0, 0.0, 0.0]);
            0u32.into_ret()
        });
        e.register_double(SIMPLEX_SOLVE, move |e, a| {
            let flags = e.mem.u32(a[0] + flag_offset);
            e.mem.set_u8(flags, 1);
            store4(e, a[1] + velocity_offset, solved);
            0u32.into_ret()
        });
        store4(&mut e, me + 0x40, [0.0, 1.0, 0.0, 0.0]);
        e.mem.set_f32(me + 0xa4, 0.5);
        let (_, collidable) = owner_block(&mut e, 0);
        let entry = contact(&mut e, collidable, [0.0, 1.0, 0.0], 0.5);
        manifold(&mut e, rig.proxy, &[entry]);
        Rig { e, ..rig }
    }

    #[test]
    fn t_00cae900_checks_the_support_with_a_collector() {
        let Rig {
            mut e,
            proxy,
            stream,
            ..
        } = support_rig([1.0, 0.0, 0.0, 0.0], 0x84, 0x10);
        let direction = e.mem.alloc(16);
        store4(&mut e, direction, [0.0, -1.0, 0.0, 0.0]);
        let info = e.mem.alloc(0x40);
        e.mem.set_u32(info, 9);
        let collector = e.mem.alloc(0x40);
        e.call(0x00ca_e900, &args![proxy, direction, info, collector]);
        assert_eq!(e.mem.u32(info), 2);
        let normal = load4(&e, info + 0x10);
        assert!(near(normal[1], 1.0) && normal[0] == 0.0 && normal[2] == 0.0);
        assert_eq!(load4(&e, info + 0x20), [3.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.f32(info + 0x30), 0.0);
        assert_eq!(calls(&e, SIMPLEX_SOLVE).len(), 1);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x40 + 2 * 12);
        // A solved velocity equal to the request means no support.
        e.register_double(SIMPLEX_SOLVE, |e, a| {
            store4(e, a[1] + 0x10, [0.0, -1.0, 0.0, 0.0]);
            0u32.into_ret()
        });
        e.call(0x00ca_e900, &args![proxy, direction, info, collector]);
        assert_eq!(e.mem.u32(info), 0);
        assert_eq!(load4(&e, info + 0x10), [0.0; 4]);
    }

    #[test]
    fn t_00caefd0_checks_the_support_without_the_collector() {
        let Rig {
            mut e,
            proxy,
            stream,
            ..
        } = support_rig([0.0, -0.5, 0.0, 0.0], 0x74, 0x10);
        let direction = e.mem.alloc(16);
        store4(&mut e, direction, [0.0, -1.0, 0.0, 0.0]);
        let info = e.mem.alloc(0x40);
        e.call(0x00ca_efd0, &args![proxy, direction, info]);
        assert_eq!(e.mem.u8(info), 1);
        assert_eq!(e.mem.u8(info + 1), 0);
        let normal = load4(&e, info + 0x10);
        assert!(near(normal[1], 1.0) && normal[0] == 0.0 && normal[2] == 0.0);
        assert_eq!(load4(&e, info + 0x20), [3.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x40 + 2 * 12);
    }

    #[test]
    fn t_00caf4d0_updates_the_manifold() {
        let Rig { mut e, proxy, .. } = rig();
        let me = proxy.addr();
        e.register_double(ROOT_CD_POINT_ASSIGN, |e, a| {
            copy_bytes(e, a[0], a[1], ROOT_CD_POINT_SIZE);
            0u32.into_ret()
        });
        noop(&mut e, &[ENTITY_ADD_LISTENER]);
        let (owner, collidable) = owner_block(&mut e, 1);
        let hit = contact(&mut e, collidable, [0.0, 1.0, 0.0], 0.25);
        let start = e.mem.alloc(0x40);
        e.mem.set_u32(start + 0x10, hit);
        e.mem.set_u32(start + 0x14, 1);
        let cast = e.mem.alloc(0x40);
        manifold(&mut e, proxy, &[]);
        array(&mut e, me + 0x8c, &[], 4);
        array(&mut e, me + 0x98, &[], 4);
        e.call(0x00ca_f4d0, &args![proxy, start, cast, false]);
        assert_eq!(e.mem.u32(me + 0x78), 1);
        let entry = e.mem.u32(me + 0x74);
        assert_eq!(e.mem.u32(entry + 0x48), collidable);
        assert_eq!(e.mem.f32(entry + 0x1c), 0.25);
        assert_eq!(e.mem.u32(me + 0x90), 1);
        assert_eq!(e.mem.u32(e.mem.u32(me + 0x8c)), owner);
        assert_eq!(calls(&e, ENTITY_ADD_LISTENER), vec![vec![owner, me + 8]]);
        assert_eq!(calls(&e, ROOT_CD_POINT_ASSIGN).len(), 1);
    }

    #[test]
    fn t_00cafd00_checks_the_support_with_its_own_collector() {
        let Rig {
            mut e,
            proxy,
            stream,
            ..
        } = rig();
        e.register_double(SIMPLEX_SOLVE, |e, a| {
            store4(e, a[1] + 0x10, [1.0, 0.0, 0.0, 0.0]);
            0u32.into_ret()
        });
        let direction = e.mem.alloc(16);
        store4(&mut e, direction, [1.0, 0.0, 0.0, 0.0]);
        let info = e.mem.alloc(0x40);
        e.mem.set_u32(info, 9);
        e.mem.set_f32(info + 0x30, 5.0);
        e.call(0x00ca_fd00, &args![proxy, direction, info]);
        assert_eq!(e.mem.u32(info), 0);
        assert_eq!(e.mem.f32(info + 0x30), 0.0);
        assert_eq!(calls(&e, SIMPLEX_SOLVE).len(), 1);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x40 + 2 * 12);
    }

    #[test]
    fn t_00cafd90_measures_how_far_apart_two_contacts_are() {
        let Rig { mut e, proxy, .. } = rig();
        e.mem.set_f32(proxy.addr() + 0x60, 0.5);
        let (owner, owned) = owner_block(&mut e, 1);
        let (_, plain) = owner_block(&mut e, 0);
        let first = contact(&mut e, plain, [0.0, 1.0, 0.0], 0.5);
        let second = contact(&mut e, plain, [1.0, 0.0, 0.0], 0.25);
        let distance = e.call(0x00ca_fd90, &args![proxy, first, second]).f32();
        assert_eq!(distance, 2.5625);
        // A body at the first point adds the squared velocity difference.
        e.register_double(BODY_POINT_VELOCITY, |e, a| {
            store4(e, a[2], [3.0, 0.0, 0.0, 0.0]);
            0u32.into_ret()
        });
        e.mem.set_u32(first + 0x48, owned);
        let distance = e.call(0x00ca_fd90, &args![proxy, first, second]).f32();
        let expected = (9.0 * (0.1f32 as f64) + 2.5 + 0.0625) as f32;
        assert!((distance - expected).abs() < 1e-5);
        assert_eq!(calls(&e, BODY_POINT_VELOCITY).len(), 1);
        assert_eq!(calls(&e, BODY_POINT_VELOCITY)[0][0..2], [owner, first]);
    }
}

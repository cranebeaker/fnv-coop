//! `BSHavok/bhkragdollpenetrationutil.obj` (Xbox PDB source unit), subsystem `BSHavok`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkRagdollPenetrationUtil` (0x30 bytes, RTTI-confirmed on its vtable `010c7324`) keeps
//! the ragdolls that are being checked for penetration of the world as an array of
//! `ConstrainedSystem` records (0x24 bytes each): the ragdoll root, its
//! `hkaRagdollInstance`, the generated `hkaSkeleton`, a bounding phantom and an
//! `hkaDetectRagdollPenetration` detector with its output. `Update` runs the detector for
//! every system (within a time budget kept in the util's `BSPrecisionTimer`), colours the
//! penetrating bodies through their collision filter and pushes them out with impulses.
//!
//! x87 and SSE notes. The game compares and rounds on the x87 stack and in SSE registers;
//! the translations compute in `f64` and round to `f32` at each `float` store (the guide's
//! rule), SSE code lane by lane in `f32`, and read the x87 condition codes the way the
//! `FNSTSW`/`TEST AH` pairs read them (NaN included). `RSQRTSS` / `RCPPS` are the host
//! CPU's estimates. A float that the code copies with `FLD`/`FSTP` has a signalling NaN
//! quieted ([`x87_quiet`]).
//!
//! The exception-unwinding frames (`FS:[0]` registrations) and `/GS` cookie checks of the
//! functions marked `Translation incomplete` are not modeled.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// The game heap singleton and its `Deallocate` (`00aa4060`, thiscall `(block)`).
const HEAP_SINGLETON: u32 = 0x011f_6238;
const HEAP_FREE: u32 = 0x00aa_4060;
/// `hkMemoryRouter` lookup (`00c85750`); `router + 0x10` is the allocator (slot `4`
/// allocates `(size)`, `8` frees `(pointer, size)`, `0xC` allocates with the granted
/// size written back, `0x10` frees `(pointer, bytes)`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `BSPrecisionTimer::GetTimer` (Xbox PDB, `00aa4d80`), 64 bits in `EDX:EAX`, and the
/// `ST0` to 64-bit integer conversion `00ec62f6`.
const GET_TIMER: u32 = 0x00aa_4d80;
const FLOAT_TO_INT64: u32 = 0x00ec_62f6;
/// `hkaDetectRagdollPenetration`: `00d1fa20` resets its per-bone state (every entry of the
/// array at `+0x18` to 1, the byte at `+0x24` to 0), `00d1fb00` is `detectPenetration`
/// (Xbox PDB), `00d1fa10` is the output element destructor (a bare `RET`).
const RESET_DETECTION: u32 = 0x00d1_fa20;
const DETECT_PENETRATION: u32 = 0x00d1_fb00;
const OUTPUT_ELEMENT_DESTRUCT: u32 = 0x00d1_fa10;
/// `hkArrayUtil::_reserve` (Xbox PDB): `00c908c0(allocator, array, capacity, element
/// size)` and the doubling grow `00c90940(allocator, array, element size)`, both cdecl.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_GROW: u32 = 0x00c9_0940;
/// Resize helper of the simple arrays (`0042fc00`, thiscall `(capacity, size)`) and the
/// reallocating `BSHavok` helper `00cb9ce0` (thiscall `(capacity, size)`).
const SIMPLE_ARRAY_RESIZE: u32 = 0x0042_fc00;
const SIMPLE_ARRAY_REALLOC: u32 = 0x00cb_9ce0;
/// The dynamic cast `object & -(IsKindOf(object, type))` (`00653290`, `ECX` = object,
/// one stack argument = type) and `bhkWorldObject::GetProperty` (Xbox PDB, cdecl
/// `(out 64-bit, hkObject, key)`, returns `out`).
const DYNAMIC_CAST: u32 = 0x0065_3290;
const GET_PROPERTY: u32 = 0x00c8_5ef0;
/// A bare `RET` called around the collision-filter stores (compiled-out multithread
/// check).
const NO_OP: u32 = 0x00a2_9680;
/// `hkpEntity::activate` (`00c9c1d0`), `hkpMotion::getMass` (`00c86430`, `ST0`) and
/// `hkReferencedObject::removeReference` (`00c905b0`).
const ACTIVATE: u32 = 0x00c9_c1d0;
const GET_MASS: u32 = 0x00c8_6430;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkpAllRayHitCollector` constructor / destructor, and `hkpWorld::castRay`
/// (`00cf2510`, `(input, collector)`).
const COLLECTOR_CONSTRUCT: u32 = 0x004a_3a70;
const COLLECTOR_DESTRUCT: u32 = 0x004a_3bc0;
const WORLD_CAST_RAY: u32 = 0x00cf_2510;
/// The detection input block helpers: `00cd9fe0` (prepares it, returns the pose array
/// struct), `00ce18d0` (fills the pose for a `hkaRagdollInstance`), `00c7c030` (destroys
/// the block).
const PREPARE_INPUT: u32 = 0x00cd_9fe0;
const FILL_POSE: u32 = 0x00ce_18d0;
const DESTROY_INPUT: u32 = 0x00c7_c030;
/// `hkQsTransform::setFromTransformNoScale` (`00ce5070`, `ECX` = destination, one stack
/// argument) and `hkQuaternion::setMul` (`00c66320`, `ECX` = destination, two stack
/// arguments).
const SET_FROM_TRANSFORM_NO_SCALE: u32 = 0x00ce_5070;
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
/// `hkStringPtr`: constructor `00cb0a60`, destructor `00cb0c00`, `operator=(char*)`
/// `00cb0c10`, `operator=(const hkStringPtr&)` `00cb0c30`; `hkString::strDup` `00cb1410`.
const STRING_PTR_CONSTRUCT: u32 = 0x00cb_0a60;
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c10;
const STRING_PTR_ASSIGN_COPY: u32 = 0x00cb_0c30;
const STRING_DUPLICATE: u32 = 0x00cb_1410;
/// The text-stream pieces of the "Rigid Body name could not be found" report: constructor
/// `00cb1600(stream, buffer, size)`, `operator<<(char*)` `00cc01c0`, destructor `00cc00e0`.
const STREAM_CONSTRUCT: u32 = 0x00cb_1600;
const STREAM_WRITE: u32 = 0x00cc_01c0;
const STREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// CRT `strcpy_s`, `strncpy_s`, `strcat_s` (cdecl).
const STRCPY_S: u32 = 0x00ec_65a6;
const STRNCPY_S: u32 = 0x00ec_8d5f;
const STRCAT_S: u32 = 0x00ec_6bd2;
/// The Havok error object (`01268360` holds a pointer to it): slot `0xC` reports a message
/// `(level, id, text, file, line)`.
const ERROR_OBJECT: u32 = 0x0126_8360;
/// Strings and the report's constants.
const NAME_MISSING_TEXT: u32 = 0x010c_7384;
const SOURCE_FILE_TEXT: u32 = 0x010c_7338;
const NUB_SUFFIX: u32 = 0x010c_732c;
const NO_NAME_TEXT: u32 = 0x010c_30bc;
const NAME_MISSING_LINE: u32 = 0x2d1;
const NAME_MISSING_ID: u32 = 0xabba_cc14;
/// Vtables: `hkaRagdollRaycastInterface` (also the util's first base), the
/// `BSSimpleArray<ConstrainedSystem *, 1024>` and `bhkRagdollPenetrationUtil`. The array
/// vtable's slot `8` (at `010c7318`) frees a buffer.
const RAYCAST_INTERFACE_VTABLE: u32 = 0x010c_7304;
const SYSTEM_ARRAY_VTABLE: u32 = 0x010c_7310;
const UTIL_VTABLE: u32 = 0x010c_7324;
/// `hkaSkeleton`'s vtable.
const SKELETON_VTABLE: u32 = 0x010c_4dec;
/// Float constants: `0.0`, `1.0`, `0.5` and `3.0` (`RSQRTSS` refinement), `2.0` (double),
/// four-lane `1.0`, `-0.5` and the identity quaternion, sign / lane masks.
const ZERO: u32 = 0x0101_1d78;
const ONE: u32 = 0x0101_2068;
const HALF: u32 = 0x0101_6248;
const THREE: u32 = 0x0101_7718;
const TWO_DOUBLE: u32 = 0x0101_1590;
const ONES_VECTOR: u32 = 0x010e_c7f0;
const MINUS_HALF_VECTOR: u32 = 0x010c_71e0;
const UNIT_SCALE_VECTOR: u32 = 0x010c_7170;
const IDENTITY_QUATERNION: u32 = 0x010c_71b0;
const SIGN_MASK_ALL: u32 = 0x010c_44d0;
const SIGN_MASK_XYZ: u32 = 0x010c_4c00;
const LANE_MASK_XYZ: u32 = 0x010c_4c10;
/// `1.1920929e-7`: the speed above which a penetrating body is pushed, the impulse
/// factor base and the impulse limit (the last two are run-time `.data` values).
const SPEED_THRESHOLD: u32 = 0x0104_da4c;
const IMPULSE_BASE: u32 = 0x0126_829c;
const IMPULSE_LIMIT: u32 = 0x0126_82a8;
/// The detection time budget: `float` seconds scaled by 1000.0 (double `01017b70`) for
/// `DetectionTimer`, the tick scale `011ac39c` and the budget seconds `012682b4`.
const BUDGET_SECONDS: u32 = 0x0126_82b4;
const MILLISECONDS: u32 = 0x0101_7b70;
const TICK_SCALE: u32 = 0x011a_c39c;
/// Key of the property `fn_00ca11e0` reads and the type-info globals of its and
/// `fn_00ca1f00`'s casts (identities not confirmed).
const PROPERTY_KEY: u32 = 0x0126_7b70;
const PROPERTY_CAST_TYPE: u32 = 0x0126_817c;
const LISTED_ITEM_RTTI: u32 = 0x0126_818c;
const NODE_OWNER_TYPE: u32 = 0x0126_7e64;
const NODE_BODY_TYPE: u32 = 0x0126_81c0;
const NODE_CAST_TYPE: u32 = 0x011f_4428;
const BODY_LIST_TYPE: u32 = 0x0120_43f8;

layout! {
    /// `bhkRagdollPenetrationUtil` (Xbox PDB), 0x30 bytes; PC offsets agree.
    pub struct BhkRagdollPenetrationUtil: 0x30 {
        /// First vtable word (`hkaRagdollRaycastInterface` base).
        0x00 vtable: u32,
        /// Vtable of the embedded `ActiveSystems` (`BSSimpleArray<ConstrainedSystem *>`).
        0x04 activeSystemsVtable: u32,
        /// `ActiveSystems.pBuffer` (Xbox PDB).
        0x08 pActiveSystems: Ptr,
        /// `ActiveSystems.iSize` (Xbox PDB).
        0x0c iActiveSystemsSize: u32,
        /// `ActiveSystems.iReservedSize` (Xbox PDB).
        0x10 iActiveSystemsReserved: u32,
        /// `iGroup` (Xbox PDB): the collision group, shifted left by 16 into ray filters.
        0x14 iGroup: i32,
        /// `bAttemptToInitializeNextUpdate` (Xbox PDB).
        0x18 bAttemptToInitializeNextUpdate: bool,
        /// `uiInitAttempts` (Xbox PDB).
        0x1c uiInitAttempts: u32,
        /// `DetectionTimer` start ticks (`BSPrecisionTimer`, Xbox PDB).
        0x20 timerStart: u64,
        /// `DetectionTimer` limit in ticks.
        0x28 timerLimit: u64,
    }

    /// `bhkRagdollPenetrationUtil::ConstrainedSystem` (Xbox PDB), 0x24 bytes.
    pub struct ConstrainedSystem: 0x24 {
        /// `pRoot3d` (Xbox PDB).
        0x00 pRoot3d: Ptr,
        /// `pRagdollInstance` (Xbox PDB): the `hkaRagdollInstance`.
        0x04 pRagdollInstance: Ptr,
        /// `pSkel` (Xbox PDB): the generated `hkaSkeleton`.
        0x08 pSkel: Ptr,
        /// `pBoundingPhantom` (Xbox PDB): the `bhkAabbPhantom`.
        0x0c pBoundingPhantom: Ptr,
        /// `pDetectionUtil` (Xbox PDB): the `hkaDetectRagdollPenetration`.
        0x10 pDetectionUtil: Ptr,
        /// `PenetratedBones.m_penetrations.m_data` (Xbox PDB), elements of 0x30 bytes.
        0x14 penetratedBones: Ptr,
        /// `PenetratedBones.m_penetrations.m_size` (Xbox PDB).
        0x18 penetratedBonesSize: i32,
        /// `PenetratedBones.m_penetrations.m_capacityAndFlags` (Xbox PDB).
        0x1c penetratedBonesCapacity: u32,
        /// `bPenetrating` (Xbox PDB).
        0x20 bPenetrating: bool,
    }

    /// The input block `Update` builds for `hkaDetectRagdollPenetration::detectPenetration`
    /// (offsets from the code; the roles of the arrays are not confirmed).
    pub struct DetectionInput: 0x38 {
        /// The ragdoll's `hkaSkeleton`.
        0x00 skeleton: Ptr,
        /// First array of 0x30-byte items: data, size, capacity-and-flags.
        0x04 firstData: Ptr,
        0x08 firstSize: i32,
        0x0c firstCapacity: u32,
        /// Second array of 0x30-byte items, the pose `00ce18d0` fills.
        0x10 poseData: Ptr,
        0x14 poseSize: i32,
        0x18 poseCapacity: u32,
        /// Third array of 4-byte items, one per bone, set to 1 by `00cd9fe0`.
        0x1c boneFlagsData: Ptr,
        0x20 boneFlagsSize: i32,
        0x24 boneFlagsCapacity: u32,
        /// Two bytes cleared at construction; `00cd9fe0` sets them to 1 and 0.
        0x28 flagA: u8,
        0x29 flagB: u8,
        /// Fourth array of 4-byte items, zero-filled up to the skeleton's `+0x34` count.
        0x2c wordsData: Ptr,
        0x30 wordsSize: i32,
        0x34 wordsCapacity: u32,
    }
}

/// An SSE register as four `f32` lanes.
type Vec4 = [f32; 4];

fn load_vec(e: &Engine, address: u32) -> Vec4 {
    [
        e.mem.f32(address),
        e.mem.f32(address + 4),
        e.mem.f32(address + 8),
        e.mem.f32(address + 12),
    ]
}

fn store_vec(e: &mut Engine, address: u32, vector: Vec4) {
    for (lane, value) in vector.iter().enumerate() {
        e.mem.set_f32(address + 4 * lane as u32, *value);
    }
}

fn splat(value: f32) -> Vec4 {
    [value; 4]
}

fn mulps(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

fn addps(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

fn subps(a: Vec4, b: Vec4) -> Vec4 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

/// `SHUFPS dst, src, imm`: lanes 0 and 1 from `dst`, lanes 2 and 3 from `src`.
fn shufps(dst: Vec4, src: Vec4, imm: u8) -> Vec4 {
    let select = |lane: u8| usize::from((imm >> (2 * lane)) & 3);
    [
        dst[select(0)],
        dst[select(1)],
        src[select(2)],
        src[select(3)],
    ]
}

/// `XORPS` / `ANDPS` with a mask kept as raw lane bits.
fn bitwise(a: Vec4, mask: [u32; 4], combine: fn(u32, u32) -> u32) -> Vec4 {
    [
        f32::from_bits(combine(a[0].to_bits(), mask[0])),
        f32::from_bits(combine(a[1].to_bits(), mask[1])),
        f32::from_bits(combine(a[2].to_bits(), mask[2])),
        f32::from_bits(combine(a[3].to_bits(), mask[3])),
    ]
}

fn load_mask(e: &Engine, address: u32) -> [u32; 4] {
    [
        e.mem.u32(address),
        e.mem.u32(address + 4),
        e.mem.u32(address + 8),
        e.mem.u32(address + 12),
    ]
}

/// Lanes 0..2 summed the way the code does: `l2 + (l1 + l0)`.
fn sum_xyz(v: Vec4) -> f32 {
    v[2] + (v[1] + v[0])
}

/// `RCPPS` of one lane (the host CPU's estimate on x86).
#[cfg(target_arch = "x86_64")]
fn rcp_estimate(x: f32) -> f32 {
    use std::arch::x86_64::{_mm_cvtss_f32, _mm_rcp_ss, _mm_set_ss};
    // SAFETY: SSE is part of the x86_64 baseline.
    unsafe { _mm_cvtss_f32(_mm_rcp_ss(_mm_set_ss(x))) }
}

#[cfg(not(target_arch = "x86_64"))]
fn rcp_estimate(x: f32) -> f32 {
    1.0 / x
}

/// `RSQRTSS` refined once the way the code does, with the `CMPEQSS`/`ANDNPS` zero guard:
/// `(0.5 * r) * (3 - (sum * r) * r)`, 0 when `sum == 0`. `half` and `three` are the exe's
/// constants.
fn refined_rsqrt(sum: f32, half: f32, three: f32) -> f32 {
    let estimate = rsqrt_estimate(sum);
    let step = three - (sum * estimate) * estimate;
    let value = (half * estimate) * step;
    if sum == 0.0 {
        0.0
    } else {
        value
    }
}

/// `hkAllocator` of the current router (`router + 0x10`).
fn allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// The dynamic cast `00653290`.
fn dynamic_cast(e: &mut Engine, object: u32, to: u32) -> u32 {
    e.call(DYNAMIC_CAST, &args![object, to]).u32()
}

/// Elapsed ticks `GetTimer() - DetectionTimer.start` compared with the limit as signed
/// 64-bit numbers: whether the budget is exceeded.
fn budget_exceeded(e: &mut Engine, this: Ptr<BhkRagdollPenetrationUtil>) -> bool {
    let now = e.call(GET_TIMER, &args![]).u64();
    let start = e.get(this, BhkRagdollPenetrationUtil::timerStart);
    let limit = e.get(this, BhkRagdollPenetrationUtil::timerLimit);
    (now.wrapping_sub(start) as i64) > (limit as i64)
}

/// `system` number `index` of the util's array.
fn system_at(
    e: &Engine,
    this: Ptr<BhkRagdollPenetrationUtil>,
    index: u32,
) -> Ptr<ConstrainedSystem> {
    let data = e
        .get(this, BhkRagdollPenetrationUtil::pActiveSystems)
        .addr();
    Ptr::new(e.mem.u32(data + 4 * index))
}

/// Pushes `value` on a `BSSimpleArray<void *>`-like list (vtable, data, size, capacity)
/// the way `00ca1080` does: when full, an empty list allocates 4 items through slot `4`,
/// a used one doubles (adds 0x400 above 0x400) through slot `0xC` (`realloc(data, new)`)
/// or slot `4` when it has no data.
fn push_realloc(e: &mut Engine, list: u32, value: u32) {
    let capacity = e.mem.u32(list + 0xc);
    if e.mem.u32(list + 8) == capacity {
        let data;
        if capacity == 0 {
            data = e.vcall(list, 4, &args![4u32]).u32();
            e.mem.set_u32(list + 0xc, 4);
        } else {
            let new_capacity = if capacity > 0x400 {
                capacity + 0x400
            } else {
                capacity * 2
            };
            let old = e.mem.u32(list + 4);
            if old != 0 {
                data = e.vcall(list, 0xc, &args![old, new_capacity]).u32();
            } else {
                data = e.vcall(list, 4, &args![new_capacity]).u32();
            }
            e.mem.set_u32(list + 0xc, new_capacity);
        }
        e.mem.set_u32(list + 4, data);
    }
    let size = e.mem.u32(list + 8) + 1;
    e.mem.set_u32(list + 8, size);
    let data = e.mem.u32(list + 4);
    e.mem.set_u32(data + 4 * (size - 1), value);
}

/// Pushes `value` on a `BSSimpleArray`-like list the way `00ca0f00` does: an empty list
/// allocates 4 items through slot `4`, a full one grows by `00042fc00(capacity, size)`
/// (doubling, +0x400 above 0x400).
fn push_resize(e: &mut Engine, list: u32, value: u32) {
    let size = e.mem.u32(list + 8);
    let capacity = e.mem.u32(list + 0xc);
    if size == capacity {
        if capacity == 0 {
            let data = e.vcall(list, 4, &args![4u32]).u32();
            e.mem.set_u32(list + 8, size + 1);
            e.mem.set_u32(list + 4, data);
            e.mem.set_u32(list + 0xc, 4);
            e.mem.set_u32(data, value);
            return;
        }
        let new_capacity = if capacity > 0x400 {
            capacity + 0x400
        } else {
            capacity * 2
        };
        e.call(SIMPLE_ARRAY_RESIZE, &args![list, new_capacity, size]);
        e.mem.set_u32(list + 0xc, new_capacity);
    }
    let size = e.mem.u32(list + 8) + 1;
    e.mem.set_u32(list + 8, size);
    let data = e.mem.u32(list + 4);
    e.mem.set_u32(data + 4 * (size - 1), value);
}

/// Appends a 4-byte `value` to a Havok `hkArray` (data, size, capacity-and-flags) the way
/// `00ca11e0` does: when the size equals the capacity, `hkArrayUtil` grows it (with the
/// allocator of a freshly looked-up router).
fn hk_array_push(e: &mut Engine, array: u32, value: u32) {
    let heap = allocator(e);
    let size = e.mem.i32(array + 4);
    if size as u32 == e.mem.u32(array + 8) & 0x3fff_ffff {
        e.call(ARRAY_GROW, &args![heap, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data + 4 * size, value);
    e.mem.set_u32(array + 4, size + 1);
}

/// `hkArrayUtil::_reserve` ahead of a resize to `needed` elements of `element` bytes: when
/// the capacity (low 30 bits of `array + 8`) is below `needed`, reserves `max(2 *
/// capacity, needed)` (`double`) or exactly `needed`.
fn reserve(e: &mut Engine, array: u32, needed: i32, element: u32, double: bool) {
    let heap = allocator(e);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < needed {
        let new_capacity = if double && 2 * capacity > needed {
            2 * capacity
        } else {
            needed
        };
        e.call(ARRAY_RESERVE, &args![heap, array, new_capacity, element]);
    }
}

// Translated from 00ca0ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaRagdollRaycastInterface::_scalar_deleting_destructor_` (Xbox PDB; vtable `010c7304`
/// RTTI-confirmed): sets the interface vtable and, when bit 0 of `flags` is set, frees the
/// block `this` points to. Returns `this`.
pub fn hka_ragdoll_raycast_interface_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), RAYCAST_INTERFACE_VTABLE);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00ca0cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Restarts detection for every active system (no PDB name): marks each system
/// penetrating (`+0x20` = 1) and resets its detector (`00d1fa20` on `+0x10`), then
/// restarts `DetectionTimer`: start = `GetTimer()`, limit = the ticks of `seconds * scale`
/// where `seconds` is the `float` at `012682b4` times 1000.0 (rounded to `float` first) and
/// `scale` the `float` at `011ac39c` (`00ec62f6`).
pub fn fn_00ca0cd0(e: &mut Engine, this: Ptr<BhkRagdollPenetrationUtil>) {
    let mut index = 0;
    while index < e.get(this, BhkRagdollPenetrationUtil::iActiveSystemsSize) {
        let system = system_at(e, this, index);
        e.set(system, ConstrainedSystem::bPenetrating, true);
        let system = system_at(e, this, index);
        let detector = e.get(system, ConstrainedSystem::pDetectionUtil);
        e.call(RESET_DETECTION, &args![detector]);
        index += 1;
    }
    let budget = e.global::<f32>(BUDGET_SECONDS);
    let milliseconds = e.global::<f64>(MILLISECONDS);
    let budget = (f64::from(x87_quiet(budget)) * milliseconds) as f32;
    let now = e.call(GET_TIMER, &args![]).u64();
    e.set(this, BhkRagdollPenetrationUtil::timerStart, now);
    let scale = e.global::<f32>(TICK_SCALE);
    let ticks = e
        .call(FLOAT_TO_INT64, &args![f64::from(scale) * f64::from(budget)])
        .u64();
    e.set(this, BhkRagdollPenetrationUtil::timerLimit, ticks);
}

// Translated from 00ca0d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `BSSimpleArray<ConstrainedSystem *, 1024>` (no PDB name): installs
/// the array vtable and, when there is a buffer, frees it through the vtable's slot `8`
/// (called at its static address `010c7318`) and clears the buffer, size and reserved
/// size.
pub fn fn_00ca0d40(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SYSTEM_ARRAY_VTABLE);
    let buffer = e.mem.u32(this.addr() + 4);
    if buffer != 0 {
        let free = e.mem.u32(SYSTEM_ARRAY_VTABLE + 8);
        e.call(free, &args![this, buffer]);
        e.mem.set_u32(this.addr() + 4, 0);
        e.mem.set_u32(this.addr() + 0xc, 0);
        e.mem.set_u32(this.addr() + 8, 0);
    }
}

// Translated from 00ca0d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<bhkRagdollPenetrationUtil::ConstrainedSystem *, 1024>::
/// _scalar_deleting_destructor_` (Xbox PDB): the destructor body, then, when bit 0 of
/// `flags` is set, frees the block `this` points to. Returns `this`.
pub fn bs_simple_array_constrained_system_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    fn_00ca0d40(e, this);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this]);
    }
    this
}

// Translated from 00ca0dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca0dc0 - the FS:[0] exception-unwinding frame is not modeled.
/// Constructor of `bhkRagdollPenetrationUtil` (no PDB name; vtable `010c7324`): installs
/// the util vtable, builds the empty `ActiveSystems` array (vtable `010c7310`, no
/// buffer), clears the flag, the attempt counter and the timer words (`+0x1C..+0x2C`; the
/// group at `+0x14` is left alone), runs the array's emptying step (which does nothing for
/// the empty buffer) and clears the timer limit. Returns `this`.
pub fn fn_00ca0dc0(
    e: &mut Engine,
    this: Ptr<BhkRagdollPenetrationUtil>,
) -> Ptr<BhkRagdollPenetrationUtil> {
    let array = this.addr() + 4;
    e.set(this, BhkRagdollPenetrationUtil::vtable, UTIL_VTABLE);
    e.set(
        this,
        BhkRagdollPenetrationUtil::activeSystemsVtable,
        SYSTEM_ARRAY_VTABLE,
    );
    e.set(this, BhkRagdollPenetrationUtil::pActiveSystems, Ptr::NULL);
    e.set(this, BhkRagdollPenetrationUtil::iActiveSystemsSize, 0);
    e.set(this, BhkRagdollPenetrationUtil::iActiveSystemsReserved, 0);
    e.set(
        this,
        BhkRagdollPenetrationUtil::bAttemptToInitializeNextUpdate,
        false,
    );
    e.set(this, BhkRagdollPenetrationUtil::uiInitAttempts, 0);
    e.set(this, BhkRagdollPenetrationUtil::timerStart, 0);
    e.set(this, BhkRagdollPenetrationUtil::timerLimit, 0);
    let buffer = e.mem.u32(array + 4);
    if buffer != 0 {
        e.vcall(array, 8, &args![buffer]);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 0xc, 0);
        e.mem.set_u32(array + 8, 0);
    }
    e.set(this, BhkRagdollPenetrationUtil::timerLimit, 0);
    this
}

// Translated from 00ca0e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a `BSSimpleArray` of 4-byte items (`this`) into the array `target` (no PDB name):
/// an empty source frees the target's buffer; otherwise the target's capacity is grown
/// (empty: slot `4` allocates; used: `00cb9ce0(capacity, size)`) or, when it shrinks below
/// a quarter of its capacity, trimmed, and its size becomes the source's; then every item
/// is copied.
pub fn fn_00ca0e60(e: &mut Engine, this: Ptr, target: Ptr) {
    let source = this.addr();
    let target = target.addr();
    let count = e.mem.u32(source + 8);
    if count == 0 {
        let buffer = e.mem.u32(target + 4);
        if buffer != 0 {
            e.vcall(target, 8, &args![buffer]);
            e.mem.set_u32(target + 4, 0);
            e.mem.set_u32(target + 0xc, 0);
            e.mem.set_u32(target + 8, 0);
        }
    } else {
        let capacity = e.mem.u32(target + 0xc);
        if count > capacity {
            if capacity == 0 {
                let data = e.vcall(target, 4, &args![count]).u32();
                e.mem.set_u32(target + 4, data);
            } else {
                let size = e.mem.u32(target + 8);
                e.call(SIMPLE_ARRAY_REALLOC, &args![target, count, size]);
            }
            e.mem.set_u32(target + 0xc, count);
            e.mem.set_u32(target + 8, count);
        } else if count >= e.mem.u32(target + 8) {
            e.mem.set_u32(target + 8, count);
        } else {
            e.mem.set_u32(target + 8, count);
            if count <= capacity >> 2 {
                e.call(SIMPLE_ARRAY_REALLOC, &args![target, count, count]);
                e.mem.set_u32(target + 0xc, count);
            }
        }
    }
    let mut index = 0;
    while index < e.mem.u32(source + 8) {
        let item = e.mem.u32(e.mem.u32(source + 4) + 4 * index);
        let destination = e.mem.u32(target + 4);
        e.mem.set_u32(destination + 4 * index, item);
        index += 1;
    }
}

// Translated from 00ca0f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collects the nodes of a scene-graph subtree whose bodies are all of kind 0xB (no PDB
/// name). For `node`: when its `+0x1C` object casts to the type at `012043f8` and that
/// object's `+0x10` holds a body list (`+8`): every item of the list's `+0xAC` array
/// (0x30-byte items, count `u16` at `+0xB0`) must answer 0xB to virtual slot `0x20` of its
/// `+0xC` object, and some item of the `+0xB4` array (count at `+0xB8`) must answer
/// something else; then `node` is appended to `list` (`push_resize`). Recurses into the
/// children (`IsNode` slot `0xC`, array `+0xA0`, count `u16` at `+0xA6`).
// The exe passes ECX unchanged into each recursive call; keep it in the
// signature so the registered ABI matches the original thiscall.
#[allow(clippy::only_used_in_recursion)]
pub fn fn_00ca0f00(e: &mut Engine, this: Ptr, node: u32, list: u32) {
    if node == 0 {
        return;
    }
    let owner = e.mem.u32(node + 0x1c);
    if owner != 0 {
        let casted = dynamic_cast(e, owner, BODY_LIST_TYPE);
        if casted != 0 {
            let holder = e.mem.u32(casted + 0x10);
            if holder != 0 {
                let bodies = e.mem.u32(holder + 8);
                if bodies != 0 {
                    let mut all_kind_b = true;
                    let mut index = 0;
                    while index < u32::from(e.mem.u16(bodies + 0xb0)) && all_kind_b {
                        let item = e.mem.u32(e.mem.u32(bodies + 0xac) + 0x30 * index);
                        let object = e.mem.u32(item + 0xc);
                        all_kind_b = e.vcall(object, 0x20, &args![]).u32() == 0xb;
                        index += 1;
                    }
                    let mut other_kind = false;
                    let mut index = 0;
                    while (index as i32) < e.mem.i32(bodies + 0xb8) && !other_kind {
                        let item = e.mem.u32(e.mem.u32(bodies + 0xb4) + 4 * index);
                        let object = e.mem.u32(item + 0xc);
                        other_kind = e.vcall(object, 0x20, &args![]).u32() != 0xb;
                        index += 1;
                    }
                    if all_kind_b && other_kind {
                        push_resize(e, list, node);
                    }
                }
            }
        }
    }
    let as_node = e.vcall(node, 0xc, &args![]).u32();
    if as_node != 0 {
        let mut index = 0;
        if u32::from(e.mem.u16(as_node + 0xa6)) > 0 {
            loop {
                let children = e.mem.u32(as_node + 0xa0);
                let child = e.mem.u32(children + 4 * index);
                fn_00ca0f00(e, this, child, list);
                index += 1;
                if index >= u32::from(e.mem.u16(as_node + 0xa6)) {
                    break;
                }
            }
        }
    }
}

// Translated from 00ca1080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Filters `source` (an `hkArray` of pointers, data / size) into `target` (no PDB name):
/// for each entry, true when its `+0xAC` items (0x30 bytes, count `u16` at `+0xB0`) all
/// answer 0xB to virtual slot `0x20` of their `+0xC` object, and when some item of its
/// `+0xB4` array (count at `+0xB8`) answers differently; entries for which neither holds
/// have their `+0xC` pointer appended to `target` (`push_realloc`).
pub fn fn_00ca1080(e: &mut Engine, _this: Ptr, source: Ptr, target: Ptr) {
    let source = source.addr();
    let target = target.addr();
    let entry = |e: &Engine, index: u32| e.mem.u32(e.mem.u32(source) + 4 * index);
    let mut index = 0;
    while (index as i32) < e.mem.i32(source + 4) {
        let mut all_kind_b = true;
        let mut item = 0;
        while item < u32::from(e.mem.u16(entry(e, index) + 0xb0)) && all_kind_b {
            let current = entry(e, index);
            let element = e.mem.u32(e.mem.u32(current + 0xac) + 0x30 * item);
            let object = e.mem.u32(element + 0xc);
            all_kind_b = e.vcall(object, 0x20, &args![]).u32() == 0xb;
            item += 1;
        }
        let mut other_kind = false;
        let mut item = 0;
        while (item as i32) < e.mem.i32(entry(e, index) + 0xb8) && !other_kind {
            let current = entry(e, index);
            let element = e.mem.u32(e.mem.u32(current + 0xb4) + 4 * item);
            let object = e.mem.u32(element + 0xc);
            other_kind = e.vcall(object, 0x20, &args![]).u32() != 0xb;
            item += 1;
        }
        if !all_kind_b && !other_kind {
            let current = entry(e, index);
            let value = if current == 0 {
                0
            } else {
                e.mem.u32(current + 0xc)
            };
            push_realloc(e, target, value);
        }
        index += 1;
    }
}

// Translated from 00ca11e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca11e0 - the FS:[0] exception-unwinding frame is not modeled.
/// Gathers the Havok objects a scene-graph physics object depends on (no PDB name). For a
/// non-null `body` (whose Havok object is at `+8`): reads the property of the Havok
/// object under key `01267b70` (`GetProperty`) and casts it to the type at `0126817c`;
/// walks the linked list at `body + 0x14` (item, next) while no item has been found whose
/// owner answers 0xC to virtual slot `0x20`, collecting the `+8` of each item whose
/// run-time type chain contains `0126818c` into a local `hkArray`; then, unless such a
/// blocking item was found, and when the property cast failed or items were collected,
/// appends `body + 8` to `list_a` and every collected item to `list_b` (both `hkArray`s of
/// 4-byte pointers).
pub fn fn_00ca11e0(e: &mut Engine, _this: Ptr, body: u32, list_a: u32, list_b: u32) {
    if body == 0 {
        return;
    }
    let hk_object = e.mem.u32(body + 8);
    let property = e.with_stack(8, |e, out| {
        e.call(GET_PROPERTY, &args![out, hk_object, PROPERTY_KEY]);
        e.mem.u32(out.addr())
    });
    let property_cast = if property == 0 {
        0
    } else {
        dynamic_cast(e, property, PROPERTY_CAST_TYPE)
    };
    e.with_stack(12, |e, local| {
        let local = local.addr();
        // The local hkArray: data, size, capacity-and-flags (not owned, empty).
        e.mem.set_u32(local, 0);
        e.mem.set_u32(local + 4, 0);
        e.mem.set_u32(local + 8, 0x8000_0000);
        let mut cursor = body + 0x14;
        let mut blocked = false;
        while cursor != 0 && !blocked {
            let item = e.mem.u32(cursor);
            if item == 0 {
                break;
            }
            let mut rtti = e.vcall(item, 8, &args![]).u32();
            let mut listed = false;
            while rtti != 0 {
                if rtti == LISTED_ITEM_RTTI {
                    listed = true;
                    break;
                }
                rtti = e.mem.u32(rtti + 4);
            }
            if listed {
                let first = e.mem.u32(item + 8);
                let owner = if first != 0 {
                    e.mem.u32(first + 0xc)
                } else {
                    let second = e.mem.u32(item + 0xc);
                    if second != 0 {
                        e.mem.u32(second)
                    } else {
                        0
                    }
                };
                if e.vcall(owner, 0x20, &args![]).u32() == 0xc {
                    blocked = true;
                } else {
                    let value = e.mem.u32(item + 8);
                    hk_array_push(e, local, value);
                }
            }
            cursor = e.mem.u32(cursor + 4);
        }
        let collected = e.mem.i32(local + 4);
        if (property_cast == 0 || collected > 0) && !blocked {
            let own = e.mem.u32(body + 8);
            hk_array_push(e, list_a, own);
            let mut index = 0;
            while index < e.mem.i32(local + 4) {
                let value = e.mem.u32(e.mem.u32(local) + 4 * index as u32);
                hk_array_push(e, list_b, value);
                index += 1;
            }
        }
        let heap = allocator(e);
        let capacity = e.mem.u32(local + 8);
        e.mem.set_u32(local + 4, 0);
        if capacity & 0x8000_0000 == 0 {
            let data = e.mem.u32(local);
            e.vcall(heap, 0x10, &args![data, (capacity & 0x3fff_ffff) * 4]);
        }
    });
}

// Translated from 00ca1410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca1410 - the FS:[0] exception-unwinding frame and the stack-cookie check are not modeled.
/// `bhkRagdollPenetrationUtil::Update` (Xbox PDB): for every active system, runs the
/// penetration detector within the time budget and pushes penetrating bodies out.
///
/// A system is skipped (its detector reset when its first rigid body is in a world) when
/// its first rigid body is missing or has no world, or some rigid body has motion type 4
/// or 5 (`+0xE8`), or, while it is not penetrating, the budget is already exceeded. The
/// detector is reset unless the budget is exceeded and the system is not penetrating;
/// then the detection input is built (arrays sized from the skeleton), the pose filled,
/// the bounding phantom's world aligned with the first body's, `detectPenetration` run and
/// its answer stored as `bPenetrating`. A penetrating system marks every penetrating
/// bone's body (filter-info bit 14, notified through slot `0xC4` of its wrapper) and, for
/// bones flagged 1, applies a point impulse
/// along the penetration direction scaled by `(base - speed * t) * mass` clamped to the
/// limit; a system that stopped penetrating has its detector reset and the bit cleared
/// again.
pub fn bhk_ragdoll_penetration_util_update(e: &mut Engine, this: Ptr<BhkRagdollPenetrationUtil>) {
    let mut index = 0;
    while index < e.get(this, BhkRagdollPenetrationUtil::iActiveSystemsSize) {
        let system = system_at(e, this, index);
        let instance = e.get(system, ConstrainedSystem::pRagdollInstance).addr();
        // hkaRagdollInstance: +0xC rigid body count, +8 rigid body array.
        let body_count = e.mem.i32(instance + 0xc);
        let mut motionless = false;
        let mut position = 0;
        while position < body_count && !motionless {
            let body = e.mem.u32(e.mem.u32(instance + 8) + 4 * position as u32);
            let motion_type = e.mem.u8(body + 0xe8);
            motionless = motion_type == 4 || motion_type == 5;
            position += 1;
        }
        let first_body = e.mem.u32(e.mem.u32(instance + 8));
        let mut skip = first_body == 0 || e.mem.u32(first_body + 8) == 0 || motionless;
        if !skip && !e.get(system, ConstrainedSystem::bPenetrating) && budget_exceeded(e, this) {
            skip = true;
        }
        if skip {
            let system = system_at(e, this, index);
            let instance = e.get(system, ConstrainedSystem::pRagdollInstance).addr();
            let body = e.mem.u32(e.mem.u32(instance + 8));
            if body != 0 && e.mem.u32(body + 8) != 0 {
                let detector = e.get(system, ConstrainedSystem::pDetectionUtil);
                e.call(RESET_DETECTION, &args![detector]);
            }
        } else {
            update_system(e, this, index);
        }
        index += 1;
    }
}

/// The body of `Update` for one system that is not skipped.
fn update_system(e: &mut Engine, this: Ptr<BhkRagdollPenetrationUtil>, index: u32) {
    if !budget_exceeded(e, this)
        || e.get(system_at(e, this, index), ConstrainedSystem::bPenetrating)
    {
        let system = system_at(e, this, index);
        let detector = e.get(system, ConstrainedSystem::pDetectionUtil);
        e.call(RESET_DETECTION, &args![detector]);
    }
    let system = system_at(e, this, index);
    let instance = e.get(system, ConstrainedSystem::pRagdollInstance).addr();
    // hkaRagdollInstance +0x2C: the hkaSkeleton; hkaSkeleton +0x1C bones, +0x34 slots.
    let skeleton = e.mem.u32(instance + 0x2c);
    e.with_stack(DetectionInput::SIZE, |e, input| {
        let input = Ptr::<DetectionInput>::new(input.addr());
        e.set(input, DetectionInput::skeleton, Ptr::new(skeleton));
        for (data, size, capacity) in [
            (
                DetectionInput::firstData,
                DetectionInput::firstSize,
                DetectionInput::firstCapacity,
            ),
            (
                DetectionInput::poseData,
                DetectionInput::poseSize,
                DetectionInput::poseCapacity,
            ),
            (
                DetectionInput::boneFlagsData,
                DetectionInput::boneFlagsSize,
                DetectionInput::boneFlagsCapacity,
            ),
            (
                DetectionInput::wordsData,
                DetectionInput::wordsSize,
                DetectionInput::wordsCapacity,
            ),
        ] {
            e.set(input, data, Ptr::NULL);
            e.set(input, size, 0);
            e.set(input, capacity, 0x8000_0000);
        }
        e.set(input, DetectionInput::flagA, 0);
        e.set(input, DetectionInput::flagB, 0);
        let bones = e.mem.i32(skeleton + 0x1c);
        let base = input.addr();
        reserve(e, base + 0x10, bones, 0x30, true);
        e.mem.set_u32(base + 0x14, bones as u32);
        reserve(e, base + 4, bones, 0x30, true);
        e.mem.set_u32(base + 8, bones as u32);
        reserve(e, base + 0x1c, bones, 4, true);
        e.mem.set_u32(base + 0x20, bones as u32);
        let rounded = (e.mem.i32(skeleton + 0x34) + 3) & !3;
        reserve(e, base + 0x2c, rounded, 4, false);
        let slots = e.mem.i32(skeleton + 0x34);
        reserve(e, base + 0x2c, slots, 4, true);
        let words_size = e.mem.i32(base + 0x30);
        let data = e.mem.u32(base + 0x2c);
        if slots - words_size > 0 {
            for word in 0..(slots - words_size) as u32 {
                e.mem.set_u32(data + 4 * (words_size as u32 + word), 0);
            }
        }
        e.mem.set_u32(base + 0x30, slots as u32);
        let pose = e.call(PREPARE_INPUT, &args![input]).u32();
        let pose_data = e.mem.u32(pose);
        let system = system_at(e, this, index);
        let instance = e.get(system, ConstrainedSystem::pRagdollInstance);
        e.call(FILL_POSE, &args![instance, pose_data]);
        align_phantom_world(e, this, index);
        let system = system_at(e, this, index);
        let was_penetrating = e.get(system, ConstrainedSystem::bPenetrating);
        let detector = e.get(system, ConstrainedSystem::pDetectionUtil);
        let output = system.addr() + 0x14;
        let answer = e.with_stack(4, |e, result| {
            let pointer = e
                .call(DETECT_PENETRATION, &args![detector, result, input, output])
                .u32();
            e.mem.u8(pointer) != 0
        });
        e.set(
            system_at(e, this, index),
            ConstrainedSystem::bPenetrating,
            answer,
        );
        let system = system_at(e, this, index);
        if e.get(system, ConstrainedSystem::bPenetrating) {
            push_out_penetrating(e, this, index);
        } else if was_penetrating {
            let detector = e.get(system, ConstrainedSystem::pDetectionUtil);
            e.call(RESET_DETECTION, &args![detector]);
            clear_penetration_marks(e, this, index);
        }
        e.call(DESTROY_INPUT, &args![input]);
    });
}

/// Gives the system's bounding phantom the world of the first rigid body when they differ:
/// the phantom's slot `0x94` answers its world; the first body's world's back pointer
/// (`+0x350`, or 0) goes to slot `0x9C`.
fn align_phantom_world(e: &mut Engine, this: Ptr<BhkRagdollPenetrationUtil>, index: u32) {
    let system = system_at(e, this, index);
    let phantom = e.get(system, ConstrainedSystem::pBoundingPhantom).addr();
    let instance = e.get(system, ConstrainedSystem::pRagdollInstance).addr();
    let first = e.mem.u32(e.mem.u32(instance + 8));
    let body_world = if first != 0 { e.mem.u32(first + 8) } else { 0 };
    let phantom_world = e.vcall(phantom, 0x94, &args![]).u32();
    if body_world != phantom_world {
        let system = system_at(e, this, index);
        let instance = e.get(system, ConstrainedSystem::pRagdollInstance).addr();
        let first = e.mem.u32(e.mem.u32(instance + 8));
        let world = if first != 0 { e.mem.u32(first + 8) } else { 0 };
        let back_pointer = if world != 0 {
            e.mem.u32(world + 0x350)
        } else {
            0
        };
        let phantom = e.get(system, ConstrainedSystem::pBoundingPhantom).addr();
        e.vcall(phantom, 0x9c, &args![back_pointer]);
    }
}

/// Sets or clears bit 14 of a rigid body's collision filter info (`hkpRigidBody + 0x2C`,
/// reached as `Havok object + 0x10 + 0x1C`) and notifies its wrapper (slot `0xC4`).
/// `set` chooses the direction; nothing happens when the bit is already as wanted.
fn set_filter_bit(e: &mut Engine, wrapper: u32, hk_body: u32, set: bool) {
    let filter = if hk_body != 0 {
        e.mem.u32(hk_body + 0x10 + 0x1c)
    } else {
        0
    };
    if (filter >> 14 & 1 != 0) == set {
        return;
    }
    let new_filter = if set {
        filter | 0x4000
    } else {
        filter & 0xffff_bfff
    };
    if hk_body != 0 {
        e.call(NO_OP, &args![wrapper]);
        e.call(NO_OP, &args![wrapper]);
        e.mem.set_u32(hk_body + 0x10 + 0x1c, new_filter);
    }
    e.vcall(wrapper, 0xc4, &args![]);
}

/// The bone loop of `Update` for a penetrating system: per bone with a rigid body
/// wrapper, the flag in the detector's array (`detector + 0x18`) decides: 0 clears the
/// filter bit; non-zero sets it and, for 1, pushes the body out.
fn push_out_penetrating(e: &mut Engine, this: Ptr<BhkRagdollPenetrationUtil>, index: u32) {
    let system = system_at(e, this, index);
    let detector = e.get(system, ConstrainedSystem::pDetectionUtil).addr();
    let flags_pointer = detector + 0x18;
    let mut bone = 0i32;
    loop {
        let system = system_at(e, this, index);
        let instance = e.get(system, ConstrainedSystem::pRagdollInstance).addr();
        if bone >= e.mem.i32(e.mem.u32(instance + 0x2c) + 0x1c) {
            break;
        }
        let map = e.mem.i32(e.mem.u32(instance + 0x20) + 4 * bone as u32);
        if map >= 0 {
            let hk_body = e.mem.u32(e.mem.u32(instance + 8) + 4 * map as u32);
            if hk_body != 0 {
                let wrapper = e.mem.u32(hk_body + 0xc);
                if wrapper != 0 {
                    let wrapped = e.mem.u32(wrapper + 8);
                    let flag = e.mem.u32(e.mem.u32(flags_pointer) + 4 * bone as u32);
                    if flag == 0 {
                        set_filter_bit(e, wrapper, wrapped, false);
                    } else {
                        set_filter_bit(e, wrapper, wrapped, true);
                        if e.mem.u32(e.mem.u32(flags_pointer) + 4 * bone as u32) == 1 {
                            apply_push(e, this, index, bone, hk_body);
                        }
                    }
                }
            }
        }
        bone += 1;
    }
}

/// The bone loop of `Update` for a system that stopped penetrating: clears the filter bit
/// of every bone's body that has it.
fn clear_penetration_marks(e: &mut Engine, this: Ptr<BhkRagdollPenetrationUtil>, index: u32) {
    let system = system_at(e, this, index);
    let instance = e.get(system, ConstrainedSystem::pRagdollInstance).addr();
    let mut bone = 0i32;
    while bone < e.mem.i32(e.mem.u32(instance + 0x2c) + 0x1c) {
        let map = e.mem.i32(e.mem.u32(instance + 0x20) + 4 * bone as u32);
        if map >= 0 {
            let hk_body = e.mem.u32(e.mem.u32(instance + 8) + 4 * map as u32);
            if hk_body != 0 {
                let wrapper = e.mem.u32(hk_body + 0xc);
                if wrapper != 0 {
                    let wrapped = e.mem.u32(wrapper + 8);
                    set_filter_bit(e, wrapper, wrapped, false);
                }
            }
        }
        bone += 1;
    }
}

/// The impulse of `Update` for bone `bone` of the system's penetrating body `hk_body`.
///
/// The point and direction default to `(0, 0, 0, 0)` and `(0, 0, 1, 0)` and are replaced by
/// the position (`+0x10`) and direction (`+0x20`) of the last `PenetratedBones` entry whose
/// bone index (`i16` at `+0`) is `bone`. The body's relative velocity is its angular
/// velocity crossed with `point - centre of mass (+0x140)` plus its linear velocity
/// (`+0x1B0`); when its speed exceeds `1.1920929e-7` the normalized velocity's dot with the
/// direction becomes `t`, else `t = 0`. The impulse factor is `((base - speed * t) *
/// mass)` clamped to `+-limit`; the impulse is that factor times the direction, applied to
/// the body's motion (`hk_body + 0xE0`, slot `0x50`) at the point after `hkpEntity::activate`.
fn apply_push(
    e: &mut Engine,
    this: Ptr<BhkRagdollPenetrationUtil>,
    index: u32,
    bone: i32,
    hk_body: u32,
) {
    let zero = e.global::<f32>(ZERO);
    let one = e.global::<f32>(ONE);
    let mut point: Vec4 = splat(zero);
    let mut direction: Vec4 = [zero, zero, one, zero];
    let system = system_at(e, this, index);
    let mut entry = 0;
    while entry < e.get(system, ConstrainedSystem::penetratedBonesSize) {
        let data = e.get(system, ConstrainedSystem::penetratedBones).addr();
        let element = data + 0x30 * entry as u32;
        if bone == i32::from(e.mem.i16(element)) {
            let system = system_at(e, this, index);
            let data = e.get(system, ConstrainedSystem::penetratedBones).addr();
            point = load_vec(e, data + 0x30 * entry as u32 + 0x10);
            direction = load_vec(e, data + 0x30 * entry as u32 + 0x20);
        }
        entry += 1;
    }
    let threshold = e.global::<f32>(SPEED_THRESHOLD);
    let centre = load_vec(e, hk_body + 0x140);
    let relative = subps(point, centre);
    let angular = load_vec(e, hk_body + 0x1c0);
    let motion = hk_body + 0xe0;
    let cross_a = mulps(
        shufps(angular, angular, 0xc9),
        shufps(relative, relative, 0xd2),
    );
    let cross_b = mulps(
        shufps(angular, angular, 0xd2),
        shufps(relative, relative, 0xc9),
    );
    let linear = load_vec(e, motion + 0xd0);
    let velocity = addps(subps(cross_a, cross_b), linear);
    let squares = mulps(velocity, velocity);
    let speed = (squares[2] + (squares[1] + squares[0])).sqrt();
    let t = if threshold < speed {
        let half = e.global::<f32>(HALF);
        let three = e.global::<f32>(THREE);
        let sum = squares[2] + (squares[1] + squares[0]);
        let inverse = refined_rsqrt(sum, half, three);
        let unit = mulps(splat(inverse), velocity);
        let product = mulps(unit, direction);
        sum_xyz(product)
    } else {
        0.0
    };
    let mass = e.call(GET_MASS, &args![motion]).st0 as f32;
    let base = e.global::<f32>(IMPULSE_BASE);
    let limit = e.global::<f32>(IMPULSE_LIMIT);
    let mut factor = ((f64::from(base) - f64::from(speed) * f64::from(t)) * f64::from(mass)) as f32;
    if limit < factor {
        factor = limit;
    }
    let negative_limit = -limit;
    if negative_limit > factor {
        factor = negative_limit;
    }
    let impulse = mulps(splat(factor), direction);
    e.call(ACTIVATE, &args![hk_body]);
    e.with_stack(0x20, |e, block| {
        store_vec(e, block.addr(), impulse);
        store_vec(e, block.addr() + 0x10, point);
        e.vcall(motion, 0x50, &args![block, block.addr() + 0x10]);
    });
}

// Translated from 00ca1c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca1c50 - the FS:[0] exception-unwinding frame and the stack-cookie check are not modeled.
/// `bhkRagdollPenetrationUtil::castRay` (Xbox PDB): casts the segment `from -> to` in
/// `world` (filter info `(iGroup << 16) | 0x1d`, shape-collection filter off) and then
/// back `to -> from` (filter on), stopping at the first hit whose collidable has type
/// byte 1 (`+0x18`), an owner (`+0x10` signed offset) and a collision layer (`owner +
/// 0x2C` & 0x7F) other than 8 and 0x1D. On a hit writes the hit point
/// `(1 - t) * start + t * end` (four lanes, `t` the hit fraction copied through the x87
/// stack) to `hit_point` and the hit normal to `hit_normal`. Writes whether there was a
/// hit to the byte at `result` and returns `result`.
#[allow(clippy::too_many_arguments)]
pub fn bhk_ragdoll_penetration_util_cast_ray(
    e: &mut Engine,
    this: Ptr<BhkRagdollPenetrationUtil>,
    result: Ptr,
    world: u32,
    from: Ptr,
    to: Ptr,
    hit_point: Ptr,
    hit_normal: Ptr,
) -> Ptr {
    const COLLECTOR_SIZE: u32 = 0x31c;
    e.with_stack(COLLECTOR_SIZE, |e, collector| {
        e.with_stack(0x30, |e, ray| {
            let mut found = false;
            e.mem.set_u8(ray.addr() + 0x20, 0);
            e.mem.set_u32(ray.addr() + 0x24, 0);
            e.call(COLLECTOR_CONSTRUCT, &args![collector]);
            e.mem.set_f32(collector.addr() + 4, 1.0);
            let group = e.get(this, BhkRagdollPenetrationUtil::iGroup);
            let filter = ((group as u32) << 16) | 0x1d;
            let one = load_vec(e, ONES_VECTOR);
            for pass in 0..2 {
                let (start, end) = if pass == 0 { (from, to) } else { (to, from) };
                let start_vec = load_vec(e, start.addr());
                let end_vec = load_vec(e, end.addr());
                store_vec(e, ray.addr(), start_vec);
                store_vec(e, ray.addr() + 0x10, end_vec);
                e.mem.set_u32(ray.addr() + 0x24, filter);
                e.mem.set_u8(ray.addr() + 0x20, pass as u8);
                e.mem.set_u32(collector.addr() + 0x14, 0);
                e.call(WORLD_CAST_RAY, &args![world, ray, collector]);
                let count = e.mem.i32(collector.addr() + 0x14);
                let mut hit = 0u32;
                while (hit as i32) < count && !found {
                    let entry = e.mem.u32(collector.addr() + 0x10) + 0x60 * hit;
                    let fraction = x87_quiet(e.mem.f32(entry + 0x10));
                    let normal = load_vec(e, entry);
                    let collidable = e.mem.u32(entry + 0x50);
                    if e.mem.u8(collidable + 0x18) == 1 {
                        let owner =
                            collidable.wrapping_add(e.mem.i8(collidable + 0x10) as i32 as u32);
                        if owner != 0 {
                            let layer = e.mem.u32(owner + 0x2c) & 0x7f;
                            if layer != 8 && layer != 0x1d {
                                let t = splat(fraction);
                                let weight = subps(one, t);
                                let point = addps(mulps(weight, start_vec), mulps(t, end_vec));
                                store_vec(e, hit_point.addr(), point);
                                store_vec(e, hit_normal.addr(), normal);
                                found = true;
                            }
                        }
                    }
                    hit += 1;
                }
                if found {
                    break;
                }
                if pass == 0 {
                    e.mem.set_f32(collector.addr() + 4, 1.0);
                }
            }
            e.mem.set_u8(result.addr(), u8::from(found));
            e.call(COLLECTOR_DESTRUCT, &args![collector]);
            result
        })
    })
}

// Translated from 00ca1f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks a scene-graph subtree handing physics objects to `fn_00ca11e0` (no PDB name): for
/// a non-null `node`, when its `+0x1C` object casts to `01267e64`, that object's `+0x10`
/// is set and casts to `012681c0` giving an object whose virtual slot `0x94` answers
/// non-zero, calls `fn_00ca11e0` on it with the two lists; then, if the node casts to the
/// node type `011f4428`, recurses into its children (array `+0xA0`, count `u16` at
/// `+0xA6`).
pub fn fn_00ca1f00(e: &mut Engine, this: Ptr, node: u32, list_a: u32, list_b: u32) {
    if node == 0 {
        return;
    }
    let owner = e.mem.u32(node + 0x1c);
    if owner != 0 {
        let casted = dynamic_cast(e, owner, NODE_OWNER_TYPE);
        if casted != 0 {
            let holder = e.mem.u32(casted + 0x10);
            if holder != 0 {
                let body = dynamic_cast(e, holder, NODE_BODY_TYPE);
                if body != 0 && e.vcall(body, 0x94, &args![]).u32() != 0 {
                    fn_00ca11e0(e, this, body, list_a, list_b);
                }
            }
        }
    }
    let as_node = dynamic_cast(e, node, NODE_CAST_TYPE);
    if as_node != 0 && e.mem.u16(as_node + 0xa6) != 0 {
        let mut index = 0;
        loop {
            let child = e.mem.u32(e.mem.u32(as_node + 0xa0) + 4 * index);
            fn_00ca1f00(e, this, child, list_a, list_b);
            index += 1;
            if index >= u32::from(e.mem.u16(as_node + 0xa6)) {
                break;
            }
        }
    }
}

// Translated from 00ca1fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties an `hkArray` of 0x30-byte items (data, size, capacity-and-flags; no PDB name):
/// runs `00d1fa10` on every item, sets the size to 0, frees the buffer
/// (`(capacity & 0x3fffffff) * 0x30` bytes through the allocator's slot `0x10`) unless the
/// capacity word has bit 31 set (not owned), and resets the array to no buffer with
/// capacity word `0x80000000`.
pub fn fn_00ca1fb0(e: &mut Engine, this: Ptr) {
    let heap = allocator(e);
    let array = this.addr();
    let size = e.mem.i32(array + 4);
    let data = e.mem.u32(array);
    if size > 0 {
        for item in 0..size as u32 {
            e.call(OUTPUT_ELEMENT_DESTRUCT, &args![data + 0x30 * item]);
        }
    }
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if capacity & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        e.vcall(heap, 0x10, &args![data, (capacity & 0x3fff_ffff) * 0x30]);
    }
    e.mem.set_u32(array + 8, 0x8000_0000);
    e.mem.set_u32(array, 0);
}

// Translated from 00ca2020 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca2020 - the FS:[0] exception-unwinding frame is not modeled.
/// Destroys one `ConstrainedSystem` (no PDB name), with the allocation-scope word at
/// `TLS + 0x2b4` set to 0x10 meanwhile: deletes the skeleton (`+8`) and the bounding
/// phantom (`+0xC`) through their virtual destructor (argument 1), releases the ragdoll
/// instance (`+4`) and the detector (`+0x10`), then empties the penetration output array
/// (`+0x14`, `fn_00ca1fb0`) after restoring the scope word.
pub fn fn_00ca2020(e: &mut Engine, this: Ptr<ConstrainedSystem>) {
    let tls = e.tls() + 0x2b4;
    let saved = e.mem.u32(tls);
    e.mem.set_u32(tls, 0x10);
    let skeleton = e.get(this, ConstrainedSystem::pSkel).addr();
    if skeleton != 0 {
        e.vcall(skeleton, 0, &args![1u32]);
    }
    let instance = e.get(this, ConstrainedSystem::pRagdollInstance);
    if !instance.is_null() {
        e.call(REMOVE_REFERENCE, &args![instance]);
    }
    let detector = e.get(this, ConstrainedSystem::pDetectionUtil);
    if !detector.is_null() {
        e.call(REMOVE_REFERENCE, &args![detector]);
    }
    let phantom = e.get(this, ConstrainedSystem::pBoundingPhantom).addr();
    if phantom != 0 {
        e.vcall(phantom, 0, &args![1u32]);
    }
    e.mem.set_u32(tls, saved);
    fn_00ca1fb0(e, Ptr::new(this.addr() + 0x14));
}

// Translated from 00ca20e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollPenetrationUtil::Clear` (Xbox PDB): destroys every active system
/// (`fn_00ca2020`, then frees the 0x24-byte block), frees the array buffer through the
/// array vtable's slot `8`, empties the array and clears the timer limit.
pub fn bhk_ragdoll_penetration_util_clear(e: &mut Engine, this: Ptr<BhkRagdollPenetrationUtil>) {
    let mut index = 0;
    while index < e.get(this, BhkRagdollPenetrationUtil::iActiveSystemsSize) {
        let system = system_at(e, this, index);
        if !system.is_null() {
            fn_00ca2020(e, system);
            e.call(HEAP_FREE, &args![HEAP_SINGLETON, system]);
        }
        index += 1;
    }
    let buffer = e.get(this, BhkRagdollPenetrationUtil::pActiveSystems);
    if !buffer.is_null() {
        let array = this.addr() + 4;
        e.vcall(array, 8, &args![buffer]);
        e.set(this, BhkRagdollPenetrationUtil::pActiveSystems, Ptr::NULL);
        e.set(this, BhkRagdollPenetrationUtil::iActiveSystemsReserved, 0);
        e.set(this, BhkRagdollPenetrationUtil::iActiveSystemsSize, 0);
    }
    e.set(this, BhkRagdollPenetrationUtil::timerLimit, 0);
}

// Translated from 00ca2140 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca2140 - the FS:[0] exception-unwinding frame is not modeled.
/// Destructor body of `bhkRagdollPenetrationUtil` (no PDB name): sets the util vtable,
/// `Clear`s, runs the `ActiveSystems` array destructor inline (vtable, free the buffer
/// through the array vtable's slot `8` at its static address `010c7318`, clear it) and
/// finally installs the `hkaRagdollRaycastInterface` vtable.
pub fn fn_00ca2140(e: &mut Engine, this: Ptr<BhkRagdollPenetrationUtil>) {
    e.set(this, BhkRagdollPenetrationUtil::vtable, UTIL_VTABLE);
    bhk_ragdoll_penetration_util_clear(e, this);
    let array = this.addr() + 4;
    let buffer = e.get(this, BhkRagdollPenetrationUtil::pActiveSystems);
    e.mem.set_u32(array, SYSTEM_ARRAY_VTABLE);
    if !buffer.is_null() {
        let free = e.mem.u32(SYSTEM_ARRAY_VTABLE + 8);
        e.call(free, &args![array, buffer]);
        e.set(this, BhkRagdollPenetrationUtil::pActiveSystems, Ptr::NULL);
        e.set(this, BhkRagdollPenetrationUtil::iActiveSystemsReserved, 0);
        e.set(this, BhkRagdollPenetrationUtil::iActiveSystemsSize, 0);
    }
    e.set(
        this,
        BhkRagdollPenetrationUtil::vtable,
        RAYCAST_INTERFACE_VTABLE,
    );
}

// Translated from 00ca21c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca21c0 - the FS:[0] exception-unwinding frame and the stack-cookie check are not modeled.
/// `bhkRagdollPenetrationUtil::CreateSkeletonForRagdoll` (Xbox PDB): builds the
/// `hkaSkeleton` (0x48 bytes, reference count 1, vtable `010c4dec`, RTTI-confirmed) of a
/// ragdoll from its rigid bodies.
///
/// `bodies` is the hkArray of rigid bodies (data, size), `parents` the hkArray whose item
/// `i - 1` names (`+0x18`) the entity that body `i` hangs from, and `extras` a
/// `BSSimpleArray` of records (record `+8` is a wrapper whose Havok shape `+0x10` gives an
/// extra "Nub" bone). The skeleton gets `bodies.size + extras.size` bones, a parent index
/// per bone (the position in `bodies` of the parent entity, -1 for the root and for
/// extras it is the body each extra belongs to), a reference pose per bone (the body
/// transform relative to its parent: translation, rotation product and inverse scale, via
/// `hkQsTransform::setFromTransformNoScale` and `hkQuaternion::setMul`) and a name per
/// bone (the rigid body's name, a copy; extras are `"<name> Nub"`, `"No Name"` for
/// unnamed shapes; a body without a name logs "Rigid Body name could not be found"
/// through the Havok error object). Each extra gets a translation computed from its shape
/// (type 5: the farther end point offset by the radius along the axis; types 4, 6, 1:
/// `2 * radius` in the first lane) and the identity rotation with unit scale. Writes the
/// body-to-map entry for each extra. The skeleton's name is copied from the first bone.
/// Returns the skeleton.
pub fn bhk_ragdoll_penetration_util_create_skeleton_for_ragdoll(
    e: &mut Engine,
    _this: Ptr,
    bodies: Ptr,
    parents: Ptr,
    extras: Ptr,
) -> Ptr {
    let bodies = bodies.addr();
    let parents = parents.addr();
    let extras = extras.addr();
    let heap = allocator(e);
    let skeleton = e.vcall(heap, 4, &args![0x48u32]).u32();
    e.mem.set_u16(skeleton + 4, 0x48);
    e.mem.set_u16(skeleton + 6, 1);
    e.mem.set_u32(skeleton, SKELETON_VTABLE);
    e.call(STRING_PTR_CONSTRUCT, &args![skeleton + 8]);
    for (data, size, capacity) in [
        (0xc, 0x10, 0x14),
        (0x18, 0x1c, 0x20),
        (0x24, 0x28, 0x2c),
        (0x30, 0x34, 0x38),
        (0x3c, 0x40, 0x44),
    ] {
        e.mem.set_u32(skeleton + data, 0);
        e.mem.set_u32(skeleton + size, 0);
        e.mem.set_u32(skeleton + capacity, 0x8000_0000);
    }
    let body_count = e.mem.i32(bodies + 4);
    let extra_count = e.mem.u32(extras + 8) as i32;
    let total = body_count.wrapping_add(extra_count);
    reserve(e, skeleton + 0xc, total, 2, true);
    e.mem.set_u32(skeleton + 0x10, total as u32);
    reserve(e, skeleton + 0x24, total, 0x30, true);
    e.mem.set_u32(skeleton + 0x28, total as u32);
    reserve(e, skeleton + 0x18, total, 8, true);
    let bones_size = e.mem.i32(skeleton + 0x1c);
    let bones_data = e.mem.u32(skeleton + 0x18);
    if bones_size - total > 0 {
        for bone in 0..(bones_size - total) {
            e.call(
                STRING_PTR_DESTRUCT,
                &args![bones_data + 8 * (total + bone) as u32],
            );
        }
    }
    let first_new = e.mem.i32(skeleton + 0x1c);
    let bones_data = e.mem.u32(skeleton + 0x18);
    if total - first_new > 0 {
        for bone in 0..(total - first_new) {
            let element = bones_data + 8 * (first_new + bone) as u32;
            e.call(STRING_PTR_CONSTRUCT, &args![element]);
            e.mem.set_u8(element + 4, 0);
        }
    }
    e.mem.set_u32(skeleton + 0x1c, total as u32);
    // The map from extra record to the body index it belongs to: an hkArray<int> of
    // `extra_count` items from the allocator's sized-allocation slot 0xC.
    let extra_total = e.mem.u32(extras + 8);
    let heap = allocator(e);
    let map_data;
    let mut map_capacity;
    if extra_total == 0 {
        map_data = 0;
        map_capacity = 0x8000_0000u32;
    } else {
        let granted = e.with_stack(4, |e, request| {
            e.mem.set_u32(request.addr(), extra_total * 4);
            let block = e.vcall(heap, 0xc, &args![request]).u32();
            (block, e.mem.i32(request.addr()))
        });
        map_data = granted.0;
        let items = (granted.1 + ((granted.1 >> 31) & 3)) >> 2;
        map_capacity = items as u32;
        if items == 0 {
            map_capacity = 0x8000_0000;
        }
    }
    for body in 0..body_count.max(0) as u32 {
        let mut parent: i32 = -1;
        if body != 0 {
            let previous = e.mem.u32(e.mem.u32(parents) + 4 * (body - 1));
            let wanted = e.mem.u32(previous + 0x18);
            let mut search = 0u32;
            while search < body {
                if e.mem.u32(e.mem.u32(bodies) + 4 * search) == wanted {
                    parent = search as i32;
                    break;
                }
                search += 1;
            }
        }
        let parent_indices = e.mem.u32(skeleton + 0xc);
        e.mem.set_u16(parent_indices + 2 * body, parent as u16);
        let rigid_body = e.mem.u32(e.mem.u32(bodies) + 4 * body);
        let mut record = 0u32;
        while record < e.mem.u32(extras + 8) {
            let entry = e.mem.u32(e.mem.u32(extras + 4) + 4 * record);
            if rigid_body == e.mem.u32(entry + 8) {
                e.mem.set_u32(map_data + 4 * record, body);
            }
            record += 1;
        }
        let pose = e.mem.u32(skeleton + 0x24) + 0x30 * body;
        if parent >= 0 {
            let parent_body = e.mem.u32(e.mem.u32(bodies) + 4 * parent as u32);
            relative_transform(e, pose, rigid_body, parent_body);
        } else {
            e.call(SET_FROM_TRANSFORM_NO_SCALE, &args![pose, rigid_body + 0xf0]);
        }
        let name_pointer = e.mem.u32(rigid_body + 0x74) & !1;
        let bones_data = e.mem.u32(skeleton + 0x18);
        let target = bones_data + 8 * body;
        let text = if name_pointer != 0 {
            e.call(STRING_DUPLICATE, &args![name_pointer]).u32()
        } else {
            report_missing_name(e);
            0
        };
        e.call(STRING_PTR_ASSIGN, &args![target, text]);
        let bones_data = e.mem.u32(skeleton + 0x18);
        e.mem.set_u8(bones_data + 8 * body + 4, u8::from(body != 0));
    }
    add_extra_bones(e, skeleton, body_count as u32, extras, map_data);
    if e.mem.u32(skeleton + 0x1c) == 0 {
        e.call(STRING_PTR_ASSIGN, &args![skeleton + 8, 0u32]);
    } else {
        let first_bone = e.mem.u32(skeleton + 0x18);
        e.call(STRING_PTR_ASSIGN_COPY, &args![skeleton + 8, first_bone]);
    }
    let heap = allocator(e);
    if map_capacity & 0x8000_0000 == 0 {
        e.vcall(
            heap,
            0x10,
            &args![map_data, (map_capacity & 0x3fff_ffff) * 4],
        );
    }
    Ptr::new(skeleton)
}

/// Writes the pose of `rigid_body` relative to `parent_body` (both Havok rigid bodies,
/// transforms at `+0xF0`) into the 0x30-byte `hkQsTransform` at `destination`: the
/// translation is `conj(q_p) (t_c - ...)`-style (computed lane by lane as the code does),
/// the rotation `hkQuaternion::setMul(conj(q_p), q_c)` and the scale `s_c` times the
/// parent's reciprocal scale (`RCPPS` refined once, lane 3 masked off).
fn relative_transform(e: &mut Engine, destination: u32, rigid_body: u32, parent_body: u32) {
    e.with_stack(0x70, |e, block| {
        let child = block.addr();
        let parent = block.addr() + 0x30;
        let conjugate = block.addr() + 0x60;
        e.call(
            SET_FROM_TRANSFORM_NO_SCALE,
            &args![child, rigid_body + 0xf0],
        );
        e.call(
            SET_FROM_TRANSFORM_NO_SCALE,
            &args![parent, parent_body + 0xf0],
        );
        let minus_half = load_vec(e, MINUS_HALF_VECTOR);
        let q_parent = load_vec(e, parent + 0x10);
        let t_parent = load_vec(e, parent);
        let w = splat(q_parent[3]);
        let h = addps(mulps(w, w), minus_half);
        let dot = sum_xyz(mulps(q_parent, t_parent));
        let along = mulps(q_parent, splat(dot));
        let a = mulps(
            shufps(t_parent, t_parent, 0xc9),
            shufps(q_parent, q_parent, 0xd2),
        );
        let b = mulps(
            shufps(t_parent, t_parent, 0xd2),
            shufps(q_parent, q_parent, 0xc9),
        );
        let mut sum = addps(mulps(t_parent, h), along);
        sum = addps(sum, mulps(subps(a, b), w));
        let doubled = addps(sum, sum);
        let sign_all = load_mask(e, SIGN_MASK_ALL);
        let base = bitwise(doubled, sign_all, |x, m| x ^ m);
        let sign_xyz = load_mask(e, SIGN_MASK_XYZ);
        let conj = bitwise(q_parent, sign_xyz, |x, m| x ^ m);
        // Reciprocal of the parent's scale with lane 3 replaced by the constant's lane 3.
        let s_parent = load_vec(e, parent + 0x20);
        let unit = load_vec(e, UNIT_SCALE_VECTOR);
        let unpacked = [s_parent[2], unit[2], s_parent[3], unit[3]];
        let scale = shufps(s_parent, unpacked, 0xc4);
        let reciprocal = [
            rcp_estimate(scale[0]),
            rcp_estimate(scale[1]),
            rcp_estimate(scale[2]),
            rcp_estimate(scale[3]),
        ];
        let refined = subps(
            addps(reciprocal, reciprocal),
            mulps(mulps(reciprocal, scale), reciprocal),
        );
        let lane_mask = load_mask(e, LANE_MASK_XYZ);
        let inverse_scale = bitwise(refined, lane_mask, |x, m| x & m);
        let w2 = splat(conj[3]);
        let h2 = addps(mulps(w2, w2), minus_half);
        store_vec(e, conjugate, conj);
        let t_child = load_vec(e, child);
        let along2 = mulps(conj, splat(sum_xyz(mulps(conj, t_child))));
        let a2 = mulps(shufps(conj, conj, 0xc9), shufps(t_child, t_child, 0xd2));
        let b2 = mulps(shufps(conj, conj, 0xd2), shufps(t_child, t_child, 0xc9));
        let mut result = addps(mulps(t_child, h2), along2);
        result = addps(result, mulps(subps(a2, b2), w2));
        let result = addps(addps(result, result), base);
        store_vec(e, destination, result);
        e.call(
            QUATERNION_SET_MUL,
            &args![destination + 0x10, conjugate, child + 0x10],
        );
        let s_child = load_vec(e, child + 0x20);
        store_vec(e, destination + 0x20, mulps(s_child, inverse_scale));
    });
}

/// Reports "Rigid Body name could not be found" through the Havok error object (level 1,
/// id `0xabbacc14`, line 721 of `bhkRagdollPenetrationUtil.cpp`) after formatting the text
/// into a 0x200-byte buffer with a text stream.
fn report_missing_name(e: &mut Engine) {
    e.with_stack(0x200, |e, buffer| {
        e.with_stack(0x10, |e, stream| {
            e.call(STREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
            e.call(STREAM_WRITE, &args![stream, NAME_MISSING_TEXT]);
            let reporter = e.global::<u32>(ERROR_OBJECT);
            e.vcall(
                reporter,
                0xc,
                &args![
                    1u32,
                    NAME_MISSING_ID,
                    buffer,
                    SOURCE_FILE_TEXT,
                    NAME_MISSING_LINE
                ],
            );
            e.call(STREAM_DESTRUCT, &args![stream]);
        });
    });
}

/// The second half of `CreateSkeletonForRagdoll`: one extra bone per record of `extras`
/// (record `k`, bone `body_count + k`, parent bone `map[k]`). The record's body
/// (`record + 8`) gives the shape (`+0x10`; records without one are skipped) and the name
/// (`+0x74`, low bit masked). The extra bone is named `"<name> Nub"` (`strncpy_s` of at
/// most 0x28 characters, or `strcpy_s` of `"No Name"`, then `strcat_s`; `strDup`ed into
/// the bone's `hkStringPtr`), locks its translation, takes the parent from `map` and gets
/// the identity rotation, unit scale and a translation from the shape: type 5 uses the
/// longer-reaching of its two end points (`+0x20`, `+0x30`) moved by the unit direction
/// between them times the radius (`+0x10`); types 4, 6 and 1 put `2 * radius` in lane 0;
/// other types leave it zero.
fn add_extra_bones(e: &mut Engine, skeleton: u32, body_count: u32, extras: u32, map_data: u32) {
    let two = e.global::<f64>(TWO_DOUBLE);
    let mut record = 0u32;
    while record < e.mem.u32(extras + 8) {
        let entry = e.mem.u32(e.mem.u32(extras + 4) + 4 * record);
        let body = e.mem.u32(entry + 8);
        let parent = e.mem.u32(map_data + 4 * record);
        let shape = e.mem.u32(body + 0x10);
        if shape != 0 {
            let kind = e.mem.u32(shape + 0xc);
            let mut position = splat(0.0);
            if kind == 5 {
                let end_a = load_vec(e, shape + 0x20);
                let end_b = load_vec(e, shape + 0x30);
                let length_a = pair_length(end_a);
                let length_b = pair_length(end_b);
                let chosen = if length_b < length_a { end_a } else { end_b };
                let radius = x87_quiet(e.mem.f32(shape + 0x10));
                let half = e.global::<f32>(HALF);
                let three = e.global::<f32>(THREE);
                let difference = subps(end_a, end_b);
                let squares = mulps(difference, difference);
                let sum = squares[2] + (squares[1] + squares[0]);
                let inverse = refined_rsqrt(sum, half, three);
                let offset = mulps(mulps(splat(inverse), difference), splat(radius));
                position = addps(chosen, offset);
            }
            if kind == 4 || kind == 6 || kind == 1 {
                let radius = x87_quiet(e.mem.f32(shape + 0x10));
                position[0] = (f64::from(radius) * two) as f32;
            }
            let name = e.mem.u32(body + 0x74);
            e.with_stack(0x30, |e, buffer| {
                if name & !1 != 0 {
                    e.call(STRNCPY_S, &args![buffer, 0x29u32, name & !1, 0x28u32]);
                    e.mem.set_u8(buffer.addr() + 0x28, 0);
                } else {
                    e.call(STRCPY_S, &args![buffer, 0x28u32, NO_NAME_TEXT]);
                }
                e.call(STRCAT_S, &args![buffer, 0x2du32, NUB_SUFFIX]);
                let bones_data = e.mem.u32(skeleton + 0x18);
                let target = bones_data + 8 * (body_count + record);
                let duplicate = e.call(STRING_DUPLICATE, &args![buffer]).u32();
                e.call(STRING_PTR_ASSIGN, &args![target, duplicate]);
            });
            let bones_data = e.mem.u32(skeleton + 0x18);
            e.mem.set_u8(bones_data + 8 * (body_count + record) + 4, 1);
            let parent_indices = e.mem.u32(skeleton + 0xc);
            e.mem
                .set_u16(parent_indices + 2 * (body_count + record), parent as u16);
            let pose = e.mem.u32(skeleton + 0x24) + 0x30 * (body_count + record);
            store_vec(e, pose, splat(0.0));
            let identity = load_vec(e, IDENTITY_QUATERNION);
            store_vec(e, pose + 0x10, identity);
            let one = e.global::<f32>(ONE);
            store_vec(e, pose + 0x20, splat(one));
            let pose = e.mem.u32(skeleton + 0x24) + 0x30 * (body_count + record);
            store_vec(e, pose, position);
        }
        record += 1;
    }
}

/// Length of a vector as the code computes it for the capsule end points: the pair sums
/// `(s2 + s0) + (s3 + s1)` of the squares (`SHUFPS 0x4E`, `0xB1`), square-rooted in `f32`.
fn pair_length(v: Vec4) -> f32 {
    let s = mulps(v, v);
    ((s[3] + s[1]) + (s[2] + s[0])).sqrt()
}

// Translated from 00ca2aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkRagdollPenetrationUtil::_scalar_deleting_destructor_` (Xbox PDB): runs the
/// destructor body (`fn_00ca2140`) and, when bit 0 of `flags` is set and `this` is not
/// null, frees the block that begins `byte[this - 1]` bytes below it (aligned allocation).
/// Returns `this`.
pub fn bhk_ragdoll_penetration_util_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BhkRagdollPenetrationUtil>,
    flags: u8,
) -> Ptr<BhkRagdollPenetrationUtil> {
    fn_00ca2140(e, this);
    if flags & 1 != 0 && !this.is_null() {
        let distance = u32::from(e.mem.u8(this.addr() - 1));
        e.call(HEAP_FREE, &args![HEAP_SINGLETON, this.addr() - distance]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca_0ca0, hka_ragdoll_raycast_interface_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00ca_0cd0, fn_00ca0cd0(Ptr<BhkRagdollPenetrationUtil>)),
        entry!(0x00ca_0d40, fn_00ca0d40(Ptr)),
        entry!(0x00ca_0d70, bs_simple_array_constrained_system_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(
            0x00ca_0dc0,
            fn_00ca0dc0(Ptr<BhkRagdollPenetrationUtil>) -> Ptr<BhkRagdollPenetrationUtil>
        ),
        entry!(0x00ca_0e60, fn_00ca0e60(Ptr, Ptr)),
        entry!(0x00ca_0f00, fn_00ca0f00(Ptr, u32, u32)),
        entry!(0x00ca_1080, fn_00ca1080(Ptr, Ptr, Ptr)),
        entry!(0x00ca_11e0, fn_00ca11e0(Ptr, u32, u32, u32)),
        entry!(
            0x00ca_1410,
            bhk_ragdoll_penetration_util_update(Ptr<BhkRagdollPenetrationUtil>)
        ),
        entry!(
            0x00ca_1c50,
            bhk_ragdoll_penetration_util_cast_ray(
                Ptr<BhkRagdollPenetrationUtil>,
                Ptr,
                u32,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
            ) -> Ptr
        ),
        entry!(0x00ca_1f00, fn_00ca1f00(Ptr, u32, u32, u32)),
        entry!(0x00ca_1fb0, fn_00ca1fb0(Ptr)),
        entry!(0x00ca_2020, fn_00ca2020(Ptr<ConstrainedSystem>)),
        entry!(
            0x00ca_20e0,
            bhk_ragdoll_penetration_util_clear(Ptr<BhkRagdollPenetrationUtil>)
        ),
        entry!(0x00ca_2140, fn_00ca2140(Ptr<BhkRagdollPenetrationUtil>)),
        entry!(0x00ca_21c0, bhk_ragdoll_penetration_util_create_skeleton_for_ragdoll(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(
            0x00ca_2aa0,
            bhk_ragdoll_penetration_util_scalar_deleting_destructor(
                Ptr<BhkRagdollPenetrationUtil>,
                u8,
            ) -> Ptr<
                BhkRagdollPenetrationUtil,
            >
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// The addresses of the calls recorded so far, in order.
    fn order(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
    }

    /// The argument words of every logged call to `address`.
    fn args_of(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An object whose vtable (at `vtable`) has the given `(byte offset, target)` slots.
    fn object_with_slots(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) -> u32 {
        let mut table = vec![0u32; 0x40];
        for (offset, target) in slots {
            table[(*offset / 4) as usize] = *target;
        }
        e.put_vtable(vtable, &table);
        let object = e.mem.alloc(0x400);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A fake memory router whose allocator vtable slots are the doubles `0x0900_0010`
    /// (allocate), `..11` (free), `..12` (sized allocate) and `..13` (free with bytes).
    fn install_heap(e: &mut Engine) {
        let allocator = object_with_slots(
            e,
            0x0200_0100,
            &[
                (4, 0x0900_0010),
                (8, 0x0900_0011),
                (0xc, 0x0900_0012),
                (0x10, 0x0900_0013),
            ],
        );
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        e.register_double(0x0900_0010, |e, a| ret(e.mem.alloc(a[1])));
        e.register(0x0900_0011, |_, _| Ret::default());
        e.register_double(0x0900_0012, |e, a| {
            let requested = e.mem.u32(a[1]);
            ret(e.mem.alloc(requested))
        });
        e.register(0x0900_0013, |_, _| Ret::default());
        // `hkArrayUtil::_reserve` / grow: a fresh buffer, capacity word set.
        e.register_double(ARRAY_RESERVE, |e, a| {
            let buffer = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], buffer);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.register_double(ARRAY_GROW, |e, a| {
            let size = e.mem.u32(a[1] + 4);
            let buffer = e.mem.alloc(16 * a[2].max(1));
            for item in 0..size {
                let value = e.mem.u32(e.mem.u32(a[1]) + 4 * item);
                e.mem.set_u32(buffer + 4 * item, value);
            }
            e.mem.set_u32(a[1], buffer);
            e.mem.set_u32(a[1] + 8, 4);
            Ret::default()
        });
    }

    /// An object whose virtual slot `0x20` answers `kind`; its double lives at a fake
    /// address derived from the vtable address.
    fn kind_object(e: &mut Engine, vtable: u32, kind: u32) -> u32 {
        let target = 0x0a00_0000 + (vtable >> 8);
        e.register_double(target, move |_, _| ret(kind));
        object_with_slots(e, vtable, &[(0x20, target)])
    }

    /// The body list `fn_00ca0f00` and `fn_00ca1080` inspect: items at `+0xAC` (0x30-byte
    /// stride, count `u16` at `+0xB0`) answering `first_kinds`, items at `+0xB4` (count at
    /// `+0xB8`) answering `second_kinds`. Vtables are placed from `base`.
    fn body_list(e: &mut Engine, base: u32, first_kinds: &[u32], second_kinds: &[u32]) -> u32 {
        let list = e.mem.alloc(0x100);
        let first = e.mem.alloc(0x30 * first_kinds.len().max(1) as u32);
        for (i, kind) in first_kinds.iter().enumerate() {
            let object = kind_object(e, base + 0x100 * i as u32, *kind);
            let item = e.mem.alloc(0x20);
            e.mem.set_u32(item + 0xc, object);
            e.mem.set_u32(first + 0x30 * i as u32, item);
        }
        e.mem.set_u16(list + 0xb0, first_kinds.len() as u16);
        e.mem.set_u32(list + 0xac, first);
        let second = e.mem.alloc(4 * second_kinds.len().max(1) as u32);
        for (i, kind) in second_kinds.iter().enumerate() {
            let object = kind_object(e, base + 0x1000 + 0x100 * i as u32, *kind);
            let item = e.mem.alloc(0x20);
            e.mem.set_u32(item + 0xc, object);
            e.mem.set_u32(second + 4 * i as u32, item);
        }
        e.mem.set_i32(list + 0xb8, second_kinds.len() as i32);
        e.mem.set_u32(list + 0xb4, second);
        list
    }

    /// A `BSSimpleArray`-like list (vtable, data, size, capacity) whose slot 4 allocates
    /// `n` words and slot `0xC` reallocates `(data, capacity)` by copying.
    fn growing_list(e: &mut Engine, vtable: u32) -> u32 {
        e.register_double(0x0900_0200, |e, a| ret(e.mem.alloc(a[1] * 4)));
        e.register_double(0x0900_0201, |e, a| {
            let old_size = e.mem.u32(a[0] + 8);
            let buffer = e.mem.alloc(a[2] * 4);
            for item in 0..old_size {
                let value = e.mem.u32(a[1] + 4 * item);
                e.mem.set_u32(buffer + 4 * item, value);
            }
            ret(buffer)
        });
        object_with_slots(e, vtable, &[(4, 0x0900_0200), (0xc, 0x0900_0201)])
    }

    fn list_items(e: &Engine, list: u32) -> Vec<u32> {
        (0..e.mem.u32(list + 8))
            .map(|i| e.mem.u32(e.mem.u32(list + 4) + 4 * i))
            .collect()
    }

    fn put_vec(e: &mut Engine, address: u32, v: Vec4) {
        store_vec(e, address, v);
    }

    #[test]
    fn raycast_interface_destructor_installs_its_vtable_and_frees_on_request() {
        let mut e = engine();
        e.register(HEAP_FREE, |_, _| Ret::default());
        let block = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let result = e.call(0x00ca_0ca0, &args![block, 0u8]).u32();
        assert_eq!(result, block);
        assert_eq!(e.mem.u32(block), 0x010c_7304);
        assert!(args_of(&e, HEAP_FREE).is_empty());
        e.call(0x00ca_0ca0, &args![block, 1u8]);
        assert_eq!(args_of(&e, HEAP_FREE), [[HEAP_SINGLETON, block]]);
    }

    #[test]
    fn restarting_detection_marks_every_system_and_resets_the_timer() {
        let mut e = engine();
        let util = e.new_object::<BhkRagdollPenetrationUtil>();
        let systems = e.mem.alloc(8);
        let mut detectors = vec![];
        let mut records = vec![];
        for i in 0..2 {
            let detector = e.mem.alloc(0x40);
            let system = e.new_object::<ConstrainedSystem>();
            e.set(
                system,
                ConstrainedSystem::pDetectionUtil,
                Ptr::new(detector),
            );
            e.mem.set_u32(systems + 4 * i, system.addr());
            detectors.push(detector);
            records.push(system);
        }
        e.set(
            util,
            BhkRagdollPenetrationUtil::pActiveSystems,
            Ptr::new(systems),
        );
        e.set(util, BhkRagdollPenetrationUtil::iActiveSystemsSize, 2);
        e.register(RESET_DETECTION, |_, _| Ret::default());
        e.set_global(BUDGET_SECONDS, 0.5f32);
        e.set_global(MILLISECONDS, 1000.0f64);
        e.set_global(TICK_SCALE, 2.0f32);
        e.register(GET_TIMER, |_, _| 12345u64.into_ret());
        e.register_double(FLOAT_TO_INT64, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as u64).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00ca_0cd0, &args![util]);
        assert_eq!(
            args_of(&e, RESET_DETECTION),
            [[detectors[0]], [detectors[1]]]
        );
        assert!(records
            .iter()
            .all(|s| e.get(*s, ConstrainedSystem::bPenetrating)));
        assert_eq!(e.get(util, BhkRagdollPenetrationUtil::timerStart), 12345);
        // 0.5 * 1000.0 = 500.0 seconds-scaled, times the tick scale 2.0.
        assert_eq!(e.get(util, BhkRagdollPenetrationUtil::timerLimit), 1000);
    }

    #[test]
    fn the_array_destructor_frees_the_buffer_through_the_static_slot() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, buffer);
        e.mem.set_u32(array + 8, 3);
        e.mem.set_u32(array + 0xc, 4);
        e.mem.set_u32(SYSTEM_ARRAY_VTABLE + 8, 0x0900_0001);
        e.register(0x0900_0001, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ca_0d40, &args![array]);
        assert_eq!(args_of(&e, 0x0900_0001), [[array, buffer]]);
        assert_eq!(e.mem.u32(array), SYSTEM_ARRAY_VTABLE);
        assert_eq!(
            [
                e.mem.u32(array + 4),
                e.mem.u32(array + 8),
                e.mem.u32(array + 0xc)
            ],
            [0, 0, 0]
        );
        // Nothing to free the second time.
        e.call_log = Some(vec![]);
        e.call(0x00ca_0d40, &args![array]);
        assert!(args_of(&e, 0x0900_0001).is_empty());
    }

    #[test]
    fn the_array_deleting_destructor_also_frees_itself_on_request() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        e.register(HEAP_FREE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ca_0d70, &args![array, 0u8]).u32(), array);
        assert!(args_of(&e, HEAP_FREE).is_empty());
        e.call(0x00ca_0d70, &args![array, 1u8]);
        assert_eq!(args_of(&e, HEAP_FREE), [[HEAP_SINGLETON, array]]);
        assert_eq!(e.mem.u32(array), SYSTEM_ARRAY_VTABLE);
    }

    #[test]
    fn construction_empties_the_util_and_keeps_its_group() {
        let mut e = engine();
        let util = e.new_object::<BhkRagdollPenetrationUtil>();
        for offset in (0..0x30).step_by(4) {
            e.mem.set_u32(util.addr() + offset, 0xaaaa_aaaa);
        }
        e.mem.set_u32(util.addr() + 8, 0);
        let result = e.call(0x00ca_0dc0, &args![util]).ptr::<()>();
        assert_eq!(result.addr(), util.addr());
        assert_eq!(e.mem.u32(util.addr()), UTIL_VTABLE);
        assert_eq!(e.mem.u32(util.addr() + 4), SYSTEM_ARRAY_VTABLE);
        assert_eq!(
            e.get(util, BhkRagdollPenetrationUtil::pActiveSystems)
                .addr(),
            0
        );
        assert_eq!(
            e.get(util, BhkRagdollPenetrationUtil::iActiveSystemsSize),
            0
        );
        assert_eq!(
            e.get(util, BhkRagdollPenetrationUtil::iActiveSystemsReserved),
            0
        );
        assert_eq!(
            e.get(util, BhkRagdollPenetrationUtil::iGroup),
            0xaaaa_aaaau32 as i32
        );
        assert!(!e.get(
            util,
            BhkRagdollPenetrationUtil::bAttemptToInitializeNextUpdate
        ));
        assert_eq!(e.get(util, BhkRagdollPenetrationUtil::uiInitAttempts), 0);
        assert_eq!(e.get(util, BhkRagdollPenetrationUtil::timerStart), 0);
        assert_eq!(e.get(util, BhkRagdollPenetrationUtil::timerLimit), 0);
    }

    /// A source array of words and a destination array with allocator slots `4` / `8`.
    fn copy_arrays(e: &mut Engine, words: &[u32]) -> (u32, u32) {
        let source = e.mem.alloc(0x10);
        let data = e.mem.alloc(4 * words.len().max(1) as u32);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, *word);
        }
        e.mem.set_u32(source + 4, data);
        e.mem.set_u32(source + 8, words.len() as u32);
        e.register_double(0x0900_0400, |e, a| ret(e.mem.alloc(a[1] * 4)));
        e.register(0x0900_0401, |_, _| Ret::default());
        let target = object_with_slots(e, 0x0200_0600, &[(4, 0x0900_0400), (8, 0x0900_0401)]);
        (source, target)
    }

    #[test]
    fn copying_an_array_resizes_the_target_and_copies_the_words() {
        // Into an empty target: allocated through slot 4.
        let mut e = engine();
        let (source, target) = copy_arrays(&mut e, &[7, 8, 9]);
        e.call_log = Some(vec![]);
        e.call(0x00ca_0e60, &args![source, target]);
        assert_eq!(list_items(&e, target), [7, 8, 9]);
        assert_eq!(e.mem.u32(target + 0xc), 3);
        assert_eq!(args_of(&e, 0x0900_0400), [[target, 3]]);
        // Into a smaller non-empty target: reallocated through 00cb9ce0(count, size).
        e.register_double(SIMPLE_ARRAY_REALLOC, |e, a| {
            let buffer = e.mem.alloc(a[1] * 4);
            e.mem.set_u32(a[0] + 4, buffer);
            Ret::default()
        });
        let (source, target) = copy_arrays(&mut e, &[1, 2, 3, 4]);
        e.mem.set_u32(target + 4, 0x1000_0000);
        e.mem.set_u32(target + 8, 2);
        e.mem.set_u32(target + 0xc, 2);
        e.call_log = Some(vec![]);
        e.call(0x00ca_0e60, &args![source, target]);
        assert_eq!(args_of(&e, SIMPLE_ARRAY_REALLOC), [[target, 4, 2]]);
        assert_eq!(list_items(&e, target), [1, 2, 3, 4]);
        assert_eq!(e.mem.u32(target + 0xc), 4);
        // A much smaller source trims the capacity (count <= capacity / 4).
        let (source, target) = copy_arrays(&mut e, &[5]);
        let big = e.mem.alloc(0x40);
        e.mem.set_u32(target + 4, big);
        e.mem.set_u32(target + 8, 8);
        e.mem.set_u32(target + 0xc, 16);
        e.call_log = Some(vec![]);
        e.call(0x00ca_0e60, &args![source, target]);
        assert_eq!(args_of(&e, SIMPLE_ARRAY_REALLOC), [[target, 1, 1]]);
        assert_eq!(e.mem.u32(target + 8), 1);
        assert_eq!(e.mem.u32(target + 0xc), 1);
        assert_eq!(e.mem.u32(e.mem.u32(target + 4)), 5);
        // A slightly smaller source only lowers the size.
        let (source, target) = copy_arrays(&mut e, &[5, 6, 7]);
        let medium = e.mem.alloc(0x40);
        e.mem.set_u32(target + 4, medium);
        e.mem.set_u32(target + 8, 5);
        e.mem.set_u32(target + 0xc, 6);
        e.call_log = Some(vec![]);
        e.call(0x00ca_0e60, &args![source, target]);
        assert!(args_of(&e, SIMPLE_ARRAY_REALLOC).is_empty());
        assert_eq!(e.mem.u32(target + 8), 3);
        assert_eq!(list_items(&e, target), [5, 6, 7]);
        // An empty source frees the target's buffer.
        let (source, target) = copy_arrays(&mut e, &[]);
        e.mem.set_u32(target + 4, big);
        e.mem.set_u32(target + 8, 2);
        e.mem.set_u32(target + 0xc, 2);
        e.call_log = Some(vec![]);
        e.call(0x00ca_0e60, &args![source, target]);
        assert_eq!(args_of(&e, 0x0900_0401), [[target, big]]);
        assert_eq!(
            [
                e.mem.u32(target + 4),
                e.mem.u32(target + 8),
                e.mem.u32(target + 0xc)
            ],
            [0, 0, 0]
        );
    }

    /// A node whose `+0x1C` owner casts to a body list built from the two kind lists.
    fn collectable_node(
        e: &mut Engine,
        base: u32,
        first_kinds: &[u32],
        second_kinds: &[u32],
    ) -> (u32, u32) {
        let owner = e.mem.alloc(0x20);
        let casted = e.mem.alloc(0x20);
        let holder = e.mem.alloc(0x20);
        let bodies = body_list(e, base, first_kinds, second_kinds);
        e.mem.set_u32(casted + 0x10, holder);
        e.mem.set_u32(holder + 8, bodies);
        let node = object_with_slots(e, base + 0x8000, &[(0xc, 0x0900_0100)]);
        e.mem.set_u32(node + 0x1c, owner);
        e.register(0x0900_0100, |_, _| ret(0));
        e.register_double(DYNAMIC_CAST, move |_, a| {
            assert_eq!(a[1], BODY_LIST_TYPE);
            ret(if a[0] == owner { casted } else { 0 })
        });
        (node, owner)
    }

    #[test]
    fn nodes_with_only_kind_b_items_and_another_kind_elsewhere_are_collected() {
        let cases: [(&[u32], &[u32], bool); 4] = [
            (&[0xb, 0xb], &[7], true),
            (&[0xb, 0xb], &[0xb], false),
            (&[0xb, 7], &[7], false),
            (&[], &[9, 0xb], true),
        ];
        for (first, second, expected) in cases {
            let mut e = engine();
            let (node, _) = collectable_node(&mut e, 0x0300_0000, first, second);
            let list = growing_list(&mut e, 0x0300_f000);
            e.call(0x00ca_0f00, &args![0x1234u32, node, list]);
            assert_eq!(
                list_items(&e, list),
                if expected { vec![node] } else { vec![] }
            );
        }
    }

    #[test]
    fn collecting_visits_the_children_and_grows_a_full_list() {
        let mut e = engine();
        let (node, _) = collectable_node(&mut e, 0x0300_0000, &[0xb], &[7]);
        let child = object_with_slots(&mut e, 0x0310_0000, &[(0xc, 0x0900_0101)]);
        e.register(0x0900_0101, |_, _| ret(0));
        let children = e.mem.alloc(4);
        e.mem.set_u32(children, child);
        let as_node = e.mem.alloc(0x100);
        e.mem.set_u16(as_node + 0xa6, 1);
        e.mem.set_u32(as_node + 0xa0, children);
        e.register_double(0x0900_0100, move |_, _| ret(as_node));
        let list = growing_list(&mut e, 0x0300_f000);
        // A full list of four grows through 0042fc00(capacity, size).
        let data = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u32(data + 4 * i, 0x100 + i);
        }
        e.mem.set_u32(list + 4, data);
        e.mem.set_u32(list + 8, 4);
        e.mem.set_u32(list + 0xc, 4);
        e.register_double(SIMPLE_ARRAY_RESIZE, |e, a| {
            let buffer = e.mem.alloc(a[1] * 4);
            for i in 0..a[2] {
                let value = e.mem.u32(e.mem.u32(a[0] + 4) + 4 * i);
                e.mem.set_u32(buffer + 4 * i, value);
            }
            e.mem.set_u32(a[0] + 4, buffer);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00ca_0f00, &args![0x1234u32, node, list]);
        assert_eq!(args_of(&e, SIMPLE_ARRAY_RESIZE), [[list, 8, 4]]);
        assert_eq!(e.mem.u32(list + 0xc), 8);
        assert_eq!(list_items(&e, list), [0x100, 0x101, 0x102, 0x103, node]);
        assert_eq!(args_of(&e, 0x0900_0101), [[child]]);
        // A null node does nothing.
        e.call_log = Some(vec![]);
        e.call(0x00ca_0f00, &args![0x1234u32, 0u32, list]);
        assert_eq!(order(&e).len(), 1);
    }

    #[test]
    fn the_filter_pushes_the_second_field_of_entries_that_fail_both_tests() {
        let mut e = engine();
        // Entry kinds: (first list kinds, second list kinds, pushed).
        let entries: [(&[u32], &[u32], bool); 4] = [
            (&[7], &[0xb], true),
            (&[0xb], &[0xb], false),
            (&[7], &[5], false),
            (&[], &[], false),
        ];
        let data = e.mem.alloc(16);
        for (i, (first, second, _)) in entries.iter().enumerate() {
            let entry = body_list(&mut e, 0x0300_0000 + 0x10000 * i as u32, first, second);
            e.mem.set_u32(entry + 0xc, 0x5000 + i as u32);
            e.mem.set_u32(data + 4 * i as u32, entry);
        }
        let source = e.mem.alloc(16);
        e.mem.set_u32(source, data);
        e.mem.set_i32(source + 4, 4);
        let list = growing_list(&mut e, 0x0300_f000);
        e.call(0x00ca_1080, &args![0x1234u32, source, list]);
        assert_eq!(list_items(&e, list), [0x5000]);
        // Enough entries to outgrow the first allocation of four (reallocation, slot 0xC).
        let many = e.mem.alloc(4 * 6);
        for i in 0..6u32 {
            let entry = body_list(&mut e, 0x0400_0000 + 0x10000 * i, &[7], &[0xb]);
            e.mem.set_u32(entry + 0xc, 0x6000 + i);
            e.mem.set_u32(many + 4 * i, entry);
        }
        e.mem.set_u32(source, many);
        e.mem.set_i32(source + 4, 6);
        let second_list = growing_list(&mut e, 0x0300_f400);
        e.call(0x00ca_1080, &args![0x1234u32, source, second_list]);
        assert_eq!(
            list_items(&e, second_list),
            [0x6000, 0x6001, 0x6002, 0x6003, 0x6004, 0x6005]
        );
        assert_eq!(e.mem.u32(second_list + 0xc), 8);
    }

    /// The scene for `fn_00ca11e0`: a body with the given list items and property.
    struct Gather {
        body: u32,
        hk_object: u32,
        list_a: u32,
        list_b: u32,
    }

    fn gather_scene(e: &mut Engine, property: u32, owner_kinds: &[u32]) -> Gather {
        install_heap(e);
        let hk_object = e.mem.alloc(0x20);
        let body = e.mem.alloc(0x40);
        e.mem.set_u32(body + 8, hk_object);
        let mut next = 0;
        let mut head = 0;
        for (i, kind) in owner_kinds.iter().enumerate().rev() {
            let owner = kind_object(e, 0x0300_0000 + 0x100 * i as u32, *kind);
            let wrapped = e.mem.alloc(0x20);
            e.mem.set_u32(wrapped + 0xc, owner);
            let item = object_with_slots(e, 0x0301_0000 + 0x100 * i as u32, &[(8, 0x0900_0500)]);
            e.mem.set_u32(item + 8, wrapped);
            if i == 0 {
                head = item;
            } else {
                // The first list node is embedded in the body at +0x14.
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, item);
                e.mem.set_u32(node + 4, next);
                next = node;
            }
        }
        e.mem.set_u32(body + 0x14, head);
        e.mem.set_u32(body + 0x18, next);
        e.register_double(0x0900_0500, |_, _| ret(LISTED_ITEM_RTTI));
        e.register_double(GET_PROPERTY, move |e, a| {
            e.mem.set_u32(a[0], property);
            e.mem.set_u32(a[0] + 4, 0);
            ret(a[0])
        });
        e.register_double(DYNAMIC_CAST, |_, a| {
            assert_eq!(a[1], PROPERTY_CAST_TYPE);
            ret(a[0])
        });
        let list_a = e.mem.alloc(12);
        let list_b = e.mem.alloc(12);
        e.mem.set_u32(list_a + 8, 0x8000_0000);
        e.mem.set_u32(list_b + 8, 0x8000_0000);
        Gather {
            body,
            hk_object,
            list_a,
            list_b,
        }
    }

    fn hk_items(e: &Engine, array: u32) -> Vec<u32> {
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u32(e.mem.u32(array) + 4 * i))
            .collect()
    }

    #[test]
    fn gathering_collects_the_items_and_the_body() {
        let mut e = engine();
        let scene = gather_scene(&mut e, 0, &[3, 4]);
        e.call_log = Some(vec![]);
        e.call(
            0x00ca_11e0,
            &args![0x1234u32, scene.body, scene.list_a, scene.list_b],
        );
        assert_eq!(hk_items(&e, scene.list_a), [scene.hk_object]);
        assert_eq!(hk_items(&e, scene.list_b).len(), 2);
        // The local array was freed with its 16 bytes (capacity 4).
        assert_eq!(args_of(&e, 0x0900_0013).len(), 1);
        assert_eq!(args_of(&e, 0x0900_0013)[0][2], 16);
        // With a property that casts and items collected the result is the same.
        let mut e = engine();
        let scene = gather_scene(&mut e, 0x77, &[3]);
        e.call(
            0x00ca_11e0,
            &args![0x1234u32, scene.body, scene.list_a, scene.list_b],
        );
        assert_eq!(hk_items(&e, scene.list_a), [scene.hk_object]);
        assert_eq!(hk_items(&e, scene.list_b).len(), 1);
    }

    #[test]
    fn gathering_does_nothing_when_a_listed_owner_blocks_or_nothing_was_found() {
        // The owner answering 0xC blocks the whole gathering.
        let mut e = engine();
        let scene = gather_scene(&mut e, 0, &[3, 0xc]);
        e.call_log = Some(vec![]);
        e.call(
            0x00ca_11e0,
            &args![0x1234u32, scene.body, scene.list_a, scene.list_b],
        );
        assert!(hk_items(&e, scene.list_a).is_empty());
        assert!(hk_items(&e, scene.list_b).is_empty());
        // A property that casts and no items: nothing either.
        let mut e = engine();
        let scene = gather_scene(&mut e, 0x77, &[]);
        e.call(
            0x00ca_11e0,
            &args![0x1234u32, scene.body, scene.list_a, scene.list_b],
        );
        assert!(hk_items(&e, scene.list_a).is_empty());
        // A null body is ignored.
        e.call_log = Some(vec![]);
        e.call(
            0x00ca_11e0,
            &args![0x1234u32, 0u32, scene.list_a, scene.list_b],
        );
        assert_eq!(order(&e).len(), 1);
    }

    /// Everything `Update` touches: one system with one rigid body and one bone.
    struct Rig {
        util: Ptr<BhkRagdollPenetrationUtil>,
        system: Ptr<ConstrainedSystem>,
        detector: u32,
        body: u32,
        wrapper: u32,
        phantom: u32,
        skeleton: u32,
        instance: u32,
    }

    fn rig(e: &mut Engine) -> Rig {
        install_heap(e);
        let world = e.mem.alloc(0x400);
        e.mem.set_u32(world + 0x350, 0x5151);
        let body = e.mem.alloc(0x240);
        e.mem.set_u32(body + 8, world);
        e.mem.set_u8(body + 0xe8, 1);
        let wrapper = object_with_slots(e, 0x0300_0000, &[(0xc4, 0x0900_0300)]);
        e.mem.set_u32(wrapper + 8, body);
        e.mem.set_u32(body + 0xc, wrapper);
        e.register(0x0900_0300, |_, _| Ret::default());
        let phantom =
            object_with_slots(e, 0x0300_0100, &[(0x94, 0x0900_0310), (0x9c, 0x0900_0311)]);
        e.register_double(0x0900_0310, move |_, _| ret(world));
        e.register(0x0900_0311, |_, _| Ret::default());
        let skeleton = e.mem.alloc(0x40);
        e.mem.set_u32(skeleton + 0x1c, 1);
        e.mem.set_u32(skeleton + 0x34, 2);
        let instance = e.mem.alloc(0x40);
        let bodies = e.mem.alloc(4);
        e.mem.set_u32(bodies, body);
        let map = e.mem.alloc(4);
        e.mem.set_u32(instance + 8, bodies);
        e.mem.set_u32(instance + 0xc, 1);
        e.mem.set_u32(instance + 0x20, map);
        e.mem.set_u32(instance + 0x2c, skeleton);
        let detector = e.mem.alloc(0x40);
        let flags = e.mem.alloc(4);
        e.mem.set_u32(flags, 1);
        e.mem.set_u32(detector + 0x18, flags);
        e.mem.set_u32(detector + 0x1c, 1);
        let penetrated = e.mem.alloc(0x30);
        put_vec(e, penetrated + 0x10, [1.0, 2.0, 3.0, 0.0]);
        put_vec(e, penetrated + 0x20, [0.0, 0.0, 1.0, 0.0]);
        let system = e.new_object::<ConstrainedSystem>();
        e.set(
            system,
            ConstrainedSystem::pRagdollInstance,
            Ptr::new(instance),
        );
        e.set(
            system,
            ConstrainedSystem::pBoundingPhantom,
            Ptr::new(phantom),
        );
        e.set(
            system,
            ConstrainedSystem::pDetectionUtil,
            Ptr::new(detector),
        );
        e.set(
            system,
            ConstrainedSystem::penetratedBones,
            Ptr::new(penetrated),
        );
        e.set(system, ConstrainedSystem::penetratedBonesSize, 1);
        let util = e.new_object::<BhkRagdollPenetrationUtil>();
        let systems = e.mem.alloc(4);
        e.mem.set_u32(systems, system.addr());
        e.set(
            util,
            BhkRagdollPenetrationUtil::pActiveSystems,
            Ptr::new(systems),
        );
        e.set(util, BhkRagdollPenetrationUtil::iActiveSystemsSize, 1);
        e.set(util, BhkRagdollPenetrationUtil::timerStart, 0);
        e.set(util, BhkRagdollPenetrationUtil::timerLimit, 1_000_000);
        e.register(GET_TIMER, |_, _| 5u64.into_ret());
        e.register(RESET_DETECTION, |_, _| Ret::default());
        e.register(NO_OP, |_, _| Ret::default());
        e.register(ACTIVATE, |_, _| Ret::default());
        e.register(FILL_POSE, |_, _| Ret::default());
        e.register(DESTROY_INPUT, |_, _| Ret::default());
        e.register_double(PREPARE_INPUT, |_, a| ret(a[0] + 0x10));
        // The body's motion: its vtable slot 0x50 takes (impulse, point).
        let mut table = vec![0u32; 0x20];
        table[0x50 / 4] = 0x0900_0320;
        e.put_vtable(0x0300_0200, &table);
        e.mem.set_u32(body + 0xe0, 0x0300_0200);
        e.set_global(SPEED_THRESHOLD, 1.1920929e-7f32);
        e.set_global(IMPULSE_BASE, 2.0f32);
        e.set_global(IMPULSE_LIMIT, 100.0f32);
        e.set_global(HALF, 0.5f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(ONE, 1.0f32);
        e.register_double(GET_MASS, |_, _| Ret {
            st0: 4.0,
            ..Ret::default()
        });
        Rig {
            util,
            system,
            detector,
            body,
            wrapper,
            phantom,
            skeleton,
            instance,
        }
    }

    /// Registers the detection double answering `answer` and recording the input block.
    fn detection_answers(e: &mut Engine, answer: u8) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(DETECT_PENETRATION, move |e, a| {
            let input = a[2];
            sink.borrow_mut().push(vec![
                e.mem.u32(input),
                e.mem.u32(input + 0x14),
                e.mem.u32(input + 8),
                e.mem.u32(input + 0x20),
                e.mem.u32(input + 0x30),
                input,
                a[3],
            ]);
            e.mem.set_u8(a[1], answer);
            ret(a[1])
        });
        seen
    }

    fn filter_info(e: &Engine, rig: &Rig) -> u32 {
        e.mem.u32(rig.body + 0x2c)
    }

    #[test]
    fn update_detects_a_penetration_and_pushes_the_body_out() {
        let mut e = engine();
        let rig = rig(&mut e);
        let seen = detection_answers(&mut e, 1);
        put_vec(&mut e, rig.body + 0x140, [0.0, 2.0, 0.0, 0.0]);
        put_vec(&mut e, rig.body + 0x1c0, [0.0, 1.0, 0.0, 0.0]);
        put_vec(&mut e, rig.body + 0x1b0, [0.0, 0.0, 2.0, 0.0]);
        let pushes = Rc::new(RefCell::new(Vec::new()));
        let sink = pushes.clone();
        e.register_double(0x0900_0320, move |e, a| {
            sink.borrow_mut()
                .push((load_vec(e, a[1]), load_vec(e, a[2])));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00ca_1410, &args![rig.util]);
        // The detector was reset (budget not exceeded), then ran on the sized input.
        assert_eq!(args_of(&e, RESET_DETECTION), [[rig.detector]]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0][0], rig.skeleton);
        assert_eq!(&seen[0][1..5], &[1, 1, 1, 2]);
        assert_eq!(seen[0][6], rig.system.addr() + 0x14);
        let reserves: Vec<Vec<u32>> = args_of(&e, ARRAY_RESERVE)
            .iter()
            .map(|c| vec![c[1] - seen[0][5], c[2], c[3]])
            .collect();
        assert_eq!(
            reserves,
            [
                vec![0x10, 1, 0x30],
                vec![0x4, 1, 0x30],
                vec![0x1c, 1, 4],
                vec![0x2c, 4, 4]
            ]
        );
        assert_eq!(
            args_of(&e, FILL_POSE)[0][0],
            e.mem.u32(rig.system.addr() + 4)
        );
        assert!(e.get(rig.system, ConstrainedSystem::bPenetrating));
        // Bit 14 of the filter info was set and the wrapper told (slot 0xC4).
        assert_eq!(filter_info(&e, &rig), 0x4000);
        assert_eq!(args_of(&e, 0x0900_0300), [[rig.wrapper]]);
        assert_eq!(args_of(&e, NO_OP), [[rig.wrapper], [rig.wrapper]]);
        // The impulse: relative velocity (3, 0, 1) along (0, 0, 1) gives t = 1 / |v|.
        let speed = 10.0f32.sqrt();
        let inverse = refined_rsqrt(10.0, 0.5, 3.0);
        let t = inverse * 1.0;
        let factor = ((2.0f64 - f64::from(speed) * f64::from(t)) * 4.0) as f32;
        assert!((factor - 4.0).abs() < 0.01);
        let pushes = pushes.borrow();
        assert_eq!(pushes.len(), 1);
        assert_eq!(pushes[0].0, [0.0, 0.0, factor, 0.0]);
        assert_eq!(pushes[0].1, [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(args_of(&e, ACTIVATE), [[rig.body]]);
        assert_eq!(args_of(&e, DESTROY_INPUT).len(), 1);
    }

    #[test]
    fn the_impulse_is_clamped_to_the_limit_in_both_directions() {
        for (base, expected) in [(10.0f32, 1.0f32), (-50.0, -1.0)] {
            let mut e = engine();
            let rig = rig(&mut e);
            e.set_global(IMPULSE_BASE, base);
            e.set_global(IMPULSE_LIMIT, 1.0f32);
            put_vec(&mut e, rig.body + 0x1b0, [0.0, 0.0, 2.0, 0.0]);
            let pushes = Rc::new(RefCell::new(Vec::new()));
            let sink = pushes.clone();
            e.register_double(0x0900_0320, move |e, a| {
                sink.borrow_mut().push(load_vec(e, a[1]));
                Ret::default()
            });
            apply_push(&mut e, rig.util, 0, 0, rig.body);
            assert_eq!(pushes.borrow()[0], [0.0, 0.0, expected, 0.0]);
        }
        // A body at rest (speed below the threshold) gets t = 0: (base * mass) clamped.
        let mut e = engine();
        let rig = rig(&mut e);
        e.set_global(IMPULSE_LIMIT, 5.0f32);
        let pushes = Rc::new(RefCell::new(Vec::new()));
        let sink = pushes.clone();
        e.register_double(0x0900_0320, move |e, a| {
            sink.borrow_mut().push(load_vec(e, a[1]));
            Ret::default()
        });
        apply_push(&mut e, rig.util, 0, 0, rig.body);
        assert_eq!(pushes.borrow()[0], [0.0, 0.0, 5.0, 0.0]);
    }

    #[test]
    fn update_skips_systems_with_keyframed_bodies_but_still_resets_them() {
        let mut e = engine();
        let rig = rig(&mut e);
        let seen = detection_answers(&mut e, 1);
        e.mem.set_u8(rig.body + 0xe8, 4);
        e.call_log = Some(vec![]);
        e.call(0x00ca_1410, &args![rig.util]);
        assert!(seen.borrow().is_empty());
        assert_eq!(args_of(&e, RESET_DETECTION), [[rig.detector]]);
        // Motion type 5 counts too.
        e.mem.set_u8(rig.body + 0xe8, 5);
        e.call_log = Some(vec![]);
        e.call(0x00ca_1410, &args![rig.util]);
        assert!(seen.borrow().is_empty());
        // A first body without a world: skipped without a reset.
        e.mem.set_u8(rig.body + 0xe8, 1);
        e.mem.set_u32(rig.body + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ca_1410, &args![rig.util]);
        assert!(seen.borrow().is_empty());
        assert!(args_of(&e, RESET_DETECTION).is_empty());
    }

    #[test]
    fn update_respects_the_time_budget_unless_the_system_is_penetrating() {
        let mut e = engine();
        let rig = rig(&mut e);
        let seen = detection_answers(&mut e, 0);
        e.register(GET_TIMER, |_, _| 2_000_000u64.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00ca_1410, &args![rig.util]);
        assert!(seen.borrow().is_empty(), "over budget and not penetrating");
        assert_eq!(args_of(&e, RESET_DETECTION), [[rig.detector]]);
        // A penetrating system is checked regardless of the budget.
        e.set(rig.system, ConstrainedSystem::bPenetrating, true);
        e.call_log = Some(vec![]);
        e.call(0x00ca_1410, &args![rig.util]);
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn update_clears_the_marks_when_the_penetration_ended() {
        let mut e = engine();
        let rig = rig(&mut e);
        let _ = detection_answers(&mut e, 0);
        e.set(rig.system, ConstrainedSystem::bPenetrating, true);
        e.mem.set_u32(rig.body + 0x2c, 0x4005);
        e.call_log = Some(vec![]);
        e.call(0x00ca_1410, &args![rig.util]);
        assert!(!e.get(rig.system, ConstrainedSystem::bPenetrating));
        assert_eq!(filter_info(&e, &rig), 5);
        assert_eq!(args_of(&e, 0x0900_0300), [[rig.wrapper]]);
        // Reset before detection (penetrating) and after it (ended).
        assert_eq!(args_of(&e, RESET_DETECTION).len(), 2);
    }

    #[test]
    fn update_gives_the_phantom_the_bodys_world_when_they_differ() {
        let mut e = engine();
        let rig = rig(&mut e);
        let _ = detection_answers(&mut e, 0);
        e.register(0x0900_0310, |_, _| ret(0x1234_0000));
        e.call_log = Some(vec![]);
        e.call(0x00ca_1410, &args![rig.util]);
        assert_eq!(args_of(&e, 0x0900_0311), [[rig.phantom, 0x5151]]);
        let _ = rig.instance;
    }

    #[test]
    fn flagged_bones_that_are_not_pushed_only_get_the_mark_and_cleared_ones_lose_it() {
        let mut e = engine();
        let rig = rig(&mut e);
        let _ = detection_answers(&mut e, 1);
        let flags = e.mem.u32(rig.detector + 0x18);
        // Flag 2: marked but not pushed.
        e.mem.set_u32(flags, 2);
        e.register(0x0900_0320, |_, _| panic!("no push for flag 2"));
        e.call(0x00ca_1410, &args![rig.util]);
        assert_eq!(filter_info(&e, &rig), 0x4000);
        // Flag 0: the mark is removed again.
        e.mem.set_u32(flags, 0);
        e.call(0x00ca_1410, &args![rig.util]);
        assert_eq!(filter_info(&e, &rig), 0);
    }

    /// One recorded hit for `castRay`: `(fraction, normal, layer, type byte)`.
    type HitSpec = (f32, Vec4, u32, u8);
    /// One recorded ray: `(from, to, shape-collection filter byte, filter info)`.
    type RayLog = Vec<(Vec4, Vec4, u8, u32)>;

    fn ray_hits(e: &mut Engine, hits: &[HitSpec]) -> u32 {
        let data = e.mem.alloc(0x60 * hits.len().max(1) as u32);
        for (i, (fraction, normal, layer, kind)) in hits.iter().enumerate() {
            let entry = data + 0x60 * i as u32;
            e.mem.set_f32(entry + 0x10, *fraction);
            put_vec(e, entry, *normal);
            let owner = e.mem.alloc(0x40);
            e.mem.set_u32(owner + 0x2c, *layer);
            let collidable = owner + 0x10;
            e.mem.set_i8(collidable + 0x10, -0x10);
            e.mem.set_u8(collidable + 0x18, *kind);
            e.mem.set_u32(entry + 0x50, collidable);
        }
        data
    }

    /// Runs `castRay` with the world's casts answering `passes` in turn. Returns the hit
    /// flag, the written point and normal (both start as sevens and sixes) and the rays.
    fn run_cast(passes: Vec<Vec<HitSpec>>, group: i32) -> (u8, Vec4, Vec4, RayLog) {
        let mut e = engine();
        e.register(COLLECTOR_CONSTRUCT, |_, _| Ret::default());
        e.register(COLLECTOR_DESTRUCT, |_, _| Ret::default());
        let rays: Rc<RefCell<RayLog>> = Rc::new(RefCell::new(Vec::new()));
        let sink = rays.clone();
        let mut remaining = passes;
        remaining.reverse();
        let prepared = Rc::new(RefCell::new(remaining));
        e.register_double(WORLD_CAST_RAY, move |e, a| {
            let ray = a[1];
            sink.borrow_mut().push((
                load_vec(e, ray),
                load_vec(e, ray + 0x10),
                e.mem.u8(ray + 0x20),
                e.mem.u32(ray + 0x24),
            ));
            let hits = prepared.borrow_mut().pop().unwrap_or_default();
            let data = ray_hits(e, &hits);
            e.mem.set_u32(a[2] + 0x10, data);
            e.mem.set_u32(a[2] + 0x14, hits.len() as u32);
            Ret::default()
        });
        let util = e.new_object::<BhkRagdollPenetrationUtil>();
        e.set(util, BhkRagdollPenetrationUtil::iGroup, group);
        put_vec(&mut e, ONES_VECTOR, [1.0; 4]);
        let from = e.mem.alloc(16);
        let to = e.mem.alloc(16);
        let point = e.mem.alloc(16);
        let normal = e.mem.alloc(16);
        let result = e.mem.alloc(4);
        put_vec(&mut e, from, [0.0, 0.0, 0.0, 0.0]);
        put_vec(&mut e, to, [4.0, 8.0, 12.0, 0.0]);
        put_vec(&mut e, point, [7.0; 4]);
        put_vec(&mut e, normal, [6.0; 4]);
        e.mem.set_u8(result, 9);
        let returned = e
            .call(
                0x00ca_1c50,
                &args![util, result, 0xbeefu32, from, to, point, normal],
            )
            .u32();
        assert_eq!(returned, result);
        let log = rays.borrow().clone();
        (
            e.mem.u8(result),
            load_vec(&e, point),
            load_vec(&e, normal),
            log,
        )
    }

    #[test]
    fn a_ray_hit_reports_the_interpolated_point_and_the_normal() {
        let (found, point, normal, rays) = run_cast(
            vec![vec![
                (0.1, [9.0, 9.0, 9.0, 0.0], 5, 0),
                (0.25, [0.0, 1.0, 0.0, 0.0], 5, 1),
                (0.5, [7.0, 7.0, 7.0, 0.0], 5, 1),
            ]],
            3,
        );
        assert_eq!(found, 1);
        assert_eq!(point, [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(normal, [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(rays.len(), 1, "the first usable hit ends the search");
        assert_eq!(rays[0].0, [0.0; 4]);
        assert_eq!(rays[0].1, [4.0, 8.0, 12.0, 0.0]);
        assert_eq!(rays[0].2, 0);
        assert_eq!(rays[0].3, (3 << 16) | 0x1d);
    }

    #[test]
    fn the_reverse_ray_runs_when_the_first_finds_nothing_usable() {
        // Layers 8 and 0x1d are ignored in the first pass; the second pass hits at 0.5.
        let (found, point, normal, rays) = run_cast(
            vec![
                vec![(0.1, [1.0; 4], 8, 1), (0.2, [2.0; 4], 0x1d, 1)],
                vec![(0.5, [0.0, 0.0, 1.0, 0.0], 9, 1)],
            ],
            0,
        );
        assert_eq!(found, 1);
        // (1 - 0.5) * to + 0.5 * from: the segment was cast back to front.
        assert_eq!(point, [2.0, 4.0, 6.0, 0.0]);
        assert_eq!(normal, [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(rays.len(), 2);
        assert_eq!(rays[1].0, [4.0, 8.0, 12.0, 0.0]);
        assert_eq!(rays[1].1, [0.0; 4]);
        assert_eq!(rays[1].2, 1);
    }

    #[test]
    fn no_usable_hit_leaves_the_outputs_alone() {
        let (found, point, normal, rays) = run_cast(vec![vec![], vec![(0.5, [1.0; 4], 9, 0)]], 0);
        assert_eq!(found, 0);
        assert_eq!(point, [7.0; 4]);
        assert_eq!(normal, [6.0; 4]);
        assert_eq!(rays.len(), 2);
    }

    #[test]
    fn a_signalling_hit_fraction_is_quieted_like_the_x87_copy() {
        let signalling = f32::from_bits(0x7f80_0001);
        assert_eq!(x87_quiet(signalling).to_bits(), 0x7fc0_0001);
        assert_eq!(x87_quiet(1.5), 1.5);
        let (found, point, _, _) =
            run_cast(vec![vec![(signalling, [0.0, 1.0, 0.0, 0.0], 5, 1)]], 0);
        assert_eq!(found, 1);
        assert!(point.iter().all(|lane| lane.is_nan()));
    }

    #[test]
    fn the_node_walk_hands_physics_objects_to_the_gatherer_and_recurses() {
        let mut e = engine();
        install_heap(&mut e);
        let owner = e.mem.alloc(0x20);
        let casted = e.mem.alloc(0x20);
        let holder = e.mem.alloc(0x20);
        let hk_object = e.mem.alloc(0x20);
        let body = object_with_slots(&mut e, 0x0300_0000, &[(0x94, 0x0900_0600)]);
        e.mem.set_u32(body + 8, hk_object);
        e.register(0x0900_0600, |_, _| ret(1));
        e.mem.set_u32(casted + 0x10, holder);
        let node = e.mem.alloc(0x40);
        e.mem.set_u32(node + 0x1c, owner);
        let child = e.mem.alloc(0x40);
        let children = e.mem.alloc(4);
        e.mem.set_u32(children, child);
        let as_node = e.mem.alloc(0x100);
        e.mem.set_u16(as_node + 0xa6, 1);
        e.mem.set_u32(as_node + 0xa0, children);
        e.register_double(DYNAMIC_CAST, move |_, a| {
            let (object, kind) = (a[0], a[1]);
            if object == owner && kind == NODE_OWNER_TYPE {
                ret(casted)
            } else if object == holder && kind == NODE_BODY_TYPE {
                ret(body)
            } else if object == node && kind == NODE_CAST_TYPE {
                ret(as_node)
            } else {
                ret(0)
            }
        });
        e.register_double(GET_PROPERTY, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            ret(a[0])
        });
        let list_a = e.mem.alloc(12);
        let list_b = e.mem.alloc(12);
        e.mem.set_u32(list_a + 8, 0x8000_0000);
        e.mem.set_u32(list_b + 8, 0x8000_0000);
        e.call_log = Some(vec![]);
        e.call(0x00ca_1f00, &args![0x1234u32, node, list_a, list_b]);
        // The body's Havok object was gathered; the child (no owner) was visited.
        assert_eq!(hk_items(&e, list_a), [hk_object]);
        let casts: Vec<u32> = args_of(&e, DYNAMIC_CAST).iter().map(|c| c[0]).collect();
        assert!(casts.contains(&child));
        // A null node does nothing.
        e.call_log = Some(vec![]);
        e.call(0x00ca_1f00, &args![0x1234u32, 0u32, list_a, list_b]);
        assert_eq!(order(&e).len(), 1);
    }

    #[test]
    fn emptying_the_penetration_array_destroys_items_and_frees_owned_buffers() {
        let mut e = engine();
        install_heap(&mut e);
        e.register(OUTPUT_ELEMENT_DESTRUCT, |_, _| Ret::default());
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(0x60);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 2);
        e.mem.set_u32(array + 8, 2);
        e.call_log = Some(vec![]);
        e.call(0x00ca_1fb0, &args![array]);
        assert_eq!(
            args_of(&e, OUTPUT_ELEMENT_DESTRUCT),
            [[data], [data + 0x30]]
        );
        assert_eq!(args_of(&e, 0x0900_0013)[0][1..], [data, 0x60]);
        assert_eq!(
            [e.mem.u32(array), e.mem.u32(array + 4), e.mem.u32(array + 8)],
            [0, 0, 0x8000_0000]
        );
        // A buffer that is not owned (bit 31 set) is not freed.
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 1);
        e.mem.set_u32(array + 8, 0x8000_0001);
        e.call_log = Some(vec![]);
        e.call(0x00ca_1fb0, &args![array]);
        assert!(args_of(&e, 0x0900_0013).is_empty());
        assert_eq!(args_of(&e, OUTPUT_ELEMENT_DESTRUCT).len(), 1);
    }

    #[test]
    fn destroying_a_system_deletes_its_parts_inside_the_allocation_scope() {
        let mut e = engine();
        install_heap(&mut e);
        e.register(OUTPUT_ELEMENT_DESTRUCT, |_, _| Ret::default());
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x33);
        e.register_double(0x0900_0700, move |e, a| {
            assert_eq!(e.mem.u32(word), 0x10);
            assert_eq!(a[1], 1);
            Ret::default()
        });
        e.register_double(REMOVE_REFERENCE, move |e, _| {
            assert_eq!(e.mem.u32(word), 0x10);
            Ret::default()
        });
        let skeleton = object_with_slots(&mut e, 0x0300_0000, &[(0, 0x0900_0700)]);
        let phantom = object_with_slots(&mut e, 0x0300_0100, &[(0, 0x0900_0700)]);
        let system = e.new_object::<ConstrainedSystem>();
        e.set(system, ConstrainedSystem::pSkel, Ptr::new(skeleton));
        e.set(
            system,
            ConstrainedSystem::pBoundingPhantom,
            Ptr::new(phantom),
        );
        e.set(
            system,
            ConstrainedSystem::pRagdollInstance,
            Ptr::new(0x4000),
        );
        e.set(system, ConstrainedSystem::pDetectionUtil, Ptr::new(0x5000));
        e.set(
            system,
            ConstrainedSystem::penetratedBonesCapacity,
            0x8000_0000,
        );
        e.call_log = Some(vec![]);
        e.call(0x00ca_2020, &args![system]);
        assert_eq!(args_of(&e, 0x0900_0700), [[skeleton, 1], [phantom, 1]]);
        assert_eq!(args_of(&e, REMOVE_REFERENCE), [[0x4000], [0x5000]]);
        assert_eq!(e.mem.u32(word), 0x33);
        // Parts that are not there are skipped.
        let empty = e.new_object::<ConstrainedSystem>();
        e.set(
            empty,
            ConstrainedSystem::penetratedBonesCapacity,
            0x8000_0000,
        );
        e.call_log = Some(vec![]);
        e.call(0x00ca_2020, &args![empty]);
        assert!(args_of(&e, REMOVE_REFERENCE).is_empty());
    }

    /// A util with two (empty) systems and an `ActiveSystems` array whose slot 8 frees.
    fn util_with_systems(e: &mut Engine) -> (Ptr<BhkRagdollPenetrationUtil>, Vec<u32>, u32) {
        let util = e.new_object::<BhkRagdollPenetrationUtil>();
        util_with_systems_at(e, util)
    }

    fn util_with_systems_at(
        e: &mut Engine,
        util: Ptr<BhkRagdollPenetrationUtil>,
    ) -> (Ptr<BhkRagdollPenetrationUtil>, Vec<u32>, u32) {
        install_heap(e);
        e.register(0x0900_0800, |_, _| Ret::default());
        e.put_vtable(SYSTEM_ARRAY_VTABLE, &[0, 0, 0x0900_0800]);
        e.mem.set_u32(util.addr() + 4, SYSTEM_ARRAY_VTABLE);
        let buffer = e.mem.alloc(8);
        let mut systems = vec![];
        for i in 0..2 {
            let system = e.new_object::<ConstrainedSystem>();
            e.set(
                system,
                ConstrainedSystem::penetratedBonesCapacity,
                0x8000_0000,
            );
            e.mem.set_u32(buffer + 4 * i, system.addr());
            systems.push(system.addr());
        }
        e.set(
            util,
            BhkRagdollPenetrationUtil::pActiveSystems,
            Ptr::new(buffer),
        );
        e.set(util, BhkRagdollPenetrationUtil::iActiveSystemsSize, 2);
        e.set(util, BhkRagdollPenetrationUtil::iActiveSystemsReserved, 2);
        e.set(util, BhkRagdollPenetrationUtil::timerLimit, 77);
        e.register(HEAP_FREE, |_, _| Ret::default());
        (util, systems, buffer)
    }

    #[test]
    fn clear_destroys_and_frees_every_system_then_the_array() {
        let mut e = engine();
        let (util, systems, buffer) = util_with_systems(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00ca_20e0, &args![util]);
        assert_eq!(
            args_of(&e, HEAP_FREE),
            [[HEAP_SINGLETON, systems[0]], [HEAP_SINGLETON, systems[1]]]
        );
        assert_eq!(args_of(&e, 0x0900_0800), [[util.addr() + 4, buffer]]);
        assert_eq!(
            e.get(util, BhkRagdollPenetrationUtil::pActiveSystems)
                .addr(),
            0
        );
        assert_eq!(
            e.get(util, BhkRagdollPenetrationUtil::iActiveSystemsSize),
            0
        );
        assert_eq!(e.get(util, BhkRagdollPenetrationUtil::timerLimit), 0);
        // A null entry is skipped.
        let (util, systems, _) = util_with_systems(&mut e);
        let buffer = e
            .get(util, BhkRagdollPenetrationUtil::pActiveSystems)
            .addr();
        e.mem.set_u32(buffer, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ca_20e0, &args![util]);
        assert_eq!(args_of(&e, HEAP_FREE), [[HEAP_SINGLETON, systems[1]]]);
    }

    #[test]
    fn the_destructor_body_clears_and_ends_with_the_interface_vtable() {
        let mut e = engine();
        let (util, systems, _) = util_with_systems(&mut e);
        e.call(0x00ca_2140, &args![util]);
        assert_eq!(e.mem.u32(util.addr()), RAYCAST_INTERFACE_VTABLE);
        assert_eq!(e.mem.u32(util.addr() + 4), SYSTEM_ARRAY_VTABLE);
        assert_eq!(
            e.get(util, BhkRagdollPenetrationUtil::iActiveSystemsSize),
            0
        );
        assert_eq!(e.get(util, BhkRagdollPenetrationUtil::timerLimit), 0);
        let _ = systems;
    }

    #[test]
    fn the_util_deleting_destructor_frees_the_aligned_block_on_request() {
        for (flags, freed) in [(0u8, false), (1u8, true)] {
            let mut e = engine();
            // The object sits 8 bytes into its block; the byte below it holds the distance.
            let block = e.mem.alloc(0x80);
            let object = Ptr::<BhkRagdollPenetrationUtil>::new(block + 8);
            e.mem.set_u8(object.addr() - 1, 8);
            let _ = util_with_systems_at(&mut e, object);
            e.call_log = Some(vec![]);
            let result = e.call(0x00ca_2aa0, &args![object, flags]).ptr::<()>();
            assert_eq!(result.addr(), object.addr());
            let frees = args_of(&e, HEAP_FREE);
            assert_eq!(frees.contains(&vec![HEAP_SINGLETON, block]), freed);
            assert_eq!(frees.len(), if freed { 3 } else { 2 });
            assert_eq!(e.mem.u32(object.addr()), RAYCAST_INTERFACE_VTABLE);
        }
    }

    /// The scene for `CreateSkeletonForRagdoll`: two rigid bodies (the second hangs from
    /// the first and has no name) and one extra record for the second body.
    struct SkeletonScene {
        bodies: u32,
        parents: u32,
        extras: u32,
        rigid: [u32; 2],
    }

    fn skeleton_scene(e: &mut Engine, shape_kind: u32) -> SkeletonScene {
        install_heap(e);
        let rigid = [e.mem.alloc(0x240), e.mem.alloc(0x240)];
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"Bip01");
        e.mem.set_u32(rigid[0] + 0x74, name | 1);
        put_vec(e, rigid[0] + 0xf0 + 0x30, [1.0, 2.0, 3.0, 0.0]);
        put_vec(e, rigid[1] + 0xf0 + 0x30, [5.0, 5.0, 5.0, 0.0]);
        let bodies = e.mem.alloc(12);
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, rigid[0]);
        e.mem.set_u32(data + 4, rigid[1]);
        e.mem.set_u32(bodies, data);
        e.mem.set_i32(bodies + 4, 2);
        // Body 1 hangs from the entity that body 0 is: parents[0]->+0x18 == rigid[0].
        let parents = e.mem.alloc(12);
        let parent_data = e.mem.alloc(4);
        let parent_entry = e.mem.alloc(0x20);
        e.mem.set_u32(parent_entry + 0x18, rigid[0]);
        e.mem.set_u32(parent_data, parent_entry);
        e.mem.set_u32(parents, parent_data);
        e.mem.set_i32(parents + 4, 1);
        // The extra record, for body 1, with a shape.
        let shape = e.mem.alloc(0x40);
        e.mem.set_u32(shape + 0xc, shape_kind);
        e.mem.set_f32(shape + 0x10, 1.5);
        put_vec(e, shape + 0x20, [0.0, 0.0, 0.0, 0.0]);
        put_vec(e, shape + 0x30, [0.0, 0.0, 2.0, 0.0]);
        e.mem.set_u32(rigid[1] + 0x10, shape);
        let record = e.mem.alloc(0x10);
        e.mem.set_u32(record + 8, rigid[1]);
        let record_data = e.mem.alloc(4);
        e.mem.set_u32(record_data, record);
        let extras = e.mem.alloc(0x10);
        e.mem.set_u32(extras + 4, record_data);
        e.mem.set_u32(extras + 8, 1);
        // Exe constants.
        e.set_global(ZERO, 0.0f32);
        e.set_global(ONE, 1.0f32);
        e.set_global(HALF, 0.5f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(TWO_DOUBLE, 2.0f64);
        put_vec(e, MINUS_HALF_VECTOR, [-0.5; 4]);
        put_vec(e, UNIT_SCALE_VECTOR, [1.0; 4]);
        put_vec(e, IDENTITY_QUATERNION, [0.0, 0.0, 0.0, 1.0]);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK_ALL + 4 * lane, 0x8000_0000);
            e.mem.set_u32(
                SIGN_MASK_XYZ + 4 * lane,
                if lane < 3 { 0x8000_0000 } else { 0 },
            );
            e.mem.set_u32(
                LANE_MASK_XYZ + 4 * lane,
                if lane < 3 { 0xffff_ffff } else { 0 },
            );
        }
        SkeletonScene {
            bodies,
            parents,
            extras,
            rigid,
        }
    }

    /// Doubles for everything `CreateSkeletonForRagdoll` calls; returns the log of
    /// `hkStringPtr::operator=` calls as `(target, text)`.
    fn skeleton_doubles(e: &mut Engine) -> Rc<RefCell<Vec<(u32, String)>>> {
        e.register(STRING_PTR_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_PTR_DESTRUCT, |_, _| Ret::default());
        e.register_double(SET_FROM_TRANSFORM_NO_SCALE, |e, a| {
            let translation = load_vec(e, a[1] + 0x30);
            store_vec(e, a[0], translation);
            store_vec(e, a[0] + 0x10, [0.0, 0.0, 0.0, 1.0]);
            store_vec(e, a[0] + 0x20, [1.0; 4]);
            Ret::default()
        });
        e.register_double(QUATERNION_SET_MUL, |e, a| {
            store_vec(e, a[0], [7.0, 7.0, 7.0, 7.0]);
            Ret::default()
        });
        e.register_double(STRING_DUPLICATE, |_, a| ret(a[0]));
        let assigned = Rc::new(RefCell::new(Vec::new()));
        let sink = assigned.clone();
        e.register_double(STRING_PTR_ASSIGN, move |e, a| {
            let text = if a[1] == 0 {
                String::new()
            } else {
                String::from_utf8(e.mem.cstr(a[1])).unwrap()
            };
            sink.borrow_mut().push((a[0], text));
            Ret::default()
        });
        e.register(STRING_PTR_ASSIGN_COPY, |_, _| Ret::default());
        e.register_double(STRCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.register_double(STRNCPY_S, |e, a| {
            let text = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &text[..text.len().min(a[3] as usize)]);
            Ret::default()
        });
        e.register_double(STRCAT_S, |e, a| {
            let mut text = e.mem.cstr(a[0]);
            text.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &text);
            Ret::default()
        });
        e.mem.set_cstr(NO_NAME_TEXT, b"No Name");
        e.mem.set_cstr(NUB_SUFFIX, b" Nub");
        assigned
    }

    #[test]
    fn the_skeleton_gets_bones_parents_poses_and_names() {
        let mut e = engine();
        let scene = skeleton_scene(&mut e, 4);
        let assigned = skeleton_doubles(&mut e);
        // The error report for the unnamed body.
        let reporter = object_with_slots(&mut e, 0x0300_9000, &[(0xc, 0x0900_0900)]);
        e.set_global(ERROR_OBJECT, reporter);
        e.register(0x0900_0900, |_, _| Ret::default());
        e.register(STREAM_CONSTRUCT, |_, _| Ret::default());
        e.register(STREAM_WRITE, |_, _| Ret::default());
        e.register(STREAM_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let util = e.new_object::<BhkRagdollPenetrationUtil>();
        let skeleton = e
            .call(
                0x00ca_21c0,
                &args![util, scene.bodies, scene.parents, scene.extras],
            )
            .u32();
        // Header: size, reference count, vtable.
        assert_eq!(e.mem.u16(skeleton + 4), 0x48);
        assert_eq!(e.mem.u16(skeleton + 6), 1);
        assert_eq!(e.mem.u32(skeleton), SKELETON_VTABLE);
        // Three arrays of three entries were reserved.
        let reserved: Vec<(u32, u32, u32)> = args_of(&e, ARRAY_RESERVE)
            .iter()
            .map(|c| (c[1] - skeleton, c[2], c[3]))
            .collect();
        assert_eq!(reserved, [(0xc, 3, 2), (0x24, 3, 0x30), (0x18, 3, 8)]);
        assert_eq!(e.mem.u32(skeleton + 0x1c), 3);
        // Parent indices: root -1, body 1 hangs from body 0, the extra from body 1.
        let parent_data = e.mem.u32(skeleton + 0xc);
        assert_eq!(
            [
                e.mem.u16(parent_data),
                e.mem.u16(parent_data + 2),
                e.mem.u16(parent_data + 4)
            ],
            [0xffff, 0, 1]
        );
        // Lock bytes: root free, the rest locked.
        let bones = e.mem.u32(skeleton + 0x18);
        assert_eq!(
            [
                e.mem.u8(bones + 4),
                e.mem.u8(bones + 12),
                e.mem.u8(bones + 20)
            ],
            [0, 1, 1]
        );
        // Names: the root's copy, the missing one (null), the extra "No Name Nub".
        assert_eq!(
            *assigned.borrow(),
            [
                (bones, "Bip01".to_string()),
                (bones + 8, String::new()),
                (bones + 16, "No Name Nub".to_string())
            ]
        );
        assert_eq!(args_of(&e, 0x0900_0900).len(), 1, "one missing-name report");
        let report = &args_of(&e, 0x0900_0900)[0];
        assert_eq!(report[1..3], [1, 0xabba_cc14]);
        assert_eq!(report[4..], [SOURCE_FILE_TEXT, 0x2d1]);
        // Reference poses: the root's own, the child relative to it, the extra bone.
        let poses = e.mem.u32(skeleton + 0x24);
        assert_eq!(load_vec(&e, poses), [1.0, 2.0, 3.0, 0.0]);
        let relative = load_vec(&e, poses + 0x30);
        assert_eq!(relative[..3], [4.0, 3.0, 2.0]);
        assert_eq!(load_vec(&e, poses + 0x30 + 0x10), [7.0; 4]);
        let scale = load_vec(&e, poses + 0x30 + 0x20);
        assert!(scale[..3].iter().all(|lane| (lane - 1.0).abs() < 1e-3));
        assert_eq!(scale[3], 0.0, "lane 3 of the inverse scale is masked off");
        let extra = poses + 0x60;
        assert_eq!(load_vec(&e, extra), [3.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, extra + 0x10), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(load_vec(&e, extra + 0x20), [1.0; 4]);
        // The map block (4 bytes) was freed with its size.
        assert_eq!(args_of(&e, 0x0900_0013).len(), 1);
        assert_eq!(args_of(&e, 0x0900_0013)[0][2], 4);
        let _ = scene.rigid;
    }

    #[test]
    fn capsule_extras_use_the_farther_end_point_and_the_radius() {
        let mut e = engine();
        let scene = skeleton_scene(&mut e, 5);
        let _ = skeleton_doubles(&mut e);
        e.mem.set_u32(scene.rigid[1] + 0x74, 0);
        let reporter = object_with_slots(&mut e, 0x0300_9000, &[(0xc, 0x0900_0900)]);
        e.set_global(ERROR_OBJECT, reporter);
        e.register(0x0900_0900, |_, _| Ret::default());
        e.register(STREAM_CONSTRUCT, |_, _| Ret::default());
        e.register(STREAM_WRITE, |_, _| Ret::default());
        e.register(STREAM_DESTRUCT, |_, _| Ret::default());
        let util = e.new_object::<BhkRagdollPenetrationUtil>();
        let skeleton = e
            .call(
                0x00ca_21c0,
                &args![util, scene.bodies, scene.parents, scene.extras],
            )
            .u32();
        // End points (0,0,0) and (0,0,2): the second is longer; the unit direction from
        // the second to the first is (0, 0, -1) scaled by the radius 1.5 (up to the
        // reciprocal-square-root estimate): (0, 0, 2 - 1.5).
        let extra = e.mem.u32(skeleton + 0x24) + 0x60;
        let position = load_vec(&e, extra);
        assert!(position[0].abs() < 1e-6 && position[1].abs() < 1e-6);
        assert!((position[2] - 0.5).abs() < 0.01, "{position:?}");
    }

    #[test]
    fn extras_without_a_shape_are_left_alone_and_unknown_shapes_stay_at_the_origin() {
        let mut e = engine();
        let scene = skeleton_scene(&mut e, 9);
        let assigned = skeleton_doubles(&mut e);
        let reporter = object_with_slots(&mut e, 0x0300_9000, &[(0xc, 0x0900_0900)]);
        e.set_global(ERROR_OBJECT, reporter);
        e.register(0x0900_0900, |_, _| Ret::default());
        e.register(STREAM_CONSTRUCT, |_, _| Ret::default());
        e.register(STREAM_WRITE, |_, _| Ret::default());
        e.register(STREAM_DESTRUCT, |_, _| Ret::default());
        let util = e.new_object::<BhkRagdollPenetrationUtil>();
        let skeleton = e
            .call(
                0x00ca_21c0,
                &args![util, scene.bodies, scene.parents, scene.extras],
            )
            .u32();
        let extra = e.mem.u32(skeleton + 0x24) + 0x60;
        assert_eq!(load_vec(&e, extra), [0.0; 4], "kind 9 has no position");
        assert_eq!(assigned.borrow().len(), 3);
        // Without a shape the extra record does not name or place a bone.
        e.mem.set_u32(scene.rigid[1] + 0x10, 0);
        let assigned_before = assigned.borrow().len();
        e.call(
            0x00ca_21c0,
            &args![util, scene.bodies, scene.parents, scene.extras],
        );
        assert_eq!(assigned.borrow().len() - assigned_before, 2);
    }
}

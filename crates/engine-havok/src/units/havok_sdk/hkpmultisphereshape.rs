//! `Havok SDK/hkpmultisphereshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpMultiSphereShape` holds up to eight spheres: `m_numSpheres` (+0x10)
//! and `m_spheres` (+0x20, `hkVector4` each, the radius in `w`). SSE code
//! computes lane by lane in `f32`; x87 code keeps its intermediate results
//! unrounded (`f64`) and rounds at each `float` store, the way its `FSTP`
//! instructions do, and compares the way its `FNSTSW` / `TEST AH` pairs read
//! the flags (NaN included).
//!
//! Two helpers of the ray cast have register parameters: `00d2a610` takes the
//! output in `EDI` and `00d2a960` the index array in `EAX`; here the register
//! value is the first argument.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpMultiSphereShape` (Xbox PDB), 0xa0 bytes.
    pub struct HkpMultiSphereShape: 0xa0 {
        /// The vtable.
        0x00 vtable: u32,
        /// `m_memSizeAndFlags` (Xbox PDB).
        0x04 m_memSizeAndFlags: u16,
        /// `m_referenceCount` (Xbox PDB).
        0x06 m_referenceCount: i16,
        /// `m_userData` (Xbox PDB).
        0x08 m_userData: u32,
        /// `m_type` (Xbox PDB).
        0x0C m_type: u32,
        /// `m_numSpheres` (Xbox PDB).
        0x10 m_numSpheres: i32,
        // +0x20 `m_spheres` (Xbox PDB): eight `hkVector4`.
    }
}

/// `m_spheres` (Xbox PDB).
const SPHERES: u32 = 0x20;
/// Four `1.0f`.
const ONES: u32 = 0x010e_c7f0;
/// `float` constants the bounds start from (`0x7f7fffee` and `0xff7fffee`).
const BOUND_MIN_START: u32 = 0x0104_da48;
const BOUND_MAX_START: u32 = 0x0104_e148;
/// `double` constants: 4.0 and 0.5.
const FOUR: u32 = 0x0101_db80;
const HALF: u32 = 0x0101_1588;
/// The CRT square root (`00ec6040`, argument in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// `TlsGetValue` import slot and the TLS index of the profiling record stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const PROFILE_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"TtrcMultiSpher"` and `"Et"`.
const PROFILE_BEGIN: u32 = 0x010c_e4f0;
const PROFILE_END: u32 = 0x010c_41f8;
/// The ray cast helpers of this unit.
const GET_BEST_HIT: u32 = 0x00d2_a610;
const CAST_RAY_INTERNAL: u32 = 0x00d2_a960;

// `hkpShapeRayCastOutput` (Xbox PDB): `m_normal` +0x00, `m_hitFraction`
// +0x10, `m_extraInfo` +0x14, `m_shapeKeys` +0x20, `m_shapeKeyIndex` +0x40.
const OUT_NORMAL: u32 = 0x00;
const OUT_HIT_FRACTION: u32 = 0x10;
const OUT_EXTRA_INFO: u32 = 0x14;
const OUT_SHAPE_KEYS: u32 = 0x20;
const OUT_SHAPE_KEY_INDEX: u32 = 0x40;
// `hkpShapeRayCastInput` (Xbox PDB): `m_from` +0x00, `m_to` +0x10.
const RAY_FROM: u32 = 0x00;
const RAY_TO: u32 = 0x10;

type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// A 16-byte copy (`MOVAPS` load and store), bit for bit.
fn copy_vec(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

/// `z + (y + x)`, the horizontal sum of `SHUFPS` / `ADDSS`.
fn sum_lanes(v: Vec4) -> f32 {
    v[2] + (v[1] + v[0])
}

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a profiling record (`label`, time stamp) to the thread's record
/// stream (`TlsGetValue` of the index at `0126835c`) while its cursor (`+4`)
/// is below its limit (`+0xC`); a record is 12 bytes.
fn profile_record(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

// Translated from 00d2a5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereShape::getNumCollisionSpheresImpl` (Xbox PDB): `m_numSpheres`.
pub fn hkp_multi_sphere_shape_get_num_collision_spheres_impl(
    e: &mut Engine,
    this: Ptr<HkpMultiSphereShape>,
) -> i32 {
    e.get(this, HkpMultiSphereShape::m_numSpheres)
}

// Translated from 00d2a610 (decompiled, FalloutNV.exe 1.4.0.525)
/// The part of the ray cast that picks the nearest of the hits
/// `00d2a960` found, called by `castRayImpl` and `castRayWithCollectorImpl`
/// (unnamed in the Xbox PDB).
///
/// Register parameter: `EDI` is `output` (an `hkpShapeRayCastOutput`), the
/// first argument here. The stack arguments are `spheres`, the `fractions`
/// (`float` array) and `indices` (`int` array) of the `count` hits, and the
/// ray `input`; the caller pops them.
///
/// Scans the fractions for the smallest one below `output.m_hitFraction`
/// (an ordered comparison, the first of equal ones wins) and returns its
/// position, or -1 when none is. For the hit it sets the shape key at
/// `m_shapeKeyIndex` to -1, stores the fraction in `m_hitFraction` and the
/// sphere's index in `m_extraInfo`, and writes the hit normal: the point on
/// the ray at the fraction minus the sphere centre, divided by the radius.
pub fn fn_00d2a610(
    e: &mut Engine,
    output: Ptr,
    spheres: Ptr,
    fractions: Ptr,
    indices: Ptr,
    count: i32,
    input: Ptr,
) -> i32 {
    let mut best = x87_quiet(e.mem.f32(output.addr() + OUT_HIT_FRACTION));
    let mut found = -1i32;
    for i in 0..count {
        let fraction = e.mem.f32(fractions.addr() + 4 * i as u32);
        // `FCOMPP` of the best against the fraction: taken only when the best
        // is above it (a tie or NaN skips).
        if best > fraction {
            best = x87_quiet(fraction);
            found = i;
        }
    }
    if found == -1 {
        return -1;
    }

    let key_index = e.mem.u32(output.addr() + OUT_SHAPE_KEY_INDEX);
    let sphere_index = e.mem.u32(indices.addr() + 4 * found as u32);
    e.mem.set_u32(
        output
            .addr()
            .wrapping_add(key_index.wrapping_mul(4))
            .wrapping_add(OUT_SHAPE_KEYS),
        0xffff_ffff,
    );
    e.mem.set_f32(output.addr() + OUT_HIT_FRACTION, best);
    let centre = load_vec(e, spheres.addr().wrapping_add(sphere_index << 4));
    let ones = load_vec(e, ONES);
    let from = load_vec(e, input.addr() + RAY_FROM);
    let to = load_vec(e, input.addr() + RAY_TO);
    let point: Vec4 = std::array::from_fn(|lane| {
        (from[lane] - centre[lane]) * (ones[lane] - best) + (to[lane] - centre[lane]) * best
    });
    // `FLD1; FDIVRP`: 1.0 / radius in double, rounded to a float.
    let inverse = (1.0f64 / centre[3] as f64) as f32;
    e.mem.set_u32(output.addr() + OUT_EXTRA_INFO, sphere_index);
    let normal: Vec4 = point.map(|lane| inverse * lane);
    store_vec(e, output.addr() + OUT_NORMAL, normal);
    found
}

// Translated from 00d2a7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereShape::getAabbImpl` (Xbox PDB): transforms every sphere
/// centre by the 4x4 `transform` (four column vectors: `y * c1 + x * c0 +
/// (z * c2 + c3)` per lane), takes the lane-wise minimum of `centre - radius`
/// and maximum of `centre + radius` (starting from the constants at `0104da48`
/// and `0104e148`, `MINPS` / `MAXPS` semantics) and writes `min - tolerance`
/// and `tolerance + max` to `out` (`hkAabb`: `m_min`, `m_max`).
pub fn hkp_multi_sphere_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpMultiSphereShape>,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let count = e.get(this, HkpMultiSphereShape::m_numSpheres);
    let c0 = load_vec(e, transform.addr());
    let c1 = load_vec(e, transform.addr() + 0x10);
    let c2 = load_vec(e, transform.addr() + 0x20);
    let c3 = load_vec(e, transform.addr() + 0x30);
    let start_min = e.mem.f32(BOUND_MIN_START);
    let start_max = e.mem.f32(BOUND_MAX_START);
    let mut min = [start_min; 4];
    let mut max = [start_max; 4];
    // The exe transforms the spheres last to first into a stack array and
    // runs that pass once even for a count of 0 or less (no visible effect).
    for index in 0..count.max(0) as u32 {
        let sphere = load_vec(e, this.addr() + SPHERES + 0x10 * index);
        let radius = sphere[3];
        let centre: Vec4 = std::array::from_fn(|lane| {
            (sphere[1] * c1[lane] + sphere[0] * c0[lane]) + (sphere[2] * c2[lane] + c3[lane])
        });
        for lane in 0..4 {
            let low = centre[lane] - radius;
            let high = centre[lane] + radius;
            // `MINPS dest, src`: the source unless `dest < src`; `MAXPS`
            // likewise with `dest > src`.
            min[lane] = if min[lane] < low { min[lane] } else { low };
            max[lane] = if max[lane] > high { max[lane] } else { high };
        }
    }
    let low: Vec4 = min.map(|lane| lane - tolerance);
    let high: Vec4 = max.map(|lane| tolerance + lane);
    store_vec(e, out.addr(), low);
    store_vec(e, out.addr() + 0x10, high);
}

// Translated from 00d2a920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereShape::getCollisionSpheresImpl` (Xbox PDB): copies the
/// `m_numSpheres` spheres to `out`.
/// Answers `out` (the `const hkSphere*` of the Xbox PDB signature; `EAX` is `out` at the
/// RET 00d2a957): hkpHeightFieldAgent::staticGetPenetrations reads it through the
/// hkpSphereRepShape slot +0x30 of body A's shape (`MOV [ESP+0x14],EAX` at 00d3187d).
pub fn hkp_multi_sphere_shape_get_collision_spheres_impl(
    e: &mut Engine,
    this: Ptr<HkpMultiSphereShape>,
    out: Ptr,
) -> Ptr {
    let count = e.get(this, HkpMultiSphereShape::m_numSpheres);
    for index in 0..count.max(0) as u32 {
        copy_vec(
            e,
            this.addr() + SPHERES + 0x10 * index,
            out.addr() + 0x10 * index,
        );
    }
    out
}

// Translated from 00d2a960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The ray against every sphere (unnamed in the Xbox PDB); called by
/// `castRayImpl` and `castRayWithCollectorImpl`.
///
/// Register parameter: `EAX` is `indices_out`, the first argument here. The
/// stack arguments are the ray `input` (`from`, `to`), the `spheres` and their
/// `count`, and `fractions_out`; the caller pops them. A profiling record
/// `"TtrcMultiSpher"` is written first and `"Et"` last.
///
/// For every sphere the ray (`from` to `to`, relative to the centre) is
/// intersected as a quadratic: with `f` the start and `d` the direction,
/// `A = f.f`, `b = 2 f.d`, `C = d.d`, `e = A - r^2` and discriminant
/// `b^2 - 4 C e`. It records a hit only when `b` is negative (or NaN), the
/// discriminant is positive (or NaN), the nearer root `t = (-b - sqrt(disc))
/// / 2` lies below `C` (not at or above it, NaN passing) and is not negative.
/// A hit appends the sphere's index and the fraction `t / C` to the output
/// arrays. Returns the number of hits.
pub fn fn_00d2a960(
    e: &mut Engine,
    indices_out: Ptr,
    input: Ptr,
    spheres: Ptr,
    count: i32,
    fractions_out: Ptr,
) -> i32 {
    profile_record(e, PROFILE_BEGIN);
    let from = load_vec(e, input.addr() + RAY_FROM);
    let to = load_vec(e, input.addr() + RAY_TO);
    let mut hits = 0i32;
    for index in 0..count {
        let sphere_address = spheres
            .addr()
            .wrapping_add(0x10u32.wrapping_mul(index as u32));
        let centre = load_vec(e, sphere_address);
        let start: Vec4 = std::array::from_fn(|lane| from[lane] - centre[lane]);
        let end: Vec4 = std::array::from_fn(|lane| to[lane] - centre[lane]);
        let direction: Vec4 = std::array::from_fn(|lane| end[lane] - start[lane]);
        let start_squared: Vec4 = start.map(|lane| lane * lane);
        let a = sum_lanes(start_squared);
        let along: Vec4 = std::array::from_fn(|lane| direction[lane] * start[lane]);
        let b = sum_lanes(along);
        let doubled = (b as f64 + b as f64) as f32;
        // `FCOM` against 0 and `TEST AH,1`: taken for a negative or NaN value.
        let not_negative = doubled >= 0.0;
        if not_negative {
            continue;
        }
        let radius = x87_quiet(e.mem.f32(sphere_address + 0xc));
        let radius_squared = (radius as f64 * radius as f64) as f32;
        let b_squared = doubled as f64 * doubled as f64;
        let direction_squared: Vec4 = direction.map(|lane| lane * lane);
        let c = sum_lanes(direction_squared);
        let gap = (a as f64 - radius_squared as f64) as f32;
        let four: f64 = e.global(FOUR);
        let discriminant = (b_squared - gap as f64 * (c as f64 * four)) as f32;
        // `FCOM` against 0, `TEST AH,0x41`: skipped when not above 0 (NaN passes).
        let not_above = discriminant <= 0.0;
        if not_above {
            continue;
        }
        let negated = (-(doubled as f64)) as f32;
        let root = e.call(X87_SQRT, &args![discriminant as f64]).f64() as f32;
        let near = (negated as f64 - root as f64) as f32;
        let half: f64 = e.global(HALF);
        let parameter = (near as f64 * half) as f32;
        // `FCOM C, t`: skipped when `C <= t`.
        if c <= parameter {
            continue;
        }
        // `FCOM 0, t`: skipped when `t` is below 0.
        if parameter < 0.0 {
            continue;
        }
        let fraction = (parameter as f64 / c as f64) as f32;
        e.mem
            .set_f32(fractions_out.addr() + 4 * hits as u32, fraction);
        e.mem
            .set_u32(indices_out.addr() + 4 * hits as u32, index as u32);
        hits += 1;
    }
    profile_record(e, PROFILE_END);
    hits
}

// Translated from 00d2ab90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereShape::castRayImpl` (Xbox PDB): finds the hits of the ray
/// `input` against the spheres (`00d2a960`, into two 8-entry arrays on the
/// stack), then the nearest one (`00d2a610`) which updates `output`. Writes
/// whether there was one (`hkBool`) to `result` and returns `result`.
pub fn hkp_multi_sphere_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr<HkpMultiSphereShape>,
    result: Ptr,
    input: Ptr,
    output: Ptr,
) -> Ptr {
    let spheres = Ptr::<()>::new(this.addr() + SPHERES);
    let count = e.get(this, HkpMultiSphereShape::m_numSpheres);
    let best = e.with_stack(0x40, |e, block| {
        let indices = block;
        let fractions = Ptr::<()>::new(block.addr() + 0x20);
        let hits = e
            .call(
                CAST_RAY_INTERNAL,
                &args![indices, input, spheres, count, fractions],
            )
            .u32() as i32;
        e.call(
            GET_BEST_HIT,
            &args![output, spheres, fractions, indices, hits, input],
        )
        .u32() as i32
    });
    e.mem.set_u8(result.addr(), u8::from(best != -1));
    result
}

// Translated from 00d2abe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereShape::castRayWithCollectorImpl` (Xbox PDB): finds the hits
/// of the ray `input` (`00d2a960`), then reports them nearest first to
/// `collector` (virtual call, slot 0, with `cd_body` and the output). Each
/// round resets the scratch output (fraction 1.0, extra info -1, first shape
/// key -1, key index 0), picks the nearest remaining hit (`00d2a610`), turns
/// its normal to world space with the columns of the body's transform
/// (`[cd_body + 8]`: `y * c1 + x * c0 + z * c2`), calls the collector, and
/// replaces that hit by the last remaining one (when no hit is found the
/// index is -1 and the slots before the arrays are overwritten, as in the exe).
pub fn hkp_multi_sphere_shape_cast_ray_with_collector_impl(
    e: &mut Engine,
    this: Ptr<HkpMultiSphereShape>,
    input: Ptr,
    cd_body: Ptr,
    collector: Ptr,
) {
    let spheres = Ptr::<()>::new(this.addr() + SPHERES);
    let count = e.get(this, HkpMultiSphereShape::m_numSpheres);
    // One scratch frame: the output (0x50 bytes), then the fractions and the
    // indices (8 entries each), as the exe lays them out on its stack.
    e.with_stack(0x90, |e, frame| {
        let output = frame;
        let fractions = Ptr::<()>::new(frame.addr() + 0x50);
        let indices = Ptr::<()>::new(frame.addr() + 0x70);
        let mut remaining = e
            .call(
                CAST_RAY_INTERNAL,
                &args![indices, input, spheres, count, fractions],
            )
            .u32() as i32;
        e.mem.set_f32(output.addr() + OUT_HIT_FRACTION, 1.0);
        e.mem.set_u32(output.addr() + OUT_EXTRA_INFO, 0xffff_ffff);
        e.mem.set_u32(output.addr() + OUT_SHAPE_KEYS, 0xffff_ffff);
        e.mem.set_u32(output.addr() + OUT_SHAPE_KEY_INDEX, 0);
        while remaining != 0 {
            e.mem.set_u32(output.addr() + OUT_EXTRA_INFO, 0xffff_ffff);
            e.mem.set_u32(output.addr() + OUT_SHAPE_KEYS, 0xffff_ffff);
            e.mem.set_f32(output.addr() + OUT_HIT_FRACTION, 1.0);
            e.mem.set_u32(output.addr() + OUT_SHAPE_KEY_INDEX, 0);
            let best = e
                .call(
                    GET_BEST_HIT,
                    &args![output, spheres, fractions, indices, remaining, input],
                )
                .u32() as i32;
            let motion = e.mem.u32(cd_body.addr() + 8);
            let normal = load_vec(e, output.addr() + OUT_NORMAL);
            let c0 = load_vec(e, motion);
            let c1 = load_vec(e, motion + 0x10);
            let c2 = load_vec(e, motion + 0x20);
            let world: Vec4 = std::array::from_fn(|lane| {
                (normal[1] * c1[lane] + normal[0] * c0[lane]) + normal[2] * c2[lane]
            });
            store_vec(e, output.addr() + OUT_NORMAL, world);
            e.vcall(collector.addr(), 0, &args![cd_body, output]);
            let last = (remaining - 1) as u32;
            let fraction = x87_quiet(e.mem.f32(fractions.addr() + 4 * last));
            let index = e.mem.u32(indices.addr() + 4 * last);
            let slot = 4u32.wrapping_mul(best as u32);
            e.mem.set_f32(fractions.addr().wrapping_add(slot), fraction);
            e.mem.set_u32(indices.addr().wrapping_add(slot), index);
            remaining -= 1;
        }
    });
}

/// `hkReferencedObject::calcContentStatistics` (`00c90360`, `this`, collector, class record).
const REFERENCED_OBJECT_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
/// The class record `calcContentStatistics` hands on (`01268910`).
const MULTI_SPHERE_CLASS: u32 = 0x0126_8910;
/// The vtable the constructor stores (`010ce4bc`) and the shape type it sets (0x15).
const MULTI_SPHERE_VTABLE: u32 = 0x010c_e4bc;
const MULTI_SPHERE_TYPE: u32 = 0x15;

// Translated from 00d2a600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereShape::calcContentStatistics` (Xbox PDB): `ECX` = shape and two stack words,
/// the collector and a second word the function never reads (it jumps to the base function
/// with the class record `01268910` in its place). Calls
/// `hkReferencedObject::calcContentStatistics` (`00c90360`) with `(this, collector, 01268910)`.
pub fn hkp_multi_sphere_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpMultiSphereShape>,
    collector: u32,
    _unused_1: u32,
) {
    e.call(
        REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
        &args![this, collector, MULTI_SPHERE_CLASS],
    );
}

// Translated from 00d2a760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpMultiSphereShape::hkpMultiSphereShape` (Xbox PDB): `ECX` = the block and two stack words,
/// `spheres` (an array of `hkVector4`, centre and radius) and `count`. Sets the 16-bit field at
/// `+6` to 1, `m_userData` to 0, `m_type` to 0x15 and the vtable `010ce4bc`; when `count` is
/// positive copies that many vectors (16 bytes each, bit for bit) to `+0x20`, with no limit on
/// `count`; then stores `count` in `m_numSpheres`. Returns `this`.
pub fn hkp_multi_sphere_shape_construct(
    e: &mut Engine,
    this: Ptr<HkpMultiSphereShape>,
    spheres: u32,
    count: i32,
) -> Ptr<HkpMultiSphereShape> {
    e.set(this, HkpMultiSphereShape::m_referenceCount, 1);
    e.set(this, HkpMultiSphereShape::m_userData, 0);
    e.set(this, HkpMultiSphereShape::m_type, MULTI_SPHERE_TYPE);
    e.set(this, HkpMultiSphereShape::vtable, MULTI_SPHERE_VTABLE);
    for index in 0..count.max(0) as u32 {
        copy_vec(
            e,
            spheres + 0x10 * index,
            this.addr() + SPHERES + 0x10 * index,
        );
    }
    e.set(this, HkpMultiSphereShape::m_numSpheres, count);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d2a600,
            hkp_multi_sphere_shape_calc_content_statistics(Ptr<HkpMultiSphereShape>, u32, u32)
        ),
        entry!(
            0x00d2a760,
            hkp_multi_sphere_shape_construct(
                Ptr<HkpMultiSphereShape>,
                u32,
                i32,
            ) -> Ptr<HkpMultiSphereShape>
        ),
        entry!(
            0x00d2a5f0,
            hkp_multi_sphere_shape_get_num_collision_spheres_impl(Ptr<HkpMultiSphereShape>) -> i32
        ),
        entry!(0x00d2a610, fn_00d2a610(Ptr, Ptr, Ptr, Ptr, i32, Ptr) -> i32),
        entry!(
            0x00d2a7c0,
            hkp_multi_sphere_shape_get_aabb_impl(Ptr<HkpMultiSphereShape>, Ptr, f32, Ptr)
        ),
        entry!(
            0x00d2a920,
            hkp_multi_sphere_shape_get_collision_spheres_impl(Ptr<HkpMultiSphereShape>, Ptr) -> Ptr
        ),
        entry!(0x00d2a960, fn_00d2a960(Ptr, Ptr, Ptr, i32, Ptr) -> i32),
        entry!(
            0x00d2ab90,
            hkp_multi_sphere_shape_cast_ray_impl(Ptr<HkpMultiSphereShape>, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00d2abe0,
            hkp_multi_sphere_shape_cast_ray_with_collector_impl(
                Ptr<HkpMultiSphereShape>,
                Ptr,
                Ptr,
                Ptr,
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        store_vec(&mut e, ONES, [1.0; 4]);
        e.set_global(BOUND_MIN_START, f32::from_bits(0x7f7f_ffee));
        e.set_global(BOUND_MAX_START, f32::from_bits(0xff7f_ffee));
        e.set_global(FOUR, 4.0f64);
        e.set_global(HALF, 0.5f64);
        e
    }

    /// A shape with the given spheres (x, y, z, radius).
    fn shape(e: &mut Engine, spheres: &[Vec4]) -> Ptr<HkpMultiSphereShape> {
        let this = e.new_object::<HkpMultiSphereShape>();
        e.set(
            this,
            HkpMultiSphereShape::m_numSpheres,
            spheres.len() as i32,
        );
        for (i, sphere) in spheres.iter().enumerate() {
            store_vec(e, this.addr() + SPHERES + 0x10 * i as u32, *sphere);
        }
        this
    }

    /// A ray input from `from` to `to`.
    fn ray(e: &mut Engine, from: Vec4, to: Vec4) -> Ptr {
        let input = e.mem.alloc(0x20);
        store_vec(e, input, from);
        store_vec(e, input + 0x10, to);
        Ptr::new(input)
    }

    fn profile_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 3);
            Ret {
                eax: stream,
                ..Ret::default()
            }
        });
        stream
    }

    #[test]
    fn num_collision_spheres_is_the_count() {
        let mut e = engine();
        let this = shape(&mut e, &[[0.0; 4]; 3]);
        assert_eq!(e.call(0x00d2_a5f0, &args![this]).u32(), 3);
    }

    #[test]
    fn collision_spheres_answer_the_buffer() {
        let mut e = engine();
        let this = shape(&mut e, &[[1.0, 2.0, 3.0, 0.5]]);
        let out = e.mem.alloc(0x10);
        // EAX is `out` at the RET (00d2a957); hkpHeightFieldAgent reads it (00d3187d).
        assert_eq!(e.call(0x00d2_a920, &args![this, out]).u32(), out);
    }

    #[test]
    fn collision_spheres_copies_the_spheres() {
        let mut e = engine();
        let this = shape(&mut e, &[[1.0, 2.0, 3.0, 0.5], [4.0, 5.0, 6.0, 0.25]]);
        let out = e.mem.alloc(0x40);
        e.call(0x00d2_a920, &args![this, out]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 3.0, 0.5]);
        assert_eq!(load_vec(&e, out + 0x10), [4.0, 5.0, 6.0, 0.25]);
        // No spheres: nothing is written.
        let empty = shape(&mut e, &[]);
        let untouched = e.mem.alloc(16);
        e.call(0x00d2_a920, &args![empty, untouched]);
        assert_eq!(load_vec(&e, untouched), [0.0; 4]);
    }

    /// The identity transform with translation (10, 20, 30).
    fn transform(e: &mut Engine) -> Ptr {
        let address = e.mem.alloc(0x40);
        store_vec(e, address, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, address + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, address + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(e, address + 0x30, [10.0, 20.0, 30.0, 0.0]);
        Ptr::new(address)
    }

    #[test]
    fn aabb_bounds_the_transformed_spheres_and_grows_by_the_tolerance() {
        let mut e = engine();
        let this = shape(&mut e, &[[1.0, 2.0, 3.0, 1.0], [-1.0, 0.0, 0.0, 2.0]]);
        let matrix = transform(&mut e);
        let out = e.mem.alloc(0x20);
        e.call(0x00d2_a7c0, &args![this, matrix, 0.5f32, out]);
        // Centres (11, 22, 33) r 1 and (9, 20, 30) r 2.
        assert_eq!(load_vec(&e, out)[..3], [6.5, 17.5, 27.5]);
        assert_eq!(load_vec(&e, out + 0x10)[..3], [12.5, 23.5, 34.5]);
        // The transformed w lane is 0 for both: [-2, 2] grown by 0.5.
        assert_eq!(e.mem.f32(out + 0xc), -2.5);
        assert_eq!(e.mem.f32(out + 0x1c), 2.5);
    }

    #[test]
    fn aabb_of_no_spheres_is_the_start_constants_grown() {
        let mut e = engine();
        let this = shape(&mut e, &[]);
        let matrix = transform(&mut e);
        let out = e.mem.alloc(0x20);
        e.call(0x00d2_a7c0, &args![this, matrix, 1.0f32, out]);
        assert_eq!(e.mem.f32(out), f32::from_bits(0x7f7f_ffee) - 1.0);
        assert_eq!(e.mem.f32(out + 0x10), 1.0 + f32::from_bits(0xff7f_ffee));
    }

    /// Fills the arrays the hit picker reads; returns (spheres, fractions, indices).
    fn hits(e: &mut Engine, fractions: &[f32], indices: &[u32]) -> (Ptr, Ptr, Ptr) {
        let spheres = e.mem.alloc(0x40);
        store_vec(e, spheres, [0.0, 0.0, 0.0, 1.0]);
        store_vec(e, spheres + 0x10, [0.0, 2.0, 0.0, 2.0]);
        let fraction_array = e.mem.alloc(0x20);
        let index_array = e.mem.alloc(0x20);
        for (i, f) in fractions.iter().enumerate() {
            e.mem.set_f32(fraction_array + 4 * i as u32, *f);
        }
        for (i, v) in indices.iter().enumerate() {
            e.mem.set_u32(index_array + 4 * i as u32, *v);
        }
        (
            Ptr::new(spheres),
            Ptr::new(fraction_array),
            Ptr::new(index_array),
        )
    }

    #[test]
    fn best_hit_picks_the_smallest_fraction_below_the_output() {
        let mut e = engine();
        let (spheres, fractions, indices) = hits(&mut e, &[0.6, 0.25, 0.25, 0.9], &[1, 0, 1, 0]);
        let input = ray(&mut e, [0.0, 0.0, 0.0, 0.0], [0.0, 4.0, 0.0, 0.0]);
        let output = e.mem.alloc(0x50);
        e.mem.set_f32(output + OUT_HIT_FRACTION, 0.8);
        e.mem.set_u32(output + OUT_SHAPE_KEY_INDEX, 2);
        let best = e
            .call(
                GET_BEST_HIT,
                &args![output, spheres, fractions, indices, 4i32, input],
            )
            .u32() as i32;
        // The first of the two 0.25 entries wins (position 1, sphere 0).
        assert_eq!(best, 1);
        assert_eq!(e.mem.f32(output + OUT_HIT_FRACTION), 0.25);
        assert_eq!(e.mem.u32(output + OUT_SHAPE_KEYS + 8), 0xffff_ffff);
        assert_eq!(e.mem.u32(output + OUT_EXTRA_INFO), 0);
        // Point on the ray (0, 1, 0) minus the centre, over the radius 1.
        assert_eq!(load_vec(&e, output)[..3], [0.0, 1.0, 0.0]);
    }

    #[test]
    fn best_hit_returns_minus_one_without_a_nearer_hit() {
        let mut e = engine();
        let (spheres, fractions, indices) = hits(&mut e, &[0.5, 0.7], &[0, 1]);
        let input = ray(&mut e, [0.0; 4], [0.0, 4.0, 0.0, 0.0]);
        let output = e.mem.alloc(0x50);
        e.mem.set_f32(output + OUT_HIT_FRACTION, 0.5);
        let best = e
            .call(
                GET_BEST_HIT,
                &args![output, spheres, fractions, indices, 2i32, input],
            )
            .u32() as i32;
        assert_eq!(best, -1);
        assert_eq!(e.mem.f32(output + OUT_HIT_FRACTION), 0.5);
        // A count of zero finds nothing either.
        let best = e
            .call(
                GET_BEST_HIT,
                &args![output, spheres, fractions, indices, 0i32, input],
            )
            .u32() as i32;
        assert_eq!(best, -1);
    }

    /// What `00d2a960` found: hit count, sphere indices, fractions, profile stream.
    type Found = (i32, Vec<u32>, Vec<f32>, u32);

    /// Runs the intersection of a ray with the given spheres.
    fn internal(e: &mut Engine, spheres: &[Vec4], from: Vec4, to: Vec4) -> Found {
        let stream = profile_stream(e);
        let this = shape(e, spheres);
        let input = ray(e, from, to);
        let indices = e.mem.alloc(0x20);
        let fractions = e.mem.alloc(0x20);
        let n = e
            .call(
                CAST_RAY_INTERNAL,
                &args![
                    indices,
                    input,
                    this.addr() + SPHERES,
                    spheres.len() as i32,
                    fractions
                ],
            )
            .u32() as i32;
        let index_values = (0..n as u32).map(|i| e.mem.u32(indices + 4 * i)).collect();
        let fraction_values = (0..n as u32)
            .map(|i| e.mem.f32(fractions + 4 * i))
            .collect();
        (n, index_values, fraction_values, stream)
    }

    #[test]
    fn internal_records_the_hit_spheres_with_their_fractions() {
        let mut e = engine();
        // A ray along +y from y=-5 to y=5: sphere 0 (r 1 at the origin) is hit at 4/10,
        // sphere 1 (r 1 at y=3) at 7/10; sphere 2 (r 1 at x=5) is missed.
        let spheres = [
            [0.0, 0.0, 0.0, 1.0],
            [0.0, 3.0, 0.0, 1.0],
            [5.0, 0.0, 0.0, 1.0],
        ];
        let (n, indices, fractions, stream) = internal(
            &mut e,
            &spheres,
            [0.0, -5.0, 0.0, 0.0],
            [0.0, 5.0, 0.0, 0.0],
        );
        assert_eq!(n, 2);
        assert_eq!(indices, vec![0, 1]);
        assert_eq!(fractions, vec![0.4, 0.7]);
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
    }

    #[test]
    fn internal_skips_the_sphere_the_ray_moves_away_from() {
        let mut e = engine();
        // b >= 0: the ray moves away from the sphere behind its start.
        let (n, ..) = internal(
            &mut e,
            &[[0.0, -9.0, 0.0, 1.0]],
            [0.0; 4],
            [0.0, 5.0, 0.0, 0.0],
        );
        assert_eq!(n, 0);
    }

    #[test]
    fn internal_skips_a_sphere_the_segment_stops_short_of() {
        let mut e = engine();
        // The nearer root is at 95 against a segment length squared of 25.
        let (n, ..) = internal(
            &mut e,
            &[[0.0, 20.0, 0.0, 1.0]],
            [0.0; 4],
            [0.0, 5.0, 0.0, 0.0],
        );
        assert_eq!(n, 0);
    }

    #[test]
    fn internal_skips_a_sphere_with_the_nearer_root_behind_the_start() {
        let mut e = engine();
        // The start is inside (radius 3) and moves toward the centre: t = -4.
        let (n, ..) = internal(
            &mut e,
            &[[0.0, 0.0, 0.0, 3.0]],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, -1.0, 0.0, 0.0],
        );
        assert_eq!(n, 0);
    }

    #[test]
    fn internal_skips_a_ray_that_passes_beside_a_sphere() {
        let mut e = engine();
        // Negative discriminant.
        let (n, ..) = internal(
            &mut e,
            &[[3.0, 0.0, 0.0, 1.0]],
            [0.0, -5.0, 0.0, 0.0],
            [0.0, 5.0, 0.0, 0.0],
        );
        assert_eq!(n, 0);
        // A count of zero still writes the profile records.
        let (n, _, _, stream) = internal(&mut e, &[], [0.0; 4], [1.0; 4]);
        assert_eq!(n, 0);
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN);
    }

    #[test]
    fn cast_ray_reports_whether_a_sphere_was_hit() {
        let mut e = engine();
        profile_stream(&mut e);
        let this = shape(&mut e, &[[0.0, 0.0, 0.0, 1.0], [0.0, 3.0, 0.0, 1.0]]);
        let input = ray(&mut e, [0.0, -5.0, 0.0, 0.0], [0.0, 5.0, 0.0, 0.0]);
        let output = e.mem.alloc(0x50);
        e.mem.set_f32(output + OUT_HIT_FRACTION, 1.0);
        let result = e.mem.alloc(8);
        let returned = e
            .call(0x00d2_ab90, &args![this, result, input, output])
            .u32();
        assert_eq!(returned, result);
        assert_eq!(e.mem.u8(result), 1);
        assert_eq!(e.mem.f32(output + OUT_HIT_FRACTION), 0.4);
        assert_eq!(e.mem.u32(output + OUT_EXTRA_INFO), 0);
        // The output already holds a nearer hit: no hit.
        e.mem.set_f32(output + OUT_HIT_FRACTION, 0.1);
        e.call(0x00d2_ab90, &args![this, result, input, output]);
        assert_eq!(e.mem.u8(result), 0);
    }

    /// What the collector double saw: hit fraction, extra info, normal.
    type Seen = std::rc::Rc<std::cell::RefCell<Vec<(f32, u32, Vec4)>>>;

    #[test]
    fn collector_receives_the_hits_nearest_first_with_world_normals() {
        let mut e = engine();
        profile_stream(&mut e);
        let this = shape(&mut e, &[[0.0, 0.0, 0.0, 1.0], [0.0, 3.0, 0.0, 1.0]]);
        let input = ray(&mut e, [0.0, -5.0, 0.0, 0.0], [0.0, 5.0, 0.0, 0.0]);
        // A body whose motion state maps y to x: c0 = (0,0,1), c1 = (1,0,0).
        let motion = e.mem.alloc(0x30);
        store_vec(&mut e, motion, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, motion + 0x10, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, motion + 0x20, [0.0, 1.0, 0.0, 0.0]);
        let body = e.mem.alloc(0x10);
        e.mem.set_u32(body + 8, motion);
        // The collector's vtable: slot 0 records the output it is handed.
        let collector = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(collector, vtable);
        e.mem.set_u32(vtable, 0x00aa_0000);
        let seen: Seen = Default::default();
        let log = seen.clone();
        e.register_double(0x00aa_0000, move |e, a| {
            assert_eq!(a[1], body);
            let output = a[2];
            log.borrow_mut().push((
                e.mem.f32(output + OUT_HIT_FRACTION),
                e.mem.u32(output + OUT_EXTRA_INFO),
                load_vec(e, output),
            ));
            Ret::default()
        });
        e.call(0x00d2_abe0, &args![this, input, body, collector]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!((seen[0].0, seen[0].1), (0.4, 0));
        assert_eq!((seen[1].0, seen[1].1), (0.7, 1));
        // Both hits have the normal (0, -1, 0), which y * c1 turns to (-1, 0, 0).
        for hit in seen.iter() {
            let expected = [-1.0f32, 0.0, 0.0];
            for (got, want) in hit.2.iter().zip(expected) {
                assert!((got - want).abs() < 1e-5, "{:?}", hit.2);
            }
        }
    }

    #[test]
    fn collector_is_not_called_without_hits() {
        let mut e = engine();
        profile_stream(&mut e);
        let this = shape(&mut e, &[[9.0, 0.0, 0.0, 1.0]]);
        let input = ray(&mut e, [0.0, -5.0, 0.0, 0.0], [0.0, 5.0, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        e.call(0x00d2_abe0, &args![this, input, 0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == GET_BEST_HIT));
    }
    #[test]
    fn calc_content_statistics_hands_the_class_record_to_the_base_function() {
        let mut e = Engine::new();
        e.register(REFERENCED_OBJECT_CALC_CONTENT_STATISTICS, |_, _| {
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00d2_a600, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(
            log[1],
            (
                REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
                args![0x1000u32, 0x2000u32, 0x0126_8910u32].to_vec()
            )
        );
    }

    #[test]
    fn constructor_copies_the_spheres_and_sets_the_header() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0x30);
        store_vec(&mut e, source, [1.0, 2.0, 3.0, 0.5]);
        store_vec(&mut e, source + 0x10, [4.0, 5.0, 6.0, 0.75]);
        store_vec(&mut e, source + 0x20, [7.0, 8.0, 9.0, 1.0]);
        let this = e.new_object::<HkpMultiSphereShape>();
        e.mem.set_u32(this.addr() + 8, 0xdead);
        let result = e.call(0x00d2_a760, &args![this, source, 3i32]).ptr::<()>();
        assert_eq!(result.addr(), this.addr());
        assert_eq!(e.get(this, HkpMultiSphereShape::vtable), 0x010c_e4bc);
        assert_eq!(e.get(this, HkpMultiSphereShape::m_referenceCount), 1);
        assert_eq!(e.get(this, HkpMultiSphereShape::m_userData), 0);
        assert_eq!(e.get(this, HkpMultiSphereShape::m_type), 0x15);
        assert_eq!(e.get(this, HkpMultiSphereShape::m_numSpheres), 3);
        assert_eq!(load_vec(&e, this.addr() + SPHERES), [1.0, 2.0, 3.0, 0.5]);
        assert_eq!(
            load_vec(&e, this.addr() + SPHERES + 0x20),
            [7.0, 8.0, 9.0, 1.0]
        );
    }

    #[test]
    fn constructor_copies_nothing_for_a_count_of_zero_or_less() {
        let mut e = Engine::new();
        let source = e.mem.alloc(0x10);
        store_vec(&mut e, source, [1.0, 2.0, 3.0, 4.0]);
        let this = e.new_object::<HkpMultiSphereShape>();
        e.call(0x00d2_a760, &args![this, source, -2i32]);
        assert_eq!(e.get(this, HkpMultiSphereShape::m_numSpheres), -2);
        assert_eq!(load_vec(&e, this.addr() + SPHERES), [0.0; 4]);
    }
}

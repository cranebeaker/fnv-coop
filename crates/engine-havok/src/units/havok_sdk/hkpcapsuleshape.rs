//! `Havok SDK/hkpcapsuleshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpCapsuleShape` is a capsule: two end points `m_vertexA` (+0x20) and
//! `m_vertexB` (+0x30), the `w` lane of each holding the radius, and
//! `m_radius` (+0x10). SSE code (`SUBPS`, `MULPS`, `ADDSS`, `MAXPS`)
//! computes lane by lane in `f32`; a dot product is `z + (y + x)` the way
//! the `SHUFPS`/`ADDSS` sequences add it. x87 code compares the way its
//! `FNSTSW`/`TEST AH` pairs read the flags (NaN included) and rounds at
//! each `float` store; a float copied through `FLD`/`FSTP` goes through
//! [`x87_quiet`]. `RSQRTSS` is the host CPU's estimate (`rsqrt_estimate`),
//! as elsewhere in the crate. The profiling records `castRayImpl` writes
//! use the host clock for `RDTSC`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

layout! {
    /// `hkpCapsuleShape` (Xbox PDB), 0x40 bytes (offsets checked against
    /// the constructor `00d22590`).
    pub struct HkpCapsuleShape: 0x40 {
        /// The vtable.
        0x00 vtable: u32,
        /// `m_memSizeAndFlags` (Xbox PDB).
        0x04 m_memSizeAndFlags: u16,
        /// `m_referenceCount` (Xbox PDB).
        0x06 m_referenceCount: i16,
        /// `m_userData` (Xbox PDB).
        0x08 m_userData: u32,
        /// `m_type` (Xbox PDB): the shape type enum (the constructor stores 5).
        0x0C m_type: u32,
        /// `m_radius` (Xbox PDB).
        0x10 m_radius: f32,
        // +0x20 `m_vertexA` and +0x30 `m_vertexB` (Xbox PDB): `hkVector4`s.
    }
}

/// `hkpCapsuleShape`'s vtable.
const CAPSULE_VTABLE: u32 = 0x010c_e25c;
/// `m_vertexA` and `m_vertexB`.
const VERTEX_A: u32 = 0x20;
const VERTEX_B: u32 = 0x30;
/// The shape type the constructor stores in `m_type`.
const CAPSULE_SHAPE_TYPE: u32 = 5;

/// `hkReferencedObject::calcContentStatistics` (Xbox PDB, `this`, collector,
/// class).
const REFERENCED_OBJECT_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
/// The `hkClass` the capsule passes to it.
const CAPSULE_CLASS: u32 = 0x0126_8940;
/// The closest-points helper `00d22700` calls (`00d231b0`, cdecl, five
/// words: two points, two directions and a 0x30-byte result).
const CLOSEST_POINTS: u32 = 0x00d2_31b0;
/// `hkpSphereUtil::castRayUtil` (Xbox PDB, cdecl `(result, radius, rays,
/// output)`, returns the result pointer).
const SPHERE_CAST_RAY_UTIL: u32 = 0x00cd_85c0;
/// The CRT square root (`00ec6040`, argument in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// `TlsGetValue` import slot and the TLS index of the profiling record stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const PROFILE_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"TtrcCapsule"` and `"Et"`.
const PROFILE_BEGIN_CAPSULE: u32 = 0x010c_e2a4;
const PROFILE_END: u32 = 0x010c_41f8;
/// `float` constants: 0.5, 0.0, 3.0 (the `RSQRTSS` refinement), -1.0 and the
/// epsilon `2^-23`.
const HALF: u32 = 0x0101_6248;
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const MINUS_ONE: u32 = 0x0101_2054;
const EPSILON: u32 = 0x0104_da4c;
/// Vector constants: four 1.0 and four 0.0.
const ONES: u32 = 0x010e_c7f0;
const ZEROS: u32 = 0x010c_7160;

// `hkpShapeRayCastInput` (Xbox PDB): `m_from` +0x00, `m_to` +0x10.
const RAY_FROM: u32 = 0x00;
const RAY_TO: u32 = 0x10;
// `hkpShapeRayCastOutput` (Xbox PDB): `m_normal` +0x00, `m_hitFraction`
// +0x10, `m_extraInfo` +0x14, `m_shapeKeys` +0x20, `m_shapeKeyIndex` +0x40.
const OUT_NORMAL: u32 = 0x00;
const OUT_HIT_FRACTION: u32 = 0x10;
const OUT_EXTRA_INFO: u32 = 0x14;
const OUT_SHAPE_KEYS: u32 = 0x20;
const OUT_SHAPE_KEY_INDEX: u32 = 0x40;

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

fn zip_with(a: Vec4, b: Vec4, f: impl Fn(f32, f32) -> f32) -> Vec4 {
    std::array::from_fn(|lane| f(a[lane], b[lane]))
}

/// `z + (y + x)`: the `SHUFPS`/`ADDSS` sum of the first three lanes.
fn sum3(v: Vec4) -> f32 {
    v[2] + (v[1] + v[0])
}

fn dot3(a: Vec4, b: Vec4) -> f32 {
    sum3(zip_with(a, b, |x, y| x * y))
}

/// The `RSQRTSS` estimate refined once, with the `CMPEQSS`/`ANDNPS` zero
/// guard: `(half * r) * (three - (sum * r) * r)`, `+0.0` when `sum == zero`.
fn inverse_length(e: &Engine, sum: f32) -> f32 {
    let zero: f32 = e.global(ZERO);
    let three: f32 = e.global(THREE);
    let half: f32 = e.global(HALF);
    let estimate = rsqrt_estimate(sum);
    let step = three - (sum * estimate) * estimate;
    let value = (half * estimate) * step;
    if sum == zero {
        0.0
    } else {
        value
    }
}

/// `MAXPS dst, src` per lane: `dst` when it is greater, else `src`.
fn maxps(dst: f32, src: f32) -> f32 {
    if dst > src {
        dst
    } else {
        src
    }
}

/// `MINPS dst, src` per lane: `dst` when it is smaller, else `src`.
fn minps(dst: f32, src: f32) -> f32 {
    if dst < src {
        dst
    } else {
        src
    }
}

/// `(1 - fraction) * start + end * fraction` in every lane, the `SUBPS`,
/// `MULPS`, `ADDPS` sequence the interpolations use (`ones` is the vector
/// constant of four 1.0).
fn interpolate(ones: Vec4, start: Vec4, end: Vec4, fraction: f32) -> Vec4 {
    std::array::from_fn(|lane| (ones[lane] - fraction) * start[lane] + end[lane] * fraction)
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

// Translated from 00d22350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleShape::getNumCollisionSpheresImpl` (Xbox PDB): 2.
pub fn hkp_capsule_shape_get_num_collision_spheres_impl(
    _e: &mut Engine,
    _this: Ptr<HkpCapsuleShape>,
) -> u32 {
    2
}

// Translated from 00d22360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleShape::calcContentStatistics` (Xbox PDB): tail-calls
/// `hkReferencedObject::calcContentStatistics` with the collector and the
/// capsule's own `hkClass` (`01268940`) in place of the second argument,
/// which it never reads.
pub fn hkp_capsule_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpCapsuleShape>,
    collector: u32,
    _unused_1: u32,
) {
    e.call(
        REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
        &args![this, collector, CAPSULE_CLASS],
    );
}

// Translated from 00d22370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleShape::getSupportingVertexImpl` (Xbox PDB): writes the end
/// point that lies further along `direction` into `out`, its `w` lane
/// holding the vertex id as a float with the bits `0x3f000000 | id` (id 0
/// for `m_vertexA`, 0x10 for `m_vertexB`). `m_vertexA` is chosen when
/// `(B - A) . direction` is below zero; zero and NaN choose `m_vertexB`.
pub fn hkp_capsule_shape_get_supporting_vertex_impl(
    e: &mut Engine,
    this: Ptr<HkpCapsuleShape>,
    direction: Ptr,
    out: Ptr,
) {
    let a = load_vec(e, this.addr() + VERTEX_A);
    let b = load_vec(e, this.addr() + VERTEX_B);
    let d = load_vec(e, direction.addr());
    let along = dot3(zip_with(b, a, |p, q| p - q), d);
    // `FLDZ; FCOMP [sum]; TEST AH,0x41; JNZ`: B unless 0 > sum.
    if 0.0 > along {
        copy_vec(e, this.addr() + VERTEX_A, out.addr());
        e.mem.set_u32(out.addr() + 0xc, 0x3f00_0000);
    } else {
        copy_vec(e, this.addr() + VERTEX_B, out.addr());
        e.mem.set_u32(out.addr() + 0xc, 0x3f00_0010);
    }
}

// Translated from 00d223f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleShape::convertVertexIdsToVerticesImpl` (Xbox PDB): for each of
/// `count` 16-bit vertex ids, writes the 16-byte vector at `this + 0x20 +
/// id` to `out` and replaces its `w` lane by the bits `0x3f000000 | id`.
pub fn hkp_capsule_shape_convert_vertex_ids_to_vertices_impl(
    e: &mut Engine,
    this: Ptr<HkpCapsuleShape>,
    ids: Ptr,
    count: i32,
    out: Ptr,
) {
    let mut id_address = ids.addr();
    let mut out_address = out.addr();
    for _ in 0..count.max(0) {
        let id = e.mem.u16(id_address) as u32;
        copy_vec(e, this.addr() + VERTEX_A + id, out_address);
        e.mem.set_u32(out_address + 0xc, id | 0x3f00_0000);
        out_address = out_address.wrapping_add(0x10);
        id_address = id_address.wrapping_add(2);
    }
}

// Translated from 00d22440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleShape::getCentreImpl` (Xbox PDB): `0.5 * (B + A)` in every
/// lane.
pub fn hkp_capsule_shape_get_centre_impl(e: &mut Engine, this: Ptr<HkpCapsuleShape>, out: Ptr) {
    let a = load_vec(e, this.addr() + VERTEX_A);
    let b = load_vec(e, this.addr() + VERTEX_B);
    let half: f32 = e.global(HALF);
    let sum = zip_with(b, a, |p, q| p + q);
    store_vec(e, out.addr(), sum.map(|lane| half * lane));
}

// Translated from 00d22480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleShape::getFirstVertex` (Xbox PDB): copies `m_vertexB`.
pub fn hkp_capsule_shape_get_first_vertex(e: &mut Engine, this: Ptr<HkpCapsuleShape>, out: Ptr) {
    copy_vec(e, this.addr() + VERTEX_B, out.addr());
}

// Translated from 00d224a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleShape::getCollisionSpheresImpl` (Xbox PDB): copies
/// `m_vertexA` then `m_vertexB` (the two collision spheres) to `out`.
/// Answers `out` (the `const hkSphere*` of the Xbox PDB signature; `EAX` is `out` at the
/// RET 00d224bb): hkpConvexTransformShape::getCollisionSpheresImpl reads it through its child
/// shape's slot +0x30 (`MOV EDI,EAX` at 00d5df2f).
pub fn hkp_capsule_shape_get_collision_spheres_impl(
    e: &mut Engine,
    this: Ptr<HkpCapsuleShape>,
    out: Ptr,
) -> Ptr {
    copy_vec(e, this.addr() + VERTEX_A, out.addr());
    copy_vec(e, this.addr() + VERTEX_B, out.addr() + 0x10);
    out
}

// Translated from 00d224c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The closest point to `point` on the segment from `a` to `b` (all
/// four-lane vectors, cdecl; no Xbox PDB name). With `t = -((a - point) .
/// (b - a))` and `l = (b - a) . (b - a)`: `a` when `t <= 0`, `b` when `l <=
/// t`, else `a + (t / l) * (b - a)`. NaN takes the last case.
pub fn fn_00d224c0(e: &mut Engine, point: Ptr, a: Ptr, b: Ptr, out: Ptr) {
    use crate::sse::{add, addps, mulps, subps};
    // The SSE arithmetic with its NaN rules (the destination operand first).
    let dot = |x: Vec4, y: Vec4| {
        let products = mulps(x, y);
        add(products[2], add(products[1], products[0]))
    };
    let p = load_vec(e, point.addr());
    let a_vec = load_vec(e, a.addr());
    let b_vec = load_vec(e, b.addr());
    let d = subps(b_vec, a_vec);
    let to_a = subps(a_vec, p);
    // `FLD; FCHS; FSTP [float]`: the negation goes through x87.
    let t = x87_quiet(-dot(to_a, d));
    let l = dot(d, d);
    // `FCOM` against 0: the flag test falls through when `t < 0` or `t == 0`.
    if t <= 0.0 {
        store_vec(e, out.addr(), a_vec);
        return;
    }
    // `FCOM` of `l` against `t`: falls through when `l <= t`.
    if l <= t {
        store_vec(e, out.addr(), b_vec);
        return;
    }
    // `FDIVP` of the two loaded floats (the x87 choice of NaN), then `MULPS` with the
    // splatted quotient and `ADDPS` of `a`.
    let q = f32::from_bits(x87_store32(x87_div(f64::from(t), x87_load32(l.to_bits()))));
    store_vec(e, out.addr(), addps(mulps([q; 4], d), a_vec));
}

// Translated from 00d22590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleShape::hkpCapsuleShape` (Xbox PDB): sets the reference count
/// to 1, `m_userData` to 0, `m_type` to 5 and the vtable, copies the end
/// points `vertex_a` and `vertex_b` and stores `radius` in `m_radius` and
/// in the `w` lanes of both. Returns `this`.
pub fn hkp_capsule_shape_hkp_capsule_shape(
    e: &mut Engine,
    this: Ptr<HkpCapsuleShape>,
    vertex_a: Ptr,
    vertex_b: Ptr,
    radius: f32,
) -> Ptr<HkpCapsuleShape> {
    let radius = x87_quiet(radius);
    e.set(this, HkpCapsuleShape::m_radius, radius);
    e.set(this, HkpCapsuleShape::m_referenceCount, 1);
    e.set(this, HkpCapsuleShape::m_userData, 0);
    e.set(this, HkpCapsuleShape::m_type, CAPSULE_SHAPE_TYPE);
    e.set(this, HkpCapsuleShape::vtable, CAPSULE_VTABLE);
    copy_vec(e, vertex_a.addr(), this.addr() + VERTEX_A);
    copy_vec(e, vertex_b.addr(), this.addr() + VERTEX_B);
    e.mem.set_f32(this.addr() + VERTEX_A + 0xc, radius);
    e.mem.set_f32(this.addr() + VERTEX_B + 0xc, radius);
    this
}

// Translated from 00d225e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleShape::getAabbImpl` (Xbox PDB): transforms both end points by
/// `transform` (an `hkTransform`: three rotation columns then the
/// translation, each a four-lane vector; `p = (y*c1 + x*c0) + (z*c2 + t)`),
/// takes their lane-wise minimum and maximum (`MINPS`/`MAXPS`) and widens
/// the box by `m_radius + tolerance` (rounded to `float`).
pub fn hkp_capsule_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpCapsuleShape>,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let radius = e.get(this, HkpCapsuleShape::m_radius);
    let margin = (radius as f64 + tolerance as f64) as f32;
    let columns: [Vec4; 4] =
        std::array::from_fn(|column| load_vec(e, transform.addr() + 0x10 * column as u32));
    let transformed = |point: Vec4| -> Vec4 {
        std::array::from_fn(|lane| {
            let x_and_y = point[1] * columns[1][lane] + point[0] * columns[0][lane];
            let z_and_translation = point[2] * columns[2][lane] + columns[3][lane];
            x_and_y + z_and_translation
        })
    };
    // The loop runs on `m_vertexB` first, then `m_vertexA`.
    let transformed_b = transformed(load_vec(e, this.addr() + VERTEX_B));
    let transformed_a = transformed(load_vec(e, this.addr() + VERTEX_A));
    let highest = zip_with(transformed_a, transformed_b, maxps);
    let lowest = zip_with(transformed_a, transformed_b, minps);
    store_vec(e, out.addr(), lowest.map(|lane| lane - margin));
    store_vec(e, out.addr() + 0x10, highest.map(|lane| lane + margin));
}

// Translated from 00d22700 (decompiled, FalloutNV.exe 1.4.0.525)
/// A cdecl wrapper of the closest-points helper `00d231b0` (no Xbox PDB
/// name): runs it on the four inputs and distributes its 0x30-byte result:
/// the floats at +0, +4 and +8 go to `out_0`, `out_1` and `out_2` (through
/// x87, which quiets signalling NaNs), the vectors at +0x10 and +0x20 to
/// `out_vec_a` and `out_vec_b`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00d22700(
    e: &mut Engine,
    in_1: Ptr,
    in_2: Ptr,
    in_3: Ptr,
    in_4: Ptr,
    out_0: Ptr,
    out_1: Ptr,
    out_2: Ptr,
    out_vec_a: Ptr,
    out_vec_b: Ptr,
) {
    e.with_stack(0x30, |e, result| {
        e.call(CLOSEST_POINTS, &args![in_1, in_2, in_3, in_4, result]);
        copy_vec(e, result.addr() + 0x10, out_vec_a.addr());
        copy_vec(e, result.addr() + 0x20, out_vec_b.addr());
        let second = x87_quiet(e.mem.f32(result.addr() + 4));
        e.mem.set_f32(out_1.addr(), second);
        let third = x87_quiet(e.mem.f32(result.addr() + 8));
        e.mem.set_f32(out_2.addr(), third);
        let first = x87_quiet(e.mem.f32(result.addr()));
        e.mem.set_f32(out_0.addr(), first);
    });
}

// Translated from 00d22760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpCapsuleShape::castRayImpl` (Xbox PDB): casts the ray `input`
/// (`hkpShapeRayCastInput`: `m_from`, `m_to`) against the capsule and
/// updates `output` (`hkpShapeRayCastOutput`) when it hits closer than
/// `output.m_hitFraction`. Writes the hit flag (`hkBool`) to `result` and
/// returns `result`; a profiling record `"TtrcCapsule"` is written first and
/// `"Et"` last.
///
/// A ray that starts inside the capsule (`r*r > |from - closest|^2`) misses.
/// Otherwise the closest points between the ray and the capsule axis
/// (`00d22700`) give the entry fraction on the infinite cylinder; when it
/// lies within the segment the hit is on the cylinder (`m_extraInfo` 2),
/// else the end cap is tested with `hkpSphereUtil::castRayUtil` on the
/// nearer end point (`m_extraInfo` 0 or 1). The normal is normalised with
/// the refined `RSQRTSS`; the shape key slot `m_shapeKeyIndex` is set to
/// -1.
pub fn hkp_capsule_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr<HkpCapsuleShape>,
    result: Ptr,
    input: Ptr,
    output: Ptr,
) -> Ptr {
    profile_record(e, PROFILE_BEGIN_CAPSULE);
    let hit = e.with_stack(0x90, |e, scratch| {
        cast_ray_against_capsule(e, this, input, output, scratch)
    });
    e.mem.set_u8(result.addr(), hit as u8);
    profile_record(e, PROFILE_END);
    result
}

/// The body of `castRayImpl`; `scratch` is 0x90 bytes of the function's
/// stack locals (closest point +0x00, ray direction +0x10, axis direction
/// +0x20, the helper's results +0x30..+0x60, the sphere cast's rays +0x60,
/// its two spare words +0x80 and its `hkBool` result +0x88).
fn cast_ray_against_capsule(
    e: &mut Engine,
    this: Ptr<HkpCapsuleShape>,
    input: Ptr,
    output: Ptr,
    scratch: Ptr,
) -> bool {
    let me = this.addr();
    let out = output.addr();
    let from_address = input.addr() + RAY_FROM;
    let to_address = input.addr() + RAY_TO;
    let closest_address = scratch.addr();
    let ray_direction_address = scratch.addr() + 0x10;
    let axis_direction_address = scratch.addr() + 0x20;
    let distance_address = scratch.addr() + 0x30;
    let parameter_address = scratch.addr() + 0x34;
    let spare_address = scratch.addr() + 0x38;
    let vector_a_address = scratch.addr() + 0x40;
    let vector_b_address = scratch.addr() + 0x50;
    let rays_address = scratch.addr() + 0x60;
    let flag_address = scratch.addr() + 0x88;

    let ptr = Ptr::<()>::new;
    fn_00d224c0(
        e,
        ptr(from_address),
        ptr(me + VERTEX_A),
        ptr(me + VERTEX_B),
        ptr(closest_address),
    );
    let radius = e.get(this, HkpCapsuleShape::m_radius);
    let from = load_vec(e, from_address);
    let to = load_vec(e, to_address);
    let closest = load_vec(e, closest_address);
    let offset = zip_with(from, closest, |p, q| p - q);
    let start_distance_squared = dot3(offset, offset);
    // `r*r` is formed in x87 and stored as a `float`.
    let radius_squared = (radius as f64 * radius as f64) as f32;
    if radius_squared > start_distance_squared {
        return false;
    }

    let a = load_vec(e, me + VERTEX_A);
    let b = load_vec(e, me + VERTEX_B);
    let ray_direction = zip_with(to, from, |p, q| p - q);
    let axis_direction = zip_with(b, a, |p, q| p - q);
    store_vec(e, ray_direction_address, ray_direction);
    store_vec(e, axis_direction_address, axis_direction);
    fn_00d22700(
        e,
        ptr(from_address),
        ptr(ray_direction_address),
        ptr(me + VERTEX_A),
        ptr(axis_direction_address),
        ptr(distance_address),
        ptr(parameter_address),
        ptr(spare_address),
        ptr(vector_a_address),
        ptr(vector_b_address),
    );
    let distance_squared = x87_quiet(e.mem.f32(distance_address));
    if radius_squared < distance_squared {
        return false;
    }

    // The unit axis and the axis length (zero for a degenerate capsule).
    let axis_length_squared = dot3(axis_direction, axis_direction);
    let epsilon: f32 = e.global(EPSILON);
    let (axis, axis_length) = if epsilon < axis_length_squared {
        let inverse = inverse_length(e, axis_length_squared);
        (
            axis_direction.map(|lane| inverse * lane),
            x87_quiet(axis_length_squared * inverse),
        )
    } else {
        ([0.0; 4], 0.0)
    };

    // The ray direction without its part along the axis, and the fraction
    // where the ray enters the infinite cylinder (-1.0 when it has none).
    let along_axis = dot3(ray_direction, axis);
    let zero_lane: f32 = e.global(ZEROS);
    let factor = zero_lane - along_axis;
    let perpendicular = zip_with(axis, ray_direction, |u, d| factor * u + d);
    let perpendicular_squared = x87_quiet(dot3(perpendicular, perpendicular));
    let entry = if perpendicular_squared == 0.0 {
        e.global::<f32>(MINUS_ONE)
    } else {
        let parameter = x87_quiet(e.mem.f32(parameter_address)) as f64;
        let quotient = ((radius as f64 * radius as f64 - distance_squared as f64)
            / perpendicular_squared as f64) as f32;
        let root = e.call(X87_SQRT, &args![quotient as f64]).f64() as f32;
        (parameter - root as f64) as f32
    };
    // A hit has to be nearer than the one already in the output.
    if e.mem.f32(out + OUT_HIT_FRACTION) <= entry {
        return false;
    }

    let ones = load_vec(e, ONES);
    let point = interpolate(ones, from, to, entry);
    let a_along = x87_quiet(dot3(a, axis));
    let point_along = x87_quiet(dot3(point, axis));
    let height = (point_along as f64 - a_along as f64) as f32;
    if entry >= 0.0 && height > 0.0 && height < axis_length {
        // The ray enters the cylinder part between the end points.
        let fraction = (height as f64 / axis_length as f64) as f32;
        e.mem.set_f32(out + OUT_HIT_FRACTION, x87_quiet(entry));
        let on_axis = interpolate(ones, a, b, fraction);
        let normal = zip_with(point, on_axis, |p, q| p - q);
        let inverse = inverse_length(e, dot3(normal, normal));
        let key_index = e.mem.u32(out + OUT_SHAPE_KEY_INDEX);
        store_vec(e, out + OUT_NORMAL, normal.map(|lane| inverse * lane));
        e.mem.set_u32(out + OUT_EXTRA_INFO, 2);
        e.mem.set_u32(
            out.wrapping_add(key_index.wrapping_mul(4))
                .wrapping_add(OUT_SHAPE_KEYS),
            0xffff_ffff,
        );
        return true;
    }

    // Beyond an end point, or entering before the ray's start. A start that
    // is inside the infinite cylinder's radius at the entry side but too far
    // from the axis point below it is rejected when the entry is behind it.
    let from_along = x87_quiet(dot3(from, axis));
    let gap = (from_along as f64 - a_along as f64) as f32;
    let cap_fraction = (gap as f64 / axis_length as f64) as f32;
    let from_on_axis = zip_with(from, interpolate(ones, a, b, cap_fraction), |p, q| p - q);
    if entry < 0.0 {
        let distance_from_axis = x87_quiet(dot3(from_on_axis, from_on_axis));
        if radius_squared < distance_from_axis {
            return false;
        }
    }
    // The end point that the ray passes: `m_vertexA` when the point is not
    // above it along the axis, else `m_vertexB`.
    let end: u32 = if height <= 0.0 { 0 } else { 1 };
    let end_point = load_vec(e, me + 0x10 * (end + 2));
    store_vec(e, rays_address, zip_with(from, end_point, |p, q| p - q));
    store_vec(
        e,
        rays_address + 0x10,
        zip_with(to, end_point, |p, q| p - q),
    );
    // Two words after the rays are cleared as well (never read here).
    e.mem.set_u32(rays_address + 0x20, 0);
    e.mem.set_u32(rays_address + 0x24, 0);
    let flag = e
        .call(
            SPHERE_CAST_RAY_UTIL,
            &args![flag_address, x87_quiet(radius), rays_address, output],
        )
        .u32();
    if e.mem.u8(flag) != 0 {
        e.mem.set_u32(out + OUT_EXTRA_INFO, end);
        true
    } else {
        false
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d22350,
            hkp_capsule_shape_get_num_collision_spheres_impl(Ptr<HkpCapsuleShape>) -> u32
        ),
        entry!(
            0x00d22360,
            hkp_capsule_shape_calc_content_statistics(Ptr<HkpCapsuleShape>, u32, u32)
        ),
        entry!(
            0x00d22370,
            hkp_capsule_shape_get_supporting_vertex_impl(Ptr<HkpCapsuleShape>, Ptr, Ptr)
        ),
        entry!(
            0x00d223f0,
            hkp_capsule_shape_convert_vertex_ids_to_vertices_impl(
                Ptr<HkpCapsuleShape>,
                Ptr,
                i32,
                Ptr,
            )
        ),
        entry!(
            0x00d22440,
            hkp_capsule_shape_get_centre_impl(Ptr<HkpCapsuleShape>, Ptr)
        ),
        entry!(
            0x00d22480,
            hkp_capsule_shape_get_first_vertex(Ptr<HkpCapsuleShape>, Ptr)
        ),
        entry!(
            0x00d224a0,
            hkp_capsule_shape_get_collision_spheres_impl(Ptr<HkpCapsuleShape>, Ptr) -> Ptr
        ),
        entry!(0x00d224c0, fn_00d224c0(Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00d22590,
            hkp_capsule_shape_hkp_capsule_shape(
                Ptr<HkpCapsuleShape>,
                Ptr,
                Ptr,
                f32,
            ) -> Ptr<HkpCapsuleShape>
        ),
        entry!(
            0x00d225e0,
            hkp_capsule_shape_get_aabb_impl(Ptr<HkpCapsuleShape>, Ptr, f32, Ptr)
        ),
        entry!(
            0x00d22700,
            fn_00d22700(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d22760,
            hkp_capsule_shape_cast_ray_impl(Ptr<HkpCapsuleShape>, Ptr, Ptr, Ptr) -> Ptr
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
        e.set_global(HALF, 0.5f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(EPSILON, f32::from_bits(0x3400_0000));
        for lane in 0..4 {
            e.mem.set_f32(ONES + 4 * lane, 1.0);
        }
        e
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn vector_at(e: &mut Engine, v: Vec4) -> Ptr {
        let address = e.mem.alloc(16);
        store_vec(e, address, v);
        Ptr::new(address)
    }

    /// A capsule from `(0,0,0)` to `(0,0,2)` with radius 1.
    fn capsule(e: &mut Engine) -> Ptr<HkpCapsuleShape> {
        let this = e.new_object::<HkpCapsuleShape>();
        store_vec(e, this.addr() + VERTEX_A, [0.0, 0.0, 0.0, 1.0]);
        store_vec(e, this.addr() + VERTEX_B, [0.0, 0.0, 2.0, 1.0]);
        e.set(this, HkpCapsuleShape::m_radius, 1.0);
        this
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn assert_near(actual: Vec4, expected: Vec4, tolerance: f32) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() <= tolerance,
                "{actual:?} != {expected:?}"
            );
        }
    }

    #[test]
    fn num_collision_spheres_is_two() {
        let mut e = engine();
        let this = capsule(&mut e);
        assert_eq!(e.call(0x00d22350, &args![this]).u32(), 2);
    }

    #[test]
    fn calc_content_statistics_passes_the_capsule_class() {
        let mut e = engine();
        let this = capsule(&mut e);
        e.register(REFERENCED_OBJECT_CALC_CONTENT_STATISTICS, |_, _| {
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00d22360, &args![this, 0x1111u32, 0x2222u32]);
        assert_eq!(
            calls_to(&e, REFERENCED_OBJECT_CALC_CONTENT_STATISTICS),
            vec![vec![this.addr(), 0x1111, 0x0126_8940]]
        );
    }

    #[test]
    fn supporting_vertex_picks_the_end_along_the_direction() {
        let mut e = engine();
        let this = capsule(&mut e);
        let out = e.mem.alloc(16);
        // Along +z the far end B (id 0x10); against it A (id 0).
        let up = vector_at(&mut e, [0.0, 0.0, 1.0, 0.0]);
        e.call(0x00d22370, &args![this, up, out]);
        assert_eq!(load_vec(&e, out)[..3], [0.0, 0.0, 2.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0010);
        let down = vector_at(&mut e, [0.0, 0.0, -1.0, 0.0]);
        e.call(0x00d22370, &args![this, down, out]);
        assert_eq!(load_vec(&e, out)[..3], [0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0000);
        // Zero and NaN directions choose B.
        let zero = vector_at(&mut e, [0.0; 4]);
        e.call(0x00d22370, &args![this, zero, out]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0010);
        let nan = vector_at(&mut e, [0.0, 0.0, f32::NAN, 0.0]);
        e.call(0x00d22370, &args![this, nan, out]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0010);
    }

    #[test]
    fn vertex_ids_select_vectors_by_byte_offset() {
        let mut e = engine();
        let this = capsule(&mut e);
        let ids = e.mem.alloc(8);
        e.mem.set_u16(ids, 0x10);
        e.mem.set_u16(ids + 2, 0);
        let out = e.mem.alloc(0x20);
        e.call(0x00d223f0, &args![this, ids, 2i32, out]);
        assert_eq!(load_vec(&e, out)[..3], [0.0, 0.0, 2.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0010);
        assert_eq!(load_vec(&e, out + 0x10)[..3], [0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(out + 0x1c), 0x3f00_0000);
        // A count of zero (or less) writes nothing.
        let untouched = e.mem.alloc(0x10);
        e.call(0x00d223f0, &args![this, ids, 0i32, untouched]);
        e.call(0x00d223f0, &args![this, ids, -3i32, untouched]);
        assert_eq!(e.mem.u32(untouched + 0xc), 0);
    }

    #[test]
    fn centre_is_half_the_sum_of_the_ends() {
        let mut e = engine();
        let this = capsule(&mut e);
        let out = e.mem.alloc(16);
        e.call(0x00d22440, &args![this, out]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 1.0, 1.0]);
    }

    #[test]
    fn first_vertex_is_the_second_end() {
        let mut e = engine();
        let this = capsule(&mut e);
        let out = e.mem.alloc(16);
        e.call(0x00d22480, &args![this, out]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 2.0, 1.0]);
    }

    #[test]
    fn collision_spheres_answer_the_buffer() {
        let mut e = engine();
        let this = capsule(&mut e);
        let out = e.mem.alloc(32);
        // EAX is `out` at the RET (00d224bb); the convex transform shape reads it (00d5df2f).
        assert_eq!(e.call(0x00d2_24a0, &args![this, out]).u32(), out);
    }

    #[test]
    fn collision_spheres_are_both_ends() {
        let mut e = engine();
        let this = capsule(&mut e);
        let out = e.mem.alloc(32);
        e.call(0x00d224a0, &args![this, out]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(load_vec(&e, out + 0x10), [0.0, 0.0, 2.0, 1.0]);
    }

    #[test]
    fn closest_point_on_a_segment() {
        let mut e = engine();
        let a = vector_at(&mut e, [0.0, 0.0, 0.0, 1.0]);
        let b = vector_at(&mut e, [0.0, 0.0, 2.0, 3.0]);
        let out = e.mem.alloc(16);
        // Before A, after B and in between (the w lane interpolates too).
        let before = vector_at(&mut e, [4.0, 0.0, -3.0, 0.0]);
        e.call(0x00d224c0, &args![before, a, b, out]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 0.0, 1.0]);
        let after = vector_at(&mut e, [4.0, 0.0, 9.0, 0.0]);
        e.call(0x00d224c0, &args![after, a, b, out]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 2.0, 3.0]);
        let beside = vector_at(&mut e, [4.0, 0.0, 0.5, 0.0]);
        e.call(0x00d224c0, &args![beside, a, b, out]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 0.5, 1.5]);
        // A NaN parameter takes the interpolating case.
        let nan = vector_at(&mut e, [f32::NAN, 0.0, 0.5, 0.0]);
        e.call(0x00d224c0, &args![nan, a, b, out]);
        assert!(load_vec(&e, out)[0].is_nan());
    }

    #[test]
    fn constructor_fills_the_capsule() {
        let mut e = engine();
        let this = e.new_object::<HkpCapsuleShape>();
        let a = vector_at(&mut e, [1.0, 2.0, 3.0, 9.0]);
        let b = vector_at(&mut e, [4.0, 5.0, 6.0, 9.0]);
        let result = e.call(0x00d22590, &args![this, a, b, 0.25f32]).ptr::<()>();
        assert_eq!(result.addr(), this.addr());
        assert_eq!(e.get(this, HkpCapsuleShape::vtable), 0x010c_e25c);
        assert_eq!(e.get(this, HkpCapsuleShape::m_referenceCount), 1);
        assert_eq!(e.get(this, HkpCapsuleShape::m_userData), 0);
        assert_eq!(e.get(this, HkpCapsuleShape::m_type), 5);
        assert_eq!(e.get(this, HkpCapsuleShape::m_radius), 0.25);
        assert_eq!(load_vec(&e, this.addr() + VERTEX_A), [1.0, 2.0, 3.0, 0.25]);
        assert_eq!(load_vec(&e, this.addr() + VERTEX_B), [4.0, 5.0, 6.0, 0.25]);
    }

    #[test]
    fn aabb_encloses_the_transformed_ends_and_grows_by_radius_and_tolerance() {
        let mut e = engine();
        let this = capsule(&mut e);
        // Rotation columns (x -> y, y -> -x, z -> z) and a translation.
        let transform = e.mem.alloc(0x40);
        store_vec(&mut e, transform, [0.0, 1.0, 0.0, 0.0]);
        store_vec(&mut e, transform + 0x10, [-1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, transform + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, transform + 0x30, [10.0, 20.0, 30.0, 0.0]);
        let out = e.mem.alloc(0x20);
        e.call(0x00d225e0, &args![this, transform, 0.5f32, out]);
        // Ends map to (10,20,30) and (10,20,32); margin 1.5.
        assert_eq!(load_vec(&e, out)[..3], [8.5, 18.5, 28.5]);
        assert_eq!(load_vec(&e, out + 0x10)[..3], [11.5, 21.5, 33.5]);
    }

    #[test]
    fn closest_points_wrapper_distributes_the_helper_result() {
        let mut e = engine();
        e.register(CLOSEST_POINTS, |e, a| {
            assert_eq!(a.len(), 5);
            let result = a[4];
            e.mem.set_u32(result, 0x7f80_0001); // signalling NaN: quieted on copy
            e.mem.set_f32(result + 4, 2.0);
            e.mem.set_f32(result + 8, 3.0);
            store_vec(e, result + 0x10, [1.0, 2.0, 3.0, 4.0]);
            store_vec(e, result + 0x20, [5.0, 6.0, 7.0, 8.0]);
            Ret::default()
        });
        let words = e.mem.alloc(0x40);
        let (zero, one, two) = (words, words + 4, words + 8);
        let (vector_a, vector_b) = (words + 0x10, words + 0x20);
        e.call(
            0x00d22700,
            &args![1u32, 2u32, 3u32, 4u32, zero, one, two, vector_a, vector_b],
        );
        assert_eq!(e.mem.u32(zero), 0x7fc0_0001);
        assert_eq!(e.mem.f32(one), 2.0);
        assert_eq!(e.mem.f32(two), 3.0);
        assert_eq!(load_vec(&e, vector_a), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, vector_b), [5.0, 6.0, 7.0, 8.0]);
    }

    // ---- castRayImpl

    /// A profile stream with room for two records; the TLS lookup returns it.
    fn profile_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 3);
            ret(stream)
        });
        stream
    }

    struct Cast {
        hit: bool,
        output: u32,
        stream: u32,
    }

    /// Casts `from` -> `to` at the capsule with `hit_fraction` already in
    /// the output; `distance_squared` and `parameter` are what the doubled
    /// closest-points helper reports.
    fn cast(
        e: &mut Engine,
        from: Vec4,
        to: Vec4,
        hit_fraction: f32,
        distance_squared: f32,
        parameter: f32,
    ) -> Cast {
        let this = capsule(e);
        let stream = profile_stream(e);
        let results = Rc::new(RefCell::new((distance_squared, parameter)));
        let shared = Rc::clone(&results);
        e.register_double(CLOSEST_POINTS, move |e, a| {
            let (distance, parameter) = *shared.borrow();
            e.mem.set_f32(a[4], distance);
            e.mem.set_f32(a[4] + 4, parameter);
            Ret::default()
        });
        let input = e.mem.alloc(0x30);
        store_vec(e, input, from);
        store_vec(e, input + 0x10, to);
        let output = e.mem.alloc(0x50);
        e.mem.set_f32(output + 0x10, hit_fraction);
        e.mem.set_u32(output + 0x40, 3);
        let result = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let returned = e
            .call(0x00d22760, &args![this, result, input, output])
            .u32();
        assert_eq!(returned, result);
        Cast {
            hit: e.mem.u8(result) != 0,
            output,
            stream,
        }
    }

    fn assert_profile_records(e: &Engine, stream: u32) {
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_CAPSULE);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x38);
    }

    #[test]
    fn cast_ray_misses_when_it_starts_inside() {
        let mut e = engine();
        let c = cast(
            &mut e,
            [0.5, 0.0, 1.0, 0.0],
            [-5.0, 0.0, 1.0, 0.0],
            1.0,
            0.0,
            0.5,
        );
        assert!(!c.hit);
        assert!(calls_to(&e, CLOSEST_POINTS).is_empty());
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_misses_when_the_lines_are_too_far_apart() {
        let mut e = engine();
        let c = cast(
            &mut e,
            [3.0, 0.0, 5.0, 0.0],
            [3.0, 1.0, 4.0, 0.0],
            1.0,
            9.0,
            0.5,
        );
        assert!(!c.hit);
        assert_eq!(calls_to(&e, CLOSEST_POINTS).len(), 1);
        assert!(calls_to(&e, SPHERE_CAST_RAY_UTIL).is_empty());
    }

    #[test]
    fn cast_ray_hits_the_cylinder_part() {
        let mut e = engine();
        let c = cast(
            &mut e,
            [5.0, 0.0, 1.0, 0.0],
            [-5.0, 0.0, 1.0, 0.0],
            1.0,
            0.0,
            0.5,
        );
        assert!(c.hit);
        assert!((e.mem.f32(c.output + 0x10) - 0.4).abs() < 1e-5);
        let normal = load_vec(&e, c.output);
        assert_near(normal, [1.0, 0.0, 0.0, -1.0], 1e-5);
        assert_eq!(e.mem.u32(c.output + 0x14), 2);
        assert_eq!(e.mem.u32(c.output + 0x20 + 3 * 4), 0xffff_ffff);
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_ignores_a_hit_farther_than_the_one_in_the_output() {
        let mut e = engine();
        let c = cast(
            &mut e,
            [5.0, 0.0, 1.0, 0.0],
            [-5.0, 0.0, 1.0, 0.0],
            0.3,
            0.0,
            0.5,
        );
        assert!(!c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 0.3);
        assert_eq!(e.mem.u32(c.output + 0x14), 0);
    }

    /// Doubles the sphere cast: records its rays and reports `hit`.
    fn sphere_double(e: &mut Engine, hit: bool) -> Rc<RefCell<Vec<Vec4>>> {
        let rays = Rc::new(RefCell::new(Vec::new()));
        let shared = Rc::clone(&rays);
        e.register_double(SPHERE_CAST_RAY_UTIL, move |e, a| {
            shared.borrow_mut().push(load_vec(e, a[2]));
            shared.borrow_mut().push(load_vec(e, a[2] + 0x10));
            e.mem.set_u8(a[0], hit as u8);
            ret(a[0])
        });
        rays
    }

    #[test]
    fn cast_ray_tests_the_end_cap_with_the_sphere_cast() {
        let mut e = engine();
        let rays = sphere_double(&mut e, true);
        // Crosses the axis line above the capsule's end B.
        let c = cast(
            &mut e,
            [3.0, 0.0, 5.0, 0.0],
            [-3.0, 0.0, 5.0, 0.0],
            1.0,
            0.0,
            0.5,
        );
        assert!(c.hit);
        assert_eq!(e.mem.u32(c.output + 0x14), 1);
        let sphere_calls = calls_to(&e, SPHERE_CAST_RAY_UTIL);
        assert_eq!(sphere_calls.len(), 1);
        assert_eq!(sphere_calls[0][1], 1.0f32.to_bits());
        assert_eq!(sphere_calls[0][3], c.output);
        // The rays are relative to B = (0,0,2) with radius 1 in `w`.
        assert_eq!(rays.borrow()[0], [3.0, 0.0, 3.0, -1.0]);
        assert_eq!(rays.borrow()[1], [-3.0, 0.0, 3.0, -1.0]);
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_reports_a_miss_when_the_sphere_cast_misses() {
        let mut e = engine();
        sphere_double(&mut e, false);
        let c = cast(
            &mut e,
            [3.0, 0.0, 5.0, 0.0],
            [-3.0, 0.0, 5.0, 0.0],
            1.0,
            0.0,
            0.5,
        );
        assert!(!c.hit);
        assert_eq!(e.mem.u32(c.output + 0x14), 0);
    }

    #[test]
    fn cast_ray_below_the_capsule_tests_the_first_end() {
        let mut e = engine();
        let rays = sphere_double(&mut e, true);
        let c = cast(
            &mut e,
            [3.0, 0.0, -4.0, 0.0],
            [-3.0, 0.0, -4.0, 0.0],
            1.0,
            0.0,
            0.5,
        );
        assert!(c.hit);
        assert_eq!(e.mem.u32(c.output + 0x14), 0);
        // Relative to A = (0,0,0).
        assert_eq!(rays.borrow()[0], [3.0, 0.0, -4.0, -1.0]);
    }

    #[test]
    fn cast_ray_without_a_direction_has_no_cylinder_entry() {
        let mut e = engine();
        sphere_double(&mut e, true);
        // from == to: the entry fraction is -1.0, so the end cap decides.
        let c = cast(
            &mut e,
            [0.5, 0.0, 5.0, 0.0],
            [0.5, 0.0, 5.0, 0.0],
            1.0,
            0.0,
            0.5,
        );
        assert!(c.hit);
        assert_eq!(e.mem.u32(c.output + 0x14), 1);
        assert_eq!(e.mem.f32(c.output + 0x10), 1.0);
    }

    #[test]
    fn cast_ray_that_enters_behind_its_start_and_is_far_from_the_axis_misses() {
        let mut e = engine();
        sphere_double(&mut e, true);
        // The cylinder is entered at parameter -8, before the ray starts,
        // and the start is 3 away from the axis point below it.
        let c = cast(
            &mut e,
            [3.0, 0.0, 5.0, 0.0],
            [3.5, 0.0, 4.0, 0.0],
            1.0,
            0.0,
            -6.0,
        );
        assert!(!c.hit);
        assert!(calls_to(&e, SPHERE_CAST_RAY_UTIL).is_empty());
    }

    #[test]
    fn cast_ray_that_enters_behind_its_start_near_the_axis_uses_the_sphere_cast() {
        let mut e = engine();
        sphere_double(&mut e, true);
        let c = cast(
            &mut e,
            [0.0, 0.0, 5.0, 0.0],
            [0.5, 0.0, 4.0, 0.0],
            1.0,
            0.0,
            0.0,
        );
        assert!(c.hit);
        assert_eq!(calls_to(&e, SPHERE_CAST_RAY_UTIL).len(), 1);
    }

    #[test]
    fn profile_record_is_skipped_when_the_stream_is_full() {
        let mut e = engine();
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x20);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| ret(stream));
        profile_record(&mut e, 0x1234);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x20);
        assert_eq!(e.mem.u32(stream + 0x20), 0);
    }
}

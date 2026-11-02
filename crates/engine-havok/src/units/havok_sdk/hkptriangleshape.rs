//! `Havok SDK/hkptriangleshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpTriangleShape` is a triangle with the corners `m_vertexA` (+0x20),
//! `m_vertexB` (+0x30) and `m_vertexC` (+0x40), an extrusion vector
//! `m_extrusion` (+0x50) and the radius `m_radius` (+0x10). When
//! `m_isExtruded` (+0x17) is set the shape is the prism of the three
//! corners and the same corners moved by the extrusion (six vertices).
//! SSE code computes lane by lane in `f32`, a dot product is `z + (y + x)`
//! the way the `SHUFPS`/`ADDSS` sequences add it; x87 code rounds at each
//! `float` store and compares the way its `FNSTSW`/`TEST AH` pairs read the
//! flags (NaN included). `RCPPS` and `RSQRTSS` are the host CPU's estimates.
//!
//! Two helpers (`00d1ff80`, `00d201a0`) are register-convention functions:
//! their parameters are listed in the order the registers and the stack
//! words are read (`ECX`, `EAX`, then the pushed words), and the caller
//! pops the pushed words.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkpsphereshape::{
    copy_vec, dot3, inverse_length, load_bits, load_vec, mask_of, maxps, minps, movhlps, movlhps,
    profile_record, refined_reciprocal, shufps, store_bits, store_vec, unpckhps, unpcklps,
    zip_with, Vec4, LANE_BITS, ONES, OUT_HIT_FRACTION, OUT_SHAPE_KEYS, OUT_SHAPE_KEY_INDEX,
    PROFILE_END, SIGN_MASK,
};

layout! {
    /// `hkpTriangleShape` (Xbox PDB), 0x60 bytes.
    pub struct HkpTriangleShape: 0x60 {
        /// The vtable.
        0x00 vtable: u32,
        /// `m_radius` (Xbox PDB).
        0x10 m_radius: f32,
        /// `m_weldingInfo` (Xbox PDB): five bits per vertex.
        0x14 m_weldingInfo: u16,
        /// `m_weldingType` (Xbox PDB).
        0x16 m_weldingType: u8,
        /// `m_isExtruded` (Xbox PDB).
        0x17 m_isExtruded: u8,
        // +0x20 `m_vertexA`, +0x30 `m_vertexB`, +0x40 `m_vertexC` and
        // +0x50 `m_extrusion` (Xbox PDB): `hkVector4`s.
    }
}

const VERTEX_A: u32 = 0x20;
const VERTEX_B: u32 = 0x30;
const VERTEX_C: u32 = 0x40;
const EXTRUSION: u32 = 0x50;

/// `hkVector4::setTransformedPos` (Xbox PDB, `this` = out; `(transform,
/// vector)`).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `hkVector4::setTransformedInversePos` (Xbox PDB, `this` = out;
/// `(transform, vector)`).
const SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
/// `hkVector4::setRotatedDir` (Xbox PDB, `this` = out; `(transform,
/// vector)`).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// The `float` minimum `weldContactPointImpl` takes (cdecl, two `float`
/// words `(a, b)`): `a` when `b > a`, else `b` (`00c9e0f0`; the engine map
/// has no name for it).
const FLOAT_MINIMUM: u32 = 0x00c9_e0f0;

/// `float` one third (`getCentreImpl`).
const ONE_THIRD: u32 = 0x0106_c2e0;
/// `-0.0001` as a `double`: `castRayImpl`'s edge test threshold factor.
const EDGE_THRESHOLD_FACTOR: u32 = 0x010c_b430;
/// Four 1.0 (the copy `castRayImpl` reads).
const ONES_CAST_RAY: u32 = 0x010e_c7f0;
/// Four all-ones dwords (a mask that keeps everything).
const ALL_ONES: u32 = 0x010c_e1c0;
/// `float` constant of `castRayBundleImpl` (the edge test threshold factor).
const BUNDLE_THRESHOLD_FACTOR: u32 = 0x010c_e1e0;
/// Profiling labels: the begin records of `castRayImpl` and
/// `castRayBundleImpl`.
const PROFILE_BEGIN_CAST_RAY: u32 = 0x010c_e1d0;
const PROFILE_BEGIN_CAST_RAY_BUNDLE: u32 = 0x010c_e1e4;
/// Tables of the shape: the `u32` vertex index table at `011b3b4c` (the
/// next-index view of it starts one entry later, at `011b3b50`), the
/// `float` table at `011b3b20` (extrusion factors by vertex id) and the one
/// at `011b3b38` (indexed by welding type).
const VERTEX_INDEX_TABLE: u32 = 0x011b_3b4c;
const NEXT_VERTEX_INDEX_TABLE: u32 = 0x011b_3b50;
const EXTRUSION_FACTOR_TABLE: u32 = 0x011b_3b20;
const WELD_FACTOR_TABLE: u32 = 0x011b_3b38;
/// The welding table: one row of six `float`s (24 bytes) per feature.
const WELDING_ROWS: u32 = 0x0126_9210;

/// `hkReferencedObject::calcContentStatistics` (`this`, collector, class record).
const REFERENCED_OBJECT_CALC_CONTENT_STATISTICS: u32 = 0x00c9_0360;
/// The class record (`01268940`) that `calcContentStatistics` hands to the base report.
const TRIANGLE_CLASS_RECORD: u32 = 0x0126_8940;

// Translated from 00d1fee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriangleShape::calcContentStatistics` (Xbox PDB): tail-calls
/// `hkReferencedObject::calcContentStatistics` with the collector and the class record
/// `01268940` in place of the second argument, which it never reads.
pub fn hkp_triangle_shape_calc_content_statistics(
    e: &mut Engine,
    this: Ptr<HkpTriangleShape>,
    collector: u32,
    _unused: u32,
) {
    e.call(
        REFERENCED_OBJECT_CALC_CONTENT_STATISTICS,
        &args![this, collector, TRIANGLE_CLASS_RECORD],
    );
}

// Translated from 00d1ff10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriangleShape::getNumCollisionSpheresImpl` (Xbox PDB): three
/// spheres, six when `m_isExtruded` is set (`(m_isExtruded + 1) * 3`).
pub fn hkp_triangle_shape_get_num_collision_spheres_impl(
    e: &mut Engine,
    this: Ptr<HkpTriangleShape>,
) -> u32 {
    (e.get(this, HkpTriangleShape::m_isExtruded) as u32 + 1) * 3
}

// Translated from 00d1ff20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriangleShape::getFirstVertex` (Xbox PDB): copies `m_vertexA`.
pub fn hkp_triangle_shape_get_first_vertex(e: &mut Engine, this: Ptr<HkpTriangleShape>, out: Ptr) {
    copy_vec(e, this.addr() + VERTEX_A, out.addr());
}

// Translated from 00d1ff40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriangleShape::getCentreImpl` (Xbox PDB): the centroid, `(1/3) * (C +
/// (B + A))` in every lane (the intermediate sums are stored to `out` on
/// the way, which only matters if it aliases the shape).
pub fn hkp_triangle_shape_get_centre_impl(e: &mut Engine, this: Ptr<HkpTriangleShape>, out: Ptr) {
    let a = load_vec(e, this.addr() + VERTEX_A);
    let b = load_vec(e, this.addr() + VERTEX_B);
    let first = zip_with(b, a, |p, q| p + q);
    store_vec(e, out.addr(), first);
    let c = load_vec(e, this.addr() + VERTEX_C);
    let second = zip_with(c, first, |p, q| p + q);
    store_vec(e, out.addr(), second);
    let third: f32 = e.global(ONE_THIRD);
    store_vec(e, out.addr(), second.map(|lane| third * lane));
}

/// `shuf_c9(x) * shuf_d2(y) - shuf_d2(x) * shuf_c9(y)`, the cross product
/// the code builds with `SHUFPS` (`0xc9` picks lanes 1, 2, 0, 3 and `0xd2`
/// lanes 2, 0, 1, 3).
fn cross(x: Vec4, y: Vec4) -> Vec4 {
    let left = zip_with(shufps(x, x, 0xc9), shufps(y, y, 0xd2), |p, q| p * q);
    let right = zip_with(shufps(x, x, 0xd2), shufps(y, y, 0xc9), |p, q| p * q);
    zip_with(left, right, |p, q| p - q)
}

/// `normal` scaled by the refined `RSQRTSS` of its squared length.
fn normalised(e: &Engine, v: Vec4) -> Vec4 {
    let inverse = inverse_length(e, dot3(v, v));
    v.map(|lane| inverse * lane)
}

// Translated from 00d1ff80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name (register convention: `a` in `ECX`, `b` in `EAX`, then
/// two pushed words; the caller pops them): writes to `out` the unit normal
/// of the triangle `a`, `b`, `c`: `(b - a) x (c - b)` scaled by the refined
/// `RSQRTSS` of its squared length (zero when that is zero).
pub fn fn_00d1ff80(e: &mut Engine, a: Ptr, b: Ptr, c: Ptr, out: Ptr) {
    let av = load_vec(e, a.addr());
    let bv = load_vec(e, b.addr());
    let cv = load_vec(e, c.addr());
    let ab = zip_with(bv, av, |p, q| p - q);
    let bc = zip_with(cv, bv, |p, q| p - q);
    let normal = normalised(e, cross(ab, bc));
    store_vec(e, out.addr(), normal);
}

// Translated from 00d20110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriangleShape::getCollisionSpheresImpl` (Xbox PDB): writes the
/// spheres to `out`, each with `m_radius` in its `w` lane (through
/// `FLD`/`FSTP`): the corners A, B and C, and when `m_isExtruded` is set
/// `extrusion + A`, `extrusion + B` and `C + extrusion`.
/// Answers `out` (the `const hkSphere*` of the Xbox PDB signature; `EAX` is `out` at the
/// RET 00d20193): hkpConvexTransformShape::getCollisionSpheresImpl reads it through its child
/// shape's slot +0x30 (`MOV EDI,EAX` at 00d5df2f).
pub fn hkp_triangle_shape_get_collision_spheres_impl(
    e: &mut Engine,
    this: Ptr<HkpTriangleShape>,
    out: Ptr,
) -> Ptr {
    let me = this.addr();
    let radius = x87_quiet(e.get(this, HkpTriangleShape::m_radius));
    let a = load_vec(e, me + VERTEX_A);
    let b = load_vec(e, me + VERTEX_B);
    let c = load_vec(e, me + VERTEX_C);
    let sphere = |e: &mut Engine, index: u32, centre: Vec4| {
        store_vec(e, out.addr() + 0x10 * index, centre);
        e.mem.set_f32(out.addr() + 0x10 * index + 0xc, radius);
    };
    sphere(e, 0, a);
    sphere(e, 1, b);
    sphere(e, 2, c);
    if e.get(this, HkpTriangleShape::m_isExtruded) != 0 {
        let extrusion = load_vec(e, me + EXTRUSION);
        // `ADDPS` with the SSE NaN rules, the destination operand first.
        use crate::sse::addps;
        sphere(e, 3, addps(extrusion, a));
        sphere(e, 4, addps(extrusion, b));
        sphere(e, 5, addps(c, extrusion));
    }
    out
}

/// The edge function `00d201a0` computes for four points at once: for each
/// lane `l` of `s` (a structure-of-arrays set of four points), with `d = a
/// - s[l]` and `v = b - s[l]`, the dot product of `d x v` with `n` as `n.z
/// * c3 + (n.y * c2 + (n.x * c1 + 0))` where `c1 = v.z * d.y - d.z * v.y`,
/// `c2 = d.z * v.x - v.z * d.x` and `c3 = v.y * d.x - d.y * v.x`.
fn edge_function(a: Vec4, s: [Vec4; 3], b: Vec4, n: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        let d = [a[0] - s[0][lane], a[1] - s[1][lane], a[2] - s[2][lane]];
        let v = [b[0] - s[0][lane], b[1] - s[1][lane], b[2] - s[2][lane]];
        let c1 = v[2] * d[1] - d[2] * v[1];
        let c2 = d[2] * v[0] - v[2] * d[0];
        let c3 = v[1] * d[0] - d[1] * v[0];
        n[2] * c3 + ((n[1] * c2) + ((n[0] * c1) + 0.0))
    })
}

// Translated from 00d201a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No Xbox PDB name (register convention: `a` in `ECX`, the point set `s`
/// in `EAX`, then three pushed words; the caller pops them): the edge
/// function of the points `s` (three vectors holding the x, y and z of four
/// points) against the edge from `a` to `b` and the triangle normal `n`,
/// four results to `out`. See `edge_function`.
pub fn fn_00d201a0(e: &mut Engine, a: Ptr, s: Ptr, b: Ptr, n: Ptr, out: Ptr) {
    let points: [Vec4; 3] = std::array::from_fn(|axis| load_vec(e, s.addr() + 0x10 * axis as u32));
    let av = load_vec(e, a.addr());
    let bv = load_vec(e, b.addr());
    let nv = load_vec(e, n.addr());
    store_vec(e, out.addr(), edge_function(av, points, bv, nv));
}

// Translated from 00d20270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriangleShape::getSupportingVertexImpl` (Xbox PDB): of the corners
/// A, B and C the one furthest along `direction` (each dot product is
/// `(y * cy + x * cx) + z * cz`; C beats B only when B is strictly less,
/// and A is chosen only when the better of those two is strictly below
/// it). The vertex is written to `out`; when `direction . m_extrusion` is
/// positive the extrusion is added to it and 3 added to the vertex id. The
/// `w` lane of `out` is `0x3f000000 | id`.
pub fn hkp_triangle_shape_get_supporting_vertex_impl(
    e: &mut Engine,
    this: Ptr<HkpTriangleShape>,
    direction: Ptr,
    out: Ptr,
) {
    let me = this.addr();
    let a = load_vec(e, me + VERTEX_A);
    let b = load_vec(e, me + VERTEX_B);
    let c = load_vec(e, me + VERTEX_C);
    let dir = load_vec(e, direction.addr());
    let along = |v: Vec4| (dir[1] * v[1] + dir[0] * v[0]) + dir[2] * v[2];
    let dots = [along(a), along(b), along(c)];
    // `FCOM` of B against C: B unless B < C (then C); then A only when the
    // best of those is below A.
    let (mut id, best) = if dots[1] < dots[2] {
        (2u32, dots[2])
    } else {
        (1u32, dots[1])
    };
    if best < dots[0] {
        id = 0;
    }
    copy_vec(e, me + VERTEX_A + 0x10 * id, out.addr());
    let extrusion = load_vec(e, me + EXTRUSION);
    let toward = dot3(dir, extrusion);
    if 0.0 < toward {
        let vertex = load_vec(e, out.addr());
        store_vec(e, out.addr(), zip_with(extrusion, vertex, |p, q| p + q));
        id += 3;
    }
    e.mem.set_u32(out.addr() + 0xc, id | 0x3f00_0000);
}

// Translated from 00d20370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriangleShape::convertVertexIdsToVerticesImpl` (Xbox PDB): for each
/// of `count` 16-bit vertex ids, writes to `out` the corner whose index is
/// the id's entry in the vertex index table (`011b3b4c`) plus the extrusion
/// times the id's entry in the factor table (`011b3b20`), with the `w` lane
/// `0x3f000000 | id`. Nothing is written for `count <= 0`.
pub fn hkp_triangle_shape_convert_vertex_ids_to_vertices_impl(
    e: &mut Engine,
    this: Ptr<HkpTriangleShape>,
    ids: Ptr,
    count: i32,
    out: Ptr,
) {
    let me = this.addr();
    let mut id_address = ids.addr();
    let mut out_address = out.addr();
    for _ in 0..count.max(0) {
        let id = e.mem.u16(id_address) as u32;
        let index = e.mem.u32(VERTEX_INDEX_TABLE + 4 * id);
        let vertex = load_vec(e, me.wrapping_add(index.wrapping_add(2) << 4));
        let factor = e.mem.f32(EXTRUSION_FACTOR_TABLE + 4 * id);
        let extrusion = load_vec(e, me + EXTRUSION);
        store_vec(
            e,
            out_address,
            zip_with(extrusion, vertex, |step, start| factor * step + start),
        );
        e.mem.set_u32(out_address + 0xc, id | 0x3f00_0000);
        out_address = out_address.wrapping_add(0x10);
        id_address = id_address.wrapping_add(2);
    }
}

// Translated from 00d20050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriangleShape::getAabbImpl` (Xbox PDB): transforms the three corners
/// with `hkVector4::setTransformedPos` (`00c7f640`), takes their lane-wise
/// minimum and maximum (`MINPS`/`MAXPS`), widens the box by the extrusion
/// (`min(min, extrusion + min)` and `max(max, extrusion + max)`) and then by
/// `m_radius + tolerance` (rounded to `float`) on every lane.
pub fn hkp_triangle_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpTriangleShape>,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let me = this.addr();
    let radius = e.get(this, HkpTriangleShape::m_radius);
    let margin = (radius as f64 + tolerance as f64) as f32;
    let corners = e.with_stack(0x30, |e, scratch| {
        for (index, vertex) in [VERTEX_A, VERTEX_B, VERTEX_C].into_iter().enumerate() {
            let target = scratch.addr() + 0x10 * index as u32;
            e.call(SET_TRANSFORMED_POS, &args![target, transform, me + vertex]);
        }
        std::array::from_fn::<Vec4, 3, _>(|index| load_vec(e, scratch.addr() + 0x10 * index as u32))
    });
    let extrusion = load_vec(e, me + EXTRUSION);
    let highest = zip_with(zip_with(corners[0], corners[1], maxps), corners[2], maxps);
    let lowest = zip_with(zip_with(corners[0], corners[1], minps), corners[2], minps);
    let lowest = zip_with(lowest, zip_with(extrusion, lowest, |p, q| p + q), minps);
    let highest = zip_with(highest, zip_with(extrusion, highest, |p, q| p + q), maxps);
    store_vec(e, out.addr(), lowest.map(|lane| lane - margin));
    store_vec(e, out.addr() + 0x10, highest.map(|lane| lane + margin));
}

// Translated from 00d20a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriangleShape::castRayImpl` (Xbox PDB): casts the ray `input`
/// (`hkpShapeRayCastInput`: `m_from`, `m_to`) at the triangle and updates
/// `output` (`hkpShapeRayCastOutput`) when it hits nearer than
/// `output.m_hitFraction`. Writes the hit flag (`hkBool`) to `result` and
/// returns `result`; a profiling record is written first and last.
///
/// With the triangle normal `n` (unnormalised), `u` and `v` the distances
/// of `from` and `to` from the plane (as `float`s) the ray has to cross it:
/// `u * v < 0`, or `u == 0` with `v != 0` (NaN counts as crossing). The
/// fraction is `u / (u - v)` and has to be below the old hit fraction. The
/// hit point has to lie inside the three edges: each edge's triple product
/// with `n` has to be at least `|n|^2 * -0.0001` (a NaN there is a miss).
/// On a hit the fraction and the unit normal (`n`, negated unless `u > 0`)
/// are written, and `-1` into the current shape key slot.
pub fn hkp_triangle_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr<HkpTriangleShape>,
    result: Ptr,
    input: Ptr,
    output: Ptr,
) -> Ptr {
    profile_record(e, PROFILE_BEGIN_CAST_RAY);
    let hit = triangle_cast(e, this.addr(), input.addr(), output.addr());
    profile_record(e, PROFILE_END);
    e.mem.set_u8(result.addr(), hit as u8);
    result
}

fn triangle_cast(e: &mut Engine, me: u32, input: u32, out: u32) -> bool {
    let a = load_vec(e, me + VERTEX_A);
    let b = load_vec(e, me + VERTEX_B);
    let c = load_vec(e, me + VERTEX_C);
    let from = load_vec(e, input);
    let to = load_vec(e, input + 0x10);
    let ab = zip_with(b, a, |p, q| p - q);
    let ac = zip_with(c, a, |p, q| p - q);
    let normal = cross(ab, ac);
    let plane = dot3(a, normal);
    let from_dot = dot3(from, normal);
    let to_dot = dot3(to, normal);
    // x87 differences, stored as `float`s.
    let u = (from_dot as f64 - plane as f64) as f32;
    let v = (to_dot as f64 - plane as f64) as f32;
    let product = u as f64 * v as f64;
    // `FCOMP`: crossing when the product is below zero (or NaN); else only
    // a start exactly on the plane with an end off it continues.
    let crossing = product < 0.0 || product.is_nan() || (u == 0.0 && v != 0.0);
    if !crossing {
        return false;
    }
    let difference = (u as f64 - v as f64) as f32;
    let fraction = (u as f64 / difference as f64) as f32;
    let old_fraction = e.mem.f32(out + OUT_HIT_FRACTION);
    if fraction >= old_fraction || fraction.is_nan() || old_fraction.is_nan() {
        return false;
    }
    let normal_squared = dot3(normal, normal);
    let ones = load_vec(e, ONES_CAST_RAY);
    let point: Vec4 =
        std::array::from_fn(|lane| (ones[lane] - fraction) * from[lane] + to[lane] * fraction);
    let to_a = zip_with(a, point, |p, q| p - q);
    let to_b = zip_with(b, point, |p, q| p - q);
    let to_c = zip_with(c, point, |p, q| p - q);
    let factor: f64 = e.global(EDGE_THRESHOLD_FACTOR);
    let threshold = (normal_squared as f64 * factor) as f32;
    // `FCOM`: a miss when the threshold is above the edge value (or NaN).
    let inside_a = dot3(cross(to_a, to_b), normal);
    if !(threshold <= inside_a) {
        return false;
    }
    let inside_b = dot3(cross(to_b, to_c), normal);
    if !(inside_b >= threshold) {
        return false;
    }
    let inside_c = dot3(cross(to_c, to_a), normal);
    if !(inside_c >= threshold) {
        return false;
    }
    e.mem.set_f32(out + OUT_HIT_FRACTION, x87_quiet(fraction));
    // The normal faces the start of the ray: negated unless `u > 0`.
    let facing = if u > 0.0 {
        normal
    } else {
        normal.map(|lane| -lane)
    };
    let key_index = e.mem.u32(out + OUT_SHAPE_KEY_INDEX);
    store_vec(e, out, normalised(e, facing));
    e.mem.set_u32(
        out.wrapping_add(key_index.wrapping_mul(4))
            .wrapping_add(OUT_SHAPE_KEYS),
        0xffff_ffff,
    );
    true
}

// Translated from 00d20e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriangleShape::castRayBundleImpl` (Xbox PDB): casts four rays at
/// once. `input` is an `hkpShapeRayBundleCastInput` in structure-of-arrays
/// form (`from.x`, `from.y`, `from.z`, `to.x`, `to.y`, `to.z`, each a vector
/// of the four rays), `outputs` four `hkpShapeRayCastOutput`s (0x50 bytes
/// apart) and `in_mask` the rays to cast (four dwords). The mask of rays
/// that hit goes to `result` (also returned); each hit ray's output gets the
/// hit fraction, the normal and `-1` in the current shape key slot.
///
/// With `u` and `v` the distances of the ray's start and end from the plane
/// of the triangle, a lane is dropped when `((v != 0) | (u == 0)) & (0 <= v
/// * u)` (so a NaN lane stays); the fraction is `u * recip(u - v)` (one
/// Newton step on `RCPPS`) and has to be below the old hit fraction, and the
/// hit point has to be inside all three edges (`00d201a0` against `a b`,
/// `b c` and `c a`: `threshold <= edge`, threshold `|n|^2` times a `float`
/// constant). The normal written is the triangle normal normalised again for
/// every hit ray in turn (each renormalises the previous result), with the
/// sign of `u` xor-ed in.
pub fn hkp_triangle_shape_cast_ray_bundle_impl(
    e: &mut Engine,
    this: Ptr<HkpTriangleShape>,
    result: Ptr,
    input: Ptr,
    outputs: Ptr,
    in_mask: Ptr,
) -> Ptr {
    profile_record(e, PROFILE_BEGIN_CAST_RAY_BUNDLE);
    let me = this.addr();
    let result = result.addr();
    let outputs = outputs.addr();
    let input = input.addr();
    for word in 0..4 {
        let value = e.mem.u32(in_mask.addr() + 4 * word);
        e.mem.set_u32(result + 4 * word, value);
    }
    let a = load_vec(e, me + VERTEX_A);
    let b = load_vec(e, me + VERTEX_B);
    let c = load_vec(e, me + VERTEX_C);
    let ab = zip_with(b, a, |p, q| p - q);
    let ac = zip_with(c, a, |p, q| p - q);
    let normal = cross(ab, ac);
    let plane = dot3(normal, a);
    let from: [Vec4; 3] = std::array::from_fn(|axis| load_vec(e, input + 0x10 * axis as u32));
    let to: [Vec4; 3] = std::array::from_fn(|axis| load_vec(e, input + 0x30 + 0x10 * axis as u32));
    let lane_ones = load_vec(e, ONES);
    let all_ones = load_bits(e, ALL_ONES);
    let signs = load_bits(e, SIGN_MASK);
    let factor = x87_quiet(e.global::<f32>(BUNDLE_THRESHOLD_FACTOR));

    let mut start_distance = [0.0f32; 4];
    let mut end_distance = [0.0f32; 4];
    for lane in 0..4 {
        // `n.z * z + (n.y * y + (n.x * x + 0))`.
        let along = |p: &[Vec4; 3]| {
            normal[2] * p[2][lane] + (normal[1] * p[1][lane] + (normal[0] * p[0][lane] + 0.0))
        };
        start_distance[lane] = along(&from) - plane;
        end_distance[lane] = along(&to) - plane;
    }
    let threshold = [dot3(normal, normal) * factor; 4];

    let mask_in = load_bits(e, result);
    let hit_fractions: Vec4 =
        std::array::from_fn(|ray| e.mem.f32(outputs + 0x50 * ray as u32 + OUT_HIT_FRACTION));
    let mut fraction = [0.0f32; 4];
    let mut mask = [0u32; 4];
    for lane in 0..4 {
        let u = start_distance[lane];
        let v = end_distance[lane];
        fraction[lane] = refined_reciprocal(u - v) * u;
        let dropped =
            ((all_ones[lane] & !mask_of(v == 0.0)) | mask_of(u == 0.0)) & mask_of(0.0 <= v * u);
        mask[lane] = (!dropped & all_ones[lane])
            & mask_in[lane]
            & mask_of(fraction[lane] < hit_fractions[lane]);
    }
    store_bits(e, result, mask);

    // The hit points and the three edge tests.
    let point: [Vec4; 3] = std::array::from_fn(|axis| {
        std::array::from_fn(|lane| {
            fraction[lane] * to[axis][lane] + (lane_ones[lane] - fraction[lane]) * from[axis][lane]
        })
    });
    let vertices = [a, b, c, a];
    for edge_index in 0..3 {
        let edge = edge_function(
            vertices[edge_index],
            point,
            vertices[edge_index + 1],
            normal,
        );
        for (lane, value) in edge.into_iter().enumerate() {
            mask[lane] &= mask_of(threshold[lane] <= value);
        }
    }
    store_bits(e, result, mask);

    // The normals: each hit ray renormalises the last one and gets it with
    // the sign bit of its start distance xor-ed in.
    let sign_bits = (0..4).fold(0u32, |bits, ray| bits | ((mask[ray] >> 31) << ray));
    let mut running = normal;
    for lane in 0..4 {
        if e.mem.u32(LANE_BITS + 4 * lane as u32) & sign_bits == 0 {
            continue;
        }
        let output = outputs + 0x50 * lane as u32;
        e.mem
            .set_f32(output + OUT_HIT_FRACTION, x87_quiet(fraction[lane]));
        running = normalised(e, running);
        let flipped: [u32; 4] = std::array::from_fn(|index| {
            (start_distance[lane].to_bits() & signs[index]) ^ running[index].to_bits()
        });
        store_bits(e, output, flipped);
        let key_index = e.mem.u32(output + OUT_SHAPE_KEY_INDEX);
        e.mem.set_u32(
            output
                .wrapping_add(key_index.wrapping_mul(4))
                .wrapping_add(OUT_SHAPE_KEYS),
            0xffff_ffff,
        );
    }
    profile_record(e, PROFILE_END);
    Ptr::new(result)
}

/// `(y * cY + x * cX) + z * cZ` with `cX = [c0x, c1x, c2x, c2y]`, `cY =
/// [c0y, c1y, c2y, c2w]` and `cZ = [c0z, c1z, c2z, c2w]`: the lane tables
/// the `UNPCKLPS`/`MOVLHPS`/`MOVHLPS` sequence builds from the three
/// columns of a transform (the inverse rotation of `v`).
pub(crate) fn inverse_rotate(columns: [Vec4; 3], v: Vec4) -> Vec4 {
    let lower = unpcklps(columns[0], columns[1]);
    let upper = unpckhps(columns[0], columns[1]);
    let x_table = movlhps(lower, columns[2]);
    let y_table = shufps(movhlps(x_table, lower), columns[2], 0xd4);
    let z_table = shufps(upper, columns[2], 0xe4);
    std::array::from_fn(|lane| (v[1] * y_table[lane] + v[0] * x_table[lane]) + v[2] * z_table[lane])
}

/// `(shuf_c9(en) * shuf_d2(t) - shuf_d2(en) * shuf_c9(t)) * b + a * t`:
/// the direction `weldContactPointImpl` builds from the edge direction `en`,
/// the triangle normal `t` and the table values `a` and `b`.
fn weld_blend(t: Vec4, en: Vec4, a: f32, b: f32) -> Vec4 {
    let crossed = cross(en, t);
    std::array::from_fn(|lane| crossed[lane] * b + a * t[lane])
}

// Translated from 00d203f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTriangleShape::weldContactPointImpl` (Xbox PDB), seven pushed words:
/// `feature_points` (`u16` vertex ids), `num_feature_points` (a byte),
/// `contact_point` (shape space), `this_transform`, `colliding_shape`,
/// `colliding_transform` and `separating_normal` (a vector whose `w` lane
/// is a distance). Returns 2, 0 or 1 (the exe's own codes; no name is
/// asserted for them): 2 leaves the contact as it is, 0 rejects it and 1
/// accepts it with `separating_normal` rewritten (`w` = the new distance).
///
/// `m_weldingType` 6 returns 2 at once. Up to three feature points are
/// reduced to the edge (two ids, the count byte set to 2); the triangle's
/// unit normal `t`, the separating normal in shape space `wn`, the unit edge
/// direction `en` and the five-bit field of `m_weldingInfo` for the edge's
/// vertex (selecting a row of six `float`s at `01269210`) decide the rest:
/// type 5 picks one of five table columns from `t . wn` and welds against
/// the plane of the edge and a blended direction; the other types compare
/// `t . wn` with column `m_weldingType` of the row, blend a direction from
/// the row's columns `m_weldingType` and `+1`, rotate it by `this_transform`
/// and, for an extruded triangle, take the smaller of a distance to vertex
/// A and the scaled `|t . wn| * distance`, or for a plain one accept only if
/// the contact is on the negative side of the colliding shape's centre.
#[allow(clippy::too_many_arguments)]
pub fn hkp_triangle_shape_weld_contact_point_impl(
    e: &mut Engine,
    this: Ptr<HkpTriangleShape>,
    feature_points: Ptr,
    num_feature_points: Ptr,
    contact_point: Ptr,
    this_transform: Ptr,
    colliding_shape: Ptr,
    colliding_transform: Ptr,
    separating_normal: Ptr,
) -> u32 {
    let me = this.addr();
    let welding_type = e.get(this, HkpTriangleShape::m_weldingType) as u32;
    if welding_type == 6 {
        return 2;
    }
    let points = feature_points.addr();
    let count = e.mem.u8(num_feature_points.addr());
    let first_id = e.mem.u16(points) as u32;
    let mut vertex = e.mem.u32(VERTEX_INDEX_TABLE + 4 * first_id);
    if count > 1 {
        let second_id = e.mem.u16(points + 2) as u32;
        let mut other = e.mem.u32(VERTEX_INDEX_TABLE + 4 * second_id);
        if count == 3 {
            let third_id = e.mem.u16(points + 4) as u32;
            if second_id + third_id + first_id == 3 {
                return 2;
            }
            if other == vertex {
                other = e.mem.u32(VERTEX_INDEX_TABLE + 4 * third_id);
            }
            e.mem.set_u8(num_feature_points.addr(), 2);
            e.mem.set_u16(points, vertex as u16);
            e.mem.set_u16(points + 2, other as u16);
        }
        if e.mem.u32(NEXT_VERTEX_INDEX_TABLE + 4 * vertex) != other {
            vertex = other;
        }
    }
    let separating = separating_normal.addr();
    let normal_in = load_vec(e, separating);
    let transform = this_transform.addr();
    let columns: [Vec4; 3] =
        std::array::from_fn(|index| load_vec(e, transform + 0x10 * index as u32));
    // The separating normal in shape space.
    let local_normal = inverse_rotate(columns, normal_in);
    let triangle_normal = e.with_stack(0x10, |e, scratch| {
        fn_00d1ff80(
            e,
            Ptr::new(me + VERTEX_A),
            Ptr::new(me + VERTEX_B),
            Ptr::new(me + VERTEX_C),
            scratch,
        );
        load_vec(e, scratch.addr())
    });
    let along = x87_quiet(dot3(triangle_normal, local_normal));
    // `FABS`, then `FMUL` by the distance in the `w` lane.
    let scaled = (along.abs() as f64 * normal_in[3] as f64) as f32;
    let next_vertex = e.mem.u32(NEXT_VERTEX_INDEX_TABLE + 4 * vertex);
    let edge_end = load_vec(e, me.wrapping_add(next_vertex.wrapping_add(2) << 4));
    let edge_start = load_vec(e, me.wrapping_add(vertex.wrapping_add(2) << 4));
    let edge_unit = normalised(e, zip_with(edge_end, edge_start, |p, q| p - q));
    let welding_info = e.get(this, HkpTriangleShape::m_weldingInfo) as u32;
    // `AL * 5` in eight bits, then the shift count masked to five bits.
    let shift = (vertex as u8).wrapping_mul(5) as u32 & 0x1f;
    let feature = (welding_info >> shift) & 0x1f;
    let row = WELDING_ROWS + 24 * feature;
    let row_value = |e: &Engine, index: u32| -> f32 { e.mem.f32(row + 4 * index) };

    if welding_type == 5 {
        // `row[0] <= s`: 1, else `row[2] <= s`: 0, `row[3] <= s`: 2,
        // `row[4] <= s`: 4, else 3.
        let column = if row_value(e, 0) <= along {
            1
        } else if row_value(e, 2) <= along {
            0
        } else if row_value(e, 3) <= along {
            2
        } else if row_value(e, 4) <= along {
            4
        } else {
            3
        };
        match column {
            2 => return 0,
            1 | 3 => return 2,
            _ => {}
        }
        if e.mem.u8(num_feature_points.addr()) == 1 {
            return 0;
        }
        let blended = weld_blend(
            triangle_normal,
            edge_unit,
            row_value(e, column),
            row_value(e, column + 1),
        );
        let unit = normalised(e, blended);
        // The part of the shape-space normal in the plane of the edge and
        // the blended direction.
        let dot_edge = dot3(edge_unit, local_normal);
        let dot_unit = dot3(unit, local_normal);
        let projected: Vec4 =
            std::array::from_fn(|lane| dot_edge * edge_unit[lane] + dot_unit * unit[lane]);
        let nvec = normalised(e, projected);
        let rotated = e.with_stack(0x20, |e, scratch| {
            let local = scratch.addr();
            store_vec(e, local + 0x10, nvec);
            e.call(SET_ROTATED_DIR, &args![local, this_transform, local + 0x10]);
            load_vec(e, local)
        });
        store_vec(e, separating, rotated);
        e.mem.set_f32(separating + 0xc, x87_quiet(scaled));
        return 1;
    }

    // Types 0 to 4: the row's column `m_weldingType` has to be above
    // (type 0) or below `t . wn`, else the contact is left alone.
    if welding_type == 0 {
        if !(row_value(e, 0) > along) {
            return 2;
        }
    } else if !(row_value(e, welding_type) < along) {
        return 2;
    }
    let blended = weld_blend(
        triangle_normal,
        edge_unit,
        row_value(e, welding_type),
        row_value(e, welding_type + 1),
    );
    let nvec = normalised(e, blended);
    let outcome = e.with_stack(0x30, |e, scratch| {
        let local = scratch.addr();
        store_vec(e, local, nvec);
        e.call(SET_ROTATED_DIR, &args![local, this_transform, local]);
        let rotated = load_vec(e, local);
        if e.get(this, HkpTriangleShape::m_isExtruded) != 0 {
            // The contact point in shape space against vertex A, along the
            // triangle normal scaled by the table entry of the type.
            let scale = e.mem.f32(WELD_FACTOR_TABLE + 4 * welding_type);
            let scaled_normal = triangle_normal.map(|lane| scale * lane);
            e.call(
                SET_TRANSFORMED_INVERSE_POS,
                &args![local + 0x20, this_transform, contact_point],
            );
            let point = load_vec(e, local + 0x20);
            let vertex_a = load_vec(e, me + VERTEX_A);
            let measured = dot3(zip_with(point, vertex_a, |p, q| p - q), scaled_normal);
            let minimum = e
                .call(
                    FLOAT_MINIMUM,
                    &args![x87_quiet(measured), x87_quiet(scaled)],
                )
                .f32();
            Some((rotated, x87_quiet(minimum)))
        } else {
            e.vcall(colliding_shape.addr(), 0x44, &args![local + 0x20]);
            e.call(
                SET_TRANSFORMED_POS,
                &args![local + 0x20, colliding_transform, local + 0x20],
            );
            let centre = load_vec(e, local + 0x20);
            let point = load_vec(e, contact_point.addr());
            let side = dot3(zip_with(point, centre, |p, q| p - q), rotated);
            // `MOVMSKPS` / `TEST CL,1`: the sign bit of the sum.
            side.is_sign_negative().then_some((rotated, scaled))
        }
    });
    let Some((normal_out, distance_out)) = outcome else {
        return 0;
    };
    store_vec(e, separating, normal_out);
    e.mem.set_f32(separating + 0xc, x87_quiet(distance_out));
    1
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d1fee0,
            hkp_triangle_shape_calc_content_statistics(Ptr<HkpTriangleShape>, u32, u32)
        ),
        entry!(
            0x00d1ff10,
            hkp_triangle_shape_get_num_collision_spheres_impl(Ptr<HkpTriangleShape>) -> u32
        ),
        entry!(
            0x00d1ff20,
            hkp_triangle_shape_get_first_vertex(Ptr<HkpTriangleShape>, Ptr)
        ),
        entry!(
            0x00d1ff40,
            hkp_triangle_shape_get_centre_impl(Ptr<HkpTriangleShape>, Ptr)
        ),
        entry!(0x00d1ff80, fn_00d1ff80(Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00d20110,
            hkp_triangle_shape_get_collision_spheres_impl(Ptr<HkpTriangleShape>, Ptr) -> Ptr
        ),
        entry!(0x00d201a0, fn_00d201a0(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00d20270,
            hkp_triangle_shape_get_supporting_vertex_impl(Ptr<HkpTriangleShape>, Ptr, Ptr)
        ),
        entry!(
            0x00d20370,
            hkp_triangle_shape_convert_vertex_ids_to_vertices_impl(
                Ptr<HkpTriangleShape>,
                Ptr,
                i32,
                Ptr,
            )
        ),
        entry!(
            0x00d20a70,
            hkp_triangle_shape_cast_ray_impl(Ptr<HkpTriangleShape>, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00d20050,
            hkp_triangle_shape_get_aabb_impl(Ptr<HkpTriangleShape>, Ptr, f32, Ptr)
        ),
        entry!(
            0x00d203f0,
            hkp_triangle_shape_weld_contact_point_impl(
                Ptr<HkpTriangleShape>,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
            ) -> u32
        ),
        entry!(
            0x00d20e70,
            hkp_triangle_shape_cast_ray_bundle_impl(
                Ptr<HkpTriangleShape>,
                Ptr,
                Ptr,
                Ptr,
                Ptr,
            ) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpsphereshape::{
        HALF_F32, PROFILE_SLOT, THREE_F32, TLS_GET_VALUE, ZERO_F32,
    };

    const COLLIDING_VTABLE: u32 = 0x0200_0000;
    const FAKE_CENTRE: u32 = 0x7000_0044;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        e.set_global(ONE_THIRD, 1.0f32 / 3.0);
        e.set_global(EDGE_THRESHOLD_FACTOR, -0.0001f64);
        e.set_global(BUNDLE_THRESHOLD_FACTOR, -0.0001f32);
        for lane in 0..4 {
            e.mem.set_f32(ONES + 4 * lane, 1.0);
            e.mem.set_f32(ONES_CAST_RAY + 4 * lane, 1.0);
            e.mem.set_u32(ALL_ONES + 4 * lane, u32::MAX);
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
            e.mem.set_u32(LANE_BITS + 4 * lane, 1 << lane);
        }
        // Vertex ids 0 to 5: the corner (0, 1, 2, then again) and the
        // extrusion factor (0 for the corners, 1 for the moved ones).
        for (index, corner) in [0u32, 1, 2, 0, 1, 2, 0].into_iter().enumerate() {
            e.mem.set_u32(VERTEX_INDEX_TABLE + 4 * index as u32, corner);
        }
        for (index, factor) in [0.0f32, 0.0, 0.0, 1.0, 1.0, 1.0].into_iter().enumerate() {
            e.mem
                .set_f32(EXTRUSION_FACTOR_TABLE + 4 * index as u32, factor);
        }
        e.call_log = Some(vec![]);
        e
    }

    /// The triangle (0, 0, 0), (1, 0, 0), (0, 1, 0) with radius 0.1 and the
    /// extrusion (0, 0, -1).
    fn triangle(e: &mut Engine, extruded: bool) -> Ptr<HkpTriangleShape> {
        let this = e.new_object::<HkpTriangleShape>();
        e.set(this, HkpTriangleShape::m_radius, 0.1);
        e.set(this, HkpTriangleShape::m_isExtruded, extruded as u8);
        let base = this.addr();
        store_vec(e, base + VERTEX_A, [0.0, 0.0, 0.0, 0.0]);
        store_vec(e, base + VERTEX_B, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, base + VERTEX_C, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, base + EXTRUSION, [0.0, 0.0, -1.0, 0.0]);
        this
    }

    fn vector_at(e: &mut Engine, v: Vec4) -> u32 {
        let address = e.mem.alloc(0x10);
        store_vec(e, address, v);
        address
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

    fn profile_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| ret(stream));
        stream
    }

    #[test]
    fn three_spheres_or_six_when_extruded() {
        let mut e = engine();
        let plain = triangle(&mut e, false);
        let prism = triangle(&mut e, true);
        assert_eq!(e.call(0x00d1ff10, &args![plain]).u32(), 3);
        assert_eq!(e.call(0x00d1ff10, &args![prism]).u32(), 6);
    }

    #[test]
    fn the_first_vertex_is_the_first_corner() {
        let mut e = engine();
        let this = triangle(&mut e, false);
        store_vec(&mut e, this.addr() + VERTEX_A, [1.0, 2.0, 3.0, 4.0]);
        let out = e.mem.alloc(0x10);
        e.call(0x00d1ff20, &args![this, out]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn the_centre_is_a_third_of_the_corner_sum() {
        let mut e = engine();
        let this = triangle(&mut e, false);
        let out = e.mem.alloc(0x10);
        e.call(0x00d1ff40, &args![this, out]);
        assert_near(load_vec(&e, out), [1.0 / 3.0, 1.0 / 3.0, 0.0, 0.0], 1e-7);
    }

    #[test]
    fn the_unit_normal_of_three_points() {
        let mut e = engine();
        let a = vector_at(&mut e, [0.0, 0.0, 0.0, 0.0]);
        let b = vector_at(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let c = vector_at(&mut e, [1.0, 1.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x10);
        e.call(0x00d1ff80, &args![a, b, c, out]);
        assert_near(load_vec(&e, out), [0.0, 0.0, 1.0, 0.0], 1e-5);
        // A degenerate triangle gives zero.
        e.call(0x00d1ff80, &args![a, a, a, out]);
        assert_eq!(load_vec(&e, out), [0.0; 4]);
    }

    #[test]
    fn collision_spheres_answer_the_buffer() {
        let mut e = engine();
        let this = triangle(&mut e, false);
        let out = e.mem.alloc(0x60);
        // EAX is `out` at the RET (00d20193); the convex transform shape reads it (00d5df2f).
        assert_eq!(e.call(0x00d2_0110, &args![this, out]).u32(), out);
    }

    #[test]
    fn collision_spheres_carry_the_radius() {
        let mut e = engine();
        let plain = triangle(&mut e, false);
        let out = e.mem.alloc(0x60);
        e.call(0x00d20110, &args![plain, out]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 0.0, 0.1]);
        assert_eq!(load_vec(&e, out + 0x10), [1.0, 0.0, 0.0, 0.1]);
        assert_eq!(load_vec(&e, out + 0x20), [0.0, 1.0, 0.0, 0.1]);
        assert_eq!(load_vec(&e, out + 0x30), [0.0; 4]);
        let prism = triangle(&mut e, true);
        e.call(0x00d20110, &args![prism, out]);
        assert_eq!(load_vec(&e, out + 0x30), [0.0, 0.0, -1.0, 0.1]);
        assert_eq!(load_vec(&e, out + 0x40), [1.0, 0.0, -1.0, 0.1]);
        assert_eq!(load_vec(&e, out + 0x50), [0.0, 1.0, -1.0, 0.1]);
    }

    #[test]
    fn the_edge_function_of_four_points() {
        let mut e = engine();
        let a = vector_at(&mut e, [0.0, 0.0, 0.0, 0.0]);
        let b = vector_at(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let n = vector_at(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let points = e.mem.alloc(0x30);
        store_vec(&mut e, points, [0.0, 1.0, 2.0, 3.0]);
        store_vec(&mut e, points + 0x10, [0.5, 1.0, -2.0, 0.25]);
        store_vec(&mut e, points + 0x20, [7.0, 7.0, 7.0, 7.0]);
        let out = e.mem.alloc(0x10);
        e.call(0x00d201a0, &args![a, points, b, n, out]);
        // With `a` the origin, `b` on the x axis and `n` the z axis the
        // value of each point is its y.
        assert_eq!(load_vec(&e, out), [0.5, 1.0, -2.0, 0.25]);
    }

    #[test]
    fn the_supporting_vertex_is_the_furthest_corner() {
        let mut e = engine();
        let this = triangle(&mut e, true);
        let out = e.mem.alloc(0x10);
        let support = |e: &mut Engine, direction: Vec4| {
            let d = vector_at(e, direction);
            e.call(0x00d20270, &args![this, d, out]);
            (load_vec(e, out), e.mem.u32(out + 0xc))
        };
        let (vertex, tag) = support(&mut e, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(vertex[..3], [1.0, 0.0, 0.0]);
        assert_eq!(tag, 0x3f00_0001);
        let (vertex, tag) = support(&mut e, [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(vertex[..3], [0.0, 1.0, 0.0]);
        assert_eq!(tag, 0x3f00_0002);
        let (vertex, tag) = support(&mut e, [-1.0, -1.0, 0.0, 0.0]);
        assert_eq!(vertex[..3], [0.0, 0.0, 0.0]);
        assert_eq!(tag, 0x3f00_0000);
        // Along the extrusion the moved corner wins and 3 is added.
        let (vertex, tag) = support(&mut e, [0.0, 0.0, -1.0, 0.0]);
        assert_eq!(vertex[..3], [1.0, 0.0, -1.0]);
        assert_eq!(tag, 0x3f00_0004);
    }

    #[test]
    fn vertex_ids_convert_to_corners_and_moved_corners() {
        let mut e = engine();
        let this = triangle(&mut e, true);
        let ids = e.mem.alloc(8);
        for (index, id) in [0u16, 4, 2].into_iter().enumerate() {
            e.mem.set_u16(ids + 2 * index as u32, id);
        }
        let out = e.mem.alloc(0x40);
        e.call(0x00d20370, &args![this, ids, 3i32, out]);
        assert_eq!(load_vec(&e, out)[..3], [0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0000);
        assert_eq!(load_vec(&e, out + 0x10)[..3], [1.0, 0.0, -1.0]);
        assert_eq!(e.mem.u32(out + 0x1c), 0x3f00_0004);
        assert_eq!(load_vec(&e, out + 0x20)[..3], [0.0, 1.0, 0.0]);
        assert_eq!(e.mem.u32(out + 0x2c), 0x3f00_0002);
        // Nothing is written for a count of zero.
        e.call(0x00d20370, &args![this, ids, 0i32, out + 0x30]);
        assert_eq!(load_vec(&e, out + 0x30), [0.0; 4]);
    }

    #[test]
    fn the_box_covers_the_transformed_prism_and_the_margin() {
        let mut e = engine();
        let this = triangle(&mut e, true);
        e.register_double(SET_TRANSFORMED_POS, |e, a| {
            let moved = zip_with(load_vec(e, a[2]), [10.0, 0.0, 0.0, 0.0], |p, q| p + q);
            store_vec(e, a[0], moved);
            Ret::default()
        });
        let transform = e.mem.alloc(0x40);
        let out = e.mem.alloc(0x20);
        e.call(0x00d20050, &args![this, transform, 0.15f32, out]);
        assert_near(load_vec(&e, out), [9.75, -0.25, -1.25, -0.25], 1e-6);
        assert_near(load_vec(&e, out + 0x10), [11.25, 1.25, 0.25, 0.25], 1e-6);
        let calls = calls_to(&e, SET_TRANSFORMED_POS);
        assert_eq!(calls.len(), 3);
        assert_eq!(calls[0][1..], [transform, this.addr() + VERTEX_A]);
        assert_eq!(calls[2][1..], [transform, this.addr() + VERTEX_C]);
    }

    struct Cast {
        hit: bool,
        output: u32,
        stream: u32,
    }

    fn cast_ray(e: &mut Engine, from: Vec4, to: Vec4, hit_fraction: f32) -> Cast {
        let this = triangle(e, false);
        let stream = profile_stream(e);
        let input = e.mem.alloc(0x20);
        store_vec(e, input, from);
        store_vec(e, input + 0x10, to);
        let output = e.mem.alloc(0x50);
        e.mem.set_f32(output + 0x10, hit_fraction);
        e.mem.set_u32(output + 0x40, 1);
        let result = e.mem.alloc(8);
        let returned = e
            .call(0x00d20a70, &args![this, result, input, output])
            .u32();
        assert_eq!(returned, result);
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_CAST_RAY);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
        Cast {
            hit: e.mem.u8(result) != 0,
            output,
            stream,
        }
    }

    #[test]
    fn a_ray_through_the_triangle_hits() {
        let mut e = engine();
        let c = cast_ray(&mut e, [0.25, 0.25, 1.0, 0.0], [0.25, 0.25, -1.0, 0.0], 1.0);
        assert!(c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 0.5);
        assert_near(load_vec(&e, c.output), [0.0, 0.0, 1.0, 0.0], 1e-5);
        assert_eq!(e.mem.u32(c.output + 0x24), 0xffff_ffff);
        assert_eq!(e.mem.u32(c.stream + 4), c.stream + 0x38);
        // From below the normal faces the start of the ray.
        let c = cast_ray(&mut e, [0.25, 0.25, -1.0, 0.0], [0.25, 0.25, 1.0, 0.0], 1.0);
        assert!(c.hit);
        assert_near(load_vec(&e, c.output), [0.0, 0.0, -1.0, 0.0], 1e-5);
    }

    #[test]
    fn a_ray_misses_outside_beside_or_behind_an_older_hit() {
        let mut e = engine();
        // Passes the plane outside the edge.
        let c = cast_ray(&mut e, [1.0, 1.0, 1.0, 0.0], [1.0, 1.0, -1.0, 0.0], 1.0);
        assert!(!c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 1.0);
        // Parallel to the plane.
        let c = cast_ray(&mut e, [0.25, 0.25, 1.0, 0.0], [0.5, 0.5, 1.0, 0.0], 1.0);
        assert!(!c.hit);
        // A nearer hit is already known.
        let c = cast_ray(&mut e, [0.25, 0.25, 1.0, 0.0], [0.25, 0.25, -1.0, 0.0], 0.3);
        assert!(!c.hit);
        assert_eq!(e.mem.u32(c.output + 0x24), 0);
    }

    #[test]
    fn the_bundle_casts_four_rays() {
        let mut e = engine();
        let this = triangle(&mut e, false);
        let stream = profile_stream(&mut e);
        // Structure of arrays: from.x, from.y, from.z, to.x, to.y, to.z.
        let input = e.mem.alloc(0x60);
        store_vec(&mut e, input, [0.25, 0.25, 1.0, 0.25]);
        store_vec(&mut e, input + 0x10, [0.25, 0.25, 1.0, 0.25]);
        store_vec(&mut e, input + 0x20, [1.0, -1.0, 1.0, 1.0]);
        store_vec(&mut e, input + 0x30, [0.25, 0.25, 1.0, 0.25]);
        store_vec(&mut e, input + 0x40, [0.25, 0.25, 1.0, 0.25]);
        store_vec(&mut e, input + 0x50, [-1.0, 1.0, -1.0, -1.0]);
        let outputs = e.mem.alloc(0x140);
        for ray in 0..4u32 {
            e.mem.set_f32(outputs + 0x50 * ray + 0x10, 1.0);
        }
        let in_mask = e.mem.alloc(0x10);
        for word in 0..3 {
            e.mem.set_u32(in_mask + 4 * word, u32::MAX);
        }
        let result = e.mem.alloc(0x10);
        let returned = e
            .call(0x00d20e70, &args![this, result, input, outputs, in_mask])
            .u32();
        assert_eq!(returned, result);
        let mask: Vec<u32> = (0..4).map(|word| e.mem.u32(result + 4 * word)).collect();
        // Ray 2 passes the plane outside the edge; ray 3 is masked off.
        assert_eq!(mask, [u32::MAX, u32::MAX, 0, 0]);
        assert_near(
            [
                e.mem.f32(outputs + 0x10),
                e.mem.f32(outputs + 0x60),
                0.0,
                0.0,
            ],
            [0.5, 0.5, 0.0, 0.0],
            1e-5,
        );
        assert_near(load_vec(&e, outputs), [0.0, 0.0, 1.0, 0.0], 1e-5);
        assert_near(load_vec(&e, outputs + 0x50), [0.0, 0.0, -1.0, 0.0], 1e-5);
        assert_eq!(e.mem.u32(outputs + 0x20), 0xffff_ffff);
        assert_eq!(e.mem.f32(outputs + 0xf0 + 0x10), 1.0);
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_CAST_RAY_BUNDLE);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
    }

    /// What `weldContactPointImpl` leaves behind.
    struct Weld {
        returned: u32,
        separating: Vec4,
        count: u8,
        rotations: Vec<Vec<u32>>,
        this_transform: u32,
    }

    /// Doubles for the vector helpers (each copies its vector argument) and a
    /// colliding shape whose centre is the origin.
    fn weld_doubles(e: &mut Engine) -> u32 {
        for helper in [
            SET_ROTATED_DIR,
            SET_TRANSFORMED_INVERSE_POS,
            SET_TRANSFORMED_POS,
        ] {
            e.register_double(helper, |e, a| {
                let v = load_vec(e, a[2]);
                store_vec(e, a[0], v);
                Ret::default()
            });
        }
        e.register_double(FLOAT_MINIMUM, |_, a| {
            let (x, y) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            Ret {
                st0: if y > x { x } else { y } as f64,
                ..Ret::default()
            }
        });
        e.put_vtable(COLLIDING_VTABLE, &[FAKE_CENTRE; 24]);
        e.register_double(FAKE_CENTRE, |e, a| {
            store_vec(e, a[1], [0.0; 4]);
            Ret::default()
        });
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape, COLLIDING_VTABLE);
        shape
    }

    fn set_row(e: &mut Engine, row: [f32; 6]) {
        for (index, value) in row.into_iter().enumerate() {
            e.mem.set_f32(WELDING_ROWS + 4 * index as u32, value);
        }
    }

    /// Welds with the identity transform and the separating normal (0, 0, 1)
    /// with distance 0.5.
    fn weld(e: &mut Engine, this: Ptr<HkpTriangleShape>, ids: &[u16], contact: Vec4) -> Weld {
        let shape = weld_doubles(e);
        e.call_log = Some(vec![]);
        let points = e.mem.alloc(8);
        for (index, id) in ids.iter().enumerate() {
            e.mem.set_u16(points + 2 * index as u32, *id);
        }
        let count = e.mem.alloc(4);
        e.mem.set_u8(count, ids.len() as u8);
        let contact = vector_at(e, contact);
        let transform = e.mem.alloc(0x40);
        for (index, column) in [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ]
        .into_iter()
        .enumerate()
        {
            store_vec(e, transform + 0x10 * index as u32, column);
        }
        let colliding_transform = e.mem.alloc(0x40);
        let separating = vector_at(e, [0.0, 0.0, 1.0, 0.5]);
        let returned = e
            .call(
                0x00d203f0,
                &args![
                    this,
                    points,
                    count,
                    contact,
                    transform,
                    shape,
                    colliding_transform,
                    separating
                ],
            )
            .u32();
        Weld {
            returned,
            separating: load_vec(e, separating),
            count: e.mem.u8(count),
            rotations: calls_to(e, SET_ROTATED_DIR),
            this_transform: transform,
        }
    }

    #[test]
    fn welding_type_6_leaves_the_contact_alone() {
        let mut e = engine();
        let this = triangle(&mut e, false);
        e.set(this, HkpTriangleShape::m_weldingType, 6);
        let w = weld(&mut e, this, &[0], [0.0, 0.0, -1.0, 0.0]);
        assert_eq!(w.returned, 2);
        assert!(w.rotations.is_empty());
        assert_eq!(w.separating, [0.0, 0.0, 1.0, 0.5]);
    }

    #[test]
    fn three_feature_points_summing_to_three_leave_the_contact_alone() {
        let mut e = engine();
        let this = triangle(&mut e, false);
        e.set(this, HkpTriangleShape::m_weldingType, 1);
        let w = weld(&mut e, this, &[0, 1, 2], [0.0, 0.0, -1.0, 0.0]);
        assert_eq!(w.returned, 2);
        assert_eq!(w.count, 3);
    }

    #[test]
    fn a_plain_triangle_accepts_a_contact_behind_the_rotated_direction() {
        let mut e = engine();
        let this = triangle(&mut e, false);
        e.set(this, HkpTriangleShape::m_weldingType, 1);
        set_row(&mut e, [0.0, 0.5, 0.5, 0.0, 0.0, 0.0]);
        let w = weld(&mut e, this, &[0], [0.0, 0.0, -1.0, 0.0]);
        assert_eq!(w.returned, 1);
        let half_root = 0.5f32.sqrt();
        assert_near(w.separating, [0.0, -half_root, half_root, 0.5], 1e-4);
        // The direction was rotated in place.
        assert_eq!(w.rotations.len(), 1);
        assert_eq!(w.rotations[0][1], w.this_transform);
        assert_eq!(w.rotations[0][0], w.rotations[0][2]);
        // On the other side of the centre the contact is rejected.
        let w = weld(&mut e, this, &[0], [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(w.returned, 0);
        assert_eq!(w.separating, [0.0, 0.0, 1.0, 0.5]);
    }

    #[test]
    fn an_extruded_triangle_takes_the_smaller_distance() {
        let mut e = engine();
        let this = triangle(&mut e, true);
        e.set(this, HkpTriangleShape::m_weldingType, 1);
        e.mem.set_f32(WELD_FACTOR_TABLE + 4, 2.0);
        set_row(&mut e, [0.0, 0.5, 0.5, 0.0, 0.0, 0.0]);
        let w = weld(&mut e, this, &[0], [0.0, 0.0, -1.0, 0.0]);
        assert_eq!(w.returned, 1);
        let half_root = 0.5f32.sqrt();
        assert_near(w.separating, [0.0, -half_root, half_root, -2.0], 1e-4);
        // The arguments come through the host's RCPPS/RSQRTSS estimate, which
        // differs by CPU vendor: one ULP either way is the same answer (#207).
        let minimum_calls = calls_to(&e, FLOAT_MINIMUM);
        assert_eq!(minimum_calls.len(), 1);
        for (got, want) in minimum_calls[0].iter().zip([-2.0f32, 0.5]) {
            assert!(
                got.abs_diff(want.to_bits()) <= 1,
                "{:#x} is not within one ULP of {want}",
                got
            );
        }
        assert_eq!(calls_to(&e, SET_TRANSFORMED_INVERSE_POS).len(), 1);
    }

    #[test]
    fn the_table_column_has_to_be_crossed_by_the_normal() {
        let mut e = engine();
        let this = triangle(&mut e, false);
        // Type 0 needs column 0 above `t . n` (1); type 1 needs column 1
        // below it.
        set_row(&mut e, [0.5, 1.5, 0.5, 0.0, 0.0, 0.0]);
        assert_eq!(weld(&mut e, this, &[0], [0.0, 0.0, -1.0, 0.0]).returned, 2);
        e.set(this, HkpTriangleShape::m_weldingType, 1);
        assert_eq!(weld(&mut e, this, &[0], [0.0, 0.0, -1.0, 0.0]).returned, 2);
        e.set(this, HkpTriangleShape::m_weldingType, 0);
        set_row(&mut e, [2.0, 0.5, 0.5, 0.0, 0.0, 0.0]);
        assert_eq!(weld(&mut e, this, &[0], [0.0, 0.0, -1.0, 0.0]).returned, 1);
    }

    #[test]
    fn welding_type_5_picks_a_column_from_the_normal() {
        let mut e = engine();
        let this = triangle(&mut e, false);
        e.set(this, HkpTriangleShape::m_weldingType, 5);
        let contact = [0.0, 0.0, -1.0, 0.0];
        set_row(&mut e, [0.5, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(weld(&mut e, this, &[0], contact).returned, 2);
        set_row(&mut e, [2.0, 0.0, 2.0, 2.0, 2.0, 0.0]);
        assert_eq!(weld(&mut e, this, &[0], contact).returned, 2);
        set_row(&mut e, [2.0, 0.0, 2.0, 0.5, 0.0, 0.0]);
        assert_eq!(weld(&mut e, this, &[0], contact).returned, 0);
        // Column 0: a single feature point is rejected, two are welded.
        set_row(&mut e, [2.0, 0.5, 0.5, 2.0, 2.0, 0.0]);
        assert_eq!(weld(&mut e, this, &[0], contact).returned, 0);
        let w = weld(&mut e, this, &[0, 1], contact);
        assert_eq!(w.returned, 1);
        assert_eq!(w.count, 2);
        let root = 4.25f32.sqrt();
        assert_near(w.separating, [0.0, -0.5 / root, 2.0 / root, 0.5], 1e-4);
        assert_eq!(w.rotations.len(), 1);
        assert_ne!(w.rotations[0][0], w.rotations[0][2]);
    }

    #[test]
    fn calc_content_statistics_passes_the_triangle_class_record() {
        let mut e = engine();
        let this = triangle(&mut e, false);
        e.register(REFERENCED_OBJECT_CALC_CONTENT_STATISTICS, |_, _| {
            Ret::default()
        });
        e.call(0x00d1_fee0, &args![this, 0x1111u32, 0x2222u32]);
        assert_eq!(
            calls_to(&e, REFERENCED_OBJECT_CALC_CONTENT_STATISTICS),
            vec![vec![this.addr(), 0x1111, 0x0126_8940]]
        );
    }
}

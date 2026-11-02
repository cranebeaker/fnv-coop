//! `Havok SDK/hkpgskmanifoldutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `hkGskManifold` (Xbox PDB) is a small byte-packed record: two vertex counts at `+0` and
//! `+1`, the number of points at `+2` and 8-byte points from `+4` (two count bytes, a 16-bit id
//! at `+2`, four vertex-index bytes at `+4`), followed by the 16-bit ids of the vertices.
//!
//! The distance functions work on a simplex: two bytes (the number of vertices of each of the
//! two shapes) and from `+4` one byte offset per vertex into a vertex block. The vertex block
//! holds 16-byte vectors; the direction vector at `+0x100` and the squared tolerance (a float)
//! at `+0x11c` belong to it. All of them are SSE code of the exe: the lanes are computed in
//! f32 in the order of the instructions. `RSQRTSS` is the host CPU's estimate
//! (`rsqrt_estimate`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

pub(crate) type Vec4 = [f32; 4];

/// The floats the code reads: 0.0, 3.0, 0.5 and the vector of four 1.0 (`010ec7f0`), and the
/// vector of four sign bits.
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
const ONES: u32 = 0x010e_c7f0;
const SIGN_MASK: u32 = 0x010c_44d0;
/// Where the vertex block keeps its direction vector and its squared tolerance.
const DIRECTION_OFFSET: u32 = 0x100;
const TOLERANCE_OFFSET: u32 = 0x11c;

pub(crate) fn load_vec(e: &Engine, address: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(address + 4 * lane as u32))
}

pub(crate) fn store_vec(e: &mut Engine, address: u32, vector: Vec4) {
    for (lane, value) in vector.into_iter().enumerate() {
        e.mem.set_f32(address + 4 * lane as u32, value);
    }
}

fn zip_with(a: Vec4, b: Vec4, f: impl Fn(f32, f32) -> f32) -> Vec4 {
    std::array::from_fn(|lane| f(a[lane], b[lane]))
}

pub(crate) fn sub(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x - y)
}

pub(crate) fn add(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x + y)
}

pub(crate) fn mul(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x * y)
}

/// `SHUFPS dst, src, imm`.
fn shufps(dst: Vec4, src: Vec4, imm: u8) -> Vec4 {
    let imm = usize::from(imm);
    [
        dst[imm & 3],
        dst[(imm >> 2) & 3],
        src[(imm >> 4) & 3],
        src[(imm >> 6) & 3],
    ]
}

/// The cross product the exe builds from two shuffles of each operand (`0xC9` and `0xD2`),
/// all four lanes: `x.c9 * y.d2 - x.d2 * y.c9`.
pub(crate) fn cross(x: Vec4, y: Vec4) -> Vec4 {
    let (x_c9, x_d2) = (shufps(x, x, 0xc9), shufps(x, x, 0xd2));
    let (y_c9, y_d2) = (shufps(y, y, 0xc9), shufps(y, y, 0xd2));
    sub(mul(x_c9, y_d2), mul(x_d2, y_c9))
}

/// `z + (y + x)`: the `SHUFPS`/`ADDSS` sum of the first three lanes.
pub(crate) fn sum3(lanes: Vec4) -> f32 {
    lanes[2] + (lanes[1] + lanes[0])
}

/// The four sums `z + (y + x)` the exe gathers with `SHUFPS`/`ADDPS`, one per vector.
fn dot_lanes(vectors: [Vec4; 4]) -> Vec4 {
    vectors.map(sum3)
}

/// `CMPLTPS` against zero and `MOVMSKPS`: bit `n` is set when lane `n` is below zero.
fn negative_mask(lanes: Vec4) -> u32 {
    lanes.iter().enumerate().fold(0, |mask, (lane, value)| {
        mask | (u32::from(*value < 0.0) << lane)
    })
}

/// The `RSQRTSS` estimate refined once, with the `CMPEQSS`/`ANDNPS` zero guard:
/// `(half * r) * (three - (sum * r) * r)`, `+0.0` when `sum == zero`.
pub(crate) fn inverse_length(e: &Engine, sum: f32) -> f32 {
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

/// The normal `n` scaled by the inverse of its length.
fn unit_normal(e: &Engine, n: Vec4) -> Vec4 {
    let inverse = inverse_length(e, sum3(mul(n, n)));
    mul([inverse; 4], n)
}

/// `(1 - t) * p + t * q` over the four lanes (the vector of ones is read from the exe).
fn blend(e: &Engine, t: f32, p: Vec4, q: Vec4) -> Vec4 {
    let ones = load_vec(e, ONES);
    add(mul(sub(ones, [t; 4]), p), mul([t; 4], q))
}

/// The vertex of the simplex whose offset byte is at `simplex + 4 + index`.
fn vertex(e: &Engine, simplex: u32, base: u32, index: u32) -> Vec4 {
    load_vec(e, base + u32::from(e.mem.u8(simplex + 4 + index)))
}

fn direction(e: &Engine, base: u32) -> Vec4 {
    load_vec(e, base + DIRECTION_OFFSET)
}

/// Stores the normal at `out + 0x20` and, when bit 3 of `mask` (the direction vector is on the
/// negative side) is set, replaces it with its negation. Answers the three lowest bits of the
/// complement of `mask`.
fn finish_normal(e: &mut Engine, out: u32, normal: Vec4, mask: u32) -> u32 {
    store_vec(e, out + 0x20, normal);
    if mask & 8 != 0 {
        let flipped = std::array::from_fn(|lane| {
            f32::from_bits(normal[lane].to_bits() ^ e.mem.u32(SIGN_MASK + 4 * lane as u32))
        });
        store_vec(e, out + 0x20, flipped);
    }
    !mask & 7
}

/// The segment test of the vertex/edge and edge/vertex cases. The edge direction is `u`; `v`
/// and `w` are the two vectors it is projected on. Fails (`None`) when the squared length of
/// `u` times the tolerance is below the second lane of the squared cross product of `u` and
/// `v` added to twice the (zero) fourth lane (the exe's lane arithmetic), or when a
/// projection is negative; otherwise answers the parameter along the edge.
fn segment_parameter(e: &Engine, base: u32, u: Vec4, v: Vec4, w: Vec4) -> Option<f32> {
    let n = cross(u, v);
    let squares = mul(n, n);
    let first = sum3(mul(v, u));
    let second = sum3(mul(w, u));
    let fourth = squares[3] + (squares[3] + squares[1]);
    let tolerance = e.mem.f32(base + TOLERANCE_OFFSET);
    let length2 = sum3(mul(u, u));
    if f64::from(tolerance) * f64::from(length2) < f64::from(fourth) {
        return None;
    }
    if first < 0.0 || second < 0.0 {
        return None;
    }
    Some(first / (first + second))
}

/// Two single vertices (simplex key `(1, 1)`): the squared distance must not be above the
/// tolerance.
fn vertex_vertex(e: &mut Engine, simplex: u32, base: u32, out: u32) -> u32 {
    let a = vertex(e, simplex, base, 0);
    let b = vertex(e, simplex, base, 1);
    let delta = sub(a, b);
    let distance2 = sum3(mul(delta, delta));
    let tolerance = e.mem.f32(base + TOLERANCE_OFFSET);
    if f64::from(tolerance) < f64::from(distance2) {
        return 1;
    }
    store_vec(e, out + 0x10, delta);
    let direction = direction(e, base);
    store_vec(e, out + 0x20, direction);
    store_vec(e, out, a);
    0
}

/// A vertex and an edge (simplex key `(1, 2)`): vertex `a`, edge `b` to `c`.
fn vertex_edge(e: &mut Engine, simplex: u32, base: u32, out: u32) -> u32 {
    let a = vertex(e, simplex, base, 0);
    let b = vertex(e, simplex, base, 1);
    let c = vertex(e, simplex, base, 2);
    store_vec(e, out, a);
    let direction = direction(e, base);
    store_vec(e, out + 0x20, direction);
    let Some(t) = segment_parameter(e, base, sub(c, b), sub(c, a), sub(a, b)) else {
        return 1;
    };
    let point = blend(e, t, b, c);
    store_vec(e, out + 0x10, sub(a, point));
    0
}

/// An edge and a vertex (simplex key `(2, 1)`): edge `a` to `b`, vertex `c`.
fn edge_vertex(e: &mut Engine, simplex: u32, base: u32, out: u32) -> u32 {
    let direction = direction(e, base);
    store_vec(e, out + 0x20, direction);
    let a = vertex(e, simplex, base, 0);
    let b = vertex(e, simplex, base, 1);
    let c = vertex(e, simplex, base, 2);
    let Some(t) = segment_parameter(e, base, sub(b, a), sub(b, c), sub(c, a)) else {
        return 1;
    };
    let point = blend(e, t, b, a);
    store_vec(e, out, point);
    store_vec(e, out + 0x10, sub(point, c));
    0
}

/// A vertex and a triangle (simplex key `(1, 3)`): vertex `a`, triangle `b`, `c`, `d`. The
/// normal of the triangle is stored (flipped to the side of the direction vector) and the
/// answer is the three lowest bits of the "not below zero" flags of the edge tests.
fn vertex_triangle(e: &mut Engine, simplex: u32, base: u32, out: u32) -> u32 {
    let a = vertex(e, simplex, base, 0);
    let b = vertex(e, simplex, base, 1);
    let c = vertex(e, simplex, base, 2);
    let d = vertex(e, simplex, base, 3);
    store_vec(e, out, a);
    store_vec(e, out + 0x10, sub(a, b));
    let (z, w, m) = (sub(d, c), sub(b, d), sub(c, b));
    let normal = unit_normal(e, cross(z, w));
    let (r, s, q) = (sub(a, c), sub(a, d), sub(a, b));
    let lane_a = mul(cross(r, z), normal);
    let lane_b = mul(cross(q, m), normal);
    let lane_c = mul(cross(s, w), normal);
    let lane_d = mul(direction(e, base), normal);
    let sums = dot_lanes([lane_a, lane_c, lane_b, lane_d]);
    finish_normal(e, out, normal, negative_mask(sums))
}

/// A triangle and a vertex (simplex key `(3, 1)`): triangle `a`, `b`, `c`, vertex `d`. The
/// projection of `d` on the plane of the triangle is stored at `out`, the vector from `d` to
/// `a` at `out + 0x10` and the normal as for [`vertex_triangle`].
fn triangle_vertex(e: &mut Engine, simplex: u32, base: u32, out: u32) -> u32 {
    let a = vertex(e, simplex, base, 0);
    let b = vertex(e, simplex, base, 1);
    let c = vertex(e, simplex, base, 2);
    let d = vertex(e, simplex, base, 3);
    let to_a = sub(a, d);
    store_vec(e, out + 0x10, to_a);
    let normal = unit_normal(e, cross(sub(c, b), sub(a, c)));
    let lane_a = mul(cross(sub(d, b), sub(c, b)), normal);
    let lane_b = mul(cross(sub(d, c), sub(a, c)), normal);
    let lane_c = mul(cross(sub(d, a), sub(b, a)), normal);
    let lane_d = mul(direction(e, base), normal);
    let height = sum3(mul(normal, to_a));
    store_vec(e, out, add(mul([height; 4], normal), d));
    let sums = dot_lanes([lane_a, lane_b, lane_c, lane_d]);
    finish_normal(e, out, normal, negative_mask(sums))
}

/// Two edges (simplex key `(2, 2)`): edge `a` to `b` and edge `c` to `d`. The closest points
/// of the two edges are `out` (on the first) and, as the difference, `out + 0x10`.
fn edge_edge(e: &mut Engine, simplex: u32, base: u32, out: u32) -> u32 {
    let a = vertex(e, simplex, base, 0);
    let b = vertex(e, simplex, base, 1);
    let c = vertex(e, simplex, base, 2);
    let d = vertex(e, simplex, base, 3);
    let first = sub(b, a);
    let second = sub(d, c);
    let normal = cross(first, second);
    let u2 = cross(second, normal);
    let u3 = cross(first, normal);
    let (b_to_d, a_to_c) = (sub(b, d), sub(a, c));
    let lane_p = mul(u2, b_to_d);
    let lane_q = mul(u2, a_to_c);
    let lane_r = mul(u3, a_to_c);
    let lane_s = mul(u3, b_to_d);
    let sums = dot_lanes([lane_q, lane_r, lane_p, lane_s]);
    let direction = direction(e, base);
    store_vec(e, out + 0x20, direction);
    let mask = negative_mask(sums);
    if mask != 3 && mask != 0xc {
        return 1;
    }
    let t1 = sums[0] / (sums[0] - sums[2]);
    let on_first = blend(e, t1, a, b);
    store_vec(e, out, on_first);
    let t2 = sums[1] / (sums[1] - sums[3]);
    let on_second = blend(e, t2, c, d);
    store_vec(e, out + 0x10, sub(on_first, on_second));
    0
}

// Translated from 00dad640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGskManifold_cleanup` (Xbox PDB; cdecl `(manifold, owner, flag)`): for each of the
/// `[manifold + 2]` points (the count is read again every round) whose 16-bit id (at `+6`,
/// 8-byte stride) is not `0xffff`, calls the owner's virtual function at slot `0x14` with
/// `(id, flag)`; then clears the three count bytes at `+0`, `+1` and `+2`.
pub fn hkgsk_manifold_cleanup(e: &mut Engine, manifold: Ptr, owner: Ptr, flag: u32) {
    let m = manifold.addr();
    let mut index = 0u32;
    if e.mem.u8(m + 2) != 0 {
        loop {
            let id = e.mem.u16(m + 6 + 8 * index);
            if id != 0xffff {
                e.vcall(owner.addr(), 0x14, &args![u32::from(id), flag]);
            }
            index += 1;
            if index >= u32::from(e.mem.u8(m + 2)) {
                break;
            }
        }
    }
    e.mem.set_u8(m + 2, 0);
    e.mem.set_u8(m, 0);
    e.mem.set_u8(m + 1, 0);
}

// Translated from 00dad6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00dad6a0` (no map name / unconfirmed), register inputs `(simplex EAX, base EDX, out
/// ESI)`: the distance test for two single vertices, simplex key `(1, 1)`. Answers 1 when the
/// squared distance between vertex 0 and vertex 1 is above the tolerance (nothing is written);
/// otherwise writes the difference (vertex 0 minus vertex 1) at `out + 0x10`, the direction
/// vector at `out + 0x20` and vertex 0 at `out`, and answers 0. (`ECX` is left holding
/// the address of vertex 0; the Rust function does not return it.)
pub fn fn_00dad6a0(e: &mut Engine, simplex: Ptr, base: Ptr, out: Ptr) -> u32 {
    vertex_vertex(e, simplex.addr(), base.addr(), out.addr())
}

// Translated from 00dad720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00dad720` (no map name / unconfirmed), register inputs `(simplex ECX, base EAX, out
/// EDI)`: the distance test for a vertex (vertex 0) and an edge (vertices 1 and 2), simplex key
/// `(1, 2)`. Returns `EAX | EDX << 32` with `EAX` 0 or 1 and `EDX` the address of
/// vertex 1. Vertex 0 and the direction vector are stored at `out` and `out + 0x20` before the
/// tests. Fails (1) when the tolerance times the squared edge length is below the second lane
/// of the squared cross product of the edge and the vector from vertex 0 to vertex 2 (lane
/// arithmetic of the exe), or when a projection of the edge is negative; otherwise the vector
/// from vertex 0 to the closest point of the edge goes to `out + 0x10`.
pub fn fn_00dad720(e: &mut Engine, simplex: Ptr, base: Ptr, out: Ptr) -> u64 {
    let offset_b = u32::from(e.mem.u8(simplex.addr() + 5));
    let result = vertex_edge(e, simplex.addr(), base.addr(), out.addr());
    u64::from(result) | (u64::from(base.addr() + offset_b) << 32)
}

// Translated from 00dad860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00dad860` (no map name / unconfirmed), register inputs `(simplex ECX, base EAX, out
/// EDI)`: the distance test for an edge (vertices 0 and 1) and a vertex (vertex 2), simplex
/// key `(2, 1)`. Returns `EAX | EDX << 32` with `EAX` 0 or 1 and `EDX` the address of vertex
/// 0. The direction vector is stored at `out + 0x20` first; the tests are those of
/// `fn_00dad720`; on success the closest point of the edge goes to `out` and its difference
/// to vertex 2 to `out + 0x10`.
pub fn fn_00dad860(e: &mut Engine, simplex: Ptr, base: Ptr, out: Ptr) -> u64 {
    let offset_a = u32::from(e.mem.u8(simplex.addr() + 4));
    let result = edge_vertex(e, simplex.addr(), base.addr(), out.addr());
    u64::from(result) | (u64::from(base.addr() + offset_a) << 32)
}

// Translated from 00dad990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00dad990` (no map name / unconfirmed), register inputs `(base ECX, out EDX)` then the
/// stack argument `simplex`: the closest-feature test for a simplex. The key is `byte[1] +
/// 4 * byte[0]` of the simplex; keys 5, 6, 9, 7, 10 and 13 (counts `(1,1)`, `(1,2)`, `(2,1)`,
/// `(1,3)`, `(2,2)` and `(3,1)`) run the case of `fn_00dad6a0`, `fn_00dad720`, `fn_00dad860`,
/// a vertex/triangle test, an edge/edge test and a triangle/vertex test; any other key answers
/// 1. The triangle cases store the (unit) normal at `out + 0x20`, negated when the direction
/// vector lies on its negative side, and answer the three lowest bits of the complement of the
/// "below zero" flags of the edge tests.
pub fn fn_00dad990(e: &mut Engine, base: Ptr, out: Ptr, simplex: Ptr) -> u32 {
    let key = u32::from(e.mem.u8(simplex.addr() + 1)) + u32::from(e.mem.u8(simplex.addr())) * 4;
    let (simplex, base, out) = (simplex.addr(), base.addr(), out.addr());
    match key {
        5 => vertex_vertex(e, simplex, base, out),
        6 => vertex_edge(e, simplex, base, out),
        7 => vertex_triangle(e, simplex, base, out),
        9 => edge_vertex(e, simplex, base, out),
        10 => edge_edge(e, simplex, base, out),
        13 => triangle_vertex(e, simplex, base, out),
        _ => 1,
    }
}

// Translated from 00dae220 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00dae220 - reads the stack slot of the prefix table that is only written for the first (count0 + count1) entries, which is uninitialised stack when the second vertex group is empty; the Rust code reads zero there.
/// `hkGskManifold_removePoint` (Xbox PDB; register input `EDX` = the manifold): compacts the
/// vertex ids of the manifold after points were removed. Marks the vertex slots (the high
/// nibble of the point's index bytes at `+8` to `+0xb`; the third and fourth only when the two
/// count bytes of the point add up to at least 3, respectively exactly 4) as used, then
/// walks the `[+0] + [+1]` ids (16-bit, after the points) keeping the used ones and recording
/// for each slot the number of used slots before it, times 16. The two counts become the
/// number of used slots before slot `[+0]` and the number of kept slots minus that, and every
/// index byte of every point is replaced by the low byte of the recorded value of its slot.
/// The exe's 16-slot frame is modelled as zero-filled arrays that grow with the total.
pub fn hkgsk_manifold_remove_point(e: &mut Engine, manifold: Ptr) {
    let m = manifold.addr();
    let count = u32::from(e.mem.u8(m + 2));
    let mut flags = [0u8; 16];
    for point in 0..count {
        let p = m + 8 * point;
        let sum = u32::from(e.mem.u8(p + 5)) + u32::from(e.mem.u8(p + 4));
        flags[usize::from(e.mem.u8(p + 8) >> 4)] = 1;
        flags[usize::from(e.mem.u8(p + 9) >> 4)] = 1;
        if sum >= 3 {
            flags[usize::from(e.mem.u8(p + 10) >> 4)] = 1;
        }
        if sum == 4 {
            flags[usize::from(e.mem.u8(p + 11) >> 4)] = 1;
        }
    }
    let ids = m + count * 8 + 4;
    let total = (i32::from(e.mem.u8(m + 1)) + i32::from(e.mem.u8(m))).max(0) as usize;
    let mut prefix = vec![0u32; total.max(16)];
    let mut running = 0u32;
    let mut kept = 0u32;
    for (slot, entry) in prefix.iter_mut().enumerate().take(total) {
        *entry = running;
        if flags.get(slot).copied().unwrap_or(0) != 0 {
            let id = e.mem.u16(ids + 2 * slot as u32);
            e.mem.set_u16(ids + 2 * kept, id);
            kept += 1;
            running += 0x10;
        }
    }
    let first = prefix.get(usize::from(e.mem.u8(m))).copied().unwrap_or(0) >> 4;
    let first_low = first as u8;
    let second_low = (kept as u8).wrapping_sub(first_low);
    e.mem.set_u8(m, first_low);
    e.mem.set_u8(m + 1, second_low);
    if e.mem.u8(m + 2) != 0 {
        let mut point = 0u32;
        loop {
            for byte in 8..12 {
                let address = m + 8 * point + byte;
                let slot = usize::from(e.mem.u8(address) >> 4);
                let recorded = prefix.get(slot).copied().unwrap_or(0);
                e.mem.set_u8(address, recorded as u8);
            }
            point += 1;
            if point >= u32::from(e.mem.u8(m + 2)) {
                break;
            }
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_d640, hkgsk_manifold_cleanup(Ptr, Ptr, u32)),
        entry!(0x00da_d6a0, fn_00dad6a0(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00da_d720, fn_00dad720(Ptr, Ptr, Ptr) -> u64),
        entry!(0x00da_d860, fn_00dad860(Ptr, Ptr, Ptr) -> u64),
        entry!(0x00da_d990, fn_00dad990(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00da_e220, hkgsk_manifold_remove_point(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine as base_engine, logging_object, new_log,
    };

    const UNTOUCHED: u32 = 0xcdcd_cdcd;

    /// An engine with the exe's constants the code reads.
    fn engine() -> Engine {
        let mut e = base_engine();
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
        store_vec(&mut e, ONES, [1.0; 4]);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e
    }

    /// A vertex block with the `vertices` at `base + 0x10 * (n + 1)`, the direction vector
    /// `(0, 0, 1, 0)`, a simplex whose counts are `key` and an output block filled with a
    /// marker. Answers `(base, simplex, out)`.
    fn scene(e: &mut Engine, key: [u8; 2], vertices: &[Vec4], tolerance: f32) -> (u32, u32, u32) {
        let base = e.mem.alloc(0x140);
        let simplex = e.mem.alloc(0x10);
        e.mem.set_u8(simplex, key[0]);
        e.mem.set_u8(simplex + 1, key[1]);
        for (index, vector) in vertices.iter().enumerate() {
            let offset = 0x10 + 0x10 * index as u32;
            store_vec(e, base + offset, *vector);
            e.mem.set_u8(simplex + 4 + index as u32, offset as u8);
        }
        store_vec(e, base + DIRECTION_OFFSET, [0.0, 0.0, 1.0, 0.0]);
        e.mem.set_f32(base + TOLERANCE_OFFSET, tolerance);
        let out = e.mem.alloc(0x30);
        for word in 0..12 {
            e.mem.set_u32(out + 4 * word, UNTOUCHED);
        }
        (base, simplex, out)
    }

    fn untouched(e: &Engine, out: u32) -> bool {
        (0..12).all(|word| e.mem.u32(out + 4 * word) == UNTOUCHED)
    }

    /// Lane-wise comparison that tolerates the error of the host's `RSQRTSS` estimate.
    fn assert_close(actual: Vec4, expected: Vec4) {
        for lane in 0..4 {
            let tolerance = 1e-6 * expected[lane].abs().max(1.0);
            assert!(
                (actual[lane] - expected[lane]).abs() <= tolerance,
                "lane {lane}: {actual:?} against {expected:?}"
            );
        }
    }

    #[test]
    fn cleanup_notifies_the_owner_of_every_used_point_and_empties_the_manifold() {
        let mut e = engine();
        let log = new_log();
        let owner = logging_object(&mut e, &log, 0x00e0_0000);
        let manifold = e.mem.alloc(0x40);
        e.mem.set_u8(manifold, 1);
        e.mem.set_u8(manifold + 1, 2);
        e.mem.set_u8(manifold + 2, 3);
        for (index, id) in [5u16, 0xffff, 7].into_iter().enumerate() {
            e.mem.set_u16(manifold + 6 + 8 * index as u32, id);
        }
        e.call(0x00da_d640, &args![manifold, owner, 0x77u32]);
        assert_eq!(
            calls(&log, 0x00e0_0014),
            vec![vec![owner, 5, 0x77], vec![owner, 7, 0x77]]
        );
        let counts = [
            e.mem.u8(manifold),
            e.mem.u8(manifold + 1),
            e.mem.u8(manifold + 2),
        ];
        assert_eq!(counts, [0, 0, 0]);
    }

    #[test]
    fn cleanup_of_a_manifold_without_points_only_clears_the_counts() {
        let mut e = engine();
        let log = new_log();
        let owner = logging_object(&mut e, &log, 0x00e0_0000);
        let manifold = e.mem.alloc(0x40);
        e.mem.set_u8(manifold, 4);
        e.mem.set_u8(manifold + 1, 3);
        e.call(0x00da_d640, &args![manifold, owner, 1u32]);
        assert!(calls(&log, 0x00e0_0014).is_empty());
        assert_eq!(e.mem.u8(manifold), 0);
        assert_eq!(e.mem.u8(manifold + 1), 0);
    }

    #[test]
    fn two_vertices_within_the_tolerance_write_the_difference() {
        let mut e = engine();
        let (base, simplex, out) = scene(
            &mut e,
            [1, 1],
            &[[1.0, 2.0, 3.0, 0.0], [0.0, 0.0, 1.0, 0.0]],
            10.0,
        );
        assert_eq!(e.call(0x00da_d6a0, &args![simplex, base, out]).u32(), 0);
        assert_eq!(load_vec(&e, out + 0x10), [1.0, 2.0, 2.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x20), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 3.0, 0.0]);
    }

    #[test]
    fn two_vertices_beyond_the_tolerance_are_refused_and_nothing_is_written() {
        let mut e = engine();
        let (base, simplex, out) = scene(
            &mut e,
            [1, 1],
            &[[1.0, 2.0, 3.0, 0.0], [0.0, 0.0, 1.0, 0.0]],
            4.0,
        );
        assert_eq!(e.call(0x00da_d6a0, &args![simplex, base, out]).u32(), 1);
        assert!(untouched(&e, out));
    }

    #[test]
    fn a_vertex_and_an_edge_write_the_vector_to_the_closest_point() {
        let mut e = engine();
        let vertices = [
            [1.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
        ];
        let (base, simplex, out) = scene(&mut e, [1, 2], &vertices, 0.0);
        let answer = e.call(0x00da_d720, &args![simplex, base, out]).u64();
        assert_eq!(answer as u32, 0);
        assert_eq!((answer >> 32) as u32, base + 0x20);
        assert_eq!(load_vec(&e, out), [1.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x10), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x20), [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn a_vertex_off_the_edge_plane_is_refused_by_the_cross_product_test() {
        let mut e = engine();
        let vertices = [
            [1.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
        ];
        // The second lane of the squared cross product is 4; the tolerance times the squared
        // edge length (4) decides.
        let (base, simplex, out) = scene(&mut e, [1, 2], &vertices, 0.5);
        let answer = e.call(0x00da_d720, &args![simplex, base, out]).u64();
        assert_eq!(answer as u32, 1);
        assert_eq!((answer >> 32) as u32, base + 0x20);
        // The vertex and the direction vector were written before the test.
        assert_eq!(load_vec(&e, out), [1.0, 0.0, 1.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x20), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(e.mem.u32(out + 0x10), UNTOUCHED);
        let (base, simplex, out) = scene(&mut e, [1, 2], &vertices, 2.0);
        assert_eq!(
            e.call(0x00da_d720, &args![simplex, base, out]).u64() as u32,
            0
        );
        assert_eq!(load_vec(&e, out + 0x10), [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn a_vertex_behind_the_edge_end_is_refused() {
        let mut e = engine();
        let vertices = [
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
        ];
        let (base, simplex, out) = scene(&mut e, [1, 2], &vertices, 10.0);
        assert_eq!(
            e.call(0x00da_d720, &args![simplex, base, out]).u64() as u32,
            1
        );
    }

    #[test]
    fn an_edge_and_a_vertex_write_the_closest_point_and_its_difference() {
        let mut e = engine();
        let vertices = [
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [1.0, 1.0, 0.0, 0.0],
        ];
        let (base, simplex, out) = scene(&mut e, [2, 1], &vertices, 0.0);
        let answer = e.call(0x00da_d860, &args![simplex, base, out]).u64();
        assert_eq!(answer as u32, 0);
        assert_eq!((answer >> 32) as u32, base + 0x10);
        assert_eq!(load_vec(&e, out), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x10), [0.0, -1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x20), [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn an_edge_and_a_vertex_beyond_the_edge_end_are_refused_after_the_direction_is_stored() {
        let mut e = engine();
        let vertices = [
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [5.0, 1.0, 0.0, 0.0],
        ];
        let (base, simplex, out) = scene(&mut e, [2, 1], &vertices, 10.0);
        let answer = e.call(0x00da_d860, &args![simplex, base, out]).u64();
        assert_eq!(answer as u32, 1);
        assert_eq!(load_vec(&e, out + 0x20), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(e.mem.u32(out), UNTOUCHED);
        assert_eq!(e.mem.u32(out + 0x10), UNTOUCHED);
    }

    #[test]
    fn the_simplex_dispatch_runs_the_small_cases() {
        let mut e = engine();
        let (base, simplex, out) = scene(
            &mut e,
            [1, 1],
            &[[1.0, 2.0, 3.0, 0.0], [0.0, 0.0, 1.0, 0.0]],
            10.0,
        );
        assert_eq!(e.call(0x00da_d990, &args![base, out, simplex]).u32(), 0);
        assert_eq!(load_vec(&e, out + 0x10), [1.0, 2.0, 2.0, 0.0]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 3.0, 0.0]);

        let vertices = [
            [1.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
        ];
        let (base, simplex, out) = scene(&mut e, [1, 2], &vertices, 0.0);
        assert_eq!(e.call(0x00da_d990, &args![base, out, simplex]).u32(), 0);
        assert_eq!(load_vec(&e, out + 0x10), [0.0, 1.0, 0.0, 0.0]);

        let vertices = [
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [1.0, 1.0, 0.0, 0.0],
        ];
        let (base, simplex, out) = scene(&mut e, [2, 1], &vertices, 0.0);
        assert_eq!(e.call(0x00da_d990, &args![base, out, simplex]).u32(), 0);
        assert_eq!(load_vec(&e, out), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x10), [0.0, -1.0, 0.0, 0.0]);
    }

    #[test]
    fn the_simplex_dispatch_refuses_keys_without_a_case() {
        let mut e = engine();
        // Keys 0 to 4, 8, 11, 12 and everything above 13 (the first byte is the high part).
        for key in [
            [0u8, 0],
            [0, 3],
            [1, 0],
            [2, 0],
            [2, 3],
            [3, 0],
            [3, 2],
            [4, 0],
        ] {
            let (base, simplex, out) = scene(&mut e, key, &[[0.0; 4]; 4], 1.0);
            assert_eq!(e.call(0x00da_d990, &args![base, out, simplex]).u32(), 1);
            assert!(untouched(&e, out));
        }
    }

    #[test]
    fn a_vertex_inside_a_triangle_answers_zero_with_the_unit_normal() {
        let mut e = engine();
        let vertices = [
            [0.5, 0.5, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
        ];
        let (base, simplex, out) = scene(&mut e, [1, 3], &vertices, 0.0);
        assert_eq!(e.call(0x00da_d990, &args![base, out, simplex]).u32(), 0);
        assert_eq!(load_vec(&e, out), [0.5, 0.5, 1.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x10), [0.5, 0.5, 1.0, 0.0]);
        assert_close(load_vec(&e, out + 0x20), [0.0, 0.0, 1.0, 0.0]);
        // A direction on the other side flips the normal and sets the fourth flag, which
        // the answer does not report.
        store_vec(&mut e, base + DIRECTION_OFFSET, [0.0, 0.0, -1.0, 0.0]);
        assert_eq!(e.call(0x00da_d990, &args![base, out, simplex]).u32(), 0);
        assert_close(load_vec(&e, out + 0x20), [0.0, 0.0, -1.0, 0.0]);
    }

    #[test]
    fn a_vertex_outside_a_triangle_reports_the_edge_that_faces_it() {
        let mut e = engine();
        let vertices = [
            [2.0, 2.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
        ];
        let (base, simplex, out) = scene(&mut e, [1, 3], &vertices, 0.0);
        assert_eq!(e.call(0x00da_d990, &args![base, out, simplex]).u32(), 1);
        assert_close(load_vec(&e, out + 0x20), [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn a_vertex_over_a_triangle_is_projected_on_its_plane() {
        let mut e = engine();
        let vertices = [
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.5, 0.5, 1.0, 0.0],
        ];
        let (base, simplex, out) = scene(&mut e, [3, 1], &vertices, 0.0);
        assert_eq!(e.call(0x00da_d990, &args![base, out, simplex]).u32(), 0);
        assert_close(load_vec(&e, out), [0.5, 0.5, 0.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x10), [-0.5, -0.5, -1.0, 0.0]);
        assert_close(load_vec(&e, out + 0x20), [0.0, 0.0, 1.0, 0.0]);

        let vertices = [
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [2.0, 2.0, 1.0, 0.0],
        ];
        let (base, simplex, out) = scene(&mut e, [3, 1], &vertices, 0.0);
        assert_eq!(e.call(0x00da_d990, &args![base, out, simplex]).u32(), 1);
    }

    #[test]
    fn two_crossing_edges_write_their_closest_points() {
        let mut e = engine();
        let vertices = [
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [1.0, -1.0, 1.0, 0.0],
            [1.0, 1.0, 1.0, 0.0],
        ];
        let (base, simplex, out) = scene(&mut e, [2, 2], &vertices, 0.0);
        assert_eq!(e.call(0x00da_d990, &args![base, out, simplex]).u32(), 0);
        assert_eq!(load_vec(&e, out), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x10), [0.0, 0.0, -1.0, 0.0]);
        assert_eq!(load_vec(&e, out + 0x20), [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn two_edges_that_miss_each_other_are_refused_after_the_direction_is_stored() {
        let mut e = engine();
        let vertices = [
            [0.0, 0.0, 0.0, 0.0],
            [2.0, 0.0, 0.0, 0.0],
            [4.0, -1.0, 1.0, 0.0],
            [4.0, 1.0, 1.0, 0.0],
        ];
        let (base, simplex, out) = scene(&mut e, [2, 2], &vertices, 0.0);
        assert_eq!(e.call(0x00da_d990, &args![base, out, simplex]).u32(), 1);
        assert_eq!(load_vec(&e, out + 0x20), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(e.mem.u32(out), UNTOUCHED);
        assert_eq!(e.mem.u32(out + 0x10), UNTOUCHED);
    }

    #[test]
    fn remove_point_compacts_the_vertex_ids_and_renumbers_the_index_bytes() {
        let mut e = engine();
        let manifold = e.mem.alloc(0x60);
        // Counts 2 and 1 (three vertex slots), one point; the point uses slots 2 and 0 only.
        e.mem.set_u8(manifold, 2);
        e.mem.set_u8(manifold + 1, 1);
        e.mem.set_u8(manifold + 2, 1);
        e.mem.set_u8(manifold + 4, 1);
        e.mem.set_u8(manifold + 5, 1);
        e.mem.set_u8(manifold + 8, 0x20);
        // The ids follow the single point.
        for (index, id) in [0xau16, 0xb, 0xc].into_iter().enumerate() {
            e.mem.set_u16(manifold + 12 + 2 * index as u32, id);
        }
        e.call(0x00da_e220, &args![manifold]);
        assert_eq!(e.mem.u8(manifold), 1);
        assert_eq!(e.mem.u8(manifold + 1), 1);
        assert_eq!(e.mem.u8(manifold + 2), 1);
        assert_eq!(e.mem.u16(manifold + 12), 0xa);
        assert_eq!(e.mem.u16(manifold + 14), 0xc);
        let indices = [
            e.mem.u8(manifold + 8),
            e.mem.u8(manifold + 9),
            e.mem.u8(manifold + 10),
            e.mem.u8(manifold + 11),
        ];
        assert_eq!(indices, [0x10, 0, 0, 0]);
    }
}

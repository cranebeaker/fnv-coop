//! `Havok SDK/hkgsk.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::{
    add4, compare_flags, float_to_x87, load_bits, load_vec4, movhlps, movlhps, mul4, rotate_c9,
    rotate_d2, shufps, store_vec4, sub4, sum3, test_parity_even, unpckhps, unpcklps, Vec4, X87,
};
use crate::x87::rsqrt_estimate;

/// The `float` `1e-4` (`010141cc`) and the `float` at `010dbb58` that `handlePenetration` puts
/// into the block it hands on, and the routines it calls: the object routine at `00e63770`
/// (`thiscall` on the block, one stack word) and the one at `00e62740` (cdecl, six words).
const SMALL_EPSILON: u32 = 0x0101_41cc;
const PENETRATION_FLOAT: u32 = 0x010d_bb58;
const UNNAMED_00E63770: u32 = 0x00e6_3770;
const UNNAMED_00E62740: u32 = 0x00e6_2740;
/// The mask that drops the tag bits 24 to 29 of a word (`0xc0ffffff`).
const TAG_MASK: u32 = 0xc0ff_ffff;

/// `0.0` as the exe's `float` (`01011d78`), the all-ones vector (`010ec7f0`), the sign mask
/// (`010c44d0`) and the absolute-value mask (`01017d10`), each in every lane.
const ONES: u32 = 0x010e_c7f0;
const SIGN_MASK: u32 = 0x010c_44d0;
const ABS_MASK: u32 = 0x0101_7d10;
/// The largest `float` the exe uses as "no distance" (`0104da48`), the threshold `1e-6`
/// (`010c6090`, a `float`) and the factor `1.1` (`010290f8`, a `double`) of `findClosestTriangle`.
const LARGEST_FLOAT: u32 = 0x0104_da48;
const NEAR_ZERO: u32 = 0x010c_6090;
const TIE_FACTOR: u32 = 0x0102_90f8;
/// `hkLineSegmentUtil::closestLineSegLineSeg` (Xbox PDB, `00d22e40`, cdecl `(start_a,
/// direction_a, start_b, direction_b, out)`).
const CLOSEST_LINE_SEG_LINE_SEG: u32 = 0x00d2_2e40;
/// The `float` `-1.0` (`01012054`).
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;

/// The sum of the three lane products, assembled from three 4-lane vectors the way
/// `00e4b810`, `00e4bbe0`, `00e4da90` and `00e4dc70` do it: the three vectors are transposed
/// with `UNPCKLPS`/`UNPCKHPS`/`MOVLHPS`/`MOVHLPS`/`SHUFPS` so that lane `i` of the answer is
/// `v_i.lane2 + (v_i.lane1 + v_i.lane0)` (lane 3 is `w3 + (w3 + w1)`).
fn lane_sums(u: Vec4, v: Vec4, w: Vec4) -> Vec4 {
    let low = unpcklps(u, v);
    let high = unpckhps(u, v);
    let first = movlhps(low, w);
    let second = shufps(movhlps(first, low), w, 0xd4);
    let third = shufps(high, w, 0xe4);
    add4(third, add4(second, first))
}

/// `CMPLTPS v, 0` and `MOVMSKPS`, masked with 7: bit `i` is set when lane `i` is below 0.
fn below_zero_mask(v: Vec4) -> u32 {
    (0..3).fold(0, |mask, i| mask | (u32::from(v[i] < 0.0) << i))
}

/// `a.cross(b)` as the code forms it: `rot_c9(a) * rot_d2(b) - rot_d2(a) * rot_c9(b)`.
fn cross(a: Vec4, b: Vec4) -> Vec4 {
    sub4(
        mul4(rotate_c9(a), rotate_d2(b)),
        mul4(rotate_d2(a), rotate_c9(b)),
    )
}

/// The barycentric numerators of the point `p` for the triangle `a`, `b`, `c` as
/// `00e4da90`, `00e4dc70` and `00e4bbe0` compute them. With the edges
/// `ab = c - b`, `ca = a - c`, `ba = b - a` and the normal `n = ab x ca` the three products are
/// `(p - b) x ab`, `(p - c) x ca` and `(p - a) x ba`, each multiplied lane by lane with `n` and
/// gathered by `lane_sums` into `(x, y, z)`.
fn triangle_numerators(p: Vec4, a: Vec4, b: Vec4, c: Vec4) -> (Vec4, Vec4) {
    let ab = sub4(c, b);
    let ca = sub4(a, c);
    let ba = sub4(b, a);
    let normal = cross(ab, ca);
    let first = mul4(cross(sub4(p, b), ab), normal);
    let second = mul4(cross(sub4(p, c), ca), normal);
    let third = mul4(cross(sub4(p, a), ba), normal);
    (lane_sums(first, second, third), normal)
}

/// Copies sixteen bytes the way a `MOVAPS` pair does.
fn copy_vec4(e: &mut Engine, from: u32, to: u32) {
    let v = load_vec4(e, from);
    store_vec4(e, to, v);
}

// Translated from 00e4dc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGsk::checkTriangleBoundaries<0>` (Xbox PDB), `thiscall` (`ECX` = `out`, a 16-byte vector)
/// with the stack words `point` (a pointer to a vector), `triangle` (three vectors) and a
/// third that is not read. With the triangle `a`, `b`, `c` (vectors at `+0`, `+0x10`, `+0x20`)
/// the code forms the three edge products `(point - b) x (c - b)`, `(point - c) x (a - c)`
/// and `(point - a) x (b - a)` (`SHUFPS` rotations), multiplies each lane by lane with the
/// normal `(c - b) x (a - c)` and sums lanes 2, 1, 0 of each; the three sums go to lanes 0 to 2
/// of `out` (lane 3 holds `w3 + (w3 + w1)` of the third product). Answers the mask of the
/// lanes 0 to 2 that are below 0.
pub fn hk_gsk_check_triangle_boundaries_0(
    e: &mut Engine,
    out: u32,
    point: u32,
    triangle: u32,
    _unused: u32,
) -> u32 {
    let a = load_vec4(e, triangle);
    let b = load_vec4(e, triangle + 0x10);
    let c = load_vec4(e, triangle + 0x20);
    let p = load_vec4(e, point);
    let (sums, _) = triangle_numerators(p, a, b, c);
    store_vec4(e, out, sums);
    below_zero_mask(sums)
}

// Translated from 00e4da90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGsk::checkTriangleBoundaries<1>` (Xbox PDB), `thiscall` (`ECX` = `this`, the object
/// whose first vector gets the numerators) with the stack words `point` (a pointer to the
/// vector), `triangle` (three vectors, two more at `+0x40` and `+0x50`) and `normal_out`. It
/// forms the numerators as `00e4dc70` does (the first vector of `this`) and stores the normal at
/// `normal_out`. When all three are below 0 and the plane test (`(this[0x20] - this[0xa0])`
/// dotted with the normal, lanes 2, 1, 0 summed, has its sign bit set) holds, the triangle
/// is turned: the normal is negated (all four sign bits), the vertices 0 and 1 trade places,
/// so do the vectors at `+0x40` and `+0x50`, the first two `float`s of the numerators are
/// exchanged (through `FLD`/`FSTP`, which quiet signalling NaNs) and `this + 0x1c` becomes 1.
/// Answers the mask.
pub fn hk_gsk_check_triangle_boundaries_1(
    e: &mut Engine,
    this: u32,
    point: u32,
    triangle: u32,
    normal_out: u32,
) -> u32 {
    let a = load_vec4(e, triangle);
    let b = load_vec4(e, triangle + 0x10);
    let c = load_vec4(e, triangle + 0x20);
    let p = load_vec4(e, point);
    let (sums, normal) = triangle_numerators(p, a, b, c);
    store_vec4(e, this, sums);
    let mask = below_zero_mask(sums);
    store_vec4(e, normal_out, normal);
    if mask != 7 {
        return mask;
    }
    let depth = sub4(load_vec4(e, this + 0x20), load_vec4(e, this + 0xa0));
    let dot = sum3(mul4(depth, normal));
    if dot.to_bits() >> 31 == 0 {
        return mask;
    }
    let first = load_vec4(e, triangle);
    let flipped = [0, 1, 2, 3]
        .map(|i| f32::from_bits(normal[i].to_bits() ^ e.mem.u32(SIGN_MASK + 4 * i as u32)));
    let second = load_vec4(e, triangle + 0x10);
    store_vec4(e, triangle, second);
    store_vec4(e, triangle + 0x10, first);
    store_vec4(e, normal_out, flipped);
    let high = load_vec4(e, triangle + 0x50);
    let low = load_vec4(e, triangle + 0x40);
    store_vec4(e, triangle + 0x40, high);
    store_vec4(e, triangle + 0x50, low);
    let x = x87_quiet(e.mem.f32(this));
    let y = x87_quiet(e.mem.f32(this + 4));
    e.mem.set_f32(this, y);
    e.mem.set_f32(this + 4, x87_quiet(x));
    e.mem.set_u32(this + 0x1c, 1);
    mask
}

// Translated from 00e4b810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `findClosestTriangle_ov2` (Xbox PDB), cdecl (`point`, `tetrahedron`, `scale`): the faces of
/// the tetrahedron `t0..t3` (vectors at `+0x0`, `+0x10`, `+0x20`, `+0x30`) that touch `t3`
/// have the normals `n1 = (t2 - t1) x (t3 - t1)`, `n2 = (t0 - t2) x (t3 - t2)` and
/// `n3 = (t1 - t0) x (t3 - t0)` (the cross products as the `SHUFPS` code forms them). With
/// `h = scale * (point - t3)` the distances are `d = (h.n1, h.n2, h.n3)` (each dot product
/// `(h.y * n.y + h.x * n.x) + h.z * n.z`) and `q = |d| * d`. Each face's key is `q / |n|^2` as a
/// `float` through the x87 stack (the largest `float` at `0104da48` when `|n|^2` is exactly 0).
/// The largest key's face number is found by compares in the code's order (NaN as the flags
/// give), together with the middle key; when the largest is below the `float` at `010c6090`
/// the answer is -1, when `1.1` times the middle (`010290f8`) is below the largest the
/// answer is the largest's face. Otherwise the code sets the entry of `d` to -1.0 for
/// the face of each pair (`d_i`, `d_(i+1)`, both at least 0) whose barycentric test
/// (`s_pc * s_td - s_pd * s_tc` over the triangle `t_(i+2)`, `t_i`, `t_(i+1)` and `t3`) puts the
/// weight on the other side, and answers the first face whose `d` is above 0 (-1 when none is).
pub fn find_closest_triangle_ov2(e: &mut Engine, point: u32, tetrahedron: u32, scale: f32) -> i32 {
    let t = [0, 1, 2, 3].map(|i| load_vec4(e, tetrahedron + 0x10 * i));
    let p = load_vec4(e, point);
    let a = sub4(t[0], t[2]);
    let b = sub4(t[3], t[0]);
    let c = sub4(t[2], t[1]);
    let d = sub4(t[1], t[0]);
    let f = sub4(t[3], t[2]);
    let g = sub4(t[3], t[1]);
    let n1 = sub4(
        mul4(rotate_d2(g), rotate_c9(c)),
        mul4(rotate_c9(g), rotate_d2(c)),
    );
    let n2 = sub4(
        mul4(rotate_d2(f), rotate_c9(a)),
        mul4(rotate_c9(f), rotate_d2(a)),
    );
    let n3 = sub4(
        mul4(rotate_d2(b), rotate_c9(d)),
        mul4(rotate_c9(b), rotate_d2(d)),
    );
    let h = mul4([scale; 4], sub4(p, t[3]));
    let low = unpcklps(n1, n2);
    let high = unpckhps(n1, n2);
    let first = movlhps(low, n3);
    let second = shufps(movhlps(first, low), n3, 0xd4);
    let third = shufps(high, n3, 0xe4);
    let dots = add4(
        add4(
            mul4(shufps(h, h, 0x55), second),
            mul4(shufps(h, h, 0x00), first),
        ),
        mul4(shufps(h, h, 0xaa), third),
    );
    let mask = load_bits(e, ABS_MASK);
    let magnitude = [0, 1, 2, 3].map(|i| f32::from_bits(dots[i].to_bits() & mask[i]));
    let q = mul4(magnitude, dots);
    // The keys, through the x87 stack (the stack holds 0.0 and the largest float).
    let mut x = X87::new();
    x.fldz();
    x.fld_f32(e.mem.f32(LARGEST_FLOAT));
    let mut keys = [0.0f32; 4];
    for (i, normal) in [n1, n2, n3].into_iter().enumerate() {
        x.fld_st(1);
        let squared = sum3(mul4(normal, normal));
        x.fld_f32(squared);
        let flags = x.fcom(1);
        x.fstp_st(1);
        if !test_parity_even(flags, 0x44) {
            x.pop();
            keys[i] = x.fst_f32();
        } else {
            x.divr_m(float_to_x87(q[i]));
            keys[i] = x.fstp_f32();
        }
    }
    x.pop();
    // The largest key (`index`, an insertion sort over the three) with the middle one on
    // the x87 stack: the stack ends as [middle, largest, 0.0].
    let mut index = 1u32;
    x.fld_f32(keys[0]);
    x.fld_f32(keys[1]);
    let flags = x.fcom(1);
    if !test_parity_even(flags, 5) {
        x.fxch(1);
        index = 0;
        let held = x.fstp_f32();
        keys[0] = x.fstp_f32();
        x.fld_f32(held);
        keys[1] = x.fstp_f32();
        x.fld_f32(keys[0]);
        x.fld_f32(keys[1]);
    }
    x.fld_f32(keys[2]);
    let flags = x.fcom(1);
    if test_parity_even(flags, 5) {
        x.fstp_st(2);
        index = 2;
    } else {
        x.fxch(1);
        let held = x.fstp_f32();
        keys[1] = x.fstp_f32();
        x.fld_f32(held);
        keys[2] = x.fstp_f32();
        x.fld_f32(keys[1]);
        let flags = x.fcom(1);
        if test_parity_even(flags, 5) {
            x.fstp_st(1);
            x.fld_f32(keys[2]);
            x.fxch(1);
        } else {
            x.pop();
            keys[1] = x.fstp_f32();
            x.fld_f32(keys[2]);
            x.fld_f32(keys[1]);
        }
    }
    x.fld_f32(e.mem.f32(NEAR_ZERO));
    let flags = x.fcom(2);
    x.pop();
    if flags & 0x41 == 0 {
        x.pop();
        x.pop();
        x.pop();
        return -1;
    }
    x.mul_m(e.mem.f64(TIE_FACTOR));
    let flags = x.fcom(1);
    x.pop();
    x.pop();
    if !test_parity_even(flags, 5) {
        x.pop();
        return index as i32;
    }
    // The pairs (`d_i`, `d_(i+1)`) whose entries are both at least 0 (the entries of `dots` are
    // the words the code copies to its frame, lane 3 included).
    let mut entries = dots;
    for i in 0..3usize {
        let next = (i + 1) % 3;
        let skip = test_parity_even(compare_flags(0.0, float_to_x87(entries[i])), 0x41)
            || test_parity_even(compare_flags(0.0, float_to_x87(entries[next])), 0x41);
        if skip {
            continue;
        }
        let base = t[(i + 2) % 3];
        let to_c = sub4(t[i], base);
        let to_d = sub4(t[(i + 1) % 3], base);
        let to_point = sub4(p, base);
        let to_apex = sub4(t[3], base);
        let s_pc = sum3(mul4(to_point, to_c));
        let s_td = sum3(mul4(to_apex, to_d));
        let s_pd = sum3(mul4(to_point, to_d));
        let s_tc = sum3(mul4(to_apex, to_c));
        let value = s_pc * s_td - s_pd * s_tc;
        let minus_one = e.mem.f32(MINUS_ONE_FLOAT);
        if value.to_bits() >> 31 != 0 {
            entries[next] = x87_quiet(minus_one);
        } else {
            entries[i] = x87_quiet(minus_one);
        }
    }
    for (i, entry) in entries.iter().take(3).enumerate() {
        if !test_parity_even(compare_flags(0.0, float_to_x87(*entry)), 5) {
            return i as i32;
        }
    }
    -1
}

// Translated from 00e4bbe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGsk::processEdgeTriangle` (Xbox PDB), `thiscall` on `this` with the stack words `a` (an
/// array of vectors: the segment's end points at `+0` and `+0x10`, and two more at `+0x40` and
/// `+0x50`), `b` (the triangle: vectors at `+0`, `+0x10`, `+0x20`, with `+0x40`, `+0x50` and
/// `+0x60` beside them), `out_a`, `out_b` (int out words), `use_triangle` (a byte) and `extra`.
/// Answers 0, 1 or 2.
///
/// With `use_triangle` set the triangle's normal is `(b1 - b2) x (b0 - b2)`; with it clear the
/// normal is the vector at `this + 0x120`. `s0` and `s1` are the end points' distances to the
/// plane (`(a_k - b0) . n`, lanes 2, 1, 0 summed). When their product has its sign bit set the
/// segment crosses the plane: the crossing point `x = (1 - t) * a0 + t * a1`, `t = s0 / (s0 - s1)`,
/// is tested against the triangle (`barycentric numerators`, all three below 0 answers 1).
///
/// Triangle case (`use_triangle` set), not inside: when the segment does not cross, the
/// end points are exchanged (with the vectors at `+0x40` and `+0x50`) if `s1 * s1 < s0 * s0` and
/// `00e4da90` runs on `(this, a, b, this + 0x120)`; a mask of 7 stores 1 at `out_a` and answers 0.
/// Otherwise the closest points of the segment (`a0`, `a1 - a0`) and the two edges `b2 -> b1`
/// and `b2 -> b0` (`00d22e40`) decide which of `b0`, `b1` is replaced by `b2` (and the vector at
/// `+0x40 + 0x10 * index` by the one at `+0x60`): the edge with the smaller squared
/// distance (at `+0x20` of its result; the second wins only when strictly smaller).
/// `out_b` gets 2 and 2 is answered.
///
/// Normal case (`use_triangle` clear): `00e4dc70` runs on the second end point; mask 7 stores
/// 1 at `out_a`, copies `a1` over `a0` (and the vector at `+0x50` over `+0x40`), stores `extra` at
/// `this + 0x14` and answers 0. Any other mask stores 2 at `out_b` and picks the vertex
/// replaced by `b2` from the mask (`b0` for 6, `b1` for 5, neither for 3, and for 1, 2 and
/// 4 by comparing the products of the old and new numerators on the x87 stack) and answers 2.
pub fn hk_gsk_process_edge_triangle(
    e: &mut Engine,
    this: u32,
    a: u32,
    b: u32,
    out_a: u32,
    out_b: u32,
    use_triangle: u8,
    extra: u32,
) -> u32 {
    if use_triangle != 0 {
        let b0 = load_vec4(e, b);
        let b2 = load_vec4(e, b + 0x20);
        let b1 = load_vec4(e, b + 0x10);
        let a0 = load_vec4(e, a);
        let a1 = load_vec4(e, a + 0x10);
        let e12 = sub4(b1, b2);
        let e02 = sub4(b0, b2);
        let normal = sub4(
            mul4(rotate_d2(e02), rotate_c9(e12)),
            mul4(rotate_c9(e02), rotate_d2(e12)),
        );
        let s0 = sum3(mul4(sub4(a0, b0), normal));
        let s1 = sum3(mul4(sub4(a1, b0), normal));
        if (s0 * s1).to_bits() >> 31 != 0 {
            let crossing = crossing_point(e, a0, a1, s0, s1);
            let (numerators, _) = triangle_numerators(crossing, b0, b1, b2);
            if below_zero_mask(numerators) == 7 {
                return 1;
            }
        } else {
            let first = x87_quiet(s0 * s0);
            let second = x87_quiet(s1 * s1);
            if !test_parity_even(compare_flags(float_to_x87(second), float_to_x87(first)), 5) {
                swap_vec4(e, a, a + 0x10);
                swap_vec4(e, a + 0x40, a + 0x50);
            }
            let result = hk_gsk_check_triangle_boundaries_1(e, this, a, b, this + 0x120);
            if result == 7 {
                e.mem.set_u32(out_a, 1);
                return 0;
            }
        }
        // Each closest-points result of 00d22e40 is 0x30 bytes (its distance at +0x20): the
        // exe keeps them at ESP+0x64 and ESP+0x94 of its `SUB ESP,0xc4` frame (direction at
        // +0x14, edges at +0x34 and +0x44), so the second result runs to +0x90 here.
        return e.with_stack(0x90, |e, frame| {
            let base = frame.addr();
            let direction = base;
            let edge_12 = base + 0x10;
            let edge_02 = base + 0x20;
            let first_result = base + 0x30;
            let second_result = base + 0x60;
            let a0 = load_vec4(e, a);
            let a1 = load_vec4(e, a + 0x10);
            store_vec4(e, direction, sub4(a1, a0));
            store_vec4(e, edge_12, e12);
            store_vec4(e, edge_02, e02);
            e.call(
                CLOSEST_LINE_SEG_LINE_SEG,
                &args![a, direction, b + 0x20, edge_12, first_result],
            );
            e.call(
                CLOSEST_LINE_SEG_LINE_SEG,
                &args![a, direction, b + 0x20, edge_02, second_result],
            );
            let first_distance = e.mem.f32(first_result + 0x20);
            let second_distance = e.mem.f32(second_result + 0x20);
            let index = if test_parity_even(
                compare_flags(float_to_x87(second_distance), float_to_x87(first_distance)),
                5,
            ) {
                0
            } else {
                1
            };
            copy_vec4(e, b + 0x20, b + 0x10 * index);
            copy_vec4(e, b + 0x60, b + 0x10 * (index + 4));
            e.mem.set_u32(out_b, 2);
            2
        });
    }
    let old = load_vec4(e, this);
    let a0 = load_vec4(e, a);
    let b0 = load_vec4(e, b);
    let normal = load_vec4(e, this + 0x120);
    let s0 = sum3(mul4(sub4(a0, b0), normal));
    let a1 = load_vec4(e, a + 0x10);
    let s1 = sum3(mul4(sub4(a1, b0), normal));
    if (s0 * s1).to_bits() >> 31 != 0 {
        let b1 = load_vec4(e, b + 0x10);
        let b2 = load_vec4(e, b + 0x20);
        let crossing = crossing_point(e, a0, a1, s0, s1);
        let (numerators, _) = triangle_numerators(crossing, b0, b1, b2);
        if below_zero_mask(numerators) == 7 {
            return 1;
        }
    }
    let result = hk_gsk_check_triangle_boundaries_0(e, this, a + 0x10, b, this + 0x120);
    if result == 7 {
        e.mem.set_u32(out_a, 1);
        copy_vec4(e, a + 0x10, a);
        copy_vec4(e, a + 0x50, a + 0x40);
        e.mem.set_u32(this + 0x14, extra);
        return 0;
    }
    e.mem.set_u32(out_b, 2);
    let new = load_vec4(e, this);
    let replace_b0 = |e: &mut Engine| {
        copy_vec4(e, b + 0x20, b);
        copy_vec4(e, b + 0x60, b + 0x40);
    };
    let replace_b1 = |e: &mut Engine| {
        copy_vec4(e, b + 0x20, b + 0x10);
        copy_vec4(e, b + 0x60, b + 0x50);
    };
    // The products compared (as `float`s through the x87 stack): `left` is pushed first and
    // `right` last, and `FCOMPP` compares `right` with `left`.
    let compare = |left: f32, right: f32| {
        let left = x87_quiet(left);
        let right = x87_quiet(right);
        compare_flags(float_to_x87(right), float_to_x87(left))
    };
    match result {
        6 => replace_b0(e),
        5 => replace_b1(e),
        3 => {}
        1 => {
            let flags = compare(old[2] * (new[1] - old[1]), old[1] * (new[2] - old[2]));
            if test_parity_even(flags, 5) {
                replace_b1(e);
            }
        }
        2 => {
            let flags = compare(old[0] * (new[2] - old[2]), old[2] * (new[0] - old[0]));
            if !test_parity_even(flags, 5) {
                replace_b0(e);
            }
        }
        4 => {
            let flags = compare(old[1] * (new[0] - old[0]), old[0] * (new[1] - old[1]));
            if test_parity_even(flags, 5) {
                replace_b0(e);
            } else {
                replace_b1(e);
            }
        }
        _ => replace_b0(e),
    }
    2
}

/// `(1 - t) * a0 + t * a1` with `t = s0 / (s0 - s1)` (`DIVSS`, then a splat; the ones are the
/// vector at `010ec7f0`): the point where the segment meets the plane.
fn crossing_point(e: &Engine, a0: Vec4, a1: Vec4, s0: f32, s1: f32) -> Vec4 {
    let t = [s0 / (s0 - s1); 4];
    let ones = load_vec4(e, ONES);
    add4(mul4(sub4(ones, t), a0), mul4(a1, t))
}

/// Exchanges two 16-byte vectors.
fn swap_vec4(e: &mut Engine, first: u32, second: u32) {
    let a = load_vec4(e, first);
    let b = load_vec4(e, second);
    store_vec4(e, first, b);
    store_vec4(e, second, a);
}

// Translated from 00e4c230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGsk::handlePenetration` (Xbox PDB), `thiscall` on `this` with the stack words `first`,
/// `second`, `matrix` (four vectors), `count_a`, `count_b` (int in/out words) and `direction`
/// (a vector). `na` and `nb` are the counts; `this + 0x20 + 0x10 * k` and `this + 0xa0 + 0x10 * k`
/// are two lists of vectors, `this + 0xe0 + 0x10 * k` and `this + 0x20 + ...` the working
/// points of the two sets (unconfirmed meaning). The lists are first padded out by copying
/// their first vectors (the cases of `na` and `nb` compared with 1, 2 and 3 as the code
/// has them, the result 1, 3 or 4 is kept), a block of vectors is built (the matrix, `1e-4`
/// in every lane from `010141cc`, the `float` at `010dbb58`, the two arguments, the two lists, the
/// four differences of the lists) and handed to the routine at `00e63770` (`thiscall`, with
/// `direction`). When that answers 5 the routine `00e62740` runs on `(first, second, matrix,
/// this + 0x20, this + 0xe0, direction)`, both counts become 1 and `count_a` is answered.
/// Otherwise both counts become 3, each is lowered by one when the two last tags (a word
/// masked with `0xc0ffffff`) match, the matching set is shortened, the second set's
/// vectors are moved through the matrix (`v.y * m1 + v.x * m0 + (v.z * m2 + m3)` stored 4
/// places lower), and the counts are reduced to a pair that fits into 4: when `na + nb` is
/// above 4 the larger count becomes 3 and the other 1. `this + 0x120` receives `direction`.
/// The value the code leaves in `EAX` is returned.
pub fn hk_gsk_handle_penetration(
    e: &mut Engine,
    this: u32,
    first: u32,
    second: u32,
    matrix: u32,
    count_a: u32,
    count_b: u32,
    direction: u32,
) -> u32 {
    let na = e.mem.i32(count_a);
    let nb = e.mem.i32(count_b);
    let slot = |k: u32| this + 0x10 * k;
    let mut kind = 4u32;
    if nb >= 3 {
        if nb > 3 {
            copy_vec4(e, slot(2), slot(5));
            copy_vec4(e, slot(2), slot(3));
            copy_vec4(e, slot(2), slot(4));
        } else if na > 1 {
            copy_vec4(e, slot(3), slot(5));
            copy_vec4(e, slot(0xe), slot(0x11));
            copy_vec4(e, slot(0xa), slot(0xd));
            copy_vec4(e, slot(2), slot(3));
            copy_vec4(e, slot(2), slot(4));
        } else {
            copy_vec4(e, slot(2), slot(3));
            kind = 3;
            copy_vec4(e, slot(2), slot(4));
        }
    } else {
        let mut tail = false;
        if na >= 3 {
            if na > 3 {
                copy_vec4(e, slot(0xe), slot(0x11));
                copy_vec4(e, slot(0xa), slot(0xd));
            } else if nb > 1 {
                copy_vec4(e, slot(2), slot(5));
                copy_vec4(e, slot(0xf), slot(0x11));
                copy_vec4(e, slot(0xb), slot(0xd));
            } else {
                kind = 3;
            }
            tail = true;
        }
        if tail {
            let low = load_vec4(e, slot(0xe));
            let high = load_vec4(e, slot(0xa));
            store_vec4(e, slot(0xf), low);
            store_vec4(e, slot(0xb), high);
            store_vec4(e, slot(0x10), low);
            store_vec4(e, slot(0xc), high);
        } else {
            let na_u = na as u32;
            let nb_u = nb as u32;
            let temp = load_vec4(e, slot(2));
            store_vec4(e, slot(na_u.wrapping_add(2)), temp);
            store_vec4(e, slot(na_u.wrapping_add(3)), temp);
            let temp = load_vec4(e, slot(0xe));
            store_vec4(e, slot(nb_u.wrapping_add(0xe)), temp);
            store_vec4(e, slot(nb_u.wrapping_add(0xf)), temp);
            let temp = load_vec4(e, slot(0xa));
            store_vec4(e, slot(nb_u.wrapping_add(0xa)), temp);
            store_vec4(e, slot(nb_u.wrapping_add(0xb)), temp);
            kind = 1;
        }
    }
    let epsilon = x87_quiet(e.mem.f32(SMALL_EPSILON));
    let extra_float = x87_quiet(e.mem.f32(PENETRATION_FLOAT));
    let diffs = [
        sub4(load_vec4(e, slot(2)), load_vec4(e, slot(0xa))),
        sub4(load_vec4(e, slot(3)), load_vec4(e, slot(0xb))),
        sub4(load_vec4(e, slot(4)), load_vec4(e, slot(0xc))),
        sub4(load_vec4(e, slot(5)), load_vec4(e, slot(0xd))),
    ];
    let count = e.with_stack(0xb0, |e, block| {
        let at = block.addr();
        for k in 0..4u32 {
            copy_vec4(e, matrix + 0x10 * k, at + 0x10 * k);
        }
        store_vec4(e, at + 0x40, [epsilon; 4]);
        e.mem.set_f32(at + 0x50, extra_float);
        e.mem.set_u32(at + 0x54, first);
        e.mem.set_u32(at + 0x58, second);
        e.mem.set_u32(at + 0x5c, 0);
        e.mem.set_u32(at + 0x60, this + 0x20);
        e.mem.set_u32(at + 0x64, this + 0xe0);
        e.mem.set_u32(at + 0x68, at + 0x70);
        e.mem.set_u32(at + 0x6c, kind);
        for (k, diff) in diffs.into_iter().enumerate() {
            store_vec4(e, at + 0x70 + 0x10 * k as u32, diff);
        }
        e.call(UNNAMED_00E63770, &args![block, direction]).u32()
    });
    if count == 5 {
        e.call(
            UNNAMED_00E62740,
            &args![first, second, matrix, this + 0x20, this + 0xe0, direction],
        );
        e.mem.set_u32(count_a, 1);
        e.mem.set_u32(count_b, 1);
        copy_vec4(e, direction, this + 0x120);
        return count_a;
    }
    e.mem.set_u32(count_a, 3);
    e.mem.set_u32(count_b, 3);
    let tag = |e: &Engine, at: u32| e.mem.u32(this + at) & TAG_MASK;
    if tag(e, 0x3c) == tag(e, 0x4c) || tag(e, 0x2c) == tag(e, 0x4c) {
        let value = e.mem.i32(count_a).wrapping_sub(1);
        e.mem.set_i32(count_a, value);
    }
    let remaining = e.mem.i32(count_a);
    if remaining >= 2 && (e.mem.u32(this + 0x2c) ^ e.mem.u32(this + 0x3c)) & TAG_MASK == 0 {
        let lowered = remaining - 1;
        e.mem.set_i32(count_a, lowered);
        copy_vec4(e, slot((lowered as u32).wrapping_add(2)), slot(2));
    }
    if tag(e, 0xfc) == tag(e, 0x10c) || tag(e, 0xec) == tag(e, 0x10c) {
        let value = e.mem.i32(count_b).wrapping_sub(1);
        e.mem.set_i32(count_b, value);
    }
    let remaining = e.mem.i32(count_b);
    if remaining >= 2 && (e.mem.u32(this + 0xec) ^ e.mem.u32(this + 0xfc)) & TAG_MASK == 0 {
        let lowered = remaining - 1;
        e.mem.set_i32(count_b, lowered);
        copy_vec4(e, slot((lowered as u32).wrapping_add(0xe)), slot(0xe));
    }
    let rows = [0, 1, 2, 3].map(|k| load_vec4(e, matrix + 0x10 * k));
    let mut k = e.mem.i32(count_b) - 1;
    while k >= 0 {
        let v = load_vec4(e, slot(k as u32 + 0xe));
        let transformed = add4(
            add4(mul4([v[1]; 4], rows[1]), mul4([v[0]; 4], rows[0])),
            add4(mul4([v[2]; 4], rows[2]), rows[3]),
        );
        store_vec4(e, slot(k as u32 + 0xa), transformed);
        k -= 1;
    }
    let na = e.mem.i32(count_a);
    let nb = e.mem.i32(count_b);
    if na.wrapping_add(nb) <= 4 {
        copy_vec4(e, direction, this + 0x120);
        return na as u32;
    }
    if na <= nb {
        e.mem.set_i32(count_b, 3);
        e.mem.set_i32(count_a, 1);
        copy_vec4(e, direction, this + 0x120);
        return direction;
    }
    e.mem.set_i32(count_a, 3);
    e.mem.set_i32(count_b, 1);
    copy_vec4(e, direction, this + 0x120);
    na as u32
}

// BEGIN c730

/// `AND`/`ANDN`/`XOR` of two vectors on their bit patterns (`ANDPS`, `ANDNPS`, `XORPS`).
fn andps(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| f32::from_bits(a[i].to_bits() & b[i].to_bits()))
}

fn andnps(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| f32::from_bits(!a[i].to_bits() & b[i].to_bits()))
}

fn xorps(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| f32::from_bits(a[i].to_bits() ^ b[i].to_bits()))
}

/// `CMPLTPS`: all ones in the lanes where `a < b` (ordered), else 0.
fn cmpltps(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| f32::from_bits(if a[i] < b[i] { u32::MAX } else { 0 }))
}

/// `CMPEQSS`: lane 0 becomes all ones when `a[0] == s`, else 0; the other lanes stay.
fn cmpeqss(a: Vec4, s: f32) -> Vec4 {
    [
        f32::from_bits(if a[0] == s { u32::MAX } else { 0 }),
        a[1],
        a[2],
        a[3],
    ]
}

/// `MOVMSKPS`: the sign bits of the four lanes.
fn movmskps(a: Vec4) -> u32 {
    (0..4).fold(0, |mask, i| mask | ((a[i].to_bits() >> 31) << i))
}

/// The integer flags the branches of `getClosestPoint` test.
#[derive(Clone, Copy, Default)]
struct Flags {
    zf: bool,
    sf: bool,
    cf: bool,
    of: bool,
    pf: bool,
}

impl Flags {
    fn mask(bits: u32) -> u32 {
        if bits == 32 {
            u32::MAX
        } else {
            (1 << bits) - 1
        }
    }

    fn zsp(&mut self, result: u32, bits: u32) {
        let r = result & Self::mask(bits);
        self.zf = r == 0;
        self.sf = r >> (bits - 1) & 1 != 0;
        self.pf = (r as u8).count_ones() % 2 == 0;
    }

    fn add(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        let m = Self::mask(bits);
        let (a, b) = (a & m, b & m);
        let wide = u64::from(a) + u64::from(b);
        let r = (wide as u32) & m;
        self.cf = wide > u64::from(m);
        self.of = ((a ^ r) & (b ^ r)) >> (bits - 1) & 1 != 0;
        self.zsp(r, bits);
        r
    }

    fn sub(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        let m = Self::mask(bits);
        let (a, b) = (a & m, b & m);
        let r = a.wrapping_sub(b) & m;
        self.cf = a < b;
        self.of = ((a ^ b) & (a ^ r)) >> (bits - 1) & 1 != 0;
        self.zsp(r, bits);
        r
    }

    fn logic(&mut self, r: u32, bits: u32) -> u32 {
        let r = r & Self::mask(bits);
        self.cf = false;
        self.of = false;
        self.zsp(r, bits);
        r
    }

    fn and(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        self.logic(a & b, bits)
    }

    fn or(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        self.logic(a | b, bits)
    }

    fn xor(&mut self, a: u32, b: u32, bits: u32) -> u32 {
        self.logic(a ^ b, bits)
    }

    /// `INC` and `DEC` leave the carry flag.
    fn inc(&mut self, a: u32, bits: u32) -> u32 {
        let carry = self.cf;
        let r = self.add(a, 1, bits);
        self.cf = carry;
        r
    }

    fn dec(&mut self, a: u32, bits: u32) -> u32 {
        let carry = self.cf;
        let r = self.sub(a, 1, bits);
        self.cf = carry;
        r
    }

    fn shl(&mut self, a: u32, count: u32, bits: u32) -> u32 {
        let m = Self::mask(bits);
        let wide = u64::from(a & m) << count;
        let r = (wide as u32) & m;
        self.cf = (wide >> bits) & 1 != 0;
        self.of = false;
        self.zsp(r, bits);
        r
    }

    fn sar(&mut self, a: u32, count: u32, bits: u32) -> u32 {
        let m = Self::mask(bits);
        let signed = match bits {
            8 => i32::from(a as u8 as i8),
            16 => i32::from(a as u16 as i16),
            _ => a as i32,
        };
        let r = ((signed >> count) as u32) & m;
        self.cf = (signed >> (count - 1)) & 1 != 0;
        self.of = false;
        self.zsp(r, bits);
        r
    }
}

// Translated from 00e4c730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGsk::getClosestPoint` (Xbox PDB name, Ghidra: `getClosestPoint`), `thiscall` on the GJK state
/// `this` with the stack words `input`, `ids` (a byte array, or null) and `output`; answers the
/// code the exe leaves in `EAX` (0, 3 or 4).
///
/// `output` is written by the last blocks and handed on to `handlePenetration`; its layout (eight
/// words) is only known here from the recorded answers in the tests, not named from the exe.
///
/// What the instruction stream reads of `input`: a transform of four vectors at `+0`, `+0x10`,
/// `+0x20` and `+0x30` (rotation rows and the translation: the second shape's points are carried
/// through it), the two shape objects at `+0x40` and `+0x44` (virtual slot `4` answers the
/// supporting vertex in a direction, slot `8` turns vertex ids back into vertices) and one more
/// vector at `+0x50` (what it is has not been confirmed; the call at `00e4cf79` adds it in).
///
/// The state: the first shape's points are at `this + 0x20`, the second shape's own points at
/// `this + 0xe0` and the same carried through the transform at `this + 0xa0` (each vector's fourth
/// lane holds the vertex id); `this + 0x60` holds a second block of four vectors that the exe
/// fills but whose meaning has not been confirmed. The direction (its fourth lane is written back
/// by the code) is at `this + 0x120` and the witness vector at `this + 0x130`; the words at
/// `this + 0x10..0x1c` are the counters the loop updates. Two stack words of the frame
/// (`[ESP + 0x14]` and `[ESP + 0x18]` once the prologue has run) hold the number of points of
/// each set and `[ESP + 0x20]` holds the answer.
///
/// A null `ids` starts with one supporting vertex of each shape: the direction `(1, 0, 0)` for
/// the first, its opposite taken through `004b4ab0` (unnamed in Ghidra) for the second, which
/// `setRotatedDir` (`00c7f700`) then carries through the transform. A non-null `ids` rebuilds
/// both sets from the vertex ids of the previous call (byte `4` holds the number of ids of the
/// first set in its high nibble and of the second in its low nibble) and carries the second set
/// through the transform.
///
/// The main loop takes `(count_a * 8 | count_b) - 9` and, through a byte map and a table of ten
/// handler addresses, picks the code for that pair of counts; the handlers find the feature of
/// the pair nearest the origin of their difference (`findClosestTriangle_ov2` `00e4b810`,
/// `checkTriangleBoundaries<1>` `00e4da90`, `processEdgeTriangle` `00e4bbe0`,
/// `closestLineSegLineSeg` `00d22e40` and inline barycentric tests), drop the points they no
/// longer need, normalise the direction (a refined `RSQRTSS`) and test it; `handlePenetration`
/// (`00e4c230`) runs when the sets have touched. Before the answer the vertex ids of both sets
/// and their counts are written back to `ids` (when it is not null).
///
/// Written as a transcription of the instruction stream: the registers are locals, the stack
/// words of the exe's frame are a block of memory at the exe's offsets (words the exe never
/// wrote read as 0 here, as stale stack words there; the same for the vector registers at entry),
/// the pushes for a call are a vector and every jump target is a block of one `match`. The jump
/// tables (`00e4da08`, `00e4da50` and `00e4da70`) and the byte map (`00e4da34`) are `match`es on the
/// case value;
/// the flags a branch tests are the integer flags the exe's compare, test, arithmetic or shift
/// before it left.
#[allow(unused_assignments, unused_variables, unused_mut, unused_parens)]
pub fn hk_gsk_get_closest_point(
    e: &mut Engine,
    this: u32,
    input: u32,
    ids: u32,
    output: u32,
) -> u32 {
    e.with_stack(0x400, |e, frame| {
        // `[ESP + k]` after the prologue's `SUB ESP` lives at `f0 + k - depth`.
        let f0 = frame.addr() + 0x100;
        let (mut eax, mut ecx, mut edx, mut ebx, mut esi, mut edi) =
            (0u32, this, 0u32, 0u32, 0u32, 0u32);
        let mut x = [[0.0f32; 4]; 8];
        let mut fl = Flags::default();
        let mut fsw = 0u8;
        let mut x87 = X87::new();
        let mut stk: Vec<u32> = Vec::new();
        let mut pc = 0x00e4_c73cu32;
        loop {
            match pc {
                0x00e4c73c => {
                    // 00e4c73c: CMP dword ptr [EBP + 0xc],0x0
                    fl.sub(ids, 0x00000000u32, 32);
                    // 00e4c740: MOVSS XMM1,dword ptr [0x01011d78]
                    x[1] = [f32::from_bits(e.mem.u32(0x01011d78)), 0.0, 0.0, 0.0];
                    // 00e4c748: PUSH EBX
                    stk.push(ebx);
                    // 00e4c749: MOV EBX,dword ptr [EBP + 0x8]
                    ebx = input;
                    // 00e4c74c: MOV EAX,dword ptr [EBX + 0x40]
                    eax = e.mem.u32(ebx.wrapping_add(0x40));
                    // 00e4c74f: PUSH ESI
                    stk.push(esi);
                    // 00e4c750: MOV ESI,ECX
                    esi = ecx;
                    // 00e4c752: MOV ECX,dword ptr [EBX + 0x44]
                    ecx = e.mem.u32(ebx.wrapping_add(0x44));
                    // 00e4c755: PUSH EDI
                    stk.push(edi);
                    // 00e4c756: MOV dword ptr [ESP + 0x28],EAX
                    e.mem.set_u32(f0.wrapping_add(0x1c), eax);
                    // 00e4c75a: MOV dword ptr [ESP + 0x24],ECX
                    e.mem.set_u32(f0.wrapping_add(0x18), ecx);
                    // 00e4c75e: JZ 0x00e4c89d
                    if fl.zf {
                        pc = 0x00e4c89d;
                        continue;
                    }
                    // 00e4c764: MOV EDX,dword ptr [EBP + 0xc]
                    edx = ids;
                    // 00e4c767: MOVZX EAX,byte ptr [EDX + 0x4]
                    eax = u32::from(e.mem.u8(edx.wrapping_add(0x4)));
                    // 00e4c76b: MOV ECX,dword ptr [ECX]
                    ecx = e.mem.u32(ecx);
                    // 00e4c76d: MOV EDX,EAX
                    edx = eax;
                    // 00e4c76f: AND EDX,0xf
                    {
                        let r = fl.and(edx, 0x0000000fu32, 32);
                        edx = r;
                    }
                    // 00e4c772: SAR EAX,0x4
                    {
                        let r = fl.sar(eax, 4, 32);
                        eax = r;
                    }
                    // 00e4c775: LEA EDI,[ESI + 0xe0]
                    edi = esi.wrapping_add(0xe0);
                    // 00e4c77b: PUSH EDI
                    stk.push(edi);
                    // 00e4c77c: PUSH EDX
                    stk.push(edx);
                    // 00e4c77d: MOV dword ptr [ESP + 0x1c],EDX
                    e.mem.set_u32(f0.wrapping_add(0x8), edx);
                    // 00e4c781: MOV EDX,dword ptr [EBP + 0xc]
                    edx = ids;
                    // 00e4c784: MOV dword ptr [ESP + 0x20],EAX
                    e.mem.set_u32(f0.wrapping_add(0xc), eax);
                    // 00e4c788: ADD EAX,EDX
                    {
                        let r = fl.add(eax, edx, 32);
                        eax = r;
                    }
                    // 00e4c78a: MOV dword ptr [ESP + 0x28],ECX
                    e.mem.set_u32(f0.wrapping_add(0x14), ecx);
                    // 00e4c78e: MOV ECX,dword ptr [ESP + 0x2c]
                    ecx = e.mem.u32(f0.wrapping_add(0x18));
                    // 00e4c792: PUSH EAX
                    stk.push(eax);
                    // 00e4c793: MOV EAX,dword ptr [ESP + 0x2c]
                    eax = e.mem.u32(f0.wrapping_add(0x14));
                    // 00e4c797: MOV EAX,dword ptr [EAX + 0x8]
                    eax = e.mem.u32(eax.wrapping_add(0x8));
                    // 00e4c79a: CALL EAX
                    {
                        let mut w = vec![ecx];
                        w.extend(stk.iter().rev().take(3));
                        let r = e.call(eax, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    stk.truncate(stk.len() - 3);
                    // 00e4c79c: MOV EAX,dword ptr [ESP + 0x18]
                    eax = e.mem.u32(f0.wrapping_add(0xc));
                    // 00e4c7a0: MOV ECX,dword ptr [ESP + 0x28]
                    ecx = e.mem.u32(f0.wrapping_add(0x1c));
                    // 00e4c7a4: MOV EDX,dword ptr [ECX]
                    edx = e.mem.u32(ecx);
                    // 00e4c7a6: MOV EDX,dword ptr [EDX + 0x8]
                    edx = e.mem.u32(edx.wrapping_add(0x8));
                    // 00e4c7a9: LEA EDI,[ESI + 0x20]
                    edi = esi.wrapping_add(0x20);
                    // 00e4c7ac: PUSH EDI
                    stk.push(edi);
                    // 00e4c7ad: PUSH EAX
                    stk.push(eax);
                    // 00e4c7ae: MOV EAX,dword ptr [EBP + 0xc]
                    eax = ids;
                    // 00e4c7b1: PUSH EAX
                    stk.push(eax);
                    // 00e4c7b2: CALL EDX
                    {
                        let mut w = vec![ecx];
                        w.extend(stk.iter().rev().take(3));
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    stk.truncate(stk.len() - 3);
                    // 00e4c7b4: MOV ECX,dword ptr [EBX + 0x4]
                    ecx = e.mem.u32(ebx.wrapping_add(0x4));
                    // 00e4c7b7: MOV EAX,dword ptr [EBX]
                    eax = e.mem.u32(ebx);
                    // 00e4c7b9: MOV EDX,dword ptr [EBX + 0x8]
                    edx = e.mem.u32(ebx.wrapping_add(0x8));
                    // 00e4c7bc: MOV dword ptr [ESP + 0x70],EAX
                    e.mem.set_u32(f0.wrapping_add(0x64), eax);
                    // 00e4c7c0: MOV EAX,dword ptr [EBX + 0xc]
                    eax = e.mem.u32(ebx.wrapping_add(0xc));
                    // 00e4c7c3: MOV dword ptr [ESP + 0x74],ECX
                    e.mem.set_u32(f0.wrapping_add(0x68), ecx);
                    // 00e4c7c7: MOV ECX,dword ptr [EBX + 0x10]
                    ecx = e.mem.u32(ebx.wrapping_add(0x10));
                    // 00e4c7ca: MOV dword ptr [ESP + 0x7c],EAX
                    e.mem.set_u32(f0.wrapping_add(0x70), eax);
                    // 00e4c7ce: MOV EAX,dword ptr [EBX + 0x18]
                    eax = e.mem.u32(ebx.wrapping_add(0x18));
                    // 00e4c7d1: MOV dword ptr [ESP + 0x90],ECX
                    e.mem.set_u32(f0.wrapping_add(0x84), ecx);
                    // 00e4c7d8: MOV ECX,dword ptr [EBX + 0x1c]
                    ecx = e.mem.u32(ebx.wrapping_add(0x1c));
                    // 00e4c7db: MOV dword ptr [ESP + 0x98],EAX
                    e.mem.set_u32(f0.wrapping_add(0x8c), eax);
                    // 00e4c7e2: MOV EAX,dword ptr [EBX + 0x24]
                    eax = e.mem.u32(ebx.wrapping_add(0x24));
                    // 00e4c7e5: MOV dword ptr [ESP + 0x9c],ECX
                    e.mem.set_u32(f0.wrapping_add(0x90), ecx);
                    // 00e4c7ec: MOV ECX,dword ptr [EBX + 0x28]
                    ecx = e.mem.u32(ebx.wrapping_add(0x28));
                    // 00e4c7ef: MOV dword ptr [ESP + 0x78],EDX
                    e.mem.set_u32(f0.wrapping_add(0x6c), edx);
                    // 00e4c7f3: MOV EDX,dword ptr [EBX + 0x14]
                    edx = e.mem.u32(ebx.wrapping_add(0x14));
                    // 00e4c7f6: MOVAPS XMM1,xmmword ptr [ESP + 0x70]
                    x[1] = load_vec4(e, f0.wrapping_add(0x64));
                    // 00e4c7fb: MOV dword ptr [ESP + 0x64],EAX
                    e.mem.set_u32(f0.wrapping_add(0x58), eax);
                    // 00e4c7ff: MOV EAX,dword ptr [EBX + 0x30]
                    eax = e.mem.u32(ebx.wrapping_add(0x30));
                    // 00e4c802: MOV dword ptr [ESP + 0x68],ECX
                    e.mem.set_u32(f0.wrapping_add(0x5c), ecx);
                    // 00e4c806: MOV ECX,dword ptr [EBX + 0x34]
                    ecx = e.mem.u32(ebx.wrapping_add(0x34));
                    // 00e4c809: MOV dword ptr [ESP + 0x94],EDX
                    e.mem.set_u32(f0.wrapping_add(0x88), edx);
                    // 00e4c810: MOV EDX,dword ptr [EBX + 0x20]
                    edx = e.mem.u32(ebx.wrapping_add(0x20));
                    // 00e4c813: MOVAPS XMM2,xmmword ptr [ESP + 0x90]
                    x[2] = load_vec4(e, f0.wrapping_add(0x84));
                    // 00e4c81b: MOV dword ptr [ESP + 0x80],EAX
                    e.mem.set_u32(f0.wrapping_add(0x74), eax);
                    // 00e4c822: MOV EAX,dword ptr [EBX + 0x3c]
                    eax = e.mem.u32(ebx.wrapping_add(0x3c));
                    // 00e4c825: MOV dword ptr [ESP + 0x84],ECX
                    e.mem.set_u32(f0.wrapping_add(0x78), ecx);
                    // 00e4c82c: MOV ECX,dword ptr [ESP + 0x14]
                    ecx = e.mem.u32(f0.wrapping_add(0x8));
                    // 00e4c830: MOV dword ptr [ESP + 0x60],EDX
                    e.mem.set_u32(f0.wrapping_add(0x54), edx);
                    // 00e4c834: MOV EDX,dword ptr [EBX + 0x2c]
                    edx = e.mem.u32(ebx.wrapping_add(0x2c));
                    // 00e4c837: MOV dword ptr [ESP + 0x8c],EAX
                    e.mem.set_u32(f0.wrapping_add(0x80), eax);
                    // 00e4c83e: DEC ECX
                    {
                        let r = fl.dec(ecx, 32);
                        ecx = r;
                    }
                    // 00e4c83f: MOV dword ptr [ESP + 0x6c],EDX
                    e.mem.set_u32(f0.wrapping_add(0x60), edx);
                    // 00e4c843: MOV EDX,dword ptr [EBX + 0x38]
                    edx = e.mem.u32(ebx.wrapping_add(0x38));
                    // 00e4c846: MOVAPS XMM3,xmmword ptr [ESP + 0x60]
                    x[3] = load_vec4(e, f0.wrapping_add(0x54));
                    // 00e4c84b: LEA EAX,[ECX + 0xe]
                    eax = ecx.wrapping_add(0xe);
                    // 00e4c84e: SHL EAX,0x4
                    {
                        let r = fl.shl(eax, 4, 32);
                        eax = r;
                    }
                    // 00e4c851: MOV dword ptr [ESP + 0x88],EDX
                    e.mem.set_u32(f0.wrapping_add(0x7c), edx);
                    // 00e4c858: MOVAPS XMM4,xmmword ptr [ESP + 0x80]
                    x[4] = load_vec4(e, f0.wrapping_add(0x74));
                    // 00e4c860: ADD EAX,ESI
                    {
                        let r = fl.add(eax, esi, 32);
                        eax = r;
                    }
                    pc = 0x00e4c862;
                }
                0x00e4c862 => {
                    // 00e4c862: MOVAPS XMM0,xmmword ptr [EAX]
                    x[0] = load_vec4(e, eax);
                    // 00e4c865: MOVAPS XMM5,XMM0
                    x[5] = x[0];
                    // 00e4c868: SHUFPS XMM5,XMM0,0x55
                    x[5] = shufps(x[5], x[0], 0x55);
                    // 00e4c86c: MOVAPS XMM6,XMM0
                    x[6] = x[0];
                    // 00e4c86f: SHUFPS XMM6,XMM0,0x0
                    x[6] = shufps(x[6], x[0], 0x0);
                    // 00e4c873: MULPS XMM6,XMM1
                    x[6] = mul4(x[6], x[1]);
                    // 00e4c876: MULPS XMM5,XMM2
                    x[5] = mul4(x[5], x[2]);
                    // 00e4c879: ADDPS XMM5,XMM6
                    x[5] = add4(x[5], x[6]);
                    // 00e4c87c: MOVAPS XMM6,XMM0
                    x[6] = x[0];
                    // 00e4c87f: SHUFPS XMM6,XMM0,0xaa
                    x[6] = shufps(x[6], x[0], 0xaa);
                    // 00e4c883: MULPS XMM6,XMM3
                    x[6] = mul4(x[6], x[3]);
                    // 00e4c886: ADDPS XMM6,XMM4
                    x[6] = add4(x[6], x[4]);
                    // 00e4c889: ADDPS XMM5,XMM6
                    x[5] = add4(x[5], x[6]);
                    // 00e4c88c: MOVAPS xmmword ptr [EAX + -0x40],XMM5
                    store_vec4(e, eax.wrapping_sub(0x40), x[5]);
                    // 00e4c890: DEC ECX
                    {
                        let r = fl.dec(ecx, 32);
                        ecx = r;
                    }
                    // 00e4c891: SUB EAX,0x10
                    {
                        let r = fl.sub(eax, 0x00000010u32, 32);
                        eax = r;
                    }
                    // 00e4c894: TEST ECX,ECX
                    fl.and(ecx, ecx, 32);
                    // 00e4c896: JGE 0x00e4c862
                    if fl.sf == fl.of {
                        pc = 0x00e4c862;
                        continue;
                    }
                    // 00e4c898: JMP 0x00e4c92f
                    pc = 0x00e4c92f;
                }
                0x00e4c89d => {
                    // 00e4c89d: MOVSS XMM0,dword ptr [0x01012068]
                    x[0] = [f32::from_bits(e.mem.u32(0x01012068)), 0.0, 0.0, 0.0];
                    // 00e4c8a5: MOVSS XMM2,XMM1
                    x[2][0] = x[1][0];
                    // 00e4c8a9: MOVSS XMM3,XMM1
                    x[3][0] = x[1][0];
                    // 00e4c8ad: LEA ECX,[ESP + 0xa0]
                    ecx = f0.wrapping_add(0x94);
                    // 00e4c8b4: MOV EAX,0x1
                    eax = 0x00000001u32;
                    // 00e4c8b9: PUSH ECX
                    stk.push(ecx);
                    // 00e4c8ba: PUSH EBX
                    stk.push(ebx);
                    // 00e4c8bb: UNPCKLPS XMM2,XMM1
                    x[2] = unpcklps(x[2], x[1]);
                    // 00e4c8be: MOVAPS XMM1,xmmword ptr [0x010c44d0]
                    x[1] = load_vec4(e, 0x010c44d0);
                    // 00e4c8c5: UNPCKLPS XMM0,XMM3
                    x[0] = unpcklps(x[0], x[3]);
                    // 00e4c8c8: UNPCKLPS XMM0,XMM2
                    x[0] = unpcklps(x[0], x[2]);
                    // 00e4c8cb: MOVAPS xmmword ptr [ESP + 0x118],XMM0
                    store_vec4(e, f0.wrapping_add(0x104), x[0]);
                    // 00e4c8d3: XORPS XMM0,XMM1
                    x[0] = xorps(x[0], x[1]);
                    // 00e4c8d6: MOV dword ptr [ESP + 0x20],EAX
                    e.mem.set_u32(f0.wrapping_add(0xc), eax);
                    // 00e4c8da: MOV dword ptr [ESP + 0x1c],EAX
                    e.mem.set_u32(f0.wrapping_add(0x8), eax);
                    // 00e4c8de: MOVAPS xmmword ptr [ESP + 0xa8],XMM0
                    store_vec4(e, f0.wrapping_add(0x94), x[0]);
                    // 00e4c8e6: CALL 0x004b4ab0
                    {
                        let mut w = vec![ecx];
                        w.extend(stk.iter().rev().take(2));
                        let r = e.call(0x004b_4ab0, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    stk.truncate(stk.len() - 2);
                    // 00e4c8eb: MOV ECX,dword ptr [ESP + 0x28]
                    ecx = e.mem.u32(f0.wrapping_add(0x1c));
                    // 00e4c8ef: MOV EDX,dword ptr [ECX]
                    edx = e.mem.u32(ecx);
                    // 00e4c8f1: MOV EDX,dword ptr [EDX + 0x4]
                    edx = e.mem.u32(edx.wrapping_add(0x4));
                    // 00e4c8f4: LEA EDI,[ESI + 0x20]
                    edi = esi.wrapping_add(0x20);
                    // 00e4c8f7: PUSH EDI
                    stk.push(edi);
                    // 00e4c8f8: LEA EAX,[ESP + 0x114]
                    eax = f0.wrapping_add(0x104);
                    // 00e4c8ff: PUSH EAX
                    stk.push(eax);
                    // 00e4c900: CALL EDX
                    {
                        let mut w = vec![ecx];
                        w.extend(stk.iter().rev().take(2));
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    stk.truncate(stk.len() - 2);
                    // 00e4c902: MOV ECX,dword ptr [ESP + 0x24]
                    ecx = e.mem.u32(f0.wrapping_add(0x18));
                    // 00e4c906: MOV EDX,dword ptr [ECX]
                    edx = e.mem.u32(ecx);
                    // 00e4c908: MOV EDX,dword ptr [EDX + 0x4]
                    edx = e.mem.u32(edx.wrapping_add(0x4));
                    // 00e4c90b: LEA EAX,[ESI + 0xe0]
                    eax = esi.wrapping_add(0xe0);
                    // 00e4c911: PUSH EAX
                    stk.push(eax);
                    // 00e4c912: LEA EAX,[ESP + 0xa4]
                    eax = f0.wrapping_add(0x94);
                    // 00e4c919: PUSH EAX
                    stk.push(eax);
                    // 00e4c91a: CALL EDX
                    {
                        let mut w = vec![ecx];
                        w.extend(stk.iter().rev().take(2));
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    stk.truncate(stk.len() - 2);
                    // 00e4c91c: LEA EAX,[ESI + 0xe0]
                    eax = esi.wrapping_add(0xe0);
                    // 00e4c922: PUSH EAX
                    stk.push(eax);
                    // 00e4c923: PUSH EBX
                    stk.push(ebx);
                    // 00e4c924: LEA ECX,[ESI + 0xa0]
                    ecx = esi.wrapping_add(0xa0);
                    // 00e4c92a: CALL 0x00c7f700
                    {
                        let mut w = vec![ecx];
                        w.extend(stk.iter().rev().take(2));
                        let r = e.call(0x00c7_f700, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    stk.truncate(stk.len() - 2);
                    pc = 0x00e4c92f;
                }
                0x00e4c92f => {
                    // 00e4c92f: MOVSS XMM0,dword ptr [0x01011d78]
                    x[0] = [f32::from_bits(e.mem.u32(0x01011d78)), 0.0, 0.0, 0.0];
                    // 00e4c937: MOVSS XMM1,dword ptr [0x01017718]
                    x[1] = [f32::from_bits(e.mem.u32(0x01017718)), 0.0, 0.0, 0.0];
                    // 00e4c93f: MOVSS XMM5,dword ptr [0x010718c0]
                    x[5] = [f32::from_bits(e.mem.u32(0x010718c0)), 0.0, 0.0, 0.0];
                    // 00e4c947: MOVAPS xmmword ptr [ESP + 0x190],XMM0
                    store_vec4(e, f0.wrapping_add(0x184), x[0]);
                    // 00e4c94f: XORPS XMM0,XMM0
                    x[0] = xorps(x[0], x[0]);
                    // 00e4c952: MOVSS XMM0,XMM1
                    x[0][0] = x[1][0];
                    // 00e4c956: MOVSS XMM1,dword ptr [0x01016248]
                    x[1] = [f32::from_bits(e.mem.u32(0x01016248)), 0.0, 0.0, 0.0];
                    // 00e4c95e: MOVAPS xmmword ptr [ESP + 0x1a0],XMM0
                    store_vec4(e, f0.wrapping_add(0x194), x[0]);
                    // 00e4c966: XORPS XMM0,XMM0
                    x[0] = xorps(x[0], x[0]);
                    // 00e4c969: MOVSS XMM0,XMM1
                    x[0][0] = x[1][0];
                    // 00e4c96d: MOV dword ptr [ESP + 0x20],0x0
                    e.mem.set_u32(f0.wrapping_add(0x14), 0x00000000u32);
                    // 00e4c975: MOVAPS xmmword ptr [ESP + 0x1c0],XMM0
                    store_vec4(e, f0.wrapping_add(0x1b4), x[0]);
                    // 00e4c97d: LEA EBX,[ESI + 0xa0]
                    ebx = esi.wrapping_add(0xa0);
                    // 00e4c983: JMP 0x00e4c990
                    pc = 0x00e4c990;
                }
                0x00e4c990 => {
                    // 00e4c990: MOV ECX,dword ptr [ESP + 0x14]
                    ecx = e.mem.u32(f0.wrapping_add(0x8));
                    // 00e4c994: MOV EAX,dword ptr [ESP + 0x18]
                    eax = e.mem.u32(f0.wrapping_add(0xc));
                    // 00e4c998: MOVSS XMM0,dword ptr [0x0104f498]
                    x[0] = [f32::from_bits(e.mem.u32(0x0104f498)), 0.0, 0.0, 0.0];
                    // 00e4c9a0: LEA EDX,[ECX + EAX*0x1]
                    edx = ecx.wrapping_add(eax);
                    // 00e4c9a3: ADD EAX,EAX
                    {
                        let r = fl.add(eax, eax, 32);
                        eax = r;
                    }
                    // 00e4c9a5: ADD EAX,EAX
                    {
                        let r = fl.add(eax, eax, 32);
                        eax = r;
                    }
                    // 00e4c9a7: ADD EAX,EAX
                    {
                        let r = fl.add(eax, eax, 32);
                        eax = r;
                    }
                    // 00e4c9a9: OR EAX,ECX
                    {
                        let r = fl.or(eax, ecx, 32);
                        eax = r;
                    }
                    // 00e4c9ab: SUB EAX,0x9
                    {
                        let r = fl.sub(eax, 0x00000009u32, 32);
                        eax = r;
                    }
                    // 00e4c9ae: MULSS XMM5,XMM0
                    x[5][0] = x[5][0] * x[0][0];
                    // 00e4c9b2: MOVAPS xmmword ptr [ESP + 0x170],XMM5
                    store_vec4(e, f0.wrapping_add(0x164), x[5]);
                    // 00e4c9ba: MOV dword ptr [ESP + 0xb4],EDX
                    e.mem.set_u32(f0.wrapping_add(0xa8), edx);
                    // 00e4c9c1: MOV dword ptr [ESI + 0x14],0x0
                    e.mem.set_u32(esi.wrapping_add(0x14), 0x00000000u32);
                    // 00e4c9c8: CMP EAX,0x18
                    fl.sub(eax, 0x00000018u32, 32);
                    // 00e4c9cb: JA 0x00e4cd40
                    if !fl.cf && !fl.zf {
                        pc = 0x00e4cd40;
                        continue;
                    }
                    pc = 0x00e4c9d1;
                }
                0x00e4c9d1 => {
                    // 00e4c9d1: MOVZX EAX,byte ptr [EAX + 0xe4da34]
                    eax = match eax {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        3 => 3,
                        8 => 4,
                        9 => 5,
                        10 => 6,
                        16 => 7,
                        17 => 8,
                        24 => 9,
                        _ => 10,
                    };
                    // 00e4c9d8: JMP dword ptr [EAX*0x4 + 0xe4da08]
                    pc = match eax {
                        0 => 0x00e4d699,
                        1 => 0x00e4d45b,
                        2 => 0x00e4d248,
                        3 => 0x00e4d20a,
                        4 => 0x00e4d6be,
                        5 => 0x00e4ca30,
                        6 => 0x00e4c9df,
                        7 => 0x00e4d5ab,
                        8 => 0x00e4c9fc,
                        9 => 0x00e4d57d,
                        10 => 0x00e4cd40,
                        _ => unreachable!(),
                    };
                }
                0x00e4c9df => {
                    // 00e4c9df: MOV EAX,dword ptr [ESI + 0x10]
                    eax = e.mem.u32(esi.wrapping_add(0x10));
                    // 00e4c9e2: CMP EAX,0x2
                    fl.sub(eax, 0x00000002u32, 32);
                    // 00e4c9e5: SETZ CL
                    ecx = (ecx & 0xffff_ff00) | (((fl.zf) as u32) & 0xff);
                    // 00e4c9e8: PUSH 0x1
                    stk.push(0x00000001u32);
                    // 00e4c9ea: MOVZX EDX,CL
                    edx = (ecx & 0xff);
                    // 00e4c9ed: PUSH EDX
                    stk.push(edx);
                    // 00e4c9ee: LEA EAX,[ESP + 0x1c]
                    eax = f0.wrapping_add(0x8);
                    // 00e4c9f2: PUSH EAX
                    stk.push(eax);
                    // 00e4c9f3: LEA ECX,[ESP + 0x24]
                    ecx = f0.wrapping_add(0xc);
                    // 00e4c9f7: PUSH ECX
                    stk.push(ecx);
                    // 00e4c9f8: PUSH EBX
                    stk.push(ebx);
                    // 00e4c9f9: PUSH EDI
                    stk.push(edi);
                    // 00e4c9fa: JMP 0x00e4ca17
                    pc = 0x00e4ca17;
                }
                0x00e4c9fc => {
                    // 00e4c9fc: MOV EAX,dword ptr [ESI + 0x10]
                    eax = e.mem.u32(esi.wrapping_add(0x10));
                    // 00e4c9ff: CMP EAX,0x2
                    fl.sub(eax, 0x00000002u32, 32);
                    // 00e4ca02: SETZ DL
                    edx = (edx & 0xffff_ff00) | (((fl.zf) as u32) & 0xff);
                    // 00e4ca05: PUSH -0x1
                    stk.push(0xffffffffu32);
                    // 00e4ca07: MOVZX EAX,DL
                    eax = (edx & 0xff);
                    // 00e4ca0a: PUSH EAX
                    stk.push(eax);
                    // 00e4ca0b: LEA ECX,[ESP + 0x20]
                    ecx = f0.wrapping_add(0xc);
                    // 00e4ca0f: PUSH ECX
                    stk.push(ecx);
                    // 00e4ca10: LEA EDX,[ESP + 0x20]
                    edx = f0.wrapping_add(0x8);
                    // 00e4ca14: PUSH EDX
                    stk.push(edx);
                    // 00e4ca15: PUSH EDI
                    stk.push(edi);
                    // 00e4ca16: PUSH EBX
                    stk.push(ebx);
                    pc = 0x00e4ca17;
                }
                0x00e4ca17 => {
                    // 00e4ca17: MOV ECX,ESI
                    ecx = esi;
                    // 00e4ca19: CALL 0x00e4bbe0
                    {
                        let mut w = vec![ecx];
                        w.extend(stk.iter().rev().take(6));
                        let r = hk_gsk_process_edge_triangle(
                            e, w[0], w[1], w[2], w[3], w[4], w[5] as u8, w[6],
                        );
                        eax = r;
                    }
                    stk.truncate(stk.len() - 6);
                    // 00e4ca1e: SUB EAX,0x1
                    {
                        let r = fl.sub(eax, 0x00000001u32, 32);
                        eax = r;
                    }
                    // 00e4ca21: JZ 0x00e4cd40
                    if fl.zf {
                        pc = 0x00e4cd40;
                        continue;
                    }
                    // 00e4ca27: SUB EAX,0x1
                    {
                        let r = fl.sub(eax, 0x00000001u32, 32);
                        eax = r;
                    }
                    // 00e4ca2a: JNZ 0x00e4d7d6
                    if !fl.zf {
                        pc = 0x00e4d7d6;
                        continue;
                    }
                    pc = 0x00e4ca30;
                }
                0x00e4ca30 => {
                    // 00e4ca30: MOV EAX,dword ptr [EDI]
                    eax = e.mem.u32(edi);
                    // 00e4ca32: MOV ECX,dword ptr [EDI + 0x4]
                    ecx = e.mem.u32(edi.wrapping_add(0x4));
                    // 00e4ca35: MOV EDX,dword ptr [EDI + 0x8]
                    edx = e.mem.u32(edi.wrapping_add(0x8));
                    // 00e4ca38: MOV dword ptr [ESP + 0xc0],EAX
                    e.mem.set_u32(f0.wrapping_add(0xb4), eax);
                    // 00e4ca3f: MOV EAX,dword ptr [EDI + 0xc]
                    eax = e.mem.u32(edi.wrapping_add(0xc));
                    // 00e4ca42: MOV dword ptr [ESP + 0xc4],ECX
                    e.mem.set_u32(f0.wrapping_add(0xb8), ecx);
                    // 00e4ca49: MOV ECX,dword ptr [ESI + 0x30]
                    ecx = e.mem.u32(esi.wrapping_add(0x30));
                    // 00e4ca4c: MOV dword ptr [ESP + 0xc8],EDX
                    e.mem.set_u32(f0.wrapping_add(0xbc), edx);
                    // 00e4ca53: MOV EDX,dword ptr [ESI + 0x34]
                    edx = e.mem.u32(esi.wrapping_add(0x34));
                    // 00e4ca56: MOV dword ptr [ESP + 0xcc],EAX
                    e.mem.set_u32(f0.wrapping_add(0xc0), eax);
                    // 00e4ca5d: MOV EAX,dword ptr [ESI + 0x38]
                    eax = e.mem.u32(esi.wrapping_add(0x38));
                    // 00e4ca60: MOV dword ptr [ESP + 0x80],ECX
                    e.mem.set_u32(f0.wrapping_add(0x74), ecx);
                    // 00e4ca67: MOV ECX,dword ptr [ESI + 0x3c]
                    ecx = e.mem.u32(esi.wrapping_add(0x3c));
                    // 00e4ca6a: MOV dword ptr [ESP + 0x84],EDX
                    e.mem.set_u32(f0.wrapping_add(0x78), edx);
                    // 00e4ca71: MOV EDX,dword ptr [EBX]
                    edx = e.mem.u32(ebx);
                    // 00e4ca73: MOV dword ptr [ESP + 0x88],EAX
                    e.mem.set_u32(f0.wrapping_add(0x7c), eax);
                    // 00e4ca7a: MOV EAX,dword ptr [EBX + 0x4]
                    eax = e.mem.u32(ebx.wrapping_add(0x4));
                    // 00e4ca7d: MOV dword ptr [ESP + 0x8c],ECX
                    e.mem.set_u32(f0.wrapping_add(0x80), ecx);
                    // 00e4ca84: MOV ECX,dword ptr [EBX + 0x8]
                    ecx = e.mem.u32(ebx.wrapping_add(0x8));
                    // 00e4ca87: MOVAPS XMM0,xmmword ptr [ESP + 0x80]
                    x[0] = load_vec4(e, f0.wrapping_add(0x74));
                    // 00e4ca8f: SUBPS XMM0,xmmword ptr [ESP + 0xc0]
                    x[0] = sub4(x[0], load_vec4(e, f0.wrapping_add(0xb4)));
                    // 00e4ca97: MOV dword ptr [ESP + 0x90],EDX
                    e.mem.set_u32(f0.wrapping_add(0x84), edx);
                    // 00e4ca9e: MOV EDX,dword ptr [EBX + 0xc]
                    edx = e.mem.u32(ebx.wrapping_add(0xc));
                    // 00e4caa1: MOV dword ptr [ESP + 0x94],EAX
                    e.mem.set_u32(f0.wrapping_add(0x88), eax);
                    // 00e4caa8: MOV EAX,dword ptr [ESI + 0xb0]
                    eax = e.mem.u32(esi.wrapping_add(0xb0));
                    // 00e4caae: MOV dword ptr [ESP + 0x98],ECX
                    e.mem.set_u32(f0.wrapping_add(0x8c), ecx);
                    // 00e4cab5: MOV ECX,dword ptr [ESI + 0xb4]
                    ecx = e.mem.u32(esi.wrapping_add(0xb4));
                    // 00e4cabb: MOV dword ptr [ESP + 0x9c],EDX
                    e.mem.set_u32(f0.wrapping_add(0x90), edx);
                    // 00e4cac2: MOV EDX,dword ptr [ESI + 0xb8]
                    edx = e.mem.u32(esi.wrapping_add(0xb8));
                    // 00e4cac8: MOVAPS XMM4,xmmword ptr [ESP + 0x90]
                    x[4] = load_vec4(e, f0.wrapping_add(0x84));
                    // 00e4cad0: MOV dword ptr [ESP + 0x60],EAX
                    e.mem.set_u32(f0.wrapping_add(0x54), eax);
                    // 00e4cad4: MOV EAX,dword ptr [ESI + 0xbc]
                    eax = e.mem.u32(esi.wrapping_add(0xbc));
                    // 00e4cada: MOVAPS XMM3,XMM0
                    x[3] = x[0];
                    // 00e4cadd: SHUFPS XMM3,XMM0,0xd2
                    x[3] = shufps(x[3], x[0], 0xd2);
                    // 00e4cae1: MOV dword ptr [ESP + 0x64],ECX
                    e.mem.set_u32(f0.wrapping_add(0x58), ecx);
                    // 00e4cae5: MOV dword ptr [ESP + 0x68],EDX
                    e.mem.set_u32(f0.wrapping_add(0x5c), edx);
                    // 00e4cae9: MOV dword ptr [ESP + 0x6c],EAX
                    e.mem.set_u32(f0.wrapping_add(0x60), eax);
                    // 00e4caed: MOVAPS XMM1,xmmword ptr [ESP + 0x60]
                    x[1] = load_vec4(e, f0.wrapping_add(0x54));
                    // 00e4caf2: SUBPS XMM1,XMM4
                    x[1] = sub4(x[1], x[4]);
                    // 00e4caf5: MOVAPS XMM6,XMM1
                    x[6] = x[1];
                    // 00e4caf8: SHUFPS XMM6,XMM1,0xc9
                    x[6] = shufps(x[6], x[1], 0xc9);
                    // 00e4cafc: MOVAPS xmmword ptr [ESP + 0x180],XMM1
                    store_vec4(e, f0.wrapping_add(0x174), x[1]);
                    // 00e4cb04: SHUFPS XMM1,XMM1,0xd2
                    x[1] = shufps(x[1], x[1], 0xd2);
                    // 00e4cb08: MOVAPS xmmword ptr [ESP + 0x1d0],XMM0
                    store_vec4(e, f0.wrapping_add(0x1c4), x[0]);
                    // 00e4cb10: SHUFPS XMM0,XMM0,0xc9
                    x[0] = shufps(x[0], x[0], 0xc9);
                    // 00e4cb14: MOVAPS XMM2,XMM3
                    x[2] = x[3];
                    // 00e4cb17: MULPS XMM2,XMM6
                    x[2] = mul4(x[2], x[6]);
                    // 00e4cb1a: MOVAPS XMM5,XMM0
                    x[5] = x[0];
                    // 00e4cb1d: MULPS XMM5,XMM1
                    x[5] = mul4(x[5], x[1]);
                    // 00e4cb20: SUBPS XMM5,XMM2
                    x[5] = sub4(x[5], x[2]);
                    // 00e4cb23: MOVAPS xmmword ptr [ESI + 0x120],XMM5
                    store_vec4(e, esi.wrapping_add(0x120), x[5]);
                    // 00e4cb2a: MOVAPS XMM2,xmmword ptr [ESI + 0x120]
                    x[2] = load_vec4(e, esi.wrapping_add(0x120));
                    // 00e4cb31: MOVAPS XMM7,XMM2
                    x[7] = x[2];
                    // 00e4cb34: SHUFPS XMM7,XMM2,0xd2
                    x[7] = shufps(x[7], x[2], 0xd2);
                    // 00e4cb38: MULPS XMM7,XMM0
                    x[7] = mul4(x[7], x[0]);
                    // 00e4cb3b: MOVAPS XMM0,xmmword ptr [ESI + 0x120]
                    x[0] = load_vec4(e, esi.wrapping_add(0x120));
                    // 00e4cb42: SHUFPS XMM2,XMM2,0xc9
                    x[2] = shufps(x[2], x[2], 0xc9);
                    // 00e4cb46: MULPS XMM2,XMM3
                    x[2] = mul4(x[2], x[3]);
                    // 00e4cb49: MOVAPS XMM3,XMM0
                    x[3] = x[0];
                    // 00e4cb4c: SHUFPS XMM3,XMM0,0xc9
                    x[3] = shufps(x[3], x[0], 0xc9);
                    // 00e4cb50: MULPS XMM3,XMM1
                    x[3] = mul4(x[3], x[1]);
                    // 00e4cb53: MOVAPS XMM1,XMM0
                    x[1] = x[0];
                    // 00e4cb56: SHUFPS XMM1,XMM0,0xd2
                    x[1] = shufps(x[1], x[0], 0xd2);
                    // 00e4cb5a: MULPS XMM1,XMM6
                    x[1] = mul4(x[1], x[6]);
                    // 00e4cb5d: SUBPS XMM3,XMM1
                    x[3] = sub4(x[3], x[1]);
                    // 00e4cb60: MOVAPS XMM1,xmmword ptr [ESP + 0xc0]
                    x[1] = load_vec4(e, f0.wrapping_add(0xb4));
                    // 00e4cb68: SUBPS XMM1,XMM4
                    x[1] = sub4(x[1], x[4]);
                    // 00e4cb6b: SUBPS XMM2,XMM7
                    x[2] = sub4(x[2], x[7]);
                    // 00e4cb6e: MOVAPS XMM0,XMM3
                    x[0] = x[3];
                    // 00e4cb71: MULPS XMM0,XMM1
                    x[0] = mul4(x[0], x[1]);
                    // 00e4cb74: MOVAPS XMM6,XMM0
                    x[6] = x[0];
                    // 00e4cb77: SHUFPS XMM6,XMM0,0x55
                    x[6] = shufps(x[6], x[0], 0x55);
                    // 00e4cb7b: ADDSS XMM6,XMM0
                    x[6][0] = x[6][0] + x[0][0];
                    // 00e4cb7f: MOVAPS XMM7,XMM0
                    x[7] = x[0];
                    // 00e4cb82: SHUFPS XMM7,XMM0,0xaa
                    x[7] = shufps(x[7], x[0], 0xaa);
                    // 00e4cb86: ADDSS XMM7,XMM6
                    x[7][0] = x[7][0] + x[6][0];
                    // 00e4cb8a: MOVAPS XMM6,xmmword ptr [ESP + 0x80]
                    x[6] = load_vec4(e, f0.wrapping_add(0x74));
                    // 00e4cb92: MOVSS dword ptr [ESP + 0xf0],XMM7
                    e.mem.set_f32(f0.wrapping_add(0xe4), x[7][0]);
                    // 00e4cb9b: FLD float ptr [ESP + 0xf0]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0xe4))));
                    // 00e4cba2: FSTP float ptr [ESP + 0x120]
                    e.mem.set_f32(f0.wrapping_add(0x114), x87.fstp_f32());
                    // 00e4cba9: MOVAPS xmmword ptr [ESP + 0x1b0],XMM1
                    store_vec4(e, f0.wrapping_add(0x1a4), x[1]);
                    // 00e4cbb1: SUBPS XMM4,XMM6
                    x[4] = sub4(x[4], x[6]);
                    // 00e4cbb4: MULPS XMM4,XMM3
                    x[4] = mul4(x[4], x[3]);
                    // 00e4cbb7: MOVAPS XMM0,XMM4
                    x[0] = x[4];
                    // 00e4cbba: SHUFPS XMM0,XMM4,0x55
                    x[0] = shufps(x[0], x[4], 0x55);
                    // 00e4cbbe: ADDSS XMM0,XMM4
                    x[0][0] = x[0][0] + x[4][0];
                    // 00e4cbc2: MOVAPS XMM3,XMM4
                    x[3] = x[4];
                    // 00e4cbc5: SHUFPS XMM3,XMM4,0xaa
                    x[3] = shufps(x[3], x[4], 0xaa);
                    // 00e4cbc9: ADDSS XMM3,XMM0
                    x[3][0] = x[3][0] + x[0][0];
                    // 00e4cbcd: MOVAPS XMM0,XMM2
                    x[0] = x[2];
                    // 00e4cbd0: MULPS XMM0,XMM1
                    x[0] = mul4(x[0], x[1]);
                    // 00e4cbd3: MOVSS dword ptr [ESP + 0xb8],XMM3
                    e.mem.set_f32(f0.wrapping_add(0xac), x[3][0]);
                    // 00e4cbdc: FLD float ptr [ESP + 0xb8]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0xac))));
                    // 00e4cbe3: MOVAPS XMM3,XMM0
                    x[3] = x[0];
                    // 00e4cbe6: FSTP float ptr [ESP + 0x124]
                    e.mem.set_f32(f0.wrapping_add(0x118), x87.fstp_f32());
                    // 00e4cbed: SHUFPS XMM3,XMM0,0x55
                    x[3] = shufps(x[3], x[0], 0x55);
                    // 00e4cbf1: ADDSS XMM3,XMM0
                    x[3][0] = x[3][0] + x[0][0];
                    // 00e4cbf5: MOVAPS XMM4,XMM0
                    x[4] = x[0];
                    // 00e4cbf8: SHUFPS XMM4,XMM0,0xaa
                    x[4] = shufps(x[4], x[0], 0xaa);
                    // 00e4cbfc: MOVAPS XMM0,xmmword ptr [ESP + 0x60]
                    x[0] = load_vec4(e, f0.wrapping_add(0x54));
                    // 00e4cc01: ADDSS XMM4,XMM3
                    x[4][0] = x[4][0] + x[3][0];
                    // 00e4cc05: MOVAPS XMM3,xmmword ptr [ESP + 0xc0]
                    x[3] = load_vec4(e, f0.wrapping_add(0xb4));
                    // 00e4cc0d: SUBPS XMM0,XMM3
                    x[0] = sub4(x[0], x[3]);
                    // 00e4cc10: MULPS XMM0,XMM2
                    x[0] = mul4(x[0], x[2]);
                    // 00e4cc13: MOVSS dword ptr [ESP + 0xe4],XMM4
                    e.mem.set_f32(f0.wrapping_add(0xd8), x[4][0]);
                    // 00e4cc1c: FLD float ptr [ESP + 0xe4]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0xd8))));
                    // 00e4cc23: MOVAPS XMM2,XMM0
                    x[2] = x[0];
                    // 00e4cc26: FSTP float ptr [ESP + 0x128]
                    e.mem.set_f32(f0.wrapping_add(0x11c), x87.fstp_f32());
                    // 00e4cc2d: SHUFPS XMM2,XMM0,0x55
                    x[2] = shufps(x[2], x[0], 0x55);
                    // 00e4cc31: ADDSS XMM2,XMM0
                    x[2][0] = x[2][0] + x[0][0];
                    // 00e4cc35: MOVAPS XMM4,XMM0
                    x[4] = x[0];
                    // 00e4cc38: SHUFPS XMM4,XMM0,0xaa
                    x[4] = shufps(x[4], x[0], 0xaa);
                    // 00e4cc3c: ADDSS XMM4,XMM2
                    x[4][0] = x[4][0] + x[2][0];
                    // 00e4cc40: MOVSS dword ptr [ESP + 0xe0],XMM4
                    e.mem.set_f32(f0.wrapping_add(0xd4), x[4][0]);
                    // 00e4cc49: FLD float ptr [ESP + 0xe0]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0xd4))));
                    // 00e4cc50: FSTP float ptr [ESP + 0x12c]
                    e.mem.set_f32(f0.wrapping_add(0x120), x87.fstp_f32());
                    // 00e4cc57: MOVAPS XMM2,xmmword ptr [ESP + 0x120]
                    x[2] = load_vec4(e, f0.wrapping_add(0x114));
                    // 00e4cc5f: XORPS XMM0,XMM0
                    x[0] = xorps(x[0], x[0]);
                    // 00e4cc62: CMPLTPS XMM0,XMM2
                    x[0] = cmpltps(x[0], x[2]);
                    // 00e4cc66: MOVMSKPS EAX,XMM0
                    eax = movmskps(x[0]);
                    // 00e4cc69: CMP EAX,0xf
                    fl.sub(eax, 0x0000000fu32, 32);
                    // 00e4cc6c: JZ 0x00e4d135
                    if fl.zf {
                        pc = 0x00e4d135;
                        continue;
                    }
                    // 00e4cc72: ADD EAX,-0x7
                    {
                        let r = fl.add(eax, 0xfffffff9u32, 32);
                        eax = r;
                    }
                    // 00e4cc75: CMP EAX,0x7
                    fl.sub(eax, 0x00000007u32, 32);
                    // 00e4cc78: JA 0x00e4cc81
                    if !fl.cf && !fl.zf {
                        pc = 0x00e4cc81;
                        continue;
                    }
                    // 00e4cc7a: JMP dword ptr [EAX*0x4 + 0xe4da50]
                    pc = match eax {
                        0 => 0x00e4d1c2,
                        1 => 0x00e4cc81,
                        2 => 0x00e4cc81,
                        3 => 0x00e4cc81,
                        4 => 0x00e4d1da,
                        5 => 0x00e4cc81,
                        6 => 0x00e4d1ae,
                        7 => 0x00e4d1b5,
                        _ => unreachable!(),
                    };
                }
                0x00e4cc81 => {
                    // 00e4cc81: LEA ECX,[ESI + 0x130]
                    ecx = esi.wrapping_add(0x130);
                    // 00e4cc87: PUSH ECX
                    stk.push(ecx);
                    // 00e4cc88: LEA EDX,[ESP + 0x184]
                    edx = f0.wrapping_add(0x174);
                    // 00e4cc8f: PUSH EDX
                    stk.push(edx);
                    // 00e4cc90: LEA EAX,[ESP + 0x98]
                    eax = f0.wrapping_add(0x84);
                    // 00e4cc97: PUSH EAX
                    stk.push(eax);
                    // 00e4cc98: LEA ECX,[ESP + 0x1dc]
                    ecx = f0.wrapping_add(0x1c4);
                    // 00e4cc9f: PUSH ECX
                    stk.push(ecx);
                    // 00e4cca0: LEA EDX,[ESP + 0xd0]
                    edx = f0.wrapping_add(0xb4);
                    // 00e4cca7: PUSH EDX
                    stk.push(edx);
                    // 00e4cca8: CALL 0x00d22e40
                    {
                        let w: Vec<u32> = stk.iter().rev().take(5).copied().collect();
                        let r = e.call(CLOSEST_LINE_SEG_LINE_SEG, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    // 00e4ccad: ADD ESP,0x14
                    stk.truncate(stk.len() - 5);
                    // 00e4ccb0: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e4ccb2: JZ 0x00e4d1e7
                    if fl.zf {
                        pc = 0x00e4d1e7;
                        continue;
                    }
                    // 00e4ccb8: TEST AL,0x1
                    fl.and((eax & 0xff), 0x00000001u32, 8);
                    // 00e4ccba: JZ 0x00e4ccda
                    if fl.zf {
                        pc = 0x00e4ccda;
                        continue;
                    }
                    // 00e4ccbc: MOV ECX,dword ptr [ESI + 0x30]
                    ecx = e.mem.u32(esi.wrapping_add(0x30));
                    // 00e4ccbf: MOV EDX,dword ptr [ESI + 0x34]
                    edx = e.mem.u32(esi.wrapping_add(0x34));
                    // 00e4ccc2: MOV dword ptr [EDI],ECX
                    e.mem.set_u32(edi, ecx);
                    // 00e4ccc4: MOV ECX,dword ptr [ESI + 0x38]
                    ecx = e.mem.u32(esi.wrapping_add(0x38));
                    // 00e4ccc7: MOV dword ptr [EDI + 0x4],EDX
                    e.mem.set_u32(edi.wrapping_add(0x4), edx);
                    // 00e4ccca: MOV EDX,dword ptr [ESI + 0x3c]
                    edx = e.mem.u32(esi.wrapping_add(0x3c));
                    // 00e4cccd: MOV dword ptr [EDI + 0x8],ECX
                    e.mem.set_u32(edi.wrapping_add(0x8), ecx);
                    // 00e4ccd0: MOV dword ptr [EDI + 0xc],EDX
                    e.mem.set_u32(edi.wrapping_add(0xc), edx);
                    // 00e4ccd3: MOV ECX,0x1
                    ecx = 0x00000001u32;
                    // 00e4ccd8: JMP 0x00e4cce3
                    pc = 0x00e4cce3;
                }
                0x00e4ccda => {
                    // 00e4ccda: MOV ECX,0x1
                    ecx = 0x00000001u32;
                    // 00e4ccdf: TEST AL,0x2
                    fl.and((eax & 0xff), 0x00000002u32, 8);
                    // 00e4cce1: JZ 0x00e4cce7
                    if fl.zf {
                        pc = 0x00e4cce7;
                        continue;
                    }
                    pc = 0x00e4cce3;
                }
                0x00e4cce3 => {
                    // 00e4cce3: MOV dword ptr [ESP + 0x18],ECX
                    e.mem.set_u32(f0.wrapping_add(0xc), ecx);
                    pc = 0x00e4cce7;
                }
                0x00e4cce7 => {
                    // 00e4cce7: TEST AL,0x4
                    fl.and((eax & 0xff), 0x00000004u32, 8);
                    // 00e4cce9: JZ 0x00e4cd1e
                    if fl.zf {
                        pc = 0x00e4cd1e;
                        continue;
                    }
                    // 00e4cceb: MOV EAX,dword ptr [ESI + 0xb0]
                    eax = e.mem.u32(esi.wrapping_add(0xb0));
                    // 00e4ccf1: MOV EDX,dword ptr [ESI + 0xb4]
                    edx = e.mem.u32(esi.wrapping_add(0xb4));
                    // 00e4ccf7: MOVAPS XMM0,xmmword ptr [ESI + 0xf0]
                    x[0] = load_vec4(e, esi.wrapping_add(0xf0));
                    // 00e4ccfe: MOV dword ptr [EBX],EAX
                    e.mem.set_u32(ebx, eax);
                    // 00e4cd00: MOV EAX,dword ptr [ESI + 0xb8]
                    eax = e.mem.u32(esi.wrapping_add(0xb8));
                    // 00e4cd06: MOV dword ptr [EBX + 0x4],EDX
                    e.mem.set_u32(ebx.wrapping_add(0x4), edx);
                    // 00e4cd09: MOV EDX,dword ptr [ESI + 0xbc]
                    edx = e.mem.u32(esi.wrapping_add(0xbc));
                    // 00e4cd0f: MOV dword ptr [EBX + 0x8],EAX
                    e.mem.set_u32(ebx.wrapping_add(0x8), eax);
                    // 00e4cd12: MOV dword ptr [EBX + 0xc],EDX
                    e.mem.set_u32(ebx.wrapping_add(0xc), edx);
                    // 00e4cd15: MOVAPS xmmword ptr [ESI + 0xe0],XMM0
                    store_vec4(e, esi.wrapping_add(0xe0), x[0]);
                    // 00e4cd1c: JMP 0x00e4cd22
                    pc = 0x00e4cd22;
                }
                0x00e4cd1e => {
                    // 00e4cd1e: TEST AL,0x8
                    fl.and((eax & 0xff), 0x00000008u32, 8);
                    // 00e4cd20: JZ 0x00e4cd26
                    if fl.zf {
                        pc = 0x00e4cd26;
                        continue;
                    }
                    pc = 0x00e4cd22;
                }
                0x00e4cd22 => {
                    // 00e4cd22: MOV dword ptr [ESP + 0x14],ECX
                    e.mem.set_u32(f0.wrapping_add(0x8), ecx);
                    pc = 0x00e4cd26;
                }
                0x00e4cd26 => {
                    // 00e4cd26: MOV EAX,dword ptr [ESP + 0x18]
                    eax = e.mem.u32(f0.wrapping_add(0xc));
                    // 00e4cd2a: ADD EAX,EAX
                    {
                        let r = fl.add(eax, eax, 32);
                        eax = r;
                    }
                    // 00e4cd2c: ADD EAX,EAX
                    {
                        let r = fl.add(eax, eax, 32);
                        eax = r;
                    }
                    // 00e4cd2e: ADD EAX,EAX
                    {
                        let r = fl.add(eax, eax, 32);
                        eax = r;
                    }
                    // 00e4cd30: OR EAX,dword ptr [ESP + 0x14]
                    {
                        let r = fl.or(eax, e.mem.u32(f0.wrapping_add(0x8)), 32);
                        eax = r;
                    }
                    // 00e4cd34: SUB EAX,0x9
                    {
                        let r = fl.sub(eax, 0x00000009u32, 32);
                        eax = r;
                    }
                    // 00e4cd37: CMP EAX,0x18
                    fl.sub(eax, 0x00000018u32, 32);
                    // 00e4cd3a: JBE 0x00e4c9d1
                    if fl.cf || fl.zf {
                        pc = 0x00e4c9d1;
                        continue;
                    }
                    pc = 0x00e4cd40;
                }
                0x00e4cd40 => {
                    // 00e4cd40: MOV EAX,0x1
                    eax = 0x00000001u32;
                    pc = 0x00e4cd45;
                }
                0x00e4cd45 => {
                    // 00e4cd45: MOV EDX,dword ptr [ESP + 0x14]
                    edx = e.mem.u32(f0.wrapping_add(0x8));
                    // 00e4cd49: SUB EDX,dword ptr [ESP + 0xb4]
                    {
                        let r = fl.sub(edx, e.mem.u32(f0.wrapping_add(0xa8)), 32);
                        edx = r;
                    }
                    // 00e4cd50: MOV ECX,dword ptr [ESP + 0x18]
                    ecx = e.mem.u32(f0.wrapping_add(0xc));
                    // 00e4cd54: MOVAPS XMM5,xmmword ptr [ESP + 0x1a0]
                    x[5] = load_vec4(e, f0.wrapping_add(0x194));
                    // 00e4cd5c: ADD EDX,ECX
                    {
                        let r = fl.add(edx, ecx, 32);
                        edx = r;
                    }
                    // 00e4cd5e: OR dword ptr [ESI + 0x1c],EDX
                    {
                        let r = fl.or(e.mem.u32(esi.wrapping_add(0x1c)), edx, 32);
                        e.mem.set_u32(esi.wrapping_add(0x1c), r);
                    }
                    // 00e4cd61: MOVAPS XMM3,xmmword ptr [ESI + 0x120]
                    x[3] = load_vec4(e, esi.wrapping_add(0x120));
                    // 00e4cd68: MOVAPS XMM1,XMM3
                    x[1] = x[3];
                    // 00e4cd6b: MULPS XMM1,XMM3
                    x[1] = mul4(x[1], x[3]);
                    // 00e4cd6e: MOVAPS XMM2,XMM1
                    x[2] = x[1];
                    // 00e4cd71: SHUFPS XMM2,XMM1,0x55
                    x[2] = shufps(x[2], x[1], 0x55);
                    // 00e4cd75: ADDSS XMM2,XMM1
                    x[2][0] = x[2][0] + x[1][0];
                    // 00e4cd79: MOVAPS XMM0,XMM1
                    x[0] = x[1];
                    // 00e4cd7c: SHUFPS XMM0,XMM1,0xaa
                    x[0] = shufps(x[0], x[1], 0xaa);
                    // 00e4cd80: ADDSS XMM0,XMM2
                    x[0][0] = x[0][0] + x[2][0];
                    // 00e4cd84: MOVAPS XMM2,XMM0
                    x[2] = x[0];
                    // 00e4cd87: RSQRTSS XMM2,XMM2
                    x[2][0] = rsqrt_estimate(x[2][0]);
                    // 00e4cd8b: MOVAPS XMM4,XMM2
                    x[4] = x[2];
                    // 00e4cd8e: MOVAPS XMM2,XMM0
                    x[2] = x[0];
                    // 00e4cd91: CMPEQSS XMM2,dword ptr [ESP + 0x190]
                    x[2] = cmpeqss(x[2], f32::from_bits(e.mem.u32(f0.wrapping_add(0x184))));
                    // 00e4cd9b: MULSS XMM0,XMM4
                    x[0][0] = x[0][0] * x[4][0];
                    // 00e4cd9f: MULSS XMM0,XMM4
                    x[0][0] = x[0][0] * x[4][0];
                    // 00e4cda3: SUBSS XMM5,XMM0
                    x[5][0] = x[5][0] - x[0][0];
                    // 00e4cda7: MOVAPS XMM0,xmmword ptr [ESP + 0x1c0]
                    x[0] = load_vec4(e, f0.wrapping_add(0x1b4));
                    // 00e4cdaf: MULSS XMM0,XMM4
                    x[0][0] = x[0][0] * x[4][0];
                    // 00e4cdb3: MULSS XMM0,XMM5
                    x[0][0] = x[0][0] * x[5][0];
                    // 00e4cdb7: ANDNPS XMM2,XMM0
                    x[2] = andnps(x[2], x[0]);
                    // 00e4cdba: SHUFPS XMM2,XMM2,0x0
                    x[2] = shufps(x[2], x[2], 0x0);
                    // 00e4cdbe: MULPS XMM3,XMM2
                    x[3] = mul4(x[3], x[2]);
                    // 00e4cdc1: MOVAPS xmmword ptr [ESI + 0x120],XMM3
                    store_vec4(e, esi.wrapping_add(0x120), x[3]);
                    // 00e4cdc8: MOVAPS XMM3,xmmword ptr [EDI]
                    x[3] = load_vec4(e, edi);
                    // 00e4cdcb: MOVAPS XMM4,xmmword ptr [EBX]
                    x[4] = load_vec4(e, ebx);
                    // 00e4cdce: MOVAPS XMM0,xmmword ptr [ESI + 0x120]
                    x[0] = load_vec4(e, esi.wrapping_add(0x120));
                    // 00e4cdd5: SUBPS XMM3,XMM4
                    x[3] = sub4(x[3], x[4]);
                    // 00e4cdd8: MULPS XMM3,XMM0
                    x[3] = mul4(x[3], x[0]);
                    // 00e4cddb: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e4cddd: JNZ 0x00e4d943
                    if !fl.zf {
                        pc = 0x00e4d943;
                        continue;
                    }
                    // 00e4cde3: MOVAPS XMM4,XMM3
                    x[4] = x[3];
                    // 00e4cde6: SHUFPS XMM4,XMM3,0x55
                    x[4] = shufps(x[4], x[3], 0x55);
                    // 00e4cdea: MOVAPS XMM5,XMM3
                    x[5] = x[3];
                    // 00e4cded: ADDSS XMM4,XMM3
                    x[4][0] = x[4][0] + x[3][0];
                    // 00e4cdf1: SHUFPS XMM5,XMM3,0xaa
                    x[5] = shufps(x[5], x[3], 0xaa);
                    // 00e4cdf5: MOVAPS XMM3,xmmword ptr [ESP + 0x170]
                    x[3] = load_vec4(e, f0.wrapping_add(0x164));
                    // 00e4cdfd: ADDSS XMM5,XMM4
                    x[5][0] = x[5][0] + x[4][0];
                    // 00e4ce01: MOVSS dword ptr [ESP + 0xf4],XMM5
                    e.mem.set_f32(f0.wrapping_add(0xe8), x[5][0]);
                    // 00e4ce0a: FLD float ptr [ESP + 0xf4]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0xe8))));
                    // 00e4ce11: MOVSS dword ptr [ESP + 0xdc],XMM3
                    e.mem.set_f32(f0.wrapping_add(0xd0), x[3][0]);
                    // 00e4ce1a: FLD float ptr [ESP + 0xdc]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0xd0))));
                    // 00e4ce21: FCOMPP
                    fsw = x87.fcom(1);
                    x87.pop();
                    x87.pop();
                    // 00e4ce23: FNSTSW AX
                    eax = (eax & 0xffff_00ff) | (u32::from(fsw) << 8);
                    // 00e4ce25: TEST AH,0x41
                    fl.and(((eax >> 8) & 0xff), 0x00000041u32, 8);
                    // 00e4ce28: JZ 0x00e4d943
                    if fl.zf {
                        pc = 0x00e4d943;
                        continue;
                    }
                    // 00e4ce2e: MOVAPS XMM4,XMM1
                    x[4] = x[1];
                    // 00e4ce31: SHUFPS XMM4,XMM1,0x55
                    x[4] = shufps(x[4], x[1], 0x55);
                    // 00e4ce35: MOVAPS XMM5,XMM1
                    x[5] = x[1];
                    // 00e4ce38: SHUFPS XMM5,XMM1,0xaa
                    x[5] = shufps(x[5], x[1], 0xaa);
                    // 00e4ce3c: ADDSS XMM4,XMM1
                    x[4][0] = x[4][0] + x[1][0];
                    // 00e4ce40: ADDSS XMM5,XMM4
                    x[5][0] = x[5][0] + x[4][0];
                    // 00e4ce44: MULSS XMM5,XMM2
                    x[5][0] = x[5][0] * x[2][0];
                    // 00e4ce48: MOVAPS XMM1,XMM3
                    x[1] = x[3];
                    // 00e4ce4b: MOVSS dword ptr [ESP + 0xec],XMM5
                    e.mem.set_f32(f0.wrapping_add(0xe0), x[5][0]);
                    // 00e4ce54: FLD float ptr [ESP + 0xec]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0xe0))));
                    // 00e4ce5b: MULSS XMM1,XMM3
                    x[1][0] = x[1][0] * x[3][0];
                    // 00e4ce5f: MOVSS dword ptr [ESP + 0xbc],XMM1
                    e.mem.set_f32(f0.wrapping_add(0xb0), x[1][0]);
                    // 00e4ce68: FLD float ptr [ESP + 0xbc]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0xb0))));
                    // 00e4ce6f: FCOMPP
                    fsw = x87.fcom(1);
                    x87.pop();
                    x87.pop();
                    // 00e4ce71: FNSTSW AX
                    eax = (eax & 0xffff_00ff) | (u32::from(fsw) << 8);
                    // 00e4ce73: TEST AH,0x41
                    fl.and(((eax >> 8) & 0xff), 0x00000041u32, 8);
                    // 00e4ce76: JZ 0x00e4d943
                    if fl.zf {
                        pc = 0x00e4d943;
                        continue;
                    }
                    // 00e4ce7c: MOVAPS XMM1,xmmword ptr [0x010c44d0]
                    x[1] = load_vec4(e, 0x010c44d0);
                    // 00e4ce83: MOV EAX,dword ptr [EBP + 0x8]
                    eax = input;
                    // 00e4ce86: MOVAPS XMM5,xmmword ptr [EAX + 0x20]
                    x[5] = load_vec4(e, eax.wrapping_add(0x20));
                    // 00e4ce8a: MOV ECX,dword ptr [ESP + 0x28]
                    ecx = e.mem.u32(f0.wrapping_add(0x1c));
                    // 00e4ce8e: XORPS XMM0,XMM1
                    x[0] = xorps(x[0], x[1]);
                    // 00e4ce91: MOVAPS XMM1,xmmword ptr [EAX + 0x10]
                    x[1] = load_vec4(e, eax.wrapping_add(0x10));
                    // 00e4ce95: MOVAPS xmmword ptr [ESP + 0x110],XMM0
                    store_vec4(e, f0.wrapping_add(0x104), x[0]);
                    // 00e4ce9d: MOVAPS XMM0,xmmword ptr [EAX]
                    x[0] = load_vec4(e, eax);
                    // 00e4cea0: MOV EAX,dword ptr [ECX]
                    eax = e.mem.u32(ecx);
                    // 00e4cea2: MOV EAX,dword ptr [EAX + 0x4]
                    eax = e.mem.u32(eax.wrapping_add(0x4));
                    // 00e4cea5: MOVAPS XMM2,XMM0
                    x[2] = x[0];
                    // 00e4cea8: UNPCKLPS XMM2,XMM1
                    x[2] = unpcklps(x[2], x[1]);
                    // 00e4ceab: UNPCKHPS XMM0,XMM1
                    x[0] = unpckhps(x[0], x[1]);
                    // 00e4ceae: MOVAPS XMM4,XMM2
                    x[4] = x[2];
                    // 00e4ceb1: MOVAPS XMM3,XMM0
                    x[3] = x[0];
                    // 00e4ceb4: MOVAPS XMM0,xmmword ptr [ESI + 0x120]
                    x[0] = load_vec4(e, esi.wrapping_add(0x120));
                    // 00e4cebb: MOVLHPS XMM4,XMM5
                    x[4] = movlhps(x[4], x[5]);
                    // 00e4cebe: MOVAPS XMM1,XMM4
                    x[1] = x[4];
                    // 00e4cec1: MOVHLPS XMM1,XMM2
                    x[1] = movhlps(x[1], x[2]);
                    // 00e4cec4: SHUFPS XMM1,XMM5,0xd4
                    x[1] = shufps(x[1], x[5], 0xd4);
                    // 00e4cec8: MOVAPS XMM2,XMM0
                    x[2] = x[0];
                    // 00e4cecb: SHUFPS XMM2,XMM0,0x55
                    x[2] = shufps(x[2], x[0], 0x55);
                    // 00e4cecf: MULPS XMM2,XMM1
                    x[2] = mul4(x[2], x[1]);
                    // 00e4ced2: MOVAPS XMM1,XMM0
                    x[1] = x[0];
                    // 00e4ced5: LEA EDX,[ESP + 0x1e0]
                    edx = f0.wrapping_add(0x1d4);
                    // 00e4cedc: SHUFPS XMM1,XMM0,0x0
                    x[1] = shufps(x[1], x[0], 0x0);
                    // 00e4cee0: PUSH EDX
                    stk.push(edx);
                    // 00e4cee1: MULPS XMM1,XMM4
                    x[1] = mul4(x[1], x[4]);
                    // 00e4cee4: SHUFPS XMM3,XMM5,0xe4
                    x[3] = shufps(x[3], x[5], 0xe4);
                    // 00e4cee8: SHUFPS XMM0,XMM0,0xaa
                    x[0] = shufps(x[0], x[0], 0xaa);
                    // 00e4ceec: LEA EDX,[ESP + 0x114]
                    edx = f0.wrapping_add(0x104);
                    // 00e4cef3: ADDPS XMM2,XMM1
                    x[2] = add4(x[2], x[1]);
                    // 00e4cef6: MULPS XMM3,XMM0
                    x[3] = mul4(x[3], x[0]);
                    // 00e4cef9: ADDPS XMM2,XMM3
                    x[2] = add4(x[2], x[3]);
                    // 00e4cefc: PUSH EDX
                    stk.push(edx);
                    // 00e4cefd: MOVAPS xmmword ptr [ESP + 0x168],XMM2
                    store_vec4(e, f0.wrapping_add(0x154), x[2]);
                    // 00e4cf05: CALL EAX
                    {
                        let mut w = vec![ecx];
                        w.extend(stk.iter().rev().take(2));
                        let r = e.call(eax, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    stk.truncate(stk.len() - 2);
                    // 00e4cf07: MOV ECX,dword ptr [ESP + 0x24]
                    ecx = e.mem.u32(f0.wrapping_add(0x18));
                    // 00e4cf0b: MOV EDX,dword ptr [ECX]
                    edx = e.mem.u32(ecx);
                    // 00e4cf0d: MOV EDX,dword ptr [EDX + 0x4]
                    edx = e.mem.u32(edx.wrapping_add(0x4));
                    // 00e4cf10: LEA EAX,[ESP + 0xa0]
                    eax = f0.wrapping_add(0x94);
                    // 00e4cf17: PUSH EAX
                    stk.push(eax);
                    // 00e4cf18: LEA EAX,[ESP + 0x164]
                    eax = f0.wrapping_add(0x154);
                    // 00e4cf1f: PUSH EAX
                    stk.push(eax);
                    // 00e4cf20: CALL EDX
                    {
                        let mut w = vec![ecx];
                        w.extend(stk.iter().rev().take(2));
                        let r = e.call(edx, &w);
                        eax = r.eax;
                        edx = r.edx;
                    }
                    stk.truncate(stk.len() - 2);
                    // 00e4cf22: MOVAPS XMM1,xmmword ptr [ESP + 0xa0]
                    x[1] = load_vec4(e, f0.wrapping_add(0x94));
                    // 00e4cf2a: MOV ECX,dword ptr [EBP + 0x8]
                    ecx = input;
                    // 00e4cf2d: MOVAPS XMM2,xmmword ptr [ECX + 0x10]
                    x[2] = load_vec4(e, ecx.wrapping_add(0x10));
                    // 00e4cf31: MOVAPS XMM3,xmmword ptr [ECX]
                    x[3] = load_vec4(e, ecx);
                    // 00e4cf34: MOV EDX,dword ptr [ESP + 0x14]
                    edx = e.mem.u32(f0.wrapping_add(0x8));
                    // 00e4cf38: MOVAPS XMM5,xmmword ptr [ESP + 0x170]
                    x[5] = load_vec4(e, f0.wrapping_add(0x164));
                    // 00e4cf40: MOVAPS XMM0,XMM1
                    x[0] = x[1];
                    // 00e4cf43: SHUFPS XMM0,XMM1,0x55
                    x[0] = shufps(x[0], x[1], 0x55);
                    // 00e4cf47: MULPS XMM0,XMM2
                    x[0] = mul4(x[0], x[2]);
                    // 00e4cf4a: MOVAPS XMM2,XMM1
                    x[2] = x[1];
                    // 00e4cf4d: SHUFPS XMM2,XMM1,0x0
                    x[2] = shufps(x[2], x[1], 0x0);
                    // 00e4cf51: MULPS XMM2,XMM3
                    x[2] = mul4(x[2], x[3]);
                    // 00e4cf54: ADDPS XMM0,XMM2
                    x[0] = add4(x[0], x[2]);
                    // 00e4cf57: MOVAPS XMM2,XMM1
                    x[2] = x[1];
                    // 00e4cf5a: SHUFPS XMM2,XMM1,0xaa
                    x[2] = shufps(x[2], x[1], 0xaa);
                    // 00e4cf5e: MOVAPS XMM1,xmmword ptr [ECX + 0x20]
                    x[1] = load_vec4(e, ecx.wrapping_add(0x20));
                    // 00e4cf62: MULPS XMM2,XMM1
                    x[2] = mul4(x[2], x[1]);
                    // 00e4cf65: MOVAPS XMM1,xmmword ptr [ECX + 0x30]
                    x[1] = load_vec4(e, ecx.wrapping_add(0x30));
                    // 00e4cf69: ADDPS XMM2,XMM1
                    x[2] = add4(x[2], x[1]);
                    // 00e4cf6c: ADDPS XMM0,XMM2
                    x[0] = add4(x[0], x[2]);
                    // 00e4cf6f: LEA EAX,[EDX + 0xa]
                    eax = edx.wrapping_add(0xa);
                    // 00e4cf72: SHL EAX,0x4
                    {
                        let r = fl.shl(eax, 4, 32);
                        eax = r;
                    }
                    // 00e4cf75: MOVAPS xmmword ptr [EAX + ESI*0x1],XMM0
                    store_vec4(e, eax.wrapping_add(esi), x[0]);
                    // 00e4cf79: MOVAPS XMM1,xmmword ptr [ECX + 0x50]
                    x[1] = load_vec4(e, ecx.wrapping_add(0x50));
                    // 00e4cf7d: MOVAPS XMM6,xmmword ptr [ESP + 0x1e0]
                    x[6] = load_vec4(e, f0.wrapping_add(0x1d4));
                    // 00e4cf85: MOVAPS XMM2,xmmword ptr [EDI]
                    x[2] = load_vec4(e, edi);
                    // 00e4cf88: MOVAPS XMM7,xmmword ptr [ESP + 0x110]
                    x[7] = load_vec4(e, f0.wrapping_add(0x104));
                    // 00e4cf90: ADD EAX,ESI
                    {
                        let r = fl.add(eax, esi, 32);
                        eax = r;
                    }
                    // 00e4cf92: MOVSS dword ptr [ESP + 0x4c],XMM1
                    e.mem.set_f32(f0.wrapping_add(0x40), x[1][0]);
                    // 00e4cf98: MOVAPS XMM1,XMM6
                    x[1] = x[6];
                    // 00e4cf9b: SUBPS XMM1,XMM0
                    x[1] = sub4(x[1], x[0]);
                    // 00e4cf9e: MOVAPS XMM0,xmmword ptr [ESI + 0x120]
                    x[0] = load_vec4(e, esi.wrapping_add(0x120));
                    // 00e4cfa5: MULPS XMM1,XMM0
                    x[1] = mul4(x[1], x[0]);
                    // 00e4cfa8: MOVAPS XMM0,XMM6
                    x[0] = x[6];
                    // 00e4cfab: SUBPS XMM0,XMM2
                    x[0] = sub4(x[0], x[2]);
                    // 00e4cfae: MULPS XMM0,XMM7
                    x[0] = mul4(x[0], x[7]);
                    // 00e4cfb1: MOVAPS XMM3,xmmword ptr [ESI + 0xe0]
                    x[3] = load_vec4(e, esi.wrapping_add(0xe0));
                    // 00e4cfb8: MOVAPS XMM2,xmmword ptr [ESP + 0xa0]
                    x[2] = load_vec4(e, f0.wrapping_add(0x94));
                    // 00e4cfc0: SUBPS XMM2,XMM3
                    x[2] = sub4(x[2], x[3]);
                    // 00e4cfc3: MULPS XMM2,xmmword ptr [ESP + 0x160]
                    x[2] = mul4(x[2], load_vec4(e, f0.wrapping_add(0x154)));
                    // 00e4cfcb: MOVAPS XMM3,XMM1
                    x[3] = x[1];
                    // 00e4cfce: UNPCKLPS XMM3,XMM0
                    x[3] = unpcklps(x[3], x[0]);
                    // 00e4cfd1: UNPCKHPS XMM1,XMM0
                    x[1] = unpckhps(x[1], x[0]);
                    // 00e4cfd4: MOVAPS XMM4,XMM3
                    x[4] = x[3];
                    // 00e4cfd7: MOVLHPS XMM4,XMM2
                    x[4] = movlhps(x[4], x[2]);
                    // 00e4cfda: SHUFPS XMM1,XMM2,0xe4
                    x[1] = shufps(x[1], x[2], 0xe4);
                    // 00e4cfde: MOVAPS XMM0,XMM4
                    x[0] = x[4];
                    // 00e4cfe1: MOVHLPS XMM0,XMM3
                    x[0] = movhlps(x[0], x[3]);
                    // 00e4cfe4: SHUFPS XMM0,XMM2,0xd4
                    x[0] = shufps(x[0], x[2], 0xd4);
                    // 00e4cfe8: MOVAPS XMM2,xmmword ptr [ESI + 0x30]
                    x[2] = load_vec4(e, esi.wrapping_add(0x30));
                    // 00e4cfec: ADDPS XMM0,XMM4
                    x[0] = add4(x[0], x[4]);
                    // 00e4cfef: ADDPS XMM1,XMM0
                    x[1] = add4(x[1], x[0]);
                    // 00e4cff2: MOVAPS XMM0,XMM6
                    x[0] = x[6];
                    // 00e4cff5: SUBPS XMM0,XMM2
                    x[0] = sub4(x[0], x[2]);
                    // 00e4cff8: MULPS XMM0,XMM7
                    x[0] = mul4(x[0], x[7]);
                    // 00e4cffb: MOVAPS XMM2,XMM0
                    x[2] = x[0];
                    // 00e4cffe: SHUFPS XMM2,XMM0,0x55
                    x[2] = shufps(x[2], x[0], 0x55);
                    // 00e4d002: ADDSS XMM2,XMM0
                    x[2][0] = x[2][0] + x[0][0];
                    // 00e4d006: MOVAPS XMM3,XMM0
                    x[3] = x[0];
                    // 00e4d009: SHUFPS XMM3,XMM0,0xaa
                    x[3] = shufps(x[3], x[0], 0xaa);
                    // 00e4d00d: ADDSS XMM3,XMM2
                    x[3][0] = x[3][0] + x[2][0];
                    // 00e4d011: MOVAPS XMM2,xmmword ptr [ESI + 0x40]
                    x[2] = load_vec4(e, esi.wrapping_add(0x40));
                    // 00e4d015: MOVSS dword ptr [ESP + 0xfc],XMM3
                    e.mem.set_f32(f0.wrapping_add(0xf0), x[3][0]);
                    // 00e4d01e: FLD float ptr [ESP + 0xfc]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0xf0))));
                    // 00e4d025: MOVAPS XMM0,XMM6
                    x[0] = x[6];
                    // 00e4d028: FSTP float ptr [ESP + 0x150]
                    e.mem.set_f32(f0.wrapping_add(0x144), x87.fstp_f32());
                    // 00e4d02f: SUBPS XMM0,XMM2
                    x[0] = sub4(x[0], x[2]);
                    // 00e4d032: MULPS XMM0,XMM7
                    x[0] = mul4(x[0], x[7]);
                    // 00e4d035: MOVAPS XMM2,XMM0
                    x[2] = x[0];
                    // 00e4d038: SHUFPS XMM2,XMM0,0x55
                    x[2] = shufps(x[2], x[0], 0x55);
                    // 00e4d03c: ADDSS XMM2,XMM0
                    x[2][0] = x[2][0] + x[0][0];
                    // 00e4d040: MOVAPS XMM3,XMM0
                    x[3] = x[0];
                    // 00e4d043: SHUFPS XMM3,XMM0,0xaa
                    x[3] = shufps(x[3], x[0], 0xaa);
                    // 00e4d047: ADDSS XMM3,XMM2
                    x[3][0] = x[3][0] + x[2][0];
                    // 00e4d04b: MOVAPS XMM2,xmmword ptr [ESP + 0xa0]
                    x[2] = load_vec4(e, f0.wrapping_add(0x94));
                    // 00e4d053: MOVSS dword ptr [ESP + 0xe8],XMM3
                    e.mem.set_f32(f0.wrapping_add(0xdc), x[3][0]);
                    // 00e4d05c: FLD float ptr [ESP + 0xe8]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0xdc))));
                    // 00e4d063: MOVAPS XMM3,xmmword ptr [ESI + 0xf0]
                    x[3] = load_vec4(e, esi.wrapping_add(0xf0));
                    // 00e4d06a: FSTP float ptr [ESP + 0x154]
                    e.mem.set_f32(f0.wrapping_add(0x148), x87.fstp_f32());
                    // 00e4d071: MOVAPS XMM0,XMM2
                    x[0] = x[2];
                    // 00e4d074: SUBPS XMM0,XMM3
                    x[0] = sub4(x[0], x[3]);
                    // 00e4d077: MOVAPS XMM3,xmmword ptr [ESP + 0x160]
                    x[3] = load_vec4(e, f0.wrapping_add(0x154));
                    // 00e4d07f: MULPS XMM0,XMM3
                    x[0] = mul4(x[0], x[3]);
                    // 00e4d082: MOVAPS XMM4,XMM0
                    x[4] = x[0];
                    // 00e4d085: SHUFPS XMM4,XMM0,0x55
                    x[4] = shufps(x[4], x[0], 0x55);
                    // 00e4d089: ADDSS XMM4,XMM0
                    x[4][0] = x[4][0] + x[0][0];
                    // 00e4d08d: MOVAPS XMM7,XMM0
                    x[7] = x[0];
                    // 00e4d090: SHUFPS XMM7,XMM0,0xaa
                    x[7] = shufps(x[7], x[0], 0xaa);
                    // 00e4d094: ADDSS XMM7,XMM4
                    x[7][0] = x[7][0] + x[4][0];
                    // 00e4d098: MOVAPS XMM4,xmmword ptr [ESI + 0x100]
                    x[4] = load_vec4(e, esi.wrapping_add(0x100));
                    // 00e4d09f: MOVAPS XMM0,XMM2
                    x[0] = x[2];
                    // 00e4d0a2: SUBPS XMM0,XMM4
                    x[0] = sub4(x[0], x[4]);
                    // 00e4d0a5: MULPS XMM0,XMM3
                    x[0] = mul4(x[0], x[3]);
                    // 00e4d0a8: MOVAPS XMM2,XMM0
                    x[2] = x[0];
                    // 00e4d0ab: SHUFPS XMM2,XMM0,0x55
                    x[2] = shufps(x[2], x[0], 0x55);
                    // 00e4d0af: ADDSS XMM2,XMM0
                    x[2][0] = x[2][0] + x[0][0];
                    // 00e4d0b3: MOVAPS XMM3,XMM0
                    x[3] = x[0];
                    // 00e4d0b6: SHUFPS XMM3,XMM0,0xaa
                    x[3] = shufps(x[3], x[0], 0xaa);
                    // 00e4d0ba: ADDSS XMM3,XMM2
                    x[3][0] = x[3][0] + x[2][0];
                    // 00e4d0be: MOVSS dword ptr [ESP + 0xd8],XMM7
                    e.mem.set_f32(f0.wrapping_add(0xcc), x[7][0]);
                    // 00e4d0c7: FLD float ptr [ESP + 0xd8]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0xcc))));
                    // 00e4d0ce: MOVSS dword ptr [ESP + 0xf8],XMM3
                    e.mem.set_f32(f0.wrapping_add(0xec), x[3][0]);
                    // 00e4d0d7: FSTP float ptr [ESP + 0x158]
                    e.mem.set_f32(f0.wrapping_add(0x14c), x87.fstp_f32());
                    // 00e4d0de: FLD float ptr [ESP + 0xf8]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0xec))));
                    // 00e4d0e5: MOVSS XMM3,dword ptr [ESP + 0x4c]
                    x[3] = [
                        f32::from_bits(e.mem.u32(f0.wrapping_add(0x40))),
                        0.0,
                        0.0,
                        0.0,
                    ];
                    // 00e4d0eb: FSTP float ptr [ESP + 0x15c]
                    e.mem.set_f32(f0.wrapping_add(0x150), x87.fstp_f32());
                    // 00e4d0f2: MOVAPS XMM0,xmmword ptr [ESP + 0x150]
                    x[0] = load_vec4(e, f0.wrapping_add(0x144));
                    // 00e4d0fa: MOVAPS XMM2,XMM5
                    x[2] = x[5];
                    // 00e4d0fd: SHUFPS XMM2,XMM5,0x0
                    x[2] = shufps(x[2], x[5], 0x0);
                    // 00e4d101: CMPLTPS XMM0,XMM2
                    x[0] = cmpltps(x[0], x[2]);
                    // 00e4d105: MOVSS XMM2,XMM5
                    x[2][0] = x[5][0];
                    // 00e4d109: MOVSS XMM4,XMM2
                    x[4][0] = x[2][0];
                    // 00e4d10d: MOVSS XMM7,XMM2
                    x[7][0] = x[2][0];
                    // 00e4d111: UNPCKLPS XMM4,XMM2
                    x[4] = unpcklps(x[4], x[2]);
                    // 00e4d114: UNPCKLPS XMM3,XMM7
                    x[3] = unpcklps(x[3], x[7]);
                    // 00e4d117: UNPCKLPS XMM3,XMM4
                    x[3] = unpcklps(x[3], x[4]);
                    // 00e4d11a: CMPLTPS XMM3,XMM1
                    x[3] = cmpltps(x[3], x[1]);
                    // 00e4d11e: MOVMSKPS EAX,XMM3
                    eax = movmskps(x[3]);
                    // 00e4d121: AND EAX,0x7
                    {
                        let r = fl.and(eax, 0x00000007u32, 32);
                        eax = r;
                    }
                    // 00e4d124: DEC EAX
                    {
                        let r = fl.dec(eax, 32);
                        eax = r;
                    }
                    // 00e4d125: CMP EAX,0x6
                    fl.sub(eax, 0x00000006u32, 32);
                    // 00e4d128: JA 0x00e4d873
                    if !fl.cf && !fl.zf {
                        pc = 0x00e4d873;
                        continue;
                    }
                    // 00e4d12e: JMP dword ptr [EAX*0x4 + 0xe4da70]
                    pc = match eax {
                        0 => 0x00e4d866,
                        1 => 0x00e4d80c,
                        2 => 0x00e4d866,
                        3 => 0x00e4d837,
                        4 => 0x00e4d866,
                        5 => 0x00e4d7e4,
                        6 => 0x00e4d866,
                        _ => unreachable!(),
                    };
                }
                0x00e4d135 => {
                    // 00e4d135: MOVAPS XMM0,XMM2
                    x[0] = x[2];
                    // 00e4d138: SHUFPS XMM0,XMM2,0x55
                    x[0] = shufps(x[0], x[2], 0x55);
                    // 00e4d13c: MOVAPS XMM4,XMM2
                    x[4] = x[2];
                    // 00e4d13f: ADDSS XMM4,XMM0
                    x[4][0] = x[4][0] + x[0][0];
                    // 00e4d143: MOVAPS XMM0,xmmword ptr [0x010ec7f0]
                    x[0] = load_vec4(e, 0x010ec7f0);
                    // 00e4d14a: DIVSS XMM2,XMM4
                    x[2][0] = x[2][0] / x[4][0];
                    // 00e4d14e: SHUFPS XMM2,XMM2,0x0
                    x[2] = shufps(x[2], x[2], 0x0);
                    // 00e4d152: SUBPS XMM0,XMM2
                    x[0] = sub4(x[0], x[2]);
                    // 00e4d155: MULPS XMM5,XMM1
                    x[5] = mul4(x[5], x[1]);
                    // 00e4d158: MULPS XMM0,XMM3
                    x[0] = mul4(x[0], x[3]);
                    // 00e4d15b: MULPS XMM6,XMM2
                    x[6] = mul4(x[6], x[2]);
                    // 00e4d15e: ADDPS XMM0,XMM6
                    x[0] = add4(x[0], x[6]);
                    // 00e4d161: MOVAPS xmmword ptr [ESI + 0x130],XMM0
                    store_vec4(e, esi.wrapping_add(0x130), x[0]);
                    // 00e4d168: MOVAPS XMM1,XMM5
                    x[1] = x[5];
                    // 00e4d16b: SHUFPS XMM1,XMM5,0x55
                    x[1] = shufps(x[1], x[5], 0x55);
                    // 00e4d16f: MOVAPS XMM0,XMM5
                    x[0] = x[5];
                    // 00e4d172: ADDSS XMM1,XMM5
                    x[1][0] = x[1][0] + x[5][0];
                    // 00e4d176: SHUFPS XMM0,XMM5,0xaa
                    x[0] = shufps(x[0], x[5], 0xaa);
                    pc = 0x00e4d17a;
                }
                0x00e4d17a => {
                    // 00e4d17a: MOV EAX,dword ptr [ESP + 0x14]
                    eax = e.mem.u32(f0.wrapping_add(0x8));
                    // 00e4d17e: ADDSS XMM0,XMM1
                    x[0][0] = x[0][0] + x[1][0];
                    // 00e4d182: MOVAPS XMM1,XMM0
                    x[1] = x[0];
                    // 00e4d185: SHUFPS XMM1,XMM0,0x0
                    x[1] = shufps(x[1], x[0], 0x0);
                    // 00e4d189: MOVAPS XMM0,xmmword ptr [0x010c44d0]
                    x[0] = load_vec4(e, 0x010c44d0);
                    // 00e4d190: ANDPS XMM1,XMM0
                    x[1] = andps(x[1], x[0]);
                    // 00e4d193: MOVAPS XMM0,xmmword ptr [ESI + 0x120]
                    x[0] = load_vec4(e, esi.wrapping_add(0x120));
                    // 00e4d19a: XORPS XMM1,XMM0
                    x[1] = xorps(x[1], x[0]);
                    // 00e4d19d: MOVAPS xmmword ptr [ESI + 0x120],XMM1
                    store_vec4(e, esi.wrapping_add(0x120), x[1]);
                    // 00e4d1a4: MOV dword ptr [ESI + 0x10],EAX
                    e.mem.set_u32(esi.wrapping_add(0x10), eax);
                    // 00e4d1a7: XOR EAX,EAX
                    {
                        let r = fl.xor(eax, eax, 32);
                        eax = r;
                    }
                    // 00e4d1a9: JMP 0x00e4cd45
                    pc = 0x00e4cd45;
                }
                0x00e4d1ae => {
                    // 00e4d1ae: MOVAPS XMM0,xmmword ptr [ESI + 0x30]
                    x[0] = load_vec4(e, esi.wrapping_add(0x30));
                    // 00e4d1b2: MOVAPS xmmword ptr [EDI],XMM0
                    store_vec4(e, edi, x[0]);
                    pc = 0x00e4d1b5;
                }
                0x00e4d1b5 => {
                    // 00e4d1b5: MOV dword ptr [ESP + 0x18],0x1
                    e.mem.set_u32(f0.wrapping_add(0xc), 0x00000001u32);
                    // 00e4d1bd: JMP 0x00e4d45b
                    pc = 0x00e4d45b;
                }
                0x00e4d1c2 => {
                    // 00e4d1c2: MOVAPS XMM0,xmmword ptr [ESI + 0xb0]
                    x[0] = load_vec4(e, esi.wrapping_add(0xb0));
                    // 00e4d1c9: MOVAPS xmmword ptr [EBX],XMM0
                    store_vec4(e, ebx, x[0]);
                    // 00e4d1cc: MOVAPS XMM0,xmmword ptr [ESI + 0xf0]
                    x[0] = load_vec4(e, esi.wrapping_add(0xf0));
                    // 00e4d1d3: MOVAPS xmmword ptr [ESI + 0xe0],XMM0
                    store_vec4(e, esi.wrapping_add(0xe0), x[0]);
                    pc = 0x00e4d1da;
                }
                0x00e4d1da => {
                    // 00e4d1da: MOV dword ptr [ESP + 0x14],0x1
                    e.mem.set_u32(f0.wrapping_add(0x8), 0x00000001u32);
                    // 00e4d1e2: JMP 0x00e4d6be
                    pc = 0x00e4d6be;
                }
                0x00e4d1e7 => {
                    // 00e4d1e7: MOVAPS XMM0,xmmword ptr [ESI + 0x120]
                    x[0] = load_vec4(e, esi.wrapping_add(0x120));
                    // 00e4d1ee: MULPS XMM0,xmmword ptr [ESP + 0x1b0]
                    x[0] = mul4(x[0], load_vec4(e, f0.wrapping_add(0x1a4)));
                    // 00e4d1f6: MOVAPS XMM1,XMM0
                    x[1] = x[0];
                    // 00e4d1f9: SHUFPS XMM1,XMM0,0x55
                    x[1] = shufps(x[1], x[0], 0x55);
                    // 00e4d1fd: ADDSS XMM1,XMM0
                    x[1][0] = x[1][0] + x[0][0];
                    // 00e4d201: SHUFPS XMM0,XMM0,0xaa
                    x[0] = shufps(x[0], x[0], 0xaa);
                    // 00e4d205: JMP 0x00e4d17a
                    pc = 0x00e4d17a;
                }
                0x00e4d20a => {
                    // 00e4d20a: FLD1
                    x87.fld1();
                    // 00e4d20c: PUSH ECX
                    stk.push(ecx);
                    // 00e4d20d: FSTP float ptr [ESP]
                    *stk.last_mut().unwrap() = x87.fstp_f32().to_bits();
                    // 00e4d210: PUSH EBX
                    stk.push(ebx);
                    // 00e4d211: PUSH EDI
                    stk.push(edi);
                    // 00e4d212: CALL 0x00e4b810
                    {
                        let w: Vec<u32> = stk.iter().rev().take(3).copied().collect();
                        let r = find_closest_triangle_ov2(e, w[0], w[1], f32::from_bits(w[2]));
                        eax = r as u32;
                    }
                    // 00e4d217: ADD ESP,0xc
                    stk.truncate(stk.len() - 3);
                    // 00e4d21a: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e4d21c: JL 0x00e4cd40
                    if fl.sf != fl.of {
                        pc = 0x00e4cd40;
                        continue;
                    }
                    // 00e4d222: MOVAPS XMM0,xmmword ptr [ESI + 0xd0]
                    x[0] = load_vec4(e, esi.wrapping_add(0xd0));
                    // 00e4d229: DEC dword ptr [ESP + 0x14]
                    {
                        let r = fl.dec(e.mem.u32(f0.wrapping_add(0x8)), 32);
                        e.mem.set_u32(f0.wrapping_add(0x8), r);
                    }
                    // 00e4d22d: LEA ECX,[EAX + 0xa]
                    ecx = eax.wrapping_add(0xa);
                    // 00e4d230: SHL ECX,0x4
                    {
                        let r = fl.shl(ecx, 4, 32);
                        ecx = r;
                    }
                    // 00e4d233: ADD EAX,0xe
                    {
                        let r = fl.add(eax, 0x0000000eu32, 32);
                        eax = r;
                    }
                    // 00e4d236: MOVAPS xmmword ptr [ECX + ESI*0x1],XMM0
                    store_vec4(e, ecx.wrapping_add(esi), x[0]);
                    // 00e4d23a: MOVAPS XMM0,xmmword ptr [ESI + 0x110]
                    x[0] = load_vec4(e, esi.wrapping_add(0x110));
                    // 00e4d241: SHL EAX,0x4
                    {
                        let r = fl.shl(eax, 4, 32);
                        eax = r;
                    }
                    // 00e4d244: MOVAPS xmmword ptr [EAX + ESI*0x1],XMM0
                    store_vec4(e, eax.wrapping_add(esi), x[0]);
                    pc = 0x00e4d248;
                }
                0x00e4d248 => {
                    // 00e4d248: LEA EAX,[ESI + 0x120]
                    eax = esi.wrapping_add(0x120);
                    // 00e4d24e: PUSH EAX
                    stk.push(eax);
                    // 00e4d24f: PUSH EBX
                    stk.push(ebx);
                    // 00e4d250: PUSH EDI
                    stk.push(edi);
                    // 00e4d251: MOV ECX,ESI
                    ecx = esi;
                    // 00e4d253: CALL 0x00e4da90
                    {
                        let mut w = vec![ecx];
                        w.extend(stk.iter().rev().take(3));
                        let r = hk_gsk_check_triangle_boundaries_1(e, w[0], w[1], w[2], w[3]);
                        eax = r;
                    }
                    stk.truncate(stk.len() - 3);
                    // 00e4d258: CMP EAX,0x7
                    fl.sub(eax, 0x00000007u32, 32);
                    // 00e4d25b: JZ 0x00e4d7d6
                    if fl.zf {
                        pc = 0x00e4d7d6;
                        continue;
                    }
                    // 00e4d261: MOVSX EAX,byte ptr [EAX + 0x10ececc]
                    eax = (u32::from(e.mem.u8(eax.wrapping_add(0x10ececc))) as i8 as i32 as u32);
                    // 00e4d268: MOV dword ptr [ESP + 0x14],0x2
                    e.mem.set_u32(f0.wrapping_add(0x8), 0x00000002u32);
                    // 00e4d270: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e4d272: JGE 0x00e4d29e
                    if fl.sf == fl.of {
                        pc = 0x00e4d29e;
                        continue;
                    }
                    // 00e4d274: MOVAPS XMM0,xmmword ptr [ESI + 0xc0]
                    x[0] = load_vec4(e, esi.wrapping_add(0xc0));
                    // 00e4d27b: AND EAX,0x3
                    {
                        let r = fl.and(eax, 0x00000003u32, 32);
                        eax = r;
                    }
                    // 00e4d27e: LEA EDX,[EAX + 0xa]
                    edx = eax.wrapping_add(0xa);
                    // 00e4d281: SHL EDX,0x4
                    {
                        let r = fl.shl(edx, 4, 32);
                        edx = r;
                    }
                    // 00e4d284: ADD EAX,0xe
                    {
                        let r = fl.add(eax, 0x0000000eu32, 32);
                        eax = r;
                    }
                    // 00e4d287: MOVAPS xmmword ptr [EDX + ESI*0x1],XMM0
                    store_vec4(e, edx.wrapping_add(esi), x[0]);
                    // 00e4d28b: MOVAPS XMM0,xmmword ptr [ESI + 0x100]
                    x[0] = load_vec4(e, esi.wrapping_add(0x100));
                    // 00e4d292: SHL EAX,0x4
                    {
                        let r = fl.shl(eax, 4, 32);
                        eax = r;
                    }
                    // 00e4d295: MOVAPS xmmword ptr [EAX + ESI*0x1],XMM0
                    store_vec4(e, eax.wrapping_add(esi), x[0]);
                    // 00e4d299: JMP 0x00e4d45b
                    pc = 0x00e4d45b;
                }
                0x00e4d29e => {
                    // 00e4d29e: MOVSX EDX,byte ptr [EAX + 0x10eced4]
                    edx = (u32::from(e.mem.u8(eax.wrapping_add(0x10eced4))) as i8 as i32 as u32);
                    // 00e4d2a5: MOVSX ECX,byte ptr [EAX + 0x10eced6]
                    ecx = (u32::from(e.mem.u8(eax.wrapping_add(0x10eced6))) as i8 as i32 as u32);
                    // 00e4d2ac: MOVAPS XMM2,xmmword ptr [EDI]
                    x[2] = load_vec4(e, edi);
                    // 00e4d2af: ADD EAX,0xa
                    {
                        let r = fl.add(eax, 0x0000000au32, 32);
                        eax = r;
                    }
                    // 00e4d2b2: SHL EAX,0x4
                    {
                        let r = fl.shl(eax, 4, 32);
                        eax = r;
                    }
                    // 00e4d2b5: MOVAPS XMM3,xmmword ptr [EAX + ESI*0x1]
                    x[3] = load_vec4(e, eax.wrapping_add(esi));
                    // 00e4d2b9: LEA EAX,[ECX + 0xa]
                    eax = ecx.wrapping_add(0xa);
                    // 00e4d2bc: SHL EAX,0x4
                    {
                        let r = fl.shl(eax, 4, 32);
                        eax = r;
                    }
                    // 00e4d2bf: MOVAPS XMM1,xmmword ptr [EAX + ESI*0x1]
                    x[1] = load_vec4(e, eax.wrapping_add(esi));
                    // 00e4d2c3: ADD EAX,ESI
                    {
                        let r = fl.add(eax, esi, 32);
                        eax = r;
                    }
                    // 00e4d2c5: MOV dword ptr [ESP + 0x50],EAX
                    e.mem.set_u32(f0.wrapping_add(0x44), eax);
                    // 00e4d2c9: LEA EAX,[EDX + 0xa]
                    eax = edx.wrapping_add(0xa);
                    // 00e4d2cc: SHL EAX,0x4
                    {
                        let r = fl.shl(eax, 4, 32);
                        eax = r;
                    }
                    // 00e4d2cf: MOVAPS XMM0,xmmword ptr [EAX + ESI*0x1]
                    x[0] = load_vec4(e, eax.wrapping_add(esi));
                    // 00e4d2d3: ADD EAX,ESI
                    {
                        let r = fl.add(eax, esi, 32);
                        eax = r;
                    }
                    // 00e4d2d5: SUBPS XMM2,XMM3
                    x[2] = sub4(x[2], x[3]);
                    // 00e4d2d8: SUBPS XMM0,XMM3
                    x[0] = sub4(x[0], x[3]);
                    // 00e4d2db: MULPS XMM0,XMM2
                    x[0] = mul4(x[0], x[2]);
                    // 00e4d2de: SUBPS XMM1,XMM3
                    x[1] = sub4(x[1], x[3]);
                    // 00e4d2e1: MOVAPS XMM3,XMM0
                    x[3] = x[0];
                    // 00e4d2e4: SHUFPS XMM3,XMM0,0x55
                    x[3] = shufps(x[3], x[0], 0x55);
                    // 00e4d2e8: MOVAPS XMM4,XMM0
                    x[4] = x[0];
                    // 00e4d2eb: MOV dword ptr [ESP + 0x30],EAX
                    e.mem.set_u32(f0.wrapping_add(0x24), eax);
                    // 00e4d2ef: ADDSS XMM3,XMM0
                    x[3][0] = x[3][0] + x[0][0];
                    // 00e4d2f3: SHUFPS XMM4,XMM0,0xaa
                    x[4] = shufps(x[4], x[0], 0xaa);
                    // 00e4d2f7: ADDSS XMM4,XMM3
                    x[4][0] = x[4][0] + x[3][0];
                    // 00e4d2fb: MOVMSKPS EAX,XMM4
                    eax = movmskps(x[4]);
                    // 00e4d2fe: TEST AL,0x1
                    fl.and((eax & 0xff), 0x00000001u32, 8);
                    // 00e4d300: MOV EAX,dword ptr [ESI + 0xc0]
                    eax = e.mem.u32(esi.wrapping_add(0xc0));
                    // 00e4d306: JZ 0x00e4d3d8
                    if fl.zf {
                        pc = 0x00e4d3d8;
                        continue;
                    }
                    // 00e4d30c: MOV dword ptr [ESP + 0x100],EAX
                    e.mem.set_u32(f0.wrapping_add(0xf4), eax);
                    // 00e4d313: MOV EAX,dword ptr [ESI + 0xc4]
                    eax = e.mem.u32(esi.wrapping_add(0xc4));
                    // 00e4d319: MOV dword ptr [ESP + 0x104],EAX
                    e.mem.set_u32(f0.wrapping_add(0xf8), eax);
                    // 00e4d320: MOV EAX,dword ptr [ESI + 0xc8]
                    eax = e.mem.u32(esi.wrapping_add(0xc8));
                    // 00e4d326: MOV dword ptr [ESP + 0x108],EAX
                    e.mem.set_u32(f0.wrapping_add(0xfc), eax);
                    // 00e4d32d: MOV EAX,dword ptr [ESI + 0xcc]
                    eax = e.mem.u32(esi.wrapping_add(0xcc));
                    // 00e4d333: MOV dword ptr [ESP + 0x10c],EAX
                    e.mem.set_u32(f0.wrapping_add(0x100), eax);
                    // 00e4d33a: MOV EAX,dword ptr [ESI + 0x100]
                    eax = e.mem.u32(esi.wrapping_add(0x100));
                    // 00e4d340: MOVAPS XMM0,xmmword ptr [ESP + 0x100]
                    x[0] = load_vec4(e, f0.wrapping_add(0xf4));
                    // 00e4d348: MOV dword ptr [ESP + 0x70],EAX
                    e.mem.set_u32(f0.wrapping_add(0x64), eax);
                    // 00e4d34c: MOV EAX,dword ptr [ESI + 0x104]
                    eax = e.mem.u32(esi.wrapping_add(0x104));
                    // 00e4d352: MOV dword ptr [ESP + 0x74],EAX
                    e.mem.set_u32(f0.wrapping_add(0x68), eax);
                    // 00e4d356: MOV EAX,dword ptr [ESI + 0x108]
                    eax = e.mem.u32(esi.wrapping_add(0x108));
                    // 00e4d35c: MOV dword ptr [ESP + 0x78],EAX
                    e.mem.set_u32(f0.wrapping_add(0x6c), eax);
                    // 00e4d360: MOV EAX,dword ptr [ESI + 0x10c]
                    eax = e.mem.u32(esi.wrapping_add(0x10c));
                    // 00e4d366: MOV dword ptr [ESP + 0x7c],EAX
                    e.mem.set_u32(f0.wrapping_add(0x70), eax);
                    // 00e4d36a: MOV EAX,dword ptr [ESP + 0x30]
                    eax = e.mem.u32(f0.wrapping_add(0x24));
                    // 00e4d36e: MOVAPS xmmword ptr [EAX],XMM0
                    store_vec4(e, eax, x[0]);
                    // 00e4d371: MOVAPS XMM0,xmmword ptr [ESP + 0x70]
                    x[0] = load_vec4(e, f0.wrapping_add(0x64));
                    // 00e4d376: MULPS XMM1,XMM2
                    x[1] = mul4(x[1], x[2]);
                    // 00e4d379: LEA EAX,[EDX + 0xe]
                    eax = edx.wrapping_add(0xe);
                    // 00e4d37c: SHL EAX,0x4
                    {
                        let r = fl.shl(eax, 4, 32);
                        eax = r;
                    }
                    // 00e4d37f: MOVAPS xmmword ptr [EAX + ESI*0x1],XMM0
                    store_vec4(e, eax.wrapping_add(esi), x[0]);
                    // 00e4d383: MOVAPS XMM0,XMM1
                    x[0] = x[1];
                    // 00e4d386: SHUFPS XMM0,XMM1,0x55
                    x[0] = shufps(x[0], x[1], 0x55);
                    // 00e4d38a: MOVAPS XMM2,XMM1
                    x[2] = x[1];
                    // 00e4d38d: ADDSS XMM0,XMM1
                    x[0][0] = x[0][0] + x[1][0];
                    // 00e4d391: SHUFPS XMM2,XMM1,0xaa
                    x[2] = shufps(x[2], x[1], 0xaa);
                    // 00e4d395: ADDSS XMM2,XMM0
                    x[2][0] = x[2][0] + x[0][0];
                    // 00e4d399: MOVMSKPS EAX,XMM2
                    eax = movmskps(x[2]);
                    // 00e4d39c: TEST AL,0x1
                    fl.and((eax & 0xff), 0x00000001u32, 8);
                    // 00e4d39e: JZ 0x00e4d45b
                    if fl.zf {
                        pc = 0x00e4d45b;
                        continue;
                    }
                    // 00e4d3a4: CMP ECX,0x2
                    fl.sub(ecx, 0x00000002u32, 32);
                    // 00e4d3a7: JNZ 0x00e4d3ab
                    if !fl.zf {
                        pc = 0x00e4d3ab;
                        continue;
                    }
                    // 00e4d3a9: MOV ECX,EDX
                    ecx = edx;
                    pc = 0x00e4d3ab;
                }
                0x00e4d3ab => {
                    // 00e4d3ab: MOVAPS XMM0,xmmword ptr [ESI + 0xf0]
                    x[0] = load_vec4(e, esi.wrapping_add(0xf0));
                    // 00e4d3b2: MOVAPS XMM1,xmmword ptr [ESI + 0xb0]
                    x[1] = load_vec4(e, esi.wrapping_add(0xb0));
                    // 00e4d3b9: LEA EDX,[ECX + 0xa]
                    edx = ecx.wrapping_add(0xa);
                    // 00e4d3bc: SHL EDX,0x4
                    {
                        let r = fl.shl(edx, 4, 32);
                        edx = r;
                    }
                    // 00e4d3bf: MOV dword ptr [ESP + 0x14],0x1
                    e.mem.set_u32(f0.wrapping_add(0x8), 0x00000001u32);
                    // 00e4d3c7: MOVAPS xmmword ptr [ESP + 0x70],XMM0
                    store_vec4(e, f0.wrapping_add(0x64), x[0]);
                    // 00e4d3cc: MOVAPS xmmword ptr [EDX + ESI*0x1],XMM1
                    store_vec4(e, edx.wrapping_add(esi), x[1]);
                    // 00e4d3d0: ADD ECX,0xe
                    {
                        let r = fl.add(ecx, 0x0000000eu32, 32);
                        ecx = r;
                    }
                    // 00e4d3d3: JMP 0x00e4d692
                    pc = 0x00e4d692;
                }
                0x00e4d3d8 => {
                    // 00e4d3d8: MOV EDX,dword ptr [ESI + 0xc4]
                    edx = e.mem.u32(esi.wrapping_add(0xc4));
                    // 00e4d3de: MOV dword ptr [ESP + 0x140],EAX
                    e.mem.set_u32(f0.wrapping_add(0x134), eax);
                    // 00e4d3e5: MOV EAX,dword ptr [ESI + 0xc8]
                    eax = e.mem.u32(esi.wrapping_add(0xc8));
                    // 00e4d3eb: MOV dword ptr [ESP + 0x144],EDX
                    e.mem.set_u32(f0.wrapping_add(0x138), edx);
                    // 00e4d3f2: MOV EDX,dword ptr [ESI + 0xcc]
                    edx = e.mem.u32(esi.wrapping_add(0xcc));
                    // 00e4d3f8: MOV dword ptr [ESP + 0x148],EAX
                    e.mem.set_u32(f0.wrapping_add(0x13c), eax);
                    // 00e4d3ff: MOV EAX,dword ptr [ESI + 0x100]
                    eax = e.mem.u32(esi.wrapping_add(0x100));
                    // 00e4d405: MOV dword ptr [ESP + 0x14c],EDX
                    e.mem.set_u32(f0.wrapping_add(0x140), edx);
                    // 00e4d40c: MOV EDX,dword ptr [ESI + 0x104]
                    edx = e.mem.u32(esi.wrapping_add(0x104));
                    // 00e4d412: MOVAPS XMM0,xmmword ptr [ESP + 0x140]
                    x[0] = load_vec4(e, f0.wrapping_add(0x134));
                    // 00e4d41a: MOV dword ptr [ESP + 0x130],EAX
                    e.mem.set_u32(f0.wrapping_add(0x124), eax);
                    // 00e4d421: MOV EAX,dword ptr [ESI + 0x108]
                    eax = e.mem.u32(esi.wrapping_add(0x108));
                    // 00e4d427: MOV dword ptr [ESP + 0x134],EDX
                    e.mem.set_u32(f0.wrapping_add(0x128), edx);
                    // 00e4d42e: MOV EDX,dword ptr [ESI + 0x10c]
                    edx = e.mem.u32(esi.wrapping_add(0x10c));
                    // 00e4d434: MOV dword ptr [ESP + 0x138],EAX
                    e.mem.set_u32(f0.wrapping_add(0x12c), eax);
                    // 00e4d43b: MOV EAX,dword ptr [ESP + 0x50]
                    eax = e.mem.u32(f0.wrapping_add(0x44));
                    // 00e4d43f: ADD ECX,0xe
                    {
                        let r = fl.add(ecx, 0x0000000eu32, 32);
                        ecx = r;
                    }
                    // 00e4d442: MOV dword ptr [ESP + 0x13c],EDX
                    e.mem.set_u32(f0.wrapping_add(0x130), edx);
                    // 00e4d449: MOVAPS xmmword ptr [EAX],XMM0
                    store_vec4(e, eax, x[0]);
                    // 00e4d44c: MOVAPS XMM0,xmmword ptr [ESP + 0x130]
                    x[0] = load_vec4(e, f0.wrapping_add(0x124));
                    // 00e4d454: SHL ECX,0x4
                    {
                        let r = fl.shl(ecx, 4, 32);
                        ecx = r;
                    }
                    // 00e4d457: MOVAPS xmmword ptr [ECX + ESI*0x1],XMM0
                    store_vec4(e, ecx.wrapping_add(esi), x[0]);
                    pc = 0x00e4d45b;
                }
                0x00e4d45b => {
                    // 00e4d45b: MOVAPS XMM5,xmmword ptr [EBX + 0x10]
                    x[5] = load_vec4(e, ebx.wrapping_add(0x10));
                    // 00e4d45f: MOVAPS XMM4,xmmword ptr [EBX]
                    x[4] = load_vec4(e, ebx);
                    // 00e4d462: MOVAPS XMM1,xmmword ptr [EDI]
                    x[1] = load_vec4(e, edi);
                    // 00e4d465: MOV ECX,dword ptr [ESP + 0x14]
                    ecx = e.mem.u32(f0.wrapping_add(0x8));
                    // 00e4d469: MOVAPS XMM3,XMM5
                    x[3] = x[5];
                    // 00e4d46c: SUBPS XMM3,XMM4
                    x[3] = sub4(x[3], x[4]);
                    // 00e4d46f: SUBPS XMM4,XMM1
                    x[4] = sub4(x[4], x[1]);
                    // 00e4d472: MOVAPS XMM0,XMM5
                    x[0] = x[5];
                    // 00e4d475: SUBPS XMM0,XMM1
                    x[0] = sub4(x[0], x[1]);
                    // 00e4d478: MOVAPS XMM1,XMM0
                    x[1] = x[0];
                    // 00e4d47b: MULPS XMM1,XMM3
                    x[1] = mul4(x[1], x[3]);
                    // 00e4d47e: MOVAPS XMM6,XMM1
                    x[6] = x[1];
                    // 00e4d481: SHUFPS XMM6,XMM1,0x55
                    x[6] = shufps(x[6], x[1], 0x55);
                    // 00e4d485: ADDSS XMM6,XMM1
                    x[6][0] = x[6][0] + x[1][0];
                    // 00e4d489: MOVAPS XMM7,XMM1
                    x[7] = x[1];
                    // 00e4d48c: SHUFPS XMM7,XMM1,0xaa
                    x[7] = shufps(x[7], x[1], 0xaa);
                    // 00e4d490: ADDSS XMM7,XMM6
                    x[7][0] = x[7][0] + x[6][0];
                    // 00e4d494: MOVAPS XMM2,XMM4
                    x[2] = x[4];
                    // 00e4d497: MULPS XMM2,XMM3
                    x[2] = mul4(x[2], x[3]);
                    // 00e4d49a: MOVAPS XMM1,XMM2
                    x[1] = x[2];
                    // 00e4d49d: SHUFPS XMM1,XMM2,0x55
                    x[1] = shufps(x[1], x[2], 0x55);
                    // 00e4d4a1: MOVAPS XMM6,XMM2
                    x[6] = x[2];
                    // 00e4d4a4: ADDSS XMM1,XMM2
                    x[1][0] = x[1][0] + x[2][0];
                    // 00e4d4a8: SHUFPS XMM6,XMM2,0xaa
                    x[6] = shufps(x[6], x[2], 0xaa);
                    // 00e4d4ac: ADDSS XMM6,XMM1
                    x[6][0] = x[6][0] + x[1][0];
                    // 00e4d4b0: MOVMSKPS EAX,XMM7
                    eax = movmskps(x[7]);
                    // 00e4d4b3: MOVMSKPS EDX,XMM6
                    edx = movmskps(x[6]);
                    // 00e4d4b6: AND EAX,0x1
                    {
                        let r = fl.and(eax, 0x00000001u32, 32);
                        eax = r;
                    }
                    // 00e4d4b9: AND EDX,0x1
                    {
                        let r = fl.and(edx, 0x00000001u32, 32);
                        edx = r;
                    }
                    // 00e4d4bc: CMP EAX,EDX
                    fl.sub(eax, edx, 32);
                    // 00e4d4be: JZ 0x00e4d549
                    if fl.zf {
                        pc = 0x00e4d549;
                        continue;
                    }
                    // 00e4d4c4: MOVAPS XMM1,XMM3
                    x[1] = x[3];
                    // 00e4d4c7: MULPS XMM1,XMM3
                    x[1] = mul4(x[1], x[3]);
                    // 00e4d4ca: MOVAPS XMM2,XMM1
                    x[2] = x[1];
                    // 00e4d4cd: SHUFPS XMM2,XMM1,0x55
                    x[2] = shufps(x[2], x[1], 0x55);
                    // 00e4d4d1: ADDSS XMM2,XMM1
                    x[2][0] = x[2][0] + x[1][0];
                    // 00e4d4d5: MOVAPS XMM5,XMM1
                    x[5] = x[1];
                    // 00e4d4d8: SHUFPS XMM5,XMM1,0xaa
                    x[5] = shufps(x[5], x[1], 0xaa);
                    // 00e4d4dc: ADDSS XMM5,XMM2
                    x[5][0] = x[5][0] + x[2][0];
                    // 00e4d4e0: RSQRTSS XMM5,XMM5
                    x[5][0] = rsqrt_estimate(x[5][0]);
                    // 00e4d4e4: MOVAPS XMM2,XMM0
                    x[2] = x[0];
                    // 00e4d4e7: SHUFPS XMM2,XMM0,0xd2
                    x[2] = shufps(x[2], x[0], 0xd2);
                    // 00e4d4eb: MOVAPS XMM1,XMM5
                    x[1] = x[5];
                    // 00e4d4ee: SHUFPS XMM1,XMM1,0x0
                    x[1] = shufps(x[1], x[1], 0x0);
                    // 00e4d4f2: MULPS XMM1,XMM3
                    x[1] = mul4(x[1], x[3]);
                    // 00e4d4f5: MOVAPS XMM3,XMM4
                    x[3] = x[4];
                    // 00e4d4f8: SHUFPS XMM3,XMM4,0xc9
                    x[3] = shufps(x[3], x[4], 0xc9);
                    // 00e4d4fc: MULPS XMM3,XMM2
                    x[3] = mul4(x[3], x[2]);
                    // 00e4d4ff: SHUFPS XMM0,XMM0,0xc9
                    x[0] = shufps(x[0], x[0], 0xc9);
                    // 00e4d503: MOVAPS XMM2,XMM4
                    x[2] = x[4];
                    // 00e4d506: SHUFPS XMM2,XMM4,0xd2
                    x[2] = shufps(x[2], x[4], 0xd2);
                    // 00e4d50a: MULPS XMM2,XMM0
                    x[2] = mul4(x[2], x[0]);
                    // 00e4d50d: SUBPS XMM2,XMM3
                    x[2] = sub4(x[2], x[3]);
                    // 00e4d510: MOVAPS XMM3,XMM1
                    x[3] = x[1];
                    // 00e4d513: SHUFPS XMM3,XMM1,0xd2
                    x[3] = shufps(x[3], x[1], 0xd2);
                    // 00e4d517: MOVAPS XMM0,XMM2
                    x[0] = x[2];
                    // 00e4d51a: SHUFPS XMM0,XMM2,0xc9
                    x[0] = shufps(x[0], x[2], 0xc9);
                    // 00e4d51e: MULPS XMM0,XMM3
                    x[0] = mul4(x[0], x[3]);
                    // 00e4d521: MOVAPS XMM3,XMM2
                    x[3] = x[2];
                    // 00e4d524: SHUFPS XMM3,XMM2,0xd2
                    x[3] = shufps(x[3], x[2], 0xd2);
                    // 00e4d528: SHUFPS XMM1,XMM1,0xc9
                    x[1] = shufps(x[1], x[1], 0xc9);
                    // 00e4d52c: MULPS XMM3,XMM1
                    x[3] = mul4(x[3], x[1]);
                    // 00e4d52f: MOV EAX,ECX
                    eax = ecx;
                    // 00e4d531: SUBPS XMM3,XMM0
                    x[3] = sub4(x[3], x[0]);
                    // 00e4d534: MOVAPS xmmword ptr [ESI + 0x120],XMM3
                    store_vec4(e, esi.wrapping_add(0x120), x[3]);
                    // 00e4d53b: MOV dword ptr [ESI + 0x10],EAX
                    e.mem.set_u32(esi.wrapping_add(0x10), eax);
                    // 00e4d53e: MOV dword ptr [ESP + 0x14],ECX
                    e.mem.set_u32(f0.wrapping_add(0x8), ecx);
                    // 00e4d542: XOR EAX,EAX
                    {
                        let r = fl.xor(eax, eax, 32);
                        eax = r;
                    }
                    // 00e4d544: JMP 0x00e4cd45
                    pc = 0x00e4cd45;
                }
                0x00e4d549 => {
                    // 00e4d549: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e4d54b: JZ 0x00e4d558
                    if fl.zf {
                        pc = 0x00e4d558;
                        continue;
                    }
                    // 00e4d54d: MOVAPS XMM0,xmmword ptr [EBX + 0x50]
                    x[0] = load_vec4(e, ebx.wrapping_add(0x50));
                    // 00e4d551: MOVAPS xmmword ptr [EBX],XMM5
                    store_vec4(e, ebx, x[5]);
                    // 00e4d554: MOVAPS xmmword ptr [EBX + 0x40],XMM0
                    store_vec4(e, ebx.wrapping_add(0x40), x[0]);
                    pc = 0x00e4d558;
                }
                0x00e4d558 => {
                    // 00e4d558: MOVAPS XMM0,xmmword ptr [EDI]
                    x[0] = load_vec4(e, edi);
                    // 00e4d55b: MOVAPS XMM1,xmmword ptr [EBX]
                    x[1] = load_vec4(e, ebx);
                    // 00e4d55e: MOV ECX,0x1
                    ecx = 0x00000001u32;
                    // 00e4d563: MOV EAX,ECX
                    eax = ecx;
                    // 00e4d565: SUBPS XMM0,XMM1
                    x[0] = sub4(x[0], x[1]);
                    // 00e4d568: MOVAPS xmmword ptr [ESI + 0x120],XMM0
                    store_vec4(e, esi.wrapping_add(0x120), x[0]);
                    // 00e4d56f: MOV dword ptr [ESI + 0x10],EAX
                    e.mem.set_u32(esi.wrapping_add(0x10), eax);
                    // 00e4d572: MOV dword ptr [ESP + 0x14],ECX
                    e.mem.set_u32(f0.wrapping_add(0x8), ecx);
                    // 00e4d576: XOR EAX,EAX
                    {
                        let r = fl.xor(eax, eax, 32);
                        eax = r;
                    }
                    // 00e4d578: JMP 0x00e4cd45
                    pc = 0x00e4cd45;
                }
                0x00e4d57d => {
                    // 00e4d57d: FLD float ptr [0x01012054]
                    x87.fld_f32(f32::from_bits(e.mem.u32(0x01012054)));
                    // 00e4d583: PUSH ECX
                    stk.push(ecx);
                    // 00e4d584: FSTP float ptr [ESP]
                    *stk.last_mut().unwrap() = x87.fstp_f32().to_bits();
                    // 00e4d587: PUSH EDI
                    stk.push(edi);
                    // 00e4d588: PUSH EBX
                    stk.push(ebx);
                    // 00e4d589: CALL 0x00e4b810
                    {
                        let w: Vec<u32> = stk.iter().rev().take(3).copied().collect();
                        let r = find_closest_triangle_ov2(e, w[0], w[1], f32::from_bits(w[2]));
                        eax = r as u32;
                    }
                    // 00e4d58e: ADD ESP,0xc
                    stk.truncate(stk.len() - 3);
                    // 00e4d591: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e4d593: JL 0x00e4cd40
                    if fl.sf != fl.of {
                        pc = 0x00e4cd40;
                        continue;
                    }
                    // 00e4d599: DEC dword ptr [ESP + 0x18]
                    {
                        let r = fl.dec(e.mem.u32(f0.wrapping_add(0xc)), 32);
                        e.mem.set_u32(f0.wrapping_add(0xc), r);
                    }
                    // 00e4d59d: MOVAPS XMM0,xmmword ptr [ESI + 0x50]
                    x[0] = load_vec4(e, esi.wrapping_add(0x50));
                    // 00e4d5a1: ADD EAX,0x2
                    {
                        let r = fl.add(eax, 0x00000002u32, 32);
                        eax = r;
                    }
                    // 00e4d5a4: SHL EAX,0x4
                    {
                        let r = fl.shl(eax, 4, 32);
                        eax = r;
                    }
                    // 00e4d5a7: MOVAPS xmmword ptr [EAX + ESI*0x1],XMM0
                    store_vec4(e, eax.wrapping_add(esi), x[0]);
                    pc = 0x00e4d5ab;
                }
                0x00e4d5ab => {
                    // 00e4d5ab: LEA EAX,[ESI + 0x120]
                    eax = esi.wrapping_add(0x120);
                    // 00e4d5b1: PUSH EAX
                    stk.push(eax);
                    // 00e4d5b2: PUSH EDI
                    stk.push(edi);
                    // 00e4d5b3: PUSH EBX
                    stk.push(ebx);
                    // 00e4d5b4: MOV ECX,ESI
                    ecx = esi;
                    // 00e4d5b6: CALL 0x00e4da90
                    {
                        let mut w = vec![ecx];
                        w.extend(stk.iter().rev().take(3));
                        let r = hk_gsk_check_triangle_boundaries_1(e, w[0], w[1], w[2], w[3]);
                        eax = r;
                    }
                    stk.truncate(stk.len() - 3);
                    // 00e4d5bb: CMP EAX,0x7
                    fl.sub(eax, 0x00000007u32, 32);
                    // 00e4d5be: JZ 0x00e4d7d6
                    if fl.zf {
                        pc = 0x00e4d7d6;
                        continue;
                    }
                    // 00e4d5c4: MOVSX EAX,byte ptr [EAX + 0x10ececc]
                    eax = (u32::from(e.mem.u8(eax.wrapping_add(0x10ececc))) as i8 as i32 as u32);
                    // 00e4d5cb: MOV dword ptr [ESP + 0x18],0x2
                    e.mem.set_u32(f0.wrapping_add(0xc), 0x00000002u32);
                    // 00e4d5d3: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e4d5d5: JGE 0x00e4d5ed
                    if fl.sf == fl.of {
                        pc = 0x00e4d5ed;
                        continue;
                    }
                    // 00e4d5d7: MOVAPS XMM0,xmmword ptr [ESI + 0x40]
                    x[0] = load_vec4(e, esi.wrapping_add(0x40));
                    // 00e4d5db: AND EAX,0x3
                    {
                        let r = fl.and(eax, 0x00000003u32, 32);
                        eax = r;
                    }
                    // 00e4d5de: ADD EAX,0x2
                    {
                        let r = fl.add(eax, 0x00000002u32, 32);
                        eax = r;
                    }
                    // 00e4d5e1: SHL EAX,0x4
                    {
                        let r = fl.shl(eax, 4, 32);
                        eax = r;
                    }
                    // 00e4d5e4: MOVAPS xmmword ptr [EAX + ESI*0x1],XMM0
                    store_vec4(e, eax.wrapping_add(esi), x[0]);
                    // 00e4d5e8: JMP 0x00e4d6be
                    pc = 0x00e4d6be;
                }
                0x00e4d5ed => {
                    // 00e4d5ed: MOVSX EDX,byte ptr [EAX + 0x10eced4]
                    edx = (u32::from(e.mem.u8(eax.wrapping_add(0x10eced4))) as i8 as i32 as u32);
                    // 00e4d5f4: MOVSX ECX,byte ptr [EAX + 0x10eced6]
                    ecx = (u32::from(e.mem.u8(eax.wrapping_add(0x10eced6))) as i8 as i32 as u32);
                    // 00e4d5fb: MOVAPS XMM2,xmmword ptr [EBX]
                    x[2] = load_vec4(e, ebx);
                    // 00e4d5fe: ADD EAX,0x2
                    {
                        let r = fl.add(eax, 0x00000002u32, 32);
                        eax = r;
                    }
                    // 00e4d601: SHL EAX,0x4
                    {
                        let r = fl.shl(eax, 4, 32);
                        eax = r;
                    }
                    // 00e4d604: MOVAPS XMM3,xmmword ptr [EAX + ESI*0x1]
                    x[3] = load_vec4(e, eax.wrapping_add(esi));
                    // 00e4d608: MOV dword ptr [ESP + 0x50],EDX
                    e.mem.set_u32(f0.wrapping_add(0x44), edx);
                    // 00e4d60c: LEA EAX,[ECX + 0x2]
                    eax = ecx.wrapping_add(0x2);
                    // 00e4d60f: SHL EAX,0x4
                    {
                        let r = fl.shl(eax, 4, 32);
                        eax = r;
                    }
                    // 00e4d612: MOVAPS XMM1,xmmword ptr [EAX + ESI*0x1]
                    x[1] = load_vec4(e, eax.wrapping_add(esi));
                    // 00e4d616: ADD EAX,ESI
                    {
                        let r = fl.add(eax, esi, 32);
                        eax = r;
                    }
                    // 00e4d618: ADD EDX,0x2
                    {
                        let r = fl.add(edx, 0x00000002u32, 32);
                        edx = r;
                    }
                    // 00e4d61b: SHL EDX,0x4
                    {
                        let r = fl.shl(edx, 4, 32);
                        edx = r;
                    }
                    // 00e4d61e: MOVAPS XMM0,xmmword ptr [EDX + ESI*0x1]
                    x[0] = load_vec4(e, edx.wrapping_add(esi));
                    // 00e4d622: MOV dword ptr [ESP + 0x30],EAX
                    e.mem.set_u32(f0.wrapping_add(0x24), eax);
                    // 00e4d626: LEA EAX,[EDX + ESI*0x1]
                    eax = edx.wrapping_add(esi);
                    // 00e4d629: SUBPS XMM0,XMM3
                    x[0] = sub4(x[0], x[3]);
                    // 00e4d62c: SUBPS XMM2,XMM3
                    x[2] = sub4(x[2], x[3]);
                    // 00e4d62f: MULPS XMM0,XMM2
                    x[0] = mul4(x[0], x[2]);
                    // 00e4d632: SUBPS XMM1,XMM3
                    x[1] = sub4(x[1], x[3]);
                    // 00e4d635: MOVAPS XMM3,XMM0
                    x[3] = x[0];
                    // 00e4d638: SHUFPS XMM3,XMM0,0x55
                    x[3] = shufps(x[3], x[0], 0x55);
                    // 00e4d63c: MOVAPS XMM4,XMM0
                    x[4] = x[0];
                    // 00e4d63f: ADDSS XMM3,XMM0
                    x[3][0] = x[3][0] + x[0][0];
                    // 00e4d643: SHUFPS XMM4,XMM0,0xaa
                    x[4] = shufps(x[4], x[0], 0xaa);
                    // 00e4d647: MOVAPS XMM0,xmmword ptr [ESI + 0x40]
                    x[0] = load_vec4(e, esi.wrapping_add(0x40));
                    // 00e4d64b: ADDSS XMM4,XMM3
                    x[4][0] = x[4][0] + x[3][0];
                    // 00e4d64f: MOVMSKPS EDX,XMM4
                    edx = movmskps(x[4]);
                    // 00e4d652: TEST DL,0x1
                    fl.and((edx & 0xff), 0x00000001u32, 8);
                    // 00e4d655: JZ 0x00e4d6b7
                    if fl.zf {
                        pc = 0x00e4d6b7;
                        continue;
                    }
                    // 00e4d657: MULPS XMM1,XMM2
                    x[1] = mul4(x[1], x[2]);
                    // 00e4d65a: MOVAPS xmmword ptr [EAX],XMM0
                    store_vec4(e, eax, x[0]);
                    // 00e4d65d: MOVAPS XMM0,XMM1
                    x[0] = x[1];
                    // 00e4d660: SHUFPS XMM0,XMM1,0x55
                    x[0] = shufps(x[0], x[1], 0x55);
                    // 00e4d664: MOVAPS XMM2,XMM1
                    x[2] = x[1];
                    // 00e4d667: ADDSS XMM0,XMM1
                    x[0][0] = x[0][0] + x[1][0];
                    // 00e4d66b: SHUFPS XMM2,XMM1,0xaa
                    x[2] = shufps(x[2], x[1], 0xaa);
                    // 00e4d66f: ADDSS XMM2,XMM0
                    x[2][0] = x[2][0] + x[0][0];
                    // 00e4d673: MOVMSKPS EAX,XMM2
                    eax = movmskps(x[2]);
                    // 00e4d676: TEST AL,0x1
                    fl.and((eax & 0xff), 0x00000001u32, 8);
                    // 00e4d678: JZ 0x00e4d6be
                    if fl.zf {
                        pc = 0x00e4d6be;
                        continue;
                    }
                    // 00e4d67a: CMP ECX,0x2
                    fl.sub(ecx, 0x00000002u32, 32);
                    // 00e4d67d: JNZ 0x00e4d683
                    if !fl.zf {
                        pc = 0x00e4d683;
                        continue;
                    }
                    // 00e4d67f: MOV ECX,dword ptr [ESP + 0x50]
                    ecx = e.mem.u32(f0.wrapping_add(0x44));
                    pc = 0x00e4d683;
                }
                0x00e4d683 => {
                    // 00e4d683: MOVAPS XMM0,xmmword ptr [ESI + 0x30]
                    x[0] = load_vec4(e, esi.wrapping_add(0x30));
                    // 00e4d687: MOV dword ptr [ESP + 0x18],0x1
                    e.mem.set_u32(f0.wrapping_add(0xc), 0x00000001u32);
                    // 00e4d68f: ADD ECX,0x2
                    {
                        let r = fl.add(ecx, 0x00000002u32, 32);
                        ecx = r;
                    }
                    pc = 0x00e4d692;
                }
                0x00e4d692 => {
                    // 00e4d692: SHL ECX,0x4
                    {
                        let r = fl.shl(ecx, 4, 32);
                        ecx = r;
                    }
                    // 00e4d695: MOVAPS xmmword ptr [ECX + ESI*0x1],XMM0
                    store_vec4(e, ecx.wrapping_add(esi), x[0]);
                    pc = 0x00e4d699;
                }
                0x00e4d699 => {
                    // 00e4d699: MOV EAX,dword ptr [ESP + 0x14]
                    eax = e.mem.u32(f0.wrapping_add(0x8));
                    // 00e4d69d: MOVAPS XMM0,xmmword ptr [EDI]
                    x[0] = load_vec4(e, edi);
                    // 00e4d6a0: MOVAPS XMM1,xmmword ptr [EBX]
                    x[1] = load_vec4(e, ebx);
                    // 00e4d6a3: SUBPS XMM0,XMM1
                    x[0] = sub4(x[0], x[1]);
                    // 00e4d6a6: MOV dword ptr [ESI + 0x10],EAX
                    e.mem.set_u32(esi.wrapping_add(0x10), eax);
                    // 00e4d6a9: MOVAPS xmmword ptr [ESI + 0x120],XMM0
                    store_vec4(e, esi.wrapping_add(0x120), x[0]);
                    // 00e4d6b0: XOR EAX,EAX
                    {
                        let r = fl.xor(eax, eax, 32);
                        eax = r;
                    }
                    // 00e4d6b2: JMP 0x00e4cd45
                    pc = 0x00e4cd45;
                }
                0x00e4d6b7 => {
                    // 00e4d6b7: MOV ECX,dword ptr [ESP + 0x30]
                    ecx = e.mem.u32(f0.wrapping_add(0x24));
                    // 00e4d6bb: MOVAPS xmmword ptr [ECX],XMM0
                    store_vec4(e, ecx, x[0]);
                    pc = 0x00e4d6be;
                }
                0x00e4d6be => {
                    // 00e4d6be: MOVAPS XMM5,xmmword ptr [EDI + 0x10]
                    x[5] = load_vec4(e, edi.wrapping_add(0x10));
                    // 00e4d6c2: MOVAPS XMM4,xmmword ptr [EDI]
                    x[4] = load_vec4(e, edi);
                    // 00e4d6c5: MOVAPS XMM1,xmmword ptr [EBX]
                    x[1] = load_vec4(e, ebx);
                    // 00e4d6c8: MOV ECX,dword ptr [ESP + 0x18]
                    ecx = e.mem.u32(f0.wrapping_add(0xc));
                    // 00e4d6cc: MOVAPS XMM3,XMM5
                    x[3] = x[5];
                    // 00e4d6cf: SUBPS XMM3,XMM4
                    x[3] = sub4(x[3], x[4]);
                    // 00e4d6d2: SUBPS XMM4,XMM1
                    x[4] = sub4(x[4], x[1]);
                    // 00e4d6d5: MOVAPS XMM0,XMM5
                    x[0] = x[5];
                    // 00e4d6d8: SUBPS XMM0,XMM1
                    x[0] = sub4(x[0], x[1]);
                    // 00e4d6db: MOVAPS XMM1,XMM0
                    x[1] = x[0];
                    // 00e4d6de: MULPS XMM1,XMM3
                    x[1] = mul4(x[1], x[3]);
                    // 00e4d6e1: MOVAPS XMM6,XMM1
                    x[6] = x[1];
                    // 00e4d6e4: SHUFPS XMM6,XMM1,0x55
                    x[6] = shufps(x[6], x[1], 0x55);
                    // 00e4d6e8: ADDSS XMM6,XMM1
                    x[6][0] = x[6][0] + x[1][0];
                    // 00e4d6ec: MOVAPS XMM7,XMM1
                    x[7] = x[1];
                    // 00e4d6ef: SHUFPS XMM7,XMM1,0xaa
                    x[7] = shufps(x[7], x[1], 0xaa);
                    // 00e4d6f3: ADDSS XMM7,XMM6
                    x[7][0] = x[7][0] + x[6][0];
                    // 00e4d6f7: MOVAPS XMM2,XMM4
                    x[2] = x[4];
                    // 00e4d6fa: MULPS XMM2,XMM3
                    x[2] = mul4(x[2], x[3]);
                    // 00e4d6fd: MOVAPS XMM1,XMM2
                    x[1] = x[2];
                    // 00e4d700: SHUFPS XMM1,XMM2,0x55
                    x[1] = shufps(x[1], x[2], 0x55);
                    // 00e4d704: MOVAPS XMM6,XMM2
                    x[6] = x[2];
                    // 00e4d707: ADDSS XMM1,XMM2
                    x[1][0] = x[1][0] + x[2][0];
                    // 00e4d70b: SHUFPS XMM6,XMM2,0xaa
                    x[6] = shufps(x[6], x[2], 0xaa);
                    // 00e4d70f: ADDSS XMM6,XMM1
                    x[6][0] = x[6][0] + x[1][0];
                    // 00e4d713: MOVMSKPS EAX,XMM7
                    eax = movmskps(x[7]);
                    // 00e4d716: MOVMSKPS EDX,XMM6
                    edx = movmskps(x[6]);
                    // 00e4d719: AND EAX,0x1
                    {
                        let r = fl.and(eax, 0x00000001u32, 32);
                        eax = r;
                    }
                    // 00e4d71c: AND EDX,0x1
                    {
                        let r = fl.and(edx, 0x00000001u32, 32);
                        edx = r;
                    }
                    // 00e4d71f: CMP EAX,EDX
                    fl.sub(eax, edx, 32);
                    // 00e4d721: JZ 0x00e4d7ae
                    if fl.zf {
                        pc = 0x00e4d7ae;
                        continue;
                    }
                    // 00e4d727: MOV EAX,dword ptr [ESP + 0x14]
                    eax = e.mem.u32(f0.wrapping_add(0x8));
                    // 00e4d72b: MOVAPS XMM1,XMM3
                    x[1] = x[3];
                    // 00e4d72e: MULPS XMM1,XMM3
                    x[1] = mul4(x[1], x[3]);
                    // 00e4d731: MOVAPS XMM2,XMM1
                    x[2] = x[1];
                    // 00e4d734: SHUFPS XMM2,XMM1,0x55
                    x[2] = shufps(x[2], x[1], 0x55);
                    // 00e4d738: ADDSS XMM2,XMM1
                    x[2][0] = x[2][0] + x[1][0];
                    // 00e4d73c: MOVAPS XMM5,XMM1
                    x[5] = x[1];
                    // 00e4d73f: SHUFPS XMM5,XMM1,0xaa
                    x[5] = shufps(x[5], x[1], 0xaa);
                    // 00e4d743: ADDSS XMM5,XMM2
                    x[5][0] = x[5][0] + x[2][0];
                    // 00e4d747: MOVAPS XMM2,XMM0
                    x[2] = x[0];
                    // 00e4d74a: SHUFPS XMM2,XMM0,0xd2
                    x[2] = shufps(x[2], x[0], 0xd2);
                    // 00e4d74e: RSQRTSS XMM5,XMM5
                    x[5][0] = rsqrt_estimate(x[5][0]);
                    // 00e4d752: MOVAPS XMM1,XMM5
                    x[1] = x[5];
                    // 00e4d755: SHUFPS XMM1,XMM1,0x0
                    x[1] = shufps(x[1], x[1], 0x0);
                    // 00e4d759: MULPS XMM1,XMM3
                    x[1] = mul4(x[1], x[3]);
                    // 00e4d75c: MOVAPS XMM3,XMM4
                    x[3] = x[4];
                    // 00e4d75f: SHUFPS XMM3,XMM4,0xc9
                    x[3] = shufps(x[3], x[4], 0xc9);
                    // 00e4d763: MULPS XMM3,XMM2
                    x[3] = mul4(x[3], x[2]);
                    // 00e4d766: SHUFPS XMM0,XMM0,0xc9
                    x[0] = shufps(x[0], x[0], 0xc9);
                    // 00e4d76a: MOVAPS XMM2,XMM4
                    x[2] = x[4];
                    // 00e4d76d: SHUFPS XMM2,XMM4,0xd2
                    x[2] = shufps(x[2], x[4], 0xd2);
                    // 00e4d771: MULPS XMM2,XMM0
                    x[2] = mul4(x[2], x[0]);
                    // 00e4d774: SUBPS XMM2,XMM3
                    x[2] = sub4(x[2], x[3]);
                    // 00e4d777: MOVAPS XMM3,XMM1
                    x[3] = x[1];
                    // 00e4d77a: SHUFPS XMM3,XMM1,0xd2
                    x[3] = shufps(x[3], x[1], 0xd2);
                    // 00e4d77e: MOVAPS XMM0,XMM2
                    x[0] = x[2];
                    // 00e4d781: SHUFPS XMM0,XMM2,0xc9
                    x[0] = shufps(x[0], x[2], 0xc9);
                    // 00e4d785: MULPS XMM0,XMM3
                    x[0] = mul4(x[0], x[3]);
                    // 00e4d788: MOVAPS XMM3,XMM2
                    x[3] = x[2];
                    // 00e4d78b: SHUFPS XMM3,XMM2,0xd2
                    x[3] = shufps(x[3], x[2], 0xd2);
                    // 00e4d78f: SHUFPS XMM1,XMM1,0xc9
                    x[1] = shufps(x[1], x[1], 0xc9);
                    // 00e4d793: MULPS XMM3,XMM1
                    x[3] = mul4(x[3], x[1]);
                    // 00e4d796: SUBPS XMM0,XMM3
                    x[0] = sub4(x[0], x[3]);
                    // 00e4d799: MOVAPS xmmword ptr [ESI + 0x120],XMM0
                    store_vec4(e, esi.wrapping_add(0x120), x[0]);
                    // 00e4d7a0: MOV dword ptr [ESI + 0x10],EAX
                    e.mem.set_u32(esi.wrapping_add(0x10), eax);
                    // 00e4d7a3: MOV dword ptr [ESP + 0x18],ECX
                    e.mem.set_u32(f0.wrapping_add(0xc), ecx);
                    // 00e4d7a7: XOR EAX,EAX
                    {
                        let r = fl.xor(eax, eax, 32);
                        eax = r;
                    }
                    // 00e4d7a9: JMP 0x00e4cd45
                    pc = 0x00e4cd45;
                }
                0x00e4d7ae => {
                    // 00e4d7ae: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e4d7b0: JZ 0x00e4d7bd
                    if fl.zf {
                        pc = 0x00e4d7bd;
                        continue;
                    }
                    // 00e4d7b2: MOVAPS XMM0,xmmword ptr [EDI + 0x50]
                    x[0] = load_vec4(e, edi.wrapping_add(0x50));
                    // 00e4d7b6: MOVAPS xmmword ptr [EDI],XMM5
                    store_vec4(e, edi, x[5]);
                    // 00e4d7b9: MOVAPS xmmword ptr [EDI + 0x40],XMM0
                    store_vec4(e, edi.wrapping_add(0x40), x[0]);
                    pc = 0x00e4d7bd;
                }
                0x00e4d7bd => {
                    // 00e4d7bd: MOVAPS XMM0,xmmword ptr [EBX]
                    x[0] = load_vec4(e, ebx);
                    // 00e4d7c0: MOVAPS XMM1,xmmword ptr [EDI]
                    x[1] = load_vec4(e, edi);
                    // 00e4d7c3: MOV ECX,0x1
                    ecx = 0x00000001u32;
                    // 00e4d7c8: SUBPS XMM1,XMM0
                    x[1] = sub4(x[1], x[0]);
                    // 00e4d7cb: MOVAPS xmmword ptr [ESI + 0x120],XMM1
                    store_vec4(e, esi.wrapping_add(0x120), x[1]);
                    // 00e4d7d2: MOV dword ptr [ESP + 0x18],ECX
                    e.mem.set_u32(f0.wrapping_add(0xc), ecx);
                    pc = 0x00e4d7d6;
                }
                0x00e4d7d6 => {
                    // 00e4d7d6: MOV EAX,dword ptr [ESP + 0x14]
                    eax = e.mem.u32(f0.wrapping_add(0x8));
                    // 00e4d7da: MOV dword ptr [ESI + 0x10],EAX
                    e.mem.set_u32(esi.wrapping_add(0x10), eax);
                    // 00e4d7dd: XOR EAX,EAX
                    {
                        let r = fl.xor(eax, eax, 32);
                        eax = r;
                    }
                    // 00e4d7df: JMP 0x00e4cd45
                    pc = 0x00e4cd45;
                }
                0x00e4d7e4 => {
                    // 00e4d7e4: MOVAPS XMM2,XMM1
                    x[2] = x[1];
                    // 00e4d7e7: SHUFPS XMM2,XMM1,0xaa
                    x[2] = shufps(x[2], x[1], 0xaa);
                    // 00e4d7eb: MOVSS dword ptr [ESP + 0x2c],XMM2
                    e.mem.set_f32(f0.wrapping_add(0x20), x[2][0]);
                    // 00e4d7f1: FLD float ptr [ESP + 0x2c]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0x20))));
                    // 00e4d7f5: SHUFPS XMM1,XMM1,0x55
                    x[1] = shufps(x[1], x[1], 0x55);
                    // 00e4d7f9: MOVSS dword ptr [ESP + 0x1c],XMM1
                    e.mem.set_f32(f0.wrapping_add(0x10), x[1][0]);
                    // 00e4d7ff: FLD float ptr [ESP + 0x1c]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0x10))));
                    // 00e4d803: FCOMPP
                    fsw = x87.fcom(1);
                    x87.pop();
                    x87.pop();
                    // 00e4d805: FNSTSW AX
                    eax = (eax & 0xffff_00ff) | (u32::from(fsw) << 8);
                    // 00e4d807: TEST AH,0x41
                    fl.and(((eax >> 8) & 0xff), 0x00000041u32, 8);
                    // 00e4d80a: JZ 0x00e4d837
                    if fl.zf {
                        pc = 0x00e4d837;
                        continue;
                    }
                    pc = 0x00e4d80c;
                }
                0x00e4d80c => {
                    // 00e4d80c: MOV EAX,dword ptr [ESP + 0x18]
                    eax = e.mem.u32(f0.wrapping_add(0xc));
                    // 00e4d810: MOVMSKPS ECX,XMM0
                    ecx = movmskps(x[0]);
                    // 00e4d813: TEST dword ptr [EAX*0x4 + 0x10ec600],ECX
                    fl.and(
                        e.mem.u32(eax.wrapping_mul(4).wrapping_add(0x10ec600)),
                        ecx,
                        32,
                    );
                    // 00e4d81a: JNZ 0x00e4d877
                    if !fl.zf {
                        pc = 0x00e4d877;
                        continue;
                    }
                    // 00e4d81c: LEA EDX,[EAX + 0x2]
                    edx = eax.wrapping_add(0x2);
                    // 00e4d81f: SHL EDX,0x4
                    {
                        let r = fl.shl(edx, 4, 32);
                        edx = r;
                    }
                    // 00e4d822: INC EAX
                    {
                        let r = fl.inc(eax, 32);
                        eax = r;
                    }
                    // 00e4d823: MOVAPS xmmword ptr [EDX + ESI*0x1],XMM6
                    store_vec4(e, edx.wrapping_add(esi), x[6]);
                    // 00e4d827: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(f0.wrapping_add(0xc), eax);
                    // 00e4d82b: MOV dword ptr [ESI + 0x1c],0x1
                    e.mem.set_u32(esi.wrapping_add(0x1c), 0x00000001u32);
                    // 00e4d832: JMP 0x00e4c990
                    pc = 0x00e4c990;
                }
                0x00e4d837 => {
                    // 00e4d837: MOVMSKPS EAX,XMM0
                    eax = movmskps(x[0]);
                    // 00e4d83a: TEST dword ptr [EDX*0x4 + 0x10ec5f0],EAX
                    fl.and(
                        e.mem.u32(edx.wrapping_mul(4).wrapping_add(0x10ec5f0)),
                        eax,
                        32,
                    );
                    // 00e4d841: JNZ 0x00e4d873
                    if !fl.zf {
                        pc = 0x00e4d873;
                        continue;
                    }
                    // 00e4d843: MOVAPS XMM0,xmmword ptr [ESP + 0xa0]
                    x[0] = load_vec4(e, f0.wrapping_add(0x94));
                    // 00e4d84b: LEA ECX,[EDX + 0xe]
                    ecx = edx.wrapping_add(0xe);
                    // 00e4d84e: SHL ECX,0x4
                    {
                        let r = fl.shl(ecx, 4, 32);
                        ecx = r;
                    }
                    // 00e4d851: INC EDX
                    {
                        let r = fl.inc(edx, 32);
                        edx = r;
                    }
                    // 00e4d852: MOVAPS xmmword ptr [ECX + ESI*0x1],XMM0
                    store_vec4(e, ecx.wrapping_add(esi), x[0]);
                    // 00e4d856: MOV dword ptr [ESP + 0x14],EDX
                    e.mem.set_u32(f0.wrapping_add(0x8), edx);
                    // 00e4d85a: MOV dword ptr [ESI + 0x1c],0x1
                    e.mem.set_u32(esi.wrapping_add(0x1c), 0x00000001u32);
                    // 00e4d861: JMP 0x00e4c990
                    pc = 0x00e4c990;
                }
                0x00e4d866 => {
                    // 00e4d866: MOV dword ptr [ESP + 0x20],0x3
                    e.mem.set_u32(f0.wrapping_add(0x14), 0x00000003u32);
                    // 00e4d86e: JMP 0x00e4d9b0
                    pc = 0x00e4d9b0;
                }
                0x00e4d873 => {
                    // 00e4d873: MOV EAX,dword ptr [ESP + 0x18]
                    eax = e.mem.u32(f0.wrapping_add(0xc));
                    pc = 0x00e4d877;
                }
                0x00e4d877 => {
                    // 00e4d877: MOVAPS XMM1,xmmword ptr [ESI + 0x120]
                    x[1] = load_vec4(e, esi.wrapping_add(0x120));
                    // 00e4d87e: MOVAPS XMM2,xmmword ptr [EBX]
                    x[2] = load_vec4(e, ebx);
                    // 00e4d881: MOVAPS XMM0,xmmword ptr [EDI]
                    x[0] = load_vec4(e, edi);
                    // 00e4d884: SUBPS XMM0,XMM2
                    x[0] = sub4(x[0], x[2]);
                    // 00e4d887: MULPS XMM0,XMM1
                    x[0] = mul4(x[0], x[1]);
                    // 00e4d88a: MOV ECX,dword ptr [EBP + 0x10]
                    ecx = output;
                    // 00e4d88d: MOVAPS XMM3,XMM0
                    x[3] = x[0];
                    // 00e4d890: SHUFPS XMM3,XMM0,0x55
                    x[3] = shufps(x[3], x[0], 0x55);
                    // 00e4d894: ADDSS XMM3,XMM0
                    x[3][0] = x[3][0] + x[0][0];
                    // 00e4d898: MOVAPS XMM2,XMM0
                    x[2] = x[0];
                    // 00e4d89b: SHUFPS XMM2,XMM0,0xaa
                    x[2] = shufps(x[2], x[0], 0xaa);
                    // 00e4d89f: ADDSS XMM2,XMM3
                    x[2][0] = x[2][0] + x[3][0];
                    // 00e4d8a3: MOVSS dword ptr [ESP + 0x1c],XMM2
                    e.mem.set_f32(f0.wrapping_add(0x10), x[2][0]);
                    // 00e4d8a9: FLD float ptr [ESP + 0x1c]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0x10))));
                    // 00e4d8ad: FSTP float ptr [ESP + 0x4c]
                    e.mem.set_f32(f0.wrapping_add(0x40), x87.fstp_f32());
                    // 00e4d8b1: MOVSS XMM4,dword ptr [ESP + 0x4c]
                    x[4] = [
                        f32::from_bits(e.mem.u32(f0.wrapping_add(0x40))),
                        0.0,
                        0.0,
                        0.0,
                    ];
                    // 00e4d8b7: MOVAPS XMM0,XMM1
                    x[0] = x[1];
                    // 00e4d8ba: SHUFPS XMM0,XMM1,0xaa
                    x[0] = shufps(x[0], x[1], 0xaa);
                    // 00e4d8be: MOVSS dword ptr [ESP + 0x1c],XMM0
                    e.mem.set_f32(f0.wrapping_add(0x10), x[0][0]);
                    // 00e4d8c4: FLD float ptr [ESP + 0x1c]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0x10))));
                    // 00e4d8c8: MOVAPS XMM0,XMM1
                    x[0] = x[1];
                    // 00e4d8cb: FSTP float ptr [ESP + 0x2c]
                    e.mem.set_f32(f0.wrapping_add(0x20), x87.fstp_f32());
                    // 00e4d8cf: MOVSS XMM3,dword ptr [ESP + 0x2c]
                    x[3] = [
                        f32::from_bits(e.mem.u32(f0.wrapping_add(0x20))),
                        0.0,
                        0.0,
                        0.0,
                    ];
                    // 00e4d8d5: SHUFPS XMM0,XMM1,0x55
                    x[0] = shufps(x[0], x[1], 0x55);
                    // 00e4d8d9: MOVSS dword ptr [ESP + 0x1c],XMM0
                    e.mem.set_f32(f0.wrapping_add(0x10), x[0][0]);
                    // 00e4d8df: FLD float ptr [ESP + 0x1c]
                    x87.fld_f32(f32::from_bits(e.mem.u32(f0.wrapping_add(0x10))));
                    // 00e4d8e3: MOVSS XMM0,XMM1
                    x[0][0] = x[1][0];
                    // 00e4d8e7: FSTP float ptr [ESP + 0x1c]
                    e.mem.set_f32(f0.wrapping_add(0x10), x87.fstp_f32());
                    // 00e4d8eb: MOVSS XMM1,dword ptr [ESP + 0x1c]
                    x[1] = [
                        f32::from_bits(e.mem.u32(f0.wrapping_add(0x10))),
                        0.0,
                        0.0,
                        0.0,
                    ];
                    // 00e4d8f1: UNPCKLPS XMM0,XMM3
                    x[0] = unpcklps(x[0], x[3]);
                    // 00e4d8f4: UNPCKLPS XMM1,XMM4
                    x[1] = unpcklps(x[1], x[4]);
                    // 00e4d8f7: UNPCKLPS XMM0,XMM1
                    x[0] = unpcklps(x[0], x[1]);
                    // 00e4d8fa: MOVAPS xmmword ptr [ECX],XMM0
                    store_vec4(e, ecx, x[0]);
                    // 00e4d8fd: MOVAPS xmmword ptr [ESI + 0x120],XMM0
                    store_vec4(e, esi.wrapping_add(0x120), x[0]);
                    // 00e4d904: CMP EAX,0x1
                    fl.sub(eax, 0x00000001u32, 32);
                    // 00e4d907: JNZ 0x00e4d917
                    if !fl.zf {
                        pc = 0x00e4d917;
                        continue;
                    }
                    // 00e4d909: MOVAPS XMM1,xmmword ptr [EDI]
                    x[1] = load_vec4(e, edi);
                    // 00e4d90c: MOV EDX,ECX
                    edx = ecx;
                    // 00e4d90e: MOVAPS xmmword ptr [EDX + 0x10],XMM1
                    store_vec4(e, edx.wrapping_add(0x10), x[1]);
                    // 00e4d912: JMP 0x00e4d9b0
                    pc = 0x00e4d9b0;
                }
                0x00e4d917 => {
                    // 00e4d917: CMP EDX,0x1
                    fl.sub(edx, 0x00000001u32, 32);
                    // 00e4d91a: JNZ 0x00e4d934
                    if !fl.zf {
                        pc = 0x00e4d934;
                        continue;
                    }
                    // 00e4d91c: MOVAPS XMM1,XMM2
                    x[1] = x[2];
                    // 00e4d91f: SHUFPS XMM1,XMM2,0x0
                    x[1] = shufps(x[1], x[2], 0x0);
                    // 00e4d923: MULPS XMM1,XMM0
                    x[1] = mul4(x[1], x[0]);
                    // 00e4d926: MOVAPS XMM0,xmmword ptr [EBX]
                    x[0] = load_vec4(e, ebx);
                    // 00e4d929: MOV EDX,ECX
                    edx = ecx;
                    // 00e4d92b: ADDPS XMM1,XMM0
                    x[1] = add4(x[1], x[0]);
                    // 00e4d92e: MOVAPS xmmword ptr [EDX + 0x10],XMM1
                    store_vec4(e, edx.wrapping_add(0x10), x[1]);
                    // 00e4d932: JMP 0x00e4d9b0
                    pc = 0x00e4d9b0;
                }
                0x00e4d934 => {
                    // 00e4d934: MOVAPS XMM1,xmmword ptr [ESI + 0x130]
                    x[1] = load_vec4(e, esi.wrapping_add(0x130));
                    // 00e4d93b: MOV EDX,ECX
                    edx = ecx;
                    // 00e4d93d: MOVAPS xmmword ptr [EDX + 0x10],XMM1
                    store_vec4(e, edx.wrapping_add(0x10), x[1]);
                    // 00e4d941: JMP 0x00e4d9b0
                    pc = 0x00e4d9b0;
                }
                0x00e4d943 => {
                    // 00e4d943: CMP dword ptr [ESI + 0x18],0x0
                    fl.sub(e.mem.u32(esi.wrapping_add(0x18)), 0x00000000u32, 32);
                    // 00e4d947: MOV EAX,0x1
                    eax = 0x00000001u32;
                    // 00e4d94c: MOV dword ptr [ESI + 0x1c],EAX
                    e.mem.set_u32(esi.wrapping_add(0x1c), eax);
                    // 00e4d94f: JNZ 0x00e4d97e
                    if !fl.zf {
                        pc = 0x00e4d97e;
                        continue;
                    }
                    // 00e4d951: MOV EAX,dword ptr [EBP + 0x10]
                    eax = output;
                    // 00e4d954: PUSH EAX
                    stk.push(eax);
                    // 00e4d955: MOV EAX,dword ptr [EBP + 0x8]
                    eax = input;
                    // 00e4d958: LEA ECX,[ESP + 0x18]
                    ecx = f0.wrapping_add(0x8);
                    // 00e4d95c: PUSH ECX
                    stk.push(ecx);
                    // 00e4d95d: MOV ECX,dword ptr [ESP + 0x2c]
                    ecx = e.mem.u32(f0.wrapping_add(0x18));
                    // 00e4d961: LEA EDX,[ESP + 0x20]
                    edx = f0.wrapping_add(0xc);
                    // 00e4d965: PUSH EDX
                    stk.push(edx);
                    // 00e4d966: MOV EDX,dword ptr [ESP + 0x34]
                    edx = e.mem.u32(f0.wrapping_add(0x1c));
                    // 00e4d96a: PUSH EAX
                    stk.push(eax);
                    // 00e4d96b: PUSH ECX
                    stk.push(ecx);
                    // 00e4d96c: PUSH EDX
                    stk.push(edx);
                    // 00e4d96d: MOV ECX,ESI
                    ecx = esi;
                    // 00e4d96f: CALL 0x00e4c230
                    {
                        let mut w = vec![ecx];
                        w.extend(stk.iter().rev().take(6));
                        let r =
                            hk_gsk_handle_penetration(e, w[0], w[1], w[2], w[3], w[4], w[5], w[6]);
                        eax = r;
                    }
                    stk.truncate(stk.len() - 6);
                    // 00e4d974: MOV dword ptr [ESP + 0x20],0x0
                    e.mem.set_u32(f0.wrapping_add(0x14), 0x00000000u32);
                    // 00e4d97c: JMP 0x00e4d9b0
                    pc = 0x00e4d9b0;
                }
                0x00e4d97e => {
                    // 00e4d97e: MOV EDX,dword ptr [ESP + 0x14]
                    edx = e.mem.u32(f0.wrapping_add(0x8));
                    // 00e4d982: LEA EDI,[EDX + ECX*0x1]
                    edi = edx.wrapping_add(ecx);
                    // 00e4d985: CMP EDI,0x4
                    fl.sub(edi, 0x00000004u32, 32);
                    // 00e4d988: JLE 0x00e4d9a8
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e4d9a8;
                        continue;
                    }
                    // 00e4d98a: CMP ECX,EDX
                    fl.sub(ecx, edx, 32);
                    // 00e4d98c: JLE 0x00e4d99c
                    if fl.zf || fl.sf != fl.of {
                        pc = 0x00e4d99c;
                        continue;
                    }
                    // 00e4d98e: MOV dword ptr [ESP + 0x18],0x3
                    e.mem.set_u32(f0.wrapping_add(0xc), 0x00000003u32);
                    // 00e4d996: MOV dword ptr [ESP + 0x14],EAX
                    e.mem.set_u32(f0.wrapping_add(0x8), eax);
                    // 00e4d99a: JMP 0x00e4d9a8
                    pc = 0x00e4d9a8;
                }
                0x00e4d99c => {
                    // 00e4d99c: MOV dword ptr [ESP + 0x18],EAX
                    e.mem.set_u32(f0.wrapping_add(0xc), eax);
                    // 00e4d9a0: MOV dword ptr [ESP + 0x14],0x3
                    e.mem.set_u32(f0.wrapping_add(0x8), 0x00000003u32);
                    pc = 0x00e4d9a8;
                }
                0x00e4d9a8 => {
                    // 00e4d9a8: MOV dword ptr [ESP + 0x20],0x4
                    e.mem.set_u32(f0.wrapping_add(0x14), 0x00000004u32);
                    pc = 0x00e4d9b0;
                }
                0x00e4d9b0 => {
                    // 00e4d9b0: MOV EAX,dword ptr [EBP + 0xc]
                    eax = ids;
                    // 00e4d9b3: TEST EAX,EAX
                    fl.and(eax, eax, 32);
                    // 00e4d9b5: JZ 0x00e4d9f9
                    if fl.zf {
                        pc = 0x00e4d9f9;
                        continue;
                    }
                    // 00e4d9b7: MOVZX ECX,byte ptr [ESI + 0x2c]
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x2c)));
                    // 00e4d9bb: MOVZX EDX,byte ptr [ESI + 0x3c]
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x3c)));
                    // 00e4d9bf: MOV byte ptr [EAX],CL
                    e.mem.set_u8(eax, (ecx & 0xff) as u8);
                    // 00e4d9c1: MOVZX ECX,byte ptr [ESI + 0x4c]
                    ecx = u32::from(e.mem.u8(esi.wrapping_add(0x4c)));
                    // 00e4d9c5: MOV byte ptr [EAX + 0x1],DL
                    e.mem.set_u8(eax.wrapping_add(0x1), (edx & 0xff) as u8);
                    // 00e4d9c8: MOVZX EDX,byte ptr [ESI + 0xec]
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0xec)));
                    // 00e4d9cf: MOV byte ptr [EAX + 0x2],CL
                    e.mem.set_u8(eax.wrapping_add(0x2), (ecx & 0xff) as u8);
                    // 00e4d9d2: MOV ECX,dword ptr [ESP + 0x18]
                    ecx = e.mem.u32(f0.wrapping_add(0xc));
                    // 00e4d9d6: MOV byte ptr [ECX + EAX*0x1],DL
                    e.mem.set_u8(ecx.wrapping_add(eax), (edx & 0xff) as u8);
                    // 00e4d9d9: MOVZX EDX,byte ptr [ESI + 0xfc]
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0xfc)));
                    // 00e4d9e0: MOV byte ptr [ECX + EAX*0x1 + 0x1],DL
                    e.mem
                        .set_u8(ecx.wrapping_add(eax).wrapping_add(0x1), (edx & 0xff) as u8);
                    // 00e4d9e4: MOVZX EDX,byte ptr [ESI + 0x10c]
                    edx = u32::from(e.mem.u8(esi.wrapping_add(0x10c)));
                    // 00e4d9eb: MOV byte ptr [ECX + EAX*0x1 + 0x2],DL
                    e.mem
                        .set_u8(ecx.wrapping_add(eax).wrapping_add(0x2), (edx & 0xff) as u8);
                    // 00e4d9ef: SHL CL,0x4
                    {
                        let r = fl.shl((ecx & 0xff), 4, 8);
                        ecx = (ecx & 0xffff_ff00) | ((r) & 0xff);
                    }
                    // 00e4d9f2: OR CL,byte ptr [ESP + 0x14]
                    {
                        let r = fl.or((ecx & 0xff), u32::from(e.mem.u8(f0.wrapping_add(0x8))), 8);
                        ecx = (ecx & 0xffff_ff00) | ((r) & 0xff);
                    }
                    // 00e4d9f6: MOV byte ptr [EAX + 0x4],CL
                    e.mem.set_u8(eax.wrapping_add(0x4), (ecx & 0xff) as u8);
                    pc = 0x00e4d9f9;
                }
                0x00e4d9f9 => {
                    // 00e4d9f9: MOV EAX,dword ptr [ESP + 0x20]
                    eax = e.mem.u32(f0.wrapping_add(0x14));
                    // 00e4d9fd: POP EDI
                    // 00e4d9fe: POP ESI
                    // 00e4d9ff: POP EBX
                    // 00e4da00: MOV ESP,EBP
                    // 00e4da02: POP EBP
                    // 00e4da03: RET 0xc
                    return eax;
                }
                _ => unreachable!("no block at {pc:08x}"),
            }
        }
    })
}
// END c730

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4c730, hk_gsk_get_closest_point(u32, u32, u32, u32) -> u32),
        entry!(0x00e4b810, find_closest_triangle_ov2(u32, u32, f32) -> i32),
        entry!(0x00e4bbe0, hk_gsk_process_edge_triangle(u32, u32, u32, u32, u32, u8, u32) -> u32),
        entry!(0x00e4c230, hk_gsk_handle_penetration(u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00e4da90, hk_gsk_check_triangle_boundaries_1(u32, u32, u32, u32) -> u32),
        entry!(0x00e4dc70, hk_gsk_check_triangle_boundaries_0(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_4000,
            0x0101_7000,
            0x0102_9000,
            0x0104_d000,
            0x010c_4000,
            0x010c_6000,
            0x010d_b000,
            0x010e_c000,
        ] {
            e.map(page, 0x1000);
        }
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
            e.mem.set_f32(ONES + 4 * lane, 1.0);
        }
        e.mem.set_f32(LARGEST_FLOAT, f32::MAX);
        e.mem.set_f32(NEAR_ZERO, 0.000001);
        e.mem.set_f64(TIE_FACTOR, f64::from(1.1f32));
        e.mem.set_f32(MINUS_ONE_FLOAT, -1.0);
        e.mem.set_f32(SMALL_EPSILON, 0.0001);
        e.mem.set_f32(PENETRATION_FLOAT, 0.75);
        e
    }

    /// A block of 16-byte vectors with room for `extra` more.
    fn vectors(e: &mut Engine, lanes: &[Vec4], extra: u32) -> u32 {
        let at = e.mem.alloc(0x10 * lanes.len() as u32 + extra);
        for (i, v) in lanes.iter().enumerate() {
            store_vec4(e, at + 0x10 * i as u32, *v);
        }
        at
    }

    /// The triangle (0, 0, 0), (2, 0, 0), (0, 2, 0) of the barycentric tests: the three
    /// numerators of a point (x, y, 0) are `8 (x + y - 2)`, `-8 x` and `-8 y`, all negative inside.
    const TRIANGLE: [Vec4; 3] = [
        [0.0, 0.0, 0.0, 0.0],
        [2.0, 0.0, 0.0, 0.0],
        [0.0, 2.0, 0.0, 0.0],
    ];

    #[test]
    fn the_edge_terms_are_negative_inside_the_triangle() {
        let mut e = engine();
        let triangle = vectors(&mut e, &TRIANGLE, 0);
        let out = e.mem.alloc(16);
        let inside = vectors(&mut e, &[[0.5, 0.5, 0.0, 0.0]], 0);
        assert_eq!(
            e.call(0x00e4_dc70, &args![out, inside, triangle, 0u32])
                .u32(),
            7
        );
        assert_eq!(load_vec4(&e, out)[..3], [-8.0, -4.0, -4.0]);
        // Beyond the hypotenuse the first term turns positive: the mask of the negative ones.
        let outside = vectors(&mut e, &[[3.0, 3.0, 0.0, 0.0]], 0);
        assert_eq!(
            e.call(0x00e4_dc70, &args![out, outside, triangle, 0u32])
                .u32(),
            6
        );
        assert_eq!(load_vec4(&e, out)[..3], [32.0, -24.0, -24.0]);
    }

    #[test]
    fn the_second_form_turns_the_triangle_when_the_plane_test_says_so() {
        let mut e = engine();
        let this = e.mem.alloc(0x140);
        let point = vectors(&mut e, &[[0.5, 0.5, 0.0, 0.0]], 0);
        let triangle = vectors(
            &mut e,
            &[
                TRIANGLE[0],
                TRIANGLE[1],
                TRIANGLE[2],
                [0.0; 4],
                [7.0; 4],
                [8.0; 4],
            ],
            0,
        );
        let normal = e.mem.alloc(16);
        // The normal of the triangle is (0, 0, 4); the plane test dots it with `this[0x20] -
        // this[0xa0]`, here (0, 0, -2): negative, so the triangle is turned.
        store_vec4(&mut e, this + 0x20, [0.0, 0.0, -1.0, 0.0]);
        store_vec4(&mut e, this + 0xa0, [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(
            e.call(0x00e4_da90, &args![this, point, triangle, normal])
                .u32(),
            7
        );
        // The numerators (-8, -4, -4) have their first two exchanged.
        assert_eq!(load_vec4(&e, this)[..3], [-4.0, -8.0, -4.0]);
        let flipped = load_vec4(&e, normal);
        assert_eq!(flipped[..2], [0.0, 0.0]);
        assert_eq!(flipped[2], -4.0);
        assert_eq!(load_vec4(&e, triangle), TRIANGLE[1]);
        assert_eq!(load_vec4(&e, triangle + 0x10), TRIANGLE[0]);
        assert_eq!(load_vec4(&e, triangle + 0x40), [8.0; 4]);
        assert_eq!(load_vec4(&e, triangle + 0x50), [7.0; 4]);
        assert_eq!(e.mem.u32(this + 0x1c), 1);
        // The other side of the plane leaves everything.
        let this = e.mem.alloc(0x140);
        store_vec4(&mut e, this + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec4(&mut e, this + 0xa0, [0.0, 0.0, -1.0, 0.0]);
        let triangle = vectors(
            &mut e,
            &[
                TRIANGLE[0],
                TRIANGLE[1],
                TRIANGLE[2],
                [0.0; 4],
                [7.0; 4],
                [8.0; 4],
            ],
            0,
        );
        assert_eq!(
            e.call(0x00e4_da90, &args![this, point, triangle, normal])
                .u32(),
            7
        );
        assert_eq!(load_vec4(&e, normal)[2], 4.0);
        assert_eq!(load_vec4(&e, triangle), TRIANGLE[0]);
        assert_eq!(load_vec4(&e, triangle + 0x40), [7.0; 4]);
        assert_eq!(e.mem.u32(this + 0x1c), 0);
        // A point outside only has its mask answered.
        let outside = vectors(&mut e, &[[3.0, 3.0, 0.0, 0.0]], 0);
        assert_eq!(
            e.call(0x00e4_da90, &args![this, outside, triangle, normal])
                .u32(),
            6
        );
        assert_eq!(e.mem.u32(this + 0x1c), 0);
    }

    /// The tetrahedron (0, 0, 0), (1, 0, 0), (0, 1, 0) with its apex (0, 0, 1): the faces
    /// touching the apex have the normals (1, 1, 1), (-1, 0, 0) and (0, -1, 0).
    const TETRAHEDRON: [Vec4; 4] = [
        [0.0, 0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ];

    #[test]
    fn the_closest_face_is_the_one_the_point_is_furthest_outside() {
        let mut e = engine();
        let tetrahedron = vectors(&mut e, &TETRAHEDRON, 0);
        let mut closest = |point: Vec4, scale: f32| {
            let at = vectors(&mut e, &[point], 0);
            e.call(0x00e4_b810, &args![at, tetrahedron, scale]).i32()
        };
        // Out along (1, 1, 1), (-1, 0, 0) and (0, -1, 0): the faces 0, 1 and 2.
        assert_eq!(closest([1.0, 1.0, 2.0, 0.0], 1.0), 0);
        assert_eq!(closest([-1.0, 0.0, 1.0, 0.0], 1.0), 1);
        assert_eq!(closest([0.0, -1.0, 1.0, 0.0], 1.0), 2);
        assert_eq!(closest([0.0, -1.0, 1.0, 0.0], 0.5), 2);
        // At the apex no face is outside (the largest key is below 1e-6).
        assert_eq!(closest([0.0, 0.0, 1.0, 0.0], 1.0), -1);
        // Two faces equally far (-1, -1, 0): the barycentric tests of the pairs decide.
        assert_eq!(closest([-1.0, -1.0, 1.0, 0.0], 1.0), 2);
    }

    /// The pieces of the edge tests: a segment (`a`, with two more vectors at `+0x40` and
    /// `+0x50`), a triangle (`b`, three more at `+0x40`, `+0x50` and `+0x60`) and the object.
    struct Edge {
        this: u32,
        a: u32,
        b: u32,
        out_a: u32,
        out_b: u32,
    }

    fn edge(e: &mut Engine, a0: Vec4, a1: Vec4) -> Edge {
        let this = e.mem.alloc(0x140);
        store_vec4(e, this, [1.0, 2.0, 3.0, 0.0]);
        store_vec4(e, this + 0x120, [0.0, 0.0, 1.0, 0.0]);
        let a = vectors(e, &[a0, a1, [0.0; 4], [0.0; 4], [1.0; 4], [2.0; 4]], 0);
        let b = vectors(
            e,
            &[
                TRIANGLE[0],
                TRIANGLE[1],
                TRIANGLE[2],
                [0.0; 4],
                [3.0; 4],
                [4.0; 4],
                [5.0; 4],
            ],
            0,
        );
        let out_a = e.mem.alloc(4);
        let out_b = e.mem.alloc(4);
        Edge {
            this,
            a,
            b,
            out_a,
            out_b,
        }
    }

    fn process(e: &mut Engine, edge: &Edge, use_triangle: u8) -> u32 {
        e.call(
            0x00e4_bbe0,
            &args![
                edge.this,
                edge.a,
                edge.b,
                edge.out_a,
                edge.out_b,
                use_triangle,
                0x99u32
            ],
        )
        .u32()
    }

    const ABOVE: Vec4 = [0.5, 0.5, 1.0, 0.0];

    #[test]
    fn a_segment_through_the_triangle_is_answered_one() {
        let mut e = engine();
        // Without the triangle's own normal: this + 0x120 is the plane's normal (0, 0, 1).
        let edge1 = edge(&mut e, ABOVE, [0.5, 0.5, -1.0, 0.0]);
        assert_eq!(process(&mut e, &edge1, 0), 1);
        assert_eq!(e.mem.u32(edge1.out_a), 0);
        assert_eq!(e.mem.u32(edge1.out_b), 0);
        // With it: the normal is (b1 - b2) x (b0 - b2) = (0, 0, -4), the crossing is the same.
        let edge2 = edge(&mut e, ABOVE, [0.5, 0.5, -1.0, 0.0]);
        assert_eq!(process(&mut e, &edge2, 1), 1);
        // A segment through the plane outside the triangle goes on to the edge tests.
        let edge3 = edge(&mut e, [3.0, 3.0, 1.0, 0.0], [3.0, 3.0, -1.0, 0.0]);
        assert_eq!(process(&mut e, &edge3, 0), 2);
    }

    #[test]
    fn an_end_over_the_triangle_ends_the_search_with_the_end_taken() {
        let mut e = engine();
        // Both ends on one side, the second over the triangle: out_a is set, the second end
        // and its companion vector replace the first, `extra` is kept.
        let edge1 = edge(&mut e, ABOVE, [0.5, 0.5, 2.0, 0.0]);
        assert_eq!(process(&mut e, &edge1, 0), 0);
        assert_eq!(e.mem.u32(edge1.out_a), 1);
        assert_eq!(load_vec4(&e, edge1.a), [0.5, 0.5, 2.0, 0.0]);
        assert_eq!(load_vec4(&e, edge1.a + 0x40), [2.0; 4]);
        assert_eq!(e.mem.u32(edge1.this + 0x14), 0x99);
        // The triangle's own form: the end nearer the plane is taken first.
        let edge2 = edge(&mut e, ABOVE, [0.5, 0.5, 2.0, 0.0]);
        assert_eq!(process(&mut e, &edge2, 1), 0);
        assert_eq!(e.mem.u32(edge2.out_a), 1);
        assert_eq!(load_vec4(&e, edge2.a), ABOVE);
        // With the ends the other way round they are exchanged first.
        let edge3 = edge(&mut e, [0.5, 0.5, 2.0, 0.0], ABOVE);
        assert_eq!(process(&mut e, &edge3, 1), 0);
        assert_eq!(load_vec4(&e, edge3.a), ABOVE);
        assert_eq!(load_vec4(&e, edge3.a + 0x10), [0.5, 0.5, 2.0, 0.0]);
        assert_eq!(load_vec4(&e, edge3.a + 0x40), [2.0; 4]);
        assert_eq!(load_vec4(&e, edge3.a + 0x50), [1.0; 4]);
    }

    #[test]
    fn the_vertex_the_end_has_passed_is_replaced_by_the_third() {
        let mut e = engine();
        // The mask of the lanes below 0 picks the replaced vertex: 6 (the end is beyond the
        // hypotenuse) replaces b0, 5 (left of the triangle) b1 and 3 (below) none.
        for (point, replaced) in [
            ([3.0, 3.0, 2.0, 0.0], Some(0u32)),
            ([-1.0, 1.0, 2.0, 0.0], Some(1)),
            ([1.0, -1.0, 2.0, 0.0], None),
            // Mask 4: the products of the old and new numerators decide (here b1).
            ([-1.0, 4.0, 2.0, 0.0], Some(1)),
        ] {
            let edge = edge(&mut e, ABOVE, point);
            assert_eq!(process(&mut e, &edge, 0), 2, "{point:?}");
            assert_eq!(e.mem.u32(edge.out_b), 2);
            assert_eq!(e.mem.u32(edge.out_a), 0);
            for k in 0..3u32 {
                let expected = if Some(k) == replaced {
                    TRIANGLE[2]
                } else {
                    TRIANGLE[k as usize]
                };
                assert_eq!(load_vec4(&e, edge.b + 0x10 * k), expected, "{point:?} {k}");
            }
            // The companion vectors of b0 and b1 are at +0x40 and +0x50.
            let first = load_vec4(&e, edge.b + 0x40);
            let second = load_vec4(&e, edge.b + 0x50);
            match replaced {
                Some(0) => assert_eq!((first, second), ([5.0; 4], [4.0; 4])),
                Some(_) => assert_eq!((first, second), ([3.0; 4], [5.0; 4])),
                None => assert_eq!((first, second), ([3.0; 4], [4.0; 4])),
            }
        }
    }

    fn closest_segment_double(e: &mut Engine, distances: [f32; 2]) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let calls: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let sink = Rc::clone(&calls);
        e.register_double(CLOSEST_LINE_SEG_LINE_SEG, move |e, a| {
            let n = sink.borrow().len();
            sink.borrow_mut().push(a.to_vec());
            e.mem.set_f32(a[4] + 0x20, distances[n]);
            Ret::default()
        });
        calls
    }

    #[test]
    fn the_edge_nearer_the_segment_decides_which_vertex_goes() {
        let mut e = engine();
        // The first edge (b2 -> b1) is nearer: b0 is replaced; the second (b2 -> b0) strictly
        // nearer: b1 is replaced.
        for (distances, replaced) in [([1.0f32, 4.0], 0u32), ([4.0, 1.0], 1), ([2.0, 2.0], 0)] {
            let calls = closest_segment_double(&mut e, distances);
            let edge = edge(&mut e, [3.0, 3.0, 1.0, 0.0], [3.0, 3.0, 2.0, 0.0]);
            assert_eq!(process(&mut e, &edge, 1), 2);
            assert_eq!(e.mem.u32(edge.out_b), 2);
            assert_eq!(load_vec4(&e, edge.b + 0x10 * replaced), TRIANGLE[2]);
            assert_eq!(load_vec4(&e, edge.b + 0x40 + 0x10 * replaced), [5.0; 4]);
            // Both edges were measured against the segment from the first end point.
            let calls = calls.borrow();
            assert_eq!(calls.len(), 2);
            for call in calls.iter() {
                assert_eq!(call[0], edge.a);
                assert_eq!(call[2], edge.b + 0x20);
            }
        }
    }

    /// Doubles for `00e63770` (it logs the block it is given and answers `answer`) and `00e62740`.
    fn penetration_doubles(
        e: &mut Engine,
        answer: u32,
    ) -> (Rc<RefCell<Vec<Vec<u32>>>>, Rc<RefCell<Vec<Vec<u32>>>>) {
        let blocks: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let sink = Rc::clone(&blocks);
        e.register_double(UNNAMED_00E63770, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..44).map(|i| e.mem.u32(a[0] + 4 * i)));
            sink.borrow_mut().push(words);
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        let runs: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let sink = Rc::clone(&runs);
        e.register_double(UNNAMED_00E62740, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (blocks, runs)
    }

    struct Penetration {
        this: u32,
        matrix: u32,
        count_a: u32,
        count_b: u32,
        direction: u32,
    }

    fn penetration(e: &mut Engine, na: u32, nb: u32) -> Penetration {
        let this = e.mem.alloc(0x140);
        // The tags (bits 24 to 29 of lane 3) of the first set's slots 2 to 4 and the second
        // set's slots 0xe to 0x10, with a vector of their own in the rest.
        for (slot, tag) in [(2u32, 1u32), (3, 2), (4, 3), (0xe, 4), (0xf, 5), (0x10, 6)] {
            let v = [slot as f32, 1.0, 2.0, 0.0];
            store_vec4(e, this + 0x10 * slot, v);
            e.mem.set_u32(this + 0x10 * slot + 0xc, (tag << 24) | slot);
        }
        let matrix = vectors(
            e,
            &[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [10.0, 20.0, 30.0, 0.0],
            ],
            0,
        );
        let count_a = e.mem.alloc(4);
        let count_b = e.mem.alloc(4);
        e.mem.set_u32(count_a, na);
        e.mem.set_u32(count_b, nb);
        let direction = vectors(e, &[[0.0, 0.0, 1.0, 0.0]], 0);
        Penetration {
            this,
            matrix,
            count_a,
            count_b,
            direction,
        }
    }

    fn handle(e: &mut Engine, p: &Penetration) -> u32 {
        e.call(
            0x00e4_c230,
            &args![
                p.this,
                0x1111u32,
                0x2222u32,
                p.matrix,
                p.count_a,
                p.count_b,
                p.direction
            ],
        )
        .u32()
    }

    #[test]
    fn a_penetrating_pair_is_reduced_to_one_point_each() {
        let mut e = engine();
        let (blocks, runs) = penetration_doubles(&mut e, 5);
        let p = penetration(&mut e, 1, 1);
        assert_eq!(handle(&mut e, &p), p.count_a);
        assert_eq!(
            *runs.borrow(),
            vec![vec![
                0x1111,
                0x2222,
                p.matrix,
                p.this + 0x20,
                p.this + 0xe0,
                p.direction
            ]]
        );
        assert_eq!((e.mem.u32(p.count_a), e.mem.u32(p.count_b)), (1, 1));
        assert_eq!(load_vec4(&e, p.this + 0x120), [0.0, 0.0, 1.0, 0.0]);
        // The block handed to `00e63770`: the matrix, 1e-4, the float at 010dbb58, the two
        // arguments, the two lists, the kind (1 for one point in each) and four differences.
        let block = blocks.borrow()[0].clone();
        let words = &block[2..];
        assert_eq!(block[1], p.direction);
        assert_eq!(
            words[..16],
            [
                1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 10.0, 20.0, 30.0,
                0.0
            ]
            .map(f32::to_bits)
        );
        assert_eq!(words[16..20], [0.0001f32.to_bits(); 4]);
        assert_eq!(words[20], 0.75f32.to_bits());
        assert_eq!(words[21..23], [0x1111, 0x2222]);
        assert_eq!(
            words[24..28],
            [p.this + 0x20, p.this + 0xe0, block[0] + 0x70, 1]
        );
    }

    #[test]
    fn counts_and_tags_decide_how_many_points_stay() {
        let mut e = engine();
        let (blocks, runs) = penetration_doubles(&mut e, 3);
        // One point in each: the slots are padded with copies (all tags equal), both counts
        // drop to 1 and the second set is moved through the matrix.
        let p = penetration(&mut e, 1, 1);
        assert_eq!(handle(&mut e, &p), 1);
        assert!(runs.borrow().is_empty());
        assert_eq!((e.mem.u32(p.count_a), e.mem.u32(p.count_b)), (1, 1));
        assert_eq!(blocks.borrow()[0][2 + 27], 1);
        let moved = load_vec4(&e, p.this + 0xa0);
        assert_eq!(moved, [0xe as f32 + 10.0, 21.0, 32.0, 0.0]);
        assert_eq!(load_vec4(&e, p.this + 0x120), [0.0, 0.0, 1.0, 0.0]);
        // Three in each: the kind is 4, the first set shrinks to one point (its tags are
        // copies), the second keeps three (different tags); 1 + 3 fits.
        let p = penetration(&mut e, 3, 3);
        assert_eq!(handle(&mut e, &p), 1);
        assert_eq!(blocks.borrow()[1][2 + 27], 4);
        assert_eq!((e.mem.u32(p.count_a), e.mem.u32(p.count_b)), (1, 3));
        for k in 0..3u32 {
            let source = load_vec4(&e, p.this + 0x10 * (0xe + k));
            let moved = load_vec4(&e, p.this + 0x10 * (0xa + k));
            assert_eq!(
                moved,
                [source[0] + 10.0, source[1] + 20.0, source[2] + 30.0, 0.0]
            );
        }
    }

    const GJK_SHAPE_VTABLE: u32 = 0x4400_a000;
    const GJK_SUPPORT: u32 = 0x4400_a001;
    const GJK_CONVERT: u32 = 0x4400_a002;
    const ROTATE_INVERSE: u32 = 0x004b_4ab0;
    const ROTATE_VECTOR: u32 = 0x00c7_f700;

    fn gjk_vertex(e: &Engine, shape: u32, id: u32) -> Vec4 {
        load_vec4(e, shape + 0x10 + 16 * id)
    }

    /// `getSupportingVertex` (slot 4): the vertex with the largest dot product, its number in lane 3.
    fn gjk_support(e: &mut Engine, a: &[u32]) -> Ret {
        let count = e.mem.u32(a[0] + 4);
        let dir = load_vec4(e, a[1]);
        let (mut best, mut best_dot) = (0, f32::NEG_INFINITY);
        for id in 0..count {
            let v = gjk_vertex(e, a[0], id);
            let dot = v[0] * dir[0] + v[1] * dir[1] + v[2] * dir[2];
            if dot > best_dot {
                (best, best_dot) = (id, dot);
            }
        }
        let v = gjk_vertex(e, a[0], best);
        store_vec4(e, a[2], [v[0], v[1], v[2], 0.0]);
        e.mem.set_u32(a[2] + 12, best);
        Ret::default()
    }

    /// Slot 8: the vertices of the numbered ids, each with its number in lane 3.
    fn gjk_convert(e: &mut Engine, a: &[u32]) -> Ret {
        for i in 0..a[2] {
            let id = u32::from(e.mem.u8(a[1] + i));
            let v = gjk_vertex(e, a[0], id);
            store_vec4(e, a[3] + 16 * i, [v[0], v[1], v[2], 0.0]);
            e.mem.set_u32(a[3] + 16 * i + 12, id);
        }
        Ret::default()
    }

    /// `00c7f700` (`out`, matrix, vector): the vector turned by the matrix' three rows.
    fn gjk_rotate(e: &mut Engine, a: &[u32]) -> Ret {
        let v = load_vec4(e, a[2]);
        let rows = [0, 1, 2].map(|k| load_vec4(e, a[1] + 16 * k));
        let out = [0, 1, 2].map(|k| rows[k][0] * v[0] + rows[k][1] * v[1] + rows[k][2] * v[2]);
        store_vec4(e, a[0], [out[0], out[1], out[2], 0.0]);
        Ret::default()
    }

    /// `004b4ab0` (`out`, matrix, vector): the vector turned by the transpose.
    fn gjk_rotate_inverse(e: &mut Engine, a: &[u32]) -> Ret {
        let v = load_vec4(e, a[2]);
        let rows = [0, 1, 2].map(|k| load_vec4(e, a[1] + 16 * k));
        let out = [0, 1, 2].map(|k| rows[0][k] * v[0] + rows[1][k] * v[1] + rows[2][k] * v[2]);
        store_vec4(e, a[0], [out[0], out[1], out[2], 0.0]);
        Ret::default()
    }

    /// `00d22e40` (cdecl: segment a, its direction, segment b, its direction, out): the closest points
    /// of the two segments at `out` and `out + 0x10`, their squared distance at `out + 0x20` and, in
    /// `EAX`, the end points they lie at (bit 0: the end of a, 1: its start, 2: the end of b, 3: its
    /// start).
    fn gjk_closest_segments(e: &mut Engine, a: &[u32]) -> Ret {
        let v = |e: &Engine, at: u32| {
            let x = load_vec4(e, at);
            [x[0], x[1], x[2]]
        };
        let (p, d1, q, d2) = (v(e, a[0]), v(e, a[1]), v(e, a[2]), v(e, a[3]));
        let dot = |x: [f32; 3], y: [f32; 3]| x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
        let r = [p[0] - q[0], p[1] - q[1], p[2] - q[2]];
        let (aa, ee, f) = (dot(d1, d1), dot(d2, d2), dot(d2, r));
        let (s, t);
        if aa <= 1e-12 && ee <= 1e-12 {
            (s, t) = (0.0, 0.0);
        } else if aa <= 1e-12 {
            (s, t) = (0.0, (f / ee).clamp(0.0, 1.0));
        } else {
            let c = dot(d1, r);
            if ee <= 1e-12 {
                (s, t) = ((-c / aa).clamp(0.0, 1.0), 0.0);
            } else {
                let b = dot(d1, d2);
                let denominator = aa * ee - b * b;
                let mut s0 = if denominator > 1e-12 {
                    ((b * f - c * ee) / denominator).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                let mut t0 = (b * s0 + f) / ee;
                if t0 < 0.0 {
                    t0 = 0.0;
                    s0 = (-c / aa).clamp(0.0, 1.0);
                } else if t0 > 1.0 {
                    t0 = 1.0;
                    s0 = ((b - c) / aa).clamp(0.0, 1.0);
                }
                (s, t) = (s0, t0);
            }
        }
        let pa = [0, 1, 2].map(|k| p[k] + d1[k] * s);
        let pb = [0, 1, 2].map(|k| q[k] + d2[k] * t);
        store_vec4(e, a[4], [pa[0], pa[1], pa[2], 0.0]);
        store_vec4(e, a[4] + 0x10, [pb[0], pb[1], pb[2], 0.0]);
        let diff = [pa[0] - pb[0], pa[1] - pb[1], pa[2] - pb[2]];
        e.mem.set_f32(a[4] + 0x20, dot(diff, diff));
        Ret {
            eax: u32::from(s == 1.0)
                | (u32::from(s == 0.0) << 1)
                | (u32::from(t == 1.0) << 2)
                | (u32::from(t == 0.0) << 3),
            ..Ret::default()
        }
    }

    /// `00e63770` (thiscall on the block `handlePenetration` builds, the direction): a function of the
    /// block words (but the pointer to its own differences and the words around it), 0 to 5.
    fn gjk_penetration(e: &mut Engine, a: &[u32]) -> Ret {
        let sum = (0..44u32)
            .filter(|i| !(24..=26).contains(i))
            .fold(0u32, |s, i| {
                s.wrapping_add(e.mem.u32(a[0] + 4 * i).rotate_left(i))
            });
        Ret {
            eax: [5u32, 1, 3, 4, 0, 2][(sum % 6) as usize],
            ..Ret::default()
        }
    }

    fn gjk_engine() -> Engine {
        let mut e = engine();
        e.map(0x0101_6000, 0x1000);
        e.map(0x0104_f000, 0x1000);
        e.map(0x0107_1000, 0x1000);
        e.mem.set_f32(0x0104_f498, f32::from_bits(0x3fb3_3333));
        e.mem
            .set_f32(PENETRATION_FLOAT, f32::from_bits(0x322b_cc76));
        e.mem.set_f32(LARGEST_FLOAT, f32::from_bits(0x7f7f_ffee));
        e.mem.set_f32(NEAR_ZERO, f32::from_bits(0x3586_37be));
        e.mem.set_f32(0x0107_18c0, f32::from_bits(0x3727_c5ac));
        e.mem.set_f32(0x0101_7718, 3.0);
        e.mem.set_f32(0x0101_6248, 0.5);
        e.mem.set_f32(0x0101_2068, 1.0);
        e.mem.set_f32(0x0101_2054, -1.0);
        // The table of the penetration cases: what each mask of `checkTriangleBoundaries<1>` does.
        e.mem.write(
            0x010e_cecc,
            &[
                0xfe, 0, 1, 0xfa, 2, 0xf9, 0xf8, 0xfe, 2, 0, 1, 2, 0, 0, 0, 0,
            ],
        );
        for (i, word) in [0u32, 0, 4, 0xc].into_iter().enumerate() {
            e.mem.set_u32(0x010e_c5f0 + 4 * i as u32, word);
        }
        for (i, word) in [0u32, 0, 1, 3].into_iter().enumerate() {
            e.mem.set_u32(0x010e_c600 + 4 * i as u32, word);
        }
        // The word after the table, which a count of 4 reads too.
        e.mem.set_u32(0x010e_c610, 0x0113_b7c8);
        e.put_vtable(GJK_SHAPE_VTABLE, &[0, GJK_SUPPORT, GJK_CONVERT]);
        e.register(GJK_SUPPORT, gjk_support);
        e.register(GJK_CONVERT, gjk_convert);
        e.register(ROTATE_INVERSE, gjk_rotate_inverse);
        e.register(ROTATE_VECTOR, gjk_rotate);
        e.register(CLOSEST_LINE_SEG_LINE_SEG, gjk_closest_segments);
        e.register(UNNAMED_00E63770, gjk_penetration);
        e.register(UNNAMED_00E62740, |_, _| Ret::default());
        e
    }

    fn gjk_shape(e: &mut Engine, vertices: &[[u32; 3]], shift: [f32; 3]) -> u32 {
        let at = (e.mem.alloc(0x20 + 16 * vertices.len() as u32) + 15) & !15;
        e.mem.set_u32(at, GJK_SHAPE_VTABLE);
        e.mem.set_u32(at + 4, vertices.len() as u32);
        for (i, v) in vertices.iter().enumerate() {
            let p = [0, 1, 2].map(|k| f32::from_bits(v[k]) + shift[k]);
            store_vec4(e, at + 0x10 + 16 * i as u32, [p[0], p[1], p[2], 0.0]);
        }
        at
    }

    /// Two scenarios recorded from the machine code (run with the same shapes and helper routines):
    /// two point sets whose closest points the loop finds, the first call from nothing, the second
    /// from the vertex numbers the first one wrote.
    #[test]
    fn the_closest_points_of_two_shapes_match_the_exes_answers() {
        type Case = (
            Vec<[u32; 3]>,
            Vec<[u32; 3]>,
            [u32; 3],
            [u32; 8],
            [u32; 8],
            [u8; 5],
            [u32; 4],
        );
        let cases: Vec<Case> = vec![
            (
                vec![
                    [1058310140, 3209268350, 1047482128],
                    [1062314862, 3210145798, 1060793168],
                    [3200747202, 1059927464, 1056008308],
                    [3196391836, 3199874786, 3208043614],
                ],
                vec![
                    [3207996637, 1027289056, 1035073696],
                    [3208427812, 1028497024, 1023168576],
                    [1016726144, 1063339950, 1051569056],
                    [1059152356, 1043448880, 3200136512],
                ],
                [0x3e91_4e38, 0x3ef1_8fc8, 0xbb5e_d200],
                [
                    0x3e93_dd8e,
                    0xbf34_2bee,
                    0x3f26_2831,
                    0x3e1a_73c8,
                    0xbeb6_bf2d,
                    0x3ed3_efc6,
                    0x3e30_3b80,
                    2,
                ],
                [
                    0x3e93_dd8e,
                    0xbf34_2bee,
                    0x3f26_2831,
                    0x3e1a_73c8,
                    0xbeb6_bf2d,
                    0x3ed3_efc6,
                    0x3e30_3b80,
                    2,
                ],
                [3, 0, 0, 0, 17],
                [2, 0, 0, 1],
            ),
            (
                vec![
                    [3209023403, 3198210488, 1049280644],
                    [3197109902, 3203948294, 3202602762],
                    [1062996018, 1058992974, 3207456411],
                    [1043388488, 3209523364, 1055760004],
                ],
                vec![
                    [3209112322, 3210951105, 3207786922],
                    [1058147402, 1029194976, 3194912088],
                    [1058979552, 3209880718, 3205550519],
                    [3197130034, 1041375216, 1053934644],
                ],
                [0x3feb_b98c, 0x3fc6_7382, 0x3df4_8800],
                [
                    0xbf62_34eb,
                    0xbe39_c0d7,
                    0xbedc_ff72,
                    0x3e6c_866e,
                    0x3f5c_0832,
                    0x3f1e_f34e,
                    0xbf2d_e69b,
                    2,
                ],
                [
                    0xbf62_34eb,
                    0xbe39_c0d7,
                    0xbedc_ff72,
                    0x3e6c_866e,
                    0x3f5c_0832,
                    0x3f1e_f34e,
                    0xbf2d_e69b,
                    2,
                ],
                [0, 2, 0, 0, 33],
                [1, 0, 0, 1],
            ),
        ];
        for (index, (first, second, shift_bits, out0, out1, ids_after, state)) in
            cases.into_iter().enumerate()
        {
            let mut e = gjk_engine();
            let shift = shift_bits.map(f32::from_bits);
            let shape0 = gjk_shape(&mut e, &first, [0.0; 3]);
            let shape1 = gjk_shape(&mut e, &second, shift);
            let input = (e.mem.alloc(0x70) + 15) & !15;
            for (k, row) in [
                [1.0f32, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ]
            .into_iter()
            .enumerate()
            {
                store_vec4(&mut e, input + 16 * k as u32, row);
            }
            e.mem.set_u32(input + 0x40, shape0);
            e.mem.set_u32(input + 0x44, shape1);
            // The vector at `+0x50` of the pair (unnamed; `getClosestPoint` reads it at `00e4cf79`).
            store_vec4(&mut e, input + 0x50, [0.25, -0.5, 0.75, 0.0]);
            let this = (e.mem.alloc(0x210) + 15) & !15;
            let ids = e.mem.alloc(16);
            let output = (e.mem.alloc(0x30) + 15) & !15;
            let words = |e: &Engine, at: u32| -> [u32; 8] {
                [0, 1, 2, 3, 4, 5, 6, 7].map(|i| e.mem.u32(at + 4 * i))
            };
            // From nothing: the code 0 of a found pair and its closest points.
            assert_eq!(
                e.call(0x00e4_c730, &args![this, input, 0u32, output]).u32(),
                0,
                "case {index}"
            );
            assert_eq!(words(&e, output), out0, "case {index} first output");
            // From a zeroed byte array (the counts it holds are 0): the answer is the same pair and the
            // array is written with the ids the call ends with.
            assert_eq!(
                e.call(0x00e4_c730, &args![this, input, ids, output]).u32(),
                0,
                "case {index}"
            );
            assert_eq!(words(&e, output), out1, "case {index} second output");
            assert_eq!(
                [0, 1, 2, 3].map(|i| e.mem.u32(this + 0x10 + 4 * i)),
                state,
                "case {index} state"
            );
            for (i, id) in ids_after.iter().enumerate() {
                assert_eq!(e.mem.u8(ids + i as u32), *id, "case {index} id {i}");
            }
        }
    }
}

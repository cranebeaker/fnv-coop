//! `Havok SDK/hkpgsk.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;
use std::cmp::Ordering;

/// A four-lane single-precision vector (one SSE register).
type Vec4 = [f32; 4];

/// Edge tables in the exe image (`010cb440`, signed bytes): entry `i` of the first table and
/// of the second (`010cb442`) are the two vertex numbers an edge test uses.
const EDGE_TABLE_FIRST: u32 = 0x010c_b440;
const EDGE_TABLE_SECOND: u32 = 0x010c_b442;
/// Signed bytes in the exe image (`010cb438`) indexed by a three-bit sign mask.
const MASK_TABLE: u32 = 0x010c_b438;
/// `-1.0f` (`01012054`).
const MINUS_ONE: u32 = 0x0101_2054;
/// The large float `0x7f7fffee` (`0104da48`) used as "no distance".
const BIG_FLOAT: u32 = 0x0104_da48;
/// A vector of four absolute-value masks (`01017d10`), one of four sign bits (`010c44d0`) and
/// one of four `1.0f` (`010ec7f0`).
const ABS_MASK: u32 = 0x0101_7d10;
const SIGN_MASK: u32 = 0x010c_44d0;
const ONES: u32 = 0x010e_c7f0;
/// A `float` threshold of about `1e-6` (`010c6090`) and a `double` factor of `1.1`
/// (`010290f8`) used by `findClosestTriangle`.
const NEAR_ZERO: u32 = 0x010c_6090;
const TIE_FACTOR: u32 = 0x0102_90f8;
/// `hkLineSegmentUtil::closestLineSegLineSeg` (Xbox PDB, `00d22e40`, cdecl `(pointA, dirA,
/// pointB, dirB, out)`, returns a bit mask).
const CLOSEST_LINE_SEG_LINE_SEG: u32 = 0x00d2_2e40;
/// `hkCalculatePenetrationDepth` (Xbox PDB, `00dfaac0`, cdecl, nine words).
const CALCULATE_PENETRATION_DEPTH: u32 = 0x00df_aac0;

fn ld(e: &Engine, at: u32) -> Vec4 {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn st(e: &mut Engine, at: u32, v: Vec4) {
    for (i, lane) in v.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

/// A 16-byte copy that keeps the bits (integer moves in the exe).
fn copy16(e: &mut Engine, from: u32, to: u32) {
    for i in 0..4 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// An `FLD` / `FSTP` round trip of a float in memory (it quiets a signalling NaN).
fn load_float(e: &Engine, at: u32) -> f32 {
    x87_quiet(e.mem.f32(at))
}

fn vsub(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] - b[i])
}

fn vadd(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] + b[i])
}

fn vmul(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] * b[i])
}

/// `SHUFPS dst, src, imm`: lanes 0 and 1 from `dst`, lanes 2 and 3 from `src`.
fn shufps(dst: Vec4, src: Vec4, imm: u8) -> Vec4 {
    let pick = |v: Vec4, shift: u8| v[usize::from((imm >> shift) & 3)];
    [pick(dst, 0), pick(dst, 2), pick(src, 4), pick(src, 6)]
}

/// `UNPCKLPS dst, src`.
fn unpcklps(dst: Vec4, src: Vec4) -> Vec4 {
    [dst[0], src[0], dst[1], src[1]]
}

/// `UNPCKHPS dst, src`.
fn unpckhps(dst: Vec4, src: Vec4) -> Vec4 {
    [dst[2], src[2], dst[3], src[3]]
}

/// `MOVLHPS dst, src`.
fn movlhps(dst: Vec4, src: Vec4) -> Vec4 {
    [dst[0], dst[1], src[0], src[1]]
}

/// `MOVHLPS dst, src`.
fn movhlps(dst: Vec4, src: Vec4) -> Vec4 {
    [src[2], src[3], dst[2], dst[3]]
}

/// The horizontal sum of the first three lanes the way the exe forms it: lane 1 plus lane 0
/// (`ADDSS`), then lane 2 plus that.
fn sum3(v: Vec4) -> f32 {
    v[2] + (v[1] + v[0])
}

/// The bit-wise AND of each lane with a mask.
fn andps(v: Vec4, mask: [u32; 4]) -> Vec4 {
    [0, 1, 2, 3].map(|i| f32::from_bits(v[i].to_bits() & mask[i]))
}

fn xorps(v: Vec4, mask: [u32; 4]) -> Vec4 {
    [0, 1, 2, 3].map(|i| f32::from_bits(v[i].to_bits() ^ mask[i]))
}

fn mask_at(e: &Engine, at: u32) -> [u32; 4] {
    [0, 4, 8, 12].map(|o| e.mem.u32(at + o))
}

fn signed_byte(e: &Engine, at: u32) -> i32 {
    i32::from(e.mem.u8(at) as i8)
}

// Translated from 00da8390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `findClosestTriangleBackup` (Xbox PDB). Register convention: `vertices` is EDI (four
/// 16-byte vectors; the first three are the triangle's, the fourth the apex), the other two
/// words are cdecl stack arguments: `point` (a vector) and `weights` (three floats). For
/// each of the three edges `i` (vertex numbers from the two edge tables) whose two weights
/// are both at least zero, the sign of a cross-product-like determinant of the point and the
/// apex against the edge decides which of the two weights is cleared to `-1.0`. Returns the
/// number of the first weight that is above zero, or -1 when none is.
pub fn find_closest_triangle_backup(
    e: &mut Engine,
    vertices: Ptr,
    point: Ptr,
    weights: Ptr,
) -> i32 {
    let v = vertices.addr();
    let w = weights.addr();
    let minus_one = load_float(e, MINUS_ONE);
    for i in 0..3u32 {
        let second = signed_byte(e, EDGE_TABLE_SECOND + i) as u32;
        let first = signed_byte(e, EDGE_TABLE_FIRST + i) as u32;
        let w_second = e.mem.f32(w + 4 * second);
        let w_first = e.mem.f32(w + 4 * first);
        if w_second >= 0.0 && w_first >= 0.0 {
            let base = ld(e, v + 16 * i);
            let apex = ld(e, v + 0x30);
            let edge_second = vsub(ld(e, v + 16 * second), base);
            let edge_first = vsub(ld(e, v + 16 * first), base);
            let to_point = vsub(ld(e, point.addr()), base);
            let to_apex = vsub(apex, base);
            let point_first = vmul(to_point, edge_first);
            let point_second = vmul(to_point, edge_second);
            let apex_second = vmul(to_apex, edge_second);
            let apex_first = vmul(to_apex, edge_first);
            let determinant =
                sum3(point_second) * sum3(apex_first) - sum3(point_first) * sum3(apex_second);
            // The exe tests the sign bit of the scalar result.
            let negative = determinant.to_bits() >> 31 != 0;
            if negative {
                e.mem.set_f32(w + 4 * first, minus_one);
            } else {
                e.mem.set_f32(w + 4 * second, minus_one);
            }
        }
    }
    for i in 0..3u32 {
        if 0.0 < e.mem.f32(w + 4 * i) {
            return i as i32;
        }
    }
    -1
}

// Translated from 00da8500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::checkTriangleBoundaries` (Xbox PDB), thiscall `(this, query, triangle, update)`.
/// `triangle` holds three vertices at +0/+0x10/+0x20 (and parallel vectors at +0x40/+0x50).
/// Computes the triangle's normal `N` (edge 1-2 cross edge 2-0) and, for the query point,
/// three scaled cross-product sums (the boundary tests against the edges 1-2, 2-0 and 0-1),
/// stores them in `this + 0x160` and returns a three-bit mask of the ones that are negative.
/// With `update` non-zero it also stores `N` at `this + 0x120`; when all three tests are
/// negative and the dot product of `N` with the difference of the vectors at `this + 0x20`
/// and `this + 0xa0` is negative, it flips `N`, swaps vertices 0 and 1 of the triangle (and
/// their parallel vectors at +0x40/+0x50), swaps the first two stored sums and sets
/// `this + 0x14` to 1.
pub fn hkp_gsk_check_triangle_boundaries(
    e: &mut Engine,
    this: Ptr,
    query: Ptr,
    triangle: Ptr,
    update: i32,
) -> u32 {
    let t = this.addr();
    let tr = triangle.addr();
    let p2 = ld(e, tr + 0x20);
    let p1 = ld(e, tr + 0x10);
    let p0 = ld(e, tr);
    let e12 = vsub(p2, p1);
    let e12_zxy = shufps(e12, e12, 0xd2);
    let e12_yzx = shufps(e12, e12, 0xc9);
    let e20 = vsub(p0, p2);
    let e20_yzx = shufps(e20, e20, 0xc9);
    let e20_zxy = shufps(e20, e20, 0xd2);
    let e01 = vsub(p1, p0);
    let q = ld(e, query.addr());
    let normal = vsub(vmul(e12_yzx, e20_zxy), vmul(e12_zxy, e20_yzx));
    let q_p1 = vsub(q, p1);
    let q_p2 = vsub(q, p2);
    let q_p0 = vsub(q, ld(e, tr));
    let c1 = vsub(
        vmul(shufps(q_p1, q_p1, 0xc9), e12_zxy),
        vmul(shufps(q_p1, q_p1, 0xd2), e12_yzx),
    );
    let c2 = vsub(
        vmul(shufps(q_p2, q_p2, 0xc9), e20_zxy),
        vmul(shufps(q_p2, q_p2, 0xd2), e20_yzx),
    );
    let c0 = vsub(
        vmul(shufps(q_p0, q_p0, 0xc9), shufps(e01, e01, 0xd2)),
        vmul(shufps(q_p0, q_p0, 0xd2), shufps(e01, e01, 0xc9)),
    );
    let n1 = vmul(c1, normal);
    let n2 = vmul(c2, normal);
    let n0 = vmul(c0, normal);
    let low = unpcklps(n1, n2);
    let high = unpckhps(n1, n2);
    let x3 = movlhps(low, n0);
    let x2 = shufps(movhlps(x3, low), n0, 0xd4);
    let x0 = shufps(high, n0, 0xe4);
    let sums = vadd(x0, vadd(x2, x3));
    st(e, t + 0x160, sums);
    let mask = (0..3).fold(0u32, |m, i| if sums[i] < 0.0 { m | (1 << i) } else { m });
    if update != 0 {
        st(e, t + 0x120, normal);
        if mask == 7 {
            let toward = vmul(vsub(ld(e, t + 0x20), ld(e, t + 0xa0)), normal);
            if sum3(toward).to_bits() >> 31 != 0 {
                let flipped = xorps(normal, mask_at(e, SIGN_MASK));
                st(e, t + 0x120, flipped);
                for (a, b) in [(tr, tr + 0x10), (tr + 0x40, tr + 0x50)] {
                    let first = [0, 1, 2, 3].map(|i| e.mem.u32(a + 4 * i));
                    copy16(e, b, a);
                    for (i, word) in first.into_iter().enumerate() {
                        e.mem.set_u32(b + 4 * i as u32, word);
                    }
                }
                let kept = load_float(e, t + 0x160);
                let second = load_float(e, t + 0x164);
                e.mem.set_f32(t + 0x160, second);
                e.mem.set_f32(t + 0x164, kept);
                e.mem.set_u32(t + 0x14, 1);
            }
        }
    }
    mask
}

// Translated from 00da8fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `findClosestTriangle` (Xbox PDB). Register convention: `triangle` is EAX (four 16-byte
/// vectors, the first three the triangle's and the fourth the apex), `point` is ESI, and
/// `scale` is a cdecl stack argument. Builds three vectors from cross products of the edges
/// and the scaled difference between the point and the apex, takes `|d| * d` of their combined
/// dot products with the difference, divides each by the squared length of its vector (the
/// large float `0x7f7fffee` when that is zero) and picks the largest. Returns -1 when the
/// largest is below about `1e-6`, its index when it exceeds 1.1 times the second largest,
/// otherwise the result of `findClosestTriangleBackup` (`00da8390`) run on the point with the
/// scaled dot products as weights.
pub fn find_closest_triangle(e: &mut Engine, triangle: Ptr, point: Ptr, scale: f32) -> i32 {
    let tr = triangle.addr();
    let p1 = ld(e, tr + 0x10);
    let p0 = ld(e, tr);
    let p3 = ld(e, tr + 0x30);
    let p2 = ld(e, tr + 0x20);
    let d02 = vsub(p0, p2);
    let d30 = vsub(p3, p0);
    let d21 = vsub(p2, p1);
    let d10 = vsub(p1, p0);
    let d32 = vsub(p3, p2);
    let d31 = vsub(p3, p1);
    let d21_zxy = shufps(d21, d21, 0xd2);
    let d21_yzx = shufps(d21, d21, 0xc9);
    let d31_yzx = shufps(d31, d31, 0xc9);
    let d31_zxy = shufps(d31, d31, 0xd2);
    let a = vsub(vmul(d31_zxy, d21_yzx), vmul(d31_yzx, d21_zxy));
    let d32_yzx = shufps(d32, d32, 0xc9);
    let d32_zxy = shufps(d32, d32, 0xd2);
    let d02_zxy = shufps(d02, d02, 0xd2);
    let d02_yzx = shufps(d02, d02, 0xc9);
    let b = vsub(vmul(d32_zxy, d02_yzx), vmul(d32_yzx, d02_zxy));
    let d30_yzx = shufps(d30, d30, 0xc9);
    let d30_zxy = shufps(d30, d30, 0xd2);
    let d10_zxy = shufps(d10, d10, 0xd2);
    let d10_yzx = shufps(d10, d10, 0xc9);
    let q = ld(e, point.addr());
    let c = vsub(vmul(d30_zxy, d10_yzx), vmul(d30_yzx, d10_zxy));
    let apex = ld(e, tr + 0x30);
    let w = vmul([scale; 4], vsub(q, apex));
    let low = unpcklps(a, b);
    let high = unpckhps(a, b);
    let x5 = movlhps(low, c);
    let x4 = shufps(movhlps(x5, low), c, 0xd4);
    let part_y = vmul(shufps(w, w, 0x55), x4);
    let part_x = vmul(shufps(w, w, 0), x5);
    let part_z = vmul(shufps(high, c, 0xe4), shufps(w, w, 0xaa));
    let d = vadd(vadd(part_y, part_x), part_z);
    let d_squared = vmul(andps(d, mask_at(e, ABS_MASK)), d);
    let big = load_float(e, BIG_FLOAT);
    let mut ratios = [0f32; 3];
    for (i, v) in [a, b, c].into_iter().enumerate() {
        let squares = vmul(v, v);
        let length_squared = x87_quiet(sum3(squares));
        ratios[i] = if length_squared == 0.0 {
            big
        } else {
            (f64::from(d_squared[i]) / f64::from(length_squared)) as f32
        };
    }
    let ratios = ratios.map(x87_quiet);
    let (small, largest_of_two, mut index) = if ratios[1] < ratios[0] {
        (ratios[1], ratios[0], 0)
    } else {
        (ratios[0], ratios[1], 1)
    };
    let (second, largest) = if ratios[2] < largest_of_two {
        if ratios[2] < small {
            (small, largest_of_two)
        } else {
            (ratios[2], largest_of_two)
        }
    } else {
        index = 2;
        (largest_of_two, ratios[2])
    };
    if e.mem.f32(NEAR_ZERO) > largest {
        return -1;
    }
    if f64::from(second) * e.mem.f64(TIE_FACTOR) < f64::from(largest) {
        return index;
    }
    e.with_stack(0x10, |e, weights| {
        st(e, weights.addr(), d);
        find_closest_triangle_backup(e, triangle, point, Ptr::new(weights.addr()))
    })
}

// Translated from 00da9790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::exitAndExportCacheImpl` (Xbox PDB), thiscall `(this, cache)`: writes the two
/// vertex counts as bytes at `cache + 8` and `cache + 9`, then the 16-bit vertex ids of the
/// first simplex (`this + 0x2c`, stride 0x10, `this[0]` of them) followed by those of the
/// second (`this + 0xec`, `this[1]`), each group ordered by the exe's small sorting
/// network (largest first; with three ids the other two are not always in order).
pub fn hkp_gsk_exit_and_export_cache_impl(e: &mut Engine, this: Ptr, cache: Ptr) {
    let t = this.addr();
    let out = cache.addr();
    let first_count = e.mem.u32(t) as i32;
    let second_count = e.mem.u32(t + 4) as i32;
    e.mem.set_u8(out + 8, first_count as u8);
    e.mem.set_u8(out + 9, second_count as u8);
    // The sorted ids of one simplex: (stored at 0, at 1, at 2).
    let sort = |e: &Engine, ids: u32, count: i32| -> (u16, u16, u16) {
        let x0 = e.mem.u16(ids);
        if count < 2 {
            return (x0, 0, 0);
        }
        let x1 = e.mem.u16(ids + 0x10);
        let (mut ax, mut si, mut swaps) = if x1 > x0 { (x1, x0, 1) } else { (x0, x1, 0) };
        let mut dx = 0;
        if count > 2 {
            let x2 = e.mem.u16(ids + 0x20);
            dx = x2;
            if x2 > ax {
                dx = ax;
                ax = x2;
                swaps += 1;
            }
            if swaps == 1 {
                std::mem::swap(&mut dx, &mut si);
            }
        }
        (ax, si, dx)
    };
    let (ax, si, dx) = sort(e, t + 0x2c, first_count);
    if first_count >= 2 {
        if first_count > 2 {
            e.mem.set_u16(out + 4, dx);
        }
        e.mem.set_u16(out + 2, si);
    }
    e.mem.set_u16(out, ax);
    let (ax, si, dx) = sort(e, t + 0xec, second_count);
    let base = out + 2 * first_count as u32;
    if second_count >= 2 {
        if second_count > 2 {
            e.mem.set_u16(base + 4, dx);
        }
        e.mem.set_u16(base + 2, si);
    }
    e.mem.set_u16(base, ax);
}

// Translated from 00da9880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::convertFeatureToClosestDistance` (Xbox PDB), thiscall `(this, input, out)`:
/// copies the 16 bytes at `input` to `out` and the float at `input + 0xc` to `out + 0x20`;
/// then fills `out + 0x10` (a vector) with `this + 0x20` when `this[0] == 1`, else with
/// `input.w * input.xyz + this + 0xa0` (lane-wise, including the fourth lane) when
/// `this[1] == 1`, else with `this + 0x130`.
pub fn hkp_gsk_convert_feature_to_closest_distance(
    e: &mut Engine,
    this: Ptr,
    input: Ptr,
    out: Ptr,
) {
    let t = this.addr();
    let (i, o) = (input.addr(), out.addr());
    let distance = load_float(e, i + 0xc);
    e.mem.set_f32(o + 0x20, distance);
    copy16(e, i, o);
    if e.mem.u32(t) == 1 {
        copy16(e, t + 0x20, o + 0x10);
    } else if e.mem.u32(t + 4) == 1 {
        let w = shufps(ld(e, i), ld(e, i), 0xff);
        let moved = crate::sse::addps(crate::sse::mulps(w, ld(e, o)), ld(e, t + 0xa0));
        st(e, o + 0x10, moved);
    } else {
        copy16(e, t + 0x130, o + 0x10);
    }
}

fn decrement(e: &mut Engine, at: u32) {
    let value = e.mem.u32(at).wrapping_sub(1);
    e.mem.set_u32(at, value);
}

// Translated from 00da9340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::processEdgeTriangle` (Xbox PDB), thiscall `(this, simplex, triangle, counter_a,
/// counter_b, edge_mode, value)`; the callee pops six words after `this`. `simplex` holds
/// two vectors at +0 and +0x10 (with parallel vectors at +0x40/+0x50), `triangle` three at
/// +0/+0x10/+0x20 (parallel ones at +0x40/+0x50/+0x60); the counters are int pointers.
/// With `edge_mode` non-zero: the signed distances of the two simplex points from the
/// triangle's plane are compared; when they differ in sign the crossing point goes through
/// `checkTriangleBoundaries` (`00da8500`) and a full result (7) returns 1; otherwise the
/// simplex is reordered (nearer point first), checked again with the update flag, and a
/// full result decrements `counter_a` and returns 0. If neither settles it, the two
/// segment-to-edge distances (`00d22e40` with the triangle's edges 1-2 and 0-2) pick which
/// triangle vertex (0 or 1) is replaced by vertex 2, `counter_b` is decremented and 2 is
/// returned. With `edge_mode` zero the stored sums at `this + 0x160` and the stored normal
/// at `this + 0x120` are used instead: opposite-side points are checked for a full result
/// (returns 1), the second point is checked (a full result drops the first point, decrements
/// `counter_a`, stores `value` at `this + 0x174` and returns 0), and otherwise the mask picks
/// which triangle vertex is replaced by vertex 2 (a comparison of saved against current
/// sums decides the ambiguous masks 1, 2 and 4), `counter_b` is decremented and 2 returned.
#[allow(clippy::too_many_arguments)]
pub fn hkp_gsk_process_edge_triangle(
    e: &mut Engine,
    this: Ptr,
    simplex: Ptr,
    triangle: Ptr,
    counter_a: Ptr,
    counter_b: Ptr,
    edge_mode: u8,
    value: u32,
) -> u32 {
    let t = this.addr();
    let s = simplex.addr();
    let tr = triangle.addr();
    e.with_stack(0xc0, |e, frame| {
        let f = frame.addr();
        if edge_mode != 0 {
            let t0 = ld(e, tr);
            let t2 = ld(e, tr + 0x20);
            let t1 = ld(e, tr + 0x10);
            let s0 = ld(e, s);
            let e12 = vsub(t1, t2);
            st(e, f + 0x40, e12);
            let e02 = vsub(t0, t2);
            let e12_zxy = shufps(e12, e12, 0xd2);
            let e12_yzx = shufps(e12, e12, 0xc9);
            let e02_yzx = shufps(e02, e02, 0xc9);
            let e02_zxy = shufps(e02, e02, 0xd2);
            let normal = vsub(vmul(e02_zxy, e12_yzx), vmul(e02_yzx, e12_zxy));
            let d0 = sum3(vmul(vsub(s0, t0), normal));
            st(e, f + 0x30, e02);
            let s1 = ld(e, s + 0x10);
            let d1 = sum3(vmul(vsub(s1, t0), normal));
            if (d0.to_bits() ^ d1.to_bits()) >> 31 != 0 {
                let fraction = d0 / (d0 - d1);
                let ones = ld(e, ONES);
                let point = vadd(vmul(vsub(ones, [fraction; 4]), s0), vmul(s1, [fraction; 4]));
                st(e, f + 0x10, point);
                let r = hkp_gsk_check_triangle_boundaries(e, this, Ptr::new(f + 0x10), triangle, 0);
                if r == 7 {
                    return 1;
                }
            } else {
                if x87_quiet(d1.abs()) < x87_quiet(d0.abs()) {
                    for (a, b) in [(s, s + 0x10), (s + 0x40, s + 0x50)] {
                        let first = [0, 1, 2, 3].map(|i| e.mem.u32(a + 4 * i));
                        copy16(e, b, a);
                        for (i, word) in first.into_iter().enumerate() {
                            e.mem.set_u32(b + 4 * i as u32, word);
                        }
                    }
                }
                let r = hkp_gsk_check_triangle_boundaries(e, this, simplex, triangle, 1);
                if r == 7 {
                    decrement(e, counter_a.addr());
                    return 0;
                }
            }
            let s0 = ld(e, s);
            let s1 = ld(e, s + 0x10);
            st(e, f + 0x10, vsub(s1, s0));
            e.call(
                CLOSEST_LINE_SEG_LINE_SEG,
                &args![s, f + 0x10, tr + 0x20, f + 0x40, f + 0x50],
            );
            e.call(
                CLOSEST_LINE_SEG_LINE_SEG,
                &args![s, f + 0x10, tr + 0x20, f + 0x30, f + 0x80],
            );
            let first_distance = load_float(e, f + 0x70);
            let second_distance = load_float(e, f + 0xa0);
            let which = u32::from(second_distance < first_distance);
            copy16(e, tr + 0x20, tr + 16 * which);
            copy16(e, tr + 0x60, tr + 16 * (which + 4));
            decrement(e, counter_b.addr());
            return 2;
        }
        let t0 = ld(e, tr);
        let s1 = ld(e, s + 0x10);
        let normal = ld(e, t + 0x120);
        let saved = [0, 4, 8, 12].map(|o| e.mem.u32(t + 0x160 + o));
        let e1 = sum3(vmul(vsub(s1, t0), normal));
        let s0 = ld(e, s);
        let e0 = sum3(vmul(vsub(s0, t0), normal));
        if (e0 * e1).to_bits() >> 31 != 0 {
            let fraction = e0 / (e0 - e1);
            let ones = ld(e, ONES);
            let point = vadd(vmul(vsub(ones, [fraction; 4]), s0), vmul(s1, [fraction; 4]));
            st(e, f + 0x30, point);
            let r = hkp_gsk_check_triangle_boundaries(e, this, Ptr::new(f + 0x30), triangle, 0);
            if r == 7 {
                return 1;
            }
        }
        let r = hkp_gsk_check_triangle_boundaries(e, this, Ptr::new(s + 0x10), triangle, 0);
        if r == 7 {
            decrement(e, counter_a.addr());
            copy16(e, s + 0x10, s);
            copy16(e, s + 0x50, s + 0x40);
            e.mem.set_u32(t + 0x174, value);
            return 0;
        }
        let s10 = f32::from_bits(saved[0]);
        let s14 = f32::from_bits(saved[1]);
        let s18 = f32::from_bits(saved[2]);
        let c160 = load_float(e, t + 0x160);
        let c164 = load_float(e, t + 0x164);
        let c168 = load_float(e, t + 0x168);
        // The three exits: replace vertex 0, replace vertex 1, or replace nothing.
        let diff = |current: f32, old: f32| (f64::from(current) - f64::from(old)) as f32;
        let wide = |lhs: f32, a: f32, rhs: f32, b: f32| {
            f64::from(lhs) * f64::from(a) > f64::from(rhs) * f64::from(b)
        };
        // 0: vertex 0, 1: vertex 1, 2: neither.
        let exit = match r {
            6 => 0,
            5 => 1,
            3 => 2,
            1 => {
                let f1 = diff(c164, s14);
                let f2 = diff(c168, s18);
                if wide(f1, s18, s14, f2) {
                    2
                } else {
                    1
                }
            }
            2 => {
                let f1 = diff(c168, s18);
                let f2 = diff(c160, s10);
                if wide(f1, s10, s18, f2) {
                    0
                } else {
                    2
                }
            }
            4 => {
                let f1 = diff(c160, s10);
                let f2 = diff(c164, s14);
                if wide(f1, s14, s10, f2) {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        };
        match exit {
            0 => {
                copy16(e, tr + 0x20, tr);
                copy16(e, tr + 0x60, tr + 0x40);
            }
            1 => {
                copy16(e, tr + 0x20, tr + 0x10);
                copy16(e, tr + 0x60, tr + 0x50);
            }
            _ => {}
        }
        decrement(e, counter_b.addr());
        2
    })
}

/// The floating-point tolerance (`010141cc`) passed to `hkCalculatePenetrationDepth`.
const PENETRATION_TOLERANCE: u32 = 0x0101_41cc;

fn count(e: &Engine, at: u32) -> i32 {
    e.mem.i32(at)
}

// Translated from 00da98f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::reduceDimensionExtended` (Xbox PDB), thiscall with no arguments. The two vertex
/// lists are `this + 0x20` (count at `this`) and `this + 0xa0` (count at `this + 4`, with the
/// matching untransformed vectors at `this + 0xe0`). When one list has three vertices and the
/// other point lies inside it (`checkTriangleBoundaries` returns 7), the other list shrinks to
/// that single point. Otherwise the closest pair of edges (`00d22e40`, over all edge pairs of the
/// three-vertex lists and the single points of the others) is found and the vertex of each
/// three-vertex list the pair's edge starts at is replaced by the list's last vertex while the
/// count drops by one.
pub fn hkp_gsk_reduce_dimension_extended(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let first = |e: &Engine, at: u32| signed_byte(e, at) as u32;
    if count(e, t + 4) == 3 {
        let mut i = 0;
        while i < count(e, t) {
            let r = hkp_gsk_check_triangle_boundaries(
                e,
                this,
                Ptr::new(t + 0x20 + 16 * i as u32),
                Ptr::new(t + 0xa0),
                0,
            );
            if r == 7 {
                copy16(e, t + 0x20 + 16 * i as u32, t + 0x20);
                e.mem.set_i32(t, 1);
                return;
            }
            i += 1;
        }
    }
    if count(e, t) == 3 {
        let mut j = 0;
        while j < count(e, t + 4) {
            let r = hkp_gsk_check_triangle_boundaries(
                e,
                this,
                Ptr::new(t + 0xa0 + 16 * j as u32),
                Ptr::new(t + 0x20),
                0,
            );
            if r == 7 {
                copy16(e, t + 0xe0 + 16 * j as u32, t + 0xe0);
                copy16(e, t + 0xa0 + 16 * j as u32, t + 0xa0);
                e.mem.set_i32(t + 4, 1);
                return;
            }
            j += 1;
        }
    }
    let count_a = if count(e, t) == 3 { 3u32 } else { 1 };
    let count_b = if count(e, t + 4) == 3 { 3u32 } else { 1 };
    let mut best_a = 0u32;
    let mut best_b = 0u32;
    e.with_stack(0x90, |e, frame| {
        let f = frame.addr();
        let mut best = load_float(e, BIG_FLOAT);
        for i in 0..count_a {
            for j in 0..count_b {
                let next_a = first(e, EDGE_TABLE_SECOND + i);
                let next_b = first(e, EDGE_TABLE_SECOND + j);
                let a = ld(e, t + 0x20 + 16 * i);
                let a_next = ld(e, t + 0x20 + 16 * next_a);
                st(e, f + 0x50, vsub(a_next, a));
                let b = ld(e, t + 0xa0 + 16 * j);
                let b_next = ld(e, t + 0xa0 + 16 * next_b);
                st(e, f + 0x40, vsub(b_next, b));
                e.call(
                    CLOSEST_LINE_SEG_LINE_SEG,
                    &args![
                        t + 0x20 + 16 * i,
                        f + 0x50,
                        t + 0xa0 + 16 * j,
                        f + 0x40,
                        f + 0x60
                    ],
                );
                let distance = load_float(e, f + 0x80);
                if best > distance {
                    best = distance;
                    best_a = i;
                    best_b = j;
                }
            }
        }
    });
    if count_a == 3 {
        let n = count(e, t) - 1;
        e.mem.set_i32(t, n);
        let slot = first(e, EDGE_TABLE_FIRST + best_a);
        copy16(e, t + 0x20 + 16 * n as u32, t + 0x20 + 16 * slot);
    }
    if count_b == 3 {
        let n = count(e, t + 4) - 1;
        e.mem.set_i32(t + 4, n);
        let slot = first(e, EDGE_TABLE_FIRST + best_b);
        copy16(e, t + 0xe0 + 16 * n as u32, t + 0xe0 + 16 * slot);
        copy16(e, t + 0xa0 + 16 * n as u32, t + 0xa0 + 16 * slot);
    }
}

fn masked_id(e: &Engine, at: u32) -> u32 {
    e.mem.u32(at) & 0xc0ff_ffff
}

// Translated from 00daa7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::handlePenetration` (Xbox PDB), thiscall `(this, shape_a, shape_b, transform)`;
/// the callee pops three words. Pads the two vertex lists (`this + 0x20` with count `this`,
/// `this + 0xa0` with count `this + 4` and the untransformed vectors at `this + 0xe0`) to a
/// simplex of up to four points by duplicating vertices, builds the differences of the lists
/// and calls `hkCalculatePenetrationDepth` (`00dfaac0`) with the tolerance float at
/// `010141cc`. A result of 1 stores counts of 1, the "converged" flag at `this + 0x14` and the
/// result vector at `this + 0x120`. Otherwise the counts are set to three and reduced for
/// repeated vertex ids (the low 24 bits plus the top two, mask `c0ffffff`), the second list is
/// re-transformed from its untransformed vectors with the 4x4 `transform`,
/// `reduceDimensionExtended` is called when the counts exceed four in total, two lists of two
/// get the segment-to-segment closest points at `this + 0x130`, and the result vector and flag
/// are stored.
pub fn hkp_gsk_handle_penetration(
    e: &mut Engine,
    this: Ptr,
    shape_a: Ptr,
    shape_b: Ptr,
    transform: Ptr,
) {
    let t = this.addr();
    let count_a = count(e, t);
    let count_b = count(e, t + 4);
    let mut n = 4u32;
    if count_b >= 3 {
        if count_b > 3 {
            copy16(e, t + 0x20, t + 0x50);
        } else if count_a > 1 {
            copy16(e, t + 0x30, t + 0x50);
            copy16(e, t + 0xe0, t + 0x110);
            copy16(e, t + 0xa0, t + 0xd0);
        } else {
            n = 3;
        }
        copy16(e, t + 0x20, t + 0x30);
        copy16(e, t + 0x20, t + 0x40);
    } else if count_a >= 3 {
        if count_a > 3 {
            copy16(e, t + 0xe0, t + 0x110);
            copy16(e, t + 0xa0, t + 0xd0);
        } else if count_b > 1 {
            copy16(e, t + 0x20, t + 0x50);
            copy16(e, t + 0xf0, t + 0x110);
            copy16(e, t + 0xb0, t + 0xd0);
        } else {
            n = 3;
        }
        copy16(e, t + 0xe0, t + 0xf0);
        copy16(e, t + 0xa0, t + 0xb0);
        copy16(e, t + 0xe0, t + 0x100);
        copy16(e, t + 0xa0, t + 0xc0);
    } else {
        let (na, nb) = (count_a as u32, count_b as u32);
        copy16(e, t + 0x20, t + 0x20 + 16 * na);
        copy16(e, t + 0x20, t + 0x30 + 16 * na);
        copy16(e, t + 0xe0, t + 0xe0 + 16 * nb);
        copy16(e, t + 0xe0, t + 0xf0 + 16 * nb);
        copy16(e, t + 0xa0, t + 0xa0 + 16 * nb);
        copy16(e, t + 0xa0, t + 0xb0 + 16 * nb);
        n = 1;
    }
    e.with_stack(0xd0, |e, frame| {
        let f = frame.addr();
        for k in 0..n {
            let a = ld(e, t + 0x20 + 16 * k);
            let b = ld(e, t + 0xa0 + 16 * k);
            st(e, f + 0x80 + 16 * k, vsub(a, b));
        }
        let tolerance = x87_quiet_bits(e.mem.u32(PENETRATION_TOLERANCE));
        let r = e
            .call(
                CALCULATE_PENETRATION_DEPTH,
                &args![
                    shape_a.addr(),
                    shape_b.addr(),
                    transform.addr(),
                    tolerance,
                    t + 0x20,
                    t + 0xe0,
                    f + 0x80,
                    n,
                    f + 0x50
                ],
            )
            .u32();
        if r == 1 {
            e.mem.set_i32(t, 1);
            e.mem.set_i32(t + 4, 1);
            e.mem.set_u32(t + 0x14, 1);
            copy16(e, f + 0x50, t + 0x120);
            return;
        }
        let id_a2 = masked_id(e, t + 0x4c);
        let id_a1 = masked_id(e, t + 0x3c);
        e.mem.set_i32(t, 3);
        e.mem.set_i32(t + 4, 3);
        if id_a1 == id_a2 || masked_id(e, t + 0x2c) == id_a2 {
            e.mem.set_i32(t, 2);
        }
        if masked_id(e, t + 0x2c) == id_a1 {
            let n = count(e, t) - 1;
            e.mem.set_i32(t, n);
            copy16(e, t + 0x20 + 16 * n as u32, t + 0x20);
        }
        let id_b2 = masked_id(e, t + 0x10c);
        let id_b1 = masked_id(e, t + 0xfc);
        if id_b1 == id_b2 || masked_id(e, t + 0xec) == id_b2 {
            let n = count(e, t + 4) - 1;
            e.mem.set_i32(t + 4, n);
        }
        let nb = count(e, t + 4);
        if nb >= 2 && masked_id(e, t + 0xec) == id_b1 {
            e.mem.set_i32(t + 4, nb - 1);
            copy16(e, t + 0xe0 + 16 * (nb - 1) as u32, t + 0xe0);
        }
        let rows = [0, 0x10, 0x20, 0x30].map(|o| ld(e, transform.addr() + o));
        let mut k = count(e, t + 4) - 1;
        while k >= 0 {
            let at = t + 0xe0 + 16 * k as u32;
            let v = ld(e, at);
            let xy = vadd(vmul([v[1]; 4], rows[1]), vmul([v[0]; 4], rows[0]));
            let zw = vadd(vmul([v[2]; 4], rows[2]), rows[3]);
            st(e, at - 0x40, vadd(xy, zw));
            k -= 1;
        }
        if count(e, t) + count(e, t + 4) > 4 {
            hkp_gsk_reduce_dimension_extended(e, this);
        }
        if count(e, t) == 2 && count(e, t + 4) == 2 {
            let a0 = ld(e, t + 0x20);
            let a1 = ld(e, t + 0x30);
            st(e, f + 0x20, vsub(a1, a0));
            let b0 = ld(e, t + 0xa0);
            let b1 = ld(e, t + 0xb0);
            st(e, f + 0x30, vsub(b1, b0));
            e.call(
                CLOSEST_LINE_SEG_LINE_SEG,
                &args![t + 0x20, f + 0x20, t + 0xa0, f + 0x30, t + 0x130],
            );
        }
        e.mem.set_u32(t + 0x14, 1);
        copy16(e, f + 0x50, t + 0x120);
    });
}

/// The starting tolerance (`010d51c4`) doubled on every pass of `getClosestFeature`.
const FEATURE_TOLERANCE: u32 = 0x010d_51c4;
/// The float constants `0.0` (`01011d78`), `3.0` (`01017718`) and `0.5` (`01016248`) the final
/// normalisation reads.
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;

// Program counters of `getClosestFeature` (the exe's addresses).
const PC_TOP: u32 = 0x00da_ae60;
const PC_DISPATCH: u32 = 0x00da_ae9f;
const PC_EDGE_A2_B3: u32 = 0x00da_aeae;
const PC_EDGE_A3_B2: u32 = 0x00da_aecd;
const PC_SEGMENTS: u32 = 0x00da_af03;
const PC_B4: u32 = 0x00da_afb2;
const PC_B3: u32 = 0x00da_aff5;
const PC_A4: u32 = 0x00da_b125;
const PC_A3: u32 = 0x00da_b158;
const PC_A2: u32 = 0x00da_b677;
const PC_B2: u32 = 0x00da_b54a;
const PC_POINTS: u32 = 0x00da_b75d;
const PC_STORE_KIND: u32 = 0x00da_b771;
const PC_POST: u32 = 0x00da_b260;
const PC_CONVERGED: u32 = 0x00da_bb07;
const PC_FINISH: u32 = 0x00da_bb28;

/// `value * |value|` the way the x87 code forms it (the product in extended precision, stored
/// as a float).
fn signed_square(value: f32) -> f32 {
    let value = x87_quiet(value);
    (f64::from(value) * f64::from(value.abs())) as f32
}

/// A point transformed by four row vectors: `(y*r1 + x*r0) + (z*r2 + r3)`.
fn transform_point(rows: &[Vec4; 4], v: Vec4) -> Vec4 {
    let xy = vadd(vmul([v[1]; 4], rows[1]), vmul([v[0]; 4], rows[0]));
    let zw = vadd(vmul([v[2]; 4], rows[2]), rows[3]);
    vadd(xy, zw)
}

fn negative(value: f32) -> bool {
    value.to_bits() >> 31 != 0
}

/// Asks shape A (or B) for a support point opposite to the contact normal and adds it to the
/// first (second) simplex list when it is far enough out. Shared by the two exe copies of the
/// code for shape A. `dir_at` is where the search direction (the negated normal) is stored.
fn extend_first_list(e: &mut Engine, t: u32, shape: u32, dir_at: u32, threshold: f32) -> bool {
    let normal = ld(e, t + 0x120);
    st(e, dir_at, xorps(normal, mask_at(e, SIGN_MASK)));
    let count = e.mem.u32(t);
    e.vcall(shape, 0x38, &args![dir_at, t + 0x20 + 16 * count]);
    let count = e.mem.u32(t);
    let dir = ld(e, dir_at);
    let new_vertex = ld(e, t + 0x20 + 16 * count);
    let reach = |e: &Engine, k: u32| {
        signed_square(sum3(vmul(vsub(new_vertex, ld(e, t + 0x20 + 16 * k)), dir)))
    };
    let far_enough = threshold < reach(e, 0);
    if !far_enough {
        return false;
    }
    match count {
        2 => {
            if reach(e, 1) < threshold {
                return false;
            }
        }
        3 => {
            if reach(e, 2) < threshold || reach(e, 1) < threshold {
                return false;
            }
        }
        _ => {}
    }
    e.mem.set_u32(t, count.wrapping_add(1));
    e.mem.set_u32(t + 0x14, 1);
    true
}

/// The same for shape B: the search direction is the normal rotated back through the
/// transform's rows, the support point is found in B's own space and then transformed.
fn extend_second_list(
    e: &mut Engine,
    t: u32,
    shape: u32,
    transform: u32,
    dir_at: u32,
    threshold: f32,
) -> bool {
    let r0 = ld(e, transform);
    let r1 = ld(e, transform + 0x10);
    let r2 = ld(e, transform + 0x20);
    let normal = ld(e, t + 0x120);
    let low = unpcklps(r0, r1);
    let high = unpckhps(r0, r1);
    let col_x = movlhps(low, r2);
    let col_y = shufps(movhlps(col_x, low), r2, 0xd4);
    let col_z = shufps(high, r2, 0xe4);
    let dir = vadd(
        vadd(vmul([normal[1]; 4], col_y), vmul([normal[0]; 4], col_x)),
        vmul(col_z, [normal[2]; 4]),
    );
    let count = e.mem.u32(t + 4);
    st(e, dir_at, dir);
    e.vcall(shape, 0x38, &args![dir_at, t + 0xe0 + 16 * count]);
    let count = e.mem.u32(t + 4);
    let dir = ld(e, dir_at);
    let new_vertex = ld(e, t + 0xe0 + 16 * count);
    let reach = |e: &Engine, k: u32| {
        signed_square(sum3(vmul(vsub(new_vertex, ld(e, t + 0xe0 + 16 * k)), dir)))
    };
    let far_enough = threshold < reach(e, 0);
    if far_enough {
        match count {
            2 => {
                if reach(e, 1) < threshold {
                    return false;
                }
            }
            3 => {
                if reach(e, 2) < threshold || reach(e, 1) < threshold {
                    return false;
                }
            }
            _ => {}
        }
        let rows = [0, 0x10, 0x20, 0x30].map(|o| ld(e, transform + o));
        let moved = transform_point(&rows, ld(e, t + 0xe0 + 16 * count));
        st(e, t + 0xa0 + 16 * count, moved);
        let n = e.mem.u32(t + 4).wrapping_add(1);
        e.mem.set_u32(t + 4, n);
        e.mem.set_u32(t + 0x14, 1);
        return true;
    }
    false
}

// Translated from 00daad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::getClosestFeature` (Xbox PDB), thiscall `(this, shape_a, shape_b, transform,
/// result)`; the callee pops four words after `this`. The GJK main loop. `this + 0x20` holds
/// shape A's simplex vertices (count at `this`), `this + 0xa0` the shape B vertices already in
/// A's space (count at `this + 4`), `this + 0xe0` the same vertices in B's own space, and
/// `this + 0x120` is the contact normal. The B vertices are first rebuilt from the `transform`
/// rows (a 4x4 of 16-byte rows). Each pass doubles the tolerance and dispatches on the two
/// counts: the sizes reduce the simplex (nearest-feature tests, `findClosestTriangle`,
/// `processEdgeTriangle`, `checkTriangleBoundaries`, segment-to-segment tests), set the
/// normal and kind (`this + 0x170`) and either end the pass as converged or ask the shapes'
/// support functions (virtual call at slot `0x38`, arguments direction and output vertex) for a
/// new vertex. A converged simplex with `this + 0x10` clear is handed to `handlePenetration`;
/// with it set the function returns 4 after trimming the counts. Otherwise the normal is
/// normalised (refined `RSQRTSS`, zero for a zero normal), copied to `result` with the
/// distance in its fourth lane (also stored at `this + 0x12c`) and 0 is returned.
pub fn hkp_gsk_get_closest_feature(
    e: &mut Engine,
    this: Ptr,
    shape_a: Ptr,
    shape_b: Ptr,
    transform: Ptr,
    result: Ptr,
) -> u32 {
    let t = this.addr();
    let ta = t + 0x20;
    let tb = t + 0xa0;
    let rows = [0u32, 0x10, 0x20, 0x30].map(|o| ld(e, transform.addr() + o));
    let mut k = e.mem.u32(t + 4).wrapping_sub(1);
    loop {
        let v = ld(e, t.wrapping_add(0xe0).wrapping_add(k.wrapping_mul(16)));
        st(
            e,
            t.wrapping_add(0xa0).wrapping_add(k.wrapping_mul(16)),
            transform_point(&rows, v),
        );
        k = k.wrapping_sub(1);
        if (k as i32) < 0 {
            break;
        }
    }
    let a = move |k: u32| ta.wrapping_add(k.wrapping_mul(16));
    let b = move |k: u32| tb.wrapping_add(k.wrapping_mul(16));
    let bp = move |k: u32| (tb + 0x40).wrapping_add(k.wrapping_mul(16));
    let mut tolerance = load_float(e, FEATURE_TOLERANCE);
    e.with_stack(0x100, |e, frame| {
        let f = frame.addr();
        let mut iteration = 0u32;
        #[allow(unused_assignments)]
        let mut previous_sum = e.mem.u32(t).wrapping_add(e.mem.u32(t + 4));
        let mut converged = false;
        let mut pc = PC_TOP;
        loop {
            match pc {
                PC_TOP => {
                    tolerance = x87_quiet(tolerance) * 2.0;
                    iteration += 1;
                    previous_sum = e.mem.u32(t).wrapping_add(e.mem.u32(t + 4));
                    e.mem.set_u32(t + 0x174, 0);
                    pc = PC_DISPATCH;
                }
                PC_DISPATCH => {
                    let key = (e.mem.u32(t).wrapping_mul(8) | e.mem.u32(t + 4)).wrapping_sub(9);
                    pc = if key > 0x18 {
                        converged = true;
                        PC_POST
                    } else {
                        match key {
                            0 => PC_POINTS,
                            1 => PC_B2,
                            2 => PC_B3,
                            3 => PC_B4,
                            8 => PC_A2,
                            9 => PC_SEGMENTS,
                            10 => PC_EDGE_A2_B3,
                            16 => PC_A3,
                            17 => PC_EDGE_A3_B2,
                            24 => PC_A4,
                            _ => {
                                converged = true;
                                PC_POST
                            }
                        }
                    };
                }
                PC_EDGE_A2_B3 | PC_EDGE_A3_B2 => {
                    let flag = u8::from(e.mem.u32(t + 0x170) == 2);
                    let r = if pc == PC_EDGE_A2_B3 {
                        hkp_gsk_process_edge_triangle(
                            e,
                            this,
                            Ptr::new(a(0)),
                            Ptr::new(b(0)),
                            Ptr::new(t),
                            Ptr::new(t + 4),
                            flag,
                            1,
                        )
                    } else {
                        hkp_gsk_process_edge_triangle(
                            e,
                            this,
                            Ptr::new(b(0)),
                            Ptr::new(a(0)),
                            Ptr::new(t + 4),
                            Ptr::new(t),
                            flag,
                            u32::MAX,
                        )
                    };
                    pc = match r {
                        1 => {
                            converged = true;
                            PC_POST
                        }
                        2 => PC_SEGMENTS,
                        _ => PC_STORE_KIND,
                    };
                }
                PC_SEGMENTS => {
                    let a0 = ld(e, a(0));
                    let a1 = ld(e, a(1));
                    st(e, f + 0xa0, vsub(a1, a0));
                    let b0 = ld(e, b(0));
                    let b1 = ld(e, b(1));
                    st(e, f + 0xc0, vsub(b1, b0));
                    let mask = e
                        .call(
                            CLOSEST_LINE_SEG_LINE_SEG,
                            &args![a(0), f + 0xa0, b(0), f + 0xc0, t + 0x130],
                        )
                        .u32();
                    if mask == 0 {
                        // The segments are parallel to nothing nearer: the plane normal.
                        let dir_b = ld(e, f + 0xc0);
                        let dir_a = ld(e, f + 0xa0);
                        let normal = vsub(
                            vmul(shufps(dir_b, dir_b, 0xd2), shufps(dir_a, dir_a, 0xc9)),
                            vmul(shufps(dir_b, dir_b, 0xc9), shufps(dir_a, dir_a, 0xd2)),
                        );
                        st(e, t + 0x120, normal);
                        let side = sum3(vmul(vsub(ld(e, a(0)), ld(e, b(0))), normal));
                        let sign = mask_at(e, SIGN_MASK);
                        let keep = [0, 1, 2, 3].map(|i| side.to_bits() & sign[i]);
                        st(e, t + 0x120, xorps(normal, keep));
                        pc = PC_STORE_KIND;
                    } else {
                        if mask & 1 != 0 {
                            copy16(e, a(1), a(0));
                        }
                        if mask & 3 != 0 {
                            e.mem.set_u32(t, 1);
                        }
                        if mask & 4 != 0 {
                            copy16(e, b(1), b(0));
                            copy16(e, bp(1), bp(0));
                            e.mem.set_u32(t + 4, 1);
                        } else if mask & 8 != 0 {
                            e.mem.set_u32(t + 4, 1);
                        }
                        pc = PC_DISPATCH;
                    }
                }
                PC_B4 => {
                    let r = find_closest_triangle(e, Ptr::new(b(0)), Ptr::new(a(0)), 1.0);
                    if r < 0 {
                        converged = true;
                        pc = PC_POST;
                    } else {
                        let n = e.mem.u32(t + 4).wrapping_sub(1);
                        e.mem.set_u32(t + 4, n);
                        copy16(e, b(3), b(r as u32));
                        copy16(e, bp(3), bp(r as u32));
                        pc = PC_B3;
                    }
                }
                PC_B3 => {
                    let r = hkp_gsk_check_triangle_boundaries(
                        e,
                        this,
                        Ptr::new(a(0)),
                        Ptr::new(b(0)),
                        1,
                    );
                    if r == 7 {
                        pc = PC_STORE_KIND;
                        continue;
                    }
                    let m = signed_byte(e, MASK_TABLE + r);
                    if m < 0 {
                        let mut index = (m + 8) as u32;
                        if index as i32 > 3 {
                            index = 1;
                        }
                        let n = e.mem.u32(t + 4).wrapping_sub(1);
                        e.mem.set_u32(t + 4, n);
                        copy16(e, b(2), b(index));
                        copy16(e, bp(2), bp(index));
                        pc = PC_B2;
                        continue;
                    }
                    let first = signed_byte(e, EDGE_TABLE_FIRST + m as u32) as u32;
                    let second = signed_byte(e, EDGE_TABLE_SECOND + m as u32) as u32;
                    let query = ld(e, a(0));
                    let base = ld(e, b(m as u32));
                    let p_second = ld(e, b(second));
                    let p_first = ld(e, b(first));
                    let to_query = vsub(query, base);
                    let side_first = sum3(vmul(vsub(p_first, base), to_query));
                    let b2 = ld(e, b(2));
                    e.mem.set_u32(t + 4, 2);
                    if negative(side_first) {
                        st(e, b(first), b2);
                        copy16(e, bp(2), bp(first));
                        let side_second = sum3(vmul(vsub(p_second, base), to_query));
                        if negative(side_second) {
                            let mut keep = second;
                            let mut n = e.mem.u32(t + 4);
                            if keep == n {
                                keep = first;
                            }
                            n = n.wrapping_sub(1);
                            e.mem.set_u32(t + 4, n);
                            copy16(e, b(n), b(keep));
                            copy16(e, bp(n), bp(keep));
                        }
                    } else {
                        st(e, b(second), b2);
                        copy16(e, bp(2), bp(second));
                    }
                    pc = PC_DISPATCH;
                }
                PC_A4 => {
                    let scale = load_float(e, MINUS_ONE);
                    let r = find_closest_triangle(e, Ptr::new(a(0)), Ptr::new(b(0)), scale);
                    if r < 0 {
                        converged = true;
                        pc = PC_POST;
                    } else {
                        let n = e.mem.u32(t).wrapping_sub(1);
                        e.mem.set_u32(t, n);
                        copy16(e, a(3), a(r as u32));
                        pc = PC_A3;
                    }
                }
                PC_A3 => {
                    let r = hkp_gsk_check_triangle_boundaries(
                        e,
                        this,
                        Ptr::new(b(0)),
                        Ptr::new(a(0)),
                        -1,
                    );
                    if r == 7 {
                        pc = PC_STORE_KIND;
                        continue;
                    }
                    let m = signed_byte(e, MASK_TABLE + r);
                    if m < 0 {
                        let mut index = (m + 8) as u32;
                        if index as i32 > 3 {
                            index = 1;
                        }
                        let n = e.mem.u32(t).wrapping_sub(1);
                        e.mem.set_u32(t, n);
                        copy16(e, a(2), a(index));
                        pc = PC_A2;
                        continue;
                    }
                    let first = signed_byte(e, EDGE_TABLE_FIRST + m as u32) as u32;
                    let second = signed_byte(e, EDGE_TABLE_SECOND + m as u32) as u32;
                    let query = ld(e, b(0));
                    let base = ld(e, a(m as u32));
                    let p_second = ld(e, a(second));
                    let p_first = ld(e, a(first));
                    let to_query = vsub(query, base);
                    let side_first = sum3(vmul(vsub(p_first, base), to_query));
                    let a2 = ld(e, a(2));
                    e.mem.set_u32(t, 2);
                    if negative(side_first) {
                        st(e, a(first), a2);
                        let side_second = sum3(vmul(vsub(p_second, base), to_query));
                        if negative(side_second) {
                            let mut keep = second;
                            let mut n = e.mem.u32(t);
                            if keep == n {
                                keep = first;
                            }
                            n = n.wrapping_sub(1);
                            e.mem.set_u32(t, n);
                            copy16(e, a(n), a(keep));
                        }
                    } else {
                        st(e, a(second), a2);
                    }
                    pc = PC_DISPATCH;
                }
                PC_B2 => {
                    let b0 = ld(e, b(0));
                    let b1 = ld(e, b(1));
                    let a0 = ld(e, a(0));
                    let edge = vsub(b1, b0);
                    let from_b0 = vsub(b0, a0);
                    let from_b1 = vsub(b1, a0);
                    let negative_b1 = negative(sum3(vmul(from_b1, edge)));
                    let negative_b0 = negative(sum3(vmul(from_b0, edge)));
                    if negative_b1 != negative_b0 {
                        let length_squared = sum3(vmul(edge, edge));
                        let direction = vmul([rsqrt_estimate(length_squared); 4], edge);
                        let cross = vsub(
                            vmul(
                                shufps(from_b1, from_b1, 0xc9),
                                shufps(from_b0, from_b0, 0xd2),
                            ),
                            vmul(
                                shufps(from_b1, from_b1, 0xd2),
                                shufps(from_b0, from_b0, 0xc9),
                            ),
                        );
                        let normal = vsub(
                            vmul(
                                shufps(direction, direction, 0xc9),
                                shufps(cross, cross, 0xd2),
                            ),
                            vmul(
                                shufps(direction, direction, 0xd2),
                                shufps(cross, cross, 0xc9),
                            ),
                        );
                        st(e, t + 0x120, normal);
                        pc = PC_STORE_KIND;
                    } else {
                        if negative_b1 {
                            st(e, b(0), b1);
                            copy16(e, bp(1), bp(0));
                        }
                        let n = e.mem.u32(t + 4).wrapping_sub(1);
                        e.mem.set_u32(t + 4, n);
                        pc = PC_POINTS;
                    }
                }
                PC_A2 => {
                    let a1 = ld(e, a(1));
                    let a0 = ld(e, a(0));
                    let b0 = ld(e, b(0));
                    let edge = vsub(a1, a0);
                    let from_b0 = vsub(a0, b0);
                    let to_a1 = vsub(a1, b0);
                    let negative_a1 = negative(sum3(vmul(to_a1, edge)));
                    let negative_a0 = negative(sum3(vmul(from_b0, edge)));
                    if negative_a1 != negative_a0 {
                        let length_squared = sum3(vmul(edge, edge));
                        let direction = vmul([rsqrt_estimate(length_squared); 4], edge);
                        let cross = vsub(
                            vmul(shufps(from_b0, from_b0, 0xd2), shufps(to_a1, to_a1, 0xc9)),
                            vmul(shufps(from_b0, from_b0, 0xc9), shufps(to_a1, to_a1, 0xd2)),
                        );
                        let normal = vsub(
                            vmul(
                                shufps(cross, cross, 0xc9),
                                shufps(direction, direction, 0xd2),
                            ),
                            vmul(
                                shufps(cross, cross, 0xd2),
                                shufps(direction, direction, 0xc9),
                            ),
                        );
                        st(e, t + 0x120, normal);
                        pc = PC_STORE_KIND;
                    } else {
                        if negative_a1 {
                            let parallel = ld(e, a(5));
                            st(e, a(0), a1);
                            st(e, a(4), parallel);
                        }
                        let n = e.mem.u32(t).wrapping_sub(1);
                        e.mem.set_u32(t, n);
                        pc = PC_POINTS;
                    }
                }
                PC_POINTS => {
                    let normal = vsub(ld(e, a(0)), ld(e, b(0)));
                    st(e, t + 0x120, normal);
                    pc = PC_STORE_KIND;
                }
                PC_STORE_KIND => {
                    let n = e.mem.u32(t + 4);
                    e.mem.set_u32(t + 0x170, n);
                    converged = false;
                    pc = PC_POST;
                }
                PC_POST => {
                    let normal = ld(e, t + 0x120);
                    let normal_squared = sum3(vmul(normal, normal));
                    let na = e.mem.u32(t);
                    let nb = e.mem.u32(t + 4);
                    let grown = na.wrapping_sub(previous_sum).wrapping_add(nb);
                    let flags = e.mem.u32(t + 0x14) | grown;
                    e.mem.set_u32(t + 0x14, flags);
                    let along = sum3(vmul(vsub(ld(e, a(0)), ld(e, b(0))), normal));
                    if converged {
                        pc = PC_CONVERGED;
                        continue;
                    }
                    let nn = f64::from(x87_quiet(normal_squared));
                    let dd = x87_quiet(along);
                    let scaled = nn * f64::from(tolerance);
                    if f64::from(dd) * f64::from(dd.abs()) <= scaled || f64::from(tolerance) > nn {
                        pc = PC_CONVERGED;
                        continue;
                    }
                    let threshold = scaled as f32;
                    let odd = iteration & 1 != 0;
                    if odd
                        && na != e.mem.u32(t + 8)
                        && e.mem.u32(t + 0x174) != 1
                        && extend_first_list(e, t, shape_a.addr(), f + 0x80, threshold)
                    {
                        pc = PC_TOP;
                        continue;
                    }
                    let nb = e.mem.u32(t + 4);
                    if nb != e.mem.u32(t + 0xc)
                        && e.mem.u32(t + 0x174) as i32 != -1
                        && extend_second_list(
                            e,
                            t,
                            shape_b.addr(),
                            transform.addr(),
                            f + 0x60,
                            threshold,
                        )
                    {
                        pc = PC_TOP;
                        continue;
                    }
                    let na = e.mem.u32(t);
                    if !odd
                        && na != e.mem.u32(t + 8)
                        && e.mem.u32(t + 0x174) != 1
                        && extend_first_list(e, t, shape_a.addr(), f + 0xe0, threshold)
                    {
                        pc = PC_TOP;
                        continue;
                    }
                    pc = PC_FINISH;
                }
                PC_CONVERGED => {
                    if e.mem.u32(t + 0x10) == 0 {
                        hkp_gsk_handle_penetration(e, this, shape_a, shape_b, transform);
                        pc = PC_FINISH;
                    } else {
                        let nb = e.mem.u32(t + 4);
                        let na = e.mem.u32(t);
                        if (na.wrapping_add(nb) as i32) > 4 {
                            if (na as i32) > (nb as i32) {
                                e.mem.set_u32(t, 3);
                                e.mem.set_u32(t + 4, 1);
                            } else {
                                e.mem.set_u32(t, 1);
                                e.mem.set_u32(t + 4, 3);
                            }
                        }
                        return 4;
                    }
                }
                _ => {
                    // PC_FINISH
                    let normal = ld(e, t + 0x120);
                    let difference = vsub(ld(e, a(0)), ld(e, b(0)));
                    let zero = e.mem.f32(ZERO);
                    let three = e.mem.f32(THREE);
                    let half = e.mem.f32(HALF);
                    let length_squared = sum3(vmul(normal, normal));
                    let estimate = rsqrt_estimate(length_squared);
                    let step = three - (length_squared * estimate) * estimate;
                    let refined = (half * estimate) * step;
                    let scale = if length_squared == zero { 0.0 } else { refined };
                    st(e, t + 0x120, vmul([scale; 4], normal));
                    copy16(e, t + 0x120, result.addr());
                    let distance = x87_quiet(sum3(vmul(ld(e, t + 0x120), difference)));
                    e.mem.set_f32(result.addr() + 0xc, distance);
                    e.mem.set_f32(t + 0x12c, distance);
                    return 0;
                }
            }
        }
    })
}

/// The `double` factor `0.001` (`01016978`) of the edge-edge tests, the `double` `1000.0`
/// (`01017b70`) of the point-point test and the vector of zeros at `010c7160` whose first lane
/// the point-feature cases subtract the distance from.
const EDGE_TOLERANCE: u32 = 0x0101_6978;
const POINT_TOLERANCE: u32 = 0x0101_7b70;
const ZERO_VECTOR: u32 = 0x010c_7160;
/// The shapes' virtual function at vtable offset `0x3c` (vertex ids, count, output vertices).
const SHAPE_VERTICES: u32 = 0x3c;

/// The refined `RSQRTSS` of the exe: `0.5 * r * (3 - x * r * r)` for `r` the estimate of
/// `1 / sqrt(x)`, and 0 when `x` equals the `0.0` of the image (`CMPEQSS` then `ANDNPS`).
fn refined_rsqrt(e: &Engine, length_squared: f32) -> f32 {
    let estimate = rsqrt_estimate(length_squared);
    let half = e.mem.f32(HALF);
    let three = e.mem.f32(THREE);
    if length_squared == e.mem.f32(ZERO) {
        0.0
    } else {
        (half * estimate) * (three - (length_squared * estimate) * estimate)
    }
}

/// `v` with the sign bit of `reference` applied to every lane (the exe splats the dot product,
/// masks it with the sign bits at `010c44d0` and exclusive-ors the vector with the result).
fn with_sign_of(e: &Engine, v: Vec4, reference: f32) -> Vec4 {
    let sign = andps([reference; 4], mask_at(e, SIGN_MASK));
    xorps(v, sign.map(f32::to_bits))
}

/// The normal of a triangle's three vertices, `(p0 - p2) x (p2 - p1)` built from shuffles and
/// multiplies in the exe's order.
fn triangle_normal(e: &Engine, triangle: u32) -> Vec4 {
    let p2 = ld(e, triangle + 0x20);
    let p1 = ld(e, triangle + 0x10);
    let p0 = ld(e, triangle);
    let side = vsub(p2, p1);
    let back = vsub(p0, p2);
    vsub(
        vmul(shufps(back, back, 0xd2), shufps(side, side, 0xc9)),
        vmul(shufps(back, back, 0xc9), shufps(side, side, 0xd2)),
    )
}

/// `y * row1 + x * row0` plus `z * row2` for the vector `v`.
fn rotate_by_rows(v: Vec4, rows: [Vec4; 3]) -> Vec4 {
    [0usize, 1, 2, 3].map(|i| (v[1] * rows[1][i] + v[0] * rows[0][i]) + v[2] * rows[2][i])
}

/// The nearest points of two segments and the contact normal built from their directions,
/// shared by `00da8820` and the edge-edge case of `hkGskRecalcContactInternal`. `segment_a`
/// and `segment_b` hold the two end points at +0 and +0x10. The segment-pair routine
/// `00d22e40` writes the closest point (a vector) into a local; the direction cross product
/// is the normal when it is large enough (`|cross|^2 * 0.001` above the squared sum of the
/// squared directions), signed by `reference` and normalised, otherwise the normal is
/// `reference`. The closest point goes to `closest_out`, the normal to `normal_out` with its
/// fourth lane replaced by the distance along the normal from `segment_b` to `segment_a`.
fn segment_segment_contact(
    e: &mut Engine,
    segment_a: u32,
    segment_b: u32,
    reference: u32,
    normal_out: u32,
    closest_out: u32,
) {
    // The closest-points result of 00d22e40 is 0x30 bytes (its distance at +0x20), so the frame
    // runs to +0x50 rather than ending inside it.
    e.with_stack(0x50, |e, frame| {
        let x = frame.addr();
        let (direction_b, direction_a, closest) = (x, x + 0x10, x + 0x20);
        let a0 = ld(e, segment_a);
        let direction = vsub(ld(e, segment_a + 0x10), a0);
        st(e, direction_a, direction);
        let b0 = ld(e, segment_b);
        let direction = vsub(ld(e, segment_b + 0x10), b0);
        st(e, direction_b, direction);
        e.call(
            CLOSEST_LINE_SEG_LINE_SEG,
            &args![segment_a, direction_a, segment_b, direction_b, closest],
        );
        let dir_b = ld(e, direction_b);
        let dir_a = ld(e, direction_a);
        let squares = vadd(vmul(dir_b, dir_b), vmul(dir_a, dir_a));
        let normal = vsub(
            vmul(shufps(dir_b, dir_b, 0xd2), shufps(dir_a, dir_a, 0xc9)),
            vmul(shufps(dir_b, dir_b, 0xc9), shufps(dir_a, dir_a, 0xd2)),
        );
        let direction_squares = sum3(squares);
        let normal_squares = sum3(vmul(normal, normal));
        st(e, normal_out, normal);
        let limit = f64::from(direction_squares) * f64::from(direction_squares);
        if f64::from(normal_squares) * e.mem.f64(EDGE_TOLERANCE) > limit {
            let dot = sum3(vmul(ld(e, reference), normal));
            let signed = with_sign_of(e, normal, dot);
            let scale = refined_rsqrt(e, sum3(vmul(signed, signed)));
            st(e, normal_out, vmul([scale; 4], signed));
        } else {
            copy16(e, reference, normal_out);
        }
        let distance = sum3(vmul(
            vsub(ld(e, segment_a), ld(e, segment_b)),
            ld(e, normal_out),
        ));
        copy16(e, closest, closest_out);
        let normal = ld(e, normal_out);
        st(e, normal_out, [normal[0], normal[1], normal[2], distance]);
    });
}

// Translated from 00da8700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00da8700`), a triangle-plane helper with a register convention:
/// `triangle` is ECX (three 16-byte vertices), `normal_out` EDX, `reference` ESI (a vector),
/// then the cdecl stack words `distance_out` and `point`. Builds the triangle normal
/// (`triangle_normal`), normalises it with the refined `RSQRTSS` (zero for a zero normal),
/// flips it to face `reference` (the sign of their dot product) and stores it in `normal_out`.
/// `distance_out` receives the signed distance of `point` from the plane in its first lane and
/// the product of the two z components in the other three.
pub fn fn_00da8700(
    e: &mut Engine,
    triangle: Ptr,
    normal_out: Ptr,
    reference: Ptr,
    distance_out: Ptr,
    point: Ptr,
) {
    let tr = triangle.addr();
    let normal = triangle_normal(e, tr);
    let scale = refined_rsqrt(e, sum3(vmul(normal, normal)));
    st(e, normal_out.addr(), normal);
    let dot = sum3(vmul(normal, ld(e, reference.addr())));
    let facing = vmul(with_sign_of(e, normal, dot), [scale; 4]);
    st(e, normal_out.addr(), facing);
    let offset = vmul(vsub(ld(e, point.addr()), ld(e, tr)), facing);
    st(
        e,
        distance_out.addr(),
        [sum3(offset), offset[2], offset[2], offset[2]],
    );
}

// Translated from 00da8820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00da8820`), a segment-segment contact helper with a register convention:
/// `segment_b` is EDI and `normal_out` ESI (each segment's two end points are 16-byte vectors
/// at +0 and +0x10); the cdecl stack words are `segment_a`, `reference` (a vector) and
/// `closest_out`. See `segment_segment_contact`.
pub fn fn_00da8820(
    e: &mut Engine,
    segment_b: Ptr,
    normal_out: Ptr,
    segment_a: Ptr,
    reference: Ptr,
    closest_out: Ptr,
) {
    segment_segment_contact(
        e,
        segment_a.addr(),
        segment_b.addr(),
        reference.addr(),
        normal_out.addr(),
        closest_out.addr(),
    );
}

/// The point-point case of `hkGskRecalcContactInternal` (both simplexes have one vertex): the
/// normal is `reference` when the squared distance exceeds 1000 times the dot product of
/// `reference` with the difference, otherwise the normalised difference; its fourth lane is the
/// dot product, or the squared length times the normalising factor.
fn recalc_point_point(e: &mut Engine, a: u32, b: u32, reference: u32, out: (u32, u32)) {
    let (out_point, out_normal) = out;
    let diff = vsub(ld(e, a), ld(e, b));
    st(e, out_normal, diff);
    let projected = sum3(vmul(ld(e, reference), diff));
    let squared = sum3(vmul(diff, diff));
    let limit = f64::from(projected) * e.mem.f64(POINT_TOLERANCE);
    let value = if f64::from(squared) > limit {
        copy16(e, reference, out_normal);
        projected
    } else {
        let scale = refined_rsqrt(e, squared);
        st(e, out_normal, vmul([scale; 4], diff));
        sum3(vmul(diff, diff)) * scale
    };
    e.mem.set_f32(out_normal + 0xc, x87_quiet(value));
    copy16(e, a, out_point);
}

/// The triangle-vertex cases of `hkGskRecalcContactInternal` (`triangle` has three vertices
/// and `vertex` one). The normal is the triangle's, normalised and facing `reference`; the
/// signed distance of the vertex from the first triangle vertex along it is stored in the
/// normal's fourth lane. With `vertex_is_first` (the first simplex is the single vertex) the
/// contact point is the vertex; otherwise the vertex moved onto the plane and the distance is
/// negated.
fn recalc_triangle(
    e: &mut Engine,
    triangle: u32,
    vertex: u32,
    reference: u32,
    out: (u32, u32),
    vertex_is_first: bool,
) {
    let (out_point, out_normal) = out;
    let raw = triangle_normal(e, triangle);
    let scale = refined_rsqrt(e, sum3(vmul(raw, raw)));
    st(e, out_normal, raw);
    let dot = sum3(vmul(ld(e, reference), raw));
    let normal = vmul(with_sign_of(e, raw, dot), [scale; 4]);
    st(e, out_normal, normal);
    let v = ld(e, vertex);
    let distance = sum3(vmul(vsub(v, ld(e, triangle)), normal));
    st(e, out_point, v);
    if vertex_is_first {
        e.mem.set_f32(out_normal + 0xc, x87_quiet(distance));
        return;
    }
    let moved = vsub(v, vmul([distance; 4], ld(e, out_normal)));
    st(e, out_point, moved);
    let negated = e.mem.f32(ZERO_VECTOR) - distance;
    e.mem.set_f32(out_normal + 0xc, x87_quiet(negated));
}

// Translated from 00da8a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGskRecalcContactInternal` (Xbox PDB), cdecl `(a, b, count_a, count_b, reference,
/// out_point, out_normal)`: recomputes the contact of two simplexes (vertices as 16-byte
/// vectors, `count_a` and `count_b` of them, 1 to 3) against the `reference` direction.
/// Dispatches on `count_a * 8 | count_b` through a jump table (`00da8f74` with the byte
/// index table at `00da8f90`) with six cases; every other combination (keys 12 to 16 and
/// anything outside 9 to 25) returns without writing anything:
/// - 9 (1,1) and 10 (1,2), 17 (2,1): point-point, point-edge and edge-point, see
///   `recalc_point_point` and the inline cases below;
/// - 11 (1,3) and 25 (3,1): a vertex against a triangle, `recalc_triangle`;
/// - 18 (2,2): two edges, `segment_segment_contact`.
///
/// The contact point goes to `out_point` and the normal, with the distance in its fourth
/// lane, to `out_normal`.
#[allow(clippy::too_many_arguments)]
pub fn hk_gsk_recalc_contact_internal(
    e: &mut Engine,
    a: Ptr,
    b: Ptr,
    count_a: u32,
    count_b: u32,
    reference: Ptr,
    out_point: Ptr,
    out_normal: Ptr,
) {
    let (a, b) = (a.addr(), b.addr());
    let (reference, out_point, out_normal) =
        (reference.addr(), out_point.addr(), out_normal.addr());
    let key = count_a.wrapping_mul(8) | count_b;
    let out = (out_point, out_normal);
    match key.wrapping_sub(9) {
        0 => recalc_point_point(e, a, b, reference, out),
        1 => {
            // A single vertex against an edge: the normal is `reference` and its fourth lane
            // the dot product of `reference` with the vertex minus the edge's first end.
            let diff = vsub(ld(e, a), ld(e, b));
            copy16(e, reference, out_normal);
            let normal = ld(e, reference);
            let dot = sum3(vmul(diff, normal));
            e.mem.set_f32(out_normal + 0xc, x87_quiet(dot));
            copy16(e, a, out_point);
        }
        2 => recalc_triangle(e, b, a, reference, out, true),
        8 => {
            // An edge against a single vertex: the normal is `reference`, the contact point
            // the vertex moved onto the edge's line by the dot product, the normal's fourth
            // lane the negated dot product.
            let diff = vsub(ld(e, b), ld(e, a));
            copy16(e, reference, out_normal);
            let normal = ld(e, reference);
            let dot = sum3(vmul(diff, normal));
            copy16(e, b, out_point);
            let moved = vsub(ld(e, b), vmul([dot; 4], ld(e, out_normal)));
            st(e, out_point, moved);
            let negated = e.mem.f32(ZERO_VECTOR) - dot;
            e.mem.set_f32(out_normal + 0xc, x87_quiet(negated));
        }
        9 => segment_segment_contact(e, a, b, reference, out_normal, out_point),
        16 => recalc_triangle(e, a, b, reference, out, false),
        // Keys 12 to 16 (indexes 3 to 7), 19 to 24 and anything above 25 fall to the table's
        // default, which only returns.
        _ => {}
    }
}

// Translated from 00da9210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00da9210`), a segment-reduction helper with a register convention: `simplex`
/// is ECX (vertices at +0 and +0x10, parallel vectors at +0x40 and +0x50), `counter` EDX (a
/// 32-bit count), `mode` ESI and `out` EDI; the cdecl stack word is `point`. Takes the signs
/// of the dot products of the segment direction with the two end points' offsets from `point`.
/// With different signs the point is inside the segment: `out` receives the perpendicular
/// built from the cross product of the two offsets and the normalised direction, in one
/// orientation when `mode` is 1 and the opposite one otherwise. With equal signs the nearer
/// end is kept (when both are negative the second end and its parallel vector replace the
/// first), the count is decremented and `out` receives `point - first` (mode 1) or `first -
/// point`.
pub fn fn_00da9210(e: &mut Engine, simplex: Ptr, counter: Ptr, mode: u32, out: Ptr, point: Ptr) {
    let s = simplex.addr();
    let p = ld(e, point.addr());
    let v1 = ld(e, s + 0x10);
    let v0 = ld(e, s);
    let edge = vsub(v1, v0);
    let from_end = vsub(v1, p);
    let from_start = vsub(v0, p);
    let end_dot = sum3(vmul(from_end, edge));
    let start_dot = sum3(vmul(from_start, edge));
    let end_negative = end_dot.to_bits() >> 31;
    let start_negative = start_dot.to_bits() >> 31;
    if end_negative != start_negative {
        let estimate = rsqrt_estimate(sum3(vmul(edge, edge)));
        let unit = vmul([estimate; 4], edge);
        let cross = vsub(
            vmul(
                shufps(from_start, from_start, 0xd2),
                shufps(from_end, from_end, 0xc9),
            ),
            vmul(
                shufps(from_start, from_start, 0xc9),
                shufps(from_end, from_end, 0xd2),
            ),
        );
        let first = vmul(shufps(cross, cross, 0xc9), shufps(unit, unit, 0xd2));
        let second = vmul(shufps(cross, cross, 0xd2), shufps(unit, unit, 0xc9));
        let result = if mode == 1 {
            vsub(second, first)
        } else {
            vsub(first, second)
        };
        st(e, out.addr(), result);
        return;
    }
    if end_negative != 0 {
        copy16(e, s + 0x10, s);
        copy16(e, s + 0x50, s + 0x40);
    }
    let count = e.mem.u32(counter.addr()).wrapping_sub(1);
    e.mem.set_u32(counter.addr(), count);
    let first = ld(e, s);
    let result = if mode == 1 {
        vsub(p, first)
    } else {
        vsub(first, p)
    };
    st(e, out.addr(), result);
}

// Translated from 00dabca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::getClosestPoint` (Xbox PDB), cdecl `(record, cache, normal_out, point_out)`.
/// `record` is `(transform, motion, shape A, shape B, limit)`: the relative transform rows at
/// +0, the first body's motion (rows of its rotation and position) at +4, the two shapes at
/// +8 and +0xc and a float limit at +0x10. Builds a local GSK object from the cache (the two
/// counts at +8 and +9, the flag byte at +0xa), converts both shapes' vertex ids (vtable
/// offset `0x3c`) and runs `getClosestFeature`; a changed cache is exported
/// (`exitAndExportCacheImpl`). `normal_out` receives the solver's vector rotated by the first
/// motion with the gap (distance minus both radii) in its fourth lane. Returns 1 when the
/// gap is not below the limit. Otherwise it moves the solver's vector by the radius of the
/// second shape (the distance and radius of the nearer simplex: with one vertex in the first
/// simplex `radius_b - distance`, with one vertex in the second `radius_b`, else `radius_b -
/// distance`), adds the matching vertex (`+0x40`, `+0xc0` or `+0x150` of the frame), places it
/// with the first motion into `point_out` and returns 0.
pub fn hkp_gsk_get_closest_point(
    e: &mut Engine,
    record: Ptr,
    cache: Ptr,
    normal_out: Ptr,
    point_out: Ptr,
) -> u32 {
    let (r, c) = (record.addr(), cache.addr());
    let shape_b = e.mem.u32(r + 0xc);
    let shape_a = e.mem.u32(r + 8);
    e.with_stack(0x400, |e, frame| {
        let x = frame.addr();
        let (result, object) = (x + 0x10, x + 0x20);
        let count_a = u32::from(e.mem.u8(c + 8));
        let count_b = u32::from(e.mem.u8(c + 9));
        let flags = u32::from(e.mem.u8(c + 0xa));
        e.mem.set_u32(object + 0x14, 0);
        e.mem.set_u32(object + 0x10, 0);
        e.mem.set_u32(object, count_a);
        e.mem.set_u32(object + 4, count_b);
        e.mem.set_u32(object + 8, flags & 0xf);
        e.mem.set_u32(object + 0xc, flags >> 4);
        e.vcall(shape_a, SHAPE_VERTICES, &args![c, count_a, x + 0x40]);
        let second_count = e.mem.u32(object + 4);
        let first_count = e.mem.u32(object);
        e.vcall(
            shape_b,
            SHAPE_VERTICES,
            &args![c + first_count * 2, second_count, x + 0x100],
        );
        let transform = e.mem.u32(r);
        hkp_gsk_get_closest_feature(
            e,
            Ptr::new(object),
            Ptr::new(shape_a),
            Ptr::new(shape_b),
            Ptr::new(transform),
            Ptr::new(result),
        );
        if e.mem.u32(object + 0x14) != 0 {
            hkp_gsk_exit_and_export_cache_impl(e, Ptr::new(object), cache);
        }
        let motion = e.mem.u32(r + 4);
        let distance = load_float(e, result + 0xc);
        let vector = ld(e, result);
        let rows = [ld(e, motion), ld(e, motion + 0x10), ld(e, motion + 0x20)];
        let radius_a = e.mem.f32(shape_a + 0x10);
        let radius_b = e.mem.f32(shape_b + 0x10);
        let gap = ((f64::from(distance) - f64::from(radius_a)) - f64::from(radius_b)) as f32;
        st(e, normal_out.addr(), rotate_by_rows(vector, rows));
        e.mem.set_f32(normal_out.addr() + 0xc, gap);
        // `FCOMPP` of the limit against the gap, then a mask test: only "limit above gap" goes on.
        let limit = f64::from(e.mem.f32(r + 0x10));
        if limit.partial_cmp(&f64::from(gap)) != Some(std::cmp::Ordering::Greater) {
            return 1;
        }
        let (factor, vertex_at) = if e.mem.u32(object) == 1 {
            let t = (f64::from(radius_b) - f64::from(distance)) as f32;
            (t, x + 0x40)
        } else if e.mem.u32(object + 4) == 1 {
            (x87_quiet(radius_b), x + 0xc0)
        } else {
            let t = (f64::from(radius_b) - f64::from(distance)) as f32;
            (t, x + 0x150)
        };
        let moved = vadd(vmul([factor; 4], vector), ld(e, vertex_at));
        let motion = e.mem.u32(r + 4);
        let (row0, row1) = (ld(e, motion), ld(e, motion + 0x10));
        let (row2, row3) = (ld(e, motion + 0x20), ld(e, motion + 0x30));
        let placed = [0usize, 1, 2, 3]
            .map(|i| (moved[1] * row1[i] + moved[0] * row0[i]) + (moved[2] * row2[i] + row3[i]));
        st(e, point_out.addr(), placed);
        0
    })
}

/// `TlsGetValue` import slot and the TLS index of Havok's monitor (timer) stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// The timer labels `castRay` records around its cast.
const LABEL_CAST_RAY_START: u32 = 0x010d_51b4;
const LABEL_CAST_RAY_END: u32 = 0x010c_41f8;
/// `009c1aa0` (thiscall `(this, source)`): copies a 0x50-byte hit record.
const COPY_HIT_RECORD: u32 = 0x009c_1aa0;

/// Low word of the time-stamp counter (`RDTSC`); the engine has no model of it and the host
/// clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte timer record `(label, time stamp)` to the thread's monitor stream (the
/// `TlsGetValue` of the index at `0126835c`) while its cursor (+4) is below its limit (+0xC).
fn monitor_record(e: &mut Engine, label: u32) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 0xc);
    }
}

/// The `double` limit `1e14` (`010d51a8`) the squared radius must stay under for the cast to
/// start, the `float` `1e7` (`010d51b0`) the nearest distance starts at, and the `double`
/// `1e-10` (`0109cc78`) below which two successive normals count as unchanged.
const LINEAR_CAST_BOUND_LIMIT: u32 = 0x010d_51a8;
const LINEAR_CAST_START_DISTANCE: u32 = 0x010d_51b0;
const LINEAR_CAST_STALL_LIMIT: u32 = 0x0109_cc78;
/// `hkVector4::setRotatedDir` (engine map name, `00c7f700`, thiscall on the result vector with
/// `(transform, vector)`).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;

// Program counters of `linearCast` (the exe's addresses).
const LC_LOOP: u32 = 0x00da_9d10;
const LC_DISPATCH: u32 = 0x00da_a302;
const LC_EDGE_A2_B3: u32 = 0x00da_9f9e;
const LC_EDGE_A3_B2: u32 = 0x00da_9fb5;
const LC_SEGMENTS: u32 = 0x00da_9fe8;
const LC_B4: u32 = 0x00da_a08e;
const LC_B3: u32 = 0x00da_a0ce;
const LC_A4: u32 = 0x00da_a1f8;
const LC_A3: u32 = 0x00da_a227;
const LC_B2: u32 = 0x00da_a424;
const LC_A2: u32 = 0x00da_a533;
const LC_POINTS: u32 = 0x00da_a617;
const LC_POST: u32 = 0x00da_a628;
const LC_EXIT: u32 = 0x00da_a31c;
const LC_MISS: u32 = 0x00da_a787;

/// A direction rotated back through the transform: the exe builds the three columns of the
/// 3x3 part of the rows at `transform` with shuffles and returns
/// `n.y * col_y + n.x * col_x + col_z * n.z`.
fn rotate_back(e: &Engine, transform: u32, n: Vec4) -> Vec4 {
    let r0 = ld(e, transform);
    let r1 = ld(e, transform + 0x10);
    let r2 = ld(e, transform + 0x20);
    let low = unpcklps(r0, r1);
    let high = unpckhps(r0, r1);
    let col_x = movlhps(low, r2);
    let col_y = shufps(movhlps(col_x, low), r2, 0xd4);
    let col_z = shufps(high, r2, 0xe4);
    vadd(
        vadd(vmul([n[1]; 4], col_y), vmul([n[0]; 4], col_x)),
        vmul(col_z, [n[2]; 4]),
    )
}

// Translated from 00da9b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::linearCast` (Xbox PDB), thiscall `(this, found, shape_a, shape_b, transform, ray,
/// result)`; the callee pops six words after `this`. `ray` holds the start vector at +0 and the
/// end vector at +0x10; `transform` is shape B's rows (unused when `shape_b` is null); `result`
/// takes the hit normal at +0 and the hit fraction at +0x10. The conservative-advancement
/// version of the closest-feature loop: the Minkowski simplex (A's vertices at `this + 0x20`,
/// B's at `this + 0xa0` and `this + 0xe0`) is grown from the support points of the shapes for
/// the current separation vector (`this + 0x120`) at the current point of the ray. A support
/// point is dotted with the normal; when the ray still approaches the surface (`dot > 0`) and
/// moves towards it, the fraction advances by `dot / (normal . direction)` (a fraction of one
/// or more, or a ray moving away, ends the cast with `*found = 0`). The simplex is then
/// reduced exactly as `getClosestFeature` does (the same case table, `findClosestTriangle`,
/// `checkTriangleBoundaries`, `processEdgeTriangle`, the segment-to-segment routine), the
/// separation vector is negated and normalised, the nearest distance of A's vertices along it
/// is tracked, and the loop repeats while the squared nearest distance stays above the squared
/// radius sum (plus `0.001`) and the normal is still changing (three unchanged passes stop it).
/// At the end the hit is stored into `result` when its fraction is below the one found.
#[allow(clippy::too_many_arguments)]
pub fn hkp_gsk_linear_cast(
    e: &mut Engine,
    this: Ptr,
    found: Ptr,
    shape_a: Ptr,
    shape_b: Ptr,
    transform: Ptr,
    ray: Ptr,
    result: Ptr,
) {
    let t = this.addr();
    let ta = t + 0x20;
    let tb = t + 0xa0;
    let a = move |k: u32| ta.wrapping_add(k.wrapping_mul(16));
    let b = move |k: u32| tb.wrapping_add(k.wrapping_mul(16));
    let bp = move |k: u32| (tb + 0x40).wrapping_add(k.wrapping_mul(16));
    e.with_stack(0x140, |e, frame| {
        let f = frame.addr();
        let ray_start = ld(e, ray.addr());
        let direction = vsub(ld(e, ray.addr() + 0x10), ray_start);
        st(e, f + 0xe0, direction);
        let radius = if shape_b.addr() != 0 {
            let radius_a = x87_quiet(e.mem.f32(shape_a.addr() + 0x10));
            let radius_b = x87_quiet(e.mem.f32(shape_b.addr() + 0x10));
            (f64::from(radius_b) + f64::from(radius_a)) as f32
        } else {
            x87_quiet(e.mem.f32(shape_a.addr() + 0x10))
        };
        let widened = f64::from(radius) + e.mem.f64(EDGE_TOLERANCE);
        let bound = f64::from(x87_quiet((widened * widened) as f32));
        let mut point = ld(e, ray.addr());
        let mut tfrac = 0.0f32;
        e.mem.set_u32(f + 0xc, 0);
        e.vcall(
            shape_a.addr(),
            SHAPE_VERTICES,
            &args![f + 0xc, 1u32, f + 0x90],
        );
        let mut min_distance = x87_quiet(e.mem.f32(LINEAR_CAST_START_DISTANCE));
        st(e, t + 0x120, vsub(point, ld(e, f + 0x90)));
        copy16(e, t + 0x120, f + 0x50);
        let first = ld(e, f + 0x50);
        let first_scale = refined_rsqrt(e, sum3(vmul(first, first)));
        let mut unit = vmul(first, [first_scale; 4]);
        let mut hit_normal: Vec4 = [0.0; 4];
        e.mem.set_u32(t, 0);
        e.mem.set_u32(t + 4, 1);
        st(e, f + 0x40, hit_normal);
        st(e, f + 0xd0, hit_normal);
        let limit = e.mem.f64(LINEAR_CAST_BOUND_LIMIT);
        let mut stalls = 0i32;
        let mut pc = if bound < limit { LC_LOOP } else { LC_EXIT };
        loop {
            match pc {
                LC_LOOP => {
                    e.vcall(shape_a.addr(), 0x38, &args![t + 0x120, f + 0x40]);
                    if shape_b.addr() != 0 {
                        let reversed = xorps(ld(e, t + 0x120), mask_at(e, SIGN_MASK));
                        let rotated = rotate_back(e, transform.addr(), reversed);
                        st(e, f + 0x120, rotated);
                        e.vcall(shape_b.addr(), 0x38, &args![f + 0x120, f + 0xd0]);
                        e.call(
                            SET_ROTATED_DIR,
                            &args![f + 0x100, transform.addr(), f + 0xd0],
                        );
                        let vertex = vsub(ld(e, f + 0x40), ld(e, f + 0x100));
                        st(e, f + 0x40, vertex);
                    }
                    let vertex = ld(e, f + 0x40);
                    let count = e.mem.u32(t);
                    let mut known = false;
                    if (count as i32) > 0 {
                        let (x, y, z) = (
                            e.mem.f32(f + 0x40),
                            e.mem.f32(f + 0x44),
                            e.mem.f32(f + 0x48),
                        );
                        let mut k = 0u32;
                        loop {
                            let at = ta + 16 * k;
                            if x == e.mem.f32(at)
                                && y == e.mem.f32(at + 4)
                                && z == e.mem.f32(at + 8)
                            {
                                known = true;
                                break;
                            }
                            k += 1;
                            if (k as i32) >= e.mem.u32(t) as i32 {
                                break;
                            }
                        }
                    }
                    if !known {
                        st(
                            e,
                            t.wrapping_add(count.wrapping_add(2).wrapping_mul(16)),
                            vertex,
                        );
                        let n = e.mem.u32(t).wrapping_add(1);
                        e.mem.set_u32(t, n);
                    }
                    let normal = ld(e, t + 0x120);
                    let reach = vadd(vmul([radius; 4], unit), vertex);
                    let dot = sum3(vmul(normal, vsub(point, reach)));
                    let dot_x87 = x87_quiet(dot);
                    if dot_x87 > 0.0 {
                        let approach = x87_quiet(sum3(vmul(normal, direction)));
                        if approach >= 0.0 {
                            pc = LC_MISS;
                            continue;
                        }
                        tfrac =
                            (f64::from(tfrac) - f64::from(dot_x87) / f64::from(approach)) as f32;
                        if 1.0 <= x87_quiet(tfrac) {
                            pc = LC_MISS;
                            continue;
                        }
                        point = vadd(ld(e, ray.addr()), vmul([tfrac; 4], direction));
                        hit_normal = unit;
                    }
                    copy16(e, t + 0x120, f + 0x80);
                    st(e, t + 0xe0, point);
                    copy16(e, t + 0xe0, t + 0xa0);
                    e.mem.set_u32(t + 0x174, 0);
                    pc = LC_DISPATCH;
                }
                LC_DISPATCH => {
                    let key = (e.mem.u32(t).wrapping_mul(8) | e.mem.u32(t + 4)).wrapping_sub(9);
                    pc = if key > 0x18 {
                        LC_EXIT
                    } else {
                        match key {
                            0 => LC_POINTS,
                            1 => LC_B2,
                            2 => LC_B3,
                            3 => LC_B4,
                            8 => LC_A2,
                            9 => LC_SEGMENTS,
                            10 => LC_EDGE_A2_B3,
                            16 => LC_A3,
                            17 => LC_EDGE_A3_B2,
                            24 => LC_A4,
                            _ => LC_EXIT,
                        }
                    };
                }
                LC_EDGE_A2_B3 | LC_EDGE_A3_B2 => {
                    let flag = u8::from(e.mem.u32(t + 0x170) == 2);
                    let r = if pc == LC_EDGE_A2_B3 {
                        hkp_gsk_process_edge_triangle(
                            e,
                            this,
                            Ptr::new(a(0)),
                            Ptr::new(b(0)),
                            Ptr::new(t),
                            Ptr::new(t + 4),
                            flag,
                            1,
                        )
                    } else {
                        hkp_gsk_process_edge_triangle(
                            e,
                            this,
                            Ptr::new(b(0)),
                            Ptr::new(a(0)),
                            Ptr::new(t + 4),
                            Ptr::new(t),
                            flag,
                            u32::MAX,
                        )
                    };
                    pc = match r {
                        1 => LC_EXIT,
                        2 => LC_SEGMENTS,
                        _ => LC_POST,
                    };
                }
                LC_SEGMENTS => {
                    let a0 = ld(e, a(0));
                    let a1 = ld(e, a(1));
                    st(e, f + 0x90, vsub(a1, a0));
                    let b0 = ld(e, b(0));
                    let b1 = ld(e, b(1));
                    st(e, f + 0xc0, vsub(b1, b0));
                    let mask = e
                        .call(
                            CLOSEST_LINE_SEG_LINE_SEG,
                            &args![a(0), f + 0x90, b(0), f + 0xc0, t + 0x130],
                        )
                        .u32();
                    if mask == 0 {
                        let dir_b = ld(e, f + 0xc0);
                        let dir_a = ld(e, f + 0x90);
                        let normal = vsub(
                            vmul(shufps(dir_b, dir_b, 0xd2), shufps(dir_a, dir_a, 0xc9)),
                            vmul(shufps(dir_b, dir_b, 0xc9), shufps(dir_a, dir_a, 0xd2)),
                        );
                        st(e, t + 0x120, normal);
                        let side = sum3(vmul(vsub(ld(e, a(0)), ld(e, b(0))), normal));
                        let signed = with_sign_of(e, normal, side);
                        st(e, t + 0x120, signed);
                        pc = LC_POST;
                    } else {
                        if mask & 1 != 0 {
                            copy16(e, a(1), a(0));
                        }
                        if mask & 3 != 0 {
                            e.mem.set_u32(t, 1);
                        }
                        if mask & 4 != 0 {
                            copy16(e, b(1), b(0));
                            copy16(e, bp(1), bp(0));
                            e.mem.set_u32(t + 4, 1);
                        } else if mask & 8 != 0 {
                            e.mem.set_u32(t + 4, 1);
                        }
                        pc = LC_DISPATCH;
                    }
                }
                LC_B4 => {
                    let r = find_closest_triangle(e, Ptr::new(b(0)), Ptr::new(a(0)), 1.0);
                    if r < 0 {
                        pc = LC_EXIT;
                    } else {
                        let n = e.mem.u32(t + 4).wrapping_sub(1);
                        e.mem.set_u32(t + 4, n);
                        copy16(e, b(3), b(r as u32));
                        copy16(e, bp(3), bp(r as u32));
                        pc = LC_B3;
                    }
                }
                LC_B3 => {
                    let r = hkp_gsk_check_triangle_boundaries(
                        e,
                        this,
                        Ptr::new(a(0)),
                        Ptr::new(b(0)),
                        1,
                    );
                    if r == 7 {
                        pc = LC_POST;
                        continue;
                    }
                    let m = signed_byte(e, MASK_TABLE + r);
                    if m < 0 {
                        let mut index = (m + 8) as u32;
                        if index as i32 > 3 {
                            index = 1;
                        }
                        let n = e.mem.u32(t + 4).wrapping_sub(1);
                        e.mem.set_u32(t + 4, n);
                        copy16(e, b(2), b(index));
                        copy16(e, bp(2), bp(index));
                        pc = LC_B2;
                        continue;
                    }
                    let first = signed_byte(e, EDGE_TABLE_FIRST + m as u32) as u32;
                    let second = signed_byte(e, EDGE_TABLE_SECOND + m as u32) as u32;
                    let query = ld(e, a(0));
                    let base = ld(e, b(m as u32));
                    let p_second = ld(e, b(second));
                    let p_first = ld(e, b(first));
                    let to_query = vsub(query, base);
                    let side_first = sum3(vmul(vsub(p_first, base), to_query));
                    let b2 = ld(e, b(2));
                    e.mem.set_u32(t + 4, 2);
                    if negative(side_first) {
                        st(e, b(first), b2);
                        copy16(e, bp(2), bp(first));
                        let side_second = sum3(vmul(vsub(p_second, base), to_query));
                        if negative(side_second) {
                            let mut keep = second;
                            let mut n = e.mem.u32(t + 4);
                            if keep == n {
                                keep = first;
                            }
                            n = n.wrapping_sub(1);
                            e.mem.set_u32(t + 4, n);
                            copy16(e, b(n), b(keep));
                            copy16(e, bp(n), bp(keep));
                        }
                    } else {
                        st(e, b(second), b2);
                        copy16(e, bp(2), bp(second));
                    }
                    pc = LC_DISPATCH;
                }
                LC_A4 => {
                    let scale = load_float(e, MINUS_ONE);
                    let r = find_closest_triangle(e, Ptr::new(a(0)), Ptr::new(b(0)), scale);
                    if r < 0 {
                        pc = LC_EXIT;
                    } else {
                        let n = e.mem.u32(t).wrapping_sub(1);
                        e.mem.set_u32(t, n);
                        copy16(e, a(3), a(r as u32));
                        pc = LC_A3;
                    }
                }
                LC_A3 => {
                    let r = hkp_gsk_check_triangle_boundaries(
                        e,
                        this,
                        Ptr::new(b(0)),
                        Ptr::new(a(0)),
                        -1,
                    );
                    if r == 7 {
                        pc = LC_POST;
                        continue;
                    }
                    let m = signed_byte(e, MASK_TABLE + r);
                    if m < 0 {
                        let mut index = (m + 8) as u32;
                        if index as i32 > 3 {
                            index = 1;
                        }
                        let n = e.mem.u32(t).wrapping_sub(1);
                        e.mem.set_u32(t, n);
                        copy16(e, a(2), a(index));
                        pc = LC_A2;
                        continue;
                    }
                    let first = signed_byte(e, EDGE_TABLE_FIRST + m as u32) as u32;
                    let second = signed_byte(e, EDGE_TABLE_SECOND + m as u32) as u32;
                    let query = ld(e, b(0));
                    let base = ld(e, a(m as u32));
                    let p_second = ld(e, a(second));
                    let p_first = ld(e, a(first));
                    let to_query = vsub(query, base);
                    let side_first = sum3(vmul(vsub(p_first, base), to_query));
                    let a2 = ld(e, a(2));
                    e.mem.set_u32(t, 2);
                    if negative(side_first) {
                        st(e, a(first), a2);
                        let side_second = sum3(vmul(vsub(p_second, base), to_query));
                        if negative(side_second) {
                            let mut keep = second;
                            let mut n = e.mem.u32(t);
                            if keep == n {
                                keep = first;
                            }
                            n = n.wrapping_sub(1);
                            e.mem.set_u32(t, n);
                            copy16(e, a(n), a(keep));
                        }
                    } else {
                        st(e, a(second), a2);
                    }
                    pc = LC_DISPATCH;
                }
                LC_B2 => {
                    let b0 = ld(e, b(0));
                    let b1 = ld(e, b(1));
                    let a0 = ld(e, a(0));
                    let edge = vsub(b1, b0);
                    let from_b0 = vsub(b0, a0);
                    let from_b1 = vsub(b1, a0);
                    let negative_b1 = negative(sum3(vmul(from_b1, edge)));
                    let negative_b0 = negative(sum3(vmul(from_b0, edge)));
                    if negative_b1 != negative_b0 {
                        let length_squared = sum3(vmul(edge, edge));
                        let along = vmul([rsqrt_estimate(length_squared); 4], edge);
                        let cross = vsub(
                            vmul(
                                shufps(from_b1, from_b1, 0xc9),
                                shufps(from_b0, from_b0, 0xd2),
                            ),
                            vmul(
                                shufps(from_b1, from_b1, 0xd2),
                                shufps(from_b0, from_b0, 0xc9),
                            ),
                        );
                        let normal = vsub(
                            vmul(shufps(along, along, 0xc9), shufps(cross, cross, 0xd2)),
                            vmul(shufps(along, along, 0xd2), shufps(cross, cross, 0xc9)),
                        );
                        st(e, t + 0x120, normal);
                        pc = LC_POST;
                    } else {
                        if negative_b1 {
                            st(e, b(0), b1);
                            copy16(e, bp(1), bp(0));
                        }
                        let n = e.mem.u32(t + 4).wrapping_sub(1);
                        e.mem.set_u32(t + 4, n);
                        pc = LC_POINTS;
                    }
                }
                LC_A2 => {
                    let a1 = ld(e, a(1));
                    let a0 = ld(e, a(0));
                    let b0 = ld(e, b(0));
                    let edge = vsub(a1, a0);
                    let from_b0 = vsub(a0, b0);
                    let to_a1 = vsub(a1, b0);
                    let negative_a1 = negative(sum3(vmul(to_a1, edge)));
                    let negative_a0 = negative(sum3(vmul(from_b0, edge)));
                    if negative_a1 != negative_a0 {
                        let length_squared = sum3(vmul(edge, edge));
                        let along = vmul([rsqrt_estimate(length_squared); 4], edge);
                        let cross = vsub(
                            vmul(shufps(from_b0, from_b0, 0xd2), shufps(to_a1, to_a1, 0xc9)),
                            vmul(shufps(from_b0, from_b0, 0xc9), shufps(to_a1, to_a1, 0xd2)),
                        );
                        let normal = vsub(
                            vmul(shufps(cross, cross, 0xc9), shufps(along, along, 0xd2)),
                            vmul(shufps(cross, cross, 0xd2), shufps(along, along, 0xc9)),
                        );
                        st(e, t + 0x120, normal);
                        pc = LC_POST;
                    } else {
                        if negative_a1 {
                            let parallel = ld(e, a(5));
                            st(e, a(0), a1);
                            st(e, a(4), parallel);
                        }
                        let n = e.mem.u32(t).wrapping_sub(1);
                        e.mem.set_u32(t, n);
                        pc = LC_POINTS;
                    }
                }
                LC_POINTS => {
                    let normal = vsub(ld(e, a(0)), ld(e, b(0)));
                    st(e, t + 0x120, normal);
                    pc = LC_POST;
                }
                LC_POST => {
                    let kind = e.mem.u32(t + 4);
                    e.mem.set_u32(t + 0x170, kind);
                    let minus_one = e.mem.f32(MINUS_ONE);
                    let negated = vmul([minus_one; 4], ld(e, t + 0x120));
                    let scale = refined_rsqrt(e, sum3(vmul(negated, negated)));
                    st(e, t + 0x120, negated);
                    unit = vmul([scale; 4], negated);
                    let count_a = e.mem.u32(t);
                    if (count_a as i32) > 0 {
                        let reach = sum3(vmul(vsub(ld(e, b(0)), ld(e, a(0))), unit));
                        let distance = x87_quiet(reach).abs();
                        for _ in 0..count_a {
                            if x87_quiet(min_distance) > distance {
                                min_distance = distance;
                            }
                        }
                    }
                    let change = vsub(negated, ld(e, f + 0x80));
                    let moved = x87_quiet(sum3(vmul(change, change)));
                    if f64::from(moved) < e.mem.f64(LINEAR_CAST_STALL_LIMIT) {
                        stalls += 1;
                        if stalls >= 3 {
                            pc = LC_EXIT;
                            continue;
                        }
                    }
                    let nearest = f64::from(x87_quiet(min_distance));
                    pc = if nearest * nearest > bound {
                        LC_LOOP
                    } else {
                        LC_EXIT
                    };
                }
                LC_EXIT => {
                    let current = x87_quiet(e.mem.f32(result.addr() + 0x10));
                    if current > x87_quiet(tfrac) {
                        e.mem.set_f32(result.addr() + 0x10, x87_quiet(tfrac));
                        st(e, result.addr(), hit_normal);
                        e.mem.set_u8(found.addr(), 1);
                        return;
                    }
                    pc = LC_MISS;
                }
                _ => {
                    // LC_MISS
                    e.mem.set_u8(found.addr(), 0);
                    return;
                }
            }
        }
    });
}

// Translated from 00daac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::castRay` (Xbox PDB), thiscall `(this, found, shape, ray, hit)`; the callee pops
/// four words after `this`. Records the timer stream's start label `010d51b4`, copies the
/// 0x50-byte hit record `hit` into a local, builds the identity rows (`1,0,0,0`, `0,1,0,0`,
/// `0,0,1,0`) of a transform and runs `linearCast` (`00da9b30`) with `shape` as shape A and no
/// shape B, then records the end label `010c41f8`. When the cast found a hit whose fraction
/// (the local's float at +0x10) is not exactly zero, the local is copied back over `hit`
/// (`009c1aa0`) and `*found = 1`; otherwise `*found = 0`.
/// Answers `found`: the exe leaves it in `EAX` at every `RET` (the hkBool/hkResult result
/// returned through a pointer, which callers read through `EAX`).
pub fn hkp_gsk_cast_ray(
    e: &mut Engine,
    this: Ptr,
    found: Ptr,
    shape: Ptr,
    ray: Ptr,
    hit: Ptr,
) -> Ptr {
    monitor_record(e, LABEL_CAST_RAY_START);
    e.with_stack(0xa0, |e, frame| {
        let f = frame.addr();
        for index in 0..20 {
            let word = e.mem.u32(hit.addr() + 4 * index);
            e.mem.set_u32(f + 0x50 + 4 * index, word);
        }
        let identity: [Vec4; 3] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ];
        for (row, value) in identity.into_iter().enumerate() {
            st(e, f + 0x20 + 16 * row as u32, value);
        }
        hkp_gsk_linear_cast(
            e,
            this,
            Ptr::new(f + 0x1b),
            shape,
            Ptr::new(0),
            Ptr::new(f + 0x20),
            ray,
            Ptr::new(f + 0x50),
        );
        monitor_record(e, LABEL_CAST_RAY_END);
        let hit_found = e.mem.u8(f + 0x1b) != 0;
        let fraction = e.mem.f32(f + 0x60);
        // `FLDZ; FCOMP; TEST AH,0x44; JNP`: only an ordered equality with zero is a miss.
        if hit_found && fraction.partial_cmp(&0.0) != Some(Ordering::Equal) {
            e.call(COPY_HIT_RECORD, &args![hit, f + 0x50]);
            e.mem.set_u8(found.addr(), 1);
        } else {
            e.mem.set_u8(found.addr(), 0);
        }
    });
    found
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_8390, find_closest_triangle_backup(Ptr, Ptr, Ptr) -> i32),
        entry!(0x00da_8700, fn_00da8700(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00da_8820, fn_00da8820(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00da_8a00,
            hk_gsk_recalc_contact_internal(Ptr, Ptr, u32, u32, Ptr, Ptr, Ptr)
        ),
        entry!(0x00da_9210, fn_00da9210(Ptr, Ptr, u32, Ptr, Ptr)),
        entry!(
            0x00da_bca0,
            hkp_gsk_get_closest_point(Ptr, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00da_8500,
            hkp_gsk_check_triangle_boundaries(Ptr, Ptr, Ptr, i32) -> u32
        ),
        entry!(0x00da_8fb0, find_closest_triangle(Ptr, Ptr, f32) -> i32),
        entry!(
            0x00da_9340,
            hkp_gsk_process_edge_triangle(Ptr, Ptr, Ptr, Ptr, Ptr, u8, u32) -> u32
        ),
        entry!(0x00da_9790, hkp_gsk_exit_and_export_cache_impl(Ptr, Ptr)),
        entry!(
            0x00da_9880,
            hkp_gsk_convert_feature_to_closest_distance(Ptr, Ptr, Ptr)
        ),
        entry!(0x00da_98f0, hkp_gsk_reduce_dimension_extended(Ptr)),
        entry!(0x00da_a7e0, hkp_gsk_handle_penetration(Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00da_ad40,
            hkp_gsk_get_closest_feature(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32
        ),
        entry!(
            0x00da_9b30,
            hkp_gsk_linear_cast(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00da_ac40, hkp_gsk_cast_ray(Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;
    type SegmentLog = Rc<RefCell<Vec<(Vec<u32>, Vec4, Vec4)>>>;

    fn constants(e: &mut Engine) {
        e.mem.write(
            MASK_TABLE,
            &[0xff, 0, 1, 0xfa, 2, 0xf9, 0xf8, 0xff, 2, 0, 1, 2, 0],
        );
        for i in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * i, 0x8000_0000);
            e.mem.set_u32(ABS_MASK + 4 * i, 0x7fff_ffff);
            e.mem.set_f32(ONES + 4 * i, 1.0);
        }
        e.mem.set_f32(MINUS_ONE, -1.0);
        e.mem.set_f32(BIG_FLOAT, f32::from_bits(0x7f7f_ffee));
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
    }

    fn triangle(e: &mut Engine, at: u32, p0: Vec4, p1: Vec4, p2: Vec4) {
        st(e, at, p0);
        st(e, at + 0x10, p1);
        st(e, at + 0x20, p2);
    }

    const BASE: Vec4 = [0.0, 0.0, 0.0, 0.0];
    const RIGHT: Vec4 = [1.0, 0.0, 0.0, 0.0];
    const UP: Vec4 = [0.0, 1.0, 0.0, 0.0];

    #[test]
    fn the_backup_search_clears_weights_by_edge_sign_and_returns_the_first_positive() {
        let mut e = test_engine();
        constants(&mut e);
        let v = e.mem.alloc(0x40);
        triangle(&mut e, v, BASE, RIGHT, UP);
        st(&mut e, v + 0x30, [0.0, 0.0, 1.0, 0.0]);
        let point = e.mem.alloc(0x10);
        st(&mut e, point, [0.25, 0.25, -1.0, 0.0]);
        let w = e.mem.alloc(0x10);
        for i in 0..3 {
            e.mem.set_f32(w + 4 * i, 1.0);
        }
        assert_eq!(e.call(0x00da_8390, &args![v, point, w]).i32(), 0);
        let weights = [0, 1, 2].map(|i| e.mem.f32(w + 4 * i));
        assert_eq!(weights, [1.0, -1.0, -1.0]);
        for i in 0..3 {
            e.mem.set_f32(w + 4 * i, -1.0);
        }
        assert_eq!(e.call(0x00da_8390, &args![v, point, w]).i32(), -1);
    }

    #[test]
    fn the_boundary_check_masks_negative_sums_and_flips_a_backwards_triangle() {
        let mut e = test_engine();
        constants(&mut e);
        let this = e.mem.alloc(0x200);
        let tri = e.mem.alloc(0x80);
        triangle(&mut e, tri, BASE, RIGHT, UP);
        let q = e.mem.alloc(0x10);
        // Outside: the first and third sums are negative.
        st(&mut e, q, [-1.0, 0.5, 0.0, 0.0]);
        assert_eq!(e.call(0x00da_8500, &args![this, q, tri, 0u32]).u32(), 5);
        // Inside with the flag: all three negative, and the shapes' points lie on the wrong
        // side of the normal, so the triangle is turned around.
        st(&mut e, q, [0.25, 0.25, 0.0, 0.0]);
        st(&mut e, this + 0xa0, [0.0, 0.0, 1.0, 0.0]);
        st(&mut e, tri + 0x40, [7.0, 0.0, 0.0, 0.0]);
        st(&mut e, tri + 0x50, [8.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.call(0x00da_8500, &args![this, q, tri, 1u32]).u32(), 7);
        assert_eq!(ld(&e, this + 0x120), [-0.0, -0.0, -1.0, -0.0]);
        assert_eq!(ld(&e, tri), RIGHT);
        assert_eq!(ld(&e, tri + 0x10), BASE);
        assert_eq!(ld(&e, tri + 0x40)[0], 8.0);
        assert_eq!(ld(&e, tri + 0x50)[0], 7.0);
        assert_eq!(e.mem.f32(this + 0x160), -0.25);
        assert_eq!(e.mem.f32(this + 0x164), -0.5);
        assert_eq!(e.mem.u32(this + 0x14), 1);
    }

    #[test]
    fn the_triangle_search_picks_the_dominant_face() {
        let mut e = test_engine();
        constants(&mut e);
        e.mem.set_f32(NEAR_ZERO, 1e-6);
        e.mem.set_f64(TIE_FACTOR, 1.1);
        let tri = e.mem.alloc(0x40);
        triangle(&mut e, tri, BASE, RIGHT, UP);
        st(&mut e, tri + 0x30, [0.0, 0.0, 1.0, 0.0]);
        let point = e.mem.alloc(0x10);
        let one = 1.0f32.to_bits();
        // On the apex: nothing is closer than the threshold.
        st(&mut e, point, [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(e.call(0x00da_8fb0, &args![tri, point, one]).i32(), -1);
        // Straight above it: the first face wins clearly.
        st(&mut e, point, [0.0, 0.0, 2.0, 0.0]);
        assert_eq!(e.call(0x00da_8fb0, &args![tri, point, one]).i32(), 0);
        // Towards -x: the second face.
        st(&mut e, point, [-1.0, 0.0, 1.0, 0.0]);
        assert_eq!(e.call(0x00da_8fb0, &args![tri, point, one]).i32(), 1);
    }

    #[test]
    fn an_edge_pair_that_crosses_the_triangle_plane_inside_it_is_reported_inside() {
        let mut e = test_engine();
        constants(&mut e);
        let this = e.mem.alloc(0x200);
        let s = e.mem.alloc(0x80);
        let tri = e.mem.alloc(0x80);
        let ca = e.mem.alloc(8);
        let cb = ca + 4;
        e.mem.set_u32(ca, 3);
        e.mem.set_u32(cb, 3);
        triangle(&mut e, tri, BASE, RIGHT, UP);
        st(&mut e, s, [0.25, 0.25, 1.0, 0.0]);
        st(&mut e, s + 0x10, [0.25, 0.25, -1.0, 0.0]);
        let got = e.call(0x00da_9340, &args![this, s, tri, ca, cb, 1u32, 0u32]);
        assert_eq!(got.u32(), 1);
        assert_eq!(e.mem.u32(ca), 3);
        // Both points on one side and the first inside: the first counter drops.
        st(&mut e, s + 0x10, [0.25, 0.25, 2.0, 0.0]);
        let got = e.call(0x00da_9340, &args![this, s, tri, ca, cb, 1u32, 0u32]);
        assert_eq!(got.u32(), 0);
        assert_eq!(e.mem.u32(ca), 2);
        assert_eq!(e.mem.u32(cb), 3);
    }

    #[test]
    fn the_stored_normal_mode_drops_the_first_point_when_the_second_is_inside() {
        let mut e = test_engine();
        constants(&mut e);
        let this = e.mem.alloc(0x200);
        let s = e.mem.alloc(0x80);
        let tri = e.mem.alloc(0x80);
        let ca = e.mem.alloc(8);
        let cb = ca + 4;
        e.mem.set_u32(ca, 2);
        e.mem.set_u32(cb, 3);
        triangle(&mut e, tri, BASE, RIGHT, UP);
        st(&mut e, this + 0x120, [0.0, 0.0, 1.0, 0.0]);
        st(&mut e, s, [0.25, 0.25, 1.0, 0.0]);
        st(&mut e, s + 0x10, [0.25, 0.25, 2.0, 0.0]);
        st(&mut e, s + 0x50, [5.0, 0.0, 0.0, 0.0]);
        let got = e.call(0x00da_9340, &args![this, s, tri, ca, cb, 0u32, 9u32]);
        assert_eq!(got.u32(), 0);
        assert_eq!(e.mem.u32(ca), 1);
        assert_eq!(ld(&e, s), [0.25, 0.25, 2.0, 0.0]);
        assert_eq!(ld(&e, s + 0x40), [5.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(this + 0x174), 9);
    }

    #[test]
    fn the_cache_export_writes_counts_and_sorted_ids() {
        let mut e = test_engine();
        let this = e.mem.alloc(0x200);
        let cache = e.mem.alloc(0x20);
        e.mem.set_u32(this, 2);
        e.mem.set_u32(this + 4, 1);
        e.mem.set_u16(this + 0x2c, 5);
        e.mem.set_u16(this + 0x3c, 9);
        e.mem.set_u16(this + 0xec, 7);
        e.call(0x00da_9790, &args![this, cache]);
        assert_eq!(e.mem.u8(cache + 8), 2);
        assert_eq!(e.mem.u8(cache + 9), 1);
        assert_eq!(e.mem.u16(cache), 9);
        assert_eq!(e.mem.u16(cache + 2), 5);
        assert_eq!(e.mem.u16(cache + 4), 7);
    }

    #[test]
    fn the_feature_conversion_picks_the_point_by_which_list_has_one_vertex() {
        let mut e = test_engine();
        let this = e.mem.alloc(0x200);
        let input = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x30);
        st(&mut e, input, [1.0, 2.0, 3.0, 2.0]);
        st(&mut e, this + 0x20, [4.0, 4.0, 4.0, 4.0]);
        st(&mut e, this + 0xa0, [10.0, 10.0, 10.0, 10.0]);
        st(&mut e, this + 0x130, [6.0, 6.0, 6.0, 6.0]);
        e.mem.set_u32(this, 1);
        e.call(0x00da_9880, &args![this, input, out]);
        assert_eq!(ld(&e, out), [1.0, 2.0, 3.0, 2.0]);
        assert_eq!(e.mem.f32(out + 0x20), 2.0);
        assert_eq!(ld(&e, out + 0x10), [4.0; 4]);
        e.mem.set_u32(this, 2);
        e.mem.set_u32(this + 4, 1);
        e.call(0x00da_9880, &args![this, input, out]);
        assert_eq!(ld(&e, out + 0x10), [12.0, 14.0, 16.0, 14.0]);
        e.mem.set_u32(this + 4, 2);
        e.call(0x00da_9880, &args![this, input, out]);
        assert_eq!(ld(&e, out + 0x10), [6.0; 4]);
    }

    fn segment_double(e: &mut Engine, distances: Vec<f32>) -> SegmentLog {
        let log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        let next = Cell::new(0usize);
        e.register_double(CLOSEST_LINE_SEG_LINE_SEG, move |e, a| {
            sink.borrow_mut()
                .push((a.to_vec(), ld(e, a[1]), ld(e, a[3])));
            let distance = distances[next.get() % distances.len()];
            next.set(next.get() + 1);
            e.mem.set_f32(a[4] + 0x20, distance);
            Ret::default()
        });
        log
    }

    #[test]
    fn the_reduction_shrinks_a_list_whose_point_lies_inside_the_other_triangle() {
        let mut e = test_engine();
        constants(&mut e);
        let log = segment_double(&mut e, vec![1.0]);
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, 1);
        e.mem.set_u32(this + 4, 3);
        st(&mut e, this + 0x20, [0.25, 0.25, 0.0, 0.0]);
        triangle(&mut e, this + 0xa0, BASE, RIGHT, UP);
        e.call(0x00da_98f0, &args![this]);
        assert_eq!(e.mem.u32(this), 1);
        assert_eq!(e.mem.u32(this + 4), 3);
        assert!(log.borrow().is_empty());
        // The second list is the single point, the first the triangle: the point inside it
        // is found at index 1 of the second list, which moves to the front.
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, 3);
        e.mem.set_u32(this + 4, 2);
        triangle(&mut e, this + 0x20, BASE, RIGHT, UP);
        st(&mut e, this + 0xa0, [5.0, 5.0, 0.0, 0.0]);
        st(&mut e, this + 0xb0, [0.25, 0.25, 0.0, 0.0]);
        st(&mut e, this + 0xe0, [100.0, 0.0, 0.0, 0.0]);
        st(&mut e, this + 0xf0, [101.0, 0.0, 0.0, 0.0]);
        e.call(0x00da_98f0, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 1);
        assert_eq!(ld(&e, this + 0xa0), [0.25, 0.25, 0.0, 0.0]);
        assert_eq!(ld(&e, this + 0xe0), [101.0, 0.0, 0.0, 0.0]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn the_reduction_drops_the_vertex_at_the_start_of_the_closest_edge_pair() {
        let mut e = test_engine();
        constants(&mut e);
        // Two single points: one pair, and the edge vectors are the second vertices minus the
        // first.
        let log = segment_double(&mut e, vec![5.0]);
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, 2);
        e.mem.set_u32(this + 4, 2);
        st(&mut e, this + 0x20, [1.0, 0.0, 0.0, 0.0]);
        st(&mut e, this + 0x30, [3.0, 0.0, 0.0, 0.0]);
        st(&mut e, this + 0xa0, [0.0, 1.0, 0.0, 0.0]);
        st(&mut e, this + 0xb0, [0.0, 4.0, 0.0, 0.0]);
        e.call(0x00da_98f0, &args![this]);
        {
            let calls = log.borrow();
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0].0[0], this + 0x20);
            assert_eq!(calls[0].0[2], this + 0xa0);
            assert_eq!(calls[0].1, [2.0, 0.0, 0.0, 0.0]);
            assert_eq!(calls[0].2, [0.0, 3.0, 0.0, 0.0]);
        }
        assert_eq!(e.mem.u32(this), 2);
    }

    #[test]
    fn the_reduction_replaces_the_start_vertex_of_the_closest_edge_of_a_triangle() {
        let mut e = test_engine();
        constants(&mut e);
        // A triangle and a lone point away from it: the closest pair is the second one (index
        // 1), so the vertex the edge table gives for it (vertex 0) is replaced by vertex 2.
        let log = segment_double(&mut e, vec![3.0, 1.0, 2.0]);
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, 3);
        e.mem.set_u32(this + 4, 1);
        triangle(&mut e, this + 0x20, BASE, RIGHT, UP);
        st(&mut e, this + 0xa0, [5.0, 5.0, 0.0, 0.0]);
        e.call(0x00da_98f0, &args![this]);
        assert_eq!(log.borrow().len(), 3);
        assert_eq!(e.mem.u32(this), 2);
        assert_eq!(ld(&e, this + 0x20), UP);
        assert_eq!(e.mem.u32(this + 4), 1);
    }

    fn identity_rows(e: &mut Engine, at: u32, translation: Vec4) {
        st(e, at, [1.0, 0.0, 0.0, 0.0]);
        st(e, at + 0x10, [0.0, 1.0, 0.0, 0.0]);
        st(e, at + 0x20, [0.0, 0.0, 1.0, 0.0]);
        st(e, at + 0x30, translation);
    }

    #[test]
    fn the_penetration_handler_stores_a_direct_hit_and_keeps_going_after_a_miss() {
        let mut e = test_engine();
        constants(&mut e);
        e.mem.set_f32(PENETRATION_TOLERANCE, 1e-4);
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&calls);
        let hit = Rc::new(Cell::new(true));
        let hit_flag = Rc::clone(&hit);
        let diff = Rc::new(RefCell::new(BASE));
        let diff_sink = Rc::clone(&diff);
        e.register_double(CALCULATE_PENETRATION_DEPTH, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            *diff_sink.borrow_mut() = ld(e, a[6]);
            st(e, a[8], [0.0, 1.0, 0.0, 0.0]);
            Ret {
                eax: u32::from(hit_flag.get()),
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x200);
        let m = e.mem.alloc(0x40);
        identity_rows(&mut e, m, [10.0, 20.0, 30.0, 0.0]);
        e.mem.set_u32(this, 1);
        e.mem.set_u32(this + 4, 1);
        st(&mut e, this + 0x20, [3.0, 3.0, 3.0, 0.0]);
        st(&mut e, this + 0xa0, [1.0, 1.0, 1.0, 0.0]);
        e.call(0x00da_a7e0, &args![this, 0x111u32, 0x222u32, m]);
        {
            let log = calls.borrow();
            assert_eq!(log.len(), 1);
            assert_eq!(log[0][0..3], [0x111, 0x222, m]);
            assert_eq!(log[0][3], 1e-4f32.to_bits());
            assert_eq!(log[0][4..6], [this + 0x20, this + 0xe0]);
            assert_eq!(log[0][7], 1);
        }
        assert_eq!(*diff.borrow(), [2.0, 2.0, 2.0, 0.0]);
        assert_eq!(e.mem.u32(this + 0x14), 1);
        assert_eq!(ld(&e, this + 0x120), [0.0, 1.0, 0.0, 0.0]);
        // A miss: the vertex ids are all equal, so both lists shrink to one vertex and the
        // second is moved by the transform.
        hit.set(false);
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, 1);
        e.mem.set_u32(this + 4, 1);
        e.mem.set_u32(this + 0x2c, 5);
        e.mem.set_u32(this + 0xec, 7);
        st(&mut e, this + 0xe0, [1.0, 2.0, 3.0, 0.0]);
        e.mem.set_u32(this + 0xec, 7);
        e.call(0x00da_a7e0, &args![this, 0x111u32, 0x222u32, m]);
        assert_eq!(e.mem.u32(this), 1);
        assert_eq!(e.mem.u32(this + 4), 1);
        assert_eq!(ld(&e, this + 0xa0), [11.0, 22.0, 33.0, 0.0]);
        assert_eq!(e.mem.u32(this + 0x14), 1);
        assert_eq!(ld(&e, this + 0x120), [0.0, 1.0, 0.0, 0.0]);
    }

    fn shapes(e: &mut Engine, support_a: Vec4, support_b: Vec4) -> (u32, u32) {
        let mut make = |address: u32, vertex: Vec4| {
            let table = e.mem.alloc(0x40);
            e.mem.set_u32(table + 0x38, address);
            let shape = e.mem.alloc(0x10);
            e.mem.set_u32(shape, table);
            e.register_double(address, move |e, a| {
                st(e, a[2], vertex);
                Ret::default()
            });
            shape
        };
        let a = make(0x00ee_0001, support_a);
        let b = make(0x00ee_0002, support_b);
        (a, b)
    }

    fn feature_setup(e: &mut Engine) -> (u32, u32) {
        constants(e);
        e.mem.set_f32(FEATURE_TOLERANCE, 1e-6);
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this + 8, 4);
        e.mem.set_u32(this + 0xc, 4);
        let m = e.mem.alloc(0x40);
        identity_rows(e, m, BASE);
        (this, m)
    }

    #[test]
    fn a_zero_gap_converges_and_returns_four() {
        let mut e = test_engine();
        let (this, m) = feature_setup(&mut e);
        let (shape_a, shape_b) = shapes(&mut e, BASE, BASE);
        let result = e.mem.alloc(0x10);
        e.mem.set_u32(this, 1);
        e.mem.set_u32(this + 4, 1);
        e.mem.set_u32(this + 0x10, 1);
        let got = e.call(0x00da_ad40, &args![this, shape_a, shape_b, m, result]);
        assert_eq!(got.u32(), 4);
        assert_eq!(e.mem.u32(this), 1);
        assert_eq!(e.mem.u32(this + 4), 1);
    }

    #[test]
    fn two_single_points_whose_supports_add_nothing_end_with_the_normalised_gap() {
        let mut e = test_engine();
        let (this, m) = feature_setup(&mut e);
        // The support points the shapes return are the vertices already held.
        let (shape_a, shape_b) = shapes(&mut e, [1.0, 0.0, 0.0, 0.0], BASE);
        let result = e.mem.alloc(0x10);
        e.mem.set_u32(this, 1);
        e.mem.set_u32(this + 4, 1);
        st(&mut e, this + 0x20, [1.0, 0.0, 0.0, 0.0]);
        let got = e.call(0x00da_ad40, &args![this, shape_a, shape_b, m, result]);
        assert_eq!(got.u32(), 0);
        let normal = ld(&e, result);
        assert!((normal[0] - 1.0).abs() < 1e-5);
        assert_eq!(normal[1], 0.0);
        let distance = e.mem.f32(result + 0xc);
        assert!((distance - 1.0).abs() < 1e-5);
        assert_eq!(e.mem.f32(this + 0x12c), distance);
        assert_eq!(e.mem.u32(this + 0x170), 1);
    }

    #[test]
    fn a_point_beside_a_segment_gets_the_segment_normal_toward_it() {
        let mut e = test_engine();
        let (this, m) = feature_setup(&mut e);
        let (shape_a, shape_b) = shapes(&mut e, [1.0, 1.0, 0.0, 0.0], BASE);
        let result = e.mem.alloc(0x10);
        e.mem.set_u32(this, 1);
        e.mem.set_u32(this + 4, 2);
        st(&mut e, this + 0x20, [1.0, 1.0, 0.0, 0.0]);
        st(&mut e, this + 0xe0, BASE);
        st(&mut e, this + 0xf0, [2.0, 0.0, 0.0, 0.0]);
        let got = e.call(0x00da_ad40, &args![this, shape_a, shape_b, m, result]);
        assert_eq!(got.u32(), 0);
        let normal = ld(&e, result);
        assert!(normal[0].abs() < 1e-3);
        assert!((normal[1] - 1.0).abs() < 1e-3);
        assert!((e.mem.f32(result + 0xc) - 1.0).abs() < 1e-3);
        // The B vertices were rebuilt from the untransformed ones.
        assert_eq!(ld(&e, this + 0xb0), [2.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(this + 0x170), 2);
    }
}

#[cfg(test)]
mod havok_geometry_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// The constants of the image these functions read.
    fn image_constants(e: &mut Engine) {
        e.mem.write(
            MASK_TABLE,
            &[0xff, 0, 1, 0xfa, 2, 0xf9, 0xf8, 0xff, 2, 0, 1, 2, 0],
        );
        for i in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * i, 0x8000_0000);
            e.mem.set_u32(ABS_MASK + 4 * i, 0x7fff_ffff);
            e.mem.set_f32(ONES + 4 * i, 1.0);
            e.mem.set_f32(ZERO_VECTOR + 4 * i, 0.0);
        }
        e.mem.set_f32(MINUS_ONE, -1.0);
        e.mem.set_f32(BIG_FLOAT, f32::from_bits(0x7f7f_ffee));
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
        e.mem.set_f32(FEATURE_TOLERANCE, 1e-6);
        e.mem.set_f64(EDGE_TOLERANCE, 0.001);
        e.mem.set_f64(POINT_TOLERANCE, 1000.0);
    }

    fn near(actual: Vec4, expected: Vec4, tolerance: f32) -> bool {
        actual
            .into_iter()
            .zip(expected)
            .all(|(a, b)| (a - b).abs() <= tolerance)
    }

    fn triangle_at(e: &mut Engine) -> u32 {
        let t = e.mem.alloc(0x40);
        st(e, t, [0.0, 0.0, 0.0, 0.0]);
        st(e, t + 0x10, [1.0, 0.0, 0.0, 0.0]);
        st(e, t + 0x20, [0.0, 1.0, 0.0, 0.0]);
        t
    }

    fn vector(e: &mut Engine, v: Vec4) -> u32 {
        let at = e.mem.alloc(0x10);
        st(e, at, v);
        at
    }

    #[test]
    fn the_plane_helper_faces_the_normal_toward_the_reference_and_measures_the_point() {
        let mut e = test_engine();
        image_constants(&mut e);
        let triangle = triangle_at(&mut e);
        let point = vector(&mut e, [0.25, 0.25, 2.0, 0.0]);
        let normal = e.mem.alloc(0x10);
        let distance = e.mem.alloc(0x10);
        let up = vector(&mut e, [0.0, 0.0, 1.0, 0.0]);
        e.call(0x00da_8700, &args![triangle, normal, up, distance, point]);
        assert!(near(ld(&e, normal), [0.0, 0.0, 1.0, 0.0], 1e-3));
        assert!(near(ld(&e, distance), [2.0; 4], 1e-3));
        // A reference on the other side flips the normal and the distance.
        let down = vector(&mut e, [0.0, 0.0, -1.0, 0.0]);
        e.call(0x00da_8700, &args![triangle, normal, down, distance, point]);
        assert!(near(ld(&e, normal), [0.0, 0.0, -1.0, 0.0], 1e-3));
        assert!(near(ld(&e, distance), [-2.0; 4], 1e-3));
        // A degenerate triangle has the zero normal.
        let flat = e.mem.alloc(0x40);
        e.call(0x00da_8700, &args![flat, normal, up, distance, point]);
        assert_eq!(ld(&e, normal), [0.0; 4]);
    }

    /// Doubles `closestLineSegLineSeg`: writes a recognisable closest point and logs its words.
    type SegmentLog = Rc<RefCell<Vec<Vec<u32>>>>;

    fn segment_double(e: &mut Engine) -> SegmentLog {
        let log: SegmentLog = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(CLOSEST_LINE_SEG_LINE_SEG, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            st(e, a[4], [4.0, 5.0, 6.0, 7.0]);
            Ret::default()
        });
        log
    }

    fn segment(e: &mut Engine, from: Vec4, to: Vec4) -> u32 {
        let at = e.mem.alloc(0x20);
        st(e, at, from);
        st(e, at + 0x10, to);
        at
    }

    #[test]
    fn the_segment_helper_uses_the_reference_for_a_small_cross_product() {
        let mut e = test_engine();
        image_constants(&mut e);
        let log = segment_double(&mut e);
        let a = segment(&mut e, [0.0, 0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]);
        let b = segment(&mut e, [0.0, 0.0, 1.0, 0.0], [0.0, 1.0, 1.0, 0.0]);
        let reference = vector(&mut e, [0.0, 1.0, 0.0, 0.0]);
        let normal = e.mem.alloc(0x10);
        let closest = e.mem.alloc(0x10);
        e.call(0x00da_8820, &args![b, normal, a, reference, closest]);
        let calls = log.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], a);
        assert_eq!(calls[0][2], b);
        // The directions are the segment vectors (A then B).
        assert_eq!(ld(&e, calls[0][1]), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(ld(&e, calls[0][3]), [0.0, 1.0, 0.0, 0.0]);
        // The normal is the reference; its fourth lane is (a0 - b0) . normal = (0, 0, -1) . (0, 1, 0).
        assert_eq!(ld(&e, normal), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(ld(&e, closest), [4.0, 5.0, 6.0, 7.0]);
    }

    #[test]
    fn the_segment_helper_takes_the_normalising_branch_when_the_cross_product_overflows() {
        let mut e = test_engine();
        image_constants(&mut e);
        segment_double(&mut e);
        // Directions of 1e15 square to 1e30 each, so the cross product's square overflows to
        // infinity while the squared lengths stay finite: the `0.001 * |cross|^2` test passes.
        let a = segment(&mut e, [0.0; 4], [1e15, 0.0, 0.0, 0.0]);
        let b = segment(&mut e, [0.0; 4], [0.0, 1e15, 0.0, 0.0]);
        let reference = vector(&mut e, [0.0, 1.0, 0.0, 0.0]);
        let normal = e.mem.alloc(0x10);
        let closest = e.mem.alloc(0x10);
        e.call(0x00da_8820, &args![b, normal, a, reference, closest]);
        let got = ld(&e, normal);
        // The normaliser of an infinite length is not a number, so the reference was not used.
        assert!(got[0].is_nan() || got[2].is_nan());
        assert_eq!(ld(&e, closest), [4.0, 5.0, 6.0, 7.0]);
    }

    /// Calls `hkGskRecalcContactInternal` and returns `(point, normal)`.
    fn recalc(e: &mut Engine, sides: (&[Vec4], &[Vec4]), reference: Vec4) -> (Vec4, Vec4) {
        let a = e.mem.alloc(0x40);
        for (i, v) in sides.0.iter().enumerate() {
            st(e, a + 0x10 * i as u32, *v);
        }
        let b = e.mem.alloc(0x40);
        for (i, v) in sides.1.iter().enumerate() {
            st(e, b + 0x10 * i as u32, *v);
        }
        let reference = vector(e, reference);
        let point = e.mem.alloc(0x10);
        let normal = e.mem.alloc(0x10);
        st(e, point, [9.0; 4]);
        st(e, normal, [8.0; 4]);
        let counts = [sides.0.len() as u32, sides.1.len() as u32];
        e.call(
            0x00da_8a00,
            &args![a, b, counts[0], counts[1], reference, point, normal],
        );
        (ld(e, point), ld(e, normal))
    }

    #[test]
    fn recalc_contact_of_two_points_normalises_the_difference() {
        let mut e = test_engine();
        image_constants(&mut e);
        let (point, normal) = recalc(
            &mut e,
            (&[[2.0, 0.0, 0.0, 0.0]], &[[0.0; 4]]),
            [1.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(point, [2.0, 0.0, 0.0, 0.0]);
        assert!(near(normal, [1.0, 0.0, 0.0, 2.0], 2e-3));
        // A difference perpendicular to the reference (squared length above 1000 times the
        // dot product) keeps the reference as the normal and its dot product as the last lane.
        let (point, normal) = recalc(
            &mut e,
            (&[[0.0, 2.0, 0.0, 0.0]], &[[0.0; 4]]),
            [1.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(point, [0.0, 2.0, 0.0, 0.0]);
        assert_eq!(normal, [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn recalc_contact_of_a_point_and_an_edge_keeps_the_reference() {
        let mut e = test_engine();
        image_constants(&mut e);
        let (point, normal) = recalc(
            &mut e,
            (&[[1.0, 2.0, 3.0, 0.0]], &[[0.0; 4], [5.0, 5.0, 5.0, 0.0]]),
            [0.0, 1.0, 0.0, 0.0],
        );
        assert_eq!(point, [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(normal, [0.0, 1.0, 0.0, 2.0]);
        // An edge and a point: the point is moved along the reference by the dot product and
        // the last lane is the negated dot product.
        let (point, normal) = recalc(
            &mut e,
            (&[[0.0; 4], [5.0, 5.0, 5.0, 0.0]], &[[1.0, 2.0, 3.0, 0.0]]),
            [0.0, 1.0, 0.0, 0.0],
        );
        assert_eq!(point, [1.0, 0.0, 3.0, 0.0]);
        assert_eq!(normal, [0.0, 1.0, 0.0, -2.0]);
    }

    #[test]
    fn recalc_contact_of_a_point_and_a_triangle_uses_the_triangle_normal() {
        let mut e = test_engine();
        image_constants(&mut e);
        let corners = [
            [0.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
        ];
        let vertex = [0.25, 0.25, 2.0, 0.0];
        // The single vertex first: the point is the vertex and the last lane the distance.
        let (point, normal) = recalc(&mut e, (&[vertex], &corners), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(point, vertex);
        assert!(near(normal, [0.0, 0.0, 1.0, 2.0], 2e-3));
        // The triangle first: the vertex is moved onto the plane and the distance negated.
        let (point, normal) = recalc(&mut e, (&corners, &[vertex]), [0.0, 0.0, 1.0, 0.0]);
        assert!(near(point, [0.25, 0.25, 0.0, 0.0], 2e-3));
        assert!(near(normal, [0.0, 0.0, 1.0, -2.0], 2e-3));
    }

    #[test]
    fn recalc_contact_of_two_edges_uses_the_segment_routine() {
        let mut e = test_engine();
        image_constants(&mut e);
        let log = segment_double(&mut e);
        let (point, normal) = recalc(
            &mut e,
            (
                &[[0.0; 4], [1.0, 0.0, 0.0, 0.0]],
                &[[0.0, 0.0, 1.0, 0.0], [0.0, 1.0, 1.0, 0.0]],
            ),
            [0.0, 1.0, 0.0, 0.0],
        );
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(point, [4.0, 5.0, 6.0, 7.0]);
        assert_eq!(normal, [0.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn recalc_contact_ignores_other_simplex_sizes() {
        let mut e = test_engine();
        image_constants(&mut e);
        let corners = [[0.0; 4], [1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]];
        let (point, normal) = recalc(&mut e, (&corners, &corners), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!((point, normal), ([9.0; 4], [8.0; 4]));
        let (point, normal) = recalc(&mut e, (&[], &corners), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!((point, normal), ([9.0; 4], [8.0; 4]));
    }

    /// A segment simplex `(0,0,0)-(2,0,0)` with parallel vectors, and its count.
    fn simplex(e: &mut Engine) -> (u32, u32) {
        let s = e.mem.alloc(0x80);
        st(e, s, [0.0; 4]);
        st(e, s + 0x10, [2.0, 0.0, 0.0, 0.0]);
        st(e, s + 0x40, [0.5, 0.5, 0.5, 0.5]);
        st(e, s + 0x50, [0.75, 0.75, 0.75, 0.75]);
        let count = e.mem.alloc(0x10);
        e.mem.set_u32(count, 2);
        (s, count)
    }

    #[test]
    fn segment_reduction_builds_a_perpendicular_when_the_point_is_between_the_ends() {
        let mut e = test_engine();
        let (s, count) = simplex(&mut e);
        let out = e.mem.alloc(0x10);
        let point = vector(&mut e, [1.0, 1.0, 0.0, 0.0]);
        e.call(0x00da_9210, &args![s, count, 1u32, out, point]);
        assert!(near(ld(&e, out), [0.0, 2.0, 0.0, 0.0], 1e-3));
        assert_eq!(e.mem.u32(count), 2);
        e.call(0x00da_9210, &args![s, count, 0u32, out, point]);
        assert!(near(ld(&e, out), [0.0, -2.0, 0.0, 0.0], 1e-3));
    }

    #[test]
    fn segment_reduction_keeps_the_nearer_end_when_the_point_is_outside() {
        let mut e = test_engine();
        // Beyond the second end: both dot products are negative, the second end replaces the
        // first (with its parallel vector) and the count drops.
        let (s, count) = simplex(&mut e);
        let out = e.mem.alloc(0x10);
        let point = vector(&mut e, [3.0, 1.0, 0.0, 0.0]);
        e.call(0x00da_9210, &args![s, count, 1u32, out, point]);
        assert_eq!(ld(&e, s), [2.0, 0.0, 0.0, 0.0]);
        assert_eq!(ld(&e, s + 0x40), [0.75; 4]);
        assert_eq!(e.mem.u32(count), 1);
        assert_eq!(ld(&e, out), [1.0, 1.0, 0.0, 0.0]);
        // Before the first end: both are positive, nothing is copied.
        let (s, count) = simplex(&mut e);
        let point = vector(&mut e, [-1.0, 1.0, 0.0, 0.0]);
        e.call(0x00da_9210, &args![s, count, 0u32, out, point]);
        assert_eq!(ld(&e, s), [0.0; 4]);
        assert_eq!(ld(&e, s + 0x40), [0.5; 4]);
        assert_eq!(e.mem.u32(count), 1);
        assert_eq!(ld(&e, out), [1.0, -1.0, 0.0, 0.0]);
    }

    /// The shapes of a closest-point query: vertices converted by slot `0x3c`, supports by
    /// slot `0x38`. Returns the shapes and the log of the `0x3c` calls.
    fn point_shapes(
        e: &mut Engine,
        vertices: (&[Vec4], &[Vec4]),
        supports: (Vec4, Vec4),
    ) -> (u32, u32, Rc<RefCell<Vec<Vec<u32>>>>) {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut make = |address: u32, list: Vec<Vec4>, support: Vec4, radius: f32| {
            let table = e.mem.alloc(0x40);
            e.mem.set_u32(table + 0x38, address);
            e.mem.set_u32(table + 0x3c, address + 0x100);
            let shape = e.mem.alloc(0x40);
            e.mem.set_u32(shape, table);
            e.mem.set_f32(shape + 0x10, radius);
            e.register_double(address, move |e, a| {
                st(e, a[2], support);
                Ret::default()
            });
            let sink = Rc::clone(&log);
            e.register_double(address + 0x100, move |e, a| {
                sink.borrow_mut().push(a.to_vec());
                for (i, v) in list.iter().enumerate() {
                    st(e, a[3] + 0x10 * i as u32, *v);
                }
                Ret::default()
            });
            shape
        };
        let a = make(0x00ee_0011, vertices.0.to_vec(), supports.0, 0.25);
        let b = make(0x00ee_0012, vertices.1.to_vec(), supports.1, 0.25);
        (a, b, log)
    }

    /// A query record `(transform, motion, shape A, shape B, limit)`: an identity transform
    /// and a motion with the position `(5, 0, 0)`.
    fn query(e: &mut Engine, shapes: (u32, u32), limit: f32) -> u32 {
        let transform = e.mem.alloc(0x40);
        let motion = e.mem.alloc(0x40);
        for (i, row) in [
            [1.0f32, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ]
        .into_iter()
        .enumerate()
        {
            st(e, transform + 0x10 * i as u32, row);
            st(e, motion + 0x10 * i as u32, row);
        }
        st(e, motion + 0x30, [5.0, 0.0, 0.0, 0.0]);
        let record = e.mem.alloc(0x20);
        e.mem.set_u32(record, transform);
        e.mem.set_u32(record + 4, motion);
        e.mem.set_u32(record + 8, shapes.0);
        e.mem.set_u32(record + 0xc, shapes.1);
        e.mem.set_f32(record + 0x10, limit);
        record
    }

    /// A cache with the vertex counts and the flag byte `0x44`.
    fn cache(e: &mut Engine, counts: (u8, u8)) -> u32 {
        let c = e.mem.alloc(0x20);
        e.mem.set_u8(c + 8, counts.0);
        e.mem.set_u8(c + 9, counts.1);
        e.mem.set_u8(c + 0xa, 0x44);
        c
    }

    #[test]
    fn get_closest_point_places_the_point_when_the_gap_is_below_the_limit() {
        let mut e = test_engine();
        image_constants(&mut e);
        let one = [1.0, 0.0, 0.0, 0.0];
        let (a, b, log) = point_shapes(&mut e, (&[one], &[[0.0; 4]]), (one, [0.0; 4]));
        let record = query(&mut e, (a, b), 1.0);
        let cache = cache(&mut e, (1, 1));
        let normal_out = e.mem.alloc(0x10);
        let point_out = e.mem.alloc(0x10);
        let got = e.call(0x00da_bca0, &args![record, cache, normal_out, point_out]);
        assert_eq!(got.u32(), 0);
        // Both shapes were asked to convert their one id each.
        let conversions = log.borrow();
        assert_eq!(conversions.len(), 2);
        assert_eq!(conversions[0][1..3], [cache, 1]);
        assert_eq!(conversions[1][1..3], [cache + 2, 1]);
        // The distance is 1: the gap is 1 - 0.25 - 0.25.
        let normal = ld(&e, normal_out);
        assert!(near(normal, [1.0, 0.0, 0.0, 0.5], 1e-3));
        // With a single vertex in the first simplex the factor is radius_b - distance = -0.75.
        assert!(near(ld(&e, point_out), [5.25, 0.0, 0.0, 0.0], 1e-3));
    }

    #[test]
    fn get_closest_point_gives_up_when_the_gap_reaches_the_limit() {
        let mut e = test_engine();
        image_constants(&mut e);
        let one = [1.0, 0.0, 0.0, 0.0];
        let (a, b, _) = point_shapes(&mut e, (&[one], &[[0.0; 4]]), (one, [0.0; 4]));
        let record = query(&mut e, (a, b), 0.25);
        let cache = cache(&mut e, (1, 1));
        let normal_out = e.mem.alloc(0x10);
        let point_out = e.mem.alloc(0x10);
        st(&mut e, point_out, [9.0; 4]);
        let got = e.call(0x00da_bca0, &args![record, cache, normal_out, point_out]);
        assert_eq!(got.u32(), 1);
        // The normal output was written before the limit test, the point output was not.
        assert!(near(ld(&e, normal_out), [1.0, 0.0, 0.0, 0.5], 1e-3));
        assert_eq!(ld(&e, point_out), [9.0; 4]);
    }

    #[test]
    fn get_closest_point_with_an_edge_and_a_point_uses_the_second_shapes_radius() {
        let mut e = test_engine();
        image_constants(&mut e);
        // An edge on x = 1 and a point at (-1, 0, 0): distance 2 along +x.
        let edge = [[1.0, 1.0, 0.0, 0.0], [1.0, -1.0, 0.0, 0.0]];
        let behind = [-1.0, 0.0, 0.0, 0.0];
        let (a, b, _) = point_shapes(&mut e, (&edge, &[behind]), (edge[0], behind));
        let record = query(&mut e, (a, b), 3.0);
        let cache = cache(&mut e, (2, 1));
        let normal_out = e.mem.alloc(0x10);
        let point_out = e.mem.alloc(0x10);
        let got = e.call(0x00da_bca0, &args![record, cache, normal_out, point_out]);
        assert_eq!(got.u32(), 0);
        assert!(near(ld(&e, normal_out), [1.0, 0.0, 0.0, 1.5], 1e-2));
        // The factor is the second radius (0.25): 0.25 * (1, 0, 0, 2) + (-1, 0, 0, 0), placed
        // by the motion's position (5, 0, 0).
        assert!(near(ld(&e, point_out), [4.25, 0.0, 0.0, 0.0], 1e-2));
    }
}

#[cfg(test)]
mod linear_cast_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    type Directions = Rc<RefCell<Vec<Vec4>>>;
    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;
    type Copies = Rc<RefCell<Vec<(u32, u32, f32, u32)>>>;

    const LINEAR_CAST: u32 = 0x00da_9b30;
    const CAST_RAY: u32 = 0x00da_ac40;
    const ORIGIN: Vec4 = [0.0; 4];
    const SENTINEL: Vec4 = [9.0; 4];
    const SPHERE_CENTRE: Vec4 = [10.0, 0.0, 0.0, 0.0];

    /// An engine with the exe's data range and the import page mapped and the constants the cast
    /// reads.
    fn cast_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        for i in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * i, 0x8000_0000);
        }
        e.mem.set_f32(MINUS_ONE, -1.0);
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
        e.mem.set_f64(EDGE_TOLERANCE, 0.001);
        e.mem.set_f64(LINEAR_CAST_BOUND_LIMIT, 1e14);
        e.mem.set_f32(LINEAR_CAST_START_DISTANCE, 1e7);
        e.mem.set_f64(LINEAR_CAST_STALL_LIMIT, 1e-10);
        e
    }

    /// A shape double with the given radius (at +0x10). Its vertex-id function (slot 0x3c)
    /// writes `first`; its support function (slot 0x38) writes the next of `supports` (the
    /// last repeats) and logs the direction it was asked about.
    fn shape(
        e: &mut Engine,
        tag: u32,
        radius: f32,
        first: Vec4,
        supports: &[Vec4],
    ) -> (u32, Directions) {
        let table = e.mem.alloc(0x40);
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, table);
        e.mem.set_f32(object + 0x10, radius);
        let ids_code = 0x00ee_0000 + tag * 0x10;
        let support_code = ids_code + 4;
        e.mem.set_u32(table + 0x3c, ids_code);
        e.mem.set_u32(table + 0x38, support_code);
        e.register_double(ids_code, move |e, a| {
            st(e, a[3], first);
            Ret::default()
        });
        let directions: Directions = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&directions);
        let supports = supports.to_vec();
        let next = Cell::new(0usize);
        e.register_double(support_code, move |e, a| {
            sink.borrow_mut().push(ld(e, a[1]));
            let at = next.get().min(supports.len() - 1);
            next.set(next.get() + 1);
            st(e, a[2], supports[at]);
            Ret::default()
        });
        (object, directions)
    }

    fn make_ray(e: &mut Engine, start: Vec4, end: Vec4) -> u32 {
        let at = e.mem.alloc(0x20);
        st(e, at, start);
        st(e, at + 0x10, end);
        at
    }

    /// A result record: the vector holds `SENTINEL`, the hit fraction at +0x10 is `fraction`.
    fn record(e: &mut Engine, fraction: f32) -> u32 {
        let at = e.mem.alloc(0x60);
        st(e, at, SENTINEL);
        e.mem.set_f32(at + 0x10, fraction);
        at
    }

    fn near(actual: Vec4, expected: Vec4, tolerance: f32) -> bool {
        actual
            .into_iter()
            .zip(expected)
            .all(|(a, b)| (a - b).abs() <= tolerance)
    }

    /// Runs `linearCast` and returns the byte it wrote (0xff before the call).
    fn cast(e: &mut Engine, this: u32, shapes: (u32, u32), ray: u32, result: u32) -> u8 {
        let found = e.mem.alloc(4);
        e.mem.set_u8(found, 0xff);
        let transform = e.mem.alloc(0x40);
        for i in 0..4u32 {
            let mut row = ORIGIN;
            row[i as usize] = 1.0;
            st(e, transform + 0x10 * i, row);
        }
        e.call(
            LINEAR_CAST,
            &args![this, found, shapes.0, shapes.1, transform, ray, result],
        );
        e.mem.u8(found)
    }

    /// The unit sphere at `SPHERE_CENTRE` (radius 1) whose support point is its centre.
    fn sphere(e: &mut Engine, radius: f32, supports: &[Vec4]) -> (u32, Directions) {
        shape(e, 1, radius, SPHERE_CENTRE, supports)
    }

    fn assert_untouched(e: &Engine, result: u32, fraction: f32) {
        assert_eq!(ld(e, result), SENTINEL);
        assert_eq!(e.mem.f32(result + 0x10), fraction);
    }

    #[test]
    fn a_ray_toward_a_sphere_stops_at_its_surface() {
        let mut e = cast_engine();
        let (shape_a, directions) = sphere(&mut e, 1.0, &[SPHERE_CENTRE]);
        let this = e.mem.alloc(0x200);
        let ray = make_ray(&mut e, ORIGIN, [20.0, 0.0, 0.0, 0.0]);
        let result = record(&mut e, 1.0);
        assert_eq!(cast(&mut e, this, (shape_a, 0), ray, result), 1);
        assert!((e.mem.f32(result + 0x10) - 0.45).abs() < 1e-4);
        assert!(near(ld(&e, result), [-1.0, 0.0, 0.0, 0.0], 1e-4));
        // One pass: the support direction was the vector from the centre to the ray start.
        assert_eq!(*directions.borrow(), [[-10.0, 0.0, 0.0, 0.0]]);
        assert_eq!(e.mem.u32(this), 1);
        assert_eq!(e.mem.u32(this + 4), 1);
        assert_eq!(ld(&e, this + 0x20), SPHERE_CENTRE);
        assert!(near(ld(&e, this + 0xa0), [9.0, 0.0, 0.0, 0.0], 1e-3));
        assert_eq!(e.mem.u32(this + 0x170), 1);
    }

    #[test]
    fn a_ray_moving_away_from_the_sphere_misses() {
        let mut e = cast_engine();
        let (shape_a, _) = sphere(&mut e, 1.0, &[SPHERE_CENTRE]);
        let this = e.mem.alloc(0x200);
        let ray = make_ray(&mut e, ORIGIN, [-20.0, 0.0, 0.0, 0.0]);
        let result = record(&mut e, 1.0);
        assert_eq!(cast(&mut e, this, (shape_a, 0), ray, result), 0);
        assert_untouched(&e, result, 1.0);
    }

    #[test]
    fn a_ray_that_ends_before_the_surface_misses() {
        let mut e = cast_engine();
        let (shape_a, _) = sphere(&mut e, 1.0, &[SPHERE_CENTRE]);
        let this = e.mem.alloc(0x200);
        // The advance would be 4.5 of the ray, which is not below one.
        let ray = make_ray(&mut e, ORIGIN, [2.0, 0.0, 0.0, 0.0]);
        let result = record(&mut e, 1.0);
        assert_eq!(cast(&mut e, this, (shape_a, 0), ray, result), 0);
        assert_untouched(&e, result, 1.0);
    }

    #[test]
    fn a_hit_no_nearer_than_the_recorded_one_is_not_stored() {
        let mut e = cast_engine();
        let (shape_a, _) = sphere(&mut e, 1.0, &[SPHERE_CENTRE]);
        let this = e.mem.alloc(0x200);
        let ray = make_ray(&mut e, ORIGIN, [20.0, 0.0, 0.0, 0.0]);
        let result = record(&mut e, 0.1);
        assert_eq!(cast(&mut e, this, (shape_a, 0), ray, result), 0);
        assert_untouched(&e, result, 0.1);
    }

    #[test]
    fn a_radius_beyond_the_limit_ends_the_cast_with_a_zero_fraction_hit() {
        let mut e = cast_engine();
        let (shape_a, directions) = sphere(&mut e, 1e8, &[SPHERE_CENTRE]);
        let this = e.mem.alloc(0x200);
        let ray = make_ray(&mut e, ORIGIN, [20.0, 0.0, 0.0, 0.0]);
        let result = record(&mut e, 1.0);
        assert_eq!(cast(&mut e, this, (shape_a, 0), ray, result), 1);
        // The loop never ran: the fraction is the initial zero and the normal the initial zeros.
        assert!(directions.borrow().is_empty());
        assert_eq!(e.mem.f32(result + 0x10), 0.0);
        assert_eq!(ld(&e, result), ORIGIN);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 1);
    }

    #[test]
    fn a_second_shape_adds_its_radius_and_subtracts_its_rotated_support_point() {
        let mut e = cast_engine();
        let (shape_a, _) = sphere(&mut e, 1.0, &[SPHERE_CENTRE]);
        let (shape_b, b_directions) = shape(&mut e, 2, 0.5, ORIGIN, &[ORIGIN]);
        let rotated: Calls = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&rotated);
        e.register_double(SET_ROTATED_DIR, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            st(e, a[0], [2.0, 0.0, 0.0, 0.0]);
            Ret::default()
        });
        let this = e.mem.alloc(0x200);
        let ray = make_ray(&mut e, ORIGIN, [20.0, 0.0, 0.0, 0.0]);
        let result = record(&mut e, 1.0);
        assert_eq!(cast(&mut e, this, (shape_a, shape_b), ray, result), 1);
        // The vertex is A's support point minus the rotated support point of B (8), the radius
        // sum 1.5: the sphere surface is met after (8 - 1.5) / 20 of the ray.
        assert!((e.mem.f32(result + 0x10) - 0.325).abs() < 1e-4);
        assert_eq!(ld(&e, this + 0x20), [8.0, 0.0, 0.0, 0.0]);
        assert_eq!(b_directions.borrow().len(), 1);
        assert!(near(b_directions.borrow()[0], [10.0, 0.0, 0.0, 0.0], 1e-4));
        let rotated = rotated.borrow();
        assert_eq!(rotated.len(), 1);
        assert_ne!(rotated[0][0], 0);
        assert_ne!(rotated[0][1], 0);
        assert_ne!(rotated[0][2], 0);
    }

    #[test]
    fn a_vertex_already_in_the_list_is_not_added_again_and_a_receding_ray_ends_the_cast() {
        let mut e = cast_engine();
        let (shape_a, directions) = sphere(&mut e, 1.0, &[SPHERE_CENTRE]);
        let this = e.mem.alloc(0x200);
        // Oblique: the first pass advances to (9, 9, 0); the second pass finds the same vertex
        // and the ray no longer approaches along the new normal.
        let ray = make_ray(&mut e, ORIGIN, [20.0, 20.0, 0.0, 0.0]);
        let result = record(&mut e, 1.0);
        assert_eq!(cast(&mut e, this, (shape_a, 0), ray, result), 0);
        assert_eq!(directions.borrow().len(), 2);
        assert!(near(directions.borrow()[1], [-1.0, 9.0, 0.0, 0.0], 1e-3));
        assert_eq!(e.mem.u32(this), 1);
        assert_untouched(&e, result, 1.0);
    }

    #[test]
    fn a_second_vertex_goes_through_the_segment_case_and_the_nearest_hit_is_kept() {
        let mut e = cast_engine();
        let second = [10.0, 10.0, 0.0, 0.0];
        let (shape_a, directions) = sphere(&mut e, 1.0, &[SPHERE_CENTRE, second]);
        let this = e.mem.alloc(0x200);
        let ray = make_ray(&mut e, ORIGIN, [20.0, 20.0, 0.0, 0.0]);
        let result = record(&mut e, 1.0);
        assert_eq!(cast(&mut e, this, (shape_a, 0), ray, result), 1);
        assert_eq!(directions.borrow().len(), 2);
        assert_eq!(e.mem.u32(this), 2);
        assert_eq!(ld(&e, this + 0x20), SPHERE_CENTRE);
        assert_eq!(ld(&e, this + 0x30), second);
        // The point (9, 9, 0) lies beside the segment: the normal points along -x, 10 long.
        assert!(near(ld(&e, this + 0x120), [-10.0, 0.0, 0.0, 0.0], 0.05));
        assert_eq!(e.mem.u32(this + 0x170), 1);
        // The hit stored is the first pass's: fraction 0.45 and the first search direction.
        assert!((e.mem.f32(result + 0x10) - 0.45).abs() < 1e-4);
        assert!(near(ld(&e, result), [-1.0, 0.0, 0.0, 0.0], 1e-4));
    }

    /// The thread's monitor stream with room for `room` bytes of records.
    fn monitor(e: &mut Engine, room: u32) -> (u32, u32) {
        e.set_global(MONITOR_SLOT, 3u32);
        let stream = e.mem.alloc(0x20);
        let records = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, records);
        e.mem.set_u32(stream + 0xc, records + room);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        (stream, records)
    }

    /// Doubles the hit-record copy: logs `(destination, source, source fraction, source word at
    /// +0x4c)`.
    fn copy_double(e: &mut Engine) -> Copies {
        let log: Copies = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(COPY_HIT_RECORD, move |e, a| {
            sink.borrow_mut()
                .push((a[0], a[1], e.mem.f32(a[1] + 0x10), e.mem.u32(a[1] + 0x4c)));
            Ret::default()
        });
        log
    }

    fn cast_ray(e: &mut Engine, shape_a: u32, ray: u32, hit: u32) -> u8 {
        let this = e.mem.alloc(0x200);
        let found = e.mem.alloc(4);
        e.mem.set_u8(found, 0xff);
        e.call(CAST_RAY, &args![this, found, shape_a, ray, hit]);
        e.mem.u8(found)
    }

    #[test]
    fn cast_ray_copies_the_local_record_back_over_the_hit_when_the_cast_found_one() {
        let mut e = cast_engine();
        let (stream, records) = monitor(&mut e, 0x100);
        let copies = copy_double(&mut e);
        let (shape_a, _) = sphere(&mut e, 1.0, &[SPHERE_CENTRE]);
        let ray = make_ray(&mut e, ORIGIN, [20.0, 0.0, 0.0, 0.0]);
        let hit = record(&mut e, 1.0);
        e.mem.set_u32(hit + 0x4c, 0x1234);
        assert_eq!(cast_ray(&mut e, shape_a, ray, hit), 1);
        let copies = copies.borrow();
        assert_eq!(copies.len(), 1);
        assert_eq!(copies[0].0, hit);
        assert_ne!(copies[0].1, hit);
        // The local started as a copy of the record (the word at +0x4c) and the cast moved its
        // fraction.
        assert!((copies[0].2 - 0.45).abs() < 1e-4);
        assert_eq!(copies[0].3, 0x1234);
        assert_eq!(e.mem.f32(hit + 0x10), 1.0);
        // The start record and the end record, 12 bytes each.
        assert_eq!(e.mem.u32(stream + 4), records + 0x18);
        assert_eq!(e.mem.u32(records), LABEL_CAST_RAY_START);
        assert_eq!(e.mem.u32(records + 0xc), LABEL_CAST_RAY_END);
    }

    #[test]
    fn cast_ray_answers_its_found_pointer() {
        let mut e = cast_engine();
        monitor(&mut e, 0x100);
        copy_double(&mut e);
        let (shape_a, _) = sphere(&mut e, 1.0, &[SPHERE_CENTRE]);
        let ray = make_ray(&mut e, ORIGIN, [20.0, 0.0, 0.0, 0.0]);
        let hit = record(&mut e, 1.0);
        let this = e.mem.alloc(0x200);
        let found = e.mem.alloc(4);
        // EAX is `found` at every RET (the hkBool result pointer the callers read through).
        let answered = e
            .call(CAST_RAY, &args![this, found, shape_a, ray, hit])
            .u32();
        assert_eq!(answered, found);
    }

    #[test]
    fn cast_ray_reports_no_hit_when_the_cast_missed_and_a_full_stream_takes_no_records() {
        let mut e = cast_engine();
        let (stream, records) = monitor(&mut e, 0);
        let copies = copy_double(&mut e);
        let (shape_a, _) = sphere(&mut e, 1.0, &[SPHERE_CENTRE]);
        let ray = make_ray(&mut e, ORIGIN, [-20.0, 0.0, 0.0, 0.0]);
        let hit = record(&mut e, 1.0);
        assert_eq!(cast_ray(&mut e, shape_a, ray, hit), 0);
        assert!(copies.borrow().is_empty());
        assert_eq!(e.mem.u32(stream + 4), records);
        assert_eq!(e.mem.u32(records), 0);
    }

    #[test]
    fn cast_ray_treats_a_hit_with_an_exactly_zero_fraction_as_no_hit() {
        let mut e = cast_engine();
        monitor(&mut e, 0x100);
        let copies = copy_double(&mut e);
        // The radius is beyond the limit, so the cast reports a hit at fraction zero.
        let (shape_a, _) = sphere(&mut e, 1e8, &[SPHERE_CENTRE]);
        let ray = make_ray(&mut e, ORIGIN, [20.0, 0.0, 0.0, 0.0]);
        let hit = record(&mut e, 1.0);
        assert_eq!(cast_ray(&mut e, shape_a, ray, hit), 0);
        assert!(copies.borrow().is_empty());
    }
}

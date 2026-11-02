//! `NiCollision/run_00eada70` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both functions are x87 code; each `FADD`/`FSUB`/`FMUL`/`FDIV` goes
//! through the `x87_*` helpers in the exe's order with the operands as the
//! instruction sees them (destination first), each `FSTP dword` through
//! `x87_store32` and each `FLD dword` through `x87_load32`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The sphere `00eada70` uses when the object has none (`011f4288`: centre
/// and radius, four floats).
const DEFAULT_SPHERE: u32 = 0x011f_4288;
/// `1e-5` (`010718c0`) and `-1e-5` (`010d5368`): the determinant limits of
/// the triangle test.
const EPSILON: u32 = 0x0107_18c0;
const MINUS_EPSILON: u32 = 0x010d_5368;
/// `NiPoint3::Dot` (`004b6190`, engine map; `thiscall`, the other point;
/// ST0), the cross product (`004b3800`, `thiscall`: out, other point), the
/// point scale (`004a3760`, `cdecl`: out, float, point; answers out) and
/// the point sum (`00439e90`, `thiscall`: out, other point; answers out).
const POINT_DOT: u32 = 0x004b_6190;
const POINT_CROSS: u32 = 0x004b_3800;
const POINT_SCALE: u32 = 0x004a_3760;
const POINT_ADD: u32 = 0x0043_9e90;

/// `FLD dword [address]`.
fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

/// A `dword` memory operand at `address`.
fn operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

/// `FSTP dword` then `FLD dword` of `value` (the float a local holds).
fn through_float(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

// Translated from 00eada70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: an object, a point, a direction):
/// whether a ray from `point` along `direction` can meet the object's
/// sphere (the pointer at +0x20, or the default at `011f4288`: centre,
/// radius). True when the point is inside or on the sphere
/// (`|p - c|^2 - r^2 <= 0`); otherwise false unless the direction points
/// towards the centre (`(p - c) . d < 0`, a NaN counting as towards), then
/// whether `((p - c) . d)^2 >= |d|^2 * (|p - c|^2 - r^2)`.
pub fn fn_00eada70(e: &mut Engine, holder: Ptr, point: Ptr, direction: Ptr) -> bool {
    let mut sphere = e.mem.u32(holder.addr() + 0x20);
    if sphere == 0 {
        sphere = DEFAULT_SPHERE;
    }
    let centre = [
        e.mem.u32(sphere),
        e.mem.u32(sphere + 4),
        e.mem.u32(sphere + 8),
    ];
    let radius = e.mem.u32(sphere + 0xc);
    let p = point.addr();
    let dx = through_float(x87_sub(load(e, p), x87_m32(centre[0])));
    let dy = through_float(x87_sub(load(e, p + 4), x87_m32(centre[1])));
    let dz = through_float(x87_sub(load(e, p + 8), x87_m32(centre[2])));
    let r = x87_load32(radius);
    let dy2 = x87_mul(dy, dy);
    let dx2 = x87_mul(dx, dx);
    let s2 = x87_add(dx2, dy2);
    let dz2 = x87_mul(dz, dz);
    let s3 = x87_store32(x87_add(s2, dz2));
    let r2 = x87_mul(r, r);
    let k = through_float(x87_sub(x87_m32(s3), r2));
    if k <= 0.0 {
        return true;
    }
    let d = direction.addr();
    let a = x87_mul(dx, load(e, d));
    let b = x87_mul(dy, load(e, d + 4));
    let ab = x87_add(b, a);
    let c = x87_mul(dz, load(e, d + 8));
    let dot = through_float(x87_add(c, ab));
    if dot >= 0.0 {
        return false;
    }
    let y = load(e, d + 4);
    let x = load(e, d);
    let z = load(e, d + 8);
    let dot2 = x87_mul(dot, dot);
    let y2 = x87_mul(y, y);
    let x2 = x87_mul(x, x);
    let xy = x87_add(x2, y2);
    let z2 = x87_mul(z, z);
    let n = through_float(x87_add(z2, xy));
    let kn = x87_mul(k, n);
    dot2 >= kn
}

/// Offsets of the triangle test's locals in its block (the exe's frame
/// below the saved registers): the two edges, the point/cross scratch and
/// the cross product block.
const EDGE1: u32 = 0x0;
const EDGE2: u32 = 0xc;
const SCRATCH: u32 = 0x18;
const CROSS: u32 = 0x24;

/// A float stored into a local and loaded back, and its bits.
fn store_local(e: &mut Engine, address: u32, value: f64) -> f64 {
    let bits = x87_store32(value);
    e.mem.set_u32(address, bits);
    x87_load32(bits)
}

// Translated from 00eadb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, ten words): a ray/triangle test
/// (origin `o`, direction `d`, corners `v0`, `v1`, `v2`; `cull` refuses
/// back faces). With `e1 = v1 - v0`, `e2 = v2 - v0`, `p = d x e2` and
/// `det = e1 . p`: for `det >= 1e-5`, `u = (o - v0) . p` (to `u_out`) must be
/// in `[0, det]`, `q = (o - v0) x e1`, `v = d . q` (to `v_out`) at least 0
/// with `u + v <= det`, and `t = e2 . q` (to `t_out`) at least 0. For
/// `det <= -1e-5` without `cull` the same tests run mirrored (`u` in
/// `[det, 0]`, `v <= 0`, `u + v >= det`, `t <= 0`), the cross product and
/// dots through `004b3800`/`004b6190`. On success `u`, `v` and `t` are
/// divided by `det` (times `1 / det`) and `point_out` gets `o + d * t`;
/// answers true. A NaN passes the tests the exe's flag checks let it pass.
#[allow(clippy::too_many_arguments)]
pub fn fn_00eadb90(
    e: &mut Engine,
    origin: Ptr,
    direction: Ptr,
    v0: Ptr,
    v1: Ptr,
    v2: Ptr,
    cull: u8,
    point_out: Ptr,
    t_out: Ptr,
    u_out: Ptr,
    v_out: Ptr,
) -> bool {
    let (o, d, a, b, c) = (
        origin.addr(),
        direction.addr(),
        v0.addr(),
        v1.addr(),
        v2.addr(),
    );
    let (t_out, u_out, v_out) = (t_out.addr(), u_out.addr(), v_out.addr());
    e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        let edge1 = frame + EDGE1;
        let edge2 = frame + EDGE2;
        let scratch = frame + SCRATCH;
        let cross = frame + CROSS;
        for i in 0..3u32 {
            let value = x87_sub(load(e, b + 4 * i), operand(e, a + 4 * i));
            e.mem.set_u32(edge1 + 4 * i, x87_store32(value));
        }
        for i in 0..3u32 {
            let value = x87_sub(load(e, c + 4 * i), operand(e, a + 4 * i));
            e.mem.set_u32(edge2 + 4 * i, x87_store32(value));
        }
        // p = d x e2, into the scratch block.
        let dy_e2z = x87_mul(load(e, d + 4), load(e, edge2 + 8));
        let dz_e2y = x87_mul(load(e, d + 8), load(e, edge2 + 4));
        store_local(e, scratch, x87_sub(dy_e2z, dz_e2y));
        let e2z = load(e, edge2 + 8);
        let e2x = load(e, edge2);
        let dz_e2x = x87_mul(load(e, d + 8), e2x);
        let e2z_dx = x87_mul(e2z, load(e, d));
        store_local(e, scratch + 4, x87_sub(dz_e2x, e2z_dx));
        let e2y = load(e, edge2 + 4);
        let e2y_dx = x87_mul(e2y, load(e, d));
        let e2x_dy = x87_mul(e2x, operand(e, d + 4));
        store_local(e, scratch + 8, x87_sub(e2y_dx, e2x_dy));
        // det = e1 . p
        let py = load(e, scratch + 4);
        let e1y = load(e, edge1 + 4);
        let py_e1y = x87_mul(py, e1y);
        let px = load(e, scratch);
        let e1x = load(e, edge1);
        let px_e1x = x87_mul(px, e1x);
        let sum = x87_add(px_e1x, py_e1y);
        let pz = load(e, scratch + 8);
        let pz_e1z = x87_mul(pz, operand(e, edge1 + 8));
        let det = through_float(x87_add(sum, pz_e1z));
        let epsilon = load(e, EPSILON);
        // T = o - v0, into the scratch block (p stays on the x87 stack).
        let fill_t = |e: &mut Engine| {
            let mut t = [0.0f64; 3];
            for i in 0..3u32 {
                let value = x87_sub(load(e, o + 4 * i), operand(e, a + 4 * i));
                t[i as usize] = store_local(e, scratch + 4 * i, value);
            }
            t
        };
        if det >= epsilon {
            let [tx, ty, tz] = fill_t(e);
            let py_ty = x87_mul(py, ty);
            let px_tx = x87_mul(px, tx);
            let s1 = x87_add(px_tx, py_ty);
            let pz_tz = x87_mul(pz, tz);
            let u_bits = x87_store32(x87_add(pz_tz, s1));
            let u = x87_load32(u_bits);
            e.mem.set_u32(u_out, u_bits);
            if 0.0 > u || det < u {
                return false;
            }
            // q = T x e1, into the scratch block.
            let ty_e1z = x87_mul(ty, operand(e, edge1 + 8));
            let tz_e1y = x87_mul(tz, e1y);
            store_local(e, scratch, x87_sub(ty_e1z, tz_e1y));
            let tz_e1x = x87_mul(tz, e1x);
            let tx_e1z = x87_mul(tx, operand(e, edge1 + 8));
            store_local(e, scratch + 4, x87_sub(tz_e1x, tx_e1z));
            let e1y_tx = x87_mul(e1y, tx);
            let e1x_ty = x87_mul(e1x, ty);
            store_local(e, scratch + 8, x87_sub(e1y_tx, e1x_ty));
            let dy_qy = x87_mul(load(e, d + 4), operand(e, scratch + 4));
            let qx_dx = x87_mul(load(e, scratch), operand(e, d));
            let s = x87_add(dy_qy, qx_dx);
            let dz_qz = x87_mul(load(e, d + 8), operand(e, scratch + 8));
            let v_bits = x87_store32(x87_add(s, dz_qz));
            let v = x87_load32(v_bits);
            e.mem.set_u32(v_out, v_bits);
            if 0.0 > v {
                return false;
            }
            let uv = x87_add(v, operand(e, u_out));
            if det < uv {
                return false;
            }
            let t = e.call(POINT_DOT, &args![edge2, scratch]).f64();
            let t_bits = x87_store32(t);
            e.mem.set_u32(t_out, t_bits);
            if 0.0 > x87_load32(t_bits) {
                return false;
            }
        } else {
            let minus_epsilon = load(e, MINUS_EPSILON);
            if !(minus_epsilon >= det) || cull != 0 {
                return false;
            }
            let [_, ty, tz] = fill_t(e);
            let py_ty = x87_mul(py, ty);
            let px_tx = x87_mul(px, operand(e, scratch));
            let s1 = x87_add(py_ty, px_tx);
            let pz_tz = x87_mul(pz, tz);
            let u_bits = x87_store32(x87_add(pz_tz, s1));
            let u = x87_load32(u_bits);
            e.mem.set_u32(u_out, u_bits);
            if 0.0 < u || u < det {
                return false;
            }
            e.call(POINT_CROSS, &args![scratch, cross, edge1]);
            let v = e.call(POINT_DOT, &args![d, cross]).f64();
            let v_bits = x87_store32(v);
            e.mem.set_u32(v_out, v_bits);
            let v = x87_load32(v_bits);
            if 0.0 < v {
                return false;
            }
            let uv = x87_add(v, operand(e, u_out));
            if det > uv {
                return false;
            }
            let t = e.call(POINT_DOT, &args![edge2, cross]).f64();
            let t_bits = x87_store32(t);
            e.mem.set_u32(t_out, t_bits);
            if 0.0 < x87_load32(t_bits) {
                return false;
            }
        }
        // Success: scale by 1 / det and build the hit point.
        let inverse = through_float(x87_div(1.0, det));
        let u = x87_mul(load(e, u_out), inverse);
        e.mem.set_u32(u_out, x87_store32(u));
        let v = x87_mul(load(e, v_out), inverse);
        e.mem.set_u32(v_out, x87_store32(v));
        let t_bits = x87_store32(x87_mul(inverse, operand(e, t_out)));
        let t_bits = x87_quiet_bits(t_bits);
        e.mem.set_u32(t_out, t_bits);
        let scaled = e.call(POINT_SCALE, &args![cross, t_bits, d]).u32();
        let sum = e.call(POINT_ADD, &args![o, scratch, scaled]).u32();
        for i in 0..3u32 {
            let word = e.mem.u32(sum + 4 * i);
            e.mem.set_u32(point_out.addr() + 4 * i, word);
        }
        true
    })
}

/// The nine words (a 3x3 matrix, zero in the image) `00eadf30` starts its sums from.
const ZERO_MATRIX_011F4248: u32 = 0x011f_4248;
/// The double 0.5 at `01011588`.
const HALF_01011588: u32 = 0x0101_1588;
/// The float at `0101e2bc` (0.1 in the image): the extent a flat box gets.
const SMALLEST_EXTENT_0101E2BC: u32 = 0x0101_e2bc;
/// `00a59980` (thiscall on a symmetric `NiMatrix3`: the out eigenvalues, the out axes;
/// `NiMain/nimatrix3.cpp`, no Xbox name): its eigen decomposition (by its use here).
const MATRIX_EIGEN: u32 = 0x00a5_9980;

/// Where a corner's coordinate widens a range: below the minimum (a memory word) it becomes the
/// minimum, otherwise above the maximum it becomes the maximum (each stored as a float; a NaN
/// changes neither).
fn widen(minimum: &mut u32, maximum: &mut u32, value: f64) {
    if x87_load32(*minimum) > value {
        *minimum = x87_store32(value);
    } else if value > x87_load32(*maximum) {
        *maximum = x87_store32(value);
    }
}

// Translated from 00eadf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a 0x3c-byte box: center +0, three axes +0xc, extents
/// +0x30; two words: the point count (a `u16`) and the points; `00e9c850` calls it): fits a box
/// around the points. The center is their mean (summed from the first point, times `1 / count`,
/// each stored) and the coordinate ranges are kept. When a range is empty (its maximum equals its
/// minimum) the box is axis-aligned: identity axes, extents half the ranges (a nonpositive one
/// raised to the float at `0101e2bc`). Otherwise the sums of the center's products (the same
/// products added once per point, from the zero matrix at `011f4248`) give a symmetric matrix
/// whose eigen decomposition (`00a59980`) fills the axes; the projections of `point - center` on
/// them (from the first point's raw differences) give the ranges, and the center moves to the
/// middle of each range along its axis (the 0.5 a double at `01011588`) with the extents half
/// the ranges.
pub fn fn_00eadf30(e: &mut Engine, this: Ptr, count: u32, points: Ptr) {
    let b = this.addr();
    let pts = points.addr();
    let count = count & 0xffff;
    let first = [0u32, 4, 8].map(|i| through_float(load(e, pts.wrapping_add(i))));
    let mut low = first.map(x87_store32);
    let mut high = low;
    for i in [0u32, 4, 8] {
        let word = e.mem.u32(pts.wrapping_add(i));
        e.mem.set_u32(b + i, word);
    }
    for k in 1..count {
        let p = pts.wrapping_add(12 * k);
        let x = x87_add(load(e, p), operand(e, b));
        e.mem.set_u32(b, x87_store32(x));
        let y = x87_add(load(e, b + 4), operand(e, p.wrapping_add(4)));
        e.mem.set_u32(b + 4, x87_store32(y));
        let z = x87_add(load(e, p.wrapping_add(8)), operand(e, b + 8));
        e.mem.set_u32(b + 8, x87_store32(z));
        for axis in 0..3 {
            let value = load(e, p.wrapping_add(4 * axis as u32));
            widen(&mut low[axis], &mut high[axis], value);
        }
    }
    let inverse = through_float(x87_div(1.0, f64::from(count)));
    let x = x87_mul(inverse, operand(e, b));
    e.mem.set_u32(b, x87_store32(x));
    let y = x87_mul(load(e, b + 4), inverse);
    e.mem.set_u32(b + 4, x87_store32(y));
    let z = x87_mul(inverse, operand(e, b + 8));
    e.mem.set_u32(b + 8, x87_store32(z));
    let half = x87_load64(e.mem.u64(HALF_01011588));
    let flat = (0..3).any(|axis| x87_load32(high[axis]) == x87_load32(low[axis]));
    if flat {
        let identity = [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        for (i, v) in (0u32..).zip(identity) {
            e.mem.set_u32(b + 0xc + 4 * i, v.to_bits());
        }
        let ex = x87_mul(x87_sub(x87_load32(high[0]), x87_load32(low[0])), half);
        let ey = x87_mul(x87_sub(x87_load32(high[1]), x87_load32(low[1])), half);
        let ez = x87_mul(half, x87_sub(x87_load32(high[2]), x87_m32(low[2])));
        let smallest = x87_store32(load(e, SMALLEST_EXTENT_0101E2BC));
        for (i, extent) in (0u32..).zip([ex, ey, ez]) {
            let mut word = x87_store32(extent);
            if x87_m32(word) <= 0.0 {
                word = smallest;
            }
            e.mem.set_u32(b + 0x30 + 4 * i, word);
        }
        return;
    }
    e.with_stack(0x6c, |e, frame| {
        let f = frame.addr();
        let matrix = f + 0x48;
        for i in 0..9u32 {
            let word = e.mem.u32(ZERO_MATRIX_011F4248 + 4 * i);
            e.mem.set_u32(matrix + 4 * i, word);
        }
        if count > 0 {
            let cx = load(e, b);
            let xx = x87_mul(cx, cx);
            let xy = x87_mul(load(e, b + 4), operand(e, b));
            let xz = x87_mul(load(e, b), operand(e, b + 8));
            let cy = load(e, b + 4);
            let yy = x87_mul(cy, cy);
            let yz = x87_mul(load(e, b + 4), operand(e, b + 8));
            let cz = load(e, b + 8);
            let zz = x87_mul(cz, cz);
            for _ in 0..count {
                for (slot, product) in [(0u32, xx), (1, xy), (2, xz), (4, yy), (5, yz), (8, zz)] {
                    let at = matrix + 4 * slot;
                    let sum = x87_add(load(e, at), product);
                    e.mem.set_u32(at, x87_store32(sum));
                }
            }
        }
        for (from, to) in [(1u32, 3u32), (2, 6), (5, 7)] {
            let word = x87_quiet_bits(e.mem.u32(matrix + 4 * from));
            e.mem.set_u32(matrix + 4 * to, word);
        }
        let axes = b + 0xc;
        e.call(MATRIX_EIGEN, &args![matrix, f + 0x3c, axes]);
        let a = |e: &Engine, i: u32| load(e, axes + 4 * i);
        let m = |e: &Engine, i: u32| operand(e, axes + 4 * i);
        low = [0u32, 4, 8]
            .map(|i| x87_store32(x87_sub(load(e, pts.wrapping_add(i)), operand(e, b + i))));
        high = low;
        for k in 1..count {
            let p = pts.wrapping_add(12 * k);
            let [dx, dy, dz] = [0u32, 4, 8]
                .map(|i| through_float(x87_sub(load(e, p.wrapping_add(i)), operand(e, b + i))));
            let s = x87_add(x87_mul(a(e, 1), dy), x87_mul(dx, m(e, 0)));
            let p0 = through_float(x87_add(x87_mul(a(e, 2), dz), s));
            widen(&mut low[0], &mut high[0], p0);
            let s = x87_add(x87_mul(a(e, 3), dx), x87_mul(dy, m(e, 4)));
            let p1 = through_float(x87_add(s, x87_mul(a(e, 5), dz)));
            widen(&mut low[1], &mut high[1], p1);
            let c = x87_mul(dx, a(e, 6));
            let d = x87_mul(dy, a(e, 7));
            let s = x87_add(d, c);
            let p2 = through_float(x87_add(x87_mul(dz, a(e, 8)), s));
            widen(&mut low[2], &mut high[2], p2);
        }
        // The center moves to the middle of each range along its axis.
        let middle = |axis: usize| x87_add(x87_load32(high[axis]), x87_m32(low[axis]));
        let m0 = through_float(x87_mul(middle(0), half));
        let t = [
            through_float(x87_mul(a(e, 0), m0)),
            through_float(x87_mul(a(e, 1), m0)),
            through_float(x87_mul(m0, m(e, 2))),
        ];
        let cx = through_float(x87_add(t[0], operand(e, b)));
        e.mem.set_u32(b, x87_store32(cx));
        let cy = through_float(x87_add(load(e, b + 4), t[1]));
        e.mem.set_u32(b + 4, x87_store32(cy));
        let cz = through_float(x87_add(t[2], operand(e, b + 8)));
        e.mem.set_u32(b + 8, x87_store32(cz));
        let m1 = through_float(x87_mul(middle(1), half));
        let t = [
            through_float(x87_mul(a(e, 3), m1)),
            through_float(x87_mul(a(e, 4), m1)),
            through_float(x87_mul(m1, m(e, 5))),
        ];
        let cx = through_float(x87_add(cx, t[0]));
        e.mem.set_u32(b, x87_store32(cx));
        let cy = through_float(x87_add(cy, t[1]));
        e.mem.set_u32(b + 4, x87_store32(cy));
        let cz = through_float(x87_add(cz, t[2]));
        e.mem.set_u32(b + 8, x87_store32(cz));
        let s2 = x87_add(x87_load32(high[2]), x87_load32(low[2]));
        let m2 = through_float(x87_mul(s2, half));
        let t = [
            through_float(x87_mul(m2, m(e, 6))),
            through_float(x87_mul(a(e, 7), m2)),
            through_float(x87_mul(m2, m(e, 8))),
        ];
        e.mem.set_u32(b, x87_store32(x87_add(cx, t[0])));
        e.mem.set_u32(b + 4, x87_store32(x87_add(cy, t[1])));
        e.mem.set_u32(b + 8, x87_store32(x87_add(cz, t[2])));
        let ex = x87_mul(x87_sub(x87_load32(high[0]), x87_m32(low[0])), half);
        e.mem.set_u32(b + 0x30, x87_store32(ex));
        let ey = x87_mul(x87_sub(x87_load32(high[1]), x87_m32(low[1])), half);
        e.mem.set_u32(b + 0x34, x87_store32(ey));
        let ez = x87_mul(half, x87_sub(x87_m32(high[2]), x87_load32(low[2])));
        e.mem.set_u32(b + 0x38, x87_store32(ez));
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ea_df30, fn_00eadf30(Ptr, u32, Ptr)),
        entry!(0x00eada70, fn_00eada70(Ptr, Ptr, Ptr) -> bool),
        entry!(
            0x00eadb90,
            fn_00eadb90(Ptr, Ptr, Ptr, Ptr, Ptr, u8, Ptr, Ptr, Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(e: &mut Engine, values: [f32; 3]) -> u32 {
        let at = e.mem.alloc(12);
        for (i, v) in values.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, v);
        }
        at
    }

    fn sphere_holder(e: &mut Engine, centre: [f32; 3], radius: f32) -> u32 {
        let sphere = e.mem.alloc(16);
        for (i, v) in centre.into_iter().enumerate() {
            e.mem.set_f32(sphere + 4 * i as u32, v);
        }
        e.mem.set_f32(sphere + 12, radius);
        let holder = e.mem.alloc(0x24);
        e.mem.set_u32(holder + 0x20, sphere);
        holder
    }

    #[test]
    fn sphere_test_inside_towards_and_away() {
        let mut e = Engine::new();
        let holder = sphere_holder(&mut e, [0.0, 0.0, 0.0], 1.0);
        let inside = point(&mut e, [0.5, 0.0, 0.0]);
        let outside = point(&mut e, [5.0, 0.0, 0.0]);
        let towards = point(&mut e, [-1.0, 0.0, 0.0]);
        let away = point(&mut e, [1.0, 0.0, 0.0]);
        let tangent_miss = point(&mut e, [-1.0, 1.0, 0.0]);
        let call =
            |e: &mut Engine, p: u32, d: u32| e.call(0x00ea_da70, &args![holder, p, d]).bool();
        assert!(call(&mut e, inside, away));
        assert!(call(&mut e, outside, towards));
        assert!(!call(&mut e, outside, away));
        assert!(!call(&mut e, outside, tangent_miss));
        // Without a sphere the default at 011f4288 is used.
        e.map(0x011f_4000, 0x1000);
        e.mem.set_f32(DEFAULT_SPHERE + 12, 10.0);
        let empty = e.mem.alloc(0x24);
        assert!(e.call(0x00ea_da70, &args![empty, outside, away]).bool());
    }

    fn triangle_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0107_1000, 0x1000);
        e.mem.set_f32(EPSILON, 1e-5);
        e.map(0x010d_5000, 0x1000);
        e.mem.set_f32(MINUS_EPSILON, -1e-5);
        e.register(POINT_DOT, |e, a| {
            let mut sum = 0.0f64;
            for i in 0..3 {
                sum += f64::from(e.mem.f32(a[0] + 4 * i)) * f64::from(e.mem.f32(a[1] + 4 * i));
            }
            sum.into_ret()
        });
        e.register(POINT_CROSS, |e, a| {
            let p = |e: &Engine, at: u32| [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)];
            let (l, r) = (p(e, a[0]), p(e, a[2]));
            let out = [
                l[1] * r[2] - l[2] * r[1],
                l[2] * r[0] - l[0] * r[2],
                l[0] * r[1] - l[1] * r[0],
            ];
            for (i, v) in out.into_iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, v);
            }
            a[1].into_ret()
        });
        e.register(POINT_SCALE, |e, a| {
            let s = f32::from_bits(a[1]);
            for i in 0..3 {
                let v = e.mem.f32(a[2] + 4 * i) * s;
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            a[0].into_ret()
        });
        e.register(POINT_ADD, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i) + e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        e
    }

    /// Runs the triangle test on the unit triangle in the z = 0 plane and
    /// answers (hit, point, t, u, v).
    fn ray(
        e: &mut Engine,
        origin: [f32; 3],
        direction: [f32; 3],
        wind: bool,
        cull: u32,
    ) -> (bool, [f32; 3], f32, f32, f32) {
        let o = point(e, origin);
        let d = point(e, direction);
        let v0 = point(e, [0.0, 0.0, 0.0]);
        let (v1, v2) = if wind {
            (point(e, [1.0, 0.0, 0.0]), point(e, [0.0, 1.0, 0.0]))
        } else {
            (point(e, [0.0, 1.0, 0.0]), point(e, [1.0, 0.0, 0.0]))
        };
        let out = point(e, [0.0; 3]);
        let t = e.mem.alloc(4);
        let u = e.mem.alloc(4);
        let v = e.mem.alloc(4);
        let hit = e
            .call(0x00ea_db90, &args![o, d, v0, v1, v2, cull, out, t, u, v])
            .bool();
        (
            hit,
            [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)],
            e.mem.f32(t),
            e.mem.f32(u),
            e.mem.f32(v),
        )
    }

    #[test]
    fn front_face_hit_and_misses() {
        let mut e = triangle_engine();
        let (hit, at, t, u, v) = ray(&mut e, [0.25, 0.25, 1.0], [0.0, 0.0, -1.0], true, 0);
        assert!(hit);
        assert_eq!(at, [0.25, 0.25, 0.0]);
        assert_eq!((t, u, v), (1.0, 0.25, 0.25));
        // Outside the triangle.
        assert!(!ray(&mut e, [0.75, 0.75, 1.0], [0.0, 0.0, -1.0], true, 0).0);
        assert!(!ray(&mut e, [-0.25, 0.25, 1.0], [0.0, 0.0, -1.0], true, 0).0);
        // Behind the origin.
        assert!(!ray(&mut e, [0.25, 0.25, -1.0], [0.0, 0.0, -1.0], true, 0).0);
        // Parallel: neither determinant limit is passed.
        assert!(!ray(&mut e, [0.25, 0.25, 1.0], [1.0, 0.0, 0.0], true, 0).0);
    }

    #[test]
    fn back_face_hits_unless_culled() {
        let mut e = triangle_engine();
        let (hit, at, t, u, v) = ray(&mut e, [0.25, 0.25, 1.0], [0.0, 0.0, -1.0], false, 0);
        assert!(hit);
        assert_eq!(at, [0.25, 0.25, 0.0]);
        assert_eq!((t, u, v), (1.0, 0.25, 0.25));
        assert!(!ray(&mut e, [0.25, 0.25, 1.0], [0.0, 0.0, -1.0], false, 1).0);
        assert!(!ray(&mut e, [0.75, 0.75, 1.0], [0.0, 0.0, -1.0], false, 0).0);
    }
}

#[cfg(test)]
mod box_fit_tests {
    use super::*;

    fn points(e: &mut Engine, values: &[[f32; 3]]) -> u32 {
        let at = e.mem.alloc(12 * values.len() as u32);
        for (i, p) in (0u32..).zip(values) {
            for (k, v) in (0u32..).zip(p) {
                e.mem.set_f32(at + 12 * i + 4 * k, *v);
            }
        }
        at
    }

    fn read(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    fn setup(e: &mut Engine) {
        e.map(0x0101_1000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.mem.set_f64(HALF_01011588, 0.5);
        e.mem.set_f32(SMALLEST_EXTENT_0101E2BC, 0.1);
    }

    #[test]
    fn a_flat_set_gets_an_axis_aligned_box() {
        let mut e = Engine::new();
        setup(&mut e);
        let p = points(&mut e, &[[0.0, 0.0, 0.0], [2.0, 0.0, 4.0], [4.0, 0.0, 2.0]]);
        let b = e.mem.alloc(0x3c);
        e.call(0x00ea_df30, &args![b, 0xffff_0003u32, p]);
        assert_eq!(read(&e, b, 3), vec![2.0, 0.0, 2.0]);
        assert_eq!(
            read(&e, b + 0xc, 9),
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert_eq!(read(&e, b + 0x30, 3), vec![2.0, 0.1, 2.0]);
    }

    #[test]
    fn a_full_set_uses_the_eigen_axes() {
        let mut e = Engine::new();
        setup(&mut e);
        e.register(MATRIX_EIGEN, |e, a| {
            assert_eq!(
                read(e, a[0], 9),
                vec![27.0, 27.0, 36.0, 27.0, 27.0, 36.0, 36.0, 36.0, 48.0]
            );
            for i in 0..9 {
                let v = if i % 4 == 0 { 1.0 } else { 0.0 };
                e.mem.set_f32(a[2] + 4 * i, v);
            }
            Ret::default()
        });
        let p = points(&mut e, &[[0.0, 0.0, 0.0], [3.0, 6.0, 3.0], [6.0, 3.0, 9.0]]);
        let b = e.mem.alloc(0x3c);
        e.call(0x00ea_df30, &args![b, 3u32, p]);
        assert_eq!(read(&e, b, 3), vec![3.0, 3.0, 4.5]);
        assert_eq!(read(&e, b + 0x30, 3), vec![3.0, 3.0, 4.5]);
    }
}

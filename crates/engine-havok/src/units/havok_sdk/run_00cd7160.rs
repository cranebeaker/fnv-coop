//! `Havok SDK/run_00cd7160` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Triangle helpers of the capsule collision code. The vectors are SSE (`f32` lanes, the
//! three-component sums added as `z + (y + x)`); the scalar solving runs on the x87 stack,
//! approximated here in `f64` with a rounding to `f32` wherever the code stores a `float`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::sse::{mulps, subps};
use crate::units::havok_sdk::hkpconvexradiusbuilder::{shufps, splat};
use crate::x87::rsqrt_estimate;

type Vec4 = [f32; 4];

fn load(e: &Engine, at: Ptr) -> Vec4 {
    let at = at.addr();
    [
        e.mem.f32(at),
        e.mem.f32(at + 4),
        e.mem.f32(at + 8),
        e.mem.f32(at + 12),
    ]
}

/// `SUBPS a, b` (the SSE NaN rules).
fn sub(a: Vec4, b: Vec4) -> Vec4 {
    subps(a, b)
}

/// `MULPS a, b`, then the sum of the three lane products added as `z + (y + x)` (`ADDSS`
/// with the higher lane as the destination), with the SSE NaN rules.
fn dot3(a: Vec4, b: Vec4) -> f32 {
    sum_of_three(mulps(a, b))
}

/// `z + (y + x)` with `ADDSS` (the higher lane the destination, the SSE NaN rules).
fn sum_of_three(v: Vec4) -> f32 {
    crate::sse::add(v[2], crate::sse::add(v[1], v[0]))
}

// Translated from 00cd72a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reciprocal edge lengths of a triangle, cdecl `(triangle, out)`. `triangle` holds the
/// vertices `a`, `b`, `c` as three 16-byte vectors. Writes to `out` the refined inverse square
/// roots of `|c - b|^2`, `|a - c|^2` and `|b - a|^2` (floats 0 to 2), then that of the squared
/// length of the face normal `(a - c).zxy * (c - b).yzx - (a - c).yzx * (c - b).zxy` (float 3),
/// then the squared normal length times its inverse root (float 4: the normal's length).
/// The root is the hardware estimate (`RSQRTPS`) refined by one Newton step,
/// `(3 - (r * x) * r) * (0.5 * r)`.
pub fn fn_00cd72a0(e: &mut Engine, triangle: Ptr, out: Ptr) {
    let a = load(e, triangle);
    let b = load(e, Ptr::new(triangle.addr() + 0x10));
    let c = load(e, Ptr::new(triangle.addr() + 0x20));
    let to_a = sub(a, c);
    let from_b = sub(c, b);
    let to_b = sub(b, a);
    // `(a - c).zxyw * (c - b).yzxw - (a - c).yzxw * (c - b).zxyw` (`MULPS`, `SUBPS`, the
    // destination first; the fourth lane is not summed).
    let normal = subps(
        mulps(shufps(to_a, to_a, 0xd2), shufps(from_b, from_b, 0xc9)),
        mulps(shufps(to_a, to_a, 0xc9), shufps(from_b, from_b, 0xd2)),
    );
    // The four sums go through the x87 stack into a scratch vector (quieted), which
    // `RSQRTPS` and the Newton step (SSE NaN rules) take.
    let squares = [
        dot3(from_b, from_b),
        dot3(to_a, to_a),
        dot3(to_b, to_b),
        dot3(normal, normal),
    ]
    .map(x87_quiet);
    let refined = squares.map(|x| {
        use crate::sse::{mul, sub};
        let estimate = rsqrt_estimate(x);
        mul(
            sub(3.0, mul(mul(estimate, x), estimate)),
            mul(0.5, estimate),
        )
    });
    for (lane, value) in refined.into_iter().enumerate() {
        e.mem
            .set_f32(out.addr() + 4 * lane as u32, x87_quiet(value));
    }
    let length = x87_mul(
        x87_load32(squares[3].to_bits()),
        x87_load32(refined[3].to_bits()),
    );
    e.mem.set_u32(out.addr() + 0x10, x87_store32(length));
}

// Translated from 00cd7530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Barycentric coordinates of the point `p` against the triangle `a`, `b`, `c`, cdecl
/// `(p, a, b, c, out)`: writes the weights of `a`, `b` and `c` (3 floats at `out`) of the
/// point of the triangle closest to `p`. With `e1 = a - b`, `e2 = c - b` and a positive
/// determinant `|e1|^2 * |e2|^2 - (e1 . e2)^2` the weights are the plane projection's.
/// For a degenerate triangle the point is projected on the longest edge instead (`a`-`b`,
/// `b`-`c` or `a`-`c`; the weights of the other vertex are 0), and when every edge has
/// length 0 all three weights are 0.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn fn_00cd7530(e: &mut Engine, p: Ptr, a: Ptr, b: Ptr, c: Ptr, out: Ptr) {
    let point = load(e, p);
    let va = load(e, a);
    let vb = load(e, b);
    let vc = load(e, c);
    let e1 = sub(va, vb);
    let e2 = sub(vc, vb);
    let n1 = dot3(e1, e1);
    let n2 = dot3(e2, e2);
    // `MULPS` with `e2` as the destination.
    let d = dot3(e2, e1);
    // The x87 stack code with the x87 choice of NaN; `rounded` is a store to a scratch
    // float, `wide` the load of one.
    let (fsub, fmul, fdiv) = (x87_sub, x87_mul, x87_div);
    let rounded = |value: f64| f32::from_bits(x87_store32(value));
    let wide = |value: f32| x87_load32(value.to_bits());
    let n1n2 = rounded(fmul(wide(n1), wide(n2)));
    let d2 = rounded(fmul(wide(d), wide(d)));
    let det = rounded(fsub(wide(n1n2), wide(d2)));
    let out = out.addr();
    if det > 0.0 {
        let inv = rounded(fdiv(1.0, wide(det)));
        let from_point = sub(vb, point);
        let p1 = dot3(from_point, e1);
        let p2 = dot3(from_point, e2);
        let (d, n1, n2) = (wide(d), wide(n1), wide(n2));
        let x = rounded(fsub(fmul(wide(p2), d), fmul(n2, wide(p1))));
        let y = rounded(fsub(fmul(wide(p1), d), fmul(n1, wide(p2))));
        let (x_d, y_d, inv_d) = (wide(x), wide(y), wide(inv));
        e.mem.set_u32(out, x87_store32(fmul(x_d, inv_d)));
        let rest = fsub(fsub(wide(det), x_d), y_d);
        e.mem.set_u32(out + 4, x87_store32(fmul(rest, inv_d)));
        e.mem.set_u32(out + 8, x87_store32(fmul(y_d, inv_d)));
        return;
    }
    let e3 = sub(va, vc);
    let n3 = dot3(e3, e3);
    // The fraction of the edge `from` + `edge`, `length` long, that `p` projects on (`FDIVR`
    // of the scratch float by the loaded length).
    let fraction = |from: Vec4, edge: Vec4, length: f32| -> f32 {
        let along = dot3(sub(point, from), edge);
        rounded(fdiv(wide(along), wide(length)))
    };
    let one_minus = |value: f32| f32::from_bits(x87_store32(fsub(1.0, wide(value))));
    if !(n1 >= n2) {
        if !(n3 >= n2) {
            let r = fraction(vb, e2, n2);
            e.mem.set_f32(out, 0.0);
            e.mem.set_f32(out + 8, r);
            e.mem.set_f32(out + 4, one_minus(r));
        } else {
            let q = fraction(vc, e3, n3);
            e.mem.set_f32(out, q);
            e.mem.set_f32(out + 4, 0.0);
            e.mem.set_f32(out + 8, one_minus(q));
        }
    } else if !(n3 >= n1) {
        let r = fraction(vb, e1, n1);
        e.mem.set_f32(out, r);
        e.mem.set_f32(out + 4, one_minus(r));
        e.mem.set_f32(out + 8, 0.0);
    } else {
        e.mem.set_f32(out + 4, 0.0);
        if 0.0 < n3 {
            let q = fraction(vc, e3, n3);
            e.mem.set_f32(out, q);
            e.mem.set_f32(out + 8, one_minus(q));
        } else {
            e.mem.set_f32(out, 0.0);
            e.mem.set_f32(out + 8, 0.0);
        }
    }
}

/// What an x87 `FSTP dword` followed by an `FLD dword` of the same slot leaves on the stack:
/// the value rounded to a float, a signalling NaN quieted.
fn through_memory(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

// Translated from 00cd7160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Triangle factors, cdecl `(triangle, out)`. `triangle` holds the vertices `a`, `b`, `c` as
/// three 16-byte vectors; with `u = a - b` and `v = c - b` (SSE) the code forms `n1 = |u|^2`,
/// `n2 = |v|^2` and `d = u . v` (each summed `z + (y + x)`) and the squared length of the
/// cross product `v x u`. On the x87 stack, with every `FSTP`/`FLD` of the scratch slot
/// rounding to a float (and quieting a signalling NaN): `det = (n1 * n2) - (d * d)` (the two
/// products rounded to floats first), `inv = 1 / det`, and `out[0] = n1 * inv`, `out[1] = n2 *
/// inv`, `out[2] = d * inv`, then `out[3] = 1 / sqrt(squared cross length)` (`SQRTSS`, then
/// `FDIV` of the float result into the remaining `1.0`). The lane-3 products
/// (`w * w - w * w`) are formed like the others.
pub fn fn_00cd7160(e: &mut Engine, triangle: Ptr, out: Ptr) {
    let a = load(e, triangle);
    let b = load(e, Ptr::new(triangle.addr() + 0x10));
    let c = load(e, Ptr::new(triangle.addr() + 0x20));
    let u = sub(a, b);
    let v = sub(c, b);
    let n1 = sum_of_three(mulps(u, u));
    let n2 = sum_of_three(mulps(v, v));
    let d = sum_of_three(mulps(v, u));
    // The x87 loads of the three sums.
    let n1_loaded = x87_load32(n1.to_bits());
    let n2_loaded = x87_load32(n2.to_bits());
    let d_loaded = x87_load32(d.to_bits());
    let product = through_memory(x87_mul(n1_loaded, n2_loaded));
    // The cross product `v x u` in SSE.
    let first = mulps(shufps(v, v, 0xd2), shufps(u, u, 0xc9));
    let second = mulps(shufps(v, v, 0xc9), shufps(u, u, 0xd2));
    let cross = subps(first, second);
    let square = through_memory(x87_mul(d_loaded, d_loaded));
    let det = through_memory(x87_sub(product, square));
    let inverse = through_memory(x87_div(1.0, det));
    let out = out.addr();
    e.mem.set_u32(out, x87_store32(x87_mul(n1_loaded, inverse)));
    e.mem
        .set_u32(out + 4, x87_store32(x87_mul(n2_loaded, inverse)));
    e.mem
        .set_u32(out + 8, x87_store32(x87_mul(d_loaded, inverse)));
    let length = crate::sse::sqrt(sum_of_three(mulps(cross, cross)));
    let reciprocal = x87_div(1.0, x87_m32(length.to_bits()));
    e.mem.set_u32(out + 0xc, x87_store32(reciprocal));
}

/// Copies the float at `from` to `to` through the x87 stack (`FLD`/`FSTP`): a signalling NaN
/// comes out quiet.
fn copy_through_x87(e: &mut Engine, from: u32, to: u32) {
    let bits = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, bits);
}

fn store(e: &mut Engine, at: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_u32(at + 4 * lane as u32, value.to_bits());
    }
}

/// `(a * shuffled-0xc9 b) - (shuffled-0xd2 a * shuffled-0xc9 ...)`: the cross product of
/// `x` with `normal` as the code forms it: `x.yzx * normal.zxy - x.zxy * normal.yzx`.
fn cross_with(x: Vec4, normal: Vec4) -> Vec4 {
    subps(
        mulps(shufps(x, x, 0xc9), shufps(normal, normal, 0xd2)),
        mulps(shufps(x, x, 0xd2), shufps(normal, normal, 0xc9)),
    )
}

// Translated from 00cd73e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Triangle frame, cdecl `(triangle, factors, matrix, normal)`. `triangle` holds the vertices
/// `a`, `b`, `c` (three 16-byte vectors); `factors` the five floats `00cd72a0` writes
/// (`q0` to `q4`). With `u = c - b`, `v = a - c` and `w = b - a`: `normal = (v.zxy * u.yzx -
/// v.yzx * u.zxy) * q3` (stored at `normal`, four lanes) and the three rows of `matrix` are
/// `row0 = (q0 * u) x normal`, `row1 = (q1 * v) x normal`, `row2 = (q2 * w) x normal`, each
/// `p.yzx * normal.zxy - p.zxy * normal.yzx`. The 3x3 part is then transposed with float
/// copies through the x87 stack (lanes 1 and 4, 2 and 8, 6 and 9), the fourth column of the
/// three rows is set to the normal's first three lanes (through the x87 stack), the last row
/// is zeroed and its first lane set to `-q0 * q4`.
pub fn fn_00cd73e0(e: &mut Engine, triangle: Ptr, factors: Ptr, matrix: Ptr, normal: Ptr) {
    let a = load(e, triangle);
    let b = load(e, Ptr::new(triangle.addr() + 0x10));
    let c = load(e, Ptr::new(triangle.addr() + 0x20));
    let u = sub(c, b);
    let v = sub(a, c);
    let w = sub(b, a);
    let q: [f32; 5] = std::array::from_fn(|i| e.mem.f32(factors.addr() + 4 * i as u32));
    let scale = splat(q[3]);
    let first = mulps(shufps(v, v, 0xd2), shufps(u, u, 0xc9));
    let second = mulps(shufps(v, v, 0xc9), shufps(u, u, 0xd2));
    let n = mulps(subps(first, second), scale);
    store(e, normal.addr(), n);
    let x0 = mulps(splat(q[0]), u);
    let x1 = mulps(splat(q[1]), v);
    let x2 = mulps(splat(q[2]), w);
    let matrix = matrix.addr();
    store(e, matrix, cross_with(x0, n));
    store(e, matrix + 0x10, cross_with(x1, n));
    store(e, matrix + 0x20, cross_with(x2, n));
    // Transpose of the 3x3 part, each word moved through the x87 stack.
    let scratch = x87_quiet_bits(e.mem.u32(matrix + 0x10));
    copy_through_x87(e, matrix + 4, matrix + 0x10);
    e.mem.set_u32(matrix + 4, x87_quiet_bits(scratch));
    let scratch = x87_quiet_bits(e.mem.u32(matrix + 0x20));
    copy_through_x87(e, matrix + 8, matrix + 0x20);
    e.mem.set_u32(matrix + 8, x87_quiet_bits(scratch));
    let scratch = x87_quiet_bits(e.mem.u32(matrix + 0x24));
    copy_through_x87(e, matrix + 0x18, matrix + 0x24);
    e.mem.set_u32(matrix + 0x18, x87_quiet_bits(scratch));
    copy_through_x87(e, normal.addr(), matrix + 0xc);
    copy_through_x87(e, normal.addr() + 4, matrix + 0x1c);
    copy_through_x87(e, normal.addr() + 8, matrix + 0x2c);
    store(e, matrix + 0x30, [0.0; 4]);
    let last = x87_load32(e.mem.u32(factors.addr() + 0x10));
    let first = x87_load32(e.mem.u32(factors.addr()));
    e.mem
        .set_u32(matrix + 0x30, x87_store32(x87_mul(last, -first)));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cd7160, fn_00cd7160(Ptr, Ptr)),
        entry!(0x00cd72a0, fn_00cd72a0(Ptr, Ptr)),
        entry!(0x00cd73e0, fn_00cd73e0(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00cd7530, fn_00cd7530(Ptr, Ptr, Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vector(e: &mut Engine, v: [f32; 3]) -> Ptr {
        let at = e.mem.alloc(0x40);
        for (i, c) in v.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, c);
        }
        e.mem.set_f32(at + 12, 1.0);
        Ptr::new(at)
    }

    fn close(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() <= 1e-4 * expected.abs().max(1.0)
    }

    #[test]
    fn edge_lengths_and_normal_length_are_inverted() {
        let mut e = Engine::new();
        let triangle = e.mem.alloc(0x30);
        for (i, v) in [[0.0, 0.0, 0.0], [3.0, 0.0, 0.0], [0.0, 4.0, 0.0]]
            .into_iter()
            .enumerate()
        {
            for (j, c) in v.into_iter().enumerate() {
                e.mem.set_f32(triangle + 0x10 * i as u32 + 4 * j as u32, c);
            }
        }
        let out = e.mem.alloc(0x20);
        e.call(0x00cd72a0, &args![triangle, out]);
        // |c - b| = 5, |a - c| = 4, |b - a| = 3, |normal| = 12.
        assert!(close(e.mem.f32(out), 0.2));
        assert!(close(e.mem.f32(out + 4), 0.25));
        assert!(close(e.mem.f32(out + 8), 1.0 / 3.0));
        assert!(close(e.mem.f32(out + 12), 1.0 / 12.0));
        assert!(close(e.mem.f32(out + 16), 12.0));
    }

    fn weights(points: [[f32; 3]; 4]) -> [f32; 3] {
        let mut e = Engine::new();
        let [p, a, b, c] = points.map(|v| vector(&mut e, v));
        let out = e.mem.alloc(0x10);
        for i in 0..3 {
            e.mem.set_f32(out + 4 * i, 9.0);
        }
        e.call(0x00cd7530, &args![p, a, b, c, out]);
        [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)]
    }

    const TRIANGLE: [[f32; 3]; 3] = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]];

    #[test]
    fn a_nondegenerate_triangle_gives_the_plane_coordinates() {
        let [a, b, c] = TRIANGLE;
        assert_eq!(weights([a, a, b, c]), [1.0, 0.0, 0.0]);
        assert_eq!(weights([c, a, b, c]), [0.0, 0.0, 1.0]);
        assert_eq!(weights([b, a, b, c]), [0.0, 1.0, 0.0]);
    }

    #[test]
    fn collinear_points_project_on_the_longest_edge() {
        // n1 == n2: the edge from c to a (n3 is longest).
        let w = weights([
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
        ]);
        assert_eq!(w, [0.5, 0.0, 0.5]);
        // n1 < n2 and n3 >= n2: the edge a-c again.
        let w = weights([
            [1.5, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [3.0, 0.0, 0.0],
        ]);
        assert_eq!(w, [0.5, 0.0, 0.5]);
    }

    #[test]
    fn collinear_points_can_project_on_either_edge_at_b() {
        // n3 < n2: the edge b-c.
        let w = weights([
            [-1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [-3.0, 0.0, 0.0],
        ]);
        assert_eq!(w, [0.0, 0.5, 0.5]);
        // n1 > n2 and n3 < n1: the edge a-b.
        let w = weights([
            [1.5, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [3.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
        ]);
        assert_eq!(w, [0.5, 0.5, 0.0]);
    }

    #[test]
    fn a_point_triangle_has_zero_weights() {
        let origin = [0.0, 0.0, 0.0];
        assert_eq!(weights([origin; 4]), [0.0, 0.0, 0.0]);
    }
}

#[cfg(test)]
mod tests_00cd7160 {
    use super::*;

    /// A triangle of three 16-byte vectors (the fourth lane given too).
    fn triangle(e: &mut Engine, vertices: [[f32; 4]; 3]) -> u32 {
        let at = e.mem.alloc(0x30);
        for (i, vertex) in vertices.into_iter().enumerate() {
            for (lane, value) in vertex.into_iter().enumerate() {
                e.mem.set_f32(at + 0x10 * i as u32 + 4 * lane as u32, value);
            }
        }
        at
    }

    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// Edge `u = (1, 0, 0)` and `v = (0, 2, 0)`: `n1 = 1`, `n2 = 4`, `d = 0`, `det = 4`; the
    /// cross product has length 2.
    #[test]
    fn right_angle_triangle_factors() {
        let mut e = Engine::new();
        let t = triangle(
            &mut e,
            [[1.0, 0.0, 0.0, 1.0], [0.0; 4], [0.0, 2.0, 0.0, 1.0]],
        );
        let out = e.mem.alloc(0x10);
        e.call(0x00cd7160, &args![t, out]);
        assert_eq!(floats(&e, out, 4), vec![0.25, 1.0, 0.0, 0.5]);
    }

    /// `u = (1, 2, 3)`, `v = (4, 5, 6)`: `n1 = 14`, `n2 = 77`, `d = 32`, `det = 54`, the cross
    /// product `(-3, 6, -3)`; the results are rounded to floats where the code stores them.
    #[test]
    fn general_triangle_factors_round_at_each_float_store() {
        let mut e = Engine::new();
        let t = triangle(
            &mut e,
            [[1.0, 2.0, 3.0, 0.0], [0.0; 4], [4.0, 5.0, 6.0, 0.0]],
        );
        let out = e.mem.alloc(0x10);
        e.call(0x00cd7160, &args![t, out]);
        let inverse = f64::from((1.0f64 / 54.0) as f32);
        let length = 54.0f32.sqrt();
        assert_eq!(
            floats(&e, out, 4),
            vec![
                (14.0 * inverse) as f32,
                (77.0 * inverse) as f32,
                (32.0 * inverse) as f32,
                (1.0 / f64::from(length)) as f32
            ]
        );
    }

    /// Equal edges make the determinant 0: the inverse and everything scaled by it is
    /// infinite, and so is the reciprocal of the zero length.
    #[test]
    fn degenerate_triangle_gives_infinities() {
        let mut e = Engine::new();
        let t = triangle(
            &mut e,
            [[1.0, 0.0, 0.0, 0.0], [0.0; 4], [1.0, 0.0, 0.0, 0.0]],
        );
        let out = e.mem.alloc(0x10);
        e.call(0x00cd7160, &args![t, out]);
        assert_eq!(floats(&e, out, 4), vec![f32::INFINITY; 4]);
    }

    /// The cross product of `x` with `n`, lane by lane, as the code forms it.
    fn cross(x: [f32; 4], n: [f32; 4]) -> [f32; 4] {
        [
            x[1] * n[2] - x[2] * n[1],
            x[2] * n[0] - x[0] * n[2],
            x[0] * n[1] - x[1] * n[0],
            x[3] * n[3] - x[3] * n[3],
        ]
    }

    /// Whole-number inputs make every product exact, so the frame can be compared with an
    /// independent computation of the formulas: the normal, the rows, the transposition, the
    /// copied normal in the fourth column and the translation lane.
    #[test]
    fn frame_matrix_and_normal_follow_the_formulas() {
        let mut e = Engine::new();
        let a = [1.0, 2.0, 3.0, 4.0];
        let b = [2.0, 0.0, 1.0, 0.0];
        let c = [0.0, 1.0, 5.0, 2.0];
        let t = triangle(&mut e, [a, b, c]);
        let q = [1.0f32, 2.0, 3.0, 4.0, 5.0];
        let factors = e.mem.alloc(0x20);
        for (i, value) in q.into_iter().enumerate() {
            e.mem.set_f32(factors + 4 * i as u32, value);
        }
        let matrix = e.mem.alloc(0x40);
        let normal = e.mem.alloc(0x10);
        e.call(0x00cd73e0, &args![t, factors, matrix, normal]);
        let sub4 = |x: [f32; 4], y: [f32; 4]| [x[0] - y[0], x[1] - y[1], x[2] - y[2], x[3] - y[3]];
        let (u, v, w) = (sub4(c, b), sub4(a, c), sub4(b, a));
        let n = [
            (v[2] * u[1] - v[1] * u[2]) * q[3],
            (v[0] * u[2] - v[2] * u[0]) * q[3],
            (v[1] * u[0] - v[0] * u[1]) * q[3],
            (v[3] * u[3] - v[3] * u[3]) * q[3],
        ];
        assert_eq!(floats(&e, normal, 4), n.to_vec());
        let scaled = |k: f32, x: [f32; 4]| [k * x[0], k * x[1], k * x[2], k * x[3]];
        let mut rows = [
            cross(scaled(q[0], u), n),
            cross(scaled(q[1], v), n),
            cross(scaled(q[2], w), n),
            [0.0; 4],
        ];
        // Transpose of the 3x3 part, the normal in the fourth column, `-q0 * q4` in the last row.
        let (r01, r02, r12) = (rows[0][1], rows[0][2], rows[1][2]);
        rows[0][1] = rows[1][0];
        rows[1][0] = r01;
        rows[0][2] = rows[2][0];
        rows[2][0] = r02;
        rows[1][2] = rows[2][1];
        rows[2][1] = r12;
        rows[0][3] = n[0];
        rows[1][3] = n[1];
        rows[2][3] = n[2];
        rows[3][0] = -q[0] * q[4];
        let got: Vec<f32> = floats(&e, matrix, 16);
        let want: Vec<f32> = rows.iter().flatten().copied().collect();
        assert_eq!(got, want);
    }

    /// A signalling NaN as the first factor is quieted by the x87 load, negated by `FCHS` and
    /// multiplied: the last row lane is a negative quiet NaN.
    #[test]
    fn a_signalling_nan_factor_gives_a_negative_quiet_nan() {
        let mut e = Engine::new();
        let t = triangle(&mut e, [[0.0; 4], [0.0; 4], [0.0; 4]]);
        let factors = e.mem.alloc(0x20);
        e.mem.set_f32(factors + 0xc, 1.0);
        let matrix = e.mem.alloc(0x40);
        let normal = e.mem.alloc(0x10);
        e.call(0x00cd73e0, &args![t, factors, matrix, normal]);
        // With an all-zero triangle every row and the normal are zero; -q0 * q4 is -0 * 0.
        assert_eq!(e.mem.u32(matrix + 0x30), 0x8000_0000);
        // The first factor is a signalling NaN: the last row's lane becomes a quiet NaN.
        e.mem.set_u32(factors, 0x7f80_0001);
        e.call(0x00cd73e0, &args![t, factors, matrix, normal]);
        assert_eq!(e.mem.u32(matrix + 0x30) & 0xffc0_0000, 0xffc0_0000);
    }
}

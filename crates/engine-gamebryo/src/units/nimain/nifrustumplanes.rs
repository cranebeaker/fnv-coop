//! `NiMain/nifrustumplanes.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPlane::NiPlane_ov3` (`00a69990`, `thiscall (plane, &normal, &point)`,
/// `RET 8`, answers the plane).
const PLANE_FROM_NORMAL_AND_POINT: u32 = 0x00a6_9990;
/// `1.0` (`double`, `01012070`).
const ONE: u32 = 0x0101_2070;
/// The CRT square root `_CIsqrt` (`00ec6040`, argument and result in ST0).
const CRT_SQRT: u32 = 0x00ec_6040;
/// `NiFrustum` (Xbox PDB): `m_fLeft` +0, `m_fRight` +4, `m_fTop` +8,
/// `m_fBottom` +0xC, `m_fNear` +0x10, `m_fFar` +0x14, `m_bOrtho` +0x18.
const LEFT: u32 = 0x0;
const RIGHT: u32 = 0x4;
const TOP: u32 = 0x8;
const BOTTOM: u32 = 0xc;
const NEAR: u32 = 0x10;
const FAR: u32 = 0x14;
const ORTHO: u32 = 0x18;
/// `NiTransform` (Xbox PDB): `m_Rotate` (nine floats, row by row) +0,
/// `m_Translate` +0x24.
const TRANSLATE: u32 = 0x24;

type Vector = [u32; 3];

fn load(bits: u32) -> f64 {
    x87_load32(bits)
}

/// One plane (`NiPlane_ov3` from `normal` and `point`, all as the float
/// words the exe keeps on its stack) stored at `this + offset`.
fn set_plane(e: &mut Engine, this: Ptr, offset: u32, normal: Vector, point: Vector) {
    e.with_stack(0x30, |e, frame| {
        let (normal_at, point_at, result) =
            (frame.addr(), frame.addr() + 0x10, frame.addr() + 0x20);
        for i in 0..3u32 {
            e.mem.set_u32(normal_at + 4 * i, normal[i as usize]);
            e.mem.set_u32(point_at + 4 * i, point[i as usize]);
        }
        let plane = e
            .call(
                PLANE_FROM_NORMAL_AND_POINT,
                &args![result, normal_at, point_at],
            )
            .u32();
        for i in 0..4u32 {
            let value = e.mem.u32(plane + 4 * i);
            e.mem
                .set_u32(this.addr().wrapping_add(offset + 4 * i), value);
        }
    });
}

/// `column * factor`, its three products in the exe's operand orders
/// (`first` and `second` for the x and y products: true when the factor is
/// the instruction's first operand) and the z product always `factor *
/// z` (an `FMUL` with the float in memory, or the stack copy, as `third`
/// says), each rounded to a float.
fn scaled(column: Vector, factor: f64, factor_first: [bool; 2], z_in_memory: bool) -> Vector {
    let product = |value: f64, first: bool| {
        if first {
            x87_mul(factor, value)
        } else {
            x87_mul(value, factor)
        }
    };
    let z = if z_in_memory {
        x87_m32(column[2])
    } else {
        load(column[2])
    };
    [
        x87_store32(product(load(column[0]), factor_first[0])),
        x87_store32(product(load(column[1]), factor_first[1])),
        x87_store32(x87_mul(factor, z)),
    ]
}

/// `first + second` component by component (`FLD first; FADD second`).
fn sum(first: Vector, second: Vector) -> Vector {
    [0, 1, 2].map(|i| x87_store32(x87_add(load(first[i]), x87_m32(second[i]))))
}

fn negated(column: Vector) -> Vector {
    column.map(|bits| x87_store32(-load(bits)))
}

/// `1 / sqrt(x * x + 1)` as the exe computes it for a perspective plane,
/// each step rounded to a float.
fn inverse_length(e: &mut Engine, x: f64) -> f64 {
    let one = f64::from_bits(e.global::<u64>(ONE));
    let square = x87_store32(x87_mul(x, x));
    let square = x87_store32(x87_add(load(square), one));
    let root = e.call(CRT_SQRT, &args![load(square)]).f64();
    let root = load(x87_store32(root));
    load(x87_store32(x87_div(1.0, root)))
}

// Translated from 00a74e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFrustumPlanes::Set` (Xbox PDB): the six planes of `frustum` seen
/// through `transform` (rotation columns `c0`, `c1`, `c2`, translation `t`),
/// each made by `NiPlane_ov3` from a normal and a point: near `(c0, t + near
/// c0)`, far `(-c0, t + far c0)`; for an orthographic frustum left `(c2, t +
/// left c2)`, right `(-c2, t + right c2)`, top `(-c1, t + top c1)`, bottom
/// `(c1, t + bottom c1)`; otherwise, with `i(x) = 1 / sqrt(x^2 + 1)`, the
/// planes through `t` with normals `-left i c0 + i c2`, `right i c0 - i c2`,
/// `top i c0 - i c1` and `-bottom i c0 + i c1`. Every intermediate is
/// rounded to a float where the exe stores it, and each product and sum
/// keeps the exe's operand order.
pub fn ni_frustum_planes_set(e: &mut Engine, this: Ptr, frustum: Ptr, transform: Ptr) {
    let t = transform.addr();
    let f = frustum.addr();
    let quiet = |e: &Engine, at: u32| x87_quiet_bits(e.mem.u32(at));
    let c0: Vector = [quiet(e, t), quiet(e, t + 0xc), quiet(e, t + 0x18)];
    let c1: Vector = [quiet(e, t + 4), quiet(e, t + 0x10), quiet(e, t + 0x1c)];
    let c2: Vector = [quiet(e, t + 8), quiet(e, t + 0x14), quiet(e, t + 0x20)];
    let translate: Vector = [0u32, 1, 2].map(|i| e.mem.u32(t + TRANSLATE + 4 * i));
    let value = |e: &Engine, offset: u32| load(e.mem.u32(f + offset));

    // Near: the translation plus near times c0 (`FLD t; FADD product`).
    let near = value(e, NEAR);
    let products = scaled(c0, near, [false, false], true);
    let point = [0, 1, 2].map(|i| x87_store32(x87_add(load(translate[i]), x87_m32(products[i]))));
    set_plane(e, this, 0, c0, point);

    // Far: the product plus the translation.
    let far = value(e, FAR);
    let products = scaled(c0, far, [true, false], false);
    set_plane(e, this, 0x10, negated(c0), sum(products, translate));

    if e.mem.u8(f + ORTHO) != 0 {
        let left = value(e, LEFT);
        let products = scaled(c2, left, [false, false], true);
        set_plane(e, this, 0x20, c2, sum(products, translate));
        let right = value(e, RIGHT);
        let products = scaled(c2, right, [true, false], false);
        set_plane(e, this, 0x30, negated(c2), sum(products, translate));
        let top = value(e, TOP);
        let products = scaled(c1, top, [true, false], false);
        set_plane(e, this, 0x40, negated(c1), sum(products, translate));
        let bottom = value(e, BOTTOM);
        let products = scaled(c1, bottom, [false, false], true);
        set_plane(e, this, 0x50, c1, sum(products, translate));
        return;
    }

    // Left: u = -left * i, normal = u c0 + i c2.
    let left = value(e, LEFT);
    let inverse = inverse_length(e, left);
    let u = load(x87_store32(x87_mul(-left, inverse)));
    let v = scaled(c2, inverse, [false, false], true);
    let w = scaled(c0, u, [false, false], true);
    set_plane(e, this, 0x20, sum(w, v), translate);

    // Right: k = right * i, m = -i, normal = k c0 + m c2.
    let right = value(e, RIGHT);
    let inverse = inverse_length(e, right);
    let k = load(x87_store32(x87_mul(right, inverse)));
    let m = load(x87_store32(-inverse));
    let v = scaled(c2, m, [false, false], true);
    let w = scaled(c0, k, [false, false], true);
    set_plane(e, this, 0x30, sum(w, v), translate);

    // Top: k = i * top (the float in memory), m = -i, normal = k c0 + m c1.
    let top_bits = e.mem.u32(f + TOP);
    let inverse = inverse_length(e, load(top_bits));
    let k = load(x87_store32(x87_mul(inverse, x87_m32(top_bits))));
    let m = load(x87_store32(-inverse));
    let v = scaled(c1, m, [false, false], true);
    let w = scaled(c0, k, [false, false], true);
    set_plane(e, this, 0x40, sum(w, v), translate);

    // Bottom: k = -bottom * i, normal = k c0 + i c1.
    let bottom = value(e, BOTTOM);
    let inverse = inverse_length(e, bottom);
    let k = load(x87_store32(x87_mul(-bottom, inverse)));
    let v = scaled(c1, inverse, [false, false], true);
    let w = scaled(c0, k, [false, false], true);
    set_plane(e, this, 0x50, sum(w, v), translate);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a74e10, ni_frustum_planes_set(Ptr, Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A plane double: the normal's three words and the dot product of the
    /// normal and the point as the constant.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.set_global(ONE, 1.0f64);
        e.register(PLANE_FROM_NORMAL_AND_POINT, |e, a| {
            let mut dot = 0.0f32;
            for i in 0..3 {
                let n = e.mem.f32(a[1] + 4 * i);
                e.mem.set_f32(a[0] + 4 * i, n);
                dot += n * e.mem.f32(a[2] + 4 * i);
            }
            e.mem.set_f32(a[0] + 12, dot);
            a[0].into_ret()
        });
        e
    }

    fn frustum(e: &mut Engine, values: [f32; 6], ortho: bool) -> u32 {
        let f = e.mem.alloc(0x1c);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(f + 4 * i as u32, *v);
        }
        e.mem.set_u8(f + 0x18, u8::from(ortho));
        f
    }

    /// The identity rotation, translation (1, 2, 3).
    fn transform(e: &mut Engine) -> u32 {
        let t = e.mem.alloc(0x34);
        for (i, v) in [
            1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 1.0,
        ]
        .iter()
        .enumerate()
        {
            e.mem.set_f32(t + 4 * i as u32, *v);
        }
        t
    }

    fn planes(e: &Engine, this: u32) -> Vec<[f32; 4]> {
        (0..6)
            .map(|p| [0, 1, 2, 3].map(|i| e.mem.f32(this + 0x10 * p + 4 * i)))
            .collect()
    }

    #[test]
    fn orthographic_planes() {
        let mut e = engine();
        let f = frustum(&mut e, [-1.0, 1.0, 1.0, -1.0, 1.0, 10.0], true);
        let t = transform(&mut e);
        let this = e.mem.alloc(0x60);
        e.call(0x00a74e10, &args![this, f, t]);
        assert_eq!(
            planes(&e, this),
            vec![
                [1.0, 0.0, 0.0, 2.0],
                [-1.0, -0.0, -0.0, -11.0],
                [0.0, 0.0, 1.0, 2.0],
                [-0.0, -0.0, -1.0, -4.0],
                [-0.0, -1.0, -0.0, -3.0],
                [0.0, 1.0, 0.0, 1.0],
            ]
        );
    }

    #[test]
    fn perspective_planes() {
        let mut e = engine();
        let f = frustum(&mut e, [-1.0, 1.0, 1.0, -1.0, 1.0, 10.0], false);
        let t = transform(&mut e);
        let this = e.mem.alloc(0x60);
        e.call(0x00a74e10, &args![this, f, t]);
        let p = planes(&e, this);
        let i = (0.5f64).sqrt() as f32;
        let close = |a: [f32; 4], b: [f32; 3]| (0..3).all(|k| (a[k] - b[k]).abs() < 1e-6);
        assert!(close(p[2], [i, 0.0, i]), "{:?}", p[2]);
        assert!(close(p[3], [i, 0.0, -i]), "{:?}", p[3]);
        assert!(close(p[4], [i, -i, 0.0]), "{:?}", p[4]);
        assert!(close(p[5], [i, i, 0.0]), "{:?}", p[5]);
        assert_eq!(p[0], [1.0, 0.0, 0.0, 2.0]);
    }
}

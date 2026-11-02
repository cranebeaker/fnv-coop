//! `Havok SDK/run_00db47a0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two unnamed SSE functions that take some of their inputs in registers (the register values
//! come first in the parameter lists, in the order the doc comments give).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::{
    add4, load_vec4, mul4, rotate_c9, rotate_d2, sub4, sum3, Vec4,
};

/// The four-lane vector of ones (`010ec7f0`) and the four-lane sign mask (`010c44d0`).
const ONES: u32 = 0x010e_c7f0;
const SIGN_MASK: u32 = 0x010c_44d0;

/// `splat` of a float the code stored to the stack and loaded with `MOVSS`/`SHUFPS 0`.
fn splat(value: f32) -> Vec4 {
    [value; 4]
}

// Translated from 00db47a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first:
/// `a` (`ECX`, a 16-byte vector) and `b` (`EAX`, two 16-byte vectors `b0` at `+0` and `b1` at
/// `+0x10`); then the cdecl stack words `first` and `second` (objects with a `float` at
/// `+0xa0`), `c` (a 16-byte vector), `out_dot` and `out_sum` (pointers to `float`s).
///
/// With the cross products `u = b0 x a` and `v = b1 x a` (the exe's `SHUFPS` 0xc9/0xd2 form,
/// `rot_c9(b) * rot_d2(a) - rot_d2(b) * rot_c9(a)`), `out_sum` receives
/// `first.f * |u| + second.f * |v|` where `|.|` is the `SQRTSS` of the lanes 0 to 2 summed in
/// the code's order; the products and the sum go through the x87 stack with each step stored
/// as a `float` (the first product is stored before the second is formed). `out_dot` receives
/// `a . c` (lanes 0 to 2 summed in the code's order, through `FLD`/`FSTP`).
pub fn fn_00db47a0(
    e: &mut Engine,
    a: u32,
    b: u32,
    first: u32,
    second: u32,
    c: u32,
    out_dot: u32,
    out_sum: u32,
) {
    let va = load_vec4(e, a);
    let vb0 = load_vec4(e, b);
    let rotated_a = (rotate_c9(va), rotate_d2(va));
    let u = sub4(
        mul4(rotate_c9(vb0), rotated_a.1),
        mul4(rotate_d2(vb0), rotated_a.0),
    );
    let vb1 = load_vec4(e, b + 0x10);
    let first_factor = e.mem.u32(first + 0xa0);
    let u_squared = mul4(u, u);
    let v = sub4(
        mul4(rotate_c9(vb1), rotated_a.1),
        mul4(rotate_d2(vb1), rotated_a.0),
    );
    let v_squared = mul4(v, v);
    let sqrt_u = sum3(u_squared).sqrt();
    // FMUL by the stored square root, FSTP: the first product as a `float`.
    let first_product = x87_store32(x87_mul(x87_load32(first_factor), x87_m32(sqrt_u.to_bits())));
    let sqrt_v = sum3(v_squared).sqrt();
    let second_factor = e.mem.u32(second + 0xa0);
    let second_product = x87_store32(x87_mul(
        x87_load32(second_factor),
        x87_m32(sqrt_v.to_bits()),
    ));
    let sum = x87_store32(x87_add(x87_load32(first_product), x87_m32(second_product)));
    e.mem.set_u32(out_sum, sum);
    let va = load_vec4(e, a);
    let vc = load_vec4(e, c);
    let dot = sum3(mul4(va, vc));
    e.mem.set_u32(out_dot, x87_quiet(dot).to_bits());
}

// Translated from 00db48a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. Inputs are in registers, so the register values come first:
/// `b` (`EAX`, an object), `d` (`EDX`, two 16-byte vectors `d0` at `+0` and `d1` at `+0x10`)
/// and `p` (`ESI`, a 16-byte vector); then the cdecl stack words `a` (an object), `w`, `n`
/// (16-byte vectors) and `t` (a `float`). Answers a `float` in `ST0`.
///
/// For each of the objects `a` and `b` (which hold two 16-byte vectors at `+0x40` and `+0x50`,
/// whose fourth lanes are a time at `+0x4c` and a scale at `+0x5c`) it forms `s = (t - time) * scale` (x87, stored as
/// a `float`) and the position `(1 - s) * v40 + v50 * s` over all four lanes; `da` and `db` are
/// `p` minus the two positions. The result is the sum of lanes 0 to 2 (in the code's order) of
/// `((rot_d2(da) * rot_c9(d0) - rot_c9(da) * rot_d2(d0)) + (-n) - (rot_d2(db) * rot_c9(d1) -
/// rot_c9(db) * rot_d2(d1))) * w`, with `-n` the sign-mask XOR.
pub fn fn_00db48a0(e: &mut Engine, b: u32, d: u32, p: u32, a: u32, w: u32, n: u32, t: f32) -> f32 {
    let time = x87_load32(t.to_bits());
    let ones = load_vec4(e, ONES);
    let a_time = e.mem.u32(a + 0x4c);
    let a_low = load_vec4(e, a + 0x40);
    let vp = load_vec4(e, p);
    let a_scale = e.mem.u32(a + 0x5c);
    let s_a = f32::from_bits(x87_store32(x87_mul(
        x87_sub(time, x87_m32(a_time)),
        x87_m32(a_scale),
    )));
    let splat_a = splat(s_a);
    let b_time = e.mem.u32(b + 0x4c);
    let one_minus_a = sub4(ones, splat_a);
    let a_scaled_low = mul4(one_minus_a, a_low);
    let a_high = load_vec4(e, a + 0x50);
    let b_scale = e.mem.u32(b + 0x5c);
    let a_scaled_high = mul4(a_high, splat_a);
    let position_a = add4(a_scaled_low, a_scaled_high);
    let b_low = load_vec4(e, b + 0x40);
    let s_b = f32::from_bits(x87_store32(x87_mul(
        x87_sub(time, x87_m32(b_time)),
        x87_m32(b_scale),
    )));
    let da = sub4(vp, position_a);
    let splat_b = splat(s_b);
    let b_scaled_low = mul4(sub4(ones, splat_b), b_low);
    let b_high = load_vec4(e, b + 0x50);
    let b_scaled_high = mul4(b_high, splat_b);
    let position_b = add4(b_scaled_low, b_scaled_high);
    let db = sub4(vp, position_b);
    let d0 = load_vec4(e, d);
    let cross_a = sub4(
        mul4(rotate_d2(da), rotate_c9(d0)),
        mul4(rotate_c9(da), rotate_d2(d0)),
    );
    let d1 = load_vec4(e, d + 0x10);
    let vn = load_vec4(e, n);
    let mask = [0, 1, 2, 3].map(|i| e.mem.u32(SIGN_MASK + 4 * i));
    let negated_n: Vec4 = [0, 1, 2, 3].map(|i| f32::from_bits(vn[i].to_bits() ^ mask[i]));
    let first = add4(cross_a, negated_n);
    let cross_b = sub4(
        mul4(rotate_d2(db), rotate_c9(d1)),
        mul4(rotate_c9(db), rotate_d2(d1)),
    );
    let vw = load_vec4(e, w);
    let weighted = mul4(sub4(first, cross_b), vw);
    x87_quiet(sum3(weighted))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00db47a0, fn_00db47a0(u32, u32, u32, u32, u32, u32, u32)),
        entry!(0x00db48a0, fn_00db48a0(u32, u32, u32, u32, u32, u32, f32) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkgeometryprocessing::store_vec4;

    fn vector(e: &mut Engine, lanes: Vec4) -> u32 {
        let at = e.mem.alloc(16);
        store_vec4(e, at, lanes);
        at
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(ONES, 0x10);
        e.map(SIGN_MASK, 0x10);
        for lane in 0..4 {
            e.mem.set_f32(ONES + 4 * lane, 1.0);
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e
    }

    fn object_with_factor(e: &mut Engine, factor: f32) -> u32 {
        let at = e.mem.alloc(0xb0);
        e.mem.set_f32(at + 0xa0, factor);
        at
    }

    #[test]
    fn the_sum_adds_the_two_scaled_lengths_and_the_dot_is_stored() {
        let mut e = engine();
        let a = vector(&mut e, [3.0, 0.0, 0.0, 0.0]);
        let b = e.mem.alloc(0x20);
        store_vec4(&mut e, b, [0.0, 1.0, 0.0, 0.0]);
        store_vec4(&mut e, b + 0x10, [0.0, 0.0, 1.0, 0.0]);
        let first = object_with_factor(&mut e, 2.0);
        let second = object_with_factor(&mut e, 0.5);
        let c = vector(&mut e, [0.5, 2.0, 3.0, 9.0]);
        let out_dot = e.mem.alloc(4);
        let out_sum = e.mem.alloc(4);
        e.call(
            0x00db_47a0,
            &args![a, b, first, second, c, out_dot, out_sum],
        );
        // |b0 x a| = |b1 x a| = 3.
        assert_eq!(e.mem.f32(out_sum), 2.0 * 3.0 + 0.5 * 3.0);
        assert_eq!(e.mem.f32(out_dot), 1.5);
    }

    #[test]
    fn a_quiet_nan_dot_keeps_its_bits() {
        let mut e = engine();
        let a = vector(&mut e, [f32::from_bits(0x7fc0_1234), 0.0, 0.0, 0.0]);
        let b = e.mem.alloc(0x20);
        let first = object_with_factor(&mut e, 1.0);
        let second = object_with_factor(&mut e, 1.0);
        let c = vector(&mut e, [1.0, 1.0, 1.0, 1.0]);
        let out_dot = e.mem.alloc(4);
        let out_sum = e.mem.alloc(4);
        e.call(
            0x00db_47a0,
            &args![a, b, first, second, c, out_dot, out_sum],
        );
        assert!(e.mem.f32(out_dot).is_nan());
        assert!(e.mem.f32(out_sum).is_nan());
    }

    /// An object holding two position vectors at `+0x40` and `+0x50`; its time (`+0x4c`) and
    /// scale (`+0x5c`) are the fourth lanes of those vectors.
    fn mover(e: &mut Engine, time: f32, scale: f32, low: [f32; 3], high: [f32; 3]) -> u32 {
        let at = e.mem.alloc(0x70);
        store_vec4(e, at + 0x40, [low[0], low[1], low[2], time]);
        store_vec4(e, at + 0x50, [high[0], high[1], high[2], scale]);
        at
    }

    #[test]
    fn the_triple_product_of_the_two_offsets_is_weighted_and_summed() {
        let mut e = engine();
        // s = (0.5 - time) * scale = 1 for both objects, so each position is its `+0x50`
        // vector: da = (1, 2, 3, 0) - (0, 1, 1, 0) and db = (1, 2, 3, 0) - (1, 1, 0, 0).
        let a = mover(&mut e, 0.0, 2.0, [9.0; 3], [0.0, 1.0, 1.0]);
        let b = mover(&mut e, 0.25, 4.0, [7.0; 3], [1.0, 1.0, 0.0]);
        let p = vector(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let d = e.mem.alloc(0x20);
        store_vec4(&mut e, d, [1.0, 0.0, 0.0, 0.0]);
        store_vec4(&mut e, d + 0x10, [0.0, 0.0, 1.0, 0.0]);
        let w = vector(&mut e, [1.0, 1.0, 5.0, 0.0]);
        let n = vector(&mut e, [2.0, 0.0, 0.0, 0.0]);
        // (da, db) = ((1, 1, 2, 0), (0, 1, 3, 0)): the first cross is (0, -2, 1, 0), minus n is
        // (-2, -2, 1, 0), the second cross is (-1, 0, 0, 0): (-1, -2, 1, 0) * w summed is 2.
        let r = e.call(0x00db_48a0, &args![b, d, p, a, w, n, 0.5f32]).f32();
        assert_eq!(r, 2.0);
    }
}

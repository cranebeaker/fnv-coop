//! `Havok SDK/run_00d4f3d0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::hkpconvexradiusbuilder::normalising_scale;

type Lanes = [f32; 4];

/// The floats `0.0` (`01011d78`), `3.0` (`01017718`) and `0.5` (`01016248`) the refined reciprocal
/// square root uses.
const ZERO_00D4F3D0: u32 = 0x0101_1d78;
const THREE_00D4F3D0: u32 = 0x0101_7718;
const HALF_00D4F3D0: u32 = 0x0101_6248;

fn lanes_00d4f3d0(e: &Engine, at: u32) -> Lanes {
    [0u32, 1, 2, 3].map(|lane| e.mem.f32(at.wrapping_add(lane * 4)))
}

fn store_00d4f3d0(e: &mut Engine, at: u32, value: Lanes) {
    for (lane, word) in value.into_iter().enumerate() {
        e.mem
            .set_u32(at.wrapping_add(lane as u32 * 4), word.to_bits());
    }
}

/// `MULPS a, b` (the SSE NaN rules: `a` is the destination).
fn mul_00d4f3d0(a: Lanes, b: Lanes) -> Lanes {
    crate::sse::mulps(a, b)
}

/// `z + (y + x)` with `ADDSS` (the higher lane the destination, the SSE NaN rules).
fn sum_of_three(v: Lanes) -> f32 {
    crate::sse::add(v[2], crate::sse::add(v[1], v[0]))
}

/// `v` scaled by the refined reciprocal square root of its first three lanes' squared length (0 for
/// a zero length): `MULPS`, the `ADDSS` sums, `RSQRTSS` (the host's estimate), one Newton step and
/// the `CMPEQSS`/`ANDNPS` zero test, then a `MULPS` with the scale in every lane.
fn normalised_00d4f3d0(e: &Engine, v: Lanes) -> Lanes {
    let zero = e.mem.f32(ZERO_00D4F3D0);
    let three = e.mem.f32(THREE_00D4F3D0);
    let half = e.mem.f32(HALF_00D4F3D0);
    let scale = normalising_scale(sum_of_three(mul_00d4f3d0(v, v)), zero, half, three);
    mul_00d4f3d0([scale; 4], v)
}

/// `(y, z, x, w)`: `SHUFPS v, v, 0xc9`.
fn yzx_00d4f3d0(v: Lanes) -> Lanes {
    [v[1], v[2], v[0], v[3]]
}

/// `(z, x, y, w)`: `SHUFPS v, v, 0xd2`.
fn zxy_00d4f3d0(v: Lanes) -> Lanes {
    [v[2], v[0], v[1], v[3]]
}

/// `a x b` as the exe orders it: `b.zxy * a.yzx - b.yzx * a.zxy`.
fn cross_00d4f3d0(a: Lanes, b: Lanes) -> Lanes {
    let minuend = mul_00d4f3d0(zxy_00d4f3d0(b), yzx_00d4f3d0(a));
    let subtrahend = mul_00d4f3d0(yzx_00d4f3d0(b), zxy_00d4f3d0(a));
    crate::sse::subps(minuend, subtrahend)
}

/// The three rows at `at` packed by the `UNPCKLPS`/`UNPCKHPS`/`MOVLHPS`/`MOVHLPS`/`SHUFPS`
/// sequence into `P = (r0.x, r1.x, r2.x, r2.y)`, `Q = (r0.y, r1.y, r2.y, r2.w)` and
/// `R = (r0.z, r1.z, r2.z, r2.w)`.
fn packed_rows_00d4f3d0(e: &Engine, at: u32) -> [Lanes; 3] {
    let r0 = lanes_00d4f3d0(e, at);
    let r1 = lanes_00d4f3d0(e, at + 0x10);
    let r2 = lanes_00d4f3d0(e, at + 0x20);
    [
        [r0[0], r1[0], r2[0], r2[1]],
        [r0[1], r1[1], r2[1], r2[3]],
        [r0[2], r1[2], r2[2], r2[3]],
    ]
}

/// `(vy * Q + vx * P) + vz * R` per lane: the three `MULPS` and two `ADDPS` of the loops.
fn transformed_00d4f3d0(v: Lanes, packed: [Lanes; 3]) -> Lanes {
    let by_y = mul_00d4f3d0([v[1]; 4], packed[1]);
    let by_x = mul_00d4f3d0([v[0]; 4], packed[0]);
    let by_z = mul_00d4f3d0([v[2]; 4], packed[2]);
    crate::sse::addps(crate::sse::addps(by_y, by_x), by_z)
}

// Translated from 00d4f3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 0x10`: four stack words `first_matrix`, `second_matrix`, `first`,
/// `second`; descriptions, not confirmed): builds an orthonormal-looking frame from two vectors and
/// stores it twice, each time expressed through a 3-row matrix. `ua` and `ub` are the vectors at
/// `first` and `second` normalised as in `normalised_00d4f3d0` (zero stays zero) and `c = ua x
/// ub`. Each of `ua`, `ub`, `c` goes through the packed rows of `first_matrix` (`(vy * Q + vx * P)
/// + vz * R`) into `this + 0x20`, `+0x30`, `+0x40`, then through those of `second_matrix` into
/// `this + 0x50`, `+0x60`, `+0x70`.
pub fn fn_00d4f3d0(
    e: &mut Engine,
    this: Ptr,
    first_matrix: u32,
    second_matrix: u32,
    first: u32,
    second: u32,
) {
    let me = this.addr();
    let ua = normalised_00d4f3d0(e, lanes_00d4f3d0(e, first));
    let ub = normalised_00d4f3d0(e, lanes_00d4f3d0(e, second));
    let c = cross_00d4f3d0(ua, ub);
    let packed = packed_rows_00d4f3d0(e, first_matrix);
    for (index, v) in [ua, ub, c].into_iter().enumerate() {
        let moved = transformed_00d4f3d0(v, packed);
        store_00d4f3d0(e, me + 0x20 + index as u32 * 0x10, moved);
    }
    let packed = packed_rows_00d4f3d0(e, second_matrix);
    for (index, v) in [ua, ub, c].into_iter().enumerate() {
        let moved = transformed_00d4f3d0(v, packed);
        store_00d4f3d0(e, me + 0x50 + index as u32 * 0x10, moved);
    }
}

// Translated from 00d4f5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 0x10`: four stack words `copy_a`, `copy_b`, `direct_a`,
/// `direct_b`; descriptions, not confirmed): stores a frame of vectors at `this + 0x20` .. `+0x70`:
/// `+0x20` is the normalised vector at `direct_a`; `+0x30` is the vector at `copy_a` copied and
/// then normalised in place; `+0x40` is the cross product of those two; `+0x50` is the normalised
/// vector at `direct_b`, `+0x60` the vector at `copy_b` copied and normalised, `+0x70` their cross
/// product (`a x b` as the exe orders it). Normalising is as in `normalised_00d4f3d0` (the
/// host's reciprocal square root estimate, one Newton step, zero stays zero).
pub fn fn_00d4f5d0(
    e: &mut Engine,
    this: Ptr,
    copy_a: u32,
    copy_b: u32,
    direct_a: u32,
    direct_b: u32,
) {
    let me = this.addr();
    let first = normalised_00d4f3d0(e, lanes_00d4f3d0(e, direct_a));
    store_00d4f3d0(e, me + 0x20, first);
    let raw = lanes_00d4f3d0(e, copy_a);
    store_00d4f3d0(e, me + 0x30, raw);
    let second = normalised_00d4f3d0(e, lanes_00d4f3d0(e, me + 0x30));
    store_00d4f3d0(e, me + 0x30, second);
    let first = lanes_00d4f3d0(e, me + 0x20);
    let second = lanes_00d4f3d0(e, me + 0x30);
    store_00d4f3d0(e, me + 0x40, cross_00d4f3d0(first, second));
    let third = normalised_00d4f3d0(e, lanes_00d4f3d0(e, direct_b));
    store_00d4f3d0(e, me + 0x50, third);
    let raw = lanes_00d4f3d0(e, copy_b);
    store_00d4f3d0(e, me + 0x60, raw);
    let fourth = normalised_00d4f3d0(e, lanes_00d4f3d0(e, me + 0x60));
    store_00d4f3d0(e, me + 0x60, fourth);
    let third = lanes_00d4f3d0(e, me + 0x50);
    let fourth = lanes_00d4f3d0(e, me + 0x60);
    store_00d4f3d0(e, me + 0x70, cross_00d4f3d0(third, fourth));
}

// Translated from 00d4f7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 4`: one stack word `out`): copies the three vectors at `this + 0x20`,
/// `+0x30`, `+0x40` to `out`, `out + 0x10`, `out + 0x20` (whole 16-byte copies).
pub fn fn_00d4f7f0(e: &mut Engine, this: Ptr, out: Ptr) {
    for (from, to) in [(0x20u32, 0u32), (0x30, 0x10), (0x40, 0x20)] {
        for lane in 0..4 {
            let bits = e.mem.u32(this.addr() + from + lane * 4);
            e.mem.set_u32(out.addr() + to + lane * 4, bits);
        }
    }
}

// Translated from 00d4f820 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 4`: one stack word `out`): copies the three vectors at `this + 0x50`,
/// `+0x60`, `+0x70` to `out`, `out + 0x10`, `out + 0x20` (whole 16-byte copies).
pub fn fn_00d4f820(e: &mut Engine, this: Ptr, out: Ptr) {
    for (from, to) in [(0x50u32, 0u32), (0x60, 0x10), (0x70, 0x20)] {
        for lane in 0..4 {
            let bits = e.mem.u32(this.addr() + from + lane * 4);
            e.mem.set_u32(out.addr() + to + lane * 4, bits);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d4_f3d0, fn_00d4f3d0(Ptr, u32, u32, u32, u32)),
        entry!(0x00d4_f5d0, fn_00d4f5d0(Ptr, u32, u32, u32, u32)),
        entry!(0x00d4_f7f0, fn_00d4f7f0(Ptr, Ptr)),
        entry!(0x00d4_f820, fn_00d4f820(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f32(ZERO_00D4F3D0, 0.0);
        e.mem.set_f32(THREE_00D4F3D0, 3.0);
        e.mem.set_f32(HALF_00D4F3D0, 0.5);
        e
    }

    fn put(e: &mut Engine, lanes: Lanes) -> u32 {
        let at = e.mem.alloc(0x10);
        store_00d4f3d0(e, at, lanes);
        at
    }

    fn rows(e: &mut Engine, rows: [Lanes; 3]) -> u32 {
        let at = e.mem.alloc(0x30);
        for (index, row) in rows.into_iter().enumerate() {
            store_00d4f3d0(e, at + index as u32 * 0x10, row);
        }
        at
    }

    /// The estimate and Newton step differ in the last bit between CPUs: a few ulps are allowed.
    fn near(actual: Lanes, expected: Lanes) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() <= 1e-5,
                "lane {lane}: {actual:?} vs {expected:?}"
            );
        }
    }

    #[test]
    fn fn_00d4f3d0_stores_the_frame_through_both_matrices() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        let identity = rows(
            &mut e,
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
        );
        // Swaps x and y.
        let swap = rows(
            &mut e,
            [
                [0.0, 1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
        );
        let a = put(&mut e, [3.0, 0.0, 0.0, 0.0]);
        let b = put(&mut e, [0.0, 5.0, 0.0, 0.0]);
        e.call(0x00d4_f3d0, &args![this, identity, swap, a, b]);
        let at = |e: &Engine, offset: u32| lanes_00d4f3d0(e, this + offset);
        near(at(&e, 0x20), [1.0, 0.0, 0.0, 0.0]);
        near(at(&e, 0x30), [0.0, 1.0, 0.0, 0.0]);
        near(at(&e, 0x40), [0.0, 0.0, 1.0, 0.0]);
        near(at(&e, 0x50), [0.0, 1.0, 0.0, 0.0]);
        near(at(&e, 0x60), [1.0, 0.0, 0.0, 0.0]);
        near(at(&e, 0x70), [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn fn_00d4f3d0_leaves_a_zero_vector_zero() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        let identity = rows(
            &mut e,
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
        );
        let a = put(&mut e, [0.0; 4]);
        let b = put(&mut e, [0.0, 2.0, 0.0, 0.0]);
        e.call(0x00d4_f3d0, &args![this, identity, identity, a, b]);
        assert_eq!(lanes_00d4f3d0(&e, this + 0x20), [0.0; 4]);
        near(lanes_00d4f3d0(&e, this + 0x30), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(lanes_00d4f3d0(&e, this + 0x40), [0.0; 4]);
    }

    #[test]
    fn fn_00d4f5d0_stores_two_normalised_pairs_and_their_cross_products() {
        let mut e = engine();
        let this = e.mem.alloc(0x100);
        let direct_a = put(&mut e, [0.0, 0.0, 2.0, 0.0]);
        let copy_a = put(&mut e, [3.0, 0.0, 0.0, 0.0]);
        let direct_b = put(&mut e, [0.0, 4.0, 0.0, 0.0]);
        let copy_b = put(&mut e, [0.0, 0.0, 7.0, 0.0]);
        e.call(
            0x00d4_f5d0,
            &args![this, copy_a, copy_b, direct_a, direct_b],
        );
        let at = |e: &Engine, offset: u32| lanes_00d4f3d0(e, this + offset);
        near(at(&e, 0x20), [0.0, 0.0, 1.0, 0.0]);
        near(at(&e, 0x30), [1.0, 0.0, 0.0, 0.0]);
        // (0, 0, 1) x (1, 0, 0) = (0, 1, 0)
        near(at(&e, 0x40), [0.0, 1.0, 0.0, 0.0]);
        near(at(&e, 0x50), [0.0, 1.0, 0.0, 0.0]);
        near(at(&e, 0x60), [0.0, 0.0, 1.0, 0.0]);
        // (0, 1, 0) x (0, 0, 1) = (1, 0, 0)
        near(at(&e, 0x70), [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn fn_00d4f7f0_and_fn_00d4f820_copy_the_vectors_out() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x100);
        for word in 0..0x20u32 {
            e.mem.set_u32(this + 0x20 + word * 4, 0x100 + word);
        }
        let out = e.mem.alloc(0x40);
        e.call(0x00d4_f7f0, &args![this, out]);
        for word in 0..12u32 {
            assert_eq!(e.mem.u32(out + word * 4), 0x100 + word);
        }
        assert_eq!(e.mem.u32(out + 0x30), 0);
        e.call(0x00d4_f820, &args![this, out]);
        for word in 0..12u32 {
            assert_eq!(e.mem.u32(out + word * 4), 0x100 + 12 + word);
        }
    }
}

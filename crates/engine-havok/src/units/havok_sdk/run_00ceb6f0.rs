//! `Havok SDK/run_00ceb6f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::sse::{mulps, subps};
use crate::units::havok_sdk::hkpconvexradiusbuilder::shufps;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, Vec4};

/// The doubles `0.0001` (`01032980`) and `-0.0001` (`010cb430`) the determinant is compared with.
const EPSILON: u32 = 0x0103_2980;
const MINUS_EPSILON: u32 = 0x010c_b430;

/// The cross product of `a` and `b` as the code forms it: `a.yzx * b.zxy - a.zxy * b.yzx`.
fn cross(a: Vec4, b: Vec4) -> Vec4 {
    subps(
        mulps(shufps(a, a, 0xc9), shufps(b, b, 0xd2)),
        mulps(shufps(a, a, 0xd2), shufps(b, b, 0xc9)),
    )
}

/// `z + (y + x)` with `ADDSS` (the higher lane the destination, the SSE NaN rules).
fn sum_of_three(v: Vec4) -> f32 {
    crate::sse::add(v[2], crate::sse::add(v[1], v[0]))
}

/// What an `FLD dword` of the float `value` puts on the x87 stack.
fn loaded(value: f32) -> f64 {
    x87_load32(value.to_bits())
}

// Translated from 00ceb6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(out_hit, out_scaled, out_u, out_v, point, direction, base, first, second, flag)`,
/// the intersection test of a line with a triangle (the class of the arguments is
/// unconfirmed). With `base`, `first` and `second` the triangle's corner and the points that
/// give its edges (`e1 = first - base`, `e2 = second - base`), `n = direction x e2` and
/// `det = n . e1` (each dot summed `z + (y + x)`):
/// - when `flag` is not 0 and `det < 0.0001`, or when `flag` is 0 and `-0.0001 < det < 0.0001`
///   (an x87 compare: a NaN determinant passes), the byte at `out_hit` is set to 0 and nothing
///   else is written;
/// - `u = (point - base) . n` is written to `out_u` (copied through the x87 stack); when `u < 0`
///   or `u > det` the byte is set to 0 and the function returns;
/// - with `w = (point - base) x e1`, `v = direction . w` is written to `out_v`; when `v < 0` or
///   `v + u > det` (reading `u` back from memory) the byte is set to 0 and the function returns;
/// - otherwise `out_scaled` receives `((w . e2) * inverse)` where `inverse` is `1 / det` rounded
///   to a float, `out_hit` is set to 1 and `u` and `v` are replaced by `inverse * u` and
///   `inverse * v` (the memory operands read back as the x87 sees them).
pub fn fn_00ceb6f0(
    e: &mut Engine,
    out_hit: Ptr,
    out_scaled: Ptr,
    out_u: Ptr,
    out_v: Ptr,
    point: Ptr,
    direction: Ptr,
    base: Ptr,
    first: Ptr,
    second: Ptr,
    flag: u8,
) {
    let origin = load_vec(e, base.addr());
    let direction_vec = load_vec(e, direction.addr());
    let to_second = subps(load_vec(e, second.addr()), origin);
    let normal = cross(direction_vec, to_second);
    let to_first = subps(load_vec(e, first.addr()), origin);
    let determinant_float = sum_of_three(mulps(normal, to_first));
    let determinant = loaded(determinant_float);
    let epsilon: f64 = e.global(EPSILON);
    let minus_epsilon: f64 = e.global(MINUS_EPSILON);
    let fail = if flag != 0 {
        determinant < epsilon
    } else {
        determinant > minus_epsilon && determinant < epsilon
    };
    if fail {
        e.mem.set_u8(out_hit.addr(), 0);
        return;
    }
    let inverse_stored = x87_store32(x87_div(1.0, determinant));
    let inverse = x87_load32(inverse_stored);
    let from_base = subps(load_vec(e, point.addr()), origin);
    let u_float = sum_of_three(mulps(from_base, normal));
    let u_bits = x87_store32(loaded(u_float));
    e.mem.set_u32(out_u.addr(), u_bits);
    let u = loaded(u_float);
    // FCOM of zero with u: C0 and C3 clear (0 > u) leaves the function.
    if 0.0 > u {
        e.mem.set_u8(out_hit.addr(), 0);
        return;
    }
    // FCOMP of u with the determinant: clear flags (u > det) leave it.
    if u > determinant {
        e.mem.set_u8(out_hit.addr(), 0);
        return;
    }
    let w = cross(from_base, to_first);
    let v_float = sum_of_three(mulps(direction_vec, w));
    let v_bits = x87_store32(loaded(v_float));
    e.mem.set_u32(out_v.addr(), v_bits);
    let v = loaded(v_float);
    // FCOM of v with zero then TEST 5, JNP: only an ordered v < 0 leaves.
    if v < 0.0 {
        e.mem.set_u8(out_hit.addr(), 0);
        return;
    }
    let sum = x87_add(v, x87_m32(e.mem.u32(out_u.addr())));
    // FCOMPP of the sum with the determinant: clear flags (sum > det) leave.
    if sum > determinant {
        e.mem.set_u8(out_hit.addr(), 0);
        return;
    }
    // `MULPS` of `w` with `second - base` still in its register.
    let scaled_float = sum_of_three(mulps(w, to_second));
    let scaled = x87_mul(loaded(scaled_float), inverse);
    e.mem.set_u32(out_scaled.addr(), x87_store32(scaled));
    e.mem.set_u8(out_hit.addr(), 1);
    let scaled_u = x87_mul(inverse, x87_m32(e.mem.u32(out_u.addr())));
    e.mem.set_u32(out_u.addr(), x87_store32(scaled_u));
    let scaled_v = x87_mul(inverse, x87_m32(e.mem.u32(out_v.addr())));
    e.mem.set_u32(out_v.addr(), x87_store32(scaled_v));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00ce_b6f0,
        fn_00ceb6f0(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, u8)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    const HIT: u32 = 0;
    const SCALED: u32 = 4;
    const U: u32 = 8;
    const V: u32 = 12;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(EPSILON, 0.0001);
        e.mem.set_f64(MINUS_EPSILON, -0.0001);
        e
    }

    fn vector(e: &mut Engine, v: [f32; 3]) -> Ptr {
        let at = e.mem.alloc(0x10);
        for (i, value) in v.into_iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, value);
        }
        Ptr::new(at)
    }

    /// Runs the test with the line `point + t * direction` against the triangle
    /// `base`, `first`, `second` and returns `(hit, scaled, u, v)`.
    fn run(
        point: [f32; 3],
        direction: [f32; 3],
        corners: [[f32; 3]; 3],
        flag: u8,
    ) -> (u8, f32, f32, f32) {
        let mut e = engine();
        let outputs = e.mem.alloc(0x10);
        e.mem.set_u8(outputs + HIT, 0xee);
        for lane in 1..4 {
            e.mem.set_f32(outputs + 4 * lane, -7.0);
        }
        let point = vector(&mut e, point);
        let direction = vector(&mut e, direction);
        let [base, first, second] = corners.map(|corner| vector(&mut e, corner));
        e.call(
            0x00ce_b6f0,
            &args![
                outputs + HIT,
                outputs + SCALED,
                outputs + U,
                outputs + V,
                point,
                direction,
                base,
                first,
                second,
                flag
            ],
        );
        (
            e.mem.u8(outputs + HIT),
            e.mem.f32(outputs + SCALED),
            e.mem.f32(outputs + U),
            e.mem.f32(outputs + V),
        )
    }

    const TRIANGLE: [[f32; 3]; 3] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];

    /// A line through the triangle interior hits. With the direction `(0, 0, -1)`: `n = (1, 0,
    /// 0)`, `det = 1`; from the point `(0.25, 0.25, 5)`: `u = 0.25`, `w = (0, 5, -0.25)`, `v =
    /// 0.25`, `w . e2 = 5`; the inverse determinant is 1.
    #[test]
    fn a_line_through_the_interior_hits() {
        let (hit, scaled, u, v) = run([0.25, 0.25, 5.0], [0.0, 0.0, -1.0], TRIANGLE, 0);
        assert_eq!(hit, 1);
        assert_eq!((u, v, scaled), (0.25, 0.25, 5.0));
    }

    /// `u` outside `[0, det]`, `v < 0` and `u + v > det` each fail; the first two still leave
    /// the values they stored.
    #[test]
    fn misses_leave_the_stored_values_as_the_exe_does() {
        // u = -1.25 < 0.
        let (hit, scaled, u, v) = run([-1.25, 0.25, 5.0], [0.0, 0.0, -1.0], TRIANGLE, 0);
        assert_eq!((hit, scaled, u, v), (0, -7.0, -1.25, -7.0));
        // u > det (u = 2).
        let (hit, _, u, v) = run([2.0, 0.25, 5.0], [0.0, 0.0, -1.0], TRIANGLE, 0);
        assert_eq!((hit, u, v), (0, 2.0, -7.0));
        // v < 0.
        let (hit, _, u, v) = run([0.25, -0.5, 5.0], [0.0, 0.0, -1.0], TRIANGLE, 0);
        assert_eq!((hit, u, v), (0, 0.25, -0.5));
        // u + v > det.
        let (hit, _, u, v) = run([0.75, 0.75, 5.0], [0.0, 0.0, -1.0], TRIANGLE, 0);
        assert_eq!((hit, u, v), (0, 0.75, 0.75));
    }

    /// With the flag set a determinant below the epsilon fails (also a negative one); with the
    /// flag clear only a determinant near zero does.
    #[test]
    fn the_flag_decides_how_a_small_determinant_is_treated() {
        // The line along the triangle's plane: det = 0.
        let flat = [0.0, 1.0, 0.0];
        let (hit, scaled, u, v) = run([0.25, 0.25, 5.0], flat, TRIANGLE, 0);
        assert_eq!((hit, scaled, u, v), (0, -7.0, -7.0, -7.0));
        let (hit, ..) = run([0.25, 0.25, 5.0], flat, TRIANGLE, 1);
        assert_eq!(hit, 0);
        // det = -1 (direction +z): fails with the flag, goes on without it (and then fails
        // the range check on u since u = -0.25 < 0).
        let (hit, _, u, _) = run([0.25, 0.25, 5.0], [0.0, 0.0, 1.0], TRIANGLE, 1);
        assert_eq!((hit, u), (0, -7.0));
        let (hit, _, u, _) = run([0.25, 0.25, 5.0], [0.0, 0.0, 1.0], TRIANGLE, 0);
        assert_eq!((hit, u), (0, -0.25));
    }
}

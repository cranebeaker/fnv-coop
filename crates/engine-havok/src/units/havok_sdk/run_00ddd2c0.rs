//! `Havok SDK/run_00ddd2c0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::{
    compare_flags, load_vec4, mul4, sub4, sum3, test_parity_even, Vec4,
};

/// The smallest and largest `float` (`0104e148`, `0104da48`), the `float` limit near zero
/// (`01013ea4`, 0.01) and the `double` limit (`01018a90`, 0.2).
const SMALLEST_FLOAT: u32 = 0x0104_e148;
const LARGEST_FLOAT: u32 = 0x0104_da48;
const NEAR_LIMIT: u32 = 0x0101_3ea4;
const FAR_LIMIT: u32 = 0x0101_8a90;

/// The smallest and largest of the three distances of the triangle's vertices (the three 16-byte
/// vectors at `triangle + 0x20`, `+0x30`, `+0x40`) along `axis` from `origin`
/// (`(vertex - origin) * axis`, lanes 0 to 2 summed in the code's order), as the x87 stack keeps
/// them: they start as the largest and smallest `float`, and a distance replaces the maximum when
/// it is above it and the minimum when it is below it (x87 compares: a NaN replaces neither).
fn extent(e: &Engine, origin: Vec4, axis: Vec4, triangle: u32) -> (f64, f64) {
    let mut maximum = x87_load32(e.mem.u32(SMALLEST_FLOAT));
    let mut minimum = x87_load32(e.mem.u32(LARGEST_FLOAT));
    for vertex in 0..3u32 {
        let at = load_vec4(e, triangle + 0x20 + 0x10 * vertex);
        let distance = sum3(mul4(sub4(at, origin), axis));
        let loaded = x87_load32(distance.to_bits());
        if compare_flags(loaded, maximum) & 0x41 == 0 {
            maximum = x87_load32(x87_store32(loaded));
        }
        if !test_parity_even(compare_flags(loaded, minimum), 5) {
            minimum = x87_load32(x87_store32(loaded));
        }
    }
    (minimum, maximum)
}

// Translated from 00ddd2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `origin`, `first`, `second` (16-byte vectors), `triangle`
/// (three vectors at `+0x20`, `+0x30`, `+0x40`), `direction` (a 16-byte vector)). Answers 1 or 0.
///
/// It looks at the triangle's extent along `first` (see `extent`) and, when the maximum is above
/// the `float` 0.01 (`01013ea4`) or the minimum below it, answers 0 if the extent also straddles
/// the `double` 0.2 (`01018a90`): maximum above and minimum below it. Otherwise it forms `flag`
/// (1 when `|minimum| < |maximum|`, else 0) and answers 1 when `(second . direction) * flag` is
/// below 0.0 (lanes 0 to 2 summed in the code's order, `MULSS` by the flag as a `float`). Then the
/// same with the roles of `first` and `second` swapped, answering 1 when `(first . direction) *
/// flag` is above 0.0; and 0 when nothing answered 1.
pub fn fn_00ddd2c0(
    e: &mut Engine,
    origin: u32,
    first: u32,
    second: u32,
    triangle: u32,
    direction: u32,
) -> u32 {
    let from = load_vec4(e, origin);
    let first_axis = load_vec4(e, first);
    let near = x87_load32(e.mem.u32(NEAR_LIMIT));
    let far = e.mem.f64(FAR_LIMIT);
    let (minimum, maximum) = extent(e, from, first_axis, triangle);
    if near < maximum || minimum < near {
        if far < maximum && minimum < far {
            return 0;
        }
        let flag = if minimum.abs() < maximum.abs() {
            1.0f32
        } else {
            0.0
        };
        let other = load_vec4(e, second);
        let towards = load_vec4(e, direction);
        let value = sum3(mul4(other, towards)) * flag;
        if compare_flags(0.0, x87_m32(value.to_bits())) & 0x41 == 0 {
            return 1;
        }
    }
    let second_axis = load_vec4(e, second);
    let (minimum, maximum) = extent(e, from, second_axis, triangle);
    if near < maximum || minimum < near {
        if far < maximum && minimum < far {
            return 0;
        }
        let flag = if minimum.abs() < maximum.abs() {
            1.0f32
        } else {
            0.0
        };
        let towards = load_vec4(e, direction);
        let value = sum3(mul4(first_axis, towards)) * flag;
        if !test_parity_even(compare_flags(0.0, x87_m32(value.to_bits())), 5) {
            return 1;
        }
    }
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ddd2c0, fn_00ddd2c0(u32, u32, u32, u32, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkgeometryprocessing::store_vec4;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0104_e000, 0x0104_d000, 0x0101_3000, 0x0101_8000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(SMALLEST_FLOAT, -f32::MAX);
        e.mem.set_f32(LARGEST_FLOAT, f32::MAX);
        e.mem.set_f32(NEAR_LIMIT, 0.01);
        e.mem.set_f64(FAR_LIMIT, 0.2);
        e
    }

    fn vector(e: &mut Engine, lanes: Vec4) -> u32 {
        let at = e.mem.alloc(16);
        store_vec4(e, at, lanes);
        at
    }

    /// A triangle whose vertices have the given x and y.
    fn triangle(e: &mut Engine, x: [f32; 3], y: [f32; 3]) -> u32 {
        let at = e.mem.alloc(0x60);
        for k in 0..3 {
            store_vec4(e, at + 0x20 + 0x10 * k as u32, [x[k], y[k], 0.0, 0.0]);
        }
        at
    }

    fn run(x: [f32; 3], y: [f32; 3], direction: Vec4) -> u32 {
        let mut e = engine();
        let origin = vector(&mut e, [0.0; 4]);
        let first = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let second = vector(&mut e, [0.0, 1.0, 0.0, 0.0]);
        let tri = triangle(&mut e, x, y);
        let d = vector(&mut e, direction);
        e.call(0x00dd_d2c0, &args![origin, first, second, tri, d])
            .u32()
    }

    #[test]
    fn a_triangle_straddling_the_far_limit_answers_zero() {
        assert_eq!(run([0.1, 0.3, 0.5], [0.0; 3], [0.0, -1.0, 0.0, 0.0]), 0);
    }

    #[test]
    fn the_first_axis_answers_one_when_the_second_axis_points_against_the_direction() {
        assert_eq!(run([0.5, 0.6, 0.7], [0.0; 3], [0.0, -1.0, 0.0, 0.0]), 1);
    }

    #[test]
    fn the_second_axis_answers_one_when_the_first_axis_points_along_the_direction() {
        assert_eq!(
            run([0.5, 0.6, 0.7], [0.3, 0.4, 0.5], [1.0, 1.0, 0.0, 0.0]),
            1
        );
    }

    #[test]
    fn a_zero_dot_product_or_a_flat_triangle_answers_zero() {
        assert_eq!(
            run([0.5, 0.6, 0.7], [0.3, 0.4, 0.5], [0.0, 1.0, 0.0, 0.0]),
            0
        );
        // All distances equal to the near limit: neither test passes, either axis.
        assert_eq!(run([0.01; 3], [0.01; 3], [1.0, 1.0, 0.0, 0.0]), 0);
    }

    #[test]
    fn a_nan_distance_changes_neither_extreme() {
        // A NaN vertex is ignored: the others straddle nothing and satisfy the first test.
        assert_eq!(
            run([f32::NAN, 0.6, 0.7], [0.3, 0.4, 0.5], [1.0, 1.0, 0.0, 0.0]),
            1
        );
    }
}

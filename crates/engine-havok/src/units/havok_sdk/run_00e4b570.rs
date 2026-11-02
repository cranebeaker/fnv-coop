//! `Havok SDK/run_00e4b570` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{
    addss, fcom_status, load_vec, movmskps, mulps, mulss, rsqrtss, shufps, store_vec, subps, subss,
    Vec4,
};

/// The `float` written into the weights of a rejected corner (`01012054`).
const REJECTED_WEIGHT: u32 = 0x0101_2054;

/// `z + (y + x)` in lane 0, as the `SHUFPS` / `ADDSS` pairs of this unit compute the dot-product sum
/// of a vector of products: `y` is added to `x` first (the broadcast `y` is the destination), then
/// that sum to the broadcast `z`.
fn sum_of_three(v: Vec4) -> Vec4 {
    let partial = addss(shufps(v, v, 0x55), v);
    addss(shufps(v, v, 0xaa), partial)
}

// Translated from 00e4b570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, three words (nothing else is known of what it is called by: no reference to it was
/// found). `vertices` holds four 16-byte points (a triangle's corners 0 to 2 and a fourth point
/// at `+0x30`), `point` one more, `weights` three `float`s. For each of the three corner pairs
/// `(0, 1)`, `(1, 2)`, `(2, 0)` whose two weights are both `>= 0` (a NaN or negative weight
/// skips the pair), a scalar triple-product-like sign test is made from the dot products of the
/// vectors from the pair's third corner (`b`) to `point`, the other corners and the fourth
/// point; when the result is non-negative the first weight of the pair is replaced by the
/// `float` at `01012054` (`-1.0` through the x87 stack), otherwise the second. Returns the index of
/// the first weight that is then above 0, or `-1`.
pub fn fn_00e4b570(e: &mut Engine, point: u32, vertices: u32, weights: u32) -> u32 {
    let rejected = x87_quiet_bits(e.mem.u32(REJECTED_WEIGHT));
    // (weight offset j, weight offset k, vertex offsets a, b, c) of the three rounds.
    let rounds: [(u32, u32, u32, u32, u32); 3] = [
        (0, 4, 0x00, 0x20, 0x10),
        (4, 8, 0x10, 0x00, 0x20),
        (8, 0, 0x20, 0x10, 0x00),
    ];
    for (first, second, a_offset, b_offset, c_offset) in rounds {
        // FCOM of 0.0 with the weight, `TEST AH, 0x41` and `JP`: only equal or below pass.
        let usable = |e: &Engine, offset: u32| {
            let status = fcom_status(0.0, x87_m32(e.mem.u32(weights.wrapping_add(offset))));
            matches!(status & 0x41, 0x01 | 0x40)
        };
        if !usable(e, first) || !usable(e, second) {
            continue;
        }
        let p = load_vec(e, point);
        let a = load_vec(e, vertices.wrapping_add(a_offset));
        let b = load_vec(e, vertices.wrapping_add(b_offset));
        let c = load_vec(e, vertices.wrapping_add(c_offset));
        let fourth = load_vec(e, vertices.wrapping_add(0x30));
        let from_b = subps(p, b);
        let a_from_b = subps(a, b);
        let fourth_from_b = subps(fourth, b);
        let c_from_b = subps(c, b);
        let point_c = mulps(from_b, c_from_b);
        let point_a = mulps(from_b, a_from_b);
        let fourth_a = mulps(fourth_from_b, a_from_b);
        let fourth_c = mulps(fourth_from_b, c_from_b);
        let sum_fourth_a = sum_of_three(fourth_a);
        let sum_point_c = sum_of_three(point_c);
        let sum_fourth_c = sum_of_three(fourth_c);
        let sum_point_a = sum_of_three(point_a);
        let left = mulss(sum_point_c, sum_fourth_a);
        let right = mulss(sum_point_a, sum_fourth_c);
        let result = subss(right, left);
        let target = if movmskps(result) & 1 == 0 {
            first
        } else {
            second
        };
        e.mem.set_u32(weights.wrapping_add(target), rejected);
    }
    // FCOM with 0.0 against each weight, `TEST AH, 5` and `JP`: only a strictly larger weight
    // (C0 set, C2 clear) answers.
    for (index, offset) in [0u32, 4, 8].into_iter().enumerate() {
        let status = fcom_status(0.0, x87_m32(e.mem.u32(weights.wrapping_add(offset))));
        if status & 0x05 == 0x01 {
            return index as u32;
        }
    }
    u32::MAX
}

// Translated from 00e4b6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register inputs first: `edx` points at a record whose vectors at `+0` and `+0x10` are the end
/// points `a` and `b` of a segment (and `+0x40`, `+0x50` a second pair), `esi` at the output
/// vector and `edi` is the mode; then one stack word, `point`, a pointer to a vector (the call is
/// cdecl). With `e = b - a`, the signs of `(b - point) . e` and `(a - point) . e` (sum
/// `z + (y + x)`) are compared. When they differ (the point is beside the segment) the output is
/// the cross product of the cross product of `(a - point)` and `(b - point)` with `e / |e|`
/// (`RSQRTSS`, a host estimate), negated unless `edi` is 1. When both are negative the segment's
/// first end is overwritten by the second (and the second pair of vectors likewise); the output
/// is then `point - a` when `edi` is 1, else `a - point`.
pub fn fn_00e4b6e0(e: &mut Engine, edx: u32, esi: u32, edi: u32, point: u32) {
    let p = load_vec(e, point);
    let b = load_vec(e, edx.wrapping_add(0x10));
    let a = load_vec(e, edx);
    let along = subps(b, a);
    let b_from_p = subps(b, p);
    let sum_b = sum_of_three(mulps(b_from_p, along));
    let a_from_p = subps(a, p);
    let sum_a = sum_of_three(mulps(a_from_p, along));
    let sign_b = movmskps(sum_b) & 1;
    let sign_a = movmskps(sum_a) & 1;
    if sign_b != sign_a {
        let squared = mulps(along, along);
        let length_squared = sum_of_three(squared);
        let inverse_length = rsqrtss(length_squared, length_squared);
        let b_d2 = shufps(b_from_p, b_from_p, 0xd2);
        let direction = mulps(shufps(inverse_length, inverse_length, 0x00), along);
        let first = mulps(shufps(a_from_p, a_from_p, 0xc9), b_d2);
        let b_c9 = shufps(b_from_p, b_from_p, 0xc9);
        let second = mulps(shufps(a_from_p, a_from_p, 0xd2), b_c9);
        let cross = subps(second, first);
        let direction_d2 = shufps(direction, direction, 0xd2);
        let left = mulps(shufps(cross, cross, 0xc9), direction_d2);
        let direction_c9 = shufps(direction, direction, 0xc9);
        let right = mulps(shufps(cross, cross, 0xd2), direction_c9);
        let result = if edi == 1 {
            subps(right, left)
        } else {
            subps(left, right)
        };
        store_vec(e, esi, result);
        return;
    }
    if sign_b != 0 {
        let moved = load_vec(e, edx.wrapping_add(0x10));
        store_vec(e, edx, moved);
        let moved = load_vec(e, edx.wrapping_add(0x50));
        store_vec(e, edx.wrapping_add(0x40), moved);
    }
    let vertex = load_vec(e, edx);
    let result = if edi == 1 {
        subps(p, vertex)
    } else {
        subps(vertex, p)
    };
    store_vec(e, esi, result);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_b570, fn_00e4b570(u32, u32, u32) -> u32),
        entry!(0x00e4_b6e0, fn_00e4b6e0(u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0010_0000);
        e.mem.set_f32(REJECTED_WEIGHT, -1.0);
        e
    }

    /// Four corners `(0, 0, 0)`, `(1, 0, 0)`, `(0, 1, 0)`, `(0, 0, 1)` at 16-byte spacing.
    fn corners(e: &mut Engine) -> u32 {
        let base = e.mem.alloc(0x40);
        for (index, v) in [
            [0.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ]
        .iter()
        .enumerate()
        {
            store_vec(e, base + 16 * index as u32, *v);
        }
        base
    }

    fn weights(e: &mut Engine, values: [f32; 3]) -> u32 {
        let at = e.mem.alloc(16);
        for (index, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * index as u32, *value);
        }
        at
    }

    fn read_weights(e: &Engine, at: u32) -> [f32; 3] {
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    fn run(e: &mut Engine, values: [f32; 3]) -> (i32, [f32; 3]) {
        let vertices = corners(e);
        let point = e.mem.alloc(16);
        store_vec(e, point, [0.25, 0.25, 0.25, 0.0]);
        let at = weights(e, values);
        let result = e.call(0x00e4_b570, &args![point, vertices, at]).i32();
        (result, read_weights(e, at))
    }

    #[test]
    fn two_corners_are_rejected_and_the_first_positive_weight_is_returned() {
        let mut e = engine();
        // Round 0 rejects the second weight (negative difference), round 1 is skipped, round 2
        // rejects its first weight (non-negative difference).
        assert_eq!(run(&mut e, [0.5, 0.5, 0.5]), (0, [0.5, -1.0, -1.0]));
    }

    #[test]
    fn a_zero_weight_passes_but_is_not_positive() {
        let mut e = engine();
        assert_eq!(run(&mut e, [0.0, 0.5, 0.5]), (-1, [0.0, -1.0, -1.0]));
    }

    #[test]
    fn nan_weight_skips_its_pairs() {
        let mut e = engine();
        let (result, weights) = run(&mut e, [f32::NAN, 0.5, 0.5]);
        // Only round 1 (weights 1 and 2) runs; its difference is non-negative: weight 1 goes.
        assert_eq!(result, 2);
        assert!(weights[0].is_nan());
        assert_eq!(&weights[1..], &[-1.0, 0.5]);
    }

    #[test]
    fn negative_weights_skip_every_round() {
        let mut e = engine();
        assert_eq!(run(&mut e, [-1.0, 0.7, -1.0]), (1, [-1.0, 0.7, -1.0]));
        assert_eq!(run(&mut e, [-1.0, -2.0, 0.25]), (2, [-1.0, -2.0, 0.25]));
    }

    /// A segment record with `a = (0, 0, 0)` and `b = (2, 0, 0)` (and the second pair).
    fn segment(e: &mut Engine) -> u32 {
        let record = e.mem.alloc(0x60);
        store_vec(e, record, [0.0, 0.0, 0.0, 0.0]);
        store_vec(e, record + 0x10, [2.0, 0.0, 0.0, 0.0]);
        store_vec(e, record + 0x40, [5.0; 4]);
        store_vec(e, record + 0x50, [6.0; 4]);
        record
    }

    fn closest(e: &mut Engine, record: u32, mode: u32, p: Vec4) -> Vec4 {
        let point = e.mem.alloc(16);
        store_vec(e, point, p);
        let out = e.mem.alloc(16);
        e.call(0x00e4_b6e0, &args![record, out, mode, point]);
        load_vec(e, out)
    }

    #[test]
    fn point_beside_the_segment_gives_a_unit_vector_result() {
        let mut e = engine();
        let record = segment(&mut e);
        let outside = closest(&mut e, record, 0, [1.0, 1.0, 0.0, 0.0]);
        let flipped = closest(&mut e, record, 1, [1.0, 1.0, 0.0, 0.0]);
        // The estimate of 1 / sqrt(4) is within a few parts per thousand of 0.5.
        assert!((outside[1] + 2.0).abs() < 0.02, "{outside:?}");
        assert!((flipped[1] - 2.0).abs() < 0.02, "{flipped:?}");
        assert_eq!((outside[0], outside[2], outside[3]), (0.0, 0.0, 0.0));
        assert_eq!((flipped[0], flipped[2], flipped[3]), (0.0, 0.0, 0.0));
        // The record is untouched.
        assert_eq!(load_vec(&e, record), [0.0; 4]);
    }

    #[test]
    fn point_past_the_end_copies_the_second_end_over_the_first() {
        let mut e = engine();
        let record = segment(&mut e);
        let result = closest(&mut e, record, 0, [3.0, 1.0, 0.0, 0.0]);
        assert_eq!(result, [-1.0, -1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, record), [2.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, record + 0x40), [6.0; 4]);
        let record = segment(&mut e);
        let result = closest(&mut e, record, 1, [3.0, 1.0, 0.0, 0.0]);
        assert_eq!(result, [1.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn point_before_the_start_uses_the_first_end() {
        let mut e = engine();
        let record = segment(&mut e);
        assert_eq!(
            closest(&mut e, record, 0, [-1.0, 1.0, 0.0, 0.0]),
            [1.0, -1.0, 0.0, 0.0]
        );
        assert_eq!(
            closest(&mut e, record, 1, [-1.0, 1.0, 0.0, 0.0]),
            [-1.0, 1.0, 0.0, 0.0]
        );
        assert_eq!(load_vec(&e, record + 0x40), [5.0; 4]);
    }
}

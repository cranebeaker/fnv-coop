//! `Havok SDK/run_00ddd620` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Small triangle helpers. The reciprocal square roots go through the host's `RSQRTSS` estimate
//! (the same estimate `refined_inverse_length` refines once, as the code does); estimates differ
//! slightly between processors, so results are only expected to agree to one ULP.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkgeometryprocessing::{
    compare_flags, load_vec4, mul4, refined_inverse_length, rotate_c9, rotate_d2, store_vec4, sub4,
    sum3, test_parity_even,
};

/// Runs `f` on an exe-shaped frame: a function that aligns `ESP` (`AND ESP,0xfffffff0`) and
/// then reserves `below` bytes (its `SUB ESP` and pushes) has its frame `below` bytes under a
/// 16-byte boundary; `f` gets the frame's lowest address.
fn with_aligned_frame<R>(e: &mut Engine, below: u32, f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    e.with_stack(below + 0x10, |e, raw| {
        let boundary = (raw.addr() + below + 0xf) & !0xf;
        f(e, boundary - below)
    })
}

/// The frame base [`with_aligned_frame`] picks for a block at `raw` (tests).
#[cfg(test)]
fn aligned_frame_at(raw: u32, below: u32) -> u32 {
    ((raw + below + 0xf) & !0xf) - below
}

/// The exe's `float` zero (`01011d78`), `3.0` (`01017718`) and `0.5` (`01016248`).
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
/// `hkMath_atan2fApproximation` (Xbox PDB, `00cbff80`, cdecl: two `float`s as words); answers a
/// `float` in `ST0`.
const ATAN2_APPROXIMATION: u32 = 0x00cb_ff80;

/// The normalizing factor of the code (`refined_inverse_length`) for a squared length.
fn inverse_length(e: &Engine, squared: f32) -> f32 {
    let zero = f32::from_bits(e.mem.u32(ZERO));
    let half = f32::from_bits(e.mem.u32(HALF));
    let three = f32::from_bits(e.mem.u32(THREE));
    refined_inverse_length(squared, zero, half, three)
}

// Translated from 00ddd620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `a`, `b`, `c` (16-byte vectors), `out`). Writes at `out` the
/// unit normal of the triangle `a`, `b`, `c`: with `ab = b - a` and `bc = c - b` the cross
/// product `n = rot_c9(ab) * rot_d2(bc) - rot_d2(ab) * rot_c9(bc)` scaled in all four lanes by
/// the refined reciprocal square root of `n . n` (lanes 0 to 2 summed in the code's order; the
/// factor is 0 when that is 0).
pub fn fn_00ddd620(e: &mut Engine, a: u32, b: u32, c: u32, out: u32) {
    let vb = load_vec4(e, b);
    let va = load_vec4(e, a);
    let ab = sub4(vb, va);
    let vc = load_vec4(e, c);
    let bc = sub4(vc, vb);
    let n = sub4(
        mul4(rotate_c9(ab), rotate_d2(bc)),
        mul4(rotate_d2(ab), rotate_c9(bc)),
    );
    let squared = sum3(mul4(n, n));
    let factor = inverse_length(e, squared);
    store_vec4(e, out, mul4([factor; 4], n));
}

// Translated from 00ddd6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `base` (three 16-byte vectors), `other` (three 16-byte
/// vectors), `index`, `out_vectors`, `out_indices`). Answers a count.
///
/// The first three vectors at `out_vectors` become the vectors of `base` numbered `index % 3`,
/// `(index + 1) % 3` and `(index + 2) % 3` (C remainder: a negative `index` reads before
/// `base`). The count starts at 3. Each of the three vectors of `other` is compared with
/// the first two output vectors (squared distance, lanes 0 to 2 summed in the code's order, equal
/// to 0.0 exactly; a NaN is not equal): when it matches output `j` the word at
/// `out_indices[j]` becomes the number of the `other` vector, otherwise the vector is appended
/// after the outputs written so far and the count grows by one.
pub fn fn_00ddd6f0(
    e: &mut Engine,
    base: u32,
    other: u32,
    index: i32,
    out_vectors: u32,
    out_indices: u32,
) -> u32 {
    for (slot, step) in [0i32, 1, 2].into_iter().enumerate() {
        let number = index.wrapping_add(step).wrapping_rem(3);
        let from = base.wrapping_add((number as u32).wrapping_shl(4));
        for lane in 0..4 {
            let word = e.mem.u32(from + 4 * lane);
            e.mem
                .set_u32(out_vectors + 0x10 * slot as u32 + 4 * lane, word);
        }
    }
    let mut count = 3u32;
    let mut cursor = out_vectors + 0x30;
    let mut at = other;
    for number in 0..3u32 {
        let vector = load_vec4(e, at);
        let mut matched = false;
        for slot in 0..2u32 {
            let output = load_vec4(e, out_vectors + 0x10 * slot);
            let difference = sub4(output, vector);
            let squared = sum3(mul4(difference, difference));
            let flags = compare_flags(0.0, x87_m32(squared.to_bits()));
            if !test_parity_even(flags, 0x44) {
                e.mem.set_u32(out_indices + 4 * slot, number);
                matched = true;
                break;
            }
        }
        if !matched {
            count += 1;
            store_vec4(e, cursor, vector);
            cursor += 0x10;
        }
        at += 0x10;
    }
    count
}

// Translated from 00ddd7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `points` (four 16-byte vectors `p0` to `p3`), `out_a`,
/// `out_b` (pointers to `float`s)). Answers a `float` in `ST0`.
///
/// With `n1` the unit normal of `(p0, p1, p2)` and `n2` that of `(p1, p0, p3)` (`00ddd620`) and
/// `h` the unit vector along `p1 - p0` (refined reciprocal square root of its squared length, all
/// four lanes scaled), it stores at `out_a` the lanes 0 to 2 summed (in the code's order) of
/// `(rot_d2(n1) * rot_c9(h) - rot_c9(n1) * rot_d2(h)) * n2` and at `out_b` those of `n2 * n1`
/// (both through `FLD`/`FSTP`, so quiet), and answers `hkMath_atan2fApproximation(*out_a,
/// dot)` (`00cbff80`): the two `float`s are passed as the words they have in memory.
pub fn fn_00ddd7e0(e: &mut Engine, points: u32, out_a: u32, out_b: u32) -> f64 {
    // The exe aligns ESP, then `SUB ESP,0x28` and two pushes: a 0x30-byte frame, the two normals
    // at +0x10 and +0x20 (00ddd7ee, 00ddd801).
    with_aligned_frame(e, 0x30, |e, frame| {
        let p0 = points;
        let p1 = points + 0x10;
        let first = frame + 0x10;
        let second = frame + 0x20;
        fn_00ddd620(e, p0, p1, points + 0x20, first);
        fn_00ddd620(e, p1, p0, points + 0x30, second);
        let from = load_vec4(e, p0);
        let to = load_vec4(e, p1);
        let edge = sub4(to, from);
        let squared = sum3(mul4(edge, edge));
        let factor = inverse_length(e, squared);
        let along = mul4([factor; 4], edge);
        let n1 = load_vec4(e, first);
        let cross = sub4(
            mul4(rotate_d2(n1), rotate_c9(along)),
            mul4(rotate_c9(n1), rotate_d2(along)),
        );
        let n2 = load_vec4(e, second);
        let projected = sum3(mul4(cross, n2));
        e.mem.set_u32(out_a, x87_quiet(projected).to_bits());
        let dot = sum3(mul4(n2, n1));
        e.mem.set_u32(out_b, x87_quiet(dot).to_bits());
        let stored = x87_quiet_bits(e.mem.u32(out_a));
        e.call(
            ATAN2_APPROXIMATION,
            &args![stored, x87_quiet_bits(dot.to_bits())],
        )
        .f64()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ddd620, fn_00ddd620(u32, u32, u32, u32)),
        entry!(0x00ddd6f0, fn_00ddd6f0(u32, u32, i32, u32, u32) -> u32),
        entry!(0x00ddd7e0, fn_00ddd7e0(u32, u32, u32) -> f64),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkgeometryprocessing::Vec4;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_1000, 0x0101_7000, 0x0101_6000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
        e
    }

    fn vector(e: &mut Engine, lanes: Vec4) -> u32 {
        let at = e.mem.alloc(16);
        store_vec4(e, at, lanes);
        at
    }

    /// Two floats are within one unit in the last place of each other.
    fn within_one_ulp(a: f32, b: f32) -> bool {
        (a.to_bits() as i64 - b.to_bits() as i64).abs() <= 1
    }

    #[test]
    fn the_normal_of_a_triangle_has_unit_length() {
        let mut e = engine();
        let a = vector(&mut e, [0.0, 0.0, 0.0, 0.0]);
        let b = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let c = vector(&mut e, [0.0, 1.0, 0.0, 0.0]);
        let out = e.mem.alloc(16);
        e.call(0x00dd_d620, &args![a, b, c, out]);
        let n = load_vec4(&e, out);
        assert_eq!((n[0], n[1], n[3]), (0.0, 0.0, 0.0));
        assert!(within_one_ulp(n[2], 1.0), "{}", n[2]);
    }

    #[test]
    fn a_degenerate_triangle_has_a_zero_normal() {
        let mut e = engine();
        let a = vector(&mut e, [1.0, 2.0, 3.0, 0.0]);
        let out = e.mem.alloc(16);
        store_vec4(&mut e, out, [9.0; 4]);
        e.call(0x00dd_d620, &args![a, a, a, out]);
        assert_eq!(load_vec4(&e, out), [0.0; 4]);
    }

    fn corners(e: &mut Engine) -> u32 {
        // A block of five vectors; the base is the second, so the vector before it is mapped.
        let block = e.mem.alloc(0x50);
        let at = block + 0x10;
        store_vec4(e, block, [8.0, 8.0, 8.0, 8.0]);
        store_vec4(e, at, [1.0, 0.0, 0.0, 0.0]);
        store_vec4(e, at + 0x10, [0.0, 1.0, 0.0, 0.0]);
        store_vec4(e, at + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec4(e, at + 0x30, [7.0, 7.0, 7.0, 7.0]);
        at
    }

    #[test]
    fn matching_corners_are_noted_and_new_ones_appended() {
        let mut e = engine();
        let base = corners(&mut e);
        let other = e.mem.alloc(0x30);
        store_vec4(&mut e, other, [0.0, 1.0, 0.0, 0.0]);
        store_vec4(&mut e, other + 0x10, [5.0, 5.0, 5.0, 0.0]);
        store_vec4(&mut e, other + 0x20, [1.0, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x80);
        let indices = e.mem.alloc(8);
        e.mem.set_u32(indices, 0xaaaa);
        e.mem.set_u32(indices + 4, 0xbbbb);
        let count = e
            .call(0x00dd_d6f0, &args![base, other, 0i32, out, indices])
            .u32();
        assert_eq!(count, 4);
        assert_eq!(load_vec4(&e, out), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, out + 0x10), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, out + 0x20), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(load_vec4(&e, out + 0x30), [5.0, 5.0, 5.0, 0.0]);
        // `other[0]` is output 1 and `other[2]` is output 0.
        assert_eq!((e.mem.u32(indices), e.mem.u32(indices + 4)), (2, 0));
    }

    #[test]
    fn the_index_picks_the_rotation_and_a_negative_one_reads_before_the_base() {
        let mut e = engine();
        let base = corners(&mut e);
        let other = e.mem.alloc(0x30);
        for k in 0..3 {
            store_vec4(&mut e, other + 0x10 * k, [9.0, 9.0, 9.0, 9.0]);
        }
        let out = e.mem.alloc(0x80);
        let indices = e.mem.alloc(8);
        // 4 % 3 = 1: the corners in the order 1, 2, 0.
        assert_eq!(
            e.call(0x00dd_d6f0, &args![base, other, 4i32, out, indices])
                .u32(),
            6
        );
        assert_eq!(load_vec4(&e, out), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, out + 0x10), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(load_vec4(&e, out + 0x20), [1.0, 0.0, 0.0, 0.0]);
        // -1 % 3 = -1: the vector before the base, then corners 0 and 1.
        e.call(0x00dd_d6f0, &args![base, other, -1i32, out, indices]);
        assert_eq!(load_vec4(&e, out), [8.0; 4]);
        assert_eq!(load_vec4(&e, out + 0x10), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, out + 0x20), [0.0, 1.0, 0.0, 0.0]);
    }

    #[test]
    fn the_angle_comes_from_the_atan_approximation_of_the_two_dot_products() {
        let mut e = engine();
        let points = e.mem.alloc(0x40);
        for (k, v) in [
            [0.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ]
        .into_iter()
        .enumerate()
        {
            store_vec4(&mut e, points + 0x10 * k as u32, v);
        }
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let seen = Rc::clone(&log);
        e.register_double(ATAN2_APPROXIMATION, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret {
                st0: 0.5,
                ..Ret::default()
            }
        });
        let out = e.mem.alloc(8);
        let result = e.call(0x00dd_d7e0, &args![points, out, out + 4]).f64();
        assert_eq!(result, 0.5);
        // n1 = (0, 0, 1), n2 = (0, 1, 0), h = (1, 0, 0): the cross term is -(n1 x h) . n2 = -1
        // and n2 . n1 = 0.
        let a = e.mem.f32(out);
        let b = e.mem.f32(out + 4);
        assert!((a + 1.0).abs() < 1e-5, "{a}");
        assert_eq!(b, 0.0);
        assert_eq!(
            *log.borrow(),
            vec![vec![e.mem.u32(out), e.mem.u32(out + 4)]]
        );
    }

    #[test]
    fn the_two_normals_sit_at_0x10_and_0x20_of_the_aligned_frame() {
        let mut e = engine();
        let points = e.mem.alloc(0x40);
        for (k, v) in [
            [0.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ]
        .into_iter()
        .enumerate()
        {
            store_vec4(&mut e, points + 0x10 * k as u32, v);
        }
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let seen = Rc::clone(&log);
        e.register_double(ATAN2_APPROXIMATION, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret {
                st0: 0.5,
                ..Ret::default()
            }
        });
        let out = e.mem.alloc(8);
        // The 0x40-byte local `with_aligned_frame` takes; its words stay on the thread's stack
        // after the call returns (only the next local there zeroes them).
        let raw = e.next_stack_local(0x40);
        e.call(0x00dd_d7e0, &args![points, out, out + 4]);
        let frame = aligned_frame_at(raw, 0x30);
        // Unit normals, to the processor's `RSQRTSS` estimate (one ULP, see the module docs).
        let first = load_vec4(&e, frame + 0x10)[2];
        let second = load_vec4(&e, frame + 0x20)[1];
        assert!(within_one_ulp(first, 1.0), "{first}");
        assert!(within_one_ulp(second, 1.0), "{second}");
    }
}

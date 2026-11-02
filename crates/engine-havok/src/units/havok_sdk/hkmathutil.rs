//! `Havok SDK/hkmathutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! SSE code (`MULPS`, `ADDPS`, `SUBPS`, `ANDPS`, `MAXPS`, `CMPLEPS`) computes lane by lane in
//! `f32`; x87 code rounds at each `float` store (the divisions in double precision first,
//! as the 53-bit control word of the process does). `RSQRTSS` is the host CPU's estimate.

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkdebugdisplay::{cross_rows, mul_lanes};
use super::hkpsphereshape::{
    copy_vec, inverse_length, load_bits, load_vec, maxps, store_vec, sum3, zip_with, Vec4,
    HALF_F32, ZERO_F32,
};

layout! {
    /// `hkMathUtil::Decomposition` (Xbox PDB), as far as `decompose4x4ColTransform` fills it:
    /// the translation (`+0`), the rotation quaternion (`+0x10`), the scale (`+0x20`), three
    /// flags, the scale-and-skew matrix (`+0x40`), the matrix of the skew without the scale
    /// (`+0x80`) and the rotation matrix (`+0xc0`); 0x100 bytes.
    pub struct HkMathUtilDecomposition: 0x100 {
        /// Whether the scale differs from one by more than the tolerance.
        0x30 hasScale: bool,
        /// Whether the skew matrix differs from the identity by more than the tolerance.
        0x31 hasSkew: bool,
        /// Whether the rotation had to be flipped (a negative determinant).
        0x32 flips: bool,
    }
}

/// The routines the decomposition calls, all unnamed in the engine map except where the Xbox
/// PDB or the repository's other units name them: `hkTransform::set4x4ColumnMajor`-like copy
/// of a 4x4 matrix (`00cb20b0`, `this` = the 0x40-byte result, one argument: the matrix),
/// the 4x4 copy (`00de9080`, `this`, `(matrix)`) and the one that closes it with the
/// `0, 0, 0, 1` last row (`00de8f10`, `this` only), `hkMatrix3::invert` (`00cd4f10`, `this`,
/// `(epsilon float)`), `hkMatrix3::transpose` (`00cd4b60`, `this` only), `hkMatrix3::add`
/// (`00cd5140`, `this`, `(other)`), `hkMatrix3::setMul` by a scalar vector (`00cd4d40`,
/// `this`, `(scalar vector, matrix)`), `hkMatrix3::isApproximatelyEqual` (`00cd4e00`, `this`,
/// `(result byte, other, epsilon float)`, returns the result pointer), `hkQuaternion::set`
/// (`00cb26e0`, `this`, `(matrix)`), a quaternion step (`005611c0`, `this` only), the rotation
/// from a quaternion (`00cb2d90`, `this` = the matrix, `(quaternion)`), `hkTransform::
/// setInverse` (`00cb21e0`, `this`, `(transform)`) and the 4x4 product (`00de8da0`, `this`,
/// `(left, right)`).
const SET_4X4_COLUMN_MAJOR: u32 = 0x00cb_20b0;
const MATRIX4_COPY: u32 = 0x00de_9080;
const MATRIX4_CLOSE: u32 = 0x00de_8f10;
const MATRIX3_INVERT: u32 = 0x00cd_4f10;
const MATRIX3_TRANSPOSE: u32 = 0x00cd_4b60;
const MATRIX3_ADD: u32 = 0x00cd_5140;
const MATRIX3_SET_MUL_SCALAR: u32 = 0x00cd_4d40;
const MATRIX3_IS_APPROXIMATELY_EQUAL: u32 = 0x00cd_4e00;
const QUATERNION_SET: u32 = 0x00cb_26e0;
const QUATERNION_STEP: u32 = 0x0056_11c0;
const ROTATION_SET_FROM_QUATERNION: u32 = 0x00cb_2d90;
const TRANSFORM_SET_INVERSE: u32 = 0x00cb_21e0;
const MATRIX4_SET_MUL: u32 = 0x00de_8da0;
/// The floats it reads: the epsilon of the inversion and the convergence test (`0104da4c`), the
/// scale tolerance (`01017d00`), minus one and one (`01012054`, `01012068`), the absolute-value
/// mask (`01017d10`), the three-by-three of the refined root (`01017718`) and the four columns
/// of the 4x4 identity (`0126a3e0`, `0126a3f0`, `0126a400`, `0126a410`).
const EPSILON: u32 = 0x0104_da4c;
const TOLERANCE: u32 = 0x0101_7d00;
const MINUS_ONE: u32 = 0x0101_2054;
const ONE: u32 = 0x0101_2068;
const ABS_MASK: u32 = 0x0101_7d10;
const IDENTITY_COLUMNS: u32 = 0x0126_a3e0;

/// The most Newton steps the polar decomposition takes.
const MAXIMUM_STEPS: i32 = 0x1e;

/// Copies the three columns of a 3x3 matrix, 16 bytes each.
fn copy_columns(e: &mut Engine, from: u32, to: u32) {
    for column in 0..3 {
        copy_vec(e, from + 0x10 * column, to + 0x10 * column);
    }
}

/// `v * splat(inverse_length(|v|^2))` the way the exe normalises a column: the refined
/// reciprocal root of the three-lane sum (zero for a zero sum), multiplied into every lane.
fn normalised_column(e: &Engine, column: Vec4) -> Vec4 {
    let inverse = inverse_length(e, sum3(mul_lanes(column, column)));
    column.map(|v| inverse * v)
}

// Translated from 00d72e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkMathUtil::decompose4x4ColTransform` (Xbox PDB), cdecl, two stack arguments: the
/// column-major 4x4 matrix and the `Decomposition` to fill. The matrix is copied into an
/// `hkTransform` and a 4x4 matrix (kept for the end); the translation goes to `+0`. The
/// rotation part is made orthogonal by up to 30 Newton steps (`M = 0.5 * (M + (M^-1)^T)`, each
/// step inverting with the epsilon `0104da4c`, transposing, adding the previous matrix and
/// halving through `00cd4d40`; the loop ends when `00cd4e00` finds the new matrix equal to the
/// previous one, within the same epsilon; the 30th step is not tested), which is stored as
/// the rotation matrix at `+0xc0`. Its columns are normalised (refined reciprocal root);
/// when the determinant of the three (the shuffle product of the second and third columns
/// times the first, summed as `z + (y + x)`) is negative (ordered compare with zero) the
/// first is multiplied by `-1` and the flag at `+0x32` is set. The quaternion (`00cb26e0` on
/// the columns, then the step at `005611c0`) goes to `+0x10`; the rotation matrix made from
/// it and the translation make a transform whose inverse (`00cb21e0`) is closed as a 4x4
/// matrix (`00de9080`, `00de8f10`) and multiplied with the copy of the input (`00de8da0`) into
/// `+0x40`. Its diagonal is the scale (`+0x20`, with the float `01011d78` in lane 3; the flag
/// at `+0x30` says some component is not within `01017d00` of one); the inverse scale (x87
/// divisions) as a diagonal matrix times it gives the skew matrix at `+0x80`, whose maximum
/// difference from the identity columns `0126a3e0`.. decides the flag at `+0x31`.
pub fn hk_math_util_decompose_4x4_col_transform(
    e: &mut Engine,
    matrix: u32,
    out: Ptr<HkMathUtilDecomposition>,
) {
    let out_at = out.addr();
    e.with_stack(0x1d0, |e, block| {
        let frame = (block.addr() + 0xf) & !0xf;
        let (transform_at, copy_at) = (frame + 0x120, frame + 0x160);
        let current = frame + 0x70;
        let previous = frame + 0xa0;
        let inverse_at = frame + 0xe0;
        let sum_at = frame + 0x30;
        let scalar_at = frame + 0x20;
        let epsilon = x87_quiet(e.global::<f32>(EPSILON)).to_bits();
        e.call(SET_4X4_COLUMN_MAJOR, &args![transform_at, matrix]);
        e.call(MATRIX4_COPY, &args![copy_at, matrix]);
        e.call(MATRIX4_CLOSE, &args![copy_at]);
        copy_vec(e, transform_at + 0x30, out_at);
        copy_columns(e, transform_at, current);
        let half: f32 = e.global(HALF_F32);
        let mut steps = 0;
        loop {
            copy_columns(e, current, previous);
            copy_columns(e, current, inverse_at);
            e.call(MATRIX3_INVERT, &args![inverse_at, epsilon]);
            e.call(MATRIX3_TRANSPOSE, &args![inverse_at]);
            copy_columns(e, previous, sum_at);
            e.call(MATRIX3_ADD, &args![sum_at, inverse_at]);
            store_vec(e, scalar_at, [half, 0.0, 0.0, 0.0]);
            e.call(MATRIX3_SET_MUL_SCALAR, &args![current, scalar_at, sum_at]);
            steps += 1;
            if steps >= MAXIMUM_STEPS {
                break;
            }
            let result_at = frame + 0x1b;
            let result = e
                .call(
                    MATRIX3_IS_APPROXIMATELY_EQUAL,
                    &args![current, result_at, previous, epsilon],
                )
                .u32();
            if e.mem.u8(result) != 0 {
                break;
            }
        }
        copy_columns(e, current, out_at + 0xc0);
        let mut columns: [Vec4; 3] = std::array::from_fn(|index| {
            let raw = load_vec(e, out_at + 0xc0 + 0x10 * index as u32);
            normalised_column(e, raw)
        });
        let determinant = sum3(mul_lanes(cross_rows(columns[2], columns[1]), columns[0]));
        let negative = 0.0 > determinant;
        if negative {
            let minus_one: f32 = e.global(MINUS_ONE);
            columns[0] = columns[0].map(|v| minus_one * v);
        }
        e.set(out, HkMathUtilDecomposition::flips, negative);
        for (index, column) in columns.iter().enumerate() {
            store_vec(e, sum_at + 0x10 * index as u32, *column);
        }
        e.call(QUATERNION_SET, &args![scalar_at, sum_at]);
        copy_vec(e, scalar_at, out_at + 0x10);
        e.call(QUATERNION_STEP, &args![out_at + 0x10]);
        e.call(ROTATION_SET_FROM_QUATERNION, &args![sum_at, out_at + 0x10]);
        copy_vec(e, out_at, frame + 0x60);
        e.call(TRANSFORM_SET_INVERSE, &args![previous, sum_at]);
        e.call(MATRIX4_COPY, &args![inverse_at, previous]);
        e.call(MATRIX4_CLOSE, &args![inverse_at]);
        e.call(MATRIX4_SET_MUL, &args![out_at + 0x40, inverse_at, copy_at]);
        let zero: f32 = e.global(ZERO_F32);
        let scale = [
            e.mem.f32(out_at + 0x40),
            e.mem.f32(out_at + 0x54),
            e.mem.f32(out_at + 0x68),
            zero,
        ];
        store_vec(e, out_at + 0x20, scale);
        let tolerance: f32 = e.global(TOLERANCE);
        let one: f32 = e.global(ONE);
        let mask = load_bits(e, ABS_MASK);
        let absolute = |v: Vec4| -> Vec4 {
            std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() & mask[lane]))
        };
        let near_one = absolute(zip_with(scale, [one; 4], |a, b| a - b)).map(|v| v <= tolerance);
        let all_near = near_one[..3].iter().all(|near| *near);
        e.set(out, HkMathUtilDecomposition::hasScale, !all_near);
        // The inverse scale as a diagonal matrix (x87 divisions, rounded to float at the store).
        for row in 0..4 {
            store_vec(e, sum_at + 0x10 * row, [0.0; 4]);
        }
        for (index, component) in scale[..3].iter().enumerate() {
            let inverse = (1.0f64 / f64::from(x87_quiet(*component))) as f32;
            e.mem
                .set_f32(sum_at + 0x10 * index as u32 + 4 * index as u32, inverse);
        }
        e.mem.set_f32(sum_at + 0x30 + 12, 1.0);
        e.call(
            MATRIX4_SET_MUL,
            &args![out_at + 0x80, sum_at, out_at + 0x40],
        );
        let difference = |column: u32| -> Vec4 {
            let actual = load_vec(e, out_at + 0x80 + 0x10 * column);
            let wanted = load_vec(e, IDENTITY_COLUMNS + 0x10 * column);
            absolute(zip_with(actual, wanted, |a, b| a - b))
        };
        let widest_low = zip_with(difference(0), difference(1), maxps);
        let widest_high = zip_with(difference(2), difference(3), maxps);
        let widest = zip_with(widest_low, widest_high, maxps);
        let within = widest.map(|v| v <= tolerance);
        e.set(
            out,
            HkMathUtilDecomposition::hasSkew,
            !within.iter().all(|ok| *ok),
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d7_2e90,
        hk_math_util_decompose_4x4_col_transform(u32, Ptr<HkMathUtilDecomposition>)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use super::super::hkpsphereshape::THREE_F32;

    fn aligned(e: &mut Engine, size: u32) -> u32 {
        (e.mem.alloc(size + 16) + 15) & !15
    }

    /// What the callee doubles saw.
    #[derive(Default)]
    struct Log {
        quaternion_input: Vec<[Vec4; 3]>,
        rotation_from: Vec<(u32, Vec4)>,
        inverse_scale: Vec<[Vec4; 4]>,
        set_mul_calls: usize,
        equal_calls: usize,
    }

    struct Setup {
        scale: [f32; 3],
        skew: f32,
        equal_from: usize,
        shrink: f32,
    }

    const IDENTITY: [Vec4; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    fn copy_four(e: &mut Engine, from: u32, to: u32) {
        for column in 0..4 {
            copy_vec(e, from + 0x10 * column, to + 0x10 * column);
        }
    }

    /// Decomposes `columns` with doubles for every callee: copies copy, `invert` multiplies by
    /// `shrink`, `add` and the scalar `setMul` do their arithmetic, the equality test is true
    /// from its `equal_from`-th call on, `hkQuaternion::set` makes `(1, 2, 3, 4)`, and the 4x4
    /// product gives the diagonal `scale` the first time and the identity (plus `skew` in the
    /// second column's first lane) the second time.
    fn decompose(columns: [Vec4; 4], setup: Setup) -> (Engine, u32, Rc<RefCell<Log>>) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(EPSILON, 1.192_092_9e-7f32);
        e.set_global(TOLERANCE, 0.001f32);
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(ONE, 1.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        for lane in 0..4 {
            e.set_global(ABS_MASK + 4 * lane, 0x7fff_ffffu32);
        }
        for (column, value) in IDENTITY.into_iter().enumerate() {
            store_vec(&mut e, IDENTITY_COLUMNS + 0x10 * column as u32, value);
        }
        let log = Rc::new(RefCell::new(Log::default()));
        let out = aligned(&mut e, 0x100);
        let matrix = aligned(&mut e, 0x40);
        for (index, column) in columns.into_iter().enumerate() {
            store_vec(&mut e, matrix + 0x10 * index as u32, column);
        }
        e.register(SET_4X4_COLUMN_MAJOR, |e, a| {
            copy_four(e, a[1], a[0]);
            Ret::default()
        });
        e.register(MATRIX4_COPY, |e, a| {
            copy_four(e, a[1], a[0]);
            Ret::default()
        });
        e.register(MATRIX4_CLOSE, |e, a| {
            e.mem.set_u32(a[0] + 0x2c, 0);
            e.mem.set_u32(a[0] + 0x1c, 0);
            e.mem.set_u32(a[0] + 0xc, 0);
            e.mem.set_f32(a[0] + 0x3c, 1.0);
            Ret::default()
        });
        let shrink = setup.shrink;
        e.register_double(MATRIX3_INVERT, move |e, a| {
            for column in 0..3 {
                let v = load_vec(e, a[0] + 0x10 * column);
                store_vec(e, a[0] + 0x10 * column, v.map(|x| x * shrink));
            }
            Ret::default()
        });
        e.register(MATRIX3_TRANSPOSE, |_, _| Ret::default());
        e.register(MATRIX3_ADD, |e, a| {
            for column in 0..3 {
                let sum = zip_with(
                    load_vec(e, a[0] + 0x10 * column),
                    load_vec(e, a[1] + 0x10 * column),
                    |x, y| x + y,
                );
                store_vec(e, a[0] + 0x10 * column, sum);
            }
            Ret::default()
        });
        let counted = Rc::clone(&log);
        e.register_double(MATRIX3_SET_MUL_SCALAR, move |e, a| {
            counted.borrow_mut().set_mul_calls += 1;
            let scalar = e.mem.f32(a[1]);
            for column in 0..3 {
                let v = load_vec(e, a[2] + 0x10 * column);
                store_vec(e, a[0] + 0x10 * column, v.map(|x| scalar * x));
            }
            Ret::default()
        });
        let calls = Rc::new(Cell::new(0usize));
        let compared = Rc::clone(&log);
        let equal_from = setup.equal_from;
        e.register_double(MATRIX3_IS_APPROXIMATELY_EQUAL, move |e, a| {
            compared.borrow_mut().equal_calls += 1;
            calls.set(calls.get() + 1);
            e.mem.set_u8(a[1], u8::from(calls.get() >= equal_from));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(QUATERNION_SET, move |e, a| {
            let input: [Vec4; 3] = std::array::from_fn(|c| load_vec(e, a[1] + 0x10 * c as u32));
            seen.borrow_mut().quaternion_input.push(input);
            store_vec(e, a[0], [1.0, 2.0, 3.0, 4.0]);
            Ret::default()
        });
        e.register(QUATERNION_STEP, |_, _| Ret::default());
        let rotated = Rc::clone(&log);
        e.register_double(ROTATION_SET_FROM_QUATERNION, move |e, a| {
            let q = load_vec(e, a[1]);
            rotated.borrow_mut().rotation_from.push((a[0], q));
            Ret::default()
        });
        e.register(TRANSFORM_SET_INVERSE, |_, _| Ret::default());
        let scaled = Rc::clone(&log);
        let (scale, skew) = (setup.scale, setup.skew);
        e.register_double(MATRIX4_SET_MUL, move |e, a| {
            if a[0] == out + 0x40 {
                let diagonal = [
                    [scale[0], 0.0, 0.0, 0.0],
                    [0.0, scale[1], 0.0, 0.0],
                    [0.0, 0.0, scale[2], 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ];
                for (column, value) in diagonal.into_iter().enumerate() {
                    store_vec(e, a[0] + 0x10 * column as u32, value);
                }
            } else {
                assert_eq!(a[0], out + 0x80);
                assert_eq!(a[2], out + 0x40);
                let inverse: [Vec4; 4] =
                    std::array::from_fn(|c| load_vec(e, a[1] + 0x10 * c as u32));
                scaled.borrow_mut().inverse_scale.push(inverse);
                let mut skewed = IDENTITY;
                skewed[1][0] += skew;
                for (column, value) in skewed.into_iter().enumerate() {
                    store_vec(e, a[0] + 0x10 * column as u32, value);
                }
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00d7_2e90, &args![matrix, out]);
        (e, out, log)
    }

    fn setup(scale: [f32; 3]) -> Setup {
        Setup {
            scale,
            skew: 0.0,
            equal_from: 1,
            shrink: 1.0,
        }
    }

    const SCALED: [Vec4; 4] = [
        [2.0, 0.0, 0.0, 0.0],
        [0.0, 3.0, 0.0, 0.0],
        [0.0, 0.0, 4.0, 0.0],
        [5.0, 6.0, 7.0, 1.0],
    ];

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(target, _)| *target == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn close(got: Vec4, want: Vec4) {
        for lane in 0..4 {
            assert!(
                (got[lane] - want[lane]).abs() < 1e-5,
                "lane {lane}: {got:?} against {want:?}"
            );
        }
    }

    #[test]
    fn a_scaled_transform_is_decomposed_into_its_parts() {
        let (e, out, log) = decompose(SCALED, setup([2.0, 3.0, 4.0]));
        let out_ptr: Ptr<HkMathUtilDecomposition> = Ptr::new(out);
        assert_eq!(load_vec(&e, out), [5.0, 6.0, 7.0, 1.0]);
        assert_eq!(load_vec(&e, out + 0x10), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, out + 0x20), [2.0, 3.0, 4.0, 0.0]);
        assert!(e.get(out_ptr, HkMathUtilDecomposition::hasScale));
        assert!(!e.get(out_ptr, HkMathUtilDecomposition::hasSkew));
        assert!(!e.get(out_ptr, HkMathUtilDecomposition::flips));
        // The rotation matrix is the polar iteration's result (unchanged by these doubles).
        assert_eq!(load_vec(&e, out + 0xc0), SCALED[0]);
        assert_eq!(load_vec(&e, out + 0xd0), SCALED[1]);
        assert_eq!(load_vec(&e, out + 0xe0), SCALED[2]);
        let log = log.borrow();
        // The quaternion is made from the normalised columns.
        assert_eq!(log.quaternion_input.len(), 1);
        for (column, want) in log.quaternion_input[0].iter().zip(IDENTITY) {
            close(*column, want);
        }
        assert_eq!(
            log.rotation_from,
            vec![(log.rotation_from[0].0, [1.0, 2.0, 3.0, 4.0])]
        );
        // The inverse scale is the reciprocal diagonal.
        assert_eq!(log.inverse_scale.len(), 1);
        close(log.inverse_scale[0][0], [0.5, 0.0, 0.0, 0.0]);
        close(log.inverse_scale[0][1], [0.0, 1.0 / 3.0, 0.0, 0.0]);
        close(log.inverse_scale[0][2], [0.0, 0.0, 0.25, 0.0]);
        close(log.inverse_scale[0][3], [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(calls_to(&e, QUATERNION_STEP), vec![vec![out + 0x10]]);
    }

    #[test]
    fn a_negative_determinant_flips_the_first_column_and_sets_the_flag() {
        let mut columns = SCALED;
        columns[0] = [-2.0, 0.0, 0.0, 0.0];
        let (e, out, log) = decompose(columns, setup([2.0, 3.0, 4.0]));
        assert!(e.get(
            Ptr::<HkMathUtilDecomposition>::new(out),
            HkMathUtilDecomposition::flips
        ));
        let log = log.borrow();
        close(log.quaternion_input[0][0], [1.0, 0.0, 0.0, 0.0]);
        close(log.quaternion_input[0][1], IDENTITY[1]);
        close(log.quaternion_input[0][2], IDENTITY[2]);
    }

    #[test]
    fn a_scale_within_the_tolerance_of_one_has_no_scale_flag() {
        let (e, out, _) = decompose(SCALED, setup([1.0005, 0.9995, 1.0]));
        let out_ptr: Ptr<HkMathUtilDecomposition> = Ptr::new(out);
        assert!(!e.get(out_ptr, HkMathUtilDecomposition::hasScale));
        let (e, out, _) = decompose(SCALED, setup([1.0, 1.0, 1.01]));
        assert!(e.get(
            Ptr::<HkMathUtilDecomposition>::new(out),
            HkMathUtilDecomposition::hasScale
        ));
    }

    #[test]
    fn a_skew_matrix_away_from_the_identity_sets_the_skew_flag() {
        let mut skewed = setup([1.0, 1.0, 1.0]);
        skewed.skew = 0.5;
        let (e, out, _) = decompose(SCALED, skewed);
        assert!(e.get(
            Ptr::<HkMathUtilDecomposition>::new(out),
            HkMathUtilDecomposition::hasSkew
        ));
        let mut slight = setup([1.0, 1.0, 1.0]);
        slight.skew = 0.0005;
        let (e, out, _) = decompose(SCALED, slight);
        assert!(!e.get(
            Ptr::<HkMathUtilDecomposition>::new(out),
            HkMathUtilDecomposition::hasSkew
        ));
    }

    #[test]
    fn the_polar_loop_runs_until_the_matrix_stops_changing() {
        let mut shrinking = setup([1.0, 1.0, 1.0]);
        shrinking.equal_from = 3;
        shrinking.shrink = 0.5;
        let (e, out, log) = decompose(SCALED, shrinking);
        // Each step makes M = 0.5 * (M + 0.5 * M) = 0.75 * M; three steps.
        assert_eq!(log.borrow().set_mul_calls, 3);
        assert_eq!(log.borrow().equal_calls, 3);
        assert_eq!(load_vec(&e, out + 0xc0)[0], 2.0 * 0.75 * 0.75 * 0.75);
        let epsilon = 1.192_092_9e-7f32.to_bits();
        let inverted = calls_to(&e, MATRIX3_INVERT);
        assert_eq!(inverted.len(), 3);
        assert_eq!(inverted[0][1], epsilon);
        let compared = calls_to(&e, MATRIX3_IS_APPROXIMATELY_EQUAL);
        assert_eq!(compared[0][3], epsilon);
        // The matrix is compared with the one it replaced.
        assert_ne!(compared[0][0], compared[0][2]);
    }

    #[test]
    fn the_polar_loop_takes_at_most_thirty_steps() {
        let mut never = setup([1.0, 1.0, 1.0]);
        never.equal_from = usize::MAX;
        let (_, _, log) = decompose(SCALED, never);
        assert_eq!(log.borrow().set_mul_calls, 30);
        assert_eq!(log.borrow().equal_calls, 29);
    }
}

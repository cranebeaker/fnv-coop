//! `Havok SDK/hktransform.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkTransform` is four 16-byte columns: the rotation (`hkRotation`/`hkMatrix3`, columns at
//! +0, +0x10, +0x20) and the translation at +0x30. The SSE code multiplies with `MULPS` of a
//! broadcast lane by a column and adds the products in the order the instructions add them
//! (`f32`, all four lanes). The inverse rotation is the compiler's transpose of the three
//! columns, whose rows leave `column2.y` (rows 0) and `column2.w` (rows 1 and 2) in the
//! fourth lane (the `SHUFPS` immediates `0xd4` and `0xe4`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkMatrix3::isApproximatelyEqual` (Xbox PDB, `00cd4e00`, `this`, `(out hkBool, other,
/// epsilon)`, returns the `hkBool` pointer) and `hkMatrix3::setTranspose` (`00cd4ba0`,
/// `this`, `(source)`).
const MATRIX3_IS_APPROXIMATELY_EQUAL: u32 = 0x00cd_4e00;
const MATRIX3_SET_TRANSPOSE: u32 = 0x00cd_4ba0;
/// The SSE sign-clearing mask (four words, `0x7fffffff`) and sign-flipping mask (four words,
/// `0x80000000`).
const ABS_MASK: u32 = 0x0101_7d10;
const SIGN_MASK: u32 = 0x010c_44d0;

type Lanes = [f32; 4];

fn load_lanes(e: &Engine, addr: u32) -> Lanes {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_lanes(e: &mut Engine, addr: u32, v: Lanes) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, *value);
    }
}

/// The three rows the compiler's transpose of the columns `c0`, `c1`, `c2` leaves in
/// registers.
fn transposed_rows(c0: Lanes, c1: Lanes, c2: Lanes) -> [Lanes; 3] {
    [
        [c0[0], c1[0], c2[0], c2[1]],
        [c0[1], c1[1], c2[1], c2[3]],
        [c0[2], c1[2], c2[2], c2[3]],
    ]
}

/// `(y * row1 + x * row0) + z * row2` (the order of the inverse code).
fn combine_rows(v: Lanes, rows: &[Lanes; 3]) -> Lanes {
    std::array::from_fn(|lane| (v[1] * rows[1][lane] + v[0] * rows[0][lane]) + v[2] * rows[2][lane])
}

/// The rows of the matrix whose columns are at `base`, `base + 0x10`, `base + 0x20`.
fn transposed_rows_at(e: &Engine, base: u32) -> [Lanes; 3] {
    transposed_rows(
        load_lanes(e, base),
        load_lanes(e, base + 0x10),
        load_lanes(e, base + 0x20),
    )
}

// Translated from 00cb2160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTransform::isApproximatelyEqual` (Xbox PDB, returns the `hkBool` pointer `out`): true
/// when the rotations are approximately equal (`hkMatrix3::isApproximatelyEqual`, passed
/// the epsilon through the x87) and `|this.translation - other.translation| <= epsilon` in
/// lanes 0..2 (`CMPLEPS`, false for a NaN; the absolute value is the `ANDPS` with the mask
/// at `01017d10`).
pub fn hk_transform_is_approximately_equal(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    other: Ptr,
    epsilon: f32,
) -> Ptr {
    let rotation_equal = e.with_stack(4, |e, flag| {
        let result = e
            .call(
                MATRIX3_IS_APPROXIMATELY_EQUAL,
                &args![this, flag, other, x87_quiet(epsilon)],
            )
            .u32();
        e.mem.u8(result) != 0
    });
    let mut equal = false;
    if rotation_equal {
        let mine = load_lanes(e, this.addr() + 0x30);
        let theirs = load_lanes(e, other.addr() + 0x30);
        equal = (0..3).all(|lane| {
            let mask = e.mem.u32(ABS_MASK + 4 * lane as u32);
            let difference = mine[lane] - theirs[lane];
            f32::from_bits(difference.to_bits() & mask) <= epsilon
        });
    }
    e.mem.set_u8(out.addr(), u8::from(equal));
    out
}

// Translated from 00cb21e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTransform::setInverse` (Xbox PDB): the rotation becomes the transpose of `other`'s
/// (`hkMatrix3::setTranspose`, `00cd4ba0`), the translation `transposed rotation *
/// (-other.translation)` (the sign flip is the `XORPS` with the mask at `010c44d0`).
pub fn hk_transform_set_inverse(e: &mut Engine, this: Ptr, other: Ptr) {
    e.call(MATRIX3_SET_TRANSPOSE, &args![this, other]);
    let translation = load_lanes(e, other.addr() + 0x30);
    let negated: Lanes = std::array::from_fn(|lane| {
        f32::from_bits(translation[lane].to_bits() ^ e.mem.u32(SIGN_MASK + 4 * lane as u32))
    });
    let (c0, c1, c2) = (
        load_lanes(e, this.addr()),
        load_lanes(e, this.addr() + 0x10),
        load_lanes(e, this.addr() + 0x20),
    );
    let result: Lanes = std::array::from_fn(|lane| {
        (negated[1] * c1[lane] + negated[0] * c0[lane]) + negated[2] * c2[lane]
    });
    store_lanes(e, this.addr() + 0x30, result);
}

// Translated from 00cb2240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTransform::setMul` (Xbox PDB): `this = a * b`. The rotation columns of `a` are loaded
/// first; then for the four columns of `b`, last to first, `this.column = (x * a0 + y * a1)
/// + z * a2`; finally `this.translation = a.translation + this.translation` (read after the
/// loop, so `this == a` adds the product to itself).
pub fn hk_transform_set_mul(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr) {
    let columns = [
        load_lanes(e, a.addr()),
        load_lanes(e, a.addr() + 0x10),
        load_lanes(e, a.addr() + 0x20),
    ];
    for column in (0..4u32).rev() {
        let v = load_lanes(e, b.addr() + 0x10 * column);
        let result: Lanes = std::array::from_fn(|lane| {
            (v[0] * columns[0][lane] + v[1] * columns[1][lane]) + v[2] * columns[2][lane]
        });
        store_lanes(e, this.addr() + 0x10 * column, result);
    }
    let a_translation = load_lanes(e, a.addr() + 0x30);
    let product = load_lanes(e, this.addr() + 0x30);
    let sum: Lanes = std::array::from_fn(|lane| a_translation[lane] + product[lane]);
    store_lanes(e, this.addr() + 0x30, sum);
}

// Translated from 00cb22f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTransform::setMulInverseMul` (Xbox PDB): `this = inverse(a) * b`. The rotation columns
/// of `this` are the columns of `b` times the transposed rotation of `a` (`(y * row1 + x *
/// row0) + z * row2`, columns 0 to 2); the translation is the same product of `b.translation
/// - a.translation`, with the rows taken again from `a` after the columns were written.
pub fn hk_transform_set_mul_inverse_mul(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr) {
    let rows = transposed_rows_at(e, a.addr());
    for column in 0..3u32 {
        let v = load_lanes(e, b.addr() + 0x10 * column);
        store_lanes(e, this.addr() + 0x10 * column, combine_rows(v, &rows));
    }
    let b_translation = load_lanes(e, b.addr() + 0x30);
    let a_translation = load_lanes(e, a.addr() + 0x30);
    let difference: Lanes = std::array::from_fn(|lane| b_translation[lane] - a_translation[lane]);
    let rows = transposed_rows_at(e, a.addr());
    store_lanes(e, this.addr() + 0x30, combine_rows(difference, &rows));
}

/// The conversion `00ce5300` (a jump to `00ce51f0`; thiscall on the source object, one stack
/// word: the `hkTransform` it writes).
const CONVERT_TO_TRANSFORM: u32 = 0x00ce_5300;
/// The size of an `hkTransform` the functions keep on their stack.
const TRANSFORM_SIZE: u32 = 0x40;

// Translated from 00cb22c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, thiscall, two stack words `(source, other)`: converts `source`
/// into a temporary `hkTransform` (`00ce5300`, the source is its `this`) and sets `this =
/// temporary * other` (`setMul`).
pub fn fn_00cb22c0(e: &mut Engine, this: Ptr, source: Ptr, other: Ptr) {
    e.with_stack(TRANSFORM_SIZE, |e, temporary| {
        e.call(CONVERT_TO_TRANSFORM, &args![source, temporary]);
        hk_transform_set_mul(e, this, temporary, other);
    });
}

// Translated from 00cb23e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTransform::setMulEq` (engine map name), thiscall, one stack word `other`: `this = this *
/// other`. The four columns of `this` are copied to a temporary first (so the product reads
/// the old values) and `setMul(this, temporary, other)` runs.
pub fn hk_transform_set_mul_eq(e: &mut Engine, this: Ptr, other: Ptr) {
    e.with_stack(TRANSFORM_SIZE, |e, copy| {
        for column in 0..4u32 {
            let lanes = load_lanes(e, this.addr() + 0x10 * column);
            store_lanes(e, copy.addr() + 0x10 * column, lanes);
        }
        hk_transform_set_mul(e, this, copy, other);
    });
}

// Translated from 00cb2420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkTransform::setMulMulInverse` (engine map name), thiscall, two stack words `(a, b)`:
/// `this = a * inverse(b)`: the inverse of `b` is built in a temporary (`setInverse`) and
/// `setMul(this, a, temporary)` runs.
pub fn hk_transform_set_mul_mul_inverse(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr) {
    e.with_stack(TRANSFORM_SIZE, |e, inverse| {
        hk_transform_set_inverse(e, inverse, b);
        hk_transform_set_mul(e, this, a, inverse);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cb_2160,
            hk_transform_is_approximately_equal(Ptr, Ptr, Ptr, f32) -> Ptr
        ),
        entry!(0x00cb_21e0, hk_transform_set_inverse(Ptr, Ptr)),
        entry!(0x00cb_2240, hk_transform_set_mul(Ptr, Ptr, Ptr)),
        entry!(0x00cb_22f0, hk_transform_set_mul_inverse_mul(Ptr, Ptr, Ptr)),
        entry!(0x00cb_22c0, fn_00cb22c0(Ptr, Ptr, Ptr)),
        entry!(0x00cb_23e0, hk_transform_set_mul_eq(Ptr, Ptr)),
        entry!(0x00cb_2420, hk_transform_set_mul_mul_inverse(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transform(e: &mut Engine, columns: [Lanes; 4]) -> u32 {
        let t = e.mem.alloc(0x40);
        for (i, c) in columns.iter().enumerate() {
            store_lanes(e, t + 16 * i as u32, *c);
        }
        t
    }

    fn columns_of(e: &Engine, t: u32) -> [Lanes; 4] {
        std::array::from_fn(|i| load_lanes(e, t + 16 * i as u32))
    }

    /// 90 degrees about z: x -> y, y -> -x, translation (1, 2, 3).
    const ROTATED: [Lanes; 4] = [
        [0.0, 1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [1.0, 2.0, 3.0, 1.0],
    ];

    #[test]
    fn set_mul_composes_rotation_and_translation() {
        let mut e = Engine::new();
        let a = transform(&mut e, ROTATED);
        let b = transform(
            &mut e,
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [10.0, 20.0, 30.0, 1.0],
            ],
        );
        let dest = e.mem.alloc(0x40);
        e.call(0x00cb_2240, &args![dest, a, b]);
        let got = columns_of(&e, dest);
        assert_eq!(got[..3], ROTATED[..3]);
        // a.rotation * (10, 20, 30) + a.translation = (-20, 10, 30) + (1, 2, 3); the fourth
        // lane is 1 * (w of each rotation column: 0) summed, plus a.translation.w
        assert_eq!(got[3], [-19.0, 12.0, 33.0, 1.0]);
    }

    #[test]
    fn set_mul_inverse_mul_undoes_the_first_transform() {
        let mut e = Engine::new();
        let a = transform(&mut e, ROTATED);
        // b = a: inverse(a) * a is the identity rotation and a zero translation
        let dest = e.mem.alloc(0x40);
        e.call(0x00cb_22f0, &args![dest, a, a]);
        let got = columns_of(&e, dest);
        assert_eq!(got[0][..3], [1.0, 0.0, 0.0]);
        assert_eq!(got[1][..3], [0.0, 1.0, 0.0]);
        assert_eq!(got[2][..3], [0.0, 0.0, 1.0]);
        assert_eq!(got[3][..3], [0.0, 0.0, 0.0]);
    }

    #[test]
    fn set_mul_inverse_mul_uses_the_transposed_rows_in_the_fourth_lane_too() {
        let mut e = Engine::new();
        let a = transform(
            &mut e,
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [0.5, 0.25, 0.125, 2.0],
            ],
        );
        let b = transform(
            &mut e,
            [
                [1.0, 1.0, 1.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [1.5, 1.25, 1.125, 3.0],
            ],
        );
        let dest = e.mem.alloc(0x40);
        e.call(0x00cb_22f0, &args![dest, a, b]);
        let rows = [
            [1.0f32, 5.0, 9.0, 10.0],
            [2.0, 6.0, 10.0, 12.0],
            [3.0, 7.0, 11.0, 12.0],
        ];
        let got = columns_of(&e, dest);
        let combine = |v: Lanes| -> Lanes {
            std::array::from_fn(|l| (v[1] * rows[1][l] + v[0] * rows[0][l]) + v[2] * rows[2][l])
        };
        assert_eq!(got[0], combine([1.0, 1.0, 1.0, 0.0]));
        assert_eq!(got[2], combine([0.0, 0.0, 2.0, 0.0]));
        assert_eq!(got[3], combine([1.0, 1.0, 1.0, 1.0]));
    }

    #[test]
    fn set_inverse_transposes_through_the_matrix_function_and_negates_the_translation() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        let other = transform(&mut e, ROTATED);
        let this = e.mem.alloc(0x40);
        // double for setTranspose: the real one is in another unit
        e.register(MATRIX3_SET_TRANSPOSE, |e, a| {
            for column in 0..3u32 {
                for row in 0..3u32 {
                    let word = e.mem.u32(a[1] + 0x10 * row + 4 * column);
                    e.mem.set_u32(a[0] + 0x10 * column + 4 * row, word);
                }
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00cb_21e0, &args![this, other]);
        let transposed = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == MATRIX3_SET_TRANSPOSE)
            .count();
        assert_eq!(transposed, 1);
        let got = columns_of(&e, this);
        // inverse rotation of ROTATED applied to -(1, 2, 3): x' = -2, y' = 1, z' = -3
        assert_eq!(got[3][..3], [-2.0, 1.0, -3.0]);
    }

    fn approximate(e: &mut Engine, a: u32, b: u32, epsilon: f32, rotation_equal: bool) -> u8 {
        e.register_double(MATRIX3_IS_APPROXIMATELY_EQUAL, move |e, args| {
            e.mem.set_u8(args[1], u8::from(rotation_equal));
            Ret {
                eax: args[1],
                ..Ret::default()
            }
        });
        let out = e.mem.alloc(4);
        let got = e.call(0x00cb_2160, &args![a, out, b, epsilon]).u32();
        assert_eq!(got, out);
        e.mem.u8(out)
    }

    #[test]
    fn approximate_equality_needs_equal_rotations_and_close_translations() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
        let a = transform(&mut e, ROTATED);
        let mut moved = ROTATED;
        moved[3] = [1.1, 1.9, 3.0, 100.0];
        let b = transform(&mut e, moved);
        assert_eq!(approximate(&mut e, a, b, 0.2, true), 1);
        assert_eq!(approximate(&mut e, a, b, 0.05, true), 0);
        // rotations that differ: no translation check
        assert_eq!(approximate(&mut e, a, b, 0.2, false), 0);
        // a NaN translation lane
        e.mem.set_f32(b + 0x34, f32::NAN);
        assert_eq!(approximate(&mut e, a, b, 100.0, true), 0);
    }
}

#[cfg(test)]
mod tests_composed {
    use super::*;

    fn transform(e: &mut Engine, columns: [Lanes; 4]) -> u32 {
        let at = e.mem.alloc(TRANSFORM_SIZE);
        for (index, column) in columns.iter().enumerate() {
            store_lanes(e, at + 0x10 * index as u32, *column);
        }
        at
    }

    fn columns_of(e: &Engine, at: u32) -> [Lanes; 4] {
        std::array::from_fn(|index| load_lanes(e, at + 0x10 * index as u32))
    }

    /// 90 degrees about z, translation (1, 2, 3).
    const TURNED: [Lanes; 4] = [
        [0.0, 1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [1.0, 2.0, 3.0, 1.0],
    ];
    const SHIFTED: [Lanes; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [10.0, 20.0, 30.0, 1.0],
    ];

    /// `hkMatrix3::setTranspose` double (the real one is in another unit).
    fn install_transpose(e: &mut Engine) {
        e.register(MATRIX3_SET_TRANSPOSE, |e, a| {
            for column in 0..3u32 {
                for row in 0..3u32 {
                    let word = e.mem.u32(a[1] + 0x10 * row + 4 * column);
                    e.mem.set_u32(a[0] + 0x10 * column + 4 * row, word);
                }
            }
            Ret::default()
        });
    }

    fn product(e: &mut Engine, a: u32, b: u32) -> [Lanes; 4] {
        let out = e.mem.alloc(TRANSFORM_SIZE);
        e.call(0x00cb_2240, &args![out, a, b]);
        columns_of(e, out)
    }

    #[test]
    fn set_mul_eq_multiplies_this_by_the_other_in_place() {
        let mut e = Engine::new();
        let this = transform(&mut e, TURNED);
        let other = transform(&mut e, SHIFTED);
        let same = transform(&mut e, TURNED);
        let want = product(&mut e, same, other);
        e.call(0x00cb_23e0, &args![this, other]);
        assert_eq!(columns_of(&e, this), want);
        // The other operand is not changed.
        assert_eq!(columns_of(&e, other), SHIFTED);
        // Squaring: both operands are the same object, which reads the old columns.
        let square = transform(&mut e, TURNED);
        let copy = transform(&mut e, TURNED);
        let want = product(&mut e, copy, copy);
        e.call(0x00cb_23e0, &args![square, square]);
        assert_eq!(columns_of(&e, square), want);
    }

    #[test]
    fn set_mul_mul_inverse_multiplies_by_the_inverse_of_the_second() {
        let mut e = Engine::new();
        install_transpose(&mut e);
        e.map(0x0100_0000, 0x0030_0000);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        let a = transform(&mut e, SHIFTED);
        let b = transform(&mut e, TURNED);
        let this = e.mem.alloc(TRANSFORM_SIZE);
        // a = SHIFTED, b = TURNED: result = a * inverse(b).
        let inverse = e.mem.alloc(TRANSFORM_SIZE);
        e.call(0x00cb_21e0, &args![inverse, b]);
        let want = product(&mut e, a, inverse);
        e.call(0x00cb_2420, &args![this, a, b]);
        assert_eq!(columns_of(&e, this), want);
        // a * inverse(a) is the identity rotation with no translation.
        let identity = e.mem.alloc(TRANSFORM_SIZE);
        e.call(0x00cb_2420, &args![identity, b, b]);
        let got = columns_of(&e, identity);
        assert_eq!(got[0][..3], [1.0, 0.0, 0.0]);
        assert_eq!(got[1][..3], [0.0, 1.0, 0.0]);
        assert_eq!(got[2][..3], [0.0, 0.0, 1.0]);
        assert_eq!(got[3][..3], [0.0, 0.0, 0.0]);
    }

    #[test]
    fn fn_00cb22c0_multiplies_the_converted_source_by_the_other() {
        let mut e = Engine::new();
        // The conversion double writes the TURNED transform into its output word's target.
        e.register_double(CONVERT_TO_TRANSFORM, |e, a| {
            for (index, column) in TURNED.iter().enumerate() {
                store_lanes(e, a[1] + 0x10 * index as u32, *column);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let source = e.mem.alloc(0x40);
        let other = transform(&mut e, SHIFTED);
        let this = e.mem.alloc(TRANSFORM_SIZE);
        let turned = transform(&mut e, TURNED);
        let want = product(&mut e, turned, other);
        e.call(0x00cb_22c0, &args![this, source, other]);
        assert_eq!(columns_of(&e, this), want);
        let conversions: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == CONVERT_TO_TRANSFORM)
            .map(|c| c.1[0])
            .collect();
        assert_eq!(conversions, vec![source]);
    }
}

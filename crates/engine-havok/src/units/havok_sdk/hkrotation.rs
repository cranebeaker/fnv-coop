//! `Havok SDK/hkrotation.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `(1.0, 0, 0, 0)` as four `float`s.
const UNIT_FIRST_LANE_010C7180: u32 = 0x010c_7180;
/// Sign-bit mask for lanes 1 to 3 (`0, -0, -0, -0`).
const SIGN_LANES_1_TO_3_010C8780: u32 = 0x010c_8780;
/// Sign-bit mask for lanes 0, 2 and 3 (`-0, 0, -0, -0`).
const SIGN_LANES_0_2_3_010C8770: u32 = 0x010c_8770;

/// Four lanes of `float` at `addr`.
fn read_lanes_00cb2d90(e: &Engine, addr: u32) -> [f32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.f32(addr + offset))
}

/// `XORPS` with the sign mask stored at `mask`.
fn xor_lanes_00cb2d90(e: &Engine, lanes: [f32; 4], mask: u32) -> [f32; 4] {
    let mut out = lanes;
    for (i, lane) in out.iter_mut().enumerate() {
        *lane = f32::from_bits(lane.to_bits() ^ e.mem.u32(mask + 4 * i as u32));
    }
    out
}

/// Lane-wise `op` of two four-lane values (`MULPS`, `SUBPS`).
fn zip_00cb2d90(a: [f32; 4], b: [f32; 4], op: fn(f32, f32) -> f32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| op(a[i], b[i]))
}

// Translated from 00cb2d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (the map places it in `hkrotation.cpp`). Thiscall, one word
/// (`RET 4`), SSE (`f32` per lane): writes the rotation matrix of the
/// quaternion `(x, y, z, w)` at `quaternion` as three four-float columns at
/// `this`, `this + 0x10` and `this + 0x20` (the fourth float of the last
/// column is 0):
/// column 0 = `(1 - 2z*z - 2y*y, 2wz + 2yx, 2zx - 2wy, 2zy - 2wx)`,
/// column 1 = `(2yx - 2wz, 1 - 2z*z - 2x*x, 2wx + 2zy, 2wy + 2zx)`,
/// column 2 = `(2wy + 2zx, 2zy - 2wx, 1 - 2x*x - 2y*y, 0)`; every
/// subtraction `a - (-b)` is kept as the code computes it, with the sign
/// flips done by `XORPS` on the masks in the exe's data.
pub fn fn_00cb2d90(e: &mut Engine, this: Ptr, quaternion: Ptr) {
    let q = read_lanes_00cb2d90(e, quaternion.addr());
    let one = read_lanes_00cb2d90(e, UNIT_FIRST_LANE_010C7180);
    let twice = [q[0] + q[0], q[1] + q[1], q[2] + q[2], q[3] + q[3]];
    // (y, x, x, y) and (2y, 2y, 2z, 2z).
    let swapped = [q[1], q[0], q[0], q[1]];
    let twice_pairs = [twice[1], twice[1], twice[2], twice[2]];
    // `MULSS`: only the first lane of the copy of `q` is multiplied.
    let x_twice_x = q[0] * twice[0];
    // (z, w, w, w) * (2z, 2z, 2y, 2x), sign-flipped in lanes 0, 2, 3.
    let products = zip_00cb2d90(
        [q[2], q[3], q[3], q[3]],
        [twice[2], twice[2], twice[1], twice[0]],
        |a, b| a * b,
    );
    let mut flipped = xor_lanes_00cb2d90(e, products, SIGN_LANES_0_2_3_010C8770);
    let pairs = zip_00cb2d90(twice_pairs, swapped, |a, b| a * b);
    // `ADDSS`: first lane only.
    flipped[0] += one[0];
    let pairs_negated = xor_lanes_00cb2d90(e, pairs, SIGN_LANES_1_TO_3_010C8780);
    let column0 = zip_00cb2d90(flipped, pairs_negated, |a, b| a - b);
    let unflipped = xor_lanes_00cb2d90(e, flipped, SIGN_LANES_1_TO_3_010C8780);
    // `MOVSS XMM2, XMM4`: the first lane of the negated pairs is replaced.
    let mut subtrahend = pairs_negated;
    subtrahend[0] = x_twice_x;
    let mut one_minus = one;
    for (i, lane) in column0.iter().enumerate() {
        e.mem.set_f32(this.addr() + 4 * i as u32, *lane);
    }
    let difference = zip_00cb2d90(unflipped, subtrahend, |a, b| a - b);
    // `SHUFPS 0xb1`: (1, 0, 3, 2).
    let column1 = [difference[1], difference[0], difference[3], difference[2]];
    for (i, lane) in column1.iter().enumerate() {
        e.mem.set_f32(this.addr() + 0x10 + 4 * i as u32, *lane);
    }
    one_minus[0] -= x_twice_x;
    // `MOVHLPS` then `SHUFPS 0x8d`: (column1[3], column0[3], first, third).
    one_minus[0] -= pairs[0];
    let column2 = [column1[3], column0[3], one_minus[0], one_minus[2]];
    for (i, lane) in column2.iter().enumerate() {
        e.mem.set_f32(this.addr() + 0x20 + 4 * i as u32, *lane);
    }
}

/// `hkMatrix3::isOk` (Xbox PDB name), `thiscall`, `this` the matrix, one stack word: the `bool`
/// result pointer.
const MATRIX3_IS_OK: u32 = 0x00cd_4af0;
/// `hkQuaternion::setAxisAngle` (Xbox PDB name), `thiscall`, axis pointer and angle.
const QUATERNION_SET_AXIS_ANGLE: u32 = 0x00cb_2450;
/// `hkQuaternion::setAndNormalize` (Xbox PDB name), `thiscall`, the rotation.
const QUATERNION_SET_AND_NORMALIZE: u32 = 0x00cb_2830;
/// The mask that clears the sign of each of four lanes (`ANDPS`).
const ABSOLUTE_VALUE_MASK_01017D10: u32 = 0x0101_7d10;

// Translated from 00cb2d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRotation::isOk` (Xbox PDB), `thiscall`, `RET 4`: `this` stays in `ECX` and the stack
/// word (the `bool` result pointer) is passed to `hkMatrix3::isOk` (`00cd4af0`); returns that
/// pointer.
pub fn hk_rotation_is_ok(e: &mut Engine, this: Ptr, result: u32) -> u32 {
    e.call(MATRIX3_IS_OK, &args![this, result]);
    result
}

// Translated from 00cb2e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRotation::setAxisAngle` (Xbox PDB), `thiscall`, `RET 8`: builds the quaternion of the
/// rotation by `angle` about the axis at `axis` in a 16-byte aligned local
/// (`hkQuaternion::setAxisAngle`, `00cb2450`; the angle goes through `FLD`/`FSTP` on its way to
/// the stack, which quiets a signalling NaN) and then the rotation matrix of that
/// quaternion into `this` (`00cb2d90`).
pub fn hk_rotation_set_axis_angle(e: &mut Engine, this: Ptr, axis: u32, angle: f32) {
    e.with_stack(0x10, |e, quaternion| {
        e.call(
            QUATERNION_SET_AXIS_ANGLE,
            &args![quaternion, axis, x87_quiet(angle)],
        );
        fn_00cb2d90(e, this, quaternion);
    });
}

// Translated from 00cb2fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRotation::renormalize` (Xbox PDB), `fastcall` (`this` in `ECX`, no stack words): turns the
/// rotation into a normalized quaternion in a 16-byte aligned local
/// (`hkQuaternion::setAndNormalize`, `00cb2830`, given `this`) and writes the matrix of that
/// quaternion back into `this` (`00cb2d90`).
pub fn hk_rotation_renormalize(e: &mut Engine, this: Ptr) {
    e.with_stack(0x10, |e, quaternion| {
        e.call(QUATERNION_SET_AND_NORMALIZE, &args![quaternion, this]);
        fn_00cb2d90(e, this, quaternion);
    });
}

/// The `ADDSS` chain over the squares of the first three lanes of the column at `column`
/// (the squares come from `MULPS`): `sq[2] + (sq[1] + sq[0])`.
fn column_square_sum_00cb2e70(e: &Engine, column: u32) -> f32 {
    let lanes = read_lanes_00cb2d90(e, column);
    let squares = lanes.map(|lane| lane * lane);
    squares[2] + (squares[1] + squares[0])
}

/// `|sum - 1.0|` as the code computes it: the sum goes through `FLD`, the subtraction is done on
/// the x87 stack, and the difference and its absolute value each make a `float` round trip
/// through memory.
fn column_deviation_00cb2e70(sum: f32) -> f32 {
    let difference = x87_sub(x87_load32(sum.to_bits()), 1.0);
    let stored = x87_store32(difference);
    f32::from_bits(x87_quiet_bits(stored) & 0x7fff_ffff)
}

// Translated from 00cb2e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRotation::isOrthonormal` (Xbox PDB), `thiscall`, `RET 8`: writes to the byte at `result`
/// whether the three columns of `this` are unit length within `epsilon` and the third is the
/// cross product of the first two within `epsilon`.
///
/// The length tests are x87 compares on `|squared length - 1|` against `epsilon`: the first
/// fails only when `epsilon < |d|`, the second and third only when `|d| > epsilon` (a NaN passes,
/// as the flag tests have it). The cross product test is SSE: lanes 0 to 2 of
/// `|cross(c0, c1) - c2| <= epsilon` (`CMPLEPS`, false for a NaN) must all hold; lane 3 is
/// computed but not looked at (`MOVMSKPS`, `AND 7`).
/// Answers `result` (`EAX` at both `RET`s; callers test the flag through it, 00cb903b).
pub fn hk_rotation_is_orthonormal(e: &mut Engine, this: Ptr, result: u32, epsilon: f32) -> u32 {
    let base = this.addr();
    let epsilon_stack = x87_load32(epsilon.to_bits());
    let first = column_deviation_00cb2e70(column_square_sum_00cb2e70(e, base));
    if epsilon_stack < f64::from(first) {
        e.mem.set_u8(result, 0);
        return result;
    }
    let second = column_deviation_00cb2e70(column_square_sum_00cb2e70(e, base + 0x10));
    if f64::from(second) > epsilon_stack {
        e.mem.set_u8(result, 0);
        return result;
    }
    let third = column_deviation_00cb2e70(column_square_sum_00cb2e70(e, base + 0x20));
    if f64::from(third) > epsilon_stack {
        e.mem.set_u8(result, 0);
        return result;
    }
    let c0 = read_lanes_00cb2d90(e, base);
    let c1 = read_lanes_00cb2d90(e, base + 0x10);
    let c2 = read_lanes_00cb2d90(e, base + 0x20);
    // `SHUFPS` 0xd2 selects lanes (2, 0, 1, 3); 0xc9 selects (1, 2, 0, 3).
    let c0_d2 = [c0[2], c0[0], c0[1], c0[3]];
    let c0_c9 = [c0[1], c0[2], c0[0], c0[3]];
    let c1_c9 = [c1[1], c1[2], c1[0], c1[3]];
    let c1_d2 = [c1[2], c1[0], c1[1], c1[3]];
    let minuend = zip_00cb2d90(c1_d2, c0_c9, |a, b| a * b);
    let subtrahend = zip_00cb2d90(c1_c9, c0_d2, |a, b| a * b);
    let cross = zip_00cb2d90(minuend, subtrahend, |a, b| a - b);
    let error = zip_00cb2d90(cross, c2, |a, b| a - b);
    let mut mask = 0u32;
    for (lane, value) in error.iter().enumerate() {
        let bits = value.to_bits() & e.mem.u32(ABSOLUTE_VALUE_MASK_01017D10 + 4 * lane as u32);
        if f32::from_bits(bits) <= epsilon {
            mask |= 1 << lane;
        }
    }
    e.mem.set_u8(result, u8::from(mask & 7 == 7));
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb2d90, fn_00cb2d90(Ptr, Ptr)),
        entry!(0x00cb2d70, hk_rotation_is_ok(Ptr, u32) -> u32),
        entry!(0x00cb2e30, hk_rotation_set_axis_angle(Ptr, u32, f32)),
        entry!(0x00cb2fc0, hk_rotation_renormalize(Ptr)),
        entry!(0x00cb2e70, hk_rotation_is_orthonormal(Ptr, u32, f32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rotation_engine_00cb2d90() -> Engine {
        let mut e = Engine::new();
        e.map(0x010c_7000, 0x2000);
        e.mem.set_f32(UNIT_FIRST_LANE_010C7180, 1.0);
        for lane in 1..4 {
            e.mem
                .set_u32(SIGN_LANES_1_TO_3_010C8780 + 4 * lane, 0x8000_0000);
        }
        for lane in [0, 2, 3] {
            e.mem
                .set_u32(SIGN_LANES_0_2_3_010C8770 + 4 * lane, 0x8000_0000);
        }
        e
    }

    fn matrix_00cb2d90(e: &mut Engine, quaternion: [f32; 4]) -> [[f32; 4]; 3] {
        let input = e.mem.alloc(16);
        for (i, value) in quaternion.iter().enumerate() {
            e.mem.set_f32(input + 4 * i as u32, *value);
        }
        let out = e.mem.alloc(0x30);
        e.call(
            0x00cb_2d90,
            &args![Ptr::<()>::new(out), Ptr::<()>::new(input)],
        );
        [0, 0x10, 0x20].map(|column| [0, 4, 8, 12].map(|lane| e.mem.f32(out + column + lane)))
    }

    #[test]
    fn fn_00cb2d90_identity_quaternion_gives_the_identity_columns() {
        let mut e = rotation_engine_00cb2d90();
        let m = matrix_00cb2d90(&mut e, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(m[0], [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(m[1], [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(m[2], [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn fn_00cb2d90_quarter_turn_about_z() {
        let mut e = rotation_engine_00cb2d90();
        let s = std::f32::consts::FRAC_1_SQRT_2;
        let m = matrix_00cb2d90(&mut e, [0.0, 0.0, s, s]);
        let near = |a: f32, b: f32| (a - b).abs() < 1e-6;
        // The columns are the rotated x, y and z axes.
        assert!(near(m[0][0], 0.0) && near(m[0][1], 1.0) && near(m[0][2], 0.0));
        assert!(near(m[1][0], -1.0) && near(m[1][1], 0.0) && near(m[1][2], 0.0));
        assert!(near(m[2][0], 0.0) && near(m[2][1], 0.0) && near(m[2][2], 1.0));
        assert_eq!([m[0][3], m[1][3], m[2][3]], [0.0, 0.0, 0.0]);
    }

    #[test]
    fn fn_00cb2d90_half_turn_about_x_flips_y_and_z() {
        let mut e = rotation_engine_00cb2d90();
        let m = matrix_00cb2d90(&mut e, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(m[0][..3], [1.0, 0.0, 0.0]);
        assert_eq!(m[1][..3], [0.0, -1.0, 0.0]);
        assert_eq!(m[2][..3], [0.0, 0.0, -1.0]);
    }

    /// Three columns of the matrix at a fresh block; `ANDPS`'s mask in the exe's data.
    fn matrix_engine(columns: [[f32; 4]; 3]) -> (Engine, u32) {
        let mut e = rotation_engine_00cb2d90();
        e.map(0x0101_7000, 0x1000);
        for lane in 0..4 {
            e.mem
                .set_u32(ABSOLUTE_VALUE_MASK_01017D10 + 4 * lane, 0x7fff_ffff);
        }
        let matrix = e.mem.alloc(0x30);
        for (column, lanes) in columns.iter().enumerate() {
            for (lane, value) in lanes.iter().enumerate() {
                e.mem
                    .set_f32(matrix + 0x10 * column as u32 + 4 * lane as u32, *value);
            }
        }
        (e, matrix)
    }

    fn is_orthonormal(columns: [[f32; 4]; 3], epsilon: f32) -> bool {
        let (mut e, matrix) = matrix_engine(columns);
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 0xaa);
        e.call(0x00cb_2e70, &args![matrix, result, epsilon]);
        match e.mem.u8(result) {
            0 => false,
            1 => true,
            other => panic!("not a bool: {other}"),
        }
    }

    #[test]
    fn is_orthonormal_answers_its_result_pointer() {
        let identity = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ];
        let (mut e, matrix) = matrix_engine(identity);
        let result = e.mem.alloc(4);
        // EAX is `result` at both RETs; the callers test the flag through it (00cb903b).
        let answered = e.call(0x00cb_2e70, &args![matrix, result, 0.01f32]).u32();
        assert_eq!(answered, result);
        assert_eq!(e.mem.u8(result), 1);
    }

    const IDENTITY: [[f32; 4]; 3] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ];

    #[test]
    fn hk_rotation_is_ok_hands_this_and_the_result_pointer_to_the_matrix_test() {
        let mut e = Engine::new();
        e.register(MATRIX3_IS_OK, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let got = e.call(0x00cb_2d70, &args![0x1000u32, 0x2000u32]).u32();
        assert_eq!(got, 0x2000);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (MATRIX3_IS_OK, vec![0x1000, 0x2000]));
    }

    #[test]
    fn hk_rotation_set_axis_angle_builds_the_quaternion_then_the_matrix() {
        let (mut e, matrix) = matrix_engine([[9.0; 4]; 3]);
        // The double writes the half turn about x (1, 0, 0, 0) into the local quaternion.
        e.register_double(QUATERNION_SET_AXIS_ANGLE, |e, a| {
            for (lane, value) in [1.0f32, 0.0, 0.0, 0.0].iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * lane as u32, *value);
            }
            Ret::default()
        });
        let axis = e.mem.alloc(16);
        e.call_log = Some(vec![]);
        // A signalling NaN angle goes to the callee quieted.
        e.call(0x00cb_2e30, &args![matrix, axis, 0x7f80_0001u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|c| c.0 == QUATERNION_SET_AXIS_ANGLE)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[1], axis);
        assert_eq!(calls[0].1[2], 0x7fc0_0001);
        assert_eq!(e.mem.f32(matrix + 0x10 + 4), -1.0);
        assert_eq!(e.mem.f32(matrix + 0x20 + 8), -1.0);
    }

    #[test]
    fn hk_rotation_renormalize_builds_the_matrix_from_the_normalized_quaternion() {
        let (mut e, matrix) = matrix_engine([[9.0; 4]; 3]);
        // The double writes the identity quaternion.
        e.register_double(QUATERNION_SET_AND_NORMALIZE, |e, a| {
            for (lane, value) in [0.0f32, 0.0, 0.0, 1.0].iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * lane as u32, *value);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00cb_2fc0, &args![matrix]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|c| c.0 == QUATERNION_SET_AND_NORMALIZE)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[1], matrix);
        for (column, expected) in IDENTITY.iter().enumerate() {
            for (lane, want) in expected.iter().enumerate() {
                assert_eq!(
                    e.mem.f32(matrix + 0x10 * column as u32 + 4 * lane as u32),
                    *want
                );
            }
        }
    }

    #[test]
    fn hk_rotation_is_orthonormal_accepts_a_rotation_and_rejects_each_failure() {
        assert!(is_orthonormal(IDENTITY, 1e-3));
        // First, second and third column too long.
        for column in 0..3 {
            let mut skewed = IDENTITY;
            skewed[column][column] = 1.1;
            assert!(!is_orthonormal(skewed, 1e-3), "column {column}");
        }
        // Unit columns whose third is not the cross product of the first two.
        let mut flipped = IDENTITY;
        flipped[2][2] = -1.0;
        assert!(!is_orthonormal(flipped, 1e-3));
        // Within epsilon: accepted.
        let mut close = IDENTITY;
        close[0][0] = 1.0004;
        assert!(is_orthonormal(close, 1e-2));
        // A NaN in the third column passes the length test (unordered flags) but fails the
        // cross product test.
        let mut not_a_number = IDENTITY;
        not_a_number[2][0] = f32::NAN;
        assert!(!is_orthonormal(not_a_number, 1e-3));
        // A NaN epsilon makes every flag test unordered: the length tests pass, the lanes
        // compare false.
        assert!(!is_orthonormal(IDENTITY, f32::NAN));
        // Lane 3 does not take part in the cross product test.
        let mut lane_three = IDENTITY;
        lane_three[2][3] = 5.0;
        assert!(is_orthonormal(lane_three, 1e-3));
    }
}

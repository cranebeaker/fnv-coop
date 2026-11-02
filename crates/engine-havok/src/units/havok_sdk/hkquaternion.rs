//! `Havok SDK/hkquaternion.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A quaternion is four floats `(x, y, z, w)` at the object's address (16-byte aligned, a
//! `MOVAPS` load). The code mixes SSE (`float` lanes) with x87 (extended precision with a
//! store to `float` at each step): the x87 steps use the shared helpers (`x87_add`, ...,
//! `x87_store32`) and the SSE steps compute in `f32`. `RSQRTSS` is the host's estimate
//! (it differs between CPU makers by one unit in the last place); the code refines it once
//! with a Newton step.

#[allow(unused_imports)]
use crate::prelude::*;

/// `_CIsqrt` (`00ec6040`, the argument in `ST0`, a leading `f64`), `_CIsin` (`00eca0a0`) and
/// `hkMath::acos` (`00c74a70`, cdecl, one float word, the result in `ST0`).
const CI_SQRT: u32 = 0x00ec_6040;
const CI_SIN: u32 = 0x00ec_a0a0;
const MATH_ACOS: u32 = 0x00c7_4a70;
/// `hkQuaternion::setAxisAngle` (`00cb2450`, `this` = the quaternion, `(axis, angle)`),
/// `setMul` (`00c66320`, `this` = the result, `(a, b)`), `getAngle` (`00c74c20`, `this` = the
/// quaternion, the angle in `ST0`), `normalize` (`005611c0`, `this`) and `hkRotation::set`
/// (`00cb2d90`, `this` = the rotation's 0x30 bytes, `(quaternion)`).
const SET_AXIS_ANGLE: u32 = 0x00cb_2450;
const SET_MUL: u32 = 0x00c6_6320;
const GET_ANGLE: u32 = 0x00c7_4c20;
const NORMALIZE: u32 = 0x0056_11c0;
/// Constants of the image: the doubles `1.0` (`01012070`) and `0.5` (`01011588`), the slerp
/// threshold `0.999` (`010c8758`), the removal limits `-0.001` (`010c8760`) and `0.001`
/// (`01016978`); the floats `0.0` (`01011d78`), `3.0` (`01017718`), `0.5` (`01016248`), `-1.0`
/// (`01012054`) and pi (`0102b3c8`); the vectors of `-0.5` (`010c71e0`), the identity
/// quaternion (`010c71b0`) and the sign bits of the first three lanes (`010c4c00`).
const DOUBLE_ONE: u32 = 0x0101_2070;
const DOUBLE_HALF: u32 = 0x0101_1588;
const SLERP_THRESHOLD: u32 = 0x010c_8758;
const REMOVE_LOWER_LIMIT: u32 = 0x010c_8760;
const REMOVE_UPPER_LIMIT: u32 = 0x0101_6978;
const FLOAT_ZERO: u32 = 0x0101_1d78;
const FLOAT_THREE: u32 = 0x0101_7718;
const FLOAT_HALF: u32 = 0x0101_6248;
const FLOAT_MINUS_ONE: u32 = 0x0101_2054;
const FLOAT_PI: u32 = 0x0102_b3c8;
const VECTOR_MINUS_HALF: u32 = 0x010c_71e0;
const VECTOR_IDENTITY: u32 = 0x010c_71b0;
const VECTOR_SIGN_BITS: u32 = 0x010c_4c00;

/// Four `float` lanes.
pub(crate) type Vec4 = [f32; 4];

/// `MOVAPS` load: the four lanes at `address`, bit for bit.
pub(crate) fn load_vec(e: &Engine, address: u32) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(e.mem.u32(address + 4 * lane as u32)))
}

/// `MOVAPS` store.
pub(crate) fn store_vec(e: &mut Engine, address: u32, value: Vec4) {
    for (lane, part) in value.into_iter().enumerate() {
        e.mem.set_u32(address + 4 * lane as u32, part.to_bits());
    }
}

/// `ADDPS a, b`, `SUBPS a, b` and `MULPS a, b`.
pub(crate) fn add_ps(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|lane| a[lane] + b[lane])
}

pub(crate) fn sub_ps(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|lane| a[lane] - b[lane])
}

pub(crate) fn mul_ps(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|lane| a[lane] * b[lane])
}

/// `SHUFPS a, b, imm`: lanes 0 and 1 from `a`, lanes 2 and 3 from `b`, each chosen by two
/// bits of `imm`.
pub(crate) fn shufps(a: Vec4, b: Vec4, imm: u8) -> Vec4 {
    [
        a[usize::from(imm & 3)],
        a[usize::from((imm >> 2) & 3)],
        b[usize::from((imm >> 4) & 3)],
        b[usize::from((imm >> 6) & 3)],
    ]
}

/// The lane-0 sum the `SHUFPS 0x4e` / `0xb1` ladder leaves: `(x3 + x1) + (x2 + x0)`.
pub(crate) fn sum_four(x: Vec4) -> f32 {
    (x[3] + x[1]) + (x[2] + x[0])
}

/// The lane-0 sum the `SHUFPS 0x55` / `0xaa` ladder of three lanes leaves: `x2 + (x1 + x0)`.
pub(crate) fn sum_three(x: Vec4) -> f32 {
    x[2] + (x[1] + x[0])
}

/// `RSQRTSS` of one lane: the host CPU's estimate of `1 / sqrt(x)` on x86.
#[cfg(target_arch = "x86_64")]
fn rsqrt_estimate(x: f32) -> f32 {
    use std::arch::x86_64::{_mm_cvtss_f32, _mm_rsqrt_ss, _mm_set_ss};
    // SAFETY: SSE is part of the x86_64 baseline.
    unsafe { _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x))) }
}

#[cfg(not(target_arch = "x86_64"))]
fn rsqrt_estimate(x: f32) -> f32 {
    1.0 / x.sqrt()
}

/// The refined reciprocal square root of the normalizers: `(0.5 * r) * (3 - (x * r) * r)` for `r`
/// the `RSQRTSS` estimate of `x`, and `0` when `x` equals the `0.0` of the image (`CMPEQSS`
/// then `ANDNPS`; a NaN is not equal and stays NaN). The constants are read from the image.
pub(crate) fn refined_rsqrt(e: &Engine, x: f32) -> f32 {
    let zero = e.mem.f32(FLOAT_ZERO);
    let three = e.mem.f32(FLOAT_THREE);
    let half = e.mem.f32(FLOAT_HALF);
    let r = rsqrt_estimate(x);
    let value = (half * r) * (three - (x * r) * r);
    if x == zero {
        0.0
    } else {
        value
    }
}

/// A `double` constant of the image.
fn double_at(e: &Engine, address: u32) -> f64 {
    f64::from_bits(e.mem.u64(address))
}

/// A float of the image copied through the x87 stack (`FLD`, `FSTP`): its bits.
fn quiet_float_at(e: &Engine, address: u32) -> u32 {
    x87_quiet_bits(e.mem.u32(address))
}

/// `quaternion / sqrt(dot)`: lanes scaled by the refined reciprocal square root of their square
/// sum (the exe's `MULPS`, `SHUFPS 0x4e`/`0xb1` and `RSQRTSS` normaliser).
fn normalized(e: &Engine, v: Vec4) -> Vec4 {
    let scale = refined_rsqrt(e, sum_four(mul_ps(v, v)));
    mul_ps([scale; 4], v)
}

// Translated from 00cb2550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `quaternionFromRotatation` (Xbox PDB name), `ESI` = the rotation matrix (three columns of
/// four floats), stack `(dest)`: the quaternion of the rotation, with the x87 steps of the
/// exe. When the trace `m0 + m5 + m10` (stored as a float) is above 0, `s = trace + 1.0`,
/// `w = sqrt(s)`, `inv = 0.5 / w` and the lanes are `(m6 - m9) * inv`, `(m8 - m2) * inv`,
/// `(m1 - m4) * inv`, `w * 0.5`. Otherwise the largest diagonal element `i` is found
/// (`m0 < m5` selects 1, then `m[5i] < m10` selects 2; `j` and `k` follow `1, 2, 0`), `s =
/// m[5i] - (m[5k] + m[5j]) + 1.0`, and lane `i` is `w * 0.5`, lane 3 `(m[k + 4j] - m[j + 4k]) *
/// inv`, lanes `j` and `k` `(m[i + 4j] + m[j + 4i]) * inv` and `(m[i + 4k] + m[k + 4i]) * inv`.
pub fn quaternion_from_rotation(e: &mut Engine, rotation: Ptr, dest: Ptr) {
    let m = |e: &Engine, index: u32| e.mem.u32(rotation.addr() + 4 * index);
    let one = double_at(e, DOUBLE_ONE);
    let half = double_at(e, DOUBLE_HALF);
    let trace = x87_store32(x87_add(
        x87_add(x87_load32(m(e, 0)), x87_m32(m(e, 5))),
        x87_m32(m(e, 10)),
    ));
    let result = if f32::from_bits(trace) > 0.0 {
        let s = x87_store32(x87_add(x87_load32(trace), one));
        let w = x87_store32(e.call(CI_SQRT, &args![x87_load32(s)]).f64());
        let inv = x87_store32(x87_div(half, x87_load32(w)));
        let lane = |first: u32, second: u32| {
            x87_store32(x87_mul(
                x87_sub(x87_load32(first), x87_m32(second)),
                x87_load32(inv),
            ))
        };
        [
            lane(m(e, 6), m(e, 9)),
            lane(m(e, 8), m(e, 2)),
            lane(m(e, 1), m(e, 4)),
            x87_store32(x87_mul(half, x87_load32(w))),
        ]
    } else {
        let next = [1usize, 2, 0];
        let mut i = usize::from(f32::from_bits(m(e, 0)) < f32::from_bits(m(e, 5)));
        if f32::from_bits(m(e, 5 * i as u32)) < f32::from_bits(m(e, 10)) {
            i = 2;
        }
        let j = next[i];
        let k = next[j];
        let (i, j, k) = (i as u32, j as u32, k as u32);
        let sum = x87_add(x87_load32(m(e, 5 * k)), x87_m32(m(e, 5 * j)));
        let s = x87_store32(x87_add(x87_sub(x87_load32(m(e, 5 * i)), sum), one));
        let w = x87_store32(e.call(CI_SQRT, &args![x87_load32(s)]).f64());
        let inv = x87_store32(x87_div(half, x87_load32(w)));
        let mut lanes = [0u32; 4];
        lanes[i as usize] = x87_store32(x87_mul(half, x87_load32(w)));
        lanes[3] = x87_store32(x87_mul(
            x87_sub(x87_load32(m(e, k + 4 * j)), x87_m32(m(e, j + 4 * k))),
            x87_load32(inv),
        ));
        lanes[j as usize] = x87_store32(x87_mul(
            x87_add(x87_load32(m(e, i + 4 * j)), x87_m32(m(e, j + 4 * i))),
            x87_load32(inv),
        ));
        lanes[k as usize] = x87_store32(x87_mul(
            x87_add(x87_load32(m(e, i + 4 * k)), x87_m32(m(e, k + 4 * i))),
            x87_load32(inv),
        ));
        lanes
    };
    for (lane, bits) in result.into_iter().enumerate() {
        e.mem.set_u32(dest.addr() + 4 * lane as u32, bits);
    }
}

// Translated from 00cb26e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkQuaternion::set` (Xbox PDB), `(rotation)`: the quaternion of the rotation matrix
/// ([`quaternion_from_rotation`]).
pub fn hk_quaternion_set(e: &mut Engine, this: Ptr, rotation: Ptr) {
    quaternion_from_rotation(e, rotation, this);
}

/// The perpendicular of `vector` that `00cb2700` and `removeAxisComponent` build: with `a0`,
/// `a1`, `a2` the absolute values of the three lanes (`FLD`, `FABS`, `FSTP`), the smaller of `a0` and
/// `a1` is found (`a0 > a1` picks `a1`; a NaN keeps `a0`), `low` and `high` are the indices of
/// the first and second, and `second` is `high` when that minimum is above `a2` (else 2). The
/// vector is zero except lane `low` = `vector[second]` and lane `second` = `-vector[low]`.
fn perpendicular(e: &Engine, vector: u32) -> [u32; 4] {
    let lane = |index: u32| x87_quiet_bits(e.mem.u32(vector + 4 * index));
    let absolute = |index: u32| f32::from_bits(lane(index) & 0x7fff_ffff);
    let (a0, a1, a2) = (absolute(0), absolute(1), absolute(2));
    let (low, high, smallest) = if a0 > a1 { (0, 1, a1) } else { (1, 0, a0) };
    let second = if smallest > a2 { high } else { 2 };
    let mut result = [0u32; 4];
    result[low] = lane(second as u32);
    result[second] = lane(low as u32) ^ 0x8000_0000;
    result
}

// Translated from 00cb2700 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (`this` = the result, `(vector)`): a unit vector perpendicular to
/// the vector's lanes: the perpendicular ([`perpendicular`]) scaled by the refined reciprocal
/// square root of its square sum (`z + (y + x)`), lane 3 set to 0.
pub fn fn_00cb2700(e: &mut Engine, this: Ptr, vector: Ptr) {
    let bits = perpendicular(e, vector.addr());
    let u: Vec4 = bits.map(f32::from_bits);
    let scale = refined_rsqrt(e, sum_three(mul_ps(u, u)));
    let mut result = mul_ps([scale; 4], u);
    result[3] = 0.0;
    store_vec(e, this.addr(), result);
}

// Translated from 00cb2830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkQuaternion::setAndNormalize` (Xbox PDB), `(rotation)`: [`quaternion_from_rotation`] and
/// then the normalizer `005611c0` on `this`.
pub fn hk_quaternion_set_and_normalize(e: &mut Engine, this: Ptr, rotation: Ptr) {
    quaternion_from_rotation(e, rotation, this);
    e.call(NORMALIZE, &args![this]);
}

// Translated from 00cb2850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkQuaternion::setSlerp` (Xbox PDB), `(from, to, t)`: the dot product of the two quaternions
/// (`(p3 + p1) + (p2 + p0)` of the lane products), made positive with a sign of -1 when
/// it is negative; when it is below 0.999 (an ordered comparison on the x87 stack: a NaN takes the
/// other branch) the spherical interpolation with `theta = acos(dot)` (`hkMath::acos`),
/// `1 / sqrt(1 - dot * dot)`, `sin(theta - theta * t)` and `sin(theta * t)` scaled by it
/// (the second also by the sign), each x87 step stored as a float; otherwise the linear
/// `(1 - t) * from + (t * sign) * to`. The result is normalized with the refined reciprocal
/// square root and stored at `this`.
pub fn hk_quaternion_set_slerp(e: &mut Engine, this: Ptr, from: Ptr, to: Ptr, t: f32) {
    let t_bits = t.to_bits();
    let q1 = load_vec(e, from.addr());
    let q2 = load_vec(e, to.addr());
    let dot = sum_four(mul_ps(q2, q1));
    let mut work = x87_quiet_bits(dot.to_bits());
    let mut sign = 1.0f32.to_bits();
    if dot < 0.0 {
        work = x87_quiet_bits(dot.to_bits()) ^ 0x8000_0000;
        sign = quiet_float_at(e, FLOAT_MINUS_ONE);
    }
    let blended = if f64::from(f32::from_bits(work)) < double_at(e, SLERP_THRESHOLD) {
        let theta = x87_store32(e.call(MATH_ACOS, &args![x87_quiet_bits(work)]).f64());
        let work_value = x87_load32(work);
        let sin_squared = x87_store32(x87_sub(1.0, x87_mul(work_value, work_value)));
        let sine = x87_store32(e.call(CI_SQRT, &args![x87_load32(sin_squared)]).f64());
        let inverse = x87_store32(x87_div(1.0, x87_load32(sine)));
        let scaled = x87_store32(x87_mul(x87_load32(theta), x87_m32(t_bits)));
        let rest = x87_store32(x87_sub(x87_load32(theta), x87_m32(scaled)));
        let sine_rest = x87_store32(e.call(CI_SIN, &args![x87_load32(rest)]).f64());
        let first = x87_store32(x87_mul(x87_load32(sine_rest), x87_m32(inverse)));
        let sine_scaled = x87_store32(e.call(CI_SIN, &args![x87_load32(scaled)]).f64());
        let second_unsigned = x87_store32(x87_mul(x87_load32(sine_scaled), x87_m32(inverse)));
        let second = x87_store32(x87_mul(x87_load32(second_unsigned), x87_m32(sign)));
        add_ps(
            mul_ps([f32::from_bits(second); 4], q2),
            mul_ps([f32::from_bits(first); 4], q1),
        )
    } else {
        let one_minus_t = x87_store32(x87_sub(1.0, x87_load32(t_bits)));
        let signed_t = x87_store32(x87_mul(x87_load32(t_bits), x87_m32(sign)));
        add_ps(
            mul_ps([f32::from_bits(signed_t); 4], q2),
            mul_ps([f32::from_bits(one_minus_t); 4], q1),
        )
    };
    let result = normalized(e, blended);
    store_vec(e, this.addr(), result);
}

// Translated from 00cb2a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkQuaternion::removeAxisComponent` (Xbox PDB), `(axis)`: with `v` the axis rotated by this
/// quaternion (`2 * (cross(q, a) * w + ((w * w - 0.5) * a + dot(a, q) * q))`, each lane
/// product and sum in the exe's order) and `dp = a . v`: when `dp - 1` is above -0.001 the
/// quaternion becomes the identity (`(0, 0, 0, 1)` from the image); when `1 + dp` is below 0.001
/// (the axes are opposite) it becomes the rotation by pi about a perpendicular of the axis
/// (`setAxisAngle`); otherwise the rotation by `acos(dp)` about the normalized `cross(a,
/// v)` (`hkMath::acos`, `setAxisAngle`). A NaN `dp` takes the last branch.
pub fn hk_quaternion_remove_axis_component(e: &mut Engine, this: Ptr, axis: Ptr) {
    let q = load_vec(e, this.addr());
    let a = load_vec(e, axis.addr());
    let dot = sum_three(mul_ps(a, q));
    let cross_q = shufps(q, q, 0xc9);
    let shifted_q = shufps(q, q, 0xd2);
    let w = [q[3]; 4];
    let cross_a = shufps(a, a, 0xc9);
    let shifted_a = shufps(a, a, 0xd2);
    let from_shifted = mul_ps(shifted_q, cross_a);
    let cross = mul_ps(sub_ps(mul_ps(cross_q, shifted_a), from_shifted), w);
    let w_squared = add_ps(mul_ps(w, w), load_vec(e, VECTOR_MINUS_HALF));
    let along = mul_ps([dot; 4], q);
    let across = add_ps(mul_ps(w_squared, a), along);
    let half_rotated = add_ps(cross, across);
    let rotated = add_ps(half_rotated, half_rotated);
    let dp = sum_three(mul_ps(a, rotated));
    let dp_value = x87_load32(dp.to_bits());
    if x87_sub(dp_value, 1.0) > double_at(e, REMOVE_LOWER_LIMIT) {
        let identity = load_vec(e, VECTOR_IDENTITY);
        store_vec(e, this.addr(), identity);
        return;
    }
    if x87_add(1.0, dp_value) < double_at(e, REMOVE_UPPER_LIMIT) {
        let axis_bits = perpendicular(e, axis.addr());
        let angle = quiet_float_at(e, FLOAT_PI);
        e.with_stack(0x10, |e, buffer| {
            for (lane, bits) in axis_bits.into_iter().enumerate() {
                e.mem.set_u32(buffer.addr() + 4 * lane as u32, bits);
            }
            e.call(SET_AXIS_ANGLE, &args![this, buffer, angle]);
        });
        return;
    }
    let vector = sub_ps(
        mul_ps(shufps(rotated, rotated, 0xd2), cross_a),
        mul_ps(shufps(rotated, rotated, 0xc9), shifted_a),
    );
    let scale = refined_rsqrt(e, sum_three(mul_ps(vector, vector)));
    let normal = mul_ps([scale; 4], vector);
    let angle = x87_store32(
        e.call(MATH_ACOS, &args![x87_quiet_bits(dp.to_bits())])
            .f64(),
    );
    e.with_stack(0x10, |e, buffer| {
        store_vec(e, buffer.addr(), normal);
        e.call(SET_AXIS_ANGLE, &args![this, buffer, angle]);
    });
}

// Translated from 00cb2ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkQuaternion::decomposeRestAxis` (Xbox PDB), `(axis, rest, angle)`: copies this quaternion to
/// `rest` and removes the axis component from it ([`hk_quaternion_remove_axis_component`]);
/// `setMul` (`00c66320`) of the conjugate of `rest` (the sign bits at `010c4c00` xor-ed in) and this
/// quaternion gives the twist; `angle` receives its `getAngle` (`00c74c20`, stored as a float),
/// negated when `result.w * (axis . result)` is negative.
pub fn hk_quaternion_decompose_rest_axis(
    e: &mut Engine,
    this: Ptr,
    axis: Ptr,
    rest: Ptr,
    angle: Ptr,
) {
    let q = load_vec(e, this.addr());
    store_vec(e, rest.addr(), q);
    hk_quaternion_remove_axis_component(e, rest, axis);
    let signs = load_vec(e, VECTOR_SIGN_BITS);
    let rest_value = load_vec(e, rest.addr());
    let conjugate: Vec4 = std::array::from_fn(|lane| {
        f32::from_bits(rest_value[lane].to_bits() ^ signs[lane].to_bits())
    });
    e.with_stack(0x20, |e, frame| {
        let result = frame.addr();
        let operand = frame.addr() + 0x10;
        store_vec(e, operand, conjugate);
        e.call(SET_MUL, &args![result, operand, this]);
        let twist = x87_store32(e.call(GET_ANGLE, &args![result]).f64());
        let twist_bits = x87_quiet_bits(twist);
        e.mem.set_u32(angle.addr(), twist_bits);
        let result_value = load_vec(e, result);
        let a = load_vec(e, axis.addr());
        let w = result_value[3];
        let dot = sum_three(mul_ps(a, result_value));
        if w * dot < 0.0 {
            e.mem.set_u32(angle.addr(), twist_bits ^ 0x8000_0000);
        }
    });
}

/// `hkVector4::isOk4` (`00c7f500`, `this` = the vector, one stack word: the result byte), which
/// sets the byte to whether every lane is finite.
const VECTOR_IS_OK4: u32 = 0x00c7_f500;
/// `0.001` as a `float`: how far from 1 the squared length may be.
const IS_OK_LENGTH_TOLERANCE: u32 = 0x0101_7d00;

// Translated from 00cb24c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A quaternion check (the engine map gives no name): `result` points to a byte that
/// `hkVector4::isOk4` (`00c7f500`) first sets from the four lanes; the byte stays set only when
/// the squared length is within `0.001` of 1, and is cleared otherwise. Returns `result`.
///
/// The squared length is the `MULPS` / `SHUFPS 0x4e` / `0xb1` ladder in `float` lanes; the
/// difference from 1 and its absolute value go through the x87 stack with a `float` store at
/// each step.
pub fn fn_00cb24c0(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    e.call(VECTOR_IS_OK4, &args![this, result]);
    let lanes = load_vec(e, this.addr());
    let length_squared = sum_four(mul_ps(lanes, lanes));
    let difference = x87_store32(x87_sub(
        x87_load32(length_squared.to_bits()),
        double_at(e, DOUBLE_ONE),
    ));
    // `FABS` on the reloaded float, stored back as a float.
    let magnitude = f32::from_bits(x87_quiet_bits(difference) & 0x7fff_ffff);
    let within = magnitude < e.mem.f32(IS_OK_LENGTH_TOLERANCE);
    let ok = e.mem.u8(result.addr()) != 0 && within;
    e.mem.set_u8(result.addr(), ok as u8);
    result
}

/// `_CIcos` (`00ec9f70`, the argument in `ST0`, a leading `f64`).
const CI_COS: u32 = 0x00ec_9f70;

// Translated from 00cb2450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkQuaternion::setAxisAngle` (Xbox PDB): the rotation of `angle` radians about the unit
/// vector `axis` (four lanes, 16-byte aligned): `(sin(angle / 2) * axis, cos(angle / 2))`.
///
/// The half angle, the sine and the cosine each round to `float` on the way (x87 stores); the
/// four lanes of the product are `MULPS` on the sine broadcast, then the cosine replaces lane 3.
pub fn hk_quaternion_set_axis_angle(e: &mut Engine, this: Ptr, axis: Ptr, angle: f32) {
    let half = x87_store32(x87_mul(
        x87_load32(angle.to_bits()),
        double_at(e, DOUBLE_HALF),
    ));
    let sine = x87_store32(e.call(CI_SIN, &args![x87_load32(half)]).f64());
    let sine = f32::from_bits(x87_quiet_bits(sine));
    let axis_lanes = load_vec(e, axis.addr());
    store_vec(e, this.addr(), mul_ps([sine; 4], axis_lanes));
    let cosine = x87_store32(e.call(CI_COS, &args![x87_load32(half)]).f64());
    e.mem.set_u32(this.addr() + 0xc, x87_quiet_bits(cosine));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_2450, hk_quaternion_set_axis_angle(Ptr, Ptr, f32)),
        entry!(0x00cb_24c0, fn_00cb24c0(Ptr, Ptr) -> Ptr),
        entry!(0x00cb_2550, quaternion_from_rotation(Ptr, Ptr)),
        entry!(0x00cb_26e0, hk_quaternion_set(Ptr, Ptr)),
        entry!(0x00cb_2700, fn_00cb2700(Ptr, Ptr)),
        entry!(0x00cb_2830, hk_quaternion_set_and_normalize(Ptr, Ptr)),
        entry!(0x00cb_2850, hk_quaternion_set_slerp(Ptr, Ptr, Ptr, f32)),
        entry!(0x00cb_2a50, hk_quaternion_remove_axis_component(Ptr, Ptr)),
        entry!(
            0x00cb_2ca0,
            hk_quaternion_decompose_rest_axis(Ptr, Ptr, Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the image constants the translations read.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e.mem.set_u64(DOUBLE_ONE, 1.0f64.to_bits());
        e.mem.set_u64(DOUBLE_HALF, 0.5f64.to_bits());
        e.mem.set_u64(SLERP_THRESHOLD, 0.999f64.to_bits());
        e.mem.set_u64(REMOVE_LOWER_LIMIT, (-0.001f64).to_bits());
        e.mem.set_u64(REMOVE_UPPER_LIMIT, 0.001f64.to_bits());
        e.mem.set_f32(FLOAT_ZERO, 0.0);
        e.mem.set_f32(FLOAT_THREE, 3.0);
        e.mem.set_f32(FLOAT_HALF, 0.5);
        e.mem.set_f32(FLOAT_MINUS_ONE, -1.0);
        e.mem.set_f32(FLOAT_PI, std::f32::consts::PI);
        store_vec(&mut e, VECTOR_MINUS_HALF, [-0.5; 4]);
        store_vec(&mut e, VECTOR_IDENTITY, [0.0, 0.0, 0.0, 1.0]);
        for (lane, bits) in [0x8000_0000u32, 0x8000_0000, 0x8000_0000, 0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(VECTOR_SIGN_BITS + 4 * lane as u32, bits);
        }
        e.register(MATH_ACOS, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).acos(),
            ..Ret::default()
        });
        e
    }

    fn vec_at(e: &mut Engine, lanes: Vec4) -> u32 {
        let address = e.mem.alloc(0x10);
        store_vec(e, address, lanes);
        address
    }

    fn close(actual: Vec4, expected: Vec4) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() < 1e-4,
                "lane {lane}: {actual:?} against {expected:?}"
            );
        }
    }

    /// A rotation matrix of three columns of three floats (the fourth lane stays 0).
    fn rotation(e: &mut Engine, columns: [[f32; 3]; 3]) -> u32 {
        let matrix = e.mem.alloc(0x30);
        for (column, values) in columns.iter().enumerate() {
            for (row, value) in values.iter().enumerate() {
                e.mem
                    .set_f32(matrix + 16 * column as u32 + 4 * row as u32, *value);
            }
        }
        matrix
    }

    #[test]
    fn quaternion_from_rotation_by_trace() {
        let mut e = engine();
        let out = vec_at(&mut e, [9.0; 4]);
        let identity = rotation(&mut e, [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
        e.call(0x00cb_2550, &args![identity, out]);
        close(load_vec(&e, out), [0.0, 0.0, 0.0, 1.0]);
        // 90 degrees about Z.
        let turned = rotation(&mut e, [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
        e.call(0x00cb_2550, &args![turned, out]);
        let h = std::f32::consts::FRAC_1_SQRT_2;
        close(load_vec(&e, out), [0.0, 0.0, h, h]);
    }

    #[test]
    fn quaternion_from_rotation_by_largest_diagonal() {
        let mut e = engine();
        let out = vec_at(&mut e, [9.0; 4]);
        // 180 degrees about X, Y and Z: the trace is -1 and the biggest diagonal element picks
        // the axis.
        let about_x = rotation(
            &mut e,
            [[1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, -1.0]],
        );
        e.call(0x00cb_2550, &args![about_x, out]);
        close(load_vec(&e, out), [1.0, 0.0, 0.0, 0.0]);
        let about_y = rotation(
            &mut e,
            [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]],
        );
        e.call(0x00cb_2550, &args![about_y, out]);
        close(load_vec(&e, out), [0.0, 1.0, 0.0, 0.0]);
        let about_z = rotation(
            &mut e,
            [[-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0]],
        );
        e.call(0x00cb_2550, &args![about_z, out]);
        close(load_vec(&e, out), [0.0, 0.0, 1.0, 0.0]);
        // 120 degrees about (1, 1, 1): the trace is exactly 0, which is not above 0.
        let cycle = rotation(&mut e, [[0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]]);
        e.call(0x00cb_2550, &args![cycle, out]);
        close(load_vec(&e, out), [0.5, 0.5, 0.5, 0.5]);
    }

    #[test]
    fn set_matches_the_conversion_and_set_and_normalize_normalizes_afterwards() {
        let mut e = engine();
        let matrix = rotation(&mut e, [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]]);
        let direct = vec_at(&mut e, [0.0; 4]);
        e.call(0x00cb_2550, &args![matrix, direct]);
        let via_set = vec_at(&mut e, [0.0; 4]);
        e.call(0x00cb_26e0, &args![via_set, matrix]);
        assert_eq!(
            load_vec(&e, via_set).map(f32::to_bits),
            load_vec(&e, direct).map(f32::to_bits)
        );
        let log = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(NORMALIZE, move |_, a| {
            seen.borrow_mut().push(a[0]);
            Ret::default()
        });
        let normalized = vec_at(&mut e, [0.0; 4]);
        e.call(0x00cb_2830, &args![normalized, matrix]);
        assert_eq!(
            load_vec(&e, normalized).map(f32::to_bits),
            load_vec(&e, direct).map(f32::to_bits)
        );
        assert_eq!(*log.borrow(), vec![normalized]);
    }

    #[test]
    fn fn_00cb2700_makes_a_unit_perpendicular() {
        let mut e = engine();
        let out = vec_at(&mut e, [9.0; 4]);
        let along_x = vec_at(&mut e, [1.0, 0.0, 0.0, 0.0]);
        e.call(0x00cb_2700, &args![out, along_x]);
        close(load_vec(&e, out), [0.0, 0.0, -1.0, 0.0]);
        let along_z = vec_at(&mut e, [0.0, 0.0, 2.0, 0.0]);
        e.call(0x00cb_2700, &args![out, along_z]);
        close(load_vec(&e, out), [0.0, 1.0, 0.0, 0.0]);
        // For (3, -4, 1) the two largest components are swapped with a sign change.
        let skew = vec_at(&mut e, [3.0, -4.0, 1.0, 0.0]);
        e.call(0x00cb_2700, &args![out, skew]);
        let result = load_vec(&e, out);
        // Lane 3 is 0 and the result is perpendicular to the input.
        assert_eq!(result[3], 0.0);
        let dot = result[0] * 3.0 + result[1] * -4.0 + result[2];
        assert!(dot.abs() < 1e-4);
        let length = (result[0] * result[0] + result[1] * result[1] + result[2] * result[2]).sqrt();
        assert!((length - 1.0).abs() < 1e-3);
    }

    #[test]
    fn set_slerp_interpolates_on_the_arc_and_blends_close_quaternions() {
        let mut e = engine();
        let h = std::f32::consts::FRAC_1_SQRT_2;
        let from = vec_at(&mut e, [0.0, 0.0, 0.0, 1.0]);
        let to = vec_at(&mut e, [0.0, 0.0, h, h]);
        let out = vec_at(&mut e, [9.0; 4]);
        e.call(0x00cb_2850, &args![out, from, to, 0.5f32]);
        // Half way along the arc: 45 degrees about Z.
        close(load_vec(&e, out), [0.0, 0.0, 0.382_683_4, 0.923_879_5]);
        e.call(0x00cb_2850, &args![out, from, to, 0.0f32]);
        close(load_vec(&e, out), [0.0, 0.0, 0.0, 1.0]);
        e.call(0x00cb_2850, &args![out, from, to, 1.0f32]);
        close(load_vec(&e, out), [0.0, 0.0, h, h]);
        // Nearly the same quaternion (dot above 0.999): linear blend.
        let near = vec_at(&mut e, [0.0, 0.0, 0.02, 0.9998]);
        e.call(0x00cb_2850, &args![out, from, near, 0.5f32]);
        close(load_vec(&e, out), [0.0, 0.0, 0.01, 0.99995]);
        // The same rotation with the opposite sign: the blend takes the short way.
        let negated = vec_at(&mut e, [0.0, 0.0, 0.0, -1.0]);
        e.call(0x00cb_2850, &args![out, from, negated, 0.5f32]);
        close(load_vec(&e, out), [0.0, 0.0, 0.0, 1.0]);
        // A NaN dot product takes the linear branch (no panic, NaN result).
        let broken = vec_at(&mut e, [f32::NAN, 0.0, 0.0, 1.0]);
        e.call(0x00cb_2850, &args![out, from, broken, 0.5f32]);
        assert!(load_vec(&e, out).iter().any(|lane| lane.is_nan()));
    }

    type AxisAngleLog = Rc<RefCell<Vec<(u32, Vec4, f32)>>>;

    /// Doubles for `setAxisAngle`: logs the quaternion address, the axis and the angle.
    fn axis_angle_log(e: &mut Engine) -> AxisAngleLog {
        let log = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&log);
        e.register_double(SET_AXIS_ANGLE, move |e, a| {
            seen.borrow_mut()
                .push((a[0], load_vec(e, a[1]), f32::from_bits(a[2])));
            Ret::default()
        });
        log
    }

    #[test]
    fn remove_axis_component_of_a_twist_about_the_axis_is_the_identity() {
        let mut e = engine();
        let h = std::f32::consts::FRAC_1_SQRT_2;
        let this = vec_at(&mut e, [0.0, 0.0, h, h]);
        let axis = vec_at(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let log = axis_angle_log(&mut e);
        e.call(0x00cb_2a50, &args![this, axis]);
        close(load_vec(&e, this), [0.0, 0.0, 0.0, 1.0]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn remove_axis_component_of_a_turn_across_the_axis_rotates_about_a_normal() {
        let mut e = engine();
        let h = std::f32::consts::FRAC_1_SQRT_2;
        // 90 degrees about X turns Z to -Y: the axes are perpendicular (dp = 0).
        let this = vec_at(&mut e, [h, 0.0, 0.0, h]);
        let axis = vec_at(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let log = axis_angle_log(&mut e);
        e.call(0x00cb_2a50, &args![this, axis]);
        let log = log.borrow();
        assert_eq!(log.len(), 1);
        let (quaternion, normal, angle) = log[0];
        assert_eq!(quaternion, this);
        assert!((normal[0].abs() - 1.0).abs() < 1e-3);
        assert!(normal[1].abs() < 1e-3 && normal[2].abs() < 1e-3 && normal[3].abs() < 1e-3);
        assert!((angle - std::f32::consts::FRAC_PI_2).abs() < 1e-3);
    }

    #[test]
    fn remove_axis_component_of_opposite_axes_rotates_by_pi_about_a_perpendicular() {
        let mut e = engine();
        // 180 degrees about X turns Z to -Z.
        let this = vec_at(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let axis = vec_at(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let log = axis_angle_log(&mut e);
        e.call(0x00cb_2a50, &args![this, axis]);
        let log = log.borrow();
        assert_eq!(log.len(), 1);
        let (quaternion, perpendicular, angle) = log[0];
        assert_eq!(quaternion, this);
        // The perpendicular of (0, 0, 1) has 1 in lane 1 and -0 in lane 2.
        assert_eq!(perpendicular[0], 0.0);
        assert_eq!(perpendicular[1], 1.0);
        assert_eq!(perpendicular[2], 0.0);
        assert_eq!(angle, std::f32::consts::PI);
    }

    #[test]
    fn decompose_rest_axis_returns_the_signed_twist() {
        for (quaternion, expected) in [
            ([0.0, 0.0, 0.6, 0.8], 1.0f32),
            ([0.0, 0.0, -0.6, 0.8], -1.0f32),
        ] {
            let mut e = engine();
            // The twist is `setMul`'s result, here a copy of this quaternion; its angle 1.0.
            e.register(SET_MUL, |e, a| {
                let q = load_vec(e, a[2]);
                store_vec(e, a[0], q);
                Ret::default()
            });
            e.register(GET_ANGLE, |_, _| Ret {
                st0: 1.0,
                ..Ret::default()
            });
            let this = vec_at(&mut e, quaternion);
            let axis = vec_at(&mut e, [0.0, 0.0, 1.0, 0.0]);
            let rest = vec_at(&mut e, [9.0; 4]);
            let angle = e.mem.alloc(4);
            e.call(0x00cb_2ca0, &args![this, axis, rest, angle]);
            assert_eq!(e.mem.f32(angle), expected);
            // The rest is the quaternion with the axis component removed: the identity.
            close(load_vec(&e, rest), [0.0, 0.0, 0.0, 1.0]);
        }
    }

    #[test]
    fn fn_00cb24c0_accepts_unit_quaternions_only() {
        let mut e = engine();
        e.register(VECTOR_IS_OK4, |e, a| {
            // Finite lanes: the byte is set.
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        e.mem.map(0x0200_0000, 0x100);
        e.mem.set_f32(IS_OK_LENGTH_TOLERANCE, 0.001);
        let result: Ptr = Ptr::new(0x0200_0000);
        let unit = vec_at(&mut e, [0.0, 0.6, 0.0, 0.8]);
        let back = e.call(0x00cb_24c0, &args![unit, result]).ptr::<()>();
        assert_eq!(back, result);
        assert_eq!(e.mem.u8(result.addr()), 1);
        let long = vec_at(&mut e, [0.0, 0.6, 0.0, 0.9]);
        e.call(0x00cb_24c0, &args![long, result]);
        assert_eq!(e.mem.u8(result.addr()), 0);
        // Just outside the tolerance (0.001 away).
        let edge = vec_at(&mut e, [0.0, 0.0, 0.0, 1.0005]);
        e.call(0x00cb_24c0, &args![edge, result]);
        assert_eq!(e.mem.u8(result.addr()), 0);
        let near = vec_at(&mut e, [0.0, 0.0, 0.0, 1.0004]);
        e.call(0x00cb_24c0, &args![near, result]);
        assert_eq!(e.mem.u8(result.addr()), 1);
    }

    #[test]
    fn fn_00cb24c0_clears_the_byte_when_the_vector_check_fails_or_a_lane_is_nan() {
        let mut e = engine();
        e.register(VECTOR_IS_OK4, |e, a| {
            e.mem.set_u8(a[1], 0);
            Ret::default()
        });
        e.mem.map(0x0200_0000, 0x100);
        e.mem.set_f32(IS_OK_LENGTH_TOLERANCE, 0.001);
        let result: Ptr = Ptr::new(0x0200_0000);
        let unit = vec_at(&mut e, [0.0, 0.0, 0.0, 1.0]);
        e.call(0x00cb_24c0, &args![unit, result]);
        assert_eq!(e.mem.u8(result.addr()), 0);
        e.register(VECTOR_IS_OK4, |e, a| {
            e.mem.set_u8(a[1], 1);
            Ret::default()
        });
        let not_a_number = vec_at(&mut e, [f32::NAN, 0.0, 0.0, 1.0]);
        e.call(0x00cb_24c0, &args![not_a_number, result]);
        assert_eq!(e.mem.u8(result.addr()), 0);
    }

    #[test]
    fn set_axis_angle_builds_the_half_angle_quaternion() {
        let mut e = engine();
        e.register(CI_SIN, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sin(),
            ..Ret::default()
        });
        e.register(CI_COS, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).cos(),
            ..Ret::default()
        });
        let axis = vec_at(&mut e, [0.0, 1.0, 0.0, 0.0]);
        let result = vec_at(&mut e, [9.0; 4]);
        e.call(
            0x00cb_2450,
            &args![result, axis, std::f32::consts::FRAC_PI_2],
        );
        let lanes = load_vec(&e, result);
        let half_sqrt = std::f32::consts::FRAC_1_SQRT_2;
        close(lanes, [0.0, half_sqrt, 0.0, half_sqrt]);
        // The sine multiplies the axis's fourth lane too, before the cosine replaces it.
        let odd_axis = vec_at(&mut e, [1.0, 0.0, 0.0, 5.0]);
        e.call(0x00cb_2450, &args![result, odd_axis, 0.0f32]);
        assert_eq!(load_vec(&e, result), [0.0, 0.0, 0.0, 1.0]);
    }
}

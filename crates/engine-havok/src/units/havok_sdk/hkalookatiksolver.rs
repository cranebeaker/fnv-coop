//! `Havok SDK/hkalookatiksolver.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The CRT helpers `00ec9f70` and `00eca0a0` (value in and out in `ST0`; the other
/// translations use them as cosine and sine, not confirmed here), the unit's helper `00ce0c40`
/// (cdecl, one float; result in `ST0`) and `00ce00f0` (cdecl, two floats; result in `ST0`).
const CRT_FIRST_TRIG: u32 = 0x00ec_9f70;
const CRT_SECOND_TRIG: u32 = 0x00ec_a0a0;
const RATIO_ANGLE: u32 = 0x00ce_0c40;
const PAIR_ANGLE: u32 = 0x00ce_00f0;
/// The float stored in the last lane of both results (`01011d78`, zero in the exe) and the
/// double a length is compared against (`01032980`, a float 0.0001 as a double).
const LAST_LANE: u32 = 0x0101_1d78;
const MINIMUM_LENGTH: u32 = 0x0103_2980;

// Translated from 00ce0110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one stack word `vector`; no PDB name): turns the vector `(r, a, b)` at
/// `vector` (the first three floats; each is copied through a load/store pair, which quiets a
/// signalling NaN) into `(cos(a) * (cos(b) * r), sin(a) * (cos(b) * r), sin(b) * r, w)` in
/// place, with `cos` and `sin` the two CRT helpers and `w` the float at `01011d78`. Every
/// product is rounded to a float.
pub fn fn_00ce0110(e: &mut Engine, vector: Ptr) {
    let at = vector.addr();
    let radius = x87_quiet(e.mem.f32(at));
    let azimuth = x87_quiet(e.mem.f32(at + 4));
    let elevation = x87_quiet(e.mem.f32(at + 8));
    // x87 `FMUL` with the x87 NaN choice (#199), rounded to a float at the store.
    let multiply = |first: f32, second: f32| {
        x87_mul(x87_m32(first.to_bits()), x87_m32(second.to_bits())) as f32
    };
    let cos_elevation = e.call(CRT_FIRST_TRIG, &args![f64::from(elevation)]).f32();
    let horizontal = multiply(cos_elevation, radius);
    let cos_azimuth = e.call(CRT_FIRST_TRIG, &args![f64::from(azimuth)]).f32();
    let x = multiply(cos_azimuth, horizontal);
    let sin_azimuth = e.call(CRT_SECOND_TRIG, &args![f64::from(azimuth)]).f32();
    let y = multiply(sin_azimuth, horizontal);
    let sin_elevation = e.call(CRT_SECOND_TRIG, &args![f64::from(elevation)]).f32();
    let z = multiply(sin_elevation, radius);
    let last = e.mem.u32(LAST_LANE);
    e.mem.set_f32(at, x);
    e.mem.set_f32(at + 4, y);
    e.mem.set_f32(at + 8, z);
    e.mem.set_u32(at + 12, last);
}

// Translated from 00ce01c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one stack word `vector`; no PDB name): the inverse of `fn_00ce0110`. With
/// `length` the square root of `z*z + (y*y + x*x)` (the products and sums as the SSE code
/// orders them), a length below the double at `01032980` (0.0001) makes the vector all zero;
/// otherwise it becomes `(length, pair_angle(y, x), ratio_angle(z / length), w)` in place,
/// with `w` the float at `01011d78`, `pair_angle` the helper at `00ce00f0` and `ratio_angle`
/// the one at `00ce0c40`. A NaN length is not below the limit.
pub fn fn_00ce01c0(e: &mut Engine, vector: Ptr) {
    let at = vector.addr();
    let [x, y, z] = [0, 4, 8].map(|offset| e.mem.f32(at + offset));
    let squared_sum = z * z + (y * y + x * x);
    let length = x87_quiet(squared_sum.sqrt());
    let limit = e.mem.f64(MINIMUM_LENGTH);
    let below_limit = matches!(
        f64::from(length).partial_cmp(&limit),
        Some(std::cmp::Ordering::Less)
    );
    if below_limit {
        for offset in [0, 4, 8, 12] {
            e.mem.set_f32(at + offset, 0.0);
        }
        return;
    }
    let x = x87_quiet(e.mem.f32(at));
    let y = x87_quiet(e.mem.f32(at + 4));
    // `z / length` with the x87 NaN choice (#199), rounded to a float.
    let ratio = x87_div(x87_m32(e.mem.u32(at + 8)), x87_m32(length.to_bits())) as f32;
    let phi = e.call(RATIO_ANGLE, &args![ratio]).f32();
    let theta = e.call(PAIR_ANGLE, &args![y, x]).f32();
    let last = e.mem.u32(LAST_LANE);
    e.mem.set_f32(at, length);
    e.mem.set_f32(at + 4, theta);
    e.mem.set_f32(at + 8, phi);
    e.mem.set_u32(at + 12, last);
}

/// Four packed floats of an XMM register.
type Lanes = [f32; 4];

/// What `solve` calls and reads. Functions: the CRT arc cosine `00eca1d0` and square root
/// `00ec6040` (value in and out in `ST0`), `hkQuaternion::setAxisAngle` (`00cb2450`, thiscall on
/// the result, `(axis, angle)`), `hkQuaternion::setMul` (`00c66320`, thiscall on the result,
/// `(first, second)`), the matrix transpose `00cd4ba0` (thiscall on the result, `(matrix)`),
/// `hkVector4::setMul3` (`00c7f750`, thiscall on the result, `(matrix, vector)`), the
/// quaternion helper `00ce0ca0` (thiscall on a quaternion, `(other)`) and the arc cosine
/// `hkMath::acos` (`00c74a70`, cdecl, one float, answers it in `ST0`). Data: the SSE vector of
/// four `-0.5` floats (`010c71e0`), the floats `01017718` (3.0), `01016248` (0.5), `01012068`
/// (1.0) and `0102b3c8` (pi), the largest positive and negative floats `0104da48` and
/// `0104e148` used as open bounds, the doubles `01011590` and `01011588` of the roll
/// estimate, and the compare mask `01017d10` with the tolerance `01017d00`.
const CRT_ARC_COSINE: u32 = 0x00ec_a1d0;
const CRT_SQUARE_ROOT: u32 = 0x00ec_6040;
const SET_AXIS_ANGLE: u32 = 0x00cb_2450;
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
const MATRIX_TRANSPOSE: u32 = 0x00cd_4ba0;
const VECTOR_SET_MUL3: u32 = 0x00c7_f750;
const QUATERNION_ROLL: u32 = 0x00ce_0ca0;
const HK_ARC_COSINE: u32 = 0x00c7_4a70;
const NEGATIVE_HALVES: u32 = 0x010c_71e0;
const ROOT_THREE: u32 = 0x0101_7718;
const ROOT_HALF: u32 = 0x0101_6248;
const UNIT_FLOAT: u32 = 0x0101_2068;
const PI_FLOAT: u32 = 0x0102_b3c8;
const OPEN_UPPER_BOUND: u32 = 0x0104_da48;
const OPEN_LOWER_BOUND: u32 = 0x0104_e148;
const ROLL_FIRST_FACTOR: u32 = 0x0101_1590;
const ROLL_SECOND_FACTOR: u32 = 0x0101_1588;
const ABSOLUTE_MASK: u32 = 0x0101_7d10;
const COMPARE_TOLERANCE: u32 = 0x0101_7d00;

fn load_lanes(e: &Engine, address: u32) -> Lanes {
    [0, 4, 8, 12].map(|offset| e.mem.f32(address + offset))
}

fn store_lanes(e: &mut Engine, address: u32, lanes: Lanes) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem.set_f32(address + 4 * lane as u32, value);
    }
}

fn copy_lanes(e: &mut Engine, from: u32, to: u32) {
    for offset in [0, 4, 8, 12] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
}

fn lanes_mul(a: Lanes, b: Lanes) -> Lanes {
    [0, 1, 2, 3].map(|lane| a[lane] * b[lane])
}

fn lanes_sub(a: Lanes, b: Lanes) -> Lanes {
    [0, 1, 2, 3].map(|lane| a[lane] - b[lane])
}

fn lanes_add(a: Lanes, b: Lanes) -> Lanes {
    [0, 1, 2, 3].map(|lane| a[lane] + b[lane])
}

/// `SHUFPS v, v, selector`: the lanes of `v` picked by the selector.
fn shuffle(v: Lanes, selector: u8) -> Lanes {
    [0, 2, 4, 6].map(|shift| v[usize::from((selector >> shift) & 3)])
}

/// The dot product `x2 + (x1 + x0)` of the three lowest lanes of a product.
fn sum_xyz(product: Lanes) -> f32 {
    product[2] + (product[1] + product[0])
}

/// `MINPS`: the first operand's lane when it is below the second's, else the second's.
fn lanes_min(a: Lanes, b: Lanes) -> Lanes {
    [0, 1, 2, 3].map(|lane| if a[lane] < b[lane] { a[lane] } else { b[lane] })
}

/// `MAXPS`: the first operand's lane when it is above the second's, else the second's.
fn lanes_max(a: Lanes, b: Lanes) -> Lanes {
    [0, 1, 2, 3].map(|lane| if a[lane] > b[lane] { a[lane] } else { b[lane] })
}

/// The cross product `a x b` the way the SSE code builds it: `a.yzx * b.zxy - a.zxy * b.yzx`.
fn cross_product(a: Lanes, b: Lanes) -> Lanes {
    lanes_sub(
        lanes_mul(shuffle(a, 0xc9), shuffle(b, 0xd2)),
        lanes_mul(shuffle(a, 0xd2), shuffle(b, 0xc9)),
    )
}

/// The vector scaled to unit length with the refined reciprocal root the SSE code uses: with
/// `s = z*z + (y*y + x*x)` and `r = RSQRTSS(s)` the scale is `(half * r) * (three - (s * r) * r)`,
/// 0 when `s` equals the float `01011d78`; all four lanes are multiplied by it.
fn unit_length(e: &Engine, vector: Lanes) -> Lanes {
    let sum = sum_xyz(lanes_mul(vector, vector));
    let fill = e.mem.f32(LAST_LANE);
    let three = e.mem.f32(ROOT_THREE);
    let half = e.mem.f32(ROOT_HALF);
    let estimate = crate::x87::rsqrt_estimate(sum);
    let refined = (half * estimate) * (three - (sum * estimate) * estimate);
    let scale = if sum == fill { 0.0 } else { refined };
    lanes_mul([scale; 4], vector)
}

/// The rotation of `vector` by `quaternion` the way the pose code does it: `cross * w +
/// ((w * w + -0.5) * vector + dot * quaternion)` doubled, with `cross` the cross product of
/// the quaternion's lanes and the vector.
fn rotate_vector(e: &Engine, vector: Lanes, quaternion: Lanes) -> Lanes {
    let dot = sum_xyz(lanes_mul(vector, quaternion));
    let cross = cross_product(quaternion, vector);
    let w = shuffle(quaternion, 0xff);
    let by_w = lanes_mul(cross, w);
    let halves = load_lanes(e, NEGATIVE_HALVES);
    let scaled = lanes_mul(lanes_add(lanes_mul(w, w), halves), vector);
    let along = lanes_mul([dot; 4], quaternion);
    let total = lanes_add(by_w, lanes_add(scaled, along));
    lanes_add(total, total)
}

/// The absolute value of the float at `address` after an `FLD`/`FSTP` round trip.
fn absolute_quiet(e: &Engine, address: u32) -> f32 {
    x87_quiet(e.mem.f32(address)).abs()
}

// Translated from 00ce0290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaLookAtIkSolver::solve` (Ghidra name), cdecl `(result, setup, target, gain, bone, limits)`;
/// answers `result`. `setup` holds three vectors (the bone's forward axis at `+0`, its roll
/// axis at `+0x10`, the reference direction at `+0x20`) and the largest angle at `+0x30`; `bone`
/// is a transform (translation at `+0`, quaternion at `+0x10`); `limits` is null or four floats
/// (`+0` and `+4` the upper and lower limits of the elevation, `+8` and `+0xc` of the azimuth,
/// then the vector `+0x10` the limits are measured around). The byte at `result` is set to 1,
/// and cleared when the direction had to be limited.
///
/// The direction `d` is the unit vector from the bone to the target. The largest angle is
/// the one of the setup, or the largest absolute value of the four limits when there are
/// any (an `FCOM` choice: the later value replaces the earlier unless it is below it). When
/// the cosine of the largest angle is above the reference direction's dot product with `d`, `d`
/// is replaced by the reference direction turned by that angle (`setAxisAngle`) towards it and
/// the byte is cleared. With limits, `d` is expressed in the matrix of the reference direction,
/// the reference direction's cross product with the limits' vector and that vector; it is
/// converted to spherical coordinates (`fn_00ce01c0`), the angles are clamped with `MINPS`
/// and `MAXPS` against the limits, and it is converted back (`fn_00ce0110`) and transformed by
/// the matrix; the byte is cleared when that moved a lane by more than the tolerance
/// `01017d00`. The forward axis rotated by the bone's quaternion `R2` is then turned towards `d`
/// by the angle between them times `gain` (the CRT arc cosine of a cosine whose absolute value is
/// below 1 or that is NaN; otherwise 0 for a cosine of 1 or more and pi for one of -1 or less)
/// as a quaternion `swing`. When the roll axis is not zero, a roll angle is estimated from the
/// distance to the target and the roll axis' components with the x87 polynomial and
/// `hkMath::acos`, and a roll quaternion about the cross product of `R2` and the
/// roll axis rotated by the bone is combined into `swing` (`00ce0ca0`). The bone's quaternion
/// becomes `swing * quaternion` (`setMul`).
///
/// Stack layout kept (`00ce0290`'s frame, the pointers to callees point into it): the direction
/// at `+0x40`, its copy at `+0x20`, the spherical vector at `+0x60` (the clamped one at `+0x50`),
/// the matrix at `+0xc0` and its transpose at `+0xf0`, the quaternions at `+0x60`, `+0x50`,
/// `+0xb0` and `+0x90`, the axes at `+0x50`, `+0x60` and `+0x90`.
pub fn hka_look_at_ik_solver_solve(
    e: &mut Engine,
    result: Ptr,
    setup: Ptr,
    target: Ptr,
    gain: f32,
    bone: Ptr,
    limits: Ptr,
) -> Ptr {
    let (setup, target, bone, limits) = (setup.addr(), target.addr(), bone.addr(), limits.addr());
    e.with_stack(0x130, |e, frame| {
        solve_in_frame(
            e,
            result.addr(),
            setup,
            target,
            gain,
            bone,
            limits,
            frame.addr(),
        );
    });
    result
}

#[allow(clippy::too_many_arguments)]
fn solve_in_frame(
    e: &mut Engine,
    result: u32,
    setup: u32,
    target: u32,
    gain: f32,
    bone: u32,
    limits: u32,
    frame: u32,
) {
    let mut max_angle = x87_quiet(e.mem.f32(setup + 0x30));
    if limits != 0 {
        let mut largest = absolute_quiet(e, limits + 8);
        for offset in [0xc, 0, 4] {
            let next = absolute_quiet(e, limits + offset);
            // `FCOM`: the earlier value stays only when the later one is below it.
            if !(next < largest) {
                largest = next;
            }
        }
        max_angle = largest;
    }
    let position = load_lanes(e, bone);
    let goal = load_lanes(e, target);
    let delta = lanes_sub(goal, position);
    let distance = sum_xyz(lanes_mul(delta, delta)).sqrt();
    let direction_at = frame + 0x40;
    let direction = unit_length(e, delta);
    store_lanes(e, direction_at, direction);
    let reference = load_lanes(e, setup + 0x20);
    let along = sum_xyz(lanes_mul(reference, direction));
    e.mem.set_u8(result, 1);
    let cosine_limit = e.call(CRT_FIRST_TRIG, &args![f64::from(max_angle)]).f32();
    // `FCOMP`: only an ordered "cosine of the limit above the dot product" turns the direction.
    if f64::from(cosine_limit) > f64::from(along) {
        let axis = unit_length(e, cross_product(reference, direction));
        store_lanes(e, frame + 0x50, axis);
        // The axis is the reference direction crossed with the direction.
        e.call(
            SET_AXIS_ANGLE,
            &args![frame + 0x60, frame + 0x50, max_angle.to_bits()],
        );
        let turn = load_lanes(e, frame + 0x60);
        let reference = load_lanes(e, setup + 0x20);
        let limited = rotate_vector(e, reference, turn);
        store_lanes(e, direction_at, limited);
        e.mem.set_u8(result, 0);
    }
    if limits != 0 {
        copy_lanes(e, direction_at, frame + 0x20);
        let reference = load_lanes(e, setup + 0x20);
        let around = load_lanes(e, limits + 0x10);
        store_lanes(e, frame + 0xc0, reference);
        store_lanes(e, frame + 0xd0, cross_product(around, reference));
        let around = load_lanes(e, limits + 0x10);
        store_lanes(e, frame + 0xe0, around);
        e.call(MATRIX_TRANSPOSE, &args![frame + 0xf0, frame + 0xc0]);
        e.call(
            VECTOR_SET_MUL3,
            &args![frame + 0x60, frame + 0xf0, direction_at],
        );
        fn_00ce01c0(e, Ptr::new(frame + 0x60));
        let fill = e.mem.f32(LAST_LANE);
        let [first, second, third, fourth] =
            [0, 4, 8, 0xc].map(|offset| e.mem.f32(limits + offset));
        let upper = [e.mem.f32(OPEN_UPPER_BOUND), third, first, fill];
        let lower = [e.mem.f32(OPEN_LOWER_BOUND), fourth, second, fill];
        let spherical = load_lanes(e, frame + 0x60);
        let clamped = lanes_max(lanes_min(spherical, upper), lower);
        store_lanes(e, frame + 0x50, clamped);
        fn_00ce0110(e, Ptr::new(frame + 0x50));
        e.call(
            VECTOR_SET_MUL3,
            &args![direction_at, frame + 0xc0, frame + 0x50],
        );
        let before = load_lanes(e, frame + 0x20);
        let after = load_lanes(e, direction_at);
        let mask = load_lanes(e, ABSOLUTE_MASK);
        let tolerance = e.mem.f32(COMPARE_TOLERANCE);
        let moved = lanes_sub(after, before);
        let difference: Lanes =
            [0, 1, 2, 3].map(|lane| f32::from_bits(moved[lane].to_bits() & mask[lane].to_bits()));
        if !(0..3).all(|lane| difference[lane] <= tolerance) {
            e.mem.set_u8(result, 0);
        }
    }
    // The forward axis turned by the bone's quaternion, and the swing towards the direction.
    let forward = load_lanes(e, setup);
    let rotation = load_lanes(e, bone + 0x10);
    let current = rotate_vector(e, forward, rotation);
    let direction = load_lanes(e, direction_at);
    let swing_axis = unit_length(e, cross_product(current, direction));
    store_lanes(e, frame + 0x60, swing_axis);
    let cosine = sum_xyz(lanes_mul(current, direction));
    // `FCOMP` / `TEST AH,1` (00ce0818..00ce0825): |cosine| below 1.0 or unordered (NaN) takes
    // the arc cosine; only an ordered |cosine| >= 1.0 picks 0 (positive cosine) or pi.
    let angle = if !(cosine.abs() >= e.mem.f32(UNIT_FLOAT)) {
        e.call(CRT_ARC_COSINE, &args![f64::from(x87_quiet(cosine))])
            .f32()
    } else if 0.0 < cosine {
        0.0
    } else {
        e.mem.f32(PI_FLOAT)
    };
    let swing_angle = x87_store32(x87_mul(
        x87_load32(angle.to_bits()),
        x87_m32(gain.to_bits()),
    ));
    e.call(
        SET_AXIS_ANGLE,
        &args![frame + 0x50, frame + 0x60, swing_angle],
    );
    let roll_axis = load_lanes(e, setup + 0x10);
    let roll_length = sum_xyz(lanes_mul(roll_axis, roll_axis)).sqrt();
    // `FCOMP`: the roll is only made for an ordered "zero below the length".
    if 0.0 < roll_length {
        let forward = load_lanes(e, setup);
        let side = unit_length(e, cross_product(forward, roll_axis));
        let up = unit_length(e, cross_product(side, forward));
        let squares = lanes_mul(up, up);
        let products = lanes_mul(up, roll_axis);
        let up_length = sum_xyz(squares).sqrt();
        let roll_ratio = sum_xyz(products) / up_length;
        let squared_distance = x87_store32(x87_mul(
            x87_load32(distance.to_bits()),
            x87_load32(distance.to_bits()),
        ));
        let squared_ratio = x87_store32(x87_mul(
            x87_load32(roll_ratio.to_bits()),
            x87_load32(roll_ratio.to_bits()),
        ));
        let gap = x87_store32(x87_sub(
            x87_load32(squared_distance),
            x87_m32(squared_ratio),
        ));
        let gap_root = e
            .call(
                CRT_SQUARE_ROOT,
                &args![f64::from(f32::from_bits(gap).abs())],
            )
            .f64();
        let root_bits = x87_store32(gap_root);
        let squared_root = x87_store32(x87_mul(x87_load32(root_bits), x87_load32(root_bits)));
        // The x87 expression over the squared distance `l`, the squared root `s` and the
        // squared ratio `t` (00ce0a3d..00ce0a7d): `((((l*l + (k*s*t - s*(k*l))) + (k*l)*t) + s*s)
        // + t*t)`. 00ce0a49 is `DC C9`, `FMUL ST(1),ST(0)`: `k*l` goes to ST1 and `k` stays on
        // top for `k*s` (00ce0a59); Ghidra lists it as `FMUL ST1` like the `D8` form, and reading
        // it as `ST0 *= ST1` once gave the head a roll of 78 degrees where the exe has 13.
        let factor: f64 = e.global(ROLL_FIRST_FACTOR);
        let l = x87_load32(squared_distance);
        let s = x87_load32(squared_root);
        let t = x87_load32(squared_ratio);
        let scaled_distance = x87_mul(l, factor);
        let product = x87_mul(x87_mul(factor, s), t);
        let difference = x87_sub(product, x87_mul(s, scaled_distance));
        let sum = x87_add(x87_mul(l, l), difference);
        let sum = x87_add(x87_mul(scaled_distance, t), sum);
        let sum = x87_add(sum, x87_mul(s, s));
        let total = x87_add(x87_mul(t, t), sum);
        let total_bits = x87_store32(total);
        let total_root = e
            .call(
                CRT_SQUARE_ROOT,
                &args![f64::from(f32::from_bits(total_bits).abs())],
            )
            .f64();
        let total_root_bits = x87_store32(total_root);
        let second_factor: f64 = e.global(ROLL_SECOND_FACTOR);
        let scaled = x87_mul(x87_load32(total_root_bits), second_factor);
        let ratio_bits = x87_store32(x87_div(scaled, x87_m32(root_bits)));
        // `FUCOM`: a ratio equal to 0.0 leaves the roll angle at 0.
        let roll_angle = if f32::from_bits(ratio_bits) == 0.0 {
            0.0f32
        } else {
            let quotient = x87_store32(x87_div(
                x87_m32(roll_ratio.to_bits()),
                x87_load32(ratio_bits),
            ));
            e.call(HK_ARC_COSINE, &args![quotient]).f32()
        };
        let roll_rotation = rotate_vector(e, roll_axis, load_lanes(e, bone + 0x10));
        let roll_cross = unit_length(e, cross_product(current, roll_rotation));
        store_lanes(e, frame + 0x90, roll_cross);
        e.call(
            SET_AXIS_ANGLE,
            &args![frame + 0xb0, frame + 0x90, (-roll_angle).to_bits()],
        );
        e.call(QUATERNION_ROLL, &args![frame + 0x50, frame + 0xb0]);
    }
    e.call(
        QUATERNION_SET_MUL,
        &args![frame + 0x90, frame + 0x50, bone + 0x10],
    );
    copy_lanes(e, frame + 0x90, bone + 0x10);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ce_0290,
            hka_look_at_ik_solver_solve(Ptr, Ptr, Ptr, f32, Ptr, Ptr) -> Ptr
        ),
        entry!(0x00ce_0110, fn_00ce0110(Ptr)),
        entry!(0x00ce_01c0, fn_00ce01c0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine with the four helpers as doubles: the first two return the cosine and sine
    /// of their `double` argument, `00ce0c40` returns twice its float argument and `00ce00f0`
    /// its first argument minus its second, all logged.
    fn engine() -> (Engine, Calls) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(MINIMUM_LENGTH, f64::from(0.0001f32));
        let calls: Calls = Rc::default();
        for address in [CRT_FIRST_TRIG, CRT_SECOND_TRIG, RATIO_ANGLE, PAIR_ANGLE] {
            let calls = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                calls.borrow_mut().push((address, a.to_vec()));
                let wide = f64::from_bits(
                    u64::from(a[0]) | (u64::from(a.get(1).copied().unwrap_or(0)) << 32),
                );
                let first = f32::from_bits(a[0]);
                let value = match address {
                    CRT_FIRST_TRIG => wide.cos(),
                    CRT_SECOND_TRIG => wide.sin(),
                    RATIO_ANGLE => f64::from(first * 2.0),
                    _ => f64::from(first - f32::from_bits(a[1])),
                };
                Ret {
                    st0: value,
                    ..Ret::default()
                }
            });
        }
        (e, calls)
    }

    fn vector(e: &mut Engine, lanes: [f32; 4]) -> u32 {
        let at = e.mem.alloc(16);
        for (lane, value) in lanes.iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, *value);
        }
        at
    }

    fn lanes(e: &Engine, at: u32) -> [u32; 4] {
        [0, 4, 8, 12].map(|offset| e.mem.u32(at + offset))
    }

    #[test]
    fn spherical_to_cartesian_uses_the_helpers_in_the_exe_order() {
        let (mut e, calls) = engine();
        let at = vector(&mut e, [2.0, 0.5, 0.25, 9.0]);
        e.mem.set_u32(LAST_LANE, 0x3f00_0000);
        fn_00ce0110(&mut e, Ptr::new(at));
        let cos_elevation = f64::from(0.25f32).cos() as f32;
        let horizontal = cos_elevation * 2.0;
        let x = f64::from(0.5f32).cos() as f32 * horizontal;
        let y = f64::from(0.5f32).sin() as f32 * horizontal;
        let z = f64::from(0.25f32).sin() as f32 * 2.0;
        assert_eq!(
            lanes(&e, at),
            [x.to_bits(), y.to_bits(), z.to_bits(), 0x3f00_0000]
        );
        let order: Vec<u32> = calls.borrow().iter().map(|(address, _)| *address).collect();
        assert_eq!(
            order,
            vec![
                CRT_FIRST_TRIG,
                CRT_FIRST_TRIG,
                CRT_SECOND_TRIG,
                CRT_SECOND_TRIG
            ]
        );
        // The helpers get the angles as doubles: elevation, azimuth, azimuth, elevation.
        let angles: Vec<f64> = calls
            .borrow()
            .iter()
            .map(|(_, a)| f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)))
            .collect();
        assert_eq!(angles, vec![0.25, 0.5, 0.5, 0.25]);
    }

    #[test]
    fn spherical_to_cartesian_quiets_signalling_nans() {
        let (mut e, _) = engine();
        let at = e.mem.alloc(16);
        e.mem.set_u32(at, 0x7f80_0001);
        e.mem.set_f32(at + 4, 0.0);
        e.mem.set_f32(at + 8, 0.0);
        fn_00ce0110(&mut e, Ptr::new(at));
        // cos(0) * radius is NaN; the quiet copy of the radius comes out in x, y and z.
        let result = lanes(&e, at);
        assert_eq!(result[0], 0x7fc0_0001);
        assert_eq!(result[1], 0x7fc0_0001);
        assert_eq!(result[2], 0x7fc0_0001);
    }

    #[test]
    fn cartesian_to_spherical_computes_length_and_angles() {
        let (mut e, calls) = engine();
        let at = vector(&mut e, [3.0, 4.0, 12.0, 77.0]);
        e.mem.set_u32(LAST_LANE, 0x3f80_0000);
        fn_00ce01c0(&mut e, Ptr::new(at));
        let length = (12.0f32 * 12.0 + (4.0 * 4.0 + 3.0 * 3.0)).sqrt();
        assert_eq!(length, 13.0);
        // The ratio helper doubles its argument, the pair helper subtracts.
        let ratio = 12.0f32 / 13.0;
        assert_eq!(
            lanes(&e, at),
            [
                13.0f32.to_bits(),
                (4.0f32 - 3.0).to_bits(),
                (ratio * 2.0).to_bits(),
                0x3f80_0000
            ]
        );
        assert_eq!(
            *calls.borrow(),
            vec![
                (RATIO_ANGLE, vec![ratio.to_bits()]),
                (PAIR_ANGLE, vec![4.0f32.to_bits(), 3.0f32.to_bits()]),
            ]
        );
    }

    #[test]
    fn a_short_vector_becomes_zero() {
        let (mut e, calls) = engine();
        let at = vector(&mut e, [0.00005, 0.0, 0.0, 5.0]);
        fn_00ce01c0(&mut e, Ptr::new(at));
        assert_eq!(lanes(&e, at), [0; 4]);
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn a_vector_just_above_the_limit_is_converted() {
        let (mut e, calls) = engine();
        let at = vector(&mut e, [0.0002, 0.0, 0.0, 5.0]);
        fn_00ce01c0(&mut e, Ptr::new(at));
        assert_eq!(calls.borrow().len(), 2);
        assert_eq!(e.mem.f32(at), 0.0002f32.abs());
    }

    #[test]
    fn a_nan_length_is_not_below_the_limit() {
        let (mut e, calls) = engine();
        let at = vector(&mut e, [f32::NAN, 1.0, 1.0, 5.0]);
        fn_00ce01c0(&mut e, Ptr::new(at));
        assert_eq!(calls.borrow().len(), 2);
        assert!(e.mem.f32(at).is_nan());
    }
}

#[cfg(test)]
mod solve_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Seen = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    struct Rig {
        e: Engine,
        seen: Seen,
        result: u32,
        setup: u32,
        target: u32,
        bone: u32,
        limits: u32,
    }

    fn put(e: &mut Engine, address: u32, lanes: [f32; 4]) {
        for (lane, value) in lanes.into_iter().enumerate() {
            e.mem.set_f32(address + 4 * lane as u32, value);
        }
    }

    fn get(e: &Engine, address: u32) -> [f32; 4] {
        [0, 4, 8, 12].map(|offset| e.mem.f32(address + offset))
    }

    fn wide(a: &[u32]) -> f64 {
        f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32))
    }

    /// The solver with every callee a double: the CRT functions compute for real, the
    /// quaternion, matrix and vector helpers do the maths of their names, and every call is
    /// logged with its arguments (and, for `setAxisAngle`, the axis lanes).
    fn rig() -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        for lane in 0..4 {
            e.mem.set_f32(NEGATIVE_HALVES + 4 * lane, -0.5);
            e.mem.set_u32(ABSOLUTE_MASK + 4 * lane, 0x7fff_ffff);
        }
        e.set_global(ROOT_THREE, 3.0f32);
        e.set_global(ROOT_HALF, 0.5f32);
        e.set_global(UNIT_FLOAT, 1.0f32);
        e.set_global(PI_FLOAT, std::f32::consts::PI);
        e.set_global(OPEN_UPPER_BOUND, f32::MAX);
        e.set_global(OPEN_LOWER_BOUND, -f32::MAX);
        e.set_global(ROLL_FIRST_FACTOR, 0.0f64);
        e.set_global(ROLL_SECOND_FACTOR, 1.0f64);
        e.set_global(COMPARE_TOLERANCE, 0.001f32);
        e.mem.set_f64(MINIMUM_LENGTH, f64::from(0.0001f32));
        let seen: Seen = Rc::default();
        for address in [
            CRT_FIRST_TRIG,
            CRT_SECOND_TRIG,
            CRT_ARC_COSINE,
            CRT_SQUARE_ROOT,
        ] {
            let log = Rc::clone(&seen);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                let value = wide(a);
                let answer = match address {
                    CRT_FIRST_TRIG => value.cos(),
                    CRT_SECOND_TRIG => value.sin(),
                    CRT_ARC_COSINE => value.acos(),
                    _ => value.sqrt(),
                };
                Ret {
                    st0: answer,
                    ..Ret::default()
                }
            });
        }
        let log = Rc::clone(&seen);
        e.register_double(RATIO_ANGLE, move |_, a| {
            log.borrow_mut().push((RATIO_ANGLE, a.to_vec()));
            Ret {
                st0: f64::from(f32::from_bits(a[0])).asin(),
                ..Ret::default()
            }
        });
        let log = Rc::clone(&seen);
        e.register_double(PAIR_ANGLE, move |_, a| {
            log.borrow_mut().push((PAIR_ANGLE, a.to_vec()));
            Ret {
                st0: f64::from(f32::from_bits(a[0])).atan2(f64::from(f32::from_bits(a[1]))),
                ..Ret::default()
            }
        });
        let log = Rc::clone(&seen);
        e.register_double(HK_ARC_COSINE, move |_, a| {
            log.borrow_mut().push((HK_ARC_COSINE, a.to_vec()));
            Ret {
                st0: f64::from(f32::from_bits(a[0])).acos(),
                ..Ret::default()
            }
        });
        let log = Rc::clone(&seen);
        e.register_double(SET_AXIS_ANGLE, move |e, a| {
            let axis = get(e, a[1]);
            let mut words = a.to_vec();
            words.extend(axis.map(f32::to_bits));
            log.borrow_mut().push((SET_AXIS_ANGLE, words));
            let half = f32::from_bits(a[2]) / 2.0;
            let (sine, cosine) = (half.sin(), half.cos());
            put(
                e,
                a[0],
                [axis[0] * sine, axis[1] * sine, axis[2] * sine, cosine],
            );
            Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(QUATERNION_SET_MUL, move |e, a| {
            log.borrow_mut().push((QUATERNION_SET_MUL, a.to_vec()));
            let (p, q) = (get(e, a[1]), get(e, a[2]));
            put(
                e,
                a[0],
                [
                    p[3] * q[0] + p[0] * q[3] + p[1] * q[2] - p[2] * q[1],
                    p[3] * q[1] - p[0] * q[2] + p[1] * q[3] + p[2] * q[0],
                    p[3] * q[2] + p[0] * q[1] - p[1] * q[0] + p[2] * q[3],
                    p[3] * q[3] - p[0] * q[0] - p[1] * q[1] - p[2] * q[2],
                ],
            );
            Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(MATRIX_TRANSPOSE, move |e, a| {
            log.borrow_mut().push((MATRIX_TRANSPOSE, a.to_vec()));
            let columns = [0, 0x10, 0x20].map(|offset| get(e, a[1] + offset));
            for (row, offset) in [0u32, 0x10, 0x20].into_iter().enumerate() {
                put(
                    e,
                    a[0] + offset,
                    [columns[0][row], columns[1][row], columns[2][row], 0.0],
                );
            }
            Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(VECTOR_SET_MUL3, move |e, a| {
            log.borrow_mut().push((VECTOR_SET_MUL3, a.to_vec()));
            let columns = [0, 0x10, 0x20].map(|offset| get(e, a[1] + offset));
            let v = get(e, a[2]);
            let lane =
                |i: usize| columns[0][i] * v[0] + columns[1][i] * v[1] + columns[2][i] * v[2];
            put(e, a[0], [lane(0), lane(1), lane(2), 0.0]);
            Ret::default()
        });
        let log = Rc::clone(&seen);
        e.register_double(QUATERNION_ROLL, move |e, a| {
            let mut words = a.to_vec();
            words.extend(get(e, a[0]).map(f32::to_bits));
            words.extend(get(e, a[1]).map(f32::to_bits));
            log.borrow_mut().push((QUATERNION_ROLL, words));
            Ret::default()
        });
        let result = e.mem.alloc(4);
        let setup = e.mem.alloc(0x40);
        let target = e.mem.alloc(0x10);
        let bone = e.mem.alloc(0x40);
        let limits = e.mem.alloc(0x20);
        put(&mut e, bone + 0x10, [0.0, 0.0, 0.0, 1.0]);
        Rig {
            e,
            seen,
            result,
            setup,
            target,
            bone,
            limits,
        }
    }

    fn solve(r: &mut Rig, gain: f32, limits: u32) -> u32 {
        r.e.call(
            0x00ce_0290,
            &args![r.result, r.setup, r.target, gain, r.bone, limits],
        )
        .u32()
    }

    fn calls(seen: &Seen, address: u32) -> Vec<Vec<u32>> {
        seen.borrow()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn close(actual: [f32; 4], expected: [f32; 4], tolerance: f32) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() < tolerance,
                "{actual:?} against {expected:?}"
            );
        }
    }

    fn axis_of(call: &[u32]) -> [f32; 4] {
        [3, 4, 5, 6].map(|word| f32::from_bits(call[word]))
    }

    /// The forward axis along x, the reference direction along y, an angle limit of 1.
    fn plain() -> Rig {
        let mut r = rig();
        put(&mut r.e, r.setup, [1.0, 0.0, 0.0, 0.0]);
        put(&mut r.e, r.setup + 0x10, [0.0; 4]);
        put(&mut r.e, r.setup + 0x20, [0.0, 1.0, 0.0, 0.0]);
        r.e.mem.set_f32(r.setup + 0x30, 1.0);
        put(&mut r.e, r.bone, [0.0; 4]);
        put(&mut r.e, r.target, [0.0, 5.0, 0.0, 0.0]);
        r
    }

    #[test]
    fn a_direction_inside_the_cone_is_swung_to_and_composed_with_the_bone() {
        let mut r = plain();
        r.e.mem.set_u8(r.result, 0x77);
        assert_eq!(solve(&mut r, 1.0, 0), r.result);
        assert_eq!(r.e.mem.u8(r.result), 1);
        // The cosine of the limit is asked for: cos(1.0).
        assert_eq!(wide(&calls(&r.seen, CRT_FIRST_TRIG)[0]), 1.0);
        // The swing from the forward axis (1, 0, 0) to (0, 1, 0): a quarter turn about z.
        let swings = calls(&r.seen, SET_AXIS_ANGLE);
        assert_eq!(swings.len(), 1);
        close(axis_of(&swings[0]), [0.0, 0.0, 1.0, 0.0], 1e-3);
        assert!((f32::from_bits(swings[0][2]) - std::f32::consts::FRAC_PI_2).abs() < 1e-5);
        let half = std::f32::consts::FRAC_1_SQRT_2;
        close(get(&r.e, r.bone + 0x10), [0.0, 0.0, half, half], 1e-3);
        assert!(calls(&r.seen, QUATERNION_ROLL).is_empty());
        assert_eq!(calls(&r.seen, QUATERNION_SET_MUL).len(), 1);
    }

    #[test]
    fn the_gain_scales_the_swing_angle() {
        let mut r = plain();
        solve(&mut r, 0.5, 0);
        let swings = calls(&r.seen, SET_AXIS_ANGLE);
        let expected = std::f32::consts::FRAC_PI_2 * 0.5;
        assert!((f32::from_bits(swings[0][2]) - expected).abs() < 1e-5);
    }

    #[test]
    fn a_direction_beyond_the_cone_is_limited_and_clears_the_byte() {
        let mut r = plain();
        r.e.mem.set_f32(r.setup + 0x30, 0.1);
        put(&mut r.e, r.target, [5.0, 0.0, 0.0, 0.0]);
        solve(&mut r, 1.0, 0);
        assert_eq!(r.e.mem.u8(r.result), 0);
        let turns = calls(&r.seen, SET_AXIS_ANGLE);
        assert_eq!(turns.len(), 2);
        // The first turn takes the reference direction (0, 1, 0) towards (1, 0, 0) by 0.1.
        close(axis_of(&turns[0]), [0.0, 0.0, -1.0, 0.0], 1e-3);
        assert_eq!(f32::from_bits(turns[0][2]), 0.1);
        // The limited direction is (sin 0.1, cos 0.1, 0): the swing from (1, 0, 0) to it is
        // the angle whose cosine is sin 0.1.
        close(axis_of(&turns[1]), [0.0, 0.0, 1.0, 0.0], 1e-3);
        let expected = 0.1f32.sin().acos();
        assert!((f32::from_bits(turns[1][2]) - expected).abs() < 1e-4);
    }

    #[test]
    fn a_cosine_of_one_or_more_gives_no_swing_and_a_negative_one_a_half_turn() {
        let mut r = plain();
        put(&mut r.e, r.target, [5.0, 0.0, 0.0, 0.0]);
        put(&mut r.e, r.setup + 0x20, [1.0, 0.0, 0.0, 0.0]);
        solve(&mut r, 1.0, 0);
        let swings = calls(&r.seen, SET_AXIS_ANGLE);
        assert_eq!(f32::from_bits(swings[0][2]), 0.0);
        assert!(calls(&r.seen, CRT_ARC_COSINE).is_empty());
        let mut r = plain();
        put(&mut r.e, r.target, [-5.0, 0.0, 0.0, 0.0]);
        put(&mut r.e, r.setup + 0x20, [-1.0, 0.0, 0.0, 0.0]);
        solve(&mut r, 1.0, 0);
        let swings = calls(&r.seen, SET_AXIS_ANGLE);
        assert_eq!(f32::from_bits(swings[0][2]), std::f32::consts::PI);
    }

    #[test]
    fn a_nan_cosine_takes_the_arc_cosine_not_pi() {
        // 00ce0818 `FCOMP` of |cosine| with 1.0 is unordered for NaN, which sets C0, so
        // `TEST AH,1` / `JNZ` goes to the CRT arc cosine (00ce0846) rather than to 0 or pi.
        let mut r = plain();
        // A NaN forward axis makes the rotated axis, and so the swing's cosine, NaN.
        put(&mut r.e, r.setup, [f32::NAN, 0.0, 0.0, 0.0]);
        solve(&mut r, 1.0, 0);
        let arc_cosines = calls(&r.seen, CRT_ARC_COSINE);
        assert_eq!(arc_cosines.len(), 1);
        assert!(wide(&arc_cosines[0]).is_nan());
        let swings = calls(&r.seen, SET_AXIS_ANGLE);
        assert_eq!(swings.len(), 1);
        assert!(f32::from_bits(swings[0][2]).is_nan());
    }

    #[test]
    fn the_limits_clamp_the_direction_in_the_frame_of_the_reference_direction() {
        let mut r = plain();
        put(&mut r.e, r.setup + 0x20, [1.0, 0.0, 0.0, 0.0]);
        let sixty = std::f32::consts::FRAC_PI_3;
        put(
            &mut r.e,
            r.target,
            [5.0 * sixty.cos(), 5.0 * sixty.sin(), 0.0, 0.0],
        );
        // Elevation limits 0.8 and -0.8, azimuth limits 0.5 and -0.5, around the z axis.
        put(&mut r.e, r.limits, [0.8, -0.8, 0.5, -0.5]);
        put(&mut r.e, r.limits + 0x10, [0.0, 0.0, 1.0, 0.0]);
        let limits = r.limits;
        solve(&mut r, 1.0, limits);
        // The largest limit, 0.8, is the angle of the cone.
        assert_eq!(wide(&calls(&r.seen, CRT_FIRST_TRIG)[0]), f64::from(0.8f32));
        assert_eq!(r.e.mem.u8(r.result), 0);
        let turns = calls(&r.seen, SET_AXIS_ANGLE);
        assert_eq!(turns.len(), 2);
        assert_eq!(f32::from_bits(turns[0][2]), 0.8);
        // The azimuth 0.8 of the limited direction goes down to the limit 0.5: the swing
        // from the forward axis (1, 0, 0) is by 0.5 about z.
        close(axis_of(&turns[1]), [0.0, 0.0, 1.0, 0.0], 1e-3);
        assert!((f32::from_bits(turns[1][2]) - 0.5).abs() < 1e-3);
        // The reference matrix: the reference direction, the vector of the limits crossed
        // with it, and that vector.
        assert_eq!(calls(&r.seen, MATRIX_TRANSPOSE).len(), 1);
        assert_eq!(calls(&r.seen, VECTOR_SET_MUL3).len(), 2);
    }

    #[test]
    fn limits_inside_the_cone_keep_the_byte() {
        let mut r = plain();
        put(&mut r.e, r.setup + 0x20, [1.0, 0.0, 0.0, 0.0]);
        // The target is along the reference direction: nothing to limit.
        put(&mut r.e, r.target, [5.0, 0.0, 0.0, 0.0]);
        put(&mut r.e, r.limits, [0.8, -0.8, 0.5, -0.5]);
        put(&mut r.e, r.limits + 0x10, [0.0, 0.0, 1.0, 0.0]);
        let limits = r.limits;
        solve(&mut r, 1.0, limits);
        assert_eq!(r.e.mem.u8(r.result), 1);
        assert_eq!(calls(&r.seen, SET_AXIS_ANGLE).len(), 1);
    }

    #[test]
    fn a_roll_axis_adds_the_roll_about_the_swing() {
        let mut r = plain();
        put(&mut r.e, r.setup + 0x20, [1.0, 0.0, 0.0, 0.0]);
        put(&mut r.e, r.setup + 0x10, [0.0, 0.0, 1.0, 0.0]);
        put(&mut r.e, r.target, [3.0, 0.0, 0.0, 0.0]);
        solve(&mut r, 1.0, 0);
        let turns = calls(&r.seen, SET_AXIS_ANGLE);
        assert_eq!(turns.len(), 2);
        // The swing is nothing (the direction is the forward axis). Near nothing: the
        // `RSQRTSS` estimate differs between CPU makers, so the cosine can land an ulp below 1.
        assert!(f32::from_bits(turns[0][2]).abs() < 1e-3);
        // The roll: distance 3 (squared 9), ratio 1 (squared 1), root sqrt(8) (squared 8), the
        // polynomial with a zero first factor gives 81 + 64 + 1 = 146 and
        // `acos(ratio / (sqrt(146) / root))` is the angle.
        let roots = calls(&r.seen, CRT_SQUARE_ROOT);
        assert_eq!(roots.len(), 2);
        let expected = (1.0f64 / (146.0f64.sqrt() / 8.0f64.sqrt())).acos();
        let angle = f64::from(f32::from_bits(turns[1][2]));
        assert!(
            (angle + expected).abs() < 1e-3,
            "{angle} against {expected}"
        );
        close(axis_of(&turns[1]), [0.0, -1.0, 0.0, 0.0], 1e-3);
        let rolls = calls(&r.seen, QUATERNION_ROLL);
        assert_eq!(rolls.len(), 1);
        // The helper gets the swing quaternion and the roll quaternion.
        assert_ne!(rolls[0][0], rolls[0][1]);
        assert_eq!(calls(&r.seen, HK_ARC_COSINE).len(), 1);
        assert_eq!(calls(&r.seen, QUATERNION_SET_MUL).len(), 1);
    }

    #[test]
    fn a_roll_with_a_zero_ratio_has_no_angle() {
        let mut r = plain();
        put(&mut r.e, r.setup + 0x20, [1.0, 0.0, 0.0, 0.0]);
        put(&mut r.e, r.setup + 0x10, [0.0, 0.0, 1.0, 0.0]);
        // The second factor 0 makes the ratio 0.
        r.e.set_global(ROLL_SECOND_FACTOR, 0.0f64);
        put(&mut r.e, r.target, [3.0, 0.0, 0.0, 0.0]);
        solve(&mut r, 1.0, 0);
        let turns = calls(&r.seen, SET_AXIS_ANGLE);
        assert_eq!(turns.len(), 2);
        assert!(calls(&r.seen, HK_ARC_COSINE).is_empty());
        assert_eq!(f32::from_bits(turns[1][2]), -0.0);
    }
}

/// The solver on a call the real game made, with the exe's own constants (`NV_EXE`).
#[cfg(test)]
mod recorded {
    /// Little-endian words from a recording's hex dump.
    fn words(hex: &str) -> Vec<u32> {
        (0..hex.len() / 8)
            .map(|i| {
                u32::from_be_bytes(
                    u32::from_str_radix(&hex[i * 8..i * 8 + 8], 16)
                        .unwrap()
                        .to_le_bytes(),
                )
            })
            .collect()
    }

    /// Doc's first head look in the opening (real game, nv-trace `opening-1`, the first call of
    /// 00ce0290 from 00c78d0e): setup, target, gain 1, the head bone's model transform, no limits.
    /// The real game answered 1 and turned the bone's rotation a little; the port is expected to
    /// give the same rotation.
    #[test]
    fn doc_s_first_look_turns_the_head_as_in_the_real_game() {
        let Ok(path) = std::env::var("NV_EXE") else {
            return;
        };
        let mut e = engine_core::Engine::with_exe(std::path::Path::new(&path)).unwrap();
        let setup = e.mem.alloc(0x40);
        let target = e.mem.alloc(0x10);
        let bone = e.mem.alloc(0x20);
        let result = e.mem.alloc(4);
        for (at, hex) in [
            (setup, "1f8b3a3bbcff7f3f031cc335e0ff7fbf8c97a43fbb745b3f00000000000000006d8a413d0b5e733f3aff9cbef3c562bf920a863fe07ccd1a80fe9812d0fd9812"),
            (target, "44a20e3f25a0cb40804434410d961fc0"),
            (bone, "4ff51c3eb02f0a3f3ae23641304d5e3e90182bbd547736bf94465b3daeb7323f"),
        ] {
            for (i, w) in words(hex).into_iter().enumerate() {
                e.mem.set_u32(at + 4 * i as u32, w);
            }
        }
        e.call(
            0x00ce_0290,
            &engine_core::args![result, setup, target, 1.0f32, bone, 0u32],
        );
        assert_eq!(e.mem.u8(result), 1);
        let got: Vec<f32> = (0..4).map(|i| e.mem.f32(bone + 0x10 + 4 * i)).collect();
        let want: Vec<f32> = words("846feabd600b35bfd539833df2d9313f")
            .into_iter()
            .map(f32::from_bits)
            .collect();
        for lane in 0..4 {
            assert!(
                (got[lane] - want[lane]).abs() < 1e-5,
                "{got:?} against {want:?}"
            );
        }
    }
}

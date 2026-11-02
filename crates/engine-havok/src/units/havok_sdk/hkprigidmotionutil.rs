//! `Havok SDK/hkprigidmotionutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both functions are cdecl with a motion state list: `motions` points to an array of pointers
//! and each pointer plus `offset` is a motion state. The float arithmetic of the x87 is done in
//! `f64` and rounded to `f32` at every store (the guide's convention), the vector arithmetic is
//! SSE. Neither function has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    load_bits, load_vec, movhlps, movlhps, rcp_estimate, shufps, store_bits, store_vec, sum3,
    unpckhps, unpcklps, zip_with, Vec4,
};

/// The CRT square root (`00ec6040`, argument and result in `ST0`).
const X87_SQRT: u32 = 0x00ec_6040;
/// `hkQuaternion::setMul` (`00c66320`, thiscall on the result: two quaternions; `RET 8`),
/// the quaternion normalisation (`005611c0`, thiscall) and the rotation set from a quaternion
/// (`00cb2d90`, thiscall on the matrix: the quaternion; `RET 4`).
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
const QUATERNION_NORMALIZE: u32 = 0x0056_11c0;
const SET_ROTATION: u32 = 0x00cb_2d90;
/// `hkMatrix3::setTranspose` (`00cd4ba0`, thiscall on the result: the source; `RET 4`).
const MATRIX3_SET_TRANSPOSE: u32 = 0x00cd_4ba0;

/// The absolute-value mask and the limit vector of the validity test (`1e6` per lane), and the
/// vector the velocities are reset to (`01268370`).
const ABS_MASK: u32 = 0x0101_7d10;
const VELOCITY_LIMIT: u32 = 0x010c_bf80;
const VELOCITY_RESET: u32 = 0x0126_8370;
/// The table of floats indexed by a quality byte (`010c7948`).
const QUALITY_TABLE: u32 = 0x010c_7948;
/// Constants: `0.5` (double), the factor `4 / pi^2` (double `010ce4b0`), the threshold `0.9`
/// (double `0106b9e8`) and the float it is replaced by (`010177e8`), the series coefficients
/// (doubles `010ce4a8`, `010ce4a0`, `010ce498`) and pi (double `0101ff40`).
const HALF: u32 = 0x0101_1588;
const ANGLE_FACTOR: u32 = 0x010c_e4b0;
const ANGLE_LIMIT: u32 = 0x0106_b9e8;
const ANGLE_LIMIT_FLOAT: u32 = 0x0101_77e8;
const SERIES_FIRST: u32 = 0x010c_e4a8;
const SERIES_SECOND: u32 = 0x010c_e4a0;
const SERIES_THIRD: u32 = 0x010c_e498;
const PI: u32 = 0x0101_ff40;
/// The vector of ones (`010ec7f0`).
const ONES: u32 = 0x010e_c7f0;

/// Splat of one lane, the `SHUFPS x, x, 0` selection.
fn splat(value: f32) -> Vec4 {
    [value; 4]
}

fn mul(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x * y)
}

fn add(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x + y)
}

/// The `MOVMSKPS` of the lanes of `bits` (masked with `mask`) that are below `limit`.
fn mask_below(bits: [u32; 4], mask: [u32; 4], limit: Vec4) -> u32 {
    (0..4).fold(0, |all, lane| {
        if f32::from_bits(bits[lane] & mask[lane]) < limit[lane] {
            all | 1 << lane
        } else {
            all
        }
    })
}

/// The `MOVMSKPS` of the lanes of `bits` that are not NaN.
fn mask_ordered(bits: [u32; 4]) -> u32 {
    (0..4).fold(0, |all, lane| {
        let value = f32::from_bits(bits[lane]);
        if value == value {
            all | 1 << lane
        } else {
            all
        }
    })
}

/// The CRT square root of an `f32` through the x87 (`ST0` in and out).
fn x87_root(e: &mut Engine, value: f32) -> f32 {
    x87_quiet(e.call(X87_SQRT, &args![f64::from(value)]).f64() as f32)
}

/// One motion of `hkRigidMotionUtilStep`; `frame` is the stack frame (0x60 bytes: the
/// quaternion at `+0x30`, the rotation delta at `+0x40`, a copy of the velocity at `+0x50`).
fn step_motion(e: &mut Engine, info: u32, state: u32, frame: u32) {
    let angular = load_bits(e, state + 0xe0);
    let linear = load_bits(e, state + 0xd0);
    let abs_mask = load_bits(e, ABS_MASK);
    let limit = load_vec(e, VELOCITY_LIMIT);
    let valid = mask_below(angular, abs_mask, limit)
        & mask_below(linear, abs_mask, limit)
        & mask_ordered(angular)
        & mask_ordered(linear)
        & 7;
    if valid != 7 {
        let reset = load_bits(e, VELOCITY_RESET);
        store_bits(e, state + 0xd0, reset);
        let reset = load_bits(e, VELOCITY_RESET);
        store_bits(e, state + 0xe0, reset);
    }
    // The previous position is the position, with the first float of the step info in the
    // fourth lane.
    let position = load_bits(e, state + 0x60);
    store_bits(e, state + 0x50, position);
    let first = x87_quiet_bits(e.mem.u32(info));
    e.mem.set_u32(state + 0x5c, first);
    // A velocity above the limit of its quality is scaled down to it.
    let velocity = load_vec(e, state + 0xd0);
    let quality = u32::from(e.mem.u8(state + 0xbc));
    store_vec(e, frame + 0x50, velocity);
    let speed_squared = sum3(mul(velocity, velocity));
    let top_speed = e.mem.f32(QUALITY_TABLE + quality * 4);
    let limit_squared = f64::from(top_speed) * f64::from(top_speed);
    if f64::from(speed_squared) > limit_squared {
        let speed = x87_root(e, speed_squared);
        let inverse = (1.0 / f64::from(speed)) as f32;
        let factor = (f64::from(inverse) * f64::from(top_speed)) as f32;
        let scaled = mul(splat(factor), load_vec(e, frame + 0x50));
        store_vec(e, state + 0xd0, scaled);
    }
    // The position moves by the velocity times the time step.
    let step = x87_quiet(e.mem.f32(info + 8));
    let velocity = load_vec(e, state + 0xd0);
    let moved = add(mul(splat(step), velocity), load_vec(e, state + 0x60));
    store_vec(e, state + 0x60, moved);
    let second = x87_quiet_bits(e.mem.u32(info + 0xc));
    e.mem.set_u32(state + 0x6c, second);
    // The rotation: the current quaternion is kept (twice).
    let rotation = load_bits(e, state + 0x80);
    store_bits(e, frame + 0x30, rotation);
    let angular_quality = u32::from(e.mem.u8(state + 0xbd));
    let kept = load_bits(e, frame + 0x30);
    store_bits(e, state + 0x70, kept);
    // The half-angle vector: half the time step times the angular velocity.
    let raw_step = e.mem.f32(info + 8);
    let half = e.mem.f64(HALF);
    let angular_velocity = load_vec(e, state + 0xe0);
    let half_step = (f64::from(raw_step) * half) as f32;
    store_vec(e, frame + 0x50, angular_velocity);
    let half_angle = mul(splat(half_step), angular_velocity);
    store_vec(e, frame + 0x40, half_angle);
    let angle_squared = sum3(mul(half_angle, half_angle));
    let angle_estimate = (f64::from(angle_squared) * e.mem.f64(ANGLE_FACTOR)) as f32;
    // The largest angle step the quality allows, capped at the float constant when above
    // `0.9`.
    let allowed = e.mem.f32(QUALITY_TABLE + angular_quality * 4);
    let mut allowed = (f64::from(allowed) * f64::from(raw_step)) as f32;
    if f64::from(allowed) > e.mem.f64(ANGLE_LIMIT) {
        allowed = x87_quiet(e.global::<f32>(ANGLE_LIMIT_FLOAT));
    }
    let allowed_squared = (f64::from(allowed) * f64::from(allowed)) as f32;
    let first_coefficient = e.mem.f64(SERIES_FIRST);
    let second_coefficient = e.mem.f64(SERIES_SECOND);
    let third_coefficient = e.mem.f64(SERIES_THIRD);
    let kept_angle;
    let cosine;
    if allowed_squared >= angle_estimate {
        // Small step: the series of the cosine of the estimate.
        let estimate = f64::from(angle_estimate);
        let squared = (estimate * estimate) as f32;
        let first = 1.0 - first_coefficient * estimate;
        let second = first - second_coefficient * f64::from(squared);
        let third = f64::from(squared) * (estimate * third_coefficient);
        cosine = (second - third) as f32;
        kept_angle = angle_estimate;
    } else {
        // Large step: the angular velocity and the half angle are scaled down so that the
        // angle is the allowed one.
        let root = x87_root(e, angle_estimate);
        let inverse = (1.0 / f64::from(root)) as f32;
        let factor = (f64::from(inverse) * f64::from(allowed)) as f32;
        let scaled_velocity = mul(splat(factor), load_vec(e, frame + 0x50));
        store_vec(e, state + 0xe0, scaled_velocity);
        let fourth = (f64::from(allowed_squared) * f64::from(allowed_squared)) as f32;
        let scaled_angle = mul(splat(factor), load_vec(e, frame + 0x40));
        store_vec(e, frame + 0x40, scaled_angle);
        let squared = f64::from(allowed_squared);
        let first = 1.0 - first_coefficient * squared;
        let second = first - second_coefficient * f64::from(fourth);
        let third = f64::from(fourth) * (squared * third_coefficient);
        cosine = (second - third) as f32;
        kept_angle = allowed_squared;
    }
    e.mem.set_f32(frame + 0x4c, cosine);
    e.call(
        QUATERNION_SET_MUL,
        &args![frame + 0x30, frame + 0x40, frame + 0x30],
    );
    e.call(QUATERNION_NORMALIZE, &args![frame + 0x30]);
    // The rotation delta doubled, with the angle in the fourth lane.
    let delta = load_vec(e, frame + 0x40);
    store_vec(e, state + 0xa0, add(delta, delta));
    let angle = x87_root(e, kept_angle);
    let rotated = load_bits(e, frame + 0x30);
    let angle_pi = (f64::from(angle) * e.mem.f64(PI)) as f32;
    e.mem.set_f32(state + 0xac, angle_pi);
    store_bits(e, state + 0x80, rotated);
    e.call(SET_ROTATION, &args![state + 0x10, state + 0x80]);
    // The transform's translation: the position minus the centre of mass (`+0x90`) rotated
    // by the transform's three columns, summed as `(y * c1 + x * c0) + z * c2`.
    let centre = load_vec(e, state + 0x90);
    let column_1 = load_vec(e, state + 0x20);
    let column_0 = load_vec(e, state + 0x10);
    let first = mul(splat(centre[1]), column_1);
    let second = mul(splat(centre[0]), column_0);
    let sum = add(first, second);
    let column_2 = load_vec(e, state + 0x30);
    let third = mul(splat(centre[2]), column_2);
    let position = load_vec(e, state + 0x60);
    let rotated_centre = add(sum, third);
    let translation = zip_with(position, rotated_centre, |a, b| a - b);
    store_vec(e, state + 0x40, translation);
}

// Translated from 00d28680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRigidMotionUtilStep` (engine map), cdecl with the step info (the first float at `+0`, the
/// time step at `+8`, another float at `+0xC`), the motion list, the number of motions and the
/// offset of the motion state in each listed object. Motions of type 5 (`+8`) are skipped.
/// For each other motion:
///
/// * a velocity pair (`+0xD0` linear, `+0xE0` angular) with a lane whose absolute value is not
///   below 1e6 (a NaN included) in its first three lanes is reset to the vector at `01268370`;
/// * the position (`+0x60`) is copied to `+0x50` with the step info's first float in its fourth
///   lane; a linear velocity whose squared length is above the square of the limit
///   `table[quality at +0xBC]` is scaled to the limit (the CRT root); the position moves by the
///   velocity times the time step and gets the info's `+0xC` float in the fourth lane;
/// * the rotation (`+0x80`) is kept at `+0x70`; the half angle vector (`half the step times
///   the angular velocity`) gets its squared length scaled by `4 / pi^2` as the estimate; the
///   largest step is `table[quality at +0xBD] * step` (replaced by the float `010177e8` when
///   above `0.9`); a larger estimate scales the angular velocity and the half angle to the
///   largest step; the cosine series gives the fourth lane; the rotation is multiplied by the
///   delta (`hkQuaternion::setMul`), normalised, stored at `+0x80` and made the transform's
///   rotation (`00cb2d90`); `+0xA0` takes twice the delta and `+0xAC` the angle times pi;
/// * the transform's translation (`+0x40`) is the position minus the centre of mass (`+0x90`)
///   rotated by the transform's columns.
///
/// The code keeps three vectors in its stack frame; the frame here has the same offsets.
pub fn hk_rigid_motion_util_step(e: &mut Engine, info: Ptr, motions: Ptr, count: i32, offset: u32) {
    let mut remaining = count.wrapping_sub(1);
    if remaining < 0 {
        return;
    }
    let mut list = motions.addr();
    e.with_stack(0x60, |e, frame| loop {
        let state = e.mem.u32(list).wrapping_add(offset);
        if e.mem.u8(state + 8) != 5 {
            step_motion(e, info.addr(), state, frame.addr());
        }
        list = list.wrapping_add(4);
        remaining = remaining.wrapping_sub(1);
        if remaining < 0 {
            break;
        }
    });
}

/// The damping factor `1 - damping * step` (rounded to a float), zero when it is below zero
/// (a NaN stays).
fn damping_factor(damping: f32, step: f32) -> f32 {
    let factor = (1.0 - f64::from(damping) * f64::from(step)) as f32;
    if factor < 0.0 {
        0.0
    } else {
        factor
    }
}

/// The damping of a motion state's velocities (linear `+0xD0` by the factor of `+0xB4`,
/// angular `+0xE0` by the factor of `+0xB8`); answers nothing. The time step is the info's
/// (`+8`) passed through the x87.
fn damp_velocities(e: &mut Engine, info: u32, state: u32) {
    let step = x87_quiet(e.mem.f32(info + 8));
    let linear = damping_factor(e.mem.f32(state + 0xb4), step);
    let velocity = mul(splat(linear), load_vec(e, state + 0xd0));
    store_vec(e, state + 0xd0, velocity);
    let angular = damping_factor(e.mem.f32(state + 0xb8), step);
    let velocity = mul(splat(angular), load_vec(e, state + 0xe0));
    store_vec(e, state + 0xe0, velocity);
}

/// The position blended between the previous (`+0x50`) and the new one (`+0x60`) by
/// `(info float 0 - previous w) * new w`, written to `target`.
fn blend_position(e: &mut Engine, info: u32, state: u32, target: u32) {
    let blend = ((f64::from(e.mem.f32(info)) - f64::from(e.mem.f32(state + 0x5c)))
        * f64::from(e.mem.f32(state + 0x6c))) as f32;
    let ones = load_vec(e, ONES);
    let previous = load_vec(e, state + 0x50);
    let current = load_vec(e, state + 0x60);
    let kept = mul(zip_with(ones, splat(blend), |a, b| a - b), previous);
    let taken = mul(current, splat(blend));
    store_vec(e, target, add(kept, taken));
}

/// The identity inertia: zero vectors at `+0x50`, `+0x60`, `+0x70` of the accumulator with
/// `1.0` on the diagonal (`+0x50`, `+0x64`, `+0x78`).
fn identity_inertia(e: &mut Engine, accumulator: u32) {
    for offset in [0x50, 0x60, 0x70] {
        store_bits(e, accumulator + offset, [0; 4]);
    }
    for offset in [0x50, 0x64, 0x78] {
        e.mem.set_f32(accumulator + offset, 1.0);
    }
}

/// The word at `+8` of the accumulator: the short at `+0x11E` of the state shifted up 16 bits.
fn store_group(e: &mut Engine, state: u32, accumulator: u32) {
    let group = (e.mem.i16(state + 0x11e) as i32) << 16;
    e.mem.set_i32(accumulator + 8, group);
}

// Translated from 00d29830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRigidMotionUtilApplyForcesAndBuildAccumulators` (engine map), cdecl with the step info,
/// the motion list, the number of motions, the offset of the motion state in each listed
/// object and the accumulator array (0x80 bytes per motion); answers the accumulator after the
/// last one. By the motion type (`+8`):
///
/// * 0, 3, 6: the velocities are damped (see `damping_factor`), the state's matrix (`+0x10`)
///   is transposed into the accumulator at `+0x50` (`hkMatrix3::setTranspose`), the accumulator
///   gets the type 0, the vectors `+0x30` (from `+0xC0`), `+0x10` (from `+0xD0`), the blended
///   position at `+0x40`, the angular velocity transformed by the transposed columns at `+0x20`,
///   the byte at `+0xBE` as a word at `+4` and the group word at `+8`;
/// * 2 and 7: the velocities are damped, the accumulator gets the type 0 (2) or 2 (7), `+0x30`
///   from `+0xC0`, the blended position at `+0x40`, the angular and linear velocities at
///   `+0x20` and `+0x10`, the identity inertia, the byte at `+0xBE` and the group word;
/// * 4: type 1, zero at `+0x30`, the position `+0x60` at `+0x40`, the velocities, the identity
///   inertia and the group word;
/// * 5: type 1 and zeros (the word at `+8` is 0.0);
/// * any other (1, above 7): nothing is written.
///
/// The code keeps the constants `1, 0, 0, 1` on the x87 stack throughout and only stores them.
pub fn hk_rigid_motion_util_apply_forces_and_build_accumulators(
    e: &mut Engine,
    info: Ptr,
    motions: Ptr,
    count: i32,
    offset: u32,
    accumulators: Ptr,
) -> Ptr {
    let mut remaining = count.wrapping_sub(1);
    if remaining < 0 {
        return accumulators;
    }
    let info = info.addr();
    let mut list = motions.addr();
    let mut accumulator = accumulators.addr();
    loop {
        let state = e.mem.u32(list).wrapping_add(offset);
        match e.mem.u8(state + 8) {
            0 | 3 | 6 => {
                damp_velocities(e, info, state);
                e.call(
                    MATRIX3_SET_TRANSPOSE,
                    &args![accumulator + 0x50, state + 0x10],
                );
                e.mem.set_u8(accumulator, 0);
                let word = load_bits(e, state + 0xc0);
                store_bits(e, accumulator + 0x30, word);
                let word = load_bits(e, state + 0xd0);
                store_bits(e, accumulator + 0x10, word);
                blend_position(e, info, state, accumulator + 0x40);
                let angular = load_vec(e, state + 0xe0);
                let column_1 = load_vec(e, accumulator + 0x60);
                let column_0 = load_vec(e, accumulator + 0x50);
                let sum = add(
                    mul(splat(angular[1]), column_1),
                    mul(splat(angular[0]), column_0),
                );
                let column_2 = load_vec(e, accumulator + 0x70);
                let rotated = add(sum, mul(splat(angular[2]), column_2));
                store_vec(e, accumulator + 0x20, rotated);
                e.mem
                    .set_u32(accumulator + 4, u32::from(e.mem.u8(state + 0xbe)));
                store_group(e, state, accumulator);
            }
            2 | 7 => {
                let kind = if e.mem.u8(state + 8) == 7 { 2 } else { 0 };
                e.mem.set_u8(accumulator, kind);
                damp_velocities(e, info, state);
                let word = load_bits(e, state + 0xc0);
                store_bits(e, accumulator + 0x30, word);
                blend_position(e, info, state, accumulator + 0x40);
                let word = load_bits(e, state + 0xe0);
                store_bits(e, accumulator + 0x20, word);
                let word = load_bits(e, state + 0xd0);
                store_bits(e, accumulator + 0x10, word);
                identity_inertia(e, accumulator);
                e.mem
                    .set_u32(accumulator + 4, u32::from(e.mem.u8(state + 0xbe)));
                store_group(e, state, accumulator);
            }
            4 => {
                e.mem.set_u8(accumulator, 1);
                store_bits(e, accumulator + 0x30, [0; 4]);
                let word = load_bits(e, state + 0x60);
                store_bits(e, accumulator + 0x40, word);
                let word = load_bits(e, state + 0xe0);
                store_bits(e, accumulator + 0x20, word);
                let word = load_bits(e, state + 0xd0);
                store_bits(e, accumulator + 0x10, word);
                identity_inertia(e, accumulator);
                store_group(e, state, accumulator);
            }
            5 => {
                e.mem.set_u32(accumulator + 8, 0);
                e.mem.set_u8(accumulator, 1);
                for offset in [0x30, 0x10, 0x20, 0x40, 0x50, 0x60, 0x70] {
                    store_bits(e, accumulator + offset, [0; 4]);
                }
            }
            _ => {}
        }
        list = list.wrapping_add(4);
        accumulator = accumulator.wrapping_add(0x80);
        remaining = remaining.wrapping_sub(1);
        if remaining < 0 {
            break;
        }
    }
    Ptr::new(accumulator)
}

/// `hkVector4::setRotatedDir` (`00c7f700`, thiscall on the result: the rotation, the vector;
/// `RET 8`).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// `004b4ab0` (no map name; thiscall on the result: three 16-byte rows, the vector; `RET 8`):
/// per its translation, the vector times the rows (lanes 0 to 2 are the rows' dot products).
const ROWS_TIMES_VECTOR: u32 = 0x004b_4ab0;
/// `hkVector4Util::packQuaternionIntoInt32` (`00d975d0`, cdecl: the quaternion; answers the
/// packed word).
const PACK_QUATERNION: u32 = 0x00d9_75d0;
/// The scale of a packed orientation byte (double `010ce488`), the mask that keeps the first
/// three lanes (`010cbfc0`) and the double `1.0` (`01012070`).
const PACKED_SCALE: u32 = 0x010c_e488;
const XYZ_MASK: u32 = 0x010c_bfc0;
const ONE_DOUBLE: u32 = 0x0101_2070;

/// One lane of `MULPS`/`ADDPS`/`SUBPS`/`ADDSS` (`dest op src`): a NaN operand comes back
/// quieted, the destination's first; a NaN the operation itself makes is the default NaN.
fn sse_lane(dest: f32, src: f32, op: fn(f32, f32) -> f32) -> f32 {
    if dest.is_nan() {
        f32::from_bits(dest.to_bits() | 0x0040_0000)
    } else if src.is_nan() {
        f32::from_bits(src.to_bits() | 0x0040_0000)
    } else {
        let result = op(dest, src);
        if result.is_nan() {
            f32::from_bits(0xffc0_0000)
        } else {
            result
        }
    }
}

fn ps_mul(dest: Vec4, src: Vec4) -> Vec4 {
    std::array::from_fn(|lane| sse_lane(dest[lane], src[lane], |a, b| a * b))
}

fn ps_add(dest: Vec4, src: Vec4) -> Vec4 {
    std::array::from_fn(|lane| sse_lane(dest[lane], src[lane], |a, b| a + b))
}

fn ps_sub(dest: Vec4, src: Vec4) -> Vec4 {
    std::array::from_fn(|lane| sse_lane(dest[lane], src[lane], |a, b| a - b))
}

fn ss_add(dest: f32, src: f32) -> f32 {
    sse_lane(dest, src, |a, b| a + b)
}

/// The `SHUFPS`/`ADDSS` sum of the first three lanes, `z + (y + x)`.
fn lane_sum3(v: Vec4) -> f32 {
    ss_add(v[2], ss_add(v[1], v[0]))
}

/// `RCPPS` refined once: `(r + r) - ((r * x) * r)`.
fn refined_rcp(x: Vec4) -> Vec4 {
    let estimate = x.map(rcp_estimate);
    let twice = ps_add(estimate, estimate);
    let error = ps_mul(ps_mul(estimate, x), estimate);
    ps_sub(twice, error)
}

/// The vector rotated by the three columns at `columns`: `(y * c1 + x * c0) + z * c2`.
fn rotate_by_columns(e: &Engine, v: Vec4, columns: u32) -> Vec4 {
    let first = ps_mul([v[1]; 4], load_vec(e, columns + 0x10));
    let second = ps_mul([v[0]; 4], load_vec(e, columns));
    let sum = ps_add(first, second);
    ps_add(sum, ps_mul([v[2]; 4], load_vec(e, columns + 0x20)))
}

/// The factor `limit / root(value)` as the code makes it: the CRT root of the x87 value, one
/// over it and the product with the float `limit` (an `FMUL dword` operand), each stored as a
/// float.
fn limit_factor(e: &mut Engine, value: f64, limit: u32) -> f32 {
    let root = x87_store32(e.call(X87_SQRT, &args![value]).f64());
    let inverse = x87_store32(x87_div(1.0, x87_load32(root)));
    f32::from_bits(x87_store32(x87_mul(x87_load32(inverse), x87_m32(limit))))
}

/// `1 - product` stored as a float, replaced by `+0` when it is below zero (a NaN stays).
fn clamped_damping(product: f64) -> f32 {
    let factor = x87_store32(x87_sub(1.0, product));
    if x87_load32(factor) < 0.0 {
        0.0
    } else {
        f32::from_bits(factor)
    }
}

/// The damping of `hkpMotion::m_linearVelocity` (`+0xD0`) by `m_linearDamping` (`+0xB4`) and
/// of `m_angularVelocity` (`+0xE0`) by `m_angularDamping` (`+0xB8`) over the step's
/// `m_deltaTime` (`hkStepInfo` `+8`; Xbox PDB names).
fn damp_motion(e: &mut Engine, info: u32, state: u32) {
    let step = x87_load32(e.mem.u32(info + 8));
    let linear = clamped_damping(x87_mul(x87_load32(e.mem.u32(state + 0xb4)), step));
    let damped = ps_mul([linear; 4], load_vec(e, state + 0xd0));
    store_vec(e, state + 0xd0, damped);
    let angular = clamped_damping(x87_mul(step, x87_load32(e.mem.u32(state + 0xb8))));
    let damped = ps_mul([angular; 4], load_vec(e, state + 0xe0));
    store_vec(e, state + 0xe0, damped);
}

/// The gravity: the linear velocity gains `m_gravityFactor` (`+0x11E`, an `hkHalf`: the top 16
/// bits of a float) times the gravity vector.
fn add_gravity(e: &mut Engine, state: u32, gravity: u32) {
    let factor = f32::from_bits(u32::from(e.mem.u16(state + 0x11e)) << 16);
    let pull = ps_mul([factor; 4], load_vec(e, gravity));
    let velocity = ps_add(pull, load_vec(e, state + 0xd0));
    store_vec(e, state + 0xd0, velocity);
}

/// Whether the first three lanes of both velocities are below the limit in absolute value
/// (`CMPLTPS`, no NaN) and ordered (`CMPEQPS` with themselves).
fn velocities_valid(e: &Engine, linear: [u32; 4], angular: [u32; 4]) -> bool {
    let abs_mask = load_bits(e, ABS_MASK);
    let limit = load_vec(e, VELOCITY_LIMIT);
    let mask = mask_below(linear, abs_mask, limit)
        & mask_below(angular, abs_mask, limit)
        & mask_ordered(linear)
        & mask_ordered(angular);
    mask & 7 == 7
}

/// The previous centre of mass (`+0x50`) takes the current one (`+0x60`) with the step's
/// `m_startTime` (`hkStepInfo` `+0`) in its fourth lane.
fn keep_previous_centre(e: &mut Engine, info: u32, state: u32) {
    let centre = load_bits(e, state + 0x60);
    store_bits(e, state + 0x50, centre);
    let start = x87_quiet_bits(e.mem.u32(info));
    e.mem.set_u32(state + 0x5c, start);
}

/// The fourth lane of the rotation delta: `((1 - c1 * x) - c2 * x2) - x2 * (x * c3)` with the
/// series coefficients (doubles), `x2` the float `x * x`.
fn cosine_series(e: &Engine, x: f64, squared: u32) -> u32 {
    let first = x87_load64(e.mem.u64(SERIES_FIRST));
    let second = x87_load64(e.mem.u64(SERIES_SECOND));
    let third = e.mem.f64(SERIES_THIRD);
    let squared = x87_load32(squared);
    let value = x87_sub(1.0, x87_mul(first, x));
    let value = x87_sub(value, x87_mul(second, squared));
    let term = x87_mul(squared, x87_mul(x, third));
    x87_store32(x87_sub(value, term))
}

/// Where one integration keeps its vectors: the linear velocity it reads (and the copy the
/// limit scales, and where a scaled one is stored, if anywhere), the angular velocity (read
/// and scaled in place, from its copy), and the stack slots of the rotation and its delta.
struct Integration {
    linear: u32,
    linear_copy: u32,
    linear_target: Option<u32>,
    angular: u32,
    angular_copy: u32,
    rotation: u32,
    delta: u32,
}

/// The integration both functions share (after the validity test and the previous centre):
/// the linear speed limit `table[m_maxLinearVelocity]` (`+0xBC`), the new centre of mass
/// (`+0x60`, `m_invDeltaTime` of the step in its fourth lane), the previous rotation (`+0x70`),
/// the half angle `half the step times the angular velocity` with its squared length times
/// `4 / pi^2` as the estimate, the largest angle `table[m_maxAngularVelocity] * step` (`+0xBD`,
/// replaced by the float `010177e8` when above `0.9`), a larger estimate scaling the angular
/// velocity and the half angle to it, the cosine series in the delta's fourth lane, the
/// rotation times the delta (`hkQuaternion::setMul`), normalised, `m_deltaAngle` (`+0xA0`, the
/// doubled delta with `root * pi` in its fourth lane), the new rotation (`+0x80`), the
/// transform's rotation from it (`00cb2d90`) and its translation (`+0x40`, the centre minus the
/// local centre of mass `+0x90` rotated).
fn integrate(e: &mut Engine, info: u32, state: u32, at: &Integration) {
    // The linear speed limit.
    let linear = load_vec(e, at.linear);
    store_vec(e, at.linear_copy, linear);
    let top = x87_load32(
        e.mem
            .u32(QUALITY_TABLE + 4 * u32::from(e.mem.u8(state + 0xbc))),
    );
    let speed_squared = x87_load32(lane_sum3(ps_mul(linear, linear)).to_bits());
    let mut moving = linear;
    if speed_squared > x87_mul(top, top) {
        let limit = e
            .mem
            .u32(QUALITY_TABLE + 4 * u32::from(e.mem.u8(state + 0xbc)));
        let factor = limit_factor(e, speed_squared, limit);
        moving = ps_mul([factor; 4], load_vec(e, at.linear_copy));
        if let Some(target) = at.linear_target {
            store_vec(e, target, moving);
        }
    }
    // The centre of mass moves by the velocity times the time step.
    let step = f32::from_bits(x87_quiet_bits(e.mem.u32(info + 8)));
    let moved = ps_add(ps_mul([step; 4], moving), load_vec(e, state + 0x60));
    store_vec(e, state + 0x60, moved);
    let inverse_step = x87_quiet_bits(e.mem.u32(info + 0xc));
    e.mem.set_u32(state + 0x6c, inverse_step);
    // The rotation is kept as the previous one.
    let rotation = load_bits(e, state + 0x80);
    store_bits(e, at.rotation, rotation);
    let angular_quality = u32::from(e.mem.u8(state + 0xbd));
    let kept = load_bits(e, at.rotation);
    store_bits(e, state + 0x70, kept);
    // The half angle and its estimate.
    let half = x87_store32(x87_mul(x87_load32(e.mem.u32(info + 8)), e.mem.f64(HALF)));
    let angular = load_vec(e, at.angular);
    store_vec(e, at.angular_copy, angular);
    let delta = ps_mul([f32::from_bits(half); 4], angular);
    store_vec(e, at.delta, delta);
    let angle_squared = lane_sum3(ps_mul(delta, delta));
    let estimate = x87_store32(x87_mul(
        x87_load32(angle_squared.to_bits()),
        e.mem.f64(ANGLE_FACTOR),
    ));
    let allowed = x87_store32(x87_mul(
        x87_load32(e.mem.u32(QUALITY_TABLE + 4 * angular_quality)),
        x87_m32(e.mem.u32(info + 8)),
    ));
    let mut allowed_value = x87_load32(allowed);
    if allowed_value > e.mem.f64(ANGLE_LIMIT) {
        allowed_value = x87_load32(e.mem.u32(ANGLE_LIMIT_FLOAT));
    }
    let allowed = x87_store32(allowed_value);
    let allowed_squared = x87_store32(x87_mul(x87_load32(allowed), x87_load32(allowed)));
    let angle = if x87_load32(allowed_squared) >= x87_load32(estimate) {
        estimate
    } else {
        // Too fast a turn: the angular velocity and the half angle are scaled to the allowed
        // angle.
        let factor = limit_factor(e, x87_load32(estimate), allowed);
        let scaled = ps_mul([factor; 4], load_vec(e, at.angular_copy));
        store_vec(e, at.angular, scaled);
        let scaled = ps_mul([factor; 4], load_vec(e, at.delta));
        store_vec(e, at.delta, scaled);
        allowed_squared
    };
    let angle_value = x87_load32(angle);
    let squared = x87_store32(x87_mul(angle_value, angle_value));
    let cosine = cosine_series(e, angle_value, squared);
    e.mem.set_u32(at.delta + 0xc, cosine);
    e.call(
        QUATERNION_SET_MUL,
        &args![at.rotation, at.delta, at.rotation],
    );
    e.call(QUATERNION_NORMALIZE, &args![at.rotation]);
    let delta = load_vec(e, at.delta);
    store_vec(e, state + 0xa0, ps_add(delta, delta));
    let root = x87_store32(e.call(X87_SQRT, &args![angle_value]).f64());
    let turned = load_bits(e, at.rotation);
    let angle_pi = x87_store32(x87_mul(x87_load32(root), e.mem.f64(PI)));
    e.mem.set_u32(state + 0xac, angle_pi);
    store_bits(e, state + 0x80, turned);
    e.call(SET_ROTATION, &args![state + 0x10, state + 0x80]);
    // The transform's translation.
    let centre = load_vec(e, state + 0x90);
    let rotated = rotate_by_columns(e, centre, state + 0x10);
    let translation = ps_sub(load_vec(e, state + 0x60), rotated);
    store_vec(e, state + 0x40, translation);
}

/// The velocities brought back under their limits after the step: the linear one (its copy
/// kept at `copy`) when its squared length is above `table[+0xBC]` squared (as a float) or NaN,
/// the angular one likewise with `table[+0xBD]`; a NaN squared length then resets the velocity
/// to the vector at `01268370`.
fn limit_velocities(e: &mut Engine, state: u32, copy: u32) {
    let angular_quality = u32::from(e.mem.u8(state + 0xbd));
    let linear = load_vec(e, state + 0xd0);
    let quality = u32::from(e.mem.u8(state + 0xbc));
    let angular_top = x87_load32(e.mem.u32(QUALITY_TABLE + 4 * angular_quality));
    let angular_top = x87_store32(x87_mul(angular_top, angular_top));
    store_vec(e, copy, linear);
    let top = x87_load32(e.mem.u32(QUALITY_TABLE + 4 * quality));
    let speed_squared = lane_sum3(ps_mul(linear, linear)).to_bits();
    let angular = load_vec(e, state + 0xe0);
    let spin_squared = lane_sum3(ps_mul(angular, angular)).to_bits();
    let top = x87_store32(x87_mul(top, top));
    let speed = x87_load32(speed_squared);
    if x87_load32(top) < speed || speed.is_nan() {
        let limit = e
            .mem
            .u32(QUALITY_TABLE + 4 * u32::from(e.mem.u8(state + 0xbc)));
        let factor = limit_factor(e, speed, limit);
        let scaled = ps_mul([factor; 4], load_vec(e, copy));
        store_vec(e, state + 0xd0, scaled);
        if speed.is_nan() {
            let reset = load_bits(e, VELOCITY_RESET);
            store_bits(e, state + 0xd0, reset);
        }
    }
    let spin = x87_load32(spin_squared);
    if x87_load32(angular_top) < spin || spin.is_nan() {
        let limit = e
            .mem
            .u32(QUALITY_TABLE + 4 * u32::from(e.mem.u8(state + 0xbd)));
        let factor = limit_factor(e, spin, limit);
        let scaled = ps_mul([factor; 4], load_vec(e, state + 0xe0));
        store_vec(e, state + 0xe0, scaled);
        if spin.is_nan() {
            let reset = load_bits(e, VELOCITY_RESET);
            store_bits(e, state + 0xe0, reset);
        }
    }
}

/// The deactivation bookkeeping (Xbox PDB names of `hkpMotion` and `hkpSolverInfo`): the
/// `m_deactivationIntegrateCounter` (`+9`) counts up; on every fourth step one of the two
/// checks runs (index 1 when the counter reaches a multiple of 16, which restarts it at 0;
/// 256 sets it to 0xFF and checks nothing). A check keeps the larger of the motion's energy
/// (`min(m_objectRadius, one)^2 * |angular|^2 + |linear|^2`) and the fourth lane of
/// `m_deactivationRefPosition[index]`, then compares the distance from that reference to the
/// centre of mass with `m_maxDistSqrd[index]` and the packed reference orientation
/// (`m_deactivationRefOrientation[index]`, four signed bytes times the scale) with the
/// rotation against `m_maxRotSqrd[index]` (an `hkHalf`), both of the solver info's
/// `m_deactivationInfo[m_deactivationClass]`. Within both, `m_deactivationNumInactiveFrames
/// [index]` counts up (bits 0 to 6, held at 64 and above); otherwise it is cleared, the
/// reference takes the centre (fourth lane zero) and the orientation is packed again. Both
/// keep `m_deactivationNumInactiveFramesSelectFlag[index]` of the solver info in bits 14 and
/// up (the 16-bit shifts of the code).
fn deactivation_step(e: &mut Engine, solver: u32, state: u32, one: f64) {
    let counter = u32::from(e.mem.u8(state + 9)) + 1;
    e.mem.set_u8(state + 9, counter as u8);
    if counter & 3 != 0 {
        return;
    }
    let index = if counter & 0xf != 0 {
        0
    } else if counter == 0x100 {
        e.mem.set_u8(state + 9, 0xff);
        return;
    } else {
        e.mem.set_u8(state + 9, 0);
        1
    };
    let mut radius = x87_load32(e.mem.u32(state + 0xb0));
    if radius > one {
        radius = 1.0;
    }
    let radius = x87_load32(x87_store32(radius));
    let linear = load_vec(e, state + 0xd0);
    let speed_squared = lane_sum3(ps_mul(linear, linear));
    let angular = load_vec(e, state + 0xe0);
    let spin_squared = lane_sum3(ps_mul(angular, angular));
    let energy = x87_mul(x87_mul(radius, radius), x87_m32(spin_squared.to_bits()));
    let energy = x87_store32(x87_add(energy, x87_m32(speed_squared.to_bits())));
    let slot = state + 0xfc + 16 * index;
    let previous = x87_load32(e.mem.u32(slot));
    let energy = x87_load32(energy);
    let larger = if energy < previous { previous } else { energy };
    e.mem.set_u32(slot, x87_store32(larger));
    // The distance from the reference position.
    let class = u32::from(e.mem.u8(state + 0xbe));
    let reference = state + 0xf0 + 16 * index;
    let offset = ps_sub(load_vec(e, reference), load_vec(e, state + 0x60));
    let distance = x87_load32(lane_sum3(ps_mul(offset, offset)).to_bits());
    let limits = solver + 0x64 + 0x1c * class;
    let max_distance = x87_load32(e.mem.u32(limits + 0x10 + 4 * index));
    let mut still = !(max_distance < distance);
    if still {
        // The distance from the reference orientation (all four lanes).
        let packed = e.mem.u32(state + 0x110 + 4 * index);
        let scale = x87_load64(e.mem.u64(PACKED_SCALE));
        let byte = |shift: u32| f64::from(((packed >> shift) & 0xff) as i32 - 0x80);
        let unpacked = [
            x87_mul(byte(0), scale),
            x87_mul(byte(8), scale),
            x87_mul(byte(16), scale),
            x87_mul(scale, byte(24)),
        ]
        .map(|value| f32::from_bits(x87_store32(value)));
        let max_rotation = f32::from_bits(u32::from(e.mem.u16(limits + 0x18 + 2 * index)) << 16);
        let turn = ps_sub(unpacked, load_vec(e, state + 0x80));
        let squares = ps_mul(turn, turn);
        let total = ss_add(
            ss_add(squares[3], squares[1]),
            ss_add(squares[2], squares[0]),
        );
        still = !(x87_load32(max_rotation.to_bits()) < x87_load32(total.to_bits()));
    }
    let select = u16::from(e.mem.u8(solver + 0x125 + index));
    let frames_at = state + 0xa + 2 * index;
    let frames = e.mem.u16(frames_at) & 0x7f;
    if still {
        let next = frames - (frames >> 6) + 1;
        e.mem
            .set_u16(frames_at, (((select << 7) | frames) << 7) | next);
    } else {
        e.mem.set_u16(frames_at, ((select << 7) | frames) << 7);
        let centre = load_bits(e, state + 0x60);
        let mask = load_bits(e, XYZ_MASK);
        let masked = std::array::from_fn(|lane| centre[lane] & mask[lane]);
        store_bits(e, reference, masked);
        let packed = e.call(PACK_QUATERNION, &args![state + 0x80]).u32();
        e.mem.set_u32(state + 0x110 + 4 * index, packed);
    }
}

/// The smaller of `least` and the larger of the two inactive frame counts (bits 0 to 6 of
/// `+0xA` and `+0xC`).
fn fewest_inactive_frames(e: &Engine, state: u32, least: i32) -> i32 {
    let second = i32::from(e.mem.u16(state + 0xc) & 0x7f);
    let first = i32::from(e.mem.u16(state + 0xa) & 0x7f);
    least.min(first.max(second))
}

/// The validity test of a motion's own velocities (reset to the vector at `01268370` when it
/// fails) and the previous centre.
fn check_velocities(e: &mut Engine, info: u32, state: u32) {
    let linear = load_bits(e, state + 0xd0);
    let angular = load_bits(e, state + 0xe0);
    if !velocities_valid(e, linear, angular) {
        let reset = load_bits(e, VELOCITY_RESET);
        store_bits(e, state + 0xd0, reset);
        let reset = load_bits(e, VELOCITY_RESET);
        store_bits(e, state + 0xe0, reset);
    }
    keep_previous_centre(e, info, state);
}

// Translated from 00d28a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRigidMotionUtilApplyForcesAndStep` (engine map), cdecl with the `hkpSolverInfo`, the
/// `hkStepInfo`, the gravity vector, the motion list, the number of motions and the offset of
/// the `hkpMotion` in each listed object; answers the fewest inactive frames (see
/// `fewest_inactive_frames`; `0x7FFFFFFF` without motions). By the motion type (`+8`; the
/// compiler's jump table for 4 to 7, `00d2981c`):
///
/// * 5: nothing (and the motion does not count for the answer);
/// * 6: the gravity and the damping, then the angular momentum kept through the step: the
///   angular velocity through the rows (`004b4ab0`), times the refined reciprocal of
///   `m_inertiaAndMassInv` (`+0xC0`), rotated (`setRotatedDir`) into a stack slot; the
///   validity test, the integration, and the angular velocity made again from that momentum
///   (through the new rows, times the inverse inertia, rotated); then `limit_velocities`;
/// * 7: the damping; 4: nothing; any other: the gravity and the damping; then the validity
///   test and the integration.
///
/// Every motion but type 5 then goes through `deactivation_step` and counts for the answer.
/// The code keeps the constants 1 and 0 on the x87 stack and uses them as `1.0` and `+0`.
pub fn hk_rigid_motion_util_apply_forces_and_step(
    e: &mut Engine,
    solver_info: Ptr,
    info: Ptr,
    gravity: Ptr,
    motions: Ptr,
    count: i32,
    offset: u32,
) -> i32 {
    let mut least = i32::MAX;
    let mut remaining = count.wrapping_sub(1);
    if remaining < 0 {
        return least;
    }
    let solver = solver_info.addr();
    let info = info.addr();
    let gravity = gravity.addr();
    let mut list = motions.addr();
    e.with_stack(0x100, |e, frame| {
        let frame = frame.addr();
        loop {
            let state = e.mem.u32(list).wrapping_add(offset);
            let kind = e.mem.u8(state + 8);
            if kind != 5 {
                if kind == 6 {
                    add_gravity(e, state, gravity);
                    damp_motion(e, info, state);
                    e.call(
                        ROWS_TIMES_VECTOR,
                        &args![frame + 0xd0, state + 0x10, state + 0xe0],
                    );
                    let inertia = refined_rcp(load_vec(e, state + 0xc0));
                    let momentum = ps_mul(inertia, load_vec(e, frame + 0xd0));
                    store_vec(e, frame + 0xd0, momentum);
                    e.call(
                        SET_ROTATED_DIR,
                        &args![frame + 0xf0, state + 0x10, frame + 0xd0],
                    );
                    check_velocities(e, info, state);
                    let at = Integration {
                        linear: state + 0xd0,
                        linear_copy: frame + 0x30,
                        linear_target: Some(state + 0xd0),
                        angular: state + 0xe0,
                        angular_copy: frame + 0x30,
                        rotation: frame + 0x40,
                        delta: frame + 0x90,
                    };
                    integrate(e, info, state, &at);
                    e.call(
                        ROWS_TIMES_VECTOR,
                        &args![frame + 0xe0, state + 0x10, frame + 0xf0],
                    );
                    let spin = ps_mul(load_vec(e, frame + 0xe0), load_vec(e, state + 0xc0));
                    store_vec(e, frame + 0xe0, spin);
                    e.call(
                        SET_ROTATED_DIR,
                        &args![state + 0xe0, state + 0x10, frame + 0xe0],
                    );
                    limit_velocities(e, state, frame + 0x30);
                } else {
                    if kind != 4 && kind != 7 {
                        add_gravity(e, state, gravity);
                    }
                    if kind != 4 {
                        damp_motion(e, info, state);
                    }
                    check_velocities(e, info, state);
                    let at = Integration {
                        linear: state + 0xd0,
                        linear_copy: frame + 0x30,
                        linear_target: Some(state + 0xd0),
                        angular: state + 0xe0,
                        angular_copy: frame + 0x30,
                        rotation: frame + 0x50,
                        delta: frame + 0xa0,
                    };
                    integrate(e, info, state, &at);
                }
                deactivation_step(e, solver, state, 1.0);
                least = fewest_inactive_frames(e, state, least);
            }
            list = list.wrapping_add(4);
            remaining = remaining.wrapping_sub(1);
            if remaining < 0 {
                break;
            }
        }
    });
    least
}

// Translated from 00d29bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRigidMotionUtilApplyAccumulators` (engine map), cdecl with the `hkpSolverInfo`, the
/// `hkStepInfo`, the `hkpVelocityAccumulator` array (0x80 bytes per motion), the motion list,
/// the number of motions and the offset of the `hkpMotion` in each listed object; answers the
/// fewest inactive frames like `00d28a30`. By the motion type (`+8`; jump table `00d2a5cc`):
///
/// * 0 and 6: the accumulator's angular velocity (`+0x20`) times the refined reciprocal of
///   `m_inertiaAndMassInv`, rotated into a stack slot (the momentum), then as 3;
/// * 3: the linear velocity from the accumulator (`+0x10`), the angular velocity its `+0x20`
///   rotated by the motion's columns, the step taken with the accumulator's `+0x40` (linear)
///   and its `+0x50` rotated (`setRotatedDir`, angular);
/// * 4: the step taken with the motion's own velocities;
/// * 2 and 7: both velocities from the accumulator (`+0x10`, `+0x20`), the step taken with its
///   `+0x40` and `+0x50`;
///
/// those types then go through the validity test of the step's two vectors (both reset to the
/// vector at `01268370` when it fails), the previous centre and the integration (the scaled
/// linear vector is used but not stored). Type 6 then takes the linear velocity from the
/// accumulator again and its angular velocity from the momentum (through the transposed
/// rows, times the inverse inertia, rotated). Every motion (1, 5 and above 7 too) then goes
/// through `limit_velocities` and `deactivation_step` and counts for the answer.
pub fn hk_rigid_motion_util_apply_accumulators(
    e: &mut Engine,
    solver_info: Ptr,
    info: Ptr,
    accumulators: Ptr,
    motions: Ptr,
    count: i32,
    offset: u32,
) -> i32 {
    let mut least = i32::MAX;
    let mut remaining = count.wrapping_sub(1);
    if remaining < 0 {
        return least;
    }
    let solver = solver_info.addr();
    let info = info.addr();
    let mut accumulator = accumulators.addr();
    let mut list = motions.addr();
    e.with_stack(0x100, |e, frame| {
        let frame = frame.addr();
        loop {
            let state = e.mem.u32(list).wrapping_add(offset);
            let kind = e.mem.u8(state + 8);
            let steps = match kind {
                0 | 3 | 6 => {
                    if kind != 3 {
                        let angular = load_bits(e, accumulator + 0x20);
                        store_bits(e, frame + 0x80, angular);
                        let inertia = refined_rcp(load_vec(e, state + 0xc0));
                        let momentum = ps_mul(inertia, load_vec(e, frame + 0x80));
                        store_vec(e, frame + 0x80, momentum);
                        e.call(
                            SET_ROTATED_DIR,
                            &args![frame + 0xd0, state + 0x10, frame + 0x80],
                        );
                    }
                    let linear = load_bits(e, accumulator + 0x10);
                    store_bits(e, state + 0xd0, linear);
                    let angular = load_vec(e, accumulator + 0x20);
                    let rotated = rotate_by_columns(e, angular, state + 0x10);
                    store_vec(e, state + 0xe0, rotated);
                    let moving = load_bits(e, accumulator + 0x40);
                    store_bits(e, frame + 0xa0, moving);
                    e.call(
                        SET_ROTATED_DIR,
                        &args![frame + 0x70, state + 0x10, accumulator + 0x50],
                    );
                    true
                }
                4 => {
                    let linear = load_bits(e, state + 0xd0);
                    let angular = load_bits(e, state + 0xe0);
                    store_bits(e, frame + 0x70, angular);
                    store_bits(e, frame + 0xa0, linear);
                    true
                }
                2 | 7 => {
                    let linear = load_bits(e, accumulator + 0x10);
                    store_bits(e, state + 0xd0, linear);
                    let angular = load_bits(e, accumulator + 0x20);
                    store_bits(e, state + 0xe0, angular);
                    let moving = load_bits(e, accumulator + 0x40);
                    let turning = load_bits(e, accumulator + 0x50);
                    store_bits(e, frame + 0x70, turning);
                    store_bits(e, frame + 0xa0, moving);
                    true
                }
                _ => false,
            };
            if steps {
                let turning = load_bits(e, frame + 0x70);
                let moving = load_bits(e, frame + 0xa0);
                if !velocities_valid(e, moving, turning) {
                    let reset = load_bits(e, VELOCITY_RESET);
                    store_bits(e, frame + 0xa0, reset);
                    store_bits(e, frame + 0x70, reset);
                }
                keep_previous_centre(e, info, state);
                let at = Integration {
                    linear: frame + 0xa0,
                    linear_copy: frame + 0xa0,
                    linear_target: None,
                    angular: frame + 0x70,
                    angular_copy: frame + 0x70,
                    rotation: frame + 0x40,
                    delta: frame + 0x90,
                };
                integrate(e, info, state, &at);
            }
            if e.mem.u8(state + 8) == 6 {
                let linear = load_bits(e, accumulator + 0x10);
                store_bits(e, state + 0xd0, linear);
                // The rows transposed into columns (`UNPCKLPS`/`UNPCKHPS`/`MOVLHPS`/
                // `MOVHLPS`/`SHUFPS`), the fourth lanes as the shuffles leave them.
                let row_0 = load_vec(e, state + 0x10);
                let row_1 = load_vec(e, state + 0x20);
                let row_2 = load_vec(e, state + 0x30);
                let high = unpckhps(row_0, row_1);
                let low = unpcklps(row_0, row_1);
                let column_0 = movlhps(low, row_2);
                let column_1 = shufps(movhlps(column_0, low), row_2, 0xd4);
                let momentum = load_vec(e, frame + 0xd0);
                let first = ps_mul([momentum[1]; 4], column_1);
                let second = ps_mul([momentum[0]; 4], column_0);
                let sum = ps_add(first, second);
                let column_2 = shufps(high, row_2, 0xe4);
                let sum = ps_add(sum, ps_mul([momentum[2]; 4], column_2));
                let spin = ps_mul(sum, load_vec(e, state + 0xc0));
                store_vec(e, frame + 0xe0, spin);
                e.call(
                    SET_ROTATED_DIR,
                    &args![state + 0xe0, state + 0x10, frame + 0xe0],
                );
            }
            limit_velocities(e, state, frame + 0xf0);
            let one = e.mem.f64(ONE_DOUBLE);
            deactivation_step(e, solver, state, one);
            least = fewest_inactive_frames(e, state, least);
            list = list.wrapping_add(4);
            accumulator = accumulator.wrapping_add(0x80);
            remaining = remaining.wrapping_sub(1);
            if remaining < 0 {
                break;
            }
        }
    });
    least
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_8680, hk_rigid_motion_util_step(Ptr, Ptr, i32, u32)),
        entry!(
            0x00d2_9830,
            hk_rigid_motion_util_apply_forces_and_build_accumulators(Ptr, Ptr, i32, u32, Ptr) -> Ptr
        ),
        entry!(
            0x00d2_8a30,
            hk_rigid_motion_util_apply_forces_and_step(Ptr, Ptr, Ptr, Ptr, i32, u32) -> i32
        ),
        entry!(
            0x00d2_9bf0,
            hk_rigid_motion_util_apply_accumulators(Ptr, Ptr, Ptr, Ptr, i32, u32) -> i32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The `4 / pi^2` factor, the series coefficients and pi as the exe stores them (doubles).
    const FACTOR: f64 = 0.405_284_7;
    const COEFFICIENTS: [f64; 3] = [0.822_948, 0.130_529, 0.044_408];

    /// The offset of the motion state inside each listed object.
    const OFFSET: u32 = 0x10;

    fn close(got: f32, expected: f64) {
        let error = (f64::from(got) - expected).abs();
        assert!(
            error <= 1e-5 * expected.abs().max(1e-3),
            "{got} is not {expected}"
        );
    }

    fn set_constants(e: &mut Engine) {
        for lane in 0..4 {
            e.set_global(ABS_MASK + 4 * lane, 0x7fff_ffffu32);
            e.set_global(VELOCITY_LIMIT + 4 * lane, 1.0e6f32);
            e.set_global(VELOCITY_RESET + 4 * lane, 7.0f32);
            e.set_global(ONES + 4 * lane, 1.0f32);
        }
        e.set_global(HALF, 0.5f64);
        e.set_global(ANGLE_FACTOR, FACTOR);
        e.set_global(ANGLE_LIMIT, f64::from(0.9f32));
        e.set_global(ANGLE_LIMIT_FLOAT, 0.9f32);
        e.set_global(SERIES_FIRST, COEFFICIENTS[0]);
        e.set_global(SERIES_SECOND, COEFFICIENTS[1]);
        e.set_global(SERIES_THIRD, COEFFICIENTS[2]);
        e.set_global(PI, f64::from(std::f32::consts::PI));
        // Quality 1 allows a speed of 10 and a step of 0.5; quality 2 a step of 5.
        e.set_global(QUALITY_TABLE + 4, 10.0f32);
        e.set_global(QUALITY_TABLE + 8, 0.5f32);
    }

    /// The step info: the first float, the time step and the other float.
    fn info(e: &mut Engine, first: f32, step: f32, second: f32) -> u32 {
        let info = e.mem.alloc(0x10);
        e.mem.set_f32(info, first);
        e.mem.set_f32(info + 8, step);
        e.mem.set_f32(info + 0xc, second);
        info
    }

    /// A motion state of `kind` (filled with `0xab`) inside an object `OFFSET` bytes before it,
    /// and a list holding the objects. Returns (list, states).
    fn motions(e: &mut Engine, kinds: &[u8]) -> (u32, Vec<u32>) {
        let list = e.mem.alloc(4 * kinds.len() as u32 + 4);
        let mut states = Vec::new();
        for (index, kind) in kinds.iter().enumerate() {
            let object = e.mem.alloc(0x180);
            e.mem.write(object, &[0xab; 0x180]);
            let state = object + OFFSET;
            e.mem.set_u8(state + 8, *kind);
            e.mem.set_u32(list + 4 * index as u32, object);
            states.push(state);
        }
        (list, states)
    }

    fn quiet_state(e: &mut Engine, state: u32) {
        // Valid velocities and zero everything the step reads as a vector.
        for offset in [0x10, 0x20, 0x30, 0x50, 0x60, 0x80, 0x90, 0xd0, 0xe0] {
            store_vec(e, state + offset, [0.0; 4]);
        }
        e.mem.set_u8(state + 0xbc, 1);
        e.mem.set_u8(state + 0xbd, 2);
    }

    /// The doubles of the step: the root (host), the quaternion product (writes `product`),
    /// the normalisation and the rotation set (identity columns). The roots asked for and the
    /// deltas the product saw are returned.
    struct Doubles {
        log: Log,
        roots: Rc<RefCell<Vec<f64>>>,
        deltas: Rc<RefCell<Vec<Vec4>>>,
    }

    const PRODUCT: Vec4 = [0.1, 0.2, 0.3, 0.9];

    fn step_doubles(e: &mut Engine) -> Doubles {
        let log = new_log();
        let roots: Rc<RefCell<Vec<f64>>> = Rc::default();
        let seen = Rc::clone(&roots);
        e.register_double(X87_SQRT, move |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            seen.borrow_mut().push(value);
            Ret {
                st0: value.sqrt(),
                ..Ret::default()
            }
        });
        let deltas: Rc<RefCell<Vec<Vec4>>> = Rc::default();
        let seen = Rc::clone(&deltas);
        let logged = Rc::clone(&log);
        e.register_double(QUATERNION_SET_MUL, move |e, a| {
            logged.borrow_mut().push((QUATERNION_SET_MUL, a.to_vec()));
            seen.borrow_mut().push(load_vec(e, a[1]));
            store_vec(e, a[0], PRODUCT);
            Ret::default()
        });
        log_double(e, &log, QUATERNION_NORMALIZE, 0);
        let logged = Rc::clone(&log);
        e.register_double(SET_ROTATION, move |e, a| {
            logged.borrow_mut().push((SET_ROTATION, a.to_vec()));
            store_vec(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            store_vec(e, a[0] + 0x10, [0.0, 1.0, 0.0, 0.0]);
            store_vec(e, a[0] + 0x20, [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        Doubles { log, roots, deltas }
    }

    #[test]
    fn a_motion_of_type_5_is_left_alone_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        let doubles = step_doubles(&mut e);
        let info = info(&mut e, 1.0, 0.5, 2.0);
        let (list, states) = motions(&mut e, &[5]);
        e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
        for offset in 0..0x140 {
            assert_eq!(e.mem.u8(states[0] + offset + 9), 0xab, "offset {offset:#x}");
        }
        assert!(doubles.log.borrow().is_empty());
        assert!(doubles.roots.borrow().is_empty());
    }

    #[test]
    fn no_motions_do_nothing_00d28680() {
        for count in [0i32, -3] {
            let mut e = engine();
            set_constants(&mut e);
            let doubles = step_doubles(&mut e);
            let info = info(&mut e, 1.0, 0.5, 2.0);
            let (list, states) = motions(&mut e, &[0]);
            e.call(0x00d2_8680, &args![info, list, count, OFFSET]);
            assert_eq!(e.mem.u8(states[0] + 9), 0xab);
            assert!(doubles.roots.borrow().is_empty());
        }
    }

    #[test]
    fn a_slow_motion_moves_by_its_velocity_and_turns_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        let doubles = step_doubles(&mut e);
        let info = info(&mut e, 7.0, 0.5, 9.0);
        let (list, states) = motions(&mut e, &[0]);
        let state = states[0];
        quiet_state(&mut e, state);
        store_vec(&mut e, state + 0xd0, [3.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, state + 0x60, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, state + 0x80, [0.5; 4]);
        store_vec(&mut e, state + 0x90, [1.0, 2.0, 3.0, 0.0]);
        e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
        // The previous position carries the first float; the position moved by 0.5 * 3 and
        // carries the other float.
        assert_eq!(load_vec(&e, state + 0x50), [1.0, 2.0, 3.0, 7.0]);
        assert_eq!(load_vec(&e, state + 0x60), [2.5, 2.0, 3.0, 9.0]);
        assert_eq!(load_vec(&e, state + 0x70), [0.5; 4]);
        // No angular velocity: the delta is (0, 0, 0, cos = 1) and the angle is 0.
        assert_eq!(*doubles.deltas.borrow(), vec![[0.0, 0.0, 0.0, 1.0]]);
        assert_eq!(*doubles.roots.borrow(), vec![0.0]);
        assert_eq!(load_vec(&e, state + 0xa0), [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, state + 0x80), PRODUCT);
        let frame_calls = calls(&doubles.log, QUATERNION_SET_MUL);
        assert_eq!(frame_calls.len(), 1);
        let frame = frame_calls[0][0];
        assert_eq!(frame_calls[0][1..], [frame + 0x10, frame]);
        assert_eq!(calls(&doubles.log, QUATERNION_NORMALIZE), vec![vec![frame]]);
        assert_eq!(
            calls(&doubles.log, SET_ROTATION),
            vec![vec![state + 0x10, state + 0x80]]
        );
        // The translation is the position minus the centre rotated by the identity columns.
        assert_eq!(load_vec(&e, state + 0x40), [1.5, 0.0, 0.0, 9.0]);
        // Bytes the function does not touch keep their fill.
        assert_eq!(e.mem.u8(state + 0xb0), 0xab);
    }

    #[test]
    fn a_fast_motion_is_scaled_to_its_speed_limit_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        e.set_global(QUALITY_TABLE + 4, 1.0f32);
        let doubles = step_doubles(&mut e);
        let info = info(&mut e, 7.0, 1.0, 9.0);
        let (list, states) = motions(&mut e, &[0]);
        let state = states[0];
        quiet_state(&mut e, state);
        store_vec(&mut e, state + 0xd0, [3.0, 4.0, 0.0, 0.0]);
        e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
        let inverse = (1.0f64 / 5.0) as f32;
        let factor = (f64::from(inverse) * 1.0) as f32;
        assert_eq!(
            load_vec(&e, state + 0xd0),
            [factor * 3.0, factor * 4.0, factor * 0.0, factor * 0.0]
        );
        // The position moved by the scaled velocity (time step 1).
        assert_eq!(
            load_vec(&e, state + 0x60)[..3],
            [factor * 3.0, factor * 4.0, 0.0]
        );
        assert_eq!(*doubles.roots.borrow(), vec![25.0, 0.0]);
    }

    #[test]
    fn a_speed_exactly_at_the_limit_is_kept_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        e.set_global(QUALITY_TABLE + 4, 5.0f32);
        let doubles = step_doubles(&mut e);
        let info = info(&mut e, 7.0, 1.0, 9.0);
        let (list, states) = motions(&mut e, &[0]);
        let state = states[0];
        quiet_state(&mut e, state);
        store_vec(&mut e, state + 0xd0, [3.0, 4.0, 0.0, 0.0]);
        e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
        assert_eq!(load_vec(&e, state + 0xd0), [3.0, 4.0, 0.0, 0.0]);
        assert_eq!(*doubles.roots.borrow(), vec![0.0]);
    }

    #[test]
    fn invalid_velocities_are_reset_00d28680() {
        let cases: [(Vec4, Vec4, bool); 6] = [
            ([0.0; 4], [f32::NAN, 0.0, 0.0, 0.0], true),
            ([0.0; 4], [0.0, 0.0, 2.0e6, 0.0], true),
            ([0.0, -1.0e6, 0.0, 0.0], [0.0; 4], true),
            ([0.0; 4], [0.0, 0.0, f32::NEG_INFINITY, 0.0], true),
            // The fourth lane is not looked at.
            ([0.0, 0.0, 0.0, 5.0e6], [0.0, 0.0, 0.0, f32::NAN], false),
            ([1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], false),
        ];
        for (linear, angular, reset) in cases {
            let mut e = engine();
            set_constants(&mut e);
            // A quality whose limits do not rescale anything.
            e.set_global(QUALITY_TABLE + 4, 1.0e9f32);
            let _doubles = step_doubles(&mut e);
            let info = info(&mut e, 7.0, 0.0, 9.0);
            let (list, states) = motions(&mut e, &[1]);
            let state = states[0];
            quiet_state(&mut e, state);
            store_vec(&mut e, state + 0xd0, linear);
            store_vec(&mut e, state + 0xe0, angular);
            e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
            let bits = |v: Vec4| v.map(f32::to_bits);
            if reset {
                assert_eq!(
                    load_vec(&e, state + 0xd0),
                    [7.0; 4],
                    "{linear:?} {angular:?}"
                );
                assert_eq!(load_vec(&e, state + 0xe0), [7.0; 4]);
            } else {
                assert_eq!(bits(load_vec(&e, state + 0xd0)), bits(linear));
            }
        }
    }

    #[test]
    fn a_fast_turn_is_scaled_to_the_allowed_angle_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        let doubles = step_doubles(&mut e);
        // Time step 1: half angle (2, 0, 0), estimate 4 * 0.4052847 = 1.62; quality 2 allows
        // 0.5, so the square 0.25 is below the estimate.
        let info = info(&mut e, 7.0, 1.0, 9.0);
        let (list, states) = motions(&mut e, &[3]);
        let state = states[0];
        quiet_state(&mut e, state);
        store_vec(&mut e, state + 0xe0, [4.0, 0.0, 0.0, 0.0]);
        e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
        let estimate = (4.0 * FACTOR) as f32;
        let roots = doubles.roots.borrow().clone();
        assert_eq!(roots.len(), 2);
        assert_eq!(roots[0], f64::from(estimate));
        assert_eq!(roots[1], 0.25);
        let inverse = (1.0 / f64::from(estimate).sqrt() as f32 as f64) as f32;
        let factor = (f64::from(inverse) * 0.5) as f32;
        let angular = load_vec(&e, state + 0xe0);
        assert_eq!(angular[0], factor * 4.0);
        assert_eq!(angular[1], factor * 0.0);
        // The delta: the scaled half angle and the cosine of the allowed angle.
        let deltas = doubles.deltas.borrow();
        assert_eq!(deltas[0][0], factor * 2.0);
        let squared = 0.25f64;
        close(
            deltas[0][3],
            1.0 - COEFFICIENTS[0] * squared
                - COEFFICIENTS[1] * squared * squared
                - squared * squared * squared * COEFFICIENTS[2],
        );
        // Twice the delta, and the angle (the root of the allowed square, 0.5) times pi.
        close(load_vec(&e, state + 0xa0)[0], f64::from(factor * 2.0 * 2.0));
        close(load_vec(&e, state + 0xa0)[3], 0.5 * std::f64::consts::PI);
    }

    #[test]
    fn the_allowed_step_is_capped_at_0_9_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        // Quality 2 now allows 5 per time unit, over 0.9.
        e.set_global(QUALITY_TABLE + 8, 5.0f32);
        let doubles = step_doubles(&mut e);
        let info = info(&mut e, 7.0, 1.0, 9.0);
        let (list, states) = motions(&mut e, &[3]);
        let state = states[0];
        quiet_state(&mut e, state);
        store_vec(&mut e, state + 0xe0, [4.0, 0.0, 0.0, 0.0]);
        e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
        // The estimate 1.62 is above the capped square (0.81) but below 25 (uncapped).
        let capped = (f64::from(0.9f32) * f64::from(0.9f32)) as f32;
        let roots = doubles.roots.borrow().clone();
        assert_eq!(roots.len(), 2);
        assert_eq!(roots[1], f64::from(capped));
    }

    #[test]
    fn a_small_turn_keeps_the_angular_velocity_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        let doubles = step_doubles(&mut e);
        // Time step 0.1: half angle (0.05, 0, 0), estimate 0.0025 * 0.4052847; quality 2
        // allows 0.05, whose square 0.0025 is above the estimate.
        let info = info(&mut e, 7.0, 0.1, 9.0);
        let (list, states) = motions(&mut e, &[3]);
        let state = states[0];
        quiet_state(&mut e, state);
        store_vec(&mut e, state + 0xe0, [1.0, 0.0, 0.0, 0.0]);
        e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
        assert_eq!(load_vec(&e, state + 0xe0), [1.0, 0.0, 0.0, 0.0]);
        let half = (f64::from(0.1f32) * 0.5) as f32;
        assert_eq!(doubles.deltas.borrow()[0][0], half * 1.0);
        let estimate = f64::from((f64::from(half * 1.0 * (half * 1.0)) * FACTOR) as f32);
        close(
            doubles.deltas.borrow()[0][3],
            1.0 - COEFFICIENTS[0] * estimate - COEFFICIENTS[1] * estimate * estimate,
        );
        // The angle is the root of the estimate times pi.
        assert_eq!(doubles.roots.borrow().len(), 1);
    }

    #[test]
    fn several_motions_are_taken_from_the_list_in_order_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        let _doubles = step_doubles(&mut e);
        let info = info(&mut e, 7.0, 1.0, 9.0);
        let (list, states) = motions(&mut e, &[0, 5, 6]);
        for state in [states[0], states[2]] {
            quiet_state(&mut e, state);
        }
        store_vec(&mut e, states[0] + 0xd0, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, states[2] + 0xd0, [0.0, 2.0, 0.0, 0.0]);
        e.call(0x00d2_8680, &args![info, list, 3i32, OFFSET]);
        assert_eq!(load_vec(&e, states[0] + 0x60)[..3], [1.0, 0.0, 0.0]);
        assert_eq!(e.mem.u8(states[1] + 0x60), 0xab);
        assert_eq!(load_vec(&e, states[2] + 0x60)[..3], [0.0, 2.0, 0.0]);
    }

    /// The accumulator block for `count` motions, filled with `0xcd`.
    fn accumulators(e: &mut Engine, count: u32) -> u32 {
        let block = e.mem.alloc(0x80 * count.max(1));
        e.mem
            .write(block, &vec![0xcd; 0x80 * count.max(1) as usize]);
        block
    }

    /// The matrix transpose double: it writes three known columns at its destination.
    fn transpose_double(e: &mut Engine, log: &Log) {
        let logged = Rc::clone(log);
        e.register_double(MATRIX3_SET_TRANSPOSE, move |e, a| {
            logged
                .borrow_mut()
                .push((MATRIX3_SET_TRANSPOSE, a.to_vec()));
            store_vec(e, a[0], [1.0, 2.0, 3.0, 0.0]);
            store_vec(e, a[0] + 0x10, [4.0, 5.0, 6.0, 0.0]);
            store_vec(e, a[0] + 0x20, [7.0, 8.0, 9.0, 0.0]);
            Ret::default()
        });
    }

    /// A state for the force functions: positions, velocities, damping and the two bytes.
    fn force_state(e: &mut Engine, state: u32) {
        store_vec(e, state + 0x50, [1.0, 1.0, 1.0, 2.0]);
        store_vec(e, state + 0x60, [3.0, 5.0, 7.0, 0.5]);
        store_vec(e, state + 0xc0, [-1.0, -2.0, -3.0, -4.0]);
        store_vec(e, state + 0xd0, [10.0, 20.0, 30.0, 40.0]);
        store_vec(e, state + 0xe0, [1.0, 10.0, 100.0, 0.0]);
        e.mem.set_f32(state + 0xb4, 0.0);
        e.mem.set_f32(state + 0xb8, 0.0);
        e.mem.set_u8(state + 0xbe, 3);
        e.mem.set_i16(state + 0x11e, -2);
    }

    #[test]
    fn the_motion_types_0_3_and_6_build_the_same_accumulator_00d29830() {
        for kind in [0u8, 3, 6] {
            let mut e = engine();
            set_constants(&mut e);
            let log = new_log();
            transpose_double(&mut e, &log);
            let info = info(&mut e, 5.0, 0.5, 0.0);
            let (list, states) = motions(&mut e, &[kind]);
            let state = states[0];
            force_state(&mut e, state);
            let acc = accumulators(&mut e, 1);
            let end = e
                .call(0x00d2_9830, &args![info, list, 1i32, OFFSET, acc])
                .u32();
            assert_eq!(end, acc + 0x80);
            assert_eq!(
                calls(&log, MATRIX3_SET_TRANSPOSE),
                vec![vec![acc + 0x50, state + 0x10]]
            );
            assert_eq!(e.mem.u8(acc), 0);
            assert_eq!(e.mem.u8(acc + 1), 0xcd);
            assert_eq!(e.mem.u32(acc + 4), 3);
            assert_eq!(e.mem.u32(acc + 8), 0xfffe_0000);
            assert_eq!(load_vec(&e, acc + 0x10), [10.0, 20.0, 30.0, 40.0]);
            assert_eq!(load_vec(&e, acc + 0x30), [-1.0, -2.0, -3.0, -4.0]);
            // t = (5 - 2) * 0.5 = 1.5: (1 - t) * previous + current * t.
            assert_eq!(load_vec(&e, acc + 0x40), [4.0, 7.0, 10.0, -0.25]);
            // The angular velocity (1, 10, 100) through the transposed columns.
            assert_eq!(load_vec(&e, acc + 0x20), [741.0, 852.0, 963.0, 0.0]);
            // The transposed columns stay where the transpose wrote them.
            assert_eq!(load_vec(&e, acc + 0x50), [1.0, 2.0, 3.0, 0.0]);
            // The state's velocities were damped by factors of 1.
            assert_eq!(load_vec(&e, state + 0xd0), [10.0, 20.0, 30.0, 40.0]);
        }
    }

    #[test]
    fn the_damping_factors_come_from_the_time_step_00d29830() {
        for kind in [0u8, 2, 7] {
            let mut e = engine();
            set_constants(&mut e);
            let log = new_log();
            transpose_double(&mut e, &log);
            let info = info(&mut e, 5.0, 0.5, 0.0);
            let (list, states) = motions(&mut e, &[kind]);
            let state = states[0];
            force_state(&mut e, state);
            e.mem.set_f32(state + 0xb4, 0.2);
            e.mem.set_f32(state + 0xb8, 4.0);
            let acc = accumulators(&mut e, 1);
            e.call(0x00d2_9830, &args![info, list, 1i32, OFFSET, acc]);
            let linear = (1.0 - f64::from(0.2f32) * 0.5) as f32;
            assert_eq!(
                load_vec(&e, state + 0xd0),
                [linear * 10.0, linear * 20.0, linear * 30.0, linear * 40.0]
            );
            // 1 - 4 * 0.5 is below zero: the angular velocity is zeroed (not negated).
            let angular = load_vec(&e, state + 0xe0);
            assert_eq!(angular.map(f32::to_bits), [0; 4]);
        }
    }

    #[test]
    fn a_nan_time_step_gives_nan_velocities_00d29830() {
        let mut e = engine();
        set_constants(&mut e);
        let info = info(&mut e, 5.0, f32::NAN, 0.0);
        let (list, states) = motions(&mut e, &[2]);
        let state = states[0];
        force_state(&mut e, state);
        e.mem.set_f32(state + 0xb4, 0.2);
        let acc = accumulators(&mut e, 1);
        e.call(0x00d2_9830, &args![info, list, 1i32, OFFSET, acc]);
        assert!(load_vec(&e, state + 0xd0).iter().all(|lane| lane.is_nan()));
        assert!(load_vec(&e, state + 0xe0).iter().all(|lane| lane.is_nan()));
    }

    #[test]
    fn the_motion_types_2_and_7_keep_the_velocities_and_the_identity_inertia_00d29830() {
        for (kind, flag) in [(2u8, 0u8), (7, 2)] {
            let mut e = engine();
            set_constants(&mut e);
            let info = info(&mut e, 5.0, 0.5, 0.0);
            let (list, states) = motions(&mut e, &[kind]);
            let state = states[0];
            force_state(&mut e, state);
            let acc = accumulators(&mut e, 1);
            let end = e
                .call(0x00d2_9830, &args![info, list, 1i32, OFFSET, acc])
                .u32();
            assert_eq!(end, acc + 0x80);
            assert_eq!(e.mem.u8(acc), flag);
            assert_eq!(e.mem.u32(acc + 4), 3);
            assert_eq!(e.mem.u32(acc + 8), 0xfffe_0000);
            assert_eq!(load_vec(&e, acc + 0x10), [10.0, 20.0, 30.0, 40.0]);
            assert_eq!(load_vec(&e, acc + 0x20), [1.0, 10.0, 100.0, 0.0]);
            assert_eq!(load_vec(&e, acc + 0x30), [-1.0, -2.0, -3.0, -4.0]);
            assert_eq!(load_vec(&e, acc + 0x40), [4.0, 7.0, 10.0, -0.25]);
            assert_eq!(load_vec(&e, acc + 0x50), [1.0, 0.0, 0.0, 0.0]);
            assert_eq!(load_vec(&e, acc + 0x60), [0.0, 1.0, 0.0, 0.0]);
            assert_eq!(load_vec(&e, acc + 0x70), [0.0, 0.0, 1.0, 0.0]);
        }
    }

    #[test]
    fn the_motion_type_4_copies_the_position_and_velocities_00d29830() {
        let mut e = engine();
        set_constants(&mut e);
        let info = info(&mut e, 5.0, 0.5, 0.0);
        let (list, states) = motions(&mut e, &[4]);
        let state = states[0];
        force_state(&mut e, state);
        let acc = accumulators(&mut e, 1);
        e.call(0x00d2_9830, &args![info, list, 1i32, OFFSET, acc]);
        assert_eq!(e.mem.u8(acc), 1);
        // The word at +4 is not written for this type.
        assert_eq!(e.mem.u32(acc + 4), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(acc + 8), 0xfffe_0000);
        assert_eq!(load_vec(&e, acc + 0x10), [10.0, 20.0, 30.0, 40.0]);
        assert_eq!(load_vec(&e, acc + 0x20), [1.0, 10.0, 100.0, 0.0]);
        assert_eq!(load_vec(&e, acc + 0x30), [0.0; 4]);
        assert_eq!(load_vec(&e, acc + 0x40), [3.0, 5.0, 7.0, 0.5]);
        assert_eq!(load_vec(&e, acc + 0x50), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, acc + 0x60), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, acc + 0x70), [0.0, 0.0, 1.0, 0.0]);
        // The state itself is not damped.
        assert_eq!(load_vec(&e, state + 0xd0), [10.0, 20.0, 30.0, 40.0]);
    }

    #[test]
    fn the_motion_type_5_clears_the_accumulator_00d29830() {
        let mut e = engine();
        set_constants(&mut e);
        let info = info(&mut e, 5.0, 0.5, 0.0);
        let (list, states) = motions(&mut e, &[5]);
        force_state(&mut e, states[0]);
        let acc = accumulators(&mut e, 1);
        e.call(0x00d2_9830, &args![info, list, 1i32, OFFSET, acc]);
        assert_eq!(e.mem.u8(acc), 1);
        assert_eq!(e.mem.u8(acc + 1), 0xcd);
        assert_eq!(e.mem.u32(acc + 4), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(acc + 8), 0);
        for offset in [0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70] {
            assert_eq!(load_vec(&e, acc + offset), [0.0; 4], "{offset:#x}");
        }
        assert_eq!(e.mem.u32(acc + 0xc), 0xcdcd_cdcd);
    }

    #[test]
    fn the_motion_types_1_and_above_7_leave_their_accumulator_alone_00d29830() {
        for kind in [1u8, 8, 0xff] {
            let mut e = engine();
            set_constants(&mut e);
            let info = info(&mut e, 5.0, 0.5, 0.0);
            let (list, states) = motions(&mut e, &[kind]);
            force_state(&mut e, states[0]);
            let acc = accumulators(&mut e, 1);
            let end = e
                .call(0x00d2_9830, &args![info, list, 1i32, OFFSET, acc])
                .u32();
            assert_eq!(end, acc + 0x80);
            for offset in 0..0x80 {
                assert_eq!(e.mem.u8(acc + offset), 0xcd);
            }
            assert_eq!(load_vec(&e, states[0] + 0xd0), [10.0, 20.0, 30.0, 40.0]);
        }
    }

    #[test]
    fn the_accumulators_follow_the_list_and_no_motions_answer_the_start_00d29830() {
        let mut e = engine();
        set_constants(&mut e);
        let info = info(&mut e, 5.0, 0.5, 0.0);
        let (list, states) = motions(&mut e, &[5, 1, 4]);
        for state in &states {
            force_state(&mut e, *state);
        }
        let acc = accumulators(&mut e, 3);
        let end = e
            .call(0x00d2_9830, &args![info, list, 3i32, OFFSET, acc])
            .u32();
        assert_eq!(end, acc + 0x180);
        assert_eq!(e.mem.u8(acc), 1);
        assert_eq!(e.mem.u8(acc + 0x80), 0xcd);
        assert_eq!(e.mem.u8(acc + 0x100), 1);
        for count in [0i32, -1] {
            let end = e
                .call(0x00d2_9830, &args![info, list, count, OFFSET, acc + 0x40])
                .u32();
            assert_eq!(end, acc + 0x40);
        }
    }

    /// The word the packing double answers.
    const PACKED: u32 = 0x1234_5678;

    /// The constants the two step functions read besides those of `set_constants`.
    fn set_step_constants(e: &mut Engine) {
        set_constants(e);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(PACKED_SCALE, f64::from_bits(0x3f81_9999_a000_0000));
        for lane in 0..4 {
            e.set_global(XYZ_MASK + 4 * lane, if lane < 3 { u32::MAX } else { 0 });
        }
    }

    /// Doubles for the step functions: the root (host), the quaternion product, the rotated
    /// direction and the rows product (each copies its vector argument into its result: the
    /// delta, the vector), the normalisation, the rotation set (identity rows) and the packing
    /// (answers `PACKED`).
    fn step_function_doubles(e: &mut Engine) -> Log {
        let log = new_log();
        let logged = Rc::clone(&log);
        e.register_double(X87_SQRT, move |_, a| {
            logged.borrow_mut().push((X87_SQRT, a.to_vec()));
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                st0: value.sqrt(),
                ..Ret::default()
            }
        });
        for address in [QUATERNION_SET_MUL, SET_ROTATED_DIR, ROWS_TIMES_VECTOR] {
            let logged = Rc::clone(&log);
            e.register_double(address, move |e, a| {
                logged.borrow_mut().push((address, a.to_vec()));
                let source = if address == QUATERNION_SET_MUL {
                    a[1]
                } else {
                    a[2]
                };
                let words = load_bits(e, source);
                store_bits(e, a[0], words);
                Ret::default()
            });
        }
        log_double(e, &log, QUATERNION_NORMALIZE, 0);
        log_double(e, &log, PACK_QUATERNION, PACKED);
        let logged = Rc::clone(&log);
        e.register_double(SET_ROTATION, move |e, a| {
            logged.borrow_mut().push((SET_ROTATION, a.to_vec()));
            store_vec(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            store_vec(e, a[0] + 0x10, [0.0, 1.0, 0.0, 0.0]);
            store_vec(e, a[0] + 0x20, [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        log
    }

    /// A solver info whose deactivation class 1 allows the squared distance and rotation 1
    /// (check 0) and 4 (check 1), with the select flags 1 and 2.
    fn solver_info(e: &mut Engine) -> u32 {
        let solver = e.mem.alloc(0x130);
        let limits = solver + 0x64 + 0x1c;
        e.mem.set_f32(limits + 0x10, 1.0);
        e.mem.set_f32(limits + 0x14, 4.0);
        e.mem.set_u16(limits + 0x18, 0x3f80);
        e.mem.set_u16(limits + 0x1a, 0x4080);
        e.mem.set_u8(solver + 0x125, 1);
        e.mem.set_u8(solver + 0x126, 2);
        solver
    }

    /// A quiet motion of class 1: identity rows, zero velocities and centres, the rotation
    /// (0, 0, 0, 1), no damping or gravity, radius 0.5, inverse inertia 2, counter 1,
    /// inactive frames 5 and 3, references at the centre with the packed rotation 0 and
    /// energies -1.
    fn step_state(e: &mut Engine, state: u32) {
        quiet_state(e, state);
        store_vec(e, state + 0x10, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, state + 0x20, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, state + 0x30, [0.0, 0.0, 1.0, 0.0]);
        for offset in [0x40, 0x70, 0xa0, 0xf0, 0x100] {
            store_vec(e, state + offset, [0.0; 4]);
        }
        store_vec(e, state + 0x80, [0.0, 0.0, 0.0, 1.0]);
        store_vec(e, state + 0xc0, [2.0; 4]);
        e.mem.set_f32(state + 0xfc, -1.0);
        e.mem.set_f32(state + 0x10c, -1.0);
        e.mem.set_f32(state + 0xb0, 0.5);
        e.mem.set_f32(state + 0xb4, 0.0);
        e.mem.set_f32(state + 0xb8, 0.0);
        e.mem.set_u16(state + 0x11e, 0);
        e.mem.set_u8(state + 0xbe, 1);
        e.mem.set_u8(state + 9, 1);
        e.mem.set_u16(state + 0xa, 5);
        e.mem.set_u16(state + 0xc, 3);
        e.mem.set_u32(state + 0x110, 0x8080_8080);
        e.mem.set_u32(state + 0x114, 0x8080_8080);
    }

    type StepSetup = (Engine, Log, u32, u32, u32, u32, Vec<u32>);

    /// An engine with the constants and doubles, a solver info, the step info (start 7, step
    /// 0.5, inverse step 9), a gravity vector (0, -2, 0, 0) and quiet motions of `kinds`.
    fn step_setup(kinds: &[u8]) -> StepSetup {
        let mut e = engine();
        set_step_constants(&mut e);
        let log = step_function_doubles(&mut e);
        let solver = solver_info(&mut e);
        let info = info(&mut e, 7.0, 0.5, 9.0);
        let gravity = e.mem.alloc(0x10);
        store_vec(&mut e, gravity, [0.0, -2.0, 0.0, 0.0]);
        let (list, states) = motions(&mut e, kinds);
        for state in &states {
            step_state(&mut e, *state);
        }
        (e, log, solver, info, gravity, list, states)
    }

    #[test]
    fn forces_and_step_by_motion_type_and_the_fewest_frames_00d28a30() {
        let (mut e, _log, solver, info, gravity, list, states) = step_setup(&[0, 5, 4, 7]);
        for state in &states {
            e.mem.set_u16(state + 0x11e, 0x3f80);
            e.mem.set_f32(state + 0xb4, 0.2);
            store_vec(&mut e, state + 0xd0, [1.0, 0.0, 0.0, 0.0]);
        }
        e.mem.set_u16(states[1] + 0xa, 1);
        e.mem.set_u16(states[2] + 0xa, 0x89);
        e.mem.set_u16(states[3] + 0xa, 7);
        let least = e
            .call(
                0x00d2_8a30,
                &args![solver, info, gravity, list, 4i32, OFFSET],
            )
            .u32();
        assert_eq!(least, 5);
        let factor = (1.0 - f64::from(0.2f32) * 0.5) as f32;
        // Type 0: gravity, then damping.
        let velocity = [factor, factor * -2.0, 0.0, 0.0];
        assert_eq!(load_vec(&e, states[0] + 0xd0), velocity);
        assert_eq!(
            load_vec(&e, states[0] + 0x60),
            [0.5 * velocity[0], 0.5 * velocity[1], 0.0, 9.0]
        );
        assert_eq!(load_vec(&e, states[0] + 0x50), [0.0, 0.0, 0.0, 7.0]);
        // Type 5: untouched.
        assert_eq!(load_vec(&e, states[1] + 0xd0), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u8(states[1] + 9), 1);
        // Type 4: neither gravity nor damping.
        assert_eq!(load_vec(&e, states[2] + 0xd0), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, states[2] + 0x60), [0.5, 0.0, 0.0, 9.0]);
        // Type 7: damping only.
        assert_eq!(load_vec(&e, states[3] + 0xd0), [factor, 0.0, 0.0, 0.0]);
        for state in [states[0], states[2], states[3]] {
            // The counters count up; no check runs at 2.
            assert_eq!(e.mem.u8(state + 9), 2);
            // No turn: the delta is (0, 0, 0, 1), the rotation takes it, the angle is 0.
            assert_eq!(load_vec(&e, state + 0x80), [0.0, 0.0, 0.0, 1.0]);
            assert_eq!(load_vec(&e, state + 0xa0), [0.0; 4]);
            assert_eq!(load_vec(&e, state + 0x70), [0.0, 0.0, 0.0, 1.0]);
            assert_eq!(load_vec(&e, state + 0x40), load_vec(&e, state + 0x60));
        }
        // No motions: the largest answer.
        let none = e
            .call(
                0x00d2_8a30,
                &args![solver, info, gravity, list, 0i32, OFFSET],
            )
            .u32();
        assert_eq!(none, 0x7fff_ffff);
    }

    #[test]
    fn a_type_6_motion_keeps_its_angular_momentum_00d28a30() {
        let (mut e, log, solver, info, gravity, list, states) = step_setup(&[6]);
        let state = states[0];
        let spin = [0.1, 0.0, 0.0, 0.0];
        store_vec(&mut e, state + 0xe0, spin);
        e.mem.set_u16(state + 0x11e, 0x3f80);
        e.call(
            0x00d2_8a30,
            &args![solver, info, gravity, list, 1i32, OFFSET],
        );
        // Gravity applied.
        assert_eq!(load_vec(&e, state + 0xd0), [0.0, -2.0, 0.0, 0.0]);
        let rows = calls(&log, ROWS_TIMES_VECTOR);
        let turns = calls(&log, SET_ROTATED_DIR);
        assert_eq!(rows.len(), 2);
        assert_eq!(turns.len(), 2);
        let frame = rows[0][0] - 0xd0;
        assert_eq!(rows[0], vec![frame + 0xd0, state + 0x10, state + 0xe0]);
        assert_eq!(turns[0], vec![frame + 0xf0, state + 0x10, frame + 0xd0]);
        assert_eq!(rows[1], vec![frame + 0xe0, state + 0x10, frame + 0xf0]);
        assert_eq!(turns[1], vec![state + 0xe0, state + 0x10, frame + 0xe0]);
        assert_eq!(
            calls(&log, QUATERNION_SET_MUL),
            vec![vec![frame + 0x40, frame + 0x90, frame + 0x40]]
        );
        let momentum = ps_mul(refined_rcp([2.0; 4]), spin);
        assert_eq!(load_vec(&e, state + 0xe0), ps_mul(momentum, [2.0; 4]));
        close(load_vec(&e, state + 0xe0)[0], 0.1);
    }

    #[test]
    fn invalid_velocities_reset_and_fast_motions_are_limited_00d28a30() {
        let (mut e, log, solver, info, gravity, list, states) = step_setup(&[4, 4]);
        for lane in 0..4 {
            e.set_global(VELOCITY_RESET + 4 * lane, 0.0f32);
        }
        store_vec(&mut e, states[0] + 0xd0, [f32::NAN, 0.0, 0.0, 0.0]);
        store_vec(&mut e, states[0] + 0xe0, [1.0, 0.0, 0.0, 0.0]);
        // Too fast in both: speed 50 against 10, a half angle of 1 against 0.25.
        store_vec(&mut e, states[1] + 0xd0, [30.0, 40.0, 0.0, 0.0]);
        store_vec(&mut e, states[1] + 0xe0, [4.0, 0.0, 0.0, 0.0]);
        e.call(
            0x00d2_8a30,
            &args![solver, info, gravity, list, 2i32, OFFSET],
        );
        assert_eq!(load_vec(&e, states[0] + 0xd0), [0.0; 4]);
        assert_eq!(load_vec(&e, states[0] + 0xe0), [0.0; 4]);
        let velocity = load_vec(&e, states[1] + 0xd0);
        close(velocity[0], 6.0);
        close(velocity[1], 8.0);
        let centre = load_vec(&e, states[1] + 0x60);
        close(centre[0], 3.0);
        close(centre[1], 4.0);
        // The half angle and the spin scaled by 0.25 / root(estimate) (type 4 has no limit
        // after the step).
        let delta = load_vec(&e, states[1] + 0x80);
        close(delta[0], 0.25 / FACTOR.sqrt());
        close(load_vec(&e, states[1] + 0xe0)[0], 1.0 / FACTOR.sqrt());
        let roots: Vec<f64> = calls(&log, X87_SQRT)
            .iter()
            .map(|a| f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32))
            .collect();
        // The speed (2500) and the allowed squared angle (0.0625) were rooted.
        assert!(roots.contains(&2500.0));
        assert!(roots.contains(&0.0625));
    }

    #[test]
    fn the_deactivation_checks_keep_or_reset_their_references_00d28a30() {
        // (counter, packed orientation, reference x) per motion.
        let cases: [(u8, u32, f32); 5] = [
            (3, 0x8080_8080, 0.0),
            (3, 0x8080_8080, 5.0),
            (3, 0x8080_8000, 0.0),
            (15, 0x8080_8080, 0.0),
            (0xff, 0x8080_8080, 0.0),
        ];
        let (mut e, log, solver, info, gravity, list, states) = step_setup(&[4; 5]);
        for (state, (counter, packed, x)) in states.iter().zip(cases) {
            e.mem.set_u8(state + 9, counter);
            e.mem.set_u32(state + 0x110, packed);
            e.mem.set_u32(state + 0x114, packed);
            store_vec(&mut e, state + 0xf0, [x, 0.0, 0.0, 0.0]);
            e.mem.set_f32(state + 0xfc, -1.0);
            e.mem.set_f32(state + 0x10c, 2.5);
        }
        let least = e
            .call(
                0x00d2_8a30,
                &args![solver, info, gravity, list, 5i32, OFFSET],
            )
            .u32();
        // Kept: frames 5 -> 6 with the select flag 1 above; the energy 0 beats -1.
        assert_eq!(e.mem.u16(states[0] + 0xa), (((1 << 7) | 5) << 7) | 6);
        assert_eq!(e.mem.u8(states[0] + 9), 4);
        assert_eq!(e.mem.f32(states[0] + 0xfc), 0.0);
        assert_eq!(e.mem.u32(states[0] + 0x110), 0x8080_8080);
        // Too far, or turned too far: cleared, the reference takes the centre, packed again.
        for state in [states[1], states[2]] {
            assert_eq!(e.mem.u16(state + 0xa), ((1 << 7) | 5) << 7);
            assert_eq!(load_vec(&e, state + 0xf0), [0.0; 4]);
            assert_eq!(e.mem.u32(state + 0x110), PACKED);
        }
        assert_eq!(
            calls(&log, PACK_QUATERNION),
            vec![vec![states[1] + 0x80], vec![states[2] + 0x80]]
        );
        // Counter 16: the second check (limits 4), the counter restarts.
        assert_eq!(e.mem.u8(states[3] + 9), 0);
        assert_eq!(e.mem.u16(states[3] + 0xc), (((2 << 7) | 3) << 7) | 4);
        assert_eq!(e.mem.u16(states[3] + 0xa), 5);
        assert_eq!(e.mem.f32(states[3] + 0x10c), 2.5);
        // Counter 256: 0xFF and no check.
        assert_eq!(e.mem.u8(states[4] + 9), 0xff);
        assert_eq!(e.mem.u16(states[4] + 0xa), 5);
        assert_eq!(e.mem.f32(states[4] + 0xfc), -1.0);
        // The smallest of max(6, 3), max(0, 3), max(0, 3), max(5, 4), max(5, 3).
        assert_eq!(least, 3);
    }

    #[test]
    fn accumulator_types_2_7_and_4_take_their_velocities_00d29bf0() {
        let (mut e, log, solver, info, _gravity, list, states) = step_setup(&[2, 7, 4]);
        let acc = accumulators(&mut e, 3);
        for (index, state) in states.iter().enumerate() {
            let at = acc + 0x80 * index as u32;
            store_vec(&mut e, at + 0x10, [1.0, 2.0, 3.0, 0.0]);
            store_vec(&mut e, at + 0x20, [0.1, 0.2, 0.3, 0.0]);
            store_vec(&mut e, at + 0x40, [4.0, 0.0, 0.0, 0.0]);
            store_vec(&mut e, at + 0x50, [0.0; 4]);
            store_vec(&mut e, state + 0xd0, [0.0, 2.0, 0.0, 0.0]);
        }
        let least = e
            .call(0x00d2_9bf0, &args![solver, info, acc, list, 3i32, OFFSET])
            .u32();
        assert_eq!(least, 5);
        for state in [states[0], states[1]] {
            assert_eq!(load_vec(&e, state + 0xd0), [1.0, 2.0, 3.0, 0.0]);
            assert_eq!(load_vec(&e, state + 0xe0), [0.1, 0.2, 0.3, 0.0]);
            assert_eq!(load_vec(&e, state + 0x60), [2.0, 0.0, 0.0, 9.0]);
        }
        // Type 4 steps with its own velocities.
        assert_eq!(load_vec(&e, states[2] + 0xd0), [0.0, 2.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, states[2] + 0x60), [0.0, 1.0, 0.0, 9.0]);
        assert!(calls(&log, SET_ROTATED_DIR).is_empty());
        assert_eq!(calls(&log, SET_ROTATION).len(), 3);
    }

    #[test]
    fn accumulator_types_0_3_and_6_rotate_their_velocities_00d29bf0() {
        let (mut e, log, solver, info, _gravity, list, states) = step_setup(&[3, 0, 6]);
        let acc = accumulators(&mut e, 3);
        let spin = [0.1, 0.2, 0.3, 0.4];
        for index in 0..3u32 {
            let at = acc + 0x80 * index;
            store_vec(&mut e, at + 0x10, [1.0, 2.0, 3.0, 0.0]);
            store_vec(&mut e, at + 0x20, spin);
            store_vec(&mut e, at + 0x40, [4.0, 0.0, 0.0, 0.0]);
            store_vec(&mut e, at + 0x50, [0.0; 4]);
        }
        e.call(0x00d2_9bf0, &args![solver, info, acc, list, 3i32, OFFSET]);
        let turns = calls(&log, SET_ROTATED_DIR);
        // Type 3: the step's angular vector; type 0: the momentum first; type 6: also the
        // final angular velocity.
        assert_eq!(turns.len(), 6);
        let frame = turns[0][0] - 0x70;
        assert_eq!(turns[0], vec![frame + 0x70, states[0] + 0x10, acc + 0x50]);
        assert_eq!(turns[1], vec![frame + 0xd0, states[1] + 0x10, frame + 0x80]);
        assert_eq!(turns[2], vec![frame + 0x70, states[1] + 0x10, acc + 0xd0]);
        assert_eq!(
            turns[5],
            vec![states[2] + 0xe0, states[2] + 0x10, frame + 0xe0]
        );
        // Identity columns: the angular velocity is the accumulator's.
        let rotated = load_vec(&e, states[0] + 0xe0);
        assert_eq!(rotated[..3], spin[..3]);
        assert_eq!(load_vec(&e, states[0] + 0x60), [2.0, 0.0, 0.0, 9.0]);
        // Type 6: the momentum through the transposed rows (fourth lane 0 from the shuffles)
        // times the inverse inertia.
        let momentum = ps_mul(refined_rcp([2.0; 4]), spin);
        let expected = ps_mul([momentum[0], momentum[1], momentum[2], 0.0], [2.0; 4]);
        assert_eq!(load_vec(&e, states[2] + 0xe0), expected);
        assert_eq!(load_vec(&e, states[2] + 0xd0), [1.0, 2.0, 3.0, 0.0]);
    }

    #[test]
    fn other_accumulator_types_only_limit_and_count_00d29bf0() {
        let (mut e, log, solver, info, _gravity, list, states) = step_setup(&[1, 5, 9]);
        let acc = accumulators(&mut e, 3);
        let radii = [2.0f32, 0.5, 0.5];
        for (state, radius) in states.iter().zip(radii) {
            e.mem.set_u8(state + 9, 3);
            e.mem.set_f32(state + 0xb0, radius);
            store_vec(&mut e, state + 0xd0, [3.0, 4.0, 0.0, 0.0]);
            store_vec(&mut e, state + 0xe0, [0.0, 0.25, 0.0, 0.0]);
        }
        store_vec(&mut e, states[1] + 0xd0, [30.0, 40.0, 0.0, 0.0]);
        store_vec(&mut e, states[2] + 0xe0, [0.0, f32::NAN, 0.0, 0.0]);
        let least = e
            .call(0x00d2_9bf0, &args![solver, info, acc, list, 3i32, OFFSET])
            .u32();
        // Nothing integrates.
        assert!(calls(&log, SET_ROTATION).is_empty());
        for state in &states {
            assert_eq!(load_vec(&e, state + 0x60), [0.0; 4]);
        }
        // The radius 2 counts as 1: energy 1 * 0.0625 + 25.
        assert_eq!(e.mem.f32(states[0] + 0xfc), 25.0625);
        // A fast motion is limited to 10 (energy 0.25 * 0.0625 + 100, nearly).
        let velocity = load_vec(&e, states[1] + 0xd0);
        close(velocity[0], 6.0);
        close(velocity[1], 8.0);
        close(e.mem.f32(states[1] + 0xfc), 100.015_625);
        // A NaN spin is reset (to the vector 7, 7, 7, 7).
        assert_eq!(load_vec(&e, states[2] + 0xe0), [7.0; 4]);
        assert_eq!(least, 6);
        assert_eq!(e.mem.u8(acc), 0xcd);
    }
}

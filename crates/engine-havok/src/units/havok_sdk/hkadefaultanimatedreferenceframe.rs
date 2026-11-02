//! `Havok SDK/hkadefaultanimatedreferenceframe.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use std::cmp::Ordering;

use super::hkaabbtree::{
    add_ps, load_vec, mul_ps, rcp_estimate, reserve_at_least, sub_ps, Vec4, FLOAT_ONE, ONES_VECTOR,
};

/// The vtable the constructor stores (`010e57a4`; slot 3, byte offset 0xc, is
/// `getReferenceFrame`).
const VTABLE: u32 = 0x010e_57a4;
/// Byte offset of `getReferenceFrame` in the vtable.
const REFERENCE_FRAME_SLOT: u32 = 0xc;
/// Four ones used by the key interpolation and the sampler (`010ec7f0`).
const ONES_INTERPOLATION: u32 = 0x010e_c7f0;
/// Four copies of -0.5 (`010c71e0`), the sign mask of the three vector lanes (`010c4c00`, used to
/// conjugate a quaternion), the sign mask of all four lanes (`010c44d0`) and the mask that keeps
/// three lanes (`010c4c10`).
const NEGATIVE_HALF_VECTOR: u32 = 0x010c_71e0;
const CONJUGATE_MASK: u32 = 0x010c_4c00;
const SIGN_MASK: u32 = 0x010c_44d0;
const THREE_LANE_MASK: u32 = 0x010c_4c10;
/// The all-zero vector the constructor passes to `project` as the vector (`01268700`).
const ZERO_VECTOR_ARGUMENT: u32 = 0x0126_8700;
/// The float 2^32 added when a signed count is read as an unsigned one (`01099378`), the first
/// lane of the vector at `010c7160` (zero in the exe), and the misalignment limit 0.01
/// (`01013ea4`).
const TWO_POW_32: u32 = 0x0109_9378;
const FIRST_LANE_BASE: u32 = 0x010c_7160;
const MISALIGNMENT_LIMIT: u32 = 0x0101_3ea4;
/// The three doubles `project` unwraps the angle with: the upper limit (`0101ff40`), the period
/// (`0101ff48`) and the lower limit (`0101ff58`).
const ANGLE_UPPER_LIMIT: u32 = 0x0101_ff40;
const ANGLE_PERIOD: u32 = 0x0101_ff48;
const ANGLE_LOWER_LIMIT: u32 = 0x0101_ff58;
/// CRT float to integer helper (`00ec62c0`; value in `ST0`, passed as a double; result in
/// `EAX`).
const FLOAT_TO_INT: u32 = 0x00ec_62c0;
/// `hkQuaternion::setAxisAngle` (Xbox PDB; `00cb2450`, thiscall on the quaternion, two words:
/// axis, angle), `hkQuaternion::decomposeRestAxis` (Xbox PDB; `00cb2ca0`, thiscall on the
/// quaternion, three words) and `hkQuaternion::setMul` (Xbox PDB; `00c66320`, thiscall on the
/// result, two words: the factors).
const SET_AXIS_ANGLE: u32 = 0x00cb_2450;
const DECOMPOSE_REST_AXIS: u32 = 0x00cb_2ca0;
const QUATERNION_MUL: u32 = 0x00c6_6320;
/// Unnamed function (`00c74ac0`, in the `bhkragdollcontroller` range; thiscall on the result,
/// two words: two quaternions) and the one-instruction thunk at `005611c0` (thiscall on a
/// quaternion, no words).
const QUATERNION_PAIR_OP: u32 = 0x00c7_4ac0;
const QUATERNION_THUNK: u32 = 0x0056_11c0;
/// Text output: the stream constructor (`00cb1600`, thiscall, two words: buffer, size), `operator<<`
/// of a string (`00cc01c0`, one word) and of a float (`00cc0320`, one word; Xbox PDB
/// `hkOstream::operator<<`), both returning the stream, and the stream destructor (`00cc00e0`).
const STREAM_CONSTRUCT: u32 = 0x00cb_1600;
const STREAM_WRITE_STRING: u32 = 0x00cc_01c0;
const STREAM_WRITE_FLOAT: u32 = 0x00cc_0320;
const STREAM_DESTRUCT: u32 = 0x00cc_00e0;
/// The pointer to the object whose virtual function at +0xc takes the message (`01268360`),
/// the number the exe passes first, the id it passes second and the source file name string
/// (`010ebdec`).
const MESSAGE_SINK: u32 = 0x0126_8360;
const MESSAGE_ID: u32 = 0xabba_9245;
const SOURCE_FILE: u32 = 0x010e_bdec;
/// The strings of the three messages: "Extracting turning motion on an animation works best
/// when the start frame is vertically aligned with the origin." (`010ebe28`), "Some misalignment
/// (" (`010ebdcc`), ", " (`01012630`), ") found." (`010ebde0`) and "For best results, try
/// moving the animation to the origin in the Create Animations filter." (`010ebd70`).
const MESSAGE_ALIGNMENT: u32 = 0x010e_be28;
const MESSAGE_PREFIX: u32 = 0x010e_bdcc;
const MESSAGE_SEPARATOR: u32 = 0x0101_2630;
const MESSAGE_SUFFIX: u32 = 0x010e_bde0;
const MESSAGE_ADVICE: u32 = 0x010e_bd70;
/// The sampler (`00e41bf0`) and `project` (`00e41220`) of this unit.
const SAMPLER: u32 = 0x00e4_1bf0;
const PROJECT: u32 = 0x00e4_1220;

/// The lane order `SHUFPS v, v, 0xd2` gives: (2, 0, 1, 3).
const ORDER_D2: [usize; 4] = [2, 0, 1, 3];
/// The lane order `SHUFPS v, v, 0xc9` gives: (1, 2, 0, 3).
const ORDER_C9: [usize; 4] = [1, 2, 0, 3];

/// Lane `lane` of `v` in all four lanes (`SHUFPS v, v, lane * 0x55`).
fn splat_lane(v: Vec4, lane: usize) -> Vec4 {
    [v[lane]; 4]
}

/// `SHUFPS v, v, control` with the lanes of `order`.
fn swizzle(v: Vec4, order: [usize; 4]) -> Vec4 {
    order.map(|lane| v[lane])
}

/// The sum the exe builds for a dot product of the first three lanes of `products`: lane 2 plus
/// (lane 1 plus lane 0).
fn dot_sum(products: Vec4) -> f32 {
    products[2] + (products[1] + products[0])
}

fn store_lanes(e: &mut Engine, at: u32, lanes: Vec4) {
    for (lane, value) in lanes.into_iter().enumerate() {
        e.mem
            .set_u32(at.wrapping_add(4 * lane as u32), value.to_bits());
    }
}

/// Copies 16 bytes (a `MOVAPS` load and store: the bits are kept as they are).
fn copy_16_bytes(e: &mut Engine, from: u32, to: u32) {
    for word in 0..4 {
        let value = e.mem.u32(from.wrapping_add(4 * word));
        e.mem.set_u32(to.wrapping_add(4 * word), value);
    }
}

fn load_mask(e: &Engine, at: u32) -> [u32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.u32(at + offset))
}

/// `XORPS` of a vector with a mask.
fn xor_bits(v: Vec4, mask: [u32; 4]) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() ^ mask[lane]))
}

/// `ANDPS` of a vector with a mask.
fn and_bits(v: Vec4, mask: [u32; 4]) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() & mask[lane]))
}

/// `first.partial_cmp(second)` is `wanted`; false when either is NaN (the x87 flag tests the exe
/// makes on a compare).
fn ordered(first: f64, second: f64, wanted: Ordering) -> bool {
    first.partial_cmp(&second) == Some(wanted)
}

/// The cross product term the exe builds from the lane orders (2, 0, 1, 3) and (1, 2, 0, 3):
/// `swizzle(v, d2) * swizzle(q, c9) - swizzle(v, c9) * swizzle(q, d2)`. Its first lane is
/// `q1 v2 - q2 v1`.
fn cross_forward(v: Vec4, q: Vec4) -> Vec4 {
    sub_ps(
        mul_ps(swizzle(v, ORDER_D2), swizzle(q, ORDER_C9)),
        mul_ps(swizzle(v, ORDER_C9), swizzle(q, ORDER_D2)),
    )
}

/// The same term with the two products the other way round (the other rotation direction).
fn cross_inverse(v: Vec4, q: Vec4) -> Vec4 {
    sub_ps(
        mul_ps(swizzle(v, ORDER_C9), swizzle(q, ORDER_D2)),
        mul_ps(swizzle(v, ORDER_D2), swizzle(q, ORDER_C9)),
    )
}

/// Half of a quaternion rotation of `v`, as the exe builds it from `cross` (one of the two
/// cross terms), `q` and `w`, the fourth lane of `q` in all lanes:
/// `cross * w + ((w * w + -0.5) * v + dot3(v, q) * q)`, the dot product summed lane 2 plus
/// (lane 1 plus lane 0).
fn rotation_half(e: &Engine, v: Vec4, q: Vec4, w: Vec4, cross: Vec4) -> Vec4 {
    let half = load_vec(e, NEGATIVE_HALF_VECTOR);
    let dot = dot_sum(mul_ps(v, q));
    let scaled = mul_ps(add_ps(mul_ps(w, w), half), v);
    let along = mul_ps([dot; 4], q);
    add_ps(mul_ps(cross, w), add_ps(scaled, along))
}

/// The reciprocal of a scale with one Newton step, last lane cleared: the scale's first three
/// lanes with the last lane of the ones vector at `010c7170` are inverted with `RCPPS`, then
/// `2r - r * s * r` is computed and the vector masked with the three-lane mask.
fn inverse_scale(e: &Engine, scale: Vec4) -> Vec4 {
    let ones = load_vec(e, ONES_VECTOR);
    let s = [scale[0], scale[1], scale[2], ones[3]];
    let r = s.map(rcp_estimate);
    let refined = sub_ps(add_ps(r, r), mul_ps(mul_ps(r, s), r));
    and_bits(refined, load_mask(e, THREE_LANE_MASK))
}

// Translated from 00e41080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDefaultAnimatedReferenceFrame::getDuration` (Xbox PDB; thiscall, no stack words):
/// returns the float at +0x30 (loaded onto the x87 stack, so a signalling NaN comes back
/// quiet).
pub fn hka_default_animated_reference_frame_get_duration(e: &mut Engine, this: Ptr) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 0x30))
}

// Translated from 00e41100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDefaultAnimatedReferenceFrame::getReferenceFrame` (Xbox PDB; thiscall, two stack words:
/// `time`, `out`). The object has the duration at +0x30, the array of 16-byte keys behind +0x34
/// with its length at +0x38 and the rest axis at +0x10. With `step` the duration over (key
/// count - 1) as a float: a time not below the duration (ordered) takes the last key and a time
/// below zero the first; any other time (a NaN too) interpolates between the keys at
/// `trunc(time / step)` (the index is moved down by one when it equals key count - 1) and the
/// next, `(1 - f) * k0 + k1 * f` per lane. The result is stored at `out`, then
/// `hkQuaternion::setAxisAngle` runs on `out + 0x10` with the axis and the fourth lane of the
/// result, and `out + 0x20` gets the float at `01012068` in all lanes.
pub fn hka_default_animated_reference_frame_get_reference_frame(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    out: Ptr,
) {
    let me = this.addr();
    let out = out.addr();
    let duration = x87_quiet(e.mem.f32(me + 0x30));
    let count = e.mem.u32(me + 0x38);
    let keys = e.mem.u32(me + 0x34);
    let last = count.wrapping_sub(1);
    // FIDIV by the signed count - 1.
    let step = (f64::from(duration) / f64::from(last as i32)) as f32;
    let time = x87_quiet(time);
    let value = if matches!(
        duration.partial_cmp(&time),
        Some(Ordering::Less | Ordering::Equal)
    ) {
        load_vec(e, keys.wrapping_add(count << 4).wrapping_sub(0x10))
    } else if ordered(0.0, f64::from(time), Ordering::Greater) {
        load_vec(e, keys)
    } else {
        let scaled = f64::from(time) / f64::from(step);
        let mut index = e.call(FLOAT_TO_INT, &args![scaled]).u32();
        if index == last {
            index = index.wrapping_sub(1);
        }
        let fraction = (scaled - f64::from(index as i32)) as f32;
        let base = keys.wrapping_add(index << 4);
        let first = load_vec(e, base);
        let second = load_vec(e, base.wrapping_add(0x10));
        let blend = [fraction; 4];
        let ones = load_vec(e, ONES_INTERPOLATION);
        add_ps(mul_ps(sub_ps(ones, blend), first), mul_ps(second, blend))
    };
    store_lanes(e, out, value);
    let angle = x87_quiet(value[3]);
    e.call(
        SET_AXIS_ANGLE,
        &args![out + 0x10, me + 0x10, angle.to_bits()],
    );
    let one = e.mem.f32(FLOAT_ONE);
    store_lanes(e, out + 0x20, [one; 4]);
}

// Translated from 00e41220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDefaultAnimatedReferenceFrame::project` (Xbox PDB; cdecl, four stack words: `options`,
/// `vector`, `previous`, `out`; the first is the options record the constructor reads, with
/// four flag bytes at +0x10 to +0x13 and the axes `a` at +0x20 and `b` at +0x30).
/// Projects `vector` on the three axes `a`, `a x b` (the cross term of the lane orders) and `b`
/// (each as `splat(dot3) * axis`) and sums the ones whose flag is set, in the order flag +0x11
/// (`b`), +0x10 (`a`), +0x12 (the cross axis), into `out`; the fourth lane of `out` is 0 unless
/// the flag at +0x13 is set. With it, `hkQuaternion::decomposeRestAxis` runs on the quaternion at
/// `vector + 0x10` with `a` and gives an angle, which is then moved by whole periods (the double
/// at `0101ff48`) until `angle - r`, with `r` the float at `previous + 0xc`, lies between the
/// limits at `0101ff58` and `0101ff40`, and is stored in the fourth lane of `out`. An infinite
/// angle (or one so large that subtracting the period changes nothing) never ends, as in the
/// exe.
pub fn hka_default_animated_reference_frame_project(
    e: &mut Engine,
    options: Ptr,
    vector: Ptr,
    previous: Ptr,
    out: Ptr,
) {
    let options = options.addr();
    let vector = vector.addr();
    let previous = previous.addr();
    let out = out.addr();
    let axis = load_vec(e, options + 0x20);
    let other = load_vec(e, options + 0x30);
    let v = load_vec(e, vector);
    let along_axis = mul_ps([dot_sum(mul_ps(v, axis)); 4], axis);
    let third = cross_forward(other, axis);
    let along_third = mul_ps([dot_sum(mul_ps(v, third)); 4], third);
    let along_other = mul_ps([dot_sum(mul_ps(v, other)); 4], other);
    store_lanes(e, out, [0.0; 4]);
    if e.mem.u8(options + 0x11) != 0 {
        store_lanes(e, out, add_ps([0.0; 4], along_other));
    }
    if e.mem.u8(options + 0x10) != 0 {
        let sum = add_ps(load_vec(e, out), along_axis);
        store_lanes(e, out, sum);
    }
    if e.mem.u8(options + 0x12) != 0 {
        let sum = add_ps(load_vec(e, out), along_third);
        store_lanes(e, out, sum);
    }
    if e.mem.u8(options + 0x13) == 0 {
        e.mem.set_f32(out + 0xc, 0.0);
        return;
    }
    // Frame: +0 the angle, +8 the rest vector (the exe's locals at its stack pointer + 8 and
    // + 0x10).
    let angle = e.with_stack(0x20, |e, frame| {
        let frame = frame.addr();
        e.call(
            DECOMPOSE_REST_AXIS,
            &args![vector + 0x10, options + 0x20, frame + 8, frame],
        );
        x87_quiet(e.mem.f32(frame))
    });
    let reference = x87_quiet(e.mem.f32(previous + 0xc));
    let upper = e.mem.f64(ANGLE_UPPER_LIMIT);
    let period = e.mem.f64(ANGLE_PERIOD);
    let lower = e.mem.f64(ANGLE_LOWER_LIMIT);
    let distance = |angle: f32| f64::from(angle) - f64::from(reference);
    let mut angle = angle;
    while ordered(distance(angle), upper, Ordering::Greater) {
        angle = (f64::from(angle) - period) as f32;
    }
    while ordered(distance(angle), lower, Ordering::Less) {
        angle = (f64::from(angle) + period) as f32;
    }
    e.mem.set_f32(out + 0xc, angle);
}

// Translated from 00e41bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Transform track sampler (cdecl, five stack words: `out`, `time`, `transforms`, `count`,
/// `duration`; no PDB name). The track is `count` transforms of 0x30 bytes (translation,
/// quaternion, scale). Position `x = (time / duration) * (count - 1)` (the count - 1 read as an
/// unsigned number) is rounded to a float and truncated to a 64-bit integer whose low 32 bits
/// are the index (0 for a NaN or an out of range value). An index above count - 2 (unsigned)
/// copies the last transform to `out`. Otherwise with `f` the fraction (the float of
/// `x - index`) the scale and the translation are blended as `(1 - f) * a + b * f` per lane and
/// the quaternion as `(1 - f) * q0 + w * q1`, with `w` the fraction, or the first lane of the
/// vector at `010c7160` minus the fraction when the dot product of the two quaternions (summed
/// as the SSE code does) is below zero; that quaternion is then passed to the thunk at
/// `005611c0`. The result is `out + 0` the translation, `out + 0x10` the quaternion and
/// `out + 0x20` the scale.
pub fn fn_00e41bf0(
    e: &mut Engine,
    out: Ptr,
    time: f32,
    transforms: Ptr,
    count: u32,
    duration: f32,
) {
    let out = out.addr();
    let transforms = transforms.addr();
    let ratio = f64::from(x87_quiet(time)) / f64::from(x87_quiet(duration));
    let last = count.wrapping_sub(1);
    let mut span = f64::from(last as i32);
    if (last as i32) < 0 {
        span += f64::from(e.mem.f32(TWO_POW_32));
    }
    let position = x87_quiet((ratio * span) as f32);
    // FISTP of a NaN or of a value outside the 64-bit range stores the "integer indefinite"
    // (0x8000000000000000), whose low dword is 0.
    let index = if position.is_nan() || position.abs() >= (1u64 << 63) as f32 {
        0
    } else {
        (position as i64) as u32
    };
    if index > count.wrapping_sub(2) {
        let source = last.wrapping_mul(0x30).wrapping_add(transforms);
        for word in 0..12 {
            let value = e.mem.u32(source.wrapping_add(4 * word));
            e.mem.set_u32(out.wrapping_add(4 * word), value);
        }
        return;
    }
    let mut index_value = f64::from(index as i32);
    if (index as i32) < 0 {
        index_value += f64::from(e.mem.f32(TWO_POW_32));
    }
    let fraction = (f64::from(position) - index_value) as f32;
    let base = transforms.wrapping_add(index.wrapping_mul(0x30));
    let ones = load_vec(e, ONES_INTERPOLATION);
    let blend = [fraction; 4];
    let remaining = sub_ps(ones, blend);
    let scale = add_ps(
        mul_ps(remaining, load_vec(e, base + 0x20)),
        mul_ps(load_vec(e, base + 0x50), blend),
    );
    let translation = add_ps(
        mul_ps(remaining, load_vec(e, base)),
        mul_ps(load_vec(e, base + 0x30), blend),
    );
    let first_quaternion = load_vec(e, base + 0x10);
    let second_quaternion = load_vec(e, base + 0x40);
    let remaining_weight = (1.0 - f64::from(x87_quiet(fraction))) as f32;
    let products = mul_ps(second_quaternion, first_quaternion);
    let pair_sums = add_ps(swizzle(products, [2, 3, 0, 1]), products);
    let dot = add_ps(swizzle(pair_sums, [1, 0, 3, 2]), pair_sums)[0];
    let weight = if ordered(0.0, f64::from(dot), Ordering::Greater) {
        e.mem.f32(FIRST_LANE_BASE) - fraction
    } else {
        fraction
    };
    let weight = x87_quiet(weight);
    let quaternion = add_ps(
        mul_ps([remaining_weight; 4], first_quaternion),
        mul_ps([weight; 4], second_quaternion),
    );
    // The exe's frame: +0x10 the quaternion the thunk works on, +0x30 the translation, +0x50
    // the scale.
    e.with_stack(0x60, |e, frame| {
        let frame = frame.addr();
        store_lanes(e, frame + 0x10, quaternion);
        store_lanes(e, frame + 0x30, translation);
        store_lanes(e, frame + 0x50, scale);
        e.call(QUATERNION_THUNK, &args![frame + 0x10]);
        copy_16_bytes(e, frame + 0x30, out);
        copy_16_bytes(e, frame + 0x10, out + 0x10);
        copy_16_bytes(e, frame + 0x50, out + 0x20);
    });
}

// Translated from 00e413d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDefaultAnimatedReferenceFrame::getDeltaReferenceFrame` (Xbox PDB; thiscall, four stack
/// words: `time_a`, `time_b`, `loops`, `out`). Calls the virtual function at +0xc
/// (`getReferenceFrame`) for frame A at `time_a` and frame B at `time_b` (each: translation,
/// quaternion, scale, 0x10 bytes apiece). With `loops` not zero it also gets the frame L at the
/// duration (the float at +0x30) and applies it |`loops`| times to B: for `loops` above zero
/// as `position = rot(qL, position) + pL`, `quaternion = qL * quaternion`, `scale = sL * scale`;
/// for `loops` below zero as `position = rot(c, position) - rot(c, pL)`, `quaternion =
/// c * quaternion`, `scale = scale * inverse(sL)` with `c` the conjugate of `qL` and `inverse`
/// the `RCPPS` plus one Newton step estimate (last lane cleared); `rot` is the doubled
/// `rotation_half`. The quaternion products are `hkQuaternion::setMul`. The result is the
/// translation `rot(c_A, position) - rot(c_A, pA)` with `c_A` the conjugate of A's quaternion, at
/// `out`, the quaternion `c_A * quaternion` (`setMul`) at `out + 0x10` and the float at
/// `01012068` in all lanes at `out + 0x20`.
///
/// The exe's frame (0x130 bytes) is modelled at its offsets: +0x10 position, +0x20 quaternion,
/// +0x30 scale (B, then the running transform), +0x40 and +0x50 the new position and
/// `setMul` result, +0x70/+0xb0/+0xc0 spills of lane orders of qL, +0x80/+0x90/+0xa0 the frame
/// L, +0xd0/+0xe0/+0xf0 the negative case's spills, +0x100 the frame A.
pub fn hka_default_animated_reference_frame_get_delta_reference_frame(
    e: &mut Engine,
    this: Ptr,
    time_a: f32,
    time_b: f32,
    loops: i32,
    out: Ptr,
) {
    let me = this.addr();
    let out = out.addr();
    let time_a = x87_quiet(time_a);
    let time_b = x87_quiet(time_b);
    e.with_stack(0x130, |e, frame| {
        let f = frame.addr();
        e.vcall(
            me,
            REFERENCE_FRAME_SLOT,
            &args![time_a.to_bits(), f + 0x100],
        );
        e.vcall(me, REFERENCE_FRAME_SLOT, &args![time_b.to_bits(), f + 0x10]);
        if loops != 0 {
            let duration = x87_quiet(e.mem.f32(me + 0x30));
            e.vcall(
                me,
                REFERENCE_FRAME_SLOT,
                &args![duration.to_bits(), f + 0x80],
            );
            let mut remaining = loops.unsigned_abs();
            // The exe's `JBE` to the end when |loops| is 0 never jumps: `loops` is not zero
            // here, and the negation of the lowest value is 0x80000000.
            if remaining != 0 {
                let quaternion = load_vec(e, f + 0x90);
                store_lanes(e, f + 0x70, swizzle(quaternion, ORDER_D2));
                store_lanes(e, f + 0xc0, splat_lane(quaternion, 3));
                store_lanes(e, f + 0xb0, swizzle(quaternion, ORDER_C9));
                loop {
                    let quaternion = load_vec(e, f + 0x90);
                    let w = load_vec(e, f + 0xc0);
                    let position = load_vec(e, f + 0x10);
                    let frame_position = load_vec(e, f + 0x80);
                    if loops < 0 {
                        let spilled_d2 = load_vec(e, f + 0x70);
                        let spilled_c9 = load_vec(e, f + 0xb0);
                        let cross = sub_ps(
                            mul_ps(swizzle(frame_position, ORDER_C9), spilled_d2),
                            mul_ps(swizzle(frame_position, ORDER_D2), spilled_c9),
                        );
                        let rotated = rotation_half(e, frame_position, quaternion, w, cross);
                        let inverse = inverse_scale(e, load_vec(e, f + 0xa0));
                        store_lanes(e, f + 0xf0, inverse);
                        let doubled = add_ps(rotated, rotated);
                        let conjugate = xor_bits(quaternion, load_mask(e, CONJUGATE_MASK));
                        let negated = xor_bits(doubled, load_mask(e, SIGN_MASK));
                        let conjugate_cross = cross_forward(position, conjugate);
                        let conjugate_w = splat_lane(conjugate, 3);
                        let rotated_position =
                            rotation_half(e, position, conjugate, conjugate_w, conjugate_cross);
                        let new_position =
                            add_ps(add_ps(rotated_position, rotated_position), negated);
                        store_lanes(e, f + 0xd0, negated);
                        store_lanes(e, f + 0xe0, conjugate);
                        store_lanes(e, f + 0x40, new_position);
                        e.call(QUATERNION_MUL, &args![f + 0x50, f + 0xe0, f + 0x20]);
                        let scale = mul_ps(load_vec(e, f + 0x30), load_vec(e, f + 0xf0));
                        store_lanes(e, f + 0x30, scale);
                    } else {
                        let spilled_d2 = load_vec(e, f + 0x70);
                        let spilled_c9 = load_vec(e, f + 0xb0);
                        let cross = sub_ps(
                            mul_ps(swizzle(position, ORDER_D2), spilled_c9),
                            mul_ps(swizzle(position, ORDER_C9), spilled_d2),
                        );
                        let rotated = rotation_half(e, position, quaternion, w, cross);
                        let new_position = add_ps(add_ps(rotated, rotated), frame_position);
                        store_lanes(e, f + 0x40, new_position);
                        e.call(QUATERNION_MUL, &args![f + 0x50, f + 0x90, f + 0x20]);
                        let scale = mul_ps(load_vec(e, f + 0xa0), load_vec(e, f + 0x30));
                        store_lanes(e, f + 0x30, scale);
                    }
                    copy_16_bytes(e, f + 0x50, f + 0x20);
                    copy_16_bytes(e, f + 0x40, f + 0x10);
                    remaining -= 1;
                    if remaining == 0 {
                        break;
                    }
                }
            }
        }
        let position = load_vec(e, f + 0x10);
        let a_position = load_vec(e, f + 0x100);
        let a_quaternion = load_vec(e, f + 0x110);
        let a_cross = cross_inverse(a_position, a_quaternion);
        let a_rotated = rotation_half(
            e,
            a_position,
            a_quaternion,
            splat_lane(a_quaternion, 3),
            a_cross,
        );
        let negated = xor_bits(add_ps(a_rotated, a_rotated), load_mask(e, SIGN_MASK));
        // Computed and stored by the exe, never read again.
        let unused_inverse = inverse_scale(e, load_vec(e, f + 0x120));
        store_lanes(e, f + 0x60, unused_inverse);
        let conjugate = xor_bits(a_quaternion, load_mask(e, CONJUGATE_MASK));
        let conjugate_cross = cross_forward(position, conjugate);
        let rotated_position = rotation_half(
            e,
            position,
            conjugate,
            splat_lane(conjugate, 3),
            conjugate_cross,
        );
        let result = add_ps(add_ps(rotated_position, rotated_position), negated);
        store_lanes(e, f + 0x40, negated);
        store_lanes(e, f + 0x50, conjugate);
        store_lanes(e, out, result);
        e.call(QUATERNION_MUL, &args![out + 0x10, f + 0x50, f + 0x20]);
        let one = e.mem.f32(FLOAT_ONE);
        store_lanes(e, out + 0x20, [one; 4]);
    });
}

// Translated from 00e41830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaDefaultAnimatedReferenceFrame::hkaDefaultAnimatedReferenceFrame` (Xbox PDB; thiscall,
/// one stack word: `options`, the record `project` reads, which also has the transform track
/// pointer at +0, the key count at +4 and +0xc (-1 there means use the one at +4), the duration
/// at +8, the axes at +0x20 and +0x30). Returns `this`.
///
/// Stores the vtable at `010e57a4`, 1 in the 16-bit word at +6, an empty array (data 0, size 0,
/// capacity word 0x80000000) at +0x34, the axes at +0x10 and +0x20 and the duration (quiet) at
/// +0x30. Runs `project` once on the zero vector (`01268700`) with a zero previous vector to get
/// the first sample's projection `v0`. With the flag byte at `options + 0x13` set it checks
/// the misalignment: `|dot3(b, v0)|` and `|dot3(cross, v0)|` (the same cross term as `project`
/// and the same summing order); when either is above the float at `01013ea4` (ordered) it sends
/// three messages through a text stream (a 0x200 byte buffer) and the object at `[01268360]`
/// (virtual function at +0xc: 1, 0xabba9245, the buffer, the source file name and the line
/// 0x58, 0x59, 0x5a): the alignment text, "Some misalignment (" |d| ", " |d2| ") found." and the
/// advice. Then reserves room for the key count (`reserve_at_least`, 0x10 bytes per element),
/// sets the array size to it and, for each index below it: samples the track at
/// `index * step` (`step` the duration over count - 1 as a float, the sampler `00e41bf0` with
/// the track, its own count at +4 and the duration), runs `00c74ac0` on the sample's quaternion
/// and the first transform's, runs `project` on the sample's translation minus the first
/// transform's (with the previous projection, the first being `v0`) and stores the projection
/// in the array.
///
/// The exe's 0x300 byte frame is modelled at its offsets (+0x10 loop byte offset, +0x2c index,
/// +0x30 the first projection and the stream, +0x44 `this`, +0x4c step, +0x50 previous
/// projection, +0x60 projection, +0x70/+0x80/+0x90 the first transform, +0xa0 the offset
/// vector, +0xb0 the `00c74ac0` result, +0xd0 the sample, +0x100 the stream buffer).
pub fn hka_default_animated_reference_frame_hka_default_animated_reference_frame(
    e: &mut Engine,
    this: Ptr,
    options: Ptr,
) -> Ptr {
    let me = this.addr();
    let options = options.addr();
    e.mem.set_u32(me, VTABLE);
    let array = me + 0x34;
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 4, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
    let mut count = e.mem.i32(options + 0xc);
    if count == -1 {
        count = e.mem.i32(options + 4);
    }
    copy_16_bytes(e, options + 0x20, me + 0x10);
    copy_16_bytes(e, options + 0x30, me + 0x20);
    let duration = x87_quiet(e.mem.f32(options + 8));
    e.mem.set_f32(me + 0x30, duration);
    let this_again = e.with_stack(0x300, |e, frame| {
        let f = frame.addr();
        e.mem.set_u32(f + 0x44, me);
        let transform = e.mem.u32(options);
        let step = (f64::from(duration) / f64::from(count.wrapping_sub(1))) as f32;
        copy_16_bytes(e, transform, f + 0x70);
        copy_16_bytes(e, transform + 0x10, f + 0x80);
        copy_16_bytes(e, transform + 0x20, f + 0x90);
        store_lanes(e, f + 0x50, [0.0; 4]);
        e.mem.set_f32(f + 0x4c, step);
        e.call(
            PROJECT,
            &args![options, ZERO_VECTOR_ARGUMENT, f + 0x50, f + 0x30],
        );
        copy_16_bytes(e, f + 0x30, f + 0x50);
        if e.mem.u8(options + 0x13) != 0 {
            let axis = load_vec(e, options + 0x20);
            let other = load_vec(e, options + 0x30);
            let first = load_vec(e, f + 0x30);
            let distance = x87_quiet(dot_sum(mul_ps(other, first)));
            let distance = distance.abs();
            e.mem.set_f32(f + 0x10, distance);
            let cross = sub_ps(
                mul_ps(swizzle(axis, ORDER_C9), swizzle(other, ORDER_D2)),
                mul_ps(swizzle(axis, ORDER_D2), swizzle(other, ORDER_C9)),
            );
            let side = x87_quiet(dot_sum(mul_ps(cross, first))).abs();
            e.mem.set_f32(f + 0x2c, side);
            let limit = f64::from(e.mem.f32(MISALIGNMENT_LIMIT));
            if ordered(limit, f64::from(distance), Ordering::Less)
                || ordered(limit, f64::from(side), Ordering::Less)
            {
                report_misalignment(e, f, distance, side);
            }
        }
        reserve_at_least(e, array, count, 0x10);
        e.mem.set_u32(array + 4, count as u32);
        e.mem.set_u32(f + 0x2c, 0);
        if count > 0 {
            let one = e.mem.f32(FLOAT_ONE);
            store_lanes(e, f + 0x30, [one; 4]);
            e.mem.set_u32(f + 0x10, 0);
            let mut index = 0i32;
            loop {
                let sample_duration = x87_quiet(e.mem.f32(options + 8));
                let transforms = e.mem.u32(options);
                let track_count = e.mem.u32(options + 4);
                let step = x87_quiet(e.mem.f32(f + 0x4c));
                let sample_time =
                    x87_quiet((f64::from(e.mem.i32(f + 0x2c)) * f64::from(step)) as f32);
                e.mem.set_f32(f + 0x48, sample_time);
                let sample_time = x87_quiet(e.mem.f32(f + 0x48));
                e.call(
                    SAMPLER,
                    &args![
                        f + 0xd0,
                        sample_time.to_bits(),
                        transforms,
                        track_count,
                        sample_duration.to_bits()
                    ],
                );
                e.call(QUATERNION_PAIR_OP, &args![f + 0xb0, f + 0xe0, f + 0x80]);
                let offset = sub_ps(load_vec(e, f + 0xd0), load_vec(e, f + 0x70));
                store_lanes(e, f + 0xa0, offset);
                copy_16_bytes(e, f + 0x30, f + 0xc0);
                e.call(PROJECT, &args![options, f + 0xa0, f + 0x50, f + 0x60]);
                let byte_offset = e.mem.u32(f + 0x10);
                let data = e.mem.u32(array);
                copy_16_bytes(e, f + 0x60, data.wrapping_add(byte_offset));
                index += 1;
                copy_16_bytes(e, f + 0x60, f + 0x50);
                e.mem.set_u32(f + 0x2c, index as u32);
                e.mem.set_u32(f + 0x10, byte_offset.wrapping_add(0x10));
                if index >= count {
                    break;
                }
            }
        }
        e.mem.u32(f + 0x44)
    });
    Ptr::new(this_again)
}

/// The three messages `hkaDefaultAnimatedReferenceFrame`'s constructor sends when the first
/// sample is misaligned: each builds a text stream at `f + 0x30` on the buffer at `f + 0x100`
/// (0x200 bytes), writes the text, sends the buffer to the object at `[01268360]` and destroys
/// the stream. `distance` and `side` are the two absolute dot products.
fn report_misalignment(e: &mut Engine, f: u32, distance: f32, side: f32) {
    let stream = f + 0x30;
    let buffer = f + 0x100;
    e.call(STREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
    e.call(STREAM_WRITE_STRING, &args![stream, MESSAGE_ALIGNMENT]);
    send_message(e, buffer, 0x58);
    e.call(STREAM_DESTRUCT, &args![stream]);

    e.call(STREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
    let next = e
        .call(STREAM_WRITE_STRING, &args![stream, MESSAGE_PREFIX])
        .u32();
    let next = e
        .call(STREAM_WRITE_FLOAT, &args![next, distance.to_bits()])
        .u32();
    let next = e
        .call(STREAM_WRITE_STRING, &args![next, MESSAGE_SEPARATOR])
        .u32();
    let next = e
        .call(STREAM_WRITE_FLOAT, &args![next, side.to_bits()])
        .u32();
    e.call(STREAM_WRITE_STRING, &args![next, MESSAGE_SUFFIX]);
    send_message(e, buffer, 0x59);
    e.call(STREAM_DESTRUCT, &args![stream]);

    e.call(STREAM_CONSTRUCT, &args![stream, buffer, 0x200u32]);
    e.call(STREAM_WRITE_STRING, &args![stream, MESSAGE_ADVICE]);
    send_message(e, buffer, 0x5a);
    e.call(STREAM_DESTRUCT, &args![stream]);
}

/// The virtual call at +0xc of the object at `[01268360]`: `(1, 0xabba9245, buffer, file name,
/// line)`.
fn send_message(e: &mut Engine, buffer: u32, line: u32) {
    let sink = e.mem.u32(MESSAGE_SINK);
    e.vcall(
        sink,
        0xc,
        &args![1u32, MESSAGE_ID, buffer, SOURCE_FILE, line],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_1080, hka_default_animated_reference_frame_get_duration(Ptr) -> f32),
        entry!(
            0x00e4_1100,
            hka_default_animated_reference_frame_get_reference_frame(Ptr, f32, Ptr)
        ),
        entry!(
            0x00e4_1220,
            hka_default_animated_reference_frame_project(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00e4_13d0,
            hka_default_animated_reference_frame_get_delta_reference_frame(Ptr, f32, f32, i32, Ptr)
        ),
        entry!(0x00e4_1830, hka_default_animated_reference_frame_hka_default_animated_reference_frame(Ptr, Ptr) -> Ptr),
        entry!(0x00e4_1bf0, fn_00e41bf0(Ptr, f32, Ptr, u32, f32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::hkaabbtree::MEMORY_ROUTER;
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    type Views = Rc<RefCell<Vec<(Vec4, Vec4)>>>;
    type Pairs = Rc<RefCell<Vec<(Vec4, Vec4)>>>;

    /// `hkArrayUtil::_reserve` (outside this file), a stand-in address for the message object's
    /// virtual function and the address of `getReferenceFrame` (replaced by a double where
    /// the delta function calls it through the vtable).
    const ARRAY_RESERVE: u32 = 0x00c9_08c0;
    const SINK_FUNCTION: u32 = 0x0070_0001;
    const REFERENCE_FRAME: u32 = 0x00e4_1100;

    fn pi() -> f64 {
        f64::from_bits(0x4009_21fb_6000_0000)
    }

    fn tau() -> f64 {
        f64::from_bits(0x4019_21fb_6000_0000)
    }

    /// An engine with the constants the functions read from the exe image.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        for lane in 0..4u32 {
            let at = 4 * lane;
            let three = if lane < 3 { u32::MAX } else { 0 };
            e.mem.set_f32(ONES_INTERPOLATION + at, 1.0);
            e.mem.set_f32(ONES_VECTOR + at, 1.0);
            e.mem.set_f32(NEGATIVE_HALF_VECTOR + at, -0.5);
            e.mem.set_u32(SIGN_MASK + at, 0x8000_0000);
            e.mem.set_u32(CONJUGATE_MASK + at, three & 0x8000_0000);
            e.mem.set_u32(THREE_LANE_MASK + at, three);
        }
        e.mem.set_f32(FLOAT_ONE, 1.0);
        e.mem.set_f32(TWO_POW_32, 4_294_967_296.0);
        e.mem.set_f32(MISALIGNMENT_LIMIT, 0.01);
        e.mem.set_f64(ANGLE_UPPER_LIMIT, pi());
        e.mem.set_f64(ANGLE_PERIOD, tau());
        e.mem.set_f64(ANGLE_LOWER_LIMIT, -pi());
        e
    }

    fn put(e: &mut Engine, at: u32, lanes: Vec4) {
        store_lanes(e, at, lanes);
    }

    fn get(e: &Engine, at: u32) -> Vec4 {
        load_vec(e, at)
    }

    fn near(first: Vec4, second: Vec4) -> bool {
        (0..4).all(|lane| (first[lane] - second[lane]).abs() < 1e-5)
    }

    /// Registers a double that logs its arguments and then runs `body`.
    fn logged(
        e: &mut Engine,
        calls: &Calls,
        address: u32,
        mut body: impl FnMut(&mut Engine, &[u32]) -> Ret + 'static,
    ) {
        let calls = Rc::clone(calls);
        e.register_double(address, move |e, a| {
            calls.borrow_mut().push((address, a.to_vec()));
            body(e, a)
        });
    }

    fn calls_to(calls: &Calls, address: u32) -> Vec<Vec<u32>> {
        calls
            .borrow()
            .iter()
            .filter(|(at, _)| *at == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The float to integer helper: `forced` as the result when given, else the truncation
    /// (0x80000000 for a NaN, as the hardware gives).
    fn install_float_to_int(e: &mut Engine, calls: &Calls, forced: Option<u32>) {
        logged(e, calls, FLOAT_TO_INT, move |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            let eax = match forced {
                Some(index) => index,
                None if value.is_nan() => 0x8000_0000,
                None => value as i32 as u32,
            };
            Ret {
                eax,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn get_duration_returns_the_quiet_copy_of_the_duration() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_f32(this + 0x30, 2.5);
        let duration = hka_default_animated_reference_frame_get_duration(&mut e, Ptr::new(this));
        assert_eq!(duration, 2.5);
        e.mem.set_u32(this + 0x30, 0x7f80_0001);
        let duration = hka_default_animated_reference_frame_get_duration(&mut e, Ptr::new(this));
        assert_eq!(duration.to_bits(), 0x7fc0_0001);
    }

    /// An object with five keys over 4 seconds (step 1): key `i` is `(i, 10 i, 100 i, i / 4)`.
    fn keyed_object(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x40);
        let keys = e.mem.alloc(5 * 16);
        for index in 0..5u32 {
            let i = index as f32;
            put(e, keys + 16 * index, [i, 10.0 * i, 100.0 * i, i / 4.0]);
        }
        e.mem.set_f32(this + 0x30, 4.0);
        e.mem.set_u32(this + 0x34, keys);
        e.mem.set_u32(this + 0x38, 5);
        this
    }

    fn reference_frame(e: &mut Engine, this: u32, time: f32) -> u32 {
        let out = e.mem.alloc(0x30);
        hka_default_animated_reference_frame_get_reference_frame(
            e,
            Ptr::new(this),
            time,
            Ptr::new(out),
        );
        out
    }

    #[test]
    fn reference_frame_outside_the_keys_takes_the_end_keys() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        logged(&mut e, &calls, SET_AXIS_ANGLE, |_, _| Ret::default());
        let this = keyed_object(&mut e);
        // The float to integer helper is not registered: these never reach the interpolation.
        let at_end = reference_frame(&mut e, this, 4.0);
        let past_end = reference_frame(&mut e, this, 9.0);
        let before = reference_frame(&mut e, this, -0.5);
        assert_eq!(get(&e, at_end), [4.0, 40.0, 400.0, 1.0]);
        assert_eq!(get(&e, past_end), [4.0, 40.0, 400.0, 1.0]);
        assert_eq!(get(&e, before), [0.0; 4]);
        for out in [at_end, past_end, before] {
            assert_eq!(get(&e, out + 0x20), [1.0; 4]);
        }
        assert_eq!(
            calls_to(&calls, SET_AXIS_ANGLE),
            vec![
                vec![at_end + 0x10, this + 0x10, 1.0f32.to_bits()],
                vec![past_end + 0x10, this + 0x10, 1.0f32.to_bits()],
                vec![before + 0x10, this + 0x10, 0.0f32.to_bits()],
            ]
        );
    }

    #[test]
    fn reference_frame_interpolates_between_the_keys() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        logged(&mut e, &calls, SET_AXIS_ANGLE, |_, _| Ret::default());
        install_float_to_int(&mut e, &calls, None);
        let this = keyed_object(&mut e);
        let out = reference_frame(&mut e, this, 2.5);
        assert_eq!(get(&e, out), [2.5, 25.0, 250.0, 0.625]);
        let convert = calls_to(&calls, FLOAT_TO_INT);
        assert_eq!(convert.len(), 1);
        assert_eq!(
            f64::from_bits(u64::from(convert[0][0]) | (u64::from(convert[0][1]) << 32)),
            2.5
        );
        assert_eq!(
            calls_to(&calls, SET_AXIS_ANGLE),
            vec![vec![out + 0x10, this + 0x10, 0.625f32.to_bits()]]
        );
        // Time 0 is not below zero: it interpolates (and gives the first key).
        let first = reference_frame(&mut e, this, 0.0);
        assert_eq!(get(&e, first), [0.0; 4]);
        assert_eq!(calls_to(&calls, FLOAT_TO_INT).len(), 2);
    }

    #[test]
    fn reference_frame_moves_the_index_down_from_the_last_key() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        logged(&mut e, &calls, SET_AXIS_ANGLE, |_, _| Ret::default());
        install_float_to_int(&mut e, &calls, Some(4));
        let this = keyed_object(&mut e);
        // The helper says 4 = key count - 1: the pair is keys 3 and 4 with the fraction 0.5.
        let out = reference_frame(&mut e, this, 3.5);
        assert_eq!(get(&e, out), [3.5, 35.0, 350.0, 0.875]);
    }

    #[test]
    fn reference_frame_nan_time_interpolates_to_nan() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        logged(&mut e, &calls, SET_AXIS_ANGLE, |_, _| Ret::default());
        install_float_to_int(&mut e, &calls, None);
        let this = keyed_object(&mut e);
        let out = reference_frame(&mut e, this, f32::NAN);
        assert!(get(&e, out).iter().all(|lane| lane.is_nan()));
        assert_eq!(calls_to(&calls, FLOAT_TO_INT).len(), 1);
    }

    fn options_record(e: &mut Engine, flags: [u8; 4]) -> u32 {
        let record = e.mem.alloc(0x40);
        for (index, flag) in flags.into_iter().enumerate() {
            e.mem.set_u8(record + 0x10 + index as u32, flag);
        }
        put(e, record + 0x20, [1.0, 0.0, 0.0, 0.0]);
        put(e, record + 0x30, [0.0, 1.0, 0.0, 0.0]);
        record
    }

    struct Projected {
        lanes: Vec4,
        vector: u32,
        options: u32,
    }

    fn run_project(e: &mut Engine, flags: [u8; 4], v: Vec4, reference: f32) -> Projected {
        let options = options_record(e, flags);
        let vector = e.mem.alloc(0x20);
        put(e, vector, v);
        let previous = e.mem.alloc(0x10);
        e.mem.set_f32(previous + 0xc, reference);
        let out = e.mem.alloc(0x10);
        put(e, out, [9.0; 4]);
        hka_default_animated_reference_frame_project(
            e,
            Ptr::new(options),
            Ptr::new(vector),
            Ptr::new(previous),
            Ptr::new(out),
        );
        Projected {
            lanes: get(e, out),
            vector,
            options,
        }
    }

    #[test]
    fn project_sums_the_flagged_axes() {
        // The axes are x and y, so the third axis (their cross term) is z.
        let v = [2.0, 3.0, 4.0, 5.0];
        let cases = [
            ([1, 1, 1, 0], [2.0, 3.0, 4.0, 0.0]),
            ([0, 1, 0, 0], [0.0, 3.0, 0.0, 0.0]),
            ([1, 0, 0, 0], [2.0, 0.0, 0.0, 0.0]),
            ([0, 0, 1, 0], [0.0, 0.0, 4.0, 0.0]),
            ([0, 0, 0, 0], [0.0; 4]),
        ];
        for (flags, expected) in cases {
            // The decomposition is not registered: it must not run without the fourth flag.
            let mut e = engine();
            let result = run_project(&mut e, flags, v, 0.0);
            assert_eq!(result.lanes, expected, "flags {flags:?}");
        }
    }

    #[test]
    fn project_unwraps_the_angle_around_the_previous_one() {
        let cases = [
            // (previous angle, decomposed angle, expected)
            (0.5f32, 1.0f32, 1.0f32),
            (0.5, 10.0, (10.0 - 2.0 * tau()) as f32),
            (0.0, -10.0, (-10.0 + 2.0 * tau()) as f32),
            (0.0, -3.0, -3.0),
        ];
        for (reference, angle, expected) in cases {
            let mut e = engine();
            let calls: Calls = Rc::default();
            logged(&mut e, &calls, DECOMPOSE_REST_AXIS, move |e, a| {
                e.mem.set_f32(a[3], angle);
                Ret::default()
            });
            let result = run_project(&mut e, [0, 0, 0, 1], [1.0, 2.0, 3.0, 0.0], reference);
            assert_eq!(result.lanes[..3], [0.0; 3]);
            assert!(
                (result.lanes[3] - expected).abs() < 1e-5,
                "angle {angle} after {reference}: {}",
                result.lanes[3]
            );
            assert!((f64::from(result.lanes[3]) - f64::from(reference)).abs() <= pi() + 1e-6);
            let log = calls_to(&calls, DECOMPOSE_REST_AXIS);
            assert_eq!(log.len(), 1);
            assert_eq!(
                log[0][..3],
                [result.vector + 0x10, result.options + 0x20, log[0][3] + 8]
            );
        }
    }

    #[test]
    fn project_keeps_a_nan_angle() {
        let mut e = engine();
        let calls: Calls = Rc::default();
        logged(&mut e, &calls, DECOMPOSE_REST_AXIS, |e, a| {
            e.mem.set_u32(a[3], 0x7f80_0001);
            Ret::default()
        });
        let result = run_project(&mut e, [0, 0, 0, 1], [1.0, 2.0, 3.0, 0.0], 0.0);
        assert_eq!(result.lanes[3].to_bits(), 0x7fc0_0001);
    }

    /// Three transforms (translation, quaternion, scale; 0x30 bytes each): the quaternions of
    /// the second and third are opposite.
    fn track_of_three(e: &mut Engine) -> u32 {
        let base = e.mem.alloc(3 * 0x30);
        let entries: [[Vec4; 3]; 3] = [
            [[0.0; 4], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0, 0.0]],
            [
                [2.0, 4.0, 6.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [3.0, 3.0, 3.0, 0.0],
            ],
            [
                [4.0, 8.0, 12.0, 0.0],
                [0.0, 0.0, -1.0, 0.0],
                [5.0, 5.0, 5.0, 0.0],
            ],
        ];
        for (index, entry) in entries.iter().enumerate() {
            for (part, lanes) in entry.iter().enumerate() {
                put(e, base + 0x30 * index as u32 + 0x10 * part as u32, *lanes);
            }
        }
        base
    }

    /// Samples the track of three over 2 seconds; the thunk doubles every lane of the
    /// quaternion it is given. Returns (translation, quaternion, scale) and the thunk calls.
    fn sample(time: f32, first_lane: f32) -> ([Vec4; 3], usize) {
        let mut e = engine();
        let calls: Calls = Rc::default();
        logged(&mut e, &calls, QUATERNION_THUNK, |e, a| {
            let doubled = get(e, a[0]).map(|lane| lane * 2.0);
            put(e, a[0], doubled);
            Ret::default()
        });
        e.mem.set_f32(FIRST_LANE_BASE, first_lane);
        let track = track_of_three(&mut e);
        let out = e.mem.alloc(0x30);
        fn_00e41bf0(&mut e, Ptr::new(out), time, Ptr::new(track), 3, 2.0);
        let result = [get(&e, out), get(&e, out + 0x10), get(&e, out + 0x20)];
        (result, calls_to(&calls, QUATERNION_THUNK).len())
    }

    #[test]
    fn sampler_blends_with_the_fraction() {
        let ([translation, quaternion, scale], thunk_calls) = sample(0.5, 0.0);
        assert_eq!(translation, [1.0, 2.0, 3.0, 0.0]);
        assert_eq!(scale, [2.0, 2.0, 2.0, 0.0]);
        // 0.5 * (0, 0, 0, 1) + 0.5 * (0, 0, 1, 0), doubled by the thunk.
        assert_eq!(quaternion, [0.0, 0.0, 1.0, 1.0]);
        assert_eq!(thunk_calls, 1);
    }

    #[test]
    fn sampler_uses_the_other_weight_for_opposite_quaternions() {
        // Transforms 1 and 2 have opposite quaternions (dot product -1): the weight is the
        // first lane of the constant vector minus the fraction.
        let ([translation, quaternion, scale], thunk_calls) = sample(1.5, 0.25);
        assert_eq!(translation, [3.0, 6.0, 9.0, 0.0]);
        assert_eq!(scale, [4.0, 4.0, 4.0, 0.0]);
        // 0.5 * (0, 0, 1, 0) + (0.25 - 0.5) * (0, 0, -1, 0) = (0, 0, 0.75, 0), doubled.
        assert_eq!(quaternion, [0.0, 0.0, 1.5, 0.0]);
        assert_eq!(thunk_calls, 1);
    }

    #[test]
    fn sampler_interpolates_at_the_last_pair_with_a_zero_fraction() {
        // Position 1.0 is count - 2: still interpolated (not copied).
        let ([translation, quaternion, scale], thunk_calls) = sample(1.0, 0.0);
        assert_eq!(translation, [2.0, 4.0, 6.0, 0.0]);
        assert_eq!(scale, [3.0, 3.0, 3.0, 0.0]);
        assert_eq!(quaternion, [0.0, 0.0, 2.0, 0.0]);
        assert_eq!(thunk_calls, 1);
    }

    #[test]
    fn sampler_copies_the_last_transform_past_the_end_and_for_negative_times() {
        // A negative time gives a huge unsigned index, so it also takes the last transform.
        for time in [2.0, 3.0, -1.0] {
            let ([translation, quaternion, scale], thunk_calls) = sample(time, 0.0);
            assert_eq!(translation, [4.0, 8.0, 12.0, 0.0], "time {time}");
            assert_eq!(quaternion, [0.0, 0.0, -1.0, 0.0]);
            assert_eq!(scale, [5.0, 5.0, 5.0, 0.0]);
            assert_eq!(thunk_calls, 0);
        }
    }

    #[test]
    fn sampler_nan_time_uses_index_zero_with_a_nan_fraction() {
        let ([translation, _, _], thunk_calls) = sample(f32::NAN, 0.0);
        assert!(translation[0].is_nan());
        assert_eq!(thunk_calls, 1);
    }

    /// `a * b` of two quaternions (x, y, z, w).
    fn quaternion_product(a: Vec4, b: Vec4) -> Vec4 {
        [
            a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1],
            a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0],
            a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3],
            a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
        ]
    }

    #[derive(Clone)]
    struct MulCall {
        this: u32,
        a: u32,
        b: u32,
        a_vec: Vec4,
        b_vec: Vec4,
        a_next: Vec4,
        b_next: Vec4,
    }

    type MulLog = Rc<RefCell<Vec<MulCall>>>;

    /// `setMul` as a double: logs the factors (and the vectors 0x10 bytes behind them, which in
    /// the exe's frame are scales) and stores the product at `this`.
    fn install_mul(e: &mut Engine) -> MulLog {
        let log: MulLog = Rc::default();
        let inner = Rc::clone(&log);
        e.register_double(QUATERNION_MUL, move |e, a| {
            let (this, left, right) = (a[0], a[1], a[2]);
            let call = MulCall {
                this,
                a: left,
                b: right,
                a_vec: get(e, left),
                b_vec: get(e, right),
                a_next: get(e, left + 0x10),
                b_next: get(e, right + 0x10),
            };
            let product = quaternion_product(call.a_vec, call.b_vec);
            inner.borrow_mut().push(call);
            put(e, this, product);
            Ret::default()
        });
        log
    }

    /// An object whose duration is 4 and whose `getReferenceFrame` (a double) gives
    /// translation `(t, 2t, 3t, 0)`, no rotation and scale 1 for time `t`, except at the
    /// duration where it gives translation `(10, 4, 7, 0)`, a half turn about z and scale 2.
    fn framed_object(e: &mut Engine, calls: &Calls) -> u32 {
        let this = e.mem.alloc(0x40);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(this, vtable);
        e.mem
            .set_u32(vtable + REFERENCE_FRAME_SLOT, REFERENCE_FRAME);
        e.mem.set_f32(this + 0x30, 4.0);
        logged(e, calls, REFERENCE_FRAME, |e, a| {
            let time = f32::from_bits(a[1]);
            let frame = if time == 4.0 {
                [
                    [10.0, 4.0, 7.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [2.0, 2.0, 2.0, 0.0],
                ]
            } else {
                [
                    [time, 2.0 * time, 3.0 * time, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                    [1.0, 1.0, 1.0, 0.0],
                ]
            };
            for (part, lanes) in frame.into_iter().enumerate() {
                put(e, a[2] + 0x10 * part as u32, lanes);
            }
            Ret::default()
        });
        this
    }

    struct DeltaRun {
        position: Vec4,
        quaternion: Vec4,
        scale: Vec4,
        out: u32,
        frames: Vec<Vec<u32>>,
        muls: Vec<MulCall>,
    }

    fn run_delta(loops: i32) -> DeltaRun {
        let mut e = engine();
        let calls: Calls = Rc::default();
        let this = framed_object(&mut e, &calls);
        let muls = install_mul(&mut e);
        let out = e.mem.alloc(0x30);
        hka_default_animated_reference_frame_get_delta_reference_frame(
            &mut e,
            Ptr::new(this),
            0.5,
            1.0,
            loops,
            Ptr::new(out),
        );
        let muls = muls.borrow().clone();
        // Each frame call keeps [time bits, frame address].
        let frames = calls_to(&calls, REFERENCE_FRAME)
            .into_iter()
            .map(|args| vec![args[1], args[2]])
            .collect();
        DeltaRun {
            position: get(&e, out),
            quaternion: get(&e, out + 0x10),
            scale: get(&e, out + 0x20),
            out,
            frames,
            muls,
        }
    }

    #[test]
    fn delta_without_loops_subtracts_the_two_frames() {
        let run = run_delta(0);
        // Frame A (time 0.5) is read at the exe's +0x100, frame B (time 1) at +0x10.
        assert_eq!(run.frames.len(), 2);
        assert_eq!(run.frames[0][0], 0.5f32.to_bits());
        assert_eq!(run.frames[1][0], 1.0f32.to_bits());
        assert_eq!(run.frames[0][1] - run.frames[1][1], 0xf0);
        assert_eq!(run.position, [0.5, 1.0, 1.5, 0.0]);
        assert_eq!(run.quaternion, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(run.scale, [1.0; 4]);
        assert_eq!(run.muls.len(), 1);
        let product = &run.muls[0];
        assert_eq!(product.this, run.out + 0x10);
        assert_eq!(product.a - product.b, 0x30);
        assert_eq!(product.a_vec, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(product.b_vec, [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn delta_applies_the_loop_frame_forward_for_positive_loops() {
        let run = run_delta(2);
        // The loop frame is asked for at the duration, into the exe's +0x80.
        assert_eq!(run.frames.len(), 3);
        assert_eq!(run.frames[2][0], 4.0f32.to_bits());
        assert_eq!(run.frames[2][1] - run.frames[1][1], 0x70);
        // Position (1, 2, 3): rotated half a turn about z and moved by (10, 4, 7) twice.
        assert_eq!(run.position, [0.5, 1.0, 15.5, 0.0]);
        assert_eq!(run.quaternion, [0.0, 0.0, 0.0, -1.0]);
        assert_eq!(run.scale, [1.0; 4]);
        assert_eq!(run.muls.len(), 3);
        let (first, second, last) = (&run.muls[0], &run.muls[1], &run.muls[2]);
        for step in [first, second] {
            assert_eq!(step.a, step.this + 0x40);
            assert_eq!(step.b + 0x30, step.this);
            assert_eq!(step.a_vec, [0.0, 0.0, 1.0, 0.0]);
        }
        assert_eq!(first.b_vec, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(second.b_vec, [0.0, 0.0, 1.0, 0.0]);
        // The scale of the loop frame and of the running transform.
        assert_eq!(first.a_next, [2.0, 2.0, 2.0, 0.0]);
        assert_eq!(first.b_next, [1.0, 1.0, 1.0, 0.0]);
        assert_eq!(second.b_next, [2.0, 2.0, 2.0, 0.0]);
        assert_eq!(last.this, run.out + 0x10);
        assert_eq!(last.a - last.b, 0x30);
        assert_eq!(last.a_vec, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(last.b_vec, [0.0, 0.0, 0.0, -1.0]);
    }

    #[test]
    fn delta_applies_the_inverse_loop_frame_for_negative_loops() {
        let run = run_delta(-2);
        assert_eq!(run.frames.len(), 3);
        assert_eq!(run.position, [0.5, 1.0, -12.5, 0.0]);
        assert_eq!(run.quaternion, [0.0, 0.0, 0.0, -1.0]);
        assert_eq!(run.muls.len(), 3);
        let (first, second, last) = (&run.muls[0], &run.muls[1], &run.muls[2]);
        for step in [first, second] {
            // The conjugate of the loop quaternion is built at the exe's +0xe0.
            assert_eq!(step.a, step.this + 0x90);
            assert_eq!(step.b + 0x30, step.this);
            assert_eq!(step.a_vec, [0.0, 0.0, -1.0, 0.0]);
        }
        assert_eq!(first.b_vec, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(second.b_vec, [0.0, 0.0, -1.0, 0.0]);
        // The inverse scale (about 1 / 2, last lane cleared) sits behind the conjugate; the
        // running scale is multiplied by it after each step.
        assert!(near(first.a_next, [0.5, 0.5, 0.5, 0.0]));
        assert_eq!(first.b_next, [1.0, 1.0, 1.0, 0.0]);
        assert!(near(second.b_next, [0.5, 0.5, 0.5, 0.0]));
        assert_eq!(last.a_vec, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(last.b_vec, [0.0, 0.0, 0.0, -1.0]);
    }

    #[test]
    fn delta_single_loops_run_one_iteration() {
        let run = run_delta(-1);
        assert_eq!(run.muls.len(), 2);
        // (9, 2, -4) after the step, minus frame A's translation.
        assert_eq!(run.position, [8.5, 1.0, -5.5, 0.0]);
        let run = run_delta(1);
        assert_eq!(run.muls.len(), 2);
        // (-1, -2, 3) + (10, 4, 7) = (9, 2, 10), minus frame A's translation.
        assert_eq!(run.position, [8.5, 1.0, 8.5, 0.0]);
    }

    struct Fixture {
        e: Engine,
        calls: Calls,
        views: Views,
        pairs: Pairs,
        options: u32,
        transforms: u32,
        this: u32,
        alloc: u32,
    }

    /// A constructor setup: key count words `count_c` (+0xc) and `count_4` (+4), the
    /// misalignment flag byte and the results `project` (a double) gives in order. The first
    /// transform is translation (1, 2, 3), quaternion (.5, .5, .5, .5), the duration is 3 and
    /// the axes are x and y. The sampler double gives translation (time, 0, 0), no rotation.
    fn fixture(count_c: u32, count_4: u32, flag: u8, results: &[Vec4]) -> Fixture {
        let mut e = engine();
        let calls: Calls = Rc::default();
        let alloc = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, alloc);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        logged(&mut e, &calls, ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let queue = Rc::new(RefCell::new(results.to_vec()));
        let views: Views = Rc::default();
        let seen = Rc::clone(&views);
        logged(&mut e, &calls, PROJECT, move |e, a| {
            seen.borrow_mut().push((get(e, a[1]), get(e, a[2])));
            let result = if queue.borrow().is_empty() {
                [0.0; 4]
            } else {
                queue.borrow_mut().remove(0)
            };
            put(e, a[3], result);
            Ret::default()
        });
        logged(&mut e, &calls, SAMPLER, |e, a| {
            put(e, a[0], [f32::from_bits(a[1]), 0.0, 0.0, 0.0]);
            put(e, a[0] + 0x10, [0.0, 0.0, 0.0, 1.0]);
            put(e, a[0] + 0x20, [1.0; 4]);
            Ret::default()
        });
        let pairs: Pairs = Rc::default();
        let seen_pairs = Rc::clone(&pairs);
        logged(&mut e, &calls, QUATERNION_PAIR_OP, move |e, a| {
            seen_pairs.borrow_mut().push((get(e, a[1]), get(e, a[2])));
            Ret::default()
        });
        for address in [STREAM_CONSTRUCT, STREAM_DESTRUCT] {
            logged(&mut e, &calls, address, |_, _| Ret::default());
        }
        for address in [STREAM_WRITE_STRING, STREAM_WRITE_FLOAT] {
            // The stream functions return the stream; the double returns a distinct value so
            // the chaining shows.
            logged(&mut e, &calls, address, |_, a| Ret {
                eax: a[0] + 1,
                ..Ret::default()
            });
        }
        let sink = e.mem.alloc(0x10);
        let sink_table = e.mem.alloc(0x20);
        e.mem.set_u32(sink, sink_table);
        e.mem.set_u32(sink_table + 0xc, SINK_FUNCTION);
        e.mem.set_u32(MESSAGE_SINK, sink);
        logged(&mut e, &calls, SINK_FUNCTION, |_, _| Ret::default());

        let transforms = e.mem.alloc(0x30);
        put(&mut e, transforms, [1.0, 2.0, 3.0, 0.0]);
        put(&mut e, transforms + 0x10, [0.5; 4]);
        put(&mut e, transforms + 0x20, [4.0, 5.0, 6.0, 0.0]);
        let options = e.mem.alloc(0x40);
        e.mem.set_u32(options, transforms);
        e.mem.set_u32(options + 4, count_4);
        e.mem.set_f32(options + 8, 3.0);
        e.mem.set_u32(options + 0xc, count_c);
        e.mem.set_u8(options + 0x13, flag);
        put(&mut e, options + 0x20, [1.0, 0.0, 0.0, 0.0]);
        put(&mut e, options + 0x30, [0.0, 1.0, 0.0, 0.0]);
        let this = e.mem.alloc(0x40);
        Fixture {
            e,
            calls,
            views,
            pairs,
            options,
            transforms,
            this,
            alloc,
        }
    }

    fn construct(f: &mut Fixture) -> Ptr {
        hka_default_animated_reference_frame_hka_default_animated_reference_frame(
            &mut f.e,
            Ptr::new(f.this),
            Ptr::new(f.options),
        )
    }

    fn results(count: usize) -> Vec<Vec4> {
        (0..count)
            .map(|n| [n as f32 + 1.0, 0.0, 0.0, 0.0])
            .collect()
    }

    #[test]
    fn constructor_fills_the_object_and_projects_every_sample() {
        let mut f = fixture(4, 4, 0, &results(5));
        let returned = construct(&mut f);
        let this = f.this;
        assert_eq!(returned.addr(), this);
        assert_eq!(f.e.mem.u32(this), VTABLE);
        assert_eq!(f.e.mem.u16(this + 6), 1);
        assert_eq!(get(&f.e, this + 0x10), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(get(&f.e, this + 0x20), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(f.e.mem.f32(this + 0x30), 3.0);
        // The array: room for 4 (the exe's `_reserve` double), 4 elements, each the
        // projection of its sample.
        assert_eq!(
            calls_to(&f.calls, ARRAY_RESERVE),
            vec![vec![f.alloc, this + 0x34, 4, 0x10]]
        );
        assert_eq!(f.e.mem.u32(this + 0x34 + 4), 4);
        assert_eq!(f.e.mem.u32(this + 0x34 + 8), 4);
        let data = f.e.mem.u32(this + 0x34);
        for index in 0..4u32 {
            assert_eq!(get(&f.e, data + 16 * index)[0], index as f32 + 2.0);
        }
        // `project`: first on the zero vector with a zero previous vector, then once per
        // sample with the sample's translation minus the first transform's and the previous
        // projection.
        let projects = calls_to(&f.calls, PROJECT);
        assert_eq!(projects.len(), 5);
        assert_eq!(projects[0][..2], [f.options, ZERO_VECTOR_ARGUMENT]);
        assert_eq!(projects[0][2] - projects[0][3], 0x20);
        let views = f.views.borrow();
        assert_eq!(views[0].1, [0.0; 4]);
        for index in 0..4usize {
            let call = &projects[index + 1];
            assert_eq!(call[0], f.options);
            assert_eq!(call[1] - call[2], 0x50);
            assert_eq!(call[3] - call[2], 0x10);
            assert_eq!(views[index + 1].0, [index as f32 - 1.0, -2.0, -3.0, 0.0]);
            assert_eq!(views[index + 1].1, [index as f32 + 1.0, 0.0, 0.0, 0.0]);
        }
        // The sampler: time = index * step (step 3 / 3 = 1), the track, its own count, the
        // duration.
        let samples = calls_to(&f.calls, SAMPLER);
        assert_eq!(samples.len(), 4);
        for (index, call) in samples.iter().enumerate() {
            assert_eq!(
                call[1..],
                [(index as f32).to_bits(), f.transforms, 4, 3.0f32.to_bits()]
            );
        }
        // `00c74ac0` runs on the sample's quaternion and the first transform's.
        let pairs = f.pairs.borrow();
        assert_eq!(pairs.len(), 4);
        assert!(pairs
            .iter()
            .all(|pair| *pair == ([0.0, 0.0, 0.0, 1.0], [0.5; 4])));
        let pair_calls = calls_to(&f.calls, QUATERNION_PAIR_OP);
        assert_eq!(pair_calls[0][1] - pair_calls[0][0], 0x30);
        assert_eq!(pair_calls[0][0] - pair_calls[0][2], 0x30);
        // No message was sent.
        assert!(calls_to(&f.calls, STREAM_CONSTRUCT).is_empty());
    }

    #[test]
    fn constructor_takes_the_count_at_plus_4_when_the_other_is_minus_one() {
        let mut f = fixture(u32::MAX, 2, 0, &results(3));
        construct(&mut f);
        assert_eq!(f.e.mem.u32(f.this + 0x34 + 4), 2);
        let samples = calls_to(&f.calls, SAMPLER);
        assert_eq!(samples.len(), 2);
        // Step: 3 / (2 - 1).
        assert_eq!(samples[1][1], 3.0f32.to_bits());
        assert_eq!(samples[1][3], 2);
        assert_eq!(
            calls_to(&f.calls, ARRAY_RESERVE),
            vec![vec![f.alloc, f.this + 0x34, 2, 0x10]]
        );
    }

    #[test]
    fn constructor_without_samples_only_projects_once() {
        for count in [0u32, (-2i32) as u32] {
            let mut f = fixture(count, 7, 0, &results(1));
            let returned = construct(&mut f);
            assert_eq!(returned.addr(), f.this);
            assert_eq!(f.e.mem.u32(f.this + 0x34 + 4), count);
            assert_eq!(calls_to(&f.calls, PROJECT).len(), 1);
            assert!(calls_to(&f.calls, SAMPLER).is_empty());
            assert!(calls_to(&f.calls, ARRAY_RESERVE).is_empty());
        }
    }

    #[test]
    fn constructor_reports_a_misaligned_start() {
        // The first projection (0, -0.5, -0.25): |b . v| = 0.5, |(a x b) . v| = 0.25.
        let mut f = fixture(0, 0, 1, &[[0.0, -0.5, -0.25, 0.0]]);
        construct(&mut f);
        let addresses = [
            STREAM_CONSTRUCT,
            STREAM_WRITE_STRING,
            STREAM_WRITE_FLOAT,
            STREAM_DESTRUCT,
            SINK_FUNCTION,
        ];
        let log: Vec<(u32, Vec<u32>)> = f
            .calls
            .borrow()
            .iter()
            .filter(|(at, _)| addresses.contains(at))
            .cloned()
            .collect();
        let sequence: Vec<u32> = log.iter().map(|(at, _)| *at).collect();
        assert_eq!(
            sequence,
            vec![
                STREAM_CONSTRUCT,
                STREAM_WRITE_STRING,
                SINK_FUNCTION,
                STREAM_DESTRUCT,
                STREAM_CONSTRUCT,
                STREAM_WRITE_STRING,
                STREAM_WRITE_FLOAT,
                STREAM_WRITE_STRING,
                STREAM_WRITE_FLOAT,
                STREAM_WRITE_STRING,
                SINK_FUNCTION,
                STREAM_DESTRUCT,
                STREAM_CONSTRUCT,
                STREAM_WRITE_STRING,
                SINK_FUNCTION,
                STREAM_DESTRUCT,
            ]
        );
        let stream = log[0].1[0];
        let buffer = log[0].1[1];
        assert_eq!(buffer - stream, 0xd0);
        for (at, args) in &log {
            match *at {
                STREAM_CONSTRUCT => assert_eq!(args[..], [stream, buffer, 0x200]),
                STREAM_DESTRUCT => assert_eq!(args[..], [stream]),
                _ => {}
            }
        }
        // The messages: the stream the first string is written to, and for the second
        // message the chain of returned streams (each call is made on the previous result).
        assert_eq!(log[1].1, vec![stream, MESSAGE_ALIGNMENT]);
        assert_eq!(log[5].1, vec![stream, MESSAGE_PREFIX]);
        assert_eq!(log[6].1, vec![stream + 1, 0.5f32.to_bits()]);
        assert_eq!(log[7].1, vec![stream + 2, MESSAGE_SEPARATOR]);
        assert_eq!(log[8].1, vec![stream + 3, 0.25f32.to_bits()]);
        assert_eq!(log[9].1, vec![stream + 4, MESSAGE_SUFFIX]);
        assert_eq!(log[13].1, vec![stream, MESSAGE_ADVICE]);
        let sink = f.e.mem.u32(MESSAGE_SINK);
        for (position, line) in [(2, 0x58), (10, 0x59), (14, 0x5a)] {
            assert_eq!(
                log[position].1,
                vec![sink, 1, MESSAGE_ID, buffer, SOURCE_FILE, line]
            );
        }
    }

    #[test]
    fn constructor_reports_when_only_the_second_distance_is_large() {
        let mut f = fixture(0, 0, 1, &[[0.0, 0.001, 0.5, 0.0]]);
        construct(&mut f);
        assert_eq!(calls_to(&f.calls, SINK_FUNCTION).len(), 3);
    }

    #[test]
    fn constructor_is_quiet_for_small_nan_or_unflagged_distances() {
        let cases: [(u8, Vec4); 4] = [
            (1, [0.0, 0.005, 0.005, 0.0]),
            (1, [0.0, f32::NAN, f32::NAN, 0.0]),
            (0, [0.0, 0.5, 0.5, 0.0]),
            (1, [0.0, 0.01, 0.01, 0.0]),
        ];
        for (flag, first) in cases {
            let mut f = fixture(0, 0, flag, &[first]);
            construct(&mut f);
            assert!(calls_to(&f.calls, STREAM_CONSTRUCT).is_empty(), "{first:?}");
            assert!(calls_to(&f.calls, SINK_FUNCTION).is_empty());
        }
    }
}

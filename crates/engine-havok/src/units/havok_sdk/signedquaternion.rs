//! `Havok SDK/signedquaternion.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Packers and unpackers of unit quaternions in the sizes 4, 5 and 6 bytes (plus
//! the 2-byte nibble form): the three components that are not the largest are
//! stored as small integers, the largest one is rebuilt from the others. The
//! x87 code of the exe is computed in `f64` and rounded to `float` at every
//! `FSTP float`, as docs/ENGINE_CRATE.md says; the library calls are made by
//! address (`ftol`, square root, sine, cosine, arccosine, the Havok `atan2`
//! approximation).
//!
//! Each unpacker keeps a function-local `static float` scale, built on the first
//! call from the double at `01011590` (its square root times a bit-width
//! constant) with a guard bit in the word after it.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// `_ftol2_sse` (`00ec62c0`): truncates the float in ST0 to an integer in EAX.
const FTOL: u32 = 0x00ec_62c0;
/// Square root of ST0 (`00ec6040`).
const X87_SQRT: u32 = 0x00ec_6040;
/// Cosine of ST0 (`00ec9f70`).
const X87_COS: u32 = 0x00ec_9f70;
/// Sine of ST0 (`00eca0a0`).
const X87_SIN: u32 = 0x00ec_a0a0;
/// Arccosine of ST0 (`00eca1d0`).
const X87_ACOS: u32 = 0x00ec_a1d0;
/// `hkMath_atan2fApproximation` (`00cbff80`, cdecl `(y, x)`, answer in ST0).
const ATAN2_APPROXIMATION: u32 = 0x00cb_ff80;
/// Quaternion normalizer (`005611c0`, fastcall on the quaternion).
const NORMALIZE_QUATERNION: u32 = 0x0056_11c0;
/// Four copies of `0x7fffffff` (`ANDPS` mask that clears the sign bits).
const ABS_MASK: u32 = 0x0101_7d10;
/// `0.5` as a `double`.
const HALF_DOUBLE: u32 = 0x0101_1588;
/// The double whose square root the scales are made from.
const SCALE_ROOT_ARGUMENT: u32 = 0x0101_1590;
/// `float` zero the normalizer compares the squared length with.
const ZERO_FLOAT: u32 = 0x0101_1d78;
/// `float` 3.0 and 0.5 of the Newton step that refines `RSQRTSS`.
const THREE_FLOAT: u32 = 0x0101_7718;
const HALF_FLOAT: u32 = 0x0101_6248;
/// `float` the arc cosine's argument is compared with (the exe's limit of `acos`).
const ACOS_LIMIT_FLOAT: u32 = 0x0101_2068;
/// `float` that stands for the arc cosine of -1 (and of NaN).
const ACOS_OF_MINUS_ONE_FLOAT: u32 = 0x0102_b3c8;
/// The `double` the angles are divided by (`pi` rounded to a `float`).
const ANGLE_UNIT_DOUBLE: u32 = 0x0101_ff40;
/// Doubles that scale the polar form: 511, 1023.
const POLAR_RADIUS_STEPS_DOUBLE: u32 = 0x010e_c940;
const POLAR_ANGLE_STEPS_DOUBLE: u32 = 0x010e_c938;
/// Bit-width constants of the unpackers' scales: 63, 2047, 16383 (doubles).
const STEPS_63_DOUBLE: u32 = 0x010e_c930;
const STEPS_2047_DOUBLE: u32 = 0x010e_c948;
const STEPS_16383_DOUBLE: u32 = 0x010e_c950;
/// `1/7` as a `double` (the 2-byte form's step).
const NIBBLE_STEP_DOUBLE: u32 = 0x010e_c958;
/// Guard words and scales of the function-local statics of each function.
const STATIC_40: (u32, u32) = (0x0126_ef94, 0x0126_ef90);
const STATIC_48: (u32, u32) = (0x0126_ef9c, 0x0126_ef98);
const STATIC_24: (u32, u32) = (0x0126_efa4, 0x0126_efa0);
const STATIC_UNPACK_40: (u32, u32) = (0x0126_efac, 0x0126_efa8);
const STATIC_UNPACK_48: (u32, u32) = (0x0126_efb4, 0x0126_efb0);
/// Byte that guards the one-time start of the colors table (`hkaProcessFlyingColors`).
const COLORS_READY: u32 = 0x0126_efb8;
/// `hkaProcessFlyingColors` (`00e63c60`, cdecl, one argument: the guard byte).
const PROCESS_FLYING_COLORS: u32 = 0x00e6_3c60;

/// `FSTP float` then `FLD float`: the stack value as a `float`, back as the x87 holds it.
fn float_round_trip(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// `FISTP qword` with the truncating control word: the low dword of the result.
/// NaN and values outside the 64-bit range give the "integer indefinite", whose
/// low dword is 0.
fn fistp_truncate_low(value: f64) -> u32 {
    if value.is_nan() || value.abs() >= 9_223_372_036_854_775_808.0 {
        0
    } else {
        value.trunc() as i64 as u32
    }
}

/// `FCOM` of 0.0 against a `float`, taken as "the value is negative" (bit test
/// `TEST AH,0x41`; NaN gives false).
fn is_negative(bits: u32) -> bool {
    f32::from_bits(bits) < 0.0
}

/// The quaternion at `quat` as four bit patterns.
fn read_quaternion(e: &Engine, quat: Ptr) -> [u32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.u32(quat.addr() + offset))
}

/// `ANDPS` of the quaternion with the sign-clearing mask.
fn absolute_values(e: &Engine, bits: [u32; 4]) -> [u32; 4] {
    let mut result = bits;
    for (lane, value) in result.iter_mut().enumerate() {
        *value &= e.mem.u32(ABS_MASK + 4 * lane as u32);
    }
    result
}

/// Index of the first largest of the four absolute values (strict `<` compares of
/// the exe's `FCOMPP`s, so a NaN never wins).
fn largest_component(abs: [u32; 4]) -> usize {
    let value = |index: usize| f32::from_bits(abs[index]);
    let mut largest = 0;
    if value(0) < value(1) {
        largest = 1;
    }
    if value(largest) < value(2) {
        largest = 2;
    }
    if value(largest) < value(3) {
        largest = 3;
    }
    largest
}

/// The static scale of a packer or unpacker, made on the first call: the guard
/// word gets bit 0, then `scale = float(float(sqrt(root_argument)) * step)` (or
/// its reciprocal for the unpackers, `reciprocal`).
fn ensure_scale(e: &mut Engine, statics: (u32, u32), step_address: u32, reciprocal: bool) {
    let (guard, scale) = statics;
    if e.mem.u8(guard) & 1 != 0 {
        return;
    }
    let argument = e.mem.f64(SCALE_ROOT_ARGUMENT);
    let word = e.mem.u32(guard);
    e.mem.set_u32(guard, word | 1);
    let root = float_round_trip(e.call(X87_SQRT, &args![argument]).f64());
    let product = x87_mul(root, e.mem.f64(step_address));
    let value = if reciprocal {
        x87_div(1.0, product)
    } else {
        product
    };
    e.mem.set_u32(scale, x87_store32(value));
}

/// The three components that are not `largest`, scaled by the static scale and
/// truncated (`ftol`): the first one from `first`, the next two skipping `largest`.
/// Returns the integer results in the order they were made.
fn quantize_remaining(e: &mut Engine, bits: [u32; 4], largest: usize, scale: u32) -> [i32; 3] {
    let scale_value = x87_load32(e.mem.u32(scale));
    let mut index = usize::from(largest == 0);
    let first = x87_mul(x87_load32(bits[index]), scale_value);
    let first = e.call(FTOL, &args![first]).i32();
    index += 1;
    index += usize::from(index == largest);
    let second = x87_mul(x87_load32(bits[index]), scale_value);
    let second = e.call(FTOL, &args![second]).i32();
    index += 1;
    index += usize::from(index == largest);
    let third = x87_mul(scale_value, x87_m32(bits[index]));
    let third = e.call(FTOL, &args![third]).i32();
    [first, second, third]
}

// Translated from 00e52320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `packSignedQuaternion32` (engine map): cdecl `(quat, out)`. Packs the unit
/// quaternion at `quat` (x, y, z, w) into the dword at `out`: the four sign bits
/// go to bits 28 to 31; when x, y and z are all zero nothing else is written.
/// Otherwise the absolute values of x, y, z are normalized (`RSQRTSS` and a
/// Newton step; a zero squared length gives 0), and
/// - `i1 = ftol((2 * acos(z') / pi) * 511 + 0.5)`, the arc cosine clamped by the
///   exe (`z' >= 1` gives 0, `z' <= -1` or NaN gives the float at `0102b3c8`),
/// - `i2 = ftol(((2 * atan2(y', x') / pi) * (2 * i1)) + 0.5)`,
/// - the low 18 bits are `i2 + i1 * i1` and bits 18 to 27 are
///   `trunc(sqrt(1 - |w|) * 1023 + 0.5)`.
pub fn fn_00e52320(e: &mut Engine, quat: Ptr, out: Ptr) {
    let bits = read_quaternion(e, quat);
    let sign = |lane: usize| u32::from(is_negative(bits[lane]));
    let sign_bits = ((((((sign(3) * 2) | sign(2)) * 2) | sign(1)) * 2) | sign(0)) << 0x1c;
    e.mem.set_u32(out.addr(), 0);
    e.mem.set_u32(out.addr(), sign_bits);
    let abs_bits = absolute_values(e, bits);
    let abs = abs_bits.map(f32::from_bits);
    if abs[0] == 0.0 && abs[1] == 0.0 && abs[2] == 0.0 {
        return;
    }
    let zero = e.mem.f32(ZERO_FLOAT);
    let three = e.mem.f32(THREE_FLOAT);
    let half = e.mem.f32(HALF_FLOAT);
    let squares = abs.map(|value| value * value);
    let sum = squares[2] + (squares[1] + squares[0]);
    let estimate = rsqrt_estimate(sum);
    let refined = (half * estimate) * (three - (sum * estimate) * estimate);
    let inverse = if sum == zero { 0.0 } else { refined };
    let normalized = abs.map(|value| inverse * value);

    // The angle from z: `|z'| < limit` goes through acos.
    let z = x87_load32(normalized[2].to_bits());
    let abs_z = x87_load32(x87_store32(z.abs()));
    let limit = f64::from(e.mem.f32(ACOS_LIMIT_FLOAT));
    let angle = if abs_z < limit {
        float_round_trip(e.call(X87_ACOS, &args![z]).f64())
    } else if z > 0.0 {
        0.0
    } else {
        f64::from(e.mem.f32(ACOS_OF_MINUS_ONE_FLOAT))
    };
    let angle_unit = e.mem.f64(ANGLE_UNIT_DOUBLE);
    let half_double = e.mem.f64(HALF_DOUBLE);
    let radius_steps = e.mem.f64(POLAR_RADIUS_STEPS_DOUBLE);
    let doubled = x87_add(angle, angle);
    let radius = x87_add(
        x87_mul(x87_div(doubled, angle_unit), radius_steps),
        half_double,
    );
    let first = e.call(FTOL, &args![radius]).i32();
    let y = x87_quiet(normalized[1]);
    let x = x87_quiet(normalized[0]);
    let azimuth = e.call(ATAN2_APPROXIMATION, &args![y, x]).f64();
    let first_twice = first.wrapping_add(first);
    let ratio = x87_add(
        x87_mul(
            x87_div(x87_add(azimuth, azimuth), angle_unit),
            f64::from(first_twice),
        ),
        half_double,
    );
    let second = e.call(FTOL, &args![ratio]).i32();
    let w_bits = x87_quiet_bits(abs_bits[3]);
    let one_minus_w = float_round_trip(x87_sub(1.0, x87_load32(w_bits)));
    let first_squared = first.wrapping_mul(first);
    let low_bits = second.wrapping_add(first_squared) as u32;
    let root = float_round_trip(e.call(X87_SQRT, &args![one_minus_w]).f64());
    let scaled = x87_add(
        x87_mul(root, e.mem.f64(POLAR_ANGLE_STEPS_DOUBLE)),
        half_double,
    );
    let high_bits = fistp_truncate_low(scaled);
    let stored = e.mem.u32(out.addr());
    e.mem
        .set_u32(out.addr(), stored | (high_bits << 0x12 | low_bits));
}

// Translated from 00e52590 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (a packer, cdecl `(quat, out)`): packs the quaternion into five
/// bytes at `out`. Finds the largest absolute component (index `m`, first on ties);
/// the other three, scaled by the static scale (`float(sqrt(2.0)) * 2047`, a
/// guarded function-local) and truncated, get 0x7ff added (16-bit wrap) and are
/// written as 12-bit fields: byte 0 low 8 bits of the first, byte 1 high 4 bits of
/// the first and low 4 of the second, byte 2 bits 4 to 11 of the second, byte 3
/// low 8 of the third, byte 4 high 4 bits of the third with `m` in bits 4 to 5
/// and the sign of component `m` in bit 6.
pub fn fn_00e52590(e: &mut Engine, quat: Ptr, out: Ptr) {
    ensure_scale(e, STATIC_40, STEPS_2047_DOUBLE, false);
    let bits = read_quaternion(e, quat);
    let abs_bits = absolute_values(e, bits);
    let largest = largest_component(abs_bits);
    let quantized = quantize_remaining(e, bits, largest, STATIC_40.1);
    let field = quantized.map(|value| (value as u16).wrapping_add(0x7ff));
    let out = out.addr();
    e.mem.set_u8(out, field[0] as u8);
    e.mem.set_u8(
        out + 1,
        ((field[0] >> 8) as u8) | ((field[1] as u8).wrapping_shl(4)),
    );
    e.mem.set_u8(out + 2, (field[1] >> 4) as u8);
    e.mem.set_u8(out + 3, field[2] as u8);
    let sign = u8::from(is_negative(bits[largest]));
    e.mem.set_u8(
        out + 4,
        ((sign * 4) | largest as u8).wrapping_shl(4) | (field[2] >> 8) as u8,
    );
}

// Translated from 00e526f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (a packer, cdecl `(quat, out)`): packs the quaternion into six
/// bytes (three words) at `out`. Like `fn_00e52590` with 15-bit fields (scale
/// `float(sqrt(2.0)) * 16383`, bias 0x3fff): word 0 is bit 0 of the largest index `m`
/// in bit 15 over the first field, word 1 is bit 1 of `m` in bit 15 over the second
/// field, word 2 is the sign of component `m` in bit 15 over the third field.
pub fn fn_00e526f0(e: &mut Engine, quat: Ptr, out: Ptr) {
    ensure_scale(e, STATIC_48, STEPS_16383_DOUBLE, false);
    let bits = read_quaternion(e, quat);
    let abs_bits = absolute_values(e, bits);
    let largest = largest_component(abs_bits);
    let quantized = quantize_remaining(e, bits, largest, STATIC_48.1);
    let field = quantized.map(|value| (value as u16).wrapping_add(0x3fff));
    let out = out.addr();
    e.mem.set_u16(out, ((largest as u16) << 15) | field[0]);
    e.mem
        .set_u16(out + 2, ((largest as u16 & 0xfffe) << 14) | field[1]);
    let sign = u16::from(is_negative(bits[largest]));
    e.mem.set_u16(out + 4, (sign << 15) | field[2]);
}

// Translated from 00e52880 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (an unpacker, cdecl `(packed, out)`): unpacks the four nibbles of
/// the two bytes at `packed` into the four floats at `out`: `(nibble - 7) * c` with
/// `c` the double at `010ec958` (1/7 as a float), low nibble of byte 0 first. Then
/// `005611c0(out)` normalizes the result.
pub fn fn_00e52880(e: &mut Engine, packed: Ptr, out: Ptr) {
    let first = e.mem.u8(packed.addr());
    let second = e.mem.u8(packed.addr() + 1);
    let step = e.mem.f64(NIBBLE_STEP_DOUBLE);
    let steps = [
        i32::from(first & 0xf) - 7,
        i32::from(first >> 4) - 7,
        i32::from(second & 0xf) - 7,
        i32::from(second >> 4) - 7,
    ];
    let out_address = out.addr();
    for lane in 0..4 {
        e.mem.set_u32(out_address + 4 * lane, 0);
    }
    for (lane, step_count) in steps.iter().enumerate() {
        let value = x87_mul(f64::from(*step_count), step);
        e.mem
            .set_u32(out_address + 4 * lane as u32, x87_store32(value));
    }
    e.call(NORMALIZE_QUATERNION, &args![out]);
}

/// Puts the three decoded values (already scaled, as float bit patterns, in
/// order) into the lanes of `out` that are not `largest`.
fn place_components(e: &mut Engine, out: Ptr, largest: usize, values: [u32; 3]) {
    let out = out.addr();
    let mut lane = usize::from(largest == 0);
    e.mem.set_u32(out + 4 * lane as u32, values[0]);
    lane += 1;
    lane += usize::from(lane == largest);
    e.mem.set_u32(out + 4 * lane as u32, values[1]);
    lane += 1;
    lane += usize::from(lane == largest);
    e.mem.set_u32(out + 4 * lane as u32, values[2]);
}

/// Stores `sqrt(1 - x0^2 - x1^2 - x2^2 - x3^2)` into the lane `largest` of
/// `out` (negated when `negative`), the squares taken off `1` one at a time as
/// the x87 code does.
fn complete_quaternion(e: &mut Engine, out: Ptr, largest: usize, negative: bool) {
    let out_address = out.addr();
    let mut accumulator = 1.0;
    for lane in 0..4 {
        let value = x87_load32(e.mem.u32(out_address + 4 * lane));
        accumulator = x87_sub(accumulator, x87_mul(value, value));
    }
    let remainder = float_round_trip(accumulator);
    let root = x87_store32(e.call(X87_SQRT, &args![remainder]).f64());
    let stored = if negative {
        (-f32::from_bits(root)).to_bits()
    } else {
        root
    };
    e.mem.set_u32(out_address + 4 * largest as u32, stored);
}

// Translated from 00e52920 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (an unpacker, cdecl `(packed, out)`): the inverse of a three-byte
/// packing with 7-bit fields. The largest component's index is `((b1 & 0x80) |
/// (b0 >> 1)) >> 6`; the other three are `((byte & 0x7f) - 0x3f) * scale`, with
/// the scale `1 / (float(sqrt(2.0)) * 63)` of a guarded function-local static; the
/// largest is `sqrt(1 - sum of squares)`, negative when bit 7 of byte 2 is set.
pub fn fn_00e52920(e: &mut Engine, packed: Ptr, out: Ptr) {
    ensure_scale(e, STATIC_24, STEPS_63_DOUBLE, true);
    let base = packed.addr();
    let (b0, b1, b2) = (e.mem.u8(base), e.mem.u8(base + 1), e.mem.u8(base + 2));
    let largest = ((u32::from(b1) & 0x80 | u32::from(b0 >> 1)) >> 6) as usize;
    for lane in 0..4 {
        e.mem.set_u32(out.addr() + 4 * lane, 0);
    }
    let scale = x87_load32(e.mem.u32(STATIC_24.1));
    let steps = [
        i32::from(b0 & 0x7f) - 0x3f,
        i32::from(b1 & 0x7f) - 0x3f,
        i32::from(b2 & 0x7f) - 0x3f,
    ];
    let values = [
        x87_store32(x87_mul(f64::from(steps[0]), scale)),
        x87_store32(x87_mul(f64::from(steps[1]), scale)),
        x87_store32(x87_mul(scale, f64::from(steps[2]))),
    ];
    place_components(e, out, largest, values);
    let negative = e.mem.u8(base + 2) & 0x80 != 0;
    complete_quaternion(e, out, largest, negative);
}

// Translated from 00e52a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (an unpacker, cdecl `(packed, out)`): the inverse of
/// `packSignedQuaternion32`. With `hi = packed >> 18 & 0x3ff` and
/// `lo = packed & 0x3ffff`: `w = 1 - (hi / 1023)^2`, `k = ftol(float(sqrt(lo)))`,
/// `t = k / 511` (a double division), the angles `a = 0.5 * t * pi` and
/// `b = ((lo - k * k) / (2 * k)) * 0.5 * pi` (0 when `2 * k` is 0); then
/// `x = cos(b) * sin(a) * s`, `y = sin(b) * sin(a) * s`, `z = cos(a) * s` with
/// `s = sqrt(1 - w^2)`, each product rounded to `float` after every step, and
/// bits 28 to 31 of the packed word negate x, y, z, w.
pub fn fn_00e52a70(e: &mut Engine, packed: Ptr, out: Ptr) {
    let word = e.mem.u32(packed.addr());
    let hi = (word >> 0x12) & 0x3ff;
    let lo = word & 0x3_ffff;
    let radius = x87_div(f64::from(hi as i32), e.mem.f64(POLAR_ANGLE_STEPS_DOUBLE));
    let w_bits = x87_store32(x87_sub(1.0, x87_mul(radius, radius)));
    let root = float_round_trip(e.call(X87_SQRT, &args![f64::from(lo as i32)]).f64());
    let k = e.call(FTOL, &args![root]).i32();
    let k_float = f64::from(k as f32);
    let t = x87_div(k_float, e.mem.f64(POLAR_RADIUS_STEPS_DOUBLE));
    let half_t = x87_mul(t, e.mem.f64(HALF_DOUBLE));
    let half_t_pi = x87_mul(half_t, e.mem.f64(ANGLE_UNIT_DOUBLE));
    let a_bits = x87_store32(half_t_pi);
    let two_k_bits = x87_store32(x87_add(k_float, k_float));
    let two_k = x87_load32(two_k_bits);
    let b_bits = if two_k == 0.0 {
        0.0f32.to_bits()
    } else {
        let difference = lo.wrapping_sub((k as u32).wrapping_mul(k as u32)) as i32;
        let ratio = x87_div(f64::from(difference), two_k);
        let half_ratio = x87_mul(e.mem.f64(HALF_DOUBLE), ratio);
        x87_store32(x87_mul(e.mem.f64(ANGLE_UNIT_DOUBLE), half_ratio))
    };
    let w = x87_load32(w_bits);
    let remainder = float_round_trip(x87_sub(1.0, x87_mul(w, w)));
    let s_bits = x87_store32(e.call(X87_SQRT, &args![remainder]).f64());

    let sin_a = x87_store32(e.call(X87_SIN, &args![x87_load32(a_bits)]).f64());
    let cos_b = x87_store32(e.call(X87_COS, &args![x87_load32(b_bits)]).f64());
    let step = x87_store32(x87_mul(x87_load32(cos_b), x87_m32(sin_a)));
    let x_bits = x87_store32(x87_mul(x87_load32(step), x87_m32(s_bits)));
    let sin_b = x87_store32(e.call(X87_SIN, &args![x87_load32(b_bits)]).f64());
    let step = x87_store32(x87_mul(x87_load32(sin_b), x87_m32(sin_a)));
    let y_bits = x87_store32(x87_mul(x87_load32(step), x87_m32(s_bits)));
    let cos_a = x87_store32(e.call(X87_COS, &args![x87_load32(a_bits)]).f64());
    let z_bits = x87_store32(x87_mul(x87_load32(cos_a), x87_m32(s_bits)));
    let out = out.addr();
    e.mem.set_u32(out, x_bits);
    e.mem.set_u32(out + 4, y_bits);
    e.mem.set_u32(out + 8, z_bits);
    e.mem.set_u32(out + 12, w_bits);
    let negate = |bits: u32| (-f32::from_bits(bits)).to_bits();
    if e.mem.u32(packed.addr()) & 0x1000_0000 != 0 {
        e.mem.set_u32(out, negate(x_bits));
    }
    if e.mem.u32(packed.addr()) & 0x2000_0000 != 0 {
        e.mem.set_u32(out + 4, negate(y_bits));
    }
    if e.mem.u32(packed.addr()) & 0x4000_0000 != 0 {
        e.mem.set_u32(out + 8, negate(z_bits));
    }
    if e.mem.u32(packed.addr()) & 0x8000_0000 != 0 {
        e.mem.set_u32(out + 12, negate(w_bits));
    }
}

// Translated from 00e52c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (an unpacker, cdecl `(packed, out)`): the inverse of
/// `fn_00e52590`. From the five bytes: 12-bit fields `v0 = (b1 & 0xf) << 8 | b0`,
/// `v1 = b2 << 4 | b1 >> 4`, `v2 = (b4 & 0xf) << 8 | b3`, the largest index
/// `b4 >> 4 & 3` and its sign in bit 6 of `b4`; scale `1 / (float(sqrt(2.0)) *
/// 2047)` (guarded function-local static), values `(v - 0x7ff) * scale`, the largest
/// is `sqrt(1 - sum of squares)`.
pub fn fn_00e52c30(e: &mut Engine, packed: Ptr, out: Ptr) {
    ensure_scale(e, STATIC_UNPACK_40, STEPS_2047_DOUBLE, true);
    let base = packed.addr();
    let b: [u16; 5] = [0, 1, 2, 3, 4].map(|offset| u16::from(e.mem.u8(base + offset)));
    let v0 = ((b[1] & 0xf) << 8) | b[0];
    let v1 = (b[2] << 4) | (b[1] >> 4);
    let v2 = ((b[4] & 0xf) << 8) | b[3];
    let largest = usize::from((b[4] >> 4) & 3);
    for lane in 0..4 {
        e.mem.set_u32(out.addr() + 4 * lane, 0);
    }
    let scale = x87_load32(e.mem.u32(STATIC_UNPACK_40.1));
    let steps = [v0, v1, v2].map(|field| i32::from(field as i16) - 0x7ff);
    let values = [
        x87_store32(x87_mul(f64::from(steps[0]), scale)),
        x87_store32(x87_mul(f64::from(steps[1]), scale)),
        x87_store32(x87_mul(scale, f64::from(steps[2]))),
    ];
    place_components(e, out, largest, values);
    let negative = e.mem.u8(base + 4) & 0x40 != 0;
    complete_quaternion(e, out, largest, negative);
}

// Translated from 00e52d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (an unpacker, cdecl `(packed, out)`): the inverse of
/// `fn_00e526f0`. From the three words: fields `w & 0x7fff`, largest index
/// `((w1 & 0x8000) | (w0 >> 1)) >> 14`, sign in bit 15 of word 2; scale
/// `1 / (float(sqrt(2.0)) * 16383)` (guarded function-local static), values
/// `(v - 0x3fff) * scale`. The largest is `sqrt(1 - s)` where `s` is the sum of the
/// four squares added the SSE way (`MULPS`, then lanes `(s3 + s1) + (s2 + s0)`).
pub fn fn_00e52d90(e: &mut Engine, packed: Ptr, out: Ptr) {
    ensure_scale(e, STATIC_UNPACK_48, STEPS_16383_DOUBLE, true);
    let base = packed.addr();
    let (w0, w1, w2) = (e.mem.u16(base), e.mem.u16(base + 2), e.mem.u16(base + 4));
    let largest = ((u32::from(w1) & 0x8000 | u32::from(w0 >> 1)) >> 14) as usize;
    for lane in 0..4 {
        e.mem.set_u32(out.addr() + 4 * lane, 0);
    }
    let scale = x87_load32(e.mem.u32(STATIC_UNPACK_48.1));
    let steps = [w0, w1, w2].map(|word| i32::from((word & 0x7fff) as i16) - 0x3fff);
    let values = [
        x87_store32(x87_mul(f64::from(steps[0]), scale)),
        x87_store32(x87_mul(f64::from(steps[1]), scale)),
        x87_store32(x87_mul(scale, f64::from(steps[2]))),
    ];
    place_components(e, out, largest, values);
    let lanes: [f32; 4] = [0, 4, 8, 12].map(|offset| e.mem.f32(out.addr() + offset));
    let squares = lanes.map(|value| value * value);
    let sum = (squares[3] + squares[1]) + (squares[2] + squares[0]);
    let remainder = float_round_trip(x87_sub(1.0, x87_load32(sum.to_bits())));
    let root = x87_store32(e.call(X87_SQRT, &args![remainder]).f64());
    let negative = e.mem.u16(base + 4) & 0x8000 != 0;
    let stored = if negative {
        (-f32::from_bits(root)).to_bits()
    } else {
        root
    };
    e.mem.set_u32(out.addr() + 4 * largest as u32, stored);
}

// Translated from 00e52fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `computePackedNurbsOffsets` (engine map): cdecl `(base, table, row, offset)`.
/// First makes sure the colors table is ready (`hkaProcessFlyingColors`
/// `00e63c60` on the guard byte `0126efb8`, which it sets); if it still is not,
/// returns 0. Otherwise `table[row] + (offset & 0x7fffffff) + base`.
pub fn compute_packed_nurbs_offsets(
    e: &mut Engine,
    base: u32,
    table: Ptr,
    row: u32,
    offset: u32,
) -> u32 {
    if e.mem.u8(COLORS_READY) == 0 {
        e.call(PROCESS_FLYING_COLORS, &args![COLORS_READY]);
        if e.mem.u8(COLORS_READY) == 0 {
            return 0;
        }
    }
    e.mem
        .u32(table.addr().wrapping_add(row.wrapping_mul(4)))
        .wrapping_add(offset & 0x7fff_ffff)
        .wrapping_add(base)
}

// Translated from 00e52f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (address held in the static data word at 011be4ac): reads no argument and only returns; how many words its callers push is not confirmed.
pub fn fn_00e52f10(_e: &mut Engine) {}

// Translated from 00e52f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (address held in the static data word at 011be4b0): reads no argument and only returns; how many words its callers push is not confirmed.
pub fn fn_00e52f20(_e: &mut Engine) {}

// Translated from 00e52f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (address held in the static data word at 011be4a4): reads no argument and only returns; how many words its callers push is not confirmed.
pub fn fn_00e52f30(_e: &mut Engine) {}

// Translated from 00e52860 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, two words, `source` and `out`; descriptions, not confirmed; the address is in the function table at 011be490): copies the four floats at `source` to `out`, one after the other through the x87 stack (`FLD`/`FSTP`), so a signalling NaN comes out quiet.
pub fn fn_00e52860(e: &mut Engine, source: Ptr, out: Ptr) {
    for lane in 0..4u32 {
        let bits = x87_quiet_bits(e.mem.u32(source.addr() + 4 * lane));
        e.mem.set_u32(out.addr() + 4 * lane, bits);
    }
}

// Translated from 00e52ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, two words, `source` and `out`; descriptions, not confirmed; the address is in the function table at 011be490): the same copy as [`fn_00e52860`].
pub fn fn_00e52ef0(e: &mut Engine, source: Ptr, out: Ptr) {
    for lane in 0..4u32 {
        let bits = x87_quiet_bits(e.mem.u32(source.addr() + 4 * lane));
        e.mem.set_u32(out.addr() + 4 * lane, bits);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e52320, fn_00e52320(Ptr, Ptr)),
        entry!(0x00e52590, fn_00e52590(Ptr, Ptr)),
        entry!(0x00e526f0, fn_00e526f0(Ptr, Ptr)),
        entry!(0x00e52880, fn_00e52880(Ptr, Ptr)),
        entry!(0x00e52920, fn_00e52920(Ptr, Ptr)),
        entry!(0x00e52a70, fn_00e52a70(Ptr, Ptr)),
        entry!(0x00e52c30, fn_00e52c30(Ptr, Ptr)),
        entry!(0x00e52d90, fn_00e52d90(Ptr, Ptr)),
        entry!(0x00e52fa0, compute_packed_nurbs_offsets(u32, Ptr, u32, u32) -> u32),
        entry!(0x00e52f10, fn_00e52f10()),
        entry!(0x00e52f20, fn_00e52f20()),
        entry!(0x00e52f30, fn_00e52f30()),
        entry!(0x00e52860, fn_00e52860(Ptr, Ptr)),
        entry!(0x00e52ef0, fn_00e52ef0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arg_f64(words: &[u32]) -> f64 {
        f64::from_bits(u64::from(words[0]) | u64::from(words[1]) << 32)
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An engine with the exe's constants this unit reads and doubles for the
    /// math library calls.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x2_0000);
        e.map(0x010e_0000, 0x1_0000);
        e.map(0x0126_e000, 0x2000);
        for lane in 0..4 {
            e.mem.set_u32(ABS_MASK + 4 * lane, 0x7fff_ffff);
        }
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        e.mem.set_f64(SCALE_ROOT_ARGUMENT, 2.0);
        e.mem.set_f32(ZERO_FLOAT, 0.0);
        e.mem.set_f32(THREE_FLOAT, 3.0);
        e.mem.set_f32(HALF_FLOAT, 0.5);
        e.mem.set_f32(ACOS_LIMIT_FLOAT, 1.0);
        e.mem.set_f32(ACOS_OF_MINUS_ONE_FLOAT, std::f32::consts::PI);
        e.mem
            .set_f64(ANGLE_UNIT_DOUBLE, f64::from(std::f32::consts::PI));
        e.mem.set_f64(POLAR_RADIUS_STEPS_DOUBLE, 511.0);
        e.mem.set_f64(POLAR_ANGLE_STEPS_DOUBLE, 1023.0);
        e.mem.set_f64(STEPS_63_DOUBLE, 63.0);
        e.mem.set_f64(STEPS_2047_DOUBLE, 2047.0);
        e.mem.set_f64(STEPS_16383_DOUBLE, 16383.0);
        e.mem.set_f64(NIBBLE_STEP_DOUBLE, f64::from(1.0f32 / 7.0));
        e.register(FTOL, |_, a| Ret {
            eax: arg_f64(a).trunc() as i32 as u32,
            ..Ret::default()
        });
        e.register(X87_SQRT, |_, a| st0(arg_f64(a).sqrt()));
        e.register(X87_COS, |_, a| st0(arg_f64(a).cos()));
        e.register(X87_SIN, |_, a| st0(arg_f64(a).sin()));
        e.register(X87_ACOS, |_, a| st0(arg_f64(a).acos()));
        e.register(ATAN2_APPROXIMATION, |_, a| {
            st0(f64::from(f32::from_bits(a[0])).atan2(f64::from(f32::from_bits(a[1]))))
        });
        e.register(NORMALIZE_QUATERNION, |_, _| Ret::default());
        e
    }

    fn alloc_ptr(e: &mut Engine, size: u32) -> Ptr {
        Ptr::new(e.mem.alloc(size))
    }

    fn put_quaternion(e: &mut Engine, values: [f32; 4]) -> Ptr {
        let at = e.mem.alloc(16);
        for (lane, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, *value);
        }
        Ptr::new(at)
    }

    fn read_floats(e: &Engine, at: Ptr) -> [f32; 4] {
        [0, 4, 8, 12].map(|offset| e.mem.f32(at.addr() + offset))
    }

    fn assert_close(actual: [f32; 4], expected: [f32; 4], tolerance: f32) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() <= tolerance,
                "lane {lane}: {actual:?} vs {expected:?}"
            );
        }
    }

    fn unit(values: [f32; 4]) -> [f32; 4] {
        let length = values.iter().map(|v| v * v).sum::<f32>().sqrt();
        values.map(|v| v / length)
    }

    #[test]
    fn pack_32_writes_the_sign_bits_alone_when_xyz_are_zero() {
        let mut e = engine();
        let quat = put_quaternion(&mut e, [-0.0, 0.0, 0.0, -1.0]);
        let out = alloc_ptr(&mut e, 4);
        e.mem.set_u32(out.addr(), 0x1234_5678);
        e.call(0x00e5_2320, &args![quat, out]);
        assert_eq!(e.mem.u32(out.addr()), 0x8000_0000);
        // A NaN x is not zero: it goes the long way and writes more.
        let quat = put_quaternion(&mut e, [f32::NAN, 0.0, 0.0, 1.0]);
        e.call(0x00e5_2320, &args![quat, out]);
        assert_ne!(e.mem.u32(out.addr()), 0);
    }

    #[test]
    fn pack_32_and_its_unpacker_round_trip() {
        for values in [
            [0.5, 0.5, 0.5, 0.5],
            [-0.1, 0.7, -0.3, 0.64],
            [0.2, -0.3, 0.8, -0.5],
            [0.3, 0.2, -0.4, 0.8],
        ] {
            let mut e = engine();
            let values = unit(values);
            let quat = put_quaternion(&mut e, values);
            let packed = alloc_ptr(&mut e, 4);
            e.call(0x00e5_2320, &args![quat, packed]);
            let word = e.mem.u32(packed.addr());
            for (lane, bit) in [0x1000_0000u32, 0x2000_0000, 0x4000_0000, 0x8000_0000]
                .iter()
                .enumerate()
            {
                assert_eq!(word & bit != 0, values[lane] < 0.0, "{values:?}");
            }
            let back = alloc_ptr(&mut e, 16);
            e.call(0x00e5_2a70, &args![packed, back]);
            assert_close(read_floats(&e, back), values, 0.03);
        }
    }

    #[test]
    fn pack_32_takes_the_arc_cosine_limits() {
        let mut e = engine();
        let packed = alloc_ptr(&mut e, 4);
        // z' about 1 (the estimate may land a hair below or above): radius 0
        // or tiny; |w| = 0 gives sqrt(1) * 1023 in bits 18 to 27.
        let quat = put_quaternion(&mut e, [0.0, 0.0, 1.0, 0.0]);
        e.call(0x00e5_2320, &args![quat, packed]);
        let word = e.mem.u32(packed.addr());
        assert_eq!(word >> 0x12 & 0x3ff, 1023);
        assert!(word & 0x3_ffff <= 1, "{word:08x}");
        // A NaN z' (z' <= -1 does the same) gives the float at 0102b3c8 (pi):
        // the radius field is ftol(2 * pi / pi * 511 + 0.5) = 1022, whose
        // square spills over bit 18, ORed with sqrt(1 - 0) * 1023 << 18.
        let quat = put_quaternion(&mut e, [0.0, 0.0, f32::NAN, 0.0]);
        e.call(0x00e5_2320, &args![quat, packed]);
        assert_eq!(e.mem.u32(packed.addr()), (1023u32 << 0x12) | (1022 * 1022));
        // With a positive z' past the limit the angle is 0, and acos is not
        // called.
        e.mem.set_f32(ACOS_LIMIT_FLOAT, 0.0);
        let quat = put_quaternion(&mut e, [0.0, 0.0, 0.5, 0.0]);
        e.call_log = Some(vec![]);
        e.call(0x00e5_2320, &args![quat, packed]);
        let log = e.call_log.clone().unwrap();
        assert_eq!(log.iter().filter(|c| c.0 == X87_ACOS).count(), 0);
        assert_eq!(e.mem.u32(packed.addr()) & 0x3_ffff, 0);
    }

    #[test]
    fn pack_40_makes_the_12_bit_fields_by_hand() {
        let mut e = engine();
        let quat = put_quaternion(&mut e, [0.1, -0.2, 0.9, 0.3]);
        let out = alloc_ptr(&mut e, 8);
        e.call(0x00e5_2590, &args![quat, out]);
        let scale = (f64::from(2.0f64.sqrt() as f32) * 2047.0) as f32;
        assert_eq!(e.mem.f32(STATIC_40.1), scale);
        assert_eq!(e.mem.u8(STATIC_40.0) & 1, 1);
        let field = |value: f32| {
            ((f64::from(value) * f64::from(scale)).trunc() as i32 as u16).wrapping_add(0x7ff)
        };
        let (a, b, c) = (field(0.1), field(-0.2), field(0.3));
        assert_eq!(
            e.mem.bytes(out.addr(), 5),
            vec![
                a as u8,
                (a >> 8) as u8 | ((b as u8) << 4),
                (b >> 4) as u8,
                c as u8,
                (2 << 4) | (c >> 8) as u8
            ]
        );
        assert_eq!(
            e.mem.bytes(out.addr(), 5),
            vec![0x20, 0xd9, 0x5b, 0x63, 0x2b]
        );
    }

    #[test]
    fn pack_40_puts_the_sign_of_the_largest_in_bit_6() {
        let mut e = engine();
        let quat = put_quaternion(&mut e, [0.1, -0.2, 0.3, -0.9]);
        let out = alloc_ptr(&mut e, 8);
        e.call(0x00e5_2590, &args![quat, out]);
        // Largest index 3, negative: (4 | 3) << 4.
        assert_eq!(e.mem.u8(out.addr() + 4) & 0xf0, 0x70);
        // The first component is the largest on a tie.
        let tie = put_quaternion(&mut e, [0.5, 0.5, 0.5, 0.5]);
        e.call(0x00e5_2590, &args![tie, out]);
        assert_eq!(e.mem.u8(out.addr() + 4) & 0xf0, 0x00);
    }

    #[test]
    fn pack_40_and_its_unpacker_round_trip() {
        for values in [
            [0.5, 0.5, 0.5, 0.5],
            [-0.1, 0.7, -0.3, 0.64],
            [0.2, -0.3, 0.8, -0.5],
            [0.3, 0.2, -0.4, -0.8],
            [-0.9, 0.1, 0.2, 0.3],
        ] {
            let mut e = engine();
            let values = unit(values);
            let quat = put_quaternion(&mut e, values);
            let packed = alloc_ptr(&mut e, 8);
            e.call(0x00e5_2590, &args![quat, packed]);
            let back = alloc_ptr(&mut e, 16);
            e.call(0x00e5_2c30, &args![packed, back]);
            assert_close(read_floats(&e, back), values, 0.002);
            assert_eq!(
                e.mem.f32(STATIC_UNPACK_40.1),
                (1.0 / (f64::from(2.0f64.sqrt() as f32) * 2047.0)) as f32
            );
        }
    }

    #[test]
    fn pack_48_makes_the_15_bit_fields_by_hand() {
        let mut e = engine();
        let quat = put_quaternion(&mut e, [0.1, -0.2, 0.9, 0.3]);
        let out = alloc_ptr(&mut e, 8);
        e.call(0x00e5_26f0, &args![quat, out]);
        let scale = (f64::from(2.0f64.sqrt() as f32) * 16383.0) as f32;
        assert_eq!(e.mem.f32(STATIC_48.1), scale);
        let field = |value: f32| {
            ((f64::from(value) * f64::from(scale)).trunc() as i32 as u16).wrapping_add(0x3fff)
        };
        // Largest index 2: bit 0 is 0, bit 1 is 1.
        assert_eq!(e.mem.u16(out.addr()), field(0.1));
        assert_eq!(e.mem.u16(out.addr() + 2), 0x8000 | field(-0.2));
        assert_eq!(e.mem.u16(out.addr() + 4), field(0.3));
        assert_eq!(e.mem.u16(out.addr()), 0x490b);
        // Negative largest, index 1: bit 0 set in word 0, sign in word 2.
        let negative = put_quaternion(&mut e, [0.1, -0.9, 0.2, 0.3]);
        e.call(0x00e5_26f0, &args![negative, out]);
        assert_eq!(e.mem.u16(out.addr()) & 0x8000, 0x8000);
        assert_eq!(e.mem.u16(out.addr() + 2) & 0x8000, 0);
        assert_eq!(e.mem.u16(out.addr() + 4) & 0x8000, 0x8000);
    }

    #[test]
    fn pack_48_and_its_unpacker_round_trip() {
        for values in [
            [0.5, 0.5, 0.5, 0.5],
            [-0.1, 0.7, -0.3, 0.64],
            [0.2, -0.3, 0.8, -0.5],
            [0.3, 0.2, -0.4, -0.8],
            [-0.9, 0.1, 0.2, 0.3],
        ] {
            let mut e = engine();
            let values = unit(values);
            let quat = put_quaternion(&mut e, values);
            let packed = alloc_ptr(&mut e, 8);
            e.call(0x00e5_26f0, &args![quat, packed]);
            let back = alloc_ptr(&mut e, 16);
            e.call(0x00e5_2d90, &args![packed, back]);
            assert_close(read_floats(&e, back), values, 0.0005);
        }
    }

    #[test]
    fn unpack_16_scales_the_four_nibbles_and_normalizes() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let packed = alloc_ptr(&mut e, 2);
        e.mem.write(packed.addr(), &[0x9a, 0x3c]);
        let out = alloc_ptr(&mut e, 16);
        e.mem.set_f32(out.addr() + 12, 99.0);
        e.call(0x00e5_2880, &args![packed, out]);
        let step = 1.0f32 / 7.0;
        let expected = [
            (10 - 7) as f32 * step,
            (9 - 7) as f32 * step,
            (12 - 7) as f32 * step,
            (3 - 7) as f32 * step,
        ];
        assert_close(read_floats(&e, out), expected, 1e-7);
        let log = e.call_log.clone().unwrap();
        let normalizer: Vec<_> = log.iter().filter(|c| c.0 == NORMALIZE_QUATERNION).collect();
        assert_eq!(normalizer.len(), 1);
        assert_eq!(normalizer[0].1, vec![out.addr()]);
    }

    #[test]
    fn unpack_24_decodes_seven_bit_fields_around_the_largest() {
        let mut e = engine();
        // Largest index 3 (bit 7 of byte 0 and of byte 1), negative (bit 7 of
        // byte 2); fields 0x3f + 20, 0x3f - 10, 0x3f + 5.
        let packed = alloc_ptr(&mut e, 4);
        e.mem.write(
            packed.addr(),
            &[0x80 | (0x3f + 20), 0x80 | (0x3f - 10), 0x80 | (0x3f + 5)],
        );
        let out = alloc_ptr(&mut e, 16);
        e.call(0x00e5_2920, &args![packed, out]);
        let scale = (1.0 / (f64::from(2.0f64.sqrt() as f32) * 63.0)) as f32;
        let x = (20.0 * f64::from(scale)) as f32;
        let y = (-10.0 * f64::from(scale)) as f32;
        let z = (5.0 * f64::from(scale)) as f32;
        let w = -(1.0
            - f64::from(x) * f64::from(x)
            - f64::from(y) * f64::from(y)
            - f64::from(z) * f64::from(z))
        .sqrt() as f32;
        assert_close(read_floats(&e, out), [x, y, z, w], 1e-6);
        // Largest index 1 (bit 7 of byte 0 only), positive: the fields skip
        // lane 1.
        e.mem
            .write(packed.addr(), &[0x80 | (0x3f + 1), 0x3f + 2, 0x3f + 3]);
        e.call(0x00e5_2920, &args![packed, out]);
        let lanes = read_floats(&e, out);
        assert_close(
            [lanes[0], 0.0, lanes[2], lanes[3]],
            [scale, 0.0, 2.0 * scale, 3.0 * scale],
            1e-6,
        );
        assert!(lanes[1] > 0.99);
    }

    #[test]
    fn unpack_32_builds_the_polar_form_and_applies_the_signs() {
        let mut e = engine();
        // hi = 511 (w = 1 - (511 / 1023)^2), lo = k * k + j with k = 10, j = 5.
        let word = (511u32 << 0x12) | (100 + 5);
        let packed = alloc_ptr(&mut e, 4);
        e.mem.set_u32(packed.addr(), word);
        let out = alloc_ptr(&mut e, 16);
        e.call(0x00e5_2a70, &args![packed, out]);
        let k = 10.0f64;
        let pi = f64::from(std::f32::consts::PI);
        let a = 0.5 * (k / 511.0) * pi;
        let b = (5.0 / (2.0 * k)) * 0.5 * pi;
        let r = 511.0f64 / 1023.0;
        let w = 1.0 - r * r;
        let s = (1.0 - w * w).sqrt();
        let expected =
            [b.cos() * a.sin() * s, b.sin() * a.sin() * s, a.cos() * s, w].map(|v| v as f32);
        assert_close(read_floats(&e, out), expected, 2e-6);
        // All four sign bits.
        e.mem.set_u32(packed.addr(), word | 0xf000_0000);
        e.call(0x00e5_2a70, &args![packed, out]);
        assert_close(read_floats(&e, out), expected.map(|v| -v), 2e-6);
        // k = 0: the azimuth is 0.
        e.mem.set_u32(packed.addr(), 511u32 << 0x12);
        e.call(0x00e5_2a70, &args![packed, out]);
        let lanes = read_floats(&e, out);
        assert_eq!(lanes[0], 0.0);
        assert_eq!(lanes[1], 0.0);
    }

    #[test]
    fn nurbs_offsets_wait_for_the_colors_table() {
        let mut e = engine();
        let table = alloc_ptr(&mut e, 16);
        e.mem.set_u32(table.addr() + 8, 0x1000);
        e.mem.set_u8(COLORS_READY, 1);
        e.call_log = Some(vec![]);
        let result = e.call(0x00e5_2fa0, &args![0x20u32, table, 2u32, 0x8000_0005u32]);
        assert_eq!(result.u32(), 0x1000 + 5 + 0x20);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        // Not ready: the start-up function is called; if it sets the byte the
        // sum is made, if not the result is 0.
        e.mem.set_u8(COLORS_READY, 0);
        e.register(PROCESS_FLYING_COLORS, |e, a| {
            e.mem.set_u8(a[0], 1);
            Ret::default()
        });
        let result = e.call(0x00e5_2fa0, &args![0x20u32, table, 2u32, 7u32]);
        assert_eq!(result.u32(), 0x1000 + 7 + 0x20);
        e.mem.set_u8(COLORS_READY, 0);
        e.register(PROCESS_FLYING_COLORS, |_, _| Ret::default());
        assert_eq!(
            e.call(0x00e5_2fa0, &args![0x20u32, table, 2u32, 7u32])
                .u32(),
            0
        );
    }
}

#[cfg(test)]
mod tests_a07_gap {
    use super::*;

    #[test]
    fn fn_00e52f10_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e5_2f10, &args![]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00e52f20_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e5_2f20, &args![]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn fn_00e52f30_only_returns() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00e5_2f30, &args![]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn the_float_copies_quiet_a_signalling_nan() {
        for function in [0x00e5_2860u32, 0x00e5_2ef0] {
            let mut e = Engine::new();
            let source = e.mem.alloc(0x10);
            let out = e.mem.alloc(0x10);
            let words = [0x3f80_0000u32, 0x7f80_0001, 0xbf00_0000, 0x7fc0_0000];
            for (lane, word) in words.into_iter().enumerate() {
                e.mem.set_u32(source + 4 * lane as u32, word);
            }
            e.call(function, &args![source, out]);
            let copied = [0u32, 1, 2, 3].map(|lane| e.mem.u32(out + 4 * lane));
            assert_eq!(copied, [0x3f80_0000, 0x7fc0_0001, 0xbf00_0000, 0x7fc0_0000]);
        }
    }
}

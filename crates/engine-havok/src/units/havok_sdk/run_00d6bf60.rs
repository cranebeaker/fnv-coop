//! `Havok SDK/run_00d6bf60` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// One SSE register as four `float` lanes (lane 0 first).
pub(crate) type Vec4 = [f32; 4];

/// The quiet NaN SSE makes of a NaN operand: the quiet bit set, the rest kept.
fn sse_quiet(value: f32) -> f32 {
    f32::from_bits(value.to_bits() | 0x0040_0000)
}

/// One lane of an SSE arithmetic instruction `OP dest, source`: a NaN in the destination
/// operand wins (quieted), then a NaN in the source operand, otherwise `op`'s result.
fn sse_lane(dest: f32, source: f32, op: fn(f32, f32) -> f32) -> f32 {
    if dest.is_nan() {
        sse_quiet(dest)
    } else if source.is_nan() {
        sse_quiet(source)
    } else {
        op(dest, source)
    }
}

/// `ADDPS dest, source`.
pub(crate) fn addps(dest: Vec4, source: Vec4) -> Vec4 {
    std::array::from_fn(|lane| sse_lane(dest[lane], source[lane], |a, b| a + b))
}

/// `SUBPS dest, source`.
pub(crate) fn subps(dest: Vec4, source: Vec4) -> Vec4 {
    std::array::from_fn(|lane| sse_lane(dest[lane], source[lane], |a, b| a - b))
}

/// `MULPS dest, source`.
pub(crate) fn mulps(dest: Vec4, source: Vec4) -> Vec4 {
    std::array::from_fn(|lane| sse_lane(dest[lane], source[lane], |a, b| a * b))
}

/// `ADDSS dest, source`: lane 0 only, the other lanes stay `dest`'s.
pub(crate) fn addss(dest: Vec4, source: Vec4) -> Vec4 {
    [
        sse_lane(dest[0], source[0], |a, b| a + b),
        dest[1],
        dest[2],
        dest[3],
    ]
}

/// `SHUFPS dest, source, selector`: lanes 0 and 1 of the result are lanes of `dest` picked by the
/// selector's two low pairs of bits, lanes 2 and 3 lanes of `source` picked by the high pairs.
pub(crate) fn shufps(dest: Vec4, source: Vec4, selector: u8) -> Vec4 {
    [
        dest[usize::from(selector & 3)],
        dest[usize::from((selector >> 2) & 3)],
        source[usize::from((selector >> 4) & 3)],
        source[usize::from((selector >> 6) & 3)],
    ]
}

/// A 16-byte vector in game memory (`MOVAPS` load).
pub(crate) fn load_vec(e: &Engine, address: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(address.wrapping_add(4 * lane as u32)))
}

/// A 16-byte vector into game memory (`MOVAPS` store).
pub(crate) fn store_vec(e: &mut Engine, address: u32, value: Vec4) {
    for (lane, part) in value.iter().enumerate() {
        e.mem.set_f32(address.wrapping_add(4 * lane as u32), *part);
    }
}

/// The high byte of the x87 status word after `FCOM`/`FCOMP` of `st0` with `operand`
/// (`FNSTSW AX`'s `AH`): `C0` is bit 0 (st0 below the operand, or unordered), `C2` bit 2
/// (unordered) and `C3` bit 6 (equal, or unordered).
pub(crate) fn fcom_status(st0: f64, operand: f64) -> u8 {
    match st0.partial_cmp(&operand) {
        Some(std::cmp::Ordering::Greater) => 0x00,
        Some(std::cmp::Ordering::Less) => 0x01,
        Some(std::cmp::Ordering::Equal) => 0x40,
        None => 0x45,
    }
}

/// The bit patterns of an SSE register's lanes.
pub(crate) fn lane_bits(v: Vec4) -> [u32; 4] {
    std::array::from_fn(|lane| v[lane].to_bits())
}

/// A register from lane bit patterns.
pub(crate) fn from_lane_bits(bits: [u32; 4]) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(bits[lane]))
}

/// `SQRTSS dest, source`: lane 0 is the square root of `source`'s lane 0 (a NaN keeps its
/// payload, quieted; a negative number gives the host's default NaN), the other lanes stay
/// `dest`'s.
pub(crate) fn sqrtss(dest: Vec4, source: Vec4) -> Vec4 {
    let root = if source[0].is_nan() {
        sse_quiet(source[0])
    } else {
        source[0].sqrt()
    };
    [root, dest[1], dest[2], dest[3]]
}

/// `CMPLTPS dest, source`: each lane all ones where `dest < source` (false for NaN).
pub(crate) fn cmpltps(dest: Vec4, source: Vec4) -> Vec4 {
    from_lane_bits(std::array::from_fn(|lane| {
        if dest[lane] < source[lane] {
            u32::MAX
        } else {
            0
        }
    }))
}

/// `CMPLEPS dest, source`: each lane all ones where `dest <= source` (false for NaN).
pub(crate) fn cmpleps(dest: Vec4, source: Vec4) -> Vec4 {
    from_lane_bits(std::array::from_fn(|lane| {
        if dest[lane] <= source[lane] {
            u32::MAX
        } else {
            0
        }
    }))
}

/// `CMPEQPS dest, source`: each lane all ones where `dest == source` (false for NaN).
pub(crate) fn cmpeqps(dest: Vec4, source: Vec4) -> Vec4 {
    from_lane_bits(std::array::from_fn(|lane| {
        if dest[lane] == source[lane] {
            u32::MAX
        } else {
            0
        }
    }))
}

/// `MOVMSKPS`: the sign bits of the four lanes as bits 0 to 3.
pub(crate) fn movmskps(v: Vec4) -> u32 {
    (0..4).fold(0, |mask, lane| mask | ((v[lane].to_bits() >> 31) << lane))
}

/// `SUBSS dest, source`: lane 0 only, the other lanes stay `dest`'s.
pub(crate) fn subss(dest: Vec4, source: Vec4) -> Vec4 {
    [
        sse_lane(dest[0], source[0], |a, b| a - b),
        dest[1],
        dest[2],
        dest[3],
    ]
}

/// `MULSS dest, source`: lane 0 only, the other lanes stay `dest`'s.
pub(crate) fn mulss(dest: Vec4, source: Vec4) -> Vec4 {
    [
        sse_lane(dest[0], source[0], |a, b| a * b),
        dest[1],
        dest[2],
        dest[3],
    ]
}

/// `DIVSS dest, source`: lane 0 only, the other lanes stay `dest`'s.
pub(crate) fn divss(dest: Vec4, source: Vec4) -> Vec4 {
    [
        sse_lane(dest[0], source[0], |a, b| a / b),
        dest[1],
        dest[2],
        dest[3],
    ]
}

/// `CMPEQSS dest, source`: lane 0 all ones where `dest == source` (false for NaN), the other
/// lanes stay `dest`'s.
pub(crate) fn cmpeqss(dest: Vec4, source: Vec4) -> Vec4 {
    let mask = if dest[0] == source[0] { u32::MAX } else { 0 };
    [f32::from_bits(mask), dest[1], dest[2], dest[3]]
}

/// `XORPS dest, source`.
pub(crate) fn xorps(dest: Vec4, source: Vec4) -> Vec4 {
    let (d, s) = (lane_bits(dest), lane_bits(source));
    from_lane_bits(std::array::from_fn(|lane| d[lane] ^ s[lane]))
}

/// `RSQRTSS dest, source`: lane 0 is the host CPU's estimate of `1 / sqrt(source[0])` (the
/// estimate differs between Intel and AMD parts, so the translations that use it are tested to a
/// tolerance; off x86-64 the exact value stands in), the other lanes stay `dest`'s.
#[cfg(target_arch = "x86_64")]
pub(crate) fn rsqrtss(dest: Vec4, source: Vec4) -> Vec4 {
    use std::arch::x86_64::{_mm_cvtss_f32, _mm_rsqrt_ss, _mm_set_ss};
    // SAFETY: SSE is part of the x86-64 baseline.
    let estimate = unsafe { _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(source[0]))) };
    [estimate, dest[1], dest[2], dest[3]]
}

/// `RSQRTSS` (not x86-64): the exact value.
#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn rsqrtss(dest: Vec4, source: Vec4) -> Vec4 {
    [1.0 / source[0].sqrt(), dest[1], dest[2], dest[3]]
}

/// `RSQRTPS`: the host CPU's per-lane estimate of `1 / sqrt(lane)` (it differs between Intel and
/// AMD parts, so the translations that use it are tested to a tolerance; off x86-64 the exact
/// value stands in).
#[cfg(target_arch = "x86_64")]
pub(crate) fn rsqrtps(v: Vec4) -> Vec4 {
    use std::arch::x86_64::{_mm_loadu_ps, _mm_rsqrt_ps, _mm_storeu_ps};
    let mut out = [0.0f32; 4];
    // SAFETY: SSE is part of the x86-64 baseline and both arrays hold four floats.
    unsafe { _mm_storeu_ps(out.as_mut_ptr(), _mm_rsqrt_ps(_mm_loadu_ps(v.as_ptr()))) };
    out
}

/// `RSQRTPS` (not x86-64): the exact value.
#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn rsqrtps(v: Vec4) -> Vec4 {
    v.map(|lane| 1.0 / lane.sqrt())
}

/// `UNPCKLPS dest, source`: `(d0, s0, d1, s1)`.
pub(crate) fn unpcklps(dest: Vec4, source: Vec4) -> Vec4 {
    [dest[0], source[0], dest[1], source[1]]
}

/// `UNPCKHPS dest, source`: `(d2, s2, d3, s3)`.
pub(crate) fn unpckhps(dest: Vec4, source: Vec4) -> Vec4 {
    [dest[2], source[2], dest[3], source[3]]
}

/// `MOVLHPS dest, source`: `(d0, d1, s0, s1)`.
pub(crate) fn movlhps(dest: Vec4, source: Vec4) -> Vec4 {
    [dest[0], dest[1], source[0], source[1]]
}

/// `MOVHLPS dest, source`: `(s2, s3, d2, d3)`.
pub(crate) fn movhlps(dest: Vec4, source: Vec4) -> Vec4 {
    [source[2], source[3], dest[2], dest[3]]
}

/// `ANDPS dest, source`.
pub(crate) fn andps(dest: Vec4, source: Vec4) -> Vec4 {
    let (d, s) = (lane_bits(dest), lane_bits(source));
    from_lane_bits(std::array::from_fn(|lane| d[lane] & s[lane]))
}

/// `ANDNPS dest, source`: `(!dest) & source`.
pub(crate) fn andnps(dest: Vec4, source: Vec4) -> Vec4 {
    let (d, s) = (lane_bits(dest), lane_bits(source));
    from_lane_bits(std::array::from_fn(|lane| !d[lane] & s[lane]))
}

/// `ORPS dest, source`.
pub(crate) fn orps(dest: Vec4, source: Vec4) -> Vec4 {
    let (d, s) = (lane_bits(dest), lane_bits(source));
    from_lane_bits(std::array::from_fn(|lane| d[lane] | s[lane]))
}

/// `MINPS dest, source`: per lane `dest` when it is below `source`, otherwise `source` (so a NaN
/// in either operand gives `source`).
pub(crate) fn minps(dest: Vec4, source: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        if dest[lane] < source[lane] {
            dest[lane]
        } else {
            source[lane]
        }
    })
}

/// `MAXPS dest, source`: per lane `dest` when it is above `source`, otherwise `source`.
pub(crate) fn maxps(dest: Vec4, source: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        if dest[lane] > source[lane] {
            dest[lane]
        } else {
            source[lane]
        }
    })
}

/// `RCPPS`: the host CPU's per-lane reciprocal estimate (the estimate differs between Intel and
/// AMD parts, so the translations that use it are tested to one ULP; off x86-64 the exact
/// reciprocal stands in).
#[cfg(target_arch = "x86_64")]
pub(crate) fn rcpps(v: Vec4) -> Vec4 {
    use std::arch::x86_64::{_mm_loadu_ps, _mm_rcp_ps, _mm_storeu_ps};
    let mut out = [0.0f32; 4];
    // SAFETY: SSE is part of the x86-64 baseline and both arrays hold four floats.
    unsafe { _mm_storeu_ps(out.as_mut_ptr(), _mm_rcp_ps(_mm_loadu_ps(v.as_ptr()))) };
    out
}

/// `RCPPS` (not x86-64): the exact reciprocal.
#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn rcpps(v: Vec4) -> Vec4 {
    v.map(|lane| 1.0 / lane)
}

/// `CVTDQ2PS`: four signed integers to floats.
pub(crate) fn cvtdq2ps(v: [u32; 4]) -> Vec4 {
    v.map(|lane| lane as i32 as f32)
}

/// `PUNPCKLWD dest, source`: the low four 16-bit words of both, interleaved (`dest` first).
pub(crate) fn punpcklwd(dest: [u32; 4], source: [u32; 4]) -> [u32; 4] {
    let word = |v: [u32; 4], index: usize| (v[index / 2] >> (16 * (index % 2))) & 0xffff;
    std::array::from_fn(|lane| word(dest, lane) | (word(source, lane) << 16))
}

/// `PUNPCKHWD dest, source`: the high four 16-bit words of both, interleaved (`dest` first).
pub(crate) fn punpckhwd(dest: [u32; 4], source: [u32; 4]) -> [u32; 4] {
    let word = |v: [u32; 4], index: usize| (v[index / 2] >> (16 * (index % 2))) & 0xffff;
    std::array::from_fn(|lane| word(dest, lane + 4) | (word(source, lane + 4) << 16))
}

/// `PSLLD` by an immediate (a count above 31 clears the lanes).
pub(crate) fn pslld(v: [u32; 4], count: u32) -> [u32; 4] {
    v.map(|lane| lane.checked_shl(count).unwrap_or(0))
}

/// `PSRLD` by an immediate (a count above 31 clears the lanes).
pub(crate) fn psrld(v: [u32; 4], count: u32) -> [u32; 4] {
    v.map(|lane| lane.checked_shr(count).unwrap_or(0))
}

/// `DIVPS dest, source`.
pub(crate) fn divps(dest: Vec4, source: Vec4) -> Vec4 {
    std::array::from_fn(|lane| sse_lane(dest[lane], source[lane], |a, b| a / b))
}

/// `FISTP qword` with the rounding control set to truncate (`0xc00`): the 64-bit integer, or the
/// integer indefinite (`0x8000000000000000`) for a NaN or a value out of range.
pub(crate) fn fistp_truncate(value: f64) -> i64 {
    if value.is_nan()
        || value >= 9_223_372_036_854_775_808.0
        || value < -9_223_372_036_854_775_808.0
    {
        i64::MIN
    } else {
        value as i64
    }
}

/// The product the SSE loops of this unit compute for a column vector `v`: the three rotation
/// columns scaled by `v`'s x, y and z, `(x * c0 + y * c1) + z * c2`, in the exe's operand order
/// (the broadcast value is the destination of each `MULPS`).
fn combine_columns(columns: [Vec4; 3], v: Vec4) -> Vec4 {
    let by_x = mulps(shufps(v, v, 0x00), columns[0]);
    let by_y = mulps(shufps(v, v, 0x55), columns[1]);
    let sum = addps(by_x, by_y);
    let by_z = mulps(shufps(v, v, 0xaa), columns[2]);
    addps(sum, by_z)
}

/// The three rotation columns at `address`, `address + 0x10` and `address + 0x20`.
fn load_columns(e: &Engine, address: u32) -> [Vec4; 3] {
    [
        load_vec(e, address),
        load_vec(e, address.wrapping_add(0x10)),
        load_vec(e, address.wrapping_add(0x20)),
    ]
}

/// `hk1dBilateralConstraintBuildJacobian`-style callee of `00d6bf60` (`00dcf5b0`, cdecl
/// `(info, word, word, float)`).
const BUILD_JACOBIAN_FIRST: u32 = 0x00dc_f5b0;
/// `hk1dLinearBilateralConstraintBuildJacobian` (`00dcf280`, engine map name), cdecl
/// `(info, word, word)`.
const LINEAR_BUILD_JACOBIAN: u32 = 0x00dc_f280;
/// `hk1dAngularBilateralConstraintBuildJacobian` (`00dcee80`, engine map name), cdecl
/// `(info, word, word)`.
const ANGULAR_BUILD_JACOBIAN: u32 = 0x00dc_ee80;
/// `004b4ab0` (no unit in the engine map): `thiscall`, `this` and two vector pointers, `RET 8`.
const VECTOR_PAIR_OP: u32 = 0x004b_4ab0;
/// The 16-byte default vector at `01268370` the normal falls back to.
const DEFAULT_VECTOR: u32 = 0x0126_8370;
/// The distance below which `00d6bf60` treats two points as the same: `1.1920929e-07`.
const SAME_POINT_DISTANCE: u32 = 0x0104_da4c;
/// The sign mask `80000000` in every lane (`010c44d0`).
const SIGN_MASK: u32 = 0x010c_44d0;
/// A table of byte offsets `0x00, 0x10, 0x20` repeated (`010d33c0`): the vector index of the
/// next axis, so that `table[k + 1]` and `table[k + 2]` are the two axes after axis `k`.
const AXIS_OFFSETS: u32 = 0x010d_33c0;

// Translated from 00d6bf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the info block of a linear constraint between two points and calls `00dcf5b0` with it.
/// Register inputs first: `eax` and `ecx` are the two objects whose 16-byte vectors at `+0x30`
/// are the points; then the three stack words `first` (a pointer whose float at `+4` is the
/// rest length), `second` and `third` (passed on). The block (A, B, direction) is 0x40 bytes of
/// stack: A at `+0x10`, B at `+0x20`, the direction at `+0x30`. The direction is `(A - B)` scaled
/// by `1 / distance` when the distance (`SQRTSS` of the sum `z + (y + x)` of the squares) is
/// above `1.1920929e-07` (or NaN: the `FCOMP` test is "not below"), and otherwise the vector at
/// `01268370` with the distance taken as `0.0`. The fourth argument of the call is the float at
/// `first + 4` minus that distance.
pub fn fn_00d6bf60(e: &mut Engine, eax: u32, ecx: u32, first: u32, second: u32, third: u32) {
    let a = load_vec(e, eax.wrapping_add(0x30));
    let b = load_vec(e, ecx.wrapping_add(0x30));
    let difference = subps(a, b);
    let squares = mulps(difference, difference);
    let y_squared = shufps(squares, squares, 0x55);
    let z_squared = shufps(squares, squares, 0xaa);
    let partial = addss(y_squared, squares);
    let total = addss(z_squared, partial);
    // SQRTSS of lane 0: a NaN keeps its payload (quieted), anything else is the square root.
    let root = if total[0].is_nan() {
        sse_quiet(total[0])
    } else {
        total[0].sqrt()
    };
    // FLD / FST: a signalling NaN is quieted on the way through the x87 stack.
    let distance = x87_quiet(root);
    let epsilon = x87_load32(e.mem.u32(SAME_POINT_DISTANCE));
    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        store_vec(e, frame + 0x10, a);
        store_vec(e, frame + 0x20, b);
        // FCOMP: `C0` is set when the epsilon is below the distance, or the two are unordered.
        let distance_bits = if fcom_status(epsilon, f64::from(distance)) & 0x01 != 0 {
            store_vec(e, frame + 0x30, difference);
            let inverse = x87_store32(x87_div(1.0, f64::from(distance)));
            e.mem.set_u32(frame + 0xc, inverse);
            let scale = [f32::from_bits(e.mem.u32(frame + 0xc)), 0.0, 0.0, 0.0];
            let normal = mulps(shufps(scale, scale, 0x00), difference);
            store_vec(e, frame + 0x30, normal);
            distance.to_bits()
        } else {
            let fallback = load_vec(e, DEFAULT_VECTOR);
            store_vec(e, frame + 0x30, fallback);
            0.0f32.to_bits()
        };
        e.mem.set_u32(frame + 8, distance_bits);
        let rest_length = x87_load32(e.mem.u32(first.wrapping_add(4)));
        let remaining = x87_sub(rest_length, x87_m32(distance_bits));
        e.mem.set_u32(frame + 0xc, x87_store32(remaining));
        let argument = x87_quiet_bits(e.mem.u32(frame + 0xc));
        e.call(
            BUILD_JACOBIAN_FIRST,
            &args![frame + 0x10, second, third, f32::from_bits(argument)],
        );
    });
}

// Translated from 00d6c030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the info block of a linear constraint along an axis and calls
/// `hk1dLinearBilateralConstraintBuildJacobian` (`00dcf280`) with it. Register inputs first:
/// `ecx` and `eax` are two transforms (the translation vectors at `+0x30` are the pivots, and
/// the axis is the vector `eax + 0x10 * k`, `k` the byte at `index_holder + 2`); then the three
/// stack words `index_holder`, `second` and `third` (the last two are passed on). The block is
/// 0x30 bytes of stack: the `ecx` pivot, the `eax` pivot and the axis.
pub fn fn_00d6c030(e: &mut Engine, ecx: u32, eax: u32, index_holder: u32, second: u32, third: u32) {
    let pivot_first = load_vec(e, ecx.wrapping_add(0x30));
    let k = u32::from(e.mem.u8(index_holder.wrapping_add(2)));
    let pivot_second = load_vec(e, eax.wrapping_add(0x30));
    let axis = load_vec(e, (k << 4).wrapping_add(eax));
    e.with_stack(0x30, |e, frame| {
        let frame = frame.addr();
        store_vec(e, frame, pivot_first);
        store_vec(e, frame + 0x10, pivot_second);
        store_vec(e, frame + 0x20, axis);
        e.call(LINEAR_BUILD_JACOBIAN, &args![frame, second, third]);
    });
}

// Translated from 00d6c080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register inputs first: `ecx`, `edx` and `edi` (the output vector); then the stack words
/// `first`, `second` (pointers to two objects), `numerator_scale` and `scale` (floats).
/// Computes `s = (1 / numerator_scale) * scale` on the x87 stack and a vector
/// `out = (cross1 * s + d) - cross2 * s` where `d = [ecx] - [edx]`, `cross1` and `cross2` are the
/// `SHUFPS` cross products of `q = [ecx] - [second + 0x40]` with the result of `004b4ab0` on
/// `second`'s vectors at `+0x50` and `+0x20`, and of `z = [edx] - [first + 0x40]` with the result
/// of `004b4ab0` on `first`'s; `d` is also stored to `edi` before the two calls. Nothing is
/// returned.
#[allow(clippy::too_many_arguments)]
pub fn fn_00d6c080(
    e: &mut Engine,
    ecx: u32,
    edx: u32,
    edi: u32,
    first: u32,
    second: u32,
    numerator_scale: f32,
    scale: f32,
) {
    let edx_vector = load_vec(e, edx);
    let reciprocal = x87_div(1.0, x87_load32(numerator_scale.to_bits()));
    let scaled = x87_mul(reciprocal, x87_m32(scale.to_bits()));
    // The frame the exe addresses from its stack pointer after `PUSH ESI`: 0x70 bytes.
    e.with_stack(0x70, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0xc, x87_store32(scaled));
        let scalar = [e.mem.f32(frame + 0xc), 0.0, 0.0, 0.0];
        store_vec(e, frame + 0x10, scalar);
        let delta = subps(load_vec(e, ecx), edx_vector);
        store_vec(e, edi, delta);
        let second_axis = load_vec(e, first.wrapping_add(0x40));
        store_vec(e, frame + 0x40, delta);
        let between = subps(load_vec(e, edx), second_axis);
        let third_axis = load_vec(e, second.wrapping_add(0x40));
        store_vec(e, frame + 0x60, between);
        let from_ecx = subps(load_vec(e, ecx), third_axis);
        store_vec(e, frame + 0x30, from_ecx);
        e.call(
            VECTOR_PAIR_OP,
            &args![
                frame + 0x50,
                first.wrapping_add(0x50),
                first.wrapping_add(0x20)
            ],
        );
        e.call(
            VECTOR_PAIR_OP,
            &args![
                frame + 0x20,
                second.wrapping_add(0x50),
                second.wrapping_add(0x20)
            ],
        );
        let second_result = load_vec(e, frame + 0x20);
        let scalar = load_vec(e, frame + 0x10);
        let factor = shufps(scalar, scalar, 0x00);
        let q = load_vec(e, frame + 0x30);
        let q_yzx = shufps(q, q, 0xd2);
        let mut product = mulps(shufps(second_result, second_result, 0xc9), q_yzx);
        let mut other = shufps(second_result, second_result, 0xd2);
        let first_result = load_vec(e, frame + 0x50);
        let q_zxy = shufps(q, q, 0xc9);
        other = mulps(other, q_zxy);
        let z = load_vec(e, frame + 0x60);
        other = subps(other, product);
        let z_yzx = shufps(z, z, 0xd2);
        product = mulps(shufps(first_result, first_result, 0xc9), z_yzx);
        let mut other_two = shufps(first_result, first_result, 0xd2);
        let z_zxy = shufps(z, z, 0xc9);
        other_two = mulps(other_two, z_zxy);
        other = mulps(other, factor);
        let kept = load_vec(e, frame + 0x40);
        other = addps(other, kept);
        other_two = subps(other_two, product);
        other_two = mulps(other_two, factor);
        other = subps(other, other_two);
        store_vec(e, edi, other);
    });
}

// Translated from 00d6c1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A product of two transforms (three 16-byte columns and a 16-byte translation each). Register
/// inputs first: `esi` (left, whose columns are scaled), `edi` (right); then the stack word
/// `result`. Each of `edi`'s four vectors, from the translation down to the first column, goes
/// through the combination of `esi`'s three columns by its x, y and z (the order is
/// `(x * c0 + y * c1) + z * c2`) and is stored at the same offset of `result`; at the end
/// `esi`'s translation is added to `result`'s translation (`esi + 0x30` is the destination of
/// the `ADDPS`).
pub fn fn_00d6c1d0(e: &mut Engine, esi: u32, edi: u32, result: u32) {
    let columns = load_columns(e, esi);
    let mut source = edi.wrapping_add(0x30);
    let delta = result.wrapping_sub(edi);
    for _ in 0..4 {
        let v = load_vec(e, source);
        let out = combine_columns(columns, v);
        store_vec(e, delta.wrapping_add(source), out);
        source = source.wrapping_sub(0x10);
    }
    let translation = load_vec(e, result.wrapping_add(0x30));
    let left_translation = load_vec(e, esi.wrapping_add(0x30));
    store_vec(
        e,
        result.wrapping_add(0x30),
        addps(left_translation, translation),
    );
}

// Translated from 00d6c240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Two transforms of four vectors each, through the rotation columns the object `object`
/// points to. Register input first: `ecx` (the first destination); then the stack words
/// `source`, `object` and `second_destination`. The vectors at `source + 0x40` down to
/// `source + 0x10` go through the columns at `[object + 0x38]` into `ecx + 0x30` down to `ecx`,
/// and that destination's translation gets the columns' translation added (`columns + 0x30` is
/// the destination of the `ADDPS`); then the vectors at `source + 0x80` down to `source + 0x50`
/// go through the columns at `[object + 0x3c]` into `second_destination + 0x30` down to
/// `second_destination`, with the same addition.
pub fn fn_00d6c240(e: &mut Engine, ecx: u32, source: u32, object: u32, second_destination: u32) {
    let columns_address = e.mem.u32(object.wrapping_add(0x38));
    let columns = load_columns(e, columns_address);
    let destination_translation = ecx.wrapping_add(0x30);
    let mut destination = destination_translation;
    let mut read = source.wrapping_add(0x40);
    for _ in 0..4 {
        let v = load_vec(e, read);
        store_vec(e, destination, combine_columns(columns, v));
        read = read.wrapping_sub(0x10);
        destination = destination.wrapping_sub(0x10);
    }
    let translation = load_vec(e, destination_translation);
    let columns_translation = load_vec(e, columns_address.wrapping_add(0x30));
    store_vec(
        e,
        destination_translation,
        addps(columns_translation, translation),
    );

    let columns_address = e.mem.u32(object.wrapping_add(0x3c));
    let columns = load_columns(e, columns_address);
    let destination_translation = second_destination.wrapping_add(0x30);
    let mut destination = destination_translation;
    let mut read = source.wrapping_add(0x80);
    for _ in 0..4 {
        let v = load_vec(e, read);
        store_vec(e, destination, combine_columns(columns, v));
        read = read.wrapping_sub(0x10);
        destination = destination.wrapping_sub(0x10);
    }
    let translation = load_vec(e, destination_translation);
    let columns_translation = load_vec(e, columns_address.wrapping_add(0x30));
    store_vec(
        e,
        destination_translation,
        addps(columns_translation, translation),
    );
}

/// One point through the columns and translation at `columns_address` in the order of
/// `00d6c320`: `(y * c1 + x * c0) + (z * c2 + translation)`.
fn transform_point(e: &Engine, columns_address: u32, v: Vec4) -> Vec4 {
    let c1 = load_vec(e, columns_address.wrapping_add(0x10));
    let c0 = load_vec(e, columns_address);
    let by_y = mulps(shufps(v, v, 0x55), c1);
    let by_x = mulps(shufps(v, v, 0x00), c0);
    let sum = addps(by_y, by_x);
    let c2 = load_vec(e, columns_address.wrapping_add(0x20));
    let by_z = mulps(shufps(v, v, 0xaa), c2);
    let translation = load_vec(e, columns_address.wrapping_add(0x30));
    let tail = addps(by_z, translation);
    addps(sum, tail)
}

// Translated from 00d6c320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register inputs first: `ecx` (an object whose `+0x38` and `+0x3c` point to transforms) and
/// `edx` (a block of vectors); then the stack words `first_destination` and
/// `second_destination`. The vector at `edx + 0x10` goes through the transform at
/// `[ecx + 0x38]` (columns scaled by x, y, z, plus the translation) into the translation slot
/// `first_destination + 0x30`, and the vector at `edx + 0x20` through the one at `[ecx + 0x3c]`
/// into `second_destination + 0x30`.
pub fn fn_00d6c320(
    e: &mut Engine,
    ecx: u32,
    edx: u32,
    first_destination: u32,
    second_destination: u32,
) {
    let v = load_vec(e, edx.wrapping_add(0x10));
    let transform = e.mem.u32(ecx.wrapping_add(0x38));
    let result = transform_point(e, transform, v);
    store_vec(e, first_destination.wrapping_add(0x30), result);
    let v = load_vec(e, edx.wrapping_add(0x20));
    let transform = e.mem.u32(ecx.wrapping_add(0x3c));
    let result = transform_point(e, transform, v);
    store_vec(e, second_destination.wrapping_add(0x30), result);
}

// Translated from 00d6c3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Register inputs first: `esi` (an object whose `+0x38` and `+0x3c` point to rotation
/// columns), `edx` (the first destination) and `edi` (the source); then the stack word
/// `second_destination`. The vectors at `edi + 0x40` down to `edi + 0x10` go through the
/// columns at `[esi + 0x38]` into `edx + 0x30` down to `edx`; then those at `edi + 0x70` down to
/// `edi + 0x40` go through the columns at `[esi + 0x3c]` (read after the first pass) into
/// `second_destination + 0x30` down to `second_destination`.
pub fn fn_00d6c3c0(e: &mut Engine, esi: u32, edx: u32, edi: u32, second_destination: u32) {
    let columns_address = e.mem.u32(esi.wrapping_add(0x38));
    let columns = load_columns(e, columns_address);
    let mut destination = edx.wrapping_add(0x30);
    let mut read = edi.wrapping_add(0x40);
    for _ in 0..4 {
        let v = load_vec(e, read);
        store_vec(e, destination, combine_columns(columns, v));
        read = read.wrapping_sub(0x10);
        destination = destination.wrapping_sub(0x10);
    }
    let columns_address = e.mem.u32(esi.wrapping_add(0x3c));
    let columns = load_columns(e, columns_address);
    let mut destination = second_destination.wrapping_add(0x30);
    let mut read = edi.wrapping_add(0x70);
    for _ in 0..4 {
        let v = load_vec(e, read);
        store_vec(e, destination, combine_columns(columns, v));
        read = read.wrapping_sub(0x10);
        destination = destination.wrapping_sub(0x10);
    }
}

// Translated from 00d6c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the info block of an angular constraint and calls
/// `hk1dAngularBilateralConstraintBuildJacobian` (`00dcee80`) twice. Register inputs first:
/// `eax` (a pointer whose byte at `+2` is the axis `k`), `ecx` (the first transform) and `esi` (the
/// second); then the stack words `first` and `second` (passed on). With `a = table[k + 1]` and
/// `b = table[k + 2]` the byte offsets of the next two axes (`010d33c0`), the 0x38-byte stack block
/// holds, from `+8`, the vector `ecx + 0x10 * k` (kept for both calls), then `esi + b` and
/// `esi + a` for the first call, and `esi + a` with every sign flipped (`XORPS` with `010c44d0`)
/// and `esi + b` for the second.
pub fn fn_00d6c470(e: &mut Engine, eax: u32, ecx: u32, esi: u32, first: u32, second: u32) {
    let k = u32::from(e.mem.u8(eax.wrapping_add(2)));
    let b = u32::from(e.mem.u8(AXIS_OFFSETS + 2 + k));
    let a = u32::from(e.mem.u8(AXIS_OFFSETS + 1 + k));
    let own_axis = load_vec(e, (k << 4).wrapping_add(ecx));
    e.with_stack(0x38, |e, frame| {
        let frame = frame.addr();
        store_vec(e, frame + 8, own_axis);
        let next = load_vec(e, a.wrapping_add(esi));
        store_vec(e, frame + 0x28, next);
        let after = load_vec(e, b.wrapping_add(esi));
        store_vec(e, frame + 0x18, after);
        e.call(ANGULAR_BUILD_JACOBIAN, &args![frame + 8, first, second]);
        let after = load_vec(e, b.wrapping_add(esi));
        let next = load_vec(e, a.wrapping_add(esi));
        store_vec(e, frame + 0x28, after);
        let mask: Vec4 = std::array::from_fn(|lane| e.mem.f32(SIGN_MASK + 4 * lane as u32));
        let flipped: Vec4 =
            std::array::from_fn(|lane| f32::from_bits(mask[lane].to_bits() ^ next[lane].to_bits()));
        store_vec(e, frame + 0x18, flipped);
        e.call(ANGULAR_BUILD_JACOBIAN, &args![frame + 8, first, second]);
    });
}

// Translated from 00d6c500 (decompiled, FalloutNV.exe 1.4.0.525)
/// For each axis index `k` from the byte at `edi + 2` up to (not including) that byte plus the
/// byte at `edi + 3` (the end is read again every time), builds the info block of an angular
/// constraint and calls `hk1dAngularBilateralConstraintBuildJacobian` (`00dcee80`). Register
/// input first: `edi`; then the stack words `first` (passed on), `base_first`, `base_second` and
/// `third` (passed on). The 0x38-byte stack block holds from `+8` the vector at
/// `base_first + table[k + 1]`, then at `+0x18` `base_first + table[k]` and at `+0x28`
/// `base_second + table[k + 2]` (the offsets of `010d33c0`).
pub fn fn_00d6c500(
    e: &mut Engine,
    edi: u32,
    first: u32,
    base_first: u32,
    base_second: u32,
    third: u32,
) {
    let count = u32::from(e.mem.u8(edi.wrapping_add(3)));
    let mut k = u32::from(e.mem.u8(edi.wrapping_add(2)));
    let end = count.wrapping_add(k);
    if (k as i32) >= (end as i32) {
        return;
    }
    e.with_stack(0x38, |e, frame| {
        let frame = frame.addr();
        loop {
            let offset_zero = u32::from(e.mem.u8(AXIS_OFFSETS + k));
            let vector_zero = load_vec(e, offset_zero.wrapping_add(base_first));
            let offset_one = u32::from(e.mem.u8(AXIS_OFFSETS + 1 + k));
            let offset_two = u32::from(e.mem.u8(AXIS_OFFSETS + 2 + k));
            store_vec(e, frame + 0x18, vector_zero);
            let vector_one = load_vec(e, offset_one.wrapping_add(base_first));
            store_vec(e, frame + 8, vector_one);
            let vector_two = load_vec(e, offset_two.wrapping_add(base_second));
            store_vec(e, frame + 0x28, vector_two);
            e.call(ANGULAR_BUILD_JACOBIAN, &args![frame + 8, first, third]);
            let end = u32::from(e.mem.u8(edi.wrapping_add(3)))
                .wrapping_add(u32::from(e.mem.u8(edi.wrapping_add(2))));
            k = k.wrapping_add(1);
            if (k as i32) >= (end as i32) {
                break;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_bf60, fn_00d6bf60(u32, u32, u32, u32, u32)),
        entry!(0x00d6_c030, fn_00d6c030(u32, u32, u32, u32, u32)),
        entry!(0x00d6_c080, fn_00d6c080(u32, u32, u32, u32, u32, f32, f32)),
        entry!(0x00d6_c1d0, fn_00d6c1d0(u32, u32, u32)),
        entry!(0x00d6_c240, fn_00d6c240(u32, u32, u32, u32)),
        entry!(0x00d6_c320, fn_00d6c320(u32, u32, u32, u32)),
        entry!(0x00d6_c3c0, fn_00d6c3c0(u32, u32, u32, u32)),
        entry!(0x00d6_c470, fn_00d6c470(u32, u32, u32, u32, u32)),
        entry!(0x00d6_c500, fn_00d6c500(u32, u32, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod a09_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What a double of a build-jacobian callee saw: its argument words and the three vectors
    /// at its first argument.
    type Seen = Rc<RefCell<Vec<(Vec<u32>, [Vec4; 3])>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e
    }

    fn put(e: &mut Engine, address: u32, vectors: &[Vec4]) {
        for (index, vector) in vectors.iter().enumerate() {
            store_vec(e, address + 16 * index as u32, *vector);
        }
    }

    /// A zeroed block of `vectors` 16-byte vectors with `contents` at its start.
    fn block(e: &mut Engine, vectors: u32, contents: &[Vec4]) -> u32 {
        let address = e.mem.alloc(vectors * 16);
        put(e, address, contents);
        address
    }

    fn record(e: &mut Engine, callee: u32) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(callee, move |e, a| {
            let at = a[0];
            let frame = [
                load_vec(e, at),
                load_vec(e, at + 0x10),
                load_vec(e, at + 0x20),
            ];
            log.borrow_mut().push((a.to_vec(), frame));
            Ret::default()
        });
        seen
    }

    /// The expected combination of the three columns by `v`'s first three lanes.
    fn rotate(columns: [Vec4; 3], v: Vec4) -> Vec4 {
        std::array::from_fn(|lane| {
            (v[0] * columns[0][lane] + v[1] * columns[1][lane]) + v[2] * columns[2][lane]
        })
    }

    const COLUMNS: [Vec4; 3] = [
        [1.0, 2.0, 3.0, 4.0],
        [0.5, -1.0, 2.0, 8.0],
        [3.0, 0.25, -2.0, 1.0],
    ];

    #[test]
    fn shufps_picks_destination_then_source_lanes() {
        let a = [1.0, 2.0, 3.0, 4.0];
        let b = [5.0, 6.0, 7.0, 8.0];
        assert_eq!(shufps(a, b, 0xd2), [3.0, 1.0, 6.0, 8.0]);
        assert_eq!(shufps(a, a, 0x55), [2.0; 4]);
    }

    #[test]
    fn sse_arithmetic_keeps_the_destination_nan_first() {
        let first = f32::from_bits(0x7fc0_0001);
        let second = f32::from_bits(0x7fc0_0002);
        let signalling = f32::from_bits(0x7f80_0003);
        assert_eq!(
            mulps([first, 1.0, second, 2.0], [second, signalling, 2.0, 3.0])[0].to_bits(),
            0x7fc0_0001
        );
        let r = addps([1.0, 1.0, second, 2.0], [second, signalling, 2.0, 3.0]);
        assert_eq!(r[0].to_bits(), 0x7fc0_0002);
        assert_eq!(r[1].to_bits(), 0x7fc0_0003);
        assert_eq!(r[2].to_bits(), 0x7fc0_0002);
        assert_eq!(r[3], 5.0);
        let s = addss([signalling, 7.0, 8.0, 9.0], [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(
            (s[0].to_bits(), s[1], s[2], s[3]),
            (0x7fc0_0003, 7.0, 8.0, 9.0)
        );
        assert_eq!(subps([5.0; 4], [1.0, 2.0, 3.0, 4.0]), [4.0, 3.0, 2.0, 1.0]);
    }

    #[test]
    fn fcom_status_is_the_status_word_high_byte() {
        assert_eq!(fcom_status(2.0, 1.0), 0x00);
        assert_eq!(fcom_status(1.0, 2.0), 0x01);
        assert_eq!(fcom_status(1.0, 1.0), 0x40);
        assert_eq!(fcom_status(f64::NAN, 1.0), 0x45);
    }

    /// Runs `00d6bf60` on points `a` and `b` with a rest length of 7 and returns what the
    /// callee saw.
    fn run_00d6bf60(e: &mut Engine, a: Vec4, b: Vec4) -> (Vec<u32>, [Vec4; 3]) {
        let seen = record(e, BUILD_JACOBIAN_FIRST);
        let object_a = block(e, 4, &[[0.0; 4], [0.0; 4], [0.0; 4], a]);
        let object_b = block(e, 4, &[[0.0; 4], [0.0; 4], [0.0; 4], b]);
        let holder = e.mem.alloc(8);
        e.mem.set_f32(holder + 4, 7.0);
        e.call(
            0x00d6_bf60,
            &args![object_a, object_b, holder, 0x1111u32, 0x2222u32],
        );
        let calls = seen.borrow();
        assert_eq!(calls.len(), 1);
        calls[0].clone()
    }

    #[test]
    fn linear_info_normalises_the_difference_of_the_points() {
        let mut e = engine();
        let (arguments, frame) = run_00d6bf60(&mut e, [3.0, 5.0, 4.0, 1.0], [0.0, 5.0, 0.0, 0.5]);
        // The distance is 5, so the direction is the difference times 0.2.
        let inverse = 0.2f32;
        assert_eq!(frame[0], [3.0, 5.0, 4.0, 1.0]);
        assert_eq!(frame[1], [0.0, 5.0, 0.0, 0.5]);
        assert_eq!(
            frame[2],
            [inverse * 3.0, inverse * 0.0, inverse * 4.0, inverse * 0.5]
        );
        assert_eq!(arguments[1..], [0x1111, 0x2222, 2.0f32.to_bits()]);
    }

    #[test]
    fn linear_info_of_coincident_points_takes_the_default_direction() {
        let mut e = engine();
        put(&mut e, DEFAULT_VECTOR, &[[5.0, 6.0, 7.0, 8.0]]);
        let (arguments, frame) = run_00d6bf60(&mut e, [1.0, 2.0, 3.0, 4.0], [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(frame[2], [5.0, 6.0, 7.0, 8.0]);
        // The distance is taken as zero: the argument is the rest length itself.
        assert_eq!(arguments[3], 7.0f32.to_bits());
    }

    #[test]
    fn linear_info_treats_a_nan_distance_as_far() {
        let mut e = engine();
        put(&mut e, DEFAULT_VECTOR, &[[5.0, 6.0, 7.0, 8.0]]);
        let (arguments, frame) = run_00d6bf60(&mut e, [f32::NAN, 2.0, 3.0, 4.0], [0.0; 4]);
        assert!(frame[2][0].is_nan());
        assert_ne!(frame[2], [5.0, 6.0, 7.0, 8.0]);
        assert!(f32::from_bits(arguments[3]).is_nan());
    }

    #[test]
    fn linear_info_just_above_the_threshold_still_normalises() {
        let mut e = engine();
        let (_, frame) = run_00d6bf60(&mut e, [1.0e-6, 0.0, 0.0, 0.0], [0.0; 4]);
        assert!(frame[2][0] > 0.99 && frame[2][0] < 1.01);
    }

    #[test]
    fn axis_info_takes_the_pivots_and_the_axis_the_index_byte_picks() {
        let mut e = engine();
        let seen = record(&mut e, LINEAR_BUILD_JACOBIAN);
        let first = block(
            &mut e,
            4,
            &[[0.0; 4], [0.0; 4], [0.0; 4], [1.0, 2.0, 3.0, 4.0]],
        );
        let second = block(
            &mut e,
            4,
            &[[9.0; 4], [8.0; 4], [7.0; 4], [5.0, 6.0, 7.0, 8.0]],
        );
        let holder = e.mem.alloc(8);
        e.mem.set_u8(holder + 2, 1);
        e.call(
            0x00d6_c030,
            &args![first, second, holder, 0xaaaau32, 0xbbbbu32],
        );
        let calls = seen.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0[1..], [0xaaaa, 0xbbbb]);
        assert_eq!(
            calls[0].1,
            [[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [8.0; 4]]
        );
    }

    #[test]
    fn cross_product_builder_combines_the_two_crosses_with_the_scale() {
        let mut e = engine();
        let ecx = block(&mut e, 1, &[[1.0, 2.0, 3.0, 4.0]]);
        let edx = block(&mut e, 1, &[[0.5, 0.25, 2.0, 1.0]]);
        let edi = block(&mut e, 1, &[]);
        let first = block(&mut e, 6, &[]);
        let second = block(&mut e, 6, &[]);
        put(&mut e, first + 0x40, &[[1.0, 1.0, 1.0, 1.0]]);
        put(&mut e, second + 0x40, &[[2.0, 0.0, 1.0, 3.0]]);
        let results: [Vec4; 2] = [[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]];
        let count = Rc::new(RefCell::new(0usize));
        let seen = Rc::new(RefCell::new(Vec::new()));
        let (counter, log) = (Rc::clone(&count), Rc::clone(&seen));
        e.register_double(VECTOR_PAIR_OP, move |e, a| {
            let mut n = counter.borrow_mut();
            store_vec(e, a[0], results[*n]);
            log.borrow_mut().push(a.to_vec());
            *n += 1;
            Ret::default()
        });
        e.call(
            0x00d6_c080,
            &args![ecx, edx, edi, first, second, 2.0f32, 3.0f32],
        );
        assert_eq!(
            *seen.borrow(),
            [
                vec![seen.borrow()[0][0], first + 0x50, first + 0x20],
                vec![seen.borrow()[1][0], second + 0x50, second + 0x20]
            ]
        );
        let cross = |a: Vec4, b: Vec4| -> Vec4 {
            [
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
                0.0,
            ]
        };
        let delta = subps([1.0, 2.0, 3.0, 4.0], [0.5, 0.25, 2.0, 1.0]);
        let q = subps([1.0, 2.0, 3.0, 4.0], [2.0, 0.0, 1.0, 3.0]);
        let z = subps([0.5, 0.25, 2.0, 1.0], [1.0, 1.0, 1.0, 1.0]);
        let scale = 1.5f32;
        let first_cross = cross(q, results[1]);
        let second_cross = cross(z, results[0]);
        let expected: Vec4 = std::array::from_fn(|lane| {
            (first_cross[lane] * scale + delta[lane]) - second_cross[lane] * scale
        });
        assert_eq!(load_vec(&e, edi), expected);
        assert_eq!(expected[3], delta[3]);
    }

    #[test]
    fn transform_product_scales_the_columns_and_adds_the_translation() {
        let mut e = engine();
        let left = block(
            &mut e,
            4,
            &[COLUMNS[0], COLUMNS[1], COLUMNS[2], [10.0, 20.0, 30.0, 40.0]],
        );
        let vectors: [Vec4; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [1.0, 2.0, 3.0, 0.0],
            [0.5, 0.25, 2.0, 1.0],
        ];
        let right = block(&mut e, 4, &vectors);
        let result = block(&mut e, 4, &[]);
        e.call(0x00d6_c1d0, &args![left, right, result]);
        for (index, vector) in vectors.iter().enumerate().take(3) {
            assert_eq!(
                load_vec(&e, result + 16 * index as u32),
                rotate(COLUMNS, *vector)
            );
        }
        let moved = rotate(COLUMNS, vectors[3]);
        let expected: Vec4 =
            std::array::from_fn(|lane| [10.0, 20.0, 30.0, 40.0][lane] + moved[lane]);
        assert_eq!(load_vec(&e, result + 0x30), expected);
    }

    #[test]
    fn transform_product_in_place_reads_each_vector_before_it_is_overwritten() {
        let mut e = engine();
        let left = block(&mut e, 4, &[COLUMNS[0], COLUMNS[1], COLUMNS[2], [0.0; 4]]);
        let vectors: [Vec4; 4] = [
            [1.0, 0.0, 2.0, 0.0],
            [0.0, 1.0, 1.0, 0.0],
            [1.0, 2.0, 3.0, 0.0],
            [0.5, 0.25, 2.0, 1.0],
        ];
        let right = block(&mut e, 4, &vectors);
        e.call(0x00d6_c1d0, &args![left, right, right]);
        for (index, vector) in vectors.iter().enumerate() {
            assert_eq!(
                load_vec(&e, right + 16 * index as u32),
                rotate(COLUMNS, *vector)
            );
        }
    }

    /// An object whose `+0x38` and `+0x3c` point to two transforms (columns and translation).
    fn two_transforms(e: &mut Engine) -> (u32, [Vec4; 4], [Vec4; 4]) {
        let first_columns: [Vec4; 4] = [
            COLUMNS[0],
            COLUMNS[1],
            COLUMNS[2],
            [100.0, 200.0, 300.0, 400.0],
        ];
        let second_columns: [Vec4; 4] = [
            [2.0, 0.0, 0.0, 1.0],
            [0.0, 3.0, 0.0, 2.0],
            [0.0, 0.0, 4.0, 3.0],
            [1.0, 1.0, 1.0, 1.0],
        ];
        let object = e.mem.alloc(0x40);
        let first = block(e, 4, &first_columns);
        let second = block(e, 4, &second_columns);
        e.mem.set_u32(object + 0x38, first);
        e.mem.set_u32(object + 0x3c, second);
        (object, first_columns, second_columns)
    }

    fn source_vectors() -> [Vec4; 9] {
        std::array::from_fn(|i| [i as f32 + 1.0, 0.5 * i as f32, 2.0 - i as f32, 1.0])
    }

    #[test]
    fn two_transforms_of_four_vectors_each_with_translations_added() {
        let mut e = engine();
        let (object, first, second) = two_transforms(&mut e);
        let vectors = source_vectors();
        let source = block(&mut e, 9, &vectors);
        let out_first = block(&mut e, 4, &[]);
        let out_second = block(&mut e, 4, &[]);
        e.call(0x00d6_c240, &args![out_first, source, object, out_second]);
        let columns = [first[0], first[1], first[2]];
        for slot in 0..4 {
            let mut expected = rotate(columns, vectors[slot + 1]);
            if slot == 3 {
                expected = std::array::from_fn(|lane| first[3][lane] + expected[lane]);
            }
            assert_eq!(load_vec(&e, out_first + 16 * slot as u32), expected);
        }
        let columns = [second[0], second[1], second[2]];
        for slot in 0..4 {
            let mut expected = rotate(columns, vectors[slot + 5]);
            if slot == 3 {
                expected = std::array::from_fn(|lane| second[3][lane] + expected[lane]);
            }
            assert_eq!(load_vec(&e, out_second + 16 * slot as u32), expected);
        }
    }

    #[test]
    fn point_transform_scales_adds_the_translation_and_stores_at_plus_0x30() {
        let mut e = engine();
        let (object, first, second) = two_transforms(&mut e);
        let vectors = source_vectors();
        let source = block(&mut e, 3, &vectors[..3]);
        let out_first = block(&mut e, 4, &[]);
        let out_second = block(&mut e, 4, &[]);
        e.call(0x00d6_c320, &args![object, source, out_first, out_second]);
        let point = |t: [Vec4; 4], v: Vec4| -> Vec4 {
            std::array::from_fn(|lane| {
                (v[1] * t[1][lane] + v[0] * t[0][lane]) + (v[2] * t[2][lane] + t[3][lane])
            })
        };
        assert_eq!(load_vec(&e, out_first + 0x30), point(first, vectors[1]));
        assert_eq!(load_vec(&e, out_second + 0x30), point(second, vectors[2]));
        assert_eq!(load_vec(&e, out_first), [0.0; 4]);
    }

    #[test]
    fn two_column_passes_read_the_second_pointer_after_the_first_pass() {
        let mut e = engine();
        let (object, first, second) = two_transforms(&mut e);
        let vectors = source_vectors();
        let source = block(&mut e, 8, &vectors[..8]);
        let out_first = block(&mut e, 4, &[]);
        let out_second = block(&mut e, 4, &[]);
        e.call(0x00d6_c3c0, &args![object, out_first, source, out_second]);
        for slot in 0..4 {
            assert_eq!(
                load_vec(&e, out_first + 16 * slot as u32),
                rotate([first[0], first[1], first[2]], vectors[slot + 1])
            );
            assert_eq!(
                load_vec(&e, out_second + 16 * slot as u32),
                rotate([second[0], second[1], second[2]], vectors[slot + 4])
            );
        }
    }

    fn put_axis_table(e: &mut Engine) {
        for (index, byte) in [
            0u8, 0x10, 0x20, 0, 0x10, 0x20, 0, 0x10, 0x20, 0, 0, 0, 0, 0, 0, 0,
        ]
        .iter()
        .enumerate()
        {
            e.mem.set_u8(AXIS_OFFSETS + index as u32, *byte);
        }
        e.set_global(SIGN_MASK, 0x8000_0000u32);
        e.set_global(SIGN_MASK + 4, 0x8000_0000u32);
        e.set_global(SIGN_MASK + 8, 0x8000_0000u32);
        e.set_global(SIGN_MASK + 12, 0x8000_0000u32);
    }

    #[test]
    fn angular_info_calls_twice_with_the_next_axes_and_the_flipped_one() {
        let mut e = engine();
        put_axis_table(&mut e);
        let seen = record(&mut e, ANGULAR_BUILD_JACOBIAN);
        let holder = e.mem.alloc(8);
        e.mem.set_u8(holder + 2, 1);
        let own = block(&mut e, 3, &[[0.0; 4], [1.0, 2.0, 3.0, 4.0], [0.0; 4]]);
        let other = block(
            &mut e,
            3,
            &[
                [10.0, 11.0, 12.0, 13.0],
                [20.0, 21.0, 22.0, 23.0],
                [30.0, 31.0, 32.0, 33.0],
            ],
        );
        e.call(0x00d6_c470, &args![holder, own, other, 0x77u32, 0x88u32]);
        let calls = seen.borrow();
        assert_eq!(calls.len(), 2);
        // k = 1: the next axes are table[2] = 0x20 and table[3] = 0.
        assert_eq!(
            calls[0].1,
            [
                [1.0, 2.0, 3.0, 4.0],
                [10.0, 11.0, 12.0, 13.0],
                [30.0, 31.0, 32.0, 33.0]
            ]
        );
        assert_eq!(
            calls[1].1,
            [
                [1.0, 2.0, 3.0, 4.0],
                [-30.0, -31.0, -32.0, -33.0],
                [10.0, 11.0, 12.0, 13.0]
            ]
        );
        assert_eq!(calls[0].0[1..], [0x77, 0x88]);
        assert_eq!(calls[1].0[0], calls[0].0[0]);
    }

    #[test]
    fn angular_loop_runs_once_per_axis_and_not_at_all_for_a_zero_count() {
        let mut e = engine();
        put_axis_table(&mut e);
        let seen = record(&mut e, ANGULAR_BUILD_JACOBIAN);
        let holder = e.mem.alloc(8);
        let base_first = block(
            &mut e,
            3,
            &[
                [1.0, 1.0, 1.0, 1.0],
                [2.0, 2.0, 2.0, 2.0],
                [3.0, 3.0, 3.0, 3.0],
            ],
        );
        let base_second = block(
            &mut e,
            3,
            &[
                [4.0, 4.0, 4.0, 4.0],
                [5.0, 5.0, 5.0, 5.0],
                [6.0, 6.0, 6.0, 6.0],
            ],
        );
        // Count 0: nothing.
        e.mem.set_u8(holder + 2, 1);
        e.mem.set_u8(holder + 3, 0);
        e.call(
            0x00d6_c500,
            &args![holder, 0x51u32, base_first, base_second, 0x53u32],
        );
        assert!(seen.borrow().is_empty());
        // Axes 1 and 2.
        e.mem.set_u8(holder + 3, 2);
        e.call(
            0x00d6_c500,
            &args![holder, 0x51u32, base_first, base_second, 0x53u32],
        );
        let calls = seen.borrow();
        assert_eq!(calls.len(), 2);
        // k = 1: table[2] = 0x20, table[1] = 0x10 and table[3] = 0 pick the three vectors.
        assert_eq!(calls[0].1, [[3.0; 4], [2.0; 4], [4.0; 4]]);
        assert_eq!(calls[0].0[1..], [0x51, 0x53]);
        // k = 2: table[3] = 0, table[2] = 0x20 and table[4] = 0x10.
        assert_eq!(calls[1].1, [[1.0; 4], [3.0; 4], [5.0; 4]]);
    }
}

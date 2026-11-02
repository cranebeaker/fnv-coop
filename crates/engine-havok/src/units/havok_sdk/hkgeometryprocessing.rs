//! `Havok SDK/hkgeometryprocessing.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The SSE helpers below (`Vec4`, `shufps`, lane arithmetic) are shared with the other
//! Havok SDK units of this batch.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::rsqrt_estimate;

/// Four `float` lanes of an `xmm` register.
pub(crate) type Vec4 = [f32; 4];

/// A 16-byte vector at `at`.
pub(crate) fn load_vec4(e: &Engine, at: u32) -> Vec4 {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

/// Stores the four lanes at `at` (bit patterns kept).
pub(crate) fn store_vec4(e: &mut Engine, at: u32, v: Vec4) {
    for (i, lane) in v.into_iter().enumerate() {
        e.mem.set_u32(at + 4 * i as u32, lane.to_bits());
    }
}

/// Four 32-bit words at `at` (a mask constant).
pub(crate) fn load_bits(e: &Engine, at: u32) -> [u32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.u32(at + 4 * i))
}

/// `SHUFPS dst, src, imm`: lanes 0 and 1 from `dst`, lanes 2 and 3 from `src`.
pub(crate) fn shufps(dst: Vec4, src: Vec4, imm: u32) -> Vec4 {
    [
        dst[(imm & 3) as usize],
        dst[((imm >> 2) & 3) as usize],
        src[((imm >> 4) & 3) as usize],
        src[((imm >> 6) & 3) as usize],
    ]
}

/// `UNPCKLPS dst, src`: `[dst0, src0, dst1, src1]`.
pub(crate) fn unpcklps(dst: Vec4, src: Vec4) -> Vec4 {
    [dst[0], src[0], dst[1], src[1]]
}

/// `UNPCKHPS dst, src`: `[dst2, src2, dst3, src3]`.
pub(crate) fn unpckhps(dst: Vec4, src: Vec4) -> Vec4 {
    [dst[2], src[2], dst[3], src[3]]
}

/// `MOVLHPS dst, src`: `[dst0, dst1, src0, src1]`.
pub(crate) fn movlhps(dst: Vec4, src: Vec4) -> Vec4 {
    [dst[0], dst[1], src[0], src[1]]
}

/// `MOVHLPS dst, src`: `[src2, src3, dst2, dst3]`.
pub(crate) fn movhlps(dst: Vec4, src: Vec4) -> Vec4 {
    [src[2], src[3], dst[2], dst[3]]
}

pub(crate) fn add4(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] + b[i])
}

pub(crate) fn sub4(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] - b[i])
}

pub(crate) fn mul4(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] * b[i])
}

/// The sum of lanes 0 to 2 the way the code builds it: lane 1 + lane 0 (`SHUFPS 0x55`,
/// `ADDSS`), then lane 2 + that (`SHUFPS 0xaa`, `ADDSS`).
pub(crate) fn sum3(v: Vec4) -> f32 {
    let low = v[1] + v[0];
    v[2] + low
}

/// `SHUFPS xmm, xmm, 0` of the scalar at `at` (a `MOVSS` load, then the splat).
pub(crate) fn splat_at(e: &Engine, at: u32) -> Vec4 {
    [e.mem.f32(at); 4]
}

/// The four-lane `1.0` vector.
pub(crate) const ONES: u32 = 0x010e_c7f0;
/// The four-lane `0.5` vector (`010c71d0`).
const HALVES: u32 = 0x010c_71d0;
/// The four-lane absolute-value mask `0x7fffffff` (`01017d10`).
const ABS_MASK: u32 = 0x0101_7d10;
/// `0.0` as a scalar (`01011d78`) and as a vector (`010c7160`).
const ZERO_SCALAR: u32 = 0x0101_1d78;
const ZERO_VECTOR: u32 = 0x010c_7160;
/// The scalars `fn_00de9b10`, `fn_00de9b50` and `fn_00de9b90` multiply by.
const BLEND_SCALAR: u32 = 0x0101_6248;
const THIRD_SCALAR: u32 = 0x0106_c2e0;
const QUARTER_SCALAR: u32 = 0x0101_622c;
/// The free routine `00401030` (cdecl, one word).
const FREE: u32 = 0x0040_1030;
/// The memory router (`00c85750`; its word at `+0x10` is the allocator freed through).
const MEMORY_ROUTER_GETTER: u32 = 0x00c8_5750;
/// Virtual tables stored by the destructors below.
const FUNCTION_VTABLE: u32 = 0x010d_9fc4;
const NEAREST_FEATURE_POLICY_VTABLE: u32 = 0x010c_d664;

// Translated from 00de94b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryProcessing::squaredDistanceFromPointToLine` (Xbox PDB), cdecl
/// (`point`, `start`, `end`, `fraction`): the squared distance from the point (a 16-byte
/// vector at `point`) to the segment `start` to `end`. With `w = start - point` and
/// `d = end - start` the parameter is `(0 - w.d) / d.d` (the zero is the lane 0 of
/// `010c7160`), stored as a `float` through the x87 stack at `fraction` and clamped
/// there: below 0 it becomes 0.0, above 1 it becomes 1.0, a NaN stays. The answer (in
/// `ST0`) is `|d * t + w|^2` over lanes 0 to 2 with the stored parameter.
pub fn hk_geometry_processing_squared_distance_from_point_to_line(
    e: &mut Engine,
    point: u32,
    start: u32,
    end: u32,
    fraction: u32,
) -> f32 {
    let a = load_vec4(e, point);
    let b = load_vec4(e, start);
    let c = load_vec4(e, end);
    let w = sub4(b, a);
    let d = sub4(c, b);
    let numerator = e.mem.f32(ZERO_VECTOR) - sum3(mul4(d, w));
    let t = x87_quiet(numerator / sum3(mul4(d, d)));
    e.mem.set_f32(fraction, t);
    if 0.0 > t {
        e.mem.set_f32(fraction, 0.0);
    } else if 1.0 < t {
        e.mem.set_f32(fraction, 1.0);
    }
    let t = e.mem.f32(fraction);
    let moved = add4(mul4([t; 4], d), w);
    x87_quiet(sum3(mul4(moved, moved)))
}

// Translated from 00de9590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): the box at `bounds` (minimum at `+0`,
/// maximum at `+0x10`) becomes the cube around its centre `(min + max) * 0.5` whose half
/// edge is the signed half extent `(max - min) * 0.5` of the axis with the largest
/// absolute half extent (x87 comparisons: the later axis wins only when strictly larger,
/// a NaN loses). The cube's lane 3 is offset by the scalar at `01011d78`.
pub fn fn_00de9590(e: &mut Engine, bounds: Ptr) {
    let at = bounds.addr();
    let max = load_vec4(e, at + 0x10);
    let min = load_vec4(e, at);
    let halves = load_vec4(e, HALVES);
    let half = mul4(sub4(max, min), halves);
    let sum = add4(min, max);
    let mask = load_bits(e, ABS_MASK);
    let abs = [0, 1, 2, 3].map(|i| f32::from_bits(mask[i] & half[i].to_bits()));
    let centre = mul4(sum, halves);
    let index = if abs[1] > abs[0] {
        if abs[2] > abs[1] {
            2
        } else {
            1
        }
    } else if abs[2] > abs[0] {
        2
    } else {
        0
    };
    let x = x87_quiet(half[index]);
    let offset = [x, x, x, e.mem.f32(ZERO_SCALAR)];
    store_vec4(e, at, sub4(centre, offset));
    store_vec4(e, at + 0x10, add4(centre, offset));
}

// Translated from 00de9b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `out`, `a`, `b`): the blend
/// `(1 - s) * a + b * s` of two 16-byte vectors, `s` being the scalar at `01016248`
/// splatted, stored at `out`.
pub fn fn_00de9b10(e: &mut Engine, out: u32, a: u32, b: u32) {
    let s = splat_at(e, BLEND_SCALAR);
    let ones = load_vec4(e, ONES);
    let va = load_vec4(e, a);
    let vb = load_vec4(e, b);
    let result = add4(mul4(sub4(ones, s), va), mul4(vb, s));
    store_vec4(e, out, result);
}

// Translated from 00de9b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `out`, `a`, `b`, `c`): `(a + b + c) * k`, `k` being
/// the scalar at `0106c2e0` splatted, stored at `out`.
pub fn fn_00de9b50(e: &mut Engine, out: u32, a: u32, b: u32, c: u32) {
    let sum = add4(add4(load_vec4(e, a), load_vec4(e, b)), load_vec4(e, c));
    let k = splat_at(e, THIRD_SCALAR);
    store_vec4(e, out, mul4(sum, k));
}

// Translated from 00de9b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `out`, `a`, `b`, `c`, `d`): `((c + d) + (a + b)) * k`,
/// `k` being the scalar at `0101622c` splatted, stored at `out`.
pub fn fn_00de9b90(e: &mut Engine, out: u32, a: u32, b: u32, c: u32, d: u32) {
    let first = add4(load_vec4(e, c), load_vec4(e, d));
    let second = add4(load_vec4(e, a), load_vec4(e, b));
    let k = splat_at(e, QUARTER_SCALAR);
    store_vec4(e, out, mul4(add4(first, second), k));
}

// Translated from 00dee620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryProcessing::IFunction<hkVector4_float>::_scalar_deleting_destructor_` (Xbox
/// PDB), `thiscall`, `RET 4`: stores the virtual table (`010d9fc4`) and, when bit 0 of
/// `flags` is set, frees the object (`00401030`, cdecl). Returns `this`.
pub fn hk_geometry_processing_i_function_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), FUNCTION_VTABLE);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00dee640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryProcessingInternals::NearestFeaturePolicy::addLeaf` (Xbox PDB), `thiscall`,
/// `RET 4` (`bounds`, a box with minimum at `+0` and maximum at `+0x10`): the squared
/// distance from the query point at `this + 0x10` to the box's centre
/// `(min + max) * 0.5` (lanes 0 to 2). When it is below the best distance stored at
/// `this + 0x20` (x87 compare: only a strictly smaller value counts) it replaces it and
/// `bounds` is stored at `this + 0x30`. Always answers 1.0 in `ST0`.
pub fn hk_geometry_processing_internals_nearest_feature_policy_add_leaf(
    e: &mut Engine,
    this: Ptr,
    bounds: u32,
) -> f32 {
    let at = this.addr();
    let min = load_vec4(e, bounds);
    let max = load_vec4(e, bounds + 0x10);
    let halves = load_vec4(e, HALVES);
    let centre = mul4(add4(max, min), halves);
    let delta = sub4(load_vec4(e, at + 0x10), centre);
    let distance = x87_quiet(sum3(mul4(delta, delta)));
    let best = x87_quiet(e.mem.f32(at + 0x20));
    if best > distance {
        e.mem.set_f32(at + 0x20, distance);
        e.mem.set_u32(at + 0x30, bounds);
    }
    1.0
}

// Translated from 00dee6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryProcessing::ConstFunction<hkGeometryProcessing::IFunction<hkVector4_float>_>::evaluate`
/// (Xbox PDB), `thiscall`, `RET 4`: the `float` at `this + 4` (through the x87 stack). The
/// argument is never read.
pub fn hk_geometry_processing_const_function_evaluate(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
) -> f32 {
    x87_quiet(e.mem.f32(this.addr() + 4))
}

// Translated from 00dee6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryProcessingInternals::NearestFeaturePolicy::_scalar_deleting_destructor_`
/// (Xbox PDB), `thiscall`, `RET 4`: stores the virtual table (`010cd664`) and, when bit 0
/// of `flags` is set, frees 0x40 bytes at `this` through the memory router's allocator
/// (`00c85750`, `+0x10`, slot 8). Returns `this`.
pub fn hk_geometry_processing_internals_nearest_feature_policy_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    e.mem.set_u32(this.addr(), NEAREST_FEATURE_POLICY_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER_GETTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![this, 0x40u32]);
    }
    this
}

/// The x87 register stack, one `f64` per register (the code computes in extended
/// precision and stores `float`s; see `docs/ENGINE_CRATE.md`). `ST0` is the last element.
/// Every method is one x87 instruction of the exe, named after it; operands are register
/// indexes as the exe writes them (`ST(i)`), `*_m` methods take the memory operand as an
/// `f64`. Float loads and stores keep NaN payloads the way the hardware does (a signalling
/// NaN gets its quiet bit, `x87_quiet_bits`).
#[derive(Debug, Default)]
pub(crate) struct X87 {
    stack: Vec<f64>,
}

/// `FLD m32`: the extended value of a `float` (NaN payload shifted up, quiet bit set).
pub(crate) fn float_to_x87(value: f32) -> f64 {
    let bits = x87_quiet_bits(value.to_bits());
    if f32::from_bits(bits).is_nan() {
        let sign = u64::from(bits >> 31) << 63;
        f64::from_bits(sign | 0x7ff0_0000_0000_0000 | (u64::from(bits & 0x007f_ffff) << 29))
    } else {
        f64::from(value)
    }
}

/// `FSTP m32`: the `float` of an extended value (a NaN keeps the top of its payload).
pub(crate) fn x87_to_float(value: f64) -> f32 {
    if value.is_nan() {
        let bits = value.to_bits();
        let sign = ((bits >> 63) as u32) << 31;
        f32::from_bits(sign | 0x7f80_0000 | 0x0040_0000 | ((bits >> 29) as u32 & 0x007f_ffff))
    } else {
        value as f32
    }
}

/// The status word's high byte after a compare: `C0` is 1, `C2` is 4, `C3` is 0x40.
pub(crate) fn compare_flags(a: f64, b: f64) -> u8 {
    if a.is_nan() || b.is_nan() {
        0x45
    } else if a > b {
        0
    } else if a < b {
        0x01
    } else {
        0x40
    }
}

/// The parity flag `TEST AH, mask` leaves: set when the masked byte has an even number of
/// bits (`JP` jumps when it is set, `JNP` when it is not).
pub(crate) fn test_parity_even(ah: u8, mask: u8) -> bool {
    (ah & mask).count_ones() % 2 == 0
}

impl X87 {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// `ST(i)`.
    pub(crate) fn st(&self, i: usize) -> f64 {
        self.stack[self.stack.len() - 1 - i]
    }

    fn set(&mut self, i: usize, value: f64) {
        let at = self.stack.len() - 1 - i;
        self.stack[at] = value;
    }

    /// True when no register is in use (the exes leave the stack empty on return, except
    /// where a function answers in `ST0`).
    pub(crate) fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Replaces `ST0` (a call that takes `ST0` and answers in `ST0`).
    pub(crate) fn set_st0(&mut self, value: f64) {
        self.set(0, value);
    }

    /// `FLD m64`.
    pub(crate) fn fld(&mut self, value: f64) {
        assert!(self.stack.len() < 8, "x87 stack overflow");
        self.stack.push(value);
    }

    /// `FLD m32`.
    pub(crate) fn fld_f32(&mut self, value: f32) {
        self.fld(float_to_x87(value));
    }

    /// `FLD ST(i)`.
    pub(crate) fn fld_st(&mut self, i: usize) {
        let value = self.st(i);
        self.fld(value);
    }

    pub(crate) fn fld1(&mut self) {
        self.fld(1.0);
    }

    pub(crate) fn fldz(&mut self) {
        self.fld(0.0);
    }

    /// Drops `ST0` (`FSTP ST0`).
    pub(crate) fn pop(&mut self) {
        self.stack.pop().expect("x87 stack underflow");
    }

    /// `FXCH ST(i)`.
    pub(crate) fn fxch(&mut self, i: usize) {
        let top = self.st(0);
        let other = self.st(i);
        self.set(0, other);
        self.set(i, top);
    }

    /// `FST m32`.
    pub(crate) fn fst_f32(&self) -> f32 {
        x87_to_float(self.st(0))
    }

    /// `FSTP m32`.
    pub(crate) fn fstp_f32(&mut self) -> f32 {
        let value = self.fst_f32();
        self.pop();
        value
    }

    /// `FSTP ST(i)`: copies `ST0` to `ST(i)` and pops.
    pub(crate) fn fstp_st(&mut self, i: usize) {
        let value = self.st(0);
        self.set(i, value);
        self.pop();
    }

    pub(crate) fn chs(&mut self) {
        let value = self.st(0);
        self.set(0, -value);
    }

    /// `ST(dst) = ST(dst) + ST(src)` (`FADD`).
    pub(crate) fn add(&mut self, dst: usize, src: usize) {
        let value = self.st(dst) + self.st(src);
        self.set(dst, value);
    }

    /// `ST(dst) = ST(dst) - ST(src)` (`FSUB`).
    pub(crate) fn sub(&mut self, dst: usize, src: usize) {
        let value = self.st(dst) - self.st(src);
        self.set(dst, value);
    }

    /// `ST(dst) = ST(src) - ST(dst)` (`FSUBR`).
    pub(crate) fn subr(&mut self, dst: usize, src: usize) {
        let value = self.st(src) - self.st(dst);
        self.set(dst, value);
    }

    /// `ST(dst) = ST(dst) * ST(src)` (`FMUL`).
    pub(crate) fn mul(&mut self, dst: usize, src: usize) {
        let value = self.st(dst) * self.st(src);
        self.set(dst, value);
    }

    /// `ST(dst) = ST(dst) / ST(src)` (`FDIV`).
    pub(crate) fn div(&mut self, dst: usize, src: usize) {
        let value = self.st(dst) / self.st(src);
        self.set(dst, value);
    }

    /// `ST(dst) = ST(src) / ST(dst)` (`FDIVR`).
    pub(crate) fn divr(&mut self, dst: usize, src: usize) {
        let value = self.st(src) / self.st(dst);
        self.set(dst, value);
    }

    /// `FADDP ST(i), ST0`.
    pub(crate) fn addp(&mut self, i: usize) {
        self.add(i, 0);
        self.pop();
    }

    /// `FSUBP ST(i), ST0`: `ST(i) = ST(i) - ST0`.
    pub(crate) fn subp(&mut self, i: usize) {
        self.sub(i, 0);
        self.pop();
    }

    /// `FSUBRP ST(i), ST0`: `ST(i) = ST0 - ST(i)`.
    pub(crate) fn subrp(&mut self, i: usize) {
        self.subr(i, 0);
        self.pop();
    }

    /// `FMULP ST(i), ST0`.
    pub(crate) fn mulp(&mut self, i: usize) {
        self.mul(i, 0);
        self.pop();
    }

    /// `FDIVP ST(i), ST0`: `ST(i) = ST(i) / ST0`.
    pub(crate) fn divp(&mut self, i: usize) {
        self.div(i, 0);
        self.pop();
    }

    /// `FDIVRP ST(i), ST0`: `ST(i) = ST0 / ST(i)`.
    pub(crate) fn divrp(&mut self, i: usize) {
        self.divr(i, 0);
        self.pop();
    }

    /// `FADD m`: `ST0 = ST0 + operand`.
    pub(crate) fn add_m(&mut self, operand: f64) {
        let value = self.st(0) + operand;
        self.set(0, value);
    }

    /// `FSUB m`: `ST0 = ST0 - operand`.
    pub(crate) fn sub_m(&mut self, operand: f64) {
        let value = self.st(0) - operand;
        self.set(0, value);
    }

    /// `FMUL m`: `ST0 = ST0 * operand`.
    pub(crate) fn mul_m(&mut self, operand: f64) {
        let value = self.st(0) * operand;
        self.set(0, value);
    }

    /// `FDIV m`: `ST0 = ST0 / operand`.
    pub(crate) fn div_m(&mut self, operand: f64) {
        let value = self.st(0) / operand;
        self.set(0, value);
    }

    /// `FCOM ST(i)` / `FUCOM ST(i)` and `FNSTSW AX`: the high byte of the status word.
    pub(crate) fn fcom(&self, i: usize) -> u8 {
        compare_flags(self.st(0), self.st(i))
    }

    /// `FCOM m` / `FCOMP m` and `FNSTSW AX` (the caller pops for `FCOMP`).
    pub(crate) fn fcom_m(&self, operand: f64) -> u8 {
        compare_flags(self.st(0), operand)
    }

    /// `FILD m32`: pushes a signed 32-bit integer.
    pub(crate) fn fild(&mut self, value: i32) {
        self.fld(f64::from(value));
    }

    /// `FABS`.
    pub(crate) fn abs(&mut self) {
        let value = self.st(0).abs();
        self.set(0, value);
    }

    /// `FDIVR m`: `ST0 = operand / ST0`.
    pub(crate) fn divr_m(&mut self, operand: f64) {
        let value = operand / self.st(0);
        self.set(0, value);
    }

    /// `FSUBR m`: `ST0 = operand - ST0`.
    pub(crate) fn subr_m(&mut self, operand: f64) {
        let value = operand - self.st(0);
        self.set(0, value);
    }
}

/// `_CIsqrt`, `_CIcos` and `_CIsin` (`00ec6040`, `00ec9f70`, `00eca0a0`): the argument is
/// in `ST0` and the answer replaces it.
pub(crate) const X87_SQRT: u32 = 0x00ec_6040;
const X87_COS: u32 = 0x00ec_9f70;
const X87_SIN: u32 = 0x00ec_a0a0;
/// `2.0`, `3.0` and `1.5707963...` (`doubles` in the exe image) and the `float` `-1.0`.
const TWO_DOUBLE: u32 = 0x0101_1590;
const THREE_DOUBLE: u32 = 0x0102_1928;
const HALF_PI_DOUBLE: u32 = 0x0103_0f38;
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;

/// Calls the CRT function at `address` on `ST0` of `x`.
pub(crate) fn call_on_st0(e: &mut Engine, x: &mut X87, address: u32) {
    let argument = x.st(0);
    let answer = e.call(address, &args![argument]).f64();
    x.set_st0(answer);
}

// Translated from 00de9650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `input`, `out`): maps a point of the unit square (two
/// `float`s at `input`) to a unit vector written at `out` (lanes x, y, z, 0). With
/// `x = 2u - 1` and `y = 2v - 1` the code chooses one of eight regions by the signs of `y`
/// and `x` and by comparing `x + y` and `y - x` with 1 and -1 (x87 compares: a NaN takes
/// the `y < 0` side and the later branches as the flags say); each region gives a radius
/// `r` and a sign for the third component, and an angle (the ratio of the other two terms
/// times the constant at `01030f38`, with 3.0 added in the regions that end at `97e6`).
/// The result is `(cos a * sqrt(2 - r^2) * r, sin a * sqrt(2 - r^2) * r, (1 - r^2) * sign,
/// 0)`, with `_CIsqrt`, `_CIcos` and `_CIsin` (`00ec6040`, `00ec9f70`, `00eca0a0`) called on
/// `ST0`. Written as a transcription of the x87 stack code (`X87`), region by region.
pub fn fn_00de9650(e: &mut Engine, input: u32, out: u32) {
    let two = e.mem.f64(TWO_DOUBLE);
    let minus_one = e.mem.f32(MINUS_ONE_FLOAT);
    let mut x = X87::new();
    x.fld_f32(e.mem.f32(input));
    x.fld(two);
    x.mul(1, 0);
    x.fld1();
    x.sub(2, 0);
    x.fxch(2);
    let slot_10 = x.fstp_f32();
    x.fld_f32(e.mem.f32(input + 4));
    x.mul(0, 1);
    x.sub(0, 2);
    let slot_14 = x.fstp_f32();
    x.fld_f32(slot_14);
    x.fld_st(0);
    x.fld_f32(slot_10);
    x.fld_st(0);
    x.addp(2);
    x.fxch(1);
    let slot_10 = x.fstp_f32();
    x.fld_st(1);
    x.sub(0, 1);
    let slot_c = x.fstp_f32();
    x.fldz();
    let first = x.fcom(2);
    let second = x.fcom(1);
    let (slot_8, slot_10) = if !test_parity_even(first, 0x41) {
        // 0x96aa: y >= 0.
        x.fld1();
        if !test_parity_even(second, 0x41) {
            // 0x96b1: x >= 0.
            x.fld_f32(slot_10);
            let flags = x.fcom(1);
            if !test_parity_even(flags, 0x41) {
                x.fstp_st(6);
                x.fstp_st(2);
                x.fxch(4);
                let slot_8 = x.fst_f32();
                x.fxch(1);
                let slot_10 = x.fstp_f32();
                x.divp(1);
                (slot_8, slot_10)
            } else {
                x.fstp_st(4);
                x.pop();
                x.fld_st(3);
                x.subrp(3);
                x.fxch(2);
                let slot_8 = x.fstp_f32();
                x.fld_f32(minus_one);
                let slot_10 = x.fstp_f32();
                x.subp(3);
                x.fld_f32(slot_8);
                x.divp(3);
                x.fxch(2);
                (slot_8, slot_10)
            }
        } else {
            // 0x96fc: x < 0.
            x.fld_f32(slot_c);
            let flags = x.fcom(1);
            if !test_parity_even(flags, 0x41) {
                x.fstp_st(4);
                x.fxch(3);
                let slot_8 = x.fst_f32();
                x.fxch(3);
                let slot_10 = x.fstp_f32();
                x.fxch(1);
                x.divrp(2);
                x.fxch(3);
                x.subrp(1);
                (slot_8, slot_10)
            } else {
                x.fstp_st(3);
                x.pop();
                x.fld_st(3);
                x.subrp(2);
                x.fxch(1);
                let slot_8 = x.fstp_f32();
                x.fld_f32(minus_one);
                let slot_10 = x.fstp_f32();
                x.fld_st(3);
                x.subrp(2);
                x.fld_f32(slot_8);
                x.divp(2);
                x.fxch(1);
                x.addp(3);
                x.fxch(2);
                (slot_8, slot_10)
            }
        }
    } else {
        // 0x9751: y < 0 (or NaN).
        x.fld_f32(minus_one);
        if (second & 0x41) != 0 {
            // 0x97a3: x >= 0 (or NaN).
            x.fld_f32(slot_c);
            let flags = x.fcom(1);
            let (slot_8, slot_10) = if flags & 1 == 0 {
                x.fstp_st(6);
                x.fstp_st(3);
                x.fstp_st(2);
                x.fxch(3);
                x.chs();
                let slot_8 = x.fstp_f32();
                x.fld1();
                let slot_10 = x.fstp_f32();
                x.fld_f32(slot_8);
                x.divp(3);
                (slot_8, slot_10)
            } else {
                x.fstp_st(3);
                x.fxch(2);
                x.add(0, 4);
                let slot_8 = x.fstp_f32();
                x.fxch(1);
                let slot_10 = x.fstp_f32();
                x.fxch(1);
                x.addp(3);
                x.fld_f32(slot_8);
                x.divp(3);
                (slot_8, slot_10)
            };
            // 0x97e6
            x.fxch(2);
            x.add_m(e.mem.f64(THREE_DOUBLE));
            (slot_8, slot_10)
        } else {
            // 0x975c: x < 0.
            x.fld_f32(slot_10);
            let flags = x.fcom(1);
            if flags & 1 == 0 {
                x.fstp_st(6);
                x.fstp_st(2);
                x.fstp_st(1);
                x.fxch(3);
                x.chs();
                let slot_8 = x.fstp_f32();
                x.fld1();
                let slot_10 = x.fstp_f32();
                x.div_m(float_to_x87(slot_8));
                x.subr(0, 1);
                (slot_8, slot_10)
            } else {
                x.fstp_st(4);
                x.fxch(3);
                x.add(0, 4);
                let slot_8 = x.fstp_f32();
                x.fxch(2);
                let slot_10 = x.fstp_f32();
                x.addp(3);
                x.fld_f32(slot_8);
                x.divp(3);
                x.fxch(2);
                x.add(0, 1);
                (slot_8, slot_10)
            }
        }
    };
    // 0x97ee
    let ratio = x.fstp_f32();
    x.fld_st(1);
    x.fld_f32(slot_8);
    let flags = x.fcom(1);
    x.fstp_st(1);
    let angle = if !test_parity_even(flags, 0x44) {
        x.fxch(2);
        x.fstp_f32()
    } else {
        x.fstp_st(2);
        x.fld_f32(ratio);
        x.mul_m(e.mem.f64(HALF_PI_DOUBLE));
        x.fstp_f32()
    };
    x.fld_st(1);
    x.mulp(2);
    x.fxch(1);
    let slot_14 = x.fstp_f32();
    x.sub_m(float_to_x87(slot_14));
    let slot_18 = x.fstp_f32();
    x.fld_f32(slot_18);
    call_on_st0(e, &mut x, X87_SQRT);
    let slot_18 = x.fstp_f32();
    x.fld_f32(slot_18);
    x.mul_m(float_to_x87(slot_8));
    let slot_8 = x.fstp_f32();
    x.fld_f32(angle);
    call_on_st0(e, &mut x, X87_COS);
    let slot_18 = x.fstp_f32();
    x.fld_f32(slot_18);
    x.mul_m(float_to_x87(slot_8));
    let cosine_term = x.fstp_f32();
    x.fld_f32(angle);
    call_on_st0(e, &mut x, X87_SIN);
    let slot_18 = x.fstp_f32();
    x.fld_f32(slot_18);
    x.mul_m(float_to_x87(slot_8));
    let sine_term = x.fstp_f32();
    x.fld_f32(slot_14);
    x.fld1();
    x.subrp(1);
    x.mul_m(float_to_x87(slot_10));
    let third = x.fstp_f32();
    assert!(x.is_empty(), "the x87 stack is empty at the return");
    store_vec4(
        e,
        out,
        [cosine_term, sine_term, third, e.mem.f32(ZERO_SCALAR)],
    );
}

/// `hkMath_atan2fApproximation` (`00cbff80`, cdecl `(y, x)`, answers in `ST0`).
const ATAN2_APPROXIMATION: u32 = 0x00cb_ff80;
/// Doubles in the exe image: `1.0`, `0.5` and the constant the angle is divided by
/// (`0101ff40`).
const ONE_DOUBLE: u32 = 0x0101_2070;
const HALF_DOUBLE: u32 = 0x0101_1588;
const ANGLE_DIVISOR_DOUBLE: u32 = 0x0101_ff40;

// Translated from 00de98c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `input`, `out`): the inverse of `fn_00de9650`. Maps a
/// vector (three `float`s at `input`) to a point of the unit square written at `out` (lanes
/// u, v, 0, 0). The angle is `2 * atan2(y, x) / c` (`00cbff80`, `c` the double at
/// `0101ff40`) and the radius `sqrt(1 -/+ z)`; the result depends on the signs of `z` and of
/// the angle against 0, 1 and -1 (x87 compares, as the exe's branches), through the x87
/// stack code as written. The exe's frame matters at the end: after the `POP ESI` the
/// offsets `+4` and `+8` read the slots that were `+8` and `+0xc` before, so the second
/// lane is `0.5 * (1 + radius result)` (the word kept for `[ESP + 0xc]`) and the first is
/// `(angle result + 1) * 0.5` (the word kept for `[ESP + 8]`).
pub fn fn_00de98c0(e: &mut Engine, input: u32, out: u32) {
    let one = e.mem.f64(ONE_DOUBLE);
    let two = e.mem.f64(TWO_DOUBLE);
    let minus_one = e.mem.f32(MINUS_ONE_FLOAT);
    let y = x87_quiet(e.mem.f32(input + 4));
    let x_coordinate = x87_quiet(e.mem.f32(input));
    let angle = e.call(ATAN2_APPROXIMATION, &args![y, x_coordinate]).f64();
    let mut x = X87::new();
    x.fld(angle);
    x.add(0, 0);
    x.div_m(e.mem.f64(ANGLE_DIVISOR_DOUBLE));
    let slot_8 = x.fstp_f32();
    let z = e.mem.f32(input + 8);
    x.fldz();
    let z_flags = x.fcom_m(float_to_x87(z));
    x.pop();
    x.fld_f32(z);
    let (slot_8, slot_c) = if z_flags & 0x41 == 0 {
        // z < 0.
        let mut slot_8 = slot_8;
        x.add_m(one);
        let mut slot_c = x.fstp_f32();
        x.fld_f32(slot_c);
        call_on_st0(e, &mut x, X87_SQRT);
        slot_c = x.fstp_f32();
        x.fld_f32(slot_c);
        slot_c = x.fstp_f32();
        x.fldz();
        x.fld_f32(slot_8);
        let flags = x.fcom(1);
        x.fstp_st(1);
        if flags & 1 == 0 {
            x.fld1();
            let flags = x.fcom(1);
            x.pop();
            if flags & 1 == 0 {
                x.fld_f32(slot_c);
                x.fld_st(0);
                x.mulp(2);
                x.fld1();
                x.subrp(2);
                x.fxch(1);
                slot_8 = x.fstp_f32();
                x.subr_m(two);
                x.sub_m(float_to_x87(slot_8));
                slot_c = x.fstp_f32();
            } else {
                x.fld(two);
                x.fld_st(0);
                x.subrp(2);
                x.fld_f32(slot_c);
                x.fld_st(0);
                x.mulp(3);
                x.fxch(2);
                x.sub_m(one);
                slot_8 = x.fstp_f32();
                x.subrp(1);
                x.add_m(float_to_x87(slot_8));
                slot_c = x.fstp_f32();
            }
        } else {
            x.fld_f32(minus_one);
            let flags = x.fcom(1);
            x.pop();
            if !test_parity_even(flags, 0x41) {
                x.fld_f32(slot_c);
                x.fld_st(0);
                x.mulp(2);
                x.fxch(1);
                x.add_m(one);
                slot_8 = x.fstp_f32();
                x.sub_m(two);
                x.add_m(float_to_x87(slot_8));
                slot_c = x.fstp_f32();
            } else {
                x.fld(two);
                x.add(1, 0);
                x.fld_f32(slot_c);
                x.fld_st(0);
                x.mulp(3);
                x.fxch(2);
                x.sub_m(one);
                slot_8 = x.fstp_f32();
                x.subp(1);
                x.sub_m(float_to_x87(slot_8));
                slot_c = x.fstp_f32();
            }
        }
        (slot_8, slot_c)
    } else {
        // z >= 0 (or NaN).
        x.fld1();
        x.subrp(1);
        let mut slot_c = x.fstp_f32();
        x.fld_f32(slot_c);
        call_on_st0(e, &mut x, X87_SQRT);
        slot_c = x.fstp_f32();
        x.fld_f32(slot_c);
        slot_c = x.fstp_f32();
        x.fldz();
        x.fld_f32(slot_8);
        let flags = x.fcom(1);
        x.fstp_st(1);
        if flags & 1 == 0 {
            x.fld1();
            let flags = x.fcom(1);
            x.pop();
            if flags & 0x41 == 0 {
                x.fld_f32(slot_c);
                x.fld_st(0);
                x.mulp(2);
                x.fxch(1);
                slot_c = x.fstp_f32();
                x.sub_m(float_to_x87(slot_c));
            } else {
                x.subr_m(two);
                x.fld_f32(slot_c);
                x.fld_st(0);
                x.mulp(2);
                x.fxch(1);
                slot_c = x.fstp_f32();
                x.subr_m(float_to_x87(slot_c));
            }
        } else {
            x.fld_f32(minus_one);
            let flags = x.fcom(1);
            x.pop();
            if !test_parity_even(flags, 0x05) {
                x.fld_f32(slot_c);
                x.fld_st(0);
                x.mulp(2);
                x.fxch(1);
                slot_c = x.fstp_f32();
                x.add_m(float_to_x87(slot_c));
            } else {
                x.add_m(two);
                x.fld_f32(slot_c);
                x.fld_st(0);
                x.chs();
                x.mulp(2);
                x.fxch(1);
                slot_c = x.fstp_f32();
                x.add_m(float_to_x87(slot_c));
                x.chs();
            }
        }
        let slot_8 = x.fstp_f32();
        (slot_8, slot_c)
    };
    // 0x9aab: the frame has moved up by one word (`POP ESI`).
    x.fld_f32(slot_8);
    x.fld1();
    x.add(1, 0);
    x.fld(e.mem.f64(HALF_DOUBLE));
    x.mul(2, 0);
    x.fxch(2);
    let first = x.fstp_f32();
    x.add_m(float_to_x87(slot_c));
    x.mulp(1);
    let second = x.fstp_f32();
    assert!(x.is_empty(), "the x87 stack is empty at the return");
    store_vec4(
        e,
        out,
        [
            first,
            second,
            e.mem.f32(ZERO_SCALAR),
            e.mem.f32(ZERO_SCALAR),
        ],
    );
}

/// `_ftol2_sse` (`00ec62c0`): truncates `ST0` (passed as an `f64` argument) to an integer.
const FTOL: u32 = 0x00ec_62c0;
/// The starting maximum of the support search, `-3.4028233e38` (`0104e148`), the four lane
/// numbers `0.5, 1.5, 2.5, 3.5` (`010d9fe0`) and the step `4.0` (`010d0fa0`).
const LOWEST_FLOAT: u32 = 0x0104_e148;
const LANE_NUMBERS: u32 = 0x010d_9fe0;
const LANE_STEP: u32 = 0x010d_0fa0;
/// The threshold `fn_00dea980` compares the sum of the squared edges with (`0104da4c`).
const EDGE_SUM_THRESHOLD: u32 = 0x0104_da4c;

/// `SHUFPS v, v, 0xc9` and `0xd2`: the two rotations the cross products use.
pub(crate) fn rotate_c9(v: Vec4) -> Vec4 {
    shufps(v, v, 0xc9)
}

pub(crate) fn rotate_d2(v: Vec4) -> Vec4 {
    shufps(v, v, 0xd2)
}

/// The cross product the code builds from the rotations, `(b - a) x (c - a)` in the
/// exe's operand order: `rot_d2(ac) * rot_c9(ab) - rot_c9(ac) * rot_d2(ab)`.
pub(crate) fn triangle_normal(ab: Vec4, ac: Vec4) -> Vec4 {
    sub4(
        mul4(rotate_d2(ac), rotate_c9(ab)),
        mul4(rotate_c9(ac), rotate_d2(ab)),
    )
}

/// `(rot_d2(x) * rot_c9(y) - rot_c9(x) * rot_d2(y)) * normal`, lanes 0 to 2 summed.
pub(crate) fn edge_term(x: Vec4, y: Vec4, normal: Vec4) -> f32 {
    sum3(mul4(
        sub4(
            mul4(rotate_d2(x), rotate_c9(y)),
            mul4(rotate_c9(x), rotate_d2(y)),
        ),
        normal,
    ))
}

// Translated from 00de9e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `a`, `b`, `c`, `out`, `fraction`): the offset from the
/// point `a` to the closest point of the segment `b` to `c`, negated, written at `out`
/// (16-byte vectors). With `w = b - a` and `d = c - b` the parameter is
/// `(0 - w.d) / d.d` (lane 0 of `010c7160`), clamped to 0 below 0 and to 1 above 1 (a NaN
/// stays); `out = ((t * d + w) * -1)` over all four lanes, and the parameter is stored at
/// `fraction` when that is not null. Answers the squared length of `t * d + w` (lanes 0
/// to 2) in `ST0`.
pub fn fn_00de9e10(e: &mut Engine, a: u32, b: u32, c: u32, out: u32, fraction: u32) -> f32 {
    let va = load_vec4(e, a);
    let vb = load_vec4(e, b);
    let vc = load_vec4(e, c);
    let w = sub4(vb, va);
    let d = sub4(vc, vb);
    let numerator = e.mem.f32(ZERO_VECTOR) - sum3(mul4(d, w));
    let t = numerator / sum3(mul4(d, d));
    let loaded = x87_quiet(t);
    let clamped = loaded.clamp(0.0, 1.0);
    let minus_one = [e.mem.f32(MINUS_ONE_FLOAT); 4];
    let moved = add4(mul4([clamped; 4], d), w);
    store_vec4(e, out, mul4(minus_one, moved));
    if fraction != 0 {
        e.mem.set_f32(fraction, x87_quiet(clamped));
    }
    x87_quiet(sum3(mul4(moved, moved)))
}

// Translated from 00dea6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryProcessing::checkPointInsideTriangle` (Xbox PDB), cdecl (`point`, `a`, `b`,
/// `c`, `epsilon`): true when the point is on the inner side of all three edges of the
/// triangle `a`, `b`, `c`. With `n` the cross product of the two edges at `a` it forms the
/// three edge terms `(a-p) x (b-p) . n`, `(b-p) x (c-p) . n` and `(c-p) x (a-p) . n`
/// (lanes 0 to 2 summed in the code's order) and answers false as soon as one is below
/// `epsilon` (x87 compares: a NaN never fails, so a NaN `epsilon` answers true).
pub fn hk_geometry_processing_check_point_inside_triangle(
    e: &mut Engine,
    point: u32,
    a: u32,
    b: u32,
    c: u32,
    epsilon: f32,
) -> bool {
    let va = load_vec4(e, a);
    let vb = load_vec4(e, b);
    let vc = load_vec4(e, c);
    let normal = triangle_normal(sub4(vb, va), sub4(vc, va));
    let vp = load_vec4(e, point);
    let ap = sub4(va, vp);
    let bp = sub4(vb, vp);
    let cp = sub4(vc, vp);
    let epsilon = float_to_x87(epsilon);
    let first = float_to_x87(edge_term(bp, ap, normal));
    if epsilon > first {
        return false;
    }
    let second = float_to_x87(edge_term(cp, bp, normal));
    if second < epsilon {
        return false;
    }
    let third = float_to_x87(edge_term(ap, cp, normal));
    let below = third < epsilon;
    !below
}

// Translated from 00dea810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `blocks`, `direction`, `out`): the support search over
/// a vertex array stored four to a block of 0x30 bytes (the x, y and z of four vertices at
/// `+0`, `+0x10`, `+0x20`). `blocks` points at a record whose word at `+0` is the first
/// block and whose word at `+4` is the number of blocks. Each block gives four dot
/// products with `direction` (`y*dy + dx*x`, then `+ z*dz`), kept lane by lane when larger
/// than the best so far (`CMPLTPS`, starting at `-3.4028233e38`) together with the
/// lane's running number (`0.5, 1.5, 2.5, 3.5` plus 4 per block). The four lanes are then
/// reduced with the x87 compares in the code's order (lane 1 beats lane 0 unless strictly
/// smaller, lane 3 beats lane 2 unless strictly smaller, the winners likewise); the
/// winner's number is truncated (`_ftol2_sse`), its vertex is written at `out` (x, y, z, 0)
/// and `out + 0xc` receives the number with `0x3f000000` or-ed in. Answers that word.
pub fn fn_00dea810(e: &mut Engine, blocks: u32, direction: u32, out: u32) -> u32 {
    let vertices = e.mem.u32(blocks);
    let count = e.mem.i32(blocks + 4);
    let lowest = e.mem.f32(LOWEST_FLOAT);
    let mut best = [lowest; 4];
    let mut best_number = [0.0f32; 4];
    let dir = load_vec4(e, direction);
    let (dx, dy, dz) = ([dir[0]; 4], [dir[1]; 4], [dir[2]; 4]);
    let mut number = load_vec4(e, LANE_NUMBERS);
    let step = load_vec4(e, LANE_STEP);
    let total = count.wrapping_mul(4);
    if total > 0 {
        let mut remaining = (((total - 1) as u32) >> 2) + 1;
        let mut at = vertices;
        while remaining != 0 {
            let x4 = load_vec4(e, at);
            let y4 = load_vec4(e, at + 0x10);
            let z4 = load_vec4(e, at + 0x20);
            let dot = add4(add4(mul4(y4, dy), mul4(dx, x4)), mul4(z4, dz));
            let mut next_best = best;
            let mut next_number = best_number;
            for lane in 0..4 {
                if best[lane] < dot[lane] {
                    next_best[lane] = dot[lane];
                    next_number[lane] = number[lane];
                }
            }
            number = add4(number, step);
            at = at.wrapping_add(0x30);
            best = next_best;
            best_number = next_number;
            remaining -= 1;
        }
    }
    let lane_wins = |winner: f32, loser: f32| {
        test_parity_even(compare_flags(f64::from(winner), f64::from(loser)), 5)
    };
    let first = usize::from(lane_wins(best[1], best[0]));
    let second = if lane_wins(best[3], best[2]) { 3 } else { 2 };
    let winner = if lane_wins(best[second], best[first]) {
        second
    } else {
        first
    };
    let index = e.call(FTOL, &args![f64::from(best_number[winner])]).u32();
    let block = (index >> 2).wrapping_mul(0x30).wrapping_add(vertices);
    let lane = block.wrapping_add((index & 3) * 4);
    let vertex = [
        e.mem.f32(lane),
        e.mem.f32(lane + 0x10),
        e.mem.f32(lane + 0x20),
        e.mem.f32(ZERO_SCALAR),
    ];
    store_vec4(e, out, vertex);
    let tagged = index | 0x3f00_0000;
    e.mem.set_u32(out + 0xc, tagged);
    tagged
}

// Translated from 00dea980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `a`, `b`, `c`): a quality measure of the triangle
/// `a`, `b`, `c` (16-byte vectors). With `n` the cross product of the edges at `a` and
/// `len = sqrt(n.n)` (`SQRTSS`), the numerator is `sqrt(3) * (2 * len)` (`_CIsqrt` of the
/// double at `01021928`, the x87 product stored as a `float`) and the denominator the sum
/// of the three squared edge lengths (`|a-b|^2 + |b-c|^2 + |c-a|^2`, lanes 0 to 2 in the
/// code's order). When the threshold at `0104da4c` is below the denominator (an x87
/// compare: a NaN or a smaller denominator fails) the answer is numerator / denominator,
/// else 0.0 (both in `ST0`).
pub fn fn_00dea980(e: &mut Engine, a: u32, b: u32, c: u32) -> f32 {
    let three = e.mem.f64(THREE_DOUBLE);
    let va = load_vec4(e, a);
    let vb = load_vec4(e, b);
    let vc = load_vec4(e, c);
    let edge_ab = sub4(vb, va);
    let edge_ac = sub4(vc, va);
    let normal = triangle_normal(edge_ab, edge_ac);
    let length = sum3(mul4(normal, normal)).sqrt();
    let root = x87_to_float(e.call(X87_SQRT, &args![three]).f64());
    let mut x = X87::new();
    x.fld_f32(root);
    x.fld_f32(length);
    x.add(0, 0);
    x.mulp(1);
    let numerator = x.fstp_f32();
    let ca_squared = x87_quiet(sum3(mul4(edge_ac, edge_ac)));
    let to_c = sub4(vb, vc);
    let bc_squared = x87_quiet(sum3(mul4(to_c, to_c)));
    let to_b = sub4(va, vb);
    let ab_squared = sum3(mul4(to_b, to_b));
    let total = sum3([ab_squared, bc_squared, ca_squared, 0.0]);
    let threshold = float_to_x87(e.mem.f32(EDGE_SUM_THRESHOLD));
    if !test_parity_even(compare_flags(threshold, float_to_x87(total)), 5) {
        x87_quiet(numerator / total)
    } else {
        0.0
    }
}

/// `hkArrayUtil::_reserve` (`00c908c0`: allocator, array, count, element size) and
/// `_reserveMore` (`00c90940`: allocator, array, element size), both cdecl.
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// A shape's bounding box (`00e11530`, `thiscall` on the shape, one word: the out box of 0x20
/// bytes, minimum then maximum), a random vector generator (`00d5d660`, `thiscall` on the
/// generator, one word: the vector to fill) and a shape's point query (`00e13660`, `thiscall`
/// on the shape: point, 16-byte out, byte out; answers a `float` in `ST0`).
const SHAPE_BOUNDS: u32 = 0x00e1_1530;
const RANDOM_VECTOR: u32 = 0x00d5_d660;
const SHAPE_POINT_QUERY: u32 = 0x00e1_3660;
/// The 16-byte global (`01267e30`) the sampling and the transposition start their vectors
/// from (all zero in the exe image).
const START_VECTOR: u32 = 0x0126_7e30;

// Translated from 00dedbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `shape`, `count`, `points`, `random`): rejection sampling
/// of `count` points inside `shape`. The shape's bounding box (`00e11530`) gives the minimum
/// and the extent (`max - min`). Each try starts a vector from the global at `01267e30`, has
/// the generator (`00d5d660`) fill it, makes the point `v * extent + min` and asks the shape
/// (`00e13660`: point, a 16-byte out, a byte flag that starts as 1; the `ST0` answer is
/// dropped). When the flag is 0 the try is rejected and the sequence of rejections is
/// counted; more than `count * 8` in a row (signed) ends the function. An accepted point gets
/// 1.0 in its fourth lane, is appended to the `hkArray` of 16-byte vectors `points` (grown
/// with `_reserveMore` when full) and the run of rejections starts again. Answers true when
/// all `count` points were found.
pub fn fn_00dedbb0(e: &mut Engine, shape: u32, count: u32, points: u32, random: u32) -> bool {
    let mut remaining = count;
    e.with_stack(0x20, |e, bounds| {
        e.call(SHAPE_BOUNDS, &args![shape, bounds]);
        let min = load_vec4(e, bounds.addr());
        let max = load_vec4(e, bounds.addr() + 0x10);
        let extent = sub4(max, min);
        let limit = count.wrapping_mul(8) as i32;
        let mut tries = 0i32;
        if remaining != 0 {
            e.with_stack(0x10, |e, vector| {
                let at = vector.addr();
                e.with_stack(4, |e, flag| {
                    e.with_stack(0x10, |e, result| loop {
                        for lane in 0..4 {
                            let word = e.mem.u32(START_VECTOR + 4 * lane);
                            e.mem.set_u32(at + 4 * lane, word);
                        }
                        e.mem.set_u8(flag.addr(), 1);
                        e.call(RANDOM_VECTOR, &args![random, vector]);
                        let sample = add4(mul4(load_vec4(e, at), extent), min);
                        store_vec4(e, at, sample);
                        e.call(SHAPE_POINT_QUERY, &args![shape, vector, result, flag]);
                        if e.mem.u8(flag.addr()) == 0 {
                            tries += 1;
                            if tries > limit {
                                break;
                            }
                        } else {
                            e.mem.set_f32(at + 0xc, 1.0);
                            let router = e.call(MEMORY_ROUTER_GETTER, &args![]).u32();
                            let capacity = e.mem.u32(points + 8) & 0x3fff_ffff;
                            if e.mem.u32(points + 4) == capacity {
                                let allocator = e.mem.u32(router + 0x10);
                                e.call(ARRAY_RESERVE_MORE, &args![allocator, points, 0x10u32]);
                            }
                            let size = e.mem.u32(points + 4);
                            let slot = (size << 4).wrapping_add(e.mem.u32(points));
                            remaining -= 1;
                            let accepted = load_vec4(e, at);
                            store_vec4(e, slot, accepted);
                            e.mem.set_u32(points + 4, size.wrapping_add(1));
                            tries = 0;
                        }
                        if remaining == 0 {
                            break;
                        }
                    });
                });
            });
        }
    });
    remaining == 0
}

// Translated from 00dedcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `vectors`, `blocks`): transposes the `hkArray` of
/// 16-byte vectors `vectors` into the `hkArray` of 0x30-byte blocks `blocks`, four vectors per
/// block: x of the four, then y, then z (the fourth lane is dropped). `blocks` first grows by
/// `(count + 3) / 4` blocks (`_reserve` to twice its capacity or the new size when larger,
/// element size 0x30). The four slots of the working buffer start as the global at `01267e30`
/// and keep their contents between blocks. A last group of fewer than four fills the missing
/// slots with its first vector before it is written.
pub fn fn_00dedcf0(e: &mut Engine, vectors: u32, blocks: u32) {
    let start = load_bits(e, START_VECTOR);
    let mut slots = [start; 4];
    let count = e.mem.i32(vectors + 4);
    let new_blocks = ((count.wrapping_add(3)) >> 2) as u32;
    let router = e.call(MEMORY_ROUTER_GETTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let old_size = e.mem.u32(blocks + 4);
    let wanted = old_size.wrapping_add(new_blocks) as i32;
    let capacity = (e.mem.u32(blocks + 8) & 0x3fff_ffff) as i32;
    if capacity < wanted {
        let mut grown = capacity.wrapping_add(capacity);
        if wanted >= grown {
            grown = wanted;
        }
        e.call(ARRAY_RESERVE, &args![allocator, blocks, grown, 0x30u32]);
    }
    let size = e.mem.u32(blocks + 4).wrapping_add(new_blocks);
    e.mem.set_u32(blocks + 4, size);
    let mut out = old_size
        .wrapping_mul(3)
        .wrapping_shl(4)
        .wrapping_add(e.mem.u32(blocks));
    let source_count = e.mem.i32(vectors + 4);
    if source_count <= 0 {
        return;
    }
    let transpose = |e: &mut Engine, slots: &[[u32; 4]; 4], out: u32| {
        for axis in 0..3 {
            for (lane, slot) in slots.iter().enumerate() {
                e.mem
                    .set_u32(out + 0x10 * axis + 4 * lane as u32, slot[axis as usize]);
            }
        }
    };
    let mut collected = 0usize;
    let mut offset = 0u32;
    let mut remaining = source_count;
    loop {
        let data = e.mem.u32(vectors);
        let vector = load_bits(e, data.wrapping_add(offset));
        slots[collected] = vector;
        collected += 1;
        if collected == 4 {
            collected = 0;
            out = out.wrapping_add(0x30);
            transpose(e, &slots, out.wrapping_sub(0x30));
        }
        offset = offset.wrapping_add(0x10);
        remaining -= 1;
        if remaining == 0 {
            break;
        }
    }
    if collected == 0 {
        return;
    }
    let first = slots[0];
    for slot in &mut slots[collected..] {
        *slot = first;
    }
    transpose(e, &slots, out);
}

/// The smallest value the parallel test accepts (`0104da4c`) and the largest `float`
/// (`0104da48`) the function answers when the segments are parallel.
const PARALLEL_EPSILON: u32 = 0x0104_da4c;
const LARGEST_DISTANCE: u32 = 0x0104_da48;

// Translated from 00dea450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `a`, `b`, `c`, `d`, `along_ab`, `along_cd`): the distance
/// between the segments `a` to `b` and `c` to `d` (16-byte vectors). With `d1 = b - a`,
/// `e1 = d - c` and `w0 = c - a` the code forms `a = d1.d1`, `b = e1.d1`, `p = w0.d1`,
/// `q = e1.w0` and `e = e1.e1` (lanes 0 to 2 summed in its order), `denominator = a * e - b * b`
/// (x87, stored as a `float`), and when its magnitude is not above the epsilon at `0104da4c`
/// (a compare in which a NaN counts as parallel) stores 0.0 at both outputs and answers the
/// largest `float` (`0104da48`). Otherwise `s = (b * p - a * q) / denominator` is clamped to
/// [0, 1] (below 0 gives 0, above 1 gives 1, a NaN stays) and stored at `along_cd`, `Q = c +
/// s * e1`, `t = (0 - (a - Q).d1 / a)` is clamped the same way and stored at `along_ab`,
/// `P = a + t * d1`, and the answer is `sqrt(|P - Q|^2)` (`SQRTSS`), in `ST0`.
pub fn fn_00dea450(
    e: &mut Engine,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    along_ab: u32,
    along_cd: u32,
) -> f32 {
    let va = load_vec4(e, a);
    let vb = load_vec4(e, b);
    let vc = load_vec4(e, c);
    let vd = load_vec4(e, d);
    let e1 = sub4(vd, vc);
    let w0 = sub4(vc, va);
    let d1 = sub4(vb, va);
    let b_dot = sum3(mul4(e1, d1));
    let p_dot = sum3(mul4(w0, d1));
    let q_dot = sum3(mul4(e1, w0));
    let a_dot = sum3(mul4(d1, d1));
    let e_dot = sum3(mul4(e1, e1));
    let mut x = X87::new();
    x.fld_f32(b_dot);
    x.fld_st(0);
    x.fld_f32(a_dot);
    x.fld_st(0);
    x.mul_m(float_to_x87(e_dot));
    x.fld_st(2);
    x.mulp(3);
    x.subrp(2);
    x.fxch(1);
    let denominator = x.fstp_f32();
    x.fld_f32(denominator);
    x.fld_st(0);
    x.abs();
    let magnitude = x.fstp_f32();
    x.fld_f32(magnitude);
    let flags = x.fcom_m(float_to_x87(e.mem.f32(PARALLEL_EPSILON)));
    x.pop();
    if flags & 0x41 != 0 {
        x.fstp_st(2);
        x.pop();
        x.pop();
        e.mem.set_f32(along_cd, 0.0);
        e.mem.set_f32(along_ab, 0.0);
        x.fld_f32(e.mem.f32(LARGEST_DISTANCE));
        let answer = x.fstp_f32();
        assert!(x.is_empty());
        return x87_quiet(answer);
    }
    x.fld_f32(q_dot);
    x.mulp(2);
    x.fld_f32(p_dot);
    x.mulp(3);
    x.fxch(1);
    x.subrp(2);
    x.fxch(1);
    let numerator = x.fstp_f32();
    x.fld_f32(numerator);
    x.chs();
    x.divrp(1);
    let unclamped = x.fstp_f32();
    x.fldz();
    x.fld_f32(unclamped);
    let below = x.fcom(1);
    x.fld1();
    let s = if !test_parity_even(below, 5) {
        x.fstp_st(1);
        x.fxch(1);
        x.fst_f32()
    } else {
        let above = x.fcom(1);
        if !test_parity_even(above, 5) {
            x.fstp_st(1);
            let value = x.fst_f32();
            x.fxch(1);
            value
        } else {
            x.fxch(1);
            let value = x.fstp_f32();
            x.fxch(1);
            value
        }
    };
    x.fld_f32(s);
    let stored_s = x.fstp_f32();
    e.mem.set_f32(along_cd, stored_s);
    let moved = mul4([stored_s; 4], e1);
    let q_point = add4(moved, vc);
    let to_q = sub4(va, q_point);
    let f = sum3(mul4(to_q, d1));
    let t_unclamped = e.mem.f32(ZERO_VECTOR) - f / a_dot;
    x.fld_f32(t_unclamped);
    let below = x.fcom(1);
    if !test_parity_even(below, 5) {
        x.pop();
        x.fstp_st(1);
    } else {
        x.fstp_st(1);
        let above = x.fcom(1);
        if above & 0x41 != 0 {
            x.fstp_st(1);
        } else {
            x.pop();
        }
    }
    let t = x.fstp_f32();
    x.fld_f32(t);
    let stored_t = x.fstp_f32();
    e.mem.set_f32(along_ab, stored_t);
    assert!(x.is_empty(), "the x87 stack is empty before the answer");
    let p_point = add4(mul4([stored_t; 4], d1), va);
    let gap = sub4(p_point, q_point);
    x87_quiet(sum3(mul4(gap, gap)).sqrt())
}

/// `0.5`, `3.0` and `1.0` as `float`s (`01016248`, `01017718`, `01012068`), `-1.0`
/// (`01012054`) and `1/3` (`0106c2e0`, used for the barycentric fallback; `THIRD_SCALAR`).
const REFINE_HALF: u32 = 0x0101_6248;
const REFINE_THREE: u32 = 0x0101_7718;
const ONE_FLOAT: u32 = 0x0101_2068;

/// `RSQRTSS` refined once the way the code does (`(0.5 * r) * (3 - (s * r) * r)`, 0 when `s`
/// equals the exe's zero), `r` being the hardware estimate.
pub(crate) fn refined_inverse_length(squared: f32, zero: f32, half: f32, three: f32) -> f32 {
    let estimate = rsqrt_estimate(squared);
    let step = three - (squared * estimate) * estimate;
    let value = (half * estimate) * step;
    if squared == zero {
        0.0
    } else {
        value
    }
}

// Translated from 00de9f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryProcessing::squaredDistanceFromPointToTriangle` (Xbox PDB), cdecl (`point`, `a`,
/// `b`, `c`, `out`, `barycentric`): the squared distance from `point` to the triangle
/// `a`, `b`, `c` (16-byte vectors). The plane normal `u` is the triangle's cross product
/// scaled by the refined `RSQRTSS` of its squared length (0 for a zero length), the plane's
/// fourth lane is `0 - u.a` (stored through the x87 stack) and the signed distance of the
/// point is `u.p + d` (lanes 0 to 2 summed in the code's order). The point is projected onto
/// the plane (`q = p - distance * u`) and the three edge terms of `q` are tested against the
/// plane (each must be strictly above 0, x87 compares).
///
/// Inside: the plane `(u, d)` is stored at `out`, negated when the distance is below 0,
/// the answer is the distance squared, and when `barycentric` is not null it receives the
/// three areas `|cq x bq|`, `|cq x aq|`, `|bq x aq|` divided by their sum (a vector of 1/3 when
/// the sum is not above the epsilon at `0104da4c`, or a NaN) in lanes 0 to 2 (lane 3 is
/// the exe's zero). Outside: the closest point is found with `00de9e10` on each of the
/// three edges (parameters kept at local words), the smallest squared distance wins (later
/// edges win ties and NaNs, the x87 compares as in the exe), `out` receives that edge's
/// offset vector and `barycentric` the edge's weights (`[1 - t, t, 0, 0]`, `[0, 1 - t, t, 0]`
/// or `[t, 0, 1 - t, 0]`); the answer is that edge's squared distance.
pub fn hk_geometry_processing_squared_distance_from_point_to_triangle(
    e: &mut Engine,
    point: u32,
    a: u32,
    b: u32,
    c: u32,
    out: u32,
    barycentric: u32,
) -> f32 {
    let va = load_vec4(e, a);
    let vb = load_vec4(e, b);
    let vc = load_vec4(e, c);
    let ac = sub4(vc, va);
    let ab = sub4(vb, va);
    let normal = triangle_normal(ab, ac);
    let squared = sum3(mul4(normal, normal));
    let zero = e.mem.f32(ZERO_SCALAR);
    let inverse = refined_inverse_length(
        squared,
        zero,
        e.mem.f32(REFINE_HALF),
        e.mem.f32(REFINE_THREE),
    );
    let vp = load_vec4(e, point);
    let unit = mul4([inverse; 4], normal);
    let offset = e.mem.f32(ZERO_VECTOR) - sum3(mul4(unit, va));
    let mut x = X87::new();
    x.fld_f32(offset);
    let offset_lane = x.fstp_f32();
    let plane = [unit[0], unit[1], unit[2], offset_lane];
    let plane_sum = {
        let products = mul4(plane, vp);
        let low = products[1] + products[0];
        (products[2] + low) + plane[3]
    };
    x.fld_f32(plane_sum);
    x.fld_st(0);
    x.chs();
    let negative = x.fstp_f32();
    x.fldz();
    let projected = add4(mul4([negative; 4], plane), vp);
    let a_q = sub4(va, projected);
    let b_q = sub4(vb, projected);
    let c_q = sub4(vc, projected);
    let edge_terms = [
        edge_term(b_q, a_q, plane),
        edge_term(c_q, b_q, plane),
        edge_term(a_q, c_q, plane),
    ];
    let inside = edge_terms.iter().all(|term| 0.0 < float_to_x87(*term));
    if inside {
        store_vec4(e, out, plane);
        if barycentric != 0 {
            let area = |left: Vec4, right: Vec4| {
                let cross = sub4(
                    mul4(rotate_d2(left), rotate_c9(right)),
                    mul4(rotate_c9(left), rotate_d2(right)),
                );
                sum3(mul4(cross, cross)).sqrt()
            };
            // The lanes are [|cq x bq|, |cq x aq|, |bq x aq|, 0].
            let weights = [area(c_q, b_q), area(a_q, c_q), area(b_q, a_q), zero];
            let total = sum3(weights);
            let epsilon = x87_quiet(e.mem.f32(PARALLEL_EPSILON));
            let flags = compare_flags(float_to_x87(epsilon), float_to_x87(total));
            if !test_parity_even(flags, 5) {
                let inverse = e.mem.f32(ONE_FLOAT) / total;
                store_vec4(e, barycentric, mul4([inverse; 4], weights));
            } else {
                let third = e.mem.f32(THIRD_SCALAR);
                store_vec4(e, barycentric, [third, third, third, zero]);
            }
        }
        let flags = x.fcom(1);
        x.pop();
        if (flags & 0x41) == 0 {
            let minus_one = [e.mem.f32(MINUS_ONE_FLOAT); 4];
            let flipped = mul4(minus_one, load_vec4(e, out));
            store_vec4(e, out, flipped);
        }
        x.mul(0, 0);
        return x87_quiet(x.fstp_f32());
    }
    x.pop();
    x.pop();
    // Outside: the closest point on each edge (the exe keeps the three parameters at
    // words of its frame and the three squared distances next to them).
    e.with_stack(0x60, |e, frame| {
        let base = frame.addr();
        let edges = [
            (a, b, 0x10u32, 0x30u32),
            (b, c, 0x14, 0x40),
            (c, a, 0x18, 0x50),
        ];
        let mut squared_distances = [0.0f32; 3];
        for (index, (from, to, fraction, vector)) in edges.into_iter().enumerate() {
            let distance = fn_00de9e10(e, point, from, to, base + vector, base + fraction);
            squared_distances[index] = x87_to_float(float_to_x87(distance));
        }
        let [d1, d2, d3] = squared_distances.map(float_to_x87);
        let closest = if (compare_flags(d2, d1) & 0x41) != 0 {
            if (compare_flags(d3, d2) & 0x41) == 0 {
                1
            } else {
                2
            }
        } else if (compare_flags(d3, d1) & 0x41) != 0 {
            2
        } else {
            0
        };
        let offset_vector = load_vec4(e, base + 0x30 + 0x10 * closest as u32);
        store_vec4(e, out, offset_vector);
        if barycentric != 0 {
            let zero = e.mem.f32(ZERO_SCALAR);
            let fraction = |e: &Engine, slot: u32| e.mem.f32(base + slot);
            let complement = |value: f32| x87_to_float(1.0 - float_to_x87(value));
            let weights = match closest {
                2 => {
                    let t = fraction(e, 0x18);
                    Some([t, zero, complement(t), zero])
                }
                1 => {
                    let t = fraction(e, 0x14);
                    Some([zero, complement(t), t, zero])
                }
                0 => {
                    let t = fraction(e, 0x10);
                    Some([complement(t), t, zero, zero])
                }
                _ => None,
            };
            if let Some(weights) = weights {
                store_vec4(e, barycentric, weights);
            }
        }
        x87_quiet(squared_distances[closest])
    })
}

/// The function-local static the code builds on its first call (`0126a420`: the vtable
/// `010d9ff4`, then the `float` `1.0` at `0126a424`; the guard dword is at `0126a428`, bit 0),
/// its `atexit` registration (`00ec658f`, one word: `00fde2c0`), and the routines `00dedf10`
/// calls on its local objects (all unconfirmed and called as the code does).
const CONST_FUNCTION_STATIC: u32 = 0x0126_a420;
const CONST_FUNCTION_GUARD: u32 = 0x0126_a428;
const CONST_FUNCTION_VTABLE: u32 = 0x010d_9ff4;
const REGISTER_AT_EXIT: u32 = 0x00ec_658f;
const STATIC_DESTRUCTOR: u32 = 0x00fd_e2c0;
const SHAPE_CONSTRUCTOR: u32 = 0x00e1_b750;
const SHAPE_SET_UP: u32 = 0x00e1_ce10;
const SHAPE_PREPARE: u32 = 0x00e1_1e10;
const SHAPE_FINISH: u32 = 0x00e1_5e50;
const SHAPE_DESTRUCTOR: u32 = 0x00e1_8a50;
const BOX_TREE_CONSTRUCTOR: u32 = 0x00d9_c110;
const BOX_TREE_BUILD: u32 = 0x00d9_d650;
const BOX_TREE_UPDATE: u32 = 0x00d9_be50;
const BOX_TREE_QUERY: u32 = 0x00d1_dbb0;
const BOX_TREE_DESTRUCTOR: u32 = 0x00d9_be30;
const BOX_TREE_MEMBER_DESTRUCTOR: u32 = 0x00d9_c010;
const QUERY_VTABLE_FIRST: u32 = 0x010c_d684;
const QUERY_VTABLE: u32 = 0x010d_9fd0;
const BOX_TREE_VTABLE: u32 = 0x010d_40a4;

/// Reserves room for one more element in the `hkArray` at `array` when it is full (the size
/// equals the capacity bits): `_reserveMore` with the router's allocator.
fn reserve_one_more(e: &mut Engine, array: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER_GETTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, element_size]);
    }
}

/// Gives back the memory of the `hkArray` at `array` through the router's allocator (slot
/// `0x10`) when the capacity word says it owns it (bit 31 clear), the bytes being
/// `(capacity & 0x3fffffff) * element_size`; the size becomes 0, the data pointer 0 and the
/// capacity word `0x80000000` (the order of the exe's stores: size, free, data, capacity).
fn release_array(e: &mut Engine, array: u32, element_shift: u32) {
    let router = e.call(MEMORY_ROUTER_GETTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let flags = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if flags & 0x8000_0000 == 0 {
        let data = e.mem.u32(array);
        let bytes = (flags & 0x3fff_ffff) << element_shift;
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

// Translated from 00dedf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `shape_source`, `count`, `points`, `iterations`, `seed`,
/// `function`; answers a byte). A weighted relaxation of sample points. The first call builds
/// the function-local static at `0126a420` (a constant function of `1.0`, registered for
/// destruction with `00ec658f`); a null `function` means that object. A local shape object
/// (`00e1b750`, then `00e1ce10(shape_source, 0x01268370, -1)`, `00e11e10`, `00e15e50`) takes
/// `count` random points inside it (`00dedbb0` with the generator state `{seed, seed}`),
/// appended to the `hkArray` of vectors `points`; failure answers 0. With `iterations` above 0
/// each new point is weighted by `function->evaluate(point)` (virtual slot 4: the point is
/// scaled by `w` and lane 3 becomes `w`) and put, as a box `[p - 0, p + 0]` (the zero constant
/// `01011d78` splat), into a local `hkArray` of 32-byte boxes with a pointer to the point in a
/// second array of words; a tree (`00d9c110`, built by `00d9d650` over the boxes and
/// pointers, with a local array of handles) holds them. Then each iteration draws `clamp(2 *
/// count, 0x100, 0x2000)` random points (`00dedbb0` again, into a local array), finds for
/// each the nearest box (`00d1dbb0`, the query object whose vtable is `010d9fd0` and whose
/// distance starts at the largest `float`), weights it the same way and adds it to the point
/// that box belongs to; then every point becomes its sum divided by its weight (lane 3 times
/// `1 / w`, through `FDIVRP`), is put back in the tree (`00d9be50`) and weighted again. Every
/// local array is given back (`0x10`, `4`, `0x20` and `4` bytes per element), the tree and
/// the shape are destroyed. Answers 1; a failing `00dedbb0` inside the loop gives all of it back
/// first and answers 0.
pub fn fn_00dedf10(
    e: &mut Engine,
    shape_source: u32,
    count: i32,
    points: u32,
    iterations: i32,
    seed: u32,
    function: u32,
) -> bool {
    if e.mem.u8(CONST_FUNCTION_GUARD) & 1 == 0 {
        let guard = e.mem.u32(CONST_FUNCTION_GUARD) | 1;
        e.mem.set_u32(CONST_FUNCTION_GUARD, guard);
        e.mem.set_f32(CONST_FUNCTION_STATIC + 4, 1.0);
        e.mem.set_u32(CONST_FUNCTION_STATIC, CONST_FUNCTION_VTABLE);
        e.call(REGISTER_AT_EXIT, &args![STATIC_DESTRUCTOR]);
    }
    let function = if function == 0 {
        CONST_FUNCTION_STATIC
    } else {
        function
    };
    e.with_stack(0x240, |e, frame| {
        let s = frame.addr();
        let seed_state = s + 0x58;
        let shape = s + 0x130;
        let tree = s + 0xb0;
        let handles = s + 0x44;
        let boxes = s + 0x28;
        let pointers = s + 0x34;
        let samples = s + 0x14;
        let query = s + 0x60;
        e.mem.set_u32(seed_state, seed);
        e.mem.set_u32(seed_state + 4, seed);
        e.call(SHAPE_CONSTRUCTOR, &args![shape]);
        e.call(
            SHAPE_SET_UP,
            &args![shape, shape_source, 0x0126_8370u32, 0xffff_ffffu32],
        );
        e.call(SHAPE_PREPARE, &args![shape]);
        e.call(SHAPE_FINISH, &args![shape]);
        let old_size = e.mem.u32(points + 4);
        if !fn_00dedbb0(e, shape, count as u32, points, seed_state) {
            e.call(SHAPE_DESTRUCTOR, &args![shape]);
            return false;
        }
        if iterations <= 0 {
            e.call(SHAPE_DESTRUCTOR, &args![shape]);
            return true;
        }
        let first_new = e.mem.u32(points).wrapping_add(old_size << 4);
        e.mem.set_u32(s + 0x54, first_new);
        e.mem.set_u32(handles, 0);
        e.mem.set_u32(handles + 4, 0);
        e.mem.set_u32(handles + 8, 0x8000_0000);
        e.call(BOX_TREE_CONSTRUCTOR, &args![tree]);
        let router = e.call(MEMORY_ROUTER_GETTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(handles + 8) & 0x3fff_ffff) as i32;
        if capacity < count {
            let doubled = capacity.wrapping_add(capacity);
            let wanted = if count < doubled { doubled } else { count };
            e.call(ARRAY_RESERVE, &args![allocator, handles, wanted, 4u32]);
        }
        e.mem.set_i32(handles + 4, count);
        for array in [boxes, pointers] {
            e.mem.set_u32(array, 0);
            e.mem.set_u32(array + 4, 0);
            e.mem.set_u32(array + 8, 0x8000_0000);
        }
        let zero_splat = [e.mem.f32(ZERO_SCALAR); 4];
        if count > 0 {
            let mut point = first_new;
            for _ in 0..count {
                let p = load_vec4(e, point);
                let lower = sub4(p, zero_splat);
                let mut x = X87::new();
                x.fldz();
                store_vec4(e, query, lower);
                e.mem.set_f32(query + 0xc, x.fst_f32());
                let upper = add4(p, zero_splat);
                store_vec4(e, query + 0x10, upper);
                e.mem.set_f32(query + 0x1c, x.fstp_f32());
                reserve_one_more(e, boxes, 0x20);
                let target = e.mem.u32(boxes).wrapping_add(e.mem.u32(boxes + 4) << 5);
                if target != 0 {
                    for word in 0..8u32 {
                        let value = e.mem.u32(query + 4 * word);
                        e.mem.set_u32(target + 4 * word, value);
                    }
                }
                let size = e.mem.u32(boxes + 4);
                e.mem.set_u32(boxes + 4, size.wrapping_add(1));
                reserve_one_more(e, pointers, 4);
                let size = e.mem.u32(pointers + 4);
                let slot = e.mem.u32(pointers).wrapping_add(size << 2);
                e.mem.set_u32(slot, point);
                e.mem.set_u32(pointers + 4, size.wrapping_add(1));
                let weight = x87_quiet(e.vcall(function, 4, &args![point]).f32());
                let scaled = mul4([weight; 4], load_vec4(e, point));
                store_vec4(e, point, scaled);
                e.mem.set_f32(point + 0xc, weight);
                point = point.wrapping_add(0x10);
            }
        }
        let box_data = e.mem.u32(boxes);
        let box_count = e.mem.u32(boxes + 4);
        let pointer_data = e.mem.u32(pointers);
        e.call(
            BOX_TREE_BUILD,
            &args![tree, box_data, pointer_data, box_count, handles],
        );
        let doubled_count = count.wrapping_add(count);
        let want = doubled_count.clamp(0x100, 0x2000);
        e.mem.set_u32(samples, 0);
        e.mem.set_u32(samples + 4, 0);
        e.mem.set_u32(samples + 8, 0x8000_0000);
        let router = e.call(MEMORY_ROUTER_GETTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = (e.mem.u32(samples + 8) & 0x3fff_ffff) as i32;
        if capacity < want {
            e.call(
                ARRAY_RESERVE,
                &args![allocator, samples, want as u32, 0x10u32],
            );
        }
        let mut failed = false;
        for _ in 0..iterations {
            e.mem.set_u32(samples + 4, 0);
            if !fn_00dedbb0(e, shape, want as u32, samples, seed_state) {
                failed = true;
                break;
            }
            let drawn = e.mem.i32(samples + 4);
            let mut offset = 0u32;
            for _ in 0..drawn.max(0) {
                let sample = e.mem.u32(samples).wrapping_add(offset);
                let mut x = X87::new();
                x.fld_f32(e.mem.f32(LARGEST_DISTANCE));
                e.mem.set_u32(query, QUERY_VTABLE_FIRST);
                for word in 0..3u32 {
                    let value = e.mem.u32(sample + 4 * word);
                    e.mem.set_u32(query + 0x10 + 4 * word, value);
                }
                let lane3 = e.mem.u32(sample + 0xc);
                e.mem.set_f32(query + 0x20, x.fstp_f32());
                e.mem.set_u32(query + 0x1c, lane3);
                e.mem.set_u32(query, QUERY_VTABLE);
                e.mem.set_u32(query + 0x30, 0);
                e.call(BOX_TREE_QUERY, &args![tree, sample, query]);
                let found = e.mem.u32(query + 0x30);
                if found != 0 {
                    let target = e.mem.u32(found + 0x20);
                    let weight = x87_quiet(e.vcall(function, 4, &args![sample]).f32());
                    let scaled = mul4([weight; 4], load_vec4(e, sample));
                    store_vec4(e, sample, scaled);
                    e.mem.set_f32(sample + 0xc, weight);
                    let sum = add4(load_vec4(e, sample), load_vec4(e, target));
                    store_vec4(e, target, sum);
                }
                offset = offset.wrapping_add(0x10);
            }
            if count > 0 {
                let mut point = first_new;
                for index in 0..count {
                    let total_weight = e.mem.f32(point + 0xc);
                    let mut x = X87::new();
                    x.fld_f32(total_weight);
                    x.fld1();
                    x.divrp(1);
                    let inverse = x.fstp_f32();
                    let normalised = mul4([inverse; 4], load_vec4(e, point));
                    store_vec4(e, point, normalised);
                    let p = load_vec4(e, point);
                    store_vec4(e, query, sub4(p, zero_splat));
                    e.mem.set_f32(query + 0xc, 0.0);
                    store_vec4(e, query + 0x10, add4(p, zero_splat));
                    e.mem.set_f32(query + 0x1c, 0.0);
                    let handle = e
                        .mem
                        .u32(e.mem.u32(handles).wrapping_add((index as u32) << 2));
                    e.call(BOX_TREE_UPDATE, &args![tree, handle, query]);
                    let weight = x87_quiet(e.vcall(function, 4, &args![point]).f32());
                    let scaled = mul4([weight; 4], load_vec4(e, point));
                    store_vec4(e, point, scaled);
                    e.mem.set_f32(point + 0xc, weight);
                    point = point.wrapping_add(0x10);
                }
            }
        }
        release_array(e, samples, 4);
        release_array(e, pointers, 2);
        release_array(e, boxes, 5);
        e.mem.set_u32(tree, BOX_TREE_VTABLE);
        e.call(BOX_TREE_DESTRUCTOR, &args![tree]);
        e.call(BOX_TREE_MEMBER_DESTRUCTOR, &args![tree + 0x1c]);
        release_array(e, handles, 2);
        e.call(SHAPE_DESTRUCTOR, &args![shape]);
        !failed
    })
}

/// The cross product the separating-axis tests of `00deab10` build from two edge vectors, in
/// the order of the code's `SHUFPS` rotations: `rot_d2(p) * rot_c9(q) - rot_c9(p) * rot_d2(q)`.
fn axis_from_edges(p: Vec4, q: Vec4) -> Vec4 {
    sub4(
        mul4(rotate_d2(p), rotate_c9(q)),
        mul4(rotate_c9(p), rotate_d2(q)),
    )
}

/// The range (minimum, maximum) of the projections of the three vertices on `axis`, as the
/// code keeps it: both start at the projection of vertex 0 (the lane 0 of the three products
/// summed lanes 2, 1, 0); each further vertex replaces the minimum when the minimum is above
/// its projection (an `FCOMPP`, a NaN does not replace it) and otherwise the maximum when
/// the maximum is below it (a NaN does not replace it either).
fn projection_range(vertices: &[Vec4; 3], axis: Vec4) -> (f32, f32) {
    let project = |v: Vec4| sum3(mul4(v, axis));
    let first = project(vertices[0]);
    let (mut minimum, mut maximum) = (first, first);
    for vertex in &vertices[1..] {
        let projection = project(*vertex);
        if compare_flags(float_to_x87(minimum), float_to_x87(projection)) & 0x41 == 0 {
            minimum = projection;
        } else if !test_parity_even(
            compare_flags(float_to_x87(maximum), float_to_x87(projection)),
            5,
        ) {
            maximum = projection;
        }
    }
    (minimum, maximum)
}

// Translated from 00deab10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryProcessing::checkTrianglesIntersection` (Xbox PDB), cdecl (`a`, `b`): `a` and `b`
/// each point at a triangle (three 16-byte vertices `a0`, `a1`, `a2` and `b0`, `b1`, `b2`). A
/// separating-axis test with twenty axes, in the order of the code: the two face normals
/// (`(a2 - a0) x (a1 - a0)`, `(b2 - b0) x (b1 - b0)`), the nine pairs of an edge of `a` and an
/// edge of `b` (`a1 - a0`, `a2 - a1`, `a2 - a0` against `b1 - b0`, `b2 - b1`, `b2 - b0`, built
/// with the code's rotations) and the nine vertex differences `a_i - b_j` (i, then j). For each
/// axis the projections of the vertices give the ranges of both triangles; when the
/// maximum of `b` is below the minimum of `a`, or the minimum of `b` above the maximum of `a`
/// (ordered compares: a NaN never separates), the triangles are apart and the answer is 0. When
/// no axis separates them the answer is 1.
pub fn hk_geometry_processing_check_triangles_intersection(e: &mut Engine, a: u32, b: u32) -> bool {
    let va = [0, 1, 2].map(|i| load_vec4(e, a + 0x10 * i));
    let vb = [0, 1, 2].map(|i| load_vec4(e, b + 0x10 * i));
    let edge_a = |from: usize, to: usize| sub4(va[to], va[from]);
    let edge_b = |from: usize, to: usize| sub4(vb[to], vb[from]);
    let mut axes = Vec::with_capacity(20);
    axes.push(axis_from_edges(edge_a(0, 2), edge_a(0, 1)));
    axes.push(axis_from_edges(edge_b(0, 2), edge_b(0, 1)));
    axes.push(axis_from_edges(edge_b(0, 1), edge_a(0, 1)));
    axes.push(axis_from_edges(edge_b(1, 2), edge_a(0, 1)));
    axes.push(axis_from_edges(edge_a(0, 1), edge_b(0, 2)));
    axes.push(axis_from_edges(edge_a(1, 2), edge_b(1, 0)));
    axes.push(axis_from_edges(edge_a(1, 2), edge_b(2, 1)));
    axes.push(axis_from_edges(edge_a(1, 2), edge_b(0, 2)));
    axes.push(axis_from_edges(edge_a(0, 2), edge_b(0, 1)));
    axes.push(axis_from_edges(edge_a(0, 2), edge_b(1, 2)));
    axes.push(axis_from_edges(edge_a(0, 2), edge_b(2, 0)));
    for i in 0..3 {
        for j in 0..3 {
            axes.push(sub4(va[i], vb[j]));
        }
    }
    for axis in axes {
        let (minimum_a, maximum_a) = projection_range(&va, axis);
        let (minimum_b, maximum_b) = projection_range(&vb, axis);
        if !test_parity_even(
            compare_flags(float_to_x87(maximum_b), float_to_x87(minimum_a)),
            5,
        ) {
            return false;
        }
        if compare_flags(float_to_x87(minimum_b), float_to_x87(maximum_a)) & 0x41 == 0 {
            return false;
        }
    }
    true
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de94b0, hk_geometry_processing_squared_distance_from_point_to_line(u32, u32, u32, u32) -> f32),
        entry!(0x00de9590, fn_00de9590(Ptr)),
        entry!(0x00de9b10, fn_00de9b10(u32, u32, u32)),
        entry!(0x00de9b50, fn_00de9b50(u32, u32, u32, u32)),
        entry!(0x00de9b90, fn_00de9b90(u32, u32, u32, u32, u32)),
        entry!(0x00dee620, hk_geometry_processing_i_function_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00dee640, hk_geometry_processing_internals_nearest_feature_policy_add_leaf(Ptr, u32) -> f32),
        entry!(0x00dee6b0, hk_geometry_processing_const_function_evaluate(Ptr, u32) -> f32),
        entry!(0x00dee6c0, hk_geometry_processing_internals_nearest_feature_policy_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00de9650, fn_00de9650(u32, u32)),
        entry!(0x00de98c0, fn_00de98c0(u32, u32)),
        entry!(0x00de9e10, fn_00de9e10(u32, u32, u32, u32, u32) -> f32),
        entry!(0x00dea6b0, hk_geometry_processing_check_point_inside_triangle(u32, u32, u32, u32, f32) -> bool),
        entry!(0x00dea810, fn_00dea810(u32, u32, u32) -> u32),
        entry!(0x00dea980, fn_00dea980(u32, u32, u32) -> f32),
        entry!(0x00dedbb0, fn_00dedbb0(u32, u32, u32, u32) -> bool),
        entry!(0x00dedcf0, fn_00dedcf0(u32, u32)),
        entry!(0x00dea450, fn_00dea450(u32, u32, u32, u32, u32, u32) -> f32),
        entry!(0x00de9f10, hk_geometry_processing_squared_distance_from_point_to_triangle(u32, u32, u32, u32, u32, u32) -> f32),
        entry!(0x00dedf10, fn_00dedf10(u32, i32, u32, i32, u32, u32) -> bool),
        entry!(0x00deab10, hk_geometry_processing_check_triangles_intersection(u32, u32) -> bool),
    ]
}
#[cfg(test)]
mod tests {
    use super::*;

    fn constants(e: &mut Engine) {
        for page in [
            0x010c_7000,
            0x0101_7000,
            0x0101_1000,
            0x0101_6000,
            0x010e_c000,
            0x0106_c000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.write(HALVES, &[0, 0, 0, 0x3f].repeat(4));
        e.mem.write(ABS_MASK, &[0xff, 0xff, 0xff, 0x7f].repeat(4));
        e.mem.write(ONES, &[0, 0, 0x80, 0x3f].repeat(4));
        e.mem.set_f32(BLEND_SCALAR, 0.25);
        e.mem.set_f32(THIRD_SCALAR, 0.5);
        e.mem.set_f32(QUARTER_SCALAR, 0.25);
    }

    fn vector(e: &mut Engine, lanes: Vec4) -> u32 {
        let at = e.mem.alloc(16);
        store_vec4(e, at, lanes);
        at
    }

    #[test]
    fn the_distance_to_a_segment_clamps_the_parameter() {
        let mut e = Engine::new();
        constants(&mut e);
        let start = vector(&mut e, [0.0; 4]);
        let end = vector(&mut e, [2.0, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(4);
        let distance = |e: &mut Engine, point: Vec4| {
            let at = vector(e, point);
            e.call(0x00de_94b0, &args![at, start, end, out]).f32()
        };
        assert_eq!(distance(&mut e, [1.0, 1.0, 0.0, 0.0]), 1.0);
        assert_eq!(e.mem.f32(out), 0.5);
        assert_eq!(distance(&mut e, [5.0, 0.0, 0.0, 0.0]), 9.0);
        assert_eq!(e.mem.f32(out), 1.0);
        assert_eq!(distance(&mut e, [-3.0, 0.0, 0.0, 0.0]), 9.0);
        assert_eq!(e.mem.f32(out), 0.0);
        // A degenerate segment divides 0 by 0: the parameter stays NaN.
        let same = vector(&mut e, [1.0; 4]);
        let at = vector(&mut e, [1.0; 4]);
        let result = e.call(0x00de_94b0, &args![at, same, same, out]).f32();
        assert!(e.mem.f32(out).is_nan());
        assert!(result.is_nan());
    }

    #[test]
    fn the_box_becomes_a_cube_along_its_longest_axis() {
        let mut e = Engine::new();
        constants(&mut e);
        let bounds = e.mem.alloc(0x20);
        store_vec4(&mut e, bounds, [0.0; 4]);
        store_vec4(&mut e, bounds + 0x10, [2.0, 6.0, 4.0, 0.0]);
        e.call(0x00de_9590, &args![bounds]);
        assert_eq!(load_vec4(&e, bounds), [-2.0, 0.0, -1.0, 0.0]);
        assert_eq!(load_vec4(&e, bounds + 0x10), [4.0, 6.0, 5.0, 0.0]);
        // The first axis wins a tie, the third wins when strictly larger.
        store_vec4(&mut e, bounds, [0.0; 4]);
        store_vec4(&mut e, bounds + 0x10, [4.0, 4.0, 2.0, 0.0]);
        e.call(0x00de_9590, &args![bounds]);
        assert_eq!(load_vec4(&e, bounds), [0.0, 0.0, -1.0, 0.0]);
        assert_eq!(load_vec4(&e, bounds + 0x10), [4.0, 4.0, 3.0, 0.0]);
        store_vec4(&mut e, bounds, [0.0; 4]);
        store_vec4(&mut e, bounds + 0x10, [2.0, 2.0, 8.0, 0.0]);
        e.call(0x00de_9590, &args![bounds]);
        assert_eq!(load_vec4(&e, bounds), [-3.0, -3.0, 0.0, 0.0]);
        assert_eq!(load_vec4(&e, bounds + 0x10), [5.0, 5.0, 8.0, 0.0]);
    }

    #[test]
    fn the_blend_and_the_averages_use_their_scalars() {
        let mut e = Engine::new();
        constants(&mut e);
        let a = vector(&mut e, [4.0, 8.0, 0.0, 1.0]);
        let b = vector(&mut e, [8.0, 0.0, 4.0, 1.0]);
        let c = vector(&mut e, [2.0, 2.0, 2.0, 2.0]);
        let d = vector(&mut e, [6.0, 6.0, 6.0, 6.0]);
        let out = e.mem.alloc(16);
        e.call(0x00de_9b10, &args![out, a, b]);
        assert_eq!(load_vec4(&e, out), [5.0, 6.0, 1.0, 1.0]);
        e.call(0x00de_9b50, &args![out, a, b, c]);
        assert_eq!(load_vec4(&e, out), [7.0, 5.0, 3.0, 2.0]);
        e.call(0x00de_9b90, &args![out, a, b, c, d]);
        assert_eq!(load_vec4(&e, out), [5.0, 4.0, 3.0, 2.5]);
    }

    #[test]
    fn the_destructors_free_only_when_asked() {
        let mut e = Engine::new();
        e.register(FREE, |_, _| Ret::default());
        let object = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00de_e620, &args![object, 0u8]).u32(), object);
        assert_eq!(e.mem.u32(object), FUNCTION_VTABLE);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.call(0x00de_e620, &args![object, 1u8]);
        assert_eq!(
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == FREE)
                .count(),
            1
        );
        // The policy's destructor frees 0x40 bytes through the router's allocator.
        let vtable = 0x4400_2000;
        e.put_vtable(vtable, &[0, 0, 0x4400_2100]);
        e.register(0x4400_2100, |_, _| Ret::default());
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER_GETTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00de_e6c0, &args![object, 0u8]);
        assert_eq!(e.mem.u32(object), NEAREST_FEATURE_POLICY_VTABLE);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        assert_eq!(e.call(0x00de_e6c0, &args![object, 1u8]).u32(), object);
        let freed: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == 0x4400_2100)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(freed, vec![vec![allocator, object, 0x40]]);
    }

    #[test]
    fn add_leaf_keeps_the_nearest_box() {
        let mut e = Engine::new();
        constants(&mut e);
        let policy = e.mem.alloc(0x40);
        store_vec4(&mut e, policy + 0x10, [0.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(policy + 0x20, 100.0);
        let near = e.mem.alloc(0x20);
        store_vec4(&mut e, near, [0.0, 0.0, 0.0, 0.0]);
        store_vec4(&mut e, near + 0x10, [2.0, 2.0, 2.0, 0.0]);
        assert_eq!(e.call(0x00de_e640, &args![policy, near]).f32(), 1.0);
        assert_eq!(e.mem.f32(policy + 0x20), 3.0);
        assert_eq!(e.mem.u32(policy + 0x30), near);
        // A farther box (squared distance 48) changes nothing.
        let far = e.mem.alloc(0x20);
        store_vec4(&mut e, far, [8.0, 8.0, 8.0, 0.0]);
        store_vec4(&mut e, far + 0x10, [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.call(0x00de_e640, &args![policy, far]).f32(), 1.0);
        assert_eq!(e.mem.f32(policy + 0x20), 3.0);
        assert_eq!(e.mem.u32(policy + 0x30), near);
    }

    #[test]
    fn the_constant_function_answers_its_value() {
        let mut e = Engine::new();
        let object = e.mem.alloc(8);
        e.mem.set_f32(object + 4, 2.5);
        assert_eq!(e.call(0x00de_e6b0, &args![object, 0u32]).f32(), 2.5);
    }

    fn f64_argument(a: &[u32]) -> f64 {
        f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32)
    }

    fn x87_function_doubles(e: &mut Engine) {
        for page in [0x0102_1000, 0x0103_0000, 0x0101_2000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f64(TWO_DOUBLE, 2.0);
        e.mem.set_f64(THREE_DOUBLE, 3.0);
        e.mem
            .set_f64(HALF_PI_DOUBLE, f64::from_bits(0x3ff9_21fb_6000_0000));
        e.mem.set_f32(MINUS_ONE_FLOAT, -1.0);
        for (address, function) in [
            (X87_SQRT, f64::sqrt as fn(f64) -> f64),
            (X87_COS, f64::cos),
            (X87_SIN, f64::sin),
        ] {
            e.register_double(address, move |_, a| Ret {
                st0: function(f64_argument(a)),
                ..Ret::default()
            });
        }
    }

    #[test]
    fn the_square_maps_to_unit_vectors_in_every_region() {
        let mut e = Engine::new();
        constants(&mut e);
        x87_function_doubles(&mut e);
        let input = e.mem.alloc(8);
        let out = e.mem.alloc(16);
        // (x, y) = (2u - 1, 2v - 1) in each of the eight regions of the code.
        for (x, y, z_positive) in [
            (0.25f32, 0.5f32, true),
            (0.75, 0.5, false),
            (-0.5, 0.25, true),
            (-0.5, 0.75, false),
            (0.25, -0.25, true),
            (0.75, -0.5, false),
            (-0.25, -0.25, true),
            (-0.75, -0.5, false),
        ] {
            e.mem.set_f32(input, (x + 1.0) / 2.0);
            e.mem.set_f32(input + 4, (y + 1.0) / 2.0);
            e.call(0x00de_9650, &args![input, out]);
            let v = load_vec4(&e, out);
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            assert!((length - 1.0).abs() < 1e-4, "({x}, {y}) gave {v:?}");
            assert_eq!(v[2] > 0.0, z_positive, "({x}, {y}) gave {v:?}");
            assert_eq!(v[3], 0.0);
        }
        // A NaN input runs every compare through the unordered side without breaking the
        // x87 stack discipline (the function asserts an empty stack at its end).
        for (u, v) in [(f32::NAN, 0.75f32), (0.75, f32::NAN), (f32::NAN, f32::NAN)] {
            e.mem.set_f32(input, u);
            e.mem.set_f32(input + 4, v);
            e.call(0x00de_9650, &args![input, out]);
        }
    }

    #[test]
    fn the_inverse_map_covers_every_branch() {
        use std::cell::Cell;
        use std::rc::Rc;
        let mut e = Engine::new();
        constants(&mut e);
        x87_function_doubles(&mut e);
        e.map(0x0101_f000, 0x1000);
        let divisor = f64::from_bits(0x4009_21fb_6000_0000);
        e.mem.set_f64(ANGLE_DIVISOR_DOUBLE, divisor);
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        e.mem.set_f64(HALF_DOUBLE, 0.5);
        let angle = Rc::new(Cell::new(0.0f64));
        let source = Rc::clone(&angle);
        e.register_double(ATAN2_APPROXIMATION, move |_, _| Ret {
            st0: source.get(),
            ..Ret::default()
        });
        let input = e.mem.alloc(16);
        let out = e.mem.alloc(16);
        // The angle word is `2 * angle / divisor`: 0.5, 1.5, -0.5 and -1.5 pick the four
        // branches of each half; z = 0.36 gives radius 0.8 and z = -0.75 gives 0.5.
        let cases = [
            (0.36f32, 0.25f64, (0.7f32, 0.7f32)),
            (0.36, 0.75, (0.3, 0.7)),
            (0.36, -0.25, (0.7, 0.3)),
            (0.36, -0.75, (0.3, 0.3)),
            (-0.75, 0.25, (0.875, 0.875)),
            (-0.75, 0.75, (0.125, 0.875)),
            (-0.75, -0.25, (0.875, 0.125)),
            (-0.75, -0.75, (0.125, 0.125)),
        ];
        for (z, fraction, (first, second)) in cases {
            angle.set(fraction * divisor);
            store_vec4(&mut e, input, [0.3, 0.4, z, 0.0]);
            e.call_log = Some(vec![]);
            e.call(0x00de_98c0, &args![input, out]);
            let log = e.call_log.take().unwrap();
            let asked: Vec<Vec<u32>> = log
                .iter()
                .filter(|call| call.0 == ATAN2_APPROXIMATION)
                .map(|call| call.1.clone())
                .collect();
            assert_eq!(asked, vec![vec![0.4f32.to_bits(), 0.3f32.to_bits()]]);
            let v = load_vec4(&e, out);
            assert!(
                (v[0] - first).abs() < 1e-5 && (v[1] - second).abs() < 1e-5,
                "z {z}, angle {fraction}: {v:?}"
            );
            assert_eq!((v[2], v[3]), (0.0, 0.0));
        }
        // A NaN z takes the z >= 0 side and still leaves the x87 stack empty.
        angle.set(0.25 * divisor);
        store_vec4(&mut e, input, [0.3, 0.4, f32::NAN, 0.0]);
        e.call(0x00de_98c0, &args![input, out]);
    }

    fn float_setup(e: &mut Engine) {
        constants(e);
        x87_function_doubles(e);
    }

    #[test]
    fn the_closest_offset_clamps_and_negates() {
        let mut e = Engine::new();
        float_setup(&mut e);
        let b = vector(&mut e, [0.0; 4]);
        let c = vector(&mut e, [2.0, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(16);
        let fraction = e.mem.alloc(4);
        let run = |e: &mut Engine, point: Vec4, with_fraction: bool| {
            let a = vector(e, point);
            let pointer = if with_fraction { fraction } else { 0 };
            let squared = e.call(0x00de_9e10, &args![a, b, c, out, pointer]).f32();
            (squared, load_vec4(e, out))
        };
        let (squared, offset) = run(&mut e, [1.0, 1.0, 0.0, 0.0], true);
        assert_eq!((squared, offset), (1.0, [0.0, 1.0, 0.0, 0.0]));
        assert_eq!(e.mem.f32(fraction), 0.5);
        let (squared, offset) = run(&mut e, [5.0, 0.0, 0.0, 0.0], true);
        assert_eq!((squared, offset), (9.0, [3.0, 0.0, 0.0, 0.0]));
        assert_eq!(e.mem.f32(fraction), 1.0);
        let (squared, offset) = run(&mut e, [-3.0, 0.0, 0.0, 0.0], true);
        assert_eq!((squared, offset), (9.0, [-3.0, 0.0, 0.0, 0.0]));
        assert_eq!(e.mem.f32(fraction), 0.0);
        // A null fraction pointer stores nothing.
        e.mem.set_f32(fraction, 7.0);
        run(&mut e, [5.0, 0.0, 0.0, 0.0], false);
        assert_eq!(e.mem.f32(fraction), 7.0);
    }

    #[test]
    fn a_point_is_inside_when_all_three_edge_terms_reach_epsilon() {
        let mut e = Engine::new();
        constants(&mut e);
        let a = vector(&mut e, [0.0; 4]);
        let b = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let c = vector(&mut e, [0.0, 1.0, 0.0, 0.0]);
        let inside = |e: &mut Engine, p: Vec4, epsilon: f32| {
            let point = vector(e, p);
            e.call(0x00de_a6b0, &args![point, a, b, c, epsilon]).bool()
        };
        // Edge terms for (0.25, 0.25): 0.25, 0.5 and 0.25.
        assert!(inside(&mut e, [0.25, 0.25, 0.0, 0.0], 0.0));
        assert!(!inside(&mut e, [0.25, 0.25, 0.0, 0.0], 0.3));
        assert!(inside(&mut e, [0.25, 0.25, 0.0, 0.0], f32::NAN));
        // Failing the first, the second and the third edge.
        assert!(!inside(&mut e, [0.5, -1.0, 0.0, 0.0], 0.0));
        assert!(!inside(&mut e, [2.0, 2.0, 0.0, 0.0], 0.0));
        assert!(!inside(&mut e, [-1.0, 0.5, 0.0, 0.0], 0.0));
    }

    fn support_setup(e: &mut Engine) {
        constants(e);
        for page in [0x0104_e000, 0x010d_9000, 0x010d_0000] {
            e.map(page, 0x1000);
        }
        e.mem.set_u32(LOWEST_FLOAT, 0xff7f_ffee);
        store_vec4(e, LANE_NUMBERS, [0.5, 1.5, 2.5, 3.5]);
        store_vec4(e, LANE_STEP, [4.0; 4]);
        e.register(FTOL, |_, a| Ret {
            eax: f64_argument(a) as i32 as u32,
            ..Ret::default()
        });
    }

    fn block(e: &mut Engine, at: u32, x: Vec4, y: Vec4, z: Vec4) {
        store_vec4(e, at, x);
        store_vec4(e, at + 0x10, y);
        store_vec4(e, at + 0x20, z);
    }

    #[test]
    fn the_support_search_finds_the_vertex_with_the_largest_dot_product() {
        let mut e = Engine::new();
        support_setup(&mut e);
        let vertices = e.mem.alloc(0x60);
        block(
            &mut e,
            vertices,
            [1.0, 2.0, 3.0, 4.0],
            [0.5; 4],
            [0.25, 0.5, 0.75, 1.0],
        );
        block(
            &mut e,
            vertices + 0x30,
            [9.0, 8.0, 7.0, 6.0],
            [1.5, 2.5, 3.5, 4.5],
            [10.0, 11.0, 12.0, 13.0],
        );
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, vertices);
        e.mem.set_u32(record + 4, 2);
        let direction = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(16);
        let tagged = e.call(0x00de_a810, &args![record, direction, out]).u32();
        assert_eq!(tagged, 4 | 0x3f00_0000);
        assert_eq!(e.mem.u32(out + 0xc), tagged);
        assert_eq!(
            [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)],
            [9.0, 1.5, 10.0]
        );
        // Ties: lane 1 beats lane 0 and lane 3 beats lane 2, then the larger wins.
        e.mem.set_u32(record + 4, 1);
        block(
            &mut e,
            vertices,
            [5.0, 5.0, 1.0, 1.0],
            [0.5; 4],
            [0.25, 0.5, 0.75, 1.0],
        );
        let tagged = e.call(0x00de_a810, &args![record, direction, out]).u32();
        assert_eq!(tagged, 0x3f00_0001);
        assert_eq!(
            [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)],
            [5.0, 0.5, 0.5]
        );
        // No blocks: the numbers stay 0 and the first vertex is answered.
        e.mem.set_u32(record + 4, 0);
        let tagged = e.call(0x00de_a810, &args![record, direction, out]).u32();
        assert_eq!(tagged, 0x3f00_0000);
    }

    #[test]
    fn the_triangle_measure_divides_by_the_squared_edges() {
        let mut e = Engine::new();
        float_setup(&mut e);
        e.map(0x0104_d000, 0x1000);
        e.mem.set_f32(EDGE_SUM_THRESHOLD, 0.0);
        let a = vector(&mut e, [0.0; 4]);
        let b = vector(&mut e, [3.0, 0.0, 0.0, 0.0]);
        let c = vector(&mut e, [0.0, 4.0, 0.0, 0.0]);
        // |n| = 12, squared edges 9 + 25 + 16 = 50.
        let measure = e.call(0x00de_a980, &args![a, b, c]).f32();
        let expected = (3.0f64.sqrt() * 24.0 / 50.0) as f32;
        assert!((measure - expected).abs() < 1e-5, "{measure} {expected}");
        // A point triangle: the sum 0 is not above the threshold 0.
        assert_eq!(e.call(0x00de_a980, &args![a, a, a]).f32(), 0.0);
        // A threshold above the sum also answers 0.
        e.mem.set_f32(EDGE_SUM_THRESHOLD, 100.0);
        assert_eq!(e.call(0x00de_a980, &args![a, b, c]).f32(), 0.0);
    }

    fn router_for_arrays(e: &mut Engine) {
        let allocator = e.mem.alloc(8);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER_GETTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.map(0x0126_7000, 0x1000);
    }

    #[test]
    fn sampling_keeps_points_the_shape_accepts() {
        use std::cell::RefCell;
        use std::collections::VecDeque;
        use std::rc::Rc;
        let mut e = Engine::new();
        router_for_arrays(&mut e);
        e.register(SHAPE_BOUNDS, |e, a| {
            store_vec4(e, a[1], [0.0; 4]);
            store_vec4(e, a[1] + 0x10, [2.0, 2.0, 2.0, 0.0]);
            Ret::default()
        });
        let script = Rc::new(RefCell::new(VecDeque::from([
            [0.8f32, 0.0, 0.0, 0.0],
            [0.25, 0.5, 0.5, 9.0],
            [0.1, 0.0, 0.0, 0.0],
        ])));
        let source = Rc::clone(&script);
        let draws = Rc::new(RefCell::new(0u32));
        let counter = Rc::clone(&draws);
        e.register_double(RANDOM_VECTOR, move |e, a| {
            *counter.borrow_mut() += 1;
            let next = source
                .borrow_mut()
                .pop_front()
                .unwrap_or([0.9, 0.0, 0.0, 0.0]);
            store_vec4(e, a[1], next);
            Ret::default()
        });
        let queries = Rc::new(RefCell::new(0u32));
        let counter = Rc::clone(&queries);
        e.register_double(SHAPE_POINT_QUERY, move |e, a| {
            *counter.borrow_mut() += 1;
            // Accepts the points whose x is below 1.
            e.mem.set_u8(a[3], u8::from(e.mem.f32(a[1]) < 1.0));
            Ret {
                st0: 0.5,
                ..Ret::default()
            }
        });
        let points = e.mem.alloc(0xc);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(points, data);
        e.mem.set_u32(points + 8, 4);
        assert!(e
            .call(0x00de_dbb0, &args![0x7000u32, 2u32, points, 0x8000u32])
            .bool());
        assert_eq!(e.mem.u32(points + 4), 2);
        assert_eq!(load_vec4(&e, data), [0.5, 1.0, 1.0, 1.0]);
        assert_eq!(load_vec4(&e, data + 0x10), [0.2, 0.0, 0.0, 1.0]);
        // One rejected, two accepted.
        assert_eq!((*draws.borrow(), *queries.borrow()), (3, 3));
        // Nothing but rejections: after 9 tries in a row (more than 1 * 8) it gives up.
        let other = e.mem.alloc(0xc);
        e.mem.set_u32(other, data);
        e.mem.set_u32(other + 8, 4);
        *draws.borrow_mut() = 0;
        assert!(!e
            .call(0x00de_dbb0, &args![0x7000u32, 1u32, other, 0x8000u32])
            .bool());
        assert_eq!(*draws.borrow(), 9);
        assert_eq!(e.mem.u32(other + 4), 0);
        // No points wanted: true, with no tries.
        *draws.borrow_mut() = 0;
        assert!(e
            .call(0x00de_dbb0, &args![0x7000u32, 0u32, other, 0x8000u32])
            .bool());
        assert_eq!(*draws.borrow(), 0);
    }

    #[test]
    fn vectors_are_transposed_four_to_a_block() {
        let mut e = Engine::new();
        router_for_arrays(&mut e);
        let asked = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = std::rc::Rc::clone(&asked);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            sink.borrow_mut().push(a[2..].to_vec());
            let data = e.mem.alloc(a[2] * a[3]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let vectors = e.mem.alloc(0xc);
        let data = e.mem.alloc(5 * 16);
        for index in 0..5u32 {
            let scale = (index + 1) as f32;
            store_vec4(
                &mut e,
                data + 16 * index,
                [scale, 10.0 * scale, 100.0 * scale, 7.0],
            );
        }
        e.mem.set_u32(vectors, data);
        e.mem.set_u32(vectors + 4, 5);
        let blocks = e.mem.alloc(0xc);
        e.mem.set_u32(blocks + 8, 0x8000_0000);
        e.call(0x00de_dcf0, &args![vectors, blocks]);
        assert_eq!(*asked.borrow(), vec![vec![2, 0x30]]);
        assert_eq!(e.mem.u32(blocks + 4), 2);
        let out = e.mem.u32(blocks);
        assert_eq!(load_vec4(&e, out), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec4(&e, out + 0x10), [10.0, 20.0, 30.0, 40.0]);
        assert_eq!(load_vec4(&e, out + 0x20), [100.0, 200.0, 300.0, 400.0]);
        // The last, partial block repeats its first vector.
        assert_eq!(load_vec4(&e, out + 0x30), [5.0; 4]);
        assert_eq!(load_vec4(&e, out + 0x40), [50.0; 4]);
        assert_eq!(load_vec4(&e, out + 0x50), [500.0; 4]);
        // Four vectors make one block appended after the existing one; no reserve is needed.
        e.mem.set_u32(vectors + 4, 4);
        e.mem.set_u32(blocks + 4, 1);
        e.mem.set_u32(blocks + 8, 4);
        e.call(0x00de_dcf0, &args![vectors, blocks]);
        assert_eq!(e.mem.u32(blocks + 4), 2);
        assert_eq!(load_vec4(&e, out), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(asked.borrow().len(), 1);
        // No vectors: no blocks.
        e.mem.set_u32(vectors + 4, 0);
        e.call(0x00de_dcf0, &args![vectors, blocks]);
        assert_eq!(e.mem.u32(blocks + 4), 2);
    }

    #[test]
    fn the_segment_distance_clamps_both_parameters() {
        let mut e = Engine::new();
        constants(&mut e);
        e.map(0x0104_d000, 0x1000);
        e.mem.set_f32(PARALLEL_EPSILON, 1e-6);
        e.mem.set_f32(LARGEST_DISTANCE, 1e30);
        let a = vector(&mut e, [0.0; 4]);
        let b = vector(&mut e, [2.0, 0.0, 0.0, 0.0]);
        let along_ab = e.mem.alloc(4);
        let along_cd = e.mem.alloc(4);
        let run = |e: &mut Engine, c: Vec4, d: Vec4| {
            let c = vector(e, c);
            let d = vector(e, d);
            let distance = e
                .call(0x00de_a450, &args![a, b, c, d, along_ab, along_cd])
                .f32();
            (distance, e.mem.f32(along_ab), e.mem.f32(along_cd))
        };
        // Crossing in the middle: both parameters 0.5, distance 1.
        let (distance, t, s) = run(&mut e, [1.0, 1.0, -1.0, 0.0], [1.0, 1.0, 1.0, 0.0]);
        assert_eq!((distance, t, s), (1.0, 0.5, 0.5));
        // The other segment's parameter below 0 and above 1.
        let (distance, t, s) = run(&mut e, [1.0, 1.0, -3.0, 0.0], [1.0, 1.0, -1.0, 0.0]);
        assert!((distance - 2.0f32.sqrt()).abs() < 1e-5);
        assert_eq!((t, s), (0.5, 1.0));
        let (distance, t, s) = run(&mut e, [1.0, 1.0, 1.0, 0.0], [1.0, 1.0, 3.0, 0.0]);
        assert!((distance - 2.0f32.sqrt()).abs() < 1e-5);
        assert_eq!((t, s), (0.5, 0.0));
        // The first segment's parameter above 1 and below 0.
        let (distance, t, s) = run(&mut e, [5.0, 1.0, -1.0, 0.0], [5.0, 1.0, 1.0, 0.0]);
        assert!((distance - 10.0f32.sqrt()).abs() < 1e-5);
        assert_eq!((t, s), (1.0, 0.5));
        let (distance, t, s) = run(&mut e, [-3.0, 1.0, -1.0, 0.0], [-3.0, 1.0, 1.0, 0.0]);
        assert!((distance - 10.0f32.sqrt()).abs() < 1e-5);
        assert_eq!((t, s), (0.0, 0.5));
        // Parallel segments: both parameters 0 and the largest float.
        let (distance, t, s) = run(&mut e, [0.0, 1.0, 0.0, 0.0], [2.0, 1.0, 0.0, 0.0]);
        assert_eq!((distance, t, s), (1e30, 0.0, 0.0));
    }

    fn triangle_setup(e: &mut Engine) {
        constants(e);
        x87_function_doubles(e);
        e.map(0x0104_d000, 0x1000);
        e.mem.set_f32(REFINE_HALF, 0.5);
        e.mem.set_f32(REFINE_THREE, 3.0);
        e.mem.set_f32(ONE_FLOAT, 1.0);
        e.mem.set_f32(PARALLEL_EPSILON, 1e-6);
    }

    #[test]
    fn a_point_above_the_triangle_projects_inside() {
        let mut e = Engine::new();
        triangle_setup(&mut e);
        let a = vector(&mut e, [0.0; 4]);
        let b = vector(&mut e, [4.0, 0.0, 0.0, 0.0]);
        let c = vector(&mut e, [0.0, 4.0, 0.0, 0.0]);
        let out = e.mem.alloc(16);
        let bary = e.mem.alloc(16);
        let above = vector(&mut e, [1.0, 1.0, 3.0, 0.0]);
        let squared = e.call(0x00de_9f10, &args![above, a, b, c, out, bary]).f32();
        assert!((squared - 9.0).abs() < 1e-3, "{squared}");
        let plane = load_vec4(&e, out);
        assert!(plane[0].abs() < 1e-6 && plane[1].abs() < 1e-6);
        assert!((plane[2] - 1.0).abs() < 1e-4 && plane[3].abs() < 1e-5);
        let weights = load_vec4(&e, bary);
        for (got, want) in weights.iter().zip([0.5, 0.25, 0.25, 0.0]) {
            assert!((got - want).abs() < 1e-5, "{weights:?}");
        }
        // Below the plane the stored plane is negated; a null barycentric pointer is allowed.
        let below = vector(&mut e, [1.0, 1.0, -3.0, 0.0]);
        let squared = e.call(0x00de_9f10, &args![below, a, b, c, out, 0u32]).f32();
        assert!((squared - 9.0).abs() < 1e-3);
        assert!((load_vec4(&e, out)[2] + 1.0).abs() < 1e-4);
        // A sum of areas not above the epsilon gives the thirds (here 0.5 from the exe's scalar).
        e.mem.set_f32(PARALLEL_EPSILON, 1e9);
        e.call(0x00de_9f10, &args![above, a, b, c, out, bary]);
        assert_eq!(load_vec4(&e, bary), [0.5, 0.5, 0.5, 0.0]);
    }

    #[test]
    fn a_point_beside_the_triangle_uses_the_closest_edge() {
        let mut e = Engine::new();
        triangle_setup(&mut e);
        let a = vector(&mut e, [0.0; 4]);
        let b = vector(&mut e, [4.0, 0.0, 0.0, 0.0]);
        let c = vector(&mut e, [0.0, 4.0, 0.0, 0.0]);
        let out = e.mem.alloc(16);
        let bary = e.mem.alloc(16);
        // Closest to vertex A: the edges AB (6) and CA (6) tie, the later one (CA) wins.
        let beside = vector(&mut e, [-1.0, -1.0, 2.0, 0.0]);
        let squared = e
            .call(0x00de_9f10, &args![beside, a, b, c, out, bary])
            .f32();
        assert!((squared - 6.0).abs() < 1e-4, "{squared}");
        let offset = load_vec4(&e, out);
        assert_eq!(offset[..3], [-1.0, -1.0, 2.0]);
        assert_eq!(load_vec4(&e, bary), [1.0, 0.0, 0.0, 0.0]);
        // Beyond edge BC: closest point (2, 2, 0), parameter 0.5 on BC.
        let across = vector(&mut e, [3.0, 3.0, 2.0, 0.0]);
        let squared = e
            .call(0x00de_9f10, &args![across, a, b, c, out, bary])
            .f32();
        assert!((squared - 6.0).abs() < 1e-4, "{squared}");
        let weights = load_vec4(&e, bary);
        assert_eq!(weights[0], 0.0);
        assert!((weights[1] - 0.5).abs() < 1e-6 && (weights[2] - 0.5).abs() < 1e-6);
    }

    type CallLog = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;

    /// The allocator's `free` (slot `0x10`) as the relaxation tests log it.
    const ALLOCATOR_FREE: u32 = 0x4400_8102;

    fn grow_array(e: &mut Engine, array: u32, capacity: u32, element: u32) {
        let data = e.mem.alloc(capacity * element);
        let old = e.mem.u32(array);
        for i in 0..e.mem.u32(array + 4) * element {
            let byte = e.mem.u8(old + i);
            e.mem.set_u8(data + i, byte);
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
    }

    /// Doubles for what `00dedf10` calls outside the unit: the shape and tree objects (each
    /// call is logged with its arguments), the memory router with an allocator that logs its
    /// `free(data, bytes)` and the growth of the `hkArray`s.
    fn relaxation_doubles(e: &mut Engine) -> CallLog {
        use std::rc::Rc;
        let log = CallLog::default();
        for page in [0x0101_1000, 0x0126_7000, 0x0126_a000, 0x0104_d000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f32(LARGEST_DISTANCE, f32::MAX);
        for address in [
            REGISTER_AT_EXIT,
            SHAPE_CONSTRUCTOR,
            SHAPE_SET_UP,
            SHAPE_PREPARE,
            SHAPE_FINISH,
            SHAPE_DESTRUCTOR,
            BOX_TREE_CONSTRUCTOR,
            BOX_TREE_UPDATE,
            BOX_TREE_DESTRUCTOR,
            BOX_TREE_MEMBER_DESTRUCTOR,
            ALLOCATOR_FREE,
        ] {
            let sink = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        e.put_vtable(0x4400_8100, &[0, 0, 0, 0, ALLOCATOR_FREE]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, 0x4400_8100);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER_GETTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, a| {
            grow_array(e, a[1], a[2], a[3]);
            Ret::default()
        });
        e.register(ARRAY_RESERVE_MORE, |e, a| {
            let capacity = (e.mem.u32(a[1] + 8) & 0x3fff_ffff) * 2;
            grow_array(e, a[1], capacity.max(4), a[2]);
            Ret::default()
        });
        log
    }

    /// The shape of `00dedbb0`: the box from 0 to 2 in every axis, the generator's scripted
    /// vectors (then `(0.5, 0.5, 0.5)`) and a point query that accepts all points or none.
    /// Answers the states the generator was called with.
    fn sampling_doubles(
        e: &mut Engine,
        script: &[Vec4],
        accept: bool,
    ) -> std::rc::Rc<std::cell::RefCell<Vec<(u32, u32)>>> {
        let mut script: std::collections::VecDeque<Vec4> = script.iter().copied().collect();
        let seeds = std::rc::Rc::<std::cell::RefCell<Vec<(u32, u32)>>>::default();
        let sink = std::rc::Rc::clone(&seeds);
        e.register(SHAPE_BOUNDS, |e, a| {
            store_vec4(e, a[1], [0.0; 4]);
            store_vec4(e, a[1] + 0x10, [2.0, 2.0, 2.0, 0.0]);
            Ret::default()
        });
        e.register_double(RANDOM_VECTOR, move |e, a| {
            sink.borrow_mut()
                .push((e.mem.u32(a[0]), e.mem.u32(a[0] + 4)));
            let next = script.pop_front().unwrap_or([0.5, 0.5, 0.5, 0.0]);
            store_vec4(e, a[1], next);
            Ret::default()
        });
        e.register_double(SHAPE_POINT_QUERY, move |e, a| {
            e.mem.set_u8(a[3], u8::from(accept));
            Ret::default()
        });
        seeds
    }

    fn calls_to(log: &CallLog, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    fn point_array(e: &mut Engine, capacity: u32) -> u32 {
        let array = e.mem.alloc(0xc);
        let data = e.mem.alloc(16 * capacity);
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 8, capacity);
        array
    }

    #[test]
    fn the_relaxation_without_iterations_only_samples_the_shape() {
        let mut e = Engine::new();
        let log = relaxation_doubles(&mut e);
        let seeds = sampling_doubles(
            &mut e,
            &[[0.25, 0.5, 0.5, 0.0], [0.75, 0.25, 0.25, 0.0]],
            true,
        );
        let points = point_array(&mut e, 8);
        assert!(e
            .call(
                0x00de_df10,
                &args![0x7000u32, 2u32, points, 0u32, 0x1234u32, 0u32]
            )
            .bool());
        // The first call builds the function-local static and registers its destructor.
        assert_eq!(e.mem.u8(CONST_FUNCTION_GUARD) & 1, 1);
        assert_eq!(e.mem.u32(CONST_FUNCTION_STATIC), CONST_FUNCTION_VTABLE);
        assert_eq!(e.mem.f32(CONST_FUNCTION_STATIC + 4), 1.0);
        assert_eq!(
            calls_to(&log, REGISTER_AT_EXIT),
            vec![vec![STATIC_DESTRUCTOR]]
        );
        let set_up = calls_to(&log, SHAPE_SET_UP);
        assert_eq!(set_up.len(), 1);
        assert_eq!(set_up[0][1..], [0x7000, 0x0126_8370, 0xffff_ffff]);
        // Two sampled points inside the box 0..2, with 1.0 as their weight.
        let data = e.mem.u32(points);
        assert_eq!(e.mem.u32(points + 4), 2);
        assert_eq!(load_vec4(&e, data), [0.5, 1.0, 1.0, 1.0]);
        assert_eq!(load_vec4(&e, data + 0x10), [1.5, 0.5, 0.5, 1.0]);
        assert!(seeds.borrow().iter().all(|s| *s == (0x1234, 0x1234)));
        assert_eq!(calls_to(&log, SHAPE_DESTRUCTOR).len(), 1);
        assert!(calls_to(&log, BOX_TREE_CONSTRUCTOR).is_empty());
        // The second call finds the static built.
        let other = point_array(&mut e, 8);
        assert!(e
            .call(
                0x00de_df10,
                &args![0x7000u32, 1u32, other, 0u32, 0x1234u32, 0u32]
            )
            .bool());
        assert_eq!(calls_to(&log, REGISTER_AT_EXIT).len(), 1);
        assert_eq!(calls_to(&log, SHAPE_DESTRUCTOR).len(), 2);
    }

    #[test]
    fn the_relaxation_fails_when_the_shape_rejects_every_sample() {
        let mut e = Engine::new();
        let log = relaxation_doubles(&mut e);
        sampling_doubles(&mut e, &[], false);
        let points = point_array(&mut e, 8);
        assert!(!e
            .call(
                0x00de_df10,
                &args![0x7000u32, 2u32, points, 3u32, 0x1234u32, 0u32]
            )
            .bool());
        assert_eq!(e.mem.u32(points + 4), 0);
        assert_eq!(calls_to(&log, SHAPE_DESTRUCTOR).len(), 1);
        assert!(calls_to(&log, BOX_TREE_CONSTRUCTOR).is_empty());
    }

    #[test]
    fn each_iteration_moves_every_point_to_the_weighted_mean_of_its_samples() {
        use std::rc::Rc;
        let mut e = Engine::new();
        let log = relaxation_doubles(&mut e);
        let seeds = sampling_doubles(
            &mut e,
            &[[0.25, 0.5, 0.5, 0.0], [0.75, 0.25, 0.25, 0.0]],
            true,
        );
        // The weight function answers 2.0 everywhere.
        let function = e.mem.alloc(8);
        e.put_vtable(0x4400_8000, &[0, 0x4400_8001]);
        e.mem.set_u32(function, 0x4400_8000);
        e.register(0x4400_8001, |_, _| Ret {
            st0: 2.0,
            ..Ret::default()
        });
        // The tree: `build` hands out the handles 0x5000 + i and the node a query finds
        // belongs to the first point.
        let node = e.mem.alloc(0x24);
        let sink = Rc::clone(&log);
        e.register_double(BOX_TREE_BUILD, move |e, a| {
            let count = a[3];
            let mut words = a.to_vec();
            for i in 0..8 * count {
                words.push(e.mem.u32(a[1] + 4 * i));
            }
            for i in 0..count {
                words.push(e.mem.u32(a[2] + 4 * i));
            }
            sink.borrow_mut().push((BOX_TREE_BUILD, words));
            let handles = e.mem.u32(a[4]);
            for i in 0..count {
                e.mem.set_u32(handles + 4 * i, 0x5000 + i);
            }
            let first = e.mem.u32(a[2]);
            e.mem.set_u32(node + 0x20, first);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(BOX_TREE_QUERY, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..9).map(|i| e.mem.u32(a[2] + 4 * i)));
            sink.borrow_mut().push((BOX_TREE_QUERY, words));
            e.mem.set_u32(a[2] + 0x30, node);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(BOX_TREE_UPDATE, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..8).map(|i| e.mem.u32(a[2] + 4 * i)));
            sink.borrow_mut().push((BOX_TREE_UPDATE, words));
            Ret::default()
        });
        let points = point_array(&mut e, 8);
        assert!(e
            .call(
                0x00de_df10,
                &args![0x7000u32, 2u32, points, 1i32, 0x1234u32, function]
            )
            .bool());
        let first = e.mem.u32(points);
        // The first point gathers 256 samples of weight 2 at (1, 1, 1): its own weighted
        // (1, 2, 2) with weight 2 makes (513, 514, 514) over a weight of 514, and it is
        // weighted again; the second only returns to its place and is weighted again.
        let p0 = load_vec4(&e, first);
        let inverse = 1.0f32 / 514.0;
        assert!((p0[0] - 2.0 * 513.0 * inverse).abs() < 1e-4);
        assert!((p0[1] - 2.0).abs() < 1e-4);
        assert!((p0[2] - 2.0).abs() < 1e-4);
        assert_eq!(p0[3], 2.0);
        assert_eq!(load_vec4(&e, first + 0x10), [3.0, 1.0, 1.0, 2.0]);
        assert!(seeds.borrow().iter().all(|s| *s == (0x1234, 0x1234)));
        // The boxes (a point with the zero of its weight lane on both corners) and the
        // pointers to the points went to the tree.
        let built = calls_to(&log, BOX_TREE_BUILD);
        assert_eq!(built.len(), 1);
        let words = &built[0];
        assert_eq!(words[3], 2);
        let bits = |values: [f32; 4]| values.map(f32::to_bits);
        assert_eq!(words[5..9], bits([0.5, 1.0, 1.0, 0.0]));
        assert_eq!(words[9..13], bits([0.5, 1.0, 1.0, 0.0]));
        assert_eq!(words[13..17], bits([1.5, 0.5, 0.5, 0.0]));
        assert_eq!(words[17..21], bits([1.5, 0.5, 0.5, 0.0]));
        assert_eq!(words[21..], [first, first + 0x10]);
        // 2 * 2 is below 0x100, so a round draws 0x100 samples; each query starts at the
        // largest distance with the sample's place and weight.
        let queries = calls_to(&log, BOX_TREE_QUERY);
        assert_eq!(queries.len(), 0x100);
        assert_eq!(queries[0][3], QUERY_VTABLE);
        assert_eq!(queries[0][3 + 4..3 + 7], bits([1.0, 1.0, 1.0, 0.0])[..3]);
        assert_eq!(queries[0][3 + 7], 1.0f32.to_bits());
        assert_eq!(queries[0][3 + 8], f32::MAX.to_bits());
        // Each point is put back in the tree at its new place (a box of one point).
        let updates = calls_to(&log, BOX_TREE_UPDATE);
        assert_eq!(updates.len(), 2);
        assert_eq!(updates[0][1], 0x5000);
        assert_eq!(updates[1][1], 0x5001);
        assert_eq!(updates[1][3..7], bits([1.5, 0.5, 0.5, 0.0]));
        assert_eq!(updates[1][7..11], bits([1.5, 0.5, 0.5, 0.0]));
        // The arrays went back (samples, pointers, boxes, handles) and the objects were
        // destroyed with the tree's member at +0x1c.
        let freed: Vec<u32> = calls_to(&log, ALLOCATOR_FREE)
            .iter()
            .map(|call| call[2])
            .collect();
        assert_eq!(freed, vec![0x1000, 16, 0x80, 8]);
        let tree = calls_to(&log, BOX_TREE_CONSTRUCTOR)[0][0];
        assert_eq!(calls_to(&log, BOX_TREE_DESTRUCTOR), vec![vec![tree]]);
        assert_eq!(
            calls_to(&log, BOX_TREE_MEMBER_DESTRUCTOR),
            vec![vec![tree + 0x1c]]
        );
        assert_eq!(calls_to(&log, SHAPE_DESTRUCTOR).len(), 1);
    }

    fn triangle(e: &mut Engine, corners: [[f32; 3]; 3]) -> u32 {
        let at = e.mem.alloc(0x30);
        for (i, p) in corners.iter().enumerate() {
            store_vec4(e, at + 0x10 * i as u32, [p[0], p[1], p[2], 0.0]);
        }
        at
    }

    #[test]
    fn triangles_intersect_unless_an_axis_separates_them() {
        let mut e = Engine::new();
        let ground = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
        let shifted = |x: f32, y: f32, z: f32| ground.map(|p| [p[0] + x, p[1] + y, p[2] + z]);
        let mut intersect = |first: [[f32; 3]; 3], second: [[f32; 3]; 3]| {
            let a = triangle(&mut e, first);
            let b = triangle(&mut e, second);
            let forward = e.call(0x00de_ab10, &args![a, b]).bool();
            // The test is symmetric.
            assert_eq!(forward, e.call(0x00de_ab10, &args![b, a]).bool());
            forward
        };
        // Overlapping in the same plane.
        assert!(intersect(ground, shifted(0.5, 0.5, 0.0)));
        // The same shape apart in the plane, and one plane above the other (the normals).
        assert!(!intersect(ground, shifted(5.0, 5.0, 0.0)));
        assert!(!intersect(ground, shifted(0.0, 0.0, 1.0)));
        // A triangle standing through the first.
        assert!(intersect(
            ground,
            [[0.5, 0.5, -1.0], [0.5, 0.5, 1.0], [1.5, 0.5, 0.0]]
        ));
        // The same one passing beside it.
        assert!(!intersect(
            ground,
            [[3.0, 3.0, -1.0], [3.0, 3.0, 1.0], [4.0, 3.0, 0.0]]
        ));
        // Touching at a corner is not separated (the ranges meet).
        assert!(intersect(
            ground,
            [[0.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, -1.0, 0.0]]
        ));
        // A NaN never separates: every axis' compare is unordered.
        assert!(intersect(
            ground,
            [[f32::NAN; 3], [f32::NAN; 3], [f32::NAN; 3]]
        ));
    }
}

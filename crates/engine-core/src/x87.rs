//! x87 behaviour shared by the translations: loads and stores, the NaN an
//! arithmetic instruction gives, and the precision control of the running
//! thread's control word.
//!
//! An `FLD`/`FSTP` round trip of a single-precision value through the x87
//! stack quiets a signalling NaN: the quiet bit (bit 22) is set and the rest
//! of the payload is kept. Every other value, quiet NaNs and infinities
//! included, comes back unchanged. This is done on the bit pattern, because
//! Rust's `as` casts do not guarantee NaN payloads.
//!
//! # Precision control
//!
//! The engine holds x87 values as `f64`. Under the control word's 53-bit
//! precision (`0x027F`, how every thread starts) `FADD`, `FSUB`, `FMUL`,
//! `FDIV` and `FSQRT` round their result to 53 bits, as `f64` arithmetic
//! does. Under 24-bit precision (`0x007F`: the main thread once Direct3D has
//! created the renderer's device, `units::platform::direct3d_device_created`)
//! they round the significand to 24 bits with the control word's rounding
//! mode, while the exponent keeps the extended range (Intel SDM vol. 1,
//! 8.1.5.2): unlike `f32` arithmetic, a result beyond the float range neither
//! overflows nor becomes a float denormal; only a store to a `float`
//! ([`x87_store32`]) does that. Loads, stores, compares and the other
//! instructions are not affected by precision control.
//!
//! [`x87_add`], [`x87_sub`], [`x87_mul`], [`x87_div`] and [`x87_sqrt`] read the
//! precision from [`control_word`] and round each result once, from the exact
//! value, as the FPU does: the `f64` result together with the sign of its
//! rounding error (computed exactly) decides the 24-bit rounding, so an `f64`
//! result that lands exactly between two 24-bit values is not rounded a second
//! time the wrong way. [`x87_round`] rounds an `f64` that a translation
//! computed in plain `f64`.
//!
//! Range: the model holds values in `f64`, whose exponent range is far smaller
//! than the extended format's (2^-16382 to 2^16383). Every nonzero `f64`,
//! subnormals included, is therefore a normal extended value, so the FPU never
//! produces an extended denormal (with its reduced precision and underflow
//! handling) for a value the model can hold: the 24-bit rounding never meets
//! one. Results beyond the `f64` range overflow to infinity, and results below
//! the smallest normal `f64` (2^-1022) lose bits in the `f64` itself, as they
//! already do at 53-bit precision; both are outside the model. 64-bit
//! precision (`0x037F`) is modelled as 53-bit, and so is the reserved
//! precision value 01. At 53-bit precision the rounding mode is not modelled
//! (round to nearest, as before); at 24-bit precision all four modes are.

/// The bit pattern an `FLD dword` / `FSTP dword` pair leaves for `bits`.
#[inline]
pub fn x87_quiet_bits(bits: u32) -> u32 {
    if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
        bits | 0x0040_0000
    } else {
        bits
    }
}

/// [`x87_quiet_bits`] on an `f32`.
#[inline]
pub fn x87_quiet(value: f32) -> f32 {
    f32::from_bits(x87_quiet_bits(value.to_bits()))
}

const F64_QUIET: u64 = 1 << 51;
const F64_EXPONENT: u64 = 0x7ff0_0000_0000_0000;
const F64_SIGNIFICAND: u64 = 0x000f_ffff_ffff_ffff;

fn is_nan_bits(bits: u64) -> bool {
    bits & F64_EXPONENT == F64_EXPONENT && bits & F64_SIGNIFICAND != 0
}

/// The NaN an x87 `FADD`/`FSUB`/`FMUL`/`FDIV` (and their `P`/`R` forms)
/// gives when an operand is a NaN, or `None` when neither is (Intel SDM
/// vol. 1, section 4.8.3.5, table 4-7, x87 column). One NaN: that NaN,
/// quieted. Two: a quiet NaN wins over a signalling one; otherwise the one
/// with the larger significand (compared quieted), and on a tie the
/// positive one. The operand order does not matter. f64 arithmetic does not
/// follow this: SSE keeps the first operand's NaN, and LLVM may swap the
/// operands of a commutative operation (#199).
#[inline]
fn nan_result(a: f64, b: f64) -> Option<f64> {
    if !(a.is_nan() | b.is_nan()) {
        return None;
    }
    nan_result_slow(a, b)
}

/// [`nan_result`] once an operand is a NaN.
#[cold]
#[inline(never)]
fn nan_result_slow(a: f64, b: f64) -> Option<f64> {
    let (a, b) = (a.to_bits(), b.to_bits());
    let chosen = match (is_nan_bits(a), is_nan_bits(b)) {
        (false, false) => return None,
        (true, false) => a,
        (false, true) => b,
        (true, true) => match (a & F64_QUIET != 0, b & F64_QUIET != 0) {
            (true, false) => a,
            (false, true) => b,
            _ => {
                let (sa, sb) = (
                    (a | F64_QUIET) & F64_SIGNIFICAND,
                    (b | F64_QUIET) & F64_SIGNIFICAND,
                );
                match sa.cmp(&sb) {
                    std::cmp::Ordering::Greater => a,
                    std::cmp::Ordering::Less => b,
                    // Equal significands: the sign bit decides, positive first.
                    std::cmp::Ordering::Equal => a.min(b),
                }
            }
        },
    };
    Some(f64::from_bits(chosen | F64_QUIET))
}

// ---- The control word ------------------------------------------------------

/// The control word every game thread starts with: all exceptions masked,
/// 53-bit precision, round to nearest. Measured (nv-probe `fpu=1`) on the main
/// thread before the renderer creates its device and on the AI linear task
/// threads for the whole run.
pub const DEFAULT_CONTROL_WORD: u16 = 0x027f;

/// Precision control (bits 8-9): `00` is 24-bit, `10` 53-bit, `11` 64-bit.
const PRECISION_CONTROL: u16 = 0x0300;
/// Rounding control (bits 10-11).
const ROUNDING_CONTROL: u16 = 0x0c00;
const ROUND_NEAREST: u16 = 0x0000;
const ROUND_DOWN: u16 = 0x0400;
const ROUND_UP: u16 = 0x0800;

thread_local! {
    /// The FPU control register of the game thread now running on this host
    /// thread. The engine runs game code on one host thread; the platform
    /// layer loads this when it switches game threads.
    static CONTROL_WORD: std::cell::Cell<u16> = const { std::cell::Cell::new(DEFAULT_CONTROL_WORD) };
}

/// The x87 control word the arithmetic helpers round with: the running game
/// thread's (`FSTCW`).
#[inline]
pub fn control_word() -> u16 {
    CONTROL_WORD.with(|c| c.get())
}

/// Loads the x87 control register (`FLDCW`) the helpers read. Only the CRT and
/// platform units call it: `units::crt::set_x87_control_word`, which also keeps
/// the thread's own copy, `Engine::new` (a new main thread) and the thread
/// switch in `units::platform`.
pub fn set_control_word(word: u16) {
    CONTROL_WORD.with(|c| c.set(word));
}

/// The rounding mode when the control word selects 24-bit precision, `None`
/// for 53-bit (and for 64-bit and the reserved value, which the model treats
/// as 53-bit).
#[inline]
fn single_precision() -> Option<u16> {
    let word = control_word();
    if word & PRECISION_CONTROL == 0 {
        Some(word & ROUNDING_CONTROL)
    } else {
        None
    }
}

fn sign_of(error: f64) -> i32 {
    if error > 0.0 {
        1
    } else if error < 0.0 {
        -1
    } else {
        0
    }
}

/// `q * 2^exponent` for `q < 2^25` and `exponent` in `-1073..=1000` (all the
/// rounding below produces), exactly: the result's lowest bit is never below
/// 2^-1074.
fn scale(q: u64, exponent: i32) -> f64 {
    let power = |k: i32| f64::from_bits(((k + 1023) as u64) << 52);
    let v = q as f64;
    if exponent < -1022 {
        v * power(exponent + 100) * power(-100)
    } else {
        v * power(exponent)
    }
}

/// The exact value `s + error` (with `error` far below `s`'s last bit, only its
/// sign known: `error_sign`) rounded to a 24-bit significand with the rounding
/// mode `rounding`, the exponent unbounded. `s` is finite and nonzero and is
/// the exact value rounded to nearest `f64`, so `|error|` is at most half a unit
/// of `s`'s last place.
///
/// In units of a quarter of `s`'s last place, `4m + sign` (`m` the significand)
/// stands for the exact magnitude: every 24-bit value and every midpoint
/// between two is a multiple of `s`'s unit, and the exact magnitude lies on the
/// same side of each of them as `4m + sign` does (it differs from `m` by at
/// most half a unit, toward `sign`). It also has the exact magnitude's binade:
/// a power of two minus a little has one bit less, so the 24 bits are counted
/// from the right place.
#[inline]
fn round_to_single(s: f64, error_sign: i32, rounding: u16) -> f64 {
    if rounding == ROUND_NEAREST {
        if let Some(r) = round_to_single_nearest_fast(s, error_sign) {
            return r;
        }
    }
    round_to_single_exact(s, error_sign, rounding)
}

/// [`round_to_single_exact`] to nearest, for the common case, with the same answer: when `s`
/// is not exactly halfway between two 24-bit values, the exact value (within half of `s`'s
/// last place of it) rounds the way `s` does, so the hardware conversion to `f32` (round to
/// nearest, ties to even) gives the result; on a midpoint it does too when the error is zero.
/// Only for a result inside `f32`'s normal range (biased `f64` exponents 897..=1149): there the
/// unbounded exponent changes nothing and nothing overflows. `None` otherwise.
#[inline]
fn round_to_single_nearest_fast(s: f64, error_sign: i32) -> Option<f64> {
    let bits = s.to_bits();
    let exponent = (bits >> 52) & 0x7ff;
    if !(897..=1149).contains(&exponent) {
        return None;
    }
    if error_sign != 0 && bits & 0x1fff_ffff == 0x1000_0000 {
        return None;
    }
    Some(f64::from(s as f32))
}

fn round_to_single_exact(s: f64, error_sign: i32, rounding: u16) -> f64 {
    let bits = s.to_bits();
    let negative = bits >> 63 != 0;
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    let fraction = bits & F64_SIGNIFICAND;
    let (m, unit) = if exponent == 0 {
        (fraction, -1074)
    } else {
        (fraction | 1 << 52, exponent - 1075)
    };
    let magnitude_error = if negative { -error_sign } else { error_sign };
    let x = (m << 2).wrapping_add(magnitude_error as i64 as u64);
    let width = 64 - x.leading_zeros() as i32;
    if width <= 26 {
        // A subnormal below 2^-1050: 24 bits would reach past the f64's last
        // place (outside the model, see the module comment).
        return s;
    }
    let drop = width - 24;
    let q = x >> drop;
    let rest = x & ((1u64 << drop) - 1);
    let half = 1u64 << (drop - 1);
    let up = match rounding {
        ROUND_NEAREST => rest > half || (rest == half && q & 1 != 0),
        ROUND_DOWN => negative && rest != 0,
        ROUND_UP => !negative && rest != 0,
        _ => false, // toward zero
    };
    let magnitude = scale(q + u64::from(up), unit - 2 + drop);
    if negative {
        -magnitude
    } else {
        magnitude
    }
}

/// Below this magnitude the residual of a product, quotient or square root
/// (about 2^-53 of it) could underflow the f64 and lose its sign.
const TINY: f64 = 1e-270;

/// 2^200: residuals near the bottom of the range are computed on operands
/// scaled up by it (exact: a power of two, far from overflow there).
fn scale_up() -> f64 {
    f64::from_bits((1023 + 200) << 52)
}

/// The 24-bit result of an operation whose `f64` result is `s`, when it can be had without the
/// exact error: to nearest, `s` inside `f32`'s normal range and not exactly halfway between two
/// 24-bit values ([`round_to_single_nearest_fast`], which then never reads the error's sign).
/// `None` otherwise (zero, infinity and NaN included): the caller computes the error.
#[inline]
fn single_fast(s: f64, rounding: u16) -> Option<f64> {
    if rounding != ROUND_NEAREST {
        return None;
    }
    let bits = s.to_bits();
    if !(897..=1149).contains(&((bits >> 52) & 0x7ff)) || bits & 0x1fff_ffff == 0x1000_0000 {
        return None;
    }
    Some(f64::from(s as f32))
}

/// [`x87_add`]'s rounding of `s = a + b` under 24-bit precision.
#[inline]
fn single_sum(a: f64, b: f64, s: f64, rounding: u16) -> f64 {
    if let Some(r) = single_fast(s, rounding) {
        return r;
    }
    single_sum_exact(a, b, s, rounding)
}

/// [`single_sum`] with the exact error.
#[cold]
#[inline(never)]
fn single_sum_exact(a: f64, b: f64, s: f64, rounding: u16) -> f64 {
    if s == 0.0 {
        // An exact zero (an f64 sum that underflows is exact). Its sign is
        // `+` in every mode but rounding down, which gives `-0` unless both
        // operands are `+0` (IEEE 754, which the x87 follows).
        return if rounding == ROUND_DOWN && (a.to_bits() | b.to_bits()) != 0 {
            -0.0
        } else {
            s
        };
    }
    if !s.is_finite() {
        return s;
    }
    // Knuth's TwoSum: the rounding error of `a + b`, exactly.
    let bb = s - a;
    let error = (a - (s - bb)) + (b - bb);
    round_to_single(s, sign_of(error), rounding)
}

/// x87 `FADD`: `a + b` with the x87 choice of NaN ([`nan_result`]), rounded to
/// the precision of the control word ([`control_word`]).
#[inline]
pub fn x87_add(a: f64, b: f64) -> f64 {
    if let Some(nan) = nan_result(a, b) {
        return nan;
    }
    let s = a + b;
    match single_precision() {
        None => s,
        Some(rounding) => single_sum(a, b, s, rounding),
    }
}

/// x87 `FSUB`: `a - b` with the x87 choice of NaN, rounded to the precision of
/// the control word.
#[inline]
pub fn x87_sub(a: f64, b: f64) -> f64 {
    if let Some(nan) = nan_result(a, b) {
        return nan;
    }
    let s = a - b;
    match single_precision() {
        None => s,
        Some(rounding) => single_sum(a, -b, s, rounding),
    }
}

/// x87 `FMUL`: `a * b` with the x87 choice of NaN, rounded to the precision of
/// the control word.
#[inline]
pub fn x87_mul(a: f64, b: f64) -> f64 {
    if let Some(nan) = nan_result(a, b) {
        return nan;
    }
    let s = a * b;
    match single_precision() {
        // A zero product of nonzero operands underflowed the f64 (outside
        // the model); every other zero is exact.
        Some(rounding) if s != 0.0 && s.is_finite() => {
            if let Some(r) = single_fast(s, rounding) {
                return r;
            }
            // `a * b - s`, exactly. Near the bottom of the f64 range the
            // residual would underflow and lose its sign: then the smaller
            // operand and `s` are scaled up by 2^200 first (exactly).
            let (small, big) = if a.abs() < b.abs() { (a, b) } else { (b, a) };
            let error = if s.abs() < TINY {
                (small * scale_up()).mul_add(big, -(s * scale_up()))
            } else {
                small.mul_add(big, -s)
            };
            round_to_single(s, sign_of(error), rounding)
        }
        _ => s,
    }
}

/// x87 `FDIV`: `a / b` with the x87 choice of NaN, rounded to the precision of
/// the control word.
#[inline]
pub fn x87_div(a: f64, b: f64) -> f64 {
    if let Some(nan) = nan_result(a, b) {
        return nan;
    }
    let s = a / b;
    match single_precision() {
        Some(rounding) if s != 0.0 && s.is_finite() => {
            if let Some(r) = single_fast(s, rounding) {
                return r;
            }
            // The remainder `a - s * b`, exactly; the error of the quotient
            // has its sign times the divisor's.
            // (`a` and `s` scaled up near the bottom of the range, as in
            // `x87_mul`.)
            let remainder = if a.abs() < TINY {
                (-(s * scale_up())).mul_add(b, a * scale_up())
            } else {
                (-s).mul_add(b, a)
            };
            round_to_single(s, sign_of(remainder) * sign_of(b), rounding)
        }
        _ => s,
    }
}

/// x87 `FSQRT`: the square root, rounded to the precision of the control word.
/// A NaN comes back quieted (its payload kept); a negative number other than
/// `-0` gives the default NaN, as `f64::sqrt` does.
#[inline]
pub fn x87_sqrt(value: f64) -> f64 {
    if value.is_nan() {
        return f64::from_bits(value.to_bits() | F64_QUIET);
    }
    let s = value.sqrt();
    match single_precision() {
        Some(rounding) if s != 0.0 && s.is_finite() => {
            if let Some(r) = single_fast(s, rounding) {
                return r;
            }
            // `value - s * s`, exactly: the root's error has its sign (both
            // scaled up near the bottom of the range, as in `x87_mul`).
            let error = if value < TINY {
                let up = s * scale_up();
                (-up).mul_add(up, value * scale_up() * scale_up())
            } else {
                (-s).mul_add(s, value)
            };
            round_to_single(s, sign_of(error), rounding)
        }
        _ => s,
    }
}

/// `value` rounded to the precision of the control word: unchanged under
/// 53-bit precision, its significand rounded to 24 bits (rounding mode of the
/// control word, exponent unbounded) under 24-bit. For a translation that
/// computes in plain `f64` and needs the x87 result at one point.
///
/// It rounds `value` itself, which is exact for an `FADD`/`FSUB`/`FMUL`/
/// `FDIV`/`FSQRT` only when `value` is that instruction's exact result or, with
/// round to nearest, when the operands are floats (24-bit values): then
/// rounding to `f64` first never changes the 24-bit result (53 >= 2 * 24 + 2).
/// With `double` operands, or a directed rounding mode, use the instruction's
/// helper. NaNs and infinities come back unchanged.
#[inline]
pub fn x87_round(value: f64) -> f64 {
    match single_precision() {
        Some(rounding) if value != 0.0 && value.is_finite() => round_to_single(value, 0, rounding),
        _ => value,
    }
}

/// A `float` memory operand of an x87 arithmetic instruction (`FMUL dword
/// ptr [..]` and the like) as the instruction sees it: widened exactly, a
/// signalling NaN still signalling, so the helpers above can tell it from a
/// quiet one. A value loaded by `FLD dword` is quiet: use
/// `f64::from(x87_quiet(..))` for that one.
#[inline]
pub fn x87_m32(bits: u32) -> f64 {
    let sign = u64::from(bits >> 31) << 63;
    let exponent = (bits >> 23) & 0xff;
    let mantissa = u64::from(bits & 0x007f_ffff);
    if exponent == 0xff && mantissa != 0 {
        f64::from_bits(sign | F64_EXPONENT | (mantissa << 29))
    } else {
        f64::from(f32::from_bits(bits))
    }
}

/// `FLD dword`: the value the x87 stack holds after loading the float with
/// the bit pattern `bits` (a signalling NaN quieted, everything else exact).
#[inline]
pub fn x87_load32(bits: u32) -> f64 {
    f64::from(f32::from_bits(x87_quiet_bits(bits)))
}

/// `FLD qword`: the value the x87 stack holds after loading the double with the
/// bit pattern `bits` (a signalling NaN quieted, everything else exact). A
/// `double` memory operand of an arithmetic instruction is the plain
/// `f64::from_bits(bits)` instead (still signalling).
#[inline]
pub fn x87_load64(bits: u64) -> f64 {
    if is_nan_bits(bits) {
        f64::from_bits(bits | F64_QUIET)
    } else {
        f64::from_bits(bits)
    }
}

/// `FSTP dword`: the stack value rounded to a float, as its bits (a NaN keeps
/// its sign and the top of its payload).
#[inline]
pub fn x87_store32(value: f64) -> u32 {
    (value as f32).to_bits()
}

// Helpers of several engine crates, kept here so each reaches them: `fistp`
// (from `units/bsmenu/font.rs`) and `rsqrt_estimate` (from
// `units/fallout_shared/tesobjectrefr.rs`, which the Havok units call too).

/// `FISTP` with the default rounding mode: nearest, ties to even; the
/// integer indefinite 0x80000000 for NaN and values out of range.
pub fn fistp(value: f64) -> i32 {
    // Round half to even (the FPU's default mode) without `round_ties_even`,
    // which is newer than the crate's MSRV: `round` goes half away from zero,
    // so an odd result of a tie steps back toward zero.
    let mut r = value.round();
    if (value - value.trunc()).abs() == 0.5 && r % 2.0 != 0.0 {
        r -= value.signum();
    }
    if r.is_nan() || r < i32::MIN as f64 || r > i32::MAX as f64 {
        i32::MIN
    } else {
        r as i32
    }
}

/// `FISTP dword` under rounding toward zero (the compiler's `FNSTCW`, `OR
/// 0xC00`, `FLDCW`, `FISTP`, `FLDCW` sequence): the value truncated; the
/// integer indefinite 0x80000000 for NaN and values out of range.
pub fn fistp_chop(value: f64) -> i32 {
    let r = value.trunc();
    if r.is_nan() || r < i32::MIN as f64 || r > i32::MAX as f64 {
        i32::MIN
    } else {
        r as i32
    }
}

/// The `CMPEQSS`/`RSQRTSS` pair of the Havok normalizer: the hardware
/// estimate of `1 / sqrt(x)` on x86, exact elsewhere.
#[cfg(target_arch = "x86_64")]
pub fn rsqrt_estimate(x: f32) -> f32 {
    use std::arch::x86_64::{_mm_cvtss_f32, _mm_rsqrt_ss, _mm_set_ss};
    // SAFETY: SSE is part of the x86_64 baseline.
    unsafe { _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x))) }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn rsqrt_estimate(x: f32) -> f32 {
    1.0 / x.sqrt()
}

// ---- Transcendental and other non-arithmetic instructions ------------------
//
// `FSIN`, `FCOS`, `FSINCOS`, `FPTAN`, `FPATAN`, `F2XM1`, `FYL2X`, `FYL2XP1`,
// `FSCALE`, `FRNDINT`, `FPREM` and `FPREM1` compute to the extended format and
// differ from Rust's `f64` functions (the FPU reduces arguments with its own
// 66-bit pi, rounds once from its internal result, and so on). On x86_64 the
// helpers below run the instruction itself on the host's FPU: the operands are
// loaded as the exe loads them (`FLD qword`/`FLD dword` into the 80-bit
// register), the instruction runs under the running game thread's control word
// ([`control_word`], every exception masked so the host never traps), and the
// result is kept as the 80-bit value ([`X87Ext`]) until the translation stores
// it as the exe does ([`X87Ext::store64`] for `FSTP qword`, [`X87Ext::store32`]
// for `FSTP dword`): the same bits as the game's x87. Precision control does
// not apply to these instructions (Intel SDM vol. 1, 8.1.5.2); the rounding
// mode does (`FRNDINT`, and the stores).
//
// Elsewhere (not x86_64) a portable fallback computes them with Rust's `f64`
// functions: close, but not bit-exact (marked in `soft`).

/// An x87 register's value as the FPU holds it: the 80-bit extended format as
/// `FSTP tbyte` stores it (little-endian: the 64-bit significand with its
/// explicit integer bit, then sign and 15-bit exponent).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct X87Ext(pub [u8; 10]);

impl X87Ext {
    /// The value from its sign-and-exponent word and its 64-bit significand.
    pub fn from_parts(sign_exponent: u16, significand: u64) -> Self {
        let mut bytes = [0u8; 10];
        bytes[..8].copy_from_slice(&significand.to_le_bytes());
        bytes[8..].copy_from_slice(&sign_exponent.to_le_bytes());
        X87Ext(bytes)
    }

    /// The 64-bit significand (bit 63 is the explicit integer bit).
    pub fn significand(self) -> u64 {
        let mut word = [0u8; 8];
        word.copy_from_slice(&self.0[..8]);
        u64::from_le_bytes(word)
    }

    /// The sign (bit 15) and biased exponent (bits 0-14).
    pub fn sign_exponent(self) -> u16 {
        u16::from_le_bytes([self.0[8], self.0[9]])
    }

    /// `FLD qword`: the double `value` in a register (exact; a signalling NaN
    /// quieted).
    pub fn load64(value: f64) -> Self {
        imp::load64(value)
    }

    /// `FLD dword`: the float with the bit pattern `bits` in a register
    /// (exact; a signalling NaN quieted).
    pub fn load32(bits: u32) -> Self {
        imp::load32(bits)
    }

    /// `FSTP qword`: the value rounded to a double with the control word's
    /// rounding mode.
    pub fn store64(self) -> f64 {
        imp::store64(self)
    }

    /// `FSTP dword`: the value rounded to a float with the control word's
    /// rounding mode, as its bits.
    pub fn store32(self) -> u32 {
        imp::store32(self)
    }
}

impl From<f64> for X87Ext {
    /// [`X87Ext::load64`].
    fn from(value: f64) -> Self {
        X87Ext::load64(value)
    }
}

/// Status word bit C2: `FPREM`/`FPREM1` incomplete, or the operand of `FSIN`/
/// `FCOS`/`FSINCOS`/`FPTAN` out of range (|x| >= 2^63).
pub const X87_STATUS_C2: u16 = 0x0400;

/// `FSIN`: the sine of ST0. An operand out of range (|x| >= 2^63) comes back
/// unchanged, as the FPU leaves it (C2 set).
#[inline]
pub fn x87_fsin(x: X87Ext) -> X87Ext {
    imp::fsin(x)
}

/// `FCOS`: the cosine of ST0 (out of range: unchanged, as [`x87_fsin`]).
#[inline]
pub fn x87_fcos(x: X87Ext) -> X87Ext {
    imp::fcos(x)
}

/// `FSINCOS`: `(sine, cosine)` of ST0 (afterwards ST1 holds the sine and ST0
/// the cosine). An operand out of range (|x| >= 2^63) pushes nothing on the
/// FPU, which the code after it does not expect: that stops with a panic.
#[inline]
pub fn x87_fsincos(x: X87Ext) -> (X87Ext, X87Ext) {
    imp::fsincos(x)
}

/// `FPTAN`: the tangent of ST0 (the 1.0 the instruction then pushes is not
/// returned: the exe pops it). An operand out of range stops as
/// [`x87_fsincos`] does.
#[inline]
pub fn x87_fptan(x: X87Ext) -> X87Ext {
    imp::fptan(x)
}

/// `FPATAN` with ST1 = `y` and ST0 = `x`: the arctangent of `y / x` with the
/// quadrant of the signs (`atan2(y, x)`), left in ST0 after the pop.
#[inline]
pub fn x87_fpatan(y: X87Ext, x: X87Ext) -> X87Ext {
    imp::fpatan(y, x)
}

/// `F2XM1`: `2^x - 1` (defined for -1 <= x <= 1).
#[inline]
pub fn x87_f2xm1(x: X87Ext) -> X87Ext {
    imp::f2xm1(x)
}

/// `FYL2X` with ST1 = `y` and ST0 = `x`: `y * log2(x)` after the pop.
#[inline]
pub fn x87_fyl2x(y: X87Ext, x: X87Ext) -> X87Ext {
    imp::fyl2x(y, x)
}

/// `FYL2XP1` with ST1 = `y` and ST0 = `x`: `y * log2(x + 1)` after the pop.
#[inline]
pub fn x87_fyl2xp1(y: X87Ext, x: X87Ext) -> X87Ext {
    imp::fyl2xp1(y, x)
}

/// `FSCALE` with ST0 = `x` and ST1 = `scale`: `x * 2^trunc(scale)` (the new
/// ST0; ST1 is left as it was).
#[inline]
pub fn x87_fscale(x: X87Ext, scale: X87Ext) -> X87Ext {
    imp::fscale(x, scale)
}

/// `FRNDINT`: ST0 rounded to an integer with the control word's rounding mode.
#[inline]
pub fn x87_frndint(x: X87Ext) -> X87Ext {
    imp::frndint(x)
}

/// `FPREM` with ST0 = `x` and ST1 = `y`: the partial remainder of `x / y` with
/// the quotient truncated (`fmod`), and the status word right after the
/// instruction (C2 [`X87_STATUS_C2`] set: incomplete, run it again; C0, C3, C1
/// the low three quotient bits).
#[inline]
pub fn x87_fprem(x: X87Ext, y: X87Ext) -> (X87Ext, u16) {
    imp::fprem(x, y)
}

/// `FPREM1`: as [`x87_fprem`] with the quotient rounded to nearest (the IEEE
/// remainder).
#[inline]
pub fn x87_fprem1(x: X87Ext, y: X87Ext) -> (X87Ext, u16) {
    imp::fprem1(x, y)
}

/// `FISTP qword`: ST0 stored as a 64-bit integer, rounded with the control
/// word's rounding mode ([`control_word`]); the integer indefinite
/// 0x8000_0000_0000_0000 for a NaN or a value out of range.
#[inline]
pub fn x87_fistp64(x: X87Ext) -> i64 {
    imp::fistp64(x, control_word())
}

/// `FISTP qword` inside the compiler's truncating sequence (`FNSTCW`, `OR
/// 0xC00`, `FLDCW`, `FISTP`, `FLDCW`, as `_ftol2`-style code sets it): the
/// value rounded toward zero, the integer indefinite for a NaN or a value out
/// of range. The thread's control word is left as it was.
#[inline]
pub fn x87_fistp64_chop(x: X87Ext) -> i64 {
    imp::fistp64(x, control_word() | ROUNDING_CONTROL)
}

// ---- Arithmetic on register values -----------------------------------------
//
// `FADD`, `FSUB`, `FMUL`, `FDIV` and `FSQRT` on the extended values the
// registers hold (the `ST(i)` forms, and a memory operand once it is loaded:
// `X87Ext::load64` of a double is exact). The result is rounded once, as the
// FPU rounds it, to the precision control and rounding mode of the running
// game thread's control word ([`control_word`]), with the extended exponent
// range; the NaN choice is the x87's. On x86_64 the instruction runs on the
// host's FPU like the transcendental helpers above (encoded as bytes, so the
// operand order is the one written here); elsewhere the portable fallback
// computes in `f64` (not bit-exact).

/// `FADD`: `a + b`.
#[inline]
pub fn x87_fadd(a: X87Ext, b: X87Ext) -> X87Ext {
    imp::fadd(a, b)
}

/// `FSUB` with ST0 = `a`: `a - b` (the reversed forms are `x87_fsub(b, a)`).
#[inline]
pub fn x87_fsub(a: X87Ext, b: X87Ext) -> X87Ext {
    imp::fsub(a, b)
}

/// `FMUL`: `a * b`.
#[inline]
pub fn x87_fmul(a: X87Ext, b: X87Ext) -> X87Ext {
    imp::fmul(a, b)
}

/// `FDIV` with ST0 = `a`: `a / b` (the reversed forms are `x87_fdiv(b, a)`).
#[inline]
pub fn x87_fdiv(a: X87Ext, b: X87Ext) -> X87Ext {
    imp::fdiv(a, b)
}

/// `FSQRT` of ST0.
#[inline]
pub fn x87_fsqrt(a: X87Ext) -> X87Ext {
    imp::fsqrt(a)
}

/// Status word bits C0 and C3, which `FCOM`/`FTST`/`FXAM` set (C2 is
/// [`X87_STATUS_C2`]); `FNSTSW AX` puts them in `AH` as 0x01 (C0), 0x04 (C2)
/// and 0x40 (C3).
pub const X87_STATUS_C0: u16 = 0x0100;
pub const X87_STATUS_C3: u16 = 0x4000;

/// `FCOM` of ST0 = `a` with `b` (and `FCOMP`/`FCOMPP`/`FUCOM`, which set the
/// same condition codes): the status word's C3, C2 and C0 (`a > b`: none;
/// `a < b`: C0; equal: C3; unordered: all three).
#[inline]
pub fn x87_fcom(a: X87Ext, b: X87Ext) -> u16 {
    imp::fcom(a, b) & (X87_STATUS_C0 | X87_STATUS_C2 | X87_STATUS_C3)
}

/// `FXAM` of a (non-empty) register: the status word's C3, C2, C1 and C0 (C1 the sign; C3 C2
/// C0 the class: 000 unsupported, 001 NaN, 010 normal, 011 infinity, 100 zero, 110 denormal).
/// The class is the register's encoding: an exponent of 0x7fff with the integer bit set is an
/// infinity (fraction zero) or a NaN, without it unsupported; a zero exponent is a zero
/// (significand zero) or a denormal; any other exponent is normal with the integer bit set and
/// unsupported (an unnormal) without it.
#[inline]
pub fn x87_fxam(x: X87Ext) -> u16 {
    const C1: u16 = 0x0200;
    let sign = if x.sign_exponent() & 0x8000 != 0 {
        C1
    } else {
        0
    };
    let exponent = x.sign_exponent() & 0x7fff;
    let significand = x.significand();
    let integer = significand & (1 << 63) != 0;
    let class = match exponent {
        0x7fff if !integer => 0,
        0x7fff if significand << 1 == 0 => X87_STATUS_C2 | X87_STATUS_C0,
        0x7fff => X87_STATUS_C0,
        0 if significand == 0 => X87_STATUS_C3,
        0 => X87_STATUS_C3 | X87_STATUS_C2,
        _ if integer => X87_STATUS_C2,
        _ => 0,
    };
    sign | class
}

/// `FCHS`: the sign bit flipped (NaNs included).
#[inline]
pub fn x87_fchs(a: X87Ext) -> X87Ext {
    let mut bytes = a.0;
    bytes[9] ^= 0x80;
    X87Ext(bytes)
}

/// `FABS`: the sign bit cleared (NaNs included).
#[inline]
pub fn x87_fabs(a: X87Ext) -> X87Ext {
    let mut bytes = a.0;
    bytes[9] &= 0x7f;
    X87Ext(bytes)
}

/// The constants the x87 loads by instruction (`FLDL2E`, `FLDL2T`, `FLDLG2`,
/// `FLDLN2`, `FLDPI`): rounded to 64 bits with the control word's rounding
/// mode.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum X87Constant {
    /// `FLDL2E`: log2(e).
    Log2E,
    /// `FLDL2T`: log2(10).
    Log2Ten,
    /// `FLDLG2`: log10(2).
    Log10Two,
    /// `FLDLN2`: ln(2).
    LnTwo,
    /// `FLDPI`: pi.
    Pi,
}

/// `FLDL2E` and the like: the constant as the running thread's control word
/// rounds it.
#[inline]
pub fn x87_fld_constant(constant: X87Constant) -> X87Ext {
    imp::fld_constant(constant)
}

#[cfg(target_arch = "x86_64")]
use hw as imp;
#[cfg(not(target_arch = "x86_64"))]
use soft as imp;

/// The instructions on the host's x87 (x86_64).
#[cfg(target_arch = "x86_64")]
mod hw {
    use super::X87Ext;

    /// The memory one instruction sequence works on, at fixed offsets the
    /// assembly names: operands `a` (+0) and `b` (+10), results `r0` (+20) and
    /// `r1` (+30), the control word to run under (+40), the host's saved one
    /// (+42) and the status word (+44).
    #[repr(C)]
    struct Frame {
        a: [u8; 10],
        b: [u8; 10],
        r0: [u8; 10],
        r1: [u8; 10],
        word: u16,
        saved: u16,
        status: u16,
    }

    fn frame(a: [u8; 10], b: [u8; 10]) -> Frame {
        Frame {
            a,
            b,
            r0: [0; 10],
            r1: [0; 10],
            // The game thread's control word with every exception masked: an
            // unmasked one would trap in the host (the game runs with all of
            // them masked, `DEFAULT_CONTROL_WORD` and 0x007F).
            word: super::control_word() | 0x003f,
            saved: 0,
            status: 0,
        }
    }

    /// Runs the instructions on `$frame` under its control word: saves the
    /// host's control word, loads the frame's, runs them, then `FNINIT` (an
    /// empty register stack and cleared flags, whatever the instructions left)
    /// and the host's control word again.
    macro_rules! run {
        ($frame:expr, $($code:literal),+ $(,)?) => {
            // SAFETY: the instructions read and write only the frame (through
            // the pointer in `f`), leave the x87 stack empty (FNINIT) and the
            // host's control word as it was; every x87 register is declared
            // clobbered.
            unsafe {
                core::arch::asm!(
                    "fnstcw word ptr [{f} + 42]",
                    "fldcw word ptr [{f} + 40]",
                    $($code,)+
                    "fninit",
                    "fldcw word ptr [{f} + 42]",
                    f = in(reg) core::ptr::addr_of_mut!($frame),
                    out("st(0)") _, out("st(1)") _, out("st(2)") _, out("st(3)") _,
                    out("st(4)") _, out("st(5)") _, out("st(6)") _, out("st(7)") _,
                    options(nostack),
                )
            }
        };
    }

    fn word8(bytes: &[u8]) -> [u8; 8] {
        let mut out = [0u8; 8];
        out.copy_from_slice(&bytes[..8]);
        out
    }

    pub fn load64(value: f64) -> X87Ext {
        let mut a = [0u8; 10];
        a[..8].copy_from_slice(&value.to_bits().to_le_bytes());
        let mut f = frame(a, [0; 10]);
        run!(f, "fld qword ptr [{f}]", "fstp tbyte ptr [{f} + 20]");
        X87Ext(f.r0)
    }

    pub fn load32(bits: u32) -> X87Ext {
        let mut a = [0u8; 10];
        a[..4].copy_from_slice(&bits.to_le_bytes());
        let mut f = frame(a, [0; 10]);
        run!(f, "fld dword ptr [{f}]", "fstp tbyte ptr [{f} + 20]");
        X87Ext(f.r0)
    }

    pub fn store64(x: X87Ext) -> f64 {
        let mut f = frame(x.0, [0; 10]);
        run!(f, "fld tbyte ptr [{f}]", "fstp qword ptr [{f} + 20]");
        f64::from_bits(u64::from_le_bytes(word8(&f.r0)))
    }

    pub fn store32(x: X87Ext) -> u32 {
        let mut f = frame(x.0, [0; 10]);
        run!(f, "fld tbyte ptr [{f}]", "fstp dword ptr [{f} + 20]");
        u32::from_le_bytes([f.r0[0], f.r0[1], f.r0[2], f.r0[3]])
    }

    pub fn fsin(x: X87Ext) -> X87Ext {
        let mut f = frame(x.0, [0; 10]);
        run!(
            f,
            "fld tbyte ptr [{f}]",
            "fsin",
            "fstp tbyte ptr [{f} + 20]"
        );
        X87Ext(f.r0)
    }

    pub fn fcos(x: X87Ext) -> X87Ext {
        let mut f = frame(x.0, [0; 10]);
        run!(
            f,
            "fld tbyte ptr [{f}]",
            "fcos",
            "fstp tbyte ptr [{f} + 20]"
        );
        X87Ext(f.r0)
    }

    /// Stops when C2 says the operand was out of range: the instruction
    /// pushed nothing.
    fn check_pushed(name: &str, x: X87Ext, status: u16) {
        assert!(
            status & super::X87_STATUS_C2 == 0,
            "{name} of {x:?}: operand out of range (|x| >= 2^63), nothing pushed"
        );
    }

    pub fn fsincos(x: X87Ext) -> (X87Ext, X87Ext) {
        let mut f = frame(x.0, [0; 10]);
        run!(
            f,
            "fld tbyte ptr [{f}]",
            "fsincos",
            "fnstsw word ptr [{f} + 44]",
            "fstp tbyte ptr [{f} + 30]",
            "fstp tbyte ptr [{f} + 20]",
        );
        check_pushed("FSINCOS", x, f.status);
        (X87Ext(f.r0), X87Ext(f.r1))
    }

    pub fn fptan(x: X87Ext) -> X87Ext {
        let mut f = frame(x.0, [0; 10]);
        run!(
            f,
            "fld tbyte ptr [{f}]",
            "fptan",
            "fnstsw word ptr [{f} + 44]",
            "fstp tbyte ptr [{f} + 30]",
            "fstp tbyte ptr [{f} + 20]",
        );
        check_pushed("FPTAN", x, f.status);
        X87Ext(f.r0)
    }

    pub fn fpatan(y: X87Ext, x: X87Ext) -> X87Ext {
        let mut f = frame(y.0, x.0);
        run!(
            f,
            "fld tbyte ptr [{f}]",
            "fld tbyte ptr [{f} + 10]",
            "fpatan",
            "fstp tbyte ptr [{f} + 20]",
        );
        X87Ext(f.r0)
    }

    pub fn f2xm1(x: X87Ext) -> X87Ext {
        let mut f = frame(x.0, [0; 10]);
        run!(
            f,
            "fld tbyte ptr [{f}]",
            "f2xm1",
            "fstp tbyte ptr [{f} + 20]"
        );
        X87Ext(f.r0)
    }

    pub fn fyl2x(y: X87Ext, x: X87Ext) -> X87Ext {
        let mut f = frame(y.0, x.0);
        run!(
            f,
            "fld tbyte ptr [{f}]",
            "fld tbyte ptr [{f} + 10]",
            "fyl2x",
            "fstp tbyte ptr [{f} + 20]",
        );
        X87Ext(f.r0)
    }

    pub fn fyl2xp1(y: X87Ext, x: X87Ext) -> X87Ext {
        let mut f = frame(y.0, x.0);
        run!(
            f,
            "fld tbyte ptr [{f}]",
            "fld tbyte ptr [{f} + 10]",
            "fyl2xp1",
            "fstp tbyte ptr [{f} + 20]",
        );
        X87Ext(f.r0)
    }

    pub fn fscale(x: X87Ext, scale: X87Ext) -> X87Ext {
        let mut f = frame(x.0, scale.0);
        run!(
            f,
            "fld tbyte ptr [{f} + 10]",
            "fld tbyte ptr [{f}]",
            "fscale",
            "fstp tbyte ptr [{f} + 20]",
        );
        X87Ext(f.r0)
    }

    pub fn frndint(x: X87Ext) -> X87Ext {
        let mut f = frame(x.0, [0; 10]);
        run!(
            f,
            "fld tbyte ptr [{f}]",
            "frndint",
            "fstp tbyte ptr [{f} + 20]"
        );
        X87Ext(f.r0)
    }

    pub fn fprem(x: X87Ext, y: X87Ext) -> (X87Ext, u16) {
        let mut f = frame(x.0, y.0);
        run!(
            f,
            "fld tbyte ptr [{f} + 10]",
            "fld tbyte ptr [{f}]",
            "fprem",
            "fnstsw word ptr [{f} + 44]",
            "fstp tbyte ptr [{f} + 20]",
        );
        (X87Ext(f.r0), f.status)
    }

    pub fn fprem1(x: X87Ext, y: X87Ext) -> (X87Ext, u16) {
        let mut f = frame(x.0, y.0);
        run!(
            f,
            "fld tbyte ptr [{f} + 10]",
            "fld tbyte ptr [{f}]",
            "fprem1",
            "fnstsw word ptr [{f} + 44]",
            "fstp tbyte ptr [{f} + 20]",
        );
        (X87Ext(f.r0), f.status)
    }

    /// `FISTP qword` under the control word `word` (exceptions masked).
    pub fn fistp64(x: X87Ext, word: u16) -> i64 {
        let mut f = frame(x.0, [0; 10]);
        f.word = word | 0x003f;
        run!(f, "fld tbyte ptr [{f}]", "fistp qword ptr [{f} + 20]");
        i64::from_le_bytes(word8(&f.r0))
    }

    // The arithmetic helpers load `b` first, so ST0 = `a` and ST1 = `b`, and
    // run the `ST(0), ST(1)` form, written as its bytes: D8 C1 `FADD`, D8 E1
    // `FSUB` (ST0 - ST1), D8 C9 `FMUL`, D8 F1 `FDIV` (ST0 / ST1), D8 D1
    // `FCOM ST(1)`.

    pub fn fadd(a: X87Ext, b: X87Ext) -> X87Ext {
        let mut f = frame(a.0, b.0);
        run!(
            f,
            "fld tbyte ptr [{f} + 10]",
            "fld tbyte ptr [{f}]",
            ".byte 0xd8, 0xc1",
            "fstp tbyte ptr [{f} + 20]",
        );
        X87Ext(f.r0)
    }

    pub fn fsub(a: X87Ext, b: X87Ext) -> X87Ext {
        let mut f = frame(a.0, b.0);
        run!(
            f,
            "fld tbyte ptr [{f} + 10]",
            "fld tbyte ptr [{f}]",
            ".byte 0xd8, 0xe1",
            "fstp tbyte ptr [{f} + 20]",
        );
        X87Ext(f.r0)
    }

    pub fn fmul(a: X87Ext, b: X87Ext) -> X87Ext {
        let mut f = frame(a.0, b.0);
        run!(
            f,
            "fld tbyte ptr [{f} + 10]",
            "fld tbyte ptr [{f}]",
            ".byte 0xd8, 0xc9",
            "fstp tbyte ptr [{f} + 20]",
        );
        X87Ext(f.r0)
    }

    pub fn fdiv(a: X87Ext, b: X87Ext) -> X87Ext {
        let mut f = frame(a.0, b.0);
        run!(
            f,
            "fld tbyte ptr [{f} + 10]",
            "fld tbyte ptr [{f}]",
            ".byte 0xd8, 0xf1",
            "fstp tbyte ptr [{f} + 20]",
        );
        X87Ext(f.r0)
    }

    pub fn fsqrt(a: X87Ext) -> X87Ext {
        let mut f = frame(a.0, [0; 10]);
        run!(
            f,
            "fld tbyte ptr [{f}]",
            "fsqrt",
            "fstp tbyte ptr [{f} + 20]"
        );
        X87Ext(f.r0)
    }

    pub fn fcom(a: X87Ext, b: X87Ext) -> u16 {
        let mut f = frame(a.0, b.0);
        run!(
            f,
            "fld tbyte ptr [{f} + 10]",
            "fld tbyte ptr [{f}]",
            ".byte 0xd8, 0xd1",
            "fnstsw word ptr [{f} + 44]",
        );
        f.status
    }

    pub fn fld_constant(constant: super::X87Constant) -> X87Ext {
        use super::X87Constant::*;
        let mut f = frame([0; 10], [0; 10]);
        match constant {
            Log2E => run!(f, "fldl2e", "fstp tbyte ptr [{f} + 20]"),
            Log2Ten => run!(f, "fldl2t", "fstp tbyte ptr [{f} + 20]"),
            Log10Two => run!(f, "fldlg2", "fstp tbyte ptr [{f} + 20]"),
            LnTwo => run!(f, "fldln2", "fstp tbyte ptr [{f} + 20]"),
            Pi => run!(f, "fldpi", "fstp tbyte ptr [{f} + 20]"),
        }
        X87Ext(f.r0)
    }
}

/// Portable fallback (not x86_64): the extended format converted in software
/// and the instructions computed with Rust's `f64` functions. NOT bit-exact
/// with the x87: the results are the `f64` library's, a store to a float
/// rounds twice, the rounding mode of the control word is honoured only by
/// `FRNDINT`, and the status word reports every `FPREM`/`FPREM1` complete. It
/// is compiled (and its conversions tested) on x86_64 too.
#[cfg_attr(target_arch = "x86_64", allow(dead_code))]
mod soft {
    use super::X87Ext;

    const EXT_BIAS: i32 = 16383;
    const INTEGER_BIT: u64 = 1 << 63;

    pub fn load64(value: f64) -> X87Ext {
        let bits = value.to_bits();
        let sign = ((bits >> 63) as u16) << 15;
        let exponent = ((bits >> 52) & 0x7ff) as i32;
        let fraction = bits & 0x000f_ffff_ffff_ffff;
        match exponent {
            0 if fraction == 0 => X87Ext::from_parts(sign, 0),
            0 => {
                // A double denormal is a normal extended value.
                let shift = fraction.leading_zeros();
                let e = -1022 - (shift as i32 - 11) + EXT_BIAS;
                X87Ext::from_parts(sign | e as u16, fraction << shift)
            }
            0x7ff if fraction == 0 => X87Ext::from_parts(sign | 0x7fff, INTEGER_BIT),
            // A NaN, quieted (bit 62 of the significand), payload kept.
            0x7ff => X87Ext::from_parts(sign | 0x7fff, INTEGER_BIT | (1 << 62) | (fraction << 11)),
            _ => X87Ext::from_parts(
                sign | (exponent - 1023 + EXT_BIAS) as u16,
                INTEGER_BIT | (fraction << 11),
            ),
        }
    }

    pub fn load32(bits: u32) -> X87Ext {
        // Widened exactly with the payload kept, then quieted by `load64`.
        load64(super::x87_m32(bits))
    }

    /// `v * 2^k` for any `k` the conversion needs, through two in-range
    /// factors applied one after the other.
    fn scaled(v: f64, k: i32) -> f64 {
        let half = k / 2;
        v * 2f64.powi(half) * 2f64.powi(k - half)
    }

    pub fn store64(x: X87Ext) -> f64 {
        let sign_exponent = x.sign_exponent();
        let negative = sign_exponent & 0x8000 != 0;
        let exponent = i32::from(sign_exponent & 0x7fff);
        let significand = x.significand();
        let magnitude = if exponent == 0x7fff {
            if significand << 1 == 0 {
                f64::INFINITY
            } else {
                f64::from_bits(0x7ff0_0000_0000_0000 | ((significand << 1) >> 12) | (1 << 51))
            }
        } else {
            // Rounded to 53 bits by the integer conversion (to nearest), then
            // scaled (exact unless the result is a double denormal).
            scaled(significand as f64, exponent - EXT_BIAS - 63)
        };
        if negative {
            -magnitude
        } else {
            magnitude
        }
    }

    pub fn store32(x: X87Ext) -> u32 {
        super::x87_store32(store64(x))
    }

    fn unary(x: X87Ext, op: impl Fn(f64) -> f64) -> X87Ext {
        load64(op(store64(x)))
    }

    pub fn fsin(x: X87Ext) -> X87Ext {
        unary(x, f64::sin)
    }

    pub fn fcos(x: X87Ext) -> X87Ext {
        unary(x, f64::cos)
    }

    pub fn fsincos(x: X87Ext) -> (X87Ext, X87Ext) {
        (fsin(x), fcos(x))
    }

    pub fn fptan(x: X87Ext) -> X87Ext {
        unary(x, f64::tan)
    }

    pub fn fpatan(y: X87Ext, x: X87Ext) -> X87Ext {
        load64(store64(y).atan2(store64(x)))
    }

    pub fn f2xm1(x: X87Ext) -> X87Ext {
        unary(x, |v| (v * std::f64::consts::LN_2).exp_m1())
    }

    pub fn fyl2x(y: X87Ext, x: X87Ext) -> X87Ext {
        load64(store64(y) * store64(x).log2())
    }

    pub fn fyl2xp1(y: X87Ext, x: X87Ext) -> X87Ext {
        load64(store64(y) * (store64(x).ln_1p() / std::f64::consts::LN_2))
    }

    pub fn fadd(a: X87Ext, b: X87Ext) -> X87Ext {
        load64(store64(a) + store64(b))
    }

    pub fn fsub(a: X87Ext, b: X87Ext) -> X87Ext {
        load64(store64(a) - store64(b))
    }

    pub fn fmul(a: X87Ext, b: X87Ext) -> X87Ext {
        load64(store64(a) * store64(b))
    }

    pub fn fdiv(a: X87Ext, b: X87Ext) -> X87Ext {
        load64(store64(a) / store64(b))
    }

    pub fn fsqrt(a: X87Ext) -> X87Ext {
        unary(a, f64::sqrt)
    }

    pub fn fcom(a: X87Ext, b: X87Ext) -> u16 {
        let (a, b) = (store64(a), store64(b));
        match a.partial_cmp(&b) {
            Some(std::cmp::Ordering::Greater) => 0,
            Some(std::cmp::Ordering::Less) => super::X87_STATUS_C0,
            Some(std::cmp::Ordering::Equal) => super::X87_STATUS_C3,
            None => super::X87_STATUS_C0 | super::X87_STATUS_C2 | super::X87_STATUS_C3,
        }
    }

    pub fn fld_constant(constant: super::X87Constant) -> X87Ext {
        use super::X87Constant::*;
        load64(match constant {
            Log2E => std::f64::consts::LOG2_E,
            Log2Ten => std::f64::consts::LOG2_10,
            Log10Two => std::f64::consts::LOG10_2,
            LnTwo => std::f64::consts::LN_2,
            Pi => std::f64::consts::PI,
        })
    }

    pub fn fscale(x: X87Ext, scale: X87Ext) -> X87Ext {
        let s = store64(scale).trunc();
        load64(store64(x) * 2f64.powf(s))
    }

    pub fn frndint(x: X87Ext) -> X87Ext {
        let v = store64(x);
        let rounded = match super::control_word() & super::ROUNDING_CONTROL {
            super::ROUND_DOWN => v.floor(),
            super::ROUND_UP => v.ceil(),
            super::ROUND_NEAREST => {
                // Ties to even.
                let mut r = v.round();
                if (v - v.trunc()).abs() == 0.5 && r % 2.0 != 0.0 {
                    r -= v.signum();
                }
                r
            }
            _ => v.trunc(),
        };
        load64(rounded)
    }

    pub fn fprem(x: X87Ext, y: X87Ext) -> (X87Ext, u16) {
        (load64(store64(x) % store64(y)), 0)
    }

    pub fn fprem1(x: X87Ext, y: X87Ext) -> (X87Ext, u16) {
        let (a, b) = (store64(x), store64(y));
        let q = a / b;
        let mut n = q.round();
        if (q - q.trunc()).abs() == 0.5 && n % 2.0 != 0.0 {
            n -= q.signum();
        }
        (load64(a - n * b), 0)
    }

    pub fn fistp64(x: X87Ext, word: u16) -> i64 {
        let v = store64(x);
        let r = match word & super::ROUNDING_CONTROL {
            super::ROUND_DOWN => v.floor(),
            super::ROUND_UP => v.ceil(),
            super::ROUND_NEAREST => {
                let mut r = v.round();
                if (v - v.trunc()).abs() == 0.5 && r % 2.0 != 0.0 {
                    r -= v.signum();
                }
                r
            }
            _ => v.trunc(),
        };
        // -2^63 is the only value of 2^63's magnitude in range.
        if r.is_nan() || r >= 2f64.powi(63) || r < -(2f64.powi(63)) {
            i64::MIN
        } else {
            r as i64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The fast path to nearest answers exactly what the exact rounding does wherever it
    /// answers: random values over the whole `f64` range, values on and next to the 24-bit
    /// midpoints and at the edges of `f32`'s normal range, each with the three error signs.
    #[test]
    fn the_fast_rounding_to_nearest_matches_the_exact_one() {
        let mut values = Vec::new();
        let mut state = 0x9e37_79b9_7f4a_7c15u64;
        for _ in 0..200_000 {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            values.push(f64::from_bits(state));
        }
        for exponent in [896u64, 897, 898, 1000, 1023, 1100, 1148, 1149, 1150] {
            for low in [0u64, 1, 0x0fff_ffff, 0x1000_0000, 0x1000_0001, 0x1fff_ffff] {
                for high in [0u64, 1 << 29, 0x000f_ffff_e000_0000] {
                    for sign in [0u64, 1 << 63] {
                        values.push(f64::from_bits(sign | exponent << 52 | high | low));
                    }
                }
            }
        }
        let mut fast = 0;
        for s in values {
            if s == 0.0 || !s.is_finite() {
                continue;
            }
            for error_sign in [-1, 0, 1] {
                if let Some(r) = round_to_single_nearest_fast(s, error_sign) {
                    let exact = round_to_single_exact(s, error_sign, ROUND_NEAREST);
                    assert_eq!(
                        r.to_bits(),
                        exact.to_bits(),
                        "{s:e} ({:016x}) {error_sign}",
                        s.to_bits()
                    );
                    fast += 1;
                }
            }
        }
        assert!(fast > 10_000, "the fast path ran {fast} times");
    }

    /// Under 24-bit precision to nearest (the game's control word after the renderer starts),
    /// `x87_add`/`sub`/`mul`/`div` with their fast path give exactly what rounding the exact
    /// result does (the error from TwoSum or a fused multiply-add, then the exact rounding), on
    /// random float and double operands, products and quotients near midpoints included.
    #[test]
    fn the_operations_fast_path_matches_the_exact_rounding() {
        set_control_word(0x007f);
        let exact = |s: f64, error: f64| {
            if s == 0.0 || !s.is_finite() {
                s
            } else {
                round_to_single_exact(s, sign_of(error), ROUND_NEAREST)
            }
        };
        let mut state = 0x2545_f491_4f6c_dd1du64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for i in 0..400_000 {
            let (a, b) = if i % 2 == 0 {
                (
                    f64::from(f32::from_bits(next() as u32 & 0x7fff_ffff | 0x3000_0000)),
                    f64::from(f32::from_bits(next() as u32 & 0x4fff_ffff)),
                )
            } else {
                let x = f64::from_bits(next() & 0x3fff_ffff_ffff_ffff | 0x3800_0000_0000_0000);
                let y = f64::from_bits(next() & 0x3fff_ffff_ffff_ffff | 0x3800_0000_0000_0000);
                (x, if next() & 1 == 0 { y } else { -y })
            };
            if a.is_nan() || b.is_nan() {
                continue;
            }
            let s = a + b;
            let bb = s - a;
            assert_eq!(
                x87_add(a, b).to_bits(),
                exact(s, (a - (s - bb)) + (b - bb)).to_bits(),
                "{a:e} + {b:e}"
            );
            let p = a * b;
            assert_eq!(
                x87_mul(a, b).to_bits(),
                exact(p, a.mul_add(b, -p)).to_bits(),
                "{a:e} * {b:e}"
            );
            if b != 0.0 {
                let q = a / b;
                let r = (-q).mul_add(b, a);
                let expected = if q == 0.0 || !q.is_finite() {
                    q
                } else {
                    round_to_single_exact(q, sign_of(r) * sign_of(b), ROUND_NEAREST)
                };
                assert_eq!(x87_div(a, b).to_bits(), expected.to_bits(), "{a:e} / {b:e}");
            }
        }
        set_control_word(DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn signalling_nan_gets_the_quiet_bit_and_keeps_its_payload() {
        assert_eq!(x87_quiet_bits(0x7f80_0001), 0x7fc0_0001);
        assert_eq!(x87_quiet_bits(0xff80_1234), 0xffc0_1234);
    }

    #[test]
    fn other_values_are_unchanged() {
        for bits in [
            0,
            0x8000_0000,
            0x3f80_0000,
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_0000,
            0x7fc0_0005,
        ] {
            assert_eq!(x87_quiet_bits(bits), bits);
        }
        assert_eq!(x87_quiet(1.5), 1.5);
    }

    fn f32_result(value: f64) -> u32 {
        (value as f32).to_bits()
    }

    #[test]
    fn two_nans_keep_the_larger_significand_then_the_positive_one() {
        // The vectors of #199: -qNaN and +qNaN with equal significands.
        let (neg, pos) = (x87_m32(0xffff_ffff), x87_m32(0x7fff_ffff));
        assert_eq!(f32_result(x87_mul(neg, pos)), 0x7fff_ffff);
        assert_eq!(f32_result(x87_add(neg, pos)), 0x7fff_ffff);
        assert_eq!(f32_result(x87_add(pos, neg)), 0x7fff_ffff);
        // Key 0x7fc00000 times scale 0xffffffff: the larger significand wins.
        let small = x87_m32(0x7fc0_0000);
        assert_eq!(f32_result(x87_mul(small, neg)), 0xffff_ffff);
        assert_eq!(f32_result(x87_mul(neg, small)), 0xffff_ffff);
        // x and |x| for x = 0xffc00000.
        let x = x87_m32(0xffc0_0000);
        assert_eq!(f32_result(x87_mul(x, x.abs())), 0x7fc0_0000);
        assert_eq!(f32_result(x87_sub(x.abs(), x)), 0x7fc0_0000);
        assert_eq!(f32_result(x87_div(x, x.abs())), 0x7fc0_0000);
    }

    #[test]
    fn a_quiet_nan_wins_over_a_signalling_one() {
        let signalling = x87_m32(0x7fbf_ffff);
        let quiet = x87_m32(0xffc0_0001);
        assert_eq!(f32_result(x87_mul(signalling, quiet)), 0xffc0_0001);
        assert_eq!(f32_result(x87_mul(quiet, signalling)), 0xffc0_0001);
        // Two signalling NaNs: the larger significand, quieted.
        let other = x87_m32(0xff80_0001);
        assert_eq!(f32_result(x87_add(signalling, other)), 0x7fff_ffff);
    }

    #[test]
    fn one_nan_comes_back_quieted_and_numbers_are_unchanged() {
        assert_eq!(f32_result(x87_mul(x87_m32(0xff80_0005), 2.0)), 0xffc0_0005);
        assert_eq!(f32_result(x87_sub(3.0, x87_m32(0x7fc0_1234))), 0x7fc0_1234);
        assert_eq!(x87_add(1.5, 2.25), 3.75);
        assert_eq!(x87_sub(1.5, 2.25), -0.75);
        assert_eq!(x87_mul(1.5, -2.0), -3.0);
        assert_eq!(x87_div(1.0, 4.0), 0.25);
        assert_eq!(x87_m32(0x3fc0_0000), 1.5);
        assert_eq!(x87_m32(0x7f80_0000), f64::INFINITY);
    }

    /// Runs `f` with the control word `word` loaded, then the default again.
    fn with_word<R>(word: u16, f: impl FnOnce() -> R) -> R {
        set_control_word(word);
        let r = f();
        set_control_word(DEFAULT_CONTROL_WORD);
        r
    }

    #[test]
    fn at_53_bit_precision_the_helpers_are_f64_arithmetic() {
        assert_eq!(control_word(), 0x027f);
        let mut x = 0x1234_5678_9abc_def1u64;
        let mut next = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            f64::from_bits(x)
        };
        for word in [0x027f, 0x037f, 0x0e7f] {
            with_word(word, || {
                for _ in 0..2000 {
                    let (a, b) = (next(), next());
                    if a.is_nan() || b.is_nan() {
                        continue;
                    }
                    assert_eq!(x87_add(a, b).to_bits(), (a + b).to_bits());
                    assert_eq!(x87_sub(a, b).to_bits(), (a - b).to_bits());
                    assert_eq!(x87_mul(a, b).to_bits(), (a * b).to_bits());
                    assert_eq!(x87_div(a, b).to_bits(), (a / b).to_bits());
                    assert_eq!(x87_round(a).to_bits(), a.to_bits());
                }
            });
        }
    }

    #[test]
    fn at_24_bit_precision_results_have_24_bit_significands_and_the_extended_range() {
        with_word(0x007f, || {
            assert_eq!(x87_div(1.0, 3.0), f64::from(1.0f32 / 3.0));
            assert_eq!(x87_add(1.0, 2f64.powi(-30)), 1.0);
            assert_eq!(x87_round(1.0 + 2f64.powi(-30)), 1.0);
            // 1e30 * 1e30 stays finite, 1e-30 * 1e-30 stays normal.
            let big = x87_mul(1e30, 1e30);
            assert!(big.is_finite() && big != 1e60);
            assert_eq!(big.to_bits() & ((1 << 29) - 1), 0, "24-bit significand");
            let small = x87_mul(1e-30, 1e-30);
            assert!(small > 0.0 && small.to_bits() & ((1 << 29) - 1) == 0);
            assert_eq!(x87_sqrt(2.0), f64::from(2f32.sqrt()));
        });
    }

    /// Results recorded on the CPU (AMD Ryzen AI 9 HX 370) with nv-call and the
    /// one-instruction stubs of `nv-pure x87`: (control word, instruction,
    /// first operand, second operand, ST0 rounded to a double, which is exact
    /// for all of these). Instructions: `add`/`sub`/`mul`/`div` are `FLD qword
    /// a; F<op> qword b`; `addm` is `FLD qword a; FADD dword b`; `adds`/`muls`
    /// are `FLD dword a; F<op> dword b` (operands as float bits); `sqrt` is
    /// `FLD qword a; FSQRT`; `round` is `FLD qword a; FLD1; FMULP`.
    const CPU: &[(u16, &str, u64, u64, u64)] = &[
        // Ties at 24 bits go to even; a double sum that is exactly a tie but
        // whose exact value is not (2^-70 below the f64's last bit) is decided
        // by that sticky part, in either direction and across a binade.
        (
            0x007f,
            "add",
            0x3ff0000000000000,
            0x3e70000000000000,
            0x3ff0000000000000,
        ),
        (
            0x007f,
            "add",
            0x3ff0000020000000,
            0x3e70000000000000,
            0x3ff0000040000000,
        ),
        (
            0x007f,
            "add",
            0x3ff0000000000000,
            0x3e70000000000040,
            0x3ff0000020000000,
        ),
        (
            0x007f,
            "add",
            0x3ff0000000000000,
            0xbe70000000000040,
            0x3fefffffe0000000,
        ),
        (
            0x007f,
            "sub",
            0x3ff0000000000000,
            0x3c70000000000000,
            0x3ff0000000000000,
        ),
        // Beyond the float range: no overflow, no float denormals.
        (
            0x007f,
            "mul",
            0x4629b7f0e94f6c0b,
            0x4629b7f0e94f6c0b,
            0x4c64ab89c0000000,
        ),
        (0x007f, "muls", 0x7149f2ca, 0x7149f2ca, 0x4c63e9e4e0000000),
        (0x007f, "muls", 0x7f7fffff, 0x40000000, 0x47ffffffe0000000),
        (0x007f, "muls", 0x00000001, 0x00000001, 0x2d50000000000000),
        // Float denormals as operands: full 24-bit results.
        (0x007f, "muls", 0x00000003, 0x3eaaaaab, 0x36a0000000000000),
        (0x007f, "muls", 0x00000001, 0x3f000000, 0x3690000000000000),
        (
            0x007f,
            "mul",
            0x36a0000000000000,
            0x3fd5555555555555,
            0x3685555560000000,
        ),
        // NaNs and infinities are not affected by precision control.
        (
            0x007f,
            "addm",
            0x3ff0000000000000,
            0x7f800001,
            0x7ff8000020000000,
        ),
        (0x007f, "adds", 0x7f800001, 0x3f800000, 0x7ff8000020000000),
        (
            0x007f,
            "add",
            0x7ff0000000000000,
            0xfff0000000000000,
            0xfff8000000000000,
        ),
        (
            0x007f,
            "div",
            0x0000000000000000,
            0x0000000000000000,
            0xfff8000000000000,
        ),
        (
            0x007f,
            "div",
            0x3ff0000000000000,
            0x0000000000000000,
            0x7ff0000000000000,
        ),
        (
            0x007f,
            "div",
            0xbff0000000000000,
            0x0000000000000000,
            0xfff0000000000000,
        ),
        (
            0x007f,
            "mul",
            0x7ff0000000000000,
            0x4000000000000000,
            0x7ff0000000000000,
        ),
        (
            0x007f,
            "add",
            0xfff0000000000000,
            0x4629b7f0e94f6c0b,
            0xfff0000000000000,
        ),
        (
            0x007f,
            "mul",
            0x7ffc000012345678,
            0x4000000000000000,
            0x7ffc000012345678,
        ),
        // Double operands, rounded once from the exact value.
        (
            0x007f,
            "add",
            0x3fb999999999999a,
            0x3fc999999999999a,
            0x3fd3333340000000,
        ),
        (
            0x007f,
            "mul",
            0x3fd5555555555555,
            0x4008000000000000,
            0x3ff0000000000000,
        ),
        (
            0x007f,
            "div",
            0x3ff0000000000000,
            0x4008000000000000,
            0x3fd5555560000000,
        ),
        // The rounding modes at 24 bits.
        (
            0x047f,
            "div",
            0x3ff0000000000000,
            0x4008000000000000,
            0x3fd5555540000000,
        ),
        (
            0x087f,
            "div",
            0x3ff0000000000000,
            0x4008000000000000,
            0x3fd5555560000000,
        ),
        (
            0x0c7f,
            "div",
            0xbff0000000000000,
            0x4008000000000000,
            0xbfd5555540000000,
        ),
        (
            0x047f,
            "add",
            0x3ff0000000000000,
            0x3e10000000000000,
            0x3ff0000000000000,
        ),
        (
            0x087f,
            "add",
            0x3ff0000000000000,
            0x3e10000000000000,
            0x3ff0000020000000,
        ),
        (
            0x047f,
            "add",
            0x3ff0000000000000,
            0xbe10000000000000,
            0x3fefffffe0000000,
        ),
        (
            0x0c7f,
            "add",
            0xbff0000000000000,
            0xbe10000000000000,
            0xbff0000000000000,
        ),
        // An exact zero sum is -0 only when rounding down.
        (
            0x047f,
            "sub",
            0x3ff0000000000000,
            0x3ff0000000000000,
            0x8000000000000000,
        ),
        (
            0x047f,
            "add",
            0x0000000000000000,
            0x8000000000000000,
            0x8000000000000000,
        ),
        (
            0x007f,
            "sub",
            0x3ff0000000000000,
            0x3ff0000000000000,
            0x0000000000000000,
        ),
        (0x007f, "sqrt", 0x4000000000000000, 0, 0x3ff6a09e60000000),
        (0x047f, "sqrt", 0x4000000000000000, 0, 0x3ff6a09e60000000),
        (0x087f, "sqrt", 0x4000000000000000, 0, 0x3ff6a09e80000000),
        (0x007f, "sqrt", 0xbff0000000000000, 0, 0xfff8000000000000),
        (0x007f, "sqrt", 0x8000000000000000, 0, 0x8000000000000000),
        (0x007f, "sqrt", 0x7ff0000000000001, 0, 0x7ff8000000000001),
        (0x007f, "round", 0x3ff0000010000000, 0, 0x3ff0000000000000),
        (0x007f, "round", 0x4c169a3dbe2a8d31, 0, 0x4c169a3dc0000000),
        (0x007f, "round", 0x3fb999999999999a, 0, 0x3fb99999a0000000),
        // 53-bit precision: f64 arithmetic.
        (
            0x027f,
            "add",
            0x3ff0000000000000,
            0x3e10000000000000,
            0x3ff0000000400000,
        ),
        (0x027f, "muls", 0x00000003, 0x3eaaaaab, 0x36a0000008000000),
        (
            0x027f,
            "mul",
            0x4629b7f0e94f6c0b,
            0x4629b7f0e94f6c0b,
            0x4c64ab89bf0e4781,
        ),
        // Found by the random vectors: the quotient's residual underflows
        // the f64 unless it is computed on scaled operands.
        (
            0x087f,
            "div",
            0x0010000000000000,
            0x3fd5555555555555,
            0x0028000020000000,
        ),
    ];

    #[test]
    fn rounding_matches_the_cpu() {
        for &(word, op, a, b, want) in CPU {
            let f = f64::from_bits;
            let got = with_word(word, || match op {
                "add" => x87_add(x87_load64(a), f(b)),
                "sub" => x87_sub(x87_load64(a), f(b)),
                "mul" => x87_mul(x87_load64(a), f(b)),
                "div" => x87_div(x87_load64(a), f(b)),
                "addm" => x87_add(x87_load64(a), x87_m32(b as u32)),
                "adds" => x87_add(x87_load32(a as u32), x87_m32(b as u32)),
                "muls" => x87_mul(x87_load32(a as u32), x87_m32(b as u32)),
                "sqrt" => x87_sqrt(x87_load64(a)),
                "round" => x87_round(x87_load64(a)),
                _ => unreachable!(),
            });
            assert_eq!(
                got.to_bits(),
                want,
                "{word:#06x} {op} {a:#x} {b:#x}: {got:?} != {:?}",
                f(want)
            );
        }
    }

    #[test]
    fn a_float_store_after_24_bit_arithmetic_rounds_to_the_float_denormal() {
        // 3 * 2^-149 times 1/3 is 2^-149 * (1 + 2^-25) exactly: 2^-149 at
        // 24 bits, and that float denormal on the store.
        with_word(0x007f, || {
            let p = x87_mul(x87_load32(3), x87_m32(0x3eaa_aaab));
            assert_eq!(x87_store32(p), 1);
            // 1e30f * 1e30f stays finite at 24 bits; the float store
            // overflows to infinity.
            let big = x87_mul(x87_load32(0x7149_f2ca), x87_m32(0x7149_f2ca));
            assert!(big.is_finite());
            assert_eq!(x87_store32(big), 0x7f80_0000);
        });
    }

    #[test]
    fn the_control_word_belongs_to_the_host_thread() {
        with_word(0x007f, || {
            let other = std::thread::spawn(control_word).join().unwrap();
            assert_eq!(other, DEFAULT_CONTROL_WORD);
            assert_eq!(control_word(), 0x007f);
        });
    }

    #[test]
    fn load32_quiets_and_store32_rounds_to_a_float() {
        assert_eq!(x87_load32(0x3fc0_0000), 1.5);
        assert_eq!(x87_store32(x87_load32(0x7f80_0001)), 0x7fc0_0001);
        assert_eq!(x87_store32(x87_load32(0xffc0_1234)), 0xffc0_1234);
        assert_eq!(x87_store32(x87_load32(0x0000_0001)), 0x0000_0001);
        // 1 + 2^-30 rounds to 1.0 in a float.
        assert_eq!(x87_store32(1.0 + 2f64.powi(-30)), 0x3f80_0000);
        assert_eq!(
            x87_store32(x87_add(x87_m32(0xffff_ffff), x87_m32(0x7fff_ffff))),
            0x7fff_ffff
        );
    }

    // ---- Transcendental helpers ----

    fn ext(v: f64) -> X87Ext {
        X87Ext::load64(v)
    }

    #[test]
    fn loads_widen_exactly_and_stores_give_the_value_back() {
        assert_eq!(ext(1.0), X87Ext::from_parts(0x3fff, 1 << 63));
        assert_eq!(ext(-2.0), X87Ext::from_parts(0xc000, 1 << 63));
        assert_eq!(
            X87Ext::load32(0x3fc0_0000),
            X87Ext::from_parts(0x3fff, 3 << 62)
        );
        // FLD quiets a signalling NaN and keeps its payload.
        let quieted = X87Ext::load32(0x7f80_0001);
        assert_eq!(quieted.store32(), 0x7fc0_0001);
        assert_eq!(
            X87Ext::load64(f64::from_bits(0x7ff0_0000_0000_0001))
                .store64()
                .to_bits(),
            0x7ff8_0000_0000_0001
        );
        for v in [1.5, -0.0, 5e-324, f64::INFINITY, f64::MAX, 0.1] {
            assert_eq!(ext(v).store64().to_bits(), v.to_bits());
        }
    }

    #[test]
    fn the_software_conversions_match_the_fpu() {
        for bits in [
            0x3ff8_0000_0000_0000u64,
            0x8000_0000_0000_0000,
            0x0000_0000_0000_0001,
            0x000f_ffff_ffff_ffff,
            0x7ff0_0000_0000_0000,
            0x7ff0_0000_0000_0001,
            0xfff8_0000_0000_1234,
            0x3fb9_9999_9999_999a,
        ] {
            let v = f64::from_bits(bits);
            assert_eq!(soft::load64(v), X87Ext::load64(v), "{bits:#x}");
            assert_eq!(
                soft::store64(X87Ext::load64(v)).to_bits(),
                bits | if v.is_nan() { 1 << 51 } else { 0 }
            );
        }
        for bits in [0x3fc0_0000u32, 0x7f80_0001, 0x0000_0001, 0xff80_0000] {
            assert_eq!(soft::load32(bits), X87Ext::load32(bits), "{bits:#x}");
        }
        // An extended value between two doubles rounds to nearest, ties to even.
        let between = X87Ext::from_parts(0x3fff, (1 << 63) | (1 << 10) | 1);
        assert_eq!(soft::store64(between), between.store64());
        let tie = X87Ext::from_parts(0x3fff, (1 << 63) | (1 << 10));
        assert_eq!(soft::store64(tie), tie.store64());
    }

    #[test]
    fn transcendental_results_are_the_fpus() {
        assert_eq!(x87_fsin(ext(0.0)).store64(), 0.0);
        assert_eq!(x87_fcos(ext(0.0)).store64(), 1.0);
        assert_eq!(x87_fptan(ext(0.0)).store64(), 0.0);
        assert_eq!(
            x87_fpatan(ext(1.0), ext(1.0)).store64(),
            std::f64::consts::FRAC_PI_4
        );
        assert_eq!(
            x87_fpatan(ext(1.0), ext(-1.0)).store64(),
            3.0 * std::f64::consts::FRAC_PI_4
        );
        assert_eq!(x87_fyl2x(ext(1.0), ext(8.0)).store64(), 3.0);
        assert_eq!(x87_fyl2x(ext(-2.0), ext(0.25)).store64(), 4.0);
        assert_eq!(x87_fyl2xp1(ext(1.0), ext(0.0)).store64(), 0.0);
        assert_eq!(x87_f2xm1(ext(0.0)).store64(), 0.0);
        assert_eq!(x87_f2xm1(ext(-1.0)).store64(), -0.5);
        assert_eq!(x87_fscale(ext(1.5), ext(3.7)).store64(), 12.0);
        assert_eq!(x87_fscale(ext(1.5), ext(-1.9)).store64(), 0.75);
        let (sin, cos) = x87_fsincos(ext(0.5));
        assert_eq!(sin, x87_fsin(ext(0.5)));
        assert_eq!(cos, x87_fcos(ext(0.5)));
        // FSIN reduces with the FPU's own pi: sin(pi as a double) is the
        // well-known x87 result, not the correctly rounded 1.2246467991473532e-16.
        assert_eq!(
            x87_fsin(ext(std::f64::consts::PI)).store64(),
            1.2246063538223773e-16
        );
        // An operand out of range comes back unchanged.
        let huge = ext(2f64.powi(64));
        assert_eq!(x87_fsin(huge), huge);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn fsincos_out_of_range_stops() {
        x87_fsincos(ext(2f64.powi(64)));
    }

    #[test]
    fn remainders_report_the_status_word() {
        let (r, status) = x87_fprem(ext(7.0), ext(3.0));
        assert_eq!(r.store64(), 1.0);
        assert_eq!(status & X87_STATUS_C2, 0);
        // Quotient 2: C1 (bit 9) holds its bit 0, C3 (bit 14) bit 1, C0 (bit 8) bit 2.
        assert_eq!(status & 0x4300, 0x4000);
        let (r, _) = x87_fprem1(ext(8.0), ext(3.0));
        assert_eq!(r.store64(), -1.0);
        let (r, _) = x87_fprem(ext(-7.5), ext(2.0));
        assert_eq!(r.store64(), -1.5);
        // Exponents 2^70 apart: one FPREM reduces only part of the way.
        let (_, status) = x87_fprem(ext(2f64.powi(80)), ext(3.0));
        assert_ne!(status & X87_STATUS_C2, 0);
    }

    #[test]
    fn the_control_word_decides_rounding_but_not_precision() {
        // FRNDINT and the stores follow the rounding mode.
        let rounded = |word: u16, v: f64| with_word(word, || x87_frndint(ext(v)).store64());
        assert_eq!(rounded(0x027f, 2.5), 2.0);
        assert_eq!(rounded(0x027f, 3.5), 4.0);
        assert_eq!(rounded(0x067f, -2.5), -3.0);
        assert_eq!(rounded(0x0a7f, 2.25), 3.0);
        assert_eq!(rounded(0x0e7f, -2.75), -2.0);
        let near_one = ext(1.0 + 2f64.powi(-30));
        assert_eq!(with_word(0x027f, || near_one.store32()), 0x3f80_0000);
        assert_eq!(with_word(0x0a7f, || near_one.store32()), 0x3f80_0001);
        // 24-bit precision control: stores and transcendentals keep their
        // precision (only FADD/FSUB/FMUL/FDIV/FSQRT round to 24 bits).
        assert_eq!(
            with_word(0x007f, || near_one.store64()),
            1.0 + 2f64.powi(-30)
        );
        let at53 = with_word(0x027f, || x87_fsin(ext(1.0)));
        let at24 = with_word(0x007f, || x87_fsin(ext(1.0)));
        assert_eq!(at24, at53);
        assert_ne!(at24.significand() & 0xff_ffff_ffff, 0, "64-bit result");
        // The helpers leave the thread's control word as it was.
        with_word(0x007f, || {
            x87_fcos(ext(1.0));
            assert_eq!(control_word(), 0x007f);
        });
    }

    #[test]
    fn fistp_chop_truncates_toward_zero() {
        assert_eq!(fistp_chop(2.9), 2);
        assert_eq!(fistp_chop(-2.9), -2);
        assert_eq!(fistp_chop(-0.5), 0);
        assert_eq!(fistp_chop(2147483647.9), i32::MAX);
        assert_eq!(fistp_chop(2147483648.0), i32::MIN);
        assert_eq!(fistp_chop(-2147483648.9), i32::MIN);
        assert_eq!(fistp_chop(-2147483649.0), i32::MIN);
        assert_eq!(fistp_chop(f64::NAN), i32::MIN);
    }

    #[test]
    fn fistp64_rounds_with_the_control_word_and_chop_truncates() {
        let at = |word: u16, v: f64| with_word(word, || x87_fistp64(ext(v)));
        assert_eq!(at(0x027f, 2.5), 2);
        assert_eq!(at(0x027f, 3.5), 4);
        assert_eq!(at(0x067f, -2.5), -3);
        assert_eq!(at(0x0a7f, 2.25), 3);
        assert_eq!(at(0x0e7f, -2.75), -2);
        assert_eq!(with_word(0x027f, || x87_fistp64_chop(ext(-2.75))), -2);
        assert_eq!(
            with_word(0x007f, || x87_fistp64_chop(ext(1e15 + 0.9))),
            1_000_000_000_000_000
        );
        assert_eq!(x87_fistp64_chop(ext(-(2f64.powi(63)))), i64::MIN);
        assert_eq!(x87_fistp64_chop(ext(2f64.powi(63))), i64::MIN);
        assert_eq!(x87_fistp64(ext(f64::NAN)), i64::MIN);
        assert_eq!(x87_fistp64(ext(f64::INFINITY)), i64::MIN);
        // The thread's control word stays.
        assert_eq!(
            with_word(0x007f, || {
                x87_fistp64_chop(ext(1.5));
                control_word()
            }),
            0x007f
        );
        // The software fallback agrees on these.
        for (word, v) in [
            (0x027fu16, 2.5f64),
            (0x0e7f, -2.75),
            (0x067f, -2.5),
            (0x027f, 1e19),
            (0x027f, f64::NAN),
        ] {
            assert_eq!(
                soft::fistp64(ext(v), word),
                hw_fistp64(ext(v), word),
                "{word:#x} {v}"
            );
        }
    }

    fn hw_fistp64(x: X87Ext, word: u16) -> i64 {
        with_word(word, || x87_fistp64(x))
    }

    #[test]
    fn register_arithmetic_rounds_once_to_the_control_words_precision() {
        // 53-bit precision: the double result, kept in a register.
        let third = with_word(0x027f, || x87_fdiv(ext(1.0), ext(3.0)));
        assert_eq!(third.store64(), 1.0 / 3.0);
        // 64-bit precision keeps the whole extended significand, 24-bit rounds it to 24 bits.
        let third = with_word(0x037f, || x87_fdiv(ext(1.0), ext(3.0)));
        assert_eq!(
            (third.sign_exponent(), third.significand()),
            (0x3ffd, 0xaaaa_aaaa_aaaa_aaab)
        );
        let third = with_word(0x007f, || x87_fdiv(ext(1.0), ext(3.0)));
        assert_eq!(
            (third.sign_exponent(), third.significand()),
            (0x3ffd, 0xaaaa_ab00_0000_0000)
        );
        // Round toward zero at 24 bits.
        let third = with_word(0x0c7f, || x87_fdiv(ext(1.0), ext(3.0)));
        assert_eq!(third.significand(), 0xaaaa_aa00_0000_0000);
        // The operand order: ST0 first.
        let on = |f: fn(X87Ext, X87Ext) -> X87Ext, a: f64, b: f64| {
            with_word(0x027f, || f(ext(a), ext(b))).store64()
        };
        assert_eq!(on(x87_fsub, 5.0, 2.0), 3.0);
        assert_eq!(on(x87_fdiv, 6.0, 2.0), 3.0);
        assert_eq!(on(x87_fadd, 5.0, 2.0), 7.0);
        assert_eq!(on(x87_fmul, 5.0, 2.0), 10.0);
        assert_eq!(with_word(0x027f, || x87_fsqrt(ext(9.0))).store64(), 3.0);
        // The extended exponent range: no overflow in between.
        let back = with_word(0x027f, || {
            x87_fdiv(x87_fmul(ext(1e300), ext(1e300)), ext(1e300))
        });
        assert_eq!(back.store64(), 1e300);
        // The x87 NaN choice: the larger significand of two quiet NaNs.
        let nan = with_word(0x027f, || {
            x87_fadd(
                ext(f64::from_bits(0x7ff8_0000_0000_0001)),
                ext(f64::from_bits(0xfff8_0000_0000_0002)),
            )
        });
        assert_eq!(nan.store64().to_bits(), 0xfff8_0000_0000_0002);
        // The fallback agrees on exact cases.
        assert_eq!(soft::fsub(ext(5.0), ext(2.0)).store64(), 3.0);
        assert_eq!(soft::fdiv(ext(6.0), ext(2.0)).store64(), 3.0);
    }

    #[test]
    fn fcom_sets_c0_c2_and_c3() {
        let c = |a: f64, b: f64| x87_fcom(ext(a), ext(b));
        assert_eq!(c(2.0, 1.0), 0);
        assert_eq!(c(1.0, 2.0), X87_STATUS_C0);
        assert_eq!(c(-0.0, 0.0), X87_STATUS_C3);
        assert_eq!(
            c(f64::NAN, 1.0),
            X87_STATUS_C0 | X87_STATUS_C2 | X87_STATUS_C3
        );
        assert_eq!(soft::fcom(ext(1.0), ext(2.0)), X87_STATUS_C0);
    }

    #[test]
    fn fxam_classifies_the_register() {
        let c = |bits: u64| x87_fxam(ext(f64::from_bits(bits)));
        let (c0, c1, c2, c3) = (X87_STATUS_C0, 0x0200, X87_STATUS_C2, X87_STATUS_C3);
        assert_eq!(c(1.5f64.to_bits()), c2);
        assert_eq!(c((-1.5f64).to_bits()), c1 | c2);
        assert_eq!(c(0), c3);
        assert_eq!(c(0x8000_0000_0000_0000), c1 | c3);
        assert_eq!(c(f64::INFINITY.to_bits()), c2 | c0);
        assert_eq!(c(0xfff8_0000_0000_0000), c1 | c0);
        // A double denormal is a normal extended value.
        assert_eq!(c(1), c2);
        assert_eq!(x87_fxam(X87Ext::from_parts(0, 5)), c3 | c2);
        assert_eq!(x87_fxam(X87Ext::from_parts(0x3fff, 5)), 0);
        assert_eq!(x87_fxam(X87Ext::from_parts(0x7fff, 0)), 0);
    }

    #[test]
    fn fchs_fabs_and_the_loaded_constants() {
        assert_eq!(x87_fchs(ext(1.5)).store64(), -1.5);
        assert_eq!(x87_fabs(ext(-2.0)).store64(), 2.0);
        let nan = x87_fchs(ext(f64::from_bits(0x7ff8_0000_0000_0005)));
        assert_eq!(nan.store64().to_bits(), 0xfff8_0000_0000_0005);
        let parts = |word: u16, c: X87Constant| {
            let v = with_word(word, || x87_fld_constant(c));
            (v.sign_exponent(), v.significand())
        };
        assert_eq!(
            parts(0x027f, X87Constant::Pi),
            (0x4000, 0xc90f_daa2_2168_c235)
        );
        assert_eq!(
            parts(0x0e7f, X87Constant::Pi),
            (0x4000, 0xc90f_daa2_2168_c234)
        );
        assert_eq!(
            parts(0x027f, X87Constant::Log2E),
            (0x3fff, 0xb8aa_3b29_5c17_f0bc)
        );
        assert_eq!(
            parts(0x027f, X87Constant::Log2Ten),
            (0x4000, 0xd49a_784b_cd1b_8afe)
        );
        assert_eq!(
            parts(0x027f, X87Constant::Log10Two),
            (0x3ffd, 0x9a20_9a84_fbcf_f799)
        );
        assert_eq!(
            parts(0x027f, X87Constant::LnTwo),
            (0x3ffe, 0xb172_17f7_d1cf_79ac)
        );
    }
}

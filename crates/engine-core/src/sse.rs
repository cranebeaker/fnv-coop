//! SSE floating-point arithmetic, bit-exact with the instructions the exe runs.
//!
//! Host float arithmetic gives the IEEE result for numbers but not the SSE
//! choice of NaN, and LLVM may swap the operands of a commutative `+`/`*`. The
//! functions here keep the rules of the SSE instructions (Intel SDM vol. 1,
//! table 4-7, SSE column; vol. 2 for `MINPS`/`MAXPS`):
//!
//! - a NaN in the destination operand wins, quieted (bit 22 set, sign and
//!   payload kept); else a NaN in the source operand, quieted;
//! - an invalid operation on two numbers (`inf - inf`, `0 * inf`, `0 / 0`,
//!   `inf / inf`, the root of a negative number) gives the default NaN
//!   `0xffc00000`;
//! - `MINPS`/`MAXPS` (and the scalar forms) answer the source operand,
//!   unchanged (a signalling NaN stays signalling), when either operand is a
//!   NaN or both are zero.
//!
//! Every function takes the destination operand first, then the source, as
//! the instruction does (`ADDPS dest, src` is `addps(dest, src)`). Rounding is
//! to nearest and denormals are neither flushed nor treated as zero: the
//! MXCSR is 0x1F80 on every game thread (measured). The reciprocal estimates
//! (`RCPPS`, `RSQRTSS`) are hardware estimates, kept in
//! [`crate::x87::rsqrt_estimate`] and the units that use them.

/// An SSE register: four `f32` lanes (moves keep the bits).
pub type Xmm = [f32; 4];

/// The quiet bit of a single-precision NaN.
const QUIET: u32 = 0x0040_0000;
/// The default NaN SSE answers for an invalid operation.
const DEFAULT_NAN: u32 = 0xffc0_0000;

/// `value` with its quiet bit set (a NaN stays the same NaN, quiet).
fn quiet(value: f32) -> f32 {
    f32::from_bits(value.to_bits() | QUIET)
}

/// The SSE result of a two-operand arithmetic instruction whose numeric
/// result (both operands numbers) is `op(dest, src)`.
fn arith(dest: f32, src: f32, op: fn(f32, f32) -> f32) -> f32 {
    if dest.is_nan() {
        quiet(dest)
    } else if src.is_nan() {
        quiet(src)
    } else {
        let result = op(dest, src);
        if result.is_nan() {
            f32::from_bits(DEFAULT_NAN)
        } else {
            result
        }
    }
}

/// `ADDSS` on one lane: `dest + src` with the SSE NaN rules.
pub fn add(dest: f32, src: f32) -> f32 {
    arith(dest, src, |a, b| a + b)
}

/// `SUBSS` on one lane: `dest - src` with the SSE NaN rules.
pub fn sub(dest: f32, src: f32) -> f32 {
    arith(dest, src, |a, b| a - b)
}

/// `MULSS` on one lane: `dest * src` with the SSE NaN rules.
pub fn mul(dest: f32, src: f32) -> f32 {
    arith(dest, src, |a, b| a * b)
}

/// `DIVSS` on one lane: `dest / src` with the SSE NaN rules.
pub fn div(dest: f32, src: f32) -> f32 {
    arith(dest, src, |a, b| a / b)
}

/// `SQRTSS` on one lane: the square root of `src`; a NaN comes back quieted,
/// a negative number other than `-0` gives the default NaN, `-0` gives `-0`.
pub fn sqrt(src: f32) -> f32 {
    if src.is_nan() {
        quiet(src)
    } else if src < 0.0 {
        f32::from_bits(DEFAULT_NAN)
    } else {
        src.sqrt()
    }
}

/// `MINSS` on one lane: `dest < src ? dest : src`, so `src` (unchanged) when
/// either is a NaN or both are zero.
pub fn min(dest: f32, src: f32) -> f32 {
    if dest < src {
        dest
    } else {
        src
    }
}

/// `MAXSS` on one lane: `dest > src ? dest : src`, so `src` (unchanged) when
/// either is a NaN or both are zero.
pub fn max(dest: f32, src: f32) -> f32 {
    if dest > src {
        dest
    } else {
        src
    }
}

/// The predicate of `CMPSS`/`CMPPS dest, src, imm` (the low three bits of
/// `imm`; the SSE encoding has no others): 0 `EQ`, 1 `LT`, 2 `LE`, 3 `UNORD`
/// (either a NaN), 4 `NEQ`, 5 `NLT`, 6 `NLE`, 7 `ORD` (neither a NaN). `EQ`,
/// `LT`, `LE` are false and `NEQ`, `NLT`, `NLE` true when either is a NaN.
pub fn compare(dest: f32, src: f32, imm: u8) -> bool {
    match imm & 7 {
        0 => dest == src,
        1 => dest < src,
        2 => dest <= src,
        3 => dest.is_nan() || src.is_nan(),
        4 => dest != src,
        5 => !(dest < src),
        6 => !(dest <= src),
        _ => !dest.is_nan() && !src.is_nan(),
    }
}

/// The lane mask a compare writes: all ones when `holds`, else zero.
fn mask(holds: bool) -> f32 {
    f32::from_bits(if holds { u32::MAX } else { 0 })
}

/// `op` on lane 0, the other lanes of `dest` kept (the `..SS` forms).
fn scalar(dest: Xmm, src: Xmm, op: fn(f32, f32) -> f32) -> Xmm {
    [op(dest[0], src[0]), dest[1], dest[2], dest[3]]
}

/// `op` on each lane (the `..PS` forms).
fn packed(dest: Xmm, src: Xmm, op: fn(f32, f32) -> f32) -> Xmm {
    std::array::from_fn(|lane| op(dest[lane], src[lane]))
}

/// `ADDPS dest, src`.
pub fn addps(dest: Xmm, src: Xmm) -> Xmm {
    packed(dest, src, add)
}

/// `SUBPS dest, src`.
pub fn subps(dest: Xmm, src: Xmm) -> Xmm {
    packed(dest, src, sub)
}

/// `MULPS dest, src`.
pub fn mulps(dest: Xmm, src: Xmm) -> Xmm {
    packed(dest, src, mul)
}

/// `DIVPS dest, src`.
pub fn divps(dest: Xmm, src: Xmm) -> Xmm {
    packed(dest, src, div)
}

/// `SQRTPS dest, src`: the root of each lane of `src` (the old `dest` is not read).
pub fn sqrtps(src: Xmm) -> Xmm {
    src.map(sqrt)
}

/// `MINPS dest, src`.
pub fn minps(dest: Xmm, src: Xmm) -> Xmm {
    packed(dest, src, min)
}

/// `MAXPS dest, src`.
pub fn maxps(dest: Xmm, src: Xmm) -> Xmm {
    packed(dest, src, max)
}

/// `CMPPS dest, src, imm`: per lane all ones where [`compare`] holds, else zero.
pub fn cmpps(dest: Xmm, src: Xmm, imm: u8) -> Xmm {
    std::array::from_fn(|lane| mask(compare(dest[lane], src[lane], imm)))
}

/// `ADDSS dest, src`: lane 0 summed, the other lanes of `dest`.
pub fn addss(dest: Xmm, src: Xmm) -> Xmm {
    scalar(dest, src, add)
}

/// `SUBSS dest, src`: lane 0 subtracted, the other lanes of `dest`.
pub fn subss(dest: Xmm, src: Xmm) -> Xmm {
    scalar(dest, src, sub)
}

/// `MULSS dest, src`: lane 0 multiplied, the other lanes of `dest`.
pub fn mulss(dest: Xmm, src: Xmm) -> Xmm {
    scalar(dest, src, mul)
}

/// `DIVSS dest, src`: lane 0 divided, the other lanes of `dest`.
pub fn divss(dest: Xmm, src: Xmm) -> Xmm {
    scalar(dest, src, div)
}

/// `SQRTSS dest, src`: the root of lane 0 of `src`, the other lanes of `dest`.
pub fn sqrtss(dest: Xmm, src: Xmm) -> Xmm {
    [sqrt(src[0]), dest[1], dest[2], dest[3]]
}

/// `MINSS dest, src`: lane 0 by [`min`], the other lanes of `dest`.
pub fn minss(dest: Xmm, src: Xmm) -> Xmm {
    scalar(dest, src, min)
}

/// `MAXSS dest, src`: lane 0 by [`max`], the other lanes of `dest`.
pub fn maxss(dest: Xmm, src: Xmm) -> Xmm {
    scalar(dest, src, max)
}

/// `CMPSS dest, src, imm`: the mask of lane 0, the other lanes of `dest`.
pub fn cmpss(dest: Xmm, src: Xmm, imm: u8) -> Xmm {
    [
        mask(compare(dest[0], src[0], imm)),
        dest[1],
        dest[2],
        dest[3],
    ]
}

#[cfg(all(test, target_arch = "x86_64"))]
mod tests {
    //! Each operation against the host CPU's own instruction, run through
    //! `asm!` (so nothing is constant-folded), compared bit for bit. The
    //! default MXCSR of a Rust thread is 0x1F80, as on the game's threads.

    use super::*;
    use std::arch::asm;
    use std::arch::x86_64::__m128;

    type Bits = [u32; 4];
    type Binary = fn(Xmm, Xmm) -> Xmm;

    fn to_reg(lanes: Bits) -> __m128 {
        // SAFETY: both are 16 bytes of plain data.
        unsafe { std::mem::transmute::<Bits, __m128>(lanes) }
    }

    fn from_reg(reg: __m128) -> Bits {
        // SAFETY: both are 16 bytes of plain data.
        unsafe { std::mem::transmute::<__m128, Bits>(reg) }
    }

    fn xmm(bits: Bits) -> Xmm {
        bits.map(f32::from_bits)
    }

    fn bits(v: Xmm) -> Bits {
        v.map(f32::to_bits)
    }

    /// Runs `INS dest, src` (Intel syntax) on the CPU.
    macro_rules! hw {
        ($ins:literal, $dest:expr, $src:expr) => {{
            let mut dest = to_reg($dest);
            let src = to_reg($src);
            // SAFETY: SSE is part of the x86_64 baseline; registers only.
            unsafe {
                asm!(
                    concat!($ins, " {d}, {s}"),
                    d = inout(xmm_reg) dest,
                    s = in(xmm_reg) src,
                    options(pure, nomem, nostack),
                );
            }
            from_reg(dest)
        }};
        ($ins:literal, $dest:expr, $src:expr, $imm:literal) => {{
            let mut dest = to_reg($dest);
            let src = to_reg($src);
            // SAFETY: SSE is part of the x86_64 baseline; registers only.
            unsafe {
                asm!(
                    concat!($ins, " {d}, {s}, ", $imm),
                    d = inout(xmm_reg) dest,
                    s = in(xmm_reg) src,
                    options(pure, nomem, nostack),
                );
            }
            from_reg(dest)
        }};
    }

    /// Edge cases: zeros, denormals, normals, the largest floats, infinities,
    /// quiet and signalling NaNs of both signs with different payloads.
    const EDGES: [u32; 24] = [
        0x0000_0000,
        0x8000_0000,
        0x0000_0001,
        0x8000_0001,
        0x007f_ffff,
        0x807f_ffff,
        0x0080_0000,
        0x3f80_0000,
        0xbf80_0000,
        0x4040_0000,
        0x3eaa_aaab,
        0xc2f6_e979,
        0x7f7f_ffff,
        0xff7f_ffff,
        0x7f80_0000,
        0xff80_0000,
        0x7fc0_0000,
        0xffc0_0000,
        0x7fc0_1234,
        0xffd5_5555,
        0x7f80_0001,
        0xff81_2345,
        0x7fbf_ffff,
        0x0040_0000,
    ];

    /// Every ordered pair of edge cases, four to a register, then pseudo-random
    /// bit patterns (an LCG, so the run is the same every time).
    fn pairs() -> Vec<(Bits, Bits)> {
        let mut flat = Vec::new();
        for &a in &EDGES {
            for &b in &EDGES {
                flat.push((a, b));
            }
        }
        let mut seed: u32 = 0x1234_5678;
        let mut next = || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            seed
        };
        for _ in 0..4096 {
            let a = next();
            let b = next();
            // Every eighth pair shares the exponent, so subtraction cancels.
            let b = if a % 8 == 0 {
                (a & 0xff80_0000) | (b & 0x007f_ffff)
            } else {
                b
            };
            flat.push((a, b));
        }
        flat.chunks(4)
            .map(|chunk| {
                let mut dest = [0; 4];
                let mut src = [0; 4];
                for (lane, &(a, b)) in chunk.iter().enumerate() {
                    dest[lane] = a;
                    src[lane] = b;
                }
                (dest, src)
            })
            .collect()
    }

    fn check(name: &str, ours: Binary, cpu: fn(Bits, Bits) -> Bits) {
        for (dest, src) in pairs() {
            let want = cpu(dest, src);
            let got = bits(ours(xmm(dest), xmm(src)));
            assert_eq!(
                got, want,
                "{name} dest {dest:08x?} src {src:08x?}: ours {got:08x?}, cpu {want:08x?}"
            );
        }
    }

    #[test]
    fn packed_arithmetic_matches_the_cpu() {
        check("addps", addps, |d, s| hw!("addps", d, s));
        check("subps", subps, |d, s| hw!("subps", d, s));
        check("mulps", mulps, |d, s| hw!("mulps", d, s));
        check("divps", divps, |d, s| hw!("divps", d, s));
    }

    #[test]
    fn scalar_arithmetic_matches_the_cpu() {
        check("addss", addss, |d, s| hw!("addss", d, s));
        check("subss", subss, |d, s| hw!("subss", d, s));
        check("mulss", mulss, |d, s| hw!("mulss", d, s));
        check("divss", divss, |d, s| hw!("divss", d, s));
    }

    #[test]
    fn min_and_max_match_the_cpu() {
        check("minps", minps, |d, s| hw!("minps", d, s));
        check("maxps", maxps, |d, s| hw!("maxps", d, s));
        check("minss", minss, |d, s| hw!("minss", d, s));
        check("maxss", maxss, |d, s| hw!("maxss", d, s));
    }

    #[test]
    fn square_roots_match_the_cpu() {
        check("sqrtps", |_, s| sqrtps(s), |d, s| hw!("sqrtps", d, s));
        check("sqrtss", sqrtss, |d, s| hw!("sqrtss", d, s));
    }

    #[test]
    fn compares_match_the_cpu() {
        check(
            "cmpps 0",
            |d, s| cmpps(d, s, 0),
            |d, s| hw!("cmpps", d, s, 0),
        );
        check(
            "cmpps 1",
            |d, s| cmpps(d, s, 1),
            |d, s| hw!("cmpps", d, s, 1),
        );
        check(
            "cmpps 2",
            |d, s| cmpps(d, s, 2),
            |d, s| hw!("cmpps", d, s, 2),
        );
        check(
            "cmpps 3",
            |d, s| cmpps(d, s, 3),
            |d, s| hw!("cmpps", d, s, 3),
        );
        check(
            "cmpps 4",
            |d, s| cmpps(d, s, 4),
            |d, s| hw!("cmpps", d, s, 4),
        );
        check(
            "cmpps 5",
            |d, s| cmpps(d, s, 5),
            |d, s| hw!("cmpps", d, s, 5),
        );
        check(
            "cmpps 6",
            |d, s| cmpps(d, s, 6),
            |d, s| hw!("cmpps", d, s, 6),
        );
        check(
            "cmpps 7",
            |d, s| cmpps(d, s, 7),
            |d, s| hw!("cmpps", d, s, 7),
        );
        check(
            "cmpss 1",
            |d, s| cmpss(d, s, 1),
            |d, s| hw!("cmpss", d, s, 1),
        );
        check(
            "cmpss 6",
            |d, s| cmpss(d, s, 6),
            |d, s| hw!("cmpss", d, s, 6),
        );
    }

    #[test]
    fn the_destination_nan_wins_and_invalid_operations_give_the_default_nan() {
        let snan = f32::from_bits(0x7f80_0001);
        let qnan = f32::from_bits(0xffd5_5555);
        assert_eq!(add(snan, qnan).to_bits(), 0x7fc0_0001);
        assert_eq!(add(qnan, snan).to_bits(), 0xffd5_5555);
        assert_eq!(sub(1.0, snan).to_bits(), 0x7fc0_0001);
        assert_eq!(sub(f32::INFINITY, f32::INFINITY).to_bits(), DEFAULT_NAN);
        assert_eq!(mul(0.0, f32::NEG_INFINITY).to_bits(), DEFAULT_NAN);
        assert_eq!(div(0.0, -0.0).to_bits(), DEFAULT_NAN);
        assert_eq!(sqrt(-1.0).to_bits(), DEFAULT_NAN);
        assert_eq!(sqrt(-0.0).to_bits(), 0x8000_0000);
        assert_eq!(min(0.0, -0.0).to_bits(), 0x8000_0000);
        assert_eq!(min(-0.0, 0.0).to_bits(), 0);
        assert_eq!(max(1.0, snan).to_bits(), 0x7f80_0001);
        assert_eq!(max(snan, 1.0).to_bits(), 0x3f80_0000);
    }
}

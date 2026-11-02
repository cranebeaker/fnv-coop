//! Small C runtime entry points the platform stood in for (MSVC 2008 LIBCMT, statically linked
//! into FalloutNV.exe 1.4.0.525), translated from the exe: `_ftol2_sse` (`00ec62c0`), the square
//! roots `_CIsqrt` (`00ec6040`, register argument) and `sqrt` (`00ec6054`, stack argument), and
//! `_invalid_parameter_noinfo` (`00ec7c56`).

use super::*;

/// `__sse2_available` (`01270a6c`).
const SSE2_FLAG: u32 = 0x0127_0a6c;
/// `_ftol2`'s x87 body (`00ec62f6`, `ST0` in, `EDX:EAX` out; `cvt.rs`).
const FTOL2_X87: u32 = 0x00ec_62f6;
/// `00ed2808` (stack double in, `EAX` the exponent bits; `ZF` set for all ones), the shared
/// x87 square-root body `00ec605d` (`ZF`, `ST0`, the stack double) and `__fload_withFB`
/// (`00ed27c5`; `ECX` unused, `EDX` the double's address).
const EXPONENT_BITS_HELPER: u32 = 0x00ed_2808;
const SQRT_BODY: u32 = 0x00ec_605d;
const FLOAD_WITH_FB_ADDR: u32 = 0x00ed_27c5;
/// The invalid-parameter report (`00ec7c30`, five words).
const INVALID_PARAMETER_REPORT: u32 = 0x00ec_7c30;

/// `CVTTSD2SI`: the double truncated toward zero, the integer indefinite 0x80000000 for a NaN
/// or a value outside the 32-bit range.
fn cvttsd2si(value: f64) -> u32 {
    let t = value.trunc();
    if t.is_nan() || t < -2_147_483_648.0 || t > 2_147_483_647.0 {
        0x8000_0000
    } else {
        t as i32 as u32
    }
}

// Translated from 00ec62c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_ftol2_sse` (no label; the entry the compiler calls for a truncating conversion), `ST0` in.
/// Without SSE2 (`__sse2_available` 0) a jump to the x87 body `00ec62f6`, whose `EDX:EAX` is
/// answered. With it the value is stored as a double (`FSTP qword`, exact for the engine's
/// double register) and converted with `CVTTSD2SI` into `EAX`; `EDX` is not written by this
/// path (it keeps the caller's value, which the engine does not have: 0 here).
pub fn ftol2_sse(e: &mut Engine, value: f64) -> u64 {
    if e.mem.u32(SSE2_FLAG) == 0 {
        return e.call(FTOL2_X87, &args![value]).u64();
    }
    u64::from(cvttsd2si(value))
}

// Translated from 00ec6040 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_CIsqrt` (no label), `ST0` in and out: `SUB ESP,0xc; FST qword [ESP]`, `00ed2808` on the
/// stored double (its exponent bits, `ZF` for an exponent of all ones) and the x87 body
/// `00ec605d` with them.
pub fn ci_sqrt(e: &mut Engine, st0: f64) -> f64 {
    let stored = x87_load64(st0.to_bits()).to_bits();
    let (low, high) = (stored as u32, (stored >> 32) as u32);
    e.call(EXPONENT_BITS_HELPER, &args![low, high]);
    let zf = high & 0x7ff0_0000 == 0x7ff0_0000;
    e.call(SQRT_BODY, &args![zf, f64::from_bits(stored), low, high])
        .f64()
}

// Translated from 00ec6054 (disassembled, FalloutNV.exe 1.4.0.525)
/// `sqrt` (no label), cdecl, the double on the stack: `LEA EDX,[ESP+4]; CALL 00ed27c5` loads it
/// (`__fload_withFB`: `ST0`, `ZF` for an exponent of all ones) and the code falls into the x87
/// body `00ec605d` with the stack double.
pub fn sqrt(e: &mut Engine, value: f64) -> f64 {
    let bits = value.to_bits();
    let (low, high) = (bits as u32, (bits >> 32) as u32);
    let st0 = e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), bits);
        e.call(FLOAD_WITH_FB_ADDR, &args![0u32, slot.addr()]).st0
    });
    let zf = high & 0x7ff0_0000 == 0x7ff0_0000;
    e.call(SQRT_BODY, &args![zf, st0, low, high]).f64()
}

// Translated from 00ec7c56 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_invalid_parameter_noinfo` (no label), cdecl, no arguments: the invalid-parameter report
/// (`00ec7c30`) with five zeros.
pub fn invalid_parameter_noinfo(e: &mut Engine) {
    e.call(
        INVALID_PARAMETER_REPORT,
        &args![0u32, 0u32, 0u32, 0u32, 0u32],
    );
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_62c0, ftol2_sse(f64) -> u64),
        entry!(0x00ec_6040, ci_sqrt(f64) -> f64),
        entry!(0x00ec_6054, sqrt(f64) -> f64),
        entry!(0x00ec_7c56, invalid_parameter_noinfo()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn ftol2_sse_truncates_or_takes_the_x87_body() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        // Without SSE2: the x87 body's 64-bit answer.
        assert_eq!(ftol2_sse(&mut e, -2.75) as i64, -2);
        assert_eq!(ftol2_sse(&mut e, 5e9) as i64, 5_000_000_000);
        assert_eq!(calls(&e, FTOL2_X87).len(), 2);
        e.mem.set_u32(SSE2_FLAG, 1);
        assert_eq!(ftol2_sse(&mut e, -2.75), u64::from((-2i32) as u32));
        assert_eq!(ftol2_sse(&mut e, 2147483647.9), 0x7fff_ffff);
        assert_eq!(ftol2_sse(&mut e, 5e9), 0x8000_0000);
        assert_eq!(ftol2_sse(&mut e, f64::NAN), 0x8000_0000);
        assert_eq!(e.call(0x00ec_62c0, &args![-0.5f64]).u32(), 0);
        assert_eq!(calls(&e, FTOL2_X87).len(), 2);
    }

    #[test]
    fn square_roots_run_the_shared_body() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        assert_eq!(ci_sqrt(&mut e, 16.0), 4.0);
        assert_eq!(sqrt(&mut e, 2.25), 1.5);
        assert_eq!(e.call(0x00ec_6054, &args![9.0f64]).f64(), 3.0);
        let bits = 16.0f64.to_bits();
        let body = calls(&e, SQRT_BODY);
        assert_eq!(body.len(), 3);
        // ZF clear (a finite argument), then ST0 (two words) and the stack double.
        assert_eq!(&body[0][3..], &[bits as u32, (bits >> 32) as u32]);
        assert_eq!(body[0][0], 0);
        assert_eq!(calls(&e, EXPONENT_BITS_HELPER).len(), 1);
        assert_eq!(calls(&e, FLOAD_WITH_FB_ADDR).len(), 2);
    }

    #[test]
    fn invalid_parameter_noinfo_reports_five_zeros() {
        let mut e = Engine::new();
        e.register(INVALID_PARAMETER_REPORT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ec_7c56, &args![]);
        assert_eq!(calls(&e, INVALID_PARAMETER_REPORT), vec![vec![0; 5]]);
    }
}

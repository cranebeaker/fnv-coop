//! The C runtime's `floor` (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe 1.4.0.525),
//! translated from the exe: the entry `00ec6940` with its SSE2 body, the x87 body `00ed4563`,
//! and the math-error machinery both reach that nothing had translated: `__sptype`
//! (`00ed5981`), `__frnd` (`00ee23bc`), `__handle_qnan1` (`00ed5837`), `__except1`
//! (`00ed588c`) with `__handle_exc` (`00ed5554`), `__raise_exc` (`00ed5531`), `__errcode`
//! (`00ed5763`), `__umatherr` (`00ed5797`), `__set_errno_from_matherr` (`00ed5736`), the
//! status-flag helper `00ed5ae4`, `00ed59e1` (mantissa and exponent) and `__set_exp`
//! (`00ed5955`), and `___libm_error_support` (`00ed429f`) with the default handler `00ee0a3a`.
//! Names are Ghidra's function ID matches; `floor` is named from the name its error report
//! passes (`"floor"` at `010943f0`, the string the `_umatherr` table gives function code 0xb).
//! `_ctrlfp` (`00ed5abb`), `_errno` (`00ec85e3`) and `__decode_pointer` (`00ed0f92`) are in
//! `crt.rs`. Every call is made by address, as the exe makes it, so a test can double each one.
//!
//! Not translated: `__raise_exc_ex` (`00ed5255`, about 200 instructions, which raises the
//! floating-point exception), called by address from `__raise_exc`.
//!
//! The x87 control word is the engine's (`super::x87_control_word` for `FSTCW`,
//! `_ctrlfp` for every load), `MXCSR` the engine's `super::MXCSR`. `crate::sse` has
//! single-precision lanes only; the double-precision SSE2 instructions of `floor`'s body
//! (`CMPLTPD`, `SUBSD`) are written out where they run, each on operands that are never NaN
//! there, so the IEEE result is the instruction's (see each one).
//!
//! The switch itself: the C initializer `00ed454f` (`__cinit`'s `_initterm_e` table, run by
//! address in `crate::static_init`) sets `__use_sse2_mathfcns` (`01270a64`) from
//! `__get_sse2_info` (`00ed1927`) and its operating-system probe `00ed18d7`. What the processor
//! answers (the `ID` flag test and `CPUID` leaves 0 and 1) comes from the platform at the
//! addresses [`EFLAGS_ID_TEST`], [`CPUID_LEAF_0`] and [`CPUID_LEAF_1`].

use super::*;

/// `_ctrlfp` (`00ed5abb`, cdecl: new word, mask), `_errno` (`00ec85e3`, cdecl) and
/// `__decode_pointer` (`00ed0f92`, cdecl, one word): in `crt.rs`.
const CTRLFP: u32 = 0x00ed_5abb;
const ERRNO_ADDR: u32 = 0x00ec_85e3;
const DECODE_POINTER: u32 = 0x00ed_0f92;
/// The functions of this file, called by address as the exe calls them (`floor` jumps to its
/// x87 body, so that one is a direct call).
const SPTYPE: u32 = 0x00ed_5981;
const FRND: u32 = 0x00ee_23bc;
const HANDLE_QNAN1: u32 = 0x00ed_5837;
const EXCEPT1: u32 = 0x00ed_588c;
const HANDLE_EXC: u32 = 0x00ed_5554;
const RAISE_EXC: u32 = 0x00ed_5531;
const ERRCODE: u32 = 0x00ed_5763;
const UMATHERR: u32 = 0x00ed_5797;
const SET_ERRNO_FROM_MATHERR: u32 = 0x00ed_5736;
const SET_STATFP: u32 = 0x00ed_5ae4;
const DECOMP: u32 = 0x00ed_59e1;
const SET_EXP: u32 = 0x00ed_5955;
const LIBM_ERROR_SUPPORT: u32 = 0x00ed_429f;
const DEFAULT_MATHERR: u32 = 0x00ee_0a3a;
/// `__raise_exc_ex` (`00ed5255`, cdecl, seven words): not translated.
const RAISE_EXC_EX: u32 = 0x00ed_5255;

/// `floor`'s SSE2 constants (16 bytes each, only the low qword reaches the result): 1.0
/// (`010f1f30`), the shift base `0x433` (`010f1f40`), -1.0 (`010f1f50`), -0.0 (`010f1f60`)
/// and the exponent mask `0x7ff` (`010f1f70`).
const FLOOR_ONE: u32 = 0x010f_1f30;
const FLOOR_SHIFT_BASE: u32 = 0x010f_1f40;
const FLOOR_MINUS_ONE: u32 = 0x010f_1f50;
const FLOOR_MINUS_ZERO: u32 = 0x010f_1f60;
const FLOOR_EXPONENT_MASK: u32 = 0x010f_1f70;
/// The control word `floor`'s x87 body loads (`011c2408`; 0x173f in the exe: round down).
const FLOOR_CONTROL_WORD: u32 = 0x011c_2408;
/// The double 1.0 the x87 body adds to an infinity or NaN for its report (`01012070`) and the
/// double `__handle_exc` multiplies a lost underflow by (`01012060`, 0.0 in the exe).
const ONE_DOUBLE: u32 = 0x0101_2070;
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `__handle_exc`'s overflow results: the double at `011c24f8` (+infinity in the exe) and the one
/// at `011c2508` (the largest double).
const OVERFLOW_INFINITY: u32 = 0x011c_24f8;
const OVERFLOW_LARGEST: u32 = 0x011c_2508;
/// The extended constants `00ed5ae4` loads to raise status flags: `011c2520` (huge) and
/// `011c252c` (tiny).
const STATFP_HUGE: u32 = 0x011c_2520;
const STATFP_TINY: u32 = 0x011c_252c;
/// A global `__handle_qnan1` and `__except1` test before reporting through `_umatherr`: nonzero
/// skips the report (no label in the map; the name is unconfirmed).
const MATHERR_SKIP: u32 = 0x011c_2b60;
/// `_umatherr`'s table of 0x1d (function code, name) pairs (`011c2410`).
const MATHERR_NAMES: u32 = 0x011c_2410;
/// The user `_matherr` flag `___libm_error_support` tests (`012704a4`) and the encoded pointer it
/// then decodes (`01270a68`); both unconfirmed names.
const USER_MATHERR_SET: u32 = 0x0127_04a4;
const USER_MATHERR: u32 = 0x0127_0a68;
/// `floor`'s function code (`__sptype`'s callers and `_umatherr`'s table) and the error code it
/// gives `___libm_error_support`.
const FLOOR_CODE: u32 = 0xb;
const FLOOR_ERROR: u32 = 0x3ed;
/// `EDOM` and `ERANGE`.
const EDOM: u32 = 0x21;
const ERANGE: u32 = 0x22;
/// The sign bit of a double (`FCHS` flips it, `OR word ..., 0x8000` sets it).
const SIGN: u64 = 1 << 63;

/// `errno` set through `_errno`.
fn store_errno(e: &mut Engine, code: u32) {
    let slot = e.call(ERRNO_ADDR, &args![]).u32();
    e.mem.set_u32(slot, code);
}

// Translated from 00ec6940 (disassembled, FalloutNV.exe 1.4.0.525)
/// `floor` (LIBCMT; no label in the map: named from the body, which rounds toward -infinity, and
/// from the name `"floor"` (`010943f0`) its error report passes), cdecl, the double on the stack,
/// the result in `ST0`. With `__use_sse2_mathfcns` (`01270a64`) zero it jumps to the x87 body
/// (`00ed4563`, [`floor_x87`]); otherwise it reads `MXCSR` (`STMXCSR`) and jumps there too
/// unless every SSE exception is masked (`& 0x1f80 == 0x1f80`), then the x87 control word
/// (`FNSTCW`) and jumps there unless every x87 exception is masked (`& 0x7f == 0x7f`): the
/// engine's `MXCSR` and the running thread's control word (`super::sse2_math_available`,
/// the same three tests in the same order). Otherwise the SSE2 body (`00ec6980`) computes the
/// result with integer shifts on the bit pattern.
///
/// SSE2 body: the shift count is `0x433 - exponent`. A positive value below 1 (zero included)
/// gives +0 (`FLDZ`); up to the exponent `0x432` its fraction bits are cleared. A negative value
/// above -1 gives -1.0, and -0 stays -0; up to the exponent `0x432` the fraction bits are cleared
/// and 1.0 subtracted when that raised the value. Beyond that the value comes back as it is (an
/// integer, or an infinity), a NaN after `___libm_error_support(&x, &x, &x, 0x3ed)` has had its
/// say about the argument's slot, which is then the result.
pub fn floor(e: &mut Engine, x: f64) -> f64 {
    if !super::sse2_math_available(e) {
        return floor_x87(e, x);
    }
    let bits = x.to_bits();
    // MOVAPD XMM2, [010f1f40]; MOVAPD XMM1, XMM0; MOVAPD XMM7, XMM0; PSRLQ XMM0, 0x34; MOVD EAX.
    let base = e.mem.u64(FLOOR_SHIFT_BASE);
    let shifted = bits >> 52;
    let sign_and_exponent = shifted as u32;
    // ANDPD XMM0, [010f1f70]; PSUBD XMM2, XMM0: each dword of the low qword separately.
    let exponent = shifted & e.mem.u64(FLOOR_EXPONENT_MASK);
    let count_low = (base as u32).wrapping_sub(exponent as u32);
    let count_high = ((base >> 32) as u32).wrapping_sub((exponent >> 32) as u32);
    let count = u64::from(count_low) | (u64::from(count_high) << 32);
    // PSRLQ / PSLLQ by a count above 63 give zero.
    let shift_right = |value: u64| if count > 63 { 0 } else { value >> count };
    let shift_left = |value: u64| if count > 63 { 0 } else { value << count };
    let right = shift_right(bits); // PSRLQ XMM1, XMM2
    if sign_and_exponent & 0x800 == 0 {
        // CMP EAX, 0x3ff; JL 00ec6a3a: FLDZ.
        if (sign_and_exponent as i32) < 0x3ff {
            return 0.0;
        }
        let truncated = shift_left(right); // PSLLQ XMM1, XMM2
        if (sign_and_exponent as i32) > 0x432 {
            return floor_out_of_range(e, x);
        }
        // MOVQ [ESP+4], XMM1; FLD double ptr [ESP+4].
        return x87_load64(truncated);
    }
    // 00ec6a02: MOVQ XMM0, [ESP+4] (the argument again); PSLLQ XMM1, XMM2; MOVAPD XMM3, XMM0.
    let truncated = shift_left(right);
    // CMPLTPD XMM0, XMM1: the argument below its truncation (a NaN goes out of range below
    // before the mask is used).
    let below = x < f64::from_bits(truncated);
    if (sign_and_exponent as i32) < 0xbff {
        // 00ec6a3d: CMPLTPD XMM3, [010f1f60] (below -0: every negative number but -0; a NaN has
        // an exponent above 0xbff); ORPD XMM3, [010f1f60]; ANDPD XMM3, [010f1f50].
        let minus_zero = e.mem.u64(FLOOR_MINUS_ZERO);
        let mask = if x < f64::from_bits(minus_zero) {
            u64::MAX
        } else {
            0
        };
        let result = (mask | minus_zero) & e.mem.u64(FLOOR_MINUS_ONE);
        return x87_load64(result);
    }
    if (sign_and_exponent as i32) > 0xc32 {
        return floor_out_of_range(e, x);
    }
    // ANDPD XMM0, [010f1f30]; SUBSD XMM1, XMM0: the truncation (a nonzero integer of at most 52
    // bits) minus 1.0 or +0, exact, never a NaN.
    let subtrahend = if below { e.mem.u64(FLOOR_ONE) } else { 0 };
    let result = f64::from_bits(truncated) - f64::from_bits(subtrahend);
    x87_load64(result.to_bits())
}

/// `floor`'s exit for an exponent above `0x432` (`00ec69d3`): `UCOMISD XMM7, XMM7` (unordered for a
/// NaN only); a number comes back as it is (`FLD` of the argument's slot); a NaN is first given to
/// `___libm_error_support` with that slot for all three pointers and the error code 0x3ed, and
/// the slot as it leaves it is loaded.
fn floor_out_of_range(e: &mut Engine, x: f64) -> f64 {
    if !x.is_nan() {
        return x87_load64(x.to_bits());
    }
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        e.call(LIBM_ERROR_SUPPORT, &args![slot, slot, slot, FLOOR_ERROR]);
        x87_load64(e.mem.u64(slot.addr()))
    })
}

// Translated from 00ed4563 (disassembled, FalloutNV.exe 1.4.0.525)
/// The x87 body of `floor` (no label; Ghidra counts it into `floor`'s range), reached by a jump
/// from [`floor`] with the same stack: cdecl, the double on the stack, the result in `ST0`. It
/// loads the round-down control word (`011c2408`) through `_ctrlfp(word, 0xffff)`, keeping the
/// old one. The argument's exponent word (`[EBP+0xe] & 0x7ff0`) all ones: `__sptype` of the
/// argument as `FLD`/`FSTP` copy it (a signalling NaN quiet); classes 1 and 2 (the infinities)
/// put the old control word back and return the argument (`FLD`), class 3 (a quiet NaN) is
/// `__handle_qnan1(0xb, x, old)`, anything else `__except1(8, 0xb, x, x + 1.0, old)` (the sum by
/// `FADD` under the loaded word). A finite value is rounded by `__frnd`; when that equals the
/// argument (`FCOMP`, `TEST AH,0x44`, `JP`), or the old word has bit 5 (the precision-exception
/// mask) set, the old word is put back and the rounded value returned, otherwise the inexact
/// result goes to `__except1(0x10, 0xb, x, rounded, old)`.
pub fn floor_x87(e: &mut Engine, x: f64) -> f64 {
    let new_control = e.mem.u32(FLOOR_CONTROL_WORD);
    // EBX: the old word as `_ctrlfp` answers it (sign-extended).
    let old_control = e.call(CTRLFP, &args![new_control, 0xffffu32]).u32();
    let bits = x.to_bits();
    // FLD double ptr [EBP+8]; FSTP double ptr [ESP]: the pushed copy.
    let pushed = x87_load64(bits).to_bits();
    // MOV ECX, dword ptr [EBP+0xe]; AND ECX, 0x7ff0; CMP CX, AX: the double's top word (the two
    // bytes above it are masked away).
    let exponent_word = ((bits >> 48) as u32) & 0x7ff0;
    if exponent_word == 0x7ff0 {
        let class = e.call(SPTYPE, &args![pushed]).i32();
        if class > 0 {
            if class <= 2 {
                // 00ed45c2
                e.call(CTRLFP, &args![old_control, 0xffffu32]);
                return x87_load64(bits);
            }
            if class == 3 {
                // 00ed45ad
                return e
                    .call(HANDLE_QNAN1, &args![FLOOR_CODE, pushed, old_control])
                    .f64();
            }
        }
        // 00ed45d0: FLD x; FADD double ptr [01012070]; FSTP into the result's slot.
        let one = f64::from_bits(e.mem.u64(ONE_DOUBLE));
        let sum = x87_add(x87_load64(bits), one).to_bits();
        return e
            .call(EXCEPT1, &args![8u32, FLOOR_CODE, pushed, sum, old_control])
            .f64();
    }
    // 00ed45ed: __frnd of the pushed copy; FSTP double ptr [EBP-8].
    let rounded = e.call(FRND, &args![pushed]).f64().to_bits();
    // FLD x; FCOMP double ptr [EBP-8]; FNSTSW AX; TEST AH, 0x44; JP: equal only when ordered and
    // equal (both are numbers here).
    if x87_load64(bits) == f64::from_bits(rounded) || old_control & 0x20 != 0 {
        // 00ed4604
        e.call(CTRLFP, &args![old_control, 0xffffu32]);
        return x87_load64(rounded);
    }
    // 00ed4617
    e.call(
        EXCEPT1,
        &args![0x10u32, FLOOR_CODE, pushed, rounded, old_control],
    )
    .f64()
}

// Translated from 00ed5981 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__sptype` (LIBCMT, Ghidra function ID), cdecl, the double on the stack (low word, high word):
/// 1 for +infinity, 2 for -infinity, 3 for a quiet NaN, 4 for a signalling NaN, 0 otherwise. The
/// NaN tests read the top word (`[EBP+0xe]`, the two bytes above masked away): `& 0x7ff8 ==
/// 0x7ff8` is quiet; `& 0x7ff8 == 0x7ff0` with fraction bits set (`high & 0x7ffff`, or the low
/// word) is signalling.
pub fn sptype(_e: &mut Engine, low: u32, high: u32) -> i32 {
    if high == 0x7ff0_0000 {
        if low == 0 {
            return 1;
        }
    } else if high == 0xfff0_0000 && low == 0 {
        return 2;
    }
    // 00ed59ae
    let top = (high >> 16) & 0x7ff8;
    if top == 0x7ff8 {
        return 3;
    }
    if top == 0x7ff0 && (high & 0x7ffff != 0 || low != 0) {
        return 4;
    }
    0
}

// Translated from 00ee23bc (disassembled, FalloutNV.exe 1.4.0.525)
/// `__frnd` (LIBCMT, Ghidra function ID), cdecl, the double on the stack, the result in `ST0`:
/// `FLD`, `FRNDINT` (the running control word's rounding mode), `FSTP double`, `FLD`.
pub fn frnd(_e: &mut Engine, x: f64) -> f64 {
    let stored = x87_frndint(X87Ext::load64(x)).store64();
    x87_load64(stored.to_bits())
}

// Translated from 00ed5837 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__handle_qnan1` (LIBCMT, Ghidra function ID), cdecl: (function code, the double, the control
/// word to restore); the result in `ST0`. With the global at `011c2b60` zero it is
/// `_umatherr(1, code, x, 0.0, x, cw)` (`x` as `FLD`/`FSTP` copy it, so quiet), whose result it
/// returns. Otherwise `errno` becomes `EDOM`, `_ctrlfp(cw, 0xffff)` restores the control word and
/// `x` is returned (`FLD`).
pub fn handle_qnan1(e: &mut Engine, code: u32, x: f64, control: u32) -> f64 {
    let bits = x.to_bits();
    if e.global::<u32>(MATHERR_SKIP) == 0 {
        let copy = x87_load64(bits).to_bits();
        // FLDZ; FSTP double ptr [ESP+8]: +0.0.
        return e
            .call(UMATHERR, &args![1u32, code, copy, 0u64, copy, control])
            .f64();
    }
    // 00ed586d
    store_errno(e, EDOM);
    e.call(CTRLFP, &args![control, 0xffffu32]);
    x87_load64(bits)
}

// Translated from 00ed588c (disassembled, FalloutNV.exe 1.4.0.525)
/// `__except1` (LIBCMT, Ghidra function ID), cdecl: (operation flags, function code, argument,
/// result, control word), the doubles two words each; the result in `ST0`. Its frame keeps the
/// argument, result and control word where the callees can change them. `__handle_exc(flags,
/// &result, cw)` first; when it answers 0 the word at +0x40 of a 0x80-byte local loses bit 0 and
/// `__raise_exc(&local, &cw, flags, code, &argument, &result)` runs. Then `__errcode(flags)`: with
/// the global at `011c2b60` zero and a nonzero code the answer is `_umatherr(errcode, code,
/// argument, 0.0, result, cw)` (doubles as `FLD`/`FSTP` copy them); otherwise
/// `__set_errno_from_matherr(errcode)`, `_ctrlfp(cw, 0xffff)` and the result (`FLD`).
// Translation incomplete: 00ed588c - the /GS stack cookie check (00ec408c) is not translated.
pub fn except1(
    e: &mut Engine,
    flags: u32,
    code: u32,
    argument: f64,
    result: f64,
    control: u32,
) -> f64 {
    // The 0x80-byte local (EBP-0x80), then the argument (EBX+0x10), result (EBX+0x18) and control
    // word (EBX+0x20).
    e.with_stack(0x98, |e, block| {
        let local = block.addr();
        let argument_at = local + 0x80;
        let result_at = local + 0x88;
        let control_at = local + 0x90;
        e.mem.set_u64(argument_at, argument.to_bits());
        e.mem.set_u64(result_at, result.to_bits());
        e.mem.set_u32(control_at, control);
        let word = e.mem.u32(control_at);
        let handled = e.call(HANDLE_EXC, &args![flags, result_at, word]).u32();
        if handled == 0 {
            // 00ed58c9: AND dword ptr [EBP-0x40], 0xfffffffe.
            let field = e.mem.u32(local + 0x40);
            e.mem.set_u32(local + 0x40, field & !1);
            e.call(
                RAISE_EXC,
                &args![local, control_at, flags, code, argument_at, result_at],
            );
        }
        let error = e.call(ERRCODE, &args![flags]).u32();
        if e.global::<u32>(MATHERR_SKIP) == 0 && error != 0 {
            // 00ed5903
            let word = e.mem.u32(control_at);
            let result_copy = x87_load64(e.mem.u64(result_at)).to_bits();
            let argument_copy = x87_load64(e.mem.u64(argument_at)).to_bits();
            return e
                .call(
                    UMATHERR,
                    &args![error, code, argument_copy, 0u64, result_copy, word],
                )
                .f64();
        }
        // 00ed592a
        e.call(SET_ERRNO_FROM_MATHERR, &args![error]);
        let word = e.mem.u32(control_at);
        e.call(CTRLFP, &args![word, 0xffffu32]);
        x87_load64(e.mem.u64(result_at))
    })
}

/// `FLD` of a double, `FCHS` when `negate`, `FSTP` into `at`.
fn store_double(e: &mut Engine, at: u32, bits: u64, negate: bool) {
    let loaded = x87_load64(bits).to_bits();
    e.mem
        .set_u64(at, if negate { loaded ^ SIGN } else { loaded });
}

// Translated from 00ed5554 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__handle_exc` (LIBCMT, Ghidra function ID), cdecl: (operation flags, pointer to the result,
/// control word); answers 1 when every flag of `flags & 0x1f` was handled by a masked exception,
/// else 0. The first of these that applies runs, each raising its status flag through
/// `00ed5ae4`:
///
/// - flag 8 with mask bit 1: (status 1) handled;
/// - flag 4 with mask bit 4: (status 4) handled;
/// - flag 1 with mask bit 8 (status 8): the result becomes the overflow value of the control
///   word's rounding mode (bits 10-11) and the result's sign (`0 < result` is positive; a NaN or
///   zero counts as negative): nearest: +-`011c24f8`; down: `011c2508` or -`011c24f8`; up:
///   `011c24f8` or -`011c2508`; toward zero: +-`011c2508`; handled;
/// - flag 2 with mask bit 0x10: an exact zero result is inexact-flagged as it is; otherwise
///   `00ed59e1` splits it into a mantissa and an exponent, the exponent less 0x600: below -0x432
///   the result is the mantissa times `01012060` (inexact); else the mantissa's top word keeps its
///   low four bits with the lowest exponent bit set, is shifted right one bit at a time while the
///   exponent is below -0x3fd (a 1 shifted out is inexact), and negated when the mantissa was
///   negative; it is stored as the result. Inexact (or flag 0x10 given) raises status 0x10;
///   handled.
///
/// Then flag 0x10 with mask bit 0x20 (status 0x20) is handled too.
pub fn handle_exc(e: &mut Engine, flags: u32, result: Ptr, control: u32) -> i32 {
    let at = result.addr();
    let mut remaining = flags & 0x1f; // ESI
    let saved = remaining; // [EBP-4]
    if flags & 8 != 0 && control & 1 != 0 {
        e.call(SET_STATFP, &args![1u32]);
        remaining &= !8;
    } else if flags & 4 != 0 && control & 4 != 0 {
        e.call(SET_STATFP, &args![4u32]);
        remaining &= !4;
    } else if flags & 1 != 0 && control & 8 != 0 {
        e.call(SET_STATFP, &args![8u32]);
        // FLDZ; FCOMP double ptr [ECX]; FNSTSW AX; TEST AH, 5: parity odd (JNP) exactly when
        // 0 < result (C0 alone).
        let positive = 0.0 < f64::from_bits(e.mem.u64(at));
        let infinity = e.mem.u64(OVERFLOW_INFINITY);
        let largest = e.mem.u64(OVERFLOW_LARGEST);
        match control & 0xc00 {
            0 => store_double(e, at, infinity, !positive),
            0x400 => {
                if positive {
                    store_double(e, at, largest, false);
                } else {
                    store_double(e, at, infinity, true);
                }
            }
            0x800 => {
                if positive {
                    store_double(e, at, infinity, false);
                } else {
                    store_double(e, at, largest, true);
                }
            }
            0xc00 => store_double(e, at, largest, !positive),
            // CMP EAX, ECX; JNZ 00ed5638: unreachable (the mask leaves only the four values
            // above); nothing is stored.
            _ => {}
        }
        remaining &= !1;
    } else if flags & 2 != 0 && control & 0x10 != 0 {
        let mut inexact = flags & 0x10 != 0;
        // FLDZ; FCOMP double ptr [EDI]; TEST AH, 0x44; JNP 00ed56fe: equal (C3 alone).
        let value = e.mem.u64(at);
        if 0.0 == f64::from_bits(value) {
            inexact = true;
        } else {
            let copy = x87_load64(value).to_bits();
            let (mantissa, exponent) = e.with_stack(4, |e, slot| {
                let mantissa = e.call(DECOMP, &args![copy, slot]).f64().to_bits();
                (mantissa, e.mem.i32(slot.addr()))
            });
            let mut bits = mantissa; // [EBP-0x14]
            let biased = exponent.wrapping_add(-0x600); // ADD ECX, 0xfffffa00
            if biased < -0x432 {
                inexact = true;
                let zero = f64::from_bits(e.mem.u64(ZERO_DOUBLE));
                bits = x87_mul(x87_load64(bits), zero).to_bits();
            } else {
                // FLDZ; FCOMP; TEST AH, 0x41; JNZ: none of C0 and C3 set exactly when 0 > mantissa.
                let negative = 0.0 > f64::from_bits(bits);
                // MOV EAX, dword ptr [EBP-0xe]; AND EAX, 0xf; OR EAX, 0x10; MOV word ptr
                // [EBP-0xe], AX.
                let top = ((bits >> 48) & 0xf) | 0x10;
                bits = (bits & 0x0000_ffff_ffff_ffff) | (top << 48);
                if biased < -0x3fd {
                    let mut count = (-0x3fd_i32).wrapping_sub(biased);
                    loop {
                        if bits & 1 != 0 && !inexact {
                            inexact = true;
                        }
                        bits >>= 1;
                        count -= 1;
                        if count == 0 {
                            break;
                        }
                    }
                }
                if negative {
                    // FLD; FCHS; FSTP double ptr [EBP-0x14].
                    bits = x87_load64(bits).to_bits() ^ SIGN;
                }
            }
            // 00ed56f7
            store_double(e, at, bits, false);
        }
        // 00ed5700
        if inexact {
            e.call(SET_STATFP, &args![0x10u32]);
        }
        remaining = saved & !2;
    }
    // 00ed5714
    if flags & 0x10 != 0 && control & 0x20 != 0 {
        e.call(SET_STATFP, &args![0x20u32]);
        remaining &= !0x10;
    }
    i32::from(remaining == 0)
}

// Translated from 00ed5531 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__raise_exc` (LIBCMT, Ghidra function ID), cdecl, six words: `__raise_exc_ex` (`00ed5255`)
/// with the same six words and a seventh of 0; its `EAX` comes back.
pub fn raise_exc(e: &mut Engine, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    e.call(RAISE_EXC_EX, &args![a0, a1, a2, a3, a4, a5, 0u32])
        .u32()
}

// Translated from 00ed5763 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__errcode` (LIBCMT, Ghidra function ID), cdecl, the operation flags (their low byte read):
/// the `_exception` type: 5 for 0x20, else 1 for 8, 2 for 4, 3 for 1, else 4 for 2 and 0.
pub fn errcode(_e: &mut Engine, flags: u32) -> i32 {
    let low = flags as u8;
    if low & 0x20 != 0 {
        5
    } else if low & 8 != 0 {
        1
    } else if low & 4 != 0 {
        2
    } else if low & 1 != 0 {
        3
    } else {
        // MOVZX EAX, AL; AND EAX, 2; ADD EAX, EAX.
        i32::from(low & 2) * 2
    }
}

// Translated from 00ed5736 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__set_errno_from_matherr` (LIBCMT, Ghidra function ID), cdecl, the `_exception` type: 1 sets
/// `errno` `EDOM`, 2 and 3 `ERANGE` (signed compares), others nothing.
pub fn set_errno_from_matherr(e: &mut Engine, kind: i32) {
    if kind == 1 {
        store_errno(e, EDOM);
    } else if kind > 1 && kind <= 3 {
        store_errno(e, ERANGE);
    }
}

// Translated from 00ed5797 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__umatherr` (LIBCMT, Ghidra function ID), cdecl: (type, function code, argument 1,
/// argument 2, result, control word), the doubles two words each; the result in `ST0`. The
/// function code is looked up in the 0x1d (code, name) pairs at `011c2410`. Without a name (none
/// found, or a null one), `_ctrlfp(cw, 0xffff)`, `__set_errno_from_matherr(type)` and the
/// result (`FLD`). Otherwise an `_exception` (type, name, the three doubles copied as words) on
/// the stack, `_ctrlfp(cw, 0xffff)`, `_matherr` (`00ee0a3a`) with its address, which when it
/// answers 0 is followed by `__set_errno_from_matherr(type)`; the result is the `_exception`'s
/// (`FLD`).
pub fn umatherr(
    e: &mut Engine,
    kind: u32,
    code: u32,
    argument1: f64,
    argument2: f64,
    result: f64,
    control: u32,
) -> f64 {
    let mut name = 0;
    for i in 0..0x1d {
        if e.mem.u32(MATHERR_NAMES + 8 * i) == code {
            name = e.mem.u32(MATHERR_NAMES + 8 * i + 4);
            break;
        }
    }
    if name == 0 {
        // 00ed581a
        e.call(CTRLFP, &args![control, 0xffffu32]);
        e.call(SET_ERRNO_FROM_MATHERR, &args![kind]);
        return x87_load64(result.to_bits());
    }
    // The `_exception` at EBP-0x20: type, name, arg1, arg2, retval.
    e.with_stack(0x20, |e, exception| {
        let at = exception.addr();
        e.mem.set_u32(at, kind);
        e.mem.set_u32(at + 4, name);
        e.mem.set_u64(at + 8, argument1.to_bits());
        e.mem.set_u64(at + 0x10, argument2.to_bits());
        e.mem.set_u64(at + 0x18, result.to_bits());
        e.call(CTRLFP, &args![control, 0xffffu32]);
        let answer = e.call(DEFAULT_MATHERR, &args![exception]).u32();
        if answer == 0 {
            e.call(SET_ERRNO_FROM_MATHERR, &args![kind]);
        }
        x87_load64(e.mem.u64(at + 0x18))
    })
}

// Translated from 00ee0a3a (disassembled, FalloutNV.exe 1.4.0.525)
/// The `_matherr` `__umatherr` calls and `___libm_error_support` defaults to (no label; Ghidra
/// gives the body no name): `XOR EAX, EAX; RET`, cdecl, the `_exception` pointer unread: answers 0
/// (the error is not handled, so `errno` is set).
pub fn fn_00ee0a3a(_e: &mut Engine, _exception: Ptr) -> u32 {
    0
}

// Translated from 00ed5ae4 (disassembled, FalloutNV.exe 1.4.0.525)
/// The status-flag helper of `__handle_exc` (no label; `_set_statfp` by its use, unconfirmed),
/// cdecl, the flags (low byte read): each set bit runs an x87 operation that raises one exception
/// flag in the status word, `WAIT` after each: 1 `FISTP dword` of the huge extended at `011c2520`
/// (invalid), 8 `FSTP double` of it (overflow), 0x10 `FSTP double` of the tiny one at `011c252c`
/// (underflow), 4 `1 / 0` (`FLDZ; FLD1; FDIVRP`, zero divide), 0x20 `FSTP double` of `FLDPI`
/// (inexact). The values go to the function's own stack slots and are not read again; the
/// constants are read here as the loads read them.
// Translation incomplete: 00ed5ae4 - the x87 status word's exception flags it raises are not modelled (the engine keeps no status word).
pub fn fn_00ed5ae4(e: &mut Engine, flags: u32) {
    let low = flags as u8;
    if low & 1 != 0 {
        let _ = e.mem.bytes(STATFP_HUGE, 10);
    }
    if low & 8 != 0 {
        let _ = e.mem.bytes(STATFP_HUGE, 10);
    }
    if low & 0x10 != 0 {
        let _ = e.mem.bytes(STATFP_TINY, 10);
    }
    // Bits 4 (`FLDZ; FLD1; FDIVRP; FSTP ST0`) and 0x20 (`FLDPI; FSTP double`) read no memory and
    // leave nothing but the status flags.
}

// Translated from 00ed59e1 (disassembled, FalloutNV.exe 1.4.0.525)
/// The mantissa and exponent of a double (no label; `_decomp` by its use, unconfirmed), cdecl:
/// (the double, pointer to the exponent); the mantissa in `ST0`. A zero (`FCOM` equal) gives +0
/// and exponent 0. A value whose exponent field is nonzero (or, unreachable after the zero test,
/// a zero bit pattern) gives `__set_exp(x, 0)` and the exponent field less 0x3fe. A denormal is
/// shifted left (as 64 bits; the sign shifts out) until bit 52 is set, the exponent counting down
/// from -0x3fd; bit 52 is cleared, the sign set again for a negative value (`0 > x`), and
/// `__set_exp` of that and 0 is the mantissa.
pub fn fn_00ed59e1(e: &mut Engine, x: f64, exponent: Ptr) -> f64 {
    let bits = x.to_bits();
    // FLDZ; FCOM double ptr [EBP+8]; TEST AH, 0x44; JP: equal falls through.
    if 0.0 == x {
        e.mem.set_i32(exponent.addr(), 0);
        return 0.0;
    }
    // MOV EDX, dword ptr [EBP+0xe]: the top word (the two bytes above it are never used).
    let top = (bits >> 48) as u32 & 0xffff;
    let (low, high) = (bits as u32, (bits >> 32) as u32);
    if top & 0x7ff0 != 0 || (high & 0xfffff == 0 && low == 0) {
        // 00ed5a71: FSTP ST0; FLD x; FSTP double ptr [ESP].
        let copy = x87_load64(bits).to_bits();
        let mantissa = e.call(SET_EXP, &args![copy, 0u32]).f64();
        let field = ((top >> 4) & 0x7ff) as i32;
        e.mem.set_i32(exponent.addr(), field - 0x3fe);
        return mantissa;
    }
    // 00ed5a14: FCOMP double ptr [EBP+8]; TEST AH, 0x41; JNZ: neither C0 nor C3 when 0 > x.
    let negative = 0.0 > x;
    let mut count = -0x3fd_i32;
    let mut shifted = bits;
    while shifted & (1 << 52) == 0 {
        shifted <<= 1;
        count -= 1;
    }
    shifted &= !(1 << 52); // AND word ptr [EBP+0xe], 0xffef
    if negative {
        shifted |= SIGN; // OR word ptr [EBP+0xe], 0x8000
    }
    let copy = x87_load64(shifted).to_bits();
    let mantissa = e.call(SET_EXP, &args![copy, 0u32]).f64();
    e.mem.set_i32(exponent.addr(), count);
    mantissa
}

// Translated from 00ed5955 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__set_exp` (LIBCMT, Ghidra function ID), cdecl: (the double, an exponent); the result in
/// `ST0`. The double as `FLD`/`FSTP` copy it, its top word replaced by the low 16 bits of
/// `(exponent + 0x3fe) << 4` ORed with the argument's own top word `& 0x800f` (sign and the top
/// fraction bits, from the argument as passed), then `FLD`.
pub fn set_exp(_e: &mut Engine, x: f64, exponent: i32) -> f64 {
    let bits = x.to_bits();
    let copy = x87_load64(bits).to_bits(); // FSTP double ptr [EBP-8]
    let field = ((bits >> 48) as u32) & 0x800f; // MOV ECX, dword ptr [EBP+0xe]; AND ECX, 0x800f
    let top = ((exponent.wrapping_add(0x3fe) as u32) << 4) | field;
    let replaced = (copy & 0x0000_ffff_ffff_ffff) | (u64::from(top as u16) << 48);
    x87_load64(replaced)
}

/// How `___libm_error_support` reports an error code: its `_exception` type and name and which
/// path fills the `_exception`.
#[derive(Clone, Copy)]
enum Report {
    /// `00ed4350`: the given type; `errno` `ERANGE` when the handler answers 0.
    Range(u32, u32),
    /// `00ed4385`: type 4 (underflow); `errno` left alone.
    Underflow(u32),
    /// `00ed43be`: type 1 (domain); `errno` `EDOM` when the handler answers 0.
    Domain(u32),
    /// `00ed4434`: the result becomes argument 1 first, then as `Domain`.
    CopyDomain(u32),
    /// `00ed44e0`: the result becomes argument 1 times +0 (`FMUL` by the zeroed local), which is
    /// also the `_exception`'s result, then as `Domain`.
    ZeroDomain(u32),
    /// `00ed4452`: the result becomes 1.0 (`FLD1`), no report.
    One,
    /// `00ed4516`: nothing.
    Nothing,
}

/// `___libm_error_support`'s dispatch on the error code (signed compares, then the jump table at
/// `00ed451b` for 1000..1012): the names are the strings' addresses.
fn libm_report(code: i32) -> Report {
    match code {
        0xa6 => Report::Range(3, 0x010f_3184),
        2 => Report::Range(2, 0x010f_319c),
        3 => Report::Domain(0x010f_319c),
        8 => Report::Range(2, 0x010f_31a0),
        9 => Report::Domain(0x010f_31a0),
        14 => Report::Range(3, 0x010f_31a8),
        15 => Report::Underflow(0x010f_31a8),
        24 => Report::Range(3, 0x010f_31ac),
        0x19 => Report::Underflow(0x010f_31ac),
        0x1a => Report::One,
        0x1b => Report::Range(2, 0x010f_31ac),
        0x1c => Report::Domain(0x010f_31ac),
        0x1d => Report::CopyDomain(0x010f_31ac),
        0x3a => Report::Domain(0x010f_318c),
        0x3d => Report::Domain(0x010f_3194),
        1000 => Report::CopyDomain(0x010f_319c),
        1001 => Report::CopyDomain(0x010f_31a0),
        1002 => Report::CopyDomain(0x010f_31a8),
        1003 => Report::CopyDomain(0x010f_317c),
        1004 => Report::CopyDomain(0x0109_4408),
        1005 => Report::CopyDomain(0x0109_43f0),
        1006 => Report::Domain(0x010f_31ac),
        1007 => Report::CopyDomain(0x010f_3174),
        1008 => Report::Domain(0x010f_318c),
        1009 => Report::Domain(0x010f_3194),
        1010 => Report::ZeroDomain(0x010f_3170),
        1011 => Report::ZeroDomain(0x010f_316c),
        1012 => Report::ZeroDomain(0x010f_3168),
        _ => Report::Nothing,
    }
}

// Translated from 00ed429f (disassembled, FalloutNV.exe 1.4.0.525)
/// `___libm_error_support` (LIBCMT, Ghidra function ID), cdecl: (pointer to argument 1, pointer to
/// argument 2, pointer to the result, error code). The handler is the decoded user `_matherr`
/// (`__decode_pointer` of `01270a68`) when `012704a4` is nonzero, else `00ee0a3a`. The error code
/// picks an `_exception` type and function name ([`libm_report`]); the `_exception` (on the
/// stack: type, name, argument 1, argument 2, result, each double as `FLD`/`FSTP` copy it) goes to
/// the handler, and its result (`FLD`/`FSTP`) becomes the result. Codes it does not know leave
/// everything alone; code 0x1a stores 1.0.
pub fn libm_error_support(e: &mut Engine, argument1: Ptr, argument2: Ptr, result: Ptr, code: u32) {
    let handler = if e.global::<u32>(USER_MATHERR_SET) != 0 {
        let encoded = e.global::<u32>(USER_MATHERR);
        e.call(DECODE_POINTER, &args![encoded]).u32()
    } else {
        DEFAULT_MATHERR
    };
    let (first, second, out) = (argument1.addr(), argument2.addr(), result.addr());
    let (kind, name, errno_code) = match libm_report(code as i32) {
        Report::Nothing => return,
        Report::One => {
            // FLD1; FSTP double ptr [ESI].
            e.mem.set_u64(out, 1.0f64.to_bits());
            return;
        }
        Report::Range(kind, name) => (kind, name, Some(ERANGE)),
        Report::Underflow(name) => (4, name, None),
        Report::Domain(name) => (1, name, Some(EDOM)),
        Report::CopyDomain(name) => {
            // 00ed4434: FLD double ptr [EDI]; FSTP double ptr [ESI].
            let copy = x87_load64(e.mem.u64(first)).to_bits();
            e.mem.set_u64(out, copy);
            (1, name, Some(EDOM))
        }
        Report::ZeroDomain(name) => {
            // 00ed44e0: FLD double ptr [EDI]; FMUL double ptr [EBP-8] (the zeroed local); FST
            // double ptr [ESI].
            let product = x87_mul(x87_load64(e.mem.u64(first)), 0.0).to_bits();
            e.mem.set_u64(out, product);
            (1, name, Some(EDOM))
        }
    };
    // The `_exception` at EBP-0x28: type, name, arg1, arg2, retval.
    e.with_stack(0x28, |e, exception| {
        let at = exception.addr();
        e.mem.set_u32(at, kind);
        e.mem.set_u32(at + 4, name);
        let copy1 = x87_load64(e.mem.u64(first)).to_bits();
        e.mem.set_u64(at + 8, copy1);
        let copy2 = x87_load64(e.mem.u64(second)).to_bits();
        e.mem.set_u64(at + 0x10, copy2);
        // The result: the product still in ST0 for `ZeroDomain` (FSTP of the register), else
        // `FLD double ptr [ESI]`.
        let retval = x87_load64(e.mem.u64(out)).to_bits();
        e.mem.set_u64(at + 0x18, retval);
        let answer = e.call(handler, &args![exception]).u32();
        // TEST EAX, EAX; JNZ 00ed4511, then errno (the underflow path skips the test).
        if let (0, Some(code)) = (answer, errno_code) {
            store_errno(e, code);
        }
        // 00ed4511: FLD double ptr [EBP-0x10]; FSTP double ptr [ESI].
        let back = x87_load64(e.mem.u64(at + 0x18)).to_bits();
        e.mem.set_u64(out, back);
    })
}

/// `__get_sse2_info` (`00ed1927`, Ghidra's label) and the probe it calls, `00ed18d7` (Ghidra
/// gives it no name), called by address as the exe calls them.
const GET_SSE2_INFO: u32 = 0x00ed_1927;
const OS_SSE_PROBE: u32 = 0x00ed_18d7;
/// The machine's answers `__get_sse2_info` reads, each supplied by the platform (the machine
/// table) at the exe address of the instruction that asks the processor; the engine has no
/// processor of its own to ask, so a run without them stops there.
///
/// - `00ed193c` (`PUSHFD; POP EAX; MOV ECX,EAX; XOR EAX,0x200000; PUSH EAX; POPFD; PUSHFD; POP
///   EDX; SUB EDX,ECX`), no arguments: answers in `EAX` the `EDX - ECX` that sequence leaves,
///   the EFLAGS word read back after loading the entry word with bit 21 (`ID`) flipped, minus
///   the entry word. Zero when the `ID` bit cannot be changed (a processor without `CPUID`);
///   a processor with `CPUID` answers `0x0020_0000` (`ID` clear at entry) or `0xffe0_0000`
///   (`ID` set at entry).
/// - `00ed1951` (`CPUID` with `EAX = 0`) and `00ed1964` (`CPUID` with `EAX = 1`), cdecl: the
///   leaf and a pointer to four words the answer is written to, `EAX`, `EBX`, `ECX`, `EDX` in
///   that order (no answer in `EAX`). Of these only leaf 1's `EDX` bit 26 (SSE2) decides
///   anything; the other registers are stored in the frame and never read again.
pub const EFLAGS_ID_TEST: u32 = 0x00ed_193c;
pub const CPUID_LEAF_0: u32 = 0x00ed_1951;
pub const CPUID_LEAF_1: u32 = 0x00ed_1964;
/// `CPUID` leaf 1, `EDX` bit 26: SSE2.
const CPUID_1_EDX_SSE2: u32 = 0x0400_0000;
/// `EXCEPTION_ACCESS_VIOLATION` and `EXCEPTION_ILLEGAL_INSTRUCTION`, the codes the probe's
/// `__except` filter handles.
const STATUS_ACCESS_VIOLATION: u32 = 0xc000_0005;
const STATUS_ILLEGAL_INSTRUCTION: u32 = 0xc000_001d;

// Translated from 00ed454f (disassembled, FalloutNV.exe 1.4.0.525)
/// The C initializer that sets `__use_sse2_mathfcns` (`01270a64`; Ghidra has no function here:
/// the entry of `__cinit`'s `_initterm_e` table at `010113dc`), no arguments: clears the flag
/// (`AND dword ptr [01270a64], 0`), stores `__get_sse2_info()`'s answer (`00ed1927`) there and
/// answers 0 (`XOR EAX, EAX`), so `_initterm_e` goes on.
pub fn fn_00ed454f(e: &mut Engine) -> u32 {
    // AND dword ptr [01270a64], 0: the flag is 0 whatever it held.
    e.set_global(super::USE_SSE2_MATHFCNS, 0u32);
    let available = e.call(GET_SSE2_INFO, &args![]).u32();
    e.set_global(super::USE_SSE2_MATHFCNS, available);
    0
}

/// `__sse2_available` (`01270a6c`; the flag `crt/fpinit.rs`'s `_control87` and
/// `___set_fpsr_sse2` read).
const SSE2_AVAILABLE_FLAG: u32 = 0x0127_0a6c;

// Translated from 00ed1989 (disassembled, FalloutNV.exe 1.4.0.525)
/// The C initializer that sets `__sse2_available` (`01270a6c`; Ghidra has no function here: the
/// entry of `__cinit`'s `_initterm_e` table at `010113d4`, before `00ed454f`'s), no arguments:
/// stores `__get_sse2_info()`'s answer (`00ed1927`) there and answers 0 (`XOR EAX, EAX`), so
/// `_initterm_e` goes on. `__cinit` runs `_fpmath` (`_FPinit`, `__setdefaultprecision`) before
/// the table, so the start-up's `_controlfp_s` still sees the flag clear; from here on
/// `_control87` (and `_controlfp`, `_controlfp_s` through it) also reads `MXCSR` and answers its
/// state with the x87's (`_EM_AMBIGUOUS` when they differ in the shared bits), and a change of
/// its SSE bits goes to `___set_fpsr_sse2`, which loads `MXCSR` (`fpinit.rs`: only `0x1f80` is
/// modelled).
pub fn fn_00ed1989(e: &mut Engine) -> u32 {
    let available = e.call(GET_SSE2_INFO, &args![]).u32();
    e.set_global(SSE2_AVAILABLE_FLAG, available);
    0
}

// Translated from 00ed1927 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__get_sse2_info` (Ghidra's label), cdecl, no arguments: 1 when the processor has SSE2 and
/// the operating system runs SSE instructions, else 0. The frame's words (`EBP-0x18..EBP-4`)
/// start as zero for the three it clears (`EBP-4`, leaf 1's `EDX`; `EBP-0xc`, leaf 0's `EAX`;
/// `EBP-8`, leaf 1's `EAX`). When the `ID` test ([`EFLAGS_ID_TEST`]) shows a change it puts the
/// entry flags back (`PUSH ECX; POPFD`), runs `CPUID` leaf 0 ([`CPUID_LEAF_0`]: `EAX` to
/// `EBP-0xc`, `EBX` to `EBP-0x18`, `EDX` to `EBP-0x14`, `ECX` to `EBP-0x10`) and leaf 1
/// ([`CPUID_LEAF_1`]: `EDX` to `EBP-4`, `EAX` to `EBP-8`); without a change it runs neither.
/// Then: `EBP-4` bit 26 clear gives 0; set, the answer is 1 when the probe `00ed18d7` answers
/// nonzero, else 0.
pub fn fn_00ed1927(e: &mut Engine) -> u32 {
    // SUB ESP, 0x18: the frame at EBP-0x18; the CPUID answer's four words after it (the
    // registers, not the exe's memory).
    e.with_stack(0x28, |e, block| {
        let frame = block.addr();
        let regs = frame + 0x18;
        // XOR EAX, EAX; MOV [EBP-4], EAX; MOV [EBP-0xc], EAX; MOV [EBP-8], EAX.
        e.mem.set_u32(frame + 0x14, 0);
        e.mem.set_u32(frame + 0xc, 0);
        e.mem.set_u32(frame + 0x10, 0);
        // 00ed193c..00ed194b: the ID flag test; JZ 00ed196c when nothing changed.
        let changed = e.call(EFLAGS_ID_TEST, &args![]).u32();
        if changed != 0 {
            // PUSH ECX; POPFD: the entry flags back. XOR EAX, EAX; CPUID.
            e.call(CPUID_LEAF_0, &args![0u32, regs]);
            let (eax, ebx, ecx, edx) = (
                e.mem.u32(regs),
                e.mem.u32(regs + 4),
                e.mem.u32(regs + 8),
                e.mem.u32(regs + 0xc),
            );
            e.mem.set_u32(frame + 0xc, eax);
            e.mem.set_u32(frame, ebx);
            e.mem.set_u32(frame + 4, edx);
            e.mem.set_u32(frame + 8, ecx);
            // MOV EAX, 1; CPUID.
            e.call(CPUID_LEAF_1, &args![1u32, regs]);
            let (eax, edx) = (e.mem.u32(regs), e.mem.u32(regs + 0xc));
            e.mem.set_u32(frame + 0x14, edx);
            e.mem.set_u32(frame + 0x10, eax);
        }
        // TEST dword ptr [EBP-4], 0x4000000; JZ 00ed1984.
        if e.mem.u32(frame + 0x14) & CPUID_1_EDX_SSE2 == 0 {
            return 0;
        }
        // CALL 00ed18d7; TEST EAX, EAX; JZ 00ed1984; else XOR EAX, EAX; INC EAX.
        let probe = e.call(OS_SSE_PROBE, &args![]).u32();
        u32::from(probe != 0)
    })
}

/// The exception code `MOVAPD XMM0, XMM1` raises in the engine: none. The engine runs every
/// SSE instruction (`crate::sse`, `MXCSR` [`super::MXCSR`]) and keeps no `XMM` registers, so
/// the probe's instruction has nothing to change and nothing to raise.
fn movapd_exception() -> Option<u32> {
    None
}

// Translated from 00ed18d7 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed18d7 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated; its __except filter (00ed18f4) and handler (00ed1910) are, run on the exception the probe's instruction raises (none in the engine).
/// The operating-system SSE probe `__get_sse2_info` calls (Ghidra gives it no
/// name), cdecl, no arguments: under `__try` (try level 0) it runs `MOVAPD XMM0,
/// XMM1`, keeps 1 in its result (`EBP-0x1c`) and leaves the try (level `-2`), answering 1. An
/// exception there goes to the filter `00ed18f4` ([`os_sse_probe_filter`]): handled (an access
/// violation or an illegal instruction), the handler `00ed1910` sets the result to 0 and
/// answers it; any other code continues the search for a handler past this function, which the
/// engine does not have, so the run stops. In the engine the instruction raises nothing
/// ([`movapd_exception`]), so those two paths are not taken.
pub fn fn_00ed18d7(_e: &mut Engine) -> u32 {
    // AND [EBP-4], 0 (try level 0); MOVAPD XMM0, XMM1.
    match movapd_exception() {
        // MOV [EBP-0x1c], 1; JMP 00ed1917: try level -2, MOV EAX, [EBP-0x1c].
        None => 1,
        Some(code) => {
            if os_sse_probe_filter(code) != 0 {
                // 00ed1910: MOV ESP, [EBP-0x18]; AND [EBP-0x1c], 0; then 00ed1917.
                0
            } else {
                panic!(
                    "00ed18d7: exception {code:08x} in the SSE probe continues the search for \
                     a handler past it (EXCEPTION_CONTINUE_SEARCH), which the engine does not have"
                )
            }
        }
    }
}

/// The `__except` filter of the probe (`00ed18f4`, part of `00ed18d7`): reads the exception
/// code (`[[EBP-0x14]]` is the `EXCEPTION_RECORD`, its first word the code) and answers 1
/// (`EXCEPTION_EXECUTE_HANDLER`) for `0xc0000005` and `0xc000001d`, else 0
/// (`EXCEPTION_CONTINUE_SEARCH`).
fn os_sse_probe_filter(code: u32) -> u32 {
    // CMP EAX, 0xc0000005; JZ 00ed190c; CMP EAX, 0xc000001d; JZ 00ed190c; XOR EAX, EAX.
    u32::from(code == STATUS_ACCESS_VIOLATION || code == STATUS_ILLEGAL_INSTRUCTION)
}

// ---- The x87 machinery the math bodies share ---------------------------------------------------
//
// The x87 bodies below keep the register stack as the FPU holds it ([`Fpu`], extended values),
// so every instruction rounds as the exe's does: `FADD`/`FSUB`/`FMUL`/`FDIV`/`FSQRT` through
// `crate::x87`'s register arithmetic (the running thread's precision control and rounding mode),
// the transcendental instructions through their CPU-exact helpers. The helpers that pass values
// in registers (`00ed2780`, `00ec9d22`, the exits) are called directly with the stack; each is
// also registered in the uniform form (register inputs `EAX`, `ECX`, `EDX` first, then `ST0` as
// a double, then the stack words from the one at `[ESP]` after the call on). A body's result
// leaves through an exit in `ST0`; its registered form gives that value rounded to a double
// with the control word the exit leaves (the caller's `FSTP`).
//
// Not modelled: the x87 status word's exception flags, which the exits read (`FSTSW`, the
// precision flag 0x20): the engine keeps no status word; the flag is taken as set, as
// `trig.rs`'s `__math_exit` takes it (each such function says so). The `_CI` entries take
// their register arguments as `FLD qword` leaves a double (a signalling NaN quiet); the
// registered forms of the internal helpers take the register exactly (a signalling NaN kept,
// as `__fload_withFB` loads one). The register stack of the caller below the values a body
// works on is untouched, as `FSAVE`/`FRSTOR` keep it.

use crate::x87::{
    x87_fabs, x87_fadd, x87_fchs, x87_fcom, x87_fdiv, x87_fld_constant, x87_fmul, x87_fsqrt,
    x87_fsub, X87Constant,
};

/// The x87 register stack of one routine, `ST0` last.
#[derive(Default)]
pub(super) struct Fpu {
    pub(super) st: Vec<X87Ext>,
}

impl Fpu {
    /// The stack with `ST0` = `value`.
    pub(super) fn with(value: X87Ext) -> Self {
        Fpu { st: vec![value] }
    }

    /// `FLD`.
    pub(super) fn push(&mut self, value: X87Ext) {
        self.st.push(value);
    }

    /// `FSTP ST0`: the value popped.
    pub(super) fn pop(&mut self) -> X87Ext {
        self.st.pop().expect("x87 stack underflow")
    }

    /// `ST(i)`.
    pub(super) fn st(&self, i: usize) -> X87Ext {
        self.st[self.st.len() - 1 - i]
    }

    /// `ST(i)` replaced.
    pub(super) fn set(&mut self, i: usize, value: X87Ext) {
        let at = self.st.len() - 1 - i;
        self.st[at] = value;
    }

    /// `FXCH ST(i)`.
    pub(super) fn fxch(&mut self, i: usize) {
        let top = self.st.len() - 1;
        self.st.swap(top, top - i);
    }

    /// `FSTP ST(i)`: `ST(i)` = `ST0`, then pop.
    pub(super) fn fstp(&mut self, i: usize) {
        let top = self.pop();
        if i > 0 {
            self.set(i - 1, top);
        }
    }
}

/// The status word's condition codes in `AH` after `FNSTSW AX` (`C0` 0x01, `C2` 0x04, `C3`
/// 0x40), as `TEST AH, ...` and `SAHF` read them.
fn condition_ah(status: u16) -> u8 {
    (status >> 8) as u8 & 0x45
}

/// `FCOM`'s condition codes as `AH` holds them, for `ST0` = `a` and the operand `b`.
fn fcom_ah(a: X87Ext, b: X87Ext) -> u8 {
    condition_ah(x87_fcom(a, b))
}

/// `TEST AH, mask` followed by `JP`: taken when the masked bits hold an even number of ones (the
/// parity flag). After `FCOM`, `TEST AH, 0x44` is not taken exactly for equal operands and
/// `TEST AH, 5` exactly for `ST0` below the operand.
fn jp_taken(ah: u8, mask: u8) -> bool {
    (ah & mask).count_ones() & 1 == 0
}

/// The extended value of a double's bit pattern, exact, a signalling NaN still signalling: what
/// `__fload_withFB` (`00ed27c5`) builds by hand for an exponent of all ones (sign and exponent
/// `0x7fff`, the significand shifted up by 11 with the integer bit, which is the lowest exponent
/// bit), and what `FLD qword` loads for every other value.
pub(super) fn ext_of_double(bits: u64) -> X87Ext {
    if bits & 0x7ff0_0000_0000_0000 != 0x7ff0_0000_0000_0000 {
        return X87Ext::load64(f64::from_bits(bits));
    }
    let (high, low) = ((bits >> 32) as u32, bits as u32);
    // OR EAX, 0x7fff0000: the top word is the sign and 0x7fff.
    let sign_exponent = ((high | 0x7fff_0000) >> 16) as u16;
    // SHLD EAX, ECX, 0xb; SHL ECX, 0xb.
    let top = (high << 11) | (low >> 21);
    X87Ext::from_parts(sign_exponent, (u64::from(top) << 32) | u64::from(low << 11))
}

/// `FLD tbyte` of the extended constant at `at`.
pub(super) fn load_extended(e: &Engine, at: u32) -> X87Ext {
    let mut bytes = [0u8; 10];
    bytes.copy_from_slice(&e.mem.bytes(at, 10));
    X87Ext(bytes)
}

/// `FLD qword` of the double at `at`.
fn load_double(e: &Engine, at: u32) -> X87Ext {
    X87Ext::load64(f64::from_bits(e.mem.u64(at)))
}

/// A double memory operand of an arithmetic instruction (`FMUL qword ptr [..]` and the like),
/// widened exactly (a signalling NaN still signalling).
fn double_operand(e: &Engine, at: u32) -> X87Ext {
    ext_of_double(e.mem.u64(at))
}

/// `__fload_withFB` (`00ed27c5`, in `crt.rs`), fastcall: `EDX` = the double's address.
const FLOAD_WITH_FB: u32 = 0x00ed_27c5;

/// `CALL 00ed27c5` with `EDX` = `at`: the double pushed as the helper loads it (a signalling NaN
/// kept), `EAX` as it leaves it, and its `ZF` (set exactly for an exponent of all ones).
fn fload_with_fb_at(e: &mut Engine, fpu: &mut Fpu, at: u32) -> (u32, bool) {
    let r = e.call(FLOAD_WITH_FB, &args![0u32, at]);
    let bits = e.mem.u64(at);
    fpu.push(ext_of_double(bits));
    let zf = (bits >> 32) as u32 & 0x7ff0_0000 == 0x7ff0_0000;
    (r.eax, zf)
}

/// `__87except` (`00ee1b04`, cdecl: operation, `_exception` pointer, control word pointer).
const EXCEPT87: u32 = 0x00ee_1b04;
/// The math-error switch (`0126fd90`; `crt.rs` has it too): nonzero leaves through the fast
/// exit without any error handling.
const ERROR_SWITCH: u32 = 0x0126_fd90;
/// `00ed2869`'s scaling for a result that underflowed (`010f2abc`, the bound `010f2aac`, the
/// factor `010f2acc`) and overflowed (`010f2ab4`, `010f2aa4`, `010f2ac4`).
const UNDERFLOW_SCALE: u32 = 0x010f_2abc;
const UNDERFLOW_BOUND: u32 = 0x010f_2aac;
const UNDERFLOW_FACTOR: u32 = 0x010f_2acc;
const OVERFLOW_SCALE: u32 = 0x010f_2ab4;
const OVERFLOW_BOUND: u32 = 0x010f_2aa4;
const OVERFLOW_FACTOR: u32 = 0x010f_2ac4;
/// The operation code `pow` passes the error handling: `00ed2869` and the error paths use the
/// two-argument handler `00ed2720` for it.
const POW_OPERATION: u32 = 0x1d;

/// The precision flag (0x20) of the x87 status word as `FSTSW` reads it at an exit. The engine
/// keeps no status word; the flag is sticky and set by every inexact x87 result, so it is taken
/// as set, as `trig.rs`'s `__math_exit` takes it.
fn status_precision_flag() -> bool {
    true
}

// Translated from 00ed2795 (disassembled, FalloutNV.exe 1.4.0.525)
/// The math bodies' control-word load (no label; Ghidra gives it no name), called with the
/// caller's saved control word in the stack slot above the return address: loads `(saved & 0x300)
/// | 0x7f` (the caller's precision, round to nearest, every exception masked) with `FLDCW`. The
/// word is built in the upper half of that slot (`MOV word ptr [ESP+6], DX`), which nothing
/// reads again; the saved word in the lower half stays.
pub fn fn_00ed2795(e: &mut Engine, slot: u32) {
    let word = (slot & 0x300) | 0x7f;
    set_x87_control_word(e, word as u16);
}

/// The control word a body saved (`PUSH`, `FSTCW`) and the one it then runs under: `00ed2795`
/// unless the saved word is `0x027f`.
fn enter_body(e: &mut Engine) -> u16 {
    let saved = x87_control_word(e);
    if saved != 0x027f {
        fn_00ed2795(e, u32::from(saved));
    }
    saved
}

// Translated from 00ed2780 (disassembled, FalloutNV.exe 1.4.0.525)
/// `2^ST0` (no label; Ghidra gives it no name), register in, register out: the integer part by
/// `FRNDINT` (the control word's rounding), the fraction as `-(n - x)` (`FSUBR`, `FCHS`),
/// `F2XM1` of it plus 1 (`FLD1`, `FADDP`), scaled by `FSCALE` with `n`.
fn two_to_st0(fpu: &mut Fpu) {
    // FLD ST0; FRNDINT.
    let x = fpu.st(0);
    fpu.push(x87_frndint(x));
    // FSUBR ST1, ST0: ST1 = n - x.
    let n = fpu.st(0);
    fpu.set(1, x87_fsub(n, fpu.st(1)));
    // FXCH; FCHS; F2XM1.
    fpu.fxch(1);
    let fraction = x87_fchs(fpu.st(0));
    fpu.set(0, x87_f2xm1(fraction));
    // FLD1; FADDP: ST1 = ST1 + ST0.
    let one = X87Ext::load64(1.0);
    let sum = x87_fadd(fpu.st(0), one);
    fpu.set(0, sum);
    // FSCALE; FSTP ST1.
    let scaled = x87_fscale(fpu.st(0), fpu.st(1));
    fpu.set(0, scaled);
    fpu.fstp(1);
}

/// The registered form of `00ed2780`: `ST0` in, `ST0` out (the double given is the register).
pub fn fn_00ed2780(_e: &mut Engine, st0: f64) -> f64 {
    let mut fpu = Fpu::with(ext_of_double(st0.to_bits()));
    two_to_st0(&mut fpu);
    fpu.pop().store64()
}

// Translated from 00ed281e (disassembled, FalloutNV.exe 1.4.0.525)
/// The bodies' fast exit (no label; Ghidra gives it no name), reached by a jump with the saved
/// control word on the stack: loads it back unless it is `0x027f`, pops it and returns `ST0` to
/// the body's caller.
fn fast_exit(e: &mut Engine, saved: u16) {
    if saved != 0x027f {
        set_x87_control_word(e, saved);
    }
}

/// The registered form of `00ed281e`: `ST0` and the saved control word.
pub fn fn_00ed281e(e: &mut Engine, st0: f64, saved: u32) -> f64 {
    fast_exit(e, saved as u16);
    st0
}

// Translated from 00ed2869 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed2869 - the x87 status word's precision flag (FSTSW) is not modelled; it is taken as set.
/// The exit that checks the result's range (no label; Ghidra gives it no name), reached by a
/// jump with `EDX` = the operation, `ECX` = the name and the saved control word on the stack.
/// `ST0` rounded to a double (`FST`): an exponent of zero (a zero or a denormal result) is scaled
/// by `FSCALE` with `010f2abc` and, unless its magnitude reaches `010f2aac`, multiplied by
/// `010f2acc` (`FMUL`), and reported as error type 4 (underflow); an exponent of all ones is
/// scaled with `010f2ab4` and, when its magnitude is above `010f2aa4` (or unordered), multiplied
/// by `010f2ac4`, and reported as type 3 (overflow). Any other result: `0x027f` returns it as it
/// is; with the precision exception masked in the saved word (bit 5), or unmasked but no flag
/// raised, the word is loaded back and it is returned; unmasked and raised, it is reported as
/// type 8. The report is `00ed2720` (two arguments) for the operation 0x1d, else
/// `__startOneArgErrorHandling` (`00ed2737`, `trig.rs`).
fn range_exit(
    e: &mut Engine,
    fpu: &mut Fpu,
    operation: u32,
    name: u32,
    saved: u16,
    first: f64,
    second: f64,
) -> X87Ext {
    // SUB ESP, 8; FST double ptr [ESP]; the high word's exponent.
    let exponent = (fpu.st(0).store64().to_bits() >> 32) as u32 & 0x7ff0_0000;
    let kind = if exponent == 0 {
        // 00ed28ba: FLD [010f2abc]; FXCH; FSCALE; FSTP ST1.
        let scaled = x87_fscale(fpu.st(0), load_double(e, UNDERFLOW_SCALE));
        fpu.set(0, scaled);
        // FLD ST0; FABS; FCOMP [010f2aac]; SAHF; JNC: C0 clear (at or above, or never: unordered
        // sets C0).
        let ah = fcom_ah(x87_fabs(scaled), double_operand(e, UNDERFLOW_BOUND));
        if ah & 0x01 != 0 {
            let product = x87_fmul(scaled, double_operand(e, UNDERFLOW_FACTOR));
            fpu.set(0, product);
        }
        4
    } else if exponent == 0x7ff0_0000 {
        // 00ed28e3
        let scaled = x87_fscale(fpu.st(0), load_double(e, OVERFLOW_SCALE));
        fpu.set(0, scaled);
        // JBE: C0 or C3 (below, equal or unordered).
        let ah = fcom_ah(x87_fabs(scaled), double_operand(e, OVERFLOW_BOUND));
        if ah & 0x41 == 0 {
            let product = x87_fmul(scaled, double_operand(e, OVERFLOW_FACTOR));
            fpu.set(0, product);
        }
        3
    } else {
        if saved == 0x027f {
            return fpu.pop();
        }
        if saved & 0x20 != 0 || !status_precision_flag() {
            // 00ed28b5
            set_x87_control_word(e, saved);
            return fpu.pop();
        }
        8
    };
    // 00ed28a2
    let st0 = fpu.pop().store64();
    let r = if operation == POW_OPERATION {
        start_two_arg_error_handling(e, kind, name, operation, st0, saved, first, second)
    } else {
        let bits = first.to_bits();
        super::trig::start_one_arg_error_handling(
            e,
            kind,
            name,
            operation,
            st0,
            u32::from(saved),
            0,
            bits as u32,
            (bits >> 32) as u32,
        )
    };
    X87Ext::load64(r)
}

/// The registered form of `00ed2869`: `ECX`, `EDX`, `ST0`, then the saved control word, the body
/// caller's return address and the two argument doubles (the second is read only for the
/// operation 0x1d).
#[allow(clippy::too_many_arguments)]
pub fn fn_00ed2869(
    e: &mut Engine,
    name: u32,
    operation: u32,
    st0: f64,
    saved: u32,
    _return_address: u32,
    first: f64,
    second: f64,
) -> f64 {
    let mut fpu = Fpu::with(ext_of_double(st0.to_bits()));
    range_exit(e, &mut fpu, operation, name, saved as u16, first, second).store64()
}

/// The two-argument form of `__startOneArgErrorHandling` (`00ed2720`, joining `00ed2737`'s code at
/// `00ed2740`): the `_exception` on the frame (type, name, argument 1, argument 2, result),
/// `__87except(operation, &exception, &saved)`, then the result as it leaves it (`FLD`), and the
/// saved control word loaded back unless it is `0x027f`. `result` is `ST0` as `FSTP double`
/// stored it.
#[allow(clippy::too_many_arguments)]
fn start_two_arg_error_handling(
    e: &mut Engine,
    kind: u32,
    name: u32,
    operation: u32,
    result: f64,
    saved: u16,
    first: f64,
    second: f64,
) -> f64 {
    // The frame: the `_exception` at EBP-0x20 and the saved control word's slot at EBP+8.
    e.with_stack(0x24, |e, frame| {
        let exception = frame.addr();
        let slot = exception + 0x20;
        e.mem.set_u16(slot, saved);
        // MOV [EBP-0x20], EAX; FSTP double ptr [EBP-8]; MOV [EBP-0x1c], ECX.
        e.mem.set_u32(exception, kind);
        e.mem
            .set_u64(exception + 0x18, x87_load64(result.to_bits()).to_bits());
        e.mem.set_u32(exception + 4, name);
        e.mem.set_u64(exception + 8, first.to_bits());
        e.mem.set_u64(exception + 0x10, second.to_bits());
        e.call(EXCEPT87, &args![operation, exception, slot]);
        // FLD double ptr [EBP-8]; CMP word ptr [EBP+8], 0x27f; FLDCW.
        let back = x87_load64(e.mem.u64(exception + 0x18));
        let word = e.mem.u16(slot);
        if word != 0x027f {
            set_x87_control_word(e, word);
        }
        back
    })
}

// Translated from 00ed2720 (disassembled, FalloutNV.exe 1.4.0.525)
/// The two-argument form of `__startOneArgErrorHandling` (no label; Ghidra gives it no name):
/// the same inputs plus the second argument after the first, which it copies into the
/// `_exception` (`[EBP+0x18]`, `[EBP+0x1c]`) before joining `00ed2740`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ed2720(
    e: &mut Engine,
    kind: u32,
    name: u32,
    operation: u32,
    st0: f64,
    saved: u32,
    _return_address: u32,
    first: f64,
    second: f64,
) -> f64 {
    start_two_arg_error_handling(e, kind, name, operation, st0, saved as u16, first, second)
}

// ---- pow ---------------------------------------------------------------------------------------

/// `pow`'s SSE2 bodies: `00ed8790` (register arguments, `_CIpow`) and `00ed87a9` (stack
/// arguments).
const POW_SSE2_REGISTERS: u32 = 0x00ed_8790;
const POW_SSE2: u32 = 0x00ed_87a9;
/// `pow`'s name for the error reports (`011c1948`), the 0.5 its integer test multiplies by
/// (`011c1940`), the extended infinity of a zero base's negative power (`011c2730`) and the
/// indefinite NaN of a domain error (`011c1d00`).
const POW_NAME: u32 = 0x011c_1948;
const POW_HALF: u32 = 0x011c_1940;
const POW_POLE: u32 = 0x011c_2730;
const INDEFINITE: u32 = 0x011c_1d00;
/// `pow`'s special-value helper (`00ed955d`, cdecl: base, exponent, pointer to the result;
/// nonzero answers a domain error) and `__d_inttype` (`00ed94f7`, cdecl, the double: 0 not an
/// integer, 1 odd, 2 even), `__fpclass` (`00ec75e5`).
const POW_SPECIAL: u32 = 0x00ed_955d;
const D_INTTYPE: u32 = 0x00ed_94f7;
const FPCLASS: u32 = 0x00ec_75e5;
/// The doubles `00ed955d` stores: `011c24f8` (+infinity in the exe), `011c2500` (the result of
/// a power of one to -infinity) and `011c2518` (the result of -infinity to a negative odd power).
const POW_SPECIAL_INFINITY: u32 = 0x011c_24f8;
const POW_SPECIAL_ONE_TO_MINUS_INFINITY: u32 = 0x011c_2500;
const POW_SPECIAL_MINUS_ZERO: u32 = 0x011c_2518;
/// The 0.5 `__d_inttype` multiplies by (`01011588`).
const INTTYPE_HALF: u32 = 0x0101_1588;
/// The control word `FNINIT` leaves (`FSAVE` stores the state, then initializes the FPU).
const FNINIT_CONTROL_WORD: u16 = 0x037f;

// Translated from 00ec9ac0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `pow` (LIBCMT; Ghidra's function ID, the `"pow"` its error reports pass at `011c1948`), cdecl,
/// the base then the exponent as doubles on the stack, the result in `ST0`. With
/// `__use_sse2_mathfcns` (`01270a64`) set and every `MXCSR` and x87 exception masked (the
/// `STMXCSR`/`FNSTCW` tests of `floor`, `super::sse2_math_available`) it jumps to the SSE2 body
/// `00ed87a9` with its own stack. Otherwise it loads the exponent with `__fload_withFB`
/// (`00ed27c5`, `LEA EDX, [ESP+0xc]`) and falls into the x87 body `00ec9b5d` with that `EAX`.
pub fn pow(e: &mut Engine, x: f64, y: f64) -> f64 {
    if super::sse2_math_available(e) {
        return e.call(POW_SSE2, &args![x, y]).f64();
    }
    e.with_stack(0x10, |e, slot| {
        let at = slot.addr();
        e.mem.set_u64(at, x.to_bits());
        e.mem.set_u64(at + 8, y.to_bits());
        let mut fpu = Fpu::default();
        let (eax, _) = fload_with_fb_at(e, &mut fpu, at + 8);
        pow_x87(e, &mut fpu, eax, at).store64()
    })
}

// Translated from 00ec9b00 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_CIpow` (the register form of `pow`; no label in the map, named from its body, which is
/// `pow`'s), base in `ST1`, exponent in `ST0`, the result in `ST0`; registered with the base
/// first. The same test as [`pow`] picks the SSE2 body `00ed8790`. Otherwise it stores the base
/// (`FXCH`, `FSTP`) and a copy of the exponent (`FST`) to a 0x14-byte frame, takes the exponent's
/// high word into `EAX` and calls the x87 body `00ec9b5d` with the exponent still in `ST0`.
pub fn ci_pow(e: &mut Engine, x: f64, y: f64) -> f64 {
    if super::sse2_math_available(e) {
        return e.call(POW_SSE2_REGISTERS, &args![x, y]).f64();
    }
    e.with_stack(0x10, |e, slot| {
        let at = slot.addr();
        // The registers as `FLD qword` leaves a double (a signalling NaN quiet); FXCH; FSTP double
        // ptr [ESP]; FST double ptr [ESP+8].
        let base = X87Ext::load64(x).store64();
        let exponent = X87Ext::load64(y);
        e.mem.set_u64(at, base.to_bits());
        e.mem.set_u64(at + 8, exponent.store64().to_bits());
        // MOV EAX, dword ptr [ESP+0xc].
        let eax = e.mem.u32(at + 0xc);
        let mut fpu = Fpu::with(exponent);
        pow_x87(e, &mut fpu, eax, at).store64()
    })
}

/// Where `pow`'s x87 body goes next (its labels).
#[derive(Clone, Copy)]
enum PowStep {
    /// `00ec9b9d`: a finite, nonzero base; `CL` from its sign.
    Finite,
    /// `00ec9baa`: `FYL2X` and `2^x`, with `CL` = 1 for an odd integer exponent.
    Log(u8),
    /// `00ec9bd5`: an error of type `EAX`.
    Error(u32),
    /// `00ec9bf4`: a NaN exponent.
    NanExponent,
    /// `00ec9c07`: `FADDP`, error type 1.
    AddNan,
    /// `00ec9c10`: a NaN base.
    NanBase,
    /// `00ec9c17`: `FADDP`, error type 7.
    AddNanSignalling,
    /// `00ec9c3a`: an infinite argument, with `ECX`.
    Special(u32),
    /// `00ec9c93`: a base with a zero exponent field.
    ZeroExponent,
    /// `00ec9cfd`: a negative base.
    Negative,
}

// Translated from 00ec9b5d (disassembled, FalloutNV.exe 1.4.0.525)
/// `pow`'s x87 body (no label; Ghidra gives it no name), reached from [`pow`] and [`ci_pow`]
/// with the exponent in `ST0`, `EAX` its high word (or what `__fload_withFB` left) and the stack
/// holding the base and the exponent above the return address (`at`: the base, `at + 8` the
/// exponent). It saves the control word and runs under `00ed2795`'s unless it was `0x027f`.
///
/// - A finite exponent and a positive finite base: `FYL2X` and `2^x` (`00ed2780`), leaving through
///   the range exit `00ed2869` (operation 0x1d, `"pow"`), or the fast exit `00ed281e` when the
///   math-error switch (`0126fd90`) is set.
/// - A negative finite base: an integer exponent (`00ec9d22`) takes the power of `|x|`, negated
///   for an odd one; any other exponent is a domain error with the indefinite NaN (type 1).
/// - A zero base (zero exponent field and fraction): a zero exponent gives 1.0; a positive one
///   +-0, a negative one the extended infinity `011c2730` with error type 2, negative for a
///   negative zero to an odd power. A denormal base takes the finite path.
/// - A NaN exponent or base: `FADDP` of the two (the x87's NaN), error type 1 for a quiet one and
///   7 for a signalling one (base checked after the exponent); a quiet NaN exponent with a
///   signalling base reports 7.
/// - An infinite argument otherwise: `00ed955d` on the two doubles under `FSAVE` (so with the
///   control word `FNINIT` leaves), its result loaded after `FRSTOR`; a nonzero answer is error
///   type 1.
///
/// Errors leave through `00ed2720` with operation 0x1d unless the math-error switch is set.
pub fn fn_00ec9b5d(
    e: &mut Engine,
    eax: u32,
    st0: f64,
    _return_address: u32,
    x: f64,
    y: f64,
) -> f64 {
    e.with_stack(0x10, |e, slot| {
        let at = slot.addr();
        e.mem.set_u64(at, x.to_bits());
        e.mem.set_u64(at + 8, y.to_bits());
        let mut fpu = Fpu::with(ext_of_double(st0.to_bits()));
        pow_x87(e, &mut fpu, eax, at).store64()
    })
}

/// The body of [`fn_00ec9b5d`] on the register stack, the base at `at` and the exponent at
/// `at + 8`; the result is `ST0` as the exit leaves it.
fn pow_x87(e: &mut Engine, fpu: &mut Fpu, eax: u32, at: u32) -> X87Ext {
    let x_low = e.mem.u32(at);
    let x_high = e.mem.u32(at + 4);
    let y_low = e.mem.u32(at + 8);
    let y_high = e.mem.u32(at + 0xc);
    let (x, y) = (
        f64::from_bits(e.mem.u64(at)),
        f64::from_bits(e.mem.u64(at + 8)),
    );
    // MOV ECX, EAX; PUSH EAX; FSTCW [ESP]; 00ed2795 unless 0x27f.
    let saved = enter_body(e);
    let mut ecx = eax;
    let mut eax = eax;
    // AND ECX, 0x7ff00000; CMP ECX, 0x7ff00000; JZ 00ec9c24.
    let mut step = if ecx & 0x7ff0_0000 == 0x7ff0_0000 {
        // 00ec9c24: XOR ECX, ECX; AND EAX, 0xfffff; OR EAX, [ESP+0x10].
        ecx = 0;
        if (eax & 0xf_ffff) | y_low != 0 {
            PowStep::NanExponent
        } else {
            let (_, _) = fload_with_fb_at(e, fpu, at);
            PowStep::Special(ecx)
        }
    } else {
        let (loaded, zf) = fload_with_fb_at(e, fpu, at);
        eax = loaded;
        if zf {
            // 00ec9c20
            PowStep::Special(0)
        } else if eax & 0x7ff0_0000 == 0 {
            PowStep::ZeroExponent
        } else {
            PowStep::Finite
        }
    };
    loop {
        step = match step {
            PowStep::Finite => {
                // MOV CL, byte ptr [ESP+0xf]; AND CL, 0x80.
                if x_high & 0x8000_0000 != 0 {
                    PowStep::Negative
                } else {
                    PowStep::Log(0)
                }
            }
            PowStep::Log(cl) => {
                // FYL2X: ST1 * log2(ST0), popped.
                let base = fpu.pop();
                let product = x87_fyl2x(fpu.st(0), base);
                fpu.set(0, product);
                two_to_st0(fpu);
                if cl == 1 {
                    let negated = x87_fchs(fpu.st(0));
                    fpu.set(0, negated);
                }
                // 00ec9bb8
                if e.global::<u32>(ERROR_SWITCH) != 0 {
                    fast_exit(e, saved);
                    return fpu.pop();
                }
                return range_exit(e, fpu, POW_OPERATION, POW_NAME, saved, x, y);
            }
            PowStep::Error(kind) => {
                // 00ec9bd5
                if e.global::<u32>(ERROR_SWITCH) != 0 {
                    fast_exit(e, saved);
                    return fpu.pop();
                }
                let st0 = fpu.pop().store64();
                let r = start_two_arg_error_handling(
                    e,
                    kind,
                    POW_NAME,
                    POW_OPERATION,
                    st0,
                    saved,
                    x,
                    y,
                );
                return X87Ext::load64(r);
            }
            PowStep::NanExponent => {
                // 00ec9bf4: the base loaded; TEST byte ptr [ESP+0x16], 8: the exponent's quiet
                // bit.
                let (_, _) = fload_with_fb_at(e, fpu, at);
                if y_high & 0x8_0000 != 0 {
                    PowStep::AddNan
                } else {
                    ecx += 1;
                    PowStep::Special(ecx)
                }
            }
            PowStep::AddNan => {
                // 00ec9c07: FADDP (ST1 + ST0), EAX = 1.
                let top = fpu.pop();
                let sum = x87_fadd(fpu.st(0), top);
                fpu.set(0, sum);
                PowStep::Error(1)
            }
            PowStep::NanBase => {
                // 00ec9c10: TEST byte ptr [ESP+0xe], 8: the base's quiet bit.
                if x_high & 0x8_0000 != 0 {
                    PowStep::AddNan
                } else {
                    PowStep::AddNanSignalling
                }
            }
            PowStep::AddNanSignalling => {
                // 00ec9c17
                let top = fpu.pop();
                let sum = x87_fadd(fpu.st(0), top);
                fpu.set(0, sum);
                PowStep::Error(7)
            }
            PowStep::Special(ecx) => {
                // 00ec9c3a: a NaN base first.
                if x_high & 0x7ff0_0000 == 0x7ff0_0000 && ((x_high & 0xf_ffff) | x_low) != 0 {
                    PowStep::NanBase
                } else if ecx != 0 {
                    PowStep::AddNanSignalling
                } else {
                    // 00ec9c5c: FSTP the base and the exponent to the call's arguments, FSAVE.
                    let base = fpu.pop().store64();
                    let exponent = fpu.pop().store64();
                    let (answer, result) = e.with_stack(8, |e, result| {
                        let control = x87_control_word(e);
                        set_x87_control_word(e, FNINIT_CONTROL_WORD);
                        let answer = e.call(POW_SPECIAL, &args![base, exponent, result]).u32();
                        // FRSTOR [ECX+8]; FLD double ptr [ECX].
                        set_x87_control_word(e, control);
                        (answer, e.mem.u64(result.addr()))
                    });
                    fpu.push(X87Ext::load64(f64::from_bits(result)));
                    if answer == 0 {
                        fast_exit(e, saved);
                        return fpu.pop();
                    }
                    PowStep::Error(1)
                }
            }
            PowStep::ZeroExponent => {
                // 00ec9c93: a denormal base takes the finite path.
                if (x_high & 0xf_ffff) | x_low != 0 {
                    PowStep::Finite
                } else {
                    // FSTP ST0: the base off.
                    fpu.pop();
                    if (y_high & 0x7fff_ffff) | y_low == 0 {
                        // 00ec9cf4: FSTP ST0; FLD1.
                        fpu.pop();
                        fpu.push(X87Ext::load64(1.0));
                        fast_exit(e, saved);
                        return fpu.pop();
                    }
                    let cl = integer_kind(e, fpu);
                    // MOV CH, byte ptr [ESP+0xf]; SHR CH, 7: the base's sign.
                    let ch = (x_high >> 31) as u8;
                    // TEST dword ptr [ESP+0x17], 0x80: the exponent's sign.
                    if y_high & 0x8000_0000 != 0 {
                        let mut pole = load_extended(e, POW_POLE);
                        if ch & cl != 0 {
                            pole = x87_fchs(pole);
                        }
                        fpu.push(pole);
                        PowStep::Error(2)
                    } else {
                        // 00ec9ce3: FLDZ, negated for a negative zero to an odd power.
                        let mut zero = X87Ext::load64(0.0);
                        if ch & cl != 0 {
                            zero = x87_fchs(zero);
                        }
                        fpu.push(zero);
                        fast_exit(e, saved);
                        return fpu.pop();
                    }
                }
            }
            PowStep::Negative => {
                // 00ec9cfd: FLD ST1; the exponent's kind; FCHS of the base.
                let exponent = fpu.st(1);
                fpu.push(exponent);
                let cl = integer_kind(e, fpu);
                let absolute = x87_fchs(fpu.st(0));
                fpu.set(0, absolute);
                if cl != 0 {
                    PowStep::Log(cl)
                } else {
                    // 00ec9d0e: both off; the indefinite NaN, type 1.
                    fpu.pop();
                    fpu.pop();
                    fpu.push(load_extended(e, INDEFINITE));
                    PowStep::Error(1)
                }
            }
        };
    }
}

// Translated from 00ec9d22 (disassembled, FalloutNV.exe 1.4.0.525)
/// Whether `ST0` is an integer (no label; Ghidra gives it no name), register in, `CL` out:
/// `FRNDINT` of a copy compared with it (`FCOMP`, `SAHF`, `JNZ` on `ZF` = C3): not equal gives 0
/// and pops `ST0`; equal multiplies it by 0.5 (`011c1940`), and `FRNDINT` of that compared with
/// it (`FCOMPP`, popping both) gives 2 when equal (even), else 1 (odd). `ST0` is consumed
/// either way. An unordered compare sets C3, so a NaN counts as an even integer.
fn integer_kind(e: &Engine, fpu: &mut Fpu) -> u8 {
    let value = fpu.st(0);
    let rounded = x87_frndint(value);
    // FCOMP: ST0 = the rounded copy, ST1 = the value.
    let ah = fcom_ah(rounded, value);
    if ah & 0x40 == 0 {
        // 00ec9d47: FSTP ST0.
        fpu.pop();
        return 0;
    }
    let half = x87_fmul(value, double_operand(e, POW_HALF));
    fpu.set(0, half);
    let rounded = x87_frndint(half);
    let ah = fcom_ah(rounded, half);
    fpu.pop();
    if ah & 0x40 != 0 {
        2
    } else {
        1
    }
}

/// The registered form of `00ec9d22`: `ST0` in, `CL` in `EAX`'s low byte (the rest of `ECX` is
/// the caller's).
pub fn fn_00ec9d22(e: &mut Engine, st0: f64) -> u32 {
    let mut fpu = Fpu::with(ext_of_double(st0.to_bits()));
    u32::from(integer_kind(e, &mut fpu))
}

// Translated from 00ed955d (disassembled, FalloutNV.exe 1.4.0.525)
/// `pow`'s special-value helper (no label; Ghidra gives it no name), cdecl: (base, exponent,
/// pointer to the result), answering 0 or 1 (a domain error). The comparisons run on the x87
/// (`FCOM`, `FNSTSW`) on `|x|` (`FCHS` unless `0` compares below or equal to `x`). An exponent
/// of +infinity stores +infinity (`011c24f8`) for `|x| > 1`, 0 below 1 and 1 at
/// 1; -infinity stores 0 above 1, the infinity below 1, and at 1 the double `011c2500` with the
/// answer 1. Otherwise a base of +infinity (its low word zero) stores the infinity for a
/// positive exponent, 0 for a negative one and 1 for a zero (or unordered) one; -infinity stores
/// for a positive exponent the infinity (negated for an odd integer, `__d_inttype`), for a zero
/// one 1, for a negative one -0 (`011c2518`) for an odd integer and +0 otherwise. Any other pair
/// stores nothing (a +infinity high word with a nonzero low word included).
pub fn fn_00ed955d(e: &mut Engine, x: f64, y: f64, result: Ptr) -> u32 {
    let out = result.addr();
    let (x_bits, y_bits) = (x.to_bits(), y.to_bits());
    let (x_low, x_high) = (x_bits as u32, (x_bits >> 32) as u32);
    let (y_low, y_high) = (y_bits as u32, (y_bits >> 32) as u32);
    let x_operand = ext_of_double(x_bits);
    let y_operand = ext_of_double(y_bits);
    let zero = X87Ext::load64(0.0);
    let one = X87Ext::load64(1.0);
    let infinity = |e: &Engine| load_double(e, POW_SPECIAL_INFINITY);
    let mut fpu = Fpu::with(zero);
    // FCOM double ptr [EBP+8]: 0 against x; FLD x; FCHS unless C0 or C3.
    let ah = fcom_ah(zero, x_operand);
    let mut base = X87Ext::load64(x);
    if ah & 0x41 == 0 {
        base = x87_fchs(base);
    }
    fpu.push(base);
    if y_high == 0x7ff0_0000 && y_low == 0 {
        // FLD1; FCOM: 1 against |x|; TEST AH, 5; JP.
        fpu.push(one);
        let ah = fcom_ah(one, base);
        if !jp_taken(ah, 5) {
            // 1 below |x|: all three off, the infinity.
            fpu.fstp(2);
            fpu.fstp(1);
            fpu.pop();
            store_st0(e, out, infinity(e));
            return 0;
        }
        // 00ed95a8: FCOM again; FSTP ST1; TEST AH, 0x41.
        let ah = fcom_ah(one, base);
        fpu.fstp(1);
        if ah & 0x41 == 0 {
            // FSTP ST0: the zero.
            fpu.pop();
        } else {
            // 00ed95bd: FSTP ST1: the one.
            fpu.fstp(1);
        }
        let top = fpu.pop();
        store_st0(e, out, top);
        return 0;
    }
    if y_high == 0xfff0_0000 && y_low == 0 {
        // 00ed95ce
        fpu.push(one);
        let ah = fcom_ah(one, base);
        if !jp_taken(ah, 5) {
            // FSTP ST0 twice: the zero.
            fpu.pop();
            fpu.pop();
            let top = fpu.pop();
            store_st0(e, out, top);
            return 0;
        }
        // 00ed95e2: FSTP ST2; FCOMPP: |x| against 1.
        fpu.fstp(2);
        let ah = fcom_ah(fpu.st(0), fpu.st(1));
        fpu.pop();
        fpu.pop();
        if !jp_taken(ah, 5) {
            store_st0(e, out, infinity(e));
            return 0;
        }
        // 00ed95fb
        let value = load_double(e, POW_SPECIAL_ONE_TO_MINUS_INFINITY);
        store_st0(e, out, value);
        return 1;
    }
    // 00ed960b: FSTP ST0: |x| off, the zero in ST0.
    fpu.pop();
    if x_high == 0x7ff0_0000 {
        if x_low != 0 {
            // 00ed9696
            fpu.pop();
            return 0;
        }
        // FCOM double ptr [EBP+0x10]: 0 against y.
        let ah = fcom_ah(zero, y_operand);
        if !jp_taken(ah, 5) {
            // 00ed959b: FSTP ST0; the infinity.
            fpu.pop();
            store_st0(e, out, infinity(e));
            return 0;
        }
        let ah = fcom_ah(zero, y_operand);
        if ah & 0x41 == 0 {
            let top = fpu.pop();
            store_st0(e, out, top);
            return 0;
        }
        fpu.pop();
        store_st0(e, out, one);
        return 0;
    }
    // 00ed9638
    fpu.pop();
    if x_high != 0xfff0_0000 || x_low != 0 {
        return 0;
    }
    let kind = e.call(D_INTTYPE, &args![x87_load64(y_bits)]).u32();
    let ah = fcom_ah(zero, y_operand);
    if !jp_taken(ah, 5) {
        // 0 below y: the infinity, negated for an odd integer.
        let mut value = infinity(e);
        if kind == 1 {
            value = x87_fchs(value);
        }
        store_st0(e, out, value);
        return 0;
    }
    // 00ed9672
    let ah = fcom_ah(zero, y_operand);
    if ah & 0x41 != 0 {
        // 00ed9690: FLD1.
        store_st0(e, out, one);
        return 0;
    }
    if kind == 1 {
        let value = load_double(e, POW_SPECIAL_MINUS_ZERO);
        store_st0(e, out, value);
        return 0;
    }
    store_st0(e, out, zero);
    0
}

/// `FSTP double ptr [EAX]` of a register value.
fn store_st0(e: &mut Engine, at: u32, value: X87Ext) {
    e.mem.set_u64(at, value.store64().to_bits());
}

// Translated from 00ed94f7 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__d_inttype` (Ghidra's function-ID label), cdecl, the double: 0 for a denormal
/// (`__fpclass` & 0x90) or a value `__frnd` changes (`FCOMP`, `TEST AH, 0x44`, `JP`), else 2 when
/// half of it (`FMUL` by `01011588`, stored as a double) is unchanged by `__frnd`, otherwise 1.
pub fn d_inttype(e: &mut Engine, x: f64) -> u32 {
    let copy = x87_load64(x.to_bits());
    let class = e.call(FPCLASS, &args![copy]).u32();
    if class & 0x90 != 0 {
        return 0;
    }
    let rounded = e.call(FRND, &args![copy]).f64();
    // FCOMP double ptr [EBP+8]: the rounded value against x.
    let ah = fcom_ah(X87Ext::load64(rounded), ext_of_double(x.to_bits()));
    if jp_taken(ah, 0x44) {
        return 0;
    }
    let half = x87_fmul(ext_of_double(x.to_bits()), double_operand(e, INTTYPE_HALF)).store64();
    let rounded = e.call(FRND, &args![half]).f64();
    let ah = fcom_ah(X87Ext::load64(rounded), X87Ext::load64(half));
    if jp_taken(ah, 0x44) {
        1
    } else {
        2
    }
}

// Translated from 00ec75e5 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__fpclass` (Ghidra's function-ID label), cdecl, the double: the `_FPCLASS` bit. An exponent
/// of all ones goes by `__sptype`: 1 gives +infinity 0x200, 2 -infinity 4, 3 a quiet NaN 2,
/// anything else a signalling NaN 1. A zero exponent field with a nonzero fraction is a
/// denormal (0x10 negative, 0x80 positive); otherwise 0 compared with it (`FCOMP`, `TEST AH,
/// 0x44`, `JP`) gives a zero (0x20, 0x40) or a normal number (8, 0x100) by the sign bit.
pub fn fpclass(e: &mut Engine, x: f64) -> u32 {
    let bits = x.to_bits();
    let (low, high) = (bits as u32, (bits >> 32) as u32);
    let top = (high >> 16) & 0xffff;
    let exponent = top & 0x7ff0;
    if exponent == 0x7ff0 {
        let copy = x87_load64(bits);
        return match e.call(SPTYPE, &args![copy]).i32() {
            1 => 0x200,
            2 => 4,
            3 => 2,
            _ => 1,
        };
    }
    let negative = top & 0x8000 != 0;
    if exponent == 0 && (high & 0xf_ffff != 0 || low != 0) {
        return if negative { 0x10 } else { 0x80 };
    }
    // 00ec764f: FLDZ; FCOMP double ptr [EBP+8].
    let ah = fcom_ah(X87Ext::load64(0.0), ext_of_double(bits));
    if jp_taken(ah, 0x44) {
        if negative {
            8
        } else {
            0x100
        }
    } else if negative {
        0x20
    } else {
        0x40
    }
}
// ---- SSE2 double-precision instructions ---------------------------------------------------------
//
// `crate::sse` has single-precision lanes only, so the SSE2 bodies' double instructions are
// written here on registers of two 64-bit lanes ([`Xmm`], the low lane first), with the
// instructions' semantics. The arithmetic (`ADDSD`/`SUBSD`/`MULSD`/`DIVSD`/`SQRTSD` and the
// packed forms) rounds to nearest without flushing denormals (the game's `MXCSR`, 0x1f80, which
// is what Rust's `f64` arithmetic does); a NaN operand gives the first (destination) operand's
// NaN when it is one, else the source's, quieted; an invalid operation without NaN operands the
// default NaN 0xfff8000000000000. The integer, logical, shuffle and move instructions work on the
// bits. A body's registers start as zero: the lanes the caller left in them flow only into lanes
// the bodies never read back into a result (each body was checked for that). The status flags
// the arithmetic raises in `MXCSR` are not kept (the engine has no `MXCSR` state).

/// An XMM register: two 64-bit lanes, low first.
type Xmm = [u64; 2];

/// A double's quiet bit.
const QUIET_BIT: u64 = 1 << 51;
/// The SSE default NaN ("QNaN floating-point indefinite").
const SSE_DEFAULT_NAN: u64 = 0xfff8_0000_0000_0000;

fn nan_bits(bits: u64) -> bool {
    bits & 0x7ff0_0000_0000_0000 == 0x7ff0_0000_0000_0000 && bits & 0x000f_ffff_ffff_ffff != 0
}

/// One lane of an SSE2 double operation `dest op src`, with the instruction's NaN choice.
fn sse_lane(dest: u64, src: u64, op: fn(f64, f64) -> f64) -> u64 {
    if nan_bits(dest) {
        return dest | QUIET_BIT;
    }
    if nan_bits(src) {
        return src | QUIET_BIT;
    }
    let r = op(f64::from_bits(dest), f64::from_bits(src));
    if r.is_nan() {
        SSE_DEFAULT_NAN
    } else {
        r.to_bits()
    }
}

fn op_add(a: f64, b: f64) -> f64 {
    a + b
}
fn op_sub(a: f64, b: f64) -> f64 {
    a - b
}
fn op_mul(a: f64, b: f64) -> f64 {
    a * b
}
fn op_div(a: f64, b: f64) -> f64 {
    a / b
}

/// `ADDSD dest, src`: the low lane.
fn addsd(dest: Xmm, src: u64) -> Xmm {
    [sse_lane(dest[0], src, op_add), dest[1]]
}
/// `SUBSD dest, src`.
fn subsd(dest: Xmm, src: u64) -> Xmm {
    [sse_lane(dest[0], src, op_sub), dest[1]]
}
/// `MULSD dest, src`.
fn mulsd(dest: Xmm, src: u64) -> Xmm {
    [sse_lane(dest[0], src, op_mul), dest[1]]
}
/// `DIVSD dest, src`.
fn divsd(dest: Xmm, src: u64) -> Xmm {
    [sse_lane(dest[0], src, op_div), dest[1]]
}
/// `SQRTSD dest, src`: the square root of the source's low lane (a NaN quieted, a negative
/// number but -0 the default NaN).
fn sqrtsd(dest: Xmm, src: u64) -> Xmm {
    let r = if nan_bits(src) {
        src | QUIET_BIT
    } else {
        let v = f64::from_bits(src).sqrt();
        if v.is_nan() {
            SSE_DEFAULT_NAN
        } else {
            v.to_bits()
        }
    };
    [r, dest[1]]
}
/// `ADDPD dest, src`: both lanes.
fn addpd(dest: Xmm, src: Xmm) -> Xmm {
    [
        sse_lane(dest[0], src[0], op_add),
        sse_lane(dest[1], src[1], op_add),
    ]
}
/// `SUBPD dest, src`.
fn subpd(dest: Xmm, src: Xmm) -> Xmm {
    [
        sse_lane(dest[0], src[0], op_sub),
        sse_lane(dest[1], src[1], op_sub),
    ]
}
/// `MULPD dest, src`.
fn mulpd(dest: Xmm, src: Xmm) -> Xmm {
    [
        sse_lane(dest[0], src[0], op_mul),
        sse_lane(dest[1], src[1], op_mul),
    ]
}
/// `ANDPD`/`PAND`.
fn andpd(a: Xmm, b: Xmm) -> Xmm {
    [a[0] & b[0], a[1] & b[1]]
}
/// `ORPD`/`POR`.
fn orpd(a: Xmm, b: Xmm) -> Xmm {
    [a[0] | b[0], a[1] | b[1]]
}
/// `XORPD`/`PXOR`.
fn xorpd(a: Xmm, b: Xmm) -> Xmm {
    [a[0] ^ b[0], a[1] ^ b[1]]
}
/// `PSRLQ` by `count` (an immediate, or the source register's low lane): above 63 gives zero.
fn psrlq(a: Xmm, count: u64) -> Xmm {
    if count > 63 {
        [0, 0]
    } else {
        [a[0] >> count, a[1] >> count]
    }
}
/// `PSLLQ` by `count`.
fn psllq(a: Xmm, count: u64) -> Xmm {
    if count > 63 {
        [0, 0]
    } else {
        [a[0] << count, a[1] << count]
    }
}
/// `PSUBQ`.
fn psubq(a: Xmm, b: Xmm) -> Xmm {
    [a[0].wrapping_sub(b[0]), a[1].wrapping_sub(b[1])]
}
/// The four dwords of a register, low first.
fn dwords(a: Xmm) -> [u32; 4] {
    [
        a[0] as u32,
        (a[0] >> 32) as u32,
        a[1] as u32,
        (a[1] >> 32) as u32,
    ]
}
fn from_dwords(d: [u32; 4]) -> Xmm {
    [
        u64::from(d[0]) | (u64::from(d[1]) << 32),
        u64::from(d[2]) | (u64::from(d[3]) << 32),
    ]
}
/// `PSUBD`.
fn psubd(a: Xmm, b: Xmm) -> Xmm {
    let (a, b) = (dwords(a), dwords(b));
    from_dwords([
        a[0].wrapping_sub(b[0]),
        a[1].wrapping_sub(b[1]),
        a[2].wrapping_sub(b[2]),
        a[3].wrapping_sub(b[3]),
    ])
}
/// `PCMPEQD`.
fn pcmpeqd(a: Xmm, b: Xmm) -> Xmm {
    let (a, b) = (dwords(a), dwords(b));
    let eq = |i: usize| if a[i] == b[i] { u32::MAX } else { 0 };
    from_dwords([eq(0), eq(1), eq(2), eq(3)])
}
/// `PMOVMSKB`: the top bit of each of the 16 bytes.
fn pmovmskb(a: Xmm) -> u32 {
    let mut mask = 0;
    for byte in 0..16 {
        let lane = a[byte / 8];
        if (lane >> (8 * (byte % 8) + 7)) & 1 != 0 {
            mask |= 1 << byte;
        }
    }
    mask
}
/// The eight words of a register, low first.
fn words(a: Xmm) -> [u16; 8] {
    let mut w = [0u16; 8];
    for (i, word) in w.iter_mut().enumerate() {
        *word = (a[i / 4] >> (16 * (i % 4))) as u16;
    }
    w
}
fn from_words(w: [u16; 8]) -> Xmm {
    let mut a = [0u64; 2];
    for (i, word) in w.iter().enumerate() {
        a[i / 4] |= u64::from(*word) << (16 * (i % 4));
    }
    a
}
/// `PMAXSW`: the signed maximum of each word.
fn pmaxsw(a: Xmm, b: Xmm) -> Xmm {
    let (a, b) = (words(a), words(b));
    let mut w = [0u16; 8];
    for i in 0..8 {
        w[i] = (a[i] as i16).max(b[i] as i16) as u16;
    }
    from_words(w)
}
/// `PEXTRW r32, xmm, i`: the word, zero-extended.
fn pextrw(a: Xmm, i: usize) -> u32 {
    u32::from(words(a)[i])
}
/// `PINSRW xmm, r32, i`: the low word of `value` in word `i`.
fn pinsrw(a: Xmm, value: u32, i: usize) -> Xmm {
    let mut w = words(a);
    w[i] = value as u16;
    from_words(w)
}
/// `PSHUFD`: dword `i` of the result is dword `(imm >> 2i) & 3` of the source.
fn pshufd(a: Xmm, imm: u8) -> Xmm {
    let d = dwords(a);
    let pick = |i: u32| d[usize::from((imm >> (2 * i)) & 3)];
    from_dwords([pick(0), pick(1), pick(2), pick(3)])
}
/// `UNPCKLPD dest, src`: the two low lanes.
fn unpcklpd(dest: Xmm, src: Xmm) -> Xmm {
    [dest[0], src[0]]
}
/// `SHUFPD dest, src, imm`.
fn shufpd(dest: Xmm, src: Xmm, imm: u8) -> Xmm {
    [
        if imm & 1 != 0 { dest[1] } else { dest[0] },
        if imm & 2 != 0 { src[1] } else { src[0] },
    ]
}
/// `MOVLPD xmm, m64` (and `MOVSD xmm, xmm`): the low lane replaced.
fn movlpd(dest: Xmm, value: u64) -> Xmm {
    [value, dest[1]]
}
/// `MOVQ xmm, m64` / `MOVQ xmm, xmm` / `MOVSD xmm, m64`: the low lane, the high one zero.
fn movq(value: u64) -> Xmm {
    [value, 0]
}
/// `MOVD xmm, r32`.
fn movd(value: u32) -> Xmm {
    [u64::from(value), 0]
}
/// `CVTDQ2PD`: the two low dwords as signed integers, converted exactly.
fn cvtdq2pd(src: Xmm) -> Xmm {
    let d = dwords(src);
    [
        f64::from(d[0] as i32).to_bits(),
        f64::from(d[1] as i32).to_bits(),
    ]
}
/// `CVTSD2SI r32, xmm`: rounded to nearest even (`MXCSR`'s rounding), the integer indefinite
/// 0x80000000 for a NaN or a value out of range: the same conversion as `FISTP` under the
/// default control word (`crate::x87::fistp`).
fn cvtsd2si(src: u64) -> u32 {
    crate::x87::fistp(f64::from_bits(src)) as u32
}

/// The flags `COMISD`/`UCOMISD` set: `ZF`, `PF`, `CF` (unordered sets all three).
#[derive(Clone, Copy)]
struct Compare {
    zf: bool,
    pf: bool,
    cf: bool,
}

/// `COMISD`/`UCOMISD a, b` (the two differ only in the exceptions they raise).
fn comisd(a: u64, b: u64) -> Compare {
    let (a, b) = (f64::from_bits(a), f64::from_bits(b));
    if a.is_nan() || b.is_nan() {
        Compare {
            zf: true,
            pf: true,
            cf: true,
        }
    } else {
        Compare {
            zf: a == b,
            pf: false,
            cf: a < b,
        }
    }
}

/// A 16-byte memory operand (`MOVAPD`, a packed instruction's source).
fn m128(e: &Engine, at: u32) -> Xmm {
    [e.mem.u64(at), e.mem.u64(at + 8)]
}
/// `pow`'s SSE2 constants (each a double, or the pair at a 16-byte address), named by how the
/// body uses them: the masks the base is split with (`010f84c0`, ANDed with it; `010f84d0`, ORed
/// in after, also the numerator of a zero base's reciprocal and the result of a zero exponent;
/// `010f84e0`, the magnitude mask), the high-part mask of the products (`010f8518`), the addend
/// of the second sum (`010f8520`), the rounding constant added and subtracted (`010f84b0`), the
/// pairs of the final polynomial (`010f84f0`, `010f8500`), its last factor (`010f8510`), the
/// pairs of the logarithm's polynomial (`010f7c90`, `010f7ca0`), the operand squared for an
/// invalid negative base (`010f8528`) and its result (`010f8560`), the results `010f8530`,
/// `010f8538` and `010f8540` (also the odd-exponent test's pattern), the overflow factors
/// (`010f8548`, `010f8550`) and the underflow one (`010f8558`).
const POW_FRACTION_MASK: u32 = 0x010f_84c0;
const POW_EXPONENT_BITS: u32 = 0x010f_84d0;
const POW_MAGNITUDE_MASK: u32 = 0x010f_84e0;
const POW_HIGH_MASK: u32 = 0x010f_8518;
const POW_SECOND_ADDEND: u32 = 0x010f_8520;
const POW_ROUNDER: u32 = 0x010f_84b0;
const POW_FINAL_PAIR_A: u32 = 0x010f_84f0;
const POW_FINAL_PAIR_B: u32 = 0x010f_8500;
const POW_FINAL_FACTOR: u32 = 0x010f_8510;
const POW_LOG_PAIR_A: u32 = 0x010f_7c90;
const POW_LOG_PAIR_B: u32 = 0x010f_7ca0;
const POW_INVALID_OPERAND: u32 = 0x010f_8528;
const POW_INVALID_RESULT: u32 = 0x010f_8560;
const POW_RESULT_8530: u32 = 0x010f_8530;
const POW_RESULT_8538: u32 = 0x010f_8538;
const POW_RESULT_8540: u32 = 0x010f_8540;
const POW_OVERFLOW_A: u32 = 0x010f_8548;
const POW_OVERFLOW_B: u32 = 0x010f_8550;
const POW_UNDERFLOW: u32 = 0x010f_8558;
/// `pow`'s SSE2 tables, indexed as the body indexes them: `010f4c30` (doubles, the first
/// factor; its first entry is also the result of a base of 1 to an infinite power), `010f5040`
/// (pairs), `010f5850` (doubles), `010f5c60` (pairs), `010f6470` (doubles), `010f6c80` (pairs)
/// and `010f7cb0` (pairs of the final scaling).
const POW_TABLE_FIRST: u32 = 0x010f_4c30;
const POW_TABLE_FIRST_PAIRS: u32 = 0x010f_5040;
const POW_TABLE_SECOND: u32 = 0x010f_5850;
const POW_TABLE_SECOND_PAIRS: u32 = 0x010f_5c60;
const POW_TABLE_THIRD: u32 = 0x010f_6470;
const POW_TABLE_THIRD_PAIRS: u32 = 0x010f_6c80;
const POW_TABLE_SCALE: u32 = 0x010f_7cb0;

// Translated from 00ed8790 (disassembled, FalloutNV.exe 1.4.0.525)
/// The register form of `pow`'s SSE2 body (no label; Ghidra gives it no name), base in `ST1`,
/// exponent in `ST0` (registered with the base first): a 16-byte aligned frame (`AND ESP,
/// -16`), the base and the exponent stored to it (`FXCH`, `FSTP` twice: a signalling NaN is
/// stored quiet), and the stack body `00ed87a9` called on it.
pub fn fn_00ed8790(e: &mut Engine, x: f64, y: f64) -> f64 {
    let base = x87_load64(x.to_bits());
    let exponent = x87_load64(y.to_bits());
    e.call(POW_SSE2, &args![base, exponent]).f64()
}

/// Where `pow`'s SSE2 body goes next (its labels).
#[derive(Clone, Copy)]
enum PowSse {
    Main,
    Special,
    InfiniteExponentSpecial,
    ZeroLike,
    Rescaled,
    ZeroHigh,
    ZeroPositiveExponent,
    InfiniteBase,
    NegativeZeroResult,
    PositiveOddResult,
    EvenResult,
    PositiveInfiniteBase,
    NanResult,
    NonInteger,
    InfiniteExponent,
    InfiniteExponentFinite,
    PositiveInfinityResult,
    NanExponent,
    ZeroExponent,
    One,
    Error,
    Wide,
    Underflow,
    Overflow,
    Extreme,
    Product,
}

// Translated from 00ed87a9 (disassembled, FalloutNV.exe 1.4.0.525)
/// `pow`'s SSE2 body (no label; Ghidra gives it no name), cdecl, the base then the exponent on
/// the stack, the result in `ST0` (`MOVLPD` to a slot, `FLD`). Table driven: the base's fraction
/// picks entries of `010f4c30`, `010f5040`, `010f5850`, `010f5c60`, `010f6470` and `010f6c80`
/// for its logarithm in two parts, which times the exponent (split by the mask at `010f8518`) is
/// rounded by `CVTSD2SI` into the index of `010f7cb0` and the scale, and the remainder goes
/// through the polynomial at `010f84f0`/`010f8500`/`010f8510`. Every instruction is the exe's,
/// in its order (see the module's SSE2 notes).
///
/// Special cases: a negative, zero, denormal, infinite or NaN base (its top word below 0x10 or
/// above 0x7fef) goes through the exponent's class: a NaN exponent (`ADDSD` of it with itself)
/// or NaN base gives error 0x3ee with the quiet NaN; a zero exponent 1.0 (`010f84d0`), with
/// error 0x1a for a zero base and 0x1d for a NaN one; a zero or denormal base is rescaled by
/// 2^64 and rejoins the main path, a zero one gives +-0 for a positive exponent and +-infinity
/// with error 0x1b for a negative one (sign for an odd integer exponent, tested with
/// `010f8540`); a negative base with a non-integer exponent gives `010f8560` with error 0x1c, an
/// integer one rejoins the main path with the result's sign; an infinite base, or an infinite
/// exponent, gives 0, 1.0, +infinity (`010f8530`), `010f8538` or `010f8540` by the signs and
/// `|x|` against 1, a base of +-1 to an infinite power error 0x1c with `010f4c30`'s entry. A
/// result exponent out of range overflows (error 0x18, `010f8548` times `010f8550`, or the
/// latter squared) or underflows (error 0x19, `010f8558` squared with the sign); the slow path
/// for large scales checks the final exponent for both. Errors go to `___libm_error_support`
/// with the argument slots and a result slot holding the low lane of `XMM0`, whose value is
/// then the result.
pub fn fn_00ed87a9(e: &mut Engine, x: f64, y: f64) -> f64 {
    e.with_stack(0x18, |e, slot| {
        let args_at = slot.addr();
        e.mem.set_u64(args_at, x.to_bits());
        e.mem.set_u64(args_at + 8, y.to_bits());
        pow_sse2(e, args_at)
    })
}

/// The body of [`fn_00ed87a9`] with the base at `at` and the exponent at `at + 8` (the stack's
/// `[ESP+4]` and `[ESP+0xc]`), the result slot at `at + 0x10`.
fn pow_sse2(e: &mut Engine, at: u32) -> f64 {
    let m = |e: &Engine, a: u32| e.mem.u64(a);
    let base = m(e, at);
    let exponent = m(e, at + 8);
    let mut x: [Xmm; 8] = [[0; 2]; 8];
    let (mut eax, mut ecx, mut edx): (u32, u32, u32);
    // 00ed87a9
    x[0] = movlpd(x[0], base);
    x[7] = movlpd(x[7], m(e, POW_FRACTION_MASK));
    x[2] = movlpd(x[2], m(e, POW_EXPONENT_BITS));
    x[7] = andpd(x[7], x[0]);
    x[4] = movlpd(x[4], x[0][0]);
    x[0] = psrlq(x[0], 0x2c);
    eax = pextrw(x[0], 0);
    x[7] = orpd(x[7], x[2]);
    ecx = pextrw(x[4], 3);
    eax &= 0xff;
    eax = eax.wrapping_add(1);
    eax &= 0x1fe;
    x[7] = mulsd(x[7], m(e, POW_TABLE_FIRST + eax * 4));
    x[5] = movlpd(x[5], m(e, POW_TABLE_FIRST + eax * 4));
    eax = eax.wrapping_add(eax);
    x[6] = m128(e, POW_TABLE_FIRST_PAIRS + eax * 4);
    edx = 0x7fef;
    edx = edx.wrapping_sub(ecx);
    ecx = ecx.wrapping_sub(0x10);
    ecx |= edx;
    let mut step = if ecx >= 0x8000_0000 {
        PowSse::Special
    } else {
        ecx = 0;
        edx = 0x3_fe7f;
        PowSse::Main
    };
    loop {
        step = match step {
            PowSse::Main => {
                // 00ed8826
                x[1] = movd(edx);
                x[0] = psubq(x[0], x[1]);
                x[0] = psrlq(x[0], 8);
                x[0] = cvtdq2pd(x[0]);
                x[1] = movlpd(x[1], m(e, POW_HIGH_MASK));
                x[3] = movlpd(x[3], x[7][0]);
                x[7] = psrlq(x[7], 0x26);
                eax = pextrw(x[7], 0);
                x[4] = andpd(x[4], m128(e, POW_FRACTION_MASK));
                eax &= 0xff;
                eax = eax.wrapping_add(1);
                eax &= 0x1fe;
                x[3] = mulsd(x[3], m(e, POW_TABLE_SECOND + eax * 4));
                x[5] = mulsd(x[5], m(e, POW_TABLE_SECOND + eax * 4));
                eax = eax.wrapping_add(eax);
                x[6] = addpd(x[6], m128(e, POW_TABLE_SECOND_PAIRS + eax * 4));
                x[4] = orpd(x[4], m128(e, POW_EXPONENT_BITS));
                x[6] = addsd(x[6], x[0][0]);
                x[1] = andpd(x[1], x[4]);
                x[2] = movlpd(x[2], x[3][0]);
                x[3] = psrlq(x[3], 0x1f);
                eax = pextrw(x[3], 0);
                x[0] = movlpd(x[0], m(e, POW_HIGH_MASK));
                x[4] = subsd(x[4], x[1][0]);
                x[7] = movlpd(x[7], m(e, POW_SECOND_ADDEND));
                eax &= 0x1ff;
                eax = eax.wrapping_add(1);
                eax &= 0x3fe;
                x[5] = mulsd(x[5], m(e, POW_TABLE_THIRD + eax * 4));
                x[2] = mulsd(x[2], m(e, POW_TABLE_THIRD + eax * 4));
                x[6] = addpd(x[6], m128(e, POW_TABLE_THIRD_PAIRS + eax * 8));
                x[0] = andpd(x[0], x[5]);
                x[5] = subsd(x[5], x[0][0]);
                x[7] = addsd(x[7], x[2][0]);
                x[3] = movlpd(x[3], x[0][0]);
                x[0] = mulsd(x[0], x[1][0]);
                x[1] = mulsd(x[1], x[5][0]);
                x[3] = mulsd(x[3], x[4][0]);
                x[2] = subsd(x[2], x[0][0]);
                x[4] = mulsd(x[4], x[5][0]);
                x[0] = movlpd(x[0], x[6][0]);
                x[2] = subsd(x[2], x[1][0]);
                x[6] = addsd(x[6], x[7][0]);
                x[1] = movlpd(x[1], exponent);
                eax = pextrw(x[1], 3);
                x[2] = subsd(x[2], x[3][0]);
                x[0] = subsd(x[0], x[6][0]);
                x[3] = movlpd(x[3], m(e, POW_HIGH_MASK));
                edx = pextrw(x[6], 3);
                x[2] = subsd(x[2], x[4][0]);
                x[4] = movlpd(x[4], x[6][0]);
                x[0] = addsd(x[0], x[7][0]);
                x[7] = subsd(x[7], x[2][0]);
                x[6] = subsd(x[6], x[2][0]);
                x[7] = unpcklpd(x[7], x[7]);
                eax &= 0x7ff0;
                if eax >= 0x7ff0 {
                    PowSse::InfiniteExponent
                } else {
                    edx &= 0x7ff0;
                    eax = eax.wrapping_sub(0x3ff0);
                    eax = eax.wrapping_add(edx);
                    edx = 0x40a0u32.wrapping_sub(eax);
                    eax = eax.wrapping_sub(0x3c70);
                    edx |= eax;
                    if edx >= 0x8000_0000 {
                        PowSse::Extreme
                    } else {
                        PowSse::Product
                    }
                }
            }
            PowSse::Product => {
                // 00ed8978
                x[4] = subsd(x[4], x[6][0]);
                x[2] = subsd(x[2], x[4][0]);
                x[4] = movlpd(x[4], m(e, POW_HIGH_MASK));
                x[3] = andpd(x[3], x[1]);
                x[4] = andpd(x[4], x[6]);
                x[0] = subsd(x[0], x[2][0]);
                x[2] = xorpd(x[2], x[2]);
                edx = 0x4060;
                x[2] = pinsrw(x[2], edx, 3);
                x[5] = movlpd(x[5], x[3][0]);
                x[3] = mulsd(x[3], x[4][0]);
                x[6] = subsd(x[6], x[4][0]);
                x[1] = subsd(x[1], x[5][0]);
                x[3] = mulsd(x[3], x[2][0]);
                x[5] = mulsd(x[5], x[6][0]);
                x[2] = m128(e, POW_LOG_PAIR_A);
                x[4] = mulsd(x[4], x[1][0]);
                eax = cvtsd2si(x[3][0]);
                x[6] = mulsd(x[6], x[1][0]);
                x[1] = m128(e, POW_LOG_PAIR_B);
                x[5] = addsd(x[5], x[4][0]);
                x[4] = pshufd(x[6], 0xee);
                x[5] = addsd(x[5], x[6][0]);
                edx = 0x1_ff7fu32.wrapping_sub(eax);
                eax = eax.wrapping_add(0x1_e1ff);
                edx |= eax;
                eax = eax.wrapping_sub(0x1_e1ff);
                if (edx as i32) <= 0 {
                    PowSse::Wide
                } else {
                    ecx = ecx.wrapping_add(eax);
                    eax &= 0x7f;
                    ecx &= 0xffff_ff80;
                    ecx = ecx.wrapping_add(0x1_ff80);
                    x[4] = addsd(x[4], x[0][0]);
                    x[0] = movlpd(x[0], x[3][0]);
                    x[3] = addsd(x[3], m(e, POW_ROUNDER));
                    x[2] = mulpd(x[2], x[7]);
                    x[3] = subsd(x[3], m(e, POW_ROUNDER));
                    x[7] = mulpd(x[7], x[7]);
                    x[0] = subsd(x[0], x[3][0]);
                    eax = eax.wrapping_mul(16);
                    x[3] = m128(e, POW_TABLE_SCALE.wrapping_add(eax));
                    x[6] = m128(e, POW_FINAL_PAIR_A);
                    x[1] = mulpd(x[1], x[7]);
                    x[2] = addpd(x[2], x[1]);
                    x[1] = pshufd(x[2], 0xee);
                    x[2] = mulsd(x[2], x[7][0]);
                    x[7] = xorpd(x[7], x[7]);
                    edx = 0x3f80;
                    x[2] = addsd(x[2], x[1][0]);
                    x[7] = pinsrw(x[7], edx, 3);
                    x[2] = addsd(x[2], x[4][0]);
                    x[4] = movlpd(x[4], m(e, POW_FINAL_FACTOR));
                    x[1] = movd(ecx);
                    x[2] = mulsd(x[2], exponent);
                    x[0] = mulsd(x[0], x[7][0]);
                    x[1] = psllq(x[1], 0x2d);
                    x[1] = pshufd(x[1], 0x44);
                    x[7] = m128(e, POW_FINAL_PAIR_B);
                    x[5] = addsd(x[5], x[2][0]);
                    x[3] = mulpd(x[3], x[1]);
                    x[0] = addsd(x[0], x[5][0]);
                    x[0] = unpcklpd(x[0], x[0]);
                    x[6] = mulpd(x[6], x[0]);
                    x[4] = mulsd(x[4], x[0][0]);
                    x[0] = mulpd(x[0], x[0]);
                    x[7] = addpd(x[7], x[6]);
                    x[7] = mulpd(x[7], x[0]);
                    x[0] = mulsd(x[0], x[3][0]);
                    x[6] = pshufd(x[7], 0xee);
                    x[0] = mulsd(x[0], x[7][0]);
                    x[5] = pshufd(x[3], 0xee);
                    x[6] = mulsd(x[6], x[3][0]);
                    x[4] = mulsd(x[4], x[3][0]);
                    x[0] = addsd(x[0], x[5][0]);
                    x[0] = addsd(x[0], x[6][0]);
                    x[0] = addsd(x[0], x[4][0]);
                    x[0] = addsd(x[0], x[3][0]);
                    return x87_load64(x[0][0]);
                }
            }
            PowSse::Special => {
                // 00ed8af7
                x[1] = movlpd(x[1], exponent);
                x[3] = movlpd(x[3], m(e, POW_MAGNITUDE_MASK));
                eax = x[1][0] as u32;
                x[2] = movlpd(x[2], x[1][0]);
                x[1] = andpd(x[1], x[3]);
                x[1] = psrlq(x[1], 0x20);
                ecx = x[1][0] as u32;
                if ecx >= 0x7ff0_0000 {
                    PowSse::InfiniteExponentSpecial
                } else {
                    eax |= ecx;
                    if eax == 0 {
                        PowSse::ZeroExponent
                    } else if (edx as i32) >= 0 {
                        // 00ed8c3f
                        ecx = 0;
                        PowSse::ZeroLike
                    } else {
                        edx = edx.wrapping_neg();
                        edx = edx.wrapping_add(0x7fef);
                        x[3] = psllq(x[3], 0x34);
                        x[2] = orpd(x[2], x[3]);
                        ecx = 0x3f3;
                        x[3] = movd(ecx);
                        x[1] = psrlq(x[1], 0x14);
                        x[1] = psubd(x[1], x[3]);
                        x[3] = xorpd(x[3], x[3]);
                        x[1] = pmaxsw(x[1], x[3]);
                        x[2] = psllq(x[2], x[1][0]);
                        x[2] = pcmpeqd(x[2], x[3]);
                        eax = pmovmskb(x[2]);
                        ecx = edx;
                        edx &= 0x7fff;
                        if edx >= 0x7ff0 {
                            PowSse::InfiniteBase
                        } else {
                            eax &= 0xff;
                            if eax != 0xff {
                                PowSse::NonInteger
                            } else {
                                // 00ed8b95: the integer's parity.
                                x[1] = movlpd(x[1], exponent);
                                x[2] = movlpd(x[2], exponent);
                                ecx = 0x3f4;
                                x[3] = movd(ecx);
                                x[1] = andpd(x[1], m128(e, POW_MAGNITUDE_MASK));
                                x[1] = psrlq(x[1], 0x34);
                                x[1] = psubd(x[1], x[3]);
                                x[3] = movlpd(x[3], m(e, POW_RESULT_8540));
                                x[2] = psllq(x[2], x[1][0]);
                                x[2] = pcmpeqd(x[2], x[3]);
                                eax = pmovmskb(x[2]);
                                eax &= 0xff;
                                ecx = 0x3_ff01;
                                ecx = ecx.wrapping_add(eax);
                                ecx &= 0x4_0000;
                                if edx < 0x10 {
                                    PowSse::ZeroLike
                                } else {
                                    edx = 0xb_fe7f;
                                    x[3] = movlpd(x[3], m(e, POW_FRACTION_MASK));
                                    x[2] = movlpd(x[2], m(e, POW_EXPONENT_BITS));
                                    PowSse::Main
                                }
                            }
                        }
                    }
                }
            }
            PowSse::InfiniteExponentSpecial => {
                // 00ed8c00
                x[7] = movlpd(x[7], base);
                x[4] = movlpd(x[4], base);
                edx = x[7][0] as u32;
                x[7] = psrlq(x[7], 0x20);
                eax = x[7][0] as u32;
                ecx = eax;
                eax &= 0x7fff_ffff;
                if eax < 0x7ff0_0000 {
                    PowSse::InfiniteExponentFinite
                } else if eax > 0x7ff0_0000 || edx != 0 {
                    PowSse::NanResult
                } else {
                    PowSse::InfiniteExponentFinite
                }
            }
            PowSse::ZeroLike => {
                // 00ed8c44
                x[0] = xorpd(x[0], x[0]);
                eax = 0x43f0;
                x[0] = pinsrw(x[0], eax, 3);
                x[7] = movlpd(x[7], m(e, POW_FRACTION_MASK));
                x[2] = movlpd(x[2], m(e, POW_EXPONENT_BITS));
                x[0] = mulsd(x[0], x[4][0]);
                edx = x[4][0] as u32;
                x[4] = psrlq(x[4], 0x20);
                eax = x[4][0] as u32;
                if edx == 0 {
                    PowSse::ZeroHigh
                } else {
                    PowSse::Rescaled
                }
            }
            PowSse::Rescaled => {
                // 00ed8c78
                x[7] = andpd(x[7], x[0]);
                x[4] = movlpd(x[4], x[0][0]);
                x[0] = andpd(x[0], m128(e, POW_MAGNITUDE_MASK));
                x[0] = psrlq(x[0], 0x2c);
                eax = pextrw(x[0], 0);
                x[7] = orpd(x[7], x[2]);
                eax &= 0xff;
                eax = eax.wrapping_add(1);
                eax &= 0x1fe;
                x[7] = mulsd(x[7], m(e, POW_TABLE_FIRST + eax * 4));
                x[5] = movlpd(x[5], m(e, POW_TABLE_FIRST + eax * 4));
                eax = eax.wrapping_add(eax);
                x[6] = m128(e, POW_TABLE_FIRST_PAIRS + eax * 4);
                edx = 0x4_3e7f;
                PowSse::Main
            }
            PowSse::ZeroHigh => {
                // 00ed8cca
                edx = eax;
                edx &= 0x7fff_ffff;
                if edx != 0 {
                    PowSse::Rescaled
                } else {
                    edx = (exponent >> 32) as u32;
                    edx &= 0x8000_0000;
                    if edx == 0 {
                        PowSse::ZeroPositiveExponent
                    } else {
                        // A zero base to a negative power: +-infinity, error 0x1b.
                        ecx <<= 0xd;
                        eax &= ecx;
                        edx = 0x7ff0_0000;
                        edx |= eax;
                        x[0] = movd(edx);
                        x[0] = psllq(x[0], 0x20);
                        x[1] = movlpd(x[1], m(e, POW_EXPONENT_BITS));
                        x[2] = movlpd(x[2], base);
                        // DIVSD XMM1, XMM2: a zero divide; the quotient is not used.
                        x[1] = divsd(x[1], x[2][0]);
                        edx = 0x1b;
                        PowSse::Error
                    }
                }
            }
            PowSse::ZeroPositiveExponent => {
                // 00ed8d17
                ecx <<= 0xd;
                eax &= ecx;
                if eax != 0 {
                    return x87_load64(m(e, POW_RESULT_8540));
                }
                return 0.0;
            }
            PowSse::InfiniteBase => {
                // 00ed8d28
                x[3] = movlpd(x[3], m(e, POW_FRACTION_MASK));
                x[1] = xorpd(x[1], x[1]);
                x[3] = andpd(x[3], x[4]);
                x[1] = pcmpeqd(x[1], x[3]);
                ecx = pmovmskb(x[1]);
                ecx &= 0xff;
                if ecx != 0xff {
                    PowSse::NanResult
                } else {
                    ecx = pextrw(x[4], 3);
                    ecx &= 0x8000;
                    if ecx == 0 {
                        PowSse::PositiveInfiniteBase
                    } else {
                        eax &= 0xff;
                        if eax != 0xff {
                            PowSse::EvenResult
                        } else {
                            x[1] = movlpd(x[1], exponent);
                            x[2] = movlpd(x[2], exponent);
                            ecx = 0x3f4;
                            x[3] = movd(ecx);
                            x[1] = andpd(x[1], m128(e, POW_MAGNITUDE_MASK));
                            x[1] = psrlq(x[1], 0x34);
                            x[1] = psubd(x[1], x[3]);
                            x[3] = xorpd(x[3], x[3]);
                            x[2] = psllq(x[2], x[1][0]);
                            x[2] = pcmpeqd(x[2], x[3]);
                            eax = pmovmskb(x[2]);
                            eax &= 0xff;
                            if eax == 0xff {
                                PowSse::EvenResult
                            } else {
                                // 00ed8db4: an odd power.
                                x[1] = movlpd(x[1], exponent);
                                eax = pextrw(x[1], 3);
                                eax &= 0x8000;
                                if eax == 0 {
                                    PowSse::PositiveOddResult
                                } else {
                                    PowSse::NegativeZeroResult
                                }
                            }
                        }
                    }
                }
            }
            PowSse::NegativeZeroResult => {
                // 00ed8dc9
                return x87_load64(m(e, POW_RESULT_8540));
            }
            PowSse::PositiveOddResult => {
                // 00ed8dd0
                return x87_load64(m(e, POW_RESULT_8538));
            }
            PowSse::EvenResult => {
                // 00ed8dd7
                x[1] = movlpd(x[1], exponent);
                eax = pextrw(x[1], 3);
                eax &= 0x8000;
                if eax == 0 {
                    PowSse::PositiveInfinityResult
                } else {
                    return 0.0;
                }
            }
            PowSse::PositiveInfiniteBase => {
                // 00ed8df3
                x[1] = movlpd(x[1], exponent);
                eax = pextrw(x[1], 3);
                eax &= 0x8000;
                if eax == 0 {
                    PowSse::PositiveInfinityResult
                } else {
                    return 0.0;
                }
            }
            PowSse::NanResult => {
                // 00ed8e0f: ADDSD XMM4, XMM4 (the base's NaN, quiet).
                x[4] = addsd(x[4], x[4][0]);
                x[0] = movlpd(x[0], x[4][0]);
                edx = 0x3ee;
                PowSse::Error
            }
            PowSse::NonInteger => {
                // 00ed8e21
                x[2] = movlpd(x[2], base);
                eax = x[2][0] as u32;
                x[2] = psrlq(x[2], 0x20);
                edx = x[2][0] as u32;
                edx &= 0x7fff_ffff;
                eax |= edx;
                ecx = 0;
                if eax == 0 {
                    PowSse::ZeroHigh
                } else {
                    x[1] = movlpd(x[1], m(e, POW_INVALID_OPERAND));
                    x[0] = movlpd(x[0], m(e, POW_INVALID_RESULT));
                    // MULSD XMM1, XMM1: raises the invalid flag; the product is not used.
                    x[1] = mulsd(x[1], x[1][0]);
                    edx = 0x1c;
                    PowSse::Error
                }
            }
            PowSse::InfiniteExponent => {
                // 00ed8e68
                x[4] = movlpd(x[4], base);
                x[2] = movlpd(x[2], exponent);
                x[3] = movlpd(x[3], m(e, POW_FRACTION_MASK));
                x[1] = xorpd(x[1], x[1]);
                x[3] = andpd(x[3], x[2]);
                x[1] = pcmpeqd(x[1], x[3]);
                eax = pmovmskb(x[1]);
                eax &= 0xff;
                if eax != 0xff {
                    PowSse::NanExponent
                } else {
                    eax = x[4][0] as u32;
                    if eax != 0 {
                        PowSse::InfiniteExponentFinite
                    } else {
                        x[4] = psrlq(x[4], 0x20);
                        edx = x[4][0] as u32;
                        if edx == 0x3ff0_0000 {
                            PowSse::One
                        } else if edx != 0xbff0_0000 {
                            PowSse::InfiniteExponentFinite
                        } else {
                            return 1.0;
                        }
                    }
                }
            }
            PowSse::InfiniteExponentFinite => {
                // 00ed8ec5
                x[3] = movlpd(x[3], m(e, POW_FRACTION_MASK));
                x[1] = xorpd(x[1], x[1]);
                x[3] = andpd(x[3], x[2]);
                x[1] = pcmpeqd(x[1], x[3]);
                eax = pmovmskb(x[1]);
                eax &= 0xff;
                if eax != 0xff {
                    PowSse::NanExponent
                } else {
                    eax = pextrw(x[2], 3);
                    x[4] = movlpd(x[4], base);
                    eax &= 0x8000;
                    ecx ^= 0xbff0_0000;
                    edx |= ecx;
                    if edx == 0 {
                        PowSse::One
                    } else {
                        let magnitude = pextrw(x[4], 3) & 0x7ff0;
                        if eax != 0 {
                            // 00ed8f0f: a negative infinite exponent.
                            if magnitude < 0x3ff0 {
                                PowSse::PositiveInfinityResult
                            } else {
                                return 0.0;
                            }
                        } else if magnitude >= 0x3ff0 {
                            PowSse::PositiveInfinityResult
                        } else {
                            return 0.0;
                        }
                    }
                }
            }
            PowSse::PositiveInfinityResult => {
                // 00ed8f37
                return x87_load64(m(e, POW_RESULT_8530));
            }
            PowSse::NanExponent => {
                // 00ed8f3e: ADDSD XMM2, XMM2 (the exponent's NaN, quiet).
                x[2] = addsd(x[2], x[2][0]);
                x[0] = movlpd(x[0], x[2][0]);
                edx = 0x3ee;
                PowSse::Error
            }
            PowSse::ZeroExponent => {
                // 00ed8f4d
                eax = x[4][0] as u32;
                x[4] = psrlq(x[4], 0x20);
                edx = x[4][0] as u32;
                edx &= 0x7fff_ffff;
                ecx = eax;
                eax |= edx;
                x[0] = movlpd(x[0], m(e, POW_EXPONENT_BITS));
                edx = 0x1a;
                if eax == 0 {
                    PowSse::Error
                } else {
                    eax = x[4][0] as u32;
                    edx = 0x1d;
                    eax &= 0x7fff_ffff;
                    if eax > 0x7ff0_0000 || (eax == 0x7ff0_0000 && ecx != 0) {
                        PowSse::Error
                    } else {
                        // 00ed8f92
                        return x87_load64(x[0][0]);
                    }
                }
            }
            PowSse::One => {
                // 00ed8fa3
                x[0] = movlpd(x[0], m(e, POW_TABLE_FIRST));
                edx = 0x1c;
                PowSse::Error
            }
            PowSse::Error => {
                // 00ed8fb0: ___libm_error_support(&x, &y, &result, code) with the result slot
                // holding XMM0's low lane.
                let result = at + 0x10;
                e.mem.set_u64(result, x[0][0]);
                e.call(LIBM_ERROR_SUPPORT, &args![at, at + 8, result, edx]);
                return x87_load64(e.mem.u64(result));
            }
            PowSse::Wide => {
                // 00ed8fe0
                if (eax as i32) > 0 {
                    if eax >= 0x4_0000 {
                        PowSse::Overflow
                    } else {
                        // 00ed8ff0
                        edx = eax;
                        eax &= 0x7f;
                        ecx = ecx.wrapping_add(0x3_ff00);
                        edx = edx.wrapping_sub(0x80);
                        edx &= 0xffff_ff80;
                        return pow_sse2_scaled(e, &mut x, eax, ecx, edx, 0x3ff0, exponent, at);
                    }
                } else if (eax as i32) <= -0x3_fe00 {
                    PowSse::Underflow
                } else {
                    // 00ed9018
                    edx = eax;
                    eax &= 0x7f;
                    ecx = ecx.wrapping_add(0x80);
                    edx &= 0xffff_ff80;
                    edx = edx.wrapping_add(0x3_fe80);
                    return pow_sse2_scaled(e, &mut x, eax, ecx, edx, 0, exponent, at);
                }
            }
            PowSse::Underflow => {
                // 00ed9209
                x[0] = movlpd(x[0], m(e, POW_UNDERFLOW));
                x[1] = movd(ecx);
                x[0] = mulsd(x[0], x[0][0]);
                x[1] = psllq(x[1], 0x2d);
                x[0] = orpd(x[0], x[1]);
                edx = 0x19;
                PowSse::Error
            }
            PowSse::Overflow => {
                // 00ed922c
                edx = 0x18;
                if ecx != 0 {
                    x[0] = movlpd(x[0], m(e, POW_OVERFLOW_A));
                    x[0] = mulsd(x[0], m(e, POW_OVERFLOW_B));
                } else {
                    x[0] = movlpd(x[0], m(e, POW_OVERFLOW_B));
                    x[0] = mulsd(x[0], x[0][0]);
                }
                PowSse::Error
            }
            PowSse::Extreme => {
                // 00ed925c
                x[4] = pshufd(x[1], 0x44);
                x[4] = mulpd(x[4], x[6]);
                eax = pextrw(x[4], 3);
                eax &= 0x7ff0;
                edx = 0x40a0u32.wrapping_sub(eax);
                eax = eax.wrapping_sub(0x3c70);
                edx |= eax;
                if edx < 0x8000_0000 {
                    PowSse::Product
                } else if eax >= 0x8000_0000 {
                    // 00ed9290: a product too small to matter: +-1.
                    ecx |= 0x1_ff80;
                    x[0] = movd(ecx);
                    x[0] = psllq(x[0], 0x2d);
                    return x87_load64(x[0][0]);
                } else {
                    // 00ed92b0
                    x[4] = movlpd(x[4], base);
                    edx = pextrw(x[4], 3);
                    edx &= 0x7ff0;
                    edx = edx.wrapping_sub(0x3ff0);
                    eax = pextrw(x[1], 3);
                    eax ^= edx;
                    eax &= 0x8000;
                    if eax != 0 {
                        PowSse::Underflow
                    } else {
                        PowSse::Overflow
                    }
                }
            }
        };
    }
}

/// `pow`'s SSE2 path for a scale beyond the main path's range (`00ed9033`, after `PUSH ESI;
/// PUSH EDI`): the same final polynomial, the scale split into two factors (`ECX` and `EDX`
/// shifted into exponents, `EDI` the second factor's exponent word), the sum masked by the
/// shift count `EDX` (`PCMPEQD`/`PSLLQ` all-ones masks) so its parts add exactly, and the
/// final exponent checked: all ones is error 0x18 (overflow), zero error 0x19 (underflow).
#[allow(clippy::too_many_arguments)]
fn pow_sse2_scaled(
    e: &mut Engine,
    x: &mut [Xmm; 8],
    eax_in: u32,
    ecx: u32,
    edx_in: u32,
    edi: u32,
    exponent: u64,
    at: u32,
) -> f64 {
    let m = |e: &Engine, a: u32| e.mem.u64(a);
    let mut eax = eax_in;
    let mut edx = edx_in;
    // 00ed9033
    x[4] = addsd(x[4], x[0][0]);
    x[0] = movlpd(x[0], x[3][0]);
    x[3] = addsd(x[3], m(e, POW_ROUNDER));
    // PUSH ESI; PUSH EDI (EDI set by the caller); MOV ESI, EDX; SUB ESI, 0x1ff80.
    let esi = edx.wrapping_sub(0x1_ff80);
    x[2] = mulpd(x[2], x[7]);
    x[3] = subsd(x[3], m(e, POW_ROUNDER));
    x[7] = mulpd(x[7], x[7]);
    x[0] = subsd(x[0], x[3][0]);
    eax = eax.wrapping_mul(16);
    x[3] = m128(e, POW_TABLE_SCALE.wrapping_add(eax));
    x[6] = m128(e, POW_FINAL_PAIR_A);
    x[1] = mulpd(x[1], x[7]);
    x[2] = addpd(x[2], x[1]);
    x[1] = pshufd(x[2], 0xee);
    x[2] = mulsd(x[2], x[7][0]);
    x[2] = addsd(x[2], x[1][0]);
    x[2] = addsd(x[2], x[4][0]);
    x[4] = movlpd(x[4], m(e, POW_FINAL_FACTOR));
    x[1] = movd(edx);
    edx = edx.wrapping_sub(0x1_ff80);
    edx = edx.wrapping_neg();
    edx = ((edx as i32) >> 7) as u32;
    edx = edx.wrapping_add(2);
    eax = edx;
    eax &= 0x20;
    edx = edx.wrapping_add(eax);
    x[7] = xorpd(x[7], x[7]);
    eax = 0x3f80;
    x[7] = pinsrw(x[7], eax, 3);
    // MULSD XMM2, [ESP+0x14]: the exponent (two words pushed above the arguments).
    x[2] = mulsd(x[2], exponent);
    x[0] = mulsd(x[0], x[7][0]);
    x[1] = psllq(x[1], 0x2d);
    x[1] = pshufd(x[1], 0x44);
    x[7] = m128(e, POW_FINAL_PAIR_B);
    x[5] = addsd(x[5], x[2][0]);
    x[3] = mulpd(x[3], x[1]);
    x[0] = addsd(x[0], x[5][0]);
    x[0] = unpcklpd(x[0], x[0]);
    x[6] = mulpd(x[6], x[0]);
    x[4] = mulsd(x[4], x[0][0]);
    x[0] = mulpd(x[0], x[0]);
    x[7] = addpd(x[7], x[6]);
    x[7] = mulpd(x[7], x[0]);
    x[0] = mulsd(x[0], x[3][0]);
    x[6] = pshufd(x[7], 0xee);
    x[0] = mulsd(x[0], x[7][0]);
    x[5] = pshufd(x[3], 0xee);
    x[6] = mulsd(x[6], x[3][0]);
    x[4] = mulsd(x[4], x[3][0]);
    x[7] = movd(ecx);
    x[7] = psllq(x[7], 0x2d);
    x[2] = movd(edx);
    x[1] = pcmpeqd(x[1], x[1]);
    x[1] = psllq(x[1], x[2][0]);
    x[0] = addsd(x[0], x[5][0]);
    x[0] = addsd(x[0], x[6][0]);
    x[1] = andpd(x[1], x[3]);
    x[0] = addsd(x[0], x[4][0]);
    x[6] = xorpd(x[6], x[6]);
    x[4] = pcmpeqd(x[4], x[4]);
    x[4] = psllq(x[4], x[2][0]);
    x[3] = subsd(x[3], x[1][0]);
    x[2] = movlpd(x[2], x[1][0]);
    x[1] = addsd(x[1], x[0][0]);
    x[1] = andpd(x[1], x[4]);
    x[6] = pinsrw(x[6], edi, 3);
    x[2] = subsd(x[2], x[1][0]);
    x[0] = addsd(x[0], x[2][0]);
    x[0] = addsd(x[0], x[3][0]);
    if (esi as i32) > 0 {
        // 00ed91bf
        x[0] = addsd(x[0], x[1][0]);
        x[0] = mulsd(x[0], x[7][0]);
        x[6] = mulsd(x[6], x[0][0]);
        x[0] = addsd(x[0], x[6][0]);
    } else {
        x[0] = mulsd(x[0], x[7][0]);
        x[1] = mulsd(x[1], x[7][0]);
        x[0] = addsd(x[0], x[1][0]);
        x[6] = mulsd(x[6], x[0][0]);
        x[0] = addsd(x[0], x[6][0]);
    }
    let top = pextrw(x[0], 3) & 0x7ff0;
    let code = if top == 0x7ff0 {
        0x18u32
    } else if top == 0 {
        0x19
    } else {
        return x87_load64(x[0][0]);
    };
    let result = at + 0x10;
    e.mem.set_u64(result, x[0][0]);
    e.call(LIBM_ERROR_SUPPORT, &args![at, at + 8, result, code]);
    x87_load64(e.mem.u64(result))
}

// ---- acos, asin, atan: the entries and the x87 bodies ------------------------------------------

/// The SSE2 bodies: `acos` (`00ed9b00` register, `00ed9b18` stack), `asin` (`00ed7720`,
/// `00ed7738`) and `atan` (`00ed5d80`, `00ed5d98`).
const ACOS_SSE2_REGISTER: u32 = 0x00ed_9b00;
const ACOS_SSE2: u32 = 0x00ed_9b18;
const ASIN_SSE2: u32 = 0x00ed_7738;
const ATAN_SSE2_REGISTER: u32 = 0x00ed_5d80;
const ATAN_SSE2: u32 = 0x00ed_5d98;
/// The x87 helpers of `trig.rs` the bodies call: `00ed2808` (the stored double's exponent bits)
/// and `00ed27ac` (a NaN's error type).
const EXPONENT_BITS: u32 = 0x00ed_2808;
const NAN_ERROR_TYPE: u32 = 0x00ed_27ac;
/// The names and operation codes the bodies' error reports pass: `"acos"` (`011c1970`, 0xd),
/// `"asin"` (`011c1930`, 0xe) and `"atan"` (`011c18d0`, 0xf).
const ACOS_NAME: u32 = 0x011c_1970;
const ASIN_NAME: u32 = 0x011c_1930;
const ATAN_NAME: u32 = 0x011c_18d0;
const ACOS_OPERATION: u32 = 0xd;
const ASIN_OPERATION: u32 = 0xe;
const ATAN_OPERATION: u32 = 0xf;
/// The extended constant `asin` and `atan` load for `|x| = 1` and an infinity (`011c1d0a`,
/// pi/2 in the exe), and the control word `atan`'s body loads (`010f2a98`).
const HALF_PI_EXTENDED: u32 = 0x011c_1d0a;
const ATAN_CONTROL_WORD: u32 = 0x010f_2a98;

/// The register entries' x87 path (`SUB ESP, 0xc; FST double ptr [ESP]; CALL 00ed2808; CALL
/// <body>`): `ST0` (as `FLD` leaves a double) stored, its exponent bits and `ZF` from
/// `00ed2808`, then the body on the register and the stored double.
fn register_x87(
    e: &mut Engine,
    st0: f64,
    body: fn(&mut Engine, bool, u32, X87Ext, u32, u32) -> X87Ext,
) -> f64 {
    let register = X87Ext::load64(st0);
    let stored = register.store64().to_bits();
    let (low, high) = (stored as u32, (stored >> 32) as u32);
    let eax = e.call(EXPONENT_BITS, &args![low, high]).u32();
    let zf = high & 0x7ff0_0000 == 0x7ff0_0000;
    body(e, zf, eax, register, low, high).store64()
}

/// The stack entries' x87 path (`LEA EDX, [ESP+4]; CALL 00ed27c5`, falling into the body): the
/// double loaded by `__fload_withFB` (a signalling NaN kept), its `EAX` and `ZF`.
fn stack_x87(
    e: &mut Engine,
    x: f64,
    body: fn(&mut Engine, bool, u32, X87Ext, u32, u32) -> X87Ext,
) -> f64 {
    let bits = x.to_bits();
    let (low, high) = (bits as u32, (bits >> 32) as u32);
    let (eax, zf, register) = e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), bits);
        let mut fpu = Fpu::default();
        let (eax, zf) = fload_with_fb_at(e, &mut fpu, slot.addr());
        (eax, zf, fpu.pop())
    });
    body(e, zf, eax, register, low, high).store64()
}

/// The end of the bodies' result paths: the fast exit with the math-error switch set, else
/// `__math_exit` (`trig.rs`) with the operation and the name.
fn inverse_exit(
    e: &mut Engine,
    result: X87Ext,
    saved: u16,
    operation: u32,
    name: u32,
    low: u32,
    high: u32,
) -> X87Ext {
    if e.global::<u32>(ERROR_SWITCH) != 0 {
        fast_exit(e, saved);
        return result;
    }
    let st0 = result.store64();
    let r = super::trig::math_exit(e, operation, name, st0, u32::from(saved), 0, low, high);
    X87Ext::load64(r)
}

/// The end of the bodies' error paths, with the error type in `EAX`: the fast exit with the
/// switch set, else `__startOneArgErrorHandling` (`trig.rs`).
#[allow(clippy::too_many_arguments)]
fn inverse_error(
    e: &mut Engine,
    kind: u32,
    result: X87Ext,
    saved: u16,
    operation: u32,
    name: u32,
    low: u32,
    high: u32,
) -> X87Ext {
    if e.global::<u32>(ERROR_SWITCH) != 0 {
        fast_exit(e, saved);
        return result;
    }
    let st0 = result.store64();
    let r = super::trig::start_one_arg_error_handling(
        e,
        kind,
        name,
        operation,
        st0,
        u32::from(saved),
        0,
        low,
        high,
    );
    X87Ext::load64(r)
}

/// An infinity or NaN argument of the bodies (`ZF` set, `EAX` the high word): a NaN (a fraction
/// bit in `EAX & 0xfffff` or the low word: `TEST EAX, 0xfffff; JNZ; CMP dword ptr [ESP+8], 0;
/// JNZ`) gets its error type and `ST0` from `00ed27ac`; `None` for an infinity.
fn nan_type(e: &mut Engine, eax: u32, register: X87Ext, low: u32) -> Option<(u32, X87Ext)> {
    if eax & 0xf_ffff != 0 || low != 0 {
        let st0 = register.store64();
        let r = e.call(NAN_ERROR_TYPE, &args![eax, st0]);
        return Some((r.eax, ext_of_double(r.st0.to_bits())));
    }
    None
}

// Translated from 00eca190 (disassembled, FalloutNV.exe 1.4.0.525)
/// `acos` (LIBCMT; the name `"acos"` its error reports pass, `011c1970`), cdecl, the double on
/// the stack, the result in `ST0`. With `__use_sse2_mathfcns` set and every `MXCSR` and x87
/// exception masked (`super::sse2_math_available`) it jumps to the SSE2 body `00ed9b18`;
/// otherwise it loads the argument with `__fload_withFB` and falls into the x87 body `00eca228`.
pub fn acos(e: &mut Engine, x: f64) -> f64 {
    if super::sse2_math_available(e) {
        return e.call(ACOS_SSE2, &args![x]).f64();
    }
    stack_x87(e, x, acos_x87)
}

// Translated from 00eca1d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_CIacos` (the register form of `acos`; no label in the map, named from its body), `ST0` in,
/// `ST0` out: the same test picks the SSE2 body `00ed9b00`; otherwise `ST0` is stored, `00ed2808`
/// gives `EAX` and `ZF`, and the x87 body `00eca228` runs.
pub fn ci_acos(e: &mut Engine, st0: f64) -> f64 {
    if super::sse2_math_available(e) {
        return e.call(ACOS_SSE2_REGISTER, &args![st0]).f64();
    }
    register_x87(e, st0, acos_x87)
}

// Translated from 00eca228 (disassembled, FalloutNV.exe 1.4.0.525)
/// `acos`'s x87 body (no label; Ghidra gives it no name): `ZF` (the argument's exponent all
/// ones), `EAX` (its exponent bits, or the high word then), `ST0` the argument, and the stored
/// double. It saves the control word and, for a finite argument, runs under `00ed2795`'s unless
/// it was `0x027f`. `|x| < 1` (`EAX` below `0x3ff00000`): `FPATAN` of `sqrt((1 + x)(1 - x))`
/// over `x` (`FLD1; FADD; FLD1; FSUB; FMULP; FSQRT; FXCH; FPATAN`), leaving through
/// `__math_exit` (operation 0xd, `"acos"`). `|x| = 1`: 0 (`FLDZ`) or pi (`FLDPI`) for -1, the
/// same exit. Anything larger, or an infinity, is a domain error with the indefinite NaN (type
/// 1); a NaN gets its type from `00ed27ac`; errors go to `__startOneArgErrorHandling`. With the
/// math-error switch set every path leaves through the fast exit.
pub fn fn_00eca228(e: &mut Engine, zf: bool, eax: u32, st0: f64, low: u32, high: u32) -> f64 {
    acos_x87(e, zf, eax, ext_of_double(st0.to_bits()), low, high).store64()
}

fn acos_x87(e: &mut Engine, zf: bool, eax: u32, x: X87Ext, low: u32, high: u32) -> X87Ext {
    let (operation, name) = (ACOS_OPERATION, ACOS_NAME);
    // PUSH EDX; FSTCW word ptr [ESP].
    let saved = x87_control_word(e);
    let (kind, result) = if zf {
        // 00eca29c
        match nan_type(e, eax, x, low) {
            Some(found) => found,
            None => (1, load_extended(e, INDEFINITE)),
        }
    } else {
        if saved != 0x027f {
            fn_00ed2795(e, u32::from(saved));
        }
        if eax < 0x3ff0_0000 {
            let one = X87Ext::load64(1.0);
            // FLD1; FADD ST0, ST1; FLD1; FSUB ST0, ST2; FMULP; FSQRT; FXCH; FPATAN.
            let sum = x87_fadd(one, x);
            let difference = x87_fsub(one, x);
            let root = x87_fsqrt(x87_fmul(sum, difference));
            let result = x87_fpatan(root, x);
            return inverse_exit(e, result, saved, operation, name, low, high);
        }
        if eax == 0x3ff0_0000 && (high & 0xf_ffff) | low == 0 {
            // 00eca283: FSTP ST0; FLDPI for -1, FLDZ for 1.
            let result = if high & 0x8000_0000 != 0 {
                x87_fld_constant(X87Constant::Pi)
            } else {
                X87Ext::load64(0.0)
            };
            return inverse_exit(e, result, saved, operation, name, low, high);
        }
        // 00eca2aa
        (1, load_extended(e, INDEFINITE))
    };
    inverse_error(e, kind, result, saved, operation, name, low, high)
}

// Translated from 00ec9610 (disassembled, FalloutNV.exe 1.4.0.525)
/// `asin` (LIBCMT; Ghidra's function ID gives this body a conflicting name, the name `"asin"`
/// its error reports pass at `011c1930` decides), cdecl, the double on the stack, the result in
/// `ST0`: the same test picks the SSE2 body `00ed7738`; otherwise `__fload_withFB` and the x87
/// body `00ec96a8`.
pub fn asin(e: &mut Engine, x: f64) -> f64 {
    if super::sse2_math_available(e) {
        return e.call(ASIN_SSE2, &args![x]).f64();
    }
    stack_x87(e, x, asin_x87)
}

// Translated from 00ec96a8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `asin`'s x87 body (no label; Ghidra gives it no name), with the inputs of `acos`'s
/// (`fn_00eca228`): `|x| < 1` is `FPATAN` of `x` over `sqrt((1 + x)(1 - x))`, `|x| = 1` the
/// extended constant at `011c1d0a` (pi/2), negated for -1, both through `__math_exit`
/// (operation 0xe, `"asin"`); errors as `acos`'s.
pub fn fn_00ec96a8(e: &mut Engine, zf: bool, eax: u32, st0: f64, low: u32, high: u32) -> f64 {
    asin_x87(e, zf, eax, ext_of_double(st0.to_bits()), low, high).store64()
}

fn asin_x87(e: &mut Engine, zf: bool, eax: u32, x: X87Ext, low: u32, high: u32) -> X87Ext {
    let (operation, name) = (ASIN_OPERATION, ASIN_NAME);
    let saved = x87_control_word(e);
    let (kind, result) = if zf {
        // 00ec971c
        match nan_type(e, eax, x, low) {
            Some(found) => found,
            None => (1, load_extended(e, INDEFINITE)),
        }
    } else {
        if saved != 0x027f {
            fn_00ed2795(e, u32::from(saved));
        }
        if eax < 0x3ff0_0000 {
            let one = X87Ext::load64(1.0);
            // FLD1; FADD ST0, ST1; FLD1; FSUB ST0, ST2; FMULP; FSQRT; FPATAN.
            let sum = x87_fadd(one, x);
            let difference = x87_fsub(one, x);
            let root = x87_fsqrt(x87_fmul(sum, difference));
            let result = x87_fpatan(x, root);
            return inverse_exit(e, result, saved, operation, name, low, high);
        }
        if eax == 0x3ff0_0000 && (high & 0xf_ffff) | low == 0 {
            // 00ec9701: AND ECX, 0x80000000; FSTP ST0; FLD tbyte [011c1d0a]; FCHS for -1.
            let mut result = load_extended(e, HALF_PI_EXTENDED);
            if high & 0x8000_0000 != 0 {
                result = x87_fchs(result);
            }
            return inverse_exit(e, result, saved, operation, name, low, high);
        }
        // 00ec972a
        (1, load_extended(e, INDEFINITE))
    };
    inverse_error(e, kind, result, saved, operation, name, low, high)
}

// Translated from 00ec9010 (disassembled, FalloutNV.exe 1.4.0.525)
/// `atan` (LIBCMT; Ghidra's function ID gives this body a conflicting name, the name `"atan"`
/// its error reports pass at `011c18d0` decides), cdecl, the double on the stack, the result in
/// `ST0`: the same test picks the SSE2 body `00ed5d98`; otherwise `__fload_withFB` and the x87
/// body `00ec90a8`.
pub fn atan(e: &mut Engine, x: f64) -> f64 {
    if super::sse2_math_available(e) {
        return e.call(ATAN_SSE2, &args![x]).f64();
    }
    stack_x87(e, x, atan_x87)
}

// Translated from 00ec9050 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_CIatan` (the register form of `atan`; no label in the map, named from its body), `ST0` in,
/// `ST0` out: the same test picks the SSE2 body `00ed5d80`; otherwise `ST0` is stored,
/// `00ed2808` gives `EAX` and `ZF`, and the x87 body `00ec90a8` runs.
pub fn ci_atan(e: &mut Engine, st0: f64) -> f64 {
    if super::sse2_math_available(e) {
        return e.call(ATAN_SSE2_REGISTER, &args![st0]).f64();
    }
    register_x87(e, st0, atan_x87)
}

// Translated from 00ec90a8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `atan`'s x87 body (no label; Ghidra gives it no name), with the inputs of `acos`'s
/// (`fn_00eca228`). A finite argument runs under the control word at `010f2a98` (loaded with
/// `FLDCW` unless the saved word is `0x027f`) and is `FPATAN` of `x` over 1, leaving through
/// `__math_exit` (operation 0xf, `"atan"`). An infinity gives the extended pi/2 (`011c1d0a`),
/// negated for -infinity, through the same exit, without changing the control word; a NaN gets
/// its type from `00ed27ac` and goes to `__startOneArgErrorHandling`.
pub fn fn_00ec90a8(e: &mut Engine, zf: bool, eax: u32, st0: f64, low: u32, high: u32) -> f64 {
    atan_x87(e, zf, eax, ext_of_double(st0.to_bits()), low, high).store64()
}

fn atan_x87(e: &mut Engine, zf: bool, eax: u32, x: X87Ext, low: u32, high: u32) -> X87Ext {
    let (operation, name) = (ATAN_OPERATION, ATAN_NAME);
    let saved = x87_control_word(e);
    if zf {
        // 00ec90e5
        if let Some((kind, result)) = nan_type(e, eax, x, low) {
            return inverse_error(e, kind, result, saved, operation, name, low, high);
        }
        // FSTP ST0; FLD tbyte [011c1d0a]; TEST EAX, 0x80000000; FCHS.
        let mut result = load_extended(e, HALF_PI_EXTENDED);
        if eax & 0x8000_0000 != 0 {
            result = x87_fchs(result);
        }
        return inverse_exit(e, result, saved, operation, name, low, high);
    }
    if saved != 0x027f {
        let word = e.mem.u16(ATAN_CONTROL_WORD);
        set_x87_control_word(e, word);
    }
    // FLD1; FPATAN.
    let result = x87_fpatan(x, X87Ext::load64(1.0));
    inverse_exit(e, result, saved, operation, name, low, high)
}
/// `ANDNPD dest, src`: `!dest & src`.
fn andnpd(dest: Xmm, src: Xmm) -> Xmm {
    [!dest[0] & src[0], !dest[1] & src[1]]
}

/// `CMPLTSD dest, src`: the low lane all ones when `dest < src` (ordered), else zero.
fn cmpltsd(dest: Xmm, src: u64) -> Xmm {
    let less = f64::from_bits(dest[0]) < f64::from_bits(src);
    [if less { u64::MAX } else { 0 }, dest[1]]
}

/// A body's `___libm_error_support(&x, &x, &result, code)` with `XMM0`'s low lane in the result
/// slot (`SUB ESP, 0x1c` and the pointers into it: the argument slot `[ESP+4]` twice), then the
/// slot as the result (`MOVLPD` back, `FLD`).
fn sse2_error_one(e: &mut Engine, argument: u32, xmm0: u64, code: u32) -> f64 {
    e.with_stack(8, |e, slot| {
        let result = slot.addr();
        e.mem.set_u64(result, xmm0);
        e.call(LIBM_ERROR_SUPPORT, &args![argument, argument, result, code]);
        x87_load64(e.mem.u64(result))
    })
}

/// `acos`'s SSE2 constants and tables, named by their addresses (`010fXXXX`): the masks and
/// constants `010fae90` (also `ANDNPD`'s mask), `010faea0`, `010fae98`, `010faea8`, the
/// polynomial coefficients `010fae38`, `010fae40`, `010fae48`, `010fae10` and the pairs
/// `010fae50`, `010fae60`, `010fae70`, `010fadd0` (also two results, the low and high lanes
/// added for a tiny argument) and `010fade0`, the results `010fadf0`/`010fadf8` of `|x| = 1`,
/// and the tables `010fa5d0` (doubles) and `010f96d0` (pairs).
const ACOS_AE90: u32 = 0x010f_ae90;
const ACOS_AEA0: u32 = 0x010f_aea0;
const ACOS_AE98: u32 = 0x010f_ae98;
const ACOS_AEA8: u32 = 0x010f_aea8;
const ACOS_AE38: u32 = 0x010f_ae38;
const ACOS_AE40: u32 = 0x010f_ae40;
const ACOS_AE48: u32 = 0x010f_ae48;
const ACOS_AE10: u32 = 0x010f_ae10;
const ACOS_AE50: u32 = 0x010f_ae50;
const ACOS_AE60: u32 = 0x010f_ae60;
const ACOS_AE70: u32 = 0x010f_ae70;
const ACOS_ADD0: u32 = 0x010f_add0;
const ACOS_ADE0: u32 = 0x010f_ade0;
const ACOS_ADF0: u32 = 0x010f_adf0;
const ACOS_TABLE_DOUBLES: u32 = 0x010f_a5d0;
const ACOS_TABLE_PAIRS: u32 = 0x010f_96d0;

// Translated from 00ed9b00 (disassembled, FalloutNV.exe 1.4.0.525)
/// The register form of `acos`'s SSE2 body (no label; Ghidra gives it no name), `ST0` in: a
/// 16-byte aligned frame, `ST0` stored to it (`FSTP`, a signalling NaN quiet), `MOVQ XMM0` of
/// it, and the core `00ed9b1e` called with the stored double above its return address.
pub fn fn_00ed9b00(e: &mut Engine, st0: f64) -> f64 {
    let stored = x87_load64(st0.to_bits());
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), stored.to_bits());
        acos_sse2(e, slot.addr(), movq(stored.to_bits()))
    })
}

// Translated from 00ed9b18 (disassembled, FalloutNV.exe 1.4.0.525)
/// The stack form of `acos`'s SSE2 body (no label), cdecl: `MOVLPD XMM0, [ESP+4]` (the high
/// lane as the caller left it) and the core `00ed9b1e` that follows.
pub fn fn_00ed9b18(e: &mut Engine, x: f64) -> f64 {
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        acos_sse2(e, slot.addr(), movlpd([0; 2], x.to_bits()))
    })
}

// Translated from 00ed9b1e (disassembled, FalloutNV.exe 1.4.0.525)
/// `acos`'s SSE2 core (no label; Ghidra gives it no name), `XMM0` the argument (registered with
/// the double above the return address, which is the same value). Every instruction is the
/// exe's, in its order. The argument's top bits (`>> 44`, the sign dropped, less `0x3fb00`)
/// pick the path: up to `0x3bb` past it a table pair (`010f96d0`) and double (`010fa5d0`) with
/// `sqrt(1 - x^2)` (`SQRTSD`) and a short polynomial; the next 0x41 steps a reduction through
/// `sqrt` of the complement and the same tables, the sign applied at the end (`XORPD`); small
/// arguments a polynomial from pi/2 (`010fadd0`); the last four steps below 1 a polynomial in
/// `1 - |x|`. A tiny argument gives the sum of `010fadd0`'s two lanes; `|x| = 1` the sum of
/// `010fadf0` and `010fadf8` masked by the sign (0 for 1). A NaN is quieted (`ADDSD` with 0)
/// and goes to `___libm_error_support` with code 0x3f0; any other argument above 1 gives `0 *
/// infinity` (the default NaN) with code 0x3a. The error support gets the argument's slot
/// twice and a result slot, whose value is the result.
pub fn fn_00ed9b1e(e: &mut Engine, x: f64) -> f64 {
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        acos_sse2(e, slot.addr(), movq(x.to_bits()))
    })
}

#[allow(unused_assignments)] // the exe's register writes, kept where a path overwrites them
fn acos_sse2(e: &mut Engine, argument: u32, xmm0: Xmm) -> f64 {
    let m = |e: &Engine, a: u32| e.mem.u64(a);
    let mut x: [Xmm; 8] = [[0; 2]; 8];
    x[0] = xmm0;
    let (mut eax, mut ecx, mut edx): (u32, u32, u32);
    // 00ed9b1e
    x[4] = movlpd(x[4], m(e, ACOS_AE90));
    x[3] = movlpd(x[3], m(e, ACOS_AEA0));
    x[5] = xorpd(x[5], x[5]);
    x[2] = movlpd(x[2], m(e, ACOS_AE98));
    x[1] = movlpd(x[1], x[0][0]);
    x[0] = psrlq(x[0], 0x2c);
    edx = x[0][0] as u32;
    x[7] = movlpd(x[7], x[1][0]);
    ecx = 0x2000;
    x[5] = pinsrw(x[5], ecx, 2);
    x[0] = movlpd(x[0], x[1][0]);
    eax = 0x7_ffff;
    eax &= edx;
    eax = eax.wrapping_sub(0x3_fb00);
    if eax < 0x3bb {
        x[1] = mulsd(x[1], x[1][0]);
        edx &= 0xffff;
        x[3] = subsd(x[3], x[1][0]);
        x[3] = sqrtsd(x[3], x[3][0]);
        x[2] = andpd(x[2], x[7]);
        edx &= 0xffff_fffc;
        edx = edx.wrapping_sub(0xfb00);
        x[1] = movlpd(
            x[1],
            m(e, ACOS_TABLE_DOUBLES.wrapping_add(edx.wrapping_mul(2))),
        );
        x[2] = orpd(x[2], x[5]);
        x[4] = m128(e, ACOS_TABLE_PAIRS.wrapping_add(edx.wrapping_mul(4)));
        x[6] = movlpd(x[6], x[7][0]);
        x[7] = addsd(x[7], x[2][0]);
        x[0] = subsd(x[0], x[2][0]);
        x[7] = mulsd(x[7], x[0][0]);
        x[6] = mulsd(x[6], x[1][0]);
        x[3] = mulsd(x[3], x[2][0]);
        x[1] = movlpd(x[1], x[6][0]);
        x[6] = addsd(x[6], x[3][0]);
        x[7] = divsd(x[7], x[6][0]);
        x[0] = movlpd(x[0], m(e, ACOS_AE48));
        x[5] = movlpd(x[5], m(e, ACOS_AE38));
        x[1] = subsd(x[1], x[3][0]);
        x[2] = psrlq(x[2], 0x3f);
        x[3] = movlpd(x[3], x[1][0]);
        x[2] = psllq(x[2], 0x3f);
        x[1] = mulsd(x[1], x[1][0]);
        x[2] = pshufd(x[2], 0x44);
        x[6] = movlpd(x[6], m(e, ACOS_AE40));
        x[3] = mulsd(x[3], x[1][0]);
        x[0] = mulsd(x[0], x[1][0]);
        x[4] = xorpd(x[4], x[2]);
        x[5] = mulsd(x[5], x[3][0]);
        x[4] = subpd(x[4], m128(e, ACOS_ADD0));
        x[3] = mulsd(x[3], x[1][0]);
        x[0] = addsd(x[0], x[6][0]);
        x[0] = mulsd(x[0], x[3][0]);
        x[5] = subsd(x[5], x[4][0]);
        x[4] = pshufd(x[4], 0xee);
        x[0] = addsd(x[0], x[5][0]);
        x[0] = subsd(x[0], x[7][0]);
        x[0] = subsd(x[0], x[4][0]);
        return x87_load64(x[0][0]);
    }
    // 00ed9c46
    eax = eax.wrapping_sub(0x3bb);
    if eax < 0x41 {
        x[7] = psrlq(x[7], 0x26);
        x[7] = psllq(x[7], 0x26);
        eax = pmovmskb(x[0]);
        x[4] = andnpd(x[4], x[0]);
        x[1] = subsd(x[1], x[7][0]);
        x[6] = movlpd(x[6], x[7][0]);
        x[7] = mulsd(x[7], x[7][0]);
        x[0] = addsd(x[0], x[6][0]);
        x[5] = orpd(x[5], x[4]);
        x[3] = subsd(x[3], x[7][0]);
        x[0] = mulsd(x[0], x[1][0]);
        x[4] = movlpd(x[4], x[3][0]);
        x[3] = subsd(x[3], x[0][0]);
        x[3] = sqrtsd(x[3], x[3][0]);
        eax &= 0x80;
        eax >>= 7;
        eax = eax.wrapping_neg();
        x[7] = movlpd(x[7], x[3][0]);
        x[2] = andpd(x[2], x[3]);
        x[3] = psllq(x[3], 2);
        edx = pextrw(x[3], 3);
        x[2] = orpd(x[2], x[5]);
        x[3] = movd(eax);
        x[3] = pshufd(x[3], 0);
        edx = edx.wrapping_sub(0xfec0);
        edx = edx.wrapping_add(edx);
        x[7] = mulsd(
            x[7],
            m(e, ACOS_TABLE_DOUBLES.wrapping_add(edx.wrapping_mul(4))),
        );
        x[6] = mulsd(x[6], x[2][0]);
        x[1] = mulsd(x[1], x[2][0]);
        x[2] = mulsd(x[2], x[2][0]);
        x[6] = subsd(x[6], x[7][0]);
        x[3] = andpd(x[3], m128(e, ACOS_ADE0));
        x[6] = addsd(x[6], x[1][0]);
        x[4] = subsd(x[4], x[2][0]);
        x[7] = addsd(x[7], x[7][0]);
        x[5] = movlpd(x[5], m(e, ACOS_AE38));
        x[4] = subsd(x[4], x[0][0]);
        x[7] = addsd(x[7], x[6][0]);
        x[0] = movlpd(x[0], m(e, ACOS_AE48));
        x[4] = divsd(x[4], x[7][0]);
        x[2] = movlpd(x[2], m(e, ACOS_AE40));
        x[3] = addpd(
            x[3],
            m128(e, ACOS_TABLE_PAIRS.wrapping_add(edx.wrapping_mul(8))),
        );
        x[1] = movlpd(x[1], x[6][0]);
        x[6] = mulsd(x[6], x[6][0]);
        x[0] = mulsd(x[0], x[6][0]);
        x[1] = mulsd(x[1], x[6][0]);
        x[5] = mulsd(x[5], x[1][0]);
        x[1] = mulsd(x[1], x[6][0]);
        x[0] = addsd(x[0], x[2][0]);
        x[6] = xorpd(x[6], x[6]);
        x[0] = mulsd(x[0], x[1][0]);
        x[5] = addsd(x[5], x[3][0]);
        x[0] = addsd(x[0], x[5][0]);
        eax &= 0x8000;
        x[6] = pinsrw(x[6], eax, 3);
        x[5] = movlpd(x[5], x[4][0]);
        x[3] = pshufd(x[3], 0xee);
        x[4] = addsd(x[4], x[3][0]);
        x[3] = subsd(x[3], x[4][0]);
        x[5] = addsd(x[5], x[3][0]);
        x[0] = addsd(x[0], x[5][0]);
        x[0] = addsd(x[0], x[4][0]);
        x[0] = xorpd(x[0], x[6]);
        return x87_load64(x[0][0]);
    }
    // 00ed9d81
    eax = eax.wrapping_add(0x3bbb);
    if eax < 0x3800 {
        x[0] = unpcklpd(x[0], x[0]);
        x[6] = m128(e, ACOS_AE50);
        x[1] = unpcklpd(x[1], x[0]);
        x[2] = m128(e, ACOS_AE60);
        x[4] = m128(e, ACOS_AE70);
        x[0] = mulpd(x[0], x[0]);
        x[5] = m128(e, ACOS_ADD0);
        x[1] = mulpd(x[1], x[0]);
        x[6] = mulpd(x[6], x[0]);
        x[0] = mulpd(x[0], x[0]);
        x[3] = movlpd(x[3], x[1][0]);
        x[1] = mulsd(x[1], x[1][0]);
        x[6] = addpd(x[6], x[2]);
        x[4] = mulpd(x[4], x[0]);
        x[1] = mulsd(x[1], x[3][0]);
        x[6] = addpd(x[6], x[4]);
        x[0] = pshufd(x[5], 0xee);
        x[1] = mulpd(x[1], x[6]);
        x[6] = pshufd(x[5], 0xee);
        x[0] = subsd(x[0], x[7][0]);
        x[2] = pshufd(x[1], 0xee);
        x[5] = subsd(x[5], x[1][0]);
        x[6] = subsd(x[6], x[0][0]);
        x[5] = subsd(x[5], x[2][0]);
        x[7] = subsd(x[7], x[6][0]);
        x[5] = subsd(x[5], x[7][0]);
        x[0] = addsd(x[0], x[5][0]);
        return x87_load64(x[0][0]);
    }
    // 00ed9e21
    eax = eax.wrapping_sub(0x3bfc);
    if eax < 4 {
        x[6] = xorpd(x[6], x[6]);
        x[7] = andpd(x[7], m128(e, ACOS_AE90));
        x[4] = movlpd(x[4], m(e, ACOS_AEA8));
        x[1] = m128(e, ACOS_AE50);
        x[7] = mulsd(x[7], x[4][0]);
        x[2] = m128(e, ACOS_AE60);
        x[4] = subsd(x[4], x[7][0]);
        x[3] = m128(e, ACOS_AE70);
        x[7] = pshufd(x[4], 0x44);
        x[4] = sqrtsd(x[4], x[4][0]);
        x[1] = mulpd(x[1], x[7]);
        x[5] = pshufd(x[7], 0x44);
        eax = pextrw(x[0], 3);
        x[7] = mulpd(x[7], x[7]);
        x[2] = addpd(x[2], x[1]);
        x[1] = movlpd(x[1], m(e, ACOS_AE10));
        x[3] = mulpd(x[3], x[7]);
        x[0] = cmpltsd(x[0], x[6][0]);
        x[7] = mulsd(x[7], x[5][0]);
        x[2] = addpd(x[2], x[3]);
        x[0] = pshufd(x[0], 0x44);
        x[2] = mulsd(x[2], x[7][0]);
        x[0] = andpd(x[0], m128(e, ACOS_ADE0));
        x[2] = mulpd(x[2], x[5]);
        x[1] = andpd(x[1], x[4]);
        x[3] = pshufd(x[4], 0x44);
        x[4] = subsd(x[4], x[1][0]);
        x[3] = addsd(x[3], x[3][0]);
        x[1] = mulsd(x[1], x[1][0]);
        x[3] = subsd(x[3], x[4][0]);
        x[5] = subsd(x[5], x[1][0]);
        x[4] = mulsd(x[4], x[3][0]);
        x[3] = pshufd(x[3], 0xee);
        x[5] = subsd(x[5], x[4][0]);
        x[5] = divsd(x[5], x[3][0]);
        x[3] = addpd(x[3], x[3]);
        x[2] = mulpd(x[2], x[3]);
        x[4] = pshufd(x[2], 0xee);
        x[2] = addsd(x[2], x[0][0]);
        eax &= 0x8000;
        x[6] = pinsrw(x[6], eax, 3);
        x[0] = pshufd(x[0], 0xee);
        x[2] = addsd(x[2], x[4][0]);
        x[2] = addsd(x[2], x[5][0]);
        x[2] = addsd(x[2], x[3][0]);
        x[0] = addsd(x[0], x[2][0]);
        x[0] = xorpd(x[0], x[6]);
        return x87_load64(x[0][0]);
    }
    // 00ed9f23
    eax = eax.wrapping_add(0x3_fefc);
    if eax < 0x3_ff00 {
        // 00eda01d: a tiny argument, the two lanes of 010fadd0 added.
        x[2] = movlpd(x[2], m(e, ACOS_ADD0));
        x[0] = movlpd(x[0], m(e, ACOS_ADD0 + 8));
        x[0] = addsd(x[0], x[2][0]);
        return x87_load64(x[0][0]);
    }
    ecx = x[7][0] as u32;
    x[7] = psrlq(x[7], 0x20);
    edx = x[7][0] as u32;
    edx &= 0x7fff_ffff;
    eax = 0x3ff0_0000u32.wrapping_sub(edx);
    eax |= ecx;
    if eax == 0 {
        // 00ed9fdd: |x| = 1.
        edx = pextrw(x[7], 1);
        edx >>= 0xf;
        edx = edx.wrapping_neg();
        x[7] = movd(edx);
        x[7] = pshufd(x[7], 0);
        x[2] = movlpd(x[2], m(e, ACOS_ADF0));
        x[0] = movlpd(x[0], m(e, ACOS_ADF0 + 8));
        x[2] = andpd(x[2], x[7]);
        x[0] = andpd(x[0], x[7]);
        x[0] = addsd(x[0], x[2][0]);
        return x87_load64(x[0][0]);
    }
    let code = sse2_nan_or_domain(e, &mut x, argument, 0x3f0, 0x3a);
    sse2_error_one(e, argument, x[0][0], code)
}

/// The shared tail of `acos`'s and `asin`'s SSE2 bodies for `|x| > 1` (`00ed9f58`,
/// `00ed7b8d`): `SUB EDX, 1; SBB ECX, 0x7ff00000` on the argument's words (its magnitude above
/// infinity: a NaN) picks `nan_code` with `XMM0 + 0` (`ADDSD` with a zeroed `XMM6`: the NaN
/// quiet); otherwise `XMM0` becomes `0 * infinity` (the default NaN) and the code `domain_code`.
fn sse2_nan_or_domain(
    e: &Engine,
    x: &mut [Xmm; 8],
    argument: u32,
    nan_code: u32,
    domain_code: u32,
) -> u32 {
    x[2] = movlpd(x[2], e.mem.u64(argument));
    let edx = x[2][0] as u32;
    x[2] = psrlq(x[2], 0x20);
    let ecx = x[2][0] as u32 & 0x7fff_ffff;
    let borrow = u32::from(edx == 0);
    let ecx = ecx.wrapping_sub(0x7ff0_0000).wrapping_sub(borrow);
    if (ecx as i32) >= 0 {
        x[6] = xorpd(x[6], x[6]);
        x[0] = addsd(x[0], x[6][0]);
        return nan_code;
    }
    x[1] = xorpd(x[1], x[1]);
    x[0] = xorpd(x[0], x[0]);
    x[1] = pinsrw(x[1], 0x7ff0, 3);
    x[0] = mulsd(x[0], x[1][0]);
    domain_code
}

/// `asin`'s SSE2 constants and tables, named by their addresses (`010f4XXX`): `010f4bd0` (the
/// magnitude mask, also `ANDNPD`'s), `010f4be0`, `010f4bd8`, `010f4b70`, `010f4b60` (masks and
/// constants), the coefficients `010f4b78`, `010f4b80`, `010f4b88`, the pairs `010f4b40` (pi/2's
/// parts), `010f4b90`, `010f4ba0`, `010f4bb0`, `010f4bc0`, and the tables `010f4340` (doubles)
/// and `010f3440` (pairs).
const ASIN_4BD0: u32 = 0x010f_4bd0;
const ASIN_4BE0: u32 = 0x010f_4be0;
const ASIN_4BD8: u32 = 0x010f_4bd8;
const ASIN_4B70: u32 = 0x010f_4b70;
const ASIN_4B60: u32 = 0x010f_4b60;
const ASIN_4B78: u32 = 0x010f_4b78;
const ASIN_4B80: u32 = 0x010f_4b80;
const ASIN_4B88: u32 = 0x010f_4b88;
const ASIN_4B40: u32 = 0x010f_4b40;
const ASIN_4B90: u32 = 0x010f_4b90;
const ASIN_4BA0: u32 = 0x010f_4ba0;
const ASIN_4BB0: u32 = 0x010f_4bb0;
const ASIN_4BC0: u32 = 0x010f_4bc0;
const ASIN_TABLE_DOUBLES: u32 = 0x010f_4340;
const ASIN_TABLE_PAIRS: u32 = 0x010f_3440;

// Translated from 00ed7720 (disassembled, FalloutNV.exe 1.4.0.525)
/// The register form of `asin`'s SSE2 body (no label; Ghidra gives it no name; `_CIasin` in
/// `crt.rs` jumps here), `ST0` in: as `acos`'s (`fn_00ed9b00`), with the core `00ed773e`.
pub fn fn_00ed7720(e: &mut Engine, st0: f64) -> f64 {
    let stored = x87_load64(st0.to_bits());
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), stored.to_bits());
        asin_sse2(e, slot.addr(), movq(stored.to_bits()))
    })
}

// Translated from 00ed7738 (disassembled, FalloutNV.exe 1.4.0.525)
/// The stack form of `asin`'s SSE2 body (no label), cdecl: `MOVLPD XMM0, [ESP+4]` and the core
/// `00ed773e` that follows.
pub fn fn_00ed7738(e: &mut Engine, x: f64) -> f64 {
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        asin_sse2(e, slot.addr(), movlpd([0; 2], x.to_bits()))
    })
}

// Translated from 00ed773e (disassembled, FalloutNV.exe 1.4.0.525)
/// `asin`'s SSE2 core (no label), `XMM0` the argument (registered with the double above the
/// return address). The same structure as `acos`'s core on its own constants (`010f4b40` ..
/// `010f4be0`) and tables (`010f4340`, `010f3440`): a table path up to `0x3bb` steps past
/// exponent 0x3fb, a reduction through `sqrt(1 - x^2)` for the next 0x43 (the sign put back with
/// `ORPD`), a polynomial for small arguments, one in the complement for the last two steps
/// below 1; a tiny or denormal argument comes back as it is (`ORPD` with `0 + x`, `MULSD` raising
/// underflow); `|x| = 1` gives pi/2 (`010f4b40`'s lanes added) with the argument's sign
/// (`ANDNPD` with `010f4bd0`); a NaN goes to the error support with code 0x3f1, any other
/// argument above 1 with `0 * infinity` and code 0x3d.
pub fn fn_00ed773e(e: &mut Engine, x: f64) -> f64 {
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        asin_sse2(e, slot.addr(), movq(x.to_bits()))
    })
}

#[allow(unused_assignments)] // the exe's register writes, kept where a path overwrites them
fn asin_sse2(e: &mut Engine, argument: u32, xmm0: Xmm) -> f64 {
    let m = |e: &Engine, a: u32| e.mem.u64(a);
    let mut x: [Xmm; 8] = [[0; 2]; 8];
    x[0] = xmm0;
    let (mut eax, mut ecx, mut edx): (u32, u32, u32);
    // 00ed773e
    x[4] = movlpd(x[4], m(e, ASIN_4BD0));
    x[3] = movlpd(x[3], m(e, ASIN_4BE0));
    x[5] = xorpd(x[5], x[5]);
    x[2] = movlpd(x[2], m(e, ASIN_4BD8));
    x[1] = movlpd(x[1], x[0][0]);
    x[0] = psrlq(x[0], 0x2c);
    edx = x[0][0] as u32;
    x[7] = movlpd(x[7], x[1][0]);
    ecx = 0x2000;
    x[5] = pinsrw(x[5], ecx, 2);
    x[0] = movlpd(x[0], x[1][0]);
    eax = 0x7_ffff;
    eax &= edx;
    eax = eax.wrapping_sub(0x3_fb00);
    if eax < 0x3bb {
        x[1] = mulsd(x[1], x[1][0]);
        edx &= 0xffff;
        x[3] = subsd(x[3], x[1][0]);
        x[3] = sqrtsd(x[3], x[3][0]);
        x[2] = andpd(x[2], x[7]);
        edx &= 0xffff_fffc;
        edx = edx.wrapping_sub(0xfb00);
        x[1] = movlpd(
            x[1],
            m(e, ASIN_TABLE_DOUBLES.wrapping_add(edx.wrapping_mul(2))),
        );
        x[2] = orpd(x[2], x[5]);
        x[4] = m128(e, ASIN_TABLE_PAIRS.wrapping_add(edx.wrapping_mul(4)));
        x[6] = movlpd(x[6], x[7][0]);
        x[7] = addsd(x[7], x[2][0]);
        x[0] = subsd(x[0], x[2][0]);
        x[0] = mulsd(x[0], x[7][0]);
        x[6] = mulsd(x[6], x[1][0]);
        x[3] = mulsd(x[3], x[2][0]);
        x[1] = movlpd(x[1], x[6][0]);
        x[6] = addsd(x[6], x[3][0]);
        x[0] = divsd(x[0], x[6][0]);
        x[7] = movlpd(x[7], m(e, ASIN_4B88));
        x[5] = movlpd(x[5], m(e, ASIN_4B78));
        x[1] = subsd(x[1], x[3][0]);
        x[2] = psrlq(x[2], 0x3f);
        x[3] = movlpd(x[3], x[1][0]);
        x[2] = psllq(x[2], 0x3f);
        x[1] = mulsd(x[1], x[1][0]);
        x[2] = pshufd(x[2], 0x44);
        x[6] = movlpd(x[6], m(e, ASIN_4B80));
        x[3] = mulsd(x[3], x[1][0]);
        x[7] = mulsd(x[7], x[1][0]);
        x[4] = xorpd(x[4], x[2]);
        x[5] = mulsd(x[5], x[3][0]);
        x[3] = mulsd(x[3], x[1][0]);
        x[6] = addsd(x[6], x[7][0]);
        x[6] = mulsd(x[6], x[3][0]);
        x[5] = addsd(x[5], x[4][0]);
        x[4] = pshufd(x[4], 0xee);
        x[6] = addsd(x[6], x[5][0]);
        x[0] = addsd(x[0], x[6][0]);
        x[0] = addsd(x[0], x[4][0]);
        return x87_load64(x[0][0]);
    }
    // 00ed785e
    eax = eax.wrapping_sub(0x3bb);
    if eax < 0x43 {
        x[1] = mulsd(x[1], x[1][0]);
        x[3] = subsd(x[3], x[1][0]);
        x[3] = sqrtsd(x[3], x[3][0]);
        eax = pmovmskb(x[7]);
        x[0] = psllq(x[0], 1);
        x[7] = andpd(x[7], m128(e, ASIN_4B70));
        x[0] = psrlq(x[0], 1);
        x[1] = movlpd(x[1], x[0][0]);
        x[4] = movlpd(x[4], m(e, ASIN_4BE0));
        x[6] = movlpd(x[6], x[7][0]);
        x[1] = subsd(x[1], x[7][0]);
        x[7] = mulsd(x[7], x[7][0]);
        x[0] = addsd(x[0], x[6][0]);
        x[4] = subsd(x[4], x[7][0]);
        x[0] = mulsd(x[0], x[1][0]);
        x[7] = movlpd(x[7], x[3][0]);
        x[2] = andpd(x[2], x[3]);
        x[3] = psllq(x[3], 2);
        edx = pextrw(x[3], 3);
        x[2] = orpd(x[2], x[5]);
        edx = edx.wrapping_sub(0xfec0);
        edx = edx.wrapping_add(edx);
        x[7] = mulsd(
            x[7],
            m(e, ASIN_TABLE_DOUBLES.wrapping_add(edx.wrapping_mul(4))),
        );
        x[6] = mulsd(x[6], x[2][0]);
        x[3] = m128(e, ASIN_4B40);
        x[1] = mulsd(x[1], x[2][0]);
        x[2] = mulsd(x[2], x[2][0]);
        x[6] = subsd(x[6], x[7][0]);
        x[6] = addsd(x[6], x[1][0]);
        x[4] = subsd(x[4], x[2][0]);
        x[7] = addsd(x[7], x[7][0]);
        x[5] = movlpd(x[5], m(e, ASIN_4B78));
        x[4] = subsd(x[4], x[0][0]);
        x[7] = addsd(x[7], x[6][0]);
        x[0] = movlpd(x[0], m(e, ASIN_4B88));
        x[4] = divsd(x[4], x[7][0]);
        x[2] = movlpd(x[2], m(e, ASIN_4B80));
        x[3] = subpd(
            x[3],
            m128(e, ASIN_TABLE_PAIRS.wrapping_add(edx.wrapping_mul(8))),
        );
        x[1] = movlpd(x[1], x[6][0]);
        x[6] = mulsd(x[6], x[6][0]);
        eax &= 0x80;
        eax <<= 8;
        x[0] = mulsd(x[0], x[6][0]);
        x[1] = mulsd(x[1], x[6][0]);
        x[5] = mulsd(x[5], x[1][0]);
        x[1] = mulsd(x[1], x[6][0]);
        x[0] = addsd(x[0], x[2][0]);
        x[6] = xorpd(x[6], x[6]);
        x[0] = mulsd(x[0], x[1][0]);
        x[5] = addsd(x[5], x[3][0]);
        x[6] = pinsrw(x[6], eax, 3);
        x[0] = addsd(x[0], x[5][0]);
        x[3] = pshufd(x[3], 0xee);
        x[0] = subsd(x[0], x[4][0]);
        x[0] = addsd(x[0], x[3][0]);
        x[0] = orpd(x[0], x[6]);
        return x87_load64(x[0][0]);
    }
    // 00ed7985
    eax = eax.wrapping_add(0x3bbb);
    if eax < 0x3800 {
        x[7] = unpcklpd(x[7], x[7]);
        x[6] = m128(e, ASIN_4B90);
        x[1] = unpcklpd(x[1], x[7]);
        x[2] = m128(e, ASIN_4BA0);
        x[4] = m128(e, ASIN_4BB0);
        x[7] = mulpd(x[7], x[7]);
        x[1] = mulpd(x[1], x[7]);
        x[6] = mulpd(x[6], x[7]);
        x[7] = mulpd(x[7], x[7]);
        x[3] = movlpd(x[3], x[1][0]);
        x[1] = mulsd(x[1], x[1][0]);
        x[6] = addpd(x[6], x[2]);
        x[4] = mulpd(x[4], x[7]);
        x[1] = mulsd(x[1], x[3][0]);
        x[6] = addpd(x[6], x[4]);
        x[1] = mulpd(x[1], x[6]);
        x[2] = pshufd(x[1], 0xee);
        x[1] = addsd(x[1], x[2][0]);
        x[0] = addsd(x[0], x[1][0]);
        return x87_load64(x[0][0]);
    }
    // 00ed79fb
    eax = eax.wrapping_sub(0x3bfe);
    if eax < 2 {
        x[1] = mulsd(x[1], x[1][0]);
        x[3] = subsd(x[3], x[1][0]);
        x[3] = sqrtsd(x[3], x[3][0]);
        eax = pextrw(x[7], 3);
        x[7] = andpd(x[7], m128(e, ASIN_4B60));
        x[5] = pshufd(x[3], 0x44);
        x[3] = andpd(x[3], m128(e, ASIN_4B60));
        x[1] = movlpd(x[1], x[7][0]);
        x[4] = movlpd(x[4], m(e, ASIN_4BE0));
        x[6] = movlpd(x[6], x[7][0]);
        x[0] = subsd(x[0], x[7][0]);
        x[7] = mulsd(x[7], x[7][0]);
        x[1] = addsd(x[1], x[1][0]);
        x[1] = mulsd(x[1], x[0][0]);
        x[4] = subsd(x[4], x[7][0]);
        x[6] = movlpd(x[6], x[3][0]);
        x[3] = mulsd(x[3], x[3][0]);
        x[0] = mulsd(x[0], x[0][0]);
        x[4] = subsd(x[4], x[1][0]);
        x[6] = subsd(x[6], x[5][0]);
        x[5] = addsd(x[5], x[5][0]);
        x[4] = subsd(x[4], x[3][0]);
        x[2] = m128(e, ASIN_4B90);
        x[3] = pshufd(x[5], 0xee);
        x[4] = subsd(x[4], x[0][0]);
        x[5] = addsd(x[5], x[6][0]);
        x[7] = pshufd(x[3], 0xee);
        x[3] = addsd(x[3], x[3][0]);
        x[5] = mulsd(x[5], x[6][0]);
        x[4] = addsd(x[4], x[5][0]);
        x[6] = pshufd(x[7], 0xee);
        x[4] = divsd(x[4], x[3][0]);
        x[1] = m128(e, ASIN_4BC0);
        x[5] = m128(e, ASIN_4BA0);
        x[0] = m128(e, ASIN_4BB0);
        x[7] = mulpd(x[7], x[7]);
        x[3] = movlpd(x[3], x[6][0]);
        x[2] = mulpd(x[2], x[7]);
        x[6] = mulpd(x[6], x[7]);
        eax &= 0x8000;
        x[1] = mulsd(x[1], x[7][0]);
        x[7] = mulpd(x[7], x[7]);
        x[5] = addpd(x[5], x[2]);
        x[2] = x[6];
        x[6] = mulsd(x[6], x[6][0]);
        x[7] = mulpd(x[7], x[0]);
        x[0] = m128(e, ASIN_4B40);
        x[2] = mulsd(x[2], x[6][0]);
        x[7] = addpd(x[7], x[5]);
        x[5] = pshufd(x[1], 0xee);
        x[6] = mulsd(x[6], x[2][0]);
        x[7] = mulpd(x[7], x[2]);
        x[1] = addsd(x[1], x[5][0]);
        x[5] = xorpd(x[5], x[5]);
        x[2] = pshufd(x[7], 0xee);
        x[1] = mulsd(x[1], x[6][0]);
        x[6] = pshufd(x[0], 0xee);
        x[7] = addsd(x[7], x[2][0]);
        x[2] = movlpd(x[2], x[3][0]);
        x[5] = pinsrw(x[5], eax, 3);
        x[3] = subsd(x[3], x[6][0]);
        x[0] = addsd(x[0], x[1][0]);
        x[6] = addsd(x[6], x[3][0]);
        x[7] = addsd(x[7], x[4][0]);
        x[2] = subsd(x[2], x[6][0]);
        x[0] = subsd(x[0], x[7][0]);
        x[0] = subsd(x[0], x[2][0]);
        x[0] = subsd(x[0], x[3][0]);
        x[0] = orpd(x[0], x[5]);
        return x87_load64(x[0][0]);
    }
    // 00ed7b58
    eax = eax.wrapping_add(0x3_fefe);
    if eax < 0x3_ff00 {
        // 00ed7c5f: a tiny argument.
        x[0] = movlpd(x[0], m(e, argument));
        x[6] = xorpd(x[6], x[6]);
        x[7] = movlpd(x[7], x[0][0]);
        edx = pextrw(x[0], 3);
        edx &= 0x7ff0;
        edx = edx.wrapping_sub(0x10);
        if edx >= 0x7fe0 {
            // A zero or denormal: ORPD with 0 + x; MULSD XMM7, XMM0 raises underflow.
            x[6] = addsd(x[6], x[0][0]);
            x[0] = orpd(x[0], x[6]);
            x[7] = mulsd(x[7], x[0][0]);
        }
        return x87_load64(x[0][0]);
    }
    ecx = x[0][0] as u32;
    x[0] = psrlq(x[0], 0x20);
    edx = x[0][0] as u32;
    edx &= 0x7fff_ffff;
    eax = 0x3ff0_0000u32.wrapping_sub(edx);
    eax |= ecx;
    if eax == 0 {
        // 00ed7c12: |x| = 1: pi/2 with the argument's sign.
        x[1] = movlpd(x[1], m(e, ASIN_4BD0));
        x[2] = movlpd(x[2], m(e, ASIN_4B40));
        x[0] = movlpd(x[0], m(e, ASIN_4B40 + 8));
        x[0] = addsd(x[0], x[2][0]);
        x[1] = andnpd(x[1], x[7]);
        x[0] = orpd(x[0], x[1]);
        return x87_load64(x[0][0]);
    }
    // 00ed7b8d: MOVLPD XMM2, [ESP+4]; SUB EDX, 1; SBB ECX, 0x7ff00000: above infinity is a NaN.
    x[2] = movlpd(x[2], m(e, argument));
    let low = x[2][0] as u32;
    let high = (x[2][0] >> 32) as u32 & 0x7fff_ffff;
    let borrow = u32::from(low == 0);
    let nan = (high.wrapping_sub(0x7ff0_0000).wrapping_sub(borrow) as i32) >= 0;
    let code = if nan {
        // 00ed7c47: MOVLPD XMM0, [ESP+4]; ADDSD with a zeroed XMM6.
        x[0] = movlpd(x[0], m(e, argument));
        x[6] = xorpd(x[6], x[6]);
        x[0] = addsd(x[0], x[6][0]);
        0x3f1
    } else {
        x[1] = xorpd(x[1], x[1]);
        x[0] = xorpd(x[0], x[0]);
        x[1] = pinsrw(x[1], 0x7ff0, 3);
        x[0] = mulsd(x[0], x[1][0]);
        0x3d
    };
    sse2_error_one(e, argument, x[0][0], code)
}

/// `atan`'s SSE2 constants (`010f3XXX`): the magnitude mask `010f32a0`, the thresholds
/// `010f33b0`, `010f33b8`, `010f33c0`, `010f33c8`, `010f33d0` and `010f33a8`, the polynomial pairs
/// `010f3340` .. `010f3370` (four) and `010f32c0` .. `010f3330` (eight), `010f33a0` (the addend of
/// the table index), `010f3380` (the index base, a `PSUBD` operand), `010f3398` and `010f3390`
/// (the reduction's constants), `010f3388` (zero), `010f33d8` (the tiny argument's operand, a
/// double the x87 squares), `010f3290` (the infinity mask) and the pairs `010f32b0` (+-pi/2 by
/// the sign); the tables `010ff040` (three doubles a row: the result's high part, its low part,
/// and the reduction point).
const ATAN_32A0: u32 = 0x010f_32a0;
const ATAN_33B0: u32 = 0x010f_33b0;
const ATAN_33B8: u32 = 0x010f_33b8;
const ATAN_33C0: u32 = 0x010f_33c0;
const ATAN_33C8: u32 = 0x010f_33c8;
const ATAN_33D0: u32 = 0x010f_33d0;
const ATAN_33A8: u32 = 0x010f_33a8;
const ATAN_33A0: u32 = 0x010f_33a0;
const ATAN_3380: u32 = 0x010f_3380;
const ATAN_3398: u32 = 0x010f_3398;
const ATAN_3390: u32 = 0x010f_3390;
const ATAN_3388: u32 = 0x010f_3388;
const ATAN_33D8: u32 = 0x010f_33d8;
const ATAN_3290: u32 = 0x010f_3290;
const ATAN_32B0: u32 = 0x010f_32b0;
const ATAN_TABLE: u32 = 0x010f_f040;
/// The two polynomials' pairs, the highest first: `010f3370` .. `010f3340` and `010f3330` ..
/// `010f32c0`.
const ATAN_SHORT: [u32; 4] = [0x010f_3370, 0x010f_3360, 0x010f_3350, 0x010f_3340];
const ATAN_LONG: [u32; 8] = [
    0x010f_3330,
    0x010f_3320,
    0x010f_3310,
    0x010f_3300,
    0x010f_32f0,
    0x010f_32e0,
    0x010f_32d0,
    0x010f_32c0,
];

// Translated from 00ed5d80 (disassembled, FalloutNV.exe 1.4.0.525)
/// The register form of `atan`'s SSE2 body (no label; Ghidra gives it no name), `ST0` in: a
/// 16-byte aligned frame, `ST0` stored (`FSTP`, a signalling NaN quiet), `MOVQ XMM7` of it, and
/// the core `00ed5d9e` called with the stored double above its return address.
pub fn fn_00ed5d80(e: &mut Engine, st0: f64) -> f64 {
    let stored = x87_load64(st0.to_bits());
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), stored.to_bits());
        atan_sse2(e, slot.addr(), movq(stored.to_bits())).store64()
    })
}

// Translated from 00ed5d98 (disassembled, FalloutNV.exe 1.4.0.525)
/// The stack form of `atan`'s SSE2 body (no label), cdecl: `MOVQ XMM7, [ESP+4]` and the core
/// `00ed5d9e` that follows.
pub fn fn_00ed5d98(e: &mut Engine, x: f64) -> f64 {
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        atan_sse2(e, slot.addr(), movq(x.to_bits())).store64()
    })
}

// Translated from 00ed5d9e (disassembled, FalloutNV.exe 1.4.0.525)
/// `atan`'s SSE2 core (no label), `XMM7` the argument (registered with the double above the
/// return address). The magnitude (`ANDPD` with `010f32a0`) is compared (`COMISD`) with the
/// thresholds: a NaN (unordered) goes to `___libm_error_support` with code 0x3eb and the
/// argument's slot for all three pointers, whose value is the result; at or above `010f33c8`
/// the result is `010f33d8` plus +-pi/2 by the sign (`FLD`/`FADD` on the x87; an infinity masked
/// by `010f3290` compares unordered only for a NaN); below `010f33b8` and at or above `010f33c0`
/// the short polynomial `x - x * p(x^2)`; between `010f33b8` and `010f33b0` the long one; from
/// `010f33b0` a reduction: below `010f33a8` the table row picked from the magnitude's top bits
/// (`(|x| + 010f33a0) >> 44 - 010f3380`), `(|x| - c) / (|x| * c + 010f3398)`, else `010f3390 /
/// |x|` with the last row (index 0x300), then the short polynomial on that, the row's parts
/// added and the sign put back (`ORPD`). Below `010f33c0`: zero (`010f3388`) comes back as it is
/// (`FLD` of the argument); otherwise the x87 computes `010f33d8 * 010f33d8 + x`, the product
/// stored as a double first when the magnitude is below `010f33d0`. The x87 results are
/// extended (`ST0`).
pub fn fn_00ed5d9e(e: &mut Engine, x: f64) -> f64 {
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        atan_sse2(e, slot.addr(), movq(x.to_bits())).store64()
    })
}

/// The short polynomial of `atan`'s core on `XMM2` (`|x|` or the reduced value, both lanes):
/// `x^2` and `x^4` by `MULPD`, the four pairs by Horner, the lanes added (`SHUFPD`, `ADDSD`):
/// the correction in `XMM5`'s low lane.
fn atan_polynomial(e: &Engine, x: &mut [Xmm; 8], pairs: &[u32]) {
    x[1] = x[2];
    x[1] = mulpd(x[1], x[2]);
    x[3] = x[1];
    x[3] = mulpd(x[3], x[1]);
    x[5] = m128(e, pairs[0]);
    for &pair in &pairs[1..] {
        x[5] = mulpd(x[5], x[3]);
        x[5] = addpd(x[5], m128(e, pair));
    }
    x[5] = mulsd(x[5], x[1][0]);
    x[3] = x[5];
    x[3] = shufpd(x[3], x[3], 1);
    x[5] = addsd(x[5], x[3][0]);
}

fn atan_sse2(e: &mut Engine, argument: u32, xmm7: Xmm) -> X87Ext {
    let m = |e: &Engine, a: u32| e.mem.u64(a);
    let mut x: [Xmm; 8] = [[0; 2]; 8];
    x[7] = xmm7;
    // 00ed5d9e: UNPCKLPD XMM7, XMM7; MOVAPD XMM2, XMM7; ANDPD XMM2, [010f32a0].
    x[7] = unpcklpd(x[7], x[7]);
    x[2] = x[7];
    x[2] = andpd(x[2], m128(e, ATAN_32A0));
    let c = comisd(x[2][0], m(e, ATAN_33C8));
    if c.pf {
        return atan_nan(e, argument);
    }
    if !c.cf {
        // 00ed6017: the infinity test.
        x[0] = movq(x[2][0]);
        x[3] = movq(m(e, ATAN_3290));
        x[0] = andpd(x[0], x[3]);
        if comisd(x[0][0], x[3][0]).pf {
            return atan_nan(e, argument);
        }
        // MOV EAX, [ESP+8]; SHR EAX, 0x1f; FLD [010f33d8]; FADD [EAX*8 + 010f32b0].
        let sign = e.mem.u32(argument + 4) >> 31;
        let base = load_double(e, ATAN_33D8);
        return x87_fadd(base, double_operand(e, ATAN_32B0 + sign * 8));
    }
    if !comisd(x[2][0], m(e, ATAN_33B8)).cf {
        // 00ed5e3a
        if comisd(x[2][0], m(e, ATAN_33B0)).cf {
            atan_polynomial(e, &mut x, &ATAN_LONG);
            x[5] = mulsd(x[5], x[7][0]);
            x[7] = subsd(x[7], x[5][0]);
            return X87Ext::load64(f64::from_bits(x[7][0]));
        }
        // 00ed5ed8: the reduction; XMM6 keeps the sign.
        x[6] = movq(x[7][0]);
        x[6] = xorpd(x[6], x[2]);
        let eax = if comisd(x[2][0], m(e, ATAN_33A8)).cf {
            x[0] = movq(m(e, ATAN_33A0));
            x[5] = movq(m(e, ATAN_3380));
            x[3] = movq(x[2][0]);
            x[3] = addsd(x[3], x[0][0]);
            x[3] = psrlq(x[3], 0x2c);
            x[3] = psubd(x[3], x[5]);
            let eax = (x[3][0] as u32).wrapping_mul(3);
            x[5] = movq(m(e, ATAN_TABLE.wrapping_add(eax.wrapping_mul(8)) + 0x10));
            x[3] = movq(x[2][0]);
            x[2] = subsd(x[2], x[5][0]);
            x[3] = mulsd(x[3], x[5][0]);
            x[3] = addsd(x[3], m(e, ATAN_3398));
            x[2] = divsd(x[2], x[3][0]);
            x[2] = unpcklpd(x[2], x[2]);
            eax
        } else {
            // 00ed5f39
            x[0] = movq(x[2][0]);
            x[2] = movq(m(e, ATAN_3390));
            x[2] = divsd(x[2], x[0][0]);
            x[2] = unpcklpd(x[2], x[2]);
            0x300
        };
        // 00ed5f52
        let row = ATAN_TABLE.wrapping_add(eax.wrapping_mul(8));
        x[0] = movq(m(e, row));
        x[4] = movq(m(e, row + 8));
        atan_polynomial(e, &mut x, &ATAN_SHORT);
        x[5] = mulsd(x[5], x[2][0]);
        x[5] = subsd(x[5], x[4][0]);
        x[5] = subsd(x[5], x[2][0]);
        x[0] = subsd(x[0], x[5][0]);
        x[0] = orpd(x[0], x[6]);
        return X87Ext::load64(f64::from_bits(x[0][0]));
    }
    if !comisd(x[2][0], m(e, ATAN_33C0)).cf {
        // 00ed5dda
        atan_polynomial(e, &mut x, &ATAN_SHORT);
        x[5] = mulsd(x[5], x[7][0]);
        x[7] = subsd(x[7], x[5][0]);
        return X87Ext::load64(f64::from_bits(x[7][0]));
    }
    // 00ed5fd0
    if comisd(x[2][0], m(e, ATAN_3388)).zf {
        return load_double(e, argument);
    }
    let square = x87_fmul(load_double(e, ATAN_33D8), double_operand(e, ATAN_33D8));
    let argument_operand = double_operand(e, argument);
    if comisd(x[2][0], m(e, ATAN_33D0)).cf {
        // 00ed5fe9: the product stored as a double and loaded again.
        let stored = X87Ext::load64(square.store64());
        return x87_fadd(stored, argument_operand);
    }
    x87_fadd(square, argument_operand)
}

/// `atan`'s NaN exit (`00ed6042`): `___libm_error_support` with code 0x3eb and the argument's
/// slot for all three pointers, then that slot (`FLD`).
fn atan_nan(e: &mut Engine, argument: u32) -> X87Ext {
    e.call(
        LIBM_ERROR_SUPPORT,
        &args![argument, argument, argument, 0x3ebu32],
    );
    load_double(e, argument)
}

// ---- exp ----------------------------------------------------------------------------------------

/// `PADDQ`.
fn paddq(a: Xmm, b: Xmm) -> Xmm {
    [a[0].wrapping_add(b[0]), a[1].wrapping_add(b[1])]
}

/// `UNPCKHPD dest, src`: the two high lanes.
fn unpckhpd(dest: Xmm, src: Xmm) -> Xmm {
    [dest[1], src[1]]
}

/// `exp`'s SSE2 constants (`010fcXXX`, `010fdXXX`): the pairs `010fcee0` (the scale factor),
/// `010fcec0` (the rounding shifter), `010fcef0` and `010fcf00` (the reduction's two parts),
/// `010fcf10`, `010fcf20` (the polynomial), `010fcea0` (the mask of the exponent part),
/// `010fceb0` (its bias), `010fce90` (the exponent mask `PANDN` clears); the doubles
/// `010fd330` (added to a tiny argument), `010fd350` and `010fd358` (the results of +-infinity),
/// `010fd360` and `010fd368` (squared for an overflow and an underflow); and the table of pairs
/// `010fcf30` (64 rows).
const EXP_CEE0: u32 = 0x010f_cee0;
const EXP_CEC0: u32 = 0x010f_cec0;
const EXP_CEF0: u32 = 0x010f_cef0;
const EXP_CF00: u32 = 0x010f_cf00;
const EXP_CF10: u32 = 0x010f_cf10;
const EXP_CF20: u32 = 0x010f_cf20;
const EXP_CEA0: u32 = 0x010f_cea0;
const EXP_CEB0: u32 = 0x010f_ceb0;
const EXP_CE90: u32 = 0x010f_ce90;
const EXP_D330: u32 = 0x010f_d330;
const EXP_D350: u32 = 0x010f_d350;
const EXP_D358: u32 = 0x010f_d358;
const EXP_D360: u32 = 0x010f_d360;
const EXP_D368: u32 = 0x010f_d368;
const EXP_TABLE: u32 = 0x010f_cf30;

// Translated from 00edbd50 (disassembled, FalloutNV.exe 1.4.0.525)
/// The register form of `exp`'s SSE2 body (no label; Ghidra gives it no name), `ST0` in: a
/// 16-byte aligned frame, `ST0` stored (`FSTP`, a signalling NaN quiet), `MOVQ XMM0` of it, and
/// the core `00edbd6e` called with the stored double above its return address.
pub fn fn_00edbd50(e: &mut Engine, st0: f64) -> f64 {
    let stored = x87_load64(st0.to_bits());
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), stored.to_bits());
        exp_sse2(e, slot.addr(), movq(stored.to_bits()))
    })
}

// Translated from 00edbd68 (disassembled, FalloutNV.exe 1.4.0.525)
/// The stack form of `exp`'s SSE2 body (no label), cdecl: `MOVLPD XMM0, [ESP+4]` and the core
/// `00edbd6e` that follows.
pub fn fn_00edbd68(e: &mut Engine, x: f64) -> f64 {
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        exp_sse2(e, slot.addr(), movlpd([0; 2], x.to_bits()))
    })
}

// Translated from 00edbd6e (disassembled, FalloutNV.exe 1.4.0.525)
/// `exp`'s SSE2 core (no label), `XMM0` the argument (registered with the double above the return
/// address). Every instruction is the exe's, in its order. For a magnitude between the top
/// words `0x3c90` and `0x408f`: `x * 010fcee0` rounded by the shifter `010fcec0` gives the
/// integer `k`, the reduced argument is `x - k * 010fcef0 - k * 010fcf00`, its polynomial
/// (`010fcf10`, `010fcf20`) is added to the row `k & 0x3f` of `010fcf30`, and the row's high part
/// with the exponent `k >> 6` (masked by `010fcea0`, biased by `010fceb0`, shifted to the exponent
/// field) scales it. A scale beyond `0x37e` either way runs the last steps on the x87 at 64-bit
/// precision (the caller's rounding: `FSTCW`, `OR 0x300`) with the exponent split in two, the
/// product stored as a double; an infinite or zero result there is error 0xe (overflow) or 0xf
/// (underflow). Below `0x3c90` the result is `1 + x` (`010fd330`); from the top word `0x4090`:
/// overflow (`010fd360` squared, 0xe) for a positive argument, underflow (`010fd368` squared,
/// 0xf) for a negative one, +infinity `010fd350` and 0 `010fd358` for the infinities, and a NaN
/// as it is with 0x3ea. Errors go to `___libm_error_support` with the argument's slot twice and
/// a result slot holding `XMM0`'s low lane, which is then the result. (The bytes at
/// `00edbf08`..`00edbf26`, after an unconditional jump, are never jumped to.)
pub fn fn_00edbd6e(e: &mut Engine, x: f64) -> f64 {
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        exp_sse2(e, slot.addr(), movq(x.to_bits()))
    })
}

fn exp_sse2(e: &mut Engine, argument: u32, xmm0: Xmm) -> f64 {
    let mut x: [Xmm; 8] = [[0; 2]; 8];
    x[0] = xmm0;
    // 00edbd6e
    x[0] = unpcklpd(x[0], x[0]);
    x[1] = m128(e, EXP_CEE0);
    x[6] = m128(e, EXP_CEC0);
    x[2] = m128(e, EXP_CEF0);
    x[3] = m128(e, EXP_CF00);
    let mut eax = pextrw(x[0], 3) & 0x7fff;
    let mut edx = 0x408fu32.wrapping_sub(eax);
    eax = eax.wrapping_sub(0x3c90);
    edx |= eax;
    if edx >= 0x8000_0000 {
        return exp_sse2_special(e, &mut x, argument);
    }
    x[1] = mulpd(x[1], x[0]);
    x[1] = addpd(x[1], x[6]);
    x[7] = x[1];
    x[1] = subpd(x[1], x[6]);
    x[2] = mulpd(x[2], x[1]);
    x[4] = m128(e, EXP_CF10);
    x[3] = mulpd(x[3], x[1]);
    x[5] = m128(e, EXP_CF20);
    x[0] = subpd(x[0], x[2]);
    eax = x[7][0] as u32;
    let mut ecx = eax & 0x3f;
    ecx <<= 4;
    eax = ((eax as i32) >> 6) as u32;
    edx = eax;
    x[0] = subpd(x[0], x[3]);
    x[2] = m128(e, EXP_TABLE.wrapping_add(ecx));
    x[4] = mulpd(x[4], x[0]);
    x[1] = x[0];
    x[0] = mulpd(x[0], x[0]);
    x[5] = addpd(x[5], x[4]);
    x[0] = mulsd(x[0], x[0][0]);
    x[1] = addsd(x[1], x[2][0]);
    x[2] = unpckhpd(x[2], x[2]);
    x[6] = m128(e, EXP_CEA0);
    x[7] = andpd(x[7], x[6]);
    x[6] = m128(e, EXP_CEB0);
    x[7] = paddq(x[7], x[6]);
    x[7] = psllq(x[7], 0x2e);
    x[0] = mulpd(x[0], x[5]);
    x[1] = addsd(x[1], x[0][0]);
    x[2] = orpd(x[2], x[7]);
    x[0] = unpckhpd(x[0], x[0]);
    x[0] = addsd(x[0], x[1][0]);
    edx = edx.wrapping_add(0x37e);
    if edx <= 0x77c {
        x[0] = mulsd(x[0], x[2][0]);
        x[0] = addsd(x[0], x[2][0]);
        return x87_load64(x[0][0]);
    }
    // 00edbe73: FSTCW; OR DX, 0x300; FLDCW: 64-bit precision, the caller's rounding.
    let saved = x87_control_word(e);
    set_x87_control_word(e, saved | 0x300);
    edx = eax;
    eax = ((eax as i32) >> 1) as u32;
    edx = edx.wrapping_sub(eax);
    x[6] = m128(e, EXP_CE90);
    x[6] = andnpd(x[6], x[2]);
    eax = eax.wrapping_add(0x3ff);
    x[3] = movd(eax);
    x[3] = psllq(x[3], 0x34);
    x[6] = orpd(x[6], x[3]);
    edx = edx.wrapping_add(0x3ff);
    x[4] = movd(edx);
    x[4] = psllq(x[4], 0x34);
    // FLD x0; FLD x6; FMUL ST(1), ST0; FADDP; FLD x4; FMULP; FSTP double.
    let a = X87Ext::load64(f64::from_bits(x[0][0]));
    let b = X87Ext::load64(f64::from_bits(x[6][0]));
    let product = x87_fmul(a, b);
    let sum = x87_fadd(product, b);
    let c = X87Ext::load64(f64::from_bits(x[4][0]));
    let scaled = x87_fmul(sum, c).store64();
    x[0] = movlpd(x[0], scaled.to_bits());
    set_x87_control_word(e, saved);
    let top = pextrw(x[0], 3) & 0x7ff0;
    let code = if top >= 0x7ff0 {
        0xe
    } else if top == 0 {
        0xf
    } else {
        return x87_load64(x[0][0]);
    };
    sse2_error_one(e, argument, x[0][0], code)
}

/// `exp`'s SSE2 core outside the table range (`00edbfde`).
fn exp_sse2_special(e: &mut Engine, x: &mut [Xmm; 8], argument: u32) -> f64 {
    let m = |e: &Engine, a: u32| e.mem.u64(a);
    let high = e.mem.u32(argument + 4);
    let low = e.mem.u32(argument);
    let magnitude = high & 0x7fff_ffff;
    if magnitude < 0x4090_0000 {
        // A tiny argument: 1 + x.
        x[0] = movlpd(x[0], m(e, argument));
        x[0] = addsd(x[0], m(e, EXP_D330));
        return x87_load64(x[0][0]);
    }
    // 00edbf73
    let code = if magnitude < 0x7ff0_0000 {
        if high < 0x8000_0000 {
            x[0] = movlpd(x[0], m(e, EXP_D360));
            x[0] = mulsd(x[0], x[0][0]);
            0xe
        } else {
            x[0] = movlpd(x[0], m(e, EXP_D368));
            x[0] = mulsd(x[0], x[0][0]);
            0xf
        }
    } else if magnitude > 0x7ff0_0000 || low != 0 {
        // 00edbfd4: a NaN, XMM0 as it is.
        0x3ea
    } else if high == 0x7ff0_0000 {
        return x87_load64(m(e, EXP_D350));
    } else {
        return x87_load64(m(e, EXP_D358));
    };
    sse2_error_one(e, argument, x[0][0], code)
}
// ---- The table-dispatched x87 bodies: exp and atan2 ----------------------------------------------
//
// `exp`'s x87 path and `atan2` run through the CRT's descriptor dispatch: `__ctrandisp1`
// (`00ed6281`) / `__cintrindisp1` (`00ed60ae`) and `__ctrandisp2` (`00ed60eb`) /
// `__cintrindisp2` (`00ed6070`) set up a 0x2d0-byte frame, `__trandisp1` (`00ed2550`) /
// `__trandisp2` (`00ed25b7`) load the dispatch control word, classify the operands with `FXAM`
// (through the 16-byte table at `011c1d1c`) and jump through the descriptor's table (the
// descriptor's address in `EDX`: `011c276a` for `exp`, `011c26a0` for `atan2`, in `.data`) to a
// handler, and the common tail (`00ed6133`/`00ed613a`) reports errors through `__87except`. The
// two `__trandisp` routines, the tail and the handlers run in the frame of the dispatcher that
// called them and have no registered form of their own; the handlers are reached by the
// address the descriptor holds, translated here for the two descriptors' targets.

/// The descriptors (`.data`): a length-prefixed name at +0, the argument count at +0xd, the
/// operation code at +0xe and the handler table at +0x10.
const EXP_DESCRIPTOR: u32 = 0x011c_276a;
const ATAN2_DESCRIPTOR: u32 = 0x011c_26a0;
/// `FXAM`'s classes as `__trandisp`'s `XLAT` table maps them to table offsets (`011c1d1c`).
const FXAM_CLASSES: u32 = 0x011c_1d1c;
/// The double `00ed2662` adds to a signalling NaN (`011c1d14`), `exp`'s range bound (extended,
/// `011c274e`), and the tail's scaling (`010f33f8`, `010f33e8`, `010f3408` for an underflow,
/// `010f33f0`, `010f33e0`, `010f3400` for an overflow).
const DISPATCH_NAN_ADDEND: u32 = 0x011c_1d14;
const EXP_BOUND: u32 = 0x011c_274e;
const TAIL_UNDERFLOW_SCALE: u32 = 0x010f_33f8;
const TAIL_UNDERFLOW_BOUND: u32 = 0x010f_33e8;
const TAIL_UNDERFLOW_FACTOR: u32 = 0x010f_3408;
const TAIL_OVERFLOW_SCALE: u32 = 0x010f_33f0;
const TAIL_OVERFLOW_BOUND: u32 = 0x010f_33e0;
const TAIL_OVERFLOW_FACTOR: u32 = 0x010f_3400;
/// The Pentium `FDIV` test's answer (`0126fd94`, `_fpmath`'s): at 1 `exp`'s handler divides
/// through `00ed6ae1` and `fmod`'s takes the remainder through `00ed6cfc` (`fdiv.rs`).
const FDIV_FLAWED: u32 = 0x0126_fd94;
/// `exp`'s entries' SSE2 bodies.
const EXP_SSE2: u32 = 0x00ed_bd68;
const EXP_SSE2_REGISTER: u32 = 0x00ed_bd50;

/// The dispatch frame's state (`EBP` of the dispatcher): the saved control word
/// (`EBP-0xa4`), the descriptor (`EBP-0x94`), the error kind byte (`EBP-0x90`: 0 none, 0xfe
/// check the result's range, 0xff check for an overflow, any other the error type), the
/// flags (`EBP-0x2c8`: bit 0 the arguments are already in the `_exception`, bit 1 two
/// arguments), `CL`/`CH` (the operands' signs, 4 when negative) and `DL`, the condition codes of
/// the status word a handler keeps at `EBP-0xa0` (its high byte, `EBP-0x9f`, as `AH` holds them),
/// the register stack, and the `_exception` block (`EBP-0x8e`: type, name, argument 1, argument
/// 2, result) with the saved word's slot after it, in memory for `__87except`.
struct Dispatch {
    saved: u16,
    descriptor: u32,
    kind: u8,
    flags: u8,
    cl: u8,
    ch: u8,
    dl: u8,
    status_ah: u8,
    fpu: Fpu,
    exception: u32,
}

impl Dispatch {
    /// The frame for a dispatcher: the `_exception` block and the saved word's slot (0x24
    /// bytes at `block`), `FSTCW` into the saved word.
    fn new(e: &mut Engine, descriptor: u32, block: u32) -> Self {
        Dispatch {
            saved: x87_control_word(e),
            descriptor,
            kind: 0,
            flags: 0,
            cl: 0,
            ch: 0,
            dl: 0,
            status_ah: 0,
            fpu: Fpu::default(),
            exception: block,
        }
    }
}

/// `FXAM` of a register through `__trandisp`'s bit shuffle (`SHL; SAR; ROL` of `AH`, `AND AL,
/// 0xf`, `XLAT` on `011c1d1c`): the handler table offset of its class, and `AH` after the
/// shuffle (bit 2 is the sign, C1).
fn dispatch_class(e: &Engine, x: X87Ext) -> (u32, u8) {
    let ah = (crate::x87::x87_fxam(x) >> 8) as u8;
    // SHL CL, 1; SAR CL, 1; ROL CL, 1.
    let shuffled = ((((ah << 1) as i8) >> 1) as u8).rotate_left(1);
    let class = e.mem.u8(FXAM_CLASSES + u32::from(shuffled & 0xf));
    (class as i8 as i32 as u32, shuffled)
}

// Translated from 00ed2550 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__trandisp1`: [`trandisp`] with one operand (`ST0`).
fn trandisp1(e: &mut Engine, d: &mut Dispatch) {
    trandisp(e, d, false);
}

// Translated from 00ed25b7 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__trandisp2`: [`trandisp`] with two operands (`ST0`, `ST1`).
fn trandisp2(e: &mut Engine, d: &mut Dispatch) {
    trandisp(e, d, true);
}

/// `__trandisp1` (`00ed2550`) and `__trandisp2` (`00ed25b7`): the dispatch control word (for
/// the operation 5, the saved word with 53-bit precision and every exception masked; otherwise
/// `0x133f`: 64-bit precision, round to nearest), the operands' classes and the handler the
/// descriptor's table holds at `0x10 + offset`, which runs.
fn trandisp(e: &mut Engine, d: &mut Dispatch, two: bool) {
    let operation = e.mem.u8(d.descriptor + 0xe);
    let word = if operation == 5 {
        let high = (((d.saved >> 8) as u8 | 2) & 0xfe) as u16;
        (high << 8) | 0x3f
    } else {
        0x133f
    };
    set_x87_control_word(e, word);
    d.kind = 0;
    let (class_cl, cl) = dispatch_class(e, d.fpu.st(0));
    let offset = if two {
        let (class_ch, ch) = dispatch_class(e, d.fpu.st(1));
        // MOV AH, AL (the second's class); SHL AH, 2; OR AL, AH.
        let al = (class_cl as u8) | ((class_ch as u8) << 2);
        d.ch = ch & 0x04;
        al as i8 as i32 as u32
    } else {
        // AND ECX, 0x404 leaves in CH the caller's ECX bit 10, which no one-operand handler of
        // these descriptors reads (`exp`'s finite handler clears CH).
        d.ch = 0;
        class_cl
    };
    d.cl = cl & 0x04;
    let target = e
        .mem
        .u32(d.descriptor.wrapping_add(offset).wrapping_add(0x10));
    run_handler(e, d, target);
}

/// The handlers `exp`'s, `atan2`'s and `fmod`'s descriptors point at, by their addresses (the
/// dispatcher jumps to them, `JMP dword ptr [EBX]`; their `RET` returns to its caller).
fn run_handler(e: &mut Engine, d: &mut Dispatch, target: u32) {
    match target {
        // exp
        0x00ed_9301 => exp_finite(e, d),
        0x00ed_265d => pop_load_one(&mut d.fpu),
        0x00ed_2662 => nan_plus_one(e, d),
        0x00ed_9441 => exp_infinite(e, d),
        // atan2
        0x00ed_7cbc => atan2_finite(d),
        0x00ed_7d38 => atan2_half_pi(e, d),
        0x00ed_7d15 => atan2_zero_y(d),
        0x00ed_268d => nan_sum_swapped(d),
        0x00ed_268f => nan_sum(d),
        0x00ed_26b7 => nan_sum_both(d),
        0x00ed_26f6 => pop_two_indefinite(e, d),
        // fmod
        0x00ec_9144 => fmod_finite(e, d),
        0x00ed_2654 => pop_two_load_zero(&mut d.fpu),
        _ => panic!("{target:08x}: a dispatch handler not translated"),
    }
}

/// `FTST`'s condition codes as `AH` holds them (`ST0` against +0).
fn ftst_ah(x: X87Ext) -> u8 {
    fcom_ah(x, X87Ext::load64(0.0))
}

// Translated from 00ed2648 (disassembled, FalloutNV.exe 1.4.0.525)
/// A handlers' tail (no Ghidra name; `exp`'s `00ed9301` jumps to it): `FXCH; FSTP ST0`, `ST1`
/// dropped.
fn drop_st1(fpu: &mut Fpu) {
    fpu.fxch(1);
    fpu.pop();
}

// Translated from 00ed2654 (disassembled, FalloutNV.exe 1.4.0.525)
/// A handler (no Ghidra name; `fmod`'s for a zero `y` with a normal `x`): `FSTP ST0`, then
/// `00ed2656`.
fn pop_two_load_zero(fpu: &mut Fpu) {
    fpu.pop();
    pop_load_zero(fpu);
}

// Translated from 00ed2656 (disassembled, FalloutNV.exe 1.4.0.525)
/// A handlers' tail (no Ghidra name): `FSTP ST0; FLDZ`, +0 in place of `ST0`.
fn pop_load_zero(fpu: &mut Fpu) {
    fpu.pop();
    fpu.push(X87Ext::load64(0.0));
}

// Translated from 00ed265b (disassembled, FalloutNV.exe 1.4.0.525)
/// A handler (no Ghidra name): `FSTP ST0`, then `00ed265d`.
fn pop_two_load_one(fpu: &mut Fpu) {
    fpu.pop();
    pop_load_one(fpu);
}

// Translated from 00ed265d (disassembled, FalloutNV.exe 1.4.0.525)
/// A handler (no Ghidra name; `exp`'s for a zero argument): `FSTP ST0; FLD1`, 1 in place of
/// `ST0`.
fn pop_load_one(fpu: &mut Fpu) {
    fpu.pop();
    fpu.push(X87Ext::load64(1.0));
}

// Translated from 00ed2662 (disassembled, FalloutNV.exe 1.4.0.525)
/// A handler (no Ghidra name; `exp`'s for a NaN): `FSTP tbyte [EBP-0x9e]; FLD tbyte
/// [EBP-0x9e]` (the value as it was), then `TEST byte ptr [EBP-0x97], 0x40`, the quiet bit
/// (significand bit 62): a quiet NaN sets the error kind 7 and is left; a signalling one sets
/// it to 1 and gets the double at `011c1d14` added (`FADD qword`, which quiets it).
fn nan_plus_one(e: &mut Engine, d: &mut Dispatch) {
    if d.fpu.st(0).significand() & (1 << 62) != 0 {
        d.kind = 7;
    } else {
        d.kind = 1;
        let sum = x87_fadd(d.fpu.st(0), double_operand(e, DISPATCH_NAN_ADDEND));
        d.fpu.set(0, sum);
    }
}

// Translated from 00ed268d (disassembled, FalloutNV.exe 1.4.0.525)
/// A handler (no Ghidra name; `atan2`'s for a NaN `y`): `FXCH`, then `00ed268f`.
fn nan_sum_swapped(d: &mut Dispatch) {
    d.fpu.fxch(1);
    nan_sum(d);
}

// Translated from 00ed268f (disassembled, FalloutNV.exe 1.4.0.525)
/// A handler (no Ghidra name; `atan2`'s for a NaN `x`): `ST0` through the frame as
/// `00ed2662` does, the error kind 7 for a quiet NaN and 1 for a signalling one, then `FADDP`
/// (`ST1 = ST1 + ST0`, popped: the NaN the x87 picks).
fn nan_sum(d: &mut Dispatch) {
    d.kind = if d.fpu.st(0).significand() & (1 << 62) != 0 {
        7
    } else {
        1
    };
    let top = d.fpu.pop();
    let sum = x87_fadd(d.fpu.st(0), top);
    d.fpu.set(0, sum);
}

// Translated from 00ed26b7 (disassembled, FalloutNV.exe 1.4.0.525)
/// A handler (no Ghidra name; `atan2`'s for two NaNs): `ST0` tested as `00ed2662` does; a quiet
/// one has `FXCH` and the other tested too, the error kind 7 only when both are quiet, else 1;
/// then `FADDP`.
fn nan_sum_both(d: &mut Dispatch) {
    d.kind = 1;
    if d.fpu.st(0).significand() & (1 << 62) != 0 {
        d.fpu.fxch(1);
        if d.fpu.st(0).significand() & (1 << 62) != 0 {
            d.kind = 7;
        }
    }
    let top = d.fpu.pop();
    let sum = x87_fadd(d.fpu.st(0), top);
    d.fpu.set(0, sum);
}

// Translated from 00ed26f6 (disassembled, FalloutNV.exe 1.4.0.525)
/// A handler (no Ghidra name; `atan2`'s for two infinities): `FSTP ST0`, then `00ed26f8`.
fn pop_two_indefinite(e: &mut Engine, d: &mut Dispatch) {
    d.fpu.pop();
    pop_indefinite(e, d);
}

// Translated from 00ed26f8 (disassembled, FalloutNV.exe 1.4.0.525)
/// A handlers' tail (no Ghidra name): `FSTP ST0; FLD tbyte [011c1d00]` (the indefinite NaN in
/// place of `ST0`); an error kind above 0 (`CMP byte [EBP-0x90], 0; JG`, signed) is kept,
/// otherwise `00ed2709` sets it to 1.
fn pop_indefinite(e: &mut Engine, d: &mut Dispatch) {
    d.fpu.pop();
    d.fpu.push(load_extended(e, INDEFINITE));
    if (d.kind as i8) <= 0 {
        kind_one(d);
    }
}

// Translated from 00ed2709 (disassembled, FalloutNV.exe 1.4.0.525)
/// A handlers' tail (no Ghidra name): `MOV byte ptr [EBP-0x90], 1` (the error kind 1), then
/// `OR CL, CL`, whose flags the code after the dispatch does not test.
fn kind_one(d: &mut Dispatch) {
    d.kind = 1;
}

// Translated from 00ed2713 (disassembled, FalloutNV.exe 1.4.0.525)
/// A handlers' tail (no Ghidra name; `CL` the sign, 0 for positive): `FCHS` unless `CL` is 0.
fn negate_unless_zero(cl: u8, fpu: &mut Fpu) {
    if cl != 0 {
        let negated = x87_fchs(fpu.st(0));
        fpu.set(0, negated);
    }
}

// Translated from 00ed7cbc (disassembled, FalloutNV.exe 1.4.0.525)
/// `atan2`'s handler for two normal operands (no Ghidra name; `ST1` = `y`, `ST0` = `x`): the
/// error kind 0xfe (the tail checks the result's range); `FABS` both (`FABS; FXCH; FABS;
/// FXCH`), `FPATAN`; `pi - it` for a negative `x` (`CL`: `FLDPI; FSUBRP`), `FCHS` for a
/// negative `y` (`CH`).
fn atan2_finite(d: &mut Dispatch) {
    d.kind = 0xfe;
    let x = x87_fabs(d.fpu.st(0));
    d.fpu.set(0, x);
    d.fpu.fxch(1);
    let y = x87_fabs(d.fpu.st(0));
    d.fpu.set(0, y);
    d.fpu.fxch(1);
    let x = d.fpu.pop();
    let r = x87_fpatan(d.fpu.st(0), x);
    d.fpu.set(0, r);
    if d.cl != 0 {
        let r = x87_fsub(x87_fld_constant(X87Constant::Pi), d.fpu.st(0));
        d.fpu.set(0, r);
    }
    negate_unless_zero(d.ch, &mut d.fpu);
}

// Translated from 00ed7d0c (disassembled, FalloutNV.exe 1.4.0.525)
/// A handlers' helper (no Ghidra name): `FSTP ST0; FLD tbyte [011c1d0a]`, the extended half
/// pi in place of `ST0`.
fn pop_half_pi(e: &Engine, fpu: &mut Fpu) {
    fpu.pop();
    fpu.push(load_extended(e, HALF_PI_EXTENDED));
}

// Translated from 00ed7d15 (disassembled, FalloutNV.exe 1.4.0.525)
/// `atan2`'s handler for a zero `y` with `x` not a NaN (no Ghidra name): `FSTP ST0`, then
/// `FSTP ST0; FLDPI` for a negative `x` (`CL`), else `FSTP ST0; FLDZ`; `FCHS` for a negative
/// `y` (`CH`).
fn atan2_zero_y(d: &mut Dispatch) {
    d.fpu.pop();
    d.fpu.pop();
    if d.cl != 0 {
        d.fpu.push(x87_fld_constant(X87Constant::Pi));
    } else {
        d.fpu.push(X87Ext::load64(0.0));
    }
    negate_unless_zero(d.ch, &mut d.fpu);
}

// Translated from 00ed7d38 (disassembled, FalloutNV.exe 1.4.0.525)
/// `atan2`'s handler for a zero or infinite `x` with a normal `y` (no Ghidra name): `FSTP
/// ST0`, `MOV CL, CH`, `00ed7d0c` (half pi) and `00ed2713` (`FCHS` for a negative `y`).
fn atan2_half_pi(e: &mut Engine, d: &mut Dispatch) {
    d.fpu.pop();
    d.cl = d.ch;
    pop_half_pi(e, &mut d.fpu);
    negate_unless_zero(d.cl, &mut d.fpu);
}

// Translated from 00ed9301 (disassembled, FalloutNV.exe 1.4.0.525)
/// `exp`'s handler for a finite nonzero argument (no Ghidra name): the error kind 0xfe, `CH`
/// cleared, `x * log2(e)` (`FLDL2E; FMULP`), split by `00ed944e` into `n` and `2^|f| - 1`
/// (unless it left through `00ed9400`); plus 1 (`FLD1; FADDP`); for a negative fraction (the
/// `FTST` kept at `EBP-0x9f`, C0) inverted (`FLD1`, then `FDIVRP` or, with the Pentium flaw
/// flag at 1, `00ed6ae1` in `fdiv.rs`); scaled by `n` unless it is zero (`FTST`'s C3 in `DL`:
/// `FSCALE`); `FCHS` with `CH` (cleared); `n` dropped (`JMP 00ed2648`).
fn exp_finite(e: &mut Engine, d: &mut Dispatch) {
    d.kind = 0xfe;
    d.ch = 0;
    let product = x87_fmul(d.fpu.pop(), x87_fld_constant(X87Constant::Log2E));
    d.fpu.push(product);
    if !exp_split(e, d) {
        return;
    }
    // 00ed9313: FLD1; FADDP.
    let sum = x87_fadd(d.fpu.st(0), X87Ext::load64(1.0));
    d.fpu.set(0, sum);
    if d.status_ah & 0x01 != 0 {
        d.fpu.push(X87Ext::load64(1.0));
        if e.global::<u32>(FDIV_FLAWED) == 1 {
            super::fdiv::divide_st0_by_st1_kept(e, &mut d.fpu);
        } else {
            // FDIVRP: ST1 = ST0 / ST1, popped.
            let one = d.fpu.pop();
            let quotient = x87_fdiv(one, d.fpu.st(0));
            d.fpu.set(0, quotient);
        }
    }
    if d.dl & 0x40 == 0 {
        let scaled = x87_fscale(d.fpu.st(0), d.fpu.st(1));
        d.fpu.set(0, scaled);
    }
    negate_unless_zero(d.ch, &mut d.fpu);
    drop_st1(&mut d.fpu);
}

// Translated from 00ed944e (disassembled, FalloutNV.exe 1.4.0.525)
/// `exp`'s split (no Ghidra name; `ST0` = `x * log2(e)`): `|t|` against the extended bound at
/// `011c274e` (`FLD ST0; FABS; FLD tbyte; FCOMPP`, the status kept at `EBP-0xa0`); at or beyond
/// it (or unordered: C3 or C0) `POP EAX` drops the return address and `00ed9400` ends the
/// handler (answered `false`). Otherwise `n = FRNDINT(t)` pushed, its `FTST` kept in `DL`, `f =
/// t - n` (`FXCH; FSUB ST0, ST1`), its `FTST` kept at `EBP-0xa0`, and `2^|f| - 1` (`FABS;
/// F2XM1`) on top of `n`.
fn exp_split(e: &mut Engine, d: &mut Dispatch) -> bool {
    let t = d.fpu.st(0);
    d.status_ah = fcom_ah(load_extended(e, EXP_BOUND), x87_fabs(t));
    if d.status_ah & 0x41 != 0 {
        exp_out_of_range(e, d);
        return false;
    }
    let n = x87_frndint(t);
    d.fpu.push(n);
    d.status_ah = ftst_ah(n);
    d.dl = d.status_ah;
    d.fpu.fxch(1);
    let fraction = x87_fsub(d.fpu.st(0), d.fpu.st(1));
    d.status_ah = ftst_ah(fraction);
    d.fpu.set(0, x87_f2xm1(x87_fabs(fraction)));
    true
}

// Translated from 00ed9400 (disassembled, FalloutNV.exe 1.4.0.525)
/// The end of `exp`'s handler for an argument out of range (inside `00ed9301`'s body; `POP
/// EAX` first): `FTST`; below zero, the error kind 4 (underflow) and +0 (`00ed2656`);
/// otherwise `FSTP ST0; FLD tbyte [011c2730]` (+infinity), `FCHS` with `CH`.
fn exp_out_of_range(e: &mut Engine, d: &mut Dispatch) {
    d.status_ah = ftst_ah(d.fpu.st(0));
    if d.status_ah & 0x01 != 0 {
        d.kind = 4;
        pop_load_zero(&mut d.fpu);
        return;
    }
    d.fpu.pop();
    d.fpu.push(load_extended(e, POW_POLE));
    negate_unless_zero(d.ch, &mut d.fpu);
}

// Translated from 00ed9441 (disassembled, FalloutNV.exe 1.4.0.525)
/// `exp`'s handler for an infinite argument (no Ghidra name): negative (`CL`), +0 (`JNZ
/// 00ed93f4`, which jumps to `00ed2656`); positive, `FSTP ST0; FLD tbyte [011c2730]`.
fn exp_infinite(e: &mut Engine, d: &mut Dispatch) {
    if d.cl != 0 {
        pop_load_zero(&mut d.fpu);
        return;
    }
    d.fpu.pop();
    d.fpu.push(load_extended(e, POW_POLE));
}

/// The double `00ed9491` halves its argument with (`011c2762`).
const INTEGER_HALF: u32 = 0x011c_2762;

// Translated from 00ed9491 (disassembled, FalloutNV.exe 1.4.0.525)
/// Which integer `ST0` is (no Ghidra name; `pow`'s handlers call it; the stack is left as it
/// was): `FRNDINT` of a copy compared with it (`FCOMP; FSTSW AX; SAHF; JNZ`): not equal (ZF,
/// C3, clear) answers 0; otherwise the value times the double at `011c2762`, `FRNDINT` of a
/// copy compared with that (`FCOMPP`): equal answers 2 (even), else 1 (odd). An unordered
/// compare sets C3 and counts as equal.
fn integer_parity(e: &Engine, fpu: &Fpu) -> u32 {
    let x = fpu.st(0);
    if fcom_ah(x87_frndint(x), x) & 0x40 == 0 {
        return 0;
    }
    let half = x87_fmul(x, double_operand(e, INTEGER_HALF));
    if fcom_ah(x87_frndint(half), half) & 0x40 != 0 {
        2
    } else {
        1
    }
}

/// The stack of a registered form, `ST0` = `st0` on `ST1` = `st1`, each taken exactly.
fn two_registers(st0: f64, st1: f64) -> Fpu {
    let mut fpu = Fpu::with(ext_of_double(st1.to_bits()));
    fpu.push(ext_of_double(st0.to_bits()));
    fpu
}

/// The registered form of `00ed2648`: `ST0`, `ST1`; `ST0` afterwards.
pub fn fn_00ed2648(_e: &mut Engine, st0: f64, st1: f64) -> f64 {
    let mut fpu = two_registers(st0, st1);
    drop_st1(&mut fpu);
    fpu.st(0).store64()
}

/// The registered form of `00ed2654`: `ST0`, `ST1`; `ST0` afterwards (+0).
pub fn fn_00ed2654(_e: &mut Engine, st0: f64, st1: f64) -> f64 {
    let mut fpu = two_registers(st0, st1);
    pop_two_load_zero(&mut fpu);
    fpu.st(0).store64()
}

/// The registered form of `00ed2656`: `ST0`; `ST0` afterwards (+0).
pub fn fn_00ed2656(_e: &mut Engine, st0: f64) -> f64 {
    let mut fpu = Fpu::with(ext_of_double(st0.to_bits()));
    pop_load_zero(&mut fpu);
    fpu.st(0).store64()
}

/// The registered form of `00ed265b`: `ST0`, `ST1`; `ST0` afterwards (1).
pub fn fn_00ed265b(_e: &mut Engine, st0: f64, st1: f64) -> f64 {
    let mut fpu = two_registers(st0, st1);
    pop_two_load_one(&mut fpu);
    fpu.st(0).store64()
}

/// The registered form of `00ed265d`: `ST0`; `ST0` afterwards (1).
pub fn fn_00ed265d(_e: &mut Engine, st0: f64) -> f64 {
    let mut fpu = Fpu::with(ext_of_double(st0.to_bits()));
    pop_load_one(&mut fpu);
    fpu.st(0).store64()
}

/// The registered form of `00ed2713`: `ECX` (its low byte `CL`), `ST0`; `ST0` afterwards.
pub fn fn_00ed2713(_e: &mut Engine, ecx: u32, st0: f64) -> f64 {
    let mut fpu = Fpu::with(ext_of_double(st0.to_bits()));
    negate_unless_zero(ecx as u8, &mut fpu);
    fpu.st(0).store64()
}

/// The registered form of `00ed7d0c`: `ST0`; `ST0` afterwards (half pi, rounded).
pub fn fn_00ed7d0c(e: &mut Engine, st0: f64) -> f64 {
    let mut fpu = Fpu::with(ext_of_double(st0.to_bits()));
    pop_half_pi(e, &mut fpu);
    fpu.st(0).store64()
}

/// The registered form of `00ed9491`: `ST0`; the answer in `EAX`.
pub fn fn_00ed9491(e: &mut Engine, st0: f64) -> u32 {
    integer_parity(e, &Fpu::with(ext_of_double(st0.to_bits())))
}

// Translated from 00ed6133 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__ctrandisp` entry of the dispatch tail: flag bit 0 cleared (`AND byte [EBP-0x2c8], 0xfe`:
/// the arguments are on the stack), then the tail (`00ed613a`, [`dispatch_tail`]).
fn dispatch_tail_from_ctrandisp(e: &mut Engine, d: &mut Dispatch, arguments: &[f64]) -> X87Ext {
    d.flags &= !1;
    dispatch_tail(e, d, arguments)
}

// Translated from 00ed613a (disassembled, FalloutNV.exe 1.4.0.525)
/// The dispatch tail (`00ed6133`, which clears flag bit 0, falls into `00ed613a`): with the
/// math-error switch set, the saved word back and `ST0` as it is. Otherwise the result is
/// stored as a double (`FST`) and the error kind decides: 0 checks the precision flag (the
/// exception unmasked in the saved word and the status flag set: type 8); 0xfe checks the
/// stored double's exponent for an underflow (zero: type 4, scaled by `010f33f8` and, below
/// `010f33e8`, multiplied by `010f3408`) or an overflow (all ones: type 3, scaled by `010f33f0`
/// and, above `010f33e0`, multiplied by `010f3400`), else as 0; 0xff checks for an overflow
/// only; any other value is the error type. A report fills the `_exception` (the descriptor's
/// name at +1, the arguments copied from the stack unless flag bit 0 says they are there, the
/// second one only for a descriptor with more than one argument, the result) and calls
/// `__87except(operation, &exception, &saved word)`, whose result comes back. The saved word is
/// loaded at the end (`FLDCW`).
fn dispatch_tail(e: &mut Engine, d: &mut Dispatch, arguments: &[f64]) -> X87Ext {
    if e.global::<u32>(ERROR_SWITCH) != 0 {
        set_x87_control_word(e, d.saved);
        return d.fpu.pop();
    }
    let stored = d.fpu.st(0).store64().to_bits();
    let exponent = ((stored >> 48) as u16) & 0x7ff0;
    let precision = |d: &Dispatch| d.saved & 0x20 == 0 && status_precision_flag();
    let kind: u32 = match d.kind {
        0 => {
            if precision(d) {
                8
            } else {
                set_x87_control_word(e, d.saved);
                return d.fpu.pop();
            }
        }
        0xfe if exponent == 0 => tail_scale(e, d, true),
        0xfe | 0xff if exponent == 0x7ff0 => tail_scale(e, d, false),
        0xfe | 0xff => {
            if precision(d) {
                8
            } else {
                set_x87_control_word(e, d.saved);
                return d.fpu.pop();
            }
        }
        other => other as i8 as i32 as u32,
    };
    // 00ed621f
    let at = d.exception;
    e.mem.set_u32(at, kind);
    let name = d.descriptor + 1;
    e.mem.set_u32(at + 4, name);
    if d.flags & 1 == 0 {
        e.mem.set_u64(at + 8, arguments[0].to_bits());
        if e.mem.u8(name + 0xc) != 1 {
            e.mem.set_u64(at + 0x10, arguments[1].to_bits());
        }
    }
    let result = d.fpu.pop().store64();
    e.mem.set_u64(at + 0x18, result.to_bits());
    let slot = at + 0x20;
    e.mem.set_u16(slot, d.saved);
    let operation = e.mem.u8(d.descriptor + 0xe) as i8 as i32 as u32;
    e.call(EXCEPT87, &args![operation, at, slot]);
    let back = load_double(e, at + 0x18);
    set_x87_control_word(e, d.saved);
    back
}

/// The tail's scaling of an underflowed (`underflow`) or overflowed result, returning the
/// error type (4 or 3).
fn tail_scale(e: &Engine, d: &mut Dispatch, underflow: bool) -> u32 {
    let (scale, bound, factor, kind) = if underflow {
        (
            TAIL_UNDERFLOW_SCALE,
            TAIL_UNDERFLOW_BOUND,
            TAIL_UNDERFLOW_FACTOR,
            4,
        )
    } else {
        (
            TAIL_OVERFLOW_SCALE,
            TAIL_OVERFLOW_BOUND,
            TAIL_OVERFLOW_FACTOR,
            3,
        )
    };
    // FLD scale; FXCH; FSCALE; FSTP ST1; FLD ST0; FABS; FCOMP bound; SAHF.
    let scaled = x87_fscale(d.fpu.st(0), load_double(e, scale));
    d.fpu.set(0, scaled);
    let ah = fcom_ah(x87_fabs(scaled), double_operand(e, bound));
    // Underflow: JNC (C0 clear) skips the factor; overflow: JBE (C0 or C3) skips it.
    let multiply = if underflow {
        ah & 0x01 != 0
    } else {
        ah & 0x41 == 0
    };
    if multiply {
        let product = x87_fmul(scaled, double_operand(e, factor));
        d.fpu.set(0, product);
    }
    kind
}

// Translated from 00ed62b4 (disassembled, FalloutNV.exe 1.4.0.525)
/// The dispatchers' loader (no label; Ghidra gives it no name), cdecl, the double: `FLD
/// double`, or for an exponent of all ones the extended value built by hand: the top word ORed
/// with 0x7fff (the sign kept), the high dword `SHLD high, low, 11`, and the low word as it is
/// (not shifted: the low 11 bits of the payload are not where `FLD` would put them), so a
/// signalling NaN stays signalling. Registered with the loaded register rounded to a double.
pub fn fn_00ed62b4(_e: &mut Engine, x: f64) -> f64 {
    dispatch_load(x.to_bits()).store64()
}

fn dispatch_load(bits: u64) -> X87Ext {
    let (low, high) = (bits as u32, (bits >> 32) as u32);
    let top = (high >> 16) as u16;
    if top & 0x7ff0 != 0x7ff0 {
        return X87Ext::load64(f64::from_bits(bits));
    }
    let upper = (high << 11) | (low >> 21);
    X87Ext::from_parts(top | 0x7fff, (u64::from(upper) << 32) | u64::from(low))
}

/// A dispatcher's frame: the `_exception` block (0x20 bytes) and the saved word's slot.
fn with_dispatch<R>(
    e: &mut Engine,
    descriptor: u32,
    f: impl FnOnce(&mut Engine, &mut Dispatch) -> R,
) -> R {
    e.with_stack(0x24, |e, block| {
        let mut d = Dispatch::new(e, descriptor, block.addr());
        f(e, &mut d)
    })
}

// Translated from 00ed6281 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__ctrandisp1` (Ghidra's function-ID label), reached by a jump with `EDX` = the descriptor
/// and the argument on the stack above the caller's return address: the argument loaded
/// (`00ed62b4`), the control word saved, flag bit 1 cleared, `__trandisp1` and the tail.
pub fn ctrandisp1(e: &mut Engine, descriptor: u32, _return_address: u32, x: f64) -> f64 {
    with_dispatch(e, descriptor, |e, d| {
        d.fpu.push(dispatch_load(x.to_bits()));
        d.flags &= !2;
        trandisp1(e, d);
        dispatch_tail_from_ctrandisp(e, d, &[x, 0.0]).store64()
    })
}

// Translated from 00ed60ae (disassembled, FalloutNV.exe 1.4.0.525)
/// `__cintrindisp1` (Ghidra's function-ID label), `EDX` = the descriptor, the argument in
/// `ST0`: the control word saved; unless the global at `011c2b60` is set the argument is stored
/// into the `_exception` first (`FST`); `__trandisp1`; flag bit 0 set and bit 1 cleared; the
/// tail (`00ed613a`).
pub fn cintrindisp1(e: &mut Engine, descriptor: u32, st0: f64) -> f64 {
    with_dispatch(e, descriptor, |e, d| {
        let x = X87Ext::load64(st0);
        if e.global::<u32>(MATHERR_SKIP) == 0 {
            e.mem.set_u64(d.exception + 8, x.store64().to_bits());
        }
        d.fpu.push(x);
        trandisp1(e, d);
        d.flags = (d.flags | 1) & !2;
        dispatch_tail(e, d, &[st0, 0.0]).store64()
    })
}

// Translated from 00ed60eb (disassembled, FalloutNV.exe 1.4.0.525)
/// `__ctrandisp2` (Ghidra's function-ID label), `EDX` = the descriptor, the two arguments on
/// the stack above the caller's return address: both loaded (`00ed62b4`, the first ending in
/// `ST1`), the control word saved, flag bit 1 set, the byte at `EBP-0x8f` set to 1 (nothing of
/// these two descriptors' paths reads it), `__trandisp2` and the tail.
pub fn ctrandisp2(e: &mut Engine, descriptor: u32, _return_address: u32, y: f64, x: f64) -> f64 {
    with_dispatch(e, descriptor, |e, d| {
        d.fpu.push(dispatch_load(y.to_bits()));
        d.fpu.push(dispatch_load(x.to_bits()));
        d.flags |= 2;
        trandisp2(e, d);
        dispatch_tail_from_ctrandisp(e, d, &[y, x]).store64()
    })
}

// Translated from 00ed6070 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__cintrindisp2` (Ghidra's function-ID label), `EDX` = the descriptor, the arguments in
/// `ST1` (the first) and `ST0`: unless the global at `011c2b60` is set both are stored into the
/// `_exception` (`FXCH; FST; FXCH; FST`); `__trandisp2`; flag bits 0 and 1 set; the tail.
pub fn cintrindisp2(e: &mut Engine, descriptor: u32, st1: f64, st0: f64) -> f64 {
    with_dispatch(e, descriptor, |e, d| {
        let (y, x) = (X87Ext::load64(st1), X87Ext::load64(st0));
        if e.global::<u32>(MATHERR_SKIP) == 0 {
            e.mem.set_u64(d.exception + 8, y.store64().to_bits());
            e.mem.set_u64(d.exception + 0x10, x.store64().to_bits());
        }
        d.fpu.push(y);
        d.fpu.push(x);
        trandisp2(e, d);
        d.flags |= 3;
        dispatch_tail(e, d, &[st1, st0]).store64()
    })
}

// Translated from 00ecae30 (disassembled, FalloutNV.exe 1.4.0.525)
/// `exp` (LIBCMT; Ghidra's function ID), cdecl, the double on the stack, the result in `ST0`:
/// with `__use_sse2_mathfcns` set and every `MXCSR` and x87 exception masked it jumps to the
/// SSE2 body `00edbd68`; otherwise `MOV EDX, 011c276a` (its descriptor) and `__ctrandisp1`.
pub fn exp(e: &mut Engine, x: f64) -> f64 {
    if super::sse2_math_available(e) {
        return e.call(EXP_SSE2, &args![x]).f64();
    }
    ctrandisp1(e, EXP_DESCRIPTOR, 0, x)
}

// Translated from 00ecae6c (disassembled, FalloutNV.exe 1.4.0.525)
/// `_CIexp` (the register form of `exp`; no label in the map, named from its body), `ST0` in,
/// `ST0` out: the same test picks the SSE2 body `00edbd50`; otherwise the descriptor and
/// `__cintrindisp1`.
pub fn ci_exp(e: &mut Engine, st0: f64) -> f64 {
    if super::sse2_math_available(e) {
        return e.call(EXP_SSE2_REGISTER, &args![st0]).f64();
    }
    cintrindisp1(e, EXP_DESCRIPTOR, st0)
}

// Translated from 00ec9760 (disassembled, FalloutNV.exe 1.4.0.525)
/// `atan2` (LIBCMT; the name `"atan2"` its descriptor holds), cdecl, `y` then `x` on the stack,
/// the result in `ST0`: `MOV EDX, 011c26a0; JMP __ctrandisp2` (no SSE2 body). Its handlers:
/// both normal, `FPATAN` of `|y|` over `|x|`, `pi -` it for a negative `x`, negated for a
/// negative `y`, with the range checked; `x` zero or `y` infinite (and `x` not infinite or NaN),
/// +-pi/2 by `y`'s sign; `x` infinite or `y` zero, 0 or pi (for a negative `x`) by `y`'s sign;
/// a NaN, the two added (error type 7 for a quiet one, 1 for a signalling one); both
/// infinite, the indefinite NaN with type 1.
pub fn atan2(e: &mut Engine, y: f64, x: f64) -> f64 {
    ctrandisp2(e, ATAN2_DESCRIPTOR, 0, y, x)
}

// Translated from 00ec976a (disassembled, FalloutNV.exe 1.4.0.525)
/// `_CIatan2` (the register form of `atan2`; no label in the map), `y` in `ST1`, `x` in `ST0`
/// (registered `y` first): the descriptor and `__cintrindisp2`.
pub fn ci_atan2(e: &mut Engine, y: f64, x: f64) -> f64 {
    cintrindisp2(e, ATAN2_DESCRIPTOR, y, x)
}

// ---- log and log10 ------------------------------------------------------------------------------

/// The names and operation codes of `log` (`"log"`, `011c1990`, 0x1a) and `log10` (`"log10"`,
/// `011c19a0`, 0x1b), the extended -infinity of a zero argument (`011c273a`), and the SSE2
/// bodies (`00edbae0`/`00edbaf8`, `00edd160`/`00edd178`).
const LOG_NAME: u32 = 0x011c_1990;
const LOG10_NAME: u32 = 0x011c_19a0;
const LOG_OPERATION: u32 = 0x1a;
const LOG10_OPERATION: u32 = 0x1b;
const MINUS_INFINITY_EXTENDED: u32 = 0x011c_273a;
const LOG_SSE2_REGISTER: u32 = 0x00ed_bae0;
const LOG_SSE2: u32 = 0x00ed_baf8;
const LOG10_SSE2_REGISTER: u32 = 0x00ed_d160;
const LOG10_SSE2: u32 = 0x00ed_d178;

// Translated from 00eca920 (disassembled, FalloutNV.exe 1.4.0.525)
/// `log` (LIBCMT; Ghidra's function ID gives this body a conflicting name, the name `"log"` its
/// error reports pass at `011c1990` decides), cdecl, the double on the stack, the result in
/// `ST0`: with `__use_sse2_mathfcns` set and every `MXCSR` and x87 exception masked the SSE2 body
/// `00edbaf8`; otherwise `__fload_withFB` and the x87 body `00eca9b8`.
pub fn log(e: &mut Engine, x: f64) -> f64 {
    if super::sse2_math_available(e) {
        return e.call(LOG_SSE2, &args![x]).f64();
    }
    stack_x87(e, x, log_x87)
}

// Translated from 00eca960 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_CIlog` (the register form of `log`; no label in the map), `ST0` in, `ST0` out: the same test
/// picks the SSE2 body `00edbae0`; otherwise `ST0` stored, `00ed2808`, and the x87 body
/// `00eca9b8`.
pub fn ci_log(e: &mut Engine, st0: f64) -> f64 {
    if super::sse2_math_available(e) {
        return e.call(LOG_SSE2_REGISTER, &args![st0]).f64();
    }
    register_x87(e, st0, log_x87)
}

// Translated from 00ecbc80 (disassembled, FalloutNV.exe 1.4.0.525)
/// `log10` (LIBCMT; Ghidra's function ID gives this body a conflicting name, the name `"log10"`
/// at `011c19a0` decides), as [`log`] with the SSE2 body `00edd178` and the x87 body `00ecbd18`.
pub fn log10(e: &mut Engine, x: f64) -> f64 {
    if super::sse2_math_available(e) {
        return e.call(LOG10_SSE2, &args![x]).f64();
    }
    stack_x87(e, x, log10_x87)
}

// Translated from 00ecbcc0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_CIlog10` (the register form of `log10`; no label in the map), as [`ci_log`] with the SSE2
/// body `00edd160` and the x87 body `00ecbd18`.
pub fn ci_log10(e: &mut Engine, st0: f64) -> f64 {
    if super::sse2_math_available(e) {
        return e.call(LOG10_SSE2_REGISTER, &args![st0]).f64();
    }
    register_x87(e, st0, log10_x87)
}

// Translated from 00eca9b8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `log`'s x87 body (no label; Ghidra gives it no name), with the inputs of `acos`'s
/// (`fn_00eca228`). A finite argument runs under the control word at `010f2a98` (unless the
/// saved one is `0x027f`); a positive one (a denormal included) is `FLDLN2; FXCH; FYL2X`,
/// leaving through `__math_exit` (operation 0x1a, `"log"`); a negative one is a domain error with
/// the indefinite NaN (type 1); a zero gives the extended -infinity (`011c273a`) with type 2.
/// +infinity comes back as it is through the exit, -infinity is the domain error, a NaN gets
/// its type from `00ed27ac`; errors go to `__startOneArgErrorHandling`.
pub fn fn_00eca9b8(e: &mut Engine, zf: bool, eax: u32, st0: f64, low: u32, high: u32) -> f64 {
    log_x87(e, zf, eax, ext_of_double(st0.to_bits()), low, high).store64()
}

fn log_x87(e: &mut Engine, zf: bool, eax: u32, x: X87Ext, low: u32, high: u32) -> X87Ext {
    logarithm_x87(e, zf, eax, x, low, high, LOG_OPERATION)
}

// Translated from 00ecbd18 (disassembled, FalloutNV.exe 1.4.0.525)
/// `log10`'s x87 body (no label), as `log`'s (`fn_00eca9b8`) with `FLDLG2` and the operation
/// 0x1b (`"log10"`).
pub fn fn_00ecbd18(e: &mut Engine, zf: bool, eax: u32, st0: f64, low: u32, high: u32) -> f64 {
    log10_x87(e, zf, eax, ext_of_double(st0.to_bits()), low, high).store64()
}

fn log10_x87(e: &mut Engine, zf: bool, eax: u32, x: X87Ext, low: u32, high: u32) -> X87Ext {
    logarithm_x87(e, zf, eax, x, low, high, LOG10_OPERATION)
}

/// The two logarithm bodies (the same code with `FLDLN2` or `FLDLG2`).
fn logarithm_x87(
    e: &mut Engine,
    zf: bool,
    eax: u32,
    x: X87Ext,
    low: u32,
    high: u32,
    operation: u32,
) -> X87Ext {
    let (name, constant) = if operation == LOG_OPERATION {
        (LOG_NAME, X87Constant::LnTwo)
    } else {
        (LOG10_NAME, X87Constant::Log10Two)
    };
    let saved = x87_control_word(e);
    let (kind, result) = if zf {
        // 00ecaa0b: an infinity or NaN; EAX the high word.
        if let Some(found) = nan_type(e, eax, x, low) {
            found
        } else if eax & 0x8000_0000 == 0 {
            // +infinity: as it is, through the exit.
            return inverse_exit(e, x, saved, operation, name, low, high);
        } else {
            (1, load_extended(e, INDEFINITE))
        }
    } else {
        // MOV EAX, dword ptr [ESP+0xc]: the stored double's high word.
        let eax = high;
        if saved != 0x027f {
            let word = e.mem.u16(ATAN_CONTROL_WORD);
            set_x87_control_word(e, word);
        }
        if eax & 0x7ff0_0000 == 0 && (eax & 0xf_ffff) | low == 0 {
            // A zero: FSTP ST0; FLD tbyte [011c273a]; type 2.
            (2, load_extended(e, MINUS_INFINITY_EXTENDED))
        } else if eax & 0x8000_0000 != 0 {
            (1, load_extended(e, INDEFINITE))
        } else {
            // FLDLN2 / FLDLG2; FXCH; FYL2X.
            let result = x87_fyl2x(x87_fld_constant(constant), x);
            return inverse_exit(e, result, saved, operation, name, low, high);
        }
    };
    inverse_error(e, kind, result, saved, operation, name, low, high)
}

/// `CVTSI2SD dest, r32`: the signed integer, converted exactly, in the low lane.
fn cvtsi2sd(dest: Xmm, value: u32) -> Xmm {
    [f64::from(value as i32).to_bits(), dest[1]]
}

/// `CMPEQSD dest, src`: the low lane all ones when the two are equal (ordered), else zero.
fn cmpeqsd(dest: Xmm, src: u64) -> Xmm {
    let equal = f64::from_bits(dest[0]) == f64::from_bits(src);
    [if equal { u64::MAX } else { 0 }, dest[1]]
}

/// The two SSE2 logarithms' constants and tables (`log`'s at `010fcXXX`, `log10`'s at
/// `010fdXXX`), by the role the bodies give them: the fraction mask, the exponent bits ORed in
/// (also the numerator of a zero's division and the test of +infinity), the rounding addend of
/// the table index, the high-part mask, (`log10` only) the subtrahend of the high part, the
/// exponent's factor pair, the pair of masks indexed by whether the reduction was exact, the
/// zero the argument is compared with, the denormals' scale, the result of +infinity, the
/// result of a zero, the polynomial pairs (three for `log`, four for `log10`), the two tables
/// (the reciprocals and the logarithms), and the error codes (NaN, zero, negative).
struct LogTables {
    fraction_mask: u32,
    exponent_bits: u32,
    index_addend: u32,
    high_mask: u32,
    high_subtrahend: Option<u32>,
    exponent_factor: u32,
    exact_masks: u32,
    zero: u32,
    denormal_scale: u32,
    infinity: u32,
    minus_infinity: u32,
    polynomial: [u32; 3],
    last_pair: Option<u32>,
    reciprocals: u32,
    logarithms: u32,
    codes: [u32; 3],
}

const LOG_TABLES: LogTables = LogTables {
    fraction_mask: 0x010f_c580,
    exponent_bits: 0x010f_c5e0,
    index_addend: 0x010f_c590,
    high_mask: 0x010f_c5a0,
    high_subtrahend: None,
    exponent_factor: 0x010f_c5b0,
    exact_masks: 0x010f_c5c0,
    zero: 0x010f_c5f0,
    denormal_scale: 0x010f_c600,
    infinity: 0x010f_c618,
    minus_infinity: 0x010f_c610,
    polynomial: [0x010f_c630, 0x010f_c640, 0x010f_c650],
    last_pair: None,
    reciprocals: 0x010f_c660,
    logarithms: 0x010f_ca70,
    codes: [0x3e8, 2, 3],
};

const LOG10_TABLES: LogTables = LogTables {
    fraction_mask: 0x010f_d380,
    exponent_bits: 0x010f_d3f0,
    index_addend: 0x010f_d3a0,
    high_mask: 0x010f_d3b0,
    high_subtrahend: Some(0x010f_d390),
    exponent_factor: 0x010f_d3c0,
    exact_masks: 0x010f_d3d0,
    zero: 0x010f_d400,
    denormal_scale: 0x010f_d410,
    infinity: 0x010f_d428,
    minus_infinity: 0x010f_d420,
    polynomial: [0x010f_d440, 0x010f_d450, 0x010f_d460],
    last_pair: Some(0x010f_d470),
    reciprocals: 0x010f_d890,
    logarithms: 0x010f_d480,
    codes: [0x3e9, 8, 9],
};

// Translated from 00edbae0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The register form of `log`'s SSE2 body (no label), `ST0` in: the aligned frame, `ST0` stored
/// (`FSTP`), `MOVQ XMM0`, and the core `00edbafe` called with it above the return address.
pub fn fn_00edbae0(e: &mut Engine, st0: f64) -> f64 {
    let stored = x87_load64(st0.to_bits());
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), stored.to_bits());
        logarithm_sse2(e, slot.addr(), movq(stored.to_bits()), &LOG_TABLES)
    })
}

// Translated from 00edbaf8 (disassembled, FalloutNV.exe 1.4.0.525)
/// The stack form of `log`'s SSE2 body (no label), cdecl: `MOVLPD XMM0, [ESP+4]` and the core
/// `00edbafe` that follows.
pub fn fn_00edbaf8(e: &mut Engine, x: f64) -> f64 {
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        logarithm_sse2(e, slot.addr(), movlpd([0; 2], x.to_bits()), &LOG_TABLES)
    })
}

// Translated from 00edbafe (disassembled, FalloutNV.exe 1.4.0.525)
/// `log`'s SSE2 core (no label), `XMM0` the argument (registered with the double above the return
/// address). Every instruction is the exe's, in its order: the fraction with the exponent of 1
/// (`010fc580`, `010fc5e0`) rounded (`010fc590`) to a table index; the reciprocal (`010fc660`)
/// reduces it, the logarithm (`010fca70`) and the exponent times `010fc5b0` (`CVTSI2SD`) are
/// added, the polynomial (`010fc630`..`010fc650`) on the reduced value, the high part masked by
/// `010fc5c0`'s row (the second when the reduction is exact). A denormal is scaled by
/// `010fc600` and runs again with the exponent less 52; a zero (`CMPEQSD` with `010fc5f0`) gives
/// `010fc610` with error 2 (after `1 / x`, the zero divide); +infinity gives `010fc618`; a NaN
/// error 0x3e8; a negative number or -infinity `0 / 0` (the default NaN) with error 3. Errors go
/// to `___libm_error_support` with the argument's slot twice and a result slot holding `XMM1`'s
/// low lane, which is then the result.
pub fn fn_00edbafe(e: &mut Engine, x: f64) -> f64 {
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        logarithm_sse2(e, slot.addr(), movq(x.to_bits()), &LOG_TABLES)
    })
}

// Translated from 00edd160 (disassembled, FalloutNV.exe 1.4.0.525)
/// The register form of `log10`'s SSE2 body (no label), as `log`'s (`fn_00edbae0`) with the core
/// `00edd17e`.
pub fn fn_00edd160(e: &mut Engine, st0: f64) -> f64 {
    let stored = x87_load64(st0.to_bits());
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), stored.to_bits());
        logarithm_sse2(e, slot.addr(), movq(stored.to_bits()), &LOG10_TABLES)
    })
}

// Translated from 00edd178 (disassembled, FalloutNV.exe 1.4.0.525)
/// The stack form of `log10`'s SSE2 body (no label), cdecl, with the core `00edd17e`.
pub fn fn_00edd178(e: &mut Engine, x: f64) -> f64 {
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        logarithm_sse2(e, slot.addr(), movlpd([0; 2], x.to_bits()), &LOG10_TABLES)
    })
}

// Translated from 00edd17e (disassembled, FalloutNV.exe 1.4.0.525)
/// `log10`'s SSE2 core (no label): `log`'s (`fn_00edbafe`) on its own constants (`010fd380` ..
/// `010fd470`) and tables (`010fd890`, `010fd480`), the high part less `010fd390` instead of the
/// exponent bits, a fourth polynomial pair (`010fd470`) added, and the error codes 0x3e9, 8 and 9.
pub fn fn_00edd17e(e: &mut Engine, x: f64) -> f64 {
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        logarithm_sse2(e, slot.addr(), movq(x.to_bits()), &LOG10_TABLES)
    })
}

fn logarithm_sse2(e: &mut Engine, argument: u32, xmm0: Xmm, t: &LogTables) -> f64 {
    let m = |e: &Engine, a: u32| e.mem.u64(a);
    let mut x: [Xmm; 8] = [[0; 2]; 8];
    x[0] = xmm0;
    // MOV EDX, 0.
    let mut edx: u32 = 0;
    loop {
        // 00edbb03
        x[5] = x[0];
        x[0] = unpcklpd(x[0], x[0]);
        x[5] = psrlq(x[5], 0x34);
        let mut ecx = pextrw(x[5], 0);
        x[1] = m128(e, t.fraction_mask);
        if let Some(subtrahend) = t.high_subtrahend {
            x[2] = m128(e, subtrahend);
        }
        x[3] = m128(e, t.exponent_bits);
        x[4] = m128(e, t.index_addend);
        x[6] = m128(e, t.high_mask);
        x[0] = andpd(x[0], x[1]);
        x[0] = orpd(x[0], x[3]);
        x[4] = addpd(x[4], x[0]);
        let mut eax = pextrw(x[4], 0) & 0x7f0;
        x[4] = m128(e, t.reciprocals.wrapping_add(eax));
        x[7] = m128(e, t.logarithms.wrapping_add(eax));
        x[6] = andpd(x[6], x[0]);
        x[0] = subpd(x[0], x[6]);
        x[6] = mulpd(x[6], x[4]);
        // log subtracts the exponent bits, log10 its own pair (XMM2).
        x[6] = subpd(
            x[6],
            if t.high_subtrahend.is_some() {
                x[2]
            } else {
                x[3]
            },
        );
        x[7] = addsd(x[7], x[6][0]);
        x[0] = mulpd(x[0], x[4]);
        x[4] = x[0];
        x[0] = addpd(x[0], x[6]);
        ecx &= 0xfff;
        ecx = ecx.wrapping_sub(1);
        if ecx <= 0x7fd {
            ecx = ecx.wrapping_sub(0x3fe);
            ecx = ecx.wrapping_add(edx);
            x[6] = cvtsi2sd(x[6], ecx);
            x[6] = unpcklpd(x[6], x[6]);
            ecx <<= 0xa;
            eax = eax.wrapping_add(ecx);
            // MOV ECX, 0x10; MOV EDX, 0; CMP EAX, 0; CMOVZ EDX, ECX.
            edx = if eax == 0 { 0x10 } else { 0 };
            x[1] = m128(e, t.polynomial[0]);
            x[3] = x[0];
            x[2] = m128(e, t.polynomial[1]);
            x[1] = mulpd(x[1], x[0]);
            x[3] = mulpd(x[3], x[3]);
            x[1] = addpd(x[1], x[2]);
            x[2] = m128(e, t.polynomial[2]);
            x[3] = mulsd(x[3], x[3][0]);
            x[5] = m128(e, t.exponent_factor);
            x[6] = mulpd(x[6], x[5]);
            x[5] = m128(e, t.exact_masks.wrapping_add(edx));
            x[4] = andpd(x[4], x[5]);
            x[7] = addpd(x[7], x[6]);
            x[7] = addpd(x[7], x[4]);
            x[1] = mulpd(x[1], x[0]);
            x[3] = mulsd(x[3], x[0][0]);
            x[1] = addpd(x[1], x[2]);
            if let Some(last) = t.last_pair {
                x[2] = m128(e, last);
                x[2] = mulpd(x[2], x[0]);
            }
            x[6] = x[7];
            x[6] = unpckhpd(x[6], x[6]);
            x[1] = mulpd(x[1], x[3]);
            x[0] = x[1];
            if t.last_pair.is_some() {
                x[1] = addpd(x[1], x[2]);
            } else {
                x[1] = unpckhpd(x[1], x[1]);
            }
            if t.last_pair.is_some() {
                x[0] = unpckhpd(x[0], x[0]);
            }
            x[0] = addsd(x[0], x[1][0]);
            x[0] = addsd(x[0], x[6][0]);
            x[0] = addsd(x[0], x[7][0]);
            return x87_load64(x[0][0]);
        }
        // 00edbc3e
        x[0] = movlpd(x[0], m(e, argument));
        x[1] = m128(e, t.zero);
        x[1] = cmpeqsd(x[1], x[0][0]);
        if pextrw(x[1], 0) > 0 {
            // A zero: 1 / x (a zero divide), the -infinity, the second code.
            x[2] = movlpd(x[2], m(e, t.exponent_bits));
            x[2] = divsd(x[2], x[0][0]);
            x[1] = movlpd(x[1], m(e, t.minus_infinity));
            return sse2_error_one(e, argument, x[1][0], t.codes[1]);
        }
        if ecx == u32::MAX {
            // A denormal: scaled, and again with the exponent less 52.
            x[1] = movlpd(x[1], m(e, t.denormal_scale));
            x[0] = mulsd(x[0], x[1][0]);
            edx = 0xffff_ffcc;
            continue;
        }
        if ecx <= 0x7fe {
            // 00edbc68: +infinity or a NaN.
            x[0] = movlpd(x[0], m(e, argument));
            x[1] = m128(e, t.fraction_mask);
            x[2] = m128(e, t.exponent_bits);
            x[0] = andpd(x[0], x[1]);
            x[0] = orpd(x[0], x[2]);
            x[2] = cmpeqsd(x[2], x[0][0]);
            if pextrw(x[2], 0) != 0 {
                return x87_load64(m(e, t.infinity));
            }
            return sse2_error_one(e, argument, x[1][0], t.codes[0]);
        }
        // 00edbcd4: a negative number, -infinity or a negative NaN.
        ecx = ecx.wrapping_add(1) & 0x7ff;
        let nan = ecx >= 0x7ff && {
            x[2] = movlpd(x[2], m(e, argument));
            x[0] = movlpd(x[0], m(e, argument));
            let low = x[2][0] as u32;
            x[2] = psrlq(x[2], 0x20);
            let high = x[2][0] as u32 & 0xf_ffff;
            (low | high) != 0
        };
        if nan {
            return sse2_error_one(e, argument, x[1][0], t.codes[0]);
        }
        // 00edbce5: 0 / 0.
        x[1] = xorpd(x[1], x[1]);
        x[1] = divsd(x[1], x[1][0]);
        return sse2_error_one(e, argument, x[1][0], t.codes[2]);
    }
}

// ---- fmod ---------------------------------------------------------------------------------------

/// `fmod`'s descriptor (`.data`, `"fmod"`, two arguments, operation 0x16).
const FMOD_DESCRIPTOR: u32 = 0x011c_18e0;

// Translated from 00ec9130 (disassembled, FalloutNV.exe 1.4.0.525)
/// `fmod` (LIBCMT; unnamed in the engine map, the name `"fmod"` its descriptor holds), cdecl,
/// `y` then `x` on the stack, the result in `ST0`: `MOV EDX, 011c18e0; JMP __ctrandisp2` (no
/// SSE2 body). Its handlers: both normal, `FPREM` of `y` by `x` until it is complete (C2
/// clear), the sign of `y`; `y` zero with `x` normal, +0 (`FLDZ`); a NaN, the two added (error
/// type 7 for a quiet one, 1 for a signalling one); `x` zero or infinite, or `y` infinite, the
/// indefinite NaN with type 1.
pub fn fmod(e: &mut Engine, y: f64, x: f64) -> f64 {
    ctrandisp2(e, FMOD_DESCRIPTOR, 0, y, x)
}

// Translated from 00ec9144 (disassembled, FalloutNV.exe 1.4.0.525)
/// `fmod`'s handler for two normal operands (no Ghidra name): `FXCH` (`y` on top), then, while
/// `FSTSW AX; SAHF; JP` finds C2 set, `FPREM` or, with the Pentium flaw flag at 1 (read each
/// round), the partial remainder `00ed6cfc` (`fdiv.rs`, the condition codes it leaves); `FSTP
/// ST1` leaves the remainder.
fn fmod_finite(e: &mut Engine, d: &mut Dispatch) {
    d.fpu.fxch(1);
    loop {
        let status = if e.global::<u32>(FDIV_FLAWED) == 1 {
            super::fdiv::fprem_adjusted(e, &mut d.fpu)
        } else {
            let (r, status) = x87_fprem(d.fpu.st(0), d.fpu.st(1));
            d.fpu.set(0, r);
            status
        };
        if status & X87_STATUS_C2 == 0 {
            break;
        }
    }
    let r = d.fpu.pop();
    d.fpu.set(0, r);
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ed_454f, fn_00ed454f() -> u32),
        entry!(0x00ed_1989, fn_00ed1989() -> u32),
        entry!(0x00ed_1927, fn_00ed1927() -> u32),
        entry!(0x00ed_18d7, fn_00ed18d7() -> u32),
        entry!(0x00ec_6940, floor(f64) -> f64),
        entry!(0x00ed_4563, floor_x87(f64) -> f64),
        entry!(0x00ed_5981, sptype(u32, u32) -> i32),
        entry!(0x00ee_23bc, frnd(f64) -> f64),
        entry!(0x00ed_5837, handle_qnan1(u32, f64, u32) -> f64),
        entry!(0x00ed_588c, except1(u32, u32, f64, f64, u32) -> f64),
        entry!(0x00ed_5554, handle_exc(u32, Ptr, u32) -> i32),
        entry!(0x00ed_5531, raise_exc(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ed_5763, errcode(u32) -> i32),
        entry!(0x00ed_5736, set_errno_from_matherr(i32)),
        entry!(0x00ed_5797, umatherr(u32, u32, f64, f64, f64, u32) -> f64),
        entry!(0x00ee_0a3a, fn_00ee0a3a(Ptr) -> u32),
        entry!(0x00ed_5ae4, fn_00ed5ae4(u32)),
        entry!(0x00ed_59e1, fn_00ed59e1(f64, Ptr) -> f64),
        entry!(0x00ed_5955, set_exp(f64, i32) -> f64),
        entry!(0x00ed_429f, libm_error_support(Ptr, Ptr, Ptr, u32)),
        entry!(0x00ec_9ac0, pow(f64, f64) -> f64),
        entry!(0x00ec_9b00, ci_pow(f64, f64) -> f64),
        entry!(0x00ec_9b5d, fn_00ec9b5d(u32, f64, u32, f64, f64) -> f64),
        entry!(0x00ec_9d22, fn_00ec9d22(f64) -> u32),
        entry!(0x00ed_955d, fn_00ed955d(f64, f64, Ptr) -> u32),
        entry!(0x00ed_94f7, d_inttype(f64) -> u32),
        entry!(0x00ec_75e5, fpclass(f64) -> u32),
        entry!(0x00ed_8790, fn_00ed8790(f64, f64) -> f64),
        entry!(0x00ed_87a9, fn_00ed87a9(f64, f64) -> f64),
        entry!(0x00ed_2795, fn_00ed2795(u32)),
        entry!(0x00ed_2780, fn_00ed2780(f64) -> f64),
        entry!(0x00ed_281e, fn_00ed281e(f64, u32) -> f64),
        entry!(0x00ed_2869, fn_00ed2869(u32, u32, f64, u32, u32, f64, f64) -> f64),
        entry!(0x00ed_2720, fn_00ed2720(u32, u32, u32, f64, u32, u32, f64, f64) -> f64),
        entry!(0x00ec_a190, acos(f64) -> f64),
        entry!(0x00ec_a1d0, ci_acos(f64) -> f64),
        entry!(0x00ec_a228, fn_00eca228(bool, u32, f64, u32, u32) -> f64),
        entry!(0x00ec_9610, asin(f64) -> f64),
        entry!(0x00ec_96a8, fn_00ec96a8(bool, u32, f64, u32, u32) -> f64),
        entry!(0x00ec_9010, atan(f64) -> f64),
        entry!(0x00ec_9050, ci_atan(f64) -> f64),
        entry!(0x00ec_90a8, fn_00ec90a8(bool, u32, f64, u32, u32) -> f64),
        entry!(0x00ed_9b00, fn_00ed9b00(f64) -> f64),
        entry!(0x00ed_9b18, fn_00ed9b18(f64) -> f64),
        entry!(0x00ed_9b1e, fn_00ed9b1e(f64) -> f64),
        entry!(0x00ed_7720, fn_00ed7720(f64) -> f64),
        entry!(0x00ed_7738, fn_00ed7738(f64) -> f64),
        entry!(0x00ed_773e, fn_00ed773e(f64) -> f64),
        entry!(0x00ed_5d80, fn_00ed5d80(f64) -> f64),
        entry!(0x00ed_5d98, fn_00ed5d98(f64) -> f64),
        entry!(0x00ed_5d9e, fn_00ed5d9e(f64) -> f64),
        entry!(0x00ec_ae30, exp(f64) -> f64),
        entry!(0x00ec_ae6c, ci_exp(f64) -> f64),
        entry!(0x00ec_9760, atan2(f64, f64) -> f64),
        entry!(0x00ec_976a, ci_atan2(f64, f64) -> f64),
        entry!(0x00ec_9130, fmod(f64, f64) -> f64),
        entry!(0x00ed_6281, ctrandisp1(u32, u32, f64) -> f64),
        entry!(0x00ed_60ae, cintrindisp1(u32, f64) -> f64),
        entry!(0x00ed_60eb, ctrandisp2(u32, u32, f64, f64) -> f64),
        entry!(0x00ed_6070, cintrindisp2(u32, f64, f64) -> f64),
        entry!(0x00ed_62b4, fn_00ed62b4(f64) -> f64),
        entry!(0x00ed_bd50, fn_00edbd50(f64) -> f64),
        entry!(0x00ed_bd68, fn_00edbd68(f64) -> f64),
        entry!(0x00ed_bd6e, fn_00edbd6e(f64) -> f64),
        entry!(0x00ec_a920, log(f64) -> f64),
        entry!(0x00ec_a960, ci_log(f64) -> f64),
        entry!(0x00ec_bc80, log10(f64) -> f64),
        entry!(0x00ec_bcc0, ci_log10(f64) -> f64),
        entry!(0x00ec_a9b8, fn_00eca9b8(bool, u32, f64, u32, u32) -> f64),
        entry!(0x00ec_bd18, fn_00ecbd18(bool, u32, f64, u32, u32) -> f64),
        entry!(0x00ed_bae0, fn_00edbae0(f64) -> f64),
        entry!(0x00ed_baf8, fn_00edbaf8(f64) -> f64),
        entry!(0x00ed_bafe, fn_00edbafe(f64) -> f64),
        entry!(0x00ed_d160, fn_00edd160(f64) -> f64),
        entry!(0x00ed_d178, fn_00edd178(f64) -> f64),
        entry!(0x00ed_d17e, fn_00edd17e(f64) -> f64),
        entry!(0x00ed_2648, fn_00ed2648(f64, f64) -> f64),
        entry!(0x00ed_2654, fn_00ed2654(f64, f64) -> f64),
        entry!(0x00ed_2656, fn_00ed2656(f64) -> f64),
        entry!(0x00ed_265b, fn_00ed265b(f64, f64) -> f64),
        entry!(0x00ed_265d, fn_00ed265d(f64) -> f64),
        entry!(0x00ed_2713, fn_00ed2713(u32, f64) -> f64),
        entry!(0x00ed_7d0c, fn_00ed7d0c(f64) -> f64),
        entry!(0x00ed_9491, fn_00ed9491(f64) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What a double saw: the words of each call.
    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;
    /// The `_exception` a double handler saw: type, name, the three doubles.
    type Exception = (u32, u32, u64, u64, u64);
    type Exceptions = Rc<RefCell<Vec<Exception>>>;

    /// `__use_sse2_mathfcns`.
    const SSE2_FLAG: u32 = 0x0127_0a64;
    /// A user `_matherr` for the tests (not a game address).
    const USER_HANDLER: u32 = 0x0040_1230;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with the constants these functions read set to the exe's values, the
    /// `_umatherr` table entries for `ceil` (index 15), `floor` (16) and the last one (28), the
    /// skip flag at `011c2b60` as the exe has it (nonzero), `_errno` doubled to answer one slot,
    /// `__use_sse2_mathfcns` as given and the call log on. Answers the engine and the slot.
    fn engine(sse2: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO_ADDR, move |_, _| slot.into_ret());
        e.map(FLOOR_ONE, 0x50);
        for (at, low, high) in [
            (FLOOR_ONE, 1.0f64.to_bits(), 1.0f64.to_bits()),
            (FLOOR_SHIFT_BASE, 0x433, 0x433),
            (FLOOR_MINUS_ONE, (-1.0f64).to_bits(), 0x4330_0000_0000_0000),
            (FLOOR_MINUS_ZERO, SIGN, SIGN),
            (FLOOR_EXPONENT_MASK, 0x7ff, 0),
        ] {
            e.mem.set_u64(at, low);
            e.mem.set_u64(at + 8, high);
        }
        e.map(FLOOR_CONTROL_WORD, 4);
        e.set_global(FLOOR_CONTROL_WORD, 0x173fu32);
        e.map(ZERO_DOUBLE, 0x18);
        e.mem.set_u64(ZERO_DOUBLE, 0);
        e.mem.set_u64(ONE_DOUBLE, 1.0f64.to_bits());
        e.map(OVERFLOW_INFINITY, 0x18);
        e.mem.set_u64(OVERFLOW_INFINITY, f64::INFINITY.to_bits());
        e.mem.set_u64(OVERFLOW_LARGEST, f64::MAX.to_bits());
        e.map(STATFP_HUGE, 0x18);
        e.mem
            .write(STATFP_HUGE, &[0, 0, 0, 0, 0, 0, 0, 0x80, 0x10, 0x44]);
        e.mem
            .write(STATFP_TINY, &[1, 0, 0, 0, 0, 0, 0, 0x80, 0, 0x30]);
        e.map(MATHERR_SKIP, 4);
        e.set_global(MATHERR_SKIP, 0x2694u32);
        e.map(MATHERR_NAMES, 0xe8);
        for (index, code, name) in [
            (15, 0xcu32, 0x0109_4408u32),
            (16, FLOOR_CODE, 0x0109_43f0),
            (28, 0x26, 0x010f_320c),
        ] {
            e.mem.set_u32(MATHERR_NAMES + 8 * index, code);
            e.mem.set_u32(MATHERR_NAMES + 8 * index + 4, name);
        }
        e.map(USER_MATHERR_SET, 4);
        e.map(USER_MATHERR, 4);
        e.map(SSE2_FLAG, 4);
        e.set_global(SSE2_FLAG, u32::from(sse2));
        e.call_log = Some(vec![]);
        (e, slot)
    }

    fn floor_of(e: &mut Engine, x: f64) -> f64 {
        e.call(0x00ec_6940, &args![x]).f64()
    }

    /// Values at, around and between integers, the edges of the shift (2^52), large magnitudes,
    /// the smallest numbers and the infinities, each with both signs.
    fn values() -> Vec<f64> {
        let two52 = 2f64.powi(52);
        let positive = [
            0.0,
            0.25,
            0.5,
            0.75,
            f64::from_bits(1.0f64.to_bits() - 1),
            1.0,
            f64::from_bits(1.0f64.to_bits() + 1),
            1.5,
            2.0,
            f64::from_bits(2.0f64.to_bits() - 1),
            2.5,
            3.0,
            3.5,
            123.456,
            1.0e15 + 0.5,
            two52 - 0.5,
            two52 - 1.0,
            two52,
            two52 + 1.0,
            2f64.powi(53),
            1.0e300,
            f64::MAX,
            f64::MIN_POSITIVE,
            f64::from_bits(1),
            f64::from_bits(0x000f_ffff_ffff_ffff),
            f64::INFINITY,
        ];
        positive.iter().flat_map(|&x| [x, -x]).collect()
    }

    #[test]
    fn floor_sse2_matches_the_host_floor_bit_for_bit() {
        let (mut e, _) = engine(true);
        for x in values() {
            let got = floor_of(&mut e, x);
            assert_eq!(got.to_bits(), x.floor().to_bits(), "floor({x:e}) = {got:e}");
        }
        // Nothing of the x87 body ran.
        assert!(calls(&e, CTRLFP).is_empty());
        assert!(calls(&e, FRND).is_empty());
        assert!(calls(&e, LIBM_ERROR_SUPPORT).is_empty());
    }

    #[test]
    fn floor_sse2_of_a_nan_goes_through_the_error_support_with_its_own_slot() {
        let (mut e, errno) = engine(true);
        // The error support as the exe has it: the default _matherr, errno EDOM, the NaN quiet.
        let signalling = f64::from_bits(0x7ff0_0000_0000_0123);
        assert_eq!(
            floor_of(&mut e, signalling).to_bits(),
            0x7ff8_0000_0000_0123
        );
        assert_eq!(e.mem.u32(errno), EDOM);
        let support = calls(&e, LIBM_ERROR_SUPPORT);
        assert_eq!(support.len(), 1);
        let slot = support[0][0];
        assert_eq!(support[0], vec![slot, slot, slot, 0x3ed]);
        assert_eq!(calls(&e, DEFAULT_MATHERR).len(), 1);
        let negative = f64::from_bits(0xfff8_0000_0000_0042);
        assert_eq!(floor_of(&mut e, negative).to_bits(), 0xfff8_0000_0000_0042);
        // A support routine that rewrites the slot: the slot, loaded (FLD quiets it).
        e.register(LIBM_ERROR_SUPPORT, |e, a| {
            assert_eq!(e.mem.u64(a[2]), 0x7ff0_0000_0000_0123);
            e.mem.set_u64(a[2], 0x7ff0_0000_0000_0777);
            Ret::default()
        });
        assert_eq!(
            floor_of(&mut e, signalling).to_bits(),
            0x7ff8_0000_0000_0777
        );
    }

    #[test]
    fn floor_picks_its_body_from_the_flag_and_the_control_words_masks() {
        // The flag clear: the x87 body, whatever the masks.
        let (mut e, _) = engine(false);
        assert_eq!(floor_of(&mut e, -1.5), -2.0);
        assert_eq!(calls(&e, CTRLFP).len(), 2);
        // The flag set (any nonzero value), every exception masked: the SSE2 body, also at
        // 24-bit precision.
        let (mut e, _) = engine(true);
        e.set_global(SSE2_FLAG, 2u32);
        assert_eq!(floor_of(&mut e, -1.5), -2.0);
        set_x87_control_word(&mut e, 0x007f);
        assert_eq!(floor_of(&mut e, 2.5), 2.0);
        assert!(calls(&e, CTRLFP).is_empty());
        // One x87 exception unmasked: the x87 body.
        set_x87_control_word(&mut e, 0x027e);
        assert_eq!(floor_of(&mut e, 2.5), 2.0);
        assert_eq!(
            calls(&e, CTRLFP),
            vec![vec![0x173f, 0xffff], vec![0x027e, 0xffff]]
        );
        assert_eq!(calls(&e, FRND).len(), 1);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn floor_x87_matches_the_host_floor_under_every_old_control_word() {
        let (mut e, _) = engine(false);
        for old in [0x027fu16, 0x007f, 0x0e7f, 0x0a7f, 0x037f] {
            set_x87_control_word(&mut e, old);
            for x in values() {
                let got = floor_of(&mut e, x);
                assert_eq!(
                    got.to_bits(),
                    x.floor().to_bits(),
                    "floor({x:e}) = {got:e} under {old:04x}"
                );
                assert_eq!(x87_control_word(&mut e), old, "the old word is back");
            }
        }
        // __frnd ran under the round-down word (bit 6 reads 1).
        let (mut e, _) = engine(false);
        e.register_double(FRND, |e, a| {
            assert_eq!(x87_control_word(e), 0x177f);
            frnd(e, f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32))).into_ret()
        });
        assert_eq!(floor_of(&mut e, 7.75), 7.0);
        assert_eq!(calls(&e, FRND), vec![args![7.75f64]]);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn floor_x87_reports_an_inexact_result_only_with_the_precision_exception_unmasked() {
        let (mut e, _) = engine(false);
        let seen: Seen = Rc::default();
        let record = seen.clone();
        e.register_double(EXCEPT1, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            42.0f64.into_ret()
        });
        set_x87_control_word(&mut e, 0x025f);
        // Exact: no report.
        assert_eq!(floor_of(&mut e, -3.0), -3.0);
        assert!(seen.borrow().is_empty());
        assert_eq!(x87_control_word(&mut e), 0x025f);
        // Inexact: __except1(0x10, 0xb, x, rounded, old) answers.
        assert_eq!(floor_of(&mut e, 2.5), 42.0);
        assert_eq!(
            *seen.borrow(),
            vec![args![0x10u32, 0xbu32, 2.5f64, 2.0f64, 0x025fu32]]
        );
        // The real __except1: not handled (inexact unmasked), so __raise_exc runs; no error code,
        // so no errno; the old word is put back and the rounded value returned.
        let (mut e, errno) = engine(false);
        let raised: Seen = Rc::default();
        let record = raised.clone();
        e.register_double(RAISE_EXC_EX, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.mem.set_u32(errno, 0);
        set_x87_control_word(&mut e, 0x025f);
        assert_eq!(floor_of(&mut e, -2.5), -3.0);
        assert_eq!(x87_control_word(&mut e), 0x025f);
        assert_eq!(e.mem.u32(errno), 0);
        let raised = raised.borrow();
        assert_eq!(raised.len(), 1);
        assert_eq!(raised[0][2..4], [0x10, 0xb]);
        assert_eq!(raised[0][6], 0);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn floor_x87_sorts_infinities_and_nans_by_sptype() {
        let (mut e, errno) = engine(false);
        // The infinities: back as they are, the old word restored.
        assert_eq!(floor_of(&mut e, f64::INFINITY), f64::INFINITY);
        assert_eq!(floor_of(&mut e, f64::NEG_INFINITY), f64::NEG_INFINITY);
        assert_eq!(
            calls(&e, SPTYPE),
            vec![vec![0, 0x7ff0_0000], vec![0, 0xfff0_0000]]
        );
        assert_eq!(x87_control_word(&mut e), 0x027f);
        // A quiet NaN: __handle_qnan1(0xb, x, old); with the exe's skip flag errno is EDOM and
        // the NaN comes back.
        let quiet = f64::from_bits(0xfff8_0000_0000_0123);
        assert_eq!(floor_of(&mut e, quiet).to_bits(), 0xfff8_0000_0000_0123);
        assert_eq!(e.mem.u32(errno), EDOM);
        assert_eq!(
            calls(&e, HANDLE_QNAN1),
            vec![args![0xbu32, 0xfff8_0000_0000_0123u64, 0x027fu32]]
        );
        assert_eq!(x87_control_word(&mut e), 0x027f);
        // A signalling NaN is quiet in the pushed copy, so it is class 3 too.
        let signalling = f64::from_bits(0x7ff0_0000_0000_0123);
        assert_eq!(
            floor_of(&mut e, signalling).to_bits(),
            0x7ff8_0000_0000_0123
        );
        assert_eq!(
            calls(&e, SPTYPE).last().unwrap(),
            &args![0x7ff8_0000_0000_0123u64]
        );
        // Without the skip flag: _umatherr(1, 0xb, x, 0, x, old), named "floor", errno EDOM.
        e.set_global(MATHERR_SKIP, 0u32);
        e.mem.set_u32(errno, 0);
        assert_eq!(
            floor_of(&mut e, signalling).to_bits(),
            0x7ff8_0000_0000_0123
        );
        assert_eq!(e.mem.u32(errno), EDOM);
        assert_eq!(
            calls(&e, UMATHERR),
            vec![args![
                1u32,
                0xbu32,
                0x7ff8_0000_0000_0123u64,
                0u64,
                0x7ff8_0000_0000_0123u64,
                0x027fu32
            ]]
        );
        assert_eq!(calls(&e, DEFAULT_MATHERR).len(), 1);
        assert_eq!(x87_control_word(&mut e), 0x027f);
        // Any other class: __except1(8, 0xb, x, x + 1.0, old).
        e.register(SPTYPE, |_, _| 4i32.into_ret());
        e.register(EXCEPT1, |_, _| 9.0f64.into_ret());
        assert_eq!(floor_of(&mut e, f64::INFINITY), 9.0);
        assert_eq!(
            calls(&e, EXCEPT1),
            vec![args![8u32, 0xbu32, f64::INFINITY, f64::INFINITY, 0x027fu32]]
        );
    }

    #[test]
    fn sptype_classifies_infinities_and_nans() {
        let (mut e, _) = engine(false);
        let class = |e: &mut Engine, bits: u64| e.call(SPTYPE, &args![bits]).i32();
        assert_eq!(class(&mut e, 0x7ff0_0000_0000_0000), 1);
        assert_eq!(class(&mut e, 0xfff0_0000_0000_0000), 2);
        assert_eq!(class(&mut e, 0x7ff8_0000_0000_0000), 3);
        assert_eq!(class(&mut e, 0xffff_ffff_ffff_ffff), 3);
        assert_eq!(class(&mut e, 0x7ff0_0000_0000_0001), 4);
        assert_eq!(class(&mut e, 0xfff4_0000_0000_0000), 4);
        assert_eq!(class(&mut e, 0x7ff7_ffff_0000_0000), 4);
        for number in [0.0f64, -0.0, 1.0, -2.5, f64::MAX, f64::from_bits(1)] {
            assert_eq!(class(&mut e, number.to_bits()), 0);
        }
    }

    #[test]
    fn frnd_rounds_with_the_control_words_mode_and_ignores_precision() {
        let (mut e, _) = engine(false);
        let cases: [(u16, f64, f64); 18] = [
            // Nearest, ties to even; the sign of a zero result kept.
            (0x027f, 2.5, 2.0),
            (0x027f, 3.5, 4.0),
            (0x027f, -2.5, -2.0),
            (0x027f, -0.4, -0.0),
            (0x027f, 0.5, 0.0),
            // Down.
            (0x067f, 2.5, 2.0),
            (0x067f, -2.5, -3.0),
            (0x067f, -0.4, -1.0),
            (0x067f, 0.4, 0.0),
            // Up.
            (0x0a7f, 2.1, 3.0),
            (0x0a7f, -2.9, -2.0),
            (0x0a7f, -0.4, -0.0),
            // Toward zero.
            (0x0e7f, 2.9, 2.0),
            (0x0e7f, -2.9, -2.0),
            // 24-bit precision does not apply to FRNDINT: 2^24 + 1 needs 25 bits.
            (0x047f, 16_777_217.5, 16_777_217.0),
            (0x007f, 16_777_216.75, 16_777_217.0),
            // Large values and infinities are integers already.
            (0x067f, 1.0e300, 1.0e300),
            (0x0a7f, f64::NEG_INFINITY, f64::NEG_INFINITY),
        ];
        for (word, x, want) in cases {
            set_x87_control_word(&mut e, word);
            let got = e.call(FRND, &args![x]).f64();
            assert_eq!(got.to_bits(), want.to_bits(), "frnd({x}) under {word:04x}");
        }
        // A signalling NaN comes back quiet.
        let got = e.call(FRND, &args![0x7ff0_0000_0000_0005u64]).f64();
        assert_eq!(got.to_bits(), 0x7ff8_0000_0000_0005);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn handle_qnan1_reports_or_sets_edom() {
        let (mut e, errno) = engine(false);
        // The skip flag set: errno EDOM, the word restored, x back (FLD quiets).
        set_x87_control_word(&mut e, 0x0c7f);
        let got = e
            .call(
                HANDLE_QNAN1,
                &args![0xcu32, 0x7ff0_0000_0000_0009u64, 0x027fu32],
            )
            .f64();
        assert_eq!(got.to_bits(), 0x7ff8_0000_0000_0009);
        assert_eq!(e.mem.u32(errno), EDOM);
        assert_eq!(x87_control_word(&mut e), 0x027f);
        assert!(calls(&e, UMATHERR).is_empty());
        // Clear: _umatherr's answer.
        e.set_global(MATHERR_SKIP, 0u32);
        e.register(UMATHERR, |_, _| 5.0f64.into_ret());
        let got = e
            .call(
                HANDLE_QNAN1,
                &args![0xcu32, 0x7ff0_0000_0000_0009u64, 0x007fu32],
            )
            .f64();
        assert_eq!(got, 5.0);
        assert_eq!(
            calls(&e, UMATHERR),
            vec![args![
                1u32,
                0xcu32,
                0x7ff8_0000_0000_0009u64,
                0u64,
                0x7ff8_0000_0000_0009u64,
                0x007fu32
            ]]
        );
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn errcode_and_set_errno_from_matherr_follow_the_flag_order() {
        let (mut e, errno) = engine(false);
        let code = |e: &mut Engine, flags: u32| e.call(ERRCODE, &args![flags]).i32();
        assert_eq!(code(&mut e, 0x3f), 5);
        assert_eq!(code(&mut e, 0x1f), 1);
        assert_eq!(code(&mut e, 0x17), 2);
        assert_eq!(code(&mut e, 0x13), 3);
        assert_eq!(code(&mut e, 0x12), 4);
        assert_eq!(code(&mut e, 0x10), 0);
        // Only the low byte counts.
        assert_eq!(code(&mut e, 0x100), 0);
        for (kind, want) in [(1i32, EDOM), (2, ERANGE), (3, ERANGE)] {
            e.mem.set_u32(errno, 0);
            e.call(SET_ERRNO_FROM_MATHERR, &args![kind]);
            assert_eq!(e.mem.u32(errno), want);
        }
        for kind in [0i32, 4, 5, -1] {
            e.mem.set_u32(errno, 0);
            e.call(SET_ERRNO_FROM_MATHERR, &args![kind]);
            assert_eq!(e.mem.u32(errno), 0, "type {kind}");
        }
    }

    #[test]
    fn umatherr_names_the_function_or_sets_errno_directly() {
        let (mut e, errno) = engine(false);
        // No name for the code: the word restored, errno from the type, the result back.
        set_x87_control_word(&mut e, 0x0c7f);
        let got = e
            .call(
                UMATHERR,
                &args![3u32, 0x99u32, 1.0f64, 2.0f64, 3.0f64, 0x027fu32],
            )
            .f64();
        assert_eq!(got, 3.0);
        assert_eq!(e.mem.u32(errno), ERANGE);
        assert_eq!(x87_control_word(&mut e), 0x027f);
        assert!(calls(&e, DEFAULT_MATHERR).is_empty());
        // A name (the table's last entry): the _exception to _matherr, whose result is the result;
        // a nonzero answer leaves errno alone.
        let seen: Seen = Rc::default();
        let record = seen.clone();
        e.register_double(DEFAULT_MATHERR, move |e, a| {
            let words = (0..8).map(|i| e.mem.u32(a[0] + 4 * i)).collect();
            record.borrow_mut().push(words);
            e.mem.set_u64(a[0] + 0x18, 0x7ff0_0000_0000_0001);
            1u32.into_ret()
        });
        e.mem.set_u32(errno, 0);
        let got = e
            .call(
                UMATHERR,
                &args![2u32, 0x26u32, 1.0f64, -2.0f64, 3.0f64, 0x007fu32],
            )
            .f64();
        assert_eq!(got.to_bits(), 0x7ff8_0000_0000_0001, "FLD quiets it");
        assert_eq!(e.mem.u32(errno), 0);
        let mut want = vec![2u32, 0x010f_320c];
        want.extend(args![1.0f64, -2.0f64, 3.0f64]);
        assert_eq!(*seen.borrow(), vec![want]);
        assert_eq!(x87_control_word(&mut e), 0x007f);
        // Answering 0: errno from the type.
        e.register(DEFAULT_MATHERR, |_, _| 0u32.into_ret());
        e.call(
            UMATHERR,
            &args![1u32, FLOOR_CODE, 1.0f64, 0.0f64, 1.0f64, 0x027fu32],
        );
        assert_eq!(e.mem.u32(errno), EDOM);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn default_matherr_answers_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(DEFAULT_MATHERR, &args![0x1234u32]).u32(), 0);
    }

    /// Three doubles in engine memory (two arguments and the result); answers their addresses.
    fn slots(e: &mut Engine, a: u64, b: u64, r: u64) -> [u32; 3] {
        let at = e.mem.alloc(0x18);
        e.mem.set_u64(at, a);
        e.mem.set_u64(at + 8, b);
        e.mem.set_u64(at + 0x10, r);
        [at, at + 8, at + 0x10]
    }

    #[test]
    fn libm_error_support_reports_each_code_its_way() {
        let (mut e, errno) = engine(false);
        let seen: Exceptions = Rc::default();
        let record = seen.clone();
        e.register_double(DEFAULT_MATHERR, move |e, a| {
            let at = a[0];
            record.borrow_mut().push((
                e.mem.u32(at),
                e.mem.u32(at + 4),
                e.mem.u64(at + 8),
                e.mem.u64(at + 0x10),
                e.mem.u64(at + 0x18),
            ));
            0u32.into_ret()
        });
        let support = |e: &mut Engine, s: [u32; 3], code: u32| {
            e.call(LIBM_ERROR_SUPPORT, &args![s[0], s[1], s[2], code]);
        };
        // 0x3ed (floor): the result becomes argument 1, a domain error named "floor".
        let s = slots(
            &mut e,
            0x7ff0_0000_0000_0001,
            2.0f64.to_bits(),
            3.0f64.to_bits(),
        );
        support(&mut e, s, 0x3ed);
        assert_eq!(e.mem.u64(s[2]), 0x7ff8_0000_0000_0001);
        assert_eq!(
            seen.borrow().last(),
            Some(&(
                1,
                0x0109_43f0,
                0x7ff8_0000_0000_0001,
                2.0f64.to_bits(),
                0x7ff8_0000_0000_0001
            ))
        );
        assert_eq!(e.mem.u32(errno), EDOM);
        // 2: a range error of type 2, errno ERANGE, the result as it was.
        let s = slots(&mut e, 1.0f64.to_bits(), 0, 5.0f64.to_bits());
        support(&mut e, s, 2);
        assert_eq!(seen.borrow().last().unwrap().0, 2);
        assert_eq!(seen.borrow().last().unwrap().1, 0x010f_319c);
        assert_eq!(e.mem.u32(errno), ERANGE);
        assert_eq!(e.mem.u64(s[2]), 5.0f64.to_bits());
        // 15: an underflow (type 4); errno left alone.
        e.mem.set_u32(errno, 0);
        support(&mut e, s, 15);
        assert_eq!(seen.borrow().last().unwrap().0, 4);
        assert_eq!(e.mem.u32(errno), 0);
        // 1010: the result is argument 1 times +0.
        let s = slots(&mut e, (-5.0f64).to_bits(), 0, 7.0f64.to_bits());
        support(&mut e, s, 1010);
        assert_eq!(e.mem.u64(s[2]), (-0.0f64).to_bits());
        assert_eq!(seen.borrow().last().unwrap().4, (-0.0f64).to_bits());
        let s = slots(&mut e, f64::INFINITY.to_bits(), 0, 7.0f64.to_bits());
        support(&mut e, s, 1012);
        assert!(f64::from_bits(e.mem.u64(s[2])).is_nan());
        // 0x1a: 1.0, no report; an unknown code: nothing at all.
        let reports = seen.borrow().len();
        let s = slots(&mut e, 0, 0, 7.0f64.to_bits());
        support(&mut e, s, 0x1a);
        assert_eq!(e.mem.u64(s[2]), 1.0f64.to_bits());
        for code in [0x99u32, 0, 999, 1013, 0xffff_ffff] {
            e.mem.set_u64(s[2], 7.0f64.to_bits());
            support(&mut e, s, code);
            assert_eq!(e.mem.u64(s[2]), 7.0f64.to_bits());
        }
        assert_eq!(seen.borrow().len(), reports);
        // A user _matherr (decoded) that handles it and changes the result.
        e.set_global(USER_MATHERR_SET, 1u32);
        e.set_global(USER_MATHERR, 0x1234_5678u32);
        e.register(DECODE_POINTER, |_, a| {
            assert_eq!(a[0], 0x1234_5678);
            USER_HANDLER.into_ret()
        });
        e.register(USER_HANDLER, |e, a| {
            e.mem.set_u64(a[0] + 0x18, 11.0f64.to_bits());
            1u32.into_ret()
        });
        e.mem.set_u32(errno, 0);
        let s = slots(&mut e, 0, 0, 7.0f64.to_bits());
        support(&mut e, s, 1006);
        assert_eq!(e.mem.u64(s[2]), 11.0f64.to_bits());
        assert_eq!(e.mem.u32(errno), 0);
        assert_eq!(seen.borrow().len(), reports);
    }

    /// `__handle_exc(flags, &value, control)`: its answer and the value after.
    fn handle(e: &mut Engine, flags: u32, value: f64, control: u32) -> (i32, f64) {
        let at = e.mem.alloc(8);
        e.mem.set_u64(at, value.to_bits());
        let handled = e.call(HANDLE_EXC, &args![flags, at, control]).i32();
        (handled, e.mem.f64(at))
    }

    #[test]
    fn handle_exc_handles_masked_exceptions() {
        let (mut e, _) = engine(false);
        // Bits 8 and 4 with their masks.
        assert_eq!(handle(&mut e, 8, 1.0, 1), (1, 1.0));
        assert_eq!(handle(&mut e, 9, 1.0, 1).0, 0, "bit 1 left over");
        assert_eq!(handle(&mut e, 8, 1.0, 0).0, 0);
        assert_eq!(handle(&mut e, 4, 1.0, 4), (1, 1.0));
        assert_eq!(calls(&e, SET_STATFP), vec![vec![1], vec![1], vec![4]]);
        // Bit 0x10 with mask 0x20, also after another one.
        assert_eq!(handle(&mut e, 0x10, 1.0, 0x20), (1, 1.0));
        assert_eq!(handle(&mut e, 0x10, 1.0, 0x1f).0, 0);
        assert_eq!(handle(&mut e, 0x14, 1.0, 0x24), (1, 1.0));
        assert_eq!(
            handle(&mut e, 0x20, 1.0, 0x3f).0,
            1,
            "bits above 0x1f are ignored"
        );
    }

    #[test]
    fn handle_exc_gives_the_overflow_value_of_each_rounding_mode() {
        let (mut e, _) = engine(false);
        let inf = f64::INFINITY;
        let max = f64::MAX;
        let nan = f64::from_bits(0x7ff8_0000_0000_0000);
        for (rounding, plus, minus) in [
            (0u32, inf, -inf),
            (0x400, max, -inf),
            (0x800, inf, -max),
            (0xc00, max, -max),
        ] {
            let control = 0x8 | rounding;
            assert_eq!(handle(&mut e, 1, 2.0, control), (1, plus));
            assert_eq!(handle(&mut e, 1, -2.0, control), (1, minus));
            // A NaN or zero counts as negative.
            assert_eq!(handle(&mut e, 1, nan, control).1, minus);
            assert_eq!(handle(&mut e, 1, 0.0, control).1, minus);
        }
        // Unmasked: nothing stored, not handled.
        assert_eq!(handle(&mut e, 1, 2.0, 0x37), (0, 2.0));
    }

    #[test]
    fn handle_exc_denormalizes_an_underflow() {
        let (mut e, _) = engine(false);
        let p = |k: i32| 2f64.powi(k);
        // 2^-1030, a denormal (powi would round through an overflow).
        let tiny = f64::from_bits(1 << 44);
        // The result comes scaled by 2^0x600 (the wrapped exponent): 2^506 is 2^-1030.
        assert_eq!(handle(&mut e, 2, p(506), 0x10), (1, tiny));
        assert_eq!(handle(&mut e, 2, -p(506), 0x10), (1, -tiny));
        assert!(calls(&e, SET_STATFP).is_empty(), "exact");
        // A bit shifted out is inexact.
        let value = f64::from_bits(p(506).to_bits() + 1);
        assert_eq!(handle(&mut e, 2, value, 0x10), (1, tiny));
        assert_eq!(calls(&e, SET_STATFP), vec![vec![0x10]]);
        // At the smallest normal no shift is made.
        assert_eq!(handle(&mut e, 2, p(514), 0x10), (1, f64::MIN_POSITIVE));
        // Below the smallest denormal: the mantissa times 0, inexact.
        let (handled, got) = handle(&mut e, 2, p(456), 0x10);
        assert_eq!((handled, got.to_bits()), (1, 0));
        let (_, got) = handle(&mut e, 2, -p(456), 0x10);
        assert_eq!(got.to_bits(), SIGN);
        // A zero result stays, flagged inexact; flag 0x10 given flags it too, and is left over
        // without its mask.
        assert_eq!(handle(&mut e, 2, 0.0, 0x10), (1, 0.0));
        let statfp = calls(&e, SET_STATFP).len();
        assert_eq!(handle(&mut e, 0x12, p(506), 0x10), (0, tiny));
        assert_eq!(calls(&e, SET_STATFP).len(), statfp + 1);
        // Unmasked: nothing.
        assert_eq!(handle(&mut e, 2, p(506), 0x2f), (0, p(506)));
    }

    #[test]
    fn decomp_and_set_exp_split_and_rebuild_doubles() {
        let (mut e, _) = engine(false);
        let split = |e: &mut Engine, x: f64| {
            let at = e.mem.alloc(4);
            let mantissa = e.call(DECOMP, &args![x, at]).f64();
            (mantissa.to_bits(), e.mem.i32(at))
        };
        let tiny = f64::from_bits(1);
        let three_tiny = -f64::from_bits(3);
        assert_eq!(split(&mut e, 6.0), (0.75f64.to_bits(), 3));
        assert_eq!(split(&mut e, -6.0), ((-0.75f64).to_bits(), 3));
        assert_eq!(split(&mut e, 0.0), (0, 0));
        assert_eq!(split(&mut e, -0.0), (0, 0), "+0 for -0");
        assert_eq!(split(&mut e, tiny), (0.5f64.to_bits(), -1073));
        assert_eq!(split(&mut e, three_tiny), ((-0.75f64).to_bits(), -1072));
        assert_eq!(split(&mut e, f64::INFINITY), (0.5f64.to_bits(), 0x401));
        let rebuild = |e: &mut Engine, x: u64, exponent: i32| {
            e.call(SET_EXP, &args![x, exponent]).f64().to_bits()
        };
        assert_eq!(rebuild(&mut e, 6.0f64.to_bits(), 0), 0.75f64.to_bits());
        assert_eq!(rebuild(&mut e, (-6.0f64).to_bits(), 2), (-3.0f64).to_bits());
        // The top word from the argument as passed, then FLD: a NaN pattern comes back quiet.
        assert_eq!(
            rebuild(&mut e, 0x7ff0_0000_0000_0001, 0x401),
            0x7ff8_0000_0000_0001
        );
        // The exponent wraps into the sign bit, leaving the exponent field 0.
        assert_eq!(
            rebuild(&mut e, 1.0f64.to_bits(), 0x402),
            (-0.0f64).to_bits()
        );
    }

    #[test]
    fn except1_handles_or_raises_then_reports() {
        let (mut e, errno) = engine(false);
        let raised: Seen = Rc::default();
        let record = raised.clone();
        e.register_double(RAISE_EXC_EX, move |e, a| {
            record.borrow_mut().push(a.to_vec());
            // The handler may change the result and the control word in place.
            e.mem.set_u64(a[5], 7.0f64.to_bits());
            e.mem.set_u32(a[1], 0x0a7f);
            Ret::default()
        });
        let except = |e: &mut Engine, flags: u32, x: f64, r: f64, control: u32| {
            e.call(EXCEPT1, &args![flags, 0xbu32, x, r, control]).f64()
        };
        // Handled (invalid masked), the skip flag set: errno from the type, the word restored.
        set_x87_control_word(&mut e, 0x0c7f);
        let got = except(&mut e, 8, f64::INFINITY, f64::INFINITY, 0x027f);
        assert_eq!(got, f64::INFINITY);
        assert_eq!(e.mem.u32(errno), EDOM);
        assert_eq!(x87_control_word(&mut e), 0x027f);
        assert!(raised.borrow().is_empty());
        assert!(calls(&e, UMATHERR).is_empty());
        // The skip flag clear: _umatherr(errcode, code, x, 0, result, cw).
        e.set_global(MATHERR_SKIP, 0u32);
        let got = except(&mut e, 1, 3.0, 1.0e308, 0x027f);
        assert_eq!(got, f64::INFINITY, "overflow to +inf (round to nearest)");
        assert_eq!(
            calls(&e, UMATHERR),
            vec![args![3u32, 0xbu32, 3.0f64, 0u64, f64::INFINITY, 0x027fu32]]
        );
        assert_eq!(e.mem.u32(errno), ERANGE);
        // Not handled: __raise_exc(&local, &cw, flags, code, &x, &result) with a 0 after; what it
        // leaves is used.
        e.set_global(MATHERR_SKIP, 1u32);
        let got = except(&mut e, 0x10, 2.5, 2.0, 0x025f);
        assert_eq!(got, 7.0);
        assert_eq!(x87_control_word(&mut e), 0x0a7f);
        let raised = raised.borrow();
        assert_eq!(raised.len(), 1);
        let call = &raised[0];
        assert_eq!(call[1], call[0] + 0x90);
        assert_eq!(call[2..4], [0x10, 0xb]);
        assert_eq!(
            (call[4], call[5], call[6]),
            (call[0] + 0x80, call[0] + 0x88, 0)
        );
        assert_eq!(calls(&e, RAISE_EXC), vec![call[..6].to_vec()]);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn status_flag_helper_reads_its_constants_only() {
        let (mut e, _) = engine(false);
        for flags in [0u32, 1, 4, 8, 0x10, 0x20, 0x3f] {
            assert_eq!(e.call(SET_STATFP, &args![flags]), Ret::default());
        }
        // The loads read the constants: without them the run stops; the other bits read nothing.
        let mut bare = Engine::new();
        // Engine::new maps the page of 011c2b60; the constants' words are dropped again.
        bare.mem.unmap(0x011c_2000, 0x1000);
        let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            bare.call(SET_STATFP, &args![1u32]);
        }));
        assert!(stopped.is_err());
        bare.call(SET_STATFP, &args![0x24u32]);
    }

    /// A `CPUID` leaf's answer: `EAX`, `EBX`, `ECX`, `EDX`.
    type Cpuid = [u32; 4];

    /// Example processor answers (not a measured machine's): leaf 0 with the highest leaf 0xd
    /// and the vendor words, leaf 1 with SSE2 (bit 26), and leaf 1 with SSE but not SSE2.
    const LEAF_0: Cpuid = [0xd, 0x756e_6547, 0x6c65_746e, 0x4965_6e69];
    const LEAF_1_SSE2: Cpuid = [0x0001_0676, 0x0001_0800, 0x0008_e3bd, 0xbfeb_fbff];
    const LEAF_1_NO_SSE2: Cpuid = [0x0000_0683, 0, 0, 0x0383_f9ff];

    /// An engine whose platform answers the ID flag test with `id_change` and `CPUID` leaves 0
    /// and 1 with `leaf_1` (each checking its leaf), `__use_sse2_mathfcns` mapped and holding a
    /// stale word, and the call log on.
    fn processor(id_change: u32, leaf_1: Cpuid) -> Engine {
        let mut e = Engine::new();
        e.register_double(EFLAGS_ID_TEST, move |_, a| {
            assert!(a.is_empty());
            id_change.into_ret()
        });
        for (at, leaf, regs) in [(CPUID_LEAF_0, 0u32, LEAF_0), (CPUID_LEAF_1, 1, leaf_1)] {
            e.register_double(at, move |e, a| {
                assert_eq!(a.len(), 2);
                assert_eq!(a[0], leaf);
                for (i, word) in regs.iter().enumerate() {
                    e.mem.set_u32(a[1] + 4 * i as u32, *word);
                }
                Ret::default()
            });
        }
        e.map(SSE2_FLAG, 4);
        e.set_global(SSE2_FLAG, 0xdead_beefu32);
        e.call_log = Some(Vec::new());
        e
    }

    #[test]
    fn the_sse2_switch_is_set_when_cpuid_has_sse2_and_the_probe_passes() {
        let mut e = processor(0x0020_0000, LEAF_1_SSE2);
        assert_eq!(e.call(0x00ed_454f, &args![]), Ret::default());
        assert_eq!(e.global::<u32>(SSE2_FLAG), 1);
        let leaf_0 = calls(&e, CPUID_LEAF_0);
        let leaf_1 = calls(&e, CPUID_LEAF_1);
        assert_eq!(leaf_0.len(), 1);
        assert_eq!(leaf_0[0][0], 0);
        assert_eq!(leaf_1.len(), 1);
        assert_eq!(leaf_1[0][0], 1);
        // Both answers go to the same four words.
        assert_eq!(leaf_0[0][1], leaf_1[0][1]);
        assert_eq!(calls(&e, GET_SSE2_INFO), vec![Vec::<u32>::new()]);
        assert_eq!(calls(&e, OS_SSE_PROBE), vec![Vec::<u32>::new()]);
        // The ID flag set at entry reads back as a negative difference: still a change.
        let mut e = processor(0xffe0_0000, LEAF_1_SSE2);
        assert_eq!(e.call(0x00ed_454f, &args![]).u32(), 0);
        assert_eq!(e.global::<u32>(SSE2_FLAG), 1);
    }

    #[test]
    fn the_sse2_switch_stays_clear_without_sse2_cpuid_or_the_os_probe() {
        // CPUID leaf 1 without bit 26: 0, and the probe is not called.
        let mut e = processor(0x0020_0000, LEAF_1_NO_SSE2);
        assert_eq!(e.call(0x00ed_454f, &args![]).u32(), 0);
        assert_eq!(e.global::<u32>(SSE2_FLAG), 0);
        assert_eq!(calls(&e, CPUID_LEAF_1).len(), 1);
        assert!(calls(&e, OS_SSE_PROBE).is_empty());
        // The ID flag does not change: no CPUID at all, the cleared words give 0.
        let mut e = processor(0, LEAF_1_SSE2);
        assert_eq!(e.call(0x00ed_454f, &args![]).u32(), 0);
        assert_eq!(e.global::<u32>(SSE2_FLAG), 0);
        assert!(calls(&e, CPUID_LEAF_0).is_empty());
        assert!(calls(&e, CPUID_LEAF_1).is_empty());
        assert!(calls(&e, OS_SSE_PROBE).is_empty());
        // SSE2 in CPUID but the probe answers 0: 0; any nonzero probe answer gives 1.
        for (probe, flag) in [(0u32, 0u32), (2, 1)] {
            let mut e = processor(0x0020_0000, LEAF_1_SSE2);
            e.register_double(OS_SSE_PROBE, move |_, _| probe.into_ret());
            assert_eq!(e.call(GET_SSE2_INFO, &args![]).u32(), flag);
            assert_eq!(e.call(0x00ed_454f, &args![]).u32(), 0);
            assert_eq!(e.global::<u32>(SSE2_FLAG), flag);
        }
    }

    #[test]
    fn the_os_sse_probe_passes_and_its_filter_handles_two_codes() {
        let mut e = Engine::new();
        assert_eq!(e.call(OS_SSE_PROBE, &args![]).u32(), 1);
        assert_eq!(os_sse_probe_filter(0xc000_0005), 1);
        assert_eq!(os_sse_probe_filter(0xc000_001d), 1);
        assert_eq!(os_sse_probe_filter(0xc000_0096), 0);
        assert_eq!(os_sse_probe_filter(0), 0);
    }

    #[test]
    fn the_c_initializer_walk_runs_the_sse2_switch() {
        let mut e = processor(0x0020_0000, LEAF_1_SSE2);
        // Both tables mapped (the C++ one ends just before the C one), empty but for `__cinit`'s
        // C initializer slot `010113dc`.
        let end = crate::static_init::C_TABLE.1;
        let cpp_start = crate::static_init::CPP_TABLE.0;
        e.map(cpp_start, end - cpp_start);
        e.mem.set_u32(0x0101_13dc, 0x00ed_454f);
        let report = crate::static_init::run(&mut e, &[]);
        assert_eq!(report.translated, 1);
        assert!(report.stopped.is_empty() && report.open.is_empty());
        assert_eq!(report.c_failed, None);
        assert_eq!(e.global::<u32>(SSE2_FLAG), 1);
    }
}

/// Tests of the x87 machinery, `pow` and the other bodies above, on the constants they read
/// (each set to the exe's value where a test reads it). The x87 results are checked against the
/// same instruction sequence run through `crate::x87`'s helpers; the vectors marked as recorded
/// were produced by the game's own code (`FalloutNV.exe` 1.4.0.525) for those arguments, with
/// `__87except` and `___libm_error_support` doubled as they are here. The SSE2 bodies' main paths
/// need the exe's tables (several kilobytes, never copied into the repository): those vectors run
/// with `NV_EXE` set to the installed executable (`cargo test ... -- --ignored`).
#[cfg(test)]
mod tests_bodies {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What a double saw.
    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;
    /// A recorded vector: the arguments, the result, and the error report (for the x87 bodies
    /// `__87except`'s `type << 16 | operation`, for the SSE2 ones `___libm_error_support`'s code;
    /// 0 for none).
    type Vector = (u64, u64, u64, u32);

    /// `__use_sse2_mathfcns`.
    const SSE2_FLAG: u32 = 0x0127_0a64;

    fn put(e: &mut Engine, at: u32, bits: u64) {
        e.map(at, 8);
        e.mem.set_u64(at, bits);
    }

    fn put_extended(e: &mut Engine, at: u32, sign_exponent: u16, significand: u64) {
        e.map(at, 10);
        e.mem.set_u64(at, significand);
        e.mem.set_u16(at + 8, sign_exponent);
    }

    /// An engine with the constants the x87 bodies and the shared exits read, at the exe's
    /// values, the math-error switch clear, `__use_sse2_mathfcns` as given, the call log on, and
    /// `__87except` / `___libm_error_support` doubled to record their words (and, for
    /// `__87except`, the `_exception`'s type) and change nothing.
    fn engine(sse2: bool) -> (Engine, Seen, Seen) {
        let mut e = Engine::new();
        e.map(SSE2_FLAG, 4);
        e.set_global(SSE2_FLAG, u32::from(sse2));
        e.map(ERROR_SWITCH, 4);
        put(&mut e, OVERFLOW_BOUND, f64::MAX.to_bits());
        put(&mut e, UNDERFLOW_BOUND, f64::MIN_POSITIVE.to_bits());
        put(&mut e, OVERFLOW_SCALE, (-1536.0f64).to_bits());
        put(&mut e, UNDERFLOW_SCALE, 1536.0f64.to_bits());
        put(&mut e, OVERFLOW_FACTOR, f64::INFINITY.to_bits());
        put(&mut e, UNDERFLOW_FACTOR, 0);
        put(&mut e, POW_HALF, 0.5f64.to_bits());
        put_extended(&mut e, POW_POLE, 0x7fff, 1 << 63);
        put_extended(&mut e, INDEFINITE, 0xffff, 0xc000_0000_0000_0000);
        put(&mut e, POW_SPECIAL_INFINITY, f64::INFINITY.to_bits());
        put(
            &mut e,
            POW_SPECIAL_ONE_TO_MINUS_INFINITY,
            0xfff8_0000_0000_0000,
        );
        put(&mut e, POW_SPECIAL_MINUS_ZERO, (-0.0f64).to_bits());
        put(&mut e, INTTYPE_HALF, 0.5f64.to_bits());
        let except: Seen = Rc::default();
        let record = except.clone();
        e.register_double(EXCEPT87, move |e, a| {
            let mut w = a.to_vec();
            w.push(e.mem.u32(a[1]));
            record.borrow_mut().push(w);
            Ret::default()
        });
        let libm: Seen = Rc::default();
        let record = libm.clone();
        e.register_double(LIBM_ERROR_SUPPORT, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, except, libm)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The error a vector records: `__87except`'s last type and operation, or 0.
    fn except_seen(except: &Seen) -> u32 {
        except.borrow().last().map_or(0, |w| (w[3] << 16) | w[0])
    }

    fn pow_of(e: &mut Engine, x: u64, y: u64) -> u64 {
        e.call(0x00ec_9ac0, &args![x, y]).f64().to_bits()
    }

    /// A small linear congruential generator for spread arguments.
    struct Spread(u64);

    impl Spread {
        fn next(&mut self) -> f64 {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (self.0 >> 11) as f64 / (1u64 << 53) as f64
        }
    }

    /// `pow`'s finite path as the exe runs it: `FYL2X` on `|x|`, then `00ed2780`'s sequence, under
    /// the body's control word (the caller's precision, round to nearest), negated for a negative
    /// base to an odd power, rounded to a double under the caller's word.
    fn pow_sequence(x: f64, y: f64, word: u16) -> u64 {
        crate::x87::set_control_word((word & 0x300) | 0x7f);
        let t = x87_fyl2x(X87Ext::load64(y), X87Ext::load64(x.abs()));
        let n = x87_frndint(t);
        let f = x87_fchs(x87_fsub(n, t));
        let mut r = x87_fscale(x87_fadd(x87_f2xm1(f), X87Ext::load64(1.0)), n);
        if x < 0.0 && y % 2.0 != 0.0 {
            r = x87_fchs(r);
        }
        crate::x87::set_control_word(word);
        let bits = r.store64().to_bits();
        crate::x87::set_control_word(crate::x87::DEFAULT_CONTROL_WORD);
        bits
    }

    #[test]
    fn pow_x87_runs_fyl2x_and_the_power_of_two_sequence_bit_for_bit() {
        let (mut e, except, _) = engine(false);
        let mut spread = Spread(0x5eed);
        for word in [0x027fu16, 0x007f, 0x0e7f, 0x0a7f] {
            for i in 0..100 {
                let x = 0.05 + spread.next() * 20.0;
                let y = (spread.next() - 0.5) * 30.0;
                let (x, y) = match i % 4 {
                    // A negative base with an integer exponent, odd and even.
                    0 => (-x, y.round()),
                    1 => (x, y.round()),
                    _ => (x, y),
                };
                set_x87_control_word(&mut e, word);
                let got = pow_of(&mut e, x.to_bits(), y.to_bits());
                assert_eq!(x87_control_word(&mut e), word, "the caller's word is back");
                set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
                assert_eq!(
                    got,
                    pow_sequence(x, y, word),
                    "pow({x:e}, {y:e}) under {word:04x}"
                );
            }
        }
        assert!(except.borrow().is_empty());
        // The register form takes the same body.
        let got = e.call(0x00ec_9b00, &args![1.5f64, 2.25f64]).f64().to_bits();
        assert_eq!(got, pow_sequence(1.5, 2.25, 0x027f));
        assert!(calls(&e, POW_SSE2).is_empty() && calls(&e, POW_SSE2_REGISTERS).is_empty());
    }

    /// Recorded from the exe: `pow` (`00ec9ac0`) on its x87 path under the control word 0x027f.
    const POW_X87_RECORDED: &[Vector] = &[
        (
            0x7ff00000_00000000,
            0x7fefffff_ffffffff,
            0x7ff00000_00000000,
            0,
        ),
        (
            0xfff80000_00000000,
            0x7ff40000_00000000,
            0xfff80000_00000000,
            0x1_001d,
        ),
        (
            0x7ff40000_00000000,
            0x43300000_00000001,
            0x7ffc0000_00000000,
            0x7_001d,
        ),
        (
            0x00000000_00000001,
            0x40000000_00000000,
            0x19b00000_00000000,
            0x4_001d,
        ),
        (
            0x3ff00000_00000000,
            0xfff00000_00000000,
            0xfff80000_00000000,
            0x1_001d,
        ),
        (
            0x40000000_00000000,
            0x3ff00000_00000001,
            0x40000000_00000001,
            0,
        ),
        (
            0xbfe00000_00000000,
            0x800fffff_ffffffff,
            0xfff80000_00000000,
            0x1_001d,
        ),
        (
            0xffefffff_ffffffff,
            0x00080000_00000000,
            0xfff80000_00000000,
            0x1_001d,
        ),
        (
            0x3ff00000_00000001,
            0x7fefffff_ffffffff,
            0x7ff00000_00000000,
            0x3_001d,
        ),
        (
            0xc3400000_00000000,
            0x7ff40000_00000000,
            0x7ffc0000_00000000,
            0x7_001d,
        ),
        (
            0xc08ff000_00000000,
            0x43300000_00000001,
            0xfff00000_00000000,
            0x3_001d,
        ),
        (
            0x00080000_00000000,
            0x40000000_00000000,
            0x20100000_00000000,
            0x4_001d,
        ),
        (
            0xbe2b224d_dcb5aef4,
            0x3ec88018_5da6ac0f,
            0xfff80000_00000000,
            0x1_001d,
        ),
        (
            0x3fdbb172_089be200,
            0x3ec56308_d14e8e56,
            0x3feffffb_85650af3,
            0,
        ),
        (
            0xbfe16551_2df98680,
            0x40380000_00000000,
            0x3e9dc5fc_41ca29b1,
            0,
        ),
        (
            0xbf0e1345_cd757865,
            0xbff5ac21_b3e0ee80,
            0xfff80000_00000000,
            0x1_001d,
        ),
        (
            0xc1162db1_7ad92243,
            0xbff97655_1caf6640,
            0xfff80000_00000000,
            0x1_001d,
        ),
        (
            0xc03b0000_00000000,
            0x41140b51_6d146856,
            0xfff80000_00000000,
            0x1_001d,
        ),
        (
            0x3ebc188b_aaa4f05d,
            0x7ff00000_00000000,
            0x00000000_00000000,
            0,
        ),
        (
            0x40100000_00000000,
            0xc0101ae0_0cce8bcb,
            0x3f6edb4d_333644e1,
            0,
        ),
        (
            0x3cb00000_00000000,
            0xc1179175_4660d32b,
            0x7ff00000_00000000,
            0x3_001d,
        ),
        (
            0x935748ea_ab858392,
            0x43400000_00000000,
            0x00000000_00000000,
            0x4_001d,
        ),
        (
            0x4048c539_08ae354b,
            0xbfe00000_00000000,
            0x3fc22f85_b7545978,
            0,
        ),
        (
            0xc0240000_00000000,
            0x7ff80000_00000000,
            0x7ff80000_00000000,
            0x1_001d,
        ),
        (
            0x3fd38684_68436400,
            0xc0330000_00000000,
            0x41f74a0f_24aff80e,
            0,
        ),
        (
            0xc1a741b0_15057da1,
            0x43400000_00000000,
            0x7ff00000_00000000,
            0x3_001d,
        ),
        (
            0x401c0000_00000000,
            0xc03d0000_00000000,
            0x3ad8076f_555a58e9,
            0,
        ),
        (
            0x40240000_00000000,
            0x8cd9bfec_09779247,
            0x3ff00000_00000000,
            0,
        ),
        (
            0xc0dd42dc_e38ad6fd,
            0x401c0000_00000000,
            0xc6711b15_743e1ca8,
            0,
        ),
        (
            0x3fefffff_ffffffff,
            0x93bb5832_cd0b6033,
            0x3ff00000_00000000,
            0,
        ),
        (
            0x3e81795b_b6ff04da,
            0xc1196571_5e8ccb3d,
            0x7ff00000_00000000,
            0x3_001d,
        ),
        (
            0x6f0aa47a_2a391956,
            0x3ff46bfb_b4d7b240,
            0x7c10743f_36a37499,
            0,
        ),
        (
            0x3ff68c22_9e64dd7c,
            0x3e389c7c_49ef9c8f,
            0x3ff00000_00871435,
            0,
        ),
        (
            0xbff00000_00000000,
            0xffefffff_ffffffff,
            0x3ff00000_00000000,
            0,
        ),
        (
            0x3f83f645_0bde2000,
            0x3e7ed5ff_33fe3be6,
            0x3feffffe_e26926ed,
            0,
        ),
        (
            0x3ec63652_cc49ede1,
            0x40360000_00000000,
            0x26754a7d_031074bc,
            0,
        ),
        (
            0x400a48ca_de03a9b3,
            0x40539427_60d1719c,
            0x485517da_8466a96f,
            0,
        ),
        (
            0x4007cfc8_d467039f,
            0x40542fee_3dab466c,
            0x47e0c049_c9e1f051,
            0,
        ),
        (
            0x3fd28804_bfc0d4d3,
            0xc04a0ab9_1129a588,
            0x45c1864c_79a16a81,
            0,
        ),
        (
            0x402dd740_06aea343,
            0x4047dc56_51accc18,
            0x4b90dd6a_3ae33a95,
            0,
        ),
        (
            0x401980fd_6ce2236b,
            0xc03b509b_e7173c90,
            0x3b5fef44_27f98cff,
            0,
        ),
        (
            0x40222911_5d8629c3,
            0xc03c2f66_1650c950,
            0x3a539f34_61a58e28,
            0,
        ),
        (
            0x4003b668_e4a368fc,
            0xc049dd48_386f0708,
            0x3bb9f813_66cb7fa7,
            0,
        ),
        (
            0x40228c6f_b2d2f49b,
            0xc040927e_1d950568,
            0x39469c6f_4494179e,
            0,
        ),
        (
            0x3fff0b4e_0cfe2e56,
            0x4041ecd0_3e402038,
            0x42137313_4e13c038,
            0,
        ),
        (
            0x40260c22_aa141f76,
            0x4053e029_547d74ec,
            0x51236ec7_8cc72602,
            0,
        ),
        (
            0x3fd30032_4a36484b,
            0xc058e320_60351444,
            0x4ad54614_be4cb866,
            0,
        ),
        (
            0x40312e48_7655e4a7,
            0xc04b2d03_d7e496e8,
            0x32001c8b_2beed351,
            0,
        ),
        (
            0x3fb35f6e_738ee05d,
            0x40416c1f_8ba61eb8,
            0x37d2dad6_0ee0c652,
            0,
        ),
        (
            0x3ffbd1ab_08ead764,
            0x4055dd36_e8303c2c,
            0x444bab62_82a565cb,
            0,
        ),
        (
            0x40280566_7e68fd7a,
            0x4056e93f_09c27afc,
            0x54793fc3_b433e993,
            0,
        ),
        (
            0x3fac20d0_eab70586,
            0x4058ac56_71af0bcc,
            0x261d2c64_2625a901,
            0,
        ),
        (
            0x403030c2_a79254d8,
            0x40581202_9ca32e9c,
            0x581b461d_1c7641f4,
            0,
        ),
        (
            0x40129120_fce685a9,
            0x4030018f_67508db0,
            0x4225d3e1_ccdabd42,
            0,
        ),
        (
            0x3ffbe171_62568ecb,
            0x404fc0b8_0ffe5478,
            0x431d730b_7e007e05,
            0,
        ),
        (
            0x401b47de_28d55830,
            0x4037d0b1_c9a60c30,
            0x440f340c_79a22301,
            0,
        ),
        (
            0x3fd0f0c7_216db4c4,
            0xc023db09_90fc9120,
            0x41206bd2_47d74b2a,
            0,
        ),
        (
            0x3ff65513_876a7398,
            0x40569d4d_55b92aac,
            0x42a6e078_d42ee66c,
            0,
        ),
        (
            0x4007ce4b_c4dba72f,
            0x4047223c_11a1f2f8,
            0x447baa42_8962d987,
            0,
        ),
        (
            0x3fbc8d79_691d3eb3,
            0xc030bdeb_e5cb96d0,
            0x433f8891_e2e72a46,
            0,
        ),
        (
            0x4000a679_bc1b6c0e,
            0x40437744_17b59a38,
            0x4281fe53_d69e78dd,
            0,
        ),
        (
            0x3faac7a5_b0e7d558,
            0xc043ea65_dfa15c28,
            0x4a879049_9f705161,
            0,
        ),
        (
            0x4026502c_c3dd6247,
            0x405128a6_0287e8bc,
            0x4edc9af4_ed5be1e4,
            0,
        ),
        (
            0x4025a269_d3724c1f,
            0x4055693b_a669d18c,
            0x52528202_8542edc9,
            0,
        ),
        (
            0x3fc11178_e871bc11,
            0xc0472b44_de3b6748,
            0x4859df9e_84f3e47e,
            0,
        ),
        (
            0x3fbb43a3_0fd68a9d,
            0xc03f994a_e9e59b50,
            0x46511f18_3106fbb8,
            0,
        ),
        (
            0x3fc8ed17_e585840c,
            0x4045bb04_e6e7eff8,
            0x3985511f_c8de8c97,
            0,
        ),
        (
            0x3fe014fd_0e35df35,
            0xc03f4726_b6c15cd0,
            0x41e08884_748ef2ef,
            0,
        ),
        (
            0x3ffb4409_4a89d866,
            0x40578dea_1f46eb9c,
            0x4475ee97_632508a4,
            0,
        ),
        (
            0x3fc6443c_b89cb5c4,
            0xc03b0a8d_17c3fe50,
            0x4432c4af_4af48be1,
            0,
        ),
        (
            0x3fb005b4_2cbce5c3,
            0xc051b31c_9e1ad8c4,
            0x51a0975c_b0da5226,
            0,
        ),
        (
            0x4013d9aa_8e0a78a8,
            0x4054b919_928ea00c,
            0x4be7c4aa_6ef10faa,
            0,
        ),
        (
            0x3ff86b4a_3ea591e7,
            0x404c46b8_02b155b8,
            0x421686ae_4525a0d4,
            0,
        ),
        (
            0x401401b0_37f37474,
            0x40319a8a_e8621eb0,
            0x427d811f_29280eac,
            0,
        ),
        (
            0x3fb08cf1_d9a07550,
            0x4050c9ce_5c2b767c,
            0x2f595d77_9ecdd014,
            0,
        ),
        (
            0x3ff46da7_7d929490,
            0x4052e0fc_7776b74c,
            0x4198936c_8dfac2a2,
            0,
        ),
        (
            0x3fcfcf3f_b3600f2b,
            0x40554ff7_09678a1c,
            0x353b3a3c_3c6b60d1,
            0,
        ),
        (
            0x400001e8_8c3cf070,
            0xc046df59_cc19a228,
            0x3d12b117_5afb7940,
            0,
        ),
        (
            0x3feb332e_872bda98,
            0xc0161ad8_7691a440,
            0x4003a3da_3ee3ee22,
            0,
        ),
        (
            0x3fde1d15_06478249,
            0xc037ff8d_9aa045d0,
            0x41912ba5_fcf52993,
            0,
        ),
        (
            0x3fd84fb9_4f9d276a,
            0x4027db14_8e1c4ae0,
            0x3ee44ced_b459f0e1,
            0,
        ),
        (
            0x3ff9586b_16b4f256,
            0xc05501fd_ca1109d4,
            0x3c72ccc9_1a3d1e37,
            0,
        ),
        (
            0x3fafdace_a7670d1f,
            0x40407d78_aaeee9f8,
            0x37ad1753_62602782,
            0,
        ),
        (
            0xc01e17c3_9ee7306b,
            0x40544000_00000000,
            0xceac3af5_fa9e1f40,
            0,
        ),
        (
            0xc030f8cd_2ea143fe,
            0x40330000_00000000,
            0xc4c8876c_71ff920e,
            0,
        ),
        (
            0xbfdbc901_da6c1b03,
            0xc04e0000_00000000,
            0x4472b485_e259c88b,
            0,
        ),
        (
            0xbfc094f6_dc8caf32,
            0xc0478000_00000000,
            0xc897e177_43fa06e3,
            0,
        ),
        (
            0xbff24307_67ced4a8,
            0x40460000_00000000,
            0x4075041d_83be0078,
            0,
        ),
        (
            0xbfe696d9_46448813,
            0xc0514000_00000000,
            0xc2196c2f_097fd899,
            0,
        ),
        (
            0xbfb3a43b_05d7ba3f,
            0x40468000_00000000,
            0xb583deed_8b37d4fc,
            0,
        ),
        (
            0xc03151b9_b7335d6f,
            0x40310000_00000000,
            0xc44ec38a_704d338b,
            0,
        ),
        (
            0xbfb3ee3f_dd727b78,
            0x404d0000_00000000,
            0x3294d6d5_e41bc85d,
            0,
        ),
        (
            0xc007a14d_391576cf,
            0xc0518000_00000000,
            0x39189cb4_cd404594,
            0,
        ),
        (
            0xc021b6cf_e4a7d7a9,
            0x404a8000_00000000,
            0xca5b8250_59df7ebc,
            0,
        ),
        (
            0xbffdabb9_0b7555a6,
            0x40568000_00000000,
            0x44f23848_21dfa5f8,
            0,
        ),
        (
            0x4c880433_4225aaef,
            0x3fee2794_7d0a5b00,
            0x4bcf2466_c44dfd52,
            0,
        ),
        (
            0x2f62a83b_75f10476,
            0xbfc01bfc_4d2b4400,
            0x42040552_9bb43040,
            0,
        ),
        (
            0x1ef07b96_2befbada,
            0x3ff563d1_aac44180,
            0x13d240b1_50a2aacb,
            0,
        ),
        (
            0x17c86f34_974c272c,
            0xbff43e8b_2e55d480,
            0x72bbc4bd_ed7d1906,
            0,
        ),
        (
            0x7c205d11_8af0c1c8,
            0xbfed2426_cc175500,
            0x092005a2_36a1b3e7,
            0,
        ),
        (
            0x0714081d_c7323337,
            0xbfe47c6f_e8ea8100,
            0x64549caf_46bbf06c,
            0,
        ),
        (
            0x3f4ff1ca_191bc866,
            0x3ffee267_27556980,
            0x3eb9db1e_76a58f6b,
            0,
        ),
        (
            0x17de5721_d59cc638,
            0xbfd1f4f1_e30eb200,
            0x4b2d3f9f_3f7a1b98,
            0,
        ),
        (
            0x47b5953e_bfcb04cf,
            0x3fe4dd79_fc4efb00,
            0x45018ea5_1aaee556,
            0,
        ),
        (
            0x606e4cf0_aaabea98,
            0xbfeb712b_90023100,
            0x24118e3d_4c7aae05,
            0,
        ),
        (
            0x67996402_2546cfbc,
            0xbff39912_a6556e80,
            0x0f5883ab_bc53bafb,
            0,
        ),
        (
            0x3debc1b8_ad8e42dc,
            0x3ff8724c_a1127b80,
            0x3cdbbb48_498eca4f,
            0,
        ),
        (
            0x3ff00000_4ce4b6ce,
            0x41a3e7d7_3b83f4f2,
            0x44401247_f7db4ad6,
            0,
        ),
        (
            0x3ff00000_2da60e60,
            0xc1b0f759_cb33faaf,
            0x3b91f333_eb616339,
            0,
        ),
        (
            0x3fefffff_fd334b7b,
            0x41715e63_3e1ff9a2,
            0x3fed19c9_acfbd243,
            0,
        ),
        (
            0x3ff00000_3bb0e53a,
            0x41b631c6_732e8898,
            0x4765f20a_3eb51a95,
            0,
        ),
        (
            0x3fefffff_45c62986,
            0xc13da15b_7aac8813,
            0x3fff613a_82d1a38d,
            0,
        ),
        (
            0x3fefffff_890cf27b,
            0xc1b4c9df_6f885fb3,
            0x46e65fa6_158593b5,
            0,
        ),
        (
            0x3ff00000_58cfd518,
            0xc17afa61_81eaaeba,
            0x3f16965e_754adbe2,
            0,
        ),
        (
            0x3fefffff_ce907c8c,
            0x41b4d134_3272b6b2,
            0x3d084ebf_124aa719,
            0,
        ),
        (
            0x3fefffff_3481f9c1,
            0xc1a99e94_ed8a7c1d,
            0x4746f256_d19220ee,
            0,
        ),
        (
            0x3ff00000_148011b2,
            0xc1b90dfd_7d49db78,
            0x3d09c282_9b28091a,
            0,
        ),
        (
            0x3fefffff_3bea6a75,
            0xc1b2bc12_039929ef,
            0x4a489786_c39c40c8,
            0,
        ),
        (
            0x3fefffff_9b4c44e0,
            0x41ab8924_88f0a224,
            0x3c06842d_2773ab12,
            0,
        ),
    ];

    /// Recorded from the exe: `pow` on its x87 path under 0x007f (24-bit precision).
    const POW_X87_RECORDED_24: &[Vector] = &[
        (
            0x7ff40000_00000000,
            0x43400000_00000000,
            0x7ffc0000_00000000,
            0x7_001d,
        ),
        (
            0x40000000_00000000,
            0x7fefffff_ffffffff,
            0x7ff00000_00000000,
            0x3_001d,
        ),
        (
            0x3ff00000_00000001,
            0x3ff00000_00000000,
            0x3ff00000_00000000,
            0,
        ),
        (
            0x00080000_00000000,
            0x7ff40000_00000000,
            0x7ffc0000_00000000,
            0x7_001d,
        ),
        (
            0x43d47be9_f0d7a74e,
            0xc11507c1_0289d7b5,
            0x00000000_00000000,
            0x4_001d,
        ),
        (
            0x7ff80000_00000123,
            0xb4d09d59_bbc567fa,
            0x7ff80000_00000123,
            0x1_001d,
        ),
        (
            0x3fe24d19_1fdbaeca,
            0xbfdd05cb_e2fceb00,
            0x3ff49d3b_80000000,
            0,
        ),
        (
            0x41047295_fd06cb5f,
            0x40eeb007_5ae854a2,
            0x7ff00000_00000000,
            0x3_001d,
        ),
        (
            0x4080b848_af373c0e,
            0x00000000_00000000,
            0x3ff00000_00000000,
            0,
        ),
        (
            0xf958e307_d640b02b,
            0xc0390000_00000000,
            0x80000000_00000000,
            0x4_001d,
        ),
        (
            0x3f6bfbe0_295a868b,
            0x4879f572_dc7a7839,
            0x00000000_00000000,
            0x4_001d,
        ),
        (
            0x4099d36d_1004c2da,
            0xc0e08dc3_f65c3f66,
            0x00000000_00000000,
            0x4_001d,
        ),
        (
            0x4032a67f_2cf5931e,
            0xc045de6d_c66c7c58,
            0x3464c822_20000000,
            0,
        ),
        (
            0x401203f7_85558066,
            0x4046045e_481fd978,
            0x45e857bd_40000000,
            0,
        ),
        (
            0x40331f47_eb380e29,
            0x4027a236_4f8ba120,
            0x4313c8c8_20000000,
            0,
        ),
        (
            0x3ff697bf_08747475,
            0x403f5f4c_82460030,
            0x40e888e2_60000000,
            0,
        ),
        (
            0x3fe4b938_9da66e3c,
            0x404d7e91_464770e8,
            0x3da04978_60000000,
            0,
        ),
        (
            0x3fe9fe62_6f50171a,
            0xc04095f2_89577548,
            0x408ee08f_e0000000,
            0,
        ),
        (
            0x3fb4b00f_30c2eb8f,
            0xc05772d1_d4d0313c,
            0x553534de_e0000000,
            0,
        ),
        (
            0x4022e4a7_bca172f5,
            0xc0543f09_fc748354,
            0x2f88ae5c_20000000,
            0,
        ),
        (
            0x3ff200e3_7bea1b92,
            0xc05222c6_b11308ec,
            0x3f29282f_00000000,
            0,
        ),
        (
            0x3fd60bdd_b2314887,
            0xc0502e47_e70d9a04,
            0x4626d8ae_20000000,
            0,
        ),
        (
            0x3ffd4989_3cd0fb9b,
            0xc04732a2_0b8b1d38,
            0x3d6729b4_40000000,
            0,
        ),
        (
            0x3fd62a14_eb37d2d0,
            0x4048bef1_a1988298,
            0x3b338268_20000000,
            0,
        ),
        (
            0x3fba5079_62b74590,
            0xc052e899_3300824c,
            0x4f7302b5_e0000000,
            0,
        ),
        (
            0x400be235_339a3c61,
            0x4036596d_eb913a70,
            0x42732516_e0000000,
            0,
        ),
        (
            0x40330cd0_62664c23,
            0xc013e820_fd4a3fc0,
            0x3e9ca694_60000000,
            0,
        ),
        (
            0x3fd67b39_72e878dd,
            0xc058e1f5_39723214,
            0x4952b626_e0000000,
            0,
        ),
        (
            0xc025d866_151760b0,
            0xc03d0000_00000000,
            0xb9af60e7_80000000,
            0,
        ),
        (
            0xbfe5ac89_6a474c2c,
            0x40530000_00000000,
            0x3d436c88_80000000,
            0,
        ),
        (
            0xbfb023b4_9ce6e8de,
            0x40260000_00000000,
            0xbd319a55_c0000000,
            0,
        ),
        (
            0xbfbd5b2b_2a8d65e4,
            0xc0578000_00000000,
            0x5249e4f0_e0000000,
            0,
        ),
        (
            0x16368d7b_4154ca8e,
            0x3ff3433e_82712280,
            0x0db4d0ee_e0000000,
            0,
        ),
        (
            0x5f49d6ea_4d65f7c1,
            0xbffcdeaa_0257b880,
            0x075b3b68_40000000,
            0,
        ),
        (
            0x71749bbe_38134b41,
            0xbff374a1_b8c28380,
            0x03b6b142_c0000000,
            0,
        ),
        (
            0x0677f1a1_a6d6c3b2,
            0x3ffc88ac_509dc180,
            0x39743d5f_20000000,
            0x4_001d,
        ),
        (
            0x3ff00000_57f01cff,
            0x41b5d204_ea072f4d,
            0x4ac035aa_60000000,
            0,
        ),
        (
            0x3feffffe_fb9749d6,
            0x41b55235_e1645bd0,
            0x3049ae41_e0000000,
            0,
        ),
        (
            0x3fefffff_0deaa277,
            0x41b2e4a1_09533ba9,
            0x330bd256_a0000000,
            0,
        ),
        (
            0x3ff00000_4789edb1,
            0x41a037ba_3d98d1d7,
            0x4333c93a_80000000,
            0,
        ),
    ];

    #[test]
    fn pow_x87_matches_the_recorded_results_special_values_included() {
        let (mut e, except, _) = engine(false);
        for (vectors, word) in [(POW_X87_RECORDED, 0x027fu16), (POW_X87_RECORDED_24, 0x007f)] {
            for &(x, y, want, error) in vectors {
                except.borrow_mut().clear();
                set_x87_control_word(&mut e, word);
                let got = pow_of(&mut e, x, y);
                assert_eq!(x87_control_word(&mut e), word);
                set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
                assert_eq!(
                    (got, except_seen(&except)),
                    (want, error),
                    "pow({x:016x}, {y:016x}) under {word:04x}"
                );
            }
        }
    }

    #[test]
    fn pow_x87_errors_report_through_the_two_argument_handler() {
        let (mut e, except, _) = engine(false);
        // A negative base to a non-integer power: the indefinite NaN, type 1, the operation 0x1d.
        let r = e.call(0x00ec_9ac0, &args![-2.0f64, 0.5f64]).f64();
        assert_eq!(r.to_bits(), 0xfff8_0000_0000_0000);
        let seen = except.borrow().clone();
        assert_eq!(seen.len(), 1);
        assert_eq!((seen[0][0], seen[0][3]), (POW_OPERATION, 1));
        // A zero base to a negative power: +infinity, type 2; negative for -0 to an odd power.
        assert_eq!(
            e.call(0x00ec_9ac0, &args![-0.0f64, -3.0f64]).f64(),
            f64::NEG_INFINITY
        );
        assert_eq!(except.borrow().last().unwrap()[3], 2);
        assert_eq!(
            e.call(0x00ec_9ac0, &args![-0.0f64, -2.0f64]).f64(),
            f64::INFINITY
        );
        // The math-error switch set: no report, the result as it is (the overflow too: the
        // extended power rounds to infinity as a double).
        e.set_global(ERROR_SWITCH, 1u32);
        except.borrow_mut().clear();
        assert!(e.call(0x00ec_9ac0, &args![-2.0f64, 0.5f64]).f64().is_nan());
        assert_eq!(
            e.call(0x00ec_9ac0, &args![10.0f64, 400.0f64]).f64(),
            f64::INFINITY
        );
        assert!(except.borrow().is_empty());
    }

    /// `00ed955d` as the registered double words give it.
    fn special_entry(e: &mut Engine, a: &[u32]) -> Ret {
        let x = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
        let y = f64::from_bits(u64::from(a[2]) | (u64::from(a[3]) << 32));
        fn_00ed955d(e, x, y, Ptr::new(a[4])).into_ret()
    }

    #[test]
    fn pow_x87_infinities_go_through_the_special_value_helper_under_fninit() {
        let (mut e, except, _) = engine(false);
        let seen: Seen = Rc::default();
        let record = seen.clone();
        e.register_double(POW_SPECIAL, move |e, a| {
            let mut w = a[..4].to_vec();
            w.push(u32::from(x87_control_word(e)));
            record.borrow_mut().push(w);
            special_entry(e, a)
        });
        set_x87_control_word(&mut e, 0x007f);
        assert_eq!(
            e.call(0x00ec_9ac0, &args![2.0f64, f64::INFINITY]).f64(),
            f64::INFINITY
        );
        assert_eq!(x87_control_word(&mut e), 0x007f);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
        // The helper ran on the two doubles with the control word FNINIT leaves.
        assert_eq!(
            seen.borrow()[0],
            vec![0, 0x4000_0000, 0, 0x7ff0_0000, 0x037f]
        );
        let cases: [(f64, f64, f64); 9] = [
            (0.5, f64::INFINITY, 0.0),
            (-1.0, f64::INFINITY, 1.0),
            (3.0, f64::NEG_INFINITY, 0.0),
            (0.25, f64::NEG_INFINITY, f64::INFINITY),
            (f64::INFINITY, 2.0, f64::INFINITY),
            (f64::INFINITY, -2.0, 0.0),
            (f64::INFINITY, 0.0, 1.0),
            (f64::NEG_INFINITY, 3.0, f64::NEG_INFINITY),
            (f64::NEG_INFINITY, -3.0, -0.0),
        ];
        for (x, y, want) in cases {
            let got = e.call(0x00ec_9ac0, &args![x, y]).f64();
            assert_eq!(got.to_bits(), want.to_bits(), "pow({x}, {y})");
        }
        assert!(except.borrow().is_empty());
        // One to -infinity: the helper answers 1, a domain error with its NaN.
        let got = e.call(0x00ec_9ac0, &args![1.0f64, f64::NEG_INFINITY]).f64();
        assert_eq!(got.to_bits(), 0xfff8_0000_0000_0000);
        assert_eq!(except.borrow().last().unwrap()[3], 1);
    }

    #[test]
    fn the_special_value_helper_and_the_integer_tests() {
        let (mut e, _, _) = engine(false);
        let special = |e: &mut Engine, x: f64, y: f64| {
            e.with_stack(8, |e, slot| {
                e.mem.set_u64(slot.addr(), 0x1234_5678_9abc_def0);
                let answer = fn_00ed955d(e, x, y, slot);
                (answer, e.mem.u64(slot.addr()))
            })
        };
        assert_eq!(
            special(&mut e, 2.0, f64::INFINITY),
            (0, f64::INFINITY.to_bits())
        );
        assert_eq!(special(&mut e, -0.5, f64::INFINITY), (0, 0));
        assert_eq!(
            special(&mut e, -1.0, f64::NEG_INFINITY),
            (1, 0xfff8_0000_0000_0000)
        );
        assert_eq!(
            special(&mut e, f64::NEG_INFINITY, 5.0),
            (0, f64::NEG_INFINITY.to_bits())
        );
        assert_eq!(special(&mut e, f64::NEG_INFINITY, -4.0), (0, 0));
        assert_eq!(
            special(&mut e, f64::NEG_INFINITY, 0.0),
            (0, 1.0f64.to_bits())
        );
        // Neither argument infinite: nothing stored.
        assert_eq!(special(&mut e, 2.0, 3.0), (0, 0x1234_5678_9abc_def0));
        assert_eq!(
            special(&mut e, f64::from_bits(0x7ff0_0000_0000_0001), 3.0),
            (0, 0x1234_5678_9abc_def0)
        );
        // __d_inttype: 0 for a fraction or a denormal, 1 odd, 2 even.
        for (v, kind) in [
            (3.0, 1),
            (4.0, 2),
            (0.5, 0),
            (-7.0, 1),
            (0.0, 2),
            (f64::from_bits(1), 0),
        ] {
            assert_eq!(e.call(D_INTTYPE, &args![v]).u32(), kind, "{v}");
        }
        for (bits, class) in [
            (0x7ff0_0000_0000_0000u64, 0x200),
            (0xfff0_0000_0000_0000, 4),
            (0x7ff8_0000_0000_0000, 2),
            // A signalling NaN is quiet in the copy `__sptype` sees.
            (0x7ff0_0000_0000_0001, 2),
            (0x8000_0000_0000_0001, 0x10),
            (0x0000_0000_0000_0001, 0x80),
            (0x8000_0000_0000_0000, 0x20),
            (0, 0x40),
            ((-2.5f64).to_bits(), 8),
            (2.5f64.to_bits(), 0x100),
        ] {
            assert_eq!(e.call(FPCLASS, &args![bits]).u32(), class, "{bits:016x}");
        }
        // 00ec9d22: 0 for a fraction, 1 odd, 2 even.
        for (v, cl) in [(2.5, 0), (3.0, 1), (-6.0, 2), (0.0, 2)] {
            assert_eq!(e.call(0x00ec_9d22, &args![v]).u32(), cl, "{v}");
        }
    }

    #[test]
    fn the_exits_restore_the_control_word_and_check_the_range() {
        let (mut e, except, _) = engine(false);
        // The fast exit: 0x027f stays, any other word is loaded back.
        assert_eq!(e.call(0x00ed_281e, &args![2.5f64, 0x027fu32]).f64(), 2.5);
        set_x87_control_word(&mut e, 0x007f);
        assert_eq!(e.call(0x00ed_281e, &args![2.5f64, 0x0e7fu32]).f64(), 2.5);
        assert_eq!(x87_control_word(&mut e), 0x0e7f);
        // The range exit: a result in range under 0x027f comes back as it is.
        set_x87_control_word(&mut e, 0x027f);
        let range = |e: &mut Engine, operation: u32, st0: f64, saved: u32| {
            e.call(
                0x00ed_2869,
                &args![POW_NAME, operation, st0, saved, 0u32, 3.0f64, 4.0f64],
            )
            .f64()
        };
        assert_eq!(range(&mut e, 0xe, f64::MAX, 0x027f), f64::MAX);
        // The precision exception masked in the saved word: the word back, no report.
        assert_eq!(range(&mut e, 0xe, 1.5, 0x007f), 1.5);
        assert_eq!(x87_control_word(&mut e), 0x007f);
        set_x87_control_word(&mut e, 0x027f);
        assert!(except.borrow().is_empty());
        // Unmasked: the precision flag (taken as set) reports type 8.
        assert_eq!(range(&mut e, 0xe, 1.5, 0x025f), 1.5);
        assert_eq!(except.borrow().last().unwrap()[3], 8);
        set_x87_control_word(&mut e, 0x027f);
        // A denormal: scaled by 2^1536, which reaches the bound, so it is not multiplied: type 4
        // through the one-argument handler (operation 0xe).
        except.borrow_mut().clear();
        assert_eq!(
            range(&mut e, 0xe, f64::from_bits(1), 0x027f),
            2f64.powi(1536 - 1074)
        );
        let seen = except.borrow().clone();
        assert_eq!((seen[0][0], seen[0][3]), (0xe, 4));
        // A zero: scaled, still below the bound, multiplied by 0: type 4 with +0, through the
        // two-argument handler for pow's operation.
        assert_eq!(range(&mut e, POW_OPERATION, 0.0, 0x027f), 0.0);
        assert_eq!(except.borrow().last().unwrap()[0], POW_OPERATION);
        // An infinity: scaled by 2^-1536, still above the bound, multiplied by +infinity: type 3.
        assert_eq!(
            range(&mut e, POW_OPERATION, f64::INFINITY, 0x027f),
            f64::INFINITY
        );
        assert_eq!(except.borrow().last().unwrap()[3], 3);
        // 00ed2795 loads the caller's precision with every exception masked, round to nearest.
        e.call(0x00ed_2795, &args![0x0e3eu32]);
        assert_eq!(x87_control_word(&mut e), 0x027f);
        e.call(0x00ed_2795, &args![0x0c7fu32]);
        assert_eq!(x87_control_word(&mut e), 0x007f);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
        // 00ed2780: 2^x.
        assert_eq!(e.call(0x00ed_2780, &args![10.0f64]).f64(), 1024.0);
        assert_eq!(e.call(0x00ed_2780, &args![-1.0f64]).f64(), 0.5);
    }

    #[test]
    fn the_two_argument_error_handler_builds_the_exception_and_reloads_the_word() {
        let (mut e, _, _) = engine(false);
        let fields: Seen = Rc::default();
        let record = fields.clone();
        e.register_double(EXCEPT87, move |e, a| {
            let at = a[1];
            let mut w = vec![a[0]];
            for i in 0..8 {
                w.push(e.mem.u32(at + 4 * i));
            }
            w.push(u32::from(e.mem.u16(a[2])));
            record.borrow_mut().push(w);
            // __87except may change the result.
            e.mem.set_u64(at + 0x18, 7.0f64.to_bits());
            Ret::default()
        });
        set_x87_control_word(&mut e, 0x007f);
        let r = e
            .call(
                0x00ed_2720,
                &args![
                    2u32,
                    POW_NAME,
                    POW_OPERATION,
                    3.5f64,
                    0x0a7fu32,
                    0u32,
                    1.5f64,
                    -2.0f64
                ],
            )
            .f64();
        assert_eq!(r, 7.0);
        assert_eq!(x87_control_word(&mut e), 0x0a7f);
        let (one, two, res) = (1.5f64.to_bits(), (-2.0f64).to_bits(), 3.5f64.to_bits());
        assert_eq!(
            fields.borrow()[0],
            vec![
                POW_OPERATION,
                2,
                POW_NAME,
                one as u32,
                (one >> 32) as u32,
                two as u32,
                (two >> 32) as u32,
                res as u32,
                (res >> 32) as u32,
                0x0a7f
            ]
        );
        // 0x027f is not loaded again.
        set_x87_control_word(&mut e, 0x007f);
        let r = e
            .call(
                0x00ed_2720,
                &args![
                    1u32,
                    POW_NAME,
                    POW_OPERATION,
                    3.5f64,
                    0x027fu32,
                    0u32,
                    1.5f64,
                    2.0f64
                ],
            )
            .f64();
        assert_eq!(r, 7.0);
        assert_eq!(x87_control_word(&mut e), 0x007f);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    /// The SSE2 constants `pow`'s special paths read, at the exe's values, and the first tables
    /// mapped (zero but for the first entry: the special paths read entries before they branch
    /// and use only that one).
    fn pow_sse2_engine() -> (Engine, Seen) {
        let (mut e, _, libm) = engine(true);
        e.map(POW_TABLE_FIRST, 0x1000);
        e.map(POW_TABLE_FIRST_PAIRS, 0x0800);
        for at in [POW_FRACTION_MASK, POW_FRACTION_MASK + 8] {
            put(&mut e, at, 0x000f_ffff_ffff_ffff);
        }
        for at in [POW_EXPONENT_BITS, POW_EXPONENT_BITS + 8] {
            put(&mut e, at, 1.0f64.to_bits());
        }
        for at in [POW_MAGNITUDE_MASK, POW_MAGNITUDE_MASK + 8] {
            put(&mut e, at, 0x7fff_ffff_ffff_ffff);
        }
        put(&mut e, POW_INVALID_OPERAND, 0x7ff0_0000_0000_0001);
        put(&mut e, POW_RESULT_8530, f64::INFINITY.to_bits());
        put(&mut e, POW_RESULT_8538, f64::NEG_INFINITY.to_bits());
        put(&mut e, POW_RESULT_8540, (-0.0f64).to_bits());
        put(&mut e, POW_INVALID_RESULT, 0xfff8_0000_0000_0000);
        e.mem.set_u64(POW_TABLE_FIRST, 1.0f64.to_bits());
        (e, libm)
    }

    #[test]
    fn pow_sse2_special_values_without_the_tables() {
        let (mut e, libm) = pow_sse2_engine();
        let code = |libm: &Seen| libm.borrow().last().map_or(0, |w| w[3]);
        let one = 1.0f64.to_bits();
        // (A positive normal base takes the main path, which reads the tables.)
        let cases: [(u64, u64, u64, u32); 13] = [
            // A zero exponent: 1.0, error 0x1a for a zero base, 0x1d for a NaN one.
            (0, 0x8000_0000_0000_0000, one, 0x1a),
            (0x7ff8_0000_0000_0001, 0, one, 0x1d),
            (f64::INFINITY.to_bits(), 0, one, 0),
            // A NaN exponent or base: 0x3ee with the quiet NaN.
            (
                (-2.0f64).to_bits(),
                0x7ff0_0000_0000_0003,
                0x7ff8_0000_0000_0003,
                0x3ee,
            ),
            (
                0xfff0_0000_0000_0002,
                2.0f64.to_bits(),
                0xfff8_0000_0000_0002,
                0x3ee,
            ),
            // Zero bases.
            (0, 3.0f64.to_bits(), 0, 0),
            (
                0x8000_0000_0000_0000,
                3.0f64.to_bits(),
                0x8000_0000_0000_0000,
                0,
            ),
            (0x8000_0000_0000_0000, 2.0f64.to_bits(), 0, 0),
            (0, (-3.0f64).to_bits(), f64::INFINITY.to_bits(), 0x1b),
            (
                0x8000_0000_0000_0000,
                (-3.0f64).to_bits(),
                f64::NEG_INFINITY.to_bits(),
                0x1b,
            ),
            // A negative base to a fraction.
            (
                (-2.0f64).to_bits(),
                0.5f64.to_bits(),
                0xfff8_0000_0000_0000,
                0x1c,
            ),
            // Infinite arguments.
            (
                f64::NEG_INFINITY.to_bits(),
                3.0f64.to_bits(),
                f64::NEG_INFINITY.to_bits(),
                0,
            ),
            (
                f64::NEG_INFINITY.to_bits(),
                (-3.0f64).to_bits(),
                0x8000_0000_0000_0000,
                0,
            ),
        ];
        for (x, y, want, error) in cases {
            libm.borrow_mut().clear();
            let got = pow_of(&mut e, x, y);
            assert_eq!((got, code(&libm)), (want, error), "pow({x:016x}, {y:016x})");
        }
        // The error support saw the argument slots and the result slot.
        libm.borrow_mut().clear();
        pow_of(&mut e, 0, 0);
        let w = libm.borrow()[0].clone();
        assert_eq!((w[1], w[2]), (w[0] + 8, w[0] + 0x10));
        // Minus one to an infinite power is one; no x87 path ran.
        assert_eq!(
            pow_of(&mut e, (-1.0f64).to_bits(), f64::INFINITY.to_bits()),
            one
        );
        assert!(calls(&e, FLOAD_WITH_FB).is_empty());
    }

    #[test]
    fn pow_picks_its_body_from_the_flag_and_the_masks() {
        let (mut e, _, _) = engine(true);
        let seen: Seen = Rc::default();
        let record = seen.clone();
        e.register_double(POW_SSE2, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            11.0f64.into_ret()
        });
        e.register(POW_SSE2_REGISTERS, |_, _| 12.0f64.into_ret());
        assert_eq!(e.call(0x00ec_9ac0, &args![2.0f64, 3.0f64]).f64(), 11.0);
        assert_eq!(e.call(0x00ec_9b00, &args![2.0f64, 3.0f64]).f64(), 12.0);
        // 24-bit precision keeps every exception masked: still SSE2.
        set_x87_control_word(&mut e, 0x007f);
        assert_eq!(e.call(0x00ec_9ac0, &args![2.0f64, 3.0f64]).f64(), 11.0);
        // One x87 exception unmasked, or the flag clear: the x87 body.
        set_x87_control_word(&mut e, 0x027e);
        assert_eq!(e.call(0x00ec_9ac0, &args![2.0f64, 3.0f64]).f64(), 8.0);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
        e.set_global(SSE2_FLAG, 0u32);
        assert_eq!(e.call(0x00ec_9b00, &args![2.0f64, 3.0f64]).f64(), 8.0);
        // The register form of the SSE2 body stores the registers (a signalling NaN quiet).
        seen.borrow_mut().clear();
        fn_00ed8790(&mut e, f64::from_bits(0x7ff0_0000_0000_0001), 2.0);
        assert_eq!(seen.borrow()[0], args![0x7ff8_0000_0000_0001u64, 2.0f64]);
    }

    /// Recorded from the exe: `pow` on its SSE2 path (the vector's error is
    /// `___libm_error_support`'s code).
    const POW_SSE2_RECORDED: &[Vector] = &[
        (
            0x7ff00000_00000000,
            0x7fefffff_ffffffff,
            0x7ff00000_00000000,
            0,
        ),
        (
            0xfff80000_00000000,
            0x7ff40000_00000000,
            0xfff80000_00000000,
            0x3ee,
        ),
        (
            0x7ff40000_00000000,
            0x43300000_00000001,
            0x7ffc0000_00000000,
            0x3ee,
        ),
        (
            0x00000000_00000001,
            0x40000000_00000000,
            0x00000000_00000000,
            0x19,
        ),
        (
            0x3ff00000_00000000,
            0xfff00000_00000000,
            0x3ff00000_00000000,
            0x1c,
        ),
        (
            0x40000000_00000000,
            0x3ff00000_00000001,
            0x40000000_00000001,
            0,
        ),
        (
            0xbfe00000_00000000,
            0x800fffff_ffffffff,
            0xfff80000_00000000,
            0x1c,
        ),
        (
            0xffefffff_ffffffff,
            0x00080000_00000000,
            0xfff80000_00000000,
            0x1c,
        ),
        (
            0x3ff00000_00000001,
            0x7fefffff_ffffffff,
            0x7ff00000_00000000,
            0x18,
        ),
        (
            0xc3400000_00000000,
            0x7ff40000_00000000,
            0x7ffc0000_00000000,
            0x3ee,
        ),
        (
            0xc08ff000_00000000,
            0x43300000_00000001,
            0xfff00000_00000000,
            0x18,
        ),
        (
            0x00080000_00000000,
            0x40000000_00000000,
            0x00000000_00000000,
            0x19,
        ),
        (
            0xbe2b224d_dcb5aef4,
            0x3ec88018_5da6ac0f,
            0xfff80000_00000000,
            0x1c,
        ),
        (
            0x3fdbb172_089be200,
            0x3ec56308_d14e8e56,
            0x3feffffb_85650af3,
            0,
        ),
        (
            0xbfe16551_2df98680,
            0x40380000_00000000,
            0x3e9dc5fc_41ca29b1,
            0,
        ),
        (
            0xbf0e1345_cd757865,
            0xbff5ac21_b3e0ee80,
            0xfff80000_00000000,
            0x1c,
        ),
        (
            0xc1162db1_7ad92243,
            0xbff97655_1caf6640,
            0xfff80000_00000000,
            0x1c,
        ),
        (
            0xc03b0000_00000000,
            0x41140b51_6d146856,
            0xfff80000_00000000,
            0x1c,
        ),
        (
            0x3ebc188b_aaa4f05d,
            0x7ff00000_00000000,
            0x00000000_00000000,
            0,
        ),
        (
            0x40100000_00000000,
            0xc0101ae0_0cce8bcb,
            0x3f6edb4d_333644e1,
            0,
        ),
        (
            0x3cb00000_00000000,
            0xc1179175_4660d32b,
            0x7ff00000_00000000,
            0x18,
        ),
        (
            0x935748ea_ab858392,
            0x43400000_00000000,
            0x00000000_00000000,
            0x19,
        ),
        (
            0x4048c539_08ae354b,
            0xbfe00000_00000000,
            0x3fc22f85_b7545978,
            0,
        ),
        (
            0xc0240000_00000000,
            0x7ff80000_00000000,
            0x7ff80000_00000000,
            0x3ee,
        ),
        (
            0x3fd38684_68436400,
            0xc0330000_00000000,
            0x41f74a0f_24aff80e,
            0,
        ),
        (
            0xc1a741b0_15057da1,
            0x43400000_00000000,
            0x7ff00000_00000000,
            0x18,
        ),
        (
            0x401c0000_00000000,
            0xc03d0000_00000000,
            0x3ad8076f_555a58e8,
            0,
        ),
        (
            0x40240000_00000000,
            0x8cd9bfec_09779247,
            0x3ff00000_00000000,
            0,
        ),
        (
            0xc0dd42dc_e38ad6fd,
            0x401c0000_00000000,
            0xc6711b15_743e1ca8,
            0,
        ),
        (
            0x3fefffff_ffffffff,
            0x93bb5832_cd0b6033,
            0x3ff00000_00000000,
            0,
        ),
        (
            0x3e81795b_b6ff04da,
            0xc1196571_5e8ccb3d,
            0x7ff00000_00000000,
            0x18,
        ),
        (
            0x6f0aa47a_2a391956,
            0x3ff46bfb_b4d7b240,
            0x7c10743f_36a37499,
            0,
        ),
        (
            0x3ff68c22_9e64dd7c,
            0x3e389c7c_49ef9c8f,
            0x3ff00000_00871435,
            0,
        ),
        (
            0xbff00000_00000000,
            0xffefffff_ffffffff,
            0x3ff00000_00000000,
            0,
        ),
        (
            0x3f83f645_0bde2000,
            0x3e7ed5ff_33fe3be6,
            0x3feffffe_e26926ed,
            0,
        ),
        (
            0x3ec63652_cc49ede1,
            0x40360000_00000000,
            0x26754a7d_031074bc,
            0,
        ),
        (
            0x400a48ca_de03a9b3,
            0x40539427_60d1719c,
            0x485517da_8466a96f,
            0,
        ),
        (
            0x4007cfc8_d467039f,
            0x40542fee_3dab466c,
            0x47e0c049_c9e1f051,
            0,
        ),
        (
            0x3fd28804_bfc0d4d3,
            0xc04a0ab9_1129a588,
            0x45c1864c_79a16a81,
            0,
        ),
        (
            0x402dd740_06aea343,
            0x4047dc56_51accc18,
            0x4b90dd6a_3ae33a95,
            0,
        ),
        (
            0x401980fd_6ce2236b,
            0xc03b509b_e7173c90,
            0x3b5fef44_27f98cff,
            0,
        ),
        (
            0x40222911_5d8629c3,
            0xc03c2f66_1650c950,
            0x3a539f34_61a58e28,
            0,
        ),
        (
            0x4003b668_e4a368fc,
            0xc049dd48_386f0708,
            0x3bb9f813_66cb7fa7,
            0,
        ),
        (
            0x40228c6f_b2d2f49b,
            0xc040927e_1d950568,
            0x39469c6f_4494179e,
            0,
        ),
        (
            0x3fff0b4e_0cfe2e56,
            0x4041ecd0_3e402038,
            0x42137313_4e13c038,
            0,
        ),
        (
            0x40260c22_aa141f76,
            0x4053e029_547d74ec,
            0x51236ec7_8cc72601,
            0,
        ),
        (
            0x3fd30032_4a36484b,
            0xc058e320_60351444,
            0x4ad54614_be4cb867,
            0,
        ),
        (
            0x40312e48_7655e4a7,
            0xc04b2d03_d7e496e8,
            0x32001c8b_2beed351,
            0,
        ),
        (
            0x3fb35f6e_738ee05d,
            0x40416c1f_8ba61eb8,
            0x37d2dad6_0ee0c652,
            0,
        ),
        (
            0x3ffbd1ab_08ead764,
            0x4055dd36_e8303c2c,
            0x444bab62_82a565cb,
            0,
        ),
        (
            0x40280566_7e68fd7a,
            0x4056e93f_09c27afc,
            0x54793fc3_b433e993,
            0,
        ),
        (
            0x3fac20d0_eab70586,
            0x4058ac56_71af0bcc,
            0x261d2c64_2625a901,
            0,
        ),
        (
            0x403030c2_a79254d8,
            0x40581202_9ca32e9c,
            0x581b461d_1c7641f4,
            0,
        ),
        (
            0x40129120_fce685a9,
            0x4030018f_67508db0,
            0x4225d3e1_ccdabd42,
            0,
        ),
        (
            0x3ffbe171_62568ecb,
            0x404fc0b8_0ffe5478,
            0x431d730b_7e007e05,
            0,
        ),
        (
            0x401b47de_28d55830,
            0x4037d0b1_c9a60c30,
            0x440f340c_79a22301,
            0,
        ),
        (
            0x3fd0f0c7_216db4c4,
            0xc023db09_90fc9120,
            0x41206bd2_47d74b2a,
            0,
        ),
        (
            0x3ff65513_876a7398,
            0x40569d4d_55b92aac,
            0x42a6e078_d42ee66c,
            0,
        ),
        (
            0x4007ce4b_c4dba72f,
            0x4047223c_11a1f2f8,
            0x447baa42_8962d987,
            0,
        ),
        (
            0x3fbc8d79_691d3eb3,
            0xc030bdeb_e5cb96d0,
            0x433f8891_e2e72a46,
            0,
        ),
        (
            0x4000a679_bc1b6c0e,
            0x40437744_17b59a38,
            0x4281fe53_d69e78dd,
            0,
        ),
        (
            0x3faac7a5_b0e7d558,
            0xc043ea65_dfa15c28,
            0x4a879049_9f705161,
            0,
        ),
        (
            0x4026502c_c3dd6247,
            0x405128a6_0287e8bc,
            0x4edc9af4_ed5be1e5,
            0,
        ),
        (
            0x4025a269_d3724c1f,
            0x4055693b_a669d18c,
            0x52528202_8542edc9,
            0,
        ),
        (
            0x3fc11178_e871bc11,
            0xc0472b44_de3b6748,
            0x4859df9e_84f3e47e,
            0,
        ),
        (
            0x3fbb43a3_0fd68a9d,
            0xc03f994a_e9e59b50,
            0x46511f18_3106fbb8,
            0,
        ),
        (
            0x3fc8ed17_e585840c,
            0x4045bb04_e6e7eff8,
            0x3985511f_c8de8c97,
            0,
        ),
        (
            0x3fe014fd_0e35df35,
            0xc03f4726_b6c15cd0,
            0x41e08884_748ef2ef,
            0,
        ),
        (
            0x3ffb4409_4a89d866,
            0x40578dea_1f46eb9c,
            0x4475ee97_632508a4,
            0,
        ),
        (
            0x3fc6443c_b89cb5c4,
            0xc03b0a8d_17c3fe50,
            0x4432c4af_4af48be1,
            0,
        ),
        (
            0x3fb005b4_2cbce5c3,
            0xc051b31c_9e1ad8c4,
            0x51a0975c_b0da5226,
            0,
        ),
        (
            0x4013d9aa_8e0a78a8,
            0x4054b919_928ea00c,
            0x4be7c4aa_6ef10faa,
            0,
        ),
        (
            0x3ff86b4a_3ea591e7,
            0x404c46b8_02b155b8,
            0x421686ae_4525a0d4,
            0,
        ),
        (
            0x401401b0_37f37474,
            0x40319a8a_e8621eb0,
            0x427d811f_29280eac,
            0,
        ),
        (
            0x3fb08cf1_d9a07550,
            0x4050c9ce_5c2b767c,
            0x2f595d77_9ecdd015,
            0,
        ),
        (
            0x3ff46da7_7d929490,
            0x4052e0fc_7776b74c,
            0x4198936c_8dfac2a2,
            0,
        ),
        (
            0x3fcfcf3f_b3600f2b,
            0x40554ff7_09678a1c,
            0x353b3a3c_3c6b60d1,
            0,
        ),
        (
            0x400001e8_8c3cf070,
            0xc046df59_cc19a228,
            0x3d12b117_5afb7940,
            0,
        ),
        (
            0x3feb332e_872bda98,
            0xc0161ad8_7691a440,
            0x4003a3da_3ee3ee22,
            0,
        ),
        (
            0x3fde1d15_06478249,
            0xc037ff8d_9aa045d0,
            0x41912ba5_fcf52994,
            0,
        ),
        (
            0x3fd84fb9_4f9d276a,
            0x4027db14_8e1c4ae0,
            0x3ee44ced_b459f0e1,
            0,
        ),
        (
            0x3ff9586b_16b4f256,
            0xc05501fd_ca1109d4,
            0x3c72ccc9_1a3d1e37,
            0,
        ),
        (
            0x3fafdace_a7670d1f,
            0x40407d78_aaeee9f8,
            0x37ad1753_62602782,
            0,
        ),
        (
            0xc01e17c3_9ee7306b,
            0x40544000_00000000,
            0xceac3af5_fa9e1f40,
            0,
        ),
        (
            0xc030f8cd_2ea143fe,
            0x40330000_00000000,
            0xc4c8876c_71ff920e,
            0,
        ),
        (
            0xbfdbc901_da6c1b03,
            0xc04e0000_00000000,
            0x4472b485_e259c88b,
            0,
        ),
        (
            0xbfc094f6_dc8caf32,
            0xc0478000_00000000,
            0xc897e177_43fa06e3,
            0,
        ),
        (
            0xbff24307_67ced4a8,
            0x40460000_00000000,
            0x4075041d_83be0078,
            0,
        ),
        (
            0xbfe696d9_46448813,
            0xc0514000_00000000,
            0xc2196c2f_097fd899,
            0,
        ),
        (
            0xbfb3a43b_05d7ba3f,
            0x40468000_00000000,
            0xb583deed_8b37d4fc,
            0,
        ),
        (
            0xc03151b9_b7335d6f,
            0x40310000_00000000,
            0xc44ec38a_704d338b,
            0,
        ),
        (
            0xbfb3ee3f_dd727b78,
            0x404d0000_00000000,
            0x3294d6d5_e41bc85d,
            0,
        ),
        (
            0xc007a14d_391576cf,
            0xc0518000_00000000,
            0x39189cb4_cd404594,
            0,
        ),
        (
            0xc021b6cf_e4a7d7a9,
            0x404a8000_00000000,
            0xca5b8250_59df7ebc,
            0,
        ),
        (
            0xbffdabb9_0b7555a6,
            0x40568000_00000000,
            0x44f23848_21dfa5f8,
            0,
        ),
        (
            0x4c880433_4225aaef,
            0x3fee2794_7d0a5b00,
            0x4bcf2466_c44dfd52,
            0,
        ),
        (
            0x2f62a83b_75f10476,
            0xbfc01bfc_4d2b4400,
            0x42040552_9bb43040,
            0,
        ),
        (
            0x1ef07b96_2befbada,
            0x3ff563d1_aac44180,
            0x13d240b1_50a2aacb,
            0,
        ),
        (
            0x17c86f34_974c272c,
            0xbff43e8b_2e55d480,
            0x72bbc4bd_ed7d1906,
            0,
        ),
        (
            0x7c205d11_8af0c1c8,
            0xbfed2426_cc175500,
            0x092005a2_36a1b3e7,
            0,
        ),
        (
            0x0714081d_c7323337,
            0xbfe47c6f_e8ea8100,
            0x64549caf_46bbf06c,
            0,
        ),
        (
            0x3f4ff1ca_191bc866,
            0x3ffee267_27556980,
            0x3eb9db1e_76a58f6b,
            0,
        ),
        (
            0x17de5721_d59cc638,
            0xbfd1f4f1_e30eb200,
            0x4b2d3f9f_3f7a1b98,
            0,
        ),
        (
            0x47b5953e_bfcb04cf,
            0x3fe4dd79_fc4efb00,
            0x45018ea5_1aaee556,
            0,
        ),
        (
            0x606e4cf0_aaabea98,
            0xbfeb712b_90023100,
            0x24118e3d_4c7aae05,
            0,
        ),
        (
            0x67996402_2546cfbc,
            0xbff39912_a6556e80,
            0x0f5883ab_bc53bafb,
            0,
        ),
        (
            0x3debc1b8_ad8e42dc,
            0x3ff8724c_a1127b80,
            0x3cdbbb48_498eca4f,
            0,
        ),
        (
            0x3ff00000_4ce4b6ce,
            0x41a3e7d7_3b83f4f2,
            0x44401247_f7db4ad6,
            0,
        ),
        (
            0x3ff00000_2da60e60,
            0xc1b0f759_cb33faaf,
            0x3b91f333_eb616339,
            0,
        ),
        (
            0x3fefffff_fd334b7b,
            0x41715e63_3e1ff9a2,
            0x3fed19c9_acfbd244,
            0,
        ),
        (
            0x3ff00000_3bb0e53a,
            0x41b631c6_732e8898,
            0x4765f20a_3eb51a95,
            0,
        ),
        (
            0x3fefffff_45c62986,
            0xc13da15b_7aac8813,
            0x3fff613a_82d1a38d,
            0,
        ),
        (
            0x3fefffff_890cf27b,
            0xc1b4c9df_6f885fb3,
            0x46e65fa6_158593b5,
            0,
        ),
        (
            0x3ff00000_58cfd518,
            0xc17afa61_81eaaeba,
            0x3f16965e_754adbe2,
            0,
        ),
        (
            0x3fefffff_ce907c8c,
            0x41b4d134_3272b6b2,
            0x3d084ebf_124aa719,
            0,
        ),
        (
            0x3fefffff_3481f9c1,
            0xc1a99e94_ed8a7c1d,
            0x4746f256_d19220ef,
            0,
        ),
        (
            0x3ff00000_148011b2,
            0xc1b90dfd_7d49db78,
            0x3d09c282_9b28091a,
            0,
        ),
        (
            0x3fefffff_3bea6a75,
            0xc1b2bc12_039929ef,
            0x4a489786_c39c40c8,
            0,
        ),
        (
            0x3fefffff_9b4c44e0,
            0x41ab8924_88f0a224,
            0x3c06842d_2773ab12,
            0,
        ),
    ];

    /// The exe's data sections, mapped from the installed executable `NV_EXE` points at.
    fn with_exe() -> Option<Engine> {
        let path = std::env::var("NV_EXE").ok().filter(|p| !p.is_empty())?;
        Some(Engine::with_exe(std::path::Path::new(&path)).expect("NV_EXE"))
    }

    /// An engine on the exe's data with `__use_sse2_mathfcns` set and the error support doubled.
    fn exe_engine() -> Option<(Engine, Seen)> {
        let mut e = with_exe()?;
        e.set_global(SSE2_FLAG, 1u32);
        let libm: Seen = Rc::default();
        let record = libm.clone();
        e.register_double(LIBM_ERROR_SUPPORT, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        Some((e, libm))
    }

    #[test]
    #[ignore = "needs NV_EXE: the SSE2 tables are the exe's"]
    fn pow_sse2_matches_the_recorded_results_on_the_exes_tables() {
        let Some((mut e, libm)) = exe_engine() else {
            return;
        };
        for &(x, y, want, error) in POW_SSE2_RECORDED {
            libm.borrow_mut().clear();
            let got = pow_of(&mut e, x, y);
            let code = libm.borrow().last().map_or(0, |w| w[3]);
            assert_eq!((got, code), (want, error), "pow({x:016x}, {y:016x})");
        }
    }

    // ---- acos, asin, atan ---------------------------------------------------------------------

    /// A recorded one-argument vector: the argument, the result, the error report (as
    /// [`Vector`]'s).
    type Vector1 = (u64, u64, u32);

    /// The constants the inverse bodies' x87 paths read (`trig.rs`'s `00ed27ac` adds 1.0 from
    /// `010f2a90`), at the exe's values.
    fn inverse_engine(sse2: bool) -> (Engine, Seen, Seen) {
        let (mut e, except, libm) = engine(sse2);
        put(&mut e, 0x010f_2a90, 1.0f64.to_bits());
        put_extended(&mut e, HALF_PI_EXTENDED, 0x3fff, 0xc90f_daa2_2168_c235);
        e.map(ATAN_CONTROL_WORD, 2);
        e.mem.set_u16(ATAN_CONTROL_WORD, 0x027f);
        (e, except, libm)
    }

    /// The x87 bodies' result paths as the exe runs them, under the body's control word.
    fn inverse_sequence(which: u32, x: f64, body: u16) -> u64 {
        crate::x87::set_control_word(body);
        let v = X87Ext::load64(x);
        let one = X87Ext::load64(1.0);
        let root = || x87_fsqrt(x87_fmul(x87_fadd(one, v), x87_fsub(one, v)));
        let r = match which {
            0 => x87_fpatan(root(), v),
            1 => x87_fpatan(v, root()),
            _ => x87_fpatan(v, one),
        };
        let bits = r.store64().to_bits();
        crate::x87::set_control_word(crate::x87::DEFAULT_CONTROL_WORD);
        bits
    }

    #[test]
    fn inverse_x87_bodies_run_their_fpatan_sequences_bit_for_bit() {
        let (mut e, _, _) = inverse_engine(false);
        let mut spread = Spread(0x0ac05);
        for word in [0x027fu16, 0x007f] {
            for _ in 0..100 {
                let x = spread.next() * 2.0 - 1.0;
                let wide = (spread.next() - 0.5) * 1e4;
                for (which, entry, register, arg) in [
                    (0u32, 0x00ec_a190u32, 0x00ec_a1d0u32, x),
                    (1, 0x00ec_9610, 0x00ec_9650, x),
                    (2, 0x00ec_9010, 0x00ec_9050, wide),
                ] {
                    // acos and asin run at the caller's precision, atan under 010f2a98's word.
                    let body = if which == 2 {
                        0x027f
                    } else {
                        (word & 0x300) | 0x7f
                    };
                    let want = inverse_sequence(which, arg, body);
                    for addr in [entry, register] {
                        set_x87_control_word(&mut e, word);
                        let got = e.call(addr, &args![arg]).f64().to_bits();
                        assert_eq!(x87_control_word(&mut e), word);
                        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
                        assert_eq!(got, want, "{addr:08x}({arg:e}) under {word:04x}");
                    }
                }
            }
        }
    }

    #[test]
    fn inverse_x87_special_values_and_errors() {
        let (mut e, except, _) = inverse_engine(false);
        let at = |e: &mut Engine, addr: u32, x: f64| e.call(addr, &args![x]).f64().to_bits();
        // acos(1) = 0, acos(-1) = pi (FLDPI rounded), asin(+-1) = +-pi/2, atan(+-inf) = +-pi/2.
        assert_eq!(at(&mut e, 0x00ec_a190, 1.0), 0);
        assert_eq!(
            at(&mut e, 0x00ec_a190, -1.0),
            std::f64::consts::PI.to_bits()
        );
        let half_pi = std::f64::consts::FRAC_PI_2.to_bits();
        assert_eq!(at(&mut e, 0x00ec_9610, 1.0), half_pi);
        assert_eq!(at(&mut e, 0x00ec_9610, -1.0), half_pi | (1 << 63));
        assert_eq!(at(&mut e, 0x00ec_9010, f64::INFINITY), half_pi);
        assert_eq!(
            at(&mut e, 0x00ec_9050, f64::NEG_INFINITY),
            half_pi | (1 << 63)
        );
        assert!(except.borrow().is_empty());
        // Domain errors: the indefinite NaN, type 1, the function's operation.
        for (addr, x, operation) in [
            (0x00ec_a190u32, 1.5f64, ACOS_OPERATION),
            (0x00ec_9610, -2.0, ASIN_OPERATION),
            (0x00ec_a1d0, f64::INFINITY, ACOS_OPERATION),
        ] {
            assert_eq!(at(&mut e, addr, x), 0xfff8_0000_0000_0000);
            let w = except.borrow().last().unwrap().clone();
            assert_eq!((w[0], w[3]), (operation, 1), "{addr:08x}({x})");
        }
        // A NaN: its type from 00ed27ac (a signalling one on the stack is type 1 and quieted).
        assert_eq!(
            at(&mut e, 0x00ec_9010, f64::from_bits(0x7ff4_0000_0000_0000)),
            0x7ffc_0000_0000_0000
        );
        let w = except.borrow().last().unwrap().clone();
        assert_eq!((w[0], w[3]), (ATAN_OPERATION, 1));
        assert_eq!(
            at(&mut e, 0x00ec_9010, f64::from_bits(0xfff8_0000_0000_0042)),
            0xfff8_0000_0000_0042
        );
        assert_eq!(except.borrow().last().unwrap()[3], 7);
        // The math-error switch: no report.
        e.set_global(ERROR_SWITCH, 1u32);
        except.borrow_mut().clear();
        assert_eq!(at(&mut e, 0x00ec_9610, 3.0), 0xfff8_0000_0000_0000);
        assert!(except.borrow().is_empty());
    }

    /// Recorded from the exe: `acos`, `asin` and `atan` (`00eca190`, `00ec9610`, `00ec9010`) on
    /// their x87 paths under 0x027f.
    const ACOS_X87_RECORDED: &[Vector1] = &[
        (0x40300000_00000000, 0xfff80000_00000000, 0x1_000d),
        (0x800fffff_ffffffff, 0x3ff921fb_54442d18, 0),
        (0x7ff40000_00000000, 0x7ffc0000_00000000, 0x1_000d),
        (0x3f79c38f_3a73600c, 0x3ff90837_b9e76b58, 0),
        (0xbfec3430_4e883d00, 0x40053248_60b571ca, 0),
        (0x554ac3bd_4d8d552b, 0xfff80000_00000000, 0x1_000d),
        (0xc06a5058_4e27fb1e, 0xfff80000_00000000, 0x1_000d),
        (0x3e744577_aaf9f102, 0x3ff921fb_3ffeb56d, 0),
        (0xc02e0000_00000000, 0xfff80000_00000000, 0x1_000d),
        (0x4096fc11_f153cdca, 0xfff80000_00000000, 0x1_000d),
        (0xc0330000_00000000, 0xfff80000_00000000, 0x1_000d),
        (0x32784719_8f63f56f, 0x3ff921fb_54442d18, 0),
        (0x40fca1a5_3e90d1d9, 0xfff80000_00000000, 0x1_000d),
        (0xbfdda656_afc51300, 0x40006b7d_ee828482, 0),
        (0x8676739f_bf6f2f0c, 0x3ff921fb_54442d18, 0),
        (0xffba8b6e_e8950ca4, 0xfff80000_00000000, 0x1_000d),
        (0x3fe750f3_f958da80, 0x3fe824a6_7c14b346, 0),
        (0x40350000_00000000, 0xfff80000_00000000, 0x1_000d),
        (0xc0000000_00000000, 0xfff80000_00000000, 0x1_000d),
        (0x7ff40000_00000000, 0x7ffc0000_00000000, 0x1_000d),
        (0x32c5f369_1f91e0ac, 0x3ff921fb_54442d18, 0),
        (0x3ff01e4b_af1036ff, 0xfff80000_00000000, 0x1_000d),
        (0xbfbf303b_6c85d000, 0x3ffb163d_302ba56b, 0),
        (0xc10bfcf6_5aed19f8, 0xfff80000_00000000, 0x1_000d),
        (0x41113c61_089a2d7b, 0xfff80000_00000000, 0x1_000d),
        (0x3fe5620c_c7b6f700, 0x3fead8f5_202b8f2f, 0),
        (0xa24b3e27_cad53f50, 0x3ff921fb_54442d18, 0),
        (0x25db985e_e59db937, 0x3ff921fb_54442d18, 0),
        (0x3fd2c2e2_ba943e00, 0x3ff45f5c_b05a8a0f, 0),
        (0x40e2240e_f253e43c, 0xfff80000_00000000, 0x1_000d),
        (0xc0b006b9_448d76db, 0xfff80000_00000000, 0x1_000d),
        (0x3fefffff_fffffffe, 0x3e56a09e_667f3bcc, 0),
        (0x033da741_a1a6c832, 0x3ff921fb_54442d18, 0),
        (0x981994a2_cd55157a, 0x3ff921fb_54442d18, 0),
        (0x3fefffff_fffffe92, 0x3e932191_811b0a53, 0),
        (0xbfefffdc_8c57d5c7, 0x40091612_ce1c7623, 0),
        (0xc24a0267_9f3300d9, 0xfff80000_00000000, 0x1_000d),
        (0x3fefffff_3a375827, 0x3f4c208b_6b138626, 0),
        (0xbfe222ab_49728740, 0x400162fb_03ed8ab6, 0),
        (0xbfd34c56_166d4400, 0x3ffe0897_8f4ab9f7, 0),
        (0xbfd7af1d_d415aa00, 0x3fff32ae_7f620e40, 0),
        (0x10528df5_9e6ffb99, 0x3ff921fb_54442d18, 0),
        (0x3fe2ae07_a74f7ac0, 0x3fee51a0_6bffc7bc, 0),
        (0x3f57085e_295b8000, 0x3ff91c39_3c9a043f, 0),
        (0xbfd02239_bc261880, 0x3ffd35cc_aece2a1e, 0),
        (0x3fe5ccad_7c9196c0, 0x3fea487b_69d821c7, 0),
        (0xbfefffff_ffebc7fa, 0x400921f2_560a20ed, 0),
        (0xbfcab083_62fa3f00, 0x3ffc7e5b_e2e31f82, 0),
        (0x428639c5_e7652593, 0xfff80000_00000000, 0x1_000d),
        (0x4007de20_12fb488b, 0xfff80000_00000000, 0x1_000d),
    ];
    const ASIN_X87_RECORDED: &[Vector1] = &[
        (0x40300000_00000000, 0xfff80000_00000000, 0x1_000e),
        (0x800fffff_ffffffff, 0x800fffff_ffffffff, 0),
        (0x7ff40000_00000000, 0x7ffc0000_00000000, 0x1_000e),
        (0x3f79c38f_3a73600c, 0x3f79c39a_5cc1c049, 0),
        (0xbfec3430_4e883d00, 0xbff14295_6d26b67b, 0),
        (0x554ac3bd_4d8d552b, 0xfff80000_00000000, 0x1_000e),
        (0xc06a5058_4e27fb1e, 0xfff80000_00000000, 0x1_000e),
        (0x3e744577_aaf9f102, 0x3e744577_aaf9f108, 0),
        (0xc02e0000_00000000, 0xfff80000_00000000, 0x1_000e),
        (0x4096fc11_f153cdca, 0xfff80000_00000000, 0x1_000e),
        (0xc0330000_00000000, 0xfff80000_00000000, 0x1_000e),
        (0x32784719_8f63f56f, 0x32784719_8f63f56f, 0),
        (0x40fca1a5_3e90d1d9, 0xfff80000_00000000, 0x1_000e),
        (0xbfdda656_afc51300, 0xbfded402_23036fad, 0),
        (0x8676739f_bf6f2f0c, 0x8676739f_bf6f2f0c, 0),
        (0xffba8b6e_e8950ca4, 0xfff80000_00000000, 0x1_000e),
        (0x3fe750f3_f958da80, 0x3fea1f50_2c73a6eb, 0),
        (0x40350000_00000000, 0xfff80000_00000000, 0x1_000e),
        (0xc0000000_00000000, 0xfff80000_00000000, 0x1_000e),
        (0x7ff40000_00000000, 0x7ffc0000_00000000, 0x1_000e),
        (0x32c5f369_1f91e0ac, 0x32c5f369_1f91e0ac, 0),
        (0x3ff01e4b_af1036ff, 0xfff80000_00000000, 0x1_000e),
        (0xbfbf303b_6c85d000, 0xbfbf441d_be778531, 0),
        (0xc10bfcf6_5aed19f8, 0xfff80000_00000000, 0x1_000e),
        (0x41113c61_089a2d7b, 0xfff80000_00000000, 0x1_000e),
        (0x3fe5620c_c7b6f700, 0x3fe76b01_885ccb02, 0),
        (0xa24b3e27_cad53f50, 0xa24b3e27_cad53f50, 0),
        (0x25db985e_e59db937, 0x25db985e_e59db937, 0),
        (0x3fd2c2e2_ba943e00, 0x3fd30a7a_8fa68c26, 0),
        (0x40e2240e_f253e43c, 0xfff80000_00000000, 0x1_000e),
        (0xc0b006b9_448d76db, 0xfff80000_00000000, 0x1_000e),
        (0x3fefffff_fffffffe, 0x3ff921fb_4e9c057f, 0),
        (0x033da741_a1a6c832, 0x033da741_a1a6c832, 0),
        (0x981994a2_cd55157a, 0x981994a2_cd55157a, 0),
        (0x3fefffff_fffffe92, 0x3ff921fb_07bde714, 0),
        (0xbfefffdc_8c57d5c7, 0xbff90a2a_47f4bf2d, 0),
        (0xc24a0267_9f3300d9, 0xfff80000_00000000, 0x1_000e),
        (0x3fefffff_3a375827, 0x3ff91e77_42d6caa8, 0),
        (0xbfe222ab_49728740, 0xbfe347f5_672dd0a7, 0),
        (0xbfd34c56_166d4400, 0xbfd39a70_ec1a337a, 0),
        (0xbfd7af1d_d415aa00, 0xbfd842cc_ac7784a1, 0),
        (0x10528df5_9e6ffb99, 0x10528df5_9e6ffb99, 0),
        (0x3fe2ae07_a74f7ac0, 0x3fe3f256_3c889274, 0),
        (0x3f57085e_295b8000, 0x3f57085e_a8a363b6, 0),
        (0xbfd02239_bc261880, 0xbfd04f45_6a27f416, 0),
        (0x3fe5ccad_7c9196c0, 0x3fe7fb7b_3eb03869, 0),
        (0xbfefffff_ffebc7fa, 0xbff921e9_57d014c1, 0),
        (0xbfcab083_62fa3f00, 0xbfcae304_74f7934b, 0),
        (0x428639c5_e7652593, 0xfff80000_00000000, 0x1_000e),
        (0x4007de20_12fb488b, 0xfff80000_00000000, 0x1_000e),
    ];
    const ATAN_X87_RECORDED: &[Vector1] = &[
        (0x40300000_00000000, 0x3ff82250_768ac529, 0),
        (0x800fffff_ffffffff, 0x800fffff_ffffffff, 0),
        (0x7ff40000_00000000, 0x7ffc0000_00000000, 0x1_000f),
        (0x3f79c38f_3a73600c, 0x3f79c378_f6133f4a, 0),
        (0xbfec3430_4e883d00, 0xbfe71e1e_d2d3f982, 0),
        (0x554ac3bd_4d8d552b, 0x3ff921fb_54442d18, 0),
        (0xc06a5058_4e27fb1e, 0xbff90e86_435684f1, 0),
        (0x3e744577_aaf9f102, 0x3e744577_aaf9f0f7, 0),
        (0xc02e0000_00000000, 0xbff81151_8cde39a6, 0),
        (0x4096fc11_f153cdca, 0x3ff91f32_81742e6f, 0),
        (0xc0330000_00000000, 0xbff84a99_fe25186b, 0),
        (0x32784719_8f63f56f, 0x32784719_8f63f56f, 0),
        (0x40fca1a5_3e90d1d9, 0x3ff921f2_6350239f, 0),
        (0xbfdda656_afc51300, 0xbfdbc40a_d7f39779, 0),
        (0x8676739f_bf6f2f0c, 0x8676739f_bf6f2f0c, 0),
        (0xffba8b6e_e8950ca4, 0xbff921fb_54442d18, 0),
        (0x3fe750f3_f958da80, 0x3fe42660_940cb182, 0),
        (0x40350000_00000000, 0x3ff85f14_d43d81be, 0),
        (0xc0000000_00000000, 0xbff1b6e1_92ebbe44, 0),
        (0x7ff40000_00000000, 0x7ffc0000_00000000, 0x1_000f),
        (0x32c5f369_1f91e0ac, 0x32c5f369_1f91e0ac, 0),
        (0x3ff01e4b_af1036ff, 0x3fe9402a_66d36c72, 0),
        (0xbfbf303b_6c85d000, 0xbfbf0914_0ab9ab2e, 0),
        (0xc10bfcf6_5aed19f8, 0xbff921f6_c17c045d, 0),
        (0x41113c61_089a2d7b, 0x3ff921f7_9db094f3, 0),
        (0x3fe5620c_c7b6f700, 0x3fe2d9b6_fe122138, 0),
        (0xa24b3e27_cad53f50, 0xa24b3e27_cad53f50, 0),
        (0x25db985e_e59db937, 0x25db985e_e59db937, 0),
        (0x3fd2c2e2_ba943e00, 0x3fd23ffe_78bfcd53, 0),
        (0x40e2240e_f253e43c, 0x3ff921df_1b06fac1, 0),
        (0xc0b006b9_448d76db, 0xbff920fb_bfaba887, 0),
        (0x3fefffff_fffffffe, 0x3fe921fb_54442d17, 0),
        (0x033da741_a1a6c832, 0x033da741_a1a6c832, 0),
        (0x981994a2_cd55157a, 0x981994a2_cd55157a, 0),
        (0x3fefffff_fffffe92, 0x3fe921fb_54442c61, 0),
        (0xbfefffdc_8c57d5c7, 0xbfe921e9_9a664650, 0),
        (0xc24a0267_9f3300d9, 0xbff921fb_5443de5b, 0),
        (0x3fefffff_3a375827, 0x3fe921fa_f15fd7fa, 0),
        (0xbfe222ab_49728740, 0xbfe07fc8_722dbe38, 0),
        (0xbfd34c56_166d4400, 0xbfd2be47_7848994d, 0),
        (0xbfd7af1d_d415aa00, 0xbfd6af12_810443ba, 0),
        (0x10528df5_9e6ffb99, 0x10528df5_9e6ffb99, 0),
        (0x3fe2ae07_a74f7ac0, 0x3fe0e87e_aa4d6937, 0),
        (0x3f57085e_295b8000, 0x3f57085d_2acbdb32, 0),
        (0xbfd02239_bc261880, 0xbfcf9bda_86f5541d, 0),
        (0x3fe5ccad_7c9196c0, 0x3fe322fc_5a8d01e5, 0),
        (0xbfefffff_ffebc7fa, 0xbfe921fb_543a1115, 0),
        (0xbfcab083_62fa3f00, 0xbfca4fff_d06fa6ce, 0),
        (0x428639c5_e7652593, 0x3ff921fb_54442756, 0),
        (0x4007de20_12fb488b, 0x3ff3f548_631b4816, 0),
    ];

    #[test]
    fn inverse_x87_matches_the_recorded_results() {
        let (mut e, except, _) = inverse_engine(false);
        for (addr, vectors) in [
            (0x00ec_a190u32, ACOS_X87_RECORDED),
            (0x00ec_9610, ASIN_X87_RECORDED),
            (0x00ec_9010, ATAN_X87_RECORDED),
        ] {
            for &(x, want, error) in vectors {
                except.borrow_mut().clear();
                let got = e.call(addr, &args![x]).f64().to_bits();
                assert_eq!(
                    (got, except_seen(&except)),
                    (want, error),
                    "{addr:08x}({x:016x})"
                );
            }
        }
    }

    #[test]
    fn inverse_entries_pick_their_bodies_from_the_flag_and_the_masks() {
        let (mut e, _, _) = inverse_engine(true);
        let seen: Seen = Rc::default();
        for (body, answer) in [
            (ACOS_SSE2, 1.0f64),
            (ACOS_SSE2_REGISTER, 2.0),
            (ASIN_SSE2, 3.0),
            (ATAN_SSE2, 4.0),
            (ATAN_SSE2_REGISTER, 5.0),
        ] {
            let record = seen.clone();
            e.register_double(body, move |_, a| {
                record.borrow_mut().push(a.to_vec());
                answer.into_ret()
            });
        }
        let at = |e: &mut Engine, addr: u32| e.call(addr, &args![0.25f64]).f64();
        assert_eq!(at(&mut e, 0x00ec_a190), 1.0);
        assert_eq!(at(&mut e, 0x00ec_a1d0), 2.0);
        assert_eq!(at(&mut e, 0x00ec_9610), 3.0);
        assert_eq!(at(&mut e, 0x00ec_9010), 4.0);
        assert_eq!(at(&mut e, 0x00ec_9050), 5.0);
        assert!(seen.borrow().iter().all(|w| w == &args![0.25f64]));
        // An unmasked x87 exception: the x87 bodies.
        set_x87_control_word(&mut e, 0x027e);
        let x87 = at(&mut e, 0x00ec_9010);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
        assert_eq!(x87.to_bits(), inverse_sequence(2, 0.25, 0x027f));
        // The register forms of the SSE2 bodies store the register (a signalling NaN quiet).
        e.register(LIBM_ERROR_SUPPORT, |_, _| Ret::default());
        put(&mut e, ATAN_32A0, 0x7fff_ffff_ffff_ffff);
        put(&mut e, ATAN_32A0 + 8, 0x7fff_ffff_ffff_ffff);
        put(&mut e, ATAN_33C8, 0x434d_0296_7c31_cdb5);
        assert_eq!(
            fn_00ed5d80(&mut e, f64::from_bits(0x7ff0_0000_0000_0001)).to_bits(),
            0x7ff8_0000_0000_0001
        );
    }

    #[test]
    fn inverse_sse2_special_values_without_the_tables() {
        let (mut e, _, libm) = engine(true);
        for (at, bits) in [
            (ACOS_AE90, 0x7fff_ffff_ffff_ffffu64),
            (ACOS_AE90 + 8, 0x7fff_ffff_ffff_ffff),
            (ACOS_AEA0, 1.0f64.to_bits()),
            (ACOS_AE98, 0xffff_c000_0000_0000),
            (ACOS_ADD0, 0x3c91_a626_3314_5c07),
            (ACOS_ADD0 + 8, 0x3ff9_21fb_5444_2d18),
            (ACOS_ADF0, 0x3ca1_a626_3314_5c07),
            (ACOS_ADF0 + 8, 0x4009_21fb_5444_2d18),
            (ASIN_4BD0, 0x7fff_ffff_ffff_ffff),
            (ASIN_4BE0, 1.0f64.to_bits()),
            (ASIN_4BD8, 0xffff_c000_0000_0000),
            (ASIN_4B40, 0x3c91_a626_3314_5c07),
            (ASIN_4B40 + 8, 0x3ff9_21fb_5444_2d18),
            (ATAN_32A0, 0x7fff_ffff_ffff_ffff),
            (ATAN_32A0 + 8, 0x7fff_ffff_ffff_ffff),
            (ATAN_33C8, 0x434d_0296_7c31_cdb5),
            (ATAN_33B8, 0x3fa0_0000_0000_0000),
            (ATAN_33C0, 0x3e40_0000_0000_0000),
            (ATAN_3388, 0),
            (ATAN_33D0, 0x0010_0000_0000_0000),
            (ATAN_33D8, 0x0200_0000_0000_0000),
            (ATAN_3290, 0x7ff0_0000_0000_0000),
            (ATAN_3290 + 8, 0x7ff0_0000_0000_0000),
            (ATAN_32B0, 0x3ff9_21fb_5444_2d18),
            (ATAN_32B0 + 8, 0xbff9_21fb_5444_2d18),
        ] {
            put(&mut e, at, bits);
        }
        let half_pi = 0x3ff9_21fb_5444_2d18u64;
        // Recorded from the exe (entry, argument, result, error code).
        let cases: [(u32, u64, u64, u32); 19] = [
            (0x00ec_a190, 0x3ff0_0000_0000_0000, 0, 0),
            (0x00ec_a190, 0xbff0_0000_0000_0000, 0x4009_21fb_5444_2d18, 0),
            (
                0x00ec_a190,
                0x4000_0000_0000_0000,
                0xfff8_0000_0000_0000,
                0x3a,
            ),
            (
                0x00ec_a190,
                0x7ff4_0000_0000_0000,
                0x7ffc_0000_0000_0000,
                0x3f0,
            ),
            (0x00ec_a190, 0x0000_0000_0000_0001, half_pi, 0),
            (0x00ec_a190, 0x3c00_0000_0000_0000, half_pi, 0),
            (0x00ec_9610, 0x3ff0_0000_0000_0000, half_pi, 0),
            (0x00ec_9610, 0xbff0_0000_0000_0000, 0xbff9_21fb_5444_2d18, 0),
            (0x00ec_9610, 0x8000_0000_0000_0001, 0x8000_0000_0000_0001, 0),
            (
                0x00ec_9610,
                0xc000_0000_0000_0000,
                0xfff8_0000_0000_0000,
                0x3d,
            ),
            (
                0x00ec_9610,
                0xfff8_0000_0000_0005,
                0xfff8_0000_0000_0005,
                0x3f1,
            ),
            (0x00ec_9610, 0x3c00_0000_0000_0000, 0x3c00_0000_0000_0000, 0),
            (0x00ec_9010, 0x7ff0_0000_0000_0000, half_pi, 0),
            (0x00ec_9010, 0xfff0_0000_0000_0000, 0xbff9_21fb_5444_2d18, 0),
            (
                0x00ec_9010,
                0x7ff4_0000_0000_0000,
                0x7ffc_0000_0000_0000,
                0x3eb,
            ),
            (0x00ec_9010, 0x8000_0000_0000_0000, 0x8000_0000_0000_0000, 0),
            (0x00ec_9010, 0x0000_0000_0000_0003, 0x0000_0000_0000_0003, 0),
            (0x00ec_9010, 0x3000_0000_0000_0000, 0x3000_0000_0000_0000, 0),
            (0x00ec_9010, 0x4400_0000_0000_0000, half_pi, 0),
        ];
        for (addr, x, want, error) in cases {
            libm.borrow_mut().clear();
            let got = e.call(addr, &args![x]).f64().to_bits();
            let code = libm.borrow().last().map_or(0, |w| w[3]);
            assert_eq!((got, code), (want, error), "{addr:08x}({x:016x})");
        }
    }

    /// Recorded from the exe: the three on their SSE2 paths.
    const ACOS_SSE2_RECORDED: &[Vector1] = &[
        (0xbff13cba_2cdf1b80, 0xfff80000_00000000, 0x3a),
        (0x7ff80000_00000000, 0x7ff80000_00000000, 0x3f0),
        (0x43e5229e_fd6f30b2, 0xfff80000_00000000, 0x3a),
        (0xc104c745_4965dada, 0xfff80000_00000000, 0x3a),
        (0xc735a718_d6e4d08e, 0xfff80000_00000000, 0x3a),
        (0xbfec3430_4e883d00, 0x40053248_60b571c9, 0),
        (0xe8c387fd_0083ca96, 0xfff80000_00000000, 0x3a),
        (0xecc44e6f_1a5c7148, 0xfff80000_00000000, 0x3a),
        (0x4190fc4f_944a10e8, 0xfff80000_00000000, 0x3a),
        (0x411bba08_990fd63c, 0xfff80000_00000000, 0x3a),
        (0x40240000_00000000, 0xfff80000_00000000, 0x3a),
        (0x4096fc11_f153cdca, 0xfff80000_00000000, 0x3a),
        (0xc11d2acd_db0e343c, 0xfff80000_00000000, 0x3a),
        (0xc03d0000_00000000, 0xfff80000_00000000, 0x3a),
        (0x3fb4703f_b9d9fc00, 0x3ff7da9e_27aa88c9, 0),
        (0xc113f0a9_12c25992, 0xfff80000_00000000, 0x3a),
        (0x44273073_7ac059aa, 0xfff80000_00000000, 0x3a),
        (0x8676739f_bf6f2f0c, 0x3ff921fb_54442d18, 0),
        (0xbfa29687_eca9c000, 0x3ff9b6b7_f1bd6b78, 0),
        (0x40b78b93_adddfb90, 0xfff80000_00000000, 0x3a),
        (0xbf89c85a_ca443977, 0x3ff9558c_631e26da, 0),
        (0x410007db_90eeda3d, 0xfff80000_00000000, 0x3a),
        (0x40f573a4_e3afbe8c, 0xfff80000_00000000, 0x3a),
        (0x7ff40000_00000000, 0x7ffc0000_00000000, 0x3f0),
        (0x49429584_e0f80cb3, 0xfff80000_00000000, 0x3a),
        (0x411a6a8c_9b03764b, 0xfff80000_00000000, 0x3a),
        (0xc0f0cf01_bfdeffca, 0xfff80000_00000000, 0x3a),
        (0x40000000_00000000, 0xfff80000_00000000, 0x3a),
        (0xbff6bf6d_ef6353c0, 0xfff80000_00000000, 0x3a),
        (0x41113c61_089a2d7b, 0xfff80000_00000000, 0x3a),
        (0xbfc96088_85915800, 0x3ffc5376_f5543f87, 0),
        (0x41fc20eb_94f519a5, 0xfff80000_00000000, 0x3a),
        (0xbfeb3bdf_728bd979, 0x4004b5d9_f5b97d6e, 0),
        (0xc378f3bc_4cd9b30a, 0xfff80000_00000000, 0x3a),
        (0x3feb6f19_7ee8b180, 0x3fe14dff_40639979, 0),
        (0x40e2240e_f253e43c, 0xfff80000_00000000, 0x3a),
        (0x4119bf57_5649010f, 0xfff80000_00000000, 0x3a),
        (0xbfe2cd5a_e0da7140, 0x4001973c_96e62853, 0),
        (0x3fef7eb6_607d85c0, 0x3fc6c55d_548b7496, 0),
        (0x3fefffff_fffe07c1, 0x3ed67496_b4ce399b, 0),
        (0xbfc672a7_8b49d700, 0x3ffbf40c_32d89abf, 0),
        (0x3fefffff_fffffe92, 0x3e932191_811b0a53, 0),
        (0xbfefffd8_adf64f71, 0x40091570_c685d32a, 0),
        (0xbfefffff_bfbdc986, 0x400920fa_cff93cc4, 0),
        (0xbfd8970b_20b6d180, 0x3fff7149_8b028ff3, 0),
        (0xbff00000_00000000, 0x400921fb_54442d18, 0),
        (0x3f1b51e0_afa36ea4, 0x3ff9218e_0cc16b39, 0),
        (0xbfd34c56_166d4400, 0x3ffe0897_8f4ab9f7, 0),
        (0x3feffc0c_b0cb795e, 0x3f9fcd68_114366c6, 0),
        (0x3fefffff_fdb095b9, 0x3f1851ab_d9bfc949, 0),
        (0x42326de7_3977435e, 0xfff80000_00000000, 0x3a),
        (0x83400b48_c13f6b4e, 0x3ff921fb_54442d18, 0),
        (0xc21b2f93_452584c1, 0xfff80000_00000000, 0x3a),
        (0xbfd02239_bc261880, 0x3ffd35cc_aece2a1e, 0),
        (0x9472cff3_b197b003, 0x3ff921fb_54442d18, 0),
        (0x3fefffef_b7efe494, 0x3f7023e0_852514ff, 0),
        (0x3fefffff_fffffff7, 0x3e680000_00000001, 0),
        (0xbfe036a6_85010800, 0x4000d121_ff001687, 0),
        (0xbfe4ea27_0d248240, 0x400243cc_b70ed656, 0),
        (0x4007de20_12fb488b, 0xfff80000_00000000, 0x3a),
    ];
    const ASIN_SSE2_RECORDED: &[Vector1] = &[
        (0xbff13cba_2cdf1b80, 0xfff80000_00000000, 0x3d),
        (0x7ff80000_00000000, 0x7ff80000_00000000, 0x3f1),
        (0x43e5229e_fd6f30b2, 0xfff80000_00000000, 0x3d),
        (0xc104c745_4965dada, 0xfff80000_00000000, 0x3d),
        (0xc735a718_d6e4d08e, 0xfff80000_00000000, 0x3d),
        (0xbfec3430_4e883d00, 0xbff14295_6d26b67b, 0),
        (0xe8c387fd_0083ca96, 0xfff80000_00000000, 0x3d),
        (0xecc44e6f_1a5c7148, 0xfff80000_00000000, 0x3d),
        (0x4190fc4f_944a10e8, 0xfff80000_00000000, 0x3d),
        (0x411bba08_990fd63c, 0xfff80000_00000000, 0x3d),
        (0x40240000_00000000, 0xfff80000_00000000, 0x3d),
        (0x4096fc11_f153cdca, 0xfff80000_00000000, 0x3d),
        (0xc11d2acd_db0e343c, 0xfff80000_00000000, 0x3d),
        (0xc03d0000_00000000, 0xfff80000_00000000, 0x3d),
        (0x3fb4703f_b9d9fc00, 0x3fb475d2_c99a44f7, 0),
        (0xc113f0a9_12c25992, 0xfff80000_00000000, 0x3d),
        (0x44273073_7ac059aa, 0xfff80000_00000000, 0x3d),
        (0x8676739f_bf6f2f0c, 0x8676739f_bf6f2f0c, 0),
        (0xbfa29687_eca9c000, 0xbfa29793_af27cc01, 0),
        (0x40b78b93_adddfb90, 0xfff80000_00000000, 0x3d),
        (0xbf89c85a_ca443977, 0xbf89c887_6cfce0eb, 0),
        (0x410007db_90eeda3d, 0xfff80000_00000000, 0x3d),
        (0x40f573a4_e3afbe8c, 0xfff80000_00000000, 0x3d),
        (0x7ff40000_00000000, 0x7ffc0000_00000000, 0x3f1),
        (0x49429584_e0f80cb3, 0xfff80000_00000000, 0x3d),
        (0x411a6a8c_9b03764b, 0xfff80000_00000000, 0x3d),
        (0xc0f0cf01_bfdeffca, 0xfff80000_00000000, 0x3d),
        (0x40000000_00000000, 0xfff80000_00000000, 0x3d),
        (0xbff6bf6d_ef6353c0, 0xfff80000_00000000, 0x3d),
        (0x41113c61_089a2d7b, 0xfff80000_00000000, 0x3d),
        (0xbfc96088_85915800, 0xbfc98bdd_0880937a, 0),
        (0x41fc20eb_94f519a5, 0xfff80000_00000000, 0x3d),
        (0xbfeb3bdf_728bd979, 0xbff049b8_972ecdc3, 0),
        (0xc378f3bc_4cd9b30a, 0xfff80000_00000000, 0x3d),
        (0x3feb6f19_7ee8b180, 0x3ff07afb_b412605c, 0),
        (0x40e2240e_f253e43c, 0xfff80000_00000000, 0x3d),
        (0x4119bf57_5649010f, 0xfff80000_00000000, 0x3d),
        (0xbfe2cd5a_e0da7140, 0xbfe418fb_b310471a, 0),
        (0x3fef7eb6_607d85c0, 0x3ff6494f_a9b2be85, 0),
        (0x3fefffff_fffe07c1, 0x3ff921f5_b71e7fe5, 0),
        (0xbfc672a7_8b49d700, 0xbfc69086_f4a36d36, 0),
        (0x3fefffff_fffffe92, 0x3ff921fb_07bde714, 0),
        (0xbfefffd8_adf64f71, 0xbff908e6_38c7793c, 0),
        (0xbfefffff_bfbdc986, 0xbff91ffa_4bae4c70, 0),
        (0xbfd8970b_20b6d180, 0xbfd93d38_daf98b6c, 0),
        (0xbff00000_00000000, 0xbff921fb_54442d18, 0),
        (0x3f1b51e0_afa36ea4, 0x3f1b51e0_b077d619, 0),
        (0xbfd34c56_166d4400, 0xbfd39a70_ec1a337a, 0),
        (0x3feffc0c_b0cb795e, 0x3ff8a2c5_b3ff1f7d, 0),
        (0x3fefffff_fdb095b9, 0x3ff9219a_0d94c619, 0),
        (0x42326de7_3977435e, 0xfff80000_00000000, 0x3d),
        (0x83400b48_c13f6b4e, 0x83400b48_c13f6b4e, 0),
        (0xc21b2f93_452584c1, 0xfff80000_00000000, 0x3d),
        (0xbfd02239_bc261880, 0xbfd04f45_6a27f416, 0),
        (0x9472cff3_b197b003, 0x9472cff3_b197b003, 0),
        (0x3fefffef_b7efe494, 0x3ff911d7_73bf0803, 0),
        (0x3fefffff_fffffff7, 0x3ff921fb_48442d18, 0),
        (0xbfe036a6_85010800, 0xbfe10091_5377ffea, 0),
        (0xbfe4ea27_0d248240, 0xbfe6cb3c_33b2ff26, 0),
        (0x4007de20_12fb488b, 0xfff80000_00000000, 0x3d),
    ];
    const ATAN_SSE2_RECORDED: &[Vector1] = &[
        (0xbff13cba_2cdf1b80, 0xbfea52c7_4a9b1284, 0),
        (0x7ff80000_00000000, 0x7ff80000_00000000, 0x3eb),
        (0x43e5229e_fd6f30b2, 0x3ff921fb_54442d18, 0),
        (0xc104c745_4965dada, 0xbff921f5_2b3e7d06, 0),
        (0xc735a718_d6e4d08e, 0xbff921fb_54442d18, 0),
        (0xbfec3430_4e883d00, 0xbfe71e1e_d2d3f982, 0),
        (0xe8c387fd_0083ca96, 0xbff921fb_54442d18, 0),
        (0xecc44e6f_1a5c7148, 0xbff921fb_54442d18, 0),
        (0x4190fc4f_944a10e8, 0x3ff921fb_507f9802, 0),
        (0x411bba08_990fd63c, 0x3ff921f9_055b0969, 0),
        (0x40240000_00000000, 0x3ff789bd_2c160054, 0),
        (0x4096fc11_f153cdca, 0x3ff91f32_81742e6f, 0),
        (0xc11d2acd_db0e343c, 0xbff921f9_228a1d6a, 0),
        (0xc03d0000_00000000, 0xbff894cb_db6bedfc, 0),
        (0x3fb4703f_b9d9fc00, 0x3fb4652c_a1c33e0c, 0),
        (0xc113f0a9_12c25992, 0xbff921f8_1e9ac7a1, 0),
        (0x44273073_7ac059aa, 0x3ff921fb_54442d18, 0),
        (0x8676739f_bf6f2f0c, 0x8676739f_bf6f2f0c, 0),
        (0xbfa29687_eca9c000, 0xbfa29471_25415a64, 0),
        (0x40b78b93_adddfb90, 0x3ff9214d_5db61a83, 0),
        (0xbf89c85a_ca443977, 0xbf89c801_88a0697e, 0),
        (0x410007db_90eeda3d, 0x3ff921f3_58300891, 0),
        (0x40f573a4_e3afbe8c, 0x3ff921ef_6538d5e5, 0),
        (0x7ff40000_00000000, 0x7ffc0000_00000000, 0x3eb),
        (0x49429584_e0f80cb3, 0x3ff921fb_54442d18, 0),
        (0x411a6a8c_9b03764b, 0x3ff921f8_e80a811b, 0),
        (0xc0f0cf01_bfdeffca, 0xbff921ec_1950879d, 0),
        (0x40000000_00000000, 0x3ff1b6e1_92ebbe44, 0),
        (0xbff6bf6d_ef6353c0, 0xbfeea66b_ebfcaf3b, 0),
        (0x41113c61_089a2d7b, 0x3ff921f7_9db094f3, 0),
        (0xbfc96088_85915800, 0xbfc90d5e_16c247bb, 0),
        (0x41fc20eb_94f519a5, 0x3ff921fb_543b1339, 0),
        (0xbfeb3bdf_728bd979, 0xbfe6903f_6e3e85c4, 0),
        (0xc378f3bc_4cd9b30a, 0xbff921fb_54442d18, 0),
        (0x3feb6f19_7ee8b180, 0x3fe6addd_6cec7538, 0),
        (0x40e2240e_f253e43c, 0x3ff921df_1b06fac1, 0),
        (0x4119bf57_5649010f, 0x3ff921f8_d7ee4d7a, 0),
        (0xbfe2cd5a_e0da7140, 0xbfe0ffd1_d3586432, 0),
        (0x3fef7eb6_607d85c0, 0x3fe8e0d3_3e180575, 0),
        (0x3fefffff_fffe07c1, 0x3fe921fb_544330f9, 0),
        (0xbfc672a7_8b49d700, 0xbfc638cd_b6cddd9b, 0),
        (0x3fefffff_fffffe92, 0x3fe921fb_54442c61, 0),
        (0xbfefffd8_adf64f71, 0xbfe921e7_ab334099, 0),
        (0xbfefffff_bfbdc986, 0xbfe921fb_342311bb, 0),
        (0xbfd8970b_20b6d180, 0xbfd77a1e_09783ed9, 0),
        (0xbff00000_00000000, 0xbfe921fb_54442d18, 0),
        (0x3f1b51e0_afa36ea4, 0x3f1b51e0_adfa9fb9, 0),
        (0xbfd34c56_166d4400, 0xbfd2be47_7848994d, 0),
        (0x3feffc0c_b0cb795e, 0x3fe92001_8d726b98, 0),
        (0x3fefffff_fdb095b9, 0x3fe921fb_531c77f5, 0),
        (0x42326de7_3977435e, 0x3ff921fb_54434ed7, 0),
        (0x83400b48_c13f6b4e, 0x83400b48_c13f6b4e, 0),
        (0xc21b2f93_452584c1, 0xbff921fb_5441d26e, 0),
        (0xbfd02239_bc261880, 0xbfcf9bda_86f5541d, 0),
        (0x9472cff3_b197b003, 0x9472cff3_b197b003, 0),
        (0x3fefffef_b7efe494, 0x3fe921f3_303a0d35, 0),
        (0x3fefffff_fffffff7, 0x3fe921fb_54442d14, 0),
        (0xbfe036a6_85010800, 0xbfde039c_1124b047, 0),
        (0xbfe4ea27_0d248240, 0xbfe28644_3a293ad7, 0),
        (0x4007de20_12fb488b, 0x3ff3f548_631b4816, 0),
    ];

    #[test]
    #[ignore = "needs NV_EXE: the SSE2 tables are the exe's"]
    fn inverse_sse2_matches_the_recorded_results_on_the_exes_tables() {
        let Some((mut e, libm)) = exe_engine() else {
            return;
        };
        for (addr, vectors) in [
            (0x00ec_a190u32, ACOS_SSE2_RECORDED),
            (0x00ec_9610, ASIN_SSE2_RECORDED),
            (0x00ec_9010, ATAN_SSE2_RECORDED),
        ] {
            for &(x, want, error) in vectors {
                libm.borrow_mut().clear();
                let got = e.call(addr, &args![x]).f64().to_bits();
                let code = libm.borrow().last().map_or(0, |w| w[3]);
                assert_eq!((got, code), (want, error), "{addr:08x}({x:016x})");
            }
        }
    }

    // ---- exp and atan2 ------------------------------------------------------------------------

    /// The dispatch's data at the exe's values: `exp`'s and `atan2`'s descriptors (name, argument
    /// count, operation, handler table), the `FXAM` class table, `exp`'s bound, the NaN addend,
    /// the tail's scaling, the extended constants, and the Pentium flag clear.
    fn dispatch_engine() -> (Engine, Seen, Seen) {
        let (mut e, except, libm) = inverse_engine(false);
        e.map(EXP_DESCRIPTOR, 0x20);
        e.mem.write(EXP_DESCRIPTOR, b"\x03exp");
        e.mem.write(EXP_DESCRIPTOR + 0xd, &[1, 0x14]);
        for (i, target) in [0x00ed_9301u32, 0x00ed_265d, 0x00ed_2662, 0x00ed_9441]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(EXP_DESCRIPTOR + 0x10 + 4 * i as u32, target);
        }
        e.map(ATAN2_DESCRIPTOR, 0x50);
        e.mem.write(ATAN2_DESCRIPTOR, b"\x05atan2");
        e.mem.write(ATAN2_DESCRIPTOR + 0xd, &[2, 0x10]);
        for (i, target) in [
            0x00ed_7cbcu32,
            0x00ed_7d38,
            0x00ed_268f,
            0x00ed_7d15,
            0x00ed_7d15,
            0x00ed_7d15,
            0x00ed_268f,
            0x00ed_7d15,
            0x00ed_268d,
            0x00ed_268d,
            0x00ed_26b7,
            0x00ed_268d,
            0x00ed_7d38,
            0x00ed_7d38,
            0x00ed_268f,
            0x00ed_26f6,
        ]
        .into_iter()
        .enumerate()
        {
            e.mem
                .set_u32(ATAN2_DESCRIPTOR + 0x10 + 4 * i as u32, target);
        }
        e.map(FXAM_CLASSES, 16);
        e.mem.write(
            FXAM_CLASSES,
            &[8, 4, 8, 8, 8, 4, 8, 8, 0, 4, 0xc, 8, 0, 4, 0xc, 8],
        );
        put_extended(&mut e, EXP_BOUND, 0x400d, 0xffff_0000_0000_0000);
        put(&mut e, DISPATCH_NAN_ADDEND, 1.0f64.to_bits());
        put(&mut e, TAIL_UNDERFLOW_SCALE, 1536.0f64.to_bits());
        put(&mut e, TAIL_UNDERFLOW_BOUND, f64::MIN_POSITIVE.to_bits());
        put(&mut e, TAIL_UNDERFLOW_FACTOR, 0);
        put(&mut e, TAIL_OVERFLOW_SCALE, (-1536.0f64).to_bits());
        put(&mut e, TAIL_OVERFLOW_BOUND, f64::MAX.to_bits());
        put(&mut e, TAIL_OVERFLOW_FACTOR, f64::INFINITY.to_bits());
        e.map(FDIV_FLAWED, 4);
        e.map(MATHERR_SKIP, 4);
        (e, except, libm)
    }

    /// `exp`'s finite handler and `atan2`'s handler for two normal operands, as the exe runs
    /// them at the dispatch word (0x133f), rounded to a double under the caller's word.
    fn dispatch_sequence(atan2: bool, y: f64, x: f64, word: u16) -> u64 {
        crate::x87::set_control_word(0x133f);
        let r = if atan2 {
            let mut r = x87_fpatan(X87Ext::load64(y.abs()), X87Ext::load64(x.abs()));
            if x < 0.0 {
                r = x87_fsub(x87_fld_constant(X87Constant::Pi), r);
            }
            if y < 0.0 {
                r = x87_fchs(r);
            }
            r
        } else {
            let t = x87_fmul(X87Ext::load64(x), x87_fld_constant(X87Constant::Log2E));
            let n = x87_frndint(t);
            let f = x87_fsub(t, n);
            let mut r = x87_fadd(x87_f2xm1(x87_fabs(f)), X87Ext::load64(1.0));
            if f.sign_exponent() & 0x8000 != 0 && f.significand() != 0 {
                r = x87_fdiv(X87Ext::load64(1.0), r);
            }
            if n.significand() != 0 {
                r = x87_fscale(r, n);
            }
            r
        };
        crate::x87::set_control_word(word);
        let bits = r.store64().to_bits();
        crate::x87::set_control_word(crate::x87::DEFAULT_CONTROL_WORD);
        bits
    }

    #[test]
    fn exp_and_atan2_x87_run_their_handlers_bit_for_bit() {
        let (mut e, except, _) = dispatch_engine();
        let mut spread = Spread(0xe4b2);
        for word in [0x027fu16, 0x007f, 0x0e7f] {
            for _ in 0..100 {
                let x = (spread.next() - 0.5) * 1000.0;
                let y = (spread.next() - 0.5) * 50.0;
                for (atan2, entry, register, args) in [
                    (false, 0x00ec_ae30u32, 0x00ec_ae6cu32, args![x]),
                    (true, 0x00ec_9760, 0x00ec_976a, args![y, x]),
                ] {
                    let want = dispatch_sequence(atan2, y, x, word);
                    for addr in [entry, register] {
                        set_x87_control_word(&mut e, word);
                        let got = e.call(addr, &args).f64().to_bits();
                        assert_eq!(x87_control_word(&mut e), word);
                        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
                        assert_eq!(got, want, "{addr:08x}({y:e}, {x:e}) under {word:04x}");
                    }
                }
            }
        }
        assert!(except.borrow().is_empty());
    }

    /// Recorded from the exe: `exp` (`00ecae30`) and `atan2` (`00ec9760`) on their x87 paths
    /// under 0x027f.
    const EXP_X87_RECORDED: &[Vector1] = &[
        (0x3f85ae37_274cc5e6, 0x3ff02b97_64f9d88d, 0),
        (0xc086554b_da14a6fe, 0x5f7f236e_a4e4af36, 0x4_0014),
        (0xc0327f6f_0ea57132, 0x3e43e195_ea682d02, 0),
        (0x402bae26_244ed780, 0x412f4743_bd9ffd31, 0),
        (0x3c0c23f0_afb4a9be, 0x3ff00000_00000000, 0),
        (0xc0589fc9_82b30540, 0x370dd7e4_60c69672, 0),
        (0xc0860548_7a870fca, 0x0064f72f_5cd1738c, 0),
        (0xc00ef049_208f5266, 0x3f956a88_20a0be19, 0),
        (0xc031f179_a870fe11, 0x3e514ec6_ede43679, 0),
        (0x40870f58_b9e03636, 0x22781a37_9f4cb9c3, 0x3_0014),
        (0xc01c5c26_8b9769e4, 0x3f4b4f1b_b3653f14, 0),
        (0x4082cc54_6348e902, 0x762ca6a7_9bf2a9bb, 0),
        (0xc06025e8_6e95c278, 0x3448b135_b363d041, 0),
        (0xbad4687d_ca9cd9ab, 0x3ff00000_00000000, 0),
        (0xbff70022_31bad119, 0x3fce66d4_11cb3551, 0),
        (0x4022f5cb_1f4642f7, 0x40c99405_5f66da89, 0),
        (0x408708be_7ac5b7ae, 0x22651e4d_a8467253, 0x3_0014),
        (0xc0869284_60546d5f, 0x5ece477a_017265a1, 0x4_0014),
        (0x407eb343_bb510c44, 0x6c393d61_9a3891ba, 0),
        (0x4085f89e_c37057fe, 0x7f541015_9dfe8b8f, 0),
        (0x1a3780d8_545ba158, 0x3ff00000_00000000, 0),
        (0x43e5229e_fd6f30b2, 0x7ff00000_00000000, 0x3_0014),
        (0xc119b631_41af58f5, 0x00000000_00000000, 0x4_0014),
        (0xbfec3430_4e883d00, 0x3fda827e_a225be95, 0),
        (0x3fdefa50_82198f00, 0x3ff9f628_535f4967, 0),
        (0x4190fc4f_944a10e8, 0x7ff00000_00000000, 0x3_0014),
        (0xf097b062_c41a3152, 0x00000000_00000000, 0x4_0014),
        (0x4096fc11_f153cdca, 0x6492c2e1_786ccea8, 0x3_0014),
        (0x3ff0ab0b_badf6700, 0x4006ac70_5742fb61, 0),
        (0x3fb4703f_b9d9fc00, 0x3ff1546c_923a4b01, 0),
        (0x4114d55e_5675d0e0, 0x7ff00000_00000000, 0x3_0014),
        (0x8676739f_bf6f2f0c, 0x3ff00000_00000000, 0),
        (0x408ff000_00000000, 0x3c159edc_d37d3631, 0x3_0014),
        (0xbf89c85a_ca443977, 0x3fef9984_128129a0, 0),
        (0xc0240000_00000000, 0x3f07cd79_b5647c9b, 0),
        (0x7ff40000_00000000, 0x7ffc0000_00000000, 0x1_0014),
        (0x3f643e6f_b6d43a5a, 0x3ff00a22_6c29b9e4, 0),
        (0xc0f0cf01_bfdeffca, 0x00000000_00000000, 0x4_0014),
        (0x7fefffff_ffffffff, 0x7ff00000_00000000, 0x3_0014),
        (0x41113c61_089a2d7b, 0x7ff00000_00000000, 0x3_0014),
        (0x40f53301_ab30c756, 0x7ff00000_00000000, 0x3_0014),
        (0xbfeb3bdf_728bd979, 0x3fdb535b_3aae60c5, 0),
        (0x3fe20369_087e91c0, 0x3ffc17b2_76419242, 0),
        (0x40e2240e_f253e43c, 0x7ff00000_00000000, 0x3_0014),
        (0xbfefffff_fffffffe, 0x3fd78b56_362cef39, 0),
        (0x3fef7eb6_607d85c0, 0x400567de_ef1eb7f7, 0),
        (0x3fb74b04_57120000, 0x3ff1862b_7da36a96, 0),
        (0x3fefffff_fffffe92, 0x4005bf0a_8b145671, 0),
        (0x81413427_fdf207a6, 0x3ff00000_00000000, 0),
        (0xbfd8970b_20b6d180, 0x3fe5ca9b_65711fcd, 0),
        (0x3fa9694f_53ab1400, 0x3ff0d06b_967b459e, 0),
        (0xbfd34c56_166d4400, 0x3fe7ab7a_b31a6de2, 0),
        (0xbfd69291_45b6a880, 0x3fe67d43_7e1ce871, 0),
        (0x42326de7_3977435e, 0x7ff00000_00000000, 0x3_0014),
        (0xb1eff83f_f9ba6119, 0x3ff00000_00000000, 0),
        (0xbfd02239_bc261880, 0x3fe8de9f_5b9f4082, 0),
        (0x3fd07f9e_f6094e00, 0x3ff4b47e_c6048a39, 0),
        (0x3fefffff_fffffff7, 0x4005bf0a_8b145763, 0),
        (0xbfdabc81_48064480, 0x3fe5129f_60534ec9, 0),
        (0x4007de20_12fb488b, 0x4033c18c_494cb377, 0),
    ];
    const ATAN2_X87_RECORDED: &[Vector] = &[
        (
            0xfff80000_00000000,
            0x7ff80000_00000123,
            0x7ff80000_00000000,
            0x7_0010,
        ),
        (
            0x00000000_00000001,
            0x7fefffff_ffffffff,
            0x1cd00000_00000000,
            0x4_0010,
        ),
        (
            0x40000000_00000000,
            0x408ff000_00000000,
            0x3f600802_aaa9dd33,
            0,
        ),
        (
            0x40240000_00000000,
            0x7ff40000_00000000,
            0x7ffc0000_00000000,
            0x1_0010,
        ),
        (
            0xc3400000_00000000,
            0x3fe00000_00000000,
            0xbff921fb_54442d18,
            0,
        ),
        (
            0x00080000_00000000,
            0x43300000_00000001,
            0x5cbfffff_fffffffe,
            0x4_0010,
        ),
        (
            0xbff55652_fd0dae80,
            0x1a599520_09fd3179,
            0xbff921fb_54442d18,
            0,
        ),
        (
            0xffefffff_ffffffff,
            0xc02e0000_00000000,
            0xbff921fb_54442d18,
            0,
        ),
        (
            0x3e6c3243_0bfd0e8f,
            0xc0330000_00000000,
            0x400921fb_53e532f3,
            0,
        ),
        (
            0xc0330000_00000000,
            0x40140000_00000000,
            0xbff503fb_db7f996e,
            0,
        ),
        (
            0x4082562b_0b59ba56,
            0x7a4a63a8_9c55665d,
            0x06263c35_acffd124,
            0,
        ),
        (
            0x3ff58948_086f9f40,
            0x2df36655_a8ad9be3,
            0x3ff921fb_54442d18,
            0,
        ),
        (
            0xc0300000_00000000,
            0x41161c6f_c9d10e49,
            0xbf0727e3_3544044f,
            0,
        ),
        (
            0x40e3ecd6_f5efd387,
            0xbfd02d9e_7f3cff00,
            0x3ff92201_d32307da,
            0,
        ),
        (
            0x699785ff_a10dd421,
            0xbe74121b_2da0b6ce,
            0x3ff921fb_54442d18,
            0,
        ),
        (
            0x3f404737_f2c324aa,
            0xc0320000_00000000,
            0x400921ec_dc1271d0,
            0,
        ),
        (
            0xdfa17949_30fa1148,
            0x40a60704_ba79b913,
            0xbff921fb_54442d18,
            0,
        ),
        (
            0x3fc50cbd_2bd07d22,
            0x404bd9ec_a53df5b8,
            0x3f682f6d_d7f43559,
            0,
        ),
        (
            0x3fcdedcd_9eab419f,
            0xc0545b53_7085c2fc,
            0x40091c19_d397a71b,
            0,
        ),
        (
            0x3ffd11d1_995c20b2,
            0xc03c6fc7_992a1150,
            0x40089f4f_6bee04c0,
            0,
        ),
        (
            0x40044eae_7183eb4c,
            0xc0477868_2f0ec258,
            0x4008b356_e94df26b,
            0,
        ),
        (
            0x3fd6e932_f9384d99,
            0xc05199d6_41cbf184,
            0x40091791_7e23daec,
            0,
        ),
        (
            0x3fe255b2_9d05293a,
            0xc04a4254_1f6e9ab8,
            0x40090ba3_b2754407,
            0,
        ),
        (
            0x4012ec1a_00617884,
            0x403bc697_8cbfcd30,
            0x3fc597c5_c8bf89f7,
            0,
        ),
        (
            0x3fe4b938_9da66e3c,
            0x404d7e91_464770e8,
            0x3f867bad_40355ee5,
            0,
        ),
        (
            0x4003e4ff_9240c670,
            0xc0270df3_f5ccaf20,
            0x40076ed4_0f2d9fce,
            0,
        ),
        (
            0x3fb9af52_2002e71b,
            0xc03a146b_8ad43ef0,
            0x40091a1a_59f84d7b,
            0,
        ),
        (
            0x3fe00cd8_9f113aaf,
            0xc05468dc_3c634d14,
            0x40091566_43417905,
            0,
        ),
        (
            0x40283ead_0091581f,
            0x4053b724_88079a14,
            0x3fc385e5_98967ec9,
            0,
        ),
        (
            0x3fe2a40c_31de9b03,
            0xc03445ab_ec7fc910,
            0x4008e725_eddce175,
            0,
        ),
        (
            0x3fdebf7a_2313e6b0,
            0x404e0134_4d55ebc8,
            0x3f80655f_8c537b91,
            0,
        ),
        (
            0x40299dd6_0101b3f1,
            0xc01da011_3ba45740,
            0x4000c2b1_c6353e19,
            0,
        ),
        (
            0x3fce0068_d435e91b,
            0xc0178e97_c71bc4c0,
            0x4008d084_17415f59,
            0,
        ),
        (
            0x3fec6954_89e5a4ce,
            0x40404a8f_5515f2b8,
            0x3f9be588_636bc0b8,
            0,
        ),
        (
            0x4022dc93_f8569b79,
            0xc033a081_0bb4f1f0,
            0x40058c9f_b54a77c3,
            0,
        ),
        (
            0x3fcb26b9_26cfdd4f,
            0x40508d1c_56f50a2c,
            0x3f6a3f37_79efa988,
            0,
        ),
        (
            0x3ff0efd8_86e1e77f,
            0xc033e0b4_35792ab0,
            0x4008b505_9a361c50,
            0,
        ),
        (
            0x402bcc2b_3fb4761e,
            0xc03abf06_4ebb4c10,
            0x40054c7b_e32eff3e,
            0,
        ),
        (
            0x402ed7ac_1ceb1f7e,
            0xc03460f4_b94e9b70,
            0x4003f34b_e95db799,
            0,
        ),
        (
            0x3fb60dcd_c25f86f2,
            0xc04b5f2f_3416bc68,
            0x40091ec2_46b4fb0c,
            0,
        ),
        (
            0x3fdc41e9_0cf0a0ec,
            0x4051d5e7_42abeef4,
            0x3f795958_1df93d4e,
            0,
        ),
        (
            0xc030f8cd_2ea143fe,
            0x40330000_00000000,
            0xbfe75499_84258903,
            0,
        ),
        (
            0xbfe8b2e6_250c27f8,
            0xc0440000_00000000,
            0xc008fa77_f2319ebb,
            0,
        ),
        (
            0xc025071b_92c21c2d,
            0x40220000_00000000,
            0xbfeb9c3e_a47d9c00,
            0,
        ),
        (
            0xc01a474d_20ea969b,
            0xc04d0000_00000000,
            0xc0083afd_8861fde2,
            0,
        ),
        (
            0xc01ca53c_cd4f9bd3,
            0xc0528000_00000000,
            0xc0085c66_c39a459a,
            0,
        ),
        (
            0xbfda7720_ec7bc96e,
            0xc02e0000_00000000,
            0xc008e989_61a19005,
            0,
        ),
        (
            0x0c8735fb_9628e799,
            0xbfeea01c_c1274500,
            0x400921fb_54442d18,
            0,
        ),
        (
            0x304b3a1d_91cc33da,
            0xbff9bb51_87efb580,
            0x400921fb_54442d18,
            0,
        ),
        (
            0x5f49d6ea_4d65f7c1,
            0xbffcdeaa_0257b880,
            0x3ff921fb_54442d18,
            0,
        ),
        (
            0x42cc6852_f047868b,
            0x3fe51bae_f532a900,
            0x3ff921fb_54442ce9,
            0,
        ),
        (
            0x5b04e8ec_03e9a673,
            0x3fe0986a_45b0e300,
            0x3ff921fb_54442d18,
            0,
        ),
        (
            0x60e6de2b_42bf27a7,
            0xbfe301b3_9e2ac300,
            0x3ff921fb_54442d18,
            0,
        ),
        (
            0x3fefffff_19a2fd7b,
            0xc1baacef_e347f353,
            0x400921fb_53f766d7,
            0,
        ),
        (
            0x3fefffff_2eb06e7c,
            0xc190cd51_7f1e17ed,
            0x400921fb_525c9db6,
            0,
        ),
        (
            0x3ff00000_657c9a37,
            0xc1894a37_f7f3b2bf,
            0x400921fb_51bc542a,
            0,
        ),
        (
            0x3fefffff_32078865,
            0xc1bd59b4_85985d84,
            0x400921fb_53fe660c,
            0,
        ),
        (
            0x3ff00000_78563c6d,
            0x41a5b623_3ee3e38b,
            0x3e3794fe_9cd4137b,
            0,
        ),
    ];

    #[test]
    fn exp_and_atan2_x87_match_the_recorded_results() {
        let (mut e, except, _) = dispatch_engine();
        for &(x, want, error) in EXP_X87_RECORDED {
            except.borrow_mut().clear();
            let got = e.call(0x00ec_ae30, &args![x]).f64().to_bits();
            assert_eq!((got, except_seen(&except)), (want, error), "exp({x:016x})");
        }
        for &(y, x, want, error) in ATAN2_X87_RECORDED {
            except.borrow_mut().clear();
            let got = e.call(0x00ec_9760, &args![y, x]).f64().to_bits();
            assert_eq!(
                (got, except_seen(&except)),
                (want, error),
                "atan2({y:016x}, {x:016x})"
            );
        }
    }

    #[test]
    fn the_dispatch_fills_the_exception_from_the_stack_or_the_registers() {
        let (mut e, _, _) = dispatch_engine();
        // The image holds 0x2694 at 011c2b60; this test takes the report path first.
        e.set_global(MATHERR_SKIP, 0u32);
        let fields: Seen = Rc::default();
        let record = fields.clone();
        e.register_double(EXCEPT87, move |e, a| {
            let at = a[1];
            let mut w = vec![a[0]];
            for i in 0..8 {
                w.push(e.mem.u32(at + 4 * i));
            }
            w.push(u32::from(e.mem.u16(a[2])));
            record.borrow_mut().push(w);
            Ret::default()
        });
        let words = |v: f64| [v.to_bits() as u32, (v.to_bits() >> 32) as u32];
        // atan2(inf, inf): the indefinite NaN, type 1, both arguments copied from the stack.
        let r = e
            .call(0x00ec_9760, &args![f64::INFINITY, f64::NEG_INFINITY])
            .f64();
        assert_eq!(r.to_bits(), 0xfff8_0000_0000_0000);
        let w = fields.borrow()[0].clone();
        assert_eq!(w[..3], [0x10, 1, ATAN2_DESCRIPTOR + 1]);
        assert_eq!(w[3..5], words(f64::INFINITY));
        assert_eq!(w[5..7], words(f64::NEG_INFINITY));
        assert_eq!(w[7..9], words(r));
        assert_eq!(w[9], 0x027f);
        // exp overflow from the register form: type 3, the argument stored by FST first; the
        // result the tail scaled by 2^-1536 (the doubled __87except leaves it).
        let r = e.call(0x00ec_ae6c, &args![1000.0f64]).f64();
        assert!(r > 0.0 && r < 1e-27, "{r:e}");
        let w = fields.borrow()[1].clone();
        assert_eq!(w[..3], [0x14, 3, EXP_DESCRIPTOR + 1]);
        assert_eq!(w[3..5], words(1000.0));
        // With the global at 011c2b60 set the register form stores nothing first.
        e.set_global(MATHERR_SKIP, 1u32);
        let r = e.call(0x00ec_ae6c, &args![-1000.0f64]).f64();
        assert!(r > 1e27 && r.is_finite(), "{r:e}");
        let w = fields.borrow()[2].clone();
        assert_eq!(w[..3], [0x14, 4, EXP_DESCRIPTOR + 1]);
        assert_eq!(w[3..5], [0, 0]);
        // The math-error switch: no report, the saved word back.
        e.set_global(ERROR_SWITCH, 1u32);
        set_x87_control_word(&mut e, 0x007f);
        assert_eq!(e.call(0x00ec_ae30, &args![1000.0f64]).f64(), f64::INFINITY);
        assert_eq!(x87_control_word(&mut e), 0x007f);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
        assert_eq!(fields.borrow().len(), 3);
        // The loader keeps a signalling NaN and the low word unshifted.
        let loaded = dispatch_load(0x7ff0_0000_0000_0123);
        assert_eq!(
            (loaded.sign_exponent(), loaded.significand()),
            (0x7fff, 0x8000_0000_0000_0123)
        );
        assert_eq!(e.call(0x00ed_62b4, &args![2.5f64]).f64(), 2.5);
    }

    #[test]
    fn exp_picks_its_body_and_its_sse2_special_values() {
        let (mut e, _, libm) = engine(true);
        let seen: Seen = Rc::default();
        for body in [EXP_SSE2, EXP_SSE2_REGISTER] {
            let record = seen.clone();
            e.register_double(body, move |_, a| {
                record.borrow_mut().push(a.to_vec());
                f64::from(a.len() as u32).into_ret()
            });
        }
        assert_eq!(e.call(0x00ec_ae30, &args![0.5f64]).f64(), 2.0);
        assert_eq!(e.call(0x00ec_ae6c, &args![0.5f64]).f64(), 2.0);
        assert_eq!(seen.borrow().len(), 2);
        // The core's special paths, with the constants they read at the exe's values.
        for at in [EXP_CEE0, EXP_CEC0, EXP_CEF0, EXP_CF00] {
            e.map(at, 16);
        }
        for (at, bits) in [
            (EXP_D330, 1.0f64.to_bits()),
            (EXP_D350, f64::INFINITY.to_bits()),
            (EXP_D358, 0),
            (EXP_D360, f64::MAX.to_bits()),
            (EXP_D368, f64::MIN_POSITIVE.to_bits()),
        ] {
            put(&mut e, at, bits);
        }
        let cases: [(u64, u64, u32); 8] = [
            (0x3c00_0000_0000_0000, 1.0f64.to_bits(), 0),
            (0x8000_0000_0000_0001, 1.0f64.to_bits(), 0),
            (0x4090_0000_0000_0000, f64::INFINITY.to_bits(), 0xe),
            (0xc090_0000_0000_0000, 0, 0xf),
            (f64::INFINITY.to_bits(), f64::INFINITY.to_bits(), 0),
            (f64::NEG_INFINITY.to_bits(), 0, 0),
            (0x7ff4_0000_0000_0000, 0x7ffc_0000_0000_0000, 0x3ea),
            (0xfff8_0000_0000_0001, 0xfff8_0000_0000_0001, 0x3ea),
        ];
        for (x, want, error) in cases {
            libm.borrow_mut().clear();
            let got = fn_00edbd68(&mut e, f64::from_bits(x)).to_bits();
            let code = libm.borrow().last().map_or(0, |w| w[3]);
            assert_eq!((got, code), (want, error), "exp({x:016x})");
        }
    }

    /// Recorded from the exe: `exp` on its SSE2 path.
    const EXP_SSE2_RECORDED: &[Vector1] = &[
        (0x4078db11_74af9148, 0x63cadec2_05b29a84, 0),
        (0xc0875f8b_f7adec82, 0x00000000_00000000, 0xf),
        (0xc0327689_cd11d073, 0x3e44958c_48191b34, 0),
        (0x4032d0e1_126460c6, 0x41a1b321_41b6265e, 0),
        (0x402bae26_244ed780, 0x412f4743_bd9ffd31, 0),
        (0x406fef59_165fc680, 0x56f7e774_fe6f71df, 0),
        (0xc0788906_4d6bfc29, 0x1c891a75_56b95823, 0),
        (0x3f99f548_ac188cf5, 0x3ff06928_e9bcf95f, 0),
        (0xc086d0ff_ff0b92f0, 0x00000000_00192581, 0xf),
        (0xc00ef049_208f5266, 0x3f956a88_20a0be19, 0),
        (0xc081beec_d78b107f, 0x0cbac783_df1f48a2, 0),
        (0x40732d4f_df22a4b4, 0x5b995e82_332d560e, 0),
        (0xc084aca6_4f9f24e0, 0x044743c7_0ee1884e, 0),
        (0x4086c239_af14cc24, 0x7ff00000_00000000, 0xe),
        (0x4082cc54_6348e902, 0x762ca6a7_9bf2a9bb, 0),
        (0xc07eb529_3144aac0, 0x13a204a9_7fa75909, 0),
        (0x3f0ca721_0c4ae04e, 0x3ff00039_4ea8b89b, 0),
        (0x40676d65_92705ea8, 0x50d4efc5_1f65e4dd, 0),
        (0x40874bb4_27f2800e, 0x7ff00000_00000000, 0xe),
        (0x4022f5cb_1f4642f7, 0x40c99405_5f66da89, 0),
        (0x4025636a_2900d3db, 0x40e5883f_70e6add4, 0),
        (0xc06c37e9_62c73d08, 0x2b93e982_aa746005, 0),
        (0xc076c4b3_9040bb8f, 0x1f15a1c1_3083e681, 0),
        (0xc086a0f6_80a69fad, 0x00000000_27cff5c9, 0xf),
        (0x4085f89e_c37057fe, 0x7f541015_9dfe8b8f, 0),
        (0x40300000_00000000, 0x4160f2eb_d0a80020, 0),
        (0x800fffff_ffffffff, 0x3ff00000_00000000, 0),
        (0x7ff40000_00000000, 0x7ffc0000_00000000, 0x3ea),
        (0x3f79c38f_3a73600c, 0x3ff019d8_589c1909, 0),
        (0xbfec3430_4e883d00, 0x3fda827e_a225be95, 0),
        (0x554ac3bd_4d8d552b, 0x7ff00000_00000000, 0xe),
        (0xc06a5058_4e27fb1e, 0x2cf3a8ca_0232b251, 0),
        (0x3e744577_aaf9f102, 0x3ff00000_144577b8, 0),
        (0xc02e0000_00000000, 0x3e94875c_a227ec38, 0),
        (0x4096fc11_f153cdca, 0x7ff00000_00000000, 0xe),
        (0xc0330000_00000000, 0x3e381056_ff2c5772, 0),
        (0x32784719_8f63f56f, 0x3ff00000_00000000, 0),
        (0x40fca1a5_3e90d1d9, 0x7ff00000_00000000, 0xe),
        (0xbfdda656_afc51300, 0x3fe4228d_c86e8930, 0),
        (0x8676739f_bf6f2f0c, 0x3ff00000_00000000, 0),
        (0xffba8b6e_e8950ca4, 0x00000000_00000000, 0xf),
        (0x3fe750f3_f958da80, 0x400093f4_88f14cf3, 0),
        (0x40350000_00000000, 0x41d3a6e1_fd9eecfd, 0),
        (0xc0000000_00000000, 0x3fc152aa_a3bf81cc, 0),
        (0x7ff40000_00000000, 0x7ffc0000_00000000, 0x3ea),
        (0x32c5f369_1f91e0ac, 0x3ff00000_00000000, 0),
        (0x3ff01e4b_af1036ff, 0x4005e85e_b09383f0, 0),
        (0xbfbf303b_6c85d000, 0x3fec545e_cc82d475, 0),
        (0xc10bfcf6_5aed19f8, 0x00000000_00000000, 0xf),
        (0x41113c61_089a2d7b, 0x7ff00000_00000000, 0xe),
        (0x3fe5620c_c7b6f700, 0x3fff3650_010c8830, 0),
        (0xa24b3e27_cad53f50, 0x3ff00000_00000000, 0),
        (0x25db985e_e59db937, 0x3ff00000_00000000, 0),
        (0x3fd2c2e2_ba943e00, 0x3ff5733f_c95654a9, 0),
        (0x40e2240e_f253e43c, 0x7ff00000_00000000, 0xe),
        (0xc0b006b9_448d76db, 0x00000000_00000000, 0xf),
        (0x3fefffff_fffffffe, 0x4005bf0a_8b145768, 0),
        (0x033da741_a1a6c832, 0x3ff00000_00000000, 0),
        (0x981994a2_cd55157a, 0x3ff00000_00000000, 0),
        (0x3fefffff_fffffe92, 0x4005bf0a_8b145671, 0),
        (0xbfefffdc_8c57d5c7, 0x3fd78b70_4bba25cb, 0),
        (0xc24a0267_9f3300d9, 0x00000000_00000000, 0xf),
        (0x3fefffff_3a375827, 0x4005bf0a_04abe40e, 0),
        (0xbfe222ab_49728740, 0x3fe227f2_e3dc396e, 0),
        (0xbfd34c56_166d4400, 0x3fe7ab7a_b31a6de2, 0),
        (0xbfd7af1d_d415aa00, 0x3fe61a23_3e43a490, 0),
        (0x10528df5_9e6ffb99, 0x3ff00000_00000000, 0),
        (0x3fe2ae07_a74f7ac0, 0x3ffcaf0d_b5b2b867, 0),
        (0x3f57085e_295b8000, 0x3ff005c3_20eac46e, 0),
        (0xbfd02239_bc261880, 0x3fe8de9f_5b9f4082, 0),
        (0x3fe5ccad_7c9196c0, 0x3fff9efe_b15be543, 0),
        (0xbfefffff_ffebc7fa, 0x3fd78b56_363bcf86, 0),
        (0xbfcab083_62fa3f00, 0x3fe9fa32_2622dd67, 0),
        (0x428639c5_e7652593, 0x7ff00000_00000000, 0xe),
        (0x4007de20_12fb488b, 0x4033c18c_494cb377, 0),
    ];

    #[test]
    #[ignore = "needs NV_EXE: the SSE2 tables are the exe's"]
    fn exp_sse2_matches_the_recorded_results_on_the_exes_tables() {
        let Some((mut e, libm)) = exe_engine() else {
            return;
        };
        for &(x, want, error) in EXP_SSE2_RECORDED {
            libm.borrow_mut().clear();
            let got = e.call(0x00ec_ae30, &args![x]).f64().to_bits();
            let code = libm.borrow().last().map_or(0, |w| w[3]);
            assert_eq!((got, code), (want, error), "exp({x:016x})");
        }
    }

    #[test]
    fn the_c_initializer_stores_the_sse2_answer_in_sse2_available() {
        let mut e = Engine::new();
        e.register(GET_SSE2_INFO, |_, _| 1u32.into_ret());
        e.map(SSE2_AVAILABLE_FLAG, 4);
        e.set_global(SSE2_AVAILABLE_FLAG, 7u32);
        assert_eq!(e.call(0x00ed_1989, &args![]).u32(), 0);
        assert_eq!(e.global::<u32>(SSE2_AVAILABLE_FLAG), 1);
        e.register(GET_SSE2_INFO, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ed_1989, &args![]).u32(), 0);
        assert_eq!(e.global::<u32>(SSE2_AVAILABLE_FLAG), 0);
    }

    // ---- log and log10 ------------------------------------------------------------------------

    #[test]
    fn logarithm_x87_bodies_run_fyl2x_bit_for_bit() {
        let (mut e, except, _) = inverse_engine(false);
        put_extended(&mut e, MINUS_INFINITY_EXTENDED, 0xffff, 1 << 63);
        let mut spread = Spread(0x1095);
        for word in [0x027fu16, 0x007f] {
            for _ in 0..100 {
                let x = (spread.next() * 40.0 - 20.0).exp();
                for (entry, register, constant) in [
                    (0x00ec_a920u32, 0x00ec_a960u32, X87Constant::LnTwo),
                    (0x00ec_bc80, 0x00ec_bcc0, X87Constant::Log10Two),
                ] {
                    // The body runs under 010f2a98's word (0x027f).
                    crate::x87::set_control_word(0x027f);
                    let r = x87_fyl2x(x87_fld_constant(constant), X87Ext::load64(x));
                    crate::x87::set_control_word(word);
                    let want = r.store64().to_bits();
                    crate::x87::set_control_word(crate::x87::DEFAULT_CONTROL_WORD);
                    for addr in [entry, register] {
                        set_x87_control_word(&mut e, word);
                        let got = e.call(addr, &args![x]).f64().to_bits();
                        assert_eq!(x87_control_word(&mut e), word);
                        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
                        assert_eq!(got, want, "{addr:08x}({x:e}) under {word:04x}");
                    }
                }
            }
        }
        // A zero: -infinity with type 2; a negative number: the indefinite NaN with type 1;
        // +infinity as it is.
        except.borrow_mut().clear();
        let at = |e: &mut Engine, addr: u32, x: f64| e.call(addr, &args![x]).f64().to_bits();
        assert_eq!(at(&mut e, 0x00ec_a920, 0.0), f64::NEG_INFINITY.to_bits());
        assert_eq!(except.borrow().last().unwrap()[3], 2);
        assert_eq!(at(&mut e, 0x00ec_bc80, -2.0), 0xfff8_0000_0000_0000);
        let w = except.borrow().last().unwrap().clone();
        assert_eq!((w[0], w[3]), (LOG10_OPERATION, 1));
        let count = except.borrow().len();
        assert_eq!(
            at(&mut e, 0x00ec_a960, f64::INFINITY),
            f64::INFINITY.to_bits()
        );
        assert_eq!(except.borrow().len(), count);
    }

    /// Recorded from the exe: `log` (`00eca920`) and `log10` (`00ecbc80`) on their x87 paths
    /// under 0x027f.
    const LOG_X87_RECORDED: &[Vector1] = &[
        (0xc00533d3_01fa917e, 0xfff80000_00000000, 0x1_001a),
        (0x444a7c77_c536ff35, 0x40482a64_74bfa940, 0),
        (0x43608359_728e1991, 0x404313cb_df94cde4, 0),
        (0x6cacc753_47974ef7, 0x407f02ff_17267afc, 0),
        (0x3fefffff_fffffc7d, 0xbd3c1800_0000018b, 0),
        (0x3ff00000_076d108b, 0x3e5db442_251b542c, 0),
        (0xc04842f2_8df2e69e, 0xfff80000_00000000, 0x1_001a),
        (0x42552118_abd56159, 0x403a9e1f_3abbb8c4, 0),
        (0x41e1631b_4ccb046f, 0x40359219_aff76a64, 0),
        (0x32eb1694_893a771a, 0xc0620aeb_876f397c, 0),
        (0x1626a498_16bb45ef, 0xc07cf5e4_2a0c1d60, 0),
        (0x4417ddda_adb69e80, 0x404712e4_c06241d8, 0),
        (0x3fefb885_bbdab137, 0xbf81f2a3_ffa03168, 0),
        (0x40071654_b6d660e0, 0x3ff0f518_cfb24cf4, 0),
        (0x41842371_5d21e678, 0x40318f08_61ae3d50, 0),
        (0x3fefffff_ffffffc7, 0xbcfc8000_00000019, 0),
        (0x3ff00000_026d6b40, 0x3e436b59_fe86e3a0, 0),
        (0x400efd77_2c80e3c0, 0x3ff5aae9_24340a74, 0),
        (0x407c7b37_93c15907, 0x40187cc2_d99228f0, 0),
        (0x3ff00000_00000012, 0x3cf1ffff_fffffff6, 0),
        (0x3ff00000_00000000, 0x00000000_00000000, 0),
        (0xfe73eb80_ccbc3d06, 0xfff80000_00000000, 0x1_001a),
        (0x3fe00000_00000000, 0xbfe62e42_fefa39ef, 0),
        (0xc0370000_00000000, 0xfff80000_00000000, 0x1_001a),
        (0xbfe00000_00000000, 0xfff80000_00000000, 0x1_001a),
        (0x40240000_00000000, 0x40026bb1_bbb55516, 0),
        (0x4114fc23_72b7ed16, 0x40297ee8_15b7c0ae, 0),
        (0x40340000_00000000, 0x4007f742_7b73e391, 0),
        (0x3fca2447_b8557200, 0xbff96a7c_a6ad3d68, 0),
        (0x3fe0fec0_cf181180, 0xbfe43ff8_dd8a31e4, 0),
        (0x83a1b64e_46d86b57, 0xfff80000_00000000, 0x1_001a),
        (0xbff00000_00000000, 0xfff80000_00000000, 0x1_001a),
        (0x414f87e2_2ae03e85, 0x402e780c_06a09210, 0),
        (0xfff80000_00000000, 0xfff80000_00000000, 0x7_001a),
        (0x26670ba6_6d946b5b, 0xc071b21d_f6026e9d, 0),
        (0xa1064f13_fa5b098b, 0xfff80000_00000000, 0x1_001a),
        (0x3ff84a46_b5e45a80, 0x3fdab805_c51e28dc, 0),
        (0xa02d76d0_cc9d495b, 0xfff80000_00000000, 0x1_001a),
        (0x3cbb33c6_eebe48a1, 0xc041c1a8_17980a03, 0),
        (0x3e83d3dd_4bd391f9, 0xc02f74af_34205810, 0),
        (0x38dc82ae_db5cbd00, 0xc0539c3a_e8d00278, 0),
        (0x3fefffff_fffaca3f, 0xbdc4d704_0001b24d, 0),
        (0x3a761bc3_b50e064d, 0xc04e5637_e2be5083, 0),
        (0xc2c1725f_900b3c48, 0xfff80000_00000000, 0x1_001a),
        (0x3ff00000_00000000, 0x00000000_00000000, 0),
        (0x34711b4b_7ac80116, 0xc05fde38_98c79859, 0),
        (0x2242d218_c474bb36, 0xc0749152_18430f9b, 0),
        (0x296062f4_fcefd7f4, 0xc06f4678_e9ee1263, 0),
        (0x423b33f5_4c941e64, 0x40397be9_c2596e3a, 0),
        (0x3fee75ff_cc0c53ff, 0xbfa93ca7_75f9bb46, 0),
        (0x3fe8066d_48135240, 0xbfd25840_f1601026, 0),
        (0xc3508b74_5a73e53a, 0xfff80000_00000000, 0x1_001a),
        (0x3fefffff_ffffffb6, 0xbd028000_00000015, 0),
        (0x32b79a11_ced6f959, 0xc06251de_f548fa4c, 0),
        (0x40a64cd0_70f2f367, 0x401fd394_0b7cf205, 0),
        (0x3feeb321_aef5c200, 0xbfa53d1f_6bc2966f, 0),
        (0x3fba7f47_c2877200, 0xc0022525_f4f41c7f, 0),
        (0xbfe87a19_6a923680, 0xfff80000_00000000, 0x1_001a),
        (0xa1c6953f_c2dee439, 0xfff80000_00000000, 0x1_001a),
        (0xc0c86e04_f92610bf, 0xfff80000_00000000, 0x1_001a),
    ];
    const LOG10_X87_RECORDED: &[Vector1] = &[
        (0xc00533d3_01fa917e, 0xfff80000_00000000, 0x1_001b),
        (0x444a7c77_c536ff35, 0x4034fd6e_a2b1a8a6, 0),
        (0x43608359_728e1991, 0x40309202_faabb6f1, 0),
        (0x6cacc753_47974ef7, 0x406aefb9_85c77dce, 0),
        (0x3fefffff_fffffc7d, 0xbd2866e4_4ed1e6ef, 0),
        (0x3ff00000_076d108b, 0x3e49ccf9_936b191a, 0),
        (0xc04842f2_8df2e69e, 0xfff80000_00000000, 0x1_001b),
        (0x42552118_abd56159, 0x40271eab_d10b234e, 0),
        (0x41e1631b_4ccb046f, 0x4022bc6f_7741e06f, 0),
        (0x32eb1694_893a771a, 0xc04f57e2_cc9ed6af, 0),
        (0x1626a498_16bb45ef, 0xc069279f_c72e8849, 0),
        (0x4417ddda_adb69e80, 0x40340aa9_9d570c11, 0),
        (0x3fefb885_bbdab137, 0xbf6f2db5_51206967, 0),
        (0x40071654_b6d660e0, 0x3fdd7541_fbe1ae25, 0),
        (0x41842371_5d21e678, 0x401e80ab_f1f4331f, 0),
        (0x3fefffff_ffffffc7, 0xbce8c139_9ed6a417, 0),
        (0x3ff00000_026d6b40, 0x3e30de0f_a3206fa4, 0),
        (0x400efd77_2c80e3c0, 0x3fe2d1fc_44cc3529, 0),
        (0x407c7b37_93c15907, 0x400544fa_0b642156, 0),
        (0x3ff00000_00000012, 0x3cdf44ea_77cbc19e, 0),
        (0x3ff00000_00000000, 0x00000000_00000000, 0),
        (0xfe73eb80_ccbc3d06, 0xfff80000_00000000, 0x1_001b),
        (0x3fe00000_00000000, 0xbfd34413_509f79ff, 0),
        (0xc0370000_00000000, 0xfff80000_00000000, 0x1_001b),
        (0xbfe00000_00000000, 0xfff80000_00000000, 0x1_001b),
        (0x40240000_00000000, 0x3ff00000_00000000, 0),
        (0x4114fc23_72b7ed16, 0x40162532_f343293d, 0),
        (0x40340000_00000000, 0x3ff4d104_d427de80, 0),
        (0x3fca2447_b8557200, 0xbfe61376_757f6da6, 0),
        (0x3fe0fec0_cf181180, 0xbfd196bd_b0f12a8c, 0),
        (0x83a1b64e_46d86b57, 0xfff80000_00000000, 0x1_001b),
        (0xbff00000_00000000, 0xfff80000_00000000, 0x1_001b),
        (0x414f87e2_2ae03e85, 0x401a7708_e3738fe1, 0),
        (0xfff80000_00000000, 0xfff80000_00000000, 0x7_001b),
        (0x26670ba6_6d946b5b, 0xc05ebd9e_8bf07865, 0),
        (0xa1064f13_fa5b098b, 0xfff80000_00000000, 0x1_001b),
        (0x3ff84a46_b5e45a80, 0x3fc7352b_07eb550a, 0),
        (0xa02d76d0_cc9d495b, 0xfff80000_00000000, 0x1_001b),
        (0x3cbb33c6_eebe48a1, 0xc02ed89d_44b17606, 0),
        (0x3e83d3dd_4bd391f9, 0xc01b5279_08ba368c, 0),
        (0x38dc82ae_db5cbd00, 0xc0410884_379d3512, 0),
        (0x3fefffff_fffaca3f, 0xbdb219ef_899f485b, 0),
        (0x3a761bc3_b50e064d, 0xc03a59a6_c94b3eff, 0),
        (0xc2c1725f_900b3c48, 0xfff80000_00000000, 0x1_001b),
        (0x3ff00000_00000000, 0x00000000_00000000, 0),
        (0x34711b4b_7ac80116, 0xc04bae24_0b2bb71c, 0),
        (0x2242d218_c474bb36, 0xc061dd66_416e6982, 0),
        (0x296062f4_fcefd7f4, 0xc05b2a55_5ff9e3da, 0),
        (0x423b33f5_4c941e64, 0x40262299_54072a53, 0),
        (0x3fee75ff_cc0c53ff, 0xbf95eba7_23b14649, 0),
        (0x3fe8066d_48135240, 0xbfbfde3a_6dfc3b15, 0),
        (0xc3508b74_5a73e53a, 0xfff80000_00000000, 0x1_001b),
        (0x3fefffff_ffffffb6, 0xbcf011a3_283a7c7f, 0),
        (0x32b79a11_ced6f959, 0xc04fd323_e84d2f41, 0),
        (0x40a64cd0_70f2f367, 0x400ba4e5_8724e134, 0),
        (0x3feeb321_aef5c200, 0xbf92729f_f33a7cc7, 0),
        (0x3fba7f47_c2877200, 0xbfef8572_f4807224, 0),
        (0xbfe87a19_6a923680, 0xfff80000_00000000, 0x1_001b),
        (0xa1c6953f_c2dee439, 0xfff80000_00000000, 0x1_001b),
        (0xc0c86e04_f92610bf, 0xfff80000_00000000, 0x1_001b),
    ];

    #[test]
    fn logarithm_x87_matches_the_recorded_results() {
        let (mut e, except, _) = inverse_engine(false);
        put_extended(&mut e, MINUS_INFINITY_EXTENDED, 0xffff, 1 << 63);
        for (addr, vectors) in [
            (0x00ec_a920u32, LOG_X87_RECORDED),
            (0x00ec_bc80, LOG10_X87_RECORDED),
        ] {
            for &(x, want, error) in vectors {
                except.borrow_mut().clear();
                let got = e.call(addr, &args![x]).f64().to_bits();
                assert_eq!(
                    (got, except_seen(&except)),
                    (want, error),
                    "{addr:08x}({x:016x})"
                );
            }
        }
    }

    #[test]
    fn logarithm_sse2_special_values_without_the_tables() {
        let (mut e, _, libm) = engine(true);
        for t in [&LOG_TABLES, &LOG10_TABLES] {
            e.map(t.reciprocals, 0x800);
            e.map(t.logarithms, 0x800);
            for at in [t.index_addend, t.high_mask] {
                e.map(at, 16);
            }
            if let Some(at) = t.high_subtrahend {
                e.map(at, 16);
            }
            for (at, bits) in [
                (t.fraction_mask, 0x000f_ffff_ffff_ffffu64),
                (t.fraction_mask + 8, 0x000f_ffff_ffff_ffff),
                (t.exponent_bits, 1.0f64.to_bits()),
                (t.exponent_bits + 8, 1.0f64.to_bits()),
                (t.zero, 0),
                (t.zero + 8, 0),
                (t.denormal_scale, 0x4330_0000_0000_0000),
                (t.minus_infinity, f64::NEG_INFINITY.to_bits()),
                (t.infinity, f64::INFINITY.to_bits()),
            ] {
                put(&mut e, at, bits);
            }
        }
        let cases: [(u32, u64, u64, u32); 10] = [
            (0x00ec_a920, 0, f64::NEG_INFINITY.to_bits(), 2),
            (
                0x00ec_a920,
                0x8000_0000_0000_0000,
                f64::NEG_INFINITY.to_bits(),
                2,
            ),
            (0x00ec_a920, (-1.0f64).to_bits(), 0xfff8_0000_0000_0000, 3),
            (
                0x00ec_a920,
                f64::NEG_INFINITY.to_bits(),
                0xfff8_0000_0000_0000,
                3,
            ),
            (
                0x00ec_a920,
                f64::INFINITY.to_bits(),
                f64::INFINITY.to_bits(),
                0,
            ),
            (
                0x00ec_a920,
                0x7ff8_0000_0000_0001,
                0x000f_ffff_ffff_ffff,
                0x3e8,
            ),
            (0x00ec_bc80, 0, f64::NEG_INFINITY.to_bits(), 8),
            (0x00ec_bc80, (-3.0f64).to_bits(), 0xfff8_0000_0000_0000, 9),
            (
                0x00ec_bc80,
                f64::INFINITY.to_bits(),
                f64::INFINITY.to_bits(),
                0,
            ),
            (0x00ec_bc80, 0xfff8_0000_0000_0001, 0, 0x3e9),
        ];
        for (addr, x, want, error) in cases {
            libm.borrow_mut().clear();
            let got = e.call(addr, &args![x]).f64().to_bits();
            let code = libm.borrow().last().map_or(0, |w| w[3]);
            assert_eq!((got, code), (want, error), "{addr:08x}({x:016x})");
        }
    }

    /// Recorded from the exe: `log` and `log10` on their SSE2 paths.
    const LOG_SSE2_RECORDED: &[Vector1] = &[
        (0x429c14c8_31fc467e, 0x403dacb8_a5046dfc, 0),
        (0x0ed3b642_5c72cf45, 0xc08104d7_233073b0, 0),
        (0x7a9e95b6_a40954b0, 0x4084568f_4ca2a9f8, 0),
        (0x4226410e_7a940ab0, 0x40389710_24bb2d74, 0),
        (0x6cacc753_47974ef7, 0x407f02ff_17267afc, 0),
        (0xc03f6b77_4bc2bf5c, 0xfff80000_00000000, 0x3),
        (0xc09586c9_b5b947a8, 0xfff80000_00000000, 0x3),
        (0x028a277a_cbdb0c72, 0xc08546fa_61bfe7ab, 0),
        (0x537a54bd_76748ead, 0x406b1852_918de4fc, 0),
        (0x42552118_abd56159, 0x403a9e1f_3abbb8c4, 0),
        (0x7b8dea8f_74e94e73, 0x4084a98f_841decd4, 0),
        (0x3ff00000_00031f09, 0x3dc8f847_fffd9082, 0),
        (0x3e1b3f70_6a9d22ff, 0xc0344314_770c2668, 0),
        (0x5154d125_5467dbae, 0x40681ea8_b9848300, 0),
        (0x4417ddda_adb69e80, 0x404712e4_c06241d8, 0),
        (0x400c7584_e1160060, 0x3ff44ded_2348d843, 0),
        (0x3fd01871_614d9187, 0xbff615e4_36622c80, 0),
        (0x3ff00000_046009a4, 0x3e518026_8d9b7575, 0),
        (0x3ce09f0e_3d859b32, 0xc040f68a_eedc1a4c, 0),
        (0x3fefffff_ffffffc7, 0xbcfc8000_00000019, 0),
        (0x3fefffff_ffff779a, 0xbd910cc0_00002456, 0),
        (0x3ff14ea4_315f027b, 0x3fb41ac4_ee476222, 0),
        (0x3da3e9a3_85e16d12, 0xc0396d7c_610a2648, 0),
        (0x40f6b93b_fc40dec5, 0x4026e1e0_e56de1b0, 0),
        (0x3ff00000_00000012, 0x3cf1ffff_fffffff6, 0),
        (0x4146f244_ed60daa6, 0x402dd554_aa69bb70, 0),
        (0xbfeeb8ea_fb4e7c80, 0xfff80000_00000000, 0x3),
        (0xc03b0000_00000000, 0xfff80000_00000000, 0x3),
        (0xbff00000_00000000, 0xfff80000_00000000, 0x3),
        (0xc0370000_00000000, 0xfff80000_00000000, 0x3),
        (0xc0bc88c1_26a80d67, 0xfff80000_00000000, 0x3),
        (0xd2759412_2651b0fa, 0xfff80000_00000000, 0x3),
        (0x400ea12e_58aa48b4, 0x3ff57afc_4a3df280, 0),
        (0x372378d6_7c06efa1, 0xc0586264_25947bec, 0),
        (0x40340000_00000000, 0x4007f742_7b73e391, 0),
        (0xc110aa43_bb662b8e, 0xfff80000_00000000, 0x3),
        (0x403b0000_00000000, 0x400a5ddf_b8038490, 0),
        (0xc108c24b_3e7cfb0d, 0xfff80000_00000000, 0x3),
        (0x3e329b33_e3bab8f1, 0xc03341d8_9fb05258, 0),
        (0xbff00000_00000000, 0xfff80000_00000000, 0x3),
        (0xc0330000_00000000, 0xfff80000_00000000, 0x3),
        (0x3ebc8415_3fd1b9a6, 0xc02a91f4_8900ebd0, 0),
        (0x40320000_00000000, 0x40071f7b_3a6b9186, 0),
        (0x40390000_00000000, 0x4009c041_f7ed8d33, 0),
        (0xa1064f13_fa5b098b, 0xfff80000_00000000, 0x3),
        (0xfff00000_00000123, 0x00000000_00000000, 0x3e8),
        (0x3ee73e44_efe22492, 0xc026d1f5_8b62cb90, 0),
        (0xc11c9bc9_6dfb8bdf, 0xfff80000_00000000, 0x3),
        (0x3fbfb31f_96b84772, 0xc000b601_94cd6840, 0),
        (0x3e83d3dd_4bd391f9, 0xc02f74af_34205810, 0),
        (0x3fefff8d_c8246b98, 0xbf0c8e29_db5f0499, 0),
        (0xb9222682_67c2bcff, 0xfff80000_00000000, 0x3),
        (0xc02383ac_b5f150d0, 0xfff80000_00000000, 0x3),
        (0xbfefffff_fffffed1, 0xfff80000_00000000, 0x3),
        (0xc2c1725f_900b3c48, 0xfff80000_00000000, 0x3),
        (0x8b23ab69_c3a0033a, 0xfff80000_00000000, 0x3),
        (0x3fe5a9ba_7c73e9c0, 0xbfd8f7e4_2b6568ff, 0),
        (0x420a02ae_1001f29d, 0x40375c19_b62880ff, 0),
        (0xbfc4219c_a328c000, 0xfff80000_00000000, 0x3),
        (0x296062f4_fcefd7f4, 0xc06f4678_e9ee1263, 0),
        (0x2142806c_f8fde0a6, 0xc075430a_36e7e75d, 0),
        (0xaf422795_e738cf4e, 0xfff80000_00000000, 0x3),
        (0x414fc8fe_b4e7a07a, 0x402e7c29_121bce38, 0),
        (0xbfb2d7eb_35e3a800, 0xfff80000_00000000, 0x3),
        (0xc3508b74_5a73e53a, 0xfff80000_00000000, 0x3),
        (0x3fdb70e4_8188b580, 0xbfeb195b_5c3bc992, 0),
        (0x3fb8aed8_9271b000, 0xc002b666_aba39783, 0),
        (0xbfd8d8dd_962f1600, 0xfff80000_00000000, 0x3),
        (0x080da7a7_403368fa, 0xc0835dff_5c786bef, 0),
        (0x3feeb321_aef5c200, 0xbfa53d1f_6bc2966f, 0),
        (0x3fdb1647_cbe15f80, 0xbfeb83b5_eaf493aa, 0),
        (0x1cd53eb9_93ca2740, 0xc078543e_2f30050a, 0),
        (0x3fe184a8_a55d2880, 0xbfe347a4_f917b91c, 0),
        (0x3fc5fc37_3d57dc00, 0xbffc2fc2_582118f8, 0),
        (0xc0c86e04_f92610bf, 0xfff80000_00000000, 0x3),
    ];
    const LOG10_SSE2_RECORDED: &[Vector1] = &[
        (0x429c14c8_31fc467e, 0x4029c66d_a215b5e0, 0),
        (0x0ed3b642_5c72cf45, 0xc06d909b_60a5d1fb, 0),
        (0x7a9e95b6_a40954b0, 0x4071aa5c_3fdfbc4a, 0),
        (0x4226410e_7a940ab0, 0x40255bd2_824e04f1, 0),
        (0x6cacc753_47974ef7, 0x406aefb9_85c77dce, 0),
        (0xc03f6b77_4bc2bf5c, 0xfff80000_00000000, 0x9),
        (0xc09586c9_b5b947a8, 0xfff80000_00000000, 0x9),
        (0x028a277a_cbdb0c72, 0xc0727b2f_5db8dd7e, 0),
        (0x537a54bd_76748ead, 0x405788d0_2e0159d6, 0),
        (0x42552118_abd56159, 0x40271eab_d10b234e, 0),
        (0x7b8dea8f_74e94e73, 0x4071f274_374f64ec, 0),
        (0x3ff00000_00031f09, 0x3db5b043_d393eb42, 0),
        (0x3e1b3f70_6a9d22ff, 0xc0219970_bd80c532, 0),
        (0x5154d125_5467dbae, 0x4054f33d_a0aaa805, 0),
        (0x4417ddda_adb69e80, 0x40340aa9_9d570c11, 0),
        (0x400c7584_e1160060, 0x3fe1a2dc_87109db4, 0),
        (0x3fd01871_614d9187, 0xbfe32ee8_605a3b39, 0),
        (0x3ff00000_046009a4, 0x3e3e66d1_986618b2, 0),
        (0x3ce09f0e_3d859b32, 0xc02d77c4_f382d3e5, 0),
        (0x3fefffff_ffffffc7, 0xbce8c139_9ed6a417, 0),
        (0x3fefffff_ffff779a, 0xbd7d9e58_ec8e6d7d, 0),
        (0x3ff14ea4_315f027b, 0x3fa1766d_4e82c474, 0),
        (0x3da3e9a3_85e16d12, 0xc0261611_4c8cff55, 0),
        (0x40f6b93b_fc40dec5, 0x4013e016_af9ca4f7, 0),
        (0x3ff00000_00000012, 0x3cdf44ea_77cbc19e, 0),
        (0x4146f244_ed60daa6, 0x4019e9b3_8097f65a, 0),
        (0xbfeeb8ea_fb4e7c80, 0xfff80000_00000000, 0x9),
        (0xc03b0000_00000000, 0xfff80000_00000000, 0x9),
        (0xbff00000_00000000, 0xfff80000_00000000, 0x9),
        (0xc0370000_00000000, 0xfff80000_00000000, 0x9),
        (0xc0bc88c1_26a80d67, 0xfff80000_00000000, 0x9),
        (0xd2759412_2651b0fa, 0xfff80000_00000000, 0x9),
        (0x400ea12e_58aa48b4, 0x3fe2a85b_ae07cc53, 0),
        (0x372378d6_7c06efa1, 0xc0452e12_7552d40e, 0),
        (0x40340000_00000000, 0x3ff4d104_d427de80, 0),
        (0xc110aa43_bb662b8e, 0xfff80000_00000000, 0x9),
        (0x403b0000_00000000, 0x3ff6e6dd_b0bbe07e, 0),
        (0xc108c24b_3e7cfb0d, 0xfff80000_00000000, 0x9),
        (0x3e329b33_e3bab8f1, 0xc020ba02_8ec51004, 0),
        (0xbff00000_00000000, 0xfff80000_00000000, 0x9),
        (0xc0330000_00000000, 0xfff80000_00000000, 0x9),
        (0x3ebc8415_3fd1b9a6, 0xc017141a_6d71e768, 0),
        (0x40320000_00000000, 0x3ff41598_9f4fc97e, 0),
        (0x40390000_00000000, 0x3ff65df6_57b04301, 0),
        (0xa1064f13_fa5b098b, 0xfff80000_00000000, 0x9),
        (0xfff00000_00000123, 0x00000000_00000000, 0x3e9),
        (0x3ee73e44_efe22492, 0xc013d242_e164ec10, 0),
        (0xc11c9bc9_6dfb8bdf, 0xfff80000_00000000, 0x9),
        (0x3fbfb31f_96b84772, 0xbfed07a8_6c6e3784, 0),
        (0x3e83d3dd_4bd391f9, 0xc01b5279_08ba368c, 0),
        (0x3fefff8d_c8246b98, 0xbef8cd86_fff0b28b, 0),
        (0xb9222682_67c2bcff, 0xfff80000_00000000, 0x9),
        (0xc02383ac_b5f150d0, 0xfff80000_00000000, 0x9),
        (0xbfefffff_fffffed1, 0xfff80000_00000000, 0x9),
        (0xc2c1725f_900b3c48, 0xfff80000_00000000, 0x9),
        (0x8b23ab69_c3a0033a, 0xfff80000_00000000, 0x9),
        (0x3fe5a9ba_7c73e9c0, 0xbfc5afed_1d66e98c, 0),
        (0x420a02ae_1001f29d, 0x40244a3f_ce416769, 0),
        (0xbfc4219c_a328c000, 0xfff80000_00000000, 0x9),
        (0x296062f4_fcefd7f4, 0xc05b2a55_5ff9e3da, 0),
        (0x2142806c_f8fde0a6, 0xc06277c3_aedd13e7, 0),
        (0xaf422795_e738cf4e, 0xfff80000_00000000, 0x9),
        (0x414fc8fe_b4e7a07a, 0x401a7a9b_8d36bc99, 0),
        (0xbfb2d7eb_35e3a800, 0xfff80000_00000000, 0x9),
        (0xc3508b74_5a73e53a, 0xfff80000_00000000, 0x9),
        (0x3fdb70e4_8188b580, 0xbfd789b6_2cc44beb, 0),
        (0x3fb8aed8_9271b000, 0xbff040e3_b59bf348, 0),
        (0xbfd8d8dd_962f1600, 0xfff80000_00000000, 0x9),
        (0x080da7a7_403368fa, 0xc070d276_4012fc42, 0),
        (0x3feeb321_aef5c200, 0xbf92729f_f33a7cc7, 0),
        (0x3fdb1647_cbe15f80, 0xbfd7e616_dcbbfbf9, 0),
        (0x1cd53eb9_93ca2740, 0xc06521c8_762fbd3f, 0),
        (0x3fe184a8_a55d2880, 0xbfd0bf0b_d8d8c37a, 0),
        (0x3fc5fc37_3d57dc00, 0xbfe87b87_5d6f9bca, 0),
        (0xc0c86e04_f92610bf, 0xfff80000_00000000, 0x9),
    ];

    #[test]
    #[ignore = "needs NV_EXE: the SSE2 tables are the exe's"]
    fn logarithm_sse2_matches_the_recorded_results_on_the_exes_tables() {
        let Some((mut e, libm)) = exe_engine() else {
            return;
        };
        for (addr, vectors) in [
            (0x00ec_a920u32, LOG_SSE2_RECORDED),
            (0x00ec_bc80, LOG10_SSE2_RECORDED),
        ] {
            for &(x, want, error) in vectors {
                libm.borrow_mut().clear();
                let got = e.call(addr, &args![x]).f64().to_bits();
                let code = libm.borrow().last().map_or(0, |w| w[3]);
                assert_eq!((got, code), (want, error), "{addr:08x}({x:016x})");
            }
        }
    }

    /// [`dispatch_engine`] with `fmod`'s descriptor at the exe's values (its handler table read
    /// from the exe at `011c18f0`).
    fn fmod_engine() -> (Engine, Seen) {
        let (mut e, except, _) = dispatch_engine();
        e.map(FMOD_DESCRIPTOR, 0x50);
        e.mem.write(FMOD_DESCRIPTOR, b"\x04fmod");
        e.mem.write(FMOD_DESCRIPTOR + 0xd, &[2, 0x16]);
        for (i, target) in [
            0x00ec_9144u32,
            0x00ed_26f6,
            0x00ed_268f,
            0x00ed_26f6,
            0x00ed_2654,
            0x00ed_26f6,
            0x00ed_268f,
            0x00ed_26f6,
            0x00ed_268d,
            0x00ed_268d,
            0x00ed_26b7,
            0x00ed_268d,
            0x00ed_26f6,
            0x00ed_26f6,
            0x00ed_268f,
            0x00ed_26f6,
        ]
        .into_iter()
        .enumerate()
        {
            e.mem.set_u32(FMOD_DESCRIPTOR + 0x10 + 4 * i as u32, target);
        }
        (e, except)
    }

    #[test]
    fn fmod_runs_fprem_to_completion_and_its_special_handlers() {
        let (mut e, except) = fmod_engine();
        let mut spread = Spread(0x5eed);
        for _ in 0..200 {
            let y = (spread.next() - 0.5) * 1e6;
            let x = (spread.next() - 0.5) * 7.0;
            let got = e.call(0x00ec_9130, &args![y, x]).f64();
            assert_eq!(got.to_bits(), (y % x).to_bits(), "fmod({y:e}, {x:e})");
        }
        // Far apart (several FPREM rounds), and a remainder of zero keeps the dividend's sign.
        for (y, x) in [
            (1e300f64, 3.0f64),
            (-1e300, 7.25),
            (6.0, -3.0),
            (-6.0, 3.0),
            (5.5, 1e10),
        ] {
            let got = e.call(0x00ec_9130, &args![y, x]).f64();
            assert_eq!(got.to_bits(), (y % x).to_bits(), "fmod({y:e}, {x:e})");
        }
        assert!(except.borrow().is_empty());
        // y zero: +0 whatever its sign.
        assert_eq!(
            e.call(0x00ec_9130, &args![-0.0f64, 5.0f64]).f64().to_bits(),
            0
        );
        assert!(except.borrow().is_empty());
        // x zero, x infinite, y infinite: the indefinite NaN, reported.
        for (y, x) in [(5.0f64, 0.0f64), (5.0, f64::INFINITY), (f64::INFINITY, 2.0)] {
            let got = e.call(0x00ec_9130, &args![y, x]).f64();
            assert_eq!(got.to_bits(), 0xfff8_0000_0000_0000, "fmod({y:e}, {x:e})");
        }
        assert_eq!(except.borrow().len(), 3);
        // A quiet NaN goes through.
        let got = e.call(0x00ec_9130, &args![f64::NAN, 2.0f64]).f64();
        assert!(got.is_nan());
    }

    #[test]
    fn fmod_takes_the_flawed_fprem_routine_with_the_pentium_flag() {
        let (mut e, _) = fmod_engine();
        // The workaround's tables (all zero: no divisor marked), then the same answers with the
        // flag at 1 (00ed6cfc) as with FPREM, the large exponent difference over several rounds.
        e.map(0x011c_2540, 0x80);
        for (y, x) in [
            (9.0f64, 4.0f64),
            (-17.5, 0.75),
            (1e300, 3.0),
            (2.5e-300, 7e-310),
        ] {
            e.mem.set_u32(FDIV_FLAWED, 0);
            let want = e.call(0x00ec_9130, &args![y, x]).f64().to_bits();
            e.mem.set_u32(FDIV_FLAWED, 1);
            let got = e.call(0x00ec_9130, &args![y, x]).f64().to_bits();
            assert_eq!(got, want, "fmod({y:e}, {x:e})");
        }
    }

    #[test]
    fn exp_divides_through_the_flawed_routine_with_the_pentium_flag() {
        let (mut e, _, _) = dispatch_engine();
        e.map(0x011c_2540, 0x80);
        for x in [-0.3f64, -2.75, -100.125, 0.4] {
            e.mem.set_u32(FDIV_FLAWED, 0);
            let want = e.call(0x00ec_ae30, &args![x]).f64().to_bits();
            e.mem.set_u32(FDIV_FLAWED, 1);
            let got = e.call(0x00ec_ae30, &args![x]).f64().to_bits();
            assert_eq!(got, want, "exp({x:e})");
        }
    }

    #[test]
    fn the_handlers_register_forms_change_the_stack_as_the_exe() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ed_2648, &args![3.5f64, 9.0f64]).f64(), 3.5);
        for addr in [0x00ed_2654u32, 0x00ed_265b] {
            let got = e.call(addr, &args![3.5f64, 9.0f64]).f64().to_bits();
            let want = if addr == 0x00ed_2654 {
                0
            } else {
                1.0f64.to_bits()
            };
            assert_eq!(got, want, "{addr:08x}");
        }
        assert_eq!(e.call(0x00ed_2656, &args![-3.5f64]).f64().to_bits(), 0);
        assert_eq!(e.call(0x00ed_265d, &args![-3.5f64]).f64(), 1.0);
        assert_eq!(e.call(0x00ed_2713, &args![0x400u32, 2.0f64]).f64(), 2.0);
        assert_eq!(e.call(0x00ed_2713, &args![0x404u32, 2.0f64]).f64(), -2.0);
        put_extended(&mut e, HALF_PI_EXTENDED, 0x3fff, 0xc90f_daa2_2168_c235);
        let got = e.call(0x00ed_7d0c, &args![7.0f64]).f64();
        assert_eq!(got, std::f64::consts::FRAC_PI_2);
        // 00ed9491: 0 not an integer, 1 odd, 2 even (a NaN compares unordered: "even").
        put(&mut e, INTEGER_HALF, 0.5f64.to_bits());
        for (x, want) in [
            (2.5f64, 0u32),
            (3.0, 1),
            (-7.0, 1),
            (4.0, 2),
            (0.0, 2),
            (1e300, 2),
            (f64::NAN, 2),
        ] {
            assert_eq!(e.call(0x00ed_9491, &args![x]).u32(), want, "{x:e}");
        }
    }
}

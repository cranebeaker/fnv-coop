//! The square roots, the x87 error machinery they end in, `floor`, the float-to-integer
//! conversions and the 64-bit integer helpers.
//!
//! The square root's path on the main thread (24-bit precision, the precision exception masked):
//! `_CIsqrt` stores `ST0` and takes its exponent (`00ed2808`), the body loads the caller's precision
//! with every exception masked (`00ed2795`), runs `FSQRT` and jumps to `__math_exit`, which finds the
//! precision exception masked and its flag set and reports error type 8 (inexact) through
//! `__startOneArgErrorHandling` and `__87except`; `__handle_exc` sees the exception masked and
//! answers that it is handled; `_ctrlfp` loads the word back. The precision flag is taken as set: the
//! engine keeps no x87 status word, and the flag is sticky and set by nearly every x87 result.

use super::{low_byte, low_half, low_word, signed, unsigned, words, FAMILY, SSE2_AVAILABLE};
use engine_core::abi::{Ret, RetVal};
use engine_core::typed::leaf;
use engine_core::units::crt::{set_x87_control_word, x87_control_word};
use engine_core::x87::{
    fistp_chop, x87_add, x87_fistp64, x87_fsub, x87_load64, x87_mul, x87_sqrt, X87Ext,
};
use engine_core::{args, typed_call, Engine, Ptr};

/// The control word every x87 path compares with (`CMP word ptr [ESP], 0x27f`).
const PLAIN_CONTROL_WORD: u16 = 0x027f;
/// The math-error switch (`0126fd90`): nonzero returns results without the error handling.
const MATH_ERROR_SWITCH: u32 = 0x0126_fd90;
/// The global `__87except` tests before calling `_matherr`: nonzero skips it.
const MATHERR_SKIP: u32 = 0x011c_2b60;
/// The extended indefinite NaN of a domain error (`011c1d00`).
const INDEFINITE_NAN: u32 = 0x011c_1d00;
/// `"sqrt"`, the name the square root reports.
const SQRT_NAME: u32 = 0x011c_1740;
/// The double 1.0 a signalling NaN gets added to (`010f2a90`).
const NAN_ADDEND: u32 = 0x010f_2a90;
/// The errno slot of a thread without a data block (`011c18c0`).
const ERRNO_FALLBACK: u32 = 0x011c_18c0;
/// `__handle_exc`'s overflow results: +infinity (`011c24f8`) and the largest double (`011c2508`);
/// and the double a lost underflow is multiplied by (`01012060`, 0.0).
const OVERFLOW_INFINITY: u32 = 0x011c_24f8;
const OVERFLOW_LARGEST: u32 = 0x011c_2508;
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `__use_sse2_mathfcns` (`01270a64`).
const USE_SSE2_MATHFCNS: u32 = 0x0127_0a64;
/// `MXCSR` as the game runs with it: every exception masked.
const MXCSR: u32 = 0x1f80;
/// `floor`'s SSE2 constants: 1.0, the shift base `0x433`, -1.0, -0.0 and the exponent mask.
const FLOOR_ONE: u32 = 0x010f_1f30;
const FLOOR_SHIFT_BASE: u32 = 0x010f_1f40;
const FLOOR_MINUS_ONE: u32 = 0x010f_1f50;
const FLOOR_MINUS_ZERO: u32 = 0x010f_1f60;
const FLOOR_EXPONENT_MASK: u32 = 0x010f_1f70;
/// `floor`'s error code for `___libm_error_support`.
const FLOOR_ERROR: u32 = 0x3ed;
/// `fmod`'s descriptor (`011c18e0`).
const FMOD_DESCRIPTOR: u32 = 0x011c_18e0;
/// `EDOM` and `ERANGE`.
const EDOM: u32 = 0x21;
const ERANGE: u32 = 0x22;
/// The sign bit of a double.
const SIGN: u64 = 1 << 63;

/// Callees outside the family: `__raise_exc_ex`, `__getptd_noexit`, `_getptd`, `__cintrindisp2`,
/// `floor`'s x87 body and `___libm_error_support`.
const RAISE_EXC_EX: u32 = 0x00ed_5255;
const GETPTD_NOEXIT: u32 = 0x00ed_1165;
const GETPTD: u32 = 0x00ed_11de;
const CINTRINDISP2: u32 = 0x00ed_6070;
const FLOOR_X87: u32 = 0x00ed_4563;
const LIBM_ERROR_SUPPORT: u32 = 0x00ed_429f;

/// `EAX` and `ST0`, the two registers a helper answers in.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EaxSt0 {
    pub eax: u32,
    pub st0: f64,
}

impl RetVal for EaxSt0 {
    #[inline]
    fn into_ret(self) -> Ret {
        Ret {
            eax: self.eax,
            st0: self.st0,
            ..Ret::default()
        }
    }
    #[inline]
    fn from_ret(r: Ret) -> Self {
        EaxSt0 {
            eax: r.eax,
            st0: r.st0,
        }
    }
}

/// `FLD tbyte` of an extended NaN or infinity, as the engine's double holds it: the sign kept, the
/// significand without its integer bit as the fraction.
fn extended_special(e: &Engine, at: u32) -> f64 {
    let significand = e.mem.u64(at);
    let sign_exponent = e.mem.u16(at + 8);
    assert!(
        sign_exponent & 0x7fff == 0x7fff,
        "{at:08x} is not an extended NaN or infinity"
    );
    let sign = u64::from(sign_exponent >> 15) << 63;
    f64::from_bits(sign | 0x7ff0_0000_0000_0000 | ((significand << 1) >> 12))
}

/// `FLDCW` of the saved word unless it is `0x027f` (`00ed281e`).
#[inline(always)]
fn restore_control_word(e: &mut Engine, saved: u16) {
    if saved != PLAIN_CONTROL_WORD {
        set_x87_control_word(e, saved);
    }
}

// Typed from 00ec6040
/// `_CIsqrt`, `ST0` in and out: `ST0` stored as a double (`FST`), its exponent taken (`00ed2808`,
/// whose `ZF` says all ones), and the shared body with them.
pub fn ci_sqrt(e: &mut Engine, st0: f64) -> f64 {
    let stored = x87_load64(st0.to_bits());
    let (low, high) = words(stored.to_bits());
    typed_call!(
        leaf::FAMILY,
        0x00ed_2808,
        leaf::double_exponent(e, low, high)
    );
    let zf = high & 0x7ff0_0000 == 0x7ff0_0000;
    typed_call!(FAMILY, 0x00ec_605d, sqrt_body(e, zf, stored, low, high))
}

// Typed from 00ec6054
/// `sqrt`, cdecl, the double on the stack: loaded by `__fload_withFB` (`LEA EDX, [ESP+4]`), then the
/// shared body, which the code falls into.
pub fn sqrt(e: &mut Engine, value: f64) -> f64 {
    let bits = value.to_bits();
    let (low, high) = words(bits);
    let st0 = e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), bits);
        let loaded: EaxSt0 = typed_call!(FAMILY, 0x00ed_27c5, fload_with_fb(e, 0u32, slot));
        loaded.st0
    });
    let zf = high & 0x7ff0_0000 == 0x7ff0_0000;
    typed_call!(FAMILY, 0x00ec_605d, sqrt_body(e, zf, st0, low, high))
}

// Typed from 00ec605d
/// The square roots' x87 body: `ZF` (the exponent all ones), `ST0` and the double on the stack in,
/// `ST0` out. With the control word saved: a finite argument loads the caller's precision with every
/// exception masked unless the word is `0x027f` (`00ed2795`); a non-negative one takes `FSQRT`, which
/// rounds to that precision; `-0` stays as it is; another negative one is a domain error (the
/// indefinite NaN, type 1). A NaN gets its type and `ST0` from `00ed27ac`; +infinity stays; -infinity
/// is a domain error. A result goes to [`sqrt_exit`], an error to [`sqrt_error`].
pub fn sqrt_body(e: &mut Engine, zf: bool, st0: f64, low: u32, high: u32) -> f64 {
    let saved = x87_control_word(e);
    if zf {
        if high & 0xf_ffff != 0 || low != 0 {
            let nan: EaxSt0 = typed_call!(FAMILY, 0x00ed_27ac, nan_error_type(e, high, st0));
            return sqrt_error(e, saved, nan.st0, nan.eax, low, high);
        }
        if high & 0x8000_0000 == 0 {
            return sqrt_exit(e, saved, st0, low, high);
        }
    } else {
        if saved != PLAIN_CONTROL_WORD {
            typed_call!(
                leaf::FAMILY,
                0x00ed_2795,
                leaf::load_math_control_word(e, u32::from(saved))
            );
        }
        if high & 0x8000_0000 == 0 {
            return sqrt_exit(e, saved, x87_sqrt(st0), low, high);
        }
        if high & 0x7fff_ffff == 0 && low == 0 {
            return sqrt_exit(e, saved, st0, low, high);
        }
    }
    // FSTP ST0; FLD the indefinite NaN; MOV EAX, 1.
    let indefinite = extended_special(e, INDEFINITE_NAN);
    sqrt_error(e, saved, indefinite, 1, low, high)
}

/// `00ec607e`: a result. With the error switch set the word goes back and `ST0` is returned;
/// otherwise `__math_exit` with operation 5 and `"sqrt"`.
fn sqrt_exit(e: &mut Engine, saved: u16, st0: f64, low: u32, high: u32) -> f64 {
    if e.global::<u32>(MATH_ERROR_SWITCH) != 0 {
        restore_control_word(e, saved);
        return st0;
    }
    typed_call!(
        FAMILY,
        0x00ed_282b,
        math_exit(e, 5u32, SQRT_NAME, st0, u32::from(saved), 0u32, low, high)
    )
}

/// `00ec60db`: an error of type `error`. With the error switch set as [`sqrt_exit`]; otherwise
/// `__startOneArgErrorHandling` with operation 5 and `"sqrt"`.
fn sqrt_error(e: &mut Engine, saved: u16, st0: f64, error: u32, low: u32, high: u32) -> f64 {
    if e.global::<u32>(MATH_ERROR_SWITCH) != 0 {
        restore_control_word(e, saved);
        return st0;
    }
    typed_call!(
        FAMILY,
        0x00ed_2737,
        start_one_arg_error_handling(
            e,
            error,
            SQRT_NAME,
            5u32,
            st0,
            u32::from(saved),
            0u32,
            low,
            high
        )
    )
}

// Typed from 00ed27ac
/// The NaN's error type, `EAX` the double's high word and `ST0` the NaN: a quiet one gives 7 and
/// `ST0` as it is; a signalling one 1 and `ST0 + 1.0` (`FADD` of `010f2a90`, which quiets it).
pub fn nan_error_type(e: &mut Engine, high: u32, st0: f64) -> EaxSt0 {
    if high & 0x8_0000 != 0 {
        return EaxSt0 { eax: 7, st0 };
    }
    let addend = f64::from_bits(e.mem.u64(NAN_ADDEND));
    EaxSt0 {
        eax: 1,
        st0: x87_add(st0, addend),
    }
}

// Typed from 00ed27c5
/// `__fload_withFB`, fastcall (`ECX` unused, `EDX` the double's address): `ST0` the double and `EAX`
/// its exponent bits; for an exponent of all ones the whole high word, and the value is built by
/// hand, so a signalling NaN stays as it is.
pub fn fload_with_fb(e: &mut Engine, _unused: u32, at: Ptr) -> EaxSt0 {
    let bits = e.mem.u64(at.addr());
    let high = e.mem.u32(at.addr().wrapping_add(4));
    let exponent = high & 0x7ff0_0000;
    EaxSt0 {
        eax: if exponent == 0x7ff0_0000 {
            high
        } else {
            exponent
        },
        st0: f64::from_bits(bits),
    }
}

// Typed from 00ed282b
/// `__math_exit`: `EDX` the operation, `ECX` the name, `ST0` the result, then the stack: the saved
/// control word, the body's return address and the argument. A word of `0x027f` returns `ST0`; a
/// word with the precision exception unmasked is loaded back and `ST0` returned; otherwise (the flag
/// taken as set) the result is reported as inexact, type 8.
#[allow(clippy::too_many_arguments)]
pub fn math_exit(
    e: &mut Engine,
    operation: u32,
    name: u32,
    st0: f64,
    saved: u32,
    return_address: u32,
    low: u32,
    high: u32,
) -> f64 {
    let word = low_half(saved);
    if word == PLAIN_CONTROL_WORD {
        return st0;
    }
    if word & 0x20 == 0 {
        set_x87_control_word(e, word);
        return st0;
    }
    typed_call!(
        FAMILY,
        0x00ed_2737,
        start_one_arg_error_handling(
            e,
            8u32,
            name,
            operation,
            st0,
            saved,
            return_address,
            low,
            high
        )
    )
}

// Typed from 00ed2737
/// `__startOneArgErrorHandling`: `EAX` the type, `ECX` the name, `EDX` the operation, `ST0` the
/// result, then the stack as [`math_exit`] has it. An `_exception` in the frame (type, name, the
/// argument, the result stored as a double), `__87except(operation, &exception, &saved word)`, its
/// result loaded, and the saved word loaded back unless `0x027f`.
#[allow(clippy::too_many_arguments)]
pub fn start_one_arg_error_handling(
    e: &mut Engine,
    error: u32,
    name: u32,
    operation: u32,
    st0: f64,
    saved: u32,
    _return_address: u32,
    low: u32,
    high: u32,
) -> f64 {
    e.with_stack(0x24, |e, block| {
        let exception = block.addr();
        let saved_at = exception + 0x20;
        e.mem.set_u32(saved_at, saved);
        e.mem.set_u32(exception, error);
        e.mem
            .set_u64(exception + 0x18, x87_load64(st0.to_bits()).to_bits());
        e.mem.set_u32(exception + 4, name);
        e.mem.set_u32(exception + 8, low);
        e.mem.set_u32(exception + 0xc, high);
        typed_call!(
            FAMILY,
            0x00ee_1b04,
            except_87(e, operation, Ptr::new(exception), Ptr::new(saved_at))
        );
        let result = x87_load64(e.mem.u64(exception + 0x18));
        restore_control_word(e, e.mem.u16(saved_at));
        result
    })
}

// Typed from 00ee1b04
/// `__87except`, cdecl: (operation, `_exception`, pointer to the control word). The type gives the
/// exception flags: 1 and 5 invalid (8), 2 zero divide (4), 3 overflow and inexact (0x11), 4
/// underflow and inexact (0x12), 8 inexact (0x10); 7 becomes 1 and, like any other type, has none.
/// With flags `__handle_exc(flags, &result, word)`; unhandled, the frame's block is set up for the
/// operation and `__raise_exc` runs. Then `_ctrlfp(word, 0xffff)`; unless the type is 8 or the global
/// `011c2b60` is set the default `_matherr` sees the `_exception`; when it answers 0 (or is skipped)
/// `__set_errno_from_matherr(type)`.
pub fn except_87(e: &mut Engine, operation: u32, exception: Ptr, control: Ptr) {
    let at = exception.addr();
    // The frame: the word, the flags, the 0x80-byte block.
    e.with_stack(0x88, |e, frame| {
        let word_at = frame.addr();
        let block = word_at + 8;
        let word = u32::from(e.mem.u16(control.addr()));
        e.mem.set_u32(word_at, word);
        let flags = match e.mem.u32(at) {
            1 | 5 => Some(8u32),
            2 => Some(4),
            3 => Some(0x11),
            4 => Some(0x12),
            7 => {
                e.mem.set_u32(at, 1);
                None
            }
            8 => Some(0x10),
            _ => None,
        };
        if let Some(flags) = flags {
            e.mem.set_u32(word_at + 4, flags);
            let result = at.wrapping_add(0x18);
            let handled: i32 = typed_call!(
                FAMILY,
                0x00ed_5554,
                handle_exc(e, flags, Ptr::new(result), word)
            );
            if handled == 0 {
                let field = e.mem.u32(block + 0x40);
                if operation == 0x10 || operation == 0x16 || operation == 0x1d {
                    // The second argument (FLD; FSTP) and bits 0-1 set, 2-4 clear.
                    let second = x87_load64(e.mem.u64(at.wrapping_add(0x10))).to_bits();
                    e.mem.set_u64(block + 0x30, second);
                    e.mem.set_u32(block + 0x40, (field & 0xffff_ffe3) | 3);
                } else {
                    e.mem.set_u32(block + 0x40, field & !1);
                }
                let _: u32 = typed_call!(
                    FAMILY,
                    0x00ed_5531,
                    raise_exc(
                        e,
                        block,
                        word_at,
                        flags,
                        operation,
                        at.wrapping_add(8),
                        result
                    )
                );
            }
        }
        let word = e.mem.u32(word_at);
        typed_call!(leaf::FAMILY, 0x00ed_5abb, leaf::ctrlfp(e, word, 0xffffu32));
        let kind = e.mem.u32(at);
        let mut set_errno = true;
        if kind != 8 && e.global::<u32>(MATHERR_SKIP) == 0 {
            let answer: u32 = typed_call!(FAMILY, 0x00ee_0a3a, default_matherr(e, exception));
            set_errno = answer == 0;
        }
        if set_errno {
            let kind = e.mem.i32(at);
            typed_call!(FAMILY, 0x00ed_5736, set_errno_from_matherr(e, kind));
        }
    })
}

/// `FLD` of a double, `FCHS` when `negate`, `FSTP` into `at`.
fn store_double(e: &mut Engine, at: u32, bits: u64, negate: bool) {
    let loaded = x87_load64(bits).to_bits();
    e.mem
        .set_u64(at, if negate { loaded ^ SIGN } else { loaded });
}

// Typed from 00ed5554
/// `__handle_exc`, cdecl: (flags, pointer to the result, control word); 1 when every flag of
/// `flags & 0x1f` was handled by a masked exception. The first that applies of: invalid (8) masked;
/// zero divide (4) masked; overflow (1) masked, the result becoming the rounding mode's overflow
/// value with the result's sign; underflow (2) masked, the result denormalised (or zero, inexact,
/// below the range). Each raises its status flag through `00ed5ae4`. Then inexact (0x10) masked.
pub fn handle_exc(e: &mut Engine, flags: u32, result: Ptr, control: u32) -> i32 {
    let at = result.addr();
    let mut remaining = flags & 0x1f;
    let saved = remaining;
    if flags & 8 != 0 && control & 1 != 0 {
        typed_call!(leaf::FAMILY, 0x00ed_5ae4, leaf::raise_status_flags(e, 1u32));
        remaining &= !8;
    } else if flags & 4 != 0 && control & 4 != 0 {
        typed_call!(leaf::FAMILY, 0x00ed_5ae4, leaf::raise_status_flags(e, 4u32));
        remaining &= !4;
    } else if flags & 1 != 0 && control & 8 != 0 {
        typed_call!(leaf::FAMILY, 0x00ed_5ae4, leaf::raise_status_flags(e, 8u32));
        // FLDZ; FCOMP; TEST AH, 5: parity odd exactly when 0 < result.
        let positive = 0.0 < f64::from_bits(e.mem.u64(at));
        let infinity = e.mem.u64(OVERFLOW_INFINITY);
        let largest = e.mem.u64(OVERFLOW_LARGEST);
        match control & 0xc00 {
            0 => store_double(e, at, infinity, !positive),
            0x400 if positive => store_double(e, at, largest, false),
            0x400 => store_double(e, at, infinity, true),
            0x800 if positive => store_double(e, at, infinity, false),
            0x800 => store_double(e, at, largest, true),
            _ => store_double(e, at, largest, !positive),
        }
        remaining &= !1;
    } else if flags & 2 != 0 && control & 0x10 != 0 {
        let mut inexact = flags & 0x10 != 0;
        let value = e.mem.u64(at);
        if 0.0 == f64::from_bits(value) {
            inexact = true;
        } else {
            let copy = f64::from_bits(x87_load64(value).to_bits());
            let (mantissa, exponent) = e.with_stack(4, |e, slot| {
                let mantissa: f64 = typed_call!(FAMILY, 0x00ed_59e1, decomp(e, copy, slot));
                (mantissa.to_bits(), e.mem.i32(slot.addr()))
            });
            let mut bits = mantissa;
            let biased = exponent.wrapping_add(-0x600);
            if biased < -0x432 {
                inexact = true;
                let zero = f64::from_bits(e.mem.u64(ZERO_DOUBLE));
                bits = x87_mul(x87_load64(bits), zero).to_bits();
            } else {
                let negative = 0.0 > f64::from_bits(bits);
                // The top word keeps its low four bits, with the lowest exponent bit set.
                let top = ((bits >> 48) & 0xf) | 0x10;
                bits = (bits & 0x0000_ffff_ffff_ffff) | (top << 48);
                if biased < -0x3fd {
                    let mut count = (-0x3fd_i32).wrapping_sub(biased);
                    loop {
                        if bits & 1 != 0 {
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
                    bits = x87_load64(bits).to_bits() ^ SIGN;
                }
            }
            store_double(e, at, bits, false);
        }
        if inexact {
            typed_call!(
                leaf::FAMILY,
                0x00ed_5ae4,
                leaf::raise_status_flags(e, 0x10u32)
            );
        }
        remaining = saved & !2;
    }
    if flags & 0x10 != 0 && control & 0x20 != 0 {
        typed_call!(
            leaf::FAMILY,
            0x00ed_5ae4,
            leaf::raise_status_flags(e, 0x20u32)
        );
        remaining &= !0x10;
    }
    i32::from(remaining == 0)
}

// Typed from 00ed5531
/// `__raise_exc`, cdecl, six words: `__raise_exc_ex` with them and a seventh of 0.
pub fn raise_exc(e: &mut Engine, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    e.call(RAISE_EXC_EX, &args![a0, a1, a2, a3, a4, a5, 0u32])
        .u32()
}

// Typed from 00ed5763
/// `__errcode`, cdecl, the flags' low byte: the `_exception` type, 5 for 0x20, else 1 for 8, 2 for 4,
/// 3 for 1, else 4 for 2 and 0 for none.
pub fn errcode(_e: &mut Engine, flags: u32) -> i32 {
    let low = low_byte(flags);
    if low & 0x20 != 0 {
        5
    } else if low & 8 != 0 {
        1
    } else if low & 4 != 0 {
        2
    } else if low & 1 != 0 {
        3
    } else {
        i32::from(low & 2) * 2
    }
}

// Typed from 00ed5736
/// `__set_errno_from_matherr`, cdecl, the type (signed): 1 sets `errno` to `EDOM`, 2 and 3 to
/// `ERANGE`.
pub fn set_errno_from_matherr(e: &mut Engine, kind: i32) {
    let code = match kind {
        1 => EDOM,
        2 | 3 => ERANGE,
        _ => return,
    };
    let slot: Ptr = typed_call!(FAMILY, 0x00ec_85e3, errno(e));
    e.mem.set_u32(slot.addr(), code);
}

// Typed from 00ee0a3a
/// The default `_matherr`, cdecl (the `_exception` unread): 0, not handled.
pub fn default_matherr(_e: &mut Engine, _exception: Ptr) -> u32 {
    0
}

// Typed from 00ed59e1
/// A double's mantissa (in `ST0`) and exponent (stored), cdecl: zero gives +0 and 0; a normal value
/// (or an infinity or NaN) `__set_exp(x, 0)` and its exponent field less 0x3fe; a denormal is
/// normalised by shifting, the exponent counting down from -0x3fd, its sign put back.
pub fn decomp(e: &mut Engine, x: f64, exponent: Ptr) -> f64 {
    let bits = x.to_bits();
    if 0.0 == x {
        e.mem.set_i32(exponent.addr(), 0);
        return 0.0;
    }
    let top = low_word(bits >> 48) & 0xffff;
    let (low, high) = words(bits);
    if top & 0x7ff0 != 0 || (high & 0xf_ffff == 0 && low == 0) {
        let copy = f64::from_bits(x87_load64(bits).to_bits());
        let mantissa: f64 = typed_call!(FAMILY, 0x00ed_5955, set_exp(e, copy, 0i32));
        let field = signed((top >> 4) & 0x7ff);
        e.mem.set_i32(exponent.addr(), field - 0x3fe);
        return mantissa;
    }
    let negative = 0.0 > x;
    let mut count = -0x3fd_i32;
    let mut shifted = bits;
    while shifted & (1 << 52) == 0 {
        shifted <<= 1;
        count -= 1;
    }
    shifted &= !(1 << 52);
    if negative {
        shifted |= SIGN;
    }
    let copy = f64::from_bits(x87_load64(shifted).to_bits());
    let mantissa: f64 = typed_call!(FAMILY, 0x00ed_5955, set_exp(e, copy, 0i32));
    e.mem.set_i32(exponent.addr(), count);
    mantissa
}

// Typed from 00ed5955
/// `__set_exp`, cdecl: (the double, an exponent), the result in `ST0`: the double as `FLD`/`FSTP`
/// copy it, its top word replaced by `(exponent + 0x3fe) << 4` with the argument's own sign and top
/// fraction bits (`& 0x800f`).
pub fn set_exp(_e: &mut Engine, x: f64, exponent: i32) -> f64 {
    let bits = x.to_bits();
    let copy = x87_load64(bits).to_bits();
    let field = low_word(bits >> 48) & 0x800f;
    let top = (unsigned(exponent.wrapping_add(0x3fe)) << 4) | field;
    let replaced = (copy & 0x0000_ffff_ffff_ffff) | (u64::from(low_half(top)) << 48);
    x87_load64(replaced)
}

// Typed from 00ec85e3
/// `_errno`, cdecl: the address of the thread's `errno` (`+8` of its data block from
/// `__getptd_noexit`), or the static slot `011c18c0` without a block.
pub fn errno(e: &mut Engine) -> Ptr {
    let block = e.call(GETPTD_NOEXIT, &args![]).u32();
    if block == 0 {
        Ptr::new(ERRNO_FALLBACK)
    } else {
        Ptr::new(block.wrapping_add(8))
    }
}

// Typed from 00ec62c0
/// `_ftol2_sse`, `ST0` in: without SSE2 the x87 body (`00ec62f6`) and its `EDX:EAX`; with it
/// `CVTTSD2SI` of the value stored as a double (truncated; 0x80000000 for a NaN or out of range),
/// `EDX` not written (0 here).
pub fn ftol2_sse(e: &mut Engine, value: f64) -> u64 {
    if e.mem.u32(SSE2_AVAILABLE) == 0 {
        return typed_call!(FAMILY, 0x00ec_62f6, ftol2_x87(e, value));
    }
    let integer = fistp_chop(value);
    u64::from(unsigned(integer))
}

/// `FILD qword`: a 64-bit integer as an extended value (exact).
fn fild64(n: i64) -> X87Ext {
    if n == 0 {
        return X87Ext::from_parts(0, 0);
    }
    let sign: u16 = if n < 0 { 0x8000 } else { 0 };
    let magnitude = n.unsigned_abs();
    let zeros = magnitude.leading_zeros();
    // 0x3fff + 63 - zeros is at most 0x403e: it fits.
    X87Ext::from_parts(sign | low_half(0x3fff + 63 - zeros), magnitude << zeros)
}

// Typed from 00ec62f6
/// `_ftol2`'s x87 body, `ST0` in, `EDX:EAX` out: the value stored as a float (its sign read) and as
/// an integer rounded by the control word (`FISTP qword`). Zero or the integer indefinite is the
/// answer as it is; otherwise the integer is corrected by one toward zero when rounding went away
/// from zero (the difference's sign against the value's): the value truncated.
pub fn ftol2_x87(_e: &mut Engine, value: f64) -> u64 {
    let st0 = X87Ext::load64(value);
    let float_bits = st0.store32();
    let integer = x87_fistp64(st0);
    let bits = u64::from_ne_bytes(integer.to_ne_bytes());
    let (low, high) = words(bits);
    let mut sign = float_bits;
    if low == 0 {
        sign = high;
        if high & 0x7fff_ffff == 0 {
            return bits;
        }
    }
    let difference = x87_fsub(st0, fild64(integer)).store32();
    if signed(sign) < 0 {
        let carry = (difference ^ 0x8000_0000)
            .checked_add(0x7fff_ffff)
            .is_none();
        bits.wrapping_add(u64::from(carry))
    } else {
        let borrow = difference.checked_add(0x7fff_ffff).is_none();
        bits.wrapping_sub(u64::from(borrow))
    }
}

/// Whether the SSE2 math bodies run: `__use_sse2_mathfcns` set, `MXCSR` and the x87 control word
/// with every exception masked (the entries' three tests, in order).
fn sse2_math_available(e: &mut Engine) -> bool {
    e.global::<u32>(USE_SSE2_MATHFCNS) != 0
        && MXCSR & 0x1f80 == 0x1f80
        && x87_control_word(e) & 0x7f == 0x7f
}

// Typed from 00ec6940
/// `floor`, cdecl, the double on the stack, `ST0` out. Without the SSE2 bodies the x87 body
/// (`00ed4563`). Otherwise integer shifts on the bits, by `0x433 - exponent`: a positive value below
/// 1 gives +0; up to the exponent `0x432` the fraction is cleared; a negative value above -1 gives
/// -1 (-0 stays -0); below that the fraction is cleared and 1.0 taken off when that raised it;
/// beyond, the value as it is, a NaN first reported to `___libm_error_support`.
pub fn floor(e: &mut Engine, x: f64) -> f64 {
    if !sse2_math_available(e) {
        return e.call(FLOOR_X87, &args![x]).f64();
    }
    let bits = x.to_bits();
    let base = e.mem.u64(FLOOR_SHIFT_BASE);
    let shifted = bits >> 52;
    let sign_and_exponent = signed(low_word(shifted));
    let exponent = shifted & e.mem.u64(FLOOR_EXPONENT_MASK);
    let (base_low, base_high) = words(base);
    let (exponent_low, exponent_high) = words(exponent);
    let count = u64::from(base_low.wrapping_sub(exponent_low))
        | (u64::from(base_high.wrapping_sub(exponent_high)) << 32);
    // PSRLQ, PSLLQ: a count above 63 clears the lane.
    let truncated = if count > 63 {
        0
    } else {
        (bits >> count) << count
    };
    if sign_and_exponent & 0x800 == 0 {
        if sign_and_exponent < 0x3ff {
            return 0.0;
        }
        if sign_and_exponent > 0x432 {
            return floor_out_of_range(e, x);
        }
        return x87_load64(truncated);
    }
    let below = x < f64::from_bits(truncated);
    if sign_and_exponent < 0xbff {
        let minus_zero = e.mem.u64(FLOOR_MINUS_ZERO);
        let mask = if x < f64::from_bits(minus_zero) {
            u64::MAX
        } else {
            0
        };
        return x87_load64((mask | minus_zero) & e.mem.u64(FLOOR_MINUS_ONE));
    }
    if sign_and_exponent > 0xc32 {
        return floor_out_of_range(e, x);
    }
    let subtrahend = if below { e.mem.u64(FLOOR_ONE) } else { 0 };
    let result = f64::from_bits(truncated) - f64::from_bits(subtrahend);
    x87_load64(result.to_bits())
}

/// `floor`'s exit above the exponent `0x432`: a number as it is (`FLD` of its slot); a NaN given to
/// `___libm_error_support` with its slot for all three pointers and 0x3ed, then the slot loaded.
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

// Typed from 00ec75b1
/// `_isnan`, cdecl, a double as two words: an exponent of all ones with the quiet bit clear is a NaN
/// when any other fraction bit is set; with it set (`0x7ff8`) always.
pub fn isnan(_e: &mut Engine, low: u32, high: u32) -> u32 {
    let top = low_half(high >> 16) & 0x7ff8;
    u32::from((top == 0x7ff0 && (high & 0x7_ffff != 0 || low != 0)) || top == 0x7ff8)
}

// Typed from 00ec7595
/// `_finite`, cdecl, a double as two words: 1 unless its exponent is all ones.
pub fn finite(_e: &mut Engine, _low: u32, high: u32) -> u32 {
    u32::from(low_half(high >> 16) & 0x7ff0 != 0x7ff0)
}

// Typed from 00ec913a
/// `_CIfmod`, `y` in `ST1` and `x` in `ST0`: `fmod`'s descriptor (`011c18e0`) and a jump to
/// `__cintrindisp2`.
pub fn ci_fmod(e: &mut Engine, y: f64, x: f64) -> f64 {
    e.call(CINTRINDISP2, &args![FMOD_DESCRIPTOR, y, x]).f64()
}

// Typed from 00ecc300
/// `_allmul` (callee pops 0x10): the low 64 bits of a 64-bit product, one `MUL` when both high
/// words are zero.
pub fn allmul(_e: &mut Engine, a: u64, b: u64) -> u64 {
    a.wrapping_mul(b)
}

// Typed from 00ecc250
/// `__alldiv` (callee pops 0x10): the signed 64-bit quotient, truncated; a zero divisor faults.
pub fn alldiv(_e: &mut Engine, dividend: u64, divisor: u64) -> u64 {
    let dividend = i64::from_ne_bytes(dividend.to_ne_bytes());
    let divisor = i64::from_ne_bytes(divisor.to_ne_bytes());
    u64::from_ne_bytes(dividend.wrapping_div(divisor).to_ne_bytes())
}

// Typed from 00ecc590
/// `__aulldiv` (stdcall, `RET 0x10`): the unsigned 64-bit quotient. A divisor below 2^32 divides in
/// two `DIV`s; a larger one shifts both until it fits, divides, and corrects the estimate down by one
/// when the divisor times it is above the dividend. A zero divisor faults (#DE).
pub fn aulldiv(
    _e: &mut Engine,
    dividend_low: u32,
    dividend_high: u32,
    divisor_low: u32,
    divisor_high: u32,
) -> u64 {
    if divisor_high == 0 {
        assert!(divisor_low != 0, "00ecc590 __aulldiv: divide by zero (#DE)");
        let dividend = (u64::from(dividend_high) << 32) | u64::from(dividend_low);
        return dividend / u64::from(divisor_low);
    }
    let divisor = (u64::from(divisor_high) << 32) | u64::from(divisor_low);
    let dividend = (u64::from(dividend_high) << 32) | u64::from(dividend_low);
    // Shifted right a bit at a time until the divisor's high word is zero.
    let shift = 32 - divisor_high.leading_zeros();
    let estimate = (dividend >> shift) / (divisor >> shift);
    assert!(
        estimate <= u64::from(u32::MAX),
        "00ecc590 __aulldiv: DIV overflow (#DE)"
    );
    let quotient = low_word(estimate);
    // The divisor times the estimate from two `MUL`s (the high word's product kept to 32 bits):
    // too big when the sum carries or is above the dividend.
    let high_product = low_word(u64::from(divisor_high) * u64::from(quotient));
    let low_product = u64::from(divisor_low) * u64::from(quotient);
    let (sum, carry) = low_word(low_product >> 32).overflowing_add(high_product);
    let too_big = carry
        || sum > dividend_high
        || (sum == dividend_high && low_word(low_product) > dividend_low);
    u64::from(if too_big {
        quotient.wrapping_sub(1)
    } else {
        quotient
    })
}

// Typed from 00ecadb8
/// `rand`: the thread's state (`_getptd` +0x14) becomes `state * 0x343fd + 0x269ec3`, and its bits
/// 16 to 30 are answered.
pub fn rand(e: &mut Engine) -> i32 {
    let ptd = e.call(GETPTD, &args![]).u32();
    let at = ptd.wrapping_add(0x14);
    let state = e.mem.u32(at).wrapping_mul(0x343fd).wrapping_add(0x26_9ec3);
    e.mem.set_u32(at, state);
    signed((state >> 16) & 0x7fff)
}

// Typed from 00ecada6
/// `srand`, cdecl: the seed stored as the thread's state (`_getptd` +0x14).
pub fn srand(e: &mut Engine, seed: u32) {
    let ptd = e.call(GETPTD, &args![]).u32();
    e.mem.set_u32(ptd.wrapping_add(0x14), seed);
}

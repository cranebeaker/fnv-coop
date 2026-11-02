//! Wide-character conversions and `%a` (MSVC 2008 LIBCMT, statically linked into
//! FalloutNV.exe 1.4.0.525), translated from the exe: `_wctomb_s_l` (`00ee21a9`, wide `%C`
//! and `%S`), `__wcstombs_l_helper` (`00ecaa70`), `__cftoa_l` (`00ed1da7`, `%a`; its
//! `_cftoe`, `00ed1d87`, is in `cvt.rs`) and the compiler helpers `__alldvrm` (`00ee1990`)
//! and `__aullshr` (`00ee1a70`) that `%a` uses.

use super::*;

/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`; `ECX` the 0x10-byte object: the locale
/// information, the multibyte information, the thread's data, the "updated" byte), `errno`
/// (`00ec85e3`), the invalid-parameter report (`00ec7c30`), `memset` (`00ec61c0`),
/// `strrchr` (`00ec6e30`) and `_cftoe` (`00ed1d87`).
const LOCALE_UPDATE: u32 = 0x00ec_660e;
const ERRNO: u32 = 0x00ec_85e3;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
const MEMSET: u32 = 0x00ec_61c0;
const STRRCHR: u32 = 0x00ec_6e30;
const CFTOE: u32 = 0x00ed_1d87;
const AULLSHR: u32 = 0x00ee_1a70;
/// The import slots of KERNEL32 `WideCharToMultiByte` (`00fdf170`) and `GetLastError`
/// (`00fdf094`).
const WIDE_CHAR_TO_MULTI_BYTE: u32 = 0x00fd_f170;
const GET_LAST_ERROR: u32 = 0x00fd_f094;
/// `wcslen` (`00ecdb00`).
const WCSLEN: u32 = 0x00ec_db00;

fn report(e: &mut Engine, code: u32) -> u32 {
    let slot = e.call(ERRNO, &args![]).u32();
    e.mem.set_u32(slot, code);
    e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
    code
}

/// `_LocaleUpdate`'s destructor, inlined: with the "updated" byte (+0xc) set, bit 0x2 of the
/// thread's `_ownlocale` (+0x70 of the data at +0x8) is cleared.
fn locale_update_done(e: &mut Engine, update: u32) {
    if e.mem.u8(update.wrapping_add(0xc)) != 0 {
        let ptd = e.mem.u32(update.wrapping_add(8));
        let own = e.mem.u32(ptd.wrapping_add(0x70));
        e.mem.set_u32(ptd.wrapping_add(0x70), own & 0xffff_fffd);
    }
}

// Translated from 00ee21a9 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_wctomb_s_l` (Ghidra's function ID), cdecl: the byte count out (may be null), the target
/// (may be null), its size, the wide character, the locale. No target with a size answers 0
/// with a count of 0. The count starts at -1; a size above 0x7fffffff is `EINVAL`. In a
/// locale without a `LC_CTYPE` handle a character above 0xff empties the target (when there
/// is room) and is `EILSEQ` (`errno` set, no report, `errno` answered); a smaller one is the
/// byte (no room: `ERANGE`, reported), count 1. Otherwise `WideCharToMultiByte` in the
/// locale's code page: a default character used is `EILSEQ`; a failure other than
/// `ERROR_INSUFFICIENT_BUFFER` (0x7a) is `EILSEQ`, that one empties the target and is
/// `ERANGE`; success sets the count. 0 on success.
pub fn wctomb_s_l(
    e: &mut Engine,
    count: u32,
    target: u32,
    size: u32,
    character: u32,
    locale: u32,
) -> u32 {
    if target == 0 && size > 0 {
        if count != 0 {
            e.mem.set_u32(count, 0);
        }
        return 0;
    }
    if count != 0 {
        e.mem.set_u32(count, 0xffff_ffff);
    }
    if size > 0x7fff_ffff {
        return report(e, 0x16);
    }
    // The frame: the _LocaleUpdate object (EBP-0x10), the character's stack word
    // (EBP+0x14) and the "default used" word (EBP+0xc, reused).
    e.with_stack(0x18, |e, frame| {
        let update = frame.addr();
        let wide = update.wrapping_add(0x10);
        let used_default = update.wrapping_add(0x14);
        e.mem.set_u32(wide, character);
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let info = e.mem.u32(update);
        let eilseq = |e: &mut Engine| {
            let slot = e.call(ERRNO, &args![]).u32();
            e.mem.set_u32(slot, 0x2a);
            let slot = e.call(ERRNO, &args![]).u32();
            let code = e.mem.u32(slot);
            locale_update_done(e, update);
            code
        };
        let erange = |e: &mut Engine| {
            let code = report(e, 0x22);
            locale_update_done(e, update);
            code
        };
        let done = |e: &mut Engine, written: Option<u32>| {
            if count != 0 {
                if let Some(n) = written {
                    e.mem.set_u32(count, n);
                }
            }
            locale_update_done(e, update);
            0u32
        };
        if e.mem.u32(info.wrapping_add(0x14)) == 0 {
            let c = character as u16;
            if c > 0xff {
                if target != 0 && size > 0 {
                    e.call(MEMSET, &args![target, 0u32, size]);
                }
                return eilseq(e);
            }
            if target != 0 {
                if size == 0 {
                    return erange(e);
                }
                e.mem.set_u8(target, c as u8);
            }
            return done(e, Some(1));
        }
        e.mem.set_u32(used_default, 0);
        let code_page = e.mem.u32(info.wrapping_add(4));
        let written = e
            .call(
                WIDE_CHAR_TO_MULTI_BYTE,
                &args![
                    code_page,
                    0u32,
                    wide,
                    1u32,
                    target,
                    size,
                    0u32,
                    used_default
                ],
            )
            .u32();
        if written != 0 {
            if e.mem.u32(used_default) != 0 {
                return eilseq(e);
            }
            return done(e, Some(written));
        }
        if e.call(GET_LAST_ERROR, &args![]).u32() != 0x7a {
            return eilseq(e);
        }
        if target != 0 && size > 0 {
            e.call(MEMSET, &args![target, 0u32, size]);
        }
        erange(e)
    })
}

// Translated from 00ee1a70 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__aullshr` (Ghidra's function ID), `EDX:EAX` shifted right (unsigned) by `CL`: 64 or more
/// gives 0, 32..63 shifts the high word into the low one.
pub fn aullshr(_e: &mut Engine, value: u64, count: u32) -> u64 {
    let cl = count as u8;
    if cl >= 0x40 {
        0
    } else if cl >= 0x20 {
        (value >> 32) >> (cl & 0x1f)
    } else {
        value >> cl
    }
}

// Translated from 00ee1990 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__alldvrm` (Ghidra's function ID): signed 64-bit division with remainder, two 64-bit
/// stack arguments (`RET 0x10`). Both are made positive (counting the signs), divided as in
/// `__aulldvrm` (a divisor below 2^32 in two `DIV`s, a larger one by shifting both until it
/// fits and correcting the estimate down by one), the remainder takes the dividend's sign and
/// the quotient is negated for one negative operand: the truncating quotient (`EDX:EAX`) and
/// remainder (`EBX:ECX`, which the uniform form does not carry: callers in this unit use the
/// Rust function). A zero divisor faults (`DIV`), as here.
pub fn alldvrm(_e: &mut Engine, dividend: i64, divisor: i64) -> (i64, i64) {
    (
        dividend.wrapping_div(divisor),
        dividend.wrapping_rem(divisor),
    )
}

/// The digit `__cftoa_l` writes for a nibble: `0`..`9`, then from `a` (or `A`) on (the
/// nibble plus 0x30, plus 0x27 or 7 above `9`; a 16-bit compare).
fn hex_digit(nibble: u64, letter_step: u32) -> u8 {
    let mut digit = u32::from((nibble as u16).wrapping_add(0x30));
    if digit as u16 > 0x39 {
        digit = digit.wrapping_add(letter_step);
    }
    digit as u8
}

// Translated from 00ed1da7 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__cftoa_l` (Ghidra's function ID), cdecl: the double's address, target, size, precision
/// (negative taken as 0), capitals, locale. A null target or a size of 0 is `EINVAL`; the
/// target is emptied and a size not above precision + 11 is `ERANGE` (each set in `errno`,
/// reported and answered). An infinity or NaN is formatted by `_cftoe` (size less 2, or -1
/// kept) two bytes in; its failure empties the target and is answered, otherwise the sign is
/// moved forward, `0x` (`0X`) written over the start and the `e` (`strrchr`) of the exponent
/// becomes `p` (`P`) with the text cut three bytes on. A finite value is `[-]0x` then `1`
/// (or `0` for a denormal or zero, whose exponent bias becomes 0x3fe, or 0 for zero), the
/// locale's decimal point (none for precision 0), the precision's worth of mantissa nibbles
/// from the top (`__aullshr`), rounded up on the next nibble above 8 (trailing `f`s become
/// `0` and carry into the digit before, `9` becoming `a`; reaching the point increments the
/// leading digit), padded with `0`s; the point is dropped when empty; then `p`, the sign and
/// the binary exponent (less the bias) in decimal with 1 to 4 digits (`__alldvrm` by 1000,
/// 100 and 10). 0 on success.
pub fn cftoa_l(
    e: &mut Engine,
    value: u32,
    target: u32,
    size: u32,
    precision: i32,
    caps: u32,
    locale: u32,
) -> u32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let mut precision = precision.max(0);
        let fail = |e: &mut Engine, code: u32| {
            let code = report(e, code);
            locale_update_done(e, update);
            code
        };
        if target == 0 || size == 0 {
            return fail(e, 0x16);
        }
        e.mem.set_u8(target, 0);
        if size <= (precision as u32).wrapping_add(0xb) {
            return fail(e, 0x22);
        }
        let low = e.mem.u32(value);
        let high = e.mem.u32(value.wrapping_add(4));
        let small_x = if caps == 0 { b'x' } else { b'X' };
        let small_p = if caps == 0 { b'p' } else { b'P' };
        let mut at = target;
        if (high >> 20) & 0x7ff == 0x7ff {
            let inner_size = if size == 0xffff_ffff {
                size
            } else {
                size.wrapping_sub(2)
            };
            let inner = target.wrapping_add(2);
            let result = e
                .call(
                    CFTOE,
                    &args![value, inner, inner_size, precision as u32, 0u32],
                )
                .u32();
            if result != 0 {
                e.mem.set_u8(target, 0);
                locale_update_done(e, update);
                return result;
            }
            if e.mem.u8(inner) == b'-' {
                e.mem.set_u8(at, b'-');
                at = at.wrapping_add(1);
            }
            e.mem.set_u8(at, b'0');
            at = at.wrapping_add(1);
            e.mem.set_u8(at, small_x);
            at = at.wrapping_add(1);
            let mark = e.call(STRRCHR, &args![at, u32::from(b'e')]).u32();
            if mark != 0 {
                e.mem.set_u8(mark, small_p);
                e.mem.set_u8(mark.wrapping_add(3), 0);
            }
            locale_update_done(e, update);
            return 0;
        }
        if high & 0x8000_0000 != 0 {
            e.mem.set_u8(at, b'-');
            at = at.wrapping_add(1);
        }
        e.mem.set_u8(at, b'0');
        at = at.wrapping_add(1);
        e.mem.set_u8(at, small_x);
        at = at.wrapping_add(1);
        let letter_step = if caps != 0 { 7u32 } else { 0x27 };
        let mut bias = 0x3ffu32;
        if high & 0x7ff0_0000 == 0 {
            e.mem.set_u8(at, b'0');
            at = at.wrapping_add(1);
            bias = if low | (high & 0xfffff) == 0 {
                0
            } else {
                0x3fe
            };
        } else {
            e.mem.set_u8(at, b'1');
            at = at.wrapping_add(1);
        }
        let point = at;
        at = at.wrapping_add(1);
        if precision == 0 {
            e.mem.set_u8(point, 0);
        } else {
            let info = e.mem.u32(update);
            let decimal = e.mem.u32(info.wrapping_add(0xbc));
            let decimal = e.mem.u32(decimal);
            let c = e.mem.u8(decimal);
            e.mem.set_u8(point, c);
        }
        let bits = u64::from(low) | u64::from(high) << 32;
        if high & 0xfffff != 0 || low != 0 {
            let mut mask: u64 = 0x000f_0000_0000_0000;
            let mut shift: u32 = 0x30;
            let nibble = |e: &mut Engine, mask: u64, shift: u32| {
                let masked = bits & mask & 0x000f_ffff_ffff_ffff;
                let count = shift as u16 as i16 as i32 as u32;
                e.call(AULLSHR, &args![masked as u32, (masked >> 32) as u32, count])
                    .u64()
            };
            loop {
                if precision <= 0 {
                    break;
                }
                let digit = nibble(e, mask, shift);
                shift = shift.wrapping_sub(4);
                e.mem.set_u8(at, hex_digit(digit, letter_step));
                mask >>= 4;
                at = at.wrapping_add(1);
                precision -= 1;
                if (shift as u16 as i16) < 0 {
                    break;
                }
            }
            if (shift as u16 as i16) >= 0 && (nibble(e, mask, shift) as u16) > 8 {
                let mut p = at.wrapping_sub(1);
                loop {
                    let c = e.mem.u8(p);
                    if c != b'f' && c != b'F' {
                        break;
                    }
                    e.mem.set_u8(p, b'0');
                    p = p.wrapping_sub(1);
                }
                if p == point {
                    let lead = p.wrapping_sub(1);
                    let c = e.mem.u8(lead);
                    e.mem.set_u8(lead, c.wrapping_add(1));
                } else {
                    let c = e.mem.u8(p);
                    if c == b'9' {
                        e.mem.set_u8(p, (letter_step as u8).wrapping_add(0x3a));
                    } else {
                        e.mem.set_u8(p, c.wrapping_add(1));
                    }
                }
            }
        }
        if precision > 0 {
            e.call(MEMSET, &args![at, 0x30u32, precision as u32]);
            at = at.wrapping_add(precision as u32);
        }
        if e.mem.u8(point) == 0 {
            at = point;
        }
        e.mem.set_u8(at, small_p);
        at = at.wrapping_add(1);
        let shifted = e.call(AULLSHR, &args![low, high, 0x34u32]).u64();
        let mut exponent = ((shifted as u32 & 0x7ff) as i64).wrapping_sub(i64::from(bias));
        if exponent >= 0 {
            e.mem.set_u8(at, b'+');
        } else {
            e.mem.set_u8(at, b'-');
            exponent = exponent.wrapping_neg();
        }
        at = at.wrapping_add(1);
        let start = at;
        e.mem.set_u8(at, b'0');
        let put = |e: &mut Engine, at: &mut u32, rest: &mut i64, by: i64| {
            let (quotient, remainder) = alldvrm(e, *rest, by);
            e.mem.set_u8(*at, (quotient as u8).wrapping_add(0x30));
            *at = at.wrapping_add(1);
            *rest = remainder;
        };
        if exponent >= 1000 {
            put(e, &mut at, &mut exponent, 1000);
        }
        if at != start || exponent >= 100 {
            put(e, &mut at, &mut exponent, 100);
        }
        if at != start || exponent >= 10 {
            put(e, &mut at, &mut exponent, 10);
        }
        e.mem.set_u8(at, (exponent as u8).wrapping_add(0x30));
        e.mem.set_u8(at.wrapping_add(1), 0);
        locale_update_done(e, update);
        0
    })
}

// Translated from 00ecaa70 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecaa70 - the /GS stack cookie check (00ec408c) is not translated
/// `__wcstombs_l_helper` (Ghidra's function ID), cdecl: target (may be null), wide text,
/// count, locale. A target with a count of 0 answers 0; no text sets `errno` to `EINVAL`,
/// reports and answers -1. With no target the converted length is answered: `wcslen` in a
/// locale without a `LC_CTYPE` handle, else `WideCharToMultiByte` of the whole text less
/// its zero. With a target: without a handle each character above 0xff is `EILSEQ`, the
/// others are stored as bytes up to the count or the zero (not counted); in a single-byte
/// code page (`mb_cur_max` 1) the count is cut to the text's end and converted at once, the
/// answer less a converted zero; otherwise the whole text is converted into the count and,
/// when that is too small (`ERROR_INSUFFICIENT_BUFFER`), character by character into a
/// five-byte buffer, each whole character that still fits copied (stopping at a zero). A
/// default character used, a failed conversion or a character over five bytes is `EILSEQ`
/// (`errno` set, -1). The `_LocaleUpdate`'s destructor runs on every way out after it.
pub fn wcstombs_l_helper(e: &mut Engine, target: u32, text: u32, count: u32, locale: u32) -> u32 {
    if target != 0 && count == 0 {
        return 0;
    }
    if text == 0 {
        report(e, 0x16);
        return 0xffff_ffff;
    }
    // The frame: the _LocaleUpdate object (EBP-0x20), the "default used" word (EBP-0x10)
    // and the character buffer (EBP-0xc).
    e.with_stack(0x20, |e, frame| {
        let update = frame.addr();
        let used_default = update.wrapping_add(0x10);
        let buffer = update.wrapping_add(0x14);
        e.mem.set_u32(used_default, 0);
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let info = e.mem.u32(update);
        let done = |e: &mut Engine, answer: u32| {
            locale_update_done(e, update);
            answer
        };
        let eilseq = |e: &mut Engine| {
            let slot = e.call(ERRNO, &args![]).u32();
            e.mem.set_u32(slot, 0x2a);
            locale_update_done(e, update);
            0xffff_ffff
        };
        let convert = |e: &mut Engine, from: u32, n: u32, to: u32, size: u32| {
            let code_page = e.mem.u32(info.wrapping_add(4));
            e.call(
                WIDE_CHAR_TO_MULTI_BYTE,
                &args![code_page, 0u32, from, n, to, size, 0u32, used_default],
            )
            .u32()
        };
        if target == 0 {
            if e.mem.u32(info.wrapping_add(0x14)) == 0 {
                let len = e.call(WCSLEN, &args![text]).u32();
                return done(e, len);
            }
            let n = convert(e, text, 0xffff_ffff, 0, 0);
            if n == 0 || e.mem.u32(used_default) != 0 {
                return eilseq(e);
            }
            return done(e, n.wrapping_sub(1));
        }
        if e.mem.u32(info.wrapping_add(0x14)) == 0 {
            let mut written = 0u32;
            let mut at = text;
            while written < count {
                if e.mem.u16(at) > 0xff {
                    return eilseq(e);
                }
                let byte = e.mem.u8(at);
                e.mem.set_u8(target.wrapping_add(written), byte);
                let c = e.mem.u16(at);
                at = at.wrapping_add(2);
                if c == 0 {
                    break;
                }
                written += 1;
            }
            return done(e, written);
        }
        if e.mem.u32(info.wrapping_add(0xac)) == 1 {
            let mut count = count;
            let mut left = count;
            let mut at = text;
            if left > 0 {
                while e.mem.u16(at) != 0 {
                    at = at.wrapping_add(2);
                    left -= 1;
                    if left == 0 {
                        break;
                    }
                }
                if left != 0 && e.mem.u16(at) == 0 {
                    count = ((at.wrapping_sub(text) as i32) >> 1).wrapping_add(1) as u32;
                }
            }
            let n = convert(e, text, count, target, count);
            if n == 0 || e.mem.u32(used_default) != 0 {
                return eilseq(e);
            }
            if e.mem.u8(target.wrapping_add(n).wrapping_sub(1)) != 0 {
                return done(e, n);
            }
            return done(e, n.wrapping_sub(1));
        }
        let n = convert(e, text, 0xffff_ffff, target, count);
        if n != 0 {
            if e.mem.u32(used_default) != 0 {
                return eilseq(e);
            }
            return done(e, n.wrapping_sub(1));
        }
        if e.mem.u32(used_default) != 0 || e.call(GET_LAST_ERROR, &args![]).u32() != 0x7a {
            return eilseq(e);
        }
        let mut written = 0u32;
        let mut at = text;
        while written < count {
            let size = e.mem.u32(info.wrapping_add(0xac));
            let k = convert(e, at, 1, buffer, size);
            if k == 0 || e.mem.u32(used_default) != 0 || (k as i32) < 0 || k > 5 {
                return eilseq(e);
            }
            if k.wrapping_add(written) > count {
                break;
            }
            let mut stop = false;
            for i in 0..k {
                let c = e.mem.u8(buffer.wrapping_add(i));
                e.mem.set_u8(target.wrapping_add(written), c);
                if c == 0 {
                    stop = true;
                    break;
                }
                written += 1;
            }
            if stop {
                break;
            }
            at = at.wrapping_add(2);
        }
        done(e, written)
    })
}

/// The registry entries of the helpers that take registers or answer more than `EDX:EAX`.
fn aullshr_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let value = u64::from(words[0]) | u64::from(words[1]) << 32;
    aullshr(e, value, words[2]).into_ret()
}

fn alldvrm_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let dividend = (u64::from(words[0]) | u64::from(words[1]) << 32) as i64;
    let divisor = (u64::from(words[2]) | u64::from(words[3]) << 32) as i64;
    (alldvrm(e, dividend, divisor).0 as u64).into_ret()
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_21a9, wctomb_s_l(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ed_1da7, cftoa_l(u32, u32, u32, i32, u32, u32) -> u32),
        (0x00ee_1a70, aullshr_entry as AbiFn),
        (0x00ee_1990, alldvrm_entry as AbiFn),
        entry!(0x00ec_aa70, wcstombs_l_helper(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine whose `_LocaleUpdate` hands out `info` (marked as updated, with the thread's
    /// data at `ptd`), with `errno` and the report doubled; (engine, info, ptd, errno slot).
    fn engine() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        let info = e.mem.alloc(0xd8);
        let ptd = e.mem.alloc(0x80);
        e.mem.set_u32(ptd + 0x70, 0xff);
        let lconv = e.mem.alloc(8);
        let point = e.mem.alloc(2);
        e.mem.set_cstr(point, b".");
        e.mem.set_u32(lconv, point);
        e.mem.set_u32(info + 0xbc, lconv);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u32(a[0] + 8, ptd);
            e.mem.set_u8(a[0] + 0xc, 1);
            a[0].into_ret()
        });
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| slot.into_ret());
        e.register_double(INVALID_PARAMETER, |_, _| Ret::default());
        e.register_double(STRRCHR, |e, a| {
            let t = e.mem.cstr(a[0]);
            match t.iter().rposition(|&c| u32::from(c) == a[1]) {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
        e.call_log = Some(vec![]);
        (e, info, ptd, slot)
    }

    fn hex(e: &mut Engine, value: f64, precision: i32, caps: u32) -> Vec<u8> {
        let at = e.mem.alloc(8);
        e.mem.set_u64(at, value.to_bits());
        let target = e.mem.alloc(0x40);
        let result = e
            .call(
                0x00ed_1da7,
                &args![at, target, 0x40u32, precision as u32, caps, 0u32],
            )
            .u32();
        assert_eq!(result, 0);
        e.mem.cstr(target)
    }

    #[test]
    fn aullshr_and_alldvrm() {
        let (mut e, ..) = engine();
        let r = e.call(AULLSHR, &args![0x89ab_cdefu32, 0x0123_4567u32, 8u32]);
        assert_eq!(r.u64(), 0x0001_2345_6789_abcd);
        assert_eq!(aullshr(&mut e, 0x0123_4567_89ab_cdef, 0x24), 0x0012_3456);
        assert_eq!(aullshr(&mut e, u64::MAX, 0x40), 0);
        assert_eq!(aullshr(&mut e, u64::MAX, 0x108), 0x00ff_ffff_ffff_ffff);
        assert_eq!(alldvrm(&mut e, -1075, 1000), (-1, -75));
        assert_eq!(alldvrm(&mut e, 1023, -10), (-102, 3));
        let r = e.call(
            0x00ee_1990,
            &args![(-7i32) as u32, 0xffff_ffffu32, 2u32, 0u32],
        );
        assert_eq!(r.u64() as i64, -3);
    }

    #[test]
    fn cftoa_l_writes_hexadecimal_floats() {
        let (mut e, _, ptd, _) = engine();
        assert_eq!(hex(&mut e, 1.0, 3, 0), b"0x1.000p+0".to_vec());
        assert_eq!(hex(&mut e, 0.5, 3, 1), b"0X1.000P-1".to_vec());
        assert_eq!(hex(&mut e, 255.5, 3, 0), b"0x1.ff0p+7".to_vec());
        assert_eq!(hex(&mut e, -2.0, 1, 0), b"-0x1.0p+1".to_vec());
        assert_eq!(hex(&mut e, 1.0, 0, 0), b"0x1p+0".to_vec());
        assert_eq!(hex(&mut e, 1.0, -5, 0), b"0x1p+0".to_vec());
        assert_eq!(hex(&mut e, 0.0, 2, 0), b"0x0.00p+0".to_vec());
        assert_eq!(hex(&mut e, 1.5, 15, 0), b"0x1.800000000000000p+0".to_vec());
        assert_eq!(
            hex(&mut e, f64::from_bits(1), 13, 0),
            b"0x0.0000000000001p-1022".to_vec()
        );
        assert_eq!(
            hex(&mut e, f64::from_bits(0x7e38_0000_0000_0000), 1, 0),
            b"0x1.8p+996".to_vec()
        );
        // Rounding on the next nibble: carries through f's into the leading digit, 9 to a.
        assert_eq!(
            hex(&mut e, f64::from_bits(0x3fff_ffff_ffff_ffff), 2, 0),
            b"0x2.00p+0".to_vec()
        );
        assert_eq!(hex(&mut e, f64::MAX, 1, 0), b"0x2.0p+1023".to_vec());
        assert_eq!(
            hex(&mut e, f64::from_bits(0x3ff1_9000_0000_0000), 1, 0),
            b"0x1.2p+0".to_vec()
        );
        assert_eq!(
            hex(&mut e, f64::from_bits(0x3ff9_9000_0000_0000), 1, 0),
            b"0x1.ap+0".to_vec()
        );
        assert_eq!(
            hex(&mut e, f64::from_bits(0x3ff9_9000_0000_0000), 1, 1),
            b"0X1.AP+0".to_vec()
        );
        // The thread's "locale updated" bit is cleared on the way out.
        assert_eq!(e.mem.u32(ptd + 0x70), 0xfd);
    }

    #[test]
    fn cftoa_l_hands_infinities_to_cftoe() {
        let (mut e, ..) = engine();
        e.register_double(CFTOE, |e, a| {
            let at = e.mem.u32(a[0] + 4);
            let t: &[u8] = if at >> 31 == 1 {
                b"-1.#INF00e+000"
            } else {
                b"1.#INF00e+000"
            };
            assert_eq!(a[2], 0x3e);
            e.mem.set_cstr(a[1], t);
            0u32.into_ret()
        });
        assert_eq!(hex(&mut e, f64::INFINITY, 2, 0), b"0x1.#INF00p+0".to_vec());
        assert_eq!(
            hex(&mut e, f64::NEG_INFINITY, 2, 1),
            b"-0X1.#INF00P+0".to_vec()
        );
        e.register_double(CFTOE, |_, _| 0x22u32.into_ret());
        let at = e.mem.alloc(8);
        e.mem.set_u64(at, f64::NAN.to_bits());
        let target = e.mem.alloc(0x40);
        e.mem.set_u8(target, b'x');
        assert_eq!(cftoa_l(&mut e, at, target, 0x40, 2, 0, 0), 0x22);
        assert_eq!(e.mem.u8(target), 0);
    }

    #[test]
    fn cftoa_l_checks_its_room() {
        let (mut e, _, _, slot) = engine();
        let at = e.mem.alloc(8);
        e.mem.set_u64(at, 1.0f64.to_bits());
        let target = e.mem.alloc(0x40);
        assert_eq!(cftoa_l(&mut e, at, 0, 0x40, 2, 0, 0), 0x16);
        assert_eq!(cftoa_l(&mut e, at, target, 0, 2, 0, 0), 0x16);
        e.mem.set_u8(target, b'x');
        assert_eq!(cftoa_l(&mut e, at, target, 13, 2, 0, 0), 0x22);
        assert_eq!(e.mem.u8(target), 0);
        assert_eq!(e.mem.u32(slot), 0x22);
        assert_eq!(cftoa_l(&mut e, at, target, 14, 2, 0, 0), 0);
    }

    #[test]
    fn wctomb_s_l_in_the_c_locale() {
        let (mut e, _, ptd, slot) = engine();
        let count = e.mem.alloc(4);
        let target = e.mem.alloc(4);
        assert_eq!(
            e.call(0x00ee_21a9, &args![count, target, 4u32, 0x41u32, 0u32])
                .u32(),
            0
        );
        assert_eq!((e.mem.u8(target), e.mem.u32(count)), (0x41, 1));
        assert_eq!(e.mem.u32(ptd + 0x70), 0xfd);
        // Above 0xff: emptied, EILSEQ in errno (not reported).
        e.mem.set_u32(target, 0x0101_0101);
        assert_eq!(wctomb_s_l(&mut e, count, target, 4, 0x20ac, 0), 0x2a);
        assert_eq!(e.mem.u32(target), 0);
        assert_eq!(e.mem.u32(count), 0xffff_ffff);
        assert_eq!(e.mem.u32(slot), 0x2a);
        // No room, a size above 0x7fffffff, no target.
        assert_eq!(wctomb_s_l(&mut e, count, target, 0, 0x41, 0), 0x22);
        assert_eq!(
            wctomb_s_l(&mut e, count, target, 0x8000_0000, 0x41, 0),
            0x16
        );
        assert_eq!(wctomb_s_l(&mut e, count, 0, 4, 0x41, 0), 0);
        assert_eq!(e.mem.u32(count), 0);
        assert_eq!(wctomb_s_l(&mut e, count, 0, 0, 0x41, 0), 0);
        assert_eq!(e.mem.u32(count), 1);
    }

    #[test]
    fn wctomb_s_l_through_the_code_page() {
        let (mut e, info, _, _) = engine();
        e.mem.set_u32(info + 0x14, 0x411);
        e.mem.set_u32(info + 4, 932);
        e.register_double(WIDE_CHAR_TO_MULTI_BYTE, |e, a| {
            assert_eq!((a[0], a[1], a[3], a[6]), (932, 0, 1, 0));
            match e.mem.u16(a[2]) {
                0x3042 => {
                    e.mem.set_u8(a[4], 0x82);
                    e.mem.set_u8(a[4] + 1, 0xa0);
                    2u32.into_ret()
                }
                0x20ac => {
                    e.mem.set_u32(a[7], 1);
                    1u32.into_ret()
                }
                _ => 0u32.into_ret(),
            }
        });
        let last_error = std::rc::Rc::new(std::cell::Cell::new(0x7au32));
        let le = last_error.clone();
        e.register_double(GET_LAST_ERROR, move |_, _| le.get().into_ret());
        let count = e.mem.alloc(4);
        let target = e.mem.alloc(4);
        assert_eq!(wctomb_s_l(&mut e, count, target, 4, 0x3042, 0), 0);
        assert_eq!(e.mem.u32(count), 2);
        assert_eq!(e.mem.u16(target), 0xa082);
        assert_eq!(wctomb_s_l(&mut e, count, target, 4, 0x20ac, 0), 0x2a);
        e.mem.set_u32(target, 0x0101_0101);
        assert_eq!(wctomb_s_l(&mut e, count, target, 4, 0x1234, 0), 0x22);
        assert_eq!(e.mem.u32(target), 0);
        last_error.set(87);
        assert_eq!(wctomb_s_l(&mut e, count, target, 4, 0x1234, 0), 0x2a);
    }

    fn wide_text(e: &mut Engine, chars: &[u16]) -> u32 {
        let at = e.mem.alloc(chars.len() as u32 * 2 + 2);
        for (i, c) in chars.iter().enumerate() {
            e.mem.set_u16(at + i as u32 * 2, *c);
        }
        at
    }

    #[test]
    fn wcstombs_l_helper_in_the_c_locale() {
        let (mut e, ..) = engine();
        let text = wide_text(&mut e, &[0x41, 0x42, 0]);
        let target = e.mem.alloc(8);
        assert_eq!(e.call(0x00ec_aa70, &args![0u32, text, 0u32, 0u32]).u32(), 2);
        assert_eq!(wcstombs_l_helper(&mut e, target, text, 8, 0), 2);
        assert_eq!(e.mem.cstr(target), b"AB".to_vec());
        e.mem.set_u32(target, 0);
        assert_eq!(wcstombs_l_helper(&mut e, target, text, 1, 0), 1);
        assert_eq!(e.mem.cstr(target), b"A".to_vec());
        assert_eq!(wcstombs_l_helper(&mut e, target, text, 0, 0), 0);
        let wide = wide_text(&mut e, &[0x41, 0x3042, 0]);
        assert_eq!(wcstombs_l_helper(&mut e, target, wide, 8, 0), 0xffff_ffff);
        assert_eq!(wcstombs_l_helper(&mut e, target, 0, 8, 0), 0xffff_ffff);
    }

    /// `WideCharToMultiByte` for code page 932: ASCII as one byte, anything else as two;
    /// with `-1` into a target, too small (0) unless it has room for everything.
    fn shift_jis(e: &mut Engine, a: &[u32]) -> Ret {
        let mut bytes = Vec::new();
        let mut at = a[2];
        let mut left = a[3];
        loop {
            let c = e.mem.u16(at);
            if c < 0x80 {
                bytes.push(c as u8);
            } else {
                bytes.extend([0x82, (c & 0xff) as u8]);
            }
            at += 2;
            if left != 0xffff_ffff {
                left -= 1;
                if left == 0 {
                    break;
                }
            } else if c == 0 {
                break;
            }
        }
        if a[4] != 0 {
            if bytes.len() as u32 > a[5] {
                return 0u32.into_ret();
            }
            e.mem.write(a[4], &bytes);
        }
        (bytes.len() as u32).into_ret()
    }

    #[test]
    fn wcstombs_l_helper_through_the_code_page() {
        let (mut e, info, ..) = engine();
        e.mem.set_u32(info + 0x14, 0x411);
        e.mem.set_u32(info + 4, 932);
        e.mem.set_u32(info + 0xac, 2);
        e.register_double(WIDE_CHAR_TO_MULTI_BYTE, shift_jis);
        e.register_double(GET_LAST_ERROR, |_, _| 0x7au32.into_ret());
        let text = wide_text(&mut e, &[0x41, 0x3042, 0x42, 0]);
        let target = e.mem.alloc(8);
        // No target: the length less the zero.
        assert_eq!(wcstombs_l_helper(&mut e, 0, text, 0, 0), 4);
        // Room for all: converted at once.
        assert_eq!(wcstombs_l_helper(&mut e, target, text, 8, 0), 4);
        assert_eq!(e.mem.bytes(target, 5), vec![0x41, 0x82, 0x42, 0x42, 0]);
        // Too little room: character by character while whole characters fit.
        e.mem.write(target, &[0xee; 8]);
        assert_eq!(wcstombs_l_helper(&mut e, target, text, 3, 0), 3);
        assert_eq!(e.mem.bytes(target, 4), vec![0x41, 0x82, 0x42, 0xee]);
        assert_eq!(wcstombs_l_helper(&mut e, target, text, 2, 0), 1);
        // A single-byte code page: the count cut at the zero, which is not counted.
        e.mem.set_u32(info + 0xac, 1);
        let ascii = wide_text(&mut e, &[0x43, 0x44, 0]);
        assert_eq!(wcstombs_l_helper(&mut e, target, ascii, 8, 0), 2);
        assert_eq!(e.mem.cstr(target), b"CD".to_vec());
        assert_eq!(wcstombs_l_helper(&mut e, target, ascii, 1, 0), 1);
    }
}

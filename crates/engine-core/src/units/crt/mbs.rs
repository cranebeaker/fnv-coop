//! Multibyte-string and conversion bodies the null-locale wrappers call (MSVC 2008 LIBCMT,
//! statically linked into FalloutNV.exe 1.4.0.525), translated from the exe:
//! `__mbsnbcmp_l` (`00ecb6df`), `__mbsnbicmp_l` (`00ecb863`), `_mbsnbcpy_l` (`00eca820`),
//! `__wcstombs_s_l` (`00ecaccc`), `_strtod_l` (`00ecde0e`) and `__fltin2` (`00edb0db`).
//!
//! The `_LocaleUpdate` object (0x10 bytes: the locale information, the multibyte
//! information, the thread's data, the "updated" byte) is made by `00ec660e`; the
//! multibyte information holds the code page flag (+0x8), the two upper-case ranges
//! (+0x10..+0x1a: low, high, delta twice), the character types (+0x1d, bit 0x4 a lead
//! byte, 0x10 a lower-case single byte) and the case map (+0x11d).

use super::*;

/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`), `errno` (`00ec85e3`), the invalid-parameter
/// report (`00ec7c30`), `strncmp` (`00ec8a19`), `_strnicmp` (`00ec7ec0`), `strncpy`
/// (`00ecc7b0`), `memset` (`00ec61c0`), `__wcstombs_l_helper` (`00ecaa70`), `_isctype_l`
/// (`00ed41e7`), `strlen` (`00ec6130`) and `__fltin2` (`00edb0db`).
const LOCALE_UPDATE: u32 = 0x00ec_660e;
const ERRNO: u32 = 0x00ec_85e3;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
const STRNCMP: u32 = 0x00ec_8a19;
const STRNICMP: u32 = 0x00ec_7ec0;
const STRNCPY: u32 = 0x00ec_c7b0;
const MEMSET: u32 = 0x00ec_61c0;
const WCSTOMBS_L_HELPER: u32 = 0x00ec_aa70;
const ISCTYPE_L: u32 = 0x00ed_41e7;
const STRLEN: u32 = 0x00ec_6130;
const FLTIN2: u32 = 0x00ed_b0db;
/// `___strgtold12_l` (`00edb182`) and `__ld12tod` (`00ee3a59`).
const STRGTOLD12_L: u32 = 0x00ed_b182;
const LD12TOD: u32 = 0x00ee_3a59;
/// The overflow answer `_strtod_l` loads (`011c2a80`, a double: +infinity in the image).
const HUGE_VALUE: u32 = 0x011c_2a80;

/// `errno = code` and the invalid-parameter report with five zeros.
fn report(e: &mut Engine, code: u32) {
    let slot = e.call(ERRNO, &args![]).u32();
    e.mem.set_u32(slot, code);
    e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
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

fn lead_byte(e: &Engine, mbc: u32, byte: u8) -> bool {
    e.mem
        .u8(mbc.wrapping_add(0x1d).wrapping_add(u32::from(byte)))
        & 4
        != 0
}

/// The upper-case ranges of a double-byte character (`CMP`/`JC`/`JA`, then `ADD` of the
/// delta): the first range, else the second.
fn upper_double(e: &Engine, mbc: u32, c: u16) -> u16 {
    let word = |offset: u32| e.mem.u16(mbc.wrapping_add(offset));
    if c >= word(0x10) && c <= word(0x12) {
        c.wrapping_add(word(0x14))
    } else if c >= word(0x16) && c <= word(0x18) {
        c.wrapping_add(word(0x1a))
    } else {
        c
    }
}

/// A single-byte character through the case map when it is a lower-case one (bit 0x10).
fn upper_single(e: &Engine, mbc: u32, c: u16) -> u16 {
    let at = mbc.wrapping_add(u32::from(c));
    if e.mem.u8(at.wrapping_add(0x1d)) & 0x10 != 0 {
        u16::from(e.mem.u8(at.wrapping_add(0x11d)))
    } else {
        c
    }
}

/// The answer for two characters that differ: 1 when the second is below the first
/// (`SBB`, `AND 2`, `DEC`), else -1.
fn order(first: u16, second: u16) -> i32 {
    if second < first {
        1
    } else {
        -1
    }
}

// Translated from 00ecb6df (disassembled, FalloutNV.exe 1.4.0.525)
/// `__mbsnbcmp_l` (Ghidra's function ID), cdecl: two texts, a byte count, a locale. A count
/// of 0 answers 0. In a single-byte code page `strncmp`. Otherwise either text null sets
/// `errno` to `EINVAL`, reports and answers 0x7fffffff. Characters are read a lead byte and
/// its trail at a time (a trail of 0, or none left in the count, reads as 0) and compared
/// as words; the count drops for each first-text lead byte and each second-text byte. A
/// first-text lead byte with nothing left compares the second text's byte with 0 (equal
/// when that is a lead byte too).
pub fn mbsnbcmp_l(e: &mut Engine, first: u32, second: u32, count: u32, locale: u32) -> i32 {
    if count == 0 {
        return 0;
    }
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let mbc = e.mem.u32(update.wrapping_add(4));
        let result = if e.mem.u32(mbc.wrapping_add(8)) == 0 {
            e.call(STRNCMP, &args![first, second, count]).i32()
        } else if first == 0 || second == 0 {
            report(e, 0x16);
            0x7fff_ffff
        } else {
            compare(e, mbc, first, second, count, false)
        };
        locale_update_done(e, update);
        result
    })
}

/// The shared walk of `__mbsnbcmp_l` and `__mbsnbicmp_l` (`ignore_case` maps both
/// characters to upper case, single bytes through the case map and double bytes through
/// the ranges).
fn compare(
    e: &mut Engine,
    mbc: u32,
    first: u32,
    second: u32,
    count: u32,
    ignore_case: bool,
) -> i32 {
    let (mut a, mut b, mut left) = (first, second, count);
    loop {
        let byte = e.mem.u8(a);
        left = left.wrapping_sub(1);
        a = a.wrapping_add(1);
        let c1: u16;
        if lead_byte(e, mbc, byte) {
            if left == 0 {
                let other = e.mem.u8(b);
                if lead_byte(e, mbc, other) {
                    return 0;
                }
                let c2 = u16::from(other);
                if c2 != 0 {
                    return order(0, c2);
                }
                return 0;
            }
            let trail = e.mem.u8(a);
            if trail == 0 {
                c1 = 0;
            } else {
                a = a.wrapping_add(1);
                let c = u16::from(byte) << 8 | u16::from(trail);
                c1 = if ignore_case {
                    upper_double(e, mbc, c)
                } else {
                    c
                };
            }
        } else {
            let c = u16::from(byte);
            c1 = if ignore_case {
                upper_single(e, mbc, c)
            } else {
                c
            };
        }
        let other = e.mem.u8(b);
        b = b.wrapping_add(1);
        let c2: u16;
        if lead_byte(e, mbc, other) {
            if left == 0 {
                c2 = 0;
            } else {
                let trail = e.mem.u8(b);
                left = left.wrapping_sub(1);
                if trail == 0 {
                    c2 = 0;
                } else {
                    b = b.wrapping_add(1);
                    let c = u16::from(other) << 8 | u16::from(trail);
                    c2 = if ignore_case {
                        upper_double(e, mbc, c)
                    } else {
                        c
                    };
                }
            }
        } else {
            let c = u16::from(other);
            c2 = if ignore_case {
                upper_single(e, mbc, c)
            } else {
                c
            };
        }
        if c2 != c1 {
            return order(c1, c2);
        }
        if c1 == 0 || left == 0 {
            return 0;
        }
    }
}

// Translated from 00ecb863 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__mbsnbicmp_l` (Ghidra's function ID), cdecl: two texts, a byte count, a locale; as
/// `__mbsnbcmp_l` ignoring case (the `_LocaleUpdate` is made first; a single-byte code page
/// uses `_strnicmp`). A first-text lead byte with nothing left compares the second text's
/// byte, unmapped, with 0.
pub fn mbsnbicmp_l(e: &mut Engine, first: u32, second: u32, count: u32, locale: u32) -> i32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let mbc = e.mem.u32(update.wrapping_add(4));
        let result = if count == 0 {
            0
        } else if e.mem.u32(mbc.wrapping_add(8)) == 0 {
            e.call(STRNICMP, &args![first, second, count]).i32()
        } else if first == 0 || second == 0 {
            report(e, 0x16);
            0x7fff_ffff
        } else {
            compare(e, mbc, first, second, count, true)
        };
        locale_update_done(e, update);
        result
    })
}

// Translated from 00eca820 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbsnbcpy_l` (Ghidra's function ID), cdecl: target, source, byte count, locale. A null
/// target or source with a count sets `errno` to `EINVAL`, reports and answers null. In a
/// single-byte code page `strncpy`. Otherwise bytes are copied a character at a time: a
/// lead byte whose trail does not fit in the count is replaced by 0; a zero (a single one,
/// or a trail, which also clears its lead byte) ends the copy and the rest of the count is
/// cleared (`memset`). The target is answered.
pub fn mbsnbcpy_l(e: &mut Engine, target: u32, source: u32, count: u32, locale: u32) -> u32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        if (target == 0 || source == 0) && count != 0 {
            report(e, 0x16);
            locale_update_done(e, update);
            return 0;
        }
        let mbc = e.mem.u32(update.wrapping_add(4));
        if e.mem.u32(mbc.wrapping_add(8)) == 0 {
            let result = e.call(STRNCPY, &args![target, source, count]).u32();
            locale_update_done(e, update);
            return result;
        }
        let (mut to, mut from, mut left) = (target, source, count);
        while left != 0 {
            let byte = e.mem.u8(from);
            left -= 1;
            e.mem.set_u8(to, byte);
            to = to.wrapping_add(1);
            from = from.wrapping_add(1);
            let ended = if lead_byte(e, mbc, byte) {
                if left == 0 {
                    e.mem.set_u8(to.wrapping_sub(1), 0);
                    break;
                }
                let trail = e.mem.u8(from);
                left -= 1;
                e.mem.set_u8(to, trail);
                to = to.wrapping_add(1);
                from = from.wrapping_add(1);
                if trail == 0 {
                    e.mem.set_u8(to.wrapping_sub(2), 0);
                }
                trail == 0
            } else {
                byte == 0
            };
            if ended {
                if left != 0 {
                    e.call(MEMSET, &args![to, 0u32, left]);
                }
                break;
            }
        }
        locale_update_done(e, update);
        target
    })
}

// Translated from 00ecaccc (disassembled, FalloutNV.exe 1.4.0.525)
/// `__wcstombs_s_l` (Ghidra's function ID), cdecl: byte count out, target, its size, wide
/// text, count (-1 is `_TRUNCATE`), locale. A target without a size, or a size without a
/// target, is `EINVAL` (reported, answered). The target and count are emptied; the count
/// is cut to the size, and one above 0x7fffffff is `EINVAL`. `__wcstombs_l_helper`
/// failing empties the target again and answers `errno`. Otherwise the converted length
/// plus the zero: more than the size empties the target and is `ERANGE` (reported), or
/// with `_TRUNCATE` is cut to the size and `STRUNCATE` (0x50) answered; the target's last
/// byte is cleared and the length stored. 0 otherwise.
pub fn wcstombs_s_l(
    e: &mut Engine,
    out: u32,
    target: u32,
    size: u32,
    text: u32,
    count: u32,
    locale: u32,
) -> u32 {
    if (target == 0) != (size == 0) {
        report(e, 0x16);
        return 0x16;
    }
    if target != 0 {
        e.mem.set_u8(target, 0);
    }
    if out != 0 {
        e.mem.set_u32(out, 0);
    }
    let limit = count.min(size);
    if limit > 0x7fff_ffff {
        report(e, 0x16);
        return 0x16;
    }
    let converted = e
        .call(WCSTOMBS_L_HELPER, &args![target, text, limit, locale])
        .u32();
    if converted == 0xffff_ffff {
        if target != 0 {
            e.mem.set_u8(target, 0);
        }
        let slot = e.call(ERRNO, &args![]).u32();
        return e.mem.u32(slot);
    }
    let mut length = converted.wrapping_add(1);
    let mut result = 0;
    if target != 0 {
        if length > size {
            if count != 0xffff_ffff {
                e.mem.set_u8(target, 0);
                if size <= length {
                    report(e, 0x22);
                    return 0x22;
                }
            }
            length = size;
            result = 0x50;
        }
        e.mem.set_u8(target.wrapping_add(length).wrapping_sub(1), 0);
    }
    if out != 0 {
        e.mem.set_u32(out, length);
    }
    result
}

// Translated from 00ecde0e (disassembled, FalloutNV.exe 1.4.0.525)
/// `_strtod_l` (no label; `00ecdf2d` is `strtod`), cdecl: text, end out, locale; the double
/// in `ST0`. The end starts at the text. A null text sets `errno` to `EINVAL`, reports and
/// answers 0. Leading spaces (type 0x8: `_isctype_l` in a multibyte locale, else the
/// locale's type table) are skipped and `__fltin2` reads the number (the text's `strlen`,
/// no scale or decimal shift); the end becomes the text plus the bytes read. Flags 0x240
/// (nothing read) answer 0 with the end back at the start; 0x81 (overflow) answer the
/// overflow value (`011c2a80`), negated after a `-`, with `ERANGE`; 0x100 (underflow)
/// answers the value with `ERANGE` when it compares equal to 0; otherwise the value.
pub fn strtod_l(e: &mut Engine, text: u32, end: u32, locale: u32) -> f64 {
    // The frame: the _LocaleUpdate object (EBP-0x18) and the __fltin2 result (EBP-0x30).
    e.with_stack(0x30, |e, frame| {
        let update = frame.addr().wrapping_add(0x18);
        let flt = frame.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        if end != 0 {
            e.mem.set_u32(end, text);
        }
        if text == 0 {
            report(e, 0x16);
            locale_update_done(e, update);
            return 0.0;
        }
        let mut at = text;
        loop {
            let info = e.mem.u32(update);
            let c = e.mem.u8(at);
            let space = if (e.mem.u32(info.wrapping_add(0xac)) as i32) > 1 {
                e.call(ISCTYPE_L, &args![u32::from(c), 8u32, update]).u32()
            } else {
                let table = e.mem.u32(info.wrapping_add(0xc8));
                u32::from(e.mem.u16(table.wrapping_add(u32::from(c) * 2))) & 8
            };
            if space == 0 {
                break;
            }
            at = at.wrapping_add(1);
        }
        let len = e.call(STRLEN, &args![at]).u32();
        let result = e
            .call(FLTIN2, &args![flt, at, len, 0u32, 0u32, update])
            .u32();
        if end != 0 {
            let read = e.mem.u32(result.wrapping_add(4));
            e.mem.set_u32(end, read.wrapping_add(at));
        }
        let flags = e.mem.u32(result);
        let value = if flags & 0x240 != 0 {
            if end != 0 {
                e.mem.set_u32(end, text);
            }
            0.0
        } else if flags & 0x81 != 0 {
            let huge = f64::from_bits(e.mem.u64(HUGE_VALUE));
            let value = if e.mem.u8(at) == b'-' { -huge } else { huge };
            let slot = e.call(ERRNO, &args![]).u32();
            e.mem.set_u32(slot, 0x22);
            value
        } else {
            let value = f64::from_bits(e.mem.u64(result.wrapping_add(0x10)));
            // FCOM with 0.0, then FNSTSW: TEST AH,0x44; JP — equal sets C3 alone.
            if flags & 0x100 != 0 && value == 0.0 {
                let slot = e.call(ERRNO, &args![]).u32();
                e.mem.set_u32(slot, 0x22);
                0.0
            } else {
                value
            }
        };
        locale_update_done(e, update);
        value
    })
}

// Translated from 00edb0db (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00edb0db - the /GS stack cookie check (00ec408c) is not translated
/// `__fltin2` (Ghidra's function ID), cdecl: the result (`_flt`: flags, bytes read, the
/// double at +0x10), the text, its length (not read), scale and decimal shift (not read),
/// a `_LocaleUpdate`. `___strgtold12_l` reads the text into a 12-byte long double (with
/// its end); a failed read (bit 0x4) gives flag 0x200 and 0.0. Otherwise `__ld12tod`
/// converts it: an overflow (read bit 0x2, or the conversion answering 1) gives 0x80, an
/// underflow (bit 0x1, or 2) 0x100. The bytes read and the double are stored and the
/// result answered.
pub fn fltin2(
    e: &mut Engine,
    flt: u32,
    text: u32,
    _len: u32,
    _scale: u32,
    _shift: u32,
    locale: u32,
) -> u32 {
    e.with_stack(0x24, |e, frame| {
        // EBP-0x24: the end; EBP-0x1c: the double; EBP-0x10: the long double.
        let end = frame.addr();
        let double = end + 8;
        let ld12 = end + 0x14;
        let read = e
            .call(
                STRGTOLD12_L,
                &args![ld12, end, text, 0u32, 0u32, 0u32, 0u32, locale],
            )
            .u32();
        let mut flags = 0u32;
        if read & 4 != 0 {
            flags = 0x200;
            e.mem.set_u64(double, 0);
        } else {
            let result = e.call(LD12TOD, &args![ld12, double]).u32();
            if read & 2 != 0 || result == 1 {
                flags = 0x80;
            }
            if read & 1 != 0 || result == 2 {
                flags |= 0x100;
            }
        }
        let stop = e.mem.u32(end);
        e.mem.set_u32(flt.wrapping_add(4), stop.wrapping_sub(text));
        let value = e.mem.u64(double);
        e.mem.set_u64(flt.wrapping_add(0x10), value);
        e.mem.set_u32(flt, flags);
        flt
    })
}

/// The registry entry of [`strtod_l`]: the double in `ST0`.
fn strtod_l_entry(e: &mut Engine, words: &[u32]) -> Ret {
    Ret {
        st0: strtod_l(e, words[0], words[1], words[2]),
        ..Ret::default()
    }
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_b6df, mbsnbcmp_l(u32, u32, u32, u32) -> i32),
        entry!(0x00ec_b863, mbsnbicmp_l(u32, u32, u32, u32) -> i32),
        entry!(0x00ec_a820, mbsnbcpy_l(u32, u32, u32, u32) -> u32),
        entry!(0x00ec_accc, wcstombs_s_l(u32, u32, u32, u32, u32, u32) -> u32),
        (0x00ec_de0e, strtod_l_entry as AbiFn),
        entry!(0x00ed_b0db, fltin2(u32, u32, u32, u32, u32, u32) -> u32),
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

    fn text(e: &mut Engine, t: &[u8]) -> u32 {
        let at = e.mem.alloc(t.len() as u32 + 1);
        e.mem.set_cstr(at, t);
        at
    }

    /// An engine whose `_LocaleUpdate` hands out a double-byte code page (lead bytes
    /// 0x81..0x9f; a..z lower case mapping to A..Z; 0x8281..0x829a mapping down by 0x21)
    /// and a locale information with a type table (0x20 a space); `errno` and the report
    /// doubled. (engine, info, multibyte information, errno slot).
    fn engine() -> (Engine, u32, u32, u32) {
        let mut e = Engine::new();
        let info = e.mem.alloc(0xd8);
        let types = e.mem.alloc(0x200);
        e.mem.set_u16(types + 0x20 * 2, 8);
        e.mem.set_u32(info + 0xc8, types);
        e.mem.set_u32(info + 0xac, 1);
        let mbc = e.mem.alloc(0x220);
        e.mem.set_u32(mbc + 8, 1);
        for b in 0x81..=0x9fu32 {
            e.mem.set_u8(mbc + 0x1d + b, 4);
        }
        for b in u32::from(b'a')..=u32::from(b'z') {
            e.mem.set_u8(mbc + 0x1d + b, 0x10);
            e.mem.set_u8(mbc + 0x11d + b, (b - 0x20) as u8);
        }
        e.mem.set_u16(mbc + 0x10, 0x8281);
        e.mem.set_u16(mbc + 0x12, 0x829a);
        e.mem.set_u16(mbc + 0x14, 0xffdf);
        e.mem.set_u16(mbc + 0x16, 0xffff);
        e.mem.set_u16(mbc + 0x18, 0);
        let ptd = e.mem.alloc(0x80);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u32(a[0] + 4, mbc);
            e.mem.set_u32(a[0] + 8, ptd);
            e.mem.set_u8(a[0] + 0xc, 0);
            a[0].into_ret()
        });
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| slot.into_ret());
        e.register_double(INVALID_PARAMETER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, info, mbc, slot)
    }

    #[test]
    fn mbsnbcmp_l_compares_whole_characters() {
        let (mut e, _, mbc, slot) = engine();
        let a = text(&mut e, b"a\x82\x81b");
        let b = text(&mut e, b"a\x82\x81c");
        assert_eq!(e.call(0x00ec_b6df, &args![a, b, 3u32, 0u32]).i32(), 0);
        assert_eq!(mbsnbcmp_l(&mut e, a, b, 4, 0), -1);
        assert_eq!(mbsnbcmp_l(&mut e, b, a, 4, 0), 1);
        assert_eq!(mbsnbcmp_l(&mut e, a, b, 0, 0), 0);
        // A lead byte in the first text with nothing left: the second's byte against 0.
        let lead = text(&mut e, b"\x82");
        let plain = text(&mut e, b"x");
        assert_eq!(mbsnbcmp_l(&mut e, lead, plain, 1, 0), -1);
        assert_eq!(mbsnbcmp_l(&mut e, lead, lead, 1, 0), 0);
        assert_eq!(mbsnbcmp_l(&mut e, 0, b, 2, 0), 0x7fff_ffff);
        assert_eq!(e.mem.u32(slot), 0x16);
        // A single-byte code page uses strncmp.
        e.mem.set_u32(mbc + 8, 0);
        e.register_double(STRNCMP, |_, a| (a[2] + 100).into_ret());
        assert_eq!(mbsnbcmp_l(&mut e, a, b, 2, 0), 102);
    }

    #[test]
    fn mbsnbicmp_l_maps_both_sides_to_upper_case() {
        let (mut e, _, mbc, _) = engine();
        let a = text(&mut e, b"Ab\x82\x81z");
        let b = text(&mut e, b"aB\x82\x60Z");
        assert_eq!(e.call(0x00ec_b863, &args![a, b, 5u32, 0u32]).i32(), 0);
        let c = text(&mut e, b"aC");
        assert_eq!(mbsnbicmp_l(&mut e, a, c, 2, 0), -1);
        assert_eq!(mbsnbicmp_l(&mut e, c, a, 2, 0), 1);
        assert_eq!(mbsnbicmp_l(&mut e, a, c, 0, 0), 0);
        assert_eq!(mbsnbicmp_l(&mut e, a, 0, 2, 0), 0x7fff_ffff);
        e.mem.set_u32(mbc + 8, 0);
        e.register_double(STRNICMP, |_, a| (a[2] + 200).into_ret());
        assert_eq!(mbsnbicmp_l(&mut e, a, b, 3, 0), 203);
    }

    #[test]
    fn mbsnbcpy_l_copies_characters_and_clears_the_rest() {
        let (mut e, _, mbc, slot) = engine();
        let source = text(&mut e, b"a\x82\x81");
        let target = e.mem.alloc(8);
        e.mem.write(target, &[0xee; 8]);
        assert_eq!(
            e.call(0x00ec_a820, &args![target, source, 6u32, 0u32])
                .u32(),
            target
        );
        assert_eq!(
            e.mem.bytes(target, 7),
            vec![b'a', 0x82, 0x81, 0, 0, 0, 0xee]
        );
        // A lead byte whose trail does not fit becomes 0.
        e.mem.write(target, &[0xee; 8]);
        mbsnbcpy_l(&mut e, target, source, 2, 0);
        assert_eq!(e.mem.bytes(target, 3), vec![b'a', 0, 0xee]);
        // A lead byte followed by the end: both cleared, then the rest.
        let broken = text(&mut e, b"\x82");
        e.mem.write(target, &[0xee; 8]);
        mbsnbcpy_l(&mut e, target, broken, 4, 0);
        assert_eq!(e.mem.bytes(target, 5), vec![0, 0, 0, 0, 0xee]);
        assert_eq!(mbsnbcpy_l(&mut e, 0, source, 2, 0), 0);
        assert_eq!(mbsnbcpy_l(&mut e, target, 0, 2, 0), 0);
        assert_eq!(e.mem.u32(slot), 0x16);
        assert_eq!(mbsnbcpy_l(&mut e, 0, 0, 0, 0), 0);
        e.mem.set_u32(mbc + 8, 0);
        e.register_double(STRNCPY, |_, a| (a[0] + 1).into_ret());
        assert_eq!(mbsnbcpy_l(&mut e, target, source, 2, 0), target + 1);
    }

    #[test]
    fn wcstombs_s_l_checks_room_and_truncates() {
        let (mut e, _, _, slot) = engine();
        // A five-character text: without a target its length, else as much as fits.
        e.register_double(WCSTOMBS_L_HELPER, |e, a| {
            if a[0] == 0 {
                return 5u32.into_ret();
            }
            let n = a[2].min(5);
            for i in 0..n {
                e.mem.set_u8(a[0] + i, b'x');
            }
            n.into_ret()
        });
        let out = e.mem.alloc(4);
        let target = e.mem.alloc(16);
        assert_eq!(
            e.call(
                0x00ec_accc,
                &args![out, target, 16u32, 0x100u32, 16u32, 0u32]
            )
            .u32(),
            0
        );
        assert_eq!(e.mem.u32(out), 6);
        assert_eq!(e.mem.cstr(target), b"xxxxx".to_vec());
        // Too little room: ERANGE, or with _TRUNCATE the cut text and STRUNCATE.
        assert_eq!(wcstombs_s_l(&mut e, out, target, 4, 0x100, 16, 0), 0x22);
        assert_eq!(e.mem.u8(target), 0);
        assert_eq!(e.mem.u32(slot), 0x22);
        assert_eq!(
            wcstombs_s_l(&mut e, out, target, 4, 0x100, 0xffff_ffff, 0),
            0x50
        );
        assert_eq!(e.mem.u32(out), 4);
        assert_eq!(e.mem.cstr(target), b"xxx".to_vec());
        // No target: the length alone.
        assert_eq!(wcstombs_s_l(&mut e, out, 0, 0, 0x100, 16, 0), 0);
        assert_eq!(e.mem.u32(out), 6);
        // Mismatched target and size.
        assert_eq!(wcstombs_s_l(&mut e, out, 0, 4, 0x100, 16, 0), 0x16);
        assert_eq!(wcstombs_s_l(&mut e, out, target, 0, 0x100, 16, 0), 0x16);
        // A failed conversion answers errno.
        e.register_double(WCSTOMBS_L_HELPER, |_, _| 0xffff_ffffu32.into_ret());
        e.mem.set_u32(slot, 0x2a);
        assert_eq!(wcstombs_s_l(&mut e, out, target, 16, 0x100, 16, 0), 0x2a);
    }

    #[test]
    fn strtod_l_reads_through_fltin2() {
        let (mut e, _, _, slot) = engine();
        e.mem
            .write(HUGE_VALUE, &f64::INFINITY.to_bits().to_le_bytes());
        let reply = std::rc::Rc::new(std::cell::Cell::new((0u32, 0u32, 0.0f64)));
        let r = reply.clone();
        e.register_double(FLTIN2, move |e, a| {
            let (flags, read, value) = r.get();
            e.mem.set_u32(a[0], flags);
            e.mem.set_u32(a[0] + 4, read);
            e.mem.set_u64(a[0] + 0x10, value.to_bits());
            a[0].into_ret()
        });
        let input = text(&mut e, b"  -12.5x");
        let end = e.mem.alloc(4);
        reply.set((0, 5, -12.5));
        assert_eq!(e.call(0x00ec_de0e, &args![input, end, 0u32]).f64(), -12.5);
        assert_eq!(e.mem.u32(end), input + 7);
        assert_eq!(calls(&e, FLTIN2)[0][1..3], [input + 2, 6]);
        // Nothing read: 0 with the end at the start.
        reply.set((0x40, 0, 0.0));
        assert_eq!(strtod_l(&mut e, input, end, 0), 0.0);
        assert_eq!(e.mem.u32(end), input);
        // Overflow after a '-': the negated overflow value, ERANGE.
        reply.set((1, 5, 0.0));
        assert_eq!(strtod_l(&mut e, input, end, 0), f64::NEG_INFINITY);
        assert_eq!(e.mem.u32(slot), 0x22);
        // Underflow to 0 is ERANGE; a nonzero underflowed value is answered.
        e.mem.set_u32(slot, 0);
        reply.set((0x100, 5, 1e-310));
        assert_eq!(strtod_l(&mut e, input, end, 0), 1e-310);
        assert_eq!(e.mem.u32(slot), 0);
        reply.set((0x100, 5, 0.0));
        assert_eq!(strtod_l(&mut e, input, end, 0), 0.0);
        assert_eq!(e.mem.u32(slot), 0x22);
        // No text: EINVAL.
        assert_eq!(strtod_l(&mut e, 0, end, 0), 0.0);
        assert_eq!(e.mem.u32(end), 0);
        assert_eq!(e.mem.u32(slot), 0x16);
    }

    #[test]
    fn fltin2_reads_and_converts_with_flags() {
        let (mut e, ..) = engine();
        let answers = std::rc::Rc::new(std::cell::Cell::new((0u32, 0u32)));
        let a1 = answers.clone();
        e.register_double(STRGTOLD12_L, move |e, a| {
            e.mem.set_u32(a[1], a[2] + 4);
            a1.get().0.into_ret()
        });
        let a2 = answers.clone();
        e.register_double(LD12TOD, move |e, a| {
            e.mem.set_u64(a[1], 2.5f64.to_bits());
            a2.get().1.into_ret()
        });
        let flt = e.mem.alloc(0x18);
        let input = text(&mut e, b"2.5e0");
        for ((read, converted), flags) in [
            ((0, 0), 0),
            ((2, 0), 0x80),
            ((0, 2), 0x100),
            ((1, 1), 0x180),
        ] {
            answers.set((read, converted));
            assert_eq!(
                e.call(0x00ed_b0db, &args![flt, input, 5u32, 0u32, 0u32, 0u32])
                    .u32(),
                flt
            );
            assert_eq!(e.mem.u32(flt), flags);
            assert_eq!(e.mem.u32(flt + 4), 4);
            assert_eq!(f64::from_bits(e.mem.u64(flt + 0x10)), 2.5);
        }
        answers.set((4, 0));
        fltin2(&mut e, flt, input, 5, 0, 0, 0);
        assert_eq!((e.mem.u32(flt), e.mem.u64(flt + 0x10)), (0x200, 0));
        assert_eq!(calls(&e, LD12TOD).len(), 4);
    }
}

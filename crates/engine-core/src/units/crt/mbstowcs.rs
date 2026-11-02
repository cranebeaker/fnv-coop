//! The C runtime's multibyte-to-wide conversion (MSVC 2008 LIBCMT/mbstowcs.c, statically
//! linked into FalloutNV.exe 1.4.0.525), translated from the exe: `_mbstowcs_l_helper`
//! (`00ecf064`) and `_mbstowcs_s_l` (`00ecf205`). Every call is made by address, as the exe
//! makes it, so a test can double each one.

use super::*;

/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`; the object, then the locale), `_errno`
/// (`00ec85e3`), the invalid-parameter report (`00ec7c30`, five words), `strlen`
/// (`00ec6130`) and `_isleadbyte_l` (`00ed419c`: byte, `_LocaleUpdate` object), cdecl.
const LOCALE_UPDATE: u32 = 0x00ec_660e;
const ERRNO: u32 = 0x00ec_85e3;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
const STRLEN: u32 = 0x00ec_6130;
const ISLEADBYTE_L: u32 = 0x00ed_419c;
/// `_mbstowcs_l_helper` itself, which `_mbstowcs_s_l` calls by address.
const MBSTOWCS_L_HELPER: u32 = 0x00ec_f064;
/// Import slots (KERNEL32, stdcall): `MultiByteToWideChar` and `GetLastError`.
const API_MULTI_BYTE_TO_WIDE_CHAR: u32 = 0x00fd_f128;
const API_GET_LAST_ERROR: u32 = 0x00fd_f094;
/// `EINVAL`, `ERANGE`, `EILSEQ` and `STRUNCATE`.
const EINVAL: u32 = 0x16;
const ERANGE: u32 = 0x22;
const EILSEQ: u32 = 0x2a;
const STRUNCATE: u32 = 0x50;

fn set_errno(e: &mut Engine, value: u32) {
    let errno = e.call(ERRNO, &[]).u32();
    e.mem.set_u32(errno, value);
}

fn report_invalid(e: &mut Engine) {
    e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
}

// Translated from 00ecf064 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbstowcs_l_helper` (cdecl: wide out, text, count, locale). With an output: 0 for a zero
/// count; else the first wide character is cleared and, in the "C" locale (no `LC_CTYPE`
/// handle, locale info +0x14), the bytes are widened up to and without the terminator or
/// `count` (that many answered); in another locale `MultiByteToWideChar` (`MB_PRECOMPOSED |
/// MB_ERR_INVALID_CHARS`, terminated) answers its length less the terminator, and when the
/// output is too small the text is measured to `count` characters (lead bytes pairing with
/// the next byte) and converted without the terminator. Without an output the length is
/// measured (`strlen`, or the conversion with no output). A null text sets `errno` to
/// `EINVAL`, reports it and answers -1; an invalid sequence sets `EILSEQ` (clearing the
/// first output character) and answers -1.
pub fn mbstowcs_l_helper(e: &mut Engine, out: u32, text: u32, count: u32, locale: u32) -> i32 {
    if out != 0 {
        if count == 0 {
            return 0;
        }
        e.mem.set_u16(out, 0);
    }
    if text == 0 {
        set_errno(e, EINVAL);
        report_invalid(e);
        return -1;
    }
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let info = e.mem.u32(update);
        let handle = e.mem.u32(info.wrapping_add(0x14));
        let code_page = e.mem.u32(info.wrapping_add(4));
        let result = if out != 0 {
            if handle == 0 {
                let mut n = 0u32;
                while n < count {
                    let c = e.mem.u8(text.wrapping_add(n));
                    e.mem.set_u16(out.wrapping_add(n * 2), u16::from(c));
                    if c == 0 {
                        break;
                    }
                    n += 1;
                }
                n as i32
            } else {
                let converted = e
                    .call(
                        API_MULTI_BYTE_TO_WIDE_CHAR,
                        &args![code_page, 9u32, text, 0xffff_ffffu32, out, count],
                    )
                    .u32();
                if converted != 0 {
                    converted.wrapping_sub(1) as i32
                } else if e.call(API_GET_LAST_ERROR, &[]).u32() != 0x7a {
                    set_errno(e, EILSEQ);
                    e.mem.set_u16(out, 0);
                    -1
                } else {
                    let mut left = count;
                    let mut at = text;
                    let mut broken = false;
                    while left != 0 {
                        let c = e.mem.u8(at);
                        left -= 1;
                        if c == 0 {
                            break;
                        }
                        if e.call(ISLEADBYTE_L, &args![u32::from(c), update]).u32() != 0 {
                            at = at.wrapping_add(1);
                            if e.mem.u8(at) == 0 {
                                broken = true;
                                break;
                            }
                        }
                        at = at.wrapping_add(1);
                    }
                    let converted = if broken {
                        0
                    } else {
                        e.call(
                            API_MULTI_BYTE_TO_WIDE_CHAR,
                            &args![code_page, 1u32, text, at.wrapping_sub(text), out, count],
                        )
                        .u32()
                    };
                    if converted != 0 {
                        converted as i32
                    } else {
                        set_errno(e, EILSEQ);
                        e.mem.set_u16(out, 0);
                        -1
                    }
                }
            }
        } else if handle == 0 {
            e.call(STRLEN, &args![text]).u32() as i32
        } else {
            let converted = e
                .call(
                    API_MULTI_BYTE_TO_WIDE_CHAR,
                    &args![code_page, 9u32, text, 0xffff_ffffu32, 0u32, 0u32],
                )
                .u32();
            if converted != 0 {
                converted.wrapping_sub(1) as i32
            } else {
                set_errno(e, EILSEQ);
                -1
            }
        };
        locale_update_release(e, update);
        result
    })
}

// Translated from 00ecf205 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbstowcs_s_l` (cdecl: converted count out, wide out, size in words, text, count, locale):
/// an output without a size, or a size without an output, is `EINVAL` (reported). The output
/// and the count out start cleared; at most `min(count, size)` characters convert
/// (`_mbstowcs_l_helper`; above `INT_MAX` is `EINVAL`). A failed conversion answers `errno`.
/// The result counts the terminator; one that does not fit is `ERANGE` (reported), or, with
/// `_TRUNCATE` (-1), cut to the size and answered `STRUNCATE`. The output is terminated and
/// the count stored. Answers 0 otherwise.
pub fn mbstowcs_s_l(
    e: &mut Engine,
    converted: u32,
    out: u32,
    size: u32,
    text: u32,
    count: u32,
    locale: u32,
) -> u32 {
    if (out == 0 && size != 0) || (out != 0 && size == 0) {
        set_errno(e, EINVAL);
        report_invalid(e);
        return EINVAL;
    }
    if out != 0 {
        e.mem.set_u16(out, 0);
    }
    if converted != 0 {
        e.mem.set_u32(converted, 0);
    }
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let limit = if count > size { size } else { count };
        let result = if limit > 0x7fff_ffff {
            set_errno(e, EINVAL);
            report_invalid(e);
            EINVAL
        } else {
            let length = e
                .call(MBSTOWCS_L_HELPER, &args![out, text, limit, update])
                .u32();
            if length == 0xffff_ffff {
                if out != 0 {
                    e.mem.set_u16(out, 0);
                }
                let errno = e.call(ERRNO, &[]).u32();
                e.mem.u32(errno)
            } else {
                let mut length = length.wrapping_add(1);
                let mut status = 0;
                let mut failed = None;
                if out != 0 {
                    if length > size {
                        if count != 0xffff_ffff {
                            e.mem.set_u16(out, 0);
                            set_errno(e, ERANGE);
                            report_invalid(e);
                            failed = Some(ERANGE);
                        } else {
                            length = size;
                            status = STRUNCATE;
                        }
                    }
                    if failed.is_none() {
                        e.mem
                            .set_u16(out.wrapping_add(length.wrapping_sub(1) * 2), 0);
                    }
                }
                match failed {
                    Some(code) => code,
                    None => {
                        if converted != 0 {
                            e.mem.set_u32(converted, length);
                        }
                        status
                    }
                }
            }
        };
        locale_update_release(e, update);
        result
    })
}

/// This file's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_f064, mbstowcs_l_helper(u32, u32, u32, u32) -> i32),
        entry!(0x00ec_f205, mbstowcs_s_l(u32, u32, u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let info = e.mem.alloc(0x100);
        let errno = e.mem.alloc(4);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u8(a[0] + 0xc, 0);
            a[0].into_ret()
        });
        e.register_double(ERRNO, move |_, _| errno.into_ret());
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        (e, info, errno)
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let at = e.mem.alloc(s.len() as u32 + 1);
        e.mem.write(at, s);
        e.mem.set_u8(at + s.len() as u32, 0);
        at
    }

    fn wide(e: &Engine, at: u32, n: u32) -> Vec<u16> {
        (0..n).map(|i| e.mem.u16(at + i * 2)).collect()
    }

    #[test]
    fn the_c_locale_widens_bytes() {
        let (mut e, _, errno) = engine();
        let s = text(&mut e, b"ab");
        let out = e.mem.alloc(16);
        assert_eq!(e.call(MBSTOWCS_L_HELPER, &[out, s, 8, 0]).i32(), 2);
        assert_eq!(wide(&e, out, 3), [0x61, 0x62, 0]);
        assert_eq!(e.call(MBSTOWCS_L_HELPER, &[out, s, 1, 0]).i32(), 1);
        assert_eq!(e.call(MBSTOWCS_L_HELPER, &[out, s, 0, 0]).i32(), 0);
        assert_eq!(e.call(MBSTOWCS_L_HELPER, &[0, s, 0, 0]).i32(), 2);
        assert_eq!(e.call(MBSTOWCS_L_HELPER, &[out, 0, 4, 0]).i32(), -1);
        assert_eq!(e.mem.u32(errno), EINVAL);
    }

    #[test]
    fn other_locales_convert_through_windows() {
        let (mut e, info, errno) = engine();
        e.mem.set_u32(info + 0x14, 0x409);
        e.mem.set_u32(info + 4, 932);
        // A pretend MultiByteToWideChar: one wide character per byte (terminator included
        // for -1), failing as "buffer too small" when the output is shorter.
        e.register(API_MULTI_BYTE_TO_WIDE_CHAR, |e, a| {
            let bytes = if a[3] == 0xffff_ffff {
                let mut v = e.mem.cstr(a[2]);
                v.push(0);
                v
            } else {
                e.mem.bytes(a[2], a[3])
            };
            if a[4] == 0 {
                return (bytes.len() as u32).into_ret();
            }
            if bytes.len() as u32 > a[5] {
                return 0u32.into_ret();
            }
            for (i, b) in bytes.iter().enumerate() {
                e.mem.set_u16(a[4] + 2 * i as u32, u16::from(*b) + 0x100);
            }
            (bytes.len() as u32).into_ret()
        });
        e.register(API_GET_LAST_ERROR, |_, _| 0x7au32.into_ret());
        e.register(ISLEADBYTE_L, |_, a| u32::from(a[0] == 0x81).into_ret());
        let s = text(&mut e, b"abc");
        let out = e.mem.alloc(16);
        assert_eq!(e.call(MBSTOWCS_L_HELPER, &[out, s, 8, 0]).i32(), 3);
        assert_eq!(e.call(MBSTOWCS_L_HELPER, &[0, s, 0, 0]).i32(), 3);
        // Too small: the first two characters, without a terminator.
        assert_eq!(e.call(MBSTOWCS_L_HELPER, &[out, s, 2, 0]).i32(), 2);
        assert_eq!(wide(&e, out, 2), [0x161, 0x162]);
        // A lead byte at the end with nothing after it: EILSEQ.
        let broken = text(&mut e, b"a\x81");
        assert_eq!(e.call(MBSTOWCS_L_HELPER, &[out, broken, 1, 0]).i32(), 1);
        assert_eq!(e.call(MBSTOWCS_L_HELPER, &[out, broken, 2, 0]).i32(), -1);
        assert_eq!(e.mem.u32(errno), EILSEQ);
        assert_eq!(e.mem.u16(out), 0);
        // Another error: EILSEQ at once.
        e.register(API_GET_LAST_ERROR, |_, _| 0x459u32.into_ret());
        e.mem.set_u32(errno, 0);
        assert_eq!(e.call(MBSTOWCS_L_HELPER, &[out, s, 2, 0]).i32(), -1);
        assert_eq!(e.mem.u32(errno), EILSEQ);
    }

    #[test]
    fn the_secure_form_checks_sizes_and_truncates() {
        let (mut e, _, errno) = engine();
        let s = text(&mut e, b"abcd");
        let out = e.mem.alloc(16);
        let n = e.mem.alloc(4);
        assert_eq!(e.call(0x00ec_f205, &[n, out, 8, s, 8, 0]).u32(), 0);
        assert_eq!(e.mem.u32(n), 5);
        assert_eq!(wide(&e, out, 5), [0x61, 0x62, 0x63, 0x64, 0]);
        // Too small without _TRUNCATE: ERANGE.
        assert_eq!(e.call(0x00ec_f205, &[n, out, 3, s, 8, 0]).u32(), ERANGE);
        assert_eq!(e.mem.u16(out), 0);
        assert_eq!(e.mem.u32(n), 0);
        // With _TRUNCATE: cut to the size, STRUNCATE.
        assert_eq!(
            e.call(0x00ec_f205, &[n, out, 3, s, 0xffff_ffff, 0]).u32(),
            STRUNCATE
        );
        assert_eq!(wide(&e, out, 3), [0x61, 0x62, 0]);
        assert_eq!(e.mem.u32(n), 3);
        // Size checks.
        assert_eq!(e.call(0x00ec_f205, &[n, out, 0, s, 8, 0]).u32(), EINVAL);
        assert_eq!(e.call(0x00ec_f205, &[n, 0, 4, s, 8, 0]).u32(), EINVAL);
        assert_eq!(e.mem.u32(errno), EINVAL);
        // No output: the length with its terminator.
        assert_eq!(e.call(0x00ec_f205, &[n, 0, 0, s, 8, 0]).u32(), 0);
        assert_eq!(e.mem.u32(n), 5);
        // A failed conversion answers errno.
        e.mem.set_u32(errno, EILSEQ);
        e.register(MBSTOWCS_L_HELPER, |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(e.call(0x00ec_f205, &[n, out, 8, s, 8, 0]).u32(), EILSEQ);
    }
}

//! The C runtime's string and conversion entry points that the platform stood in for (MSVC 2008
//! LIBCMT, statically linked into FalloutNV.exe 1.4.0.525), translated from the exe: `strlen`
//! (`00ec6130`), `strcpy` (`00ec6370`), `strchr` (`00ec7690`), `strcat_s` (`00ec6bd2`),
//! `strncpy_s` (`00ec8d5f`), `strncmp` (`00ec8a19`), `strcspn` (`00ecdbb0`), `tolower`
//! (`00ec67aa`), `toupper` (`00eca7f4`) with `__toupper_l` (`00eca6de`), `_stricmp`
//! (`00ec68e4`), `atol` (`00eca6d3`) and its body (`00eca6bd`), and `atof` (`00eca573`).
//!
//! `strlen`, `strcpy` and `strchr` are the word-at-a-time assembly versions: aligned dword
//! reads that never cross into a word past the one holding the terminator (or the match),
//! with a zero-byte test that only decides when the word's bytes are looked at one by one.
//! The bytes they answer for are the ones a byte loop finds, which is how they are written
//! here (the same is done for `strcat`, `00ec6380`, in `crt.rs`).

use super::*;

/// `_errno` (`00ec85e3`), the invalid-parameter report (`00ec7c30`), `_LocaleUpdate`
/// (`00ec660e`), `__isctype_l` (`00ed41e7`), `__isleadbyte_l` (`00ed419c`),
/// `___crtLCMapStringA` (`00ed0883`), `__tolower_l` (`00ec6695`), `__toupper_l` (`00eca6de`),
/// `___ascii_stricmp` (`00ec67d6`), `__stricmp_l` (`00ec680f`), `strtol` (`00ec8d08`), the
/// `atol` body (`00eca6bd`) and `_atof_l` (`00eca4c8`).
const ERRNO_ADDR: u32 = 0x00ec_85e3;
const INVALID_PARAMETER_ADDR: u32 = 0x00ec_7c30;
const LOCALE_UPDATE_ADDR: u32 = 0x00ec_660e;
const ISCTYPE_L_ADDR: u32 = 0x00ed_41e7;
const ISLEADBYTE_L_ADDR: u32 = 0x00ed_419c;
const CRT_LC_MAP_STRING_A_ADDR: u32 = 0x00ed_0883;
const TOLOWER_L: u32 = 0x00ec_6695;
const TOUPPER_L: u32 = 0x00ec_a6de;
const ASCII_STRICMP: u32 = 0x00ec_67d6;
const STRICMP_L: u32 = 0x00ec_680f;
const STRTOL_ADDR: u32 = 0x00ec_8d08;
const ATOL_BODY: u32 = 0x00ec_a6bd;
const ATOF_L: u32 = 0x00ec_a4c8;
/// `__locale_changed` (`0126fdf4`): 0 while the C locale is the global one.
const LOCALE_CHANGED_FLAG: u32 = 0x0126_fdf4;

/// `*_errno() = code` then the invalid-parameter report with five zeros, the exe's order.
fn report(e: &mut Engine, code: u32) {
    let slot = e.call(ERRNO_ADDR, &args![]).u32();
    e.mem.set_u32(slot, code);
    e.call(INVALID_PARAMETER_ADDR, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
}

// Translated from 00ec6130 (disassembled, FalloutNV.exe 1.4.0.525)
/// `strlen` (Ghidra's function ID), cdecl: the number of bytes before the terminating zero
/// (bytes up to a 4-aligned address, then dwords until one holds the zero).
pub fn strlen(e: &mut Engine, text: u32) -> u32 {
    let mut at = text;
    while e.mem.u8(at) != 0 {
        at = at.wrapping_add(1);
    }
    at.wrapping_sub(text)
}

// Translated from 00ec6370 (disassembled, FalloutNV.exe 1.4.0.525)
/// `strcpy` (Ghidra's function ID), cdecl: destination, source; the destination as `EDI` and a
/// jump into `strcat`'s copy (`00ec63e5`): the source's bytes with their terminating zero are
/// copied to the destination, which is answered.
pub fn strcpy(e: &mut Engine, dst: u32, src: u32) -> u32 {
    let (mut d, mut s) = (dst, src);
    loop {
        let b = e.mem.u8(s);
        e.mem.set_u8(d, b);
        if b == 0 {
            return dst;
        }
        d = d.wrapping_add(1);
        s = s.wrapping_add(1);
    }
}

// Translated from 00ec7690 (disassembled, FalloutNV.exe 1.4.0.525)
/// `strchr` (Ghidra's function ID), cdecl: text, character; the address of the first byte equal
/// to the character's low byte, checked before the terminator (so 0 finds the terminator), or
/// null.
pub fn strchr(e: &mut Engine, text: u32, character: u32) -> u32 {
    let wanted = character as u8;
    let mut at = text;
    loop {
        let b = e.mem.u8(at);
        if b == wanted {
            return at;
        }
        if b == 0 {
            return 0;
        }
        at = at.wrapping_add(1);
    }
}

// Translated from 00ec6bd2 (disassembled, FalloutNV.exe 1.4.0.525)
/// `strcat_s` (Ghidra's function ID), cdecl: destination, size, source; answers 0 or an error.
/// A null destination or a size of 0 is `EINVAL` (0x16) with the report. A null source empties
/// the destination and is `EINVAL`. The destination's end is found within the size (none:
/// emptied, `EINVAL`), then the source is copied with its zero while room is left; running out
/// of room empties the destination and is `ERANGE` (0x22) with the report.
pub fn strcat_s(e: &mut Engine, dst: u32, size: u32, src: u32) -> u32 {
    if dst != 0 && size != 0 {
        if src != 0 {
            let mut at = dst;
            let mut room = size;
            loop {
                if e.mem.u8(at) == 0 {
                    break;
                }
                at = at.wrapping_add(1);
                room -= 1;
                if room == 0 {
                    break;
                }
            }
            if room != 0 {
                let mut from = src;
                loop {
                    let b = e.mem.u8(from);
                    e.mem.set_u8(at, b);
                    at = at.wrapping_add(1);
                    from = from.wrapping_add(1);
                    if b == 0 {
                        break;
                    }
                    room -= 1;
                    if room == 0 {
                        break;
                    }
                }
                if room != 0 {
                    return 0;
                }
                e.mem.set_u8(dst, 0);
                report(e, 0x22);
                return 0x22;
            }
        }
        e.mem.set_u8(dst, 0);
    }
    report(e, 0x16);
    0x16
}

// Translated from 00ec8d5f (disassembled, FalloutNV.exe 1.4.0.525)
/// `strncpy_s` (Ghidra's function ID), cdecl: destination, size, source, count; answers 0, 0x50
/// (`STRUNCATE`) or an error. A count of 0 with a null destination and size 0 answers 0.
/// Otherwise a null destination, or a size of 0, is `EINVAL` (0x16) with the report; a count
/// of 0 empties the destination and answers 0; a null source empties it and is `EINVAL`. The
/// copy: with the count `_TRUNCATE` (-1) the source and its zero while room is left; otherwise
/// at most `count` bytes (stopping after a zero or when the room runs out), and a terminating
/// zero after them when the count ran out. Room left answers 0. Out of room: with `_TRUNCATE`
/// the last byte of the destination becomes the zero and 0x50 is answered; otherwise the
/// destination is emptied and it is `ERANGE` (0x22) with the report.
pub fn strncpy_s(e: &mut Engine, dst: u32, size: u32, src: u32, count: u32) -> u32 {
    if count == 0 && dst == 0 && size == 0 {
        return 0;
    }
    if dst != 0 && size != 0 {
        if count == 0 {
            e.mem.set_u8(dst, 0);
            return 0;
        }
        if src != 0 {
            let mut at = dst;
            let mut from = src;
            let mut room = size;
            let mut left = count;
            if count == 0xffff_ffff {
                loop {
                    let b = e.mem.u8(from);
                    e.mem.set_u8(at, b);
                    at = at.wrapping_add(1);
                    from = from.wrapping_add(1);
                    if b == 0 {
                        break;
                    }
                    room -= 1;
                    if room == 0 {
                        break;
                    }
                }
            } else {
                loop {
                    let b = e.mem.u8(from);
                    e.mem.set_u8(at, b);
                    at = at.wrapping_add(1);
                    from = from.wrapping_add(1);
                    if b == 0 {
                        break;
                    }
                    room -= 1;
                    if room == 0 {
                        break;
                    }
                    left -= 1;
                    if left == 0 {
                        break;
                    }
                }
                if left == 0 {
                    e.mem.set_u8(at, 0);
                }
            }
            if room != 0 {
                return 0;
            }
            if count == 0xffff_ffff {
                e.mem.set_u8(dst.wrapping_add(size).wrapping_sub(1), 0);
                return 0x50;
            }
            e.mem.set_u8(dst, 0);
            report(e, 0x22);
            return 0x22;
        }
        e.mem.set_u8(dst, 0);
    }
    report(e, 0x16);
    0x16
}

// Translated from 00ec8a19 (disassembled, FalloutNV.exe 1.4.0.525)
/// `strncmp` (Ghidra's function ID), cdecl: two strings and a count; answers the difference of
/// the first pair of bytes (unsigned) that differ or where the first string ends, within the
/// count, else 0. With a count above 4 the first `count - 4` positions (rounded up to a whole
/// group of four) go four at a time, the rest one at a time; the bytes compared are the same.
pub fn strncmp(e: &mut Engine, a: u32, b: u32, count: u32) -> i32 {
    let mut i = 0u32;
    while i < count {
        let x = e.mem.u8(a.wrapping_add(i));
        let y = e.mem.u8(b.wrapping_add(i));
        if x == 0 || x != y {
            return i32::from(x) - i32::from(y);
        }
        i += 1;
    }
    0
}

// Translated from 00ecdbb0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `strcspn` (Ghidra's function ID), cdecl: text, control; the length of the text's first run
/// of bytes that are not in the control set (a 256-bit map on the stack, `BTS` for each control
/// byte; the terminator is never in it).
pub fn strcspn(e: &mut Engine, text: u32, control: u32) -> u32 {
    let mut map = [0u8; 32];
    let mut at = control;
    loop {
        let c = e.mem.u8(at);
        if c == 0 {
            break;
        }
        at = at.wrapping_add(1);
        map[usize::from(c >> 3)] |= 1 << (c & 7);
    }
    let mut count = 0xffff_ffffu32;
    let mut at = text;
    loop {
        count = count.wrapping_add(1);
        let c = e.mem.u8(at);
        if c == 0 {
            return count;
        }
        at = at.wrapping_add(1);
        if map[usize::from(c >> 3)] & (1 << (c & 7)) != 0 {
            return count;
        }
    }
}

// Translated from 00ec67aa (disassembled, FalloutNV.exe 1.4.0.525)
/// `tolower` (Ghidra's function ID), cdecl: with the C locale (`__locale_changed` 0) `'A'` to
/// `'Z'` (an unsigned range test of `c - 0x41`) get 0x20 added and anything else is answered as
/// it is; otherwise `__tolower_l(c, NULL)`.
pub fn tolower(e: &mut Engine, c: u32) -> u32 {
    if e.mem.u32(LOCALE_CHANGED_FLAG) != 0 {
        return e.call(TOLOWER_L, &args![c, 0u32]).u32();
    }
    if c.wrapping_sub(0x41) <= 0x19 {
        c.wrapping_add(0x20)
    } else {
        c
    }
}

// Translated from 00eca7f4 (disassembled, FalloutNV.exe 1.4.0.525)
/// `toupper` (Ghidra's function ID), cdecl: with the C locale `'a'` to `'z'` lose 0x20, anything
/// else is answered as it is; otherwise `__toupper_l(c, NULL)`.
pub fn toupper(e: &mut Engine, c: u32) -> u32 {
    if e.mem.u32(LOCALE_CHANGED_FLAG) != 0 {
        return e.call(TOUPPER_L, &args![c, 0u32]).u32();
    }
    if c.wrapping_sub(0x61) <= 0x19 {
        c.wrapping_sub(0x20)
    } else {
        c
    }
}

// Translated from 00eca6de (disassembled, FalloutNV.exe 1.4.0.525)
/// `__toupper_l` (Ghidra's function ID), cdecl: character, locale. With the locale taken
/// (`_LocaleUpdate`, `EBP-0x18`): a character below 0x100 that is lower case (`_LOWER`, bit 2:
/// through `__isctype_l` when `mb_cur_max` (+0xAC) is above 1, else the ctype table at +0xC8)
/// becomes its entry in the upper-case map (+0xD0); other characters below 0x100 stay. A
/// larger one is two bytes (high, low) when `mb_cur_max` is above 1 and its high byte (`SAR 8`)
/// is a lead byte; otherwise `errno` becomes `EILSEQ` (0x2A) and only its low byte is mapped.
/// The bytes go through `___crtLCMapStringA(update, LC_CTYPE handle (+0x14), LCMAP_UPPERCASE,
/// bytes, count, out (`EBP-8`), 3, code page (+4), 1)`: 0 keeps the character, 1 answers the
/// first byte out, more the first two (high first).
pub fn toupper_l(e: &mut Engine, c: u32, locale: u32) -> u32 {
    e.with_stack(0x18, |e, frame| {
        let update = frame.addr();
        let out = update + 0x10;
        let source = update + 0x14;
        e.call(LOCALE_UPDATE_ADDR, &args![update, locale]);
        let info = e.mem.u32(update);
        let mapped = if c < 0x100 {
            let lower = if e.mem.u32(info + 0xac) as i32 > 1 {
                e.call(ISCTYPE_L_ADDR, &args![c, 2u32, update]).u32()
            } else {
                let table = e.mem.u32(info + 0xc8);
                u32::from(e.mem.u16(table.wrapping_add(c.wrapping_mul(2)))) & 2
            };
            if lower != 0 {
                let info = e.mem.u32(update);
                let map = e.mem.u32(info + 0xd0);
                Some(u32::from(e.mem.u8(map.wrapping_add(c))))
            } else {
                None
            }
        } else {
            let high = ((c as i32) >> 8) as u32;
            let lead = e.mem.u32(info + 0xac) as i32 > 1
                && e.call(ISLEADBYTE_L_ADDR, &args![high & 0xff, update]).u32() != 0;
            let count = if lead {
                e.mem.set_u8(source, high as u8);
                e.mem.set_u8(source + 1, c as u8);
                e.mem.set_u8(source + 2, 0);
                2u32
            } else {
                let slot = e.call(ERRNO_ADDR, &args![]).u32();
                e.mem.set_u32(slot, 0x2a);
                e.mem.set_u8(source, c as u8);
                e.mem.set_u8(source + 1, 0);
                1u32
            };
            let info = e.mem.u32(update);
            let lcid = e.mem.u32(info + 0x14);
            let code_page = e.mem.u32(info + 4);
            let written = e
                .call(
                    CRT_LC_MAP_STRING_A_ADDR,
                    &args![update, lcid, 0x200u32, source, count, out, 3u32, code_page, 1u32],
                )
                .u32();
            match written {
                0 => None,
                1 => Some(u32::from(e.mem.u8(out))),
                _ => Some((u32::from(e.mem.u8(out)) << 8) | u32::from(e.mem.u8(out + 1))),
            }
        };
        locale_update_release(e, update);
        mapped.unwrap_or(c)
    })
}

// Translated from 00ec68e4 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_stricmp` (Ghidra's function ID), cdecl: two strings. With a changed locale
/// `__stricmp_l(a, b, NULL)` answers. Otherwise a null string is `EINVAL` (0x16) with the
/// report and `0x7fffffff`; two strings go to `___ascii_stricmp` (a jump: its answer).
pub fn stricmp(e: &mut Engine, a: u32, b: u32) -> i32 {
    if e.mem.u32(LOCALE_CHANGED_FLAG) != 0 {
        return e.call(STRICMP_L, &args![a, b, 0u32]).i32();
    }
    if a == 0 || b == 0 {
        report(e, 0x16);
        return 0x7fff_ffff;
    }
    e.call(ASCII_STRICMP, &args![a, b]).i32()
}

/// `__strnicmp_l` (`00ec7dce`) and `___ascii_strnicmp` (`00ed5bc0`), cdecl.
const STRNICMP_L: u32 = 0x00ec_7dce;
const ASCII_STRNICMP: u32 = 0x00ed_5bc0;

// Translated from 00ec7ec0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_strnicmp` (no label; named from its calls), cdecl: two strings and a count. With a changed
/// locale `__strnicmp_l(a, b, count, NULL)` answers. Otherwise a null string, or a count above
/// `0x7fffffff`, is `EINVAL` (0x16) with the report and `0x7fffffff`; else `___ascii_strnicmp`
/// (a jump: its answer).
pub fn strnicmp(e: &mut Engine, a: u32, b: u32, count: u32) -> i32 {
    if e.mem.u32(LOCALE_CHANGED_FLAG) != 0 {
        return e.call(STRNICMP_L, &args![a, b, count, 0u32]).i32();
    }
    if a == 0 || b == 0 || count > 0x7fff_ffff {
        report(e, 0x16);
        return 0x7fff_ffff;
    }
    e.call(ASCII_STRNICMP, &args![a, b, count]).i32()
}

// Translated from 00ec3bad (disassembled, FalloutNV.exe 1.4.0.525)
/// `_byteswap_ulong` (no label), cdecl: `((v & 0xff00) + (v << 16)) << 8`, plus `(v >> 8) &
/// 0xff00`, plus `v >> 24`: the bytes reversed.
pub fn byteswap_ulong(_e: &mut Engine, value: u32) -> u32 {
    (value & 0xff00)
        .wrapping_add(value << 16)
        .wrapping_shl(8)
        .wrapping_add((value >> 8) & 0xff00)
        .wrapping_add(value >> 24)
}

// Translated from 00eca6bd (disassembled, FalloutNV.exe 1.4.0.525)
/// `atol`'s body (no label; `atol` jumps here), cdecl: `strtol(text, NULL, 10)`.
pub fn atol_body(e: &mut Engine, text: u32) -> i32 {
    e.call(STRTOL_ADDR, &args![text, 0u32, 10u32]).i32()
}

// Translated from 00eca6d3 (disassembled, FalloutNV.exe 1.4.0.525)
/// `atol` (Ghidra's function ID), cdecl: a jump to its body (`00eca6bd`).
pub fn atol(e: &mut Engine, text: u32) -> i32 {
    e.call(ATOL_BODY, &args![text]).i32()
}

// Translated from 00eca573 (disassembled, FalloutNV.exe 1.4.0.525)
/// `atof` (Ghidra's function ID), cdecl: `_atof_l(text, NULL)`, the double in `ST0`.
pub fn atof(e: &mut Engine, text: u32) -> f64 {
    e.call(ATOF_L, &args![text, 0u32]).f64()
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_6130, strlen(u32) -> u32),
        entry!(0x00ec_6370, strcpy(u32, u32) -> u32),
        entry!(0x00ec_7690, strchr(u32, u32) -> u32),
        entry!(0x00ec_6bd2, strcat_s(u32, u32, u32) -> u32),
        entry!(0x00ec_8d5f, strncpy_s(u32, u32, u32, u32) -> u32),
        entry!(0x00ec_8a19, strncmp(u32, u32, u32) -> i32),
        entry!(0x00ec_dbb0, strcspn(u32, u32) -> u32),
        entry!(0x00ec_67aa, tolower(u32) -> u32),
        entry!(0x00ec_a7f4, toupper(u32) -> u32),
        entry!(0x00ec_a6de, toupper_l(u32, u32) -> u32),
        entry!(0x00ec_68e4, stricmp(u32, u32) -> i32),
        entry!(0x00ec_7ec0, strnicmp(u32, u32, u32) -> i32),
        entry!(0x00ec_3bad, byteswap_ulong(u32) -> u32),
        entry!(0x00ec_a6bd, atol_body(u32) -> i32),
        entry!(0x00ec_a6d3, atol(u32) -> i32),
        entry!(0x00ec_a573, atof(u32) -> f64),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(e: &mut Engine, t: &[u8]) -> u32 {
        let at = e.mem.alloc(t.len() as u32 + 8);
        e.mem.set_cstr(at, t);
        at
    }

    /// An engine with `errno` doubled to one slot and the report doubled; the slot.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO_ADDR, move |_, _| slot.into_ret());
        e.register(INVALID_PARAMETER_ADDR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, slot)
    }

    fn reports(e: &Engine) -> usize {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == INVALID_PARAMETER_ADDR)
            .count()
    }

    #[test]
    fn strlen_strcpy_strchr() {
        let (mut e, _) = engine();
        let s = text(&mut e, b"Vault 21");
        assert_eq!(e.call(0x00ec_6130, &args![s]).u32(), 8);
        assert_eq!(strlen(&mut e, s + 3), 5);
        let d = e.mem.alloc(16);
        assert_eq!(strcpy(&mut e, d, s), d);
        assert_eq!(e.mem.cstr(d), b"Vault 21".to_vec());
        assert_eq!(strchr(&mut e, s, u32::from(b'2')), s + 6);
        assert_eq!(strchr(&mut e, s, 0x100 + u32::from(b't')), s + 4);
        assert_eq!(strchr(&mut e, s, 0), s + 8);
        assert_eq!(strchr(&mut e, s, u32::from(b'x')), 0);
    }

    #[test]
    fn strcat_s_appends_or_reports() {
        let (mut e, slot) = engine();
        let d = e.mem.alloc(16);
        e.mem.set_cstr(d, b"New ");
        let s = text(&mut e, b"Vegas");
        assert_eq!(strcat_s(&mut e, d, 16, s), 0);
        assert_eq!(e.mem.cstr(d), b"New Vegas".to_vec());
        // Exactly the room for the text and its zero.
        e.mem.set_cstr(d, b"New ");
        assert_eq!(strcat_s(&mut e, d, 10, s), 0);
        // One byte short: emptied, ERANGE.
        e.mem.set_cstr(d, b"New ");
        assert_eq!(strcat_s(&mut e, d, 9, s), 0x22);
        assert_eq!(e.mem.u8(d), 0);
        assert_eq!(e.mem.u32(slot), 0x22);
        // No end within the size, a null source, a null destination: EINVAL.
        e.mem.set_cstr(d, b"New ");
        assert_eq!(strcat_s(&mut e, d, 3, s), 0x16);
        assert_eq!(e.mem.u8(d), 0);
        assert_eq!(strcat_s(&mut e, d, 16, 0), 0x16);
        assert_eq!(strcat_s(&mut e, 0, 16, s), 0x16);
        assert_eq!(strcat_s(&mut e, d, 0, s), 0x16);
        assert_eq!(reports(&e), 5);
    }

    #[test]
    fn strncpy_s_copies_truncates_or_reports() {
        let (mut e, slot) = engine();
        let d = e.mem.alloc(16);
        let s = text(&mut e, b"Mojave");
        assert_eq!(strncpy_s(&mut e, d, 16, s, 3), 0);
        assert_eq!(e.mem.cstr(d), b"Moj".to_vec());
        assert_eq!(strncpy_s(&mut e, d, 16, s, 10), 0);
        assert_eq!(e.mem.cstr(d), b"Mojave".to_vec());
        // _TRUNCATE into too little room: the last byte ends it, STRUNCATE.
        assert_eq!(strncpy_s(&mut e, d, 4, s, 0xffff_ffff), 0x50);
        assert_eq!(e.mem.cstr(d), b"Moj".to_vec());
        // A count that does not fit: emptied, ERANGE.
        assert_eq!(strncpy_s(&mut e, d, 4, s, 5), 0x22);
        assert_eq!(e.mem.u8(d), 0);
        assert_eq!(e.mem.u32(slot), 0x22);
        // A count of 0 empties; nothing at all is fine; null pieces are EINVAL.
        e.mem.set_u8(d, b'x');
        assert_eq!(strncpy_s(&mut e, d, 4, s, 0), 0);
        assert_eq!(e.mem.u8(d), 0);
        assert_eq!(strncpy_s(&mut e, 0, 0, s, 0), 0);
        assert_eq!(strncpy_s(&mut e, 0, 4, s, 0), 0x16);
        assert_eq!(strncpy_s(&mut e, d, 0, s, 3), 0x16);
        assert_eq!(strncpy_s(&mut e, d, 4, 0, 3), 0x16);
        assert_eq!(reports(&e), 4);
    }

    #[test]
    fn strncmp_and_strcspn() {
        let (mut e, _) = engine();
        let a = text(&mut e, b"Benny's chip");
        let b = text(&mut e, b"Benny's chop");
        assert_eq!(strncmp(&mut e, a, b, 9), 0);
        assert_eq!(strncmp(&mut e, a, b, 12), i32::from(b'i') - i32::from(b'o'));
        assert_eq!(e.call(0x00ec_8a19, &args![b, a, 20u32]).i32(), 6);
        let c = text(&mut e, b"Ben");
        assert_eq!(strncmp(&mut e, c, a, 5), -i32::from(b'n'));
        assert_eq!(strncmp(&mut e, a, b, 0), 0);
        let set = text(&mut e, b"' ");
        assert_eq!(strcspn(&mut e, a, set), 5);
        let none = text(&mut e, b"");
        assert_eq!(strcspn(&mut e, a, none), 12);
    }

    #[test]
    fn case_mapping_in_the_c_locale() {
        let (mut e, _) = engine();
        assert_eq!(tolower(&mut e, u32::from(b'Q')), u32::from(b'q'));
        assert_eq!(tolower(&mut e, u32::from(b'q')), u32::from(b'q'));
        assert_eq!(tolower(&mut e, 0xc4), 0xc4);
        assert_eq!(toupper(&mut e, u32::from(b'q')), u32::from(b'Q'));
        assert_eq!(toupper(&mut e, 0xffff_ffff), 0xffff_ffff);
        // A changed locale goes to the locale forms.
        e.mem.set_u32(LOCALE_CHANGED_FLAG, 1);
        e.register(TOLOWER_L, |_, a| (a[0] + 1).into_ret());
        e.register(TOUPPER_L, |_, a| (a[0] + 2).into_ret());
        assert_eq!(tolower(&mut e, 5), 6);
        assert_eq!(toupper(&mut e, 5), 7);
    }

    #[test]
    fn toupper_l_maps_through_the_locale_tables() {
        let (mut e, _) = engine();
        let info = e.mem.alloc(0x100);
        let ctype = e.mem.alloc(0x200);
        let upper = e.mem.alloc(0x100);
        e.mem.set_u32(info + 0xac, 1);
        e.mem.set_u32(info + 0xc8, ctype);
        e.mem.set_u32(info + 0xd0, upper);
        e.mem.set_u16(ctype + 2 * 0x61, 2);
        e.mem.set_u8(upper + 0x61, 0x41);
        e.register_double(LOCALE_UPDATE_ADDR, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u8(a[0] + 0xc, 0);
            a[0].into_ret()
        });
        assert_eq!(toupper_l(&mut e, 0x61, 0), 0x41);
        assert_eq!(toupper_l(&mut e, 0x62, 0), 0x62);
        // A wide character: EILSEQ and the low byte through LCMapString (answering 1 byte).
        e.register(CRT_LC_MAP_STRING_A_ADDR, |e, a| {
            assert_eq!((a[2], a[4], a[6]), (0x200, 1, 3));
            e.mem.set_u8(a[5], 0x5a);
            1u32.into_ret()
        });
        assert_eq!(toupper_l(&mut e, 0x17a, 0), 0x5a);
    }

    #[test]
    fn stricmp_and_strnicmp_check_their_arguments() {
        let (mut e, slot) = engine();
        e.register(ASCII_STRICMP, |_, _| 3u32.into_ret());
        e.register(ASCII_STRNICMP, |_, _| 4u32.into_ret());
        let a = text(&mut e, b"a");
        assert_eq!(stricmp(&mut e, a, a), 3);
        assert_eq!(stricmp(&mut e, 0, a), 0x7fff_ffff);
        assert_eq!(stricmp(&mut e, a, 0), 0x7fff_ffff);
        assert_eq!(strnicmp(&mut e, a, a, 2), 4);
        assert_eq!(strnicmp(&mut e, a, a, 0x8000_0000), 0x7fff_ffff);
        assert_eq!(e.mem.u32(slot), 0x16);
        assert_eq!(reports(&e), 3);
        e.mem.set_u32(LOCALE_CHANGED_FLAG, 1);
        e.register(STRICMP_L, |_, a| (a[2] + 9).into_ret());
        e.register(STRNICMP_L, |_, a| (a[3] + 8).into_ret());
        assert_eq!(stricmp(&mut e, 0, 0), 9);
        assert_eq!(strnicmp(&mut e, 0, 0, 5), 8);
    }

    #[test]
    fn atol_and_atof_pass_their_text_on() {
        let (mut e, _) = engine();
        e.register(STRTOL_ADDR, |_, a| (a[0] + a[2]).into_ret());
        e.register(ATOF_L, |_, a| Ret {
            st0: f64::from(a[0]) + f64::from(a[1]),
            ..Ret::default()
        });
        assert_eq!(atol(&mut e, 0x100), 0x10a);
        assert_eq!(e.call(0x00ec_a6bd, &args![0x200u32]).i32(), 0x20a);
        assert_eq!(atof(&mut e, 0x300), 768.0);
    }

    #[test]
    fn byteswap_reverses_the_bytes() {
        let (mut e, _) = engine();
        assert_eq!(byteswap_ulong(&mut e, 0x1234_5678), 0x7856_3412);
        assert_eq!(byteswap_ulong(&mut e, 0xff00_00ff), 0xff00_00ff);
    }
}

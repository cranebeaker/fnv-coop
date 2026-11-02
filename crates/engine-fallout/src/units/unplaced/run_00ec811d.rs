//! `(unplaced)/run_00ec811d` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`, thiscall, one word: the `_locale_t`).
const LOCALE_UPDATE: u32 = 0x00ec_660e;
/// `_errno` (`00ec85e3`): the address of the thread's `errno`.
const ERRNO: u32 = 0x00ec_85e3;
/// `_invalid_parameter` (`00ec7c30`, five words, all zero here).
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
/// `_strcmp` (`00ec6da0`, cdecl: two strings).
const STRCMP: u32 = 0x00ec_6da0;
/// `__stricmp_l` (`00ec680f`, cdecl: two strings and a `_locale_t`).
const STRICMP_L: u32 = 0x00ec_680f;
/// `___crtLCMapStringA` (`00ed0883`, cdecl, nine words).
const CRT_LC_MAP_STRING_A: u32 = 0x00ed_0883;
/// `__mbsdec` (`00ed5d3e`, cdecl: start, current): the character before `current`.
const MBSDEC: u32 = 0x00ed_5d3e;
/// `LCMAP_UPPERCASE`, the mapping `_mbsicmp_l` asks `___crtLCMapStringA` for.
const LCMAP_UPPERCASE: u32 = 0x200;

/// The `_LocaleUpdate` local (0x10 bytes): the `_locale_tstruct` (`locinfo` at +0, `mbcinfo`
/// at +4), the thread data at +8 and the "updated" byte at +0xc.
const LOCALE_UPDATE_SIZE: u32 = 0x10;
const LOCALE_UPDATE_MBCINFO: u32 = 0x4;
const LOCALE_UPDATE_THREAD_DATA: u32 = 0x8;
const LOCALE_UPDATE_UPDATED: u32 = 0xc;
/// `threadmbcinfostruct` fields the code reads: `mbcodepage` (+0x4), `ismbcodepage` (+0x8),
/// `mblcid` (+0xc), the `mbctype` table (+0x1d, indexed by byte) and the `mbcasemap` table
/// (+0x11d, indexed by byte).
const MBCINFO_MBCODEPAGE: u32 = 0x4;
const MBCINFO_ISMBCODEPAGE: u32 = 0x8;
const MBCINFO_MBLCID: u32 = 0xc;
const MBCINFO_MBCTYPE: u32 = 0x1d;
const MBCINFO_MBCASEMAP: u32 = 0x11d;
/// `mbctype` bits: lead byte (`_M1`, 4) and lower-case letter (`_SBLOW`, 0x10).
const MBCTYPE_LEAD_BYTE: u8 = 0x4;
const MBCTYPE_LOWER: u8 = 0x10;

/// The end of each path through a function with a `_LocaleUpdate`: when it changed the
/// thread's locale (its byte at +0xc), clears bit 1 of the thread data's word at +0x70.
fn locale_update_end(e: &mut Engine, update: u32) {
    if e.mem.u8(update + LOCALE_UPDATE_UPDATED) != 0 {
        let thread_data = e.mem.u32(update + LOCALE_UPDATE_THREAD_DATA);
        let flags = e.mem.u32(thread_data + 0x70);
        e.mem.set_u32(thread_data + 0x70, flags & 0xffff_fffd);
    }
}

/// Sets `errno` to `error` and calls `_invalid_parameter(0, 0, 0, 0, 0)`.
fn invalid_parameter(e: &mut Engine, error: u32) {
    let errno = e.call(ERRNO, &args![]).u32();
    e.mem.set_u32(errno, error);
    e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
}

/// The result of the multibyte comparisons at the first differing character: `SBB`/`AND 2`/
/// `DEC` on `CMP second, first`, so 1 when the second string's character is below the
/// first's (unsigned 16-bit), -1 otherwise.
fn order(first: u16, second: u16) -> i32 {
    if second < first {
        1
    } else {
        -1
    }
}

/// One character of `_mbscmp_l`'s strings at `*p` (advanced past it): a lead byte joins the
/// next byte as `lead << 8 | next`, or reads as 0 when the next byte is 0 (the pointer then
/// stays on that 0).
fn mbscmp_char(e: &mut Engine, mbcinfo: u32, p: &mut u32) -> u16 {
    let lead = e.mem.u8(*p);
    *p = p.wrapping_add(1);
    if e.mem.u8(mbcinfo + MBCINFO_MBCTYPE + u32::from(lead)) & MBCTYPE_LEAD_BYTE == 0 {
        return u16::from(lead);
    }
    let next = e.mem.u8(*p);
    if next == 0 {
        return 0;
    }
    *p = p.wrapping_add(1);
    (u16::from(lead) << 8) | u16::from(next)
}

// Translated from 00ec811d (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mbscmp_l` (Ghidra library match; cdecl, three words). With a `_LocaleUpdate` for
/// `locale` on the stack: a null string sets `errno` to `0x16`, calls
/// `_invalid_parameter(0, 0, 0, 0, 0)` and returns `0x7fffffff`. When the locale's code page is
/// not multibyte (`mbcinfo + 8` is 0) it returns `_strcmp(first, second)`. Otherwise it reads
/// one character of each string at a time (`mbscmp_char`) and returns 1 or -1 at the first
/// difference, 0 when both end together.
pub fn mbscmp_l(e: &mut Engine, first: Ptr, second: Ptr, locale: u32) -> i32 {
    e.with_stack(LOCALE_UPDATE_SIZE, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        if first.is_null() || second.is_null() {
            invalid_parameter(e, 0x16);
            locale_update_end(e, update);
            return 0x7fff_ffff;
        }
        let mbcinfo = e.mem.u32(update + LOCALE_UPDATE_MBCINFO);
        if e.mem.u32(mbcinfo + MBCINFO_ISMBCODEPAGE) == 0 {
            let result = e.call(STRCMP, &args![first, second]).i32();
            locale_update_end(e, update);
            return result;
        }
        let (mut a, mut b) = (first.addr(), second.addr());
        loop {
            let first_char = mbscmp_char(e, mbcinfo, &mut a);
            let second_char = mbscmp_char(e, mbcinfo, &mut b);
            if second_char != first_char {
                let result = order(first_char, second_char);
                locale_update_end(e, update);
                return result;
            }
            if first_char == 0 {
                locale_update_end(e, update);
                return 0;
            }
        }
    })
}

// Translated from 00ec8244 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mbscmp` (Ghidra library match; cdecl, two words): `__mbscmp_l(first, second, 0)`.
pub fn mbscmp(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    mbscmp_l(e, first, second, 0)
}

// Translated from 00ec825b (decompiled, FalloutNV.exe 1.4.0.525)
/// `__makepath_s` (Ghidra library match; cdecl, six words): builds `drive:`, the directory
/// (followed by `\` unless its last character, found with `__mbsdec`, is `/` or `\`), the file
/// name and the extension (with a `.` put in front unless it starts with one) into `result` of
/// `size` bytes, each part only when it is non-null and (drive, directory, extension) non-empty.
/// A null `result` or a zero `size` sets `errno` to `0x16`, calls `_invalid_parameter` and
/// returns `0x16`. When the next character (or the closing 0) would not fit, `result` becomes an
/// empty string, `errno` is set to `0x22`, `_invalid_parameter` is called and `0x22` is returned.
/// Otherwise the path is closed with a 0 and the result is 0.
pub fn makepath_s(
    e: &mut Engine,
    result: Ptr,
    size: u32,
    drive: Ptr,
    dir: Ptr,
    file_name: Ptr,
    ext: Ptr,
) -> u32 {
    if result.is_null() || size == 0 {
        invalid_parameter(e, 0x16);
        return 0x16;
    }
    match makepath_parts(e, result.addr(), size, drive, dir, file_name, ext) {
        Some(()) => 0,
        None => {
            e.mem.set_u8(result.addr(), 0);
            invalid_parameter(e, 0x22);
            0x22
        }
    }
}

/// The copying part of [`makepath_s`]; `None` where the exe jumps to its overflow path.
fn makepath_parts(
    e: &mut Engine,
    result: u32,
    size: u32,
    drive: Ptr,
    dir: Ptr,
    file_name: Ptr,
    ext: Ptr,
) -> Option<()> {
    let mut out = result;
    let mut length = 0u32;
    let mut put = |e: &mut Engine, byte: u8| {
        e.mem.set_u8(out, byte);
        out = out.wrapping_add(1);
    };
    if !drive.is_null() {
        let letter = e.mem.u8(drive.addr());
        if letter != 0 {
            length = 2;
            if size <= 2 {
                return None;
            }
            put(e, letter);
            put(e, b':');
        }
    }
    if !dir.is_null() && e.mem.u8(dir.addr()) != 0 {
        let mut p = dir.addr();
        loop {
            length += 1;
            if length >= size {
                return None;
            }
            let byte = e.mem.u8(p);
            put(e, byte);
            p = p.wrapping_add(1);
            if e.mem.u8(p) == 0 {
                break;
            }
        }
        let last = e.call(MBSDEC, &args![dir, p]).u32();
        let last = e.mem.u8(last);
        if last != b'/' && last != b'\\' {
            length += 1;
            if length >= size {
                return None;
            }
            put(e, b'\\');
        }
    }
    if !file_name.is_null() {
        let mut p = file_name.addr();
        while e.mem.u8(p) != 0 {
            length += 1;
            if length >= size {
                return None;
            }
            let byte = e.mem.u8(p);
            put(e, byte);
            p = p.wrapping_add(1);
        }
    }
    if !ext.is_null() {
        let mut p = ext.addr();
        let first = e.mem.u8(p);
        if first != 0 {
            if first != b'.' {
                length += 1;
                if length >= size {
                    return None;
                }
                put(e, b'.');
            }
            while e.mem.u8(p) != 0 {
                length += 1;
                if length >= size {
                    return None;
                }
                let byte = e.mem.u8(p);
                put(e, byte);
                p = p.wrapping_add(1);
            }
        }
    }
    length += 1;
    if length > size {
        return None;
    }
    put(e, 0);
    Some(())
}

/// One character of `_mbsicmp_l`'s strings at `*p` (advanced past it), upper-cased: a lead
/// byte followed by 0 reads as 0 (the pointer stays on the 0); a lead byte with its trail byte
/// goes through `___crtLCMapStringA(update, mblcid, LCMAP_UPPERCASE, p, 2, buffer, 2,
/// mbcodepage, 1)` (one byte back reads as that byte, two as `buffer[0] * 256 + buffer[1]`,
/// any other count is `None`, the error path); a lower-case single byte is mapped through
/// `mbcasemap`.
fn mbsicmp_char(
    e: &mut Engine,
    update: u32,
    buffer: u32,
    mbcinfo: u32,
    p: &mut u32,
) -> Option<u16> {
    let byte = e.mem.u8(*p);
    *p = p.wrapping_add(1);
    let flags = e.mem.u8(mbcinfo + MBCINFO_MBCTYPE + u32::from(byte));
    if flags & MBCTYPE_LEAD_BYTE != 0 {
        if e.mem.u8(*p) == 0 {
            return Some(0);
        }
        let lcid = e.mem.u32(mbcinfo + MBCINFO_MBLCID);
        let code_page = e.mem.u32(mbcinfo + MBCINFO_MBCODEPAGE);
        let source = p.wrapping_sub(1);
        let mapped = e
            .call(
                CRT_LC_MAP_STRING_A,
                &args![
                    update,
                    lcid,
                    LCMAP_UPPERCASE,
                    source,
                    2u32,
                    buffer,
                    2u32,
                    code_page,
                    1u32
                ],
            )
            .u32();
        let character = match mapped {
            1 => u16::from(e.mem.u8(buffer)),
            2 => {
                let high = u16::from(e.mem.u8(buffer)).wrapping_mul(0x100);
                high.wrapping_add(u16::from(e.mem.u8(buffer + 1)))
            }
            _ => return None,
        };
        *p = p.wrapping_add(1);
        Some(character)
    } else if flags & MBCTYPE_LOWER != 0 {
        Some(u16::from(
            e.mem.u8(mbcinfo + MBCINFO_MBCASEMAP + u32::from(byte)),
        ))
    } else {
        Some(u16::from(byte))
    }
}

// Translated from 00ec8364 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mbsicmp_l` (Ghidra library match; cdecl, three words). With a `_LocaleUpdate` for
/// `locale` and a two-byte buffer on the stack: a null string sets `errno` to `0x16`, calls
/// `_invalid_parameter(0, 0, 0, 0, 0)` and returns `0x7fffffff`. When the locale's code page
/// is not multibyte (`mbcinfo + 8` is 0) it returns `__stricmp_l(first, second, &update)`.
/// Otherwise it compares one upper-cased character of each string at a time (see
/// `mbsicmp_char`) and returns 1 or -1 at the first difference, 0 when both end together; a
/// failed mapping sets `errno` to `0x16` (no `_invalid_parameter`) and returns `0x7fffffff`.
pub fn mbsicmp_l(e: &mut Engine, first: Ptr, second: Ptr, locale: u32) -> i32 {
    e.with_stack(LOCALE_UPDATE_SIZE + 4, |e, frame| {
        let update = frame.addr();
        let buffer = update + LOCALE_UPDATE_SIZE;
        e.call(LOCALE_UPDATE, &args![update, locale]);
        if first.is_null() || second.is_null() {
            invalid_parameter(e, 0x16);
            locale_update_end(e, update);
            return 0x7fff_ffff;
        }
        let mbcinfo = e.mem.u32(update + LOCALE_UPDATE_MBCINFO);
        if e.mem.u32(mbcinfo + MBCINFO_ISMBCODEPAGE) == 0 {
            let result = e.call(STRICMP_L, &args![first, second, update]).i32();
            locale_update_end(e, update);
            return result;
        }
        let (mut a, mut b) = (first.addr(), second.addr());
        while let Some(first_char) = mbsicmp_char(e, update, buffer, mbcinfo, &mut a) {
            let Some(second_char) = mbsicmp_char(e, update, buffer, mbcinfo, &mut b) else {
                break;
            };
            if second_char != first_char {
                let result = order(first_char, second_char);
                locale_update_end(e, update);
                return result;
            }
            if first_char == 0 {
                locale_update_end(e, update);
                return 0;
            }
        }
        let errno = e.call(ERRNO, &args![]).u32();
        e.mem.set_u32(errno, 0x16);
        locale_update_end(e, update);
        0x7fff_ffff
    })
}

// Translated from 00ec858a (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mbsicmp` (Ghidra library match; cdecl, two words): `__mbsicmp_l(first, second, 0)`.
pub fn mbsicmp(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    mbsicmp_l(e, first, second, 0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec811d, mbscmp_l(Ptr, Ptr, u32) -> i32),
        entry!(0x00ec8244, mbscmp(Ptr, Ptr) -> i32),
        entry!(0x00ec825b, makepath_s(Ptr, u32, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00ec8364, mbsicmp_l(Ptr, Ptr, u32) -> i32),
        entry!(0x00ec858a, mbsicmp(Ptr, Ptr) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The thread data, `errno` word and `mbcinfo` a test's `_LocaleUpdate` double hands out.
    struct Locale {
        thread_data: u32,
        errno: u32,
        mbcinfo: u32,
    }

    /// Doubles for the CRT callees: `_LocaleUpdate` fills the local with `mbcinfo` and the
    /// thread data and marks it updated; `_errno` answers a word of its own.
    fn locale(e: &mut Engine, multibyte: bool) -> Locale {
        let thread_data = e.mem.alloc(0x80);
        e.mem.set_u32(thread_data + 0x70, 0xff);
        let errno = e.mem.alloc(4);
        let mbcinfo = e.mem.alloc(0x220);
        e.mem
            .set_u32(mbcinfo + MBCINFO_ISMBCODEPAGE, u32::from(multibyte));
        e.mem.set_u32(mbcinfo + MBCINFO_MBCODEPAGE, 932);
        e.mem.set_u32(mbcinfo + MBCINFO_MBLCID, 0x411);
        // 0x81 is a lead byte, a..z are lower case and map to A..Z.
        e.mem
            .set_u8(mbcinfo + MBCINFO_MBCTYPE + 0x81, MBCTYPE_LEAD_BYTE);
        for c in b'a'..=b'z' {
            e.mem
                .set_u8(mbcinfo + MBCINFO_MBCTYPE + u32::from(c), MBCTYPE_LOWER);
            e.mem
                .set_u8(mbcinfo + MBCINFO_MBCASEMAP + u32::from(c), c - 0x20);
        }
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0] + LOCALE_UPDATE_MBCINFO, mbcinfo);
            e.mem.set_u32(a[0] + LOCALE_UPDATE_THREAD_DATA, thread_data);
            e.mem.set_u8(a[0] + LOCALE_UPDATE_UPDATED, 1);
            Ret::default()
        });
        e.register_double(ERRNO, move |_, _| word(errno));
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        Locale {
            thread_data,
            errno,
            mbcinfo,
        }
    }

    fn string(e: &mut Engine, bytes: &[u8]) -> u32 {
        let p = e.mem.alloc(bytes.len() as u32 + 1);
        for (i, b) in bytes.iter().enumerate() {
            e.mem.set_u8(p + i as u32, *b);
        }
        p
    }

    fn read_string(e: &Engine, mut p: u32) -> Vec<u8> {
        let mut out = vec![];
        while e.mem.u8(p) != 0 {
            out.push(e.mem.u8(p));
            p += 1;
        }
        out
    }

    #[test]
    fn mbscmp_l_rejects_a_null_string() {
        let mut e = Engine::new();
        let l = locale(&mut e, true);
        e.call_log = Some(vec![]);
        let s = string(&mut e, b"a");
        assert_eq!(e.call(0x00ec811d, &args![0u32, s, 7u32]).i32(), 0x7fff_ffff);
        assert_eq!(e.mem.u32(l.errno), 0x16);
        assert_eq!(e.mem.u32(l.thread_data + 0x70), 0xfd);
        assert_eq!(calls_to(&e, INVALID_PARAMETER), vec![vec![0; 5]]);
        assert_eq!(calls_to(&e, LOCALE_UPDATE)[0][1], 7);
        e.mem.set_u32(l.errno, 0);
        assert_eq!(e.call(0x00ec811d, &args![s, 0u32, 7u32]).i32(), 0x7fff_ffff);
        assert_eq!(e.mem.u32(l.errno), 0x16);
    }

    #[test]
    fn mbscmp_l_uses_strcmp_without_a_multibyte_code_page() {
        let mut e = Engine::new();
        let l = locale(&mut e, false);
        e.register(STRCMP, |_, _| word(-5i32 as u32));
        e.call_log = Some(vec![]);
        let (a, b) = (string(&mut e, b"x"), string(&mut e, b"y"));
        assert_eq!(e.call(0x00ec811d, &args![a, b, 0u32]).i32(), -5);
        assert_eq!(calls_to(&e, STRCMP), vec![vec![a, b]]);
        assert_eq!(e.mem.u32(l.thread_data + 0x70), 0xfd);
    }

    #[test]
    fn mbscmp_l_compares_double_byte_characters() {
        let mut e = Engine::new();
        locale(&mut e, true);
        let a = string(&mut e, b"a\x81\x40z");
        let same = string(&mut e, b"a\x81\x40z");
        let lower_trail = string(&mut e, b"a\x81\x3fz");
        let lone_lead = string(&mut e, b"a\x81");
        assert_eq!(e.call(0x00ec811d, &args![a, same, 0u32]).i32(), 0);
        // The second string's character is below: 1; above: -1.
        assert_eq!(e.call(0x00ec811d, &args![a, lower_trail, 0u32]).i32(), 1);
        assert_eq!(e.call(0x00ec811d, &args![lower_trail, a, 0u32]).i32(), -1);
        // A lead byte before the end reads as 0, the end of the string.
        assert_eq!(
            e.call(0x00ec811d, &args![lone_lead, lone_lead, 0u32]).i32(),
            0
        );
        assert_eq!(e.call(0x00ec811d, &args![a, lone_lead, 0u32]).i32(), 1);
    }

    #[test]
    fn mbscmp_passes_no_locale() {
        let mut e = Engine::new();
        locale(&mut e, true);
        e.call_log = Some(vec![]);
        let (a, b) = (string(&mut e, b"b"), string(&mut e, b"a"));
        assert_eq!(e.call(0x00ec8244, &args![a, b]).i32(), 1);
        assert_eq!(calls_to(&e, LOCALE_UPDATE)[0][1], 0);
    }

    #[test]
    fn makepath_s_builds_every_part() {
        let mut e = Engine::new();
        let l = locale(&mut e, false);
        e.register(MBSDEC, |_, a| word(a[1] - 1));
        let out = e.mem.alloc(0x40);
        let drive = string(&mut e, b"c");
        let dir = string(&mut e, b"data");
        let slash_dir = string(&mut e, b"data/");
        let name = string(&mut e, b"file");
        let ext = string(&mut e, b"nif");
        let dot_ext = string(&mut e, b".dds");
        let empty = string(&mut e, b"");
        assert_eq!(
            e.call(0x00ec825b, &args![out, 0x40u32, drive, dir, name, ext])
                .u32(),
            0
        );
        assert_eq!(read_string(&e, out), b"c:data\\file.nif");
        assert_eq!(
            e.call(
                0x00ec825b,
                &args![out, 0x40u32, 0u32, slash_dir, name, dot_ext]
            )
            .u32(),
            0
        );
        assert_eq!(read_string(&e, out), b"data/file.dds");
        assert_eq!(
            e.call(0x00ec825b, &args![out, 0x40u32, empty, empty, 0u32, empty])
                .u32(),
            0
        );
        assert_eq!(read_string(&e, out), b"");
        // Exactly fitting: "file" and its 0 in 5 bytes.
        assert_eq!(
            e.call(0x00ec825b, &args![out, 5u32, 0u32, 0u32, name, 0u32])
                .u32(),
            0
        );
        assert_eq!(read_string(&e, out), b"file");
        assert_eq!(e.mem.u32(l.errno), 0);
    }

    #[test]
    fn makepath_s_reports_bad_arguments_and_overflow() {
        let mut e = Engine::new();
        let l = locale(&mut e, false);
        e.register(MBSDEC, |_, a| word(a[1] - 1));
        e.call_log = Some(vec![]);
        let out = e.mem.alloc(0x40);
        let drive = string(&mut e, b"c");
        let name = string(&mut e, b"file");
        assert_eq!(
            e.call(0x00ec825b, &args![0u32, 4u32, 0u32, 0u32, name, 0u32])
                .u32(),
            0x16
        );
        assert_eq!(
            e.call(0x00ec825b, &args![out, 0u32, 0u32, 0u32, name, 0u32])
                .u32(),
            0x16
        );
        assert_eq!(e.mem.u32(l.errno), 0x16);
        e.mem.set_u8(out, b'q');
        // The 0 does not fit.
        assert_eq!(
            e.call(0x00ec825b, &args![out, 4u32, 0u32, 0u32, name, 0u32])
                .u32(),
            0x22
        );
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(e.mem.u32(l.errno), 0x22);
        // A drive needs more than two bytes.
        assert_eq!(
            e.call(0x00ec825b, &args![out, 2u32, drive, 0u32, 0u32, 0u32])
                .u32(),
            0x22
        );
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 4);
    }

    #[test]
    fn mbsicmp_l_rejects_null_strings_and_uses_stricmp_l_without_multibyte() {
        let mut e = Engine::new();
        let l = locale(&mut e, false);
        e.register(STRICMP_L, |_, _| word(3u32));
        e.call_log = Some(vec![]);
        let s = string(&mut e, b"a");
        assert_eq!(e.call(0x00ec8364, &args![s, 0u32, 9u32]).i32(), 0x7fff_ffff);
        assert_eq!(e.mem.u32(l.errno), 0x16);
        assert_eq!(e.mem.u32(l.thread_data + 0x70), 0xfd);
        assert_eq!(e.call(0x00ec8364, &args![s, s, 9u32]).i32(), 3);
        let update = calls_to(&e, LOCALE_UPDATE)[1][0];
        assert_eq!(calls_to(&e, STRICMP_L), vec![vec![s, s, update]]);
    }

    #[test]
    fn mbsicmp_l_upper_cases_both_strings() {
        let mut e = Engine::new();
        let l = locale(&mut e, true);
        // Two bytes back for 0x81 0x40 (as 0x82 0x41), one byte for 0x81 0x50 (as 0x60),
        // nothing for any other pair.
        e.register(CRT_LC_MAP_STRING_A, |e, a| {
            let trail = e.mem.u8(a[3] + 1);
            match trail {
                0x40 => {
                    e.mem.set_u8(a[5], 0x82);
                    e.mem.set_u8(a[5] + 1, 0x41);
                    word(2u32)
                }
                0x50 => {
                    e.mem.set_u8(a[5], 0x60);
                    word(1u32)
                }
                _ => word(0u32),
            }
        });
        e.call_log = Some(vec![]);
        let a = string(&mut e, b"ab\x81\x40\x81\x50");
        let b = string(&mut e, b"AB\x81\x40\x81\x50");
        assert_eq!(e.call(0x00ec8364, &args![a, b, 0u32]).i32(), 0);
        let map_calls = calls_to(&e, CRT_LC_MAP_STRING_A);
        let update = calls_to(&e, LOCALE_UPDATE)[0][0];
        assert_eq!(
            map_calls[0],
            vec![update, 0x411, 0x200, a + 2, 2, update + 0x10, 2, 932, 1]
        );
        assert_eq!(map_calls.len(), 4);
        let c = string(&mut e, b"b");
        assert_eq!(e.call(0x00ec8364, &args![c, a, 0u32]).i32(), 1);
        assert_eq!(e.call(0x00ec8364, &args![a, c, 0u32]).i32(), -1);
        assert_eq!(e.mem.u32(l.thread_data + 0x70), 0xfd);
        assert_eq!(e.mem.u32(l.errno), 0);
        // A failed mapping: errno 0x16 and no invalid-parameter call.
        let bad = string(&mut e, b"\x81\x41");
        assert_eq!(e.call(0x00ec8364, &args![bad, a, 0u32]).i32(), 0x7fff_ffff);
        assert_eq!(e.mem.u32(l.errno), 0x16);
        assert!(calls_to(&e, INVALID_PARAMETER).is_empty());
        let _ = l.mbcinfo;
    }

    #[test]
    fn mbsicmp_passes_no_locale() {
        let mut e = Engine::new();
        locale(&mut e, true);
        e.call_log = Some(vec![]);
        let (a, b) = (string(&mut e, b"x"), string(&mut e, b"X"));
        assert_eq!(e.call(0x00ec858a, &args![a, b]).i32(), 0);
        assert_eq!(calls_to(&e, LOCALE_UPDATE)[0][1], 0);
    }
}

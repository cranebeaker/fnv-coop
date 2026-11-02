//! More C runtime pieces the play build's CRT code calls (MSVC 2008 LIBCMT, statically linked
//! into FalloutNV.exe 1.4.0.525), translated from the exe: `_ftelli64_nolock` (`00eded40`),
//! `_fseeki64_nolock` (`00edf0d6`), `__crtGetLocaleInfoA_stat` (`00ee1e2b`), `__iswctype_l`
//! (`00ee2332`), `___crtGetStringTypeW` (`00ee5eff`) and `___crtLCMapStringW` (`00ee4774`).
//! Every call is made by address, as the exe makes it, so a test can double each one.

use super::*;

/// `_fileno` (`00ed806c`), `_lseeki64` (`00ee202c`: handle, 64-bit offset as two words,
/// origin; the result in `EDX:EAX`), `_flush` (`00ecd8e6`), `_errno` (`00ec85e3`),
/// `_ftelli64_nolock` (`00eded40`), `malloc` (`00ecd1c7`), `_freea` (`00ec8e14`),
/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`) and `___crtGetStringTypeW` (`00ee5eff`).
const FILENO: u32 = 0x00ed_806c;
const LSEEKI64: u32 = 0x00ee_202c;
const FLUSH: u32 = 0x00ec_d8e6;
const ERRNO: u32 = 0x00ec_85e3;
const FTELLI64_NOLOCK: u32 = 0x00ed_ed40;
const MALLOC: u32 = 0x00ec_d1c7;
const FREEA: u32 = 0x00ec_8e14;
const LOCALE_UPDATE: u32 = 0x00ec_660e;
const CRT_GET_STRING_TYPE_W: u32 = 0x00ee_5eff;
/// Import slots (KERNEL32, stdcall): `ReadFile`, `GetLocaleInfoW`, `GetLocaleInfoA`,
/// `GetLastError`, `WideCharToMultiByte`, `GetStringTypeW`, `LCMapStringW`.
const API_READ_FILE: u32 = 0x00fd_f080;
const API_GET_LOCALE_INFO_W: u32 = 0x00fd_f22c;
const API_GET_LOCALE_INFO_A: u32 = 0x00fd_f268;
const API_GET_LAST_ERROR: u32 = 0x00fd_f094;
const API_WIDE_CHAR_TO_MULTI_BYTE: u32 = 0x00fd_f170;
const API_GET_STRING_TYPE_W: u32 = 0x00fd_f230;
const API_LC_MAP_STRING_W: u32 = 0x00fd_f1d4;
/// `__pioinfo` (`01270960`: blocks of 32 `ioinfo` records of 0x40 bytes), the UTF-8 trail
/// byte counts (`_lookuptrailbytes`, `011c2790`), the wide class table (`_pwctype`,
/// `011c1cb8`) and `__crtGetLocaleInfoA_stat`'s choice of API (`01270818`: 0 unknown, 1 the
/// wide one, 2 the narrow one).
const PIOINFO: u32 = 0x0127_0960;
const TRAIL_BYTES: u32 = 0x011c_2790;
const PWCTYPE: u32 = 0x011c_1cb8;
const LOCALE_INFO_API: u32 = 0x0127_0818;

/// The `ioinfo` record of handle `fh`, and the address of its block pointer.
fn ioinfo(e: &Engine, fh: u32) -> u32 {
    let block = e
        .mem
        .u32(PIOINFO + ((fh as i32 >> 5) as u32).wrapping_mul(4));
    block.wrapping_add((fh & 0x1f) << 6)
}

fn lseeki64(e: &mut Engine, fh: u32, offset: u64, origin: u32) -> u64 {
    e.call(
        LSEEKI64,
        &args![fh, offset as u32, (offset >> 32) as u32, origin],
    )
    .u64()
}

/// The exe's failure test on a `_lseeki64` result: a negative high word.
fn failed(position: u64) -> bool {
    ((position >> 32) as i32) < 0
}

// Translated from 00eded40 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00eded40 - the /GS stack cookie check (00ec408c) is not translated
/// `_ftelli64_nolock` (cdecl: the stream): the stream's 64-bit position. The handle's position
/// (`_lseeki64(.., 0, SEEK_CUR)`, -1 when negative) less the unread count for an unbuffered
/// stream. For a buffered one in a read or write mode: a UTF-8 text handle (`ioinfo` +0x24
/// text mode 1 with translations at +0x30) re-reads up to 0x1000 bytes from its start
/// position (+0x28) to count the characters before the buffer pointer (CR LF as one, trail
/// bytes skipped); otherwise the buffer offset (each LF of a text file counting two) is added
/// to the position, which for a read stream first goes back by the buffered bytes (measured
/// to the end of file for text files at the end, else the buffer size, plus one for a CR LF
/// pending). UTF-16 modes halve the byte counts. A stream in no mode is `EINVAL`, -1.
pub fn ftelli64_nolock(e: &mut Engine, stream: u32) -> u64 {
    const FAIL: u64 = u64::MAX;
    let fh = e.call(FILENO, &args![stream]).u32();
    if (e.mem.u32(stream + 4) as i32) < 0 {
        e.mem.set_u32(stream + 4, 0);
    }
    let mut filepos = lseeki64(e, fh, 0, 1);
    if failed(filepos) {
        return FAIL;
    }
    let info = ioinfo(e, fh);
    let tmode = ((e.mem.u8(info + 0x24) << 1) as i8) >> 1;
    let flag = e.mem.u32(stream + 0xc);
    if flag & 0x108 == 0 {
        let count = e.mem.u32(stream + 4) as i32 as i64 as u64;
        return filepos.wrapping_sub(count);
    }
    let ptr = e.mem.u32(stream);
    let base = e.mem.u32(stream + 8);
    let mut offset = ptr.wrapping_sub(base);
    if flag & 3 != 0 {
        if tmode == 1 && e.mem.u32(info + 0x30) != 0 {
            let mut skip = offset >> 1;
            if e.mem.u32(stream + 4) == 0 {
                return filepos;
            }
            let start = u64::from(e.mem.u32(info + 0x28)) | u64::from(e.mem.u32(info + 0x2c)) << 32;
            let at = lseeki64(e, fh, start, 0);
            let info = ioinfo(e, fh);
            let start = u64::from(e.mem.u32(info + 0x28)) | u64::from(e.mem.u32(info + 0x2c)) << 32;
            if at != start {
                return FAIL;
            }
            let handle = e.mem.u32(info);
            return e.with_stack(0x1004, |e, frame| {
                let read = frame.addr();
                let buffer = read + 4;
                let ok = e
                    .call(API_READ_FILE, &args![handle, buffer, 0x1000u32, read, 0u32])
                    .u32();
                if ok == 0 {
                    return FAIL;
                }
                if failed(lseeki64(e, fh, filepos, 0)) {
                    return FAIL;
                }
                let read = e.mem.u32(read);
                if skip > read {
                    return FAIL;
                }
                let end = buffer + read;
                let mut p = buffer;
                while skip != 0 {
                    skip -= 1;
                    if p >= end {
                        break;
                    }
                    let c = e.mem.u8(p);
                    if c == 0x0d {
                        if p < end - 1 && e.mem.u8(p + 1) == 0x0a {
                            p += 1;
                        }
                    } else {
                        let trail = e.mem.u8(TRAIL_BYTES + u32::from(c)) as i8 as i32;
                        p = p.wrapping_add(trail as u32);
                    }
                    p = p.wrapping_add(1);
                }
                u64::from(p.wrapping_sub(buffer)).wrapping_add(at)
            });
        }
        if e.mem.u8(info + 4) & 0x80 != 0 {
            for p in base..ptr {
                if e.mem.u8(p) == 0x0a {
                    offset = offset.wrapping_add(1);
                }
            }
        }
    } else if flag & 0x80 == 0 {
        let errno = e.call(ERRNO, &[]).u32();
        e.mem.set_u32(errno, 0x16);
        return FAIL;
    }
    if filepos == 0 {
        return u64::from(offset);
    }
    if flag & 1 != 0 {
        let count = e.mem.u32(stream + 4);
        if count == 0 {
            offset = 0;
        } else {
            let mut rdcnt = ptr.wrapping_sub(base).wrapping_add(count);
            if e.mem.u8(info + 4) & 0x80 != 0 {
                let eof = lseeki64(e, fh, 0, 2);
                if eof == filepos {
                    let base = e.mem.u32(stream + 8);
                    let end = base.wrapping_add(rdcnt);
                    for p in base..end {
                        if e.mem.u8(p) == 0x0a {
                            rdcnt = rdcnt.wrapping_add(1);
                        }
                    }
                    if e.mem.u32(stream + 0xc) & 0x2000 != 0 {
                        rdcnt = rdcnt.wrapping_add(1);
                    }
                } else {
                    if failed(lseeki64(e, fh, filepos, 0)) {
                        return FAIL;
                    }
                    let flag = e.mem.u32(stream + 0xc);
                    rdcnt = if rdcnt <= 0x200 && flag & 8 != 0 && flag & 0x400 == 0 {
                        0x200
                    } else {
                        e.mem.u32(stream + 0x18)
                    };
                    if e.mem.u8(ioinfo(e, fh) + 4) & 4 != 0 {
                        rdcnt = rdcnt.wrapping_add(1);
                    }
                }
            }
            if tmode == 1 {
                rdcnt >>= 1;
            }
            filepos = filepos.wrapping_sub(u64::from(rdcnt));
        }
    }
    if tmode == 1 {
        offset >>= 1;
    }
    u64::from(offset).wrapping_add(filepos)
}

// Translated from 00edf0d6 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_fseeki64_nolock` (cdecl: stream, 64-bit offset as two words, origin): a stream in no
/// mode (`_flag & 0x83` clear) or an origin other than 0 to 2 is `EINVAL`, -1. Otherwise the
/// EOF flag is cleared, `SEEK_CUR` becomes `SEEK_SET` from the current position, the stream
/// is flushed (a read-write one drops its mode; a buffered read one without `setvbuf` gets the
/// 0x200 buffer size back) and the handle seeks. Answers 0, or -1 when the seek fails.
pub fn fseeki64_nolock(e: &mut Engine, stream: u32, low: u32, high: u32, origin: u32) -> i32 {
    let flag = e.mem.u32(stream + 0xc);
    if flag & 0x83 == 0 || origin > 2 {
        let errno = e.call(ERRNO, &[]).u32();
        e.mem.set_u32(errno, 0x16);
        return -1;
    }
    e.mem.set_u32(stream + 0xc, flag & !0x10);
    let mut offset = u64::from(low) | u64::from(high) << 32;
    let mut origin = origin;
    if origin == 1 {
        let position = e.call(FTELLI64_NOLOCK, &args![stream]).u64();
        offset = offset.wrapping_add(position);
        origin = 0;
    }
    e.call(FLUSH, &args![stream]);
    let flag = e.mem.u32(stream + 0xc);
    if flag & 0x80 != 0 {
        e.mem.set_u32(stream + 0xc, flag & !3);
    } else if flag & 1 != 0 && flag & 8 != 0 && flag & 0x400 == 0 {
        e.mem.set_u32(stream + 0x18, 0x200);
    }
    let fh = e.call(FILENO, &args![stream]).u32();
    let r = lseeki64(e, fh, offset, origin);
    if (r as u32) & ((r >> 32) as u32) == 0xffff_ffff {
        -1
    } else {
        0
    }
}

// Translated from 00ee1e2b (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee1e2b - the /GS stack cookie check (00ec408c) is not translated
/// `__crtGetLocaleInfoA_stat` (cdecl: `_LocaleUpdate` object, locale id, field, buffer, size,
/// code page). The first call finds out whether `GetLocaleInfoW` works (`GetLocaleInfoW(0, 1,
/// 0, 0)`; `ERROR_CALL_NOT_IMPLEMENTED` 0x78 picks the narrow API). With the narrow API (or
/// still unknown) `GetLocaleInfoA` answers. With the wide one the field is read into a wide
/// buffer of the size it asks for (on the stack up to 0x400 bytes with its 0xCCCC mark, else
/// `malloc` with 0xDDDD; `_freea` after) and converted with `WideCharToMultiByte` in the
/// code page (the locale's when 0); answers its result, or 0.
pub fn crt_get_locale_info_a_stat(
    e: &mut Engine,
    update: u32,
    lcid: u32,
    field: u32,
    buffer: u32,
    size: u32,
    code_page: u32,
) -> u32 {
    let mut api = e.mem.u32(LOCALE_INFO_API);
    if api == 0 {
        if e.call(API_GET_LOCALE_INFO_W, &args![0u32, 1u32, 0u32, 0u32])
            .u32()
            != 0
        {
            e.mem.set_u32(LOCALE_INFO_API, 1);
            api = 1;
        } else if e.call(API_GET_LAST_ERROR, &[]).u32() == 0x78 {
            e.mem.set_u32(LOCALE_INFO_API, 2);
            api = 2;
        } else {
            api = e.mem.u32(LOCALE_INFO_API);
        }
    }
    if api == 2 || api == 0 {
        return e
            .call(API_GET_LOCALE_INFO_A, &args![lcid, field, buffer, size])
            .u32();
    }
    if api != 1 {
        return 0;
    }
    let code_page = if code_page == 0 {
        let info = e.mem.u32(update);
        e.mem.u32(info + 4)
    } else {
        code_page
    };
    let count = e
        .call(API_GET_LOCALE_INFO_W, &args![lcid, field, 0u32, 0u32])
        .u32();
    if count == 0 {
        return 0;
    }
    if (count as i32) <= 0 || 0xffff_ffe0 / count < 2 {
        return 0;
    }
    let bytes = count.wrapping_mul(2).wrapping_add(8);
    let convert = |e: &mut Engine, wide: u32| -> u32 {
        let mut result = 0;
        if e.call(API_GET_LOCALE_INFO_W, &args![lcid, field, wide, count])
            .u32()
            != 0
        {
            let (out, out_size) = if size == 0 { (0, 0) } else { (buffer, size) };
            result = e
                .call(
                    API_WIDE_CHAR_TO_MULTI_BYTE,
                    &args![
                        code_page,
                        0u32,
                        wide,
                        0xffff_ffffu32,
                        out,
                        out_size,
                        0u32,
                        0u32
                    ],
                )
                .u32();
        }
        e.call(FREEA, &args![wide]);
        result
    };
    if bytes <= 0x400 {
        e.with_stack(bytes, |e, block| {
            let block = block.addr();
            e.mem.set_u32(block, 0xcccc);
            convert(e, block + 8)
        })
    } else {
        let block = e.call(MALLOC, &args![bytes]).u32();
        if block == 0 {
            return 0;
        }
        e.mem.set_u32(block, 0xdddd);
        convert(e, block + 8)
    }
}

// Translated from 00ee2332 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__iswctype_l` (cdecl: wide character, mask, locale): `WEOF` (0xFFFF) is 0; below 0x100
/// the class from `_pwctype`; above, `___crtGetStringTypeW` (`CT_CTYPE1`, one character, the
/// locale's code page and LC_CTYPE handle; 0 when it fails). Answers the class and the mask.
pub fn iswctype_l(e: &mut Engine, c: u32, mask: u32, locale: u32) -> u32 {
    let c16 = c as u16;
    if c16 == 0xffff {
        return 0;
    }
    if c16 < 0x100 {
        let table = e.mem.u32(PWCTYPE);
        let class = e.mem.u16(table.wrapping_add(u32::from(c16) * 2)) & mask as u16;
        return u32::from(class);
    }
    e.with_stack(0x18, |e, frame| {
        let update = frame.addr();
        let slot = update + 0x10;
        let class = update + 0x14;
        e.mem.set_u32(slot, c);
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let info = e.mem.u32(update);
        let code_page = e.mem.u32(info + 4);
        let handle = e.mem.u32(info + 0x14);
        let ok = e
            .call(
                CRT_GET_STRING_TYPE_W,
                &args![update, 1u32, slot, 1u32, class, code_page, handle],
            )
            .u32();
        if ok == 0 {
            e.mem.set_u32(class, 0);
        }
        locale_update_release(e, update);
        u32::from(e.mem.u16(class)) & u32::from(mask as u16)
    })
}

// Translated from 00ee5eff (disassembled, FalloutNV.exe 1.4.0.525)
/// `___crtGetStringTypeW` (cdecl: locale, info type, text, count, types out, code page,
/// locale id): `GetStringTypeW(info type, text, count, types)` inside a `_LocaleUpdate`; a
/// count below -1 answers 0.
#[allow(clippy::too_many_arguments)]
pub fn crt_get_string_type_w(
    e: &mut Engine,
    locale: u32,
    kind: u32,
    text: u32,
    count: u32,
    types: u32,
    _code_page: u32,
    _lcid: u32,
) -> u32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let result = if (count as i32) < -1 {
            0
        } else {
            e.call(API_GET_STRING_TYPE_W, &args![kind, text, count, types])
                .u32()
        };
        locale_update_release(e, update);
        result
    })
}

// Translated from 00ee4774 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___crtLCMapStringW` (cdecl: locale, locale id, flags, text, length, out, out size, code
/// page): a positive length is cut at the text's terminator (`wcsncnt`); then
/// `LCMapStringW(locale id, flags, text, length, out, out size)` inside a `_LocaleUpdate`.
#[allow(clippy::too_many_arguments)]
pub fn crt_lcmap_string_w(
    e: &mut Engine,
    locale: u32,
    lcid: u32,
    flags: u32,
    text: u32,
    length: u32,
    out: u32,
    out_size: u32,
    _code_page: u32,
) -> u32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let mut length = length;
        if (length as i32) > 0 {
            let mut left = length;
            let mut p = text;
            let mut ended = false;
            loop {
                left = left.wrapping_sub(1);
                if e.mem.u16(p) == 0 {
                    ended = true;
                    break;
                }
                p = p.wrapping_add(2);
                if left == 0 {
                    break;
                }
            }
            if !ended {
                left = 0xffff_ffff;
            }
            length = length.wrapping_sub(left).wrapping_sub(1);
        }
        let result = e
            .call(
                API_LC_MAP_STRING_W,
                &args![lcid, flags, text, length, out, out_size],
            )
            .u32();
        locale_update_release(e, update);
        result
    })
}

/// This file's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ed_ed40, ftelli64_nolock(u32) -> u64),
        entry!(0x00ed_f0d6, fseeki64_nolock(u32, u32, u32, u32) -> i32),
        entry!(0x00ee_1e2b, crt_get_locale_info_a_stat(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ee_2332, iswctype_l(u32, u32, u32) -> u32),
        entry!(0x00ee_5eff, crt_get_string_type_w(u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ee_4774, crt_lcmap_string_w(u32, u32, u32, u32, u32, u32, u32, u32) -> u32),
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

    /// An engine with a handle table (handle 3 at `info`), `_fileno` answering 3, and
    /// `_lseeki64` answering `position` for SEEK_CUR, `end` for SEEK_END and the offset for
    /// SEEK_SET.
    fn engine(position: u64, end: u64) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0127_0000, 0x1000);
        e.map(0x011c_1000, 0x2000);
        let block = e.mem.alloc(0x800);
        e.mem.set_u32(PIOINFO, block);
        let info = block + 3 * 0x40;
        let stream = e.mem.alloc(0x20);
        e.register(FILENO, |_, _| 3u32.into_ret());
        e.register_double(LSEEKI64, move |_, a| {
            let v = match a[3] {
                1 => position,
                2 => end,
                _ => u64::from(a[1]) | u64::from(a[2]) << 32,
            };
            Ret {
                eax: v as u32,
                edx: (v >> 32) as u32,
                st0: 0.0,
            }
        });
        (e, info, stream)
    }

    fn set_stream(e: &mut Engine, stream: u32, base: u32, ptr: u32, count: u32, flag: u32) {
        e.mem.set_u32(stream, ptr);
        e.mem.set_u32(stream + 4, count);
        e.mem.set_u32(stream + 8, base);
        e.mem.set_u32(stream + 0xc, flag);
        e.mem.set_u32(stream + 0x18, 0x1000);
    }

    #[test]
    fn tell_on_unbuffered_and_binary_streams() {
        let (mut e, _, stream) = engine(1000, 5000);
        let buffer = e.mem.alloc(0x200);
        // Unbuffered: position less the count.
        set_stream(&mut e, stream, buffer, buffer, 7, 0x1);
        assert_eq!(e.call(FTELLI64_NOLOCK, &[stream]).u64(), 993);
        // Buffered read, binary: position less the buffered bytes plus the offset.
        set_stream(&mut e, stream, buffer, buffer + 10, 30, 0x9);
        assert_eq!(e.call(FTELLI64_NOLOCK, &[stream]).u64(), 1000 - 40 + 10);
        // Buffered write: the position plus what is buffered.
        set_stream(&mut e, stream, buffer, buffer + 12, 0, 0xa);
        assert_eq!(e.call(FTELLI64_NOLOCK, &[stream]).u64(), 1012);
        // No mode: EINVAL.
        let errno = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| errno.into_ret());
        set_stream(&mut e, stream, buffer, buffer, 0, 0x8);
        assert_eq!(e.call(FTELLI64_NOLOCK, &[stream]).u64(), u64::MAX);
        assert_eq!(e.mem.u32(errno), 0x16);
    }

    #[test]
    fn tell_on_text_streams_counts_line_feeds() {
        let (mut e, info, stream) = engine(1000, 1000);
        e.mem.set_u8(info + 4, 0x80);
        let buffer = e.mem.alloc(0x200);
        e.mem.write(buffer, b"a\nb\nc\nd");
        // Read stream at the end of the file: 7 bytes buffered, 3 LFs, 2 consumed with 1 LF.
        set_stream(&mut e, stream, buffer, buffer + 2, 5, 0x9);
        assert_eq!(e.call(FTELLI64_NOLOCK, &[stream]).u64(), 1000 - (7 + 3) + 3);
        // Not at the end: the buffer size (0x200 for its own small buffer).
        let (mut e, info, stream) = engine(1000, 9000);
        e.mem.set_u8(info + 4, 0x80);
        let buffer = e.mem.alloc(0x200);
        e.mem.write(buffer, b"a\nb\nc\nd");
        set_stream(&mut e, stream, buffer, buffer + 2, 5, 0x9);
        assert_eq!(e.call(FTELLI64_NOLOCK, &[stream]).u64(), 1000 - 0x200 + 3);
    }

    #[test]
    fn seek_converts_current_to_set_and_flushes() {
        let (mut e, _, stream) = engine(100, 0);
        e.register(FTELLI64_NOLOCK, |_, _| Ret {
            eax: 40,
            edx: 0,
            st0: 0.0,
        });
        e.register(FLUSH, |_, _| 0u32.into_ret());
        let errno = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| errno.into_ret());
        e.mem.set_u32(stream + 0xc, 0x19);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_f0d6, &[stream, 5, 0, 1]).i32(), 0);
        assert_eq!(calls(&e, LSEEKI64), vec![vec![3, 45, 0, 0]]);
        assert_eq!(e.mem.u32(stream + 0xc), 0x9);
        assert_eq!(e.mem.u32(stream + 0x18), 0x200);
        assert_eq!(e.call(0x00ed_f0d6, &[stream, 0, 0, 3]).i32(), -1);
        assert_eq!(e.mem.u32(errno), 0x16);
        // A read-write stream drops its mode.
        e.mem.set_u32(stream + 0xc, 0x83);
        assert_eq!(e.call(0x00ed_f0d6, &[stream, 9, 0, 0]).i32(), 0);
        assert_eq!(e.mem.u32(stream + 0xc), 0x80);
        // A failed seek.
        e.register(LSEEKI64, |_, _| Ret {
            eax: 0xffff_ffff,
            edx: 0xffff_ffff,
            st0: 0.0,
        });
        assert_eq!(e.call(0x00ed_f0d6, &[stream, 9, 0, 0]).i32(), -1);
    }

    #[test]
    fn locale_info_a_picks_its_api() {
        let mut e = Engine::new();
        e.map(0x0127_0000, 0x1000);
        e.register(API_GET_LOCALE_INFO_W, |e, a| {
            if a[2] != 0 {
                e.mem.write(a[2], &[0x41, 0, 0x42, 0, 0, 0]);
            }
            3u32.into_ret()
        });
        e.register(API_WIDE_CHAR_TO_MULTI_BYTE, |e, a| {
            if a[4] != 0 {
                e.mem.write(a[4], b"AB\0");
            }
            3u32.into_ret()
        });
        e.register(FREEA, |_, _| Ret::default());
        e.register(API_GET_LOCALE_INFO_A, |_, _| 99u32.into_ret());
        let update = e.mem.alloc(0x10);
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(update, info);
        e.mem.set_u32(info + 4, 1252);
        let out = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_1e2b, &[update, 0x409, 3, out, 8, 0]).u32(), 3);
        assert_eq!(e.mem.cstr(out), b"AB");
        assert_eq!(e.mem.u32(LOCALE_INFO_API), 1);
        let wide = calls(&e, API_WIDE_CHAR_TO_MULTI_BYTE);
        assert_eq!(wide[0][0], 1252);
        assert_eq!(wide[0][3..], [0xffff_ffff, out, 8, 0, 0]);
        // The narrow API once chosen.
        e.mem.set_u32(LOCALE_INFO_API, 2);
        assert_eq!(
            e.call(0x00ee_1e2b, &[update, 0x409, 3, out, 8, 0]).u32(),
            99
        );
    }

    #[test]
    fn wide_class_and_map_wrappers() {
        let mut e = Engine::new();
        e.map(0x011c_1000, 0x1000);
        let table = e.mem.alloc(0x200);
        e.mem.set_u16(table + 0x41 * 2, 0x181);
        e.mem.set_u32(PWCTYPE, table);
        let info = e.mem.alloc(0x20);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u8(a[0] + 0xc, 0);
            a[0].into_ret()
        });
        assert_eq!(e.call(0x00ee_2332, &[0xffff, 0xffff, 0]).u32(), 0);
        assert_eq!(e.call(0x00ee_2332, &[0x41, 0x101, 0]).u32(), 0x101);
        e.register(API_GET_STRING_TYPE_W, |e, a| {
            e.mem.set_u16(a[3], 2);
            1u32.into_ret()
        });
        assert_eq!(e.call(0x00ee_2332, &[0x430, 2, 0]).u32(), 2);
        assert_eq!(e.call(0x00ee_2332, &[0x430, 1, 0]).u32(), 0);
        assert_eq!(
            e.call(CRT_GET_STRING_TYPE_W, &[0, 1, 0, (-2i32) as u32, 0, 0, 0])
                .u32(),
            0
        );
        e.register(API_LC_MAP_STRING_W, |_, a| a[3].into_ret());
        let text = e.mem.alloc(16);
        e.mem.write(text, &[0x61, 0, 0x62, 0, 0, 0, 0x63, 0]);
        // Cut at the terminator: 2; without one in range: the length; -1 passes.
        assert_eq!(
            e.call(0x00ee_4774, &[0, 0x409, 0x100, text, 4, 0, 0, 0])
                .u32(),
            2
        );
        assert_eq!(
            e.call(0x00ee_4774, &[0, 0x409, 0x100, text, 2, 0, 0, 0])
                .u32(),
            2
        );
        assert_eq!(
            e.call(0x00ee_4774, &[0, 0x409, 0x100, text, 1, 0, 0, 0])
                .u32(),
            1
        );
        assert_eq!(
            e.call(0x00ee_4774, &[0, 0x409, 0x100, text, 0xffff_ffff, 0, 0, 0])
                .u32(),
            0xffff_ffff
        );
    }
}

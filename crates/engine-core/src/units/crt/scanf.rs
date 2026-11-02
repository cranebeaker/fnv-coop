//! The C runtime's formatted input (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe
//! 1.4.0.525), translated from the exe: `sscanf` (`00eca4a6`) on a string stream
//! (`_vscan_fn`), the scan core `_input_l` with its character helpers (`_inc`, `_un_inc`,
//! `_whiteout`, `_hextodec`, `__check_float_string`, `_filbuf`, `_ungetc_nolock`), the
//! character classes it asks (`isdigit`, `isspace`, `isxdigit` and their `_l` forms,
//! `_isleadbyte`, `_mbtowc_l`), and the text-to-float conversion `%e`/`%f`/`%g` reach through
//! the `_cfltcvt_tab` slot `011c1cf0`: `_fassign_l`, `_atodbl_l`, `_atoflt_l`,
//! `__strgtold12_l`, `__mtold12`, `_ld12tod` and `_ld12tof`.
//!
//! The float conversion is integer arithmetic on the CRT's 12-byte `_LDBL12` (16 extra
//! mantissa bits below an 80-bit extended value) and needs no x87 instruction; it is
//! translated step by step (including where it differs from IEEE rounding: an exact half is
//! rounded down, and a denormal is rounded one bit too low and then cut), so the results are
//! the exe's bit for bit. The powers of ten it multiplies by are read from the exe's `.data`
//! (`_pow10pos` `011c2cb0`, `_pow10neg` `011c2e10`) and the target formats from its
//! `FpFormatDescriptor`s (`011c2c80` double, `011c2c98` float), as the code reads them.

use super::*;

/// `_inc`'s, `_input_l`'s and `_vscan_fn`'s callees and data.
const SCAN_INPUT_L: u32 = 0x00ed_a11e;
const SCAN_MEMCPY: u32 = 0x00ec_44d0;
const SCAN_CALLOC_CRT: u32 = 0x00ed_0d24;
const SCAN_RECALLOC_CRT: u32 = 0x00ed_0dbe;
const SCAN_RECALLOC: u32 = 0x00ee_1700;
const SCAN_READ: u32 = 0x00ed_cf0c;
const SCAN_ENCODE_POINTER: u32 = 0x00ed_0f17;
/// `Sleep` (import slot) and `__maxwait` (`0126fdfc`), which `__recalloc_crt` waits up to.
const IMPORT_SLEEP: u32 = 0x00fd_f104;
const MAX_WAIT: u32 = 0x0126_fdfc;
/// `_pctype`: the C locale's character-class table `isdigit` and its kin read while the locale
/// is unchanged.
const PCTYPE: u32 = 0x011c_23e8;
/// `_cfltcvt_tab` (ten slots from `011c1cd4`): `__cfltcvt_init` fills it and
/// `__initp_misc_cfltcvt_tab` encodes it in place. `_input_l` calls slot 7 (`011c1cf0`),
/// `_fassign_l`.
const CFLTCVT_TAB: u32 = 0x011c_1cd4;
const CFLTCVT_FASSIGN: u32 = 0x011c_1cf0;
/// The `FpFormatDescriptor`s `_ld12tod` and `_ld12tof` read (max exponent, min exponent,
/// precision, exponent width, format width, bias; six words each).
const DOUBLE_FORMAT: u32 = 0x011c_2c80;
const FLOAT_FORMAT: u32 = 0x011c_2c98;
/// `_pow10pos` and `_pow10neg` (twelve-byte `_LDBL12` entries: 10^1..10^7, 10^8..10^56 by 8,
/// 10^64..10^448 by 64, 10^512..10^3584 by 512, 10^4096; and their reciprocals);
/// `__strgtold12_l` starts each 0x60 bytes before the table.
const POW10_POS: u32 = 0x011c_2cb0;
const POW10_NEG: u32 = 0x011c_2e10;

// ---- sscanf and the string stream ----------------------------------------------------------

// Translated from 00eca4a6 (decompiled, FalloutNV.exe 1.4.0.525)
/// `sscanf` (LIBCMT/sscanf.c; cdecl, variable arguments): `_vscan_fn(_input_l, format, NULL,
/// args)` with the string in ESI; `args` is the address of the first variable argument (the
/// caller's stack). Returns what `_input_l` returns: the number of fields assigned, or -1 when
/// the input ended before the first conversion.
pub fn sscanf(e: &mut Engine, string: u32, format: u32, args: u32) -> i32 {
    vscan_fn(e, string, SCAN_INPUT_L, format, 0, args)
}

/// The registry entry of [`sscanf`]: string, format, then the variable arguments, which are
/// copied to game memory as the argument list the exe reads.
fn sscanf_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let rest = &words[2..];
    let size = 4 * (rest.len() as u32).max(1);
    e.with_stack(size, |e, list| {
        for (i, word) in rest.iter().enumerate() {
            e.mem.set_u32(list.addr() + 4 * i as u32, *word);
        }
        sscanf(e, words[0], words[1], list.addr())
    })
    .into_ret()
}

// Translated from 00eca43c (decompiled, FalloutNV.exe 1.4.0.525)
/// `_vscan_fn` (LIBCMT/sscanf.c; cdecl, the string in ESI: passed first): measures the
/// string (`strlen`, before the null test, as the exe does), and for a null string or format
/// sets `errno` to `EINVAL`, reports an invalid parameter and returns -1. Otherwise builds the
/// string stream on its stack (`_ptr` = `_base` = string, `_cnt` = the length capped at
/// `0x7fffffff`, `_flag` = `_IOREAD | _IOSTRG | _IOMYBUF` 0x49) and returns
/// `input(stream, format, locale, args)`.
pub fn vscan_fn(
    e: &mut Engine,
    string: u32,
    input: u32,
    format: u32,
    locale: u32,
    args: u32,
) -> i32 {
    let length = e.call(STRLEN_ADDR, &[string]).u32();
    if string == 0 || format == 0 {
        let slot = errno(e);
        e.mem.set_u32(slot.addr(), 0x16);
        fn_00ec7c30(e, 0, 0, 0, 0, 0);
        return -1;
    }
    // The FILE at EBP-0x20: _ptr, _cnt, _base, _flag (the other fields are not set).
    e.with_stack(0x20, |e, stream| {
        let stream = stream.addr();
        e.mem.set_u32(stream + 0xc, 0x49);
        e.mem.set_u32(stream + 8, string);
        e.mem.set_u32(stream, string);
        let count = if length > 0x7fff_ffff {
            0x7fff_ffff
        } else {
            length
        };
        e.mem.set_u32(stream + 4, count);
        e.call(input, &[stream, format, locale, args]).u32() as i32
    })
}

// ---- Getting and ungetting characters ------------------------------------------------------

// Translated from 00eda0cb (decompiled, FalloutNV.exe 1.4.0.525)
/// `_inc` (LIBCMT/input.c; the stream in EDX): `_getc_nolock`: the next byte of the stream's
/// buffer (`--_cnt >= 0`), else `_filbuf`.
pub fn inc(e: &mut Engine, stream: u32) -> u32 {
    let count = e.mem.u32(stream + 4).wrapping_sub(1);
    e.mem.set_u32(stream + 4, count);
    if (count as i32) < 0 {
        return filbuf(e, Ptr::new(stream)) as u32;
    }
    let at = e.mem.u32(stream);
    let c = u32::from(e.mem.u8(at));
    e.mem.set_u32(stream, at.wrapping_add(1));
    c
}

// Translated from 00eda0e1 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_un_inc` (LIBCMT/input.c; cdecl: character, stream): `_ungetc_nolock` unless the
/// character is `EOF`. The exe leaves EAX as `_ungetc_nolock` (a tail jump) or untouched; no
/// caller reads it.
pub fn un_inc(e: &mut Engine, c: u32, stream: u32) {
    if c != u32::MAX {
        ungetc_nolock(e, c, Ptr::new(stream));
    }
}

// Translated from 00eda0f4 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_whiteout` (LIBCMT/input.c; the character counter's address in ESI, passed first, and the
/// stream): reads characters, counting each, until one is `EOF` or not white space
/// (`isspace`), and returns it.
pub fn whiteout(e: &mut Engine, counter: u32, stream: u32) -> u32 {
    loop {
        let n = e.mem.u32(counter).wrapping_add(1);
        e.mem.set_u32(counter, n);
        let c = inc(e, stream);
        if c == u32::MAX || isspace(e, c & 0xff) == 0 {
            return c;
        }
    }
}

// Translated from 00eda0ab (decompiled, FalloutNV.exe 1.4.0.525)
/// `_hextodec` (LIBCMT/input.c; cdecl: the character, its low byte read): a digit as it is
/// (sign-extended), a letter as `(c & ~0x20) - 7`, so that `- '0'` gives its value.
pub fn hextodec(e: &mut Engine, c: u32) -> u32 {
    let byte = c as u8;
    let digit = isdigit(e, u32::from(byte));
    let value = i32::from(byte as i8) as u32;
    if digit != 0 {
        value
    } else {
        (value & 0xffff_ffdf).wrapping_sub(7)
    }
}

// Translated from 00eda054 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__check_float_string` (LIBCMT/input.c; the buffer size's address in ESI and the buffer
/// pointer's address in EDI, passed first, then cdecl: characters used, the initial stack
/// buffer, the "allocated" flag's address): when the buffer is full, the initial buffer is
/// replaced by a zeroed heap block twice its size (`__calloc_crt(size, 2)`, flag set to 1,
/// the old contents copied), a heap buffer is grown (`__recalloc_crt(buffer, size, 2)`), and
/// the size doubles. Returns 0 when the allocation failed (the pointer is then null only in
/// the first case), else 1.
pub fn check_float_string(
    e: &mut Engine,
    size_at: u32,
    buffer_at: u32,
    used: u32,
    initial: u32,
    allocated_at: u32,
) -> u32 {
    let size = e.mem.u32(size_at);
    if used == size {
        let buffer = e.mem.u32(buffer_at);
        if buffer == initial {
            let block = e.call(SCAN_CALLOC_CRT, &[size, 2]).u32();
            e.mem.set_u32(buffer_at, block);
            if block == 0 {
                return 0;
            }
            e.mem.set_u32(allocated_at, 1);
            let size = e.mem.u32(size_at);
            let block = e.mem.u32(buffer_at);
            e.call(SCAN_MEMCPY, &[block, initial, size]);
        } else {
            let block = e.call(SCAN_RECALLOC_CRT, &[buffer, size, 2]).u32();
            if block == 0 {
                return 0;
            }
            e.mem.set_u32(buffer_at, block);
        }
        let size = e.mem.u32(size_at);
        e.mem.set_u32(size_at, size << 1);
    }
    1
}

// Translated from 00ed0dbe (decompiled, FalloutNV.exe 1.4.0.525)
/// `__recalloc_crt` (LIBCMT/crtheap.c; cdecl: block, count, size): `_recalloc`, retried while
/// it fails for a nonzero size and `__maxwait` is above the wait so far: `Sleep(wait)`, the
/// wait growing by 1000 ms and becoming -1 (stop) past `__maxwait`. Returns the block or 0.
pub fn recalloc_crt(e: &mut Engine, block: u32, count: u32, size: u32) -> u32 {
    let mut wait = 0u32;
    loop {
        let result = e.call(SCAN_RECALLOC, &[block, count, size]).u32();
        // `CMP [__maxwait],EAX; JBE` with EAX the null result: a zero `__maxwait` stops.
        if result != 0 || size == 0 || e.mem.u32(MAX_WAIT) == 0 {
            return result;
        }
        e.call(IMPORT_SLEEP, &[wait]);
        let mut next = wait.wrapping_add(0x3e8);
        if next > e.mem.u32(MAX_WAIT) {
            next = u32::MAX;
        }
        wait = next;
        if wait == u32::MAX {
            return 0;
        }
    }
}

// Translated from 00edc81f (decompiled, FalloutNV.exe 1.4.0.525)
/// `_filbuf` (LIBCMT/_filbuf.c; cdecl: stream): refills a read stream's buffer and returns
/// its first byte. A null stream is an invalid parameter (`EINVAL`); a stream not open for
/// reading (`_IOREAD | _IOWRT | _IORW` clear) or a string stream (`_IOSTRG`) gives `EOF`; one
/// open for writing only (`_IOWRT`) gets `_IOERR` and `EOF`. Otherwise it is marked
/// `_IOREAD`, gets a buffer (`_getbuf` unless it has one: `_IOMYBUF | _IONBF | _IOYOURBUF`),
/// and `_read`s up to `_bufsiz` bytes into it; none read (or an error) sets `_IOEOF` (or
/// `_IOERR`), clears the count and gives `EOF`. A read on a handle open in text mode
/// to a device or pipe (`ioinfo` flags 0x82 both set) marks the stream `_IOCTRLZ`; a
/// default-sized 0x200 buffer of the stream's own (`_IOMYBUF` without `_IOSETVBUF`) grows to
/// 0x1000 for the next read.
pub fn filbuf(e: &mut Engine, stream: Ptr) -> i32 {
    let stream = stream.addr();
    if stream == 0 {
        let slot = errno(e);
        e.mem.set_u32(slot.addr(), 0x16);
        fn_00ec7c30(e, 0, 0, 0, 0, 0);
        return -1;
    }
    let flags = e.mem.u32(stream + 0xc);
    if flags & 0x83 == 0 || flags & 0x40 != 0 {
        return -1;
    }
    if flags & 2 != 0 {
        e.mem.set_u32(stream + 0xc, flags | 0x20);
        return -1;
    }
    let flags = flags | 1;
    e.mem.set_u32(stream + 0xc, flags);
    if flags & 0x10c == 0 {
        getbuf(e, Ptr::new(stream));
    } else {
        let base = e.mem.u32(stream + 8);
        e.mem.set_u32(stream, base);
    }
    let size = e.mem.u32(stream + 0x18);
    let base = e.mem.u32(stream + 8);
    let handle = fileno(e, Ptr::new(stream));
    let count = e.call(SCAN_READ, &[handle, base, size]).u32();
    e.mem.set_u32(stream + 4, count);
    if count == 0 || count == u32::MAX {
        // NEG; SBB; AND 0x10; ADD 0x10: _IOERR 0x20 for -1, _IOEOF 0x10 for 0.
        let bits = if count != 0 { 0x20 } else { 0x10 };
        let flags = e.mem.u32(stream + 0xc) | bits;
        e.mem.set_u32(stream + 0xc, flags);
        e.mem.set_u32(stream + 4, 0);
        return -1;
    }
    if e.mem.u32(stream + 0xc) & 0x82 == 0 {
        let handle = fileno(e, Ptr::new(stream));
        let info = if handle == u32::MAX || fileno(e, Ptr::new(stream)) == 0xffff_fffe {
            BAD_IOINFO
        } else {
            let block = fileno(e, Ptr::new(stream));
            let low = fileno(e, Ptr::new(stream));
            e.mem
                .u32(PIOINFO.wrapping_add((((block as i32) >> 5) as u32).wrapping_mul(4)))
                .wrapping_add((low & 0x1f) << 6)
        };
        if e.mem.u8(info + 4) & 0x82 == 0x82 {
            let flags = e.mem.u32(stream + 0xc) | 0x2000;
            e.mem.set_u32(stream + 0xc, flags);
        }
    }
    if e.mem.u32(stream + 0x18) == 0x200 {
        let flags = e.mem.u32(stream + 0xc);
        if flags & 8 != 0 && flags & 0x400 == 0 {
            e.mem.set_u32(stream + 0x18, 0x1000);
        }
    }
    let at = e.mem.u32(stream);
    let count = e.mem.u32(stream + 4).wrapping_sub(1);
    e.mem.set_u32(stream + 4, count);
    let c = e.mem.u8(at);
    e.mem.set_u32(stream, at.wrapping_add(1));
    i32::from(c)
}

// Translated from 00ecd4ce (decompiled, FalloutNV.exe 1.4.0.525)
/// `_ungetc_nolock` (LIBCMT/ungetc.c; cdecl: character, stream): pushes a character back.
/// A stream that is not a string stream must not be open on a handle in a Unicode text mode
/// (`ioinfo` +0x24: the 0x7f bits or bit 0x80), else `EINVAL`, invalid parameter, `EOF`.
/// `EOF`, or a stream neither reading (`_IOREAD`) nor read-write (`_IORW`) without writing
/// (`_IOWRT`), gives `EOF`. The stream gets a buffer if it has none (`_getbuf`); at the start
/// of the buffer with characters still counted nothing can go back (`EOF`), otherwise the
/// pointer moves back. A string stream only accepts the character already there (otherwise
/// the pointer moves forward again and `EOF`); any other stream gets the character written.
/// Then the count goes up, `_IOEOF` is cleared and `_IOREAD` set, and the character's low
/// byte is returned.
pub fn ungetc_nolock(e: &mut Engine, c: u32, stream: Ptr) -> i32 {
    let stream = stream.addr();
    if e.mem.u8(stream + 0xc) & 0x40 == 0 {
        let handle = fileno(e, Ptr::new(stream));
        let entry = |e: &Engine| {
            if handle == u32::MAX || handle == 0xffff_fffe {
                BAD_IOINFO
            } else {
                ioinfo(e, handle)
            }
        };
        let info = entry(e);
        let bad = if e.mem.u8(info + 0x24) & 0x7f != 0 {
            true
        } else {
            let info = entry(e);
            e.mem.u8(info + 0x24) & 0x80 != 0
        };
        if bad {
            let slot = errno(e);
            e.mem.set_u32(slot.addr(), 0x16);
            fn_00ec7c30(e, 0, 0, 0, 0, 0);
            return -1;
        }
    }
    if c == u32::MAX {
        return -1;
    }
    let flags = e.mem.u32(stream + 0xc);
    let low = flags as u8;
    if low & 1 == 0 && ((low as i8) >= 0 || low & 2 != 0) {
        return -1;
    }
    if e.mem.u32(stream + 8) == 0 {
        getbuf(e, Ptr::new(stream));
    }
    let at = e.mem.u32(stream);
    if at == e.mem.u32(stream + 8) {
        if e.mem.u32(stream + 4) != 0 {
            return -1;
        }
        e.mem.set_u32(stream, at.wrapping_add(1));
    }
    let at = e.mem.u32(stream).wrapping_sub(1);
    e.mem.set_u32(stream, at);
    if e.mem.u8(stream + 0xc) & 0x40 == 0 {
        e.mem.set_u8(at, c as u8);
    } else if e.mem.u8(at) != c as u8 {
        e.mem.set_u32(stream, at.wrapping_add(1));
        return -1;
    }
    let flags = e.mem.u32(stream + 0xc);
    let count = e.mem.u32(stream + 4).wrapping_add(1);
    e.mem.set_u32(stream + 4, count);
    e.mem.set_u32(stream + 0xc, (flags & 0xffff_ffef) | 1);
    (c & 0xff) as i32
}

// ---- Character classes -----------------------------------------------------------------------

/// The `_LocaleUpdate`-based class test the three `_l` forms share: `__isctype_l` for a
/// multibyte locale (`mb_cur_max` above 1), else the locale's ctype table (+0xC8), masked.
fn class_l(e: &mut Engine, c: u32, locale: u32, mask: u32) -> u32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &[update, locale]);
        let info = e.mem.u32(update);
        let result = if (e.mem.u32(info + 0xac) as i32) > 1 {
            e.call(ISCTYPE_L, &[c, mask, update]).u32()
        } else {
            let table = e.mem.u32(info + 0xc8);
            u32::from(e.mem.u16(table.wrapping_add(c.wrapping_mul(2)))) & mask
        };
        locale_update_release(e, update);
        result
    })
}

/// The plain forms: while `__locale_changed` is 0 the C table `_pctype` masked, else the `_l`
/// form with a null locale.
fn class(e: &mut Engine, c: u32, mask: u32, with_locale: fn(&mut Engine, u32, u32) -> u32) -> u32 {
    if e.mem.u32(LOCALE_CHANGED) == 0 {
        let table = e.mem.u32(PCTYPE);
        return u32::from(e.mem.u16(table.wrapping_add(c.wrapping_mul(2)))) & mask;
    }
    with_locale(e, c, 0)
}

// Translated from 00ec880f (decompiled, FalloutNV.exe 1.4.0.525)
/// `_isdigit_l` (LIBCMT/_ctype.c; cdecl: character, locale): class bit 4 (`_DIGIT`).
pub fn isdigit_l(e: &mut Engine, c: u32, locale: u32) -> u32 {
    class_l(e, c, locale, 4)
}

// Translated from 00ec8860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `isdigit` (LIBCMT/_ctype.c; cdecl): `_pctype[c] & _DIGIT`, or `_isdigit_l(c, NULL)` once
/// the locale has changed.
pub fn isdigit(e: &mut Engine, c: u32) -> u32 {
    class(e, c, 4, isdigit_l)
}

// Translated from 00ec8914 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_isspace_l` (LIBCMT/_ctype.c; cdecl: character, locale): class bit 8 (`_SPACE`).
pub fn isspace_l(e: &mut Engine, c: u32, locale: u32) -> u32 {
    class_l(e, c, locale, 8)
}

// Translated from 00ec8965 (decompiled, FalloutNV.exe 1.4.0.525)
/// `isspace` (LIBCMT/_ctype.c; cdecl): `_pctype[c] & _SPACE`, or `_isspace_l(c, NULL)`.
pub fn isspace(e: &mut Engine, c: u32) -> u32 {
    class(e, c, 8, isspace_l)
}

// Translated from 00ec888e (decompiled, FalloutNV.exe 1.4.0.525)
/// `_isxdigit_l` (LIBCMT/_ctype.c; cdecl: character, locale): class bit 0x80 (`_HEX`).
pub fn isxdigit_l(e: &mut Engine, c: u32, locale: u32) -> u32 {
    class_l(e, c, locale, 0x80)
}

// Translated from 00ec88e4 (decompiled, FalloutNV.exe 1.4.0.525)
/// `isxdigit` (LIBCMT/_ctype.c; cdecl): `_pctype[c] & _HEX`, or `_isxdigit_l(c, NULL)`.
pub fn isxdigit(e: &mut Engine, c: u32) -> u32 {
    class(e, c, 0x80, isxdigit_l)
}

// Translated from 00ec868b (disassembled, FalloutNV.exe 1.4.0.525)
/// `_isalpha_l` (LIBCMT/_ctype.c; cdecl: character, locale): class bits 0x103 (`_ALPHA`).
pub fn isalpha_l(e: &mut Engine, c: u32, locale: u32) -> u32 {
    class_l(e, c, locale, 0x103)
}

// Translated from 00ec8711 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_isupper_l` (LIBCMT/_ctype.c; cdecl: character, locale): class bit 1 (`_UPPER`).
pub fn isupper_l(e: &mut Engine, c: u32, locale: u32) -> u32 {
    class_l(e, c, locale, 1)
}

// Translated from 00ec8790 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_islower_l` (LIBCMT/_ctype.c; cdecl: character, locale): class bit 2 (`_LOWER`).
pub fn islower_l(e: &mut Engine, c: u32, locale: u32) -> u32 {
    class_l(e, c, locale, 2)
}

// Translated from 00ec8993 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_isalnum_l` (LIBCMT/_ctype.c; cdecl: character, locale): class bits 0x107 (`_ALPHA | _DIGIT`).
pub fn isalnum_l(e: &mut Engine, c: u32, locale: u32) -> u32 {
    class_l(e, c, locale, 0x107)
}

// Translated from 00ed41d4 (decompiled, FalloutNV.exe 1.4.0.525)
/// `isleadbyte` (LIBCMT/_wctype.c; cdecl): `_isleadbyte_l(c, NULL)`.
pub fn isleadbyte(e: &mut Engine, c: u32) -> u32 {
    e.call(ISLEADBYTE_L, &[c, 0]).u32()
}

// Translated from 00ee3928 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_mbtowc_l` (LIBCMT/mbtowc.c; cdecl: wide out, text, size, locale): 0 for a null text, a
/// zero size or an empty text (storing a wide 0 when the text is empty). In the "C" locale
/// (no `LC_CTYPE` handle, locale info +0x14) the byte widened, 1. Otherwise a lead byte
/// converts `mb_cur_max` bytes (`MultiByteToWideChar(code page, MB_PRECOMPOSED |
/// MB_ERR_INVALID_CHARS, ..)`, when the size allows) and answers `mb_cur_max` (also when that
/// fails but the size allows and a second byte is there); anything else converts one byte, 1.
/// A failure sets `errno` to `EILSEQ` (0x2A) and answers -1.
pub fn mbtowc_l(e: &mut Engine, out: u32, text: u32, size: u32, locale: u32) -> i32 {
    if text == 0 || size == 0 {
        return 0;
    }
    if e.mem.u8(text) == 0 {
        if out != 0 {
            e.mem.set_u16(out, 0);
        }
        return 0;
    }
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &[update, locale]);
        let info = e.mem.u32(update);
        let result = if e.mem.u32(info + 0x14) == 0 {
            if out != 0 {
                let byte = e.mem.u8(text);
                e.mem.set_u16(out, u16::from(byte));
            }
            1
        } else {
            let first = u32::from(e.mem.u8(text));
            if e.call(ISLEADBYTE_L, &[first, update]).u32() != 0 {
                let info = e.mem.u32(update);
                let max = e.mem.u32(info + 0xac);
                let converted = (max as i32) > 1 && (size as i32) >= (max as i32) && {
                    let page = e.mem.u32(info + 4);
                    let n = e
                        .call(
                            IMPORT_MULTI_BYTE_TO_WIDE_CHAR,
                            &[page, 9, text, max, out, u32::from(out != 0)],
                        )
                        .u32();
                    n != 0
                };
                let info = e.mem.u32(update);
                if converted || (size >= e.mem.u32(info + 0xac) && e.mem.u8(text + 1) != 0) {
                    e.mem.u32(info + 0xac) as i32
                } else {
                    let slot = errno(e);
                    e.mem.set_u32(slot.addr(), 0x2a);
                    -1
                }
            } else {
                let info = e.mem.u32(update);
                let page = e.mem.u32(info + 4);
                let n = e
                    .call(
                        IMPORT_MULTI_BYTE_TO_WIDE_CHAR,
                        &[page, 9, text, 1, out, u32::from(out != 0)],
                    )
                    .u32();
                if n != 0 {
                    1
                } else {
                    let slot = errno(e);
                    e.mem.set_u32(slot.addr(), 0x2a);
                    -1
                }
            }
        };
        locale_update_release(e, update);
        result
    })
}

// Translated from 00ecc300 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_allmul` (LIBCMT/llmul.asm; four words, callee pops 0x10): the low 64 bits of a 64-bit
/// product. With both high words zero one `MUL`; otherwise the low product plus the cross
/// products in the high word.
pub fn allmul(_e: &mut Engine, a: u64, b: u64) -> u64 {
    let (a_low, a_high) = (a as u32, (a >> 32) as u32);
    let (b_low, b_high) = (b as u32, (b >> 32) as u32);
    if a_high | b_high == 0 {
        return u64::from(a_low) * u64::from(b_low);
    }
    let cross = a_high
        .wrapping_mul(b_low)
        .wrapping_add(a_low.wrapping_mul(b_high));
    let low = u64::from(a_low) * u64::from(b_low);
    let high = ((low >> 32) as u32).wrapping_add(cross);
    (u64::from(high) << 32) | (low & 0xffff_ffff)
}

// ---- _input_l ------------------------------------------------------------------------------

/// `_input_l`'s frame (`SUB ESP,0x1fc`): the locals it passes by address, by their offset
/// below EBP.
const IN_FRAME: u32 = 0x1fc;
/// The `_LocaleUpdate` object (`locinfo`, `mbcinfo`, thread block, "updated" byte).
const IN_LOCALE: u32 = 0x1f8;
/// The wide character `_mbtowc_l` stores, and the two bytes it converts.
const IN_WIDE: u32 = 0x1e8;
const IN_MULTIBYTE: u32 = 0x1e0;
/// The float text buffer's size, its "allocated on the heap" flag, its pointer, the character
/// count and the initial 0x15E-byte buffer on the stack.
const IN_FLOAT_SIZE: u32 = 0x1dc;
const IN_FLOAT_ALLOCATED: u32 = 0x1d0;
const IN_FLOAT_BUFFER: u32 = 0x1ac;
const IN_COUNT: u32 = 0x18c;
const IN_FLOAT_INITIAL: u32 = 0x184;
/// The scan set's 32-byte bit table (`%[`).
const IN_SCANSET: u32 = 0x24;

/// Where `_input_l` goes after a step (its labels).
#[derive(Clone, Copy, PartialEq, Debug)]
enum In {
    /// `00eda248`: the next format character (not 0).
    Format,
    /// `00edafa4`: a directive done: count it and step past its conversion character.
    Done,
    /// `00edb019`: stop at the end of the input unless `%n` follows.
    AtEnd,
    /// `00edb035`: go on unless the format has ended.
    More,
    /// `00edb041`: push the current character back and stop.
    Unget,
    /// `00edb06b`: stop.
    Stop,
}

/// `_input_l`'s state across directives.
struct Input {
    stream: u32,
    ebp: u32,
    /// The format pointer (EDI) and its saved copy (`EBP-0x1b8`).
    fmt: u32,
    fmt_saved: u32,
    /// The current character (`EBP-0x188`), `EOF` as `0xffffffff`.
    ch: u32,
    /// Fields assigned (`EBP-0x1c4`) and directives matched (byte `EBP-0x19d`).
    assigned: u32,
    matched: u8,
    /// The argument list cursor and the one before the last argument was taken.
    args: u32,
    args_before: u32,
    /// The 64-bit number (`EBP-0x1cc`): only cleared by `ll`/`I64`, as in the exe.
    number64: u64,
}

impl Input {
    fn count_add(&self, e: &mut Engine, delta: i32) {
        let at = self.ebp - IN_COUNT;
        let n = e.mem.u32(at).wrapping_add(delta as u32);
        e.mem.set_u32(at, n);
    }
    /// `INC count; _inc(stream)` into the current character.
    fn next(&mut self, e: &mut Engine) {
        self.count_add(e, 1);
        self.ch = inc(e, self.stream);
    }
    /// `DEC count; _un_inc(ch, stream)`.
    fn back(&mut self, e: &mut Engine) {
        self.count_add(e, -1);
        un_inc(e, self.ch, self.stream);
    }
}

// Translated from 00eda11e (decompiled, FalloutNV.exe 1.4.0.525)
/// `_input_l` (LIBCMT/input.c; cdecl: stream, format, locale, argument list): the scan core
/// of `scanf`, `fscanf` and `sscanf`. A null format or stream, or a non-string stream open on
/// a handle in a Unicode text mode, sets `EINVAL`, reports an invalid parameter and returns -1.
///
/// White space in the format skips white space in the input (`_whiteout`, then the
/// character pushed back); other characters, and `%%`, must match the next input character
/// (both bytes of a lead byte pair), else the scan stops. A directive takes `*` (suppress),
/// a width, and the sizes `h`, `l`, `ll`, `L`, `w`, `I64`, `I32`, `I` (`N` and `F` are
/// read and ignored); its argument is taken unless suppressed. Except for `%c`, `%[` and
/// `%n` the input's white space is skipped first. Conversions:
/// - `d`, `u`, `o`, `x`, `p`, `i`: a sign, then digits (octal below '8'; hex through
///   `isxdigit`/`_hextodec`; `i` reads a `0x` prefix as hex and a leading 0 as octal, `x`
///   skips a `0x` prefix), within the width; stored as 16, 32 (`l`, `p`) or 64 bits (`ll`,
///   `I64`, multiplied through `_allmul`);
/// - `e`, `f`, `g`: a sign, digits, the locale's decimal point, digits, then `e`/`E` with
///   a sign and digits, copied to a text buffer (`__check_float_string` grows it) and
///   converted by the `_cfltcvt_tab` slot `011c1cf0` (`_fassign_l`: `float`, or `double`
///   with `l`/`L`/`h`);
/// - `c` (width 1 by default), `s` (up to white space 9..13 or ' '), `[` (a scan set, `^`
///   negates, `]` first is a member, `a-b` a range): bytes, or with `l`/`w`/`C`/`S` wide
///   characters through `_mbtowc_l`; `s` and `[` add a terminating 0;
/// - `n`: the characters read so far (16 or 32 bits; 64 bits are the cleared number);
/// - anything else must match the input character literally.
///
/// Returns the number of fields assigned; -1 when the input ended before any directive
/// matched.
// Translation incomplete: 00eda11e - the /GS stack cookie check (00ec408c) is not translated.
pub fn input_l(e: &mut Engine, stream: u32, format: u32, locale: u32, args: u32) -> i32 {
    e.with_stack(IN_FRAME, |e, frame| {
        let ebp = frame.addr() + IN_FRAME;
        e.mem.set_u32(ebp - IN_FLOAT_BUFFER, ebp - IN_FLOAT_INITIAL);
        e.mem.set_u32(ebp - IN_FLOAT_SIZE, 0x15e);
        e.mem.set_u32(ebp - IN_FLOAT_ALLOCATED, 0);
        e.mem.set_u32(ebp - IN_WIDE, 0);
        if format == 0 || stream == 0 {
            let slot = errno(e);
            e.mem.set_u32(slot.addr(), 0x16);
            fn_00ec7c30(e, 0, 0, 0, 0, 0);
            return -1;
        }
        if e.mem.u8(stream + 0xc) & 0x40 == 0 {
            let handle = fileno(e, Ptr::new(stream));
            let entry = |e: &Engine| {
                if handle == u32::MAX || handle == 0xffff_fffe {
                    BAD_IOINFO
                } else {
                    ioinfo(e, handle)
                }
            };
            let info = entry(e);
            let bad = if e.mem.u8(info + 0x24) & 0x7f != 0 {
                true
            } else {
                let info = entry(e);
                e.mem.u8(info + 0x24) & 0x80 != 0
            };
            if bad {
                let slot = errno(e);
                e.mem.set_u32(slot.addr(), 0x16);
                fn_00ec7c30(e, 0, 0, 0, 0, 0);
                return -1;
            }
        }
        let update = ebp - IN_LOCALE;
        e.call(LOCALE_UPDATE, &[update, locale]);
        let mut s = Input {
            stream,
            ebp,
            fmt: format,
            fmt_saved: format,
            ch: 0,
            assigned: 0,
            matched: 0,
            args,
            args_before: 0,
            number64: 0,
        };
        e.mem.set_u32(ebp - IN_COUNT, 0);
        if e.mem.u8(format) == 0 {
            locale_update_release(e, update);
            return s.assigned as i32;
        }
        let mut step = In::Format;
        loop {
            step = match step {
                In::Format => input_format(e, &mut s),
                In::Done => {
                    s.matched = s.matched.wrapping_add(1);
                    s.fmt_saved = s.fmt_saved.wrapping_add(1);
                    s.fmt = s.fmt_saved;
                    In::AtEnd
                }
                In::AtEnd => {
                    if s.ch == u32::MAX {
                        if e.mem.u8(s.fmt) != b'%' || e.mem.u8(s.fmt_saved + 1) != b'n' {
                            In::Stop
                        } else {
                            s.fmt = s.fmt_saved;
                            In::More
                        }
                    } else {
                        In::More
                    }
                }
                In::More => {
                    if e.mem.u8(s.fmt) != 0 {
                        In::Format
                    } else {
                        In::Stop
                    }
                }
                In::Unget => {
                    un_inc(e, s.ch, s.stream);
                    In::Stop
                }
                In::Stop => break,
            };
        }
        if e.mem.u32(ebp - IN_FLOAT_ALLOCATED) == 1 {
            let buffer = e.mem.u32(ebp - IN_FLOAT_BUFFER);
            e.call(FREE_CRT, &[buffer]);
        }
        let mut result = s.assigned;
        if s.ch == u32::MAX && result == 0 && s.matched == 0 {
            result = u32::MAX;
        }
        locale_update_release(e, update);
        result as i32
    })
}

/// `00eda248`: one format character: white space, a literal, or a directive.
fn input_format(e: &mut Engine, s: &mut Input) -> In {
    let c = e.mem.u8(s.fmt);
    if isspace(e, u32::from(c)) != 0 {
        s.count_add(e, -1);
        let counter = s.ebp - IN_COUNT;
        let r = whiteout(e, counter, s.stream);
        un_inc(e, r, s.stream);
        loop {
            s.fmt = s.fmt.wrapping_add(1);
            let next = e.mem.u8(s.fmt);
            if isspace(e, u32::from(next)) == 0 {
                break;
            }
        }
        return In::More;
    }
    let c = e.mem.u8(s.fmt);
    if c == b'%' && e.mem.u8(s.fmt + 1) != c {
        return input_directive(e, s);
    }
    if c == b'%' && e.mem.u8(s.fmt + 1) == b'%' {
        s.fmt += 1;
    }
    // 00edafc7: a literal character.
    s.count_add(e, 1);
    let first = inc(e, s.stream);
    let want = u32::from(e.mem.u8(s.fmt));
    s.fmt = s.fmt.wrapping_add(1);
    s.ch = first;
    s.fmt_saved = s.fmt;
    if want != first {
        un_inc(e, first, s.stream);
        return In::Stop;
    }
    if isleadbyte(e, first & 0xff) != 0 {
        s.count_add(e, 1);
        let second = inc(e, s.stream);
        let want = u32::from(e.mem.u8(s.fmt));
        s.fmt = s.fmt.wrapping_add(1);
        s.fmt_saved = s.fmt;
        if want != second {
            un_inc(e, second, s.stream);
            un_inc(e, first, s.stream);
            return In::Stop;
        }
        s.count_add(e, -1);
    }
    In::AtEnd
}

/// One directive's flags (the locals `_input_l` clears at `00eda29f`).
struct Directive {
    suppress: i8,
    long_one: i8,
    wide_char: i8,
    wide: u8,
    negative: u8,
    reject: u8,
    started: u32,
    width_set: u32,
    width: i32,
    i64: u32,
    start: u32,
    arg: u32,
    kind: u32,
    /// BL: the width ran out while reading a prefix.
    ended: u8,
}

/// `00eda29f`: a `%` directive.
fn input_directive(e: &mut Engine, s: &mut Input) -> In {
    let mut d = Directive {
        suppress: 0,
        long_one: 1,
        wide_char: 0,
        wide: 0,
        negative: 0,
        reject: 0,
        started: 0,
        width_set: 0,
        width: 0,
        i64: 0,
        start: 0,
        arg: 0,
        kind: 0,
        ended: 0,
    };
    let mut done: u8 = 0;
    // The scan set's previous character (EBP-0x1d1), cleared and never set.
    let prev_char: u8 = 0;
    loop {
        s.fmt = s.fmt.wrapping_add(1);
        let c = u32::from(e.mem.u8(s.fmt));
        if isdigit(e, c) != 0 {
            d.width_set = d.width_set.wrapping_add(1);
            d.width = d.width.wrapping_mul(10).wrapping_add(c as i32 - 0x30);
        } else if c > 0x4e {
            match c {
                0x68 => {
                    d.long_one = d.long_one.wrapping_sub(1);
                    d.wide_char = d.wide_char.wrapping_sub(1);
                }
                0x6c => {
                    if e.mem.u8(s.fmt + 1) == b'l' {
                        d.i64 = d.i64.wrapping_add(1);
                        s.fmt += 1;
                        s.number64 = 0;
                    } else {
                        d.long_one = d.long_one.wrapping_add(1);
                        d.wide_char = d.wide_char.wrapping_add(1);
                    }
                }
                0x77 => d.wide_char = d.wide_char.wrapping_add(1),
                _ => done = done.wrapping_add(1),
            }
        } else if c == 0x4e {
            // 'N': ignored.
        } else {
            match c {
                0x2a => d.suppress = d.suppress.wrapping_add(1),
                0x46 => {}
                0x49 => {
                    let next = e.mem.u8(s.fmt + 1);
                    if next == b'6' && e.mem.u8(s.fmt + 2) == b'4' {
                        d.i64 = d.i64.wrapping_add(1);
                        s.fmt += 2;
                        s.number64 = 0;
                    } else if next == b'3' && e.mem.u8(s.fmt + 2) == b'2' {
                        s.fmt += 2;
                    } else if !matches!(next, b'd' | b'i' | b'o' | b'x' | b'X') {
                        done = done.wrapping_add(1);
                    }
                }
                0x4c => d.long_one = d.long_one.wrapping_add(1),
                _ => done = done.wrapping_add(1),
            }
        }
        if done != 0 {
            break;
        }
    }
    s.fmt_saved = s.fmt;
    if d.suppress == 0 {
        d.arg = e.mem.u32(s.args);
        s.args_before = s.args;
        s.args = s.args.wrapping_add(4);
    } else {
        d.arg = 0;
    }
    if d.wide_char == 0 {
        let c = e.mem.u8(s.fmt);
        d.wide_char = if c == b'S' || c == b'C' { 1 } else { -1 };
    }
    d.kind = u32::from(e.mem.u8(s.fmt)) | 0x20;
    if d.kind != 0x6e {
        if d.kind == 0x63 || d.kind == 0x7b {
            s.next(e);
        } else {
            let counter = s.ebp - IN_COUNT;
            s.ch = whiteout(e, counter, s.stream);
        }
        if s.ch == u32::MAX {
            return In::Stop;
        }
    }
    if d.width_set != 0 && d.width == 0 {
        return In::Unget;
    }
    match d.kind {
        0x70 => {
            d.long_one = 1;
            input_integer(e, s, &mut d, true)
        }
        0x73 => input_chars(e, s, &mut d, false),
        0x75 | 0x6f | 0x64 => input_integer(e, s, &mut d, true),
        0x78 => input_integer(e, s, &mut d, false),
        0x7b => {
            if d.wide_char > 0 {
                d.wide = 1;
            }
            let mut at = s.fmt.wrapping_add(1);
            if e.mem.u8(at) == b'^' {
                at = at.wrapping_add(1);
                d.reject = 0xff;
            }
            let table = s.ebp - IN_SCANSET;
            mem::memset(e, table, 0, 0x20);
            let mut last = if e.mem.u8(at) == b']' {
                at = at.wrapping_add(1);
                e.mem.set_u8(table + 0xb, 0x20);
                b']'
            } else {
                prev_char
            };
            loop {
                let c = e.mem.u8(at);
                if c == b']' {
                    break;
                }
                at = at.wrapping_add(1);
                if c == b'-' && last != 0 && e.mem.u8(at) != b']' {
                    let upper = e.mem.u8(at);
                    at = at.wrapping_add(1);
                    let (low, high) = if last >= upper {
                        (upper, last)
                    } else {
                        (last, upper)
                    };
                    if low <= high {
                        let mut bit = u32::from(low);
                        let mut n = u32::from(high.wrapping_sub(low).wrapping_add(1));
                        loop {
                            let byte = table.wrapping_add(bit >> 3);
                            let v = e.mem.u8(byte) | (1u8 << (bit & 7));
                            e.mem.set_u8(byte, v);
                            bit = bit.wrapping_add(1);
                            n = n.wrapping_sub(1);
                            if n == 0 {
                                break;
                            }
                        }
                    }
                    last = 0;
                } else {
                    let byte = table + u32::from(c >> 3);
                    let v = e.mem.u8(byte) | (1u8 << (c & 7));
                    e.mem.set_u8(byte, v);
                    last = c;
                }
            }
            // The exe tests the ']' it stopped on for 0 (never true).
            if e.mem.u8(at) == 0 {
                return In::Stop;
            }
            s.fmt_saved = at;
            input_chars(e, s, &mut d, true)
        }
        0x63 => {
            if d.width_set == 0 {
                d.width = d.width.wrapping_add(1);
                d.width_set = 1;
            }
            input_chars(e, s, &mut d, false)
        }
        0x65..=0x67 => input_float(e, s, &mut d),
        0x69 => {
            d.kind = 0x64;
            input_integer(e, s, &mut d, false)
        }
        0x6e => {
            if d.suppress != 0 {
                return In::Done;
            }
            let count = e.mem.u32(s.ebp - IN_COUNT);
            input_store(e, s, &d, count)
        }
        _ => {
            // 00edaa66: the conversion character must match the input.
            if u32::from(e.mem.u8(s.fmt_saved)) != s.ch {
                return In::Unget;
            }
            s.matched = s.matched.wrapping_sub(1);
            if d.suppress == 0 {
                s.args = s.args_before;
            }
            In::Done
        }
    }
}

/// `00edaf78`: stores an integer result (64-bit number, 32-bit `value`, or its low 16 bits).
fn input_store(e: &mut Engine, s: &Input, d: &Directive, value: u32) -> In {
    if d.i64 != 0 {
        e.mem.set_u32(d.arg, s.number64 as u32);
        e.mem.set_u32(d.arg + 4, (s.number64 >> 32) as u32);
    } else if d.long_one != 0 {
        e.mem.set_u32(d.arg, value);
    } else {
        e.mem.set_u16(d.arg, value as u16);
    }
    In::Done
}

/// `00eda909`: `%c`, `%s` and `%[` (`scanset` true for `%[`, whose table is built).
fn input_chars(e: &mut Engine, s: &mut Input, d: &mut Directive, scanset: bool) -> In {
    // 00eda8f9 (`%c`, `%s`; `%[` set its own at 00edaa99).
    if !scanset && d.wide_char > 0 {
        d.wide = 1;
    }
    s.back(e);
    let mut out = d.arg;
    d.start = out;
    let finished;
    loop {
        if d.width_set != 0 {
            let w = d.width;
            d.width = d.width.wrapping_sub(1);
            if w == 0 {
                finished = false;
                break;
            }
        }
        s.next(e);
        let c = s.ch;
        if c == u32::MAX {
            finished = true;
            break;
        }
        let take = if d.kind == 0x63 {
            true
        } else if d.kind == 0x73 && (9..=0xd).contains(&(c as i32)) {
            false
        } else if d.kind == 0x73 && c != 0x20 {
            true
        } else if d.kind != 0x7b {
            false
        } else {
            let byte = e
                .mem
                .u8((s.ebp - IN_SCANSET).wrapping_add(((c as i32) >> 3) as u32));
            let bits = i32::from(byte as i8) ^ i32::from(d.reject as i8);
            bits & (1 << (c & 7)) != 0
        };
        if !take {
            finished = true;
            break;
        }
        if d.suppress != 0 {
            d.start = d.start.wrapping_add(1);
        } else if d.wide != 0 {
            let multibyte = s.ebp - IN_MULTIBYTE;
            e.mem.set_u8(multibyte, c as u8);
            if isleadbyte(e, c & 0xff) != 0 {
                s.count_add(e, 1);
                let second = inc(e, s.stream);
                e.mem.set_u8(multibyte + 1, second as u8);
            }
            let wide = s.ebp - IN_WIDE;
            e.mem.set_u32(wide, 0x3f);
            let info = e.mem.u32(s.ebp - IN_LOCALE);
            let max = e.mem.u32(info + 0xac);
            mbtowc_l(e, wide, multibyte, max, s.ebp - IN_LOCALE);
            let w = e.mem.u16(wide);
            e.mem.set_u16(out, w);
            out = out.wrapping_add(2);
            d.arg = out;
        } else {
            e.mem.set_u8(out, c as u8);
            out = out.wrapping_add(1);
            d.arg = out;
        }
    }
    if finished {
        // 00edac6a
        s.back(e);
    }
    // 00edac79
    if d.start == out {
        return In::Stop;
    }
    if d.suppress != 0 {
        return In::Done;
    }
    s.assigned = s.assigned.wrapping_add(1);
    if d.kind == 0x63 {
        return In::Done;
    }
    if d.wide != 0 {
        e.mem.set_u16(d.arg, 0);
    } else {
        e.mem.set_u8(d.arg, 0);
    }
    In::Done
}

/// `00edacd3` (`signed_entry`: `d`, `o`, `u`, `p`) and `00eda51d` (`x`, and `i` as `d`):
/// the sign, a prefix, the digits and the store.
fn input_integer(e: &mut Engine, s: &mut Input, d: &mut Directive, signed_entry: bool) -> In {
    let sign = s.ch == 0x2d || s.ch == 0x2b;
    if s.ch == 0x2d {
        d.negative = 1;
    }
    if sign {
        d.width = d.width.wrapping_sub(1);
        if d.width == 0 && d.width_set != 0 {
            d.ended = 1;
        } else {
            s.next(e);
        }
    }
    if !signed_entry && s.ch == 0x30 {
        // 00edab9d: a 0 prefix.
        s.next(e);
        let c = s.ch as u8;
        if c == b'x' || c == b'X' {
            s.next(e);
            if d.width_set != 0 {
                d.width = d.width.wrapping_sub(2);
                if d.width < 1 {
                    d.ended = d.ended.wrapping_add(1);
                }
            }
            d.kind = 0x78;
        } else {
            d.started = 1;
            if d.kind == 0x78 {
                s.back(e);
                s.ch = 0x30;
            } else {
                if d.width_set != 0 {
                    d.width = d.width.wrapping_sub(1);
                    if d.width == 0 {
                        d.ended = d.ended.wrapping_add(1);
                    }
                }
                d.kind = 0x6f;
            }
        }
    }
    // 00edad15
    let value;
    if d.i64 != 0 {
        if d.ended == 0 {
            let mut unget = true;
            loop {
                let next;
                if d.kind == 0x78 || d.kind == 0x70 {
                    if isxdigit(e, s.ch & 0xff) == 0 {
                        break;
                    }
                    next = s.number64 << 4;
                    s.ch = hextodec(e, s.ch);
                } else {
                    if isdigit(e, s.ch & 0xff) == 0 {
                        break;
                    }
                    if d.kind == 0x6f {
                        if (s.ch as i32) >= 0x38 {
                            break;
                        }
                        next = s.number64 << 3;
                    } else {
                        next = allmul(e, s.number64, 10);
                    }
                }
                d.started = d.started.wrapping_add(1);
                let digit = i64::from(s.ch.wrapping_sub(0x30) as i32) as u64;
                s.number64 = next.wrapping_add(digit);
                if d.width_set != 0 {
                    d.width = d.width.wrapping_sub(1);
                    if d.width == 0 {
                        unget = false;
                        break;
                    }
                }
                s.next(e);
            }
            if unget {
                s.back(e);
            }
        }
        // 00edae35
        value = d.start;
        if d.negative != 0 {
            s.number64 = s.number64.wrapping_neg();
        }
    } else {
        let mut v = d.start;
        if d.ended == 0 {
            let mut unget = true;
            loop {
                if d.kind == 0x78 || d.kind == 0x70 {
                    if isxdigit(e, s.ch & 0xff) == 0 {
                        break;
                    }
                    v <<= 4;
                    s.ch = hextodec(e, s.ch);
                } else {
                    if isdigit(e, s.ch & 0xff) == 0 {
                        break;
                    }
                    if d.kind == 0x6f {
                        if (s.ch as i32) >= 0x38 {
                            break;
                        }
                        v <<= 3;
                    } else {
                        v = v.wrapping_mul(10);
                    }
                }
                d.started = d.started.wrapping_add(1);
                v = v.wrapping_add(s.ch).wrapping_sub(0x30);
                if d.width_set != 0 {
                    d.width = d.width.wrapping_sub(1);
                    if d.width == 0 {
                        unget = false;
                        break;
                    }
                }
                s.next(e);
            }
            if unget {
                s.back(e);
            }
        }
        // 00edaf3b
        if d.negative != 0 {
            v = v.wrapping_neg();
        }
        value = v;
    }
    // 00edaf46 ('F' cannot come here: the conversion character was OR'd with 0x20).
    if d.kind == 0x46 {
        d.started = 0;
    }
    if d.started == 0 {
        return In::Stop;
    }
    if d.suppress != 0 {
        return In::Done;
    }
    s.assigned = s.assigned.wrapping_add(1);
    input_store(e, s, d, value)
}

/// `00eda536`: `%e`, `%f`, `%g`: the text into the float buffer, then `_fassign_l` through
/// `_cfltcvt_tab`.
fn input_float(e: &mut Engine, s: &mut Input, d: &mut Directive) -> In {
    let ebp = s.ebp;
    let size_at = ebp - IN_FLOAT_SIZE;
    let buffer_at = ebp - IN_FLOAT_BUFFER;
    let initial = ebp - IN_FLOAT_INITIAL;
    let allocated_at = ebp - IN_FLOAT_ALLOCATED;
    let put = |e: &mut Engine, at: u32, c: u8| {
        let buffer = e.mem.u32(buffer_at);
        e.mem.set_u8(buffer.wrapping_add(at), c);
    };
    let mut used = 0u32;
    if s.ch == 0x2d {
        put(e, 0, b'-');
        used = 1;
    }
    if s.ch == 0x2d || s.ch == 0x2b {
        d.width = d.width.wrapping_sub(1);
        s.next(e);
    }
    if d.width_set == 0 {
        d.width = -1;
    }
    // The digit loops at 00eda58c, 00eda693 and 00eda7ff.
    let digits = |e: &mut Engine, s: &mut Input, d: &mut Directive, used: &mut u32| -> bool {
        loop {
            if isdigit(e, s.ch & 0xff) == 0 {
                return true;
            }
            let w = d.width;
            d.width = d.width.wrapping_sub(1);
            if w == 0 {
                return true;
            }
            d.started = d.started.wrapping_add(1);
            put(e, *used, s.ch as u8);
            *used += 1;
            if check_float_string(e, size_at, buffer_at, *used, initial, allocated_at) == 0 {
                return false;
            }
            s.next(e);
        }
    };
    if !digits(e, s, d, &mut used) {
        return In::Stop;
    }
    let info = e.mem.u32(ebp - IN_LOCALE);
    let lconv = e.mem.u32(info + 0xbc);
    let point = e.mem.u8(e.mem.u32(lconv));
    d.reject = point;
    if point == s.ch as u8 {
        let w = d.width;
        d.width = d.width.wrapping_sub(1);
        if w != 0 {
            s.next(e);
            put(e, used, point);
            used += 1;
            if check_float_string(e, size_at, buffer_at, used, initial, allocated_at) == 0 {
                return In::Stop;
            }
            if !digits(e, s, d, &mut used) {
                return In::Stop;
            }
        }
    }
    if d.started != 0 && (s.ch == 0x65 || s.ch == 0x45) {
        let w = d.width;
        d.width = d.width.wrapping_sub(1);
        if w != 0 {
            put(e, used, b'e');
            used += 1;
            if check_float_string(e, size_at, buffer_at, used, initial, allocated_at) == 0 {
                return In::Stop;
            }
            s.next(e);
            let sign = s.ch == 0x2d || s.ch == 0x2b;
            if s.ch == 0x2d {
                put(e, used, b'-');
                used += 1;
                if check_float_string(e, size_at, buffer_at, used, initial, allocated_at) == 0 {
                    return In::Stop;
                }
            }
            if sign {
                let w = d.width;
                d.width = d.width.wrapping_sub(1);
                if w == 0 {
                    d.width = 0;
                } else {
                    s.next(e);
                }
            }
            if !digits(e, s, d, &mut used) {
                return In::Stop;
            }
        }
    }
    // 00eda875
    s.back(e);
    if d.started == 0 {
        return In::Stop;
    }
    if d.suppress != 0 {
        return In::Done;
    }
    s.assigned = s.assigned.wrapping_add(1);
    put(e, used, 0);
    let buffer = e.mem.u32(buffer_at);
    let flag = (i32::from(d.long_one) - 1) as u32;
    let encoded = e.mem.u32(CFLTCVT_FASSIGN);
    let fassign = e.call(DECODE_POINTER, &[encoded]).u32();
    e.call(fassign, &[flag, d.arg, buffer, ebp - IN_LOCALE]);
    In::Done
}

// ---- The float conversion table ------------------------------------------------------------

// Translated from 00ec5eed (decompiled, FalloutNV.exe 1.4.0.525)
/// `_cfltcvt_init` (LIBCMT/cmiscdat.c; called by `_fpmath`): fills `_cfltcvt_tab` with the
/// floating-point helpers (`_cfltcvt` twice, `_cropzeros`, `_fassign`, `_forcdecpt`,
/// `_positive`, `_cldcvt`, `_fassign_l`, `_cropzeros_l`, `_forcdecpt_l`), plain;
/// `__initp_misc_cfltcvt_tab` encodes them afterwards.
pub fn cfltcvt_init(e: &mut Engine) {
    let cfltcvt = 0x00ed_246b;
    e.mem.set_u32(CFLTCVT_TAB, cfltcvt);
    e.mem.set_u32(CFLTCVT_TAB + 4, 0x00ed_1b35);
    e.mem.set_u32(CFLTCVT_TAB + 8, 0x00ed_1ae9);
    e.mem.set_u32(CFLTCVT_TAB + 0xc, 0x00ed_1b22);
    e.mem.set_u32(CFLTCVT_TAB + 0x10, 0x00ed_1a8b);
    e.mem.set_u32(CFLTCVT_TAB + 0x14, cfltcvt);
    e.mem.set_u32(CFLTCVT_TAB + 0x18, 0x00ed_23e3);
    e.mem.set_u32(CFLTCVT_TAB + 0x1c, 0x00ed_1aa7);
    e.mem.set_u32(CFLTCVT_TAB + 0x20, 0x00ed_1a09);
    e.mem.set_u32(CFLTCVT_TAB + 0x24, 0x00ed_1996);
}

// Translated from 00ed248e (decompiled, FalloutNV.exe 1.4.0.525)
/// `__initp_misc_cfltcvt_tab` (LIBCMT/cmiscdat.c; `__cinit` calls it after `_fpmath`):
/// each of the ten `_cfltcvt_tab` slots replaced by `__encode_pointer` of itself.
pub fn initp_misc_cfltcvt_tab(e: &mut Engine) {
    let mut offset = 0;
    while offset < 0x28 {
        let slot = CFLTCVT_TAB + offset;
        let value = e.mem.u32(slot);
        let encoded = e.call(SCAN_ENCODE_POINTER, &[value]).u32();
        offset += 4;
        e.mem.set_u32(slot, encoded);
    }
}

// ---- Text to float -------------------------------------------------------------------------

// Translated from 00ed1aa7 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_fassign_l` (LIBCMT/cvt.c; cdecl: flag, argument, text, locale): with a flag the text as a
/// `double` (`_atodbl_l`) stored at `argument`, else as a `float` (`_atoflt_l`, into the
/// flag's own stack slot) stored there. The conversion's status is not looked at.
pub fn fassign_l(e: &mut Engine, flag: u32, argument: u32, text: u32, locale: u32) {
    if flag != 0 {
        e.with_stack(8, |e, value| {
            let value = value.addr();
            atodbl_l(e, value, text, locale);
            let low = e.mem.u32(value);
            e.mem.set_u32(argument, low);
            let high = e.mem.u32(value + 4);
            e.mem.set_u32(argument + 4, high);
        });
    } else {
        e.with_stack(4, |e, value| {
            let value = value.addr();
            atoflt_l(e, value, text, locale);
            let bits = e.mem.u32(value);
            e.mem.set_u32(argument, bits);
        });
    }
}

/// `_atodbl_l` and `_atoflt_l` (the same code with `_ld12tod` or `_ld12tof`): the text to
/// `_LDBL12` (`__strgtold12_l` with no scale, decimal point or implicit exponent and the low
/// word dropped before scaling), then to the result. Returns `_UNDERFLOW` 4 when the text's
/// exponent was below the range or the conversion underflowed, `_OVERFLOW` 3 when above it or
/// the conversion overflowed, else 0. Frame: the `_LocaleUpdate` object at EBP-0x24, the end
/// pointer at EBP-0x28, the `_LDBL12` at EBP-0x10.
fn ato_l(
    e: &mut Engine,
    result: u32,
    text: u32,
    locale: u32,
    convert: fn(&mut Engine, u32, u32) -> u32,
) -> u32 {
    e.with_stack(0x28, |e, frame| {
        let ebp = frame.addr() + 0x28;
        let update = ebp - 0x24;
        e.call(LOCALE_UPDATE, &[update, locale]);
        let value = ebp - 0x10;
        let flags = strgtold12_l(e, value, ebp - 0x28, text, 0, 0, 0, 0, update);
        let status = convert(e, value, result);
        let answer = if flags & 3 != 0 {
            if flags & 1 != 0 {
                4
            } else if flags & 2 != 0 {
                3
            } else {
                0
            }
        } else if status == 1 {
            3
        } else if status == 2 {
            4
        } else {
            0
        };
        locale_update_release(e, update);
        answer
    })
}

// Translated from 00ee177b (decompiled, FalloutNV.exe 1.4.0.525)
/// `_atodbl_l` (LIBCMT/atodbl.c; cdecl: result, text, locale): the text as a `double`
/// (see [`ato_l`]).
// Translation incomplete: 00ee177b - the /GS stack cookie check (00ec408c) is not translated.
pub fn atodbl_l(e: &mut Engine, result: u32, text: u32, locale: u32) -> u32 {
    ato_l(e, result, text, locale, ld12tod)
}

// Translated from 00ee1823 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_atoflt_l` (LIBCMT/atodbl.c; cdecl: result, text, locale): the text as a `float`
/// (see [`ato_l`]).
// Translation incomplete: 00ee1823 - the /GS stack cookie check (00ec408c) is not translated.
pub fn atoflt_l(e: &mut Engine, result: u32, text: u32, locale: u32) -> u32 {
    ato_l(e, result, text, locale, ld12tof)
}

/// `__strgtold12_l`'s frame: its size and the locals by their offset below EBP.
const ST_FRAME: u32 = 0x7c;
/// The digits (up to 25 bytes from EBP-0x20), the mantissa `_LDBL12`, the copy of an
/// adjusted power of ten and the 96-bit product.
const ST_DIGITS: u32 = 0x20;
const ST_VALUE: u32 = 0x3c;
const ST_POWER: u32 = 0x48;
const ST_PRODUCT: u32 = 0x2c;

// Translated from 00edb182 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__strgtold12_l` (LIBCMT/strgtold.c; cdecl: result, end, text, mult12, scale, decpt,
/// implicit_E, locale): parses a decimal floating-point number into a `_LDBL12`.
/// A null locale is an invalid parameter (`EINVAL`) and answers 0 with nothing stored.
///
/// Skips ' ', '\t', '\n', '\r', then a state machine reads a sign, digits, the locale's
/// decimal point (locale info +0xBC), digits, and an exponent (`D`, `d`, `E`, `e`, or a bare
/// sign with `implicit_E`) with up to 0x1451 as its value; the end pointer is the first
/// character not taken (the start of the text when nothing was taken, the exponent letter
/// when its digits are missing). The first 25 significant digits are kept: with more than 24
/// the 24th is rounded up when it is 5 or more (it looks at itself, as the exe does) and the
/// rest dropped; trailing zeros are dropped. `__mtold12` makes the mantissa; the decimal
/// exponent (plus `scale` without an exponent, minus `decpt` without a decimal point) above
/// 0x1450 gives infinity and flag 2, below -0x1450 zero and flag 1; no digits zero and flag 4.
/// Otherwise the mantissa is multiplied by the powers of ten of the exponent's octal digits
/// (`_pow10pos`/`_pow10neg`; an entry whose low word is 0x8000 or more is one less in its
/// middle words; `mult12` 0 clears the low word first), each product rounded to 80 bits
/// (half rounds up only with the last kept bit set or a lower bit set) and turned into
/// infinity or zero when out of range. The sign is or'd in last. Returns the flags.
// Translation incomplete: 00edb182 - the /GS stack cookie check (00ec408c) is not translated.
pub fn strgtold12_l(
    e: &mut Engine,
    result: u32,
    end: u32,
    text: u32,
    mult12: u32,
    scale: i32,
    decpt: i32,
    implicit_e: u32,
    locale: u32,
) -> u32 {
    e.with_stack(ST_FRAME, |e, frame| {
        let ebp = frame.addr() + ST_FRAME;
        strgtold12_body(
            e, ebp, result, end, text, mult12, scale, decpt, implicit_e, locale,
        )
    })
}

/// The decimal point character: `*locale->locinfo->lconv->decimal_point`.
fn decimal_point(e: &Engine, locale: u32) -> u8 {
    let info = e.mem.u32(locale);
    let lconv = e.mem.u32(info + 0xbc);
    let text = e.mem.u32(lconv);
    e.mem.u8(text)
}

#[allow(clippy::too_many_arguments)]
fn strgtold12_body(
    e: &mut Engine,
    ebp: u32,
    result: u32,
    end: u32,
    text: u32,
    mult12: u32,
    scale: i32,
    decpt: i32,
    implicit_e: u32,
    locale: u32,
) -> u32 {
    let mut sign: u32 = 0; // EBP-0x74
    let mut exponent_sign: i32 = 1; // EBP-0x68
    let mut count: u32 = 0; // EBP-0x4c
    let mut found: u32 = 0; // EBP-0x58
    let mut point_seen: u32 = 0; // EBP-0x5c
    let mut exponent_seen: u32 = 0; // EBP-0x60
    let mut exponent: i32 = 0; // EBP-0x64
    let mut adjust: i32 = 0; // EBP-0x50
    let mut flags: u32 = 0; // EBP-0x6c
    let mut digit_at = ebp - ST_DIGITS; // EDI
    if locale == 0 {
        let slot = errno(e);
        e.mem.set_u32(slot.addr(), 0x16);
        fn_00ec7c30(e, 0, 0, 0, 0, 0);
        return 0;
    }
    let mut saved = text; // EBP-0x54
    let mut p = text; // EDX
    loop {
        let c = e.mem.u8(p);
        if c == 0x20 || c == 9 || c == 0xa || c == 0xd {
            p = p.wrapping_add(1);
        } else {
            break;
        }
    }
    let mut state: u32 = 0; // ECX
    let is_19 = |c: u8| c.wrapping_sub(0x31) <= 8;
    let is_digit = |c: u8| (0x30..=0x39).contains(&(c as i8));
    loop {
        // 00edb209: read a character and switch on the state.
        let mut c = e.mem.u8(p);
        p = p.wrapping_add(1);
        let go = match state {
            0 => {
                if is_19(c) {
                    state = 3;
                    StGo::Back
                } else if c == decimal_point(e, locale) {
                    state = 5;
                    StGo::Read
                } else if c == b'+' {
                    sign = 0;
                    state = 2;
                    StGo::Read
                } else if c == b'-' {
                    state = 2;
                    sign = 0x8000;
                    StGo::Read
                } else if c == b'0' {
                    state = 1;
                    StGo::Read
                } else {
                    StGo::EndBack
                }
            }
            1 => {
                found = 1;
                if is_19(c) {
                    state = 3;
                    StGo::Back
                } else if c == decimal_point(e, locale) {
                    state = 4;
                    StGo::Read
                } else if c == b'+' || c == b'-' {
                    p = p.wrapping_sub(1);
                    state = 0xb;
                    StGo::Read
                } else if c == b'0' {
                    state = 1;
                    StGo::Read
                } else {
                    exponent_letter(c, &mut state)
                }
            }
            2 => {
                if is_19(c) {
                    state = 3;
                    StGo::Back
                } else if c == decimal_point(e, locale) {
                    state = 5;
                    StGo::Read
                } else if c == b'0' {
                    state = 1;
                    StGo::Read
                } else {
                    StGo::EndSaved
                }
            }
            3 => {
                found = 1;
                while is_digit(c) {
                    if count < 0x19 {
                        count += 1;
                        e.mem.set_u8(digit_at, c.wrapping_sub(0x30));
                        digit_at = digit_at.wrapping_add(1);
                    } else {
                        adjust = adjust.wrapping_add(1);
                    }
                    c = e.mem.u8(p);
                    p = p.wrapping_add(1);
                }
                if c == decimal_point(e, locale) {
                    state = 4;
                    StGo::Read
                } else {
                    sign_or_letter(c, &mut p, &mut state)
                }
            }
            4 => {
                found = 1;
                point_seen = 1;
                if count == 0 {
                    while c == b'0' {
                        adjust = adjust.wrapping_sub(1);
                        c = e.mem.u8(p);
                        p = p.wrapping_add(1);
                    }
                }
                while is_digit(c) {
                    if count < 0x19 {
                        count += 1;
                        e.mem.set_u8(digit_at, c.wrapping_sub(0x30));
                        digit_at = digit_at.wrapping_add(1);
                        adjust = adjust.wrapping_sub(1);
                    }
                    c = e.mem.u8(p);
                    p = p.wrapping_add(1);
                }
                // 00edb331 (no second decimal point).
                sign_or_letter(c, &mut p, &mut state)
            }
            5 => {
                point_seen = 1;
                if c.wrapping_sub(0x30) > 9 {
                    StGo::EndSaved
                } else {
                    state = 4;
                    StGo::Back
                }
            }
            6 => {
                saved = p.wrapping_sub(2);
                if is_19(c) {
                    state = 9;
                    StGo::Back
                } else if c == b'+' {
                    state = 7;
                    StGo::Read
                } else if c == b'-' {
                    exponent_sign = -1;
                    state = 7;
                    StGo::Read
                } else if c == b'0' {
                    state = 8;
                    StGo::Read
                } else {
                    StGo::EndSaved
                }
            }
            7 => {
                if is_19(c) {
                    state = 9;
                    StGo::Back
                } else if c == b'0' {
                    state = 8;
                    StGo::Read
                } else {
                    StGo::EndSaved
                }
            }
            8 => {
                exponent_seen = 1;
                while c == b'0' {
                    c = e.mem.u8(p);
                    p = p.wrapping_add(1);
                }
                if is_19(c) {
                    state = 9;
                    StGo::Back
                } else {
                    StGo::EndBack
                }
            }
            9 => {
                exponent_seen = 1;
                let mut value: i32 = 0;
                while is_digit(c) {
                    value = value
                        .wrapping_mul(10)
                        .wrapping_add(i32::from(c as i8) - 0x30);
                    if value > 0x1450 {
                        value = 0x1451;
                        break;
                    }
                    c = e.mem.u8(p);
                    p = p.wrapping_add(1);
                }
                exponent = value;
                while is_digit(c) {
                    c = e.mem.u8(p);
                    p = p.wrapping_add(1);
                }
                StGo::EndBack
            }
            0xb => {
                if implicit_e == 0 {
                    // 00edb440: state 10, which ends at once.
                    p = p.wrapping_sub(1);
                    StGo::End
                } else {
                    saved = p.wrapping_sub(1);
                    if c == b'+' {
                        state = 7;
                        StGo::Read
                    } else if c == b'-' {
                        exponent_sign = -1;
                        state = 7;
                        StGo::Read
                    } else {
                        p = saved;
                        StGo::End
                    }
                }
            }
            // 00edb444 (the jump table's entry for 10, and anything above 11): state 10
            // ends; any other state reads on (none is ever set).
            10 => StGo::End,
            _ => StGo::Read,
        };
        match go {
            StGo::Read => {}
            StGo::Back => p = p.wrapping_sub(1),
            StGo::EndBack => {
                p = p.wrapping_sub(1);
                break;
            }
            StGo::End => break,
            StGo::EndSaved => {
                p = saved;
                break;
            }
        }
    }
    // 00edb40d
    e.mem.set_u32(end, p);
    let value = ebp - ST_VALUE;
    let (word0, middle_low, middle_high, top) = 'value: {
        if found == 0 {
            flags = 4;
            break 'value (0u32, 0u32, 0u32, 0u32);
        }
        if count > 0x18 {
            let last = ebp - 9;
            if (e.mem.u8(last) as i8) >= 5 {
                let v = e.mem.u8(last).wrapping_add(1);
                e.mem.set_u8(last, v);
            }
            digit_at = digit_at.wrapping_sub(1);
            adjust = adjust.wrapping_add(1);
            count = 0x18;
        }
        if count == 0 {
            break 'value (0, 0, 0, 0);
        }
        loop {
            digit_at = digit_at.wrapping_sub(1);
            if e.mem.u8(digit_at) != 0 {
                break;
            }
            count = count.wrapping_sub(1);
            adjust = adjust.wrapping_add(1);
        }
        mtold12(e, ebp - ST_DIGITS, count, value);
        let mut power = if exponent_sign < 0 {
            exponent.wrapping_neg()
        } else {
            exponent
        };
        power = power.wrapping_add(adjust);
        if exponent_seen == 0 {
            power = power.wrapping_add(scale);
        }
        if point_seen == 0 {
            power = power.wrapping_sub(decpt);
        }
        if power > 0x1450 {
            flags = 2;
            break 'value (0, 0, 0x8000_0000, 0x7fff);
        }
        if power < -0x1450 {
            flags = 1;
            break 'value (0, 0, 0, 0);
        }
        let mut table = POW10_POS - 0x60;
        if power != 0 {
            if power < 0 {
                power = power.wrapping_neg();
                table = POW10_NEG - 0x60;
            }
            if mult12 == 0 {
                e.mem.set_u16(value, 0);
            }
            while power != 0 {
                let index = (power & 7) as u32;
                power >>= 3;
                table = table.wrapping_add(0x54);
                if index != 0 {
                    multiply_power(e, ebp, table.wrapping_add(index * 12));
                }
            }
        }
        // 00edb7e3
        (
            u32::from(e.mem.u16(value)),
            e.mem.u32(value + 2),
            e.mem.u32(value + 6),
            e.mem.u32(value + 8) >> 16,
        )
    };
    // 00edb824
    e.mem.set_u16(result, word0 as u16);
    e.mem.set_u16(result + 0xa, (top | sign) as u16);
    e.mem.set_u32(result + 2, middle_low);
    e.mem.set_u32(result + 6, middle_high);
    flags
}

/// Where `__strgtold12_l`'s state machine goes after a character.
#[derive(Clone, Copy)]
enum StGo {
    /// 00edb209: read the next character.
    Read,
    /// 00edb228: step back onto the character and read it again in the new state.
    Back,
    /// 00edb3e2: the number ends at the character just read.
    EndBack,
    /// 00edb40d: the number ends at the current pointer.
    End,
    /// 00edb2f1: the number ends at the saved pointer (the start, or the exponent letter).
    EndSaved,
}

/// 00edb29e: `D`, `E`, `d`, `e` start the exponent (state 6); anything else ends the number
/// at the character just read.
fn exponent_letter(c: u8, state: &mut u32) -> StGo {
    let c = c as i8;
    if c <= 0x43 || (c > 0x45 && (c <= 0x63 || c > 0x65)) {
        return StGo::EndBack;
    }
    *state = 6;
    StGo::Read
}

/// 00edb331: a sign starts an implicit exponent (state 11, stepping back onto it);
/// otherwise [`exponent_letter`].
fn sign_or_letter(c: u8, p: &mut u32, state: &mut u32) -> StGo {
    if c == b'+' || c == b'-' {
        *p = p.wrapping_sub(1);
        *state = 0xb;
        return StGo::Read;
    }
    exponent_letter(c, state)
}

/// 00edb524..00edb7d9: multiplies the `_LDBL12` at EBP-0x3c by the power of ten at `entry`
/// (`__ld12mul`, inlined), in the frame as the exe does.
fn multiply_power(e: &mut Engine, ebp: u32, entry: u32) {
    let value = ebp - ST_VALUE;
    let product = ebp - ST_PRODUCT;
    let mut factor = entry;
    if e.mem.u16(factor) >= 0x8000 {
        let copy = ebp - ST_POWER;
        let bytes = e.mem.bytes(factor, 12);
        e.mem.write(copy, &bytes);
        let middle = e.mem.u32(copy + 2).wrapping_sub(1);
        e.mem.set_u32(copy + 2, middle);
        factor = copy;
    }
    let factor_exponent = u32::from(e.mem.u16(factor + 0xa));
    let mut sticky: u32 = 0; // EBP-0x50
    e.mem.set_u32(product, 0);
    e.mem.set_u32(product + 4, 0);
    e.mem.set_u32(product + 8, 0);
    let value_exponent = e.mem.u32(value + 0xa);
    let result_sign = (factor_exponent ^ value_exponent) & 0x8000; // EBP-0x70
    let a = value_exponent & 0x7fff;
    let b = factor_exponent & 0x7fff;
    let mut sum = (b + a) & 0xffff; // EDX (MOVZX DX)
    if a as u16 >= 0x7fff || b as u16 >= 0x7fff || sum as u16 > 0xbffd {
        // 00edb7bb
        e.mem.set_u32(value + 4, 0);
        e.mem.set_u32(value, 0);
        let top = if result_sign == 0 {
            0x7fff_8000
        } else {
            0xffff_8000
        };
        e.mem.set_u32(value + 8, top);
        return;
    }
    if sum as u16 <= 0x3fbf {
        e.mem.set_u32(value + 4, 0);
        e.mem.set_u32(value, 0);
        e.mem.set_u32(value + 8, 0);
        return;
    }
    if a == 0 {
        sum += 1;
        if e.mem.u32(value + 8) & 0x7fff_ffff == 0
            && e.mem.u32(value + 4) == 0
            && e.mem.u32(value) == 0
        {
            e.mem.set_u16(value + 0xa, 0);
            return;
        }
    }
    if b == 0 {
        sum += 1;
        if e.mem.u32(factor + 8) & 0x7fff_ffff == 0
            && e.mem.u32(factor + 4) == 0
            && e.mem.u32(factor) == 0
        {
            e.mem.set_u32(value + 8, 0);
            e.mem.set_u32(value + 4, 0);
            e.mem.set_u32(value, 0);
            return;
        }
    }
    // 00edb614: the partial products of the five words, top half kept.
    let mut i: u32 = 0;
    let mut n: i32 = 5;
    let mut at = product + 4; // EDI
    while n > 0 {
        let mut a_at = value + 2 * i;
        let mut b_at = factor + 8;
        let mut left = n;
        while left > 0 {
            let term = u32::from(e.mem.u16(b_at)) * u32::from(e.mem.u16(a_at));
            let old = e.mem.u32(at - 4);
            let new = old.wrapping_add(term);
            let carry = new < old || new < term;
            e.mem.set_u32(at - 4, new);
            if carry {
                let w = e.mem.u16(at).wrapping_add(1);
                e.mem.set_u16(at, w);
            }
            a_at += 2;
            b_at -= 2;
            left -= 1;
        }
        at += 2;
        i += 1;
        n -= 1;
    }
    let mut dx = (sum.wrapping_add(0xc002)) as u16;
    let shift_left = |e: &mut Engine| {
        let low = e.mem.u32(product);
        let middle = e.mem.u32(product + 4);
        let high = e.mem.u32(product + 8);
        e.mem.set_u32(product, low << 1);
        e.mem.set_u32(product + 4, (middle << 1) | (low >> 31));
        e.mem.set_u32(product + 8, (high << 1) | (middle >> 31));
    };
    let mut normal = false;
    if (dx as i16) > 0 {
        loop {
            if (e.mem.u32(product + 8) as i32) < 0 {
                break;
            }
            shift_left(e);
            dx = dx.wrapping_sub(1);
            if (dx as i16) <= 0 {
                break;
            }
        }
        normal = (dx as i16) > 0;
    }
    if !normal {
        // 00edb6d2
        dx = dx.wrapping_sub(1);
        if (dx as i16) < 0 {
            let mut n = u32::from(dx.wrapping_neg());
            dx = dx.wrapping_add(n as u16);
            loop {
                if e.mem.u8(product) & 1 != 0 {
                    sticky = sticky.wrapping_add(1);
                }
                let high = e.mem.u32(product + 8);
                let middle = e.mem.u32(product + 4);
                let low = e.mem.u32(product);
                e.mem.set_u32(product + 8, high >> 1);
                e.mem.set_u32(product + 4, (middle >> 1) | (high << 31));
                e.mem.set_u32(product, (low >> 1) | (middle << 31));
                n -= 1;
                if n == 0 {
                    break;
                }
            }
            if sticky != 0 {
                let w = e.mem.u16(product) | 1;
                e.mem.set_u16(product, w);
            }
        }
    }
    // 00edb71f: round to the 80 bits above the low word.
    if e.mem.u16(product) > 0x8000 || e.mem.u32(product) & 0x1_ffff == 0x1_8000 {
        if e.mem.u32(product + 2) == u32::MAX {
            e.mem.set_u32(product + 2, 0);
            if e.mem.u32(product + 6) == u32::MAX {
                e.mem.set_u32(product + 6, 0);
                if e.mem.u16(product + 0xa) == 0xffff {
                    e.mem.set_u16(product + 0xa, 0x8000);
                    dx = dx.wrapping_add(1);
                } else {
                    let w = e.mem.u16(product + 0xa).wrapping_add(1);
                    e.mem.set_u16(product + 0xa, w);
                }
            } else {
                let w = e.mem.u32(product + 6).wrapping_add(1);
                e.mem.set_u32(product + 6, w);
            }
        } else {
            let w = e.mem.u32(product + 2).wrapping_add(1);
            e.mem.set_u32(product + 2, w);
        }
    }
    // 00edb771
    if dx >= 0x7fff {
        e.mem.set_u32(value + 4, 0);
        e.mem.set_u32(value, 0);
        let top = if result_sign as u16 == 0 {
            0x7fff_8000
        } else {
            0xffff_8000
        };
        e.mem.set_u32(value + 8, top);
    } else {
        let w = e.mem.u16(product + 2);
        e.mem.set_u16(value, w);
        let w = e.mem.u32(product + 4);
        e.mem.set_u32(value + 2, w);
        let w = e.mem.u32(product + 8);
        e.mem.set_u32(value + 6, w);
        e.mem.set_u16(value + 0xa, dx | result_sign as u16);
    }
}

// Translated from 00ee44e1 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mtold12` (LIBCMT/mantold.c; cdecl: digits, count, result): the decimal digits (bytes
/// 0..9) as a 96-bit integer (`x*4 + x` doubled, plus the digit, for each), then normalised
/// (16 bits at a time while the top word is zero, then one bit at a time until bit 15 of the
/// top word is set), the exponent starting at 0x404E. The count is unsigned; a zero result
/// would never finish normalising (no caller passes one).
// Translation incomplete: 00ee44e1 - the /GS stack cookie check (00ec408c) is not translated.
pub fn mtold12(e: &mut Engine, digits: u32, count: u32, result: u32) {
    let mut exponent: u32 = 0x404e;
    let (mut m0, mut m1, mut m2) = (0u32, 0u32, 0u32);
    e.mem.set_u32(result, 0);
    e.mem.set_u32(result + 4, 0);
    e.mem.set_u32(result + 8, 0);
    let mut at = digits;
    let mut left = count;
    while left != 0 {
        let (s0, s1, s2) = (m0, m1, m2);
        // Times 4.
        let a0 = m0 << 1;
        let a1 = (m1 << 1) | (m0 >> 31);
        let a2 = (m2 << 1) | (m1 >> 31);
        let b0 = a0 << 1;
        let b1 = (a1 << 1) | (a0 >> 31);
        let b2 = (a2 << 1) | (a1 >> 31);
        // Plus the saved value.
        let sum0 = b0.wrapping_add(s0);
        m2 = b2;
        m1 = b1;
        let carry = sum0 < b0 || sum0 < s0;
        m0 = sum0;
        if carry {
            let next = m1.wrapping_add(1);
            let carry2 = next < m1 || next < 1;
            m1 = next;
            if carry2 {
                m2 = m2.wrapping_add(1);
            }
        }
        let sum1 = m1.wrapping_add(s1);
        let carry = sum1 < m1 || sum1 < s1;
        m1 = sum1;
        if carry {
            m2 = m2.wrapping_add(1);
        }
        m2 = m2.wrapping_add(s2);
        // Doubled.
        let d0 = m0 << 1;
        let d1 = (m1 << 1) | (m0 >> 31);
        let d2 = (m2 << 1) | (m1 >> 31);
        m0 = d0;
        m1 = d1;
        m2 = d2;
        // Plus the digit (sign-extended).
        let digit = i32::from(e.mem.u8(at) as i8) as u32;
        let sum = m0.wrapping_add(digit);
        let carry = sum < m0 || sum < digit;
        m0 = sum;
        if carry {
            let next = m1.wrapping_add(1);
            let carry2 = next < m1 || next < 1;
            m1 = next;
            if carry2 {
                m2 = m2.wrapping_add(1);
            }
        }
        left -= 1;
        at = at.wrapping_add(1);
    }
    while m2 == 0 {
        m2 = m1 >> 16;
        m1 = (m1 << 16) | (m0 >> 16);
        m0 <<= 16;
        exponent = exponent.wrapping_add(0xfff0);
    }
    while m2 & 0x8000 == 0 {
        exponent = exponent.wrapping_add(0xffff);
        let n0 = m0 << 1;
        let n1 = (m1 << 1) | (m0 >> 31);
        let n2 = (m2 << 1) | (m1 >> 31);
        m0 = n0;
        m1 = n1;
        m2 = n2;
    }
    e.mem.set_u32(result, m0);
    e.mem.set_u32(result + 4, m1);
    e.mem.set_u32(result + 8, m2);
    e.mem.set_u16(result + 0xa, exponent as u16);
}

/// `CDQ; AND EDX,0x1f; ADD; SAR 5`: a signed bit count's word (rounded toward zero).
fn bit_word(n: i32) -> i32 {
    n.wrapping_add((n >> 31) & 0x1f) >> 5
}

/// `AND 0x8000001f` with the negative fix-up: a signed bit count's bit within its word.
fn bit_in_word(n: i32) -> i32 {
    let r = (n as u32 & 0x8000_001f) as i32;
    if r < 0 {
        (r.wrapping_sub(1) | -32).wrapping_add(1)
    } else {
        r
    }
}

/// `_ld12cvt`'s rounding (`_RoundMan`, inlined twice): when the bit after `precision` bits is
/// set and any bit below it is set, adds one at the last kept bit, carrying upward; then
/// clears everything below the bit after the precision (that bit itself is kept). An exact
/// half is not rounded. Returns the carry out of the top word (0 when not rounded or the
/// carry stopped).
fn ld12_round(man: &mut [u32; 3], precision: i32) -> u32 {
    let last = precision.wrapping_sub(1);
    let word = bit_word(precision);
    let shift = (31 - bit_in_word(precision)) as u32;
    let mut carry_out = 0;
    let w = word as usize;
    if man[w] & 1u32.wrapping_shl(shift) != 0 {
        let mut idx = word;
        let mut rest = man[w] & !u32::MAX.wrapping_shl(shift);
        loop {
            if rest != 0 {
                break;
            }
            idx += 1;
            if idx >= 3 {
                break;
            }
            rest = man[idx as usize];
        }
        if rest != 0 {
            let mut at = bit_word(last);
            let add = 1u32.wrapping_shl((31 - bit_in_word(last)) as u32);
            let old = man[at as usize];
            let mut v = old.wrapping_add(add);
            let mut carry = v < old || v < add;
            loop {
                man[at as usize] = v;
                at -= 1;
                if at < 0 {
                    carry_out = u32::from(carry);
                    break;
                }
                if !carry {
                    break;
                }
                let old = man[at as usize];
                v = old.wrapping_add(1);
                carry = v < old || v < 1;
            }
        }
    }
    man[w] &= u32::MAX.wrapping_shl(shift);
    for word in man.iter_mut().skip(w + 1) {
        *word = 0;
    }
    carry_out
}

/// `_ShrMan` (inlined): the three words shifted right by `n` bits (bits, then whole words).
fn ld12_shift_right(man: &mut [u32; 3], n: i32) {
    let words = bit_word(n);
    let bits = bit_in_word(n) as u32;
    let mask = !u32::MAX.wrapping_shl(bits);
    let mut carry = 0u32;
    for word in man.iter_mut() {
        let v = *word;
        let low = v & mask;
        *word = v.wrapping_shr(bits) | carry;
        carry = low.wrapping_shl(32 - bits);
    }
    let mut i: i32 = 2;
    while i >= 0 {
        man[i as usize] = if i < words {
            0
        } else {
            man[(i - words) as usize]
        };
        i -= 1;
    }
}

/// `_ld12cvt` (LIBCMT/strgtold.c, inlined into `_ld12tod` and `_ld12tof`, which differ only in
/// the `FpFormatDescriptor` at `format`): the `_LDBL12` at `value` rounded to the format's
/// precision and stored as a 64-bit (`double`) or 32-bit (`float`) value at `out`. A zero
/// exponent gives zero (status 2 when the mantissa was not zero); below the denormal range
/// zero, status 2; at or below the minimum exponent a denormal from the unrounded mantissa
/// shifted down, rounded, and shifted down one more bit than a normal number (status 2); at
/// or above the maximum exponent infinity (status 1); otherwise the rounded mantissa (status
/// 0). The sign is kept in every case.
fn ld12cvt(e: &mut Engine, value: u32, out: u32, format: u32) -> u32 {
    let max_exp = e.mem.u32(format) as i32;
    let min_exp = e.mem.u32(format + 4) as i32;
    let precision = e.mem.u32(format + 8) as i32;
    let exp_width = e.mem.u32(format + 0xc) as i32;
    let format_width = e.mem.u32(format + 0x10);
    let bias = e.mem.u32(format + 0x14) as i32;
    let top = u32::from(e.mem.u16(value + 0xa));
    let sign = top & 0x8000;
    let mut man = [
        e.mem.u32(value + 6),
        e.mem.u32(value + 2),
        u32::from(e.mem.u16(value)) << 16,
    ];
    let mut exponent = (top & 0x7fff) as i32 - 0x3fff;
    let status;
    if exponent == -0x3fff {
        exponent = 0;
        if man.iter().any(|w| *w != 0) {
            man = [0; 3];
            status = 2;
        } else {
            status = 0;
        }
    } else {
        let saved = man;
        let unrounded = exponent;
        if ld12_round(&mut man, precision) != 0 {
            exponent += 1;
        }
        if exponent < min_exp.wrapping_sub(precision) {
            man = [0; 3];
            exponent = 0;
            status = 2;
        } else if exponent <= min_exp {
            man = saved;
            ld12_shift_right(&mut man, min_exp.wrapping_sub(unrounded));
            ld12_round(&mut man, precision);
            ld12_shift_right(&mut man, exp_width + 1);
            exponent = 0;
            status = 2;
        } else if exponent >= max_exp {
            man = [0x8000_0000, 0, 0];
            ld12_shift_right(&mut man, exp_width);
            exponent = max_exp.wrapping_add(bias);
            status = 1;
        } else {
            man[0] &= 0x7fff_ffff;
            exponent = exponent.wrapping_add(bias);
            ld12_shift_right(&mut man, exp_width);
            status = 0;
        }
    }
    // 00ee3f5a
    let high = (exponent as u32).wrapping_shl((31 - exp_width) as u32)
        | if sign != 0 { 0x8000_0000 } else { 0 }
        | man[0];
    if format_width == 0x40 {
        e.mem.set_u32(out + 4, high);
        e.mem.set_u32(out, man[1]);
    } else if format_width == 0x20 {
        e.mem.set_u32(out, high);
    }
    status
}

// Translated from 00ee3a59 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_ld12tod` (LIBCMT/strgtold.c; cdecl: `_LDBL12`, double out): [`ld12cvt`] with the double
/// format (`011c2c80`: exponents 1024 and -1023, 53 bits, 11-bit exponent, 64 bits, bias
/// 1023). Returns `INTRNCVT_OK` 0, `INTRNCVT_OVERFLOW` 1 or `INTRNCVT_UNDERFLOW` 2.
pub fn ld12tod(e: &mut Engine, value: u32, out: u32) -> u32 {
    ld12cvt(e, value, out, DOUBLE_FORMAT)
}

// Translated from 00ee3f9d (decompiled, FalloutNV.exe 1.4.0.525)
/// `_ld12tof` (LIBCMT/strgtold.c; cdecl: `_LDBL12`, float out): [`ld12cvt`] with the float
/// format (`011c2c98`: exponents 128 and -127, 24 bits, 8-bit exponent, 32 bits, bias 127).
pub fn ld12tof(e: &mut Engine, value: u32, out: u32) -> u32 {
    ld12cvt(e, value, out, FLOAT_FORMAT)
}

/// This file's registrations (added to [`super::funcs`]).
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00ec_a4a6, sscanf_entry as AbiFn),
        entry!(0x00ec_a43c, vscan_fn(u32, u32, u32, u32, u32) -> i32),
        entry!(0x00ed_a11e, input_l(u32, u32, u32, u32) -> i32),
        entry!(0x00ed_a0cb, inc(u32) -> u32),
        entry!(0x00ed_a0e1, un_inc(u32, u32)),
        entry!(0x00ed_a0f4, whiteout(u32, u32) -> u32),
        entry!(0x00ed_a0ab, hextodec(u32) -> u32),
        entry!(0x00ed_a054, check_float_string(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ed_0dbe, recalloc_crt(u32, u32, u32) -> u32),
        entry!(0x00ed_c81f, filbuf(Ptr) -> i32),
        entry!(0x00ec_d4ce, ungetc_nolock(u32, Ptr) -> i32),
        entry!(0x00ec_880f, isdigit_l(u32, u32) -> u32),
        entry!(0x00ec_8860, isdigit(u32) -> u32),
        entry!(0x00ec_8914, isspace_l(u32, u32) -> u32),
        entry!(0x00ec_8965, isspace(u32) -> u32),
        entry!(0x00ec_888e, isxdigit_l(u32, u32) -> u32),
        entry!(0x00ec_88e4, isxdigit(u32) -> u32),
        entry!(0x00ec_868b, isalpha_l(u32, u32) -> u32),
        entry!(0x00ec_8711, isupper_l(u32, u32) -> u32),
        entry!(0x00ec_8790, islower_l(u32, u32) -> u32),
        entry!(0x00ec_8993, isalnum_l(u32, u32) -> u32),
        entry!(0x00ed_41d4, isleadbyte(u32) -> u32),
        entry!(0x00ee_3928, mbtowc_l(u32, u32, u32, u32) -> i32),
        entry!(0x00ec_c300, allmul(u64, u64) -> u64),
        entry!(0x00ec_5eed, cfltcvt_init()),
        entry!(0x00ed_248e, initp_misc_cfltcvt_tab()),
        entry!(0x00ed_1aa7, fassign_l(u32, u32, u32, u32)),
        entry!(0x00ee_177b, atodbl_l(u32, u32, u32) -> u32),
        entry!(0x00ee_1823, atoflt_l(u32, u32, u32) -> u32),
        entry!(0x00ed_b182, strgtold12_l(u32, u32, u32, u32, i32, i32, u32, u32) -> u32),
        entry!(0x00ee_44e1, mtold12(u32, u32, u32)),
        entry!(0x00ee_3a59, ld12tod(u32, u32) -> u32),
        entry!(0x00ee_3f9d, ld12tof(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The C locale's class bits for `c` (upper 1, lower 2, digit 4, space 8, hex 0x80).
    fn c_type(c: u8) -> u16 {
        let mut bits = 0;
        if c.is_ascii_uppercase() {
            bits |= 1;
        }
        if c.is_ascii_lowercase() {
            bits |= 2;
        }
        if c.is_ascii_digit() {
            bits |= 4;
        }
        if c == b' ' || (9..=13).contains(&c) {
            bits |= 8;
        }
        if c.is_ascii_hexdigit() {
            bits |= 0x80;
        }
        bits
    }

    /// A number as little-endian 32-bit limbs.
    type Big = Vec<u32>;

    fn big_mul_small(v: &mut Big, m: u32) {
        let mut carry = 0u64;
        for limb in v.iter_mut() {
            let x = u64::from(*limb) * u64::from(m) + carry;
            *limb = x as u32;
            carry = x >> 32;
        }
        if carry != 0 {
            v.push(carry as u32);
        }
    }

    fn big_bits(v: &Big) -> u32 {
        let top = v.iter().rposition(|l| *l != 0).unwrap();
        top as u32 * 32 + (32 - v[top].leading_zeros())
    }

    fn big_bit(v: &Big, i: u32) -> bool {
        let limb = (i / 32) as usize;
        limb < v.len() && v[limb] >> (i % 32) & 1 != 0
    }

    fn big_ge(a: &Big, b: &Big) -> bool {
        let n = a.len().max(b.len());
        for i in (0..n).rev() {
            let x = a.get(i).copied().unwrap_or(0);
            let y = b.get(i).copied().unwrap_or(0);
            if x != y {
                return x > y;
            }
        }
        true
    }

    fn big_sub(a: &mut Big, b: &Big) {
        let mut borrow = 0i64;
        for (i, limb) in a.iter_mut().enumerate() {
            let x = i64::from(*limb) - i64::from(b.get(i).copied().unwrap_or(0)) - borrow;
            borrow = i64::from(x < 0);
            *limb = x.rem_euclid(1 << 32) as u32;
        }
    }

    fn big_pow10(n: u32) -> Big {
        let mut v = vec![1];
        for _ in 0..n {
            big_mul_small(&mut v, 10);
        }
        v
    }

    /// The `_LDBL12` the CRT's power tables hold for 10^n or 10^-n, computed: the top 64
    /// significand bits rounded to nearest, then the next 16 bits (after the unrounded 64)
    /// rounded to nearest; no entry is a tie. Checked against `_pow10pos`/`_pow10neg`.
    fn pow10_entry(n: u32, negative: bool) -> [u8; 12] {
        let mut bits = [false; 81];
        let exponent: i32;
        if negative {
            // The binary expansion of 1/10^n by long division.
            let d = big_pow10(n);
            let length = big_bits(&d);
            let mut r: Big = vec![0; (length / 32 + 2) as usize];
            r[((length - 1) / 32) as usize] = 1 << ((length - 1) % 32);
            for bit in bits.iter_mut() {
                big_mul_small(&mut r, 2);
                if big_ge(&r, &d) {
                    *bit = true;
                    big_sub(&mut r, &d);
                }
            }
            assert!(bits[0]);
            exponent = -(length as i32);
        } else {
            let v = big_pow10(n);
            let length = big_bits(&v);
            for (k, bit) in bits.iter_mut().enumerate() {
                let k = k as u32;
                *bit = k < length && big_bit(&v, length - 1 - k);
            }
            exponent = length as i32 - 1;
        }
        let take = |from: usize, count: usize| {
            bits[from..from + count]
                .iter()
                .fold(0u64, |acc, b| acc << 1 | u64::from(*b))
        };
        let high = take(0, 64) + u64::from(bits[64]);
        let low = take(64, 16) + u64::from(bits[80]);
        assert!(high >> 63 == 1 && low <= 0xffff);
        let mut entry = [0u8; 12];
        entry[0..2].copy_from_slice(&(low as u16).to_le_bytes());
        entry[2..10].copy_from_slice(&high.to_le_bytes());
        entry[10..12].copy_from_slice(&((0x3fff + exponent) as u16).to_le_bytes());
        entry
    }

    /// The table's powers: 10^1..10^7, then 8, 64, 512 and 4096 times 1..7 (the last once).
    fn table_power(index: u32) -> u32 {
        (index % 7 + 1) * 8u32.pow(index / 7)
    }

    /// The double and float `FpFormatDescriptor`s (IEEE 754 binary64 and binary32: max and
    /// min exponent, precision, exponent width, format width, bias).
    const DOUBLE_DESCRIPTOR: [u32; 6] = [0x400, 0xffff_fc01, 53, 11, 64, 0x3ff];
    const FLOAT_DESCRIPTOR: [u32; 6] = [0x80, 0xffff_ff81, 24, 8, 32, 0x7f];

    /// An engine with the C locale: `_pctype`, a locale info block (`mb_cur_max` 1, the ctype
    /// table, a `lconv` whose decimal point is "."), a `_LocaleUpdate` double, `errno` in its
    /// static slot, no lead bytes, `_fassign_l` in its `_cfltcvt_tab` slot (decoding is the
    /// identity), the format descriptors and the power tables. Returns the engine and the
    /// locale info block.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011c_1000, 0x2000);
        let table = e.mem.alloc(0x200);
        for c in 0..=255u8 {
            e.mem.set_u16(table + u32::from(c) * 2, c_type(c));
        }
        e.mem.set_u32(PCTYPE, table);
        let point = e.mem.alloc(4);
        e.mem.write(point, b".\0");
        let lconv = e.mem.alloc(0x30);
        e.mem.set_u32(lconv, point);
        let info = e.mem.alloc(0x100);
        e.mem.set_u32(info + 0xac, 1);
        e.mem.set_u32(info + 0xc8, table);
        e.mem.set_u32(info + 0xbc, lconv);
        let thread = e.mem.alloc(0x80);
        e.mem.set_u32(thread + 0x70, 3);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u32(a[0] + 4, 0);
            e.mem.set_u32(a[0] + 8, thread);
            e.mem.set_u8(a[0] + 0xc, 0);
            a[0].into_ret()
        });
        e.register(GETPTD_NOEXIT, |_, _| 0u32.into_ret());
        e.register(DECODE_POINTER, |_, a| a[0].into_ret());
        e.register(ISLEADBYTE_L, |_, _| 0u32.into_ret());
        e.mem.set_u32(CFLTCVT_FASSIGN, 0x00ed_1aa7);
        for (i, word) in DOUBLE_DESCRIPTOR.iter().enumerate() {
            e.mem.set_u32(DOUBLE_FORMAT + 4 * i as u32, *word);
        }
        for (i, word) in FLOAT_DESCRIPTOR.iter().enumerate() {
            e.mem.set_u32(FLOAT_FORMAT + 4 * i as u32, *word);
        }
        for index in 0..29 {
            let n = table_power(index);
            e.mem.write(POW10_POS + 12 * index, &pow10_entry(n, false));
            e.mem.write(POW10_NEG + 12 * index, &pow10_entry(n, true));
        }
        (e, info)
    }

    /// `sscanf(text, format, args...)`.
    fn scan(e: &mut Engine, text: &[u8], format: &[u8], args: &[u32]) -> i32 {
        let s = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(s, text);
        e.mem.set_u8(s + text.len() as u32, 0);
        let f = e.mem.alloc(format.len() as u32 + 1);
        e.mem.write(f, format);
        e.mem.set_u8(f + format.len() as u32, 0);
        let mut words = vec![s, f];
        words.extend_from_slice(args);
        e.call(0x00ec_a4a6, &words).u32() as i32
    }

    /// `sscanf(text, "%f", &x)` (or `%lf`): the result and the stored bits.
    fn scan_float(e: &mut Engine, text: &[u8]) -> (i32, u32) {
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x5555_5555);
        let n = scan(e, text, b"%f", &[out]);
        (n, e.mem.u32(out))
    }

    fn scan_double(e: &mut Engine, text: &[u8]) -> (i32, u64) {
        let out = e.mem.alloc(8);
        let n = scan(e, text, b"%lf", &[out]);
        (
            n,
            u64::from(e.mem.u32(out)) | u64::from(e.mem.u32(out + 4)) << 32,
        )
    }

    #[test]
    fn sscanf_reads_floats_bit_exact() {
        let (mut e, _) = engine();
        assert_eq!(scan_float(&mut e, b"0.1"), (1, 0x3dcc_cccd));
        assert_eq!(scan_float(&mut e, b"1.5"), (1, 0x3fc0_0000));
        assert_eq!(scan_float(&mut e, b"  -2.25"), (1, 0xc010_0000));
        assert_eq!(scan_float(&mut e, b"3.4028235e38"), (1, 0x7f7f_ffff));
        // FLT_MIN and a denormal (rounded at the denormal's half bit, then cut).
        assert_eq!(scan_float(&mut e, b"1.17549435e-38"), (1, 0x0080_0000));
        assert_eq!(scan_float(&mut e, b"1e-40"), (1, 0x0001_16c2));
        // Beyond the range: infinity and zero.
        assert_eq!(scan_float(&mut e, b"1e39"), (1, 0x7f80_0000));
        assert_eq!(scan_float(&mut e, b"-1e39"), (1, 0xff80_0000));
        assert_eq!(scan_float(&mut e, b"1e-50"), (1, 0));
        // The sign of zero is kept.
        assert_eq!(scan_float(&mut e, b"-0"), (1, 0x8000_0000));
        assert_eq!(scan_float(&mut e, b"0.0"), (1, 0));
        // An exact half is not rounded: 2^24 + 3 gives 2^24 + 2, not 2^24 + 4.
        assert_eq!(scan_float(&mut e, b"16777219"), (1, 0x4b80_0001));
        assert_eq!(scan_float(&mut e, b"16777217"), (1, 0x4b80_0000));
        // The scanner takes no "nan" or "inf": nothing is assigned, no EOF.
        assert_eq!(scan_float(&mut e, b"nan"), (0, 0x5555_5555));
        assert_eq!(scan_float(&mut e, b"inf"), (0, 0x5555_5555));
        assert_eq!(scan_float(&mut e, b""), (-1, 0x5555_5555));
        // Exponent forms and text after the number.
        assert_eq!(scan_float(&mut e, b"25E-1x"), (1, 0x4020_0000));
        assert_eq!(scan_float(&mut e, b".5"), (1, 0x3f00_0000));
        assert_eq!(scan_float(&mut e, b"5.e1"), (1, 0x4248_0000));
    }

    #[test]
    fn sscanf_reads_doubles_bit_exact() {
        let (mut e, _) = engine();
        assert_eq!(scan_double(&mut e, b"0.1"), (1, 0x3fb9_9999_9999_999a));
        assert_eq!(scan_double(&mut e, b"1"), (1, 0x3ff0_0000_0000_0000));
        assert_eq!(scan_double(&mut e, b"-0"), (1, 0x8000_0000_0000_0000));
        assert_eq!(scan_double(&mut e, b"1e308"), (1, 0x7fe1_ccf3_85eb_c8a0));
        assert_eq!(scan_double(&mut e, b"1e309"), (1, 0x7ff0_0000_0000_0000));
        assert_eq!(
            scan_double(&mut e, b"3.4028235e38"),
            (1, 0x47ef_ffff_e54d_aff8)
        );
    }

    #[test]
    fn sscanf_reads_unsigned_triples_and_stops_on_partial_input() {
        let (mut e, _) = engine();
        let out = e.mem.alloc(12);
        let reset = |e: &mut Engine| {
            for i in 0..3 {
                e.mem.set_u32(out + 4 * i, 0xaaaa_aaaa);
            }
        };
        let read = |e: &Engine| [e.mem.u32(out), e.mem.u32(out + 4), e.mem.u32(out + 8)];
        let args = [out, out + 4, out + 8];
        reset(&mut e);
        assert_eq!(scan(&mut e, b"255,128,64", b"%u,%u,%u", &args), 3);
        assert_eq!(read(&e), [255, 128, 64]);
        reset(&mut e);
        assert_eq!(scan(&mut e, b"640,480", b"%u,%u,%u", &args), 2);
        assert_eq!(read(&e), [640, 480, 0xaaaa_aaaa]);
        reset(&mut e);
        assert_eq!(scan(&mut e, b"7,x,9", b"%u,%u,%u", &args), 1);
        assert_eq!(read(&e), [7, 0xaaaa_aaaa, 0xaaaa_aaaa]);
        reset(&mut e);
        assert_eq!(scan(&mut e, b"", b"%u,%u,%u", &args), -1);
        assert_eq!(scan(&mut e, b"5;6", b"%u,%u,%u", &args), 1);
        assert_eq!(scan(&mut e, b" 1 , 2,3", b"%u , %u,%u", &args), 3);
        assert_eq!(read(&e), [1, 2, 3]);
        // Four values, as INISettingCollection::ReadSetting asks.
        let four = e.mem.alloc(16);
        let n = scan(
            &mut e,
            b"1,2,3,4",
            b"%u,%u,%u,%u",
            &[four, four + 4, four + 8, four + 12],
        );
        assert_eq!(n, 4);
        assert_eq!(e.mem.u32(four + 12), 4);
    }

    #[test]
    fn sscanf_integer_conversions() {
        let (mut e, _) = engine();
        let out = e.mem.alloc(8);
        let one = |e: &mut Engine, text: &[u8], format: &[u8]| {
            e.mem.set_u32(out, 0xaaaa_aaaa);
            e.mem.set_u32(out + 4, 0xaaaa_aaaa);
            let n = scan(e, text, format, &[out]);
            (n, e.mem.u32(out), e.mem.u32(out + 4))
        };
        assert_eq!(
            one(&mut e, b"-42", b"%d"),
            (1, (-42i32) as u32, 0xaaaa_aaaa)
        );
        assert_eq!(one(&mut e, b"+17", b"%d"), (1, 17, 0xaaaa_aaaa));
        assert_eq!(one(&mut e, b"0x1F", b"%x"), (1, 0x1f, 0xaaaa_aaaa));
        assert_eq!(one(&mut e, b"ff", b"%x"), (1, 0xff, 0xaaaa_aaaa));
        assert_eq!(one(&mut e, b"17", b"%o"), (1, 0o17, 0xaaaa_aaaa));
        assert_eq!(one(&mut e, b"0x10", b"%i"), (1, 16, 0xaaaa_aaaa));
        assert_eq!(one(&mut e, b"010", b"%i"), (1, 8, 0xaaaa_aaaa));
        assert_eq!(one(&mut e, b"12345", b"%3d"), (1, 123, 0xaaaa_aaaa));
        // `h` stores 16 bits.
        assert_eq!(one(&mut e, b"70000", b"%hd"), (1, 0xaaaa_1170, 0xaaaa_aaaa));
        // `ll` and `I64` store 64 bits (multiplied through _allmul).
        let v = (-12_345_678_901i64) as u64;
        assert_eq!(
            one(&mut e, b"-12345678901", b"%lld"),
            (1, v as u32, (v >> 32) as u32)
        );
        assert_eq!(
            one(&mut e, b"123456789abc", b"%I64x"),
            (1, 0x5678_9abc, 0x1234)
        );
        // Suppressed fields are read but not assigned or counted.
        assert_eq!(one(&mut e, b"1 2", b"%*d %d"), (1, 2, 0xaaaa_aaaa));
        // No digits: nothing assigned.
        assert_eq!(one(&mut e, b"-x", b"%d"), (0, 0xaaaa_aaaa, 0xaaaa_aaaa));
        // %n counts the characters read.
        let count = e.mem.alloc(4);
        assert_eq!(scan(&mut e, b" 123", b"%d%n", &[out, count]), 1);
        assert_eq!(e.mem.u32(count), 4);
    }

    #[test]
    fn sscanf_strings_characters_and_scan_sets() {
        let (mut e, _) = engine();
        let out = e.mem.alloc(16);
        let one = |e: &mut Engine, text: &[u8], format: &[u8]| {
            e.mem.write(out, &[0x55; 16]);
            let n = scan(e, text, format, &[out]);
            (n, e.mem.bytes(out, 6))
        };
        assert_eq!(
            one(&mut e, b"  abc def", b"%s"),
            (1, b"abc\0\x55\x55".to_vec())
        );
        assert_eq!(
            one(&mut e, b"abcdef", b"%3c"),
            (1, b"abc\x55\x55\x55".to_vec())
        );
        assert_eq!(
            one(&mut e, b" x", b"%c"),
            (1, b" \x55\x55\x55\x55\x55".to_vec())
        );
        assert_eq!(
            one(&mut e, b"abcd", b"%[a-c]"),
            (1, b"abc\0\x55\x55".to_vec())
        );
        assert_eq!(
            one(&mut e, b"xy,z", b"%[^,]"),
            (1, b"xy\0\x55\x55\x55".to_vec())
        );
        assert_eq!(
            one(&mut e, b"]]a", b"%[]]"),
            (1, b"]]\0\x55\x55\x55".to_vec())
        );
        assert_eq!(one(&mut e, b"abcdef", b"%4s"), (1, b"abcd\0\x55".to_vec()));
        // A literal that does not match ends the scan.
        assert_eq!(one(&mut e, b"a=1", b"b=%s"), (0, vec![0x55; 6]));
        assert_eq!(
            one(&mut e, b"%x", b"%%%s"),
            (1, b"x\0\x55\x55\x55\x55".to_vec())
        );
    }

    /// `__strgtold12_l` on a text: the flags, the `_LDBL12` bytes and the characters taken.
    fn strgtold12(e: &mut Engine, info: u32, text: &[u8], implicit_e: u32) -> (u32, [u8; 12], u32) {
        let s = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(s, text);
        e.mem.set_u8(s + text.len() as u32, 0);
        let locale = e.mem.alloc(0x10);
        e.mem.set_u32(locale, info);
        let result = e.mem.alloc(12);
        let end = e.mem.alloc(4);
        let flags = strgtold12_l(e, result, end, s, 1, 0, 0, implicit_e, locale);
        let mut bytes = [0u8; 12];
        bytes.copy_from_slice(&e.mem.bytes(result, 12));
        (flags, bytes, e.mem.u32(end) - s)
    }

    #[test]
    fn strgtold12_states_ends_and_flags() {
        let (mut e, info) = engine();
        // 1.0: mantissa 0x8000..., exponent 0x3fff.
        let one = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0xff, 0x3f];
        assert_eq!(strgtold12(&mut e, info, b"1", 0), (0, one, 1));
        assert_eq!(strgtold12(&mut e, info, b" \t+1.000x", 0), (0, one, 8));
        // The exponent letter without digits is not taken.
        assert_eq!(strgtold12(&mut e, info, b"1e+", 0).2, 1);
        // A sign followed by nothing: the end is the start (before the white space).
        assert_eq!(
            strgtold12(&mut e, info, b"  -x", 0),
            (4, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80], 0)
        );
        // An implicit exponent ("1+2" = 100) only when asked for.
        let hundred = strgtold12(&mut e, info, b"1+2", 1);
        assert_eq!(
            (hundred.0, hundred.1[9], hundred.1[10], hundred.2),
            (0, 0xc8, 0x05, 3)
        );
        assert_eq!(strgtold12(&mut e, info, b"1+2", 0).2, 1);
        // Out of range: infinity with flag 2, zero with flag 1.
        let big = strgtold12(&mut e, info, b"1e6000", 0);
        assert_eq!(
            (big.0, big.1[9], big.1[10], big.1[11]),
            (2, 0x80, 0xff, 0x7f)
        );
        assert_eq!(
            strgtold12(&mut e, info, b"-1e-6000", 0),
            (1, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80], 8)
        );
        // A null locale is an invalid parameter (a handler double set where
        // _set_invalid_parameter_handler keeps it).
        e.mem.set_u32(0x0126_fd9c, 0x0077_0000);
        e.register(0x0077_0000, |_, _| Ret::default());
        let result = e.mem.alloc(12);
        let end = e.mem.alloc(4);
        assert_eq!(strgtold12_l(&mut e, result, end, result, 0, 0, 0, 0, 0), 0);
        assert_eq!(e.mem.u32(ERRNO_FALLBACK), 0x16);
    }

    #[test]
    fn mtold12_and_ld12_conversions() {
        let (mut e, _) = engine();
        let digits = e.mem.alloc(4);
        e.mem.write(digits, &[1, 2, 3]);
        let value = e.mem.alloc(12);
        mtold12(&mut e, digits, 3, value);
        // 123 = 0x7b: normalised to 0xf6 << 56 with exponent 0x4005.
        assert_eq!(e.mem.u32(value + 6), 0xf600_0000);
        assert_eq!(e.mem.u16(value + 0xa), 0x4005);
        let out = e.mem.alloc(8);
        assert_eq!(ld12tof(&mut e, value, out), 0);
        assert_eq!(e.mem.u32(out), 123f32.to_bits());
        assert_eq!(ld12tod(&mut e, value, out), 0);
        assert_eq!(e.mem.u32(out + 4), 0x405e_c000);
        assert_eq!(e.mem.u32(out), 0);
        // A zero exponent with a mantissa underflows to zero (status 2), the sign kept.
        e.mem.write(value, &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80]);
        assert_eq!(ld12tof(&mut e, value, out), 2);
        assert_eq!(e.mem.u32(out), 0x8000_0000);
        // A rounding that carries out of the top word raises the exponent: just below 2.0.
        e.mem.write(
            value,
            &[
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x3f,
            ],
        );
        assert_eq!(ld12tof(&mut e, value, out), 0);
        assert_eq!(e.mem.u32(out), 0x4000_0000);
        // The 64-bit multiply's two paths.
        assert_eq!(
            allmul(&mut e, 0xffff_ffff, 0xffff_ffff),
            0xffff_fffe_0000_0001
        );
        assert_eq!(allmul(&mut e, 0x1_0000_0002, 3), 0x3_0000_0006);
        assert_eq!(allmul(&mut e, u64::MAX, 10), u64::MAX.wrapping_mul(10));
    }

    #[test]
    fn float_buffer_grows_onto_the_heap() {
        let (mut e, _) = engine();
        // 400 digits pass the 0x15E-byte stack buffer: one calloc, then freed.
        let mut text = vec![b'0'; 400];
        text.push(b'1');
        e.call_log = Some(vec![]);
        assert_eq!(scan_float(&mut e, &text), (1, 0x3f80_0000));
        let log = e.call_log.take().unwrap();
        let calls = |a: u32| log.iter().filter(|(x, _)| *x == a).count();
        assert_eq!(calls(SCAN_CALLOC_CRT), 1);
        assert_eq!(calls(SCAN_RECALLOC_CRT), 0);
        assert_eq!(calls(FREE_CRT), 1);
    }

    #[test]
    fn cfltcvt_table_is_filled_and_encoded() {
        let (mut e, _) = engine();
        e.register(SCAN_ENCODE_POINTER, |_, a| (a[0] ^ 0x1234_5678).into_ret());
        cfltcvt_init(&mut e);
        assert_eq!(e.mem.u32(CFLTCVT_FASSIGN), 0x00ed_1aa7);
        initp_misc_cfltcvt_tab(&mut e);
        assert_eq!(e.mem.u32(CFLTCVT_FASSIGN), 0x00ed_1aa7 ^ 0x1234_5678);
        assert_eq!(e.mem.u32(CFLTCVT_TAB), 0x00ed_246b ^ 0x1234_5678);
        assert_eq!(e.mem.u32(CFLTCVT_TAB + 0x24), 0x00ed_1996 ^ 0x1234_5678);
    }

    /// `__cinit`'s head as the game runs it, `_fpmath(1)` (with the loader answering the
    /// Pentium test's `IsProcessorFeaturePresent` lookup) then `__initp_misc_cfltcvt_tab`, takes
    /// the table slot from the exe's static `_fptrap` (`00ee1a8f`) to the encoded
    /// `_fassign_l`, and `sscanf("%f")` then converts through it.
    #[test]
    fn cinit_float_set_up_lets_sscanf_reach_fassign_l() {
        use crate::units::platform::loader;
        let (mut e, _) = engine();
        e.map(0x0127_0000, 0x1000);
        e.map(0x010f_2000, 0x1000);
        e.mem.set_u32(CFLTCVT_FASSIGN, 0x00ee_1a8f);
        e.mem.set_cstr(0x010f_2a7c, b"KERNEL32");
        e.mem.set_cstr(0x010f_2a60, b"IsProcessorFeaturePresent");
        e.register(
            0x00fd_f118,
            entry!(0, loader::get_module_handle_a(Ptr) -> u32).1,
        );
        e.register(
            0x00fd_f0ac,
            entry!(0, loader::get_proc_address(u32, u32) -> u32).1,
        );
        e.register(SCAN_ENCODE_POINTER, |_, a| (a[0] ^ 0x5a5a_0000).into_ret());
        e.register(DECODE_POINTER, |_, a| (a[0] ^ 0x5a5a_0000).into_ret());
        e.call(0x00ec_5f4d, &[1]);
        assert_eq!(e.mem.u32(0x0126_fd94), 0);
        assert_eq!(x87_control_word(&mut e), 0x027f);
        e.call(0x00ed_248e, &[]);
        assert_eq!(e.mem.u32(CFLTCVT_FASSIGN), 0x00ed_1aa7 ^ 0x5a5a_0000);
        assert_eq!(scan_float(&mut e, b"0.1"), (1, 0x3dcc_cccd));
    }

    #[test]
    fn the_alpha_upper_lower_alnum_classes_mask_the_table() {
        let (mut e, info) = engine();
        // The test table has no _ALPHA (0x100) bit: the masks still read their own bits.
        let table = e.mem.u32(info + 0xc8);
        e.mem.set_u16(table + u32::from(b'q') * 2, 0x102);
        assert_eq!(e.call(0x00ec_868b, &[u32::from(b'q'), 0]).u32(), 0x102);
        assert_eq!(e.call(0x00ec_868b, &[u32::from(b'7'), 0]).u32(), 0);
        assert_eq!(e.call(0x00ec_8711, &[u32::from(b'Q'), 0]).u32(), 1);
        assert_eq!(e.call(0x00ec_8711, &[u32::from(b'q'), 0]).u32(), 0);
        assert_eq!(e.call(0x00ec_8790, &[u32::from(b'q'), 0]).u32(), 2);
        assert_eq!(e.call(0x00ec_8993, &[u32::from(b'7'), 0]).u32(), 4);
        assert_eq!(e.call(0x00ec_8993, &[u32::from(b'q'), 0]).u32(), 0x102);
        assert_eq!(e.call(0x00ec_8993, &[u32::from(b'-'), 0]).u32(), 0);
        // A multibyte locale asks __isctype_l with the mask.
        e.mem.set_u32(info + 0xac, 2);
        e.register(0x00ed_41e7, |_, a| (a[1] << 8 | a[0]).into_ret());
        assert_eq!(e.call(0x00ec_8993, &[0x41, 0]).u32(), 0x107 << 8 | 0x41);
    }
}

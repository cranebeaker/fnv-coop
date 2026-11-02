//! The C runtime's low-level write (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe
//! 1.4.0.525), translated from the exe: `_write_nolock` (`00edc010`: binary, text mode with LF
//! written as CR-LF, the UTF-16 and UTF-8 modes and the console path) and the two functions it
//! calls that nothing else had: `_mbtowc` (`00ee3a3f`) and `__putwch_nolock` (`00ee46af`).
//! `_write` (`00edc743`), which locks the handle around `_write_nolock`, and the rest of the
//! handle layer (`ioinfo`, `_isatty`, `_lseeki64_nolock`, `errno`, `_doserrno`, `_dosmaperr`)
//! are in `crt.rs`; `_mbtowc_l` (`00ee3928`) and `_isleadbyte` (`00ed41d4`) in `crt/scanf.rs`.
//! The Windows calls are made through their import slots, so every write goes to the
//! platform's `WriteFile` (`units/platform/files.rs`).

use super::*;

/// KERNEL32 imports `GetConsoleCP`, `GetConsoleMode`, `WriteConsoleW`, `GetConsoleOutputCP`
/// and `WriteConsoleA` (import slots; the names from Ghidra's decompilation of the callers).
const IMPORT_GET_CONSOLE_CP: u32 = 0x00fd_f204;
const IMPORT_GET_CONSOLE_MODE: u32 = 0x00fd_f23c;
const IMPORT_WRITE_CONSOLE_W: u32 = 0x00fd_f218;
const IMPORT_GET_CONSOLE_OUTPUT_CP: u32 = 0x00fd_f21c;
const IMPORT_WRITE_CONSOLE_A: u32 = 0x00fd_f220;
/// `_errno` (`00ec85e3`), `__doserrno` (`00ec85f6`), the invalid-parameter handler
/// (`00ec7c30`, five words), `_dosmaperr` (`00ec8609`), `_lseeki64_nolock` (`00ee1fa7`),
/// `_getptd` (`00ed11de`), `_isleadbyte` (`00ed41d4`), `_mbtowc_l` (`00ee3928`) and
/// `__initconout` (`00ee626f`), all cdecl. `_isatty` (`00ee2145`) is `ISATTY` in `crt.rs`.
const ERRNO_ADDR: u32 = 0x00ec_85e3;
const DOSERRNO_ADDR: u32 = 0x00ec_85f6;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
const DOSMAPERR_ADDR: u32 = 0x00ec_8609;
const LSEEKI64_NOLOCK_ADDR: u32 = 0x00ee_1fa7;
const GETPTD_ADDR: u32 = 0x00ed_11de;
const ISLEADBYTE_ADDR: u32 = 0x00ed_41d4;
const MBTOWC_L_ADDR: u32 = 0x00ee_3928;
const INITCONOUT: u32 = 0x00ee_626f;
/// `__putwch_nolock`'s globals: whether `WriteConsoleW` is used (`011c2f6c`: 2 untried, the
/// exe's initial value; 1 it worked; 0 it is not supported) and the console output handle
/// `__initconout` opens (`011c2f74`: -2 not opened yet, the initial value; -1 failed).
const CONSOLE_WIDE: u32 = 0x011c_2f6c;
const CONSOLE_OUT: u32 = 0x011c_2f74;

/// `_write_nolock`'s frame (`SUB` 0x1ae4 through the stack probe), as offsets below `EBP`: the
/// console mode and then the console code page (`[EBP-0x1ae4]`), the count `WriteFile`
/// writes (`[EBP-0x1ad4]`; `[EBP-0x1ac4]` on the console path), the wide character `_mbtowc`
/// makes, which is also the Windows error kept for the end (`[EBP-0x1ac0]`), the buffer the
/// text modes fill (`[EBP-0x1ab8]`, 0x1400 bytes), the UTF-8 output (`[EBP-0x1410]`, 0xd55
/// bytes), the UTF-16 copy made for it (`[EBP-0x6b8]`, 0x6a8 bytes) and the console path's
/// multibyte character (`[EBP-0xc]`, 5 bytes).
const FRAME: u32 = 0x1ae4;
const CONSOLE_MODE: u32 = 0x1ae4;
const WRITTEN: u32 = 0x1ad4;
const CONSOLE_WRITTEN: u32 = 0x1ac4;
const WIDE: u32 = 0x1ac0;
const LF_BUFFER: u32 = 0x1ab8;
const UTF8_BUFFER: u32 = 0x1410;
const WIDE_BUFFER: u32 = 0x6b8;
const CHARACTER: u32 = 0xc;

/// Where `_write_nolock`'s paths end: `00edc6b3` (the count written, else the Windows error
/// kept at `[EBP-0x1ac0]`) or `00edc6f8` (a text-mode count of 0, before any write).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum WriteEnd {
    Written,
    Empty,
}

/// `_write_nolock`'s two counters: the bytes accounted for (`[EBP-0x1ac8]`) and the CRs it
/// added (`[EBP-0x1ad0]`); the answer is their difference.
#[derive(Default)]
struct WriteCounts {
    characters: u32,
    added: u32,
}

/// `errno` set through `_errno` (`00ec85e3`).
fn store_errno(e: &mut Engine, code: u32) {
    let slot = e.call(ERRNO_ADDR, &args![]).u32();
    e.mem.set_u32(slot, code);
}

/// `_doserrno` set through `__doserrno` (`00ec85f6`).
fn store_doserrno(e: &mut Engine, code: u32) {
    let slot = e.call(DOSERRNO_ADDR, &args![]).u32();
    e.mem.set_u32(slot, code);
}

/// `00edc6a7`: `GetLastError` kept at `[EBP-0x1ac0]`.
fn keep_last_error(e: &mut Engine, ebp: u32) {
    let error = e.call(IMPORT_GET_LAST_ERROR, &args![]).u32();
    e.mem.set_u32(ebp - WIDE, error);
}

/// `WriteFile(the handle's osfhnd, from, count, written, NULL)`.
fn write_os(e: &mut Engine, handle: u32, from: u32, count: u32, written: u32) -> u32 {
    let os = e.mem.u32(ioinfo(e, handle));
    e.call(IMPORT_WRITE_FILE, &args![os, from, count, written, 0u32])
        .u32()
}

// Translated from 00edc010 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_write_nolock` (LIBCMT/write.c, engine map), cdecl: (handle, buffer, count). A count of 0
/// answers 0. A null buffer, or an odd count on a UTF-16 or UTF-8 handle (`ioinfo` +0x24 bits
/// 0-6 sign-extended, 2 or 1), sets `_doserrno` 0 and `errno` `EINVAL` (0x16), calls the
/// invalid-parameter handler and answers -1. An append handle (`FAPPEND`, `osfile` 0x20) is
/// moved to its end first (`_lseeki64_nolock(handle, 0, SEEK_END)`).
///
/// A text handle (`FTEXT` 0x80) that is a device (`_isatty`) and a console (`GetConsoleMode`
/// succeeds) is written character by character, unless it is ANSI in the "C" locale (the
/// thread's locale info, `_getptd()` +0x6c, has no `LC_CTYPE` handle at +0x14): ANSI
/// characters become a wide character (`_mbtowc`; a lead byte takes the next byte too, and a
/// lead byte at the very end is kept in `ioinfo` +0x34 with +0x38 set, to go before the next
/// write's first byte), converted to the console's code page (`GetConsoleCP`,
/// `WideCharToMultiByte` into 5 bytes) and written with `WriteFile`, and an LF is followed by
/// a CR written alone; UTF-16 and UTF-8 handles write each 16-bit unit with
/// `__putwch_nolock`, an LF followed by a CR.
///
/// Otherwise a binary handle is one `WriteFile`. Text: ANSI is copied into a 0x1400-byte buffer
/// with a CR before each LF (up to 0x13ff bytes a time), UTF-16 likewise by 16-bit units (up
/// to 0x13fe bytes); UTF-8 handles take 16-bit units into a 0x6a8-byte buffer (a CR unit before
/// each LF), convert it to UTF-8 (`WideCharToMultiByte(CP_UTF8, ...)` into 0xd55 bytes) and
/// write that until all of it is written. Each buffer is written with `WriteFile`; a short
/// write ends the loop.
///
/// The answer is the bytes taken from the buffer (the bytes written less the CRs added). With
/// nothing written: a failed Windows call's error (`GetLastError`) gives `EBADF` (9) and
/// `_doserrno` 5 for `ERROR_ACCESS_DENIED` (5), else `_dosmaperr`, and -1; no error answers 0
/// for a device (`FDEV` 0x40) whose buffer starts with Ctrl-Z (0x1a), else `errno` `ENOSPC`
/// (0x1c), `_doserrno` 0 and -1.
///
/// The frame is in engine memory at the exe's offsets (`write_frame`); the stack probe
/// (`__alloca_probe` `00ec8660` with 0x1ae4) only touches the stack's pages and is left out.
/// The exe reads the kept wide character `[EBP-0x1ac0]` as a Windows error when the console
/// path ends before writing anything (a failed conversion), as it does here.
// Translation incomplete: 00edc010 - the /GS stack cookie check (00ec408c) is not translated.
pub fn write_nolock(e: &mut Engine, handle: u32, buffer: Ptr, count: u32) -> i32 {
    let start = buffer.addr();
    if count == 0 {
        return 0;
    }
    if start == 0 {
        store_doserrno(e, 0);
        store_errno(e, 0x16);
        e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        return -1;
    }
    let entry = ioinfo(e, handle);
    // MOV BL,[+0x24]; ADD BL,BL; SAR BL,1: bits 0-6 sign-extended.
    let mode = ((e.mem.u8(entry + 0x24) << 1) as i8) >> 1;
    // NOT ECX; TEST CL,1: an odd count is refused.
    if (mode == 2 || mode == 1) && count & 1 != 0 {
        store_doserrno(e, 0);
        store_errno(e, 0x16);
        e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        return -1;
    }
    if e.mem.u8(entry + 4) & 0x20 != 0 {
        e.call(LSEEKI64_NOLOCK_ADDR, &args![handle, 0u32, 0u32, 2u32]);
    }
    e.with_stack(FRAME, |e, frame| {
        write_frame(e, handle, start, count, mode, frame.addr() + FRAME)
    })
}

/// `_write_nolock` from `00edc0f7` on, with `EBP` at `ebp`.
fn write_frame(e: &mut Engine, handle: u32, start: u32, count: u32, mode: i8, ebp: u32) -> i32 {
    let mut counts = WriteCounts::default();
    let mut console = false;
    if e.call(ISATTY, &args![handle]).u32() != 0 && e.mem.u8(ioinfo(e, handle) + 4) & 0x80 != 0 {
        let block = e.call(GETPTD_ADDR, &args![]).u32();
        // _getptd()->ptlocinfo (+0x6c), its lc_handle[LC_CTYPE] (+0x14): SETZ.
        let c_locale = e.mem.u32(e.mem.u32(block + 0x6c) + 0x14) == 0;
        let os = e.mem.u32(ioinfo(e, handle));
        let ok = e
            .call(IMPORT_GET_CONSOLE_MODE, &args![os, ebp - CONSOLE_MODE])
            .u32();
        console = ok != 0 && (!c_locale || mode != 0);
    }
    let end = if console {
        write_console(e, handle, start, count, mode, ebp, &mut counts)
    } else {
        write_file_body(e, handle, start, count, mode, ebp, &mut counts)
    };
    if end == WriteEnd::Written {
        if counts.characters != 0 {
            return counts.characters.wrapping_sub(counts.added) as i32;
        }
        let error = e.mem.u32(ebp - WIDE);
        if error != 0 {
            if error == 5 {
                store_errno(e, 9);
                store_doserrno(e, 5);
            } else {
                e.call(DOSMAPERR_ADDR, &args![error]);
            }
            return -1;
        }
    }
    if e.mem.u8(ioinfo(e, handle) + 4) & 0x40 != 0 && e.mem.u8(start) == 0x1a {
        return 0;
    }
    store_errno(e, 0x1c);
    store_doserrno(e, 0);
    -1
}

/// `_write_nolock`'s console path (`00edc157` .. `00edc3a0`).
fn write_console(
    e: &mut Engine,
    handle: u32,
    start: u32,
    count: u32,
    mode: i8,
    ebp: u32,
    counts: &mut WriteCounts,
) -> WriteEnd {
    let code_page = e.call(IMPORT_GET_CONSOLE_CP, &args![]).u32();
    e.mem.set_u32(ebp - CONSOLE_MODE, code_page);
    let mut at = start;
    e.mem.set_u32(ebp - CONSOLE_WRITTEN, 0);
    // JBE 00edc6bc: never taken (the count is not 0 here); nothing is counted yet, so it ends
    // as 00edc6b3 does.
    if count == 0 {
        return WriteEnd::Written;
    }
    let wide = ebp - WIDE;
    let character = ebp - CHARACTER;
    let mut used = 0u32;
    loop {
        if mode == 0 {
            let c = e.mem.u8(at);
            let newline = c == b'\n';
            let entry = ioinfo(e, handle);
            let converted = if e.mem.u32(entry + 0x38) != 0 {
                // The lead byte the last write kept, then this byte.
                let lead = e.mem.u8(entry + 0x34);
                e.mem.set_u8(character, lead);
                e.mem.set_u8(character + 1, c);
                e.mem.set_u32(entry + 0x38, 0);
                mbtowc(e, wide, character, 2)
            } else if e.call(ISLEADBYTE_ADDR, &args![c as i8 as i32 as u32]).u32() != 0 {
                let left = start.wrapping_sub(at).wrapping_add(count);
                if left <= 1 {
                    // 00edc38c: the lead byte is the last one; kept for the next write.
                    counts.characters = counts.characters.wrapping_add(1);
                    let b = e.mem.u8(at);
                    e.mem.set_u8(ioinfo(e, handle) + 0x34, b);
                    e.mem.set_u32(ioinfo(e, handle) + 0x38, 1);
                    return WriteEnd::Written;
                }
                let r = mbtowc(e, wide, at, 2);
                if r == -1 {
                    return WriteEnd::Written;
                }
                at = at.wrapping_add(1);
                used = used.wrapping_add(1);
                r
            } else {
                mbtowc(e, wide, at, 1)
            };
            if converted == -1 {
                return WriteEnd::Written;
            }
            at = at.wrapping_add(1);
            used = used.wrapping_add(1);
            let code_page = e.mem.u32(ebp - CONSOLE_MODE);
            let size = e
                .call(
                    IMPORT_WIDE_CHAR_TO_MULTI_BYTE,
                    &args![code_page, 0u32, wide, 1u32, character, 5u32, 0u32, 0u32],
                )
                .u32();
            if size == 0 {
                return WriteEnd::Written;
            }
            if write_os(e, handle, character, size, ebp - CONSOLE_WRITTEN) == 0 {
                keep_last_error(e, ebp);
                return WriteEnd::Written;
            }
            counts.characters = used.wrapping_add(counts.added);
            if (e.mem.u32(ebp - CONSOLE_WRITTEN) as i32) < (size as i32) {
                return WriteEnd::Written;
            }
            if newline {
                e.mem.set_u8(character, 0xd);
                if write_os(e, handle, character, 1, ebp - CONSOLE_WRITTEN) == 0 {
                    keep_last_error(e, ebp);
                    return WriteEnd::Written;
                }
                if (e.mem.u32(ebp - CONSOLE_WRITTEN) as i32) < 1 {
                    return WriteEnd::Written;
                }
                counts.added = counts.added.wrapping_add(1);
                counts.characters = counts.characters.wrapping_add(1);
            }
        } else if mode == 1 || mode == 2 {
            // The exe tests the mode twice (00edc2f5 and 00edc31e); nothing between changes it.
            let c = u32::from(e.mem.u16(at));
            let newline = c == 0xa;
            at = at.wrapping_add(2);
            used = used.wrapping_add(2);
            e.mem.set_u32(wide, c);
            let w = e.mem.u32(wide);
            if putwch_nolock(e, w) as u16 != e.mem.u16(wide) {
                keep_last_error(e, ebp);
                return WriteEnd::Written;
            }
            counts.characters = counts.characters.wrapping_add(2);
            if newline {
                e.mem.set_u32(wide, 0xd);
                if putwch_nolock(e, 0xd) as u16 != e.mem.u16(wide) {
                    keep_last_error(e, ebp);
                    return WriteEnd::Written;
                }
                counts.characters = counts.characters.wrapping_add(1);
                counts.added = counts.added.wrapping_add(1);
            }
        }
        // Any other mode (the CRT only sets 0, 1 and 2) takes nothing and loops for ever, as
        // the exe does.
        if used >= count {
            return WriteEnd::Written;
        }
    }
}

/// `_write_nolock`'s path for a handle that is not a console (`00edc3a5` .. `00edc6a5`).
fn write_file_body(
    e: &mut Engine,
    handle: u32,
    start: u32,
    count: u32,
    mode: i8,
    ebp: u32,
    counts: &mut WriteCounts,
) -> WriteEnd {
    let entry = ioinfo(e, handle);
    if e.mem.u8(entry + 4) & 0x80 == 0 {
        // 00edc674: binary, one write.
        if write_os(e, handle, start, count, ebp - WRITTEN) == 0 {
            keep_last_error(e, ebp);
            return WriteEnd::Written;
        }
        counts.characters = e.mem.u32(ebp - WRITTEN);
        e.mem.set_u32(ebp - WIDE, 0);
        return WriteEnd::Written;
    }
    e.mem.set_u32(ebp - WIDE, 0);
    match mode {
        0 => write_text_ansi(e, handle, start, count, ebp, counts),
        2 => write_text_utf16(e, handle, start, count, ebp, counts),
        _ => write_text_utf8(e, handle, start, count, ebp, counts),
    }
}

/// `_write_nolock`'s ANSI text loop (`00edc3c9` .. `00edc48e`).
fn write_text_ansi(
    e: &mut Engine,
    handle: u32,
    start: u32,
    count: u32,
    ebp: u32,
    counts: &mut WriteCounts,
) -> WriteEnd {
    let mut from = start;
    if count == 0 {
        return WriteEnd::Empty;
    }
    let buffer = ebp - LF_BUFFER;
    loop {
        let mut taken = from.wrapping_sub(start);
        let mut stored = 0u32;
        let mut to = buffer;
        while taken < count {
            let c = e.mem.u8(from);
            from = from.wrapping_add(1);
            taken = taken.wrapping_add(1);
            if c == b'\n' {
                counts.added = counts.added.wrapping_add(1);
                e.mem.set_u8(to, 0xd);
                to = to.wrapping_add(1);
                stored = stored.wrapping_add(1);
            }
            e.mem.set_u8(to, c);
            to = to.wrapping_add(1);
            stored = stored.wrapping_add(1);
            if stored >= 0x13ff {
                break;
            }
        }
        let size = to.wrapping_sub(buffer);
        if write_os(e, handle, buffer, size, ebp - WRITTEN) == 0 {
            keep_last_error(e, ebp);
            return WriteEnd::Written;
        }
        let written = e.mem.u32(ebp - WRITTEN);
        counts.characters = counts.characters.wrapping_add(written);
        if (written as i32) < (size as i32) || from.wrapping_sub(start) >= count {
            return WriteEnd::Written;
        }
    }
}

/// `_write_nolock`'s UTF-16 text loop (`00edc493` .. `00edc56e`).
fn write_text_utf16(
    e: &mut Engine,
    handle: u32,
    start: u32,
    count: u32,
    ebp: u32,
    counts: &mut WriteCounts,
) -> WriteEnd {
    let mut from = start;
    if count == 0 {
        return WriteEnd::Empty;
    }
    let buffer = ebp - LF_BUFFER;
    loop {
        let mut taken = from.wrapping_sub(start);
        let mut stored = 0u32;
        let mut to = buffer;
        while taken < count {
            let c = e.mem.u16(from);
            from = from.wrapping_add(2);
            taken = taken.wrapping_add(2);
            if c == 0xa {
                counts.added = counts.added.wrapping_add(2);
                e.mem.set_u16(to, 0xd);
                to = to.wrapping_add(2);
                stored = stored.wrapping_add(2);
            }
            stored = stored.wrapping_add(2);
            e.mem.set_u16(to, c);
            to = to.wrapping_add(2);
            if stored >= 0x13fe {
                break;
            }
        }
        let size = to.wrapping_sub(buffer);
        if write_os(e, handle, buffer, size, ebp - WRITTEN) == 0 {
            keep_last_error(e, ebp);
            return WriteEnd::Written;
        }
        let written = e.mem.u32(ebp - WRITTEN);
        counts.characters = counts.characters.wrapping_add(written);
        if (written as i32) < (size as i32) || from.wrapping_sub(start) >= count {
            return WriteEnd::Written;
        }
    }
}

/// `_write_nolock`'s UTF-8 text loop (`00edc573` .. `00edc672`; also any mode but 0 and 2).
fn write_text_utf8(
    e: &mut Engine,
    handle: u32,
    start: u32,
    count: u32,
    ebp: u32,
    counts: &mut WriteCounts,
) -> WriteEnd {
    let mut from = start;
    if count == 0 {
        return WriteEnd::Empty;
    }
    let wide = ebp - WIDE_BUFFER;
    let utf8 = ebp - UTF8_BUFFER;
    loop {
        let mut taken = from.wrapping_sub(start);
        let mut stored = 0u32;
        let mut to = wide;
        while taken < count {
            let c = e.mem.u16(from);
            from = from.wrapping_add(2);
            taken = taken.wrapping_add(2);
            if c == 0xa {
                e.mem.set_u16(to, 0xd);
                to = to.wrapping_add(2);
                stored = stored.wrapping_add(2);
            }
            stored = stored.wrapping_add(2);
            e.mem.set_u16(to, c);
            to = to.wrapping_add(2);
            if stored >= 0x6a8 {
                break;
            }
        }
        // SUB EAX,ECX; CDQ; SUB EAX,EDX; SAR EAX,1: the units, a signed halving.
        let units = (to.wrapping_sub(wide) as i32 / 2) as u32;
        let bytes = e
            .call(
                IMPORT_WIDE_CHAR_TO_MULTI_BYTE,
                &args![0xfde9u32, 0u32, wide, units, utf8, 0xd55u32, 0u32, 0u32],
            )
            .u32();
        if bytes == 0 {
            keep_last_error(e, ebp);
            return WriteEnd::Written;
        }
        let mut sent = 0u32;
        loop {
            let at = utf8.wrapping_add(sent);
            let left = bytes.wrapping_sub(sent);
            if write_os(e, handle, at, left, ebp - WRITTEN) == 0 {
                // 00edc647
                keep_last_error(e, ebp);
                break;
            }
            sent = sent.wrapping_add(e.mem.u32(ebp - WRITTEN));
            if (bytes as i32) <= (sent as i32) {
                break;
            }
        }
        if (bytes as i32) > (sent as i32) {
            return WriteEnd::Written;
        }
        counts.characters = from.wrapping_sub(start);
        if counts.characters >= count {
            return WriteEnd::Written;
        }
    }
}

// Translated from 00ee3a3f (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbtowc` (LIBCMT/mbtowc.c, engine map), cdecl: (wide out, text, size):
/// `_mbtowc_l(out, text, size, NULL)` (`00ee3928`), the current locale.
pub fn mbtowc(e: &mut Engine, out: u32, text: u32, size: u32) -> i32 {
    e.call(MBTOWC_L_ADDR, &args![out, text, size, 0u32]).i32()
}

// Translated from 00ee46af (disassembled, FalloutNV.exe 1.4.0.525)
/// `__putwch_nolock` (LIBCMT/putwch.c, engine map), cdecl: the character word. While
/// `WriteConsoleW` is not known to be unsupported (`011c2f6c` not 0): the console output
/// handle (`011c2f74`) is opened first if it is not yet (-2, `__initconout`); -1 answers WEOF
/// (0xffff). `WriteConsoleW(handle, &the argument, 1, &written, NULL)` succeeding marks it used
/// (1) and answers the character; failing answers WEOF unless it was untried (2) and the error
/// is `ERROR_CALL_NOT_IMPLEMENTED` (0x78), which marks it unsupported (0). Then the character
/// is converted to the console's output code page (`GetConsoleOutputCP`,
/// `WideCharToMultiByte` into 5 bytes) and written with `WriteConsoleA` (a handle of -1 or a
/// failure: WEOF). The character is answered in `AX` (`MOV AX,[EBP+8]`), the upper half of
/// `EAX` being the upper half of the `WriteConsoleW`/`WriteConsoleA` result, as in the exe.
// Translation incomplete: 00ee46af - the /GS stack cookie check (00ec408c) is not translated.
pub fn putwch_nolock(e: &mut Engine, character: u32) -> u32 {
    // [EBP-0x10] the count written, [EBP-0xc] the multibyte bytes, [EBP+8] the argument.
    e.with_stack(0x1c, |e, frame| {
        let ebp = frame.addr() + 0x10;
        let argument = ebp + 8;
        e.mem.set_u32(argument, character);
        putwch_frame(e, ebp - 0x10, ebp - 0xc, argument)
    })
}

/// `__putwch_nolock`'s body with its frame's slots.
fn putwch_frame(e: &mut Engine, written: u32, bytes: u32, argument: u32) -> u32 {
    if e.mem.u32(CONSOLE_WIDE) != 0 {
        if e.mem.u32(CONSOLE_OUT) == 0xffff_fffe {
            e.call(INITCONOUT, &args![]);
        }
        let out = e.mem.u32(CONSOLE_OUT);
        if out == 0xffff_ffff {
            return 0xffff;
        }
        let ok = e
            .call(
                IMPORT_WRITE_CONSOLE_W,
                &args![out, argument, 1u32, written, 0u32],
            )
            .u32();
        if ok != 0 {
            e.mem.set_u32(CONSOLE_WIDE, 1);
            return (ok & 0xffff_0000) | u32::from(e.mem.u16(argument));
        }
        if e.mem.u32(CONSOLE_WIDE) != 2 {
            return 0xffff;
        }
        if e.call(IMPORT_GET_LAST_ERROR, &args![]).u32() != 0x78 {
            return 0xffff;
        }
        e.mem.set_u32(CONSOLE_WIDE, 0);
    }
    let code_page = e.call(IMPORT_GET_CONSOLE_OUTPUT_CP, &args![]).u32();
    let size = e
        .call(
            IMPORT_WIDE_CHAR_TO_MULTI_BYTE,
            &args![code_page, 0u32, argument, 1u32, bytes, 5u32, 0u32, 0u32],
        )
        .u32();
    let out = e.mem.u32(CONSOLE_OUT);
    if out == 0xffff_ffff {
        return 0xffff;
    }
    let ok = e
        .call(
            IMPORT_WRITE_CONSOLE_A,
            &args![out, bytes, size, written, 0u32],
        )
        .u32();
    if ok == 0 {
        return 0xffff;
    }
    (ok & 0xffff_0000) | u32::from(e.mem.u16(argument))
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ed_c010, write_nolock(u32, Ptr, u32) -> i32),
        entry!(0x00ee_3a3f, mbtowc(u32, u32, u32) -> i32),
        entry!(0x00ee_46af, putwch_nolock(u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// What a doubled `WriteFile` was given: (Windows handle, bytes).
    type Writes = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The invalid-parameter handler the doubled `__decode_pointer` answers.
    const HANDLER: u32 = 0x0e30_0000;
    /// The CRT handle the tests write to and its Windows handle.
    const H: u32 = 3;
    const OS: u32 = 0x600;

    /// An engine with the CRT's data pages mapped (the handler's slot, the console globals,
    /// `__pioinfo`), a thread block (`errno` +8, `_doserrno`
    /// +0xc, the locale info at +0x6c with an `LC_CTYPE` handle) for `__getptd_noexit` and
    /// `_getptd`, one `ioinfo` block of 32 entries with handle 3 open (`osfile` as given,
    /// Windows handle 0x600), and `WriteFile` doubled to write everything it is given, at most
    /// `limit` bytes a call. Answers the engine, the thread block and the writes.
    fn engine(osfile: u8, limit: u32) -> (Engine, u32, Writes) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011c_1000, 0x2000);
        e.map(0x0127_0000, 0x2000);
        let block = e.mem.alloc(0x80);
        e.register_double(GETPTD_NOEXIT, move |_, _| block.into_ret());
        e.register_double(GETPTD_ADDR, move |_, _| block.into_ret());
        let locale = e.mem.alloc(0x100);
        e.mem.set_u32(locale + 0x14, 0x409);
        e.mem.set_u32(block + 0x6c, locale);
        e.register(DECODE_POINTER, |_, _| HANDLER.into_ret());
        e.register(HANDLER, |_, _| Ret::default());
        let entries = e.mem.alloc(0x800);
        for index in 0..0x20 {
            e.mem.set_u32(entries + 0x40 * index, 0xffff_ffff);
        }
        e.mem.set_u32(entries + 0x40 * H, OS);
        e.mem.set_u8(entries + 0x40 * H + 4, osfile);
        e.mem.set_u32(PIOINFO, entries);
        e.mem.set_u32(NHANDLE, 0x20);
        let writes: Writes = Rc::default();
        let log = writes.clone();
        e.register_double(IMPORT_WRITE_FILE, move |e, a| {
            let n = a[2].min(limit);
            log.borrow_mut().push((a[0], e.mem.bytes(a[1], n)));
            e.mem.set_u32(a[3], n);
            1u32.into_ret()
        });
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        (e, block, writes)
    }

    /// `bytes` in engine memory; answers the address.
    fn buffer(e: &mut Engine, bytes: &[u8]) -> u32 {
        let at = e.mem.alloc(bytes.len() as u32 + 4);
        e.mem.write(at, bytes);
        at
    }

    /// UTF-16 units as little-endian bytes.
    fn units(text: &[u16]) -> Vec<u8> {
        text.iter().flat_map(|u| u.to_le_bytes()).collect()
    }

    fn all(writes: &Writes) -> Vec<u8> {
        writes
            .borrow()
            .iter()
            .flat_map(|w| w.1.iter().copied())
            .collect()
    }

    #[test]
    fn write_nolock_text_writes_cr_lf_in_buffers_of_0x13ff() {
        let (mut e, _, writes) = engine(0x81, u32::MAX);
        let text = buffer(&mut e, b"a\nb\n");
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 4u32]).i32(), 4);
        assert_eq!(all(&writes), b"a\r\nb\r\n".to_vec());
        assert_eq!(writes.borrow()[0].0, OS);
        let file = calls(&e, IMPORT_WRITE_FILE);
        assert_eq!(file[0][2], 6);
        assert_eq!(file[0][4], 0);
        // Not a device: no console call.
        assert!(calls(&e, IMPORT_GET_CONSOLE_MODE).is_empty());
        // 0x1000 LFs: 0xa00 of them fill 0x1400 bytes, the rest 0xc00.
        writes.borrow_mut().clear();
        let lines = buffer(&mut e, &[b'\n'; 0x1000]);
        assert_eq!(
            e.call(0x00ed_c010, &args![H, lines, 0x1000u32]).i32(),
            0x1000
        );
        let sizes: Vec<usize> = writes.borrow().iter().map(|w| w.1.len()).collect();
        assert_eq!(sizes, vec![0x1400, 0xc00]);
        // 0x13ff plain bytes a buffer.
        writes.borrow_mut().clear();
        let plain = buffer(&mut e, &[b'x'; 0x1400]);
        assert_eq!(
            e.call(0x00ed_c010, &args![H, plain, 0x1400u32]).i32(),
            0x1400
        );
        let sizes: Vec<usize> = writes.borrow().iter().map(|w| w.1.len()).collect();
        assert_eq!(sizes, vec![0x13ff, 1]);
    }

    #[test]
    fn write_nolock_text_stops_at_a_short_write() {
        // Two bytes a call: "a\r" written, the answer less the CR not yet written.
        let (mut e, _, writes) = engine(0x81, 2);
        let text = buffer(&mut e, b"a\nb");
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 3u32]).i32(), 1);
        assert_eq!(all(&writes), b"a\r".to_vec());
        assert_eq!(calls(&e, IMPORT_WRITE_FILE).len(), 1);
    }

    #[test]
    fn write_nolock_binary_appends_and_reports_errors() {
        let (mut e, block, writes) = engine(0x21, u32::MAX);
        e.register(LSEEKI64_NOLOCK_ADDR, |_, _| 0u64.into_ret());
        let data = buffer(&mut e, b"a\n\x1a");
        assert_eq!(e.call(0x00ed_c010, &args![H, data, 3u32]).i32(), 3);
        assert_eq!(all(&writes), b"a\n\x1a".to_vec());
        assert_eq!(calls(&e, LSEEKI64_NOLOCK_ADDR), vec![vec![H, 0, 0, 2]]);
        // A count of 0: nothing.
        assert_eq!(e.call(0x00ed_c010, &args![H, data, 0u32]).i32(), 0);
        assert_eq!(calls(&e, IMPORT_WRITE_FILE).len(), 1);
        // A null buffer: EINVAL with the handler.
        assert_eq!(e.call(0x00ed_c010, &args![H, 0u32, 3u32]).i32(), -1);
        assert_eq!(e.mem.u32(block + 8), 0x16);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        assert_eq!(calls(&e, HANDLER).len(), 1);
        // A failed write: access denied is EBADF, others are mapped.
        e.register(IMPORT_WRITE_FILE, |_, _| 0u32.into_ret());
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 5u32.into_ret());
        assert_eq!(e.call(0x00ed_c010, &args![H, data, 3u32]).i32(), -1);
        assert_eq!(e.mem.u32(block + 8), 9);
        assert_eq!(e.mem.u32(block + 0xc), 5);
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 0x70u32.into_ret());
        assert_eq!(e.call(0x00ed_c010, &args![H, data, 3u32]).i32(), -1);
        assert_eq!(calls(&e, DOSMAPERR_ADDR).last().unwrap(), &vec![0x70]);
        assert_eq!(e.mem.u32(block + 0xc), 0x70);
        // Nothing written without an error: ENOSPC, but 0 on a device starting with Ctrl-Z.
        e.register(IMPORT_WRITE_FILE, |e, a| {
            e.mem.set_u32(a[3], 0);
            1u32.into_ret()
        });
        assert_eq!(e.call(0x00ed_c010, &args![H, data, 3u32]).i32(), -1);
        assert_eq!(e.mem.u32(block + 8), 0x1c);
        assert_eq!(e.mem.u32(block + 0xc), 0);
        let entry = ioinfo(&e, H);
        e.mem.set_u8(entry + 4, 0x41);
        assert_eq!(e.call(0x00ed_c010, &args![H, data, 3u32]).i32(), -1);
        let ctrl_z = buffer(&mut e, b"\x1a");
        assert_eq!(e.call(0x00ed_c010, &args![H, ctrl_z, 1u32]).i32(), 0);
        // A device that is binary is not a console.
        assert!(calls(&e, IMPORT_GET_CONSOLE_MODE).is_empty());
    }

    #[test]
    fn write_nolock_utf16_writes_cr_units_and_refuses_odd_counts() {
        let (mut e, block, writes) = engine(0x81, u32::MAX);
        let entry = ioinfo(&e, H);
        e.mem.set_u8(entry + 0x24, 2);
        let text = buffer(&mut e, &units(&[0x61, 0xa]));
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 3u32]).i32(), -1);
        assert_eq!(e.mem.u32(block + 8), 0x16);
        assert_eq!(calls(&e, HANDLER).len(), 1);
        assert!(writes.borrow().is_empty());
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 4u32]).i32(), 4);
        assert_eq!(all(&writes), units(&[0x61, 0xd, 0xa]));
        // The Unicode bit (+0x24 bit 7) is not part of the mode.
        e.mem.set_u8(entry + 0x24, 0x82);
        writes.borrow_mut().clear();
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 4u32]).i32(), 4);
        assert_eq!(all(&writes), units(&[0x61, 0xd, 0xa]));
        // 0x13fe bytes a buffer.
        writes.borrow_mut().clear();
        let long = buffer(&mut e, &units(&[0x78; 0xa00]));
        assert_eq!(
            e.call(0x00ed_c010, &args![H, long, 0x1400u32]).i32(),
            0x1400
        );
        let sizes: Vec<usize> = writes.borrow().iter().map(|w| w.1.len()).collect();
        assert_eq!(sizes, vec![0x13fe, 2]);
    }

    #[test]
    fn write_nolock_utf8_converts_and_writes_until_all_is_written() {
        let (mut e, _, writes) = engine(0x81, 3);
        let entry = ioinfo(&e, H);
        e.mem.set_u8(entry + 0x24, 1);
        e.register(IMPORT_WIDE_CHAR_TO_MULTI_BYTE, |e, a| {
            let text: Vec<u16> = (0..a[3]).map(|i| e.mem.u16(a[2] + 2 * i)).collect();
            let utf8 = String::from_utf16(&text).unwrap().into_bytes();
            e.mem.write(a[4], &utf8);
            (utf8.len() as u32).into_ret()
        });
        let text = buffer(&mut e, &units(&[0xe9, 0xa]));
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 4u32]).i32(), 4);
        assert_eq!(all(&writes), b"\xc3\xa9\r\n".to_vec());
        let converted = calls(&e, IMPORT_WIDE_CHAR_TO_MULTI_BYTE);
        assert_eq!(converted[0][..2], [0xfde9, 0]);
        assert_eq!(converted[0][3], 3);
        assert_eq!(converted[0][5..], [0xd55, 0, 0]);
        // The UTF-16 copy sits 0xd58 bytes above the UTF-8 output, as in the exe's frame.
        assert_eq!(converted[0][2] - converted[0][4], 0xd58);
        // Three bytes, then the one left from where the first write stopped.
        let file = calls(&e, IMPORT_WRITE_FILE);
        assert_eq!(file.len(), 2);
        assert_eq!(file[1][1], file[0][1] + 3);
        assert_eq!((file[0][2], file[1][2]), (4, 1));
        // A failed conversion: GetLastError, mapped.
        e.register(IMPORT_WIDE_CHAR_TO_MULTI_BYTE, |_, _| 0u32.into_ret());
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 0x57u32.into_ret());
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 4u32]).i32(), -1);
        assert_eq!(calls(&e, DOSMAPERR_ADDR), vec![vec![0x57]]);
    }

    /// Handle 3 as a console (`FOPEN | FDEV | FTEXT`): `GetConsoleMode` succeeds,
    /// `GetConsoleCP` is 437, `_mbtowc_l` widens one byte or joins two (`lead << 8 | trail`),
    /// 0x81 is a lead byte, and `WideCharToMultiByte` gives the wide character's low byte.
    fn console() -> (Engine, u32, Writes) {
        let (mut e, block, writes) = engine(0xc1, u32::MAX);
        e.register(IMPORT_GET_CONSOLE_MODE, |e, a| {
            e.mem.set_u32(a[1], 3);
            1u32.into_ret()
        });
        e.register(IMPORT_GET_CONSOLE_CP, |_, _| 437u32.into_ret());
        e.register(MBTOWC_L_ADDR, |e, a| {
            let c = if a[2] == 2 {
                u16::from(e.mem.u8(a[1])) << 8 | u16::from(e.mem.u8(a[1] + 1))
            } else {
                u16::from(e.mem.u8(a[1]))
            };
            e.mem.set_u16(a[0], c);
            a[2].into_ret()
        });
        e.register(ISLEADBYTE_ADDR, |_, a| {
            u32::from(a[0] == 0xffff_ff81).into_ret()
        });
        e.register(IMPORT_WIDE_CHAR_TO_MULTI_BYTE, |e, a| {
            let c = e.mem.u16(a[2]) as u8;
            e.mem.set_u8(a[4], c);
            1u32.into_ret()
        });
        (e, block, writes)
    }

    #[test]
    fn write_nolock_console_writes_each_character_and_keeps_a_last_lead_byte() {
        let (mut e, _, writes) = console();
        let text = buffer(&mut e, b"a\n");
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 2u32]).i32(), 2);
        // Each character alone, the CR after the LF.
        let each: Vec<Vec<u8>> = writes.borrow().iter().map(|w| w.1.clone()).collect();
        assert_eq!(each, vec![b"a".to_vec(), b"\n".to_vec(), b"\r".to_vec()]);
        assert_eq!(calls(&e, IMPORT_GET_CONSOLE_MODE)[0][0], OS);
        let converted = calls(&e, IMPORT_WIDE_CHAR_TO_MULTI_BYTE);
        assert_eq!(converted[0][..2], [437, 0]);
        assert_eq!(converted[0][3], 1);
        assert_eq!(converted[0][5..], [5, 0, 0]);
        // [EBP-0x1ac0] and [EBP-0xc].
        assert_eq!(converted[0][4] - converted[0][2], 0x1ab4);
        // A lead byte at the end is kept in ioinfo +0x34/+0x38 and counted.
        writes.borrow_mut().clear();
        let lead = buffer(&mut e, b"x\x81");
        assert_eq!(e.call(0x00ed_c010, &args![H, lead, 2u32]).i32(), 2);
        let entry = ioinfo(&e, H);
        assert_eq!((e.mem.u8(entry + 0x34), e.mem.u32(entry + 0x38)), (0x81, 1));
        // It goes before the next write's first byte, as one character.
        let trail = buffer(&mut e, b"y");
        assert_eq!(e.call(0x00ed_c010, &args![H, trail, 1u32]).i32(), 1);
        assert_eq!(e.mem.u32(entry + 0x38), 0);
        let last = calls(&e, MBTOWC_L_ADDR).last().unwrap().clone();
        assert_eq!(last[2..], [2, 0]);
        assert_eq!(e.mem.u16(last[0]), 0x8179);
        assert_eq!(all(&writes), b"xy".to_vec());
        // A lead byte with its trail byte: two bytes taken, one character.
        writes.borrow_mut().clear();
        let pair = buffer(&mut e, b"\x81zq");
        assert_eq!(e.call(0x00ed_c010, &args![H, pair, 3u32]).i32(), 3);
        assert_eq!(all(&writes), b"zq".to_vec());
        // ANSI in the "C" locale is not written as a console.
        let block = e.call(GETPTD_ADDR, &args![]).u32();
        let locale = e.mem.u32(block + 0x6c);
        e.mem.set_u32(locale + 0x14, 0);
        writes.borrow_mut().clear();
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 2u32]).i32(), 2);
        assert_eq!(all(&writes), b"a\r\n".to_vec());
        assert_eq!(writes.borrow().len(), 1);
    }

    #[test]
    fn write_nolock_console_failures_end_the_write() {
        let (mut e, block, writes) = console();
        let text = buffer(&mut e, b"ab");
        // A failed conversion before anything is written: the kept wide character is read as
        // the error, as in the exe ('a' 0x61, mapped).
        e.register(IMPORT_WIDE_CHAR_TO_MULTI_BYTE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 2u32]).i32(), -1);
        assert_eq!(calls(&e, DOSMAPERR_ADDR), vec![vec![0x61]]);
        assert!(writes.borrow().is_empty());
        // _mbtowc failing: nothing kept, so ENOSPC.
        e.register(MBTOWC_L_ADDR, |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 2u32]).i32(), -1);
        assert_eq!(e.mem.u32(block + 8), 0x1c);
        assert_eq!(calls(&e, DOSMAPERR_ADDR).len(), 1);
        // WriteFile failing after the first character: what was written is answered.
        let (mut e, _, _) = console();
        let text = buffer(&mut e, b"ab");
        let mut n = 0;
        e.register_double(IMPORT_WRITE_FILE, move |e, a| {
            n += 1;
            e.mem.set_u32(a[3], 1);
            u32::from(n == 1).into_ret()
        });
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 2u32]).i32(), 1);
        assert_eq!(calls(&e, IMPORT_GET_LAST_ERROR).len(), 1);
    }

    #[test]
    fn write_nolock_console_wide_writes_units_through_putwch() {
        let (mut e, block, _) = console();
        let entry = ioinfo(&e, H);
        e.mem.set_u8(entry + 0x24, 2);
        e.mem.set_u32(CONSOLE_WIDE, 1);
        e.mem.set_u32(CONSOLE_OUT, 0x77);
        let shown: Rc<RefCell<Vec<u16>>> = Rc::default();
        let log = shown.clone();
        e.register_double(IMPORT_WRITE_CONSOLE_W, move |e, a| {
            log.borrow_mut().push(e.mem.u16(a[1]));
            1u32.into_ret()
        });
        let text = buffer(&mut e, &units(&[0x61, 0xa]));
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 4u32]).i32(), 4);
        assert_eq!(*shown.borrow(), vec![0x61, 0xa, 0xd]);
        assert!(calls(&e, IMPORT_WRITE_FILE).is_empty());
        // The C locale does not matter for a wide mode.
        let locale = e.mem.u32(block + 0x6c);
        e.mem.set_u32(locale + 0x14, 0);
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 2u32]).i32(), 2);
        // putwch failing first: GetLastError, mapped.
        e.register(IMPORT_WRITE_CONSOLE_W, |_, _| 0u32.into_ret());
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 6u32.into_ret());
        assert_eq!(e.call(0x00ed_c010, &args![H, text, 4u32]).i32(), -1);
        assert_eq!(calls(&e, DOSMAPERR_ADDR), vec![vec![6]]);
        assert_eq!(e.mem.u32(block + 0xc), 6);
    }

    #[test]
    fn putwch_nolock_tries_wide_then_falls_back_to_the_code_page() {
        let (mut e, _, _) = engine(0x81, u32::MAX);
        e.mem.set_u32(CONSOLE_WIDE, 2);
        e.mem.set_u32(CONSOLE_OUT, 0xffff_fffe);
        e.register(INITCONOUT, |e, _| {
            e.mem.set_u32(CONSOLE_OUT, 0x55);
            Ret::default()
        });
        e.register(IMPORT_WRITE_CONSOLE_W, |_, _| 0u32.into_ret());
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 0x78u32.into_ret());
        e.register(IMPORT_GET_CONSOLE_OUTPUT_CP, |_, _| 850u32.into_ret());
        e.register(IMPORT_WIDE_CHAR_TO_MULTI_BYTE, |e, a| {
            e.mem.set_u8(a[4], 0x82);
            1u32.into_ret()
        });
        e.register(IMPORT_WRITE_CONSOLE_A, |_, _| 0x0002_0001u32.into_ret());
        // The upper word of the argument is not part of the answer's character.
        assert_eq!(
            e.call(0x00ee_46af, &args![0x1234_00e9u32]).u32(),
            0x0002_00e9
        );
        assert_eq!(e.mem.u32(CONSOLE_WIDE), 0);
        assert_eq!(calls(&e, INITCONOUT).len(), 1);
        let wide = calls(&e, IMPORT_WRITE_CONSOLE_W);
        assert_eq!((wide[0][0], wide[0][2], wide[0][4]), (0x55, 1, 0));
        let converted = calls(&e, IMPORT_WIDE_CHAR_TO_MULTI_BYTE);
        assert_eq!(converted[0][..2], [850, 0]);
        assert_eq!(converted[0][3], 1);
        assert_eq!(converted[0][5..], [5, 0, 0]);
        // Both convert the argument's own slot and count into the same word.
        assert_eq!(converted[0][2], wide[0][1]);
        let ansi = calls(&e, IMPORT_WRITE_CONSOLE_A);
        assert_eq!(ansi[0][..3], [0x55, converted[0][4], 1]);
        assert_eq!(ansi[0][4], 0);
        assert_eq!(ansi[0][3], wide[0][3]);
        // Now straight to the code page.
        assert_eq!(e.call(0x00ee_46af, &args![0x41u32]).u32(), 0x0002_0041);
        assert_eq!(calls(&e, IMPORT_WRITE_CONSOLE_W).len(), 1);
        // WriteConsoleA failing, or no console: WEOF.
        e.register(IMPORT_WRITE_CONSOLE_A, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ee_46af, &args![0x41u32]).u32(), 0xffff);
        e.mem.set_u32(CONSOLE_OUT, 0xffff_ffff);
        assert_eq!(e.call(0x00ee_46af, &args![0x41u32]).u32(), 0xffff);
        e.mem.set_u32(CONSOLE_WIDE, 2);
        assert_eq!(e.call(0x00ee_46af, &args![0x41u32]).u32(), 0xffff);
        assert_eq!(calls(&e, IMPORT_WRITE_CONSOLE_W).len(), 1);
        // WriteConsoleW working marks it used; a later failure is WEOF without asking why.
        e.mem.set_u32(CONSOLE_OUT, 0x55);
        e.register(IMPORT_WRITE_CONSOLE_W, |_, _| 0x0003_0001u32.into_ret());
        assert_eq!(e.call(0x00ee_46af, &args![0x263au32]).u32(), 0x0003_263a);
        assert_eq!(e.mem.u32(CONSOLE_WIDE), 1);
        e.register(IMPORT_WRITE_CONSOLE_W, |_, _| 0u32.into_ret());
        let errors = calls(&e, IMPORT_GET_LAST_ERROR).len();
        assert_eq!(e.call(0x00ee_46af, &args![0x41u32]).u32(), 0xffff);
        assert_eq!(calls(&e, IMPORT_GET_LAST_ERROR).len(), errors);
        // Untried with another error: WEOF, still untried.
        e.mem.set_u32(CONSOLE_WIDE, 2);
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 6u32.into_ret());
        assert_eq!(e.call(0x00ee_46af, &args![0x41u32]).u32(), 0xffff);
        assert_eq!(e.mem.u32(CONSOLE_WIDE), 2);
    }

    #[test]
    fn mbtowc_is_mbtowc_l_in_the_current_locale() {
        let (mut e, _, _) = engine(0x81, u32::MAX);
        e.register(MBTOWC_L_ADDR, |_, _| 2u32.into_ret());
        assert_eq!(e.call(0x00ee_3a3f, &args![0x10u32, 0x20u32, 3u32]).i32(), 2);
        assert_eq!(calls(&e, MBTOWC_L_ADDR), vec![vec![0x10, 0x20, 3, 0]]);
    }
}

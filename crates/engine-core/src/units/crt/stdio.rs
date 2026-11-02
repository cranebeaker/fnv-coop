//! The C runtime's FILE streams for reading text files (MSVC 2008 LIBCMT, statically linked
//! into FalloutNV.exe 1.4.0.525), translated from the exe: `fopen_s` (`00ec9a5e`), `feof`
//! (`00ecdacc`), `fgets` (`00ecc104`), `strtok_s` (`00ecc5f8`), and below them `_read`
//! (`00edcf0c`) and `_read_nolock` (`00edc94a`, with the text-mode CR-LF, Ctrl-Z, pipe
//! look-ahead and UTF-8/UTF-16 handling). The rest of the chain is in `crt.rs` (`_fsopen`,
//! `_getstream`, `_openfile`, `_sopen_s`, `__tsopen_nolock`, the stream and handle locks,
//! `_fileno`, `_getbuf`, `_lseeki64_nolock`) and `crt/scanf.rs` (`_filbuf`).

use super::*;

/// `ReadFile` (KERNEL32 import slot `00fdf080`).
const IMPORT_READ_FILE: u32 = 0x00fd_f080;
/// `_fsopen` (`00ec9983`, cdecl: path, mode, share flag), `_filbuf` (`00edc81f`, cdecl: the
/// stream), `__malloc_crt` (`00ed0cdf`, cdecl: size) and `_read_nolock` (`00edc94a`).
const FSOPEN: u32 = 0x00ec_9983;
const FILBUF: u32 = 0x00ed_c81f;
const MALLOC_CRT: u32 = 0x00ed_0cdf;
/// The UTF-8 trail-byte counts by lead byte (`011c2790`, 256 bytes: 0 for 0x00-0xbf and
/// 0xf8-0xff, 1 for 0xc0-0xdf, 2 for 0xe0-0xef, 3 for 0xf0-0xf7; read in Ghidra), which
/// `_read_nolock` reads for a UTF-8 file.
const UTF8_TRAIL_BYTES: u32 = 0x011c_2790;

// Translated from 00ec9a5e (disassembled, FalloutNV.exe 1.4.0.525)
/// `_fopen_s` (LIBCMT/fopen.c, engine map), cdecl: (out stream, path, mode). A null out
/// pointer sets `errno` `EINVAL` (0x16), calls the invalid-parameter handler (`00ec7c30` with
/// five zeros) and answers 0x16. Otherwise the stream from `_fsopen(path, mode, _SH_DENYNO
/// 0x80)` is stored; null answers the current `errno`, a stream 0.
pub fn fopen_s(e: &mut Engine, out: Ptr, path: Ptr, mode: Ptr) -> u32 {
    if out.addr() == 0 {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return 0x16;
    }
    let stream = e.call(FSOPEN, &args![path, mode, 0x80u32]).u32();
    e.mem.set_u32(out.addr(), stream);
    if stream == 0 {
        errno_value(e)
    } else {
        0
    }
}

// Translated from 00ecdacc (disassembled, FalloutNV.exe 1.4.0.525)
/// `_feof` (LIBCMT/feoferr.c, engine map), cdecl: the stream. A null stream sets `errno`
/// `EINVAL` (0x16), calls the invalid-parameter handler and answers 0; otherwise the stream's
/// `_IOEOF` bit (`_flag` +0xc & 0x10).
pub fn feof(e: &mut Engine, stream: Ptr) -> u32 {
    if stream.addr() == 0 {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return 0;
    }
    e.mem.u32(stream.addr() + 0xc) & 0x10
}

/// The `ioinfo` entry `fgets` and its kin read for a stream's handle: `__badioinfo` for -1 and
/// -2, else the handle's entry.
fn stream_ioinfo(e: &Engine, handle: u32) -> u32 {
    if handle == 0xffff_ffff || handle == 0xffff_fffe {
        BAD_IOINFO
    } else {
        ioinfo(e, handle)
    }
}

// Translated from 00ecc104 (disassembled, FalloutNV.exe 1.4.0.525)
/// `fgets` (LIBCMT/fgets.c, engine map), cdecl: (buffer, count, stream). A null buffer with a
/// count, a negative count or a null stream sets `errno` `EINVAL` (0x16), calls the
/// invalid-parameter handler and answers null; a count of 0 answers null. Otherwise, with the
/// stream locked (`_lock_file`): a stream that is not a string (`_IOSTRG` 0x40) whose handle
/// (`_fileno`; `__badioinfo` for -1/-2) is in a UTF-8/UTF-16 mode (`ioinfo` +0x24 & 0x7f) or
/// Unicode (+0x24 & 0x80) gives `EINVAL` with the handler and null. Else up to `count - 1`
/// characters are copied (each from the buffer, `_filbuf` when the count +4 goes negative)
/// through the first `\n`, and a terminating zero added; an end or error (-1) before any
/// character answers null without a zero, after some ends the line. The stream is unlocked
/// (the `__finally` block at `00ecc244`, `_unlock_file`). Answers the buffer.
// Translation incomplete: 00ecc104 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated; its __finally block (00ecc244) runs inline.
pub fn fgets(e: &mut Engine, buffer: Ptr, count: i32, stream: Ptr) -> Ptr {
    let buffer = buffer.addr();
    let stream = stream.addr();
    let mut result = buffer;
    if (buffer == 0 && count != 0) || count < 0 || stream == 0 {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return Ptr::new(0);
    }
    if count == 0 {
        return Ptr::new(0);
    }
    lock_file(e, Ptr::new(stream));
    if e.mem.u8(stream + 0xc) & 0x40 == 0 {
        let handle = e.call(FILENO, &args![stream]).u32();
        let refused = e.mem.u8(stream_ioinfo(e, handle) + 0x24) & 0x7f != 0
            || e.mem.u8(stream_ioinfo(e, handle) + 0x24) & 0x80 != 0;
        if refused {
            set_errno(e, 0x16);
            invalid_parameter_zeros(e);
            result = 0;
        }
    }
    if result != 0 {
        let mut at = buffer;
        let mut left = count;
        loop {
            left = left.wrapping_sub(1);
            if left == 0 {
                break;
            }
            let available = e.mem.u32(stream + 4).wrapping_sub(1);
            e.mem.set_u32(stream + 4, available);
            let character = if (available as i32) < 0 {
                e.call(FILBUF, &args![stream]).u32()
            } else {
                let from = e.mem.u32(stream);
                let c = u32::from(e.mem.u8(from));
                e.mem.set_u32(stream, from.wrapping_add(1));
                c
            };
            if character == 0xffff_ffff {
                if at == buffer {
                    result = 0;
                }
                break;
            }
            e.mem.set_u8(at, character as u8);
            at = at.wrapping_add(1);
            if character as u8 == b'\n' {
                break;
            }
        }
        if result != 0 {
            e.mem.set_u8(at, 0);
        }
    }
    unlock_file(e, Ptr::new(stream));
    Ptr::new(result)
}

// Translated from 00ecc5f8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `strtok_s` (LIBCMT/strtok_s.c, engine map), cdecl: (string, delimiters, context). A null
/// context or delimiters, or a null string with a null `*context`, sets `errno` `EINVAL`
/// (0x16), calls the invalid-parameter handler and answers null. Otherwise the delimiters
/// (with their terminating zero) go into a 256-bit map; the scan starts at the string (or
/// `*context` when null), skips delimiters, then runs to the next delimiter, which becomes a
/// zero and the scan goes past it, or to the end. `*context` gets where the scan stopped; the
/// answer is the token's start, or null when the token is empty.
// Translation incomplete: 00ecc5f8 - the /GS stack cookie check (00ec408c) is not translated.
pub fn strtok_s(e: &mut Engine, string: Ptr, delimiters: Ptr, context: Ptr) -> Ptr {
    let context = context.addr();
    let mut delimiter = delimiters.addr();
    if context == 0 || delimiter == 0 || (string.addr() == 0 && e.mem.u32(context) == 0) {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return Ptr::new(0);
    }
    let mut map = [0u8; 32];
    loop {
        let d = e.mem.u8(delimiter);
        map[usize::from(d >> 3)] |= 1 << (d & 7);
        delimiter = delimiter.wrapping_add(1);
        if d == 0 {
            break;
        }
    }
    let in_map = |c: u8| map[usize::from(c >> 3)] & (1 << (c & 7)) != 0;
    let mut at = if string.addr() != 0 {
        string.addr()
    } else {
        e.mem.u32(context)
    };
    loop {
        let c = e.mem.u8(at);
        if !in_map(c) || c == 0 {
            break;
        }
        at = at.wrapping_add(1);
    }
    let start = at;
    while e.mem.u8(at) != 0 {
        if in_map(e.mem.u8(at)) {
            e.mem.set_u8(at, 0);
            at = at.wrapping_add(1);
            break;
        }
        at = at.wrapping_add(1);
    }
    e.mem.set_u32(context, at);
    // MOV EAX,EBX; SUB EAX,EDX; NEG; SBB; AND EBX: the start unless the scan did not move.
    if start != at {
        Ptr::new(start)
    } else {
        Ptr::new(0)
    }
}

// Translated from 00edcf0c (disassembled, FalloutNV.exe 1.4.0.525)
/// `_read` (LIBCMT/read.c, engine map), cdecl: (handle, buffer, count). -2 gives `_doserrno`
/// 0, `errno` `EBADF` (9) and -1; a handle out of range or not open (`osfile` bit 0) the same
/// with the invalid-parameter handler; a count above 0x7fffffff `_doserrno` 0, `EINVAL` and the
/// handler, -1. Otherwise, with the handle locked (`__lock_fhandle`), `_read_nolock` answers
/// when it is still open (else `EBADF`, `_doserrno` 0, -1), and the handle is unlocked (the
/// `__finally` block at `00edcfff`, `_unlock_fhandle`).
// Translation incomplete: 00edcf0c - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated; its __finally block (00edcfff) runs inline.
pub fn read(e: &mut Engine, handle: u32, buffer: Ptr, count: u32) -> i32 {
    if handle == 0xffff_fffe {
        set_doserrno(e, 0);
        set_errno(e, 9);
        return -1;
    }
    if !handle_in_range(e, handle) || e.mem.u8(ioinfo(e, handle) + 4) & 1 == 0 {
        set_doserrno(e, 0);
        set_errno(e, 9);
        invalid_parameter_zeros(e);
        return -1;
    }
    if count > 0x7fff_ffff {
        set_doserrno(e, 0);
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return -1;
    }
    lock_fhandle(e, handle);
    let result = if e.mem.u8(ioinfo(e, handle) + 4) & 1 != 0 {
        e.call(READ_NOLOCK, &args![handle, buffer, count]).i32()
    } else {
        set_errno(e, 9);
        set_doserrno(e, 0);
        -1
    };
    unlock_fhandle(e, handle);
    result
}

/// `_doserrno` 0, `errno` `EINVAL` (0x16), the invalid-parameter handler and -1
/// (`_read_nolock` at `00edca42`).
fn read_refused(e: &mut Engine) -> i32 {
    set_doserrno(e, 0);
    set_errno(e, 0x16);
    invalid_parameter_zeros(e);
    -1
}

/// `ReadFile(the handle's osfhnd, at, count, read, null)`: whether it succeeded and the count
/// stored through `read` (the slot `count_slot`).
fn read_os(e: &mut Engine, handle: u32, at: u32, count: u32, count_slot: u32) -> (u32, u32) {
    let os = e.mem.u32(ioinfo(e, handle));
    let ok = e
        .call(IMPORT_READ_FILE, &args![os, at, count, count_slot, 0u32])
        .u32();
    (ok, e.mem.u32(count_slot))
}

/// `_read_nolock`'s look-ahead read after a CR at the end of the data (`size` bytes into the
/// look-ahead slot): false when it failed with a Windows error (`GetLastError` not 0) or read
/// nothing, else true.
fn read_look_ahead(
    e: &mut Engine,
    handle: u32,
    peek_slot: u32,
    size: u32,
    count_slot: u32,
) -> bool {
    let (ok, _) = read_os(e, handle, peek_slot, size, count_slot);
    if ok == 0 && e.call(IMPORT_GET_LAST_ERROR, &args![]).u32() != 0 {
        return false;
    }
    e.mem.u32(count_slot) != 0
}

// Translated from 00edc94a (disassembled, FalloutNV.exe 1.4.0.525)
/// `_read_nolock` (LIBCMT/read.c, engine map), cdecl: (handle, buffer, count). -2 gives
/// `_doserrno` 0, `EBADF` (9) and -1; a handle out of range the same with the invalid-parameter
/// handler, and one not open too. A count above 0x7fffffff, a null buffer, or an odd count in
/// UTF-8 (mode 1, `ioinfo` +0x24 bits 0-6 sign-extended) or UTF-16 (mode 2) gives `_doserrno`
/// 0, `EINVAL` (0x16), the handler and -1. A count of 0 or a handle at its end (`FEOFLAG` 2)
/// answers 0.
///
/// UTF-8 reads into a `__malloc_crt` buffer of half the count (at least 4; none: `ENOMEM`
/// 0xc, `_doserrno` 8, -1) and records the position (`_lseeki64_nolock(0, SEEK_CUR)`) at
/// `ioinfo` +0x28; the others read straight into the buffer (UTF-16 with the count made even).
/// A pipe or device (`osfile` & 0x48) first gives back the look-ahead bytes kept from the last
/// read (+5, then +0x25 for UTF-8/UTF-16, then +0x26 for UTF-8; 0xa marks none). Then
/// `ReadFile`; a failure, or a count read that is negative or above the count asked, reads
/// `GetLastError`: `ERROR_ACCESS_DENIED` (5) gives `EBADF` and `_doserrno` 5, -1;
/// `ERROR_BROKEN_PIPE` (0x6d) answers 0; anything else is mapped (`_dosmaperr`), -1. A binary
/// handle (`FTEXT` 0x80 clear) answers the bytes read.
///
/// Text: `FCRLF` (4) is set when the read gave something starting with `\n`, else cleared. The
/// data is copied onto itself: Ctrl-Z (0x1a) ends it (a device keeps it, a file gets
/// `FEOFLAG`); CR-LF becomes LF and a lone CR stays; a CR at the end of the data reads one
/// character (UTF-16: one unit) more: none or a failure keeps the CR; on a pipe or device an LF
/// makes LF and anything else keeps the CR and the character as look-ahead (UTF-16: both bytes
/// and 0xa); on a file an LF at the very start makes LF, else the file moves back by what was
/// read (`_lseeki64_nolock(-1 or -2, SEEK_CUR)`) and the CR is dropped before an LF, kept
/// otherwise. The answer is the bytes kept. UTF-8 then cuts an incomplete sequence at the end
/// (lead byte found by the table at `011c2790`, at most 5 bytes back, reading before the buffer
/// as the code does: a byte with no count gives `EILSEQ` 0x2a and -1; an incomplete sequence
/// is kept as look-ahead on a pipe or device, else the file moves back over it) and converts the
/// rest (`MultiByteToWideChar(CP_UTF8, 0, ...)` into the buffer, half the original count in
/// characters; 0 maps `GetLastError` and gives -1); +0x30 records whether the count of
/// characters differs from the bytes, and the answer is twice the characters. A UTF-8 buffer
/// is freed at the end.
pub fn read_nolock(e: &mut Engine, handle: u32, buffer: Ptr, count: u32) -> i32 {
    let buffer = buffer.addr();
    if handle == 0xffff_fffe {
        set_doserrno(e, 0);
        set_errno(e, 9);
        return -1;
    }
    if !handle_in_range(e, handle) {
        set_doserrno(e, 0);
        set_errno(e, 9);
        invalid_parameter_zeros(e);
        return -1;
    }
    let osfile = e.mem.u8(ioinfo(e, handle) + 4);
    if osfile & 1 == 0 {
        set_doserrno(e, 0);
        set_errno(e, 9);
        invalid_parameter_zeros(e);
        return -1;
    }
    if count > 0x7fff_ffff {
        return read_refused(e);
    }
    if count == 0 || osfile & 2 != 0 {
        return 0;
    }
    if buffer == 0 {
        return read_refused(e);
    }
    // MOV AL,[+0x24]; ADD AL,AL; SAR AL,1: bits 0-6 sign-extended.
    let mode = ((e.mem.u8(ioinfo(e, handle) + 0x24) << 1) as i8) >> 1;
    let mut size = count;
    let work = match mode {
        1 => {
            // NOT; TEST AL,1: an odd count is refused.
            if count & 1 != 0 {
                return read_refused(e);
            }
            let half = count >> 1;
            size = 4;
            if half >= 4 {
                size = half;
            }
            let work = e.call(MALLOC_CRT, &args![size]).u32();
            if work == 0 {
                set_errno(e, 0xc);
                set_doserrno(e, 8);
                return -1;
            }
            let position = lseeki64_nolock(e, handle, 0, 0, 1);
            let entry = ioinfo(e, handle);
            e.mem.set_u32(entry + 0x28, position as u32);
            e.mem.set_u32(entry + 0x2c, (position >> 32) as u32);
            work
        }
        2 => {
            // NOT; TEST AL,1: an odd count is refused.
            if count & 1 != 0 {
                return read_refused(e);
            }
            size = count & 0xffff_fffe;
            buffer
        }
        _ => buffer,
    };
    let (total, result) = e.with_stack(8, |e, slots| {
        let count_slot = slots.addr();
        let peek_slot = count_slot + 4;
        read_into(
            e, handle, buffer, count, work, size, mode, count_slot, peek_slot,
        )
    });
    if work != buffer {
        free(e, Ptr::new(work));
    }
    if result == -2 {
        total as i32
    } else {
        result
    }
}

/// `_read_nolock` from the look-ahead bytes on: answers its count (`[EBP-0x10]`) and its
/// result word (`[EBP-0x14]`: -2 for "answer the count", else the answer).
#[allow(clippy::too_many_arguments)]
fn read_into(
    e: &mut Engine,
    handle: u32,
    buffer: u32,
    count: u32,
    work: u32,
    mut size: u32,
    mode: i8,
    count_slot: u32,
    peek_slot: u32,
) -> (u32, i32) {
    let mut total = 0u32;
    let mut at = work;
    let entry = ioinfo(e, handle);
    if e.mem.u8(entry + 4) & 0x48 != 0 {
        let c = e.mem.u8(entry + 5);
        if c != 0xa && size != 0 {
            e.mem.set_u8(at, c);
            at = at.wrapping_add(1);
            size = size.wrapping_sub(1);
            total = 1;
            e.mem.set_u8(ioinfo(e, handle) + 5, 0xa);
            if mode != 0 {
                let c = e.mem.u8(ioinfo(e, handle) + 0x25);
                if c != 0xa && size != 0 {
                    e.mem.set_u8(at, c);
                    at = at.wrapping_add(1);
                    size = size.wrapping_sub(1);
                    total = 2;
                    e.mem.set_u8(ioinfo(e, handle) + 0x25, 0xa);
                    if mode == 1 {
                        let c = e.mem.u8(ioinfo(e, handle) + 0x26);
                        if c != 0xa && size != 0 {
                            e.mem.set_u8(at, c);
                            at = at.wrapping_add(1);
                            size = size.wrapping_sub(1);
                            total = 3;
                            e.mem.set_u8(ioinfo(e, handle) + 0x26, 0xa);
                        }
                    }
                }
            }
        }
    }
    let (ok, got) = read_os(e, handle, at, size, count_slot);
    if ok == 0 || (got as i32) < 0 || got > size {
        let error = e.call(IMPORT_GET_LAST_ERROR, &args![]).u32();
        if error == 5 {
            set_errno(e, 9);
            set_doserrno(e, 5);
            return (total, -1);
        }
        if error == 0x6d {
            return (total, 0);
        }
        dosmaperr(e, error);
        return (total, -1);
    }
    total = total.wrapping_add(got);
    let flags_at = ioinfo(e, handle) + 4;
    if e.mem.u8(flags_at) & 0x80 == 0 {
        return (total, -2);
    }
    if mode == 2 {
        let first_lf = got != 0 && e.mem.u16(work) == 0xa;
        let flags = e.mem.u8(flags_at);
        e.mem
            .set_u8(flags_at, if first_lf { flags | 4 } else { flags & 0xfb });
        let total = read_text_utf16(
            e,
            handle,
            work,
            work.wrapping_add(total),
            count_slot,
            peek_slot,
        );
        return (total, -2);
    }
    let first_lf = got != 0 && e.mem.u8(work) == 0xa;
    let flags = e.mem.u8(flags_at);
    e.mem
        .set_u8(flags_at, if first_lf { flags | 4 } else { flags & 0xfb });
    let end = work.wrapping_add(total);
    let mut from = work;
    let mut to = work;
    while from < end {
        let c = e.mem.u8(from);
        if c == 0x1a {
            let flags_at = ioinfo(e, handle) + 4;
            if e.mem.u8(flags_at) & 0x40 == 0 {
                let flags = e.mem.u8(flags_at) | 2;
                e.mem.set_u8(flags_at, flags);
            } else {
                let c = e.mem.u8(from);
                e.mem.set_u8(to, c);
                to = to.wrapping_add(1);
            }
            break;
        }
        if c != 0xd {
            e.mem.set_u8(to, c);
            to = to.wrapping_add(1);
            from = from.wrapping_add(1);
            continue;
        }
        if from < end.wrapping_sub(1) {
            if e.mem.u8(from.wrapping_add(1)) == 0xa {
                from = from.wrapping_add(2);
                e.mem.set_u8(to, 0xa);
            } else {
                from = from.wrapping_add(1);
                e.mem.set_u8(to, 0xd);
            }
            to = to.wrapping_add(1);
            continue;
        }
        from = from.wrapping_add(1);
        if !read_look_ahead(e, handle, peek_slot, 1, count_slot) {
            e.mem.set_u8(to, 0xd);
            to = to.wrapping_add(1);
            continue;
        }
        let peek = e.mem.u8(peek_slot);
        if e.mem.u8(ioinfo(e, handle) + 4) & 0x48 != 0 {
            if peek == 0xa {
                e.mem.set_u8(to, 0xa);
            } else {
                e.mem.set_u8(to, 0xd);
                e.mem.set_u8(ioinfo(e, handle) + 5, peek);
            }
            to = to.wrapping_add(1);
        } else if to == work && peek == 0xa {
            e.mem.set_u8(to, 0xa);
            to = to.wrapping_add(1);
        } else {
            lseeki64_nolock(e, handle, 0xffff_ffff, 0xffff_ffff, 1);
            if peek != 0xa {
                e.mem.set_u8(to, 0xd);
                to = to.wrapping_add(1);
            }
        }
    }
    let kept = to.wrapping_sub(work);
    if mode != 1 || kept == 0 {
        return (kept, -2);
    }
    // UTF-8: cut an incomplete sequence at the end, then convert.
    let mut last = to.wrapping_sub(1);
    let c = e.mem.u8(last);
    let stop = if (c as i8) >= 0 {
        last.wrapping_add(1)
    } else {
        let mut length = 1i32;
        let mut lead = c;
        while e.mem.u8(UTF8_TRAIL_BYTES + u32::from(lead)) == 0 {
            if length > 4 {
                break;
            }
            if last < work {
                break;
            }
            last = last.wrapping_sub(1);
            lead = e.mem.u8(last);
            length += 1;
        }
        let lead = e.mem.u8(last);
        let trail = i32::from(e.mem.u8(UTF8_TRAIL_BYTES + u32::from(lead)) as i8);
        if trail == 0 {
            set_errno(e, 0x2a);
            return (kept, -1);
        }
        if trail + 1 == length {
            last.wrapping_add(length as u32)
        } else if e.mem.u8(ioinfo(e, handle) + 4) & 0x48 != 0 {
            let mut at = last.wrapping_add(1);
            e.mem.set_u8(ioinfo(e, handle) + 5, lead);
            if length >= 2 {
                let b = e.mem.u8(at);
                e.mem.set_u8(ioinfo(e, handle) + 0x25, b);
                at = at.wrapping_add(1);
            }
            if length == 3 {
                let b = e.mem.u8(at);
                e.mem.set_u8(ioinfo(e, handle) + 0x26, b);
                at = at.wrapping_add(1);
            }
            at.wrapping_sub(length as u32)
        } else {
            // NEG EAX; CDQ: the distance back as a signed 64-bit value.
            let back = (length as u32).wrapping_neg();
            lseeki64_nolock(e, handle, back, ((back as i32) >> 31) as u32, 1);
            last
        }
    };
    let bytes = stop.wrapping_sub(work);
    let characters = e
        .call(
            IMPORT_MULTI_BYTE_TO_WIDE_CHAR,
            &args![0xfde9u32, 0u32, work, bytes, buffer, count >> 1],
        )
        .u32();
    if characters == 0 {
        let error = e.call(IMPORT_GET_LAST_ERROR, &args![]).u32();
        dosmaperr(e, error);
        return (kept, -1);
    }
    let entry = ioinfo(e, handle);
    e.mem.set_u32(entry + 0x30, u32::from(characters != bytes));
    (characters.wrapping_add(characters), -2)
}

/// `_read_nolock`'s text pass for a UTF-16 handle (mode 2), on 16-bit units from `work` to
/// `end` (`00edcdb0` .. `00edcec5`): answers the bytes kept.
fn read_text_utf16(
    e: &mut Engine,
    handle: u32,
    work: u32,
    end: u32,
    count_slot: u32,
    peek_slot: u32,
) -> u32 {
    let mut from = work;
    let mut to = work;
    while from < end {
        let c = e.mem.u16(from);
        if c == 0x1a {
            let flags_at = ioinfo(e, handle) + 4;
            if e.mem.u8(flags_at) & 0x40 == 0 {
                let flags = e.mem.u8(flags_at) | 2;
                e.mem.set_u8(flags_at, flags);
            } else {
                let c = e.mem.u16(from);
                e.mem.set_u16(to, c);
                to = to.wrapping_add(2);
            }
            break;
        }
        if c != 0xd {
            e.mem.set_u16(to, c);
            to = to.wrapping_add(2);
            from = from.wrapping_add(2);
            continue;
        }
        if from < end.wrapping_add(0xffff_fffe) {
            if e.mem.u16(from.wrapping_add(2)) == 0xa {
                from = from.wrapping_add(4);
                e.mem.set_u16(to, 0xa);
            } else {
                from = from.wrapping_add(2);
                e.mem.set_u16(to, 0xd);
            }
            to = to.wrapping_add(2);
            continue;
        }
        from = from.wrapping_add(2);
        if !read_look_ahead(e, handle, peek_slot, 2, count_slot) {
            e.mem.set_u16(to, 0xd);
            to = to.wrapping_add(2);
            continue;
        }
        let peek = e.mem.u16(peek_slot);
        if e.mem.u8(ioinfo(e, handle) + 4) & 0x48 != 0 {
            if peek == 0xa {
                e.mem.set_u16(to, 0xa);
            } else {
                e.mem.set_u16(to, 0xd);
                let low = e.mem.u8(peek_slot);
                e.mem.set_u8(ioinfo(e, handle) + 5, low);
                let high = e.mem.u8(peek_slot + 1);
                e.mem.set_u8(ioinfo(e, handle) + 0x25, high);
                e.mem.set_u8(ioinfo(e, handle) + 0x26, 0xa);
            }
            to = to.wrapping_add(2);
        } else if to == work && peek == 0xa {
            e.mem.set_u16(to, 0xa);
            to = to.wrapping_add(2);
        } else {
            lseeki64_nolock(e, handle, 0xffff_fffe, 0xffff_ffff, 1);
            if peek != 0xa {
                e.mem.set_u16(to, 0xd);
                to = to.wrapping_add(2);
            }
        }
    }
    to.wrapping_sub(work)
}

/// `_flush` (`00ecd8e6`), `_read` (`00edcf0c`), `_lseek_nolock` (`00edd009`), `memset`
/// (`00ec61c0`) and `memcpy` (`00ec44d0`), cdecl.
const FLUSH: u32 = 0x00ec_d8e6;
const READ: u32 = 0x00ed_cf0c;
const LSEEK_NOLOCK: u32 = 0x00ed_d009;
const MEMSET: u32 = 0x00ec_61c0;
const MEMCPY: u32 = 0x00ec_44d0;
/// `ERANGE`.
const ERANGE: u32 = 0x22;

// Translated from 00ec7c66 (disassembled, FalloutNV.exe 1.4.0.525)
/// `memcpy_s` (LIBCMT/memcpy_s.c, engine map), cdecl: (destination, its size, source, count).
/// A count of 0 answers 0. A null destination sets `errno` `EINVAL`, calls the
/// invalid-parameter handler and answers `EINVAL`. With a source and room for the count it is
/// `memcpy` and 0; otherwise the destination is zeroed (`memset` of its size), and then a null
/// source answers `EINVAL` with the handler, a too-small destination `ERANGE` (0x22) with the
/// handler (the branch for a source with room, which the first test already took, answers
/// `EINVAL` without it).
pub fn memcpy_s(e: &mut Engine, destination: Ptr, size: u32, source: Ptr, count: u32) -> u32 {
    if count == 0 {
        return 0;
    }
    if destination.addr() == 0 {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return 0x16;
    }
    if source.addr() != 0 && size >= count {
        e.call(MEMCPY, &args![destination, source, count]);
        return 0;
    }
    e.call(MEMSET, &args![destination, 0u32, size]);
    if source.addr() == 0 {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return 0x16;
    }
    if size >= count {
        return 0x16;
    }
    set_errno(e, ERANGE);
    invalid_parameter_zeros(e);
    ERANGE
}

// Translated from 00edd07e (disassembled, FalloutNV.exe 1.4.0.525)
/// `_lseek` (LIBCMT/lseek.c, engine map), cdecl: (handle, offset, origin). -2 gives
/// `_doserrno` 0, `errno` `EBADF` (9) and -1; a handle out of range or not open (`osfile`
/// bit 0) `_doserrno` 0, `EBADF`, the invalid-parameter handler and -1. Otherwise, with the
/// handle locked (`__lock_fhandle`), `_lseek_nolock` answers when it is still open (else
/// `EBADF`, `_doserrno` 0, -1), and the handle is unlocked (the `__finally` block at
/// `00edd150`, `_unlock_fhandle`).
// Translation incomplete: 00edd07e - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated; its __finally block (00edd150) runs inline.
pub fn lseek(e: &mut Engine, handle: u32, offset: u32, origin: u32) -> u32 {
    if handle == 0xffff_fffe {
        set_doserrno(e, 0);
        set_errno(e, 9);
        return 0xffff_ffff;
    }
    if !handle_in_range(e, handle) || e.mem.u8(ioinfo(e, handle) + 4) & 1 == 0 {
        set_doserrno(e, 0);
        set_errno(e, 9);
        invalid_parameter_zeros(e);
        return 0xffff_ffff;
    }
    lock_fhandle(e, handle);
    let result = if e.mem.u8(ioinfo(e, handle) + 4) & 1 != 0 {
        e.call(LSEEK_NOLOCK, &args![handle, offset, origin]).u32()
    } else {
        set_errno(e, 9);
        set_doserrno(e, 0);
        0xffff_ffff
    };
    unlock_fhandle(e, handle);
    result
}

// Translated from 00ecb3c5 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_ftell_nolock` (LIBCMT/ftell.c, engine map), cdecl: the stream. A null stream sets `errno`
/// `EINVAL`, calls the invalid-parameter handler and answers -1. A negative `_cnt` (+4) is
/// cleared; the handle's position (`_lseek(handle, 0, SEEK_CUR)`) negative answers -1. A stream
/// without a buffer in use (`_flag` & 0x108 clear) answers the position less `_cnt`. Otherwise
/// the offset in the buffer (`_ptr` - `_base`), plus one per `\n` before `_ptr` for a
/// text-mode handle (`osfile` 0x80) of a read or write stream (`_flag` & 3); a stream that is
/// neither and not `_IORW` (0x80) gives `EINVAL` and -1. At position 0 that offset answers. A
/// read stream (`_flag` bit 0) with nothing buffered answers the position; with bytes buffered
/// the position less what the buffer read (`_ptr` - `_base` + `_cnt`), for a text-mode handle
/// counted at the end of the file (`_lseek(handle, 0, SEEK_END)` equal to the position) with a
/// `\n` each and one more for `_IOCTRLZ` (0x2000), else (the position put back with `SEEK_SET`,
/// -1 on failure) 0x200 or the buffer size (`_bufsiz`, when it read more than 0x200 or the
/// stream is not `_IOMYBUF` (8) or is `_IOSETVBUF` (0x400)) and one more for `FCRLF`
/// (`osfile` 4); plus the offset.
pub fn ftell_nolock(e: &mut Engine, stream: Ptr) -> u32 {
    let s = stream.addr();
    if s == 0 {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return 0xffff_ffff;
    }
    let handle = e.call(FILENO, &args![s]).u32();
    if (e.mem.u32(s + 4) as i32) < 0 {
        e.mem.set_u32(s + 4, 0);
    }
    let mut position = lseek(e, handle, 0, 1);
    if (position as i32) < 0 {
        return 0xffff_ffff;
    }
    let flag = e.mem.u32(s + 0xc);
    if flag & 0x108 == 0 {
        return position.wrapping_sub(e.mem.u32(s + 4));
    }
    let ptr = e.mem.u32(s);
    let base = e.mem.u32(s + 8);
    let mut offset = ptr.wrapping_sub(base);
    if flag & 3 != 0 {
        if e.mem.u8(ioinfo(e, handle) + 4) & 0x80 != 0 {
            let mut at = base;
            while at < ptr {
                if e.mem.u8(at) == b'\n' {
                    offset = offset.wrapping_add(1);
                }
                at = at.wrapping_add(1);
            }
        }
    } else if (flag as u8 as i8) >= 0 {
        set_errno(e, 0x16);
        return 0xffff_ffff;
    }
    if position == 0 {
        return offset;
    }
    if e.mem.u8(s + 0xc) & 1 != 0 {
        let count = e.mem.u32(s + 4);
        if count == 0 {
            offset = 0;
        } else {
            let mut read = ptr.wrapping_sub(base).wrapping_add(count);
            if e.mem.u8(ioinfo(e, handle) + 4) & 0x80 != 0 {
                if lseek(e, handle, 0, 2) == position {
                    let start = e.mem.u32(s + 8);
                    let end = read.wrapping_add(start);
                    let mut at = start;
                    while at < end {
                        if e.mem.u8(at) == b'\n' {
                            read = read.wrapping_add(1);
                        }
                        at = at.wrapping_add(1);
                    }
                    if e.mem.u32(s + 0xc) & 0x2000 != 0 {
                        read = read.wrapping_add(1);
                    }
                } else {
                    if (lseek(e, handle, position, 0) as i32) < 0 {
                        return 0xffff_ffff;
                    }
                    let flag = e.mem.u32(s + 0xc);
                    read = if read > 0x200 || flag & 8 == 0 || flag & 0x400 != 0 {
                        e.mem.u32(s + 0x18)
                    } else {
                        0x200
                    };
                    if e.mem.u8(ioinfo(e, handle) + 4) & 4 != 0 {
                        read = read.wrapping_add(1);
                    }
                }
            }
            position = position.wrapping_sub(read);
        }
    }
    offset.wrapping_add(position)
}

// Translated from 00ecb562 (disassembled, FalloutNV.exe 1.4.0.525)
/// `ftell` (LIBCMT/ftell.c, engine map), cdecl: the stream. A null stream sets `errno`
/// `EINVAL`, calls the invalid-parameter handler and answers -1; otherwise `_ftell_nolock`
/// with the stream locked (`_lock_file`; the `__finally` block at `00ecb5c6` unlocks it).
// Translation incomplete: 00ecb562 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated; its __finally block (00ecb5c6) runs inline.
pub fn ftell(e: &mut Engine, stream: Ptr) -> u32 {
    if stream.addr() == 0 {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return 0xffff_ffff;
    }
    lock_file(e, stream);
    let result = ftell_nolock(e, stream);
    unlock_file(e, stream);
    result
}

// Translated from 00ecb5d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_fseek_nolock` (LIBCMT/fseek.c, engine map), cdecl: (stream, offset, origin). A stream
/// that is not read, write or `_IORW` (`_flag` & 0x83) gives `errno` `EINVAL` and -1.
/// Otherwise `_IOEOF` (0x10) is cleared, `SEEK_CUR` becomes `SEEK_SET` from `_ftell_nolock`,
/// the stream is flushed (`_flush`); an `_IORW` stream loses its read/write bits, a read stream
/// with its own buffer (`_IOMYBUF`, not `_IOSETVBUF`) gets `_bufsiz` 0x200; then `_lseek` on
/// its handle: 0, or -1 when that failed.
pub fn fseek_nolock(e: &mut Engine, stream: Ptr, offset: u32, origin: u32) -> u32 {
    let s = stream.addr();
    let flag = e.mem.u32(s + 0xc);
    if flag & 0x83 == 0 {
        set_errno(e, 0x16);
        return 0xffff_ffff;
    }
    e.mem.set_u32(s + 0xc, flag & 0xffff_ffef);
    let mut offset = offset;
    let mut origin = origin;
    if origin == 1 {
        offset = offset.wrapping_add(ftell_nolock(e, stream));
        origin = 0;
    }
    e.call(FLUSH, &args![s]);
    let flag = e.mem.u32(s + 0xc);
    if (flag as u8 as i8) < 0 {
        e.mem.set_u32(s + 0xc, flag & 0xffff_fffc);
    } else if flag & 1 != 0 && flag & 8 != 0 && flag & 0x400 == 0 {
        e.mem.set_u32(s + 0x18, 0x200);
    }
    let handle = e.call(FILENO, &args![s]).u32();
    let position = lseek(e, handle, offset, origin);
    u32::from(position != 0xffff_ffff).wrapping_sub(1)
}

// Translated from 00ecb65a (disassembled, FalloutNV.exe 1.4.0.525)
/// `fseek` (LIBCMT/fseek.c, engine map), cdecl: (stream, offset, origin). A null stream or an
/// origin other than 0, 1 and 2 sets `errno` `EINVAL`, calls the invalid-parameter handler and
/// answers -1; otherwise `_fseek_nolock` with the stream locked (`_lock_file`; the `__finally`
/// block at `00ecb6d5` unlocks it).
// Translation incomplete: 00ecb65a - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated; its __finally block (00ecb6d5) runs inline.
pub fn fseek(e: &mut Engine, stream: Ptr, offset: u32, origin: u32) -> u32 {
    if stream.addr() == 0 || (origin != 0 && origin != 1 && origin != 2) {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return 0xffff_ffff;
    }
    lock_file(e, stream);
    let result = fseek_nolock(e, stream, offset, origin);
    unlock_file(e, stream);
    result
}

// Translated from 00ecb108 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_fread_nolock_s` (LIBCMT/fread.c, engine map), cdecl: (destination, its size, element
/// size, count, stream). A zero element size or count answers 0; a null destination, or a null
/// stream or a count above `0xFFFFFFFF / element size` (the destination zeroed first unless its
/// size is -1), gives `errno` `EINVAL`, the invalid-parameter handler and 0. Otherwise the
/// `element size * count` bytes come from the stream's buffer while it holds some (`memcpy_s`;
/// a negative `_cnt` is an error), else straight from the handle in whole buffer sizes
/// (`_bufsiz` for a buffered stream, else 0x1000; at most 0x7FFFFFFF; `_read`) when at least
/// that many are left, else one byte at a time through `_filbuf`. More than the destination
/// holds zeroes it (unless its size is -1) with `ERANGE`, the handler and 0. An end of file
/// sets `_IOEOF` (0x10), an error `_IOERR` (0x20); both answer the whole elements read. All
/// read answers the count.
pub fn fread_nolock_s(
    e: &mut Engine,
    destination: Ptr,
    size: u32,
    element: u32,
    count: u32,
    stream: Ptr,
) -> u32 {
    let start = destination.addr();
    let s = stream.addr();
    if element == 0 || count == 0 {
        return 0;
    }
    let refuse = |e: &mut Engine| {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        0u32
    };
    if start == 0 {
        return refuse(e);
    }
    if s == 0 || count > 0xffff_ffff / element {
        if size != 0xffff_ffff {
            e.call(MEMSET, &args![start, 0u32, size]);
        }
        if s == 0 || count > 0xffff_ffff / element {
            return refuse(e);
        }
    }
    let too_small = |e: &mut Engine| {
        if size != 0xffff_ffff {
            e.call(MEMSET, &args![start, 0u32, size]);
        }
        set_errno(e, ERANGE);
        invalid_parameter_zeros(e);
        0u32
    };
    let total = element.wrapping_mul(count);
    let mut left = total;
    let mut at = start;
    let mut room = size;
    let mut buffer_size = if e.mem.u32(s + 0xc) & 0x10c != 0 {
        e.mem.u32(s + 0x18)
    } else {
        0x1000
    };
    if total == 0 {
        return count;
    }
    // The whole elements read, after an end of file or an error.
    let partial = |left: u32| total.wrapping_sub(left) / element;
    while left != 0 {
        let buffered = e.mem.u32(s + 4);
        if e.mem.u32(s + 0xc) & 0x10c != 0 && buffered != 0 {
            if (buffered as i32) < 0 {
                let flag = e.mem.u32(s + 0xc) | 0x20;
                e.mem.set_u32(s + 0xc, flag);
                return partial(left);
            }
            let n = if left < buffered { left } else { buffered };
            if n > room {
                return too_small(e);
            }
            let from = e.mem.u32(s);
            e.call(MEMCPY_S, &args![at, room, from, n]);
            e.mem.set_u32(s + 4, e.mem.u32(s + 4).wrapping_sub(n));
            e.mem.set_u32(s, e.mem.u32(s).wrapping_add(n));
            at = at.wrapping_add(n);
            left = left.wrapping_sub(n);
            room = room.wrapping_sub(n);
        } else if left >= buffer_size {
            let n = if buffer_size != 0 {
                if left > 0x7fff_ffff {
                    0x7fff_ffff - 0x7fff_ffff % buffer_size
                } else {
                    left - left % buffer_size
                }
            } else if left > 0x7fff_ffff {
                0x7fff_ffff
            } else {
                left
            };
            if n > room {
                return too_small(e);
            }
            let handle = e.call(FILENO, &args![s]).u32();
            let got = e.call(READ, &args![handle, at, n]).u32();
            if got == 0 {
                let flag = e.mem.u32(s + 0xc) | 0x10;
                e.mem.set_u32(s + 0xc, flag);
                return partial(left);
            }
            if got == 0xffff_ffff {
                let flag = e.mem.u32(s + 0xc) | 0x20;
                e.mem.set_u32(s + 0xc, flag);
                return partial(left);
            }
            at = at.wrapping_add(got);
            left = left.wrapping_sub(got);
            room = room.wrapping_sub(got);
        } else {
            let character = e.call(FILBUF, &args![s]).u32();
            if character == 0xffff_ffff {
                return partial(left);
            }
            if room == 0 {
                return too_small(e);
            }
            e.mem.set_u8(at, character as u8);
            at = at.wrapping_add(1);
            buffer_size = e.mem.u32(s + 0x18);
            left = left.wrapping_sub(1);
            room = room.wrapping_sub(1);
        }
    }
    count
}

/// `memcpy_s` (`00ec7c66`) and `_fread_nolock_s` (`00ecb108`), cdecl.
const MEMCPY_S: u32 = 0x00ec_7c66;
const FREAD_NOLOCK_S: u32 = 0x00ec_b108;

// Translated from 00ecb312 (disassembled, FalloutNV.exe 1.4.0.525)
/// `fread_s` (LIBCMT/fread.c, engine map), cdecl: (destination, its size, element size,
/// count, stream). A zero element size or count answers 0; a null stream zeroes the
/// destination (unless its size is -1) and gives `errno` `EINVAL`, the invalid-parameter
/// handler and 0. Otherwise `_fread_nolock_s` with the stream locked (`_lock_file`; the
/// `__finally` block at `00ecb39e` unlocks it).
// Translation incomplete: 00ecb312 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated; its __finally block (00ecb39e) runs inline.
pub fn fread_s(
    e: &mut Engine,
    destination: Ptr,
    size: u32,
    element: u32,
    count: u32,
    stream: Ptr,
) -> u32 {
    if element == 0 || count == 0 {
        return 0;
    }
    if stream.addr() == 0 {
        if size != 0xffff_ffff {
            e.call(MEMSET, &args![destination, 0u32, size]);
        }
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return 0;
    }
    lock_file(e, stream);
    let result = e
        .call(
            FREAD_NOLOCK_S,
            &args![destination, size, element, count, stream],
        )
        .u32();
    unlock_file(e, stream);
    result
}

// Translated from 00ecb3a8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `fread` (LIBCMT/fread.c, engine map), cdecl: (destination, element size, count, stream):
/// `fread_s` with the destination size -1.
pub fn fread(e: &mut Engine, destination: Ptr, element: u32, count: u32, stream: Ptr) -> u32 {
    fread_s(e, destination, 0xffff_ffff, element, count, stream)
}

// Translated from 00ec9d4a (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec9d4a - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated; its __finally block (00ec9de6, _unlock_file2(1, stdout)) runs inline.
/// `printf` (LIBCMT/printf.c, engine map), cdecl: (format, ...). A null format sets `errno`
/// `EINVAL`, calls the invalid-parameter handler and answers -1. Otherwise `stdout`
/// (`__iob_func() + 0x20`) is locked (`_lock_file2(1, ..)`), given a temporary buffer
/// (`_stbuf`), written with `_output_l(stdout, format, NULL, the arguments)`, the buffer taken
/// back (`_ftbuf`) and unlocked; answers `_output_l`'s count. The variable arguments are copied
/// into game memory as the `va_list`.
pub fn printf(e: &mut Engine, format: Ptr, args: &[u32]) -> i32 {
    if format.addr() == 0 {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return -1;
    }
    let stdout = e.call(IOB_FUNC, &args![]).u32().wrapping_add(0x20);
    lock_file2(e, 1, Ptr::new(stdout));
    let stream = e.call(IOB_FUNC, &args![]).u32().wrapping_add(0x20);
    let flag = e.call(STBUF, &args![stream]).u32();
    let size = 4 * (args.len() as u32).max(1);
    let result = e.with_stack(size, |e, list| {
        for (i, word) in args.iter().enumerate() {
            e.mem.set_u32(list.addr() + 4 * i as u32, *word);
        }
        let stream = e.call(IOB_FUNC, &args![]).u32().wrapping_add(0x20);
        e.call(OUTPUT_L, &args![stream, format, 0u32, list]).i32()
    });
    let stream = e.call(IOB_FUNC, &args![]).u32().wrapping_add(0x20);
    e.call(FTBUF, &args![flag, stream]);
    let stream = e.call(IOB_FUNC, &args![]).u32().wrapping_add(0x20);
    unlock_file2(e, 1, Ptr::new(stream));
    result
}

/// The registry entry of [`printf`]: the format, then the variable arguments.
fn printf_entry(e: &mut Engine, words: &[u32]) -> Ret {
    printf(e, Ptr::new(words[0]), &words[1..]).into_ret()
}

/// `MoveFileA` and `GetLastError` (KERNEL32 import slots `00fdf188`, `00fdf094`) and
/// `_dosmaperr` (`00ec8609`, cdecl: the Windows error).
const IMPORT_MOVE_FILE_A: u32 = 0x00fd_f188;
const IMPORT_GET_LAST_ERROR: u32 = 0x00fd_f094;
const DOSMAPERR: u32 = 0x00ec_8609;

// Translated from 00ec862c (disassembled, FalloutNV.exe 1.4.0.525)
/// `rename` (LIBCMT/rename.c, engine map), cdecl: (old name, new name). `MoveFileA(old, new)`;
/// when it fails, `GetLastError()` is the error, else 0. A nonzero error goes through
/// `_dosmaperr` and answers -1; otherwise 0.
pub fn rename(e: &mut Engine, old: Ptr, new: Ptr) -> i32 {
    let moved = e.call(IMPORT_MOVE_FILE_A, &args![old, new]).u32();
    let error = if moved == 0 {
        e.call(IMPORT_GET_LAST_ERROR, &args![]).u32()
    } else {
        0
    };
    if error != 0 {
        e.call(DOSMAPERR, &args![error]);
        return -1;
    }
    0
}

// Translated from 00ecc244 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `fgets` (`00ecc104`, which runs the same unlock inline): run on
/// `fgets`'s frame with the stream in `ESI` (the one word here), `_unlock_file(stream)`.
pub fn fgets_finally(e: &mut Engine, stream: Ptr) {
    e.call(UNLOCK_FILE, &args![stream]);
}

// Translated from 00ec997b (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `fclose` (`00ec9907`, which runs the same unlock inline): run on
/// `fclose`'s frame with the stream in `ESI` (the one word here), `_unlock_file(stream)`.
pub fn fclose_finally(e: &mut Engine, stream: Ptr) {
    e.call(UNLOCK_FILE, &args![stream]);
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00ec_9d4a, printf_entry as AbiFn),
        entry!(0x00ec_7c66, memcpy_s(Ptr, u32, Ptr, u32) -> u32),
        entry!(0x00ed_d07e, lseek(u32, u32, u32) -> u32),
        entry!(0x00ec_b3c5, ftell_nolock(Ptr) -> u32),
        entry!(0x00ec_b562, ftell(Ptr) -> u32),
        entry!(0x00ec_b5d0, fseek_nolock(Ptr, u32, u32) -> u32),
        entry!(0x00ec_b65a, fseek(Ptr, u32, u32) -> u32),
        entry!(0x00ec_b108, fread_nolock_s(Ptr, u32, u32, u32, Ptr) -> u32),
        entry!(0x00ec_b312, fread_s(Ptr, u32, u32, u32, Ptr) -> u32),
        entry!(0x00ec_b3a8, fread(Ptr, u32, u32, Ptr) -> u32),
        entry!(0x00ec_9a5e, fopen_s(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00ec_dacc, feof(Ptr) -> u32),
        entry!(0x00ec_c104, fgets(Ptr, i32, Ptr) -> Ptr),
        entry!(0x00ec_c5f8, strtok_s(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ed_cf0c, read(u32, Ptr, u32) -> i32),
        entry!(0x00ed_c94a, read_nolock(u32, Ptr, u32) -> i32),
        entry!(0x00ec_862c, rename(Ptr, Ptr) -> i32),
        entry!(0x00ec_c244, fgets_finally(Ptr)),
        entry!(0x00ec_997b, fclose_finally(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The invalid-parameter handler the doubled `__decode_pointer` answers.
    const HANDLER: u32 = 0x0e30_0000;

    /// An engine with the CRT's data pages mapped, a thread block (`errno` +8, `_doserrno`
    /// +0xc), the locks 1, 0xa and 0xb set up, one `ioinfo` block of 32 closed entries, three
    /// busy streams and a free `__piob` slot, `_get_fmode` (in another crate) doubled to the
    /// text default, and the platform's own file, error and critical-section calls at their
    /// import slots. Answers the engine and the thread block.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.map(0x011c_1000, 0x2000);
        e.map(0x0127_0000, 0x2000);
        let block = e.mem.alloc(0x40);
        e.register_double(GETPTD_NOEXIT, move |_, _| block.into_ret());
        e.register(DECODE_POINTER, |_, _| HANDLER.into_ret());
        e.register(HANDLER, |_, _| Ret::default());
        e.mem.set_u32(LOCKS_READY, 1);
        for number in [1u32, 0xa, 0xb] {
            let section = e.mem.alloc(0x18);
            e.mem.set_u32(LOCK_TABLE + 8 * number, section);
        }
        let entries = e.mem.alloc(0x800);
        for index in 0..0x20 {
            e.mem.set_u32(entries + 0x40 * index, 0xffff_ffff);
        }
        e.mem.set_u32(PIOINFO, entries);
        e.mem.set_u32(NHANDLE, 0x20);
        let piob = e.mem.alloc(0x10);
        for index in 0..3 {
            let busy = e.mem.alloc(0x38);
            e.mem.set_u32(busy + 0xc, 1);
            e.mem.set_u32(piob + 4 * index, busy);
        }
        e.mem.set_u32(PIOB, piob);
        e.mem.set_u32(NSTREAM, 4);
        e.register(GET_FMODE, |e, a| {
            e.mem.set_u32(a[0], 0);
            0u32.into_ret()
        });
        let slots = [
            (IMPORT_CREATE_FILE_A, "CreateFileA"),
            (IMPORT_GET_FILE_TYPE, "GetFileType"),
            (IMPORT_SET_FILE_POINTER, "SetFilePointer"),
            (IMPORT_READ_FILE, "ReadFile"),
            (IMPORT_CLOSE_HANDLE, "CloseHandle"),
            (IMPORT_GET_LAST_ERROR, "GetLastError"),
            (IMPORT_ENTER_CRITICAL_SECTION, "EnterCriticalSection"),
            (IMPORT_LEAVE_CRITICAL_SECTION, "LeaveCriticalSection"),
            (
                IMPORT_INIT_CRITICAL_SECTION_SPIN,
                "InitializeCriticalSectionAndSpinCount",
            ),
        ];
        let imports: Vec<crate::exe::Import> = slots
            .iter()
            .map(|&(slot, name)| crate::exe::Import {
                slot,
                dll: "KERNEL32.dll".into(),
                name: name.into(),
            })
            .collect();
        assert_eq!(e.register_imports(&imports), slots.len());
        e.call_log = Some(vec![]);
        (e, block)
    }

    /// `C:\Test\<name>` holding `data` on the run's disk; answers the path's address.
    fn file(e: &mut Engine, name: &str, data: &[u8]) -> u32 {
        let text = format!("C:\\Test\\{name}");
        let path = e.mem.alloc(0x40);
        e.mem.set_cstr(path, text.as_bytes());
        e.kernel
            .disk
            .files
            .insert(text.to_ascii_lowercase().into_bytes(), data.to_vec());
        path
    }

    /// The file opened read-only through `_sopen_s` with the flags (`_O_TEXT` 0x4000 or
    /// `_O_BINARY` 0x8000), `_SH_DENYNO`; answers the CRT handle.
    fn open(e: &mut Engine, name: &str, data: &[u8], flags: u32) -> u32 {
        let path = file(e, name, data);
        let handle = e.mem.alloc(4);
        let r = e
            .call(0x00ee_3908, &args![handle, path, flags, 0x40u32, 0x180u32])
            .u32();
        assert_eq!(r, 0);
        e.mem.u32(handle)
    }

    #[test]
    fn plugins_txt_reads_line_by_line_until_feof() {
        let (mut e, _) = engine();
        let path = file(
            &mut e,
            "Plugins.txt",
            b"# Comment\r\nFalloutNV.esm\r\n\r\nDeadMoney.esm\r\n",
        );
        let mode = e.mem.alloc(4);
        e.mem.set_cstr(mode, b"r");
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00ec_9a5e, &args![out, path, mode]).u32(), 0);
        let stream = e.mem.u32(out);
        assert_ne!(stream, 0);
        // _IOREAD, the handle's ioinfo in text mode (FTEXT | FOPEN).
        assert_eq!(e.mem.u32(stream + 0xc), 1);
        let handle = e.mem.u32(stream + 0x10);
        assert_eq!(e.mem.u8(ioinfo(&e, handle) + 4), 0x81);
        let buffer = e.mem.alloc(0x104);
        let mut lines = vec![];
        // As 00af4550 reads it: while (!feof(f)) fgets(buf, 0x104, f).
        while e.call(0x00ec_dacc, &args![stream]).u32() == 0 {
            let r = e.call(0x00ec_c104, &args![buffer, 0x104u32, stream]).u32();
            lines.push((r, e.mem.cstr(buffer)));
        }
        let expected: Vec<(u32, Vec<u8>)> = vec![
            (buffer, b"# Comment\n".to_vec()),
            (buffer, b"FalloutNV.esm\n".to_vec()),
            (buffer, b"\n".to_vec()),
            (buffer, b"DeadMoney.esm\n".to_vec()),
            // The end comes with the fifth fgets: null, the buffer keeps the last line.
            (0, b"DeadMoney.esm\n".to_vec()),
        ];
        assert_eq!(lines, expected);
        assert_eq!(e.call(0x00ec_dacc, &args![stream]).u32(), 0x10);
        // One read of the whole file into the 0x1000-byte buffer, one that finds the end.
        let reads = calls(&e, IMPORT_READ_FILE);
        assert_eq!(reads.len(), 2);
        assert_eq!(reads[0][2], 0x1000);
        assert_eq!(e.mem.u32(stream + 0x18), 0x1000);
    }

    #[test]
    fn a_last_line_without_newline_sets_feof_with_it() {
        let (mut e, _) = engine();
        let path = file(&mut e, "Short.txt", b"a\r\nlast");
        let mode = e.mem.alloc(4);
        e.mem.set_cstr(mode, b"r");
        let out = e.mem.alloc(4);
        e.call(0x00ec_9a5e, &args![out, path, mode]);
        let stream = e.mem.u32(out);
        let buffer = e.mem.alloc(0x104);
        assert_eq!(
            e.call(0x00ec_c104, &args![buffer, 0x104u32, stream]).u32(),
            buffer
        );
        assert_eq!(e.mem.cstr(buffer), b"a\n".to_vec());
        assert_eq!(e.call(0x00ec_dacc, &args![stream]).u32(), 0);
        assert_eq!(
            e.call(0x00ec_c104, &args![buffer, 0x104u32, stream]).u32(),
            buffer
        );
        assert_eq!(e.mem.cstr(buffer), b"last".to_vec());
        assert_eq!(e.call(0x00ec_dacc, &args![stream]).u32(), 0x10);
    }

    #[test]
    fn fopen_s_and_feof_check_their_arguments() {
        let (mut e, block) = engine();
        assert_eq!(e.call(0x00ec_9a5e, &args![0u32, 1u32, 2u32]).u32(), 0x16);
        assert_eq!(e.mem.u32(block + 8), 0x16);
        assert_eq!(calls(&e, HANDLER).len(), 1);
        // _fsopen failing: its errno is the answer, the out pointer null.
        e.register(FSOPEN, |e, _| {
            let slot = errno(e);
            e.mem.set_u32(slot.addr(), 0x18);
            0u32.into_ret()
        });
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x1234);
        assert_eq!(e.call(0x00ec_9a5e, &args![out, 5u32, 6u32]).u32(), 0x18);
        assert_eq!(e.mem.u32(out), 0);
        assert_eq!(calls(&e, FSOPEN)[0], vec![5, 6, 0x80]);
        assert_eq!(e.call(0x00ec_dacc, &args![0u32]).u32(), 0);
        assert_eq!(calls(&e, HANDLER).len(), 2);
        let stream = e.mem.alloc(0x38);
        e.mem.set_u32(stream + 0xc, 0x19);
        assert_eq!(e.call(0x00ec_dacc, &args![stream]).u32(), 0x10);
    }

    /// A stream on handle 3 (text, ANSI) reading `data` from its buffer; `_filbuf` doubled to
    /// the end (-1).
    fn buffered_stream(e: &mut Engine, data: &[u8]) -> u32 {
        let entries = e.mem.u32(PIOINFO);
        e.mem.set_u32(entries + 0xc0, 0x600);
        e.mem.set_u8(entries + 0xc4, 0x81);
        let stream = e.mem.alloc(0x38);
        let base = e.mem.alloc(0x40);
        e.mem.write(base, data);
        e.mem.set_u32(stream, base);
        e.mem.set_u32(stream + 4, data.len() as u32);
        e.mem.set_u32(stream + 8, base);
        e.mem.set_u32(stream + 0xc, 9);
        e.mem.set_u32(stream + 0x10, 3);
        e.register(FILBUF, |_, _| 0xffff_ffffu32.into_ret());
        stream
    }

    #[test]
    fn fgets_stops_at_newline_count_and_end() {
        let (mut e, _) = engine();
        let stream = buffered_stream(&mut e, b"ab\ncdefg");
        let buffer = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x00ec_c104, &args![buffer, 0x20u32, stream]).u32(),
            buffer
        );
        assert_eq!(e.mem.cstr(buffer), b"ab\n".to_vec());
        // Count 3: two characters and the zero.
        assert_eq!(
            e.call(0x00ec_c104, &args![buffer, 3u32, stream]).u32(),
            buffer
        );
        assert_eq!(e.mem.cstr(buffer), b"cd".to_vec());
        assert_eq!(e.mem.u32(stream + 4), 3);
        // The end after some characters ends the line.
        assert_eq!(
            e.call(0x00ec_c104, &args![buffer, 0x20u32, stream]).u32(),
            buffer
        );
        assert_eq!(e.mem.cstr(buffer), b"efg".to_vec());
        assert_eq!(calls(&e, FILBUF), vec![vec![stream]]);
        // The end first: null, the buffer untouched.
        assert_eq!(
            e.call(0x00ec_c104, &args![buffer, 0x20u32, stream]).u32(),
            0
        );
        assert_eq!(e.mem.cstr(buffer), b"efg".to_vec());
        // Count 1: only the zero.
        assert_eq!(
            e.call(0x00ec_c104, &args![buffer, 1u32, stream]).u32(),
            buffer
        );
        assert_eq!(e.mem.u8(buffer), 0);
        // Each call locked and unlocked the stream (a heap stream: its critical section).
        assert_eq!(
            calls(&e, IMPORT_ENTER_CRITICAL_SECTION),
            vec![vec![stream + 0x20]; 5]
        );
        assert_eq!(
            calls(&e, IMPORT_LEAVE_CRITICAL_SECTION),
            vec![vec![stream + 0x20]; 5]
        );
    }

    #[test]
    fn fgets_refuses_bad_arguments_and_unicode_handles() {
        let (mut e, block) = engine();
        let stream = buffered_stream(&mut e, b"ab\n");
        let buffer = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00ec_c104, &args![buffer, 0u32, stream]).u32(), 0);
        assert!(calls(&e, IMPORT_ENTER_CRITICAL_SECTION).is_empty());
        assert!(calls(&e, HANDLER).is_empty());
        for a in [
            args![buffer, 0xffff_ffffu32, stream],
            args![0u32, 4u32, stream],
            args![buffer, 4u32, 0u32],
        ] {
            assert_eq!(e.call(0x00ec_c104, &a).u32(), 0);
        }
        assert_eq!(calls(&e, HANDLER).len(), 3);
        assert_eq!(e.mem.u32(block + 8), 0x16);
        // A null buffer with count 0 is just null.
        assert_eq!(e.call(0x00ec_c104, &args![0u32, 0u32, stream]).u32(), 0);
        assert_eq!(calls(&e, HANDLER).len(), 3);
        // A UTF-8 or Unicode handle: EINVAL, nothing read, unlocked.
        let entries = e.mem.u32(PIOINFO);
        for modes in [1u8, 0x80] {
            e.mem.set_u8(entries + 0xc0 + 0x24, modes);
            assert_eq!(e.call(0x00ec_c104, &args![buffer, 4u32, stream]).u32(), 0);
            assert_eq!(e.mem.u32(stream + 4), 3);
        }
        assert_eq!(calls(&e, HANDLER).len(), 5);
        // A string stream (_IOSTRG) is not checked.
        e.mem.set_u32(stream + 0xc, 0x49);
        assert_eq!(
            e.call(0x00ec_c104, &args![buffer, 4u32, stream]).u32(),
            buffer
        );
        assert_eq!(e.mem.cstr(buffer), b"ab\n".to_vec());
        assert_eq!(
            calls(&e, IMPORT_ENTER_CRITICAL_SECTION).len(),
            calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len()
        );
    }

    #[test]
    fn strtok_s_splits_on_delimiters() {
        let (mut e, block) = engine();
        let text = e.mem.alloc(0x20);
        e.mem.set_cstr(text, b"a.bsa, b.bsa,,c");
        let comma = e.mem.alloc(4);
        e.mem.set_cstr(comma, b",");
        let context = e.mem.alloc(4);
        let mut tokens = vec![];
        let mut r = e.call(0x00ec_c5f8, &args![text, comma, context]).u32();
        while r != 0 {
            tokens.push(e.mem.cstr(r));
            r = e.call(0x00ec_c5f8, &args![0u32, comma, context]).u32();
        }
        assert_eq!(
            tokens,
            vec![b"a.bsa".to_vec(), b" b.bsa".to_vec(), b"c".to_vec()]
        );
        // The context is left at the end; the separators became zeros.
        assert_eq!(e.mem.u32(context), text + 15);
        assert_eq!(e.mem.u8(text + 5), 0);
        assert_eq!(e.mem.u8(text + 12), 0);
        // Only delimiters: null, the context past them.
        e.mem.set_cstr(text, b",,");
        assert_eq!(e.call(0x00ec_c5f8, &args![text, comma, context]).u32(), 0);
        assert_eq!(e.mem.u32(context), text + 2);
        // A null context, delimiters, or string with a null *context: EINVAL.
        e.mem.set_u32(context, 0);
        for a in [
            args![text, comma, 0u32],
            args![text, 0u32, context],
            args![0u32, comma, context],
        ] {
            assert_eq!(e.call(0x00ec_c5f8, &a).u32(), 0);
        }
        assert_eq!(calls(&e, HANDLER).len(), 3);
        assert_eq!(e.mem.u32(block + 8), 0x16);
    }

    #[test]
    fn read_joins_cr_lf_across_the_end_of_a_read() {
        let (mut e, _) = engine();
        let h = open(&mut e, "a.txt", b"ab\r\ncd", 0x4000);
        let buffer = e.mem.alloc(0x10);
        // The CR is the last byte read: the look-ahead finds LF, the file moves back one and
        // the CR is dropped.
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 3u32]).i32(), 2);
        assert_eq!(e.mem.bytes(buffer, 2), b"ab".to_vec());
        let seeks = calls(&e, IMPORT_SET_FILE_POINTER);
        assert_eq!(seeks.last().unwrap()[1], 0xffff_ffff);
        assert_eq!(seeks.last().unwrap()[3], 1);
        // The LF starts the next read: FCRLF.
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 3);
        assert_eq!(e.mem.bytes(buffer, 3), b"\ncd".to_vec());
        assert_eq!(e.mem.u8(ioinfo(&e, h) + 4) & 4, 4);
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 0);
        assert_eq!(e.mem.u8(ioinfo(&e, h) + 4) & 4, 0);
    }

    #[test]
    fn read_keeps_lone_crs_and_stops_at_ctrl_z() {
        let (mut e, _) = engine();
        let buffer = e.mem.alloc(0x10);
        // A CR alone at the start with an LF after it: LF, nothing moved back.
        let h = open(&mut e, "b.txt", b"\r\nx", 0x4000);
        let seeks = calls(&e, IMPORT_SET_FILE_POINTER).len();
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 1u32]).i32(), 1);
        assert_eq!(e.mem.u8(buffer), b'\n');
        assert_eq!(calls(&e, IMPORT_SET_FILE_POINTER).len(), seeks);
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 4u32]).i32(), 1);
        assert_eq!(e.mem.u8(buffer), b'x');
        // A CR before another character, and a CR at the very end of the file, stay.
        let h = open(&mut e, "c.txt", b"a\rb\r", 0x4000);
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 4);
        assert_eq!(e.mem.bytes(buffer, 4), b"a\rb\r".to_vec());
        // Ctrl-Z ends a text file for good.
        let h = open(&mut e, "d.txt", b"ab\x1acd", 0x4000);
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 2);
        assert_eq!(e.mem.u8(ioinfo(&e, h) + 4) & 2, 2);
        let reads = calls(&e, IMPORT_READ_FILE).len();
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 0);
        assert_eq!(calls(&e, IMPORT_READ_FILE).len(), reads);
        // Binary: as read.
        let h = open(&mut e, "e.bin", b"a\r\n\x1a", 0x8000);
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 4);
        assert_eq!(e.mem.bytes(buffer, 4), b"a\r\n\x1a".to_vec());
    }

    #[test]
    fn read_on_a_pipe_keeps_the_look_ahead_byte() {
        let (mut e, _) = engine();
        let buffer = e.mem.alloc(0x10);
        let h = open(&mut e, "pipe.txt", b"ab\rx", 0x4000);
        // FPIPE.
        let flags = e.mem.u8(ioinfo(&e, h) + 4) | 8;
        e.mem.set_u8(ioinfo(&e, h) + 4, flags);
        e.mem.set_u8(ioinfo(&e, h) + 5, 0xa);
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 3u32]).i32(), 3);
        assert_eq!(e.mem.bytes(buffer, 3), b"ab\r".to_vec());
        assert_eq!(e.mem.u8(ioinfo(&e, h) + 5), b'x');
        // The kept byte comes first in the next read.
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 4u32]).i32(), 1);
        assert_eq!(e.mem.u8(buffer), b'x');
        assert_eq!(e.mem.u8(ioinfo(&e, h) + 5), 0xa);
    }

    #[test]
    fn read_utf16_joins_cr_lf_units() {
        let (mut e, _) = engine();
        let buffer = e.mem.alloc(0x10);
        let h = open(&mut e, "w.txt", b"a\0\r\0\n\0b\0c\0\r\0\n\0", 0x4000);
        e.mem.set_u8(ioinfo(&e, h) + 0x24, 2);
        // Odd counts are refused.
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 7u32]).i32(), -1);
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 12u32]).i32(), 8);
        assert_eq!(e.mem.bytes(buffer, 8), b"a\0\n\0b\0c\0".to_vec());
        let seeks = calls(&e, IMPORT_SET_FILE_POINTER);
        assert_eq!(seeks.last().unwrap()[1], 0xffff_fffe);
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 12u32]).i32(), 2);
        assert_eq!(e.mem.bytes(buffer, 2), b"\n\0".to_vec());
    }

    #[test]
    fn read_utf8_moves_back_over_a_split_sequence() {
        let (mut e, _) = engine();
        for lead in 0xc0u32..0xf8 {
            let trail = if lead < 0xe0 {
                1
            } else if lead < 0xf0 {
                2
            } else {
                3
            };
            e.mem.set_u8(UTF8_TRAIL_BYTES + lead, trail);
        }
        e.register(IMPORT_MULTI_BYTE_TO_WIDE_CHAR, |_, a| {
            (if a[3] == 3 { 3u32 } else { 1 }).into_ret()
        });
        let buffer = e.mem.alloc(0x10);
        let h = open(&mut e, "u.txt", b"abc\xc3\xa9", 0x4000);
        e.mem.set_u8(ioinfo(&e, h) + 0x24, 1);
        // Count 8: a buffer of 4 bytes, "abc" and the lead of a two-byte sequence.
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 6);
        assert_eq!(calls(&e, IMPORT_READ_FILE)[0][2], 4);
        let converted = calls(&e, IMPORT_MULTI_BYTE_TO_WIDE_CHAR);
        assert_eq!(converted[0][..4], [0xfde9, 0, converted[0][2], 3]);
        assert_eq!(converted[0][4..], [buffer, 4]);
        assert_eq!(e.mem.u32(ioinfo(&e, h) + 0x30), 0);
        assert_eq!(e.mem.u32(ioinfo(&e, h) + 0x28), 0);
        // The file moved back one: the whole sequence comes next.
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 8u32]).i32(), 2);
        assert_eq!(calls(&e, IMPORT_MULTI_BYTE_TO_WIDE_CHAR)[1][3], 2);
        assert_eq!(e.mem.u32(ioinfo(&e, h) + 0x30), 1);
        assert_eq!(e.mem.u32(ioinfo(&e, h) + 0x28), 3);
        // Odd counts are refused.
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 7u32]).i32(), -1);
    }

    #[test]
    fn read_reports_bad_handles_and_failed_reads() {
        let (mut e, block) = engine();
        let buffer = e.mem.alloc(0x10);
        assert_eq!(
            e.call(0x00ed_cf0c, &args![0xffff_fffeu32, buffer, 1u32])
                .i32(),
            -1
        );
        assert_eq!(e.mem.u32(block + 8), 9);
        assert!(calls(&e, HANDLER).is_empty());
        // Out of range, not open, too large a count.
        assert_eq!(e.call(0x00ed_cf0c, &args![0x20u32, buffer, 1u32]).i32(), -1);
        assert_eq!(e.call(0x00ed_cf0c, &args![2u32, buffer, 1u32]).i32(), -1);
        assert_eq!(e.mem.u32(block + 8), 9);
        let h = open(&mut e, "f.txt", b"abc", 0x4000);
        assert_eq!(
            e.call(0x00ed_cf0c, &args![h, buffer, 0x8000_0000u32]).i32(),
            -1
        );
        assert_eq!(e.mem.u32(block + 8), 0x16);
        assert_eq!(calls(&e, HANDLER).len(), 3);
        // A count of 0 answers 0; a null buffer is EINVAL.
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 0u32]).i32(), 0);
        assert_eq!(e.call(0x00ed_cf0c, &args![h, 0u32, 4u32]).i32(), -1);
        assert_eq!(calls(&e, HANDLER).len(), 4);
        // ReadFile failing: a broken pipe reads as the end, access denied is EBADF, the
        // rest is mapped.
        e.register(IMPORT_READ_FILE, |_, _| 0u32.into_ret());
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 0x6du32.into_ret());
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 4u32]).i32(), 0);
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 5u32.into_ret());
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 4u32]).i32(), -1);
        assert_eq!(e.mem.u32(block + 8), 9);
        assert_eq!(e.mem.u32(block + 0xc), 5);
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 2u32.into_ret());
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 4u32]).i32(), -1);
        assert_eq!(e.mem.u32(block + 0xc), 2);
        // A count read above the count asked is a failure too.
        e.register(IMPORT_READ_FILE, |e, a| {
            e.mem.set_u32(a[3], a[2] + 1);
            1u32.into_ret()
        });
        assert_eq!(e.call(0x00ed_cf0c, &args![h, buffer, 4u32]).i32(), -1);
        // Every read took and left the handle's lock.
        assert_eq!(
            calls(&e, IMPORT_ENTER_CRITICAL_SECTION).len(),
            calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len()
        );
    }

    /// A binary stream (`rb`) over `data`: answers the stream.
    fn open_binary(e: &mut Engine, data: &[u8]) -> u32 {
        let path = file(e, "Archive.bsa", data);
        let mode = e.mem.alloc(4);
        e.mem.set_cstr(mode, b"rb");
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00ec_9a5e, &args![out, path, mode]).u32(), 0);
        e.mem.u32(out)
    }

    #[test]
    fn fread_fseek_and_ftell_walk_a_binary_file() {
        let (mut e, _) = engine();
        let data: Vec<u8> = (0..0x3000u32).map(|i| (i * 7) as u8).collect();
        let stream = open_binary(&mut e, &data);
        let buffer = e.mem.alloc(0x2000);
        // The size as BSFile asks for it: SEEK_END then ftell, and back.
        assert_eq!(e.call(0x00ec_b562, &args![stream]).u32(), 0);
        assert_eq!(e.call(0x00ec_b65a, &args![stream, 0u32, 2u32]).u32(), 0);
        assert_eq!(e.call(0x00ec_b562, &args![stream]).u32(), 0x3000);
        assert_eq!(e.call(0x00ec_b65a, &args![stream, 0u32, 0u32]).u32(), 0);
        // A small read goes through the stream's buffer (0x1000 bytes).
        assert_eq!(
            e.call(0x00ec_b3a8, &args![buffer, 1u32, 0x24u32, stream])
                .u32(),
            0x24
        );
        assert_eq!(e.mem.bytes(buffer, 0x24), data[..0x24].to_vec());
        assert_eq!(e.call(0x00ec_b562, &args![stream]).u32(), 0x24);
        // SEEK_CUR from there, then a read larger than the buffer goes to the handle.
        assert_eq!(e.call(0x00ec_b65a, &args![stream, 0x10u32, 1u32]).u32(), 0);
        assert_eq!(e.call(0x00ec_b562, &args![stream]).u32(), 0x34);
        assert_eq!(
            e.call(0x00ec_b3a8, &args![buffer, 0x10u32, 0x180u32, stream])
                .u32(),
            0x180
        );
        assert_eq!(e.mem.bytes(buffer, 0x1800), data[0x34..0x1834].to_vec());
        assert_eq!(e.call(0x00ec_b562, &args![stream]).u32(), 0x1834);
        // Past the end: the whole elements read, and _IOEOF.
        assert_eq!(
            e.call(0x00ec_b3a8, &args![buffer, 0x100u32, 0x20u32, stream])
                .u32(),
            0x17
        );
        assert_ne!(e.mem.u32(stream + 0xc) & 0x10, 0);
        // A bad origin and a null stream are refused.
        assert_eq!(
            e.call(0x00ec_b65a, &args![stream, 0u32, 3u32]).u32(),
            0xffff_ffff
        );
        assert_eq!(e.call(0x00ec_b562, &args![0u32]).u32(), 0xffff_ffff);
        assert_eq!(
            calls(&e, IMPORT_ENTER_CRITICAL_SECTION).len(),
            calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len()
        );
    }

    #[test]
    fn fread_s_refuses_a_destination_too_small() {
        let (mut e, block) = engine();
        let stream = open_binary(&mut e, &[1, 2, 3, 4, 5, 6, 7, 8]);
        let buffer = e.mem.alloc(0x10);
        e.mem.write(buffer, &[0xaa; 8]);
        assert_eq!(
            e.call(0x00ec_b312, &args![buffer, 4u32, 1u32, 8u32, stream])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(block + 8), 0x22);
        assert_eq!(e.mem.bytes(buffer, 8), [0, 0, 0, 0, 0xaa, 0xaa, 0xaa, 0xaa]);
        // Zero elements read nothing; a null stream zeroes the destination.
        assert_eq!(
            e.call(0x00ec_b312, &args![buffer, 8u32, 0u32, 8u32, stream])
                .u32(),
            0
        );
        e.mem.write(buffer, &[0xaa; 8]);
        assert_eq!(
            e.call(0x00ec_b312, &args![buffer, 8u32, 1u32, 8u32, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.mem.bytes(buffer, 8), [0; 8]);
        assert_eq!(e.mem.u32(block + 8), 0x16);
    }

    #[test]
    fn memcpy_s_copies_or_zeroes_with_the_error() {
        let (mut e, block) = engine();
        let to = e.mem.alloc(8);
        let from = e.mem.alloc(8);
        e.mem.write(from, b"abcdefgh");
        assert_eq!(e.call(0x00ec_7c66, &args![to, 8u32, from, 0u32]).u32(), 0);
        assert_eq!(e.call(0x00ec_7c66, &args![to, 8u32, from, 4u32]).u32(), 0);
        assert_eq!(e.mem.bytes(to, 4), b"abcd".to_vec());
        assert_eq!(
            e.call(0x00ec_7c66, &args![to, 2u32, from, 4u32]).u32(),
            0x22
        );
        assert_eq!(e.mem.bytes(to, 4), [0, 0, b'c', b'd']);
        assert_eq!(e.mem.u32(block + 8), 0x22);
        assert_eq!(
            e.call(0x00ec_7c66, &args![to, 8u32, 0u32, 4u32]).u32(),
            0x16
        );
        assert_eq!(e.mem.bytes(to, 4), [0; 4]);
        assert_eq!(
            e.call(0x00ec_7c66, &args![0u32, 8u32, from, 4u32]).u32(),
            0x16
        );
    }

    #[test]
    fn printf_writes_stdout_through_output_l_with_its_arguments() {
        let (mut e, block) = engine();
        let section = e.mem.alloc(0x18);
        e.mem.set_u32(LOCK_TABLE + 8 * 0x11, section);
        let iob = e.mem.alloc(0x60);
        e.register_double(IOB_FUNC, move |_, _| iob.into_ret());
        e.register(STBUF, |_, _| 1u32.into_ret());
        e.register(FTBUF, |_, _| Ret::default());
        e.register(OUTPUT_L, |e, a| {
            // The va_list holds the arguments: answer the first one.
            e.mem.u32(a[3]).into_ret()
        });
        let format = e.mem.alloc(8);
        e.mem.set_cstr(format, b"%i\n");
        assert_eq!(e.call(0x00ec_9d4a, &args![format, 0x2au32]).i32(), 0x2a);
        assert_eq!(calls(&e, OUTPUT_L)[0][..3], [iob + 0x20, format, 0]);
        assert_eq!(calls(&e, FTBUF), [vec![1, iob + 0x20]]);
        // The stream was unlocked again.
        assert_eq!(e.mem.u32(iob + 0x20 + 0xc) & 0x8000, 0);
        assert_eq!(e.call(0x00ec_9d4a, &args![0u32]).i32(), -1);
        assert_eq!(e.mem.u32(block + 8), 0x16);
    }

    #[test]
    fn rename_moves_the_file_or_maps_the_windows_error() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let moved = std::rc::Rc::new(std::cell::Cell::new(1u32));
        let answer = moved.clone();
        e.register_double(IMPORT_MOVE_FILE_A, move |_, _| answer.get().into_ret());
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 5u32.into_ret());
        e.register(DOSMAPERR, |_, _| Ret::default());
        let (old, new) = (e.mem.alloc(8), e.mem.alloc(8));
        assert_eq!(e.call(0x00ec_862c, &args![old, new]).i32(), 0);
        assert_eq!(calls(&e, IMPORT_MOVE_FILE_A), vec![vec![old, new]]);
        assert!(calls(&e, IMPORT_GET_LAST_ERROR).is_empty());
        assert!(calls(&e, DOSMAPERR).is_empty());
        moved.set(0);
        assert_eq!(e.call(0x00ec_862c, &args![old, new]).i32(), -1);
        assert_eq!(calls(&e, DOSMAPERR), vec![vec![5]]);
        // A failure with no error code answers 0 without `_dosmaperr`.
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ec_862c, &args![old, new]).i32(), 0);
        assert_eq!(calls(&e, DOSMAPERR).len(), 1);
    }

    #[test]
    fn the_fgets_and_fclose_finally_blocks_unlock_the_stream() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(UNLOCK_FILE, |_, _| Ret::default());
        e.call(0x00ec_c244, &args![0x0123_0000u32]);
        e.call(0x00ec_997b, &args![0x0456_0000u32]);
        assert_eq!(
            calls(&e, UNLOCK_FILE),
            vec![vec![0x0123_0000], vec![0x0456_0000]]
        );
    }
}

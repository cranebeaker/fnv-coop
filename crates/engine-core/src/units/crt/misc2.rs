//! Smaller C runtime pieces the translated code reaches (MSVC 2008 LIBCMT, statically linked
//! into FalloutNV.exe 1.4.0.525), translated from the exe: `std::exception`'s destructor
//! (`00ec79a1`), `__alldiv` (`00ecc250`), the flushing family (`_fflush_nolock` `00ecd94e`,
//! `flsall` `00ecd996`, `fflush` `00ecda70`, `_flushall` `00ecdac3`, `_commit` `00edf1c1`),
//! `wcslen` (`00ecdb00`), the error texts (`_get_sys_err_msg` `00ecdb1a`, `strerror`
//! `00ecdb42`, `__sys_nerr` `00edf2a2`, `__sys_errlist` `00edf2a8`), `___get_sigabrt`
//! (`00eddfc1`), `_statfp`/`_clrfp` (`00ed5a9a`, `00ed5aaa`), the thin cdecl wrappers
//! that pass a null locale (`_wcstombs`, `_mbsnbcmp`, `_mbsnbicmp`, `_mbsnbcpy`,
//! `_wcstombs_s`, `00ecdf2d`, and the game's own `0071cfd0`/`0071cff0`), and the
//! `__finally` blocks (unlocks) of the functions around them.

use super::*;

/// `free` (`00ecd291`), `errno` (`00ec85e3`), `__doserrno` (`00ec85f6`), the
/// invalid-parameter report (`00ec7c30`), `__invoke_watson` (`00ec7b08`), `_lock`
/// (`00eddf39`), `_unlock` (`00edde5f`), `__decode_pointer` (`00ed0f92`),
/// `_getptd_noexit` (`00ed1165`), `__calloc_crt` (`00ed0d24`), `strcpy_s` (`00ec65a6`).
const FREE_FN: u32 = 0x00ec_d291;
const ERRNO: u32 = 0x00ec_85e3;
const DOSERRNO: u32 = 0x00ec_85f6;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
const INVOKE_WATSON: u32 = 0x00ec_7b08;
const LOCK: u32 = 0x00ed_df39;
const UNLOCK: u32 = 0x00ed_de5f;
const DECODE_POINTER: u32 = 0x00ed_0f92;
const GETPTD_NOEXIT: u32 = 0x00ed_1165;
const CALLOC_CRT: u32 = 0x00ed_0d24;
const STRCPY_S: u32 = 0x00ec_65a6;
/// The stream helpers: `_flush` (`00ecd8e6`), `_fileno` (`00ed806c`), `_lock_file`
/// (`00ece01b`), `_lock_file2` (`00ece05c`), `__unlock_file` (`00ece08e`),
/// `_unlock_file2` (`00ece0ca`), `__iob_func` (`00ecdf44`); the handle helpers
/// `___lock_fhandle` (`00ee2ebc`), `__get_osfhandle` (`00ee2e45`), `__unlock_fhandle`
/// (`00ee2f5c`).
const FLUSH: u32 = 0x00ec_d8e6;
const FILENO: u32 = 0x00ed_806c;
const LOCK_FILE: u32 = 0x00ec_e01b;
const LOCK_FILE2: u32 = 0x00ec_e05c;
const UNLOCK_FILE: u32 = 0x00ec_e08e;
const UNLOCK_FILE2: u32 = 0x00ec_e0ca;
const IOB_FUNC: u32 = 0x00ec_df44;
const LOCK_FHANDLE: u32 = 0x00ee_2ebc;
const GET_OSFHANDLE: u32 = 0x00ee_2e45;
const UNLOCK_FHANDLE: u32 = 0x00ee_2f5c;
/// The functions of this file that call each other.
const FFLUSH_NOLOCK: u32 = 0x00ec_d94e;
const FLSALL: u32 = 0x00ec_d996;
const FLSALL_FINALLY_STREAM: u32 = 0x00ec_da38;
const FLSALL_FINALLY: u32 = 0x00ec_da67;
const FFLUSH_FINALLY: u32 = 0x00ec_dab9;
const COMMIT: u32 = 0x00ed_f1c1;
const COMMIT_FINALLY: u32 = 0x00ed_f298;
const GET_SYS_ERR_MSG: u32 = 0x00ec_db1a;
const SYS_NERR: u32 = 0x00ed_f2a2;
const SYS_ERRLIST: u32 = 0x00ed_f2a8;
/// The callees of the null-locale wrappers: `__wcstombs_l_helper` (`00ecaa70`),
/// `__mbsnbcmp_l` (`00ecb6df`), `__mbsnbicmp_l` (`00ecb863`), `_mbsnbcpy_l`
/// (`00eca820`), `__wcstombs_s_l` (`00ecaccc`), `00ecde0e`, and the `_mbsnbcmp` and
/// `_mbsnbicmp` the game's wrappers call (`00ecb849`, `00ecba8e`).
const WCSTOMBS_L_HELPER: u32 = 0x00ec_aa70;
const MBSNBCMP_L: u32 = 0x00ec_b6df;
const MBSNBICMP_L: u32 = 0x00ec_b863;
const MBSNBCPY_L: u32 = 0x00ec_a820;
const WCSTOMBS_S_L: u32 = 0x00ec_accc;
const FN_00ECDE0E: u32 = 0x00ec_de0e;
const MBSNBCMP: u32 = 0x00ec_b849;
const MBSNBICMP: u32 = 0x00ec_ba8e;
/// `std::exception`'s vtable (`010f1fa4`).
const EXCEPTION_VTABLE: u32 = 0x010f_1fa4;
/// The stream table (`__piob`, `01270a70`) and its length (`_nstream`, `01271a80`); the
/// handle table (`__pioinfo`, `01270960`, 32 0x40-byte entries per block) and its length
/// (`_nhandle`, `0127094c`).
const PIOB: u32 = 0x0127_0a70;
const NSTREAM: u32 = 0x0127_1a80;
const PIOINFO: u32 = 0x0127_0960;
const NHANDLE: u32 = 0x0127_094c;
/// The import slots of KERNEL32 `FlushFileBuffers` (`00fdf138`) and `GetLastError`
/// (`00fdf094`).
const FLUSH_FILE_BUFFERS: u32 = 0x00fd_f138;
const GET_LAST_ERROR: u32 = 0x00fd_f094;
/// `strerror`'s fallback text (`010f2060`, the "out of memory" message), `_sys_nerr`
/// (`011c2a78`), `_sys_errlist` (`011c29c8`) and the encoded `SIGABRT` handler
/// (`01270718`).
const STRERROR_FALLBACK: u32 = 0x010f_2060;
const SYS_NERR_WORD: u32 = 0x011c_2a78;
const SYS_ERRLIST_TABLE: u32 = 0x011c_29c8;
const SIGABRT_HANDLER: u32 = 0x0127_0718;

// Translated from 00ec79a1 (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::exception::~exception` (Ghidra's function ID), thiscall: the vtable becomes
/// `std::exception`'s (`010f1fa4`) and, when the "owns its text" word (+0x8) is set, the
/// text (+0x4) is freed.
pub fn exception_destructor(e: &mut Engine, this: u32) {
    let owns = e.mem.u32(this.wrapping_add(8));
    e.mem.set_u32(this, EXCEPTION_VTABLE);
    if owns != 0 {
        let text = e.mem.u32(this.wrapping_add(4));
        e.call(FREE_FN, &args![text]);
    }
}

// Translated from 00ecc250 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__alldiv` (Ghidra's function ID): signed 64-bit division, two 64-bit stack arguments
/// (`RET 0x10`). Both are made positive (counting the signs), divided as `__aulldiv` divides
/// (a divisor below 2^32 in two `DIV`s, a larger one by shifting both until it fits and
/// correcting the estimate down by one), and the quotient negated for one negative operand:
/// the truncating quotient in `EDX:EAX`. A zero divisor faults (`DIV`), as here.
pub fn alldiv(_e: &mut Engine, dividend: i64, divisor: i64) -> i64 {
    dividend.wrapping_div(divisor)
}

// Translated from 00ecd94e (disassembled, FalloutNV.exe 1.4.0.525)
/// `_fflush_nolock` (Ghidra's function ID), cdecl, a stream. Null flushes every stream
/// (`flsall(0)`). Otherwise `_flush` failing answers -1; a stream opened for commit (flag
/// 0x4000) is committed (`_commit(_fileno(stream))`, failing -1), else 0.
pub fn fflush_nolock(e: &mut Engine, stream: u32) -> u32 {
    if stream == 0 {
        return e.call(FLSALL, &args![0u32]).u32();
    }
    if e.call(FLUSH, &args![stream]).u32() != 0 {
        return 0xffff_ffff;
    }
    if e.mem.u32(stream.wrapping_add(0xc)) & 0x4000 != 0 {
        let handle = e.call(FILENO, &args![stream]).u32();
        let result = e.call(COMMIT, &args![handle]).u32();
        return if result != 0 { 0xffff_ffff } else { 0 };
    }
    0
}

// Translated from 00ecd996 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecd996 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 0117ffd0) is not translated; its __finally blocks (00ecda38, 00ecda67) are run as the exe runs them on the way out of each guarded part.
/// `flsall` (Ghidra's function ID), cdecl, the mode: 1 flushes every open stream and answers
/// how many flushed, 0 flushes the streams open for writing and answers -1 when one failed
/// (else 0). Under lock 1 each set stream (`__piob`, `_nstream` of them) in use (flags
/// 0x83) is locked (`_lock_file2`), checked again, flushed (`_fflush_nolock`) and unlocked
/// (`00ecda38`).
pub fn flsall(e: &mut Engine, mode: u32) -> u32 {
    let mut flushed = 0u32;
    let mut failed = 0u32;
    e.call(LOCK, &args![1u32]);
    let mut index = 0u32;
    while (index as i32) < (e.mem.u32(NSTREAM) as i32) {
        let table = e.mem.u32(PIOB);
        let slot = table.wrapping_add(index.wrapping_mul(4));
        let stream = e.mem.u32(slot);
        if stream != 0 && e.mem.u8(stream.wrapping_add(0xc)) & 0x83 != 0 {
            e.call(LOCK_FILE2, &args![index, stream]);
            let table = e.mem.u32(PIOB);
            let stream = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
            let flags = e.mem.u32(stream.wrapping_add(0xc));
            if flags as u8 & 0x83 != 0 {
                if mode == 1 {
                    if e.call(FFLUSH_NOLOCK, &args![stream]).u32() != 0xffff_ffff {
                        flushed = flushed.wrapping_add(1);
                    }
                } else if mode == 0
                    && flags & 2 != 0
                    && e.call(FFLUSH_NOLOCK, &args![stream]).u32() == 0xffff_ffff
                {
                    failed = 0xffff_ffff;
                }
            }
            e.call(FLSALL_FINALLY_STREAM, &args![index]);
        }
        index = index.wrapping_add(1);
    }
    e.call(FLSALL_FINALLY, &args![]);
    if mode == 1 {
        flushed
    } else {
        failed
    }
}

// Translated from 00ecda38 (disassembled, FalloutNV.exe 1.4.0.525)
/// `flsall`'s inner `__finally` block: `_unlock_file2(index, __piob[index])`, the index being
/// the frame's loop word (`ESI`).
pub fn flsall_finally_stream(e: &mut Engine, index: u32) {
    let table = e.mem.u32(PIOB);
    let stream = e.mem.u32(table.wrapping_add(index.wrapping_mul(4)));
    e.call(UNLOCK_FILE2, &args![index, stream]);
}

// Translated from 00ecda67 (disassembled, FalloutNV.exe 1.4.0.525)
/// `flsall`'s outer `__finally` block: `_unlock(1)`.
pub fn flsall_finally(e: &mut Engine) {
    e.call(UNLOCK, &args![1u32]);
}

// Translated from 00ecda70 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecda70 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 0117fff8) is not translated; its __finally block (00ecdab9) is called as the exe calls it on the way out.
/// `fflush` (Ghidra's function ID), cdecl, a stream: null flushes every stream
/// (`flsall(0)`); otherwise `_fflush_nolock` with the stream locked (`_lock_file`, then the
/// `__finally` block's `__unlock_file`).
pub fn fflush(e: &mut Engine, stream: u32) -> u32 {
    if stream == 0 {
        return e.call(FLSALL, &args![0u32]).u32();
    }
    e.call(LOCK_FILE, &args![stream]);
    let result = e.call(FFLUSH_NOLOCK, &args![stream]).u32();
    e.call(FFLUSH_FINALLY, &args![stream]);
    result
}

// Translated from 00ecdab9 (disassembled, FalloutNV.exe 1.4.0.525)
/// `fflush`'s `__finally` block: `__unlock_file` of the word at `[EBP+8]` (the stream).
pub fn fflush_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE, &args![stream]);
}

// Translated from 00ecdac3 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_flushall` (no label): `flsall(1)`.
pub fn flushall(e: &mut Engine) -> u32 {
    e.call(FLSALL, &args![1u32]).u32()
}

// Translated from 00ecdb00 (disassembled, FalloutNV.exe 1.4.0.525)
/// `wcslen` (no label), cdecl: the number of 16-bit characters before the zero one.
pub fn wcslen(e: &mut Engine, text: u32) -> u32 {
    let mut at = text;
    loop {
        let c = e.mem.u16(at);
        at = at.wrapping_add(2);
        if c == 0 {
            break;
        }
    }
    (((at.wrapping_sub(text)) as i32) >> 1).wrapping_sub(1) as u32
}

// Translated from 00edf2a2 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__sys_nerr` (no label): the address of `_sys_nerr` (`011c2a78`).
pub fn sys_nerr(_e: &mut Engine) -> u32 {
    SYS_NERR_WORD
}

// Translated from 00edf2a8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__sys_errlist` (no label): the address of `_sys_errlist` (`011c29c8`).
pub fn sys_errlist(_e: &mut Engine) -> u32 {
    SYS_ERRLIST_TABLE
}

// Translated from 00ecdb1a (disassembled, FalloutNV.exe 1.4.0.525)
/// `_get_sys_err_msg` (Ghidra's function ID), cdecl, an error number: its text from
/// `_sys_errlist`, a number below 0 or not below `_sys_nerr` (signed) taking the last
/// ("unknown error") entry, `_sys_nerr`'s own.
pub fn get_sys_err_msg(e: &mut Engine, number: u32) -> u32 {
    let mut index = number;
    let in_range = (number as i32) >= 0 && {
        let count = e.call(SYS_NERR, &args![]).u32();
        (number as i32) < (e.mem.u32(count) as i32)
    };
    if !in_range {
        let count = e.call(SYS_NERR, &args![]).u32();
        index = e.mem.u32(count);
    }
    let table = e.call(SYS_ERRLIST, &args![]).u32();
    e.mem.u32(table.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00ecdb42 (disassembled, FalloutNV.exe 1.4.0.525)
/// `strerror` (Ghidra's function ID), cdecl, an error number. Without the thread's data
/// (`_getptd_noexit`), or when its 0x86-byte buffer (+0x24, made with `__calloc_crt` on
/// first use) cannot be had, the fallback text (`010f2060`). Otherwise the message
/// (`_get_sys_err_msg`) is copied into the buffer (`strcpy_s`; a failure calls
/// `__invoke_watson`), which is answered.
pub fn strerror(e: &mut Engine, number: u32) -> u32 {
    let ptd = e.call(GETPTD_NOEXIT, &args![]).u32();
    if ptd == 0 {
        return STRERROR_FALLBACK;
    }
    if e.mem.u32(ptd.wrapping_add(0x24)) == 0 {
        let buffer = e.call(CALLOC_CRT, &args![0x86u32, 1u32]).u32();
        e.mem.set_u32(ptd.wrapping_add(0x24), buffer);
        if buffer == 0 {
            return STRERROR_FALLBACK;
        }
    }
    let buffer = e.mem.u32(ptd.wrapping_add(0x24));
    let message = e.call(GET_SYS_ERR_MSG, &args![number]).u32();
    if e.call(STRCPY_S, &args![buffer, 0x86u32, message]).u32() != 0 {
        e.call(INVOKE_WATSON, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
    }
    buffer
}

// Translated from 00edf1c1 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00edf1c1 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 011804a8) is not translated; its __finally block (00edf298) is called as the exe calls it on the way out.
/// `_commit` (Ghidra's function ID), cdecl, a handle. -2 sets `errno` to `EBADF` and answers
/// -1; a handle outside `0.._nhandle` or not open (`FOPEN`, bit 0 of its `osfile` byte) also
/// reports. With the handle locked (`___lock_fhandle`) and still open,
/// `FlushFileBuffers(_get_osfhandle(handle))`; its failure's `GetLastError` goes to
/// `_doserrno`, `errno` becomes `EBADF` and -1 is answered; success answers 0. A handle
/// closed meanwhile is `EBADF` too. The `__finally` block unlocks it.
pub fn commit(e: &mut Engine, handle: u32) -> u32 {
    let ebadf = |e: &mut Engine| {
        let slot = e.call(ERRNO, &args![]).u32();
        e.mem.set_u32(slot, 9);
    };
    if handle == 0xffff_fffe {
        ebadf(e);
        return 0xffff_ffff;
    }
    let open = |e: &mut Engine| {
        let block = e
            .mem
            .u32(PIOINFO.wrapping_add((((handle as i32) >> 5) as u32).wrapping_mul(4)));
        let entry = block.wrapping_add((handle & 0x1f) << 6);
        e.mem.u8(entry.wrapping_add(4)) & 1 != 0
    };
    if (handle as i32) < 0 || handle >= e.mem.u32(NHANDLE) || !open(e) {
        ebadf(e);
        e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        return 0xffff_ffff;
    }
    e.call(LOCK_FHANDLE, &args![handle]);
    let result = if open(e) {
        let os_handle = e.call(GET_OSFHANDLE, &args![handle]).u32();
        let error = if e.call(FLUSH_FILE_BUFFERS, &args![os_handle]).u32() == 0 {
            e.call(GET_LAST_ERROR, &args![]).u32()
        } else {
            0
        };
        if error == 0 {
            0
        } else {
            let slot = e.call(DOSERRNO, &args![]).u32();
            e.mem.set_u32(slot, error);
            ebadf(e);
            0xffff_ffff
        }
    } else {
        ebadf(e);
        0xffff_ffff
    };
    e.call(COMMIT_FINALLY, &args![handle]);
    result
}

// Translated from 00edf298 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_commit`'s `__finally` block: `__unlock_fhandle` of the word at `[EBP+8]` (the handle).
pub fn commit_finally(e: &mut Engine, handle: u32) {
    e.call(UNLOCK_FHANDLE, &args![handle]);
}

// Translated from 00eddfc1 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___get_sigabrt` (Ghidra's function ID): the `SIGABRT` handler (`01270718`) decoded
/// (`__decode_pointer`).
pub fn get_sigabrt(e: &mut Engine) -> u32 {
    let encoded = e.mem.u32(SIGABRT_HANDLER);
    e.call(DECODE_POINTER, &args![encoded]).u32()
}

// Translated from 00ed5a9a (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed5a9a - the x87 status word (FSTSW) is not modelled by the engine; it is answered as 0 (no exception flags, stack top 0)
/// `__statfp` (Ghidra's function ID): the x87 status word (`FSTSW`), sign-extended.
pub fn statfp(_e: &mut Engine) -> u32 {
    0
}

// Translated from 00ed5aaa (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed5aaa - the x87 status word (FNSTSW, then FNCLEX) is not modelled by the engine; it is answered as 0 and there are no flags to clear
/// `__clrfp` (Ghidra's function ID): the x87 status word (`FNSTSW`, sign-extended), then its
/// exception flags cleared (`FNCLEX`).
pub fn clrfp(_e: &mut Engine) -> u32 {
    0
}

// Translated from 00ecacb2 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_wcstombs` (Ghidra's function ID), cdecl: target, wide text, count;
/// `__wcstombs_l_helper` with no locale.
pub fn wcstombs(e: &mut Engine, target: u32, text: u32, count: u32) -> u32 {
    e.call(WCSTOMBS_L_HELPER, &args![target, text, count, 0u32])
        .u32()
}

// Translated from 00ecb849 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__mbsnbcmp` (Ghidra's function ID), cdecl: two texts and a byte count;
/// `__mbsnbcmp_l` with no locale.
pub fn mbsnbcmp(e: &mut Engine, a: u32, b: u32, count: u32) -> u32 {
    e.call(MBSNBCMP_L, &args![a, b, count, 0u32]).u32()
}

// Translated from 00ecba8e (disassembled, FalloutNV.exe 1.4.0.525)
/// `__mbsnbicmp` (Ghidra's function ID), cdecl: two texts and a byte count;
/// `__mbsnbicmp_l` with no locale.
pub fn mbsnbicmp(e: &mut Engine, a: u32, b: u32, count: u32) -> u32 {
    e.call(MBSNBICMP_L, &args![a, b, count, 0u32]).u32()
}

// Translated from 0071cfd0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbsnbcmp` (Ghidra's function ID; the game's own copy), cdecl: passes its three
/// arguments to `__mbsnbcmp` (`00ecb849`).
pub fn game_mbsnbcmp(e: &mut Engine, a: u32, b: u32, count: u32) -> u32 {
    e.call(MBSNBCMP, &args![a, b, count]).u32()
}

// Translated from 0071cff0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbsnicmp` (Ghidra's function ID; the game's own copy), cdecl: passes its three
/// arguments to `__mbsnbicmp` (`00ecba8e`).
pub fn game_mbsnicmp(e: &mut Engine, a: u32, b: u32, count: u32) -> u32 {
    e.call(MBSNBICMP, &args![a, b, count]).u32()
}

// Translated from 00eca903 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__mbsnbcpy` (Ghidra's function ID), cdecl: target, source, byte count; `_mbsnbcpy_l`
/// with no locale.
pub fn mbsnbcpy(e: &mut Engine, target: u32, source: u32, count: u32) -> u32 {
    e.call(MBSNBCPY_L, &args![target, source, count, 0u32])
        .u32()
}

// Translated from 00ecad86 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_wcstombs_s` (Ghidra's function ID), cdecl: count out, target, its size, wide text,
/// count; `__wcstombs_s_l` with no locale.
pub fn wcstombs_s(e: &mut Engine, out: u32, target: u32, size: u32, text: u32, count: u32) -> u32 {
    e.call(WCSTOMBS_S_L, &args![out, target, size, text, count, 0u32])
        .u32()
}

// Translated from 00ecdf2d (disassembled, FalloutNV.exe 1.4.0.525)
/// The null-locale form of `00ecde0e` (no labels), cdecl: its two arguments and a null
/// locale.
pub fn fn_00ecdf2d(e: &mut Engine, a: u32, b: u32) -> u32 {
    e.call(FN_00ECDE0E, &args![a, b, 0u32]).u32()
}

// Translated from 00ec9de6 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_printf`'s `__finally` block: `_unlock_file2(1, stdout)` (`__iob_func()` + 0x20).
pub fn printf_finally(e: &mut Engine) {
    let iob = e.call(IOB_FUNC, &args![]).u32();
    e.call(UNLOCK_FILE2, &args![1u32, iob.wrapping_add(0x20)]);
}

// Translated from 00ecb39e (disassembled, FalloutNV.exe 1.4.0.525)
/// `_fread_s`'s `__finally` block: `__unlock_file` of the word at `[EBP+0x18]` (the stream).
pub fn fread_s_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE, &args![stream]);
}

// Translated from 00ecb5c6 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_ftell`'s `__finally` block: `__unlock_file` of the word at `[EBP+8]` (the stream).
pub fn ftell_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE, &args![stream]);
}

// Translated from 00ecb6d5 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_fseek`'s `__finally` block: `__unlock_file` of the word at `[EBP+8]` (the stream).
pub fn fseek_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE, &args![stream]);
}

// Translated from 00edcfff (disassembled, FalloutNV.exe 1.4.0.525)
/// `__read`'s `__finally` block: `__unlock_fhandle` of the word at `[EBP+8]` (the handle).
pub fn read_finally(e: &mut Engine, handle: u32) {
    e.call(UNLOCK_FHANDLE, &args![handle]);
}

// Translated from 00edd150 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__lseek`'s `__finally` block: `__unlock_fhandle` of the word at `[EBP+8]` (the handle).
pub fn lseek_finally(e: &mut Engine, handle: u32) {
    e.call(UNLOCK_FHANDLE, &args![handle]);
}

// Translated from 00ede142 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_raise`'s `__finally` block: `_unlock(0)` when the frame's "locked" word (`EBP-0x1c`) is
/// set.
pub fn raise_finally(e: &mut Engine, locked: u32) {
    if locked != 0 {
        e.call(UNLOCK, &args![0u32]);
    }
}

// Translated from 00eccc1c (disassembled, FalloutNV.exe 1.4.0.525)
/// A `__finally` block of `__getcwd`: `_unlock(0x7)`.
pub fn unlock_block_00eccc1c(e: &mut Engine) {
    e.call(UNLOCK, &args![0x7u32]);
}

// Translated from 00edf496 (disassembled, FalloutNV.exe 1.4.0.525)
/// A `__finally` block of `__fcloseall`: `_unlock(0x1)`.
pub fn unlock_block_00edf496(e: &mut Engine) {
    e.call(UNLOCK, &args![0x1u32]);
}

// Translated from 00ee2782 (disassembled, FalloutNV.exe 1.4.0.525)
/// A `__finally` block of `__tzset_nolock`: `_unlock(0x7)`.
pub fn unlock_block_00ee2782(e: &mut Engine) {
    e.call(UNLOCK, &args![0x7u32]);
}

// Translated from 00ee2c31 (disassembled, FalloutNV.exe 1.4.0.525)
/// A `__finally` block of `___tzset`: `_unlock(0x6)`.
pub fn unlock_block_00ee2c31(e: &mut Engine) {
    e.call(UNLOCK, &args![0x6u32]);
}

// Translated from 00ee2c72 (disassembled, FalloutNV.exe 1.4.0.525)
/// A `__finally` block of `__isindst`: `_unlock(0x6)`.
pub fn unlock_block_00ee2c72(e: &mut Engine) {
    e.call(UNLOCK, &args![0x6u32]);
}

// Translated from 00ee2f53 (disassembled, FalloutNV.exe 1.4.0.525)
/// A `__finally` block of `___lock_fhandle`: `_unlock(0xa)`.
pub fn unlock_block_00ee2f53(e: &mut Engine) {
    e.call(UNLOCK, &args![0xau32]);
}

// Translated from 00ee3056 (disassembled, FalloutNV.exe 1.4.0.525)
/// A `__finally` block of `__alloc_osfhnd`: `_unlock(0xa)`.
pub fn unlock_block_00ee3056(e: &mut Engine) {
    e.call(UNLOCK, &args![0xau32]);
}

// Translated from 00ed049c (disassembled, FalloutNV.exe 1.4.0.525)
/// A `__finally` block of `_setlocale`: `_unlock(0xc)`.
pub fn unlock_block_00ed049c(e: &mut Engine) {
    e.call(UNLOCK, &args![0xcu32]);
}

// Translated from 00ed04a8 (disassembled, FalloutNV.exe 1.4.0.525)
/// A `__finally` block of `_setlocale`: `_unlock(0xc)`.
pub fn unlock_block_00ed04a8(e: &mut Engine) {
    e.call(UNLOCK, &args![0xcu32]);
}

/// The registry entries of the functions that take registers or 64-bit words.
fn exception_destructor_entry(e: &mut Engine, words: &[u32]) -> Ret {
    exception_destructor(e, words[0]);
    Ret::default()
}

fn alldiv_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let dividend = (u64::from(words[0]) | u64::from(words[1]) << 32) as i64;
    let divisor = (u64::from(words[2]) | u64::from(words[3]) << 32) as i64;
    (alldiv(e, dividend, divisor) as u64).into_ret()
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00ec_79a1, exception_destructor_entry as AbiFn),
        (0x00ec_c250, alldiv_entry as AbiFn),
        entry!(0x00ec_d94e, fflush_nolock(u32) -> u32),
        entry!(0x00ec_d996, flsall(u32) -> u32),
        entry!(0x00ec_da38, flsall_finally_stream(u32)),
        entry!(0x00ec_da67, flsall_finally()),
        entry!(0x00ec_da70, fflush(u32) -> u32),
        entry!(0x00ec_dab9, fflush_finally(u32)),
        entry!(0x00ec_dac3, flushall() -> u32),
        entry!(0x00ec_db00, wcslen(u32) -> u32),
        entry!(0x00ed_f2a2, sys_nerr() -> u32),
        entry!(0x00ed_f2a8, sys_errlist() -> u32),
        entry!(0x00ec_db1a, get_sys_err_msg(u32) -> u32),
        entry!(0x00ec_db42, strerror(u32) -> u32),
        entry!(0x00ed_f1c1, commit(u32) -> u32),
        entry!(0x00ed_f298, commit_finally(u32)),
        entry!(0x00ed_dfc1, get_sigabrt() -> u32),
        entry!(0x00ed_5a9a, statfp() -> u32),
        entry!(0x00ed_5aaa, clrfp() -> u32),
        entry!(0x00ec_acb2, wcstombs(u32, u32, u32) -> u32),
        entry!(0x00ec_b849, mbsnbcmp(u32, u32, u32) -> u32),
        entry!(0x00ec_ba8e, mbsnbicmp(u32, u32, u32) -> u32),
        entry!(0x0071_cfd0, game_mbsnbcmp(u32, u32, u32) -> u32),
        entry!(0x0071_cff0, game_mbsnicmp(u32, u32, u32) -> u32),
        entry!(0x00ec_a903, mbsnbcpy(u32, u32, u32) -> u32),
        entry!(0x00ec_ad86, wcstombs_s(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ec_df2d, fn_00ecdf2d(u32, u32) -> u32),
        entry!(0x00ec_9de6, printf_finally()),
        entry!(0x00ec_b39e, fread_s_finally(u32)),
        entry!(0x00ec_b5c6, ftell_finally(u32)),
        entry!(0x00ec_b6d5, fseek_finally(u32)),
        entry!(0x00ed_cfff, read_finally(u32)),
        entry!(0x00ed_d150, lseek_finally(u32)),
        entry!(0x00ed_e142, raise_finally(u32)),
        entry!(0x00ec_cc1c, unlock_block_00eccc1c()),
        entry!(0x00ed_f496, unlock_block_00edf496()),
        entry!(0x00ee_2782, unlock_block_00ee2782()),
        entry!(0x00ee_2c31, unlock_block_00ee2c31()),
        entry!(0x00ee_2c72, unlock_block_00ee2c72()),
        entry!(0x00ee_2f53, unlock_block_00ee2f53()),
        entry!(0x00ee_3056, unlock_block_00ee3056()),
        entry!(0x00ed_049c, unlock_block_00ed049c()),
        entry!(0x00ed_04a8, unlock_block_00ed04a8()),
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

    /// An engine with `errno`, `__doserrno`, the report and the locks doubled; the errno
    /// slot.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let slot = e.mem.alloc(8);
        e.register_double(ERRNO, move |_, _| slot.into_ret());
        e.register_double(DOSERRNO, move |_, _| (slot + 4).into_ret());
        for addr in [
            INVALID_PARAMETER,
            LOCK,
            UNLOCK,
            LOCK_FILE,
            LOCK_FILE2,
            UNLOCK_FILE,
            UNLOCK_FILE2,
            LOCK_FHANDLE,
            UNLOCK_FHANDLE,
            FREE_FN,
            INVOKE_WATSON,
        ] {
            e.register_double(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, slot)
    }

    #[test]
    fn exception_destructor_frees_an_owned_text() {
        let (mut e, _) = engine();
        let object = e.mem.alloc(12);
        e.mem.set_u32(object, 0x1234);
        e.mem.set_u32(object + 4, 0x5000);
        e.mem.set_u32(object + 8, 1);
        e.call(0x00ec_79a1, &args![object]);
        assert_eq!(e.mem.u32(object), EXCEPTION_VTABLE);
        assert_eq!(calls(&e, FREE_FN), vec![vec![0x5000]]);
        e.mem.set_u32(object + 8, 0);
        exception_destructor(&mut e, object);
        assert_eq!(calls(&e, FREE_FN).len(), 1);
    }

    #[test]
    fn alldiv_truncates_toward_zero() {
        let (mut e, _) = engine();
        assert_eq!(alldiv(&mut e, -7, 2), -3);
        assert_eq!(alldiv(&mut e, 7, -2), -3);
        assert_eq!(alldiv(&mut e, -7, -2), 3);
        assert_eq!(alldiv(&mut e, 1 << 40, 3), (1 << 40) / 3);
        let r = e.call(
            0x00ec_c250,
            &args![0u32, 0x10u32, 0xffff_fffeu32, 0xffff_ffffu32],
        );
        assert_eq!(r.u64() as i64, -(0x10_0000_0000i64 / 2));
    }

    #[test]
    fn fflush_flushes_and_commits() {
        let (mut e, _) = engine();
        let stream = e.mem.alloc(0x20);
        e.register_double(FLUSH, |_, _| 0u32.into_ret());
        e.register_double(FILENO, |_, _| 3u32.into_ret());
        e.register_double(COMMIT, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ec_da70, &args![stream]).u32(), 0);
        assert!(calls(&e, COMMIT).is_empty());
        e.mem.set_u32(stream + 0xc, 0x4000);
        assert_eq!(fflush_nolock(&mut e, stream), 0);
        assert_eq!(calls(&e, COMMIT), vec![vec![3]]);
        e.register_double(COMMIT, |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(fflush_nolock(&mut e, stream), 0xffff_ffff);
        e.register_double(FLUSH, |_, _| 1u32.into_ret());
        assert_eq!(fflush(&mut e, stream), 0xffff_ffff);
        assert_eq!(calls(&e, LOCK_FILE), vec![vec![stream]; 2]);
        assert_eq!(calls(&e, UNLOCK_FILE), vec![vec![stream]; 2]);
        // Null: every stream.
        e.register_double(FLSALL, |_, a| (a[0] + 7).into_ret());
        assert_eq!(fflush(&mut e, 0), 7);
        assert_eq!(fflush_nolock(&mut e, 0), 7);
        assert_eq!(e.call(0x00ec_dac3, &args![]).u32(), 8);
    }

    #[test]
    fn flsall_counts_or_reports_failures() {
        let (mut e, _) = engine();
        e.mem.map(PIOB, 4);
        e.mem.map(NSTREAM, 4);
        let table = e.mem.alloc(16);
        let streams: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x20)).collect();
        // A reading stream, a writing one, an unused one, and an empty slot.
        e.mem.set_u32(streams[0] + 0xc, 0x01);
        e.mem.set_u32(streams[1] + 0xc, 0x02);
        e.mem.set_u32(streams[2] + 0xc, 0x40);
        for (i, s) in streams.iter().enumerate() {
            e.mem.set_u32(table + i as u32 * 4, *s);
        }
        e.mem.set_u32(PIOB, table);
        e.mem.set_u32(NSTREAM, 4);
        e.register_double(FFLUSH_NOLOCK, |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(flsall(&mut e, 0), 0xffff_ffff);
        assert_eq!(calls(&e, FFLUSH_NOLOCK), vec![vec![streams[1]]]);
        assert_eq!(calls(&e, LOCK_FILE2).len(), 2);
        assert_eq!(
            calls(&e, UNLOCK_FILE2),
            vec![vec![0, streams[0]], vec![1, streams[1]]]
        );
        assert_eq!(calls(&e, LOCK), vec![vec![1]]);
        assert_eq!(calls(&e, UNLOCK), vec![vec![1]]);
        e.call_log = Some(vec![]);
        e.register_double(FFLUSH_NOLOCK, |_, a| a[0].into_ret());
        assert_eq!(e.call(FLSALL, &args![1u32]).u32(), 2);
        assert_eq!(flsall(&mut e, 0), 0);
    }

    #[test]
    fn wcslen_counts_wide_characters() {
        let (mut e, _) = engine();
        let text = e.mem.alloc(16);
        for (i, c) in [0x41u16, 0x3042, 0x42, 0].into_iter().enumerate() {
            e.mem.set_u16(text + i as u32 * 2, c);
        }
        assert_eq!(e.call(0x00ec_db00, &args![text]).u32(), 3);
        assert_eq!(wcslen(&mut e, text + 6), 0);
    }

    #[test]
    fn strerror_copies_the_message_into_the_thread_buffer() {
        let (mut e, _) = engine();
        e.mem.map(SYS_NERR_WORD, 4);
        e.mem.map(SYS_ERRLIST_TABLE, 0x10);
        e.mem.set_u32(SYS_NERR_WORD, 2);
        let texts: Vec<u32> = [b"No error".as_slice(), b"Not owner", b"Unknown error"]
            .into_iter()
            .map(|t| {
                let at = e.mem.alloc(16);
                e.mem.set_cstr(at, t);
                at
            })
            .collect();
        for (i, t) in texts.iter().enumerate() {
            e.mem.set_u32(SYS_ERRLIST_TABLE + i as u32 * 4, *t);
        }
        assert_eq!(get_sys_err_msg(&mut e, 1), texts[1]);
        assert_eq!(get_sys_err_msg(&mut e, 2), texts[2]);
        assert_eq!(get_sys_err_msg(&mut e, 0xffff_ffff), texts[2]);
        let ptd = e.mem.alloc(0x40);
        e.register_double(GETPTD_NOEXIT, move |_, _| ptd.into_ret());
        e.register_double(CALLOC_CRT, |e, a| e.mem.alloc(a[0] * a[1]).into_ret());
        let buffer = e.call(0x00ec_db42, &args![1u32]).u32();
        assert_eq!(e.mem.u32(ptd + 0x24), buffer);
        assert_eq!(e.mem.cstr(buffer), b"Not owner".to_vec());
        assert_eq!(strerror(&mut e, 7), buffer);
        assert_eq!(e.mem.cstr(buffer), b"Unknown error".to_vec());
        assert_eq!(calls(&e, CALLOC_CRT), vec![vec![0x86, 1]]);
        // No thread data, or no buffer: the fallback.
        e.mem.set_u32(ptd + 0x24, 0);
        e.register_double(CALLOC_CRT, |_, _| 0u32.into_ret());
        assert_eq!(strerror(&mut e, 1), STRERROR_FALLBACK);
        e.register_double(GETPTD_NOEXIT, |_, _| 0u32.into_ret());
        assert_eq!(strerror(&mut e, 1), STRERROR_FALLBACK);
    }

    #[test]
    fn commit_flushes_an_open_handle() {
        let (mut e, slot) = engine();
        e.mem.map(PIOINFO, 4);
        e.mem.map(NHANDLE, 4);
        let block = e.mem.alloc(0x40 * 32);
        e.mem.set_u32(PIOINFO, block);
        e.mem.set_u32(NHANDLE, 32);
        e.mem.set_u8(block + 3 * 0x40 + 4, 1);
        e.register_double(GET_OSFHANDLE, |_, a| (a[0] + 0x100).into_ret());
        e.register_double(FLUSH_FILE_BUFFERS, |_, _| 1u32.into_ret());
        assert_eq!(e.call(COMMIT, &args![3u32]).u32(), 0);
        assert_eq!(calls(&e, FLUSH_FILE_BUFFERS), vec![vec![0x103]]);
        assert_eq!(calls(&e, LOCK_FHANDLE), vec![vec![3]]);
        assert_eq!(calls(&e, UNLOCK_FHANDLE), vec![vec![3]]);
        // A failed flush: the system error in _doserrno, EBADF.
        e.register_double(FLUSH_FILE_BUFFERS, |_, _| 0u32.into_ret());
        e.register_double(GET_LAST_ERROR, |_, _| 6u32.into_ret());
        assert_eq!(commit(&mut e, 3), 0xffff_ffff);
        assert_eq!((e.mem.u32(slot), e.mem.u32(slot + 4)), (9, 6));
        // Bad handles.
        e.mem.set_u32(slot, 0);
        assert_eq!(commit(&mut e, 0xffff_fffe), 0xffff_ffff);
        assert_eq!(e.mem.u32(slot), 9);
        assert_eq!(commit(&mut e, 4), 0xffff_ffff);
        assert_eq!(commit(&mut e, 32), 0xffff_ffff);
        assert_eq!(commit(&mut e, 0xffff_fff0), 0xffff_ffff);
        assert_eq!(calls(&e, INVALID_PARAMETER).len(), 3);
    }

    #[test]
    fn wrappers_pass_a_null_locale() {
        let (mut e, _) = engine();
        for addr in [
            WCSTOMBS_L_HELPER,
            MBSNBCMP_L,
            MBSNBICMP_L,
            MBSNBCPY_L,
            WCSTOMBS_S_L,
            FN_00ECDE0E,
        ] {
            e.register_double(addr, |_, a| (a.len() as u32).into_ret());
        }
        assert_eq!(e.call(0x00ec_acb2, &args![1u32, 2u32, 3u32]).u32(), 4);
        assert_eq!(e.call(0x0071_cfd0, &args![1u32, 2u32, 3u32]).u32(), 4);
        assert_eq!(e.call(0x0071_cff0, &args![1u32, 2u32, 3u32]).u32(), 4);
        assert_eq!(mbsnbcpy(&mut e, 1, 2, 3), 4);
        assert_eq!(wcstombs_s(&mut e, 1, 2, 3, 4, 5), 6);
        assert_eq!(fn_00ecdf2d(&mut e, 1, 2), 3);
        assert_eq!(calls(&e, MBSNBCMP_L), vec![vec![1, 2, 3, 0]]);
        assert_eq!(calls(&e, MBSNBICMP_L), vec![vec![1, 2, 3, 0]]);
        assert_eq!(calls(&e, WCSTOMBS_S_L), vec![vec![1, 2, 3, 4, 5, 0]]);
    }

    #[test]
    fn finally_blocks_unlock() {
        let (mut e, _) = engine();
        e.register_double(IOB_FUNC, |_, _| 0x1000u32.into_ret());
        e.register_double(DECODE_POINTER, |_, a| (a[0] ^ 0xff).into_ret());
        e.mem.map(SIGABRT_HANDLER, 4);
        e.mem.set_u32(SIGABRT_HANDLER, 0x1200);
        assert_eq!(e.call(0x00ed_dfc1, &args![]).u32(), 0x12ff);
        e.call(0x00ec_9de6, &args![]);
        assert_eq!(calls(&e, UNLOCK_FILE2), vec![vec![1, 0x1020]]);
        for addr in [0x00ec_b39e, 0x00ec_b5c6, 0x00ec_b6d5, 0x00ec_dab9] {
            e.call(addr, &args![0x55u32]);
        }
        assert_eq!(calls(&e, UNLOCK_FILE).len(), 4);
        for addr in [0x00ed_cfff, 0x00ed_d150, 0x00ed_f298] {
            e.call(addr, &args![9u32]);
        }
        assert_eq!(calls(&e, UNLOCK_FHANDLE), vec![vec![9]; 3]);
        e.call(0x00ed_e142, &args![0u32]);
        e.call(0x00ed_e142, &args![1u32]);
        for addr in [
            0x00ec_cc1c,
            0x00ed_f496,
            0x00ee_2782,
            0x00ee_2c31,
            0x00ee_2c72,
            0x00ee_2f53,
            0x00ee_3056,
            0x00ed_049c,
            0x00ed_04a8,
            0x00ec_da67,
        ] {
            e.call(addr, &args![]);
        }
        let locks: Vec<u32> = calls(&e, UNLOCK).into_iter().map(|c| c[0]).collect();
        assert_eq!(locks, vec![0, 7, 1, 7, 6, 6, 0xa, 0xa, 0xc, 0xc, 1]);
        assert_eq!(statfp(&mut e), 0);
        assert_eq!(e.call(0x00ed_5aaa, &args![]).u32(), 0);
        assert_eq!(e.call(SYS_NERR, &args![]).u32(), SYS_NERR_WORD);
        assert_eq!(e.call(SYS_ERRLIST, &args![]).u32(), SYS_ERRLIST_TABLE);
    }
}

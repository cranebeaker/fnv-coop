//! C runtime pieces the play build reaches next (MSVC 2008 LIBCMT/libcpmt, statically linked
//! into FalloutNV.exe 1.4.0.525), translated from the exe: `strcmp` (`00ec6da0`),
//! `type_info::operator==` (`00ec40cc`), `_purecall` (`00ec60fa`) and `_set_abort_behavior`
//! (`00ecf44c`), the `std::exception` family (`exception(const char* const&)` `00ec78f1`, its
//! copy constructor `00ec7944`, `what` `00ec79b7`, `bad_typeid` `00ec7a0a`/`00ec7a28`,
//! `bad_exception` `00ece2d2` and the scalar deleting destructors `00ec7a8a`, `00ec7aab`,
//! `00ece2fb`), `vprintf_helper` (`00ecbdd0`), `_ftelli64` (`00edf08c`) and `_fseeki64`
//! (`00edf172`), `__cexit` (`00eccef1`), `fast_error_exit` (`00ecc334`), `terminate`
//! (`00ed14b4`), `unexpected` (`00ed14ed`), `_inconsistency` (`00ed1500`), the float `atan2`
//! wrapper (`004b5550`), and the `__finally` blocks of the functions around them.
//!
//! A `__finally` block runs on its function's frame; the frame word or register it reads is
//! its argument here. Every call is made by address, as the exe makes it, so a test can double
//! each one.

use super::*;

/// `strcmp` (`00ec6da0`), `strlen` (`00ec6130`), `malloc` (`00ecd1c7`), `strcpy_s`
/// (`00ec65a6`), `operator delete` (`00401030`), all cdecl.
const STRCMP: u32 = 0x00ec_6da0;
const STRLEN: u32 = 0x00ec_6130;
const MALLOC: u32 = 0x00ec_d1c7;
const STRCPY_S: u32 = 0x00ec_65a6;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `std::exception`'s constructor from a text reference (`00ec78f1`), its copy constructor
/// (`00ec7944`) and its destructor (`00ec79a1`), thiscall.
const EXCEPTION_CTOR: u32 = 0x00ec_78f1;
const EXCEPTION_COPY: u32 = 0x00ec_7944;
const EXCEPTION_DTOR: u32 = 0x00ec_79a1;
/// The vtables of `std::exception` (`010f1fa4`), `std::bad_cast` (`010f1fc4`),
/// `std::bad_typeid` (`010f1fd0`) and `std::bad_exception` (`010f20a8`), and the text `what`
/// answers for an exception without one (`010f1fac`, "Unknown exception").
const EXCEPTION_VTABLE: u32 = 0x010f_1fa4;
const BAD_CAST_VTABLE: u32 = 0x010f_1fc4;
const BAD_TYPEID_VTABLE: u32 = 0x010f_1fd0;
const BAD_EXCEPTION_VTABLE: u32 = 0x010f_20a8;
const UNKNOWN_EXCEPTION: u32 = 0x010f_1fac;
/// `DecodePointer` (`00ed0f92`), `_NMSG_WRITE` (`00ed290c`), `abort` (`00ecf335`),
/// `_getptd` (`00ed11de`), cdecl.
const DECODE_POINTER: u32 = 0x00ed_0f92;
const NMSG_WRITE: u32 = 0x00ed_290c;
const ABORT: u32 = 0x00ec_f335;
const GETPTD: u32 = 0x00ed_11de;
/// The encoded `_purecall` handler (`01270464`), the encoded `_inconsistency` hook
/// (`0126fe10`) and the abort behaviour flags (`__abort_behavior`, `011c1cac`).
const PURECALL_HANDLER: u32 = 0x0127_0464;
const INCONSISTENCY_HOOK: u32 = 0x0126_fe10;
const ABORT_BEHAVIOR: u32 = 0x011c_1cac;
/// `_set_abort_behavior` (`00ecf44c`) and `terminate` (`00ed14b4`).
const SET_ABORT_BEHAVIOR: u32 = 0x00ec_f44c;
const TERMINATE: u32 = 0x00ed_14b4;
/// `__iob_func` (`00ecdf44`), `_errno` (`00ec85e3`), the invalid-parameter report
/// (`00ec7c30`, five words), `_lock_file` (`00ece01b`), `_unlock_file` (`00ece08e`), `_stbuf`
/// (`00ed7d48`) and `_ftbuf` (`00ed7de4`), cdecl.
const IOB_FUNC: u32 = 0x00ec_df44;
const ERRNO: u32 = 0x00ec_85e3;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
const LOCK_FILE: u32 = 0x00ec_e01b;
const UNLOCK_FILE: u32 = 0x00ec_e08e;
const STBUF: u32 = 0x00ed_7d48;
const FTBUF: u32 = 0x00ed_7de4;
/// `vprintf_helper`'s `__finally` block (`00ecbe58`).
const VPRINTF_HELPER_FINALLY: u32 = 0x00ec_be58;
/// `_ftelli64_nolock` (`00eded40`) and `_fseeki64_nolock` (`00edf0d6`), cdecl, and the
/// `__finally` blocks of `_ftelli64` (`00edf0cc`) and `_fseeki64` (`00edf1b7`).
const FTELLI64_NOLOCK: u32 = 0x00ed_ed40;
const FSEEKI64_NOLOCK: u32 = 0x00ed_f0d6;
const FTELLI64_FINALLY: u32 = 0x00ed_f0cc;
const FSEEKI64_FINALLY: u32 = 0x00ed_f1b7;
/// `_unlock` (`00edde5f`, cdecl: the lock number).
const UNLOCK: u32 = 0x00ed_de5f;
/// `doexit` (`00eccd99`, cdecl: code, quick, return), `_FF_MSGBANNER` (`00ed2ab7`),
/// `__crtExitProcess` (`00eccca9`) and the error mode (`__error_mode`, `0126fdb4`).
const DOEXIT: u32 = 0x00ec_cd99;
const FF_MSGBANNER: u32 = 0x00ed_2ab7;
const CRT_EXIT_PROCESS: u32 = 0x00ec_cca9;
const ERROR_MODE: u32 = 0x0126_fdb4;
/// The float `atan2` worker (`004b5570`, cdecl: two floats; the result in `ST0`).
const ATAN2_FLOAT_WORKER: u32 = 0x004b_5570;

// Translated from 00ec6da0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `strcmp` (LIBCMT's assembly version), cdecl: the texts compared byte by byte, unsigned;
/// 0 when they are equal, else -1 or 1 by the first differing byte. (The exe reads aligned
/// words where it can; the bytes it compares are the same.)
pub fn strcmp(e: &mut Engine, a: u32, b: u32) -> i32 {
    let mut i = 0u32;
    loop {
        let x = e.mem.u8(a.wrapping_add(i));
        let y = e.mem.u8(b.wrapping_add(i));
        if x != y {
            return if x < y { -1 } else { 1 };
        }
        if x == 0 {
            return 0;
        }
        i = i.wrapping_add(1);
    }
}

// Translated from 00ec40cc (disassembled, FalloutNV.exe 1.4.0.525)
/// `type_info::operator==` (thiscall, one word: the other `type_info`; `RET 4`): 1 when
/// `strcmp(other + 9, this + 9)` (the decorated names) is 0, else 0.
pub fn type_info_equal(e: &mut Engine, this: u32, other: u32) -> u32 {
    let order = e
        .call(STRCMP, &args![other.wrapping_add(9), this.wrapping_add(9)])
        .u32();
    u32::from(order == 0)
}

// Translated from 00ec60fa (disassembled, FalloutNV.exe 1.4.0.525)
/// `_purecall` (LIBCMT/purevirt.c, no arguments): runs the handler `_set_purecall_handler`
/// left (`DecodePointer` of `01270464`) when there is one, then writes runtime error 25
/// (`_NMSG_WRITE(0x19)`), turns off the abort message (`_set_abort_behavior(0, 1)`) and
/// aborts (a jump to `abort`).
pub fn purecall(e: &mut Engine) {
    let encoded = e.mem.u32(PURECALL_HANDLER);
    let handler = e.call(DECODE_POINTER, &args![encoded]).u32();
    if handler != 0 {
        e.call(handler, &[]);
    }
    e.call(NMSG_WRITE, &args![0x19u32]);
    e.call(SET_ABORT_BEHAVIOR, &args![0u32, 1u32]);
    e.call(ABORT, &[]);
}

// Translated from 00ecf44c (disassembled, FalloutNV.exe 1.4.0.525)
/// `_set_abort_behavior` (cdecl: flags, mask): the bits of `mask` in `__abort_behavior`
/// (`011c1cac`) become those of `flags`. Answers the old value.
pub fn set_abort_behavior(e: &mut Engine, flags: u32, mask: u32) -> u32 {
    let old = e.mem.u32(ABORT_BEHAVIOR);
    e.mem
        .set_u32(ABORT_BEHAVIOR, (!mask & old) | (flags & mask));
    old
}

/// A copy of the text at `text` in a new `malloc` block of its length plus one
/// (`strlen`, `malloc`, `strcpy_s`), or 0 when the allocation fails.
fn copy_text(e: &mut Engine, text: u32) -> u32 {
    let size = e.call(STRLEN, &args![text]).u32().wrapping_add(1);
    let copy = e.call(MALLOC, &args![size]).u32();
    if copy != 0 {
        e.call(STRCPY_S, &args![copy, size, text]);
    }
    copy
}

// Translated from 00ec78f1 (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::exception::exception(const char* const&)` (thiscall, one word: the address of the
/// text pointer; `RET 4`): the `std::exception` vtable; a copy of the text at +4 (0 for a
/// null text or a failed allocation) and 1 ("owns its text") at +8. Answers `this`.
pub fn exception_from_text(e: &mut Engine, this: u32, text_ref: u32) -> u32 {
    e.mem.set_u32(this, EXCEPTION_VTABLE);
    let text = e.mem.u32(text_ref);
    let copy = if text != 0 { copy_text(e, text) } else { 0 };
    e.mem.set_u32(this.wrapping_add(4), copy);
    e.mem.set_u32(this.wrapping_add(8), 1);
    this
}

// Translated from 00ec7944 (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::exception`'s copy constructor (thiscall, one word: the other exception; `RET 4`):
/// the `std::exception` vtable and the other's "owns its text" word (+8); an owned text is
/// copied (0 for none or a failed allocation), a borrowed one is shared. Answers `this`.
pub fn exception_copy(e: &mut Engine, this: u32, other: u32) -> u32 {
    e.mem.set_u32(this, EXCEPTION_VTABLE);
    let owns = e.mem.u32(other.wrapping_add(8));
    e.mem.set_u32(this.wrapping_add(8), owns);
    let text = e.mem.u32(other.wrapping_add(4));
    let kept = if owns == 0 {
        text
    } else if text != 0 {
        copy_text(e, text)
    } else {
        0
    };
    e.mem.set_u32(this.wrapping_add(4), kept);
    this
}

// Translated from 00ec79b7 (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::exception::what` (thiscall, no stack words): the text (+4), or "Unknown exception"
/// (`010f1fac`) when there is none.
pub fn exception_what(e: &mut Engine, this: u32) -> u32 {
    let text = e.mem.u32(this.wrapping_add(4));
    if text != 0 {
        text
    } else {
        UNKNOWN_EXCEPTION
    }
}

/// An exception constructed from a text (`00ec78f1` on the address of the text word), then
/// given `vtable`. Answers `this`.
fn derived_from_text(e: &mut Engine, this: u32, text: u32, vtable: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, text);
        e.call(EXCEPTION_CTOR, &args![this, slot]);
    });
    e.mem.set_u32(this, vtable);
    this
}

// Translated from 00ec7a0a (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::bad_typeid::bad_typeid(const char*)` (thiscall, one word; `RET 4`): the exception
/// constructor on the address of its argument, then the `bad_typeid` vtable. Answers `this`.
pub fn bad_typeid_from_text(e: &mut Engine, this: u32, text: u32) -> u32 {
    derived_from_text(e, this, text, BAD_TYPEID_VTABLE)
}

// Translated from 00ec7a28 (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::bad_typeid`'s copy constructor (thiscall, one word; `RET 4`): the exception copy
/// constructor, then the `bad_typeid` vtable. Answers `this`.
pub fn bad_typeid_copy(e: &mut Engine, this: u32, other: u32) -> u32 {
    e.call(EXCEPTION_COPY, &args![this, other]);
    e.mem.set_u32(this, BAD_TYPEID_VTABLE);
    this
}

// Translated from 00ece2d2 (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::bad_exception::bad_exception(const char*)` (thiscall, one word; `RET 4`): the
/// exception constructor on the address of its argument, then the `bad_exception` vtable.
/// Answers `this`.
pub fn bad_exception_from_text(e: &mut Engine, this: u32, text: u32) -> u32 {
    derived_from_text(e, this, text, BAD_EXCEPTION_VTABLE)
}

/// A scalar deleting destructor: `vtable` first when there is one, the exception destructor,
/// then `operator delete` when bit 0 of `flags` is set. Answers `this`.
fn deleting_destructor(e: &mut Engine, this: u32, flags: u32, vtable: Option<u32>) -> u32 {
    if let Some(vtable) = vtable {
        e.mem.set_u32(this, vtable);
    }
    e.call(EXCEPTION_DTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00ec7a8a (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::exception`'s scalar deleting destructor (thiscall, one word: the flags; `RET 4`).
pub fn exception_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    deleting_destructor(e, this, flags, None)
}

// Translated from 00ec7aab (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::bad_cast`'s scalar deleting destructor (thiscall, one word; `RET 4`): its vtable,
/// then as `std::exception`'s.
pub fn bad_cast_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    deleting_destructor(e, this, flags, Some(BAD_CAST_VTABLE))
}

// Translated from 00ece2fb (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::bad_exception`'s scalar deleting destructor (thiscall, one word; `RET 4`): its
/// vtable, then as `std::exception`'s.
pub fn bad_exception_deleting_destructor(e: &mut Engine, this: u32, flags: u32) -> u32 {
    deleting_destructor(e, this, flags, Some(BAD_EXCEPTION_VTABLE))
}

// Translated from 00ecbdd0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecbdd0 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 0117fe28) is not translated; its __finally block (00ecbe58) is called as the exe calls it on the way out.
/// `vprintf_helper` (LIBCMT/vprintf.c; cdecl: output function, format, locale, argument
/// list): on `stdout` (`__iob_func() + 0x20`). A null format sets `errno` to `EINVAL` (0x16),
/// reports the invalid parameter and answers -1. Otherwise, with the stream locked, the
/// output function (cdecl: stream, format, locale, arguments) runs between `_stbuf` and
/// `_ftbuf`; its result is answered.
pub fn vprintf_helper(e: &mut Engine, output: u32, format: u32, locale: u32, list: u32) -> i32 {
    let stream = e.call(IOB_FUNC, &[]).u32().wrapping_add(0x20);
    if format == 0 {
        let errno = e.call(ERRNO, &[]).u32();
        e.mem.set_u32(errno, 0x16);
        e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        return -1;
    }
    e.call(LOCK_FILE, &args![stream]);
    let buffering = e.call(STBUF, &args![stream]).u32();
    let result = e.call(output, &args![stream, format, locale, list]).u32();
    e.call(FTBUF, &args![buffering, stream]);
    e.call(VPRINTF_HELPER_FINALLY, &args![stream]);
    result as i32
}

// Translated from 00ecbe58 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `vprintf_helper` (`00ecbdd0`): `_unlock_file` of the stream
/// (`ESI`).
pub fn vprintf_helper_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE, &args![stream]);
}

// Translated from 00edf08c (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00edf08c - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 01180468) is not translated; its __finally block (00edf0cc) is called as the exe calls it on the way out.
/// `_ftelli64` (cdecl: the stream): `_ftelli64_nolock` (`00eded40`) with the stream locked;
/// its 64-bit position (`EDX:EAX`) is answered.
pub fn ftelli64(e: &mut Engine, stream: u32) -> u64 {
    e.call(LOCK_FILE, &args![stream]);
    let position = e.call(FTELLI64_NOLOCK, &args![stream]).u64();
    e.call(FTELLI64_FINALLY, &args![stream]);
    position
}

// Translated from 00edf0cc (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `_ftelli64` (`00edf08c`): `_unlock_file` of `[EBP+8]` (the stream).
pub fn ftelli64_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE, &args![stream]);
}

// Translated from 00edf172 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00edf172 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 01180488) is not translated; its __finally block (00edf1b7) is called as the exe calls it on the way out.
/// `_fseeki64` (cdecl: stream, 64-bit offset as two words, origin): `_fseeki64_nolock`
/// (`00edf0d6`) with the stream locked; its result is answered.
pub fn fseeki64(e: &mut Engine, stream: u32, low: u32, high: u32, origin: u32) -> i32 {
    e.call(LOCK_FILE, &args![stream]);
    let result = e
        .call(FSEEKI64_NOLOCK, &args![stream, low, high, origin])
        .u32();
    e.call(FSEEKI64_FINALLY, &args![stream]);
    result as i32
}

// Translated from 00edf1b7 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `_fseeki64` (`00edf172`): `_unlock_file` of `[EBP+8]` (the stream).
pub fn fseeki64_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE, &args![stream]);
}

// Translated from 00ecd4c6 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `fputc` (`00ecd3c2`): `_unlock_file` of the stream (`ESI`).
pub fn fputc_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE, &args![stream]);
}

// Translated from 00ecd627 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `ungetc` (`00ecd5bf`): `_unlock_file` of `[EBP+0xC]` (the stream).
pub fn ungetc_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE, &args![stream]);
}

// Translated from 00ecd72d (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `fgetc` (`00ecd631`): `_unlock_file` of the stream (`ESI`).
pub fn fgetc_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE, &args![stream]);
}

// Translated from 00ecd8dc (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `setvbuf` (`00ecd7e8`): `_unlock_file` of `[EBP+8]` (the stream).
pub fn setvbuf_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE, &args![stream]);
}

// Translated from 00ecf05a (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `_tmpfile_helper` (`00ecef9e`): `_unlock_file` of `[EBP+8]`.
pub fn tmpfile_helper_finally(e: &mut Engine, stream: u32) {
    e.call(UNLOCK_FILE, &args![stream]);
}

// Translated from 00ed1709 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `type_info::_Type_info_dtor` (`00ed16a2`): `_unlock(0xE)`.
pub fn type_info_dtor_finally(e: &mut Engine) {
    e.call(UNLOCK, &args![0xeu32]);
}

// Translated from 00ed4145 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `__setmbcp` (`00ed3fe4`): `_unlock(0xD)` (the multibyte lock).
pub fn setmbcp_finally(e: &mut Engine) {
    e.call(UNLOCK, &args![0xdu32]);
}

// Translated from 00eccef1 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__cexit` (no arguments): `doexit(0, 0, 1)` (run the exit tables, then return).
pub fn cexit(e: &mut Engine) {
    e.call(DOEXIT, &args![0u32, 0u32, 1u32]);
}

// Translated from 00ecc334 (disassembled, FalloutNV.exe 1.4.0.525)
/// `fast_error_exit` (cdecl: the runtime error number): the banner (`_FF_MSGBANNER`) when the
/// error mode is 1 (`_OUT_TO_STDERR`), the message (`_NMSG_WRITE`), then `__crtExitProcess(255)`.
pub fn fast_error_exit(e: &mut Engine, error: u32) {
    if e.mem.u32(ERROR_MODE) == 1 {
        e.call(FF_MSGBANNER, &[]);
    }
    e.call(NMSG_WRITE, &args![error]);
    e.call(CRT_EXIT_PROCESS, &args![0xffu32]);
}

// Translated from 00ed14b4 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed14b4 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 01180248) whose filter catches anything the handler throws is not translated.
/// `terminate` (no arguments): the thread's terminate handler (`_getptd()` +0x78) runs when
/// there is one, then `abort`.
pub fn terminate(e: &mut Engine) {
    let ptd = e.call(GETPTD, &[]).u32();
    let handler = e.mem.u32(ptd.wrapping_add(0x78));
    if handler != 0 {
        e.call(handler, &[]);
    }
    e.call(ABORT, &[]);
}

// Translated from 00ed14ed (disassembled, FalloutNV.exe 1.4.0.525)
/// `unexpected` (no arguments): the thread's unexpected handler (`_getptd()` +0x7C) runs when
/// there is one, then `terminate` (a jump).
pub fn unexpected(e: &mut Engine) {
    let ptd = e.call(GETPTD, &[]).u32();
    let handler = e.mem.u32(ptd.wrapping_add(0x7c));
    if handler != 0 {
        e.call(handler, &[]);
    }
    e.call(TERMINATE, &[]);
}

// Translated from 00ed1500 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed1500 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 01180268) whose filter catches anything the hook throws is not translated.
/// `_inconsistency` (no arguments): the hook (`DecodePointer` of `0126fe10`) runs when there
/// is one, then `terminate`.
pub fn inconsistency(e: &mut Engine) {
    let encoded = e.mem.u32(INCONSISTENCY_HOOK);
    let hook = e.call(DECODE_POINTER, &args![encoded]).u32();
    if hook != 0 {
        e.call(hook, &[]);
    }
    e.call(TERMINATE, &[]);
}

// Translated from 004b5550 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float `atan2` (LIBCMT/atan.c; cdecl: y, x as floats): `004b5570` on copies of both
/// (`FLD`/`FSTP`, so signalling NaNs are passed quiet); its `ST0` result is this function's.
pub fn atan2_float(e: &mut Engine, y: f32, x: f32) -> f32 {
    e.call(ATAN2_FLOAT_WORKER, &args![x87_quiet(y), x87_quiet(x)])
        .f32()
}

/// This file's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_6da0, strcmp(u32, u32) -> i32),
        entry!(0x00ec_40cc, type_info_equal(u32, u32) -> u32),
        entry!(0x00ec_60fa, purecall()),
        entry!(0x00ec_f44c, set_abort_behavior(u32, u32) -> u32),
        entry!(0x00ec_78f1, exception_from_text(u32, u32) -> u32),
        entry!(0x00ec_7944, exception_copy(u32, u32) -> u32),
        entry!(0x00ec_79b7, exception_what(u32) -> u32),
        entry!(0x00ec_7a0a, bad_typeid_from_text(u32, u32) -> u32),
        entry!(0x00ec_7a28, bad_typeid_copy(u32, u32) -> u32),
        entry!(0x00ec_e2d2, bad_exception_from_text(u32, u32) -> u32),
        entry!(0x00ec_7a8a, exception_deleting_destructor(u32, u32) -> u32),
        entry!(0x00ec_7aab, bad_cast_deleting_destructor(u32, u32) -> u32),
        entry!(0x00ec_e2fb, bad_exception_deleting_destructor(u32, u32) -> u32),
        entry!(0x00ec_bdd0, vprintf_helper(u32, u32, u32, u32) -> i32),
        entry!(0x00ec_be58, vprintf_helper_finally(u32)),
        entry!(0x00ed_f08c, ftelli64(u32) -> u64),
        entry!(0x00ed_f0cc, ftelli64_finally(u32)),
        entry!(0x00ed_f172, fseeki64(u32, u32, u32, u32) -> i32),
        entry!(0x00ed_f1b7, fseeki64_finally(u32)),
        entry!(0x00ec_d4c6, fputc_finally(u32)),
        entry!(0x00ec_d627, ungetc_finally(u32)),
        entry!(0x00ec_d72d, fgetc_finally(u32)),
        entry!(0x00ec_d8dc, setvbuf_finally(u32)),
        entry!(0x00ec_f05a, tmpfile_helper_finally(u32)),
        entry!(0x00ed_1709, type_info_dtor_finally()),
        entry!(0x00ed_4145, setmbcp_finally()),
        entry!(0x00ec_cef1, cexit()),
        entry!(0x00ec_c334, fast_error_exit(u32)),
        entry!(0x00ed_14b4, terminate()),
        entry!(0x00ed_14ed, unexpected()),
        entry!(0x00ed_1500, inconsistency()),
        entry!(0x004b_5550, atan2_float(f32, f32) -> f32),
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

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let at = e.mem.alloc(s.len() as u32 + 1);
        e.mem.write(at, s);
        e.mem.set_u8(at + s.len() as u32, 0);
        at
    }

    fn nothing(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    #[test]
    fn strcmp_orders_bytes_unsigned() {
        let mut e = Engine::new();
        let a = text(&mut e, b"abc");
        let b = text(&mut e, b"abd");
        let c = text(&mut e, b"ab");
        let high = text(&mut e, b"\xe9");
        assert_eq!(e.call(STRCMP, &[a, a]).i32(), 0);
        assert_eq!(e.call(STRCMP, &[a, b]).i32(), -1);
        assert_eq!(e.call(STRCMP, &[b, a]).i32(), 1);
        assert_eq!(e.call(STRCMP, &[a, c]).i32(), 1);
        assert_eq!(e.call(STRCMP, &[c, a]).i32(), -1);
        assert_eq!(e.call(STRCMP, &[high, a]).i32(), 1);
    }

    #[test]
    fn type_info_compares_the_decorated_names() {
        let mut e = Engine::new();
        let one = e.mem.alloc(0x20);
        let two = e.mem.alloc(0x20);
        e.mem.write(one + 9, b".?AVA@@\0");
        e.mem.write(two + 9, b".?AVA@@\0");
        assert_eq!(e.call(0x00ec_40cc, &[one, two]).u32(), 1);
        e.mem.write(two + 9, b".?AVB@@\0");
        assert_eq!(e.call(0x00ec_40cc, &[one, two]).u32(), 0);
    }

    #[test]
    fn purecall_runs_the_handler_then_aborts() {
        let mut e = Engine::new();
        e.map(PURECALL_HANDLER, 4);
        e.map(ABORT_BEHAVIOR, 4);
        e.mem.set_u32(PURECALL_HANDLER, 0x5555);
        e.mem.set_u32(ABORT_BEHAVIOR, 3);
        e.register(DECODE_POINTER, |_, a| (a[0] + 0x0040_0000).into_ret());
        nothing(&mut e, &[0x0040_5555, NMSG_WRITE, ABORT]);
        e.call_log = Some(vec![]);
        e.call(0x00ec_60fa, &[]);
        assert_eq!(calls(&e, 0x0040_5555).len(), 1);
        assert_eq!(calls(&e, NMSG_WRITE), vec![vec![0x19]]);
        assert_eq!(calls(&e, SET_ABORT_BEHAVIOR), vec![vec![0, 1]]);
        assert_eq!(calls(&e, ABORT).len(), 1);
        assert_eq!(e.mem.u32(ABORT_BEHAVIOR), 2);
        // No handler: straight to the message.
        e.register(DECODE_POINTER, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00ec_60fa, &[]);
        assert!(calls(&e, 0x0040_5555).is_empty());
        assert_eq!(calls(&e, ABORT).len(), 1);
    }

    #[test]
    fn set_abort_behavior_answers_the_old_flags() {
        let mut e = Engine::new();
        e.map(ABORT_BEHAVIOR, 4);
        e.mem.set_u32(ABORT_BEHAVIOR, 0b1010);
        assert_eq!(e.call(SET_ABORT_BEHAVIOR, &[0b0101, 0b0011]).u32(), 0b1010);
        assert_eq!(e.mem.u32(ABORT_BEHAVIOR), 0b1001);
    }

    fn exception_engine() -> Engine {
        let mut e = Engine::new();
        e.register(STRLEN, |e, a| {
            let mut n = 0;
            while e.mem.u8(a[0] + n) != 0 {
                n += 1;
            }
            n.into_ret()
        });
        e.register(MALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(STRCPY_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[1]);
            e.mem.write(a[0], &bytes);
            0u32.into_ret()
        });
        nothing(&mut e, &[EXCEPTION_DTOR, OPERATOR_DELETE]);
        e
    }

    #[test]
    fn exceptions_copy_owned_texts_and_share_borrowed_ones() {
        let mut e = exception_engine();
        let message = text(&mut e, b"bad");
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, message);
        let a = e.mem.alloc(12);
        assert_eq!(e.call(EXCEPTION_CTOR, &[a, slot]).u32(), a);
        assert_eq!(e.mem.u32(a), EXCEPTION_VTABLE);
        let copy = e.mem.u32(a + 4);
        assert_ne!(copy, message);
        assert_eq!(e.mem.cstr(copy), b"bad");
        assert_eq!(e.mem.u32(a + 8), 1);
        assert_eq!(e.call(0x00ec_79b7, &[a]).u32(), copy);
        // A null text: none kept, still "owned".
        e.mem.set_u32(slot, 0);
        let b = e.mem.alloc(12);
        e.call(EXCEPTION_CTOR, &[b, slot]);
        assert_eq!((e.mem.u32(b + 4), e.mem.u32(b + 8)), (0, 1));
        assert_eq!(e.call(0x00ec_79b7, &[b]).u32(), UNKNOWN_EXCEPTION);
        // Copies: an owned text is duplicated, a borrowed one shared.
        let c = e.mem.alloc(12);
        e.call(EXCEPTION_COPY, &[c, a]);
        assert_ne!(e.mem.u32(c + 4), copy);
        assert_eq!(e.mem.cstr(e.mem.u32(c + 4)), b"bad");
        e.mem.set_u32(a + 8, 0);
        e.call(EXCEPTION_COPY, &[c, a]);
        assert_eq!((e.mem.u32(c + 4), e.mem.u32(c + 8)), (copy, 0));
    }

    #[test]
    fn derived_exceptions_set_their_vtables() {
        let mut e = exception_engine();
        let message = text(&mut e, b"typeid");
        let t = e.mem.alloc(12);
        assert_eq!(e.call(0x00ec_7a0a, &[t, message]).u32(), t);
        assert_eq!(e.mem.u32(t), BAD_TYPEID_VTABLE);
        assert_eq!(e.mem.cstr(e.mem.u32(t + 4)), b"typeid");
        let u = e.mem.alloc(12);
        assert_eq!(e.call(0x00ec_7a28, &[u, t]).u32(), u);
        assert_eq!(e.mem.u32(u), BAD_TYPEID_VTABLE);
        let x = e.mem.alloc(12);
        e.call(0x00ec_e2d2, &[x, message]);
        assert_eq!(e.mem.u32(x), BAD_EXCEPTION_VTABLE);
        e.call_log = Some(vec![]);
        for (addr, vtable) in [
            (0x00ec_7a8a, None),
            (0x00ec_7aab, Some(BAD_CAST_VTABLE)),
            (0x00ec_e2fb, Some(BAD_EXCEPTION_VTABLE)),
        ] {
            e.mem.set_u32(x, 0x1234);
            assert_eq!(e.call(addr, &[x, 0]).u32(), x);
            assert_eq!(e.mem.u32(x), vtable.unwrap_or(0x1234));
            assert_eq!(e.call(addr, &[x, 1]).u32(), x);
        }
        assert_eq!(calls(&e, EXCEPTION_DTOR).len(), 6);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![x]; 3]);
    }

    #[test]
    fn vprintf_helper_writes_to_stdout_under_its_lock() {
        let mut e = Engine::new();
        let errno = e.mem.alloc(4);
        e.register(IOB_FUNC, |_, _| 0x0500_0000u32.into_ret());
        e.register_double(ERRNO, move |_, _| errno.into_ret());
        e.register(STBUF, |_, _| 7u32.into_ret());
        e.register(0x0040_7000, |_, a| (a[1] + a[3]).into_ret());
        nothing(&mut e, &[INVALID_PARAMETER, LOCK_FILE, FTBUF, UNLOCK_FILE]);
        e.call_log = Some(vec![]);
        let r = e.call(0x00ec_bdd0, &[0x0040_7000, 0x100, 0, 0x20]).i32();
        assert_eq!(r, 0x120);
        let stream = 0x0500_0020;
        assert_eq!(calls(&e, LOCK_FILE), vec![vec![stream]]);
        assert_eq!(calls(&e, 0x0040_7000), vec![vec![stream, 0x100, 0, 0x20]]);
        assert_eq!(calls(&e, FTBUF), vec![vec![7, stream]]);
        assert_eq!(calls(&e, UNLOCK_FILE), vec![vec![stream]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_bdd0, &[0x0040_7000, 0, 0, 0]).i32(), -1);
        assert_eq!(e.mem.u32(errno), 0x16);
        assert_eq!(calls(&e, INVALID_PARAMETER), vec![vec![0; 5]]);
        assert!(calls(&e, LOCK_FILE).is_empty());
    }

    #[test]
    fn the_64_bit_seek_and_tell_lock_the_stream() {
        let mut e = Engine::new();
        e.register(FTELLI64_NOLOCK, |_, _| Ret {
            eax: 0x89ab_cdef,
            edx: 0x0123_4567,
            ..Ret::default()
        });
        e.register(FSEEKI64_NOLOCK, |_, _| 0u32.into_ret());
        nothing(&mut e, &[LOCK_FILE, UNLOCK_FILE]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_f08c, &[0x900]).u64(), 0x0123_4567_89ab_cdef);
        assert_eq!(e.call(0x00ed_f172, &[0x900, 5, 6, 1]).i32(), 0);
        assert_eq!(calls(&e, FSEEKI64_NOLOCK), vec![vec![0x900, 5, 6, 1]]);
        assert_eq!(calls(&e, LOCK_FILE), vec![vec![0x900]; 2]);
        assert_eq!(calls(&e, UNLOCK_FILE), vec![vec![0x900]; 2]);
    }

    #[test]
    fn the_finally_blocks_give_their_locks_back() {
        let mut e = Engine::new();
        nothing(&mut e, &[UNLOCK_FILE, UNLOCK]);
        e.call_log = Some(vec![]);
        for addr in [
            0x00ec_d4c6,
            0x00ec_d627,
            0x00ec_d72d,
            0x00ec_d8dc,
            0x00ec_f05a,
        ] {
            e.call(addr, &[0x700]);
        }
        e.call(0x00ed_1709, &[]);
        e.call(0x00ed_4145, &[]);
        assert_eq!(calls(&e, UNLOCK_FILE), vec![vec![0x700]; 5]);
        assert_eq!(calls(&e, UNLOCK), vec![vec![0xe], vec![0xd]]);
    }

    #[test]
    fn the_exit_paths_call_their_steps() {
        let mut e = Engine::new();
        e.map(ERROR_MODE, 4);
        nothing(
            &mut e,
            &[DOEXIT, FF_MSGBANNER, NMSG_WRITE, CRT_EXIT_PROCESS],
        );
        e.call_log = Some(vec![]);
        e.call(0x00ec_cef1, &[]);
        assert_eq!(calls(&e, DOEXIT), vec![vec![0, 0, 1]]);
        e.call(0x00ec_c334, &[8]);
        assert!(calls(&e, FF_MSGBANNER).is_empty());
        e.mem.set_u32(ERROR_MODE, 1);
        e.call(0x00ec_c334, &[9]);
        assert_eq!(calls(&e, FF_MSGBANNER).len(), 1);
        assert_eq!(calls(&e, NMSG_WRITE), vec![vec![8], vec![9]]);
        assert_eq!(calls(&e, CRT_EXIT_PROCESS), vec![vec![0xff]; 2]);
    }

    #[test]
    fn terminate_unexpected_and_inconsistency_run_their_handlers() {
        let mut e = Engine::new();
        let ptd = e.mem.alloc(0x80);
        e.mem.set_u32(ptd + 0x78, 0x0040_1111);
        e.mem.set_u32(ptd + 0x7c, 0x0040_2222);
        e.register_double(GETPTD, move |_, _| ptd.into_ret());
        e.map(INCONSISTENCY_HOOK, 4);
        e.register(DECODE_POINTER, |_, a| a[0].into_ret());
        nothing(&mut e, &[0x0040_1111, 0x0040_2222, 0x0040_3333, ABORT]);
        e.call_log = Some(vec![]);
        e.call(0x00ed_14b4, &[]);
        assert_eq!(calls(&e, 0x0040_1111).len(), 1);
        assert_eq!(calls(&e, ABORT).len(), 1);
        e.call(0x00ed_14ed, &[]);
        assert_eq!(calls(&e, 0x0040_2222).len(), 1);
        assert_eq!(calls(&e, 0x0040_1111).len(), 2);
        assert_eq!(calls(&e, ABORT).len(), 2);
        // No hook: `_inconsistency` goes straight to terminate.
        e.call(0x00ed_1500, &[]);
        assert!(calls(&e, 0x0040_3333).is_empty());
        e.mem.set_u32(INCONSISTENCY_HOOK, 0x0040_3333);
        e.call(0x00ed_1500, &[]);
        assert_eq!(calls(&e, 0x0040_3333).len(), 1);
        assert_eq!(calls(&e, TERMINATE).len(), 4);
        // No handler: terminate only aborts.
        e.mem.set_u32(ptd + 0x78, 0);
        e.call(0x00ed_14b4, &[]);
        assert_eq!(calls(&e, 0x0040_1111).len(), 4);
    }

    #[test]
    fn the_float_atan2_passes_quiet_copies_on() {
        let mut e = Engine::new();
        e.register(ATAN2_FLOAT_WORKER, |_, a| {
            let y = f32::from_bits(a[0]);
            let x = f32::from_bits(a[1]);
            Ret {
                st0: f64::from(y.atan2(x)),
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        let r = e.call(0x004b_5550, &args![1.0f32, 1.0f32]).f32();
        assert_eq!(r, 1.0f32.atan2(1.0));
        e.call(0x004b_5550, &[0x7f80_0001, 0]);
        assert_eq!(calls(&e, ATAN2_FLOAT_WORKER)[1], vec![0x7fc0_0001, 0]);
    }
}

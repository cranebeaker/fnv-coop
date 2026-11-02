//! More C runtime entry points the translated code reaches (MSVC 2008 LIBCMT, statically
//! linked into FalloutNV.exe 1.4.0.525), translated from the exe: `strtoul` (`00ec8d33`),
//! `__set_error_mode` (`00ee1dac`), `siglookup` (`00eddf8a`), `__validdrive` (`00ecca6c`),
//! `__getcwd` (`00eccbdc`), `__strupr_s_l` (`00ecbc26`), `_abort` (`00ecf335`), `raise`
//! (`00eddfce`), `__raise_exc_ex` (`00ed5255`), `__getdcwd_nolock` (`00eccaa5`), `__fullpath`
//! (`00ed707b`), `__fullpath_helper` (`00ec91dc`) and `_strupr_s_l_stat` (`00ecbaa8`).

use super::*;
use crate::x87::{x87_load32, x87_load64, x87_store32};

/// `strtoxl` (`00ec8ad9`; locale, text, end out, base, unsigned flag), `errno` (`00ec85e3`),
/// the invalid-parameter report (`00ec7c30`), `_lock` (`00eddf39`), `__getdcwd_nolock`
/// (`00eccaa5`; drive, target, size), `__getcwd`'s `__finally` block (`00eccc1c`),
/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`; `ECX` the object) and `_strupr_s_l_stat`
/// (`00ecbaa8`; `EDI` the text, then the size and the `_LocaleUpdate` object).
const STRTOXL: u32 = 0x00ec_8ad9;
const ERRNO: u32 = 0x00ec_85e3;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
const LOCK: u32 = 0x00ed_df39;
const GETDCWD_NOLOCK: u32 = 0x00ec_caa5;
const GETCWD_FINALLY: u32 = 0x00ec_cc1c;
const LOCALE_UPDATE: u32 = 0x00ec_660e;
const STRUPR_S_L_STAT: u32 = 0x00ec_baa8;
/// `__locale_changed` (`0126fdf4`), the initial locale (`__initiallocalestructinfo`,
/// `011c2400`), the error mode (`__error_mode`, `0126fdb4`) and the signal table's length
/// (`_XcptActTabCount`, `011c289c`).
const LOCALE_CHANGED: u32 = 0x0126_fdf4;
const INITIAL_LOCALE: u32 = 0x011c_2400;
const ERROR_MODE: u32 = 0x0126_fdb4;
const SIGNAL_TABLE_COUNT: u32 = 0x011c_289c;
/// The import slot of KERNEL32 `GetDriveTypeA` (`00fdf0e8`).
const GET_DRIVE_TYPE_A: u32 = 0x00fd_f0e8;

// Translated from 00ec8d33 (disassembled, FalloutNV.exe 1.4.0.525)
/// `strtoul` (Ghidra's function ID), cdecl: text, end out, base. `strtoxl` with the unsigned
/// flag, in the initial locale while `__locale_changed` is 0, else with no locale (the
/// thread's).
pub fn strtoul(e: &mut Engine, text: u32, end: u32, base: u32) -> u32 {
    let locale = if e.mem.u32(LOCALE_CHANGED) == 0 {
        INITIAL_LOCALE
    } else {
        0
    };
    e.call(STRTOXL, &args![locale, text, end, base, 1u32]).u32()
}

// Translated from 00ee1dac (disassembled, FalloutNV.exe 1.4.0.525)
/// `__set_error_mode` (Ghidra's function ID), cdecl, a mode: 0..2 sets the error mode and
/// answers the old one, 3 answers it unchanged; anything else (signed) sets `errno` to
/// `EINVAL`, reports and answers -1.
pub fn set_error_mode(e: &mut Engine, mode: u32) -> u32 {
    match mode as i32 {
        0..=2 => {
            let old = e.mem.u32(ERROR_MODE);
            e.mem.set_u32(ERROR_MODE, mode);
            old
        }
        3 => e.mem.u32(ERROR_MODE),
        _ => {
            let slot = e.call(ERRNO, &args![]).u32();
            e.mem.set_u32(slot, 0x16);
            e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
            0xffff_ffff
        }
    }
}

// Translated from 00eddf8a (disassembled, FalloutNV.exe 1.4.0.525)
/// `siglookup` (Ghidra's function ID), `EDX` the signal, the action table on the stack
/// (0xc-byte entries, the signal at +4, `_XcptActTabCount` of them): the first entry with the
/// signal, or null. The first entry is compared before the count is looked at.
pub fn siglookup(e: &mut Engine, signal: u32, table: u32) -> u32 {
    let count = e.mem.u32(SIGNAL_TABLE_COUNT);
    let end = count.wrapping_mul(0xc).wrapping_add(table);
    let mut entry = table;
    loop {
        if e.mem.u32(entry.wrapping_add(4)) == signal {
            break;
        }
        entry = entry.wrapping_add(0xc);
        if entry >= end {
            break;
        }
    }
    if entry < end && e.mem.u32(entry.wrapping_add(4)) == signal {
        entry
    } else {
        0
    }
}

// Translated from 00ecca6c (disassembled, FalloutNV.exe 1.4.0.525)
/// `__validdrive` (Ghidra's function ID), cdecl, a drive number (1 = A): 0 (the current
/// drive) is valid; otherwise its root (`"X:\"`, written over the argument's stack slot) is
/// valid unless `GetDriveTypeA` answers `DRIVE_UNKNOWN` (0) or `DRIVE_NO_ROOT_DIR` (1).
pub fn validdrive(e: &mut Engine, drive: u32) -> u32 {
    if drive == 0 {
        return 1;
    }
    e.with_stack(4, |e, root| {
        let root = root.addr();
        e.mem.set_u32(root, drive);
        e.mem.set_u8(root, (drive as u8).wrapping_add(0x40));
        e.mem.set_u8(root + 1, b':');
        e.mem.set_u8(root + 2, b'\\');
        e.mem.set_u8(root + 3, 0);
        let kind = e.call(GET_DRIVE_TYPE_A, &args![root]).u32();
        u32::from(kind != 0 && kind != 1)
    })
}

// Translated from 00eccbdc (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00eccbdc - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 0117fe90) is not translated; its __finally block (00eccc1c) is called as the exe calls it on the way out.
/// `__getcwd` (Ghidra's function ID), cdecl: target, size. Under lock 7,
/// `__getdcwd_nolock(0, target, size)` answers; the `__finally` block gives the lock back.
pub fn getcwd(e: &mut Engine, target: u32, size: u32) -> u32 {
    e.call(LOCK, &args![7u32]);
    let result = e.call(GETDCWD_NOLOCK, &args![0u32, target, size]).u32();
    e.call(GETCWD_FINALLY, &args![]);
    result
}

// Translated from 00ecbc26 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__strupr_s_l` (Ghidra's function ID), cdecl: text, size, locale. With a `_LocaleUpdate`
/// for the locale, `_strupr_s_l_stat(text, size, update)` answers; the update's destructor
/// clears bit 0x2 of the thread's `_ownlocale` when it was set.
pub fn strupr_s_l(e: &mut Engine, text: u32, size: u32, locale: u32) -> u32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let result = e.call(STRUPR_S_L_STAT, &args![text, size, update]).u32();
        if e.mem.u8(update.wrapping_add(0xc)) != 0 {
            let ptd = e.mem.u32(update.wrapping_add(8));
            let own = e.mem.u32(ptd.wrapping_add(0x70));
            e.mem.set_u32(ptd.wrapping_add(0x70), own & 0xffff_fffd);
        }
        result
    })
}

/// `_NMSG_WRITE` (`00ed290c`), `___get_sigabrt` (`00eddfc1`), `raise` (`00eddfce`), `_exit`
/// (`00eccedb`), `memset` (`00ec61c0`), `_getptd_noexit` (`00ed1165`), `siglookup`
/// (`00eddf8a`; `EDX` the signal), `__decode_pointer` (`00ed0f92`), `__encoded_null`
/// (`00ed0f89`), `_unlock`-guarded `__finally` of `raise` (`00ede142`).
const NMSG_WRITE: u32 = 0x00ed_290c;
const GET_SIGABRT: u32 = 0x00ed_dfc1;
const RAISE: u32 = 0x00ed_dfce;
const EXIT_3: u32 = 0x00ec_cedb;
const MEMSET: u32 = 0x00ec_61c0;
const GETPTD_NOEXIT: u32 = 0x00ed_1165;
const SIGLOOKUP: u32 = 0x00ed_df8a;
const DECODE_POINTER: u32 = 0x00ed_0f92;
const ENCODED_NULL: u32 = 0x00ed_0f89;
const RAISE_FINALLY: u32 = 0x00ed_e142;
/// The import slots of KERNEL32 `SetUnhandledExceptionFilter` (`00fdf180`) and
/// `UnhandledExceptionFilter` (`00fdf17c`).
const SET_UNHANDLED_EXCEPTION_FILTER: u32 = 0x00fd_f180;
const UNHANDLED_EXCEPTION_FILTER: u32 = 0x00fd_f17c;
/// `__abort_behavior` (`011c1cac`: 1 writes the message, 2 reports to Windows), the
/// encoded global handlers of `SIGINT` (`01270710`), `SIGBREAK` (`01270714`), `SIGABRT`
/// (`01270718`) and `SIGTERM` (`0127071c`), and the floating-point entries of the action
/// table (`_First_FPE_Indx`, `011c2890`; `_Num_FPE`, `011c2894`).
const ABORT_BEHAVIOR: u32 = 0x011c_1cac;
const CTRLC_HANDLER: u32 = 0x0127_0710;
const CTRLBREAK_HANDLER: u32 = 0x0127_0714;
const ABORT_HANDLER: u32 = 0x0127_0718;
const TERM_HANDLER: u32 = 0x0127_071c;
const FIRST_FPE_INDEX: u32 = 0x011c_2890;
const FPE_COUNT: u32 = 0x011c_2894;

// Translated from 00ecf335 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecf335 - the CONTEXT's registers (ECX, EDX, EBX, ESI, EDI, EBP, EIP and the record's address from the return address, the segment registers and EFLAGS) are machine state the engine does not have and stay zero; the /GS cookie check is unreachable after _exit and not translated
/// `_abort` (Ghidra's function ID), cdecl. With bit 1 of `__abort_behavior` the abort
/// message (10) is written; a `SIGABRT` handler (`___get_sigabrt`) gets `raise(SIGABRT)`.
/// With bit 2 an `EXCEPTION_RECORD` (code `0x40000015`, the rest cleared) and a `CONTEXT`
/// (`ContextFlags` `0x10001`, `Eax` the handler, `Esp` the frame's `EBP + 4`) are built in a
/// 0x328-byte frame and passed to `UnhandledExceptionFilter` after
/// `SetUnhandledExceptionFilter(NULL)`. Then `_exit(3)`, which does not come back: the
/// engine stops there with a panic.
pub fn abort(e: &mut Engine) {
    if e.mem.u8(ABORT_BEHAVIOR) & 1 != 0 {
        e.call(NMSG_WRITE, &args![10u32]);
    }
    let handler = e.call(GET_SIGABRT, &args![]).u32();
    if handler != 0 {
        e.call(RAISE, &args![0x16u32]);
    }
    if e.mem.u8(ABORT_BEHAVIOR) & 2 != 0 {
        // The frame below EBP: the record (EBP-0x328), the pointers (EBP-0x2d8) and the
        // CONTEXT (EBP-0x2d0).
        let frame = e.mem.alloc(0x328 + 8);
        let ebp = frame + 0x328;
        let (record, pointers, context) = (frame, frame + 0x50, frame + 0x58);
        e.mem.set_u32(context + 0xb0, handler); // Eax
        e.mem.set_u32(context + 0xc4, ebp + 4); // Esp
        e.mem.set_u32(context, 0x0001_0001); // ContextFlags
        e.call(MEMSET, &args![record, 0u32, 0x50u32]);
        e.mem.set_u32(pointers, record);
        e.mem.set_u32(record, 0x4000_0015);
        e.mem.set_u32(pointers + 4, context);
        e.call(SET_UNHANDLED_EXCEPTION_FILTER, &args![0u32]);
        e.call(UNHANDLED_EXCEPTION_FILTER, &args![pointers]);
    }
    e.call(EXIT_3, &args![3u32]);
    panic!("00ecf335 _abort: _exit(3) ended the process");
}

// Translated from 00eddfce (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00eddfce - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 01180428) is not translated; its __finally block (00ede142) is called as the exe calls it on the way out.
/// `raise` (Ghidra's function ID), cdecl, a signal. `SIGINT`, `SIGBREAK`, `SIGABRT` (and its
/// old number 6) and `SIGTERM` use the global handlers (decoded, under lock 0);
/// `SIGFPE`, `SIGILL` and `SIGSEGV` the thread's action table (`_getptd_noexit`, none
/// answers -1; `siglookup`). Any other signal sets `errno` to `EINVAL`, reports and answers
/// -1. `SIG_IGN` (1) answers 0; `SIG_DFL` (0) ends the process (`_exit(3)`). For the
/// thread's signals the exception pointers (+0x60) are saved and cleared, for `SIGFPE` the
/// code (+0x64) too (set to 0x8c) and every floating-point entry's handler is reset;
/// otherwise the handler's word becomes the encoded null. After the `__finally` block the
/// handler runs (`SIGFPE` with the code as a second argument) and what was saved is put
/// back. 0 is answered.
pub fn raise(e: &mut Engine, signal: u32) -> u32 {
    let sig = signal as i32;
    let mut locked = 0u32;
    let mut ptd = 0u32;
    let global = match sig {
        2 => Some(CTRLC_HANDLER),
        6 | 0x16 => Some(ABORT_HANDLER),
        0x15 => Some(CTRLBREAK_HANDLER),
        0xf => Some(TERM_HANDLER),
        4 | 8 | 0xb => None,
        _ => {
            let slot = e.call(0x00ec_85e3, &args![]).u32();
            e.mem.set_u32(slot, 0x16);
            e.call(0x00ec_7c30, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
            return 0xffff_ffff;
        }
    };
    let (slot, handler) = match global {
        Some(slot) => {
            locked = 1;
            let encoded = e.mem.u32(slot);
            (slot, e.call(DECODE_POINTER, &args![encoded]).u32())
        }
        None => {
            ptd = e.call(GETPTD_NOEXIT, &args![]).u32();
            if ptd == 0 {
                return 0xffff_ffff;
            }
            let table = e.mem.u32(ptd.wrapping_add(0x5c));
            let entry = e.call(SIGLOOKUP, &args![signal, table]).u32();
            let slot = entry.wrapping_add(8);
            (slot, e.mem.u32(slot))
        }
    };
    if handler == 1 {
        return 0;
    }
    if handler == 0 {
        e.call(EXIT_3, &args![3u32]);
    }
    if locked != 0 {
        e.call(0x00ed_df39, &args![0u32]);
    }
    let thread_signal = matches!(sig, 8 | 0xb | 4);
    let (mut saved_pointers, mut saved_code) = (0, 0);
    if thread_signal {
        saved_pointers = e.mem.u32(ptd.wrapping_add(0x60));
        e.mem.set_u32(ptd.wrapping_add(0x60), 0);
        if sig == 8 {
            saved_code = e.mem.u32(ptd.wrapping_add(0x64));
            e.mem.set_u32(ptd.wrapping_add(0x64), 0x8c);
        }
    }
    if sig == 8 {
        let mut index = e.mem.u32(FIRST_FPE_INDEX);
        loop {
            let end = e
                .mem
                .u32(FPE_COUNT)
                .wrapping_add(e.mem.u32(FIRST_FPE_INDEX));
            if (index as i32) >= (end as i32) {
                break;
            }
            let table = e.mem.u32(ptd.wrapping_add(0x5c));
            e.mem.set_u32(
                index.wrapping_mul(0xc).wrapping_add(table).wrapping_add(8),
                0,
            );
            index = index.wrapping_add(1);
        }
    } else {
        let null = e.call(ENCODED_NULL, &args![]).u32();
        e.mem.set_u32(slot, null);
    }
    e.call(RAISE_FINALLY, &args![locked]);
    if sig == 8 {
        let code = e.mem.u32(ptd.wrapping_add(0x64));
        e.call(handler, &args![signal, code]);
    } else {
        e.call(handler, &args![signal]);
    }
    if thread_signal {
        e.mem.set_u32(ptd.wrapping_add(0x60), saved_pointers);
        if sig == 8 {
            e.mem.set_u32(ptd.wrapping_add(0x64), saved_code);
        }
    }
    0
}

/// `__statfp` (`00ed5a9a`), `__clrfp` (`00ed5aaa`) and the import slot of KERNEL32
/// `RaiseException` (`00fdf154`).
const STATFP: u32 = 0x00ed_5a9a;
const CLRFP: u32 = 0x00ed_5aaa;
const RAISE_EXCEPTION: u32 = 0x00fd_f154;

/// `word ^= ((!source) ^ word) & mask` on the record word at `at`: the bit(s) of `mask`
/// become the complement of `source`'s.
fn set_from_complement(e: &mut Engine, at: u32, source: u32, mask: u32) {
    let word = e.mem.u32(at);
    e.mem.set_u32(at, word ^ ((!source ^ word) & mask));
}

// Translated from 00ed5255 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__raise_exc_ex` (Ghidra's function ID), cdecl: the `_FPIEEE_RECORD`, the control word's
/// address, the exception flags, the operation code, the operand's and the result's
/// addresses, and whether they are floats. The record's cause, enable and status words are
/// cleared; each flag (inexact 0x10, underflow 2, overflow 1, divide-by-zero 4, invalid 8)
/// sets its cause bit and picks the exception code (the last one wins). The enable bits are
/// the control word's masks inverted; the status bits come from `__statfp`; the rounding
/// mode and precision are copied from the control word, the operation code stored at bit 5.
/// The operand and result are stored (as floats or doubles, through the x87: a signalling
/// NaN is quieted) with their formats, the status cleared (`__clrfp`) and
/// `RaiseException(code, 0, 1, &record)` raised. Afterwards the control word takes the
/// record's enable bits (each set one unmasks its exception), rounding mode and precision
/// (the precision cases also clear the rounding bits, as the exe's mask `0xfffff3ff` does),
/// and the result is stored back from the record.
#[allow(clippy::too_many_arguments)]
pub fn raise_exc_ex(
    e: &mut Engine,
    record: u32,
    control: u32,
    flags: u32,
    operation: u32,
    operand: u32,
    result: u32,
    is_float: u32,
) {
    let cause = record.wrapping_add(4);
    let enable = record.wrapping_add(8);
    let status = record.wrapping_add(0xc);
    e.mem.set_u32(cause, 0);
    e.mem.set_u32(enable, 0);
    e.mem.set_u32(status, 0);
    let mut code = flags;
    for (flag, bit, exception) in [
        (0x10u32, 1u32, 0xc000_008fu32),
        (2, 2, 0xc000_0093),
        (1, 4, 0xc000_0091),
        (4, 8, 0xc000_008e),
        (8, 0x10, 0xc000_0090),
    ] {
        if flags as u8 & flag as u8 != 0 {
            let word = e.mem.u32(cause);
            e.mem.set_u32(cause, word | bit);
            code = exception;
        }
    }
    let word = e.mem.u32(control);
    set_from_complement(e, enable, word << 4, 0x10);
    set_from_complement(e, enable, word.wrapping_add(word), 8);
    set_from_complement(e, enable, word >> 1, 4);
    set_from_complement(e, enable, word >> 3, 2);
    set_from_complement(e, enable, word >> 5, 1);
    let fpu_status = e.call(STATFP, &args![]).u32() as u8;
    for (bit, value) in [(1u8, 0x10u32), (4, 8), (8, 4), (0x10, 2), (0x20, 1)] {
        if fpu_status & bit != 0 {
            let w = e.mem.u32(status);
            e.mem.set_u32(status, w | value);
        }
    }
    let first = e.mem.u32(record);
    let first = match word & 0xc00 {
        0 => first & 0xffff_fffc,
        0x400 => (first & 0xffff_fffd) | 1,
        0x800 => (first & 0xffff_fffe) | 2,
        _ => first | 3,
    };
    let first = match word & 0x300 {
        0 => (first & 0xffff_ffeb) | 8,
        0x200 => (first & 0xffff_ffe7) | 4,
        0x300 => first & 0xffff_ffe3,
        _ => first,
    };
    e.mem
        .set_u32(record, first ^ ((operation << 5 ^ first) & 0x1_ffe0));
    let operand_format = record.wrapping_add(0x20);
    let result_format = record.wrapping_add(0x60);
    let w = e.mem.u32(operand_format);
    e.mem.set_u32(operand_format, w | 1);
    if is_float != 0 {
        let w = e.mem.u32(operand_format);
        e.mem.set_u32(operand_format, w & 0xffff_ffe1);
        let value = x87_load32(e.mem.u32(operand));
        e.mem.set_u32(record.wrapping_add(0x10), x87_store32(value));
        let w = e.mem.u32(result_format);
        e.mem.set_u32(result_format, w | 1);
        let w = e.mem.u32(result_format);
        e.mem.set_u32(result_format, w & 0xffff_ffe1);
        let value = x87_load32(e.mem.u32(result));
        e.mem.set_u32(record.wrapping_add(0x50), x87_store32(value));
    } else {
        let w = e.mem.u32(operand_format);
        e.mem.set_u32(operand_format, (w & 0xffff_ffe3) | 2);
        let value = x87_load64(e.mem.u64(operand));
        e.mem.set_u64(record.wrapping_add(0x10), value.to_bits());
        let w = e.mem.u32(result_format);
        e.mem.set_u32(result_format, w | 1);
        let w = e.mem.u32(result_format);
        e.mem.set_u32(result_format, (w & 0xffff_ffe3) | 2);
        let value = x87_load64(e.mem.u64(result));
        e.mem.set_u64(record.wrapping_add(0x50), value.to_bits());
    }
    e.call(CLRFP, &args![]);
    // The argument array is the record's own stack slot (`LEA EAX, [EBP+8]`).
    let record = e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, record);
        e.call(RAISE_EXCEPTION, &args![code, 0u32, 1u32, slot]);
        e.mem.u32(slot)
    });
    let enable = e.mem.u8(record.wrapping_add(8));
    for (bit, unmask) in [(0x10u8, 1u32), (8, 4), (4, 8), (2, 0x10), (1, 0x20)] {
        if enable & bit != 0 {
            let w = e.mem.u32(control);
            e.mem.set_u32(control, w & !unmask);
        }
    }
    let first = e.mem.u32(record);
    let w = e.mem.u32(control);
    let w = match first & 3 {
        0 => w & 0xffff_f3ff,
        1 => (w & 0xffff_f7ff) | 0x400,
        2 => (w & 0xffff_fbff) | 0x800,
        _ => w | 0xc00,
    };
    e.mem.set_u32(control, w);
    let w = e.mem.u32(control);
    let w = match (first >> 2) & 7 {
        0 => (w & 0xffff_f3ff) | 0x300,
        1 => (w & 0xffff_f3ff) | 0x200,
        2 => w & 0xffff_f3ff,
        _ => w,
    };
    e.mem.set_u32(control, w);
    if is_float != 0 {
        let value = x87_load32(e.mem.u32(record.wrapping_add(0x50)));
        e.mem.set_u32(result, x87_store32(value));
    } else {
        let value = x87_load64(e.mem.u64(record.wrapping_add(0x50)));
        e.mem.set_u64(result, value.to_bits());
    }
}

/// `__getdcwd_nolock`'s and `__fullpath`'s helpers: `__doserrno` (`00ec85f6`),
/// `_dosmaperr` (`00ec8609`), `_calloc` (`00eddd7d`), `free` (`00ecd291`), `__getdrive`
/// (`00ed74ad`), `__validdrive` (`00ecca6c`), `__fullpath` (`00ed707b`), and the import
/// slots of KERNEL32 `GetFullPathNameA` (`00fdf194`) and `GetLastError` (`00fdf094`).
const DOSERRNO: u32 = 0x00ec_85f6;
const DOSMAPERR: u32 = 0x00ec_8609;
const CALLOC: u32 = 0x00ed_dd7d;
const FREE_FN: u32 = 0x00ec_d291;
const GETDRIVE: u32 = 0x00ed_74ad;
const VALIDDRIVE: u32 = 0x00ec_ca6c;
const FULLPATH: u32 = 0x00ed_707b;
const GETCWD: u32 = 0x00ec_cbdc;
const GET_FULL_PATH_NAME_A: u32 = 0x00fd_f194;
const GET_LAST_ERROR_SLOT: u32 = 0x00fd_f094;

fn errno_set(e: &mut Engine, code: u32) {
    let slot = e.call(ERRNO, &args![]).u32();
    e.mem.set_u32(slot, code);
}

fn map_last_error(e: &mut Engine) {
    let error = e.call(GET_LAST_ERROR_SLOT, &args![]).u32();
    e.call(DOSMAPERR, &args![error]);
}

// Translated from 00eccaa5 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__getdcwd_nolock` (Ghidra's function ID), cdecl: drive (0 the current one, 1 = A),
/// target (null: a new block), size. An invalid drive sets `_doserrno` to
/// `ERROR_INVALID_DRIVE` and `errno` to `EACCES`, reports and answers null; a target with a
/// size of 0 or below `EINVAL`. The directory is `GetFullPathNameA` of `"X:."` (or `"."`
/// for the current drive's number 0): into the target, which must hold it (else `ERANGE`,
/// emptied); without one into a `_calloc` block of the larger of the size and the needed
/// length (none: `ENOMEM`, `_doserrno` 8). A failing or short second call maps
/// `GetLastError` (`_dosmaperr`) and answers null (the block is not freed).
pub fn getdcwd_nolock(e: &mut Engine, drive: u32, target: u32, size: u32) -> u32 {
    let number = if drive != 0 {
        if e.call(VALIDDRIVE, &args![drive]).u32() == 0 {
            let slot = e.call(DOSERRNO, &args![]).u32();
            e.mem.set_u32(slot, 0xf);
            errno_set(e, 0xd);
            e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
            return 0;
        }
        drive
    } else {
        e.call(GETDRIVE, &args![]).u32()
    };
    let room = if target != 0 {
        if (size as i32) <= 0 {
            errno_set(e, 0x16);
            e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
            return 0;
        }
        e.mem.set_u8(target, 0);
        size
    } else {
        0
    };
    // The frame: the path text (EBP-0x4, four bytes) and the file-part word (EBP-0x8).
    e.with_stack(8, |e, frame| {
        let file_part = frame.addr();
        let text = file_part + 4;
        if number != 0 {
            e.mem.set_u8(text, (number as u8).wrapping_add(0x40));
            e.mem.set_u8(text + 1, b':');
            e.mem.set_u8(text + 2, b'.');
            e.mem.set_u8(text + 3, 0);
        } else {
            e.mem.set_u8(text, b'.');
            e.mem.set_u8(text + 1, 0);
        }
        let needed = e
            .call(GET_FULL_PATH_NAME_A, &args![text, room, target, file_part])
            .u32();
        if needed == 0 {
            map_last_error(e);
            return 0;
        }
        if target != 0 {
            if (needed as i32) < (room as i32) {
                return target;
            }
            errno_set(e, 0x22);
            e.mem.set_u8(target, 0);
            return 0;
        }
        let size = if (needed as i32) > (size as i32) {
            needed
        } else {
            size
        };
        let block = e.call(CALLOC, &args![size, 1u32]).u32();
        if block == 0 {
            errno_set(e, 0xc);
            let slot = e.call(DOSERRNO, &args![]).u32();
            e.mem.set_u32(slot, 8);
            return 0;
        }
        let written = e
            .call(GET_FULL_PATH_NAME_A, &args![text, size, block, file_part])
            .u32();
        if written != 0 && (written as i32) < (size as i32) {
            return block;
        }
        map_last_error(e);
        0
    })
}

// Translated from 00ed707b (disassembled, FalloutNV.exe 1.4.0.525)
/// `__fullpath` (Ghidra's function ID), cdecl: target (null: a new block), relative path,
/// size. No path, or an empty one, is the current directory (`__getcwd`). Without a target
/// the needed length (`GetFullPathNameA` with no buffer; 0 maps `GetLastError`) or the size,
/// whichever is larger, is allocated (`_calloc`; none: `ENOMEM`); a target needs a size
/// (else `EINVAL`, reported). The full path is written there: not fitting is `ERANGE`, a
/// failure maps `GetLastError`, either freeing the block made here; else the buffer.
pub fn fullpath(e: &mut Engine, target: u32, path: u32, size: u32) -> u32 {
    if path == 0 || e.mem.u8(path) == 0 {
        return e.call(GETCWD, &args![target, size]).u32();
    }
    let (buffer, room) = if target == 0 {
        let needed = e
            .call(GET_FULL_PATH_NAME_A, &args![path, 0u32, 0u32, 0u32])
            .u32();
        if needed == 0 {
            map_last_error(e);
            return 0;
        }
        let room = size.max(needed);
        let block = e.call(CALLOC, &args![room, 1u32]).u32();
        if block == 0 {
            errno_set(e, 0xc);
            return 0;
        }
        (block, room)
    } else {
        if size == 0 {
            errno_set(e, 0x16);
            e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
            return 0;
        }
        (target, size)
    };
    let written = e.with_stack(4, |e, file_part| {
        e.call(
            GET_FULL_PATH_NAME_A,
            &args![path, room, buffer, file_part.addr()],
        )
        .u32()
    });
    if written >= room {
        if target == 0 {
            e.call(FREE_FN, &args![buffer]);
        }
        errno_set(e, 0x22);
        return 0;
    }
    if written == 0 {
        if target == 0 {
            e.call(FREE_FN, &args![buffer]);
        }
        map_last_error(e);
        return 0;
    }
    buffer
}

// Translated from 00ec91dc (disassembled, FalloutNV.exe 1.4.0.525)
/// `__fullpath_helper` (Ghidra's function ID), cdecl: target, path, size, an out word.
/// `errno` is saved and cleared around `__fullpath`; success restores it and answers the
/// buffer. A failure other than `ERANGE` answers null (leaving that `errno`); `ERANGE`
/// restores `errno` and asks again for a new block, stored in the out word and answered.
pub fn fullpath_helper(e: &mut Engine, target: u32, path: u32, size: u32, out: u32) -> u32 {
    let slot = e.call(ERRNO, &args![]).u32();
    let saved = e.mem.u32(slot);
    let slot = e.call(ERRNO, &args![]).u32();
    e.mem.set_u32(slot, 0);
    let result = e.call(FULLPATH, &args![target, path, size]).u32();
    let slot = e.call(ERRNO, &args![]).u32();
    if result != 0 {
        e.mem.set_u32(slot, saved);
        return result;
    }
    if e.mem.u32(slot) != 0x22 {
        return 0;
    }
    let slot = e.call(ERRNO, &args![]).u32();
    e.mem.set_u32(slot, saved);
    let block = e.call(FULLPATH, &args![0u32, path, 0u32]).u32();
    e.mem.set_u32(out, block);
    block
}

/// `_strnlen` (`00ed5d55`), `___crtLCMapStringA` (`00ed0883`; nine words), `strcpy_s`
/// (`00ec65a6`), `__freea` (`00ec8e14`) and the heap allocation `_malloca` falls back on
/// (`00ecd1c7`).
const STRNLEN: u32 = 0x00ed_5d55;
const CRT_LC_MAP_STRING_A: u32 = 0x00ed_0883;
const STRCPY_S: u32 = 0x00ec_65a6;
const FREEA: u32 = 0x00ec_8e14;
const MALLOC: u32 = 0x00ec_d1c7;

// Translated from 00ecbaa8 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecbaa8 - the /GS stack cookie check (00ec408c) is not translated
/// `_strupr_s_l_stat` (Ghidra's function ID), `EDI` the text, then its size and the
/// `_LocaleUpdate` object. No text, or no end within the size (`_strnlen`; the text is
/// emptied), is `EINVAL` (reported, answered). Without a `LC_CTYPE` handle `a`..`z` become
/// upper case in place and 0 is answered. Otherwise `___crtLCMapStringA` (`LCMAP_UPPERCASE`,
/// the locale's code page, failing on invalid characters) gives the length (0: `EILSEQ`,
/// answered), which must fit the size (else emptied, `ERANGE`), maps into a `_malloca`
/// buffer (none: `ENOMEM`) that `strcpy_s` copies back (a failed map is `EILSEQ`), and the
/// buffer is released (`__freea`).
pub fn strupr_s_l_stat(e: &mut Engine, text: u32, size: u32, update: u32) -> u32 {
    let einval = |e: &mut Engine, code: u32| {
        let slot = e.call(ERRNO, &args![]).u32();
        e.mem.set_u32(slot, code);
        e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        code
    };
    let errno_value = |e: &mut Engine| {
        let slot = e.call(ERRNO, &args![]).u32();
        e.mem.u32(slot)
    };
    if text == 0 {
        return einval(e, 0x16);
    }
    if e.call(STRNLEN, &args![text, size]).u32() >= size {
        e.mem.set_u8(text, 0);
        return einval(e, 0x16);
    }
    let info = e.mem.u32(update);
    let lcid = e.mem.u32(info.wrapping_add(0x14));
    if lcid == 0 {
        let mut at = text;
        while e.mem.u8(at) != 0 {
            let c = e.mem.u8(at);
            if (c as i8) >= 0x61 && (c as i8) <= 0x7a {
                e.mem.set_u8(at, c - 0x20);
            }
            at = at.wrapping_add(1);
        }
        return 0;
    }
    let code_page = e.mem.u32(info.wrapping_add(4));
    let needed = e
        .call(
            CRT_LC_MAP_STRING_A,
            &args![
                update,
                lcid,
                0x200u32,
                text,
                0xffff_ffffu32,
                0u32,
                0u32,
                code_page,
                1u32
            ],
        )
        .u32();
    if needed == 0 {
        let slot = e.call(ERRNO, &args![]).u32();
        e.mem.set_u32(slot, 0x2a);
        return errno_value(e);
    }
    if size < needed {
        e.mem.set_u8(text, 0);
        return einval(e, 0x22);
    }
    let fits = (needed as i32) > 0 && 0xffff_ffe0 / needed >= 1;
    let mut stack_block = None;
    let buffer = if !fits {
        0
    } else if needed.wrapping_add(8) <= 0x400 {
        // `_alloca` through the probe `00ec5ec0`: a stack block here.
        let block = e.mem.alloc(needed.wrapping_add(8));
        e.mem.set_u32(block, 0xcccc);
        stack_block = Some(block);
        block + 8
    } else {
        let block = e.call(MALLOC, &args![needed.wrapping_add(8)]).u32();
        if block == 0 {
            0
        } else {
            e.mem.set_u32(block, 0xdddd);
            block + 8
        }
    };
    if buffer == 0 {
        let slot = e.call(ERRNO, &args![]).u32();
        e.mem.set_u32(slot, 0xc);
        return errno_value(e);
    }
    let info = e.mem.u32(update);
    let lcid = e.mem.u32(info.wrapping_add(0x14));
    let code_page = e.mem.u32(info.wrapping_add(4));
    let mapped = e
        .call(
            CRT_LC_MAP_STRING_A,
            &args![
                update,
                lcid,
                0x200u32,
                text,
                0xffff_ffffu32,
                buffer,
                needed,
                code_page,
                1u32
            ],
        )
        .u32();
    let result = if mapped != 0 {
        e.call(STRCPY_S, &args![text, size, buffer]).u32()
    } else {
        let slot = e.call(ERRNO, &args![]).u32();
        e.mem.set_u32(slot, 0x2a);
        0x2a
    };
    e.call(FREEA, &args![buffer]);
    if let Some(block) = stack_block {
        // The frame ends: its stack block goes away.
        e.mem.free(block);
    }
    result
}

/// The registry entry of [`strupr_s_l_stat`]: `EDI` first.
fn strupr_s_l_stat_entry(e: &mut Engine, words: &[u32]) -> Ret {
    strupr_s_l_stat(e, words[0], words[1], words[2]).into_ret()
}

/// The registry entry of [`siglookup`]: `EDX` first.
fn siglookup_entry(e: &mut Engine, words: &[u32]) -> Ret {
    siglookup(e, words[0], words[1]).into_ret()
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_8d33, strtoul(u32, u32, u32) -> u32),
        entry!(0x00ee_1dac, set_error_mode(u32) -> u32),
        (0x00ed_df8a, siglookup_entry as AbiFn),
        entry!(0x00ec_ca6c, validdrive(u32) -> u32),
        entry!(0x00ec_cbdc, getcwd(u32, u32) -> u32),
        entry!(0x00ec_bc26, strupr_s_l(u32, u32, u32) -> u32),
        (0x00ec_baa8, strupr_s_l_stat_entry as AbiFn),
        entry!(0x00ec_f335, abort()),
        entry!(0x00ed_dfce, raise(u32) -> u32),
        entry!(0x00ed_5255, raise_exc_ex(u32, u32, u32, u32, u32, u32, u32)),
        entry!(0x00ec_caa5, getdcwd_nolock(u32, u32, u32) -> u32),
        entry!(0x00ed_707b, fullpath(u32, u32, u32) -> u32),
        entry!(0x00ec_91dc, fullpath_helper(u32, u32, u32, u32) -> u32),
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

    #[test]
    fn strtoul_picks_the_locale_and_asks_for_unsigned() {
        let mut e = Engine::new();
        e.register_double(STRTOXL, |_, a| (a[0] ^ a[4]).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ec_8d33, &args![1u32, 2u32, 16u32]).u32(),
            INITIAL_LOCALE ^ 1
        );
        e.mem.set_u32(LOCALE_CHANGED, 1);
        assert_eq!(strtoul(&mut e, 1, 2, 16), 1);
        assert_eq!(calls(&e, STRTOXL)[1], vec![0, 1, 2, 16, 1]);
    }

    #[test]
    fn set_error_mode_sets_reads_or_refuses() {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| slot.into_ret());
        e.register_double(INVALID_PARAMETER, |_, _| Ret::default());
        e.mem.map(ERROR_MODE, 4);
        assert_eq!(e.call(0x00ee_1dac, &args![2u32]).u32(), 0);
        assert_eq!(set_error_mode(&mut e, 1), 2);
        assert_eq!(set_error_mode(&mut e, 3), 1);
        assert_eq!(set_error_mode(&mut e, 4), 0xffff_ffff);
        assert_eq!(set_error_mode(&mut e, 0xffff_ffff), 0xffff_ffff);
        assert_eq!(e.mem.u32(slot), 0x16);
        assert_eq!(e.mem.u32(ERROR_MODE), 1);
    }

    #[test]
    fn siglookup_finds_the_signal_entry() {
        let mut e = Engine::new();
        e.mem.map(SIGNAL_TABLE_COUNT, 4);
        e.mem.set_u32(SIGNAL_TABLE_COUNT, 3);
        let table = e.mem.alloc(0x30);
        for (i, signal) in [8u32, 4, 8].into_iter().enumerate() {
            e.mem.set_u32(table + i as u32 * 0xc + 4, signal);
        }
        assert_eq!(e.call(0x00ed_df8a, &args![8u32, table]).u32(), table);
        assert_eq!(siglookup(&mut e, 4, table), table + 0xc);
        assert_eq!(siglookup(&mut e, 2, table), 0);
        // With no entries the first is still compared, but not answered.
        e.mem.set_u32(SIGNAL_TABLE_COUNT, 0);
        assert_eq!(siglookup(&mut e, 8, table), 0);
    }

    #[test]
    fn validdrive_asks_for_the_root_type() {
        let mut e = Engine::new();
        e.register_double(GET_DRIVE_TYPE_A, |e, a| match e.mem.cstr(a[0]).as_slice() {
            b"C:\\" => 3u32.into_ret(),
            b"D:\\" => 1u32.into_ret(),
            _ => 0u32.into_ret(),
        });
        assert_eq!(e.call(0x00ec_ca6c, &args![0u32]).u32(), 1);
        assert_eq!(validdrive(&mut e, 3), 1);
        assert_eq!(validdrive(&mut e, 4), 0);
        assert_eq!(validdrive(&mut e, 26), 0);
    }

    #[test]
    fn getcwd_and_strupr_s_l_wrap_their_bodies() {
        let mut e = Engine::new();
        e.register_double(LOCK, |_, _| Ret::default());
        e.register_double(GETCWD_FINALLY, |_, _| Ret::default());
        e.register_double(GETDCWD_NOLOCK, |_, a| (a[1] + a[2]).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_cbdc, &args![0x100u32, 0x20u32]).u32(), 0x120);
        assert_eq!(calls(&e, GETDCWD_NOLOCK), vec![vec![0, 0x100, 0x20]]);
        assert_eq!(calls(&e, LOCK), vec![vec![7]]);
        assert_eq!(calls(&e, GETCWD_FINALLY).len(), 1);
        let ptd = e.mem.alloc(0x80);
        e.mem.set_u32(ptd + 0x70, 0xff);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0] + 8, ptd);
            e.mem.set_u8(a[0] + 0xc, 1);
            a[0].into_ret()
        });
        e.register_double(STRUPR_S_L_STAT, |_, a| (a[0] + a[1]).into_ret());
        assert_eq!(strupr_s_l(&mut e, 0x10, 0x20, 0), 0x30);
        assert_eq!(e.mem.u32(ptd + 0x70), 0xfd);
    }

    /// An engine for `raise`/`_abort`: pointers decoded by XOR with 0x5a5a5a5a, the encoded
    /// null that key, the locks, `errno`, the report and `_exit` doubled, the call log on.
    fn signal_engine() -> Engine {
        let mut e = Engine::new();
        e.register_double(DECODE_POINTER, |_, a| (a[0] ^ 0x5a5a_5a5a).into_ret());
        e.register_double(ENCODED_NULL, |_, _| 0x5a5a_5a5au32.into_ret());
        for addr in [0x00ed_df39, 0x00ed_de5f, 0x00ec_7c30, EXIT_3, NMSG_WRITE] {
            e.register_double(addr, |_, _| Ret::default());
        }
        let slot = e.mem.alloc(4);
        e.register_double(0x00ec_85e3, move |_, _| slot.into_ret());
        e.mem.map(ABORT_BEHAVIOR, 4);
        e.mem.map(CTRLC_HANDLER, 0x10);
        e.mem.map(FIRST_FPE_INDEX, 8);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn raise_runs_a_global_handler_once() {
        let mut e = signal_engine();
        let handler = 0x0040_2000u32;
        e.register_double(handler, |_, _| Ret::default());
        e.mem.set_u32(TERM_HANDLER, handler ^ 0x5a5a_5a5a);
        assert_eq!(e.call(0x00ed_dfce, &args![0xfu32]).u32(), 0);
        assert_eq!(calls(&e, handler), vec![vec![0xf]]);
        // The handler's word went back to the encoded null (SIG_DFL), under lock 0.
        assert_eq!(e.mem.u32(TERM_HANDLER), 0x5a5a_5a5a);
        assert_eq!(calls(&e, 0x00ed_df39), vec![vec![0]]);
        assert_eq!(calls(&e, 0x00ed_de5f), vec![vec![0]]);
        // SIG_IGN does nothing; SIG_DFL ends the process (_exit does not come back).
        e.mem.set_u32(CTRLC_HANDLER, 1 ^ 0x5a5a_5a5a);
        assert_eq!(raise(&mut e, 2), 0);
        e.register_double(EXIT_3, |_, _| panic!("_exit"));
        let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| raise(&mut e, 0xf)));
        assert!(stopped.is_err());
        // An unknown signal.
        assert_eq!(raise(&mut e, 7), 0xffff_ffff);
        assert_eq!(calls(&e, 0x00ec_7c30).len(), 1);
    }

    #[test]
    fn raise_fpe_uses_the_thread_table_and_restores_its_words() {
        let mut e = signal_engine();
        let ptd = e.mem.alloc(0x80);
        let table = e.mem.alloc(0x60);
        e.mem.set_u32(ptd + 0x5c, table);
        e.mem.set_u32(ptd + 0x60, 0x1111);
        e.mem.set_u32(ptd + 0x64, 0x2222);
        e.register_double(GETPTD_NOEXIT, move |_, _| ptd.into_ret());
        e.register_double(SIGLOOKUP, move |_, _| (table + 0xc).into_ret());
        let handler = 0x0040_3000u32;
        let seen = std::rc::Rc::new(std::cell::RefCell::new(vec![]));
        let s = seen.clone();
        e.register_double(handler, move |e, a| {
            s.borrow_mut()
                .push((a.to_vec(), e.mem.u32(ptd + 0x60), e.mem.u32(table + 0x14)));
            Ret::default()
        });
        e.mem.set_u32(table + 0x14, handler);
        e.mem.set_u32(table + 0x20, 0x77);
        e.mem.set_u32(FIRST_FPE_INDEX, 1);
        e.mem.set_u32(FPE_COUNT, 2);
        assert_eq!(raise(&mut e, 8), 0);
        // The handler saw the code 0x8c, cleared pointers and its own entry reset.
        assert_eq!(seen.borrow()[0], (vec![8, 0x8c], 0, 0));
        assert_eq!(e.mem.u32(table + 0x20), 0);
        assert_eq!(
            (e.mem.u32(ptd + 0x60), e.mem.u32(ptd + 0x64)),
            (0x1111, 0x2222)
        );
        // No lock for the thread's signals; no thread data: -1.
        assert!(calls(&e, 0x00ed_df39).is_empty());
        e.register_double(GETPTD_NOEXIT, |_, _| 0u32.into_ret());
        assert_eq!(raise(&mut e, 0xb), 0xffff_ffff);
    }

    #[test]
    fn abort_reports_then_exits() {
        let mut e = signal_engine();
        e.mem.set_u32(ABORT_BEHAVIOR, 3);
        e.register_double(GET_SIGABRT, |_, _| 0x0040_4000u32.into_ret());
        e.register_double(RAISE, |_, _| 0u32.into_ret());
        e.register_double(SET_UNHANDLED_EXCEPTION_FILTER, |_, _| Ret::default());
        let record = std::rc::Rc::new(std::cell::Cell::new((0u32, 0u32, 0u32)));
        let r = record.clone();
        e.register_double(UNHANDLED_EXCEPTION_FILTER, move |e, a| {
            let (rec, ctx) = (e.mem.u32(a[0]), e.mem.u32(a[0] + 4));
            r.set((e.mem.u32(rec), e.mem.u32(ctx), e.mem.u32(ctx + 0xb0)));
            Ret::default()
        });
        let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x00ec_f335, &args![]);
        }));
        assert!(stopped.is_err());
        assert_eq!(calls(&e, NMSG_WRITE), vec![vec![10]]);
        assert_eq!(calls(&e, RAISE), vec![vec![0x16]]);
        assert_eq!(calls(&e, SET_UNHANDLED_EXCEPTION_FILTER), vec![vec![0]]);
        assert_eq!(record.get(), (0x4000_0015, 0x0001_0001, 0x0040_4000));
        assert_eq!(calls(&e, EXIT_3), vec![vec![3]]);
    }

    #[test]
    fn raise_exc_ex_fills_the_record_and_takes_the_handler_s_answer() {
        let mut e = Engine::new();
        let record = e.mem.alloc(0x70);
        let control = e.mem.alloc(4);
        e.mem.set_u32(control, 0x027f);
        let operand = e.mem.alloc(8);
        e.mem.set_u64(operand, 2.0f64.to_bits());
        let result = e.mem.alloc(8);
        e.mem.set_u64(result, f64::NAN.to_bits());
        let seen = std::rc::Rc::new(std::cell::Cell::new((0u32, 0u32, 0u32, 0u32, 0u32)));
        let s = seen.clone();
        e.register_double(RAISE_EXCEPTION, move |e, a| {
            let rec = e.mem.u32(a[3]);
            s.set((
                a[0],
                e.mem.u32(rec),
                e.mem.u32(rec + 4),
                e.mem.u32(rec + 8),
                e.mem.u32(rec + 0x20),
            ));
            // The handler unmasks invalid and answers 7.0.
            let w = e.mem.u32(rec + 8);
            e.mem.set_u32(rec + 8, w | 0x10);
            e.mem.set_u64(rec + 0x50, 7.0f64.to_bits());
            Ret::default()
        });
        e.call(
            0x00ed_5255,
            &args![record, control, 8u32, 0x1au32, operand, result, 0u32],
        );
        let (code, first, cause, enable, format) = seen.get();
        assert_eq!(code, 0xc000_0090);
        // Precision 53-bit (4), rounding nearest (0), the operation at bit 5.
        assert_eq!(first, (0x1a << 5) | 4);
        assert_eq!(cause, 0x10);
        // Every exception masked: no enable bit.
        assert_eq!(enable, 0);
        assert_eq!(format, 3);
        assert_eq!(f64::from_bits(e.mem.u64(record + 0x10)), 2.0);
        assert_eq!(e.mem.u32(control), 0x027e);
        assert_eq!(f64::from_bits(e.mem.u64(result)), 7.0);
        // Floats, and several flags: the last one's code wins.
        let operand32 = e.mem.alloc(4);
        e.mem.set_u32(operand32, 1.5f32.to_bits());
        let result32 = e.mem.alloc(4);
        raise_exc_ex(&mut e, record, control, 0x13, 0, operand32, result32, 1);
        let (code, _, cause, _, format) = seen.get();
        assert_eq!((code, cause, format & 0x1f), (0xc000_0091, 7, 1));
        assert_eq!(e.mem.u32(record + 0x10), 1.5f32.to_bits());
        assert_eq!(e.mem.u32(result32), 7.0f64.to_bits() as u32);
    }

    /// An engine whose `GetFullPathNameA` answers `C:/Games/` plus the relative text (the
    /// length with the zero when it does not fit), with `errno`, `__doserrno`, the report,
    /// `_calloc`, `free` and `_dosmaperr` doubled.
    fn path_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let slot = e.mem.alloc(8);
        e.register_double(ERRNO, move |_, _| slot.into_ret());
        e.register_double(DOSERRNO, move |_, _| (slot + 4).into_ret());
        for addr in [INVALID_PARAMETER, FREE_FN, DOSMAPERR] {
            e.register_double(addr, |_, _| Ret::default());
        }
        e.register_double(CALLOC, |e, a| e.mem.alloc(a[0] * a[1]).into_ret());
        e.register_double(GET_LAST_ERROR_SLOT, |_, _| 3u32.into_ret());
        e.register_double(GET_FULL_PATH_NAME_A, |e, a| {
            let mut full = b"C:/Games/".to_vec();
            full.extend(e.mem.cstr(a[0]));
            let n = full.len() as u32;
            if a[2] != 0 && n < a[1] {
                e.mem.set_cstr(a[2], &full);
                n.into_ret()
            } else {
                (n + 1).into_ret()
            }
        });
        e.call_log = Some(vec![]);
        (e, slot)
    }

    #[test]
    fn fullpath_writes_into_the_target_or_a_new_block() {
        let (mut e, slot) = path_engine();
        let path = e.mem.alloc(8);
        e.mem.set_cstr(path, b"a.esp");
        let target = e.mem.alloc(32);
        assert_eq!(
            e.call(0x00ed_707b, &args![target, path, 32u32]).u32(),
            target
        );
        assert_eq!(e.mem.cstr(target), b"C:/Games/a.esp".to_vec());
        let block = fullpath(&mut e, 0, path, 0);
        assert_eq!(e.mem.cstr(block), b"C:/Games/a.esp".to_vec());
        assert_eq!(calls(&e, CALLOC), vec![vec![15, 1]]);
        // Too small: ERANGE; no size: EINVAL.
        assert_eq!(fullpath(&mut e, target, path, 4), 0);
        assert_eq!(e.mem.u32(slot), 0x22);
        assert_eq!(fullpath(&mut e, target, path, 0), 0);
        assert_eq!(e.mem.u32(slot), 0x16);
        // The helper asks again for a block on ERANGE.
        let out = e.mem.alloc(4);
        e.mem.set_u32(slot, 5);
        let again = fullpath_helper(&mut e, target, path, 4, out);
        assert_eq!(e.mem.u32(out), again);
        assert_eq!(e.mem.cstr(again), b"C:/Games/a.esp".to_vec());
        assert_eq!(e.mem.u32(slot), 5);
        // An empty path is the current directory.
        e.register_double(GETCWD, |_, a| (a[0] + 1).into_ret());
        let empty = e.mem.alloc(1);
        assert_eq!(fullpath(&mut e, target, empty, 8), target + 1);
    }

    #[test]
    fn getdcwd_nolock_asks_for_the_drive_s_directory() {
        let (mut e, slot) = path_engine();
        e.register_double(VALIDDRIVE, |_, a| u32::from(a[0] == 3).into_ret());
        e.register_double(GETDRIVE, |_, _| 3u32.into_ret());
        let target = e.mem.alloc(32);
        assert_eq!(
            e.call(0x00ec_caa5, &args![3u32, target, 32u32]).u32(),
            target
        );
        assert_eq!(e.mem.cstr(target), b"C:/Games/C:.".to_vec());
        let block = getdcwd_nolock(&mut e, 0, 0, 0);
        assert_eq!(e.mem.cstr(block), b"C:/Games/C:.".to_vec());
        assert_eq!(getdcwd_nolock(&mut e, 3, target, 4), 0);
        assert_eq!((e.mem.u32(slot), e.mem.u8(target)), (0x22, 0));
        assert_eq!(getdcwd_nolock(&mut e, 4, target, 32), 0);
        assert_eq!((e.mem.u32(slot), e.mem.u32(slot + 4)), (0xd, 0xf));
        assert_eq!(getdcwd_nolock(&mut e, 3, target, 0), 0);
        assert_eq!(e.mem.u32(slot), 0x16);
    }

    #[test]
    fn strupr_s_l_stat_maps_in_place_or_through_lcmapstring() {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| slot.into_ret());
        e.register_double(INVALID_PARAMETER, |_, _| Ret::default());
        e.register_double(FREEA, |_, _| Ret::default());
        let info = e.mem.alloc(0xd8);
        let update = e.mem.alloc(0x10);
        e.mem.set_u32(update, info);
        let text = e.mem.alloc(16);
        e.mem.set_cstr(text, b"vault\xe9");
        assert_eq!(e.call(0x00ec_baa8, &args![text, 16u32, update]).u32(), 0);
        assert_eq!(e.mem.cstr(text), b"VAULT\xe9".to_vec());
        assert_eq!(strupr_s_l_stat(&mut e, text, 6, update), 0x16);
        assert_eq!(e.mem.u8(text), 0);
        assert_eq!(strupr_s_l_stat(&mut e, 0, 6, update), 0x16);
        // With a LC_CTYPE handle: through ___crtLCMapStringA.
        e.mem.set_u32(info + 0x14, 0x407);
        e.mem.set_u32(info + 4, 1252);
        e.register_double(CRT_LC_MAP_STRING_A, |e, a| {
            let source = e.mem.cstr(a[3]);
            if a[5] != 0 {
                let mut upper = source.to_ascii_uppercase();
                upper.push(0);
                e.mem.write(a[5], &upper);
            }
            (source.len() as u32 + 1).into_ret()
        });
        e.mem.set_cstr(text, b"ncr");
        assert_eq!(strupr_s_l_stat(&mut e, text, 16, update), 0);
        assert_eq!(e.mem.cstr(text), b"NCR".to_vec());
        assert_eq!(strupr_s_l_stat(&mut e, text, 4, update), 0);
        e.mem.set_cstr(text, b"ncr");
        e.register_double(CRT_LC_MAP_STRING_A, |_, _| 0u32.into_ret());
        assert_eq!(strupr_s_l_stat(&mut e, text, 16, update), 0x2a);
    }
}

//! The C/C++ runtime library (MSVC LIBCMT, statically linked into
//! FalloutNV.exe 1.4.0.525), replaced by Rust at the game's own addresses so
//! translations call it like any other function. Lead-owned.
//!
//! These are `library` code in the ledger; each is marked
//! `Platform: replaces <addr>` so the ledger counts it as stood in for.
//! Semantics are the C standard's (C locale), which is what the game relies
//! on; where MSVC's behaviour is more specific it is noted.
//!
//! Floating point: x87 transcendentals (`FSIN`, `FCOS`) and Rust's `sin`,
//! `cos` can differ in the last bit; values that must be bit-exact need an
//! `nv-call` check.

#[allow(unused_imports)]
use crate::prelude::*;

pub mod cvt;
pub mod cxxframe;
pub mod fdiv;
pub mod files;
pub mod fpinit;
pub mod glue;
pub mod io2;
pub mod lcmap;
pub mod locale2;
pub mod locale3;
pub mod lowio;
pub mod math;
pub mod mbs;
pub mod mbstowcs;
pub mod mem;
pub mod misc2;
pub mod misc3;
pub mod misc4;
pub mod misc5;
pub mod misc6;
pub mod misc7;
pub mod misc8;
pub mod misc9;
pub mod num2;
pub mod printf;
pub mod qualified;
pub mod rtti;
pub mod runtime;
pub mod sbh;
pub mod scanf;
pub mod search;
pub mod seh;
pub mod sort;
#[cfg(any(test, feature = "test-support"))]
pub mod start_up_test;
pub mod startup;
pub mod stat;
pub mod stdio;
pub mod str2;
pub mod string;
pub mod time;
pub mod trig;
pub mod wide;

// ---- Memory ---------------------------------------------------------------

// Platform: replaces 00ec3bf0 (_byteswap_uint64, LIBCMT)
/// Reverses the byte order of a 64-bit value (cdecl, two words, low word
/// first; the result in `EDX:EAX`).
pub fn byteswap_uint64(_e: &mut Engine, value: u64) -> u64 {
    value.swap_bytes()
}

// Translated from 00401000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `operator new`: allocates through the `MemoryManager` singleton.
pub fn operator_new(e: &mut Engine, size: u32) -> Ptr {
    let mm: Ptr = crate::units::platform::get_memory_manager(e);
    e.call_as(0x00aa_3e40, &args![mm, size])
}

// Translated from 00401460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The game's `memcpy` wrapper: forwards to the runtime's `_memcpy`.
pub fn memcpy_wrapper(e: &mut Engine, dst: Ptr, src: Ptr, n: u32) -> Ptr {
    e.call_as(0x00ec_44d0, &args![dst, src, n])
}

// Translated from 00403d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The game's `memset` wrapper: forwards to the runtime's `_memset`.
pub fn memset_wrapper(e: &mut Engine, dst: Ptr, c: i32, n: u32) -> Ptr {
    e.call_as(0x00ec_61c0, &args![dst, c, n])
}

/// A block from the platform allocator for the CRT's heap, its asked-for size kept for `_msize`
/// (`HeapSize` answers the size `HeapAlloc` was asked for; 0 is asked as 1, as the exe's
/// `malloc` body does on the system heap).
pub(crate) fn crt_alloc(e: &mut Engine, n: u32) -> u32 {
    let block = e.mem.alloc(n);
    e.kernel.crt_block_sizes.insert(block, n.max(1));
    block
}

// Platform: replaces 00ed0cdf (malloc, LIBCMT)
pub fn malloc(e: &mut Engine, n: u32) -> Ptr {
    Ptr::new(crt_alloc(e, n))
}

// Platform: replaces 00ecd291 (free, LIBCMT)
pub fn free(e: &mut Engine, p: Ptr) {
    if let (Some(hook), true) = (e.free_hook, p.addr() != 0) {
        hook(e, p.addr());
    }
    e.kernel.crt_block_sizes.remove(&p.addr());
    e.mem.free(p.addr())
}

// Platform: replaces 00ed0d24 (__calloc_crt, LIBCMT)
pub fn calloc_crt(e: &mut Engine, count: u32, size: u32) -> Ptr {
    Ptr::new(crt_alloc(e, count.saturating_mul(size)))
}

/// `_callnewh` (`00ede20b`, the call at 00ecd137 in the exe's `realloc`).
const CALLNEWH: u32 = CALL_NEW_HANDLER;

// Platform: replaces 00eccf5d (realloc, LIBCMT)
/// `realloc(block, size)` on the platform allocator, with the exe's observable edges (read from
/// 00eccf5d): a null block is `malloc(size)`; size 0 frees the block and answers null; a size
/// above 0xFFFFFFE0 calls the new handler (`__callnewh(size)`), sets `errno` `ENOMEM` (0xc) and
/// answers null, the block kept. Otherwise a block of the new size holds the old one's bytes up
/// to the smaller size and replaces it (the system heap's `HeapReAlloc`, the path the exe takes
/// with `__active_heap` 1; the allocator never fails, so the exe's retry through `_newmode` and
/// `__callnewh` and its `errno` from `GetLastError` are not reached). A block the CRT's heap did
/// not hand out is not provided.
pub fn realloc(e: &mut Engine, block: Ptr, size: u32) -> Ptr {
    if block.is_null() {
        return malloc(e, size);
    }
    if size == 0 {
        free(e, block);
        return Ptr::new(0);
    }
    if size > 0xffff_ffe0 {
        e.call(CALLNEWH, &[size]);
        let slot = errno(e);
        e.mem.set_u32(slot.addr(), 0xc);
        return Ptr::new(0);
    }
    let Some(old) = e.kernel.crt_block_sizes.get(&block.addr()).copied() else {
        panic!(
            "realloc of {:08x}, which the CRT's heap did not hand out: not provided",
            block.addr()
        );
    };
    let new = crt_alloc(e, size);
    let bytes = e.mem.bytes(block.addr(), old.min(size));
    e.mem.write(new, &bytes);
    free(e, block);
    Ptr::new(new)
}

// Platform: replaces 00ecd31f (_msize, LIBCMT)
/// `_msize(block)` on the platform allocator, with the exe's edges (read from 00ecd31f): a null
/// block sets `errno` `EINVAL` (0x16), calls the invalid-parameter handler (five zeros) and
/// answers -1; otherwise the block's size as `HeapSize` gives it on the system heap (the path
/// the exe takes with `__active_heap` 1): the size it was asked for. A block the CRT's heap did
/// not hand out is not provided.
pub fn msize(e: &mut Engine, block: Ptr) -> u32 {
    if block.is_null() {
        let slot = errno(e);
        e.mem.set_u32(slot.addr(), 0x16);
        e.call(0x00ec_7c30, &[0, 0, 0, 0, 0]);
        return 0xffff_ffff;
    }
    match e.kernel.crt_block_sizes.get(&block.addr()) {
        Some(size) => *size,
        None => panic!(
            "_msize of {:08x}, which the CRT's heap did not hand out: not provided",
            block.addr()
        ),
    }
}

// ---- Strings --------------------------------------------------------------

/// `__gcvt_s` (`00ecdbf6`, cdecl: buffer, size, the double's two words, digits).
const GCVT_S: u32 = 0x00ec_dbf6;

// Translated from 00ecdd16 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_gcvt` (LIBCMT/gcvt.c, Ghidra library match), cdecl: (value, digits, buffer):
/// `__gcvt_s(buffer, -1 (no size limit), value, digits)`; the buffer when it answers 0, else
/// null. The value goes through `FLD`/`FSTP double` (a signalling NaN comes out quiet).
pub fn gcvt(e: &mut Engine, value: f64, digits: i32, buffer: Ptr) -> Ptr {
    let value = crate::prelude::x87_load64(value.to_bits());
    let failed = e
        .call(GCVT_S, &args![buffer, 0xffff_ffffu32, value, digits])
        .u32();
    if failed == 0 {
        buffer
    } else {
        Ptr::new(0)
    }
}

// Translated from 00ec5fce (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec5fce - the SEH frame (__SEH_prolog4/__SEH_epilog4) and its __finally block 00ec6019 (which calls __ArrayUnwind 00ec5f70 only when a destructor raised an exception, which the engine does not model) are not translated.
/// `eh vector destructor iterator` (MSVC runtime, Ghidra library match), stdcall: (array, the
/// element size, the count, the destructor): runs the destructor (`thiscall`, no stack words)
/// on each element, last first.
pub fn eh_vector_destructor_iterator(
    e: &mut Engine,
    array: Ptr,
    size: u32,
    count: i32,
    destructor: u32,
) {
    let mut at = array.addr().wrapping_add(size.wrapping_mul(count as u32));
    let mut left = count;
    loop {
        left = left.wrapping_sub(1);
        if left < 0 {
            break;
        }
        at = at.wrapping_sub(size);
        e.call(destructor, &args![at]);
    }
}

/// `_TRUNCATE` for the `count` of `strncpy_s`.
pub const TRUNCATE: u32 = 0xffff_ffff;

/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`, thiscall, one word: the
/// locale or null): fills the 0x10-byte object (locale info, multibyte
/// info, thread data, "updated" byte at +0x0C).
const LOCALE_UPDATE: u32 = 0x00ec_660e;
/// `__isctype_l` (`00ed41e7`, cdecl: character, mask, locale object).
const ISCTYPE_L: u32 = 0x00ed_41e7;
/// `__locale_changed`: 0 while the C locale is the global one.
const LOCALE_CHANGED: u32 = 0x0126_fdf4;
/// The initial locale object `strtol` passes while the locale is unchanged.
const INITIAL_LOCALE: u32 = 0x011c_2400;

/// The inlined `_LocaleUpdate` destructor: when the constructor set the
/// "updated" byte (+0x0C), clears bit 1 of the thread data's `_ownlocale`
/// (+0x70).
fn locale_update_release(e: &mut Engine, update: u32) {
    if e.mem.u8(update + 0xc) != 0 {
        let thread = e.mem.u32(update + 8);
        let own = e.mem.u32(thread + 0x70);
        e.mem.set_u32(thread + 0x70, own & 0xffff_fffd);
    }
}

// Translated from 00ec8ad9 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strtoxl` (LIBCMT/strtol.c, engine map; cdecl: locale, text, end, base,
/// flags; flag 1 is unsigned, 2 negative, 4 overflow, 8 a digit read).
/// Takes the locale (`_LocaleUpdate`), stores `text` in `*end`; a null text
/// or a base outside 0 and 2..36 sets `errno` to `EINVAL` (0x16), reports
/// an invalid parameter and returns 0. Otherwise skips characters of type
/// 8 (space, through `__isctype_l` when the locale's `mb_cur_max` (+0xAC)
/// is above 1, else the ctype table at +0xC8), takes a sign, picks base 8,
/// 10 or 16 for base 0 and skips a `0x`/`0X` prefix for base 16, then
/// accumulates digits (type 4) and letters (types 0x103) below the base.
/// With no digit `*end` is `text` and the result 0; on overflow (unsigned,
/// or past `0x7FFFFFFF`/`0x80000000` signed) `errno` is `ERANGE` (0x22)
/// and the result `0xFFFFFFFF`, `0x7FFFFFFF` or `0x80000000`. `*end` is the
/// first unread character; a negative result is negated.
pub fn strtoxl(e: &mut Engine, locale: u32, text: u32, end: u32, base: i32, flags: u32) -> u32 {
    // The `_LocaleUpdate` object at EBP-0x14 and the value at EBP-0x4.
    e.with_stack(0x14, |e, frame| {
        let update = frame.addr();
        e.call(LOCALE_UPDATE, &[update, locale]);
        if end != 0 {
            e.mem.set_u32(end, text);
        }
        let invalid = text == 0 || (base != 0 && !(2..=0x24).contains(&base));
        if invalid {
            let slot = errno(e);
            e.mem.set_u32(slot.addr(), 0x16);
            fn_00ec7c30(e, 0, 0, 0, 0, 0);
            locale_update_release(e, update);
            return 0;
        }
        let mut flags = flags;
        let mut c = e.mem.u8(text);
        let mut value: u32 = 0;
        let mut p = text + 1;
        loop {
            let info = e.mem.u32(update);
            let space = if (e.mem.u32(info + 0xac) as i32) > 1 {
                e.call(ISCTYPE_L, &[u32::from(c), 8, update]).u32()
            } else {
                let table = e.mem.u32(info + 0xc8);
                u32::from(e.mem.u16(table + u32::from(c) * 2)) & 8
            };
            if space == 0 {
                break;
            }
            c = e.mem.u8(p);
            p += 1;
        }
        if c == b'-' {
            flags |= 2;
            c = e.mem.u8(p);
            p += 1;
        } else if c == b'+' {
            c = e.mem.u8(p);
            p += 1;
        }
        // Checked again after the sign (never true after the check above).
        if base < 0 || base == 1 || base > 0x24 {
            if end != 0 {
                e.mem.set_u32(end, text);
            }
            locale_update_release(e, update);
            return 0;
        }
        let mut base = base;
        let mut check_prefix = false;
        if base == 0 {
            if c != b'0' {
                base = 10;
            } else {
                let next = e.mem.u8(p);
                if next == b'x' || next == b'X' {
                    base = 16;
                    check_prefix = true;
                } else {
                    base = 8;
                }
            }
        } else if base == 16 && c == b'0' {
            check_prefix = true;
        }
        if check_prefix {
            let next = e.mem.u8(p);
            if next == b'x' || next == b'X' {
                p += 1;
                c = e.mem.u8(p);
                p += 1;
            }
        }
        let base = base as u32;
        let info = e.mem.u32(update);
        let table = e.mem.u32(info + 0xc8);
        let limit = u32::MAX / base;
        let last_digit = u32::MAX % base;
        loop {
            let kind = u32::from(e.mem.u16(table + u32::from(c) * 2));
            let digit = if kind & 4 != 0 {
                (i32::from(c as i8) - 0x30) as u32
            } else if kind & 0x103 != 0 {
                let mut letter = i32::from(c as i8);
                if c.wrapping_sub(0x61) <= 0x19 {
                    letter -= 0x20;
                }
                (letter - 0x37) as u32
            } else {
                break;
            };
            if digit >= base {
                break;
            }
            flags |= 8;
            if value < limit || (value == limit && digit <= last_digit) {
                value = value.wrapping_mul(base).wrapping_add(digit);
            } else {
                flags |= 4;
                if end == 0 {
                    break;
                }
            }
            c = e.mem.u8(p);
            p += 1;
        }
        p -= 1;
        if flags & 8 == 0 {
            if end != 0 {
                p = text;
            }
            value = 0;
        } else {
            let overflow = if flags & 4 != 0 {
                true
            } else if flags & 1 != 0 {
                false
            } else if flags & 2 != 0 {
                value > 0x8000_0000
            } else {
                value > 0x7fff_ffff
            };
            if overflow {
                let slot = errno(e);
                e.mem.set_u32(slot.addr(), 0x22);
                value = if flags & 1 != 0 {
                    u32::MAX
                } else {
                    0x7fff_ffff + u32::from(flags & 2 != 0)
                };
            }
        }
        if end != 0 {
            e.mem.set_u32(end, p);
        }
        if flags & 2 != 0 {
            value = value.wrapping_neg();
        }
        locale_update_release(e, update);
        value
    })
}

// Translated from 00ec8d08 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strtol` (LIBCMT/strtol.c; cdecl): `strtoxl(locale, text, end, base, 0)`
/// with the initial locale object `011c2400` while `__locale_changed`
/// (`0126fdf4`) is 0, else a null locale (the thread's).
pub fn strtol(e: &mut Engine, text: u32, end: u32, base: i32) -> i32 {
    let locale = if e.mem.u32(LOCALE_CHANGED) == 0 {
        INITIAL_LOCALE
    } else {
        0
    };
    strtoxl(e, locale, text, end, base, 0) as i32
}

// ---- Per-thread data and locale (LIBCMT tidtable.c, setlocal.c, mbctype.c) ----

/// `GetLastError` (import slot).
const IMPORT_GET_LAST_ERROR: u32 = 0x00fd_f094;
/// `SetLastError` (import slot).
const IMPORT_SET_LAST_ERROR: u32 = 0x00fd_f1dc;
/// `GetCurrentThreadId` (import slot).
const IMPORT_GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
/// `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `___set_flsgetvalue` (`00ed100d`, cdecl, no arguments): the
/// `FlsGetValue` function to call.
const SET_FLSGETVALUE: u32 = 0x00ed_100d;
/// `__flsindex`: the FLS slot of the thread data block (-1 until the CRT's
/// start-up allocates it).
const FLS_INDEX: u32 = 0x011c_1cbc;
/// The encoded `FlsSetValue` pointer.
const ENCODED_FLS_SET_VALUE: u32 = 0x0126_fe08;
/// `_initptd` (`00ed107e`, cdecl: block, locale info).
const INIT_PTD: u32 = 0x00ed_107e;
/// `_amsg_exit` (`00eccc55`, cdecl: run-time error number).
const AMSG_EXIT: u32 = 0x00ec_cc55;
/// `_lock` (`00eddf39`, cdecl: lock number).
const CRT_LOCK: u32 = 0x00ed_df39;
/// `_unlock` (`00edde5f`, cdecl: lock number).
const CRT_UNLOCK: u32 = 0x00ed_de5f;
/// `__updatetlocinfoEx_nolock` (`00ecf704`): register arguments, EAX the
/// slot to update and EDI the new locale info, passed as those two words.
const UPDATE_TLOCINFO_EX_NOLOCK: u32 = 0x00ec_f704;
/// `__globallocalestatus`: the `_ownlocale` bits that keep a thread's own
/// locale.
const GLOBAL_LOCALE_STATUS: u32 = 0x011c_2310;
/// `__ptlocinfo`: the global locale info.
const GLOBAL_LOCALE_INFO: u32 = 0x011c_23f8;
/// `__ptmbcinfo`: the global multibyte info.
const GLOBAL_MBC_INFO: u32 = 0x011c_2218;
/// The CRT's initial (static) multibyte info, never freed.
const INITIAL_MBC_INFO: u32 = 0x011c_1df0;
/// `__crtGetStringTypeA_stat` (`00ee11e9`): the locale object in ECX and
/// seven stack words, passed as eight words (the object first).
const GET_STRING_TYPE_A_STAT: u32 = 0x00ee_11e9;

// Translated from 00ed1165 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__getptd_noexit` (LIBCMT/tidtable.c, Ghidra's function ID; cdecl): the
/// calling thread's CRT data block. Keeps the last error, reads the block
/// from the FLS slot (`FlsGetValue` as `___set_flsgetvalue` returns it); a
/// thread without one gets a zeroed 0x214-byte block stored in the slot
/// (the decoded `FlsSetValue`) and set up by `_initptd(block, 0)` with its
/// thread id and handle -1; a failed store frees it and returns 0. Restores
/// the last error.
pub fn getptd_noexit(e: &mut Engine) -> u32 {
    let last_error = e.call(IMPORT_GET_LAST_ERROR, &[]).u32();
    let index = e.mem.u32(FLS_INDEX);
    let get_value = e.call(SET_FLSGETVALUE, &[]).u32();
    let mut block = e.call(get_value, &[index]).u32();
    if block == 0 {
        block = calloc_crt(e, 1, 0x214).addr();
        if block != 0 {
            let encoded = e.mem.u32(ENCODED_FLS_SET_VALUE);
            let index = e.mem.u32(FLS_INDEX);
            let set_value = e.call(DECODE_POINTER, &[encoded]).u32();
            if e.call(set_value, &[index, block]).u32() != 0 {
                e.call(INIT_PTD, &[block, 0]);
                let thread = e.call(IMPORT_GET_CURRENT_THREAD_ID, &[]).u32();
                e.mem.set_u32(block + 4, u32::MAX);
                e.mem.set_u32(block, thread);
            } else {
                free(e, Ptr::new(block));
                block = 0;
            }
        }
    }
    e.call(IMPORT_SET_LAST_ERROR, &[last_error]);
    block
}

// Translated from 00ed11de (decompiled, FalloutNV.exe 1.4.0.525)
/// `_getptd` (LIBCMT/tidtable.c; cdecl): [`getptd_noexit`], ending the
/// program through `_amsg_exit(0x10)` (`_RT_THREAD`) when there is no
/// block.
pub fn getptd(e: &mut Engine) -> u32 {
    let block = getptd_noexit(e);
    if block == 0 {
        e.call(AMSG_EXIT, &[0x10]);
    }
    block
}

// Translated from 00ecf742 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__updatetlocinfo` (LIBCMT/setlocal.c, Ghidra's function ID; cdecl): the
/// thread's locale info: the thread's own (+0x6C) when its `_ownlocale`
/// (+0x70) has a bit of `__globallocalestatus` and it has one; otherwise,
/// under lock 0xC, `__updatetlocinfoEx_nolock(&ptd->ptlocinfo,
/// __ptlocinfo)` (the `__finally` block at `00ecf7ac`, which unlocks and
/// takes the result, is this function's own code). A null result ends the
/// program (`_amsg_exit(0x20)`).
// Translation incomplete: 00ecf742 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated.
pub fn update_tlocinfo(e: &mut Engine) -> u32 {
    let thread = getptd(e);
    let status = e.mem.u32(GLOBAL_LOCALE_STATUS);
    let info = if e.mem.u32(thread + 0x70) & status != 0 && e.mem.u32(thread + 0x6c) != 0 {
        let thread = getptd(e);
        e.mem.u32(thread + 0x6c)
    } else {
        e.call(CRT_LOCK, &[0xc]);
        let global = e.mem.u32(GLOBAL_LOCALE_INFO);
        let result = e
            .call(UPDATE_TLOCINFO_EX_NOLOCK, &[thread + 0x6c, global])
            .u32();
        e.call(CRT_UNLOCK, &[0xc]);
        result
    };
    if info == 0 {
        e.call(AMSG_EXIT, &[0x20]);
    }
    info
}

// Translated from 00ed3cdf (decompiled, FalloutNV.exe 1.4.0.525)
/// `__updatetmbcinfo` (LIBCMT/mbctype.c, Ghidra's function ID; cdecl): the
/// thread's multibyte info (+0x68). When the thread does not keep its own
/// locale (the `_ownlocale` test, then its locale info +0x6C), under lock
/// 0xD: a block other than `__ptmbcinfo` is released (`InterlockedDecrement`
/// on its count; freed at zero unless it is the static initial one) and
/// replaced by `__ptmbcinfo`, whose count goes up (the `__finally` block at
/// `00ed3d7a`, the unlock, is this function's own code). A null result ends
/// the program (`_amsg_exit(0x20)`).
// Translation incomplete: 00ed3cdf - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated.
pub fn update_tmbcinfo(e: &mut Engine) -> u32 {
    let thread = getptd(e);
    let status = e.mem.u32(GLOBAL_LOCALE_STATUS);
    let info = if e.mem.u32(thread + 0x70) & status != 0 && e.mem.u32(thread + 0x6c) != 0 {
        e.mem.u32(thread + 0x68)
    } else {
        e.call(CRT_LOCK, &[0xd]);
        let mut info = e.mem.u32(thread + 0x68);
        if info != e.mem.u32(GLOBAL_MBC_INFO) {
            if info != 0
                && e.call(IMPORT_INTERLOCKED_DECREMENT, &[info]).u32() == 0
                && info != INITIAL_MBC_INFO
            {
                free(e, Ptr::new(info));
            }
            let global = e.mem.u32(GLOBAL_MBC_INFO);
            e.mem.set_u32(thread + 0x68, global);
            info = e.mem.u32(GLOBAL_MBC_INFO);
            e.call(IMPORT_INTERLOCKED_INCREMENT, &[info]);
        }
        e.call(CRT_UNLOCK, &[0xd]);
        info
    };
    if info == 0 {
        e.call(AMSG_EXIT, &[0x20]);
    }
    info
}

// Translated from 00ec660e (decompiled, FalloutNV.exe 1.4.0.525)
/// `_LocaleUpdate::_LocaleUpdate` (Ghidra's function ID; thiscall, one word):
/// fills the 0x10-byte object: with a locale, its locale and multibyte info;
/// without one, the thread's (`_getptd`, +0x6C and +0x68) brought up to date
/// (`__updatetlocinfo` / `__updatetmbcinfo`) unless they are the globals or
/// the thread keeps its own locale, the thread block at +0x08 and, when bit
/// 1 of its `_ownlocale` (+0x70) was clear, that bit set and the "updated"
/// byte (+0x0C) set. Returns `this`.
pub fn locale_update(e: &mut Engine, this: u32, locale: u32) -> u32 {
    e.mem.set_u8(this + 0xc, 0);
    if locale != 0 {
        let info = e.mem.u32(locale);
        e.mem.set_u32(this, info);
        let mbc = e.mem.u32(locale + 4);
        e.mem.set_u32(this + 4, mbc);
        return this;
    }
    let thread = getptd(e);
    e.mem.set_u32(this + 8, thread);
    let info = e.mem.u32(thread + 0x6c);
    e.mem.set_u32(this, info);
    let mbc = e.mem.u32(thread + 0x68);
    e.mem.set_u32(this + 4, mbc);
    if e.mem.u32(this) != e.mem.u32(GLOBAL_LOCALE_INFO)
        && e.mem.u32(thread + 0x70) & e.mem.u32(GLOBAL_LOCALE_STATUS) == 0
    {
        let info = update_tlocinfo(e);
        e.mem.set_u32(this, info);
    }
    if e.mem.u32(this + 4) != e.mem.u32(GLOBAL_MBC_INFO) {
        let thread = e.mem.u32(this + 8);
        if e.mem.u32(thread + 0x70) & e.mem.u32(GLOBAL_LOCALE_STATUS) == 0 {
            let mbc = update_tmbcinfo(e);
            e.mem.set_u32(this + 4, mbc);
        }
    }
    let thread = e.mem.u32(this + 8);
    let own = e.mem.u32(thread + 0x70);
    if own & 2 == 0 {
        e.mem.set_u32(thread + 0x70, own | 2);
        e.mem.set_u8(this + 0xc, 1);
    }
    this
}

// Translated from 00ed419c (decompiled, FalloutNV.exe 1.4.0.525)
/// `_isleadbyte_l` (Ghidra's function ID; cdecl: character, locale): bit
/// 0x8000 (`_LEADBYTE`) of the low byte's entry in the locale's ctype table
/// (+0xC8).
pub fn isleadbyte_l(e: &mut Engine, c: u32, locale: u32) -> u32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        locale_update(e, update, locale);
        let info = e.mem.u32(update);
        let table = e.mem.u32(info + 0xc8);
        let kind = u32::from(e.mem.u16(table + (c & 0xff) * 2)) & 0x8000;
        locale_update_release(e, update);
        kind
    })
}

// Translated from 00ee13a3 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__crtGetStringTypeA` (Ghidra's function ID; cdecl, eight words): takes
/// the locale and passes the seven other words to `__crtGetStringTypeA_stat`
/// (`00ee11e9`) with the locale object, returning its result.
#[allow(clippy::too_many_arguments)]
pub fn crt_get_string_type_a(
    e: &mut Engine,
    locale: u32,
    info_type: u32,
    text: u32,
    length: u32,
    types: u32,
    code_page: u32,
    lcid: u32,
    error: u32,
) -> u32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        locale_update(e, update, locale);
        let result = e
            .call(
                GET_STRING_TYPE_A_STAT,
                &[
                    update, info_type, text, length, types, code_page, lcid, error,
                ],
            )
            .u32();
        locale_update_release(e, update);
        result
    })
}

// Translated from 00ed41e7 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_isctype_l` (Ghidra's function ID; cdecl: character, mask, locale): for
/// -1..255 the character's entry in the locale's ctype table (+0xC8);
/// otherwise the type `__crtGetStringTypeA(.., CT_CTYPE1, ..)` gives for
/// its one or two bytes (two when the high byte is a lead byte), 0 when
/// that fails. Returns the type masked by `mask`.
pub fn isctype_l(e: &mut Engine, c: i32, mask: u32, locale: u32) -> u32 {
    // The frame from EBP-0x18: the `_LocaleUpdate` object, the 3-byte
    // text at EBP-0x8 and the 16-bit type at EBP-0x4.
    e.with_stack(0x18, |e, frame| {
        let update = frame.addr();
        let text = update + 0x10;
        let kind_slot = update + 0x14;
        locale_update(e, update, locale);
        let kind = if (c as u32).wrapping_add(1) <= 0x100 {
            let info = e.mem.u32(update);
            let table = e.mem.u32(info + 0xc8);
            u32::from(e.mem.u16(table.wrapping_add((c as u32).wrapping_mul(2))))
        } else {
            let high = c >> 8;
            let length = if isleadbyte_l(e, (high as u32) & 0xff, update) != 0 {
                e.mem.set_u8(text, high as u8);
                e.mem.set_u8(text + 1, c as u8);
                e.mem.set_u8(text + 2, 0);
                2
            } else {
                e.mem.set_u8(text, c as u8);
                e.mem.set_u8(text + 1, 0);
                1
            };
            let info = e.mem.u32(update);
            let code_page = e.mem.u32(info + 4);
            let lcid = e.mem.u32(info + 0x14);
            let ok =
                crt_get_string_type_a(e, update, 1, text, length, kind_slot, code_page, lcid, 1);
            if ok == 0 {
                locale_update_release(e, update);
                return 0;
            }
            u32::from(e.mem.u16(kind_slot))
        };
        let result = kind & mask;
        locale_update_release(e, update);
        result
    })
}

// ---- Thread start-up: __mtinit and its chain (LIBCMT tidtable.c, mlock.c) ----

/// `GetModuleHandleW` (import slot).
const IMPORT_GET_MODULE_HANDLE_W: u32 = 0x00fd_f198;
/// `GetProcAddress` (import slot).
const IMPORT_GET_PROC_ADDRESS: u32 = 0x00fd_f0ac;
/// `TlsAlloc` (import slot).
const IMPORT_TLS_ALLOC: u32 = 0x00fd_f048;
/// `TlsGetValue` (import slot).
const IMPORT_TLS_GET_VALUE: u32 = 0x00fd_f044;
/// `TlsSetValue` (import slot).
const IMPORT_TLS_SET_VALUE: u32 = 0x00fd_f04c;
/// `TlsFree` (import slot).
const IMPORT_TLS_FREE: u32 = 0x00fd_f050;
/// `Sleep` (import slot).
const IMPORT_SLEEP: u32 = 0x00fd_f104;
/// `EnterCriticalSection` (import slot).
const IMPORT_ENTER_CRITICAL_SECTION: u32 = 0x00fd_f05c;
/// `LeaveCriticalSection` (import slot).
const IMPORT_LEAVE_CRITICAL_SECTION: u32 = 0x00fd_f100;
/// `InitializeCriticalSectionAndSpinCount` (import slot).
const IMPORT_INIT_CRITICAL_SECTION_SPIN: u32 = 0x00fd_f130;
/// `GetStringTypeW` (import slot).
const IMPORT_GET_STRING_TYPE_W: u32 = 0x00fd_f230;
/// `GetStringTypeA` (import slot).
const IMPORT_GET_STRING_TYPE_A: u32 = 0x00fd_f234;
/// `MultiByteToWideChar` (import slot).
const IMPORT_MULTI_BYTE_TO_WIDE_CHAR: u32 = 0x00fd_f128;
/// The wide string `KERNEL32.DLL`.
const KERNEL32_NAME: u32 = 0x010f_29e8;
/// The strings `FlsAlloc`, `FlsGetValue`, `FlsSetValue`, `FlsFree`,
/// `EncodePointer` and `DecodePointer`.
const FLS_ALLOC_NAME: u32 = 0x010f_2a34;
const FLS_GET_VALUE_NAME: u32 = 0x010f_2a28;
const FLS_SET_VALUE_NAME: u32 = 0x010f_2a1c;
const FLS_FREE_NAME: u32 = 0x010f_2a14;
const ENCODE_POINTER_NAME: u32 = 0x010f_29d8;
const DECODE_POINTER_NAME: u32 = 0x010f_2a04;
/// The four FLS function pointers `__mtinit` fills (encoded once set):
/// `FlsAlloc`, `FlsGetValue`, `FlsSetValue`, `FlsFree`.
const FLS_ALLOC_POINTER: u32 = 0x0126_fe00;
const FLS_GET_VALUE_POINTER: u32 = 0x0126_fe04;
const FLS_SET_VALUE_POINTER: u32 = 0x0126_fe08;
const FLS_FREE_POINTER: u32 = 0x0126_fe0c;
/// `__tlsindex`: the TLS slot holding the `FlsGetValue` pointer (-1 until
/// `__mtinit`).
const TLS_INDEX: u32 = 0x011c_1cc0;
/// `__crtTlsAlloc` (`00ed1004`): the `FlsAlloc` stand-in when the system
/// has no FLS (`TlsAlloc`, its callback ignored).
const CRT_TLS_ALLOC: u32 = 0x00ed_1004;
/// `_freefls` (`00ed11f8`), the FLS destructor `FlsAlloc` is given.
const FREE_FLS: u32 = 0x00ed_11f8;
/// `__mtdeletelocks` (`00edde08`), where `__mtterm` ends.
const MT_DELETE_LOCKS: u32 = 0x00ed_de08;
/// The lock table: 0x24 entries of (critical section, kind), kind 1 for
/// the locks `__mtinitlocks` sets up from the static block at `012705c0`.
const LOCK_TABLE: u32 = 0x011c_28a8;
const STATIC_LOCKS: u32 = 0x0127_05c0;
/// `_crtheap`, the CRT heap's handle (0 before the heap's start-up: too
/// early, `__mtinitlocknum` and `_malloc` end the program).
const LOCKS_READY: u32 = 0x0127_05bc;
/// `_FF_MSGBANNER` (`00ed2ab7`, no arguments) and `_NMSG_WRITE`
/// (`00ed290c`, cdecl: message number).
const FF_MSGBANNER: u32 = 0x00ed_2ab7;
const NMSG_WRITE: u32 = 0x00ed_290c;
/// The encoded exit routine `_amsg_exit` calls with 0xFF.
const ENCODED_EXIT: u32 = 0x011c_19c0;
/// `___addlocaleref`, `___removelocaleref`, `___freetlocinfo` (cdecl, one
/// word each).
const ADD_LOCALE_REF: u32 = 0x00ec_f5b6;
const REMOVE_LOCALE_REF: u32 = 0x00ec_f645;
const FREE_TLOCINFO: u32 = 0x00ec_f46d;
/// The CRT's initial (static) locale info, never freed.
const INITIAL_LOCALE_INFO: u32 = 0x011c_2320;
/// `_malloc` (`00ecd1c7`) and `__freea` (`00ec8e14`), cdecl.
const MALLOC_BODY: u32 = 0x00ec_d1c7;
const FREEA: u32 = 0x00ec_8e14;
/// `___ansicp` (`00ee13e5`, cdecl: locale id) and `___convertcp`
/// (`00ee142e`, cdecl, six words).
const ANSI_CODE_PAGE: u32 = 0x00ee_13e5;
const CONVERT_CODE_PAGE: u32 = 0x00ee_142e;
/// Which string-type API works: 0 unknown, 1 the wide one, 2 the ANSI one.
const STRING_TYPE_MODE: u32 = 0x0127_07c0;
/// The wide empty string the probe passes.
const EMPTY_WIDE_STRING: u32 = 0x0101_1d78;

// Translated from 00eccc25 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__crt_waiting_on_module_handle` (engine map; cdecl: module name): sleeps
/// 1000 ms, then 2000, ... while `GetModuleHandleW` finds nothing, giving
/// up past 60000. Returns the handle (0 when it gave up).
pub fn crt_waiting_on_module_handle(e: &mut Engine, name: u32) -> u32 {
    let mut wait = 1000u32;
    loop {
        e.call(IMPORT_SLEEP, &[wait]);
        let handle = e.call(IMPORT_GET_MODULE_HANDLE_W, &[name]).u32();
        wait += 1000;
        if wait > 60000 || handle != 0 {
            return handle;
        }
    }
}

/// The function `name` of `KERNEL32.DLL` (`GetModuleHandleW`, waiting for
/// it when needed, then `GetProcAddress`), 0 without the module: the
/// shared tail of `__encode_pointer` and `__decode_pointer`.
fn kernel32_function(e: &mut Engine, name: u32) -> u32 {
    let mut module = e.call(IMPORT_GET_MODULE_HANDLE_W, &[KERNEL32_NAME]).u32();
    if module == 0 {
        module = crt_waiting_on_module_handle(e, KERNEL32_NAME);
        if module == 0 {
            return 0;
        }
    }
    e.call(IMPORT_GET_PROC_ADDRESS, &[module, name]).u32()
}

/// `EncodePointer`/`DecodePointer` as the thread block caches them
/// (`ptd + offset`), else from `KERNEL32.DLL` (`name`); the value passed
/// through it, or unchanged when there is none.
fn code_pointer(e: &mut Engine, value: u32, offset: u32, name: u32) -> u32 {
    let mut function = 0;
    let mut cached = false;
    let tls_index = e.mem.u32(TLS_INDEX);
    if e.call(IMPORT_TLS_GET_VALUE, &[tls_index]).u32() != 0 {
        let fls_index = e.mem.u32(FLS_INDEX);
        if fls_index != u32::MAX {
            let tls_index = e.mem.u32(TLS_INDEX);
            let get_value = e.call(IMPORT_TLS_GET_VALUE, &[tls_index]).u32();
            let block = e.call(get_value, &[fls_index]).u32();
            if block != 0 {
                function = e.mem.u32(block + offset);
                cached = true;
            }
        }
    }
    if !cached {
        function = kernel32_function(e, name);
    }
    if function != 0 {
        e.call(function, &[value]).u32()
    } else {
        value
    }
}

// Translated from 00ed0f17 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__encode_pointer` (engine map; cdecl: pointer): through the thread
/// block's `EncodePointer` (+0x1F8) when the TLS slot and FLS slot give one,
/// else `KERNEL32.DLL`'s; unchanged without either.
pub fn encode_pointer(e: &mut Engine, value: u32) -> u32 {
    code_pointer(e, value, 0x1f8, ENCODE_POINTER_NAME)
}

// Translated from 00ed0f92 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__decode_pointer` (Ghidra's function ID; the engine map's `doexit` name
/// is folded code; cdecl: pointer): as [`encode_pointer`] with
/// `DecodePointer` (+0x1FC).
pub fn decode_pointer(e: &mut Engine, value: u32) -> u32 {
    code_pointer(e, value, 0x1fc, DECODE_POINTER_NAME)
}

// Translated from 00ed0f89 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__encoded_null` (engine map): `__encode_pointer(0)`.
pub fn encoded_null(e: &mut Engine) -> u32 {
    encode_pointer(e, 0)
}

// Translated from 00ed100d (decompiled, FalloutNV.exe 1.4.0.525)
/// `___set_flsgetvalue` (engine map; cdecl): the TLS slot's value (the
/// `FlsGetValue` function); when empty, the decoded `FlsGetValue` pointer,
/// stored in the slot.
pub fn set_flsgetvalue(e: &mut Engine) -> u32 {
    let index = e.mem.u32(TLS_INDEX);
    let mut value = e.call(IMPORT_TLS_GET_VALUE, &[index]).u32();
    if value == 0 {
        let encoded = e.mem.u32(FLS_GET_VALUE_POINTER);
        value = decode_pointer(e, encoded);
        let index = e.mem.u32(TLS_INDEX);
        e.call(IMPORT_TLS_SET_VALUE, &[index, value]);
    }
    value
}

// Translated from 00ed1004 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__crtTlsAlloc` (Ghidra made no function here; read from the bytes;
/// stdcall, one word): `TlsAlloc()`, the callback ignored.
pub fn crt_tls_alloc(e: &mut Engine, _unused_1: u32) -> u32 {
    e.call(IMPORT_TLS_ALLOC, &[]).u32()
}

// Translated from 00ede1fc (decompiled, FalloutNV.exe 1.4.0.525)
/// One of `__init_pointers`' set-ups (unnamed): stores its argument in
/// `0127072c`.
pub fn fn_00ede1fc(e: &mut Engine, value: u32) {
    e.mem.set_u32(0x0127_072c, value);
}

// Translated from 00ede18d (decompiled, FalloutNV.exe 1.4.0.525)
/// One of `__init_pointers`' set-ups (unnamed): stores its argument in
/// `01270728`.
pub fn fn_00ede18d(e: &mut Engine, value: u32) {
    e.mem.set_u32(0x0127_0728, value);
}

// Translated from 00ed2c6f (decompiled, FalloutNV.exe 1.4.0.525)
/// One of `__init_pointers`' set-ups (unnamed): stores its argument in
/// `01270464`.
pub fn fn_00ed2c6f(e: &mut Engine, value: u32) {
    e.mem.set_u32(0x0127_0464, value);
}

// Translated from 00ede17e (decompiled, FalloutNV.exe 1.4.0.525)
/// One of `__init_pointers`' set-ups (unnamed): stores its argument in
/// `01270724`.
pub fn fn_00ede17e(e: &mut Engine, value: u32) {
    e.mem.set_u32(0x0127_0724, value);
}

// Translated from 00eddf6c (decompiled, FalloutNV.exe 1.4.0.525)
/// `__initp_misc_winsig` (engine map): stores its argument in the four
/// signal-handler words `01270710`..`0127071c`.
pub fn initp_misc_winsig(e: &mut Engine, value: u32) {
    for slot in [0x0127_0710, 0x0127_0714, 0x0127_0718, 0x0127_071c] {
        e.mem.set_u32(slot, value);
    }
}

// Translated from 00ec5eec (decompiled, FalloutNV.exe 1.4.0.525)
/// One of `__init_pointers`' set-ups (unnamed): an empty function (`RET`);
/// its one word is not read.
pub fn fn_00ec5eec(_e: &mut Engine, _unused_1: u32) {}

// Translated from 00ed1538 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__initp_eh_hooks` (engine map; its word is not read): stores the
/// encoded `00ed14b4` in `0126fe10`.
pub fn initp_eh_hooks(e: &mut Engine, _unused_1: u32) {
    let encoded = encode_pointer(e, 0x00ed_14b4);
    e.mem.set_u32(0x0126_fe10, encoded);
}

/// `00ec7af9`, the third of `__init_pointers`' set-ups
/// (`units/unplaced/run_00ec7ad2.rs`).
const INITP_00EC7AF9: u32 = 0x00ec_7af9;

// Translated from 00eccf0f (decompiled, FalloutNV.exe 1.4.0.525)
/// `__init_pointers` (engine map; cdecl): hands the encoded null to the
/// CRT's eight pointer set-ups and keeps the encoded `00eccedb` at
/// `011c19c0` (the exit routine `_amsg_exit` calls).
pub fn init_pointers(e: &mut Engine) {
    let null = encoded_null(e);
    fn_00ede1fc(e, null);
    fn_00ede18d(e, null);
    e.call(INITP_00EC7AF9, &[null]);
    fn_00ed2c6f(e, null);
    fn_00ede17e(e, null);
    initp_misc_winsig(e, null);
    fn_00ec5eec(e, null);
    initp_eh_hooks(e, null);
    let exit = encode_pointer(e, 0x00ec_cedb);
    e.mem.set_u32(ENCODED_EXIT, exit);
}

// Translated from 00ede19c (decompiled, FalloutNV.exe 1.4.0.525)
/// `__crtInitCritSecAndSpinCount` (engine map; cdecl: section, spin count):
/// `InitializeCriticalSectionAndSpinCount`'s result.
// Translation incomplete: 00ede19c - the SEH frame and its __except handler (__SEH_prolog4/__SEH_epilog4) are not translated.
pub fn crt_init_crit_sec_and_spin_count(e: &mut Engine, section: u32, spin: u32) -> u32 {
    e.call(IMPORT_INIT_CRITICAL_SECTION_SPIN, &[section, spin])
        .u32()
}

// Translated from 00edddbd (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mtinitlocks` (engine map; cdecl): gives each lock of kind 1 its
/// critical section from the static block (0x18 bytes each, in order) and
/// initializes it (spin count 0xFA0); a failure clears that entry and
/// returns 0. Returns 1.
pub fn mtinitlocks(e: &mut Engine) -> u32 {
    let mut section = STATIC_LOCKS;
    for index in 0..0x24u32 {
        let entry = LOCK_TABLE + index * 8;
        if e.mem.u32(entry + 4) == 1 {
            e.mem.set_u32(entry, section);
            section += 0x18;
            let current = e.mem.u32(entry);
            if crt_init_crit_sec_and_spin_count(e, current, 0xfa0) == 0 {
                e.mem.set_u32(entry, 0);
                return 0;
            }
        }
    }
    1
}

// Translated from 00eddf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `__mtinitlocknum` (its own function; it uses
/// no frame): `_unlock(0xA)`.
pub fn fn_00eddf30(e: &mut Engine) {
    unlock(e, 0xa);
}

// Translated from 00edde76 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mtinitlocknum` (engine map; cdecl: lock number): before the CRT's
/// start-up is done (`012705bc` 0) writes the banner and message 0x1E and
/// exits with 0xFF. An entry that has its section returns 1; otherwise a
/// 0x18-byte section is allocated (`ENOMEM` and 0 when that fails) and,
/// under lock 0xA, initialized into the entry unless another thread did it
/// first (then freed); a failed initialization frees it, sets `ENOMEM` and
/// returns 0.
// Translation incomplete: 00edde76 - the SEH frame (__SEH_prolog4/__SEH_epilog4) is not translated.
pub fn mtinitlocknum(e: &mut Engine, number: u32) -> u32 {
    let mut result = 1;
    if e.mem.u32(LOCKS_READY) == 0 {
        e.call(FF_MSGBANNER, &[]);
        e.call(NMSG_WRITE, &[0x1e]);
        // `___crtExitProcess` by address: it does not return.
        e.call(0x00ec_cca9, &[0xff]);
    }
    let entry = LOCK_TABLE.wrapping_add(number.wrapping_mul(8));
    if e.mem.u32(entry) != 0 {
        return 1;
    }
    let section = malloc(e, 0x18).addr();
    if section == 0 {
        let slot = errno(e);
        e.mem.set_u32(slot.addr(), 0xc);
        return 0;
    }
    lock(e, 0xa);
    if e.mem.u32(entry) == 0 {
        if crt_init_crit_sec_and_spin_count(e, section, 0xfa0) == 0 {
            free(e, Ptr::new(section));
            let slot = errno(e);
            e.mem.set_u32(slot.addr(), 0xc);
            result = 0;
        } else {
            e.mem.set_u32(entry, section);
        }
    } else {
        free(e, Ptr::new(section));
    }
    fn_00eddf30(e);
    result
}

// Translated from 00eddf39 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_lock` (engine map; cdecl: lock number): sets up the lock's section on
/// first use (`__mtinitlocknum`; failing that, `_amsg_exit(0x11)`) and
/// enters it.
pub fn lock(e: &mut Engine, number: u32) {
    let entry = LOCK_TABLE.wrapping_add(number.wrapping_mul(8));
    if e.mem.u32(entry) == 0 && mtinitlocknum(e, number) == 0 {
        amsg_exit(e, 0x11);
    }
    let section = e.mem.u32(entry);
    e.call(IMPORT_ENTER_CRITICAL_SECTION, &[section]);
}

// Translated from 00edde5f (decompiled, FalloutNV.exe 1.4.0.525)
/// `_unlock` (cdecl: lock number): leaves the lock's section.
pub fn unlock(e: &mut Engine, number: u32) {
    let section = e.mem.u32(LOCK_TABLE.wrapping_add(number.wrapping_mul(8)));
    e.call(IMPORT_LEAVE_CRITICAL_SECTION, &[section]);
}

// Translated from 00eccc55 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_amsg_exit` (engine map; cdecl: run-time error number): writes the
/// banner and the message, then calls the decoded exit routine
/// (`011c19c0`) with 0xFF.
pub fn amsg_exit(e: &mut Engine, number: u32) {
    e.call(FF_MSGBANNER, &[]);
    e.call(NMSG_WRITE, &[number]);
    let encoded = e.mem.u32(ENCODED_EXIT);
    let exit = decode_pointer(e, encoded);
    e.call(exit, &[0xff]);
}

// Translated from 00ed1153 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first `__finally` block of `_initptd` (its own function; it uses no
/// frame): `_unlock(0xD)`.
pub fn fn_00ed1153(e: &mut Engine) {
    unlock(e, 0xd);
}

// Translated from 00ed115c (decompiled, FalloutNV.exe 1.4.0.525)
/// The second `__finally` block of `_initptd` (its own function; it uses
/// no frame): `_unlock(0xC)`.
pub fn fn_00ed115c(e: &mut Engine) {
    unlock(e, 0xc);
}

// Translated from 00ed107e (decompiled, FalloutNV.exe 1.4.0.525)
/// `_initptd` (engine map; cdecl: thread block, locale info): sets the
/// block's `_pxcptacttab` (+0x5C, `010fdca0`), `_holdrand` (+0x14, 1),
/// `EncodePointer`/`DecodePointer` (+0x1F8/+0x1FC, from `KERNEL32.DLL` when
/// it is found), `_ownlocale` (+0x70, 1), the two `'C'` bytes (+0xC8,
/// +0x14B), the multibyte info (+0x68, the static one, its count raised
/// under lock 0xD) and the locale info (+0x6C, `locale` or `__ptlocinfo`,
/// its references added under lock 0xC).
// Translation incomplete: 00ed107e - the SEH frame (__SEH_prolog4/__SEH_epilog4) is not translated.
pub fn initptd(e: &mut Engine, block: u32, locale: u32) {
    let mut module = e.call(IMPORT_GET_MODULE_HANDLE_W, &[KERNEL32_NAME]).u32();
    if module == 0 {
        module = crt_waiting_on_module_handle(e, KERNEL32_NAME);
    }
    e.mem.set_u32(block + 0x5c, 0x010f_dca0);
    e.mem.set_u32(block + 0x14, 1);
    if module != 0 {
        let encode = e
            .call(IMPORT_GET_PROC_ADDRESS, &[module, ENCODE_POINTER_NAME])
            .u32();
        e.mem.set_u32(block + 0x1f8, encode);
        let decode = e
            .call(IMPORT_GET_PROC_ADDRESS, &[module, DECODE_POINTER_NAME])
            .u32();
        e.mem.set_u32(block + 0x1fc, decode);
    }
    e.mem.set_u32(block + 0x70, 1);
    e.mem.set_u8(block + 0xc8, 0x43);
    e.mem.set_u8(block + 0x14b, 0x43);
    e.mem.set_u32(block + 0x68, INITIAL_MBC_INFO);
    lock(e, 0xd);
    let mbc = e.mem.u32(block + 0x68);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &[mbc]);
    fn_00ed1153(e);
    lock(e, 0xc);
    e.mem.set_u32(block + 0x6c, locale);
    if locale == 0 {
        let global = e.mem.u32(GLOBAL_LOCALE_INFO);
        e.mem.set_u32(block + 0x6c, global);
    }
    let info = e.mem.u32(block + 0x6c);
    e.call(ADD_LOCALE_REF, &[info]);
    fn_00ed115c(e);
}

// Translated from 00ed1041 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mtterm` (engine map; cdecl): frees the FLS slot (the decoded
/// `FlsFree`) and the TLS slot (`TlsFree`) that are set, marking them -1,
/// then jumps to `__mtdeletelocks` (`00edde08`).
pub fn mtterm(e: &mut Engine) {
    let fls_index = e.mem.u32(FLS_INDEX);
    if fls_index != u32::MAX {
        let encoded = e.mem.u32(FLS_FREE_POINTER);
        let fls_free = decode_pointer(e, encoded);
        e.call(fls_free, &[fls_index]);
        e.mem.set_u32(FLS_INDEX, u32::MAX);
    }
    let tls_index = e.mem.u32(TLS_INDEX);
    if tls_index != u32::MAX {
        e.call(IMPORT_TLS_FREE, &[tls_index]);
        e.mem.set_u32(TLS_INDEX, u32::MAX);
    }
    e.call(MT_DELETE_LOCKS, &[]);
}

// Translated from 00ed1327 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mtinit` (engine map; cdecl): the CRT's thread start-up. Finds
/// `KERNEL32.DLL`'s `FlsAlloc`/`FlsGetValue`/`FlsSetValue`/`FlsFree`; when
/// one is missing uses `__crtTlsAlloc` and the import table's `TlsGetValue`,
/// `TlsSetValue` and `TlsFree` words instead. Allocates the TLS slot and
/// stores the `FlsGetValue` pointer in it, sets up the CRT's pointers,
/// encodes the four FLS pointers, sets up the locks, allocates the FLS slot
/// (with `_freefls`) and gives the calling thread its 0x214-byte block
/// (stored in the slot, `_initptd`, thread id, handle -1). Returns 1; a
/// failure returns 0 (after `__mtterm`, except when the TLS slot could not
/// be allocated or set).
pub fn mtinit(e: &mut Engine) -> u32 {
    let mut module = e.call(IMPORT_GET_MODULE_HANDLE_W, &[KERNEL32_NAME]).u32();
    if module == 0 {
        module = crt_waiting_on_module_handle(e, KERNEL32_NAME);
    }
    if module == 0 {
        mtterm(e);
        return 0;
    }
    for (slot, name) in [
        (FLS_ALLOC_POINTER, FLS_ALLOC_NAME),
        (FLS_GET_VALUE_POINTER, FLS_GET_VALUE_NAME),
        (FLS_SET_VALUE_POINTER, FLS_SET_VALUE_NAME),
        (FLS_FREE_POINTER, FLS_FREE_NAME),
    ] {
        let function = e.call(IMPORT_GET_PROC_ADDRESS, &[module, name]).u32();
        e.mem.set_u32(slot, function);
    }
    let found = [
        FLS_ALLOC_POINTER,
        FLS_GET_VALUE_POINTER,
        FLS_SET_VALUE_POINTER,
        FLS_FREE_POINTER,
    ]
    .iter()
    .all(|slot| e.mem.u32(*slot) != 0);
    if !found {
        // The import table's words, as the code reads them.
        let get_value = e.mem.u32(IMPORT_TLS_GET_VALUE);
        e.mem.set_u32(FLS_GET_VALUE_POINTER, get_value);
        let free_slot = e.mem.u32(IMPORT_TLS_FREE);
        e.mem.set_u32(FLS_ALLOC_POINTER, CRT_TLS_ALLOC);
        let set_value = e.mem.u32(IMPORT_TLS_SET_VALUE);
        e.mem.set_u32(FLS_SET_VALUE_POINTER, set_value);
        e.mem.set_u32(FLS_FREE_POINTER, free_slot);
    }
    let tls_index = e.call(IMPORT_TLS_ALLOC, &[]).u32();
    e.mem.set_u32(TLS_INDEX, tls_index);
    if tls_index == u32::MAX {
        return 0;
    }
    let get_value = e.mem.u32(FLS_GET_VALUE_POINTER);
    if e.call(IMPORT_TLS_SET_VALUE, &[tls_index, get_value]).u32() == 0 {
        return 0;
    }
    init_pointers(e);
    for slot in [
        FLS_ALLOC_POINTER,
        FLS_GET_VALUE_POINTER,
        FLS_SET_VALUE_POINTER,
        FLS_FREE_POINTER,
    ] {
        let plain = e.mem.u32(slot);
        let encoded = encode_pointer(e, plain);
        e.mem.set_u32(slot, encoded);
    }
    if mtinitlocks(e) == 0 {
        mtterm(e);
        return 0;
    }
    let encoded = e.mem.u32(FLS_ALLOC_POINTER);
    let fls_alloc = decode_pointer(e, encoded);
    let fls_index = e.call(fls_alloc, &[FREE_FLS]).u32();
    e.mem.set_u32(FLS_INDEX, fls_index);
    if fls_index == u32::MAX {
        mtterm(e);
        return 0;
    }
    let block = calloc_crt(e, 1, 0x214).addr();
    if block == 0 {
        mtterm(e);
        return 0;
    }
    let encoded = e.mem.u32(FLS_SET_VALUE_POINTER);
    let fls_set = decode_pointer(e, encoded);
    let fls_index = e.mem.u32(FLS_INDEX);
    if e.call(fls_set, &[fls_index, block]).u32() == 0 {
        mtterm(e);
        return 0;
    }
    initptd(e, block, 0);
    let thread = e.call(IMPORT_GET_CURRENT_THREAD_ID, &[]).u32();
    e.mem.set_u32(block + 4, u32::MAX);
    e.mem.set_u32(block, thread);
    1
}

// Translated from 00ecf704 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__updatetlocinfoEx_nolock` (engine map; register arguments: `slot` in
/// EAX, `info` in EDI, passed as these two words): 0 when either is null;
/// otherwise, when the slot holds another info, stores `info`, adds its
/// references and removes the old one's, freeing the old one when its count
/// is 0 and it is not the static initial info. Returns `info`.
pub fn update_tlocinfo_ex_nolock(e: &mut Engine, slot: u32, info: u32) -> u32 {
    if info == 0 || slot == 0 {
        return 0;
    }
    let old = e.mem.u32(slot);
    if old != info {
        e.mem.set_u32(slot, info);
        e.call(ADD_LOCALE_REF, &[info]);
        if old != 0 {
            e.call(REMOVE_LOCALE_REF, &[old]);
            if e.mem.u32(old) == 0 && old != INITIAL_LOCALE_INFO {
                e.call(FREE_TLOCINFO, &[old]);
            }
        }
    }
    info
}

// Translated from 00ee11e9 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__crtGetStringTypeA_stat` (Ghidra's function ID; the locale object in
/// ECX, then seven cdecl words: info type, text, length, types, code page,
/// locale id, error flag). Probes once which API works (`GetStringTypeW` on
/// an empty wide string; `ERROR_CALL_NOT_IMPLEMENTED` 0x78 picks the ANSI
/// one). Wide: converts the text with `MultiByteToWideChar` (code page from
/// the locale when 0; `MB_ERR_INVALID_CHARS` with the error flag) into a
/// zeroed buffer (on the stack up to 0x400 bytes, marked 0xCCCC, else
/// `_malloc`'d, marked 0xDDDD) and returns `GetStringTypeW`'s result (0 on
/// a failure). ANSI: with the locale's id and code page when 0, converts the
/// text to the ANSI code page when it differs (`___convertcp`) and returns
/// `GetStringTypeA`'s result. The code-page conversion takes the length by
/// address; here it is a stack word.
// Translation incomplete: 00ee11e9 - the /GS stack cookie check (00ec408c) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn crt_get_string_type_a_stat(
    e: &mut Engine,
    locale: u32,
    info_type: u32,
    text: u32,
    length: u32,
    types: u32,
    code_page: u32,
    lcid: u32,
    error: u32,
) -> u32 {
    // EBP-0x8: the probe's out word and the wide result.
    e.with_stack(4, |e, result_slot| {
        let result_slot = result_slot.addr();
        let mut mode = e.mem.u32(STRING_TYPE_MODE);
        let mut wide = false;
        if mode == 0 {
            let probed = e
                .call(
                    IMPORT_GET_STRING_TYPE_W,
                    &[1, EMPTY_WIDE_STRING, 1, result_slot],
                )
                .u32();
            if probed != 0 {
                e.mem.set_u32(STRING_TYPE_MODE, 1);
                wide = true;
            } else if e.call(IMPORT_GET_LAST_ERROR, &[]).u32() == 0x78 {
                mode = 2;
                e.mem.set_u32(STRING_TYPE_MODE, 2);
            } else {
                mode = e.mem.u32(STRING_TYPE_MODE);
            }
        }
        if !wide {
            if mode == 2 || mode == 0 {
                return string_type_ansi(
                    e, locale, info_type, text, length, types, code_page, lcid,
                );
            }
            if mode != 1 {
                return 0;
            }
        }
        e.mem.set_u32(result_slot, 0);
        let code_page = if code_page == 0 {
            let info = e.mem.u32(locale);
            e.mem.u32(info + 4)
        } else {
            code_page
        };
        let flags = u32::from(error != 0) * 8 + 1;
        let count = e
            .call(
                IMPORT_MULTI_BYTE_TO_WIDE_CHAR,
                &[code_page, flags, text, length, 0, 0],
            )
            .u32();
        if count == 0 {
            return 0;
        }
        let size = count.wrapping_mul(2).wrapping_add(8);
        let fits = (count as i32) > 0 && count <= 0x7fff_fff0;
        if fits && size <= 0x400 {
            // `_alloca` through the probe `00ec5ec0`: a stack block here.
            e.with_stack(size, |e, block| {
                let block = block.addr();
                e.mem.set_u32(block, 0xcccc);
                string_type_wide(
                    e,
                    block + 8,
                    info_type,
                    text,
                    length,
                    types,
                    code_page,
                    count,
                    result_slot,
                )
            })
        } else {
            let mut buffer = 0;
            if fits {
                let block = e.call(MALLOC_BODY, &[size]).u32();
                if block != 0 {
                    e.mem.set_u32(block, 0xdddd);
                    buffer = block + 8;
                }
            }
            if buffer == 0 {
                return 0;
            }
            string_type_wide(
                e,
                buffer,
                info_type,
                text,
                length,
                types,
                code_page,
                count,
                result_slot,
            )
        }
    })
}

/// The wide path's tail: clears the buffer, converts into it and asks
/// `GetStringTypeW`, then `__freea`.
#[allow(clippy::too_many_arguments)]
fn string_type_wide(
    e: &mut Engine,
    buffer: u32,
    info_type: u32,
    text: u32,
    length: u32,
    types: u32,
    code_page: u32,
    count: u32,
    result_slot: u32,
) -> u32 {
    mem::memset(e, buffer, 0, count.wrapping_mul(2));
    let converted = e
        .call(
            IMPORT_MULTI_BYTE_TO_WIDE_CHAR,
            &[code_page, 1, text, length, buffer, count],
        )
        .u32();
    if converted != 0 {
        let result = e
            .call(
                IMPORT_GET_STRING_TYPE_W,
                &[info_type, buffer, converted, types],
            )
            .u32();
        e.mem.set_u32(result_slot, result);
    }
    e.call(FREEA, &[buffer]);
    e.mem.u32(result_slot)
}

/// The ANSI path of [`crt_get_string_type_a_stat`].
#[allow(clippy::too_many_arguments)]
fn string_type_ansi(
    e: &mut Engine,
    locale: u32,
    info_type: u32,
    text: u32,
    length: u32,
    types: u32,
    code_page: u32,
    lcid: u32,
) -> u32 {
    let lcid = if lcid == 0 {
        let info = e.mem.u32(locale);
        e.mem.u32(info + 0x14)
    } else {
        lcid
    };
    let code_page = if code_page == 0 {
        let info = e.mem.u32(locale);
        e.mem.u32(info + 4)
    } else {
        code_page
    };
    let ansi = e.call(ANSI_CODE_PAGE, &[lcid]).u32();
    if ansi == u32::MAX {
        return 0;
    }
    // The length argument's slot, which `___convertcp` updates.
    e.with_stack(4, |e, length_slot| {
        let length_slot = length_slot.addr();
        e.mem.set_u32(length_slot, length);
        let mut source = text;
        let mut converted = 0;
        if ansi != code_page {
            converted = e
                .call(
                    CONVERT_CODE_PAGE,
                    &[code_page, ansi, text, length_slot, 0, 0],
                )
                .u32();
            if converted == 0 {
                return 0;
            }
            source = converted;
        }
        let length = e.mem.u32(length_slot);
        let result = e
            .call(
                IMPORT_GET_STRING_TYPE_A,
                &[lcid, info_type, source, length, types],
            )
            .u32();
        if converted != 0 {
            free(e, Ptr::new(converted));
        }
        result
    })
}

// ---- Locks' end, run-time messages, locale references, heap (LIBCMT) ----

/// `DeleteCriticalSection` (import slot).
const IMPORT_DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// `GetModuleFileNameA`, `GetStdHandle`, `WriteFile` (import slots).
const IMPORT_GET_MODULE_FILE_NAME_A: u32 = 0x00fd_f150;
const IMPORT_GET_STD_HANDLE: u32 = 0x00fd_f1e0;
const IMPORT_WRITE_FILE: u32 = 0x00fd_f0dc;
/// `GetLocaleInfoA`, `GetCPInfo`, `WideCharToMultiByte`, `HeapAlloc`
/// (import slots).
const IMPORT_GET_LOCALE_INFO_A: u32 = 0x00fd_f268;
const IMPORT_GET_CP_INFO: u32 = 0x00fd_f1d8;
const IMPORT_WIDE_CHAR_TO_MULTI_BYTE: u32 = 0x00fd_f170;
const IMPORT_HEAP_ALLOC: u32 = 0x00fd_f1a0;
/// The end of the lock table (0x24 entries of 8 bytes).
const LOCK_TABLE_END: u32 = 0x011c_29c8;
/// `__set_error_mode` (`00ee1dac`, cdecl: mode; 3 asks for the current one).
const SET_ERROR_MODE: u32 = 0x00ee_1dac;
/// `__app_type` (1: a console program).
const APP_TYPE: u32 = 0x011c_19b0;
/// The run-time message table: 0x17 entries of (number, text).
const RUNTIME_MESSAGES: u32 = 0x011c_1d30;
/// The 0x314-byte message buffer and the program name inside it.
const MESSAGE_BUFFER: u32 = 0x0127_0148;
const MESSAGE_PROGRAM_NAME: u32 = 0x0127_0161;
const MESSAGE_PROGRAM_NAME_END: u32 = 0x0127_0265;
const MESSAGE_BUFFER_END: u32 = 0x0127_045c;
/// The strings the message is built from: `Runtime Error!\n\nProgram: `,
/// `<program name unknown>`, `...`, `\n\n` and the box caption
/// `Microsoft Visual C++ Runtime Library`.
const MESSAGE_HEADER: u32 = 0x010f_3070;
const MESSAGE_UNKNOWN_PROGRAM: u32 = 0x010f_3058;
const MESSAGE_ELLIPSIS: u32 = 0x0107_2308;
const MESSAGE_BREAK: u32 = 0x0109_5188;
const MESSAGE_CAPTION: u32 = 0x010f_3030;
/// `strcpy_s`, `strcat_s`, `strncpy_s`, `strlen` (this file's stand-ins,
/// called by address as the code does).
const STRCPY_S_ADDR: u32 = 0x00ec_65a6;
const STRCAT_S_ADDR: u32 = 0x00ec_6bd2;
const STRNCPY_S_ADDR: u32 = 0x00ec_8d5f;
const STRLEN_ADDR: u32 = 0x00ec_6130;
/// `__invoke_watson` (`00ec7b08`; it does not return).
const INVOKE_WATSON_ADDR: u32 = 0x00ec_7b08;
/// `__crtMessageBoxA` (`00ee1c43`, cdecl: text, caption, type).
const CRT_MESSAGE_BOX_A: u32 = 0x00ee_1c43;
/// `doexit` (`00eccd99`, cdecl: code, quick, return to caller).
const DOEXIT_ADDR: u32 = 0x00ec_cd99;
/// The C locale's static category names, never released.
const C_LOCALE_NAMES: u32 = 0x011c_2318;
/// The C locale's static `lconv` and its pieces, never freed.
const C_LCONV: u32 = 0x011c_19d4;
const C_LC_TIME: u32 = 0x011c_2a98;
/// The `lconv` and `lc_time` frees (`00ee06e6`, `00ee04d7`, `00ee02c9`;
/// cdecl, one word).
const FREE_LCONV_MONETARY: u32 = 0x00ee_06e6;
const FREE_LCONV_NUMERIC: u32 = 0x00ee_04d7;
const FREE_LC_TIME: u32 = 0x00ee_02c9;
/// `__active_heap` (1 the system heap, 3 the small-block heap) and
/// `_newmode`.
const ACTIVE_HEAP: u32 = 0x0127_0948;
const NEW_MODE: u32 = 0x0127_0730;
/// `__sbh_alloc_block`-style small-block allocation (`00ecd178`, cdecl:
/// size) and `_callnewh` (`00ede20b`, cdecl: size).
const SMALL_BLOCK_ALLOC: u32 = 0x00ec_d178;
const CALL_NEW_HANDLER: u32 = 0x00ed_e20b;

// Translated from 00edde08 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mtdeletelocks` (engine map; cdecl): deletes and frees every lock
/// section that is set and not of kind 1 (clearing its entry), then deletes
/// the kind-1 (static) sections.
pub fn mtdeletelocks(e: &mut Engine) {
    let mut entry = LOCK_TABLE;
    while entry < LOCK_TABLE_END {
        let section = e.mem.u32(entry);
        if section != 0 && e.mem.u32(entry + 4) != 1 {
            e.call(IMPORT_DELETE_CRITICAL_SECTION, &[section]);
            free(e, Ptr::new(section));
            e.mem.set_u32(entry, 0);
        }
        entry += 8;
    }
    let mut entry = LOCK_TABLE;
    while entry < LOCK_TABLE_END {
        let section = e.mem.u32(entry);
        if section != 0 && e.mem.u32(entry + 4) == 1 {
            e.call(IMPORT_DELETE_CRITICAL_SECTION, &[section]);
        }
        entry += 8;
    }
}

// Translated from 00ed2ab7 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_FF_MSGBANNER` (LIBCMT/crt0msg.c; no arguments): for a console program
/// (error mode 1, or the default mode with `__app_type` 1) writes the
/// messages 0xFC (a new line) and 0xFF (the banner).
pub fn ff_msgbanner(e: &mut Engine) {
    let console = e.call(SET_ERROR_MODE, &[3]).u32() == 1
        || (e.call(SET_ERROR_MODE, &[3]).u32() == 0 && e.mem.u32(APP_TYPE) == 1);
    if console {
        nmsg_write(e, 0xfc);
        nmsg_write(e, 0xff);
    }
}

/// `__invoke_watson(0, 0, 0, 0, 0)` after a failed string call (it does
/// not return).
fn watson_unless_zero(e: &mut Engine, result: u32) {
    if result != 0 {
        e.call(INVOKE_WATSON_ADDR, &[0, 0, 0, 0, 0]);
    }
}

// Translated from 00ed290c (decompiled, FalloutNV.exe 1.4.0.525)
/// `_NMSG_WRITE` (LIBCMT/crt0msg.c; cdecl: message number): for a number in
/// the run-time message table, a console program writes the text to the
/// standard error handle (`WriteFile`); otherwise (except for 0xFC) builds
/// `Runtime Error!\n\nProgram: <name, cut to "...">\n\n<text>` in the
/// message buffer and shows it with `__crtMessageBoxA` (type 0x12010).
pub fn nmsg_write(e: &mut Engine, number: u32) {
    let mut index = 0u32;
    while index < 0x17 {
        if number == e.mem.u32(RUNTIME_MESSAGES + index * 8) {
            break;
        }
        index += 1;
    }
    if index >= 0x17 {
        return;
    }
    let console = e.call(SET_ERROR_MODE, &[3]).u32() == 1
        || (e.call(SET_ERROR_MODE, &[3]).u32() == 0 && e.mem.u32(APP_TYPE) == 1);
    if console {
        let handle = e.call(IMPORT_GET_STD_HANDLE, &[(-12i32) as u32]).u32();
        if handle != 0 && handle != u32::MAX {
            let text = e.mem.u32(RUNTIME_MESSAGES + index * 8 + 4);
            e.with_stack(4, |e, written| {
                let length = e.call(STRLEN_ADDR, &[text]).u32();
                e.call(
                    IMPORT_WRITE_FILE,
                    &[handle, text, length, written.addr(), 0],
                );
            });
        }
        return;
    }
    if number == 0xfc {
        return;
    }
    let r = e
        .call(STRCPY_S_ADDR, &[MESSAGE_BUFFER, 0x314, MESSAGE_HEADER])
        .u32();
    watson_unless_zero(e, r);
    e.mem.set_u8(MESSAGE_PROGRAM_NAME_END, 0);
    let named = e
        .call(
            IMPORT_GET_MODULE_FILE_NAME_A,
            &[0, MESSAGE_PROGRAM_NAME, 0x104],
        )
        .u32();
    if named == 0 {
        let r = e
            .call(
                STRCPY_S_ADDR,
                &[MESSAGE_PROGRAM_NAME, 0x2fb, MESSAGE_UNKNOWN_PROGRAM],
            )
            .u32();
        watson_unless_zero(e, r);
    }
    let length = e.call(STRLEN_ADDR, &[MESSAGE_PROGRAM_NAME]).u32();
    if length.wrapping_add(1) > 0x3c {
        let length = e.call(STRLEN_ADDR, &[MESSAGE_PROGRAM_NAME]).u32();
        let start = length.wrapping_add(MESSAGE_PROGRAM_NAME - 0x3b);
        let r = e
            .call(
                STRNCPY_S_ADDR,
                &[
                    start,
                    MESSAGE_BUFFER_END.wrapping_sub(start),
                    MESSAGE_ELLIPSIS,
                    3,
                ],
            )
            .u32();
        watson_unless_zero(e, r);
    }
    let r = e
        .call(STRCAT_S_ADDR, &[MESSAGE_BUFFER, 0x314, MESSAGE_BREAK])
        .u32();
    watson_unless_zero(e, r);
    let text = e.mem.u32(RUNTIME_MESSAGES + index * 8 + 4);
    let r = e.call(STRCAT_S_ADDR, &[MESSAGE_BUFFER, 0x314, text]).u32();
    watson_unless_zero(e, r);
    e.call(
        CRT_MESSAGE_BOX_A,
        &[MESSAGE_BUFFER, MESSAGE_CAPTION, 0x12010],
    );
}

// Translated from 00eccedb (decompiled, FalloutNV.exe 1.4.0.525)
/// `__exit` (engine map; cdecl: exit code): `doexit(code, 1, 0)`, the quick
/// exit `_amsg_exit` reaches through its encoded pointer.
pub fn exit_quick(e: &mut Engine, code: u32) {
    e.call(DOEXIT_ADDR, &[code, 1, 0]);
}

/// Applies `step` (`InterlockedIncrement` or `InterlockedDecrement`) to every
/// count of the locale info `info`, in the order the CRT walks them: the
/// info, its `lconv` counts (+0xB0, +0xB8, +0xB4, +0xC0), the six
/// categories' name and wide-name counts that are not the C locale's, and
/// the `lc_time` count (+0xD4's +0xB4).
fn locale_counts(e: &mut Engine, info: u32, step: u32) {
    e.call(step, &[info]);
    for offset in [0xb0, 0xb8, 0xb4, 0xc0] {
        let count = e.mem.u32(info + offset);
        if count != 0 {
            e.call(step, &[count]);
        }
    }
    let mut category = info + 0x50;
    for _ in 0..6 {
        if e.mem.u32(category - 8) != C_LOCALE_NAMES {
            let count = e.mem.u32(category);
            if count != 0 {
                e.call(step, &[count]);
            }
        }
        if e.mem.u32(category - 4) != 0 {
            let count = e.mem.u32(category + 4);
            if count != 0 {
                e.call(step, &[count]);
            }
        }
        category += 0x10;
    }
    let time = e.mem.u32(info + 0xd4);
    e.call(step, &[time.wrapping_add(0xb4)]);
}

// Translated from 00ecf5b6 (decompiled, FalloutNV.exe 1.4.0.525)
/// `___addlocaleref` (engine map; cdecl: locale info): raises every
/// reference count of the info (`InterlockedIncrement`).
pub fn addlocaleref(e: &mut Engine, info: u32) {
    locale_counts(e, info, IMPORT_INTERLOCKED_INCREMENT);
}

// Translated from 00ecf645 (decompiled, FalloutNV.exe 1.4.0.525)
/// `___removelocaleref` (engine map; cdecl: locale info): lowers every
/// reference count of a non-null info (`InterlockedDecrement`). Returns
/// the info.
pub fn removelocaleref(e: &mut Engine, info: u32) -> u32 {
    if info != 0 {
        locale_counts(e, info, IMPORT_INTERLOCKED_DECREMENT);
    }
    info
}

// Translated from 00ecf46d (decompiled, FalloutNV.exe 1.4.0.525)
/// `___freetlocinfo` (engine map; cdecl: locale info): frees what has no
/// reference left: the `lconv` (+0xBC, unless it is the C locale's) with
/// its monetary (+0xB8) and numeric (+0xB4) parts, the ctype tables (+0xC0
/// count; the blocks before +0xC4, +0xCC, +0xD0), the `lc_time` (+0xD4,
/// unless the C locale's), the categories' names (not the C locale's), and
/// the info itself.
pub fn freetlocinfo(e: &mut Engine, info: u32) {
    let lconv = e.mem.u32(info + 0xbc);
    if lconv != 0 && lconv != C_LCONV {
        let count = e.mem.u32(info + 0xb0);
        if count != 0 && e.mem.u32(count) == 0 {
            let monetary = e.mem.u32(info + 0xb8);
            if monetary != 0 && e.mem.u32(monetary) == 0 {
                free(e, Ptr::new(monetary));
                let lconv = e.mem.u32(info + 0xbc);
                e.call(FREE_LCONV_MONETARY, &[lconv]);
            }
            let numeric = e.mem.u32(info + 0xb4);
            if numeric != 0 && e.mem.u32(numeric) == 0 {
                free(e, Ptr::new(numeric));
                let lconv = e.mem.u32(info + 0xbc);
                e.call(FREE_LCONV_NUMERIC, &[lconv]);
            }
            let count = e.mem.u32(info + 0xb0);
            free(e, Ptr::new(count));
            let lconv = e.mem.u32(info + 0xbc);
            free(e, Ptr::new(lconv));
        }
    }
    let ctype = e.mem.u32(info + 0xc0);
    if ctype != 0 && e.mem.u32(ctype) == 0 {
        let table = e.mem.u32(info + 0xc4).wrapping_sub(0xfe);
        free(e, Ptr::new(table));
        let lower = e.mem.u32(info + 0xcc).wrapping_sub(0x80);
        free(e, Ptr::new(lower));
        let upper = e.mem.u32(info + 0xd0).wrapping_sub(0x80);
        free(e, Ptr::new(upper));
        let ctype = e.mem.u32(info + 0xc0);
        free(e, Ptr::new(ctype));
    }
    let time = e.mem.u32(info + 0xd4);
    if time != C_LC_TIME && e.mem.u32(time + 0xb4) == 0 {
        e.call(FREE_LC_TIME, &[time]);
        let time = e.mem.u32(info + 0xd4);
        free(e, Ptr::new(time));
    }
    let mut category = info + 0x50;
    for _ in 0..6 {
        if e.mem.u32(category - 8) != C_LOCALE_NAMES {
            let name = e.mem.u32(category);
            if name != 0 && e.mem.u32(name) == 0 {
                free(e, Ptr::new(name));
            }
        }
        if e.mem.u32(category - 4) != 0 {
            let wide = e.mem.u32(category + 4);
            if wide != 0 && e.mem.u32(wide) == 0 {
                free(e, Ptr::new(wide));
            }
        }
        category += 0x10;
    }
    free(e, Ptr::new(info));
}

// Translated from 00ee13e5 (decompiled, FalloutNV.exe 1.4.0.525)
/// `___ansicp` (engine map; cdecl: locale id): the locale's ANSI code page
/// (`GetLocaleInfoA(lcid, LOCALE_IDEFAULTANSICODEPAGE, buffer, 6)`, read
/// with `atol`), -1 when it is not known.
// Translation incomplete: 00ee13e5 - the /GS stack cookie check (00ec408c) is not translated.
pub fn ansicp(e: &mut Engine, lcid: u32) -> u32 {
    e.with_stack(8, |e, buffer| {
        let buffer = buffer.addr();
        e.mem.set_u8(buffer + 6, 0);
        if e.call(IMPORT_GET_LOCALE_INFO_A, &[lcid, 0x1004, buffer, 6])
            .u32()
            == 0
        {
            u32::MAX
        } else {
            str2::atol_body(e, buffer) as u32
        }
    })
}

// Translated from 00ee142e (decompiled, FalloutNV.exe 1.4.0.525)
/// `___convertcp` (engine map; cdecl: from code page, to code page, text,
/// length by address, output, output size): converts the text through
/// wide characters (`MultiByteToWideChar` / `WideCharToMultiByte`; a length
/// of -1 means up to the terminator). The wide buffer is on the stack up to
/// 0x400 bytes (marked 0xCCCC), else `_malloc`'d (0xDDDD), released with
/// `__freea`. With an output buffer the result is that buffer; without one
/// a new block (`_calloc_crt`) sized by the conversion, or by the wide
/// count for two single-byte code pages, whose length is stored back
/// (unless it was -1). Returns 0 for the same code page or any failure.
// Translation incomplete: 00ee142e - the /GS stack cookie check (00ec408c) is not translated.
pub fn convertcp(
    e: &mut Engine,
    from: u32,
    to: u32,
    text: u32,
    length_slot: u32,
    output: u32,
    output_size: u32,
) -> u32 {
    let length = e.mem.u32(length_slot);
    if from == to {
        return 0;
    }
    // CPINFO at EBP-0x18 (MaxCharSize first).
    let single_byte = e.with_stack(0x14, |e, info| {
        let info = info.addr();
        e.call(IMPORT_GET_CP_INFO, &[from, info]).u32() != 0
            && e.mem.u32(info) == 1
            && e.call(IMPORT_GET_CP_INFO, &[to, info]).u32() != 0
            && e.mem.u32(info) == 1
    });
    let count = if single_byte {
        if length == u32::MAX {
            e.call(STRLEN_ADDR, &[text]).u32().wrapping_add(1)
        } else {
            length
        }
    } else {
        let count = e
            .call(
                IMPORT_MULTI_BYTE_TO_WIDE_CHAR,
                &[from, 1, text, length, 0, 0],
            )
            .u32();
        if count == 0 {
            return 0;
        }
        count
    };
    let fits = (count as i32) > 0 && count <= 0x7fff_fff0;
    let size = count.wrapping_mul(2).wrapping_add(8);
    let convert = |e: &mut Engine, wide: u32| {
        convert_wide(
            e,
            wide,
            ConvertArgs {
                from,
                to,
                text,
                length,
                length_slot,
                output,
                output_size,
                count,
                single_byte,
            },
        )
    };
    if fits && size <= 0x400 {
        // `_alloca` through the probe `00ec5ec0`: a stack block here.
        e.with_stack(size, |e, block| {
            let block = block.addr();
            e.mem.set_u32(block, 0xcccc);
            convert(e, block + 8)
        })
    } else {
        let mut wide = 0;
        if fits {
            let block = e.call(MALLOC_BODY, &[size]).u32();
            if block != 0 {
                e.mem.set_u32(block, 0xdddd);
                wide = block + 8;
            }
        }
        if wide == 0 {
            return 0;
        }
        convert(e, wide)
    }
}

/// The values [`convertcp`]'s second half works with.
struct ConvertArgs {
    from: u32,
    to: u32,
    text: u32,
    length: u32,
    length_slot: u32,
    output: u32,
    output_size: u32,
    count: u32,
    single_byte: bool,
}

/// The second half of [`convertcp`]: into wide characters at `wide`, then
/// out to the target code page; releases the wide buffer.
fn convert_wide(e: &mut Engine, wide: u32, a: ConvertArgs) -> u32 {
    let mut result = 0;
    mem::memset(e, wide, 0, a.count.wrapping_mul(2));
    let converted = e
        .call(
            IMPORT_MULTI_BYTE_TO_WIDE_CHAR,
            &[a.from, 1, a.text, a.length, wide, a.count],
        )
        .u32();
    if converted != 0 {
        if a.output != 0 {
            let written = e
                .call(
                    IMPORT_WIDE_CHAR_TO_MULTI_BYTE,
                    &[a.to, 0, wide, a.count, a.output, a.output_size, 0, 0],
                )
                .u32();
            if written != 0 {
                result = a.output;
            }
        } else {
            let mut size = a.count;
            let mut sized = true;
            if !a.single_byte {
                size = e
                    .call(
                        IMPORT_WIDE_CHAR_TO_MULTI_BYTE,
                        &[a.to, 0, wide, a.count, 0, 0, 0, 0],
                    )
                    .u32();
                sized = size != 0;
            }
            if sized {
                result = calloc_crt(e, 1, size).addr();
                if result != 0 {
                    let written = e
                        .call(
                            IMPORT_WIDE_CHAR_TO_MULTI_BYTE,
                            &[a.to, 0, wide, size, result, size, 0, 0],
                        )
                        .u32();
                    if written == 0 {
                        free(e, Ptr::new(result));
                        result = 0;
                    } else if a.length != u32::MAX {
                        e.mem.set_u32(a.length_slot, written);
                    }
                }
            }
        }
    }
    freea(e, wide);
    result
}

// Translated from 00ecd1c7 (decompiled, FalloutNV.exe 1.4.0.525)
/// The body of `_malloc` (engine map; cdecl: size). Above 0xFFFFFFE0 bytes
/// it calls the new handler, sets `ENOMEM` and returns 0. Otherwise, with
/// the CRT heap (`_crtheap`; none: banner, message 0x1E, exit 0xFF): from
/// the system heap (`HeapAlloc`, at least one byte) when `__active_heap` is
/// 1; else the small-block heap first when it is 3, then the system heap
/// with the size rounded up to 16. A failure retries while `_newmode` is
/// set and the new handler succeeds; otherwise `ENOMEM` and 0.
pub fn malloc_body(e: &mut Engine, size: u32) -> u32 {
    if size > 0xffff_ffe0 {
        e.call(CALL_NEW_HANDLER, &[size]);
        let slot = errno(e);
        e.mem.set_u32(slot.addr(), 0xc);
        return 0;
    }
    let mut request = size;
    loop {
        if e.mem.u32(LOCKS_READY) == 0 {
            ff_msgbanner(e);
            nmsg_write(e, 0x1e);
            e.call(0x00ec_cca9, &[0xff]);
        }
        let mode = e.mem.u32(ACTIVE_HEAP);
        let mut block = 0;
        let mut allocated = false;
        if mode == 1 {
            let bytes = if request != 0 { request } else { 1 };
            let heap = e.mem.u32(LOCKS_READY);
            block = e.call(IMPORT_HEAP_ALLOC, &[heap, 0, bytes]).u32();
            allocated = true;
        } else if mode == 3 {
            block = e.call(SMALL_BLOCK_ALLOC, &[request]).u32();
            allocated = block != 0;
        }
        if !allocated {
            if request == 0 {
                request = 1;
            }
            request = request.wrapping_add(0xf) & 0xffff_fff0;
            let heap = e.mem.u32(LOCKS_READY);
            block = e.call(IMPORT_HEAP_ALLOC, &[heap, 0, request]).u32();
        }
        if block != 0 {
            return block;
        }
        if e.mem.u32(NEW_MODE) == 0 {
            let slot = errno(e);
            e.mem.set_u32(slot.addr(), 0xc);
        } else if e.call(CALL_NEW_HANDLER, &[size]).u32() != 0 {
            request = size;
            continue;
        }
        let slot = errno(e);
        e.mem.set_u32(slot.addr(), 0xc);
        return 0;
    }
}

// Translated from 00ec8e14 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__freea` (engine map; cdecl: buffer): frees a non-null `_malloca`
/// buffer whose marker (8 bytes before it) is 0xDDDD (a heap block); a
/// stack block (0xCCCC) is left alone.
pub fn freea(e: &mut Engine, buffer: u32) {
    if buffer != 0 {
        let block = buffer - 8;
        if e.mem.u32(block) == 0xdddd {
            free(e, Ptr::new(block));
        }
    }
}

// Translated from 00ed11f8 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_freefls` (the FLS destructor `__mtinit` registers; Ghidra made no
/// function here, read from the bytes; stdcall: thread block): for a
/// non-null block frees its buffers (+0x24, +0x2C, +0x34, +0x3C, +0x40,
/// +0x44, +0x48 when set, +0x5C unless the static table), releases its
/// multibyte info under lock 0xD (freed at a zero count unless the static
/// one) and its locale info under lock 0xC (freed when unreferenced unless
/// it is the global or the static one; the `__finally` blocks at
/// `00ed1312`/`00ed131e`, the unlocks, are this function's own code), then
/// frees the block.
// Translation incomplete: 00ed11f8 - the SEH frame (__SEH_prolog4/__SEH_epilog4) is not translated.
pub fn freefls(e: &mut Engine, block: u32) {
    if block == 0 {
        return;
    }
    for offset in [0x24, 0x2c, 0x34, 0x3c, 0x40, 0x44, 0x48] {
        let buffer = e.mem.u32(block + offset);
        if buffer != 0 {
            free(e, Ptr::new(buffer));
        }
    }
    let table = e.mem.u32(block + 0x5c);
    if table != 0x010f_dca0 {
        free(e, Ptr::new(table));
    }
    lock(e, 0xd);
    let mbc = e.mem.u32(block + 0x68);
    if mbc != 0
        && e.call(IMPORT_INTERLOCKED_DECREMENT, &[mbc]).u32() == 0
        && mbc != INITIAL_MBC_INFO
    {
        free(e, Ptr::new(mbc));
    }
    unlock(e, 0xd);
    lock(e, 0xc);
    let info = e.mem.u32(block + 0x6c);
    if info != 0 {
        removelocaleref(e, info);
        if info != e.mem.u32(GLOBAL_LOCALE_INFO)
            && info != INITIAL_LOCALE_INFO
            && e.mem.u32(info) == 0
        {
            freetlocinfo(e, info);
        }
    }
    unlock(e, 0xc);
    free(e, Ptr::new(block));
}

// ---- Numbers --------------------------------------------------------------

// Platform: replaces 00ec782f (`eh vector constructor iterator', MSVC runtime;
// stdcall: array, element size, count, constructor, destructor): runs the
// constructor (`thiscall`) on each element in order. The destructor is only
// for unwinding after an exception, which the engine doesn't raise.
pub fn eh_vector_constructor_iterator(
    e: &mut Engine,
    array: u32,
    size: u32,
    count: i32,
    constructor: u32,
    _destructor: u32,
) {
    let mut element = array;
    for _ in 0..count.max(0) {
        e.call(constructor, &[element]);
        element = element.wrapping_add(size);
    }
}

/// Where the engine keeps the running thread's x87 control word (`FSTCW`/`FLDCW`): a word of
/// the thread's block past the exe's `.tls` data, holding `0x10000 | cw` once set; zero means
/// the control word every thread starts with, `0x027F` (all exceptions masked, 53-bit
/// precision, round to nearest: `crate::x87::DEFAULT_CONTROL_WORD`), the value the CRT's x87
/// code checks for. The FPU register the x87 arithmetic helpers round with
/// (`crate::x87::control_word`) always holds the same word: [`set_x87_control_word`] sets
/// both. A thread made by `CreateThread` keeps its word in its kernel object while it does
/// not run (`units::platform::run_as_thread` swaps it in).
const X87_CONTROL_WORD: u32 = 0x0e04;

/// The x87 control word as `FSTCW` reads it.
pub fn x87_control_word(e: &mut Engine) -> u16 {
    let at = e.tls() + X87_CONTROL_WORD;
    let slot = e.mem.u32(at);
    if slot & 0x1_0000 == 0 {
        crate::x87::DEFAULT_CONTROL_WORD
    } else {
        slot as u16
    }
}

/// `FLDCW`: loads the running thread's x87 control word, its word in the thread block and the
/// FPU register the x87 arithmetic helpers read (`crate::x87::set_control_word`), so precision
/// control (bits 8-9) applies to them from here on. The word is kept as the FPU keeps it:
/// reserved bit 6 always reads 1 and bits 7 and 13-15 always read 0, so `FSTCW` gives
/// `(word & 0x1F3F) | 0x40` (measured on the player's machine, 2026-10-21: `FLDCW 0x023F` then
/// `FSTCW` gives `0x027F`, `0x0000` gives `0x0040`, `0xFFFF` gives `0x1F7F`).
pub fn set_x87_control_word(e: &mut Engine, word: u16) {
    let word = (word & 0x1f3f) | 0x40;
    let at = e.tls() + X87_CONTROL_WORD;
    e.mem.set_u32(at, 0x1_0000 | u32::from(word));
    crate::x87::set_control_word(word);
}

// Translated from 00ed5abb (decompiled, FalloutNV.exe 1.4.0.525)
/// `_ctrlfp(newctrl, mask)` (LIBCMT, `cdecl`; `fabs` `00ec6cde` calls it): reads the x87
/// control word (`FSTCW`), loads `(old & !mask) | (newctrl & mask)` (its low 16 bits,
/// `FLDCW`) and returns the old control word sign-extended (`MOVSX`).
pub fn ctrlfp(e: &mut Engine, newctrl: u32, mask: u32) -> i32 {
    let old = x87_control_word(e);
    let new = (!mask & u32::from(old)) | (newctrl & mask);
    set_x87_control_word(e, new as u16);
    i32::from(old as i16)
}

// Translated from 00ec8660 (disassembled, FalloutNV.exe 1.4.0.525)
/// The stack probe the compiler calls before a large frame. It takes its inputs in
/// registers, so they come first: `size` is `EAX` (the bytes wanted) and `esp` the value
/// of `ESP` at entry (it points at the return address). It touches each page between the
/// current stack page and the new top (a read of `[page]`), moves the return address to the
/// new top and returns. The return value is the `ESP` the caller continues with
/// (`esp - size + 4`; when `size` is above `esp` the new top is address 0 and the return
/// address is stored there); `EAX` on return holds the return address (the word at `esp`).
pub fn alloca_probe(e: &mut Engine, size: u32, esp: u32) -> u32 {
    // `LEA ECX, [ESP + 4]` after the `PUSH ECX`, minus `EAX`; zero when that borrows.
    let top = esp.saturating_sub(size);
    let mut page = esp.wrapping_sub(4) & 0xffff_f000;
    while top < page {
        page = page.wrapping_sub(0x1000);
        // `TEST [EAX], EAX`: a read that commits the guard page.
        let _ = e.mem.u32(page);
    }
    let return_address = e.mem.u32(esp);
    e.mem.set_u32(top, return_address);
    top.wrapping_add(4)
}

/// The import slot of KERNEL32 `GetSystemTimeAsFileTime` (`00fdf184`) and `__aulldiv`
/// (`00ecc590`, four words `(low, high, divisor low, divisor high)`, 64-bit unsigned
/// quotient in `EDX:EAX`).
const GET_SYSTEM_TIME_AS_FILE_TIME_SLOT: u32 = 0x00fd_f184;
const AULLDIV: u32 = 0x00ec_c590;

// Translated from 00ecadda (disassembled, FalloutNV.exe 1.4.0.525)
/// `__time64(__time64_t *)` (cdecl): the seconds since 1970-01-01. The `FILETIME` from
/// `GetSystemTimeAsFileTime` (100 ns units since 1601) has 116444736000000000 added (as the
/// exe does, `0xfe624e212ac18000` modulo 2^64) and is divided by 10000000 through
/// `__aulldiv`. A result above 0x793406fff (the year 3000) becomes -1. The result is stored
/// to `out` when it is not null and returned (`EDX:EAX`).
pub fn time64(e: &mut Engine, out: Ptr) -> u64 {
    let (low, high) = e.with_stack(8, |e, filetime| {
        let function = e.mem.u32(GET_SYSTEM_TIME_AS_FILE_TIME_SLOT);
        e.call(function, &args![filetime]);
        (e.mem.u32(filetime.addr()), e.mem.u32(filetime.addr() + 4))
    });
    let (low, carry) = low.overflowing_add(0x2ac1_8000);
    let high = high
        .wrapping_add(0xfe62_4e21)
        .wrapping_add(u32::from(carry));
    let quotient = e
        .call(AULLDIV, &args![low, high, 0x0098_9680u32, 0u32])
        .u64();
    let (quotient_low, quotient_high) = (quotient as u32, (quotient >> 32) as u32);
    let result = if (quotient_high as i32) < 7 {
        quotient
    } else if (quotient_high as i32) > 7 || quotient_low > 0x9340_6fff {
        u64::MAX
    } else {
        quotient
    };
    if out.addr() != 0 {
        e.mem.set_u32(out.addr(), result as u32);
        e.mem.set_u32(out.addr() + 4, (result >> 32) as u32);
    }
    result
}

// ---- Program ---------------------------------------------------------------

/// `__onexit` (`00ec6553`), `__encode_pointer` (`00ed0f17`), `_msize` (`00ecd31f`) and
/// `__realloc_crt` (`00ed0d70`), all cdecl.
const ONEXIT: u32 = 0x00ec_6553;
const ENCODE_POINTER: u32 = 0x00ed_0f17;
const MSIZE: u32 = 0x00ec_d31f;
const REALLOC_CRT: u32 = 0x00ed_0d70;

// Translated from 00ec658f (disassembled, FalloutNV.exe 1.4.0.525)
/// `_atexit` (Ghidra's function ID), cdecl, a function: `__onexit(function)`, and 0 when
/// that answers it (nonzero), -1 when it answers 0.
pub fn atexit(e: &mut Engine, f: Ptr) -> i32 {
    let stored = e.call(ONEXIT, &args![f.addr()]).u32();
    if stored != 0 {
        0
    } else {
        -1
    }
}

/// The words at `01271a8c` and `01271a88`: the start and the end (one past the last
/// word) of the table of exit functions, both kept encoded (`__encode_pointer`). The names
/// (`__onexitbegin`, `__onexitend` in the CRT's source) are not confirmed in Ghidra.
const ONEXIT_BEGIN: u32 = 0x0127_1a8c;
const ONEXIT_END: u32 = 0x0127_1a88;

// Translated from 00ec6468 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__onexit_nolock` (Ghidra's function ID), cdecl, a function. The table's start and end
/// are decoded; an end below the start, or a byte count whose next word wraps, answers 0.
/// When the block (`_msize`) has no room for one more word it grows by its size, at most
/// 0x800 bytes (`__realloc_crt`; the sum wrapping or the call failing tries 0x10 more,
/// which failing or wrapping answers 0), the new start is stored encoded and the slot is
/// the old byte count (as words, `SAR`) into it. The function is stored encoded, the end
/// moved one word on (encoded) and the function answered.
pub fn onexit_nolock(e: &mut Engine, function: u32) -> u32 {
    let encoded = e.mem.u32(ONEXIT_BEGIN);
    let begin = e.call(DECODE_POINTER, &args![encoded]).u32();
    let encoded = e.mem.u32(ONEXIT_END);
    let end = e.call(DECODE_POINTER, &args![encoded]).u32();
    if end < begin {
        return 0;
    }
    let used = end.wrapping_sub(begin);
    if used.wrapping_add(4) < 4 {
        return 0;
    }
    let size = e.call(MSIZE, &args![begin]).u32();
    let mut slot = end;
    if size < used.wrapping_add(4) {
        let grow = size.min(0x800);
        let wanted = grow.wrapping_add(size);
        let mut block = 0;
        if wanted >= size {
            block = e.call(REALLOC_CRT, &args![begin, wanted]).u32();
        }
        if block == 0 {
            let wanted = size.wrapping_add(0x10);
            if wanted < size {
                return 0;
            }
            block = e.call(REALLOC_CRT, &args![begin, wanted]).u32();
            if block == 0 {
                return 0;
            }
        }
        slot = ((used as i32) >> 2)
            .wrapping_mul(4)
            .wrapping_add(block as i32) as u32;
        let encoded = e.call(ENCODE_POINTER, &args![block]).u32();
        e.mem.set_u32(ONEXIT_BEGIN, encoded);
    }
    let encoded = e.call(ENCODE_POINTER, &args![function]).u32();
    e.mem.set_u32(slot, encoded);
    let next = slot.wrapping_add(4);
    let encoded = e.call(ENCODE_POINTER, &args![next]).u32();
    e.mem.set_u32(ONEXIT_END, encoded);
    function
}

/// The `_initterm` ranges `doexit` runs, as the exe passes them (`PUSH end;
/// MOV EAX,start; CALL 00ecccd3`, at `00ecce65` and `00ecce75`): the
/// pre-terminators (`__xp_a`..`__xp_z` in the CRT's source) and the
/// terminators (`__xt_a`..`__xt_z`). The exe holds 3 pre-terminator slots:
/// `010113f4` empty, `010113f8` = `00ee628e` and `010113fc` = `00ecdffb`
/// (neither has a Ghidra function); and one terminator slot, empty.
pub const PRE_TERMINATORS: (u32, u32) = (0x0101_13f4, 0x0101_1400);
pub const TERMINATORS: (u32, u32) = (0x0101_1404, 0x0101_1408);

/// The words at `0126fde8` (set once the exit has finished: a second
/// `doexit` runs no handlers) and `0126fde4` (set when it starts), and the
/// byte at `0126fde0` (the caller's `retcaller` argument).
const EXIT_DONE: u32 = 0x0126_fde8;
const EXIT_STARTED: u32 = 0x0126_fde4;
const EXIT_RETURNS: u32 = 0x0126_fde0;

/// What a run of [`doexit`] did. The exe has no such report; it is how the
/// engine accounts for handlers it cannot run.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ExitReport {
    /// The handlers run, in order: exit functions, pre-terminators,
    /// terminators.
    pub ran: Vec<u32>,
    /// Handlers skipped because nobody has translated them.
    pub open: Vec<u32>,
    /// Handlers that stopped part-way in a function nobody has translated
    /// (handler, the stop); what they did before stays.
    pub stopped: Vec<(u32, String)>,
    /// `Some(code)` when the exe now ends the process with `___crtExitProcess`
    /// (`00eccca9`, `ExitProcess`): not run here, the host decides.
    pub exit_process: Option<i32>,
}

/// Runs one handler (`CALL f`, no arguments), or skips and counts it if it
/// isn't translated.
fn exit_handler(e: &mut Engine, f: u32, report: &mut ExitReport) {
    if !e.is_translated(f) {
        report.open.push(f);
        return;
    }
    match crate::static_init::guarded(e, |e| e.call(f, &[])) {
        Ok(_) => report.ran.push(f),
        Err(stop) => report.stopped.push((f, stop)),
    }
}

/// `_initterm` (`00ecccd3`): every non-null word of `start..end`, in order,
/// is a function called without arguments.
fn run_exit_table(e: &mut Engine, (start, end): (u32, u32), report: &mut ExitReport) {
    e.mem.map(start, end - start);
    let mut slot = start;
    while slot < end {
        let f = e.mem.u32(slot);
        if f != 0 {
            exit_handler(e, f, report);
        }
        slot = slot.wrapping_add(4);
    }
}

/// `__encoded_null` (`00ed0f89`, no arguments).
const ENCODED_NULL: u32 = 0x00ed_0f89;

/// The part of `doexit` that runs the exit functions (`00eccdd8`..`00ecce63`): the table's
/// start decoded (null: nothing to run) and its end; from the last word to the first,
/// each word that is not the encoded null is decoded, replaced by the encoded null and
/// called, and the walk restarts from the new end when the decoded start or end changed.
fn run_onexit(e: &mut Engine, report: &mut ExitReport) {
    e.mem.map(ONEXIT_END, 8);
    let encoded = e.mem.u32(ONEXIT_BEGIN);
    let mut low = e.call(DECODE_POINTER, &args![encoded]).u32();
    if low == 0 {
        return;
    }
    let encoded = e.mem.u32(ONEXIT_END);
    let end = e.call(DECODE_POINTER, &args![encoded]).u32();
    // The table as the walk last saw it (EBP-0x1c, EBP-0x20).
    let (mut seen_begin, mut seen_end) = (low, end);
    let mut at = end;
    loop {
        at = at.wrapping_sub(4);
        if at < low {
            break;
        }
        let null = e.call(ENCODED_NULL, &args![]).u32();
        if e.mem.u32(at) == null {
            continue;
        }
        // The exe compares `at` with the start a second time here (`JC`).
        if at < low {
            break;
        }
        let word = e.mem.u32(at);
        let f = e.call(DECODE_POINTER, &args![word]).u32();
        let null = e.call(ENCODED_NULL, &args![]).u32();
        e.mem.set_u32(at, null);
        exit_handler(e, f, report);
        let encoded = e.mem.u32(ONEXIT_BEGIN);
        let begin = e.call(DECODE_POINTER, &args![encoded]).u32();
        let encoded = e.mem.u32(ONEXIT_END);
        let end = e.call(DECODE_POINTER, &args![encoded]).u32();
        if seen_begin != begin || seen_end != end {
            (seen_begin, low, seen_end, at) = (begin, begin, end, end);
        }
    }
}

// Translation incomplete: 00eccd99 - the SEH frame (__SEH_prolog4) is not modelled; with `retcaller` 0 the exe ends in ___crtExitProcess (00eccca9, ExitProcess), reported in `exit_process` and not run; _lock(8) and _unlock(8) (00eddf39, 00edde5f) are no-ops (one thread).
// Translated from 00eccd99 (decompiled, FalloutNV.exe 1.4.0.525)
/// `doexit(code, quick, retcaller)`, the CRT's exit path (Ghidra: `doexit`;
/// called by `00eccec5` as `(code, 0, 0)`, by `00eccedb` as `(code, 1, 0)`
/// and by `00eccef1` as `(0, 0, 1)`; Ghidra labels those `_exit`, `__exit`
/// and `__cexit`). Unless an earlier call finished the exit (`0126fde8`
/// is 1), it sets `0126fde4` and `0126fde0` (the low byte of `retcaller`),
/// and, when `quick` is 0, runs
/// 1. the exit functions registered with [`atexit`], from the last to the
///    first (each cleared before it runs),
/// 2. the pre-terminators ([`PRE_TERMINATORS`]),
///
/// and always then 3. the terminators ([`TERMINATORS`]). With `retcaller` 0
/// it then sets `0126fde8` and ends the process; that last step is only
/// reported (`exit_process`).
///
/// A handler nobody has translated is skipped and counted in the report; one
/// that stops part-way is recorded with its stop. The registered form (the
/// address `00eccd99`, three words) discards the report.
pub fn doexit(e: &mut Engine, code: i32, quick: i32, retcaller: i32) -> ExitReport {
    let mut report = ExitReport::default();
    e.mem.map(EXIT_RETURNS, 0x10);
    // __lock(8): one thread.
    if e.mem.u32(EXIT_DONE) != 1 {
        e.mem.set_u32(EXIT_STARTED, 1);
        e.mem.set_u8(EXIT_RETURNS, retcaller as u8);
        if quick == 0 {
            run_onexit(e, &mut report);
            run_exit_table(e, PRE_TERMINATORS, &mut report);
        }
        run_exit_table(e, TERMINATORS, &mut report);
    }
    // 00ecceb0: _unlock(8) when `retcaller` is not 0 (no-op).
    if retcaller == 0 {
        e.mem.set_u32(EXIT_DONE, 1);
        // _unlock(8); then ___crtExitProcess(code): not run.
        report.exit_process = Some(code);
    }
    report
}

/// The registered form of [`doexit`]: `(code, quick, retcaller)`.
fn doexit_entry(e: &mut Engine, words: &[u32]) -> Ret {
    doexit(e, words[0] as i32, words[1] as i32, words[2] as i32);
    Ret::default()
}

// ---- The physics step's remaining runtime functions ----------------------------------------
//
// The x87 helpers below take and give what the exe passes in registers and flags: the uniform
// form lists register inputs first, in the order EAX, ECX, EDX, then `ST0` (two words), then the
// stack words from the one at `[ESP]` on; a flag input (`ZF`) is a `bool` word before them. The
// engine computes in `f64`, so `ST0` is an `f64`, and the control word is the one `ctrlfp` keeps.

/// `__use_sse2_mathfcns` (`01270a64`): nonzero when the CRT's SSE2 math routines may be used.
const USE_SSE2_MATHFCNS: u32 = 0x0127_0a64;
/// The CRT's math-error switch (`0126fd90`): nonzero skips the error handling and returns the
/// result as it is.
const MATH_ERROR_SWITCH: u32 = 0x0126_fd90;
/// `MXCSR` as the game runs with it (every exception masked, `0x1f80`). The SSE2 routines test
/// `STMXCSR`; the engine does not track the register.
const MXCSR: u32 = 0x1f80;

/// `FLD m64`: a double loaded onto the x87 stack. A signalling NaN becomes quiet (the quiet bit
/// is set, the payload kept); every other value is exact. There is no shared helper for
/// doubles (`x87_quiet` is for floats).
fn fld64(bits: u64) -> f64 {
    let exponent = 0x7ff0_0000_0000_0000u64;
    if bits & exponent == exponent && bits & 0x000f_ffff_ffff_ffff != 0 {
        f64::from_bits(bits | 0x0008_0000_0000_0000)
    } else {
        f64::from_bits(bits)
    }
}

/// `FLD tbyte` then `FST double` of an extended NaN or infinity at `address` (the CRT's default
/// "indefinite" NaN at `011c1d00` is one): the sign is kept, the fraction is the extended
/// significand without its integer bit, shifted to 52 bits.
fn fld_extended_special(e: &Engine, address: u32) -> f64 {
    let significand = e.mem.u64(address);
    let sign_exponent = e.mem.u16(address + 8);
    assert!(
        sign_exponent & 0x7fff == 0x7fff,
        "{address:08x} is not an extended NaN or infinity"
    );
    let sign = u64::from(sign_exponent >> 15) << 63;
    f64::from_bits(sign | 0x7ff0_0000_0000_0000 | ((significand << 1) >> 12))
}

/// Whether the SSE2 math routines run: `__use_sse2_mathfcns` is set and `MXCSR` and the x87
/// control word have their exception masks set (`MXCSR & 0x1f80 == 0x1f80`, control word
/// `& 0x7f == 0x7f`), the test every `_CIxxx` entry makes before it jumps to its SSE2 body.
fn sse2_math_available(e: &mut Engine) -> bool {
    e.global::<u32>(USE_SSE2_MATHFCNS) != 0
        && MXCSR & 0x1f80 == 0x1f80
        && x87_control_word(e) & 0x7f == 0x7f
}

/// `__sptype` (`00ed5981`, cdecl, the double on the stack) and its siblings `ceil`'s x87 body
/// calls: `__handle_qnan1` (`00ed5837`: function code, the double, the control word),
/// `__except1` (`00ed588c`: operation, function code, argument, result, control word),
/// `__frnd` (`00ee23bc`: the double, result in `ST0`) and `___libm_error_support` (`00ed429f`:
/// three pointers and a code).
const SPTYPE: u32 = 0x00ed_5981;
const HANDLE_QNAN1: u32 = 0x00ed_5837;
const EXCEPT1: u32 = 0x00ed_588c;
const FRND: u32 = 0x00ee_23bc;
const LIBM_ERROR_SUPPORT: u32 = 0x00ed_429f;
/// The control word `ceil`'s x87 body loads (`011c278c`, round up) and the constant it adds to
/// an infinity or NaN for the error report (`01012070`, 1.0).
const CEIL_CONTROL_WORD: u32 = 0x011c_278c;
const ONE_DOUBLE: u32 = 0x0101_2070;
/// The SSE2 constants of `ceil`: the shift base (`010f2000`, `0x433` per qword), the 1.0 mask
/// (`010f1ff0`), zero (`010f2010`), the exponent mask (`010f2020`, `0x7ff`) and minus zero
/// (`010f2030`).
const CEIL_SHIFT_BASE: u32 = 0x010f_2000;
const CEIL_ONE_MASK: u32 = 0x010f_1ff0;
const CEIL_ZERO: u32 = 0x010f_2010;
const CEIL_EXPONENT_MASK: u32 = 0x010f_2020;
const CEIL_MINUS_ZERO: u32 = 0x010f_2030;

// Translated from 00ec9e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ceil` (LIBCMT; no label in the map: named from the body, which rounds toward +infinity, and
/// from `011c278c`, the round-up control word its x87 path loads), cdecl, the double on the stack,
/// the result in `ST0`. With the SSE2 routines available (`sse2_math_available`) it computes the
/// result with integer shifts on the bit pattern: the shift count is `0x433 - exponent`, a
/// positive number below 1 gives 1.0 (zero stays +0), a negative one below 1 gives -0.0, up to
/// the exponent `0x432` the fraction bits are cleared (and 1.0 is added when that lowered a
/// positive value), and beyond that the value is returned as it is, a NaN after
/// `___libm_error_support(&x, &x, &x, 0x3ec)` has had its say about the result slot. Without them
/// it loads the round-up control word through `ctrlfp` (keeping the old one). A finite value is
/// rounded by `__frnd`; the old control word is put back and the rounded value returned when it
/// equals the argument or the old word has bit 5 (the precision-exception mask) set, otherwise
/// the inexact result is reported through `__except1` (operation `0x10`). An infinity or NaN goes
/// through `__sptype` of the argument as `FLD`/`FSTP` copy it (a signalling NaN quiet, so class
/// 3): classes 1 and 2 put the old control word back and return the value, class 3 is
/// `__handle_qnan1`, anything else `__except1` operation 8 with the value plus 1.0.
pub fn ceil(e: &mut Engine, x: f64) -> f64 {
    if sse2_math_available(e) {
        ceil_sse2(e, x)
    } else {
        ceil_x87(e, x)
    }
}

/// The SSE2 body of `ceil` (`00ec9e50`).
fn ceil_sse2(e: &mut Engine, x: f64) -> f64 {
    let bits = x.to_bits();
    let shifted = bits >> 52; // PSRLQ XMM0, 0x34
    let sign_and_exponent = shifted as u32; // MOVD EAX, XMM0
    let exponent_mask = e.mem.u64(CEIL_EXPONENT_MASK);
    let exponent = shifted & exponent_mask; // ANDPD XMM0, [010f2020]
    let base = e.mem.u64(CEIL_SHIFT_BASE);
    // PSUBD: the two dwords of the low qword subtract separately; the qword is the shift count.
    let count_low = (base as u32).wrapping_sub(exponent as u32);
    let count_high = ((base >> 32) as u32).wrapping_sub((exponent >> 32) as u32);
    let count = u64::from(count_low) | (u64::from(count_high) << 32);
    // PSRLQ / PSLLQ by a count above 63 give zero.
    let shift_right = |value: u64| if count > 63 { 0 } else { value >> count };
    let shift_left = |value: u64| if count > 63 { 0 } else { value << count };
    let one_mask = e.mem.u64(CEIL_ONE_MASK);
    if sign_and_exponent & 0x800 == 0 {
        // 00ec9ed2: a positive value.
        let truncated = f64::from_bits(shift_left(shift_right(bits)));
        let not_le = !(x <= truncated); // CMPNLEPD XMM0, XMM1
        if (sign_and_exponent as i32) < 0x3ff {
            // 00ec9f11
            let zero = f64::from_bits(e.mem.u64(CEIL_ZERO));
            let mask = if !(x <= zero) { u64::MAX } else { 0 };
            return fld64(mask & one_mask);
        }
        if (sign_and_exponent as i32) > 0x432 {
            return ceil_out_of_range(e, x);
        }
        let addend = if not_le { one_mask } else { 0 };
        return fld64((truncated + f64::from_bits(addend)).to_bits()); // ADDSD XMM1, XMM0
    }
    if (sign_and_exponent as i32) < 0xbff {
        return fld64(e.mem.u64(CEIL_MINUS_ZERO)); // 00ec9f0a
    }
    let truncated = shift_left(shift_right(bits));
    if (sign_and_exponent as i32) > 0xc32 {
        return ceil_out_of_range(e, x);
    }
    fld64(truncated)
}

/// `ceil`'s exit for a value whose exponent is at least `0x433` (`00ec9ea3`): a number comes back
/// as it is; a NaN is first given to `___libm_error_support`, which may rewrite the slot the
/// value sits in (all three pointers are that slot), and the slot is the result.
fn ceil_out_of_range(e: &mut Engine, x: f64) -> f64 {
    if !x.is_nan() {
        return fld64(x.to_bits());
    }
    e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), x.to_bits());
        e.call(LIBM_ERROR_SUPPORT, &args![slot, slot, slot, 0x3ecu32]);
        fld64(e.mem.u64(slot.addr()))
    })
}

/// The x87 body of `ceil` (`00ed969d`).
fn ceil_x87(e: &mut Engine, x: f64) -> f64 {
    let new_control = e.mem.u32(CEIL_CONTROL_WORD);
    let old_control = ctrlfp(e, new_control, 0xffff) as u32; // EBX
    let bits = x.to_bits();
    // The exponent word of the double, at `[EBP + 0xe]` (the bytes above it are masked away).
    let exponent_word = ((bits >> 48) as u32) & 0x7ff0;
    // FLD double ptr [EBP+8] (00ed96b7); FSTP double ptr [ESP] (00ed96ca): the pushed copy
    // `__sptype` and `__frnd` take, a signalling NaN quiet.
    let quiet = fld64(bits).to_bits();
    let (quiet_low, quiet_high) = (quiet as u32, (quiet >> 32) as u32);
    if exponent_word == 0x7ff0 {
        // 00ed96d2: __sptype of the pushed copy, so a signalling NaN is class 3.
        let class = e.call(SPTYPE, &args![quiet_low, quiet_high]).i32();
        if class > 0 {
            if class <= 2 {
                // 00ed96fc
                ctrlfp(e, old_control, 0xffff);
                return fld64(bits);
            }
            if class == 3 {
                return e
                    .call(
                        HANDLE_QNAN1,
                        &args![0xcu32, quiet_low, quiet_high, old_control],
                    )
                    .f64();
            }
        }
        // 00ed970a: the value plus 1.0, reported as an error of operation 8.
        let one = f64::from_bits(e.mem.u64(ONE_DOUBLE));
        let result = x87_add(fld64(bits), one).to_bits();
        return e
            .call(
                EXCEPT1,
                &args![
                    8u32,
                    0xcu32,
                    quiet_low,
                    quiet_high,
                    result as u32,
                    (result >> 32) as u32,
                    old_control
                ],
            )
            .f64();
    }
    // 00ed9727: __frnd of the same pushed copy (a finite value here, so the argument's bits).
    let rounded = e.call(FRND, &args![quiet_low, quiet_high]).f64();
    let (rounded_low, rounded_high) = (rounded.to_bits() as u32, (rounded.to_bits() >> 32) as u32);
    // FCOMP and the `TEST AH, 0x44` / `JP`: equal only when ordered and equal.
    if fld64(bits) == rounded || old_control & 0x20 != 0 {
        ctrlfp(e, old_control, 0xffff);
        return rounded;
    }
    e.call(
        EXCEPT1,
        &args![
            0x10u32,
            0xcu32,
            quiet_low,
            quiet_high,
            rounded_low,
            rounded_high,
            old_control
        ],
    )
    .f64()
}

/// `_CIasin`'s SSE2 body (`00ed7720`, `ST0`) and its x87 helpers: `00ed2808` (stack double in,
/// `EAX` = the exponent bits, or the whole high word for an infinity or NaN; `ZF` set exactly in
/// that case) and the x87 body `00ec96a8` (`ZF`, `EAX`, `ST0`, the stack double).
const ASIN_SSE2: u32 = 0x00ed_7720;
const LOAD_FLAGS_HELPER: u32 = 0x00ed_2808;
const ASIN_X87: u32 = 0x00ec_96a8;

// Translated from 00ec9650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_CIasin` (no label in the map: named from the body, the x87 arcsine at `00ec96a8` with
/// `FPATAN` of `x / sqrt((1 + x)(1 - x))` and the error string `"asin"` at `011c1930`; called by
/// the Euler-angle conversions): `ST0` in, `ST0` out. With the SSE2 routines available
/// (`sse2_math_available`) it jumps to the SSE2 body `00ed7720`. Otherwise it stores the double
/// on the stack (`FST`), runs `00ed2808` (which leaves `ZF` set when the exponent is all ones),
/// and runs the x87 body `00ec96a8` on it, returning its `ST0`. The register holds the argument
/// as `FLD qword` leaves a double, and `FST` stores it so: a signalling NaN is quiet in both,
/// so its error type is 7 (a quiet NaN), as the exe reports it.
pub fn ci_asin(e: &mut Engine, st0: f64) -> f64 {
    if sse2_math_available(e) {
        return e.call(ASIN_SSE2, &args![st0]).f64();
    }
    let st0 = x87_load64(st0.to_bits());
    let bits = st0.to_bits();
    let (low, high) = (bits as u32, (bits >> 32) as u32);
    let eax = e.call(LOAD_FLAGS_HELPER, &args![low, high]).u32();
    // `ZF` is the result of `00ed2808`'s `CMP EAX, 0x7ff00000`.
    let zf = high & 0x7ff0_0000 == 0x7ff0_0000;
    e.call(ASIN_X87, &args![zf, eax, st0, low, high]).f64()
}

/// The name `sqrt`'s error report is given (`"sqrt"`, `011c1740`).
const SQRT_NAME: u32 = 0x011c_1740;
/// The x87 square root's helpers: `00ed2795` (loads the control word `(old & 0x300) | 0x7f`; the
/// saved control word is its stack word), `00ed27ac` (`EAX` = high word, `ST0`: `EAX` 7 for a
/// quiet NaN, else 1 with `ST0 + 1.0`), `00ed282b` (the exit: `EDX` = operation, `ECX` = name,
/// `ST0`, the saved control word, the caller's return address, the double) and `00ed2737` (the
/// error handler: `EAX` = type, `ECX` = name, `EDX` = operation, `ST0`, the saved control word,
/// the caller's return address, the double).
const SET_CONTROL_WORD_HELPER: u32 = 0x00ed_2795;
const NAN_TYPE_HELPER: u32 = 0x00ed_27ac;
const MATH_EXIT: u32 = 0x00ed_282b;
const ONE_ARG_ERROR_HANDLING: u32 = 0x00ed_2737;
/// The indefinite NaN the CRT returns for a domain error (`011c1d00`, extended).
const INDEFINITE_NAN: u32 = 0x011c_1d00;

/// `00ec607e`: the end of the square-root paths that have a result. With the error switch set the
/// control word is restored (`00ed281e`: loaded back unless it is `0x027f`) and `ST0` returned;
/// otherwise the exit `00ed282b` runs with operation 5 and the name `"sqrt"`.
fn sqrt_exit(e: &mut Engine, saved_control: u16, st0: f64, low: u32, high: u32) -> f64 {
    if e.global::<u32>(MATH_ERROR_SWITCH) != 0 {
        if saved_control != 0x027f {
            set_x87_control_word(e, saved_control);
        }
        return st0;
    }
    e.call(
        MATH_EXIT,
        &args![
            5u32,
            SQRT_NAME,
            st0,
            u32::from(saved_control),
            0u32,
            low,
            high
        ],
    )
    .f64()
}

/// `00ec60db`: the end of the error paths, with the error type in `EAX`. With the error switch
/// set it restores the control word and returns `ST0`; otherwise `00ed2737` reports the error
/// (operation 5, name `"sqrt"`) and returns the result `ST0`.
fn sqrt_error(
    e: &mut Engine,
    saved_control: u16,
    st0: f64,
    error_type: u32,
    low: u32,
    high: u32,
) -> f64 {
    if e.global::<u32>(MATH_ERROR_SWITCH) != 0 {
        if saved_control != 0x027f {
            set_x87_control_word(e, saved_control);
        }
        return st0;
    }
    e.call(
        ONE_ARG_ERROR_HANDLING,
        &args![
            error_type,
            SQRT_NAME,
            5u32,
            st0,
            u32::from(saved_control),
            0u32,
            low,
            high
        ],
    )
    .f64()
}

// Translated from 00ec605d (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec605d - FSQRT's x87 status flags (read later by the exit 00ed282b through FSTSW) are not modelled
/// The shared x87 body of `_CIsqrt` (`00ec6040`, which stores `ST0` to the stack and calls
/// `00ed2808` first) and `sqrt` (`00ec6054`, which loads the stack double with `__fload_withFB`
/// and falls into this code); no label in the map, named from those two callers. Its inputs are
/// `ZF` (set exactly when the double's exponent is all ones), `ST0` and the double on the stack
/// (`low`, `high`), returning `ST0`. It saves the control word (`FSTCW`). A finite argument:
/// with a control word other than `0x027f` it first loads `(cw & 0x300) | 0x7f` (`00ed2795`);
/// a non-negative one takes `FSQRT` and ends in the exit (`sqrt_exit`); `-0.0` skips the `FSQRT`
/// and ends there with `-0.0`; any other negative one (a negative denormal included) is a domain
/// error: `ST0` becomes the indefinite NaN (`011c1d00`) with error type 1. An infinity or NaN
/// (`ZF` set): a NaN goes to `00ed27ac` for the error type and the new `ST0`, a positive infinity
/// ends in the exit unchanged, a negative one is the domain error. Error paths end in
/// `sqrt_error`.| 0x7f`
/// (`00ed2795`); a positive or zero-exponent-and-mantissa (`-0.0`) argument takes `FSQRT` (the
/// -0.0 skips it, keeping -0.0) and ends in the exit (`sqrt_exit`); any other negative argument is
/// a domain error: `ST0` becomes the indefinite NaN (`011c1d00`) with error type 1. An infinity
/// or NaN: a NaN goes to `00ed27ac` for the error type (and `ST0`), a positive infinity ends in
/// the exit unchanged, a negative one is the domain error. Error paths end in `sqrt_error`.
pub fn fn_00ec605d(e: &mut Engine, zf: bool, st0: f64, low: u32, high: u32) -> f64 {
    let saved_control = x87_control_word(e);
    if zf {
        // 00ec60b9
        if high & 0xfffff != 0 || low != 0 {
            let result = e.call(NAN_TYPE_HELPER, &args![high, st0]);
            return sqrt_error(e, saved_control, result.st0, result.eax, low, high);
        }
        if high & 0x8000_0000 == 0 {
            return sqrt_exit(e, saved_control, st0, low, high);
        }
    } else {
        if saved_control != 0x027f {
            e.call(SET_CONTROL_WORD_HELPER, &args![u32::from(saved_control)]);
        }
        if high & 0x8000_0000 == 0 {
            // FSQRT
            return sqrt_exit(e, saved_control, st0.sqrt(), low, high);
        }
        // 00ec609b: a negative argument; only -0.0 has no error.
        if high & 0x7ff0_0000 == 0 && high & 0xfffff == 0 && low == 0 {
            return sqrt_exit(e, saved_control, st0, low, high);
        }
    }
    // 00ec60ce: FSTP ST0, FLD the indefinite NaN, EAX = 1
    let indefinite = fld_extended_special(e, INDEFINITE_NAN);
    sqrt_error(e, saved_control, indefinite, 1, low, high)
}

/// `__getptd_noexit` (`00ed1165`): the calling thread's CRT data block, or 0 when it has none.
const GETPTD_NOEXIT: u32 = 0x00ed_1165;
/// The errno slot the CRT uses when a thread has no data block (`011c18c0`).
const ERRNO_FALLBACK: u32 = 0x011c_18c0;

// Translated from 00ec85e3 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_errno` (LIBCMT/dosmap.c; Ghidra: `__errno`), cdecl: the address of the thread's `errno`:
/// the `_terrno` field at `+8` of the thread's CRT data block (`__getptd_noexit`, `00ed1165`),
/// or the static slot `011c18c0` when the thread has no block. `__getptd_noexit` is
/// translated ([`getptd_noexit`]); it finds the block through the TLS/FLS slots that `__mtinit`
/// ([`mtinit`]) sets up, so the block exists once the host has run `__mtinit`.
pub fn errno(e: &mut Engine) -> Ptr {
    let block = e.call(GETPTD_NOEXIT, &args![]).u32();
    if block == 0 {
        Ptr::new(ERRNO_FALLBACK)
    } else {
        Ptr::new(block.wrapping_add(8))
    }
}

/// `__decode_pointer` (`00ed0f92`, cdecl, one word) and the encoded handler `_set_invalid_parameter_handler`
/// stores (`0126fd9c`).
const DECODE_POINTER: u32 = 0x00ed_0f92;
const INVALID_PARAMETER_HANDLER: u32 = 0x0126_fd9c;
/// `00ed5bae` (its body is `AND [01270a60], 0`, whatever argument it is given; the CRT pushes
/// 1 or 2 here) and `__invoke_watson` (`00ec7b08`).
const CLEAR_01270A60: u32 = 0x00ed_5bae;
const INVOKE_WATSON: u32 = 0x00ec_7b08;

// Translated from 00ec7c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra labels the body `__invalid_parameter` (function ID); the engine map names it
/// `_invalid_parameter_noinfo` (LIBCMT/invarg.c), but it takes the five arguments of
/// `_invalid_parameter` and the no-argument form is the caller at `00ec7c56`, so it is named
/// by address. Cdecl, five words that it only passes on: it decodes the handler stored by
/// `_set_invalid_parameter_handler` (`__decode_pointer(0126fd9c)`) and, if there is one, jumps to
/// it with the same arguments; without one it calls `00ed5bae(2)` and jumps to `__invoke_watson`
/// (`00ec7b08`) with the same arguments, which ends the process.
pub fn fn_00ec7c30(
    e: &mut Engine,
    expression: u32,
    function: u32,
    file: u32,
    line: u32,
    reserved: u32,
) {
    let encoded = e.global::<u32>(INVALID_PARAMETER_HANDLER);
    let handler = e.call(DECODE_POINTER, &args![encoded]).u32();
    if handler != 0 {
        e.call(handler, &args![expression, function, file, line, reserved]);
        return;
    }
    e.call(CLEAR_01270A60, &args![2u32]);
    e.call(
        INVOKE_WATSON,
        &args![expression, function, file, line, reserved],
    );
}

/// `__vsnprintf_l` (`00ecdd3e`, cdecl: destination, count, format, locale, argument list).
const VSNPRINTF_L: u32 = 0x00ec_dd3e;

// Translated from 00ecddf1 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_vsnprintf` (LIBCMT/vsnprint.c), cdecl: `_vsnprintf_l` with a null locale. The words go on in
/// the order destination, count, format, locale (0), argument list, and the result comes back.
pub fn vsnprintf(e: &mut Engine, dest: Ptr, count: u32, format: Ptr, va_list: u32) -> i32 {
    e.call(VSNPRINTF_L, &args![dest, count, format, 0u32, va_list])
        .i32()
}

// Translated from 00ecc590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__aulldiv` (Ghidra's function-ID label), stdcall (`RET 0x10`): the 64-bit unsigned quotient
/// `dividend / divisor` in `EDX:EAX`, the dividend and the divisor each as a low and a high word.
/// A divisor below 2^32 divides the high word first (`DIV`) and then the remainder with the low
/// word. Otherwise both operands are shifted right until the divisor's high word is zero, the
/// quotient estimate is `DIV` of the shifted values, and it is corrected by one: the divisor times
/// the estimate is formed from two `MUL`s, and the estimate is decremented when that product
/// carries or is greater than the dividend. A zero divisor stops with the `DIV` fault (#DE).
pub fn aulldiv(
    _e: &mut Engine,
    dividend_low: u32,
    dividend_high: u32,
    divisor_low: u32,
    divisor_high: u32,
) -> u64 {
    if divisor_high == 0 {
        assert!(divisor_low != 0, "00ecc590 __aulldiv: divide by zero (#DE)");
        // DIV ECX on 0:high, then on remainder:low.
        let high_quotient = dividend_high / divisor_low;
        let remainder = dividend_high % divisor_low;
        let low_quotient =
            ((u64::from(remainder) << 32) | u64::from(dividend_low)) / u64::from(divisor_low);
        return (u64::from(high_quotient) << 32) | (low_quotient & 0xffff_ffff);
    }
    let (mut shifted_divisor_high, mut shifted_divisor_low) = (divisor_high, divisor_low);
    let (mut shifted_high, mut shifted_low) = (dividend_high, dividend_low);
    loop {
        // SHR ECX, 1 / RCR EBX, 1 and SHR EDX, 1 / RCR EAX, 1
        let carry = shifted_divisor_high & 1;
        shifted_divisor_high >>= 1;
        shifted_divisor_low = (shifted_divisor_low >> 1) | (carry << 31);
        let carry = shifted_high & 1;
        shifted_high >>= 1;
        shifted_low = (shifted_low >> 1) | (carry << 31);
        if shifted_divisor_high == 0 {
            break;
        }
    }
    let estimate =
        ((u64::from(shifted_high) << 32) | u64::from(shifted_low)) / u64::from(shifted_divisor_low);
    assert!(
        estimate <= u64::from(u32::MAX),
        "00ecc590 __aulldiv: DIV overflow (#DE)"
    );
    let mut quotient = estimate as u32; // ESI
                                        // MUL [divisor high] (the low word of the product in ECX), then MUL ESI by the divisor low.
    let high_product = (u64::from(divisor_high) * u64::from(quotient)) as u32;
    let low_product = u64::from(divisor_low) * u64::from(quotient);
    let (sum, carry) = ((low_product >> 32) as u32).overflowing_add(high_product);
    let low_word = low_product as u32;
    // JC, then JA / JC / JBE on the comparisons with the dividend's words.
    let too_big = carry || sum > dividend_high || (sum == dividend_high && low_word > dividend_low);
    if too_big {
        quotient = quotient.wrapping_sub(1);
    }
    u64::from(quotient)
}

/// `___crtCorExitProcess` (`00eccc7e`, cdecl, the exit code) and the import slot of KERNEL32
/// `ExitProcess` (`00fdf0ec`, stdcall).
const CRT_COR_EXIT_PROCESS: u32 = 0x00ec_cc7e;
const EXIT_PROCESS: u32 = 0x00fd_f0ec;

// Translated from 00eccca9 (decompiled, FalloutNV.exe 1.4.0.525)
/// `___crtExitProcess` (Ghidra's function-ID label), cdecl, the exit code: gives it to
/// `___crtCorExitProcess` (`00eccc7e`, which tells the .NET runtime if `mscoree.dll` is loaded)
/// and then to `ExitProcess`. `ExitProcess` does not return, so the engine stops with a panic
/// naming this address and the exit code.
pub fn crt_exit_process(e: &mut Engine, code: i32) {
    e.call(CRT_COR_EXIT_PROCESS, &args![code]);
    e.call(EXIT_PROCESS, &args![code]);
    panic!("00eccca9 ___crtExitProcess: ExitProcess({code}) ended the process");
}

// Translated from 00ecc7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strncpy` (LIBCMT/strncpy.c), cdecl: copies up to `count` bytes of the string at `source` to
/// `dest`, padding with zero bytes up to `count` once the terminator is copied, and returns
/// `dest`. It is the exe's word-at-a-time routine, run label by label: bytes until `source` is
/// 4-aligned, then 4-byte words (a word holding a zero byte is detected with the
/// `(w ^ 0xffffffff) ^ (w + 0x7efefeff)` mask `0x81010100` and stored cut at the zero, the rest
/// zero-filled by words), then the last `count % 4` bytes. It reads whole aligned words, so it
/// may read up to three bytes past the terminator, as the exe does.
pub fn strncpy(e: &mut Engine, dest: Ptr, source: Ptr, count: u32) -> Ptr {
    /// The labels of the routine, by address.
    #[derive(Clone, Copy)]
    enum At {
        L7dc,
        L7fe,
        L803,
        L81e,
        L826,
        L83c,
        L843,
        L848,
        L855,
        L85f,
        L8ad,
        L8b7,
        L8b9,
        L8c3,
    }
    if count == 0 {
        return dest; // 00ecc7b7: JZ 00ecc84f
    }
    let mut ecx = count;
    let mut ebx = count;
    let mut esi = source.addr();
    let mut edi = dest.addr();
    let mut eax = 0u32;
    let mut edx = 0u32;
    let mut at = if esi & 3 != 0 {
        At::L7dc
    } else {
        ecx >>= 2;
        if ecx != 0 {
            At::L85f
        } else {
            At::L803
        }
    };
    loop {
        match at {
            At::L7dc => {
                eax = (eax & !0xff) | u32::from(e.mem.u8(esi));
                esi = esi.wrapping_add(1);
                e.mem.set_u8(edi, eax as u8);
                edi = edi.wrapping_add(1);
                ecx = ecx.wrapping_sub(1);
                if ecx == 0 {
                    return dest;
                }
                if eax as u8 == 0 {
                    at = At::L81e;
                } else if esi & 3 != 0 {
                    at = At::L7dc;
                } else {
                    ebx = ecx;
                    ecx >>= 2;
                    at = if ecx != 0 { At::L85f } else { At::L7fe };
                }
            }
            At::L7fe => {
                ebx &= 3;
                if ebx == 0 {
                    return dest;
                }
                at = At::L803;
            }
            At::L803 => {
                eax = (eax & !0xff) | u32::from(e.mem.u8(esi));
                esi = esi.wrapping_add(1);
                e.mem.set_u8(edi, eax as u8);
                edi = edi.wrapping_add(1);
                if eax as u8 == 0 {
                    at = At::L848;
                } else {
                    ebx = ebx.wrapping_sub(1);
                    if ebx == 0 {
                        return dest;
                    }
                }
            }
            At::L81e => {
                at = if edi & 3 == 0 { At::L83c } else { At::L826 };
            }
            At::L826 => {
                e.mem.set_u8(edi, eax as u8);
                edi = edi.wrapping_add(1);
                ecx = ecx.wrapping_sub(1);
                if ecx == 0 {
                    return dest;
                }
                at = if edi & 3 != 0 { At::L826 } else { At::L83c };
            }
            At::L83c => {
                ebx = ecx;
                ecx >>= 2;
                at = if ecx != 0 { At::L8b7 } else { At::L843 };
            }
            At::L843 => {
                e.mem.set_u8(edi, eax as u8);
                edi = edi.wrapping_add(1);
                at = At::L848;
            }
            At::L848 => {
                ebx = ebx.wrapping_sub(1);
                if ebx == 0 {
                    return dest;
                }
                at = At::L843;
            }
            At::L855 => {
                e.mem.set_u32(edi, edx);
                edi = edi.wrapping_add(4);
                ecx = ecx.wrapping_sub(1);
                at = if ecx == 0 { At::L7fe } else { At::L85f };
            }
            At::L85f => {
                edx = 0x7efe_feff;
                eax = e.mem.u32(esi);
                edx = edx.wrapping_add(eax);
                eax ^= 0xffff_ffff;
                eax ^= edx;
                edx = e.mem.u32(esi);
                esi = esi.wrapping_add(4);
                if eax & 0x8101_0100 == 0 {
                    at = At::L855;
                } else if edx as u8 == 0 {
                    // 00ecc8a9
                    edx = 0;
                    e.mem.set_u32(edi, edx);
                    at = At::L8ad;
                } else if (edx >> 8) as u8 == 0 {
                    // 00ecc89f
                    edx &= 0xff;
                    e.mem.set_u32(edi, edx);
                    at = At::L8ad;
                } else if edx & 0x00ff_0000 == 0 {
                    // 00ecc895
                    edx &= 0xffff;
                    e.mem.set_u32(edi, edx);
                    at = At::L8ad;
                } else if edx & 0xff00_0000 != 0 {
                    at = At::L855;
                } else {
                    // 00ecc891
                    e.mem.set_u32(edi, edx);
                    at = At::L8ad;
                }
            }
            At::L8ad => {
                edi = edi.wrapping_add(4);
                eax = 0;
                ecx = ecx.wrapping_sub(1);
                at = if ecx == 0 { At::L8c3 } else { At::L8b7 };
            }
            At::L8b7 => {
                eax = 0;
                at = At::L8b9;
            }
            At::L8b9 => {
                e.mem.set_u32(edi, eax);
                edi = edi.wrapping_add(4);
                ecx = ecx.wrapping_sub(1);
                at = if ecx != 0 { At::L8b9 } else { At::L8c3 };
            }
            At::L8c3 => {
                ebx &= 3;
                if ebx == 0 {
                    return dest;
                }
                at = At::L843;
            }
        }
    }
}

// Translated from 00ed2ec8 (decompiled, FalloutNV.exe 1.4.0.525)
/// `write_char` (LIBCMT/outputs.c), a register-argument helper of the `printf` core: `eax` holds the
/// character in its low byte, `stream` is `ECX` (a `FILE`: `_ptr` +0, `_cnt` +4, `_base` +8,
/// `_flag` +0xc) and `count` is `ESI` (the number of characters written, `-1` on error). Unless
/// the stream is a string stream with no buffer (`_flag & 0x40` and `_base == 0`: nothing is
/// stored), it counts the stream down (`_cnt - 1`) and stores the byte through `_ptr` (which then
/// advances) while the count stays non-negative, otherwise `_flsbuf(char, stream)` (`00ed2d64`,
/// cdecl, the byte sign-extended). A result of -1 sets `*count` to -1 (`OR [ESI], EAX`); any other
/// adds 1 to `*count`. The result is `EAX`: the byte or the `_flsbuf` result, or the incoming
/// `EAX` when nothing was stored.
pub fn write_char(e: &mut Engine, eax: u32, stream: Ptr, count: Ptr) -> u32 {
    let (ptr_at, cnt_at, base_at, flag_at) = (
        stream.addr(),
        stream.addr().wrapping_add(4),
        stream.addr().wrapping_add(8),
        stream.addr().wrapping_add(0xc),
    );
    // TEST byte [ECX + 0xc], 0x40 / JZ; CMP [ECX + 8], 0 / JZ 00ed2ef8
    let skip = e.mem.u8(flag_at) & 0x40 != 0 && e.mem.u32(base_at) == 0;
    if skip {
        e.mem
            .set_u32(count.addr(), e.mem.u32(count.addr()).wrapping_add(1));
        return eax;
    }
    let remaining = e.mem.i32(cnt_at).wrapping_sub(1);
    e.mem.set_i32(cnt_at, remaining);
    let result = if remaining < 0 {
        // MOVSX EAX, AL, then _flsbuf(EAX, ECX).
        let character = i32::from(eax as u8 as i8) as u32;
        e.call(FLSBUF, &args![character, stream]).u32()
    } else {
        let at = e.mem.u32(ptr_at);
        e.mem.set_u8(at, eax as u8);
        e.mem.set_u32(ptr_at, at.wrapping_add(1));
        u32::from(eax as u8)
    };
    if result == 0xffff_ffff {
        let written = e.mem.u32(count.addr());
        e.mem.set_u32(count.addr(), written | result);
    } else {
        e.mem
            .set_u32(count.addr(), e.mem.u32(count.addr()).wrapping_add(1));
    }
    result
}

/// `_flsbuf` (`00ed2d64`, cdecl: the character, the stream).
const FLSBUF: u32 = 0x00ed_2d64;

// Translated from 00ed4638 (decompiled, FalloutNV.exe 1.4.0.525)
/// `write_string` (LIBCMT/outputs.c), a helper of the `printf` core with register arguments:
/// `count` is `EAX` (the characters-written counter, `ESI` inside), `string` `ECX`, `stream` `EDI`
/// and `length` the one stack word. For a string stream with no buffer (`_flag & 0x40` set,
/// `_base` zero) it just adds `length` to `*count`. Otherwise it writes the bytes through
/// `write_char` while `length` is positive; when a write leaves `*count` at -1 and `errno` is
/// `0x2a` (`EILSEQ`) it writes a `?` instead and goes on, with any other `errno` it stops.
pub fn write_string(e: &mut Engine, count: Ptr, string: Ptr, stream: Ptr, length: i32) {
    let mut eax = count.addr();
    let mut remaining = length;
    let mut at = string.addr();
    let flag = e.mem.u8(stream.addr().wrapping_add(0xc));
    if flag & 0x40 != 0 && e.mem.u32(stream.addr().wrapping_add(8)) == 0 {
        let written = e.mem.u32(count.addr());
        e.mem
            .set_u32(count.addr(), written.wrapping_add(length as u32));
        return;
    }
    while remaining > 0 {
        eax = (eax & !0xff) | u32::from(e.mem.u8(at));
        remaining -= 1;
        eax = write_char(e, eax, stream, count);
        at = at.wrapping_add(1);
        if e.mem.i32(count.addr()) != -1 {
            continue;
        }
        let slot = errno(e);
        if e.mem.u32(slot.addr()) != 0x2a {
            return;
        }
        eax = (eax & !0xff) | 0x3f;
        eax = write_char(e, eax, stream, count);
    }
}

// Translated from 00ecdf44 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__iob_func` (LIBCMT/_file.c), cdecl: the address of the CRT's table of the standard streams
/// (`stdin` first), `011c1a08`; reads nothing.
pub fn iob_func(_e: &mut Engine) -> Ptr {
    Ptr::new(0x011c_1a08)
}

// Translated from 00ed27c5 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__fload_withFB` (Ghidra's function-ID label), a fastcall helper (`ECX` unused, `EDX` = the
/// address of a double) that returns `ST0` and, in `EAX`, the double's exponent bits
/// (`high & 0x7ff00000`, which is not `0x7ff00000`) after `FLD double [EDX]`. For an exponent of
/// all ones (an infinity or NaN) it builds the extended-precision value by hand instead (exponent
/// `0x7fff`, the significand shifted up by 11 with its integer bit set, the sign kept) and loads
/// that, which does not quiet a signalling NaN; `EAX` is then the whole high word. It sets `ZF`
/// exactly in that case (`TEST EAX, 0`), clear otherwise (the `CMP`'s result). In the engine's
/// `f64` model both loads give the same bits.
pub fn fload_with_fb(e: &mut Engine, _unused_ecx: u32, edx: Ptr) -> Ret {
    let high = e.mem.u32(edx.addr().wrapping_add(4));
    let exponent = high & 0x7ff0_0000;
    let bits = e.mem.u64(edx.addr());
    if exponent != 0x7ff0_0000 {
        return Ret {
            eax: exponent,
            st0: f64::from_bits(bits),
            ..Ret::default()
        };
    }
    Ret {
        eax: high,
        st0: f64::from_bits(bits),
        ..Ret::default()
    }
}

/// The registered form of [`fload_with_fb`]: `(ECX, EDX)`, giving `EAX` and `ST0`.
fn fload_with_fb_entry(e: &mut Engine, words: &[u32]) -> Ret {
    fload_with_fb(e, words[0], Ptr::new(words[1]))
}

/// The CRT's exception record, context and debugger flag for `___report_gsfailure`:
/// `ExceptionCode` `0126fe18`, `ExceptionFlags` `0126fe1c`, `ContextFlags` `0126fe70` and the
/// `IsDebuggerPresent` result `0126fe68`. The pointer pair handed to the filter is the static
/// `010f2a40`.
const GS_EXCEPTION_CODE: u32 = 0x0126_fe18;
const GS_EXCEPTION_FLAGS: u32 = 0x0126_fe1c;
const GS_CONTEXT_FLAGS: u32 = 0x0126_fe70;
const GS_DEBUGGER_PRESENT: u32 = 0x0126_fe68;
const GS_EXCEPTION_POINTERS: u32 = 0x010f_2a40;
const IS_DEBUGGER_PRESENT: u32 = 0x00fd_f0f0;
const SET_UNHANDLED_EXCEPTION_FILTER: u32 = 0x00fd_f180;
const UNHANDLED_EXCEPTION_FILTER: u32 = 0x00fd_f17c;
const GET_CURRENT_PROCESS: u32 = 0x00fd_f110;
const TERMINATE_PROCESS: u32 = 0x00fd_f178;
/// `STATUS_STACK_BUFFER_OVERRUN`, the exception code and exit code of a failed cookie check.
const STACK_BUFFER_OVERRUN: u32 = 0xc000_0409;

// Translated from 00ed159c (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed159c - the caller's registers (EAX, ECX, EDX, EBX, ESI, EDI, EBP), segment registers, EFLAGS, return address and stack pointer that it saves into the context at 0126fefc..0126ff38 are machine state the engine does not have and are not written
/// `___report_gsfailure` (Ghidra's function-ID label), cdecl, no arguments: what the stack cookie
/// check (`00ec408c`) calls on a mismatch. It fills the CRT's static exception record and context
/// (`ExceptionCode` `0xc0000409`, `ExceptionFlags` 1, `ContextFlags` `0x10001`; the register
/// fields come from machine state and are skipped), keeps `IsDebuggerPresent()` in `0126fe68`,
/// calls `00ed5bae(1)`, `SetUnhandledExceptionFilter(NULL)` and
/// `UnhandledExceptionFilter(010f2a40)` (the record and context pointers), calls `00ed5bae(1)`
/// again when no debugger is present, and ends with `TerminateProcess(GetCurrentProcess(),
/// 0xc0000409)`. Nothing returns from that, so the engine stops with a panic naming this address
/// and the reason.
pub fn report_gsfailure(e: &mut Engine) {
    e.mem.set_u32(GS_CONTEXT_FLAGS, 0x0001_0001);
    e.mem.set_u32(GS_EXCEPTION_CODE, STACK_BUFFER_OVERRUN);
    e.mem.set_u32(GS_EXCEPTION_FLAGS, 1);
    let debugger = e.call(IS_DEBUGGER_PRESENT, &args![]).u32();
    e.mem.set_u32(GS_DEBUGGER_PRESENT, debugger);
    e.call(CLEAR_01270A60, &args![1u32]);
    e.call(SET_UNHANDLED_EXCEPTION_FILTER, &args![0u32]);
    e.call(UNHANDLED_EXCEPTION_FILTER, &args![GS_EXCEPTION_POINTERS]);
    if e.mem.u32(GS_DEBUGGER_PRESENT) == 0 {
        e.call(CLEAR_01270A60, &args![1u32]);
    }
    let process = e.call(GET_CURRENT_PROCESS, &args![]).u32();
    e.call(TERMINATE_PROCESS, &args![process, STACK_BUFFER_OVERRUN]);
    panic!(
        "00ed159c ___report_gsfailure: TerminateProcess(GetCurrentProcess(), 0xc0000409): \
         a stack buffer overrun (cookie mismatch) ended the process"
    );
}

/// `__fsopen` (`00ec9983`, cdecl: path, mode, share flag), `_lock_file` (`00ece01b`) and
/// `__unlock_file` (`00ece08e`) (cdecl, the stream), `_fclose_nolock` (`00ec9890`, cdecl, the
/// stream) and `_fwrite_nolock` (`00ecaf24`, cdecl: buffer, size, count, stream) (engine map).
const FSOPEN: u32 = 0x00ec_9983;
const LOCK_FILE: u32 = 0x00ec_e01b;
const UNLOCK_FILE: u32 = 0x00ec_e08e;
const FCLOSE_NOLOCK: u32 = 0x00ec_9890;
const FWRITE_NOLOCK: u32 = 0x00ec_af24;

// Translated from 00ec9a47 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fopen` (LIBCMT/fopen.c; the engine map's name, `std::basic_string<..>::c_str`, is a folded
/// one; Ghidra: `_fopen`), cdecl: `_fsopen(path, mode, _SH_DENYNO (0x40))`.
pub fn fopen(e: &mut Engine, path: Ptr, mode: Ptr) -> Ptr {
    e.call(FSOPEN, &args![path, mode, 0x40u32]).ptr()
}

// Translated from 00ec9907 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fclose` (LIBCMT/fclose.c, Ghidra's function ID), cdecl, the stream. A null stream sets
/// `errno` to `EINVAL` (0x16), calls the invalid-parameter handler (`00ec7c30` with five zeros)
/// and answers -1. A string stream (`_flag` +0xc has 0x40) gets its flags cleared and answers
/// -1. Otherwise, with the stream locked (`_lock_file`), `_fclose_nolock` answers, and the
/// stream is unlocked (the `__finally` block at `00ec997b`, this function's own code).
// Translation incomplete: 00ec9907 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated.
pub fn fclose(e: &mut Engine, stream: Ptr) -> i32 {
    let stream = stream.addr();
    if stream == 0 {
        let error = errno(e);
        e.mem.set_u32(error.addr(), 0x16);
        fn_00ec7c30(e, 0, 0, 0, 0, 0);
        return -1;
    }
    if e.mem.u8(stream + 0xc) & 0x40 != 0 {
        e.mem.set_u32(stream + 0xc, 0);
        return -1;
    }
    e.call(LOCK_FILE, &args![stream]);
    let result = e.call(FCLOSE_NOLOCK, &args![stream]).i32();
    e.call(UNLOCK_FILE, &args![stream]);
    result
}

// Translated from 00ecb086 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fwrite` (LIBCMT/fwrite.c), cdecl: buffer, element size, count, stream. Answers 0 when the
/// size or the count is 0; a null stream sets `errno` to `EINVAL` (0x16), calls the
/// invalid-parameter handler (`00ec7c30` with five zeros) and answers 0. Otherwise, with the
/// stream locked (`_lock_file`), `_fwrite_nolock` answers, and the stream is unlocked (the
/// `__finally` block at `00ecb0fe`, this function's own code).
// Translation incomplete: 00ecb086 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated.
pub fn fwrite(e: &mut Engine, buffer: Ptr, size: u32, count: u32, stream: Ptr) -> u32 {
    if size == 0 || count == 0 {
        return 0;
    }
    if stream.addr() == 0 {
        let error = errno(e);
        e.mem.set_u32(error.addr(), 0x16);
        fn_00ec7c30(e, 0, 0, 0, 0, 0);
        return 0;
    }
    e.call(LOCK_FILE, &args![stream]);
    let written = e
        .call(FWRITE_NOLOCK, &args![buffer, size, count, stream])
        .u32();
    e.call(UNLOCK_FILE, &args![stream]);
    written
}

/// `_stbuf` (`00ed7d48`, cdecl, the stream: answers a flag), the stream form of `_output_l`
/// (`00ed2f21`, cdecl: stream, format, locale, argument list) and `_ftbuf` (`00ed7de4`, cdecl:
/// the flag, the stream) (LIBCMT, engine map).
const STBUF: u32 = 0x00ed_7d48;
const OUTPUT_L: u32 = 0x00ed_2f21;
const FTBUF: u32 = 0x00ed_7de4;

// Translated from 00ec9774 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fprintf` (LIBCMT/fprintf.c, Ghidra's function ID), cdecl: stream, format, then the
/// variable arguments. A null stream or format sets `errno` to `EINVAL` (0x16), calls the
/// invalid-parameter handler (`00ec7c30` with five zeros) and answers -1 without locking.
/// Otherwise, with the stream locked (`_lock_file`): a stream that is not a string stream
/// (`_flag` +0xc without 0x40) whose handle's `ioinfo` entry (`_fileno`; `__badioinfo` for
/// handles -1 and -2) has a text-mode bit (+0x24 & 0x7f) or the unicode bit (& 0x80) is
/// refused the same way (`errno`, handler, -1). Otherwise `_stbuf`, `_output_l(stream, format,
/// NULL, the arguments)`, whose answer is the result, and `_ftbuf(flag, stream)`. The stream
/// is then unlocked (the `__finally` block at `00ec9886`, `_unlock_file`). The argument words
/// are copied to game memory as the `va_list`.
// Translation incomplete: 00ec9774 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated; its __finally block runs inline.
pub fn fprintf(e: &mut Engine, stream: Ptr, format: Ptr, args: &[u32]) -> i32 {
    let stream = stream.addr();
    if stream == 0 || format.addr() == 0 {
        let error = errno(e);
        e.mem.set_u32(error.addr(), 0x16);
        fn_00ec7c30(e, 0, 0, 0, 0, 0);
        return -1;
    }
    e.call(LOCK_FILE, &args![stream]);
    let mut result = 0i32;
    if e.mem.u8(stream + 0xc) & 0x40 == 0 {
        let handle = e.call(FILENO, &args![stream]).u32();
        let entry = |e: &Engine| {
            if handle == 0xffff_ffff || handle == 0xffff_fffe {
                BAD_IOINFO
            } else {
                ioinfo(e, handle)
            }
        };
        let refused =
            e.mem.u8(entry(e) + 0x24) & 0x7f != 0 || e.mem.u8(entry(e) + 0x24) & 0x80 != 0;
        if refused {
            let error = errno(e);
            e.mem.set_u32(error.addr(), 0x16);
            fn_00ec7c30(e, 0, 0, 0, 0, 0);
            result = -1;
        }
    }
    if result == 0 {
        let flag = e.call(STBUF, &args![stream]).u32();
        let size = 4 * (args.len() as u32).max(1);
        result = e.with_stack(size, |e, list| {
            for (i, word) in args.iter().enumerate() {
                e.mem.set_u32(list.addr() + 4 * i as u32, *word);
            }
            e.call(OUTPUT_L, &args![stream, format, 0u32, list]).i32()
        });
        e.call(FTBUF, &args![flag, stream]);
    }
    e.call(UNLOCK_FILE, &args![stream]);
    result
}

/// The registry entry of [`fprintf`]: stream, format, then the variable arguments.
fn fprintf_entry(e: &mut Engine, words: &[u32]) -> Ret {
    fprintf(e, Ptr::new(words[0]), Ptr::new(words[1]), &words[2..]).into_ret()
}

/// `_malloc_crt` (`00ed0cdf`), the counter `_stbuf` counts up (`_cflush`, `0126fdf0`), and the
/// two shared buffers' slots (`_stdbuf`, `012704a8`: stdout's, then stderr's).
const MALLOC_CRT: u32 = 0x00ed_0cdf;
const STDBUF: u32 = 0x0127_04a8;
/// `_isatty` (`00ee2145`) and `__iob_func` (`00ecdf44`) (LIBCMT, engine map; cdecl).
const ISATTY: u32 = 0x00ee_2145;
const IOB_FUNC: u32 = 0x00ec_df44;

// Translated from 00ed7d48 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_stbuf` (LIBCMT/_sftbuf.c, engine map), cdecl, the stream: gives `stdout` or `stderr` a
/// temporary buffer when it writes to a terminal. Answers 0 when the stream's handle
/// (`_fileno`) is not a terminal (`_isatty`), when it is neither `stdout` (`__iob_func() +
/// 0x20`) nor `stderr` (`+ 0x40`), or when it already has a buffer (`_flag` +0xc & 0x10c; the
/// `_cflush` counter at `0126fdf0` is counted up first). Otherwise the stream's shared buffer
/// (`_stdbuf` slot 0 or 1 at `012704a8`, allocated with 0x1000 bytes by `_malloc_crt` on first
/// use) becomes `_base` and `_ptr` with size and count 0x1000; when the allocation fails the
/// stream's own `_charbuf` word (+0x14) is used with size and count 2. Then `_flag` gets
/// 0x1102 and 1 is answered.
pub fn stbuf(e: &mut Engine, stream: Ptr) -> u32 {
    let stream = stream.addr();
    let handle = e.call(FILENO, &args![stream]).u32();
    if e.call(ISATTY, &args![handle]).u32() == 0 {
        return 0;
    }
    let index = if stream == e.call(IOB_FUNC, &args![]).u32().wrapping_add(0x20) {
        0u32
    } else if stream == e.call(IOB_FUNC, &args![]).u32().wrapping_add(0x40) {
        1
    } else {
        return 0;
    };
    let flushes = e.mem.u32(CFLUSH).wrapping_add(1);
    e.mem.set_u32(CFLUSH, flushes);
    if e.mem.u32(stream + 0xc) & 0x10c != 0 {
        return 0;
    }
    let slot = STDBUF + 4 * index;
    let (buffer, size) = if e.mem.u32(slot) == 0 {
        let block = e.call(MALLOC_CRT, &args![0x1000u32]).u32();
        e.mem.set_u32(slot, block);
        if block == 0 {
            (stream + 0x14, 2)
        } else {
            (block, 0x1000)
        }
    } else {
        (e.mem.u32(slot), 0x1000)
    };
    e.mem.set_u32(stream + 8, buffer);
    e.mem.set_u32(stream, buffer);
    e.mem.set_u32(stream + 0x18, size);
    e.mem.set_u32(stream + 4, size);
    let flags = e.mem.u32(stream + 0xc) | 0x1102;
    e.mem.set_u32(stream + 0xc, flags);
    1
}

// Translated from 00ed7de4 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_ftbuf` (LIBCMT/_sftbuf.c, engine map), cdecl: the flag `_stbuf` answered, the stream.
/// With the flag set and the stream's temporary-buffer bit (`_flag` +0xc & 0x1000), the stream
/// is flushed (`_flush`), loses bits 0x1100, and its buffer size (+0x18), `_ptr` and `_base`
/// are cleared.
pub fn ftbuf(e: &mut Engine, flag: u32, stream: Ptr) {
    if flag == 0 {
        return;
    }
    let stream = stream.addr();
    if e.mem.u32(stream + 0xc) & 0x1000 == 0 {
        return;
    }
    e.call(FLUSH, &args![stream]);
    let flags = e.mem.u32(stream + 0xc) & 0xffff_eeff;
    e.mem.set_u32(stream + 0xc, flags);
    e.mem.set_u32(stream + 0x18, 0);
    e.mem.set_u32(stream, 0);
    e.mem.set_u32(stream + 8, 0);
}

// Translated from 00ed2efb (decompiled, FalloutNV.exe 1.4.0.525)
/// `write_multi_char` (LIBCMT/output.c, engine map), a helper of the `printf` core: cdecl
/// character, repeat count and stream, with the characters-written counter in `EAX`. Writes
/// the character ([`write_char`]) while the repeat count, counted down first, was positive,
/// stopping when the counter becomes -1.
pub fn write_multi_char(e: &mut Engine, character: u32, repeat: i32, stream: Ptr, count: Ptr) {
    let mut left = repeat;
    while left > 0 {
        left -= 1;
        write_char(e, character, stream, count);
        if e.mem.i32(count.addr()) == -1 {
            break;
        }
    }
}

// Translated from 00ecc4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__aulldvrm` (LIBCMT, engine map): unsigned 64-bit division with remainder, two 64-bit
/// stack arguments (`RET 0x10`). A divisor below 2^32 divides in two 32-bit `DIV`s; a larger
/// one shifts both until the divisor fits, divides, and corrects the estimate down by one when
/// quotient times divisor overflows or exceeds the dividend: either way the exact quotient
/// (answered in `EDX:EAX`) and remainder (`EBX:ECX`, which the uniform form does not carry:
/// callers in this unit use the Rust function). A zero divisor faults (`DIV`), as here.
pub fn aulldvrm(_e: &mut Engine, dividend: u64, divisor: u64) -> (u64, u64) {
    (dividend / divisor, dividend % divisor)
}

/// The registry entry of [`aulldvrm`]: the quotient.
fn aulldvrm_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let dividend = u64::from(words[0]) | u64::from(words[1]) << 32;
    let divisor = u64::from(words[2]) | u64::from(words[3]) << 32;
    aulldvrm(e, dividend, divisor).0.into_ret()
}

/// `_wctomb_s_l` (`00ee21a9`, cdecl: size out, buffer, buffer size, character, locale).
const WCTOMB_S_L: u32 = 0x00ee_21a9;

// Translated from 00ee2315 (decompiled, FalloutNV.exe 1.4.0.525)
/// `wctomb_s` (LIBCMT/wctomb.c, Ghidra's function ID), cdecl: size out, buffer, buffer size,
/// wide character: `_wctomb_s_l` with a null locale.
pub fn wctomb_s(e: &mut Engine, size_out: Ptr, buffer: Ptr, size: u32, character: u32) -> i32 {
    e.call(WCTOMB_S_L, &args![size_out, buffer, size, character, 0u32])
        .i32()
}

/// The `printf` `%n` permission word (`0126fda8`) and the security cookie (`011c16bc`).
const PRINTF_COUNT_OUTPUT: u32 = 0x0126_fda8;
const SECURITY_COOKIE: u32 = 0x011c_16bc;

// Translated from 00ec9df9 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_get_printf_count_output` (LIBCMT/printf.c, Ghidra's function ID), no arguments: 1 when
/// the word at `0126fda8` equals the security cookie with bit 0 set (what
/// `_set_printf_count_output(1)` stores), else 0.
pub fn get_printf_count_output(e: &mut Engine) -> u32 {
    let enabled = e.mem.u32(SECURITY_COOKIE) | 1;
    u32::from(e.mem.u32(PRINTF_COUNT_OUTPUT) == enabled)
}

/// What `_output_l` reads: the character class and state table (`__lookuptable`, read with
/// the character at `010f3080` and with class * 8 + state at `010f30a0`; one table, two
/// views), the null-string texts (`011c1de8` narrow, `011c1dec` wide), the encoded floating
/// point helpers (`_cfltcvt` `011c1cec`, `_forcdecpt` `011c1cf8`, `_cropzeros` `011c1cf4`),
/// `_isleadbyte_l`, `free` (`00ecd291`), `_get_printf_count_output` and `wctomb_s`.
const LOOKUP_CLASSES: u32 = 0x010f_3080;
const LOOKUP_STATES: u32 = 0x010f_30a0;
const NULL_STRING: u32 = 0x011c_1de8;
const NULL_WIDE_STRING: u32 = 0x011c_1dec;
const CFLTCVT: u32 = 0x011c_1cec;
const FORCDECPT: u32 = 0x011c_1cf8;
const CROPZEROS: u32 = 0x011c_1cf4;
const ISLEADBYTE_L: u32 = 0x00ed_419c;
const FREE_CRT: u32 = 0x00ec_d291;
const GET_PRINTF_COUNT_OUTPUT: u32 = 0x00ec_9df9;
const WCTOMB_S: u32 = 0x00ee_2315;

/// `_output_l`'s frame below its frame pointer (`SUB ESP,0x278`) and the locals it passes by
/// address, by their offset below it.
const OUTPUT_FRAME: u32 = 0x278;
const OUTPUT_LOCALE: u32 = 0x25c;
const OUTPUT_COUNT: u32 = 0x228;
const OUTPUT_TEXT_LENGTH: u32 = 0x220;
const OUTPUT_BUFFER: u32 = 0x20c;
const OUTPUT_BUFFER_SIZE: u32 = 0x200;
const OUTPUT_DIGITS_END: u32 = 0xd;
const OUTPUT_PREFIX: u32 = 0x22c;
const OUTPUT_DOUBLE: u32 = 0x278;
const OUTPUT_WIDE_SIZE: u32 = 0x268;
const OUTPUT_MULTIBYTE: u32 = 0xc;

/// `_output_l`'s flag bits (LIBCMT names): `FL_SIGN`, `FL_SIGNSP`, `FL_LEFT`, `FL_LEADZERO`,
/// `FL_LONG`, `FL_SHORT`, `FL_SIGNED`, `FL_ALTERNATE`, `FL_NEGATIVE`, `FL_FORCEOCTAL`,
/// `FL_WIDECHAR`, `FL_LONGLONG`, `FL_I64`.
const FL_SIGN: u32 = 0x1;
const FL_SIGNSP: u32 = 0x2;
const FL_LEFT: u32 = 0x4;
const FL_LEADZERO: u32 = 0x8;
const FL_LONG: u32 = 0x10;
const FL_SHORT: u32 = 0x20;
const FL_SIGNED: u32 = 0x40;
const FL_ALTERNATE: u32 = 0x80;
const FL_NEGATIVE: u32 = 0x100;
const FL_FORCEOCTAL: u32 = 0x200;
const FL_WIDECHAR: u32 = 0x800;
const FL_LONGLONG: u32 = 0x1000;
const FL_I64: u32 = 0x8000;

/// How `_output_l` goes on after a format character: integer conversion, then the output of
/// the text, then the heap buffer's release; or straight to the next format character.
#[derive(Clone, Copy, PartialEq)]
enum OutputStep {
    Integer,
    Output,
    Release,
    Next,
}

// Translated from 00ed2f21 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_output_l` (LIBCMT/output.c, engine map), the stream form, cdecl: stream, format, locale,
/// argument list. Answers the number of characters written, or -1.
///
/// Takes the locale (`_LocaleUpdate`). A null stream or format, or a stream that is not a
/// string stream (`_flag` +0xc without 0x40) whose handle's `ioinfo` entry (`__badioinfo` for
/// -1 and -2) has a text-mode bit (+0x24 & 0x7f) or the unicode bit (& 0x80), sets `errno` to
/// `EINVAL`, calls the invalid-parameter handler and answers -1. Then each format character
/// goes through the state machine of the lookup table (class from the character for 0x20 to
/// 0x78, else 0; next state from class * 8 + state, high nibble; the states' jump table at
/// `00ed3a99`): normal characters are written (a lead byte with the next one; a lead byte
/// ending the format is an invalid parameter), `%` resets the conversion, flags (`' '` `#` `+`
/// `-` `0`), width and precision (digits or `*`; a negative width is left alignment, a negative
/// precision none), size prefixes (`h`, `l`, `ll`, `w`, `I`, `I32`, `I64`; an `I` before
/// anything but those or `d i o u x X` is written as a normal character), and the conversion:
/// `c`/`C` (wide through `wctomb_s` into the buffer), `s`/`S` (counted up to the precision;
/// null gives `"(null)"`), `Z` (a counted string), `n` (only with `_get_printf_count_output`,
/// else an invalid parameter; 16 or 32 bits), `d i u o x X p` (32, 16 or 64 bits, divided with
/// `__aulldvrm`, digits from the end of the buffer, `#` prefixes `0`/`0x`/`0X`), and `a e f g`
/// and capitals (the `double` through the decoded `_cfltcvt`, a heap buffer above 0xa3 digits
/// of precision, `_forcdecpt` for `#` with precision 0, `_cropzeros` for `g` without `#`). Any
/// other conversion character outputs the previous text again. Output: the sign prefix,
/// blanks before (right alignment), the prefix, zeros (`0` without `-`), the text (wide text
/// converted character by character with `wctomb_s`; a failure makes the count -1), and blanks
/// after for left alignment while the count is not negative. The heap buffer is freed after
/// each conversion. The loop ends at the format's end or when the count turns negative; the
/// locale is released (bit 1 of the thread's `_ownlocale` cleared when the constructor set it).
// Translation incomplete: 00ed2f21 - the /GS stack cookie check (00ec408c) is not translated.
pub fn output_l(e: &mut Engine, stream: Ptr, format: Ptr, locale: u32, args: u32) -> i32 {
    e.with_stack(OUTPUT_FRAME, |e, frame| {
        let at = |offset: u32| frame.addr() + OUTPUT_FRAME - offset;
        let stream = stream.addr();
        let stream_ptr = Ptr::new(stream);
        let count = Ptr::new(at(OUTPUT_COUNT));
        let update = at(OUTPUT_LOCALE);
        let text_length = at(OUTPUT_TEXT_LENGTH);
        let mut argument = args;
        let mut hex_add = 0u32;
        let mut flags = 0u32;
        let mut width = 0i32;
        let mut precision = 0i32;
        let mut prefix_length = 0i32;
        let mut no_output = 0u32;
        let mut text_is_wide = 0u32;
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let fail = |e: &mut Engine| {
            let error = errno(e);
            e.mem.set_u32(error.addr(), 0x16);
            fn_00ec7c30(e, 0, 0, 0, 0, 0);
            locale_update_release(e, update);
            -1
        };
        if stream == 0 {
            return fail(e);
        }
        if e.mem.u8(stream + 0xc) & 0x40 == 0 {
            let handle = e.call(FILENO, &args![stream]).u32();
            let entry = |e: &Engine| {
                if handle == 0xffff_ffff || handle == 0xffff_fffe {
                    BAD_IOINFO
                } else {
                    ioinfo(e, handle)
                }
            };
            if e.mem.u8(entry(e) + 0x24) & 0x7f != 0 || e.mem.u8(entry(e) + 0x24) & 0x80 != 0 {
                return fail(e);
            }
        }
        let mut cursor = format.addr();
        if cursor == 0 {
            return fail(e);
        }
        let mut ch = e.mem.u8(cursor);
        e.mem.set_u32(count.addr(), 0);
        e.mem.set_u32(text_length, 0);
        let mut heap_buffer = 0u32;
        let mut state = 0u32;
        let mut cap_exponent = 0u32;
        let mut text = 0u32;
        while ch != 0 {
            cursor = cursor.wrapping_add(1);
            if e.mem.i32(count.addr()) < 0 {
                break;
            }
            let class = if ch.wrapping_sub(0x20) <= 0x58 {
                let entry = LOOKUP_CLASSES.wrapping_add(i32::from(ch as i8) as u32);
                (i32::from(e.mem.u8(entry) as i8) as u32) & 0xf
            } else {
                0
            };
            let entry = LOOKUP_STATES.wrapping_add(class * 8 + state);
            state = (i32::from(e.mem.u8(entry) as i8) >> 4) as u32;
            let mut step = OutputStep::Next;
            match state {
                0 => {
                    if output_normal_character(e, ch, &mut cursor, stream_ptr, count, update) {
                        return fail(e);
                    }
                }
                1 => {
                    precision = -1;
                    cap_exponent = 0;
                    no_output = 0;
                    width = 0;
                    prefix_length = 0;
                    flags = 0;
                    text_is_wide = 0;
                }
                2 => match ch {
                    0x20 => flags |= FL_SIGNSP,
                    0x23 => flags |= FL_ALTERNATE,
                    0x2b => flags |= FL_SIGN,
                    0x2d => flags |= FL_LEFT,
                    0x30 => flags |= FL_LEADZERO,
                    _ => {}
                },
                3 => {
                    if ch == b'*' {
                        argument = argument.wrapping_add(4);
                        width = e.mem.i32(argument.wrapping_sub(4));
                        if width < 0 {
                            flags |= FL_LEFT;
                            width = width.wrapping_neg();
                        }
                    } else {
                        width = width
                            .wrapping_mul(10)
                            .wrapping_add(i32::from(ch as i8))
                            .wrapping_sub(0x30);
                    }
                }
                4 => precision = 0,
                5 => {
                    if ch == b'*' {
                        argument = argument.wrapping_add(4);
                        precision = e.mem.i32(argument.wrapping_sub(4));
                        if precision < 0 {
                            precision = -1;
                        }
                    } else {
                        precision = precision
                            .wrapping_mul(10)
                            .wrapping_add(i32::from(ch as i8))
                            .wrapping_sub(0x30);
                    }
                }
                6 => match ch {
                    b'I' => {
                        let next = e.mem.u8(cursor);
                        if next == b'6' && e.mem.u8(cursor.wrapping_add(1)) == b'4' {
                            cursor = cursor.wrapping_add(2);
                            flags |= FL_I64;
                        } else if next == b'3' && e.mem.u8(cursor.wrapping_add(1)) == b'2' {
                            cursor = cursor.wrapping_add(2);
                            flags &= !FL_I64;
                        } else if !matches!(next, b'd' | b'i' | b'o' | b'u' | b'x' | b'X') {
                            state = 0;
                            let stop = output_normal_character(
                                e,
                                ch,
                                &mut cursor,
                                stream_ptr,
                                count,
                                update,
                            );
                            if stop {
                                return fail(e);
                            }
                        }
                    }
                    b'h' => flags |= FL_SHORT,
                    b'l' => {
                        if e.mem.u8(cursor) == b'l' {
                            cursor = cursor.wrapping_add(1);
                            flags |= FL_LONGLONG;
                        } else {
                            flags |= FL_LONG;
                        }
                    }
                    b'w' => flags |= FL_WIDECHAR,
                    _ => {}
                },
                7 => match ch {
                    b'A' | b'E' | b'G' | b'a' | b'e' | b'f' | b'g' => {
                        if matches!(ch, b'A' | b'E' | b'G') {
                            ch = ch.wrapping_add(0x20);
                            cap_exponent = 1;
                        }
                        flags |= FL_SIGNED;
                        let mut buffer = at(OUTPUT_BUFFER);
                        text = buffer;
                        let mut buffer_size = OUTPUT_BUFFER_SIZE;
                        match precision.cmp(&0) {
                            std::cmp::Ordering::Less => precision = 6,
                            std::cmp::Ordering::Equal => {
                                if ch == b'g' {
                                    precision = 1;
                                }
                            }
                            std::cmp::Ordering::Greater => {
                                if precision > 0x200 {
                                    precision = 0x200;
                                }
                                if precision > 0xa3 {
                                    let size = (precision as u32).wrapping_add(0x15d);
                                    heap_buffer = e.call(MALLOC_CRT, &args![size]).u32();
                                    if heap_buffer != 0 {
                                        text = heap_buffer;
                                        buffer_size = size;
                                        buffer = heap_buffer;
                                    } else {
                                        precision = 0xa3;
                                    }
                                }
                            }
                        }
                        let value = at(OUTPUT_DOUBLE);
                        let low = e.mem.u32(argument);
                        e.mem.set_u32(value, low);
                        let high = e.mem.u32(argument.wrapping_add(4));
                        e.mem.set_u32(value + 4, high);
                        argument = argument.wrapping_add(8);
                        let encoded = e.mem.u32(CFLTCVT);
                        let convert = e.call(DECODE_POINTER, &args![encoded]).u32();
                        e.call(
                            convert,
                            &args![
                                value,
                                buffer,
                                buffer_size,
                                i32::from(ch as i8) as u32,
                                precision,
                                cap_exponent,
                                update
                            ],
                        );
                        let alternate = flags & FL_ALTERNATE;
                        if alternate != 0 && precision == 0 {
                            let encoded = e.mem.u32(FORCDECPT);
                            let force = e.call(DECODE_POINTER, &args![encoded]).u32();
                            e.call(force, &args![buffer, update]);
                        }
                        if ch == b'g' && alternate == 0 {
                            let encoded = e.mem.u32(CROPZEROS);
                            let crop = e.call(DECODE_POINTER, &args![encoded]).u32();
                            e.call(crop, &args![buffer, update]);
                        }
                        if e.mem.u8(buffer) == b'-' {
                            flags |= FL_NEGATIVE;
                            buffer = buffer.wrapping_add(1);
                            text = buffer;
                        }
                        let length = e.call(STRLEN_ADDR, &args![buffer]).u32();
                        e.mem.set_u32(text_length, length);
                        step = OutputStep::Output;
                    }
                    b'C' | b'c' => {
                        if ch == b'C' && flags & 0x830 == 0 {
                            flags |= FL_WIDECHAR;
                        }
                        argument = argument.wrapping_add(4);
                        if flags & 0x810 != 0 {
                            let character = u32::from(e.mem.u16(argument.wrapping_sub(4)));
                            let failed = e
                                .call(
                                    WCTOMB_S,
                                    &args![
                                        text_length,
                                        at(OUTPUT_BUFFER),
                                        OUTPUT_BUFFER_SIZE,
                                        character
                                    ],
                                )
                                .u32();
                            if failed != 0 {
                                no_output = 1;
                            }
                        } else {
                            let byte = e.mem.u8(argument.wrapping_sub(4));
                            e.mem.set_u8(at(OUTPUT_BUFFER), byte);
                            e.mem.set_u32(text_length, 1);
                        }
                        text = at(OUTPUT_BUFFER);
                        step = OutputStep::Output;
                    }
                    b'S' | b's' => {
                        if ch == b'S' && flags & 0x830 == 0 {
                            flags |= FL_WIDECHAR;
                        }
                        let mut limit = precision as u32;
                        if precision == -1 {
                            limit = 0x7fff_ffff;
                        }
                        argument = argument.wrapping_add(4);
                        let pointer = e.mem.u32(argument.wrapping_sub(4));
                        text = pointer;
                        if flags & 0x810 == 0 {
                            if pointer == 0 {
                                text = e.mem.u32(NULL_STRING);
                            }
                            let mut walk = text;
                            while limit != 0 {
                                limit -= 1;
                                if e.mem.u8(walk) == 0 {
                                    break;
                                }
                                walk = walk.wrapping_add(1);
                            }
                            e.mem.set_u32(text_length, walk.wrapping_sub(text));
                        } else {
                            if pointer == 0 {
                                text = e.mem.u32(NULL_WIDE_STRING);
                            }
                            text_is_wide = 1;
                            let mut walk = text;
                            while limit != 0 {
                                limit -= 1;
                                if e.mem.u16(walk) == 0 {
                                    break;
                                }
                                walk = walk.wrapping_add(2);
                            }
                            let length = (walk.wrapping_sub(text) as i32) >> 1;
                            e.mem.set_i32(text_length, length);
                        }
                        step = OutputStep::Output;
                    }
                    b'Z' => {
                        let counted = e.mem.u32(argument);
                        argument = argument.wrapping_add(4);
                        let buffer = if counted == 0 {
                            0
                        } else {
                            e.mem.u32(counted.wrapping_add(4))
                        };
                        if buffer == 0 {
                            text = e.mem.u32(NULL_STRING);
                            let length = e.call(STRLEN_ADDR, &args![text]).u32();
                            e.mem.set_u32(text_length, length);
                        } else {
                            let length = i32::from(e.mem.i16(counted));
                            text = buffer;
                            if flags & FL_WIDECHAR != 0 {
                                e.mem.set_i32(text_length, length / 2);
                                text_is_wide = 1;
                            } else {
                                text_is_wide = 0;
                                e.mem.set_i32(text_length, length);
                            }
                        }
                        step = OutputStep::Output;
                    }
                    b'n' => {
                        let target = e.mem.u32(argument);
                        argument = argument.wrapping_add(4);
                        if e.call(GET_PRINTF_COUNT_OUTPUT, &args![]).u32() == 0 {
                            return fail(e);
                        }
                        let written = e.mem.u32(count.addr());
                        if flags & FL_SHORT != 0 {
                            e.mem.set_u16(target, written as u16);
                        } else {
                            e.mem.set_u32(target, written);
                        }
                        no_output = 1;
                        step = OutputStep::Release;
                    }
                    b'd' | b'i' | b'u' => {
                        if ch != b'u' {
                            flags |= FL_SIGNED;
                        }
                        e.mem.set_u32(text_length, 10);
                        step = OutputStep::Integer;
                    }
                    b'o' => {
                        e.mem.set_u32(text_length, 8);
                        if flags & FL_ALTERNATE != 0 {
                            flags |= FL_FORCEOCTAL;
                        }
                        step = OutputStep::Integer;
                    }
                    b'p' | b'X' | b'x' => {
                        if ch == b'p' {
                            precision = 8;
                        }
                        hex_add = if ch == b'x' { 0x27 } else { 7 };
                        e.mem.set_u32(text_length, 0x10);
                        if flags & FL_ALTERNATE != 0 {
                            e.mem.set_u8(at(OUTPUT_PREFIX), b'0');
                            let letter = (hex_add as u8).wrapping_add(0x51);
                            e.mem.set_u8(at(OUTPUT_PREFIX) + 1, letter);
                            prefix_length = 2;
                        }
                        step = OutputStep::Integer;
                    }
                    _ => step = OutputStep::Output,
                },
                _ => {}
            }

            if step == OutputStep::Integer {
                let mut value: u64 = if flags & (FL_I64 | FL_LONGLONG) != 0 {
                    let low = e.mem.u32(argument);
                    let high = e.mem.u32(argument.wrapping_add(4));
                    argument = argument.wrapping_add(8);
                    u64::from(low) | u64::from(high) << 32
                } else {
                    argument = argument.wrapping_add(4);
                    if flags & FL_SHORT != 0 {
                        let word = e.mem.u16(argument.wrapping_sub(4));
                        if flags & FL_SIGNED != 0 {
                            i64::from(word as i16) as u64
                        } else {
                            u64::from(word)
                        }
                    } else {
                        let word = e.mem.u32(argument.wrapping_sub(4));
                        if flags & FL_SIGNED != 0 {
                            i64::from(word as i32) as u64
                        } else {
                            u64::from(word)
                        }
                    }
                };
                if flags & FL_SIGNED != 0 && (value as i64) < 0 {
                    value = value.wrapping_neg();
                    flags |= FL_NEGATIVE;
                }
                if flags & (FL_I64 | FL_LONGLONG) == 0 {
                    value &= 0xffff_ffff;
                }
                if precision < 0 {
                    precision = 1;
                } else {
                    flags &= !FL_LEADZERO;
                    if precision > 0x200 {
                        precision = 0x200;
                    }
                }
                if value == 0 {
                    prefix_length = 0;
                }
                let radix = i64::from(e.mem.i32(text_length)) as u64;
                let end = at(OUTPUT_DIGITS_END);
                let mut digit_at = end;
                loop {
                    let before = precision;
                    precision = precision.wrapping_sub(1);
                    if before <= 0 && value == 0 {
                        break;
                    }
                    let (quotient, remainder) = aulldvrm(e, value, radix);
                    let mut digit = (remainder as u32).wrapping_add(0x30);
                    value = quotient;
                    if (digit as i32) > 0x39 {
                        digit = digit.wrapping_add(hex_add);
                    }
                    e.mem.set_u8(digit_at, digit as u8);
                    digit_at = digit_at.wrapping_sub(1);
                }
                let mut length = end.wrapping_sub(digit_at);
                text = digit_at.wrapping_add(1);
                e.mem.set_u32(text_length, length);
                if flags & FL_FORCEOCTAL != 0 && (length == 0 || e.mem.u8(text) != b'0') {
                    text = text.wrapping_sub(1);
                    e.mem.set_u8(text, b'0');
                    length = length.wrapping_add(1);
                    e.mem.set_u32(text_length, length);
                }
                step = OutputStep::Output;
            }

            if step == OutputStep::Output {
                if no_output == 0 {
                    if flags & FL_SIGNED != 0 {
                        let sign = if flags & FL_NEGATIVE != 0 {
                            Some(b'-')
                        } else if flags & FL_SIGN != 0 {
                            Some(b'+')
                        } else if flags & FL_SIGNSP != 0 {
                            Some(b' ')
                        } else {
                            None
                        };
                        if let Some(sign) = sign {
                            e.mem.set_u8(at(OUTPUT_PREFIX), sign);
                            prefix_length = 1;
                        }
                    }
                    let length = e.mem.i32(text_length);
                    let padding = width.wrapping_sub(length).wrapping_sub(prefix_length);
                    if flags & (FL_LEFT | FL_LEADZERO) == 0 {
                        write_multi_char(e, 0x20, padding, stream_ptr, count);
                    }
                    let prefix = Ptr::new(at(OUTPUT_PREFIX));
                    write_string(e, count, prefix, stream_ptr, prefix_length);
                    if flags & FL_LEADZERO != 0 && flags & FL_LEFT == 0 {
                        write_multi_char(e, 0x30, padding, stream_ptr, count);
                    }
                    let length = e.mem.i32(text_length);
                    if text_is_wide != 0 && length > 0 {
                        let mut walk = text;
                        let mut remaining = length;
                        loop {
                            let character = u32::from(e.mem.u16(walk));
                            remaining -= 1;
                            walk = walk.wrapping_add(2);
                            let size_out = at(OUTPUT_WIDE_SIZE);
                            let multibyte = at(OUTPUT_MULTIBYTE);
                            let failed = e
                                .call(WCTOMB_S, &args![size_out, multibyte, 6u32, character])
                                .u32();
                            let size = e.mem.i32(size_out);
                            if failed != 0 || size == 0 {
                                e.mem.set_u32(count.addr(), 0xffff_ffff);
                                break;
                            }
                            write_string(e, count, Ptr::new(multibyte), stream_ptr, size);
                            if remaining == 0 {
                                break;
                            }
                        }
                    } else {
                        write_string(e, count, Ptr::new(text), stream_ptr, length);
                    }
                    if e.mem.i32(count.addr()) >= 0 && flags & FL_LEFT != 0 {
                        write_multi_char(e, 0x20, padding, stream_ptr, count);
                    }
                }
                step = OutputStep::Release;
            }

            if step == OutputStep::Release && heap_buffer != 0 {
                e.call(FREE_CRT, &args![heap_buffer]);
                heap_buffer = 0;
            }
            ch = e.mem.u8(cursor);
        }
        locale_update_release(e, update);
        e.mem.i32(count.addr())
    })
}

/// The registry entry of [`output_l`]: stream, format, locale, argument list.
fn output_l_entry(e: &mut Engine, words: &[u32]) -> Ret {
    output_l(
        e,
        Ptr::new(words[0]),
        Ptr::new(words[1]),
        words[2],
        words[3],
    )
    .into_ret()
}

/// A normal character of `_output_l` (state 0): written with [`write_char`]; a lead byte
/// (`_isleadbyte_l`) is written first and the next format byte with it. Answers true when that
/// next byte is the format's end (the caller's invalid-parameter exit).
fn output_normal_character(
    e: &mut Engine,
    ch: u8,
    cursor: &mut u32,
    stream: Ptr,
    count: Ptr,
    update: u32,
) -> bool {
    let mut character = ch;
    if e.call(ISLEADBYTE_L, &args![u32::from(ch), update]).u32() != 0 {
        write_char(e, u32::from(ch), stream, count);
        character = e.mem.u8(*cursor);
        *cursor = cursor.wrapping_add(1);
        if character == 0 {
            return true;
        }
    }
    write_char(e, u32::from(character), stream, count);
    false
}

/// The CRT's static stream table `_iob` (`011c1a08` to `011c1c68`, 0x20 bytes a stream): its
/// streams are locked through `_lock` number `0x10 + index`, other streams through the critical
/// section at their +0x20.
const IOB_FIRST: u32 = 0x011c_1a08;
const IOB_LAST: u32 = 0x011c_1c68;

/// The `_lock` number of a stream in `_iob`, or `None` for another stream.
fn iob_lock_number(stream: u32) -> Option<u32> {
    if (IOB_FIRST..=IOB_LAST).contains(&stream) {
        Some((((stream.wrapping_sub(IOB_FIRST)) as i32 >> 5) as u32).wrapping_add(0x10))
    } else {
        None
    }
}

// Translated from 00ece01b (decompiled, FalloutNV.exe 1.4.0.525)
/// `_lock_file` (LIBCMT/_file.c, engine map), cdecl, the stream: a stream of `_iob` takes its
/// `_lock` (0x10 + its index) and gets `_IOLOCKED` (0x8000) in its flags (+0xc); another
/// stream enters the critical section at its +0x20 (`EnterCriticalSection`).
pub fn lock_file(e: &mut Engine, stream: Ptr) {
    let stream = stream.addr();
    match iob_lock_number(stream) {
        Some(number) => {
            lock(e, number);
            let flags = e.mem.u32(stream + 0xc) | 0x8000;
            e.mem.set_u32(stream + 0xc, flags);
        }
        None => {
            e.call(
                IMPORT_ENTER_CRITICAL_SECTION,
                &args![stream.wrapping_add(0x20)],
            );
        }
    }
}

// Translated from 00ece08e (decompiled, FalloutNV.exe 1.4.0.525)
/// `_unlock_file` (LIBCMT/_file.c, engine map), cdecl, the stream: a stream of `_iob` loses
/// `_IOLOCKED` (0x8000) and leaves its `_lock` (0x10 + its index); another stream leaves the
/// critical section at its +0x20 (`LeaveCriticalSection`).
pub fn unlock_file(e: &mut Engine, stream: Ptr) {
    let stream = stream.addr();
    match iob_lock_number(stream) {
        Some(number) => {
            let flags = e.mem.u32(stream + 0xc) & 0xffff_7fff;
            e.mem.set_u32(stream + 0xc, flags);
            unlock(e, number);
        }
        None => {
            e.call(
                IMPORT_LEAVE_CRITICAL_SECTION,
                &args![stream.wrapping_add(0x20)],
            );
        }
    }
}

/// `_flush` (`00ecd8e6`), `_freebuf` (`00ed8207`), `_fileno` (`00ed806c`) and `_close`
/// (`00ed813a`) (LIBCMT, engine map; cdecl).
const FLUSH: u32 = 0x00ec_d8e6;
const FREEBUF: u32 = 0x00ed_8207;
const FILENO: u32 = 0x00ed_806c;
const CLOSE: u32 = 0x00ed_813a;

// Translated from 00ec9890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_fclose_nolock` (LIBCMT/fclose.c, engine map), cdecl, the stream. A null stream sets
/// `errno` to `EINVAL` (0x16), calls the invalid-parameter handler (`00ec7c30` with five zeros)
/// and answers -1. A stream in use (`_flag` +0xc has one of 0x83: read, write, read-write)
/// is flushed (`_flush`, whose result is the answer), its buffer freed (`_freebuf`) and its
/// handle closed (`_close` of `_fileno`); a failed close makes the answer -1, otherwise a
/// temporary file name at +0x1c is freed and cleared. The flags are then cleared; an unused
/// stream answers -1.
pub fn fclose_nolock(e: &mut Engine, stream: Ptr) -> i32 {
    let stream = stream.addr();
    if stream == 0 {
        let error = errno(e);
        e.mem.set_u32(error.addr(), 0x16);
        fn_00ec7c30(e, 0, 0, 0, 0, 0);
        return -1;
    }
    let mut result = -1i32;
    if e.mem.u8(stream + 0xc) & 0x83 != 0 {
        result = e.call(FLUSH, &args![stream]).i32();
        e.call(FREEBUF, &args![stream]);
        let handle = e.call(FILENO, &args![stream]).u32();
        if e.call(CLOSE, &args![handle]).i32() < 0 {
            result = -1;
        } else {
            let name = e.mem.u32(stream + 0x1c);
            if name != 0 {
                free(e, Ptr::new(name));
                e.mem.set_u32(stream + 0x1c, 0);
            }
        }
    }
    e.mem.set_u32(stream + 0xc, 0);
    result
}

/// `_getstream` (`00ed8501`; answers a free stream, locked, or 0) and `_openfile` (`00ed8238`;
/// cdecl: path, mode, share flag, stream) (LIBCMT, engine map).
const GETSTREAM: u32 = 0x00ed_8501;
const OPENFILE: u32 = 0x00ed_8238;

// Translated from 00ec9983 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_fsopen` (LIBCMT/fopen.c, engine map), cdecl: path, mode, share flag. A null path or mode or
/// an empty mode sets `errno` to `EINVAL` (0x16), calls the invalid-parameter handler
/// (`00ec7c30` with five zeros) and answers null. Otherwise a stream comes from `_getstream`
/// (none: `errno` `EMFILE` (0x18), null). An empty path sets `errno` to `EINVAL` and answers
/// null; otherwise `_openfile` answers. Either way the stream is unlocked afterwards (the
/// `__finally` block at `00ec9a3d`, `_unlock_file`; for the empty path reached through
/// `__local_unwind4`).
// Translation incomplete: 00ec9983 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) and the __local_unwind4 (00ed863c) call are not translated; its __finally block runs inline.
pub fn fsopen(e: &mut Engine, path: Ptr, mode: Ptr, share: u32) -> Ptr {
    let path = path.addr();
    let mode = mode.addr();
    if path == 0 || mode == 0 || e.mem.u8(mode) == 0 {
        let error = errno(e);
        e.mem.set_u32(error.addr(), 0x16);
        fn_00ec7c30(e, 0, 0, 0, 0, 0);
        return Ptr::new(0);
    }
    let stream = e.call(GETSTREAM, &args![]).u32();
    if stream == 0 {
        let error = errno(e);
        e.mem.set_u32(error.addr(), 0x18);
        return Ptr::new(0);
    }
    if e.mem.u8(path) == 0 {
        let error = errno(e);
        e.mem.set_u32(error.addr(), 0x16);
        e.call(UNLOCK_FILE, &args![stream]);
        return Ptr::new(0);
    }
    let opened = e.call(OPENFILE, &args![path, mode, share, stream]).u32();
    e.call(UNLOCK_FILE, &args![stream]);
    Ptr::new(opened)
}

/// `_write` (`00edc743`, cdecl: handle, buffer, count; LIBCMT, engine map).
const WRITE: u32 = 0x00ed_c743;

// Translated from 00ecaf24 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_fwrite_nolock` (LIBCMT/fwrite.c, engine map), cdecl: buffer, element size, count, stream.
/// 0 for a zero size or count; a null stream or buffer, or a count above `0xffffffff / size`,
/// sets `errno` to `EINVAL` (0x16), calls the invalid-parameter handler and answers 0. The
/// total (`size * count`) is written in a loop, with the buffer size taken from the stream
/// (+0x18) when it has a buffer (flags +0xc & 0x10c), else 0x1000: while the stream has room in
/// its buffer (flags & 0x108 and a count +4 not 0) the bytes are copied into it (`memcpy`, as
/// many as fit; a negative count marks an error); else a whole number of buffer sizes (all when
/// the buffer size is 0) is written straight to the handle (`_write` of `_fileno`, after
/// `_flush` when buffered), else one byte goes through `_flsbuf` (the buffer size re-read, 1
/// when not positive). Errors set `_IOERR` (0x20; not for a failed flush or `_flsbuf`) and
/// answer the whole elements written so far; a finished write answers the count.
pub fn fwrite_nolock(e: &mut Engine, buffer: Ptr, size: u32, count: u32, stream: Ptr) -> u32 {
    if size == 0 || count == 0 {
        return 0;
    }
    let stream = stream.addr();
    let buffer = buffer.addr();
    if stream == 0 || buffer == 0 || count > u32::MAX / size {
        let error = errno(e);
        e.mem.set_u32(error.addr(), 0x16);
        fn_00ec7c30(e, 0, 0, 0, 0, 0);
        return 0;
    }
    let total = size.wrapping_mul(count);
    let mut data = buffer;
    let mut left = total;
    let mut buffer_size = if e.mem.u32(stream + 0xc) & 0x10c != 0 {
        e.mem.u32(stream + 0x18)
    } else {
        0x1000
    };
    let written_elements = |left: u32| total.wrapping_sub(left) / size;
    if total == 0 {
        return count;
    }
    loop {
        let buffered = e.mem.u32(stream + 0xc) & 0x108;
        let room = e.mem.u32(stream + 4);
        if buffered != 0 && room != 0 {
            if (room as i32) < 0 {
                let flags = e.mem.u32(stream + 0xc) | 0x20;
                e.mem.set_u32(stream + 0xc, flags);
                return written_elements(left);
            }
            let chunk = if left < room { left } else { room };
            let target = e.mem.u32(stream);
            mem::memcpy(e, target, data, chunk);
            e.mem.set_u32(stream + 4, room.wrapping_sub(chunk));
            e.mem.set_u32(stream, target.wrapping_add(chunk));
            left = left.wrapping_sub(chunk);
            data = data.wrapping_add(chunk);
        } else if left >= buffer_size {
            if buffered != 0 && e.call(FLUSH, &args![stream]).u32() != 0 {
                return written_elements(left);
            }
            let chunk = if buffer_size != 0 {
                left - left % buffer_size
            } else {
                left
            };
            let handle = e.call(FILENO, &args![stream]).u32();
            let written = e.call(WRITE, &args![handle, data, chunk]).u32();
            if written == 0xffff_ffff {
                let flags = e.mem.u32(stream + 0xc) | 0x20;
                e.mem.set_u32(stream + 0xc, flags);
                return written_elements(left);
            }
            let done = if written > chunk { chunk } else { written };
            data = data.wrapping_add(done);
            left = left.wrapping_sub(done);
            if written < chunk {
                let flags = e.mem.u32(stream + 0xc) | 0x20;
                e.mem.set_u32(stream + 0xc, flags);
                return written_elements(left);
            }
        } else {
            let character = e.mem.u8(data) as i8 as i32 as u32;
            if e.call(FLSBUF, &args![character, stream]).i32() == -1 {
                return written_elements(left);
            }
            data = data.wrapping_add(1);
            left = left.wrapping_sub(1);
            buffer_size = e.mem.u32(stream + 0x18);
            if (buffer_size as i32) <= 0 {
                buffer_size = 1;
            }
        }
        if left == 0 {
            return count;
        }
    }
}

// ---- Low-level file handles (LIBCMT osfinfo.c, close.c, lseek.c, open.c) -----------

/// `__pioinfo` (`01270960`: 64 pointers to blocks of 32 `ioinfo` entries of 0x40 bytes:
/// `osfhnd` +0, `osfile` +4, `pipech` +5, `lockinitflag` +8, the lock's critical section
/// +0xc, `textmode`/`unicode` bits +0x24), `_nhandle` (`0127094c`), `__app_type`
/// (`011c19b0`, 1 for a console program) and `__badioinfo` (`011c26f0`).
const PIOINFO: u32 = 0x0127_0960;
const NHANDLE: u32 = 0x0127_094c;
const BAD_IOINFO: u32 = 0x011c_26f0;
/// The Windows imports this layer calls (import slots; names from the exe's import table).
const IMPORT_CLOSE_HANDLE: u32 = 0x00fd_f07c;
const IMPORT_CREATE_FILE_A: u32 = 0x00fd_f088;
const IMPORT_GET_FILE_TYPE: u32 = 0x00fd_f200;
const IMPORT_SET_STD_HANDLE: u32 = 0x00fd_f224;
const IMPORT_SET_FILE_POINTER: u32 = 0x00fd_f240;
/// The static `_doserrno` used when the thread has no CRT data block (`011c18c4`) and the
/// `errtable` of (Windows error, `errno`) pairs (`011c1758`, 45 entries).
const DOSERRNO_FALLBACK: u32 = 0x011c_18c4;
const ERROR_TABLE: u32 = 0x011c_1758;

/// The `ioinfo` entry of a handle as the code addresses it (`SAR` by 5 for the block, the low
/// five bits times 0x40 within it; no range check).
fn ioinfo(e: &Engine, handle: u32) -> u32 {
    let block = e
        .mem
        .u32(PIOINFO.wrapping_add((((handle as i32) >> 5) as u32).wrapping_mul(4)));
    block.wrapping_add((handle & 0x1f) << 6)
}

/// A handle inside the table: not negative and below `_nhandle` (signed, then unsigned, as the
/// code tests it).
fn handle_in_range(e: &Engine, handle: u32) -> bool {
    (handle as i32) >= 0 && handle < e.mem.u32(NHANDLE)
}

/// `errno` set to `code`.
fn set_errno(e: &mut Engine, code: u32) {
    let error = errno(e);
    e.mem.set_u32(error.addr(), code);
}

/// The current `errno`.
fn errno_value(e: &mut Engine) -> u32 {
    let slot = errno(e);
    e.mem.u32(slot.addr())
}

/// The current `_doserrno`.
fn doserrno_value(e: &mut Engine) -> u32 {
    let slot = doserrno(e);
    e.mem.u32(slot.addr())
}

/// `_doserrno` set to `code`.
fn set_doserrno(e: &mut Engine, code: u32) {
    let error = doserrno(e);
    e.mem.set_u32(error.addr(), code);
}

/// The invalid-parameter handler with five zeros (`00ec7c30`).
fn invalid_parameter_zeros(e: &mut Engine) {
    fn_00ec7c30(e, 0, 0, 0, 0, 0);
}

// Translated from 00ec85f6 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__doserrno` (LIBCMT/dosmap.c, engine map), cdecl: the address of the thread's
/// `_doserrno` (+0xc of its CRT data block, `__getptd_noexit`), or the static one at
/// `011c18c4` when the thread has no block.
pub fn doserrno(e: &mut Engine) -> Ptr {
    let block = e.call(GETPTD_NOEXIT, &args![]).u32();
    if block == 0 {
        Ptr::new(DOSERRNO_FALLBACK)
    } else {
        Ptr::new(block.wrapping_add(0xc))
    }
}

// Translated from 00ec85a1 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_get_errno_from_oserr` (LIBCMT/dosmap.c, engine map), cdecl: the `errno` for a Windows
/// error: the table at `011c1758` (45 pairs); errors 0x13 to 0x24 give `EACCES` (0xd); errors
/// 0xbc to 0xca give `ENOEXEC` (8); anything else `EINVAL` (0x16).
pub fn get_errno_from_oserr(e: &mut Engine, code: u32) -> u32 {
    for index in 0..0x2du32 {
        if code == e.mem.u32(ERROR_TABLE + 8 * index) {
            return e.mem.u32(ERROR_TABLE + 8 * index + 4);
        }
    }
    if code.wrapping_sub(0x13) <= 0x11 {
        return 0xd;
    }
    if code.wrapping_sub(0xbc) > 0xe {
        0x16
    } else {
        8
    }
}

// Translated from 00ec8609 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_dosmaperr` (LIBCMT/dosmap.c, engine map), cdecl: stores the Windows error in `_doserrno`
/// and its `errno` (`_get_errno_from_oserr`) in `errno`.
pub fn dosmaperr(e: &mut Engine, code: u32) {
    set_doserrno(e, code);
    let mapped = get_errno_from_oserr(e, code);
    set_errno(e, mapped);
}

// Translated from 00ee2e45 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_get_osfhandle` (LIBCMT/osfinfo.c, engine map), cdecl: the Windows handle of a CRT handle.
/// -2 gives `_doserrno` 0 and `errno` `EBADF` (9) and -1 without the handler; a handle out of
/// range or not open (`osfile` bit 0) does the same and calls the invalid-parameter handler.
pub fn get_osfhandle(e: &mut Engine, handle: u32) -> u32 {
    if handle == 0xffff_fffe {
        set_doserrno(e, 0);
        set_errno(e, 9);
        return 0xffff_ffff;
    }
    if handle_in_range(e, handle) {
        let entry = ioinfo(e, handle);
        if e.mem.u8(entry + 4) & 1 != 0 {
            return e.mem.u32(entry);
        }
    }
    set_doserrno(e, 0);
    set_errno(e, 9);
    invalid_parameter_zeros(e);
    0xffff_ffff
}

// Translated from 00ee2d3e (decompiled, FalloutNV.exe 1.4.0.525)
/// `_set_osfhnd` (LIBCMT/osfinfo.c, engine map), cdecl: handle, Windows handle. For a handle
/// in range whose `osfhnd` is still -1: in a console program handles 0, 1 and 2 also become
/// the standard input, output and error (`SetStdHandle` with -10, -11, -12); the Windows handle
/// is stored and 0 answered. Otherwise `errno` `EBADF`, `_doserrno` 0, -1.
pub fn set_osfhnd(e: &mut Engine, handle: u32, os_handle: u32) -> i32 {
    if handle_in_range(e, handle) {
        let entry = ioinfo(e, handle);
        if e.mem.u32(entry) == 0xffff_ffff {
            if e.mem.u32(APP_TYPE) == 1 {
                let standard = match handle {
                    0 => Some(0xffff_fff6u32),
                    1 => Some(0xffff_fff5),
                    2 => Some(0xffff_fff4),
                    _ => None,
                };
                if let Some(standard) = standard {
                    e.call(IMPORT_SET_STD_HANDLE, &args![standard, os_handle]);
                }
            }
            let entry = ioinfo(e, handle);
            e.mem.set_u32(entry, os_handle);
            return 0;
        }
    }
    set_errno(e, 9);
    set_doserrno(e, 0);
    -1
}

// Translated from 00ee2dbf (decompiled, FalloutNV.exe 1.4.0.525)
/// `_free_osfhnd` (LIBCMT/osfinfo.c, engine map), cdecl: for an open handle in range with a
/// Windows handle: in a console program handles 0, 1 and 2 clear the standard handle
/// (`SetStdHandle` with -10, -11, -12 and 0); `osfhnd` becomes -1 and 0 is answered.
/// Otherwise `errno` `EBADF`, `_doserrno` 0, -1.
pub fn free_osfhnd(e: &mut Engine, handle: u32) -> i32 {
    if handle_in_range(e, handle) {
        let entry = ioinfo(e, handle);
        if e.mem.u8(entry + 4) & 1 != 0 && e.mem.u32(entry) != 0xffff_ffff {
            if e.mem.u32(APP_TYPE) == 1 {
                let standard = match handle {
                    0 => Some(0xffff_fff6u32),
                    1 => Some(0xffff_fff5),
                    2 => Some(0xffff_fff4),
                    _ => None,
                };
                if let Some(standard) = standard {
                    e.call(IMPORT_SET_STD_HANDLE, &args![standard, 0u32]);
                }
            }
            let entry = ioinfo(e, handle);
            e.mem.set_u32(entry, 0xffff_ffff);
            return 0;
        }
    }
    set_errno(e, 9);
    set_doserrno(e, 0);
    -1
}

// Translated from 00ee2ebc (decompiled, FalloutNV.exe 1.4.0.525)
/// `__lock_fhandle` (LIBCMT/osfinfo.c, engine map), cdecl: the handle's lock. A lock not set up
/// yet (`lockinitflag` +8) is set up under lock 0xa (`__crtInitCritSecAndSpinCount` with 4000;
/// a failure makes the answer 0; the flag counts up either way); then, when it is ready, its
/// critical section is entered. Answers 1 or 0.
// Translation incomplete: 00ee2ebc - the SEH frame (__SEH_prolog4/__SEH_epilog4) is not translated; its __finally block (00ee2f53, _unlock(0xa)) runs inline.
pub fn lock_fhandle(e: &mut Engine, handle: u32) -> u32 {
    let entry = ioinfo(e, handle);
    let mut ready = 1u32;
    if e.mem.u32(entry + 8) == 0 {
        lock(e, 0xa);
        if e.mem.u32(entry + 8) == 0 {
            if crt_init_crit_sec_and_spin_count(e, entry + 0xc, 0xfa0) == 0 {
                ready = 0;
            }
            let flag = e.mem.u32(entry + 8).wrapping_add(1);
            e.mem.set_u32(entry + 8, flag);
        }
        unlock(e, 0xa);
    }
    if ready != 0 {
        let entry = ioinfo(e, handle);
        e.call(IMPORT_ENTER_CRITICAL_SECTION, &args![entry + 0xc]);
    }
    ready
}

// Translated from 00ee2f5c (decompiled, FalloutNV.exe 1.4.0.525)
/// `_unlock_fhandle` (LIBCMT/osfinfo.c), cdecl: leaves the handle's critical section (+0xc).
pub fn unlock_fhandle(e: &mut Engine, handle: u32) {
    let entry = ioinfo(e, handle);
    e.call(IMPORT_LEAVE_CRITICAL_SECTION, &args![entry + 0xc]);
}

// Translated from 00ee2f83 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_alloc_osfhnd` (LIBCMT/osfinfo.c), cdecl, no arguments: a free CRT handle, locked. Lock
/// 0xb is set up (`__mtinitlocknum`; -1 when that fails) and taken. Each of the 64 blocks is
/// searched in order: an existing block's entries not open (`osfile` bit 0 clear) get their
/// lock set up (under lock 0xa, as `__lock_fhandle` does; a failure skips the entry for good)
/// and entered; an entry still not open is taken (`osfile` 1, `osfhnd` -1, the handle answered
/// locked), else its lock is left and the search goes on. A missing block is allocated (32
/// entries of 0x40 bytes, `__calloc_crt`; `_nhandle` up by 32, each entry `osfhnd` -1,
/// `pipech` 0xa, `osfile` and `lockinitflag` 0), its first entry taken (`osfile` 1) and
/// locked (`__lock_fhandle`; -1 when that fails). Answers -1 when nothing is free.
// Translation incomplete: 00ee2f83 - the SEH frame (__SEH_prolog4/__SEH_epilog4) is not translated; its __finally blocks (00ee3056, 00ee3114: _unlock(0xa), _unlock(0xb)) run inline.
pub fn alloc_osfhnd(e: &mut Engine) -> u32 {
    let mut answer = 0xffff_ffffu32;
    let mut failed = false;
    if mtinitlocknum(e, 0xb) == 0 {
        return 0xffff_ffff;
    }
    lock(e, 0xb);
    let mut block = 0u32;
    while block < 0x40 {
        let first = e.mem.u32(PIOINFO + 4 * block);
        if first == 0 {
            let entries = calloc_crt(e, 0x20, 0x40).addr();
            if entries != 0 {
                e.mem.set_u32(PIOINFO + 4 * block, entries);
                let handles = e.mem.u32(NHANDLE).wrapping_add(0x20);
                e.mem.set_u32(NHANDLE, handles);
                let mut entry = entries;
                while entry < e.mem.u32(PIOINFO + 4 * block).wrapping_add(0x800) {
                    e.mem.set_u8(entry + 4, 0);
                    e.mem.set_u32(entry, 0xffff_ffff);
                    e.mem.set_u8(entry + 5, 0xa);
                    e.mem.set_u32(entry + 8, 0);
                    entry += 0x40;
                }
                answer = block << 5;
                let taken = ioinfo(e, answer);
                e.mem.set_u8(taken + 4, 1);
                if lock_fhandle(e, answer) == 0 {
                    answer = 0xffff_ffff;
                }
            }
            break;
        }
        let mut entry = first;
        while entry < e.mem.u32(PIOINFO + 4 * block).wrapping_add(0x800) {
            if e.mem.u8(entry + 4) & 1 == 0 {
                if e.mem.u32(entry + 8) == 0 {
                    lock(e, 0xa);
                    if e.mem.u32(entry + 8) == 0 {
                        if crt_init_crit_sec_and_spin_count(e, entry + 0xc, 0xfa0) == 0 {
                            failed = true;
                        } else {
                            let flag = e.mem.u32(entry + 8).wrapping_add(1);
                            e.mem.set_u32(entry + 8, flag);
                        }
                    }
                    unlock(e, 0xa);
                }
                if !failed {
                    e.call(IMPORT_ENTER_CRITICAL_SECTION, &args![entry + 0xc]);
                    if e.mem.u8(entry + 4) & 1 == 0 {
                        e.mem.set_u8(entry + 4, 1);
                        e.mem.set_u32(entry, 0xffff_ffff);
                        let index = ((entry.wrapping_sub(e.mem.u32(PIOINFO + 4 * block)) as i32)
                            >> 6) as u32;
                        answer = index.wrapping_add(block << 5);
                        break;
                    }
                    e.call(IMPORT_LEAVE_CRITICAL_SECTION, &args![entry + 0xc]);
                }
            }
            entry += 0x40;
        }
        if answer != 0xffff_ffff {
            break;
        }
        block += 1;
    }
    unlock(e, 0xb);
    answer
}

// Translated from 00edd009 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_lseek_nolock` (LIBCMT/lseek.c, engine map), cdecl: handle, offset, origin. A handle
/// without a Windows handle gives `EBADF` and -1; otherwise `SetFilePointer` (no high word)
/// moves it; a failure (-1 with an error from `GetLastError`) goes through `_dosmaperr` and
/// answers -1; success clears `FEOFLAG` (`osfile` 0x02) and answers the new position.
pub fn lseek_nolock(e: &mut Engine, handle: u32, offset: u32, origin: u32) -> u32 {
    let os_handle = get_osfhandle(e, handle);
    if os_handle == 0xffff_ffff {
        set_errno(e, 9);
        return 0xffff_ffff;
    }
    let position = e
        .call(
            IMPORT_SET_FILE_POINTER,
            &args![os_handle, offset, 0u32, origin],
        )
        .u32();
    let error = if position == 0xffff_ffff {
        e.call(IMPORT_GET_LAST_ERROR, &args![]).u32()
    } else {
        0
    };
    if error != 0 {
        dosmaperr(e, error);
        return 0xffff_ffff;
    }
    let entry = ioinfo(e, handle);
    let flags = e.mem.u8(entry + 4) & 0xfd;
    e.mem.set_u8(entry + 4, flags);
    position
}

// Translated from 00ee1fa7 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_lseeki64_nolock` (LIBCMT/lseeki64.c, engine map), cdecl: handle, 64-bit offset (two
/// words), origin; answers the 64-bit position (`EDX:EAX`). As `_lseek_nolock`, with the high
/// word passed to `SetFilePointer` through a stack word; -1 (both words) on failure, an error
/// only when `GetLastError` reports one.
pub fn lseeki64_nolock(e: &mut Engine, handle: u32, low: u32, high: u32, origin: u32) -> u64 {
    let os_handle = get_osfhandle(e, handle);
    if os_handle == 0xffff_ffff {
        set_errno(e, 9);
        return u64::MAX;
    }
    let (position, high) = e.with_stack(4, |e, word| {
        let word = word.addr();
        e.mem.set_u32(word, high);
        let position = e
            .call(
                IMPORT_SET_FILE_POINTER,
                &args![os_handle, low, word, origin],
            )
            .u32();
        (position, e.mem.u32(word))
    });
    if position == 0xffff_ffff {
        let error = e.call(IMPORT_GET_LAST_ERROR, &args![]).u32();
        if error != 0 {
            dosmaperr(e, error);
            return u64::MAX;
        }
    }
    let entry = ioinfo(e, handle);
    let flags = e.mem.u8(entry + 4) & 0xfd;
    e.mem.set_u8(entry + 4, flags);
    u64::from(position) | (u64::from(high) << 32)
}

// Translated from 00ee202c (decompiled, FalloutNV.exe 1.4.0.525)
/// `_lseeki64` (LIBCMT/lseeki64.c, engine map), cdecl: handle, 64-bit offset (two words),
/// origin. -2 gives `_doserrno` 0, `EBADF` and -1; a handle out of range or not open does the
/// same with the invalid-parameter handler. Otherwise, with the handle locked
/// (`__lock_fhandle`), `_lseeki64_nolock` answers when it is still open (else `EBADF`,
/// `_doserrno` 0, -1), and the handle is unlocked (the `__finally` block at `00ee213b`).
// Translation incomplete: 00ee202c - the SEH frame (__SEH_prolog4/__SEH_epilog4) is not translated; its __finally block runs inline.
pub fn lseeki64(e: &mut Engine, handle: u32, low: u32, high: u32, origin: u32) -> u64 {
    if handle == 0xffff_fffe {
        set_doserrno(e, 0);
        set_errno(e, 9);
        return u64::MAX;
    }
    if !handle_in_range(e, handle) || e.mem.u8(ioinfo(e, handle) + 4) & 1 == 0 {
        set_doserrno(e, 0);
        set_errno(e, 9);
        invalid_parameter_zeros(e);
        return u64::MAX;
    }
    lock_fhandle(e, handle);
    let position = if e.mem.u8(ioinfo(e, handle) + 4) & 1 != 0 {
        lseeki64_nolock(e, handle, low, high, origin)
    } else {
        set_errno(e, 9);
        set_doserrno(e, 0);
        u64::MAX
    };
    unlock_fhandle(e, handle);
    position
}

// Translated from 00ee2145 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_isatty` (LIBCMT/isatty.c), cdecl: `FDEV` (`osfile` 0x40) of a handle, 0 for -2 (with
/// `EBADF`) and for a handle out of range (`EBADF` and the invalid-parameter handler).
pub fn isatty(e: &mut Engine, handle: u32) -> u32 {
    if handle == 0xffff_fffe {
        set_errno(e, 9);
        return 0;
    }
    if !handle_in_range(e, handle) {
        set_errno(e, 9);
        invalid_parameter_zeros(e);
        return 0;
    }
    let entry = ioinfo(e, handle);
    (e.mem.u8(entry + 4) as i8 as i32 as u32) & 0x40
}

// Translated from 00ed809e (decompiled, FalloutNV.exe 1.4.0.525)
/// `_close_nolock` (LIBCMT/close.c, engine map), cdecl, the handle. A handle with a Windows
/// handle is closed (`CloseHandle`) unless it is handle 1 or 2 sharing its Windows handle with
/// the other while both are open; a failed close keeps `GetLastError`'s error. Then
/// `_free_osfhnd`, `osfile` cleared, and with an error `_dosmaperr` and -1, else 0.
pub fn close_nolock(e: &mut Engine, handle: u32) -> i32 {
    let mut error = 0u32;
    if get_osfhandle(e, handle) != 0xffff_ffff {
        let first = e.mem.u32(PIOINFO);
        let shared = (handle == 1 && e.mem.u8(first + 0x84) & 1 != 0)
            || (handle == 2 && e.mem.u8(first + 0x44) & 1 != 0);
        let skip = shared && {
            let standard_error = get_osfhandle(e, 2);
            get_osfhandle(e, 1) == standard_error
        };
        if !skip {
            let os_handle = get_osfhandle(e, handle);
            if e.call(IMPORT_CLOSE_HANDLE, &args![os_handle]).u32() == 0 {
                error = e.call(IMPORT_GET_LAST_ERROR, &args![]).u32();
            }
        }
    }
    free_osfhnd(e, handle);
    let entry = ioinfo(e, handle);
    e.mem.set_u8(entry + 4, 0);
    if error != 0 {
        dosmaperr(e, error);
        -1
    } else {
        0
    }
}

// Translated from 00ed813a (decompiled, FalloutNV.exe 1.4.0.525)
/// `_close` (LIBCMT/close.c, engine map), cdecl, the handle. -2 gives `_doserrno` 0, `EBADF`
/// and -1; a handle out of range or not open the same with the invalid-parameter handler.
/// Otherwise, with the handle locked, `_close_nolock` answers when it is still open (else
/// `EBADF` and -1), and the handle is unlocked (the `__finally` block at `00ed81fd`).
// Translation incomplete: 00ed813a - the SEH frame (__SEH_prolog4/__SEH_epilog4) is not translated; its __finally block runs inline.
pub fn close(e: &mut Engine, handle: u32) -> i32 {
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
    lock_fhandle(e, handle);
    let result = if e.mem.u8(ioinfo(e, handle) + 4) & 1 != 0 {
        close_nolock(e, handle)
    } else {
        set_errno(e, 9);
        -1
    };
    unlock_fhandle(e, handle);
    result
}

/// `_write_nolock` (`00edc010`, cdecl: handle, buffer, count; LIBCMT, engine map).
const WRITE_NOLOCK: u32 = 0x00ed_c010;

// Translated from 00edc743 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_write` (LIBCMT/write.c, engine map), cdecl: handle, buffer, count. -2 gives `_doserrno` 0,
/// `EBADF` and -1; a handle out of range or not open the same with the invalid-parameter
/// handler. Otherwise, with the handle locked, `_write_nolock` answers when it is still open
/// (else `EBADF`, `_doserrno` 0, -1), and the handle is unlocked (the `__finally` block at
/// `00edc815`).
// Translation incomplete: 00edc743 - the SEH frame (__SEH_prolog4/__SEH_epilog4) is not translated; its __finally block runs inline.
pub fn write(e: &mut Engine, handle: u32, buffer: Ptr, count: u32) -> i32 {
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
    lock_fhandle(e, handle);
    let result = if e.mem.u8(ioinfo(e, handle) + 4) & 1 != 0 {
        e.call(WRITE_NOLOCK, &args![handle, buffer, count]).i32()
    } else {
        set_errno(e, 9);
        set_doserrno(e, 0);
        -1
    };
    unlock_fhandle(e, handle);
    result
}

/// `_umask`'s mask (`0126fdb8`), `_commode` (`012708d8`) and
/// `_cflush` (`0126fdf0`).
const UMASK: u32 = 0x0126_fdb8;
const COMMODE: u32 = 0x0127_08d8;
const CFLUSH: u32 = 0x0126_fdf0;
/// `_read_nolock` (`00edc94a`, cdecl: handle, buffer, count) and `_chsize_nolock`
/// (`00ee5fc4`, cdecl: handle, 64-bit size) (LIBCMT, engine map).
const READ_NOLOCK: u32 = 0x00ed_c94a;
const CHSIZE_NOLOCK: u32 = 0x00ee_5fc4;

/// `_get_fmode` (`00ee6236`, cdecl: the address of a word; translated in its own unit,
/// `unplaced/run_00ee6236`).
const GET_FMODE: u32 = 0x00ee_6236;

/// How `__tsopen_nolock`'s steps after the open end.
enum OpenStep {
    Done,
    /// The handle is closed (`_close_nolock`) and `errno` answered (`00ee34b0`).
    Failed,
    /// The file is a big-endian UTF-16 one: closed, `EINVAL` answered.
    Invalid,
}

/// The byte-order mark (`EF BB BF` for mode 1, `FF FE` for mode 2) written through `_write`
/// until all of it is out; nothing for mode 0.
fn tsopen_write_mark(e: &mut Engine, handle: u32, encoding: u8) -> OpenStep {
    let (mark, length) = match encoding {
        1 => (0x00bf_bbefu32, 3u32),
        2 => (0xfeff, 2),
        _ => return OpenStep::Done,
    };
    e.with_stack(4, |e, buffer| {
        let buffer = buffer.addr();
        e.mem.set_u32(buffer, mark);
        let mut written = 0u32;
        loop {
            let wrote = write(
                e,
                handle,
                Ptr::new(buffer.wrapping_add(written)),
                length.wrapping_sub(written),
            );
            if wrote == -1 {
                return OpenStep::Failed;
            }
            written = written.wrapping_add(wrote as u32);
            if (length as i32) <= (written as i32) {
                return OpenStep::Done;
            }
        }
    })
}

/// The byte-order mark read (`_read_nolock`, 3 bytes): UTF-8 sets mode 1; `FE FF` closes the
/// file with `EINVAL`; `FF FE` seeks past it and sets mode 2; anything else seeks back to 0.
fn tsopen_read_mark(e: &mut Engine, handle: u32, encoding: &mut u8) -> OpenStep {
    e.with_stack(4, |e, buffer| {
        let buffer = buffer.addr();
        e.mem.set_u32(buffer, 0);
        let read = e.call(READ_NOLOCK, &args![handle, buffer, 3u32]).u32();
        if read == 0xffff_ffff {
            return OpenStep::Failed;
        }
        let mark = e.mem.u32(buffer);
        if read == 3 && mark == 0x00bf_bbef {
            *encoding = 1;
            return OpenStep::Done;
        }
        if read == 2 || read == 3 {
            let first = mark & 0xffff;
            if first == 0xfffe {
                close_nolock(e, handle);
                set_errno(e, 0x16);
                return OpenStep::Invalid;
            }
            if first == 0xfeff {
                if lseek_nolock(e, handle, 2, 0) == 0xffff_ffff {
                    return OpenStep::Failed;
                }
                *encoding = 2;
                return OpenStep::Done;
            }
        }
        if lseek_nolock(e, handle, 0, 0) == 0xffff_ffff {
            OpenStep::Failed
        } else {
            OpenStep::Done
        }
    })
}

/// The mark step by the access: write-only files get the mark written when created or
/// truncated (creations 1, 2, 5) or empty (3, 4; a non-empty one is only seeked back to 0);
/// read-only files have it read; read-write files as write-only, but a non-empty one has its
/// mark read after the seek back.
fn tsopen_mark(
    e: &mut Engine,
    handle: u32,
    access: u32,
    creation: u32,
    encoding: &mut u8,
) -> OpenStep {
    let read_write = match access & 0xc000_0000 {
        0x4000_0000 => false,
        0x8000_0000 => return tsopen_read_mark(e, handle, encoding),
        0xc000_0000 => true,
        _ => return OpenStep::Done,
    };
    match creation {
        0 => OpenStep::Done,
        1 | 2 | 5 => tsopen_write_mark(e, handle, *encoding),
        3 | 4 => {
            if lseeki64_nolock(e, handle, 0, 0, 2) == 0 {
                return tsopen_write_mark(e, handle, *encoding);
            }
            // `AND EAX,EDX` then `CMP EAX,-1`: both halves all ones.
            if lseeki64_nolock(e, handle, 0, 0, 0) == u64::MAX {
                return OpenStep::Failed;
            }
            if read_write {
                tsopen_read_mark(e, handle, encoding)
            } else {
                OpenStep::Done
            }
        }
        _ => OpenStep::Done,
    }
}

// Translated from 00ee311d (decompiled, FalloutNV.exe 1.4.0.525)
/// `__tsopen_nolock` (LIBCMT/open.c), cdecl with the result handle's address in `EAX` (the
/// first word here) and five stack words: the address of the caller's unlock flag, the path,
/// the open flags, the share flag and the permission mode. Answers 0 or an `errno`.
/// `CreateFileA`'s access (read 0x80000000, write 0x40000000, both 0xc0000000; write-only
/// with `_O_APPEND` and a Unicode mode reads too), share mode (`_SH_DENYRW` 0x10 ..
/// `_SH_DENYNO` 0x40, `_SH_SECURE` 0x80 shares reading for a read-only open), creation (from
/// the `_O_CREAT`/`_O_TRUNC`/`_O_EXCL` bits) and attributes (read-only when `_O_CREAT` with a
/// mode that `_umask` leaves without write; `_O_TEMPORARY` adds delete-on-close, delete access
/// and delete sharing; `_O_SHORT_LIVED`; `_O_SEQUENTIAL` or `_O_RANDOM`) come from the
/// arguments, the inheritance from `_O_NOINHERIT` (in `SECURITY_ATTRIBUTES` on the stack) and
/// text mode from `_O_BINARY`/`_O_TEXT`/the Unicode modes or `_get_fmode`. A handle comes from
/// `_alloc_osfhnd` (none: `EMFILE`); the unlock flag is set; the file is opened (a failed
/// read-write open with `_O_WRONLY` is retried write-only); `GetFileType` 0 closes it (`EACCES`
/// when there is no error), a pipe sets `FPIPE`, a character device `FDEV`; the handle is
/// stored (`_set_osfhnd`) with `FOPEN` and the flags. A text file opened read-write has a
/// trailing Ctrl-Z cut off (`_lseek_nolock`, `_read_nolock`, `_chsize_nolock`; a negative
/// seek is tolerated); a Unicode text file that is not a device gets its byte-order mark
/// handled (see the helpers). The entry's +0x24 takes the mode (low 7 bits) and `_O_WTEXT`'s
/// bit; `_O_APPEND` sets `FAPPEND` for a disk file; a file opened read-write only to read its
/// mark is reopened (`CloseHandle`, `CreateFileA` `OPEN_EXISTING` without read).
pub fn tsopen_nolock(
    e: &mut Engine,
    handle_out: Ptr,
    unlock_flag: Ptr,
    path: Ptr,
    flags: u32,
    share: u32,
    mode: u32,
) -> u32 {
    e.with_stack(0xc, |e, security| {
        tsopen_body(
            e,
            handle_out.addr(),
            unlock_flag.addr(),
            path.addr(),
            flags,
            share,
            mode,
            security.addr(),
        )
    })
}

/// `__tsopen_nolock` with its `SECURITY_ATTRIBUTES` stack block.
fn tsopen_body(
    e: &mut Engine,
    handle_out: u32,
    unlock_flag: u32,
    path: u32,
    flags: u32,
    share: u32,
    mode: u32,
    security: u32,
) -> u32 {
    let mut flags = flags;
    let mut encoding: u8 = 0;
    e.mem.set_u32(security, 0xc);
    e.mem.set_u32(security + 4, 0);
    let mut osfile: u8 = if flags & 0x80 != 0 {
        e.mem.set_u32(security + 8, 0);
        0x10
    } else {
        e.mem.set_u32(security + 8, 1);
        0
    };
    let file_mode = e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        if e.call(GET_FMODE, &args![word]).u32() != 0 {
            e.call(INVOKE_WATSON_ADDR, &[0, 0, 0, 0, 0]);
        }
        e.mem.u32(word.addr())
    });
    if flags & 0x8000 == 0 && (flags & 0x74000 != 0 || file_mode != 0x8000) {
        osfile |= 0x80;
    }
    let invalid = |e: &mut Engine| -> u32 {
        set_doserrno(e, 0);
        e.mem.set_u32(handle_out, 0xffff_ffff);
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        0x16
    };
    let mut access = match flags & 3 {
        0 => 0x8000_0000u32,
        1 => {
            if flags & 8 != 0 && flags & 0x70000 != 0 {
                0xc000_0000
            } else {
                0x4000_0000
            }
        }
        2 => 0xc000_0000,
        _ => return invalid(e),
    };
    let mut share_mode = match share {
        0x10 => 0u32,
        0x20 => 1,
        0x30 => 2,
        0x40 => 3,
        0x80 => u32::from(access == 0x8000_0000),
        _ => return invalid(e),
    };
    let creation = match flags & 0x700 {
        0 | 0x400 => 3u32,
        0x100 => 4,
        0x200 | 0x600 => 5,
        0x300 => 2,
        0x500 | 0x700 => 1,
        _ => return invalid(e),
    };
    let mut attributes = 0x80u32;
    if flags & 0x100 != 0 && (!e.mem.u32(UMASK) & mode) as u8 & 0x80 == 0 {
        attributes = 1;
    }
    if flags & 0x40 != 0 {
        attributes |= 0x400_0000;
        access |= 0x1_0000;
        share_mode |= 4;
    }
    if flags & 0x1000 != 0 {
        attributes |= 0x100;
    }
    if flags & 0x20 != 0 {
        attributes |= 0x800_0000;
    } else if flags & 0x10 != 0 {
        attributes |= 0x1000_0000;
    }
    let handle = alloc_osfhnd(e);
    e.mem.set_u32(handle_out, handle);
    if handle == 0xffff_ffff {
        set_doserrno(e, 0);
        e.mem.set_u32(handle_out, 0xffff_ffff);
        set_errno(e, 0x18);
        return errno_value(e);
    }
    e.mem.set_u32(unlock_flag, 1);
    let open = |e: &mut Engine, access: u32, creation: u32| -> u32 {
        e.call(
            IMPORT_CREATE_FILE_A,
            &args![path, access, share_mode, security, creation, attributes, 0u32],
        )
        .u32()
    };
    let mut os_handle = open(e, access, creation);
    if os_handle == 0xffff_ffff && access & 0xc000_0000 == 0xc000_0000 && flags & 1 != 0 {
        access &= 0x7fff_ffff;
        os_handle = open(e, access, creation);
    }
    let clear_open = |e: &mut Engine, handle: u32| {
        let entry = ioinfo(e, handle);
        let file = e.mem.u8(entry + 4) & 0xfe;
        e.mem.set_u8(entry + 4, file);
    };
    if os_handle == 0xffff_ffff {
        clear_open(e, handle);
        let error = e.call(IMPORT_GET_LAST_ERROR, &args![]).u32();
        dosmaperr(e, error);
        return errno_value(e);
    }
    let kind = e.call(IMPORT_GET_FILE_TYPE, &args![os_handle]).u32();
    if kind == 0 {
        clear_open(e, handle);
        let error = e.call(IMPORT_GET_LAST_ERROR, &args![]).u32();
        dosmaperr(e, error);
        e.call(IMPORT_CLOSE_HANDLE, &args![os_handle]);
        if error == 0 {
            set_errno(e, 0xd);
        }
        return errno_value(e);
    }
    if kind == 2 {
        osfile |= 0x40;
    } else if kind == 3 {
        osfile |= 8;
    }
    set_osfhnd(e, handle, os_handle);
    osfile |= 1;
    let entry = ioinfo(e, handle);
    e.mem.set_u8(entry + 4, osfile);
    let modes = e.mem.u8(entry + 0x24) & 0x80;
    e.mem.set_u8(entry + 0x24, modes);
    let device_or_pipe = osfile & 0x48;
    let mut step = OpenStep::Done;
    if device_or_pipe == 0 && osfile & 0x80 != 0 && flags & 2 != 0 {
        step = tsopen_cut_ctrl_z(e, handle);
    }
    if matches!(step, OpenStep::Done) && osfile & 0x80 != 0 {
        if flags & 0x74000 == 0 {
            let default = file_mode & 0x74000;
            if default == 0 {
                flags |= 0x4000;
            } else {
                flags |= default;
            }
        }
        match flags & 0x74000 {
            0x4000 => encoding = 0,
            0x10000 | 0x14000 => {
                if flags & 0x301 == 0x301 {
                    encoding = 2;
                }
            }
            0x20000 | 0x24000 => encoding = 2,
            0x40000 | 0x44000 => encoding = 1,
            _ => {}
        }
        if flags & 0x70000 != 0 && osfile & 0x40 == 0 {
            step = tsopen_mark(e, handle, access, creation, &mut encoding);
        }
    }
    match step {
        OpenStep::Done => {}
        OpenStep::Failed => {
            close_nolock(e, handle);
            return errno_value(e);
        }
        OpenStep::Invalid => return 0x16,
    }
    let entry = ioinfo(e, handle);
    let modes = e.mem.u8(entry + 0x24);
    e.mem
        .set_u8(entry + 0x24, modes ^ ((modes ^ encoding) & 0x7f));
    let modes = e.mem.u8(entry + 0x24);
    e.mem
        .set_u8(entry + 0x24, (((flags >> 16) as u8) << 7) | (modes & 0x7f));
    if device_or_pipe == 0 && flags & 8 != 0 {
        let file = e.mem.u8(entry + 4) | 0x20;
        e.mem.set_u8(entry + 4, file);
    }
    if access & 0xc000_0000 == 0xc000_0000 && flags & 1 != 0 {
        e.call(IMPORT_CLOSE_HANDLE, &args![os_handle]);
        let reopened = open(e, access & 0x7fff_ffff, 3);
        if reopened == 0xffff_ffff {
            let error = e.call(IMPORT_GET_LAST_ERROR, &args![]).u32();
            dosmaperr(e, error);
            clear_open(e, handle);
            free_osfhnd(e, handle);
            return errno_value(e);
        }
        let entry = ioinfo(e, handle);
        e.mem.set_u32(entry, reopened);
    }
    0
}

/// A trailing Ctrl-Z (0x1a) cut off a text file opened read-write: seek to the last byte
/// (`_lseek_nolock(-1, SEEK_END)`; a failure other than `ERROR_NEGATIVE_SEEK` (0x83) in
/// `_doserrno` fails), read it, cut the file there when it is 0x1a (`_chsize_nolock` with the
/// sign-extended position), seek back to 0.
fn tsopen_cut_ctrl_z(e: &mut Engine, handle: u32) -> OpenStep {
    let end = lseek_nolock(e, handle, 0xffff_ffff, 2);
    if end == 0xffff_ffff {
        if doserrno_value(e) == 0x83 {
            return OpenStep::Done;
        }
        return OpenStep::Failed;
    }
    let (read, last) = e.with_stack(4, |e, byte| {
        e.mem.set_u8(byte.addr(), 0);
        let read = e.call(READ_NOLOCK, &args![handle, byte, 1u32]).u32();
        (read, e.mem.u8(byte.addr()))
    });
    if read == 0 && last == 0x1a {
        let size = end as i32 as i64 as u64;
        let cut = e
            .call(
                CHSIZE_NOLOCK,
                &args![handle, size as u32, (size >> 32) as u32],
            )
            .u32();
        if cut == 0xffff_ffff {
            return OpenStep::Failed;
        }
    }
    if lseek_nolock(e, handle, 0, 0) == 0xffff_ffff {
        return OpenStep::Failed;
    }
    OpenStep::Done
}

// Translated from 00ee383c (decompiled, FalloutNV.exe 1.4.0.525)
/// `__sopen_helper` (LIBCMT/open.c), cdecl: path, open flags, share flag, permission mode, the
/// address of the result handle, a "secure" flag. A null result pointer, or (it set to -1) a
/// null path, or with the secure flag a mode with bits other than read/write (0x180) gives
/// `EINVAL` with the invalid-parameter handler and answers 0x16. Otherwise `__tsopen_nolock`
/// (the result pointer in `EAX`, an unlock flag on the stack) answers; when the flag was set
/// the handle is unlocked afterwards, its `FOPEN` cleared first when the open failed (the
/// `__finally` block at `00ee38da`), and a failed open leaves -1 in the result.
// Translation incomplete: 00ee383c - the SEH frame (__SEH_prolog4/__SEH_epilog4) is not translated; its __finally block runs inline.
pub fn sopen_helper(
    e: &mut Engine,
    path: Ptr,
    flags: u32,
    share: u32,
    mode: u32,
    handle_out: Ptr,
    secure: u32,
) -> u32 {
    let einval = |e: &mut Engine| -> u32 {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        0x16
    };
    if handle_out.addr() == 0 {
        return einval(e);
    }
    e.mem.set_u32(handle_out.addr(), 0xffff_ffff);
    if path.addr() == 0 {
        return einval(e);
    }
    if secure != 0 && mode & 0xffff_fe7f != 0 {
        return einval(e);
    }
    let (result, unlock) = e.with_stack(4, |e, flag| {
        e.mem.set_u32(flag.addr(), 0);
        let result = tsopen_nolock(e, handle_out, flag, path, flags, share, mode);
        (result, e.mem.u32(flag.addr()))
    });
    if unlock != 0 {
        let handle = e.mem.u32(handle_out.addr());
        if result != 0 {
            let entry = ioinfo(e, handle);
            let file = e.mem.u8(entry + 4) & 0xfe;
            e.mem.set_u8(entry + 4, file);
        }
        unlock_fhandle(e, handle);
    }
    if result != 0 {
        e.mem.set_u32(handle_out.addr(), 0xffff_ffff);
    }
    result
}

// Translated from 00ee3908 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_sopen_s` (LIBCMT/open.c), cdecl: the address of the result handle, path, open flags,
/// share flag, permission mode: `__sopen_helper` with the secure flag set.
pub fn sopen_s(
    e: &mut Engine,
    handle_out: Ptr,
    path: Ptr,
    flags: u32,
    share: u32,
    mode: u32,
) -> u32 {
    sopen_helper(e, path, flags, share, mode, handle_out, 1)
}

/// `_mbsnbcmp` (`00ecb849`) and `_mbsnbicmp` (`00ecba8e`) (LIBCMT, engine map; cdecl: two
/// strings, a count) and the mode strings `"ccs"`, `"UTF-8"`, `"UTF-16LE"`, `"UNICODE"`
/// (`010f4c10` .. `010f4c28`, read in Ghidra).
const MBSNBCMP: u32 = 0x00ec_b849;
const MBSNBICMP: u32 = 0x00ec_ba8e;
const CCS_010F4C10: u32 = 0x010f_4c10;
const UTF8_010F4C14: u32 = 0x010f_4c14;
const UTF16LE_010F4C1C: u32 = 0x010f_4c1c;
const UNICODE_010F4C28: u32 = 0x010f_4c28;

/// The address after the spaces at `at`.
fn skip_spaces(e: &Engine, mut at: u32) -> u32 {
    while e.mem.u8(at) == 0x20 {
        at = at.wrapping_add(1);
    }
    at
}

// Translated from 00ed8238 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_openfile` (LIBCMT/_open.c, engine map), cdecl: path, mode string, share flag, stream.
/// The mode (after spaces) starts with `r` (`_IOREAD`), `w` (`_O_WRONLY|_O_CREAT|_O_TRUNC`)
/// or `a` (`_O_WRONLY|_O_CREAT|_O_APPEND`, both `_IOWRT`); then `+` (read-write, `_IORW`),
/// `b`/`t` (binary/text, once), `c`/`n` (commit or not, once), `S`/`R` (sequential/random,
/// once), `T` (short-lived), `D` (temporary), `N` (no inherit) and spaces; a repeated or
/// conflicting letter ends the letters; a `,` asks for `ccs=UTF-8`/`UTF-16LE`/`UNICODE` (with
/// spaces around `=`); anything else left over gives `EINVAL` with the invalid-parameter
/// handler and a null answer. The stream flags start from `_commode`. The file is opened by
/// `_sopen_s` (mode 0x180; a failure answers null); `_cflush` counts up and the stream gets its
/// flags, a cleared buffer and count, no temporary name and the handle. Answers the stream.
pub fn openfile(e: &mut Engine, path: Ptr, mode: Ptr, share: u32, stream: Ptr) -> Ptr {
    let einval = |e: &mut Engine| -> Ptr {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        Ptr::new(0)
    };
    let mut stream_flags = e.mem.u32(COMMODE);
    let mut commit_set = false;
    let mut scan_set = false;
    let mut ccs = false;
    let mut at = skip_spaces(e, mode.addr());
    let mut open_flags = match e.mem.u8(at) {
        b'a' => {
            stream_flags |= 2;
            0x109u32
        }
        b'r' => {
            stream_flags |= 1;
            0
        }
        b'w' => {
            stream_flags |= 2;
            0x301
        }
        _ => return einval(e),
    };
    let mut valid = true;
    at = at.wrapping_add(1);
    let mut letter = e.mem.u8(at);
    let mut bad = false;
    while letter != 0 && valid {
        match letter {
            b' ' => {}
            b'+' => {
                if open_flags & 2 != 0 {
                    valid = false;
                } else {
                    open_flags = (open_flags & !1) | 2;
                    stream_flags = (stream_flags & !3) | 0x80;
                }
            }
            b',' => {
                ccs = true;
                valid = false;
            }
            b'D' => {
                if open_flags & 0x40 != 0 {
                    valid = false;
                } else {
                    open_flags |= 0x40;
                }
            }
            b'N' => open_flags |= 0x80,
            b'R' => {
                if scan_set {
                    valid = false;
                } else {
                    open_flags |= 0x10;
                    scan_set = true;
                }
            }
            b'S' => {
                if scan_set {
                    valid = false;
                } else {
                    open_flags |= 0x20;
                    scan_set = true;
                }
            }
            b'T' => {
                if open_flags & 0x1000 != 0 {
                    valid = false;
                } else {
                    open_flags |= 0x1000;
                }
            }
            b'b' => {
                if open_flags & 0xc000 != 0 {
                    valid = false;
                } else {
                    open_flags |= 0x8000;
                }
            }
            b'c' => {
                if commit_set {
                    valid = false;
                } else {
                    stream_flags |= 0x4000;
                    commit_set = true;
                }
            }
            b'n' => {
                if commit_set {
                    valid = false;
                } else {
                    stream_flags &= !0x4000;
                    commit_set = true;
                }
            }
            b't' => {
                if open_flags & 0xc000 != 0 {
                    valid = false;
                } else {
                    open_flags |= 0x4000;
                }
            }
            _ => {
                bad = true;
                break;
            }
        }
        at = at.wrapping_add(1);
        letter = e.mem.u8(at);
    }
    if !bad && ccs {
        at = skip_spaces(e, at);
        bad = true;
        if e.call(MBSNBCMP, &args![CCS_010F4C10, at, 3u32]).u32() == 0 {
            at = skip_spaces(e, at.wrapping_add(3));
            if e.mem.u8(at) == b'=' {
                at = skip_spaces(e, at.wrapping_add(1));
                if e.call(MBSNBICMP, &args![at, UTF8_010F4C14, 5u32]).u32() == 0 {
                    at = at.wrapping_add(5);
                    open_flags |= 0x40000;
                    bad = false;
                } else if e.call(MBSNBICMP, &args![at, UTF16LE_010F4C1C, 8u32]).u32() == 0 {
                    at = at.wrapping_add(8);
                    open_flags |= 0x20000;
                    bad = false;
                } else if e.call(MBSNBICMP, &args![at, UNICODE_010F4C28, 7u32]).u32() == 0 {
                    at = at.wrapping_add(7);
                    open_flags |= 0x10000;
                    bad = false;
                }
            }
        }
    }
    if !bad {
        at = skip_spaces(e, at);
        bad = e.mem.u8(at) != 0;
    }
    if bad {
        return einval(e);
    }
    let (failed, handle) = e.with_stack(4, |e, handle| {
        e.mem.set_u32(handle.addr(), 0);
        let failed = sopen_s(e, handle, path, open_flags, share, 0x180);
        (failed, e.mem.u32(handle.addr()))
    });
    if failed != 0 {
        return Ptr::new(0);
    }
    let flushes = e.mem.u32(CFLUSH).wrapping_add(1);
    e.mem.set_u32(CFLUSH, flushes);
    let stream = stream.addr();
    e.mem.set_u32(stream + 0xc, stream_flags);
    e.mem.set_u32(stream + 4, 0);
    e.mem.set_u32(stream, 0);
    e.mem.set_u32(stream + 8, 0);
    e.mem.set_u32(stream + 0x1c, 0);
    e.mem.set_u32(stream + 0x10, handle);
    Ptr::new(stream)
}

/// `__piob` (`01270a70`, the streams' pointer array) and `_nstream` (`01271a80`).
const PIOB: u32 = 0x0127_0a70;
const NSTREAM: u32 = 0x0127_1a80;

// Translated from 00ece05c (decompiled, FalloutNV.exe 1.4.0.525)
/// `_lock_file2` (LIBCMT/_file.c, engine map), cdecl: the stream's index and the stream. The
/// first 20 streams take `_lock` 0x10 + index and get `_IOLOCKED`; others enter the critical
/// section at their +0x20.
pub fn lock_file2(e: &mut Engine, index: i32, stream: Ptr) {
    if index < 0x14 {
        lock(e, (index as u32).wrapping_add(0x10));
        let flags = e.mem.u32(stream.addr() + 0xc) | 0x8000;
        e.mem.set_u32(stream.addr() + 0xc, flags);
    } else {
        e.call(
            IMPORT_ENTER_CRITICAL_SECTION,
            &args![stream.addr().wrapping_add(0x20)],
        );
    }
}

// Translated from 00ece0ca (decompiled, FalloutNV.exe 1.4.0.525)
/// `_unlock_file2` (LIBCMT/_file.c, engine map), cdecl: the stream's index and the stream: the
/// first 20 lose `_IOLOCKED` and leave `_lock` 0x10 + index; others leave the critical section
/// at their +0x20.
pub fn unlock_file2(e: &mut Engine, index: i32, stream: Ptr) {
    if index < 0x14 {
        let flags = e.mem.u32(stream.addr() + 0xc) & 0xffff_7fff;
        e.mem.set_u32(stream.addr() + 0xc, flags);
        unlock(e, (index as u32).wrapping_add(0x10));
    } else {
        e.call(
            IMPORT_LEAVE_CRITICAL_SECTION,
            &args![stream.addr().wrapping_add(0x20)],
        );
    }
}

// Translated from 00ed8501 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_getstream` (LIBCMT/stream.c, engine map), cdecl, no arguments: a free stream, locked.
/// Under lock 1 the `__piob` slots up to `_nstream` are searched: a missing stream is
/// allocated (0x38 bytes, `malloc`), its critical section at +0x20 set up
/// (`__crtInitCritSecAndSpinCount` 4000; a failure frees it and ends the search with none),
/// entered, its flags cleared and taken; an existing one not in use (no `_IOREAD|_IOWRT|_IORW`,
/// not `_IOLOCKED`) is locked (`_lock_file2`, after `__mtinitlocknum` of its lock for slots 3
/// to 19; a failure ends the search) and taken when still not in use, else unlocked. The stream
/// taken keeps only `_IOLOCKED` and gets a cleared count, base, pointer and temporary name and
/// the handle -1. Lock 1 is left (the `__finally` block at `00ed8630`).
// Translation incomplete: 00ed8501 - the SEH frame (__SEH_prolog4/__SEH_epilog4) is not translated; its __finally block runs inline.
pub fn getstream(e: &mut Engine) -> Ptr {
    let mut found = 0u32;
    lock(e, 1);
    let mut index = 0i32;
    while index < e.mem.u32(NSTREAM) as i32 {
        let slot = e.mem.u32(PIOB).wrapping_add((index as u32) * 4);
        let stream = e.mem.u32(slot);
        if stream == 0 {
            let offset = (index as u32) << 2;
            let block = malloc(e, 0x38).addr();
            let piob = e.mem.u32(PIOB);
            e.mem.set_u32(piob.wrapping_add(offset), block);
            let piob = e.mem.u32(PIOB);
            let stream = e.mem.u32(piob.wrapping_add(offset));
            if stream != 0 {
                if crt_init_crit_sec_and_spin_count(e, stream + 0x20, 0xfa0) == 0 {
                    let piob = e.mem.u32(PIOB);
                    let stream = e.mem.u32(piob.wrapping_add(offset));
                    free(e, Ptr::new(stream));
                    let piob = e.mem.u32(PIOB);
                    e.mem.set_u32(piob.wrapping_add(offset), 0);
                } else {
                    let piob = e.mem.u32(PIOB);
                    let stream = e.mem.u32(piob.wrapping_add(offset));
                    e.call(IMPORT_ENTER_CRITICAL_SECTION, &args![stream + 0x20]);
                    let piob = e.mem.u32(PIOB);
                    found = e.mem.u32(piob.wrapping_add(offset));
                    e.mem.set_u32(found + 0xc, 0);
                }
            }
            break;
        }
        let flags = e.mem.u32(stream + 0xc);
        if flags & 0x83 == 0 && flags & 0x8000 == 0 {
            if (index as u32).wrapping_sub(3) <= 0x10
                && mtinitlocknum(e, (index as u32).wrapping_add(0x10)) == 0
            {
                break;
            }
            let stream = e.mem.u32(e.mem.u32(PIOB).wrapping_add((index as u32) * 4));
            lock_file2(e, index, Ptr::new(stream));
            let stream = e.mem.u32(e.mem.u32(PIOB).wrapping_add((index as u32) * 4));
            if e.mem.u8(stream + 0xc) & 0x83 == 0 {
                found = stream;
                break;
            }
            unlock_file2(e, index, Ptr::new(stream));
        }
        index += 1;
    }
    if found != 0 {
        let flags = e.mem.u32(found + 0xc) & 0x8000;
        e.mem.set_u32(found + 0xc, flags);
        e.mem.set_u32(found + 4, 0);
        e.mem.set_u32(found + 8, 0);
        e.mem.set_u32(found, 0);
        e.mem.set_u32(found + 0x1c, 0);
        e.mem.set_u32(found + 0x10, 0xffff_ffff);
    }
    unlock(e, 1);
    Ptr::new(found)
}

// Translated from 00ed806c (decompiled, FalloutNV.exe 1.4.0.525)
/// `_fileno` (LIBCMT/fileno.c, engine map), cdecl: the stream's handle (+0x10); a null stream
/// gives `EINVAL` with the invalid-parameter handler and -1.
pub fn fileno(e: &mut Engine, stream: Ptr) -> u32 {
    if stream.addr() == 0 {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return 0xffff_ffff;
    }
    e.mem.u32(stream.addr() + 0x10)
}

// Translated from 00ed8207 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_freebuf` (LIBCMT/_freebuf.c, engine map), cdecl: a stream in use with its own buffer
/// (`_IOMYBUF` 8) has the buffer freed, `_IOMYBUF` and `_IOSETVBUF` (0x400) cleared and its
/// pointer, base and count cleared.
pub fn freebuf(e: &mut Engine, stream: Ptr) {
    let stream = stream.addr();
    let flags = e.mem.u32(stream + 0xc);
    if flags & 0x83 != 0 && flags & 8 != 0 {
        let buffer = e.mem.u32(stream + 8);
        free(e, Ptr::new(buffer));
        let flags = e.mem.u32(stream + 0xc) & 0xffff_fbf7;
        e.mem.set_u32(stream + 0xc, flags);
        e.mem.set_u32(stream, 0);
        e.mem.set_u32(stream + 8, 0);
        e.mem.set_u32(stream + 4, 0);
    }
}

// Translated from 00ecd8e6 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_flush` (LIBCMT/fflush.c, engine map), cdecl: a stream being written (`_IOWRT` in the low
/// two bits) with a buffer (`_IOMYBUF|_IOYOURBUF`) writes what it holds (`_write` of
/// `_fileno`): a short write sets `_IOERR` and answers -1; a complete one clears `_IOWRT` for a
/// read-write stream. The pointer goes back to the base and the count to 0. Answers 0 or -1.
pub fn flush(e: &mut Engine, stream: Ptr) -> i32 {
    let stream = stream.addr();
    let flags = e.mem.u32(stream + 0xc);
    let mut result = 0;
    if flags & 3 == 2 && flags & 0x108 != 0 {
        let base = e.mem.u32(stream + 8);
        let held = e.mem.u32(stream).wrapping_sub(base);
        if (held as i32) > 0 {
            let handle = fileno(e, Ptr::new(stream));
            let written = write(e, handle, Ptr::new(base), held);
            if written as u32 == held {
                let flags = e.mem.u32(stream + 0xc);
                if flags & 0x80 != 0 {
                    e.mem.set_u32(stream + 0xc, flags & 0xffff_fffd);
                }
            } else {
                let flags = e.mem.u32(stream + 0xc) | 0x20;
                e.mem.set_u32(stream + 0xc, flags);
                result = -1;
            }
        }
    }
    let base = e.mem.u32(stream + 8);
    e.mem.set_u32(stream + 4, 0);
    e.mem.set_u32(stream, base);
    result
}

// Translated from 00edecf7 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_getbuf` (LIBCMT/_getbuf.c, engine map), cdecl: gives a stream a buffer: `_cflush` counts
/// up; 0x1000 bytes from `malloc` (`_IOMYBUF`, size 0x1000), or when that fails the stream's
/// own two bytes at +0x14 (`_IONBF` 4, size 2). The pointer takes the base, the count 0.
pub fn getbuf(e: &mut Engine, stream: Ptr) {
    let stream = stream.addr();
    let flushes = e.mem.u32(CFLUSH).wrapping_add(1);
    e.mem.set_u32(CFLUSH, flushes);
    let buffer = malloc(e, 0x1000).addr();
    e.mem.set_u32(stream + 8, buffer);
    if buffer != 0 {
        let flags = e.mem.u32(stream + 0xc) | 8;
        e.mem.set_u32(stream + 0xc, flags);
        e.mem.set_u32(stream + 0x18, 0x1000);
    } else {
        let flags = e.mem.u32(stream + 0xc) | 4;
        e.mem.set_u32(stream + 0xc, flags);
        e.mem.set_u32(stream + 8, stream + 0x14);
        e.mem.set_u32(stream + 0x18, 2);
    }
    let base = e.mem.u32(stream + 8);
    e.mem.set_u32(stream + 4, 0);
    e.mem.set_u32(stream, base);
}

// Translated from 00ed2d64 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_flsbuf` (LIBCMT/_flsbuf.c), cdecl: a character and the stream; writes the character
/// through the stream's buffer. A stream not open for writing (no `_IOWRT|_IORW`) gives
/// `EBADF`, a string stream (`_IOSTRG`) `ERANGE`, both with `_IOERR` and -1. A read stream not
/// at its end (`_IOEOF` clear) also errs; one at its end turns to writing. The stream becomes
/// `_IOWRT`; without a buffer one is made (`_getbuf`) unless it is `stdout`/`stderr` on a
/// device (`_isatty`). With a buffer the pending bytes are written (`_write`) and the character
/// stored at its start; with an empty buffer an appending file (`FAPPEND` of its `ioinfo`,
/// `__badioinfo` for -1/-2) is first moved to its end (`_lseeki64`); without one the character
/// is written alone. A short write sets `_IOERR` and answers -1; else the character (as a byte).
pub fn flsbuf(e: &mut Engine, character: u32, stream: Ptr) -> i32 {
    let stream = stream.addr();
    let handle = fileno(e, Ptr::new(stream));
    let flags = e.mem.u32(stream + 0xc);
    let fail = |e: &mut Engine| -> i32 {
        let flags = e.mem.u32(stream + 0xc) | 0x20;
        e.mem.set_u32(stream + 0xc, flags);
        -1
    };
    if flags & 0x82 == 0 {
        set_errno(e, 9);
        return fail(e);
    }
    if flags & 0x40 != 0 {
        set_errno(e, 0x22);
        return fail(e);
    }
    if flags & 1 != 0 {
        e.mem.set_u32(stream + 4, 0);
        if flags & 0x10 == 0 {
            e.mem.set_u32(stream + 0xc, flags | 0x20);
            return -1;
        }
        let base = e.mem.u32(stream + 8);
        e.mem.set_u32(stream, base);
        e.mem.set_u32(stream + 0xc, flags & 0xffff_fffe);
    }
    let flags = (e.mem.u32(stream + 0xc) & 0xffff_ffef) | 2;
    e.mem.set_u32(stream + 0xc, flags);
    e.mem.set_u32(stream + 4, 0);
    let mut written = 0i32;
    if flags & 0x10c == 0 {
        let iob = iob_func(e).addr();
        let standard = stream == iob.wrapping_add(0x20) || stream == iob.wrapping_add(0x40);
        if !standard || isatty(e, handle) == 0 {
            getbuf(e, Ptr::new(stream));
        }
    }
    let expected: i32;
    if e.mem.u32(stream + 0xc) & 0x108 != 0 {
        let base = e.mem.u32(stream + 8);
        let held = e.mem.u32(stream).wrapping_sub(base) as i32;
        e.mem.set_u32(stream, base.wrapping_add(1));
        let room = e.mem.u32(stream + 0x18).wrapping_sub(1);
        e.mem.set_u32(stream + 4, room);
        expected = held;
        if held > 0 {
            written = write(e, handle, Ptr::new(base), held as u32);
        } else {
            let entry = if handle == 0xffff_ffff || handle == 0xffff_fffe {
                BAD_IOINFO
            } else {
                ioinfo(e, handle)
            };
            if e.mem.u8(entry + 4) & 0x20 != 0 && lseeki64(e, handle, 0, 0, 2) == u64::MAX {
                return fail(e);
            }
        }
        let base = e.mem.u32(stream + 8);
        e.mem.set_u8(base, character as u8);
    } else {
        expected = 1;
        written = e.with_stack(4, |e, byte| {
            e.mem.set_u8(byte.addr(), character as u8);
            write(e, handle, byte, 1)
        });
    }
    if written != expected {
        return fail(e);
    }
    (character & 0xff) as i32
}

/// `GetFileAttributesA` (KERNEL32 import slot `00fdf160`).
const API_GET_FILE_ATTRIBUTES: u32 = 0x00fd_f160;
/// `GetLastError` (KERNEL32 import slot `00fdf094`).
const API_GET_LAST_ERROR_SLOT: u32 = 0x00fd_f094;

// Translated from 00ecbe7c (disassembled, FalloutNV.exe 1.4.0.525)
/// `_access_s` (LIBCMT/access.c, engine map), cdecl: (path, mode). A null path or a mode with
/// bits other than 2 and 4 sets `_doserrno` 0 and `errno` `EINVAL` (0x16), calls the
/// invalid-parameter handler (`00ec7c30` with five zeros) and answers 0x16. Otherwise the
/// path's attributes (`GetFileAttributesA`): none maps the Windows error (`_dosmaperr` of
/// `GetLastError`) and answers `errno`; a read-only file (attribute 1, not a directory 0x10)
/// asked for writing (mode bit 2) sets `_doserrno` 5 and `errno` `EACCES` (0xd) and answers
/// it; anything else answers 0.
pub fn access_s(e: &mut Engine, path: Ptr, mode: u32) -> u32 {
    if path.addr() == 0 || mode & 0xffff_fff9 != 0 {
        let dos = e.call(0x00ec_85f6, &args![]).u32();
        e.mem.set_u32(dos, 0);
        let error = e.call(0x00ec_85e3, &args![]).u32();
        e.mem.set_u32(error, 0x16);
        e.call(0x00ec_7c30, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        return 0x16;
    }
    let attributes = e.call(API_GET_FILE_ATTRIBUTES, &args![path]).u32();
    if attributes == 0xffff_ffff {
        let code = e.call(API_GET_LAST_ERROR_SLOT, &args![]).u32();
        e.call(0x00ec_8609, &args![code]);
        let error = e.call(0x00ec_85e3, &args![]).u32();
        return e.mem.u32(error);
    }
    if attributes & 0x10 == 0 && attributes & 1 != 0 && mode & 2 != 0 {
        let dos = e.call(0x00ec_85f6, &args![]).u32();
        e.mem.set_u32(dos, 5);
        let error = e.call(0x00ec_85e3, &args![]).u32();
        e.mem.set_u32(error, 0xd);
        let error = e.call(0x00ec_85e3, &args![]).u32();
        return e.mem.u32(error);
    }
    0
}

// Translated from 00ecbf05 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_access` (LIBCMT/access.c, engine map), cdecl: (path, mode): 0 when `_access_s` answers 0,
/// else -1 (`NEG`/`SBB`).
pub fn access(e: &mut Engine, path: Ptr, mode: u32) -> i32 {
    let r = e.call(0x00ec_be7c, &args![path, mode]).u32();
    if r == 0 {
        0
    } else {
        -1
    }
}

// Translated from 00ec6380 (disassembled, FalloutNV.exe 1.4.0.525)
/// `strcat` (LIBCMT, the word-at-a-time assembly version; the engine map gives no name), cdecl:
/// (destination, source): appends the source's bytes up to and with its terminating zero at
/// the destination's terminating zero and answers the destination. The assembly reads both
/// strings a word at a time (aligned reads that stop at the word holding the zero, never past
/// it into another page); the bytes it writes are the same.
pub fn strcat(e: &mut Engine, destination: Ptr, source: Ptr) -> Ptr {
    let mut at = destination.addr();
    while e.mem.u8(at) != 0 {
        at = at.wrapping_add(1);
    }
    let mut from = source.addr();
    loop {
        let b = e.mem.u8(from);
        e.mem.set_u8(at, b);
        if b == 0 {
            return destination;
        }
        at = at.wrapping_add(1);
        from = from.wrapping_add(1);
    }
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    let mut table = vec![
        entry!(0x00ec_dd16, gcvt(f64, i32, Ptr) -> Ptr),
        entry!(
            0x00ec_5fce,
            eh_vector_destructor_iterator(Ptr, u32, i32, u32)
        ),
        entry!(0x00ec_be7c, access_s(Ptr, u32) -> u32),
        entry!(0x00ec_bf05, access(Ptr, u32) -> i32),
        entry!(0x00ec_6380, strcat(Ptr, Ptr) -> Ptr),
        entry!(0x00401000, operator_new(u32) -> Ptr),
        entry!(0x00401460, memcpy_wrapper(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00403d30, memset_wrapper(Ptr, i32, u32) -> Ptr),
        entry!(0x00ec3bf0, byteswap_uint64(u64) -> u64),
        entry!(0x00ed0cdf, malloc(u32) -> Ptr),
        entry!(0x00ecd291, free(Ptr)),
        entry!(0x00ec_cf5d, realloc(Ptr, u32) -> Ptr),
        entry!(0x00ec_d31f, msize(Ptr) -> u32),
        entry!(0x00ed0d24, calloc_crt(u32, u32) -> Ptr),
        entry!(0x00ec8d08, strtol(u32, u32, i32) -> i32),
        entry!(0x00ec8ad9, strtoxl(u32, u32, u32, i32, u32) -> u32),
        entry!(0x00ed1165, getptd_noexit() -> u32),
        entry!(0x00ed11de, getptd() -> u32),
        entry!(0x00ecf742, update_tlocinfo() -> u32),
        entry!(0x00ed3cdf, update_tmbcinfo() -> u32),
        entry!(0x00ec660e, locale_update(u32, u32) -> u32),
        entry!(0x00ed419c, isleadbyte_l(u32, u32) -> u32),
        entry!(0x00ee13a3, crt_get_string_type_a(u32, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ed41e7, isctype_l(i32, u32, u32) -> u32),
        entry!(0x00eccc25, crt_waiting_on_module_handle(u32) -> u32),
        entry!(0x00ed0f17, encode_pointer(u32) -> u32),
        entry!(0x00ed0f92, decode_pointer(u32) -> u32),
        entry!(0x00ed0f89, encoded_null() -> u32),
        entry!(0x00ed100d, set_flsgetvalue() -> u32),
        entry!(0x00ed1004, crt_tls_alloc(u32) -> u32),
        entry!(0x00ede1fc, fn_00ede1fc(u32)),
        entry!(0x00ede18d, fn_00ede18d(u32)),
        entry!(0x00ed2c6f, fn_00ed2c6f(u32)),
        entry!(0x00ede17e, fn_00ede17e(u32)),
        entry!(0x00eddf6c, initp_misc_winsig(u32)),
        entry!(0x00ec5eec, fn_00ec5eec(u32)),
        entry!(0x00ed1538, initp_eh_hooks(u32)),
        entry!(0x00eccf0f, init_pointers()),
        entry!(0x00ede19c, crt_init_crit_sec_and_spin_count(u32, u32) -> u32),
        entry!(0x00edddbd, mtinitlocks() -> u32),
        entry!(0x00eddf30, fn_00eddf30()),
        entry!(0x00edde76, mtinitlocknum(u32) -> u32),
        entry!(0x00eddf39, lock(u32)),
        entry!(0x00edde5f, unlock(u32)),
        entry!(0x00eccc55, amsg_exit(u32)),
        entry!(0x00ed1153, fn_00ed1153()),
        entry!(0x00ed115c, fn_00ed115c()),
        entry!(0x00ed107e, initptd(u32, u32)),
        entry!(0x00ed1041, mtterm()),
        entry!(0x00ed1327, mtinit() -> u32),
        entry!(0x00ecf704, update_tlocinfo_ex_nolock(u32, u32) -> u32),
        entry!(0x00ee11e9, crt_get_string_type_a_stat(u32, u32, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00edde08, mtdeletelocks()),
        entry!(0x00ed2ab7, ff_msgbanner()),
        entry!(0x00ed290c, nmsg_write(u32)),
        entry!(0x00eccedb, exit_quick(u32)),
        entry!(0x00ecf5b6, addlocaleref(u32)),
        entry!(0x00ecf645, removelocaleref(u32) -> u32),
        entry!(0x00ecf46d, freetlocinfo(u32)),
        entry!(0x00ee13e5, ansicp(u32) -> u32),
        entry!(0x00ee142e, convertcp(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ecd1c7, malloc_body(u32) -> u32),
        entry!(0x00ec8e14, freea(u32)),
        entry!(0x00ed11f8, freefls(u32)),
        entry!(
            0x00ec782f,
            eh_vector_constructor_iterator(u32, u32, i32, u32, u32)
        ),
        entry!(0x00ed5abb, ctrlfp(u32, u32) -> i32),
        entry!(0x00ec8660, alloca_probe(u32, u32) -> u32),
        entry!(0x00ecadda, time64(Ptr) -> u64),
        entry!(0x00ec658f, atexit(Ptr) -> i32),
        entry!(0x00ec_6468, onexit_nolock(u32) -> u32),
        (0x00eccd99, doexit_entry as AbiFn),
        entry!(0x00ec9e10, ceil(f64) -> f64),
        entry!(0x00ec605d, fn_00ec605d(bool, f64, u32, u32) -> f64),
        entry!(0x00ec85e3, errno() -> Ptr),
        entry!(0x00ec9650, ci_asin(f64) -> f64),
        entry!(0x00ed159c, report_gsfailure()),
        entry!(0x00ecddf1, vsnprintf(Ptr, u32, Ptr, u32) -> i32),
        entry!(0x00ec7c30, fn_00ec7c30(u32, u32, u32, u32, u32)),
        entry!(0x00ecc590, aulldiv(u32, u32, u32, u32) -> u64),
        entry!(0x00eccca9, crt_exit_process(i32)),
        entry!(0x00ecc7b0, strncpy(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00ed2ec8, write_char(u32, Ptr, Ptr) -> u32),
        entry!(0x00ed4638, write_string(Ptr, Ptr, Ptr, i32)),
        entry!(0x00ecdf44, iob_func() -> Ptr),
        (0x00ed27c5, fload_with_fb_entry as AbiFn),
        entry!(0x00ec_9a47, fopen(Ptr, Ptr) -> Ptr),
        entry!(0x00ec_9907, fclose(Ptr) -> i32),
        entry!(0x00ec_b086, fwrite(Ptr, u32, u32, Ptr) -> u32),
        entry!(0x00ec_e01b, lock_file(Ptr)),
        entry!(0x00ec_e08e, unlock_file(Ptr)),
        entry!(0x00ec_9890, fclose_nolock(Ptr) -> i32),
        entry!(0x00ec_9983, fsopen(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00ec_af24, fwrite_nolock(Ptr, u32, u32, Ptr) -> u32),
        entry!(0x00ec_85f6, doserrno() -> Ptr),
        entry!(0x00ec_85a1, get_errno_from_oserr(u32) -> u32),
        entry!(0x00ec_8609, dosmaperr(u32)),
        entry!(0x00ee_2e45, get_osfhandle(u32) -> u32),
        entry!(0x00ee_2d3e, set_osfhnd(u32, u32) -> i32),
        entry!(0x00ee_2dbf, free_osfhnd(u32) -> i32),
        entry!(0x00ee_2ebc, lock_fhandle(u32) -> u32),
        entry!(0x00ee_2f5c, unlock_fhandle(u32)),
        entry!(0x00ee_2f83, alloc_osfhnd() -> u32),
        entry!(0x00ed_d009, lseek_nolock(u32, u32, u32) -> u32),
        entry!(0x00ee_1fa7, lseeki64_nolock(u32, u32, u32, u32) -> u64),
        entry!(0x00ee_202c, lseeki64(u32, u32, u32, u32) -> u64),
        entry!(0x00ee_2145, isatty(u32) -> u32),
        entry!(0x00ed_809e, close_nolock(u32) -> i32),
        entry!(0x00ed_813a, close(u32) -> i32),
        entry!(0x00ed_c743, write(u32, Ptr, u32) -> i32),
        entry!(0x00ee_311d, tsopen_nolock(Ptr, Ptr, Ptr, u32, u32, u32) -> u32),
        entry!(0x00ee_383c, sopen_helper(Ptr, u32, u32, u32, Ptr, u32) -> u32),
        entry!(0x00ee_3908, sopen_s(Ptr, Ptr, u32, u32, u32) -> u32),
        entry!(0x00ed_8238, openfile(Ptr, Ptr, u32, Ptr) -> Ptr),
        entry!(0x00ec_e05c, lock_file2(i32, Ptr)),
        entry!(0x00ec_e0ca, unlock_file2(i32, Ptr)),
        entry!(0x00ed_8501, getstream() -> Ptr),
        entry!(0x00ed_806c, fileno(Ptr) -> u32),
        entry!(0x00ed_8207, freebuf(Ptr)),
        entry!(0x00ec_d8e6, flush(Ptr) -> i32),
        entry!(0x00ed_ecf7, getbuf(Ptr)),
        entry!(0x00ed_2d64, flsbuf(u32, Ptr) -> i32),
        (0x00ec_9774, fprintf_entry as AbiFn),
        entry!(0x00ed_7d48, stbuf(Ptr) -> u32),
        entry!(0x00ed_7de4, ftbuf(u32, Ptr)),
        entry!(0x00ed_2efb, write_multi_char(u32, i32, Ptr, Ptr)),
        (0x00ec_c4f0, aulldvrm_entry as AbiFn),
        entry!(0x00ee_2315, wctomb_s(Ptr, Ptr, u32, u32) -> i32),
        entry!(0x00ec_9df9, get_printf_count_output() -> u32),
        (0x00ed_2f21, output_l_entry as AbiFn),
    ];
    table.extend(cvt::funcs());
    table.extend(cxxframe::funcs());
    table.extend(fdiv::funcs());
    table.extend(sbh::funcs());
    table.extend(seh::funcs());
    table.extend(files::funcs());
    table.extend(fpinit::funcs());
    table.extend(glue::funcs());
    table.extend(lcmap::funcs());
    table.extend(locale2::funcs());
    table.extend(lowio::funcs());
    table.extend(math::funcs());
    table.extend(mbs::funcs());
    table.extend(mem::funcs());
    table.extend(misc2::funcs());
    table.extend(misc3::funcs());
    table.extend(misc4::funcs());
    table.extend(misc5::funcs());
    table.extend(misc6::funcs());
    table.extend(misc7::funcs());
    table.extend(misc8::funcs());
    table.extend(misc9::funcs());
    table.extend(locale3::funcs());
    table.extend(mbstowcs::funcs());
    table.extend(io2::funcs());
    table.extend(num2::funcs());
    table.extend(printf::funcs());
    table.extend(qualified::funcs());
    table.extend(rtti::funcs());
    table.extend(runtime::funcs());
    table.extend(scanf::funcs());
    table.extend(search::funcs());
    table.extend(sort::funcs());
    table.extend(stdio::funcs());
    table.extend(str2::funcs());
    table.extend(string::funcs());
    table.extend(startup::funcs());
    table.extend(stat::funcs());
    table.extend(time::funcs());
    table.extend(trig::funcs());
    table.extend(wide::funcs());
    table
}

/// Test support for the constructors of the forms (`CreateFormOfType`'s
/// table): a constructor is a run of calls into the component and base
/// constructors and stores of vtable pointers and zeros. [`run`] builds
/// the object, doubles every callee, runs the constructor by its address
/// and compares the calls (with the object's address substituted for
/// [`A::T`]) and the stores; every other byte of the object must be left
/// as it was. Other engine crates' tests reach it through the core's
/// `test-support` feature.
#[cfg(any(test, feature = "test-support"))]
pub mod ctor_test {
    use crate::prelude::*;

    /// An expected argument word: the object's address plus an offset, or
    /// a plain value.
    #[derive(Clone, Copy, Debug)]
    pub enum A {
        T(u32),
        V(u32),
    }

    pub struct Case<'a> {
        pub addr: u32,
        pub size: u32,
        /// (address, value) of the globals the constructor reads.
        pub globals: &'a [(u32, u32)],
        /// (address, value) a callee answers; other callees answer 0.
        pub returns: &'a [(u32, u32)],
        pub calls: &'a [(u32, &'a [A])],
        /// (offset, width in bytes, value).
        pub stores: &'a [(u32, u32, u32)],
    }

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    pub fn run(c: &Case) {
        let mut e = Engine::new();
        let this = e.mem.alloc(c.size);
        e.mem.write(this, &vec![0xcd; c.size as usize]);
        for (a, v) in c.globals {
            e.mem.map(*a, 4);
            e.mem.set_u32(*a, *v);
        }
        for (a, _) in c.calls {
            e.register(*a, noop);
        }
        for (a, v) in c.returns {
            let v = *v;
            e.register_double(*a, move |_, _| Ret {
                eax: v,
                ..Ret::default()
            });
        }
        e.call_log = Some(vec![]);
        let ret = e.call(c.addr, &args![Ptr::<()>::new(this)]).u32();
        assert_eq!(ret, this, "{:08x} returns this", c.addr);
        let log = e.call_log.take().unwrap();
        let seen: Vec<(u32, Vec<u32>)> = log.into_iter().skip(1).collect();
        let want: Vec<(u32, Vec<u32>)> = c
            .calls
            .iter()
            .map(|(a, args)| {
                (
                    *a,
                    args.iter()
                        .map(|x| match x {
                            A::T(o) => this + o,
                            A::V(v) => *v,
                        })
                        .collect(),
                )
            })
            .collect();
        assert_eq!(seen.len(), want.len(), "{:08x}: calls {:x?}", c.addr, seen);
        for (s, w) in seen.iter().zip(&want) {
            assert_eq!(s.0, w.0, "{:08x}: callee order", c.addr);
            assert_eq!(
                s.1[..w.1.len().min(s.1.len())],
                w.1[..],
                "{:08x}: arguments of {:08x}",
                c.addr,
                w.0
            );
        }
        let mut expected = vec![0xcd_u8; c.size as usize];
        for (off, width, v) in c.stores {
            let bytes = v.to_le_bytes();
            expected[*off as usize..(*off + *width) as usize]
                .copy_from_slice(&bytes[..*width as usize]);
        }
        assert_eq!(
            e.mem.bytes(this, c.size),
            expected,
            "{:08x}: object bytes",
            c.addr
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cstr(e: &mut Engine, s: &[u8]) -> Ptr {
        let p = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(p, s);
        Ptr::new(p)
    }

    #[test]
    fn the_vector_constructor_iterator_constructs_each_element_in_order() {
        let mut e = Engine::new();
        e.register(0x0090_0000, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ec782f, &[0x1000, 0xc, 3, 0x0090_0000, 0x0090_1000]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log[1..].iter().map(|(a, w)| (*a, w.clone())).collect();
        assert_eq!(
            calls,
            [
                (0x0090_0000, vec![0x1000]),
                (0x0090_0000, vec![0x100c]),
                (0x0090_0000, vec![0x1018]),
            ]
        );
    }

    #[test]
    fn byteswap_uint64_reverses_the_bytes() {
        let mut e = Engine::new();
        let r = e.call(0x00ec3bf0, &[0x0506_0708, 0x0102_0304]);
        assert_eq!((r.eax, r.edx), (0x0403_0201, 0x0807_0605));
    }

    #[test]
    fn byteswap_ulong_reverses_the_bytes() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ec3bad, &[0x1234_5678]).u32(), 0x7856_3412);
        assert_eq!(e.call(0x00ec3bad, &[0xff00_0001]).u32(), 0x0100_00ff);
    }

    #[test]
    fn strings() {
        let mut e = Engine::new();
        let a = cstr(&mut e, b"Goodsprings");
        let b = cstr(&mut e, b"GOODSPRINGS");
        assert_eq!(e.call(0x00ec_6130, &args![a]).u32(), 11);
        assert_eq!(e.call(0x00ec_68e4, &args![a, b]).i32(), 0);
        assert_eq!(e.call(0x00ec_8a19, &args![a, b, 1u32]).i32(), 0);
        assert!(e.call(0x00ec_8a19, &args![a, b, 2u32]).i32() > 0);
        assert_eq!(e.call(0x00ec_7ec0, &args![a, b, 11u32]).i32(), 0);
        let o = e.call(0x00ec_7690, &args![a, b'p' as i32]).u32();
        assert_eq!(o, a.addr() + 5);
        let buf: Ptr = Ptr::new(e.mem.alloc(8));
        assert_eq!(
            e.call(0x00ec_8d5f, &args![buf, 8u32, a, TRUNCATE]).i32(),
            80
        );
        assert_eq!(e.mem.cstr(buf.addr()), b"Goodspr");
    }

    #[test]
    fn numbers() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ec_62c0, &args![-2.75f64]).u64() as i64, -2);
        assert_eq!(e.call(0x00ec_62c0, &args![f64::NAN]).u64(), 1 << 63);
        // `atol` and `atof` hand their text to `strtol(text, NULL, 10)` (`00ec8d08`) and
        // `_atof_l(text, NULL)` (`00eca4c8`), which read the locale data of the exe image:
        // doubled here to answer -42 and 150.
        e.register(0x00ec_8d08, |_, a| {
            assert_eq!(&a[1..], &[0, 10]);
            (-42i32).into_ret()
        });
        e.register(0x00ec_a4c8, |_, a| {
            assert_eq!(a[1], 0);
            150.0f64.into_ret()
        });
        let s = cstr(&mut e, b"  -42xyz");
        assert_eq!(e.call(0x00ec_a6d3, &args![s]).i32(), -42);
        let f = cstr(&mut e, b"1.5e2 rest");
        assert_eq!(e.call(0x00ec_a573, &args![f]).f64(), 150.0);
        // MSVC rand from the default seed: 41, 18467, 6334. The thread data is a block with
        // the seed `_initptd` stores (+0x14 = 1), answered by `_getptd` (`00ed11de`).
        let thread = e.mem.alloc(0x400);
        e.mem.set_u32(thread + 0x14, 1);
        e.register_double(0x00ed_11de, move |_, _| thread.into_ret());
        let r: Vec<i32> = (0..3).map(|_| e.call(0x00ec_adb8, &[]).i32()).collect();
        assert_eq!(r, vec![41, 18467, 6334]);
    }

    #[test]
    fn memory_and_operator_new() {
        let mut e = Engine::new();
        let p: Ptr = e.call_as(0x0040_1000, &args![12u32]);
        e.call(0x0040_3d30, &args![p, 0xab_i32, 12u32]);
        assert_eq!(e.mem.u32(p.addr() + 8), 0xabab_abab);
        let q: Ptr = e.call_as(0x0040_1000, &args![12u32]);
        e.call(0x0040_1460, &args![q, p, 12u32]);
        assert_eq!(e.call(0x00ec_4835, &args![p, q, 12u32]).i32(), 0);
    }

    #[test]
    fn the_stack_probe_returns_the_new_stack_pointer_and_moves_the_return_address() {
        let mut e = Engine::new();
        let stack = e.mem.alloc(0x3000);
        // ESP at entry in the top page, 0x2400 bytes wanted: the probe walks two pages down.
        let esp = stack + 0x2f00;
        e.mem.set_u32(esp, 0x00c0_ffee);
        let new_esp = e.call(0x00ec8660, &args![0x2400u32, esp]).u32();
        assert_eq!(new_esp, esp - 0x2400 + 4);
        assert_eq!(e.mem.u32(esp - 0x2400), 0x00c0_ffee);
    }

    #[test]
    #[should_panic]
    fn the_stack_probe_stores_at_address_zero_when_the_size_borrows() {
        let mut e = Engine::new();
        let stack = e.mem.alloc(0x100);
        e.mem.set_u32(stack, 1);
        e.call(0x00ec8660, &args![stack + 1, stack]);
    }

    #[test]
    fn time64_adds_the_epoch_offset_divides_and_stores() {
        let mut e = Engine::new();
        e.map(0x00fd_f000, 0x1000);
        e.mem
            .set_u32(GET_SYSTEM_TIME_AS_FILE_TIME_SLOT, 0x0900_0010);
        // The FILETIME for 2020-01-01 00:00:00 UTC: 132223104000000000.
        e.register(0x0900_0010, |e, a| {
            e.mem.set_u32(a[0], 0x6905_0000);
            e.mem.set_u32(a[0] + 4, 0x01d5_c036);
            Ret::default()
        });
        e.register(AULLDIV, |_, a| {
            let dividend = a[0] as u64 | (a[1] as u64) << 32;
            let divisor = a[2] as u64 | (a[3] as u64) << 32;
            let q = dividend / divisor;
            Ret {
                eax: q as u32,
                edx: (q >> 32) as u32,
                ..Ret::default()
            }
        });
        let out = e.mem.alloc(8);
        let now = e.call(0x00ecadda, &args![out]).u64();
        assert_eq!(now, 1_577_836_800);
        assert_eq!(e.mem.u32(out), 1_577_836_800);
        assert_eq!(e.mem.u32(out + 4), 0);
        // A null pointer is allowed.
        assert_eq!(e.call(0x00ecadda, &args![0u32]).u64(), 1_577_836_800);
    }

    #[test]
    fn time64_gives_minus_one_beyond_the_year_3000() {
        let mut e = Engine::new();
        e.map(0x00fd_f000, 0x1000);
        e.mem
            .set_u32(GET_SYSTEM_TIME_AS_FILE_TIME_SLOT, 0x0900_0010);
        e.register(0x0900_0010, |_, _| Ret::default());
        // The quotient is the high word and low word the double returns.
        for (high, low, expected) in [
            (6u32, 0xffff_ffffu32, (6u64 << 32) | 0xffff_ffff),
            (7, 0x9340_6fff, (7u64 << 32) | 0x9340_6fff),
            (7, 0x9340_7000, u64::MAX),
            (8, 0, u64::MAX),
        ] {
            // Plain functions cannot capture the case: drive it through memory.
            e.mem.set_u32(0x00fd_f400, high);
            e.mem.set_u32(0x00fd_f404, low);
            e.register(AULLDIV, |e, _| Ret {
                eax: e.mem.u32(0x00fd_f404),
                edx: e.mem.u32(0x00fd_f400),
                ..Ret::default()
            });
            assert_eq!(e.call(0x00ecadda, &args![0u32]).u64(), expected);
        }
    }
}

#[cfg(test)]
mod exit_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Order = Rc<RefCell<Vec<u32>>>;

    /// Doubles at `addrs` that append their own address to the returned log.
    fn recorders(e: &mut Engine, addrs: &[u32]) -> Order {
        let order: Order = Rc::default();
        for &a in addrs {
            let o = order.clone();
            e.register_double(a, move |_, _| {
                o.borrow_mut().push(a);
                Ret::default()
            });
        }
        order
    }

    /// The exit table's functions, decoded as `doexit` decodes them.
    fn table_words(e: &mut Engine) -> Vec<u32> {
        let begin = table_begin(e);
        let encoded = e.mem.u32(ONEXIT_END);
        let end = e.call(DECODE_POINTER, &args![encoded]).u32();
        (begin..end)
            .step_by(4)
            .map(|a| {
                let word = e.mem.u32(a);
                e.call(DECODE_POINTER, &args![word]).u32()
            })
            .collect()
    }

    /// The exit table's block (its start, decoded).
    fn table_begin(e: &mut Engine) -> u32 {
        let encoded = e.mem.u32(ONEXIT_BEGIN);
        e.call(DECODE_POINTER, &args![encoded]).u32()
    }

    /// An engine after the start-up's exit-table set-up.
    fn exit_engine() -> Engine {
        let mut e = Engine::new();
        start_up_test::exit_table(&mut e);
        e
    }

    #[test]
    fn atexit_records_the_function_in_registration_order() {
        let mut e = exit_engine();
        assert_eq!(e.call(0x00ec658f, &[0x0090_0010]).i32(), 0);
        assert_eq!(e.call(0x00ec658f, &[0x0090_0020]).i32(), 0);
        assert_eq!(table_words(&mut e), [0x0090_0010, 0x0090_0020]);
        // The exe's first table: 32 words of zeros, filled from the start.
        let begin = table_begin(&mut e);
        assert_eq!(e.mem.block_size(begin), Some(128));
        assert_eq!(e.mem.u32(begin + 8), 0);
    }

    #[test]
    fn atexit_of_a_null_function_stores_it_and_answers_minus_one() {
        let mut e = exit_engine();
        assert_eq!(e.call(0x00ec658f, &[0]).i32(), -1);
        assert_eq!(table_words(&mut e), [0]);
    }

    #[test]
    fn the_table_grows_by_its_size_and_keeps_its_entries() {
        let mut e = exit_engine();
        for i in 0..100u32 {
            assert_eq!(atexit(&mut e, Ptr::new(0x0090_0000 + i)), 0);
        }
        let expect: Vec<u32> = (0..100).map(|i| 0x0090_0000 + i).collect();
        assert_eq!(table_words(&mut e), expect);
        // 128 bytes, then 256 (full at 64 words), then 512 (128 words).
        let begin = table_begin(&mut e);
        assert_eq!(e.mem.block_size(begin), Some(512));
    }

    #[test]
    fn doexit_runs_the_exit_functions_last_first_then_the_two_tables() {
        let mut e = exit_engine();
        let order = recorders(
            &mut e,
            &[
                0x0090_0010,
                0x0090_0020,
                0x0090_0030,
                0x0090_1000,
                0x0090_1010,
                0x0090_2000,
            ],
        );
        for f in [0x0090_0010, 0x0090_0020, 0x0090_0030] {
            atexit(&mut e, Ptr::new(f));
        }
        e.mem.map(PRE_TERMINATORS.0, 0x20);
        // Slots: pre-terminators 3 (one empty), then the terminator slot.
        e.mem.set_u32(0x0101_13f8, 0x0090_1000);
        e.mem.set_u32(0x0101_13fc, 0x0090_1010);
        e.mem.set_u32(0x0101_1404, 0x0090_2000);
        // A word past each table's end is not run.
        e.mem.set_u32(0x0101_1400, 0x0090_3000);
        e.mem.set_u32(0x0101_1408, 0x0090_3000);
        let report = doexit(&mut e, 5, 0, 0);
        assert_eq!(
            *order.borrow(),
            [
                0x0090_0030,
                0x0090_0020,
                0x0090_0010,
                0x0090_1000,
                0x0090_1010,
                0x0090_2000
            ]
        );
        assert_eq!(report.ran, *order.borrow());
        assert!(report.open.is_empty() && report.stopped.is_empty());
        assert_eq!(report.exit_process, Some(5));
        // Each exit function was cleared before it ran.
        assert_eq!(table_words(&mut e), [0, 0, 0]);
        assert_eq!(e.mem.u32(EXIT_STARTED), 1);
        assert_eq!(e.mem.u32(EXIT_DONE), 1);
        assert_eq!(e.mem.u8(EXIT_RETURNS), 0);
    }

    #[test]
    fn a_quick_exit_runs_only_the_terminators() {
        let mut e = exit_engine();
        let order = recorders(&mut e, &[0x0090_0010, 0x0090_1000, 0x0090_2000]);
        atexit(&mut e, Ptr::new(0x0090_0010));
        e.mem.map(PRE_TERMINATORS.0, 0x20);
        e.mem.set_u32(0x0101_13f8, 0x0090_1000);
        e.mem.set_u32(0x0101_1404, 0x0090_2000);
        let report = doexit(&mut e, 1, 1, 0);
        assert_eq!(*order.borrow(), [0x0090_2000]);
        assert_eq!(report.exit_process, Some(1));
        // The exit function is still registered.
        assert_eq!(table_words(&mut e), [0x0090_0010]);
    }

    #[test]
    fn a_second_exit_after_the_process_exit_began_runs_nothing() {
        let mut e = exit_engine();
        let order = recorders(&mut e, &[0x0090_2000]);
        e.mem.map(PRE_TERMINATORS.0, 0x20);
        e.mem.set_u32(0x0101_1404, 0x0090_2000);
        doexit(&mut e, 0, 0, 0);
        let again = doexit(&mut e, 3, 0, 0);
        assert_eq!(*order.borrow(), [0x0090_2000]);
        assert!(again.ran.is_empty());
        assert_eq!(again.exit_process, Some(3));
    }

    #[test]
    fn a_returning_exit_keeps_going_and_records_the_caller_flag() {
        let mut e = exit_engine();
        let order = recorders(&mut e, &[0x0090_2000]);
        e.mem.map(PRE_TERMINATORS.0, 0x20);
        e.mem.set_u32(0x0101_1404, 0x0090_2000);
        let report = doexit(&mut e, 0, 0, 1);
        assert_eq!(report.exit_process, None);
        assert_eq!(e.mem.u32(EXIT_DONE), 0);
        assert_eq!(e.mem.u8(EXIT_RETURNS), 1);
        doexit(&mut e, 0, 0, 1);
        assert_eq!(*order.borrow(), [0x0090_2000, 0x0090_2000]);
    }

    #[test]
    fn a_handler_that_registers_another_restarts_the_walk_at_the_new_end() {
        let mut e = exit_engine();
        let order = recorders(&mut e, &[0x0090_0020, 0x0090_0040]);
        let o = order.clone();
        e.register_double(0x0090_0030, move |e, _| {
            o.borrow_mut().push(0x0090_0030);
            atexit(e, Ptr::new(0x0090_0040));
            Ret::default()
        });
        atexit(&mut e, Ptr::new(0x0090_0020));
        atexit(&mut e, Ptr::new(0x0090_0030));
        doexit(&mut e, 0, 0, 1);
        // 30 runs first and registers 40, which runs next, then 20.
        assert_eq!(*order.borrow(), [0x0090_0030, 0x0090_0040, 0x0090_0020]);
    }

    #[test]
    fn handlers_nobody_translated_are_skipped_and_counted() {
        let mut e = exit_engine();
        let order = recorders(&mut e, &[0x0090_0010]);
        // 00f00f00 is not translated; the handler at 00900030 stops in it.
        e.register(0x0090_0030, |e, _| e.call(0x00f0_0f00, &[]));
        for f in [0x0090_0010, 0x00f0_0f00, 0x0090_0030] {
            atexit(&mut e, Ptr::new(f));
        }
        let report = doexit(&mut e, 0, 0, 1);
        assert_eq!(report.open, [0x00f0_0f00]);
        assert_eq!(report.stopped.len(), 1);
        assert_eq!(report.stopped[0].0, 0x0090_0030);
        assert!(report.stopped[0].1.contains("open function 00f00f00"));
        assert_eq!(*order.borrow(), [0x0090_0010]);
        assert_eq!(report.ran, [0x0090_0010]);
    }

    #[test]
    fn destructor_rows_run_through_the_exit_functions() {
        // 00fc7a70: call 00404a00 on 011c3264 (data/atexit_destructors.tsv).
        let mut e = exit_engine();
        let seen: Order = Rc::default();
        let s = seen.clone();
        e.register_double(0x0040_4a00, move |_, a| {
            s.borrow_mut().extend_from_slice(a);
            Ret::default()
        });
        assert_eq!(e.call(0x00ec658f, &[0x00fc_7a70]).i32(), 0);
        let report = doexit(&mut e, 0, 0, 1);
        assert_eq!(report.ran, [0x00fc_7a70]);
        assert_eq!(*seen.borrow(), [0x011c_3264]);
    }

    #[test]
    fn doexit_is_callable_at_its_address() {
        let mut e = exit_engine();
        let order = recorders(&mut e, &[0x0090_0010]);
        atexit(&mut e, Ptr::new(0x0090_0010));
        e.call(0x00eccd99, &args![0u32, 0u32, 1u32]);
        assert_eq!(*order.borrow(), [0x0090_0010]);
    }

    #[test]
    fn ctrlfp_merges_the_masked_bits_and_returns_the_old_control_word() {
        let mut e = Engine::new();
        // The game's control word until something loads another.
        // The FPU keeps bit 6 set (measured): 0x133f reads back as 0x137f.
        assert_eq!(ctrlfp(&mut e, 0x133f, 0xffff), 0x027f);
        assert_eq!(x87_control_word(&mut e), 0x137f);
        // Only the masked bits change.
        assert_eq!(ctrlfp(&mut e, 0x0c00, 0x0c00), 0x137f);
        assert_eq!(x87_control_word(&mut e), 0x1f7f);
        // FLDCW takes the low 16 bits (bit 15 reads 0 afterwards: 0x8000 is kept as 0x0040);
        // the old word comes back sign-extended.
        assert_eq!(
            e.call(0x00ed5abb, &args![0x0001_8000u32, 0xffff_ffffu32])
                .i32(),
            0x1f7f
        );
        assert_eq!(ctrlfp(&mut e, 0x027f, 0xffff), 0x0040);
        assert_eq!(x87_control_word(&mut e), 0x027f);
    }
}

#[cfg(test)]
mod physics_step_tests {
    use super::*;
    use std::cell::RefCell;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A double at `addr` that records its call in `log` and answers `ret`.
    fn recorder(e: &mut Engine, log: &Log, addr: u32, ret: Ret) {
        let log = log.clone();
        e.register_double(addr, move |_, a| {
            log.borrow_mut().push((addr, a.to_vec()));
            ret
        });
    }

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn panic_text(result: std::thread::Result<Ret>) -> String {
        let payload = result.expect_err("the engine stops with a panic");
        match payload.downcast_ref::<String>() {
            Some(text) => text.clone(),
            None => (*payload.downcast_ref::<&str>().unwrap()).to_string(),
        }
    }

    fn words(value: f64) -> [u32; 2] {
        let bits = value.to_bits();
        [bits as u32, (bits >> 32) as u32]
    }

    // ---- _errno, _vsnprintf, __iob_func ------------------------------------------------

    #[test]
    fn errno_is_the_thread_blocks_terrno_or_the_static_slot() {
        let mut e = Engine::new();
        e.register(GETPTD_NOEXIT, |_, _| eax(0x7000));
        assert_eq!(e.call(0x00ec_85e3, &args![]).u32(), 0x7008);
        e.register(GETPTD_NOEXIT, |_, _| eax(0));
        assert_eq!(e.call(0x00ec_85e3, &args![]).u32(), 0x011c_18c0);
    }

    #[test]
    fn vsnprintf_passes_a_null_locale_to_the_locale_version() {
        let mut e = Engine::new();
        let log = Log::default();
        recorder(&mut e, &log, VSNPRINTF_L, eax(17));
        let result = e.call(0x00ec_ddf1, &args![0x100u32, 64u32, 0x200u32, 0x300u32]);
        assert_eq!(result.i32(), 17);
        assert_eq!(
            calls_to(&log, VSNPRINTF_L),
            vec![vec![0x100, 64, 0x200, 0, 0x300]]
        );
    }

    #[test]
    fn iob_func_is_the_address_of_the_stream_table() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ec_df44, &args![]).u32(), 0x011c_1a08);
    }

    // ---- the invalid-parameter path ------------------------------------------------------

    #[test]
    fn a_registered_handler_gets_the_five_words() {
        let mut e = Engine::new();
        e.map(INVALID_PARAMETER_HANDLER, 4);
        e.set_global(INVALID_PARAMETER_HANDLER, 0xaaaa_bbbbu32);
        let log = Log::default();
        recorder(&mut e, &log, DECODE_POINTER, eax(0x0050_0000));
        recorder(&mut e, &log, 0x0050_0000, Ret::default());
        recorder(&mut e, &log, INVOKE_WATSON, Ret::default());
        e.call(0x00ec_7c30, &args![1u32, 2u32, 3u32, 4u32, 5u32]);
        assert_eq!(calls_to(&log, DECODE_POINTER), vec![vec![0xaaaa_bbbb]]);
        assert_eq!(calls_to(&log, 0x0050_0000), vec![vec![1, 2, 3, 4, 5]]);
        assert!(calls_to(&log, INVOKE_WATSON).is_empty());
        assert!(calls_to(&log, CLEAR_01270A60).is_empty());
    }

    #[test]
    fn without_a_handler_it_clears_the_flag_and_invokes_watson() {
        let mut e = Engine::new();
        e.map(INVALID_PARAMETER_HANDLER, 4);
        let log = Log::default();
        recorder(&mut e, &log, DECODE_POINTER, eax(0));
        recorder(&mut e, &log, CLEAR_01270A60, Ret::default());
        recorder(&mut e, &log, INVOKE_WATSON, Ret::default());
        e.call(0x00ec_7c30, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, vec![DECODE_POINTER, CLEAR_01270A60, INVOKE_WATSON]);
        assert_eq!(calls_to(&log, CLEAR_01270A60), vec![vec![2]]);
        assert_eq!(calls_to(&log, INVOKE_WATSON), vec![vec![0, 0, 0, 0, 0]]);
    }

    // ---- the exits -------------------------------------------------------------------------

    #[test]
    fn crt_exit_process_tells_the_runtime_then_exits_and_stops_the_engine() {
        let mut e = Engine::new();
        let log = Log::default();
        recorder(&mut e, &log, CRT_COR_EXIT_PROCESS, Ret::default());
        recorder(&mut e, &log, EXIT_PROCESS, Ret::default());
        let text = panic_text(catch_unwind(AssertUnwindSafe(|| {
            e.call(0x00ec_cca9, &args![3u32])
        })));
        assert!(text.contains("00eccca9"));
        assert!(text.contains("ExitProcess(3)"));
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(order, vec![CRT_COR_EXIT_PROCESS, EXIT_PROCESS]);
        assert_eq!(calls_to(&log, CRT_COR_EXIT_PROCESS), vec![vec![3]]);
        assert_eq!(calls_to(&log, EXIT_PROCESS), vec![vec![3]]);
    }

    #[test]
    fn report_gsfailure_fills_the_record_and_ends_the_process() {
        let mut e = Engine::new();
        e.map(0x0126_fe00, 0x200);
        let log = Log::default();
        recorder(&mut e, &log, IS_DEBUGGER_PRESENT, eax(0));
        recorder(&mut e, &log, CLEAR_01270A60, Ret::default());
        recorder(&mut e, &log, SET_UNHANDLED_EXCEPTION_FILTER, Ret::default());
        recorder(&mut e, &log, UNHANDLED_EXCEPTION_FILTER, Ret::default());
        recorder(&mut e, &log, GET_CURRENT_PROCESS, eax(0xffff_ffff));
        recorder(&mut e, &log, TERMINATE_PROCESS, Ret::default());
        let text = panic_text(catch_unwind(AssertUnwindSafe(|| {
            e.call(0x00ed_159c, &args![])
        })));
        assert!(text.contains("00ed159c"));
        assert!(text.contains("0xc0000409"));
        assert_eq!(e.mem.u32(0x0126_fe18), 0xc000_0409);
        assert_eq!(e.mem.u32(0x0126_fe1c), 1);
        assert_eq!(e.mem.u32(0x0126_fe70), 0x0001_0001);
        assert_eq!(e.mem.u32(0x0126_fe68), 0);
        let order: Vec<u32> = log.borrow().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                IS_DEBUGGER_PRESENT,
                CLEAR_01270A60,
                SET_UNHANDLED_EXCEPTION_FILTER,
                UNHANDLED_EXCEPTION_FILTER,
                CLEAR_01270A60,
                GET_CURRENT_PROCESS,
                TERMINATE_PROCESS,
            ]
        );
        assert_eq!(calls_to(&log, CLEAR_01270A60), vec![vec![1], vec![1]]);
        assert_eq!(
            calls_to(&log, UNHANDLED_EXCEPTION_FILTER),
            vec![vec![0x010f_2a40]]
        );
        assert_eq!(
            calls_to(&log, TERMINATE_PROCESS),
            vec![vec![0xffff_ffff, 0xc000_0409]]
        );
    }

    #[test]
    fn report_gsfailure_with_a_debugger_skips_the_second_hook_call() {
        let mut e = Engine::new();
        e.map(0x0126_fe00, 0x200);
        let log = Log::default();
        recorder(&mut e, &log, IS_DEBUGGER_PRESENT, eax(1));
        recorder(&mut e, &log, CLEAR_01270A60, Ret::default());
        recorder(&mut e, &log, SET_UNHANDLED_EXCEPTION_FILTER, Ret::default());
        recorder(&mut e, &log, UNHANDLED_EXCEPTION_FILTER, Ret::default());
        recorder(&mut e, &log, GET_CURRENT_PROCESS, eax(1));
        recorder(&mut e, &log, TERMINATE_PROCESS, Ret::default());
        let _ = catch_unwind(AssertUnwindSafe(|| e.call(0x00ed_159c, &args![])));
        assert_eq!(e.mem.u32(0x0126_fe68), 1);
        assert_eq!(calls_to(&log, CLEAR_01270A60), vec![vec![1]]);
    }

    // ---- __aulldiv -------------------------------------------------------------------------

    fn divide(e: &mut Engine, n: u64, d: u64) -> u64 {
        e.call(
            0x00ec_c590,
            &args![n as u32, (n >> 32) as u32, d as u32, (d >> 32) as u32],
        )
        .u64()
    }

    #[test]
    fn aulldiv_agrees_with_64_bit_division() {
        let mut e = Engine::new();
        let mut state = 0x1234_5678_9abc_def1u64;
        let mut next = move || {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            state
        };
        let edge = [
            0u64,
            1,
            2,
            0xffff_ffff,
            0x1_0000_0000,
            0x1_0000_0001,
            0x7fff_ffff_ffff_ffff,
            0x8000_0000_0000_0000,
            0xffff_ffff_ffff_ffff,
            0xfedc_ba98_7654_3210,
        ];
        for &n in &edge {
            for &d in edge.iter().filter(|d| **d != 0) {
                assert_eq!(divide(&mut e, n, d), n / d, "{n:#x} / {d:#x}");
            }
        }
        for round in 0..4000 {
            let n = next();
            // Divisors of every width: shift a random word down by 0..63 bits.
            let d = (next() >> (round % 64)).max(1);
            assert_eq!(divide(&mut e, n, d), n / d, "{n:#x} / {d:#x}");
        }
    }

    #[test]
    fn aulldiv_needs_its_correction_step_for_a_wide_divisor() {
        let mut e = Engine::new();
        // The shifted estimate is one too big here: the corrected quotient is 0.
        let n = 0x1_ffff_ffffu64;
        let d = 0x2_0000_0000u64;
        assert_eq!(divide(&mut e, n, d), 0);
        // And exactly on the boundary: the estimate stands.
        assert_eq!(divide(&mut e, 0x2_0000_0000, d), 1);
        assert_eq!(divide(&mut e, 0x3_ffff_ffff, d), 1);
        assert_eq!(divide(&mut e, 0x4_0000_0000, d), 2);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn aulldiv_by_zero_faults() {
        let mut e = Engine::new();
        divide(&mut e, 5, 0);
    }

    // ---- strncpy ---------------------------------------------------------------------------

    /// The C `strncpy` the exe's routine must agree with: bytes of the string up to `count`,
    /// zero-padded, nothing written past `count`.
    fn reference(source: &[u8], count: usize) -> Vec<u8> {
        let mut out = vec![0xaau8; count + 8];
        let mut ended = false;
        for i in 0..count {
            let byte = if ended { 0 } else { source[i] };
            if byte == 0 {
                ended = true;
            }
            out[i] = byte;
        }
        out
    }

    #[test]
    fn strncpy_matches_the_c_function_for_every_alignment_length_and_count() {
        let mut e = Engine::new();
        for source_align in 0..4u32 {
            for dest_align in 0..4u32 {
                for length in 0..14usize {
                    for count in 0..20usize {
                        let source_block = e.mem.alloc(64);
                        let dest_block = e.mem.alloc(64);
                        let mut text: Vec<u8> = (0..length).map(|i| b'A' + i as u8).collect();
                        text.push(0);
                        // Bytes after the terminator are not zero: the copy must not use them.
                        text.extend_from_slice(&[0x55; 24]);
                        let source = source_block + source_align;
                        e.mem.write(source, &text);
                        let dest = dest_block + dest_align;
                        e.mem.write(dest, &[0xaa; 40]);
                        let result = e
                            .call(0x00ec_c7b0, &args![dest, source, count as u32])
                            .u32();
                        assert_eq!(result, dest);
                        let expected = reference(&text, count);
                        let got = e.mem.bytes(dest, count as u32 + 8);
                        assert_eq!(
                            got, expected,
                            "source+{source_align} dest+{dest_align} length {length} count {count}"
                        );
                        e.mem.free(source_block);
                        e.mem.free(dest_block);
                    }
                }
            }
        }
    }

    #[test]
    fn strncpy_stores_a_word_cut_at_each_zero_byte_position() {
        let mut e = Engine::new();
        // An aligned source of exactly 8 bytes copied: the zero is in byte 0, 1, 2 or 3 of the
        // second word, and the first word is full (the (w ^ !0) ^ (w + 0x7efefeff) test fires
        // only for the second).
        for zero_at in 0..4usize {
            let source = e.mem.alloc(16);
            let dest = e.mem.alloc(16);
            let mut text = *b"WXYZabcd";
            text[4 + zero_at] = 0;
            e.mem.write(source, &text);
            e.mem.write(dest, &[0xaa; 16]);
            e.call(0x00ec_c7b0, &args![dest, source, 8u32]);
            let mut expected = text.to_vec();
            for byte in expected[4 + zero_at..].iter_mut() {
                *byte = 0;
            }
            assert_eq!(e.mem.bytes(dest, 8), expected, "zero at {zero_at}");
            assert_eq!(e.mem.bytes(dest + 8, 8), vec![0xaa; 8]);
        }
    }

    // ---- write_char / write_string ---------------------------------------------------------

    /// A `FILE` at a fresh block: `_ptr`, `_cnt`, `_base`, `_flag`, and a 16-byte buffer.
    fn stream(e: &mut Engine, cnt: i32, base: bool, flag: u32) -> (Ptr, u32) {
        let file = e.mem.alloc(0x20);
        let buffer = e.mem.alloc(16);
        e.mem.set_u32(file, buffer);
        e.mem.set_i32(file + 4, cnt);
        e.mem.set_u32(file + 8, if base { buffer } else { 0 });
        e.mem.set_u32(file + 0xc, flag);
        (Ptr::new(file), buffer)
    }

    #[test]
    fn write_char_stores_the_byte_and_counts_it() {
        let mut e = Engine::new();
        let (file, buffer) = stream(&mut e, 3, true, 0);
        let count = e.mem.alloc(4);
        e.mem.set_u32(count, 5);
        let result = e.call(
            0x00ed_2ec8,
            &args![0x1234_5641u32, file, Ptr::<()>::new(count)],
        );
        assert_eq!(result.u32(), 0x41, "the byte, zero-extended");
        assert_eq!(e.mem.u8(buffer), 0x41);
        assert_eq!(e.mem.u32(file.addr()), buffer + 1);
        assert_eq!(e.mem.i32(file.addr() + 4), 2);
        assert_eq!(e.mem.u32(count), 6);
    }

    #[test]
    fn write_char_calls_flsbuf_when_the_buffer_is_full() {
        let mut e = Engine::new();
        let (file, buffer) = stream(&mut e, 0, true, 0);
        let count = e.mem.alloc(4);
        let log = Log::default();
        recorder(&mut e, &log, FLSBUF, eax(0x80));
        let result = e.call(0x00ed_2ec8, &args![0xf0u32, file, Ptr::<()>::new(count)]);
        assert_eq!(result.u32(), 0x80);
        assert_eq!(e.mem.i32(file.addr() + 4), -1);
        assert_eq!(e.mem.u8(buffer), 0, "nothing stored by write_char");
        assert_eq!(
            calls_to(&log, FLSBUF),
            vec![vec![0xffff_fff0, file.addr()]],
            "the byte sign-extended, then the stream"
        );
        assert_eq!(e.mem.u32(count), 1);
    }

    #[test]
    fn write_char_with_a_failed_flsbuf_sets_the_count_to_minus_one() {
        let mut e = Engine::new();
        let (file, _) = stream(&mut e, 0, true, 0);
        let count = e.mem.alloc(4);
        e.mem.set_u32(count, 7);
        e.register(FLSBUF, |_, _| eax(0xffff_ffff));
        let result = e.call(0x00ed_2ec8, &args![0x41u32, file, Ptr::<()>::new(count)]);
        assert_eq!(result.u32(), 0xffff_ffff);
        assert_eq!(e.mem.i32(count), -1);
    }

    #[test]
    fn write_char_to_a_string_stream_without_a_buffer_only_counts() {
        let mut e = Engine::new();
        let (file, buffer) = stream(&mut e, 5, false, 0x40);
        let count = e.mem.alloc(4);
        let result = e.call(
            0x00ed_2ec8,
            &args![0xabcd_ef41u32, file, Ptr::<()>::new(count)],
        );
        assert_eq!(result.u32(), 0xabcd_ef41, "EAX is untouched");
        assert_eq!(e.mem.u32(count), 1);
        assert_eq!(e.mem.u8(buffer), 0);
        assert_eq!(e.mem.i32(file.addr() + 4), 5);
        // The flag alone is not enough: with a buffer the byte is stored.
        let (file, buffer) = stream(&mut e, 5, true, 0x40);
        e.call(0x00ed_2ec8, &args![0x42u32, file, Ptr::<()>::new(count)]);
        assert_eq!(e.mem.u8(buffer), 0x42);
    }

    #[test]
    fn write_string_writes_every_byte() {
        let mut e = Engine::new();
        let (file, buffer) = stream(&mut e, 8, true, 0);
        let count = e.mem.alloc(4);
        let text = e.mem.alloc(8);
        e.mem.write(text, b"hello");
        e.call(
            0x00ed_4638,
            &args![Ptr::<()>::new(count), Ptr::<()>::new(text), file, 5i32],
        );
        assert_eq!(e.mem.bytes(buffer, 5), b"hello");
        assert_eq!(e.mem.u32(count), 5);
        assert_eq!(e.mem.i32(file.addr() + 4), 3);
        // A length of zero or less writes nothing.
        e.call(
            0x00ed_4638,
            &args![Ptr::<()>::new(count), Ptr::<()>::new(text), file, -2i32],
        );
        assert_eq!(e.mem.u32(count), 5);
    }

    #[test]
    fn write_string_to_a_string_stream_without_a_buffer_adds_the_length() {
        let mut e = Engine::new();
        let (file, buffer) = stream(&mut e, 8, false, 0x40);
        let count = e.mem.alloc(4);
        e.mem.set_u32(count, 3);
        let text = e.mem.alloc(8);
        e.mem.write(text, b"hello");
        e.call(
            0x00ed_4638,
            &args![Ptr::<()>::new(count), Ptr::<()>::new(text), file, 5i32],
        );
        assert_eq!(e.mem.u32(count), 8);
        assert_eq!(e.mem.u8(buffer), 0);
    }

    #[test]
    fn write_string_stops_on_an_error_but_writes_a_question_mark_for_eilseq() {
        let mut e = Engine::new();
        let count = e.mem.alloc(4);
        let text = e.mem.alloc(8);
        e.mem.write(text, b"abc");
        let errno_slot = e.mem.alloc(4);
        e.register_double(GETPTD_NOEXIT, move |_, _| eax(errno_slot - 8));
        // Every flush fails; with errno EILSEQ (0x2a) the string goes on with a '?' each time.
        let (file, _) = stream(&mut e, 0, true, 0);
        let log = Log::default();
        recorder(&mut e, &log, FLSBUF, eax(0xffff_ffff));
        e.mem.set_u32(errno_slot, 0x2a);
        e.call(
            0x00ed_4638,
            &args![Ptr::<()>::new(count), Ptr::<()>::new(text), file, 3i32],
        );
        // For each of the three bytes: the byte (fails), then '?' (fails again).
        let flushed: Vec<u32> = calls_to(&log, FLSBUF).iter().map(|c| c[0]).collect();
        assert_eq!(flushed, vec![0x61, 0x3f, 0x62, 0x3f, 0x63, 0x3f]);
        // With any other errno it stops after the first failure.
        let (file, _) = stream(&mut e, 0, true, 0);
        e.mem.set_u32(errno_slot, 9);
        log.borrow_mut().clear();
        e.mem.set_u32(count, 0);
        e.call(
            0x00ed_4638,
            &args![Ptr::<()>::new(count), Ptr::<()>::new(text), file, 3i32],
        );
        let flushed: Vec<u32> = calls_to(&log, FLSBUF).iter().map(|c| c[0]).collect();
        assert_eq!(flushed, vec![0x61]);
        assert_eq!(e.mem.i32(count), -1);
    }

    // ---- __fload_withFB --------------------------------------------------------------------

    #[test]
    fn fload_with_fb_loads_a_finite_double_and_returns_its_exponent_bits() {
        let mut e = Engine::new();
        let slot = e.mem.alloc(8);
        e.mem.set_f64(slot, 6.5);
        let result = e.call(0x00ed_27c5, &[0, slot]);
        assert_eq!(result.st0, 6.5);
        assert_eq!(result.eax, 0x4010_0000);
        e.mem.set_f64(slot, -0.0);
        let result = e.call(0x00ed_27c5, &[0, slot]);
        assert_eq!(result.st0.to_bits(), (-0.0f64).to_bits());
        assert_eq!(result.eax, 0);
    }

    #[test]
    fn fload_with_fb_keeps_infinities_and_signalling_nans_as_they_are() {
        let mut e = Engine::new();
        let slot = e.mem.alloc(8);
        for bits in [
            0x7ff0_0000_0000_0000u64,
            0xfff0_0000_0000_0000,
            0x7ff0_0000_0000_0001, // signalling: not quieted by this load
            0x7ff8_0000_0000_1234,
            0xfff8_0000_0000_0000,
        ] {
            e.mem.set_u64(slot, bits);
            let result = e.call(0x00ed_27c5, &[0xdead, slot]);
            assert_eq!(result.st0.to_bits(), bits, "{bits:#x}");
            assert_eq!(result.eax, (bits >> 32) as u32, "the whole high word");
        }
    }

    // ---- ceil ------------------------------------------------------------------------------

    /// The constants of `ceil`'s SSE2 body and the flags its dispatch reads.
    fn ceil_setup(e: &mut Engine, sse2: bool) {
        e.map(0x010f_1fe0, 0x60);
        e.mem.set_u64(CEIL_SHIFT_BASE, 0x433);
        e.mem.set_u64(CEIL_SHIFT_BASE + 8, 0x433);
        e.mem.set_u64(CEIL_ONE_MASK, 1.0f64.to_bits());
        e.mem.set_u64(CEIL_ONE_MASK + 8, 1.0f64.to_bits());
        e.mem.set_u64(CEIL_ZERO, 0);
        e.mem.set_u64(CEIL_EXPONENT_MASK, 0x7ff);
        e.mem.set_u64(CEIL_MINUS_ZERO, (-0.0f64).to_bits());
        e.map(USE_SSE2_MATHFCNS, 4);
        e.set_global(USE_SSE2_MATHFCNS, u32::from(sse2));
    }

    fn ceil_via_registry(e: &mut Engine, x: f64) -> f64 {
        e.call(0x00ec_9e10, &args![x]).f64()
    }

    #[test]
    fn ceil_sse2_matches_the_host_ceil_bit_for_bit() {
        let mut e = Engine::new();
        ceil_setup(&mut e, true);
        let values = [
            0.0f64,
            -0.0,
            0.25,
            0.5,
            0.999_999_999_999,
            1.0,
            1.000_000_000_001,
            1.5,
            2.0,
            2.000_000_1,
            123.456,
            4_503_599_627_370_495.5, // 2^52 - 0.5
            4_503_599_627_370_496.0, // 2^52
            9_007_199_254_740_991.0,
            1.0e300,
            f64::MAX,
            f64::INFINITY,
            f64::MIN_POSITIVE,
            f64::from_bits(1),
            -f64::from_bits(1),
            -0.25,
            -0.5,
            -0.999_999_999_999,
            -1.0,
            -1.000_000_000_001,
            -1.5,
            -2.5,
            -123.456,
            -4_503_599_627_370_495.5,
            -4_503_599_627_370_496.0,
            -1.0e300,
            f64::MIN,
            f64::NEG_INFINITY,
        ];
        for x in values {
            let got = ceil_via_registry(&mut e, x);
            assert_eq!(
                got.to_bits(),
                x.ceil().to_bits(),
                "ceil({x:e}) = {got:e}, host {:e}",
                x.ceil()
            );
        }
    }

    #[test]
    fn ceil_sse2_of_a_nan_lets_the_error_support_rewrite_the_result_slot() {
        let mut e = Engine::new();
        ceil_setup(&mut e, true);
        let log = Log::default();
        let seen = log.clone();
        e.register_double(LIBM_ERROR_SUPPORT, move |e, a| {
            seen.borrow_mut().push((LIBM_ERROR_SUPPORT, a.to_vec()));
            assert_eq!(a[0], a[1]);
            assert_eq!(a[1], a[2]);
            // The slot holds the argument; the support routine replaces it.
            assert_eq!(e.mem.u64(a[2]), 0x7ff0_0000_0000_0123);
            e.mem.set_u64(a[2], 0x7ff8_0000_0000_0777);
            Ret::default()
        });
        let nan = f64::from_bits(0x7ff0_0000_0000_0123); // signalling
        let got = ceil_via_registry(&mut e, nan);
        assert_eq!(got.to_bits(), 0x7ff8_0000_0000_0777);
        let calls = calls_to(&log, LIBM_ERROR_SUPPORT);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][3], 0x3ec);
        // A support routine that leaves the slot alone: the NaN comes back quieted by FLD.
        e.register(LIBM_ERROR_SUPPORT, |_, _| Ret::default());
        let got = ceil_via_registry(&mut e, nan);
        assert_eq!(got.to_bits(), 0x7ff8_0000_0000_0123);
    }

    #[test]
    fn ceil_takes_the_x87_body_without_the_sse2_flag_or_with_unmasked_control_bits() {
        let mut e = Engine::new();
        ceil_setup(&mut e, false);
        e.map(CEIL_CONTROL_WORD, 4);
        e.set_global(CEIL_CONTROL_WORD, 0x1b3fu32);
        let log = Log::default();
        // __frnd rounds up here (the round-up control word is loaded).
        let seen = log.clone();
        e.register_double(FRND, move |e, a| {
            seen.borrow_mut().push((FRND, a.to_vec()));
            assert_eq!(
                x87_control_word(e),
                0x1b7f,
                "round up while it runs (bit 6 reads 1)"
            );
            st0(f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).ceil())
        });
        let got = ceil_via_registry(&mut e, 2.25);
        assert_eq!(got, 3.0);
        assert_eq!(
            x87_control_word(&mut e),
            0x027f,
            "the old control word is back"
        );
        assert_eq!(calls_to(&log, FRND), vec![words(2.25).to_vec()]);
        // SSE2 flag set but the control word's exception masks not all set: x87 body again.
        e.set_global(USE_SSE2_MATHFCNS, 1u32);
        ctrlfp(&mut e, 0x027e, 0xffff);
        log.borrow_mut().clear();
        assert_eq!(ceil_via_registry(&mut e, -2.25), -2.0);
        assert_eq!(calls_to(&log, FRND).len(), 1);
    }

    #[test]
    fn ceil_x87_reports_an_inexact_result_only_when_the_precision_mask_is_clear() {
        let mut e = Engine::new();
        ceil_setup(&mut e, false);
        e.map(CEIL_CONTROL_WORD, 4);
        e.set_global(CEIL_CONTROL_WORD, 0x1b3fu32);
        e.register(FRND, |_, a| {
            st0(f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).ceil())
        });
        let log = Log::default();
        recorder(&mut e, &log, EXCEPT1, st0(42.0));
        // Exact: no report, whatever the mask.
        ctrlfp(&mut e, 0x025f, 0xffff); // precision exception unmasked (bit 5 clear)
        assert_eq!(ceil_via_registry(&mut e, 3.0), 3.0);
        assert!(calls_to(&log, EXCEPT1).is_empty());
        // Inexact with the mask clear: __except1(0x10, 0xc, x, result, old control word).
        let got = ceil_via_registry(&mut e, 2.5);
        assert_eq!(got, 42.0, "the report's result is the result");
        let x = words(2.5);
        let r = words(3.0);
        assert_eq!(
            calls_to(&log, EXCEPT1),
            vec![vec![0x10, 0xc, x[0], x[1], r[0], r[1], 0x025f]]
        );
        // Inexact with the mask set: the rounded value, no report.
        ctrlfp(&mut e, 0x027f, 0xffff);
        log.borrow_mut().clear();
        assert_eq!(ceil_via_registry(&mut e, 2.5), 3.0);
        assert!(calls_to(&log, EXCEPT1).is_empty());
    }

    #[test]
    fn ceil_x87_sorts_infinities_and_nans_by_sptype() {
        let mut e = Engine::new();
        ceil_setup(&mut e, false);
        e.map(CEIL_CONTROL_WORD, 4);
        e.set_global(CEIL_CONTROL_WORD, 0x1b3fu32);
        e.map(ONE_DOUBLE, 8);
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        let log = Log::default();
        let class = Rc::new(RefCell::new(0u32));
        let c = class.clone();
        let seen = log.clone();
        e.register_double(SPTYPE, move |_, a| {
            seen.borrow_mut().push((SPTYPE, a.to_vec()));
            eax(*c.borrow())
        });
        recorder(&mut e, &log, HANDLE_QNAN1, st0(7.0));
        recorder(&mut e, &log, EXCEPT1, st0(9.0));
        // A signalling NaN; `q` is it quieted, as FLD/FSTP copy it.
        let x = f64::from_bits(0x7ff0_0000_0000_0042);
        let q = words(f64::from_bits(0x7ff8_0000_0000_0042));
        // Classes 1 and 2: the value (quieted by FLD) and the control word restored.
        for value in [1u32, 2] {
            *class.borrow_mut() = value;
            let got = ceil_via_registry(&mut e, x);
            assert_eq!(got.to_bits(), 0x7ff8_0000_0000_0042);
            assert_eq!(x87_control_word(&mut e), 0x027f);
        }
        // Class 3: __handle_qnan1(0xc, x quieted, old control word).
        *class.borrow_mut() = 3;
        ctrlfp(&mut e, 0x027f, 0xffff);
        assert_eq!(ceil_via_registry(&mut e, x), 7.0);
        assert_eq!(
            calls_to(&log, HANDLE_QNAN1),
            vec![vec![0xc, q[0], q[1], 0x027f]]
        );
        // __sptype got the FLD/FSTP copy each time (00ed96ca), not the argument's own bits.
        assert_eq!(calls_to(&log, SPTYPE), vec![q.to_vec(); 3]);
        // Other classes: __except1(8, 0xc, x, x + 1.0 (NaN quieted), old control word).
        for value in [0u32, 4, 0xffff_ffff] {
            *class.borrow_mut() = value;
            log.borrow_mut().clear();
            ctrlfp(&mut e, 0x027f, 0xffff);
            assert_eq!(ceil_via_registry(&mut e, x), 9.0);
            assert_eq!(
                calls_to(&log, EXCEPT1),
                vec![vec![8, 0xc, q[0], q[1], q[0], q[1], 0x027f]],
                "class {value:#x}"
            );
            assert_eq!(calls_to(&log, SPTYPE), vec![q.to_vec()], "class {value:#x}");
        }
        // An infinity of another kind: x + 1.0 is the infinity.
        *class.borrow_mut() = 0;
        log.borrow_mut().clear();
        ctrlfp(&mut e, 0x027f, 0xffff);
        ceil_via_registry(&mut e, f64::NEG_INFINITY);
        let inf = words(f64::NEG_INFINITY);
        assert_eq!(
            calls_to(&log, EXCEPT1),
            vec![vec![8, 0xc, inf[0], inf[1], inf[0], inf[1], 0x027f]]
        );
        assert_eq!(calls_to(&log, SPTYPE), vec![inf.to_vec()]);
    }

    #[test]
    fn ceil_x87_hands_a_signalling_nan_to_handle_qnan1_as_sptype_sees_it_quiet() {
        // The translated __sptype (00ed5981) runs: 00ed96ca stores the FLD'd argument for it, so
        // 0x7ff0_0000_0000_0042 arrives as 0x7ff8_0000_0000_0042, class 3 (not 4), and goes to
        // __handle_qnan1 rather than __except1.
        let mut e = Engine::new();
        ceil_setup(&mut e, false);
        e.map(CEIL_CONTROL_WORD, 4);
        e.set_global(CEIL_CONTROL_WORD, 0x1b3fu32);
        e.map(ONE_DOUBLE, 8);
        e.mem.set_f64(ONE_DOUBLE, 1.0);
        let log = Log::default();
        recorder(&mut e, &log, HANDLE_QNAN1, st0(7.0));
        recorder(&mut e, &log, EXCEPT1, st0(9.0));
        ctrlfp(&mut e, 0x027f, 0xffff);
        let got = ceil_via_registry(&mut e, f64::from_bits(0x7ff0_0000_0000_0042));
        assert_eq!(got, 7.0);
        let q = words(f64::from_bits(0x7ff8_0000_0000_0042));
        assert_eq!(
            calls_to(&log, HANDLE_QNAN1),
            vec![vec![0xc, q[0], q[1], 0x027f]]
        );
        assert!(calls_to(&log, EXCEPT1).is_empty());
    }

    // ---- _CIasin ---------------------------------------------------------------------------

    #[test]
    fn ci_asin_jumps_to_the_sse2_body_when_available() {
        let mut e = Engine::new();
        e.map(USE_SSE2_MATHFCNS, 4);
        e.set_global(USE_SSE2_MATHFCNS, 1u32);
        let log = Log::default();
        recorder(&mut e, &log, ASIN_SSE2, st0(0.5));
        recorder(&mut e, &log, LOAD_FLAGS_HELPER, Ret::default());
        recorder(&mut e, &log, ASIN_X87, st0(0.25));
        assert_eq!(e.call(0x00ec_9650, &args![0.3f64]).f64(), 0.5);
        assert_eq!(calls_to(&log, ASIN_SSE2), vec![words(0.3).to_vec()]);
        assert!(calls_to(&log, ASIN_X87).is_empty());
    }

    #[test]
    fn ci_asin_runs_the_x87_body_with_zf_from_the_exponent() {
        let mut e = Engine::new();
        e.map(USE_SSE2_MATHFCNS, 4);
        let log = Log::default();
        recorder(&mut e, &log, ASIN_SSE2, st0(0.5));
        recorder(&mut e, &log, LOAD_FLAGS_HELPER, eax(0x3fd0_0000));
        recorder(&mut e, &log, ASIN_X87, st0(0.25));
        assert_eq!(e.call(0x00ec_9650, &args![0.3f64]).f64(), 0.25);
        let w = words(0.3);
        assert_eq!(calls_to(&log, LOAD_FLAGS_HELPER), vec![vec![w[0], w[1]]]);
        assert_eq!(
            calls_to(&log, ASIN_X87),
            vec![vec![0, 0x3fd0_0000, w[0], w[1], w[0], w[1]]],
            "ZF clear, EAX from the helper, ST0, the stack double"
        );
        assert!(calls_to(&log, ASIN_SSE2).is_empty());
        // An infinity or NaN sets ZF.
        log.borrow_mut().clear();
        e.call(0x00ec_9650, &args![f64::NAN]);
        let w = words(f64::NAN);
        assert_eq!(
            calls_to(&log, ASIN_X87),
            vec![vec![1, 0x3fd0_0000, w[0], w[1], w[0], w[1]]]
        );
        // The SSE2 flag set but the control word's masks not: still the x87 body.
        e.set_global(USE_SSE2_MATHFCNS, 1u32);
        ctrlfp(&mut e, 0x0200, 0xffff);
        log.borrow_mut().clear();
        e.call(0x00ec_9650, &args![0.1f64]);
        assert_eq!(calls_to(&log, ASIN_X87).len(), 1);
        assert!(calls_to(&log, ASIN_SSE2).is_empty());
    }

    #[test]
    fn ci_asin_stores_a_signalling_nan_quiet_so_its_error_type_is_7() {
        let mut e = Engine::new();
        e.map(USE_SSE2_MATHFCNS, 4);
        let log = Log::default();
        recorder(&mut e, &log, LOAD_FLAGS_HELPER, eax(0x7ff8_0000));
        recorder(&mut e, &log, ASIN_X87, st0(1.0));
        e.call(0x00ec_9650, &args![0x7ff0_0000_0000_0001u64]);
        let quiet = words(f64::from_bits(0x7ff8_0000_0000_0001));
        assert_eq!(calls_to(&log, LOAD_FLAGS_HELPER), vec![quiet.to_vec()]);
        assert_eq!(
            calls_to(&log, ASIN_X87),
            vec![vec![1, 0x7ff8_0000, quiet[0], quiet[1], quiet[0], quiet[1]]],
            "the register and the stored double quiet"
        );
        // Through the real 00ed2808, 00ec96a8 and 00ed27ac: __87except sees type 7 for the
        // operation 0xe, as the exe reports it, and the NaN comes back quiet.
        let mut e = Engine::new();
        e.map(USE_SSE2_MATHFCNS, 4);
        e.map(MATH_ERROR_SWITCH, 4);
        let seen = Log::default();
        let record = seen.clone();
        e.register_double(0x00ee_1b04, move |e, a| {
            record.borrow_mut().push((a[0], vec![e.mem.u32(a[1])]));
            Ret::default()
        });
        let r = e.call(0x00ec_9650, &args![0x7ff0_0000_0000_0001u64]).f64();
        assert_eq!(r.to_bits(), 0x7ff8_0000_0000_0001);
        assert_eq!(*seen.borrow(), vec![(0xe, vec![7])]);
    }

    // ---- the shared square-root body -------------------------------------------------------

    fn sqrt_setup(e: &mut Engine, error_switch: u32) {
        e.map(MATH_ERROR_SWITCH, 4);
        e.set_global(MATH_ERROR_SWITCH, error_switch);
        e.map(INDEFINITE_NAN, 16);
        e.mem.set_u64(INDEFINITE_NAN, 0xc000_0000_0000_0000);
        e.mem.set_u16(INDEFINITE_NAN + 8, 0xffff);
    }

    fn sqrt_body(e: &mut Engine, zf: bool, x: f64) -> f64 {
        let w = words(x);
        e.call(0x00ec_605d, &args![zf, x, w[0], w[1]]).f64()
    }

    #[test]
    fn the_sqrt_body_takes_the_square_root_of_a_positive_value() {
        let mut e = Engine::new();
        sqrt_setup(&mut e, 1);
        assert_eq!(sqrt_body(&mut e, false, 6.25), 2.5);
        assert_eq!(sqrt_body(&mut e, false, 0.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(
            sqrt_body(&mut e, false, f64::from_bits(1)),
            f64::from_bits(1).sqrt(),
            "a denormal"
        );
        // A positive infinity (ZF set, no fraction, sign clear) is returned unchanged.
        assert_eq!(sqrt_body(&mut e, true, f64::INFINITY), f64::INFINITY);
    }

    #[test]
    fn the_sqrt_body_keeps_minus_zero_and_flags_other_negatives_as_domain_errors() {
        let mut e = Engine::new();
        sqrt_setup(&mut e, 1);
        assert_eq!(
            sqrt_body(&mut e, false, -0.0).to_bits(),
            (-0.0f64).to_bits()
        );
        for x in [-1.0f64, -4.0e-320, -1.0e300] {
            assert_eq!(
                sqrt_body(&mut e, false, x).to_bits(),
                0xfff8_0000_0000_0000,
                "{x:e}: the indefinite NaN"
            );
        }
        assert_eq!(
            sqrt_body(&mut e, true, f64::NEG_INFINITY).to_bits(),
            0xfff8_0000_0000_0000
        );
    }

    #[test]
    fn the_sqrt_body_asks_the_nan_helper_for_the_type_and_value_of_a_nan() {
        let mut e = Engine::new();
        sqrt_setup(&mut e, 1);
        let log = Log::default();
        let seen = log.clone();
        e.register_double(NAN_TYPE_HELPER, move |_, a| {
            seen.borrow_mut().push((NAN_TYPE_HELPER, a.to_vec()));
            Ret {
                eax: 1,
                st0: 3.5,
                ..Ret::default()
            }
        });
        let nan = f64::from_bits(0x7ff0_0000_0000_0007);
        // With the error switch set, the helper's ST0 is the result.
        assert_eq!(sqrt_body(&mut e, true, nan), 3.5);
        let w = words(nan);
        let st0_words = words(nan);
        assert_eq!(
            calls_to(&log, NAN_TYPE_HELPER),
            vec![vec![w[1], st0_words[0], st0_words[1]]],
            "EAX is the high word, then ST0"
        );
    }

    #[test]
    fn the_sqrt_body_ends_in_the_exit_or_the_error_handler_without_the_error_switch() {
        let mut e = Engine::new();
        sqrt_setup(&mut e, 0);
        let log = Log::default();
        recorder(&mut e, &log, MATH_EXIT, st0(11.0));
        recorder(&mut e, &log, ONE_ARG_ERROR_HANDLING, st0(13.0));
        // A result: the exit (operation 5, the name "sqrt", ST0, the saved control word, the
        // return address slot, the double).
        assert_eq!(sqrt_body(&mut e, false, 9.0), 11.0);
        let w = words(9.0);
        let r = words(3.0);
        assert_eq!(
            calls_to(&log, MATH_EXIT),
            vec![vec![5, SQRT_NAME, r[0], r[1], 0x027f, 0, w[0], w[1]]]
        );
        // A domain error: EAX = 1, the indefinite NaN in ST0.
        assert_eq!(sqrt_body(&mut e, false, -9.0), 13.0);
        let w = words(-9.0);
        let nan = words(f64::from_bits(0xfff8_0000_0000_0000));
        assert_eq!(
            calls_to(&log, ONE_ARG_ERROR_HANDLING),
            vec![vec![1, SQRT_NAME, 5, nan[0], nan[1], 0x027f, 0, w[0], w[1]]]
        );
    }

    #[test]
    fn the_sqrt_body_loads_a_masked_control_word_and_restores_the_old_one() {
        let mut e = Engine::new();
        sqrt_setup(&mut e, 1);
        let log = Log::default();
        let seen = log.clone();
        e.register_double(SET_CONTROL_WORD_HELPER, move |e, a| {
            seen.borrow_mut()
                .push((SET_CONTROL_WORD_HELPER, a.to_vec()));
            // The real helper loads (old & 0x300) | 0x7f.
            set_x87_control_word(e, ((a[0] & 0x300) | 0x7f) as u16);
            Ret::default()
        });
        ctrlfp(&mut e, 0x0b7f, 0xffff);
        assert_eq!(sqrt_body(&mut e, false, 16.0), 4.0);
        assert_eq!(calls_to(&log, SET_CONTROL_WORD_HELPER), vec![vec![0x0b7f]]);
        assert_eq!(x87_control_word(&mut e), 0x0b7f, "restored on the way out");
        // With the usual control word nothing is loaded.
        ctrlfp(&mut e, 0x027f, 0xffff);
        log.borrow_mut().clear();
        assert_eq!(sqrt_body(&mut e, false, 16.0), 4.0);
        assert!(calls_to(&log, SET_CONTROL_WORD_HELPER).is_empty());
        // The special path (ZF set) does not load it either.
        ctrlfp(&mut e, 0x037f, 0xffff);
        assert_eq!(sqrt_body(&mut e, true, f64::INFINITY), f64::INFINITY);
        assert!(calls_to(&log, SET_CONTROL_WORD_HELPER).is_empty());
        assert_eq!(x87_control_word(&mut e), 0x037f);
    }
}

#[cfg(test)]
mod strtol_tests {
    use super::*;

    /// The C locale's ctype bits for `c` (upper 1, lower 2, digit 4,
    /// space 8, hex 0x80).
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

    const HANDLER: u32 = 0x0077_0000;

    /// An engine whose `_LocaleUpdate` double fills the object with a locale
    /// info block (`mb_cur_max` at +0xAC, the C ctype table at +0xC8), a
    /// thread block (`_ownlocale` 3 at +0x70) and the "updated" byte
    /// `updated`; `errno` falls back to its static slot and the invalid
    /// parameter handler is a double. Returns the engine and the thread
    /// block.
    fn engine(mb_cur_max: u32, updated: u8) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011c_1000, 0x2000);
        let table = e.mem.alloc(0x200);
        for c in 0..=255u8 {
            e.mem.set_u16(table + u32::from(c) * 2, c_type(c));
        }
        let info = e.mem.alloc(0x100);
        e.mem.set_u32(info + 0xac, mb_cur_max);
        e.mem.set_u32(info + 0xc8, table);
        let thread = e.mem.alloc(0x80);
        e.mem.set_u32(thread + 0x70, 3);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u32(a[0] + 4, 0);
            e.mem.set_u32(a[0] + 8, thread);
            e.mem.set_u8(a[0] + 0xc, updated);
            a[0].into_ret()
        });
        e.register(ISCTYPE_L, |e, a| {
            let info = e.mem.u32(a[2]);
            let table = e.mem.u32(info + 0xc8);
            (u32::from(e.mem.u16(table + a[0] * 2)) & a[1]).into_ret()
        });
        e.register(GETPTD_NOEXIT, |_, _| 0u32.into_ret());
        e.register(DECODE_POINTER, |_, _| HANDLER.into_ret());
        e.register(HANDLER, |_, _| 0u32.into_ret());
        (e, thread)
    }

    /// `strtol(text, &end, base)`: the value, how many bytes it read and
    /// errno (0 when unset).
    fn parse(e: &mut Engine, text: &[u8], base: i32) -> (i32, u32, u32) {
        let s = e.mem.alloc(64);
        e.mem.write(s, text);
        e.mem.set_u8(s + text.len() as u32, 0);
        let end = e.mem.alloc(4);
        e.mem.set_u32(end, 0xdead_beef);
        e.mem.set_u32(ERRNO_FALLBACK, 0);
        let value = e.call(0x00ec_8d08, &[s, end, base as u32]).u32() as i32;
        (value, e.mem.u32(end) - s, e.mem.u32(ERRNO_FALLBACK))
    }

    #[test]
    fn strtol_reads_bases_signs_prefixes_and_limits() {
        let (mut e, thread) = engine(1, 0);
        e.call_log = Some(vec![]);
        let mut p = |text: &[u8], base| parse(&mut e, text, base);
        assert_eq!(p(b"1F", 16), (0x1f, 2, 0));
        assert_eq!(p(b"0x1fz", 16), (0x1f, 4, 0));
        assert_eq!(p(b"  -42abc", 10), (-42, 5, 0));
        assert_eq!(p(b"\t+017", 0), (0o17, 5, 0));
        assert_eq!(p(b"0X10", 0), (16, 4, 0));
        assert_eq!(p(b"129", 0), (129, 3, 0));
        assert_eq!(p(b"0", 0), (0, 1, 0));
        assert_eq!(p(b"zz", 36), (35 * 36 + 35, 2, 0));
        assert_eq!(p(b"102", 2), (2, 2, 0));
        assert_eq!(p(b"7fffffff", 16), (i32::MAX, 8, 0));
        assert_eq!(p(b"-80000000", 16), (i32::MIN, 9, 0));
        // Out of range: ERANGE, the limit, and every digit still read.
        assert_eq!(p(b"80000000", 16), (i32::MAX, 8, 0x22));
        assert_eq!(p(b"-80000001", 16), (i32::MIN, 9, 0x22));
        assert_eq!(p(b"123456789012", 10), (i32::MAX, 12, 0x22));
        // No digits: 0 and the end at the start (also after "0x").
        assert_eq!(p(b"", 16), (0, 0, 0));
        assert_eq!(p(b" -x", 10), (0, 0, 0));
        assert_eq!(p(b"0x", 16), (0, 0, 0));
        // A bad base: EINVAL, 0, the end at the start.
        assert_eq!(p(b"12", 1), (0, 0, 0x16));
        assert_eq!(p(b"12", 37), (0, 0, 0x16));
        assert_eq!(p(b"12", -1), (0, 0, 0x16));
        assert_eq!(e.mem.u32(thread + 0x70), 3);
        let locales: Vec<u32> = e
            .call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == LOCALE_UPDATE)
            .map(|(_, w)| w[1])
            .collect();
        assert!(locales.iter().all(|l| *l == INITIAL_LOCALE));
    }

    #[test]
    fn strtoxl_paths_through_the_locale_and_flags() {
        // A multibyte locale asks __isctype_l; an updated locale is released;
        // a changed locale passes null.
        let (mut e, thread) = engine(2, 1);
        e.mem.set_u32(LOCALE_CHANGED, 1);
        e.call_log = Some(vec![]);
        assert_eq!(parse(&mut e, b"  77", 8), (0o77, 4, 0));
        let log = e.call_log.take().unwrap();
        let isctype = log.iter().filter(|(a, _)| *a == ISCTYPE_L).count();
        assert_eq!(isctype, 3);
        assert_eq!(
            log.iter().find(|(a, _)| *a == LOCALE_UPDATE).unwrap().1[1],
            0
        );
        assert_eq!(e.mem.u32(thread + 0x70), 1);
        // Unsigned (flag 1): no signed limit; overflow gives 0xFFFFFFFF.
        let s = e.mem.alloc(16);
        e.mem.write(s, b"ffffffff\0");
        assert_eq!(e.call(0x00ec_8ad9, &[0, s, 0, 16, 1]).u32(), u32::MAX);
        e.mem.write(s, b"100000000\0");
        e.mem.set_u32(ERRNO_FALLBACK, 0);
        assert_eq!(e.call(0x00ec_8ad9, &[0, s, 0, 16, 1]).u32(), u32::MAX);
        assert_eq!(e.mem.u32(ERRNO_FALLBACK), 0x22);
        // A null text: EINVAL, the invalid parameter handler, 0.
        e.call_log = Some(vec![]);
        let end = e.mem.alloc(4);
        assert_eq!(e.call(0x00ec_8ad9, &[0, 0, end, 10, 0]).u32(), 0);
        assert_eq!(e.mem.u32(end), 0);
        let handler = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == HANDLER)
            .count();
        assert_eq!(handler, 1);
        // A null end pointer is allowed.
        e.mem.write(s, b"-9\0");
        assert_eq!(e.call(0x00ec_8d08, &[s, 0, 10]).u32() as i32, -9);
    }
}

#[cfg(test)]
mod thread_data_tests {
    use super::*;

    const FLS_GET_VALUE: u32 = 0x0077_0000;
    const FLS_SET_VALUE: u32 = 0x0077_0010;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An engine with the platform's imports at the slots these functions
    /// call, the CRT globals' pages mapped and the thread block `block`
    /// (0: none) behind the FLS doubles; `FlsSetValue` succeeds when
    /// `store_ok`.
    fn engine(block: u32, store_ok: bool) -> Engine {
        let mut e = Engine::new();
        let table = crate::units::platform::imports();
        for (slot, name) in [
            (IMPORT_GET_LAST_ERROR, "GetLastError"),
            (IMPORT_SET_LAST_ERROR, "SetLastError"),
            (IMPORT_GET_CURRENT_THREAD_ID, "GetCurrentThreadId"),
            (IMPORT_INTERLOCKED_INCREMENT, "InterlockedIncrement"),
            (IMPORT_INTERLOCKED_DECREMENT, "InterlockedDecrement"),
        ] {
            let (_, _, f) = table.iter().find(|(_, n, _)| *n == name).unwrap();
            e.register(slot, *f);
        }
        e.map(0x011c_1000, 0x2000);
        e.map(0x0126_f000, 0x1000);
        e.mem.set_u32(FLS_INDEX, 3);
        e.register(SET_FLSGETVALUE, |_, _| FLS_GET_VALUE.into_ret());
        e.register_double(FLS_GET_VALUE, move |_, a| {
            assert_eq!(a, [3]);
            block.into_ret()
        });
        e.register(DECODE_POINTER, |_, _| FLS_SET_VALUE.into_ret());
        e.register_double(FLS_SET_VALUE, move |_, _| u32::from(store_ok).into_ret());
        e.register(INIT_PTD, |_, _| 0u32.into_ret());
        e.register(AMSG_EXIT, |_, _| 0u32.into_ret());
        e.register(CRT_LOCK, |_, _| 0u32.into_ret());
        e.register(CRT_UNLOCK, |_, _| 0u32.into_ret());
        e
    }

    #[test]
    fn getptd_returns_the_thread_block_and_keeps_the_last_error() {
        let mut e = engine(0x0055_0000, true);
        e.call(IMPORT_SET_LAST_ERROR, &[7]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_11de, &[]).u32(), 0x0055_0000);
        assert_eq!(calls_to(&e, IMPORT_SET_LAST_ERROR), vec![vec![7]]);
        assert!(calls_to(&e, FLS_SET_VALUE).is_empty());
        assert!(calls_to(&e, AMSG_EXIT).is_empty());
    }

    #[test]
    fn getptd_noexit_makes_a_block_for_a_new_thread() {
        let mut e = engine(0, true);
        e.call_log = Some(vec![]);
        let block = e.call(0x00ed_1165, &[]).u32();
        assert_ne!(block, 0);
        assert_eq!(calls_to(&e, FLS_SET_VALUE), vec![vec![3, block]]);
        assert_eq!(calls_to(&e, INIT_PTD), vec![vec![block, 0]]);
        assert_eq!(e.mem.u32(block), crate::units::platform::MAIN_THREAD_ID);
        assert_eq!(e.mem.u32(block + 4), u32::MAX);
        assert!(e.mem.bytes(block + 8, 0x20c).iter().all(|b| *b == 0));
        // The store fails: the block is freed, 0, and _getptd ends the program.
        let mut e = engine(0, false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_11de, &[]).u32(), 0);
        assert!(calls_to(&e, INIT_PTD).is_empty());
        assert_eq!(calls_to(&e, AMSG_EXIT), vec![vec![0x10]]);
    }

    /// A thread block with `_ownlocale` `own` and the given locale and
    /// multibyte info.
    fn thread(e: &mut Engine, own: u32, info: u32, mbc: u32) -> u32 {
        let block = e.mem.alloc(0x214);
        e.mem.set_u32(block + 0x70, own);
        e.mem.set_u32(block + 0x6c, info);
        e.mem.set_u32(block + 0x68, mbc);
        block
    }

    #[test]
    fn locale_update_copies_a_locale_or_takes_the_threads() {
        let mut e = engine(0, true);
        let locale = e.mem.alloc(8);
        e.mem.set_u32(locale, 0x11);
        e.mem.set_u32(locale + 4, 0x22);
        let this = e.mem.alloc(0x10);
        e.mem.set_u8(this + 0xc, 9);
        assert_eq!(e.call(0x00ec_660e, &[this, locale]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x11);
        assert_eq!(e.mem.u32(this + 4), 0x22);
        assert_eq!(e.mem.u8(this + 0xc), 0);

        // The thread's globals: nothing to update; bit 1 set and noted.
        let mut e = engine(0, true);
        e.mem.set_u32(GLOBAL_LOCALE_INFO, 0x100);
        e.mem.set_u32(GLOBAL_MBC_INFO, 0x200);
        let block = thread(&mut e, 0, 0x100, 0x200);
        e.register_double(FLS_GET_VALUE, move |_, _| block.into_ret());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00ec_660e, &[this, 0]);
        assert_eq!(e.mem.u32(this), 0x100);
        assert_eq!(e.mem.u32(this + 4), 0x200);
        assert_eq!(e.mem.u32(this + 8), block);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert_eq!(e.mem.u32(block + 0x70), 2);
        assert!(calls_to(&e, CRT_LOCK).is_empty());
        // Bit 1 already set: not noted again.
        e.call(0x00ec_660e, &[this, 0]);
        assert_eq!(e.mem.u8(this + 0xc), 0);
    }

    #[test]
    fn locale_update_refreshes_stale_thread_locales() {
        let mut e = engine(0, true);
        e.mem.set_u32(GLOBAL_LOCALE_INFO, 0x100);
        let global_mbc = e.mem.alloc(4);
        e.mem.set_u32(global_mbc, 1);
        e.mem.set_u32(GLOBAL_MBC_INFO, global_mbc);
        let old_mbc = e.mem.alloc(4);
        e.mem.set_u32(old_mbc, 1);
        let block = thread(&mut e, 0, 0x300, old_mbc);
        e.register_double(FLS_GET_VALUE, move |_, _| block.into_ret());
        e.register(UPDATE_TLOCINFO_EX_NOLOCK, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            a[1].into_ret()
        });
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00ec_660e, &[this, 0]);
        assert_eq!(
            calls_to(&e, UPDATE_TLOCINFO_EX_NOLOCK),
            vec![vec![block + 0x6c, 0x100]]
        );
        assert_eq!(e.mem.u32(this), 0x100);
        // The old multibyte info's count fell to zero; the global's rose.
        assert_eq!(e.mem.u32(this + 4), global_mbc);
        assert_eq!(e.mem.u32(block + 0x68), global_mbc);
        assert_eq!(e.mem.u32(global_mbc), 2);
        assert_eq!(
            calls_to(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![old_mbc]]
        );
        assert_eq!(calls_to(&e, CRT_LOCK), vec![vec![0xc], vec![0xd]]);
        assert_eq!(calls_to(&e, CRT_UNLOCK), vec![vec![0xc], vec![0xd]]);
    }

    #[test]
    fn update_functions_keep_a_threads_own_locale_and_stop_on_null() {
        let mut e = engine(0, true);
        e.mem.set_u32(GLOBAL_LOCALE_STATUS, 1);
        let block = thread(&mut e, 1, 0x400, 0x500);
        e.register_double(FLS_GET_VALUE, move |_, _| block.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_f742, &[]).u32(), 0x400);
        assert_eq!(e.call(0x00ed_3cdf, &[]).u32(), 0x500);
        assert!(calls_to(&e, CRT_LOCK).is_empty());
        assert_eq!(calls_to(&e, SET_FLSGETVALUE).len(), 3);
        // Not kept, and the update gives null: _amsg_exit(0x20).
        e.mem.set_u32(GLOBAL_LOCALE_STATUS, 0);
        e.register(UPDATE_TLOCINFO_EX_NOLOCK, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_f742, &[]).u32(), 0);
        assert_eq!(calls_to(&e, AMSG_EXIT), vec![vec![0x20]]);
        // The multibyte info equal to the global: kept as it is.
        e.mem.set_u32(GLOBAL_MBC_INFO, 0x500);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_3cdf, &[]).u32(), 0x500);
        assert!(calls_to(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        // The static initial info is not freed when its count reaches 0.
        e.mem.set_u32(INITIAL_MBC_INFO, 1);
        e.mem.set_u32(block + 0x68, INITIAL_MBC_INFO);
        let global = e.mem.alloc(4);
        e.mem.set_u32(GLOBAL_MBC_INFO, global);
        assert_eq!(e.call(0x00ed_3cdf, &[]).u32(), global);
        assert_eq!(e.mem.u32(INITIAL_MBC_INFO), 0);
    }

    /// A locale object (`_locale_tstruct`) whose locale info has a ctype
    /// table where `A` is upper case (1) and 0x81 a lead byte (0x8000), and
    /// code page 932 at +4, locale id 0x411 at +0x14.
    fn locale(e: &mut Engine) -> u32 {
        let table = e.mem.alloc(0x202);
        e.mem.set_u16(table + 2 * 0x41, 1);
        e.mem.set_u16(table + 2 * 0x81, 0x8000);
        let info = e.mem.alloc(0x100);
        e.mem.set_u32(info + 0xc8, table);
        e.mem.set_u32(info + 4, 932);
        e.mem.set_u32(info + 0x14, 0x411);
        let locale = e.mem.alloc(8);
        e.mem.set_u32(locale, info);
        locale
    }

    /// The byte strings the string-type double received.
    type Seen = std::rc::Rc<std::cell::RefCell<Vec<Vec<u8>>>>;

    #[test]
    fn character_types_read_the_table_or_ask_the_string_types() {
        let mut e = engine(0, true);
        let l = locale(&mut e);
        assert_eq!(e.call(0x00ed_419c, &[0x181, l]).u32(), 0x8000);
        assert_eq!(e.call(0x00ed_419c, &[0x41, l]).u32(), 0);
        assert_eq!(e.call(0x00ed_41e7, &[0x41, 1, l]).u32(), 1);
        assert_eq!(e.call(0x00ed_41e7, &[0x41, 2, l]).u32(), 0);
        // Above 255: the string types of one or two bytes.
        let seen: Seen = Default::default();
        let record = seen.clone();
        e.register_double(GET_STRING_TYPE_A_STAT, move |e, a| {
            assert_eq!(a[1], 1);
            assert_eq!((a[5], a[6], a[7]), (932, 0x411, 1));
            record.borrow_mut().push(e.mem.bytes(a[2], a[3]));
            e.mem.set_u16(a[4], 0x0104);
            u32::from(a[3] == 2).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_41e7, &[0x8142, 0xffff, l]).u32(), 0x0104);
        assert_eq!(e.call(0x00ed_41e7, &[0x4142, 0xffff, l]).u32(), 0);
        assert_eq!(*seen.borrow(), vec![vec![0x81, 0x42], vec![0x42]]);
        assert_eq!(calls_to(&e, GET_STRING_TYPE_A_STAT).len(), 2);
        // __crtGetStringTypeA passes the locale object and its seven words.
        let text = e.mem.alloc(4);
        e.mem.write(text, &[0x81, 0x42]);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00ee_13a3, &[l, 1, text, 2, out, 932, 0x411, 1])
            .u32();
        assert_eq!(result, 1);
        let call = &calls_to(&e, GET_STRING_TYPE_A_STAT)[0];
        assert_eq!(&call[1..], &[1, text, 2, out, 932, 0x411, 1]);
    }
}

#[cfg(test)]
mod thread_start_up_tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    const MODULE: u32 = 0x7700_0000;
    const FLS_ALLOC: u32 = 0x0077_1000;
    const FLS_GET: u32 = 0x0077_1010;
    const FLS_SET: u32 = 0x0077_1020;
    const FLS_FREE: u32 = 0x0077_1030;
    const ENCODE: u32 = 0x0077_1040;
    const DECODE: u32 = 0x0077_1050;
    const EXIT: u32 = 0x0077_1060;
    const KEY: u32 = 0x5a5a_5a5a;

    /// Slot values of the doubles' TLS and FLS.
    type Slots = Rc<RefCell<HashMap<u32, u32>>>;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An engine with doubles for the imports `__mtinit`'s chain calls:
    /// `KERNEL32.DLL` at `MODULE` whose `GetProcAddress` knows the FLS
    /// functions (when `fls`) and `EncodePointer`/`DecodePointer` (XOR with
    /// `KEY`); TLS and FLS slots kept in maps; critical sections that
    /// initialize when `sections_ok`. Lock entries 0xA, 0xC and 0xD are of
    /// kind 1. Returns the engine and the TLS and FLS slots.
    fn engine(fls: bool, sections_ok: bool) -> (Engine, Slots, Slots) {
        let mut e = Engine::new();
        for page in [
            0x00fd_f000,
            0x0101_1000,
            0x011c_1000,
            0x011c_2000,
            0x0126_f000,
            0x0127_0000,
        ] {
            e.map(page, 0x1000);
        }
        let table = crate::units::platform::imports();
        for (slot, name) in [
            (IMPORT_GET_LAST_ERROR, "GetLastError"),
            (IMPORT_SET_LAST_ERROR, "SetLastError"),
            (IMPORT_GET_CURRENT_THREAD_ID, "GetCurrentThreadId"),
            (IMPORT_INTERLOCKED_INCREMENT, "InterlockedIncrement"),
        ] {
            let (_, _, f) = table.iter().find(|(_, n, _)| *n == name).unwrap();
            e.register(slot, *f);
        }
        e.register(IMPORT_GET_MODULE_HANDLE_W, |_, _| MODULE.into_ret());
        e.register_double(IMPORT_GET_PROC_ADDRESS, move |_, a| {
            assert_eq!(a[0], MODULE);
            let found = match a[1] {
                FLS_ALLOC_NAME => FLS_ALLOC,
                FLS_GET_VALUE_NAME => FLS_GET,
                FLS_SET_VALUE_NAME => FLS_SET,
                FLS_FREE_NAME => FLS_FREE,
                ENCODE_POINTER_NAME => return ENCODE.into_ret(),
                DECODE_POINTER_NAME => return DECODE.into_ret(),
                other => panic!("GetProcAddress({other:08x})"),
            };
            (if fls { found } else { 0 }).into_ret()
        });
        e.register(ENCODE, |_, a| (a[0] ^ KEY).into_ret());
        e.register(DECODE, |_, a| (a[0] ^ KEY).into_ret());
        let tls: Slots = Rc::default();
        let fls_slots: Slots = Rc::default();
        e.register(IMPORT_TLS_ALLOC, |_, _| 5u32.into_ret());
        let slots = tls.clone();
        e.register_double(IMPORT_TLS_GET_VALUE, move |_, a| {
            slots.borrow().get(&a[0]).copied().unwrap_or(0).into_ret()
        });
        let slots = tls.clone();
        e.register_double(IMPORT_TLS_SET_VALUE, move |_, a| {
            slots.borrow_mut().insert(a[0], a[1]);
            1u32.into_ret()
        });
        e.register(IMPORT_TLS_FREE, |_, _| 1u32.into_ret());
        e.register(FLS_ALLOC, |_, a| {
            assert_eq!(a, [FREE_FLS]);
            7u32.into_ret()
        });
        let slots = fls_slots.clone();
        e.register_double(FLS_GET, move |_, a| {
            slots.borrow().get(&a[0]).copied().unwrap_or(0).into_ret()
        });
        let slots = fls_slots.clone();
        e.register_double(FLS_SET, move |_, a| {
            slots.borrow_mut().insert(a[0], a[1]);
            1u32.into_ret()
        });
        e.register(FLS_FREE, |_, _| 1u32.into_ret());
        e.register(INITP_00EC7AF9, |_, _| 0u32.into_ret());
        e.register_double(IMPORT_INIT_CRITICAL_SECTION_SPIN, move |_, _| {
            u32::from(sections_ok).into_ret()
        });
        e.register(IMPORT_ENTER_CRITICAL_SECTION, |_, _| 0u32.into_ret());
        e.register(IMPORT_LEAVE_CRITICAL_SECTION, |_, _| 0u32.into_ret());
        e.register(ADD_LOCALE_REF, |_, _| 0u32.into_ret());
        e.register(MT_DELETE_LOCKS, |_, _| 0u32.into_ret());
        e.register(IMPORT_SLEEP, |_, _| 0u32.into_ret());
        e.register(FF_MSGBANNER, |_, _| 0u32.into_ret());
        e.register(NMSG_WRITE, |_, _| 0u32.into_ret());
        e.register(EXIT, |_, _| 0u32.into_ret());
        e.register(GETPTD_NOEXIT, |_, _| 0u32.into_ret());
        for number in [0xa, 0xc, 0xd] {
            e.mem.set_u32(LOCK_TABLE + number * 8 + 4, 1);
        }
        e.mem.set_u32(LOCKS_READY, 1);
        e.mem.set_u32(FLS_INDEX, u32::MAX);
        e.mem.set_u32(TLS_INDEX, u32::MAX);
        (e, tls, fls_slots)
    }

    #[test]
    fn mtinit_sets_up_fls_locks_and_the_first_thread_block() {
        let (mut e, tls, fls) = engine(true, true);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_1327, &[]).u32(), 1);
        assert_eq!(e.mem.u32(TLS_INDEX), 5);
        assert_eq!(tls.borrow()[&5], FLS_GET);
        assert_eq!(e.mem.u32(FLS_INDEX), 7);
        // The four FLS pointers are kept encoded.
        assert_eq!(e.mem.u32(FLS_ALLOC_POINTER), FLS_ALLOC ^ KEY);
        assert_eq!(e.mem.u32(FLS_GET_VALUE_POINTER), FLS_GET ^ KEY);
        assert_eq!(e.mem.u32(FLS_SET_VALUE_POINTER), FLS_SET ^ KEY);
        assert_eq!(e.mem.u32(FLS_FREE_POINTER), FLS_FREE ^ KEY);
        // __init_pointers: the encoded null everywhere, the exit routine.
        assert_eq!(e.mem.u32(0x0127_072c), KEY);
        assert_eq!(e.mem.u32(0x0127_071c), KEY);
        assert_eq!(e.mem.u32(0x0126_fe10), 0x00ed_14b4 ^ KEY);
        assert_eq!(e.mem.u32(ENCODED_EXIT), 0x00ec_cedb ^ KEY);
        assert_eq!(calls_to(&e, INITP_00EC7AF9), vec![vec![KEY]]);
        // The locks of kind 1 got the static sections in order.
        assert_eq!(e.mem.u32(LOCK_TABLE + 0xa * 8), STATIC_LOCKS);
        assert_eq!(e.mem.u32(LOCK_TABLE + 0xc * 8), STATIC_LOCKS + 0x18);
        assert_eq!(e.mem.u32(LOCK_TABLE + 0xd * 8), STATIC_LOCKS + 0x30);
        // The thread block.
        let block = fls.borrow()[&7];
        assert_eq!(e.mem.u32(block), crate::units::platform::MAIN_THREAD_ID);
        assert_eq!(e.mem.u32(block + 4), u32::MAX);
        assert_eq!(e.mem.u32(block + 0x5c), 0x010f_dca0);
        assert_eq!(e.mem.u32(block + 0x14), 1);
        assert_eq!(e.mem.u32(block + 0x1f8), ENCODE);
        assert_eq!(e.mem.u32(block + 0x1fc), DECODE);
        assert_eq!(e.mem.u32(block + 0x70), 1);
        assert_eq!(e.mem.u8(block + 0xc8), 0x43);
        assert_eq!(e.mem.u8(block + 0x14b), 0x43);
        assert_eq!(e.mem.u32(block + 0x68), INITIAL_MBC_INFO);
        assert_eq!(
            calls_to(&e, IMPORT_ENTER_CRITICAL_SECTION),
            vec![vec![STATIC_LOCKS + 0x30], vec![STATIC_LOCKS + 0x18]]
        );
        assert_eq!(
            calls_to(&e, IMPORT_LEAVE_CRITICAL_SECTION),
            vec![vec![STATIC_LOCKS + 0x30], vec![STATIC_LOCKS + 0x18]]
        );
        assert!(calls_to(&e, MT_DELETE_LOCKS).is_empty());
        // Afterwards the pointers decode through the thread block.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_0f92, &[0x1234 ^ KEY]).u32(), 0x1234);
        assert!(calls_to(&e, IMPORT_GET_PROC_ADDRESS).is_empty());
        assert_eq!(e.call(0x00ed_100d, &[]).u32(), FLS_GET);
    }

    #[test]
    fn mtinit_falls_back_to_tls_and_stops_on_failures() {
        let (mut e, tls, _) = engine(false, true);
        e.mem.set_u32(IMPORT_TLS_GET_VALUE, 0x0077_2044);
        e.mem.set_u32(IMPORT_TLS_SET_VALUE, 0x0077_204c);
        e.mem.set_u32(IMPORT_TLS_FREE, 0x0077_2050);
        // The FLS stand-ins: __crtTlsAlloc, then the import table's words.
        e.register(0x0077_204c, |_, _| 1u32.into_ret());
        e.register(0x0077_2044, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_1327, &[]).u32(), 1);
        assert_eq!(e.mem.u32(FLS_ALLOC_POINTER), CRT_TLS_ALLOC ^ KEY);
        assert_eq!(e.mem.u32(FLS_GET_VALUE_POINTER), 0x0077_2044 ^ KEY);
        assert_eq!(e.mem.u32(FLS_SET_VALUE_POINTER), 0x0077_204c ^ KEY);
        assert_eq!(e.mem.u32(FLS_FREE_POINTER), 0x0077_2050 ^ KEY);
        assert_eq!(tls.borrow()[&5], 0x0077_2044);
        // __crtTlsAlloc gave the FLS slot through TlsAlloc.
        assert_eq!(e.mem.u32(FLS_INDEX), 5);
        assert_eq!(calls_to(&e, 0x0077_204c).len(), 1);

        // TlsAlloc fails: 0 without __mtterm.
        let (mut e, _, _) = engine(true, true);
        e.register(IMPORT_TLS_ALLOC, |_, _| u32::MAX.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_1327, &[]).u32(), 0);
        assert!(calls_to(&e, MT_DELETE_LOCKS).is_empty());

        // A lock that cannot be initialized: __mtterm frees the TLS slot.
        let (mut e, _, _) = engine(true, false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_1327, &[]).u32(), 0);
        assert_eq!(e.mem.u32(LOCK_TABLE + 0xa * 8), 0);
        assert_eq!(calls_to(&e, IMPORT_TLS_FREE), vec![vec![5]]);
        assert_eq!(e.mem.u32(TLS_INDEX), u32::MAX);
        assert_eq!(calls_to(&e, MT_DELETE_LOCKS).len(), 1);
    }

    #[test]
    fn mtterm_frees_the_slots_that_are_set() {
        let (mut e, _, _) = engine(true, true);
        e.mem.set_u32(FLS_INDEX, 7);
        e.mem.set_u32(TLS_INDEX, 5);
        e.mem.set_u32(FLS_FREE_POINTER, FLS_FREE ^ KEY);
        e.call_log = Some(vec![]);
        e.call(0x00ed_1041, &[]);
        assert_eq!(calls_to(&e, FLS_FREE), vec![vec![7]]);
        assert_eq!(calls_to(&e, IMPORT_TLS_FREE), vec![vec![5]]);
        assert_eq!(e.mem.u32(FLS_INDEX), u32::MAX);
        assert_eq!(e.mem.u32(TLS_INDEX), u32::MAX);
        e.call_log = Some(vec![]);
        e.call(0x00ed_1041, &[]);
        assert!(calls_to(&e, FLS_FREE).is_empty());
        assert!(calls_to(&e, IMPORT_TLS_FREE).is_empty());
        assert_eq!(calls_to(&e, MT_DELETE_LOCKS).len(), 1);
    }

    #[test]
    fn locks_are_made_on_first_use_and_failures_exit() {
        let (mut e, _, _) = engine(true, true);
        // Lock 0xA (which __mtinitlocks sets up) guards the set-up.
        e.mem.set_u32(LOCK_TABLE + 0xa * 8, 0x0055_0100);
        e.call_log = Some(vec![]);
        e.call(0x00ed_df39, &[3]);
        let section = e.mem.u32(LOCK_TABLE + 3 * 8);
        assert_ne!(section, 0);
        assert_eq!(
            calls_to(&e, IMPORT_INIT_CRITICAL_SECTION_SPIN),
            vec![vec![section, 0xfa0]]
        );
        let lock_a = 0x0055_0100;
        assert_eq!(
            calls_to(&e, IMPORT_ENTER_CRITICAL_SECTION),
            vec![vec![lock_a], vec![section]]
        );
        e.call(0x00ed_de5f, &[3]);
        assert_eq!(
            calls_to(&e, IMPORT_LEAVE_CRITICAL_SECTION).last().unwrap(),
            &vec![section]
        );
        // Initialization fails: ENOMEM and _amsg_exit(0x11) through the
        // decoded exit routine.
        let (mut e, _, _) = engine(true, false);
        e.mem.set_u32(LOCK_TABLE + 0xa * 8, 0x0055_0000);
        e.mem.set_u32(ENCODED_EXIT, EXIT ^ KEY);
        e.call_log = Some(vec![]);
        e.call(0x00ed_df39, &[4]);
        assert_eq!(e.mem.u32(LOCK_TABLE + 4 * 8), 0);
        assert_eq!(e.mem.u32(ERRNO_FALLBACK), 0xc);
        assert_eq!(calls_to(&e, NMSG_WRITE), vec![vec![0x11]]);
        assert_eq!(calls_to(&e, EXIT), vec![vec![0xff]]);
        // Too early in start-up: banner, message 0x1E, exit 0xFF.
        let (mut e, _, _) = engine(true, true);
        e.mem.set_u32(LOCKS_READY, 0);
        e.mem.set_u32(LOCK_TABLE + 0xa * 8, 0x0055_0100);
        e.register(0x00ec_cca9, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00ed_de76, &[6]);
        assert_eq!(calls_to(&e, NMSG_WRITE), vec![vec![0x1e]]);
        assert_eq!(calls_to(&e, 0x00ec_cca9), vec![vec![0xff]]);
    }

    #[test]
    fn waiting_for_a_module_sleeps_longer_each_time() {
        let (mut e, _, _) = engine(true, true);
        e.register(IMPORT_GET_MODULE_HANDLE_W, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_cc25, &[KERNEL32_NAME]).u32(), 0);
        let sleeps = calls_to(&e, IMPORT_SLEEP);
        assert_eq!(sleeps.len(), 60);
        assert_eq!(sleeps[0], vec![1000]);
        assert_eq!(sleeps[59], vec![60000]);
        // Without the module the pointers pass unchanged.
        assert_eq!(e.call(0x00ed_0f17, &[0x99]).u32(), 0x99);
    }

    #[test]
    fn locale_info_update_swaps_references() {
        let (mut e, _, _) = engine(true, true);
        e.register(REMOVE_LOCALE_REF, |e, a| {
            e.mem.set_u32(a[0], 0);
            0u32.into_ret()
        });
        e.register(FREE_TLOCINFO, |_, _| 0u32.into_ret());
        let slot = e.mem.alloc(4);
        let old = e.mem.alloc(4);
        e.mem.set_u32(slot, old);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_f704, &[slot, 0x0066_0000]).u32(), 0x0066_0000);
        assert_eq!(e.mem.u32(slot), 0x0066_0000);
        assert_eq!(calls_to(&e, ADD_LOCALE_REF), vec![vec![0x0066_0000]]);
        assert_eq!(calls_to(&e, REMOVE_LOCALE_REF), vec![vec![old]]);
        assert_eq!(calls_to(&e, FREE_TLOCINFO), vec![vec![old]]);
        // The same info, or a null argument: nothing changes.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_f704, &[slot, 0x0066_0000]).u32(), 0x0066_0000);
        assert_eq!(e.call(0x00ec_f704, &[0, 0x0066_0000]).u32(), 0);
        assert_eq!(e.call(0x00ec_f704, &[slot, 0]).u32(), 0);
        assert!(calls_to(&e, ADD_LOCALE_REF).is_empty());
    }

    /// The locale object for the string types: code page 1252 at info +4,
    /// locale id 0x409 at +0x14.
    fn string_locale(e: &mut Engine) -> u32 {
        let info = e.mem.alloc(0x20);
        e.mem.set_u32(info + 4, 1252);
        e.mem.set_u32(info + 0x14, 0x409);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, info);
        object
    }

    #[test]
    fn string_types_go_through_the_wide_api_when_it_works() {
        let (mut e, _, _) = engine(true, true);
        let locale = string_locale(&mut e);
        e.register(IMPORT_GET_STRING_TYPE_W, |e, a| {
            if a[1] != EMPTY_WIDE_STRING {
                assert_eq!(e.mem.u16(a[1]), u16::from(b'h'));
                e.mem.set_u16(a[3], 0x0102);
            }
            1u32.into_ret()
        });
        e.register(IMPORT_MULTI_BYTE_TO_WIDE_CHAR, |e, a| {
            if a[4] != 0 {
                for i in 0..a[3] {
                    let c = e.mem.u8(a[2] + i);
                    e.mem.set_u16(a[4] + i * 2, u16::from(c));
                }
            }
            a[3].into_ret()
        });
        e.register(FREEA, |_, _| 0u32.into_ret());
        let text = e.mem.alloc(4);
        e.mem.write(text, b"hi");
        let types = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let args = [locale, 1, text, 2, types, 0, 0, 1];
        assert_eq!(e.call(0x00ee_11e9, &args).u32(), 1);
        assert_eq!(e.mem.u32(STRING_TYPE_MODE), 1);
        assert_eq!(e.mem.u16(types), 0x0102);
        let conversions = calls_to(&e, IMPORT_MULTI_BYTE_TO_WIDE_CHAR);
        assert_eq!(conversions[0], vec![1252, 9, text, 2, 0, 0]);
        assert_eq!(conversions[1][..4], [1252, 1, text, 2]);
        assert_eq!(calls_to(&e, FREEA).len(), 1);
        // A long text takes a _malloc'd buffer (marked 0xDDDD).
        let long = e.mem.alloc(0x300);
        e.mem.write(long, &[b'h'; 0x300]);
        e.register(MALLOC_BODY, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        let args = [locale, 1, long, 0x300, types, 0, 0, 0];
        assert_eq!(e.call(0x00ee_11e9, &args).u32(), 1);
        let block = calls_to(&e, MALLOC_BODY);
        assert_eq!(block, vec![vec![0x300 * 2 + 8]]);
        let freed = calls_to(&e, FREEA)[0][0];
        assert_eq!(e.mem.u32(freed - 8), 0xdddd);
        // A failed conversion gives 0.
        e.register(IMPORT_MULTI_BYTE_TO_WIDE_CHAR, |_, _| 0u32.into_ret());
        assert_eq!(
            e.call(0x00ee_11e9, &[locale, 1, text, 2, types, 0, 0, 0])
                .u32(),
            0
        );
    }

    #[test]
    fn string_types_fall_back_to_the_ansi_api() {
        let (mut e, _, _) = engine(true, true);
        let locale = string_locale(&mut e);
        e.register(IMPORT_GET_STRING_TYPE_W, |e, _| {
            e.call(IMPORT_SET_LAST_ERROR, &[0x78]);
            0u32.into_ret()
        });
        e.register(ANSI_CODE_PAGE, |_, a| {
            assert_eq!(a[0], 0x409);
            1250u32.into_ret()
        });
        e.register(CONVERT_CODE_PAGE, |e, a| {
            assert_eq!(a[..2], [1252, 1250]);
            e.mem.set_u32(a[3], 5);
            e.mem.alloc(8).into_ret()
        });
        e.register(IMPORT_GET_STRING_TYPE_A, |_, _| 7u32.into_ret());
        let types = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let args = [locale, 1, 0x0055_0000, 3, types, 0, 0, 0];
        assert_eq!(e.call(0x00ee_11e9, &args).u32(), 7);
        assert_eq!(e.mem.u32(STRING_TYPE_MODE), 2);
        let converted = calls_to(&e, CONVERT_CODE_PAGE);
        let ansi = calls_to(&e, IMPORT_GET_STRING_TYPE_A);
        assert_eq!(ansi.len(), 1);
        assert_eq!(ansi[0][0], 0x409);
        assert_eq!(ansi[0][3], 5);
        assert_ne!(ansi[0][2], 0x0055_0000);
        assert_eq!(converted.len(), 1);
        // The same code page: no conversion; an unknown locale: 0.
        e.call_log = Some(vec![]);
        let args = [locale, 1, 0x0055_0000, 3, types, 1250, 0, 0];
        assert_eq!(e.call(0x00ee_11e9, &args).u32(), 7);
        assert!(calls_to(&e, CONVERT_CODE_PAGE).is_empty());
        assert_eq!(calls_to(&e, IMPORT_GET_STRING_TYPE_A)[0][2], 0x0055_0000);
        e.register(ANSI_CODE_PAGE, |_, _| u32::MAX.into_ret());
        assert_eq!(e.call(0x00ee_11e9, &args).u32(), 0);
    }
}

#[cfg(test)]
mod runtime_support_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn freed(e: &Engine, block: u32) -> bool {
        e.mem.block_size(block).is_none()
    }

    /// An engine with the CRT's data pages mapped, the Interlocked imports
    /// at their slots and doubles for the section imports.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x00fd_f000,
            0x0107_2000,
            0x0109_5000,
            0x010f_3000,
            0x011c_1000,
            0x011c_2000,
            0x0126_f000,
            0x0127_0000,
        ] {
            e.map(page, 0x1000);
        }
        let table = crate::units::platform::imports();
        for (slot, name) in [
            (IMPORT_INTERLOCKED_INCREMENT, "InterlockedIncrement"),
            (IMPORT_INTERLOCKED_DECREMENT, "InterlockedDecrement"),
        ] {
            let (_, _, f) = table.iter().find(|(_, n, _)| *n == name).unwrap();
            e.register(slot, *f);
        }
        for slot in [
            IMPORT_DELETE_CRITICAL_SECTION,
            IMPORT_ENTER_CRITICAL_SECTION,
            IMPORT_LEAVE_CRITICAL_SECTION,
        ] {
            e.register(slot, |_, _| 0u32.into_ret());
        }
        e.register(GETPTD_NOEXIT, |_, _| 0u32.into_ret());
        e
    }

    #[test]
    fn mtdeletelocks_frees_dynamic_sections_and_deletes_static_ones() {
        let mut e = engine();
        let dynamic = e.mem.alloc(0x18);
        e.mem.set_u32(LOCK_TABLE + 3 * 8, dynamic);
        e.mem.set_u32(LOCK_TABLE + 5 * 8, STATIC_LOCKS);
        e.mem.set_u32(LOCK_TABLE + 5 * 8 + 4, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ed_de08, &[]);
        assert_eq!(
            calls_to(&e, IMPORT_DELETE_CRITICAL_SECTION),
            vec![vec![dynamic], vec![STATIC_LOCKS]]
        );
        assert!(freed(&e, dynamic));
        assert_eq!(e.mem.u32(LOCK_TABLE + 3 * 8), 0);
        assert_eq!(e.mem.u32(LOCK_TABLE + 5 * 8), STATIC_LOCKS);
    }

    /// The message table with 0xFC, 0xFF and 0x1E, their texts in the heap.
    fn messages(e: &mut Engine) -> [u32; 3] {
        let mut texts = [0; 3];
        for (i, (number, text)) in [
            (0xfcu32, &b"\r\n\0"[..]),
            (0xff, b"banner\0"),
            (0x1e, b"R6030\0"),
        ]
        .into_iter()
        .enumerate()
        {
            let s = e.mem.alloc(16);
            e.mem.write(s, text);
            e.mem.set_u32(RUNTIME_MESSAGES + i as u32 * 8, number);
            e.mem.set_u32(RUNTIME_MESSAGES + i as u32 * 8 + 4, s);
            texts[i] = s;
        }
        for i in 3..0x17 {
            e.mem.set_u32(RUNTIME_MESSAGES + i * 8, 0xdead);
        }
        texts
    }

    #[test]
    fn console_messages_go_to_standard_error() {
        let mut e = engine();
        let texts = messages(&mut e);
        e.register(SET_ERROR_MODE, |_, _| 1u32.into_ret());
        e.register(IMPORT_GET_STD_HANDLE, |_, a| {
            assert_eq!(a[0], (-12i32) as u32);
            0x30u32.into_ret()
        });
        e.register(IMPORT_WRITE_FILE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00ed_2ab7, &[]);
        let writes: Vec<(u32, u32, u32)> = calls_to(&e, IMPORT_WRITE_FILE)
            .iter()
            .map(|w| (w[0], w[1], w[2]))
            .collect();
        assert_eq!(writes, vec![(0x30, texts[0], 2), (0x30, texts[1], 6)]);
        // A number outside the table writes nothing.
        e.call_log = Some(vec![]);
        e.call(0x00ed_290c, &[0x1234]);
        assert!(calls_to(&e, SET_ERROR_MODE).is_empty());
        // No console and the default mode with a GUI app: no banner.
        e.register(SET_ERROR_MODE, |_, _| 0u32.into_ret());
        e.mem.set_u32(APP_TYPE, 2);
        e.call_log = Some(vec![]);
        e.call(0x00ed_2ab7, &[]);
        assert!(calls_to(&e, IMPORT_WRITE_FILE).is_empty());
    }

    #[test]
    fn window_messages_build_the_runtime_error_box() {
        let mut e = engine();
        messages(&mut e);
        for (at, text) in [
            (MESSAGE_HEADER, &b"Runtime Error!\n\nProgram: \0"[..]),
            (MESSAGE_UNKNOWN_PROGRAM, b"<program name unknown>\0"),
            (MESSAGE_ELLIPSIS, b"...\0"),
            (MESSAGE_BREAK, b"\n\n\0"),
            (MESSAGE_CAPTION, b"Microsoft Visual C++ Runtime Library\0"),
        ] {
            e.mem.write(at, text);
        }
        e.register(SET_ERROR_MODE, |_, _| 2u32.into_ret());
        let name = "C:\\Games\\Fallout New Vegas\\a rather long folder name\\FalloutNV.exe";
        e.register(IMPORT_GET_MODULE_FILE_NAME_A, |e, a| {
            let name = "C:\\Games\\Fallout New Vegas\\a rather long folder name\\FalloutNV.exe";
            e.mem.write(a[1], name.as_bytes());
            e.mem.set_u8(a[1] + name.len() as u32, 0);
            (name.len() as u32).into_ret()
        });
        let shown = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
        let record = shown.clone();
        e.register_double(CRT_MESSAGE_BOX_A, move |e, a| {
            assert_eq!(a[1], MESSAGE_CAPTION);
            assert_eq!(a[2], 0x12010);
            *record.borrow_mut() = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            1u32.into_ret()
        });
        e.call(0x00ed_290c, &[0x1e]);
        let kept = &name[..name.len() - 0x3b];
        assert_eq!(
            *shown.borrow(),
            format!("Runtime Error!\n\nProgram: {kept}...\n\nR6030")
        );
        // An unknown program name, and 0xFC shows nothing.
        e.register(IMPORT_GET_MODULE_FILE_NAME_A, |_, _| 0u32.into_ret());
        e.call(0x00ed_290c, &[0x1e]);
        assert_eq!(
            *shown.borrow(),
            "Runtime Error!\n\nProgram: <program name unknown>\n\nR6030"
        );
        shown.borrow_mut().clear();
        e.call(0x00ed_290c, &[0xfc]);
        assert!(shown.borrow().is_empty());
    }

    #[test]
    fn exit_quick_is_doexit_quick() {
        let mut e = engine();
        e.register(DOEXIT_ADDR, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00ec_cedb, &[3]);
        assert_eq!(calls_to(&e, DOEXIT_ADDR), vec![vec![3, 1, 0]]);
    }

    /// A locale info with every count at 1 (category 2 uses the C names and
    /// category 4 has no wide name); returns it and its count words.
    fn locale_info(e: &mut Engine) -> (u32, Vec<u32>) {
        let info = e.mem.alloc(0xd8);
        let mut counts = vec![info];
        e.mem.set_u32(info, 1);
        let count = |e: &mut Engine| {
            let c = e.mem.alloc(4);
            e.mem.set_u32(c, 1);
            c
        };
        for offset in [0xb0, 0xb8, 0xb4, 0xc0] {
            let c = count(e);
            e.mem.set_u32(info + offset, c);
            counts.push(c);
        }
        for category in 0..6u32 {
            let at = info + 0x50 + category * 0x10;
            e.mem
                .set_u32(at - 8, if category == 2 { C_LOCALE_NAMES } else { 0 });
            let name = count(e);
            e.mem.set_u32(at, name);
            e.mem.set_u32(at - 4, u32::from(category != 4));
            let wide = count(e);
            e.mem.set_u32(at + 4, wide);
            if category != 2 {
                counts.push(name);
            }
            if category != 4 {
                counts.push(wide);
            }
        }
        let time = e.mem.alloc(0xb8);
        e.mem.set_u32(time + 0xb4, 1);
        e.mem.set_u32(info + 0xd4, time);
        counts.push(time + 0xb4);
        (info, counts)
    }

    #[test]
    fn locale_references_go_up_and_down_together() {
        let mut e = engine();
        let (info, counts) = locale_info(&mut e);
        e.call(0x00ec_f5b6, &[info]);
        assert!(counts.iter().all(|c| e.mem.u32(*c) == 2));
        assert_eq!(e.call(0x00ec_f645, &[info]).u32(), info);
        assert!(counts.iter().all(|c| e.mem.u32(*c) == 1));
        // The skipped names keep their counts.
        let c_names = e.mem.u32(info + 0x50 + 2 * 0x10);
        assert_eq!(e.mem.u32(c_names), 1);
        assert_eq!(e.call(0x00ec_f645, &[0]).u32(), 0);
    }

    #[test]
    fn freetlocinfo_frees_the_unreferenced_pieces() {
        let mut e = engine();
        e.register(FREE_LCONV_MONETARY, |_, _| 0u32.into_ret());
        e.register(FREE_LCONV_NUMERIC, |_, _| 0u32.into_ret());
        e.register(FREE_LC_TIME, |_, _| 0u32.into_ret());
        let (info, counts) = locale_info(&mut e);
        for c in &counts {
            e.mem.set_u32(*c, 0);
        }
        let lconv = e.mem.alloc(0x40);
        e.mem.set_u32(info + 0xbc, lconv);
        let tables: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x200)).collect();
        e.mem.set_u32(info + 0xc4, tables[0] + 0xfe);
        e.mem.set_u32(info + 0xcc, tables[1] + 0x80);
        e.mem.set_u32(info + 0xd0, tables[2] + 0x80);
        let time = e.mem.u32(info + 0xd4);
        let c_names = e.mem.u32(info + 0x50 + 2 * 0x10);
        let no_wide = e.mem.u32(info + 0x50 + 4 * 0x10 + 4);
        e.call_log = Some(vec![]);
        e.call(0x00ec_f46d, &[info]);
        assert_eq!(calls_to(&e, FREE_LCONV_MONETARY), vec![vec![lconv]]);
        assert_eq!(calls_to(&e, FREE_LCONV_NUMERIC), vec![vec![lconv]]);
        assert_eq!(calls_to(&e, FREE_LC_TIME), vec![vec![time]]);
        for block in [info, lconv, time, tables[0], tables[1], tables[2]] {
            assert!(freed(&e, block), "{block:08x}");
        }
        for c in counts.iter().filter(|c| **c != info && **c != time + 0xb4) {
            assert!(freed(&e, *c), "{c:08x}");
        }
        assert!(!freed(&e, c_names));
        assert!(!freed(&e, no_wide));
    }

    #[test]
    fn freetlocinfo_keeps_the_c_locales_pieces() {
        let mut e = engine();
        let (info, _) = locale_info(&mut e);
        e.mem.set_u32(info + 0xbc, C_LCONV);
        e.mem.set_u32(info + 0xd4, C_LC_TIME);
        let ctype = e.mem.u32(info + 0xc0);
        e.call_log = Some(vec![]);
        e.call(0x00ec_f46d, &[info]);
        assert!(!freed(&e, ctype));
        assert!(freed(&e, info));
        assert!(e.call_log.as_ref().unwrap().len() == 1);
    }

    #[test]
    fn ansicp_reads_the_locales_code_page() {
        let mut e = engine();
        // `atol`'s body hands the text to `strtol(text, NULL, 10)` (`00ec8d08`), which reads
        // the exe image's locale data: doubled to parse the decimal digits.
        e.register(0x00ec_8d08, |e, a| {
            let text = e.mem.cstr(a[0]);
            let digits: String = text.iter().map(|&c| c as char).collect();
            digits.parse::<u32>().unwrap_or(0).into_ret()
        });
        e.register(IMPORT_GET_LOCALE_INFO_A, |e, a| {
            assert_eq!((a[0], a[1], a[3]), (0x409, 0x1004, 6));
            e.mem.write(a[2], b"1252\0");
            5u32.into_ret()
        });
        assert_eq!(e.call(0x00ee_13e5, &[0x409]).u32(), 1252);
        e.register(IMPORT_GET_LOCALE_INFO_A, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ee_13e5, &[0x409]).u32(), u32::MAX);
    }

    /// Doubles for the code-page imports: code page 932 is double-byte,
    /// the others single-byte; conversions copy bytes to and from 16-bit
    /// characters and give their counts.
    fn code_pages(e: &mut Engine) {
        e.register(IMPORT_GET_CP_INFO, |e, a| {
            e.mem.set_u32(a[1], if a[0] == 932 { 2 } else { 1 });
            1u32.into_ret()
        });
        e.register(IMPORT_MULTI_BYTE_TO_WIDE_CHAR, |e, a| {
            if a[4] != 0 {
                for i in 0..a[3] {
                    let c = e.mem.u8(a[2] + i);
                    e.mem.set_u16(a[4] + i * 2, u16::from(c));
                }
            }
            a[3].into_ret()
        });
        e.register(IMPORT_WIDE_CHAR_TO_MULTI_BYTE, |e, a| {
            if a[4] != 0 {
                for i in 0..a[3] {
                    let c = e.mem.u16(a[2] + i * 2);
                    e.mem.set_u8(a[4] + i, c as u8 + 1);
                }
            }
            a[3].into_ret()
        });
        e.register(MALLOC_BODY, |e, a| e.mem.alloc(a[0]).into_ret());
    }

    #[test]
    fn convertcp_converts_through_wide_characters() {
        let mut e = engine();
        code_pages(&mut e);
        let text = e.mem.alloc(8);
        e.mem.write(text, b"abc\0");
        let length = e.mem.alloc(4);
        e.mem.set_u32(length, 3);
        // The same code page: nothing.
        assert_eq!(
            e.call(0x00ee_142e, &[1252, 1252, text, length, 0, 0]).u32(),
            0
        );
        // Single-byte to single-byte, a new block sized by the text.
        let out = e.call(0x00ee_142e, &[1252, 1250, text, length, 0, 0]).u32();
        assert_eq!(e.mem.bytes(out, 3), b"bcd");
        assert_eq!(e.mem.u32(length), 3);
        // Double-byte source, into a given buffer; -1 counts the terminator.
        e.mem.set_u32(length, u32::MAX);
        let buffer = e.mem.alloc(8);
        e.register(IMPORT_MULTI_BYTE_TO_WIDE_CHAR, |e, a| {
            let count = if a[3] == u32::MAX { 4 } else { a[3] };
            if a[4] != 0 {
                for i in 0..count {
                    let c = e.mem.u8(a[2] + i);
                    e.mem.set_u16(a[4] + i * 2, u16::from(c));
                }
            }
            count.into_ret()
        });
        e.call_log = Some(vec![]);
        let result = e
            .call(0x00ee_142e, &[932, 1252, text, length, buffer, 8])
            .u32();
        assert_eq!(result, buffer);
        assert_eq!(e.mem.bytes(buffer, 4), b"bcd\x01");
        assert_eq!(e.mem.u32(length), u32::MAX);
        let wide = calls_to(&e, IMPORT_WIDE_CHAR_TO_MULTI_BYTE);
        assert_eq!(wide.len(), 1);
        assert_eq!(wide[0][3], 4);
        // A failed first conversion gives 0.
        e.register(IMPORT_MULTI_BYTE_TO_WIDE_CHAR, |_, _| 0u32.into_ret());
        assert_eq!(
            e.call(0x00ee_142e, &[932, 1252, text, length, 0, 0]).u32(),
            0
        );
    }

    #[test]
    fn convertcp_long_texts_use_a_heap_buffer_that_is_freed() {
        let mut e = engine();
        code_pages(&mut e);
        let text = e.mem.alloc(0x300);
        e.mem.write(text, &[b'a'; 0x300]);
        let length = e.mem.alloc(4);
        e.mem.set_u32(length, 0x300);
        e.call_log = Some(vec![]);
        let out = e.call(0x00ee_142e, &[932, 1252, text, length, 0, 0]).u32();
        assert_ne!(out, 0);
        assert_eq!(e.mem.u32(length), 0x300);
        let block = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .find(|(a, _)| *a == MALLOC_BODY)
            .unwrap()
            .1[0];
        assert_eq!(block, 0x300 * 2 + 8);
        // Sized by the conversion for a double-byte source: two calls out.
        assert_eq!(calls_to(&e, IMPORT_WIDE_CHAR_TO_MULTI_BYTE).len(), 2);
    }

    #[test]
    fn malloc_body_picks_the_heap_and_reports_failures() {
        let mut e = engine();
        e.mem.set_u32(LOCKS_READY, 0x0044_0000);
        e.register(IMPORT_HEAP_ALLOC, |_, a| {
            assert_eq!(&a[..2], &[0x0044_0000, 0]);
            (0x0010_0000 + a[2]).into_ret()
        });
        e.mem.set_u32(ACTIVE_HEAP, 1);
        assert_eq!(e.call(0x00ec_d1c7, &[0]).u32(), 0x0010_0001);
        assert_eq!(e.call(0x00ec_d1c7, &[0x21]).u32(), 0x0010_0021);
        // Other heaps round up to 16 bytes; the small-block heap goes first.
        e.mem.set_u32(ACTIVE_HEAP, 3);
        e.register(SMALL_BLOCK_ALLOC, |_, a| u32::from(a[0] == 8).into_ret());
        assert_eq!(e.call(0x00ec_d1c7, &[8]).u32(), 1);
        assert_eq!(e.call(0x00ec_d1c7, &[0x21]).u32(), 0x0010_0030);
        e.mem.set_u32(ACTIVE_HEAP, 2);
        assert_eq!(e.call(0x00ec_d1c7, &[0]).u32(), 0x0010_0010);
        // Failures: ENOMEM; with _newmode the new handler gets a retry.
        e.register(IMPORT_HEAP_ALLOC, |_, _| 0u32.into_ret());
        e.register_double(CALL_NEW_HANDLER, |e, _| {
            // The handler frees memory: the next attempt succeeds.
            e.register(IMPORT_HEAP_ALLOC, |_, _| 0x0020_0000u32.into_ret());
            1u32.into_ret()
        });
        assert_eq!(e.call(0x00ec_d1c7, &[4]).u32(), 0);
        assert_eq!(e.mem.u32(ERRNO_FALLBACK), 0xc);
        e.mem.set_u32(NEW_MODE, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_d1c7, &[4]).u32(), 0x0020_0000);
        assert_eq!(calls_to(&e, CALL_NEW_HANDLER), vec![vec![4]]);
        // Too large: the new handler, ENOMEM, 0.
        e.mem.set_u32(ERRNO_FALLBACK, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_d1c7, &[0xffff_fff0]).u32(), 0);
        assert_eq!(calls_to(&e, CALL_NEW_HANDLER), vec![vec![0xffff_fff0]]);
        assert_eq!(e.mem.u32(ERRNO_FALLBACK), 0xc);
    }

    #[test]
    fn freea_frees_only_heap_buffers() {
        let mut e = engine();
        let heap = e.mem.alloc(0x20);
        e.mem.set_u32(heap, 0xdddd);
        let stack = e.mem.alloc(0x20);
        e.mem.set_u32(stack, 0xcccc);
        e.call(0x00ec_8e14, &[heap + 8]);
        e.call(0x00ec_8e14, &[stack + 8]);
        e.call(0x00ec_8e14, &[0]);
        assert!(freed(&e, heap));
        assert!(!freed(&e, stack));
    }

    #[test]
    fn freefls_releases_a_thread_block() {
        let mut e = engine();
        for number in [0xc, 0xd] {
            e.mem.set_u32(LOCK_TABLE + number * 8, 0x0055_0000 + number);
        }
        let block = e.mem.alloc(0x214);
        let buffers: Vec<u32> = [0x24, 0x2c, 0x34, 0x3c, 0x40, 0x44, 0x48, 0x5c]
            .iter()
            .map(|offset| {
                let b = e.mem.alloc(8);
                e.mem.set_u32(block + offset, b);
                b
            })
            .collect();
        let mbc = e.mem.alloc(4);
        e.mem.set_u32(mbc, 1);
        e.mem.set_u32(block + 0x68, mbc);
        let (info, _) = locale_info(&mut e);
        e.mem.set_u32(info, 2);
        e.mem.set_u32(block + 0x6c, info);
        e.call_log = Some(vec![]);
        e.call(0x00ed_11f8, &[block]);
        assert!(buffers.iter().all(|b| freed(&e, *b)));
        assert!(freed(&e, mbc));
        assert!(freed(&e, block));
        // The locale info is still referenced: kept.
        assert!(!freed(&e, info));
        assert_eq!(
            calls_to(&e, IMPORT_ENTER_CRITICAL_SECTION),
            vec![vec![0x0055_000d], vec![0x0055_000c]]
        );
        assert_eq!(
            calls_to(&e, IMPORT_LEAVE_CRITICAL_SECTION),
            vec![vec![0x0055_000d], vec![0x0055_000c]]
        );
        // A null block does nothing.
        e.call_log = Some(vec![]);
        e.call(0x00ed_11f8, &[0]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod stream_tests {
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

    /// An engine whose thread block (`__getptd_noexit`) is a block it answers (`errno` at
    /// +8), with a handler set and the stream calls doubled.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let block = e.mem.alloc(0x10);
        e.register_double(GETPTD_NOEXIT, move |_, _| block.into_ret());
        e.register(DECODE_POINTER, |_, _| HANDLER.into_ret());
        for target in [HANDLER, LOCK_FILE, UNLOCK_FILE] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(FCLOSE_NOLOCK, |_, _| 7u32.into_ret());
        e.register(FWRITE_NOLOCK, |_, a| a[2].into_ret());
        e.register(FSOPEN, |_, _| 0x0e00_0000u32.into_ret());
        e.call_log = Some(vec![]);
        (e, block + 8)
    }

    #[test]
    fn fopen_opens_shared() {
        let (mut e, _) = engine();
        assert_eq!(e.call(0x00ec_9a47, &args![1u32, 2u32]).u32(), 0x0e00_0000);
        assert_eq!(calls(&e, FSOPEN), vec![vec![1, 2, 0x40]]);
    }

    #[test]
    fn fclose_locks_or_refuses() {
        let (mut e, error) = engine();
        let stream = e.mem.alloc(0x20);
        assert_eq!(e.call(0x00ec_9907, &args![stream]).i32(), 7);
        assert_eq!(calls(&e, LOCK_FILE), vec![vec![stream]]);
        assert_eq!(calls(&e, FCLOSE_NOLOCK), vec![vec![stream]]);
        assert_eq!(calls(&e, UNLOCK_FILE), vec![vec![stream]]);
        // A string stream: flags cleared, -1.
        e.mem.set_u32(stream + 0xc, 0x49);
        assert_eq!(e.call(0x00ec_9907, &args![stream]).i32(), -1);
        assert_eq!(e.mem.u32(stream + 0xc), 0);
        assert_eq!(calls(&e, LOCK_FILE).len(), 1);
        // No stream: EINVAL and the handler.
        assert_eq!(e.call(0x00ec_9907, &args![0u32]).i32(), -1);
        assert_eq!(e.mem.u32(error), 0x16);
        assert_eq!(calls(&e, HANDLER), vec![vec![0, 0, 0, 0, 0]]);
    }

    #[test]
    fn fwrite_locks_or_refuses() {
        let (mut e, error) = engine();
        let stream = e.mem.alloc(0x20);
        assert_eq!(
            e.call(0x00ec_b086, &args![9u32, 4u32, 3u32, stream]).u32(),
            3
        );
        assert_eq!(calls(&e, FWRITE_NOLOCK), vec![vec![9, 4, 3, stream]]);
        assert_eq!(calls(&e, UNLOCK_FILE), vec![vec![stream]]);
        assert_eq!(
            e.call(0x00ec_b086, &args![9u32, 0u32, 3u32, stream]).u32(),
            0
        );
        assert_eq!(e.call(0x00ec_b086, &args![9u32, 4u32, 0u32, 0u32]).u32(), 0);
        assert!(calls(&e, HANDLER).is_empty());
        assert_eq!(e.call(0x00ec_b086, &args![9u32, 4u32, 3u32, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(error), 0x16);
        assert_eq!(calls(&e, HANDLER).len(), 1);
        assert_eq!(calls(&e, LOCK_FILE).len(), 1);
    }
}

#[cfg(test)]
mod stream_internals_tests {
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

    /// An engine with `errno` at +8 of a block `__getptd_noexit` answers, a handler, the lock
    /// table's sections and the critical-section imports doubled; answers it and `errno`.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011c_1000, 0x2000);
        let block = e.mem.alloc(0x10);
        e.register_double(GETPTD_NOEXIT, move |_, _| block.into_ret());
        e.register(DECODE_POINTER, |_, _| HANDLER.into_ret());
        for target in [
            HANDLER,
            IMPORT_ENTER_CRITICAL_SECTION,
            IMPORT_LEAVE_CRITICAL_SECTION,
            FREEBUF,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, block + 8)
    }

    #[test]
    fn lock_file_uses_the_iob_lock_or_the_stream_section() {
        let (mut e, _) = engine();
        // `_iob` stream 2: lock 0x12, whose section the table holds.
        let stream = IOB_FIRST + 0x40;
        e.mem.set_u32(LOCK_TABLE + 0x12 * 8, 0x0e40_0000);
        e.call(0x00ec_e01b, &args![stream]);
        assert_eq!(e.mem.u32(stream + 0xc), 0x8000);
        assert_eq!(
            calls(&e, IMPORT_ENTER_CRITICAL_SECTION),
            vec![vec![0x0e40_0000]]
        );
        e.call(0x00ec_e08e, &args![stream]);
        assert_eq!(e.mem.u32(stream + 0xc), 0);
        assert_eq!(
            calls(&e, IMPORT_LEAVE_CRITICAL_SECTION),
            vec![vec![0x0e40_0000]]
        );
        // Another stream: its own section at +0x20.
        let other = e.mem.alloc(0x38);
        e.call(0x00ec_e01b, &args![other]);
        e.call(0x00ec_e08e, &args![other]);
        assert_eq!(
            calls(&e, IMPORT_ENTER_CRITICAL_SECTION)[1],
            vec![other + 0x20]
        );
        assert_eq!(
            calls(&e, IMPORT_LEAVE_CRITICAL_SECTION)[1],
            vec![other + 0x20]
        );
        assert_eq!(e.mem.u32(other + 0xc), 0);
    }

    #[test]
    fn fclose_nolock_flushes_closes_and_frees_the_name() {
        let (mut e, error) = engine();
        e.register(FLUSH, |_, _| 5u32.into_ret());
        e.register(FILENO, |_, _| 3u32.into_ret());
        e.register(CLOSE, |_, _| 0u32.into_ret());
        let stream = e.mem.alloc(0x20);
        e.mem.set_u32(stream + 0xc, 0x2);
        let name = e.mem.alloc(8);
        e.mem.set_u32(stream + 0x1c, name);
        assert_eq!(e.call(0x00ec_9890, &args![stream]).i32(), 5);
        assert_eq!(calls(&e, CLOSE), vec![vec![3]]);
        assert_eq!(calls(&e, FREEBUF), vec![vec![stream]]);
        assert_eq!(e.mem.u32(stream + 0x1c), 0);
        assert_eq!(e.mem.u32(stream + 0xc), 0);
        // A failed close answers -1 and keeps the name.
        e.register(CLOSE, |_, _| 0xffff_ffffu32.into_ret());
        e.mem.set_u32(stream + 0xc, 0x1);
        let kept = e.mem.alloc(8);
        e.mem.set_u32(stream + 0x1c, kept);
        assert_eq!(e.call(0x00ec_9890, &args![stream]).i32(), -1);
        assert_eq!(e.mem.u32(stream + 0x1c), kept);
        // An unused stream: -1, flags cleared; no stream: EINVAL.
        e.mem.set_u32(stream + 0xc, 0x4);
        assert_eq!(e.call(0x00ec_9890, &args![stream]).i32(), -1);
        assert_eq!(e.mem.u32(stream + 0xc), 0);
        assert_eq!(calls(&e, FLUSH).len(), 2);
        assert_eq!(e.call(0x00ec_9890, &args![0u32]).i32(), -1);
        assert_eq!(e.mem.u32(error), 0x16);
        assert_eq!(calls(&e, HANDLER).len(), 1);
    }

    #[test]
    fn fsopen_checks_the_names_and_unlocks_the_stream() {
        let (mut e, error) = engine();
        e.register(UNLOCK_FILE, |_, _| Ret::default());
        e.register(GETSTREAM, |_, _| 0x0e60_0000u32.into_ret());
        e.register(OPENFILE, |_, a| a[3].into_ret());
        let path = e.mem.alloc(8);
        e.mem.write(path, b"a.txt\0");
        let mode = e.mem.alloc(4);
        e.mem.write(mode, b"wb\0");
        let opened = e.call(0x00ec_9983, &args![path, mode, 0x40u32]).u32();
        assert_eq!(opened, 0x0e60_0000);
        assert_eq!(
            calls(&e, OPENFILE),
            vec![vec![path, mode, 0x40, 0x0e60_0000]]
        );
        assert_eq!(calls(&e, UNLOCK_FILE), vec![vec![0x0e60_0000]]);
        // An empty path: EINVAL, the stream unlocked, null.
        let empty = e.mem.alloc(4);
        assert_eq!(e.call(0x00ec_9983, &args![empty, mode, 0x40u32]).u32(), 0);
        assert_eq!(e.mem.u32(error), 0x16);
        assert_eq!(calls(&e, UNLOCK_FILE).len(), 2);
        assert!(calls(&e, HANDLER).is_empty());
        // An empty mode: the handler, no stream taken.
        assert_eq!(e.call(0x00ec_9983, &args![path, empty, 0x40u32]).u32(), 0);
        assert_eq!(calls(&e, HANDLER).len(), 1);
        assert_eq!(calls(&e, GETSTREAM).len(), 2);
        // No free stream: EMFILE.
        e.register(GETSTREAM, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ec_9983, &args![path, mode, 0x40u32]).u32(), 0);
        assert_eq!(e.mem.u32(error), 0x18);
    }

    /// A buffered stream (flags 0x108 by default) whose buffer has `room` bytes, buffer size 8.
    fn new_stream(e: &mut Engine, flags: u32, room: u32) -> (u32, u32) {
        let stream = e.mem.alloc(0x20);
        let buffer = e.mem.alloc(0x40);
        e.mem.set_u32(stream, buffer);
        e.mem.set_u32(stream + 4, room);
        e.mem.set_u32(stream + 0xc, flags);
        e.mem.set_u32(stream + 0x18, 8);
        (stream, buffer)
    }

    #[test]
    fn fwrite_nolock_fills_the_buffer_then_writes_whole_blocks() {
        let (mut e, _) = engine();
        e.register(FLUSH, |_, _| 0u32.into_ret());
        e.register(FILENO, |_, _| 3u32.into_ret());
        e.register(WRITE, |_, a| a[2].into_ret());
        e.register(FLSBUF, |_, _| 0u32.into_ret());
        let data = e.mem.alloc(0x20);
        e.mem.write(data, &(0..0x20).collect::<Vec<u8>>());
        let (stream, buffer) = new_stream(&mut e, 0x108, 4);
        // 5 elements of 4 bytes: 4 into the buffer, 16 written (two blocks of 8), none left.
        assert_eq!(
            e.call(0x00ec_af24, &args![data, 4u32, 5u32, stream]).u32(),
            5
        );
        assert_eq!(e.mem.bytes(buffer, 4), vec![0, 1, 2, 3]);
        assert_eq!(e.mem.u32(stream), buffer + 4);
        assert_eq!(e.mem.u32(stream + 4), 0);
        assert_eq!(calls(&e, WRITE), vec![vec![3, data + 4, 16]]);
        assert_eq!(calls(&e, FLUSH), vec![vec![stream]]);
        // 3 bytes left under the buffer size go one by one through `_flsbuf`.
        let (stream, _) = new_stream(&mut e, 0x108, 0);
        assert_eq!(
            e.call(0x00ec_af24, &args![data, 1u32, 3u32, stream]).u32(),
            3
        );
        let bytes: Vec<u32> = calls(&e, FLSBUF).iter().map(|a| a[0]).collect();
        assert_eq!(bytes, vec![0, 1, 2]);
    }

    #[test]
    fn fwrite_nolock_reports_errors_in_whole_elements() {
        let (mut e, error) = engine();
        e.register(FILENO, |_, _| 3u32.into_ret());
        e.register(WRITE, |_, a| (a[2] - 4).into_ret());
        let data = e.mem.alloc(0x40);
        // Unbuffered (size 0x1000): a short write of 0x1000 - 4 bytes of 0x1000 bytes.
        let big = e.mem.alloc(0x1000);
        let (stream, _) = new_stream(&mut e, 0, 0);
        assert_eq!(
            e.call(0x00ec_af24, &args![big, 0x100u32, 0x10u32, stream])
                .u32(),
            0xf
        );
        assert_eq!(e.mem.u32(stream + 0xc), 0x20);
        // A negative room marks the error at once.
        let (stream, _) = new_stream(&mut e, 0x108, 0xffff_ffff);
        assert_eq!(
            e.call(0x00ec_af24, &args![data, 1u32, 2u32, stream]).u32(),
            0
        );
        assert_eq!(e.mem.u32(stream + 0xc), 0x128);
        // Too many elements or no stream: EINVAL; a zero size: 0 quietly.
        assert_eq!(
            e.call(0x00ec_af24, &args![data, 2u32, 0x8000_0000u32, stream])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(error), 0x16);
        assert_eq!(e.call(0x00ec_af24, &args![data, 2u32, 1u32, 0u32]).u32(), 0);
        assert_eq!(calls(&e, HANDLER).len(), 2);
        assert_eq!(e.call(0x00ec_af24, &args![data, 0u32, 1u32, 0u32]).u32(), 0);
        assert_eq!(calls(&e, HANDLER).len(), 2);
    }
}

#[cfg(test)]
mod file_layer_tests {
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
    /// +0xc), the locks 1, 0xa and 0xb set up, one `ioinfo` block of 32 closed entries and the
    /// kernel imports doubled (`CreateFileA` answers 0x500, `GetFileType` 1, `SetFilePointer`
    /// 0x40, `CloseHandle` 1, `GetLastError` 0). Answers the engine, the thread block and the
    /// `ioinfo` block.
    fn engine() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.map(0x011c_1000, 0x2000);
        let block = e.mem.alloc(0x40);
        e.register_double(GETPTD_NOEXIT, move |_, _| block.into_ret());
        e.register(DECODE_POINTER, |_, _| HANDLER.into_ret());
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
        for target in [
            HANDLER,
            IMPORT_ENTER_CRITICAL_SECTION,
            IMPORT_LEAVE_CRITICAL_SECTION,
            IMPORT_SET_STD_HANDLE,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(IMPORT_INIT_CRITICAL_SECTION_SPIN, |_, _| 1u32.into_ret());
        e.register(IMPORT_CREATE_FILE_A, |_, _| 0x500u32.into_ret());
        e.register(IMPORT_GET_FILE_TYPE, |_, _| 1u32.into_ret());
        e.register(IMPORT_SET_FILE_POINTER, |_, _| 0x40u32.into_ret());
        e.register(IMPORT_CLOSE_HANDLE, |_, _| 1u32.into_ret());
        // `_get_fmode` (in another crate) answers the text default 0.
        e.register(GET_FMODE, |e, a| {
            e.mem.set_u32(a[0], 0);
            0u32.into_ret()
        });
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        (e, block, entries)
    }

    /// Handle `index` opened on the Windows handle `os` with the `osfile` flags.
    fn open_handle(e: &mut Engine, entries: u32, index: u32, os: u32, osfile: u8) {
        e.mem.set_u32(entries + 0x40 * index, os);
        e.mem.set_u8(entries + 0x40 * index + 4, osfile);
    }

    #[test]
    fn os_errors_map_to_errno() {
        let (mut e, block, _) = engine();
        e.mem.set_u32(ERROR_TABLE, 2);
        e.mem.set_u32(ERROR_TABLE + 4, 2);
        assert_eq!(e.call(0x00ec_85a1, &args![2u32]).u32(), 2);
        assert_eq!(e.call(0x00ec_85a1, &args![0x20u32]).u32(), 0xd);
        assert_eq!(e.call(0x00ec_85a1, &args![0xc1u32]).u32(), 8);
        assert_eq!(e.call(0x00ec_85a1, &args![0x500u32]).u32(), 0x16);
        e.call(0x00ec_8609, &args![0x20u32]);
        assert_eq!(e.mem.u32(block + 0xc), 0x20);
        assert_eq!(e.mem.u32(block + 8), 0xd);
        assert_eq!(e.call(0x00ec_85f6, &args![]).u32(), block + 0xc);
        e.register(GETPTD_NOEXIT, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ec_85f6, &args![]).u32(), DOSERRNO_FALLBACK);
    }

    #[test]
    fn os_handles_are_read_set_and_freed() {
        let (mut e, block, entries) = engine();
        e.mem.set_u32(APP_TYPE, 1);
        assert_eq!(e.call(0x00ee_2e45, &args![1u32]).u32(), 0xffff_ffff);
        assert_eq!(e.mem.u32(block + 8), 9);
        assert_eq!(calls(&e, HANDLER).len(), 1);
        assert_eq!(
            e.call(0x00ee_2e45, &args![0xffff_fffeu32]).u32(),
            0xffff_ffff
        );
        assert_eq!(calls(&e, HANDLER).len(), 1);
        assert_eq!(e.call(0x00ee_2d3e, &args![1u32, 0x700u32]).i32(), 0);
        assert_eq!(e.mem.u32(entries + 0x40), 0x700);
        assert_eq!(
            calls(&e, IMPORT_SET_STD_HANDLE),
            vec![vec![0xffff_fff5, 0x700]]
        );
        // Already set: refused.
        assert_eq!(e.call(0x00ee_2d3e, &args![1u32, 0x701u32]).i32(), -1);
        e.mem.set_u8(entries + 0x44, 1);
        assert_eq!(e.call(0x00ee_2e45, &args![1u32]).u32(), 0x700);
        assert_eq!(e.call(0x00ee_2dbf, &args![1u32]).i32(), 0);
        assert_eq!(e.mem.u32(entries + 0x40), 0xffff_ffff);
        assert_eq!(calls(&e, IMPORT_SET_STD_HANDLE)[1], vec![0xffff_fff5, 0]);
        assert_eq!(e.call(0x00ee_2dbf, &args![1u32]).i32(), -1);
        assert_eq!(e.call(0x00ee_2dbf, &args![0x40u32]).i32(), -1);
    }

    #[test]
    fn seeks_move_the_file_and_clear_its_eof() {
        let (mut e, block, entries) = engine();
        open_handle(&mut e, entries, 3, 0x600, 0x3);
        assert_eq!(e.call(0x00ed_d009, &args![3u32, 5u32, 1u32]).u32(), 0x40);
        assert_eq!(
            calls(&e, IMPORT_SET_FILE_POINTER),
            vec![vec![0x600, 5, 0, 1]]
        );
        assert_eq!(e.mem.u8(entries + 0xc4), 1);
        // 64-bit: the high word goes through a stack word.
        e.register(IMPORT_SET_FILE_POINTER, |e, a| {
            e.mem.set_u32(a[2], 7);
            0x10u32.into_ret()
        });
        let answer = e.call(0x00ee_202c, &args![3u32, 0u32, 1u32, 2u32]);
        assert_eq!(answer.u32(), 0x10);
        assert_eq!(answer.u64(), 0x7_0000_0010);
        // Lock 0xa while the handle's lock is set up, then the handle's own lock.
        assert_eq!(calls(&e, IMPORT_ENTER_CRITICAL_SECTION).len(), 2);
        assert_eq!(calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len(), 2);
        // A failure with an error maps it and answers -1.
        e.register(IMPORT_SET_FILE_POINTER, |_, _| 0xffff_ffffu32.into_ret());
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 0x83u32.into_ret());
        assert_eq!(
            e.call(0x00ed_d009, &args![3u32, 0u32, 0u32]).u32(),
            0xffff_ffff
        );
        assert_eq!(e.mem.u32(block + 0xc), 0x83);
        assert_eq!(
            e.call(0x00ee_1fa7, &args![3u32, 0u32, 0u32, 0u32]).u64(),
            u64::MAX
        );
        // A closed handle: EBADF.
        assert_eq!(
            e.call(0x00ee_202c, &args![4u32, 0u32, 0u32, 0u32]).u64(),
            u64::MAX
        );
        assert_eq!(e.mem.u32(block + 8), 9);
        assert_eq!(e.call(0x00ee_2145, &args![3u32]).u32(), 0);
        e.mem.set_u8(entries + 0xc4, 0x41);
        assert_eq!(e.call(0x00ee_2145, &args![3u32]).u32(), 0x40);
    }

    #[test]
    fn close_frees_the_handle_but_keeps_a_shared_standard_one() {
        let (mut e, block, entries) = engine();
        open_handle(&mut e, entries, 3, 0x600, 0x1);
        assert_eq!(e.call(0x00ed_813a, &args![3u32]).i32(), 0);
        assert_eq!(calls(&e, IMPORT_CLOSE_HANDLE), vec![vec![0x600]]);
        assert_eq!(e.mem.u32(entries + 0xc0), 0xffff_ffff);
        assert_eq!(e.mem.u8(entries + 0xc4), 0);
        assert_eq!(e.call(0x00ed_813a, &args![3u32]).i32(), -1);
        assert_eq!(e.mem.u32(block + 8), 9);
        // stdout and stderr on one Windows handle: closing stdout does not close it.
        open_handle(&mut e, entries, 1, 0x601, 0x1);
        open_handle(&mut e, entries, 2, 0x601, 0x1);
        assert_eq!(e.call(0x00ed_809e, &args![1u32]).i32(), 0);
        assert_eq!(calls(&e, IMPORT_CLOSE_HANDLE).len(), 1);
        // A failed CloseHandle maps the error.
        open_handle(&mut e, entries, 5, 0x602, 0x1);
        e.register(IMPORT_CLOSE_HANDLE, |_, _| 0u32.into_ret());
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 6u32.into_ret());
        assert_eq!(e.call(0x00ed_809e, &args![5u32]).i32(), -1);
        assert_eq!(e.mem.u32(block + 0xc), 6);
    }

    #[test]
    fn alloc_osfhnd_takes_a_free_entry_or_a_new_block() {
        let (mut e, _, entries) = engine();
        open_handle(&mut e, entries, 0, 0x600, 1);
        assert_eq!(e.call(0x00ee_2f83, &args![]).u32(), 1);
        assert_eq!(e.mem.u8(entries + 0x44), 1);
        assert_eq!(e.mem.u32(entries + 0x48), 1);
        for index in 0..0x20 {
            e.mem.set_u8(entries + 0x40 * index + 4, 1);
        }
        assert_eq!(e.call(0x00ee_2f83, &args![]).u32(), 0x20);
        assert_eq!(e.mem.u32(NHANDLE), 0x40);
        let second = e.mem.u32(PIOINFO + 4);
        assert_eq!(e.mem.u8(second + 4), 1);
        assert_eq!(e.mem.u8(second + 0x45), 0xa);
        assert_eq!(e.mem.u32(second + 0x40), 0xffff_ffff);
        assert_eq!(e.mem.u32(second + 8), 1);
    }

    #[test]
    fn sopen_s_opens_a_binary_file_and_reports_failures() {
        let (mut e, block, entries) = engine();
        let path = e.mem.alloc(8);
        let handle = e.mem.alloc(4);
        // _O_RDONLY | _O_BINARY, _SH_DENYNO.
        assert_eq!(
            e.call(
                0x00ee_3908,
                &args![handle, path, 0x8000u32, 0x40u32, 0x180u32]
            )
            .u32(),
            0
        );
        assert_eq!(e.mem.u32(handle), 0);
        assert_eq!(e.mem.u32(entries), 0x500);
        assert_eq!(e.mem.u8(entries + 4), 1);
        let create = &calls(&e, IMPORT_CREATE_FILE_A)[0];
        assert_eq!(create[..3], [path, 0x8000_0000, 3]);
        assert_eq!(create[4..], [3, 0x80, 0]);
        // The lock taken by _alloc_osfhnd is left again.
        assert_eq!(
            calls(&e, IMPORT_LEAVE_CRITICAL_SECTION).len(),
            calls(&e, IMPORT_ENTER_CRITICAL_SECTION).len()
        );
        // A failed CreateFileA: the error mapped, the handle -1.
        e.register(IMPORT_CREATE_FILE_A, |_, _| 0xffff_ffffu32.into_ret());
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 2u32.into_ret());
        e.mem.set_u32(ERROR_TABLE, 2);
        e.mem.set_u32(ERROR_TABLE + 4, 2);
        assert_eq!(
            e.call(
                0x00ee_3908,
                &args![handle, path, 0x8000u32, 0x40u32, 0x180u32]
            )
            .u32(),
            2
        );
        assert_eq!(e.mem.u32(handle), 0xffff_ffff);
        assert_eq!(e.mem.u8(entries + 0x44), 0);
        // A bad share flag or mode: EINVAL.
        assert_eq!(
            e.call(
                0x00ee_3908,
                &args![handle, path, 0x8000u32, 0x50u32, 0x180u32]
            )
            .u32(),
            0x16
        );
        assert_eq!(
            e.call(
                0x00ee_3908,
                &args![handle, path, 0x8000u32, 0x40u32, 0x1u32]
            )
            .u32(),
            0x16
        );
        assert_eq!(e.mem.u32(block + 8), 0x16);
    }

    #[test]
    fn sopen_s_writes_a_utf8_mark_for_a_new_text_file() {
        let (mut e, _, entries) = engine();
        e.register(WRITE_NOLOCK, |_, a| a[2].into_ret());
        let path = e.mem.alloc(8);
        let handle = e.mem.alloc(4);
        // _O_WRONLY | _O_CREAT | _O_TRUNC | _O_TEXT | _O_U8TEXT.
        let flags = 0x301 | 0x4000 | 0x40000;
        assert_eq!(
            e.call(
                0x00ee_3908,
                &args![handle, path, flags as u32, 0x40u32, 0x180u32]
            )
            .u32(),
            0
        );
        let writes = calls(&e, WRITE_NOLOCK);
        assert_eq!(writes.len(), 1);
        assert_eq!(writes[0][2], 3);
        assert_eq!(e.mem.u8(entries + 4), 0x81);
        assert_eq!(e.mem.u8(entries + 0x24) & 0x7f, 1);
    }

    #[test]
    fn openfile_parses_the_mode() {
        let (mut e, _, entries) = engine();
        let path = e.mem.alloc(8);
        let mode = e.mem.alloc(16);
        e.mem.write(mode, b" rb\0");
        let stream = e.mem.alloc(0x38);
        e.mem.set_u32(COMMODE, 0);
        assert_eq!(
            e.call(0x00ed_8238, &args![path, mode, 0x40u32, stream])
                .u32(),
            stream
        );
        assert_eq!(e.mem.u32(stream + 0xc), 1);
        assert_eq!(e.mem.u32(stream + 0x10), 0);
        assert_eq!(e.mem.u32(CFLUSH), 1);
        assert_eq!(e.mem.u8(entries + 4), 1);
        // Text read-write: the last byte is read for a Ctrl-Z (none here).
        e.register(READ_NOLOCK, |_, _| 0u32.into_ret());
        e.mem.write(mode, b"w+t\0");
        e.call(0x00ed_8238, &args![path, mode, 0x40u32, stream]);
        assert_eq!(e.mem.u32(stream + 0xc), 0x80);
        assert_eq!(e.mem.u32(stream + 0x10), 1);
        let create = calls(&e, IMPORT_CREATE_FILE_A);
        assert_eq!(create[1][1], 0xc000_0000);
        assert_eq!(create[1][4], 2);
        // An unknown letter: EINVAL, no open.
        e.mem.write(mode, b"rq\0");
        assert_eq!(
            e.call(0x00ed_8238, &args![path, mode, 0x40u32, stream])
                .u32(),
            0
        );
        assert_eq!(calls(&e, IMPORT_CREATE_FILE_A).len(), 2);
        assert_eq!(calls(&e, HANDLER).len(), 1);
    }

    #[test]
    fn getstream_reuses_a_free_stream_or_makes_one() {
        let (mut e, _, _) = engine();
        e.map(0x0127_0000, 0x2000);
        let piob = e.mem.alloc(0x10);
        let busy = e.mem.alloc(0x38);
        e.mem.set_u32(busy + 0xc, 1);
        let free_stream = e.mem.alloc(0x38);
        e.mem.set_u32(free_stream + 0xc, 0x10);
        e.mem.set_u32(piob, busy);
        e.mem.set_u32(piob + 4, free_stream);
        e.mem.set_u32(PIOB, piob);
        e.mem.set_u32(NSTREAM, 3);
        for number in [0x11u32, 0x12] {
            let section = e.mem.alloc(0x18);
            e.mem.set_u32(LOCK_TABLE + 8 * number, section);
        }
        assert_eq!(e.call(0x00ed_8501, &args![]).u32(), free_stream);
        assert_eq!(e.mem.u32(free_stream + 0xc), 0x8000);
        assert_eq!(e.mem.u32(free_stream + 0x10), 0xffff_ffff);
        // Now both are in use: slot 2 gets a new stream.
        e.mem.set_u32(free_stream + 0xc, 2);
        let made = e.call(0x00ed_8501, &args![]).u32();
        assert_eq!(e.mem.u32(piob + 8), made);
        assert_eq!(e.mem.u32(made + 0xc), 0);
        assert_eq!(
            calls(&e, IMPORT_INIT_CRITICAL_SECTION_SPIN).last().unwrap(),
            &vec![made + 0x20, 0xfa0]
        );
        // No free slot: none.
        e.mem.set_u32(made + 0xc, 1);
        assert_eq!(e.call(0x00ed_8501, &args![]).u32(), 0);
    }

    #[test]
    fn stream_buffers_flush_and_free() {
        let (mut e, _, entries) = engine();
        e.register(WRITE_NOLOCK, |_, a| a[2].into_ret());
        open_handle(&mut e, entries, 3, 0x600, 1);
        let stream = e.mem.alloc(0x38);
        e.mem.set_u32(stream + 0x10, 3);
        assert_eq!(e.call(0x00ed_806c, &args![stream]).u32(), 3);
        assert_eq!(e.call(0x00ed_806c, &args![0u32]).u32(), 0xffff_ffff);
        e.call(0x00ed_ecf7, &args![stream]);
        let buffer = e.mem.u32(stream + 8);
        assert_ne!(buffer, 0);
        assert_eq!(e.mem.u32(stream + 0xc), 8);
        assert_eq!(e.mem.u32(stream + 0x18), 0x1000);
        // Written through _flsbuf: the first byte goes into the buffer.
        e.mem.set_u32(stream + 0xc, 0x8a);
        assert_eq!(e.call(0x00ed_2d64, &args![0x41u32, stream]).i32(), 0x41);
        assert_eq!(e.mem.u8(buffer), 0x41);
        assert_eq!(e.mem.u32(stream), buffer + 1);
        assert_eq!(e.mem.u32(stream + 4), 0xfff);
        // _flush writes the pending byte and turns a read-write stream back.
        assert_eq!(e.call(0x00ec_d8e6, &args![stream]).i32(), 0);
        assert_eq!(calls(&e, WRITE_NOLOCK), vec![vec![3, buffer, 1]]);
        assert_eq!(e.mem.u32(stream + 0xc), 0x88);
        assert_eq!(e.mem.u32(stream), buffer);
        e.mem.set_u32(stream + 0xc, 0x8a);
        e.call(0x00ed_8207, &args![stream]);
        assert_eq!(e.mem.u32(stream + 0xc), 0x82);
        assert_eq!(e.mem.u32(stream + 8), 0);
        // A stream not open for writing.
        e.mem.set_u32(stream + 0xc, 0x1);
        assert_eq!(e.call(0x00ed_2d64, &args![0x41u32, stream]).i32(), -1);
        assert_eq!(e.mem.u32(stream + 0xc), 0x21);
        // An unbuffered stream writes the byte alone.
        e.mem.set_u32(stream + 0xc, 0x6);
        assert_eq!(e.call(0x00ed_2d64, &args![0x142u32, stream]).i32(), 0x42);
        assert_eq!(calls(&e, WRITE_NOLOCK)[1][2], 1);
    }

    #[test]
    fn stream_locks_and_mode_queries() {
        let (mut e, _, _) = engine();
        let section = e.mem.alloc(0x18);
        e.mem.set_u32(LOCK_TABLE + 8 * 0x13, section);
        let stream = e.mem.alloc(0x38);
        e.call(0x00ec_e05c, &args![3u32, stream]);
        assert_eq!(e.mem.u32(stream + 0xc), 0x8000);
        e.call(0x00ec_e0ca, &args![3u32, stream]);
        assert_eq!(e.mem.u32(stream + 0xc), 0);
        e.call(0x00ec_e05c, &args![0x14u32, stream]);
        e.call(0x00ec_e0ca, &args![0x14u32, stream]);
        assert_eq!(
            calls(&e, IMPORT_ENTER_CRITICAL_SECTION),
            vec![vec![section], vec![stream + 0x20]]
        );
        assert_eq!(
            calls(&e, IMPORT_LEAVE_CRITICAL_SECTION)[1],
            vec![stream + 0x20]
        );
    }
}

#[cfg(test)]
mod b000s_fprintf_tests {
    use super::*;

    /// The invalid-parameter handler the doubled `__decode_pointer` answers.
    const HANDLER: u32 = 0x0e30_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with a thread block (`errno` at +8), a handler, the stream calls doubled,
    /// `_fileno` answering `handle`, and handle 3's `ioinfo` entry with `mode` at +0x24.
    /// Answers the engine and the `errno` address.
    fn engine(handle: u32, mode: u8) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.map(BAD_IOINFO, 0x40);
        let block = e.mem.alloc(0x10);
        e.register_double(GETPTD_NOEXIT, move |_, _| block.into_ret());
        e.register(DECODE_POINTER, |_, _| HANDLER.into_ret());
        for target in [HANDLER, LOCK_FILE, UNLOCK_FILE, FTBUF] {
            e.register(target, |_, _| Ret::default());
        }
        e.register_double(FILENO, move |_, _| handle.into_ret());
        e.register(STBUF, |_, _| 5u32.into_ret());
        // `_output_l` double: answers the sum of the first two `va_list` words.
        e.register(OUTPUT_L, |e, a| {
            (e.mem.u32(a[3]) + e.mem.u32(a[3] + 4)).into_ret()
        });
        let entries = e.mem.alloc(0x40 * 4);
        e.mem.set_u32(PIOINFO, entries);
        e.mem.set_u8(entries + 3 * 0x40 + 0x24, mode);
        e.call_log = Some(vec![]);
        (e, block + 8)
    }

    #[test]
    fn fprintf_formats_through_output_l() {
        let (mut e, _) = engine(3, 0);
        let stream = e.mem.alloc(0x20);
        let result = e
            .call(0x00ec_9774, &args![stream, 0x7777u32, 30u32, 12u32])
            .i32();
        assert_eq!(result, 42);
        assert_eq!(calls(&e, LOCK_FILE), vec![vec![stream]]);
        let output = calls(&e, OUTPUT_L);
        assert_eq!(&output[0][..3], &[stream, 0x7777, 0]);
        assert_eq!(calls(&e, FTBUF), vec![vec![5, stream]]);
        assert_eq!(calls(&e, UNLOCK_FILE), vec![vec![stream]]);
        // A string stream skips the handle check.
        e.mem.set_u32(stream + 0xc, 0x40);
        e.call(0x00ec_9774, &args![stream, 0x7777u32, 1u32, 1u32]);
        assert_eq!(calls(&e, FILENO).len(), 1);
    }

    #[test]
    fn fprintf_refuses_a_text_mode_handle() {
        for (handle, mode) in [(3u32, 0x01u8), (3, 0x80), (u32::MAX, 0)] {
            let (mut e, error) = engine(handle, mode);
            if handle == u32::MAX {
                e.mem.set_u8(BAD_IOINFO + 0x24, 0x80);
            }
            let stream = e.mem.alloc(0x20);
            let result = e.call(0x00ec_9774, &args![stream, 0x7777u32]).i32();
            assert_eq!(result, -1);
            assert_eq!(e.mem.u32(error), 0x16);
            assert_eq!(calls(&e, HANDLER), vec![vec![0, 0, 0, 0, 0]]);
            assert!(calls(&e, OUTPUT_L).is_empty());
            assert_eq!(calls(&e, UNLOCK_FILE), vec![vec![stream]]);
        }
    }

    #[test]
    fn fprintf_refuses_null_arguments_unlocked() {
        let (mut e, error) = engine(3, 0);
        assert_eq!(e.call(0x00ec_9774, &args![0u32, 0x7777u32]).i32(), -1);
        assert_eq!(e.call(0x00ec_9774, &args![0x100u32, 0u32]).i32(), -1);
        assert_eq!(e.mem.u32(error), 0x16);
        assert_eq!(calls(&e, HANDLER).len(), 2);
        assert!(calls(&e, LOCK_FILE).is_empty());
        assert!(calls(&e, UNLOCK_FILE).is_empty());
    }
}

#[cfg(test)]
mod b000t_output_tests {
    use super::*;

    const HANDLER: u32 = 0x0e30_0000;
    const CONVERT: u32 = 0x0e31_0000;
    const FORCE_POINT: u32 = 0x0e32_0000;
    const CROP: u32 = 0x0e33_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The format state machine's table as LIBCMT defines it: each character's class in the
    /// low nibble of the byte at `010f3080 + character`, each (class, state)'s next state in
    /// the high nibble of the byte at `010f30a0 + class * 8 + state` (one table, two views).
    fn lookup_table(e: &mut Engine) {
        e.map(LOOKUP_CLASSES, 0x100);
        let classes: &[(&[u8], u8)] = &[
            (b" #+-", 6),
            (b"%", 1),
            (b"*", 3),
            (b".", 2),
            (b"0", 4),
            (b"123456789", 5),
            (b"FILNhlw", 7),
            (b"ACEGSXZacdefginopsux", 8),
        ];
        for (characters, class) in classes {
            for c in characters.iter() {
                let at = LOOKUP_CLASSES + u32::from(*c);
                let byte = e.mem.u8(at) | class;
                e.mem.set_u8(at, byte);
            }
        }
        // Rows: classes 0 (other) to 8 (type); columns: states 0 (normal) to 7 (type).
        let next: [[u8; 8]; 9] = [
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 1],
            [0, 4, 4, 4, 0, 0, 0, 0],
            [0, 3, 3, 0, 5, 0, 0, 0],
            [0, 2, 2, 3, 5, 5, 0, 0],
            [0, 3, 3, 3, 5, 5, 0, 0],
            [0, 2, 2, 0, 0, 0, 0, 0],
            [0, 6, 6, 6, 6, 6, 6, 0],
            [0, 7, 7, 7, 7, 7, 7, 0],
        ];
        for (class, row) in next.iter().enumerate() {
            for (state, value) in row.iter().enumerate() {
                let at = LOOKUP_STATES + (class * 8 + state) as u32;
                let byte = e.mem.u8(at) | (value << 4);
                e.mem.set_u8(at, byte);
            }
        }
    }

    /// An engine for `_output_l`: thread block (errno), handler, identity pointer decoding,
    /// the lookup table, the null strings, and doubles for the locale and the float helpers.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.map(0x011c_1000, 0x2000);
        e.map(SECURITY_COOKIE, 4);
        let block = e.mem.alloc(0x80);
        e.register_double(GETPTD_NOEXIT, move |_, _| block.into_ret());
        e.register(DECODE_POINTER, |_, a| a[0].into_ret());
        e.mem.set_u32(INVALID_PARAMETER_HANDLER, HANDLER);
        e.register(HANDLER, |_, _| Ret::default());
        e.register(LOCALE_UPDATE, |e, a| {
            e.mem.set_u8(a[0] + 0xc, 0);
            a[0].into_ret()
        });
        e.register(ISLEADBYTE_L, |_, _| 0u32.into_ret());
        e.register(WCTOMB_S, |e, a| {
            e.mem.set_u8(a[1], a[3] as u8);
            e.mem.set_u32(a[0], 1);
            0u32.into_ret()
        });
        e.register(GET_PRINTF_COUNT_OUTPUT, |_, _| 1u32.into_ret());
        e.register(MALLOC_CRT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE_CRT, |_, _| Ret::default());
        e.mem.set_u32(CFLTCVT, CONVERT);
        e.mem.set_u32(FORCDECPT, FORCE_POINT);
        e.mem.set_u32(CROPZEROS, CROP);
        // `_cfltcvt` double: writes "-1.50" for 'f' and "2.5e+000" otherwise.
        e.register(CONVERT, |e, a| {
            let text: &[u8] = if a[3] == u32::from(b'f') {
                b"-1.50"
            } else {
                b"2.5e+000"
            };
            e.mem.set_cstr(a[1], text);
            Ret::default()
        });
        e.register(FORCE_POINT, |_, _| Ret::default());
        e.register(CROP, |_, _| Ret::default());
        let null = e.mem.alloc(8);
        e.mem.set_cstr(null, b"(null)");
        e.mem.set_u32(NULL_STRING, null);
        lookup_table(&mut e);
        e.call_log = Some(vec![]);
        (e, block + 8)
    }

    /// A string stream writing into a buffer: `_ptr`, `_cnt`, `_base`, `_flag` 0x42.
    fn string_stream(e: &mut Engine) -> (u32, u32) {
        let buffer = e.mem.alloc(0x100);
        let stream = e.mem.alloc(0x20);
        e.mem.set_u32(stream, buffer);
        e.mem.set_u32(stream + 4, 0x100);
        e.mem.set_u32(stream + 8, buffer);
        e.mem.set_u32(stream + 0xc, 0x42);
        (stream, buffer)
    }

    /// Runs `_output_l` on `format` with `words` as the argument list; answers the result and
    /// the text written.
    fn run(e: &mut Engine, format: &[u8], words: &[u32]) -> (i32, Vec<u8>) {
        let (stream, buffer) = string_stream(e);
        let text = e.mem.alloc(format.len() as u32 + 1);
        e.mem.set_cstr(text, format);
        let list = e.mem.alloc(4 * words.len() as u32 + 4);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *word);
        }
        let result = e.call(0x00ed_2f21, &args![stream, text, 0u32, list]).i32();
        let written = e.mem.u32(stream).wrapping_sub(buffer);
        (result, e.mem.bytes(buffer, written))
    }

    #[test]
    fn output_l_formats_integers_strings_and_characters() {
        let (mut e, _) = engine();
        let hi = e.mem.alloc(4);
        e.mem.set_cstr(hi, b"hi");
        let (result, text) = run(
            &mut e,
            b"a%5d|%-4s|%#x|%c%%|%X",
            &[42, hi, 255, u32::from(b'z'), 0xbeef],
        );
        assert_eq!(text, b"a   42|hi  |0xff|z%|BEEF".to_vec());
        assert_eq!(result, text.len() as i32);
    }

    #[test]
    fn output_l_signs_precision_sizes_and_octal() {
        let (mut e, _) = engine();
        let minus_five = (-5i64) as u64;
        let (result, text) = run(
            &mut e,
            b"%+.3d %lld %05d %o %#o %hd %I64u",
            &[
                (-7i32) as u32,
                minus_five as u32,
                (minus_five >> 32) as u32,
                12,
                8,
                8,
                0x1_ffff,
                0,
                1,
            ],
        );
        assert_eq!(text, b"-007 -5 00012 10 010 -1 4294967296".to_vec());
        assert_eq!(result, text.len() as i32);
    }

    #[test]
    fn output_l_converts_floats_through_cfltcvt() {
        let (mut e, _) = engine();
        let (result, text) = run(&mut e, b"%.2f|%E|%g", &[0, 0, 0, 0, 0, 0]);
        assert_eq!(text, b"-1.50|2.5e+000|2.5e+000".to_vec());
        assert_eq!(result, text.len() as i32);
        let converts = calls(&e, CONVERT);
        // (value, buffer, size, character, precision, capitals, locale)
        assert_eq!(
            (converts[0][2], converts[0][3], converts[0][4]),
            (0x200, 0x66, 2)
        );
        assert_eq!(
            (converts[1][3], converts[1][4], converts[1][5]),
            (0x65, 6, 1)
        );
        assert_eq!((converts[2][3], converts[2][4]), (0x67, 6));
        assert_eq!(calls(&e, CROP).len(), 1);
        assert!(calls(&e, FORCE_POINT).is_empty());
        // A large precision takes a heap buffer, freed afterwards.
        run(&mut e, b"%.200f", &[0, 0]);
        assert_eq!(calls(&e, MALLOC_CRT), vec![vec![200 + 0x15d]]);
        assert_eq!(calls(&e, FREE_CRT).len(), 1);
    }

    #[test]
    fn output_l_null_and_wide_strings_and_counts() {
        let (mut e, _) = engine();
        let wide = e.mem.alloc(8);
        e.mem.set_u16(wide, u16::from(b'a'));
        e.mem.set_u16(wide + 2, u16::from(b'b'));
        let count = e.mem.alloc(4);
        let (result, text) = run(&mut e, b"%s|%S|%.1s%n|%Iq", &[0, wide, wide, count]);
        assert_eq!(text, b"(null)|ab|a|Iq".to_vec());
        assert_eq!(result, 14);
        assert_eq!(e.mem.u32(count), 11);
        // %.1s on a narrow string stops after one character; the wide one converts both.
        assert_eq!(calls(&e, WCTOMB_S).len(), 2);
    }

    #[test]
    fn output_l_refuses_bad_streams_and_disabled_counts() {
        let (mut e, error) = engine();
        let text = e.mem.alloc(4);
        e.mem.set_cstr(text, b"x");
        assert_eq!(
            e.call(0x00ed_2f21, &args![0u32, text, 0u32, 0u32]).i32(),
            -1
        );
        assert_eq!(e.mem.u32(error), 0x16);
        assert_eq!(calls(&e, HANDLER).len(), 1);
        e.register(GET_PRINTF_COUNT_OUTPUT, |_, _| 0u32.into_ret());
        let count = e.mem.alloc(4);
        let (result, written) = run(&mut e, b"ab%n", &[count]);
        assert_eq!(result, -1);
        assert_eq!(written, b"ab".to_vec());
        assert_eq!(calls(&e, HANDLER).len(), 2);
    }

    #[test]
    fn stbuf_and_ftbuf_lend_stdout_a_buffer() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.map(0x011c_1a00, 0x300);
        e.register(FILENO, |_, _| 1u32.into_ret());
        e.register(ISATTY, |_, _| 1u32.into_ret());
        e.register(MALLOC_CRT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FLUSH, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let stdout = IOB_FIRST + 0x20;
        assert_eq!(e.call(0x00ed_7d48, &args![stdout]).u32(), 1);
        let buffer = e.mem.u32(STDBUF);
        assert_ne!(buffer, 0);
        assert_eq!(e.mem.u32(stdout), buffer);
        assert_eq!(e.mem.u32(stdout + 4), 0x1000);
        assert_eq!(e.mem.u32(stdout + 0xc), 0x1102);
        assert_eq!(e.mem.u32(CFLUSH), 1);
        // Buffered already: 0 (the counter still counts).
        assert_eq!(e.call(0x00ed_7d48, &args![stdout]).u32(), 0);
        assert_eq!(e.mem.u32(CFLUSH), 2);
        // Not stdout or stderr: 0 without counting.
        assert_eq!(e.call(0x00ed_7d48, &args![IOB_FIRST]).u32(), 0);
        assert_eq!(e.mem.u32(CFLUSH), 2);
        e.call(0x00ed_7de4, &args![1u32, stdout]);
        assert_eq!(calls(&e, FLUSH), vec![vec![stdout]]);
        assert_eq!(e.mem.u32(stdout + 0xc), 0x2);
        assert_eq!(e.mem.u32(stdout), 0);
        e.call(0x00ed_7de4, &args![0u32, stdout]);
        assert_eq!(calls(&e, FLUSH).len(), 1);
        // stderr without memory: its own two-byte buffer.
        e.register(MALLOC_CRT, |_, _| 0u32.into_ret());
        let stderr = IOB_FIRST + 0x40;
        assert_eq!(e.call(0x00ed_7d48, &args![stderr]).u32(), 1);
        assert_eq!(e.mem.u32(stderr), stderr + 0x14);
        assert_eq!(e.mem.u32(stderr + 0x18), 2);
        // Not a terminal: 0.
        e.register(ISATTY, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ed_7d48, &args![stdout]).u32(), 0);
    }

    #[test]
    fn small_helpers_of_the_printf_core() {
        let mut e = Engine::new();
        assert_eq!(
            aulldvrm(&mut e, 1_000_000_000_007, 10),
            (100_000_000_000, 7)
        );
        assert_eq!(
            aulldvrm(&mut e, u64::MAX, 0x1_0000_0001),
            (u64::MAX / 0x1_0000_0001, u64::MAX % 0x1_0000_0001)
        );
        let q = e.call(0x00ec_c4f0, &args![100u32, 0u32, 7u32, 0u32]).u64();
        assert_eq!(q, 14);
        e.map(PRINTF_COUNT_OUTPUT, 4);
        e.map(SECURITY_COOKIE, 4);
        e.mem.set_u32(SECURITY_COOKIE, 0x1234);
        assert_eq!(e.call(0x00ec_9df9, &args![]).u32(), 0);
        e.mem.set_u32(PRINTF_COUNT_OUTPUT, 0x1235);
        assert_eq!(e.call(0x00ec_9df9, &args![]).u32(), 1);
        e.register(WCTOMB_S_L, |_, a| (a[3] + a[4]).into_ret());
        assert_eq!(
            e.call(0x00ee_2315, &args![1u32, 2u32, 3u32, 0x41u32]).i32(),
            0x41
        );
        // write_multi_char: three characters, or none for a count of 0.
        let buffer = e.mem.alloc(8);
        let stream = e.mem.alloc(0x20);
        e.mem.set_u32(stream, buffer);
        e.mem.set_u32(stream + 4, 8);
        e.mem.set_u32(stream + 8, buffer);
        let count = e.mem.alloc(4);
        e.call(0x00ed_2efb, &args![0x2au32, 3u32, stream, count]);
        e.call(0x00ed_2efb, &args![0x2bu32, 0u32, stream, count]);
        assert_eq!(e.mem.bytes(buffer, 4), b"***\0".to_vec());
        assert_eq!(e.mem.u32(count), 3);
    }
}

#[cfg(test)]
mod control_word_tests {
    use super::*;

    /// `FLDCW` then `FSTCW` as measured on the player's machine (2026-10-21).
    #[test]
    fn the_control_word_reads_back_as_the_fpu_keeps_it() {
        let mut e = Engine::new();
        for (loaded, read) in [
            (0x023fu16, 0x027fu16),
            (0x027f, 0x027f),
            (0x003f, 0x007f),
            (0x0000, 0x0040),
            (0xffff, 0x1f7f),
            (0x1f3f, 0x1f7f),
            (0x0e7f, 0x0e7f),
        ] {
            set_x87_control_word(&mut e, loaded);
            assert_eq!(x87_control_word(&mut e), read, "{loaded:#06x}");
        }
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }
}

#[cfg(test)]
mod tests_gcvt_and_iterator {
    use super::*;

    #[test]
    fn gcvt_hands_the_value_on_and_answers_the_buffer_or_null() {
        let mut e = Engine::new();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(GCVT_S, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            u32::from(a[4] == 99).into_ret()
        });
        let buffer: Ptr = Ptr::new(0x1000);
        assert_eq!(
            e.call(0x00ec_dd16, &args![1.5f64, 6i32, buffer]).u32(),
            0x1000
        );
        assert_eq!(e.call(0x00ec_dd16, &args![1.5f64, 99i32, buffer]).u32(), 0);
        let bits = 1.5f64.to_bits();
        assert_eq!(
            seen.borrow()[0],
            vec![0x1000, 0xffff_ffff, bits as u32, (bits >> 32) as u32, 6]
        );
        // A signalling NaN arrives quiet.
        e.call(
            0x00ec_dd16,
            &args![f64::from_bits(0x7ff0_0000_0000_0001), 6i32, buffer],
        );
        assert_eq!(seen.borrow()[2][3], 0x7ff8_0000);
    }

    #[test]
    fn the_vector_destructor_iterator_runs_last_first() {
        let mut e = Engine::new();
        let order = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let record = order.clone();
        e.register_double(0x7f00_0300, move |_, a| {
            record.borrow_mut().push(a[0]);
            Ret::default()
        });
        e.call(
            0x00ec_5fce,
            &args![0x2000u32, 0x10u32, 3i32, 0x7f00_0300u32],
        );
        assert_eq!(*order.borrow(), vec![0x2020, 0x2010, 0x2000]);
        order.borrow_mut().clear();
        e.call(
            0x00ec_5fce,
            &args![0x2000u32, 0x10u32, 0i32, 0x7f00_0300u32],
        );
        assert!(order.borrow().is_empty());
    }
}

#[cfg(test)]
mod tests_realloc_msize {
    use super::*;

    #[test]
    fn realloc_keeps_the_bytes_and_follows_the_exe_s_edges() {
        let mut e = Engine::new();
        let thread_data = e.mem.alloc(0x80);
        e.register_double(GETPTD_NOEXIT, move |_, _| thread_data.into_ret());
        let a = e.call(0x00ed0cdf, &args![6u32]).u32();
        e.mem.write(a, b"abcdef");
        assert_eq!(e.call(0x00ec_d31f, &args![a]).u32(), 6);
        let b = e.call(0x00ec_cf5d, &args![a, 3u32]).u32();
        assert_eq!(e.mem.bytes(b, 3), b"abc");
        assert_eq!(e.call(0x00ec_d31f, &args![b]).u32(), 3);
        let c = e.call(0x00ec_cf5d, &args![b, 0x40u32]).u32();
        assert_eq!(e.mem.bytes(c, 4), b"abc\0");
        // A null block is malloc; size 0 frees and answers null.
        let d = e.call(0x00ec_cf5d, &args![0u32, 8u32]).u32();
        assert_ne!(d, 0);
        assert_eq!(e.call(0x00ec_cf5d, &args![d, 0u32]).u32(), 0);
        assert!(!e.kernel.crt_block_sizes.contains_key(&d));
        // Too big: the new handler, ENOMEM, null, the block kept.
        let called = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let seen = called.clone();
        e.register_double(CALL_NEW_HANDLER, move |_, a| {
            seen.set(a[0]);
            Ret::default()
        });
        assert_eq!(e.call(0x00ec_cf5d, &args![c, 0xffff_fff0u32]).u32(), 0);
        assert_eq!(called.get(), 0xffff_fff0);
        assert_eq!(errno_value(&mut e), 0xc);
        assert_eq!(e.call(0x00ec_d31f, &args![c]).u32(), 0x40);
    }

    #[test]
    fn msize_of_null_is_einval_and_minus_one() {
        let mut e = Engine::new();
        let thread_data = e.mem.alloc(0x80);
        e.register_double(GETPTD_NOEXIT, move |_, _| thread_data.into_ret());
        e.register(0x00ec_7c30, |_, _| Ret::default());
        assert_eq!(e.call(0x00ec_d31f, &args![0u32]).u32(), 0xffff_ffff);
        assert_eq!(errno_value(&mut e), 0x16);
    }
}

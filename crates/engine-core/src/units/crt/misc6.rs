//! More C runtime pieces (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe 1.4.0.525),
//! translated from the exe: the wide comparisons `wcsncmp` (`00ee5a8b`), `_wcsnicmp`
//! (`00ee59e5`) and `_wcsnicmp_l` (`00ee58fb`); the C++ exception helpers `__TypeMatch`
//! (`00ece322`), `__AdjustPointer` (`00ece545`), `IsInExceptionSpec` (`00ece581`), the
//! "processing a throw" test `00ece56e`, `__DestructExceptionObject` (`00ece4f0`) with its
//! `__except` filter (`00ece533`), and `__CxxUnhandledExceptionFilter` (`00edd4a5`); and the
//! zeroing allocators `calloc` (`00eddd7d`), `_calloc_impl` (`00ee15e2`) and `_recalloc`
//! (`00ee1700`).
//!
//! Every call is made by address, as the exe makes it, so a test can double each one.

use crate::prelude::*;

/// `__locale_changed` (`0126fdf4`).
const LOCALE_CHANGED: u32 = 0x0126_fdf4;
/// `_errno` (`00ec85e3`, cdecl, no arguments) and `_invalid_parameter` (`00ec7c30`, five words).
const ERRNO: u32 = 0x00ec_85e3;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`, thiscall, one word: the locale or null) and
/// `_towlower_l` (`00edd3f0`, cdecl: character, locale object).
const LOCALE_UPDATE: u32 = 0x00ec_660e;
const TOWLOWER_L: u32 = 0x00ed_d3f0;
/// `_wcsnicmp_l` (`00ee58fb`, cdecl: first, second, count, locale).
const WCSNICMP_L: u32 = 0x00ee_58fb;
/// `strcmp` (`00ec6da0`), `__TypeMatch` (`00ece322`), `_inconsistency` (`00ed1500`),
/// `terminate` (`00ed14b4`), `_getptd` (`00ed11de`), cdecl.
const STRCMP: u32 = 0x00ec_6da0;
const TYPE_MATCH: u32 = 0x00ec_e322;
const INCONSISTENCY: u32 = 0x00ed_1500;
const TERMINATE: u32 = 0x00ed_14b4;
const GETPTD: u32 = 0x00ed_11de;
/// `_CallMemberFunction0` (Ghidra's FID: a `_CallMemberFunction` conflict, `00ec3cd8`, cdecl
/// here: object, member function).
const CALL_MEMBER_FUNCTION: u32 = 0x00ec_3cd8;
/// The MSVC C++ exception code (`'msc' | 0xE0000000`).
const CXX_EXCEPTION: u32 = 0xe06d_7363;
/// `_calloc_impl` (`00ee15e2`, cdecl: count, size, errno out), `__sbh_alloc_block`
/// (`00edea12`, cdecl, one word), `_callnewh` (`00ede20b`, cdecl, one word), `_lock`
/// (`00eddf39`), `memset` (`00ec61c0`, cdecl: destination, byte, count), `_msize`
/// (`00ecd31f`), `realloc` (`00eccf5d`, cdecl: block, size) and the heap lock's `__finally`
/// block of `_calloc_impl` (`00ee16de`, no arguments).
const CALLOC_IMPL: u32 = 0x00ee_15e2;
const SBH_ALLOC_BLOCK: u32 = 0x00ed_ea12;
const CALLNEWH: u32 = 0x00ed_e20b;
const LOCK: u32 = 0x00ed_df39;
const MEMSET: u32 = 0x00ec_61c0;
const MSIZE: u32 = 0x00ec_d31f;
const REALLOC: u32 = 0x00ec_cf5d;
const CALLOC_IMPL_FINALLY: u32 = 0x00ee_16de;
/// The `HeapAlloc` import slot (`00fdf1a0`, stdcall: heap, flags, size).
const HEAP_ALLOC: u32 = 0x00fd_f1a0;
/// `__active_heap` (`01270948`, 3 for the small-block heap), `__sbh_threshold` (`01270938`),
/// `_crtheap` (`012705bc`) and `_newmode` (`01270730`).
const ACTIVE_HEAP: u32 = 0x0127_0948;
const SBH_THRESHOLD: u32 = 0x0127_0938;
const CRT_HEAP: u32 = 0x0127_05bc;
const NEW_MODE: u32 = 0x0127_0730;

/// `errno = code` and `_invalid_parameter(0, 0, 0, 0, 0)`.
fn invalid(e: &mut Engine, code: u32) {
    let errno = e.call(ERRNO, &args![]).u32();
    e.mem.set_u32(errno, code);
    e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
}

/// The inlined ASCII lowering: `A`..=`Z` (0x41..=0x5A, unsigned 16-bit compares) plus 0x20.
fn ascii_lower(c: u16) -> u16 {
    if (0x41..=0x5a).contains(&c) {
        c.wrapping_add(0x20)
    } else {
        c
    }
}

// ---- Wide comparisons ------------------------------------------------------------------------

// Translated from 00ee5a8b (disassembled, FalloutNV.exe 1.4.0.525)
/// `wcsncmp` (LIBCMT/wcsncmp.c; cdecl: first, second, count): 0 for a zero count; otherwise
/// the units are walked while the count (decremented first) stays non-zero, the first's unit
/// is not 0 and both are equal; the answer is the difference of the units where it stopped
/// (zero-extended, first minus second).
pub fn wcsncmp(e: &mut Engine, first: u32, second: u32, count: u32) -> i32 {
    if count == 0 {
        return 0;
    }
    let (mut a, mut b, mut n) = (first, second, count);
    loop {
        n = n.wrapping_sub(1);
        if n == 0 {
            break;
        }
        let c = e.mem.u16(a);
        if c == 0 || c != e.mem.u16(b) {
            break;
        }
        a = a.wrapping_add(2);
        b = b.wrapping_add(2);
    }
    (u32::from(e.mem.u16(a)).wrapping_sub(u32::from(e.mem.u16(b)))) as i32
}

/// The ASCII loop `_wcsnicmp` and `_wcsnicmp_l` share: both units lowered, the pointers moved
/// on and the count decremented; the walk goes on while the count is not 0, the first's
/// lowered unit is not 0 and both are equal. Answers the difference of the last pair.
fn wcsnicmp_ascii(e: &mut Engine, first: u32, second: u32, count: u32) -> i32 {
    let (mut a, mut b, mut n) = (first, second, count);
    loop {
        let x = ascii_lower(e.mem.u16(a));
        let y = ascii_lower(e.mem.u16(b));
        a = a.wrapping_add(2);
        b = b.wrapping_add(2);
        n = n.wrapping_sub(1);
        if n == 0 || x == 0 || x != y {
            return (u32::from(x).wrapping_sub(u32::from(y))) as i32;
        }
    }
}

// Translated from 00ee58fb (disassembled, FalloutNV.exe 1.4.0.525)
/// `_wcsnicmp_l` (LIBCMT/wcsnicmp.c; cdecl: first, second, count, locale): 0 for a zero count;
/// a null string reports `EINVAL` and answers 0x7FFFFFFF. Otherwise, with the locale taken
/// (`_LocaleUpdate`), a locale without an `LC_CTYPE` handle (+0x14 of its info) compares with
/// the ASCII lowering, any other with `_towlower_l` on each unit; the walk stops as in
/// `wcsnicmp_ascii` and the difference of the last pair is answered. The update's own-locale
/// bit is released on the way out.
pub fn wcsnicmp_l(e: &mut Engine, first: u32, second: u32, count: u32, locale: u32) -> i32 {
    if count == 0 {
        return 0;
    }
    if first == 0 || second == 0 {
        invalid(e, 0x16);
        return 0x7fff_ffff;
    }
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let info = e.mem.u32(update);
        let result = if e.mem.u32(info.wrapping_add(0x14)) == 0 {
            wcsnicmp_ascii(e, first, second, count)
        } else {
            let (mut a, mut b, mut n) = (first, second, count);
            loop {
                let x = e
                    .call(TOWLOWER_L, &args![u32::from(e.mem.u16(a)), update])
                    .u32() as u16;
                let y = e
                    .call(TOWLOWER_L, &args![u32::from(e.mem.u16(b)), update])
                    .u32() as u16;
                a = a.wrapping_add(2);
                b = b.wrapping_add(2);
                n = n.wrapping_sub(1);
                if n == 0 || x == 0 || x != y {
                    break (u32::from(x).wrapping_sub(u32::from(y))) as i32;
                }
            }
        };
        if e.mem.u8(update.wrapping_add(0xc)) != 0 {
            let thread = e.mem.u32(update.wrapping_add(8));
            let own = e.mem.u32(thread.wrapping_add(0x70)) & 0xffff_fffd;
            e.mem.set_u32(thread.wrapping_add(0x70), own);
        }
        result
    })
}

// Translated from 00ee59e5 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_wcsnicmp` (LIBCMT/wcsnicmp.c; cdecl: first, second, count): once the locale has changed,
/// `_wcsnicmp_l` with a null locale. Otherwise 0 for a zero count, `EINVAL` and 0x7FFFFFFF
/// for a null string, else the ASCII comparison.
pub fn wcsnicmp(e: &mut Engine, first: u32, second: u32, count: u32) -> i32 {
    if e.mem.u32(LOCALE_CHANGED) != 0 {
        return e.call(WCSNICMP_L, &args![first, second, count, 0u32]).u32() as i32;
    }
    if count == 0 {
        return 0;
    }
    if first == 0 || second == 0 {
        invalid(e, 0x16);
        return 0x7fff_ffff;
    }
    wcsnicmp_ascii(e, first, second, count)
}

// ---- C++ exception helpers -------------------------------------------------------------------

// Translated from 00ece322 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__TypeMatch` (LIBCMT/frame.cpp; cdecl: handler type, catchable type, throw information):
/// 1 for a catch-all (no type descriptor at +4, or an empty name at descriptor +8). Otherwise
/// 0 when the descriptors differ and their names compare unequal (`strcmp`), when the
/// catchable type is by reference (bit 2 of its attributes) and the handler is not (bit 8),
/// or when the thrown object is const (bit 1 of the throw attributes) or volatile (bit 2) and
/// the handler is not; else 1.
pub fn type_match(e: &mut Engine, handler: u32, catchable: u32, throw_info: u32) -> u32 {
    let descriptor = e.mem.u32(handler.wrapping_add(4));
    if descriptor == 0 {
        return 1;
    }
    let name = descriptor.wrapping_add(8);
    if e.mem.u8(name) == 0 {
        return 1;
    }
    let other = e.mem.u32(catchable.wrapping_add(4));
    if descriptor != other && e.call(STRCMP, &args![name, other.wrapping_add(8)]).u32() != 0 {
        return 0;
    }
    if e.mem.u8(catchable) & 2 != 0 && e.mem.u8(handler) & 8 == 0 {
        return 0;
    }
    let attributes = e.mem.u32(throw_info) as u8;
    if attributes & 1 != 0 && e.mem.u8(handler) & 1 == 0 {
        return 0;
    }
    if attributes & 2 != 0 && e.mem.u8(handler) & 2 == 0 {
        return 0;
    }
    1
}

// Translated from 00ece545 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__AdjustPointer` (LIBCMT/frame.cpp; cdecl: object, `PMD`): the object plus `mdisp` (+0);
/// when `pdisp` (+4) is not negative, plus the word at `vdisp` (+8) into the virtual base table
/// the object holds at `pdisp`, plus `pdisp`.
pub fn adjust_pointer(e: &mut Engine, object: u32, pmd: u32) -> u32 {
    let mut adjusted = e.mem.u32(pmd).wrapping_add(object);
    if (e.mem.u32(pmd.wrapping_add(4)) as i32) >= 0 {
        let pdisp = e.mem.u32(pmd.wrapping_add(4));
        let vdisp = e.mem.u32(pmd.wrapping_add(8));
        let table = e.mem.u32(pdisp.wrapping_add(object));
        let offset = e.mem.u32(table.wrapping_add(vdisp)).wrapping_add(pdisp);
        adjusted = adjusted.wrapping_add(offset);
    }
    adjusted
}

// Translated from 00ece56e (disassembled, FalloutNV.exe 1.4.0.525)
/// No Ghidra name (cdecl, no arguments): whether the thread's data (`_getptd`) has a throw
/// in progress (`_ProcessingThrow`, +0x90, not 0), answered in `AL`.
pub fn processing_throw(e: &mut Engine) -> u8 {
    let ptd = e.call(GETPTD, &args![]).u32();
    u8::from(e.mem.u32(ptd.wrapping_add(0x90)) != 0)
}

// Translated from 00ece581 (disassembled, FalloutNV.exe 1.4.0.525)
/// `IsInExceptionSpec` (LIBCMT/frame.cpp; the exception specification in `EDI`, then the
/// stack word: the exception record). A null specification calls `_inconsistency` and
/// `terminate` (neither returns in the exe; the code after them is the main path). Each of the
/// specification's handler types (`+0` the count, `+4` the 16-byte entries) is matched
/// (`__TypeMatch`) against the thrown object's catchable types (`[record+0x1C]` throw
/// information, its `+0xC` array: a count then the types) until one matches; the outer walk
/// always goes on to the end. Answers in `AL` whether any matched.
pub fn is_in_exception_spec(e: &mut Engine, spec: u32, record: u32) -> u8 {
    if spec == 0 {
        e.call(INCONSISTENCY, &args![]);
        e.call(TERMINATE, &args![]);
    }
    let mut index = 0u32;
    let mut found = 0u8;
    if (e.mem.u32(spec) as i32) > 0 {
        loop {
            let throw_info = e.mem.u32(record.wrapping_add(0x1c));
            let array = e.mem.u32(throw_info.wrapping_add(0xc));
            let mut left = e.mem.u32(array) as i32;
            let mut slot = array.wrapping_add(4);
            if left > 0 {
                let offset = index << 4;
                loop {
                    let throw_info = e.mem.u32(record.wrapping_add(0x1c));
                    let catchable = e.mem.u32(slot);
                    let handler = e.mem.u32(spec.wrapping_add(4)).wrapping_add(offset);
                    if e.call(TYPE_MATCH, &args![handler, catchable, throw_info])
                        .u32()
                        != 0
                    {
                        found = 1;
                        break;
                    }
                    left = left.wrapping_sub(1);
                    slot = slot.wrapping_add(4);
                    if left <= 0 {
                        break;
                    }
                }
            }
            index = index.wrapping_add(1);
            if (index as i32) >= (e.mem.u32(spec) as i32) {
                break;
            }
        }
    }
    found
}

// Translated from 00ece4f0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ece4f0 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 01180040) and its __except block are not translated: the filter 00ece533 takes any exception from the destructor when the throw flag is set and the handler 00ece53c calls terminate; the engine raises no SEH exceptions.
/// `__DestructExceptionObject` (LIBCMT/frame.cpp; cdecl: exception record, throw flag): for a
/// C++ exception (code 0xE06D7363) whose throw information (+0x1C) names a destructor (+4),
/// the destructor runs on the thrown object (+0x18) through `_CallMemberFunction0`.
pub fn destruct_exception_object(e: &mut Engine, record: u32, _throw_flag: u32) {
    if record == 0 || e.mem.u32(record) != CXX_EXCEPTION {
        return;
    }
    let throw_info = e.mem.u32(record.wrapping_add(0x1c));
    if throw_info == 0 {
        return;
    }
    let destructor = e.mem.u32(throw_info.wrapping_add(4));
    if destructor == 0 {
        return;
    }
    let object = e.mem.u32(record.wrapping_add(0x18));
    e.call(CALL_MEMBER_FUNCTION, &args![object, destructor]);
}

// Translated from 00ece533 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__except` filter of `__DestructExceptionObject` (`00ece4f0`): its frame word
/// `[EBP+0xC]` (the throw flag) is the argument; answers 1 (`EXCEPTION_EXECUTE_HANDLER`) when
/// its low byte is not 0, else 0.
pub fn destruct_exception_object_filter(_e: &mut Engine, throw_flag: u32) -> u32 {
    u32::from(throw_flag as u8 != 0)
}

// Translated from 00edd4a5 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__CxxUnhandledExceptionFilter` (LIBCMT/unhandld.cpp; stdcall, one word: the exception
/// pointers): a C++ exception (code 0xE06D7363, three parameters) with a known magic number
/// (0x19930520, 0x19930521, 0x19930522 or 0x01994000) calls `terminate`. Answers 0
/// (`EXCEPTION_CONTINUE_SEARCH`).
pub fn cxx_unhandled_exception_filter(e: &mut Engine, pointers: u32) -> u32 {
    let record = e.mem.u32(pointers);
    if e.mem.u32(record) == CXX_EXCEPTION && e.mem.u32(record.wrapping_add(0x10)) == 3 {
        let magic = e.mem.u32(record.wrapping_add(0x14));
        if magic == 0x1993_0520
            || magic == 0x1993_0521
            || magic == 0x1993_0522
            || magic == 0x0199_4000
        {
            e.call(TERMINATE, &args![]);
        }
    }
    0
}

// ---- Zeroing allocators ----------------------------------------------------------------------

// Translated from 00eddd7d (disassembled, FalloutNV.exe 1.4.0.525)
/// `calloc` (LIBCMT/calloc.c; cdecl: count, size): `_calloc_impl` with a local error word
/// (0 at first); when that answers null with the word set and `_errno()` is not null, the
/// word goes to `errno`. Answers the block.
pub fn calloc(e: &mut Engine, count: u32, size: u32) -> u32 {
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, 0);
        let block = e.call(CALLOC_IMPL, &args![count, size, local]).u32();
        if block == 0 && e.mem.u32(local) != 0 && e.call(ERRNO, &args![]).u32() != 0 {
            let errno = e.call(ERRNO, &args![]).u32();
            let code = e.mem.u32(local);
            e.mem.set_u32(errno, code);
        }
        block
    })
}

// Translated from 00ee15e2 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee15e2 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 01180508) is not translated; its __finally block (00ee16de) is called as the exe calls it on the way out of the locked part.
/// `_calloc_impl` (LIBCMT/calloc.c; cdecl: count, size, error out). A product over 0xFFFFFFE0
/// (`0xFFFFFFE0 / count < size` for a non-zero count) sets `errno` to `ENOMEM` (0xC), reports
/// the invalid parameter and answers 0. The byte count (`count * size`, 1 for 0) is then tried
/// until it is had: when it is at most 0xFFFFFFE0, on the small-block heap (`__active_heap`
/// 3) it is rounded up to 16 and a product at most `__sbh_threshold` comes from
/// `__sbh_alloc_block` under the heap lock (zeroed with `memset` over the product); failing
/// that `HeapAlloc(_crtheap, HEAP_ZERO_MEMORY, bytes)`. Without a block, with `_newmode` set
/// and `_callnewh(bytes)` succeeding it tries again; otherwise `ENOMEM` goes to the error word
/// when there is one and 0 is answered.
pub fn calloc_impl(e: &mut Engine, count: u32, size: u32, error: u32) -> u32 {
    if count != 0 && 0xffff_ffe0 / count < size {
        invalid(e, 0xc);
        return 0;
    }
    let total = count.wrapping_mul(size);
    let mut bytes = total;
    if bytes == 0 {
        bytes = 1;
    }
    loop {
        let mut block = 0u32;
        if bytes <= 0xffff_ffe0 {
            if e.mem.u32(ACTIVE_HEAP) == 3 {
                bytes = bytes.wrapping_add(0xf) & 0xffff_fff0;
                if total <= e.mem.u32(SBH_THRESHOLD) {
                    e.call(LOCK, &args![4u32]);
                    block = e.call(SBH_ALLOC_BLOCK, &args![total]).u32();
                    e.call(CALLOC_IMPL_FINALLY, &args![]);
                    if block != 0 {
                        e.call(MEMSET, &args![block, 0u32, total]);
                    }
                }
            }
            if block == 0 {
                let heap = e.mem.u32(CRT_HEAP);
                block = e.call(HEAP_ALLOC, &args![heap, 8u32, bytes]).u32();
            }
        }
        if block != 0 {
            return block;
        }
        if e.mem.u32(NEW_MODE) != 0 {
            if e.call(CALLNEWH, &args![bytes]).u32() != 0 {
                continue;
            }
            if error != 0 {
                e.mem.set_u32(error, 0xc);
            }
            return 0;
        }
        if error != 0 {
            e.mem.set_u32(error, 0xc);
        }
        return block;
    }
}

// Translated from 00ee1700 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_recalloc` (LIBCMT/recalloc.c; cdecl: block, count, size): an overflowing product
/// (`0xFFFFFFE0 / count < size`) sets `ENOMEM`, reports the invalid parameter and answers 0.
/// Otherwise the old size (`_msize`, 0 for a null block) is taken, the block resized with
/// `realloc` and, when that gives a block and it grew, the new tail cleared (`memset`).
/// Answers the new block.
pub fn recalloc(e: &mut Engine, block: u32, count: u32, size: u32) -> u32 {
    if count != 0 && 0xffff_ffe0 / count < size {
        invalid(e, 0xc);
        return 0;
    }
    let total = count.wrapping_mul(size);
    let old = if block != 0 {
        e.call(MSIZE, &args![block]).u32()
    } else {
        0
    };
    let new = e.call(REALLOC, &args![block, total]).u32();
    if new != 0 && old < total {
        e.call(
            MEMSET,
            &args![old.wrapping_add(new), 0u32, total.wrapping_sub(old)],
        );
    }
    new
}

// ---- Command line and environment ------------------------------------------------------------

/// `__mbctype_initialized` (`01271a90`), `___initmbctable` (`00ed417e`, no arguments),
/// `_ismbblead` (`00ed5c8d`, cdecl, one word), `_acmdln` (`01271a98`) and the empty string
/// `__wincmdln` falls back to (`01011584`).
const MBCTYPE_INITIALIZED: u32 = 0x0127_1a90;
const INITMBCTABLE: u32 = 0x00ed_417e;
const ISMBBLEAD: u32 = 0x00ed_5c8d;
const ACMDLN: u32 = 0x0127_1a98;
const EMPTY_TEXT: u32 = 0x0101_1584;
/// `_aenvptr` (`0126fdac`), `_environ` (`0126fdc8`), `__env_initialized` (`01271a84`);
/// `strlen` (`00ec6130`), `_calloc_crt` (`00ed0d24`, cdecl: count, size), `strcpy_s`
/// (`00ec65a6`, cdecl: destination, size, source), `_invoke_watson` (`00ec7b08`, five words)
/// and `free` (`00ecd291`).
const AENVPTR: u32 = 0x0126_fdac;
const ENVIRON: u32 = 0x0126_fdc8;
const ENV_INITIALIZED: u32 = 0x0127_1a84;
const STRLEN: u32 = 0x00ec_6130;
const CALLOC_CRT: u32 = 0x00ed_0d24;
const STRCPY_S: u32 = 0x00ec_65a6;
const INVOKE_WATSON: u32 = 0x00ec_7b08;
const FREE: u32 = 0x00ec_d291;

/// `INC dword ptr [at]`.
fn count_up(e: &mut Engine, at: u32) {
    let n = e.mem.u32(at).wrapping_add(1);
    e.mem.set_u32(at, n);
}

// Translated from 00edd655 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_wincmdln` (LIBCMT/wincmdln.c; no arguments): the command line `_acmdln` (the empty string
/// when null) past the program name: `___initmbctable` first when the multibyte table is not
/// set up; the name runs to the first byte at most 0x20 outside double quotes (a quote
/// toggles; a lead byte takes its trail byte along), then blanks (1..=0x20) are skipped.
/// Answers the address reached.
pub fn wincmdln(e: &mut Engine) -> u32 {
    if e.mem.u32(MBCTYPE_INITIALIZED) == 0 {
        e.call(INITMBCTABLE, &args![]);
    }
    let mut p = e.mem.u32(ACMDLN);
    if p == 0 {
        p = EMPTY_TEXT;
    }
    let mut quoted = false;
    loop {
        let c = e.mem.u8(p);
        if c <= 0x20 {
            if c == 0 {
                return p;
            }
            if !quoted {
                break;
            }
        }
        if c == 0x22 {
            quoted = !quoted;
        }
        if e.call(ISMBBLEAD, &args![u32::from(c)]).u32() != 0 {
            p = p.wrapping_add(1);
        }
        p = p.wrapping_add(1);
    }
    loop {
        let c = e.mem.u8(p);
        if c == 0 || c > 0x20 {
            return p;
        }
        p = p.wrapping_add(1);
    }
}

// Translated from 00edd792 (disassembled, FalloutNV.exe 1.4.0.525)
/// `parse_cmdline` (LIBCMT/stdargv.c; the command line in `EDX`, the character count's
/// address in `EDI`, then the stack words: the `argv` table, the argument text buffer, the
/// argument count's address). Either output may be null, which only counts. The program name
/// runs to a blank outside quotes (quotes toggle and are dropped, a lead byte takes its trail
/// byte). Each later argument follows the C rules: `2n` backslashes before a quote give `n` and
/// the quote toggles quoting (inside quotes a doubled quote gives one quote), `2n+1` give `n`
/// and a literal quote, other backslashes are literal; blanks outside quotes end it. The
/// `argv` table ends with a null entry and the count includes it.
pub fn parse_cmdline(
    e: &mut Engine,
    cmdstart: u32,
    numchars: u32,
    argv: u32,
    args: u32,
    numargs: u32,
) {
    let mut p = cmdstart;
    let mut argv = argv;
    // `EDX` and the frame word `[EBP+0xC]` both hold the buffer position; they are kept apart
    // where the exe updates one without the other.
    let mut edx = args;
    let mut args_word = args;
    e.mem.set_u32(numchars, 0);
    e.mem.set_u32(numargs, 1);
    if argv != 0 {
        e.mem.set_u32(argv, edx);
        argv = argv.wrapping_add(4);
    }
    let mut quoted = 0u32;
    // The program name.
    loop {
        let c = if e.mem.u8(p) == 0x22 {
            quoted = u32::from(quoted == 0);
            p = p.wrapping_add(1);
            0x22
        } else {
            count_up(e, numchars);
            if edx != 0 {
                let byte = e.mem.u8(p);
                e.mem.set_u8(edx, byte);
                edx = edx.wrapping_add(1);
                args_word = edx;
            }
            let c = e.mem.u8(p);
            p = p.wrapping_add(1);
            if e.call(ISMBBLEAD, &args![u32::from(c)]).u32() != 0 {
                count_up(e, numchars);
                if args_word != 0 {
                    let byte = e.mem.u8(p);
                    e.mem.set_u8(args_word, byte);
                    args_word = args_word.wrapping_add(1);
                }
                p = p.wrapping_add(1);
            }
            edx = args_word;
            if c == 0 {
                p = p.wrapping_sub(1);
                break;
            }
            c
        };
        if quoted != 0 {
            continue;
        }
        if c == 0x20 || c == 9 {
            if edx != 0 {
                e.mem.set_u8(edx.wrapping_sub(1), 0);
            }
            break;
        }
    }
    // The arguments.
    loop {
        quoted = 0;
        if e.mem.u8(p) == 0 {
            break;
        }
        while e.mem.u8(p) == 0x20 || e.mem.u8(p) == 9 {
            p = p.wrapping_add(1);
        }
        if e.mem.u8(p) == 0 {
            break;
        }
        if argv != 0 {
            e.mem.set_u32(argv, edx);
            argv = argv.wrapping_add(4);
        }
        count_up(e, numargs);
        loop {
            let mut copy = true;
            let mut slashes = 0u32;
            while e.mem.u8(p) == 0x5c {
                p = p.wrapping_add(1);
                slashes = slashes.wrapping_add(1);
            }
            if e.mem.u8(p) == 0x22 {
                if slashes & 1 == 0 {
                    if quoted != 0 && e.mem.u8(p.wrapping_add(1)) == 0x22 {
                        p = p.wrapping_add(1);
                    } else {
                        copy = false;
                        quoted = u32::from(quoted == 0);
                    }
                }
                slashes >>= 1;
            }
            if slashes != 0 {
                loop {
                    slashes -= 1;
                    if edx != 0 {
                        e.mem.set_u8(edx, 0x5c);
                        edx = edx.wrapping_add(1);
                    }
                    count_up(e, numchars);
                    if slashes == 0 {
                        break;
                    }
                }
                args_word = edx;
            }
            let c = e.mem.u8(p);
            if c == 0 || (quoted == 0 && (c == 0x20 || c == 9)) {
                break;
            }
            if copy {
                // MOVSX EAX, AL: the byte is passed sign-extended here.
                let word = c as i8 as i32 as u32;
                if edx != 0 {
                    if e.call(ISMBBLEAD, &args![word]).u32() != 0 {
                        let byte = e.mem.u8(p);
                        e.mem.set_u8(args_word, byte);
                        args_word = args_word.wrapping_add(1);
                        p = p.wrapping_add(1);
                        count_up(e, numchars);
                    }
                    let byte = e.mem.u8(p);
                    e.mem.set_u8(args_word, byte);
                    args_word = args_word.wrapping_add(1);
                } else if e.call(ISMBBLEAD, &args![word]).u32() != 0 {
                    p = p.wrapping_add(1);
                    count_up(e, numchars);
                }
                count_up(e, numchars);
                edx = args_word;
            }
            p = p.wrapping_add(1);
        }
        if edx != 0 {
            e.mem.set_u8(edx, 0);
            edx = edx.wrapping_add(1);
            args_word = edx;
        }
        count_up(e, numchars);
    }
    if argv != 0 {
        e.mem.set_u32(argv, 0);
    }
    count_up(e, numargs);
}

// Translated from 00edd6b4 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_setenvp` (LIBCMT/stdenvp.c; no arguments): `___initmbctable` first when needed; -1 when
/// the environment block `_aenvptr` is null. Its strings not starting with `=` are counted, a
/// table of that many plus one words made (`_calloc_crt`; -1 when that fails) and stored in
/// `_environ`, and each such string copied into its own block (`_calloc_crt(len + 1, 1)`,
/// `strcpy_s`; a failing copy calls `_invoke_watson`). A failing block frees the table, clears
/// `_environ` and answers -1. At the end the block is freed and `_aenvptr` cleared, the table
/// ended with a null word, `__env_initialized` set and 0 answered.
pub fn setenvp(e: &mut Engine) -> i32 {
    if e.mem.u32(MBCTYPE_INITIALIZED) == 0 {
        e.call(INITMBCTABLE, &args![]);
    }
    let mut p = e.mem.u32(AENVPTR);
    if p == 0 {
        return -1;
    }
    let mut count = 0u32;
    loop {
        let c = e.mem.u8(p);
        if c == 0 {
            break;
        }
        if c != 0x3d {
            count = count.wrapping_add(1);
        }
        let len = e.call(STRLEN, &args![p]).u32();
        p = p.wrapping_add(len).wrapping_add(1);
    }
    let table = e
        .call(CALLOC_CRT, &args![count.wrapping_add(1), 4u32])
        .u32();
    e.mem.set_u32(ENVIRON, table);
    if table == 0 {
        return -1;
    }
    let mut slot = table;
    let mut p = e.mem.u32(AENVPTR);
    while e.mem.u8(p) != 0 {
        let size = e.call(STRLEN, &args![p]).u32().wrapping_add(1);
        if e.mem.u8(p) != 0x3d {
            let copy = e.call(CALLOC_CRT, &args![size, 1u32]).u32();
            e.mem.set_u32(slot, copy);
            if copy == 0 {
                let table = e.mem.u32(ENVIRON);
                e.call(FREE, &args![table]);
                e.mem.set_u32(ENVIRON, 0);
                return -1;
            }
            if e.call(STRCPY_S, &args![copy, size, p]).u32() != 0 {
                e.call(INVOKE_WATSON, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
            }
            slot = slot.wrapping_add(4);
        }
        p = p.wrapping_add(size);
    }
    let block = e.mem.u32(AENVPTR);
    e.call(FREE, &args![block]);
    e.mem.set_u32(AENVPTR, 0);
    e.mem.set_u32(slot, 0);
    e.mem.set_u32(ENV_INITIALIZED, 1);
    0
}

/// This file's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_5a8b, wcsncmp(u32, u32, u32) -> i32),
        entry!(0x00ee_58fb, wcsnicmp_l(u32, u32, u32, u32) -> i32),
        entry!(0x00ee_59e5, wcsnicmp(u32, u32, u32) -> i32),
        entry!(0x00ec_e322, type_match(u32, u32, u32) -> u32),
        entry!(0x00ec_e545, adjust_pointer(u32, u32) -> u32),
        entry!(0x00ec_e56e, processing_throw() -> u8),
        entry!(0x00ec_e581, is_in_exception_spec(u32, u32) -> u8),
        entry!(0x00ec_e4f0, destruct_exception_object(u32, u32)),
        entry!(0x00ec_e533, destruct_exception_object_filter(u32) -> u32),
        entry!(0x00ed_d4a5, cxx_unhandled_exception_filter(u32) -> u32),
        entry!(0x00ed_dd7d, calloc(u32, u32) -> u32),
        entry!(0x00ee_15e2, calloc_impl(u32, u32, u32) -> u32),
        entry!(0x00ee_1700, recalloc(u32, u32, u32) -> u32),
        entry!(0x00ed_d655, wincmdln() -> u32),
        entry!(0x00ed_d792, parse_cmdline(u32, u32, u32, u32, u32)),
        entry!(0x00ed_d6b4, setenvp() -> i32),
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

    fn nothing(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    fn wide(e: &mut Engine, s: &str) -> u32 {
        let units: Vec<u16> = s.encode_utf16().collect();
        let at = e.mem.alloc(units.len() as u32 * 2 + 2);
        for (i, u) in units.iter().enumerate() {
            e.mem.set_u16(at + i as u32 * 2, *u);
        }
        at
    }

    /// An engine with `_errno` and `_invalid_parameter` doubled and the CRT words mapped;
    /// answers it and the `errno` word.
    fn crt_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let errno = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| errno.into_ret());
        nothing(&mut e, &[INVALID_PARAMETER]);
        e.map(0x0126_f000, 0x1000);
        e.map(0x0127_0000, 0x1000);
        (e, errno)
    }

    #[test]
    fn wcsncmp_stops_at_the_count_a_zero_or_a_difference() {
        let mut e = Engine::new();
        let a = wide(&mut e, "abcd");
        let b = wide(&mut e, "abxd");
        assert_eq!(e.call(0x00ee_5a8b, &[a, b, 0]).i32(), 0);
        assert_eq!(e.call(0x00ee_5a8b, &[a, b, 2]).i32(), 0);
        let cx = i32::from(b'c') - i32::from(b'x');
        assert_eq!(e.call(0x00ee_5a8b, &[a, b, 3]).i32(), cx);
        assert_eq!(e.call(0x00ee_5a8b, &[b, a, 9]).i32(), -cx);
        let c = wide(&mut e, "ab");
        assert_eq!(e.call(0x00ee_5a8b, &[c, a, 9]).i32(), -i32::from(b'c'));
        assert_eq!(e.call(0x00ee_5a8b, &[a, a, 9]).i32(), 0);
    }

    #[test]
    fn wcsnicmp_lowers_ascii_until_the_locale_changes() {
        let (mut e, errno) = crt_engine();
        e.mem.set_u32(LOCALE_CHANGED, 0);
        let a = wide(&mut e, "HeLLo[");
        let b = wide(&mut e, "hello{");
        assert_eq!(e.call(0x00ee_59e5, &[a, b, 5]).i32(), 0);
        // '[' (0x5B) is not lowered.
        assert_eq!(e.call(0x00ee_59e5, &[a, b, 6]).i32(), 0x5b - 0x7b);
        assert_eq!(e.call(0x00ee_59e5, &[a, b, 0]).i32(), 0);
        assert_eq!(e.call(0x00ee_59e5, &[0, b, 0]).i32(), 0);
        assert_eq!(e.call(0x00ee_59e5, &[a, 0, 3]).i32(), 0x7fff_ffff);
        assert_eq!(e.mem.u32(errno), 0x16);
        e.mem.set_u32(LOCALE_CHANGED, 1);
        e.register(WCSNICMP_L, |_, a| (a[2] + a[3] + 0x100).into_ret());
        assert_eq!(e.call(0x00ee_59e5, &[a, b, 5]).i32(), 0x105);
    }

    #[test]
    fn wcsnicmp_l_uses_towlower_l_for_a_ctype_locale() {
        let (mut e, errno) = crt_engine();
        let info = e.mem.alloc(0x100);
        e.mem.set_u32(info + 0x14, 1);
        let thread = e.mem.alloc(0x80);
        e.mem.set_u32(thread + 0x70, 0xff);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u32(a[0] + 8, thread);
            e.mem.set_u8(a[0] + 0xc, 1);
            a[0].into_ret()
        });
        // Lowers by setting bit 5 of every unit, so '[' meets '{'.
        e.register(TOWLOWER_L, |_, a| (a[0] | 0x20).into_ret());
        let a = wide(&mut e, "HeLLo[");
        let b = wide(&mut e, "hello{");
        assert_eq!(e.call(0x00ee_58fb, &[a, b, 6, 0]).i32(), 0);
        assert_eq!(e.mem.u32(thread + 0x70), 0xfd);
        // Without an LC_CTYPE handle: the ASCII lowering, '[' kept.
        e.mem.set_u32(info + 0x14, 0);
        assert_eq!(e.call(0x00ee_58fb, &[a, b, 6, 0]).i32(), 0x5b - 0x7b);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_58fb, &[a, b, 0, 0]).i32(), 0);
        assert_eq!(e.call(0x00ee_58fb, &[0, b, 2, 0]).i32(), 0x7fff_ffff);
        assert_eq!(e.mem.u32(errno), 0x16);
        assert!(calls(&e, LOCALE_UPDATE).is_empty());
    }

    #[test]
    fn type_match_follows_names_and_qualifiers() {
        let mut e = Engine::new();
        e.register(STRCMP, |e, a| {
            u32::from(e.mem.cstr(a[0]) != e.mem.cstr(a[1])).into_ret()
        });
        let named = |e: &mut Engine, name: &[u8]| {
            let d = e.mem.alloc(0x20);
            e.mem.write(d + 8, name);
            d
        };
        let int_a = named(&mut e, b".H\0");
        let int_b = named(&mut e, b".H\0");
        let float = named(&mut e, b".M\0");
        let empty = named(&mut e, b"\0");
        let handler = e.mem.alloc(0x10);
        let catchable = e.mem.alloc(0x10);
        let throw_info = e.mem.alloc(0x10);
        e.mem.set_u32(catchable + 4, int_a);
        let t = [handler, catchable, throw_info];
        // Catch-alls.
        assert_eq!(e.call(0x00ec_e322, &t).u32(), 1);
        e.mem.set_u32(handler + 4, empty);
        assert_eq!(e.call(0x00ec_e322, &t).u32(), 1);
        e.mem.set_u32(handler + 4, float);
        assert_eq!(e.call(0x00ec_e322, &t).u32(), 0);
        e.mem.set_u32(handler + 4, int_b);
        assert_eq!(e.call(0x00ec_e322, &t).u32(), 1);
        // By reference needs a reference handler; const and volatile need theirs.
        e.mem.set_u32(catchable, 2);
        assert_eq!(e.call(0x00ec_e322, &t).u32(), 0);
        e.mem.set_u32(handler, 8);
        assert_eq!(e.call(0x00ec_e322, &t).u32(), 1);
        e.mem.set_u32(throw_info, 3);
        assert_eq!(e.call(0x00ec_e322, &t).u32(), 0);
        e.mem.set_u32(handler, 9);
        assert_eq!(e.call(0x00ec_e322, &t).u32(), 0);
        e.mem.set_u32(handler, 0xb);
        assert_eq!(e.call(0x00ec_e322, &t).u32(), 1);
    }

    #[test]
    fn adjust_pointer_adds_the_member_and_virtual_base_offsets() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        let table = e.mem.alloc(0x10);
        let pmd = e.mem.alloc(12);
        e.mem.set_u32(pmd, 0x10);
        e.mem.set_u32(pmd + 4, u32::MAX);
        assert_eq!(e.call(0x00ec_e545, &[object, pmd]).u32(), object + 0x10);
        e.mem.set_u32(pmd + 4, 8);
        e.mem.set_u32(pmd + 8, 4);
        e.mem.set_u32(object + 8, table);
        e.mem.set_u32(table + 4, 0x20);
        assert_eq!(
            e.call(0x00ec_e545, &[object, pmd]).u32(),
            object + 0x10 + 0x20 + 8
        );
    }

    #[test]
    fn processing_throw_reads_the_thread_word() {
        let mut e = Engine::new();
        let ptd = e.mem.alloc(0x100);
        e.register_double(GETPTD, move |_, _| ptd.into_ret());
        assert_eq!(e.call(0x00ec_e56e, &[]).u32(), 0);
        e.mem.set_u32(ptd + 0x90, 5);
        assert_eq!(e.call(0x00ec_e56e, &[]).u32(), 1);
    }

    #[test]
    fn is_in_exception_spec_matches_every_handler_against_the_catchables() {
        let mut e = Engine::new();
        // Matches when the handler entry's first word equals the catchable word.
        e.register(TYPE_MATCH, |e, a| {
            u32::from(e.mem.u32(a[0]) == a[1]).into_ret()
        });
        nothing(&mut e, &[INCONSISTENCY, TERMINATE]);
        let handlers = e.mem.alloc(0x40);
        let spec = e.mem.alloc(8);
        e.mem.set_u32(spec, 2);
        e.mem.set_u32(spec + 4, handlers);
        e.mem.set_u32(handlers, 7);
        e.mem.set_u32(handlers + 0x10, 9);
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 2);
        e.mem.set_u32(array + 4, 9);
        e.mem.set_u32(array + 8, 3);
        let throw_info = e.mem.alloc(0x10);
        e.mem.set_u32(throw_info + 0xc, array);
        let record = e.mem.alloc(0x20);
        e.mem.set_u32(record + 0x1c, throw_info);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_e581, &[spec, record]).u32(), 1);
        // Handler 0 against both catchables, handler 1 stops at its first match.
        assert_eq!(
            calls(&e, TYPE_MATCH),
            vec![
                vec![handlers, 9, throw_info],
                vec![handlers, 3, throw_info],
                vec![handlers + 0x10, 9, throw_info],
            ]
        );
        e.mem.set_u32(handlers + 0x10, 8);
        assert_eq!(e.call(0x00ec_e581, &[spec, record]).u32(), 0);
        e.mem.set_u32(spec, 0);
        assert_eq!(e.call(0x00ec_e581, &[spec, record]).u32(), 0);
        assert!(calls(&e, TERMINATE).is_empty());
    }

    #[test]
    fn destruct_exception_object_runs_the_thrown_destructor() {
        let mut e = Engine::new();
        nothing(&mut e, &[CALL_MEMBER_FUNCTION]);
        let record = e.mem.alloc(0x20);
        let throw_info = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x00ec_e4f0, &[0, 1]);
        e.call(0x00ec_e4f0, &[record, 1]);
        e.mem.set_u32(record, CXX_EXCEPTION);
        e.call(0x00ec_e4f0, &[record, 1]);
        e.mem.set_u32(record + 0x1c, throw_info);
        e.call(0x00ec_e4f0, &[record, 1]);
        assert!(calls(&e, CALL_MEMBER_FUNCTION).is_empty());
        e.mem.set_u32(throw_info + 4, 0x00a0_0000);
        e.mem.set_u32(record + 0x18, 0x0600_0000);
        e.call(0x00ec_e4f0, &[record, 0]);
        assert_eq!(
            calls(&e, CALL_MEMBER_FUNCTION),
            vec![vec![0x0600_0000, 0x00a0_0000]]
        );
        assert_eq!(e.call(0x00ec_e533, &[0x100]).u32(), 0);
        assert_eq!(e.call(0x00ec_e533, &[1]).u32(), 1);
    }

    #[test]
    fn the_unhandled_filter_terminates_only_known_cxx_exceptions() {
        let mut e = Engine::new();
        nothing(&mut e, &[TERMINATE]);
        let record = e.mem.alloc(0x20);
        let pointers = e.mem.alloc(8);
        e.mem.set_u32(pointers, record);
        e.call_log = Some(vec![]);
        for (code, params, magic, ends) in [
            (CXX_EXCEPTION, 3, 0x1993_0520, true),
            (CXX_EXCEPTION, 3, 0x1993_0521, true),
            (CXX_EXCEPTION, 3, 0x1993_0522, true),
            (CXX_EXCEPTION, 3, 0x0199_4000, true),
            (CXX_EXCEPTION, 3, 0x1993_0523, false),
            (CXX_EXCEPTION, 4, 0x1993_0520, false),
            (0xc000_0005, 3, 0x1993_0520, false),
        ] {
            e.mem.set_u32(record, code);
            e.mem.set_u32(record + 0x10, params);
            e.mem.set_u32(record + 0x14, magic);
            let before = calls(&e, TERMINATE).len();
            assert_eq!(e.call(0x00ed_d4a5, &[pointers]).u32(), 0);
            assert_eq!(calls(&e, TERMINATE).len() - before, usize::from(ends));
        }
    }

    #[test]
    fn calloc_impl_tries_the_small_block_heap_then_heap_alloc_then_the_new_handler() {
        let (mut e, errno) = crt_engine();
        nothing(&mut e, &[LOCK, MEMSET, 0x00ed_de5f]);
        let error = e.mem.alloc(4);
        // Overflow.
        assert_eq!(e.call(0x00ee_15e2, &[0x10, 0x1000_0000, error]).u32(), 0);
        assert_eq!(e.mem.u32(errno), 0xc);
        // HeapAlloc with zeroing, 1 byte for 0.
        e.register(HEAP_ALLOC, |_, a| (a[2] | 0x0700_0000).into_ret());
        e.mem.set_u32(CRT_HEAP, 0x55);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_15e2, &[0, 4, error]).u32(), 0x0700_0001);
        assert_eq!(calls(&e, HEAP_ALLOC), vec![vec![0x55, 8, 1]]);
        // The small-block heap: rounded to 16 for HeapAlloc, the product to the sbh.
        e.mem.set_u32(ACTIVE_HEAP, 3);
        e.mem.set_u32(SBH_THRESHOLD, 0x3f8);
        e.register(SBH_ALLOC_BLOCK, |_, a| {
            let block = if a[0] == 6 { 0x0800_0000u32 } else { 0 };
            block.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_15e2, &[2, 3, error]).u32(), 0x0800_0000);
        assert_eq!(calls(&e, MEMSET), vec![vec![0x0800_0000, 0, 6]]);
        assert_eq!(calls(&e, LOCK), vec![vec![4]]);
        assert_eq!(calls(&e, CALLOC_IMPL_FINALLY).len(), 1);
        assert!(calls(&e, HEAP_ALLOC).is_empty());
        assert_eq!(e.call(0x00ee_15e2, &[1, 7, error]).u32(), 0x0700_0010);
        assert_eq!(e.call(0x00ee_15e2, &[1, 0x400, error]).u32(), 0x0700_0400);
        assert_eq!(calls(&e, SBH_ALLOC_BLOCK), vec![vec![6], vec![7]]);
        // Nothing to be had: the new handler is asked while it succeeds.
        e.register(HEAP_ALLOC, |_, _| 0u32.into_ret());
        e.mem.set_u32(ACTIVE_HEAP, 1);
        e.mem.set_u32(error, 0);
        assert_eq!(e.call(0x00ee_15e2, &[1, 0x20, error]).u32(), 0);
        assert_eq!(e.mem.u32(error), 0xc);
        let tries = e.mem.alloc(4);
        e.register_double(CALLNEWH, move |e, _| {
            let n = e.mem.u32(tries) + 1;
            e.mem.set_u32(tries, n);
            u32::from(n < 3).into_ret()
        });
        e.mem.set_u32(NEW_MODE, 1);
        e.mem.set_u32(error, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_15e2, &[1, 0x20, error]).u32(), 0);
        assert_eq!(calls(&e, HEAP_ALLOC).len(), 3);
        assert_eq!(e.mem.u32(error), 0xc);
        // A null error word is left alone.
        assert_eq!(e.call(0x00ee_15e2, &[1, 0x20, 0]).u32(), 0);
    }

    #[test]
    fn calloc_passes_its_error_word_to_errno() {
        let (mut e, errno) = crt_engine();
        e.register(CALLOC_IMPL, |e, a| {
            e.mem.set_u32(a[2], 0xc);
            let block = if a[0] == 1 { 0x0700_0000u32 } else { 0 };
            block.into_ret()
        });
        assert_eq!(e.call(0x00ed_dd7d, &[1, 2]).u32(), 0x0700_0000);
        assert_eq!(e.mem.u32(errno), 0);
        assert_eq!(e.call(0x00ed_dd7d, &[2, 2]).u32(), 0);
        assert_eq!(e.mem.u32(errno), 0xc);
    }

    #[test]
    fn recalloc_clears_only_the_grown_tail() {
        let (mut e, errno) = crt_engine();
        nothing(&mut e, &[MEMSET]);
        e.register(MSIZE, |_, _| 0x10u32.into_ret());
        e.register(REALLOC, |_, a| {
            let block = if a[1] == 0 { 0u32 } else { 0x0700_0000 };
            block.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_1700, &[0x0600_0000, 4, 8]).u32(), 0x0700_0000);
        assert_eq!(calls(&e, MEMSET), vec![vec![0x0700_0010, 0, 0x10]]);
        assert_eq!(calls(&e, REALLOC), vec![vec![0x0600_0000, 0x20]]);
        assert_eq!(e.call(0x00ee_1700, &[0x0600_0000, 2, 8]).u32(), 0x0700_0000);
        assert_eq!(e.call(0x00ee_1700, &[0, 2, 4]).u32(), 0x0700_0000);
        assert_eq!(calls(&e, MEMSET).len(), 2);
        assert_eq!(calls(&e, MSIZE).len(), 2);
        assert_eq!(e.call(0x00ee_1700, &[0, 0x10, 0x1000_0000]).u32(), 0);
        assert_eq!(e.mem.u32(errno), 0xc);
    }
    /// An engine whose `_ismbblead` answers 1 for the bytes 0x81..=0x9F only (and the
    /// sign-extended forms of them) and whose multibyte table counts as set up.
    fn cmdline_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0127_1000, 0x1000);
        e.mem.set_u32(MBCTYPE_INITIALIZED, 1);
        e.register(ISMBBLEAD, |_, a| {
            u32::from((0x81..=0x9f).contains(&(a[0] & 0xff))).into_ret()
        });
        e
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let at = e.mem.alloc(s.len() as u32 + 1);
        e.mem.write(at, s);
        at
    }

    /// Runs `parse_cmdline` twice as `__setargv` does (counting, then filling) and answers the
    /// arguments, the character count and the argument count.
    fn parse(e: &mut Engine, line: &[u8]) -> (Vec<Vec<u8>>, u32, u32) {
        let at = text(e, line);
        let counts = e.mem.alloc(8);
        e.call(0x00ed_d792, &[at, counts, 0, 0, counts + 4]);
        let (chars, n) = (e.mem.u32(counts), e.mem.u32(counts + 4));
        let argv = e.mem.alloc(n * 4);
        let buffer = e.mem.alloc(chars);
        e.call(0x00ed_d792, &[at, counts, argv, buffer, counts + 4]);
        assert_eq!((e.mem.u32(counts), e.mem.u32(counts + 4)), (chars, n));
        assert_eq!(e.mem.u32(argv + (n - 1) * 4), 0);
        let words = (0..n - 1)
            .map(|i| e.mem.cstr(e.mem.u32(argv + i * 4)).to_vec())
            .collect();
        (words, chars, n)
    }

    #[test]
    fn parse_cmdline_follows_the_quote_and_backslash_rules() {
        let mut e = cmdline_engine();
        let (words, chars, n) = parse(&mut e, b"\"C:\\a b\\FalloutNV.exe\"  one\t\"two three\"");
        assert_eq!(
            words,
            vec![
                b"C:\\a b\\FalloutNV.exe".to_vec(),
                b"one".to_vec(),
                b"two three".to_vec(),
            ]
        );
        assert_eq!(n, 4);
        assert_eq!(chars, 21 + 4 + 10);
        let (words, _, _) = parse(&mut e, b"x a\\\\\\\"b c\\\\\"d e\" f\\g \"h\"\"i\"");
        assert_eq!(
            words,
            vec![
                b"x".to_vec(),
                b"a\\\"b".to_vec(),
                b"c\\d e".to_vec(),
                b"f\\g".to_vec(),
                b"h\"i".to_vec(),
            ]
        );
        // A lead byte takes its trail byte, even a quote.
        let (words, _, _) = parse(&mut e, b"p \x81\"z");
        assert_eq!(words, vec![b"p".to_vec(), b"\x81\"z".to_vec()]);
        // Only the program name.
        let (words, chars, n) = parse(&mut e, b"prog");
        assert_eq!((words, chars, n), (vec![b"prog".to_vec()], 5, 2));
    }

    #[test]
    fn wincmdln_skips_the_program_name_and_the_blanks() {
        let mut e = cmdline_engine();
        let line = text(&mut e, b"\"a b\\c.exe\" \t -x y");
        e.mem.set_u32(ACMDLN, line);
        assert_eq!(e.call(0x00ed_d655, &[]).u32(), line + 14);
        let line = text(&mut e, b"prog");
        e.mem.set_u32(ACMDLN, line);
        assert_eq!(e.call(0x00ed_d655, &[]).u32(), line + 4);
        let line = text(&mut e, b"p\x81 q r");
        e.mem.set_u32(ACMDLN, line);
        assert_eq!(e.call(0x00ed_d655, &[]).u32(), line + 5);
        // A null command line is the empty string; the table is set up first when needed.
        e.mem.set_u32(ACMDLN, 0);
        e.mem.set_u32(MBCTYPE_INITIALIZED, 0);
        e.map(0x0101_1000, 0x1000);
        e.mem.set_u8(EMPTY_TEXT, 0);
        nothing(&mut e, &[INITMBCTABLE]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_d655, &[]).u32(), EMPTY_TEXT);
        assert_eq!(calls(&e, INITMBCTABLE).len(), 1);
    }

    #[test]
    fn setenvp_copies_the_strings_not_starting_with_equals() {
        let mut e = cmdline_engine();
        e.map(0x0126_f000, 0x1000);
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(CALLOC_CRT, |e, a| e.mem.alloc(a[0] * a[1]).into_ret());
        e.register(STRCPY_S, |e, a| {
            let s = e.mem.cstr(a[2]).to_vec();
            e.mem.write(a[0], &s);
            0u32.into_ret()
        });
        nothing(&mut e, &[FREE, INVOKE_WATSON]);
        let block = text(&mut e, b"=C:=C:\\\0PATH=x\0TEMP=y\0\0");
        e.mem.set_u32(AENVPTR, block);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_d6b4, &[]).i32(), 0);
        let table = e.mem.u32(ENVIRON);
        assert_eq!(e.mem.cstr(e.mem.u32(table)), b"PATH=x");
        assert_eq!(e.mem.cstr(e.mem.u32(table + 4)), b"TEMP=y");
        assert_eq!(e.mem.u32(table + 8), 0);
        assert_eq!(calls(&e, CALLOC_CRT)[0], vec![3, 4]);
        assert_eq!(calls(&e, FREE), vec![vec![block]]);
        assert_eq!(e.mem.u32(AENVPTR), 0);
        assert_eq!(e.mem.u32(ENV_INITIALIZED), 1);
        assert!(calls(&e, INVOKE_WATSON).is_empty());
        // No block: -1.
        assert_eq!(e.call(0x00ed_d6b4, &[]).i32(), -1);
        // A string's block fails: the table is freed.
        let block = text(&mut e, b"A=1\0B=2\0\0");
        e.mem.set_u32(AENVPTR, block);
        e.register(CALLOC_CRT, |e, a| {
            if a[1] == 1 && a[0] == 4 && e.mem.u32(ENVIRON) != 0 {
                let first = e.mem.u32(e.mem.u32(ENVIRON));
                if first != 0 {
                    return 0u32.into_ret();
                }
            }
            e.mem.alloc(a[0] * a[1]).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_d6b4, &[]).i32(), -1);
        assert_eq!(e.mem.u32(ENVIRON), 0);
        assert_eq!(calls(&e, FREE).len(), 1);
        assert_eq!(e.mem.u32(AENVPTR), block);
    }
}

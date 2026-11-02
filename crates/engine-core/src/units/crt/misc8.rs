//! More C runtime pieces (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe 1.4.0.525),
//! translated from the exe: `__init_ctype` (`00ed08f1`), the process entry point
//! `_WinMainCRTStartup` (`00ecc4db`), the `__except` filters of `__tmainCRTStartup` (`00ecc4c2`)
//! and `__BuildCatchObject` (`00ece9e5`), the rethrow filter `00ece4ab` (`ExFilterRethrow`'s
//! shape, unconfirmed), the `__finally` block of `__FrameUnwindToState` (`00ece490`) and
//! `std::bad_exception`'s copy constructor (`00ecee9b`), and `__XcptFilter` (`00edd4f5`).
//!
//! Every call is made by address, as the exe makes it, so a test can double each one.

use crate::prelude::*;

/// `__getlocaleinfo` (`00ed2af0`, cdecl: locale object, kind, LCID, LCTYPE, destination),
/// `_malloc_crt` (`00ed0cdf`, one word), `_calloc_crt` (`00ed0d24`: count, size), `free`
/// (`00ecd291`), `memcpy` (`00ec44d0`: destination, source, count), `__crtGetStringTypeA`
/// (`00ee13a3`, eight words) and `__crtLCMapStringA` (`00ed0883`, nine words), cdecl.
const GETLOCALEINFO: u32 = 0x00ed_2af0;
const MALLOC_CRT: u32 = 0x00ed_0cdf;
const CALLOC_CRT: u32 = 0x00ed_0d24;
const FREE: u32 = 0x00ec_d291;
const MEMCPY: u32 = 0x00ec_44d0;
const GET_STRING_TYPE_A: u32 = 0x00ee_13a3;
const LC_MAP_STRING_A: u32 = 0x00ed_0883;
/// The `GetCPInfo` (`00fdf1d8`, stdcall: code page, `CPINFO`) and `InterlockedDecrement`
/// (`00fdf1c4`) import slots.
const GET_CP_INFO: u32 = 0x00fd_f1d8;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The C locale's tables `__init_ctype` restores: `_pctype` (`010f22d0`, `__newctype + 0x100`),
/// `_pclmap` (`010f2758`) and `_pcumap` (`010f28d8`).
const C_PCTYPE: u32 = 0x010f_22d0;
const C_PCLMAP: u32 = 0x010f_2758;
const C_PCUMAP: u32 = 0x010f_28d8;
/// `___security_init_cookie` (`00eddb9a`) and `__tmainCRTStartup` (`00ecc35d`), no arguments.
const SECURITY_INIT_COOKIE: u32 = 0x00ed_db9a;
const TMAIN_CRT_STARTUP: u32 = 0x00ec_c35d;

/// The walk over a `CPINFO`'s lead-byte ranges (pairs from `LeadByte`, `cpinfo + 6`,
/// ended by a zero end byte): calls `each(low, high)` for every pair, as `__init_ctype`'s two
/// loops read them (the end byte re-read after each step).
fn lead_ranges(e: &mut Engine, cpinfo: u32, mut each: impl FnMut(&mut Engine, u32, u32)) {
    let mut at = cpinfo.wrapping_add(7);
    loop {
        if e.mem.u8(at) == 0 {
            break;
        }
        let low = u32::from(e.mem.u8(at.wrapping_sub(1)));
        each(e, low, at);
        at = at.wrapping_add(2);
        if e.mem.u8(at.wrapping_sub(1)) == 0 {
            break;
        }
    }
}

// Translated from 00ed08f1 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed08f1 - the /GS stack cookie check (00ec408c) is not translated
/// `__init_ctype` (LIBCMT/initctyp.c; cdecl: the locale information). For the C locale (no
/// `LC_CTYPE` handle, +0x14) the old tables' count (+0xC0) is released and the C tables
/// restored (`mb_cur_max` 1). Otherwise the code page (+4) is read when unknown
/// (`LOCALE_IDEFAULTANSICODEPAGE` as a number, `LCID` at +0x30), a count word, a 0x180-entry
/// class table and the two 0x180-byte case maps are made with a 0x101-byte buffer of the
/// bytes 0..=0xFF; `GetCPInfo` must succeed with `MaxCharSize` at most 5. Lead-byte ranges
/// become spaces in the buffer; the classes come from `__crtGetStringTypeA` and the maps from
/// `__crtLCMapStringA` (lower, upper); the slots below the tables are cleared, the lead bytes
/// get `_LEADBYTE` (0x8000) and the negative halves are copied from the top. The old tables
/// are released (`InterlockedDecrement`; at 0 they and their count are freed) and the new
/// ones stored (+0xC0 count, +0xC4/+0xC8 classes, +0xCC lower, +0xD0 upper, +0xAC
/// `mb_cur_max`). Any failure frees what was made and answers 1; success answers 0.
pub fn init_ctype(e: &mut Engine, info: u32) -> u32 {
    if e.mem.u32(info.wrapping_add(0x14)) == 0 {
        let count_at = info.wrapping_add(0xc0);
        let count = e.mem.u32(count_at);
        if count != 0 {
            e.call(INTERLOCKED_DECREMENT, &args![count]);
        }
        e.mem.set_u32(count_at, 0);
        e.mem.set_u32(info.wrapping_add(0xc4), 0);
        e.mem.set_u32(info.wrapping_add(0xc8), C_PCTYPE);
        e.mem.set_u32(info.wrapping_add(0xcc), C_PCLMAP);
        e.mem.set_u32(info.wrapping_add(0xd0), C_PCUMAP);
        e.mem.set_u32(info.wrapping_add(0xac), 1);
        return 0;
    }
    // The frame: the locale object (8 bytes) and the CPINFO (0x14 bytes).
    e.with_stack(0x1c, |e, frame| {
        let local = frame.addr();
        let cpinfo = local.wrapping_add(8);
        e.mem.set_u32(local, info);
        e.mem.set_u32(local.wrapping_add(4), 0);
        let (mut count, mut ctype, mut lower, mut upper, mut buffer) = (0u32, 0u32, 0, 0, 0);
        let mut result = 1u32;
        'built: {
            let codepage_at = info.wrapping_add(4);
            if e.mem.u32(codepage_at) == 0 {
                let lcid = u32::from(e.mem.u16(info.wrapping_add(0x30)));
                if e.call(
                    GETLOCALEINFO,
                    &args![local, 0u32, lcid, 0x1004u32, codepage_at],
                )
                .u32()
                    != 0
                {
                    break 'built;
                }
            }
            count = e.call(MALLOC_CRT, &args![4u32]).u32();
            ctype = e.call(CALLOC_CRT, &args![0x180u32, 2u32]).u32();
            lower = e.call(CALLOC_CRT, &args![0x180u32, 1u32]).u32();
            upper = e.call(CALLOC_CRT, &args![0x180u32, 1u32]).u32();
            buffer = e.call(CALLOC_CRT, &args![0x101u32, 1u32]).u32();
            if count == 0 || ctype == 0 || buffer == 0 || lower == 0 || upper == 0 {
                break 'built;
            }
            e.mem.set_u32(count, 0);
            for i in 0..0x100u32 {
                e.mem.set_u8(buffer.wrapping_add(i), i as u8);
            }
            let codepage = e.mem.u32(codepage_at);
            if e.call(GET_CP_INFO, &args![codepage, cpinfo]).u32() == 0 {
                break 'built;
            }
            if e.mem.u32(cpinfo) > 5 {
                break 'built;
            }
            let max = u32::from(e.mem.u16(cpinfo));
            if (max as i32) > 1 && e.mem.u8(cpinfo.wrapping_add(6)) != 0 {
                lead_ranges(e, cpinfo, |e, low, end_at| {
                    let mut j = low;
                    while (j as i32) <= i32::from(e.mem.u8(end_at)) {
                        e.mem.set_u8(buffer.wrapping_add(j), 0x20);
                        j = j.wrapping_add(1);
                    }
                });
            }
            let classes = ctype.wrapping_add(0x100);
            let codepage = e.mem.u32(codepage_at);
            if e.call(
                GET_STRING_TYPE_A,
                &args![0u32, 1u32, buffer, 0x100u32, classes, codepage, 0u32, 0u32],
            )
            .u32()
                == 0
            {
                break 'built;
            }
            for (map, kind) in [(lower, 0x100u32), (upper, 0x200)] {
                let handle = e.mem.u32(info.wrapping_add(0x14));
                let codepage = e.mem.u32(codepage_at);
                if e.call(
                    LC_MAP_STRING_A,
                    &args![
                        0u32,
                        handle,
                        kind,
                        buffer.wrapping_add(1),
                        0xffu32,
                        map.wrapping_add(0x81),
                        0xffu32,
                        codepage,
                        0u32
                    ],
                )
                .u32()
                    == 0
                {
                    break 'built;
                }
            }
            let ctype_base = ctype.wrapping_add(0xfe);
            e.mem.set_u16(ctype_base, 0);
            e.mem.set_u8(lower.wrapping_add(0x7f), 0);
            e.mem.set_u8(upper.wrapping_add(0x7f), 0);
            e.mem.set_u8(lower.wrapping_add(0x80), 0);
            e.mem.set_u8(upper.wrapping_add(0x80), 0);
            if (max as i32) > 1 && e.mem.u8(cpinfo.wrapping_add(6)) != 0 {
                lead_ranges(e, cpinfo, |e, low, end_at| {
                    if (low as i32) > i32::from(e.mem.u8(end_at)) {
                        return;
                    }
                    let mut j = low;
                    let mut slot = ctype.wrapping_add(low.wrapping_mul(2)).wrapping_add(0x100);
                    loop {
                        j = j.wrapping_add(1);
                        e.mem.set_u16(slot, 0x8000);
                        slot = slot.wrapping_add(2);
                        if (j as i32) > i32::from(e.mem.u8(end_at)) {
                            break;
                        }
                    }
                });
            }
            e.call(MEMCPY, &args![ctype, ctype.wrapping_add(0x200), 0xfeu32]);
            e.call(MEMCPY, &args![lower, lower.wrapping_add(0x100), 0x7fu32]);
            e.call(MEMCPY, &args![upper, upper.wrapping_add(0x100), 0x7fu32]);
            let old = e.mem.u32(info.wrapping_add(0xc0));
            if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old]).u32() == 0 {
                let block = e.mem.u32(info.wrapping_add(0xc4)).wrapping_sub(0xfe);
                e.call(FREE, &args![block]);
                let block = e.mem.u32(info.wrapping_add(0xcc)).wrapping_sub(0x80);
                e.call(FREE, &args![block]);
                let block = e.mem.u32(info.wrapping_add(0xd0)).wrapping_sub(0x80);
                e.call(FREE, &args![block]);
                let block = e.mem.u32(info.wrapping_add(0xc0));
                e.call(FREE, &args![block]);
            }
            e.mem.set_u32(count, 1);
            e.mem.set_u32(info.wrapping_add(0xc0), count);
            e.mem.set_u32(info.wrapping_add(0xc8), classes);
            e.mem.set_u32(info.wrapping_add(0xc4), ctype_base);
            e.mem
                .set_u32(info.wrapping_add(0xcc), lower.wrapping_add(0x80));
            e.mem
                .set_u32(info.wrapping_add(0xd0), upper.wrapping_add(0x80));
            e.mem.set_u32(info.wrapping_add(0xac), max);
            result = 0;
        }
        if result != 0 {
            for block in [count, ctype, lower, upper] {
                e.call(FREE, &args![block]);
            }
        }
        e.call(FREE, &args![buffer]);
        result
    })
}

// Translated from 00ecc4c2 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__except` filter of `__tmainCRTStartup` (`00ecc35d`): `EXCEPTION_EXECUTE_HANDLER` (1).
pub fn tmain_crt_startup_filter(_e: &mut Engine) -> u32 {
    1
}

// Translated from 00ecc4db (disassembled, FalloutNV.exe 1.4.0.525)
/// `_WinMainCRTStartup` (LIBCMT/crt0.c; the process entry point, no arguments):
/// `__security_init_cookie`, then a tail jump to `__tmainCRTStartup`, whose `EAX` is this
/// function's.
pub fn win_main_crt_startup(e: &mut Engine) -> u32 {
    e.call(SECURITY_INIT_COOKIE, &args![]);
    e.call(TMAIN_CRT_STARTUP, &args![]).u32()
}

/// `_getptd` (`00ed11de`, no arguments), `std::exception`'s copy constructor (`00ec7944`,
/// thiscall, `RET 4`), the vtable of `std::bad_exception` (`010f20a8`) and the MSVC C++
/// exception code.
const GETPTD: u32 = 0x00ed_11de;
const EXCEPTION_COPY: u32 = 0x00ec_7944;
const BAD_EXCEPTION_VTABLE: u32 = 0x010f_20a8;
const CXX_EXCEPTION: u32 = 0xe06d_7363;

// Translated from 00ece490 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `__FrameUnwindToState` (`00ece3ca`, no arguments): the thread's
/// `_ProcessingThrow` count (`_getptd() + 0x90`) is counted down when it is above 0 (signed).
pub fn frame_unwind_to_state_finally(e: &mut Engine) {
    let ptd = e.call(GETPTD, &args![]).u32();
    if (e.mem.u32(ptd.wrapping_add(0x90)) as i32) > 0 {
        let ptd = e.call(GETPTD, &args![]).u32();
        let at = ptd.wrapping_add(0x90);
        let n = e.mem.u32(at).wrapping_sub(1);
        e.mem.set_u32(at, n);
    }
}

// Translated from 00ece4ab (disassembled, FalloutNV.exe 1.4.0.525)
/// No Ghidra name (the shape of LIBCMT's `ExFilterRethrow`, unconfirmed; the
/// `EXCEPTION_POINTERS` in `EAX`), the filter `CallCatchBlock`'s `__except` runs (`00ece6d8`):
/// a C++ exception (code 0xE06D7363, three parameters, magic 0x19930520, 0x19930521 or
/// 0x19930522) with no throw information (+0x1C: a rethrow) sets the thread's `_cxxReThrow`
/// (`_getptd() + 0x20C`) and answers 1; anything else 0.
pub fn ex_filter_rethrow(e: &mut Engine, pointers: u32) -> u32 {
    let record = e.mem.u32(pointers);
    if e.mem.u32(record) != CXX_EXCEPTION || e.mem.u32(record.wrapping_add(0x10)) != 3 {
        return 0;
    }
    let magic = e.mem.u32(record.wrapping_add(0x14));
    if magic != 0x1993_0520 && magic != 0x1993_0521 && magic != 0x1993_0522 {
        return 0;
    }
    if e.mem.u32(record.wrapping_add(0x1c)) != 0 {
        return 0;
    }
    let ptd = e.call(GETPTD, &args![]).u32();
    e.mem.set_u32(ptd.wrapping_add(0x20c), 1);
    1
}

// Translated from 00ece9e5 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__except` filter of `__BuildCatchObject` (`00ece960`, scope table `01180100`):
/// `EXCEPTION_EXECUTE_HANDLER` (1).
pub fn build_catch_object_filter(_e: &mut Engine) -> u32 {
    1
}

// Translated from 00ecee9b (disassembled, FalloutNV.exe 1.4.0.525)
/// `std::bad_exception`'s copy constructor (thiscall, one word; `RET 4`): the exception copy
/// constructor (`00ec7944`), then the `bad_exception` vtable. Answers `this`.
pub fn bad_exception_copy(e: &mut Engine, this: u32, other: u32) -> u32 {
    e.call(EXCEPTION_COPY, &args![this, other]);
    e.mem.set_u32(this, BAD_EXCEPTION_VTABLE);
    this
}

/// `_getptd_noexit` (`00ed1165`, no arguments), `_XcptActTabCount` (`011c289c`),
/// `_First_FPE_Indx` (`011c2890`) and `_Num_FPE` (`011c2894`).
const GETPTD_NOEXIT: u32 = 0x00ed_1165;
const XCPT_ACT_TAB_COUNT: u32 = 0x011c_289c;
const FIRST_FPE_INDEX: u32 = 0x011c_2890;
const FPE_COUNT: u32 = 0x011c_2894;

/// The floating-point exception codes `__XcptFilter` turns into `_FPE_*` codes for `SIGFPE`.
const FPE_CODES: [(u32, u32); 7] = [
    (0xc000_008e, 0x83),
    (0xc000_0090, 0x81),
    (0xc000_0091, 0x84),
    (0xc000_0093, 0x85),
    (0xc000_008d, 0x82),
    (0xc000_008f, 0x86),
    (0xc000_0092, 0x8a),
];

// Translated from 00edd4f5 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__XcptFilter` (LIBCMT/winxfltr.c; cdecl: exception code, exception pointers): 0 without
/// thread data (`_getptd_noexit`). The thread's action table (+0x5C, 12-byte entries:
/// code, signal, action; `_XcptActTabCount` of them, the first code read before the bound
/// is tested) is searched for the code; no entry or a default action (0) answers 0
/// (`EXCEPTION_CONTINUE_SEARCH`), `SIG_DIE` (5) is reset and answers 1, `SIG_IGN` (1) answers
/// -1. Otherwise, with the pointers in the thread's `_tpxcptinfoptrs` (+0x60) for the
/// handler's time: for `SIGFPE` (8) every floating-point entry's action is reset, the thread's
/// `_tfpecode` (+0x64) set from the code (kept for others) and the handler called with
/// (8, that code), the old code then put back; for any other signal the entry's action is
/// reset and the handler called with the signal. Answers -1 (`EXCEPTION_CONTINUE_EXECUTION`).
pub fn xcpt_filter(e: &mut Engine, code: u32, pointers: u32) -> i32 {
    let ptd = e.call(GETPTD_NOEXIT, &args![]).u32();
    if ptd == 0 {
        return 0;
    }
    let table = e.mem.u32(ptd.wrapping_add(0x5c));
    let count = e.mem.u32(XCPT_ACT_TAB_COUNT);
    let end = count.wrapping_mul(0xc).wrapping_add(table);
    let mut entry = table;
    loop {
        if e.mem.u32(entry) == code {
            break;
        }
        entry = entry.wrapping_add(0xc);
        if entry >= end {
            break;
        }
    }
    if entry >= end || e.mem.u32(entry) != code {
        return 0;
    }
    let handler = e.mem.u32(entry.wrapping_add(8));
    if handler == 0 {
        return 0;
    }
    if handler == 5 {
        e.mem.set_u32(entry.wrapping_add(8), 0);
        return 1;
    }
    if handler == 1 {
        return -1;
    }
    let old_pointers = e.mem.u32(ptd.wrapping_add(0x60));
    e.mem.set_u32(ptd.wrapping_add(0x60), pointers);
    let signal = e.mem.u32(entry.wrapping_add(4));
    if signal == 8 {
        let mut index = e.mem.u32(FIRST_FPE_INDEX);
        let mut slot = index.wrapping_mul(0xc);
        while (index as i32)
            < (e.mem
                .u32(FPE_COUNT)
                .wrapping_add(e.mem.u32(FIRST_FPE_INDEX)) as i32)
        {
            let table = e.mem.u32(ptd.wrapping_add(0x5c));
            e.mem.set_u32(slot.wrapping_add(table).wrapping_add(8), 0);
            index = index.wrapping_add(1);
            slot = slot.wrapping_add(0xc);
        }
        let exception = e.mem.u32(entry);
        let old_code = e.mem.u32(ptd.wrapping_add(0x64));
        if let Some((_, fpe)) = FPE_CODES.iter().find(|(c, _)| *c == exception) {
            e.mem.set_u32(ptd.wrapping_add(0x64), *fpe);
        }
        let fpe = e.mem.u32(ptd.wrapping_add(0x64));
        e.call(handler, &args![8u32, fpe]);
        e.mem.set_u32(ptd.wrapping_add(0x64), old_code);
    } else {
        e.mem.set_u32(entry.wrapping_add(8), 0);
        e.call(handler, &args![signal]);
    }
    e.mem.set_u32(ptd.wrapping_add(0x60), old_pointers);
    -1
}

/// This file's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ed_08f1, init_ctype(u32) -> u32),
        entry!(0x00ec_c4c2, tmain_crt_startup_filter() -> u32),
        entry!(0x00ec_c4db, win_main_crt_startup() -> u32),
        entry!(0x00ec_e490, frame_unwind_to_state_finally()),
        entry!(0x00ec_e4ab, ex_filter_rethrow(u32) -> u32),
        entry!(0x00ec_e9e5, build_catch_object_filter() -> u32),
        entry!(0x00ec_ee9b, bad_exception_copy(u32, u32) -> u32),
        entry!(0x00ed_d4f5, xcpt_filter(u32, u32) -> i32),
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

    /// An engine for `__init_ctype` with a double-byte code page whose lead bytes are
    /// 0x81..=0x82: the allocators from the engine's memory, `GetCPInfo` answering
    /// `MaxCharSize` 2, `__crtGetStringTypeA` writing 0x20 per byte (0x40 for a space),
    /// `__crtLCMapStringA` writing the byte plus the case kind's high byte.
    fn ctype_engine() -> Engine {
        let mut e = Engine::new();
        e.register(MALLOC_CRT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CALLOC_CRT, |e, a| e.mem.alloc(a[0] * a[1]).into_ret());
        nothing(&mut e, &[FREE]);
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(GET_CP_INFO, |e, a| {
            e.mem.set_u32(a[1], 2);
            e.mem.write(a[1] + 6, &[0x81, 0x82, 0, 0]);
            1u32.into_ret()
        });
        e.register(GET_STRING_TYPE_A, |e, a| {
            for i in 0..a[3] {
                let c = e.mem.u8(a[2] + i);
                e.mem
                    .set_u16(a[4] + i * 2, if c == 0x20 { 0x40 } else { 0x20 });
            }
            1u32.into_ret()
        });
        e.register(LC_MAP_STRING_A, |e, a| {
            for i in 0..a[4] {
                let c = e.mem.u8(a[3] + i);
                e.mem.set_u8(a[5] + i, c ^ (a[2] >> 8) as u8);
            }
            1u32.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e
    }

    #[test]
    fn init_ctype_builds_the_tables_for_a_double_byte_code_page() {
        let mut e = ctype_engine();
        let info = e.mem.alloc(0xd4);
        e.mem.set_u32(info + 0x14, 0x55);
        e.mem.set_u32(info + 4, 932);
        let old_count = e.mem.alloc(4);
        e.mem.set_u32(old_count, 1);
        e.mem.set_u32(info + 0xc0, old_count);
        e.mem.set_u32(info + 0xc4, 0x0600_00fe);
        e.mem.set_u32(info + 0xcc, 0x0600_1080);
        e.mem.set_u32(info + 0xd0, 0x0600_2080);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_08f1, &[info]).u32(), 0);
        assert!(calls(&e, GETLOCALEINFO).is_empty());
        let classes = e.mem.u32(info + 0xc8);
        assert_eq!(e.mem.u32(info + 0xc4), classes - 2);
        assert_eq!(e.mem.u32(info + 0xac), 2);
        // The lead bytes were spaces for the class query, then _LEADBYTE.
        assert_eq!(e.mem.u16(classes + 0x81 * 2), 0x8000);
        assert_eq!(e.mem.u16(classes + 0x82 * 2), 0x8000);
        assert_eq!(e.mem.u16(classes + 0x80 * 2), 0x20);
        assert_eq!(e.mem.u16(classes + 0x20 * 2), 0x40);
        assert_eq!(e.mem.u16(classes - 2), 0);
        // The negative half is the top half's copy.
        assert_eq!(e.mem.u16(classes - 0x100), e.mem.u16(classes + 0x100));
        let lower = e.mem.u32(info + 0xcc);
        let upper = e.mem.u32(info + 0xd0);
        assert_eq!(e.mem.u8(lower + 0x41), 0x41 ^ 1);
        assert_eq!(e.mem.u8(upper + 0x41), 0x41 ^ 2);
        assert_eq!(e.mem.u8(lower), 0);
        assert_eq!(e.mem.u8(lower - 1), 0);
        let asked = calls(&e, LC_MAP_STRING_A);
        assert_eq!(asked[0][..3], [0, 0x55, 0x100]);
        assert_eq!(asked[1][..3], [0, 0x55, 0x200]);
        assert_eq!(asked[0][7], 932);
        let count = e.mem.u32(info + 0xc0);
        assert_eq!(e.mem.u32(count), 1);
        // The old tables reached 0: freed, then the buffer.
        let freed = calls(&e, FREE);
        assert_eq!(
            freed[..4],
            [
                vec![0x0600_0000],
                vec![0x0600_1000],
                vec![0x0600_2000],
                vec![old_count]
            ]
        );
        assert_eq!(freed.len(), 5);
    }

    #[test]
    fn init_ctype_reads_the_code_page_and_fails_cleanly() {
        let mut e = ctype_engine();
        let info = e.mem.alloc(0xd4);
        e.mem.set_u32(info + 0x14, 0x55);
        e.mem.set_u16(info + 0x30, 0x411);
        e.register(GETLOCALEINFO, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_08f1, &[info]).u32(), 1);
        assert_eq!(calls(&e, GETLOCALEINFO)[0][1..4], [0, 0x411, 0x1004]);
        assert_eq!(calls(&e, FREE), vec![vec![0]; 5]);
        assert!(calls(&e, MALLOC_CRT).is_empty());
        // A code page with MaxCharSize above 5 is refused after the blocks are made.
        e.register(GETLOCALEINFO, |e, a| {
            e.mem.set_u32(a[4], 1252);
            0u32.into_ret()
        });
        e.register(GET_CP_INFO, |e, a| {
            e.mem.set_u32(a[1], 6);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_08f1, &[info]).u32(), 1);
        assert_eq!(calls(&e, GET_CP_INFO)[0][0], 1252);
        let freed = calls(&e, FREE);
        assert_eq!(freed.len(), 5);
        assert!(freed.iter().all(|f| f[0] != 0));
        assert_eq!(e.mem.u32(info + 0xc0), 0);
    }

    #[test]
    fn init_ctype_restores_the_c_tables_without_a_handle() {
        let mut e = ctype_engine();
        let info = e.mem.alloc(0xd4);
        let old_count = e.mem.alloc(4);
        e.mem.set_u32(old_count, 3);
        e.mem.set_u32(info + 0xc0, old_count);
        assert_eq!(e.call(0x00ed_08f1, &[info]).u32(), 0);
        assert_eq!(e.mem.u32(old_count), 2);
        assert_eq!(
            (
                e.mem.u32(info + 0xc0),
                e.mem.u32(info + 0xc4),
                e.mem.u32(info + 0xc8),
                e.mem.u32(info + 0xcc),
                e.mem.u32(info + 0xd0),
                e.mem.u32(info + 0xac)
            ),
            (0, 0, C_PCTYPE, C_PCLMAP, C_PCUMAP, 1)
        );
    }

    #[test]
    fn the_entry_point_sets_the_cookie_then_runs_tmain() {
        let mut e = Engine::new();
        nothing(&mut e, &[SECURITY_INIT_COOKIE]);
        e.register(TMAIN_CRT_STARTUP, |_, _| 7u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_c4db, &[]).u32(), 7);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| *a == SECURITY_INIT_COOKIE || *a == TMAIN_CRT_STARTUP)
            .collect();
        assert_eq!(order, vec![SECURITY_INIT_COOKIE, TMAIN_CRT_STARTUP]);
        assert_eq!(e.call(0x00ec_c4c2, &[]).u32(), 1);
    }
    #[test]
    fn the_frame_unwind_finally_counts_the_throw_down_to_zero() {
        let mut e = Engine::new();
        let ptd = e.mem.alloc(0x210);
        e.register_double(GETPTD, move |_, _| ptd.into_ret());
        e.mem.set_u32(ptd + 0x90, 2);
        e.call(0x00ec_e490, &[]);
        assert_eq!(e.mem.u32(ptd + 0x90), 1);
        e.call(0x00ec_e490, &[]);
        e.call(0x00ec_e490, &[]);
        assert_eq!(e.mem.u32(ptd + 0x90), 0);
        e.mem.set_u32(ptd + 0x90, u32::MAX);
        e.call(0x00ec_e490, &[]);
        assert_eq!(e.mem.u32(ptd + 0x90), u32::MAX);
    }

    #[test]
    fn ex_filter_rethrow_takes_only_a_rethrow() {
        let mut e = Engine::new();
        let ptd = e.mem.alloc(0x210);
        e.register_double(GETPTD, move |_, _| ptd.into_ret());
        let record = e.mem.alloc(0x20);
        let pointers = e.mem.alloc(8);
        e.mem.set_u32(pointers, record);
        for (code, params, magic, info, taken) in [
            (CXX_EXCEPTION, 3, 0x1993_0520, 0, true),
            (CXX_EXCEPTION, 3, 0x1993_0521, 0, true),
            (CXX_EXCEPTION, 3, 0x1993_0522, 0, true),
            (CXX_EXCEPTION, 3, 0x0199_4000, 0, false),
            (CXX_EXCEPTION, 3, 0x1993_0520, 0x0700_0000, false),
            (CXX_EXCEPTION, 2, 0x1993_0520, 0, false),
            (0xc000_0005, 3, 0x1993_0520, 0, false),
        ] {
            e.mem.set_u32(ptd + 0x20c, 0);
            e.mem.set_u32(record, code);
            e.mem.set_u32(record + 0x10, params);
            e.mem.set_u32(record + 0x14, magic);
            e.mem.set_u32(record + 0x1c, info);
            assert_eq!(e.call(0x00ec_e4ab, &[pointers]).u32(), u32::from(taken));
            assert_eq!(e.mem.u32(ptd + 0x20c), u32::from(taken));
        }
    }

    #[test]
    fn bad_exception_copy_and_the_catch_object_filter() {
        let mut e = Engine::new();
        nothing(&mut e, &[EXCEPTION_COPY]);
        let a = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_ee9b, &[a, 0x0600_0000]).u32(), a);
        assert_eq!(e.mem.u32(a), BAD_EXCEPTION_VTABLE);
        assert_eq!(calls(&e, EXCEPTION_COPY), vec![vec![a, 0x0600_0000]]);
        assert_eq!(e.call(0x00ec_e9e5, &[]).u32(), 1);
    }
    /// An engine with thread data whose action table has `entries` (code, signal, action).
    fn xcpt_engine(entries: &[(u32, u32, u32)]) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x011c_2000, 0x1000);
        let ptd = e.mem.alloc(0x100);
        e.register_double(GETPTD_NOEXIT, move |_, _| ptd.into_ret());
        let table = e.mem.alloc(entries.len() as u32 * 0xc + 0xc);
        for (i, (c, s, a)) in entries.iter().enumerate() {
            let at = table + i as u32 * 0xc;
            e.mem.set_u32(at, *c);
            e.mem.set_u32(at + 4, *s);
            e.mem.set_u32(at + 8, *a);
        }
        e.mem.set_u32(ptd + 0x5c, table);
        e.mem.set_u32(XCPT_ACT_TAB_COUNT, entries.len() as u32);
        (e, ptd, table)
    }

    #[test]
    fn xcpt_filter_follows_the_action_table() {
        const HANDLER: u32 = 0x00a0_0000;
        let (mut e, ptd, table) = xcpt_engine(&[
            (0xc000_0005, 0xb, HANDLER),
            (0xc000_001d, 4, 5),
            (0xc000_0096, 4, 1),
            (0xc000_008e, 8, HANDLER),
            (0xc000_0090, 8, 0x77),
            (0xc000_0094, 8, 0),
        ]);
        e.mem.set_u32(FIRST_FPE_INDEX, 3);
        e.mem.set_u32(FPE_COUNT, 2);
        let seen = e.mem.alloc(8);
        e.register_double(HANDLER, move |e, a| {
            let (ptd_pointers, code) = (e.mem.u32(ptd + 0x60), e.mem.u32(ptd + 0x64));
            e.mem.set_u32(seen, ptd_pointers);
            e.mem.set_u32(seen + 4, code);
            assert!(a[0] == 0xb || a[0] == 8);
            Ret::default()
        });
        e.mem.set_u32(ptd + 0x60, 0x11);
        e.mem.set_u32(ptd + 0x64, 0x22);
        e.call_log = Some(vec![]);
        // Unknown code, default action.
        assert_eq!(e.call(0x00ed_d4f5, &[0x1234, 0x0600_0000]).i32(), 0);
        assert_eq!(e.call(0x00ed_d4f5, &[0xc000_0094, 0x0600_0000]).i32(), 0);
        // SIG_IGN and SIG_DIE.
        assert_eq!(e.call(0x00ed_d4f5, &[0xc000_0096, 0x0600_0000]).i32(), -1);
        assert_eq!(e.call(0x00ed_d4f5, &[0xc000_001d, 0x0600_0000]).i32(), 1);
        assert_eq!(e.mem.u32(table + 0xc + 8), 0);
        // A handler for another signal: its action reset, called with the signal.
        assert_eq!(e.call(0x00ed_d4f5, &[0xc000_0005, 0x0600_0000]).i32(), -1);
        assert_eq!(e.mem.u32(table + 8), 0);
        assert_eq!(e.mem.u32(seen), 0x0600_0000);
        assert_eq!(e.mem.u32(ptd + 0x60), 0x11);
        // SIGFPE: every FPE entry reset, the code mapped for the handler and put back.
        assert_eq!(e.call(0x00ed_d4f5, &[0xc000_008e, 0x0700_0000]).i32(), -1);
        assert_eq!(e.mem.u32(seen + 4), 0x83);
        assert_eq!(e.mem.u32(ptd + 0x64), 0x22);
        assert_eq!(e.mem.u32(table + 3 * 0xc + 8), 0);
        assert_eq!(e.mem.u32(table + 4 * 0xc + 8), 0);
        let handled: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == HANDLER)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(handled, vec![vec![0xb], vec![8, 0x83]]);
        // No thread data.
        e.register(GETPTD_NOEXIT, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ed_d4f5, &[0xc000_0005, 0]).i32(), 0);
    }
}

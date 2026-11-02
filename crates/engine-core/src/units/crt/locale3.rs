//! The C runtime's multibyte code page and locale-information pieces the play build reaches
//! (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe 1.4.0.525), translated from the
//! exe: `CPtoLCID` (`00ed3ab9`), `setSBCS` (`00ed3ae8`), `setSBUpLow` (`00ed3b4c`),
//! `getSystemCP` (`00ed3d83`), `__setmbcp_nolock` (`00ed3dff`), `__setmbcp` (`00ed3fe4`),
//! `__getlocaleinfo` (`00ed2af0`), `__crtGetLocaleInfoW` (`00ee1df7`), `__crtGetLocaleInfoA`
//! (`00ee1f6a`), `__free_lc_time` (`00ee02c9`), `__free_lconv_num` (`00ee04d7`),
//! `__free_lconv_mon` (`00ee06e6`) and `__towlower_l` (`00edd3f0`).
//!
//! `CPtoLCID`, `setSBCS`, `setSBUpLow` and `getSystemCP` take their argument in a register
//! (`EAX`, `EAX`, `ESI`, `ESI`); it is their first word here, as the registry passes it.
//! Every call is made by address, as the exe makes it, so a test can double each one.

use super::*;

/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`; the object, then the locale).
const LOCALE_UPDATE: u32 = 0x00ec_660e;
/// `memset` (`00ec61c0`), `free` (`00ecd291`), `_malloc_crt` (`00ed0cdf`), `_calloc_crt`
/// (`00ed0d24`: count, size), `strncpy_s` (`00ec8d5f`), `__invoke_watson` (`00ec7b08`),
/// `isdigit` (`00ec8860`), `_errno` (`00ec85e3`), `_getptd` (`00ed11de`),
/// `__updatetmbcinfo` (`00ed3cdf`), `_lock` (`00eddf39`), cdecl.
const MEMSET: u32 = 0x00ec_61c0;
const FREE: u32 = 0x00ec_d291;
const MALLOC_CRT: u32 = 0x00ed_0cdf;
const CALLOC_CRT: u32 = 0x00ed_0d24;
const STRNCPY_S: u32 = 0x00ec_8d5f;
const INVOKE_WATSON: u32 = 0x00ec_7b08;
const ISDIGIT: u32 = 0x00ec_8860;
const ERRNO: u32 = 0x00ec_85e3;
const GETPTD: u32 = 0x00ed_11de;
const UPDATETMBCINFO: u32 = 0x00ed_3cdf;
const LOCK: u32 = 0x00ed_df39;
/// `__crtGetStringTypeA` (`00ee13a3`, eight words), `__crtLCMapStringA` (`00ed0883`, nine
/// words), `__crtLCMapStringW` (`00ee4774`, eight words), the `_LocaleUpdate`-taking
/// `__crtGetLocaleInfoA` worker (`00ee1e2b`, six words) and the class test `00ee2332`
/// (character, mask, `_LocaleUpdate` object), cdecl.
const GET_STRING_TYPE_A: u32 = 0x00ee_13a3;
const LC_MAP_STRING_A: u32 = 0x00ed_0883;
const LC_MAP_STRING_W: u32 = 0x00ee_4774;
const GET_LOCALE_INFO_A_WORKER: u32 = 0x00ee_1e2b;
const CLASS_TEST: u32 = 0x00ee_2332;
/// This file's own functions, called by address where the exe calls them.
const CP_TO_LCID: u32 = 0x00ed_3ab9;
const SET_SBCS: u32 = 0x00ed_3ae8;
const SET_SB_UP_LOW: u32 = 0x00ed_3b4c;
const GET_SYSTEM_CP: u32 = 0x00ed_3d83;
const SETMBCP_NOLOCK: u32 = 0x00ed_3dff;
const SETMBCP_FINALLY: u32 = 0x00ed_4145;
const CRT_GET_LOCALE_INFO_W: u32 = 0x00ee_1df7;
const CRT_GET_LOCALE_INFO_A: u32 = 0x00ee_1f6a;
/// Import slots (KERNEL32, stdcall): `GetCPInfo`, `IsValidCodePage`, `GetOEMCP`, `GetACP`,
/// `GetLocaleInfoW`, `GetLastError`, `InterlockedDecrement`, `InterlockedIncrement`.
const API_GET_CP_INFO: u32 = 0x00fd_f1d8;
const API_IS_VALID_CODE_PAGE: u32 = 0x00fd_f1f0;
const API_GET_OEMCP: u32 = 0x00fd_f1e8;
const API_GET_ACP: u32 = 0x00fd_f1e4;
const API_GET_LOCALE_INFO_W: u32 = 0x00fd_f22c;
const API_GET_LAST_ERROR: u32 = 0x00fd_f094;
const API_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
const API_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `fSystemSet` (`01270468`), the initial multibyte info (`__initialmbcinfo`, `011c1df0`),
/// the code page table (`__rgcpinfo`, five 0x30-byte entries from `011c2220`: code page,
/// then the range bytes from +0x10 and the six `mbulinfo` words from +4) with its four type
/// bits (`__rgctypeflag`, `011c221c`), the global multibyte info (`__ptmbcinfo`, `011c2218`),
/// `__globallocalestatus` (`011c2310`), and the global copies `__setmbcp` refreshes:
/// `__mbcodepage` (`01270478`), `__ismbcodepage` (`0127047c`), `__mblcid` (`01270480`),
/// `__mbulinfo` (`0127046c`), `_mbctype` (`011c2010`), `_mbcasemap` (`011c2118`).
const SYSTEM_SET: u32 = 0x0127_0468;
const INITIAL_MBCINFO: u32 = 0x011c_1df0;
const CP_TABLE: u32 = 0x011c_2220;
const CP_TYPE_FLAGS: u32 = 0x011c_221c;
const GLOBAL_MBCINFO: u32 = 0x011c_2218;
const GLOBAL_LOCALE_STATUS: u32 = 0x011c_2310;
const MB_CODEPAGE: u32 = 0x0127_0478;
const IS_MB_CODEPAGE: u32 = 0x0127_047c;
const MB_LCID: u32 = 0x0127_0480;
const MB_ULINFO: u32 = 0x0127_046c;
const MB_CTYPE: u32 = 0x011c_2010;
const MB_CASEMAP: u32 = 0x011c_2118;
/// The four wide characters `__getlocaleinfo` reads an integer field into (`0127045c`).
const INT_FIELD_BUFFER: u32 = 0x0127_045c;
/// The `lconv` defaults the free functions keep (`__lconv_c`'s fields, `011c19d4` to
/// `011c19f8`).
const LCONV_NUM_DEFAULTS: u32 = 0x011c_19d4;
const LCONV_MON_DEFAULTS: u32 = 0x011c_19e0;

// Translated from 00ed3ab9 (disassembled, FalloutNV.exe 1.4.0.525)
/// `CPtoLCID` (`EAX`: the code page): the locale of the four East Asian code pages (932
/// Japanese 0x411, 936 Chinese 0x804, 949 Korean 0x412, 950 Taiwanese 0x404), else 0.
pub fn cp_to_lcid(_e: &mut Engine, code_page: u32) -> u32 {
    match code_page {
        932 => 0x411,
        936 => 0x804,
        949 => 0x412,
        950 => 0x404,
        _ => 0,
    }
}

// Translated from 00ed3ae8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `setSBCS` (`EAX`: the multibyte info): a single-byte code page. `mbctype` (+0x1C, 0x101
/// bytes) cleared, code page, multibyte flag and locale (+4, +8, +0xC) and the `mbulinfo`
/// words (+0x10 to +0x1B) zero, then `mbctype` and `mbcasemap` (+0x11D, 0x100 bytes) copied
/// from the initial multibyte info's.
pub fn set_sbcs(e: &mut Engine, info: u32) {
    e.call(MEMSET, &args![info.wrapping_add(0x1c), 0u32, 0x101u32]);
    for offset in [4u32, 8, 0xc, 0x10, 0x14, 0x18] {
        e.mem.set_u32(info.wrapping_add(offset), 0);
    }
    for i in 0..0x201u32 {
        let byte = e.mem.u8(INITIAL_MBCINFO + 0x1c + i);
        e.mem.set_u8(info.wrapping_add(0x1c + i), byte);
    }
}

// Translated from 00ed3b4c (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed3b4c - the /GS stack cookie check (00ec408c) is not translated
/// `setSBUpLow` (`ESI`: the multibyte info): the upper/lower flags (0x10/0x20 in `mbctype`,
/// +0x1D) and the case map (+0x11D) of the 256 single bytes. With the code page's `CPINFO`
/// (`GetCPInfo`) the bytes 0 to 255, with a space for byte 0 and every lead byte range, are
/// classified (`__crtGetStringTypeA`, `CT_CTYPE1`) and mapped (`__crtLCMapStringA`, lower and
/// upper): an upper byte maps to its lower form, a lower one to its upper form, others to 0.
/// Without it, only 'A' to 'Z' and 'a' to 'z' are.
pub fn set_sb_up_low(e: &mut Engine, info: u32) {
    let code_page = e.mem.u32(info.wrapping_add(4));
    let lcid = e.mem.u32(info.wrapping_add(0xc));
    e.with_stack(0x518, |e, frame| {
        let frame = frame.addr();
        let cpinfo = frame;
        let types = frame + 0x14;
        let upper = frame + 0x214;
        let lower = frame + 0x314;
        let bytes = frame + 0x414;
        let ok = e.call(API_GET_CP_INFO, &args![code_page, cpinfo]).u32();
        if ok != 0 {
            for i in 0..0x100u32 {
                e.mem.set_u8(bytes + i, i as u8);
            }
            e.mem.set_u8(bytes, 0x20);
            let mut pair = cpinfo + 6;
            loop {
                let low = e.mem.u8(pair);
                if low == 0 {
                    break;
                }
                let high = e.mem.u8(pair + 1);
                if low <= high {
                    let count = u32::from(high - low) + 1;
                    e.call(MEMSET, &args![bytes + u32::from(low), 0x20u32, count]);
                }
                pair += 2;
            }
            e.call(
                GET_STRING_TYPE_A,
                &args![0u32, 1u32, bytes, 0x100u32, types, code_page, lcid, 0u32],
            );
            e.call(
                LC_MAP_STRING_A,
                &args![0u32, lcid, 0x100u32, bytes, 0x100u32, lower, 0x100u32, code_page, 0u32],
            );
            e.call(
                LC_MAP_STRING_A,
                &args![0u32, lcid, 0x200u32, bytes, 0x100u32, upper, 0x100u32, code_page, 0u32],
            );
            for i in 0..0x100u32 {
                let kind = e.mem.u16(types + i * 2);
                let flags = info.wrapping_add(0x1d + i);
                let map = info.wrapping_add(0x11d + i);
                if kind & 1 != 0 {
                    let f = e.mem.u8(flags);
                    e.mem.set_u8(flags, f | 0x10);
                    let to = e.mem.u8(lower + i);
                    e.mem.set_u8(map, to);
                } else if kind & 2 != 0 {
                    let f = e.mem.u8(flags);
                    e.mem.set_u8(flags, f | 0x20);
                    let to = e.mem.u8(upper + i);
                    e.mem.set_u8(map, to);
                } else {
                    e.mem.set_u8(map, 0);
                }
            }
        } else {
            for i in 0..0x100u32 {
                let flags = info.wrapping_add(0x1d + i);
                let map = info.wrapping_add(0x11d + i);
                if i.wrapping_sub(0x41) <= 0x19 {
                    let f = e.mem.u8(flags);
                    e.mem.set_u8(flags, f | 0x10);
                    e.mem.set_u8(map, (i as u8).wrapping_add(0x20));
                } else if i.wrapping_sub(0x61) <= 0x19 {
                    let f = e.mem.u8(flags);
                    e.mem.set_u8(flags, f | 0x20);
                    e.mem.set_u8(map, (i as u8).wrapping_sub(0x20));
                } else {
                    e.mem.set_u8(map, 0);
                }
            }
        }
    });
}

// Translated from 00ed3d83 (disassembled, FalloutNV.exe 1.4.0.525)
/// `getSystemCP` (`ESI`: the requested code page): `_MB_CP_OEM` (-2) answers `GetOEMCP()`,
/// `_MB_CP_ANSI` (-3) `GetACP()`, `_MB_CP_LOCALE` (-4) the thread locale's code page (+4),
/// each setting `fSystemSet` (`01270468`) to 1; any other value is answered as it is, with
/// `fSystemSet` 0.
pub fn get_system_cp(e: &mut Engine, code_page: u32) -> u32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, 0u32]);
        e.mem.set_u32(SYSTEM_SET, 0);
        let result = match code_page as i32 {
            -2 => {
                e.mem.set_u32(SYSTEM_SET, 1);
                e.call(API_GET_OEMCP, &[]).u32()
            }
            -3 => {
                e.mem.set_u32(SYSTEM_SET, 1);
                e.call(API_GET_ACP, &[]).u32()
            }
            -4 => {
                let info = e.mem.u32(update);
                e.mem.set_u32(SYSTEM_SET, 1);
                e.mem.u32(info.wrapping_add(4))
            }
            _ => code_page,
        };
        locale_update_release(e, update);
        result
    })
}

/// `mbctype` byte of `byte` (+0x1D + byte) or'ed with `bits`.
fn mark(e: &mut Engine, info: u32, byte: u32, bits: u8) {
    let at = info.wrapping_add(0x1d).wrapping_add(byte);
    let old = e.mem.u8(at);
    e.mem.set_u8(at, old | bits);
}

// Translated from 00ed3dff (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed3dff - the /GS stack cookie check (00ec408c) is not translated
/// `__setmbcp_nolock` (cdecl: code page, multibyte info): fills the info for the code page
/// `getSystemCP` gives. 0 sets a single-byte table (`setSBCS`). A code page of the
/// built-in table (`__rgcpinfo`, five entries) takes its four lead-byte range lists, each
/// with its type bit, its `mbulinfo` words and its locale (`CPtoLCID`). UTF-7/UTF-8
/// (65000/65001) and invalid code pages answer -1. Any other uses `GetCPInfo`: lead byte
/// ranges get bit 4 and bytes 1 to 254 bit 8 when it is multibyte; when `GetCPInfo` fails, a
/// system code page falls back to the single-byte table and others answer -1. The case
/// tables then come from `setSBUpLow`. Answers 0 on success.
pub fn setmbcp_nolock(e: &mut Engine, code_page: u32, info: u32) -> i32 {
    let code_page = e.call(GET_SYSTEM_CP, &args![code_page]).u32();
    if code_page == 0 {
        e.call(SET_SBCS, &args![info]);
        return 0;
    }
    if let Some(index) = (0..5u32).find(|i| e.mem.u32(CP_TABLE + i * 0x30) == code_page) {
        let entry = index * 0x30;
        e.call(MEMSET, &args![info.wrapping_add(0x1c), 0u32, 0x101u32]);
        for list in 0..4u32 {
            let mut pair = CP_TABLE + 0x10 + entry + list * 8;
            while e.mem.u8(pair) != 0 {
                let high = e.mem.u8(pair + 1);
                if high == 0 {
                    break;
                }
                let bits = e.mem.u8(CP_TYPE_FLAGS + list);
                let mut byte = u32::from(e.mem.u8(pair));
                while byte <= u32::from(e.mem.u8(pair + 1)) {
                    mark(e, info, byte, bits);
                    byte += 1;
                }
                pair += 2;
            }
        }
        e.mem.set_u32(info.wrapping_add(4), code_page);
        e.mem.set_u32(info.wrapping_add(8), 1);
        let lcid = e.call(CP_TO_LCID, &args![code_page]).u32();
        e.mem.set_u32(info.wrapping_add(0xc), lcid);
        for i in 0..6u32 {
            let word = e.mem.u16(CP_TABLE + 4 + entry + i * 2);
            e.mem.set_u16(info.wrapping_add(0x10 + i * 2), word);
        }
        e.call(SET_SB_UP_LOW, &args![info]);
        return 0;
    }
    if code_page == 65000 || code_page == 65001 {
        return -1;
    }
    if e.call(API_IS_VALID_CODE_PAGE, &args![code_page & 0xffff])
        .u32()
        == 0
    {
        return -1;
    }
    e.with_stack(0x14, |e, cpinfo| {
        let cpinfo = cpinfo.addr();
        if e.call(API_GET_CP_INFO, &args![code_page, cpinfo]).u32() == 0 {
            if e.mem.u32(SYSTEM_SET) != 0 {
                e.call(SET_SBCS, &args![info]);
                return 0;
            }
            return -1;
        }
        e.call(MEMSET, &args![info.wrapping_add(0x1c), 0u32, 0x101u32]);
        e.mem.set_u32(info.wrapping_add(4), code_page);
        e.mem.set_u32(info.wrapping_add(0xc), 0);
        if e.mem.u32(cpinfo) > 1 {
            if e.mem.u8(cpinfo + 6) != 0 {
                let mut pair = cpinfo + 6;
                loop {
                    let high = e.mem.u8(pair + 1);
                    if high == 0 {
                        break;
                    }
                    let low = u32::from(e.mem.u8(pair));
                    for byte in low..=u32::from(high) {
                        mark(e, info, byte, 4);
                    }
                    pair += 2;
                    if e.mem.u8(pair) == 0 {
                        break;
                    }
                }
            }
            for byte in 1..0xffu32 {
                mark(e, info, byte, 8);
            }
            let page = e.mem.u32(info.wrapping_add(4));
            let lcid = e.call(CP_TO_LCID, &args![page]).u32();
            e.mem.set_u32(info.wrapping_add(0xc), lcid);
            e.mem.set_u32(info.wrapping_add(8), 1);
        } else {
            e.mem.set_u32(info.wrapping_add(8), 0);
        }
        for offset in [0x10u32, 0x14, 0x18] {
            e.mem.set_u32(info.wrapping_add(offset), 0);
        }
        e.call(SET_SB_UP_LOW, &args![info]);
        0
    })
}

// Translated from 00ed3fe4 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed3fe4 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 011802e8) is not translated; its __finally block (00ed4145) is called as the exe calls it on the way out.
/// `__setmbcp` (cdecl: the code page): when `getSystemCP` gives a code page other than the
/// thread's (`_getptd()` +0x68 after `__updatetmbcinfo`), a 0x220-byte copy of the thread's
/// info (reference count 0) is filled by `__setmbcp_nolock`. On success it replaces the
/// thread's (the old one freed when its count drops to 0, unless it is the initial one) and,
/// unless the thread has its own locale (+0x70 bit 2) or the global locale is per-thread
/// (`__globallocalestatus` bit 0), the global copies and `__ptmbcinfo` under lock 0xD. A
/// failure with -1 frees the copy and sets `errno` to `EINVAL`. Answers 0, -1 or the
/// failure; 0 when the code page is the current one, -1 when the copy cannot be allocated.
pub fn setmbcp(e: &mut Engine, code_page: u32) -> i32 {
    let ptd = e.call(GETPTD, &[]).u32();
    e.call(UPDATETMBCINFO, &[]);
    let current = e.mem.u32(ptd.wrapping_add(0x68));
    let code_page = e.call(GET_SYSTEM_CP, &args![code_page]).u32();
    if code_page == e.mem.u32(current.wrapping_add(4)) {
        return 0;
    }
    let info = e.call(MALLOC_CRT, &args![0x220u32]).u32();
    if info == 0 {
        return -1;
    }
    let source = e.mem.u32(ptd.wrapping_add(0x68));
    let bytes = e.mem.bytes(source, 0x220);
    e.mem.write(info, &bytes);
    e.mem.set_u32(info, 0);
    let result = e.call(SETMBCP_NOLOCK, &args![code_page, info]).u32() as i32;
    if result != 0 {
        if result == -1 {
            if info != INITIAL_MBCINFO {
                e.call(FREE, &args![info]);
            }
            let errno = e.call(ERRNO, &[]).u32();
            e.mem.set_u32(errno, 0x16);
        }
        return result;
    }
    let old = e.mem.u32(ptd.wrapping_add(0x68));
    if e.call(API_INTERLOCKED_DECREMENT, &args![old]).u32() == 0 && old != INITIAL_MBCINFO {
        e.call(FREE, &args![old]);
    }
    e.mem.set_u32(ptd.wrapping_add(0x68), info);
    e.call(API_INTERLOCKED_INCREMENT, &args![info]);
    if e.mem.u32(ptd.wrapping_add(0x70)) & 2 != 0 || e.mem.u8(GLOBAL_LOCALE_STATUS) & 1 != 0 {
        return result;
    }
    e.call(LOCK, &args![0xdu32]);
    for (target, offset) in [(MB_CODEPAGE, 4u32), (IS_MB_CODEPAGE, 8), (MB_LCID, 0xc)] {
        let word = e.mem.u32(info + offset);
        e.mem.set_u32(target, word);
    }
    for i in 0..5u32 {
        let word = e.mem.u16(info + 0x10 + i * 2);
        e.mem.set_u16(MB_ULINFO + i * 2, word);
    }
    for i in 0..0x101u32 {
        let byte = e.mem.u8(info + 0x1c + i);
        e.mem.set_u8(MB_CTYPE + i, byte);
    }
    for i in 0..0x100u32 {
        let byte = e.mem.u8(info + 0x11d + i);
        e.mem.set_u8(MB_CASEMAP + i, byte);
    }
    let global = e.mem.u32(GLOBAL_MBCINFO);
    if e.call(API_INTERLOCKED_DECREMENT, &args![global]).u32() == 0 {
        let global = e.mem.u32(GLOBAL_MBCINFO);
        if global != INITIAL_MBCINFO {
            e.call(FREE, &args![global]);
        }
    }
    e.mem.set_u32(GLOBAL_MBCINFO, info);
    e.call(API_INTERLOCKED_INCREMENT, &args![info]);
    e.call(SETMBCP_FINALLY, &[]);
    result
}

// Translated from 00ed2af0 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed2af0 - the /GS stack cookie check (00ec408c) is not translated
/// `__getlocaleinfo` (cdecl: locale info, type, locale id, field, out). Type 1 (a string):
/// the field through `__crtGetLocaleInfoA` into a 0x80-byte local, or, when that is too
/// small (`GetLastError` 0x7A), into a `_calloc_crt` block of the size it asks for; the text
/// is copied to a new `_calloc_crt` block stored at `out` (`strncpy_s`, `__invoke_watson` on
/// failure). Type 0 (an integer): the field's four wide characters (`__crtGetLocaleInfoW`
/// into `0127045c`) read as decimal digits into the byte at `out`, stopping at the first
/// non-digit. Answers 0, or -1 when a step fails or for any other type.
pub fn getlocaleinfo(
    e: &mut Engine,
    locale: u32,
    kind: u32,
    lcid: u32,
    field: u32,
    out: u32,
) -> i32 {
    if kind == 1 {
        return e.with_stack(0x80, |e, local| {
            let local = local.addr();
            let mut buffer = local;
            let mut allocated = false;
            let mut size = e
                .call(
                    CRT_GET_LOCALE_INFO_A,
                    &args![locale, lcid, field, local, 0x80u32, 0u32],
                )
                .u32();
            if size == 0 {
                if e.call(API_GET_LAST_ERROR, &[]).u32() != 0x7a {
                    return -1;
                }
                let needed = e
                    .call(
                        CRT_GET_LOCALE_INFO_A,
                        &args![locale, lcid, field, 0u32, 0u32, 0u32],
                    )
                    .u32();
                if needed == 0 {
                    return -1;
                }
                buffer = e.call(CALLOC_CRT, &args![needed, 1u32]).u32();
                if buffer == 0 {
                    return -1;
                }
                allocated = true;
                size = e
                    .call(
                        CRT_GET_LOCALE_INFO_A,
                        &args![locale, lcid, field, buffer, needed, 0u32],
                    )
                    .u32();
                if size == 0 {
                    e.call(FREE, &args![buffer]);
                    return -1;
                }
            }
            let copy = e.call(CALLOC_CRT, &args![size, 1u32]).u32();
            e.mem.set_u32(out, copy);
            if copy == 0 {
                if allocated {
                    e.call(FREE, &args![buffer]);
                }
                return -1;
            }
            let failed = e
                .call(STRNCPY_S, &args![copy, size, buffer, size.wrapping_sub(1)])
                .u32();
            if failed != 0 {
                e.call(INVOKE_WATSON, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
            }
            if allocated {
                e.call(FREE, &args![buffer]);
            }
            0
        });
    }
    if kind != 0 {
        return -1;
    }
    let read = e
        .call(
            CRT_GET_LOCALE_INFO_W,
            &args![locale, lcid, field, INT_FIELD_BUFFER, 4u32, 0u32],
        )
        .u32();
    if read == 0 {
        return -1;
    }
    e.mem.set_u8(out, 0);
    for i in 0..4u32 {
        let c = e.mem.u8(INT_FIELD_BUFFER + i * 2);
        if e.call(ISDIGIT, &args![u32::from(c)]).u32() == 0 {
            break;
        }
        let value = e
            .mem
            .u8(out)
            .wrapping_mul(10)
            .wrapping_add(c)
            .wrapping_sub(0x30);
        e.mem.set_u8(out, value);
    }
    0
}

// Translated from 00ee1df7 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__crtGetLocaleInfoW` (cdecl: locale, locale id, field, buffer, size): `GetLocaleInfoW`
/// with the last four, inside a `_LocaleUpdate` of the locale. Answers its result.
pub fn crt_get_locale_info_w(
    e: &mut Engine,
    locale: u32,
    lcid: u32,
    field: u32,
    buffer: u32,
    size: u32,
) -> u32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let result = e
            .call(API_GET_LOCALE_INFO_W, &args![lcid, field, buffer, size])
            .u32();
        locale_update_release(e, update);
        result
    })
}

// Translated from 00ee1f6a (disassembled, FalloutNV.exe 1.4.0.525)
/// `__crtGetLocaleInfoA` (cdecl: locale, locale id, field, buffer, size, code page): the
/// worker `00ee1e2b` with the `_LocaleUpdate` object first. Answers its result.
pub fn crt_get_locale_info_a(
    e: &mut Engine,
    locale: u32,
    lcid: u32,
    field: u32,
    buffer: u32,
    size: u32,
    code_page: u32,
) -> u32 {
    e.with_stack(0x10, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let result = e
            .call(
                GET_LOCALE_INFO_A_WORKER,
                &args![update, lcid, field, buffer, size, code_page],
            )
            .u32();
        locale_update_release(e, update);
        result
    })
}

// Translated from 00ee02c9 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__free_lc_time` (cdecl: an `__lc_time_data`, may be null): frees its 43 strings (+0 to
/// +0xA8), in the exe's order.
pub fn free_lc_time(e: &mut Engine, data: u32) {
    if data == 0 {
        return;
    }
    let mut order: Vec<u32> = vec![4, 8, 0xc, 0x10, 0x14, 0x18, 0];
    order.extend((0x20..=0x34).step_by(4));
    order.push(0x1c);
    order.extend((0x38..=0xa8).step_by(4));
    for offset in order {
        let text = e.mem.u32(data.wrapping_add(offset));
        e.call(FREE, &args![text]);
    }
}

/// Frees each of `count` words from `first` (+ 4 per word) of `lconv` that differs from the
/// default at the same position from `defaults`.
fn free_lconv_fields(e: &mut Engine, lconv: u32, first: u32, defaults: u32, count: u32) {
    if lconv == 0 {
        return;
    }
    for i in 0..count {
        let text = e.mem.u32(lconv.wrapping_add(first + i * 4));
        if text != e.mem.u32(defaults + i * 4) {
            e.call(FREE, &args![text]);
        }
    }
}

// Translated from 00ee04d7 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__free_lconv_num` (cdecl: an `lconv`, may be null): frees `decimal_point`,
/// `thousands_sep` and `grouping` (+0 to +8) where they are not the C locale's (`011c19d4`).
pub fn free_lconv_num(e: &mut Engine, lconv: u32) {
    free_lconv_fields(e, lconv, 0, LCONV_NUM_DEFAULTS, 3);
}

// Translated from 00ee06e6 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__free_lconv_mon` (cdecl: an `lconv`, may be null): frees the seven monetary strings
/// (+0xC to +0x24) where they are not the C locale's (`011c19e0`).
pub fn free_lconv_mon(e: &mut Engine, lconv: u32) {
    free_lconv_fields(e, lconv, 0xc, LCONV_MON_DEFAULTS, 7);
}

// Translated from 00edd3f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__towlower_l` (cdecl: wide character, locale): `WEOF` (0xFFFF) as it is. In the "C"
/// locale (no `LC_CTYPE` handle, locale info +0x14) 'A' to 'Z' are lowered. Below 0x100 an
/// upper character (`00ee2332` with `_UPPER`) maps through the locale's lower map (+0xCC);
/// above, `__crtLCMapStringW` (`LCMAP_LOWERCASE`) maps it when it can. Answers the 16-bit
/// result.
pub fn towlower_l(e: &mut Engine, c: u32, locale: u32) -> u32 {
    if c as u16 == 0xffff {
        return 0xffff;
    }
    e.with_stack(0x18, |e, frame| {
        let update = frame.addr();
        let slot = update + 0x10;
        let lowered = update + 0x14;
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let info = e.mem.u32(update);
        let handle = e.mem.u32(info.wrapping_add(0x14));
        let result = if handle == 0 {
            if (c as u16).wrapping_sub(0x41) <= 0x19 {
                u32::from((c as u16).wrapping_add(0x20))
            } else {
                u32::from(c as u16)
            }
        } else if (c as u16) < 0x100 {
            if e.call(CLASS_TEST, &args![c, 1u32, update]).u32() != 0 {
                let map = e.mem.u32(info.wrapping_add(0xcc));
                u32::from(e.mem.u8(map.wrapping_add(u32::from(c as u16))))
            } else {
                u32::from(c as u16)
            }
        } else {
            e.mem.set_u32(slot, c);
            let code_page = e.mem.u32(info.wrapping_add(4));
            let mapped = e
                .call(
                    LC_MAP_STRING_W,
                    &args![update, handle, 0x100u32, slot, 1u32, lowered, 1u32, code_page],
                )
                .u32();
            if mapped != 0 {
                u32::from(e.mem.u16(lowered))
            } else {
                u32::from(c as u16)
            }
        };
        locale_update_release(e, update);
        result
    })
}

/// This file's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ed_3ab9, cp_to_lcid(u32) -> u32),
        entry!(0x00ed_3ae8, set_sbcs(u32)),
        entry!(0x00ed_3b4c, set_sb_up_low(u32)),
        entry!(0x00ed_3d83, get_system_cp(u32) -> u32),
        entry!(0x00ed_3dff, setmbcp_nolock(u32, u32) -> i32),
        entry!(0x00ed_3fe4, setmbcp(u32) -> i32),
        entry!(0x00ed_2af0, getlocaleinfo(u32, u32, u32, u32, u32) -> i32),
        entry!(0x00ee_1df7, crt_get_locale_info_w(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ee_1f6a, crt_get_locale_info_a(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ee_02c9, free_lc_time(u32)),
        entry!(0x00ee_04d7, free_lconv_num(u32)),
        entry!(0x00ee_06e6, free_lconv_mon(u32)),
        entry!(0x00ed_d3f0, towlower_l(u32, u32) -> u32),
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

    /// An engine with the globals mapped, `memset`, and a `_LocaleUpdate` double that
    /// answers `info` (a thread that owns nothing).
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011c_1000, 0x2000);
        e.map(0x0127_0000, 0x1000);
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            a[0].into_ret()
        });
        let info = e.mem.alloc(0x100);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u32(a[0] + 8, 0);
            e.mem.set_u8(a[0] + 0xc, 0);
            a[0].into_ret()
        });
        (e, info)
    }

    #[test]
    fn cp_to_lcid_knows_the_east_asian_pages() {
        let mut e = Engine::new();
        for (page, lcid) in [
            (932, 0x411),
            (936, 0x804),
            (949, 0x412),
            (950, 0x404),
            (1252, 0),
        ] {
            assert_eq!(e.call(CP_TO_LCID, &[page]).u32(), lcid);
        }
    }

    #[test]
    fn set_sbcs_copies_the_initial_tables() {
        let (mut e, _) = engine();
        for i in 0..0x201 {
            e.mem.set_u8(INITIAL_MBCINFO + 0x1c + i, (i * 7) as u8);
        }
        let info = e.mem.alloc(0x220);
        for i in 0..0x220 {
            e.mem.set_u8(info + i, 0xee);
        }
        e.call(SET_SBCS, &[info]);
        for offset in [4, 8, 0xc, 0x10, 0x14, 0x18] {
            assert_eq!(e.mem.u32(info + offset), 0);
        }
        for i in 0..0x201 {
            assert_eq!(e.mem.u8(info + 0x1c + i), (i * 7) as u8);
        }
        assert_eq!(e.mem.u32(info), 0xeeee_eeee);
    }

    #[test]
    fn set_sb_up_low_without_cpinfo_maps_ascii_letters() {
        let (mut e, _) = engine();
        e.register(API_GET_CP_INFO, |_, _| 0u32.into_ret());
        let info = e.mem.alloc(0x220);
        e.call(SET_SB_UP_LOW, &[info]);
        assert_eq!(e.mem.u8(info + 0x1d + 0x41), 0x10);
        assert_eq!(e.mem.u8(info + 0x11d + 0x41), 0x61);
        assert_eq!(e.mem.u8(info + 0x1d + 0x7a), 0x20);
        assert_eq!(e.mem.u8(info + 0x11d + 0x7a), 0x5a);
        assert_eq!(e.mem.u8(info + 0x1d + 0x31), 0);
        assert_eq!(e.mem.u8(info + 0x11d + 0x31), 0);
    }

    #[test]
    fn set_sb_up_low_with_cpinfo_classifies_and_maps() {
        let (mut e, _) = engine();
        e.register(API_GET_CP_INFO, |e, a| {
            e.mem.set_u32(a[1], 2);
            // One lead byte range, 0x81 to 0x82.
            e.mem.write(a[1] + 6, &[0x81, 0x82, 0, 0]);
            1u32.into_ret()
        });
        e.register(GET_STRING_TYPE_A, |e, a| {
            for i in 0..0x100 {
                let b = e.mem.u8(a[2] + i);
                let kind = if b.is_ascii_uppercase() {
                    1
                } else if b.is_ascii_lowercase() {
                    2
                } else {
                    0
                };
                e.mem.set_u16(a[4] + i * 2, kind);
            }
            1u32.into_ret()
        });
        e.register(LC_MAP_STRING_A, |e, a| {
            for i in 0..0x100 {
                let b = e.mem.u8(a[3] + i);
                let m = if a[2] == 0x100 {
                    b.to_ascii_lowercase()
                } else {
                    b.to_ascii_uppercase()
                };
                e.mem.set_u8(a[5] + i, m);
            }
            1u32.into_ret()
        });
        let info = e.mem.alloc(0x220);
        e.mem.set_u32(info + 4, 1252);
        e.mem.set_u32(info + 0xc, 0x409);
        e.call_log = Some(vec![]);
        e.call(SET_SB_UP_LOW, &[info]);
        assert_eq!(e.mem.u8(info + 0x11d + 0x42), 0x62);
        assert_eq!(e.mem.u8(info + 0x1d + 0x42), 0x10);
        assert_eq!(e.mem.u8(info + 0x11d + 0x62), 0x42);
        assert_eq!(e.mem.u8(info + 0x1d + 0x62), 0x20);
        let types = calls(&e, GET_STRING_TYPE_A);
        assert_eq!(types[0][..2], [0, 1]);
        assert_eq!(types[0][3], 0x100);
        assert_eq!(types[0][5..], [1252, 0x409, 0]);
        // The lead bytes and byte 0 were spaces in the classified text.
        let memsets = calls(&e, MEMSET);
        assert_eq!(memsets.len(), 1);
        assert_eq!(memsets[0][1..], [0x20, 2]);
        let maps = calls(&e, LC_MAP_STRING_A);
        assert_eq!((maps[0][1], maps[0][2], maps[1][2]), (0x409, 0x100, 0x200));
    }

    #[test]
    fn get_system_cp_resolves_the_special_pages() {
        let (mut e, info) = engine();
        e.mem.set_u32(info + 4, 1251);
        e.register(API_GET_OEMCP, |_, _| 437u32.into_ret());
        e.register(API_GET_ACP, |_, _| 1252u32.into_ret());
        assert_eq!(e.call(GET_SYSTEM_CP, &[(-2i32) as u32]).u32(), 437);
        assert_eq!(e.mem.u32(SYSTEM_SET), 1);
        assert_eq!(e.call(GET_SYSTEM_CP, &[(-3i32) as u32]).u32(), 1252);
        assert_eq!(e.call(GET_SYSTEM_CP, &[(-4i32) as u32]).u32(), 1251);
        assert_eq!(e.call(GET_SYSTEM_CP, &[850]).u32(), 850);
        assert_eq!(e.mem.u32(SYSTEM_SET), 0);
    }

    #[test]
    fn setmbcp_nolock_takes_the_builtin_table() {
        let (mut e, _) = engine();
        nothing(&mut e, &[SET_SB_UP_LOW]);
        // Entry 1 is code page 932 with one range in list 0 and one in list 2.
        let entry = CP_TABLE + 0x30;
        e.mem.set_u32(entry, 932);
        e.mem.write(entry + 0x10, &[0x81, 0x83, 0, 0]);
        e.mem.write(entry + 0x20, &[0xa1, 0xa1, 0, 0]);
        for i in 0..6 {
            e.mem.set_u16(entry + 4 + i * 2, 0x100 + i as u16);
        }
        e.mem.write(CP_TYPE_FLAGS, &[4, 0, 1, 0]);
        let info = e.mem.alloc(0x220);
        assert_eq!(e.call(SETMBCP_NOLOCK, &[932, info]).i32(), 0);
        assert_eq!(e.mem.u32(info + 4), 932);
        assert_eq!(e.mem.u32(info + 8), 1);
        assert_eq!(e.mem.u32(info + 0xc), 0x411);
        assert_eq!(e.mem.u8(info + 0x1d + 0x81), 4);
        assert_eq!(e.mem.u8(info + 0x1d + 0x83), 4);
        assert_eq!(e.mem.u8(info + 0x1d + 0x84), 0);
        assert_eq!(e.mem.u8(info + 0x1d + 0xa1), 1);
        assert_eq!(e.mem.u16(info + 0x1a), 0x105);
    }

    #[test]
    fn setmbcp_nolock_asks_windows_for_other_pages() {
        let (mut e, _) = engine();
        nothing(&mut e, &[SET_SB_UP_LOW, SET_SBCS]);
        e.register(API_IS_VALID_CODE_PAGE, |_, a| {
            u32::from(a[0] != 1).into_ret()
        });
        e.register(API_GET_CP_INFO, |e, a| {
            if a[0] == 2 {
                return 0u32.into_ret();
            }
            e.mem.set_u32(a[1], if a[0] == 1253 { 1 } else { 2 });
            e.mem.write(a[1] + 6, &[0x90, 0x91, 0, 0]);
            1u32.into_ret()
        });
        let info = e.mem.alloc(0x220);
        assert_eq!(e.call(SETMBCP_NOLOCK, &[65001, info]).i32(), -1);
        assert_eq!(e.call(SETMBCP_NOLOCK, &[1, info]).i32(), -1);
        // GetCPInfo fails: -1, or the single-byte table for a system code page.
        assert_eq!(e.call(SETMBCP_NOLOCK, &[2, info]).i32(), -1);
        // (_MB_CP_ANSI through GetACP sets fSystemSet.)
        e.register(API_GET_ACP, |_, _| 2u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(SETMBCP_NOLOCK, &[(-3i32) as u32, info]).i32(), 0);
        assert_eq!(calls(&e, SET_SBCS), vec![vec![info]]);
        // A double-byte page: lead bytes bit 4, bytes 1 to 254 bit 8.
        assert_eq!(e.call(SETMBCP_NOLOCK, &[51949, info]).i32(), 0);
        assert_eq!(e.mem.u8(info + 0x1d + 0x90), 0xc);
        assert_eq!(e.mem.u8(info + 0x1d + 0x92), 8);
        assert_eq!(e.mem.u8(info + 0x1d + 0xff), 0);
        assert_eq!((e.mem.u32(info + 8), e.mem.u32(info + 0xc)), (1, 0));
        // A single-byte page.
        assert_eq!(e.call(SETMBCP_NOLOCK, &[1253, info]).i32(), 0);
        assert_eq!(e.mem.u8(info + 0x1d + 0x90), 0);
        assert_eq!(e.mem.u32(info + 8), 0);
        // Zero: the single-byte table.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(SETMBCP_NOLOCK, &[0, info]).i32(), 0);
        assert_eq!(calls(&e, SET_SBCS), vec![vec![info]]);
    }

    #[test]
    fn setmbcp_installs_the_new_info() {
        let (mut e, _) = engine();
        let ptd = e.mem.alloc(0x80);
        let old = e.mem.alloc(0x220);
        e.mem.set_u32(old, 1);
        e.mem.set_u32(old + 4, 1252);
        e.mem.set_u32(ptd + 0x68, old);
        e.mem.set_u32(GLOBAL_MBCINFO, old);
        e.register_double(GETPTD, move |_, _| ptd.into_ret());
        e.register(MALLOC_CRT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SETMBCP_NOLOCK, |e, a| {
            e.mem.set_u32(a[1] + 4, a[0]);
            e.mem.set_u8(a[1] + 0x1c + 0x82, 4);
            e.mem.set_u8(a[1] + 0x11d + 0x41, 0x61);
            0u32.into_ret()
        });
        e.register(API_INTERLOCKED_DECREMENT, |e, a| {
            let n = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], n);
            n.into_ret()
        });
        e.register(API_INTERLOCKED_INCREMENT, |e, a| {
            let n = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], n);
            n.into_ret()
        });
        nothing(&mut e, &[UPDATETMBCINFO, FREE, LOCK, 0x00ed_de5f]);
        e.call_log = Some(vec![]);
        // The current page: nothing changes.
        assert_eq!(e.call(0x00ed_3fe4, &[1252]).i32(), 0);
        assert!(calls(&e, MALLOC_CRT).is_empty());
        assert_eq!(e.call(0x00ed_3fe4, &[932]).i32(), 0);
        let info = e.mem.u32(ptd + 0x68);
        assert_ne!(info, old);
        assert_eq!(e.mem.u32(GLOBAL_MBCINFO), info);
        assert_eq!(e.mem.u32(info), 2);
        assert_eq!(e.mem.u32(MB_CODEPAGE), 932);
        assert_eq!(e.mem.u8(MB_CTYPE + 0x82), 4);
        assert_eq!(e.mem.u8(MB_CASEMAP + 0x41), 0x61);
        // The old info dropped to 0 references on the second decrement and was freed.
        assert_eq!(calls(&e, FREE), vec![vec![old]]);
        assert_eq!(calls(&e, LOCK), vec![vec![0xd]]);
        assert_eq!(calls(&e, 0x00ed_de5f), vec![vec![0xd]]);
    }

    #[test]
    fn setmbcp_frees_the_copy_on_failure() {
        let (mut e, _) = engine();
        let ptd = e.mem.alloc(0x80);
        let old = e.mem.alloc(0x220);
        e.mem.set_u32(ptd + 0x68, old);
        let errno = e.mem.alloc(4);
        e.register_double(GETPTD, move |_, _| ptd.into_ret());
        e.register_double(ERRNO, move |_, _| errno.into_ret());
        e.register(MALLOC_CRT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(SETMBCP_NOLOCK, |_, _| (-1i32 as u32).into_ret());
        nothing(&mut e, &[UPDATETMBCINFO, FREE]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_3fe4, &[932]).i32(), -1);
        assert_eq!(calls(&e, FREE).len(), 1);
        assert_eq!(e.mem.u32(errno), 0x16);
        assert_eq!(e.mem.u32(ptd + 0x68), old);
    }

    #[test]
    fn getlocaleinfo_reads_strings_and_numbers() {
        let (mut e, _) = engine();
        e.register(CRT_GET_LOCALE_INFO_A, |e, a| {
            if a[3] != 0 {
                e.mem.write(a[3], b"Mon\0");
            }
            4u32.into_ret()
        });
        e.register(CALLOC_CRT, |e, a| e.mem.alloc(a[0] * a[1]).into_ret());
        e.register(STRNCPY_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            e.mem.set_u8(a[0] + a[3], 0);
            0u32.into_ret()
        });
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00ed_2af0, &[0, 1, 0x409, 0x2a, out]).i32(), 0);
        assert_eq!(e.mem.cstr(e.mem.u32(out)), b"Mon");
        e.register(CRT_GET_LOCALE_INFO_W, |e, a| {
            for (i, c) in b"12x4".iter().enumerate() {
                e.mem.set_u16(a[3] + 2 * i as u32, u16::from(*c));
            }
            3u32.into_ret()
        });
        e.register(ISDIGIT, |_, a| {
            u32::from((a[0] as u8).is_ascii_digit()).into_ret()
        });
        assert_eq!(e.call(0x00ed_2af0, &[0, 0, 0x409, 0x14, out]).i32(), 0);
        assert_eq!(e.mem.u8(out), 12);
        assert_eq!(e.call(0x00ed_2af0, &[0, 2, 0x409, 0x14, out]).i32(), -1);
    }

    #[test]
    fn getlocaleinfo_grows_its_buffer_when_asked() {
        let (mut e, _) = engine();
        e.register(CRT_GET_LOCALE_INFO_A, |e, a| {
            if a[4] == 0 {
                return 0x100u32.into_ret();
            }
            if a[4] == 0x80 {
                return 0u32.into_ret();
            }
            e.mem.write(a[3], b"long\0");
            5u32.into_ret()
        });
        e.register(API_GET_LAST_ERROR, |_, _| 0x7au32.into_ret());
        e.register(CALLOC_CRT, |e, a| e.mem.alloc(a[0] * a[1]).into_ret());
        e.register(STRNCPY_S, |_, _| 0u32.into_ret());
        nothing(&mut e, &[FREE]);
        e.call_log = Some(vec![]);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00ed_2af0, &[0, 1, 0x409, 0x2a, out]).i32(), 0);
        let callocs = calls(&e, CALLOC_CRT);
        assert_eq!(callocs, vec![vec![0x100, 1], vec![5, 1]]);
        assert_eq!(calls(&e, FREE).len(), 1);
        // Another error: -1 with nothing allocated.
        e.register(API_GET_LAST_ERROR, |_, _| 5u32.into_ret());
        assert_eq!(e.call(0x00ed_2af0, &[0, 1, 0x409, 0x2a, out]).i32(), -1);
    }

    #[test]
    fn the_locale_info_wrappers_pass_their_words_on() {
        let (mut e, _) = engine();
        e.register(API_GET_LOCALE_INFO_W, |_, a| (a[0] + a[3]).into_ret());
        e.register(GET_LOCALE_INFO_A_WORKER, |_, a| (a[1] + a[5]).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_1df7, &[0, 0x409, 2, 0x900, 4]).u32(), 0x40d);
        assert_eq!(
            calls(&e, API_GET_LOCALE_INFO_W),
            vec![vec![0x409, 2, 0x900, 4]]
        );
        assert_eq!(
            e.call(0x00ee_1f6a, &[0, 0x409, 2, 0x900, 4, 7]).u32(),
            0x410
        );
        let worker = calls(&e, GET_LOCALE_INFO_A_WORKER);
        assert_eq!(worker[0][1..], [0x409, 2, 0x900, 4, 7]);
    }

    #[test]
    fn the_free_functions_skip_defaults() {
        let (mut e, _) = engine();
        nothing(&mut e, &[FREE]);
        let time = e.mem.alloc(0xac);
        for i in 0..43 {
            e.mem.set_u32(time + i * 4, 0x1000 + i);
        }
        e.call_log = Some(vec![]);
        e.call(0x00ee_02c9, &[time]);
        e.call(0x00ee_02c9, &[0]);
        let freed: Vec<u32> = calls(&e, FREE).into_iter().map(|c| c[0]).collect();
        assert_eq!(freed.len(), 43);
        assert_eq!(
            freed[..8],
            [0x1001, 0x1002, 0x1003, 0x1004, 0x1005, 0x1006, 0x1000, 0x1008]
        );
        assert_eq!(freed[13], 0x1007);
        let lconv = e.mem.alloc(0x30);
        for i in 0..10 {
            e.mem.set_u32(lconv + i * 4, 0x2000 + i);
            e.mem.set_u32(LCONV_NUM_DEFAULTS + i * 4, 0x2000 + i);
        }
        e.mem.set_u32(lconv + 4, 0x3333);
        e.mem.set_u32(lconv + 0x24, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x00ee_04d7, &[lconv]);
        e.call(0x00ee_06e6, &[lconv]);
        e.call(0x00ee_06e6, &[0]);
        assert_eq!(calls(&e, FREE), vec![vec![0x3333], vec![0x4444]]);
    }

    #[test]
    fn towlower_lowers_in_each_locale_kind() {
        let (mut e, info) = engine();
        assert_eq!(e.call(0x00ed_d3f0, &[0xffff, 0]).u32(), 0xffff);
        assert_eq!(
            e.call(0x00ed_d3f0, &[u32::from(b'Q'), 0]).u32(),
            u32::from(b'q')
        );
        assert_eq!(e.call(0x00ed_d3f0, &[0xc9, 0]).u32(), 0xc9);
        // A real locale: the map below 0x100, LCMapStringW above.
        e.mem.set_u32(info + 0x14, 0x409);
        e.mem.set_u32(info + 4, 1252);
        let map = e.mem.alloc(0x100);
        e.mem.set_u8(map + 0xc9, 0xe9);
        e.mem.set_u32(info + 0xcc, map);
        e.register(CLASS_TEST, |_, a| u32::from(a[0] == 0xc9).into_ret());
        e.register(LC_MAP_STRING_W, |e, a| {
            let c = e.mem.u16(a[3]);
            e.mem.set_u16(a[5], c + 0x50);
            u32::from(c != 0x500).into_ret()
        });
        assert_eq!(e.call(0x00ed_d3f0, &[0xc9, 0]).u32(), 0xe9);
        assert_eq!(e.call(0x00ed_d3f0, &[0x41, 0]).u32(), 0x41);
        assert_eq!(e.call(0x00ed_d3f0, &[0x410, 0]).u32(), 0x460);
        assert_eq!(e.call(0x00ed_d3f0, &[0x500, 0]).u32(), 0x500);
    }
}

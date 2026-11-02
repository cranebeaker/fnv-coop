//! The C runtime's local-time conversion (MSVC 2008 LIBCMT, statically linked into
//! FalloutNV.exe 1.4.0.525), translated from the exe: `__loctotime64_t` (`00ed7242`, which
//! `_stat64i32` uses for the file times) and the time-zone state under it: `__tzset` and
//! `__tzset_nolock` (the `TZ` variable or `GetTimeZoneInformation`), `_isindst` and its
//! `_nolock` body, `cvtdate`, the `_get_`/`_set_` accessors of `_timezone`, `_daylight` and
//! `_dstbias`, and below them `___lc_codepage_func` and `_getenv_helper_nolock`.

use super::*;

/// `__allmul` (`00ecc300`, four words: two 64-bit values; the product in `EDX:EAX`).
const ALLMUL: u32 = 0x00ec_c300;
/// `_invoke_watson` (`00ec7b08`, five zero words), `_lock` (`00eddf39`) and `_unlock`
/// (`00edde5f`), cdecl.
const INVOKE_WATSON: u32 = 0x00ec_7b08;
const LOCK: u32 = 0x00ed_df39;
const UNLOCK: u32 = 0x00ed_de5f;
/// The CRT's time-zone words: `_timezone` (`011c2b80`), `_daylight` (`011c2b84`) and
/// `_dstbias` (`011c2b88`), and `_tzname` (`011c2c10`, two pointers).
const TIMEZONE: u32 = 0x011c_2b80;
const DAYLIGHT: u32 = 0x011c_2b84;
const DSTBIAS: u32 = 0x011c_2b88;
const TZNAME: u32 = 0x011c_2c10;
/// `cvtdate`'s results: the start of daylight time (year `011c2b64`, day of the year
/// `011c2b68`, milliseconds into it `011c2b6c`) and its end (year `011c2b70`, day `011c2b74`,
/// milliseconds `011c2b78`).
const DST_START_YEAR: u32 = 0x011c_2b64;
const DST_START_DAY: u32 = 0x011c_2b68;
const DST_START_MS: u32 = 0x011c_2b6c;
const DST_END_YEAR: u32 = 0x011c_2b70;
const DST_END_DAY: u32 = 0x011c_2b74;
const DST_END_MS: u32 = 0x011c_2b78;
/// The day tables, indexed by the month from 1 (`base + 4 * month`): the day of the year
/// before each month starts, from -1 (`011c2c48`), and the same for a leap year (`011c2c14`).
const DAYS: u32 = 0x011c_2c48;
const LEAP_DAYS: u32 = 0x011c_2c14;
/// `__tzset`'s once word (`012708d4`), whether the zone came from Windows (`012708cc`), the
/// copy of the last `TZ` value (`012708d0`) and the `TIME_ZONE_INFORMATION` (`01270820`).
const TZSET_DONE: u32 = 0x0127_08d4;
const TZ_FROM_WINDOWS: u32 = 0x0127_08cc;
const LAST_TZ: u32 = 0x0127_08d0;
const TZ_INFO: u32 = 0x0127_0820;
/// `"TZ"` (`01100850`).
const TEXT_TZ: u32 = 0x0110_0850;
/// KERNEL32 import slots: `GetTimeZoneInformation` and `WideCharToMultiByte`.
const IMPORT_GET_TIME_ZONE_INFORMATION: u32 = 0x00fd_f228;
/// CRT functions (cdecl) by address.
const TZSET: u32 = 0x00ee_2beb;
const TZSET_NOLOCK: u32 = 0x00ee_24d6;
const ISINDST: u32 = 0x00ee_2c3a;
const GET_DAYLIGHT: u32 = 0x00ee_2c7b;
const GET_DSTBIAS: u32 = 0x00ee_2cb4;
const GET_TIMEZONE: u32 = 0x00ee_2ced;
const SET_TIMEZONE: u32 = 0x00ee_24c5;
const SET_DAYLIGHT: u32 = 0x00ee_24a3;
const SET_DSTBIAS: u32 = 0x00ee_24b4;
const TIMEZONE_ADDRESS: u32 = 0x00ee_2d32;
const DAYLIGHT_ADDRESS: u32 = 0x00ee_2d26;
const DSTBIAS_ADDRESS: u32 = 0x00ee_2d2c;
const TZNAME_ADDRESS: u32 = 0x00ee_2d38;
const LC_CODEPAGE_FUNC: u32 = 0x00ed_0c93;
const GETENV_HELPER_NOLOCK: u32 = 0x00ee_5f3d;
const STRCMP: u32 = 0x00ec_6da0;
const FREE: u32 = 0x00ec_d291;
const STRLEN: u32 = 0x00ec_6130;
const MALLOC_CRT: u32 = 0x00ed_0cdf;
const STRCPY_S: u32 = 0x00ec_65a6;
const STRNCPY_S: u32 = 0x00ec_8d5f;
const ATOL: u32 = 0x00ec_a6bd;
const GETPTD: u32 = 0x00ed_11de;
const UPDATETLOCINFO: u32 = 0x00ec_f742;
const MBTOW_ENVIRON: u32 = 0x00ee_63c3;
const ENV_NAME_COMPARE: u32 = 0x00ee_63a9;
/// The thread-locale globals `___lc_codepage_func` compares with: `__ptlocinfo`
/// (`011c23f8`) and `__globallocalestatus` (`011c2310`).
const PTLOCINFO: u32 = 0x011c_23f8;
const GLOBAL_LOCALE_STATUS: u32 = 0x011c_2310;
/// `_environ` (`0126fdc8`), `_wenviron` (`0126fdd0`) and `__env_initialized` (`01271a84`).
const ENVIRON: u32 = 0x0126_fdc8;
const WENVIRON: u32 = 0x0126_fdd0;
const ENV_INITIALIZED: u32 = 0x0127_1a84;

fn watson(e: &mut Engine) {
    e.call(INVOKE_WATSON, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
}

fn allmul(e: &mut Engine, value: i64, by: u32) -> i64 {
    e.call(ALLMUL, &args![value as u32, (value >> 32) as u32, by, 0u32])
        .u64() as i64
}

/// The leap-year test the exe repeats inline: `year % 4 == 0 && year % 100 != 0`, or
/// `(year + 1900) % 400 == 0` (years since 1900, signed remainders).
fn leap(year: i32) -> bool {
    (year.wrapping_rem(4) == 0 && year.wrapping_rem(100) != 0)
        || year.wrapping_add(0x76c).wrapping_rem(400) == 0
}

fn table(e: &Engine, base: u32, month: u32) -> i32 {
    e.mem.u32(base.wrapping_add(month.wrapping_mul(4))) as i32
}

// Translated from 00ed7242 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__loctotime64_t` (LIBCMT/dtoxtm64.c, engine map), cdecl: (year, month, day, hour, minute,
/// second, dst flag): the local time as seconds since 1970 UTC. A year outside 1970..3000, a
/// month outside 1..12, an hour above 23, a minute or second above 59, a day below 1, or a day
/// past the month's (29 February of a leap year allowed) gives `errno` `EINVAL` and -1. The
/// days come from the year (365 a year and the leap days, `__allmul`) and the day of the year
/// (one more past February in a leap year), then hours, minutes and seconds; after `__tzset`
/// the `_timezone` is added, and the `_dstbias` when the flag is 1, or -1 with `_daylight` set
/// and `_isindst` true for the time. A failing `_get_` accessor calls `_invoke_watson`.
#[allow(clippy::too_many_arguments)]
pub fn loctotime64(
    e: &mut Engine,
    year: i32,
    month: i32,
    day: i32,
    hour: u32,
    minute: u32,
    second: u32,
    dst: i32,
) -> u64 {
    let y = year.wrapping_sub(0x76c);
    let invalid = |e: &mut Engine| {
        set_errno(e, 0x16);
        u64::MAX
    };
    if !(0x46..=0x44c).contains(&y)
        || (month.wrapping_sub(1) as u32) > 0xb
        || hour > 0x17
        || minute > 0x3b
        || second > 0x3b
        || day < 1
    {
        return invalid(e);
    }
    let before = table(e, DAYS, month as u32);
    let after = table(e, DAYS, (month + 1) as u32);
    if after.wrapping_sub(before) < day && (!leap(y) || month != 2 || day > 0x1d) {
        return invalid(e);
    }
    let mut yday = before.wrapping_add(day);
    if leap(y) && month > 2 {
        yday = yday.wrapping_add(1);
    }
    let previous = y.wrapping_sub(1);
    let leap_days = (y.wrapping_add(0x12b) / 400)
        .wrapping_sub(previous / 100)
        .wrapping_add((previous.wrapping_add((previous >> 31) & 3)) >> 2)
        .wrapping_sub(0x11);
    let mut total = i64::from(leap_days).wrapping_add(allmul(e, i64::from(y) - 0x46, 0x16d));
    total = total.wrapping_add(i64::from(yday));
    total = allmul(e, total, 0x18).wrapping_add(i64::from(hour as i32));
    total = allmul(e, total, 0x3c).wrapping_add(i64::from(minute as i32));
    total = allmul(e, total, 0x3c).wrapping_add(i64::from(second as i32));
    e.call(TZSET, &args![]);
    // The frame below EBP-0x34: the `tm` (+0: seconds, minutes, hours, day, month, year,
    // weekday, day of the year), `_dstbias` (+0x28), `_daylight` (+0x2C), `_timezone` (+0x30).
    e.with_stack(0x34, |e, frame| {
        let tm = frame.addr();
        let dstbias = tm + 0x28;
        let daylight = tm + 0x2c;
        let timezone = tm + 0x30;
        for slot in [daylight, dstbias, timezone] {
            e.mem.set_u32(slot, 0);
        }
        if e.call(GET_DAYLIGHT, &args![daylight]).u32() != 0 {
            watson(e);
        }
        if e.call(GET_DSTBIAS, &args![dstbias]).u32() != 0 {
            watson(e);
        }
        if e.call(GET_TIMEZONE, &args![timezone]).u32() != 0 {
            watson(e);
        }
        total = total.wrapping_add(i64::from(e.mem.u32(timezone) as i32));
        e.mem.set_u32(tm + 0x1c, yday as u32);
        e.mem.set_u32(tm + 0x10, month.wrapping_sub(1) as u32);
        e.mem.set_u32(tm + 8, hour);
        e.mem.set_u32(tm + 4, minute);
        e.mem.set_u32(tm + 0x14, y as u32);
        e.mem.set_u32(tm, second);
        let add_bias = dst == 1
            || (dst == -1 && e.mem.u32(daylight) != 0 && e.call(ISINDST, &args![tm]).u32() != 0);
        if add_bias {
            total = total.wrapping_add(i64::from(e.mem.u32(dstbias) as i32));
        }
        total as u64
    })
}

// Translated from 00ee2beb (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee2beb - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated; its __finally block (00ee2c31, _unlock(6)) runs inline.
/// `__tzset` (LIBCMT/tzset.c, engine map), cdecl: once (`012708d4`), under lock 6,
/// `__tzset_nolock`.
pub fn tzset(e: &mut Engine) {
    if e.mem.u32(TZSET_DONE) != 0 {
        return;
    }
    e.call(LOCK, &args![6u32]);
    if e.mem.u32(TZSET_DONE) == 0 {
        e.call(TZSET_NOLOCK, &args![]);
        let done = e.mem.u32(TZSET_DONE).wrapping_add(1);
        e.mem.set_u32(TZSET_DONE, done);
    }
    e.call(UNLOCK, &args![6u32]);
}

/// One of the `_get_timezone`/`_get_daylight`/`_get_dstbias` accessors: a null out gives
/// `errno` `EINVAL`, the invalid-parameter handler and `EINVAL`; else the word and 0.
fn get_word(e: &mut Engine, out: Ptr, word: u32) -> u32 {
    if out.addr() == 0 {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return 0x16;
    }
    let value = e.mem.u32(word);
    e.mem.set_u32(out.addr(), value);
    0
}

// Translated from 00ee2c7b (disassembled, FalloutNV.exe 1.4.0.525)
/// `_get_daylight` (LIBCMT/timeset.c, engine map), cdecl: `_daylight` (`011c2b84`) to `out`.
pub fn get_daylight(e: &mut Engine, out: Ptr) -> u32 {
    get_word(e, out, DAYLIGHT)
}

// Translated from 00ee2cb4 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_get_dstbias` (LIBCMT/timeset.c, engine map), cdecl: `_dstbias` (`011c2b88`) to `out`.
pub fn get_dstbias(e: &mut Engine, out: Ptr) -> u32 {
    get_word(e, out, DSTBIAS)
}

// Translated from 00ee2ced (disassembled, FalloutNV.exe 1.4.0.525)
/// `_get_timezone` (LIBCMT/timeset.c, engine map), cdecl: `_timezone` (`011c2b80`) to `out`.
pub fn get_timezone(e: &mut Engine, out: Ptr) -> u32 {
    get_word(e, out, TIMEZONE)
}

// Translated from 00ee2d32 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__timezone` (engine map), no arguments: the address of `_timezone` (`011c2b80`).
pub fn timezone_address(_e: &mut Engine) -> u32 {
    TIMEZONE
}

// Translated from 00ee2d26 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__daylight` (engine map), no arguments: the address of `_daylight` (`011c2b84`).
pub fn daylight_address(_e: &mut Engine) -> u32 {
    DAYLIGHT
}

// Translated from 00ee2d2c (disassembled, FalloutNV.exe 1.4.0.525)
/// `__dstbias` (engine map), no arguments: the address of `_dstbias` (`011c2b88`).
pub fn dstbias_address(_e: &mut Engine) -> u32 {
    DSTBIAS
}

// Translated from 00ee2d38 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__tzname` (engine map), no arguments: the address of `_tzname` (`011c2c10`).
pub fn tzname_address(_e: &mut Engine) -> u32 {
    TZNAME
}

// Translated from 00ee24c5 (disassembled, FalloutNV.exe 1.4.0.525)
/// Stores `value` through `__timezone` (`00ee2d32`).
pub fn set_timezone(e: &mut Engine, value: u32) {
    let at = e.call(TIMEZONE_ADDRESS, &args![]).u32();
    e.mem.set_u32(at, value);
}

// Translated from 00ee24a3 (disassembled, FalloutNV.exe 1.4.0.525)
/// Stores `value` through `__daylight` (`00ee2d26`).
pub fn set_daylight(e: &mut Engine, value: u32) {
    let at = e.call(DAYLIGHT_ADDRESS, &args![]).u32();
    e.mem.set_u32(at, value);
}

// Translated from 00ee24b4 (disassembled, FalloutNV.exe 1.4.0.525)
/// Stores `value` through `__dstbias` (`00ee2d2c`).
pub fn set_dstbias(e: &mut Engine, value: u32) {
    let at = e.call(DSTBIAS_ADDRESS, &args![]).u32();
    e.mem.set_u32(at, value);
}

// Translated from 00ed0c93 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___lc_codepage_func` (LIBCMT/initctyp.c, engine map), no arguments: the thread's locale
/// (`_getptd` +0x6C, refreshed by `__updatetlocinfo` when it is not the global one and the
/// thread has no own locale by `__globallocalestatus`) and its `lc_codepage` (+4).
pub fn lc_codepage_func(e: &mut Engine) -> u32 {
    let ptd = e.call(GETPTD, &args![]).u32();
    let mut info = e.mem.u32(ptd + 0x6c);
    if info != e.mem.u32(PTLOCINFO) && e.mem.u32(ptd + 0x70) & e.mem.u32(GLOBAL_LOCALE_STATUS) == 0
    {
        info = e.call(UPDATETLOCINFO, &args![]).u32();
    }
    e.mem.u32(info + 4)
}

// Translated from 00ee5f3d (disassembled, FalloutNV.exe 1.4.0.525)
/// `_getenv_helper_nolock` (LIBCMT/getenv.c, engine map), cdecl: the name. Null before the
/// environment is set up (`__env_initialized` `01271a84`); without `_environ` it is made from
/// `_wenviron` (`__mbtow_environ`) when there is one, else null. A null name is null. Else the
/// value of the first entry longer than the name with `=` after it whose head matches the name
/// (`00ee63a9(entry, name, length)` 0), or null.
pub fn getenv_helper_nolock(e: &mut Engine, name: Ptr) -> u32 {
    if e.mem.u32(ENV_INITIALIZED) == 0 {
        return 0;
    }
    let mut entries = e.mem.u32(ENVIRON);
    if entries == 0 {
        if e.mem.u32(WENVIRON) == 0 {
            return 0;
        }
        if e.call(MBTOW_ENVIRON, &args![]).u32() != 0 {
            return 0;
        }
        entries = e.mem.u32(ENVIRON);
        if entries == 0 {
            return 0;
        }
    }
    if name.addr() == 0 {
        return 0;
    }
    let length = e.call(STRLEN, &args![name]).u32();
    loop {
        let entry = e.mem.u32(entries);
        if entry == 0 {
            return 0;
        }
        if e.call(STRLEN, &args![entry]).u32() > length
            && e.mem.u8(entry.wrapping_add(length)) == b'='
            && e.call(ENV_NAME_COMPARE, &args![entry, name, length]).u32() == 0
        {
            return entry.wrapping_add(length).wrapping_add(1);
        }
        entries = entries.wrapping_add(4);
    }
}

/// A time-zone name from the `TIME_ZONE_INFORMATION` (UTF-16 at `wide`) into `_tzname`'s buffer
/// `out` (0x3F bytes, `WideCharToMultiByte(code page, 0, wide, -1, out, 0x3f, 0, &used)`): its
/// last byte cleared when it converted without a default character, else the buffer emptied.
fn tz_name(e: &mut Engine, code_page: u32, wide: u32, out: u32, used: u32) {
    let converted = e
        .call(
            IMPORT_WIDE_CHAR_TO_MULTI_BYTE,
            &args![
                code_page,
                0u32,
                wide,
                0xffff_ffffu32,
                out,
                0x3fu32,
                0u32,
                used
            ],
        )
        .u32();
    if converted != 0 && e.mem.u32(used) == 0 {
        e.mem.set_u8(out + 0x3f, 0);
    } else {
        e.mem.set_u8(out, 0);
    }
}

/// `atol` of the text at `at` and `at` moved past the digits after it: `+` and digits (the
/// hours, `plus`), or digits only.
fn digits_end(e: &Engine, mut at: u32, plus: bool) -> u32 {
    loop {
        let c = e.mem.u8(at) as i8;
        if (plus && c == b'+' as i8) || (b'0' as i8..=b'9' as i8).contains(&c) {
            at = at.wrapping_add(1);
        } else {
            return at;
        }
    }
}

// Translated from 00ee24d6 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee24d6 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated; its __finally block (00ee2782, _unlock(7)) runs inline.
/// `__tzset_nolock` (LIBCMT/tzset.c, engine map), cdecl, no arguments. Under lock 7, from the
/// current `_timezone`, `_daylight` and `_dstbias` (their `_get_` accessors; `_invoke_watson`
/// on failure) and the code page (`___lc_codepage_func`): the Windows flag and both
/// `cvtdate` years are reset. Without a `TZ` value (`_getenv_helper_nolock`) the copy of the
/// last one is freed and `GetTimeZoneInformation` fills `01270820`; unless that fails (-1)
/// the flag is set, `_timezone` is the bias in seconds (plus the standard bias when the
/// standard date is given), `_daylight` and `_dstbias` (the daylight bias less the standard,
/// in seconds) set when a daylight date and bias are given, and both `_tzname` names converted
/// (`WideCharToMultiByte`). A `TZ` value equal to the last copy changes nothing more; a new
/// one is copied (`__malloc_crt`, `strcpy_s`) and parsed after the lock is left: three name
/// characters (`strncpy_s`), a signed `hh[:mm[:ss]]` offset (`atol`) into `_timezone`, and
/// the daylight name's first character as `_daylight` (its three characters into the second
/// name, else that name emptied). The three values are stored with their `_set_` helpers
/// before the parse.
pub fn tzset_nolock(e: &mut Engine) {
    // The frame below EBP-0x3C: the TZ value (+0), its negative flag (+4), the code page (+8),
    // the default-character flag (+0xC), the done flag (+0x10), the dstbias (+0x14), the
    // daylight (+0x18), the tzname address (+0x1C) and the timezone (+0x20).
    e.with_stack(0x24, |e, frame| {
        let base = frame.addr();
        let used = base + 0xc;
        let dstbias = base + 0x14;
        let daylight = base + 0x18;
        let timezone = base + 0x20;
        for slot in [timezone, daylight, dstbias] {
            e.mem.set_u32(slot, 0);
        }
        let mut negative = false;
        let mut done = false;
        e.call(LOCK, &args![7u32]);
        let names = e.call(TZNAME_ADDRESS, &args![]).u32();
        if e.call(GET_TIMEZONE, &args![timezone]).u32() != 0 {
            watson(e);
        }
        if e.call(GET_DAYLIGHT, &args![daylight]).u32() != 0 {
            watson(e);
        }
        if e.call(GET_DSTBIAS, &args![dstbias]).u32() != 0 {
            watson(e);
        }
        let code_page = e.call(LC_CODEPAGE_FUNC, &args![]).u32();
        e.mem.set_u32(TZ_FROM_WINDOWS, 0);
        e.mem.set_u32(DST_END_YEAR, 0xffff_ffff);
        e.mem.set_u32(DST_START_YEAR, 0xffff_ffff);
        let value = e.call(GETENV_HELPER_NOLOCK, &args![TEXT_TZ]).u32();
        let from_windows = value == 0 || e.mem.u8(value) == 0;
        if !from_windows {
            let last = e.mem.u32(LAST_TZ);
            let same = last != 0 && e.call(STRCMP, &args![value, last]).u32() == 0;
            if same {
                done = true;
            } else {
                let last = e.mem.u32(LAST_TZ);
                if last != 0 {
                    e.call(FREE, &args![last]);
                }
                let size = e.call(STRLEN, &args![value]).u32().wrapping_add(1);
                let copy = e.call(MALLOC_CRT, &args![size]).u32();
                e.mem.set_u32(LAST_TZ, copy);
                if copy == 0 {
                    done = true;
                } else {
                    let size = e.call(STRLEN, &args![value]).u32().wrapping_add(1);
                    let copy = e.mem.u32(LAST_TZ);
                    if e.call(STRCPY_S, &args![copy, size, value]).u32() != 0 {
                        watson(e);
                    }
                }
            }
        }
        if from_windows {
            let last = e.mem.u32(LAST_TZ);
            if last != 0 {
                e.call(FREE, &args![last]);
                e.mem.set_u32(LAST_TZ, 0);
            }
            if e.call(IMPORT_GET_TIME_ZONE_INFORMATION, &args![TZ_INFO])
                .u32()
                != 0xffff_ffff
            {
                e.mem.set_u32(TZ_FROM_WINDOWS, 1);
                let mut zone = (e.mem.u32(TZ_INFO) as i32).wrapping_mul(0x3c);
                if e.mem.u16(TZ_INFO + 0x46) != 0 {
                    zone = zone.wrapping_add((e.mem.u32(TZ_INFO + 0x54) as i32).wrapping_mul(0x3c));
                }
                e.mem.set_u32(timezone, zone as u32);
                let daylight_bias = e.mem.u32(TZ_INFO + 0xa8) as i32;
                if e.mem.u16(TZ_INFO + 0x9a) != 0 && daylight_bias != 0 {
                    e.mem.set_u32(daylight, 1);
                    let standard_bias = e.mem.u32(TZ_INFO + 0x54) as i32;
                    let bias = daylight_bias.wrapping_sub(standard_bias).wrapping_mul(0x3c);
                    e.mem.set_u32(dstbias, bias as u32);
                } else {
                    e.mem.set_u32(daylight, 0);
                    e.mem.set_u32(dstbias, 0);
                }
                let standard = e.mem.u32(names);
                tz_name(e, code_page, TZ_INFO + 4, standard, used);
                let daylight_name = e.mem.u32(names + 4);
                tz_name(e, code_page, TZ_INFO + 0x58, daylight_name, used);
            }
            done = true;
        }
        let zone = e.mem.u32(timezone);
        e.call(SET_TIMEZONE, &args![zone]);
        let flag = e.mem.u32(daylight);
        e.call(SET_DAYLIGHT, &args![flag]);
        let bias = e.mem.u32(dstbias);
        e.call(SET_DSTBIAS, &args![bias]);
        e.call(UNLOCK, &args![7u32]);
        if done {
            return;
        }
        let standard = e.mem.u32(names);
        if e.call(STRNCPY_S, &args![standard, 0x40u32, value, 3u32])
            .u32()
            != 0
        {
            watson(e);
        }
        let mut at = value.wrapping_add(3);
        if e.mem.u8(at) == b'-' {
            negative = true;
            at = at.wrapping_add(1);
        }
        let mut zone = (e.call(ATOL, &args![at]).u32() as i32).wrapping_mul(0xe10);
        at = digits_end(e, at, true);
        if e.mem.u8(at) == b':' {
            at = at.wrapping_add(1);
            zone = zone.wrapping_add((e.call(ATOL, &args![at]).u32() as i32).wrapping_mul(0x3c));
            at = digits_end(e, at, false);
            if e.mem.u8(at) == b':' {
                at = at.wrapping_add(1);
                zone = zone.wrapping_add(e.call(ATOL, &args![at]).u32() as i32);
                at = digits_end(e, at, false);
            }
        }
        if negative {
            zone = zone.wrapping_neg();
        }
        let flag = e.mem.u8(at) as i8 as i32;
        if flag != 0 {
            let daylight_name = e.mem.u32(names + 4);
            if e.call(STRNCPY_S, &args![daylight_name, 0x40u32, at, 3u32])
                .u32()
                != 0
            {
                watson(e);
            }
        } else {
            let daylight_name = e.mem.u32(names + 4);
            e.mem.set_u8(daylight_name, 0);
        }
        let at = e.call(TIMEZONE_ADDRESS, &args![]).u32();
        e.mem.set_u32(at, zone as u32);
        let at = e.call(DAYLIGHT_ADDRESS, &args![]).u32();
        e.mem.set_u32(at, flag as u32);
    });
}

// Translated from 00ee2c3a (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee2c3a - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) is not translated; its __finally block (00ee2c72, _unlock(6)) runs inline.
/// `_isindst` (LIBCMT/tzset.c, engine map), cdecl: the `tm`: `_isindst_nolock` under lock 6.
pub fn isindst(e: &mut Engine, tm: Ptr) -> u32 {
    e.call(LOCK, &args![6u32]);
    let result = isindst_nolock(e, tm);
    e.call(UNLOCK, &args![6u32]);
    result
}

/// `cvtdate`'s call for one end of daylight time from a `SYSTEMTIME` of the
/// `TIME_ZONE_INFORMATION` at `date` (`trans` 1 for the start, 0 for the end): with a zero
/// `wYear` the "week `wDay` of `wDayOfWeek`" form, else the absolute `wDay`.
fn convert_zone_date(e: &mut Engine, trans: u32, year: u32, date: u32) {
    let word = |e: &Engine, offset: u32| u32::from(e.mem.u16(date + offset));
    let (month, hour) = (word(e, 2), word(e, 8));
    let (minute, second, ms) = (word(e, 0xa), word(e, 0xc), word(e, 0xe));
    if word(e, 0) == 0 {
        let (week, day_of_week) = (word(e, 6), word(e, 4));
        cvtdate(
            e,
            month,
            hour,
            trans,
            1,
            year,
            week,
            day_of_week,
            0,
            minute,
            second,
            ms,
        );
    } else {
        let day = word(e, 6);
        cvtdate(
            e, month, hour, trans, 0, year, 0, 0, day, minute, second, ms,
        );
    }
}

// Translated from 00ee2a13 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_isindst_nolock` (LIBCMT/tzset.c, engine map); its `tm` comes in `EDI` (the first argument
/// word here). False without `_daylight`. Unless the start and end were worked out for the
/// `tm`'s year, they are (`cvtdate`): from Windows' daylight and standard dates when the zone
/// came from there, else the US rules (from 2007: second Sunday of March to first Sunday of
/// November; before: first Sunday of April to last Sunday of October; at 2:00). Then the day
/// of the year (+0x1C) against the start and end days (either order: daylight time across the
/// new year), and on a start or end day the time of day in milliseconds against theirs.
pub fn isindst_nolock(e: &mut Engine, tm: Ptr) -> u32 {
    let tm = tm.addr();
    let flag = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        if e.call(GET_DAYLIGHT, &args![slot]).u32() != 0 {
            watson(e);
        }
        e.mem.u32(slot.addr())
    });
    if flag == 0 {
        return 0;
    }
    let year = e.mem.u32(tm + 0x14);
    if year != e.mem.u32(DST_START_YEAR) || year != e.mem.u32(DST_END_YEAR) {
        if e.mem.u32(TZ_FROM_WINDOWS) != 0 {
            convert_zone_date(e, 1, year, TZ_INFO + 0x98);
            let year = e.mem.u32(tm + 0x14);
            convert_zone_date(e, 0, year, TZ_INFO + 0x44);
        } else {
            let (start_month, start_week, end_month, end_week) = if (year as i32) < 0x6b {
                (4, 1, 0xa, 5)
            } else {
                (3, 2, 0xb, 1)
            };
            cvtdate(e, start_month, 2, 1, 1, year, start_week, 0, 0, 0, 0, 0);
            let year = e.mem.u32(tm + 0x14);
            cvtdate(e, end_month, 2, 0, 1, year, end_week, 0, 0, 0, 0, 0);
        }
    }
    let start = e.mem.u32(DST_START_DAY) as i32;
    let end = e.mem.u32(DST_END_DAY) as i32;
    let day = e.mem.u32(tm + 0x1c) as i32;
    if start < end {
        if day < start || day > end {
            return 0;
        }
        if day > start && day < end {
            return 1;
        }
    } else {
        if day < end || day > start {
            return 1;
        }
        if day > end && day < start {
            return 0;
        }
    }
    let hour = e.mem.u32(tm + 8);
    let minute = e.mem.u32(tm + 4);
    let second = e.mem.u32(tm);
    let ms = hour
        .wrapping_mul(0x3c)
        .wrapping_add(minute)
        .wrapping_mul(0x3c)
        .wrapping_add(second)
        .wrapping_mul(0x3e8) as i32;
    if day == start {
        u32::from(ms >= e.mem.u32(DST_START_MS) as i32)
    } else {
        u32::from(ms < e.mem.u32(DST_END_MS) as i32)
    }
}

// Translated from 00ee281c (disassembled, FalloutNV.exe 1.4.0.525)
/// `cvtdate` (LIBCMT/tzset.c, engine map); the month comes in `EAX` and the hour in `ECX` (the
/// first two argument words here), then nine stack words: (start 1 / end 0, form, year since
/// 1900, week, day of week, day, minute, second, milliseconds). Form 1 is "week `week` (5:
/// the last) of the day of week in the month", from the weekday of the month's first day
/// (365 days a year and the leap days, 1 January 1900 a Monday); form 0 the absolute day.
/// The day of the year and the time of day in milliseconds go to the start (`011c2b64..6c`,
/// with the year) or, with `_dstbias` added (`_get_dstbias`, `_invoke_watson` on failure)
/// and carried across midnight, to the end (`011c2b70..78`).
#[allow(clippy::too_many_arguments)]
pub fn cvtdate(
    e: &mut Engine,
    month: u32,
    hour: u32,
    trans: u32,
    form: u32,
    year: u32,
    week: u32,
    day_of_week: u32,
    day: u32,
    minute: u32,
    second: u32,
    ms: u32,
) {
    let y = year as i32;
    let mut yday: i32;
    if form == 1 {
        yday = if leap(y) {
            table(e, LEAP_DAYS, month)
        } else {
            table(e, DAYS, month)
        };
        yday = yday.wrapping_add(1);
        let previous = y.wrapping_sub(1);
        let leap_days = (y.wrapping_add(0x12b) / 400).wrapping_sub(previous / 100);
        let quarter = previous.wrapping_add((previous >> 31) & 3) >> 2;
        let days = y
            .wrapping_mul(0x16d)
            .wrapping_add(quarter.wrapping_add(yday).wrapping_add(leap_days))
            .wrapping_sub(0x63db);
        let weekday = days.wrapping_rem(7);
        let offset = (week as i32)
            .wrapping_mul(7)
            .wrapping_sub(weekday)
            .wrapping_add(day_of_week as i32);
        yday = if weekday > day_of_week as i32 {
            yday.wrapping_add(offset)
        } else {
            yday.wrapping_add(offset).wrapping_sub(7)
        };
        if week == 5 {
            let last = if leap(y) {
                table(e, LEAP_DAYS + 4, month)
            } else {
                table(e, DAYS + 4, month)
            };
            if yday > last {
                yday = yday.wrapping_sub(7);
            }
        }
    } else {
        yday = if leap(y) {
            table(e, LEAP_DAYS, month)
        } else {
            table(e, DAYS, month)
        };
        yday = yday.wrapping_add(day as i32);
    }
    let time = hour
        .wrapping_mul(0x3c)
        .wrapping_add(minute)
        .wrapping_mul(0x3c)
        .wrapping_add(second)
        .wrapping_mul(0x3e8)
        .wrapping_add(ms);
    if trans == 1 {
        e.mem.set_u32(DST_START_DAY, yday as u32);
        e.mem.set_u32(DST_START_MS, time);
        e.mem.set_u32(DST_START_YEAR, year);
        return;
    }
    e.mem.set_u32(DST_END_DAY, yday as u32);
    e.mem.set_u32(DST_END_MS, time);
    let bias = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        if e.call(GET_DSTBIAS, &args![slot]).u32() != 0 {
            watson(e);
        }
        e.mem.u32(slot.addr())
    });
    let end = (e.mem.u32(DST_END_MS) as i32).wrapping_add((bias as i32).wrapping_mul(0x3e8));
    e.mem.set_u32(DST_END_MS, end as u32);
    if end < 0 {
        e.mem
            .set_u32(DST_END_MS, end.wrapping_add(0x0526_5c00) as u32);
        let d = e.mem.u32(DST_END_DAY).wrapping_sub(1);
        e.mem.set_u32(DST_END_DAY, d);
    } else if end >= 0x0526_5c00 {
        e.mem
            .set_u32(DST_END_MS, end.wrapping_sub(0x0526_5c00) as u32);
        let d = e.mem.u32(DST_END_DAY).wrapping_add(1);
        e.mem.set_u32(DST_END_DAY, d);
    }
    e.mem.set_u32(DST_END_YEAR, year);
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ed_7242, loctotime64(i32, i32, i32, u32, u32, u32, i32) -> u64),
        entry!(0x00ee_2beb, tzset()),
        entry!(0x00ee_24d6, tzset_nolock()),
        entry!(0x00ee_2c3a, isindst(Ptr) -> u32),
        entry!(0x00ee_2a13, isindst_nolock(Ptr) -> u32),
        entry!(
            0x00ee_281c,
            cvtdate(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00ee_2c7b, get_daylight(Ptr) -> u32),
        entry!(0x00ee_2cb4, get_dstbias(Ptr) -> u32),
        entry!(0x00ee_2ced, get_timezone(Ptr) -> u32),
        entry!(0x00ee_2d32, timezone_address() -> u32),
        entry!(0x00ee_2d26, daylight_address() -> u32),
        entry!(0x00ee_2d2c, dstbias_address() -> u32),
        entry!(0x00ee_2d38, tzname_address() -> u32),
        entry!(0x00ee_24c5, set_timezone(u32)),
        entry!(0x00ee_24a3, set_daylight(u32)),
        entry!(0x00ee_24b4, set_dstbias(u32)),
        entry!(0x00ed_0c93, lc_codepage_func() -> u32),
        entry!(0x00ee_5f3d, getenv_helper_nolock(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine with the CRT's data pages mapped, the day tables (from the month lengths), the
    /// zone already set (`__tzset` done) to `zone` seconds with `daylight` and a bias of -3600,
    /// and the locks doubled.
    fn engine(zone: i32, daylight: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_2000, 0x1000);
        e.map(0x0127_0000, 0x1000);
        let lengths = [31u32, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        for (base, extra) in [(DAYS, 0u32), (LEAP_DAYS, 1)] {
            let mut before = 0xffff_ffffu32;
            for month in 1..=13u32 {
                e.mem.set_u32(base + 4 * month, before);
                if month <= 12 {
                    let add = lengths[month as usize - 1] + if month == 2 { extra } else { 0 };
                    before = before.wrapping_add(add);
                }
            }
        }
        e.mem.set_u32(TZSET_DONE, 1);
        e.mem.set_u32(TIMEZONE, zone as u32);
        e.mem.set_u32(DAYLIGHT, daylight);
        e.mem.set_u32(DSTBIAS, (-3600i32) as u32);
        e.register(LOCK, |_, _| Ret::default());
        e.register(UNLOCK, |_, _| Ret::default());
        e.map(0x0126_f000, 0x1000);
        let block = e.mem.alloc(0x80);
        e.register_double(GETPTD_NOEXIT, move |_, _| block.into_ret());
        e.register(DECODE_POINTER, |_, _| 0x0e30_0000u32.into_ret());
        e.register(0x0e30_0000, |_, _| Ret::default());
        e
    }

    fn time(e: &mut Engine, date: [u32; 6], dst: i32) -> u64 {
        e.call(
            0x00ed_7242,
            &args![date[0], date[1], date[2], date[3], date[4], date[5], dst as u32],
        )
        .u64()
    }

    #[test]
    fn local_times_become_seconds_since_1970() {
        let mut e = engine(0, 0);
        assert_eq!(time(&mut e, [1970, 1, 1, 0, 0, 0], -1), 0);
        assert_eq!(time(&mut e, [2020, 1, 1, 0, 0, 0], -1), 1_577_836_800);
        assert_eq!(time(&mut e, [2020, 3, 1, 12, 30, 15], -1), 1_583_065_815);
        assert_eq!(time(&mut e, [2024, 2, 29, 0, 0, 0], -1), 1_709_164_800);
        // 29 February outside a leap year, a month 13, a minute 60: EINVAL and -1.
        assert_eq!(time(&mut e, [2023, 2, 29, 0, 0, 0], -1), u64::MAX);
        assert_eq!(time(&mut e, [2023, 13, 1, 0, 0, 0], -1), u64::MAX);
        assert_eq!(time(&mut e, [2023, 1, 1, 0, 60, 0], -1), u64::MAX);
        // The zone is added; the bias only with the flag 1.
        let mut e = engine(18000, 0);
        assert_eq!(time(&mut e, [2020, 1, 1, 0, 0, 0], -1), 1_577_854_800);
        assert_eq!(time(&mut e, [2020, 1, 1, 0, 0, 0], 1), 1_577_851_200);
    }

    #[test]
    fn daylight_time_follows_the_us_rules_without_windows_dates() {
        // EST with daylight time, the zone from TZ (not Windows): from 2007 the second Sunday of
        // March to the first Sunday of November.
        let mut e = engine(18000, 1);
        // 2020-07-01 12:00 is in daylight time: one hour less.
        assert_eq!(time(&mut e, [2020, 7, 1, 12, 0, 0], -1), 1_593_619_200);
        assert_eq!(e.mem.u32(DST_START_YEAR), 120);
        // 2020-03-08 is the second Sunday of March (day 67 of the year from 0).
        assert_eq!(e.mem.u32(DST_START_DAY), 67);
        assert_eq!(e.mem.u32(DST_START_MS), 2 * 3600 * 1000);
        // 2020-11-01 is the first Sunday of November (day 305); the end moves by the bias.
        assert_eq!(e.mem.u32(DST_END_DAY), 305);
        assert_eq!(e.mem.u32(DST_END_MS), 3600 * 1000);
        // January is not.
        assert_eq!(time(&mut e, [2020, 1, 15, 12, 0, 0], -1), 1_579_107_600);
    }

    #[test]
    fn the_accessors_refuse_a_null_out() {
        let mut e = engine(0, 0);
        let block = e.call(GETPTD_NOEXIT, &args![]).u32();
        assert_eq!(e.call(GET_DSTBIAS, &args![0u32]).u32(), 0x16);
        assert_eq!(e.mem.u32(block + 8), 0x16);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(GET_TIMEZONE, &args![out]).u32(), 0);
        assert_eq!(e.call(TZNAME_ADDRESS, &args![]).u32(), TZNAME);
    }
}

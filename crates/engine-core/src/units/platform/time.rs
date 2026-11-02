//! The platform's clock and time-zone calls: `GetSystemTimeAsFileTime` (the real system clock,
//! read when the game asks), `GetTimeZoneInformation` and `FileTimeToLocalFileTime` (the
//! player's time zone from the machine table, `scripts/d3d9-capture`, with the bias in effect
//! now worked out by [`zone`]), `FileTimeToSystemTime`, and `WideCharToMultiByte` for the
//! ASCII text the CRT converts (the time-zone names).
//!
//! Measured on this machine (2026-10-21, `build43-scratch/measure/time-2026-10-21.txt`, a
//! program calling the real functions; zone US Eastern, daylight time in effect):
//! - `GetTimeZoneInformation` answered `TIME_ZONE_ID_DAYLIGHT` (2) and the zone's rules; the
//!   rules ([`zone::bias_at`]) gave the same local time as `SystemTimeToTzSpecificLocalTime`
//!   for every half hour of 2025-2027 (52,560 times).
//! - `FileTimeToLocalFileTime` subtracts the bias in effect *now* (not at the given time) from
//!   any `FILETIME`, wrapping around 2^64 (0, 1, 0x8000_0000_0000_0000 and u64::MAX included),
//!   and always answers TRUE.
//! - `FileTimeToSystemTime` converts every value up to 0x7FFF_FFFF_FFFF_FFFF (30828-09-14
//!   02:48:05.477); above it answers FALSE with `ERROR_INVALID_PARAMETER` (87) and leaves the
//!   output as it was. Equal to [`zone::system_time`] for every half hour of 2025-2027.
//! - `WideCharToMultiByte(CP_ACP (1252 here), 0, text, -1, out, size, NULL, &used)` copies
//!   ASCII as is and counts the zero; `used` becomes FALSE; with `size` 0 it answers the size
//!   needed and writes nothing; too small a buffer is filled as far as it goes and the call
//!   answers 0 with `ERROR_INSUFFICIENT_BUFFER` (122). `é` became 0xE9 and a character outside
//!   code page 1252 `?` with `used` TRUE: non-ASCII text is not provided here (the code page's
//!   table is not captured).

use super::zone::{self, ZoneInfo};
use crate::{Engine, Ptr};

/// The real system clock as a `FILETIME` (100 ns since 1601-01-01 UTC).
fn now_filetime() -> u64 {
    let since = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("the system clock is before 1970");
    zone::UNIX_EPOCH_TICKS
        + since.as_secs() * zone::TICKS_PER_SECOND
        + u64::from(since.subsec_nanos() / 100)
}

fn zone_info(e: &mut Engine) -> ZoneInfo {
    ZoneInfo::from_bytes(&super::d3d9::time_zone(e))
}

// Platform: replaces KERNEL32.DLL GetSystemTimeAsFileTime (import)
/// `GetSystemTimeAsFileTime(out)`: the real system clock now (UTC).
pub fn get_system_time_as_file_time(e: &mut Engine, out: Ptr) {
    let now = now_filetime();
    e.mem.set_u32(out.addr(), now as u32);
    e.mem.set_u32(out.addr() + 4, (now >> 32) as u32);
}

// Platform: replaces KERNEL32.DLL GetTimeZoneInformation (import)
/// `GetTimeZoneInformation(out)`: the player's `TIME_ZONE_INFORMATION` (172 bytes, from the
/// machine table) and `TIME_ZONE_ID_UNKNOWN` (0) for a zone without daylight time, else
/// `TIME_ZONE_ID_DAYLIGHT` (2) or `TIME_ZONE_ID_STANDARD` (1) as the real clock now falls.
pub fn get_time_zone_information(e: &mut Engine, out: Ptr) -> u32 {
    let bytes = super::d3d9::time_zone(e);
    e.mem.write(out.addr(), &bytes);
    let zone = ZoneInfo::from_bytes(&bytes);
    match zone::in_daylight(&zone, zone::filetime_seconds(now_filetime())) {
        None => 0,
        Some(true) => 2,
        Some(false) => 1,
    }
}

// Platform: replaces KERNEL32.DLL FileTimeToLocalFileTime (import)
/// `FileTimeToLocalFileTime(utc, local)`: `utc` less the player's bias in effect now (minutes,
/// 600,000,000 ticks each), wrapping; TRUE (measured, module comment).
pub fn file_time_to_local_file_time(e: &mut Engine, utc: Ptr, local: Ptr) -> u32 {
    let ticks = u64::from(e.mem.u32(utc.addr())) | (u64::from(e.mem.u32(utc.addr() + 4)) << 32);
    let zone = zone_info(e);
    let bias = zone::bias_at(&zone, zone::filetime_seconds(now_filetime()));
    let result = ticks.wrapping_sub((i64::from(bias) * 600_000_000) as u64);
    e.mem.set_u32(local.addr(), result as u32);
    e.mem.set_u32(local.addr() + 4, (result >> 32) as u32);
    1
}

// Platform: replaces KERNEL32.DLL GetLocalTime (import)
/// `GetLocalTime(out)`: the real clock now less the player's bias in effect now, as a
/// `SYSTEMTIME` (eight 16-bit words): `FileTimeToLocalFileTime` of `GetSystemTimeAsFileTime`,
/// then `FileTimeToSystemTime` (all three measured, module comment). The save system calls it
/// (`008511a1`, after the character is named).
pub fn get_local_time(e: &mut Engine, out: Ptr) {
    let zone = zone_info(e);
    let now = now_filetime();
    let bias = zone::bias_at(&zone, zone::filetime_seconds(now));
    let local = now.wrapping_sub((i64::from(bias) * 600_000_000) as u64);
    let words = zone::system_time(local).expect("the clock is before 30828");
    for (i, w) in words.iter().enumerate() {
        e.mem.set_u16(out.addr() + 2 * i as u32, *w);
    }
}

// Platform: replaces KERNEL32.DLL FileTimeToSystemTime (import)
/// `FileTimeToSystemTime(time, out)`: the `SYSTEMTIME` of a `FILETIME` ([`zone::system_time`]);
/// above 0x7FFF_FFFF_FFFF_FFFF FALSE with `ERROR_INVALID_PARAMETER` and `out` untouched
/// (measured).
pub fn file_time_to_system_time(e: &mut Engine, time: Ptr, out: Ptr) -> u32 {
    let ticks = u64::from(e.mem.u32(time.addr())) | (u64::from(e.mem.u32(time.addr() + 4)) << 32);
    match zone::system_time(ticks) {
        Some(words) => {
            for (i, w) in words.iter().enumerate() {
                e.mem.set_u16(out.addr() + 2 * i as u32, *w);
            }
            1
        }
        None => {
            e.kernel.last_error = 87;
            0
        }
    }
}

// Platform: replaces KERNEL32.DLL WideCharToMultiByte (import)
/// `WideCharToMultiByte(code page, flags, wide, count, out, size, default, used)` for ASCII
/// text in `CP_ACP` (0) or 1252 with no flags and no default character: each character as its
/// byte (`count` -1: through the zero, which is counted); `used` (when given) FALSE; `size` 0
/// answers the size needed; too small a buffer is filled as far as it goes and answers 0 with
/// `ERROR_INSUFFICIENT_BUFFER` (122). As measured (module comment). Other code pages, flags,
/// a default character or non-ASCII text are not provided (they stop the run).
#[allow(clippy::too_many_arguments)]
pub fn wide_char_to_multi_byte(
    e: &mut Engine,
    code_page: u32,
    flags: u32,
    wide: Ptr,
    count: i32,
    out: Ptr,
    size: i32,
    default: Ptr,
    used: Ptr,
) -> u32 {
    assert!(
        (code_page == 0 || code_page == 1252) && flags == 0 && default.addr() == 0,
        "WideCharToMultiByte(code page {code_page}, flags {flags:#x}, default {:#x}) is not provided",
        default.addr()
    );
    let mut text = Vec::new();
    let mut at = wide.addr();
    loop {
        if count >= 0 && text.len() as i32 >= count {
            break;
        }
        let c = e.mem.u16(at);
        assert!(
            c < 0x80,
            "WideCharToMultiByte: the non-ASCII character {c:#x} is not provided"
        );
        text.push(c as u8);
        at += 2;
        if count < 0 && c == 0 {
            break;
        }
    }
    if used.addr() != 0 {
        e.mem.set_u32(used.addr(), 0);
    }
    if size == 0 {
        return text.len() as u32;
    }
    let room = size.max(0) as usize;
    let written = text.len().min(room);
    e.mem.write(out.addr(), &text[..written]);
    if written < text.len() {
        e.kernel.last_error = 122;
        return 0;
    }
    text.len() as u32
}

/// The days of `month` in `year` (proleptic Gregorian).
fn month_days(year: u32, month: u32) -> u32 {
    match month {
        2 if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// The `FILETIME` of `SYSTEMTIME` words as `SystemTimeToTzSpecificLocalTime` accepts them (the
/// day of the week is not read); `None` for a time it refuses (measured: year 1601..=30827,
/// month 1..=12, a day of that month, hour < 24, minute < 60, second < 60, milliseconds < 1000).
pub fn system_time_ticks(w: &[u16; 8]) -> Option<u64> {
    let [year, month, _, day, hour, minute, second, ms] = w.map(u32::from);
    if !(1601..=30827).contains(&year)
        || !(1..=12).contains(&month)
        || day == 0
        || day > month_days(year, month)
        || hour >= 24
        || minute >= 60
        || second >= 60
        || ms >= 1000
    {
        return None;
    }
    let days = zone::days_from_civil(i64::from(year), month, day);
    let ms_total =
        (days * 86_400 + i64::from(hour * 3600 + minute * 60 + second)) * 1000 + i64::from(ms);
    Some((zone::UNIX_EPOCH_TICKS as i64 + ms_total * 10_000) as u64)
}

// Platform: replaces KERNEL32.DLL SystemTimeToTzSpecificLocalTime (import)
/// `SystemTimeToTzSpecificLocalTime(zone, utc, local)` with no zone given (NULL: the player's
/// own): the local time of `utc` by the zone's rules for that year (Windows keeps rules per year,
/// which can differ from the current ones: `super::d3d9::time_zone_for_year`, captured), with
/// the bias in effect at that moment; the day of the week is worked out. A time Windows refuses
/// ([`system_time_ticks`]), or one whose local time would fall before 1601-01-01, answers FALSE
/// with `ERROR_INVALID_PARAMETER` (87) and leaves `local` as it was; on success the last error
/// is left as it was.
///
/// Measured on this machine (2026-10-27, `build43-scratch/measure/platform5`; zone US
/// Eastern): the field checks above, a wrong day of the week ignored, 1601-01-01 00:00 UTC
/// refused and 05:00 accepted (bias 300), 30827-12-31 23:59:59.999 accepted; the daylight
/// changes it applied, scanned hour by hour in 15 years from 1601 to 30827, were those of
/// `GetTimeZoneInformationForYear` for each year (first Sunday of April to last Sunday of
/// October through 2006, second Sunday of March to first Sunday of November from 2007), not the
/// current rules for every year. Not provided (stop the run): a zone passed in; a local time
/// after 30827 (not measured: this zone is west of UTC); a time whose year by UTC and by local
/// standard time differ while those years' rules differ (which year's rules Windows takes then
/// is not measured).
pub fn system_time_to_tz_specific_local_time(
    e: &mut Engine,
    zone_info: Ptr,
    utc: Ptr,
    local: Ptr,
) -> u32 {
    assert!(
        zone_info.is_null(),
        "SystemTimeToTzSpecificLocalTime with a zone passed in is not provided"
    );
    let words: [u16; 8] = std::array::from_fn(|i| e.mem.u16(utc.addr() + 2 * i as u32));
    let Some(ticks) = system_time_ticks(&words) else {
        e.kernel.last_error = 87;
        return 0;
    };
    let utc_year = u32::from(words[0]);
    let rules_utc = ZoneInfo::from_bytes(&super::d3d9::time_zone_for_year(e, utc_year));
    let seconds = zone::filetime_seconds(ticks);
    // `zone::in_daylight` works the changes out in the year of the local standard time.
    let standard = seconds - 60 * i64::from(rules_utc.bias + rules_utc.standard_bias);
    let (standard_year, _, _) = zone::civil_from_days(standard.div_euclid(86_400));
    // Outside 1601..=30827 Windows keeps no rules; such a local time is refused or not provided
    // below whichever rules are taken.
    let rules = if standard_year as u32 == utc_year || !(1601..=30827).contains(&standard_year) {
        rules_utc
    } else {
        let other = ZoneInfo::from_bytes(&super::d3d9::time_zone_for_year(e, standard_year as u32));
        assert!(
            other == rules_utc,
            "SystemTimeToTzSpecificLocalTime: {utc_year} by UTC is {standard_year} locally and the two years' rules differ: not provided"
        );
        other
    };
    let bias = zone::bias_at(&rules, seconds);
    let local_ticks = ticks as i64 - i64::from(bias) * 600_000_000;
    if local_ticks < 0 {
        e.kernel.last_error = 87;
        return 0;
    }
    let out =
        zone::system_time(local_ticks as u64).expect("a local time within the FILETIME range");
    assert!(
        out[0] <= 30827,
        "SystemTimeToTzSpecificLocalTime: a local time after 30827 is not provided"
    );
    for (i, w) in out.iter().enumerate() {
        e.mem.set_u16(local.addr() + 2 * i as u32, *w);
    }
    1
}

#[cfg(test)]
mod tests_local_time {
    use super::*;

    /// US Eastern's per-year rules as the measurement found them: first Sunday of April to last
    /// Sunday of October through 2006, second Sunday of March to first Sunday of November after.
    fn eastern_years(e: &mut Engine) {
        let line = |first: u32, last: u32, std: [u16; 8], dst: [u16; 8]| {
            let mut b = vec![0u8; 172];
            b[0..4].copy_from_slice(&300i32.to_le_bytes());
            for (i, w) in std.iter().enumerate() {
                b[0x44 + 2 * i..0x46 + 2 * i].copy_from_slice(&w.to_le_bytes());
            }
            for (i, w) in dst.iter().enumerate() {
                b[0x98 + 2 * i..0x9a + 2 * i].copy_from_slice(&w.to_le_bytes());
            }
            b[0xa8..0xac].copy_from_slice(&(-60i32).to_le_bytes());
            let hex: String = b.iter().map(|x| format!("{x:02x}")).collect();
            format!("GetTimeZoneInformationForYear\t{first:x} {last:x}\t1\t{hex}\n")
        };
        let table = line(
            1601,
            2006,
            [0, 10, 0, 5, 2, 0, 0, 0],
            [0, 4, 0, 1, 2, 0, 0, 0],
        ) + &line(
            2007,
            30827,
            [0, 11, 0, 1, 2, 0, 0, 0],
            [0, 3, 0, 2, 2, 0, 0, 0],
        );
        super::super::d3d9::load_adapter_table(e, &table);
    }

    fn convert(e: &mut Engine, words: [u16; 8]) -> (u32, [u16; 8]) {
        let utc = e.mem.alloc(16);
        let local = e.mem.alloc(16);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u16(utc + 2 * i as u32, *w);
            e.mem.set_u16(local + 2 * i as u32, 0xaaaa);
        }
        let r =
            system_time_to_tz_specific_local_time(e, Ptr::new(0), Ptr::new(utc), Ptr::new(local));
        (r, std::array::from_fn(|i| e.mem.u16(local + 2 * i as u32)))
    }

    /// The measured answers (`platform5-2026-10-27.tsv`).
    #[test]
    fn local_times_match_the_measured_answers() {
        let mut e = Engine::new();
        eastern_years(&mut e);
        let ok: &[([u16; 8], [u16; 8])] = &[
            ([2026, 7, 3, 1, 12, 0, 0, 0], [2026, 7, 3, 1, 8, 0, 0, 0]),
            (
                [2026, 1, 9, 15, 3, 4, 5, 678],
                [2026, 1, 3, 14, 22, 4, 5, 678],
            ),
            (
                [2026, 3, 0, 8, 6, 59, 59, 999],
                [2026, 3, 0, 8, 1, 59, 59, 999],
            ),
            ([2026, 3, 0, 8, 7, 0, 0, 0], [2026, 3, 0, 8, 3, 0, 0, 0]),
            (
                [2026, 11, 0, 1, 5, 59, 59, 999],
                [2026, 11, 0, 1, 1, 59, 59, 999],
            ),
            ([2026, 11, 0, 1, 6, 0, 0, 0], [2026, 11, 0, 1, 1, 0, 0, 0]),
            ([1601, 1, 1, 1, 5, 0, 0, 0], [1601, 1, 1, 1, 0, 0, 0, 0]),
            (
                [30827, 12, 0, 31, 23, 59, 59, 999],
                [30827, 12, 5, 31, 18, 59, 59, 999],
            ),
            ([2024, 2, 0, 29, 12, 0, 0, 0], [2024, 2, 4, 29, 7, 0, 0, 0]),
            ([2000, 2, 0, 29, 12, 0, 0, 0], [2000, 2, 2, 29, 7, 0, 0, 0]),
        ];
        for (utc, local) in ok {
            e.kernel.last_error = 0x1234;
            assert_eq!(convert(&mut e, *utc), (1, *local), "{utc:?}");
            assert_eq!(e.kernel.last_error, 0x1234);
        }
        let refused: &[[u16; 8]] = &[
            [1601, 1, 1, 1, 0, 0, 0, 0],
            [1600, 12, 0, 31, 23, 0, 0, 0],
            [30828, 1, 0, 1, 0, 0, 0, 0],
            [0, 1, 0, 1, 0, 0, 0, 0],
            [2026, 0, 0, 1, 0, 0, 0, 0],
            [2026, 13, 0, 1, 0, 0, 0, 0],
            [2026, 1, 0, 0, 0, 0, 0, 0],
            [2026, 1, 0, 32, 0, 0, 0, 0],
            [2026, 2, 0, 29, 0, 0, 0, 0],
            [2100, 2, 0, 29, 12, 0, 0, 0],
            [2026, 4, 0, 31, 0, 0, 0, 0],
            [2026, 1, 0, 1, 24, 0, 0, 0],
            [2026, 1, 0, 1, 0, 60, 0, 0],
            [2026, 1, 0, 1, 0, 0, 60, 0],
            [2026, 1, 0, 1, 0, 0, 0, 1000],
            [0xcccc; 8],
            [0; 8],
        ];
        for utc in refused {
            e.kernel.last_error = 0;
            assert_eq!(convert(&mut e, *utc), (0, [0xaaaa; 8]), "{utc:?}");
            assert_eq!(e.kernel.last_error, 87);
        }
    }

    /// The daylight changes the measurement's hour-by-hour scan found, by year (UTC hour of
    /// each change).
    #[test]
    fn daylight_changes_follow_each_year_s_rules() {
        let mut e = Engine::new();
        eastern_years(&mut e);
        let changes: &[(u16, (u16, u16), (u16, u16))] = &[
            (1700, (4, 4), (10, 31)),
            (1986, (4, 6), (10, 26)),
            (2006, (4, 2), (10, 29)),
            (2007, (3, 11), (11, 4)),
            (2100, (3, 14), (11, 7)),
        ];
        for &(year, (m1, d1), (m2, d2)) in changes {
            assert_eq!(convert(&mut e, [year, m1, 0, d1, 6, 0, 0, 0]).1[4], 1);
            assert_eq!(convert(&mut e, [year, m1, 0, d1, 7, 0, 0, 0]).1[4], 3);
            assert_eq!(convert(&mut e, [year, m2, 0, d2, 5, 0, 0, 0]).1[4], 1);
            assert_eq!(convert(&mut e, [year, m2, 0, d2, 6, 0, 0, 0]).1[4], 1);
            assert_eq!(convert(&mut e, [year, m2, 0, d2, 7, 0, 0, 0]).1[4], 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The US Eastern zone's `TIME_ZONE_INFORMATION` as a capture line would give it.
    fn eastern(e: &mut Engine) {
        let mut b = vec![0u8; 172];
        b[0..4].copy_from_slice(&300i32.to_le_bytes());
        for (i, w) in [0u16, 11, 0, 1, 2, 0, 0, 0].iter().enumerate() {
            b[0x44 + 2 * i..0x46 + 2 * i].copy_from_slice(&w.to_le_bytes());
        }
        for (i, w) in [0u16, 3, 0, 2, 2, 0, 0, 0].iter().enumerate() {
            b[0x98 + 2 * i..0x9a + 2 * i].copy_from_slice(&w.to_le_bytes());
        }
        b[0xa8..0xac].copy_from_slice(&(-60i32).to_le_bytes());
        let hex: String = b.iter().map(|x| format!("{x:02x}")).collect();
        super::super::d3d9::load_adapter_table(e, &format!("GetTimeZoneInformation\t\t2\t{hex}\n"));
    }

    #[test]
    fn local_time_is_the_system_time_of_the_local_file_time_now() {
        let mut e = Engine::new();
        eastern(&mut e);
        let at = e.mem.alloc(0x40);
        get_system_time_as_file_time(&mut e, Ptr::new(at));
        file_time_to_local_file_time(&mut e, Ptr::new(at), Ptr::new(at + 8));
        file_time_to_system_time(&mut e, Ptr::new(at + 8), Ptr::new(at + 0x10));
        get_local_time(&mut e, Ptr::new(at + 0x20));
        let words = |e: &Engine, p: u32| (0..8).map(|i| e.mem.u16(p + 2 * i)).collect::<Vec<_>>();
        let (expected, got) = (words(&e, at + 0x10), words(&e, at + 0x20));
        // Same date and minute (a second may tick between the two clock reads).
        assert_eq!(expected[..5], got[..5]);
        assert!(got[0] >= 2026);
    }

    #[test]
    fn local_file_times_take_the_bias_in_effect_now() {
        let mut e = Engine::new();
        eastern(&mut e);
        let utc = e.mem.alloc(8);
        let local = e.mem.alloc(8);
        e.mem.set_u32(utc, 0);
        e.mem.set_u32(utc + 4, 0);
        assert_eq!(
            file_time_to_local_file_time(&mut e, Ptr::new(utc), Ptr::new(local)),
            1
        );
        let zone = zone_info(&mut e);
        let bias = zone::bias_at(&zone, zone::filetime_seconds(now_filetime()));
        let got = u64::from(e.mem.u32(local)) | (u64::from(e.mem.u32(local + 4)) << 32);
        assert_eq!(got, 0u64.wrapping_sub(bias as u64 * 600_000_000));
        let info = e.mem.alloc(172);
        let id = get_time_zone_information(&mut e, Ptr::new(info));
        assert_eq!(id, if bias == 240 { 2 } else { 1 });
        assert_eq!(e.mem.u32(info), 300);
    }

    #[test]
    fn system_times_and_their_limit() {
        let mut e = Engine::new();
        let time = e.mem.alloc(8);
        let out = e.mem.alloc(16);
        e.mem.set_u32(time, 0);
        e.mem.set_u32(time + 4, 0x8000_0000);
        e.mem.set_u32(out, 0xaaaa_aaaa);
        assert_eq!(
            file_time_to_system_time(&mut e, Ptr::new(time), Ptr::new(out)),
            0
        );
        assert_eq!(e.kernel.last_error, 87);
        assert_eq!(e.mem.u32(out), 0xaaaa_aaaa);
        // 1970-01-01 00:00:12.345, a Thursday.
        let ticks = zone::UNIX_EPOCH_TICKS + 123_450_000;
        e.mem.set_u32(time, ticks as u32);
        e.mem.set_u32(time + 4, (ticks >> 32) as u32);
        assert_eq!(
            file_time_to_system_time(&mut e, Ptr::new(time), Ptr::new(out)),
            1
        );
        let words: Vec<u16> = (0..8).map(|i| e.mem.u16(out + 2 * i)).collect();
        assert_eq!(words, [1970, 1, 4, 1, 0, 0, 12, 345]);
    }

    #[test]
    fn ascii_names_convert_and_short_buffers_fail() {
        let mut e = Engine::new();
        let wide = e.mem.alloc(64);
        for (i, c) in "Eastern\0".encode_utf16().enumerate() {
            e.mem.set_u16(wide + 2 * i as u32, c);
        }
        let out = e.mem.alloc(16);
        let used = e.mem.alloc(4);
        e.mem.set_u32(used, 7);
        let call = |e: &mut Engine, size: i32| {
            wide_char_to_multi_byte(
                e,
                0,
                0,
                Ptr::new(wide),
                -1,
                Ptr::new(out),
                size,
                Ptr::new(0),
                Ptr::new(used),
            )
        };
        assert_eq!(call(&mut e, 0x3f), 8);
        assert_eq!(e.mem.cstr(out), b"Eastern".to_vec());
        assert_eq!(e.mem.u32(used), 0);
        assert_eq!(call(&mut e, 0), 8);
        e.mem.write(out, &[0xaa; 8]);
        assert_eq!(call(&mut e, 4), 0);
        assert_eq!(e.kernel.last_error, 122);
        assert_eq!(e.mem.bytes(out, 5), b"East\xaa".to_vec());
    }
}

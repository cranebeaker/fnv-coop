//! Calendar and time-zone arithmetic for the platform's time calls (`FileTimeToSystemTime`,
//! `FileTimeToLocalFileTime`, `GetTimeZoneInformation`): pure functions, no engine state, so
//! the measurement program (`build43-scratch/measure`) checks them against Windows itself.
//!
//! The rules are the documented `TIME_ZONE_INFORMATION` ones: biases in minutes (UTC = local +
//! bias); a `SYSTEMTIME` transition date with `wYear` 0 is "the `wDay`-th (5: the last)
//! `wDayOfWeek` of `wMonth`" at `wHour:wMinute:wSecond.wMilliseconds` local time, the daylight
//! start in standard time and the standard start in daylight time; no daylight time when the
//! daylight date's `wMonth` is 0. Checked on this machine against `SystemTimeToTzSpecificLocalTime`
//! for every hour of 2025-2027 and against `FileTimeToLocalFileTime` and
//! `GetTimeZoneInformation`'s answer for now (see the measurement notes in `time.rs`).

/// 100-nanosecond units per second, and `FILETIME` of 1970-01-01 00:00 UTC.
pub const TICKS_PER_SECOND: u64 = 10_000_000;
pub const UNIX_EPOCH_TICKS: u64 = 116_444_736_000_000_000;
/// Days from 1601-01-01 (`FILETIME`'s epoch, a Monday) to 1970-01-01.
const DAYS_1601_TO_1970: i64 = 134_774;

/// A `TIME_ZONE_INFORMATION` (172 bytes): its biases (minutes) and the two transition dates
/// (`SYSTEMTIME` words: year, month, day of week, day, hour, minute, second, milliseconds).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZoneInfo {
    pub bias: i32,
    pub standard_date: [u16; 8],
    pub standard_bias: i32,
    pub daylight_date: [u16; 8],
    pub daylight_bias: i32,
}

impl ZoneInfo {
    /// The fields of a `TIME_ZONE_INFORMATION`'s bytes (Bias +0, StandardDate +0x44,
    /// StandardBias +0x54, DaylightDate +0x98, DaylightBias +0xA8).
    pub fn from_bytes(b: &[u8]) -> ZoneInfo {
        let i32_at = |o: usize| i32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
        let date = |o: usize| {
            let mut d = [0u16; 8];
            for (i, w) in d.iter_mut().enumerate() {
                *w = u16::from_le_bytes([b[o + 2 * i], b[o + 2 * i + 1]]);
            }
            d
        };
        ZoneInfo {
            bias: i32_at(0),
            standard_date: date(0x44),
            standard_bias: i32_at(0x54),
            daylight_date: date(0x98),
            daylight_bias: i32_at(0xa8),
        }
    }
}

/// Days since 1970-01-01 of a proleptic Gregorian date.
pub fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400;
    let m = i64::from(month);
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + i64::from(day) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// The date of a day count since 1970-01-01: (year, month 1..12, day 1..31).
pub fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let year = yoe + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

/// The day of the week (0 Sunday) of a day count since 1970-01-01 (a Thursday).
pub fn weekday(days: i64) -> u32 {
    (days + 4).rem_euclid(7) as u32
}

fn days_in_month(year: i64, month: u32) -> u32 {
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// The local wall-clock time (seconds since 1970-01-01 as if UTC) at which a transition date
/// falls in `year`.
pub fn transition_seconds(year: i64, date: &[u16; 8]) -> i64 {
    let month = u32::from(date[1]);
    let day = if date[0] != 0 {
        u32::from(date[3])
    } else {
        let first = weekday(days_from_civil(year, month, 1));
        let wanted = u32::from(date[2]);
        let mut day = 1 + (wanted + 7 - first) % 7 + 7 * (u32::from(date[3]).max(1) - 1);
        while day > days_in_month(year, month) {
            day -= 7;
        }
        day
    };
    let year = if date[0] != 0 {
        i64::from(date[0])
    } else {
        year
    };
    days_from_civil(year, month, day) * 86_400
        + i64::from(date[4]) * 3600
        + i64::from(date[5]) * 60
        + i64::from(date[6])
}

/// Whether daylight time is in effect at `utc` (seconds since 1970); `None` without daylight
/// time in the zone.
pub fn in_daylight(zone: &ZoneInfo, utc: i64) -> Option<bool> {
    if zone.daylight_date[1] == 0 {
        return None;
    }
    let standard = utc - 60 * i64::from(zone.bias + zone.standard_bias);
    let (year, _, _) = civil_from_days(standard.div_euclid(86_400));
    let start = transition_seconds(year, &zone.daylight_date);
    // The standard date is in daylight time: as standard time it is the bias difference
    // earlier.
    let end = transition_seconds(year, &zone.standard_date)
        + 60 * i64::from(zone.daylight_bias - zone.standard_bias);
    Some(if start < end {
        standard >= start && standard < end
    } else {
        !(standard >= end && standard < start)
    })
}

/// The bias (minutes, UTC = local + bias) in effect at `utc` (seconds since 1970).
pub fn bias_at(zone: &ZoneInfo, utc: i64) -> i32 {
    match in_daylight(zone, utc) {
        Some(true) => zone.bias + zone.daylight_bias,
        _ => zone.bias + zone.standard_bias,
    }
}

/// A `FILETIME` as seconds since 1970 (rounded down).
pub fn filetime_seconds(ticks: u64) -> i64 {
    (ticks as i64 - UNIX_EPOCH_TICKS as i64).div_euclid(TICKS_PER_SECOND as i64)
}

/// The `SYSTEMTIME` words of a `FILETIME` (`None` above 0x7FFF_FFFF_FFFF_FFFF, which Windows
/// refuses; 0x7FFF_FFFF_FFFF_FFFF itself is 30828-09-14 02:48:05.477, measured): year, month, day of week, day, hour, minute, second, milliseconds.
pub fn system_time(ticks: u64) -> Option<[u16; 8]> {
    if ticks > 0x7fff_ffff_ffff_ffff {
        return None;
    }
    let ms_total = ticks / 10_000;
    let days_1601 = (ms_total / 86_400_000) as i64;
    let ms_of_day = ms_total % 86_400_000;
    let (year, month, day) = civil_from_days(days_1601 - DAYS_1601_TO_1970);
    Some([
        year as u16,
        month as u16,
        weekday(days_1601 - DAYS_1601_TO_1970) as u16,
        day as u16,
        (ms_of_day / 3_600_000) as u16,
        (ms_of_day / 60_000 % 60) as u16,
        (ms_of_day / 1000 % 60) as u16,
        (ms_of_day % 1000) as u16,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_calendar_round_trips_and_knows_its_weekdays() {
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(civil_from_days(days_from_civil(2024, 2, 29)), (2024, 2, 29));
        assert_eq!(weekday(days_from_civil(2026, 10, 21)), 3);
        assert_eq!(
            system_time(UNIX_EPOCH_TICKS),
            Some([1970, 1, 4, 1, 0, 0, 0, 0])
        );
        assert_eq!(system_time(0), Some([1601, 1, 1, 1, 0, 0, 0, 0]));
        assert_eq!(system_time(0x8000_0000_0000_0000), None);
        assert_eq!(
            system_time(0x7fff_ffff_ffff_ffff),
            Some([30828, 9, 4, 14, 2, 48, 5, 477])
        );
    }

    #[test]
    fn us_eastern_rules_put_july_in_daylight_time() {
        // Bias 300, daylight -60 from the second Sunday of March 2:00 to the first Sunday of
        // November 2:00.
        let zone = ZoneInfo {
            bias: 300,
            standard_date: [0, 11, 0, 1, 2, 0, 0, 0],
            standard_bias: 0,
            daylight_date: [0, 3, 0, 2, 2, 0, 0, 0],
            daylight_bias: -60,
        };
        let at = |y: i64, m: u32, d: u32, h: i64| days_from_civil(y, m, d) * 86_400 + h * 3600;
        assert_eq!(bias_at(&zone, at(2026, 7, 1, 12)), 240);
        assert_eq!(bias_at(&zone, at(2026, 1, 1, 12)), 300);
        // 2026-03-08 07:00 UTC is 2:00 EST: daylight begins.
        assert_eq!(bias_at(&zone, at(2026, 3, 8, 6) + 3599), 300);
        assert_eq!(bias_at(&zone, at(2026, 3, 8, 7)), 240);
        // 2026-11-01 06:00 UTC is 2:00 EDT: standard begins.
        assert_eq!(bias_at(&zone, at(2026, 11, 1, 5) + 3599), 240);
        assert_eq!(bias_at(&zone, at(2026, 11, 1, 6)), 300);
    }
}

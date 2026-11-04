//! `fallout/misc/calendar.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The game calendar keeps its date and time in six `TESGlobal` forms (year,
//! month, day, hour, days passed, time scale). Every function here reads or
//! writes them through `TESGlobal`'s value getter (`00526ac0`, which the
//! engine map names `BSMultiBoundCapsule::QMultiBoundRadius` because the
//! linker folded the identical code) and setter (`0046dce0`).
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` where the
//! code stores a `float`; the two can differ only in the last bit of a
//! result rounded twice, which does not matter for the whole numbers and the
//! quarter-hour steps the calendar handles.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESGlobal` value getter (`float TESGlobal::fValue`, returned in ST0).
const TES_GLOBAL_GET_VALUE: u32 = 0x0052_6ac0;
/// `TESGlobal` value setter (`float` argument on the stack).
const TES_GLOBAL_SET_VALUE: u32 = 0x0046_dce0;
/// `_ftol2_sse` (`00ec62c0`): truncates the float in ST0 to an integer in
/// EAX. The uniform form has no ST0 argument, so the value is passed as an
/// `f64` argument (two words, exact for every `float`).
const FTOL: u32 = 0x00ec_62c0;
/// Number of days in month `n` (0 = January), 0 for `n >= 12` (`u16` result).
const DAYS_IN_MONTH: u32 = 0x004b_10d0;

/// The calendar's `float` default for the year when `pGameYear` is null.
const DEFAULT_YEAR: u32 = 0x0101_8fa8;
/// The default for the day when `pGameDay` is null.
const DEFAULT_DAY: u32 = 0x0101_8f8c;
/// The default for the hour when `pGameHour` is null.
const DEFAULT_HOUR: u32 = 0x0101_8f7c;
/// `3600.0` (`double`): seconds per hour.
const SECONDS_PER_HOUR: u32 = 0x0101_2640;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `24.0` (`double`): hours per day.
const HOURS_PER_DAY: u32 = 0x0103_56d8;
/// `12.0` (`double`): months per year.
const MONTHS_PER_YEAR: u32 = 0x0103_5710;
/// `Calendar::Update`'s function-local `static float` holding the hour of
/// the previous update, and the guard bit (bit 0 of the word after it) that
/// says it has been initialized.
const LAST_HOUR: u32 = 0x011d_e7e4;
const LAST_HOUR_INITIALIZED: u32 = 0x011d_e7e8;
/// `"%02d.%02d.%02d"`.
const DATE_FORMAT: u32 = 0x0108_2624;

layout! {
    /// `Calendar` (Xbox PDB), 0x1C bytes on the Xbox; the PC build adds one
    /// byte at +0x1C.
    pub struct Calendar: 0x20 {
        /// `pGameYear` (Xbox PDB): `TESGlobal*`.
        0x00 pGameYear: Ptr,
        /// `pGameMonth` (Xbox PDB): `TESGlobal*`.
        0x04 pGameMonth: Ptr,
        /// `pGameDay` (Xbox PDB): `TESGlobal*`.
        0x08 pGameDay: Ptr,
        /// `pGameHour` (Xbox PDB): `TESGlobal*`.
        0x0C pGameHour: Ptr,
        /// `pGameDaysPassed` (Xbox PDB): `TESGlobal*`.
        0x10 pGameDaysPassed: Ptr,
        /// `pTimeScale` (Xbox PDB): `TESGlobal*`.
        0x14 pTimeScale: Ptr,
        /// `iMidnightsPassed` (Xbox PDB).
        0x18 iMidnightsPassed: u32,
        /// PC only (not in the Xbox PDB): set by `00867a20`, cleared by the
        /// update once it has rewritten the days-passed global.
        0x1C bDaysPassedStale: bool,
    }
}

/// `TESGlobal`'s value in ST0, as a `float`.
fn global_value(e: &mut Engine, global: Ptr) -> f32 {
    e.call(TES_GLOBAL_GET_VALUE, &args![global]).f32()
}

fn set_global_value(e: &mut Engine, global: Ptr, value: f32) {
    e.call(TES_GLOBAL_SET_VALUE, &args![global, value]);
}

/// `_ftol2_sse` on a `float`.
fn float_to_int(e: &mut Engine, value: f32) -> i32 {
    e.call(FTOL, &args![value as f64]).i32()
}

// Translated from 00867950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Calendar::GetTimeScale` (Xbox PDB): the value of the time-scale global.
pub fn calendar_get_time_scale(e: &mut Engine, this: Ptr<Calendar>) -> f32 {
    let time_scale = e.get(this, Calendar::pTimeScale);
    global_value(e, time_scale)
}

// Translated from 00867970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Calendar::GetDateString` (Xbox PDB): writes the date as
/// `"%02d.%02d.%02d"` (month.day.two-digit year) into the string at `result`
/// and returns `result`.
///
/// The temporary string is an 8-byte `BSStringT` (`004037b0` constructs it,
/// `004047f0` copies it into `result`, `004037d0` destroys it). The
/// compiler's exception-unwinding frame is not translated.
pub fn calendar_get_date_string(e: &mut Engine, this: Ptr<Calendar>, result: Ptr) -> Ptr {
    let text = e.mem.alloc(8);
    e.call(0x0040_37b0, &args![text]);
    let year = fn_00867c60(e, this) as u32;
    let day = calendar_get_day(e, this) as i8;
    let month = calendar_get_month(e, this);
    e.call(
        0x0040_6f60,
        &args![text, DATE_FORMAT, month + 1, day as i32, year % 100],
    );
    e.call(0x0040_47f0, &args![result, text]);
    e.call(0x0040_37d0, &args![text]);
    e.mem.free(text);
    result
}

// Translated from 00867a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks the days-passed global as needing a rewrite on the next update.
pub fn fn_00867a20(e: &mut Engine, this: Ptr<Calendar>) {
    e.set(this, Calendar::bDaysPassedStale, true);
}

// Translated from 00867a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Advances the calendar by `seconds` of real time: scales it by the time
/// scale into game hours, adds them to the hour global, and carries over
/// into day, month, year and `iMidnightsPassed` whenever the hour passes 24.
pub fn fn_00867a40(e: &mut Engine, this: Ptr<Calendar>, seconds: f32) {
    let time_scale_global = e.get(this, Calendar::pTimeScale);
    let time_scale = global_value(e, time_scale_global);
    let seconds_per_hour: f64 = e.global(SECONDS_PER_HOUR);
    let elapsed_hours = ((time_scale as f64 * seconds as f64) / seconds_per_hour) as f32;

    let hour_global = e.get(this, Calendar::pGameHour);
    let hour = global_value(e, hour_global);
    let mut new_hour = (hour as f64 + elapsed_hours as f64) as f32;

    // `Calendar::Update`'s `static float` (initialized on the first call).
    if e.global::<u32>(LAST_HOUR_INITIALIZED) & 1 == 0 {
        let flags: u32 = e.global(LAST_HOUR_INITIALIZED);
        e.set_global(LAST_HOUR_INITIALIZED, flags | 1);
        e.set_global(LAST_HOUR, new_hour);
    }

    let one: f64 = e.global(ONE);
    let hours_per_day: f64 = e.global(HOURS_PER_DAY);
    let last_hour: f32 = e.global(LAST_HOUR);
    if e.get(this, Calendar::bDaysPassedStale) || last_hour as f64 + one < new_hour as f64 {
        let days_passed_global = e.get(this, Calendar::pGameDaysPassed);
        let whole_days_value = global_value(e, days_passed_global);
        let whole_days = float_to_int(e, whole_days_value);
        let days_passed = (whole_days as f64 + new_hour as f64 / hours_per_day) as f32;
        set_global_value(e, days_passed_global, days_passed);
        e.set(this, Calendar::bDaysPassedStale, false);
    }

    if new_hour as f64 > hours_per_day {
        let day_global = e.get(this, Calendar::pGameDay);
        let mut day = global_value(e, day_global);
        let month_global = e.get(this, Calendar::pGameMonth);
        let mut month = global_value(e, month_global);
        let year_global = e.get(this, Calendar::pGameYear);
        let mut year = global_value(e, year_global);

        let month_index = float_to_int(e, month) as i8;
        let days_in_month = e.call(DAYS_IN_MONTH, &args![month_index as i32]).u16();

        while new_hour as f64 > hours_per_day {
            new_hour = (new_hour as f64 - hours_per_day) as f32;
            day = (day as f64 + one) as f32;
        }

        if day as f64 > days_in_month as f64 {
            day = (day as f64 - days_in_month as f64) as f32;
            month = (month as f64 + one) as f32;
            let months_per_year: f64 = e.global(MONTHS_PER_YEAR);
            if month as f64 >= months_per_year {
                month = (month as f64 - months_per_year) as f32;
                year = (year as f64 + one) as f32;
                set_global_value(e, year_global, year);
            }
            set_global_value(e, month_global, month);
        }
        set_global_value(e, day_global, day);

        let midnights = e.get(this, Calendar::iMidnightsPassed);
        e.set(this, Calendar::iMidnightsPassed, midnights.wrapping_add(1));
    }

    let days_passed_global = e.get(this, Calendar::pGameDaysPassed);
    let days_passed = global_value(e, days_passed_global);
    let days_passed = (days_passed as f64 + elapsed_hours as f64 / hours_per_day) as f32;
    set_global_value(e, days_passed_global, days_passed);

    e.set_global(LAST_HOUR, new_hour);
    let hour_global = e.get(this, Calendar::pGameHour);
    set_global_value(e, hour_global, new_hour);
}

// Translated from 00867c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The year: the year global truncated to an integer, 77 when there is no
/// year global.
pub fn fn_00867c60(e: &mut Engine, this: Ptr<Calendar>) -> i32 {
    let year_global = e.get(this, Calendar::pGameYear);
    let year = if year_global.is_null() {
        e.global::<f32>(DEFAULT_YEAR)
    } else {
        global_value(e, year_global)
    };
    float_to_int(e, year)
}

// Translated from 00867ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The season of the current month: 0 for March to May, 1 for June to
/// August, 2 for September to November, 3 for December to February (0 for a
/// month outside 0..12).
pub fn fn_00867ca0(e: &mut Engine, this: Ptr<Calendar>) -> i32 {
    let month = calendar_get_month(e, this);
    // The compiler's switch: a byte table maps the month to one of four
    // cases (`00867d0c`: 0 0 1 1 1 2 2 2 3 3 3 0), compared as unsigned.
    match month as u32 {
        0 | 1 | 11 => 3,
        2..=4 => 0,
        5..=7 => 1,
        8..=10 => 2,
        _ => 0,
    }
}

// Translated from 00867d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Calendar::GetMonth` (Xbox PDB): the month global truncated to a signed
/// byte (0 = January), 7 when there is no month global.
pub fn calendar_get_month(e: &mut Engine, this: Ptr<Calendar>) -> i32 {
    let month_global = e.get(this, Calendar::pGameMonth);
    if month_global.is_null() {
        return 7;
    }
    let month = global_value(e, month_global);
    float_to_int(e, month) as i8 as i32
}

// Translated from 00867d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Calendar::GetDay` (Xbox PDB): the day global truncated to an integer,
/// 17 when there is no day global.
pub fn calendar_get_day(e: &mut Engine, this: Ptr<Calendar>) -> i32 {
    let day_global = e.get(this, Calendar::pGameDay);
    let day = if day_global.is_null() {
        e.global::<f32>(DEFAULT_DAY)
    } else {
        global_value(e, day_global)
    };
    float_to_int(e, day)
}

// Translated from 00867da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Calendar::GetHour` (Xbox PDB): the hour global, 12.0 when there is no
/// hour global.
pub fn calendar_get_hour(e: &mut Engine, this: Ptr<Calendar>) -> f32 {
    let hour_global = e.get(this, Calendar::pGameHour);
    if hour_global.is_null() {
        e.global::<f32>(DEFAULT_HOUR)
    } else {
        global_value(e, hour_global)
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00867950, calendar_get_time_scale(Ptr<Calendar>) -> f32),
        entry!(
            0x00867970,
            calendar_get_date_string(Ptr<Calendar>, Ptr) -> Ptr
        ),
        entry!(0x00867a20, fn_00867a20(Ptr<Calendar>)),
        entry!(0x00867a40, fn_00867a40(Ptr<Calendar>, f32)),
        entry!(0x00867c60, fn_00867c60(Ptr<Calendar>) -> i32),
        entry!(0x00867ca0, fn_00867ca0(Ptr<Calendar>) -> i32),
        entry!(0x00867d20, calendar_get_month(Ptr<Calendar>) -> i32),
        entry!(0x00867d60, calendar_get_day(Ptr<Calendar>) -> i32),
        entry!(0x00867da0, calendar_get_hour(Ptr<Calendar>) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test doubles for `TESGlobal`'s getter/setter (`fValue` at +0x24),
    /// `_ftol2_sse` (truncation) and the days-per-month table, plus the
    /// constants the code loads from the exe's data.
    fn calendar_engine() -> Engine {
        let mut e = Engine::new();
        e.register(TES_GLOBAL_GET_VALUE, |e, a| {
            e.mem.f32(a[0] + 0x24).into_ret()
        });
        e.register(TES_GLOBAL_SET_VALUE, |e, a| {
            e.mem.set_f32(a[0] + 0x24, f32::from_bits(a[1]));
            Ret::default()
        });
        e.register(FTOL, |_, a| {
            let value = f64::take(a, &mut 0);
            Ret {
                eax: value as i32 as u32,
                ..Ret::default()
            }
        });
        e.register(DAYS_IN_MONTH, |_, a| {
            let days = [31u32, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            Ret {
                eax: days.get(a[0] as usize).copied().unwrap_or(0),
                ..Ret::default()
            }
        });
        // The pages holding the constants and the update's static.
        for page in [0x0101_2000, 0x0101_8000, 0x0103_5000, 0x011d_e000] {
            e.map(page, 0x1000);
        }
        e.set_global(DEFAULT_YEAR, 77.0f32);
        e.set_global(DEFAULT_DAY, 17.0f32);
        e.set_global(DEFAULT_HOUR, 12.0f32);
        e.set_global(SECONDS_PER_HOUR, 3600.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(HOURS_PER_DAY, 24.0f64);
        e.set_global(MONTHS_PER_YEAR, 12.0f64);
        e
    }

    /// A calendar whose six globals hold the given values (year, month, day,
    /// hour, days passed, time scale).
    fn calendar_with(e: &mut Engine, values: [f32; 6]) -> (Ptr<Calendar>, [Ptr; 6]) {
        let calendar: Ptr<Calendar> = e.new_object();
        let mut globals = [Ptr::NULL; 6];
        for (i, value) in values.iter().enumerate() {
            let global = Ptr::new(e.mem.alloc(0x28));
            e.mem.set_f32(global.addr() + 0x24, *value);
            e.mem.set_u32(calendar.addr() + 4 * i as u32, global.addr());
            globals[i] = global;
        }
        (calendar, globals)
    }

    fn value(e: &Engine, global: Ptr) -> f32 {
        e.mem.f32(global.addr() + 0x24)
    }

    #[test]
    fn get_time_scale_reads_the_time_scale_global() {
        let mut e = calendar_engine();
        let (calendar, _) = calendar_with(&mut e, [0.0, 0.0, 0.0, 0.0, 0.0, 30.0]);
        assert_eq!(e.call(0x0086_7950, &args![calendar]).f32(), 30.0);
    }

    #[test]
    fn get_date_string_formats_month_day_and_two_digit_year() {
        let mut e = calendar_engine();
        let (calendar, _) = calendar_with(&mut e, [277.0, 9.0, 3.0, 0.0, 0.0, 0.0]);
        let result: Ptr = Ptr::new(e.mem.alloc(8));
        e.register(0x0040_37b0, |_, _| Ret::default());
        e.register(0x0040_37d0, |_, _| Ret::default());
        e.register(0x0040_47f0, |_, _| Ret::default());
        e.register(0x0040_6f60, |e, a| {
            // The format call: text, format, then three integers.
            e.mem.set_u32(a[0], a[2]);
            e.mem.set_u32(a[0] + 4, a[3]);
            e.mem.set_u32(a[0] + 8, a[4]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let back = e.call(0x0086_7970, &args![calendar, result]).ptr::<()>();
        assert_eq!(back, result);
        let log = e.call_log.take().unwrap();
        let format = log.iter().find(|(addr, _)| *addr == 0x0040_6f60).unwrap();
        // Month is 1-based, the year is reduced to two digits.
        assert_eq!(&format.1[1..], &[DATE_FORMAT, 10, 3, 77]);
        let copy = log.iter().find(|(addr, _)| *addr == 0x0040_47f0).unwrap();
        assert_eq!(copy.1[0], result.addr());
    }

    #[test]
    fn get_date_string_uses_defaults_without_globals() {
        let mut e = calendar_engine();
        let calendar: Ptr<Calendar> = e.new_object();
        let result: Ptr = Ptr::new(e.mem.alloc(8));
        e.register(0x0040_37b0, |_, _| Ret::default());
        e.register(0x0040_37d0, |_, _| Ret::default());
        e.register(0x0040_47f0, |_, _| Ret::default());
        e.register(0x0040_6f60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0086_7970, &args![calendar, result]);
        let log = e.call_log.take().unwrap();
        let format = log.iter().find(|(addr, _)| *addr == 0x0040_6f60).unwrap();
        // Month 7 (August) as 8, day 17, year 77.
        assert_eq!(&format.1[2..], &[8, 17, 77]);
    }

    #[test]
    fn mark_stale_sets_the_flag() {
        let mut e = calendar_engine();
        let calendar: Ptr<Calendar> = e.new_object();
        e.call(0x0086_7a20, &args![calendar]);
        assert!(e.get(calendar, Calendar::bDaysPassedStale));
    }

    #[test]
    fn update_advances_the_hour_within_a_day() {
        let mut e = calendar_engine();
        // 2277-10-03, 10:00, day 5.0 of the game, time scale 30.
        let (calendar, g) = calendar_with(&mut e, [2277.0, 9.0, 3.0, 10.0, 5.0, 30.0]);
        // 120 real seconds at scale 30 = one game hour.
        e.call(0x0086_7a40, &args![calendar, 120.0f32]);
        assert_eq!(value(&e, g[3]), 11.0);
        assert_eq!(value(&e, g[2]), 3.0);
        assert_eq!(value(&e, g[4]), 5.0 + 1.0 / 24.0);
        assert_eq!(e.global::<f32>(LAST_HOUR), 11.0);
        assert_eq!(e.get(calendar, Calendar::iMidnightsPassed), 0);
    }

    #[test]
    fn update_rewrites_days_passed_when_stale_or_more_than_an_hour_jumped() {
        let mut e = calendar_engine();
        let (calendar, g) = calendar_with(&mut e, [2277.0, 9.0, 3.0, 10.0, 5.0, 30.0]);
        e.set_global(LAST_HOUR_INITIALIZED, 1u32);
        e.set_global(LAST_HOUR, 10.0f32);
        // Stale flag: days passed becomes 5 whole days + hour/24, then the
        // elapsed hours are added.
        e.set(calendar, Calendar::bDaysPassedStale, true);
        e.call(0x0086_7a40, &args![calendar, 120.0f32]);
        let expected = (5.0f64 + 11.0 / 24.0) as f32;
        assert_eq!(value(&e, g[4]), (expected as f64 + 1.0 / 24.0) as f32);
        assert!(!e.get(calendar, Calendar::bDaysPassedStale));

        // No flag, and the hour moved by less than an hour (10.5 -> 11.5
        // against a last hour of 11.5): days passed only gets the increment.
        let (calendar, g) = calendar_with(&mut e, [2277.0, 9.0, 3.0, 10.5, 5.0, 30.0]);
        e.set_global(LAST_HOUR, 10.5f32);
        e.call(0x0086_7a40, &args![calendar, 120.0f32]);
        assert_eq!(value(&e, g[4]), (5.0f64 + 1.0 / 24.0) as f32);

        // No flag, but the new hour is more than 1.0 past the last one.
        let (calendar, g) = calendar_with(&mut e, [2277.0, 9.0, 3.0, 10.0, 5.0, 30.0]);
        e.set_global(LAST_HOUR, 5.0f32);
        e.call(0x0086_7a40, &args![calendar, 120.0f32]);
        let expected = (5.0f64 + 11.0 / 24.0) as f32;
        assert_eq!(value(&e, g[4]), (expected as f64 + 1.0 / 24.0) as f32);
    }

    #[test]
    fn update_first_call_initializes_the_static_hour() {
        let mut e = calendar_engine();
        let (calendar, _) = calendar_with(&mut e, [2277.0, 9.0, 3.0, 10.0, 5.0, 30.0]);
        e.call(0x0086_7a40, &args![calendar, 120.0f32]);
        assert_eq!(e.global::<u32>(LAST_HOUR_INITIALIZED) & 1, 1);
        assert_eq!(e.global::<f32>(LAST_HOUR), 11.0);
    }

    #[test]
    fn update_rolls_over_midnight_into_the_next_day() {
        let mut e = calendar_engine();
        let (calendar, g) = calendar_with(&mut e, [2277.0, 9.0, 3.0, 23.5, 5.0, 30.0]);
        e.set_global(LAST_HOUR_INITIALIZED, 1u32);
        e.set_global(LAST_HOUR, 23.5f32);
        // 240 real seconds = two game hours: 25.5 -> hour 1.5 of day 4.
        e.call(0x0086_7a40, &args![calendar, 240.0f32]);
        assert_eq!(value(&e, g[3]), 1.5);
        assert_eq!(value(&e, g[2]), 4.0);
        assert_eq!(value(&e, g[1]), 9.0);
        assert_eq!(e.get(calendar, Calendar::iMidnightsPassed), 1);
    }

    #[test]
    fn update_rolls_the_day_into_the_next_month() {
        let mut e = calendar_engine();
        // October 31st, 23:00: the next day is November 1st.
        let (calendar, g) = calendar_with(&mut e, [2277.0, 9.0, 31.0, 23.0, 5.0, 30.0]);
        e.set_global(LAST_HOUR_INITIALIZED, 1u32);
        e.set_global(LAST_HOUR, 23.0f32);
        e.call(0x0086_7a40, &args![calendar, 240.0f32]);
        assert_eq!(value(&e, g[3]), 1.0);
        assert_eq!(value(&e, g[2]), 1.0);
        assert_eq!(value(&e, g[1]), 10.0);
        assert_eq!(value(&e, g[0]), 2277.0);
    }

    #[test]
    fn update_rolls_december_into_the_next_year() {
        let mut e = calendar_engine();
        let (calendar, g) = calendar_with(&mut e, [2277.0, 11.0, 31.0, 23.0, 5.0, 30.0]);
        e.set_global(LAST_HOUR_INITIALIZED, 1u32);
        e.set_global(LAST_HOUR, 23.0f32);
        e.call(0x0086_7a40, &args![calendar, 240.0f32]);
        assert_eq!(value(&e, g[2]), 1.0);
        assert_eq!(value(&e, g[1]), 0.0);
        assert_eq!(value(&e, g[0]), 2278.0);
    }

    #[test]
    fn year_truncates_the_global_and_defaults_to_77() {
        let mut e = calendar_engine();
        let (calendar, _) = calendar_with(&mut e, [277.9, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.call(0x0086_7c60, &args![calendar]).i32(), 277);
        let bare: Ptr<Calendar> = e.new_object();
        assert_eq!(e.call(0x0086_7c60, &args![bare]).i32(), 77);
    }

    #[test]
    fn season_follows_the_month() {
        let mut e = calendar_engine();
        let expected = [3, 3, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3];
        for (month, season) in expected.iter().enumerate() {
            let (calendar, _) = calendar_with(&mut e, [0.0, month as f32, 0.0, 0.0, 0.0, 0.0]);
            assert_eq!(e.call(0x0086_7ca0, &args![calendar]).i32(), *season);
        }
        // A month outside 0..12 (here 12 and a negative one) is season 0.
        for month in [12.0f32, -1.0] {
            let (calendar, _) = calendar_with(&mut e, [0.0, month, 0.0, 0.0, 0.0, 0.0]);
            assert_eq!(e.call(0x0086_7ca0, &args![calendar]).i32(), 0);
        }
        // Without a month global the month is 7 (August): season 1.
        let bare: Ptr<Calendar> = e.new_object();
        assert_eq!(e.call(0x0086_7ca0, &args![bare]).i32(), 1);
    }

    #[test]
    fn month_is_a_signed_byte_and_defaults_to_7() {
        let mut e = calendar_engine();
        let (calendar, _) = calendar_with(&mut e, [0.0, 4.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.call(0x0086_7d20, &args![calendar]).i32(), 4);
        // 200 does not fit a signed byte: the game sees -56.
        let (calendar, _) = calendar_with(&mut e, [0.0, 200.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.call(0x0086_7d20, &args![calendar]).i32(), -56);
        let bare: Ptr<Calendar> = e.new_object();
        assert_eq!(e.call(0x0086_7d20, &args![bare]).i32(), 7);
    }

    #[test]
    fn day_truncates_the_global_and_defaults_to_17() {
        let mut e = calendar_engine();
        let (calendar, _) = calendar_with(&mut e, [0.0, 0.0, 12.75, 0.0, 0.0, 0.0]);
        assert_eq!(e.call(0x0086_7d60, &args![calendar]).i32(), 12);
        let bare: Ptr<Calendar> = e.new_object();
        assert_eq!(e.call(0x0086_7d60, &args![bare]).i32(), 17);
    }

    #[test]
    fn hour_reads_the_global_and_defaults_to_12() {
        let mut e = calendar_engine();
        let (calendar, _) = calendar_with(&mut e, [0.0, 0.0, 0.0, 6.25, 0.0, 0.0]);
        assert_eq!(e.call(0x0086_7da0, &args![calendar]).f32(), 6.25);
        let bare: Ptr<Calendar> = e.new_object();
        assert_eq!(e.call(0x0086_7da0, &args![bare]).f32(), 12.0);
    }

    /// `Calendar::Update`'s calls follow `world::frame::world_time`'s model of
    /// it for each of its tests (docs/FRAME_SKELETON.md, PR 5): the
    /// `TESGlobal` setter on the globals of the reached sub-steps, in order.
    #[test]
    fn update_follows_the_frame_model() {
        use world::frame::world_time::{follows, function, steps_run, Callee, WorldState};
        let model = function(0x0086_7a40).expect("modelled");
        // The global each setter call site writes (indices into
        // `calendar_with`'s list).
        let global_of = |site: u32| match site {
            0x0086_7ae6 | 0x0086_7c35 => 4,
            0x0086_7bd7 => 0,
            0x0086_7be9 => 1,
            0x0086_7bfb => 2,
            0x0086_7c50 => 3,
            _ => usize::MAX,
        };
        // (year, month, day, hour, days passed, time scale), seconds, stale.
        let cases: [([f32; 6], f32, bool, WorldState); 5] = [
            (
                [2277.0, 9.0, 3.0, 10.0, 5.0, 30.0],
                120.0,
                false,
                WorldState::default(),
            ),
            (
                [2277.0, 9.0, 3.0, 10.0, 5.0, 30.0],
                120.0,
                true,
                WorldState {
                    calendar_rewrite: true,
                    ..WorldState::default()
                },
            ),
            (
                [2277.0, 9.0, 3.0, 23.5, 5.0, 30.0],
                120.0,
                false,
                WorldState {
                    midnight: true,
                    ..WorldState::default()
                },
            ),
            (
                [2277.0, 3.0, 30.0, 23.5, 5.0, 30.0],
                120.0,
                false,
                WorldState {
                    midnight: true,
                    month_ends: true,
                    ..WorldState::default()
                },
            ),
            (
                [2277.0, 11.0, 31.0, 23.5, 5.0, 30.0],
                120.0,
                false,
                WorldState {
                    midnight: true,
                    month_ends: true,
                    year_ends: true,
                    ..WorldState::default()
                },
            ),
        ];
        for (values, seconds, stale, s) in cases {
            let mut e = calendar_engine();
            let (calendar, globals) = calendar_with(&mut e, values);
            e.set(calendar, Calendar::bDaysPassedStale, stale);
            e.call_log = Some(vec![]);
            e.call(0x0086_7a40, &args![calendar, seconds]);
            let log = e.call_log.take().unwrap();
            let callees: Vec<Callee> = log.iter().map(|(a, _)| Callee::Direct(*a)).collect();
            follows(model, &s, &callees, &[]).unwrap_or_else(|why| panic!("{s:?}: {why}"));
            let set: Vec<u32> = log
                .iter()
                .filter(|(a, _)| *a == TES_GLOBAL_SET_VALUE)
                .map(|(_, w)| w[0])
                .collect();
            let want: Vec<u32> = steps_run(&s, model)
                .into_iter()
                .map(|i| model.steps[i].sites[0])
                .filter(|&site| global_of(site) != usize::MAX)
                .map(|site| globals[global_of(site)].addr())
                .collect();
            assert_eq!(set, want, "{s:?}");
        }
    }
}

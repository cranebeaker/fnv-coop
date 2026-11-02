//! `fallout/misc/run_00867de0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Small `Calendar` methods that follow `calendar.cpp`'s functions in the
//! exe (`Calendar` is declared in `calendar.rs`; the engine map has no name
//! for any of them): the whole days passed (`00867de0`), the days passed times 24 floored
//! (`00867e30`), the days passed times 24 times 60 (`00867ea0`), the day of the
//! week (`00867ef0`) and the table string for that weekday (`00867f10`).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::fallout_misc::calendar::Calendar;

/// `TESGlobal` value getter (`float TESGlobal::fValue`, returned in ST0);
/// the same code as in `calendar.rs`.
const TES_GLOBAL_GET_VALUE: u32 = 0x0052_6ac0;
/// `24.0` (`double`) and `60.0` (`double`).
const HOURS_PER_DAY: u32 = 0x0103_56d8;
const SIXTY: u32 = 0x0101_2638;
/// `00406cc0`: cdecl, a `float`; returns a `float` in ST0 (it forwards to
/// `00406ce0`, which calls `floor` through `00ec6940`).
const FLOOR_FLOAT: u32 = 0x0040_6cc0;
/// Seven pointers to setting objects, indexed by the day of the week.
const DAY_NAME_SETTINGS: u32 = 0x0118_95b8;
/// `00403df0`: the setting's string value: the word at `+4` of the object
/// in `ECX`, or 0 for a null object.
const SETTING_STRING: u32 = 0x0040_3df0;
/// `"Bad Day"`.
const BAD_DAY: u32 = 0x0108_2634;

// Translated from 00867de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The whole number of days passed: the days-passed global (`pGameDaysPassed`)
/// as a `float`, 1.0 when the calendar has no such global, truncated toward
/// zero (the code switches the x87 rounding mode to truncate around its
/// `FISTP` of a 64-bit integer) and returned as the low 32 bits. A value the
/// 64-bit integer cannot hold gives the processor's "integer indefinite", whose
/// low 32 bits are 0.
pub fn fn_00867de0(e: &mut Engine, this: Ptr<Calendar>) -> u32 {
    let days_passed_global = e.get(this, Calendar::pGameDaysPassed);
    let days_passed = if days_passed_global.is_null() {
        1.0f32
    } else {
        e.call(TES_GLOBAL_GET_VALUE, &args![days_passed_global])
            .f32()
    };
    if days_passed.is_finite() && days_passed.abs() < 9.223_372e18 {
        days_passed as i64 as u32
    } else {
        0
    }
}

// Translated from 00867ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The day of the week: the whole days passed (`00867de0`) modulo 7, as an
/// unsigned number.
pub fn fn_00867ef0(e: &mut Engine, this: Ptr<Calendar>) -> u32 {
    fn_00867de0(e, this) % 7
}

/// The days-passed global as a `float`: 1.0 when the calendar has none.
fn days_passed_value(e: &mut Engine, this: Ptr<Calendar>) -> f32 {
    let days_passed_global = e.get(this, Calendar::pGameDaysPassed);
    if days_passed_global.is_null() {
        1.0f32
    } else {
        e.call(TES_GLOBAL_GET_VALUE, &args![days_passed_global])
            .f32()
    }
}

// Translated from 00867e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The days passed (1.0 without the global) times 24 (`double`), rounded to a
/// `float`, rounded down by `00406cc0`, then truncated toward zero to a 64-bit
/// integer (the code switches the x87 rounding mode around its `FISTP`) of
/// which the low 32 bits are returned. A value the integer cannot hold gives
/// the processor's "integer indefinite", whose low 32 bits are 0. Other
/// translations call it for the game time.
pub fn fn_00867e30(e: &mut Engine, this: Ptr<Calendar>) -> u32 {
    let days_passed = days_passed_value(e, this);
    let hours_per_day: f64 = e.global(HOURS_PER_DAY);
    let hours = (days_passed as f64 * hours_per_day) as f32;
    let floored = e.call(FLOOR_FLOAT, &args![hours]).f32();
    if floored.is_finite() && floored.abs() < 9.223_372e18 {
        floored as i64 as u32
    } else {
        0
    }
}

// Translated from 00867ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The days passed (1.0 without the global) times 24 times 60 (both
/// `double` constants), rounded to a `float`.
pub fn fn_00867ea0(e: &mut Engine, this: Ptr<Calendar>) -> f32 {
    let days_passed = days_passed_value(e, this);
    let hours_per_day: f64 = e.global(HOURS_PER_DAY);
    let sixty: f64 = e.global(SIXTY);
    (days_passed as f64 * hours_per_day * sixty) as f32
}

// Translated from 00867f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The string for the weekday (`00867ef0`): the string value (`00403df0`)
/// of the setting object in the table of seven at `011895b8`, or the
/// address of `"Bad Day"` when the weekday is -1 or at least 7 (the
/// comparisons are signed, and the weekday is always below 7, so the table
/// is always used).
pub fn fn_00867f10(e: &mut Engine, this: Ptr<Calendar>) -> u32 {
    let weekday = fn_00867ef0(e, this) as i32;
    if weekday != -1 && weekday < 7 {
        let setting = e
            .mem
            .u32(DAY_NAME_SETTINGS.wrapping_add((weekday as u32).wrapping_mul(4)));
        e.call(SETTING_STRING, &args![setting]).u32()
    } else {
        BAD_DAY
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00867de0, fn_00867de0(Ptr<Calendar>) -> u32),
        entry!(0x00867e30, fn_00867e30(Ptr<Calendar>) -> u32),
        entry!(0x00867ea0, fn_00867ea0(Ptr<Calendar>) -> f32),
        entry!(0x00867ef0, fn_00867ef0(Ptr<Calendar>) -> u32),
        entry!(0x00867f10, fn_00867f10(Ptr<Calendar>) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A calendar whose days-passed global (a `TESGlobal`-sized block with
    /// its value at +0x24) holds `value`; the getter is a test double.
    fn calendar_with_days_passed(e: &mut Engine, value: f32) -> Ptr<Calendar> {
        e.register(TES_GLOBAL_GET_VALUE, |e, a| {
            e.mem.f32(a[0] + 0x24).into_ret()
        });
        let calendar: Ptr<Calendar> = e.new_object();
        let global = e.mem.alloc(0x28);
        e.mem.set_f32(global + 0x24, value);
        e.set(calendar, Calendar::pGameDaysPassed, Ptr::new(global));
        calendar
    }

    #[test]
    fn whole_days_truncate_toward_zero() {
        let mut e = Engine::new();
        let calendar = calendar_with_days_passed(&mut e, 12.99);
        assert_eq!(e.call(0x0086_7de0, &args![calendar]).u32(), 12);
        let calendar = calendar_with_days_passed(&mut e, -2.5);
        assert_eq!(e.call(0x0086_7de0, &args![calendar]).u32(), -2i32 as u32);
    }

    /// The two constants and the floor routine the other functions use.
    fn constants_and_floor(e: &mut Engine) {
        e.map(HOURS_PER_DAY, 8);
        e.mem.set_f64(HOURS_PER_DAY, 24.0);
        e.map(SIXTY, 8);
        e.mem.set_f64(SIXTY, 60.0);
        e.register(FLOOR_FLOAT, |_, a| f32::from_bits(a[0]).floor().into_ret());
    }

    #[test]
    fn without_a_days_passed_global_the_count_is_one() {
        let mut e = Engine::new();
        let calendar: Ptr<Calendar> = e.new_object();
        assert_eq!(e.call(0x0086_7de0, &args![calendar]).u32(), 1);
        assert_eq!(e.call(0x0086_7ef0, &args![calendar]).u32(), 1);
    }

    #[test]
    fn values_the_integer_cannot_hold_give_zero() {
        let mut e = Engine::new();
        for value in [f32::NAN, f32::INFINITY, 1.0e30] {
            let calendar = calendar_with_days_passed(&mut e, value);
            assert_eq!(e.call(0x0086_7de0, &args![calendar]).u32(), 0);
        }
    }

    #[test]
    fn day_of_week_is_the_whole_days_modulo_seven() {
        let mut e = Engine::new();
        for (days, weekday) in [(0.0f32, 0u32), (6.9, 6), (7.0, 0), (100.5, 2)] {
            let calendar = calendar_with_days_passed(&mut e, days);
            assert_eq!(e.call(0x0086_7ef0, &args![calendar]).u32(), weekday);
        }
        // The count is unsigned in the modulo: -1 is 0xFFFFFFFF, which is 3 mod 7.
        let calendar = calendar_with_days_passed(&mut e, -1.0);
        assert_eq!(e.call(0x0086_7ef0, &args![calendar]).u32(), 3);
    }

    #[test]
    fn days_times_twenty_four_is_floored_then_truncated() {
        let mut e = Engine::new();
        constants_and_floor(&mut e);
        let calendar = calendar_with_days_passed(&mut e, 2.55);
        // 2.55f32 * 24 = 61.2..., floored to 61.
        assert_eq!(e.call(0x0086_7e30, &args![calendar]).u32(), 61);
        let calendar = calendar_with_days_passed(&mut e, -0.5);
        assert_eq!(e.call(0x0086_7e30, &args![calendar]).u32(), -12i32 as u32);
        let calendar = calendar_with_days_passed(&mut e, -0.01);
        // floor(-0.24) is -1.
        assert_eq!(e.call(0x0086_7e30, &args![calendar]).u32(), -1i32 as u32);
    }

    #[test]
    fn hours_without_a_global_use_one_day_and_unrepresentable_values_give_zero() {
        let mut e = Engine::new();
        constants_and_floor(&mut e);
        let calendar: Ptr<Calendar> = e.new_object();
        assert_eq!(e.call(0x0086_7e30, &args![calendar]).u32(), 24);
        for value in [f32::NAN, f32::INFINITY, 1.0e30] {
            let calendar = calendar_with_days_passed(&mut e, value);
            assert_eq!(e.call(0x0086_7e30, &args![calendar]).u32(), 0);
        }
    }

    #[test]
    fn days_times_twenty_four_times_sixty_is_a_float() {
        let mut e = Engine::new();
        constants_and_floor(&mut e);
        let calendar = calendar_with_days_passed(&mut e, 0.5);
        assert_eq!(e.call(0x0086_7ea0, &args![calendar]).f32(), 720.0);
        let calendar: Ptr<Calendar> = e.new_object();
        assert_eq!(e.call(0x0086_7ea0, &args![calendar]).f32(), 1440.0);
        // The result is rounded to a float: 0.1f32 * 1440 in extended
        // precision, stored as f32.
        let calendar = calendar_with_days_passed(&mut e, 0.1);
        let expected = (0.1f32 as f64 * 24.0 * 60.0) as f32;
        assert_eq!(e.call(0x0086_7ea0, &args![calendar]).f32(), expected);
        // A NaN stays one.
        let calendar = calendar_with_days_passed(&mut e, f32::NAN);
        assert!(e.call(0x0086_7ea0, &args![calendar]).f32().is_nan());
    }

    #[test]
    fn weekday_string_comes_from_the_table_of_settings() {
        let mut e = Engine::new();
        e.map(DAY_NAME_SETTINGS, 28);
        for day in 0..7u32 {
            e.mem
                .set_u32(DAY_NAME_SETTINGS + day * 4, 0x0900_0000 + day);
        }
        e.register(SETTING_STRING, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        for (days, setting) in [(0.0f32, 0u32), (3.9, 3), (6.0, 6), (7.0, 0), (13.5, 6)] {
            let calendar = calendar_with_days_passed(&mut e, days);
            assert_eq!(
                e.call(0x0086_7f10, &args![calendar]).u32(),
                0x0900_0100 + setting
            );
        }
    }
}

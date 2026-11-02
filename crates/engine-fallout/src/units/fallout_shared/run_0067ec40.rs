//! `fallout shared/run_0067ec40` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Calendar` singleton (`011de7b8`).
const CALENDAR: u32 = 0x011d_e7b8;
/// `Calendar::GetDay` (`00867d60`, a `char` in `AL`).
const CALENDAR_GET_DAY: u32 = 0x0086_7d60;
/// `Calendar::GetMonth` (`00867d20`).
const CALENDAR_GET_MONTH: u32 = 0x0086_7d20;
/// `Calendar::GetHour` (`00867da0`, a `float` in `ST0`).
const CALENDAR_GET_HOUR: u32 = 0x0086_7da0;
/// A `Calendar` getter (`00867ef0`, unnamed in the engine map) compared
/// with the schedule's value at `00678cc0`.
const CALENDAR_VALUE_00867EF0: u32 = 0x0086_7ef0;
/// Reads the word at +0x68 (`008d8520`; the engine map's name
/// `MiddleHighProcess::GetSavedAcquireObject` belongs to folded code).
const OBJECT_AT_0X68: u32 = 0x008d_8520;
/// `MobileObject::GetCurrentPackage` (`009344a0`).
const MOBILE_OBJECT_GET_CURRENT_PACKAGE: u32 = 0x0093_44a0;
/// A list node's item address (`006815c0`, returns the node itself).
const NODE_ITEM: u32 = 0x0068_15c0;
/// A list node's next node (`00726070`, the word at +4).
const NODE_NEXT: u32 = 0x0072_6070;
/// Whether a list is empty (`008256d0`: both of the head node's words are 0).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// Number of nodes with an item (`005ae380`; the engine map's name
/// `VATS::GetCount` belongs to folded code).
const LIST_COUNT: u32 = 0x005a_e380;
/// Stores `*item` as a node's item when it is not null (`00726c60`).
const NODE_SET_ITEM: u32 = 0x0072_6c60;
/// Package test with the actor (`00677760`, `fallout shared/package.cpp`).
const PACKAGE_TEST_00677760: u32 = 0x0067_7760;
/// The package's schedule (`0041d8a0`, its address +0x38).
const PACKAGE_SCHEDULE: u32 = 0x0041_d8a0;
/// Schedule value tested against the month (`00678ca0`; the engine map
/// names it `PackageLocation::GetLocType`).
const SCHEDULE_VALUE_00678CA0: u32 = 0x0067_8ca0;
/// Schedule value tested against the day (`00678ce0`, a `char`).
const SCHEDULE_VALUE_00678CE0: u32 = 0x0067_8ce0;
/// Schedule value tested against `00867ef0` (`00678cc0`).
const SCHEDULE_VALUE_00678CC0: u32 = 0x0067_8cc0;
/// Schedule hour tested against the hour (`0086a460`, a `char`).
const SCHEDULE_HOUR_0086A460: u32 = 0x0086_a460;
/// Whether the output list already holds the package (`005f65d0`, the list
/// and the address of the package pointer).
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// The package's word at +0x18 (`009611e0`); while it is -1 the function
/// has `CalculateProcedureType` run.
const PACKAGE_PROCEDURE_TYPE: u32 = 0x0096_11e0;
/// `TESObjectREFR` getter (`00569b80`) whose result goes to
/// `CalculateProcedureType`.
const REFERENCE_VALUE_00569B80: u32 = 0x0056_9b80;
/// `TESPackage::CalculateProcedureType` (`006777b0`).
const TES_PACKAGE_CALCULATE_PROCEDURE_TYPE: u32 = 0x0067_77b0;
/// Adds the package to the output list (`00905820`, the list and the
/// address of the package pointer).
const LIST_ADD: u32 = 0x0090_5820;
/// Compares two packages for the sort (`00678a80`, the first package and
/// the second one's schedule; 1 when they are out of order).
const PACKAGE_COMPARE: u32 = 0x0067_8a80;
/// Number of days in a month (`004b10d0`, cdecl, a `u16`).
const DAYS_IN_MONTH: u32 = 0x004b_10d0;
/// `_ftol2` (`00ec62c0`): the `ST0` value as a leading `f64` argument.
const FTOL: u32 = 0x00ec_62c0;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `23.0` (`double`): the last hour of a day.
const LAST_HOUR: u32 = 0x0103_5838;

// Translated from 0067efd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 of the package's flags word (+0x1C) is set.
pub fn fn_0067efd0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr().wrapping_add(0x1c)) & 4 != 0
}

// Translated from 0067ec40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collects the packages of the list `this` that the actor can run from
/// `hour` on into the list `out`, then sorts `out`.
///
/// Starting from the day, month and year that the object at the actor's
/// +0x68 answers (its vtable slots +0x30, +0x34 and +0x38) and `hour`,
/// it steps hour by hour while the day is not 0: at each step every
/// package of `this` (until the first null item) with bit 2 of its flags
/// set, passing `00677760` with the actor, and whose schedule allows the
/// month, day, `00867ef0` value and hour, which is not the actor's
/// current package and not yet in `out`, has `CalculateProcedureType` run when
/// its word at +0x18 is -1 and is added to `out`. An empty `this` ends the
/// stepping. After hour 23 the day advances (carrying into month and year
/// with `004b10d0`); reaching the calendar's day and month at or after
/// its hour ends the stepping too.
///
/// Then `out` is bubble-sorted: up to its item count passes, each swapping
/// the items of a node and of every later node that `00678a80` reports out
/// of order, stopping after a pass without a swap. The third argument is
/// never read.
pub fn fn_0067ec40(e: &mut Engine, this: Ptr, actor: Ptr, out: Ptr, _unused_3: u32, hour: f32) {
    let mut current_package = 0u32;
    let source = e.call(OBJECT_AT_0X68, &args![actor]).u32();
    let mut day = e.vcall(source, 0x30, &args![]).i32();
    let source = e.call(OBJECT_AT_0X68, &args![actor]).u32();
    let mut month = e.vcall(source, 0x34, &args![]).i32();
    let source = e.call(OBJECT_AT_0X68, &args![actor]).u32();
    // The year is carried along but never read.
    let mut _year = e.vcall(source, 0x38, &args![]).i32();
    let mut hour_bits = x87_quiet_bits(hour.to_bits());
    let calendar_day = i32::from(e.call(CALENDAR_GET_DAY, &args![CALENDAR]).u8() as i8);
    let calendar_month = e.call(CALENDAR_GET_MONTH, &args![CALENDAR]).i32();

    e.with_stack(4, |e, package| {
        while day != 0 {
            let mut node = e.call(NODE_ITEM, &args![this]).u32();
            if !actor.is_null() {
                current_package = e
                    .call(MOBILE_OBJECT_GET_CURRENT_PACKAGE, &args![actor])
                    .u32();
            }
            if e.call(LIST_IS_EMPTY, &args![node]).bool() {
                day = 0;
                continue;
            }
            e.mem.set_u32(package.addr(), 0);
            while node != 0 {
                let item_address = e.call(NODE_ITEM, &args![node]).u32();
                let item = e.mem.u32(item_address);
                e.mem.set_u32(package.addr(), item);
                if item == 0 {
                    break;
                }
                let step = Step {
                    actor,
                    out,
                    day,
                    month,
                    hour_bits,
                    current_package,
                    package,
                };
                if package_qualifies(e, item, &step) {
                    if e.call(PACKAGE_PROCEDURE_TYPE, &args![item]).i32() == -1 {
                        let value = e.call(REFERENCE_VALUE_00569B80, &args![actor]).u32();
                        e.call(TES_PACKAGE_CALCULATE_PROCEDURE_TYPE, &args![item, value]);
                    }
                    e.call(LIST_ADD, &args![out, package]);
                }
                node = e.call(NODE_NEXT, &args![node]).u32();
            }

            let one: f64 = e.global(ONE);
            hour_bits = x87_store32(x87_add(x87_load32(hour_bits), one));
            let last_hour: f64 = e.global(LAST_HOUR);
            if x87_load32(hour_bits) > last_hour {
                day = day.wrapping_add(1);
                hour_bits = 0;
                let days = e.call(DAYS_IN_MONTH, &args![i32::from(month as i8)]).u16();
                if day > i32::from(days) {
                    day = day.wrapping_sub(i32::from(days));
                    month = month.wrapping_add(1);
                    if month >= 12 {
                        month = month.wrapping_sub(12);
                        _year = _year.wrapping_add(1);
                    }
                }
            }
            if day == calendar_day && month == calendar_month {
                let step_hour = e.call(FTOL, &args![x87_load32(hour_bits)]).i32();
                let calendar_hour = e.call(CALENDAR_GET_HOUR, &args![CALENDAR]).f64();
                let calendar_hour = e.call(FTOL, &args![calendar_hour]).i32();
                if step_hour >= calendar_hour {
                    day = 0;
                }
            }
        }
    });

    if out.is_null() {
        return;
    }
    let mut outer = out.addr();
    let mut count = e.call(LIST_COUNT, &args![outer]).u32();
    let mut swapped = true;
    e.with_stack(8, |e, pair| {
        let first = pair.addr();
        let second = pair.addr() + 4;
        while count != 0 && swapped {
            swapped = false;
            let mut node = outer;
            while node != 0 {
                let address = e.call(NODE_ITEM, &args![outer]).u32();
                let value = e.mem.u32(address);
                e.mem.set_u32(first, value);
                let address = e.call(NODE_ITEM, &args![node]).u32();
                let value = e.mem.u32(address);
                e.mem.set_u32(second, value);
                let other = e.mem.u32(second);
                let schedule = e.call(PACKAGE_SCHEDULE, &args![other]).u32();
                let package = e.mem.u32(first);
                if e.call(PACKAGE_COMPARE, &args![package, schedule]).i32() == 1 {
                    e.call(NODE_SET_ITEM, &args![outer, second]);
                    e.call(NODE_SET_ITEM, &args![node, first]);
                    swapped = true;
                }
                node = e.call(NODE_NEXT, &args![node]).u32();
            }
            outer = e.call(NODE_NEXT, &args![outer]).u32();
            count = count.wrapping_sub(1);
        }
    });
}

/// What `0067ec40`'s package tests read at one hour step.
struct Step {
    actor: Ptr,
    out: Ptr,
    day: i32,
    month: i32,
    hour_bits: u32,
    current_package: u32,
    /// The stack slot holding the package pointer (passed by address).
    package: Ptr,
}

/// The tests of `0067ec40` a package of `this` passes before it is added to
/// `out`, in the exe's order (each stops at the first failure).
fn package_qualifies(e: &mut Engine, item: u32, step: &Step) -> bool {
    if !fn_0067efd0(e, Ptr::new(item)) {
        return false;
    }
    if !e
        .call(PACKAGE_TEST_00677760, &args![item, step.actor])
        .bool()
    {
        return false;
    }
    let schedule = e.call(PACKAGE_SCHEDULE, &args![item]).u32();
    if schedule == 0 {
        return false;
    }
    if e.call(SCHEDULE_VALUE_00678CA0, &args![schedule]).i32() != -1
        && e.call(SCHEDULE_VALUE_00678CA0, &args![schedule]).i32() > step.month
    {
        return false;
    }
    if e.call(SCHEDULE_VALUE_00678CE0, &args![schedule]).u8() as i8 != 0
        && i32::from(e.call(SCHEDULE_VALUE_00678CE0, &args![schedule]).u8() as i8) > step.day
    {
        return false;
    }
    if e.call(SCHEDULE_VALUE_00678CC0, &args![schedule]).i32() != -1 {
        let value = e.call(SCHEDULE_VALUE_00678CC0, &args![schedule]).i32();
        if value > e.call(CALENDAR_VALUE_00867EF0, &args![CALENDAR]).i32() {
            return false;
        }
    }
    if e.call(SCHEDULE_HOUR_0086A460, &args![schedule]).u8() as i8 != -1 {
        let schedule_hour = e.call(SCHEDULE_HOUR_0086A460, &args![schedule]).u8() as i8;
        // FCOMPP of the hour with the schedule's hour: on only when the hour
        // is greater (not below, equal or unordered).
        if !(x87_load32(step.hour_bits) > f64::from(schedule_hour)) {
            return false;
        }
    }
    if item == step.current_package {
        return false;
    }
    !e.call(LIST_CONTAINS, &args![step.out, step.package]).bool()
}

// Translated from 0067eff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears two parts (`thiscall`, no stack words; the engine map gives it
/// no name): `00401080` (`cdecl`) with a zero byte on +4 and +8.
pub fn fn_0067eff0(e: &mut Engine, this: Ptr) {
    for offset in [4, 8] {
        e.call(0x0040_1080, &args![this.addr() + offset, 0u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0067efd0, fn_0067efd0(Ptr) -> bool),
        entry!(0x0067ec40, fn_0067ec40(Ptr, Ptr, Ptr, u32, f32)),
        entry!(0x0067_eff0, fn_0067eff0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn word(e: &Engine, a: u32) -> Ret {
        e.mem.u32(a).into_ret()
    }

    /// List, schedule and calendar doubles shaped after the exe's helpers.
    fn setup(day: i32, month: i32, calendar: (i8, i32, f32)) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(ONE, 8);
        e.set_global(ONE, 1.0f64);
        e.map(LAST_HOUR, 8);
        e.set_global(LAST_HOUR, 23.0f64);
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| word(e, a[0] + 4));
        e.register(LIST_IS_EMPTY, |e, a| {
            (e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_COUNT, |e, a| {
            let mut n = 0u32;
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) != 0 {
                    n += 1;
                }
                node = e.mem.u32(node + 4);
            }
            n.into_ret()
        });
        e.register(NODE_SET_ITEM, |e, a| {
            let item = e.mem.u32(a[1]);
            if item != 0 {
                e.mem.set_u32(a[0], item);
            }
            Ret::default()
        });
        e.register(PACKAGE_SCHEDULE, |_, a| (a[0] + 0x38).into_ret());
        e.register(SCHEDULE_VALUE_00678CA0, |e, a| word(e, a[0]));
        e.register(SCHEDULE_VALUE_00678CE0, |e, a| word(e, a[0] + 4));
        e.register(SCHEDULE_VALUE_00678CC0, |e, a| word(e, a[0] + 8));
        e.register(SCHEDULE_HOUR_0086A460, |e, a| word(e, a[0] + 12));
        e.register(CALENDAR_VALUE_00867EF0, |_, _| 10u32.into_ret());
        e.register(PACKAGE_TEST_00677760, |_, _| true.into_ret());
        e.register(PACKAGE_PROCEDURE_TYPE, |e, a| word(e, a[0] + 0x30));
        e.register(REFERENCE_VALUE_00569B80, |_, _| 0x77u32.into_ret());
        e.register(TES_PACKAGE_CALCULATE_PROCEDURE_TYPE, |_, _| Ret::default());
        e.register(LIST_CONTAINS, |_, _| false.into_ret());
        e.register(LIST_ADD, |_, _| Ret::default());
        e.register(MOBILE_OBJECT_GET_CURRENT_PACKAGE, |e, a| {
            word(e, a[0] + 0x10)
        });
        e.register(DAYS_IN_MONTH, |_, _| 31u16.into_ret());
        e.register(FTOL, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (v as i32).into_ret()
        });
        e.register(PACKAGE_COMPARE, |e, a| word(e, a[0] + 0x40));
        let (calendar_day, calendar_month, calendar_hour) = calendar;
        e.register_double(CALENDAR_GET_DAY, move |_, _| {
            (calendar_day as u8).into_ret()
        });
        e.register_double(CALENDAR_GET_MONTH, move |_, _| calendar_month.into_ret());
        e.register_double(CALENDAR_GET_HOUR, move |_, _| calendar_hour.into_ret());
        // The actor: its +0x68 object answers the day, month and year.
        let actor = e.mem.alloc(0x80);
        let source = e.mem.alloc(4);
        e.mem.set_u32(actor + 0x68, source);
        e.put_vtable(0x0300_0000, &[0; 0x0f]);
        e.mem.set_u32(0x0300_0030, 0x0300_1000);
        e.mem.set_u32(0x0300_0034, 0x0300_1004);
        e.mem.set_u32(0x0300_0038, 0x0300_1008);
        e.mem.set_u32(source, 0x0300_0000);
        e.register_double(0x0300_1000, move |_, _| day.into_ret());
        e.register_double(0x0300_1004, move |_, _| month.into_ret());
        e.register(0x0300_1008, |_, _| 2281u32.into_ret());
        e.register(OBJECT_AT_0X68, |e, a| word(e, a[0] + 0x68));
        (e, actor)
    }

    fn node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let n = e.mem.alloc(8);
        e.mem.set_u32(n, item);
        e.mem.set_u32(n + 4, next);
        n
    }

    /// A package with flag bit 2 as given, a schedule allowing everything
    /// (month -1, day 0, value -1, hour -1) and procedure type `procedure`.
    fn package(e: &mut Engine, flagged: bool, procedure: i32) -> u32 {
        let p = e.mem.alloc(0x50);
        e.mem.set_u32(p + 0x1c, if flagged { 4 } else { 0 });
        e.mem.set_i32(p + 0x38, -1);
        e.mem.set_u32(p + 0x3c, 0);
        e.mem.set_i32(p + 0x40, -1);
        e.mem.set_u32(p + 0x44, 0xff);
        e.mem.set_i32(p + 0x30, procedure);
        p
    }

    #[test]
    fn package_flag_bit_2_is_read_from_the_flags_word() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x20);
        assert!(!e.call(0x0067_efd0, &args![p]).bool());
        e.mem.set_u32(p + 0x1c, 4);
        assert!(e.call(0x0067_efd0, &args![p]).bool());
        e.mem.set_u32(p + 0x1c, !4u32);
        assert!(!e.call(0x0067_efd0, &args![p]).bool());
    }

    #[test]
    fn collects_the_runnable_packages_then_sorts_the_output() {
        let (mut e, actor) = setup(5, 3, (5, 3, 23.5));
        let runnable = package(&mut e, true, -1);
        let unflagged = package(&mut e, false, 1);
        let current = package(&mut e, true, 1);
        e.mem.set_u32(actor + 0x10, current);
        let late = package(&mut e, true, 1);
        e.mem.set_u32(late + 0x44, 22);
        let late_month = package(&mut e, true, 1);
        e.mem.set_i32(late_month + 0x38, 4);
        let late_day = package(&mut e, true, 1);
        e.mem.set_u32(late_day + 0x3c, 6);
        let late_value = package(&mut e, true, 1);
        e.mem.set_i32(late_value + 0x40, 11);
        let after_null = package(&mut e, true, 1);
        let mut list = node(&mut e, after_null, 0);
        list = node(&mut e, 0, list);
        for p in [
            late_value, late_day, late_month, late, current, unflagged, runnable,
        ] {
            list = node(&mut e, p, list);
        }
        // The output list already holds A then B, out of order.
        let a = package(&mut e, true, 1);
        let b = package(&mut e, true, 1);
        let second = node(&mut e, b, 0);
        let out = node(&mut e, a, second);
        e.register_double(PACKAGE_COMPARE, move |_, w| {
            u32::from(w[0] == a && w[1] == b + 0x38).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0067_ec40, &args![list, actor, out, 0u32, 22.0f32]);
        assert_eq!(
            calls_to(&e, TES_PACKAGE_CALCULATE_PROCEDURE_TYPE),
            vec![vec![runnable, 0x77]]
        );
        let added = calls_to(&e, LIST_ADD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], out);
        // One hour step: 23 is not after hour 23, and the calendar's hour
        // (23.5, truncated) is reached.
        assert_eq!(calls_to(&e, CALENDAR_GET_HOUR).len(), 1);
        assert!(calls_to(&e, DAYS_IN_MONTH).is_empty());
        assert_eq!(e.mem.u32(out), b);
        assert_eq!(e.mem.u32(second), a);
        assert_eq!(calls_to(&e, PACKAGE_COMPARE).len(), 3);
    }

    #[test]
    fn steps_past_midnight_carry_into_the_next_year() {
        let (mut e, actor) = setup(31, 11, (1, 0, 0.0));
        let tail = node(&mut e, 0, 0);
        let list = node(&mut e, 0, tail);
        e.call_log = Some(vec![]);
        e.call(0x0067_ec40, &args![list, actor, 0u32, 0u32, 23.0f32]);
        assert_eq!(calls_to(&e, DAYS_IN_MONTH), vec![vec![11]]);
        let hours = calls_to(&e, FTOL);
        assert_eq!(hours.len(), 2);
        assert_eq!(hours[0], args![0.0f64]);
        assert!(calls_to(&e, LIST_COUNT).is_empty());
    }

    #[test]
    fn an_empty_list_or_a_zero_day_does_nothing() {
        let (mut e, actor) = setup(4, 1, (9, 9, 0.0));
        let list = node(&mut e, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x0067_ec40, &args![list, actor, 0u32, 0u32, 1.0f32]);
        assert_eq!(calls_to(&e, LIST_IS_EMPTY).len(), 1);
        assert!(calls_to(&e, FTOL).is_empty());
        let (mut e, actor) = setup(0, 1, (9, 9, 0.0));
        e.call_log = Some(vec![]);
        e.call(0x0067_ec40, &args![list, actor, 0u32, 0u32, 1.0f32]);
        assert!(calls_to(&e, LIST_IS_EMPTY).is_empty());
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    #[test]
    fn clears_two_parts() {
        let mut e = Engine::new();
        e.register(0x0040_1080, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0067_eff0, &args![0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0040_1080, vec![0x5004, 0]),
                (0x0040_1080, vec![0x5008, 0])
            ]
        );
    }
}

//! `BSCore/bstimer.cpp` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `Sleep` (KERNEL32 import slot).
const SLEEP_IMPORT: u32 = 0x00fd_f104;
/// `0.0` (`double`), the value `fClamp` is compared with.
const ZERO: u32 = 0x0101_2060;
/// `0.001` (`double`): milliseconds to seconds.
const MILLISECONDS_TO_SECONDS: u32 = 0x0101_6978;
/// `BSTimer::fGlobalTimeMultiplier` (Xbox PDB static `float`).
const GLOBAL_TIME_MULTIPLIER: u32 = 0x011a_c3a0;

layout! {
    /// `BSTimer` (Xbox PDB), 0x20 bytes.
    pub struct BSTimer: 0x20 {
        /// `cDisableCounter` (Xbox PDB): non-zero while the timer is disabled.
        0x00 cDisableCounter: u8,
        /// `fClamp` (Xbox PDB): fixed step in milliseconds, 0 for real time.
        0x04 fClamp: f32,
        /// `fClampRemainder` (Xbox PDB).
        0x08 fClampRemainder: f32,
        /// `fDelta` (Xbox PDB): seconds of the last step, times the global multiplier.
        0x0C fDelta: f32,
        /// `fDeltaUnmodified` (Xbox PDB): seconds of the last step.
        0x10 fDeltaUnmodified: f32,
        /// `iLastTime` (Xbox PDB).
        0x14 iLastTime: u32,
        /// `iFirstTime` (Xbox PDB).
        0x18 iFirstTime: u32,
        /// `bZeroDelta` (Xbox PDB): set when no time passed since the last step.
        0x1D bZeroDelta: bool,
    }
}

/// `FISTP qword` with the control word set to truncate (`OR 0xC00`): the
/// integer indefinite `i64::MIN` for NaN and values out of range.
fn truncate_to_i64(value: f64) -> i64 {
    if value.is_nan()
        || value >= 9_223_372_036_854_775_808.0
        || value < -9_223_372_036_854_775_808.0
    {
        i64::MIN
    } else {
        value as i64
    }
}

// Translated from 00aa4ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTimer` step (the engine map gives no name; the body is the timer's
/// per-frame update, `now` being the current millisecond tick): computes
/// `fDelta` from the time since `iLastTime`.
///
/// Disabled timer: `fDelta` is 0. With `fClamp` 0 (real time) the elapsed
/// milliseconds (1 and `bZeroDelta` set when none passed, otherwise at most
/// 166) are lifted to at least 10 by sleeping the difference. With a
/// non-zero `fClamp` the clamp is added to the remainder, whole milliseconds
/// move `iLastTime` and the fraction stays in `fClampRemainder`. The step in
/// seconds goes to `fDeltaUnmodified`, and `fDelta` is that times
/// `fGlobalTimeMultiplier`.
pub fn fn_00aa4ee0(e: &mut Engine, this: Ptr<BSTimer>, now: u32) {
    if e.get(this, BSTimer::cDisableCounter) != 0 {
        e.set(this, BSTimer::fDelta, 0.0);
        return;
    }
    let milliseconds_to_seconds: f64 = e.global(MILLISECONDS_TO_SECONDS);
    let mut time = now.wrapping_sub(e.get(this, BSTimer::iFirstTime));
    let clamp = e.get(this, BSTimer::fClamp);
    let zero: f64 = e.global(ZERO);
    if f64::from(clamp) == zero {
        let mut elapsed = time.wrapping_sub(e.get(this, BSTimer::iLastTime));
        if elapsed == 0 {
            e.set(this, BSTimer::bZeroDelta, true);
            elapsed = 1;
        } else {
            e.set(this, BSTimer::bZeroDelta, false);
            if elapsed > 0xa6 {
                elapsed = 0xa6;
            }
        }
        if elapsed < 10 {
            e.call(SLEEP_IMPORT, &args![10 - elapsed]);
            elapsed = 10;
        }
        let seconds = x87_mul(f64::from(elapsed), milliseconds_to_seconds);
        e.set(this, BSTimer::fDelta, f32::from_bits(x87_store32(seconds)));
    } else {
        let remainder = e.get(this, BSTimer::fClampRemainder);
        let sum = x87_add(x87_load32(remainder.to_bits()), x87_m32(clamp.to_bits()));
        e.set(
            this,
            BSTimer::fClampRemainder,
            f32::from_bits(x87_store32(sum)),
        );
        let remainder = e.get(this, BSTimer::fClampRemainder);
        // `FISTP qword` truncating; `FILD` of the low word alone (the high
        // word is stored as 0), then `FSUBR`.
        let whole = truncate_to_i64(x87_load32(remainder.to_bits())) as u32;
        let rest = x87_sub(
            x87_m32(e.get(this, BSTimer::fClampRemainder).to_bits()),
            f64::from(whole),
        );
        e.set(
            this,
            BSTimer::fClampRemainder,
            f32::from_bits(x87_store32(rest)),
        );
        time = e.get(this, BSTimer::iLastTime).wrapping_add(whole);
        let seconds = x87_mul(x87_load32(clamp.to_bits()), milliseconds_to_seconds);
        e.set(this, BSTimer::fDelta, f32::from_bits(x87_store32(seconds)));
    }
    e.set(this, BSTimer::iLastTime, time);
    let delta = e.get(this, BSTimer::fDelta);
    e.set(
        this,
        BSTimer::fDeltaUnmodified,
        f32::from_bits(x87_quiet_bits(delta.to_bits())),
    );
    let multiplier: u32 = e.global(GLOBAL_TIME_MULTIPLIER);
    let scaled = x87_mul(x87_load32(delta.to_bits()), x87_m32(multiplier));
    e.set(this, BSTimer::fDelta, f32::from_bits(x87_store32(scaled)));
}

// Translated from 00aa50a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTimer::Disable` (engine map; `ECX` only): counts the disable counter up (a byte,
/// wrapping); when it was zero, `iLastTime` and `iFirstTime` become `GetTickCount()` minus
/// themselves (saving the times relative to now, which `Enable` turns back).
pub fn bs_timer_disable(e: &mut Engine, this: Ptr<BSTimer>) {
    let counter = e.get(this, BSTimer::cDisableCounter);
    e.set(this, BSTimer::cDisableCounter, counter.wrapping_add(1));
    if counter != 0 {
        return;
    }
    let now = e.call(GET_TICK_COUNT_IMPORT, &args![]).u32();
    let last = now.wrapping_sub(e.get(this, BSTimer::iLastTime));
    e.set(this, BSTimer::iLastTime, last);
    let first = now.wrapping_sub(e.get(this, BSTimer::iFirstTime));
    e.set(this, BSTimer::iFirstTime, first);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_50a0, bs_timer_disable(Ptr<BSTimer>)),
        entry!(0x00aa4ee0, fn_00aa4ee0(Ptr<BSTimer>, u32)),
        entry!(
            0x00aa_4db0,
            bs_timer_set_global_time_multiplier(Ptr<BSTimer>, u32, u32)
        ),
        entry!(0x00aa_4e40, fn_00aa4e40()),
        entry!(0x00aa_5040, bs_timer_enable(Ptr<BSTimer>)),
    ]
}

/// The float `SetGlobalTimeMultiplier` always stores the new value in and `00aa4e40` moves
/// `fGlobalTimeMultiplier` towards (`011ac3a4`; by its use the Xbox PDB static
/// `fGlobalTimeMultiplierTarget`, the address not confirmed by a name), and the float rate
/// of that move (`011ac3ac`).
const GLOBAL_TIME_MULTIPLIER_TARGET: u32 = 0x011a_c3a4;
const GLOBAL_TIME_MULTIPLIER_RATE: u32 = 0x011a_c3ac;
/// `GetTickCount` (KERNEL32 import slot).
const GET_TICK_COUNT_IMPORT: u32 = 0x00fd_f060;
/// `bUseGlobalTimeMultiplierTarget` (Xbox PDB, `+0x1c`; not in the layout above).
const USE_TARGET: u32 = 0x1c;

// Translated from 00aa4db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTimer::SetGlobalTimeMultiplier` (Xbox PDB; `thiscall`, `RET 8`: the float `value`
/// as its bits and a byte `immediate`): the target (`011ac3a4`) becomes `value`. With
/// `bUseGlobalTimeMultiplierTarget` set, `fGlobalTimeMultiplier` follows only when
/// `immediate` is set. Without it, when the multiplier differs from `value` (`FUCOMPP`,
/// unordered counts as different) `fDelta` is divided by the old multiplier, the
/// multiplier becomes `value` and `fDelta` is multiplied by it. Every copy goes through the
/// x87 stack.
pub fn bs_timer_set_global_time_multiplier(
    e: &mut Engine,
    this: Ptr<BSTimer>,
    value: u32,
    immediate: u32,
) {
    e.mem
        .set_u32(GLOBAL_TIME_MULTIPLIER_TARGET, x87_quiet_bits(value));
    if e.mem.u8(this.addr() + USE_TARGET) != 0 {
        if immediate & 0xff != 0 {
            let target = x87_quiet_bits(e.mem.u32(GLOBAL_TIME_MULTIPLIER_TARGET));
            e.mem.set_u32(GLOBAL_TIME_MULTIPLIER, target);
        }
        return;
    }
    let multiplier = x87_load32(e.mem.u32(GLOBAL_TIME_MULTIPLIER));
    if multiplier == x87_load32(value) {
        return;
    }
    let delta = x87_load32(e.get(this, BSTimer::fDelta).to_bits());
    let divided = x87_div(delta, x87_m32(e.mem.u32(GLOBAL_TIME_MULTIPLIER)));
    e.set(this, BSTimer::fDelta, f32::from_bits(x87_store32(divided)));
    e.mem.set_u32(GLOBAL_TIME_MULTIPLIER, x87_quiet_bits(value));
    let delta = x87_load32(e.get(this, BSTimer::fDelta).to_bits());
    let product = x87_mul(delta, x87_m32(e.mem.u32(GLOBAL_TIME_MULTIPLIER)));
    e.set(this, BSTimer::fDelta, f32::from_bits(x87_store32(product)));
}

// Translated from 00aa4e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `bstimer.cpp`; no argument): moves
/// `fGlobalTimeMultiplier` towards the target at `011ac3a4` by the rate at `011ac3ac`: below
/// the target it is multiplied by `rate + 1` and capped at the target; above it (ordered)
/// by `1 - rate` and raised back to the target when it falls below. Equal or unordered:
/// unchanged.
pub fn fn_00aa4e40(e: &mut Engine) {
    let target = |e: &Engine| x87_load32(e.mem.u32(GLOBAL_TIME_MULTIPLIER_TARGET));
    let multiplier = |e: &Engine| x87_load32(e.mem.u32(GLOBAL_TIME_MULTIPLIER));
    let rate = x87_load32(e.mem.u32(GLOBAL_TIME_MULTIPLIER_RATE));
    // FLD mult; FLD target; FCOMPP; TEST AH,0x41; JNZ: falls through when target > mult.
    if target(e) > multiplier(e) {
        let factor = x87_add(rate, 1.0);
        let grown = x87_mul(factor, x87_m32(e.mem.u32(GLOBAL_TIME_MULTIPLIER)));
        e.mem.set_u32(GLOBAL_TIME_MULTIPLIER, x87_store32(grown));
        // TEST AH,5; JP: capped when target < mult.
        if target(e) < multiplier(e) {
            let capped = x87_quiet_bits(e.mem.u32(GLOBAL_TIME_MULTIPLIER_TARGET));
            e.mem.set_u32(GLOBAL_TIME_MULTIPLIER, capped);
        }
        return;
    }
    // TEST AH,5; JP: only target < mult goes on.
    if !(target(e) < multiplier(e)) {
        return;
    }
    // FLD rate; FLD1; FSUBRP (DE E1): 1 - rate.
    let factor = x87_sub(1.0, rate);
    let shrunk = x87_mul(factor, x87_m32(e.mem.u32(GLOBAL_TIME_MULTIPLIER)));
    e.mem.set_u32(GLOBAL_TIME_MULTIPLIER, x87_store32(shrunk));
    // TEST AH,0x41; JNZ: raised to the target when target > mult.
    if target(e) > multiplier(e) {
        let raised = x87_quiet_bits(e.mem.u32(GLOBAL_TIME_MULTIPLIER_TARGET));
        e.mem.set_u32(GLOBAL_TIME_MULTIPLIER, raised);
    }
}

// Translated from 00aa5040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTimer::Enable` (Xbox PDB; `ECX` only): with a non-zero disable counter, counts it
/// down; when it reaches zero, `iLastTime` and `iFirstTime` become `GetTickCount()` minus
/// themselves (turning the times saved by `Disable` back into tick values).
pub fn bs_timer_enable(e: &mut Engine, this: Ptr<BSTimer>) {
    let counter = e.get(this, BSTimer::cDisableCounter);
    if counter == 0 {
        return;
    }
    e.set(this, BSTimer::cDisableCounter, counter.wrapping_sub(1));
    if e.get(this, BSTimer::cDisableCounter) != 0 {
        return;
    }
    let now = e.call(GET_TICK_COUNT_IMPORT, &args![]).u32();
    let last = now.wrapping_sub(e.get(this, BSTimer::iLastTime));
    e.set(this, BSTimer::iLastTime, last);
    let first = now.wrapping_sub(e.get(this, BSTimer::iFirstTime));
    e.set(this, BSTimer::iFirstTime, first);
}

#[cfg(test)]
mod b012_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011a_c000, 0x1000);
        e.register(GET_TICK_COUNT_IMPORT, |_, _| 5000u32.into_ret());
        e
    }

    #[test]
    fn setting_the_multiplier_rescales_the_delta() {
        let mut e = engine();
        let this: Ptr<BSTimer> = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_f32(GLOBAL_TIME_MULTIPLIER, 2.0);
        e.set(this, BSTimer::fDelta, 0.5);
        e.call(0x00aa_4db0, &args![this, 3.0f32.to_bits(), 0u32]);
        assert_eq!(e.mem.f32(GLOBAL_TIME_MULTIPLIER_TARGET), 3.0);
        assert_eq!(e.mem.f32(GLOBAL_TIME_MULTIPLIER), 3.0);
        assert_eq!(e.get(this, BSTimer::fDelta), 0.75);
        // The same value: nothing changes.
        e.call(0x00aa_4db0, &args![this, 3.0f32.to_bits(), 0u32]);
        assert_eq!(e.get(this, BSTimer::fDelta), 0.75);
    }

    #[test]
    fn with_a_target_only_an_immediate_call_sets_the_multiplier() {
        let mut e = engine();
        let this: Ptr<BSTimer> = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u8(this.addr() + USE_TARGET, 1);
        e.mem.set_f32(GLOBAL_TIME_MULTIPLIER, 1.0);
        e.call(0x00aa_4db0, &args![this, 0.5f32.to_bits(), 0x100u32]);
        assert_eq!(e.mem.f32(GLOBAL_TIME_MULTIPLIER_TARGET), 0.5);
        assert_eq!(e.mem.f32(GLOBAL_TIME_MULTIPLIER), 1.0);
        e.call(0x00aa_4db0, &args![this, 0.25f32.to_bits(), 1u32]);
        assert_eq!(e.mem.f32(GLOBAL_TIME_MULTIPLIER), 0.25);
    }

    #[test]
    fn the_multiplier_moves_towards_the_target() {
        let mut e = engine();
        e.mem.set_f32(GLOBAL_TIME_MULTIPLIER_RATE, 0.5);
        // Below: times 1.5, capped.
        e.mem.set_f32(GLOBAL_TIME_MULTIPLIER, 1.0);
        e.mem.set_f32(GLOBAL_TIME_MULTIPLIER_TARGET, 2.0);
        e.call(0x00aa_4e40, &args![]);
        assert_eq!(e.mem.f32(GLOBAL_TIME_MULTIPLIER), 1.5);
        e.call(0x00aa_4e40, &args![]);
        assert_eq!(e.mem.f32(GLOBAL_TIME_MULTIPLIER), 2.0);
        // Above: times 0.5, raised back to the target.
        e.mem.set_f32(GLOBAL_TIME_MULTIPLIER, 4.0);
        e.mem.set_f32(GLOBAL_TIME_MULTIPLIER_TARGET, 1.0);
        e.call(0x00aa_4e40, &args![]);
        assert_eq!(e.mem.f32(GLOBAL_TIME_MULTIPLIER), 2.0);
        e.mem.set_f32(GLOBAL_TIME_MULTIPLIER_TARGET, 1.5);
        e.call(0x00aa_4e40, &args![]);
        assert_eq!(e.mem.f32(GLOBAL_TIME_MULTIPLIER), 1.5);
        // Equal or NaN: unchanged.
        e.call(0x00aa_4e40, &args![]);
        assert_eq!(e.mem.f32(GLOBAL_TIME_MULTIPLIER), 1.5);
        e.mem.set_f32(GLOBAL_TIME_MULTIPLIER_TARGET, f32::NAN);
        e.call(0x00aa_4e40, &args![]);
        assert_eq!(e.mem.f32(GLOBAL_TIME_MULTIPLIER), 1.5);
    }

    #[test]
    fn enabling_restores_the_tick_values_at_zero() {
        let mut e = engine();
        let this: Ptr<BSTimer> = Ptr::new(e.mem.alloc(0x20));
        e.set(this, BSTimer::cDisableCounter, 2);
        e.set(this, BSTimer::iLastTime, 100);
        e.set(this, BSTimer::iFirstTime, 300);
        e.call(0x00aa_5040, &args![this]);
        assert_eq!(e.get(this, BSTimer::cDisableCounter), 1);
        assert_eq!(e.get(this, BSTimer::iLastTime), 100);
        e.call(0x00aa_5040, &args![this]);
        assert_eq!(e.get(this, BSTimer::cDisableCounter), 0);
        assert_eq!(e.get(this, BSTimer::iLastTime), 4900);
        assert_eq!(e.get(this, BSTimer::iFirstTime), 4700);
        // Already enabled: nothing.
        e.call(0x00aa_5040, &args![this]);
        assert_eq!(e.get(this, BSTimer::iLastTime), 4900);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn timer_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.map(0x011a_c000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        e.set_global(MILLISECONDS_TO_SECONDS, 0.001f64);
        e.set_global(GLOBAL_TIME_MULTIPLIER, 2.0f32);
        e
    }

    fn sleeps(e: &mut Engine) -> std::rc::Rc<std::cell::RefCell<Vec<u32>>> {
        let log = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = log.clone();
        e.register_double(SLEEP_IMPORT, move |_, a| {
            seen.borrow_mut().push(a[0]);
            Ret::default()
        });
        log
    }

    #[test]
    fn a_disabled_timer_has_zero_delta() {
        let mut e = timer_engine();
        let timer: Ptr<BSTimer> = e.new_object();
        e.set(timer, BSTimer::cDisableCounter, 1);
        e.set(timer, BSTimer::fDelta, 5.0);
        e.set(timer, BSTimer::iLastTime, 77);
        e.call(0x00aa_4ee0, &args![timer, 1000u32]);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.0);
        assert_eq!(e.get(timer, BSTimer::iLastTime), 77);
    }

    #[test]
    fn real_time_with_no_elapsed_time_sleeps_ten_milliseconds() {
        let mut e = timer_engine();
        let log = sleeps(&mut e);
        let timer: Ptr<BSTimer> = e.new_object();
        e.set(timer, BSTimer::iFirstTime, 100);
        e.set(timer, BSTimer::iLastTime, 50);
        e.call(0x00aa_4ee0, &args![timer, 150u32]);
        assert!(e.get(timer, BSTimer::bZeroDelta));
        // One millisecond elapsed is lifted to ten by sleeping nine.
        assert_eq!(*log.borrow(), vec![9]);
        assert_eq!(e.get(timer, BSTimer::iLastTime), 50);
        assert_eq!(e.get(timer, BSTimer::fDeltaUnmodified), 0.01);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.02);
    }

    #[test]
    fn real_time_caps_a_long_step_and_does_not_sleep() {
        let mut e = timer_engine();
        let log = sleeps(&mut e);
        let timer: Ptr<BSTimer> = e.new_object();
        e.set(timer, BSTimer::bZeroDelta, true);
        e.call(0x00aa_4ee0, &args![timer, 5000u32]);
        assert!(!e.get(timer, BSTimer::bZeroDelta));
        assert!(log.borrow().is_empty());
        assert_eq!(e.get(timer, BSTimer::iLastTime), 5000);
        assert_eq!(e.get(timer, BSTimer::fDeltaUnmodified), 0.166);
    }

    #[test]
    fn real_time_keeps_a_normal_step() {
        let mut e = timer_engine();
        let log = sleeps(&mut e);
        let timer: Ptr<BSTimer> = e.new_object();
        e.set(timer, BSTimer::iLastTime, 1000);
        e.call(0x00aa_4ee0, &args![timer, 1020u32]);
        assert!(log.borrow().is_empty());
        assert!(!e.get(timer, BSTimer::bZeroDelta));
        assert_eq!(e.get(timer, BSTimer::iLastTime), 1020);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.04);
    }

    #[test]
    fn a_fixed_step_carries_the_fraction() {
        let mut e = timer_engine();
        let timer: Ptr<BSTimer> = e.new_object();
        e.set(timer, BSTimer::fClamp, 16.5);
        e.set(timer, BSTimer::fClampRemainder, 0.75);
        e.set(timer, BSTimer::iLastTime, 1000);
        e.call(0x00aa_4ee0, &args![timer, 9999u32]);
        // 0.75 + 16.5 = 17.25: 17 whole milliseconds, 0.25 left.
        assert_eq!(e.get(timer, BSTimer::iLastTime), 1017);
        assert_eq!(e.get(timer, BSTimer::fClampRemainder), 0.25);
        assert_eq!(e.get(timer, BSTimer::fDeltaUnmodified), 0.0165);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.033);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn disable_saves_the_times_on_the_first_count_only() {
        let mut e = Engine::new();
        e.register(GET_TICK_COUNT_IMPORT, |_, _| 1000u32.into_ret());
        let timer: Ptr<BSTimer> = e.new_object();
        e.set(timer, BSTimer::iLastTime, 900);
        e.set(timer, BSTimer::iFirstTime, 100);
        e.call(0x00aa_50a0, &args![timer]);
        assert_eq!(e.get(timer, BSTimer::cDisableCounter), 1);
        assert_eq!(e.get(timer, BSTimer::iLastTime), 100);
        assert_eq!(e.get(timer, BSTimer::iFirstTime), 900);
        // A second disable only counts.
        e.call_log = Some(vec![]);
        e.call(0x00aa_50a0, &args![timer]);
        assert_eq!(e.get(timer, BSTimer::cDisableCounter), 2);
        assert_eq!(e.get(timer, BSTimer::iLastTime), 100);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(a, _)| *a != GET_TICK_COUNT_IMPORT));
        // The counter wraps from 0xFF to 0 without reading the clock.
        e.set(timer, BSTimer::cDisableCounter, 0xff);
        e.call(0x00aa_50a0, &args![timer]);
        assert_eq!(e.get(timer, BSTimer::cDisableCounter), 0);
        assert_eq!(e.get(timer, BSTimer::iLastTime), 100);
        // Later than the 32-bit wrap: now minus a larger saved time.
        let timer: Ptr<BSTimer> = e.new_object();
        e.set(timer, BSTimer::iLastTime, 2000);
        e.call(0x00aa_50a0, &args![timer]);
        assert_eq!(e.get(timer, BSTimer::iLastTime), 1000u32.wrapping_sub(2000));
    }
}

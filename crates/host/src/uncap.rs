//! `NV_UNCAP=1`: an opt-in fix that is **not 1:1 with FalloutNV.exe** (off by default; nothing
//! here runs unless it is asked for, and the default path never reaches this file).
//!
//! The game paces its frames with `BSTimer`'s step (`00aa4ee0`, engine-bs `bscore/bstimer.rs`):
//! for a real-time timer (`fClamp` 0) a step that comes less than 10 ms (by `GetTickCount`)
//! after the timer's last one sleeps the difference, and every step reports at least 10 ms.
//! In game three timers step each frame (Main::OnIdle's through `00771520`, `HUDMainMenu::
//! DoIdle`'s and `MessageMenu::DoIdle`'s), so with W the frame's other work the frame takes
//! (W + 30) / 4 ms: at most 133 frames a second however fast the rest is (a29's profile,
//! 2026-10-28). The original game is held the same way.
//!
//! With the switch, the step at `00aa4ee0` is replaced (registered over the translation by
//! [`install`], after the play-loop patches): a real-time step neither sleeps nor rounds up to
//! 10 ms. It takes the time since that timer's own last step from the platform's microsecond
//! clock (what `QueryPerformanceCounter` answers), not from `GetTickCount`'s 15.625 ms ticks,
//! so the game's time keeps pace with the real time however short the frames are. The rest is
//! the translation's: a disabled timer and a fixed step (`fClamp`) run the translation itself;
//! the step is at most 166 ms; `iLastTime` moves on as before; `fDeltaUnmodified` and the global
//! time multiplier as before. A timer's first step (or its first after being disabled or
//! clamped) takes the tick count's milliseconds, as the translation does, without the sleep.
//!
//! The game was made for steps of 10 ms and more; shorter ones are what mods like NVTF give it.
//!
//! Settings: the environment for now. A settings file, when there is one, sets the same switch
//! (proposal: `nv-rs.ini` next to the viewer, section `[Fixes]`, `bUncappedFrameRate=1`; see
//! docs/PERFORMANCE.md, "Opt-in fixes").

use engine::prelude::*;
use engine::units::bscore::bstimer::{fn_00aa4ee0, BSTimer};
use std::collections::HashMap;

/// `BSTimer`'s step (the translation: engine-bs `bstimer.rs`).
pub const BS_TIMER_STEP: u32 = 0x00aa_4ee0;
/// `0.0` (`double`), the value the translation compares `fClamp` with.
const ZERO: u32 = 0x0101_2060;
/// `BSTimer::fGlobalTimeMultiplier` (Xbox PDB static `float`).
const GLOBAL_TIME_MULTIPLIER: u32 = 0x011a_c3a0;
/// The longest step the translation takes (`0xa6` ms), in microseconds.
const LONGEST_STEP_US: u64 = 166_000;

/// Whether the switch is on (`NV_UNCAP=1`).
pub fn wanted() -> bool {
    std::env::var("NV_UNCAP").is_ok_and(|v| v.trim() == "1")
}

/// Registers the uncapped step over `00aa4ee0`.
pub fn install(e: &mut Engine) {
    let mut last_step_us: HashMap<u32, u64> = HashMap::new();
    e.register_double(BS_TIMER_STEP, move |e, a| {
        uncapped_step(e, Ptr::new(a[0]), a[1], &mut last_step_us);
        Ret::default()
    });
}

/// `00aa4ee0` without the sleep (module documentation); `last_step_us` keeps each timer's last
/// real-time step on the microsecond clock.
pub fn uncapped_step(
    e: &mut Engine,
    this: Ptr<BSTimer>,
    now: u32,
    last_step_us: &mut HashMap<u32, u64>,
) {
    let zero: f64 = e.global(ZERO);
    let real_time = e.get(this, BSTimer::cDisableCounter) == 0
        && f64::from(e.get(this, BSTimer::fClamp)) == zero;
    if !real_time {
        last_step_us.remove(&this.addr());
        fn_00aa4ee0(e, this, now);
        return;
    }
    let now_us = e.clock_now_us();
    let time = now.wrapping_sub(e.get(this, BSTimer::iFirstTime));
    let elapsed_ms = time.wrapping_sub(e.get(this, BSTimer::iLastTime));
    let elapsed_us = match last_step_us.insert(this.addr(), now_us) {
        Some(before) => now_us.saturating_sub(before),
        None => u64::from(elapsed_ms) * 1000,
    }
    .min(LONGEST_STEP_US);
    e.set(this, BSTimer::bZeroDelta, elapsed_us == 0);
    e.set(this, BSTimer::fDelta, (elapsed_us as f64 / 1e6) as f32);
    e.set(this, BSTimer::iLastTime, time);
    // From here as the translation.
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

#[cfg(test)]
mod tests {
    use super::*;

    const SLEEP: u32 = 0x00fd_f104;
    const MILLISECONDS_TO_SECONDS: u32 = 0x0101_6978;

    /// An engine with the globals the step reads, a `Sleep` that moves the clock and logs its
    /// time, and a real-time timer whose steps start at clock 1000 ms.
    fn timer_engine() -> (
        Engine,
        Ptr<BSTimer>,
        std::rc::Rc<std::cell::RefCell<Vec<u32>>>,
    ) {
        let mut e = Engine::new();
        e.map(ZERO, 8);
        e.map(MILLISECONDS_TO_SECONDS, 8);
        e.mem.set_u64(MILLISECONDS_TO_SECONDS, 0.001f64.to_bits());
        e.map(GLOBAL_TIME_MULTIPLIER, 4);
        e.mem.set_f32(GLOBAL_TIME_MULTIPLIER, 1.0);
        let slept = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let log = slept.clone();
        e.register_double(SLEEP, move |e, a| {
            log.borrow_mut().push(a[0]);
            e.advance_clock_us(u64::from(a[0]) * 1000);
            Ret::default()
        });
        let timer = e.new_object::<BSTimer>();
        e.clock_ms = 1000;
        e.set(timer, BSTimer::iFirstTime, 0);
        e.set(timer, BSTimer::iLastTime, 1000);
        (e, timer, slept)
    }

    /// The translation lifts steps 4 ms apart to 10 ms by sleeping and reports 10 ms; the uncapped step
    /// neither sleeps nor rounds up: 4 ms each (the first from the tick count, then the clock).
    #[test]
    fn the_uncapped_step_does_not_sleep_and_reports_the_real_time() {
        let (mut e, timer, slept) = timer_engine();
        for _ in 0..3 {
            e.advance_clock_us(4000);
            let now = e.clock_ms;
            e.call(BS_TIMER_STEP, &[timer.addr(), now]);
            assert_eq!(e.get(timer, BSTimer::fDelta), 0.01);
        }
        // The second step's 10 ms include the first one's sleep.
        assert_eq!(*slept.borrow(), [6, 6]);

        let (mut e, timer, slept) = timer_engine();
        install(&mut e);
        for _ in 0..3 {
            e.advance_clock_us(4000);
            let now = e.clock_ms;
            e.call(BS_TIMER_STEP, &[timer.addr(), now]);
            assert_eq!(e.get(timer, BSTimer::fDelta), 0.004);
            assert_eq!(e.get(timer, BSTimer::fDeltaUnmodified), 0.004);
            assert_eq!(e.get(timer, BSTimer::iLastTime), now);
            assert!(!e.get(timer, BSTimer::bZeroDelta));
        }
        assert!(slept.borrow().is_empty());
        // Under a millisecond: the microsecond clock still sees it.
        e.advance_clock_us(250);
        let now = e.clock_ms;
        e.call(BS_TIMER_STEP, &[timer.addr(), now]);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.00025);
        // A long pause is cut to 166 ms, as the translation cuts it.
        e.advance_clock_us(2_000_000);
        let now = e.clock_ms;
        e.call(BS_TIMER_STEP, &[timer.addr(), now]);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.166);
    }

    /// A disabled timer and a fixed step run the translation itself.
    #[test]
    fn disabled_and_fixed_step_timers_run_the_translation() {
        let (mut e, timer, slept) = timer_engine();
        install(&mut e);
        e.set(timer, BSTimer::cDisableCounter, 1);
        e.advance_clock_us(4000);
        e.call(BS_TIMER_STEP, &[timer.addr(), e.clock_ms]);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.0);
        e.set(timer, BSTimer::cDisableCounter, 0);
        e.set(timer, BSTimer::fClamp, 16.0);
        e.call(BS_TIMER_STEP, &[timer.addr(), e.clock_ms]);
        assert_eq!(e.get(timer, BSTimer::fDelta), 0.016);
        assert!(slept.borrow().is_empty());
    }
}

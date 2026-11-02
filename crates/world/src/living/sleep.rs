//! Sleeping and waiting, read from the game's code (`findings\living.md`
//! §1; `005095b0`, `00969fa0`, `005e0090`, `007c0220`, `007c0580`,
//! `0094df80`, `008a0960`, `0088b510`, `00648b00`).
//!
//! **A bed** (furniture whose `MNAM` has the bed flag, 0x80000000; the
//! game asks whether the nearest free marker is a sleep marker, 1–9, which
//! a bed's are): owned by someone the player may not take from (the
//! owner chain and take test of `world::crime`, faction members of any
//! rank allowed, `XGLB` without effect) → `sNoSleepInOwnedBed`. Then, in
//! this order, each refusal a notice: the place forbids waiting
//! (`005444c0`: an interior's record flag 0x80000, outdoors the
//! worldspace's) → `sNoWaitInCell`; trespassing → `sNoSleepTrespass`; an
//! enemy near (`009764a0`) → `sNoSleepHostileActorsNear`; an effect hurting
//! Health (`00822e00`: an active detrimental effect on actor value 16) →
//! `sNoSleepTakingHealthDamage`. Otherwise the sleep/wait menu opens in
//! sleep mode. Sitting in an owned chair is never refused
//! (`sNoSitOnOwnedFurniture` has no users).
//!
//! **Waiting** (`00969fa0`; the wait key's own list at `009423d8`… has the
//! same order): trespassing (`sNoWaitTrespass`), an enemy near
//! (`sNoWaitHostilActorsNear`), the place forbids it or a script's
//! `EnableFastTravel … 0` does (`sNoWaitInCell`), an effect hurting Health
//! (`sNoWaitTakingHealthDamage`). `ShowSleepWaitMenu sleep 1` runs this
//! list and then opens the menu in sleep mode (`007054f0(1)`), whatever
//! its first argument; `ShowSleepWaitMenu sleep 0` opens it in that mode.
//!
//! **The menu** (1–24 hours; `menus\sleep_wait_menu.xml`, drawn by
//! `ui::menus::sleepwait`): on accepting ([`begin`]), the hours and the
//! sleeping flag are set (`005c1a00`; `IsPCSleeping`, `GetPCSleepHours`)
//! and a sleep counts in misc stat 23 "Times Slept" (`004d5c60(0x17)`).
//! Then **one hour passes every real second** ([`menu_frame`], `007c0580`):
//! in hardcore the needs grow first ([`super::needs::grow`]), then the hour
//! ([`pass_hour`], `0094df80`): the clock moves by 3600 ÷ `TimeScale` real
//! seconds (one game hour); the player heals Heal Rate ÷ 3600 a second of
//! it (the awake rate: nothing in the menu puts the player in a bed's
//! "sleeping" state, so `fHealingRateSleeping…` don't apply) plus the
//! perks' entry point 12 (Modify Recovered Health) × the step, and action
//! points come back over the same real seconds (`0088b660`, `0088b740`);
//! in hardcore while sleeping Sleep Deprivation drops by min(it,
//! `fHCSleepRestorationMod` 60). After the last hour the sleeping flag is
//! cleared and **a sleep outside hardcore restores Health and Fatigue in
//! full and every body part by 1000** (`008a0960`). Cancelling ([`cancel`],
//! its button 5) clears hours and flag: no restore. While the menu is open
//! the scripts' `MenuMode` blocks run ([`menu_mode`]).
//!
//! What else accepting does (`007c0220`, its button 4): an autosave when
//! the INI's `[GamePlay] bSaveOnRest` (sleeping) or `bSaveOnWait` (waiting)
//! is on (`00850a40`; both 1 in the exe), and for a sleep the screen fades
//! to black over `fFadeToBlackFadeSeconds` ([`fade_seconds`]; `00700960`,
//! the fader manager's fader 0, `Interface\Faders\Black.dds`, alpha growing
//! by the frame's seconds ÷ the time until 1, `007011d0`), fading back the
//! same way when the menu closes (`007c01b0` → `007010e0(0, 0)`: alpha
//! falling from 0.9999 by the frame's seconds ÷ the time). The time line
//! it shows is [`Clock`]'s (`007c0000`).
//!
//! The beds scripted with `PlayerBedSCRIPT` give Well Rested themselves:
//! their `MenuMode` block notes `IsPCSleeping` (the menu is number
//! [`SLEEP_WAIT_MENU`] for `MenuMode` blocks), their `GameMode` block casts
//! `WellRestedSpell`, whose script effect calls `ResetHealth` (a full heal,
//! in hardcore too) and gives `WellRestedPerk` (+10 % experience).
//!
//! Left out (not traced): the refusals for an alarm sounding, being under
//! water, in the air and being irradiated (the player's +0x15c/+0x15d
//! flags and the process's +0x764 value: what sets them isn't known); the
//! hour's run of every AI process "as if 3600 seconds passed" (people move
//! on, their effects run) and the followers told to come along
//! (`00974fc0`, `00975160`).

use esm::{FormId, LoadOrder};

use crate::dialogue::PLAYER_REF;
use crate::scripting::{Facts, GameState, Runner};

/// The sleep/wait menu's number for `MenuMode` blocks (`WaitWhackScript`,
/// `LilyMedicineTimerSCRIPT`: `MenuMode 1012`).
pub const SLEEP_WAIT_MENU: u16 = 1012;

/// The most hours the menu offers (its slider's 24 positions, hours =
/// position + 1).
pub const MOST_HOURS: u32 = 24;

/// The perks' entry point "Modify Recovered Health" (the game's table).
const MODIFY_RECOVERED_HEALTH: u8 = 12;

/// The weekday names' text settings, Sunday first (the exe's table at
/// `011895b8`, read by `00867f10`: "Bad Day" outside 0..6).
pub const WEEKDAY_SETTINGS: [&str; 7] = [
    "sDaySunday",
    "sDayMonday",
    "sDayTuesday",
    "sDayWednesday",
    "sDayThursday",
    "sDayFriday",
    "sDaySaturday",
];

/// The game's clock as the menu's time line reads it (`007c0000`): the
/// globals of the clock object at `011de7b8` (year, month, day, hour, days
/// passed: `00867c60`, `00867d20`, `00867d60`, `00867da0`, `00867de0`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Clock {
    /// `GameHour` (0 to 24).
    pub hour: f32,
    /// `GameMonth` (from 0), `GameDay`, `GameYear`.
    pub month: i32,
    pub day: i32,
    pub year: i32,
    /// `GameDaysPassed`.
    pub days_passed: f32,
}

impl Clock {
    /// The clock now, from the state's globals (the game's defaults when a
    /// global is missing: the clock's own, `00867c60`… give the exe's
    /// constants then).
    pub fn now(order: &LoadOrder, state: &GameState) -> Clock {
        let g = |n: &str, d: f32| state.global(order, n).unwrap_or(d);
        Clock {
            hour: g("GameHour", 0.0),
            month: g("GameMonth", 0.0) as i32,
            day: g("GameDay", 1.0) as i32,
            year: g("GameYear", 0.0) as i32,
            days_passed: g("GameDaysPassed", 1.0),
        }
    }

    /// The weekday, Sunday 0 (`00867ef0`: the days passed rounded to a
    /// whole number, modulo 7; 1.0 when the global is missing).
    pub fn weekday(&self) -> usize {
        (self.days_passed.round() as i64).rem_euclid(7) as usize
    }

    /// The date as the game writes it (`00867970`): "%02d.%02d.%02d" with
    /// the month + 1, the day and the year's last two digits.
    pub fn date(&self) -> String {
        format!(
            "{:02}.{:02}.{:02}",
            self.month + 1,
            self.day,
            self.year.rem_euclid(100)
        )
    }

    /// The 12-hour clock (`007c0000`): the hour and its minutes, both
    /// truncated; 12 from 0:00 up to 1:00, the hour less 12 from 13:00,
    /// else as it is; `true` for PM from 12:00.
    pub fn twelve_hour(&self) -> (i32, i32, bool) {
        let hour = self.hour;
        let mut h = hour.trunc() as i32;
        let m = ((hour - hour.trunc()) * 60.0).trunc() as i32;
        if (0.0..1.0).contains(&hour) {
            h = 12;
        } else if hour >= 13.0 {
            h -= 12;
        }
        (h, m, hour >= 12.0)
    }
}

/// Whether accepting the hours autosaves (`007c0220` → `00850a40`): the
/// INI's `[GamePlay] bSaveOnRest` for a sleep, `bSaveOnWait` for a wait
/// (both 1 in the exe when the INIs don't set them).
pub fn autosaves(sleeping: bool, save_on_rest: Option<bool>, save_on_wait: Option<bool>) -> bool {
    if sleeping {
        save_on_rest.unwrap_or(true)
    } else {
        save_on_wait.unwrap_or(true)
    }
}

/// How long a sleep's fade to black takes (`fFadeToBlackFadeSeconds`, the
/// exe's 0.3), and the fade back.
pub fn fade_seconds(order: &LoadOrder) -> f32 {
    super::setting(order, "fFadeToBlackFadeSeconds", 0.3)
}

/// The screen's fade to black and back (the fader manager's fader 0,
/// `FaderManager.cpp`: `00700960` starts it, `007010e0` releases it,
/// `007011d0` moves it every frame). Alpha 0 to 1 over `seconds` while
/// fading in, held at 1, then from 0.9999 back to 0 at the same rate once
/// released; gone at 0.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Fade {
    pub alpha: f32,
    pub seconds: f32,
    pub fading_in: bool,
    pub on: bool,
}

impl Fade {
    /// A fade to black started (`00700960(0, seconds, 0)`): over no time
    /// at all it's black at once.
    pub fn start(seconds: f32) -> Fade {
        Fade {
            alpha: if seconds == 0.0 { 1.0 } else { 0.0 },
            seconds,
            fading_in: true,
            on: true,
        }
    }

    /// Released (`007010e0(0, 0)`): fades back; a fade that had arrived
    /// starts back from 0.9999.
    pub fn release(&mut self) {
        if !self.on {
            return;
        }
        self.fading_in = false;
        if self.alpha == 1.0 {
            self.alpha = 0.9999;
        }
    }

    /// A frame of `dt` real seconds (`007011d0`).
    pub fn frame(&mut self, dt: f32) {
        if !self.on {
            return;
        }
        let step = if self.seconds > 0.0 {
            dt / self.seconds
        } else {
            1.0
        };
        if self.fading_in {
            if self.alpha < 1.0 {
                self.alpha = (self.alpha + step).min(1.0);
            }
        } else if self.alpha > 0.0 {
            self.alpha = (self.alpha - step).max(0.0);
        } else {
            self.on = false;
        }
    }
}

/// The record flag that forbids waiting (interior cells; worldspaces).
const CANT_WAIT: u32 = 0x8_0000;

/// Whether a placed piece of furniture is a bed (its base's `MNAM` bed
/// flag).
pub fn is_bed(order: &LoadOrder, furniture: FormId) -> bool {
    crate::scripting::base_of(order, furniture)
        .is_some_and(|b| crate::furniture::marker_flags(order, b) & crate::furniture::BED != 0)
}

/// Whether the place the player is in forbids waiting (`005444c0`): an
/// interior with record flag 0x80000, outdoors its worldspace's.
pub fn place_forbids_waiting(order: &LoadOrder, state: &GameState) -> bool {
    let flags = |id: FormId| order.get(id).map_or(0, |r| r.entry.header.flags);
    match (state.player_world, state.player_cell) {
        (Some(w), _) => flags(w) & CANT_WAIT != 0,
        (None, Some(c)) => flags(c) & CANT_WAIT != 0,
        _ => false,
    }
}

/// Whether an effect hurting Health works on the player (`00822e00`: an
/// active, detrimental effect on actor value 16).
pub fn taking_health_damage(state: &GameState) -> bool {
    state.active_effects.iter().any(|e| {
        e.target == PLAYER_REF
            && e.actor_value == i32::from(crate::combat::av::HEALTH)
            && e.detrimental
            && e.remaining > 0.0
    })
}

fn trespassing(order: &LoadOrder, state: &GameState) -> bool {
    state
        .player_cell
        .is_some_and(|c| crate::crime::trespassing(order, state, c))
}

/// Why the player can't wait now (`00969fa0`), in the game's words; `Ok`
/// when the menu may open.
pub fn may_wait(order: &LoadOrder, state: &GameState) -> Result<(), String> {
    let refuse = |name: &str, exe: &str| Err(super::text(order, name, exe));
    if trespassing(order, state) {
        return refuse("sNoWaitTrespass", "You cannot wait while trespassing.");
    }
    if state.enemies_near(order) {
        return refuse(
            "sNoWaitHostilActorsNear",
            "You cannot wait when enemies are nearby.",
        );
    }
    if place_forbids_waiting(order, state) || !state.set_by_scripts.fast_travel.can_wait {
        return refuse("sNoWaitInCell", "You cannot wait in this location.");
    }
    if taking_health_damage(state) {
        return refuse(
            "sNoWaitTakingHealthDamage",
            "You cannot wait while taking health damage.",
        );
    }
    Ok(())
}

/// Why the player can't sleep in a bed (`005095b0`), in the game's words;
/// `Ok` when its sleep menu may open.
pub fn may_sleep_in(order: &LoadOrder, state: &GameState, bed: FormId) -> Result<(), String> {
    let refuse = |name: &str, exe: &str| Err(super::text(order, name, exe));
    let owner = crate::crime::owner_of(order, state, bed);
    if !crate::crime::may_take(order, state, owner) {
        return refuse("sNoSleepInOwnedBed", "You cannot sleep in an owned bed.");
    }
    if place_forbids_waiting(order, state) {
        return refuse("sNoWaitInCell", "You cannot wait in this location.");
    }
    if trespassing(order, state) {
        return refuse("sNoSleepTrespass", "You cannot sleep while trespassing.");
    }
    if state.enemies_near(order) {
        return refuse(
            "sNoSleepHostileActorsNear",
            "You cannot sleep when enemies are near by.",
        );
    }
    if taking_health_damage(state) {
        return refuse(
            "sNoSleepTakingHealthDamage",
            "You cannot sleep while taking health damage.",
        );
    }
    Ok(())
}

/// The menu's hours accepted (`007c0220`, its button 4): the hours and the
/// sleeping flag set; a sleep counts in "Times Slept".
pub fn begin(state: &mut GameState, hours: u32, sleeping: bool) {
    state.living.hours_left = hours as i32;
    state.living.sleeping = sleeping;
    if sleeping {
        crate::stats::bump(state, crate::stats::TIMES_SLEPT, 1);
    }
}

/// The menu cancelled (its button 5): hours and the sleeping flag cleared,
/// so no restore.
pub fn cancel(state: &mut GameState) {
    state.living.hours_left = 0;
    state.living.sleeping = false;
}

/// One hour of the menu passes (`007c0580` each real second: the needs in
/// hardcore, then `0094df80`; see the module notes). Whether the hours are
/// over (the menu closes).
pub fn pass_hour(order: &LoadOrder, state: &mut GameState) -> bool {
    if state.living.hardcore {
        super::needs::grow(order, state);
    }
    let scale = state.global(order, "TimeScale").unwrap_or(30.0).max(1e-3);
    let step = 3600.0 / scale;
    state.advance_clock(order, step);
    if !state.dead.contains(&PLAYER_REF) {
        heal_for(order, state, f64::from(step));
    }
    // Action points come back over the same real seconds (`0088b660`).
    crate::vats::regenerate(order, state, step);
    if state.living.hardcore && state.living.sleeping {
        let sd = super::needs::value(order, state, PLAYER_REF, super::needs::SLEEP_DEPRIVATION);
        let by = f64::from(super::setting(order, "fHCSleepRestorationMod", 60.0)).min(sd);
        if by > 0.0 {
            crate::magic::change(
                order,
                state,
                PLAYER_REF,
                super::needs::SLEEP_DEPRIVATION,
                by,
                PLAYER_REF,
            );
        }
    }
    state.living.hours_left -= 1;
    let done = state.living.hours_left < 1;
    if done {
        let slept = std::mem::take(&mut state.living.sleeping);
        if slept && !state.living.hardcore {
            rested(order, state);
        }
    }
    done
}

/// The player heals for `seconds` of game time awake (`0088b510` with
/// `00648b00`): Heal Rate (actor value 15) ÷ 3600 a second, plus the
/// perks' "Modify Recovered Health" a second.
fn heal_for(order: &LoadOrder, state: &mut GameState, seconds: f64) {
    let rate = Facts {
        order,
        state,
        speaker: None,
    }
    .current_actor_value(PLAYER_REF, 15)
    .unwrap_or(0.0)
        / 3600.0;
    let perk = f64::from(crate::perks::apply(
        order,
        state,
        MODIFY_RECOVERED_HEALTH,
        0.0,
    ));
    let healed = (rate + perk) * seconds;
    if healed > 0.0 {
        crate::magic::change(
            order,
            state,
            PLAYER_REF,
            crate::combat::av::HEALTH,
            healed,
            PLAYER_REF,
        );
    }
}

/// The end of a sleep outside hardcore (`008a0960`): Health and Fatigue
/// back to full (restored by what they lack + 1), every body part
/// condition (25–31) restored by 1000.
pub fn rested(order: &LoadOrder, state: &mut GameState) {
    state.damage.remove(&PLAYER_REF);
    let fatigue: u16 = 22;
    let lacking = state
        .value_damage
        .get(&(PLAYER_REF, fatigue))
        .copied()
        .unwrap_or(0.0);
    if lacking > 0.0 {
        crate::magic::change(order, state, PLAYER_REF, fatigue, lacking + 1.0, PLAYER_REF);
    }
    for part in crate::body_parts::av::FIRST_CONDITION..=crate::body_parts::av::LAST_CONDITION {
        crate::magic::change(order, state, PLAYER_REF, part, 1000.0, PLAYER_REF);
    }
}

/// The menu at work for one frame of `seconds` (`007c0580`): its clock
/// runs (`timer`, kept by the menu from 0), and every whole real second an
/// hour passes ([`pass_hour`]). `Some(over)` when an hour passed.
pub fn menu_frame(
    order: &LoadOrder,
    state: &mut GameState,
    timer: &mut f32,
    seconds: f32,
) -> Option<bool> {
    *timer += seconds;
    if *timer < 1.0 {
        return None;
    }
    *timer = 0.0;
    Some(pass_hour(order, state))
}

/// While the menu is open the scripts' `MenuMode` blocks for it (or for
/// any menu) run: the running quests' and those of `references` (the
/// scripted objects where the player is, as for `GameMode`), as
/// `PlayerBedSCRIPT` notes `IsPCSleeping` there.
pub fn menu_mode(runner: &mut Runner, references: &[FormId]) {
    runner.menu_mode(SLEEP_WAIT_MENU);
    for &r in references {
        runner.run_blocks(r, Some(r), "menumode", |b| match b.args.first() {
            None => true,
            Some(script::Arg::Number(n)) => *n as u16 == SLEEP_WAIT_MENU,
            Some(_) => false,
        });
    }
}

/// All the chosen hours at once (`GameState::wait`, and what a test or the
/// command line asks): the refusals, then each hour in turn.
pub fn wait_all(
    order: &LoadOrder,
    state: &mut GameState,
    hours: u32,
    sleeping: bool,
) -> Result<(), String> {
    may_wait(order, state)?;
    begin(state, hours, sleeping);
    while state.living.hours_left > 0 {
        pass_hour(order, state);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clock(hour: f32, days_passed: f32) -> Clock {
        Clock {
            hour,
            month: 9,
            day: 19,
            year: 2281,
            days_passed,
        }
    }

    /// `007c0000`: the 12-hour clock and AM/PM; `00867970`: the date;
    /// `00867ef0`: the weekday from the days passed.
    #[test]
    fn the_time_line_as_the_game_writes_it() {
        assert_eq!(clock(0.5, 1.0).twelve_hour(), (12, 30, false));
        assert_eq!(clock(1.0, 1.0).twelve_hour(), (1, 0, false));
        assert_eq!(clock(11.999, 1.0).twelve_hour(), (11, 59, false));
        assert_eq!(clock(12.25, 1.0).twelve_hour(), (12, 15, true));
        assert_eq!(clock(13.0, 1.0).twelve_hour(), (1, 0, true));
        assert_eq!(clock(23.9, 1.0).twelve_hour(), (11, 53, true));
        assert_eq!(clock(9.0, 1.0).date(), "10.19.81");
        assert_eq!(clock(9.0, 1.0).weekday(), 1);
        assert_eq!(clock(9.0, 1.49).weekday(), 1);
        assert_eq!(clock(9.0, 1.5).weekday(), 2);
        assert_eq!(clock(9.0, 7.0).weekday(), 0);
        assert_eq!(clock(9.0, 13.2).weekday(), 6);
        assert_eq!(WEEKDAY_SETTINGS[0], "sDaySunday");
    }

    /// `007c0220` → `00850a40`: the INI's save-on-rest and save-on-wait,
    /// both on by default.
    #[test]
    fn autosaving_follows_the_ini() {
        assert!(autosaves(true, None, None));
        assert!(autosaves(false, None, None));
        assert!(!autosaves(true, Some(false), Some(true)));
        assert!(autosaves(false, Some(false), Some(true)));
        assert!(!autosaves(false, Some(true), Some(false)));
    }

    /// `00700960`, `007011d0`, `007010e0`: the fade to black grows by the
    /// frame's share of the time, holds at 1, and once released falls back
    /// from 0.9999 and goes away at 0.
    #[test]
    fn the_fade_to_black_and_back() {
        let mut f = Fade::start(0.3);
        assert_eq!(f.alpha, 0.0);
        f.frame(0.15);
        assert!((f.alpha - 0.5).abs() < 1e-6);
        f.frame(0.3);
        assert_eq!(f.alpha, 1.0);
        f.frame(0.3);
        assert_eq!(f.alpha, 1.0);
        assert!(f.on);
        f.release();
        assert_eq!(f.alpha, 0.9999);
        f.frame(0.15);
        assert!((f.alpha - 0.4999).abs() < 1e-5);
        f.frame(0.3);
        assert_eq!(f.alpha, 0.0);
        assert!(f.on);
        f.frame(0.01);
        assert!(!f.on);
        // Released half way in: back from where it was.
        let mut f = Fade::start(1.0);
        f.frame(0.25);
        f.release();
        f.frame(0.1);
        assert!((f.alpha - 0.15).abs() < 1e-6);
        // No time: black at once.
        let f = Fade::start(0.0);
        assert_eq!(f.alpha, 1.0);
    }
}

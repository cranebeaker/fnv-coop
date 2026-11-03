//! Weather and clock presentation for co-op: reading the host's sky into
//! plain numbers, and applying the host's sky on a guest through the game's
//! own weather calls (`WeatherState::force` and `set`, what `ForceWeather`
//! and `SetWeather` run).
//!
//! The guest never rolls weather: the host's weather is installed as the
//! override (`forced`), so the guest's own weather step only fades between
//! what the host says. That fade is driven by the game hour, and the guest's
//! hour is the smoothed host hour, so both fades line up; `started` is copied
//! from the host to keep them in step, and a fade that still drifts is
//! corrected in place.
//!
//! The two ends meet in `coop-session` (`live_frame`), which carries these
//! numbers in `coop_core::env` messages.

use crate::weather::WeatherState;
use esm::FormId;

/// A fade this far off the host's is set to the host's (0..1).
pub const FADE_SNAP: f32 = 0.12;

/// The host's sky as plain numbers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SkyShot {
    /// Weather fading in (0: none).
    pub current: u32,
    /// Weather fading out (0: none).
    pub previous: u32,
    /// A script's override (0: none).
    pub forced: u32,
    /// Fade progress in thousandths.
    pub fade_milli: u16,
    /// Hour the current weather began, hundredths.
    pub started_centi: u16,
}

/// Reads a state's sky.
pub fn read_sky(w: &WeatherState) -> SkyShot {
    SkyShot {
        current: w.current.map_or(0, |f| f.0),
        previous: w.previous.map_or(0, |f| f.0),
        forced: w.forced.map_or(0, |f| f.0),
        fade_milli: (w.fade.clamp(0.0, 1.0) * 1000.0).round() as u16,
        started_centi: (w.started.rem_euclid(24.0) * 100.0).round().min(2399.0) as u16,
    }
}

/// What applying a sky did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Applied {
    /// Nothing to do.
    Same,
    /// Only the fade needed a correction.
    FadeCorrected,
    /// The host started a fade into a new weather; the guest began it too.
    Transition,
    /// The weather was installed without a fade (first sky, or a jump the
    /// guest could not follow: it missed a weather change).
    Cut,
}

fn form(v: u32) -> Option<FormId> {
    (v != 0).then_some(FormId(v))
}

/// Mirrors the host's sky on a guest. `hour` is the guest's (smoothed) game
/// hour.
pub fn apply_sky(w: &mut WeatherState, s: &SkyShot, hour: f32) -> Applied {
    let Some(cur) = form(s.current) else {
        return Applied::Same;
    };
    let prev = form(s.previous);
    let started = f32::from(s.started_centi) / 100.0;
    // the guest never rolls: the host's weather is the override
    let result = if w.current.is_none() {
        w.force(cur, true, hour);
        if prev.is_some() {
            w.previous = prev;
            w.started = started;
            w.fade = f32::from(s.fade_milli) / 1000.0;
        } else {
            w.started = started;
        }
        Applied::Cut
    } else if w.current != Some(cur) {
        if prev.is_some() && prev == w.current {
            // the host started a fade out of what we show: begin it the game's way
            w.set(cur, true, hour);
            w.previous = w.current;
            w.current = Some(cur);
            w.started = started;
            w.sped_up = None;
            w.fade = f32::from(s.fade_milli) / 1000.0;
            Applied::Transition
        } else {
            w.force(cur, true, hour);
            w.started = started;
            if prev.is_some() {
                w.previous = prev;
                w.fade = f32::from(s.fade_milli) / 1000.0;
            }
            Applied::Cut
        }
    } else {
        w.forced = Some(cur);
        let host_fade = f32::from(s.fade_milli) / 1000.0;
        match (w.previous.is_some(), prev.is_some()) {
            (true, true) if (w.fade - host_fade).abs() > FADE_SNAP => {
                w.fade = host_fade;
                w.started = started;
                Applied::FadeCorrected
            }
            (true, false) if (1.0 - w.fade) > FADE_SNAP => {
                // the host's fade is over; ours lags: finish it
                w.fade = 1.0;
                w.previous = None;
                w.sped_up = None;
                Applied::FadeCorrected
            }
            (false, true) => {
                // the host fades and we do not (a missed start): join the fade
                w.previous = prev;
                w.started = started;
                w.fade = host_fade;
                Applied::FadeCorrected
            }
            _ => Applied::Same,
        }
    };
    w.forced = Some(cur);
    result
}

/// The clock numbers a host publishes: `(GameDaysPassed, GameHour, TimeScale)`.
pub fn read_clock(order: &esm::LoadOrder, gs: &crate::scripting::GameState) -> (f32, f32, f32) {
    (
        gs.global(order, "GameDaysPassed").unwrap_or(0.0),
        gs.global(order, "GameHour")
            .unwrap_or(crate::weather::DEFAULT_HOUR),
        gs.global(order, "TimeScale").unwrap_or(30.0),
    )
}

/// Writes the guest's smoothed absolute hours into the clock globals the
/// sky reads (`GameHour`, `GameDaysPassed`). Presentation only: the shared
/// state still carries the host's quantized values.
pub fn write_clock(
    order: &esm::LoadOrder,
    gs: &mut crate::scripting::GameState,
    absolute_hours: f64,
) {
    let days = (absolute_hours / 24.0).floor();
    let hour = (absolute_hours - days * 24.0) as f32;
    if let Some(id) = order.form_by_editor_id("GameHour") {
        gs.globals.insert(id, hour);
    }
    if let Some(id) = order.form_by_editor_id("GameDaysPassed") {
        gs.globals.insert(id, days as f32);
    }
}

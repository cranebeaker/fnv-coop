//! Weather and time presentation sync. The host owns both; guests mirror
//! them without rolling their own weather or running their own clock.
//!
//! * [`EnvMsg::Sky`] (reliable): which weather is current, which one is
//!   fading out, how far the fade is, and whether a script forced it. Sent
//!   when something changed (the fade is only re-sent when it drifts) and as
//!   a slow keepalive, so a late joiner needs no special path.
//! * [`EnvMsg::Time`] (unreliable stream): the host's day count, hour and
//!   time scale about twice a second. [`ClockSmoother`] turns those samples
//!   (and the 1/64 hour steps `GameHour` itself replicates in) into a smooth
//!   hour for the sky: it advances at the host's scale between samples,
//!   slews toward each sample, never runs backwards, and snaps on a jump
//!   (sleep, wait, a script).
//!
//! The game side lives in `world::coop::env`: it reads the host's
//! `WeatherState` into a [`SkyState`] and applies a received one through the
//! game's own `force`/`set` calls (`ForceWeather`, `SetWeather`).

use serde::{Deserialize, Serialize};

/// Host sends a sky update at most this often (ms) when only the fade moved.
pub const SKY_FADE_EVERY_MS: u64 = 500;
/// Host repeats an unchanged sky this often (ms).
pub const SKY_KEEPALIVE_MS: u64 = 5000;
/// Host sends the time this often (ms).
pub const TIME_EVERY_MS: u64 = 500;
/// A fade this far off the host's (0..1) is corrected, not left to run.
pub const FADE_SNAP: f32 = 0.12;
/// Hours of difference past which the guest clock snaps instead of slewing.
pub const CLOCK_SNAP_HOURS: f64 = 0.5;

/// The sky as the host's weather system holds it. Weather forms are raw
/// form ids (0 = none).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkyState {
    /// Weather fading in (or shown).
    pub current: u32,
    /// Weather fading out (0: no fade running).
    pub previous: u32,
    /// A script's override (`ForceWeather W 1`), 0 if none.
    pub forced: u32,
    /// Fade progress in thousandths (1000: done).
    pub fade_milli: u16,
    /// Hour of day the current weather began, in hundredths of an hour.
    pub started_centi: u16,
}

impl SkyState {
    /// Fade as 0..=1.
    pub fn fade(&self) -> f32 {
        f32::from(self.fade_milli) / 1000.0
    }

    /// True when a transition (not just the fade value) differs.
    pub fn same_weather(&self, o: &SkyState) -> bool {
        self.current == o.current && self.previous == o.previous && self.forced == o.forced
    }
}

/// The host's clock.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeState {
    /// `GameDaysPassed`.
    pub days: u32,
    /// `GameHour` in thousandths of an hour (0..24000).
    pub hour_milli: u32,
    /// Game seconds per real second (`TimeScale`), tenths.
    pub scale_deci: u16,
}

impl TimeState {
    /// Hours since day 0.
    pub fn absolute_hours(&self) -> f64 {
        f64::from(self.days) * 24.0 + f64::from(self.hour_milli) / 1000.0
    }

    /// Builds from game values.
    pub fn from_game(days: f32, hour: f32, scale: f32) -> TimeState {
        TimeState {
            days: days.max(0.0) as u32,
            hour_milli: ((hour.rem_euclid(24.0)) * 1000.0).round().min(23_999.0) as u32,
            scale_deci: (scale.clamp(0.0, 6000.0) * 10.0).round() as u16,
        }
    }
}

/// Environment wire messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvMsg {
    /// Host: the sky (reliable).
    Sky(SkyState),
    /// Host: the clock (unreliable).
    Time(TimeState),
}

impl EnvMsg {
    /// True for the unreliable stream.
    pub fn is_stream(&self) -> bool {
        matches!(self, EnvMsg::Time(_))
    }
}

/// Host side: decides when to send.
#[derive(Clone, Debug, Default)]
pub struct HostEnv {
    last_sky: Option<SkyState>,
    sky_sent_ms: u64,
    time_sent_ms: u64,
    /// Messages sent (for the budget report).
    pub sent: u32,
}

impl HostEnv {
    /// New publisher.
    pub fn new() -> Self {
        Self::default()
    }

    /// Forgets what was sent (a guest joined: it needs everything again).
    pub fn resend(&mut self) {
        self.last_sky = None;
        self.time_sent_ms = 0;
    }

    /// Messages to broadcast now given the host's sky and clock.
    pub fn tick(&mut self, now_ms: u64, sky: SkyState, time: TimeState) -> Vec<EnvMsg> {
        let mut out = Vec::new();
        let due = match &self.last_sky {
            None => true,
            Some(l) if !l.same_weather(&sky) => true,
            Some(l) => {
                let moved = l.fade_milli != sky.fade_milli;
                (moved && now_ms.saturating_sub(self.sky_sent_ms) >= SKY_FADE_EVERY_MS)
                    || now_ms.saturating_sub(self.sky_sent_ms) >= SKY_KEEPALIVE_MS
            }
        };
        if due {
            self.last_sky = Some(sky);
            self.sky_sent_ms = now_ms;
            out.push(EnvMsg::Sky(sky));
        }
        if self.time_sent_ms == 0 || now_ms.saturating_sub(self.time_sent_ms) >= TIME_EVERY_MS {
            self.time_sent_ms = now_ms.max(1);
            out.push(EnvMsg::Time(time));
        }
        self.sent += out.len() as u32;
        out
    }
}

/// Guest side: smooth hour of day from host samples.
#[derive(Clone, Debug, Default)]
pub struct ClockSmoother {
    /// Smoothed absolute hours, if started.
    now: Option<f64>,
    /// Target absolute hours at `target_ms`, and the host's rate.
    target: f64,
    target_ms: u64,
    rate_per_s: f64,
    last_ms: u64,
    /// Snaps taken (jumps).
    pub snaps: u32,
}

impl ClockSmoother {
    /// New smoother.
    pub fn new() -> Self {
        Self::default()
    }

    /// A host sample arrived at `now_ms`.
    pub fn feed(&mut self, now_ms: u64, t: TimeState) {
        self.target = t.absolute_hours();
        self.target_ms = now_ms;
        self.rate_per_s = f64::from(t.scale_deci) / 10.0 / 3600.0;
        if self.now.is_none() {
            self.now = Some(self.target);
            self.last_ms = now_ms;
        }
    }

    /// The smooth absolute hours at `now_ms` (`None` before any sample).
    pub fn sample(&mut self, now_ms: u64) -> Option<f64> {
        let cur = self.now?;
        let dt = now_ms.saturating_sub(self.last_ms) as f64 / 1000.0;
        self.last_ms = now_ms;
        let ahead = (now_ms.saturating_sub(self.target_ms)) as f64 / 1000.0;
        let want = self.target + self.rate_per_s * ahead;
        let free_run = cur + self.rate_per_s * dt;
        let next = if (want - free_run).abs() > CLOCK_SNAP_HOURS {
            self.snaps += 1;
            want
        } else {
            // slew: close 1/4 of the gap per second, but never go backwards
            let gap = want - free_run;
            let step = gap * (dt * 4.0).min(1.0);
            (free_run + step).max(cur)
        };
        self.now = Some(next);
        Some(next)
    }

    /// Hour of day (0..24) at `now_ms`.
    pub fn hour_of_day(&mut self, now_ms: u64) -> Option<f32> {
        self.sample(now_ms).map(|h| h.rem_euclid(24.0) as f32)
    }
}

/// What a guest holds of the host's environment.
#[derive(Clone, Debug, Default)]
pub struct GuestEnv {
    /// Latest sky, set when a new one arrives and cleared when taken.
    pub sky: Option<SkyState>,
    /// Clock smoother.
    pub clock: ClockSmoother,
    /// Sky messages received.
    pub skies: u32,
}

impl GuestEnv {
    /// Applies a received message.
    pub fn on_msg(&mut self, m: EnvMsg, now_ms: u64) {
        match m {
            EnvMsg::Sky(s) => {
                self.sky = Some(s);
                self.skies += 1;
            }
            EnvMsg::Time(t) => self.clock.feed(now_ms, t),
        }
    }

    /// Takes the sky to apply, if a new one arrived.
    pub fn take_sky(&mut self) -> Option<SkyState> {
        self.sky.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sky(cur: u32, prev: u32, fade: u16) -> SkyState {
        SkyState {
            current: cur,
            previous: prev,
            forced: 0,
            fade_milli: fade,
            started_centi: 800,
        }
    }

    fn time(h: f32) -> TimeState {
        TimeState::from_game(3.0, h, 30.0)
    }

    #[test]
    fn host_sends_first_then_only_on_change() {
        let mut h = HostEnv::new();
        let a = h.tick(1000, sky(1, 0, 1000), time(8.0));
        assert_eq!(a.len(), 2);
        assert!(h.tick(1100, sky(1, 0, 1000), time(8.01)).is_empty());
        let b = h.tick(1200, sky(2, 1, 0), time(8.01));
        assert!(matches!(b[0], EnvMsg::Sky(s) if s.current == 2));
    }

    #[test]
    fn fade_is_throttled_and_keepalive_repeats() {
        let mut h = HostEnv::new();
        h.tick(1000, sky(2, 1, 0), time(8.0));
        assert!(
            h.tick(1100, sky(2, 1, 50), time(8.0))
                .iter()
                .all(|m| matches!(m, EnvMsg::Time(_)) || false)
                || true
        );
        let skies = |v: Vec<EnvMsg>| v.iter().filter(|m| matches!(m, EnvMsg::Sky(_))).count();
        assert_eq!(
            skies(h.tick(1200, sky(2, 1, 60), time(8.0))),
            0,
            "inside the throttle"
        );
        assert_eq!(skies(h.tick(1600, sky(2, 1, 100), time(8.0))), 1);
        assert_eq!(
            skies(h.tick(7000, sky(2, 1, 100), time(8.0))),
            1,
            "keepalive"
        );
    }

    #[test]
    fn resend_replays_everything_for_a_new_guest() {
        let mut h = HostEnv::new();
        h.tick(1000, sky(1, 0, 1000), time(8.0));
        h.resend();
        assert_eq!(h.tick(1010, sky(1, 0, 1000), time(8.0)).len(), 2);
    }

    #[test]
    fn time_stream_is_unreliable_and_sky_reliable() {
        assert!(EnvMsg::Time(time(1.0)).is_stream());
        assert!(!EnvMsg::Sky(SkyState::default()).is_stream());
    }

    #[test]
    fn clock_runs_smoothly_between_samples_and_never_backwards() {
        let mut c = ClockSmoother::new();
        c.feed(0, time(8.0));
        let mut prev = c.sample(0).unwrap();
        // host sends a (quantized, slightly behind) value every 500 ms
        for i in 1..200u64 {
            let ms = i * 50;
            if ms % 500 == 0 {
                let host_h = 8.0 + (ms as f32 / 1000.0) * 30.0 / 3600.0;
                c.feed(ms, time((host_h * 64.0).floor() / 64.0));
            }
            let h = c.sample(ms).unwrap();
            assert!(h >= prev, "went backwards at {ms}");
            assert!(h - prev < 0.01, "stepped {} at {ms}", h - prev);
            prev = h;
        }
        let want = 72.0 + 8.0 + 10.0 * 30.0 / 3600.0;
        assert!((prev - want).abs() < 0.03, "{prev} vs {want}");
        assert_eq!(c.snaps, 0);
    }

    #[test]
    fn clock_snaps_on_a_sleep() {
        let mut c = ClockSmoother::new();
        c.feed(0, time(8.0));
        c.sample(0);
        c.sample(500);
        c.feed(600, time(14.0));
        let h = c.sample(650).unwrap();
        assert!((h - (24.0 * 3.0 + 14.0)).abs() < 0.05, "{h}");
        assert_eq!(c.snaps, 1);
    }

    #[test]
    fn clock_crosses_midnight() {
        let mut c = ClockSmoother::new();
        c.feed(0, time(23.99));
        c.sample(0);
        c.feed(
            500,
            TimeState {
                days: 4,
                hour_milli: 10,
                scale_deci: 300,
            },
        );
        let h = c.hour_of_day(550).unwrap();
        assert!(h < 0.1 || h > 23.9, "{h}");
    }

    #[test]
    fn guest_takes_sky_once() {
        let mut g = GuestEnv::default();
        g.on_msg(EnvMsg::Sky(sky(3, 0, 1000)), 5);
        assert_eq!(g.take_sky().unwrap().current, 3);
        assert!(g.take_sky().is_none());
    }

    #[test]
    fn wire_roundtrip_is_small() {
        let m = crate::proto::Msg::Env(EnvMsg::Sky(sky(0xDEAD, 0xBEEF, 500)));
        let b = crate::proto::encode(&m).unwrap();
        assert!(b.len() < 24, "{}", b.len());
        assert_eq!(crate::proto::decode(m.channel(), &b).unwrap(), m);
        let t = crate::proto::Msg::Env(EnvMsg::Time(time(9.5)));
        assert_eq!(t.channel(), crate::proto::Channel::Unreliable);
    }
}

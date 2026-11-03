//! Receiver-side snapshot interpolation with a capped extrapolation and
//! teleport detection.

use std::collections::VecDeque;

use super::{dequant_heading, dequant_pos, AvatarState, SEND_INTERVAL_MS, VEL_Q};
use crate::ids::{CellId, Seq};

/// Render delay behind the newest data, in ms.
pub const DELAY_MS: i64 = 100;
/// Longest time we extrapolate past the newest snapshot, in ms.
pub const EXTRAPOLATE_CAP_MS: i64 = 250;
/// A jump longer than this (world units) between samples is a teleport.
pub const TELEPORT_DIST: f32 = 4000.0;
const MAX_SNAPS: usize = 32;

/// An interpolated view of a remote avatar.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sample {
    /// Cell.
    pub cell: CellId,
    /// World position.
    pub pos: [f32; 3],
    /// Heading in radians.
    pub heading: f32,
    /// Velocity in units per second.
    pub vel: [f32; 3],
    /// Animation id.
    pub anim: u16,
    /// Weapon form.
    pub weapon: u32,
    /// Health.
    pub health: u16,
    /// Flags.
    pub flags: u16,
    /// Furniture in use (0: none).
    pub seat: u32,
    /// True when the position is a prediction beyond the newest data.
    pub extrapolated: bool,
    /// True exactly once after a teleport: snap, do not blend.
    pub teleported: bool,
}

/// Jitter buffer for one remote avatar.
#[derive(Clone, Debug, Default)]
pub struct InterpBuffer {
    snaps: VecDeque<(Seq, AvatarState)>,
    /// Smoothed estimate of `arrival - sender_time`, ms.
    base: Option<f64>,
    teleport_pending: bool,
    /// Samples rejected as old or duplicate.
    pub dropped: u32,
    /// Teleports detected.
    pub teleports: u32,
}

fn sender_time(seq: Seq) -> i64 {
    seq.0 as i64 * SEND_INTERVAL_MS as i64
}

fn dist(a: &AvatarState, b: &AvatarState) -> f32 {
    let d: f32 = (0..3)
        .map(|i| dequant_pos(a.pos[i] - b.pos[i]).powi(2))
        .sum();
    d.sqrt()
}

impl InterpBuffer {
    /// Empty buffer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Snapshots held.
    pub fn len(&self) -> usize {
        self.snaps.len()
    }

    /// True when nothing is buffered.
    pub fn is_empty(&self) -> bool {
        self.snaps.is_empty()
    }

    /// Adds a received sample. Returns false when it was old or duplicate.
    pub fn push(&mut self, now_ms: u64, seq: Seq, state: AvatarState) -> bool {
        if let Some((newest, _)) = self.snaps.back() {
            if newest.0.wrapping_sub(seq.0) > 200 && !seq.is_newer_than(*newest) {
                // sender restarted: start over
                self.snaps.clear();
                self.base = None;
            }
        }
        if self.snaps.iter().any(|(s, _)| *s == seq) {
            self.dropped += 1;
            return false;
        }
        let newest = self.snaps.back().map(|(s, st)| (*s, *st));
        if let Some((ns, _)) = newest {
            if !seq.is_newer_than(ns) {
                // late arrival: insert in order if it is still useful
                let pos = self
                    .snaps
                    .iter()
                    .position(|(s, _)| s.is_newer_than(seq))
                    .unwrap_or(self.snaps.len());
                if pos == 0 {
                    self.dropped += 1;
                    return false;
                }
                self.snaps.insert(pos, (seq, state));
                return true;
            }
        }
        let cand = now_ms as f64 - sender_time(seq) as f64;
        self.base = Some(match self.base {
            None => cand,
            Some(b) if cand < b => cand,
            Some(b) => b + (cand - b) / 32.0,
        });
        if let Some((_, prev)) = newest {
            if prev.cell != state.cell || dist(&prev, &state) > TELEPORT_DIST {
                self.snaps.clear();
                self.teleport_pending = true;
                self.teleports += 1;
            }
        }
        self.snaps.push_back((seq, state));
        while self.snaps.len() > MAX_SNAPS {
            self.snaps.pop_front();
        }
        true
    }

    /// Where the avatar should be drawn at `now_ms`.
    pub fn sample(&mut self, now_ms: u64) -> Option<Sample> {
        let base = self.base?;
        let t = (now_ms as f64 - base) as i64 - DELAY_MS;
        let tele = std::mem::take(&mut self.teleport_pending);
        let first = self.snaps.front()?;
        if t <= sender_time(first.0) {
            return Some(make(&first.1, &first.1, 0.0, 0, tele));
        }
        // drop snapshots older than the bracket start, keeping one before t
        while self.snaps.len() > 2 && sender_time(self.snaps[1].0) <= t {
            self.snaps.pop_front();
        }
        let a = self.snaps.front()?;
        let (ta, sa) = (sender_time(a.0), a.1);
        match self.snaps.get(1) {
            Some((sb_seq, sb)) if sender_time(*sb_seq) > t => {
                let tb = sender_time(*sb_seq);
                let alpha = (t - ta) as f32 / (tb - ta).max(1) as f32;
                Some(make(&sa, sb, alpha.clamp(0.0, 1.0), 0, tele))
            }
            _ => {
                // newest snapshot is behind t: extrapolate with a cap
                let (last_seq, last) = self.snaps.back()?;
                let ahead = (t - sender_time(*last_seq)).clamp(0, EXTRAPOLATE_CAP_MS);
                Some(make(last, last, 0.0, ahead, tele))
            }
        }
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn make(a: &AvatarState, b: &AvatarState, alpha: f32, extrap_ms: i64, teleported: bool) -> Sample {
    let mut pos = [0.0; 3];
    let mut vel = [0.0; 3];
    for i in 0..3 {
        pos[i] = lerp(dequant_pos(a.pos[i]), dequant_pos(b.pos[i]), alpha);
        vel[i] = lerp(a.vel[i] as f32, b.vel[i] as f32, alpha) / VEL_Q;
        pos[i] += vel[i] * extrap_ms as f32 / 1000.0;
    }
    let dh = b.heading.wrapping_sub(a.heading) as i16 as f32;
    let heading = dequant_heading((a.heading as f32 + dh * alpha).rem_euclid(65536.0) as u16);
    Sample {
        cell: a.cell,
        pos,
        heading,
        vel,
        anim: a.anim,
        weapon: a.weapon,
        health: a.health,
        flags: a.flags,
        seat: a.seat,
        extrapolated: extrap_ms > 0,
        teleported,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::avatar::quant_pos;

    fn st(x: f32, vx: f32) -> AvatarState {
        AvatarState {
            cell: CellId(1),
            pos: [quant_pos(x), 0, 0],
            vel: [(vx * VEL_Q) as i16, 0, 0],
            ..Default::default()
        }
    }

    /// Feeds a constant-velocity walker at 20 Hz with `delay` ms latency.
    fn feed(b: &mut InterpBuffer, from: u32, to: u32, speed: f32, delay: u64) {
        for q in from..to {
            let t = q as u64 * 50;
            b.push(t + delay, Seq(q), st(speed * t as f32 / 1000.0, speed));
        }
    }

    #[test]
    fn interpolates_a_walker_smoothly() {
        let mut b = InterpBuffer::new();
        feed(&mut b, 1, 41, 100.0, 30);
        for now in (1500..2030).step_by(7) {
            let s = b.sample(now).unwrap();
            assert!(!s.extrapolated);
            let want = 100.0 * (now as f32 - 30.0 - DELAY_MS as f32) / 1000.0;
            assert!(
                (s.pos[0] - want).abs() < 1.0,
                "now {now}: {} vs {want}",
                s.pos[0]
            );
        }
    }

    #[test]
    fn the_furniture_in_use_reaches_the_sample() {
        let mut b = InterpBuffer::new();
        for q in 1..6u32 {
            let mut s = st(0.0, 0.0);
            s.seat = 0x00_1234;
            b.push(q as u64 * 50, Seq(q), s);
        }
        assert_eq!(b.sample(400).unwrap().seat, 0x1234);
        let mut up = st(0.0, 0.0);
        up.seat = 0;
        b.push(300, Seq(6), up);
        assert_eq!(b.sample(300 + DELAY_MS as u64 + 400).unwrap().seat, 0);
    }

    #[test]
    fn extrapolates_then_holds() {
        let mut b = InterpBuffer::new();
        feed(&mut b, 1, 21, 40.0, 0); // data until t = 1000
        let s = b.sample(1000 + DELAY_MS as u64 + 100).unwrap();
        assert!(s.extrapolated);
        let p2 = b.sample(1000 + DELAY_MS as u64 + 5000).unwrap().pos[0];
        // capped: never more than EXTRAPOLATE_CAP_MS of travel past the last sample
        assert!(
            (p2 - 40.0 - 40.0 * EXTRAPOLATE_CAP_MS as f32 / 1000.0).abs() < 1.0,
            "{p2}"
        );
        assert!(p2 > s.pos[0]);
    }

    #[test]
    fn teleport_on_cell_change_snaps_once() {
        let mut b = InterpBuffer::new();
        feed(&mut b, 1, 11, 10.0, 0);
        let mut other = st(5.0, 0.0);
        other.cell = CellId(2);
        b.push(560, Seq(11), other);
        assert_eq!(b.len(), 1);
        let s = b.sample(570).unwrap();
        assert!(s.teleported);
        assert_eq!(s.cell, CellId(2));
        assert!((s.pos[0] - 5.0).abs() < 0.1, "no blending across the jump");
        assert!(!b.sample(580).unwrap().teleported, "flag fires once");
        assert_eq!(b.teleports, 1);
    }

    #[test]
    fn teleport_on_distance() {
        let mut b = InterpBuffer::new();
        b.push(0, Seq(1), st(0.0, 0.0));
        b.push(50, Seq(2), st(TELEPORT_DIST + 100.0, 0.0));
        assert_eq!(b.teleports, 1);
    }

    #[test]
    fn duplicates_and_reordering() {
        let mut b = InterpBuffer::new();
        assert!(b.push(0, Seq(1), st(0.0, 0.0)));
        assert!(!b.push(1, Seq(1), st(0.0, 0.0)));
        assert!(b.push(100, Seq(3), st(2.0, 0.0)));
        assert!(b.push(110, Seq(2), st(1.0, 0.0)), "late but between");
        assert!(
            !b.push(120, Seq(0), st(1.0, 0.0)),
            "older than everything held"
        );
        let seqs: Vec<u32> = b.snaps.iter().map(|(s, _)| s.0).collect();
        assert_eq!(seqs, vec![1, 2, 3]);
    }

    proptest::proptest! {
        /// A walker moving only forward, delivered with random delay, loss,
        /// duplication and reordering, is never drawn moving backwards by more
        /// than the clock-estimate slack.
        #[test]
        fn forward_walker_never_goes_backwards(
            delays in proptest::collection::vec(0u64..400, 120),
            dup in proptest::collection::vec(proptest::bool::weighted(0.1), 120),
            lost in proptest::collection::vec(proptest::bool::weighted(0.15), 120),
        ) {
            let mut events: Vec<(u64, u32)> = Vec::new();
            for q in 1..=120u32 {
                let t = q as u64 * 50;
                if !lost[q as usize - 1] { events.push((t + delays[q as usize - 1], q)); }
                if dup[q as usize - 1] { events.push((t + delays[q as usize - 1] + 90, q)); }
            }
            events.sort();
            let mut b = InterpBuffer::new();
            let mut i = 0;
            let mut last = f32::MIN;
            for now in (0..7000u64).step_by(10) {
                while i < events.len() && events[i].0 <= now {
                    let (_, q) = events[i];
                    b.push(now, Seq(q), st(80.0 * (q as f32 * 0.05), 80.0));
                    i += 1;
                }
                if let Some(s) = b.sample(now) {
                    proptest::prop_assert!(s.pos[0].is_finite());
                    proptest::prop_assert!(s.pos[0] >= last - 8.0, "{} then {}", last, s.pos[0]);
                    last = last.max(s.pos[0]);
                }
            }
        }
    }

    #[test]
    fn heading_takes_the_short_way_round() {
        let mut a = st(0.0, 0.0);
        let mut c = st(0.0, 0.0);
        a.heading = 65_000;
        c.heading = 500;
        let s = make(&a, &c, 0.5, 0, false);
        let h = s.heading / std::f32::consts::TAU;
        assert!(h > 0.99 || h < 0.01, "went the long way: {h}");
    }

    #[test]
    fn restarted_sender_resets() {
        let mut b = InterpBuffer::new();
        feed(&mut b, 5000, 5010, 0.0, 0);
        assert!(b.push(1_000_000, Seq(1), st(9.0, 0.0)));
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn empty_buffer_samples_none() {
        assert!(InterpBuffer::new().sample(100).is_none());
    }
}

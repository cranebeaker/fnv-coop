//! Where a copy of an actor another game runs is shown (docs/coop/DECISIONS.md D6, stage B): the
//! owner's states, played back a little behind its clock so that the copy moves smoothly along
//! the owner's path instead of jumping to each state as it arrives.
//!
//! The owner sends a state about every 50 ms while the actor changes, with the time on the
//! owner's clock (`ActorState::ms`), and none while it stands still. A [`Track`] keeps the recent
//! states; [`Track::at`] answers the place at `now - DELAY_MS` on the owner's clock, between the
//! two states around that time. The owner's clock is matched to this one by the smallest
//! difference between a state's arrival and its time seen so far (the quickest delivery), which
//! is let rise slowly in case the two clocks drift. Nothing is guessed past the newest state: an
//! actor whose states stop is shown where the last one put it.
//!
//! A gap between two states longer than [`GAP_MS`] means the actor stood still in between (no
//! state is sent while nothing changes): the earlier place is held until one send interval
//! before the later state, and only then does the copy move on. Two states further apart than
//! [`JUMP_DIST`] are a jump of the owner's own (a door, a `MoveTo`): the copy jumps with it rather
//! than sliding across.

use std::collections::VecDeque;

use coop_core::actors::ActorState;
use coop_core::avatar::{dequant_heading, dequant_pos};

/// How far behind the owner's clock a copy is shown (ms): two send intervals, so that a state
/// late by up to one interval still arrives in time.
pub const DELAY_MS: i64 = 100;
/// The owner's send interval (ms).
pub const SEND_MS: i64 = 50;
/// Two states further apart in time than this had the actor standing still in between (ms).
pub const GAP_MS: i64 = 3 * SEND_MS;
/// Two states further apart than this (units) are a jump.
pub const JUMP_DIST: f32 = 1024.0;
/// A state this much older than the newest one is not a late arrival but another game's clock.
pub const BACK_MS: i64 = 2000;
/// How fast the matched clock difference may rise (ms per second of this clock).
const DRIFT_PER_S: i64 = 2;
/// Most states kept.
const MAX: usize = 64;
/// The owner's movement flags: any of its four directions.
pub const MOVE_DIRECTIONS: u16 = 0x0f;
/// The owner's movement flags: sneaking (crouched, moving or not).
pub const MOVE_SNEAKING: u16 = 0x400;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    /// Owner's time, unwrapped (ms).
    t: i64,
    pos: [f32; 3],
    heading: f32,
    /// The owner's movement flags (`ActorState::anim`).
    moves: u16,
}

/// Where the copy is to be shown now.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Place {
    /// Position.
    pub pos: [f32; 3],
    /// Heading, radians.
    pub heading: f32,
    /// The owner's movement flags at that time.
    pub moves: u16,
    /// How long the owner's actor has stood where it is shown (ms; 0 while it moves).
    pub still_ms: i64,
}

impl Place {
    /// The owner's actor walks or runs at that time.
    pub fn moving(&self) -> bool {
        self.moves & MOVE_DIRECTIONS != 0
    }

    /// The owner's actor is crouched at that time.
    pub fn sneaking(&self) -> bool {
        self.moves & MOVE_SNEAKING != 0
    }
}

/// The recent states of one actor from its owner.
#[derive(Debug, Clone, Default)]
pub struct Track {
    points: VecDeque<Point>,
    /// The last owner's time received, as sent (to unwrap the next).
    last_ms: Option<u32>,
    /// Smallest `arrival - owner's time` seen (ms), and when it was last let rise.
    base: Option<(i64, u64)>,
}

fn dist(a: [f32; 3], b: [f32; 3]) -> f32 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f32>().sqrt()
}

/// `a` to `b` (radians) by the shorter way round, at `f` (0 to 1).
fn turn(a: f32, b: f32, f: f32) -> f32 {
    let tau = std::f32::consts::TAU;
    let mut d = (b - a) % tau;
    if d > tau / 2.0 {
        d -= tau;
    } else if d < -tau / 2.0 {
        d += tau;
    }
    a + d * f
}

impl Track {
    /// An empty track.
    pub fn new() -> Track {
        Track::default()
    }

    /// Forgets every state (the actor changed owner: another clock).
    pub fn clear(&mut self) {
        *self = Track::default();
    }

    /// Whether no state is kept.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Takes a state that arrived at `now_ms` (this clock).
    pub fn push(&mut self, now_ms: u64, s: &ActorState) {
        let t = match (self.last_ms, self.points.back()) {
            (Some(last), Some(p)) => p.t + i64::from(s.ms.wrapping_sub(last) as i32),
            _ => i64::from(s.ms),
        };
        if self.points.back().is_some_and(|p| t <= p.t) {
            if self.points.back().is_some_and(|p| p.t - t <= BACK_MS) {
                // Old or the same: states can arrive out of order.
                return;
            }
            // Far back: another game's clock. An actor's first state after it changes hands is
            // the old owner's, on the old owner's clock; a game that joined later has a clock
            // far behind, and every state of its own was left out as old, so the copy stood
            // where the hand-off left it for as long as that game ran it (the owner,
            // 2026-11-16: Sunny walked to the well in the guest's game, which ran her, and
            // stayed behind in the host's; Cheyenne followed the one that stayed). The track
            // starts again from this state.
            self.clear();
            return self.push(now_ms, s);
        }
        self.last_ms = Some(s.ms);
        let now = now_ms as i64;
        let off = now - t;
        self.base = Some(match self.base {
            None => (off, now_ms),
            Some((b, _)) if off < b => (off, now_ms),
            Some((b, at)) => {
                // Let it rise slowly towards later deliveries (the clocks may drift apart).
                let secs = now_ms.saturating_sub(at) as i64 / 1000;
                if secs > 0 {
                    ((b + secs * DRIFT_PER_S).min(off), now_ms)
                } else {
                    (b, at)
                }
            }
        });
        let point = Point {
            t,
            pos: s.pos.map(dequant_pos),
            heading: dequant_heading(s.heading),
            moves: s.anim,
        };
        if let Some(last) = self.points.back().copied() {
            if t - last.t > GAP_MS {
                // It stood at the earlier place until just before this state.
                self.points.push_back(Point {
                    t: t - SEND_MS,
                    moves: 0,
                    ..last
                });
            }
        }
        self.points.push_back(point);
        while self.points.len() > MAX {
            self.points.pop_front();
        }
    }

    /// Where the copy is to be shown at `now_ms` (this clock); `None` before the first state.
    pub fn at(&mut self, now_ms: u64) -> Option<Place> {
        let (base, _) = self.base?;
        let rt = now_ms as i64 - base - DELAY_MS;
        // Drop what is behind the pair around `rt`.
        while self.points.len() > 2 && self.points[1].t <= rt {
            self.points.pop_front();
        }
        let first = *self.points.front()?;
        let last = *self.points.back()?;
        if rt >= last.t {
            let still_ms = if last.moves & MOVE_DIRECTIONS == 0 {
                rt - last.t
            } else {
                0
            };
            return Some(Place {
                pos: last.pos,
                heading: last.heading,
                moves: last.moves,
                still_ms,
            });
        }
        if rt <= first.t || self.points.len() < 2 {
            return Some(Place {
                pos: first.pos,
                heading: first.heading,
                moves: first.moves,
                still_ms: 0,
            });
        }
        let (a, b) = (self.points[0], self.points[1]);
        if dist(a.pos, b.pos) > JUMP_DIST {
            // The owner's own jump: no slide across.
            return Some(Place {
                pos: a.pos,
                heading: a.heading,
                moves: a.moves,
                still_ms: 0,
            });
        }
        let f = ((rt - a.t) as f32 / (b.t - a.t) as f32).clamp(0.0, 1.0);
        Some(Place {
            pos: [0, 1, 2].map(|i| a.pos[i] + (b.pos[i] - a.pos[i]) * f),
            heading: turn(a.heading, b.heading, f),
            moves: b.moves,
            still_ms: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use coop_core::avatar::{quant_heading, quant_pos};

    fn state(ms: u32, x: f32, moves: u16) -> ActorState {
        ActorState {
            pos: [x, 0.0, 0.0].map(quant_pos),
            heading: quant_heading(0.0),
            anim: moves,
            ms,
            ..ActorState::default()
        }
    }

    fn x(t: &mut Track, now: u64) -> f32 {
        t.at(now).unwrap().pos[0]
    }

    #[test]
    fn a_copy_moves_smoothly_along_the_owners_states_a_little_behind() {
        let mut t = Track::new();
        assert_eq!(t.at(0), None);
        // States every 50 ms of an actor walking 10 units each; each arrives 30 ms late.
        for i in 0..10u32 {
            t.push(u64::from(i * 50 + 30), &state(i * 50, i as f32 * 10.0, 1));
        }
        // At 480 here, 450 on the owner's clock: 100 ms behind is 350, x = 70.
        assert!((x(&mut t, 480) - 70.0).abs() < 0.01);
        // Halfway between two states.
        assert!((x(&mut t, 505) - 75.0).abs() < 0.01);
        assert!(t.at(505).unwrap().moving());
    }

    #[test]
    fn a_late_state_still_arrives_in_time_and_an_old_one_is_dropped() {
        let mut t = Track::new();
        t.push(10, &state(0, 0.0, 1));
        t.push(60, &state(50, 10.0, 1));
        // The state of 100 is 40 ms later than the others.
        t.push(150, &state(100, 20.0, 1));
        // Shown at 160 here (50 on the owner's clock): x = 10, smooth.
        assert!((x(&mut t, 160) - 10.0).abs() < 0.01);
        // A copy of an old state changes nothing.
        t.push(170, &state(50, 999.0, 1));
        assert!((x(&mut t, 185) - 15.0).abs() < 0.01);
    }

    #[test]
    fn nothing_is_guessed_past_the_last_state_and_standing_still_is_counted() {
        let mut t = Track::new();
        t.push(0, &state(0, 0.0, 1));
        t.push(50, &state(50, 10.0, 0));
        let p = t.at(400).unwrap();
        assert_eq!(p.pos[0], 10.0);
        assert!(!p.moving());
        assert_eq!(p.still_ms, 400 - 100 - 50);
        // Still moving at its last state, and nothing more came: shown there, not counted still.
        let mut t = Track::new();
        t.push(0, &state(0, 0.0, 1));
        t.push(50, &state(50, 10.0, 1));
        let p = t.at(400).unwrap();
        assert_eq!((p.pos[0], p.still_ms), (10.0, 0));
    }

    #[test]
    fn an_actor_that_stood_still_between_two_states_moves_on_only_at_the_later_one() {
        let mut t = Track::new();
        t.push(0, &state(0, 0.0, 0));
        // Two seconds later it has walked 10 units.
        t.push(2000, &state(2000, 10.0, 1));
        // Shown at 1900 here (1800 on its clock): still at 0, not sliding over two seconds.
        assert_eq!(x(&mut t, 1900), 0.0);
        // It moves in the last send interval before the new state.
        assert!((x(&mut t, 2075) - 5.0).abs() < 0.01);
        assert_eq!(x(&mut t, 2100), 10.0);
    }

    #[test]
    fn a_jump_of_the_owners_is_a_jump_of_the_copy() {
        let mut t = Track::new();
        t.push(0, &state(0, 0.0, 1));
        t.push(50, &state(50, 5000.0, 1));
        assert_eq!(x(&mut t, 125), 0.0);
        assert_eq!(x(&mut t, 150), 5000.0);
    }

    #[test]
    fn the_owners_clock_wraps_and_drifts() {
        let mut t = Track::new();
        let start = u32::MAX - 60;
        t.push(1000, &state(start, 0.0, 1));
        t.push(1050, &state(start.wrapping_add(50), 10.0, 1));
        t.push(1100, &state(start.wrapping_add(100), 20.0, 1));
        assert!((x(&mut t, 1175) - 15.0).abs() < 0.01);
        // Ten seconds of states that each arrive 20 ms later than the first ones: the matched
        // difference rises by at most a few ms a second, and never past the deliveries.
        let mut t = Track::new();
        t.push(0, &state(0, 0.0, 1));
        for i in 1..=200u32 {
            t.push(u64::from(i * 50 + 20), &state(i * 50, i as f32, 1));
        }
        let (base, _) = t.base.unwrap();
        assert!((10..=20).contains(&base), "{base}");
    }

    /// The owner, 2026-11-16: an actor's first state after it changes hands is the old owner's,
    /// on that game's clock. The new owner's clock was far behind (its game joined later), every
    /// state of its own was dropped as old, and the copy stood still for as long as that game
    /// ran the actor.
    #[test]
    fn states_on_a_clock_far_behind_start_the_track_again() {
        let mut t = Track::new();
        // The old owner's last state, on its clock (400 s).
        t.push(1000, &state(400_000, 0.0, 1));
        // The new owner's states, on a clock 300 s behind.
        t.push(1050, &state(100_000, 100.0, 1));
        t.push(1100, &state(100_050, 110.0, 1));
        t.push(1150, &state(100_100, 120.0, 1));
        assert_eq!(x(&mut t, 1200), 110.0);
        assert_eq!(x(&mut t, 1300), 120.0);
        // A state a little late is still only late.
        t.push(1310, &state(100_060, 999.0, 1));
        assert_eq!(x(&mut t, 1320), 120.0);
    }

    #[test]
    fn a_quicker_delivery_sets_the_clock_match() {
        let mut t = Track::new();
        // The first state is 60 ms late; the next ones only 10.
        t.push(60, &state(0, 0.0, 1));
        t.push(60, &state(50, 10.0, 1));
        t.push(110, &state(100, 20.0, 1));
        // Shown 100 ms behind the quick deliveries: at 160 here, 50 on the owner's clock.
        assert!((x(&mut t, 160) - 10.0).abs() < 0.01);
    }

    #[test]
    fn headings_turn_the_short_way_round() {
        let tau = std::f32::consts::TAU;
        assert!((turn(0.1, tau - 0.1, 0.5) - 0.0).abs() < 1e-5);
        assert!((turn(1.0, 2.0, 0.5) - 1.5).abs() < 1e-5);
    }
}

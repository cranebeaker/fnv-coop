//! Glitch detectors over the remote-avatar stream, as this peer draws it.
//!
//! The live layer hands the viewer one [`RemoteAvatarView`] per other player
//! and frame: the interpolated position, heading and animation id the
//! puppet is drawn with. [`GlitchWatch`] reads that stream frame by frame and
//! counts three kinds of visible glitch:
//!
//! * a position pop: the drawn position jumps farther than
//!   [`POP_DIST`] (plus a plausible run for the frame time) in one frame,
//!   with no teleport flag and no cell change;
//! * a heading snap: the drawn heading turns more than [`SNAP_RAD`] (plus a
//!   plausible turn for the frame time) in one frame, with no teleport;
//! * an animation flip-flop: the drawn animation id goes back to the value
//!   it just left, [`FLIP_MIN`] times inside [`FLIP_WINDOW_MS`], each return
//!   within [`FLIP_GAP_MS`] of the change before it (one burst counts once).
//!
//! The detectors see what the remote peer draws from the replicated stream.
//! They do not see rendered pixels or the skeleton pose.

use std::collections::{BTreeMap, VecDeque};

use coop_core::ids::PlayerId;

use crate::live::RemoteAvatarView;

/// A drawn jump longer than this (world units) in one frame, on top of
/// [`PLAUSIBLE_SPEED`] times the frame time, is a pop.
pub const POP_DIST: f32 = 200.0;
/// Fastest plausible avatar movement, units per second (a sprint is about 300).
pub const PLAUSIBLE_SPEED: f32 = 1200.0;
/// A drawn turn larger than this (radians) in one frame, on top of
/// [`PLAUSIBLE_TURN`] times the frame time, is a snap.
pub const SNAP_RAD: f32 = 1.0;
/// Fastest plausible smooth turn, radians per second.
pub const PLAUSIBLE_TURN: f32 = 6.0;
/// A change back to the previous animation within this many ms is a return.
pub const FLIP_GAP_MS: u64 = 150;
/// The window in which returns are counted.
pub const FLIP_WINDOW_MS: u64 = 1000;
/// Returns inside the window that make one flip-flop.
pub const FLIP_MIN: usize = 4;
/// A gap in the stream longer than this starts the track again (no pop is
/// reported for a player who was out of view).
pub const GAP_MS: u64 = 500;

/// Counts of each glitch kind (the dump's `live.glitch`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GlitchCounts {
    /// Position pops.
    pub pops: u64,
    /// Heading snaps.
    pub snaps: u64,
    /// Animation flip-flops (bursts).
    pub flips: u64,
}

impl GlitchCounts {
    /// `{"pops":N,"snaps":N,"flips":N}`.
    pub fn json(&self) -> String {
        format!(
            "{{\"pops\":{},\"snaps\":{},\"flips\":{}}}",
            self.pops, self.snaps, self.flips
        )
    }

    /// The sum of every kind.
    pub fn total(&self) -> u64 {
        self.pops + self.snaps + self.flips
    }
}

/// The kind of one glitch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlitchKind {
    /// Position pop.
    Pop,
    /// Heading snap.
    Snap,
    /// Animation flip-flop.
    Flip,
}

/// One glitch the detectors saw.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Glitch {
    /// What kind.
    pub kind: GlitchKind,
    /// Whose avatar.
    pub player: PlayerId,
    /// Session time of the frame, ms.
    pub now_ms: u64,
    /// Size: units (pop), radians (snap), returns in the window (flip).
    pub size: f32,
}

#[derive(Clone, Debug, Default)]
struct Track {
    /// Last drawn frame: time, cell, position, heading.
    last: Option<(u64, u32, [f32; 3], f32)>,
    /// Current and previous animation id, with the time of the last change.
    anim: Option<u16>,
    prev_anim: Option<u16>,
    changed_at: u64,
    /// Times of returns to the previous animation.
    returns: VecDeque<u64>,
    /// A flip-flop burst is open (counted once).
    in_burst: bool,
}

/// The detectors for every remote avatar one peer draws.
#[derive(Clone, Debug, Default)]
pub struct GlitchWatch {
    tracks: BTreeMap<PlayerId, Track>,
    /// Totals so far.
    pub counts: GlitchCounts,
    /// Every glitch seen, oldest first (capped at [`GlitchWatch::LOG_CAP`]).
    pub log: Vec<Glitch>,
}

fn turn(a: f32, b: f32) -> f32 {
    let d = (b - a).rem_euclid(std::f32::consts::TAU);
    d.min(std::f32::consts::TAU - d)
}

impl GlitchWatch {
    /// Most glitches kept in the log.
    pub const LOG_CAP: usize = 256;

    /// A watch with nothing seen.
    pub fn new() -> GlitchWatch {
        GlitchWatch::default()
    }

    fn note(&mut self, kind: GlitchKind, player: PlayerId, now_ms: u64, size: f32) {
        match kind {
            GlitchKind::Pop => self.counts.pops += 1,
            GlitchKind::Snap => self.counts.snaps += 1,
            GlitchKind::Flip => self.counts.flips += 1,
        }
        if self.log.len() < Self::LOG_CAP {
            self.log.push(Glitch {
                kind,
                player,
                now_ms,
                size,
            });
        }
    }

    /// Reads one drawn frame: every remote avatar as the viewer draws it at
    /// session time `now_ms`.
    pub fn frame(&mut self, now_ms: u64, avatars: &[RemoteAvatarView]) {
        let mut found: Vec<(GlitchKind, PlayerId, f32)> = Vec::new();
        for a in avatars {
            let tr = self.tracks.entry(a.player).or_default();
            if let Some((t0, cell, pos, heading)) = tr.last {
                let gap = now_ms.saturating_sub(t0);
                if gap > 0 && gap <= GAP_MS && !a.teleported && cell == a.cell {
                    let dt = gap as f32 / 1000.0;
                    let moved = (0..3)
                        .map(|i| (a.pos[i] - pos[i]).powi(2))
                        .sum::<f32>()
                        .sqrt();
                    if moved > POP_DIST + PLAUSIBLE_SPEED * dt {
                        found.push((GlitchKind::Pop, a.player, moved));
                    }
                    let turned = turn(heading, a.heading);
                    if turned > SNAP_RAD + PLAUSIBLE_TURN * dt {
                        found.push((GlitchKind::Snap, a.player, turned));
                    }
                }
            }
            if tr.last.is_some_and(|l| now_ms.saturating_sub(l.0) > GAP_MS) {
                tr.anim = None;
                tr.prev_anim = None;
                tr.returns.clear();
                tr.in_burst = false;
            }
            tr.last = Some((now_ms, a.cell, a.pos, a.heading));
            // animation flip-flops
            match tr.anim {
                None => tr.anim = Some(a.anim),
                Some(cur) if cur != a.anim => {
                    if tr.prev_anim == Some(a.anim)
                        && now_ms.saturating_sub(tr.changed_at) <= FLIP_GAP_MS
                    {
                        tr.returns.push_back(now_ms);
                    }
                    tr.prev_anim = Some(cur);
                    tr.anim = Some(a.anim);
                    tr.changed_at = now_ms;
                }
                Some(_) => {}
            }
            while tr
                .returns
                .front()
                .is_some_and(|t| now_ms.saturating_sub(*t) > FLIP_WINDOW_MS)
            {
                tr.returns.pop_front();
            }
            if tr.returns.len() >= FLIP_MIN {
                if !tr.in_burst {
                    tr.in_burst = true;
                    found.push((GlitchKind::Flip, a.player, tr.returns.len() as f32));
                }
            } else if tr.returns.is_empty() {
                tr.in_burst = false;
            }
        }
        // players no longer drawn start again when they come back
        self.tracks
            .retain(|p, _| avatars.iter().any(|a| a.player == *p));
        for (kind, player, size) in found {
            self.note(kind, player, now_ms, size);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn av(x: f32, heading: f32, anim: u16) -> RemoteAvatarView {
        RemoteAvatarView {
            player: PlayerId(1),
            cell: 7,
            pos: [x, 0.0, 0.0],
            heading,
            anim,
            ..Default::default()
        }
    }

    /// 40 frames a second of a walker at 300 units a second, turning slowly.
    fn walk(w: &mut GlitchWatch, from_ms: u64, frames: u64) {
        for i in 0..frames {
            let t = from_ms + i * 25;
            w.frame(t, &[av(t as f32 * 0.3, t as f32 * 0.001, 3)]);
        }
    }

    #[test]
    fn a_smooth_walker_is_clean() {
        let mut w = GlitchWatch::new();
        walk(&mut w, 0, 400);
        assert_eq!(w.counts, GlitchCounts::default(), "{:?}", w.log);
    }

    #[test]
    fn a_jump_without_a_teleport_is_a_pop_and_with_one_it_is_not() {
        let mut w = GlitchWatch::new();
        walk(&mut w, 0, 40);
        w.frame(1000, &[av(1500.0, 1.0, 3)]);
        assert_eq!(w.counts.pops, 1);
        let mut t = av(9000.0, 1.0, 3);
        t.teleported = true;
        w.frame(1025, &[t]);
        assert_eq!(w.counts.pops, 1, "a flagged teleport is no pop");
    }

    #[test]
    fn a_half_turn_in_one_frame_is_a_snap() {
        let mut w = GlitchWatch::new();
        w.frame(0, &[av(0.0, 0.2, 0)]);
        w.frame(25, &[av(0.0, 0.2 + 1.6, 0)]);
        assert_eq!(w.counts.snaps, 1);
        // the short way round across zero is a small turn
        w.frame(50, &[av(0.0, 6.2, 0)]);
        w.frame(75, &[av(0.0, 0.05, 0)]);
        assert_eq!(
            w.counts.snaps, 2,
            "only the 1.6 rad turn and the 1.6 to 6.2 one"
        );
    }

    #[test]
    fn an_animation_that_flips_every_sample_is_one_flip_flop() {
        let mut w = GlitchWatch::new();
        for i in 0..60u64 {
            let anim = if (i / 2) % 2 == 0 { 3 } else { 0 };
            w.frame(i * 25, &[av(0.0, 0.0, anim)]);
        }
        assert_eq!(w.counts.flips, 1, "{:?}", w.log);
        // a walker that stops and starts once a second is no flip-flop
        let mut w = GlitchWatch::new();
        for i in 0..200u64 {
            let anim = if (i / 40) % 2 == 0 { 3 } else { 0 };
            w.frame(i * 25, &[av(0.0, 0.0, anim)]);
        }
        assert_eq!(w.counts.flips, 0);
    }

    #[test]
    fn a_player_out_of_view_starts_again() {
        let mut w = GlitchWatch::new();
        w.frame(0, &[av(0.0, 0.0, 0)]);
        w.frame(25, &[]);
        w.frame(50, &[av(3000.0, 3.0, 0)]);
        assert_eq!(w.counts.total(), 0);
    }
}

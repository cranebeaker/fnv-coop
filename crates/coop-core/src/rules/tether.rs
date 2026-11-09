//! Story tether: during a story-critical moment the party must stay in one
//! area. The tether holds the story stage, never freezes a player; stragglers
//! are warned and then warped after the timeout.

use std::collections::BTreeSet;

use crate::ids::PlayerId;

/// Time a straggler may stay out of the zone before being warped.
pub const TIMEOUT_MS: u64 = 120_000;
/// Warning lead before the warp.
pub const WARN_LEAD_MS: u64 = 10_000;

/// What the tether wants done.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TetherEvent {
    /// Tell these players they will be warped in `in_ms`.
    Warn {
        /// Stragglers.
        players: Vec<PlayerId>,
        /// Milliseconds until the warp.
        in_ms: u64,
    },
    /// Warp these players to the tether anchor.
    Warp {
        /// Stragglers.
        players: Vec<PlayerId>,
        /// Anchor id chosen by the game when the tether started.
        anchor: u32,
    },
    /// Everyone is in the zone; the story stage may proceed.
    Cleared,
}

/// A story tether.
#[derive(Clone, Debug, Default)]
pub struct Tether {
    active: bool,
    anchor: u32,
    members: BTreeSet<PlayerId>,
    in_zone: BTreeSet<PlayerId>,
    since_ms: Option<u64>,
    warned: bool,
    held: bool,
}

impl Tether {
    /// Inactive tether.
    pub fn new() -> Self {
        Self::default()
    }

    /// True while a tether is in force.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Starts a tether for `members`; `in_zone` are already inside.
    pub fn start(&mut self, now_ms: u64, anchor: u32, members: &[PlayerId], in_zone: &[PlayerId]) {
        self.active = true;
        self.anchor = anchor;
        self.members = members.iter().copied().collect();
        self.in_zone = in_zone
            .iter()
            .copied()
            .filter(|p| self.members.contains(p))
            .collect();
        self.since_ms = None;
        self.warned = false;
        self.held = false;
        self.refresh(now_ms);
    }

    /// Ends the tether.
    pub fn end(&mut self) {
        *self = Tether::default();
    }

    /// Records whether a member is inside the zone.
    pub fn set_in_zone(&mut self, p: PlayerId, inside: bool, now_ms: u64) -> Vec<TetherEvent> {
        if !self.active || !self.members.contains(&p) {
            return vec![];
        }
        if inside {
            self.in_zone.insert(p);
        } else {
            self.in_zone.remove(&p);
        }
        self.refresh(now_ms);
        self.take_cleared()
    }

    /// A member left the session: it no longer holds the party back.
    pub fn remove_member(&mut self, p: PlayerId, now_ms: u64) -> Vec<TetherEvent> {
        self.members.remove(&p);
        self.in_zone.remove(&p);
        if !self.active {
            return vec![];
        }
        self.refresh(now_ms);
        self.take_cleared()
    }

    /// Members outside the zone.
    pub fn stragglers(&self) -> Vec<PlayerId> {
        if !self.active {
            return vec![];
        }
        self.members
            .iter()
            .filter(|p| !self.in_zone.contains(p))
            .copied()
            .collect()
    }

    /// True while the story stage must wait for the party.
    pub fn stage_held(&self) -> bool {
        self.active && !self.stragglers().is_empty()
    }

    fn refresh(&mut self, now_ms: u64) {
        let any = !self.stragglers().is_empty();
        match (any, self.since_ms) {
            (true, None) => {
                self.since_ms = Some(now_ms);
                self.warned = false;
                self.held = true;
            }
            (false, Some(_)) => {
                self.since_ms = None;
                self.warned = false;
            }
            _ => {}
        }
    }

    fn take_cleared(&mut self) -> Vec<TetherEvent> {
        if self.held && self.since_ms.is_none() {
            self.held = false;
            vec![TetherEvent::Cleared]
        } else {
            vec![]
        }
    }

    /// Advances the clock: warns, then warps.
    pub fn tick(&mut self, now_ms: u64) -> Vec<TetherEvent> {
        let Some(since) = self.since_ms.filter(|_| self.active) else {
            return vec![];
        };
        let waited = now_ms.saturating_sub(since);
        let stragglers = self.stragglers();
        if waited >= TIMEOUT_MS {
            // the game moves them; if they somehow stay out the clock restarts
            self.since_ms = Some(now_ms);
            self.warned = false;
            return vec![TetherEvent::Warp {
                players: stragglers,
                anchor: self.anchor,
            }];
        }
        if waited >= TIMEOUT_MS - WARN_LEAD_MS && !self.warned {
            self.warned = true;
            return vec![TetherEvent::Warn {
                players: stragglers,
                in_ms: TIMEOUT_MS - waited,
            }];
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: PlayerId = PlayerId(0);
    const B: PlayerId = PlayerId(1);
    const C: PlayerId = PlayerId(2);

    #[test]
    fn straggler_is_warned_then_warped() {
        let mut t = Tether::new();
        t.start(1000, 7, &[A, B, C], &[A, B]);
        assert!(t.stage_held());
        assert_eq!(t.stragglers(), vec![C]);
        assert!(t.tick(1000 + TIMEOUT_MS - WARN_LEAD_MS - 1).is_empty());
        let w = t.tick(1000 + TIMEOUT_MS - WARN_LEAD_MS);
        assert_eq!(
            w,
            vec![TetherEvent::Warn {
                players: vec![C],
                in_ms: WARN_LEAD_MS
            }]
        );
        assert!(t.tick(1000 + TIMEOUT_MS - 1).is_empty(), "warn only once");
        let w = t.tick(1000 + TIMEOUT_MS);
        assert_eq!(
            w,
            vec![TetherEvent::Warp {
                players: vec![C],
                anchor: 7
            }]
        );
    }

    #[test]
    fn arriving_clears_hold_and_resets_clock() {
        let mut t = Tether::new();
        t.start(0, 1, &[A, B], &[A]);
        assert_eq!(t.set_in_zone(B, true, 5000), vec![TetherEvent::Cleared]);
        assert!(!t.stage_held());
        assert!(t.tick(TIMEOUT_MS * 2).is_empty());
        // stepping out again starts a fresh clock
        t.set_in_zone(B, false, 200_000);
        assert!(t.tick(200_000 + TIMEOUT_MS - 1).len() <= 1);
        assert!(matches!(
            t.tick(200_000 + TIMEOUT_MS)[0],
            TetherEvent::Warp { .. }
        ));
    }

    #[test]
    fn leaving_player_stops_holding_the_party() {
        let mut t = Tether::new();
        t.start(0, 1, &[A, B], &[A]);
        assert_eq!(t.remove_member(B, 10), vec![TetherEvent::Cleared]);
        assert!(!t.stage_held());
    }

    #[test]
    fn warp_restarts_the_clock_if_player_stays_out() {
        let mut t = Tether::new();
        t.start(0, 1, &[A, B], &[A]);
        t.tick(TIMEOUT_MS - WARN_LEAD_MS);
        assert!(matches!(t.tick(TIMEOUT_MS)[0], TetherEvent::Warp { .. }));
        assert!(t.tick(TIMEOUT_MS + 1).is_empty());
        assert!(matches!(
            t.tick(2 * TIMEOUT_MS)[0],
            TetherEvent::Warp { .. }
        ));
    }

    #[test]
    fn inactive_and_outsiders_are_ignored() {
        let mut t = Tether::new();
        assert!(t.set_in_zone(A, true, 0).is_empty());
        assert!(t.tick(1_000_000).is_empty());
        t.start(0, 1, &[A], &[A]);
        assert!(t.set_in_zone(C, false, 1).is_empty());
        assert!(!t.stage_held());
        t.end();
        assert!(!t.is_active());
    }
}

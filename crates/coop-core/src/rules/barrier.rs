//! Unanimous barrier for one-way transitions (DLC entry, point of no
//! return): every player present when it opens must agree, then everyone is
//! warped together. Nobody is left behind; a timeout cancels.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::ids::PlayerId;

/// Time the party has to agree.
pub const TIMEOUT_MS: u64 = 120_000;

/// Why a barrier did not pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CancelReason {
    /// A player said no.
    Declined,
    /// Not everyone agreed in time.
    Timeout,
    /// Everyone required left.
    Empty,
}

/// Where a barrier stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarrierState {
    /// Waiting for votes.
    Waiting,
    /// Everyone agreed: warp the whole party.
    Released,
    /// Cancelled; nobody moves.
    Cancelled(CancelReason),
}

/// One barrier.
#[derive(Clone, Debug)]
pub struct Barrier {
    /// Barrier id.
    pub id: u32,
    /// Game-defined tag (which transition).
    pub tag: u32,
    required: BTreeSet<PlayerId>,
    ready: BTreeSet<PlayerId>,
    started_ms: u64,
    state: BarrierState,
}

impl Barrier {
    /// Opens a barrier requiring every player in `members` at this moment.
    pub fn new(id: u32, tag: u32, members: &[PlayerId], now_ms: u64) -> Self {
        let mut b = Barrier {
            id,
            tag,
            required: members.iter().copied().collect(),
            ready: BTreeSet::new(),
            started_ms: now_ms,
            state: BarrierState::Waiting,
        };
        b.settle();
        b
    }

    /// Current state.
    pub fn state(&self) -> BarrierState {
        self.state
    }

    /// Players the barrier waits for.
    pub fn required(&self) -> Vec<PlayerId> {
        self.required.iter().copied().collect()
    }

    /// Players not yet ready.
    pub fn missing(&self) -> Vec<PlayerId> {
        self.required.difference(&self.ready).copied().collect()
    }

    fn settle(&mut self) {
        if self.state != BarrierState::Waiting {
            return;
        }
        if self.required.is_empty() {
            self.state = BarrierState::Cancelled(CancelReason::Empty);
        } else if self.required.is_subset(&self.ready) {
            self.state = BarrierState::Released;
        }
    }

    /// A player agrees. Players who joined after the barrier opened are not
    /// required and are ignored.
    pub fn ready(&mut self, p: PlayerId) -> BarrierState {
        if self.state == BarrierState::Waiting && self.required.contains(&p) {
            self.ready.insert(p);
            self.settle();
        }
        self.state
    }

    /// A required player refuses: the barrier is cancelled.
    pub fn decline(&mut self, p: PlayerId) -> BarrierState {
        if self.state == BarrierState::Waiting && self.required.contains(&p) {
            self.state = BarrierState::Cancelled(CancelReason::Declined);
        }
        self.state
    }

    /// A player disconnected: it stops counting at once.
    pub fn leave(&mut self, p: PlayerId) -> BarrierState {
        self.required.remove(&p);
        self.ready.remove(&p);
        self.settle();
        self.state
    }

    /// Cancels on timeout.
    pub fn tick(&mut self, now_ms: u64) -> BarrierState {
        if self.state == BarrierState::Waiting
            && now_ms.saturating_sub(self.started_ms) >= TIMEOUT_MS
        {
            self.state = BarrierState::Cancelled(CancelReason::Timeout);
        }
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn ps(n: u8) -> Vec<PlayerId> {
        (0..n).map(PlayerId).collect()
    }

    #[test]
    fn releases_only_when_everyone_is_ready() {
        let mut b = Barrier::new(1, 9, &ps(3), 0);
        assert_eq!(b.ready(PlayerId(0)), BarrierState::Waiting);
        assert_eq!(
            b.ready(PlayerId(0)),
            BarrierState::Waiting,
            "double ready counts once"
        );
        assert_eq!(b.ready(PlayerId(1)), BarrierState::Waiting);
        assert_eq!(b.missing(), vec![PlayerId(2)]);
        assert_eq!(b.ready(PlayerId(2)), BarrierState::Released);
    }

    #[test]
    fn decline_cancels_and_is_final() {
        let mut b = Barrier::new(1, 0, &ps(2), 0);
        assert_eq!(
            b.decline(PlayerId(1)),
            BarrierState::Cancelled(CancelReason::Declined)
        );
        assert_eq!(
            b.ready(PlayerId(0)),
            BarrierState::Cancelled(CancelReason::Declined)
        );
        assert_eq!(
            b.tick(10 * TIMEOUT_MS),
            BarrierState::Cancelled(CancelReason::Declined)
        );
    }

    #[test]
    fn leaver_stops_counting() {
        let mut b = Barrier::new(1, 0, &ps(3), 0);
        b.ready(PlayerId(0));
        b.ready(PlayerId(1));
        assert_eq!(b.leave(PlayerId(2)), BarrierState::Released);
        let mut b = Barrier::new(2, 0, &ps(1), 0);
        assert_eq!(
            b.leave(PlayerId(0)),
            BarrierState::Cancelled(CancelReason::Empty)
        );
    }

    #[test]
    fn timeout_cancels() {
        let mut b = Barrier::new(1, 0, &ps(2), 1000);
        b.ready(PlayerId(0));
        assert_eq!(b.tick(1000 + TIMEOUT_MS - 1), BarrierState::Waiting);
        assert_eq!(
            b.tick(1000 + TIMEOUT_MS),
            BarrierState::Cancelled(CancelReason::Timeout)
        );
    }

    #[test]
    fn late_joiner_is_not_required_and_cannot_vote() {
        let mut b = Barrier::new(1, 0, &ps(2), 0);
        assert_eq!(b.ready(PlayerId(5)), BarrierState::Waiting);
        assert_eq!(b.decline(PlayerId(5)), BarrierState::Waiting);
        assert!(!b.required().contains(&PlayerId(5)));
        b.ready(PlayerId(0));
        assert_eq!(b.ready(PlayerId(1)), BarrierState::Released);
    }

    proptest! {
        /// Released implies every player present at the end (required) agreed;
        /// a released barrier never has a missing player.
        #[test]
        fn released_means_nobody_missing(
            n in 1u8..5,
            ops in proptest::collection::vec((0u8..3, 0u8..6), 0..40),
        ) {
            let mut b = Barrier::new(1, 0, &ps(n), 0);
            for (op, who) in ops {
                let p = PlayerId(who);
                match op { 0 => { b.ready(p); } 1 => { b.leave(p); } _ => { b.decline(p); } }
                if b.state() == BarrierState::Released {
                    prop_assert!(b.missing().is_empty());
                }
            }
        }
    }
}

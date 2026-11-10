//! Downed and revive: HP 0 is downed (no death screen); a friend holding the
//! activate key for 3 s within range revives at 30 percent health; after 60 s
//! the player respawns at the party checkpoint; all down means a wipe.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::ids::PlayerId;

/// Time a downed player can be revived.
pub const DOWN_WINDOW_MS: u64 = 60_000;
/// Time the helper must keep holding.
pub const HOLD_MS: u64 = 3_000;
/// Range the client uses to start a revive (informational for the UI).
pub const CLIENT_RANGE: f32 = 150.0;
/// Range the host enforces (generous against latency).
pub const HOST_RANGE: f32 = 600.0;
/// Health fraction a revived player returns with, in percent.
pub const REVIVE_PCT: u8 = 30;

/// Why a revive could not begin.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviveDeny {
    /// Target is not downed, or cannot be revived.
    NotRevivable,
    /// Someone is already reviving the target.
    Busy,
    /// Helper is itself downed, unknown, or reviving someone else.
    BadHelper,
    /// Too far or in another space.
    OutOfRange,
}

/// Something the game must do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReviveEvent {
    /// Player went down.
    Downed(PlayerId),
    /// Player was revived in place.
    Revived {
        /// The revived player.
        player: PlayerId,
        /// The helper.
        helper: PlayerId,
        /// Health to restore, percent of maximum.
        health_pct: u8,
    },
    /// The window ran out (or the player cannot be revived): respawn at the
    /// party checkpoint.
    Respawn(PlayerId),
    /// A helper's attempt ended without a revive.
    Cancelled {
        /// Target.
        target: PlayerId,
        /// Helper.
        helper: PlayerId,
    },
    /// Everyone is down: reload the host save.
    Wipe,
}

#[derive(Clone, Copy, Debug)]
enum Status {
    Up,
    Downed {
        since: u64,
        helper: Option<(PlayerId, u64)>,
    },
}

/// Revive bookkeeping on the host clock.
#[derive(Clone, Debug, Default)]
pub struct Reviver {
    players: BTreeMap<PlayerId, Status>,
    wiped: bool,
}

impl Reviver {
    /// Empty tracker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a player to the party.
    pub fn add(&mut self, p: PlayerId) {
        self.players.entry(p).or_insert(Status::Up);
    }

    /// True when the player is downed.
    pub fn is_downed(&self, p: PlayerId) -> bool {
        matches!(self.players.get(&p), Some(Status::Downed { .. }))
    }

    /// The helper currently reviving `target`.
    pub fn helper_of(&self, target: PlayerId) -> Option<PlayerId> {
        match self.players.get(&target) {
            Some(Status::Downed {
                helper: Some((h, _)),
                ..
            }) => Some(*h),
            _ => None,
        }
    }

    /// A player hit 0 health. `revivable` is false for collar deaths and
    /// respawn-mode campaigns.
    pub fn set_down(&mut self, p: PlayerId, now_ms: u64, revivable: bool) -> Vec<ReviveEvent> {
        if !self.players.contains_key(&p) || self.is_downed(p) {
            return vec![];
        }
        if !revivable {
            return vec![ReviveEvent::Respawn(p)];
        }
        self.players.insert(
            p,
            Status::Downed {
                since: now_ms,
                helper: None,
            },
        );
        let mut v = vec![ReviveEvent::Downed(p)];
        v.extend(self.check_wipe());
        v
    }

    fn check_wipe(&mut self) -> Option<ReviveEvent> {
        let all = !self.players.is_empty()
            && self
                .players
                .values()
                .all(|s| matches!(s, Status::Downed { .. }));
        if all && !self.wiped {
            self.wiped = true;
            return Some(ReviveEvent::Wipe);
        }
        if !all {
            self.wiped = false;
        }
        None
    }

    /// Starts a revive. `dist` is the host's measured distance (None when in
    /// different spaces or unknown).
    pub fn begin(
        &mut self,
        helper: PlayerId,
        target: PlayerId,
        now_ms: u64,
        dist: Option<f32>,
    ) -> Result<(), ReviveDeny> {
        let helper_ok = matches!(self.players.get(&helper), Some(Status::Up))
            && helper != target
            && !self
                .players
                .values()
                .any(|s| matches!(s, Status::Downed { helper: Some((h, _)), .. } if *h == helper));
        if !helper_ok {
            return Err(ReviveDeny::BadHelper);
        }
        match self.players.get_mut(&target) {
            Some(Status::Downed { helper: slot, .. }) => {
                if slot.is_some() {
                    return Err(ReviveDeny::Busy);
                }
                if !dist.is_some_and(|d| d <= HOST_RANGE) {
                    return Err(ReviveDeny::OutOfRange);
                }
                *slot = Some((helper, now_ms));
                Ok(())
            }
            _ => Err(ReviveDeny::NotRevivable),
        }
    }

    /// The helper let go.
    pub fn cancel(&mut self, helper: PlayerId) -> Option<ReviveEvent> {
        for (t, s) in self.players.iter_mut() {
            if let Status::Downed { helper: slot, .. } = s {
                if slot.map(|(h, _)| h) == Some(helper) {
                    *slot = None;
                    return Some(ReviveEvent::Cancelled { target: *t, helper });
                }
            }
        }
        None
    }

    /// A player disconnected.
    pub fn remove(&mut self, p: PlayerId) -> Vec<ReviveEvent> {
        let mut v: Vec<_> = self.cancel(p).into_iter().collect();
        self.players.remove(&p);
        v.extend(self.check_wipe());
        v
    }

    /// Advances the host clock; `dist(helper, target)` re-measures range.
    pub fn tick(
        &mut self,
        now_ms: u64,
        dist: impl Fn(PlayerId, PlayerId) -> Option<f32>,
    ) -> Vec<ReviveEvent> {
        let mut ev = Vec::new();
        let ids: Vec<PlayerId> = self.players.keys().copied().collect();
        for t in ids {
            let Some(Status::Downed { since, helper }) = self.players.get(&t).copied() else {
                continue;
            };
            if now_ms.saturating_sub(since) >= DOWN_WINDOW_MS {
                if let Some((h, _)) = helper {
                    ev.push(ReviveEvent::Cancelled {
                        target: t,
                        helper: h,
                    });
                }
                self.players.insert(t, Status::Up);
                ev.push(ReviveEvent::Respawn(t));
                continue;
            }
            let Some((h, began)) = helper else { continue };
            if !dist(h, t).is_some_and(|d| d <= HOST_RANGE)
                || !matches!(self.players.get(&h), Some(Status::Up))
            {
                self.players.insert(
                    t,
                    Status::Downed {
                        since,
                        helper: None,
                    },
                );
                ev.push(ReviveEvent::Cancelled {
                    target: t,
                    helper: h,
                });
            } else if now_ms.saturating_sub(began) >= HOLD_MS {
                self.players.insert(t, Status::Up);
                ev.push(ReviveEvent::Revived {
                    player: t,
                    helper: h,
                    health_pct: REVIVE_PCT,
                });
            }
        }
        if !ev.is_empty() {
            ev.extend(self.check_wipe());
        }
        ev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: PlayerId = PlayerId(0);
    const B: PlayerId = PlayerId(1);
    const C: PlayerId = PlayerId(2);

    fn party() -> Reviver {
        let mut r = Reviver::new();
        for p in [A, B, C] {
            r.add(p);
        }
        r
    }
    fn near(_: PlayerId, _: PlayerId) -> Option<f32> {
        Some(100.0)
    }

    #[test]
    fn hold_three_seconds_revives_at_thirty_percent() {
        let mut r = party();
        assert_eq!(r.set_down(B, 1000, true), vec![ReviveEvent::Downed(B)]);
        assert!(r.begin(A, B, 2000, Some(120.0)).is_ok());
        assert!(r.tick(2000 + HOLD_MS - 1, near).is_empty());
        let ev = r.tick(2000 + HOLD_MS, near);
        assert_eq!(
            ev,
            vec![ReviveEvent::Revived {
                player: B,
                helper: A,
                health_pct: 30
            }]
        );
        assert!(!r.is_downed(B));
    }

    #[test]
    fn window_expiry_respawns() {
        let mut r = party();
        r.set_down(B, 0, true);
        assert!(r.tick(DOWN_WINDOW_MS - 1, near).is_empty());
        assert_eq!(r.tick(DOWN_WINDOW_MS, near), vec![ReviveEvent::Respawn(B)]);
        assert!(!r.is_downed(B));
    }

    #[test]
    fn revive_started_late_still_beats_expiry_only_if_complete() {
        let mut r = party();
        r.set_down(B, 0, true);
        r.begin(A, B, DOWN_WINDOW_MS - 1000, Some(10.0)).unwrap();
        let ev = r.tick(DOWN_WINDOW_MS, near);
        assert!(
            ev.contains(&ReviveEvent::Respawn(B)),
            "the window closes before the hold completes"
        );
        assert!(ev.contains(&ReviveEvent::Cancelled {
            target: B,
            helper: A
        }));
    }

    #[test]
    fn host_range_and_state_validation() {
        let mut r = party();
        assert_eq!(r.begin(A, B, 0, Some(10.0)), Err(ReviveDeny::NotRevivable));
        r.set_down(B, 0, true);
        assert_eq!(
            r.begin(A, B, 0, Some(HOST_RANGE + 1.0)),
            Err(ReviveDeny::OutOfRange)
        );
        assert_eq!(r.begin(A, B, 0, None), Err(ReviveDeny::OutOfRange));
        assert_eq!(r.begin(B, B, 0, Some(0.0)), Err(ReviveDeny::BadHelper));
        assert_eq!(
            r.begin(PlayerId(9), B, 0, Some(0.0)),
            Err(ReviveDeny::BadHelper)
        );
        r.begin(A, B, 0, Some(HOST_RANGE)).unwrap();
        assert_eq!(
            r.begin(C, B, 0, Some(10.0)),
            Err(ReviveDeny::Busy),
            "one helper"
        );
        r.set_down(C, 0, true);
        assert_eq!(
            r.begin(C, B, 0, Some(10.0)),
            Err(ReviveDeny::BadHelper),
            "downed helper"
        );
    }

    #[test]
    fn helper_moving_away_or_cancelling_aborts() {
        let mut r = party();
        r.set_down(B, 0, true);
        r.begin(A, B, 0, Some(10.0)).unwrap();
        let ev = r.tick(1000, |_, _| Some(HOST_RANGE + 50.0));
        assert_eq!(
            ev,
            vec![ReviveEvent::Cancelled {
                target: B,
                helper: A
            }]
        );
        assert!(r.helper_of(B).is_none());
        r.begin(A, B, 2000, Some(10.0)).unwrap();
        assert_eq!(
            r.cancel(A),
            Some(ReviveEvent::Cancelled {
                target: B,
                helper: A
            })
        );
        assert!(r.tick(2000 + 10 * HOLD_MS, near).is_empty());
    }

    #[test]
    fn unrevivable_respawns_immediately() {
        let mut r = party();
        assert_eq!(r.set_down(B, 0, false), vec![ReviveEvent::Respawn(B)]);
        assert!(!r.is_downed(B));
    }

    #[test]
    fn everyone_down_is_a_single_wipe() {
        let mut r = party();
        r.set_down(A, 0, true);
        r.set_down(B, 0, true);
        let ev = r.set_down(C, 0, true);
        assert_eq!(ev, vec![ReviveEvent::Downed(C), ReviveEvent::Wipe]);
        assert!(r.set_down(C, 1, true).is_empty());
    }

    #[test]
    fn helper_disconnect_cancels_and_target_disconnect_can_complete_a_wipe() {
        let mut r = party();
        r.set_down(B, 0, true);
        r.begin(A, B, 0, Some(1.0)).unwrap();
        assert_eq!(
            r.remove(A),
            vec![ReviveEvent::Cancelled {
                target: B,
                helper: A
            }]
        );
        r.set_down(C, 0, true);
        assert!(r.remove(PlayerId(9)).is_empty());
        let mut r = Reviver::new();
        r.add(A);
        r.add(B);
        r.set_down(A, 0, true);
        assert_eq!(
            r.remove(B),
            vec![ReviveEvent::Wipe],
            "last upright player left"
        );
    }
}

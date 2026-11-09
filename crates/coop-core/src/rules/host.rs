//! Host orchestration of the session rules.

use std::collections::BTreeMap;

use super::barrier::{Barrier, BarrierState, CancelReason};
use super::difficulty::{Mode, Ramp, Scaling};
use super::policy::{PolicyTable, RewardLedger};
use super::revive::{ReviveEvent, Reviver};
use super::tether::{Tether, TetherEvent};
use super::vote::{Proposal, VoteHost, VoteKind, VoteOutcome};
use super::RulesMsg;
use crate::ids::PlayerId;
use crate::interest::Audience;

/// One addressed output message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ROut {
    /// Recipients.
    pub aud: Audience,
    /// Message.
    pub msg: RulesMsg,
}

fn out(aud: Audience, msg: RulesMsg) -> ROut {
    ROut { aud, msg }
}

/// What the host game must do.
#[derive(Clone, Debug, PartialEq)]
pub enum RulesEvent {
    /// Move these players to the tether anchor.
    Warp {
        /// Players.
        players: Vec<PlayerId>,
        /// Anchor id.
        anchor: u32,
    },
    /// Everyone agreed: warp the whole party for transition `tag`.
    BarrierReleased {
        /// Barrier id.
        id: u32,
        /// Transition tag.
        tag: u32,
        /// Everyone to warp.
        players: Vec<PlayerId>,
    },
    /// Barrier cancelled; nobody moves.
    BarrierCancelled {
        /// Barrier id.
        id: u32,
        /// Transition tag.
        tag: u32,
        /// Why.
        reason: CancelReason,
    },
    /// A vote finished.
    Vote(VoteOutcome),
    /// A player's cast counted on an open vote (the host's game mirrors the
    /// casts of a consent vote into its own decision).
    Cast {
        /// Vote id.
        id: u32,
        /// Who voted.
        player: PlayerId,
        /// Yes or no.
        yes: bool,
    },
    /// Fast travel needs no vote: just do it.
    FreeTravel {
        /// Traveller.
        player: PlayerId,
        /// Destination parameter.
        param: u32,
    },
    /// Revive state change.
    Revive(ReviveEvent),
}

/// Host-side rules.
#[derive(Clone, Debug)]
pub struct RulesHost {
    /// Story tether.
    pub tether: Tether,
    /// Time-skip and fork votes.
    pub votes: VoteHost,
    /// Downed and revive.
    pub revive: Reviver,
    /// Difficulty blend.
    pub ramp: Ramp,
    /// Stat routing.
    pub policy: PolicyTable,
    /// Reward grants.
    pub rewards: RewardLedger,
    /// Set by the game: a tether makes fast travel a vote.
    pub tether_votes_fast_travel: bool,
    /// Set by the game: the party is in combat.
    pub in_combat: bool,
    /// Set by the game: a story row locks fast travel, so the host refuses
    /// every player's trip (a guest's game does not know the row).
    pub fast_travel_locked: bool,
    /// The host's friendly-fire choice, announced with the scaling (the
    /// combat vitals apply it).
    pub friendly_fire: bool,
    barriers: BTreeMap<u32, Barrier>,
    next_barrier: u32,
    party: Vec<PlayerId>,
    mode: Mode,
    events: Vec<RulesEvent>,
}

impl RulesHost {
    /// New rules for a campaign in `mode`.
    pub fn new(mode: Mode) -> Self {
        RulesHost {
            tether: Tether::new(),
            votes: VoteHost::new(),
            revive: Reviver::new(),
            ramp: Ramp::new(mode, 1),
            policy: PolicyTable::default(),
            rewards: RewardLedger::new(),
            tether_votes_fast_travel: false,
            in_combat: false,
            fast_travel_locked: false,
            friendly_fire: false,
            barriers: BTreeMap::new(),
            next_barrier: 1,
            party: Vec::new(),
            mode,
            events: Vec::new(),
        }
    }

    /// The campaign mode.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// The host chose the campaign mode (the launcher or viewer flag, a
    /// loaded party save). The scaling for the current party is in force at
    /// once. The next scaling announcement tells the clients.
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.ramp = Ramp::new(mode, (self.party.len() as u32).max(1));
    }

    /// The scaling announcement for the current party (a host may repeat it:
    /// a guest that was still syncing when it first went out missed it).
    pub fn scaling_announcement(&self) -> ROut {
        self.scaling_msg()
    }

    /// Actions for the game, in order.
    pub fn take_events(&mut self) -> Vec<RulesEvent> {
        std::mem::take(&mut self.events)
    }

    /// Players in the party.
    pub fn party(&self) -> &[PlayerId] {
        &self.party
    }

    /// Current scaling.
    pub fn scaling(&self, now_ms: u64) -> Scaling {
        self.ramp.current(now_ms)
    }

    fn scaling_msg(&self) -> ROut {
        let n = self.party.len() as u32;
        out(
            Audience::All,
            RulesMsg::ScalingSet {
                players: n.min(255) as u8,
                mode: self.mode,
                friendly_fire: self.friendly_fire,
            },
        )
    }

    /// A player joined (the host player too).
    pub fn on_join(&mut self, p: PlayerId, now_ms: u64) -> Vec<ROut> {
        if self.party.contains(&p) {
            return vec![];
        }
        self.party.push(p);
        self.revive.add(p);
        self.ramp.set_players(self.party.len() as u32, now_ms);
        vec![self.scaling_msg()]
    }

    /// A player left: every open rule stops waiting for them.
    pub fn on_leave(&mut self, p: PlayerId, now_ms: u64) -> Vec<ROut> {
        self.party.retain(|m| *m != p);
        self.ramp.set_players(self.party.len() as u32, now_ms);
        let mut v = vec![self.scaling_msg()];
        for e in self.tether.remove_member(p, now_ms) {
            v.extend(self.tether_out(e));
        }
        let ids: Vec<u32> = self.barriers.keys().copied().collect();
        for id in ids {
            if let Some(b) = self.barriers.get_mut(&id) {
                b.leave(p);
            }
            v.extend(self.settle_barrier(id));
        }
        if let Some(o) = self.votes.leave(p) {
            v.extend(self.vote_out(o));
        }
        for e in self.revive.remove(p) {
            v.extend(self.revive_out(e));
        }
        v
    }

    /// Opens a barrier over the current party.
    pub fn open_barrier(&mut self, tag: u32, now_ms: u64) -> (u32, Vec<ROut>) {
        let id = self.next_barrier;
        self.next_barrier += 1;
        let b = Barrier::new(id, tag, &self.party, now_ms);
        let required = b.required();
        self.barriers.insert(id, b);
        let mut v = vec![out(
            Audience::All,
            RulesMsg::BarrierOpen { id, tag, required },
        )];
        v.extend(self.settle_barrier(id));
        (id, v)
    }

    fn settle_barrier(&mut self, id: u32) -> Vec<ROut> {
        let Some(b) = self.barriers.get(&id) else {
            return vec![];
        };
        let (state, tag) = (b.state(), b.tag);
        match state {
            BarrierState::Waiting => vec![],
            BarrierState::Released => {
                let players = self.party.clone();
                self.barriers.remove(&id);
                self.events
                    .push(RulesEvent::BarrierReleased { id, tag, players });
                vec![out(
                    Audience::All,
                    RulesMsg::BarrierResult {
                        id,
                        tag,
                        released: true,
                        reason: None,
                    },
                )]
            }
            BarrierState::Cancelled(reason) => {
                self.barriers.remove(&id);
                self.events
                    .push(RulesEvent::BarrierCancelled { id, tag, reason });
                vec![out(
                    Audience::All,
                    RulesMsg::BarrierResult {
                        id,
                        tag,
                        released: false,
                        reason: Some(reason),
                    },
                )]
            }
        }
    }

    fn tether_out(&mut self, e: TetherEvent) -> Vec<ROut> {
        match e {
            TetherEvent::Warn { players, in_ms } => players
                .into_iter()
                .map(|p| {
                    out(
                        Audience::To(p),
                        RulesMsg::TetherWarn {
                            in_ms: in_ms as u32,
                        },
                    )
                })
                .collect(),
            TetherEvent::Warp { players, anchor } => {
                self.events.push(RulesEvent::Warp {
                    players: players.clone(),
                    anchor,
                });
                players
                    .into_iter()
                    .map(|p| out(Audience::To(p), RulesMsg::TetherWarp { anchor }))
                    .collect()
            }
            TetherEvent::Cleared => vec![],
        }
    }

    /// The host's game decided an open consent vote: everyone is told.
    pub fn settle_vote(&mut self, id: u32, approved: bool) -> Vec<ROut> {
        match self.votes.settle(id, approved) {
            Some(o) => self.vote_out(o),
            None => vec![],
        }
    }

    fn vote_out(&mut self, o: VoteOutcome) -> Vec<ROut> {
        self.events.push(RulesEvent::Vote(o));
        vec![out(
            Audience::All,
            RulesMsg::VoteResult {
                id: o.id,
                kind: o.kind,
                param: o.param,
                approved: o.approved,
            },
        )]
    }

    fn revive_out(&mut self, e: ReviveEvent) -> Vec<ROut> {
        self.events.push(RulesEvent::Revive(e));
        match e {
            ReviveEvent::Downed(p) => vec![out(Audience::All, RulesMsg::Downed { player: p })],
            ReviveEvent::Revived {
                player,
                helper,
                health_pct,
            } => vec![
                out(
                    Audience::All,
                    RulesMsg::ReviveProgress {
                        target: player,
                        helper: None,
                    },
                ),
                out(
                    Audience::All,
                    RulesMsg::Revived {
                        player,
                        helper,
                        health_pct,
                    },
                ),
            ],
            ReviveEvent::Respawn(p) => vec![out(Audience::All, RulesMsg::Respawn { player: p })],
            ReviveEvent::Cancelled { target, .. } => {
                vec![out(
                    Audience::All,
                    RulesMsg::ReviveProgress {
                        target,
                        helper: None,
                    },
                )]
            }
            ReviveEvent::Wipe => vec![out(Audience::All, RulesMsg::Wipe)],
        }
    }

    /// Handles one message from `from`. `dist` measures avatar distance.
    pub fn on_msg(
        &mut self,
        from: PlayerId,
        msg: RulesMsg,
        now_ms: u64,
        dist: &dyn Fn(PlayerId, PlayerId) -> Option<f32>,
    ) -> Vec<ROut> {
        if !self.party.contains(&from) {
            return vec![];
        }
        match msg {
            RulesMsg::BarrierReady { id } => {
                if let Some(b) = self.barriers.get_mut(&id) {
                    b.ready(from);
                }
                self.settle_barrier(id)
            }
            RulesMsg::BarrierDecline { id } => {
                if let Some(b) = self.barriers.get_mut(&id) {
                    b.decline(from);
                }
                self.settle_barrier(id)
            }
            RulesMsg::VoteRequest { kind, param } => {
                if kind == VoteKind::FastTravel && self.fast_travel_locked {
                    // the story locks fast travel: no vote, no trip
                    return vec![out(
                        Audience::To(from),
                        RulesMsg::VoteResult {
                            id: 0,
                            kind,
                            param,
                            approved: false,
                        },
                    )];
                }
                let (p, outcome) = self.votes.propose(
                    from,
                    kind,
                    param,
                    &self.party.clone(),
                    now_ms,
                    self.in_combat,
                    self.tether_votes_fast_travel,
                );
                match p {
                    Proposal::Free => {
                        self.events.push(RulesEvent::FreeTravel {
                            player: from,
                            param,
                        });
                        vec![out(
                            Audience::To(from),
                            RulesMsg::VoteResult {
                                id: 0,
                                kind: VoteKind::FastTravel,
                                param,
                                approved: true,
                            },
                        )]
                    }
                    Proposal::Started(id) => {
                        let mut v = vec![out(
                            Audience::All,
                            RulesMsg::VoteOpen {
                                id,
                                kind,
                                param,
                                proposer: from,
                            },
                        )];
                        if let Some(o) = outcome {
                            v.extend(self.vote_out(o));
                        }
                        v
                    }
                    Proposal::Busy | Proposal::InCombat => {
                        vec![out(
                            Audience::To(from),
                            RulesMsg::VoteResult {
                                id: 0,
                                kind,
                                param,
                                approved: false,
                            },
                        )]
                    }
                }
            }
            RulesMsg::VoteCast { id, yes } => {
                if self.votes.accepts(from, id) {
                    self.events.push(RulesEvent::Cast {
                        id,
                        player: from,
                        yes,
                    });
                }
                match self.votes.cast(from, id, yes) {
                    Some(o) => self.vote_out(o),
                    None => vec![],
                }
            }
            RulesMsg::ReportDown { revivable } => {
                let ev = self.revive.set_down(from, now_ms, revivable);
                ev.into_iter().flat_map(|e| self.revive_out(e)).collect()
            }
            RulesMsg::ReviveBegin { target } => {
                match self.revive.begin(from, target, now_ms, dist(from, target)) {
                    Ok(()) => vec![out(
                        Audience::All,
                        RulesMsg::ReviveProgress {
                            target,
                            helper: Some(from),
                        },
                    )],
                    Err(reason) => vec![out(
                        Audience::To(from),
                        RulesMsg::ReviveDenied { target, reason },
                    )],
                }
            }
            RulesMsg::ReviveCancel => match self.revive.cancel(from) {
                Some(e) => self.revive_out(e),
                None => vec![],
            },
            _ => vec![],
        }
    }

    /// Advances every timer.
    pub fn tick(
        &mut self,
        now_ms: u64,
        dist: &dyn Fn(PlayerId, PlayerId) -> Option<f32>,
    ) -> Vec<ROut> {
        let mut v = Vec::new();
        for e in self.tether.tick(now_ms) {
            v.extend(self.tether_out(e));
        }
        let ids: Vec<u32> = self.barriers.keys().copied().collect();
        for id in ids {
            if let Some(b) = self.barriers.get_mut(&id) {
                b.tick(now_ms);
            }
            v.extend(self.settle_barrier(id));
        }
        if let Some(o) = self.votes.tick(now_ms) {
            v.extend(self.vote_out(o));
        }
        for e in self.revive.tick(now_ms, dist) {
            v.extend(self.revive_out(e));
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::barrier;

    const H: PlayerId = PlayerId(0);
    const A: PlayerId = PlayerId(1);
    const B: PlayerId = PlayerId(2);

    fn rules() -> RulesHost {
        let mut r = RulesHost::new(Mode::CoopBalanced);
        for p in [H, A, B] {
            r.on_join(p, 0);
        }
        r
    }
    fn near(_: PlayerId, _: PlayerId) -> Option<f32> {
        Some(50.0)
    }

    #[test]
    fn joins_update_scaling() {
        let r = rules();
        assert!((r.scaling(100_000).toughness - 2.0).abs() < 1e-4);
        let mut r = r;
        let o = r.on_leave(B, 200_000);
        assert!(matches!(o[0].msg, RulesMsg::ScalingSet { players: 2, .. }));
        assert!(
            r.on_join(A, 0).is_empty(),
            "rejoin of a known player is a no-op"
        );
    }

    #[test]
    fn barrier_roundtrip_and_leaver() {
        let mut r = rules();
        let (id, _) = r.open_barrier(5, 0);
        r.on_msg(H, RulesMsg::BarrierReady { id }, 1, &near);
        r.on_msg(A, RulesMsg::BarrierReady { id }, 1, &near);
        let o = r.on_leave(B, 2);
        assert!(o
            .iter()
            .any(|x| matches!(x.msg, RulesMsg::BarrierResult { released: true, .. })));
        let ev = r.take_events();
        assert!(ev.iter().any(|e| matches!(e, RulesEvent::BarrierReleased { tag: 5, players, .. } if players.len() == 2)));
    }

    #[test]
    fn barrier_decline_and_timeout() {
        let mut r = rules();
        let (id, _) = r.open_barrier(1, 0);
        let o = r.on_msg(A, RulesMsg::BarrierDecline { id }, 1, &near);
        assert!(matches!(
            o[0].msg,
            RulesMsg::BarrierResult {
                released: false,
                reason: Some(CancelReason::Declined),
                ..
            }
        ));
        let (_, _) = r.open_barrier(2, 10);
        let o = r.tick(10 + barrier::TIMEOUT_MS, &near);
        assert!(matches!(
            o[0].msg,
            RulesMsg::BarrierResult {
                reason: Some(CancelReason::Timeout),
                ..
            }
        ));
    }

    #[test]
    fn non_members_are_ignored() {
        let mut r = rules();
        assert!(r
            .on_msg(
                PlayerId(9),
                RulesMsg::ReportDown { revivable: true },
                0,
                &near
            )
            .is_empty());
        assert!(!r.revive.is_downed(PlayerId(9)));
    }

    #[test]
    fn sleep_vote_flow() {
        let mut r = rules();
        let o = r.on_msg(
            A,
            RulesMsg::VoteRequest {
                kind: VoteKind::Sleep,
                param: 8,
            },
            0,
            &near,
        );
        let RulesMsg::VoteOpen { id, .. } = o[0].msg else {
            panic!()
        };
        r.on_msg(H, RulesMsg::VoteCast { id, yes: true }, 1, &near);
        let o = r.on_msg(B, RulesMsg::VoteCast { id, yes: true }, 2, &near);
        assert!(matches!(
            o[0].msg,
            RulesMsg::VoteResult {
                approved: true,
                param: 8,
                ..
            }
        ));
    }

    #[test]
    fn fast_travel_free_and_combat_blocks_sleep() {
        let mut r = rules();
        let o = r.on_msg(
            A,
            RulesMsg::VoteRequest {
                kind: VoteKind::FastTravel,
                param: 3,
            },
            0,
            &near,
        );
        assert!(matches!(
            o[0].msg,
            RulesMsg::VoteResult {
                id: 0,
                approved: true,
                ..
            }
        ));
        assert!(r
            .take_events()
            .iter()
            .any(|e| matches!(e, RulesEvent::FreeTravel { param: 3, .. })));
        r.in_combat = true;
        let o = r.on_msg(
            A,
            RulesMsg::VoteRequest {
                kind: VoteKind::Sleep,
                param: 8,
            },
            0,
            &near,
        );
        assert!(matches!(
            o[0].msg,
            RulesMsg::VoteResult {
                approved: false,
                ..
            }
        ));
    }

    #[test]
    fn a_story_lock_refuses_fast_travel_with_or_without_a_tether() {
        let mut r = rules();
        r.fast_travel_locked = true;
        for tether in [false, true] {
            r.tether_votes_fast_travel = tether;
            let o = r.on_msg(
                B,
                RulesMsg::VoteRequest {
                    kind: VoteKind::FastTravel,
                    param: 3,
                },
                0,
                &near,
            );
            assert_eq!(o.len(), 1, "no vote opens");
            assert!(matches!(
                o[0].msg,
                RulesMsg::VoteResult {
                    approved: false,
                    param: 3,
                    ..
                }
            ));
            assert!(matches!(o[0].aud, Audience::To(p) if p == B));
        }
        r.fast_travel_locked = false;
        r.tether_votes_fast_travel = false;
        let o = r.on_msg(
            B,
            RulesMsg::VoteRequest {
                kind: VoteKind::FastTravel,
                param: 3,
            },
            0,
            &near,
        );
        assert!(matches!(
            o[0].msg,
            RulesMsg::VoteResult { approved: true, .. }
        ));
    }

    #[test]
    fn revive_flow_through_messages() {
        let mut r = rules();
        let o = r.on_msg(A, RulesMsg::ReportDown { revivable: true }, 1000, &near);
        assert!(matches!(o[0].msg, RulesMsg::Downed { player: A }));
        let o = r.on_msg(B, RulesMsg::ReviveBegin { target: A }, 2000, &near);
        assert!(matches!(
            o[0].msg,
            RulesMsg::ReviveProgress {
                helper: Some(B),
                ..
            }
        ));
        let far = |_: PlayerId, _: PlayerId| Some(900.0);
        let o = r.on_msg(H, RulesMsg::ReviveBegin { target: A }, 2100, &far);
        assert!(matches!(o[0].msg, RulesMsg::ReviveDenied { .. }));
        let o = r.tick(5000, &near);
        assert!(o.iter().any(|x| matches!(
            x.msg,
            RulesMsg::Revived {
                player: A,
                helper: B,
                health_pct: 30
            }
        )));
    }

    #[test]
    fn tether_warp_reaches_the_straggler_and_the_game() {
        let mut r = rules();
        let party = r.party().to_vec();
        r.tether.start(0, 42, &party, &[H, A]);
        let o = r.tick(tether_warn_at(), &near);
        assert!(
            matches!(o[0].msg, RulesMsg::TetherWarn { in_ms: 10_000 })
                && o[0].aud == Audience::To(B)
        );
        let o = r.tick(120_000, &near);
        assert!(matches!(o[0].msg, RulesMsg::TetherWarp { anchor: 42 }));
        assert!(r
            .take_events()
            .iter()
            .any(|e| matches!(e, RulesEvent::Warp { anchor: 42, .. })));
    }

    fn tether_warn_at() -> u64 {
        crate::rules::tether::TIMEOUT_MS - crate::rules::tether::WARN_LEAD_MS
    }
}

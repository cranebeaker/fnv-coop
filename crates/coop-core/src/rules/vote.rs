//! World-time consent and party votes: sleep, wait, fast travel, and
//! irreversible story forks.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::ids::PlayerId;

/// How long a vote stays open.
pub const VOTE_TIMEOUT_MS: u64 = 30_000;

/// What is being decided.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteKind {
    /// Sleep (skip time): needs everyone.
    Sleep,
    /// Wait (skip time): needs everyone.
    Wait,
    /// Fast travel when a tether demands a vote.
    FastTravel,
    /// Irreversible story fork: majority, host breaks ties.
    Fork,
    /// A choice the campaign policy puts to the party (a message box or a
    /// topic of a consent row; the parameter is the choice). The host's
    /// game decides from the casts ([`VoteHost::settle`]); this table only
    /// carries the question and the answers.
    Consent,
}

/// Longest a consent vote stays open when the host's game never settles it.
pub const CONSENT_TIMEOUT_MS: u64 = 300_000;

impl VoteKind {
    fn unanimous(self) -> bool {
        matches!(self, VoteKind::Sleep | VoteKind::Wait)
    }

    /// Story choices: allowed in combat.
    fn story(self) -> bool {
        matches!(self, VoteKind::Fork | VoteKind::Consent)
    }
}

/// Final answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VoteOutcome {
    /// Vote id.
    pub id: u32,
    /// Kind.
    pub kind: VoteKind,
    /// Game-defined parameter (hours, destination, fork id).
    pub param: u32,
    /// Who proposed.
    pub proposer: PlayerId,
    /// Approved or not.
    pub approved: bool,
}

/// Result of proposing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Proposal {
    /// No vote needed (fast travel out of combat with no tether): do it.
    Free,
    /// A vote opened with this id.
    Started(u32),
    /// Another vote is already open.
    Busy,
    /// Not allowed right now (combat).
    InCombat,
}

#[derive(Clone, Debug)]
struct Open {
    id: u32,
    kind: VoteKind,
    param: u32,
    proposer: PlayerId,
    started_ms: u64,
    members: Vec<PlayerId>,
    votes: BTreeMap<PlayerId, bool>,
}

/// Host-side vote table (one vote at a time).
#[derive(Clone, Debug, Default)]
pub struct VoteHost {
    open: Option<Open>,
    next_id: u32,
}

impl VoteHost {
    /// Empty table.
    pub fn new() -> Self {
        VoteHost {
            open: None,
            next_id: 1,
        }
    }

    /// Id of the open vote.
    pub fn open_id(&self) -> Option<u32> {
        self.open.as_ref().map(|o| o.id)
    }

    /// Proposes `kind`. `members` is the party now; the proposer votes yes.
    /// Fast travel is free unless `tether_votes`; nothing is allowed in
    /// combat for the time skips and fast travel.
    #[allow(clippy::too_many_arguments)]
    pub fn propose(
        &mut self,
        proposer: PlayerId,
        kind: VoteKind,
        param: u32,
        members: &[PlayerId],
        now_ms: u64,
        in_combat: bool,
        tether_votes: bool,
    ) -> (Proposal, Option<VoteOutcome>) {
        if in_combat && !kind.story() {
            return (Proposal::InCombat, None);
        }
        if self.open.is_some() {
            return (Proposal::Busy, None);
        }
        if kind == VoteKind::FastTravel && !tether_votes {
            return (Proposal::Free, None);
        }
        let id = self.next_id;
        self.next_id += 1;
        let mut o = Open {
            id,
            kind,
            param,
            proposer,
            started_ms: now_ms,
            members: members.to_vec(),
            votes: BTreeMap::new(),
        };
        o.votes.insert(proposer, true);
        self.open = Some(o);
        let r = self.resolve(false);
        (Proposal::Started(id), r)
    }

    /// True when a cast from `p` on vote `id` would count (the vote is open,
    /// `p` is in it and has not voted yet).
    pub fn accepts(&self, p: PlayerId, id: u32) -> bool {
        self.open
            .as_ref()
            .is_some_and(|o| o.id == id && o.members.contains(&p) && !o.votes.contains_key(&p))
    }

    /// The host's game decides an open consent vote (any other kind
    /// decides itself and is left alone).
    pub fn settle(&mut self, id: u32, approved: bool) -> Option<VoteOutcome> {
        if !self
            .open
            .as_ref()
            .is_some_and(|o| o.id == id && o.kind == VoteKind::Consent)
        {
            return None;
        }
        self.close(approved)
    }

    /// Records a vote.
    pub fn cast(&mut self, p: PlayerId, id: u32, yes: bool) -> Option<VoteOutcome> {
        let o = self
            .open
            .as_mut()
            .filter(|o| o.id == id && o.members.contains(&p))?;
        o.votes.entry(p).or_insert(yes);
        self.resolve(false)
    }

    /// A member left; their vote no longer counts.
    pub fn leave(&mut self, p: PlayerId) -> Option<VoteOutcome> {
        let o = self.open.as_mut()?;
        o.members.retain(|m| *m != p);
        o.votes.remove(&p);
        if o.members.is_empty() || p == o.proposer {
            return self.close(false);
        }
        self.resolve(false)
    }

    /// Times the vote out.
    pub fn tick(&mut self, now_ms: u64) -> Option<VoteOutcome> {
        let o = self.open.as_ref()?;
        let limit = if o.kind == VoteKind::Consent {
            CONSENT_TIMEOUT_MS
        } else {
            VOTE_TIMEOUT_MS
        };
        if now_ms.saturating_sub(o.started_ms) >= limit {
            return self.close(false);
        }
        None
    }

    fn close(&mut self, approved: bool) -> Option<VoteOutcome> {
        let o = self.open.take()?;
        Some(VoteOutcome {
            id: o.id,
            kind: o.kind,
            param: o.param,
            proposer: o.proposer,
            approved,
        })
    }

    fn resolve(&mut self, _force: bool) -> Option<VoteOutcome> {
        let o = self.open.as_ref()?;
        if o.kind == VoteKind::Consent {
            // the host's game decides ([`VoteHost::settle`])
            return None;
        }
        let n = o.members.len();
        let yes = o
            .members
            .iter()
            .filter(|m| o.votes.get(m) == Some(&true))
            .count();
        let no = o
            .members
            .iter()
            .filter(|m| o.votes.get(m) == Some(&false))
            .count();
        let decided = if o.kind.unanimous() {
            if no > 0 {
                Some(false)
            } else if yes == n {
                Some(true)
            } else {
                None
            }
        } else if yes * 2 > n {
            Some(true)
        } else if no * 2 > n {
            Some(false)
        } else if yes + no == n {
            // exact tie: the host's vote decides (slot 0, else the lowest slot)
            let host = o.members.iter().min().copied();
            Some(host.and_then(|h| o.votes.get(&h).copied()).unwrap_or(false))
        } else {
            None
        };
        decided.and_then(|a| self.close(a))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ps(n: u8) -> Vec<PlayerId> {
        (0..n).map(PlayerId).collect()
    }
    fn start(v: &mut VoteHost, kind: VoteKind, n: u8) -> u32 {
        match v.propose(PlayerId(1), kind, 8, &ps(n), 0, false, true).0 {
            Proposal::Started(id) => id,
            p => panic!("{p:?}"),
        }
    }

    #[test]
    fn sleep_needs_everyone() {
        let mut v = VoteHost::new();
        let id = start(&mut v, VoteKind::Sleep, 3);
        assert!(v.cast(PlayerId(0), id, true).is_none());
        let o = v.cast(PlayerId(2), id, true).unwrap();
        assert!(o.approved && o.param == 8 && o.kind == VoteKind::Sleep);
        assert!(v.open_id().is_none());
    }

    #[test]
    fn one_no_blocks_a_unanimous_vote_at_once() {
        let mut v = VoteHost::new();
        let id = start(&mut v, VoteKind::Wait, 3);
        assert!(!v.cast(PlayerId(2), id, false).unwrap().approved);
    }

    #[test]
    fn solo_player_sleeps_immediately() {
        let mut v = VoteHost::new();
        let (p, o) = v.propose(PlayerId(0), VoteKind::Sleep, 1, &ps(1), 0, false, false);
        assert!(matches!(p, Proposal::Started(_)));
        assert!(o.unwrap().approved);
    }

    #[test]
    fn fast_travel_is_free_unless_tethered() {
        let mut v = VoteHost::new();
        assert_eq!(
            v.propose(
                PlayerId(1),
                VoteKind::FastTravel,
                0,
                &ps(3),
                0,
                false,
                false
            )
            .0,
            Proposal::Free
        );
        assert_eq!(
            v.propose(PlayerId(1), VoteKind::FastTravel, 0, &ps(3), 0, true, false)
                .0,
            Proposal::InCombat
        );
        assert!(matches!(
            v.propose(PlayerId(1), VoteKind::FastTravel, 0, &ps(3), 0, false, true)
                .0,
            Proposal::Started(_)
        ));
        assert_eq!(
            v.propose(PlayerId(2), VoteKind::Sleep, 0, &ps(3), 0, false, true)
                .0,
            Proposal::Busy
        );
    }

    #[test]
    fn fork_is_majority_with_host_breaking_ties() {
        let mut v = VoteHost::new();
        let id = start(&mut v, VoteKind::Fork, 3);
        assert!(
            v.cast(PlayerId(0), id, true).unwrap().approved,
            "2 of 3 yes"
        );
        // 4 players, 2-2 tie: host (slot 0) says no
        let id = start(&mut v, VoteKind::Fork, 4);
        assert!(v.cast(PlayerId(2), id, true).is_none());
        let o = v.cast(PlayerId(0), id, false);
        assert!(o.is_none(), "still waiting on slot 3");
        assert!(!v.cast(PlayerId(3), id, false).unwrap().approved);
    }

    #[test]
    fn a_consent_vote_waits_for_the_host_game_and_counts_each_cast_once() {
        let mut v = VoteHost::new();
        let (p, o) = v.propose(PlayerId(0), VoteKind::Consent, 1, &ps(2), 0, true, false);
        let Proposal::Started(id) = p else {
            panic!("{p:?}: a story choice opens in combat too")
        };
        assert!(o.is_none(), "the proposer's yes does not decide it");
        assert!(v.accepts(PlayerId(1), id));
        assert!(!v.accepts(PlayerId(0), id), "the proposer already voted");
        assert!(
            v.cast(PlayerId(1), id, true).is_none(),
            "every cast in, still open"
        );
        assert!(!v.accepts(PlayerId(1), id), "one cast each");
        assert!(v.tick(VOTE_TIMEOUT_MS).is_none(), "not the short timeout");
        let o = v.settle(id, true).unwrap();
        assert!(o.approved && o.kind == VoteKind::Consent && o.param == 1);
        assert!(v.open_id().is_none());
        // a vote of another kind is never settled from outside
        let id = start(&mut v, VoteKind::Sleep, 3);
        assert!(v.settle(id, true).is_none());
        assert_eq!(v.open_id(), Some(id));
    }

    #[test]
    fn tie_goes_the_hosts_way_when_host_says_yes() {
        let mut v = VoteHost::new();
        let id = start(&mut v, VoteKind::Fork, 2);
        // proposer slot 1 yes, host no => tie, host decides: no
        assert!(!v.cast(PlayerId(0), id, false).unwrap().approved);
        let id = start(&mut v, VoteKind::Fork, 2);
        assert!(v.cast(PlayerId(0), id, true).unwrap().approved);
    }

    #[test]
    fn timeout_denies_and_departures_recount() {
        let mut v = VoteHost::new();
        let id = start(&mut v, VoteKind::Sleep, 3);
        assert!(v.tick(VOTE_TIMEOUT_MS - 1).is_none());
        let o = v.tick(VOTE_TIMEOUT_MS).unwrap();
        assert!(!o.approved && o.id == id);
        let id = start(&mut v, VoteKind::Sleep, 3);
        v.cast(PlayerId(0), id, true);
        assert!(
            v.leave(PlayerId(2)).unwrap().approved,
            "the missing voter left"
        );
    }

    #[test]
    fn votes_are_final_and_checked() {
        let mut v = VoteHost::new();
        let id = start(&mut v, VoteKind::Sleep, 3);
        assert!(v.cast(PlayerId(9), id, false).is_none(), "outsider");
        assert!(v.cast(PlayerId(0), id + 1, false).is_none(), "wrong id");
        v.cast(PlayerId(0), id, true);
        assert!(v.cast(PlayerId(0), id, false).is_none(), "cannot flip");
        assert_eq!(v.open_id(), Some(id));
    }

    #[test]
    fn proposer_leaving_cancels() {
        let mut v = VoteHost::new();
        start(&mut v, VoteKind::Sleep, 3);
        assert!(!v.leave(PlayerId(1)).unwrap().approved);
    }
}

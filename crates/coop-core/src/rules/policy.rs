//! Party-wide shared-stats policy hooks: who receives a stat change, and a
//! ledger so a full reward is granted exactly once per player.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::ids::PlayerId;

/// A game-defined stat kind (karma, reputation, caps, xp, perk ...).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StatKind(pub u16);

/// How a change to a stat reaches the party.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatPolicy {
    /// One shared value (karma, reputation, faction rank): applied once on the
    /// host and mirrored; everyone sees the same number.
    Shared,
    /// Every player gets the full amount once (caps, XP, quest perks).
    FullToAll,
    /// Only the player it happened to (trainer perks, implants, organs).
    PerPlayer,
    /// Only the player who started the scene (payments, skill checks).
    Initiator,
}

/// Maps stat kinds to policies.
#[derive(Clone, Debug)]
pub struct PolicyTable {
    map: BTreeMap<StatKind, StatPolicy>,
    default: StatPolicy,
}

impl PolicyTable {
    /// A table where unlisted stats follow `default`.
    pub fn new(default: StatPolicy) -> Self {
        PolicyTable {
            map: BTreeMap::new(),
            default,
        }
    }

    /// Sets the policy of one stat.
    pub fn set(&mut self, kind: StatKind, policy: StatPolicy) {
        self.map.insert(kind, policy);
    }

    /// Policy of a stat.
    pub fn policy(&self, kind: StatKind) -> StatPolicy {
        self.map.get(&kind).copied().unwrap_or(self.default)
    }

    /// Players who must apply a change that `origin` caused, given everyone
    /// in the party. For [`StatPolicy::Shared`] the host applies it once and
    /// mirrors, so the answer is "all"; the caller distinguishes by policy.
    pub fn recipients(
        &self,
        kind: StatKind,
        origin: PlayerId,
        party: &[PlayerId],
    ) -> Vec<PlayerId> {
        match self.policy(kind) {
            StatPolicy::Shared | StatPolicy::FullToAll => party.to_vec(),
            StatPolicy::PerPlayer | StatPolicy::Initiator => {
                party.iter().copied().filter(|p| *p == origin).collect()
            }
        }
    }
}

impl Default for PolicyTable {
    fn default() -> Self {
        Self::new(StatPolicy::PerPlayer)
    }
}

/// Remembers which players received which reward. A player who joins later
/// (or reconnects) can claim exactly the rewards it is missing.
#[derive(Clone, Debug, Default)]
pub struct RewardLedger {
    granted: BTreeMap<u32, BTreeSet<PlayerId>>,
}

impl RewardLedger {
    /// Empty ledger.
    pub fn new() -> Self {
        Self::default()
    }

    /// Grants `reward` to everyone in `players` who lacks it; returns those.
    pub fn grant(&mut self, reward: u32, players: &[PlayerId]) -> Vec<PlayerId> {
        let set = self.granted.entry(reward).or_default();
        players.iter().copied().filter(|p| set.insert(*p)).collect()
    }

    /// Rewards `p` has not received yet.
    pub fn missing_for(&self, p: PlayerId) -> Vec<u32> {
        self.granted
            .iter()
            .filter(|(_, s)| !s.contains(&p))
            .map(|(r, _)| *r)
            .collect()
    }

    /// True when `p` already holds `reward`.
    pub fn has(&self, reward: u32, p: PlayerId) -> bool {
        self.granted.get(&reward).is_some_and(|s| s.contains(&p))
    }

    /// Marks `reward` as granted to `p` (when the game gave it directly).
    pub fn mark(&mut self, reward: u32, p: PlayerId) {
        self.granted.entry(reward).or_default().insert(p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KARMA: StatKind = StatKind(1);
    const XP: StatKind = StatKind(2);
    const IMPLANT: StatKind = StatKind(3);
    const PARTY: [PlayerId; 3] = [PlayerId(0), PlayerId(1), PlayerId(2)];

    fn table() -> PolicyTable {
        let mut t = PolicyTable::default();
        t.set(KARMA, StatPolicy::Shared);
        t.set(XP, StatPolicy::FullToAll);
        t
    }

    #[test]
    fn recipients_follow_policy() {
        let t = table();
        assert_eq!(t.recipients(KARMA, PlayerId(1), &PARTY).len(), 3);
        assert_eq!(t.recipients(XP, PlayerId(2), &PARTY).len(), 3);
        assert_eq!(
            t.recipients(IMPLANT, PlayerId(1), &PARTY),
            vec![PlayerId(1)],
            "default per player"
        );
        assert!(
            t.recipients(IMPLANT, PlayerId(9), &PARTY).is_empty(),
            "origin not in party"
        );
        assert_eq!(t.policy(StatKind(99)), StatPolicy::PerPlayer);
    }

    #[test]
    fn reward_is_granted_once_per_player() {
        let mut l = RewardLedger::new();
        assert_eq!(l.grant(7, &PARTY[..2]).len(), 2);
        assert!(l.grant(7, &PARTY[..2]).is_empty(), "repeat grants nothing");
        assert_eq!(l.grant(7, &PARTY), vec![PlayerId(2)], "only the newcomer");
        assert!(l.has(7, PlayerId(2)));
    }

    #[test]
    fn late_joiner_claims_missing_rewards() {
        let mut l = RewardLedger::new();
        l.grant(1, &PARTY[..2]);
        l.grant(2, &PARTY[..2]);
        assert_eq!(l.missing_for(PlayerId(2)), vec![1, 2]);
        l.mark(1, PlayerId(2));
        assert_eq!(l.missing_for(PlayerId(2)), vec![2]);
        assert!(l.missing_for(PlayerId(0)).is_empty());
    }
}

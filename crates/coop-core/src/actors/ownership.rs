//! Ownership service: the owner of an actor is the nearest player in the
//! actor's cell. Every owner change bumps the epoch in the shared
//! [`AuthorityTable`], so late writes from a previous owner are rejected.

use std::collections::{BTreeMap, BTreeSet};

use super::ActorState;
use crate::authority::{Authority, AuthorityTable};
use crate::avatar::dequant_pos;
use crate::ids::{CellId, Epoch, FormId, PlayerId, Seq};

/// A new owner must be this much nearer (world units) before a handoff.
pub const HYSTERESIS: f32 = 300.0;
/// Added to the distance of a player that only has the actor's cell loaded,
/// so a player standing in the cell wins over any loaded-only neighbour.
pub const RANK_PENALTY: f32 = 1.0e6;
/// Minimum time between voluntary handoffs of one actor.
pub const MIN_HOLD_MS: u64 = 1000;

/// Where a player is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerLoc {
    /// Cell.
    pub cell: CellId,
    /// Position in world units.
    pub pos: [f32; 3],
}

/// An ownership change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnerChange {
    /// Actor.
    pub actor: FormId,
    /// Previous owner.
    pub old: Option<PlayerId>,
    /// New owner (`None` = dormant).
    pub new: Option<PlayerId>,
    /// Epoch after the change.
    pub epoch: Epoch,
}

/// Outcome of offering a state sample to [`Ownership::on_state`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StateVerdict {
    /// Accepted; relay to the cell.
    Accepted,
    /// Sender once owned the actor but the epoch moved on.
    StaleEpoch,
    /// Sender is not the owner.
    NotOwner,
    /// Old or duplicate sample.
    Old,
    /// Actor is not registered.
    Unknown,
}

/// One tracked actor.
#[derive(Clone, Copy, Debug)]
pub struct ActorRec {
    /// Last accepted state.
    pub state: ActorState,
    /// Current owner.
    pub owner: Option<PlayerId>,
    /// Epoch of the current (or last) ownership.
    pub epoch: Epoch,
    since_ms: u64,
    last_seq: Seq,
    /// Companions follow their recruiter.
    pub follower_of: Option<PlayerId>,
}

/// Owner-per-actor bookkeeping (host side).
#[derive(Clone, Debug, Default)]
pub struct Ownership {
    actors: BTreeMap<FormId, ActorRec>,
    /// Samples refused for a stale epoch.
    pub stale_rejected: u32,
    /// Cells each player has loaded besides the one it stands in (set by
    /// the scene model). A player owns actors there when nobody stands in
    /// their cell.
    pub reach: BTreeMap<PlayerId, BTreeSet<CellId>>,
    /// When an actor went dormant (nobody has its cell loaded).
    pub dormant_since: BTreeMap<FormId, u64>,
}

fn dist(state: &ActorState, loc: &PlayerLoc) -> f32 {
    (0..3)
        .map(|i| (dequant_pos(state.pos[i]) - loc.pos[i]).powi(2))
        .sum::<f32>()
        .sqrt()
}

impl Ownership {
    /// Empty service.
    pub fn new() -> Self {
        Self::default()
    }

    /// Starts tracking an actor (dormant until a player shares its cell).
    pub fn register(&mut self, actor: FormId, state: ActorState) {
        self.actors.entry(actor).or_insert(ActorRec {
            state,
            owner: None,
            epoch: Epoch(0),
            since_ms: 0,
            last_seq: Seq(0),
            follower_of: None,
        });
    }

    /// Stops tracking an actor.
    pub fn unregister(&mut self, actor: FormId, auth: &mut AuthorityTable) {
        self.actors.remove(&actor);
        auth.release_actor(actor);
    }

    /// Marks an actor as a companion of `owner`.
    pub fn set_follower(&mut self, actor: FormId, owner: Option<PlayerId>) {
        if let Some(r) = self.actors.get_mut(&actor) {
            r.follower_of = owner;
        }
    }

    /// Tracked record.
    pub fn get(&self, actor: FormId) -> Option<&ActorRec> {
        self.actors.get(&actor)
    }

    /// Mutable record (damage ledger writes health here).
    pub fn get_mut(&mut self, actor: FormId) -> Option<&mut ActorRec> {
        self.actors.get_mut(&actor)
    }

    /// Every tracked actor.
    pub fn iter(&self) -> impl Iterator<Item = (&FormId, &ActorRec)> {
        self.actors.iter()
    }

    /// Re-evaluates every owner against current player locations.
    pub fn evaluate(
        &mut self,
        now_ms: u64,
        players: &BTreeMap<PlayerId, PlayerLoc>,
        auth: &mut AuthorityTable,
    ) -> Vec<OwnerChange> {
        let mut out = Vec::new();
        for (actor, rec) in self.actors.iter_mut() {
            // rank 0: standing in the actor's cell; rank 1: has the cell loaded
            // around its own. Same-cell players always beat loaded-only ones.
            let reach = &self.reach;
            let here: Vec<(PlayerId, f32)> = players
                .iter()
                .filter_map(|(p, l)| {
                    let rank = if l.cell == rec.state.cell {
                        0.0
                    } else if reach.get(p).is_some_and(|r| r.contains(&rec.state.cell)) {
                        RANK_PENALTY
                    } else {
                        return None;
                    };
                    Some((*p, dist(&rec.state, l) + rank))
                })
                .collect();
            let best = here
                .iter()
                .min_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)))
                .copied();
            let cur = rec
                .owner
                .and_then(|o| here.iter().find(|(p, _)| *p == o).copied());
            let follower = rec
                .follower_of
                .and_then(|f| here.iter().find(|(p, _)| *p == f).copied());
            let want = if let Some(f) = follower {
                Some(f.0)
            } else {
                match (cur, best) {
                    (Some(c), Some(b)) => {
                        let may_move = now_ms.saturating_sub(rec.since_ms) >= MIN_HOLD_MS;
                        if b.0 != c.0 && may_move && b.1 + HYSTERESIS < c.1 {
                            Some(b.0)
                        } else {
                            Some(c.0)
                        }
                    }
                    (None, Some(b)) => Some(b.0),
                    (_, None) => None,
                }
            };
            if want == rec.owner {
                continue;
            }
            let old = rec.owner;
            match want {
                Some(_) => {
                    self.dormant_since.remove(actor);
                }
                None => {
                    self.dormant_since.insert(*actor, now_ms);
                }
            }
            rec.owner = want;
            rec.since_ms = now_ms;
            rec.last_seq = Seq(0);
            match want {
                Some(p) => rec.epoch = auth.claim_actor(*actor, p),
                None => auth.release_actor(*actor),
            }
            out.push(OwnerChange {
                actor: *actor,
                old,
                new: want,
                epoch: rec.epoch,
            });
        }
        out
    }

    /// Validates and records a state sample from `from`.
    pub fn on_state(
        &mut self,
        auth: &AuthorityTable,
        from: PlayerId,
        actor: FormId,
        epoch: Epoch,
        seq: Seq,
        state: ActorState,
    ) -> StateVerdict {
        let Some(rec) = self.actors.get_mut(&actor) else {
            return StateVerdict::Unknown;
        };
        match auth.authority_of(actor) {
            Authority::Actor { owner, epoch: cur } if owner == from => {
                if cur != epoch {
                    self.stale_rejected += 1;
                    return StateVerdict::StaleEpoch;
                }
            }
            _ if epoch < rec.epoch => {
                self.stale_rejected += 1;
                return StateVerdict::StaleEpoch;
            }
            _ => return StateVerdict::NotOwner,
        }
        if rec.last_seq != Seq(0) && !seq.is_newer_than(rec.last_seq) {
            return StateVerdict::Old;
        }
        rec.last_seq = seq;
        let health = rec.state.health;
        rec.state = ActorState { health, ..state };
        StateVerdict::Accepted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::avatar::quant_pos;

    fn at(cell: u32, x: f32) -> ActorState {
        ActorState {
            cell: CellId(cell),
            pos: [quant_pos(x), 0, 0],
            health: 100,
            ..Default::default()
        }
    }
    fn loc(cell: u32, x: f32) -> PlayerLoc {
        PlayerLoc {
            cell: CellId(cell),
            pos: [x, 0.0, 0.0],
        }
    }
    fn players(v: &[(u8, u32, f32)]) -> BTreeMap<PlayerId, PlayerLoc> {
        v.iter()
            .map(|(p, c, x)| (PlayerId(*p), loc(*c, *x)))
            .collect()
    }
    const A: FormId = FormId(0x100);

    #[test]
    fn nearest_in_same_cell_owns() {
        let (mut o, mut auth) = (Ownership::new(), AuthorityTable::new());
        o.register(A, at(1, 0.0));
        let ch = o.evaluate(
            0,
            &players(&[(0, 1, 900.0), (1, 1, 100.0), (2, 2, 0.0)]),
            &mut auth,
        );
        assert_eq!(ch.len(), 1);
        assert_eq!(ch[0].new, Some(PlayerId(1)));
        assert_eq!(o.get(A).unwrap().owner, Some(PlayerId(1)));
        assert!(o
            .evaluate(10, &players(&[(0, 1, 900.0), (1, 1, 100.0)]), &mut auth)
            .is_empty());
    }

    #[test]
    fn dormant_when_nobody_in_cell() {
        let (mut o, mut auth) = (Ownership::new(), AuthorityTable::new());
        o.register(A, at(1, 0.0));
        assert!(o
            .evaluate(0, &players(&[(1, 2, 0.0)]), &mut auth)
            .is_empty());
        assert_eq!(o.get(A).unwrap().owner, None);
    }

    #[test]
    fn handoff_on_cell_change_bumps_epoch_and_fences_old_owner() {
        let (mut o, mut auth) = (Ownership::new(), AuthorityTable::new());
        o.register(A, at(1, 0.0));
        let c1 = o.evaluate(0, &players(&[(1, 1, 50.0), (2, 1, 500.0)]), &mut auth)[0];
        assert_eq!(
            o.on_state(&auth, PlayerId(1), A, c1.epoch, Seq(1), at(1, 1.0)),
            StateVerdict::Accepted
        );
        // player 1 leaves the cell: immediate handoff despite the hold time
        let c2 = o.evaluate(10, &players(&[(1, 2, 0.0), (2, 1, 500.0)]), &mut auth)[0];
        assert_eq!((c2.old, c2.new), (Some(PlayerId(1)), Some(PlayerId(2))));
        assert!(c2.epoch > c1.epoch);
        // late sample from the old owner is fenced
        assert_eq!(
            o.on_state(&auth, PlayerId(1), A, c1.epoch, Seq(2), at(1, 2.0)),
            StateVerdict::StaleEpoch
        );
        assert_eq!(o.stale_rejected, 1);
        assert_eq!(
            o.on_state(&auth, PlayerId(1), A, c2.epoch, Seq(3), at(1, 2.0)),
            StateVerdict::NotOwner
        );
        assert_eq!(
            o.on_state(&auth, PlayerId(2), A, c2.epoch, Seq(1), at(1, 2.0)),
            StateVerdict::Accepted
        );
    }

    #[test]
    fn hysteresis_and_hold_prevent_thrash() {
        let (mut o, mut auth) = (Ownership::new(), AuthorityTable::new());
        o.register(A, at(1, 0.0));
        o.evaluate(0, &players(&[(1, 1, 100.0), (2, 1, 600.0)]), &mut auth);
        // player 2 becomes slightly nearer: no change
        assert!(o
            .evaluate(5000, &players(&[(1, 1, 400.0), (2, 1, 350.0)]), &mut auth)
            .is_empty());
        // much nearer but too soon after the last change
        let mut o2 = Ownership::new();
        let mut a2 = AuthorityTable::new();
        o2.register(A, at(1, 0.0));
        o2.evaluate(100, &players(&[(1, 1, 100.0), (2, 1, 600.0)]), &mut a2);
        assert!(o2
            .evaluate(200, &players(&[(1, 1, 900.0), (2, 1, 50.0)]), &mut a2)
            .is_empty());
        assert_eq!(
            o2.evaluate(1200, &players(&[(1, 1, 900.0), (2, 1, 50.0)]), &mut a2)
                .len(),
            1
        );
    }

    #[test]
    fn disconnect_reassigns_or_goes_dormant() {
        let (mut o, mut auth) = (Ownership::new(), AuthorityTable::new());
        o.register(A, at(1, 0.0));
        o.evaluate(0, &players(&[(1, 1, 10.0), (2, 1, 900.0)]), &mut auth);
        let ch = o.evaluate(1, &players(&[(2, 1, 900.0)]), &mut auth);
        assert_eq!(ch[0].new, Some(PlayerId(2)));
        let ch = o.evaluate(2, &players(&[]), &mut auth);
        assert_eq!(ch[0].new, None);
        assert_eq!(auth.authority_of(A), Authority::Host);
    }

    #[test]
    fn follower_goes_with_recruiter() {
        let (mut o, mut auth) = (Ownership::new(), AuthorityTable::new());
        o.register(A, at(1, 0.0));
        o.set_follower(A, Some(PlayerId(2)));
        o.evaluate(0, &players(&[(1, 1, 10.0), (2, 1, 900.0)]), &mut auth);
        assert_eq!(o.get(A).unwrap().owner, Some(PlayerId(2)));
    }

    #[test]
    fn sample_order_and_unknown() {
        let (mut o, mut auth) = (Ownership::new(), AuthorityTable::new());
        o.register(A, at(1, 0.0));
        let e = o.evaluate(0, &players(&[(1, 1, 0.0)]), &mut auth)[0].epoch;
        assert_eq!(
            o.on_state(&auth, PlayerId(1), A, e, Seq(5), at(1, 0.0)),
            StateVerdict::Accepted
        );
        assert_eq!(
            o.on_state(&auth, PlayerId(1), A, e, Seq(5), at(1, 0.0)),
            StateVerdict::Old
        );
        assert_eq!(
            o.on_state(&auth, PlayerId(1), A, e, Seq(4), at(1, 0.0)),
            StateVerdict::Old
        );
        assert_eq!(
            o.on_state(&auth, PlayerId(1), FormId(9), e, Seq(9), at(1, 0.0)),
            StateVerdict::Unknown
        );
    }

    proptest::proptest! {
        /// Whatever the players do: the owner is always a player standing in
        /// the actor's cell (or nobody), the table agrees with the ownership
        /// service, and epochs only ever move forward.
        #[test]
        fn random_movement_keeps_ownership_sane(
            steps in proptest::collection::vec((0u8..4, 0u32..3, 0f32..2000.0, proptest::bool::ANY), 1..80),
        ) {
            let (mut o, mut auth) = (Ownership::new(), AuthorityTable::new());
            let actors = [FormId(1), FormId(2)];
            o.register(actors[0], at(0, 0.0));
            o.register(actors[1], at(1, 500.0));
            let mut locs: BTreeMap<PlayerId, PlayerLoc> = BTreeMap::new();
            let mut last_epoch = [Epoch(0); 2];
            let mut now = 0u64;
            for (p, cell, x, gone) in steps {
                now += 700;
                if gone { locs.remove(&PlayerId(p)); } else { locs.insert(PlayerId(p), loc(cell, x)); }
                o.evaluate(now, &locs, &mut auth);
                for (i, a) in actors.iter().enumerate() {
                    let rec = o.get(*a).unwrap();
                    if let Some(owner) = rec.owner {
                        let l = locs.get(&owner).expect("owner is connected");
                        proptest::prop_assert_eq!(l.cell, rec.state.cell);
                        proptest::prop_assert_eq!(auth.authority_of(*a), Authority::Actor { owner, epoch: rec.epoch });
                    } else {
                        proptest::prop_assert_eq!(auth.authority_of(*a), Authority::Host);
                        proptest::prop_assert!(!locs.values().any(|l| l.cell == rec.state.cell));
                    }
                    proptest::prop_assert!(rec.epoch >= last_epoch[i]);
                    last_epoch[i] = rec.epoch;
                }
            }
        }
    }

    #[test]
    fn host_health_is_not_overwritten_by_stream() {
        let (mut o, mut auth) = (Ownership::new(), AuthorityTable::new());
        o.register(A, at(1, 0.0));
        let e = o.evaluate(0, &players(&[(1, 1, 0.0)]), &mut auth)[0].epoch;
        o.get_mut(A).unwrap().state.health = 40;
        let mut s = at(1, 3.0);
        s.health = 999;
        o.on_state(&auth, PlayerId(1), A, e, Seq(1), s);
        assert_eq!(o.get(A).unwrap().state.health, 40);
        assert_eq!(o.get(A).unwrap().state.pos[0], quant_pos(3.0));
    }
}

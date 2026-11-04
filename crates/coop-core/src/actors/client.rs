//! Guest side of the actor domain: tracks who owns what, publishes owned
//! actors, applies remote streams (with stale-epoch rejection and an early
//! buffer for samples that beat their assignment).

use std::collections::BTreeMap;

use super::{ActorMsg, ActorState, SpawnOutcome};
use crate::ids::{Epoch, FormId, PlayerId, Seq};

/// What this machine knows about one actor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RemoteActor {
    /// Current owner.
    pub owner: Option<PlayerId>,
    /// Current epoch.
    pub epoch: Epoch,
    /// Newest accepted state.
    pub state: Option<ActorState>,
    /// Alive per the host.
    pub alive: bool,
    seq: Seq,
}

/// Guest actor bookkeeping.
#[derive(Clone, Debug)]
pub struct ActorClient {
    me: PlayerId,
    actors: BTreeMap<FormId, RemoteActor>,
    early: BTreeMap<FormId, (Epoch, Seq, ActorState)>,
    out_seq: BTreeMap<FormId, u32>,
    next_hit: u32,
    next_req: u32,
    /// Spawn verdicts received, by request id.
    pub spawn_results: BTreeMap<u32, SpawnOutcome>,
    /// Leveled picks received, by (list, key).
    pub leveled: BTreeMap<(FormId, u32), FormId>,
    /// Dynamic refs announced by the host, with their base forms.
    pub spawned: BTreeMap<FormId, FormId>,
    /// Where and in which cell the announced dynamic refs were made
    /// (position in 1/16 unit fixed point).
    pub spawn_at: BTreeMap<FormId, (crate::ids::CellId, [i32; 3])>,
    /// Samples dropped for a stale epoch.
    pub stale_dropped: u32,
    /// Who hurt which actor, from the host's verdicts (the threat director
    /// reads and clears it; bounded).
    pub damage_log: Vec<(FormId, PlayerId, u16)>,
    /// The blows of this peer that the host's friendly-fire guard dropped:
    /// victim and NPC flag, from [`ActorMsg::HitBlocked`] (the live adapter
    /// reads and clears it; bounded).
    pub blocked: Vec<(FormId, bool)>,
}

impl ActorClient {
    /// A client for player `me`.
    pub fn new(me: PlayerId) -> Self {
        ActorClient {
            me,
            actors: BTreeMap::new(),
            early: BTreeMap::new(),
            out_seq: BTreeMap::new(),
            next_hit: 1,
            next_req: 1,
            spawn_results: BTreeMap::new(),
            leveled: BTreeMap::new(),
            spawned: BTreeMap::new(),
            spawn_at: BTreeMap::new(),
            stale_dropped: 0,
            damage_log: Vec::new(),
            blocked: Vec::new(),
        }
    }

    /// What is known about an actor.
    pub fn get(&self, actor: FormId) -> Option<&RemoteActor> {
        self.actors.get(&actor)
    }

    /// Every actor this machine knows about.
    pub fn iter(&self) -> impl Iterator<Item = (&FormId, &RemoteActor)> {
        self.actors.iter()
    }

    /// True when this machine currently owns the actor and may publish it.
    pub fn owns(&self, actor: FormId) -> bool {
        self.actors
            .get(&actor)
            .is_some_and(|a| a.owner == Some(self.me))
    }

    /// Actors this machine owns.
    pub fn owned(&self) -> Vec<FormId> {
        self.actors
            .iter()
            .filter(|(_, a)| a.owner == Some(self.me))
            .map(|(f, _)| *f)
            .collect()
    }

    /// Publishes an owned actor; `None` when we do not own it.
    pub fn publish(&mut self, actor: FormId, state: ActorState) -> Option<ActorMsg> {
        let epoch = self
            .actors
            .get(&actor)
            .filter(|a| a.owner == Some(self.me))?
            .epoch;
        let seq = self.out_seq.entry(actor).or_insert(0);
        *seq += 1;
        Some(ActorMsg::State {
            actor,
            epoch,
            seq: Seq(*seq),
            state,
        })
    }

    /// A hit request with a fresh id.
    pub fn hit(&mut self, actor: FormId, amount: u16) -> ActorMsg {
        let hit = self.next_hit;
        self.next_hit += 1;
        ActorMsg::Hit {
            actor,
            hit,
            amount,
            npc: None,
        }
    }

    /// A hit request for the blow of NPC `by` with a fresh id (see
    /// `ActorMsg::Hit::npc`).
    pub fn npc_hit(&mut self, actor: FormId, by: FormId, amount: u16) -> ActorMsg {
        let hit = self.next_hit;
        self.next_hit += 1;
        ActorMsg::Hit {
            actor,
            hit,
            amount,
            npc: Some(by),
        }
    }

    /// A spawn request; returns its id and the message.
    pub fn request_spawn(
        &mut self,
        site: FormId,
        base: FormId,
        cell: crate::ids::CellId,
        pos: [i32; 3],
        health: u16,
    ) -> (u32, ActorMsg) {
        let req = self.next_req;
        self.next_req += 1;
        (
            req,
            ActorMsg::SpawnReq {
                req,
                site,
                base,
                cell,
                pos,
                health,
            },
        )
    }

    /// A report of the actors this machine has loaded, for the host to track
    /// (`None` when there is nothing to report).
    pub fn report(&self, actors: Vec<(FormId, ActorState)>) -> Option<ActorMsg> {
        if actors.is_empty() {
            return None;
        }
        Some(ActorMsg::Track { actors })
    }

    /// A leveled-pick proposal.
    pub fn request_leveled(&mut self, list: FormId, key: u32, pick: FormId) -> ActorMsg {
        let req = self.next_req;
        self.next_req += 1;
        ActorMsg::LeveledReq {
            req,
            list,
            key,
            pick,
        }
    }

    /// Applies a host message.
    pub fn on_msg(&mut self, msg: ActorMsg) {
        match msg {
            ActorMsg::Assign {
                actor,
                owner,
                epoch,
                seed,
            } => {
                if self.actors.get(&actor).is_some_and(|a| a.epoch > epoch) {
                    return; // an older assignment arriving late
                }
                let alive = self.actors.get(&actor).map_or(true, |a| a.alive);
                let mut rec = RemoteActor {
                    owner,
                    epoch,
                    state: seed,
                    alive,
                    seq: Seq(0),
                };
                if let Some((e, s, st)) = self.early.remove(&actor) {
                    if e == epoch {
                        rec.state = Some(st);
                        rec.seq = s;
                    }
                }
                self.actors.insert(actor, rec);
                self.out_seq.insert(actor, 0);
            }
            ActorMsg::State {
                actor,
                epoch,
                seq,
                state,
            } => match self.actors.get_mut(&actor) {
                None => {
                    // beat its assignment: keep the newest per actor
                    let newer = self.early.get(&actor).map_or(true, |(e, s, _)| {
                        epoch > *e || (epoch == *e && seq.is_newer_than(*s))
                    });
                    if newer {
                        self.early.insert(actor, (epoch, seq, state));
                    }
                }
                Some(a) => {
                    if epoch < a.epoch {
                        self.stale_dropped += 1;
                    } else if epoch > a.epoch {
                        // a newer assignment is on its way
                        self.early.insert(actor, (epoch, seq, state));
                    } else if a.owner != Some(self.me)
                        && (a.seq == Seq(0) || seq.is_newer_than(a.seq))
                    {
                        a.seq = seq;
                        a.state = Some(state);
                    }
                }
            },
            ActorMsg::Damaged {
                actor,
                health,
                by,
                amount,
            } => {
                // an NPC's blow credits nobody
                if let Some(by) = by.filter(|_| self.damage_log.len() < 256) {
                    self.damage_log.push((actor, by, amount));
                }
                if let Some(a) = self.actors.get_mut(&actor) {
                    if let Some(s) = a.state.as_mut() {
                        s.health = health;
                    }
                }
            }
            ActorMsg::Healed { actor, health } => {
                if let Some(s) = self.actors.get_mut(&actor).and_then(|a| a.state.as_mut()) {
                    s.health = health;
                }
            }
            ActorMsg::HitBlocked { actor, npc } => {
                if self.blocked.len() < 256 {
                    self.blocked.push((actor, npc));
                }
            }
            ActorMsg::Died { actor, .. } => {
                if let Some(a) = self.actors.get_mut(&actor) {
                    a.alive = false;
                }
            }
            ActorMsg::SpawnReply { req, outcome } => {
                self.spawn_results.insert(req, outcome);
            }
            ActorMsg::Spawned {
                id,
                base,
                cell,
                pos,
                ..
            } => {
                self.spawned.insert(id, base);
                self.spawn_at.insert(id, (cell, pos));
            }
            ActorMsg::Despawn { id } => {
                self.spawned.remove(&id);
                self.spawn_at.remove(&id);
                self.actors.remove(&id);
                self.early.remove(&id);
            }
            ActorMsg::LeveledReply {
                list, key, pick, ..
            } => {
                self.leveled.insert((list, key), pick);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: FormId = FormId(0x100);
    fn st(x: i32) -> ActorState {
        ActorState {
            pos: [x, 0, 0],
            health: 10,
            ..Default::default()
        }
    }
    fn assign(owner: u8, epoch: u32) -> ActorMsg {
        ActorMsg::Assign {
            actor: A,
            owner: Some(PlayerId(owner)),
            epoch: Epoch(epoch),
            seed: Some(st(0)),
        }
    }
    fn state(epoch: u32, seq: u32, x: i32) -> ActorMsg {
        ActorMsg::State {
            actor: A,
            epoch: Epoch(epoch),
            seq: Seq(seq),
            state: st(x),
        }
    }

    #[test]
    fn owner_publishes_with_epoch_and_rising_seq() {
        let mut c = ActorClient::new(PlayerId(1));
        assert!(c.publish(A, st(0)).is_none(), "not assigned yet");
        c.on_msg(assign(1, 3));
        assert!(c.owns(A));
        let m1 = c.publish(A, st(1)).unwrap();
        let m2 = c.publish(A, st(2)).unwrap();
        assert!(matches!(
            m1,
            ActorMsg::State {
                epoch: Epoch(3),
                seq: Seq(1),
                ..
            }
        ));
        assert!(matches!(m2, ActorMsg::State { seq: Seq(2), .. }));
        c.on_msg(assign(2, 4));
        assert!(!c.owns(A));
        assert!(c.publish(A, st(3)).is_none(), "handed off: stop publishing");
    }

    #[test]
    fn remote_stream_filters_stale_epoch_and_old_seq() {
        let mut c = ActorClient::new(PlayerId(0));
        c.on_msg(assign(2, 5));
        c.on_msg(state(5, 2, 20));
        c.on_msg(state(5, 1, 10));
        assert_eq!(c.get(A).unwrap().state.unwrap().pos[0], 20);
        c.on_msg(state(4, 9, 99));
        assert_eq!(c.stale_dropped, 1);
        assert_eq!(c.get(A).unwrap().state.unwrap().pos[0], 20);
    }

    #[test]
    fn early_sample_is_kept_until_the_assignment_lands() {
        let mut c = ActorClient::new(PlayerId(0));
        c.on_msg(state(2, 7, 77));
        c.on_msg(state(2, 6, 66));
        assert!(c.get(A).is_none());
        c.on_msg(assign(1, 2));
        assert_eq!(c.get(A).unwrap().state.unwrap().pos[0], 77);
    }

    #[test]
    fn early_sample_from_another_epoch_is_not_adopted() {
        let mut c = ActorClient::new(PlayerId(0));
        c.on_msg(state(1, 7, 77));
        c.on_msg(assign(1, 2));
        assert_eq!(
            c.get(A).unwrap().state.unwrap().pos[0],
            0,
            "seed, not the old epoch's sample"
        );
    }

    #[test]
    fn late_old_assignment_is_ignored() {
        let mut c = ActorClient::new(PlayerId(0));
        c.on_msg(assign(2, 6));
        c.on_msg(assign(1, 5));
        assert_eq!(c.get(A).unwrap().owner, Some(PlayerId(2)));
    }

    #[test]
    fn damage_and_death_update_the_copy() {
        let mut c = ActorClient::new(PlayerId(0));
        c.on_msg(assign(1, 1));
        c.on_msg(ActorMsg::Damaged {
            actor: A,
            by: Some(PlayerId(1)),
            amount: 4,
            health: 6,
        });
        assert_eq!(c.get(A).unwrap().state.unwrap().health, 6);
        assert_eq!(c.damage_log, vec![(A, PlayerId(1), 4)]);
        // an NPC's blow moves the health but credits no player
        c.on_msg(ActorMsg::Damaged {
            actor: A,
            by: None,
            amount: 1,
            health: 5,
        });
        assert_eq!(c.get(A).unwrap().state.unwrap().health, 5);
        assert_eq!(c.damage_log.len(), 1);
        c.on_msg(ActorMsg::Died {
            actor: A,
            killer: None,
        });
        assert!(!c.get(A).unwrap().alive);
        c.on_msg(assign(2, 2));
        assert!(!c.get(A).unwrap().alive, "death survives a handoff");
    }

    #[test]
    fn spawned_refs_remember_where_and_go_with_despawn() {
        let mut c = ActorClient::new(PlayerId(1));
        let id = FormId(0xFF00_0001);
        c.on_msg(ActorMsg::Spawned {
            id,
            base: FormId(7),
            site: id,
            cell: crate::ids::CellId(4),
            pos: [16, 32, 0],
        });
        assert_eq!(c.spawned[&id], FormId(7));
        assert_eq!(c.spawn_at[&id], (crate::ids::CellId(4), [16, 32, 0]));
        c.on_msg(ActorMsg::Despawn { id });
        assert!(c.spawned.is_empty() && c.spawn_at.is_empty());
        assert!(c.report(vec![]).is_none());
        assert!(matches!(
            c.report(vec![(A, st(0))]),
            Some(ActorMsg::Track { .. })
        ));
    }

    #[test]
    fn request_ids_and_replies() {
        let mut c = ActorClient::new(PlayerId(1));
        let (r1, _) = c.request_spawn(FormId(1), FormId(2), crate::ids::CellId(1), [0; 3], 5);
        let (r2, _) = c.request_spawn(FormId(1), FormId(2), crate::ids::CellId(1), [0; 3], 5);
        assert_ne!(r1, r2);
        c.on_msg(ActorMsg::SpawnReply {
            req: r1,
            outcome: SpawnOutcome::Granted {
                id: FormId(0xFF00_0000),
            },
        });
        assert!(c.spawn_results.contains_key(&r1));
        let ActorMsg::Hit { hit: h1, .. } = c.hit(A, 5) else {
            panic!()
        };
        let ActorMsg::Hit { hit: h2, .. } = c.hit(A, 5) else {
            panic!()
        };
        assert!(h2 > h1);
        c.on_msg(ActorMsg::LeveledReply {
            req: 1,
            list: FormId(1),
            key: 2,
            pick: FormId(3),
        });
        assert_eq!(c.leveled[&(FormId(1), 2)], FormId(3));
    }
}

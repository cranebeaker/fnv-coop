//! Host side of the actor domain: ownership, the state relay, the damage and
//! death ledger, and spawn arbitration.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::ownership::{Ownership, PlayerLoc, StateVerdict};
use super::spawn::{SpawnArbiter, SpawnOutcome, SpawnRec};
use super::{ActorMsg, ActorState, MAX_HIT, MAX_HITS_PER_SEC};
use crate::authority::AuthorityTable;
use crate::ids::{CellId, FormId, PlayerId};
use crate::interest::Audience;

/// Default respawn delay after a spawn site's ref is removed.
pub const RESPAWN_MS: u64 = 60_000;
const HIT_MEMORY: usize = 1024;

/// One addressed output message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Out {
    /// Recipients.
    pub aud: Audience,
    /// Message.
    pub msg: ActorMsg,
}

fn out(aud: Audience, msg: ActorMsg) -> Out {
    Out { aud, msg }
}

/// Host-side actor authority.
#[derive(Clone, Debug, Default)]
pub struct ActorHost {
    /// Who owns what.
    pub own: Ownership,
    /// Spawn ids, sites and leveled picks.
    pub spawn: SpawnArbiter,
    dead: BTreeSet<FormId>,
    protected: BTreeSet<FormId>,
    seen_hits: BTreeSet<(PlayerId, u32)>,
    seen_order: VecDeque<(PlayerId, u32)>,
    hit_times: BTreeMap<PlayerId, VecDeque<u64>>,
    /// Hit requests applied.
    pub hits_applied: u32,
    /// Hit requests refused (duplicate, range, rate, dead, bad).
    pub hits_rejected: u32,
}

impl ActorHost {
    /// Empty host side.
    pub fn new() -> Self {
        Self::default()
    }

    /// Starts tracking a placed actor.
    pub fn register(&mut self, actor: FormId, state: ActorState) {
        self.own.register(actor, state);
    }

    /// Marks a named or story actor: players cannot kill it, only story
    /// authority ([`kill_story`](Self::kill_story)) can.
    pub fn set_protected(&mut self, actor: FormId, protected: bool) {
        if protected {
            self.protected.insert(actor);
        } else {
            self.protected.remove(&actor);
        }
    }

    /// True when players cannot kill the actor (see [`set_protected`](Self::set_protected)).
    pub fn is_protected(&self, actor: FormId) -> bool {
        self.protected.contains(&actor)
    }

    /// True once the actor died.
    pub fn is_dead(&self, actor: FormId) -> bool {
        self.dead.contains(&actor)
    }

    /// Handles one actor message from `from`. `attacker_cell` is the cell the
    /// host believes `from` stands in.
    pub fn on_msg(
        &mut self,
        from: PlayerId,
        msg: ActorMsg,
        now_ms: u64,
        attacker_cell: Option<CellId>,
        auth: &mut AuthorityTable,
    ) -> Vec<Out> {
        match msg {
            ActorMsg::State {
                actor,
                epoch,
                seq,
                state,
            } => {
                if self.own.on_state(auth, from, actor, epoch, seq, state) != StateVerdict::Accepted
                {
                    return vec![];
                }
                let rec = *self.own.get(actor).expect("accepted");
                vec![out(
                    Audience::CellExcept(rec.state.cell, from),
                    ActorMsg::State {
                        actor,
                        epoch,
                        seq,
                        state: rec.state,
                    },
                )]
            }
            ActorMsg::Hit {
                actor,
                hit,
                amount,
                npc,
            } => self.on_hit(from, actor, hit, amount, now_ms, attacker_cell, npc),
            ActorMsg::SpawnReq {
                req,
                site,
                base,
                cell,
                pos,
                health,
            } => {
                let (outcome, rec) = self.spawn.request(from, req, site, base, cell, pos, now_ms);
                let mut v = vec![out(
                    Audience::To(from),
                    ActorMsg::SpawnReply { req, outcome },
                )];
                if let (SpawnOutcome::Granted { .. }, Some((id, r))) = (outcome, rec) {
                    self.own.register(
                        id,
                        ActorState {
                            cell,
                            pos,
                            health,
                            ..Default::default()
                        },
                    );
                    v.push(out(
                        Audience::Cell(cell),
                        ActorMsg::Spawned {
                            id,
                            base: r.base,
                            site: r.site,
                            cell,
                            pos,
                        },
                    ));
                }
                v
            }
            ActorMsg::Track { actors } => {
                // a peer's report of what it has loaded: the host tracks it
                // too (once), so everyone sees it
                for (a, st) in actors.into_iter().take(super::MAX_TRACK) {
                    if self.spawn.is_live(a) || self.dead.contains(&a) {
                        continue;
                    }
                    self.own.register(a, st);
                }
                vec![]
            }
            ActorMsg::Despawn { id } => {
                if self.spawn.site_owner_of(id) != Some(from) {
                    return vec![];
                }
                self.remove_dynamic(id, now_ms, auth)
            }
            ActorMsg::LeveledReq {
                req,
                list,
                key,
                pick,
            } => {
                let pick = self.spawn.leveled(list, key, pick);
                vec![out(
                    Audience::To(from),
                    ActorMsg::LeveledReply {
                        req,
                        list,
                        key,
                        pick,
                    },
                )]
            }
            _ => vec![],
        }
    }

    /// The host made a ref under its own id (a script's `PlaceAtMe`) and
    /// decides here, once, that it exists: tracked, and announced to the
    /// cell. Empty when the id was announced before.
    pub fn announce_made(
        &mut self,
        id: FormId,
        base: FormId,
        site: FormId,
        state: ActorState,
    ) -> Vec<Out> {
        let rec = SpawnRec {
            base,
            site,
            cell: state.cell,
            pos: state.pos,
        };
        if !self.spawn.adopt(id, rec) {
            return vec![];
        }
        self.own.register(id, state);
        vec![out(
            Audience::Cell(state.cell),
            ActorMsg::Spawned {
                id,
                base,
                site,
                cell: state.cell,
                pos: state.pos,
            },
        )]
    }

    /// The host removes a dynamic ref (cleaned up): everyone drops it. The
    /// site's respawn timer starts.
    pub fn despawn_dynamic(
        &mut self,
        id: FormId,
        now_ms: u64,
        auth: &mut AuthorityTable,
    ) -> Vec<Out> {
        self.remove_dynamic(id, now_ms, auth)
    }

    fn remove_dynamic(&mut self, id: FormId, now_ms: u64, auth: &mut AuthorityTable) -> Vec<Out> {
        let cell = self.own.get(id).map(|r| r.state.cell);
        if !self.spawn.despawn(id, now_ms, RESPAWN_MS) {
            return vec![];
        }
        self.own.unregister(id, auth);
        self.dead.remove(&id);
        cell.map(|c| vec![out(Audience::Cell(c), ActorMsg::Despawn { id })])
            .unwrap_or_default()
    }

    #[allow(clippy::too_many_arguments)]
    fn on_hit(
        &mut self,
        from: PlayerId,
        actor: FormId,
        hit: u32,
        amount: u16,
        now_ms: u64,
        attacker_cell: Option<CellId>,
        npc: Option<FormId>,
    ) -> Vec<Out> {
        let reject = |s: &mut Self| {
            s.hits_rejected += 1;
            vec![]
        };
        // a blow under a point: the guard counted it, nothing to apply
        if amount == 0 {
            return vec![];
        }
        if amount > MAX_HIT || self.seen_hits.contains(&(from, hit)) {
            return reject(self);
        }
        // The host's own world dealt the blow and applied it already (a
        // player's blow or an NPC fight there): a refusal only splits the
        // world from the ledger. The rate and the place check a guest's request.
        let host = from.is_host();
        let times = self.hit_times.entry(from).or_default();
        while times
            .front()
            .is_some_and(|t| now_ms.saturating_sub(*t) >= 1000)
        {
            times.pop_front();
        }
        if !host && times.len() >= MAX_HITS_PER_SEC {
            return reject(self);
        }
        let Some(rec) = self.own.get(actor) else {
            return reject(self);
        };
        // the sender stands in the victim's cell or has that cell loaded: a
        // peer owns the actors there too (an NPC's blow in that cell, or a
        // shot into the next exterior cell). The host works out the loaded
        // cells itself, from where the sender's player stands (the grid
        // square around its avatar, `SceneModel::update_peer`). No tighter
        // limit by distance: the game's projectiles go 10000 units (most
        // bullets) to 53000 (.308) and 60000 (Gauss), and the square reaches
        // about 14500 units from the player.
        let reach = self.own.reach.get(&from);
        let loaded = |c: CellId| reach.is_some_and(|r| r.contains(&c));
        let near = attacker_cell == Some(rec.state.cell) || loaded(rec.state.cell);
        if self.dead.contains(&actor) || !(host || near) {
            return reject(self);
        }
        // A guest's NPC blow needs the host's own evidence, or a guest's own
        // blow could pass as one and take no crime, threat or kill credit:
        // the ledger tracks the NPC that struck, alive in the victim's cell or
        // a cell the guest has loaded, and the guest owns that NPC or the
        // victim (the blows that `guest_sends` sends on the guest).
        if let Some(by) = npc.filter(|_| !host) {
            let ok = self.own.get(by).is_some_and(|s| {
                !self.dead.contains(&by)
                    && (s.owner == Some(from) || rec.owner == Some(from))
                    && (s.state.cell == rec.state.cell || loaded(s.state.cell))
            });
            if !ok {
                return reject(self);
            }
        }
        if !host {
            times.push_back(now_ms);
        }
        self.seen_hits.insert((from, hit));
        self.seen_order.push_back((from, hit));
        while self.seen_order.len() > HIT_MEMORY {
            if let Some(k) = self.seen_order.pop_front() {
                self.seen_hits.remove(&k);
            }
        }
        let floor = if self.protected.contains(&actor) {
            1
        } else {
            0
        };
        // an NPC's blow keeps the health but credits nobody
        self.apply_damage(actor, npc.is_none().then_some(from), amount, floor, true)
    }

    /// The host's world raised an actor's health by `amount` (a script's
    /// `ForceActorValue Health`, a heal). The ledger takes the raise and
    /// tells the cell. A guest cannot ask for this: `on_msg` drops `Healed`.
    pub fn heal(&mut self, actor: FormId, amount: u16) -> Vec<Out> {
        if amount == 0 || self.dead.contains(&actor) {
            return vec![];
        }
        let Some(rec) = self.own.get_mut(actor) else {
            return vec![];
        };
        let health = rec.state.health.saturating_add(amount);
        if health == rec.state.health {
            return vec![];
        }
        rec.state.health = health;
        vec![out(
            Audience::Cell(rec.state.cell),
            ActorMsg::Healed { actor, health },
        )]
    }

    /// `announce`: tell the cell with `Damaged` (a story kill without a killer
    /// sends only `Died`).
    fn apply_damage(
        &mut self,
        actor: FormId,
        by: Option<PlayerId>,
        amount: u16,
        floor: u16,
        announce: bool,
    ) -> Vec<Out> {
        let rec = self.own.get_mut(actor).expect("checked");
        let cell = rec.state.cell;
        let health = rec
            .state
            .health
            .saturating_sub(amount)
            .max(floor.min(rec.state.health));
        rec.state.health = health;
        self.hits_applied += 1;
        let mut v = Vec::new();
        if announce {
            v.push(out(
                Audience::Cell(cell),
                ActorMsg::Damaged {
                    actor,
                    by,
                    amount,
                    health,
                },
            ));
        }
        if health == 0 && self.dead.insert(actor) {
            v.push(out(
                Audience::Cell(cell),
                ActorMsg::Died { actor, killer: by },
            ));
        }
        v
    }

    /// Story authority kills a (possibly protected) actor.
    pub fn kill_story(&mut self, actor: FormId, killer: Option<PlayerId>) -> Vec<Out> {
        if self.own.get(actor).is_none() || self.dead.contains(&actor) {
            return vec![];
        }
        let h = self.own.get(actor).map_or(0, |r| r.state.health);
        self.apply_damage(actor, killer, h.max(1), 0, killer.is_some())
    }

    /// Re-evaluates ownership; announces every change to the actor's cell.
    pub fn tick(
        &mut self,
        now_ms: u64,
        players: &BTreeMap<PlayerId, PlayerLoc>,
        auth: &mut AuthorityTable,
    ) -> Vec<Out> {
        let mut v = Vec::new();
        for c in self.own.evaluate(now_ms, players, auth) {
            let rec = self.own.get(c.actor).expect("changed");
            let cell = rec.state.cell;
            let msg = ActorMsg::Assign {
                actor: c.actor,
                owner: c.new,
                epoch: c.epoch,
                seed: Some(rec.state),
            };
            match c.old {
                // the previous owner may have left the cell: tell it directly so it stops publishing
                Some(old) => {
                    v.push(out(Audience::CellExcept(cell, old), msg.clone()));
                    v.push(out(Audience::To(old), msg));
                }
                None => v.push(out(Audience::Cell(cell), msg)),
            }
        }
        v
    }

    /// Everything a peer entering `cell` must learn: current owners and
    /// dynamic refs.
    pub fn snapshot_for_cell(&self, cell: CellId) -> Vec<ActorMsg> {
        let mut v = Vec::new();
        for (id, r) in self.spawn.live() {
            if r.cell == cell {
                v.push(ActorMsg::Spawned {
                    id: *id,
                    base: r.base,
                    site: r.site,
                    cell,
                    pos: r.pos,
                });
            }
        }
        for (actor, rec) in self.own.iter() {
            if rec.state.cell == cell && rec.owner.is_some() {
                v.push(ActorMsg::Assign {
                    actor: *actor,
                    owner: rec.owner,
                    epoch: rec.epoch,
                    seed: Some(rec.state),
                });
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actors::DYN_ID_BASE;
    use crate::ids::{Epoch, Seq};

    const A: FormId = FormId(0x100);
    const C: CellId = CellId(1);

    fn st(health: u16) -> ActorState {
        ActorState {
            cell: C,
            health,
            ..Default::default()
        }
    }
    fn players(v: &[(u8, u32)]) -> BTreeMap<PlayerId, PlayerLoc> {
        v.iter()
            .map(|(p, c)| {
                (
                    PlayerId(*p),
                    PlayerLoc {
                        cell: CellId(*c),
                        pos: [0.0; 3],
                    },
                )
            })
            .collect()
    }
    fn hit(
        h: &mut ActorHost,
        auth: &mut AuthorityTable,
        from: u8,
        id: u32,
        amt: u16,
        now: u64,
    ) -> Vec<Out> {
        h.on_msg(
            PlayerId(from),
            ActorMsg::Hit {
                actor: A,
                hit: id,
                amount: amt,
                npc: None,
            },
            now,
            Some(C),
            auth,
        )
    }
    fn setup() -> (ActorHost, AuthorityTable) {
        let mut h = ActorHost::new();
        h.register(A, st(100));
        (h, AuthorityTable::new())
    }

    #[test]
    fn tick_announces_owner_with_seed() {
        let (mut h, mut auth) = setup();
        let o = h.tick(0, &players(&[(1, 1)]), &mut auth);
        assert_eq!(o.len(), 1);
        assert_eq!(o[0].aud, Audience::Cell(C));
        assert!(matches!(
            o[0].msg,
            ActorMsg::Assign {
                owner: Some(PlayerId(1)),
                seed: Some(_),
                ..
            }
        ));
        assert_eq!(h.snapshot_for_cell(C).len(), 1);
        assert!(h.snapshot_for_cell(CellId(9)).is_empty());
    }

    #[test]
    fn stream_is_relayed_to_everyone_but_the_owner() {
        let (mut h, mut auth) = setup();
        h.tick(0, &players(&[(1, 1)]), &mut auth);
        let e = h.own.get(A).unwrap().epoch;
        let m = ActorMsg::State {
            actor: A,
            epoch: e,
            seq: Seq(1),
            state: st(7),
        };
        let o = h.on_msg(PlayerId(1), m.clone(), 10, Some(C), &mut auth);
        assert_eq!(o[0].aud, Audience::CellExcept(C, PlayerId(1)));
        assert!(
            matches!(&o[0].msg, ActorMsg::State { state, .. } if state.health == 100),
            "ledger health wins"
        );
        assert!(
            h.on_msg(PlayerId(2), m, 10, Some(C), &mut auth).is_empty(),
            "non-owner refused"
        );
        let stale = ActorMsg::State {
            actor: A,
            epoch: Epoch(0),
            seq: Seq(2),
            state: st(7),
        };
        assert!(h
            .on_msg(PlayerId(1), stale, 10, Some(C), &mut auth)
            .is_empty());
    }

    #[test]
    fn a_host_heal_raises_the_ledger_and_the_next_hit_starts_from_it() {
        let (mut h, mut auth) = setup();
        hit(&mut h, &mut auth, 1, 1, 60, 0);
        let o = h.heal(A, 4960);
        assert_eq!(o.len(), 1);
        assert_eq!(o[0].aud, Audience::Cell(C));
        assert_eq!(
            o[0].msg,
            ActorMsg::Healed {
                actor: A,
                health: 5000
            }
        );
        let o = hit(&mut h, &mut auth, 1, 2, 100, 10);
        assert!(matches!(o[0].msg, ActorMsg::Damaged { health: 4900, .. }));
        assert_eq!(
            o.len(),
            1,
            "a 100 point hit does not kill a 5000 point actor"
        );
    }

    #[test]
    fn a_hit_from_a_loaded_cell_next_to_the_victim_counts() {
        const B: FormId = FormId(0x200);
        let (mut h, mut auth) = setup();
        let from_next = |h: &mut ActorHost, auth: &mut AuthorityTable, id, npc| {
            h.on_msg(
                PlayerId(1),
                ActorMsg::Hit {
                    actor: A,
                    hit: id,
                    amount: 5,
                    npc,
                },
                0,
                Some(CellId(2)),
                auth,
            )
        };
        assert!(
            from_next(&mut h, &mut auth, 1, Some(B)).is_empty(),
            "the sender does not have the victim's cell loaded"
        );
        // player 1 has the victim's cell loaded next to its own, and owns
        // the NPC that strikes there
        h.own.reach.insert(PlayerId(1), [C].into_iter().collect());
        h.register(B, st(100));
        h.tick(0, &players(&[(1, 2)]), &mut auth);
        assert_eq!(h.own.get(B).unwrap().owner, Some(PlayerId(1)));
        assert_eq!(
            from_next(&mut h, &mut auth, 2, Some(B)).len(),
            1,
            "an NPC's blow"
        );
        assert_eq!(
            from_next(&mut h, &mut auth, 3, None).len(),
            1,
            "a player's shot"
        );
        assert_eq!(h.own.get(A).unwrap().state.health, 90);
    }

    #[test]
    fn an_npc_blow_keeps_the_health_and_credits_nobody() {
        let (mut h, mut auth) = setup();
        let npc = |h: &mut ActorHost, auth: &mut AuthorityTable, id, amount| {
            h.on_msg(
                PlayerId(0),
                ActorMsg::Hit {
                    actor: A,
                    hit: id,
                    amount,
                    npc: Some(FormId(0x200)),
                },
                0,
                Some(C),
                auth,
            )
        };
        let o = npc(&mut h, &mut auth, 1, 30);
        assert_eq!(
            o[0].msg,
            ActorMsg::Damaged {
                actor: A,
                by: None,
                amount: 30,
                health: 70
            }
        );
        let o = npc(&mut h, &mut auth, 2, 70);
        assert!(h.is_dead(A));
        assert!(
            o.iter().any(|o| o.msg
                == ActorMsg::Died {
                    actor: A,
                    killer: None
                }),
            "{o:?}"
        );
    }

    #[test]
    fn a_guest_cannot_heal_and_the_dead_stay_dead() {
        let (mut h, mut auth) = setup();
        let healed = ActorMsg::Healed {
            actor: A,
            health: 5000,
        };
        assert!(h
            .on_msg(PlayerId(1), healed, 0, Some(C), &mut auth)
            .is_empty());
        let o = hit(&mut h, &mut auth, 1, 1, 30, 0);
        assert!(
            matches!(o[0].msg, ActorMsg::Damaged { health: 70, .. }),
            "a guest's Healed changes nothing"
        );
        hit(&mut h, &mut auth, 1, 2, 70, 10);
        assert!(h.is_dead(A));
        assert!(h.heal(A, 50).is_empty());
        assert!(h.heal(FormId(0x999), 50).is_empty(), "unknown actor");
    }

    #[test]
    fn damage_kills_exactly_once() {
        let (mut h, mut auth) = setup();
        let o = hit(&mut h, &mut auth, 1, 1, 60, 0);
        assert!(matches!(o[0].msg, ActorMsg::Damaged { health: 40, .. }));
        let o = hit(&mut h, &mut auth, 2, 1, 60, 10);
        assert!(matches!(o[0].msg, ActorMsg::Damaged { health: 0, .. }));
        assert!(matches!(
            o[1].msg,
            ActorMsg::Died {
                killer: Some(PlayerId(2)),
                ..
            }
        ));
        assert!(
            hit(&mut h, &mut auth, 1, 2, 60, 20).is_empty(),
            "no hits on the dead"
        );
        assert!(h.is_dead(A));
    }

    #[test]
    fn duplicate_hit_ids_apply_once() {
        let (mut h, mut auth) = setup();
        assert_eq!(hit(&mut h, &mut auth, 1, 5, 10, 0).len(), 1);
        assert!(hit(&mut h, &mut auth, 1, 5, 10, 1).is_empty());
        assert_eq!(
            hit(&mut h, &mut auth, 2, 5, 10, 1).len(),
            1,
            "ids are per attacker"
        );
        assert_eq!(h.own.get(A).unwrap().state.health, 80);
    }

    #[test]
    fn range_rate_and_size_checks() {
        let (mut h, mut auth) = setup();
        let far = h.on_msg(
            PlayerId(1),
            ActorMsg::Hit {
                actor: A,
                hit: 1,
                amount: 5,
                npc: None,
            },
            0,
            Some(CellId(2)),
            &mut auth,
        );
        assert!(far.is_empty());
        assert!(hit(&mut h, &mut auth, 1, 2, MAX_HIT + 1, 0).is_empty());
        assert!(hit(&mut h, &mut auth, 1, 3, 0, 0).is_empty());
        for i in 0..MAX_HITS_PER_SEC as u32 {
            hit(&mut h, &mut auth, 1, 100 + i, 1, 10);
        }
        assert!(
            hit(&mut h, &mut auth, 1, 999, 1, 20).is_empty(),
            "rate limited"
        );
        assert!(
            !hit(&mut h, &mut auth, 1, 1000, 1, 1500).is_empty(),
            "window slides"
        );
    }

    /// The host's own world applied a blow before it reached the ledger: the
    /// ledger takes it at any rate and from any cell, so the two keep the
    /// same health (a big NPC fight on the host, an avatar not placed yet).
    #[test]
    fn the_hosts_own_blows_skip_the_checks_on_a_guests_request() {
        let (mut h, mut auth) = setup();
        let send = |h: &mut ActorHost, auth: &mut AuthorityTable, from: PlayerId, id, cell| {
            let m = ActorMsg::Hit {
                actor: A,
                hit: id,
                amount: 1,
                npc: from.is_host().then_some(FormId(0x200)),
            };
            h.on_msg(from, m, 10, cell, auth)
        };
        let more = MAX_HITS_PER_SEC as u32 + 10;
        for i in 0..more {
            let o = send(&mut h, &mut auth, PlayerId::HOST, i, Some(C));
            assert!(!o.is_empty(), "host blow {i} in one second");
        }
        let o = send(&mut h, &mut auth, PlayerId::HOST, more, None);
        assert!(!o.is_empty(), "a host blow with no cell known yet");
        let left = 100 - u16::try_from(more + 1).unwrap();
        assert_eq!(h.own.get(A).unwrap().state.health, left);
        // a guest's request keeps the checks
        let o = send(&mut h, &mut auth, PlayerId(1), 1, Some(CellId(2)));
        assert!(o.is_empty(), "a guest's blow from a cell it has not loaded");
        for i in 0..more {
            send(&mut h, &mut auth, PlayerId(1), 100 + i, Some(C));
        }
        let rate = MAX_HITS_PER_SEC as u16;
        assert_eq!(h.own.get(A).unwrap().state.health, left - rate);
    }

    /// A guest's NPC blow counts only with the host's own evidence: the
    /// ledger tracks the NPC that struck, alive and near the victim, and the
    /// guest owns that NPC or the victim. Else a guest's own blow could
    /// pass as an NPC's blow and take no crime, threat or kill credit.
    #[test]
    fn a_guests_npc_blow_needs_a_tracked_npc_near_the_victim() {
        let (mut h, mut auth) = setup();
        let (b, far, nobody) = (FormId(0x200), FormId(0x201), FormId(0x2FF));
        h.register(b, st(100));
        h.register(
            far,
            ActorState {
                cell: CellId(9),
                health: 100,
                ..Default::default()
            },
        );
        // player 1 stands in cell 1 and owns A and b, player 2 owns nothing
        h.tick(0, &players(&[(1, 1), (2, 5)]), &mut auth);
        assert_eq!(h.own.get(A).unwrap().owner, Some(PlayerId(1)));
        let blow = |h: &mut ActorHost, auth: &mut AuthorityTable, from, id, by| {
            let m = ActorMsg::Hit {
                actor: A,
                hit: id,
                amount: 5,
                npc: Some(by),
            };
            h.on_msg(PlayerId(from), m, 10, Some(C), auth)
        };
        let health = |h: &ActorHost| h.own.get(A).unwrap().state.health;
        let o = blow(&mut h, &mut auth, 1, 1, b);
        assert!(
            matches!(
                o[0].msg,
                ActorMsg::Damaged {
                    by: None,
                    health: 95,
                    ..
                }
            ),
            "a tracked NPC near the victim: {o:?}"
        );
        let o = blow(&mut h, &mut auth, 1, 2, nobody);
        assert!(o.is_empty(), "an NPC the ledger does not track");
        let o = blow(&mut h, &mut auth, 1, 3, far);
        assert!(o.is_empty(), "an NPC in a cell the guest has not loaded");
        let o = blow(&mut h, &mut auth, 2, 4, b);
        assert!(o.is_empty(), "a guest that owns neither");
        h.kill_story(b, None);
        let o = blow(&mut h, &mut auth, 1, 5, b);
        assert!(o.is_empty(), "a dead NPC");
        assert_eq!(health(&h), 95);
        assert_eq!(h.hits_rejected, 4);
    }

    #[test]
    fn protected_actor_survives_players_but_not_story() {
        let (mut h, mut auth) = setup();
        h.set_protected(A, true);
        hit(&mut h, &mut auth, 1, 1, 500, 0);
        assert_eq!(h.own.get(A).unwrap().state.health, 1);
        assert!(!h.is_dead(A));
        let o = h.kill_story(A, None);
        assert!(matches!(
            o.last().unwrap().msg,
            ActorMsg::Died { killer: None, .. }
        ));
        assert!(h.kill_story(A, None).is_empty(), "already dead");
    }

    #[test]
    fn spawn_flow_registers_and_announces() {
        let (mut h, mut auth) = setup();
        let req = |r, site| ActorMsg::SpawnReq {
            req: r,
            site: FormId(site),
            base: FormId(7),
            cell: C,
            pos: [16, 0, 0],
            health: 50,
        };
        let o = h.on_msg(PlayerId(1), req(1, 0x50), 0, None, &mut auth);
        assert!(matches!(
            o[0].msg,
            ActorMsg::SpawnReply {
                outcome: SpawnOutcome::Granted { .. },
                ..
            }
        ));
        let ActorMsg::Spawned { id, .. } = o[1].msg else {
            panic!()
        };
        assert_eq!(h.own.get(id).unwrap().state.health, 50);
        let o = h.on_msg(PlayerId(2), req(1, 0x50), 0, None, &mut auth);
        assert_eq!(
            o.len(),
            1,
            "second player's roll is denied, nothing announced"
        );
        assert!(matches!(
            o[0].msg,
            ActorMsg::SpawnReply {
                outcome: SpawnOutcome::Denied(_),
                ..
            }
        ));
        // only the site holder may despawn
        assert!(h
            .on_msg(PlayerId(2), ActorMsg::Despawn { id }, 5, None, &mut auth)
            .is_empty());
        let o = h.on_msg(PlayerId(1), ActorMsg::Despawn { id }, 5, None, &mut auth);
        assert!(matches!(o[0].msg, ActorMsg::Despawn { .. }));
        assert!(h.own.get(id).is_none());
    }

    #[test]
    fn a_peers_report_makes_the_host_track_the_actor_once() {
        let (mut h, mut auth) = setup();
        let b = FormId(0x200);
        let rep = ActorMsg::Track {
            actors: vec![(b, st(30)), (A, st(5))],
        };
        assert!(h
            .on_msg(PlayerId(1), rep.clone(), 0, None, &mut auth)
            .is_empty());
        assert_eq!(h.own.get(b).unwrap().state.health, 30);
        assert_eq!(
            h.own.get(A).unwrap().state.health,
            100,
            "an actor already tracked keeps its ledger"
        );
        // nobody is told twice; a later report changes nothing
        h.on_msg(
            PlayerId(2),
            ActorMsg::Track {
                actors: vec![(b, st(1))],
            },
            5,
            None,
            &mut auth,
        );
        assert_eq!(h.own.get(b).unwrap().state.health, 30);
        // then the nearest player owns it like any other
        let o = h.tick(10, &players(&[(2, 1)]), &mut auth);
        assert!(o.iter().any(|x| matches!(x.msg, ActorMsg::Assign { actor, owner: Some(PlayerId(2)), .. } if actor == b)));
    }

    #[test]
    fn a_made_ref_is_announced_exactly_once_and_can_die_once() {
        let (mut h, mut auth) = setup();
        let id = FormId(DYN_ID_BASE + 1);
        let o = h.announce_made(id, FormId(7), id, st(40));
        assert_eq!(o.len(), 1);
        assert!(matches!(o[0].msg, ActorMsg::Spawned { id: i, .. } if i == id));
        assert_eq!(o[0].aud, Audience::Cell(C));
        assert!(
            h.announce_made(id, FormId(7), id, st(40)).is_empty(),
            "decided once"
        );
        assert_eq!(
            h.snapshot_for_cell(C)
                .iter()
                .filter(|m| matches!(m, ActorMsg::Spawned { .. }))
                .count(),
            1
        );
        let kill = |h: &mut ActorHost, auth: &mut AuthorityTable, n| {
            h.on_msg(
                PlayerId(1),
                ActorMsg::Hit {
                    actor: id,
                    hit: n,
                    amount: 100,
                    npc: None,
                },
                0,
                Some(C),
                auth,
            )
        };
        let first = kill(&mut h, &mut auth, 1);
        assert!(first.iter().any(|x| matches!(x.msg, ActorMsg::Died { .. })));
        let again = kill(&mut h, &mut auth, 2);
        assert!(again.is_empty(), "dies once");
        // an arbiter grant after it never reuses the id
        let (_, rec) = h
            .spawn
            .request(PlayerId(1), 1, FormId(0x50), FormId(7), C, [0; 3], 0);
        assert_ne!(rec.unwrap().0, id);
        // the host removing it tells the cell
        let o = h.despawn_dynamic(id, 10, &mut auth);
        assert!(matches!(o[0].msg, ActorMsg::Despawn { .. }));
    }

    #[test]
    fn leveled_reply_is_the_first_pick() {
        let (mut h, mut auth) = setup();
        let ask = |pick| ActorMsg::LeveledReq {
            req: 1,
            list: FormId(3),
            key: 4,
            pick: FormId(pick),
        };
        let a = h.on_msg(PlayerId(1), ask(10), 0, None, &mut auth);
        let b = h.on_msg(PlayerId(2), ask(20), 0, None, &mut auth);
        assert!(matches!(
            a[0].msg,
            ActorMsg::LeveledReply {
                pick: FormId(10),
                ..
            }
        ));
        assert!(matches!(
            b[0].msg,
            ActorMsg::LeveledReply {
                pick: FormId(10),
                ..
            }
        ));
    }

    /// The cells that count for a sender are the grid square around where its
    /// player stands, as the host works it out: a hit counts on an actor inside
    /// that square and on no actor beyond it, whatever the sender sends.
    #[test]
    fn a_hit_counts_only_inside_the_square_around_the_senders_player() {
        use crate::roam::graph::CELL_SIZE;
        use crate::roam::model::SceneModel;
        let mut model = SceneModel::new();
        for x in 0..10 {
            for y in 0..10 {
                model
                    .graph
                    .register_exterior(CellId(1000 + (x * 10 + y) as u32), 0x3C, x, y);
            }
        }
        let world = CellId(0x3C);
        let at = |x: f32| [x * CELL_SIZE, 5.5 * CELL_SIZE, 0.0];
        // player 1 stands in square (5, 5)
        model.update_peer(PlayerId(1), world, at(5.5));
        let mut h = ActorHost::new();
        let mut auth = AuthorityTable::new();
        // two squares east, and three
        let (near, far) = (FormId(0x201), FormId(0x202));
        for (a, cell) in [(near, 1075), (far, 1085)] {
            let state = ActorState {
                cell: CellId(cell),
                health: 100,
                ..Default::default()
            };
            h.register(a, state);
        }
        model.apply_ownership(&mut h.own);
        let mut shot = |h: &mut ActorHost, actor, id| {
            let m = ActorMsg::Hit {
                actor,
                hit: id,
                amount: 5,
                npc: None,
            };
            h.on_msg(PlayerId(1), m, 0, Some(world), &mut auth).len()
        };
        assert_eq!(shot(&mut h, near, 1), 1, "two squares away: in the square");
        assert_eq!(shot(&mut h, far, 2), 0, "three squares away: outside");
        // the player walks two squares east: now the far actor is in the square
        model.update_peer(PlayerId(1), world, at(7.5));
        model.apply_ownership(&mut h.own);
        assert_eq!(shot(&mut h, far, 3), 1);
        assert_eq!(h.own.get(far).unwrap().state.health, 95);
    }
}

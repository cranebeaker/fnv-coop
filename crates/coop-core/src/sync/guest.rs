//! Guest side of the replication protocol.

use std::collections::{BTreeMap, VecDeque};

use super::glive::GuestLive;
use super::RETRY_TICKS;
use crate::chunk::Reassembler;
use crate::delta::StateStore;
use crate::hash::{Repair, ShardHashes};
use crate::ids::{Epoch, FormId, PlayerId, Rev, Seq, Tick};
use crate::proto::{Hello, Msg, RejectReason};
use crate::schema::Field;
use crate::snapshot::Snapshot;

/// Where the guest is in the connection lifecycle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuestPhase {
    /// Not started.
    Idle,
    /// Hello sent, waiting for Welcome.
    Greeting,
    /// Join/Rejoin sent, waiting for state.
    Joining,
    /// Receiving deltas.
    Active,
    /// Host said no.
    Rejected(RejectReason),
}

type Changes = Vec<(FormId, Field)>;

/// A replica of the host's state.
pub struct GuestSync {
    hello: Hello,
    /// The host's identity, when it sent one with a refusal or warning.
    pub host_identity: Option<Hello>,
    /// This guest's slot (assigned by the transport/host).
    pub player: PlayerId,
    phase: GuestPhase,
    /// Replicated state.
    pub store: StateStore,
    session: Option<u64>,
    want_rejoin: bool,
    last_seq: Seq,
    reasm: Reassembler,
    pending_hashes: VecDeque<(Rev, u64)>,
    next_intent: u32,
    unacked: BTreeMap<u32, (Epoch, Changes)>,
    wait_ticks: u32,
    /// Reliable live messages that arrived while the join snapshot was still
    /// coming (the host starts routing them as soon as it accepts the join);
    /// handed to the live layer when the guest turns active.
    early_live: Vec<Msg>,
    /// Live layer: avatars, actors, dialogue, loot, rules, social.
    pub live: GuestLive,
    now_ms: u64,
    /// Hash checks performed.
    pub hash_checks: u32,
    /// Hash mismatches seen.
    pub desyncs_detected: u32,
    /// Repairs applied.
    pub repairs_applied: u32,
    /// Full snapshots loaded.
    pub snapshots_loaded: u32,
    /// Snapshots that came late and were not loaded: each was older than this guest's state.
    pub stale_snapshots: u32,
    /// This guest started its link again, and no state message came after that. The host's
    /// first snapshot is then its answer, of any age: a host that restored an older world
    /// answers a rejoin with a snapshot of a lower revision.
    fresh_link: bool,
}

/// Most live messages kept while the join snapshot is still coming.
const EARLY_LIVE_MAX: usize = 512;

impl GuestSync {
    /// A fresh guest that will quote `hello` to the host.
    pub fn new(player: PlayerId, hello: Hello) -> Self {
        GuestSync {
            hello,
            host_identity: None,
            player,
            phase: GuestPhase::Idle,
            store: StateStore::new(),
            session: None,
            want_rejoin: false,
            last_seq: Seq(0),
            reasm: Reassembler::new(),
            pending_hashes: VecDeque::new(),
            next_intent: 1,
            unacked: BTreeMap::new(),
            wait_ticks: 0,
            early_live: Vec::new(),
            live: GuestLive::new(player),
            now_ms: 0,
            hash_checks: 0,
            desyncs_detected: 0,
            repairs_applied: 0,
            snapshots_loaded: 0,
            stale_snapshots: 0,
            fresh_link: true,
        }
    }

    /// Sets the wall clock (ms) the live layer runs on. Drivers call this
    /// before feeding messages and ticking.
    pub fn set_time_ms(&mut self, ms: u64) {
        self.now_ms = ms;
    }

    /// Current phase.
    pub fn phase(&self) -> GuestPhase {
        self.phase
    }

    /// Starts a fresh join. Returns the opening message.
    pub fn start(&mut self) -> Vec<Msg> {
        self.want_rejoin = false;
        self.fresh_link = true;
        self.begin()
    }

    /// Reconnects keeping the local state (resume from its cursor).
    pub fn reconnect(&mut self) -> Vec<Msg> {
        self.want_rejoin = self.session.is_some();
        self.fresh_link = true;
        self.reasm = Reassembler::new();
        self.pending_hashes.clear();
        self.begin()
    }

    fn begin(&mut self) -> Vec<Msg> {
        self.early_live.clear();
        self.phase = GuestPhase::Greeting;
        self.wait_ticks = 0;
        vec![Msg::Hello(self.hello.clone())]
    }

    fn fresh_join(&mut self, session: u64) -> Vec<Msg> {
        self.session = Some(session);
        self.store = StateStore::new();
        self.last_seq = Seq(0);
        self.phase = GuestPhase::Joining;
        self.wait_ticks = 0;
        vec![Msg::Join]
    }

    /// Queues a proposal for the host; it is resent until acknowledged.
    pub fn send_intent(&mut self, epoch: Epoch, changes: Changes) -> Vec<Msg> {
        let id = self.next_intent;
        self.next_intent += 1;
        self.unacked.insert(id, (epoch, changes.clone()));
        vec![Msg::Intent { id, epoch, changes }]
    }

    /// Differences between this guest's game and the host's, once the host
    /// has said (empty until then).
    pub fn compat_issues(&self) -> Vec<crate::compat::Issue> {
        self.host_identity
            .as_ref()
            .map(|h| crate::compat::issues(h, &self.hello))
            .unwrap_or_default()
    }

    /// Intents not yet acknowledged.
    pub fn unacked_intents(&self) -> usize {
        self.unacked.len()
    }

    /// Handles one decoded message; returns replies.
    pub fn on_msg(&mut self, msg: Msg) -> Vec<Msg> {
        match msg {
            Msg::Welcome { session, .. } => {
                if self.phase != GuestPhase::Greeting {
                    return vec![];
                }
                if self.want_rejoin && self.session == Some(session) {
                    self.phase = GuestPhase::Active;
                    self.want_rejoin = false;
                    vec![Msg::Rejoin {
                        session,
                        last_seq: self.last_seq,
                        have_rev: self.store.base(),
                    }]
                } else {
                    self.fresh_join(session)
                }
            }
            Msg::Incompat(h) => {
                self.host_identity = Some(h);
                vec![]
            }
            Msg::Reject(RejectReason::SessionMismatch) => {
                self.session = None;
                self.want_rejoin = false;
                self.begin()
            }
            Msg::Reject(r) => {
                self.phase = GuestPhase::Rejected(r);
                vec![]
            }
            Msg::Delta { seq, delta } => {
                if self.phase != GuestPhase::Active {
                    return vec![];
                }
                self.store.apply_delta(&delta);
                if seq.is_newer_than(self.last_seq) {
                    self.last_seq = seq;
                }
                let mut out = Vec::new();
                if self.store.base() >= delta.rev {
                    // The guest follows the host's deltas: the link's first answer came.
                    self.fresh_link = false;
                    out.push(Msg::Ack { seq });
                }
                out.extend(self.check_hashes());
                out
            }
            Msg::Snapshot(chunk) => {
                let id = chunk.id;
                let Ok(Some(bytes)) = self.reasm.push(chunk) else {
                    return vec![];
                };
                let Ok(snap) = Snapshot::decode(&bytes) else {
                    return vec![];
                };
                // A late snapshot does not take newer state away. A host can queue more
                // than one snapshot for a guest that is silent for a long time (a game that
                // loads a cell in one frame). The reliable channel can deliver an older one
                // after the guest applied later deltas, and the host does not send those
                // deltas again: the guest's state went back and stayed there (found on two
                // real games, 2026-11-10, 2 of 5 runs).
                //
                // A late snapshot has a number that is not newer than the newest message
                // that the guest took, and a revision below the guest's own. The guest keeps
                // its state. It acknowledges the snapshot, so that a host that waits for this
                // number goes on with deltas from the snapshot's revision.
                //
                // The first snapshot after a new link is the host's answer, of any age: a
                // host that restored an older world counts its numbers and its revisions
                // from the start again.
                let late = !id.is_newer_than(self.last_seq) && snap.rev < self.store.base();
                if self.phase == GuestPhase::Active && !self.fresh_link && late {
                    self.stale_snapshots += 1;
                    return vec![Msg::Ack { seq: id }];
                }
                self.fresh_link = false;
                self.store = snap.into_store();
                self.snapshots_loaded += 1;
                self.last_seq = id;
                self.phase = GuestPhase::Active;
                self.pending_hashes.clear();
                let mut out = vec![Msg::Ack { seq: id }];
                for m in std::mem::take(&mut self.early_live) {
                    out.extend(self.live.on_msg(m, self.now_ms));
                }
                out.extend(self.check_hashes());
                out
            }
            Msg::IntentAck { id, .. } => {
                self.unacked.remove(&id);
                vec![]
            }
            Msg::Hash { rev, root, .. } => {
                if self.phase != GuestPhase::Active {
                    return vec![];
                }
                self.pending_hashes.push_back((rev, root));
                while self.pending_hashes.len() > 8 {
                    self.pending_hashes.pop_front();
                }
                self.check_hashes()
            }
            Msg::Repair(r) => {
                self.apply_repair(r);
                vec![]
            }
            m @ (Msg::Avatar(_)
            | Msg::Actor(_)
            | Msg::Dialogue(_)
            | Msg::Loot(_)
            | Msg::Rules(_)
            | Msg::Social(_)
            | Msg::Scene(_)
            | Msg::Interact(_)
            | Msg::Env(_)
            | Msg::Roam(_)
            | Msg::Combat(_)) => {
                if self.phase == GuestPhase::Active {
                    self.live.on_msg(m, self.now_ms)
                } else {
                    // keep what the stream guarantees (snapshots of actors and
                    // containers, grants, results); datagrams would be stale
                    if self.phase == GuestPhase::Joining
                        && m.channel() == crate::proto::Channel::ReliableOrdered
                        && self.early_live.len() < EARLY_LIVE_MAX
                    {
                        self.early_live.push(m);
                    }
                    vec![]
                }
            }
            _ => vec![],
        }
    }

    fn apply_repair(&mut self, r: Repair) {
        r.apply(&mut self.store);
        self.repairs_applied += 1;
        self.pending_hashes.clear();
    }

    /// Compares pending host hashes when this replica sits exactly at that
    /// revision; asks for a repair on mismatch.
    fn check_hashes(&mut self) -> Vec<Msg> {
        let rev = self.store.rev();
        self.pending_hashes.retain(|(r, _)| *r >= rev);
        if !self.store.is_exact() {
            return vec![];
        }
        let Some(pos) = self.pending_hashes.iter().position(|(r, _)| *r == rev) else {
            return vec![];
        };
        let (_, root) = self.pending_hashes.remove(pos).expect("found");
        let mine = ShardHashes::compute(&self.store);
        self.hash_checks += 1;
        if mine.root() == root {
            return vec![];
        }
        self.desyncs_detected += 1;
        vec![Msg::RepairRequest {
            rev,
            shards: mine.shards().to_vec(),
        }]
    }

    /// Per-tick housekeeping: handshake retries and intent resends.
    pub fn tick(&mut self, _now: Tick) -> Vec<Msg> {
        let mut live = if self.phase == GuestPhase::Active {
            self.live.tick(self.now_ms)
        } else {
            vec![]
        };
        self.wait_ticks += 1;
        if self.wait_ticks < RETRY_TICKS {
            return live;
        }
        self.wait_ticks = 0;
        match self.phase {
            GuestPhase::Greeting | GuestPhase::Joining => {
                // restart the handshake (keeps rejoin intent if we had one)
                self.want_rejoin =
                    self.want_rejoin || self.session.is_some() && self.store.base() > Rev::ZERO;
                self.begin()
            }
            GuestPhase::Active => {
                live.extend(
                    self.unacked
                        .iter()
                        .map(|(id, (epoch, changes))| Msg::Intent {
                            id: *id,
                            epoch: *epoch,
                            changes: changes.clone(),
                        }),
                );
                live
            }
            _ => live,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk;
    use crate::sim_test::test_hello;

    const QUEST: FormId = FormId(0x0015_d912);
    const SESSION: u64 = 0x5e55;

    /// The messages that carry a snapshot of `store` with the number `id`.
    fn snapshot_of(store: &StateStore, id: u32) -> Vec<Msg> {
        let bytes = Snapshot::from_store(store, Tick(0)).encode().unwrap();
        chunk::split(Seq(id), &bytes, 64)
            .unwrap()
            .into_iter()
            .map(Msg::Snapshot)
            .collect()
    }

    fn feed(guest: &mut GuestSync, msgs: Vec<Msg>) -> Vec<Msg> {
        msgs.into_iter().flat_map(|m| guest.on_msg(m)).collect()
    }

    fn stage(guest: &GuestSync) -> Option<Field> {
        guest
            .store
            .get(QUEST, &Field::QuestStage { stage: 0 })
            .map(|e| e.field.clone())
    }

    /// A guest that follows the host, with the host's store: the join snapshot at stage 5,
    /// then one delta to stage 10.
    fn guest_at_stage_10() -> (GuestSync, StateStore, StateStore) {
        let mut host = StateStore::new();
        host.write_local(QUEST, Field::QuestStage { stage: 5 });
        let old = host.clone();
        let mut guest = GuestSync::new(PlayerId(1), test_hello());
        guest.start();
        feed(
            &mut guest,
            vec![Msg::Welcome {
                session: SESSION,
                player: PlayerId(1),
                tick: Tick(0),
                rev: old.rev(),
            }],
        );
        feed(&mut guest, snapshot_of(&old, 1));
        assert_eq!(guest.phase(), GuestPhase::Active);
        host.write_local(QUEST, Field::QuestStage { stage: 10 });
        // The host queued the snapshot 2 before this delta (a second join message).
        let delta = host.delta_since(old.rev(), Tick(1), 24);
        let acks = feed(&mut guest, vec![Msg::Delta { seq: Seq(5), delta }]);
        assert!(acks.contains(&Msg::Ack { seq: Seq(5) }), "{acks:?}");
        assert_eq!(stage(&guest), Some(Field::QuestStage { stage: 10 }));
        (guest, host, old)
    }

    /// A snapshot that the host queued before a change comes to the guest after the change.
    /// The guest keeps the change, and it acknowledges the snapshot.
    #[test]
    fn a_late_snapshot_does_not_take_newer_state_away() {
        let (mut guest, host, old) = guest_at_stage_10();
        let replies = feed(&mut guest, snapshot_of(&old, 2));
        assert_eq!(stage(&guest), Some(Field::QuestStage { stage: 10 }));
        assert_eq!(guest.store.base(), host.rev(), "the store has no gap");
        assert_eq!(replies, [Msg::Ack { seq: Seq(2) }]);
        assert_eq!((guest.snapshots_loaded, guest.stale_snapshots), (1, 1));
    }

    /// The twins: a snapshot that is not older is loaded, a guest that joins again loads
    /// each snapshot, and so does a guest whose link started again.
    #[test]
    fn a_snapshot_that_is_not_older_is_loaded() {
        // The host's state at a later revision: the stage went back to 7 there.
        let (mut guest, mut host, _) = guest_at_stage_10();
        host.write_local(QUEST, Field::QuestStage { stage: 7 });
        feed(&mut guest, snapshot_of(&host, 6));
        assert_eq!(stage(&guest), Some(Field::QuestStage { stage: 7 }));
        assert_eq!((guest.snapshots_loaded, guest.stale_snapshots), (2, 0));

        // A new snapshot number with an older revision is the host's own choice (it made
        // the snapshot after each message that the guest has): the guest loads it.
        let (mut guest, _, old) = guest_at_stage_10();
        feed(&mut guest, snapshot_of(&old, 6));
        assert_eq!(stage(&guest), Some(Field::QuestStage { stage: 5 }));
        assert_eq!(guest.stale_snapshots, 0);

        // The guest's link started again, and the host restored an older world: its answer
        // to the rejoin is a snapshot with a low number and a low revision.
        let (mut guest, _, old) = guest_at_stage_10();
        guest.reconnect();
        let rejoin = feed(
            &mut guest,
            vec![Msg::Welcome {
                session: SESSION,
                player: PlayerId(1),
                tick: Tick(0),
                rev: old.rev(),
            }],
        );
        assert!(matches!(rejoin[..], [Msg::Rejoin { .. }]), "{rejoin:?}");
        assert_eq!(guest.phase(), GuestPhase::Active);
        feed(&mut guest, snapshot_of(&old, 0));
        assert_eq!(stage(&guest), Some(Field::QuestStage { stage: 5 }));
        assert_eq!(guest.stale_snapshots, 0);

        // A guest that starts its join again is not in the active phase: it loads the
        // snapshot that the host sends, also when its revision is lower (a host that
        // started again counts from zero).
        let (mut guest, _, _) = guest_at_stage_10();
        guest.start();
        feed(
            &mut guest,
            vec![Msg::Welcome {
                session: SESSION + 1,
                player: PlayerId(1),
                tick: Tick(0),
                rev: old.rev(),
            }],
        );
        feed(&mut guest, snapshot_of(&old, 4));
        assert_eq!(stage(&guest), Some(Field::QuestStage { stage: 5 }));
        assert_eq!(guest.stale_snapshots, 0);
    }
}

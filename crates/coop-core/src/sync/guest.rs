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
        self.begin()
    }

    /// Reconnects keeping the local state (resume from its cursor).
    pub fn reconnect(&mut self) -> Vec<Msg> {
        self.want_rejoin = self.session.is_some();
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

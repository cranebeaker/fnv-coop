//! Host side of the replication protocol.

use std::collections::{BTreeMap, HashMap, VecDeque};

use super::live::HostLive;
use crate::authority::AuthorityTable;
use crate::baseline::{BaselineRing, SendPlan};
use crate::chunk;
use crate::delta::StateStore;
use crate::hash::{HashHistory, Repair, ShardHashes};
use crate::ids::SHARD_COUNT;
use crate::ids::{Epoch, FormId, PlayerId, Rev, Seq, Tick};
use crate::proto::{self, Hello, Msg, RejectReason, SNAPSHOT_CHUNK_BYTES};
use crate::rules::Mode;
use crate::schema::{Field, Key};
use crate::session::{self, HostCursor, RejoinDecision, RejoinRequest};
use crate::snapshot::Snapshot;

/// Host tunables.
#[derive(Clone, Debug)]
pub struct HostConfig {
    /// Identity checked against every guest hello.
    pub hello: Hello,
    /// World lineage id, quoted by rejoining guests.
    pub session: u64,
    /// Most guests.
    pub max_guests: usize,
    /// Baseline ring depth (sends remembered per client).
    pub ring_cap: usize,
    /// Hash points remembered for repair comparisons.
    pub hash_history: usize,
    /// Campaign difficulty mode for the live layer.
    pub mode: Mode,
    /// Join compatibility policy (the developer override lives here).
    pub compat: crate::compat::CompatPolicy,
}

impl HostConfig {
    /// Sensible defaults for a given identity.
    pub fn new(hello: Hello, session: u64) -> Self {
        HostConfig {
            hello,
            session,
            max_guests: 3,
            ring_cap: 128,
            hash_history: 32,
            mode: Mode::CoopBalanced,
            compat: Default::default(),
        }
    }
}

/// A handshake that found differences (refused, or let in by the override);
/// the session turns these into messages for the host's player.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompatNote {
    /// The guest.
    pub player: PlayerId,
    /// What differed.
    pub issues: Vec<crate::compat::Issue>,
    /// True when the guest was turned away.
    pub refused: bool,
}

/// Store writes remembered for [`HostSync::recent_roots`] (about 40 s of a
/// timer that changes every frame).
const UNDO_CAP: usize = 4096;

/// One store write, with the value it replaced.
#[derive(Debug)]
struct Undo {
    rev: Rev,
    key: Key,
    old: Option<Field>,
    at_ms: u64,
}

#[derive(Debug, Default)]
struct Guest {
    active: bool,
    last_intent: u32,
    last_rejected: u16,
    /// The guest still loads (see [`HostSync::hold`]).
    held: bool,
}

/// The authoritative side.
pub struct HostSync {
    cfg: HostConfig,
    /// Handshakes that differed, not yet reported.
    pub compat_log: Vec<CompatNote>,
    /// Authoritative state.
    pub store: StateStore,
    /// Who may write what.
    pub auth: AuthorityTable,
    ring: BaselineRing,
    hist: HashHistory,
    guests: BTreeMap<PlayerId, Guest>,
    tick: Tick,
    inputs: Vec<(PlayerId, Tick, Vec<u8>)>,
    /// Live layer: avatars, actors, dialogue, loot, rules, social.
    pub live: HostLive,
    now_ms: u64,
    deferred: Vec<(PlayerId, Msg)>,
    /// Repairs sent so far (diagnostics).
    pub repairs_sent: u32,
    /// Full snapshots sent so far (diagnostics).
    pub snapshots_sent: u32,
    /// Joins turned away because the party was full (`Reject(Full)`), one per
    /// refused player (a repeated hello of the same player is not counted again).
    pub full_refusals: u32,
    refused_full: std::collections::BTreeSet<PlayerId>,
    /// The last store writes, oldest first (see [`HostSync::recent_roots`]).
    undo: VecDeque<Undo>,
}

impl HostSync {
    /// A host with an empty world.
    pub fn new(cfg: HostConfig) -> Self {
        let ring = BaselineRing::new(cfg.ring_cap);
        let hist = HashHistory::new(cfg.hash_history);
        let live = HostLive::new(cfg.mode);
        HostSync {
            cfg,
            compat_log: Vec::new(),
            store: StateStore::new(),
            auth: AuthorityTable::new(),
            ring,
            hist,
            guests: BTreeMap::new(),
            tick: Tick(0),
            inputs: Vec::new(),
            live,
            now_ms: 0,
            deferred: Vec::new(),
            repairs_sent: 0,
            snapshots_sent: 0,
            full_refusals: 0,
            refused_full: std::collections::BTreeSet::new(),
            undo: VecDeque::new(),
        }
    }

    /// Current tick.
    pub fn tick_no(&self) -> Tick {
        self.tick
    }

    /// Host-side game write.
    pub fn write(&mut self, form: FormId, field: Field) -> Option<Rev> {
        // COOP_STORE_TRACE=1 prints each store change (finds keys that change every frame)
        static TRACE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        let trace = *TRACE.get_or_init(|| std::env::var_os("COOP_STORE_TRACE").is_some());
        let shown = trace.then(|| format!("{form:?} {field:?}"));
        let rev = self.write_logged(form, field);
        if let (Some(r), Some(s)) = (rev, shown) {
            eprintln!(
                "coop store {r:?}: {}",
                s.chars().take(160).collect::<String>()
            );
        }
        rev
    }

    /// Writes the store and remembers the replaced value.
    fn write_logged(&mut self, form: FormId, field: Field) -> Option<Rev> {
        let key = field.key(form);
        let old = self.store.get_key(&key).map(|e| e.field.clone());
        let rev = self.store.write_local(form, field)?;
        self.undo.push_back(Undo {
            rev,
            key,
            old,
            at_ms: self.now_ms,
        });
        while self.undo.len() > UNDO_CAP {
            self.undo.pop_front();
        }
        Some(rev)
    }

    /// The root hash of the store at each recent revision, newest first,
    /// from the current revision back to the writes of the last `within_ms`.
    /// `keep` selects the fields that count (the game hash skips cleared
    /// fields). The list stops at a write that this host did not log (a
    /// direct store change), because the older revisions are then unknown.
    /// A guest that dumps its state a few revisions behind is compared at
    /// its own revision with this list.
    pub fn recent_roots(&self, within_ms: u64, keep: &dyn Fn(&Field) -> bool) -> Vec<(Rev, u64)> {
        let mut shards = vec![0u64; SHARD_COUNT];
        for (k, e) in self.store.iter().filter(|(_, e)| keep(&e.field)) {
            let s = crate::ids::shard_of(k.form) as usize;
            shards[s] = shards[s].wrapping_add(crate::hash::record_hash(k.form, &e.field));
        }
        let root = |shards: &[u64]| ShardHashes::from_vec(shards.to_vec()).map_or(0, |h| h.root());
        let mut rev = self.store.rev();
        let mut out = vec![(rev, root(&shards))];
        let mut now: HashMap<Key, Option<Field>> = HashMap::new();
        let since = self.now_ms.saturating_sub(within_ms);
        for u in self.undo.iter().rev() {
            if u.rev != rev || u.at_ms < since {
                break;
            }
            let cur = now
                .get(&u.key)
                .cloned()
                .unwrap_or_else(|| self.store.get_key(&u.key).map(|e| e.field.clone()));
            let s = crate::ids::shard_of(u.key.form) as usize;
            if let Some(f) = cur.as_ref().filter(|f| keep(f)) {
                shards[s] = shards[s].wrapping_sub(crate::hash::record_hash(u.key.form, f));
            }
            if let Some(f) = u.old.as_ref().filter(|f| keep(f)) {
                shards[s] = shards[s].wrapping_add(crate::hash::record_hash(u.key.form, f));
            }
            now.insert(u.key, u.old.clone());
            rev = Rev(rev.0 - 1);
            out.push((rev, root(&shards)));
        }
        out
    }

    /// Drains guest input messages received so far.
    pub fn take_inputs(&mut self) -> Vec<(PlayerId, Tick, Vec<u8>)> {
        std::mem::take(&mut self.inputs)
    }

    /// Players currently in the session.
    pub fn players(&self) -> Vec<PlayerId> {
        self.guests.keys().copied().collect()
    }

    /// Sets the wall clock (ms) the live layer runs on. Drivers call this
    /// before feeding messages and ticking.
    pub fn set_time_ms(&mut self, ms: u64) {
        self.now_ms = ms;
    }

    /// Holds (`on`) or resumes the state sends to guest `p`. A guest's viewer
    /// loads its first cell without reading the link. Each send window that
    /// the guest did not ack made the host queue a new full snapshot (7 in one
    /// 60 s load), and the guest's receive buffer overflowed. A held guest gets
    /// its join snapshot, then nothing until it resumes: then the deltas start
    /// at the snapshot's revision.
    pub fn hold(&mut self, p: PlayerId, on: bool) {
        if let Some(g) = self.guests.get_mut(&p) {
            g.held = on;
        }
    }

    /// Removes a guest; its actors go back to the host. Messages the live
    /// layer wants to send about it go out with the next [`tick`](Self::tick).
    pub fn disconnect(&mut self, p: PlayerId) {
        self.guests.remove(&p);
        self.ring.remove(p);
        self.auth.release_all_of(p);
        let out = self.live.leave(p, self.now_ms);
        self.deferred.extend(out);
    }

    /// Feeds a live-layer message from the host player's own game.
    pub fn on_local(&mut self, msg: Msg) -> Vec<(PlayerId, Msg)> {
        self.live
            .on_msg(PlayerId::HOST, msg, self.now_ms, &mut self.auth)
    }

    fn cursor_for(&self, p: PlayerId) -> HostCursor {
        HostCursor {
            session: self.cfg.session,
            rev: self.store.rev(),
            min_resume: Rev::ZERO,
            last_seq: self.ring.last_seq(p),
        }
    }

    /// Handles one decoded message from `from`; returns replies.
    pub fn on_msg(&mut self, from: PlayerId, msg: Msg) -> Vec<(PlayerId, Msg)> {
        if from.is_host() {
            return vec![];
        }
        let mut out = Vec::new();
        match msg {
            Msg::Hello(h) => {
                if !self.guests.contains_key(&from) && self.guests.len() >= self.cfg.max_guests {
                    out.push((from, Msg::Reject(RejectReason::Full)));
                    if self.refused_full.insert(from) {
                        self.full_refusals += 1;
                    }
                } else if let crate::compat::Verdict::Refuse(r, found) =
                    crate::compat::decide(&self.cfg.hello, &h, self.cfg.compat)
                {
                    out.push((from, Msg::Incompat(self.cfg.hello.clone())));
                    out.push((from, Msg::Reject(r)));
                    self.compat_log.push(CompatNote {
                        player: from,
                        issues: found,
                        refused: true,
                    });
                } else {
                    if let crate::compat::Verdict::Warn(found) =
                        crate::compat::decide(&self.cfg.hello, &h, self.cfg.compat)
                    {
                        out.push((from, Msg::Incompat(self.cfg.hello.clone())));
                        self.compat_log.push(CompatNote {
                            player: from,
                            issues: found,
                            refused: false,
                        });
                    }
                    self.guests.entry(from).or_default().active = false;
                    self.refused_full.remove(&from);
                    out.push((
                        from,
                        Msg::Welcome {
                            session: self.cfg.session,
                            player: from,
                            tick: self.tick,
                            rev: self.store.rev(),
                        },
                    ));
                }
            }
            Msg::Join => {
                if let Some(g) = self.guests.get_mut(&from) {
                    g.active = true;
                    g.last_intent = 0;
                    g.last_rejected = 0;
                    self.ring.reset(from);
                    out.extend(self.live.join(from, self.now_ms));
                } else {
                    out.push((from, Msg::Reject(RejectReason::SessionMismatch)));
                }
            }
            Msg::Rejoin {
                session,
                last_seq,
                have_rev,
            } => {
                if !self.guests.contains_key(&from) {
                    out.push((from, Msg::Reject(RejectReason::SessionMismatch)));
                    return out;
                }
                let req = RejoinRequest {
                    session,
                    last_seq,
                    have_rev,
                };
                match session::decide_rejoin(&self.cursor_for(from), &req) {
                    RejoinDecision::Resume { base } => {
                        self.ring.resume_from(from, base);
                    }
                    RejoinDecision::FullSnapshot => self.ring.reset(from),
                    RejoinDecision::Reject(r) => {
                        out.push((from, Msg::Reject(r)));
                        return out;
                    }
                }
                self.guests.get_mut(&from).expect("checked").active = true;
                out.extend(self.live.join(from, self.now_ms));
            }
            Msg::Ack { seq } => {
                self.ring.on_ack(from, seq);
            }
            Msg::Input { tick, payload } => {
                if self.guests.get(&from).is_some_and(|g| g.active) {
                    self.inputs.push((from, tick, payload));
                }
            }
            Msg::Intent { id, epoch, changes } => {
                out.extend(self.on_intent(from, id, epoch, changes))
            }
            m @ (Msg::Avatar(_)
            | Msg::Actor(_)
            | Msg::Dialogue(_)
            | Msg::Loot(_)
            | Msg::Rules(_)
            | Msg::Social(_)
            | Msg::Scene(_)
            | Msg::Interact(_)
            | Msg::Roam(_)
            | Msg::Combat(_)) => {
                if self.guests.get(&from).is_some_and(|g| g.active) {
                    out.extend(self.live.on_msg(from, m, self.now_ms, &mut self.auth));
                }
            }
            Msg::Ping { nonce } => out.push((from, Msg::Pong { nonce })),
            Msg::RepairRequest { rev, shards } => {
                out.extend(self.on_repair_request(from, rev, shards))
            }
            _ => {}
        }
        out
    }

    fn on_intent(
        &mut self,
        from: PlayerId,
        id: u32,
        epoch: Epoch,
        changes: Vec<(FormId, Field)>,
    ) -> Vec<(PlayerId, Msg)> {
        let Some(g) = self.guests.get_mut(&from) else {
            return vec![];
        };
        if !g.active {
            return vec![];
        }
        if id <= g.last_intent {
            // retransmission: repeat the verdict, apply nothing
            return vec![(
                from,
                Msg::IntentAck {
                    id,
                    rejected: g.last_rejected,
                },
            )];
        }
        let mut rejected = 0u16;
        for (form, field) in changes {
            if self.auth.check_write(from, epoch, form, &field).is_ok() {
                self.write_logged(form, field);
            } else {
                rejected += 1;
            }
        }
        let g = self.guests.get_mut(&from).expect("present");
        g.last_intent = id;
        g.last_rejected = rejected;
        vec![(from, Msg::IntentAck { id, rejected })]
    }

    fn on_repair_request(
        &mut self,
        from: PlayerId,
        rev: Rev,
        shards: Vec<u64>,
    ) -> Vec<(PlayerId, Msg)> {
        let (Some(theirs), Some(ours)) = (ShardHashes::from_vec(shards), self.hist.get(rev)) else {
            return vec![];
        };
        let report = ours.diff(&theirs);
        if report.is_clean() {
            return vec![];
        }
        let repair = Repair::build(&self.store, &report);
        let msg = Msg::Repair(repair);
        if proto::encode(&msg).is_err() {
            // too big for one message: fall back to a full resync
            self.ring.reset(from);
            return vec![];
        }
        self.repairs_sent += 1;
        vec![(from, msg)]
    }

    /// Advances one tick and builds the per-client sends.
    pub fn tick(&mut self) -> Vec<(PlayerId, Msg)> {
        self.tick = Tick(self.tick.0 + 1);
        let mut out = std::mem::take(&mut self.deferred);
        out.extend(self.live.tick(self.now_ms, &mut self.auth));
        let active: Vec<PlayerId> = self
            .guests
            .iter()
            .filter(|(_, g)| g.active)
            .map(|(p, _)| *p)
            .collect();
        for p in active {
            // a held guest has its snapshot: the rest waits until it plays
            if self.guests[&p].held && self.ring.acked_rev(p).is_some() {
                continue;
            }
            match self.ring.plan(p, self.store.rev()) {
                SendPlan::Nothing => {}
                SendPlan::Delta { base } => {
                    let seq = self.ring.next_seq(p);
                    let mut n = 24;
                    loop {
                        let delta = self.store.delta_since(base, self.tick, n);
                        let rev = delta.rev;
                        let msg = Msg::Delta { seq, delta };
                        if proto::encode(&msg).is_ok() || n == 1 {
                            self.ring.record_sent(p, seq, rev);
                            out.push((p, msg));
                            break;
                        }
                        n = (n / 2).max(1);
                    }
                }
                SendPlan::Full => {
                    let id = self.ring.next_seq(p);
                    let snap = Snapshot::from_store(&self.store, self.tick);
                    if let Ok(bytes) = snap.encode() {
                        if let Ok(chunks) = chunk::split(id, &bytes, SNAPSHOT_CHUNK_BYTES) {
                            // The snapshot rides the reliable channel, so it counts as
                            // delivered: deltas continue from its revision at once. (An
                            // unreliable ack of it could be lost and stall the client.)
                            self.ring.resume_from(p, snap.rev);
                            self.snapshots_sent += 1;
                            out.extend(chunks.into_iter().map(|c| (p, Msg::Snapshot(c))));
                        }
                    }
                }
            }
        }
        out
    }

    /// Records the shard hashes at the current revision and tells every
    /// active guest the root.
    pub fn hash_broadcast(&mut self) -> Vec<(PlayerId, Msg)> {
        let h = ShardHashes::compute(&self.store);
        let root = h.root();
        let rev = self.store.rev();
        self.hist.record(rev, h);
        self.guests
            .iter()
            .filter(|(_, g)| g.active)
            .map(|(p, _)| {
                (
                    *p,
                    Msg::Hash {
                        tick: self.tick,
                        rev,
                        root,
                    },
                )
            })
            .collect()
    }

    /// Last send sequence for a guest (diagnostics).
    pub fn last_seq(&self, p: PlayerId) -> Seq {
        self.ring.last_seq(p)
    }

    /// Baseline the host believes the guest holds.
    pub fn acked_rev(&self, p: PlayerId) -> Option<Rev> {
        self.ring.acked_rev(p)
    }
}

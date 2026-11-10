//! coop-engine-session: the session between two games on the upstream engine.
//!
//! A thin driver over coop-core's sync. The host's game is the story's authority; each guest's
//! game follows it. The driver talks to a game only through the adapter of `coop-engine`: the
//! session's interface (the traits of `coop-game`, implemented by
//! [`coop_engine::game::EngineCoop`]) and a few calls of the adapter for what that interface
//! does not have yet (the space of the local player, an avatar, an actor's life). It sees the
//! game program through [`coop_engine::Program`] and names no engine type, so its tests run on
//! the fake program.
//!
//! - [`HostDriver`]: once per frame, between two of the game's frames, exports the shared state,
//!   writes what changed into coop-core's `HostSync` (each quest as one record,
//!   [`record`]), shares the clock about once a second, shares the deaths of placed actors,
//!   enacts the guests' death claims, and sends joins their snapshot, then deltas and hashes.
//! - [`GuestDriver`]: puts the guest gate on its game before the first frame of play, takes the
//!   host's messages into `GuestSync`, and applies what changed in its replica to its game as
//!   plain state. Every few steps it compares its game with the replica (the adapter's audit)
//!   and puts back what drifted.
//! - [`ladder`]: the players in the world: a guest moved to the host's space, and avatars.
//!
//! The transport is coop-core's `Transport`: coop-net's endpoint for a real session, the
//! loopback and chaos transports in tests.

pub mod ladder;
pub mod record;

use std::collections::{BTreeMap, BTreeSet};

use coop_core::avatar::{AvatarMsg, AvatarSender, Sample};
use coop_core::compat;
use coop_core::ids::{Epoch, FormId, PlayerId, Rev, Tick};
use coop_core::proto::{self, Hello, Msg};
use coop_core::schema::{Field, Key};
use coop_core::sync::{GuestLive, GuestPhase, GuestSync, HostConfig, HostSync};
use coop_core::transport::Transport;
use coop_engine::game::{EngineCoop, EngineState, NameTable};
use coop_engine::life::{self, LifeWatch};
use coop_engine::Program;
use coop_game::{Change, Names, Role, Setup, Shared};

use record::{QuestRecord, QUEST_RECORD_TAG};

/// The tag of the clock's `Field::Custom` (a clock global's value bits, little-endian). The
/// game's clock moves by itself on every machine, so it is not story state: the host writes it
/// about once a second, a guest sets its own clock to it, and the heal leaves it alone.
pub const CLOCK_TAG: u16 = 0x434b;
/// The host writes its clock, and a guest sets its own to the last one it has, this often.
pub const CLOCK_EVERY_MS: u64 = 1000;
/// The globals that the game's clock moves by itself (coop-core's volatile keys) and the
/// calendar that turns over with them, by editor id.
pub const CLOCK_NAMES: &[&str] = &[
    "GameHour",
    "GameDaysPassed",
    "GameDay",
    "GameMonth",
    "GameYear",
];

/// The tag of a placed actor's life (`Field::Custom`, one byte: 1 dead, 0 alive again), on the
/// actor's reference id. The host writes it when an actor of its game dies; a guest kills its
/// own copy with the game's kill. A guest whose own copy died sends the same field as a claim,
/// and the host's game does the kill, so the death's scripts and the story's answers run there,
/// once.
pub const LIFE_TAG: u16 = 0x4c46;

fn life_field(dead: bool) -> Field {
    Field::Custom {
        tag: LIFE_TAG,
        data: vec![u8::from(dead)],
    }
}

/// `Some(dead)` for a well-formed life field.
pub fn life_value(field: &Field) -> Option<bool> {
    match field {
        Field::Custom { tag, data } if *tag == LIFE_TAG && data.len() == 1 => Some(data[0] != 0),
        _ => None,
    }
}

fn clock_field(bits: u32) -> Field {
    Field::Custom {
        tag: CLOCK_TAG,
        data: bits.to_le_bytes().to_vec(),
    }
}

/// The value bits of a well-formed clock field.
pub fn clock_value(field: &Field) -> Option<u32> {
    match field {
        Field::Custom { tag, data } if *tag == CLOCK_TAG => {
            Some(u32::from_le_bytes(data[..].try_into().ok()?))
        }
        _ => None,
    }
}

/// The clock's globals of a load order (form ids), from their editor ids.
pub fn clock_ids(names: &NameTable) -> Vec<u32> {
    CLOCK_NAMES
        .iter()
        .filter_map(|n| names.form_of(n))
        .map(|f| f.0)
        .collect()
}

/// What this build quotes at the handshake for a game whose plugins load in `plugins` order:
/// the protocol, this crate's version as the build, the story channel's encoding as the rules,
/// the plugin list, the game version and the clock keys. Two games join only when these agree
/// (coop-core's `compat::decide`), so a guest never applies records made from other data.
pub fn hello(plugins: Vec<String>) -> Hello {
    let build = format!("coop-engine-session {}", env!("CARGO_PKG_VERSION"));
    let rules = format!(
        "quest-record {QUEST_RECORD_TAG:04x} v{} {} clock {CLOCK_TAG:04x} life {LIFE_TAG:04x}",
        record::VERSION,
        record::LEN,
    );
    Hello {
        proto: proto::PROTOCOL_VERSION,
        build_hash: compat::plugin_list_hash(&[build]),
        rules_hash: compat::plugin_list_hash(&[rules]),
        mods_hash: compat::plugin_list_hash(&plugins),
        exe: "1.4.0.525".into(),
        volatile_hash: compat::volatile_hash(compat::VOLATILE_KEYS),
        plugins,
    }
}

/// Encodes and sends `msg`. A message too large to encode, or a peer that the transport does
/// not have, is counted and dropped (coop-core's sync splits what it sends to fit; a refused
/// send ends the session through the transport's own liveness check).
fn send(net: &mut dyn Transport, to: PlayerId, msg: &Msg, failures: &mut u32) {
    match proto::encode(msg) {
        Ok(bytes) => {
            if net.send(to, msg.channel(), bytes).is_err() {
                *failures += 1;
            }
        }
        Err(_) => *failures += 1,
    }
}

/// The messages waiting on `net`, decoded. A packet that does not decode is counted and dropped.
fn receive(net: &mut dyn Transport, bad: &mut u32) -> Vec<(PlayerId, Msg)> {
    let mut out = Vec::new();
    while let Some(p) = net.recv() {
        match proto::decode(p.channel, &p.data) {
            Ok(msg) => out.push((p.from, msg)),
            Err(_) => *bad += 1,
        }
    }
    out
}

/// The story's fields of a store as the adapter's shared fields: each quest record as its
/// fields, each global and script variable that is not at its default. Clocks and lives are
/// left out.
pub fn story_fields<'a>(
    entries: impl Iterator<Item = (FormId, &'a Field)>,
) -> Vec<(FormId, Field)> {
    let mut out = Vec::new();
    for (form, field) in entries {
        match field {
            Field::Custom { tag, .. } if *tag == QUEST_RECORD_TAG => {
                if let Some(r) = QuestRecord::decode(field) {
                    out.extend(r.to_fields().into_iter().map(|f| (form, f)));
                }
            }
            Field::Global { .. } => out.push((form, field.clone())),
            Field::ScriptVar { .. } if !field.is_tombstone() => out.push((form, field.clone())),
            _ => {}
        }
    }
    out
}

/// The host's side.
pub struct HostDriver {
    /// coop-core's host sync: the replicated store, joins, deltas, hashes.
    pub sync: HostSync,
    /// The adapter's state of this game.
    pub game: EngineState,
    setup: Setup,
    started: bool,
    /// The clock's globals: shared as the clock, not as story globals.
    clock: BTreeSet<u32>,
    clock_values: BTreeMap<u32, u32>,
    clock_sent_ms: Option<u64>,
    /// The story's values last written to the store, by key.
    sent: BTreeMap<Key, (FormId, Field)>,
    life: LifeWatch,
    /// Guests' death claims that the host's game enacted.
    pub claims_enacted: u32,
    /// Guests' claims left alone: the actor is not in the host's game now, or is dead already.
    pub claims_dropped: u32,
    /// What happened to each claim, for the log (taken by the caller).
    pub claim_notes: Vec<String>,
    /// Quest records that could not be carried exactly: not sent.
    pub unencodable: u32,
    /// What cannot be sent, once each: the quest and why (for the log, taken by the caller).
    pub unencodable_notes: Vec<String>,
    noted: BTreeSet<(u32, u32)>,
    /// The live ladder (players in the world). `None` leaves the session story-only.
    pub ladder: Option<ladder::Ladder>,
    sender: AvatarSender,
    /// The host player's own view of the other players (their interpolated states).
    view: GuestLive,
    steps: u64,
    /// Hashes are broadcast every this many steps.
    pub hash_every: u64,
    /// The shared state is exported every this many steps (a real game's export reads every
    /// quest of the load order).
    pub export_every: u64,
    /// The change map is read for deaths every this many steps.
    pub life_every: u64,
    /// Packets that did not decode.
    pub bad_packets: u32,
    /// Sends that failed.
    pub send_failures: u32,
}

impl HostDriver {
    /// A host with coop-core's config `cfg`, on the game whose adapter state is `game`. `policy`:
    /// the campaign policy, its rows keyed by this load order (the quests that each player runs
    /// itself are not shared).
    pub fn new(
        cfg: HostConfig,
        game: EngineState,
        policy: Option<std::sync::Arc<coop_policy::PolicyTable>>,
    ) -> HostDriver {
        let mut sync = HostSync::new(cfg);
        sync.claim_tags.insert(LIFE_TAG);
        let clock = clock_ids(&game.names).into_iter().collect();
        HostDriver {
            sync,
            game,
            setup: Setup {
                role: Role::Host,
                me: 0,
                policy,
                keep_own_rows: false,
            },
            started: false,
            clock,
            clock_values: BTreeMap::new(),
            clock_sent_ms: None,
            sent: BTreeMap::new(),
            life: LifeWatch::default(),
            claims_enacted: 0,
            claims_dropped: 0,
            claim_notes: Vec::new(),
            unencodable: 0,
            unencodable_notes: Vec::new(),
            noted: BTreeSet::new(),
            ladder: None,
            sender: AvatarSender::new(PlayerId::HOST),
            view: GuestLive::new(PlayerId::HOST),
            steps: 0,
            hash_every: 10,
            export_every: 1,
            life_every: 1,
            bad_packets: 0,
            send_failures: 0,
        }
    }

    /// Starts the story channel: sets the role, names once what cannot be sent, and writes the
    /// whole shared state, so that a guest's join snapshot holds the whole story whatever the
    /// two games did before. [`HostDriver::step`] calls it first.
    pub fn start(&mut self, p: &mut dyn Program) -> Result<(), String> {
        let mut coop = EngineCoop::over(p, &mut self.game);
        coop.configure(&self.setup)?;
        for quest in coop.quests() {
            let (objectives, variables) = coop.too_wide(FormId(quest));
            for id in objectives {
                if self.noted.insert((quest, id)) {
                    self.unencodable_notes.push(format!(
                        "quest {quest:08x}: objective {id} has a number above 65535, not sendable"
                    ));
                }
            }
            for id in variables {
                if self.noted.insert((quest, id)) {
                    self.unencodable_notes.push(format!(
                        "quest {quest:08x}: variable id {id} is above 65535, not sendable"
                    ));
                }
            }
        }
        self.started = true;
        self.collect(p);
        Ok(())
    }

    /// Writes into the store what changed in the host's game since the last call: each quest
    /// as one record, each global and script variable. The clock's globals are kept for the
    /// clock. The store keeps only real changes.
    pub fn collect(&mut self, p: &mut dyn Program) {
        let mut coop = EngineCoop::over(p, &mut self.game);
        let export = coop.export_shared(true).unwrap_or_default();
        let mut by_quest: BTreeMap<u32, Vec<Field>> =
            coop.quests().into_iter().map(|q| (q, Vec::new())).collect();
        let mut now: BTreeMap<Key, (FormId, Field)> = BTreeMap::new();
        for (form, field) in export {
            if self.clock.contains(&form.0) {
                if let Field::Global { bits } = field {
                    self.clock_values.insert(form.0, bits);
                }
                continue;
            }
            if record::carries(&field) {
                by_quest.entry(form.0).or_default().push(field);
                continue;
            }
            now.insert(field.key(form), (form, field));
        }
        for (quest, fields) in by_quest {
            let form = FormId(quest);
            let r = QuestRecord::from_fields(&fields);
            let key = Field::Custom {
                tag: QUEST_RECORD_TAG,
                data: Vec::new(),
            }
            .key(form);
            match r.encode() {
                Some(f) => {
                    now.insert(key, (form, f));
                }
                None => {
                    self.unencodable += 1;
                    if self.noted.insert((quest, u32::MAX)) {
                        self.unencodable_notes.push(format!(
                            "quest {quest:08x}: record not sendable ({} objectives; a record holds at most 255 objectives in {} bytes)",
                            r.objectives.len(),
                            record::MAX_LEN
                        ));
                    }
                    // The guest keeps the last record that could be sent.
                    if let Some(old) = self.sent.get(&key) {
                        now.insert(key, old.clone());
                    }
                }
            }
        }
        for (key, (form, field)) in &self.sent {
            if !now.contains_key(key) {
                let gone = field.tombstone();
                if gone.is_tombstone() {
                    self.sync.write(*form, gone);
                }
            }
        }
        for (key, (form, field)) in &now {
            if self.sent.get(key).map(|(_, f)| f) != Some(field) {
                self.sync.write(*form, field.clone());
            }
        }
        self.sent = now;
    }

    /// Writes the deaths (and the lives again) of the placed actors of the host's game.
    fn collect_lives(&mut self, p: &mut dyn Program) {
        for (id, dead) in self.life.look(p) {
            let field = life_field(dead);
            if dead || self.sync.store.get(FormId(id), &field).is_some() {
                self.sync.write(FormId(id), field);
            }
        }
    }

    /// Enacts the guests' claims in the host's game: an actor that a guest's game saw die is
    /// killed here with the game's kill, outside any gate, so its death runs as any death on the
    /// host does, and it reaches every guest through the store.
    fn enact_claims(&mut self, p: &mut dyn Program) {
        for (from, form, field) in std::mem::take(&mut self.sync.claims) {
            let id = form.0;
            if life_value(&field) != Some(true) || !life::shareable(id) {
                self.claims_dropped += 1;
                continue;
            }
            match life::dead(p, id) {
                Some(false) => {
                    life::kill(p, id);
                    self.claims_enacted += 1;
                    self.claim_notes.push(format!(
                        "actor {id:08x} died in player {}'s game: killed here",
                        from.0
                    ));
                }
                Some(true) => self.claims_dropped += 1,
                None => {
                    self.claims_dropped += 1;
                    self.claim_notes.push(format!(
                        "actor {id:08x} died in player {}'s game, but is not in this game now",
                        from.0
                    ));
                }
            }
        }
    }

    /// Writes the clock into the store when it is due.
    fn write_clock(&mut self, now_ms: u64) {
        if self
            .clock_sent_ms
            .is_some_and(|t| now_ms.saturating_sub(t) < CLOCK_EVERY_MS)
        {
            return;
        }
        self.clock_sent_ms = Some(now_ms);
        for (id, bits) in &self.clock_values {
            self.sync.write(FormId(*id), clock_field(*bits));
        }
    }

    /// One step, between two of the game's frames: the guests' messages, this frame's
    /// changes, the sends, and every [`HostDriver::hash_every`] steps the hashes. `Err`: the
    /// game cannot take the host's role, and the session must not go on.
    pub fn step(
        &mut self,
        p: &mut dyn Program,
        net: &mut dyn Transport,
        now_ms: u64,
    ) -> Result<(), String> {
        if !self.started {
            self.start(p)?;
        }
        self.sync.set_time_ms(now_ms);
        for (from, msg) in receive(net, &mut self.bad_packets) {
            for (to, reply) in self.sync.on_msg(from, msg) {
                send(net, to, &reply, &mut self.send_failures);
            }
        }
        self.enact_claims(p);
        if self.steps % self.export_every.max(1) == 0 {
            self.collect(p);
        }
        if self.steps % self.life_every.max(1) == 0 {
            self.collect_lives(p);
        }
        self.write_clock(now_ms);
        let mut out = self.sync.tick();
        self.steps += 1;
        if self.steps % self.hash_every.max(1) == 0 {
            out.extend(self.sync.hash_broadcast());
        }
        for (to, msg) in out {
            send(net, to, &msg, &mut self.send_failures);
        }
        self.live_step(p, net, now_ms);
        Ok(())
    }

    /// The live ladder's frame: the other players' states into the host's view, the ladder's
    /// rungs, then this player's own state to every guest. The host's state goes to all of
    /// them, whatever space they are in: a guest needs it to come to the host.
    fn live_step(&mut self, p: &mut dyn Program, net: &mut dyn Transport, now_ms: u64) {
        let Some(ladder) = self.ladder.as_mut() else {
            return;
        };
        for msg in self.sync.live.take_local() {
            self.view.on_msg(msg, now_ms);
        }
        let players = self.sync.players();
        let peers: Vec<(PlayerId, Sample)> = players
            .iter()
            .filter_map(|p| Some((*p, self.view.sample(*p, now_ms)?)))
            .filter(|(_, s)| s.cell.0 != 0)
            .collect();
        let Some(me) = ladder.frame(p, now_ms, &peers) else {
            return;
        };
        let Some(m) = self.sender.tick(now_ms, &ladder::state_of(&me)) else {
            return;
        };
        let relayed = self.sync.live.on_msg(
            PlayerId::HOST,
            Msg::Avatar(m.clone()),
            now_ms,
            &mut self.sync.auth,
        );
        for (to, msg) in relayed {
            if !matches!(msg, Msg::Avatar(AvatarMsg::State { .. })) {
                send(net, to, &msg, &mut self.send_failures);
            }
        }
        for to in players {
            send(net, to, &Msg::Avatar(m.clone()), &mut self.send_failures);
        }
    }
}

/// The guest's side.
pub struct GuestDriver {
    /// coop-core's guest sync: the replica of the host's store.
    pub sync: GuestSync,
    /// The adapter's state of this game.
    pub game: EngineState,
    setup: Setup,
    gated: bool,
    /// Why this game could not take its gate. The session does not go on then.
    pub gate_error: Option<String>,
    started: bool,
    applied_upto: Rev,
    snapshots_applied: u32,
    repairs_applied: u32,
    steps: u64,
    clock: BTreeSet<u32>,
    clock_set_ms: Option<u64>,
    life: LifeWatch,
    /// Actors this guest has told the host died in its game.
    claimed: BTreeSet<u32>,
    /// Actors whose life changed in this game before the host could be told.
    life_pending: BTreeSet<u32>,
    /// How many death claims were sent.
    pub claims_sent: u32,
    /// Every this many steps the guest's game is compared with the replica (the adapter's
    /// audit), and what drifted is put back. Replica hashes compare replicas only; this keeps
    /// the game itself on the host's state.
    pub heal_every: u64,
    /// The change map is read for deaths every this many steps.
    pub life_every: u64,
    /// Slots put back because the game had drifted from the replica.
    pub healed: u32,
    /// Objectives put back into the player's objective list (the Pip-Boy's quest page).
    pub relisted: u32,
    /// The distinct slots healed (the first 40), for the log: what on this guest keeps
    /// changing shared state.
    pub healed_notes: Vec<String>,
    healed_seen: BTreeSet<(u32, u8)>,
    /// Replica entries that this guest's game does not have (a quest that is not in its
    /// catalog, a record that does not decode): the two games' data differ.
    pub refused: u32,
    /// The live ladder (players in the world). `None` leaves the session story-only.
    pub ladder: Option<ladder::Ladder>,
    /// Packets that did not decode.
    pub bad_packets: u32,
    /// Sends that failed.
    pub send_failures: u32,
}

impl GuestDriver {
    /// A guest in slot `player` that quotes `hello` to the host, on the game whose adapter
    /// state is `game`. `policy`: the campaign policy, keyed by this load order.
    pub fn new(
        player: PlayerId,
        hello: Hello,
        game: EngineState,
        policy: Option<std::sync::Arc<coop_policy::PolicyTable>>,
    ) -> GuestDriver {
        let clock = clock_ids(&game.names).into_iter().collect();
        GuestDriver {
            sync: GuestSync::new(player, hello),
            game,
            setup: Setup {
                role: Role::Guest,
                me: player.0,
                policy,
                keep_own_rows: false,
            },
            gated: false,
            gate_error: None,
            started: false,
            applied_upto: Rev(0),
            snapshots_applied: 0,
            repairs_applied: 0,
            steps: 0,
            clock,
            clock_set_ms: None,
            life: LifeWatch::default(),
            claimed: BTreeSet::new(),
            life_pending: BTreeSet::new(),
            claims_sent: 0,
            heal_every: 20,
            life_every: 1,
            healed: 0,
            relisted: 0,
            healed_notes: Vec::new(),
            healed_seen: BTreeSet::new(),
            refused: 0,
            ladder: None,
            bad_packets: 0,
            send_failures: 0,
        }
    }

    /// Makes this game a guest's: from now on the story's scripts are gated (the host runs
    /// them). Call it when a machine sets out to join, before its first frame of play: a story
    /// script that ran on the guest in the meantime would change state that the host never
    /// sends back. [`GuestDriver::step`] calls it too. `Err` tells why the game cannot take the
    /// gate: the session must not go on then.
    pub fn gate(&mut self, p: &mut dyn Program) -> Result<(), String> {
        if self.gated {
            return Ok(());
        }
        if let Some(e) = &self.gate_error {
            return Err(e.clone());
        }
        match EngineCoop::over(p, &mut self.game).configure(&self.setup) {
            Ok(()) => {
                self.gated = true;
                // What died in this game before now is not news.
                self.life.look(p);
                Ok(())
            }
            Err(e) => {
                self.gate_error = Some(e.clone());
                Err(e)
            }
        }
    }

    /// One step, between two of the game's frames: the first gates this game and says hello;
    /// each takes the host's messages and applies what changed. `Err`: the game could not take
    /// its gate, and nothing else was done.
    pub fn step(
        &mut self,
        p: &mut dyn Program,
        net: &mut dyn Transport,
        now_ms: u64,
    ) -> Result<(), String> {
        self.gate(p)?;
        self.sync.set_time_ms(now_ms);
        if !self.started {
            for msg in self.sync.start() {
                send(net, PlayerId::HOST, &msg, &mut self.send_failures);
            }
            self.started = true;
        }
        for (_, msg) in receive(net, &mut self.bad_packets) {
            for reply in self.sync.on_msg(msg) {
                send(net, PlayerId::HOST, &reply, &mut self.send_failures);
            }
        }
        if let Some(ladder) = self.ladder.as_mut() {
            let ids: Vec<PlayerId> = self.sync.live.remote.keys().copied().collect();
            let peers: Vec<(PlayerId, Sample)> = ids
                .into_iter()
                .filter_map(|id| Some((id, self.sync.live.sample(id, now_ms)?)))
                .filter(|(_, s)| s.cell.0 != 0)
                .collect();
            if let Some(me) = ladder.frame(p, now_ms, &peers) {
                // Sent by coop-core's guest sync at its own rate, in the tick below.
                self.sync.live.local = ladder::state_of(&me);
            }
        }
        for msg in self.sync.tick(Tick(0)) {
            send(net, PlayerId::HOST, &msg, &mut self.send_failures);
        }
        if self.steps % self.life_every.max(1) == 0 {
            self.claim_deaths(p, net);
        }
        self.apply(p);
        self.keep_clock(p, now_ms);
        self.steps += 1;
        if self.steps % self.heal_every.max(1) == 0 {
            self.heal(p);
        }
        Ok(())
    }

    /// After the connection to the host was lost and the transport joined again as `player`:
    /// resumes from this guest's replica (a host that started a new session answers with a
    /// fresh join and a snapshot, which [`GuestDriver::apply`] applies whole).
    pub fn rejoin(&mut self, player: PlayerId, net: &mut dyn Transport) {
        self.sync.player = player;
        for msg in self.sync.reconnect() {
            send(net, PlayerId::HOST, &msg, &mut self.send_failures);
        }
    }

    /// Tells the host of each placed actor that died in this game and is not dead in the
    /// host's story (a death applied from the host is in the replica by then, so it is not sent
    /// back).
    fn claim_deaths(&mut self, p: &mut dyn Program, net: &mut dyn Transport) {
        self.life_pending.extend(
            self.life
                .look(p)
                .into_iter()
                .filter(|(_, dead)| *dead)
                .map(|(id, _)| id),
        );
        if self.sync.phase() != GuestPhase::Active || self.life_pending.is_empty() {
            return;
        }
        let dead = life_field(true);
        let mut claims = Vec::new();
        for id in std::mem::take(&mut self.life_pending) {
            let known = self
                .sync
                .store
                .get(FormId(id), &dead)
                .is_some_and(|en| life_value(&en.field) == Some(true));
            if life::dead(p, id) == Some(true) && !known && self.claimed.insert(id) {
                claims.push((FormId(id), dead.clone()));
            }
        }
        if claims.is_empty() {
            return;
        }
        self.claims_sent += claims.len() as u32;
        for msg in self.sync.send_intent(Epoch(0), claims) {
            send(net, PlayerId::HOST, &msg, &mut self.send_failures);
        }
    }

    /// Sets this game's clock to the host's last one again, about once a second: a clock that
    /// the host has stopped (a menu, a wait) does not arrive again, and this game's own would
    /// run on.
    fn keep_clock(&mut self, p: &mut dyn Program, now_ms: u64) {
        if self
            .clock_set_ms
            .is_some_and(|t| now_ms.saturating_sub(t) < CLOCK_EVERY_MS)
        {
            return;
        }
        self.clock_set_ms = Some(now_ms);
        let changes: Vec<Change> = self
            .sync
            .store
            .iter()
            .filter(|(k, _)| self.clock.contains(&k.form.0))
            .filter_map(|(k, en)| {
                let global = Field::Global {
                    bits: clock_value(&en.field)?,
                };
                Some((global.key(k.form), Some(global)))
            })
            .collect();
        if !changes.is_empty() {
            EngineCoop::over(p, &mut self.game).apply_shared(&changes, false);
        }
    }

    /// The changes that make one quest of this game the record's: each field of the record,
    /// and each record field that this game has and the record does not. `None`: this game's
    /// catalog has no such quest.
    fn quest_changes(coop: &EngineCoop<'_>, form: FormId, r: &QuestRecord) -> Option<Vec<Change>> {
        let current = coop.fields_of(form)?;
        let want = r.to_fields();
        let keys: BTreeSet<Key> = want.iter().map(|f| f.key(form)).collect();
        let mut changes: Vec<Change> = want.into_iter().map(|f| (f.key(form), Some(f))).collect();
        for f in current {
            if record::carries(&f) && !keys.contains(&f.key(form)) {
                changes.push((f.key(form), None));
            }
        }
        Some(changes)
    }

    /// Applies to the game what changed in the replica since the last call; after a snapshot or
    /// a repair replaced a part of the replica, everything (their entries can carry revisions
    /// at or below the applied mark: a rejoin, or a host that started a new session).
    pub fn apply(&mut self, p: &mut dyn Program) {
        let all = self.sync.snapshots_loaded != self.snapshots_applied
            || self.sync.repairs_applied != self.repairs_applied;
        let due: Vec<(FormId, Field)> = self
            .sync
            .store
            .iter()
            .filter(|(_, en)| all || en.rev > self.applied_upto)
            .map(|(k, en)| (k.form, en.field.clone()))
            .collect();
        self.applied_upto = self.sync.store.rev();
        self.snapshots_applied = self.sync.snapshots_loaded;
        self.repairs_applied = self.sync.repairs_applied;
        if due.is_empty() {
            return;
        }
        let mut changes: Vec<Change> = Vec::new();
        let mut kills = Vec::new();
        {
            let coop = EngineCoop::over(p, &mut self.game);
            for (form, field) in due {
                match &field {
                    Field::Custom { tag, .. } if *tag == QUEST_RECORD_TAG => {
                        match QuestRecord::decode(&field)
                            .and_then(|r| Self::quest_changes(&coop, form, &r))
                        {
                            Some(c) => changes.extend(c),
                            None => self.refused += 1,
                        }
                    }
                    Field::Custom { tag, .. } if *tag == LIFE_TAG => {
                        // A death: this game's copy dies by the game's kill, its scripts held
                        // back (they ran on the host). An actor alive again on the host is left
                        // as it is here: nothing here can raise it yet.
                        if life_value(&field) == Some(true) {
                            kills.push(form.0);
                        }
                    }
                    Field::Global { .. } | Field::ScriptVar { .. } => {
                        changes.push((field.key(form), Some(field)));
                    }
                    // The clock is kept by its own step.
                    _ => {}
                }
            }
        }
        if !changes.is_empty() {
            EngineCoop::over(p, &mut self.game).apply_shared(&changes, false);
        }
        for id in kills {
            if life::dead(p, id) == Some(false) {
                p.applying(true);
                life::kill(p, id);
                p.applying(false);
            }
        }
    }

    /// Compares this game with the replica (the adapter's audit), and applies again each slot
    /// that drifted. Puts each shown objective of a shared quest that is missing from the
    /// player's objective list back into it. A death that this game does not have yet is
    /// applied again too.
    pub fn heal(&mut self, p: &mut dyn Program) {
        let want = story_fields(self.sync.store.iter().map(|(k, en)| (k.form, &en.field)));
        let quests: Vec<FormId> = self
            .sync
            .store
            .iter()
            .filter(|(_, en)| matches!(&en.field, Field::Custom { tag, .. } if *tag == QUEST_RECORD_TAG))
            .map(|(k, _)| k.form)
            .collect();
        let deaths: Vec<u32> = self
            .sync
            .store
            .iter()
            .filter(|(_, en)| life_value(&en.field) == Some(true))
            .map(|(k, _)| k.form.0)
            .collect();
        let clock = self.clock.clone();
        let mut coop = EngineCoop::over(p, &mut self.game);
        let here: Vec<(FormId, Field)> = coop
            .export_shared(true)
            .unwrap_or_default()
            .into_iter()
            .filter(|(form, _)| !clock.contains(&form.0))
            .collect();
        self.game.audit.snapshot(&want);
        let leaks = self.game.audit.leaks(&here);
        let mut changes: Vec<Change> = Vec::new();
        for leak in &leaks {
            let Some(like) = leak.host.as_ref().or(leak.here.as_ref()) else {
                continue;
            };
            changes.push((like.key(leak.form), leak.host.clone()));
            if self.healed_seen.len() < 40 && self.healed_seen.insert((leak.form.0, leak.kind)) {
                self.healed_notes.push(format!(
                    "form {:08x}: this game had {:?}, the host's story has {:?}",
                    leak.form.0, leak.here, leak.host
                ));
            }
        }
        self.healed += changes.len() as u32;
        let mut coop = EngineCoop::over(p, &mut self.game);
        if !changes.is_empty() {
            coop.apply_shared(&changes, false);
        }
        for quest in quests {
            self.relisted += coop.relist(quest);
        }
        for id in deaths {
            if life::dead(p, id) == Some(false) {
                p.applying(true);
                life::kill(p, id);
                p.applying(false);
                self.healed += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_hello_is_sane_and_names_the_plugins_in_order() {
        let a = hello(vec!["falloutnv.esm".into(), "deadmoney.esm".into()]);
        let b = hello(vec!["deadmoney.esm".into(), "falloutnv.esm".into()]);
        assert!(compat::hello_is_sane(&a));
        assert_ne!(a.mods_hash, b.mods_hash);
        assert_eq!(a.build_hash, hello(vec![]).build_hash);
    }

    #[test]
    fn clock_and_life_fields_round_trip() {
        let f = clock_field(12.5f32.to_bits());
        assert_eq!(clock_value(&f), Some(12.5f32.to_bits()));
        assert_eq!(clock_value(&life_field(true)), None);
        assert_eq!(life_value(&life_field(true)), Some(true));
        assert_eq!(life_value(&life_field(false)), Some(false));
        assert_eq!(life_value(&f), None);
        let names = NameTable::parse("P 00 FalloutNV.esm\nF GLOB 00000038 GameHour\nF GLOB 00000039 GameDaysPassed\nF GLOB 0000003a Other\n").unwrap();
        assert_eq!(clock_ids(&names), [0x38, 0x39]);
    }
}

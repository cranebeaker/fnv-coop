//! Co-op's session on the translated engine (docs/coop/DECISIONS.md D2; docs/coop/SPECS.md S5).
//!
//! A thin driver over coop-core's sync. The host's game is the story's authority; the guests'
//! games follow it.
//!
//! - [`HostDriver`]: at its start writes every quest's record and script variables; then once per
//!   frame, between the game's frames, drains co-op's change journal, reads again the record
//!   ([`record`]) and variables of every quest that changed and every global whose value changed,
//!   and writes them into coop-core's `HostSync`, which sends joins their snapshot, then deltas,
//!   and hashes.
//! - [`GuestDriver`]: once per frame, takes the host's messages into `GuestSync` and applies what
//!   changed in its replica to the guest's game: quests through `quest::apply`, quest variables and
//!   globals through the game's values, all inside the script gate, so the story's scripts run only
//!   on the host.
//! - [`echo`]: reads the guest's game back and compares it with the replica, so a test (and a
//!   run on a PC) checks the game itself, not only the replica.
//!
//! The transport is coop-core's `Transport`: coop-net's endpoint in the game, the loopback and
//! chaos transports in tests.

// fights
pub mod fight;
pub mod ladder;
pub mod record;
// items: loot taken once and barter through the host (docs/coop/SPECS.md S18).
pub mod items;
// world
pub mod world;

// social: conversations, party saves and the rest vote (docs/coop/SPECS.md S12 to S15).
pub mod party;
pub mod rest;
pub mod social;
pub mod talk;

// vats: V.A.T.S. with several players (docs/coop/SPECS.md S20); actors the host's game does
// not run, and held actors moved in steps (S21).
pub mod far;
pub mod smooth;
pub mod vats;

use std::collections::BTreeSet;

use coop_core::avatar::{AvatarMsg, AvatarSender, Sample};
use coop_core::compat;
use coop_core::delta::StateStore;
use coop_core::ids::{Epoch, FormId, PlayerId, Rev, Tick};
use coop_core::proto::{self, Hello, Msg};
use coop_core::schema::Field;
use coop_core::sync::{GuestLive, HostConfig, HostSync};
use coop_core::sync::{GuestPhase, GuestSync};
use coop_core::transport::Transport;
use coop_engine::forms::{self, GlobalWatch};
use coop_engine::{actors, addr, gate, quest, vars};
use engine_core::Engine;

/// The tag of a clock global's `Field::Custom` (its value's bits, little-endian). The game's clock
/// moves by itself on every machine, so it is not story state: the host writes it about once a
/// second, a guest sets its own clock to it, and [`echo`] and the heal leave it alone (a guest's
/// clock is always a little off between two writes).
pub const CLOCK_TAG: u16 = 0x434b;
/// The host writes its clock, and a guest sets its own to the last one it has, this often.
pub const CLOCK_EVERY_MS: u64 = 1000;

/// The tag of a placed actor's life (`Field::Custom`, one byte: 1 dead, 0 alive again), on the
/// actor's reference id. The host writes it when an actor of its game dies; a guest kills its own
/// copy through the game. A guest whose own copy died sends the same field as a claim, and the
/// host's game does the kill (so the death's scripts and the story's reactions run there, once).
pub const LIFE_TAG: u16 = 0x4c46;

fn life_field(dead: bool) -> Field {
    Field::Custom {
        tag: LIFE_TAG,
        data: vec![u8::from(dead)],
    }
}

/// `Some(dead)` for a well-formed life field.
fn life_value(field: &Field) -> Option<bool> {
    match field {
        Field::Custom { tag, data } if *tag == LIFE_TAG && data.len() == 1 => Some(data[0] != 0),
        _ => None,
    }
}

/// The placed actors among `changes` whose life state changed and that both games know by id.
fn life_changes(changes: &[coop_engine::Change]) -> Vec<u32> {
    changes
        .iter()
        .filter(|c| {
            actors::is_actor_type(c.kind)
                && c.flags & addr::CHANGE_ACTOR_LIFESTATE != 0
                && actors::shareable(c.form)
        })
        .map(|c| c.form)
        .collect()
}

fn clock_field(v: f32) -> Field {
    Field::Custom {
        tag: CLOCK_TAG,
        data: v.to_bits().to_le_bytes().to_vec(),
    }
}

fn clock_value(field: &Field) -> Option<f32> {
    match field {
        Field::Custom { tag, data } if *tag == CLOCK_TAG => Some(f32::from_bits(
            u32::from_le_bytes(data[..].try_into().ok()?),
        )),
        _ => None,
    }
}

/// What this build quotes at the handshake for a game whose plugins load in `plugins` order: the
/// protocol, this crate's version as the build, the story channel's encoding as the rules, the
/// plugin list, the game version the engine runs, and the clock keys. Two games join only when
/// these agree (coop-core's `compat::decide`), so a guest never applies records made from other
/// data.
pub fn hello(plugins: Vec<String>) -> Hello {
    let build = format!("coop-engine-session {}", env!("CARGO_PKG_VERSION"));
    let rules = format!(
        "quest-record {:04x} v{} {} clock {:04x} life {:04x}",
        record::QUEST_RECORD_TAG,
        record::VERSION,
        record::LEN,
        CLOCK_TAG,
        LIFE_TAG
    );
    // world
    let rules = format!("{rules} {}", world::rules());
    // items
    let rules = format!("{rules} {}", items::rules());
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

/// Encodes and sends `msg`; a message too large to encode, or a peer the transport does not have,
/// is counted and dropped (coop-core's sync splits what it sends to fit; a refused send ends the
/// session through the transport's own liveness check).
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

/// The messages waiting on `net`, decoded; a packet that does not decode is counted and dropped.
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

/// The host's side.
pub struct HostDriver {
    /// coop-core's host sync: the replicated store, joins, deltas, hashes.
    pub sync: HostSync,
    quests: BTreeSet<u32>,
    globals: GlobalWatch,
    /// The game clock's globals (hour, day, month, year, days passed): sent as the clock, not as
    /// story globals.
    clock: Vec<u32>,
    clock_sent_ms: Option<u64>,
    /// Guests' death claims the host's game enacted.
    pub claims_enacted: u32,
    /// Guests' claims left alone: the actor is not in the host's game now, or is dead already.
    pub claims_dropped: u32,
    /// What happened to each claim, for the log (taken by the caller).
    pub claim_notes: Vec<String>,
    /// Quest records or variables that could not be carried exactly (see [`record::encode`]; a
    /// variable id above 65535): not sent.
    pub unencodable: u32,
    /// The live ladder (players in the world); `None` leaves the session story-only.
    pub ladder: Option<ladder::Ladder>,
    /// Shared fights (fights); `None` leaves every game running its own actors.
    pub fight: Option<fight::Fight>,
    sender: AvatarSender,
    /// The host player's own view of the other players (their interpolated states).
    view: GuestLive,
    /// What could not be sent, once each: the quest and why (for the log).
    pub unencodable_notes: Vec<String>,
    noted: BTreeSet<(u32, u32)>,
    steps: u64,
    /// Hashes are broadcast every this many steps.
    pub hash_every: u64,
    /// Packets that did not decode.
    pub bad_packets: u32,
    /// Sends that failed.
    pub send_failures: u32,
    // world
    /// The rest of the world's state (records, doors, locks); `None` leaves it unshared. Set it
    /// with [`HostDriver::share_world`].
    pub world: Option<world::HostWorld>,
    // items
    /// Loot and barter through this host's game; `None` leaves every game its own items.
    pub items: Option<items::HostItems>,
    // social
    /// Conversations, party saves and the rest vote.
    pub social: social::HostSocial,
}

impl HostDriver {
    // world
    /// Shares the rest of the world's state: changed forms' records to the guests, doors and
    /// locks both ways (docs/coop/SPECS.md S12).
    pub fn share_world(&mut self, w: world::HostWorld) {
        self.sync.claim_tags.insert(world::LOCK_TAG);
        self.world = Some(w);
    }

    /// A host with coop-core's config `cfg`, replicating the quests `quest_ids` (the shared story's:
    /// leave out the quests each player runs for themself; all of them are
    /// written at the start, so a guest gets every quest's state from the snapshot, whatever save
    /// either side loaded) and the globals `global_ids` (the game's globals do not pass through the
    /// change journal, so they are compared each step; leave out the ones that change by
    /// themselves, the game clock's).
    pub fn new(
        cfg: HostConfig,
        quest_ids: impl IntoIterator<Item = u32>,
        global_ids: impl IntoIterator<Item = u32>,
    ) -> HostDriver {
        let mut sync = HostSync::new(cfg);
        sync.claim_tags.insert(LIFE_TAG);
        HostDriver {
            sync,
            quests: quest_ids.into_iter().collect(),
            globals: GlobalWatch::new(global_ids),
            clock: Vec::new(),
            clock_sent_ms: None,
            claims_enacted: 0,
            claims_dropped: 0,
            claim_notes: Vec::new(),
            unencodable: 0,
            ladder: None,
            fight: None, // fights
            sender: AvatarSender::new(PlayerId::HOST),
            view: GuestLive::new(PlayerId::HOST),
            unencodable_notes: Vec::new(),
            noted: BTreeSet::new(),
            steps: 0,
            hash_every: 10,
            bad_packets: 0,
            send_failures: 0,
            world: None, // world
            items: None, // items
            // social
            social: social::HostSocial::default(),
        }
    }

    /// Shares the game's time: the host's values of the clock globals `ids` go to the guests about
    /// once a second, so day and night are the same for everyone.
    pub fn set_clock(&mut self, ids: impl IntoIterator<Item = u32>) {
        self.clock = ids.into_iter().collect();
    }

    /// Enacts the guests' claims in the host's game: an actor a guest's game saw die is killed
    /// here through the game, outside any gate, so its death runs as any death on the host does
    /// (and reaches every guest through the journal).
    fn enact_claims(&mut self, e: &mut Engine) {
        for (from, form, field) in std::mem::take(&mut self.sync.claims) {
            let id = form.0;
            if life_value(&field) != Some(true) || !actors::shareable(id) {
                self.claims_dropped += 1;
                continue;
            }
            match actors::dead(e, id) {
                Some(false) => {
                    actors::kill(e, id);
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

    /// Writes the clock into the replica when it is due.
    fn write_clock(&mut self, e: &mut Engine, now_ms: u64) {
        if self
            .clock_sent_ms
            .is_some_and(|t| now_ms.saturating_sub(t) < CLOCK_EVERY_MS)
        {
            return;
        }
        self.clock_sent_ms = Some(now_ms);
        for id in self.clock.clone() {
            if let Some(v) = forms::global_value(e, id) {
                self.sync.write(FormId(id), clock_field(v));
            }
        }
    }

    /// Starts the story channel: arms the change journal and writes every replicated quest's
    /// record and variables and every replicated global's value, so a guest's join snapshot holds
    /// the whole story whatever the two games did before.
    pub fn start(&mut self, e: &mut Engine) {
        coop_engine::arm(e, true);
        for id in self.quests.clone() {
            self.write_quest(e, id);
        }
        self.collect(e);
    }

    /// Writes quest `id`'s record and its script's variables into the replica.
    fn write_quest(&mut self, e: &mut Engine, id: u32) {
        if let Some(r) = quest::record(e, id) {
            match record::encode(&r) {
                Some(f) => {
                    self.sync.write(FormId(id), f);
                }
                None => {
                    self.unencodable += 1;
                    if self.noted.insert((id, u32::MAX)) {
                        self.unencodable_notes.push(format!(
                            "quest {id:08x}: record not sendable ({} objectives, highest state {}; a record holds at most 255 objectives in {} bytes)",
                            r.objectives.len(),
                            r.objectives.iter().map(|o| o.1).max().unwrap_or(0),
                            record::MAX_LEN
                        ));
                    }
                }
            }
        }
        for (var, bits) in vars::quest_vars(e, id).unwrap_or_default() {
            match u16::try_from(var) {
                Ok(index) => {
                    self.sync
                        .write(FormId(id), Field::ScriptVar { index, bits });
                }
                Err(_) => {
                    self.unencodable += 1;
                    if self.noted.insert((id, var)) {
                        self.unencodable_notes.push(format!(
                            "quest {id:08x}: variable id {var} is above 65535, not sendable"
                        ));
                    }
                }
            }
        }
    }

    /// Writes into the replica what changed in the host's game since the last call: the record and
    /// variables of every quest whose form changed (stages, flags, objectives, variables), and
    /// every global whose value changed. The replica keeps only real changes.
    pub fn collect(&mut self, e: &mut Engine) {
        // Only the quests this session replicates: a player's own quests (the opening, character
        // creation, ...) change on the host too and are not the shared story.
        let changes = coop_engine::drain(e);
        // world
        if let Some(w) = self.world.as_mut() {
            w.collect(e, &mut self.sync, &changes);
        }
        // items
        if let Some(i) = self.items.as_mut() {
            i.collect(e, &mut self.sync, &changes);
        }
        let quests: BTreeSet<u32> = changes
            .iter()
            .filter(|c| c.kind == addr::FORM_TYPE_QUEST && self.quests.contains(&c.form))
            .map(|c| c.form)
            .collect();
        for id in quests {
            self.write_quest(e, id);
        }
        // A placed actor that died (or, once it has an entry, lives again).
        for id in life_changes(&changes) {
            let Some(dead) = actors::dead(e, id) else {
                continue;
            };
            let field = life_field(dead);
            if dead || self.sync.store.get(FormId(id), &field).is_some() {
                self.sync.write(FormId(id), field);
            }
        }
        for (id, v) in self.globals.diff(e) {
            self.sync
                .write(FormId(id), Field::Global { bits: v.to_bits() });
        }
    }

    /// One step, between two of the game's frames: the guests' messages, this frame's changes,
    /// the sends, and every [`HostDriver::hash_every`] steps the hashes.
    pub fn step(&mut self, e: &mut Engine, net: &mut dyn Transport, now_ms: u64) {
        self.sync.set_time_ms(now_ms);
        for (from, msg) in receive(net, &mut self.bad_packets) {
            // vats: the actors a guest's game runs and the host's does not. A guest's report
            // of them is the fights' to judge, not coop-core's to enter as it comes.
            if let Some(f) = self.fight.as_mut() {
                for (to, m) in f.host_pre(&mut self.sync, from, &msg) {
                    send(net, to, &m, &mut self.send_failures);
                }
                if matches!(msg, Msg::Actor(coop_core::actors::ActorMsg::Track { .. })) {
                    continue;
                }
            }
            for (to, reply) in self.sync.on_msg(from, msg) {
                send(net, to, &reply, &mut self.send_failures);
            }
        }
        // world: doors and locks guests moved, before the other claims are judged.
        // items: the item layer's requests go through the same dispatcher.
        let enacted = match (self.world.as_mut(), self.items.as_mut()) {
            (Some(w), i) => w.enact_with(e, &mut self.sync, i),
            (None, Some(i)) => i.enact_alone(e, &mut self.sync),
            (None, None) => Vec::new(),
        };
        for (to, msg) in enacted {
            send(net, to, &msg, &mut self.send_failures);
        }
        // social
        social::host_step(self, e, net, now_ms);
        self.enact_claims(e);
        self.collect(e);
        self.write_clock(e, now_ms);
        // fights
        if let Some(f) = self.fight.as_mut() {
            let avatars = self
                .ladder
                .as_ref()
                .map(ladder::Ladder::avatar_ids)
                .unwrap_or_default();
            for (to, msg) in f.host_step(e, &mut self.sync, &avatars, now_ms) {
                send(net, to, &msg, &mut self.send_failures);
            }
            for (to, msg) in f.host_body(e, &mut self.sync, &mut self.view, now_ms) {
                send(net, to, &msg, &mut self.send_failures);
            }
        }
        let mut out = self.sync.tick();
        self.steps += 1;
        if self.steps % self.hash_every.max(1) == 0 {
            out.extend(self.sync.hash_broadcast());
        }
        for (to, msg) in out {
            send(net, to, &msg, &mut self.send_failures);
        }
        self.live_step(e, net, now_ms);
    }

    /// The live ladder's frame: the other players' states into the host's view, the ladder's
    /// rungs, then this player's own state to every guest. The host's state goes to all of them,
    /// whatever cell they are in: a guest needs it to come to the host.
    fn live_step(&mut self, e: &mut Engine, net: &mut dyn Transport, now_ms: u64) {
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
        let Some(me) = ladder.frame(e, now_ms, &peers) else {
            return;
        };
        // fights: the host's own health and fall, for the other players.
        let mut state = ladder::state_of(&me);
        if let Some(f) = self.fight.as_ref() {
            f.stamp(e, &mut state);
        }
        let Some(m) = self.sender.tick(now_ms, &state) else {
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

/// Makes `e`'s game a guest's: from now on the story's scripts are gated (the host runs them). Call
/// it when a machine sets out to join, from its first frame, before the host has answered: a story
/// script that ran on the guest in the meantime would change state the host never sends back.
/// [`GuestDriver::step`] calls it too.
pub fn gate_guest(e: &mut Engine) {
    gate::set_guest(e, true);
}

/// The guest's side.
pub struct GuestDriver {
    /// coop-core's guest sync: the replica of the host's store.
    pub sync: GuestSync,
    started: bool,
    applied_upto: Rev,
    snapshots_applied: u32,
    repairs_applied: u32,
    steps: u64,
    clock_set_ms: Option<u64>,
    /// Actors this guest has told the host died in its game.
    claimed: BTreeSet<u32>,
    /// Actors whose life changed in this game before the host could be told.
    life_pending: BTreeSet<u32>,
    /// How many such claims were sent.
    pub claims_sent: u32,
    /// Every this many steps the guest's game is read back ([`echo`]) and whatever drifted from the
    /// replica is applied again (a guest's own script writing a story global, a rejoin's snapshot
    /// whose revisions do not pass the applied mark). Replica hashes compare replicas only; this
    /// keeps the game itself on the host's state.
    pub heal_every: u64,
    /// Entries applied again because the game had drifted from the replica.
    pub healed: u32,
    /// Objectives put back into the player's objective list (the Pip-Boy's quest page).
    pub relisted: u32,
    /// The distinct entries healed (the first 40), for the log: what on this guest keeps changing
    /// shared state.
    pub healed_notes: Vec<String>,
    healed_seen: BTreeSet<(u32, u8)>,
    /// The live ladder (players in the world); `None` leaves the session story-only.
    pub ladder: Option<ladder::Ladder>,
    /// Shared fights (fights); `None` leaves this game running its own actors.
    pub fight: Option<fight::Fight>,
    /// Drifted entries the game refused when applied again (counted once each time they are
    /// tried; the same entry is in [`GuestDriver::refused`] from its first apply).
    pub heal_refused: u32,
    /// Replica entries the guest's game refused (a quest stage it does not have, a form that is
    /// not a quest or a global): the two games' data differ.
    pub refused: u32,
    /// Packets that did not decode.
    pub bad_packets: u32,
    /// Sends that failed.
    pub send_failures: u32,
    // world
    /// The rest of the world's state (records, doors, locks); `None` leaves it unshared.
    pub world: Option<world::GuestWorld>,
    // items
    /// Loot and barter through the host's game; `None` leaves this game its own items.
    pub items: Option<items::GuestItems>,
    // social
    /// Conversations, this player's part of a party save, and the rest vote.
    pub social: social::GuestSocial,
}

impl GuestDriver {
    /// A guest in slot `player` that quotes `hello` to the host.
    pub fn new(player: PlayerId, hello: Hello) -> GuestDriver {
        GuestDriver {
            sync: GuestSync::new(player, hello),
            started: false,
            applied_upto: Rev(0),
            snapshots_applied: 0,
            repairs_applied: 0,
            steps: 0,
            clock_set_ms: None,
            claimed: BTreeSet::new(),
            life_pending: BTreeSet::new(),
            claims_sent: 0,
            heal_every: 20,
            healed: 0,
            relisted: 0,
            healed_notes: Vec::new(),
            healed_seen: BTreeSet::new(),
            ladder: None,
            fight: None, // fights
            heal_refused: 0,
            refused: 0,
            bad_packets: 0,
            send_failures: 0,
            world: None, // world
            items: None, // items
            // social
            social: social::GuestSocial::default(),
        }
    }

    /// One step, between two of the game's frames: the first makes this game a guest (its story
    /// scripts gated) and says hello; each takes the host's messages and applies what changed.
    pub fn step(&mut self, e: &mut Engine, net: &mut dyn Transport, now_ms: u64) {
        self.sync.set_time_ms(now_ms);
        if !self.started {
            gate_guest(e);
            // The guest's own journal: what dies in this game is reported to the host.
            coop_engine::arm(e, true);
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
                .filter_map(|p| Some((p, self.sync.live.sample(p, now_ms)?)))
                .filter(|(_, s)| s.cell.0 != 0)
                .collect();
            if let Some(me) = ladder.frame(e, now_ms, &peers) {
                // Sent by coop-core's guest sync at its fixed rate, in the tick below.
                self.sync.live.local = ladder::state_of(&me);
            }
        }
        // fights
        if let Some(f) = self.fight.as_mut() {
            let avatars = self
                .ladder
                .as_ref()
                .map(ladder::Ladder::avatar_ids)
                .unwrap_or_default();
            for msg in f.guest_step(e, &mut self.sync, &avatars, now_ms) {
                send(net, PlayerId::HOST, &msg, &mut self.send_failures);
            }
        }
        for msg in self.sync.tick(Tick(0)) {
            send(net, PlayerId::HOST, &msg, &mut self.send_failures);
        }
        // social
        social::guest_step(self, e, net, now_ms);
        self.claim_deaths(e, net);
        self.apply(e);
        // world
        if let Some(w) = self.world.as_mut() {
            w.apply(e, &self.sync.store);
        }
        // items
        if let Some(i) = self.items.as_mut() {
            i.apply(e, &self.sync.store, self.sync.player);
        }
        self.keep_clock(e, now_ms);
        self.steps += 1;
        if self.steps % self.heal_every.max(1) == 0 {
            self.heal(e);
            // world
            if let Some(w) = self.world.as_mut() {
                w.heal(e, &self.sync.store);
            }
        }
    }

    /// Tells the host of every placed actor that died in this game and is not dead in the host's
    /// story (a death applied from the host is in the replica by then, so it is not sent back).
    fn claim_deaths(&mut self, e: &mut Engine, net: &mut dyn Transport) {
        let changes = coop_engine::drain(e);
        // items: this player's takes and trades, before the world layer empties the verdicts.
        if let Some(i) = self.items.as_mut() {
            for msg in i.observe(e, &mut self.sync, &changes) {
                send(net, PlayerId::HOST, &msg, &mut self.send_failures);
            }
        }
        // world: this game's own doors and locks, told to the host.
        if let Some(w) = self.world.as_mut() {
            for msg in w.observe(e, &mut self.sync, &changes) {
                send(net, PlayerId::HOST, &msg, &mut self.send_failures);
            }
        }
        self.life_pending.extend(life_changes(&changes));
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
            if actors::dead(e, id) == Some(true) && !known && self.claimed.insert(id) {
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

    /// Sets this game's clock to the host's last one again, about once a second: a clock the host
    /// has stopped (a menu, a wait) does not arrive again, and this game's own would run on.
    fn keep_clock(&mut self, e: &mut Engine, now_ms: u64) {
        if self
            .clock_set_ms
            .is_some_and(|t| now_ms.saturating_sub(t) < CLOCK_EVERY_MS)
        {
            return;
        }
        self.clock_set_ms = Some(now_ms);
        let clock: Vec<(u32, Field)> = self
            .sync
            .store
            .iter()
            .filter(|(_, en)| clock_value(&en.field).is_some())
            .map(|(k, en)| (k.form.0, en.field.clone()))
            .collect();
        for (form, field) in clock {
            apply_field(e, form, &field);
        }
    }

    /// After the connection to the host was lost and the transport joined again as `player`:
    /// resumes from this guest's replica (coop-core's `GuestSync::reconnect`; a host that started
    /// a new session answers with a fresh join and a snapshot, which [`GuestDriver::apply`] then
    /// applies whole).
    pub fn rejoin(&mut self, player: PlayerId, net: &mut dyn Transport) {
        self.sync.player = player;
        for msg in self.sync.reconnect() {
            send(net, PlayerId::HOST, &msg, &mut self.send_failures);
        }
    }

    /// Applies again every replica entry the guest's game does not hold ([`echo`]), and puts back
    /// into the player's objective list any shown objective of a shared quest that is not in it.
    pub fn heal(&mut self, e: &mut Engine) {
        let quests: Vec<u32> = self
            .sync
            .store
            .iter()
            .filter(|(_, en)| {
                matches!(&en.field, Field::Custom { tag, .. } if *tag == record::QUEST_RECORD_TAG)
            })
            .map(|(k, _)| k.form.0)
            .collect();
        for id in quests {
            self.relisted += quest::relist(e, id);
        }
        for m in echo(e, &self.sync.store) {
            if apply_field(e, m.form, &m.want) {
                self.healed += 1;
                let key = (m.form, m.want.kind());
                if self.healed_seen.len() < 40 && self.healed_seen.insert(key) {
                    self.healed_notes.push(format!(
                        "form {:08x}: this game had {:?}, the host's story has {:?}",
                        m.form, m.found, m.want
                    ));
                }
            } else {
                self.heal_refused += 1;
            }
        }
    }

    /// Applies to the game what changed in the replica since the last call; after a snapshot or a
    /// repair replaced part of the replica, everything (their entries may carry revisions at or
    /// below the applied mark: a rejoin, or a host that started a new session).
    pub fn apply(&mut self, e: &mut Engine) {
        let all = self.sync.snapshots_loaded != self.snapshots_applied
            || self.sync.repairs_applied != self.repairs_applied;
        let due: Vec<(u32, Field)> = self
            .sync
            .store
            .iter()
            .filter(|(_, en)| all || en.rev > self.applied_upto)
            .map(|(k, en)| (k.form.0, en.field.clone()))
            .collect();
        for (form, field) in due {
            if !apply_field(e, form, &field) {
                self.refused += 1;
            }
        }
        self.applied_upto = self.sync.store.rev();
        self.snapshots_applied = self.sync.snapshots_loaded;
        self.repairs_applied = self.sync.repairs_applied;
    }
}

/// Applies one replicated field to the guest's game; false when the game refuses it. Fields this
/// driver does not carry yet are left alone.
fn apply_field(e: &mut Engine, form: u32, field: &Field) -> bool {
    match field {
        Field::Custom { tag, .. } if *tag == record::QUEST_RECORD_TAG => {
            match record::decode(form, field) {
                Some(r) => quest::apply(e, &r),
                None => false,
            }
        }
        Field::Custom { tag, .. } if *tag == CLOCK_TAG => match clock_value(field) {
            Some(v) => gate::applying(e, |e| forms::set_global_value(e, form, v)),
            None => false,
        },
        // A death: this game's copy is killed through the game, its scripts held back (they ran on
        // the host). An actor that is not here now is killed when it is (the heal). An actor alive
        // again on the host is left as it is here: nothing here can raise it yet.
        Field::Custom { tag, .. } if *tag == LIFE_TAG => match life_value(field) {
            Some(true) => {
                if actors::dead(e, form) == Some(false) {
                    gate::applying(e, |e| actors::kill(e, form));
                }
                true
            }
            Some(false) => true,
            None => false,
        },
        Field::Global { bits } => gate::applying(e, |e| {
            forms::set_global_value(e, form, f32::from_bits(*bits))
        }),
        Field::ScriptVar { index, bits } => gate::applying(e, |e| {
            vars::set_quest_var(e, form, u32::from(*index), *bits)
        }),
        _ => true,
    }
}

/// A replica entry the guest's game does not hold.
#[derive(Debug, Clone, PartialEq)]
pub struct Mismatch {
    /// The form.
    pub form: u32,
    /// The replica's field.
    pub want: Field,
    /// What the game holds, as the same kind of field (`None`: no such form, or not of that kind).
    pub found: Option<Field>,
}

/// Reads the guest's game back for every quest record, quest variable and global in `store` and
/// lists where the game differs from the replica.
pub fn echo(e: &mut Engine, store: &StateStore) -> Vec<Mismatch> {
    let mut out = Vec::new();
    for (k, en) in store.iter() {
        let form = k.form.0;
        let found = match &en.field {
            Field::Custom { tag, .. } if *tag == record::QUEST_RECORD_TAG => {
                quest::record(e, form).and_then(|r| record::encode(&r))
            }
            Field::ScriptVar { index, .. } => vars::quest_vars(e, form).and_then(|vs| {
                vs.into_iter()
                    .find(|(v, _)| *v == u32::from(*index))
                    .map(|(_, bits)| Field::ScriptVar {
                        index: *index,
                        bits,
                    })
            }),
            Field::Global { .. } => {
                forms::global_value(e, form).map(|v| Field::Global { bits: v.to_bits() })
            }
            // Only a death this game does not have yet is a difference: an actor that is not
            // here now waits, and one alive again on the host is left.
            Field::Custom { tag, .. } if *tag == LIFE_TAG => {
                if life_value(&en.field) == Some(true) && actors::dead(e, form) == Some(false) {
                    Some(life_field(false))
                } else {
                    continue;
                }
            }
            _ => continue,
        };
        if found.as_ref() != Some(&en.field) {
            out.push(Mismatch {
                form,
                want: en.field.clone(),
                found,
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_hello_is_sane_and_names_the_plugins_in_order() {
        let a = hello(vec!["FalloutNV.esm".into(), "DeadMoney.esm".into()]);
        let b = hello(vec!["DeadMoney.esm".into(), "FalloutNV.esm".into()]);
        assert!(compat::hello_is_sane(&a));
        assert_ne!(a.mods_hash, b.mods_hash);
        assert_eq!(a.build_hash, hello(vec![]).build_hash);
    }
}

// The unit tests' own program: no other crate's functions are linked in.
#[cfg(test)]
engine_core::assemble!();

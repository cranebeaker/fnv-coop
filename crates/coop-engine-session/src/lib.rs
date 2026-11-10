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

pub mod record;

use std::collections::BTreeSet;

use coop_core::compat;
use coop_core::delta::StateStore;
use coop_core::ids::{FormId, PlayerId, Rev, Tick};
use coop_core::proto::{self, Hello, Msg};
use coop_core::schema::Field;
use coop_core::sync::GuestSync;
use coop_core::sync::{HostConfig, HostSync};
use coop_core::transport::Transport;
use coop_engine::forms::{self, GlobalWatch};
use coop_engine::{addr, gate, quest, vars};
use engine_core::Engine;

/// What this build quotes at the handshake for a game whose plugins load in `plugins` order: the
/// protocol, this crate's version as the build, the story channel's encoding as the rules, the
/// plugin list, the game version the engine runs, and the clock keys. Two games join only when
/// these agree (coop-core's `compat::decide`), so a guest never applies records made from other
/// data.
pub fn hello(plugins: Vec<String>) -> Hello {
    let build = format!("coop-engine-session {}", env!("CARGO_PKG_VERSION"));
    let rules = format!(
        "quest-record {:04x} v{} {}",
        record::QUEST_RECORD_TAG,
        record::VERSION,
        record::LEN
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
    quests: Vec<u32>,
    globals: GlobalWatch,
    /// Quest records or variables that could not be carried exactly (see [`record::encode`]; a
    /// variable id above 65535): not sent.
    pub unencodable: u32,
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
}

impl HostDriver {
    /// A host with coop-core's config `cfg`, replicating the quests `quest_ids` (all of them are
    /// written at the start, so a guest gets every quest's state from the snapshot, whatever save
    /// either side loaded) and the globals `global_ids` (the game's globals do not pass through the
    /// change journal, so they are compared each step; leave out the ones that change by
    /// themselves, the game clock's).
    pub fn new(
        cfg: HostConfig,
        quest_ids: impl IntoIterator<Item = u32>,
        global_ids: impl IntoIterator<Item = u32>,
    ) -> HostDriver {
        HostDriver {
            sync: HostSync::new(cfg),
            quests: quest_ids.into_iter().collect(),
            globals: GlobalWatch::new(global_ids),
            unencodable: 0,
            unencodable_notes: Vec::new(),
            noted: BTreeSet::new(),
            steps: 0,
            hash_every: 10,
            bad_packets: 0,
            send_failures: 0,
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
        let quests: BTreeSet<u32> = coop_engine::drain(e)
            .into_iter()
            .filter(|c| c.kind == addr::FORM_TYPE_QUEST)
            .map(|c| c.form)
            .collect();
        for id in quests {
            self.write_quest(e, id);
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
            for (to, reply) in self.sync.on_msg(from, msg) {
                send(net, to, &reply, &mut self.send_failures);
            }
        }
        self.collect(e);
        let mut out = self.sync.tick();
        self.steps += 1;
        if self.steps % self.hash_every.max(1) == 0 {
            out.extend(self.sync.hash_broadcast());
        }
        for (to, msg) in out {
            send(net, to, &msg, &mut self.send_failures);
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
    /// Every this many steps the guest's game is read back ([`echo`]) and whatever drifted from the
    /// replica is applied again (a guest's own script writing a story global, a rejoin's snapshot
    /// whose revisions do not pass the applied mark). Replica hashes compare replicas only; this
    /// keeps the game itself on the host's state.
    pub heal_every: u64,
    /// Entries applied again because the game had drifted from the replica.
    pub healed: u32,
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
            heal_every: 20,
            healed: 0,
            heal_refused: 0,
            refused: 0,
            bad_packets: 0,
            send_failures: 0,
        }
    }

    /// One step, between two of the game's frames: the first makes this game a guest (its story
    /// scripts gated) and says hello; each takes the host's messages and applies what changed.
    pub fn step(&mut self, e: &mut Engine, net: &mut dyn Transport, now_ms: u64) {
        self.sync.set_time_ms(now_ms);
        if !self.started {
            gate_guest(e);
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
        for msg in self.sync.tick(Tick(0)) {
            send(net, PlayerId::HOST, &msg, &mut self.send_failures);
        }
        self.apply(e);
        self.steps += 1;
        if self.steps % self.heal_every.max(1) == 0 {
            self.heal(e);
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

    /// Applies again every replica entry the guest's game does not hold ([`echo`]).
    pub fn heal(&mut self, e: &mut Engine) {
        for m in echo(e, &self.sync.store) {
            if apply_field(e, m.form, &m.want) {
                self.healed += 1;
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

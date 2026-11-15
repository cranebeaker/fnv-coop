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

// companions: a companion follows the player who hired it (docs/coop/SPECS.md S33).
pub mod companion;
// fights
pub mod fight;
pub mod ladder;
// look: the other players' avatars as those players (docs/coop/SPECS.md S35).
pub mod look;
// anim: every other animation of the other players' avatars (docs/coop/SPECS.md S41).
pub mod anim;
pub mod record;
// items: loot taken once and barter through the host (docs/coop/SPECS.md S20).
pub mod items;
// world
pub mod world;

// social: conversations, party saves and the rest vote (docs/coop/SPECS.md S16 to S19).
pub mod party;
pub mod rest;
// character: a character in a party part (docs/coop/SPECS.md S32).
pub mod sheet;
pub mod social;
pub mod talk;
pub mod track;

// vats: V.A.T.S. with several players (docs/coop/SPECS.md S21); held actors moved in steps
// (S22).
pub mod smooth;
pub mod vats;

// voice: voice chat's frames and places (docs/coop/SPECS.md S32).
pub mod voice;
// voiceui: every player's name, who is speaking, the players list (SPECS.md, voice list).
pub mod roster;
pub mod voicelist;
// voiceui: end

use std::collections::{BTreeMap, BTreeSet};

use coop_core::avatar::{AvatarMsg, AvatarSender, Sample};
use coop_core::compat;
use coop_core::delta::StateStore;
use coop_core::ids::{Epoch, FormId, PlayerId, Rev, Tick};
use coop_core::proto::{self, Hello, Msg};
use coop_core::rules::{Gift, Notice};
use coop_core::schema::Field;
use coop_core::sync::{GuestLive, HostConfig, HostSync};
use coop_core::sync::{GuestPhase, GuestSync};
use coop_core::transport::Transport;
use coop_engine::forms::{self, GlobalWatch};
use coop_engine::{actors, addr, commands, gate, quest, vars};
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

/// The tag of a host command to an actor, "evaluate your package again" (`Field::Custom`, one
/// byte: a count, so each new command is a change). A stage's result script says `evp` to make an
/// actor act on the new stage at once; it runs on the host only. Not state: [`echo`] and the heal
/// leave it alone.
pub const EVALUATE_TAG: u16 = 0x4556;

/// The tag of a host command to an actor, "start this conversation" (`Field::Custom` on the
/// actor's id: a count, then the scene). A script's `StartConversation` between two actors runs
/// in the host's game only; a guest's game drops its own and plays the host's. Not state:
/// [`echo`] and the heal leave it alone, and one that comes with a join or a repair is not
/// played (it is over).
pub const SCENE_TAG: u16 = 0x5343;
/// A scene a guest's game asks for that played on the host less than this long ago (ms) is not
/// played again: the guest came later to what the host had set off.
pub const SCENE_AGAIN_MS: u64 = 10 * 60_000;
/// The tag of a line an actor was made to say to the player (`SayTo`), on the speaker's form:
/// a count, then the topic. The guest whose player the speaker is near says it there.
pub const SAY_TAG: u16 = 0x5359;
/// How far from the host's player a speaker must be (or in another space, or not loaded) for
/// its line to be said in the guests' games too.
pub const SAY_FAR: f32 = 1500.0;
/// How near a guest's player a speaker must be for that guest's game to say the line.
pub const SAY_NEAR: f32 = 2500.0;

fn say_of(field: &Field) -> Option<(u8, u32)> {
    match field {
        Field::Custom { tag, data } if *tag == SAY_TAG && data.len() == 5 => Some((
            data[0],
            u32::from_le_bytes([data[1], data[2], data[3], data[4]]),
        )),
        _ => None,
    }
}

/// The tag of the package the host's copy of an actor runs (`Field::Custom` on the actor's id:
/// the package's form id, 0 for none both games have). A guest's copy is given the same package
/// in place of what its own game's conditions would pick. [`echo`] and the heal leave it alone
/// (the fights' follow step keeps it).
pub const PACKAGE_TAG: u16 = 0x504b;

/// The field for an actor's package.
/// Host: every quest's variables are read again once in this many collect passes (a variable set
/// by a script leaves nothing in the journal).
pub const VAR_SWEEP_EVERY: u32 = 8;

/// Host: how long the actors of a scene this game plays have their lines run here (ms).
pub const SCENE_ACTORS_MS: u64 = 120_000;

pub fn package_field(package: u32) -> Field {
    Field::Custom {
        tag: PACKAGE_TAG,
        data: package.to_le_bytes().to_vec(),
    }
}

fn package_of(field: &Field) -> Option<u32> {
    match field {
        Field::Custom { tag, data } if *tag == PACKAGE_TAG => {
            Some(u32::from_le_bytes(data[..].try_into().ok()?))
        }
        _ => None,
    }
}

fn scene_field(count: u8, s: &commands::Scene) -> Field {
    let mut data = vec![count, u8::from(s.flags[0]) | (u8::from(s.flags[1]) << 1)];
    for w in [s.target, s.topic, s.locations[0], s.locations[1], s.space] {
        data.extend_from_slice(&w.to_le_bytes());
    }
    for v in s.actor_pos.iter().chain(&s.target_pos) {
        data.extend_from_slice(&v.to_bits().to_le_bytes());
    }
    Field::Custom {
        tag: SCENE_TAG,
        data,
    }
}

fn scene_of(actor: u32, field: &Field) -> Option<commands::Scene> {
    let Field::Custom { tag, data } = field else {
        return None;
    };
    if *tag != SCENE_TAG || data.len() != 2 + 4 * 11 {
        return None;
    }
    let word = |i: usize| u32::from_le_bytes(data[2 + 4 * i..6 + 4 * i].try_into().unwrap());
    let real = |i: usize| f32::from_bits(word(i));
    Some(commands::Scene {
        actor,
        target: word(0),
        topic: word(1),
        locations: [word(2), word(3)],
        flags: [data[1] & 1 != 0, data[1] & 2 != 0],
        space: word(4),
        actor_pos: [real(5), real(6), real(7)],
        target_pos: [real(8), real(9), real(10)],
    })
}

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
        "quest-record {:04x} v{} {} clock {:04x} life {:04x} refs evp scene package",
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
/// A reward as the rules carry it (S30).
fn gift_of(what: coop_engine::reward::What) -> Gift {
    match what {
        coop_engine::reward::What::Xp(xp) => Gift::Xp(xp),
        coop_engine::reward::What::Item { form, count } => Gift::Item { form, count },
    }
}

/// A reward as the game gives it (S30).
fn what_of(gift: Gift) -> coop_engine::reward::What {
    match gift {
        Gift::Xp(xp) => coop_engine::reward::What::Xp(xp),
        Gift::Item { form, count } => coop_engine::reward::What::Item { form, count },
    }
}

/// A reward in the log's words.
fn describe(gift: Gift) -> String {
    match gift {
        Gift::Xp(xp) => format!("{xp} experience"),
        Gift::Item { form, count } => format!("{count} of item {form:08x}"),
    }
}

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
    /// Host: collect passes so far (the variable sweep's clock).
    collects: u32,
    /// Host: the game's loads seen so far ([`coop_engine::saves::loads`]).
    loads_seen: u64,
    /// Host: [`quest::guest_completions_kept`] at the last step.
    kept_seen: u64,
    /// Host: the actors of the scenes started since the last step.
    scene_started: Vec<u32>,
    /// Host: the actors of the scenes this game plays, with when each began.
    scene_actors: BTreeMap<u32, u64>,
    /// Host: each scene played here (actor, target, topic), with when it last began.
    scenes_played: BTreeMap<(u32, u32, u32), u64>,
    /// Host: scenes guests' games asked for that were not played (they had just played, or an
    /// actor was talking).
    pub scenes_refused: u32,
    /// Host: the time of this step.
    now_ms: u64,
    globals: GlobalWatch,
    /// The game clock's globals (hour, day, month, year, days passed): sent as the clock, not as
    /// story globals.
    clock: Vec<u32>,
    clock_sent_ms: Option<u64>,
    /// How many times each actor was told by a script to evaluate its package.
    evaluations: std::collections::BTreeMap<u32, u8>,
    /// How many conversations with another actor a script started for each actor.
    scenes: std::collections::BTreeMap<u32, u8>,
    /// The count of the lines forwarded of each speaker.
    says: std::collections::BTreeMap<u32, u8>,
    /// Stage changes of shared quests, for the log (taken by the caller).
    pub stage_notes: Vec<String>,
    /// Host: the story's rewards this game's player got and the guests were sent (S30), each
    /// one's number for the rules' ledger.
    pub rewards: u32,
    /// Guests' death claims the host's game enacted.
    pub claims_enacted: u32,
    /// Guests' claims left alone: the actor is not in the host's game now, or is dead already.
    pub claims_dropped: u32,
    /// Triggers this game's player spent that were not put into the story (each player's own).
    pub triggers_kept: u32,
    /// Guests' proposals this game did for everyone, and those it did not.
    pub proposals_enacted: u32,
    /// (see `proposals_enacted`)
    pub proposals_dropped: u32,
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
    /// Whether conversations, party saves and the rest vote are shared (on unless the caller
    /// turns it off: none of it has run on a PC yet, and it is switched on by itself).
    pub social_on: bool,
    // companions
    /// Who keeps which companion.
    pub companions: companion::HostCompanions,
    // voice
    /// Voice chat's frames and places; `None` leaves the session without voice.
    pub voice: Option<voice::VoiceLink>,
    // voiceui
    /// Every player's name, written for every game ([`roster`]).
    pub roster: roster::HostRoster,
}

impl HostDriver {
    // world
    /// Shares the rest of the world's state: changed forms' records to the guests, doors and
    /// locks both ways (docs/coop/SPECS.md S14).
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
        sync.claim_tags.insert(PROPOSE_TAG);
        sync.claim_tags.insert(SCENE_TAG);
        HostDriver {
            sync,
            quests: quest_ids.into_iter().collect(),
            collects: 0,
            loads_seen: 0,
            kept_seen: 0,
            scene_started: Vec::new(),
            scene_actors: BTreeMap::new(),
            scenes_played: BTreeMap::new(),
            scenes_refused: 0,
            now_ms: 0,
            globals: GlobalWatch::new(global_ids),
            clock: Vec::new(),
            clock_sent_ms: None,
            evaluations: std::collections::BTreeMap::new(),
            scenes: std::collections::BTreeMap::new(),
            says: std::collections::BTreeMap::new(),
            stage_notes: Vec::new(),
            rewards: 0,
            claims_enacted: 0,
            claims_dropped: 0,
            triggers_kept: 0,
            proposals_enacted: 0,
            proposals_dropped: 0,
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
            companions: companion::HostCompanions::default(), // companions
            social_on: true,
            voice: None,                           // voice
            roster: roster::HostRoster::default(), // voiceui
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
            if let Some(p) = Proposal::read(&field) {
                self.enact_proposal(e, from, id, p);
                continue;
            }
            // A scene a guest's own script started: played here, and so for everyone. Not one
            // that played here a little while ago (the guest walked up later to the trigger the
            // host had set it off with: PC run 31, Joe Cobb began his line again as he ran off),
            // nor one with an actor already in a conversation.
            if let Some(scene) = scene_of(id, &field) {
                let key = (scene.actor, scene.target, scene.topic);
                let ago = self
                    .scenes_played
                    .get(&key)
                    .map(|t| self.now_ms.saturating_sub(*t))
                    .filter(|ago| *ago < SCENE_AGAIN_MS);
                let talking = [scene.actor, scene.target]
                    .into_iter()
                    .find(|a| coop_engine::fight::in_dialogue(e, *a));
                if ago.is_some() || talking.is_some() {
                    self.scenes_refused += 1;
                    self.claim_notes.push(format!(
                        "player {}'s game started a scene: actor {:08x} talks to {:08x} (topic {:08x}): not played, {}",
                        from.0,
                        scene.actor,
                        scene.target,
                        scene.topic,
                        match (ago, talking) {
                            (Some(ago), _) => format!("it played here {} s ago", ago / 1000),
                            (None, Some(a)) => format!("actor {a:08x} is in a conversation"),
                            (None, None) => unreachable!(),
                        }
                    ));
                    continue;
                }
                let played = commands::play_scene(e, &scene);
                self.claim_notes.push(format!(
                    "player {}'s game started a scene: actor {:08x} talks to {:08x} (topic {:08x}): {}",
                    from.0,
                    scene.actor,
                    scene.target,
                    scene.topic,
                    match played {
                        Ok(()) => "played here and for everyone".to_string(),
                        Err(why) => format!("not played here: {why:?}"),
                    }
                ));
                if played.is_ok() {
                    self.scenes_played.insert(key, self.now_ms);
                    commands::played_for_a_guest(e, &scene);
                    let n = self.scenes.entry(scene.actor).or_insert(0);
                    *n = n.wrapping_add(1);
                    let field = scene_field(*n, &scene);
                    self.sync.write(FormId(scene.actor), field);
                }
                continue;
            }
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

    /// Does in this game what a guest's script did in that guest's (D5, rung 2), if it still fits:
    /// a variable only from the value the guest's script saw (a count that has moved on since
    /// takes the guest's step instead), a reference only from the state it saw, a stage or an
    /// objective only when this game has not got there. This game's own scripts run for it (a
    /// stage's result), and the story carries the outcome to every guest.
    fn enact_proposal(&mut self, e: &mut Engine, from: PlayerId, id: u32, p: Proposal) {
        // A line a stage's result has an actor say to the player while the guest's doing is done
        // here was said in that guest's game already, where the stage was set (harness,
        // 2026-11-13: Sunny's bark after the sneak, sent back to the guest, was said twice).
        let earlier_says = commands::drain_says(e);
        let done = self.enact_proposal_body(e, id, p);
        let _ = commands::drain_says(e);
        commands::put_back_says(e, earlier_says);
        self.note_enacted(from, id, p, done);
    }

    fn enact_proposal_body(&mut self, e: &mut Engine, id: u32, p: Proposal) -> bool {
        match p {
            Proposal::Var {
                index,
                before,
                after,
            } => {
                let now = vars::quest_vars(e, id)
                    .and_then(|vs| vs.into_iter().find(|(i, _)| *i == u32::from(index)))
                    .map(|(_, bits)| bits);
                let to = match now {
                    Some(now) if now == before => Some(after),
                    Some(now) if now == after => None,
                    Some(now) => {
                        // Both moved it (two bottles shot at once): the guest's step on top.
                        let v =
                            f64::from_bits(now) + f64::from_bits(after) - f64::from_bits(before);
                        v.is_finite().then_some(v.to_bits())
                    }
                    None => None,
                };
                match to {
                    Some(bits) if vars::set_quest_var(e, id, u32::from(index), bits) => {
                        self.sync
                            .write(FormId(id), Field::ScriptVar { index, bits });
                        true
                    }
                    _ => false,
                }
            }
            Proposal::Disabled { before, after } => {
                actors::shareable(id)
                    && !actors::trigger_volume(e, id)
                    && actors::disabled(e, id) == Some(before)
                    && before != after
                    && actors::set_disabled(e, id, after)
                    && {
                        self.sync
                            .write(FormId(id), Field::RefDisabled { disabled: after });
                        true
                    }
            }
            Proposal::Stage(n) => match quest::record(e, id) {
                Some(r) if !r.done.contains(&n) => {
                    let ran = forms::set_quest_stage(e, id, n);
                    self.write_quest(e, id);
                    ran
                }
                _ => false,
            },
            Proposal::Objective { number, state } => match quest::record(e, id) {
                Some(mut r) if r.objectives.iter().any(|o| o.0 == number && o.1 != state) => {
                    for o in r.objectives.iter_mut().filter(|o| o.0 == number) {
                        o.1 = state;
                    }
                    // The guest's doing, as such (an objective it takes back is taken back).
                    let ok = coop_engine::gate::applying(e, |e| quest::apply(e, &r));
                    if ok && state & 2 != 0 {
                        // Not taken back by this game's own scripts, whose `Player` is the
                        // host's player alone (S29).
                        quest::note_guest_completed(e, id, number);
                        self.stage_notes.push(format!(
                            "objective {number} of {id:08x} completed by a guest: kept from this game's own scripts"
                        ));
                    }
                    self.write_quest(e, id);
                    ok
                }
                _ => false,
            },
        }
    }

    fn note_enacted(&mut self, from: PlayerId, id: u32, p: Proposal, done: bool) {
        if done {
            self.proposals_enacted += 1;
        } else {
            self.proposals_dropped += 1;
        }
        if self.proposals_enacted + self.proposals_dropped <= 400 {
            self.claim_notes.push(format!(
                "player {}'s game did {p:?} to {id:08x}: {}",
                from.0,
                if done {
                    "done here for everyone"
                } else {
                    "not done here (it no longer fits)"
                }
            ));
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

    /// After this game loaded a save: the story read again from the game (PC harness, 2026-11-13:
    /// the story was read once at the main menu; after the host loaded its save, Sunny's quest
    /// was at stage 5 in its game and 0 in the story, and the guests were put back to 0). A load
    /// leaves nothing in the change journal, so every quest's record and variables, and the
    /// state of every reference the story holds one of, are written as the game has them now.
    fn reread_after_load(&mut self, e: &mut Engine) {
        for id in self.quests.clone() {
            self.write_quest(e, id);
        }
        let refs: Vec<u32> = self
            .sync
            .store
            .iter()
            .filter(|(_, en)| matches!(en.field, Field::RefDisabled { .. }))
            .map(|(k, _)| k.form.0)
            .collect();
        for id in refs {
            if let Some(disabled) = actors::disabled(e, id) {
                self.sync.write(FormId(id), Field::RefDisabled { disabled });
            }
        }
        // Likewise every actor the story holds the life of (one dead before the load may live).
        let lives: Vec<u32> = self
            .sync
            .store
            .iter()
            .filter(|(_, en)| life_value(&en.field).is_some())
            .map(|(k, _)| k.form.0)
            .collect();
        for id in lives {
            if let Some(dead) = actors::dead(e, id) {
                self.sync.write(FormId(id), life_field(dead));
            }
        }
        self.stage_notes
            .push("this game loaded a save: the story is read again from it".into());
    }

    /// Writes quest `id`'s record and its script's variables into the replica.
    fn write_quest(&mut self, e: &mut Engine, id: u32) {
        if let Some(r) = quest::record(e, id) {
            match record::encode(&r) {
                Some(f) => {
                    let before = self
                        .sync
                        .store
                        .get(FormId(id), &f)
                        .and_then(|en| record::decode(id, &en.field));
                    if before.as_ref().map(|b| (b.stage, b.flags, &b.done))
                        != Some((r.stage, r.flags, &r.done))
                    {
                        self.stage_notes.push(format!(
                            "quest {id:08x}: stage {} flags {:02x} done {:?} (was {})",
                            r.stage,
                            r.flags,
                            r.done,
                            before.map_or("not in the story".to_string(), |b| format!(
                                "stage {} flags {:02x}",
                                b.stage, b.flags
                            ))
                        ));
                    }
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
        // A quest variable a script sets (`set Quest.var to ...`, a dialogue line's result) leaves
        // nothing in the journal: every quest's variables are read again every few frames (PC
        // run 28: the host's Sunny greeted its player and set `bMetSunny`, the story kept 0, the
        // guest's own greeting was refused as already done, and its game was put back to 0, so
        // its Sunny walked up to its player again and again).
        self.collects = self.collects.wrapping_add(1);
        if self.collects % VAR_SWEEP_EVERY == 0 {
            for id in self.quests.clone() {
                for (var, bits) in vars::quest_vars(e, id).unwrap_or_default() {
                    if let Ok(index) = u16::try_from(var) {
                        self.sync
                            .write(FormId(id), Field::ScriptVar { index, bits });
                    }
                }
            }
        }
        // A placed reference a script enabled or disabled (its form flags changed): its state as
        // it is now. Kept in the story from then on, so a later change back is sent too.
        for c in &changes {
            if actors::is_ref_type(c.kind)
                && c.flags & addr::CHANGE_FORM_FLAGS != 0
                && actors::shareable(c.form)
            {
                // A trigger spent by its own script is spent for the host's player only: each
                // guest's is its own player's to walk into.
                if actors::own_trigger_change(e, c) {
                    self.triggers_kept += 1;
                    if self.triggers_kept <= 40 {
                        self.stage_notes.push(format!(
                            "trigger {:08x} was spent by this game's player: not shared (each player has their own)",
                            c.form
                        ));
                    }
                    continue;
                }
                if let Some(disabled) = actors::disabled(e, c.form) {
                    self.sync
                        .write(FormId(c.form), Field::RefDisabled { disabled });
                }
            }
        }
        // An actor a script told to evaluate its package: the guests' copies are told too.
        for id in commands::drain_evaluations(e) {
            let n = self.evaluations.entry(id).or_insert(0);
            *n = n.wrapping_add(1);
            let data = vec![*n];
            self.sync.write(
                FormId(id),
                Field::Custom {
                    tag: EVALUATE_TAG,
                    data,
                },
            );
        }
        // A line a script had an actor say to the host's player far from it (or one a guest's
        // game runs): the guest near it says it there.
        let theirs = self
            .fight
            .as_ref()
            .map(|f| f.theirs().clone())
            .unwrap_or_default();
        for (speaker, topic) in commands::drain_says(e) {
            let far = match (coop_engine::live::local(e), actors::actor(e, speaker)) {
                (Some(me), Some(a)) if me.loaded => {
                    let here = coop_engine::fight::place_of_ptr(e, a);
                    coop_engine::fight::space_of(e, a) != me.cell
                        || e.mem.u32(a.wrapping_add(addr::ACTOR_PROCESS_OFFSET)) == 0
                        || (0..3)
                            .map(|i| (here[i] - me.pos[i]).powi(2))
                            .sum::<f32>()
                            .sqrt()
                            > SAY_FAR
                }
                _ => true,
            };
            if !(far || theirs.contains(&speaker)) {
                continue;
            }
            let n = self.says.entry(speaker).or_insert(0);
            *n = n.wrapping_add(1);
            let mut data = vec![*n];
            data.extend(topic.to_le_bytes());
            self.stage_notes.push(format!(
                "say: actor {speaker:08x} says topic {topic:08x} to the player, away from this game's: the guest near it says it"
            ));
            self.sync
                .write(FormId(speaker), Field::Custom { tag: SAY_TAG, data });
        }
        // A conversation a script started between two actors: the guests' games play it too.
        for scene in commands::drain_scenes(e) {
            self.scenes_played
                .insert((scene.actor, scene.target, scene.topic), self.now_ms);
            self.scene_started.extend([scene.actor, scene.target]);
            let n = self.scenes.entry(scene.actor).or_insert(0);
            *n = n.wrapping_add(1);
            self.stage_notes.push(format!(
                "scene: actor {:08x} starts a conversation with {:08x} (topic {:08x})",
                scene.actor, scene.target, scene.topic
            ));
            let field = scene_field(*n, &scene);
            self.sync.write(FormId(scene.actor), field);
        }
        // A placed actor that died (or, once it has an entry, lives again).
        for id in life_changes(&changes) {
            let Some(dead) = actors::dead(e, id) else {
                continue;
            };
            let field = life_field(dead);
            if dead || self.sync.store.get(FormId(id), &field).is_some() {
                if self.sync.store.get(FormId(id), &field).map(|en| &en.field) != Some(&field) {
                    self.stage_notes.push(format!(
                        "actor {id:08x}: {} in this game",
                        if dead { "died" } else { "alive again" }
                    ));
                }
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
        self.now_ms = now_ms;
        let kept = quest::guest_completions_kept(e);
        // Said the first time, then once in 1000 (a quest script tries again every frame).
        if kept / 1000 != self.kept_seen / 1000 || (self.kept_seen == 0 && kept > 0) {
            self.stage_notes.push(format!(
                "this game's own scripts were kept from taking back a guest's objective ({} times)",
                kept
            ));
            self.kept_seen = kept;
        }
        // A save this game loaded: the story is read again from it.
        let loads = coop_engine::saves::loads(e);
        if loads != self.loads_seen {
            self.loads_seen = loads;
            if self.sync.tick_no().0 != 0 {
                self.reread_after_load(e);
            }
        }
        self.sync.set_time_ms(now_ms);
        for (from, msg) in receive(net, &mut self.bad_packets) {
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
        // companions: who keeps which, and the keepers' claims (before the others are judged).
        let avatars = self
            .ladder
            .as_ref()
            .map(ladder::Ladder::avatar_ids)
            .unwrap_or_default();
        self.companions.step(e, &mut self.sync, &avatars);
        self.enact_claims(e);
        self.collect(e);
        // The story's rewards (S30): this game's player has each; every guest gets the same.
        for given in coop_engine::reward::take(e) {
            self.rewards += 1;
            let gift = gift_of(given.what);
            let outs = self.sync.live.reward(gift);
            self.stage_notes.push(format!(
                "the story gave this game's player {} ({:08x}'s script): to {} guest(s) too",
                describe(gift),
                given.quest,
                outs.len()
            ));
            for (to, msg) in outs {
                send(net, to, &msg, &mut self.send_failures);
            }
        }
        // Rewards a guest's game gave its player from the story (a line an NPC said there outside
        // the conversation): this game's player gets the same; the rules sent the other guests.
        for gift in std::mem::take(&mut self.sync.live.rules.gifts_here) {
            if coop_engine::reward::give(e, what_of(gift)) {
                self.stage_notes.push(format!(
                    "a guest's game gave its player {} from the story: this game's player too",
                    describe(gift)
                ));
            }
        }
        // The actors of a scene this game plays: their lines run here even when a guest's game
        // runs them (the scene is the host's, and its results the story's), for SCENE_ACTORS_MS.
        for id in std::mem::take(&mut self.scene_started) {
            self.scene_actors.insert(id, now_ms);
        }
        self.scene_actors
            .retain(|_, t| now_ms.saturating_sub(*t) < SCENE_ACTORS_MS);
        coop_engine::gate::set_scene_actors(e, self.scene_actors.keys().copied());
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
        // voiceui: the party's names, before the tick sends the store's changes.
        self.roster.step(&mut self.sync);
        let mut out = self.sync.tick();
        self.steps += 1;
        if self.steps % self.hash_every.max(1) == 0 {
            out.extend(self.sync.hash_broadcast());
        }
        for (to, msg) in out {
            send(net, to, &msg, &mut self.send_failures);
        }
        self.live_step(e, net, now_ms);
        // voice
        voice::host_step(self, e, net, now_ms);
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
        // look: the other players' parts and actions, for their avatars.
        if let Some(l) = ladder.look.as_mut() {
            l.take(&mut self.view);
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
        // look: this player's gait and weapon in its state; its parts and actions to the others.
        if let Some(l) = ladder.look.as_mut() {
            l.stamp(e, &mut state);
            let own = l.own(e, now_ms);
            let mut msgs: Vec<AvatarMsg> = Vec::new();
            if let Some((equipment, appearance)) = own.parts {
                msgs.push(self.sender.set_parts(equipment, appearance));
            }
            for (kind, arg) in own.actions {
                msgs.push(self.sender.action(kind, arg));
            }
            for m in msgs {
                let relayed = self.sync.live.on_msg(
                    PlayerId::HOST,
                    Msg::Avatar(m),
                    now_ms,
                    &mut self.sync.auth,
                );
                for (to, msg) in relayed {
                    send(net, to, &msg, &mut self.send_failures);
                }
            }
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
    /// The script last seen changing each form in this game (the guest's own journal): what a
    /// drift of that form is put down to.
    causes: std::collections::BTreeMap<u32, gate::Cause>,
    /// Triggers this player spent (their own script switched them when the player walked in),
    /// with what the host's story said of each then: left as this game has them.
    own_triggers: std::collections::BTreeMap<u32, Option<bool>>,
    /// The time of the last heal.
    now_ms: u64,
    /// Proposals made and not yet answered: what this game held then, and until when it is left.
    proposed: std::collections::BTreeMap<(u32, u8, u32), (Field, u64)>,
    /// Proposals to send.
    proposals_out: Vec<(FormId, Field)>,
    /// Loads of a save this game made, as last seen.
    loads_seen: u64,
    /// Until when nothing is proposed (after a load, or a burst of differences).
    quiet_until: u64,
    /// What differed from the story at the last heal.
    differed: std::collections::BTreeSet<(u32, u8, u32)>,
    /// Entries of the story this game could not take when they last came.
    unapplied: std::collections::BTreeSet<(u32, u8, u32)>,
    /// The quests each object script of the game's data names (from the data files): a change
    /// to one of them with no cause in the journal is proposed only when such a script ran here
    /// just before.
    pub object_script_quests: std::collections::BTreeMap<u32, Vec<u32>>,
    /// Object scripts naming a quest that ran here lately, and when.
    recent_object_scripts: std::collections::BTreeMap<(u32, u32), u64>,
    /// Whether this game's own object scripts' changes are proposed to the host
    /// (`NV_COOP_PROPOSE=off` turns it off: they are put back as before).
    pub proposals_on: bool,
    /// Proposals sent to the host, and those the host let lapse (put back here).
    pub proposals_sent: u32,
    /// (see `proposals_sent`)
    pub proposals_lapsed: u32,
    /// The count of the last scene seen for each actor, and whether the join's state is in.
    scene_seen: std::collections::BTreeMap<u32, u8>,
    /// The count of the last line of each speaker this game has seen.
    say_seen: std::collections::BTreeMap<u32, u8>,
    /// The host's lines said here.
    pub says_said: u32,
    /// Scenes this game's own scripts started, asked of the host.
    pub scenes_asked: u32,
    joined: bool,
    /// The host's conversations between actors played in this game, and those that could not be.
    pub scenes_played: u32,
    /// See [`GuestDriver::scenes_played`].
    pub scenes_missed: u32,
    /// Distinct drifts whose cause the journal named, and those it did not (docs/coop/DECISIONS.md
    /// D5, rung 1: the census of what a guest's own scripts write to the shared story).
    pub drift_named: u32,
    /// See [`GuestDriver::drift_named`].
    pub drift_unnamed: u32,
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
    /// The experience the story gave this game's player through the host (S30).
    pub xp_given: u64,
    /// The story's item rewards given to this game's player through the host (S30).
    pub items_given: u32,
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
    /// See [`HostDriver::social_on`].
    pub social_on: bool,
    // companions
    /// The companions this player keeps.
    pub companions: companion::GuestCompanions,
    // voice
    /// See [`HostDriver::voice`].
    pub voice: Option<voice::VoiceLink>,
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
            causes: std::collections::BTreeMap::new(),
            own_triggers: std::collections::BTreeMap::new(),
            now_ms: 0,
            proposed: std::collections::BTreeMap::new(),
            proposals_out: Vec::new(),
            loads_seen: 0,
            quiet_until: 0,
            differed: std::collections::BTreeSet::new(),
            unapplied: std::collections::BTreeSet::new(),
            proposals_on: true,
            object_script_quests: std::collections::BTreeMap::new(),
            recent_object_scripts: std::collections::BTreeMap::new(),
            proposals_sent: 0,
            proposals_lapsed: 0,
            scene_seen: std::collections::BTreeMap::new(),
            say_seen: std::collections::BTreeMap::new(),
            says_said: 0,
            scenes_asked: 0,
            joined: false,
            scenes_played: 0,
            scenes_missed: 0,
            drift_named: 0,
            drift_unnamed: 0,
            claims_sent: 0,
            heal_every: 20,
            healed: 0,
            relisted: 0,
            healed_notes: Vec::new(),
            xp_given: 0,
            items_given: 0,
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
            social_on: true,
            companions: companion::GuestCompanions::default(), // companions
            voice: None,                                       // voice
        }
    }

    /// One step, between two of the game's frames: the first makes this game a guest (its story
    /// scripts gated) and says hello; each takes the host's messages and applies what changed.
    /// The story's rewards the host sent (S30): this game's player gets each, the game's own
    /// way. One that cannot be given yet (no player) waits.
    fn take_rewards(&mut self, e: &mut Engine, net: &mut dyn Transport) {
        // The story's rewards this game gave its player (a line an NPC said here outside the
        // conversation): the host's player and the other guests get the same.
        for given in coop_engine::reward::take(e) {
            let gift = gift_of(given.what);
            if let (Some(i), Gift::Item { form, count }) = (self.items.as_mut(), gift) {
                i.story_gave(form, count);
            }
            self.healed_notes.push(format!(
                "co-op guest: the story gave this game's player {} here ({:08x}'s line): sent to the host",
                describe(gift),
                given.quest
            ));
            send(
                net,
                PlayerId::HOST,
                &Msg::Rules(coop_core::rules::RulesMsg::Reward { id: 0, gift }),
                &mut self.send_failures,
            );
        }
        let notices = std::mem::take(&mut self.sync.live.rules.notices);
        for n in notices {
            match n {
                Notice::Reward(gift) if coop_engine::reward::give(e, what_of(gift)) => {
                    match gift {
                        Gift::Xp(xp) => self.xp_given += u64::from(xp),
                        Gift::Item { form, count } => {
                            self.items_given += 1;
                            // items: a quest's reward is not a take (see `story_gave`).
                            if let Some(i) = self.items.as_mut() {
                                i.story_gave(form, count);
                            }
                        }
                    }
                    self.healed_notes.push(format!(
                        "co-op guest: the story gave this game's player {}",
                        describe(gift)
                    ));
                }
                other => self.sync.live.rules.notices.push(other),
            }
        }
    }

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
        self.take_rewards(e, net);
        if let Some(ladder) = self.ladder.as_mut() {
            // look: the other players' parts and actions, for their avatars.
            if let Some(l) = ladder.look.as_mut() {
                l.take(&mut self.sync.live);
            }
            let ids: Vec<PlayerId> = self.sync.live.remote.keys().copied().collect();
            let peers: Vec<(PlayerId, Sample)> = ids
                .into_iter()
                .filter_map(|p| Some((p, self.sync.live.sample(p, now_ms)?)))
                .filter(|(_, s)| s.cell.0 != 0)
                .collect();
            if let Some(me) = ladder.frame(e, now_ms, &peers) {
                // Sent by coop-core's guest sync at its fixed rate, in the tick below.
                self.sync.live.local = ladder::state_of(&me);
                // look: this player's gait and weapon; its parts and actions to the host, once
                // it takes them (the host drops what a guest sends before it is in).
                if let Some(l) = ladder.look.as_mut() {
                    l.stamp(e, &mut self.sync.live.local);
                }
                let active = self.sync.phase() == GuestPhase::Active;
                if let Some(l) = ladder.look.as_mut().filter(|_| active) {
                    let own = l.own(e, now_ms);
                    if let Some((equipment, appearance)) = own.parts {
                        let m = self.sync.live.sender.set_parts(equipment, appearance);
                        send(
                            net,
                            PlayerId::HOST,
                            &Msg::Avatar(m),
                            &mut self.send_failures,
                        );
                    }
                    for (kind, arg) in own.actions {
                        let m = self.sync.live.action(kind, arg);
                        send(net, PlayerId::HOST, &m, &mut self.send_failures);
                    }
                }
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
        // companions: this player's own, brought along; the wheel's switches to the host.
        let avatars = self
            .ladder
            .as_ref()
            .map(ladder::Ladder::avatar_ids)
            .unwrap_or_default();
        let claims = self.companions.step(e, &self.sync, &avatars, now_ms);
        for batch in claims.chunks(proto::MAX_INTENT_CHANGES) {
            for msg in self.sync.send_intent(Epoch(0), batch.to_vec()) {
                send(net, PlayerId::HOST, &msg, &mut self.send_failures);
            }
        }
        // voice
        voice::guest_step(self, e, net, now_ms);
        self.now_ms = now_ms;
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
        for (script, reference) in gate::drain_object_scripts(e) {
            if self.object_script_quests.contains_key(&script) {
                self.recent_object_scripts
                    .insert((script, reference), now_ms);
            }
        }
        self.recent_object_scripts
            .retain(|_, at| now_ms.saturating_sub(*at) <= PROPOSE_RECENT_MS);
        if self.proposals_on && self.sync.phase() == GuestPhase::Active {
            for scene in commands::drain_scenes(e) {
                self.proposals_out
                    .push((FormId(scene.actor), scene_field(0, &scene)));
                self.scenes_asked += 1;
            }
        } else {
            let _ = commands::drain_scenes(e);
        }
        let claims = std::mem::take(&mut self.proposals_out);
        if !claims.is_empty() {
            for msg in self.sync.send_intent(Epoch(0), claims) {
                send(net, PlayerId::HOST, &msg, &mut self.send_failures);
            }
        }
        self.steps += 1;
        if self.steps % self.heal_every.max(1) == 0 {
            self.now_ms = now_ms;
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
        self.note_loads(e);
        let changes = coop_engine::drain(e);
        // items: this player's takes and trades, before the world layer empties the verdicts.
        if let Some(i) = self.items.as_mut() {
            for msg in i.observe(e, &mut self.sync, &changes) {
                send(net, PlayerId::HOST, &msg, &mut self.send_failures);
            }
        }
        for c in &changes {
            if let Some(cause) = c.cause {
                self.causes.insert(c.form, cause);
            }
            // A trigger this player walked into and its own script spent: this player's own from
            // now, whatever the host's story says of it today.
            // A reference the story says nothing of yet, switched by one of this game's own
            // object scripts (a target shot, a thing used up): proposed straight from the
            // journal, since there is nothing in the story to compare it with.
            if actors::is_ref_type(c.kind)
                && c.flags & addr::CHANGE_FORM_FLAGS != 0
                && actors::shareable(c.form)
                && c.cause
                    .is_some_and(|k| k.script != 0 && !k.quest_script && k.owner_quest == 0)
                && self.proposals_on
                && self.now_ms >= self.quiet_until
                && self.story_disabled(c.form).is_none()
                && !actors::trigger_volume(e, c.form)
                && self.sync.phase() == GuestPhase::Active
            {
                if let Some(now) = actors::disabled(e, c.form) {
                    let p = Proposal::Disabled {
                        before: !now,
                        after: now,
                    };
                    self.proposals_out.push((FormId(c.form), p.field()));
                    self.proposals_sent += 1;
                }
            }
            if actors::own_trigger_change(e, c) {
                let story = self.story_disabled(c.form);
                if self.own_triggers.insert(c.form, story).is_none() {
                    self.healed_notes.push(format!(
                        "co-op guest: trigger {:08x} was spent by this player: kept as this game has it",
                        c.form
                    ));
                }
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

    /// What the host's story says of reference `form` being disabled, if it says anything.
    fn story_disabled(&self, form: u32) -> Option<bool> {
        self.sync
            .store
            .get(FormId(form), &Field::RefDisabled { disabled: false })
            .and_then(|en| match en.field {
                Field::RefDisabled { disabled } => Some(disabled),
                _ => None,
            })
    }

    /// Whether `field` of `form` is left as this game has it: the state of a trigger this player
    /// spent, for as long as the host's story says of it what it said then. A story that says
    /// something new (a quest of the host's switched the trigger) is the story's again.
    fn own_trigger_keeps(&mut self, form: u32, field: &Field) -> bool {
        let Field::RefDisabled { disabled } = field else {
            return false;
        };
        match self.own_triggers.get(&form) {
            Some(then) if *then == Some(*disabled) => true,
            Some(_) => {
                self.own_triggers.remove(&form);
                false
            }
            None => false,
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

    /// Forgets what the journal said caused each change so far (tests: a change that comes later
    /// with no cause of its own).
    pub fn forget_causes(&mut self) {
        self.causes.clear();
    }

    /// Notes a load of a save in this game: nothing is proposed for a while.
    fn note_loads(&mut self, e: &mut Engine) {
        let loads = coop_engine::saves::loads(e);
        if loads != self.loads_seen {
            self.loads_seen = loads;
            self.quiet_until = self.now_ms + PROPOSE_QUIET_MS;
            self.proposed.clear();
            self.causes.clear();
            self.healed_notes.push(
                "co-op guest: this game loaded a save: put back to the story, nothing proposed for a while"
                    .into(),
            );
        }
    }

    /// Whether a difference from the story is left alone for now: it is what one of this game's
    /// own object scripts did (not a quest's script, which the gate holds; not a line of dialogue,
    /// which the conversations carry), it has been proposed to the host, and the host has had
    /// less than [`PROPOSE_HOLD_MS`] to answer. The first time, the proposal is queued. When the
    /// time is up the difference is put back like any other.
    fn proposes(&mut self, e: &mut Engine, m: &Mismatch) -> bool {
        let Some(found) = &m.found else {
            return false;
        };
        let key = (m.form, m.want.kind(), sub_of(&m.want));
        if let Some((held, until)) = self.proposed.get(&key) {
            if held == found && self.now_ms < *until {
                return true;
            }
            if held == found {
                // The host did not take it: put back, and not proposed again from this cause.
                self.proposed.remove(&key);
                self.causes.remove(&m.form);
                self.proposals_lapsed += 1;
                return false;
            }
            self.proposed.remove(&key);
        }
        // This game changed it, not the story: the story's entry went into this game when it
        // last came (an entry this game could not take is this game being behind, and is never
        // proposed), and this game agreed with the story at the last heal.
        let stable = self.joined && !self.unapplied.contains(&key);
        let fresh = !self.differed.contains(&key);
        let cause = self.causes.get(&m.form);
        // A quest's variable or stage changed with no cause in the journal (`set Quest.var to`
        // leaves none) is this player's doing only if an object script that names that quest
        // ran in this game just before (PC run 19: quests of the pre-order packs changed here on
        // their own, every few seconds, and were proposed).
        // Which of them ran on an actor that has died here since: what such a script does to the
        // quest is its death's doing, and the host runs the death of its own copy (PC run 25:
        // the third well's geckos were counted in both games, the count passed 3 without being
        // 3, the well was never cleared, and the settler and Sunny waited for ever).
        let naming: Vec<u32> = self
            .recent_object_scripts
            .keys()
            .filter(|(script, _)| {
                self.object_script_quests
                    .get(script)
                    .is_some_and(|qs| qs.contains(&m.form))
            })
            .map(|(_, reference)| *reference)
            .collect();
        let a_death = naming.iter().any(|r| actors::dead(e, *r) == Some(true));
        let named_by_a_script = !naming.is_empty() && !a_death;
        // What this game's own doing is: an object script's, or a line of this player's own
        // dialogue whose result ran here (PC run 20: Sunny's rifle line completed an objective
        // and showed the next in the guest's game; put back, the guest got no instructions). A
        // quest's script is the host's (the gate holds those). Everything proposed is done by
        // the host only if it still fits, so a result the host also runs is not done twice.
        let cause_dead = match cause {
            Some(c) => actors::dead(e, c.reference) == Some(true),
            None => false,
        };
        // (a script on an actor that died here: its death, which the host runs for its own copy)
        let own_doing = |c: &gate::Cause| !c.quest_script && c.owner_quest == 0 && !cause_dead;
        let allowed = match &m.want {
            // A reference: only when this game's own doing is known to have done it (lights and
            // the like differ between two games for reasons of their own).
            Field::RefDisabled { .. } => cause.is_some_and(own_doing),
            // A quest's variable or record: a script that names the quest ran here just before,
            // or the journal's last word on the quest is this game's own doing. (A variable leaves
            // nothing in the journal, so the journal's word may be older than the change: PC run
            // 20, the bottles' count was put down to the rifle line.)
            Field::ScriptVar { .. } => {
                !a_death && (named_by_a_script || cause.is_some_and(own_doing))
            }
            Field::Custom { tag, .. } if *tag == record::QUEST_RECORD_TAG => {
                !a_death && (named_by_a_script || cause.is_some_and(own_doing))
            }
            _ => false,
        };
        if !(stable && fresh && allowed)
            || !self.proposals_on
            || self.sync.phase() != GuestPhase::Active
            || self.now_ms < self.quiet_until
        {
            return false;
        }
        let ps = Proposal::between(m.form, &m.want, found);
        if ps.is_empty() {
            return false;
        }
        for p in &ps {
            self.proposals_out.push((FormId(m.form), p.field()));
        }
        self.proposals_sent += ps.len() as u32;
        if self.proposals_sent <= 400 {
            self.healed_notes.push(format!(
                "co-op guest: this game's script {:08x} did {ps:?} to {:08x}: proposed to the host",
                self.causes.get(&m.form).map_or(0, |c| c.script),
                m.form
            ));
        }
        // The cause has been used: a later difference on this form needs a cause of its own.
        self.causes.remove(&m.form);
        self.proposed
            .insert(key, (found.clone(), self.now_ms + PROPOSE_HOLD_MS));
        true
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
        self.note_loads(e);
        let mismatches = echo(e, &self.sync.store);
        // A game that loaded a save, or that differs from the story in many things at once, has
        // not been played into it: everything is put back, nothing proposed, for a while (PC run
        // 25: the guest's player died, the game loaded its last save, and the guest proposed the
        // whole of that save to the host: the tutorial's quests went back, Sunny died).
        let proposable = mismatches
            .iter()
            .filter(|m| {
                !self
                    .differed
                    .contains(&(m.form, m.want.kind(), sub_of(&m.want)))
            })
            .count();
        if proposable > PROPOSE_BURST && self.now_ms >= self.quiet_until {
            self.quiet_until = self.now_ms + PROPOSE_QUIET_MS;
            self.healed_notes.push(format!(
                "co-op guest: {proposable} things differ from the story at once: put back, nothing proposed for a while"
            ));
        }
        let differ: std::collections::BTreeSet<(u32, u8, u32)> = mismatches
            .iter()
            .map(|m| (m.form, m.want.kind(), sub_of(&m.want)))
            .collect();
        for m in mismatches {
            if self.own_trigger_keeps(m.form, &m.want) {
                continue;
            }
            if self.proposes(e, &m) {
                continue;
            }
            if apply_field(e, m.form, &m.want) {
                self.healed += 1;
                let key = (m.form, m.want.kind());
                if self.healed_seen.len() < 200 && self.healed_seen.insert(key) {
                    let cause = match self.causes.get(&m.form) {
                        Some(c) => {
                            self.drift_named += 1;
                            format!("changed here by {c}")
                        }
                        None => {
                            self.drift_unnamed += 1;
                            "cause unknown".into()
                        }
                    };
                    self.healed_notes.push(format!(
                        "COOP-DRIFT form {:08x} kind {}: {cause}; this game had {:?}, the host's story has {:?}; put back",
                        m.form, key.1, m.found, m.want
                    ));
                }
            } else {
                self.heal_refused += 1;
            }
        }
        self.differed = differ;
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
            if let Some((count, topic)) = say_of(&field) {
                let seen = self.say_seen.insert(form, count);
                if !self.joined || seen == Some(count) {
                    continue;
                }
                let near = match (coop_engine::live::local(e), actors::actor(e, form)) {
                    (Some(me), Some(a)) if me.loaded => {
                        let here = coop_engine::fight::place_of_ptr(e, a);
                        coop_engine::fight::space_of(e, a) == me.cell
                            && (0..3)
                                .map(|i| (here[i] - me.pos[i]).powi(2))
                                .sum::<f32>()
                                .sqrt()
                                <= SAY_NEAR
                    }
                    _ => false,
                };
                if !near {
                    continue;
                }
                match commands::say_to_player(e, form, topic) {
                    Ok(()) => {
                        self.says_said += 1;
                        self.healed_notes.push(format!(
                            "co-op guest: the host's line, said here: actor {form:08x} says topic {topic:08x} to this player"
                        ));
                    }
                    Err(why) => self.healed_notes.push(format!(
                        "co-op guest: the host's line of actor {form:08x} (topic {topic:08x}) was not said here: {why:?}"
                    )),
                }
                continue;
            }
            if let Some(scene) = scene_of(form, &field) {
                // One that comes with the join happened before: it is over. After that, one whose
                // count this game has not played is new, whether it comes alone or with a repair.
                let count = match &field {
                    Field::Custom { data, .. } => data.first().copied().unwrap_or(0),
                    _ => 0,
                };
                let seen = self.scene_seen.insert(form, count);
                if !self.joined || seen == Some(count) {
                    continue;
                }
                match gate::applying(e, |e| commands::play_scene(e, &scene)) {
                    Ok(()) => {
                        self.scenes_played += 1;
                        self.healed_notes.push(format!(
                            "co-op guest: the host's scene, asked of this game: actor {:08x} talks to {:08x} (topic {:08x})",
                            scene.actor, scene.target, scene.topic
                        ));
                    }
                    Err(why) => {
                        self.scenes_missed += 1;
                        self.healed_notes.push(format!(
                            "co-op guest: the host's scene of actor {:08x} with {:08x} was not played here: {why:?}",
                            scene.actor, scene.target
                        ));
                    }
                }
                continue;
            }
            if self.own_trigger_keeps(form, &field) {
                continue;
            }
            let key = (form, field.kind(), sub_of(&field));
            if apply_field(e, form, &field) {
                self.unapplied.remove(&key);
            } else {
                self.refused += 1;
                self.unapplied.insert(key);
            }
        }
        if all {
            self.joined = true;
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
        // The package the host's copy runs: remembered for the game's own evaluation to be
        // answered with, and the actor told to evaluate.
        Field::Custom { tag, .. } if *tag == PACKAGE_TAG => match package_of(field) {
            // Remembered only: the fights' follow step asks the copy to take it, when this
            // player's own opening is done and the host runs the actor near this player.
            Some(package) => {
                commands::set_wanted_package(e, form, package);
                true
            }
            None => false,
        },
        // A host's scene: played by the driver when it comes, never from here.
        Field::Custom { tag, .. } if *tag == SCENE_TAG || *tag == SAY_TAG => true,
        // A host command: the actor evaluates its package again, as the host's script told its
        // own copy. One that is not in this game now picks its package when it loads anyway.
        Field::Custom { tag, .. } if *tag == EVALUATE_TAG => {
            gate::applying(e, |e| commands::evaluate_package(e, form));
            true
        }
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
        // A reference the host's scripts enabled or disabled: the same here, through the game. One
        // that is not in this game now is set when it is (the heal).
        Field::RefDisabled { disabled } => {
            if actors::disabled(e, form).is_some_and(|d| d != *disabled) {
                gate::applying(e, |e| actors::set_disabled(e, form, *disabled));
            }
            true
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

/// The tag of a proposal (D5, rung 2): what one of a guest's own object scripts did to the shared
/// story in that guest's game (a bottle shot, a trap sprung), sent to the host as a claim. The
/// host's game does it for everyone if it still fits; the guest's game is not put back meanwhile.
pub const PROPOSE_TAG: u16 = 0x5052;
/// How long a guest leaves its own change alone while the host decides.
pub const PROPOSE_HOLD_MS: u64 = 3000;

/// One thing a guest's script did, as the host can do it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Proposal {
    /// A quest script's variable went from `before` to `after`.
    Var { index: u16, before: u64, after: u64 },
    /// A reference was switched off or on.
    Disabled { before: bool, after: bool },
    /// A quest's stage was set.
    Stage(u8),
    /// A quest's objective was given a state.
    Objective { number: u32, state: u32 },
}

impl Proposal {
    /// As a claim's field.
    pub fn field(&self) -> Field {
        let mut data = Vec::new();
        match *self {
            Proposal::Var {
                index,
                before,
                after,
            } => {
                data.push(1);
                data.extend(index.to_le_bytes());
                data.extend(before.to_le_bytes());
                data.extend(after.to_le_bytes());
            }
            Proposal::Disabled { before, after } => data.extend([2, before as u8, after as u8]),
            Proposal::Stage(n) => data.extend([3, n]),
            Proposal::Objective { number, state } => {
                data.push(4);
                data.extend(number.to_le_bytes());
                data.extend(state.to_le_bytes());
            }
        }
        Field::Custom {
            tag: PROPOSE_TAG,
            data,
        }
    }

    /// Reads a claim's field; `None` for anything else or a malformed one.
    pub fn read(f: &Field) -> Option<Proposal> {
        let Field::Custom { tag, data } = f else {
            return None;
        };
        if *tag != PROPOSE_TAG {
            return None;
        }
        let u64_at = |i: usize| {
            data.get(i..i + 8)
                .map(|b| u64::from_le_bytes(b.try_into().unwrap()))
        };
        let u32_at = |i: usize| {
            data.get(i..i + 4)
                .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
        };
        match (data.first()?, data.len()) {
            (1, 19) => Some(Proposal::Var {
                index: u16::from_le_bytes([data[1], data[2]]),
                before: u64_at(3)?,
                after: u64_at(11)?,
            }),
            (2, 3) => Some(Proposal::Disabled {
                before: data[1] != 0,
                after: data[2] != 0,
            }),
            (3, 2) => Some(Proposal::Stage(data[1])),
            (4, 9) => Some(Proposal::Objective {
                number: u32_at(1)?,
                state: u32_at(5)?,
            }),
            _ => None,
        }
    }

    /// What a guest's game holding `found` where the story has `want` proposes.
    pub fn between(form: u32, want: &Field, found: &Field) -> Vec<Proposal> {
        match (want, found) {
            (
                Field::ScriptVar {
                    index,
                    bits: before,
                },
                Field::ScriptVar { bits: after, .. },
            ) => {
                vec![Proposal::Var {
                    index: *index,
                    before: *before,
                    after: *after,
                }]
            }
            (Field::RefDisabled { disabled: before }, Field::RefDisabled { disabled: after }) => {
                vec![Proposal::Disabled {
                    before: *before,
                    after: *after,
                }]
            }
            _ => {
                let (Some(w), Some(f)) = (record::decode(form, want), record::decode(form, found))
                else {
                    return Vec::new();
                };
                let mut out: Vec<Proposal> = f
                    .done
                    .iter()
                    .filter(|n| !w.done.contains(n))
                    .map(|n| Proposal::Stage(*n))
                    .collect();
                for (n, state) in &f.objectives {
                    if w.objectives.iter().find(|o| o.0 == *n).map(|o| o.1) != Some(*state) {
                        out.push(Proposal::Objective {
                            number: *n,
                            state: *state,
                        });
                    }
                }
                out
            }
        }
    }
}

/// The most differences from the story one heal may find new and still propose: more is a game
/// that was not played into them (a load, a reset).
pub const PROPOSE_BURST: usize = 12;
/// How long nothing is proposed after a load or a burst.
pub const PROPOSE_QUIET_MS: u64 = 20_000;

/// How long after an object script naming a quest ran a change to that quest is put down to it.
pub const PROPOSE_RECENT_MS: u64 = 3000;

/// What tells two fields of one kind on one form apart (a variable's index).
fn sub_of(f: &Field) -> u32 {
    match f {
        Field::ScriptVar { index, .. } => u32::from(*index),
        _ => 0,
    }
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
            Field::RefDisabled { .. } => match actors::disabled(e, form) {
                Some(disabled) => Some(Field::RefDisabled { disabled }),
                // Not in this game now: it waits.
                None => continue,
            },
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

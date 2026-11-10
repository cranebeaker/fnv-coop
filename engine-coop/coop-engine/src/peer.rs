//! The peer server's line protocol: one game, driven by a test harness in another process.
//!
//! One binary links one engine, so a harness that links another engine (or none) talks to a
//! game through this protocol. Each request is one line, and each answer is one line: `ok`
//! with values, or `err` with a reason. Forms are hex form ids. Field lists are the text
//! form of `coop_core::schema` ([`coop_core::schema::fields_to_hex`]).
//!
//! | Request | Answer |
//! |---|---|
//! | `catalog Q1,Q2,.. G1,G2,.. [S1,S2,..]` | `ok FOUND MISSING`: the quests and globals to share (`-` for none). S: the scripts that each machine runs itself. The variables of a quest whose own script is one of them are not shared |
//! | `enter CELL [SETTLE]` | `ok FRAMES`: the game goes from its main menu into CELL, and SETTLE frames of play more (default 60). For a server that waits at the menu (`--menu`) |
//! | `frames N` | `ok` after N frames of the game |
//! | `set_stage FORM STAGE` | `ok 1` when the game's own function set it, `ok 0` when refused (a guest refuses) |
//! | `stage FORM` | `ok N`, or `ok none` when no stage was set |
//! | `set_global FORM VALUE` | `ok 1`, or `ok 0` for an unknown global |
//! | `global FORM` | `ok VALUE`, or `ok none` |
//! | `export` | `ok FIELDS` |
//! | `apply FIELDS` | `ok SET CLEARED` |
//! | `apply_snapshot FIELDS` | `ok SET CLEARED` |
//! | `kinds` | `ok KIND:COUNT ..`: how many exported fields of each kind (`Field::kind`) |
//! | `gate Q1,Q2,.. S1,S2,..` | `ok`: this game gets the guest gate, with the local quests and the local scripts (`-` for none). `err` when the gate cannot be put on |
//! | `intents` | `ok INTENT ..`: the changes of shared quests that the gate turned to the host, each one time (`stage:QUEST:N`, `running:QUEST:0|1`, `completed:QUEST:0|1`, `failed:QUEST:0|1`, `objective:QUEST:ID:STATE`) |
//! | `gate_stats` | `ok runs=N stopped=N guarded=N last=FORM quiet_ms=N blind=0|1` |
//! | `moving G1:LIMIT,G2:LIMIT,..` | `ok`: the globals that the program moves itself (its clock), each with the difference from the host that is no leak |
//! | `audit` | `ok leaks=N restored=N last=FORM FORM:KIND ..`: each shared slot that is not the host's last value (at most 64 named), then the host's values are given back. `err` before the host's first snapshot |
//! | `audit report` | the same, and nothing is given back |
//! | `stages` | `ok FORM:STAGE ..`: each catalog quest that has a stage |
//! | `local_stage FORM` | `ok N`: the stage of a quest that is not in the catalog (a quest of each player), `ok none` for an unknown form |
//! | `console LINE` | `ok`: the adapter runs LINE as a console line (not gated) |
//! | `script LINE` | `ok`: LINE runs as a script of the game would (gated). A test's stand-in for a script that the game runs |
//! | `changes baseline` | `ok COUNT`: the game's map of changed forms as it is now is the baseline (COUNT forms) |
//! | `changes` | `ok COUNT FORM:NEW ..`: each form that is new in the map or has a new flag bit since the last look (at most 64 named, NEW in hex). `err` before a baseline |
//! | `record FORM [FLAGS]` | `ok FLAGS KIND VERSION PAIRS BYTES`: the game's own save of this form, for these change flags (default: the flags of its entry in the change map). Hex, the bytes as hex pairs (`-` for none). `err` when the form is not in memory |
//! | `pairs FROM` | `ok INDEX:FORM ..`: the game's form index pairs from index FROM (hex) |
//! | `join_pairs` | `ok`: a guest's own form indexes start above the host's from now on |
//! | `take_pairs INDEX:FORM,..` | `ok TAKEN NEXT`: the host's pairs go into a guest's table, in index order |
//! | `take_record FORM FLAGS KIND VERSION PAIRS BYTES [attach]` | `ok loaded`, `ok waits` (the form is not in memory, or `attach` was asked) or `err refused` |
//! | `form_word FORM OFFSET` | `ok WORD`: the 32-bit word at this offset of the form (hex). `err` when the game has no such form |
//! | `form FORM` | `ok 1` when the game has a form with this id now, else `ok 0` |
//! | `player` | `ok CELL X Y Z HEADING`: the local player's cell (an address, hex), place and heading in degrees. `err` before the game made the player |
//! | `avatar_make SLOT FEMALE RACE X Y Z HEADING` | `ok ID`: the avatar of the remote player SLOT, with a female body (1) or not (0) and the race's form id (0: the base's own), at this place. ID is its form id. `err` when the game makes none |
//! | `avatar_drive SLOT X Y Z HEADING FLAGS [z]` | `ok`: one frame's command (the movement flags in hex, 0 to stand; `z` commands Z too). `err destroyed` or `err dead` when the avatar cannot be used |
//! | `avatar_state SLOT` | `ok X Y Z HEADING LIFE GROUP FLAGS CELL`: what the avatar is now (group, flags and cell in hex) |
//! | `avatar_look SLOT RACE` | `ok FEMALE IS_RACE`: what the game's own tests say of the avatar's base (1 or 0 each) |
//! | `avatar_remove SLOT` | `ok`: the avatar goes from the game |
//! | `avatar_health SLOT` | `ok PART`: the avatar's health as a part of 1, by the game's own test |
//! | `avatar_heal SLOT` | `ok`: the avatar gets its full health again, with the game's own command |
//! | `avatar_guard` | `ok`: this game gets the avatars' damage rules: a local hit on an avatar is recorded and not applied, and local code does not kill an avatar. `err` when a rule cannot be put on |
//! | `avatar_hits` | `ok KILLS SLOT:SOURCE:WEAPON:DAMAGE:LOCATION:FLAGS ..`: the kills of an avatar that were stopped, and the recorded hits, each one time |
//! | `quit` | `ok` |

use coop_core::ids::FormId;
use coop_core::schema::{fields_from_hex, fields_to_hex, Field};

use crate::addr;
use crate::audit::Audit;
use crate::avatar::{Avatar, Base, Drive, Gone, Hit, Look};
use crate::changes::Changes;
use crate::gate::{Counters, Intent, Rules, BLIND_MS};
use crate::program::Program;
use crate::quest;
use crate::record::{self, Pairs, Record, Taken};
use crate::shared::Shared;

/// Which side of a session this game is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// The authority: its game runs the story's scripts.
    Host,
    /// A follower: it gets the story's state as plain values.
    Guest,
}

/// What the server does with a request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    /// Send this line.
    Reply(String),
    /// Run this many frames of the game, then send `ok`.
    Frames(u32),
    /// Go from the main menu into this cell, then send `ok FRAMES` or `err`.
    Enter {
        /// The cell's editor id.
        cell: String,
        /// Frames of play after the cell's first one.
        settle: u32,
    },
    /// Put the guest gate on this game with these rules, then send `ok` or `err`.
    Gate {
        /// The policy's rules.
        rules: Rules,
        /// The address of a shared quest of the catalog, for the gate's start-up check.
        shared_quest: Option<u32>,
    },
    /// Send the gate's waiting intents ([`intents_reply`]).
    Intents,
    /// Send the gate's counters ([`stats_reply`]).
    GateStats,
    /// Put the avatars' damage rules on this game, then send `ok` or `err`.
    AvatarGuard,
    /// Send the recorded hits on avatars ([`hits_reply`]).
    AvatarHits,
    /// Run this console line, then send `ok`. `own`: the adapter's own line (not gated).
    Line {
        /// The line.
        text: String,
        /// The adapter's own line.
        own: bool,
    },
    /// Send `ok` and end.
    Quit,
}

/// One game behind the protocol.
pub struct Peer {
    role: Role,
    shared: Shared,
    audit: Audit,
    /// What this game's change map held at the last look. `None` before a baseline.
    changes: Option<Changes>,
    /// The host's index pairs that this guest's game has. `None` before `join_pairs`.
    pairs: Option<Pairs>,
    /// The avatars of the remote players, by slot: the base copy and the reference.
    avatars: std::collections::BTreeMap<u8, (Base, Option<Avatar>)>,
    /// The script that reached the gate last, for the audit's report. The server sets it.
    pub last_script: u32,
}

fn number<T: std::str::FromStr>(word: Option<&str>, what: &str) -> Result<T, String> {
    word.and_then(|w| w.parse().ok())
        .ok_or(format!("{what} is missing or bad"))
}

fn hex_bytes(word: &str) -> Result<Vec<u8>, String> {
    if word == "-" {
        return Ok(Vec::new());
    }
    if word.len() % 2 != 0 || !word.is_ascii() {
        return Err("bad hex bytes".into());
    }
    (0..word.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&word[i..i + 2], 16).map_err(|_| "bad hex bytes".to_string()))
        .collect()
}

fn gone(why: Gone) -> String {
    match why {
        Gone::NoPlayer => "the game has no player yet",
        Gone::NotMade => "the game made no avatar",
        Gone::Destroyed => "destroyed",
        Gone::Dead => "dead",
    }
    .into()
}

fn hex(word: Option<&str>) -> Result<u32, String> {
    let word = word.ok_or("a form id is missing")?;
    u32::from_str_radix(word.trim_start_matches("0x"), 16).map_err(|_| format!("bad hex {word}"))
}

/// The answer line for the gate's intents.
pub fn intents_reply(intents: &[Intent]) -> String {
    let on = |b: &bool| u8::from(*b);
    let words: Vec<String> = intents
        .iter()
        .map(|i| match i {
            Intent::Stage { quest, stage } => format!("stage:{quest:08X}:{stage}"),
            Intent::Running { quest, on: b } => format!("running:{quest:08X}:{}", on(b)),
            Intent::Completed { quest, on: b } => format!("completed:{quest:08X}:{}", on(b)),
            Intent::Failed { quest, on: b } => format!("failed:{quest:08X}:{}", on(b)),
            Intent::Objective {
                quest,
                objective,
                state,
            } => format!("objective:{quest:08X}:{objective}:{state}"),
        })
        .collect();
    format!("ok {}", words.join(" ")).trim_end().to_string()
}

/// The answer line for the gate's counters. `None`: the game has no gate.
pub fn stats_reply(counters: Option<Counters>) -> String {
    match counters {
        Some(c) => format!(
            "ok runs={} stopped={} guarded={} last={:08X} quiet_ms={} blind={}",
            c.runs,
            c.stopped,
            c.guarded,
            c.last_script,
            c.quiet_ms,
            u8::from(c.quiet_ms >= BLIND_MS)
        ),
        None => "err no gate".into(),
    }
}

/// How many leaks one audit answer names.
const AUDIT_NAMES: usize = 64;

/// The answer line for the stopped kills and the recorded hits on avatars.
pub fn hits_reply(kills: u64, hits: &[Hit]) -> String {
    let mut text = format!("ok {kills}");
    for h in hits {
        text.push_str(&format!(
            " {}:{:08X}:{:08X}:{}:{}:{:X}",
            h.slot, h.source, h.weapon, h.damage, h.location, h.flags
        ));
    }
    text
}

fn id_list(word: Option<&str>) -> Result<Vec<u32>, String> {
    match word {
        None => Err("a list is missing".into()),
        Some("-") => Ok(Vec::new()),
        Some(list) => list.split(',').map(|w| hex(Some(w))).collect(),
    }
}

impl Peer {
    /// A game with an empty catalog.
    pub fn new(role: Role) -> Peer {
        Peer {
            role,
            shared: Shared::default(),
            audit: Audit::default(),
            changes: None,
            pairs: None,
            avatars: Default::default(),
            last_script: 0,
        }
    }

    /// The avatars that are in this game now: (address, slot). The server gives them to the
    /// avatars' damage rules.
    pub fn bodies(&self) -> Vec<(u32, u8)> {
        self.avatars
            .iter()
            .filter_map(|(slot, (_, avatar))| avatar.map(|a| (a.at, *slot)))
            .collect()
    }

    /// Answers one request line.
    pub fn handle(&mut self, p: &mut dyn Program, line: &str) -> Action {
        match self.answer(p, line) {
            Ok(action) => action,
            Err(why) => Action::Reply(format!("err {why}")),
        }
    }

    fn answer(&mut self, p: &mut dyn Program, line: &str) -> Result<Action, String> {
        let mut words = line.split_whitespace();
        let verb = words.next().ok_or("an empty request")?;
        let reply = |text: String| Ok(Action::Reply(text));
        match verb {
            "catalog" => {
                let quests = id_list(words.next())?;
                let globals = id_list(words.next())?;
                let local_scripts = match words.next() {
                    Some(list) => id_list(Some(list))?,
                    None => Vec::new(),
                };
                self.shared = Shared::with_local_scripts(p, &quests, &globals, &local_scripts);
                let missing = self.shared.missing.len();
                reply(format!(
                    "ok {} {missing}",
                    quests.len() + globals.len() - missing
                ))
            }
            "enter" => {
                let cell = words.next().ok_or("enter needs a cell")?.to_string();
                let settle = match words.next() {
                    Some(word) => word.parse().map_err(|_| "a bad frame count")?,
                    None => 60,
                };
                Ok(Action::Enter { cell, settle })
            }
            "frames" => {
                let n = words.next().and_then(|w| w.parse().ok());
                Ok(Action::Frames(n.ok_or("frames needs a number")?))
            }
            "set_stage" => {
                let quest = self.shared.address(hex(words.next())?);
                let stage: u8 = words
                    .next()
                    .and_then(|w| w.parse().ok())
                    .ok_or("set_stage needs a stage")?;
                // Only the host's game runs the story: a guest refuses, and calls nothing.
                let done = match (self.role, quest) {
                    (Role::Host, Some(at)) => p
                        .call(addr::QUEST_SET_STAGE, &[at, u32::from(stage)])
                        .bool(),
                    _ => false,
                };
                reply(format!("ok {}", u8::from(done)))
            }
            "stage" => {
                let stage = self.shared.address(hex(words.next())?).and_then(|at| {
                    quest::fields(&quest::read(p, at))
                        .into_iter()
                        .find_map(|f| match f {
                            Field::QuestStage { stage } => Some(stage),
                            _ => None,
                        })
                });
                reply(stage.map_or("ok none".into(), |s| format!("ok {s}")))
            }
            "set_global" => {
                let global = self.shared.address(hex(words.next())?);
                let value: f32 = words
                    .next()
                    .and_then(|w| w.parse().ok())
                    .ok_or("set_global needs a value")?;
                if let Some(at) = global {
                    p.set_f32(at + addr::GLOBAL_VALUE, value);
                }
                reply(format!("ok {}", u8::from(global.is_some())))
            }
            "global" => {
                let value = self
                    .shared
                    .address(hex(words.next())?)
                    .map(|at| p.f32(at + addr::GLOBAL_VALUE));
                reply(value.map_or("ok none".into(), |v| format!("ok {v}")))
            }
            "export" => reply(format!("ok {}", fields_to_hex(&self.shared.export(p)))),
            "apply" | "apply_snapshot" => {
                let fields: Vec<(FormId, Field)> =
                    fields_from_hex(words.next().ok_or("a field list is missing")?)?;
                let applied = if verb == "apply" {
                    self.audit.delta(&fields);
                    self.shared.apply(p, &fields)
                } else {
                    self.audit.snapshot(&fields);
                    self.shared.apply_snapshot(p, &fields)
                };
                reply(format!("ok {} {}", applied.set, applied.cleared))
            }
            "kinds" => {
                let mut counts: std::collections::BTreeMap<u8, u32> = Default::default();
                for (_, field) in self.shared.export(p) {
                    *counts.entry(field.kind()).or_default() += 1;
                }
                let words: Vec<String> = counts.iter().map(|(k, n)| format!("{k}:{n}")).collect();
                reply(format!("ok {}", words.join(" ")).trim_end().to_string())
            }
            "gate" => {
                if self.role != Role::Guest {
                    return Err("only a guest's game takes the gate".into());
                }
                let local_quests = id_list(words.next())?.into_iter().collect();
                let local_scripts = id_list(words.next())?.into_iter().collect();
                let rules = Rules {
                    local_quests,
                    local_scripts,
                };
                let shared_quest = self
                    .shared
                    .quests()
                    .find(|(id, _)| !rules.quest_is_local(*id))
                    .map(|(_, at)| at);
                Ok(Action::Gate {
                    rules,
                    shared_quest,
                })
            }
            "moving" => {
                let mut moving = Vec::new();
                match words.next().ok_or("a list is missing")? {
                    "-" => {}
                    list => {
                        for word in list.split(',') {
                            let (id, limit) = word.split_once(':').ok_or("moving needs G:LIMIT")?;
                            let limit: f32 = limit.parse().map_err(|_| "a bad limit")?;
                            moving.push((hex(Some(id))?, limit));
                        }
                    }
                }
                self.audit.set_moving(moving);
                reply("ok".into())
            }
            "audit" => {
                if !self.audit.known() {
                    return Err("the host sent no snapshot".into());
                }
                let leaks = self.audit.leaks(&self.shared.export(p));
                let restored = match words.next() {
                    Some("report") => 0,
                    Some(other) => return Err(format!("unknown audit mode {other}")),
                    None => self.shared.restore(p, &leaks),
                };
                let mut text = format!(
                    "ok leaks={} restored={restored} last={:08X}",
                    leaks.len(),
                    self.last_script
                );
                for leak in leaks.iter().take(AUDIT_NAMES) {
                    text.push_str(&format!(" {:08X}:{}", leak.form.0, leak.kind));
                }
                reply(text)
            }
            "stages" => {
                let mut text = String::from("ok");
                for (id, at) in self.shared.quests() {
                    for field in quest::fields(&quest::read(p, at)) {
                        if let Field::QuestStage { stage } = field {
                            text.push_str(&format!(" {id:08X}:{stage}"));
                        }
                    }
                }
                reply(text)
            }
            "local_stage" => {
                let at = p.call(addr::FORM_BY_ID, &[hex(words.next())?]).eax;
                reply(match at {
                    0 => "ok none".into(),
                    at => format!("ok {}", p.u8(at + addr::QUEST_STAGE)),
                })
            }
            "changes" => match words.next() {
                Some("baseline") => {
                    let changes = Changes::baseline(p);
                    let count = changes.len();
                    self.changes = Some(changes);
                    reply(format!("ok {count}"))
                }
                Some(other) => Err(format!("unknown changes mode {other}")),
                None => {
                    let changes = self.changes.as_mut().ok_or("no baseline of the changes")?;
                    let dirty = changes.look(p);
                    let mut text = format!("ok {}", dirty.len());
                    for d in dirty.iter().take(AUDIT_NAMES) {
                        text.push_str(&format!(" {:08X}:{:X}", d.form, d.new));
                    }
                    reply(text)
                }
            },
            "record" => {
                let form = hex(words.next())?;
                let flags = match words.next() {
                    Some(word) => hex(Some(word))?,
                    None => crate::changes::walk(p)
                        .iter()
                        .find(|e| e.form == form)
                        .map_or(0, |e| e.flags),
                };
                let r = record::make(p, form, flags).ok_or("the form is not in memory")?;
                let bytes: String = r.bytes.iter().map(|b| format!("{b:02x}")).collect();
                reply(format!(
                    "ok {:X} {:X} {:X} {:X} {}",
                    r.flags,
                    r.kind,
                    r.version,
                    r.pairs,
                    if bytes.is_empty() { "-" } else { &bytes }
                ))
            }
            "pairs" => {
                let mut text = String::from("ok");
                for (index, form) in record::pairs(p, hex(words.next())?) {
                    text.push_str(&format!(" {index:X}:{form:08X}"));
                }
                reply(text)
            }
            "join_pairs" => {
                if self.role != Role::Guest {
                    return Err("only a guest joins".into());
                }
                self.pairs = Some(Pairs::join(p).ok_or("the game has no save manager yet")?);
                reply("ok".into())
            }
            "take_pairs" => {
                let have = self.pairs.as_mut().ok_or("no join_pairs yet")?;
                let mut pairs = Vec::new();
                for word in words.next().ok_or("a pair list is missing")?.split(',') {
                    let (index, form) = word.split_once(':').ok_or("a pair is INDEX:FORM")?;
                    pairs.push((hex(Some(index))?, hex(Some(form))?));
                }
                let taken = have.insert(p, &pairs);
                reply(format!("ok {taken} {:X}", have.next()))
            }
            "take_record" => {
                let have = self.pairs.ok_or("no join_pairs yet")?;
                let r = Record {
                    form: hex(words.next())?,
                    flags: hex(words.next())?,
                    kind: hex(words.next())? as u8,
                    version: hex(words.next())? as u8,
                    pairs: hex(words.next())?,
                    bytes: hex_bytes(words.next().ok_or("the bytes are missing")?)?,
                };
                let load = words.next() != Some("attach");
                match record::take(p, &r, &have, load) {
                    Taken::Loaded => reply("ok loaded".into()),
                    Taken::Waits => reply("ok waits".into()),
                    Taken::Refused => Err("refused".into()),
                }
            }
            "form_word" => {
                let at = p.call(addr::FORM_BY_ID, &[hex(words.next())?]).eax;
                if at == 0 {
                    return Err("the game has no such form".into());
                }
                reply(format!("ok {:08X}", p.u32(at + hex(words.next())?)))
            }
            "form" => {
                let at = p.call(addr::FORM_BY_ID, &[hex(words.next())?]).eax;
                reply(format!("ok {}", u8::from(at != 0)))
            }
            "player" => {
                let at = p.u32(addr::PLAYER);
                if at == 0 {
                    return Err("the game has no player yet".into());
                }
                let place = [0, 4, 8].map(|o| p.f32(at + addr::REF_POSITION + o));
                reply(format!(
                    "ok {:08X} {} {} {} {}",
                    p.u32(at + addr::REF_CELL),
                    place[0],
                    place[1],
                    place[2],
                    p.f32(at + addr::REF_ANGLE + 8).to_degrees()
                ))
            }
            "avatar_make" => {
                let slot: u8 = number(words.next(), "the slot")?;
                let look = Look {
                    female: number::<u8>(words.next(), "the sex")? != 0,
                    race: hex(words.next())?,
                };
                let first = Drive {
                    place: [
                        number(words.next(), "x")?,
                        number(words.next(), "y")?,
                        number(words.next(), "z")?,
                    ],
                    heading: number(words.next(), "the heading")?,
                    flags: 0,
                    with_z: true,
                };
                // One base for each remote player. An avatar that is made again uses it again.
                let base = match self.avatars.get(&slot) {
                    Some((base, _)) => *base,
                    None => Base::make(p, look).ok_or("the game made no base copy")?,
                };
                if let Some((_, Some(old))) = self.avatars.get(&slot) {
                    old.remove(p);
                }
                self.avatars.insert(slot, (base, None));
                let avatar = Avatar::make(p, base, &first).map_err(gone)?;
                self.avatars.insert(slot, (base, Some(avatar)));
                reply(format!("ok {:08X}", avatar.id))
            }
            "avatar_guard" => Ok(Action::AvatarGuard),
            "avatar_hits" => Ok(Action::AvatarHits),
            "avatar_drive" | "avatar_state" | "avatar_look" | "avatar_health" | "avatar_heal"
            | "avatar_remove" => {
                let slot: u8 = number(words.next(), "the slot")?;
                let Some((base, Some(avatar))) = self.avatars.get(&slot).copied() else {
                    return Err(format!("no avatar in slot {slot}"));
                };
                match verb {
                    "avatar_drive" => {
                        let place = [
                            number(words.next(), "x")?,
                            number(words.next(), "y")?,
                            number(words.next(), "z")?,
                        ];
                        let drive = Drive {
                            place,
                            heading: number(words.next(), "the heading")?,
                            flags: hex(words.next())?,
                            with_z: words.next() == Some("z"),
                        };
                        avatar.drive(p, &drive).map_err(gone)?;
                        reply("ok".into())
                    }
                    "avatar_state" => {
                        let seen = avatar.seen(p).map_err(gone)?;
                        reply(format!(
                            "ok {} {} {} {} {} {:X} {:X} {:08X}",
                            seen.place[0],
                            seen.place[1],
                            seen.place[2],
                            seen.heading,
                            seen.life,
                            seen.group,
                            seen.flags,
                            seen.cell
                        ))
                    }
                    "avatar_health" => {
                        let part = avatar.health(p).map_err(gone)?;
                        reply(format!("ok {part}"))
                    }
                    "avatar_heal" => {
                        if !avatar.heal(p).map_err(gone)? {
                            return Err("the game has no console manager".into());
                        }
                        reply("ok".into())
                    }
                    "avatar_look" => {
                        let (female, is_race) = avatar.look(p, hex(words.next())?).map_err(gone)?;
                        reply(format!("ok {} {}", u8::from(female), u8::from(is_race)))
                    }
                    _ => {
                        avatar.remove(p);
                        self.avatars.insert(slot, (base, None));
                        reply("ok".into())
                    }
                }
            }
            "intents" => Ok(Action::Intents),
            "gate_stats" => Ok(Action::GateStats),
            "console" | "script" => {
                let text = words.collect::<Vec<_>>().join(" ");
                if text.is_empty() {
                    return Err(format!("{verb} needs a line"));
                }
                Ok(Action::Line {
                    text,
                    own: verb == "console",
                })
            }
            "quit" => Ok(Action::Quit),
            other => Err(format!("unknown request {other}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeProgram;

    const QUEST: &str = "0015D912";
    const HOUR: &str = "00000038";

    fn game(role: Role) -> (FakeProgram, Peer) {
        let mut p = FakeProgram::new();
        p.add_quest(0x0015_d912, &[10, 20], &[10]);
        p.add_global(0x38, 12.0);
        let mut peer = Peer::new(role);
        let got = peer.handle(&mut p, &format!("catalog {QUEST},00000BAD {HOUR}"));
        assert_eq!(got, Action::Reply("ok 2 1".into()));
        (p, peer)
    }

    fn ask(peer: &mut Peer, p: &mut FakeProgram, line: &str) -> String {
        match peer.handle(p, line) {
            Action::Reply(text) => text,
            other => panic!("{line}: {other:?}"),
        }
    }

    /// A host sets a stage with the game's function. A guest refuses and calls nothing.
    /// Mutation twin: the guest's stage stays unset.
    #[test]
    fn a_host_sets_a_stage_and_a_guest_refuses() {
        let (mut hp, mut host) = game(Role::Host);
        assert_eq!(
            ask(&mut host, &mut hp, &format!("stage {QUEST}")),
            "ok none"
        );
        assert_eq!(
            ask(&mut host, &mut hp, &format!("set_stage {QUEST} 10")),
            "ok 1"
        );
        assert_eq!(
            ask(&mut host, &mut hp, &format!("set_stage {QUEST} 11")),
            "ok 0"
        );
        assert_eq!(ask(&mut host, &mut hp, &format!("stage {QUEST}")), "ok 10");

        let (mut gp, mut guest) = game(Role::Guest);
        let calls = gp.calls.len();
        assert_eq!(
            ask(&mut guest, &mut gp, &format!("set_stage {QUEST} 10")),
            "ok 0"
        );
        assert_eq!(gp.calls.len(), calls, "a guest calls no game function");
        assert_eq!(
            ask(&mut guest, &mut gp, &format!("stage {QUEST}")),
            "ok none"
        );
    }

    /// The host's export, applied as a snapshot, gives the guest the same export.
    #[test]
    fn an_export_crosses_the_protocol_to_a_guest() {
        let (mut hp, mut host) = game(Role::Host);
        ask(&mut host, &mut hp, &format!("set_stage {QUEST} 10"));
        assert_eq!(
            ask(&mut host, &mut hp, &format!("set_global {HOUR} 3.5")),
            "ok 1"
        );
        let exported = ask(&mut host, &mut hp, "export");
        let fields = exported.strip_prefix("ok ").unwrap();

        let (mut gp, mut guest) = game(Role::Guest);
        assert_ne!(ask(&mut guest, &mut gp, "export"), exported);
        let applied = ask(&mut guest, &mut gp, &format!("apply_snapshot {fields}"));
        assert!(
            applied.starts_with("ok ") && applied.ends_with(" 0"),
            "{applied}"
        );
        assert_eq!(ask(&mut guest, &mut gp, "export"), exported);
        assert_eq!(
            ask(&mut guest, &mut gp, &format!("apply_snapshot {fields}")),
            "ok 0 0"
        );
        assert_eq!(
            ask(&mut guest, &mut gp, &format!("global {HOUR}")),
            "ok 3.5"
        );
        assert_eq!(ask(&mut guest, &mut gp, &format!("stage {QUEST}")), "ok 10");
        // a stage (0), flags (1), a global (3) and a chunk of done stages (255)
        assert_eq!(ask(&mut guest, &mut gp, "kinds"), "ok 0:1 1:1 3:1 255:1");
    }

    /// The gate requests parse to actions for the server, and their answers have one form.
    #[test]
    fn the_gate_requests_and_their_answers() {
        let (mut p, mut guest) = game(Role::Guest);
        let want = Rules {
            local_quests: [0x0010_4c1c].into(),
            local_scripts: [0x0007_691e, 0x10].into(),
        };
        let shared_quest = p.call(addr::FORM_BY_ID, &[0x0015_d912]).eax;
        assert_ne!(shared_quest, 0);
        assert_eq!(
            guest.handle(&mut p, "gate 00104C1C 0007691E,00000010"),
            Action::Gate {
                rules: want,
                shared_quest: Some(shared_quest),
            }
        );
        assert_eq!(
            guest.handle(&mut p, "gate 0015D912 -"),
            Action::Gate {
                rules: Rules {
                    local_quests: [0x0015_d912].into(),
                    local_scripts: Default::default(),
                },
                shared_quest: None,
            },
            "a local quest is not the check's shared quest"
        );
        assert_eq!(guest.handle(&mut p, "intents"), Action::Intents);
        assert_eq!(
            guest.handle(&mut p, "enter GSDocMitchellHouse"),
            Action::Enter {
                cell: "GSDocMitchellHouse".into(),
                settle: 60,
            }
        );
        assert_eq!(
            guest.handle(&mut p, "enter Goodsprings 5"),
            Action::Enter {
                cell: "Goodsprings".into(),
                settle: 5,
            }
        );
        assert!(ask(&mut guest, &mut p, "enter").starts_with("err "));
        assert_eq!(guest.handle(&mut p, "gate_stats"), Action::GateStats);
        assert_eq!(
            guest.handle(&mut p, "console SetStage 0015D912 10"),
            Action::Line {
                text: "SetStage 0015D912 10".into(),
                own: true,
            }
        );
        assert_eq!(
            guest.handle(&mut p, "script set GameHour to 3"),
            Action::Line {
                text: "set GameHour to 3".into(),
                own: false,
            }
        );
        assert!(ask(&mut guest, &mut p, "console").starts_with("err "));
        let (mut hp, mut host) = game(Role::Host);
        assert!(ask(&mut host, &mut hp, "gate - -").starts_with("err "));

        let intents = [
            Intent::Stage {
                quest: 0x0015_d912,
                stage: 10,
            },
            Intent::Running {
                quest: 0x0015_d912,
                on: true,
            },
            Intent::Objective {
                quest: 0x0015_d912,
                objective: 20,
                state: 3,
            },
        ];
        assert_eq!(
            intents_reply(&intents),
            "ok stage:0015D912:10 running:0015D912:1 objective:0015D912:20:3"
        );
        assert_eq!(intents_reply(&[]), "ok");
        let counters = Counters {
            runs: 4,
            stopped: 3,
            guarded: 2,
            last_script: 0x100,
            quiet_ms: 5000,
        };
        assert_eq!(
            stats_reply(Some(counters)),
            "ok runs=4 stopped=3 guarded=2 last=00000100 quiet_ms=5000 blind=1"
        );
        assert_eq!(stats_reply(None), "err no gate");
    }

    /// The audit of a guest: a stage that the guest's own code set is named with the
    /// quest's form id and the last script, and the host's value comes back. The clock is
    /// no leak inside its limit. Mutation twin: `audit report` gives nothing back.
    #[test]
    fn the_audit_reports_and_restores() {
        let (mut hp, mut host) = game(Role::Host);
        assert_eq!(
            ask(&mut host, &mut hp, &format!("set_stage {QUEST} 10")),
            "ok 1"
        );
        let snapshot = ask(&mut host, &mut hp, "export");
        let fields = snapshot.strip_prefix("ok ").unwrap();

        let (mut gp, mut guest) = game(Role::Guest);
        assert!(ask(&mut guest, &mut gp, "audit").starts_with("err "));
        ask(&mut guest, &mut gp, &format!("apply_snapshot {fields}"));
        assert_eq!(
            ask(&mut guest, &mut gp, "audit"),
            "ok leaks=0 restored=0 last=00000000"
        );
        assert_eq!(ask(&mut guest, &mut gp, "stages"), format!("ok {QUEST}:10"));
        assert_eq!(
            ask(&mut guest, &mut gp, &format!("local_stage {QUEST}")),
            "ok 10"
        );
        assert_eq!(ask(&mut guest, &mut gp, "local_stage 00000BAD"), "ok none");

        // The guest's own code moves the quest and the clock.
        let quest = gp.call(addr::FORM_BY_ID, &[0x0015_d912]).eax;
        gp.call(addr::QUEST_SET_STAGE, &[quest, 20]);
        let hour = gp.call(addr::FORM_BY_ID, &[0x38]).eax;
        gp.set_f32(hour + addr::GLOBAL_VALUE, 12.01);
        guest.last_script = 0x0010_4c02;
        assert_eq!(
            ask(&mut guest, &mut gp, &format!("moving {HOUR}:0.05")),
            "ok"
        );
        let report = ask(&mut guest, &mut gp, "audit report");
        assert!(
            report.starts_with("ok leaks=") && report.contains(" restored=0 last=00104C02"),
            "{report}"
        );
        assert!(report.contains(&format!(" {QUEST}:0")), "{report}");
        assert!(
            !report.contains(HOUR),
            "the clock is inside its limit: {report}"
        );
        assert_ne!(ask(&mut guest, &mut gp, "export"), snapshot, "the twin");

        let report = ask(&mut guest, &mut gp, "audit");
        assert!(!report.contains(" restored=0 "), "{report}");
        assert_eq!(
            ask(&mut guest, &mut gp, "audit"),
            "ok leaks=0 restored=0 last=00104C02"
        );
        // The clock keeps its own value: it was no leak.
        assert_eq!(
            ask(&mut guest, &mut gp, &format!("global {HOUR}")),
            "ok 12.01"
        );
        assert!(ask(&mut guest, &mut gp, "moving 38").starts_with("err "));
    }

    /// The avatar requests: make, one frame's command, the state, and the removal. A slot
    /// keeps its base copy when its avatar is made again.
    #[test]
    fn the_avatar_requests() {
        let (mut p, mut peer) = game(Role::Guest);
        assert!(ask(&mut peer, &mut p, "player").starts_with("err "));
        assert!(ask(&mut peer, &mut p, "avatar_make 1 1 0 1 2 3 90").starts_with("err "));
        p.add_player([100.0, 200.0, 300.0]);
        assert_eq!(
            ask(&mut peer, &mut p, "player"),
            "ok 00000000 100 200 300 0"
        );
        assert!(ask(&mut peer, &mut p, "avatar_state 1").starts_with("err no avatar"));

        let made = ask(&mut peer, &mut p, "avatar_make 1 1 0 2300 1611.5 7361 270");
        let id = made.strip_prefix("ok ").expect("an id").to_string();
        let copies = |p: &FakeProgram| p.calls.iter().filter(|(a, _)| *a == addr::NPC_COPY).count();
        assert_eq!(copies(&p), 1);
        assert_eq!(
            ask(&mut peer, &mut p, "avatar_state 1"),
            "ok 2300 1611.5 7361 270 5 FFFF 0 00000000"
        );
        assert_eq!(
            ask(&mut peer, &mut p, "avatar_drive 1 2298.6 1611.5 0 270 101"),
            "ok"
        );
        assert_eq!(
            ask(&mut peer, &mut p, "avatar_state 1"),
            "ok 2298.6 1611.5 7361 270 5 3 101 00000000",
            "Z is the game's without the word z"
        );
        assert_eq!(
            ask(
                &mut peer,
                &mut p,
                "avatar_drive 1 2298.6 1611.5 7000 90 0 z"
            ),
            "ok"
        );
        assert_eq!(
            ask(&mut peer, &mut p, "avatar_state 1"),
            "ok 2298.6 1611.5 7000 90 5 FFFF 0 00000000"
        );

        assert_eq!(ask(&mut peer, &mut p, "avatar_look 1 0000424A"), "ok 1 0");
        assert_eq!(ask(&mut peer, &mut p, "avatar_health 1"), "ok 0");
        assert!(ask(&mut peer, &mut p, "avatar_heal 1").starts_with("err the game has no console"));
        p.console_manager = 0x3000_0000;
        assert_eq!(ask(&mut peer, &mut p, "avatar_heal 1"), "ok");
        let run = p
            .calls
            .iter()
            .rev()
            .find(|(a, _)| *a == addr::SCRIPT_RUN)
            .unwrap();
        assert_eq!(
            run.1[3],
            peer.bodies()[0].0,
            "the avatar is the calling reference"
        );
        assert_eq!(peer.bodies().len(), 1);
        assert_eq!(peer.bodies()[0].1, 1);
        assert_eq!(peer.handle(&mut p, "avatar_guard"), Action::AvatarGuard);
        assert_eq!(peer.handle(&mut p, "avatar_hits"), Action::AvatarHits);
        let hit = Hit {
            slot: 1,
            source: 0x14,
            weapon: 0x000e_3778,
            damage: 12.5,
            location: 1,
            flags: 4,
        };
        assert_eq!(hits_reply(2, &[hit]), "ok 2 1:00000014:000E3778:12.5:1:4");
        assert_eq!(hits_reply(0, &[]), "ok 0");
        assert_eq!(ask(&mut peer, &mut p, &format!("form {id}")), "ok 1");
        assert_eq!(ask(&mut peer, &mut p, "avatar_remove 1"), "ok");
        assert_eq!(ask(&mut peer, &mut p, &format!("form {id}")), "ok 0");
        assert!(ask(&mut peer, &mut p, "avatar_drive 1 0 0 0 0 0").starts_with("err no avatar"));
        let again = ask(&mut peer, &mut p, "avatar_make 1 1 0 2300 1611.5 7361 270");
        assert!(
            again.starts_with("ok ") && again != format!("ok {id}"),
            "{again}"
        );
        assert_eq!(copies(&p), 1, "the slot keeps its base copy");
        assert!(ask(&mut peer, &mut p, "avatar_make 1 1 0 2300").starts_with("err "));
    }

    /// The changes requests: a baseline, then each new form and flag bit one time.
    #[test]
    fn the_changes_requests() {
        let (mut p, mut peer) = game(Role::Host);
        assert!(ask(&mut peer, &mut p, "changes").starts_with("err "));
        assert_eq!(ask(&mut peer, &mut p, "changes baseline"), "ok 0");
        p.add_change_map(8);
        p.mark_changed(0x0016_8c28, 0x20);
        assert_eq!(ask(&mut peer, &mut p, "changes baseline"), "ok 1");
        assert_eq!(ask(&mut peer, &mut p, "changes"), "ok 0");
        p.mark_changed(0x0010_4c13, 1);
        p.mark_changed(0x0016_8c28, 0x8000_0040);
        assert_eq!(
            ask(&mut peer, &mut p, "changes"),
            "ok 2 00104C13:1 00168C28:80000040"
        );
        assert_eq!(ask(&mut peer, &mut p, "changes"), "ok 0");
        assert!(ask(&mut peer, &mut p, "changes all").starts_with("err "));
    }

    /// A record crosses the protocol: the host's save of a form, its index pairs, and the
    /// guest's take. A guest that lacks the pairs refuses the record.
    #[test]
    fn the_record_requests() {
        const DOOR: u32 = 0x0010_4c13;
        let build = |role: Role| {
            let (mut p, peer) = game(role);
            p.add_change_map(8);
            p.add_save_tables(0x0b);
            let door = p.add_form(DOOR);
            (p, peer, door)
        };
        let (mut hp, mut host, door) = build(Role::Host);
        hp.set_form_bytes(door, &[0xaa, 0x01]);
        hp.mark_changed(DOOR, 0x8000_0001);
        hp.add_pair(0x0010_4c0f);
        assert_eq!(
            ask(&mut host, &mut hp, "record 00104C13"),
            "ok 80000001 3A B 2 aa01"
        );
        assert_eq!(
            ask(&mut host, &mut hp, "record 00104C13 1"),
            "ok 1 3A B 2 aa01"
        );
        assert!(ask(&mut host, &mut hp, "record 00000BAD").starts_with("err "));
        assert_eq!(ask(&mut host, &mut hp, "pairs 1"), "ok 1:00104C0F");
        assert!(ask(&mut host, &mut hp, "join_pairs").starts_with("err "));
        assert_eq!(
            ask(&mut host, &mut hp, "form_word 00104C13 C"),
            "ok 00104C13"
        );

        let (mut gp, mut guest, guest_door) = build(Role::Guest);
        let take = "take_record 00104C13 1 3A B 2 aa01";
        assert!(ask(&mut guest, &mut gp, take).starts_with("err no join_pairs"));
        assert_eq!(ask(&mut guest, &mut gp, "join_pairs"), "ok");
        assert_eq!(ask(&mut guest, &mut gp, take), "err refused");
        assert_eq!(ask(&mut guest, &mut gp, "take_pairs 1:00104C0F"), "ok 1 2");
        assert_eq!(
            ask(&mut guest, &mut gp, &format!("{take} attach")),
            "ok waits"
        );
        assert!(gp.loaded.is_empty());
        assert_eq!(ask(&mut guest, &mut gp, take), "ok loaded");
        assert_eq!(gp.loaded, [guest_door]);
        assert_eq!(gp.pending_bytes(DOOR), Some(vec![0xaa, 0x01]));
        assert!(ask(&mut guest, &mut gp, "take_record 00104C13 1 3A B 2 a").starts_with("err "));
        assert!(ask(&mut guest, &mut gp, "form_word 00000BAD 8").starts_with("err "));
    }

    #[test]
    fn frames_quit_and_bad_requests() {
        let (mut p, mut peer) = game(Role::Host);
        assert_eq!(peer.handle(&mut p, "frames 30"), Action::Frames(30));
        assert_eq!(peer.handle(&mut p, "quit"), Action::Quit);
        for bad in [
            "",
            "frames",
            "fly",
            "stage",
            "stage xyz",
            "apply zz",
            "set_global 38",
        ] {
            let got = ask(&mut peer, &mut p, bad);
            assert!(got.starts_with("err "), "{bad:?}: {got}");
        }
        assert_eq!(ask(&mut peer, &mut p, "global 00000999"), "ok none");
        assert_eq!(ask(&mut peer, &mut p, "set_global 00000999 1"), "ok 0");
    }
}

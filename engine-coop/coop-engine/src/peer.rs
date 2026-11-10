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
//! | `quit` | `ok` |

use coop_core::ids::FormId;
use coop_core::schema::{fields_from_hex, fields_to_hex, Field};

use crate::addr;
use crate::audit::Audit;
use crate::gate::{Counters, Intent, Rules, BLIND_MS};
use crate::program::Program;
use crate::quest;
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
    /// The script that reached the gate last, for the audit's report. The server sets it.
    pub last_script: u32,
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
            last_script: 0,
        }
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

//! The peer server's line protocol: one game, driven by a test harness in another process.
//!
//! One binary links one engine, so a harness that links another engine (or none) talks to a
//! game through this protocol. Each request is one line, and each answer is one line: `ok`
//! with values, or `err` with a reason. Forms are hex form ids. Field lists are the text
//! form of `coop_core::schema` ([`coop_core::schema::fields_to_hex`]).
//!
//! | Request | Answer |
//! |---|---|
//! | `catalog Q1,Q2,.. G1,G2,..` | `ok FOUND MISSING`: the quests and globals to share (`-` for none) |
//! | `frames N` | `ok` after N frames of the game |
//! | `set_stage FORM STAGE` | `ok 1` when the game's own function set it, `ok 0` when refused (a guest refuses) |
//! | `stage FORM` | `ok N`, or `ok none` when no stage was set |
//! | `set_global FORM VALUE` | `ok 1`, or `ok 0` for an unknown global |
//! | `global FORM` | `ok VALUE`, or `ok none` |
//! | `export` | `ok FIELDS` |
//! | `apply FIELDS` | `ok SET CLEARED` |
//! | `apply_snapshot FIELDS` | `ok SET CLEARED` |
//! | `quit` | `ok` |

use coop_core::ids::FormId;
use coop_core::schema::{fields_from_hex, fields_to_hex, Field};

use crate::addr;
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
    /// Send `ok` and end.
    Quit,
}

/// One game behind the protocol.
pub struct Peer {
    role: Role,
    shared: Shared,
}

fn hex(word: Option<&str>) -> Result<u32, String> {
    let word = word.ok_or("a form id is missing")?;
    u32::from_str_radix(word.trim_start_matches("0x"), 16).map_err(|_| format!("bad hex {word}"))
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
                self.shared = Shared::new(p, &quests, &globals);
                let missing = self.shared.missing.len();
                reply(format!(
                    "ok {} {missing}",
                    quests.len() + globals.len() - missing
                ))
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
                    self.shared.apply(p, &fields)
                } else {
                    self.shared.apply_snapshot(p, &fields)
                };
                reply(format!("ok {} {}", applied.set, applied.cleared))
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

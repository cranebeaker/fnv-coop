//! The engine backend's peer: one game on the upstream engine in a child process.
//!
//! One binary links one engine. This crate links the engine snapshot of this workspace, so it
//! reaches the upstream engine through a process boundary: the peer server of `engine-coop`
//! (`coop-engine-peer`) runs one game and answers a line protocol on its standard input and
//! output. [`EnginePeer`] starts one server and is a [`BackendPeer`], so the conformance
//! tests run on the real game program as they run on the `world` layer.
//!
//! `COOP_ENGINE_PEER` names the server's program. Without it the engine backend stays
//! `PENDING (engine backend)`. The caller's environment gives the server its machine table
//! (`NV_D3D9_ADAPTER`, a scratch instance: see `engine-coop/README.md`).

use std::cell::RefCell;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::Duration;

use coop_core::delta::{Delta, SnapEntry};
use coop_core::hash::ShardHashes;
use coop_core::ids::FormId as CForm;
use coop_core::schema::{fields_from_hex, fields_to_hex, Field};

use crate::backend::{AuditReport, AvatarHit, AvatarState, BackendPeer, ChangeRecord, GateStats};
use crate::data::Data;

/// The environment variable that names the peer server's program.
pub const PEER_PROGRAM: &str = "COOP_ENGINE_PEER";
/// What each line of the server starts with. Other lines are the engine's own.
const MARK: &str = "@ ";
/// The longest wait for the game to start and reach play.
const READY_WAIT: Duration = Duration::from_secs(300);
/// The longest wait for one answer.
const ANSWER_WAIT: Duration = Duration::from_secs(300);
/// How many times a server is started. The engine's stepped threads can stop a run in its
/// first frames ("would block forever"): a new start works.
const STARTS: u32 = 4;
/// The cell that a peer's game goes into from its main menu.
const CELL: &str = "GSDocMitchellHouse";

/// What two games share, for a peer's catalog (form ids).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Catalog {
    /// The shared quests.
    pub quests: Vec<u32>,
    /// The shared globals.
    pub globals: Vec<u32>,
    /// The scripts that each machine runs itself. The variables of a quest whose own script
    /// is one of them are each machine's own.
    pub local_scripts: Vec<u32>,
}

/// A guest's gate: the quests and the scripts that its game runs itself (form ids).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Gate {
    /// The local quests.
    pub quests: Vec<u32>,
    /// The local scripts.
    pub scripts: Vec<u32>,
}

/// The server's program, when the environment names one.
pub fn program() -> Option<PathBuf> {
    std::env::var_os(PEER_PROGRAM)
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
}

struct Io {
    child: Child,
    input: ChildStdin,
    lines: Receiver<String>,
}

impl Io {
    /// The server's next line, without its mark.
    fn line(&mut self, wait: Duration) -> Result<String, String> {
        match self.lines.recv_timeout(wait) {
            Ok(line) => Ok(line),
            Err(RecvTimeoutError::Timeout) => Err(format!("no answer in {} s", wait.as_secs())),
            Err(RecvTimeoutError::Disconnected) => Err("the peer server ended".into()),
        }
    }

    /// Sends one request and returns the values of its `ok` answer.
    fn ask(&mut self, request: &str) -> Result<String, String> {
        writeln!(self.input, "{request}")
            .and_then(|()| self.input.flush())
            .map_err(|e| format!("the peer server takes no request: {e}"))?;
        let answer = self.line(ANSWER_WAIT)?;
        match answer.strip_prefix("ok") {
            Some(values) => Ok(values.trim_start().to_string()),
            None => Err(format!("{request}: {answer}")),
        }
    }
}

/// One game on the upstream engine, behind the peer protocol.
pub struct EnginePeer<'a> {
    d: &'a Data,
    io: RefCell<Io>,
    /// The shared state at the main menu, before the game's first frame of play.
    at_menu: Vec<(CForm, Field)>,
}

fn hex_list(ids: &[u32]) -> String {
    if ids.is_empty() {
        return "-".into();
    }
    let words: Vec<String> = ids.iter().map(|id| format!("{id:08X}")).collect();
    words.join(",")
}

impl<'a> EnginePeer<'a> {
    /// Starts one server at its main menu. `Err` when it does not get there. `draws`: the
    /// game draws its frames on the real graphics device, in a window that does not take the
    /// focus.
    fn start_at_menu(d: &Data, program: &PathBuf, host: bool, draws: bool) -> Result<Io, String> {
        let mut command = Command::new(program);
        if draws {
            command
                .env("NV_D3D9", "real")
                .env("NV_D3D9_BACKGROUND", "1");
        }
        let mut child = command
            .arg(&d.dir)
            .args(["--role", if host { "host" } else { "guest" }, "--menu"])
            // The engine's test clock: each run reads the same times, whatever the load of
            // the machine. With the real clock a loaded machine changes the order of the
            // game's threads, and a run can stop in its first frames.
            .env("NV_REPEATABLE", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            // The game's own log is long. A test that needs it runs the server by hand.
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("{}: {e}", program.display()))?;
        let input = child.stdin.take().ok_or("the peer server has no input")?;
        let output = child.stdout.take().ok_or("the peer server has no output")?;
        let (send, lines) = std::sync::mpsc::channel();
        // A reader thread, so that a server that stops answering ends a test with a reason
        // and not with a hang.
        std::thread::spawn(move || {
            for line in BufReader::new(output).lines() {
                let Ok(line) = line else { break };
                if let Some(text) = line.strip_prefix(MARK) {
                    if send.send(text.to_string()).is_err() {
                        break;
                    }
                }
            }
        });
        let mut io = Io {
            child,
            input,
            lines,
        };
        match io.line(READY_WAIT) {
            Ok(ready) if ready.starts_with("ready") => Ok(io),
            other => {
                let _ = io.child.kill();
                let _ = io.child.wait();
                Err(format!("the peer server did not reach its menu: {other:?}"))
            }
        }
    }

    /// Starts a server on the game whose Data folder `d` has, gives it the catalog of shared
    /// quests and globals, and goes into the game. A guest with `gate` gets its gate at the
    /// main menu, before its first frame of play.
    pub fn start(
        d: &'a Data,
        program: &PathBuf,
        host: bool,
        catalog: &Catalog,
        gate: Option<&Gate>,
        draws: bool,
    ) -> Result<EnginePeer<'a>, String> {
        let mut last = String::new();
        for _ in 0..STARTS {
            let mut io = match Self::start_at_menu(d, program, host, draws) {
                Ok(io) => io,
                Err(why) => {
                    last = why;
                    continue;
                }
            };
            let request = format!(
                "catalog {} {} {}",
                hex_list(&catalog.quests),
                hex_list(&catalog.globals),
                hex_list(&catalog.local_scripts)
            );
            let entered = io.ask(&request).and_then(|_| match gate {
                Some(gate) => {
                    let request = format!(
                        "gate {} {}",
                        hex_list(&gate.quests),
                        hex_list(&gate.scripts)
                    );
                    io.ask(&request)
                }
                None => Ok(String::new()),
            });
            // A gate that the game refuses is a fault, not a start to try again.
            entered?;
            if !host {
                // A guest's own form indexes start above the host's: a record of the host then
                // names the same forms in this game.
                io.ask("join_pairs")?;
            }
            let at_menu = fields_from_hex(&io.ask("export")?)?;
            match io.ask(&format!("enter {CELL}")) {
                Ok(_) => {
                    return Ok(EnginePeer {
                        d,
                        io: RefCell::new(io),
                        at_menu,
                    })
                }
                Err(why) => {
                    last = why;
                    let _ = io.child.kill();
                    let _ = io.child.wait();
                }
            }
        }
        Err(format!(
            "the peer server did not start in {STARTS} tries: {last}"
        ))
    }

    fn form(&self, editor_id: &str) -> Option<u32> {
        self.d.order.form_by_editor_id(editor_id).map(|f| f.0)
    }

    /// An answer's values, or `None` when the server refused or ended (the test then fails
    /// on the missing value).
    fn ask(&self, request: &str) -> Option<String> {
        self.io.borrow_mut().ask(request).ok()
    }

    fn applied(&self, verb: &str, fields: &[(CForm, Field)]) -> (u32, u32) {
        let answer = self.ask(&format!("{verb} {}", fields_to_hex(fields)));
        let mut counts = answer
            .iter()
            .flat_map(|a| a.split_whitespace())
            .map(|w| w.parse().unwrap_or(u32::MAX));
        // An answer that does not come counts as "everything changed": no test takes a
        // silent server for an idempotent apply.
        (
            counts.next().unwrap_or(u32::MAX),
            counts.next().unwrap_or(u32::MAX),
        )
    }
}

impl Drop for EnginePeer<'_> {
    fn drop(&mut self) {
        let io = self.io.get_mut();
        let _ = io.ask("quit");
        // The server ends on `quit` or on its closed input. A server that does neither is
        // stopped, so that no game is left behind a test.
        let _ = io.child.kill();
        let _ = io.child.wait();
    }
}

impl BackendPeer for EnginePeer<'_> {
    fn set_stage(&mut self, quest: &str, stage: u16) -> bool {
        self.form(quest)
            .and_then(|q| self.ask(&format!("set_stage {q:08X} {stage}")))
            .is_some_and(|a| a == "1")
    }
    fn set_stage_as(&mut self, _slot: u8, _quest: &str, _stage: u16) -> bool {
        false
    }
    fn stage(&self, quest: &str) -> Option<u16> {
        let q = self.form(quest)?;
        self.ask(&format!("stage {q:08X}"))?.parse().ok()
    }
    fn player_stage(&self, _slot: u8, _quest: &str) -> Option<u16> {
        None
    }
    fn set_global(&mut self, global: &str, v: f32) -> bool {
        self.form(global)
            .and_then(|g| self.ask(&format!("set_global {g:08X} {v}")))
            .is_some_and(|a| a == "1")
    }
    fn global(&self, global: &str) -> Option<f32> {
        let g = self.form(global)?;
        self.ask(&format!("global {g:08X}"))?.parse().ok()
    }
    fn export(&self) -> Vec<(CForm, Field)> {
        self.ask("export")
            .and_then(|a| fields_from_hex(&a).ok())
            .unwrap_or_default()
    }
    fn shard_hashes(&self) -> ShardHashes {
        let fields = self.export();
        ShardHashes::of_fields(fields.iter().map(|(form, field)| (*form, field)))
    }
    fn apply_snapshot(&mut self, entries: &[SnapEntry]) -> (u32, u32) {
        let fields: Vec<(CForm, Field)> =
            entries.iter().map(|e| (e.form, e.field.clone())).collect();
        self.applied("apply_snapshot", &fields)
    }
    fn apply_delta(&mut self, d: &Delta) -> (u32, u32) {
        let fields: Vec<(CForm, Field)> = d
            .changes
            .iter()
            .map(|c| (c.form, c.field.clone()))
            .collect();
        self.applied("apply", &fields)
    }
    fn register_player(&mut self, _slot: u8) {}
    fn has_player_rows(&self) -> bool {
        false
    }
    fn start_state(&self) -> Option<Vec<(CForm, Field)>> {
        Some(self.at_menu.clone())
    }
    fn frames(&mut self, n: u32) -> bool {
        self.ask(&format!("frames {n}")).is_some()
    }
    fn script_line(&mut self, line: &str) -> bool {
        self.ask(&format!("script {line}")).is_some()
    }
    fn adapter_line(&mut self, line: &str) -> bool {
        self.ask(&format!("console {line}")).is_some()
    }
    fn gate_stats(&self) -> Option<GateStats> {
        let answer = self.ask("gate_stats")?;
        let value = |name: &str| {
            answer
                .split_whitespace()
                .find_map(|w| w.strip_prefix(name)?.strip_prefix('='))
        };
        Some(GateStats {
            runs: value("runs")?.parse().ok()?,
            stopped: value("stopped")?.parse().ok()?,
            guarded: value("guarded")?.parse().ok()?,
            blind: value("blind")? == "1",
        })
    }
    fn intents(&mut self) -> Vec<String> {
        self.ask("intents")
            .map(|a| a.split_whitespace().map(str::to_owned).collect())
            .unwrap_or_default()
    }
    fn set_moving(&mut self, limits: &[(&str, f32)]) -> bool {
        let words: Option<Vec<String>> = limits
            .iter()
            .map(|(global, limit)| Some(format!("{:08X}:{limit}", self.form(global)?)))
            .collect();
        words.is_some_and(|w| {
            let list = if w.is_empty() {
                "-".into()
            } else {
                w.join(",")
            };
            self.ask(&format!("moving {list}")).is_some()
        })
    }
    fn audit(&mut self, restore: bool) -> Option<AuditReport> {
        let answer = self.ask(if restore { "audit" } else { "audit report" })?;
        let mut report = AuditReport::default();
        for word in answer.split_whitespace() {
            if let Some(n) = word.strip_prefix("leaks=") {
                report.leaks = n.parse().ok()?;
            } else if let Some(n) = word.strip_prefix("restored=") {
                report.restored = n.parse().ok()?;
            } else if let Some(form) = word.strip_prefix("last=") {
                report.last_script = u32::from_str_radix(form, 16).ok()?;
            } else if let Some((form, kind)) = word.split_once(':') {
                report
                    .named
                    .push((u32::from_str_radix(form, 16).ok()?, kind.parse().ok()?));
            }
        }
        Some(report)
    }
    fn local_stage(&self, quest: &str) -> Option<u16> {
        let q = self.form(quest)?;
        self.ask(&format!("local_stage {q:08X}"))?.parse().ok()
    }
    fn avatar_make(
        &mut self,
        slot: u8,
        female: bool,
        race: u32,
        place: [f32; 3],
        heading: f32,
    ) -> Result<u32, String> {
        let request = format!(
            "avatar_make {slot} {} {race:08X} {} {} {} {heading}",
            u8::from(female),
            place[0],
            place[1],
            place[2]
        );
        let id = self.io.borrow_mut().ask(&request)?;
        u32::from_str_radix(&id, 16).map_err(|_| format!("a bad form id {id}"))
    }
    fn avatar_drive(
        &mut self,
        slot: u8,
        place: [f32; 3],
        heading: f32,
        flags: u32,
    ) -> Result<(), String> {
        let request = format!(
            "avatar_drive {slot} {} {} {} {heading} {flags:X}",
            place[0], place[1], place[2]
        );
        self.io.borrow_mut().ask(&request).map(|_| ())
    }
    fn avatar_state(&self, slot: u8) -> Option<AvatarState> {
        let answer = self.ask(&format!("avatar_state {slot}"))?;
        let words: Vec<&str> = answer.split_whitespace().collect();
        let float = |i: usize| words.get(i)?.parse::<f32>().ok();
        let hex = |i: usize| u32::from_str_radix(words.get(i)?, 16).ok();
        Some(AvatarState {
            place: [float(0)?, float(1)?, float(2)?],
            heading: float(3)?,
            life: words.get(4)?.parse().ok()?,
            group: hex(5)?,
            flags: hex(6)?,
            cell: hex(7)?,
        })
    }
    fn avatar_look(&self, slot: u8, race: u32) -> Option<(bool, bool)> {
        let answer = self.ask(&format!("avatar_look {slot} {race:08X}"))?;
        let mut words = answer.split_whitespace();
        Some((words.next()? == "1", words.next()? == "1"))
    }
    fn avatar_remove(&mut self, slot: u8) -> bool {
        self.ask(&format!("avatar_remove {slot}")).is_some()
    }
    fn avatar_remote(
        &mut self,
        slot: u8,
        at: Option<(u32, [f32; 3], f32, u32)>,
    ) -> Result<String, String> {
        let request = match at {
            Some((cell, place, heading, flags)) => format!(
                "remote {slot} {cell:08X} {} {} {} {heading} {flags:X}",
                place[0], place[1], place[2]
            ),
            None => format!("remote {slot} gone"),
        };
        self.io.borrow_mut().ask(&request)
    }
    fn player(&self) -> Option<(u32, [f32; 3], f32)> {
        let answer = self.ask("player")?;
        let words: Vec<&str> = answer.split_whitespace().collect();
        let float = |i: usize| words.get(i)?.parse::<f32>().ok();
        Some((
            u32::from_str_radix(words.first()?, 16).ok()?,
            [float(1)?, float(2)?, float(3)?],
            float(4)?,
        ))
    }
    fn avatar_guard(&mut self) -> Result<(), String> {
        self.io.borrow_mut().ask("avatar_guard").map(|_| ())
    }
    fn avatar_hits(&mut self) -> Option<(u64, Vec<AvatarHit>)> {
        let answer = self.ask("avatar_hits")?;
        let mut words = answer.split_whitespace();
        let kills = words.next()?.parse().ok()?;
        let hits: Option<Vec<AvatarHit>> = words
            .map(|word| {
                let mut parts = word.split(':');
                Some(AvatarHit {
                    slot: parts.next()?.parse().ok()?,
                    source: u32::from_str_radix(parts.next()?, 16).ok()?,
                    weapon: u32::from_str_radix(parts.next()?, 16).ok()?,
                    damage: parts.next()?.parse().ok()?,
                })
            })
            .collect();
        Some((kills, hits?))
    }
    fn avatar_health(&self, slot: u8) -> Option<f64> {
        self.ask(&format!("avatar_health {slot}"))?.parse().ok()
    }
    fn avatar_heal(&mut self, slot: u8) -> bool {
        self.ask(&format!("avatar_heal {slot}")).is_some()
    }
    fn has_form(&self, id: u32) -> bool {
        self.ask(&format!("form {id:08X}"))
            .is_some_and(|a| a == "1")
    }
    fn form_word(&self, id: u32, offset: u32) -> Option<u32> {
        let answer = self.ask(&format!("form_word {id:08X} {offset:X}"))?;
        u32::from_str_radix(&answer, 16).ok()
    }
    fn node_place(&self, id: u32) -> Option<[f32; 3]> {
        let answer = self.ask(&format!("node_place {id:08X}"))?;
        let mut words = answer.split_whitespace().map(str::parse::<f32>);
        Some([
            words.next()?.ok()?,
            words.next()?.ok()?,
            words.next()?.ok()?,
        ])
    }
    fn enter(&mut self, cell: &str) -> bool {
        self.ask(&format!("enter {cell}")).is_some()
    }
    fn changes_baseline(&mut self) -> Option<u32> {
        self.ask("changes baseline")?.parse().ok()
    }
    fn changes(&mut self) -> Option<Vec<(u32, u32)>> {
        let answer = self.ask("changes")?;
        answer
            .split_whitespace()
            .skip(1)
            .map(|word| {
                let (form, new) = word.split_once(':')?;
                Some((
                    u32::from_str_radix(form, 16).ok()?,
                    u32::from_str_radix(new, 16).ok()?,
                ))
            })
            .collect()
    }
    fn record(&mut self, form: u32) -> Option<ChangeRecord> {
        let answer = self.ask(&format!("record {form:08X}"))?;
        let words: Vec<&str> = answer.split_whitespace().collect();
        let hex = |i: usize| u32::from_str_radix(words.get(i)?, 16).ok();
        Some(ChangeRecord {
            form,
            flags: hex(0)?,
            kind: hex(1)?,
            version: hex(2)?,
            pairs: hex(3)?,
            bytes: (*words.get(4)?).to_string(),
        })
    }
    fn index_pairs(&self, from: u32) -> Vec<(u32, u32)> {
        let pair = |word: &str| {
            let (index, form) = word.split_once(':')?;
            Some((
                u32::from_str_radix(index, 16).ok()?,
                u32::from_str_radix(form, 16).ok()?,
            ))
        };
        self.ask(&format!("pairs {from:X}"))
            .map(|a| a.split_whitespace().filter_map(pair).collect())
            .unwrap_or_default()
    }
    fn take_pairs(&mut self, pairs: &[(u32, u32)]) -> Option<u32> {
        if pairs.is_empty() {
            return Some(0);
        }
        let words: Vec<String> = pairs
            .iter()
            .map(|(index, form)| format!("{index:X}:{form:08X}"))
            .collect();
        let answer = self.ask(&format!("take_pairs {}", words.join(",")))?;
        answer.split_whitespace().next()?.parse().ok()
    }
    fn take_record(&mut self, r: &ChangeRecord) -> Result<bool, String> {
        let request = format!(
            "take_record {:08X} {:X} {:X} {:X} {:X} {}",
            r.form, r.flags, r.kind, r.version, r.pairs, r.bytes
        );
        match self.io.borrow_mut().ask(&request)?.as_str() {
            "loaded" => Ok(true),
            "waits" => Ok(false),
            other => Err(format!("an unknown answer {other}")),
        }
    }
}

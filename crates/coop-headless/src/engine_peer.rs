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

use crate::backend::{AuditReport, BackendPeer, GateStats};
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
    /// Starts one server at its main menu. `Err` when it does not get there.
    fn start_at_menu(d: &Data, program: &PathBuf, host: bool) -> Result<Io, String> {
        let mut child = Command::new(program)
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
    ) -> Result<EnginePeer<'a>, String> {
        let mut last = String::new();
        for _ in 0..STARTS {
            let mut io = match Self::start_at_menu(d, program, host) {
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
}

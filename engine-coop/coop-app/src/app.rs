//! The program's loop: one frame of the game, with the session before it and after it.
//!
//! ```text
//! control requests (each one between two frames)
//! own { session.pre_frame_game;  guest: session.start_poll_game }
//! the cell entry, the timed lines
//! the game's frame
//! guest in play: the gate's frame count
//! own { guest: each gate intent -> a command for the host;  session.post_frame_game }
//! the session's events, the state dump
//! ```
//!
//! `own { }` is the adapter's bracket: what the program runs for the session is not gated.
//! A frame that shows a loading screen runs `session.poll` in place of the two session
//! calls: the network must not wait for a cell.

use std::collections::VecDeque;
use std::net::SocketAddr;
use std::sync::mpsc::Receiver;
use std::sync::Arc;

use coop_core::ids::FormId;
use coop_core::transport::{ChaosProfile, Transport};
use coop_engine::console;
use coop_engine::files::File;
use coop_engine::game::{resolve_policy, EngineCoop, EngineState, NameTable};
use coop_game::{Intent, Names, Players, Shared, Story};
use coop_session::control::{self, Cmd, LiveStats, Request};
use coop_session::json::V;
use coop_session::link::{self, LinkRequest};
use coop_session::{
    Command, CompatConfig, Event, OwnCharacter, Role, Session, SessionConfig, StartConfig,
};

use crate::args::Args;
use crate::dump;
use crate::intents;
use crate::machine::{Machine, Screen};
use crate::names;

/// The version of the game program that this adapter knows.
pub const EXE_VERSION: &str = "1.4.0.525";

/// How long a host keeps a guest that is silent, in milliseconds. One frame of the game
/// loads a whole cell on this engine (measured: 9 seconds for the first cell with two games
/// on one PC), and the program cannot poll the network inside a frame. The transport itself
/// drops a peer after 20 seconds of silence, so a longer time here has no effect.
const PEER_TIMEOUT_MS: u64 = 20_000;

/// The state dump is written again when the state changed, and not more often than this.
const DUMP_EVERY_S: f64 = 1.0;

/// What the caller does after a step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flow {
    /// Run the next step.
    Go,
    /// The program ends, for this reason.
    Quit(&'static str),
    /// The game's main does not run any more.
    Ended,
}

/// FNV-1a, as each program of the session contract hashes its build name.
fn fnv(bytes: impl IntoIterator<Item = u8>) -> u64 {
    bytes.into_iter().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// One player's program: the game, the session, and what the command line asked for.
pub struct App<M: Machine> {
    /// The game.
    pub machine: M,
    /// The co-op session.
    pub session: Session,
    state: EngineState,
    args: Args,
    host: bool,
    /// The cell entry was asked for.
    entered: bool,
    /// The game reached play after the cell entry.
    in_play: bool,
    /// The cell that could not be entered, logged one time.
    no_cell: Option<u32>,
    /// Seconds since the program started.
    t: f64,
    frames: u64,
    /// When the session was ready with the game in play.
    ready_at: Option<f64>,
    /// When the timed lines started: at `ready_at`, or later when the go file came.
    go_at: Option<f64>,
    lines: VecDeque<(f32, String)>,
    stages: Vec<(String, u16)>,
    control: Option<Receiver<Request>>,
    control_addr: Option<SocketAddr>,
    quit: Option<&'static str>,
    dump_seq: u64,
    dump_key: Option<(u64, Option<u64>, bool)>,
    dump_at: f64,
    /// The intents that went to the host, and those that had no command.
    intents: (u64, u64),
    blind: bool,
    fps: f32,
}

impl<M: Machine> App<M> {
    /// The program for this command line on this game. The game is at its main menu.
    ///
    /// `names`: the names of the game's data, for the loaded `files`. `transport`: a ready
    /// transport (a test), or `None` to open the link that the command line asks for.
    pub fn new(
        args: Args,
        machine: M,
        names: NameTable,
        files: &[File],
        transport: Option<Box<dyn Transport>>,
    ) -> Result<App<M>, String> {
        let host = args.is_host();
        let role = if host { Role::Host } else { Role::Guest };
        let bundled = coop_policy::PolicyTable::bundled().map_err(|e| format!("{e}"))?;
        let policy = resolve_policy(&bundled, &names)?;
        let name = args.name.clone().unwrap_or_else(|| {
            if host {
                "Host".into()
            } else {
                format!("Guest {}", std::process::id() % 1000)
            }
        });
        let addr = args
            .host
            .clone()
            .or_else(|| args.join.clone())
            .unwrap_or_default();
        let chaos = match &args.net {
            Some(profile) => Some((
                ChaosProfile::by_name(profile)
                    .ok_or_else(|| format!("--coop-net: no fault profile {profile}"))?,
                0x5eed ^ u64::from(std::process::id()),
            )),
            None => None,
        };
        let mut cfg = SessionConfig {
            name: name.clone(),
            addr: addr.clone(),
            build_hash: fnv(concat!("nv-coop ", env!("CARGO_PKG_VERSION")).bytes()),
            mods_hash: names::mods_hash(files),
            chaos,
            log: args.log.clone(),
            peer_timeout_ms: PEER_TIMEOUT_MS,
            policy: Some(Arc::new(policy.clone())),
            compat: CompatConfig {
                exe: EXE_VERSION.into(),
                volatile_hash: coop_session::compat::volatile_hash(
                    coop_session::compat::VOLATILE_KEYS,
                ),
                plugins: files.iter().map(|f| f.name.clone()).collect(),
                allow_mismatch: args.allow_mismatch,
            },
            ..SessionConfig::default()
        };
        let mut notices = Vec::new();
        match transport {
            Some(transport) => cfg.transport = Some(transport),
            None => {
                let mut request = LinkRequest::new(role, &name, &addr);
                request.test_token = args.test_token;
                request.token = args.token;
                let opened = link::open(&request).map_err(|e| format!("{e}"))?;
                for line in &opened.lines {
                    println!("{line}");
                }
                cfg.transport = Some(opened.transport);
                notices = opened.notices;
            }
        }
        let mut session = match role {
            Role::Host => Session::host(cfg),
            Role::Guest => Session::join(cfg),
        }
        .map_err(|e| format!("{e}"))?;
        for notice in &notices {
            let fields: Vec<(&str, V)> = notice
                .fields
                .iter()
                .map(|(k, v)| (*k, V::S(v.as_str())))
                .collect();
            session.log_event(notice.ev, &fields);
        }
        // The command line needs `--coop-no-chargen` for a guest: no character making yet.
        session.set_start_config(StartConfig {
            new_game: false,
            own: OwnCharacter::Skip,
        });
        for flag in &args.no_effect {
            session.log_event("flag_no_effect", &[("flag", V::S(flag))]);
        }
        let (control, control_addr) = match args.control {
            Some(port) => {
                let (requests, at) = control::serve(port)?;
                session.log_event("control", &[("port", V::U(u64::from(at.port())))]);
                (Some(requests), Some(at))
            }
            None => (None, None),
        };
        Ok(App {
            machine,
            session,
            state: EngineState::new(names).with_policy(policy),
            host,
            entered: false,
            in_play: false,
            no_cell: None,
            t: 0.0,
            frames: 0,
            ready_at: None,
            go_at: None,
            lines: args.lines.iter().cloned().collect(),
            stages: args.stages.clone(),
            control,
            control_addr,
            quit: None,
            dump_seq: 0,
            dump_key: None,
            dump_at: 0.0,
            intents: (0, 0),
            blind: false,
            fps: 0.0,
            args,
        })
    }

    /// Where the control port listens, when the command line asked for one.
    pub fn control_addr(&self) -> Option<SocketAddr> {
        self.control_addr
    }

    /// True when the game is in play and the session is ready: the scenario can start.
    pub fn ready(&self) -> bool {
        self.ready_at.is_some()
    }

    /// The count of frames that the game ran.
    pub fn frames(&self) -> u64 {
        self.frames
    }

    /// The adapter's state: the names and the role.
    pub fn state(&self) -> &EngineState {
        &self.state
    }

    /// The game's shared state now.
    pub fn export(&mut self) -> Vec<(FormId, coop_core::schema::Field)> {
        EngineCoop::over(&mut self.machine, &mut self.state).fields()
    }

    /// One frame of the program. `dt`: the real time since the last step, in seconds.
    pub fn step(&mut self, dt: f32) -> Flow {
        self.t += f64::from(dt);
        if dt > 0.0 {
            self.fps += (1.0 / dt - self.fps) * 0.1;
        }
        self.control_requests();
        let screen = self.machine.screen();
        if self.entered && screen == Screen::Play {
            self.in_play = true;
        }
        // A host's game takes part when it plays. A guest's game takes part at the main
        // menu too: it gets its gate there, before its first frame of play.
        let with_game = match (self.host, self.entered) {
            (true, false) => false,
            (false, false) => screen == Screen::Menu,
            (_, true) => self.in_play && screen != Screen::Loading,
        };
        if with_game {
            if self.state.role().is_none() {
                self.take_role();
            }
            self.machine.own(true);
            let mut coop = EngineCoop::over(&mut self.machine, &mut self.state);
            self.session.pre_frame_game(&mut coop, true);
            if !self.host {
                self.session.start_poll_game(&coop, false);
            }
            self.machine.own(false);
        }
        if !self.entered && screen == Screen::Menu {
            self.enter_start_cell();
        }
        self.timed_lines(screen);

        if !self.machine.frame() {
            return Flow::Ended;
        }
        self.frames += 1;

        if !self.host && self.machine.screen() == Screen::Play {
            let blind = self.machine.gate_frame((dt * 1000.0) as u32);
            if blind && !self.blind {
                self.session.log_event(
                    "error",
                    &[("msg", V::S("the guest gate saw no script for too long"))],
                );
            }
            self.blind = blind;
        }
        if with_game {
            if !self.host {
                self.send_intents();
            }
            self.machine.own(true);
            let mut coop = EngineCoop::over(&mut self.machine, &mut self.state);
            self.session.post_frame_game(&mut coop, dt);
            self.machine.own(false);
        } else {
            self.session.poll(dt);
        }
        self.events();
        self.dump_when_due(false);
        self.quit_checks()
    }

    /// Gives the game its role before the session's first call on it: the catalog of shared
    /// state, and a guest's gate. This work can take seconds on a real game. The session
    /// tells the same role each frame after it, and that costs nothing then.
    fn take_role(&mut self) {
        let started = std::time::Instant::now();
        let setup = coop_game::Setup {
            role: if self.host {
                coop_game::Role::Host
            } else {
                coop_game::Role::Guest
            },
            me: self.session.status().player.0,
            policy: None,
            keep_own_rows: false,
        };
        self.machine.own(true);
        let done = EngineCoop::over(&mut self.machine, &mut self.state).configure(&setup);
        self.machine.own(false);
        let ms = started.elapsed().as_millis() as u64;
        match done {
            Ok(()) => self.session.log_event("role", &[("ms", V::U(ms))]),
            Err(why) => {
                let msg = format!("the game did not take its role: {why}");
                self.session.log_event("error", &[("msg", V::S(&msg))]);
                // A guest with no gate must not play in the session.
                self.quit = Some("no_role");
            }
        }
    }

    /// Asks the game to enter the start cell: a host's own cell, or the host's cell for a
    /// guest that has its gate.
    fn enter_start_cell(&mut self) {
        let cell = if self.host {
            self.args.cell.clone()
        } else {
            // The gate comes with the role. No frame of play runs before it.
            if self.state.role().is_none() {
                return;
            }
            let Some(place) = self.session.host_place() else {
                return;
            };
            let name = self.state.names.name_of(FormId(place.cell));
            if name.is_none() && self.no_cell != Some(place.cell) {
                self.no_cell = Some(place.cell);
                let msg = format!(
                    "the host's cell {:08X} has no editor id: this program cannot enter it yet",
                    place.cell
                );
                self.session.log_event("error", &[("msg", V::S(&msg))]);
            }
            name
        };
        let Some(cell) = cell else { return };
        self.machine.own(true);
        let ran = console::run_line(&mut self.machine, &format!("coc {cell}"), 0);
        self.machine.own(false);
        if ran {
            self.entered = true;
            self.session
                .log_event("enter_cell", &[("cell", V::S(&cell))]);
        }
    }

    /// Runs the lines and the stages of the command line whose time came. They wait for
    /// the session, and for the go file when the command line names one.
    fn timed_lines(&mut self, screen: Screen) {
        // The game reached play, and the session is ready. A menu of the game's own (a
        // dialogue that the story starts) does not hold the scenario.
        if self.ready_at.is_none()
            && self.in_play
            && screen != Screen::Loading
            && self.session.is_ready()
        {
            self.ready_at = Some(self.t);
        }
        if self.ready_at.is_none() {
            return;
        }
        if self.go_at.is_none() {
            let go = self.args.go_file.as_ref().map_or(true, |f| f.exists());
            if !go {
                return;
            }
            self.go_at = Some(self.t);
            for (quest, stage) in std::mem::take(&mut self.stages) {
                self.run(Command::Stage(quest, stage), false);
            }
        }
        let since = (self.t - self.go_at.unwrap_or(self.t)) as f32;
        while self.lines.front().is_some_and(|(at, _)| *at <= since) {
            if let Some((_, line)) = self.lines.pop_front() {
                self.run(Command::Line(line), false);
            }
        }
    }

    /// Runs a command: on the host's game with its own code, or from a guest through the
    /// host. `local`: on this game also when it is a guest's game (a test's own setup).
    /// Returns how it ran: `host`, `sent` or `local`.
    fn run(&mut self, command: Command, local: bool) -> &'static str {
        let line = command.as_line();
        self.session.log_event("script", &[("line", V::S(&line))]);
        if !self.host && !local {
            self.session.send_command(command);
            return "sent";
        }
        self.machine.own(true);
        let mut coop = EngineCoop::over(&mut self.machine, &mut self.state);
        let answer = if self.host {
            let intent = match &command {
                Command::Line(line) => Some(Intent::ScriptLine(line.clone())),
                Command::Stage(quest, stage) => coop.form_of(quest).map(|quest| Intent::SetStage {
                    quest,
                    stage: *stage,
                }),
            };
            match intent {
                Some(intent) => format!("{:?}", coop.run_intent(0, &intent)),
                None => "no such quest".into(),
            }
        } else {
            // A guest's game refuses each intent. A local line is the test's own work on
            // this game: it runs as a console line, inside the adapter's bracket.
            let line = self.state.names.line_for_program(&line);
            format!("{}", console::run_line(&mut self.machine, &line, 0))
        };
        self.machine.own(false);
        self.session.log_event(
            "script_ran",
            &[("line", V::S(&line)), ("answer", V::S(&answer))],
        );
        if self.host {
            "host"
        } else {
            "local"
        }
    }

    /// Sends each change that this guest's game wanted to make to the host.
    fn send_intents(&mut self) {
        for intent in self.machine.take_intents() {
            let text = format!("{intent:?}");
            match intents::commands(&intent, &self.state.names) {
                Ok(commands) => {
                    self.intents.0 += 1;
                    self.session.log_event("intent", &[("what", V::S(&text))]);
                    for command in commands {
                        self.session.send_command(command);
                    }
                }
                Err(why) => {
                    self.intents.1 += 1;
                    self.session
                        .log_event("intent_open", &[("what", V::S(&text)), ("why", V::S(&why))]);
                }
            }
        }
    }

    /// Notes what the session told.
    fn events(&mut self) {
        for event in self.session.drain_events() {
            if let Some(message) = event.message() {
                println!("{message}");
            }
            if let Event::Rejected(_) = event {
                self.quit = Some("rejected");
            }
        }
    }

    /// The state dump as text, with the game's state now.
    pub fn dump_text(&mut self) -> String {
        let (export, local) = {
            let mut coop = EngineCoop::over(&mut self.machine, &mut self.state);
            (coop.fields(), coop.local_player())
        };
        let status = self.session.status();
        let points = self.session.hash_points();
        dump::state_json(&dump::View {
            host: self.host,
            slot: status.player.0,
            t: self.t,
            seq: self.dump_seq,
            ready: self.ready(),
            export: &export,
            names: &self.state.names,
            cell: local.place.cell,
            place: local.place.pos,
            rev: points.rev,
            recent: points.recent,
            gate: self.machine.counters(),
            intents: self.intents,
            session: format!(
                "{{\"state\":\"{:?}\",\"rev\":{},\"desync\":{},\"desyncs\":{},\"repairs\":{},\"snapshots\":{},\"pending_commands\":{},\"decode_errors\":{},\"peers\":{}}}",
                status.state,
                status.rev,
                status.desync,
                status.desyncs,
                status.repairs,
                status.snapshots,
                status.pending_commands,
                status.decode_errors,
                status.peers.len()
            ),
        })
    }

    /// Writes the state dump when the state changed since the last one, at most one time
    /// in [`DUMP_EVERY_S`]. `now`: write it in each case.
    fn dump_when_due(&mut self, now: bool) {
        let Some(path) = self.args.state_dump.clone() else {
            return;
        };
        if !now && (!self.in_play || self.t - self.dump_at < DUMP_EVERY_S) {
            return;
        }
        self.dump_at = self.t;
        let export = self.export();
        let key = (
            dump::shared_root(&export),
            self.session.hash_points().rev,
            self.ready(),
        );
        if !now && self.dump_key == Some(key) {
            return;
        }
        self.dump_key = Some(key);
        self.dump_seq += 1;
        let text = self.dump_text();
        // A reader never sees a part of a dump: the file changes in one step.
        let part = path.with_extension("tmp");
        let written = std::fs::write(&part, text).and_then(|()| std::fs::rename(&part, &path));
        match written {
            Ok(()) => self.session.log_event(
                "state_dump",
                &[
                    ("path", V::S(&path.display().to_string())),
                    ("seq", V::U(self.dump_seq)),
                ],
            ),
            Err(e) => {
                let msg = format!("the state dump {}: {e}", path.display());
                self.session.log_event("error", &[("msg", V::S(&msg))]);
            }
        }
    }

    /// Answers each request of the control port.
    fn control_requests(&mut self) {
        let mut requests = Vec::new();
        if let Some(port) = &self.control {
            while let Ok(request) = port.try_recv() {
                requests.push(request);
            }
        }
        for request in requests {
            let name = request.cmd.name();
            let reply = match request.cmd {
                Cmd::State => {
                    let dump = self.dump_text();
                    let cell = {
                        let coop = EngineCoop::over(&mut self.machine, &mut self.state);
                        coop.local_player().place.cell
                    };
                    let cell = cell
                        .map(|c| {
                            self.state
                                .names
                                .name_of(c)
                                .unwrap_or_else(|| format!("{:08X}", c.0))
                        })
                        .unwrap_or_default();
                    control::state_reply(
                        &dump,
                        &LiveStats {
                            cell,
                            fps: self.fps,
                            t: self.t as f32,
                            host: self.host,
                            ready: self.ready(),
                            rows: Vec::new(),
                            counters: [0; 4],
                        },
                    )
                }
                Cmd::Entities => control::entities_reply(&[]),
                Cmd::Run { line, local } => {
                    let (line, local) = match line.strip_prefix("local ") {
                        Some(rest) => (rest.to_string(), true),
                        None => (line, local),
                    };
                    let how = self.run(Command::Line(line), local);
                    control::ok(name, &format!("\"how\":\"{how}\""))
                }
                Cmd::Quit => {
                    self.quit = Some("control");
                    control::ok(name, "")
                }
                Cmd::Bot { .. } | Cmd::Keys { .. } | Cmd::Screenshot { .. } => control::fail(
                    name,
                    "this request is not built in this program yet (the game on the upstream engine)",
                ),
            };
            let _ = request.reply.send(reply);
        }
    }

    /// The reasons to end: a request, the quit file, the time after ready.
    fn quit_checks(&mut self) -> Flow {
        if self.quit.is_none() && self.args.quit_file.as_ref().is_some_and(|f| f.exists()) {
            self.quit = Some("quit_file");
        }
        if let (None, Some(after), Some(ready)) = (self.quit, self.args.quit_after, self.ready_at) {
            if self.t - ready >= f64::from(after) {
                self.quit = Some("quit_after");
            }
        }
        match self.quit {
            Some(reason) => {
                self.dump_when_due(true);
                self.session.log_event(
                    "shutdown",
                    &[("reason", V::S(reason)), ("clean", V::B(true))],
                );
                Flow::Quit(reason)
            }
            None => Flow::Go,
        }
    }
}

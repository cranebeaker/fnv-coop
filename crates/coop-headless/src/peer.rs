//! One simulated viewer: a `GameState`, a `Session`, and the per-frame glue
//! of `viewer/src/coop.rs` and `viewer/src/coop_live/*` without Bevy.
//!
//! Frame order (as the viewer's): receive, the game's scripts and movement,
//! the bot and the live intents, the live frame, then collect/send, events,
//! guest commands and the dump.

use std::collections::BTreeSet;

use coop_session::dump::state_json_full;
use coop_session::{Command, Event as CoopEvent, LiveCounters, LiveView, Role, Session};
use esm::FormId;
use world::coop::apply::{Ctx, Opts};
use world::scripting::{Event, GameState, Runner};

use crate::bot::{self, Out, TalkBot, READ_SECONDS};
use crate::data::{Data, Place};
use crate::json::{n, obj, s, J};
use crate::scenario::{Action, Side};

/// Units a second a held movement key moves the player: the viewer's Always Run default (walk 77 x fMoveRunMult 4); the fake viewer of tools/coop-run uses 150.
pub const RUN_SPEED: f32 = 308.0;

/// A peer's scripted lines, split as the viewer splits them at ready.
#[derive(Default)]
struct Held {
    stage: Option<(String, u16)>,
    run: Vec<String>,
    run_at: Vec<(f64, String)>,
}

/// One simulated viewer process.
pub struct Peer {
    /// Host or guest.
    pub role: Role,
    /// Session.
    pub sess: Session,
    /// Game state.
    pub gs: GameState,
    /// Physical player feet.
    pub pos: [f32; 3],
    /// Heading, radians.
    pub heading: f32,
    /// Counters for the dump.
    pub counters: LiveCounters,
    /// The log, as the viewer would write it.
    pub events: Vec<J>,
    /// Stand-ins this peer used (viewer behaviour it replaces).
    pub modelled: BTreeSet<String>,
    /// `coop_ready` time.
    pub ready_at: Option<f64>,
    /// Errors logged.
    pub errors: Vec<String>,
    /// Repairs logged.
    pub repairs: u32,
    /// Last live view.
    pub view: Option<LiveView>,
    fixed_glue: bool,
    held: Held,
    start_cmds: Vec<String>,
    start_stage: Option<(String, u16)>,
    later: Vec<(f64, String)>,
    guest_timed: Vec<(f64, String)>,
    bot: Vec<(f64, String)>,
    keys: Vec<(f64, String, f64)>,
    talk: Option<f64>,
    talker: TalkBot,
    greeting: Option<FormId>,
    people: Vec<(FormId, FormId)>,
    tracked: BTreeSet<FormId>,
    next_track: f64,
    spectate: Vec<f64>,
    last_dump: f64,
    seq: u64,
    dump_text: String,
}

fn is_local(l: &str) -> Option<&str> {
    l.strip_prefix("local ")
}

impl Peer {
    /// A peer standing at the cell's arrival point with `side`'s script.
    pub fn new(role: Role, sess: Session, d: &Data, place: &Place, side: &Side, early_talk: bool, fixed_glue: bool) -> Peer {
        let mut gs = GameState::new(&d.order);
        gs.player_cell = Some(place.cell);
        let mut p = Peer {
            role,
            sess,
            gs,
            pos: place.spawn,
            heading: place.heading,
            counters: LiveCounters::default(),
            events: Vec::new(),
            modelled: BTreeSet::new(),
            ready_at: None,
            errors: Vec::new(),
            repairs: 0,
            view: None,
            fixed_glue,
            held: Held::default(),
            start_cmds: Vec::new(),
            start_stage: None,
            later: Vec::new(),
            guest_timed: Vec::new(),
            bot: Vec::new(),
            keys: Vec::new(),
            talk: None,
            talker: TalkBot::default(),
            greeting: None,
            people: place.actors.clone(),
            tracked: BTreeSet::new(),
            next_track: 0.0,
            spectate: Vec::new(),
            last_dump: -1.0,
            seq: 0,
            dump_text: String::new(),
        };
        let name = if role == Role::Host { "host" } else { "guest" };
        p.log(0.0, "start", vec![("role", s(name))]);
        p.log(0.0, "loaded", vec![("cell", s(place.cell.to_string()))]);
        for l in &side.script {
            match &l.action {
                Action::Run(r) => match l.at {
                    Some(at) => p.held.run_at.push((at, r.clone())),
                    None => p.held.run.push(r.clone()),
                },
                Action::Stage(q, st) => match l.at {
                    Some(at) => p.held.run_at.push((at, format!("SetStage {q} {st}"))),
                    None => p.held.stage = Some((q.clone(), *st)),
                },
                Action::Key(k, hold) => p.keys.push((l.at.unwrap_or(0.0), k.clone(), *hold)),
                Action::Talk => p.talk = Some(if early_talk { 0.0 } else { f64::INFINITY }),
                Action::Choose(_) | Action::Say(_) => {
                    p.modelled.insert("--choose/--say: the stand-in menu picks the greeting only".into());
                }
            }
        }
        p
    }

    fn log(&mut self, t: f64, ev: &str, f: Vec<(&str, J)>) {
        let mut v = vec![("t", n((t * 1000.0).round() / 1000.0)), ("ev", s(ev))];
        v.extend(f);
        self.events.push(obj(v));
    }

    fn line(&mut self, t: f64, l: &str) {
        self.log(t, "script", vec![("line", s(l))]);
    }

    /// The cell's people the host replicates (the viewer's `actors::track`).
    fn track(&mut self, t: f64, d: &Data, cx: Ctx) {
        if self.role != Role::Host || t < self.next_track {
            return;
        }
        self.next_track = t + 1.0;
        let new: Vec<FormId> = self
            .people
            .iter()
            .map(|p| p.0)
            .filter(|r| !self.tracked.contains(r) && world::enabled_now(&d.order, *r, &self.gs.disabled) && self.gs.place(&d.order, *r).is_some_and(|p| Some(p.1) == self.gs.player_cell))
            .collect();
        if !new.is_empty() {
            self.sess.live_track(cx, &self.gs, &new);
            self.tracked.extend(new);
        }
    }

    fn run_line(&mut self, d: &Data, line: &str) {
        let mut r = Runner::new(&d.order, &d.scripts, &mut self.gs);
        r.run_source(line, None, None);
        r.run_item_scripts();
    }

    /// Splits the held lines at `coop_ready` like `coop::split_special`.
    fn on_ready(&mut self, t: f64, d: &Data) {
        self.ready_at = Some(t);
        let name = if self.role == Role::Host { "host" } else { "guest" };
        self.log(t, "coop_ready", vec![("role", s(name))]);
        let mut local: Vec<String> = Vec::new();
        let mut host_run: Vec<String> = Vec::new();
        for l in std::mem::take(&mut self.held.run) {
            if bot::is_bot_line(&l) {
                self.bot.push((t, l));
            } else if let Some(rest) = is_local(&l) {
                local.push(rest.to_string());
            } else {
                host_run.push(l);
            }
        }
        let mut timed: Vec<(f64, String)> = Vec::new();
        for (at, l) in std::mem::take(&mut self.held.run_at) {
            if bot::is_bot_line(&l) {
                self.bot.push((t + at, l));
            } else if let Some(rest) = is_local(&l) {
                self.later.push((at, rest.to_string()));
            } else {
                timed.push((at, l));
            }
        }
        self.start_cmds.extend(local);
        if let Some(tk) = self.talk.as_mut().filter(|tk| tk.is_infinite()) {
            *tk = t + 1.5;
            self.modelled.insert("--talk: a stand-in opens the dialogue with the nearest greeter and picks the greeting".into());
        }
        if self.role == Role::Host {
            if let Some(st) = self.held.stage.take() {
                self.line(t, &format!("SetStage {} {}", st.0, st.1));
                self.start_stage = Some(st);
            }
            for l in host_run {
                self.line(t, &l);
                self.start_cmds.push(l);
            }
            self.later.extend(timed);
        } else {
            if let Some((q, st)) = self.held.stage.take() {
                self.line(t, &format!("SetStage {q} {st}"));
                self.sess.send_command(Command::Stage(q, st));
            }
            for l in host_run {
                self.line(t, &l);
                self.sess.send_command(Command::Line(l));
            }
            self.guest_timed = timed;
        }
        let _ = d;
    }

    /// One viewer frame at virtual time `t`.
    pub fn frame(&mut self, t: f64, dt: f32, d: &Data, cx: Ctx) {
        self.sess.pre_frame_with(&mut self.gs, Opts { present: true, ctx: Some(cx) });
        // the game's scripts system: pose in, then the lines
        self.gs.player_position = Some(self.pos);
        self.gs.player_heading = self.heading;
        if let Some((q, st)) = self.start_stage.take() {
            if let Some(qf) = d.order.form_by_editor_id(&q) {
                Runner::new(&d.order, &d.scripts, &mut self.gs).set_stage(qf, st);
            }
        }
        for l in std::mem::take(&mut self.start_cmds) {
            self.run_line(d, &l);
        }
        if let Some(t0) = self.ready_at {
            let (due, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.later).into_iter().partition(|(at, _)| t - t0 >= *at);
            self.later = rest;
            let mut due = due;
            due.sort_by(|a, b| a.0.total_cmp(&b.0));
            for (_, l) in due {
                self.run_line(d, &l);
            }
        }
        self.after_scripts(d);
        // walking
        for (at, key, hold) in &self.keys {
            let from = t.max(*at);
            let to = (t + dt as f64).min(at + hold);
            if to > from {
                let step = ((to - from) as f32) * RUN_SPEED;
                let (sx, cy) = (self.heading.sin(), self.heading.cos());
                let (dx, dy) = match key.as_str() {
                    "w" => (sx, cy),
                    "s" => (-sx, -cy),
                    "a" => (-cy, sx),
                    "d" => (cy, -sx),
                    _ => (0.0, 0.0),
                };
                self.pos[0] += dx * step;
                self.pos[1] += dy * step;
            }
        }
        // the bot
        let (due, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.bot).into_iter().partition(|(at, _)| *at <= t);
        self.bot = rest;
        for (_, l) in due {
            self.line(t, &l);
            match bot::run_line(&l, d, &mut self.gs, &mut self.sess, cx) {
                Out::Teleport(p) => self.pos = p,
                Out::Unknown(w) => self.errors.push(format!("Co-op bot: unknown line {w} in {l:?}")),
                Out::None => {}
            }
        }
        if self.fixed_glue {
            self.sess.live_loot_ops(cx, &mut self.gs);
            self.modelled.insert("container actions: Session::live_loot_ops (proposed viewer change)".into());
        } else {
            bot::carry_loot(d, &mut self.gs, &mut self.sess, cx);
        }
        self.talk_frame(t, d);
        self.track(t, d, cx);
        // the live frame (frame::run)
        self.gs.player_position = Some(self.pos);
        self.gs.player_heading = self.heading;
        self.counters.hits_sent = u64::from(self.sess.live_hits_sent());
        let view = self.sess.live_frame(&mut self.gs, cx, dt).cloned();
        self.play_lines(t, view.as_ref());
        self.view = view;
        self.after_scripts(d);
        // coop::send
        self.sess.post_frame(&mut self.gs, dt);
        self.events_and_commands(t, d);
        if t - self.last_dump >= 1.0 {
            self.last_dump = t;
            self.dump(t, d);
        }
    }

    /// Events scripts and the live layer queued: a MoveTo of the player moves
    /// the physical player (as `scripts::move_player` does in the viewer).
    fn after_scripts(&mut self, d: &Data) {
        if self.gs.events.is_empty() {
            return;
        }
        for e in std::mem::take(&mut self.gs.events) {
            if let Event::MoveTo { what, to } = &e {
                if *what == world::dialogue::PLAYER_REF {
                    if let Some((_, _, at, heading)) = self.gs.place(&d.order, *to) {
                        self.pos = at;
                        self.heading = heading;
                    }
                }
            }
            if matches!(e, Event::Died { .. }) {
                self.gs.events.push(e);
            }
        }
        self.gs.player_placed = false;
    }

    fn talk_frame(&mut self, t: f64, d: &Data) {
        let Some(at) = self.talk else { return };
        if t < at {
            return;
        }
        if self.greeting.is_none() && !self.talker.opened {
            match TalkBot::nearest_speaker(d, &self.gs, self.pos, &self.people) {
                Some((npc, info)) => {
                    self.greeting = Some(info);
                    self.line(t, &format!("talk {npc}"));
                    self.talker.open(&mut self.sess, npc);
                }
                None => {
                    self.errors.push("no one near enough has a greeting to talk to".into());
                    self.talk = None;
                }
            }
            return;
        }
        if let Some(g) = self.greeting {
            self.talker.frame(t, &mut self.sess, self.view.as_ref(), g);
        }
    }

    /// `intents::play_lines`: count the picked lines; spectators report them done after a reading time.
    fn play_lines(&mut self, t: f64, view: Option<&LiveView>) {
        let Some(v) = view else { return };
        let mine = v.dialogue.as_ref().is_some_and(|d| d.mine);
        for _ in &v.lines {
            self.counters.lines_played += 1;
            if !mine {
                self.spectate.push(t + READ_SECONDS);
            }
        }
        if v.dialogue.is_none() {
            self.spectate.clear();
            return;
        }
        let before = self.spectate.len();
        self.spectate.retain(|at| *at > t);
        for _ in self.spectate.len()..before {
            self.sess.live_intent(coop_session::Intent::DialogueLineDone);
        }
    }

    fn events_and_commands(&mut self, t: f64, d: &Data) {
        for ev in self.sess.drain_events() {
            match ev {
                CoopEvent::Ready => self.on_ready(t, d),
                CoopEvent::Repaired => {
                    self.repairs += 1;
                    self.log(t, "coop_repair", vec![("kind", s("hash")), ("subsystem", s("shared"))]);
                }
                CoopEvent::Connected | CoopEvent::PeerJoined(..) => {
                    let name = if self.role == Role::Host { "host" } else { "guest" };
                    self.log(t, "coop_connected", vec![("role", s(name))]);
                }
                _ => {}
            }
        }
        if let (Role::Guest, Some(t0)) = (self.role, self.ready_at) {
            let (due, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.guest_timed).into_iter().partition(|(at, _)| t - t0 >= *at);
            self.guest_timed = rest;
            for (_, l) in due {
                self.line(t, &l);
                self.sess.send_command(Command::Line(l));
            }
        }
        for (from, cmd) in self.sess.take_commands() {
            let line = cmd.as_line();
            self.log(t, "script", vec![("line", s(line.clone())), ("from", n(from.0 as f64))]);
            if from.is_host() || !self.gs.coop.bank.enabled() {
                self.start_cmds.push(line);
            } else {
                let (order, scripts) = (&d.order, &d.scripts);
                self.gs.with_player(from.0, |g| {
                    Runner::new(order, scripts, g).run_source(&line, None, None);
                });
            }
        }
        if let Some(e) = self.sess.last_error() {
            let e = e.to_string();
            if !self.errors.contains(&e) {
                self.errors.push(e);
            }
        }
    }

    /// Writes the state dump (kept in memory; the last one is the verdict's).
    pub fn dump(&mut self, t: f64, d: &Data) {
        self.seq += 1;
        if std::env::var_os("COOP_SIM_TRACE_ACTORS").is_some() {
            if let Some(v) = &self.view {
                eprintln!("actors {:?} t={t:.1} owned={:?} remote={:?} dead={}", self.role, v.owned_actors, v.remote_actors.iter().map(|a| (a.id, a.dead, a.health)).collect::<Vec<_>>(), self.gs.dead.len());
            }
        }
        if let Some(spec) = std::env::var_os("COOP_SIM_TRACE") {
            // `HOLDER:ITEM` editor ids: prints that stack once a second (debugging aid)
            let spec = spec.to_string_lossy().to_string();
            if let Some((h, i)) = spec.split_once(':').and_then(|(h, i)| Some((d.form(h)?, d.form(i)?))) {
                eprintln!("trace {:?} t={t:.1} {spec} = {:?}", self.role, self.gs.items.get(&(h, i)));
            }
        }
        let root = Session::state_root(&self.gs);
        self.dump_text = state_json_full(&self.gs, &d.order, self.role, t, self.seq, root, self.view.as_ref(), &self.counters);
    }

    /// The last dump text.
    pub fn dump_text(&self) -> &str {
        &self.dump_text
    }

    /// The final dump and the clean shutdown line.
    pub fn finish(&mut self, t: f64, d: &Data) {
        self.dump(t, d);
        self.log(t, "state_dump", vec![("seq", n(self.seq as f64))]);
        self.log(t, "shutdown", vec![("reason", s("quit-file")), ("clean", J::Bool(true)), ("code", n(0.0))]);
    }
}

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

/// Seconds of scene time the modelled Bink player takes for any movie.
const MOVIE_SECONDS: f64 = 6.0;

/// Units a second a held movement key moves the player: the viewer's Always Run default (walk 77 x fMoveRunMult 4); the fake viewer of tools/coop-run uses 150.
pub const RUN_SPEED: f32 = 308.0;

/// The viewer's avatar animation bits (`coop_live::frame`, see `control::anim_names`).
const ANIM_MOVING: u16 = 1;
const ANIM_RUNNING: u16 = 2;

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
    /// Pings, waypoints and voice.
    pub social: crate::social_bot::SocialBot,
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
    /// The party directory when the scenario has a `party` object.
    pub party_dir: Option<std::path::PathBuf>,
    /// Names of the party events seen, in order.
    pub party_log: Vec<String>,
    /// Glitch detectors over the remote avatars this peer draws.
    pub glitch: coop_session::glitch::GlitchWatch,
    /// Flight recorder: every dump with its time, when recording is on
    /// (`RunOpts::record`).
    pub flight: Option<Vec<(f64, String)>>,
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
    /// A script's `StartConversation` with the player: (speaker, topic).
    script_talk: Option<(FormId, Option<FormId>)>,
    /// `coop clock on`: the host runs `Runner::update`'s clock step.
    clock_model: bool,
    people: Vec<(FormId, FormId)>,
    tracked: BTreeSet<FormId>,
    next_track: f64,
    spectate: Vec<f64>,
    last_dump: f64,
    movie_end: Option<f64>,
    seq: u64,
    dump_text: String,
    menu_left: u32,
    /// Live-layer places (a worldspace outdoors) back to (space, cell).
    places: world::coop::live::PlaceResolver,
    /// Worldspaces whose grid is in the session's cell table.
    grids_done: BTreeSet<FormId>,
}

fn is_local(l: &str) -> Option<&str> {
    l.strip_prefix("local ")
}

impl Peer {
    /// A peer standing at the cell's arrival point with `side`'s script.
    pub fn new(
        role: Role,
        sess: Session,
        d: &Data,
        place: &Place,
        side: &Side,
        early_talk: bool,
        fixed_glue: bool,
    ) -> Peer {
        let mut gs = GameState::new(&d.order);
        gs.player_cell = Some(place.cell);
        // an exterior cell starts in its worldspace, as the viewer starts it
        gs.player_world = place.world;
        let mut p = Peer {
            role,
            sess,
            gs,
            pos: place.spawn,
            heading: place.heading,
            counters: LiveCounters::default(),
            social: Default::default(),
            events: Vec::new(),
            modelled: BTreeSet::new(),
            ready_at: None,
            errors: Vec::new(),
            repairs: 0,
            view: None,
            party_dir: None,
            party_log: Vec::new(),
            glitch: Default::default(),
            flight: None,
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
            script_talk: None,
            clock_model: false,
            people: place.actors.clone(),
            tracked: BTreeSet::new(),
            next_track: 0.0,
            spectate: Vec::new(),
            last_dump: -1.0,
            movie_end: None,
            seq: 0,
            dump_text: String::new(),
            menu_left: 0,
            places: Default::default(),
            grids_done: BTreeSet::new(),
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
                    p.modelled
                        .insert("--choose/--say: the stand-in menu picks the greeting only".into());
                }
            }
        }
        p
    }

    /// Adds a line to the event log (for the party glue).
    pub fn log_pub(&mut self, t: f64, ev: &str, f: Vec<(&str, J)>) {
        self.log(t, ev, f);
    }

    fn log(&mut self, t: f64, ev: &str, f: Vec<(&str, J)>) {
        let mut v = vec![("t", n((t * 1000.0).round() / 1000.0)), ("ev", s(ev))];
        v.extend(f);
        self.events.push(obj(v));
    }

    fn line(&mut self, t: f64, l: &str) {
        self.log(t, "script", vec![("line", s(l))]);
    }

    /// The viewer's `coop_live::intents::travel`: fast travel the party
    /// allowed is carried out (the trip was refused with "Waiting for the
    /// party" when it was asked for). The arrival moves the player through
    /// `after_scripts`.
    fn travel(&mut self, d: &Data, view: Option<&LiveView>) {
        let Some(marker) = view.and_then(|v| v.flags.travel_allowed) else {
            return;
        };
        // the viewer looks the marker up in the map's list
        let m = world::map::marker(&d.order, marker).unwrap_or(world::map::MapMarker {
            reference: marker,
            name: String::new(),
            position: [0.0; 3],
            radius: 0.0,
            flags: 0,
            kind: 0,
            arrival: None,
        });
        if world::map::travel(&d.order, &mut self.gs, &m).is_ok() {
            self.counters.travels += 1;
        }
    }

    /// The viewer's `coop_live::roam::register_grid`: the grid of the
    /// worldspace the player is in goes into the session's cell table once.
    fn register_grid(&mut self, d: &Data, w: FormId) {
        if self.grids_done.contains(&w) {
            return;
        }
        let Some(graph) = self.sess.live_roam_graph() else {
            return;
        };
        if let Some(g) = self.places.grid(&d.order, w) {
            world::coop::roam_ops::register_world_grid(graph, g);
        }
        self.grids_done.insert(w);
    }

    /// The cell's people the host replicates (the viewer's `actors::track`).
    fn track(&mut self, t: f64, d: &Data, cx: Ctx) {
        if t < self.next_track {
            return;
        }
        self.next_track = t + 1.0;
        // the player's place as the live layer files it: the interior cell, or
        // outdoors the worldspace (the viewer loads the grid around the player)
        let space = self.gs.player_world.or(self.gs.player_cell);
        // outdoors only the grid square around the player is loaded
        let outdoors = self.gs.player_world.is_some();
        let here = coop_core::roam::graph::square_of(self.pos);
        let r = coop_core::roam::GRID_RADIUS;
        let near = |p: [f32; 3]| {
            let s = coop_core::roam::graph::square_of(p);
            (s.0 - here.0).abs() <= r && (s.1 - here.1).abs() <= r
        };
        let new: Vec<FormId> = self
            .people
            .iter()
            .map(|p| p.0)
            .filter(|r| {
                !self.tracked.contains(r)
                    && world::enabled_now(&d.order, *r, &self.gs.disabled)
                    && self
                        .gs
                        .place(&d.order, *r)
                        .is_some_and(|p| Some(p.0) == space && (!outdoors || near(p.2)))
            })
            .collect();
        if !new.is_empty() {
            if self.sess.live_track(cx, &self.gs, &new) {
                self.tracked.extend(new);
            }
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
        let name = if self.role == Role::Host {
            "host"
        } else {
            "guest"
        };
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
        self.sess.pre_frame_with(
            &mut self.gs,
            Opts {
                present: true,
                ctx: Some(cx),
            },
        );
        // the game's scripts system: pose in, then the lines
        self.gs.player_position = Some(self.pos);
        self.gs.player_heading = self.heading;
        if let Some((q, st)) = self.start_stage.take() {
            if let Some(qf) = d.form(&q) {
                Runner::new(&d.order, &d.scripts, &mut self.gs).set_stage(qf, st);
            }
        }
        for l in std::mem::take(&mut self.start_cmds) {
            self.run_line(d, &l);
        }
        if let Some(t0) = self.ready_at {
            let (due, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.later)
                .into_iter()
                .partition(|(at, _)| t - t0 >= *at);
            self.later = rest;
            let mut due = due;
            due.sort_by(|a, b| a.0.total_cmp(&b.0));
            for (_, l) in due {
                self.run_line(d, &l);
            }
        }
        self.after_scripts(d);
        // walking
        let mut walking = false;
        for (at, key, hold) in &self.keys {
            let from = t.max(*at);
            let to = (t + dt as f64).min(at + hold);
            if to > from {
                walking = true;
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
        let (due, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.bot)
            .into_iter()
            .partition(|(at, _)| *at <= t);
        self.bot = rest;
        for (_, l) in due {
            self.line(t, &l);
            if crate::party::line(self, t, &l) {
                continue;
            }
            if let Some(on) = l.trim().strip_prefix("coop clock ") {
                self.clock_model = on.trim() == "on";
                continue;
            }
            if crate::social_bot::SocialBot::owns(&l) {
                let me = self.sess.status().player;
                let cell = self.gs.player_cell.map_or(0, |c| c.0);
                self.social
                    .handle(&l, t, me, cell, self.pos, self.heading, &mut self.sess);
                continue;
            }
            match bot::run_line(&l, d, &mut self.gs, &mut self.sess, cx) {
                Out::Teleport(p) => self.pos = p,
                Out::Face(h) => self.heading = h,
                Out::Unknown(w) => self
                    .errors
                    .push(format!("Co-op bot: unknown line {w} in {l:?}")),
                Out::None => {}
            }
        }
        if self.fixed_glue {
            self.sess.live_loot_ops(cx, &mut self.gs);
            self.modelled.insert(
                "container actions: Session::live_loot_ops (proposed viewer change)".into(),
            );
        } else {
            bot::carry_loot(d, &mut self.gs, &mut self.sess, cx);
        }
        self.talk_frame(t, d, cx);
        self.clock_step(d, dt);
        self.track(t, d, cx);
        // the live frame (frame::run)
        self.gs.player_position = Some(self.pos);
        self.gs.player_heading = self.heading;
        // outdoors the viewer files the player in the grid cell under its feet
        if let Some(w) = self.gs.player_world {
            if let Some((_, c)) = self.places.resolve(&d.order, w, self.pos) {
                self.gs.player_cell = Some(c);
            }
            self.register_grid(d, w);
        }
        // the avatar's animation bits, as `coop_live::frame::run` sets them from the
        // player's speed (keys move the player at the Always Run speed: moving + running)
        self.gs.coop.live.local_anim = if walking {
            ANIM_MOVING | ANIM_RUNNING
        } else {
            0
        };
        self.counters.hits_sent = u64::from(self.sess.live_hits_sent());
        self.counters.tracked = self.sess.live_tracked() as u64;
        self.count_entities(d);
        let view = self.sess.live_frame(&mut self.gs, cx, dt).cloned();
        self.play_lines(t, view.as_ref());
        self.model_movie(t);
        self.social.frame(t, dt, &mut self.sess, view.as_ref());
        self.counters.voice_out = self.social.voice_out;
        self.counters.voice_in = self.social.voice_in;
        self.counters.voiced_frames = self.social.voiced_frames;
        self.counters.pings = self.social.pings;
        self.counters.speaking_frames = self.social.speaking_frames;
        self.counters.chat_in = self.social.chat_in;
        // the glitch detectors over the remote avatars as this peer draws them
        if let Some(v) = view.as_ref() {
            self.glitch.frame(v.now_ms, &v.avatars);
        }
        self.counters.glitch = self.glitch.counts;
        self.counters.full_refused = u64::from(self.sess.full_refusals());
        // the viewer moves the player to a placement the host asked for (late join, party warp)
        if let Some(pl) = view.as_ref().and_then(|v| v.roam.place) {
            // a placement outdoors names the worldspace: the grid cell at the spot
            let to = FormId(pl.cell.0);
            let (space, cell) = self
                .places
                .resolve(&d.order, to, pl.pos)
                .unwrap_or((to, to));
            self.gs.player_cell = Some(cell);
            self.gs.player_world = (space != cell).then_some(space);
            self.pos = pl.pos;
            self.heading = pl.heading;
            self.modelled.insert("LiveView.roam.place: the player is moved to the placement (viewer glue to be written)".into());
        }
        self.travel(d, view.as_ref());
        self.view = view;
        self.character_menus();
        self.follow_door(d);
        self.after_scripts(d);
        // coop::send
        self.sess.post_frame(&mut self.gs, dt);
        crate::party::pump(self, t);
        self.events_and_commands(t, d);
        if t - self.last_dump >= 1.0 {
            self.last_dump = t;
            self.dump(t, d);
        }
    }

    /// A door the party went through: the physical player arrives on the other
    /// side (the viewer's `coop_live::interact::follow_door`).
    fn follow_door(&mut self, d: &Data) {
        let Some(f) = self.gs.coop.interact.follow.take() else {
            return;
        };
        if let Some(a) =
            world::coop::interact::arrival_of(&d.order, &self.gs, esm::FormId(f.door.0))
        {
            self.pos = a.pos;
            self.heading = a.heading;
            self.gs.player_cell = Some(a.cell);
            self.gs.player_world = (!a.interior).then_some(a.space);
        } else {
            // a door the data does not know: the cell still changes
            self.gs.player_cell = Some(esm::FormId(f.to.0));
        }
        self.counters.doors_followed += 1;
    }

    /// The viewer's guest-side character making (`coop::start_menus`): the menus the flow asks for are
    /// stood in for by setting the rows each one sets when it closes.
    fn character_menus(&mut self) {
        if self.role != Role::Guest {
            return;
        }
        self.menu_left = self.menu_left.saturating_sub(1);
        let Some(step) = self.sess.start_poll(&self.gs, self.menu_left > 0) else {
            return;
        };
        self.modelled.insert("character creation: the menus are stood in for (the rows they set are set when they close); a local room is not loaded, the guest stays in the host's cell".into());
        self.menu_left = 20;
        let gs = &mut self.gs;
        match step {
            coop_session::Step::Name => gs.player_name = Some("Courier Guest".into()),
            coop_session::Step::Special => {
                for (i, av) in coop_session::start::SPECIAL.iter().enumerate() {
                    gs.actor_values
                        .insert((world::dialogue::PLAYER_REF, *av), (3 + i) as f64);
                }
            }
            coop_session::Step::TagSkills => gs.tag_skills = [32u16, 40, 41].into_iter().collect(),
            coop_session::Step::Traits => {}
        }
    }

    /// The Bink player, modelled: a movie the scene layer starts runs for
    /// `MOVIE_SECONDS` of scene time, a stop ends it, and the world is told
    /// when it is on screen (the viewer's `Movies` does the real thing).
    fn model_movie(&mut self, t: f64) {
        let sc = &mut self.gs.coop.live.scene;
        if let Some(m) = sc.movie_start.take() {
            let left = (MOVIE_SECONDS - f64::from(m.offset_ms) / 1000.0).max(0.0);
            self.movie_end = Some(t + left);
            self.modelled.insert(
                "movie playback: a timer stands in for the Bink player (live_movie_sync)".into(),
            );
        }
        if std::mem::take(&mut sc.movie_stop) {
            self.movie_end = None;
        }
        if self.movie_end.is_some_and(|e| t >= e) {
            self.movie_end = None;
        }
        sc.movie_busy = self.movie_end.is_some();
    }

    /// Events scripts and the live layer queued: a MoveTo of the player moves
    /// the physical player (as `scripts::move_player` does in the viewer).
    fn after_scripts(&mut self, d: &Data) {
        if self.gs.events.is_empty() {
            return;
        }
        for e in std::mem::take(&mut self.gs.events) {
            if let Event::MoveTo { what, to, at } = &e {
                if *what == world::dialogue::PLAYER_REF {
                    if let Some((_, _, spot, heading)) = self.gs.place(&d.order, *to) {
                        self.pos = at.unwrap_or(spot);
                        self.heading = heading;
                    }
                }
            }
            // a script's conversation with the player opens the viewer's menu
            // (`scripts` → `ai::Starts` → the conversation)
            if let Event::Talk {
                speaker,
                to,
                topic,
                conversation: true,
            } = &e
            {
                if *to == world::dialogue::PLAYER_REF && *speaker != world::dialogue::PLAYER_REF {
                    self.script_talk = Some((*speaker, *topic));
                }
            }
            if matches!(e, Event::Died { .. }) {
                self.gs.events.push(e);
            }
        }
        self.gs.player_placed = false;
    }

    /// `Runner::update`'s clock step on the host, as the viewer's `scripts`
    /// gates it: the host's conversation menu stops it while
    /// `GameState::coop_conversation_stops_time` says so. Only after
    /// `coop clock on` (the headless peer runs no `Runner::update`). A
    /// guest's update moves no clock, so a guest does nothing here.
    fn clock_step(&mut self, d: &Data, dt: f32) {
        if !self.clock_model || self.role != Role::Host {
            return;
        }
        self.modelled.insert("coop clock on: the host's Runner::update clock step and the viewer's dialogue-menu gate (scripts::run_scripts)".into());
        let menu = self
            .view
            .as_ref()
            .and_then(|v| v.dialogue.as_ref())
            .is_some_and(|x| x.mine);
        if !menu || !self.gs.coop_conversation_stops_time() {
            self.gs.advance_clock(&d.order, dt);
        }
    }

    fn talk_frame(&mut self, t: f64, d: &Data, cx: Ctx) {
        // the viewer's menu for a script's conversation: it opens the session
        // and says the first line (`intents::tap_dialogue`), and stays up
        if let Some((npc, topic)) = self.script_talk.take() {
            if let Some(info) = world::coop::dialogue::line_of(cx, &self.gs, npc, topic) {
                self.modelled.insert("a script's StartConversation: the stand-in menu opens the session, picks the first line and stays up (the viewer's conversation menu and intents::tap_dialogue)".into());
                self.greeting = Some(info);
                self.talk = Some(t);
                self.talker.stay = true;
                self.line(t, &format!("talk {npc}"));
                self.talker.open(&mut self.sess, npc);
            }
        }
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
                    self.errors
                        .push("no one near enough has a greeting to talk to".into());
                    self.talk = None;
                }
            }
            return;
        }
        if let Some(g) = self.greeting {
            self.talker.frame(t, &mut self.sess, self.view.as_ref(), g);
        }
    }

    /// The viewer's `actors::count`: the people and creatures that would
    /// stand in the scene (placed ones of the player's place and the ones
    /// scripts made), none hidden.
    fn count_entities(&mut self, d: &Data) {
        let space = self.gs.player_world.or(self.gs.player_cell);
        let made: Vec<(FormId, FormId)> = self
            .gs
            .more
            .placed
            .refs
            .iter()
            .map(|(r, m)| (*r, m.base))
            .collect();
        let (mut npcs, mut creatures) = (0u64, 0u64);
        for (r, base) in self.people.iter().copied().chain(made) {
            if !world::enabled_now(&d.order, r, &self.gs.disabled)
                || self.gs.place(&d.order, r).map(|p| p.0) != space
            {
                continue;
            }
            if d.order
                .get(base)
                .is_some_and(|x| x.entry.header.kind.as_bytes() == b"CREA")
            {
                creatures += 1;
            } else {
                npcs += 1;
            }
        }
        self.counters.npc_entities = npcs;
        self.counters.npc_entities_visible = npcs;
        self.counters.creature_entities = creatures;
        self.counters.creature_entities_visible = creatures;
    }

    /// `intents::play_lines`: count the picked lines; spectators report them done after a reading time.
    fn play_lines(&mut self, t: f64, view: Option<&LiveView>) {
        let Some(v) = view else { return };
        self.counters.puppet_actions += v.actions.len() as u64;
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
            self.sess
                .live_intent(coop_session::Intent::DialogueLineDone);
        }
    }

    fn events_and_commands(&mut self, t: f64, d: &Data) {
        for ev in self.sess.drain_events() {
            match ev {
                CoopEvent::Ready => self.on_ready(t, d),
                CoopEvent::Repaired => {
                    self.repairs += 1;
                    self.log(
                        t,
                        "coop_repair",
                        vec![("kind", s("hash")), ("subsystem", s("shared"))],
                    );
                }
                CoopEvent::Compat { refused, .. } => {
                    if refused {
                        self.counters.compat_refused += 1;
                    } else {
                        self.counters.compat_warned += 1;
                    }
                }
                CoopEvent::Connected | CoopEvent::PeerJoined(..) => {
                    let name = if self.role == Role::Host {
                        "host"
                    } else {
                        "guest"
                    };
                    self.log(t, "coop_connected", vec![("role", s(name))]);
                }
                _ => {}
            }
        }
        if let (Role::Guest, Some(t0)) = (self.role, self.ready_at) {
            let (due, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut self.guest_timed)
                .into_iter()
                .partition(|(at, _)| t - t0 >= *at);
            self.guest_timed = rest;
            for (_, l) in due {
                self.line(t, &l);
                self.sess.send_command(Command::Line(l));
            }
        }
        for (from, cmd) in self.sess.take_commands() {
            let line = cmd.as_line();
            self.log(
                t,
                "script",
                vec![("line", s(line.clone())), ("from", n(from.0 as f64))],
            );
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
                eprintln!(
                    "actors {:?} t={t:.1} owned={:?} remote={:?} dead={}",
                    self.role,
                    v.owned_actors,
                    v.remote_actors
                        .iter()
                        .map(|a| (a.id, a.dead, a.health))
                        .collect::<Vec<_>>(),
                    self.gs.dead.len()
                );
            }
        }
        if let Some(spec) = std::env::var_os("COOP_SIM_TRACE") {
            // `HOLDER:ITEM` editor ids: prints that stack once a second (debugging aid)
            let spec = spec.to_string_lossy().to_string();
            if let Some((h, i)) = spec
                .split_once(':')
                .and_then(|(h, i)| Some((d.form(h)?, d.form(i)?)))
            {
                eprintln!(
                    "trace {:?} t={t:.1} {spec} = {:?}",
                    self.role,
                    self.gs.items.get(&(h, i))
                );
            }
        }
        self.counters.start = Some(self.sess.start_info());
        self.counters.status = Some(self.sess.status());
        self.counters.hash_points = Some(self.sess.hash_points());
        let root = Session::state_root(&self.gs);
        self.dump_text = state_json_full(
            &self.gs,
            &d.order,
            self.role,
            t,
            self.seq,
            root,
            self.view.as_ref(),
            &self.counters,
        );
        if let Some(f) = self.flight.as_mut() {
            f.push((t, self.dump_text.clone()));
        }
    }

    /// The last dump text.
    pub fn dump_text(&self) -> &str {
        &self.dump_text
    }

    /// The final dump and the clean shutdown line.
    pub fn finish(&mut self, t: f64, d: &Data) {
        self.dump(t, d);
        self.log(t, "state_dump", vec![("seq", n(self.seq as f64))]);
        self.log(
            t,
            "shutdown",
            vec![
                ("reason", s("quit-file")),
                ("clean", J::Bool(true)),
                ("code", n(0.0)),
            ],
        );
    }
}
